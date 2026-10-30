//! Stable Phase 1 test IDs. Ephemeral keys never leave process memory.

use super::{Error, backend_info, ffi, kem, secret::SecretBuffer, signature};
use std::sync::atomic::Ordering;

#[test]
fn pqc_kem_001_valid_roundtrip() -> Result<(), Error> {
    let (pk, sk) = kem::generate_keypair()?;
    let (ct, original) = kem::encapsulate(&pk)?;
    let recovered = kem::decapsulate(&sk, &ct)?;
    assert_eq!(original, recovered);
    Ok(())
}

#[test]
fn pqc_kem_002_wrong_key_implicit_rejection() -> Result<(), Error> {
    let (pk_a, _sk_a) = kem::generate_keypair()?;
    let (_pk_b, sk_b) = kem::generate_keypair()?;
    let (ct, original) = kem::encapsulate(&pk_a)?;
    let wrong = kem::decapsulate(&sk_b, &ct)?;
    assert_ne!(original, wrong);
    Ok(())
}

#[test]
fn pqc_kem_003_modified_ciphertext_implicit_rejection() -> Result<(), Error> {
    let (pk, sk) = kem::generate_keypair()?;
    let (mut ct, original) = kem::encapsulate(&pk)?;
    ct[0] ^= 1;
    let altered = kem::decapsulate(&sk, &ct)?;
    assert_ne!(original, altered);
    Ok(())
}

#[test]
fn pqc_kem_004_truncated_before_ffi() -> Result<(), Error> {
    let (pk, sk) = kem::generate_keypair()?;
    let (ct, _shared) = kem::encapsulate(&pk)?;
    let before = ffi::test_support::operations();
    assert!(matches!(
        kem::decapsulate(&sk, &ct[..ct.len() - 1]),
        Err(Error::IncorrectInputLength {
            input: "KEM ciphertext",
            ..
        })
    ));
    assert_eq!(before, ffi::test_support::operations());
    Ok(())
}

#[test]
fn pqc_kem_005_noncanonical_public_key_failure_cleanup() {
    let (before, _) = ffi::test_support::cleanses();
    // Fixed-size but noncanonical ML-KEM encoding: the maintained backend,
    // rather than hand-written validation, must reject encapsulation.
    assert!(matches!(
        kem::encapsulate(&[0xFFu8; kem::PUBLIC_KEY_LENGTH]),
        Err(Error::EncapsulationFailure)
    ));
    let (after, all_zero) = ffi::test_support::cleanses();
    assert_eq!(after - before, 1);
    assert!(all_zero);
}

#[test]
fn pqc_sig_001_valid_signature() -> Result<(), Error> {
    let (pk, sk) = signature::generate_keypair()?;
    let signed = signature::sign(&sk, b"Phase 1 public test message")?;
    assert_eq!(signed.len(), signature::MAX_SIGNATURE_LENGTH);
    assert!(signature::verify(
        &pk,
        b"Phase 1 public test message",
        &signed
    )?);
    Ok(())
}

#[test]
fn pqc_sig_002_wrong_public_key() -> Result<(), Error> {
    let (_pk_a, sk_a) = signature::generate_keypair()?;
    let (pk_b, _sk_b) = signature::generate_keypair()?;
    let signed = signature::sign(&sk_a, b"test")?;
    assert!(!signature::verify(&pk_b, b"test", &signed)?);
    Ok(())
}

#[test]
fn pqc_sig_003_modified_message() -> Result<(), Error> {
    let (pk, sk) = signature::generate_keypair()?;
    let signed = signature::sign(&sk, b"test")?;
    assert!(!signature::verify(&pk, b"tess", &signed)?);
    Ok(())
}

#[test]
fn pqc_sig_004_modified_signature() -> Result<(), Error> {
    let (pk, sk) = signature::generate_keypair()?;
    let mut signed = signature::sign(&sk, b"test")?;
    signed[0] ^= 1;
    assert!(!signature::verify(&pk, b"test", &signed)?);
    Ok(())
}

#[test]
fn pqc_sig_005_truncated_signature() -> Result<(), Error> {
    let (pk, sk) = signature::generate_keypair()?;
    let mut signed = signature::sign(&sk, b"test")?;
    signed.pop();
    let before = ffi::test_support::operations();
    assert!(matches!(
        signature::verify(&pk, b"test", &signed),
        Err(Error::MalformedSignature { .. })
    ));
    assert_eq!(before, ffi::test_support::operations());
    Ok(())
}

#[test]
fn pqc_backend_001_identity_and_algorithms() -> Result<(), Error> {
    let backend = backend_info()?;
    assert_eq!(backend.version, "0.16.0");
    assert_eq!(
        backend.source_commit,
        "5a1a854b0dc9f2141bdc771c555ee60c37950183"
    );
    assert_eq!(backend.kem_algorithm, "ML-KEM-768");
    assert_eq!(backend.signature_algorithm, "ML-DSA-65");
    assert!(backend.empty_context_supported);
    // Native initialization also enumerates enabled algorithms, requiring exactly
    // the two named parameter sets; metadata is not guessed from Cargo features.
    Ok(())
}

