# Glossary

Status: Draft.

| Term | Definition |
|---|---|
| **Bucks ID** | A 128-bit UUID (version nibble 8) derived from the BLAKE3 hash of a person's unsigned inception operation. The global identifier. See [SPEC-001](specs/001-identity.md). |
| **Identity log** | The ordered, signed list of operations (inception, add device, remove device, rotate keys) that defines a person's current keys and devices. |
| **Inception** | The first operation of an identity log. Its unsigned bytes define the Bucks ID. |
| **Rotation key** | A key that signs identity-log operations. Kept offline or hardware-protected. 1 to 3 per identity. |
| **Device key** | A key unique to one installation. Signs events and authenticates to nodes. Hardware-backed where the platform allows. |
| **Event** | A signed record authored by a Bucks ID and signed by a device key. The unit of data. See [SPEC-002](specs/002-events.md). |
| **Device stream** | The chain of events from one device, linked by sequence number and previous-event hash. |
| **Context (`ctx`)** | The identity-log sequence number a signer had seen when signing an event. |
| **Node** | A server program run by anyone that provides one or more roles to clients. Replaceable. |
| **Home node** | A node a person has listed in their identity log as their preferred host. |
| **Role** | A capability a node offers: relay, identity directory, content pin, gateway, service module, push gateway, inference. |
| **Service module** | Domain logic hosted by a node, such as dispatch or commerce. |
| **Directory** | A node role that stores and serves identity logs, verifying every operation before accepting it. |
| **Core** | The shared library (Rust) that implements identity, events, storage and sync on every platform. |
| **CID** | Content identifier: a hash-based address for immutable data (IPFS format). |
| **Attestation** | A signed statement by someone other than the user about the user (for example, that a device is genuine). |
| **Lens** | A user-chosen set of voters over which trust counts are computed. Inherited from the predecessor mobile app. |
| **Soul** | The user-facing AI assistant and its persona layer. Not an identity primitive. |
| **Predecessor / reference repositories** | The earlier repositories and desktop app this project learns from. Read-only. See [reference-repos](references/reference-repos.md). |
| **Home node vs directory** | A home node serves a person's data. A directory serves identity logs. One node may do both. |
| **Deterministic CBOR** | The canonical binary encoding used for signed data. See [SPEC-003](specs/003-encoding.md). |
| **Protocol break** | Any change that makes previously valid signed data invalid or changes the Bucks ID of existing identities. Requires a new protocol version. |
