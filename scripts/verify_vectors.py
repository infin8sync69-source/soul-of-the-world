#!/usr/bin/env python3
"""Independent verifier for spec/test-vectors (SPEC-001..003 v0.2).

Written separately from the Rust core and sharing no code with it: its own CBOR codec,
ECDSA via the `cryptography` package, BLAKE3 via the `blake3` package. Passing means two
implementations agree on IDs, hashes, signatures and chain rules.

    pip install cryptography blake3
    python3 scripts/verify_vectors.py
"""
import json
import sys
from pathlib import Path

import blake3
from cryptography.exceptions import InvalidSignature
from cryptography.hazmat.primitives import hashes
from cryptography.hazmat.primitives.asymmetric import ec
from cryptography.hazmat.primitives.asymmetric.utils import encode_dss_signature

ROOT = Path(__file__).resolve().parent.parent
VEC = ROOT / "spec" / "test-vectors"
N = 0xFFFFFFFF00000000FFFFFFFFFFFFFFFFBCE6FAADA7179E84F3B9CAC2FC632551  # P-256 order
CTX = {
    "id": "bucks 2026-10-09 identity id v1",
    "op": "bucks 2026-10-09 identity op hash v1",
    "event": "bucks 2026-10-09 event hash v1",
    "short": "bucks 2026-10-09 short code v1",
}
TAG = {"op": b"bucks/identity-op/v1", "event": b"bucks/event/v1"}
failures = []


def check(cond, msg):
    if not cond:
        failures.append(msg)


# --- independent minimal canonical CBOR -------------------------------------------------
def enc(v):
    def head(major, n):
        if n < 24:
            return bytes([major << 5 | n])
        for info, size in ((24, 1), (25, 2), (26, 4), (27, 8)):
            if n < 1 << (8 * size):
                return bytes([major << 5 | info]) + n.to_bytes(size, "big")
        raise ValueError("int too large")

    if v is None:
        return b"\xf6"
    if isinstance(v, int):
        return head(0, v)
    if isinstance(v, bytes):
        return head(2, len(v)) + v
    if isinstance(v, str):
        b = v.encode()
        return head(3, len(b)) + b
    if isinstance(v, list):
        return head(4, len(v)) + b"".join(enc(x) for x in v)
    if isinstance(v, dict):
        items = sorted(((enc(k), enc(x)) for k, x in v.items()), key=lambda kv: kv[0])
        return head(5, len(items)) + b"".join(k + x for k, x in items)
    raise TypeError(type(v))


def dec(b):
    def rd(i):
        ib = b[i]
        i += 1
        if ib == 0xF6:
            return None, i
        major, info = ib >> 5, ib & 31
        if info < 24:
            n = info
        else:
            size = {24: 1, 25: 2, 26: 4, 27: 8}[info]
            n = int.from_bytes(b[i:i + size], "big")
            i += size
        if major == 0:
            return n, i
        if major in (2, 3):
            s = b[i:i + n]
            return (s if major == 2 else s.decode()), i + n
        if major == 4:
            out = []
            for _ in range(n):
                x, i = rd(i)
                out.append(x)
            return out, i
        if major == 5:
            out = {}
            for _ in range(n):
                k, i = rd(i)
                x, i = rd(i)
                out[k] = x
            return out, i
        raise ValueError("unsupported")

    v, end = rd(0)
    assert end == len(b) and enc(v) == b, "not canonical"
    return v


# --- primitives -------------------------------------------------------------------------
def h(ctx, data):
    return blake3.blake3(data, derive_key_context=CTX[ctx]).digest()


def verify_sig(pub33, msg, sig64):
    r, s = int.from_bytes(sig64[:32], "big"), int.from_bytes(sig64[32:], "big")
    if s > N // 2:
        return False
    key = ec.EllipticCurvePublicKey.from_encoded_point(ec.SECP256R1(), pub33)
    try:
        key.verify(encode_dss_signature(r, s), msg, ec.ECDSA(hashes.SHA256()))
        return True
    except InvalidSignature:
        return False


