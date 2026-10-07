use super::{
    Error, RandomId,
    cbor::{self, Bytes, Class, Decode, Domain, Encode, Reader, Separated, Text, Writer},
    crypto, kem_key_id,
    secret::Secret32,
    signature_key_id,
};

// Synthetic exact schema only, NOT a certificate, PoP, bundle or CRL.
#[derive(Debug, PartialEq)]
struct Fixture<'a> {
    number: u64,
    bytes: &'a [u8],
}
impl Encode for Fixture<'_> {
    fn encode(&self, w: &mut Writer) -> Result<(), Error> {
        w.map([
            ("bbb", &Bytes(self.bytes) as &dyn Encode),
            ("a", &self.number as &dyn Encode),
        ])
    }
}
impl<'a> Decode<'a> for Fixture<'a> {
    fn decode(r: &mut Reader<'a, '_>) -> Result<Self, Error> {
        let mut number = None;
        let mut bytes = None;
        r.map(["a", "bbb"], |r, i| match i {
            0 => {
                number = Some(r.uint()?);
                Ok(())
            }
            1 => {
                bytes = Some(r.bytes()?);
                Ok(())
            }
            _ => Err(Error::Schema),
        })?;
        Ok(Self {
            number: number.ok_or(Error::Schema)?,
            bytes: bytes.ok_or(Error::Schema)?,
        })
    }
}
struct Number(u64);
impl Encode for Number {
    fn encode(&self, w: &mut Writer) -> Result<(), Error> {
        w.uint(self.0)
    }
}
impl<'a> Decode<'a> for Number {
    fn decode(r: &mut Reader<'a, '_>) -> Result<Self, Error> {
        Ok(Self(r.uint()?))
    }
}
struct StringValue<'a>(&'a str);
impl Encode for StringValue<'_> {
    fn encode(&self, w: &mut Writer) -> Result<(), Error> {
        w.text(self.0)
    }
}
impl<'a> Decode<'a> for StringValue<'a> {
    fn decode(r: &mut Reader<'a, '_>) -> Result<Self, Error> {
        Ok(Self(r.text()?))
    }
}
struct ByteValue<'a>(&'a [u8]);
impl Encode for ByteValue<'_> {
    fn encode(&self, w: &mut Writer) -> Result<(), Error> {
        w.bytes(self.0)
    }
}
impl<'a> Decode<'a> for ByteValue<'a> {
    fn decode(r: &mut Reader<'a, '_>) -> Result<Self, Error> {
        Ok(Self(r.bytes()?))
    }
}
fn valid_fixture(bytes: &[u8]) -> Result<Fixture<'_>, Error> {
    cbor::validated(bytes, Class::Small)
}
const GOLDEN: &[u8] = &[
    0xa2, 0x61, b'a', 0x18, 24, 0x63, b'b', b'b', b'b', 0x42, 1, 2,
];