#[test]
fn pqc_size_001_backend_contract() -> Result<(), Error> {
    assert_eq!(
        backend_info()?.sizes,
        [1184, 2400, 1088, 32, 1952, 4032, 3309]
    );
    Ok(())
}

#[test]
fn pqc_boundary_001_kem_public_inputs() -> Result<(), Error> {
    let (_pk, sk) = kem::generate_keypair()?;
    let before = ffi::test_support::operations();
    for length in [0, kem::PUBLIC_KEY_LENGTH - 1, kem::PUBLIC_KEY_LENGTH + 1] {
        assert!(matches!(
            kem::encapsulate(&vec![0u8; length]),
            Err(Error::IncorrectInputLength {
                input: "KEM public key",
                ..
            })
        ));
    }
    for length in [0, kem::CIPHERTEXT_LENGTH - 1, kem::CIPHERTEXT_LENGTH + 1] {
        assert!(matches!(
            kem::decapsulate(&sk, &vec![0u8; length]),
            Err(Error::IncorrectInputLength {
                input: "KEM ciphertext",
                ..
            })
        ));
    }
    assert_eq!(before, ffi::test_support::operations());
    Ok(())
}

#[test]
fn pqc_boundary_002_kem_secret_and_output_lengths() {
    let before = ffi::test_support::operations();
    for length in [0, kem::SECRET_KEY_LENGTH - 1, kem::SECRET_KEY_LENGTH + 1] {
        let key = vec![0u8; length];
        let mut shared = [0u8; kem::SHARED_SECRET_LENGTH];
        assert!(matches!(
            ffi::decapsulate(&key, &[0u8; kem::CIPHERTEXT_LENGTH], &mut shared),
            Err(Error::IncorrectInputLength {
                input: "KEM secret key",
                ..
            })
        ));
    }
    assert!(matches!(
        ffi::encapsulate(
            &[0u8; kem::PUBLIC_KEY_LENGTH],
            &mut [0u8; kem::CIPHERTEXT_LENGTH],
            &mut []
        ),
        Err(Error::IncorrectInputLength {
            input: "KEM shared secret",
            ..
        })
    ));
    assert_eq!(before, ffi::test_support::operations());
}

#[test]
fn pqc_boundary_003_signature_public_inputs() -> Result<(), Error> {
    let (pk, sk) = signature::generate_keypair()?;
    let signed = signature::sign(&sk, b"test")?;
    let before = ffi::test_support::operations();
    for length in [
        0,
        signature::PUBLIC_KEY_LENGTH - 1,
        signature::PUBLIC_KEY_LENGTH + 1,
    ] {
        assert!(matches!(
            signature::verify(&vec![0u8; length], b"test", &signed),
            Err(Error::IncorrectInputLength {
                input: "signature public key",
                ..
            })
        ));
    }
    for length in [
        0,
        1,
        signature::MAX_SIGNATURE_LENGTH - 1,
        signature::MAX_SIGNATURE_LENGTH + 1,
    ] {
        assert!(matches!(
            signature::verify(&pk, b"test", &vec![0u8; length]),
            Err(Error::MalformedSignature { .. })
        ));
    }
    assert_eq!(before, ffi::test_support::operations());
    Ok(())
}

#[test]
fn pqc_boundary_004_signature_secret_and_capacity_lengths() {
    let before = ffi::test_support::operations();
    for length in [
        0,
        signature::SECRET_KEY_LENGTH - 1,
        signature::SECRET_KEY_LENGTH + 1,
    ] {
        assert!(matches!(
            ffi::sign(
                &vec![0u8; length],
                b"test",
                &mut [0u8; signature::MAX_SIGNATURE_LENGTH]
            ),
            Err(Error::IncorrectInputLength {
                input: "signature secret key",
                ..
            })
        ));
    }
    assert!(matches!(
        ffi::sign(&[0u8; signature::SECRET_KEY_LENGTH], b"test", &mut []),
        Err(Error::IncorrectInputLength {
            input: "signature capacity",
            ..
        })
    ));
    assert_eq!(before, ffi::test_support::operations());
}

#[test]
fn pqc_msg_001_empty_and_large_borrowed_messages() -> Result<(), Error> {
    let (pk, sk) = signature::generate_keypair()?;
    for message in [Vec::new(), vec![0xA5u8; 1024 * 1024]] {
        let signed = signature::sign(&sk, &message)?;
        assert!(signature::verify(&pk, &message, &signed)?);
    }
    Ok(())
}

