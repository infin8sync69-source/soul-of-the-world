# ADR-0017: Review rules during the solo-maintainer phase

- Status: Accepted
- Date: 2026-10-09
- Related: [engineering handbook](../engineering/handbook.md), OQ-03, OQ-09, R-13

## Context
The handbook requires a second human reviewer on every pull request. Today there is one maintainer and an AI assistant acting on the owner's instruction. Waiting for a reviewer who does not exist would stop all work; merging without any independent check repeats the predecessor's mistakes.

## Decision
Until a second maintainer exists:

1. Every change still goes through a pull request on a branch, never a direct push to `main`.
2. CI (docs, hygiene, core) must be green before merge. CI is the mandatory reviewer.
3. Protocol, cryptography and security-relevant changes get an **adversarial automated review** before merge: independent reviewers prompted to refute each finding, with confirmed findings fixed or recorded as known gaps. The review summary is linked from the pull request.
4. Merges are squash merges by the maintainer. Self-merge is allowed under these conditions and is recorded here so nobody mistakes it for the long-term rule.
5. The first external contributor triggers a return to two-person review for protocol and security changes.

Governance (OQ-03): the repository owner is the sole decision-maker and release authority for now. Security reports (OQ-09) go through GitHub private vulnerability reporting on this repository, which the owner enables in the repository's security settings; the maintainer cannot enable it through the API.

## Alternatives considered
- Keep the two-reviewer rule and stall: no progress.
- Direct pushes to `main`: loses CI gating and history.

## Consequences
Lower assurance than human peer review, stated openly. Reviews are automated and adversarial rather than absent.

## Revisit when
A second maintainer joins, or the first external pull request arrives.
