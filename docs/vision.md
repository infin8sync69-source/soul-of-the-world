# Vision

Status: Draft. Owner: project owner. Last reviewed: 2026-10-09.

## The problem

A person's digital life depends on a few companies and a handful of servers. Their identity is an account someone else issued and can revoke. Their messages, orders and location history sit in databases they cannot move. If a company changes its terms, is blocked, or fails, the person has no recourse and no way to take their relationships with them.

Existing "decentralised" projects often solve a narrower problem (a token, a social feed) and still depend on a central issuer, a central cloud, or a single operator for the parts people actually use daily: signing in, finding a local service, getting a ride.

## The vision

One person has **one identity** that no company issues. It lives on devices they control, links those devices together, and works across phones, desktops and the web. Services (messaging, local commerce, mobility, an AI assistant) are provided by **replaceable community-run nodes**. A person can change nodes, or run one, without losing who they are or what they own.

## Principles

| # | Principle | What it means in practice |
|---|---|---|
| P1 | The user owns the keys | Private keys are generated on the user's device and never leave it. No server stores, escrows or can reconstruct a user's key. |
| P2 | Nobody issues identity | An identifier is derived from the user's own signed data. No registry, company or node can grant or revoke it. |
| P3 | Local-first | The app works offline on the data it has. The network synchronises; it does not gate. |
| P4 | Infrastructure is replaceable | Any node can be swapped for another. Any single external service can fail without ending the system. |
| P5 | Open specifications with test vectors | A second team can implement the protocol from the documents and prove compatibility. |
| P6 | Honest security | Threats are written down. Known gaps are listed next to features. Nothing is called secure without evidence. |
| P7 | Every dependency is a recorded decision | External services are classified, contained, and given an exit plan. |
| P8 | Small verified steps | Each milestone ships something that works and leaves earlier work intact. |

## Non-goals

- **Not a new blockchain or token.** Integrity needs (timestamps, anchoring) are met without one for now. See [ADR-0009](decisions/0009-no-custom-chain.md).
- **Not a clone of an existing social network.** Social features exist to support trust between people and local services.
- **Not custodial.** The project will not hold users' keys or funds.
- **Not gated by belief or affiliation.** Admission to the network depends on signatures and community trust mechanisms, not on agreeing to a text. See [OQ-05](open-questions.md).
- **Not a promise of anonymity.** Pseudonymity is possible; anonymity against a global network observer is out of scope.

## Intended users and context

[Likely] First users are in India: the most mature predecessor code (Android and iOS apps) targets Bengaluru, uses UPI payments and phone-number onboarding. Whether India is the launch market is an open decision ([OQ-04](open-questions.md)). Requirements are written to avoid hard-coding that assumption.

## What "done" looks like

A person installs the app on a phone, creates an identity in under a minute, scans a code on their laptop to add it as a second device, messages a friend end to end, orders from a local business, and does all of it through a node run by their own community. If that node disappears they point their identity at another and carry on. Nothing in that sequence required a company's permission.

## The word "Soul"

The predecessor desktop app used "Soul" for a per-install agent identity and persona. In this project, **Soul** names the user-facing AI assistant and its configurable values/persona layer. It is not an identity primitive and not a network admission rule. Decided in [ADR-0016](decisions/0016-soul-is-a-persona-not-a-gate.md); see also the [glossary](glossary.md).
