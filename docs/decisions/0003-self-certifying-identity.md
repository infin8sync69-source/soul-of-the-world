# ADR-0003: Self-certifying identity

- Status: Accepted
- Date: 2026-10-09
- Related: [SPEC-001](../specs/001-identity.md), REQ-ID-01..03

## Context
The requirement is one global identifier per user. A random UUID needs an issuer, and whoever issues it owns the users. The predecessors issued identity through a hosted phone-OTP provider, a server that generated keys, and a stub. A bare public-key identifier cannot rotate, so losing the key loses the identity.

## Decision
The Bucks ID is a UUID derived from the BLAKE3 hash of the unsigned **inception operation** of an **identity log**. Later operations (add or remove device, rotate keys) are signed by rotation keys and chained by hash. The ID does not change when keys change.

## Alternatives considered
- Random UUID issued by a node: an issuer owns the users.
- Public key as identity: no rotation or multi-device.
- `did:plc`: a single directory operator; the operation and rotation design is borrowed.
- Blockchain account: introduces a chain dependency (see ADR-0009).

## Consequences
Every implementation needs the same verifier and vectors. Directory availability matters for resolving unknown identities. Recovery needs a separate design. Known gaps are listed in SPEC-001.

## Revisit when
A wider standard for portable identity appears that meets the requirements without an operator, or cryptanalytic results weaken the primitives.
