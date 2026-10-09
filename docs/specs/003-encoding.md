# SPEC-003: Deterministic encoding

Status: Draft. Version 0.1. Used by [SPEC-001](001-identity.md) and [SPEC-002](002-events.md).

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

## 5. Hashing and signing inputs

Hash function: BLAKE3 with 256-bit output. Signature algorithm: ECDSA over NIST P-256 with SHA-256 (see SPEC-001). The signed message is the canonical unsigned encoding of the structure.

## Known gaps

- **No domain separation.** The signed message carries no prefix saying which protocol object it is. Operations and events are distinguishable by their fields and by the keys that sign them, but a signature made over arbitrary bytes by a key that is also exposed through a signing API could be misused. Recommendation: add a fixed prefix per object type before the first deployment. This is a protocol break relative to the current vectors; see [OQ-11](../open-questions.md).
- No schema language is defined; field tables in each spec are authoritative.
