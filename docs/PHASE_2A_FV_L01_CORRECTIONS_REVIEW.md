# Phase 2A P2A-FV-L01 corrective review

Date: 2026-10-13. Scope: LOW compiler-information cache regression only, plus its
query-exception clarification. P2A-RA-L01 remains independently CLOSED. Phase 2B/3
are unapproved and were not implemented. This is corrective self-review, not Astra
confirmation or independent Phase 2A approval.

Evidence: [PHASE_2A_FV_L01_CORRECTIONS_VALIDATION.txt](PHASE_2A_FV_L01_CORRECTIONS_VALIDATION.txt).
The original Astra finding, verdict and all historical independent evidence are
preserved unchanged. Status: **FIXED / READY FOR FINAL ASTRA CONFIRMATION**.
All required regressions completed; this is not independent approval.

## Before-correction reproduction

An existing empty disposable target reproduced invalid response-file Clippy exit
101 followed immediately by ordinary strict-Clippy exit 101. No cache was removed
between these steps. Cargo stored the failed invocation
`guard_rustc.py /usr/bin/clippy-driver rustc -vV` in .rustc_info.json; its cache key
omits invocation-specific CLIPPY_ARGS. The normal default build's backend remained
linux_getrandom; the finding is availability/test sensitivity, not RNG downgrade.

## Observed Cargo/Clippy queries

A separate forwarding observation proxy recorded actual argv and selected public
flag channels. It necessarily fails the project's required wrapper-identity check
later and is not an accepted build. Cargo/rustc 1.98.1 with Clippy 0.1.98 was
observed using --version/-vV identity calls and this fixed target-information request
(the existing exact -V identity form is also supported):

```text
- --crate-name ___ --print=file-names
[optional --cfg getrandom_backend="linux_getrandom"]
[optional --target x86_64-unknown-linux-gnu]
--crate-type bin --crate-type rlib --crate-type dylib --crate-type cdylib
--crate-type staticlib --crate-type proc-macro
--print=sysroot --print=split-debuginfo --print=crate-name --print=cfg -Wwarnings
```

The native explicit-target host request omits target cfg; the target request carries
it. The equals form of the single approved cfg is also accepted. The real driver
ignores CLIPPY_ARGS for the observed identity/metadata requests; direct driver probes
with forbidden response-file markers succeeded as metadata queries. The observed
private extra-argument separator is __CLIPPY_HACKERY__. Production does not parse
that private language or open response files.

## Correction and policy

The wrapper validates direct @arguments and trusted compiler/workspace executables
first. Only exact identity or enumerated target-information shapes can bypass the
invocation-specific CLIPPY_ARGS check. No source/output/emission/additional print
argument can extend a handshake. Unknown Cargo placeholder-query shapes reject even with valid CLIPPY_ARGS, so an
unreviewed shape cannot pass normally but cache a channel-dependent failure. Extra
Rust flags that alter this handshake require review; this restriction is separate
from ordinary supported Clippy lint flags. Unknown query shapes do not gain an exemption.
All artifact calls, including host build-script tools, still reject @ or backend
markers in the extra channel before exec. The generic --print exemption is removed;
--print=link-args can compile/link and is never intrinsically harmless.

No cfg injection, shell, argument rewriting, response-file read, cache deletion or
cache-disable setting is introduced. Cargo's compiler wrapper identity requirement,
root compile-time exclusivity, rustdoc launcher and syscall-only RNG remain unchanged.
Handshakes may contain zero or exactly one approved cfg; duplicate/alternate settings
cannot match them. Future toolchain query shapes require reviewed adaptation.

## Persistent regression coverage

P2A-BUILD-006 starts with an existing empty target, retains the same target/cache for
all 14 steps, and executes five sequences:

