use std::cell::{Cell, RefCell};
thread_local! {
    static DROPS: RefCell<Vec<(&'static str,usize,bool)>> = const { RefCell::new(Vec::new()) };
    static WORKSPACES: Cell<usize> = const { Cell::new(0) };
    static KDFS: Cell<usize> = const { Cell::new(0) };
    static SERIAL_FAIL: Cell<bool> = const { Cell::new(false) };
    static WRITE_FAIL: Cell<bool> = const { Cell::new(false) };
}
pub(super) fn observe_drop(kind: &'static str, bytes: &[u8]) {
    DROPS.with_borrow_mut(|v| v.push((kind, bytes.len(), bytes.iter().all(|b| *b == 0))));
}
pub(super) fn observe_workspace(blocks: &[argon2::Block]) {
    assert!(blocks.iter().all(|b| b.as_ref().iter().all(|v| *v == 0)));
}
pub(super) fn record_workspace() {
    WORKSPACES.set(WORKSPACES.get() + 1);
}
pub(super) fn record_kdf() {
    KDFS.set(KDFS.get() + 1);
}
pub(super) fn serialization_fault() -> Result<(), super::Error> {
    if SERIAL_FAIL.replace(false) {
        Err(super::Error::SerializationFailure)
    } else {
        Ok(())
    }
}
pub(super) fn partial_write_fault() -> bool {
    WRITE_FAIL.replace(false)
}

use super::schema::{Bundle, Header, Kdf, Protected, Role};
use super::*;
use crate::foundation::{
    RandomId,
    cbor::{self, Bytes, Class, Text},
};
use std::os::unix::fs::{DirBuilderExt, PermissionsExt};
use std::{fs, path::PathBuf};

struct Fixture {
    pk: [u8; 1184],
    sk: kem::SecretKey,
    sp: [u8; 1952],
    ss: signature::SecretKey,
    pw: Password,
    id: [u8; 32],
}
impl Fixture {
    fn new() -> Self {
        let (pk, sk) = kem::generate_keypair().unwrap();
        let (sp, ss) = signature::generate_keypair().unwrap();
        Self {
            pk,
            sk,
            sp,
            ss,
            pw: Password::fixture("PUBLIC TEST PASSWORD"),
            id: *RandomId::generate().unwrap().as_bytes(),
        }
    }
    fn identity(&self) -> Identity<'_> {
        Identity::new(self.id, &self.pk, &self.sp).unwrap()
    }
    fn bundle(&self) -> EncryptedBundle {
        create_with_rng(
            &self.pw,
            &self.identity(),
            &self.sk,
            &self.ss,
            KdfParameters::new(65536, 3, 4).unwrap(),
            &mut random,
        )
        .unwrap()
    }
    fn reject_auth(&self, bytes: &[u8]) {
        assert!(matches!(
            unlock_bundle(bytes, &self.pw, &self.identity()),
            Err(Error::AuthenticationFailure)
        ));
    }
}
fn output(b: &Bundle<'_>) -> Vec<u8> {
    schema::encode(b).unwrap()
}
fn drop_ok(kind: &str, length: usize) -> bool {
    DROPS.with_borrow(|v| v.iter().any(|(k, n, ok)| *k == kind && *n == length && *ok))
}
fn clear_drops() {
    DROPS.with_borrow_mut(Vec::clear);
}
fn encode_bytes(value: &[u8]) -> Vec<u8> {
    cbor::deterministic(&Bytes(value), Class::Bundle).unwrap()
}
fn encode_text(value: &str) -> Vec<u8> {
    cbor::deterministic(&Text(value), Class::Bundle).unwrap()
}
fn encode_uint(value: u64) -> Vec<u8> {
    cbor::deterministic(&value, Class::Bundle).unwrap()
}
// Test-only span operations on an existing bounded public/ciphertext fixture.
// No production decoder skip/generic CBOR tree is introduced.
fn span(bytes: &[u8], path: &[&str]) -> std::ops::Range<usize> {
    if path.is_empty() {
        return 0..bytes.len();
    }
    let mut decoder = minicbor::Decoder::new(bytes);
    let n = decoder.map().unwrap().unwrap();
    for _ in 0..n {
        let key = decoder.str().unwrap();
        let start = decoder.position();
        decoder.skip().unwrap();
        let end = decoder.position();
        if key == path[0] {
            let sub = span(&bytes[start..end], &path[1..]);
            return start + sub.start..start + sub.end;
        }
    }
    panic!("test fixture field not found")
}
fn replace(bytes: &[u8], path: &[&str], value: &[u8]) -> Vec<u8> {
    let r = span(bytes, path);
    let mut result = bytes[..r.start].to_vec();
    result.extend_from_slice(value);
    result.extend_from_slice(&bytes[r.end..]);
    result
}
fn extra_field(bytes: &[u8], path: &[&str], duplicate: bool) -> Vec<u8> {
    let range = span(bytes, path);
    let object = &bytes[range.clone()];
    let mut d = minicbor::Decoder::new(object);
    let n = d.map().unwrap().unwrap();
    let mut entries = Vec::new();
    for _ in 0..n {
        let k = d.position();
        d.str().unwrap();
        let v = d.position();
        d.skip().unwrap();
        let end = d.position();
        entries.push((object[k..v].to_vec(), object[v..end].to_vec()));
    }
    if duplicate {
        entries.push(entries[0].clone());
    } else {
        entries.push((vec![0x61, b'x'], vec![0]));
    }
    entries.sort_by(|a, b| a.0.cmp(&b.0));
    let mut encoded = vec![0xa0 + u8::try_from(entries.len()).unwrap()];
    for (k, v) in entries {
        encoded.extend(k);
        encoded.extend(v);
    }
    replace(bytes, path, &encoded)
}

