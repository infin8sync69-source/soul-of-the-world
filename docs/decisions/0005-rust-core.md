# ADR-0005: One Rust core shared by every client

- Status: Accepted
- Date: 2026-10-09
- Related: [clients](../architecture/clients.md), REQ-CL-01

## Context
The predecessors implemented identity and crypto separately in Kotlin, Swift, TypeScript, Python and C++, and the implementations disagreed (for example, three byte-identical copies in one language and four incompatible identity schemes overall).

## Decision
Identity, events, verification, local storage, sync, content addressing and the confirmation policy live in one Rust library. Android and iOS call it through generated bindings; desktop calls it directly. Platform code provides a Signer interface for hardware-backed keys.

## Alternatives considered
- Reimplement per platform with shared test vectors: cheaper to start, but drift is likely and every fix is done thrice.
- TypeScript core: weaker bindings for native mobile.
- Kotlin Multiplatform: strong on Android, weaker on desktop and server targets.

## Consequences
A Rust toolchain in every build. Build complexity for mobile (cross-compilation, binary size). One place to fix and audit.

## Revisit when
Binding maintenance cost exceeds the duplication cost, or the core's size becomes a problem on low-end devices.
