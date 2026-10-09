# ADR-0011: Specification first, with test vectors

- Status: Accepted
- Date: 2026-10-09
- Related: [specs README](../specs/README.md), REQ-QA-02

## Context
Predecessor documentation described features the code lacked. Protocols were implemented in several languages with no shared test.

## Decision
No protocol is implemented before a spec exists. Each normative spec ships committed test vectors that every implementation must reproduce. Changing a vector is a protocol break and must be labelled.

## Alternatives considered
Code-first with docs afterwards: the observed failure mode.

## Consequences
Upfront writing cost; cheap cross-implementation compatibility checks; a clear review artefact.

## Revisit when
The spec process blocks learning from prototypes; allow clearly labelled spikes in a separate directory, never merged into the core.
