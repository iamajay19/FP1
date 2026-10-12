//! PQ primitives and Phase 2A foundations; no identity/storage/document workflow.
#![deny(unsafe_code)]

#[cfg(not(all(target_os = "linux", getrandom_backend = "linux_getrandom")))]
compile_error!("Phase 2A requires Linux syscall-only OS randomness");

// Independent of build.rs token validation: rustc expands response files before
// evaluating cfg. All non-approved values recognized by pinned getrandom 0.4.3
// are forbidden even when the approved value is simultaneously present.
#[cfg(any(
    getrandom_backend = "custom",
    getrandom_backend = "efi_rng",
    getrandom_backend = "rdrand",
    getrandom_backend = "rndr",
    getrandom_backend = "linux_raw",
    getrandom_backend = "windows_legacy",
    getrandom_backend = "unsupported",
    getrandom_backend = "extern_impl"
))]
compile_error!("Phase 2A rejects non-approved getrandom backend cfg");

pub mod foundation;
pub mod pqc;
