//! Shared test helpers. Keys are derived from public labels: TEST ONLY, never real.
#![allow(dead_code)]
use bucks_core::*;
use p256::ecdsa::SigningKey;

pub fn key(label: &str) -> SigningKey {
    SigningKey::from_slice(blake3::hash(label.as_bytes()).as_bytes()).unwrap()
}
pub fn dev(label: &str, n: u8) -> Device {
    Device {
        id: [n; 16],
        key: key(label).public_key(),
        name: label.into(),
    }
}
pub fn hex(b: &[u8]) -> String {
    b.iter().map(|x| format!("{x:02x}")).collect()
}
/// Inception with a primary and a backup rotation key and one phone.
pub fn incept() -> (Op, SigningKey, SigningKey) {
    let (r1, r2) = (key("root-primary"), key("root-backup"));
    let op = Op::sign(
        0,
        None,
        OpBody::Inception {
            rotation_keys: vec![r1.public_key(), r2.public_key()],
            devices: vec![dev("phone", 1)],
            home: vec!["https://node.example".into()],
        },
        &r1,
    )
    .unwrap();
    (op, r1, r2)
}
