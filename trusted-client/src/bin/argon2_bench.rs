//! Synthetic/public input only. No password arguments, entropy or derived output.
fn main() -> Result<(), cadp_pqc::storage::Error> {
    if std::env::args_os().len() != 1 {
        return Err(cadp_pqc::storage::Error::InvalidBundle);
    }
    for [memory, time, lanes] in [[65536, 3, 4], [131072, 6, 4], [262144, 6, 4]] {
        let costs = cadp_pqc::storage::KdfParameters::new(memory, time, lanes)?;
        for (i, elapsed) in cadp_pqc::storage::benchmark_argon2_profile(costs)?
            .iter()
            .enumerate()
        {
            println!(
                "memory_kib={} time_cost={} parallelism={} run={} wall_ms={:.3}",
                memory,
                time,
                lanes,
                i + 1,
                elapsed.as_secs_f64() * 1000.0
            );
        }
    }
    Ok(())
}
