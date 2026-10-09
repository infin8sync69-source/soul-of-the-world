# ADR-0015: Names and identifiers

- Status: Accepted
- Date: 2026-10-09
- Related: OQ-02, [SPEC-001](../specs/001-identity.md), [SPEC-003](../specs/003-encoding.md)

## Context
The repository is `soul-of-the-world`, the predecessor products were all called "Bucks" something, the domain `bucks.global` exists, and the protocol identifiers (`did:bucks:`, event kinds `bucks.*`, signature tags `bucks/...`) are already embedded in test vectors. Changing a protocol identifier later is a protocol break.

## Decision
- **Project name:** Soul of the World (the initiative, this repository, the documentation).
- **Product and brand name:** Bucks (the app on every platform, "Bucks ID", "Bucks Node").
- **Protocol identifiers:** `bucks` is frozen: `did:bucks:`, event kinds under `bucks.`, signature tags `bucks/...`, hash contexts `bucks ...`. These do not change if the brand changes.
- **Assistant name:** Soul (see ADR-0016).
- **Domain:** `bucks.global` remains the public domain; it is registered to the founder's other account and is a contained dependency (see the sovereignty register).

## Alternatives considered
- Rename the protocol to `sotw`: a protocol break for no user benefit.
- Use the repository name as the brand: long, and the apps are already known as Bucks to early users.

## Consequences
Documentation uses "Bucks" for the product and "Soul of the World" for the project. Store listings and the website follow the brand. The protocol keeps working under any future rebrand.

## Revisit when
Trademark search or counsel (OQ-04) finds a conflict with "Bucks" in a launch market.
