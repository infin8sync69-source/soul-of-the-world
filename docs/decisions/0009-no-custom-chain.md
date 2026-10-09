# ADR-0009: No custom blockchain or token for now

- Status: Accepted
- Date: 2026-10-09
- Related: [vision](../vision.md), T-04, T-22

## Context
The predecessor chain (C++) did not enforce its monetary rules: block rewards were unchecked, duplicate inputs were accepted, fork choice ignored accumulated work, transactions did not relay, and wallet endpoints were unauthenticated. A second chain node (Go) and Solidity contracts were added later and have not been reviewed here. No user-facing feature depended on any chain beyond a demonstration wallet. Payments in the target market run on UPI and cash.

## Decision
Do not build or operate a chain or token. Meet integrity needs with signed events, verifiable logs, and optional external timestamping of directory state. The predecessor chain code is archived, not reused.

## Alternatives considered
- Fix and ship the C++ chain: large security and consensus effort with no user need.
- Adopt a general-purpose chain SDK now: premature without a requirement.
- Use an existing public chain for settlement: adds dependency and fees without need.

## Consequences
Removes a large attack surface and a source of overclaims. Escrow and staking ideas wait for a concrete requirement.

## Revisit when
A requirement appears that signed receipts and node-run escrow cannot meet (for example, cross-node settlement between parties who do not trust any node). Then evaluate existing chains and SDKs, with a threat model and independent review first.
