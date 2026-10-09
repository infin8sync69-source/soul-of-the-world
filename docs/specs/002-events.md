# SPEC-002: Signed events

Status: Draft. Version 0.1. Depends on [SPEC-001](001-identity.md) and [SPEC-003](003-encoding.md). Vectors: [events-v1.json](../../spec/test-vectors/events-v1.json).

## 1. Overview

An **event** is a signed record authored by a Bucks ID and signed by one of that identity's device keys. Each device keeps a chain of its own events. Events are the unit of data, synchronisation and verification.

## 2. Structure

Unsigned map (the signed message):

| Field | Type | Rule |
|---|---|---|
| `v` | uint | 1 |
| `kind` | text | 1 to 64 bytes. Namespaced, for example `bucks.post.create`. |
| `author` | bytes(16) | The Bucks ID. |
| `device` | bytes(16) | Id of the signing device in the author's identity log. |
| `seq` | uint | Per-device counter starting at 0. |
| `prev` | bytes(32) or null | Hash of this device's previous event. Null if `seq` is 0. |
| `ctx` | uint | Identity-log `seq` the signer had seen. |
| `ts` | uint | Milliseconds since the Unix epoch, as claimed by the signer. |
| `payload` | bytes | At most 65,536 bytes. Interpretation depends on `kind`. |

Wire form: the unsigned fields plus `sig` (bytes(64)). The signature is low-S ECDSA P-256 over the canonical encoding of the unsigned map, by the device key.

`hash(event) = BLAKE3(wire form)`.

## 3. Verification

Given the author's identity **history** (the state after every operation, per SPEC-001 section 5):

1. `author` MUST equal the history's Bucks ID. (E1)
2. `ctx` MUST be a valid index into the history. (E2)
3. The device `device` MUST exist in the state at `ctx` **and** in the latest state, with the same key in both. (E3)
4. `sig` MUST verify under that key and MUST be low-S. (E4)
5. `kind` and `payload` MUST be within the stated limits. (E5)

### 3.1 Device stream

A list of one device's events is valid if: every event verifies, `seq` is contiguous from the expected start, and each `prev` equals the hash of the preceding event. A node that sees two events with the same `(device, seq)` and different hashes has detected a fork; it MUST NOT accept both and SHOULD flag the device.

## 4. Removed devices (deliberately strict)

Rule E3 means that events signed by a device that was **later removed** stop verifying. This stops a stolen-then-removed device from backdating events with an old `ctx`, which cannot be distinguished from legitimate old events without a trusted clock. The cost is that history signed by a retired device must be re-attested or archived under another policy. This is an open design question ([OQ-06](../open-questions.md)).

## 5. Security considerations

- **Timestamps** are signer claims and MUST NOT be used for ordering or security decisions.
- **Replay.** Re-sending a valid event is harmless: nodes identify events by hash and ignore duplicates.
- **Confidentiality.** The payload is not encrypted by this spec. Private event types MUST encrypt end to end inside `payload` (REQ-DA-04).
- **Size.** Limits bound memory and bandwidth. Nodes MUST enforce them before parsing deeply.
- **No domain separation** (see SPEC-003 gaps).
- **Metadata.** `author`, `device`, `kind`, `seq` and `ts` are visible to nodes. Designs that need metadata privacy must go beyond this spec.

## 6. Known gaps (v0.1)

- No event type registry or payload schemas (planned SPEC-004).
- No expiry or retention semantics ([OQ-10](../open-questions.md)).
- Hybrid logical clock encoding for last-writer-wins views is not defined.
- No batch or Merkle structure for efficient range sync.

## 7. Reference implementation

`bucks-core` module `event` on the predecessor branch named in SPEC-001. Six event tests pass: valid and round-trip, tamper and wrong author and unknown device and bad context, removed device cannot backdate, stream gaps and reorder and forks, limits, and vectors. [Certain]
