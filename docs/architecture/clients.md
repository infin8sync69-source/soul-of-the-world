# Clients

Status: Draft. Related: [ADR-0005](../decisions/0005-rust-core.md), [ADR-0006](../decisions/0006-native-mobile-tauri-desktop.md).

## Shared core

All clients use one Rust core for identity, events, verification, storage, sync and the confirmation policy. Platform code supplies only what the core cannot: secure key operations, UI, push, location and camera.

```
  Android (Kotlin/Compose)   iOS (Swift/SwiftUI)   Desktop (Tauri 2 + Svelte)
          |  generated bindings     |                      |  native calls
          +-----------+-------------+----------------------+
                      |
                 Core (Rust)
          identity - events - store - sync - content - policy
                      |
              Signer interface (platform keystore)
```

The **Signer interface** is how the core asks the platform to sign with a hardware-backed key without ever receiving the private key.

## Responsibilities by platform

| Concern | Android | iOS | Desktop | Web |
|---|---|---|---|---|
| Device key | Android Keystore (StrongBox when present) | Secure Enclave | OS keychain; optional hardware token | None (never holds keys) |
| Push | Open push protocol, with optional platform push flavour | Platform push, wake-only | Not needed | Not needed |
| Location | Platform location service, coarse by default | Core Location, coarse by default | Not used | Not used |
| Local AI | Rules first, small on-device model | Rules first, small on-device model | Local model runtime | None |
| Distribution | Own signed builds, F-Droid, store | App Store | Signed installers for three OSs | Static site, mirrored |

## Why native mobile

Hardware-backed keys, background location, biometrics and push are first-class in native toolkits and uneven in cross-platform ones. The most mature predecessor code is native and has working CI. The decision and alternatives are in [ADR-0006](../decisions/0006-native-mobile-tauri-desktop.md).

## Offline behaviour

Every client reads from the local store and queues writes in an outbox. UI states distinguish "saved locally", "sent to node", and "confirmed". Money and assignment actions require the node's receipt to be shown as complete.

## The confirmation gate

Actions that move money or commit another person (order, ride) open a confirmation sheet and require biometrics above a configurable amount or for a first-time counterparty. Voice or assistant input cannot bypass it ([REQ-CL-03](../requirements.md), [REQ-AI-03](../requirements.md)). The policy lives in the core so every client behaves identically.

## AI assistant

The assistant proposes; deterministic code validates and executes. Order of resolution: rules, then local model, then a node or provider the user chose. No remote provider is used silently ([REQ-AI-02](../requirements.md)).

## Privacy defaults

No telemetry. Location coarsened except during an active ride or delivery. Contacts and media are accessed only on explicit user action.