#[test]
fn p2a_cbor_001_canonical_values_and_exact_schema() {
    let fixture = Fixture {
        number: 24,
        bytes: &[1, 2],
    };
    assert_eq!(cbor::deterministic(&fixture, Class::Small).unwrap(), GOLDEN);
    assert_eq!(valid_fixture(GOLDEN).unwrap(), fixture);
    for value in [
        0,
        1,
        23,
        24,
        255,
        256,
        65535,
        65536,
        u32::MAX as u64,
        u64::MAX,
    ] {
        let bytes = cbor::deterministic(&Number(value), Class::Small).unwrap();
        assert_eq!(
            cbor::validated::<Number>(&bytes, Class::Small).unwrap().0,
            value
        );
        assert_eq!(
            bytes,
            cbor::deterministic(&Number(value), Class::Small).unwrap()
        );
    }
    for value in ["", "hello", "é"] {
        let bytes = cbor::deterministic(&Text(value), Class::Small).unwrap();
        assert_eq!(
            cbor::validated::<StringValue>(&bytes, Class::Small)
                .unwrap()
                .0,
            value
        );
    }
}
#[test]
fn p2a_cbor_002_reject_nonshortest_and_length_encodings() {
    for bytes in [
        &[0x18, 0][..],
        &[0x19, 0, 24],
        &[0x1a, 0, 0, 0, 24],
        &[0x1b, 0, 0, 0, 0, 0, 0, 0, 24],
    ] {
        assert!(matches!(
            cbor::validated::<Number>(bytes, Class::Small),
            Err(Error::NonCanonical)
        ));
    }
    for bytes in [&[0x78, 1, b'a'][..], &[0x79, 0, 1, b'a']] {
        assert!(matches!(
            cbor::validated::<StringValue>(bytes, Class::Small),
            Err(Error::NonCanonical)
        ));
    }
    assert!(matches!(
        cbor::validated::<ByteValue>(&[0x58, 1, 7], Class::Small),
        Err(Error::NonCanonical)
    ));
    let mut map = vec![0xb8, 2];
    map.extend_from_slice(&GOLDEN[1..]);
    assert_eq!(valid_fixture(&map), Err(Error::NonCanonical));
}
#[test]
fn p2a_cbor_003_forbidden_forms() {
    for bytes in [
        &[0x9f, 0xff][..],
        &[0xbf, 0xff],
        &[0x5f, 0xff],
        &[0x7f, 0xff],
        &[0xf9, 0, 0],
        &[0xfa, 0, 0, 0, 0],
        &[0xfb, 0, 0, 0, 0, 0, 0, 0, 0],
        &[0xc0, 0],
        &[0xf4],
        &[0xf5],
        &[0xf6],
        &[0xf7],
        &[0xff],
        &[0x20],
    ] {
        assert!(matches!(
            cbor::validated::<Number>(bytes, Class::Small),
            Err(Error::ForbiddenCbor)
        ));
    }
}
#[test]
fn p2a_cbor_004_duplicates_unknown_order_trailing() {
    assert_eq!(
        valid_fixture(&[0xa2, 0x61, b'a', 0, 0x61, b'a', 0]),
        Err(Error::DuplicateField)
    );
    assert_eq!(
        valid_fixture(&[0xa2, 0x61, b'a', 0, 0x63, b'c', b'c', b'c', 0x40]),
        Err(Error::UnknownField)
    );
    let wrong_order = [0xa2, 0x63, b'b', b'b', b'b', 0x40, 0x61, b'a', 0];
    assert_eq!(valid_fixture(&wrong_order), Err(Error::NonCanonical));
    let mut trailing = GOLDEN.to_vec();
    trailing.push(0);
    assert_eq!(valid_fixture(&trailing), Err(Error::TrailingBytes));
    assert_eq!(valid_fixture(&[0xa0]), Err(Error::Schema));
    assert_eq!(valid_fixture(&[0xa1, 0, 0]), Err(Error::Schema));
    // Identical decoded key, alternate spelling: never overwrite a field.
    assert!(valid_fixture(&[0xa2, 0x61, b'a', 0, 0x78, 1, b'a', 0]).is_err());
}
#[test]
fn p2a_cbor_005_truncation_oversized_and_utf8() {
    for n in 0..GOLDEN.len() {
        assert!(valid_fixture(&GOLDEN[..n]).is_err());
    }
    for bytes in [
        &[0x5b, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff][..],
        &[0x9b, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff],
        &[0xbb, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff],
        &[0x61, 0xff],
        &[0x1c],
    ] {
        assert!(valid_fixture(bytes).is_err());
    }
    assert_eq!(valid_fixture(&vec![0; 16385]), Err(Error::SizeLimit));
    assert!(matches!(
        cbor::deterministic(&Bytes(&vec![0; 32769]), Class::Bundle),
        Err(Error::SizeLimit)
    ));
}

