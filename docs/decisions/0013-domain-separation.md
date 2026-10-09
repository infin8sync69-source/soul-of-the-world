# ADR-0013: Domain separation for every signature and hash

- Status: Accepted
- Date: 2026-10-09
- Related: [SPEC-003](../specs/003-encoding.md), [SPEC-001](../specs/001-identity.md), [SPEC-002](../specs/002-events.md), T-06, OQ-11

## Context
In SPEC v0.1 an identity operation and an event were signed over their raw canonical bytes, and hashes were plain BLAKE3. Nothing in a signature said what was being signed. If a key were ever exposed through a generic signing interface (the predecessor chain had exactly such an endpoint), or reused across roles, a signature made for one purpose could be presented for another. No deployment existed, so a protocol break cost nothing yet.

## Decision
- Signatures cover `tag || 0x00 || canonical unsigned bytes`, with tags `bucks/identity-op/v1` and `bucks/event/v1`.
- Hashes use BLAKE3 key-derivation mode with one fixed context string per purpose: Bucks ID, operation hash, event hash, short code.
- Implementations normalise signatures to low-S before emitting them, because platform keystores may return high-S, and still reject high-S on the wire.
- Vectors were regenerated (protocol break relative to v0.1) and are verified by an independent implementation (`scripts/verify_vectors.py`).

## Alternatives considered
- Rely on structural differences between objects: fragile; any future object type could collide.
- A version byte only: says which version, not which object.
- Separate keys per role only: helps, but cannot be enforced on a hardware keystore that exposes a general signing API.

## Consequences
Signatures and hashes from v0.1 are invalid. Every implementation must use the exact tag and context strings. New object types need new tags and contexts recorded in SPEC-003.

## Revisit when
A standard signing-envelope format is adopted for interoperability (for example, with another identity ecosystem).
