# Architecture overview

Status: Draft. Related: [requirements](../requirements.md), [ADR index](../decisions/README.md).

## Context

```
                 +---------------------------+
                 |        A person           |
                 |  (one Bucks ID, N devices)|
                 +-------------+-------------+
                               |
        +----------------------+----------------------+
        |                      |                      |
  +-----+------+        +------+-----+         +------+-----+
  |  Phone     |        |  Laptop    |         |  Web page  |
  | Android/iOS|        |  Desktop   |         | read-only  |
  +-----+------+        +------+-----+         +------+-----+
        |   shared core: identity, events, store, sync  |
        +----------------------+----------------------+
                               |
                 signed events over authenticated connections
                               |
        +----------------------+-----------------------+
        |                      |                       |
  +-----+------+        +------+------+        +-------+-----+
  | Home node  |<------>| Neighbour   |<------>| Directory   |
  | (yours)    | federate| node        | gossip | mirrors     |
  +------------+        +-------------+        +-------------+
```

A person's devices talk to nodes, and nodes talk to each other. A node never needs to be trusted with keys: it verifies signatures and stores or relays signed data.

## Layers

| Layer | Responsibility | Primary decision |
|---|---|---|
| Apps | UI, platform integration (keystore, push, location, camera) | [ADR-0006](../decisions/0006-native-mobile-tauri-desktop.md) |
| Core (Rust) | Identity log, events, verification, local store, sync, CIDs, confirmation policy | [ADR-0005](../decisions/0005-rust-core.md) |
| Transport | Authenticated connections between devices and nodes; content retrieval | [ADR-0007](../decisions/0007-transport.md) |
| Nodes | Relay, identity directory, content pinning, gateway, service modules, push gateway, inference | [ADR-0008](../decisions/0008-federated-nodes.md) |
| Anchoring | Optional external timestamps of directory state | [ADR-0009](../decisions/0009-no-custom-chain.md) |

## Trust boundaries

1. **Device vs everything else.** The device is the only place private keys exist. Anything leaving the device is signed or encrypted.
2. **Client vs node.** A node is untrusted for confidentiality of end-to-end content and for authenticity of signed data. It is trusted only for availability and for domain decisions inside service modules (for example, ride assignment), and a user can choose a different node.
3. **Node vs node.** Federation exchanges verified signed data. No node trusts another's claims without verification.
4. **Local services vs the web.** Local ports (assistant, tool server) accept only authenticated local clients. See [REQ-SE-02](../requirements.md).
5. **Build and release vs users.** Installers verify signatures against a pinned key. See [REQ-SE-03](../requirements.md).

## Key flows

### Create an identity
1. App generates a rotation key and a device key on the device (hardware-backed where possible).
2. Core builds and signs the inception operation. The Bucks ID is computed from its unsigned bytes.
3. App shows the ID, short code and QR. It prompts to create a backup rotation key (paper or second device).
4. App publishes the log to the user's chosen home node and any directory mirror. Publication is optional for local use.

### Add a second device
1. New device generates its own device key and shows a request code.
2. Existing device scans it, shows the device name for approval, and requires biometrics.
3. Existing device signs an add-device operation with a rotation key and publishes it.
4. The new device fetches the log, verifies it, and starts syncing.

### Post or order
1. App creates an event with the current identity-log position as context and signs it with the device key.
2. Event is stored locally first, then sent to the home node.
3. Node verifies the signature against the author's identity log and applies it to its views or service module.
4. Other devices and nodes receive and verify the same event.

### Request a ride
1. Rider's app signs a ride-request event.
2. A dispatch service module on a node near the rider decides assignment (single authority), emits a signed receipt event, and notifies the driver via push.
3. Both parties' apps verify the receipt and the signatures of each state change.

## Failure modes

| Failure | Behaviour |
|---|---|
| Home node down | Apps keep working offline, queue events, and can fail over to another node listed in the identity log. |
| Directory mirror serves a stale log | Client verifies the chain and compares against other mirrors; a shorter valid chain is accepted only if no longer valid chain is found. |
| Lost phone | Remove it from another device or recover with the backup rotation key. Events from a removed device stop verifying (see [SPEC-002](../specs/002-events.md)). |
| Malicious node | Cannot forge events or read end-to-end content. Can withhold or censor, so users are able to change nodes. |
| Release signing key lost or leaked | Pinned-key rotation procedure in [secrets policy](../security/secrets-policy.md). |

## What is not decided yet

Domain schemas for rides, orders and listings; the node API; the federation protocol; the push design. Each will get a spec before implementation. See the [roadmap](../roadmap.md).
