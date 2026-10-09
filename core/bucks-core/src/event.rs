//! Signed events (SPEC-002 v0.2). Rule numbers (E1..E5) refer to the spec.

use crate::cbor::Value;
use crate::domain::{hash, Domain, CTX_EVENT_HASH};
use crate::log::{sign_in, verify_in, Signer, State};
use crate::{BucksId, Error};

pub const MAX_PAYLOAD: usize = 64 * 1024;
pub const MAX_KIND: usize = 64;

/// Everything an event contains except its signature.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EventDraft {
    pub kind: String,
    pub author: BucksId,
    pub device: [u8; 16],
    pub seq: u64,
    pub prev: Option<[u8; 32]>,
    pub ctx: u64,
    pub ts: u64,
    pub payload: Vec<u8>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Event {
    pub draft: EventDraft,
    pub sig: [u8; 64],
}

fn b(x: &[u8]) -> Value {
    Value::Bytes(x.to_vec())
}

fn check_limits(d: &EventDraft) -> Result<(), Error> {
    if d.kind.is_empty() || d.kind.len() > MAX_KIND || d.payload.len() > MAX_PAYLOAD {
        return Err(Error::Event("size limits")); // E5
    }
    Ok(())
}

impl EventDraft {
    fn fields(&self) -> Vec<(String, Value)> {
        vec![
            ("v".into(), Value::Uint(1)),
            ("kind".into(), Value::Text(self.kind.clone())),
            ("author".into(), b(self.author.as_bytes())),
            ("device".into(), b(&self.device)),
            ("seq".into(), Value::Uint(self.seq)),
            ("prev".into(), self.prev.map_or(Value::Null, |p| b(&p))),
            ("ctx".into(), Value::Uint(self.ctx)),
            ("ts".into(), Value::Uint(self.ts)),
            ("payload".into(), b(&self.payload)),
        ]
    }

    pub fn unsigned_bytes(&self) -> Vec<u8> {
        Value::Map(self.fields()).encode()
    }

    pub fn sign(self, signer: &dyn Signer) -> Result<Event, Error> {
        check_limits(&self)?;
        let sig = sign_in(Domain::Event, &self.unsigned_bytes(), signer)?;
        Ok(Event { draft: self, sig })
    }
}

impl Event {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut f = self.draft.fields();
        f.push(("sig".into(), b(&self.sig)));
        Value::Map(f).encode()
    }

    /// Domain-separated hash of the wire form: the event's id and the next event's `prev`.
    pub fn hash(&self) -> [u8; 32] {
        hash(CTX_EVENT_HASH, &self.to_bytes())
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Event, Error> {
        let v = Value::decode(bytes)?;
        let Value::Map(f) = &v else {
            return Err(Error::Encoding("event must be a map"));
        };
        if f.len() != 10 || !matches!(v.get("v"), Some(Value::Uint(1))) {
            return Err(Error::Encoding("event fields"));
        }
        let bad = Error::Encoding("event field");
        let (Some(Value::Text(kind)), Some(Value::Bytes(author)), Some(Value::Bytes(device))) =
            (v.get("kind"), v.get("author"), v.get("device"))
        else {
            return Err(bad);
        };
        let (Some(Value::Uint(seq)), Some(Value::Uint(ctx)), Some(Value::Uint(ts))) =
            (v.get("seq"), v.get("ctx"), v.get("ts"))
        else {
            return Err(bad);
        };
        let (Some(Value::Bytes(payload)), Some(Value::Bytes(sig))) =
            (v.get("payload"), v.get("sig"))
        else {
            return Err(bad);
        };
        let prev = match v.get("prev") {
            Some(Value::Null) => None,
            Some(Value::Bytes(p)) => {
                Some(<[u8; 32]>::try_from(p.as_slice()).map_err(|_| bad.clone())?)
            }
            _ => return Err(bad),
        };
        let author = <[u8; 16]>::try_from(author.as_slice()).map_err(|_| bad.clone())?;
        let draft = EventDraft {
            kind: kind.clone(),
            author: BucksId::from_bytes(author)?,
            device: <[u8; 16]>::try_from(device.as_slice()).map_err(|_| bad.clone())?,
            seq: *seq,
            prev,
            ctx: *ctx,
            ts: *ts,
            payload: payload.clone(),
        };
        check_limits(&draft)?;
        let sig = <[u8; 64]>::try_from(sig.as_slice()).map_err(|_| bad)?;
        Ok(Event { draft, sig })
    }

    /// Verify against the author's identity history (`replay` output).
    ///
    /// Deliberately strict (SPEC-002 section 4): the signing device must be authorised both at
    /// `ctx` and in the latest state, with the same key. Events from a device removed later stop
    /// verifying, so a stolen-then-removed key cannot backdate events.
    pub fn verify(&self, history: &[State]) -> Result<(), Error> {
        let d = &self.draft;
        let latest = history.last().ok_or(Error::Event("empty history"))?;
        if d.author != latest.id {
            return Err(Error::Event("author mismatch")); // E1
        }
        let at_ctx = usize::try_from(d.ctx)
            .ok()
            .and_then(|i| history.get(i))
            .ok_or(Error::Event("ctx beyond history"))?; // E2
        let key = match (at_ctx.device(&d.device), latest.device(&d.device)) {
            (Some(a), Some(c)) if a.key == c.key => a.key,
            _ => return Err(Error::Event("device not authorised")), // E3
        };
        check_limits(d)?; // E5
        verify_in(Domain::Event, &d.unsigned_bytes(), &key, &self.sig) // E4
    }
}

/// Verify one device's events in order: all verify, `seq` is contiguous from `start_seq`,
/// each `prev` is the hash of the event before, and all come from the same device.
pub fn verify_stream(
    events: &[Event],
    history: &[State],
    start_seq: u64,
    start_prev: Option<[u8; 32]>,
) -> Result<(), Error> {
    let (mut seq, mut prev) = (start_seq, start_prev);
    let device = events.first().map(|e| e.draft.device);
    for e in events {
        if Some(e.draft.device) != device || e.draft.seq != seq || e.draft.prev != prev {
            return Err(Error::Event("broken device stream"));
        }
        e.verify(history)?;
        seq = seq
            .checked_add(1)
            .ok_or(Error::Event("sequence overflow"))?;
        prev = Some(e.hash());
    }
    Ok(())
}
