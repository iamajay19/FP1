//! Isolated executable linked to the production rlib with test-only C wrapping.
//! The native fixture supplies synthetic keys; no real primitive keys are generated.
#![deny(unsafe_code)]

use cadp_pqc::pqc::{Error, kem, signature};

fn main() -> Result<(), Error> {
    match std::env::var("CADP_TEST_PARTIAL_OUTPUT").as_deref() {
        Ok("kem-keygen") => assert!(matches!(
            kem::generate_keypair(),
            Err(Error::KeyGenerationFailure {
                algorithm: "ML-KEM-768"
            })
        )),
        Ok("sig-keygen") => assert!(matches!(
            signature::generate_keypair(),
            Err(Error::KeyGenerationFailure {
                algorithm: "ML-DSA-65"
            })
        )),
        Ok("encaps") => {
            let (pk, _sk) = kem::generate_keypair()?;
            assert!(matches!(
                kem::encapsulate(&pk),
                Err(Error::EncapsulationFailure)
            ));
        }
        Ok("decaps") => {
            let (_pk, sk) = kem::generate_keypair()?;
            assert!(matches!(
                kem::decapsulate(&sk, &[0u8; kem::CIPHERTEXT_LENGTH]),
                Err(Error::DecapsulationFailure)
            ));
        }
        _ => panic!("isolated fixture requires one of its four public operation modes"),
    }
    // Failed output buffers belong to the wrapper, so their Drop has already
    // run before the typed error reaches this driver. No manual cleanse here.
    Ok(())
}
