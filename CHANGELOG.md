# Changelog

All notable changes are recorded here. Format follows [Keep a Changelog](https://keepachangelog.com/). Specifications carry their own version history in their headers.

## [Unreleased]

### Added (M1)
- `core/bucks-core`: identity log, Bucks ID, signed events and deterministic encoding (SPEC-001..003 v0.2), ported from the predecessor crate and re-reviewed. 24 tests: unit, vector and property-based fuzzing.
- `core/bucks-cli` (`bucks`): development CLI for identities, devices, rotation and events, with an end-to-end test.
- `scripts/verify_vectors.py`: independent Python verifier of the vectors.
- `scripts/check_repo.py`, pinned gitleaks scanning, and CI workflows `core` and `hygiene`.
- ADR-0013: domain separation for signatures and hashes.

### Changed
- **Protocol break:** specs 001 to 003 move to 0.2 with domain-separated signatures and hashes; all vectors regenerated. v0.1 data does not verify.
- SPEC-001: keys must be 33-byte compressed on the wire; `home` entries limited to 256 bytes; signers' high-S output is normalised.
- SPEC-002: event authors must carry valid Bucks ID version and variant bits.

### Security
- Bucks ID parsing no longer accepts a leading `+` in hex pairs (found while porting).

### Added
- Foundation documentation set: vision, requirements, architecture, specifications 001 to 003, threat model, decision records, roadmap, engineering handbook, reference-repository analysis, risk register, open questions.
- Identity and event test vectors (v0.1, since replaced by v0.2).
- Documentation checker (`scripts/check_docs.py`) and CI workflow.