#[test]
fn p2b_kdf_001_deterministic_and_salt_password_separation() {
    let p = KdfParameters::new(65536, 3, 4).unwrap();
    let a = kdf::derive(b"PUBLIC INPUT", &[1; 16], p).unwrap();
    let b = kdf::derive(b"PUBLIC INPUT", &[1; 16], p).unwrap();
    let c = kdf::derive(b"CHANGED PUBLIC INPUT", &[1; 16], p).unwrap();
    let d = kdf::derive(b"PUBLIC INPUT", &[2; 16], p).unwrap();
    assert!(a.borrow() == b.borrow());
    assert!(a.borrow() != c.borrow());
    assert!(a.borrow() != d.borrow());
    assert_eq!(a.borrow().len(), 32);
}
#[test]
fn p2b_kdf_002_bounds_before_work() {
    let before = KDFS.get();
    let allocations = WORKSPACES.get();
    for (m, t, p) in [
        (0, 3, 4),
        (65535, 3, 4),
        (262145, 3, 4),
        (u64::MAX, 3, 4),
        (65536, 0, 4),
        (65536, 2, 4),
        (65536, 7, 4),
        (65536, u64::MAX, 4),
        (65536, 3, 0),
        (65536, 3, 5),
        (65536, 3, u64::MAX),
    ] {
        assert_eq!(
            KdfParameters::new(m, t, p),
            Err(Error::InvalidKdfParameters)
        );
    }
    for (m, t, p) in [(65536, 3, 1), (262144, 6, 4)] {
        assert!(KdfParameters::new(m, t, p).is_ok());
    }
    for costs in [
        KdfParameters {
            memory_kib: 0,
            time_cost: 3,
            parallelism: 4,
        },
        KdfParameters {
            memory_kib: u32::MAX,
            time_cost: 3,
            parallelism: 4,
        },
    ] {
        assert!(matches!(
            kdf::derive(b"PUBLIC", &[0; 16], costs),
            Err(Error::InvalidKdfParameters)
        ));
    }
    assert!(matches!(
        kdf::derive(
            b"PUBLIC",
            &[0; 15],
            KdfParameters::new(65536, 3, 4).unwrap()
        ),
        Err(Error::InvalidKdfParameters)
    ));
    assert_eq!(KDFS.get(), before);
    assert_eq!(WORKSPACES.get(), allocations);
}
#[test]
fn p2b_kdf_003_maximum_and_workspace_cleanup() {
    clear_drops();
    let key = kdf::derive(
        b"PUBLIC MAXIMUM PROFILE",
        &[1; 16],
        KdfParameters::new(262144, 6, 4).unwrap(),
    )
    .unwrap();
    assert_eq!(key.borrow().len(), 32);
    drop(key);
    assert!(drop_ok("kek", 32));
}
#[test]
fn p2b_bundle_001_canonical_roundtrip_and_public_binding() {
    let f = Fixture::new();
    let encrypted = create_bundle(&f.pw, &f.identity(), &f.sk, &f.ss).unwrap();
    let decoded = schema::parse(encrypted.as_bytes()).unwrap();
    assert_eq!(decoded.header.kdf.memory_kib, 131072);
    assert_eq!(decoded.header.kdf.time_cost, 6);
    assert!(output(&decoded) == encrypted.as_bytes());
    assert_eq!(decoded.kem.ciphertext_and_tag.len(), 2416);
    assert_eq!(decoded.signature.ciphertext_and_tag.len(), 4048);
    let recovered = unlock_bundle(encrypted.as_bytes(), &f.pw, &f.identity()).unwrap();
    let (ct, shared) = kem::encapsulate(&f.pk).unwrap();
    assert_eq!(shared, kem::decapsulate(&recovered.kem, &ct).unwrap());
    let proof = signature::sign(&recovered.signature, b"PUBLIC STORAGE ROUNDTRIP").unwrap();
    assert!(signature::verify(&f.sp, b"PUBLIC STORAGE ROUNDTRIP", &proof).unwrap());
    assert!(Identity::new(f.id, &f.sp, &f.pk).is_err());
    let other = Fixture::new();
    assert!(matches!(
        create_bundle(&f.pw, &f.identity(), &other.sk, &f.ss),
        Err(Error::AuthenticationFailure)
    ));
    assert!(matches!(
        create_bundle(&f.pw, &f.identity(), &f.sk, &other.ss),
        Err(Error::AuthenticationFailure)
    ));
}
#[test]
fn p2b_aead_001_wrong_password_generic_error() {
    let f = Fixture::new();
    let encrypted = f.bundle();
    let bad = Password::fixture("DIFFERENT PUBLIC TEST INPUT");
    let e = unlock_bundle(encrypted.as_bytes(), &bad, &f.identity()).unwrap_err();
    assert_eq!(e, Error::AuthenticationFailure);
    assert_eq!(e.to_string(), "authentication failed");
}
#[test]
fn p2b_aead_002_ciphertext_tag_nonce_tampering() {
    let f = Fixture::new();
    let encrypted = f.bundle();
    for role in ["kem", "signature"] {
        for last in [false, true] {
            let mut encoded = encrypted.as_bytes().to_vec();
            let r = span(&encoded, &[role, "ciphertext_and_tag"]);
            let mut decoder = minicbor::Decoder::new(&encoded[r.clone()]);
            let mut bytes = decoder.bytes().unwrap().to_vec();
            let n = if last { bytes.len() - 1 } else { 0 };
            bytes[n] ^= 1;
            encoded = replace(
                &encoded,
                &[role, "ciphertext_and_tag"],
                &encode_bytes(&bytes),
            );
            f.reject_auth(&encoded);
        }
        let mut nonce = [0u8; 12];
        let b = schema::parse(encrypted.as_bytes()).unwrap();
        nonce.copy_from_slice(if role == "kem" {
            b.kem.nonce
        } else {
            b.signature.nonce
        });
        nonce[0] ^= 1;
        f.reject_auth(&replace(
            encrypted.as_bytes(),
            &[role, "nonce"],
            &encode_bytes(&nonce),
        ));
    }
}
#[test]
fn p2b_aad_001_independent_vectors_and_role_authentication() {
    let h = Header {
        schema_version: 1,
        recipient_id: &[1; 32],
        kem_algorithm: "ML-KEM-768",
        kem_key_id: &[2; 32],
        signature_algorithm: "ML-DSA-65",
        signature_key_id: &[3; 32],
        aead_algorithm: "AES-256-GCM",
        kdf: Kdf {
            algorithm: "Argon2id",
            version: 19,
            memory_kib: 65536,
            time_cost: 3,
            parallelism: 4,
            output_len: 32,
            salt: &[4; 16],
        },
    };
    for (role, length, expected) in [
        (
            Role::Kem,
            391,
            "c5ef77df93fe9fe2f1ae03e98f1a2fe89f6de6d6e258d1d8fce9ddd7eadaf4d4",
        ),
        (
            Role::Signature,
            397,
            "6767117d497f73ae84b680522b50a53c615875e3f826dd919916800799464116",
        ),
    ] {
        let bytes = schema::aad(&h, role).unwrap();
        assert_eq!(bytes.len(), length);
        let digest = crate::foundation::crypto::sha256(&bytes);
        let hex: String = digest.iter().map(|b| format!("{b:02x}")).collect();
        assert_eq!(hex, expected);
    }
    let f = Fixture::new();
    let e = f.bundle();
    let b = schema::parse(e.as_bytes()).unwrap();
    let key = kdf::derive(
        f.pw.borrow(),
        b.header.kdf.salt,
        b.header.validate().unwrap(),
    )
    .unwrap();
    assert!(aead::open(&key, b.kem, &b.header, Role::Kem).is_ok());
    assert!(matches!(
        aead::open(&key, b.kem, &b.header, Role::Signature),
        Err(Error::AuthenticationFailure)
    ));
}
#[test]
fn p2b_aad_002_complete_header_authenticated() {
    let f = Fixture::new();
    let encrypted = f.bundle();
    for name in ["recipient_id", "kem_key_id", "signature_key_id"] {
        let r = span(encrypted.as_bytes(), &["header", name]);
        let mut d = minicbor::Decoder::new(&encrypted.as_bytes()[r]);
        let mut bytes = d.bytes().unwrap().to_vec();
        bytes[0] ^= 1;
        f.reject_auth(&replace(
            encrypted.as_bytes(),
            &["header", name],
            &encode_bytes(&bytes),
        ));
    }
    for (name, value) in [("memory_kib", 65537), ("time_cost", 4), ("parallelism", 1)] {
        f.reject_auth(&replace(
            encrypted.as_bytes(),
            &["header", "kdf", name],
            &encode_uint(value),
        ));
    }
    let mut salt = schema::parse(encrypted.as_bytes())
        .unwrap()
        .header
        .kdf
        .salt
        .to_vec();
    salt[0] ^= 1;
    f.reject_auth(&replace(
        encrypted.as_bytes(),
        &["header", "kdf", "salt"],
        &encode_bytes(&salt),
    ));
}
#[test]
fn p2b_aad_003_entry_nonce_and_id_swaps() {
    let f = Fixture::new();
    let encrypted = f.bundle();
    let b = schema::parse(encrypted.as_bytes()).unwrap();
    clear_drops();
    let before = KDFS.get();
    let swapped = Bundle {
        header: b.header,
        kem: b.signature,
        signature: b.kem,
    };
    f.reject_auth(&output(&swapped));
    assert_eq!(KDFS.get(), before + 1);
    assert!(drop_ok("kem-plaintext", 4032));
    let ciphertext_swap = Bundle {
        header: b.header,
        kem: Protected {
            nonce: b.kem.nonce,
            ciphertext_and_tag: b.signature.ciphertext_and_tag,
        },
        signature: Protected {
            nonce: b.signature.nonce,
            ciphertext_and_tag: b.kem.ciphertext_and_tag,
        },
    };
    f.reject_auth(&output(&ciphertext_swap));
    let swapped = Bundle {
        header: b.header,
        kem: Protected {
            nonce: b.signature.nonce,
            ..b.kem
        },
        signature: Protected {
            nonce: b.kem.nonce,
            ..b.signature
        },
    };
    f.reject_auth(&output(&swapped));
    let swapped = Bundle {
        header: Header {
            kem_key_id: b.header.signature_key_id,
            signature_key_id: b.header.kem_key_id,
            ..b.header
        },
        ..b
    };
    f.reject_auth(&output(&swapped));
}
#[test]
fn p2b_bundle_002_unknown_duplicate_at_every_level() {
    let f = Fixture::new();
    let encrypted = f.bundle();
    let before = KDFS.get();
    for path in [
        &[][..],
        &["header"][..],
        &["header", "kdf"][..],
        &["kem"][..],
        &["signature"][..],
    ] {
        for duplicate in [false, true] {
            let altered = extra_field(encrypted.as_bytes(), path, duplicate);
            assert!(matches!(
                unlock_bundle(&altered, &f.pw, &f.identity()),
                Err(Error::InvalidBundle)
            ));
        }
    }
    assert_eq!(KDFS.get(), before);
}
#[test]
fn p2b_bundle_003_noncanonical_forbidden_truncated_bounds() {
    let f = Fixture::new();
    let encrypted = f.bundle();
    let b = encrypted.as_bytes();
    let before = KDFS.get();
    let mut variants = vec![
        replace(b, &["header", "schema_version"], &[0x18, 1]),
        replace(b, &["header", "schema_version"], &[0xc0, 1]),
        replace(b, &["header", "schema_version"], &[0xf9, 0, 0]),
        vec![0; 32769],
    ];
    let mut wide = vec![0xb8, 3];
    wide.extend_from_slice(&b[1..]);
    variants.push(wide);
    let mut indef = b.to_vec();
    indef[0] = 0xbf;
    indef.push(0xff);
    variants.push(indef);
    let mut trailing = b.to_vec();
    trailing.push(0);
    variants.push(trailing);
    let mut d = minicbor::Decoder::new(b);
    d.map().unwrap();
    let first = d.position();
    d.str().unwrap();
    d.skip().unwrap();
    let second = d.position();
    d.str().unwrap();
    d.skip().unwrap();
    let end = d.position();
    let mut reordered = b[..first].to_vec();
    reordered.extend_from_slice(&b[second..end]);
    reordered.extend_from_slice(&b[first..second]);
    reordered.extend_from_slice(&b[end..]);
    variants.push(reordered);
    for n in (0..b.len()).step_by(97).chain(std::iter::once(b.len() - 1)) {
        variants.push(b[..n].to_vec());
    }
    for bytes in variants {
        assert!(unlock_bundle(&bytes, &f.pw, &f.identity()).is_err());
    }
    assert_eq!(KDFS.get(), before);
}
#[test]
fn p2b_bundle_004_algorithms_kdf_versions_and_costs_before_allocation() {
    let f = Fixture::new();
    let encrypted = f.bundle();
    let before = KDFS.get();
    let allocations = WORKSPACES.get();
    for field in ["kem_algorithm", "signature_algorithm", "aead_algorithm"] {
        assert_eq!(
            unlock_bundle(
                &replace(
                    encrypted.as_bytes(),
                    &["header", field],
                    &encode_text("INVALID")
                ),
                &f.pw,
                &f.identity()
            )
            .unwrap_err(),
            Error::UnsupportedAlgorithm
        );
    }
    assert_eq!(
        unlock_bundle(
            &replace(
                encrypted.as_bytes(),
                &["header", "kdf", "algorithm"],
                &encode_text("Argon2i")
            ),
            &f.pw,
            &f.identity()
        )
        .unwrap_err(),
        Error::UnsupportedAlgorithm
    );
    for version in [0, 2, u64::MAX] {
        assert_eq!(
            unlock_bundle(
                &replace(
                    encrypted.as_bytes(),
                    &["header", "schema_version"],
                    &encode_uint(version)
                ),
                &f.pw,
                &f.identity()
            )
            .unwrap_err(),
            Error::UnsupportedVersion
        );
    }
    for (field, values) in [
        ("version", vec![0, 16, 20, u64::MAX]),
        ("memory_kib", vec![0, 65535, 262145, u64::MAX]),
        ("time_cost", vec![0, 2, 7, u64::MAX]),
        ("parallelism", vec![0, 5, u64::MAX]),
        ("output_len", vec![0, 31, 33, u64::MAX]),
    ] {
        for v in values {
            assert_eq!(
                unlock_bundle(
                    &replace(
                        encrypted.as_bytes(),
                        &["header", "kdf", field],
                        &encode_uint(v)
                    ),
                    &f.pw,
                    &f.identity()
                )
                .unwrap_err(),
                Error::InvalidKdfParameters
            );
        }
    }
    for n in [0, 15, 17] {
        assert_eq!(
            unlock_bundle(
                &replace(
                    encrypted.as_bytes(),
                    &["header", "kdf", "salt"],
                    &encode_bytes(&vec![0; n])
                ),
                &f.pw,
                &f.identity()
            )
            .unwrap_err(),
            Error::InvalidKdfParameters
        );
    }
    assert_eq!(KDFS.get(), before);
    assert_eq!(WORKSPACES.get(), allocations);
}
#[test]
fn p2b_bundle_005_nonce_and_ciphertext_lengths() {
    let f = Fixture::new();
    let encrypted = f.bundle();
    let before = KDFS.get();
    for (role, size) in [("kem", 2416), ("signature", 4048)] {
        for n in [0, 11, 13] {
            assert!(
                unlock_bundle(
                    &replace(
                        encrypted.as_bytes(),
                        &[role, "nonce"],
                        &encode_bytes(&vec![0; n])
                    ),
                    &f.pw,
                    &f.identity()
                )
                .is_err()
            );
        }
        for n in [0, 15, size - 1, size + 1] {
            assert!(
                unlock_bundle(
                    &replace(
                        encrypted.as_bytes(),
                        &[role, "ciphertext_and_tag"],
                        &encode_bytes(&vec![0; n])
                    ),
                    &f.pw,
                    &f.identity()
                )
                .is_err()
            );
        }
    }
    let b = schema::parse(encrypted.as_bytes()).unwrap();
    let equal = Bundle {
        signature: Protected {
            nonce: b.kem.nonce,
            ..b.signature
        },
        ..b
    };
    assert!(matches!(
        unlock_bundle(&output(&equal), &f.pw, &f.identity()),
        Err(Error::InvalidBundle)
    ));
    assert_eq!(KDFS.get(), before);
}
#[test]
fn p2b_bundle_006_deterministic_malformed_properties() {
    let f = Fixture::new();
    let encrypted = f.bundle();
    let before = KDFS.get();
    for seed in 0..256 {
        let mut bytes = encrypted.as_bytes().to_vec();
        let n = seed * 19 % bytes.len();
        bytes[n] ^= 0x80;
        let result = std::panic::catch_unwind(|| schema::parse(&bytes));
        assert!(result.is_ok());
        if let Ok(value) = result.unwrap() {
            assert_eq!(output(&value), bytes);
        }
    }
    assert_eq!(KDFS.get(), before);
}
#[test]
fn p2b_rng_001_fresh_salt_and_independent_nonces() {
    let f = Fixture::new();
    let first = f.bundle();
    let second = f.bundle();
    let a = schema::parse(first.as_bytes()).unwrap();
    let b = schema::parse(second.as_bytes()).unwrap();
    assert_ne!(a.header.kdf.salt, b.header.kdf.salt);
    assert_ne!(a.kem.nonce, a.signature.nonce);
    assert_ne!(a.kem.nonce, b.kem.nonce);
    assert_ne!(a.signature.nonce, b.signature.nonce);
}
#[test]
fn p2b_rng_002_failure_and_collision_no_kdf_or_fallback() {
    let f = Fixture::new();
    let before = KDFS.get();
    for fail in 1..=3 {
        let mut calls = 0;
        let result = create_with_rng(
            &f.pw,
            &f.identity(),
            &f.sk,
            &f.ss,
            KdfParameters::new(65536, 3, 4).unwrap(),
            &mut |b| {
                calls += 1;
                b.fill(calls);
                if calls == fail {
                    Err(Error::RandomnessFailure)
                } else {
                    Ok(())
                }
            },
        );
        assert!(matches!(result, Err(Error::RandomnessFailure)));
        assert_eq!(calls, fail);
    }
    let result = create_with_rng(
        &f.pw,
        &f.identity(),
        &f.sk,
        &f.ss,
        KdfParameters::new(65536, 3, 4).unwrap(),
        &mut |b| {
            b.fill(0);
            Ok(())
        },
    );
    assert!(matches!(result, Err(Error::RandomnessFailure)));
    assert_eq!(KDFS.get(), before);
}
#[test]
fn p2b_secret_001_redaction_password_and_kek_drop() {
    clear_drops();
    let password = Password::fixture("PUBLIC SECRET OWNER FIXTURE");
    assert!(format!("{password:?}") == "Password([REDACTED])");
    drop(password);
    assert!(drop_ok("password", 27));
    let mut key = secret::SecretBytes::zeroed(32, "kek").unwrap();
    key.borrow_mut().fill(0x5a);
    assert!(format!("{key:?}") == "SecretBytes([REDACTED])");
    drop(key);
    assert!(drop_ok("kek", 32));
    assert!(matches!(
        Password::take_string_for_test(String::new()),
        Err(Error::InvalidPassword)
    ));
    assert!(matches!(
        Password::take_string_for_test("x".repeat(4097)),
        Err(Error::InvalidPassword)
    ));
    assert!(Password::take_string_for_test("x".repeat(4096)).is_ok());
}
#[test]
fn p2b_secret_002_first_decrypt_then_second_failure() {
    let f = Fixture::new();
    let encrypted = f.bundle();
    let mut b = schema::parse(encrypted.as_bytes()).unwrap();
    let mut ct = b.signature.ciphertext_and_tag.to_vec();
    let n = ct.len() - 1;
    ct[n] ^= 1;
    b.signature.ciphertext_and_tag = &ct;
    clear_drops();
    f.reject_auth(&output(&b));
    assert!(drop_ok("kem-plaintext", 2400));
    assert!(drop_ok("signature-plaintext", 4032));
    assert!(drop_ok("kek", 32));
}
#[test]
fn p2b_secret_003_authenticated_wrong_recovered_length() {
    let f = Fixture::new();
    let encrypted = f.bundle();
    let mut b = schema::parse(encrypted.as_bytes()).unwrap();
    let key = kdf::derive(
        f.pw.borrow(),
        b.header.kdf.salt,
        b.header.validate().unwrap(),
    )
    .unwrap();
    let nonce: [u8; 12] = b.kem.nonce.try_into().unwrap();
    let malformed = aead::protect(&key, &nonce, &b.header, Role::Kem, &[0x5a; 4032]).unwrap();
    b.kem.ciphertext_and_tag = &malformed;
    clear_drops();
    f.reject_auth(&output(&b));
    assert!(drop_ok("kem-plaintext", 4032));
    assert!(drop_ok("kek", 32));
    let mut signature_bad = schema::parse(encrypted.as_bytes()).unwrap();
    let sn: [u8; 12] = signature_bad.signature.nonce.try_into().unwrap();
    let wrong = aead::protect(
        &key,
        &sn,
        &signature_bad.header,
        Role::Signature,
        &[0x5a; 2400],
    )
    .unwrap();
    signature_bad.signature.ciphertext_and_tag = &wrong;
    clear_drops();
    f.reject_auth(&output(&signature_bad));
    assert!(drop_ok("kem-plaintext", 2400));
    assert!(drop_ok("signature-plaintext", 2400));
}
#[test]
fn p2b_secret_004_both_plaintexts_then_binding_failure() {
    let f = Fixture::new();
    let encrypted = f.bundle();
    let other = Identity::new([0x77; 32], &f.pk, &f.sp).unwrap();
    clear_drops();
    assert!(matches!(
        unlock_bundle(encrypted.as_bytes(), &f.pw, &other),
        Err(Error::AuthenticationFailure)
    ));
    assert!(drop_ok("kem-plaintext", 2400));
    assert!(drop_ok("signature-plaintext", 4032));
    assert!(drop_ok("kek", 32));
}
#[test]
fn p2b_secret_005_serialization_error_cleans_kek() {
    let f = Fixture::new();
    clear_drops();
    SERIAL_FAIL.set(true);
    assert!(matches!(
        create_with_rng(
            &f.pw,
            &f.identity(),
            &f.sk,
            &f.ss,
            KdfParameters::new(65536, 3, 4).unwrap(),
            &mut random
        ),
        Err(Error::SerializationFailure)
    ));
    assert!(drop_ok("kek", 32));
}

