# Final Phase 2A Astra Verification

Audit record: 2026-10-13 (local date).

Scope: final targeted verification of P2A-RA-L01 and regressions directly caused by its correction. Reviewed commit `6c803e904eebd44f3ea4168d79a3bc1166ea7564`, compared with `a4824f0`. This is not a repeat of the broad Phase 2A audit. No production files were modified. The only deliverable files added by this verification are `docs/PHASE_2A_ASTRA_FINAL_VERIFICATION.md` and `docs/PHASE_2A_ASTRA_FINAL_VALIDATION.txt`; disposable probes remain ignored under `.deps/p2a-astra-final`. Existing owner changes to protocol/key-management documentation and untracked historical evidence were preserved. Phase 2B and Phase 3 remain unapproved.

## Original LOW reproduction

The actual `cargo rustc --offline --locked --manifest-path trusted-client/Cargo.toml -- @duplicate.rsp` route now exits 101 at the final `cadp_pqc` compiler invocation. The response file contained another approved `getrandom_backend="linux_getrandom"` cfg. In an initially absent disposable target, no project rlib/rmeta was produced. After a successful ordinary build, repeating the attack left the set and SHA-256 hashes of existing project libraries unchanged. The persistent suite additionally checks custom, alternate, nested and literal argument forms.

Historical acceptance remains recorded in the original re-audit and corrective evidence. This verification did not patch production back to a vulnerable configuration.

## Actual rustc-boundary enforcement

Cargo's `build.rustc-wrapper` supplies the real compiler executable and actual arguments to `scripts/guard_rustc.py`, including per-crate `cargo rustc -- ...` additions. The wrapper preserves argv and uses `os.execvp`, without a shell or response-file reads. Missing compiler and self-recursion controls reject. The trusted Clippy workspace-driver path is handled without dropping its real compiler argument. Compiler, Cargo, Python, PATH and local toolchain remain trusted inputs.

Explicit native-target Cargo checks succeeded. Host build scripts can omit the runtime cfg because they are host tooling; alternate/duplicate cfgs and response files are still rejected. Query handling has a narrower assurance than its comment states: a direct standalone wrapper invocation with `--print=link-args` can produce an artifact without `--emit`. Tested Cargo artifact invocations include `--emit`, so missing/duplicate/custom/response-file attempts with that print option were rejected. This did not yield an accepted project/backend bypass; see informational observation below.

## Response-file rejection

The actual argument guard rejects every token beginning `@` before compiler execution. It never opens the file, so nesting does not matter. The reviewed persistent suite covers ordinary/encoded Rust flags, Cargo configuration, target-specific flags, final-crate arguments and rustdoc paths. Independent final-crate and rustdoc response probes also rejected. Blanket rejection intentionally includes benign response files.

## Duplicate cfg rejection

Both split `--cfg getrandom_backend="linux_getrandom"` and equals `--cfg=getrandom_backend="linux_getrandom"` duplicates reject at actual argv validation. The parser counts cfg occurrences; merely finding an approved literal is insufficient. Fresh-stage and reused-stage tests check no new or changed accepted project libraries after rejected final-crate builds.

## Backend exclusivity

Pinned getrandom 0.4.3 source independently yields nine recognized values: `linux_getrandom`, `custom`, `linux_raw`, `rdrand`, `rndr`, `efi_rng`, `windows_legacy`, `unsupported`, `extern_impl`. Only `linux_getrandom` is permitted. The other eight remain rejected by the independent root compile-time guard as well as the launcher. Per-crate split/equals controls exercise every alternate; the original hidden CUSTOM path remains blocked. The root compile-time guard and its sensitivity controls remain present.

## Clippy-path verification

The wrapper checks Clippy's separate `CLIPPY_ARGS` channel before forwarding. Independently, with compiler-information caching disabled solely in disposable tests, response files, duplicate approved cfg and all eight alternate values rejected, while ordinary strict warnings plus `-W clippy::all` succeeded. Normal strict Clippy also succeeded in the core run.

However, a cache-dependent regression is reproducible: with an existing target directory and no successful Clippy compiler-identity cache, a rejected Clippy argument can cache a failed `rustc -vV` result. A subsequent fully valid Clippy command then fails from that cached result. Resetting only the disposable compiler-information cache restores success. A successful-cache -> invalid -> valid control remains fail-closed and succeeds on the final valid invocation. This is a LOW build-availability/test-quality issue, not an RNG downgrade.

## Wrapper override verification

The persistent controls attempt disabling/replacing the launcher through `RUSTC_WRAPPER`, `CARGO_BUILD_RUSTC_WRAPPER`, Cargo `--config`, and a workspace wrapper. Build-script canonical-path enforcement and wrapper checks reject these tested routes. The actual root compile-time backend guard remains defense in depth. This does not authenticate the toolchain or defend against maliciously modified repository/build tools.

## Normal approved build behavior

The configured Cargo-wide Rust and rustdoc cfg remains exclusively `linux_getrandom`; dependencies receive the Rust cfg. Ordinary build/check/test/release/doctest paths pass in the approved environment. ENOSYS and EPERM controls require typed RNG failure and reject fallback behavior. There is no runtime RandomId change. Valid Clippy retries have the cache limitation described above.

## Preservation/regression check

Diff and hash checks confirm no correction changes to CBOR, key-ID bytes, SHA-256, HKDF, HMAC, Secret32, RandomId runtime API, Q-02/Q-03, PQ algorithms, native FFI or dependency pins. Existing owner protocol/documentation changes were not attributed to this correction or edited by the audit. The unsafe inventory remains one extern block and ten unsafe call blocks, all in Phase 1 `ffi.rs`.

