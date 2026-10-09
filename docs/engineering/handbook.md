# Engineering handbook

Status: Draft. Applies to everyone changing this repository. Rationale for most rules is in [lessons learned](../references/lessons-learned.md).

## 1. Repository layout

```
docs/       documentation (decisions, specs, architecture, security)
spec/       normative test vectors, later schemas
core/       Rust workspace: core library, CLI, bindings
apps/       android, ios, desktop, web
node/       node binary and service modules
infra/      deployment, CI helpers, release tooling
scripts/    repository tooling
```

Directories appear only when work in them starts. Empty placeholders are not committed.

## 2. Branching and review

- Trunk-based. `main` is always releasable once code exists. Branch names: `type/short-topic` (`docs/identity-gaps`, `feat/core-events`).
- Every change is a pull request with at least one reviewer who is not the author. Specs and security-sensitive code need a reviewer who has read the threat model.
- Squash-merge. The squash title follows Conventional Commits.
- Do not merge with failing CI. Do not merge your own change without a second reviewer, even for docs, until the team is large enough to relax this by an ADR.

## 3. Commit messages

Conventional Commits: `docs:`, `spec:`, `feat:`, `fix:`, `refactor:`, `test:`, `chore:`, `ci:`. Mark protocol breaks with `spec!:` and a `BREAKING CHANGE:` footer. Explain why, not only what.

## 4. Definition of done

A change is done when:

1. The behaviour is specified (or the existing spec is updated).
2. Automated tests cover it and run in CI. For protocols, vectors are updated.
3. The threat model was consulted; new threats are added.
4. New dependencies are in the [dependency register](../architecture/sovereignty.md).
5. Documentation is updated and `python3 scripts/check_docs.py` passes.
6. No secrets, no machine-specific paths, no large binaries.
7. User-visible claims are backed by a test or a measurement.

## 5. Testing strategy

| Layer | What | Where it runs |
|---|---|---|
| Unit | Pure logic: encoding, verification, policy | Every pull request |
| Vectors | Reproduce committed vectors | Every pull request |
| Property and fuzz | Decoders and verifiers with arbitrary input | Every pull request (short), nightly (long) |
| Integration | Core with a real store and two simulated devices; node with a real client | Every pull request |
| Device | Keystore and Secure Enclave flows on real devices | Before each milestone exit, then on a schedule |
| End to end | Create identity, add device, send event, recover | Before each milestone exit |
| Security | Dependency audit, secret scan, abuse cases from the threat model | Every pull request, plus periodic review |

A bug fix starts with a failing test.

## 6. CI gates

1. Documentation check (`scripts/check_docs.py`): links, IDs, ADR format, vector files parse, secret patterns.
2. Secret scanning (a maintained scanner) on the diff and on history for protected branches.
3. Unit, vector and fuzz tests.
4. Formatting and linting.
5. Repository size and file-type check: reject files over a size limit and archive or binary types outside an allow-list.
6. Dependency register check: any new external host or package source must appear in the register.
7. **Build-output checks** for packaging-sensitive code (browser extensions, mobile apps): assert the structure of the built artifact (for example, content scripts contain no module imports; the app bundle includes only declared libraries). Source tests are not enough.

Implemented now: gates 1 (`docs` workflow), 2 (`hygiene` workflow, gitleaks), 3 and 4 (`core` workflow), 5 (`scripts/check_repo.py`), plus an independent vector check. Not yet: gate 6 (dependency register check) and gate 7 (no packaged artifacts yet).

## 7. Code standards

- Use vetted cryptographic libraries. Do not invent primitives or protocols ([L-12](../references/lessons-learned.md)).
- Stubs and unimplemented paths return explicit errors; they never report success ([L-16](../references/lessons-learned.md)).
- Parse untrusted input with bounded sizes, then verify, then use.
- No `unsafe` code without a comment explaining the invariant and a reviewer's sign-off.
- Public APIs are documented. Error types are specific, not strings, where callers act on them.
- Logging never includes keys, tokens, message content or precise location.

## 8. Versioning and releases

- Each component follows SemVer. Protocol versions are separate (the `v` field in encoded data and the spec version).
- Release steps: tag, build in CI, run the full gate set, sign, publish the manifest with hashes, pin to at least two nodes, announce. The signing procedure keeps keys out of CI ([secrets policy](../security/secrets-policy.md)).
- A release note states what changed, known issues, and protocol-break status.

## 9. Documentation rules

- Every document has a status line. A document in conflict with a newer ADR is marked Superseded.
- Use stable IDs (REQ, SPEC, ADR, T, R, OQ, L). Do not renumber.
- Say what is verified. Tag uncertain statements.
- Prefer a table or a numbered rule over prose when stating requirements.

## 10. Incidents

A security or availability incident gets a written record (what, when, impact, cause, fix, follow-ups) without sensitive values. Credential exposure follows the [secrets policy](../security/secrets-policy.md): rotate first.

## 11. Tooling

Pin the Rust toolchain and tool versions in the repository when code starts. Document how to build and test on a fresh machine in the README of each component and keep that file verified by CI where possible.
