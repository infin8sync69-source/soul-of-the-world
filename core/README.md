# core

Rust workspace for the shared core. Status: M1, development only.

| Crate | What | Spec |
|---|---|---|
| `bucks-core` | Deterministic encoding, identity log and Bucks ID, signed events, verification | [SPEC-001](../docs/specs/001-identity.md), [SPEC-002](../docs/specs/002-events.md), [SPEC-003](../docs/specs/003-encoding.md) v0.2 |
| `bucks-cli` | `bucks` development CLI | none |

## Build and test

```
cargo test --locked                 # 25 tests: unit, vectors, property fuzzing, CLI end to end
cargo clippy --all-targets -- -D warnings
UPDATE_VECTORS=1 cargo test         # only for an intended protocol change (it is a protocol break)
```

The toolchain is pinned in `../rust-toolchain.toml`.

## Signing interface

Apps never hand private keys to the core. They implement `bucks_core::Signer` on top of the platform keystore. The core builds the domain-separated message, asks the signer to sign it, normalises the result to low-S and checks it against the signer's public key before using it.

## The `bucks` CLI

```
bucks --home DIR id new --device-name phone [--home-node URL]
bucks --home DIR id show
bucks --home DIR device add --name laptop
bucks --home DIR device remove --id <hex>
bucks --home DIR rotate
bucks --home DIR event sign --device <hex> --kind bucks.post.create --payload "hello"
bucks --home DIR event verify --event <hex> [--log FILE]
bucks verify-log FILE
```

**Warning:** the CLI stores private keys as plain hex in `DIR/keys.json` (mode 0600). It exists to exercise the protocol. Never use it for a real identity.

## Provenance

Ported from the predecessor crate on `infin8sync69-source/buckscore`, branch `claude/unification-plan-and-core`, then changed as listed in `bucks-core/src/lib.rs`.