struct List(Vec<u64>); // Test-only typed list; no production generic Value tree.
impl Encode for List {
    fn encode(&self, w: &mut Writer) -> Result<(), Error> {
        w.array(self.0.len() as u64)?;
        for v in &self.0 {
            w.uint(*v)?;
        }
        Ok(())
    }
}
impl<'a> Decode<'a> for List {
    fn decode(r: &mut Reader<'a, '_>) -> Result<Self, Error> {
        let mut values = Vec::new();
        r.array(255, |r| {
            for _ in 0..255 {
                values.push(r.uint()?);
            }
            Ok(())
        })?;
        Ok(Self(values))
    }
}
#[test]
fn p2a_cbor_006_item_depth_and_embedded_budget() {
    let max = List(vec![0; 255]);
    let bytes = cbor::deterministic(&max, Class::Small).unwrap();
    assert_eq!(
        cbor::validated::<List>(&bytes, Class::Small)
            .unwrap()
            .0
            .len(),
        255
    );
    assert!(matches!(
        cbor::deterministic(&List(vec![0; 256]), Class::Small),
        Err(Error::ItemLimit)
    ));
    let mut nested = vec![0x81; 8];
    nested.push(0); // primitive at depth 9
    assert!(matches!(
        cbor::validated::<Number>(&nested, Class::Small),
        Err(Error::DepthLimit)
    ));
    struct Embedded<'a>(&'a [u8]);
    impl Encode for Embedded<'_> {
        fn encode(&self, w: &mut Writer) -> Result<(), Error> {
            w.bytes(self.0)
        }
    }
    impl<'a> Decode<'a> for Embedded<'a> {
        fn decode(r: &mut Reader<'a, '_>) -> Result<Self, Error> {
            let _: List = r.embedded(Class::Small)?;
            Ok(Self(&[]))
        }
    }
    let outer = cbor::deterministic(&Bytes(&bytes), Class::Small).unwrap();
    assert!(matches!(
        cbor::validated::<Embedded>(&outer, Class::Small),
        Err(Error::ItemLimit)
    ));
    // CRL aggregate bound, before any loop/allocation from declared length.
    assert!(matches!(
        cbor::validated::<List>(&[0x9a, 0, 0x0f, 0x42, 0x40], Class::Crl),
        Err(Error::ItemLimit)
    ));
    assert_eq!(cbor::MAX_CRL_ENTRIES, 100_000);
}
#[test]
fn p2a_cbor_007_fixed_domains() {
    let encoded = cbor::deterministic(
        &Separated {
            domain: Domain::PopKemKey,
            body: &Bytes(&[0; 32]),
        },
        Class::Small,
    )
    .unwrap();
    let mut d = minicbor::Decoder::new(&encoded);
    assert_eq!(d.array().unwrap(), Some(2));
    assert_eq!(d.str().unwrap(), "CADP-POP-KEM-KEY-v1");
    assert_eq!(d.bytes().unwrap(), &[0; 32]);
    assert_eq!(d.position(), encoded.len());
    let labels = [
        Domain::Certificate,
        Domain::Crl,
        Domain::PopChallenge,
        Domain::PopKem,
        Domain::PopSignature,
    ];
    let exact = [
        "CADP-CERT-v1",
        "CADP-CRL-v1",
        "CADP-POP-CHALLENGE-v1",
        "CADP-POP-KEM-v1",
        "CADP-POP-SIG-v1",
    ];
    for (domain, label) in labels.into_iter().zip(exact) {
        let bytes = cbor::deterministic(
            &Separated {
                domain,
                body: &0u64,
            },
            Class::Small,
        )
        .unwrap();
        let mut d = minicbor::Decoder::new(&bytes);
        d.array().unwrap();
        assert_eq!(d.str().unwrap(), label);
    }
}
#[test]
fn p2a_cbor_008_deterministic_properties_and_mutations() {
    for n in 0..256u64 {
        let public = [n as u8; 8];
        let fixture = Fixture {
            number: n,
            bytes: &public,
        };
        let bytes = cbor::deterministic(&fixture, Class::Small).unwrap();
        assert_eq!(valid_fixture(&bytes).unwrap(), fixture);
        for i in 0..bytes.len() {
            let mut altered = bytes.clone();
            altered[i] ^= 0x80;
            if let Ok(value) = valid_fixture(&altered) {
                assert_eq!(cbor::deterministic(&value, Class::Small).unwrap(), altered);
            }
        }
    }
    for length in 0..80 {
        let mut v = vec![0u8; length];
        for seed in 0..16u8 {
            for (i, b) in v.iter_mut().enumerate() {
                *b = seed.wrapping_mul(17).wrapping_add(i as u8);
            }
            let _ = valid_fixture(&v);
        }
    }
}
#[test]
fn p2a_id_001_lengths_determinism_difference_vectors() {
    let kem = [0u8; 1184];
    let sig = [0u8; 1952];
    // Public all-zero byte fixtures, not usable generated identities or secrets.
    assert_eq!(
        kem_key_id(&kem).unwrap().as_bytes().as_slice(),
        hex("1e41ffea04d255bc186b21eefa9039f6b92347bc005433f6cc11f552c9efbd6d")
    );
    assert_eq!(
        signature_key_id(&sig).unwrap().as_bytes().as_slice(),
        hex("7b2447cb726e5a16083ba09e8502cb9761ac6c2027b995024af4849c73462bf7")
    );
    assert_eq!(kem_key_id(&kem).unwrap(), kem_key_id(&kem).unwrap());
    assert_eq!(
        signature_key_id(&sig).unwrap(),
        signature_key_id(&sig).unwrap()
    );
    assert_ne!(kem_key_id(&kem).unwrap(), kem_key_id(&[1; 1184]).unwrap());
    assert_ne!(
        signature_key_id(&sig).unwrap(),
        signature_key_id(&[1; 1952]).unwrap()
    );
    for length in [0, 1183, 1185, 1952] {
        assert_eq!(kem_key_id(&vec![0; length]), Err(Error::IncorrectKeyLength));
    }
    for length in [0, 1951, 1953, 1184] {
        assert_eq!(
            signature_key_id(&vec![0; length]),
            Err(Error::IncorrectKeyLength)
        );
    }
}
#[test]
fn p2a_rng_001_os_random_identifiers() {
    let a = RandomId::generate().unwrap();
    let b = RandomId::generate().unwrap();
    assert_eq!(a.as_bytes().len(), 32);
    assert_ne!(a, b);
}
fn hex(s: &str) -> Vec<u8> {
    s.as_bytes()
        .as_chunks::<2>()
        .0
        .iter()
        .map(|v| u8::from_str_radix(std::str::from_utf8(v).unwrap(), 16).unwrap())
        .collect()
}
#[test]
fn p2a_hash_001_sha256_vector() {
    assert_eq!(
        crypto::sha256(b"abc").as_slice(),
        hex("ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad")
    );
}
#[test]
fn p2a_mac_001_rfc4231_and_rejections() {
    // RFC 4231 case 1: zero extension preserves HMAC's block-padded key.
    let mut key = Secret32::zeroed().unwrap();
    key.borrow_mut()[..20].fill(0x0b);
    let tag = crypto::hmac_sha256(&key, b"Hi There").unwrap();
    assert_eq!(
        tag.as_bytes().as_slice(),
        hex("b0344c61d8db38535ca8afceaf0bf12b881dc200c9833da726e9376c2e32cff7")
    );
    assert_eq!(
        crypto::verify_hmac_sha256(&key, b"Hi There", tag.as_bytes()),
        Ok(())
    );
    let mut bad = *tag.as_bytes();
    bad[0] ^= 1;
    for bytes in [&bad[..], &tag.as_bytes()[..31], &[]] {
        assert_eq!(
            crypto::verify_hmac_sha256(&key, b"Hi There", bytes),
            Err(Error::AuthenticationFailure)
        );
    }
    assert_eq!(
        crypto::verify_hmac_sha256(&key, b"tampered", tag.as_bytes()),
        Err(Error::AuthenticationFailure)
    );
}
#[test]
fn p2a_kdf_001_rfc5869_fixed32() {
    let key = crypto::hkdf_sha256(
        &[0x0b; 22],
        &hex("000102030405060708090a0b0c"),
        &hex("f0f1f2f3f4f5f6f7f8f9"),
    )
    .unwrap();
    assert_eq!(
        key.borrow(),
        hex("3cb25f25faacd57a90434f64d0362f2a2d2d0a90cf1a5a4c5db02d56ecc4c5bf")
    );
    assert_eq!(key.borrow().len(), 32);
}
#[test]
fn p2a_secret_001_redaction_and_live_drop() {
    use std::sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    };
    let observed = Arc::new(AtomicBool::new(false));
    let copy = Arc::clone(&observed);
    let mut owner = Secret32::zeroed().unwrap();
    owner.borrow_mut().fill(0x5a);
    assert_eq!(format!("{owner:?}"), "Secret32([REDACTED])");
    owner.observer = Some(Box::new(move |bytes| {
        assert_eq!(bytes, &[0; 32]);
        copy.store(true, Ordering::SeqCst);
    }));
    drop(owner);
    assert!(observed.load(Ordering::SeqCst));
}

