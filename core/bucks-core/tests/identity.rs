mod common;
use bucks_core::cbor::Value;
use bucks_core::*;
use common::*;
use p256::ecdsa::{signature::Verifier, Signature, SigningKey};

#[test]
fn id_shape_parse_and_rejections() {
    let (op, _, _) = incept();
    let id = op.bucks_id().unwrap();
    let s = id.to_string();
    assert_eq!(s.len(), 36);
    assert_eq!(&s[14..15], "8", "version nibble");
    assert!(matches!(&s[19..20], "8" | "9" | "a" | "b"), "variant bits");
    assert_eq!(BucksId::parse(&s).unwrap(), id);
    assert_eq!(BucksId::parse(&s.to_uppercase()).unwrap(), id);
    assert_eq!(BucksId::parse(&id.did()).unwrap(), id);
    assert_eq!(id.short_code().len(), 8);
    assert!(
        BucksId::parse("3f9a6c2e-1b47-4d3c-9e21-7a4b0c5d8e1f").is_err(),
        "v4 uuid"
    );
    assert!(
        BucksId::parse("3f9a6c2e1b47-8d3c-9e21-7a4b0c5d8e1f-").is_err(),
        "hyphen positions"
    );
    let plus = format!("+{}", &s[1..]);
    assert!(BucksId::parse(&plus).is_err(), "'+' must not parse as hex");
}

#[test]
fn did_key_form_for_p256() {
    assert!(key("phone")
        .public_key()
        .did_key()
        .starts_with("did:key:zDn"));
}

#[test]
fn wire_roundtrip_and_canonical() {
    let (op, _, _) = incept();
    let b = op.to_bytes();
    assert_eq!(Op::from_bytes(&b).unwrap(), op);
    let mut trailing = b.clone();
    trailing.push(0);
    assert!(Op::from_bytes(&trailing).is_err());
    let mut noncanon = b.clone();
    let pos = noncanon.windows(2).position(|w| w == [0x61, b'v']).unwrap();
    noncanon.splice(pos + 2..pos + 3, [0x18, 0x01]); // uint 1 in non-shortest form
    assert!(Op::from_bytes(&noncanon).is_err());
}

#[test]
fn uncompressed_key_is_not_canonical() {
    let (op, _, _) = incept();
    let Value::Map(mut f) = Value::decode(&op.to_bytes()).unwrap() else {
        panic!()
    };
    let unc = key("root-primary")
        .verifying_key()
        .to_encoded_point(false)
        .as_bytes()
        .to_vec();
    for (k, v) in f.iter_mut() {
        if k == "by" {
            *v = Value::Bytes(unc.clone());
        }
    }
    assert!(Op::from_bytes(&Value::Map(f).encode()).is_err());
}

#[test]
fn id_independent_of_signature_and_stable_across_rotation() {
    let (op, r1, r2) = incept();
    let id = op.bucks_id().unwrap();
    let mut other = op.clone();
    other.sig = [7; 64];
    assert_eq!(other.bucks_id().unwrap(), id);

    let tablet = dev("tablet", 2);
    let add = Op::sign(
        1,
        Some(op.hash()),
        OpBody::AddDevice {
            device: tablet.clone(),
        },
        &r1,
    )
    .unwrap();
    let rot = Op::sign(
        2,
        Some(add.hash()),
        OpBody::RotateKeys {
            rotation_keys: vec![key("root-new").public_key(), r2.public_key()],
        },
        &r2,
    )
    .unwrap();
    let st = verify_log(&[op.clone(), add.clone(), rot.clone()]).unwrap();
    assert_eq!(st.id, id, "rotation must not change the id");
    assert!(st.devices.contains(&tablet));
    assert_eq!(st.seq, 2);

    let late = Op::sign(
        3,
        Some(rot.hash()),
        OpBody::RemoveDevice { device_id: [2; 16] },
        &r1,
    )
    .unwrap();
    assert_eq!(
        verify_log(&[op, add, rot, late]).unwrap_err(),
        Error::Log("signer is not a current rotation key")
    );
}

