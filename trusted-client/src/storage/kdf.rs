use super::{Error, secret::SecretBytes};
use argon2::{Algorithm, Argon2, Block, Params, Version};
use std::sync::Mutex;
use std::time::{Duration, Instant};
static ACTIVE_KDF: Mutex<()> = Mutex::new(());
use zeroize::Zeroize;

/// Validated costs only. Algorithm, version, output and salt sizes are fixed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KdfParameters {
    pub(super) memory_kib: u32,
    pub(super) time_cost: u32,
    pub(super) parallelism: u32,
}
impl KdfParameters {
    pub fn new(memory_kib: u64, time_cost: u64, parallelism: u64) -> Result<Self, Error> {
        if !(65536..=262144).contains(&memory_kib)
            || !(3..=6).contains(&time_cost)
            || !(1..=4).contains(&parallelism)
        {
            return Err(Error::InvalidKdfParameters);
        }
        Ok(Self {
            memory_kib: u32::try_from(memory_kib).map_err(|_| Error::InvalidKdfParameters)?,
            time_cost: u32::try_from(time_cost).map_err(|_| Error::InvalidKdfParameters)?,
            parallelism: u32::try_from(parallelism).map_err(|_| Error::InvalidKdfParameters)?,
        })
    }
    pub fn costs(&self) -> [u32; 3] {
        [self.memory_kib, self.time_cost, self.parallelism]
    }
}
struct Workspace(Vec<Block>);
impl Drop for Workspace {
    fn drop(&mut self) {
        for block in &mut self.0 {
            block.zeroize();
        }
        #[cfg(test)]
        super::tests::observe_workspace(&self.0);
    }
}
pub(super) fn derive(
    password: &[u8],
    salt: &[u8],
    costs: KdfParameters,
) -> Result<SecretBytes, Error> {
    // Revalidate BEFORE Params, workspace allocation and backend work. No public
    // construction of an unchecked KdfParameters or bundle-controlled allocation.
    KdfParameters::new(
        u64::from(costs.memory_kib),
        u64::from(costs.time_cost),
        u64::from(costs.parallelism),
    )?;
    if salt.len() != 16 {
        return Err(Error::InvalidKdfParameters);
    }
    if password.is_empty() || password.len() > 4096 {
        return Err(Error::InvalidPassword);
    }
    let params = Params::new(
        costs.memory_kib,
        costs.time_cost,
        costs.parallelism,
        Some(32),
    )
    .map_err(|_| Error::InvalidKdfParameters)?;
    let _active = ACTIVE_KDF
        .lock()
        .map_err(|_| Error::AuthenticationFailure)?;
    #[cfg(test)]
    super::tests::record_workspace();
    let mut memory = Workspace(Vec::new());
    memory
        .0
        .try_reserve_exact(params.block_count())
        .map_err(|_| Error::AllocationFailure)?;
    memory.0.resize(params.block_count(), Block::default());
    let mut output = SecretBytes::zeroed(32, "kek")?;
    #[cfg(test)]
    super::tests::record_kdf();
    Argon2::new(Algorithm::Argon2id, Version::V0x13, params)
        .hash_password_into_with_memory(
            password,
            salt,
            output.borrow_mut(),
            memory.0.as_mut_slice(),
        )
        .map_err(|_| Error::AuthenticationFailure)?;
    Ok(output)
}

/// Five bounded public-input runs. Includes allocation, derivation and workspace
/// cleansing. Prints no derived material and never updates a creation profile.
pub fn benchmark_argon2_profile(candidate: KdfParameters) -> Result<[Duration; 5], Error> {
    let mut durations = [Duration::ZERO; 5];
    for elapsed in &mut durations {
        let start = Instant::now();
        let output = derive(
            b"CADP PUBLIC ARGON2 BENCHMARK INPUT",
            &[0x5a; 16],
            candidate,
        )?;
        drop(output);
        *elapsed = start.elapsed();
    }
    Ok(durations)
}

/// Selected after five measured runs per profile; see Phase 2B implementation evidence.
pub fn creation_parameters() -> KdfParameters {
    KdfParameters {
        memory_kib: 131072,
        time_cost: 6,
        parallelism: 4,
    }
}