def bucks_id(unsigned):
    b = bytearray(h("id", unsigned)[:16])
    b[6] = (b[6] & 0x0F) | 0x80
    b[8] = (b[8] & 0x3F) | 0x80
    x = b.hex()
    return f"{x[:8]}-{x[8:12]}-{x[12:16]}-{x[16:20]}-{x[20:]}"


def short_code(id_text):
    raw = bytes.fromhex(id_text.replace("-", ""))
    n = int.from_bytes(h("short", raw)[:5], "big")
    a = "0123456789ABCDEFGHJKMNPQRSTVWXYZ"
    return "".join(a[(n >> (5 * i)) & 31] for i in reversed(range(8)))


def b58(b):
    a = "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz"
    n, out = int.from_bytes(b, "big"), ""
    while n:
        n, r = divmod(n, 58)
        out = a[r] + out
    return "1" * (len(b) - len(b.lstrip(b"\0"))) + out


def did_key(pub33):
    return "did:key:z" + b58(b"\x80\x24" + pub33)


def unsigned_of(op):
    return enc({k: v for k, v in op.items() if k not in ("by", "sig")})


# --- identity vectors -------------------------------------------------------------------
def identity():
    v = json.loads((VEC / "identity-v1.json").read_text())
    for label, sk_hex in v["test_secret_keys"].items():
        check(sk_hex == blake3.blake3(label.encode()).hexdigest(), f"secret for {label} is not BLAKE3(label)")
    ops = [dec(bytes.fromhex(x)) for x in v["log_hex"]]
    inc = ops[0]
    check(unsigned_of(inc).hex() == v["inception_unsigned_hex"], "inception unsigned bytes differ")
    rot, prev = list(inc["rotation_keys"]), None
    for i, (op, raw) in enumerate(zip(ops, v["log_hex"])):
        check(op["seq"] == i, f"op {i}: seq")
        check(op.get("prev") == prev, f"op {i}: prev chain")
        check(op["by"] in rot, f"op {i}: signer not a current rotation key")
        check(verify_sig(op["by"], TAG["op"] + b"\0" + unsigned_of(op), op["sig"]), f"op {i}: signature")
        prev = h("op", bytes.fromhex(raw))
        check(prev.hex() == v["op_hashes"][i], f"op {i}: hash")
        if op["type"] == "rotate_keys":
            rot = list(op["rotation_keys"])
    the_id = bucks_id(unsigned_of(inc))
    check(the_id == v["bucks_id"], "bucks id")
    check(v["did"] == "did:bucks:" + the_id, "did")
    check(short_code(the_id) == v["short_code"], "short code")
    check(prev.hex() == v["final_state"]["head"], "final head")
    check([did_key(k) for k in rot] == v["final_state"]["rotation_keys"], "final rotation keys")


# --- event vectors ----------------------------------------------------------------------
def events():
    v = json.loads((VEC / "events-v1.json").read_text())
    inc = dec(bytes.fromhex(v["inception_hex"]))
    check(verify_sig(inc["by"], TAG["op"] + b"\0" + unsigned_of(inc), inc["sig"]), "event inception signature")
    the_id = bucks_id(unsigned_of(inc))
    check(the_id == v["author"], "event author id")
    devices = {d["id"]: d["key"] for d in inc["devices"]}
    prev = None
    for i, raw in enumerate(v["events_hex"]):
        e = dec(bytes.fromhex(raw))
        check(e["author"].hex() == the_id.replace("-", ""), f"event {i}: author")
        check(e["seq"] == i and e.get("prev") == prev, f"event {i}: stream")
        key = devices.get(e["device"])
        check(key is not None, f"event {i}: device")
        unsigned = enc({k: x for k, x in e.items() if k != "sig"})
        check(key is not None and verify_sig(key, TAG["event"] + b"\0" + unsigned, e["sig"]), f"event {i}: signature")
        prev = h("event", bytes.fromhex(raw))
        check(prev.hex() == v["event_hashes"][i], f"event {i}: hash")


identity()
events()
if failures:
    print("FAIL:")
    for f in failures:
        print("  -", f)
    sys.exit(1)
print("ok: identity and event vectors verified independently")
