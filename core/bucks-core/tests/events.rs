mod common;
use bucks_core::*;
use common::*;
use p256::ecdsa::SigningKey;

struct World {
    ops: Vec<Op>,
    history: Vec<State>,
    id: BucksId,
    root: SigningKey,
    phone: SigningKey,
}

fn world() -> World {
    let (root, phone) = (key("root-primary"), key("phone"));
    let inc = Op::sign(
        0,
        None,
        OpBody::Inception {
            rotation_keys: vec![root.public_key()],
            devices: vec![dev("phone", 1)],
            home: vec![],
        },
        &root,
    )
    .unwrap();
    let id = inc.bucks_id().unwrap();
    let history = replay(std::slice::from_ref(&inc)).unwrap();
    World {
        ops: vec![inc],
        history,
        id,
        root,
        phone,
    }
}

#[allow(clippy::too_many_arguments)]
fn draft(
    kind: &str,
    author: BucksId,
    device: u8,
    seq: u64,
    prev: Option<[u8; 32]>,
    ctx: u64,
    ts: u64,
    payload: &[u8],
) -> EventDraft {
    EventDraft {
        kind: kind.into(),
        author,
        device: [device; 16],
        seq,
        prev,
        ctx,
        ts,
        payload: payload.to_vec(),
    }
}

fn ev(w: &World, seq: u64, prev: Option<[u8; 32]>, payload: &[u8]) -> Event {
    draft(
        "bucks.post.create",
        w.id,
        1,
        seq,
        prev,
        0,
        1_760_000_000_000 + seq,
        payload,
    )
    .sign(&w.phone)
    .unwrap()
}

#[test]
fn valid_event_and_wire_roundtrip() {
    let w = world();
    let e = ev(&w, 0, None, b"hello");
    e.verify(&w.history).unwrap();
    assert_eq!(Event::from_bytes(&e.to_bytes()).unwrap(), e);
}

#[test]
fn rejects_tamper_author_device_ctx_and_key() {
    let w = world();
    let mut t = ev(&w, 0, None, b"hello");
    t.draft.payload = b"hellp".to_vec();
    assert_eq!(t.verify(&w.history).unwrap_err(), Error::BadSignature);

    let other = BucksId::parse("3f9a6c2e-1b47-8d3c-9e21-7a4b0c5d8e1f").unwrap();
    assert!(draft("bucks.x", other, 1, 0, None, 0, 1, b"")
        .sign(&w.phone)
        .unwrap()
        .verify(&w.history)
        .is_err());
    assert!(
        draft("bucks.x", w.id, 9, 0, None, 0, 1, b"")
            .sign(&w.phone)
            .unwrap()
            .verify(&w.history)
            .is_err(),
        "unknown device"
    );
    assert!(
        draft("bucks.x", w.id, 1, 0, None, 5, 1, b"")
            .sign(&w.phone)
            .unwrap()
            .verify(&w.history)
            .is_err(),
        "ctx beyond history"
    );
    assert!(
        draft("bucks.x", w.id, 1, 0, None, u64::MAX, 1, b"")
            .sign(&w.phone)
            .unwrap()
            .verify(&w.history)
            .is_err(),
        "huge ctx"
    );
    let stolen = draft("bucks.x", w.id, 1, 0, None, 0, 1, b"")
        .sign(&key("stolen"))
        .unwrap();
    assert_eq!(stolen.verify(&w.history).unwrap_err(), Error::BadSignature);
}

#[test]
fn removed_device_stops_verifying_and_cannot_backdate() {
    let mut w = world();
    let add = Op::sign(
        1,
        Some(w.ops[0].hash()),
        OpBody::AddDevice {
            device: dev("tablet", 2),
        },
        &w.root,
    )
    .unwrap();
    let rm = Op::sign(
        2,
        Some(add.hash()),
        OpBody::RemoveDevice { device_id: [1; 16] },
        &w.root,
    )
    .unwrap();
    w.ops.extend([add, rm]);
    let h = replay(&w.ops).unwrap();
    assert_eq!(
        ev(&w, 0, None, b"before removal").verify(&h).unwrap_err(),
        Error::Event("device not authorised")
    );
    assert!(draft("bucks.x", w.id, 1, 7, None, 0, 1, b"")
        .sign(&w.phone)
        .unwrap()
        .verify(&h)
        .is_err());
}

