# ADR-0006: Native mobile apps; Tauri desktop

- Status: Accepted
- Date: 2026-10-09
- Related: [clients](../architecture/clients.md), REQ-CL-01, REQ-CL-02

## Context
Mobile needs hardware-backed keys, background location, biometrics, store-compliant push. The most mature predecessor code is native (Kotlin and Swift) with working CI. On desktop, the predecessors had an Electron app (feature-rich, macOS arm64 only, source outside version control) and a Tauri app (smaller, cross-platform design, fewer features), plus abandoned Qt and Next.js shells.

## Decision
Android in Kotlin with Jetpack Compose, iOS in Swift with SwiftUI, desktop in Tauri 2 with a Svelte front end and the Rust core in-process. Features of the Electron app are ported in priority order, not shipped as is.

## Alternatives considered
- Tauri for mobile: maturity of mobile support lags desktop, and would discard the best existing code.
- Flutter or React Native: reimplementation cost; keystore and background-service friction.
- Keep Electron: large footprint, a second runtime beside the Rust core, and the existing build is not cross-platform.

## Consequences
Two mobile UIs are maintained by hand with a parity checklist. Desktop features regress temporarily during ports.

## Revisit when
Cross-platform mobile frameworks match native for keystore, background services and push, or the team cannot sustain two mobile UIs.
