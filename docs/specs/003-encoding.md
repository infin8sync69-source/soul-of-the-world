# SPEC-003: Deterministic encoding

Status: Draft. Version 0.2 (2026-10-09; v0.1 had no domain separation). Used by [SPEC-001](001-identity.md) and [SPEC-002](002-events.md).

All signed and hashed structures are encoded as a strict subset of CBOR (RFC 8949) so that each structure has **exactly one** valid byte string.

## 1. Permitted types

| Type | CBOR major | Rule |
|---|---|---|
| null | 7 | Single byte `0xf6`. |
| Unsigned integer | 0 | Range 0 to 2^64-1. Shortest form only. |
| Byte string | 2 | Definite length. |
| Text string | 3 | Definite length. Valid UTF-8. |
| Array | 4 | Definite length. |
| Map | 5 | Definite length. Keys are text strings only. |

Everything else (negative integers, floats, booleans, tags, indefinite lengths, other simple values) is **not permitted**. A decoder MUST reject it.

## 2. Integer and length encoding

A decoder and an encoder MUST use the shortest form for each argument:

| Value | Encoding |
|---|---|
| 0 to 23 | Argument in the initial byte |
| 24 to 255 | Initial byte with additional info 24, then 1 byte |
| 256 to 65535 | Additional info 25, then 2 bytes (big-endian) |
| 65536 to 2^32-1 | Additional info 26, then 4 bytes |
| Larger | Additional info 27, then 8 bytes |

## 3. Map ordering

Map entries MUST be ordered by the **bytes of their encoded key**, compared lexicographically. Because the encoded key begins with its length, shorter keys sort first, and keys of equal length sort bytewise. Duplicate keys MUST be rejected.

Example: the map `{"bb":1, "a":2, "ccc":null}` encodes as
`a3 61 61 02 62 62 62 01 63 63 63 63 f6`.

## 4. Decoder requirements

A decoder MUST:

1. reject any input with trailing bytes,
2. reject nesting deeper than 16 levels,
3. reject lengths that exceed the remaining input,
4. reject map keys that are not text, and
5. after decoding, **re-encode and compare**. If the result is not byte-identical to the input the input MUST be rejected as non-canonical.

Rule 5 makes every accepted structure unique and removes a class of signature-malleability and parser-differential attacks.

## 5. Signing and hashing inputs (domain separation, ADR-0013)

**Signatures.** ECDSA over NIST P-256 with SHA-256. The message is

```
message = tag || 0x00 || canonical encoding of the unsigned structure
```

| Object | Tag (ASCII) |
|---|---|
| Identity operation (SPEC-001) | `bucks/identity-op/v1` |
| Event (SPEC-002) | `bucks/event/v1` |

**Hashes.** BLAKE3 in key-derivation mode (`derive_key(context, input)`), 256-bit output, with exactly these context strings:

| Purpose | Context string |
|---|---|
| Bucks ID derivation | `bucks 2026-10-09 identity id v1` |
| Identity operation hash | `bucks 2026-10-09 identity op hash v1` |
| Event hash | `bucks 2026-10-09 event hash v1` |
| Short code | `bucks 2026-10-09 short code v1` |

A new object type MUST get a new tag and new contexts, recorded here. A tag or context MUST NOT be reused for a different purpose.

## Known gaps

- No schema language is defined; field tables in each spec are authoritative.

## History

- 0.2: domain separation for signatures and hashes ([ADR-0013](../decisions/0013-domain-separation.md)). Protocol break; vectors regenerated.
- 0.1: initial draft from the predecessor implementation.
