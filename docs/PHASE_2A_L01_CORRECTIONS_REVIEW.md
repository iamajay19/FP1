# Phase 2A P2A-RA-L01 final corrective review

Dates: 2026-10-12–13. Scope: the authorized final compiler-argument policy correction
only. P2A-RA-L01 is FIXED / READY FOR FINAL TARGETED ASTRA VERIFICATION. This is
corrective implementation and self-review, not independent Phase 2A approval.
Phase 2B and Phase 3 remain unapproved and unimplemented.

Actual commands, outputs, controls and preservation hashes are recorded separately
in [PHASE_2A_L01_CORRECTIONS_VALIDATION.txt](PHASE_2A_L01_CORRECTIONS_VALIDATION.txt).
All original Astra reports, verdicts and evidence remain unchanged.

## Root cause and correction

Cargo does not expose extra final-crate arguments after `cargo rustc --` to
build.rs. The original effective-cfg Rust guard rejects alternate/custom backends
but cannot distinguish duplicate identical cfgs after rustc expansion. Before
editing, the actual project accepted `cargo rustc -- @old-duplicate.rsp` and
produced its library, exit 0. The file contained only an additional approved cfg.

`.cargo/config.toml` now selects Cargo's standard `build.rustc-wrapper`:
`scripts/guard_rustc.py`. Cargo supplies the real tool and actual argv, including
per-crate extra flags. The executable Python wrapper rejects every argument
starting with `@` before forwarding; it never opens response files. It reuses the
unchanged rustdoc literal-cfg parser, requiring exactly one
`getrandom_backend="linux_getrandom"` for runtime artifact compilation. Split and
equals forms are covered. Any duplicate, alternate, unknown or ambiguous backend
setting rejects. No cfg is added or rewritten by the production wrapper.

Forwarding uses `os.execvp` with the original argument list, without a shell or
command concatenation. Cargo provides the executable; PATH and the toolchain remain
trusted. Executable lookup failure and direct wrapper self-recursion reject. No
fixed toolchain installation path is embedded. Normal Cargo Clippy nesting is
supported; arbitrary workspace compiler wrappers reject.

Build.rs requires `RUSTC_WRAPPER` to resolve to the checked-in wrapper. It tracks
the environment setting and wrapper source so disabling/replacing the configured
wrapper through ordinary Cargo environment or configuration overrides fails closed.
The existing root compile-time backend-exclusivity guard and guarded rustdoc path
are unchanged.

## Cargo host calls and Clippy

Exact compiler identity requests, non-artifact `--print` queries without `--emit`,
and target-cfg-free host `build_script_build` tools can omit the runtime cfg. Cargo
uses these when an explicit native `--target` is selected. They do not use the
application RNG in the approved graph. Responses and duplicate/alternate cfgs still
reject in these calls. A `--print` call combined with artifact emission has no
query exemption. Actual native explicit-target compilation passed.

Self-review found a second argument channel within the proposed correction:
Clippy appends CLI arguments from `CLIPPY_ARGS` after Cargo's wrapper-visible argv.
An initial `cargo clippy -- @old-duplicate.rsp` probe succeeded. The final wrapper
therefore conservatively rejects `@` or `getrandom_backend` markers in that channel
before invoking Clippy. It does not parse Clippy's private delimiter or argument
language. Ordinary warning flags, including `-D warnings`, remain supported. Six
actual Clippy rejection cases and a normal strict Clippy run verify this correction.

## Persistent regression and sensitivity

New stable ID **P2A-BUILD-005**, integrated into `check_rng_policy.py` and the existing
Phase 2A validator/Make check path, exercises the unmodified production crate:

- 23 `cargo rustc` cases: duplicate/custom/alternate/nested response files, duplicate
  approved cfgs in both forms, all eight recognized alternates in both forms and a
  whitespace variant. Each case rejects at the actual final-crate wrapper.
- The first stage begins with an absent target directory. Failed cases share only
  generated dependency/native artifacts. No project rlib/rmeta is produced. After
  an approved build, the reused stage repeats all 23 cases and asserts the existing
  library set and hashes remain unchanged. This is not 23 separately clean builds.
