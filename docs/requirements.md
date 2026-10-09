# Requirements

Status: Draft. Each requirement has an ID, a priority (MUST, SHOULD, MAY) and a target milestone from the [roadmap](roadmap.md). A requirement is "met" only when a test or measurement says so; the verification column says what that is.

Milestones: M1 identity core, M2 mobile identity, M3 desktop, M4 node v0, M5 service migration, M6 federation. `later` means unscheduled.

## Identity

| ID | Requirement | Pri | Milestone | Verification |
|---|---|---|---|---|
| REQ-ID-01 | Each person MUST have exactly one Bucks ID derived from their own inception operation, with no issuer. | MUST | M1 | SPEC-001 vectors |
| REQ-ID-02 | The Bucks ID MUST NOT change when keys rotate, devices are added or removed, or the home node changes. | MUST | M1 | Unit test: rotate then compare ID |
| REQ-ID-03 | Private keys MUST be generated on the user's device and MUST NOT be transmitted, escrowed or reconstructable by any server. | MUST | M1 | Code review gate; no key export API in core; threat T-01 |
| REQ-ID-04 | Device keys SHOULD be hardware-backed (Android Keystore/StrongBox, Apple Secure Enclave). Software keys MUST be marked as such in the log's attestations. | SHOULD | M2 | Device test matrix |
| REQ-ID-05 | A user MUST be able to add a second device by scanning a code on the first and approving with biometrics. | MUST | M2 | End-to-end test |
| REQ-ID-06 | A user MUST be able to remove a lost device from another device. | MUST | M2 | End-to-end test |
| REQ-ID-07 | A user MUST be able to recover the identity after losing all devices, using a backup rotation key or social recovery. | MUST | M2 | Recovery drill |
| REQ-ID-08 | A higher-priority rotation key SHOULD be able to override a lower-priority one within a bounded window, so a stolen backup key is not equal to the primary. | SHOULD | M4 | Spec + tests (needs [OQ-07](open-questions.md)) |
| REQ-ID-09 | Verification levels (device genuine, phone, community, document) MUST be signed attestations by third parties, never self-asserted. | MUST | M5 | Spec + tests |
| REQ-ID-10 | Handles or display names MUST NOT be identity. They are hints and MAY collide. | MUST | M2 | Review |

## Data and events

| ID | Requirement | Pri | Milestone | Verification |
|---|---|---|---|---|
| REQ-DA-01 | All user-generated data MUST be representable as signed events ([SPEC-002](specs/002-events.md)). | MUST | M1 | Spec |
| REQ-DA-02 | Events MUST be verifiable offline given the author's identity log. | MUST | M1 | Unit tests |
| REQ-DA-03 | Every signed structure MUST have exactly one valid byte encoding ([SPEC-003](specs/003-encoding.md)). | MUST | M1 | Canonicality tests |
| REQ-DA-04 | Private content MUST be end-to-end encrypted and MUST NOT be stored unencrypted on any node or IPFS. | MUST | M3 | Design review, tests |
| REQ-DA-05 | Public content SHOULD be content-addressed so any holder can serve it and any reader can verify it. | SHOULD | M4 | CID tests |
| REQ-DA-06 | Users MUST be able to export all their data in an open, documented format. | MUST | M5 | Export test |

## Sync and offline

| ID | Requirement | Pri | Milestone | Verification |
|---|---|---|---|---|
| REQ-SY-01 | The apps MUST be usable offline for reading and composing, and MUST queue outgoing events. | MUST | M2 | Airplane-mode test |
| REQ-SY-02 | A user's devices MUST converge to the same set of their own events after connectivity returns. | MUST | M3 | Sync tests |
| REQ-SY-03 | A node MUST reject unverifiable events and MUST NOT rely on client claims. | MUST | M4 | Node tests |
| REQ-SY-04 | Domain state that needs a single authority (for example, which driver accepted a ride) MUST be decided by a service module on a node, with signed events as inputs and receipts. | MUST | M5 | Design + tests |

## Nodes and federation

