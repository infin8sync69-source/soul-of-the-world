# Roadmap

Status: Draft. Order matters; sizes are relative and [Guessing] until M1 gives a measured baseline. No dates are promised. A milestone is complete only when every exit criterion is verified.

```
M0 Foundations -> M1 Identity core -> M2 Mobile identity -> M3 Desktop
                                   \-> M4 Node v0 -> M5 Service migration -> M6 Federation
```

M3 and M4 can proceed in parallel after M1 and M2. M5 needs M4.

## M0: Foundations (this phase)

**Goal:** a repository others can contribute to without repeating the predecessors' mistakes.

| Deliverable | Notes |
|---|---|
| Documentation set | This repository |
| Test vectors committed | `spec/test-vectors` |
| Documentation checker and CI | `scripts/check_docs.py`, workflow |
| Secret scanning in pre-commit and CI | REQ-SE-01 |
| Dependency register checked in CI | REQ-SO-01 |
| Owner decisions: licence, name, security contact | [OQ-01, OQ-02, OQ-09](open-questions.md) |

**Exit:** all documents reviewed by one other person; CI green; no open blocker among OQ-01, OQ-02, OQ-09.

## M1: Identity core

**Goal:** the identity and event protocols exist as a tested Rust library and a command-line tool.

| Deliverable | Notes |
|---|---|
| Resolve [OQ-11](open-questions.md) (domain separation) and regenerate vectors | Before any deployment |
| Core crate: encoding, identity log, events, verification | Re-import and re-review the predecessor crate |
| Fuzz tests for the decoder | T-23 |
| CLI: create identity, add device, rotate, sign and verify event | For manual and automated testing |
| Second implementation or independent review of the vectors | Spec acceptance rule |
| Specs 001 to 003 at 0.2 with known gaps updated | |

**Exit:** vectors reproduced by an independent check; SPEC-001..003 reviewed against the threat model; CI runs tests and fuzzers; no protocol break pending.

## M2: Mobile identity

**Goal:** the Android and iOS apps hold the identity in hardware and use the core.

| Deliverable | Notes |
|---|---|
| Signer interface implemented on Android Keystore and Secure Enclave | REQ-ID-04 |
| Generated bindings from the core | ADR-0005 |
| Create identity, backup key, add and remove device flows | REQ-ID-05..07 |
| Bucks ID card screen | Same on every platform |
| Remove client-held identity code from the apps | REQ-CL-01 |
| Offline outbox for events | REQ-SY-01 |

**Exit:** device test matrix passes on at least two Android and two iOS models; recovery drill performed; no identity code remains in app code outside the core binding.

## M3: Desktop

**Goal:** a cross-platform desktop app with the core, local assistant and signed releases.

| Deliverable | Notes |
|---|---|
| Tauri shell with core, add-device by scan | |
| Local assistant with local model default | REQ-AI-01..04 |
| Messaging end to end between phone and desktop | Needs a minimal relay (M4 stub) |
| Token-authenticated local services | REQ-SE-02 |
| Signed installers for macOS, Windows, Linux; verifying installer script | REQ-SE-03 |

**Exit:** install on a clean machine of each OS from the project site; assistant works with the network off; signature verification demonstrated, including a tampered download being rejected.

## M4: Node v0

**Goal:** anyone can run a node that serves identity directory, relay and content.

| Deliverable | Notes |
|---|---|
| SPEC-005 node API, SPEC-006 federation draft | Before code |
| Node binary, container image, configuration reference | REQ-ND-01 |
| Identity directory with verification and mirror sync | T-08, T-22 |
| Signed bootstrap list in the clients | REQ-ND-03 |
| Override window and fork rules in SPEC-001 | [OQ-07](open-questions.md), REQ-ID-08 |
| Two nodes run by different people | Proves replaceability |

**Exit:** a fresh machine becomes a node by following the guide; clients fail over between two nodes; a node serving a stale log is detected.

## M5: Service migration

**Goal:** the predecessor's dispatch, commerce, social and jobs logic runs on nodes with Bucks ID authentication, and the mobile apps no longer need the hosted backend or phone-OTP identity provider.

| Deliverable | Notes |
|---|---|
| Event type registry (SPEC-004) and module spec | |
| Port schema and rules; replace identity column and auth | ADR-0008 |
| Open push path for Android; wake-only iOS push | |
| Attestation verification (device, phone, community) | REQ-ID-09 |
| Moderation policy mechanism | REQ-ND-05 |
| Data export | REQ-DA-06 |

**Exit:** end-to-end ride and order flows pass on a node with none of the predecessor's hosted services; the dependency register shows no class C entries.

## M6: Federation

**Goal:** several communities run nodes that interoperate.

| Deliverable | Notes |
|---|---|
| Locality discovery and node descriptors | |
| Home-node change operation and migration | REQ-ND-02 |
| Isolation test: only Bucks nodes reachable | REQ-SO-03 |
| Optional timestamp anchoring of directory roots | ADR-0009 |

**Exit:** a person moves from one community's node to another without changing identity and without losing data; the isolation test passes.

## Out of scope until a reason appears

A token or chain (ADR-0009), a name registry, a marketplace for third-party apps, post-quantum signatures (watch the field).

## Cross-cutting, every milestone

Tests before ports. Docs that match code. Threat model review per protocol change. Dependency register kept current. Privacy policy, terms and moderation tooling before any public store release ([R-05](risks.md)).
