# Contributing

This project is in its documentation phase. The most valuable contributions are corrections, missing threat scenarios, and answers to [open questions](docs/open-questions.md).

## Ground rules

1. **Docs are code.** Documentation changes go through a pull request and pass `python3 scripts/check_docs.py`.
2. **Decisions are recorded.** A change to architecture, a protocol, or an external dependency needs an ADR (see [docs/decisions](docs/decisions/README.md)). Do not bury decisions in prose.
3. **Specs ship with vectors.** A change to a normative spec must update [spec/test-vectors](spec/test-vectors) in the same pull request, and say whether it is a protocol break.
4. **No secrets, ever.** Not in code, docs, examples, commit messages or issue text. See [docs/security/secrets-policy.md](docs/security/secrets-policy.md).
5. **Claims need evidence.** Tag uncertain statements `[Likely]` or `[Guessing]`. Do not write "production ready", "secure" or "decentralised" without a cited test or measurement.
6. **One change, one purpose.** Small pull requests with a clear title. Use [Conventional Commits](https://www.conventionalcommits.org/) (`docs:`, `spec:`, `feat:`, `fix:`, `chore:`).

## Workflow

Trunk-based. Branch from `main`, keep branches short-lived, open a pull request, get one review, squash-merge. Details in [docs/engineering/handbook.md](docs/engineering/handbook.md).

## Proposing something large

Open an issue titled `RFC: <topic>` describing the problem, options and a recommendation, then convert the outcome to an ADR.
