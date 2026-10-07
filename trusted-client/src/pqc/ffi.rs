//! The only Rust unsafe boundary. C owns all OQS handles and frees them per call.
//! Safe entry points recheck lengths; borrowed buffers remain alive and disjoint.

use super::{BackendInfo, Error, error::check_length, kem, signature};
use std::ffi::c_int;
use std::sync::OnceLock;

unsafe extern "C" {
    fn cadp_initialize(sizes: *mut usize) -> c_int;
    fn cadp_thread_stop();
    fn cadp_cleanse(buffer: *mut u8, length: usize);
    fn cadp_secret_equal(a: *const u8, b: *const u8, length: usize) -> c_int;
    fn cadp_kem_keypair(pk: *mut u8, pk_len: usize, sk: *mut u8, sk_len: usize) -> c_int;
    fn cadp_kem_encaps(
        pk: *const u8,
        pk_len: usize,
        ct: *mut u8,
        ct_len: usize,
        ss: *mut u8,
        ss_len: usize,
    ) -> c_int;
    fn cadp_kem_decaps(
        sk: *const u8,
        sk_len: usize,
        ct: *const u8,
        ct_len: usize,
        ss: *mut u8,
        ss_len: usize,
    ) -> c_int;
    fn cadp_sig_keypair(pk: *mut u8, pk_len: usize, sk: *mut u8, sk_len: usize) -> c_int;
    fn cadp_sig_sign(
        sk: *const u8,
        sk_len: usize,
        message: *const u8,
        message_len: usize,
        signature: *mut u8,
        capacity: usize,
        written: *mut usize,
        empty_context_api: c_int,
    ) -> c_int;
    fn cadp_sig_verify(
        pk: *const u8,
        pk_len: usize,
        message: *const u8,
        message_len: usize,
        signature: *const u8,
        length: usize,
        empty_context_api: c_int,
    ) -> c_int;
}

static BACKEND: OnceLock<Result<BackendInfo, Error>> = OnceLock::new();

// Per-operation cleanup executes on this same thread after all native handles
// were freed in C. With OpenSSL OFF this is a no-op, but the contract is explicit.
struct ThreadCleanup;
impl Drop for ThreadCleanup {
    fn drop(&mut self) {
        // SAFETY: paired with initialized synchronous native calls on this thread.
        unsafe { cadp_thread_stop() };
        #[cfg(test)]
        test_support::record_stop();
    }
}

pub(super) fn backend_info() -> Result<&'static BackendInfo, Error> {
    BACKEND
        .get_or_init(|| {
            let _cleanup = ThreadCleanup;
            let mut sizes = [0usize; 7];
            #[cfg(test)]
            test_support::INITIALIZATIONS.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            // SAFETY: OnceLock serializes OQS_init and publishes initialized CPU state.
            // C writes exactly seven size_t entries into this exclusive usize array.
            let status = unsafe { cadp_initialize(sizes.as_mut_ptr()) };
            operation_status(status, Error::BackendUnavailable)?;
            if sizes != [1184, 2400, 1088, 32, 1952, 4032, 3309] {
                return Err(Error::UnexpectedBackendMetadata);
            }
            Ok(BackendInfo {
                version: "0.16.0",
                source_commit: env!("CADP_LIBOQS_COMMIT"),
                kem_algorithm: "ML-KEM-768",
                signature_algorithm: "ML-DSA-65",
                sizes,
                empty_context_supported: true,
            })
        })
        .as_ref()
        .map_err(|error| *error)
}

// Process-lifetime initialization is intentionally never paired with OQS_destroy.
// No global shutdown API: it could invalidate active concurrent callers. In this
// OpenSSL-free configuration destroy has no work; the OS reclaims process state.

fn operation_status(status: c_int, failure: Error) -> Result<(), Error> {
    match status {
        0 => Ok(()),
        1 => Err(Error::BackendUnavailable),
        2 | 6 => Err(Error::UnexpectedBackendMetadata),
        3 => Err(Error::AllocationFailure),
        4 => Err(failure),
        _ => Err(Error::UnexpectedBackendMetadata),
    }
}

fn begin_operation() -> Result<ThreadCleanup, Error> {
    backend_info()?;
    Ok(ThreadCleanup)
}