#[test]
fn rejects_tampering_and_bad_chains() {
    let (op, r1, _) = incept();
    let add = Op::sign(
        1,
        Some(op.hash()),
        OpBody::AddDevice {
            device: dev("tablet", 2),
        },
        &r1,
    )
    .unwrap();
    let mut forged = add.clone();
    forged.body = OpBody::AddDevice {
        device: dev("attacker", 9),
    };
    assert_eq!(
        verify_log(&[op.clone(), forged]).unwrap_err(),
        Error::BadSignature
    );

    let s = |seq, prev, body, k: &SigningKey| Op::sign(seq, prev, body, k).unwrap();
    assert!(verify_log(&[
        op.clone(),
        s(
            1,
            Some([0; 32]),
            OpBody::AddDevice {
                device: dev("tablet", 2)
            },
            &r1
        )
    ])
    .is_err());
    assert!(verify_log(&[
        op.clone(),
        s(
            1,
            Some(op.hash()),
            OpBody::AddDevice {
                device: dev("tablet", 2)
            },
            &key("stranger")
        )
    ])
    .is_err());
    assert!(verify_log(&[
        op.clone(),
        s(
            1,
            Some(op.hash()),
            OpBody::AddDevice {
                device: dev("phone", 1)
            },
            &r1
        )
    ])
    .is_err());
    assert!(verify_log(&[
        op.clone(),
        s(
            1,
            Some(op.hash()),
            OpBody::RemoveDevice { device_id: [1; 16] },
            &r1
        )
    ])
    .is_err());
    assert!(verify_log(&[op.clone(), s(1, Some(op.hash()), op.body.clone(), &r1)]).is_err());
    assert!(verify_log(&[]).is_err());
}

#[test]
fn rejects_high_s_on_the_wire() {
    let (mut op, _, _) = incept();
    let sig = Signature::from_slice(&op.sig).unwrap();
    let (r, s) = sig.split_scalars();
    op.sig = Signature::from_scalars(r, -*s).unwrap().to_bytes().into();
    assert_eq!(verify_log(&[op]).unwrap_err(), Error::BadSignature);
}

/// A platform keystore may return high-S signatures. The core must normalise them.
struct HighS(SigningKey);
impl Signer for HighS {
    fn public_key(&self) -> PubKey {
        self.0.public_key()
    }
    fn sign_raw(&self, msg: &[u8]) -> Result<[u8; 64], Error> {
        let s: Signature = p256::ecdsa::signature::Signer::sign(&self.0, msg);
        let s = s.normalize_s().unwrap_or(s);
        let (r, sv) = s.split_scalars();
        Ok(Signature::from_scalars(r, -*sv).unwrap().to_bytes().into())
    }
}

#[test]
fn signer_high_s_output_is_normalised() {
    let r = HighS(key("root-primary"));
    let op = Op::sign(
        0,
        None,
        OpBody::Inception {
            rotation_keys: vec![r.public_key()],
            devices: vec![dev("phone", 1)],
            home: vec![],
        },
        &r,
    )
    .unwrap();
    verify_log(&[op]).unwrap();
}

/// A signer whose key does not match its signatures must be refused at signing time.
struct Mismatched;
impl Signer for Mismatched {
    fn public_key(&self) -> PubKey {
        key("claimed").public_key()
    }
    fn sign_raw(&self, msg: &[u8]) -> Result<[u8; 64], Error> {
        key("actual").sign_raw(msg)
    }
}

#[test]
fn mismatched_signer_is_refused() {
    let r = Mismatched;
    let err = Op::sign(
        0,
        None,
        OpBody::Inception {
            rotation_keys: vec![r.public_key()],
            devices: vec![dev("phone", 1)],
            home: vec![],
        },
        &r,
    )
    .unwrap_err();
    assert!(matches!(err, Error::Signer(_)));
}

#[test]
fn domain_separation_holds() {
    let (op, _, _) = incept();
    let unsigned = op.unsigned_bytes();
    let sig = Signature::from_slice(&op.sig).unwrap();
    let vk = key("root-primary").verifying_key().to_owned();
    assert!(vk
        .verify(&signing_message(Domain::IdentityOp, &unsigned), &sig)
        .is_ok());
    assert!(
        vk.verify(&signing_message(Domain::Event, &unsigned), &sig)
            .is_err(),
        "op signature must not verify as an event"
    );
    assert!(
        vk.verify(&unsigned, &sig).is_err(),
        "raw bytes without a tag must not verify"
    );
}