| Sequence | Actual required result |
|---|---|
| invalid response -> valid strict Clippy | reject -> accept |
| valid -> invalid CUSTOM -> valid | accept -> reject -> accept |
| invalid response -> valid -> invalid CUSTOM -> valid | reject -> accept -> reject -> accept |
| duplicate approved literal -> valid | reject -> accept |
| repeated lint-only flags | accept -> accept -> accept |

Targeted execution passed: 5 actual compilation rejections and 9 valid invocations.
No target/cache metadata was deleted; CARGO_CACHE_RUSTC_INFO was not disabled. The
cache remains present with successful identity and target-information results.

Each negative must include an actual artifact-producing --crate-name invocation
in compiler stderr plus the policy diagnostic; a cached -vV/print failure cannot
satisfy that assertion. P2A-BUILD-005 preserves 23 fresh/reused final-crate cases,
wrapper/forwarding/rustdoc/exclusivity controls, and adds valid recovery after every
Clippy negative. Its 13 Clippy cases include the previous six and the other seven
alternate backend values. No negative-only cached-failure chain is counted as proof.

Public stubs independently test exact argv preservation and handshake sensitivity:
adding output, emission, link-args, response-file or conflicting cfg arguments loses
the exemption. Generic --print=cfg/link-args without approved cfg rejects. Actual
Clippy driver behavior and full supported Cargo commands provide integration evidence
beyond those stubs. Before-correction failure is retained as sensitivity evidence.

## Security self-review and preservation

Reviewed the actual diff, response-file/duplicate/alternate behavior, query shapes,
Clippy nesting, cache sequences, direct forwarding and error paths. Zero new unsafe
Rust, shell execution, production secret hook, RNG fallback or cryptographic change.
No permissive generic print exception remains. Build.rs and .cargo/config.toml are
unchanged. The approved source pin, static linkage and Phase 1 options remain.

Toolchain/PATH/Python/local sources remain trusted inputs. This is not a malicious
compiler sandbox. Exact query policy is intentionally coupled to reviewed Cargo/
Clippy 0.1.98 with Cargo/rustc 1.98.1/native x86_64; toolchain changes may fail
closed and need reinspection.
Software erasure, upstream M-01/L-19, dependency-copy and offline-time limitations
remain. No TSan, LeakSanitizer, full-backend sanitization or AVX-512 hardware claim.
All 19 PKI runtime test rows remain PLANNED. Preservation and actual command
results are recorded below and in the separate evidence file.

## Manual verification / demonstration

Run from checkout root with the approved ignored vendor/source inputs:

```sh
export CARGO_HOME="$PWD/.deps/cargo-home"
python3 scripts/check_rng_policy.py
python3 scripts/validate_phase2a.py
make check
cargo test --offline --locked --release --manifest-path trusted-client/Cargo.toml
git diff --check
```

The policy runner prints P2A-BUILD-006's fourteen outcomes and requires valid
recovery after each independent Clippy negative. It owns/removes only temporary
public fixtures after the full sequence; nothing is cleared between steps.

## Proposed next step

Request final targeted Astra confirmation of this LOW correction. No Phase 2B
or Phase 3 implementation is authorized by this report.


## Final regression results

All 12 required commands returned exit 0. Debug and release each executed **41 unit
tests and 3 doctests: 44 passed, 0 failed, 0 ignored**. Formatting and strict Clippy
passed. Phase 1 documentation validation passed six checks; Mermaid rendering was
skipped because mmdc is unavailable. This is the only documented tooling skip.

The first full run used an initially empty task-owned Cargo home and fresh targets,
with local vendor sources and offline/locked Cargo mode. A final query hardening
(reject unreviewed placeholder-query shapes regardless of CLIPPY_ARGS) was applied
while that first run was underway. Therefore final formatting/Clippy/debug/release
were rerun in a fresh final-normal-target, and check_rng_policy/validate_phase2a/
make check were rerun against frozen final source hashes in independent fresh final
policy targets. All final reruns returned 0. No earlier-wave result is presented as
proof of a later source revision. Native/source-pin scripts do not depend on the
changed query-matching branch and passed in the full run and final make check.

