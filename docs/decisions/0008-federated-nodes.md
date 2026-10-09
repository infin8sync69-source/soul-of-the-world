# ADR-0008: Federated, replaceable nodes; service logic in modules

- Status: Accepted
- Date: 2026-10-09
- Related: [nodes](../architecture/nodes.md), REQ-ND-01..05, REQ-SY-04

## Context
The most valuable predecessor logic (dispatch, commerce, jobs, social, trust) lives as about 120 database functions on a hosted backend-as-a-service, guarded by row-level policies and an identity provider. That ties business logic to a vendor and to its identity. Some functions need one authority (ride assignment).

## Decision
Define a **node** that anyone can run, offering roles (relay, directory, pinning, push, inference, service modules). Port the existing database schema and rules into a **service module** that runs on a node, with identity based on Bucks IDs and signed-challenge authentication. Nodes federate by locality; people choose their home nodes.

## Alternatives considered
- Pure peer-to-peer with no coordinator: cannot provide single-authority decisions for dispatch and orders without complex consensus.
- A single project-run backend: simple, but a single point of control and failure.
- Stay on the hosted backend: contradicts the project's purpose.

## Consequences
Operating a node is a real task; documentation and packaging matter (REQ-ND-01). Moderation is local policy per node. The module boundary needs a spec before code.

## Revisit when
A practical decentralised coordination method for dispatch appears, or node operation proves too hard for communities.