#[test]
fn limits_are_enforced() {
    let r = key("root-primary");
    let inc = |rk: Vec<PubKey>, devices: Vec<Device>, home: Vec<String>| {
        verify_log(&[Op::sign(
            0,
            None,
            OpBody::Inception {
                rotation_keys: rk,
                devices,
                home,
            },
            &r,
        )
        .unwrap()])
    };
    let rk = || vec![r.public_key()];
    assert!(inc(
        rk(),
        vec![dev("phone", 1)],
        vec!["x".repeat(MAX_HOME_ENTRY)]
    )
    .is_ok());
    assert!(inc(
        rk(),
        vec![dev("phone", 1)],
        vec!["x".repeat(MAX_HOME_ENTRY + 1)]
    )
    .is_err());
    assert!(inc(rk(), vec![dev("phone", 1)], vec!["h".into(); MAX_HOME + 1]).is_err());
    let mut long = dev("phone", 1);
    long.name = "n".repeat(MAX_DEVICE_NAME + 1);
    assert!(inc(rk(), vec![long], vec![]).is_err());
    let four: Vec<PubKey> = (0..4).map(|i| key(&format!("k{i}")).public_key()).collect();
    let mut four_with_signer = four.clone();
    four_with_signer[0] = r.public_key();
    assert!(inc(four_with_signer, vec![dev("phone", 1)], vec![]).is_err());
    let many: Vec<Device> = (0..=MAX_DEVICES as u8)
        .map(|i| dev(&format!("d{i}"), i))
        .collect();
    assert!(inc(rk(), many, vec![]).is_err());
    assert!(
        inc(
            vec![r.public_key(), r.public_key()],
            vec![dev("phone", 1)],
            vec![]
        )
        .is_err(),
        "duplicate rotation keys"
    );
}

#[test]
fn cbor_map_order_is_canonical() {
    let a = Value::Map(vec![
        ("bb".into(), Value::Uint(1)),
        ("a".into(), Value::Uint(2)),
        ("ccc".into(), Value::Null),
    ]);
    let b = Value::Map(vec![
        ("ccc".into(), Value::Null),
        ("a".into(), Value::Uint(2)),
        ("bb".into(), Value::Uint(1)),
    ]);
    assert_eq!(a.encode(), b.encode());
    assert_eq!(
        a.encode(),
        vec![0xa3, 0x61, b'a', 0x02, 0x62, b'b', b'b', 0x01, 0x63, b'c', b'c', b'c', 0xf6]
    );
}

/// Cross-implementation vectors. Regenerate with `UPDATE_VECTORS=1 cargo test`.
#[test]
fn vectors() {
    let (op, r1, r2) = incept();
    let add = Op::sign(
        1,
        Some(op.hash()),
        OpBody::AddDevice {
            device: dev("laptop", 2),
        },
        &r1,
    )
    .unwrap();
    let rot = Op::sign(
        2,
        Some(add.hash()),
        OpBody::RotateKeys {
            rotation_keys: vec![key("root-new").public_key(), r2.public_key()],
        },
        &r2,
    )
    .unwrap();
    let st = verify_log(&[op.clone(), add.clone(), rot.clone()]).unwrap();
    let doc = serde_json::json!({
        "spec": "SPEC-001 v0.2 (docs/specs/001-identity.md), SPEC-003 v0.2",
        "warning": "TEST KEYS ONLY. Each secret is BLAKE3(label). Never use for anything real.",
        "signature": "ECDSA P-256 SHA-256, low-S, over 'bucks/identity-op/v1' || 0x00 || unsigned_bytes; RFC 6979 deterministic nonces",
        "hash_contexts": { "id": "bucks 2026-10-09 identity id v1", "op_hash": "bucks 2026-10-09 identity op hash v1", "short_code": "bucks 2026-10-09 short code v1" },
        "test_secret_keys": { "root-primary": hex(&key("root-primary").to_bytes()),
                              "root-backup": hex(&key("root-backup").to_bytes()),
                              "root-new": hex(&key("root-new").to_bytes()),
                              "phone": hex(&key("phone").to_bytes()),
                              "laptop": hex(&key("laptop").to_bytes()) },
        "bucks_id": st.id.to_string(),
        "did": st.id.did(),
        "short_code": st.id.short_code(),
        "inception_unsigned_hex": hex(&op.unsigned_bytes()),
        "log_hex": [hex(&op.to_bytes()), hex(&add.to_bytes()), hex(&rot.to_bytes())],
        "op_hashes": [hex(&op.hash()), hex(&add.hash()), hex(&rot.hash())],
        "final_state": { "seq": st.seq, "head": hex(&st.head),
                         "rotation_keys": st.rotation_keys.iter().map(|k| k.did_key()).collect::<Vec<_>>(),
                         "devices": st.devices.iter().map(|d| d.name.clone()).collect::<Vec<_>>() },
    });
    check_vector_file("identity-v1.json", &doc);
}

pub fn check_vector_file(name: &str, doc: &serde_json::Value) {
    let path = format!(
        "{}/../../spec/test-vectors/{name}",
        env!("CARGO_MANIFEST_DIR")
    );
    let text = serde_json::to_string_pretty(doc).unwrap() + "\n";
    if std::env::var("UPDATE_VECTORS").is_ok() {
        std::fs::write(&path, &text).unwrap();
    }
    assert_eq!(
        std::fs::read_to_string(&path).expect("vectors missing"),
        text,
        "vector mismatch: protocol break, or regenerate deliberately"
    );
}