- Seven wrapper disable/replacement controls reject through environment, Cargo
  configuration and workspace-wrapper routes; two additional workspace override
  probes were performed during self-review.
- Six actual Clippy cases reject: duplicate/custom/nested response files, duplicate
  approved split/equals cfgs and custom literal cfg.
- Public compiler stubs verify exact argument boundaries, including spaces and
  shell metacharacters, normal Clippy nesting, missing compiler and recursion.
  A no-policy stub accepts the rejected args, demonstrating sensitivity.
- Identity/query/host-call controls reject forbidden arguments. Artifact emission
  without the required cfg rejects. Explicit native-target compilation succeeds.

Existing P2A-BUILD-003 (36 actual Cargo/rustdoc rejection cases) and P2A-BUILD-004
(18 independent rustc/rustdoc exclusivity rejections and two omitted-guard negative
controls) remain intact. All recognized getrandom 0.4.3 cfg values remain covered:
`custom`, `linux_getrandom`, `linux_raw`, `rdrand`, `rndr`, `efi_rng`,
`windows_legacy`, `unsupported`, `extern_impl`. The compile-time Rust defense was
not changed. Forced ENOSYS and EPERM still return typed RNG failure with no fallback.

## Commands and actual results

All twelve required command entries exited 0. Rust tests passed in both debug and
release: **41 unit tests and 3 doctests per profile; 0 failed, 0 ignored**. Native
checks passed the four FFI contract groups and four Rust partial-output cleanup
paths; source-pin checks rejected all six corrupt/incorrect-source controls. The
documentation validator passed six checks and skipped one: Mermaid rendering,
because `mmdc` is unavailable. No required security/runtime check was skipped.

The full runner started with an empty task-specific CARGO_HOME and absent target,
using the supplied offline vendor tree. Later commands reused that cache. Because
the Clippy-channel refinement occurred while earlier policy runs were in flight,
strict Clippy, formatting, debug/release tests were rerun after the final change;
the integrated policy suite was rerun separately after the final change. Evidence
retains the earlier outputs rather than relabeling them as final-source executions.

From the project root, the exact manual regression commands are:

```sh
export CARGO_HOME="$PWD/.deps/cargo-home"
cargo fmt --manifest-path trusted-client/Cargo.toml --check
cargo clippy --offline --locked --manifest-path trusted-client/Cargo.toml --all-targets -- -D warnings
cargo test --offline --locked --manifest-path trusted-client/Cargo.toml
cargo test --offline --locked --manifest-path trusted-client/Cargo.toml --release
python3 scripts/check_rng_policy.py
python3 scripts/validate_phase2a.py
python3 scripts/validate_phase1.py
python3 scripts/check_pqc_native.py
python3 scripts/check_pqc_source_pin.py
make build
make check
git diff --check
```

For a separate clean target, set CARGO_TARGET_DIR to a new disposable directory
before these commands. The persistent policy suite independently creates/removes
its own temporary fixtures and tests a reused approved target.

## Demonstration

The following public fixture must reject, with `Phase 2A rustc policy rejected` and
a nonzero Cargo exit. It must not replace any prior approved project library:

```sh
response_dir=$(mktemp -d /tmp/fp1-l01-manual.XXXXXX)
printf '%s\n' '--cfg=getrandom_backend="linux_getrandom"' > "$response_dir/duplicate.rsp"
cargo rustc --offline --locked --manifest-path trusted-client/Cargo.toml -- @"$response_dir/duplicate.rsp"
cargo rustc --offline --locked --manifest-path trusted-client/Cargo.toml -- '--cfg=getrandom_backend="linux_getrandom"'
rm "$response_dir/duplicate.rsp"
rmdir "$response_dir"
```

Run `python3 scripts/check_rng_policy.py` for complete public adversarial fixtures,
including hidden CUSTOM, every alternate, nested files, both flag forms, compiler
overrides and rustdoc routes. No password or production secret is involved.