// Supply a real readable address for a zero-length message, without copying it.
fn message_pointer(message: &[u8]) -> *const u8 {
    static EMPTY: [u8; 1] = [0];
    if message.is_empty() {
        EMPTY.as_ptr()
    } else {
        message.as_ptr()
    }
}

pub(super) fn kem_keypair(pk: &mut [u8], sk: &mut [u8]) -> Result<(), Error> {
    check_length("KEM public key", pk.len(), kem::PUBLIC_KEY_LENGTH)?;
    check_length("KEM secret key", sk.len(), kem::SECRET_KEY_LENGTH)?;
    let _cleanup = begin_operation()?;
    #[cfg(test)]
    test_support::record_operation();
    // SAFETY: exclusive nonoverlapping outputs have the checked native lengths.
    let status = unsafe { cadp_kem_keypair(pk.as_mut_ptr(), pk.len(), sk.as_mut_ptr(), sk.len()) };
    operation_status(
        status,
        Error::KeyGenerationFailure {
            algorithm: "ML-KEM-768",
        },
    )
}

pub(super) fn encapsulate(pk: &[u8], ct: &mut [u8], ss: &mut [u8]) -> Result<(), Error> {
    check_length("KEM public key", pk.len(), kem::PUBLIC_KEY_LENGTH)?;
    check_length("KEM ciphertext", ct.len(), kem::CIPHERTEXT_LENGTH)?;
    check_length("KEM shared secret", ss.len(), kem::SHARED_SECRET_LENGTH)?;
    let _cleanup = begin_operation()?;
    #[cfg(test)]
    test_support::record_operation();
    // SAFETY: immutable input and exclusive outputs are disjoint, live and sized.
    let status = unsafe {
        cadp_kem_encaps(
            pk.as_ptr(),
            pk.len(),
            ct.as_mut_ptr(),
            ct.len(),
            ss.as_mut_ptr(),
            ss.len(),
        )
    };
    operation_status(status, Error::EncapsulationFailure)
}

pub(super) fn decapsulate(sk: &[u8], ct: &[u8], ss: &mut [u8]) -> Result<(), Error> {
    check_length("KEM secret key", sk.len(), kem::SECRET_KEY_LENGTH)?;
    check_length("KEM ciphertext", ct.len(), kem::CIPHERTEXT_LENGTH)?;
    check_length("KEM shared secret", ss.len(), kem::SHARED_SECRET_LENGTH)?;
    let _cleanup = begin_operation()?;
    #[cfg(test)]
    test_support::record_operation();
    // SAFETY: both inputs are live readable slices; checked output is exclusive.
    let status = unsafe {
        cadp_kem_decaps(
            sk.as_ptr(),
            sk.len(),
            ct.as_ptr(),
            ct.len(),
            ss.as_mut_ptr(),
            ss.len(),
        )
    };
    operation_status(status, Error::DecapsulationFailure)
}

pub(super) fn signature_keypair(pk: &mut [u8], sk: &mut [u8]) -> Result<(), Error> {
    check_length(
        "signature public key",
        pk.len(),
        signature::PUBLIC_KEY_LENGTH,
    )?;
    check_length(
        "signature secret key",
        sk.len(),
        signature::SECRET_KEY_LENGTH,
    )?;
    let _cleanup = begin_operation()?;
    #[cfg(test)]
    test_support::record_operation();
    // SAFETY: exclusive nonoverlapping outputs have the checked native lengths.
    let status = unsafe { cadp_sig_keypair(pk.as_mut_ptr(), pk.len(), sk.as_mut_ptr(), sk.len()) };
    operation_status(
        status,
        Error::KeyGenerationFailure {
            algorithm: "ML-DSA-65",
        },
    )
}

pub(super) fn sign(sk: &[u8], message: &[u8], signature: &mut [u8]) -> Result<usize, Error> {
    sign_impl(sk, message, signature, false)
}

