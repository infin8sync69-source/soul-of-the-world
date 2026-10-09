# ADR-0007: iroh for connections, content-addressed data for public content

- Status: Accepted
- Date: 2026-10-09
- Related: [nodes](../architecture/nodes.md), REQ-ND-03, REQ-SO-03

## Context
Devices and nodes must connect across NATs without a central service. The predecessors used Helia with public bootstrap nodes and a shared default topic secret (so every install joined the same public mesh), a Kubo daemon, and a hosted realtime service. Mobile had no peer-to-peer path.

## Decision
Use **iroh** (QUIC, dial-by-public-key, hole punching, relay fallback; reported stable at 1.0 in June 2026, with Android and iOS bindings) for device-to-device and device-to-node streams. Use IPFS-style **CIDs** for public immutable content, computed by the core so all implementations agree, served by nodes. Relays are run by nodes. There is no default dependency on public bootstrap nodes; a signed list of project nodes is shipped.

## Alternatives considered
- libp2p everywhere: heavy on phones; discovery pitfalls seen in the predecessor.
- A hosted realtime service: a single provider holding the traffic.
- Matrix or Nostr relays as the transport: useful ideas, but the data model does not fit local commerce and mobility.

## Consequences
Relays are infrastructure to operate. The iroh bindings for each platform must be validated in M2. [Likely: the 1.0 and mobile-binding claims rely on third-party sources and must be re-checked against the project's own release notes before building on them.]

## Revisit when
iroh's maturity, licence or mobile support changes, or a better option appears.