#[test]
fn stream_detects_gaps_reorder_forks_and_mixed_devices() {
    let w = world();
    let e0 = ev(&w, 0, None, b"a");
    let e1 = ev(&w, 1, Some(e0.hash()), b"b");
    let e2 = ev(&w, 2, Some(e1.hash()), b"c");
    verify_stream(&[e0.clone(), e1.clone(), e2.clone()], &w.history, 0, None).unwrap();
    assert!(
        verify_stream(&[e0.clone(), e2.clone()], &w.history, 0, None).is_err(),
        "gap"
    );
    assert!(
        verify_stream(&[e1.clone(), e0.clone()], &w.history, 0, None).is_err(),
        "reorder"
    );
    let fork = ev(&w, 1, Some(e0.hash()), b"B");
    assert!(
        verify_stream(&[e0.clone(), e1.clone(), fork], &w.history, 0, None).is_err(),
        "fork"
    );
    verify_stream(&[e2], &w.history, 2, Some(e1.hash())).unwrap();
}

#[test]
fn limits_enforced() {
    let w = world();
    assert!(draft("", w.id, 1, 0, None, 0, 0, b"")
        .sign(&w.phone)
        .is_err());
    assert!(
        draft(&"k".repeat(MAX_KIND + 1), w.id, 1, 0, None, 0, 0, b"")
            .sign(&w.phone)
            .is_err()
    );
    assert!(
        draft("bucks.x", w.id, 1, 0, None, 0, 0, &vec![0; MAX_PAYLOAD + 1])
            .sign(&w.phone)
            .is_err()
    );
    assert!(
        draft("bucks.x", w.id, 1, 0, None, 0, 0, &vec![0; MAX_PAYLOAD])
            .sign(&w.phone)
            .is_ok()
    );
}

#[test]
fn event_signature_cannot_be_reused_as_op_signature() {
    let w = world();
    let e = ev(&w, 0, None, b"x");
    let vk = w.phone.verifying_key().to_owned();
    let sig = p256::ecdsa::Signature::from_slice(&e.sig).unwrap();
    use p256::ecdsa::signature::Verifier;
    let unsigned = e.draft.unsigned_bytes();
    assert!(vk
        .verify(&signing_message(Domain::Event, &unsigned), &sig)
        .is_ok());
    assert!(vk
        .verify(&signing_message(Domain::IdentityOp, &unsigned), &sig)
        .is_err());
}

#[test]
fn vectors() {
    let w = world();
    let e0 = ev(&w, 0, None, b"hello bucks");
    let e1 = ev(&w, 1, Some(e0.hash()), b"second");
    let doc = serde_json::json!({
        "spec": "SPEC-002 v0.2 (docs/specs/002-events.md), SPEC-003 v0.2",
        "warning": "TEST KEYS ONLY. Each secret is BLAKE3(label). Never use for anything real.",
        "signature": "ECDSA P-256 SHA-256, low-S, over 'bucks/event/v1' || 0x00 || unsigned_bytes; RFC 6979 deterministic nonces",
        "hash_context": "bucks 2026-10-09 event hash v1",
        "identity": "inception signed by root-primary with rotation_keys [root-primary], devices [phone id 0x01*16], home []",
        "inception_hex": hex(&w.ops[0].to_bytes()),
        "author": w.id.to_string(),
        "events_hex": [hex(&e0.to_bytes()), hex(&e1.to_bytes())],
        "event_hashes": [hex(&e0.hash()), hex(&e1.hash())],
    });
    let path = format!(
        "{}/../../spec/test-vectors/events-v1.json",
        env!("CARGO_MANIFEST_DIR")
    );
    let text = serde_json::to_string_pretty(&doc).unwrap() + "\n";
    if std::env::var("UPDATE_VECTORS").is_ok() {
        std::fs::write(&path, &text).unwrap();
    }
    assert_eq!(
        std::fs::read_to_string(&path).unwrap(),
        text,
        "vector mismatch"
    );
}
