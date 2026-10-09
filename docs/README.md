# Documentation map

Every document has a status line. **Draft** means open to change, **Accepted** means agreed and binding until superseded, **Superseded** points to its replacement.

| Area | Document | Status |
|---|---|---|
| Purpose | [vision.md](vision.md) | Draft |
| Purpose | [glossary.md](glossary.md) | Draft |
| What it must do | [requirements.md](requirements.md) | Draft |
| Design | [architecture/overview.md](architecture/overview.md) | Draft |
| Design | [architecture/data-and-sync.md](architecture/data-and-sync.md) | Draft |
| Design | [architecture/nodes.md](architecture/nodes.md) | Draft |
| Design | [architecture/clients.md](architecture/clients.md) | Draft |
| Design | [architecture/sovereignty.md](architecture/sovereignty.md) | Draft |
| Protocols | [specs/README.md](specs/README.md) | Draft |
| Protocols | [specs/001-identity.md](specs/001-identity.md) | Draft |
| Protocols | [specs/002-events.md](specs/002-events.md) | Draft |
| Protocols | [specs/003-encoding.md](specs/003-encoding.md) | Draft |
| Security | [security/threat-model.md](security/threat-model.md) | Draft |
| Security | [security/secrets-policy.md](security/secrets-policy.md) | Accepted |
| Decisions | [decisions/README.md](decisions/README.md) | Accepted |
| Delivery | [roadmap.md](roadmap.md) | Draft |
| Delivery | [risks.md](risks.md) | Draft |
| Process | [engineering/handbook.md](engineering/handbook.md) | Draft |
| History | [references/reference-repos.md](references/reference-repos.md) | Accepted |
| History | [references/lessons-learned.md](references/lessons-learned.md) | Accepted |
| Open items | [open-questions.md](open-questions.md) | Living |

## Reading order

New to the project: vision, requirements, architecture overview, lessons learned.
Implementing a protocol: specs README, then the spec, then the test vectors in `spec/test-vectors`.
Reviewing risk: threat model, risks, open questions.

## Conventions

- Requirement IDs look like `REQ-ID-03`. Threat IDs look like `T-07`. Risk IDs look like `R-04`. Open questions look like `OQ-05`. Decisions are `ADR-0003`. Specs are `SPEC-001`.
- Confidence tags: **[Certain]** verified directly, **[Likely]** strong inference, **[Guessing]** a placeholder for an unknown.
- Normative words (MUST, SHOULD, MAY) follow [RFC 2119](https://www.rfc-editor.org/rfc/rfc2119) and appear in capitals only in specifications and requirements.
