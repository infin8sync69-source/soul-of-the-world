# Soul of the World

**One person, one identity, on devices they control, served by infrastructure nobody can switch off.**

Soul of the World (working name; product name "Bucks", see [open question OQ-02](docs/open-questions.md)) is a local-first, decentralised software platform for identity, messaging, local commerce and mobility. It runs on phones, desktops and community-run nodes. Every person has a single global identifier that no company issues.

> **Status: early development (milestone M1).** The identity core and a development CLI exist with tests and independently verified test vectors. Nothing here is ready for real identities or real users. Documents carry a status (Draft, Accepted, Superseded) and claims are tagged by confidence. Read [docs/README.md](docs/README.md) for the map.

## Why this exists

The ecosystem this project grows from (six repositories and a desktop app) proved many ideas but never became one product. An audit on 2026-10-09 found four incompatible identity schemes, user private keys held by servers, live credentials in public repositories, and documentation that claimed more than the code did. This project restarts from the audit's lessons, with the old repositories as read-only references. See [docs/references/lessons-learned.md](docs/references/lessons-learned.md).

## The core ideas

1. **Self-certifying identity.** A Bucks ID is a UUID derived from the hash of a person's first signed identity operation. Nobody issues it, it survives key rotation and device loss, and anyone can verify it offline. Spec: [SPEC-001](docs/specs/001-identity.md).
2. **Everything is a signed event.** Posts, orders, rides and messages are events signed by a device key and chained per device. Spec: [SPEC-002](docs/specs/002-events.md).
3. **Replaceable infrastructure.** Nodes are community-run and federated by locality. A person can move to another node without changing identity. See [architecture overview](docs/architecture/overview.md).
4. **No dependency without a record.** Every external service is classified and has an exit plan. See [sovereignty policy](docs/architecture/sovereignty.md).
5. **Claims are backed by tests.** Specifications ship with test vectors that independent implementations must reproduce. See [spec/test-vectors](spec/test-vectors).

## Where to start

| If you want to... | Read |
|---|---|
| Understand the goal and principles | [docs/vision.md](docs/vision.md) |
| See what the system must do | [docs/requirements.md](docs/requirements.md) |
| Understand the design | [docs/architecture/overview.md](docs/architecture/overview.md) |
| Implement the identity protocol | [docs/specs/001-identity.md](docs/specs/001-identity.md) and [spec/test-vectors](spec/test-vectors) |
| See why decisions were made | [docs/decisions](docs/decisions/README.md) |
| See the plan | [docs/roadmap.md](docs/roadmap.md) |
| Contribute | [CONTRIBUTING.md](CONTRIBUTING.md) and [docs/engineering/handbook.md](docs/engineering/handbook.md) |
| Report a vulnerability | [SECURITY.md](SECURITY.md) |
| Know what is undecided | [docs/open-questions.md](docs/open-questions.md) |

## Repository layout (planned)

```
docs/            all documentation (this phase)
spec/            normative test vectors and, later, schemas
core/            Rust core and development CLI                   (identity and events done; store and sync not started)
apps/            android, ios, desktop, web                       (not started)
node/            the node binary and service modules              (not started)
infra/           deployment, CI, release tooling                  (not started)
scripts/         docs, hygiene and independent vector checks
```

## Try it

```
cd core
cargo test                       # unit, vector, property-fuzz and CLI end-to-end tests
cargo run -q --bin bucks -- --home /tmp/demo id new --device-name phone
python3 ../scripts/verify_vectors.py   # independent check (pip install -r ../scripts/requirements-vectors.txt)
```

The CLI keeps keys in plain files. It is a development tool, not a wallet. See [core/README.md](core/README.md).

## Licence

Not yet chosen. See [OQ-01](docs/open-questions.md). Until a licence file is added, all rights are reserved by the authors.