#[test]
fn p2a_cbor_009_acceptance_requires_exact_reencoding() {
    // Deliberately defective TEST schema: dropping a decoded value must not make
    // the original bytes acceptable for any subsequent cryptographic operation.
    struct Discarded;
    impl Encode for Discarded {
        fn encode(&self, w: &mut Writer) -> Result<(), Error> {
            w.uint(0)
        }
    }
    impl<'a> Decode<'a> for Discarded {
        fn decode(r: &mut Reader<'a, '_>) -> Result<Self, Error> {
            r.uint()?;
            Ok(Self)
        }
    }
    assert!(matches!(
        cbor::validated::<Discarded>(&[1], Class::Small),
        Err(Error::NonCanonical)
    ));
    // Stable schema must also reject its own overlong array representation.
    struct Pair;
    impl Encode for Pair {
        fn encode(&self, w: &mut Writer) -> Result<(), Error> {
            w.array(2)?;
            w.uint(0)?;
            w.uint(1)
        }
    }
    impl<'a> Decode<'a> for Pair {
        fn decode(r: &mut Reader<'a, '_>) -> Result<Self, Error> {
            r.array(2, |r| {
                if r.uint()? != 0 || r.uint()? != 1 {
                    return Err(Error::Schema);
                }
                Ok(Self)
            })
        }
    }
    assert!(cbor::validated::<Pair>(&[0x82, 0, 1], Class::Small).is_ok());
    assert!(matches!(
        cbor::validated::<Pair>(&[0x98, 2, 0, 1], Class::Small),
        Err(Error::NonCanonical)
    ));
}
