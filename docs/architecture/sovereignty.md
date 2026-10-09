# Sovereignty and dependency policy

Status: Draft. Related: [REQ-SO-01..03](../requirements.md), [lessons learned](../references/lessons-learned.md).

The goal "not dependent on any corporate conglomerate" is made testable by classifying every external dependency and requiring an exit plan.

## Classes

| Class | Meaning | Rule |
|---|---|---|
| **A: Open and replaceable** | Open protocol or data, multiple providers, can be self-hosted | Allowed. Record it. |
| **B: Contained** | Single provider or platform we cannot avoid | Allowed only behind an interface, minimising what the provider sees. Record an exit or mitigation. |
| **C: Forbidden by default** | Closed service that holds identity, keys, data or core logic | Needs an ADR with a time-boxed exit plan. |

## Register (initial)

| Dependency | Use | Class | Containment or exit |
|---|---|---|---|
| Apple push service | Waking iOS devices | B | Wake-only messages; content fetched over the app's own transport. No alternative exists on iOS. |
| Apple App Store | iOS distribution | B | Required for most iOS users. Keep source and spec open. |
| Google Play | Android distribution | B | Also ship signed builds directly and through an open store. |
| Domain registrar and DNS | The project's name | B | Publish the site and bootstrap list under additional names that cannot be seized by one party. |
| Code-signing certificates | Desktop installers | B | Also publish detached signatures verifiable with a pinned key. |
| UPI | Payments in India | B | National rail, not a conglomerate. Behind a payment interface. |
| OpenStreetMap data and tile servers | Maps | A | Self-hosted tiles and routing on nodes when traffic justifies it. |
| Open push protocol (UnifiedPush) | Android push | A | Self-hosted push server on a node. |
| IPFS content addressing | Public content | A | Any gateway, any pinning node. |
| iroh relays | Connectivity | A | Relays run by nodes. |
| GitHub | Source hosting, CI | B | Mirror to a self-hosted forge. Releases are signed and pinned independently. |
| Language model weights (Hugging Face) | Local assistant | B | Mirror blessed models to content-addressed storage on nodes. |
| crates.io | Rust package source for the core | B | Lockfile committed; vendor dependencies into release builds; a registry mirror is possible. |
| Rust crates in use: `p256` (RustCrypto), `blake3`, `bs58`, `thiserror`, `clap`, `serde_json`, `rand_core`, `proptest` (tests) | Core and CLI | A | Open-source, pinned by `core/Cargo.lock`. Additions need review (T-17). |
| PyPI: `blake3`, `cryptography` | Independent vector verifier only (CI) | A | Pinned in `scripts/requirements-vectors.txt`. Not used by any product. |
| GitHub Actions | CI runners | B | Every CI step is a plain script runnable locally or on a self-hosted runner. |
| gitleaks (GitHub release binary) | Secret scanning in CI | A | Version and SHA-256 pinned in the workflow. |

Dependencies from the predecessor projects that this project does **not** adopt: hosted backend-as-a-service for data and authentication, third-party phone-OTP identity providers as the identity root, cloud push as the only push path, cloud language-model APIs as defaults, hosted serverless key generation, public IPFS gateways as the only distribution path.

## Process for a new dependency

1. Add a row to the register in the same pull request that introduces it.
2. State the class and, for B and C, the containment and exit plan.
3. For C, add an ADR with a deadline.
4. CI checks that every domain and package source named in the repository's configuration appears in the register. [Guessing: the checker is a milestone M0 task; see [roadmap](../roadmap.md).]

## Testing sovereignty

[REQ-SO-03](../requirements.md) calls for an isolation test: run the whole system with network access limited to Bucks nodes and confirm every feature works (iOS push excepted). Until that test exists, the project may not claim independence.
