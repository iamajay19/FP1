//! PQ primitives and Phase 2A foundations; no identity/storage/document workflow.
#![deny(unsafe_code)]

#[cfg(not(all(target_os = "linux", getrandom_backend = "linux_getrandom")))]
compile_error!("Phase 2A requires Linux syscall-only OS randomness");

pub mod foundation;
pub mod pqc;
