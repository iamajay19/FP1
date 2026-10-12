# Phase 2A P2A-A-M01 corrective review

Scope: authorized RNG build-policy correction only. No Phase 2B/3 implementation.
Independent Astra evidence/verdict are unchanged. This is corrective self-review,
not independent approval. Actual commands/output: [PHASE_2A_CORRECTIONS_VALIDATION.txt](PHASE_2A_CORRECTIONS_VALIDATION.txt).

## Root cause and correction

The former build guard inspected only literal Cargo unit-separated flags and ignored
@response-file arguments; rustc expanded a hidden CUSTOM cfg. The root guard required
only presence of the approved cfg. getrandom 0.4.3 selects CUSTOM before Linux when
both are active. No default-build downgrade or CBOR-triggered attack is implied.

Reject every @argument without reading its contents. Build.rs validates exactly one
approved literal in effective CARGO_ENCODED_RUSTFLAGS, effective
CARGO_CFG_GETRANDOM_BACKEND and exposed environment flag sources. It tracks these
sources and the rustdoc launcher for rebuilds. Strict literal validation rejects
ambiguous whitespace/alternate/unknown/duplicate settings; it does not normalize them.

The root compile guard independently rejects custom, efi_rng, rdrand, rndr,
linux_raw, windows_legacy, unsupported and extern_impl, the complete non-approved
set from pinned getrandom/src/backends.rs. Check-cfg declares all nine values.
Identical cfg duplicates are a set at compile time; textual guards enforce multiplicity.

Cargo config rustdocflags are not exposed to build.rs. A small required Python
launcher checks actual rustdoc argv before execvp of trusted PATH rustdoc. It rejects
@files and requires exactly one approved --cfg value, separately from --check-cfg.
No shell, response-file parser, arbitrary file-content reads or compiler patch.
Build.rs requires its canonical checkout path; missing/overridden launcher rejects.
Keep the checked-in launcher executable. The toolchain/PATH/source trust boundary
remains; this is not a hostile-toolchain sandbox.

## Regression coverage

P2A-BUILD-001/002 and all existing IDs are preserved. New P2A-BUILD-003 performs 36
actual Cargo rejections: the original CUSTOM check and build, alternate/duplicate/
conflict/nested response files, encoded Rust and rustdoc flags, direct/mixed/missing/
duplicate literals, ordering/quoting, Cargo --config, target-specific config/environment
and rustdoc launcher overrides. Some cases reject within getrandom or rustc before
our build script; output states that rather than claiming the project guard ran.

P2A-BUILD-004 checks the pinned backend set against both declarations, compiles the
exact root guard prefix independently with rustc and rustdoc, and rejects each of
eight alternatives plus expanded CUSTOM (18 expected failures). Both approved
controls succeed; deleting only exclusivity makes the hidden-CUSTOM controls succeed,
proving sensitivity. It does not link/run a custom RNG or introduce unsafe fixtures.
The post-fix original configuration cannot produce an accepted project rlib, so no
custom-symbol executable can be built from that rejected invocation.

## Corrective security self-review

Checked response/nested files, custom/alternate/duplicate cfgs, ordering, whitespace,
encoded/ordinary flags, Cargo config/target routes and rustdoc arguments. New paths
add no project unsafe, runtime panic/unwrap/expect or secret access. No shell
interpolation: subprocess argument arrays and execvp only. Cargo determines effective
Rust flags by precedence; lower-priority flags it discards do not become active cfgs.
Compiler wrappers/local tools/source modifications remain trusted rather than being
claimed controlled by a textual guard. Removing the compile-time exclusivity alone
is caught by the persistent sensitivity control.

Foundation runtime files, Phase 1 PQC/native/FFI files, exact dependency manifest/
lock, source pin, crypto labels/bytes and audited backend options are unchanged.
Generated and installed headers agree: liboqs 0.16.0; approved exact commit;
SHA3 AVX512VL and OpenSSL undefined; pthreads and only required algorithms enabled.
One unsafe extern block plus ten call blocks remains confined to ffi.rs.
Normal Drop and partial-output cleanup policies are unchanged. M-01/L-19 remains
OPEN/accepted, with no fallback entropy or process-abort interception.

No unresolved new CRITICAL/HIGH/MEDIUM correction finding identified by self-review.
P2A-A-M01: FIXED / READY FOR TARGETED ASTRA RE-AUDIT, with all 12 required top-level commands exiting 0. Phase 2A is not independently approved; hold Phase 2B authorization.

## Initial corrective issues observed

- An initial [env] RUSTDOC configuration was not Cargo's selected tool path;
  the new build guard rejected it. Use supported build.rustdoc instead.
- An overridden bare rustdoc path initially produced a generic canonicalization
  error. Changed to the explicit guarded-launcher rejection diagnostic.
- First full regression stopped at the repository's 100-character Python line
  rule. Wrapped the new script lines; subsequent document/static validation passes.
These are recorded corrective attempts, not changes to crypto or upstream source.

## Manual verification / demonstration

Run from checkout root with ignored approved vendor and liboqs sources present:

```sh
export CARGO_HOME="$PWD/.deps/cargo-home"
correction_target=$(mktemp -d /tmp/fp1-p2a-m01-manual.XXXXXX)
export CARGO_TARGET_DIR="$correction_target"
cargo fmt --manifest-path trusted-client/Cargo.toml --check
cargo clippy --offline --locked --manifest-path trusted-client/Cargo.toml --all-targets -- -D warnings
cargo test --offline --locked --manifest-path trusted-client/Cargo.toml
cargo test --offline --locked --release --manifest-path trusted-client/Cargo.toml
cargo test --offline --locked --manifest-path trusted-client/Cargo.toml pqc_conc_001 -- --test-threads=1
python3 scripts/check_rng_policy.py
python3 scripts/validate_phase2a.py
python3 scripts/check_pqc_native.py
python3 scripts/check_pqc_source_pin.py
python3 scripts/validate_phase1.py
make build
make check
git diff --check
```

The policy script prints exact adversarial flag values, commands and rejection
sources; assert P2A-BUILD-003/004 PASS. Then show ENOSYS/EPERM, native cleanup,
metadata/context and ordinary doctest PASS lines. Use only public synthetic fixture
patterns/ephemeral test keys. No production identity, PKI, password or storage data.
Proposed next action: targeted independent Astra re-audit of P2A-A-M01, not Phase 2B.
