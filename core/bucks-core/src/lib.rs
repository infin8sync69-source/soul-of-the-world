//! Bucks core: identity log, Bucks ID and signed events.
//!
//! Implements SPEC-001 (identity), SPEC-002 (events) and SPEC-003 (encoding), version 0.2.
//! See `docs/specs/` in the repository root.
//!
//! Provenance: ported from the predecessor crate `bucks-core` (repository
//! `infin8sync69-source/buckscore`, branch `claude/unification-plan-and-core`, commit
//! `fd7b1e5`), then changed: domain-separated signatures and hashes (ADR-0013), a `Signer`
//! trait that may fail and whose output the core normalises to low-S, and a size limit on
//! `home` entries. Re-reviewed for this repository.

pub mod cbor;
mod domain;
mod event;
mod id;
mod log;

pub use domain::{signing_message, Domain};
pub use event::{verify_stream, Event, EventDraft, MAX_KIND, MAX_PAYLOAD};
pub use id::{BucksId, PubKey};
pub use log::{
    replay, verify_log, Device, Op, OpBody, Signer, State, MAX_DEVICES, MAX_DEVICE_NAME, MAX_HOME,
    MAX_HOME_ENTRY, MAX_ROTATION_KEYS,
};

/// Errors are specific so callers can act on them. They never contain key material.
#[derive(Debug, Clone, thiserror::Error, PartialEq, Eq)]
pub enum Error {
    #[error("encoding: {0}")]
    Encoding(&'static str),
    #[error("invalid public key")]
    BadKey,
    #[error("invalid signature")]
    BadSignature,
    #[error("signer failed: {0}")]
    Signer(String),
    #[error("log: {0}")]
    Log(&'static str),
    #[error("event: {0}")]
    Event(&'static str),
    #[error("invalid id")]
    BadId,
}