| Required command | Actual result |
|---|---|
| `cargo fmt --manifest-path trusted-client/Cargo.toml --check` | exit 0 |
| `cargo clippy --offline --locked --manifest-path trusted-client/Cargo.toml --all-targets -- -D warnings` | exit 0 |
| `cargo test --offline --locked --manifest-path trusted-client/Cargo.toml` | exit 0 |
| `cargo test --offline --locked --manifest-path trusted-client/Cargo.toml --release` | exit 0 |
| `python3 scripts/check_rng_policy.py` | exit 0 |
| `python3 scripts/validate_phase2a.py` | exit 0 |
| `python3 scripts/validate_phase1.py` | exit 0 |
| `python3 scripts/check_pqc_native.py` | exit 0 |
| `python3 scripts/check_pqc_source_pin.py` | exit 0 |
| `make build` | exit 0 |
| `make check` | exit 0 |
| `git diff --check` | exit 0 |

P2A-BUILD-003: 36 Cargo rejections; P2A-BUILD-004: 18 rustc/rustdoc rejections
with omitted-exclusivity sensitivity controls; P2A-BUILD-005: 23 per-crate cases in
fresh/reused stages, 7 wrapper overrides and 13 actual Clippy rejections each followed
by valid recovery; P2A-BUILD-006: 14 steps (5 rejected, 9 accepted) retaining the same
cache. ENOSYS and EPERM production syscall-failure controls passed without fallback.
Phase 1 native/configuration checks and all four Rust partial-output cleanup paths
passed, as did six source-pin rejection controls. Expected adversarial failures are
successful negative tests, not failures of the final suites.

The original poisoned target was also retried after correction: the exact normal
strict-Clippy command returned 0 without deleting its cache. Before/after snapshots
and actual commands are retained in the validation file.

## Preservation evidence

Before-edit SHA-256 snapshots were compared to final files. Only the two scripts and
five existing documentation files listed below changed in this task; every other
snapshotted source, dependency pin, configuration and historical Astra file remains
byte-for-byte unchanged. Owner's preexisting CRYPTO_PROTOCOL.md/KEY_MANAGEMENT.md
changes were preserved and are not this task's work. Unsafe inventory remains one
extern block / ten call blocks in Phase 1 ffi.rs, with zero additions. All 19 PKI
rows are unchanged from the baseline and remain PLANNED.

The approved liboqs checkout is clean at 5a1a854b0dc9f2141bdc771c555ee60c37950183.
Six generated/installed debug/release oqsconfig.h headers from the final fresh target
confirm SHA3_AVX512VL/OpenSSL undefined, pthreads enabled and the two approved
algorithms enabled. Phase 1 runtime metadata, sizes and empty-context tests passed.

## Corrective security findings

- CRITICAL / HIGH / MEDIUM / LOW: no unresolved finding discovered in this corrective
  self-review. P2A-FV-L01 is FIXED / READY FOR FINAL ASTRA CONFIRMATION, not independently
  approved. P2A-RA-L01 remains historically CLOSED.
- INFORMATIONAL: exact metadata matching is coupled to the reviewed native toolchain.
  Additional Rust flags that alter its query shape require review and fail closed.
  Ordinary strict/lint-only Clippy flags remain supported. Compiler/PATH/local build
  inputs remain trusted. No generic --print, arbitrary query combination, cache-reset
  workaround, response-file expansion or shell forwarding was introduced.

## Files changed by this task

Modified: scripts/guard_rustc.py, scripts/check_rng_policy.py, docs/DECISIONS.md,
docs/ENVIRONMENT.md, docs/KNOWN_LIMITATIONS.md, docs/TEST_MATRIX.md,
docs/PHASE_2A_REVIEW.md. Created: this review and
PHASE_2A_FV_L01_CORRECTIONS_VALIDATION.txt. Disposable public observation/test/log
artifacts remain Git-ignored under .deps/p2a-fv-l01 and are not production code.