Fresh audit debug/release generated and installed liboqs configuration headers retain AVX-512 SHA3 disabled, OpenSSL disabled, pthread enabled and the required two algorithms enabled. The pinned source commit remains `5a1a854b0dc9f2141bdc771c555ee60c37950183` (liboqs 0.16.0), with static linkage unchanged. All 19 existing Phase 2 runtime rows remain PLANNED.

## Tests executed

See companion `PHASE_2A_ASTRA_FINAL_VALIDATION.txt` for exact commands, environments, outputs, return codes and independent fixture sources. All ten requested commands completed with exit 0.

| Command | Result |
| --- | --- |
| `cargo fmt --manifest-path trusted-client/Cargo.toml --check` | PASS (exit 0) |
| `cargo clippy --offline --locked --manifest-path trusted-client/Cargo.toml --all-targets -- -D warnings` | PASS (exit 0) |
| `cargo test --offline --locked --manifest-path trusted-client/Cargo.toml` | PASS (exit 0); 41 unit tests + 3 doctests, 0 failed/ignored |
| `cargo test --offline --locked --manifest-path trusted-client/Cargo.toml --release` | PASS (exit 0); 41 unit tests + 3 doctests, 0 failed/ignored |
| `python3 scripts/check_rng_policy.py` | PASS (exit 0) |
| `python3 scripts/validate_phase2a.py` | PASS (exit 0) |
| `python3 scripts/validate_phase1.py` | PASS (exit 0); 6 passed, 1 Mermaid-render skip (mmdc unavailable) |
| `make build` | PASS (exit 0) |
| `make check` | PASS (exit 0) |
| `git diff --check` | PASS (exit 0) |

Additional independent Clippy cache-retry testing FAILED as detailed in P2A-FV-L01; forbidden configurations otherwise rejected. No TSan, LeakSanitizer, full-backend sanitization or AVX-512 hardware execution is claimed. Two interrupted policy invocations are recorded as incomplete and are not counted as completed passes; the command was rerun to completion.

## Findings

### CRITICAL

None identified in this targeted scope.

### HIGH

None identified in this targeted scope.

### MEDIUM

None identified in this targeted scope. The original hidden-CUSTOM issue remains blocked.

### LOW

**P2A-FV-L01 — Clippy argument rejection can poison Cargo's compiler-information cache.**

- Affected code: `scripts/guard_rustc.py:39` (CLIPPY_ARGS rejection before identity handling at line 55); regression suite `scripts/check_rng_policy.py:105` (shared-target negative-only Clippy sequence).
- Exact problem: invocation-dependent forbidden Clippy arguments cause the wrapper to fail a cacheable compiler-identity query. Cargo can retain the failed query in an already existing target's `.rustc_info.json`, then replay it for later valid Clippy flags.
- Reachable scenario: create an empty target directory; run Clippy with `-- @duplicate.rsp`; then run ordinary `--all-targets -- -D warnings` using the same target. Both return 101. Rename only that disposable `.rustc_info.json`; the identical valid command returns 0. No production files change. If the target did not exist when the rejected invocation began, Cargo did not retain the failed cache in the tested environment. A previously successful Clippy cache also avoided the sticky failure.
- Attacker input: none at runtime. A local developer/build invocation with a prohibited flag is sufficient. No secret or hostile document is involved.
- Consequence: a valid lint/check workflow remains unavailable until cache reset/bypass, and successive negative test cases may be satisfied by a cached failure rather than fresh evaluation of their distinct inputs. No prohibited backend was accepted.
- Recommended correction: separate invocation-specific Clippy policy checks from cacheable compiler identity without weakening enforcement on actual compilation, or use another small demonstrated cache-safe policy. Do not simply remove the Clippy channel check.
- Required regression: on an existing disposable target, invalid -> valid and valid -> invalid -> valid must yield rejection/success and success/rejection/success respectively. Each negative Clippy backend/response case must independently reach current policy evaluation rather than inherit a failed identity result. Preserve the actual-compilation negative controls.

### INFORMATIONAL

**P2A-FV-I01 — query-exception comment overstates standalone behavior.** At `scripts/guard_rustc.py:62`, `--print` without `--emit` is described as never compiling an artifact. A public standalone `fn main() {}` fixture with `--print=link-args -o ...` disproves that description. Tested Cargo artifact commands still include `--emit` and fail closed without the approved cfg. No project bypass was demonstrated. Restrict/document query exceptions to known metadata-only requests when touching this code; do not claim the wrapper universally prevents artifact emission for arbitrary direct rustc invocations.

Existing limits remain: trusted local compiler/build environment, no third-party authenticity claim, no new sanitizer/hardware claims, and no change to prior secret-erasure or backend process-abort limitations.

## P2A-RA-L01 status

**CLOSED in the reviewed correction:** actual final-crate response-file and duplicate-cfg routes reject on fresh and reused targets, and no new accepted project library is produced. The separate LOW Clippy regression remains open.

## Phase 2A final recommendation

Correct P2A-FV-L01 with a narrow follow-up and targeted verification before unqualified final Phase 2A approval. No cryptographic redesign is requested. Phase 2B and Phase 3 were not started.

PHASE 2A SECURITY FINAL VERIFICATION: APPROVED WITH REQUIRED FIXES