struct Directory(PathBuf);
impl Directory {
    fn new() -> Self {
        let id = RandomId::generate().unwrap();
        let suffix: String = id.as_bytes().iter().map(|b| format!("{b:02x}")).collect();
        let path = std::env::temp_dir().join(format!("fp1-p2b-unit-{suffix}"));
        fs::DirBuilder::new().mode(0o700).create(&path).unwrap();
        Self(path)
    }
    fn file(&self) -> PathBuf {
        self.0.join("keys.cbor")
    }
    fn count(&self) -> usize {
        fs::read_dir(&self.0).unwrap().count()
    }
}
impl Drop for Directory {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}
#[test]
fn p2b_fs_001_atomic_create_permissions_ciphertext_only() {
    let f = Fixture::new();
    let encrypted = f.bundle();
    let dir = Directory::new();
    persist(&dir.file(), &encrypted, Persistence::CreateNew).unwrap();
    assert_eq!(
        fs::metadata(dir.file()).unwrap().permissions().mode() & 0o777,
        0o600
    );
    assert!(fs::read(dir.file()).unwrap() == encrypted.as_bytes());
    assert_eq!(dir.count(), 1);
    let keys = unlock_file(&dir.file(), &f.pw, &f.identity()).unwrap();
    let proof = signature::sign(&keys.signature, b"PUBLIC FILE ROUNDTRIP").unwrap();
    assert!(signature::verify(&f.sp, b"PUBLIC FILE ROUNDTRIP", &proof).unwrap());
    let disk = fs::read(dir.file()).unwrap();
    f.sk.with_storage_bytes(|secret| assert!(!disk.windows(secret.len()).any(|s| s == secret)));
    f.ss.with_storage_bytes(|secret| assert!(!disk.windows(secret.len()).any(|s| s == secret)));
    assert_eq!(
        persist(&dir.file(), &encrypted, Persistence::CreateNew),
        Err(Error::AlreadyExists)
    );
}
#[test]
fn p2b_fs_002_partial_write_cleanup_and_old_file_survives() {
    let f = Fixture::new();
    let first = f.bundle();
    let second = f.bundle();
    let dir = Directory::new();
    WRITE_FAIL.set(true);
    assert_eq!(
        persist(&dir.file(), &first, Persistence::CreateNew),
        Err(Error::IoFailure)
    );
    assert_eq!(dir.count(), 0);
    persist(&dir.file(), &first, Persistence::CreateNew).unwrap();
    WRITE_FAIL.set(true);
    assert_eq!(
        persist(&dir.file(), &second, Persistence::Replace),
        Err(Error::IoFailure)
    );
    assert_eq!(dir.count(), 1);
    assert!(fs::read(dir.file()).unwrap() == first.as_bytes());
}
#[test]
fn p2b_fs_003_symlink_and_parent_traversal_reject() {
    use std::os::unix::fs::symlink;
    let f = Fixture::new();
    let encrypted = f.bundle();
    let dir = Directory::new();
    let target = dir.0.join("original");
    fs::write(&target, b"PUBLIC MARKER").unwrap();
    symlink(&target, dir.file()).unwrap();
    assert_eq!(
        persist(&dir.file(), &encrypted, Persistence::CreateNew),
        Err(Error::UnsafePath)
    );
    assert!(matches!(
        unlock_file(&dir.file(), &f.pw, &f.identity()),
        Err(Error::UnsafePath)
    ));
    assert_eq!(fs::read(&target).unwrap(), b"PUBLIC MARKER");
    let link = dir.0.join("parent-link");
    symlink(&dir.0, &link).unwrap();
    assert_eq!(
        persist(&link.join("keys.cbor"), &encrypted, Persistence::CreateNew),
        Err(Error::UnsafePath)
    );
    assert_eq!(
        persist(
            &dir.0.join("..").join("unexpected.cbor"),
            &encrypted,
            Persistence::CreateNew
        ),
        Err(Error::UnsafePath)
    );
}
#[test]
fn p2b_fs_004_insecure_modes_reject() {
    let f = Fixture::new();
    let encrypted = f.bundle();
    let dir = Directory::new();
    fs::set_permissions(&dir.0, fs::Permissions::from_mode(0o750)).unwrap();
    assert_eq!(
        persist(&dir.file(), &encrypted, Persistence::CreateNew),
        Err(Error::UnsafePath)
    );
    fs::set_permissions(&dir.0, fs::Permissions::from_mode(0o700)).unwrap();
    persist(&dir.file(), &encrypted, Persistence::CreateNew).unwrap();
    fs::set_permissions(dir.file(), fs::Permissions::from_mode(0o640)).unwrap();
    assert!(matches!(
        unlock_file(&dir.file(), &f.pw, &f.identity()),
        Err(Error::UnsafePath)
    ));
    assert_eq!(
        persist(&dir.file(), &encrypted, Persistence::Replace),
        Err(Error::UnsafePath)
    );
}
#[test]
fn p2b_fs_005_fresh_replacement_and_reuse_rejection() {
    let f = Fixture::new();
    let first = f.bundle();
    let second = f.bundle();
    let dir = Directory::new();
    persist(&dir.file(), &first, Persistence::CreateNew).unwrap();
    assert_eq!(
        persist(&dir.file(), &first, Persistence::Replace),
        Err(Error::InvalidBundle)
    );
    persist(&dir.file(), &second, Persistence::Replace).unwrap();
    assert_eq!(dir.count(), 1);
    assert!(fs::read(dir.file()).unwrap() == second.as_bytes());
    assert!(unlock_file(&dir.file(), &f.pw, &f.identity()).is_ok());
    let current = schema::parse(second.as_bytes()).unwrap();
    let third = f.bundle();
    for old_nonce in [current.kem.nonce, current.signature.nonce] {
        let mut new = schema::parse(third.as_bytes()).unwrap();
        new.kem.nonce = old_nonce;
        let reused = EncryptedBundle {
            bytes: output(&new),
        };
        assert_eq!(
            persist(&dir.file(), &reused, Persistence::Replace),
            Err(Error::InvalidBundle)
        );
    }
    let mut salt_reuse = schema::parse(third.as_bytes()).unwrap();
    salt_reuse.header.kdf.salt = current.header.kdf.salt;
    assert_eq!(
        persist(
            &dir.file(),
            &EncryptedBundle {
                bytes: output(&salt_reuse)
            },
            Persistence::Replace
        ),
        Err(Error::InvalidBundle)
    );
    let changed = Password::fixture("PUBLIC NEW TEST PASSWORD");
    let changed_bundle = create_with_rng(
        &changed,
        &f.identity(),
        &f.sk,
        &f.ss,
        KdfParameters::new(65536, 3, 4).unwrap(),
        &mut random,
    )
    .unwrap();
    persist(&dir.file(), &changed_bundle, Persistence::Replace).unwrap();
    assert!(unlock_file(&dir.file(), &changed, &f.identity()).is_ok());
    assert!(matches!(
        unlock_file(&dir.file(), &f.pw, &f.identity()),
        Err(Error::AuthenticationFailure)
    ));
}
#[test]
fn p2b_fs_006_malformed_existing_and_bounded_read() {
    let f = Fixture::new();
    let encrypted = f.bundle();
    let dir = Directory::new();
    for bytes in [
        vec![],
        vec![0],
        vec![0; 32769],
        encrypted.as_bytes()[..32].to_vec(),
    ] {
        fs::write(dir.file(), bytes).unwrap();
        fs::set_permissions(dir.file(), fs::Permissions::from_mode(0o600)).unwrap();
        assert!(unlock_file(&dir.file(), &f.pw, &f.identity()).is_err());
        assert!(persist(&dir.file(), &encrypted, Persistence::Replace).is_err());
        assert_eq!(dir.count(), 1);
    }
}

