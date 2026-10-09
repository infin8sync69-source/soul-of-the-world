# ADR-0002: Start fresh; predecessor repositories are read-only references

- Status: Accepted
- Date: 2026-10-09
- Related: [reference repositories](../references/reference-repos.md), [lessons learned](../references/lessons-learned.md)

## Context
An audit of six repositories and a shipped desktop archive found overlapping, partly incompatible code, committed secrets, user keys held by servers, and documentation that overstated reality. Merging them would carry the defects forward and the history is mostly squashed already.

## Decision
This repository starts from documentation. Code from the predecessors is **referenced**, not merged. Anything reused is re-reviewed, re-tested and re-committed with its provenance noted. The predecessor repositories are frozen and not extended.

## Alternatives considered
- Consolidate by merging repositories: imports secrets, binaries and contradictory designs.
- Incrementally refactor the mobile repository into the platform: viable for the app, but would leave the rest of the platform shaped by its backend-as-a-service dependencies. The mobile app's logic is still a primary reference.

## Consequences
Slower first visible progress; fewer inherited mistakes. Some work is redone on purpose.

## Revisit when
A reference repository turns out to be reusable wholesale after review (record the exception here).
