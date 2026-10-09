//! Property-based fuzzing of the decoders and verifiers (stable Rust, proptest).
//! Coverage-guided fuzzing with libFuzzer is a later addition; see docs/roadmap.md M1.
mod common;
use bucks_core::cbor::Value;
use bucks_core::*;
use common::*;
use proptest::prelude::*;

fn value() -> impl Strategy<Value = Value> {
    let leaf = prop_oneof![
        Just(Value::Null),
        any::<u64>().prop_map(Value::Uint),
        proptest::collection::vec(any::<u8>(), 0..40).prop_map(Value::Bytes),
        ".{0,20}".prop_map(Value::Text),
    ];
    leaf.prop_recursive(4, 64, 8, |inner| {
        prop_oneof![
            proptest::collection::vec(inner.clone(), 0..8).prop_map(Value::Array),
            proptest::collection::btree_map(".{0,8}", inner, 0..8)
                .prop_map(|m| Value::Map(m.into_iter().collect())),
        ]
    })
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(2000))]

    #[test]
    fn decode_never_panics_and_accepts_only_canonical(bytes in proptest::collection::vec(any::<u8>(), 0..256)) {
        if let Ok(v) = Value::decode(&bytes) {
            prop_assert_eq!(v.encode(), bytes);
        }
    }

    #[test]
    fn encode_decode_roundtrip(v in value()) {
        let enc = v.encode();
        let dec = Value::decode(&enc).expect("own encoding must decode");
        prop_assert_eq!(dec.encode(), enc);
    }

    #[test]
    fn op_and_event_parsers_never_panic(bytes in proptest::collection::vec(any::<u8>(), 0..512)) {
        let _ = Op::from_bytes(&bytes);
        let _ = Event::from_bytes(&bytes);
    }

}

proptest! {
    // Each case signs an inception, so fewer cases keep CI fast.
    #![proptest_config(ProptestConfig::with_cases(300))]

    #[test]
    fn no_single_byte_mutation_of_a_signed_op_verifies(pos in any::<prop::sample::Index>(), x in 1u8..=255) {
        let (op, _, _) = incept();
        let mut b = op.to_bytes();
        let i = pos.index(b.len());
        b[i] ^= x;
        if let Ok(m) = Op::from_bytes(&b) {
            prop_assert!(verify_log(&[m]).is_err());
        }
    }
}
