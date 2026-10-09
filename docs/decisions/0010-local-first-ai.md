# ADR-0010: Local-first AI; remote providers opt-in

- Status: Accepted
- Date: 2026-10-09
- Related: REQ-AI-01..04, [clients](../architecture/clients.md)

## Context
The shipped desktop app defaulted to a cloud inference provider and instructed the model not to reveal which model was in use. The mobile app embedded a cloud model key in the application package. Both conflict with privacy and sovereignty goals.

## Decision
The default path is deterministic rules, then a local model, then a node or provider the user chose. Remote providers are named in the UI and need the user's own credentials. The assistant proposes actions; deterministic code validates and executes them; the confirmation gate applies. Prompts must not hide the model identity.

## Alternatives considered
- Cloud model as default: better quality, but breaks privacy and offline use.
- No assistant: loses a differentiating feature.

## Consequences
Lower quality on weak devices; model distribution needs mirrors on nodes. Assistant behaviour is testable because action execution is deterministic.

## Revisit when
On-device models are good enough that remote use becomes rare, or user research shows opt-in remote use is the norm and needs a better flow.
