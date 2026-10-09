//! Domain separation (ADR-0013, SPEC-003 section 5).
//!
//! Every signature and every hash names what it is for, so a value produced for one purpose
//! can never be accepted for another.

/// Signature domains. The tag is prefixed to the canonical bytes, followed by one zero byte.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Domain {
    IdentityOp,
    Event,
}

impl Domain {
    pub const fn tag(self) -> &'static str {
        match self {
            Domain::IdentityOp => "bucks/identity-op/v1",
            Domain::Event => "bucks/event/v1",
        }
    }
}

/// The exact message given to ECDSA: `tag || 0x00 || canonical unsigned bytes`.
pub fn signing_message(domain: Domain, unsigned: &[u8]) -> Vec<u8> {
    let tag = domain.tag().as_bytes();
    let mut m = Vec::with_capacity(tag.len() + 1 + unsigned.len());
    m.extend_from_slice(tag);
    m.push(0);
    m.extend_from_slice(unsigned);
    m
}

/// BLAKE3 key-derivation contexts. Never reuse a context for a different purpose.
pub(crate) const CTX_ID: &str = "bucks 2026-10-09 identity id v1";
pub(crate) const CTX_OP_HASH: &str = "bucks 2026-10-09 identity op hash v1";
pub(crate) const CTX_EVENT_HASH: &str = "bucks 2026-10-09 event hash v1";
pub(crate) const CTX_SHORT_CODE: &str = "bucks 2026-10-09 short code v1";

pub(crate) fn hash(ctx: &str, data: &[u8]) -> [u8; 32] {
    blake3::derive_key(ctx, data)
}
