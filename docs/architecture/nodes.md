# Nodes

Status: Draft. Related: [ADR-0008](../decisions/0008-federated-nodes.md), [REQ-ND-01..05](../requirements.md).

A **node** is one program that anyone can run. It offers one or more **roles**. Nodes are replaceable and federated; no node is special.

## Roles

| Role | Purpose | Trust placed in the node |
|---|---|---|
| Relay | Help devices connect through NATs and firewalls | None for content (traffic is encrypted end to end) |
| Identity directory | Store and serve identity logs; verify every operation before accepting it | None for correctness (clients re-verify). Needed for availability |
| Content pin and gateway | Pin immutable content and serve it over HTTP | None for correctness (CIDs are verified) |
| Service module: dispatch | Match riders and drivers in an area | Authority for assignment decisions in its area |
| Service module: commerce | Listings, carts, orders, reviews | Authority for order state |
| Push gateway | Wake devices (Android through an open push protocol; iOS through the platform service) | Sees that a device was woken, not content |
| Inference | Serve an AI model over a standard API | Sees prompts sent to it; user chooses |
| Web gateway | Serve the static website and installer | None |

A **minimal node** runs relay, directory and content roles. Service modules are optional add-ons.

## Node identity

A node has its own key pair and publishes a signed descriptor listing its roles, locality, API versions, contact and moderation policy. Clients pin a node's key the first time they use it. Descriptors are distributed through the signed bootstrap list and through federation.

## Discovery

1. A signed **bootstrap list** ships with the app and is updated through signed releases and DNS. Users may add or remove entries.
2. A person's identity log lists their **home nodes**.
3. Nodes exchange descriptors with neighbours (federation).

There is no hard-coded single endpoint. See [REQ-ND-03](../requirements.md).

## Locality

Service discovery is by area. A node declares the areas it serves. A client contacts the nodes for its current area. A person's identity works on every node because identity is not issued by nodes.

## Operating a node

Requirements ([REQ-ND-01](../requirements.md)): one binary, one config file, a container image, automatic TLS, metrics, backups of its small state. Target: runnable on a low-cost virtual machine or a home server. [Guessing: sizing figures need measurement in M4.]

## Abuse and moderation

Each node publishes its policy. Operators can refuse to host or relay content. Users can switch nodes. Federated nodes may decline to peer with nodes that violate their policy. This is a social layer, not a protocol guarantee, and it is a known tension with censorship resistance ([R-06](../risks.md)).

## Security properties a node must have

- Verifies signatures and stream positions before storing anything.
- Rate-limits unauthenticated requests and bounds all sizes.
- Stores no user private keys (it never receives them).
- Authenticates clients by signed challenge against the identity log, never by a password or a third-party token.
- Publishes a security contact and supports signed updates.

## What is intentionally not specified yet

HTTP and connection-level APIs, the federation protocol, push design, and module APIs. Each needs its own spec and test vectors before code ([roadmap M4](../roadmap.md)).
