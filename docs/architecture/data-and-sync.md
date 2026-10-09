# Data model and synchronisation

Status: Draft. Related: [SPEC-002](../specs/002-events.md), [REQ-SY-01..04](../requirements.md).

## Principle

The source of truth for a person's data is their set of signed events. Everything else (lists, feeds, balances, conversation views) is a **materialised view** computed from events and can be rebuilt.

## Event streams

Each device keeps one append-only stream. An event carries a sequence number and the hash of the previous event on that device, so gaps, reordering and forks are detectable. A person's data is the union of their devices' streams.

## Synchronisation

Sync is the exchange of events a peer has not seen, identified by hash and by `(device, seq)`. A receiver:

1. verifies the author's identity log (fetching it if unknown),
2. verifies the event signature and stream position,
3. stores it, and
4. updates affected views.

Sync happens device to device (direct connection when possible), and device to node. Nodes replicate with each other only the data their policy and the user's settings allow.

## Where conflicts can and cannot occur

| Kind of data | Conflict handling |
|---|---|
| A person's own profile, settings, follows | Per-field last-writer-wins by a hybrid logical clock carried in the event payload. Deterministic reducers; documented per type. |
| Conversations | Messages are immutable events. Ordering is by per-sender stream plus a claimed timestamp. Edits and deletions are new events. |
| Shared mutable state needing one authority (ride assignment, order status, inventory) | **Not solved by the event log alone.** A service module on a node is the authority; events are inputs and signed receipts are outputs. See [REQ-SY-04](../requirements.md). |
| Votes and trust | Votes are events bound to a completed transaction receipt. Counts are computed per lens over verified votes. |

This honesty matters: a fully serverless design cannot give a rider and a driver a single agreed assignment without a coordinator. The coordinator is replaceable (any node can host the module) but it exists.

## Time

Event timestamps are claims by the signer and are not trusted for ordering or security. Trusted time comes only from directory-attested logs or external timestamping. This is why some policies are conservative (see removed devices in SPEC-002 and [OQ-06](../open-questions.md)).

## Storage

| Place | Store | Notes |
|---|---|---|
| Device | SQLite via the core | Encrypted at rest by platform facilities; events, views, outbox |
| Node | SQLite or Postgres per module | Service modules may use Postgres; the identity directory and pins are append-only stores |
| Content | IPFS-style CIDs | Public, immutable content. Never private plaintext |

## Export and portability

A person can export their identity log and all their events in a documented, open format ([REQ-DA-06](../requirements.md)). Moving to another node is publishing the log there and replicating events to it.

## Open design work

- Reducer definitions and event type registry (a future spec).
- Hybrid logical clock encoding.
- Retention and deletion semantics for a log that is append-only ([OQ-10](../open-questions.md)).