#[test]
fn p2b_fs_007_concurrent_no_clobber_publication() {
    let f = Fixture::new();
    let first = f.bundle();
    let second = f.bundle();
    let dir = Directory::new();
    let path = dir.file();
    let results = std::thread::scope(|scope| {
        let a = scope.spawn(|| persist(&path, &first, Persistence::CreateNew));
        let b = scope.spawn(|| persist(&path, &second, Persistence::CreateNew));
        [a.join().unwrap(), b.join().unwrap()]
    });
    assert_eq!(results.iter().filter(|r| r.is_ok()).count(), 1);
    assert_eq!(
        results
            .iter()
            .filter(|r| **r == Err(Error::AlreadyExists))
            .count(),
        1
    );
    assert_eq!(dir.count(), 1);
    assert!(unlock_file(&path, &f.pw, &f.identity()).is_ok());
    let fresh = f.bundle();
    let barrier = std::sync::Barrier::new(2);
    let results = std::thread::scope(|scope| {
        let a = scope.spawn(|| {
            barrier.wait();
            persist(&path, &fresh, Persistence::Replace)
        });
        let b = scope.spawn(|| {
            barrier.wait();
            persist(&path, &fresh, Persistence::Replace)
        });
        [a.join().unwrap(), b.join().unwrap()]
    });
    assert_eq!(results.iter().filter(|r| r.is_ok()).count(), 1);
    assert_eq!(
        results
            .iter()
            .filter(|r| **r == Err(Error::InvalidBundle))
            .count(),
        1
    );
    assert_eq!(dir.count(), 1);
}