#[test]
fn pqc_ctx_001_ordinary_and_empty_context_interoperate() -> Result<(), Error> {
    let (pk, sk) = signature::generate_keypair()?;
    for message in [b"".as_slice(), b"pure ML-DSA interoperability\0public test"] {
        let ordinary = signature::sign(&sk, message)?;
        assert!(ffi::verify_empty_context(&pk, message, &ordinary)?);
        let mut contextual = [0u8; signature::MAX_SIGNATURE_LENGTH];
        let written = ffi::sign_empty_context(sk.0.as_slice(), message, &mut contextual)?;
        assert_eq!(written, signature::MAX_SIGNATURE_LENGTH);
        assert!(signature::verify(&pk, message, &contextual[..written])?);
        assert!(ffi::verify_empty_context(
            &pk,
            message,
            &contextual[..written]
        )?);
        // Randomized signatures need not match byte-for-byte.
    }
    Ok(())
}

#[test]
fn pqc_secret_001_redacted_debug() -> Result<(), Error> {
    let (_pk, kem_key) = kem::generate_keypair()?;
    let (pk, sig_key) = signature::generate_keypair()?;
    let (_ct, shared) = kem::encapsulate(&kem::generate_keypair()?.0)?;
    assert_eq!(format!("{kem_key:?}"), "KemSecretKey([REDACTED])");
    assert_eq!(format!("{sig_key:?}"), "SignatureSecretKey([REDACTED])");
    assert_eq!(format!("{shared:?}"), "SharedSecret([REDACTED])");
    assert_eq!(pk.len(), signature::PUBLIC_KEY_LENGTH);
    Ok(())
}

#[test]
fn pqc_secret_002_drop_cleanses_live_allocation() -> Result<(), Error> {
    let (before, _) = ffi::test_support::cleanses();
    for length in [
        kem::SECRET_KEY_LENGTH,
        signature::SECRET_KEY_LENGTH,
        kem::SHARED_SECRET_LENGTH,
    ] {
        let mut buffer = SecretBuffer::zeroed(length)?;
        buffer.as_mut_slice().fill(0xAB);
        drop(buffer);
    }
    let (after, all_zero) = ffi::test_support::cleanses();
    assert_eq!(after - before, 3);
    // Test-only observation occurs while the allocation is still alive, after
    // native cleanse and before Vec deallocation; never read freed memory.
    assert!(all_zero);
    Ok(())
}

#[test]
fn pqc_alloc_001_capacity_failure_is_typed() {
    assert!(matches!(
        SecretBuffer::zeroed(usize::MAX),
        Err(Error::AllocationFailure)
    ));
}

#[test]
fn pqc_life_001_process_initialization_is_once() -> Result<(), Error> {
    backend_info()?;
    let stops_before = ffi::test_support::stops();
    for _ in 0..8 {
        backend_info()?;
        let (pk, sk) = kem::generate_keypair()?;
        let (ct, shared) = kem::encapsulate(&pk)?;
        assert_eq!(shared, kem::decapsulate(&sk, &ct)?);
    }
    assert_eq!(ffi::test_support::INITIALIZATIONS.load(Ordering::SeqCst), 1);
    assert_eq!(ffi::test_support::stops() - stops_before, 8 * 3);
    Ok(())
}

#[test]
fn pqc_conc_001_concurrent_smoke() {
    let barrier = std::sync::Barrier::new(4);
    std::thread::scope(|scope| {
        let jobs: Vec<_> = (0..4)
            .map(|_| {
                scope.spawn(|| -> Result<(), Error> {
                    barrier.wait();
                    for _ in 0..3 {
                        let (pk, sk) = kem::generate_keypair()?;
                        let (ct, shared) = kem::encapsulate(&pk)?;
                        assert_eq!(shared, kem::decapsulate(&sk, &ct)?);
                        let (pk, sk) = signature::generate_keypair()?;
                        let signed = signature::sign(&sk, b"concurrency test")?;
                        assert!(signature::verify(&pk, b"concurrency test", &signed)?);
                    }
                    Ok(())
                })
            })
            .collect();
        for job in jobs {
            assert_eq!(job.join().expect("test worker must not panic"), Ok(()));
        }
    });
    assert_eq!(ffi::test_support::INITIALIZATIONS.load(Ordering::SeqCst), 1);
}

#[test]
fn p2b_secret_006_storage_adoption_uses_existing_cleanse() {
    let (before, _) = ffi::test_support::cleanses();
    for len in [0, 2399, 2401] {
        assert!(matches!(
            kem::SecretKey::from_storage(vec![0x5a; len]),
            Err(Error::IncorrectInputLength { .. })
        ));
    }
    for len in [0, 4031, 4033] {
        assert!(matches!(
            signature::SecretKey::from_storage(vec![0x5a; len]),
            Err(Error::IncorrectInputLength { .. })
        ));
    }
    drop(kem::SecretKey::from_storage(vec![0x5a; 2400]).unwrap());
    drop(signature::SecretKey::from_storage(vec![0x5a; 4032]).unwrap());
    let (after, clean) = ffi::test_support::cleanses();
    assert_eq!(after - before, 8);
    assert!(clean);
}