fn sign_impl(
    sk: &[u8],
    message: &[u8],
    signature: &mut [u8],
    context_api: bool,
) -> Result<usize, Error> {
    check_length(
        "signature secret key",
        sk.len(),
        signature::SECRET_KEY_LENGTH,
    )?;
    check_length(
        "signature capacity",
        signature.len(),
        signature::MAX_SIGNATURE_LENGTH,
    )?;
    let _cleanup = begin_operation()?;
    let mut written = 0usize;
    #[cfg(test)]
    test_support::record_operation();
    // SAFETY: inputs remain live; output has the verified maximum capacity and
    // cannot alias inputs. written is an exclusive size_t. Context is always empty.
    let status = unsafe {
        cadp_sig_sign(
            sk.as_ptr(),
            sk.len(),
            message_pointer(message),
            message.len(),
            signature.as_mut_ptr(),
            signature.len(),
            &mut written,
            context_api.into(),
        )
    };
    operation_status(status, Error::SigningFailure)?;
    // ML-DSA-65 is fixed-size in FIPS 204, but respect/check the API's actual length.
    // Never truncate/index using an unchecked native result.
    if written != signature::MAX_SIGNATURE_LENGTH {
        return Err(Error::UnexpectedSignatureLength { actual: written });
    }
    Ok(written)
}

pub(super) fn verify(pk: &[u8], message: &[u8], signature: &[u8]) -> Result<bool, Error> {
    verify_impl(pk, message, signature, false)
}

fn verify_impl(
    pk: &[u8],
    message: &[u8],
    signature: &[u8],
    context_api: bool,
) -> Result<bool, Error> {
    check_length(
        "signature public key",
        pk.len(),
        signature::PUBLIC_KEY_LENGTH,
    )?;
    if signature.len() != signature::MAX_SIGNATURE_LENGTH {
        return Err(Error::MalformedSignature {
            actual: signature.len(),
        });
    }
    let _cleanup = begin_operation()?;
    #[cfg(test)]
    test_support::record_operation();
    // SAFETY: checked fixed-size key/signature and borrowed message stay readable;
    // no outputs/aliases. C exposes ordinary or NULL,0 pure-context APIs only.
    let status = unsafe {
        cadp_sig_verify(
            pk.as_ptr(),
            pk.len(),
            message_pointer(message),
            message.len(),
            signature.as_ptr(),
            signature.len(),
            context_api.into(),
        )
    };
    if status == 5 {
        // liboqs reports verification rejection as OQS_ERROR. No success on error.
        Ok(false)
    } else {
        operation_status(status, Error::UnexpectedBackendMetadata).map(|()| true)
    }
}

pub(super) fn cleanse(buffer: &mut [u8]) {
    // SAFETY: exclusive live buffer, correct byte count; no initialization required
    // for OQS_MEM_cleanse in the approved OpenSSL-free build.
    unsafe { cadp_cleanse(buffer.as_mut_ptr(), buffer.len()) };
    #[cfg(test)]
    test_support::observe_cleanse(buffer);
}

pub(super) fn secret_equal(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    // SAFETY: equal-sized readable slices live for this call. The reviewed library
    // comparison avoids early exit based on the secret bytes; length is public.
    unsafe { cadp_secret_equal(a.as_ptr(), b.as_ptr(), a.len()) == 1 }
}

#[cfg(test)]
pub(super) fn sign_empty_context(
    sk: &[u8],
    message: &[u8],
    output: &mut [u8],
) -> Result<usize, Error> {
    sign_impl(sk, message, output, true)
}

#[cfg(test)]
pub(super) fn verify_empty_context(
    pk: &[u8],
    message: &[u8],
    signature: &[u8],
) -> Result<bool, Error> {
    verify_impl(pk, message, signature, true)
}

#[cfg(test)]
pub(super) mod test_support {
    use std::cell::Cell;
    use std::sync::atomic::AtomicUsize;
    pub static INITIALIZATIONS: AtomicUsize = AtomicUsize::new(0);
    thread_local! {
        static OPERATIONS: Cell<usize> = const { Cell::new(0) };
        static STOPS: Cell<usize> = const { Cell::new(0) };
        static CLEANSES: Cell<(usize, bool)> = const { Cell::new((0, true)) };
    }
    pub fn record_operation() {
        OPERATIONS.set(OPERATIONS.get() + 1);
    }
    pub fn operations() -> usize {
        OPERATIONS.get()
    }
    pub fn record_stop() {
        STOPS.set(STOPS.get() + 1);
    }
    pub fn stops() -> usize {
        STOPS.get()
    }
    pub fn observe_cleanse(buffer: &[u8]) {
        let (count, prior_clean) = CLEANSES.get();
        CLEANSES.set((
            count + 1,
            prior_clean && buffer.iter().all(|byte| *byte == 0),
        ));
    }
    pub fn cleanses() -> (usize, bool) {
        CLEANSES.get()
    }
}
