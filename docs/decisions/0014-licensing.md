# ADR-0014: Licensing

- Status: Accepted
- Date: 2026-10-09
- Deciders: project owner (delegated to the maintainer acting on the owner's instruction to proceed without further approval)
- Related: OQ-01, [CONTRIBUTING](../../CONTRIBUTING.md)

## Context
No licence file existed, so all rights were reserved and nobody could legally contribute, run or redistribute the code. The predecessor website repository used MIT. The identity protocol is meant to be implemented by others, which makes an explicit patent grant valuable. The Rust ecosystem convention is a dual MIT or Apache-2.0 licence, which downstream Rust projects can consume without friction.

## Decision
- **Code** (everything not listed below): dual-licensed under **MIT OR Apache-2.0**, at the user's option. Files: `LICENSE-MIT`, `LICENSE-APACHE`.
- **Documentation and specification text**: **CC BY 4.0**. File: `LICENSE-DOCS.md`.
- **Test vectors**: additionally public-domain dedication so any implementation can embed them.
- Contributions are accepted under the same terms (inbound = outbound). No contributor licence agreement.
- Copyright holder line: "The Soul of the World contributors". The owner may replace this with a legal entity once one exists (OQ-03).

## Alternatives considered
- MIT only: simplest, matches the predecessor, but no explicit patent grant for a protocol others will implement.
- Apache-2.0 only: patent grant, but GPLv2-incompatible and heavier for small reuse.
- AGPL or a source-available licence: would discourage community nodes and independent implementations, which the architecture depends on.

## Consequences
Anyone can run nodes, ship clients and write independent implementations. The project cannot later restrict use of released versions. Changing the licence after outside contributions arrive would need every contributor's consent, so this should be revisited, if at all, before the first external pull request.

## Revisit when
A legal entity is formed (OQ-03) or counsel advises a change before the first external contribution.