| ID | Requirement | Pri | Milestone | Verification |
|---|---|---|---|---|
| REQ-ND-01 | A node MUST be deployable by a non-expert from one binary and one configuration file. | MUST | M4 | Fresh-machine install test |
| REQ-ND-02 | A user MUST be able to change home node with one identity-log operation and without losing data they exported or replicated. | MUST | M6 | Migration test |
| REQ-ND-03 | The client MUST work with any conformant node; no node address may be hard-coded beyond a signed, replaceable bootstrap list. | MUST | M4 | Config review |
| REQ-ND-04 | Nodes SHOULD be able to serve a locality (service discovery by area) and federate with neighbours. | SHOULD | M6 | Federation test |
| REQ-ND-05 | Node operators MUST be able to set and publish moderation policy for content they host. | MUST | M5 | Policy doc + tests |

## Clients

| ID | Requirement | Pri | Milestone | Verification |
|---|---|---|---|---|
| REQ-CL-01 | Android and iOS apps MUST use the shared core for identity, events and verification. They MUST NOT reimplement it. | MUST | M2 | Dependency check |
| REQ-CL-02 | The desktop app MUST run on macOS, Windows and Linux. | MUST | M3 | CI matrix |
| REQ-CL-03 | Money-moving actions MUST require explicit confirmation, and biometric confirmation above a configurable threshold. | MUST | M2 | UI test |
| REQ-CL-04 | The web presence SHOULD be static and mirrorable, and MUST NOT generate or hold keys. | SHOULD | M3 | Review |

## AI assistant (Soul)

| ID | Requirement | Pri | Milestone | Verification |
|---|---|---|---|---|
| REQ-AI-01 | The default AI path MUST run locally or on a node the user selected. | MUST | M3 | Network-off test |
| REQ-AI-02 | Any remote AI provider MUST be named in the UI and MUST require the user's own credentials. It MUST NOT be enabled silently. | MUST | M3 | UI review |
| REQ-AI-03 | The assistant MUST only propose actions. Deterministic code validates and executes them, and the confirmation gate applies. | MUST | M3 | Tests |
| REQ-AI-04 | The assistant MUST NOT be instructed to conceal what model or provider it is running on. | MUST | M3 | Prompt review |

## Security and privacy

| ID | Requirement | Pri | Milestone | Verification |
|---|---|---|---|---|
| REQ-SE-01 | No secret MAY be committed to any repository. A secret scanner MUST run in CI and pre-commit. | MUST | M0 | CI |
| REQ-SE-02 | Local service ports (agent, tool server) MUST require a per-launch token and MUST NOT accept cross-origin browser requests. | MUST | M3 | Security tests |
| REQ-SE-03 | Release artifacts MUST be signed and verified by the installer against a pinned key. | MUST | M3 | Install test |
| REQ-SE-04 | Telemetry MUST be off by default and MUST NOT include identifiers or content. | MUST | M2 | Review |
| REQ-SE-05 | Every protocol change MUST be reviewed against the [threat model](security/threat-model.md). | MUST | M0 | PR template |
| REQ-SE-06 | Location data MUST be coarsened before leaving the device unless a ride or delivery is active. | MUST | M5 | Tests |

## Sovereignty

| ID | Requirement | Pri | Milestone | Verification |
|---|---|---|---|---|
| REQ-SO-01 | Every external service MUST appear in the [dependency register](architecture/sovereignty.md) with a class and an exit plan. | MUST | M0 | CI check |
| REQ-SO-02 | A build for Android MUST be possible without proprietary Google libraries. | SHOULD | M5 | F-Droid-style build |
| REQ-SO-03 | The system MUST function end to end with only Bucks nodes reachable, except where Apple forces otherwise. | MUST | M6 | Isolation test |

## Quality and operability

| ID | Requirement | Pri | Milestone | Verification |
|---|---|---|---|---|
| REQ-QA-01 | Core logic MUST have automated tests that run in CI on every change. | MUST | M1 | CI |
| REQ-QA-02 | Normative specs MUST have committed test vectors reproducible by the reference implementation. | MUST | M1 | CI |
| REQ-QA-03 | Builds MUST be reproducible enough that a release's published hash matches a rebuild from the tagged source. | SHOULD | M4 | Rebuild check |
| REQ-QA-04 | Documentation MUST NOT claim a capability that has no passing test. | MUST | M0 | Review |
| REQ-QA-05 | Large binaries and models MUST NOT be stored in git. | MUST | M0 | CI size check |