## Corrective security self-review

- **LOW P2A-RA-L01: FIXED / READY FOR FINAL TARGETED ASTRA VERIFICATION.** The old
  accepted per-crate duplicate response now rejects in initial and reused targets.
- **LOW implementation issue, fixed:** the initial wrapper did not account for
  Clippy's separate extra-argument channel. Actual before/after probes and six
  permanent cases verify the final conservative rejection.
- **LOW portability regression, fixed:** an early wrapper rejected Cargo's cfg-free
  host query during explicit native-target builds. The scoped non-artifact/host
  exceptions restore the supported build without permitting forbidden cfgs/files.
- **INFORMATIONAL:** Python/static formatting failures during development were
  corrected and the documentation/static validator subsequently passed.
- No unresolved CRITICAL, HIGH, MEDIUM or LOW issue was found in corrective
  self-review. This conclusion awaits the requested independent final verification.

No new unsafe Rust, production panic/unwrap/expect, secret accessor, RNG fallback,
response-file read, shell forwarding or production failure-injection hook was added.
The unsafe inventory remains one extern block and ten unsafe calls, all confined to
Phase 1 ffi.rs. Python test assertions are public-fixture checks, not runtime paths.

## Preservation and limitations

SHA-256 comparisons against the pre-edit working-tree snapshot confirm all Rust
runtime modules, native/PQC fixtures, Cargo dependency pins, guarded rustdoc,
Makefile, protocol/key-management files and original Astra evidence remain unchanged.
The protocol/key-management files already had owner changes before this task; those
were preserved and are not part of this correction. Original audit evidence remains
historical. All 19 PKI runtime rows are byte-identical to HEAD and remain PLANNED.

Backend remains statically linked liboqs 0.16.0 at
`5a1a854b0dc9f2141bdc771c555ee60c37950183`. Actual generated/installed configuration
retains disabled AVX-512 SHA3 and OpenSSL, required pthreads, ML-KEM-768 and
ML-DSA-65. The existing size, pure/empty-context, implicit-rejection and concurrency
tests passed without changes. M-01 fatal upstream failures remain an accepted
operational limitation; no recovery or fallback was introduced.

This policy assumes trusted local sources, Python, Cargo, compiler/Clippy, PATH and
filesystem. It is not a malicious-toolchain sandbox or an artifact attestation scheme.
Runtime compilation requires the Cargo wrapper; direct unmanaged compiler calls
are not an approved build. The unchanged compile-time guard remains defense in
depth. Response files and arbitrary workspace wrappers are intentionally unsupported.
Conservative Clippy marker checks may reject harmless extra flags mentioning the
forbidden markers. Offline vendor and repository-root invocation assumptions remain.

No TSan, LeakSanitizer, full-backend sanitization or AVX-512 hardware execution is
claimed. Existing native ASan/UBSan coverage is limited to the shim/harness as
documented. Software cleanup does not imply physical erasure or cleanup after abort.

## Files changed

- `.cargo/config.toml`
- `trusted-client/build.rs`
- New `scripts/guard_rustc.py` (executable)
- `scripts/check_rng_policy.py`
- `scripts/validate_phase2a.py`
- `docs/DECISIONS.md`
- `docs/ENVIRONMENT.md`
- `docs/KNOWN_LIMITATIONS.md`
- `docs/TEST_MATRIX.md`
- `docs/PHASE_2A_REVIEW.md` (append only)
- New `docs/PHASE_2A_L01_CORRECTIONS_REVIEW.md`
- New `docs/PHASE_2A_L01_CORRECTIONS_VALIDATION.txt`

Existing user/history files are not included in this list. Disposable build/probe
outputs remain Git-ignored under the task-specific .deps directory; durable evidence
is copied into the validation file. No keys or shared secrets were written there.

## Recommendation

Proceed only to final targeted independent Astra verification of P2A-RA-L01.
Do not treat this self-review as Phase 2A approval. Do not begin Phase 2B or Phase 3.
