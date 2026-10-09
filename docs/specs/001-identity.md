# SPEC-001: Identity (Bucks ID and identity log)

Status: Draft. Version 0.1. Depends on [SPEC-003](003-encoding.md). Vectors: [identity-v1.json](../../spec/test-vectors/identity-v1.json).

## 1. Overview

A person's identity is a **log** of signed operations. The first operation, the **inception**, defines the **Bucks ID**. Later operations add or remove devices and rotate keys. Anyone holding the log can verify, offline, the person's current keys and devices.

## 2. Primitives

| Item | Definition |
|---|---|
| Public key | NIST P-256 point, compressed SEC1, 33 bytes. MUST be a valid curve point. |
| Signature | ECDSA P-256 over SHA-256 of the message, as 64 bytes `r || s`. **Low-S only**: a signature whose `s` is greater than half the group order MUST be rejected. |
| Hash | BLAKE3, 256-bit. |
| Key roles | **Rotation key** signs identity operations. **Device key** signs events (SPEC-002). A key SHOULD NOT serve both roles. |

Signers SHOULD use deterministic ECDSA (RFC 6979). Verifiers MUST NOT require it.

## 3. Structures

All maps are encoded per SPEC-003. Field names are exact.

### 3.1 Device

| Field | Type | Rule |
|---|---|---|
| `id` | bytes(16) | Unique within the identity. |
| `key` | bytes(33) | Device public key. Unique within the identity. |
| `name` | text | At most 64 bytes of UTF-8. Informational. |

### 3.2 Unsigned operation (the signed message)

Common fields: `v` = 1, `type` (text), `seq` (uint), `prev` (bytes(32) or null).

| `type` | Extra fields |
|---|---|
| `inception` | `rotation_keys` (array of bytes(33), 1 to 3), `devices` (array of Device, 1 to 32), `home` (array of text, 0 to 8) |
| `add_device` | `device` (Device) |
| `remove_device` | `device_id` (bytes(16)) |
| `rotate_keys` | `rotation_keys` (array of bytes(33), 1 to 3) |

### 3.3 Signed operation (wire form)

The unsigned fields plus `by` (bytes(33), the signing rotation key) and `sig` (bytes(64)). The signature covers the canonical encoding of the **unsigned** map. The decoder MUST reject any other field.

### 3.4 Operation hash

`hash(op) = BLAKE3(wire form of op)`. The next operation's `prev` is this value.

## 4. The Bucks ID

```
digest  = BLAKE3(unsigned encoding of the inception)
id      = digest[0..16]
id[6]   = (id[6] & 0x0f) | 0x80      // version nibble 8
id[8]   = (id[8] & 0x3f) | 0x80      // RFC 9562 variant
```

- Text form: 32 lowercase hex digits grouped 8-4-4-4-12 with hyphens.
- DID form: `did:bucks:` followed by the text form.
- A parser MUST reject a text whose version nibble is not 8 or whose variant bits are not `10`.
- The ID is computed from the **unsigned** inception so a third party cannot change it by altering the signature.

### 4.1 Short code

`short = Crockford-base32( first 5 bytes of BLAKE3(id bytes) )`, 40 bits read big-endian and written most significant first as 8 characters from the alphabet `0123456789ABCDEFGHJKMNPQRSTVWXYZ`. The short code is a convenience label and is not unique; implementations MUST NOT use it as an identifier.

### 4.2 `did:key` for a P-256 key

`did:key:z` followed by base58btc of `0x80 0x24` (multicodec p256-pub, 0x1200, varint) then the 33-byte key.

## 5. Verification algorithm

Given an ordered list of operations `ops`:

1. `ops` MUST be non-empty and `ops[0]` MUST be an `inception` with `seq` = 0 and `prev` = null. (V1)
2. `rotation_keys` MUST have 1 to 3 entries, all valid points, no duplicates. (V2)
3. `ops[0].by` MUST be one of the inception's `rotation_keys`. (V3)
4. `devices` MUST have 1 to 32 entries, unique `id` and unique `key`, valid keys, names within limit. `home` MUST have at most 8 entries. (V4)
5. `ops[0].sig` MUST verify over the unsigned encoding. (V5)
6. State after inception: `id` per section 4, the keys, devices and home, `head` = hash(op), `seq` = 0.
7. For each later op: `op.seq` MUST equal `state.seq + 1` and `op.prev` MUST equal `state.head` (V6); `op.by` MUST be in the state's **current** rotation keys (V7); `op.sig` MUST verify (V8); the type MUST NOT be `inception` (V9).
8. Apply: `add_device` requires room (at most 32) and a unique id and key (V10). `remove_device` requires the device to exist and at least one device to remain (V11). `rotate_keys` requires a valid key set per V2 (V12).
9. Update `head` and `seq`. The state after the last op is the identity's current state.

A verifier MUST return the state after **every** operation (the history), because events refer to historical positions (SPEC-002).

## 6. Security considerations

- **Signature malleability.** Low-S is required, and the Bucks ID excludes the signature.
- **Key reuse across roles.** Using one key as both a rotation and a device key couples their compromise.
- **Compromised rotation key.** Any single current rotation key can rotate all keys (no priority or time window in v0.1). A stolen backup key is as powerful as the primary. See Known gaps.
- **Log withholding.** A directory can withhold a later operation, such as a revocation. Clients SHOULD consult more than one directory and prefer the longest valid chain.
- **Fork.** Two valid operations with the same `seq` and `prev` form a fork. v0.1 does not define resolution; see Known gaps.
- **Hash truncation.** 128 bits of ID give a collision resistance near 2^64 work against a determined attacker, and second-preimage resistance near 2^128. [Likely adequate; flagged for review.]

## 7. Known gaps (v0.1)

| Gap | Impact | Tracking |
|---|---|---|
| No domain-separation prefix on signed bytes | Cross-protocol signature reuse if a key is exposed to a signing oracle | [OQ-11](../open-questions.md) |
| No priority or time window for rotation keys | A single stolen rotation key can take over | [OQ-07](../open-questions.md), REQ-ID-08 |
| No fork-resolution rule | Conflicting logs possible if two keys sign concurrently | [OQ-07](../open-questions.md) |
| No handle, services-update, recovery or attestation operations | Features in REQ-ID-07 and REQ-ID-09 cannot be expressed yet | Future spec versions |
| Per-entry size limit for `home` not enforced | Large entries possible | Add limit before 1.0 |
| No time source | Cannot bound how long ago an operation was made | OQ-07 |

## 8. Reference implementation

The predecessor Rust crate `bucks-core` (module `log`, `id`, `cbor`) implements this version. It lives on the branch `claude/unification-plan-and-core` of the `buckscore` repository and will be re-imported here at milestone M1. Fourteen unit and vector tests pass at the time of writing. [Certain]
