# Architecture decision records

Status: Accepted (process). Each significant decision is a numbered, immutable record. To change a decision, write a new ADR that supersedes it; do not edit history except to fix typos or set the status.

Use [template.md](template.md). Statuses: **Proposed**, **Accepted**, **Superseded by ADR-NNNN**, **Rejected**.

| ADR | Decision | Status |
|---|---|---|
| [0001](0001-record-decisions.md) | Record decisions as ADRs | Accepted |
| [0002](0002-fresh-start-references-read-only.md) | Start fresh; predecessor repositories are read-only references | Accepted |
| [0003](0003-self-certifying-identity.md) | Self-certifying identity: Bucks ID derived from an identity log | Accepted |
| [0004](0004-p256-keys.md) | P-256 as the mandatory signature curve | Accepted |
| [0005](0005-rust-core.md) | One Rust core shared by every client | Accepted |
| [0006](0006-native-mobile-tauri-desktop.md) | Native mobile apps; Tauri desktop | Accepted |
| [0007](0007-transport.md) | iroh for connections, IPFS-style CIDs for content | Accepted |
| [0008](0008-federated-nodes.md) | Federated, replaceable nodes; service logic in modules | Accepted |
| [0009](0009-no-custom-chain.md) | No custom blockchain or token for now | Accepted |
| [0010](0010-local-first-ai.md) | Local-first AI; remote providers opt-in | Accepted |
| [0011](0011-spec-first-with-vectors.md) | Specification first, with test vectors | Accepted |
| [0012](0012-monorepo-and-trunk-based.md) | One repository, trunk-based development | Accepted |
| [0013](0013-domain-separation.md) | Domain separation for every signature and hash | Accepted |

"Accepted" here means accepted by the project owner's direction on 2026-10-09 as the working plan. Each ADR lists what would make us revisit it.
