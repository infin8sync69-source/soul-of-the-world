# Security policy

## Reporting a vulnerability

Do **not** open a public issue for a vulnerability. Use GitHub's private vulnerability reporting for this repository (Security tab, "Report a vulnerability"). If that is unavailable, contact the repository owner privately through their GitHub profile. The repository owner is the security contact during the solo-maintainer phase ([ADR-0017](docs/decisions/0017-solo-maintainer-review.md)); a dedicated address is still to be set ([OQ-09](docs/open-questions.md)).

Include: affected component and version, impact, reproduction steps, and whether the issue is already public. Expect an acknowledgement within 3 working days. [Guessing: realistic for a small team; revisit when staffed.]

## Scope

There is no released software yet. In scope today: the specifications (a protocol flaw is a vulnerability), the documentation (a recommendation that would lead to insecure code), and any secret accidentally committed to this repository.

## If you find a secret in this repository

Report it privately at once. The maintainers will rotate it first, then remove it from history. Removing it from the latest commit does not make it safe.

## Related documents

[Threat model](docs/security/threat-model.md) and [secrets policy](docs/security/secrets-policy.md).
