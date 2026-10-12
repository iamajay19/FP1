# Targeted Phase 2A Astra Re-Audit

Date: 2026-10-12. Scope: P2A-A-M01 correction and direct regressions only.
Reviewed correction commit: `ac9822c81c25e927625bea5faf4af872eeae4eb1`;
pre-correction baseline: `252eb63328195d87a5f922b3439dc9b9f0f477e8`.
Actual independent output: [PHASE_2A_ASTRA_REAUDIT_VALIDATION.txt](PHASE_2A_ASTRA_REAUDIT_VALIDATION.txt).

The original MEDIUM backend-substitution exploit is blocked. One LOW gap remains
in the expressly required blanket response-file/duplicate-argument rejection
policy. No new CRITICAL, HIGH or MEDIUM issue was demonstrated. This review does
not authorize Phase 2B or Phase 3 and makes no production changes.

## Original bypass reproduction

Fresh-target `cargo check` and `cargo build`, each with the approved Linux literal
plus a response file containing CUSTOM, both exited 101 at the project build guard.
No cadp-pqc rlib or rmeta was produced in that target. The original independent
Astra attack and runtime CUSTOM demonstration remain historical evidence;
neither the old finding nor its verdict was rewritten.

Separately, `cargo rustc -- @hidden-custom.rsp` bypasses build.rs's visibility of
per-crate arguments but fails at the new root compile_error. This proves a real
full-crate defense in depth, beyond compiling the small extracted guard fixture.
No custom RNG symbol or runtime fixture is needed once compilation fails.

## Backend exclusivity verification

Pinned getrandom 0.4.3 src/backends.rs recognizes exactly these nine explicit
backend values, in dispatch priority order: custom, linux_getrandom, linux_raw,
rdrand, rndr, efi_rng, windows_legacy, unsupported, extern_impl. Its check-cfg
manifest declaration agrees. The project's root guard and check-cfg declaration
contain precisely this set; all eight non-approved values are forbidden.

Independent full, unmodified crate compilations with the approved cfg plus each
of the eight alternatives all failed with the root guard diagnostic. The
persistent rustc and rustdoc controls each reject eight alternatives plus hidden
CUSTOM; removing only exclusivity admits the hidden-CUSTOM compile control.
Exactly repeated approved cfg values collapse to one Rust cfg membership value;
compile-time cfg membership cannot enforce argument multiplicity. See LOW finding.

## Response-file protection

The persistent 36-case Cargo suite passed: environment, encoded flags, target
configuration/environment, Cargo --config, reordered flags, hidden CUSTOM,
alternate, duplicate, conflicting and nested response-file attempts, plus rustdoc
routes and launcher overrides. Some invalid inputs fail in rustc/getrandom before
the project guard; that proves rejection, not that the project guard executed.
In particular nested response-file input can fail with rustc's multiple-input
error, rather than being recursively expanded. The project does not parse files.

One ordinary Cargo route is absent from those controls: arguments after
`cargo rustc --`. A duplicate approved cfg in a response file through this route
was ACCEPTED, producing a cadp-pqc static Rust library. A literal duplicate through
that route was also accepted. These arguments are not part of build.rs's effective
encoded-flags or Cargo cfg environment. This is the only required residual finding.
The dependency still uses linux_getrandom; this is not a reproduced RNG downgrade.

## Rustdoc/doctest protection

The launcher validates actual argv, refuses @tokens and missing/duplicate/alternate
backend settings, then uses os.execvp with an argument list and no shell. A
controlled executable stub independently observed exact preservation of argument
boundaries, spaces and shell metacharacters. No shell expansion occurred. Under
the inspected trusted PATH, rustdoc is the toolchain executable, not this launcher.

Normal debug and release doctests passed (three each). The persistent rustdoc
negative routes passed, including Cargo configuration-provided response files.
Additional `cargo rustdoc -- @duplicate.rsp` failed in the launcher. Direct
build.rustdoc and CARGO_BUILD_RUSTDOC overrides to /usr/bin/rustdoc failed the
build-script identity check. A missing launcher path also failed closed. Together
with existing RUSTDOC override tests, these establish the reviewed supported
launcher routes. Malicious tool replacement/PATH modification is outside this
build-policy trust boundary; this is not a compiler sandbox.

## Build-script review

The effective CARGO_ENCODED_RUSTFLAGS representation is split at U+001F, not shell
whitespace. Each visible @token is rejected without opening it. Exactly one
literal approved cfg is required, and CARGO_CFG_GETRANDOM_BACKEND must equal
linux_getrandom. Exposed rustdoc environment inputs are also checked. Launcher
identity is canonical-path based; env/file rerun triggers are present.

An independent zero-dependency observation fixture confirmed actual Cargo 1.98.1
behavior: normal encoded flags contain a U+001F separator; a hidden CUSTOM response
file gives effective cfg `custom,linux_getrandom`; duplicate approved values give
only `linux_getrandom`; per-crate extra arguments are absent from both build-script
views. Cargo removes RUSTFLAGS before running build scripts. When explicit encoded
flags contain only Linux and an ignored RUSTFLAGS contains CUSTOM, the build succeeds
with Linux exclusively. This precedence case is not simultaneous backend activation.

The parsing is deliberately restrictive, not a Rust argument-language parser.
No arbitrary response file is trusted/read. The per-crate visibility limitation is
why the root exclusivity guard is necessary and why blanket response-file rejection
is not yet complete.

## Regression-test quality

P2A-BUILD-001 tests the production predicate; P2A-BUILD-002 exercises original
missing/alternate/conflict cases; P2A-BUILD-003 performs 36 real Cargo rejections;
P2A-BUILD-004 compiles actual root guard text with rustc and rustdoc, giving 18
rejections and two successful omitted-exclusivity controls. These are meaningful
controls, not just source-string assertions. The extracted guard tests do not
compile all modules or execute a custom RNG; the full-crate independent probes in
this audit close that narrower evidence gap.

The 36-case suite nevertheless missed cargo rustc's additional per-crate argv.
Add persistent acceptance-sensitive regressions for the LOW finding. Its successful
library build while all existing checks passed demonstrates the coverage gap.

## Normal approved RNG behavior

Fresh offline compilation built dependencies with only the checked-in
linux_getrandom cfg. The production RandomId ENOSYS and EPERM injection controls
passed: both returned typed RandomnessUnavailable. Inspection of the pinned
selected backend shows the getrandom syscall path, without /dev/urandom fallback;
no custom fallback was introduced.

A separate dependency-only `cargo rustc -p getrandom -- --cfg=...custom...` invocation
can compile that dependency in isolation, but the following normal Cargo build
recompiled getrandom without the custom flag. No cache-based project RNG substitution
was observed. A dependency-only compilation is not an accepted project build.

## Phase 1 / Phase 2A preservation

The correction diff leaves foundation runtime files, PQC code, native shim,
Cargo pins/features, Q-02/Q-03 protocol documents and cryptographic byte encodings
unchanged. Hash comparison against the start of this audit found no changes to
any preexisting tracked files or either original independent Astra evidence file.
Preexisting owner edits in CRYPTO_PROTOCOL.md and KEY_MANAGEMENT.md were preserved.

Fresh debug/release installed headers confirm liboqs 0.16.0, SHA3 AVX512VL undefined,
OpenSSL undefined, pthreads enabled and the approved ML-KEM-768/ML-DSA-65 algorithms.
The source pin remains 5a1a854b0dc9f2141bdc771c555ee60c37950183 and static linkage is
unchanged. Existing metadata/size, implicit rejection, empty-context and cleanup
tests passed. Unsafe inventory remains one extern block and ten unsafe call blocks,
all confined to pqc/ffi.rs; zero new project-owned unsafe Rust.

Only this audit report and its separate validation record were added to documentation.
Disposable public fixtures/builds/logs reside under ignored .deps/p2a-astra-reaudit.
No production file, source dependency, prior evidence or matrix row was edited.
All 19 Phase 2 PKI runtime rows remain PLANNED.

## Tests independently executed

All ten requested top-level commands exited zero. Commands and complete outputs
are in the linked validation record. Initial normal target and Cargo home were
new; subsequent normal commands shared that target/home. Adversarial cases used
separate disposable targets. All Cargo operations used local vendored sources and
offline/locked options where applicable; no network access or downloads.

| Command | Actual result |
|---|---|
| cargo fmt --manifest-path trusted-client/Cargo.toml --check | PASS |
| cargo clippy --offline --locked --manifest-path trusted-client/Cargo.toml --all-targets -- -D warnings | PASS |
| cargo test --offline --locked --manifest-path trusted-client/Cargo.toml | 41 unit + 3 doctests passed; 0 failed/ignored |
| cargo test --offline --locked --release --manifest-path trusted-client/Cargo.toml | 41 unit + 3 doctests passed; 0 failed/ignored |
| python3 scripts/check_rng_policy.py | 36 Cargo + 18 rustc/rustdoc rejections; two sensitivity controls passed |
| python3 scripts/validate_phase2a.py | Integrity/features, ENOSYS/EPERM, cleanup sensitivity and policy controls passed |
| python3 scripts/validate_phase1.py | 6 passed, 0 failed, 1 skipped (mmdc unavailable) |
| make build | PASS |
| make check | PASS; reran docs, fmt, lint, unit/doctests, native, source-pin and foundation checks |
| git diff --check | PASS |

make check also ran two build-script tests, four native contracts, four Rust partial-
output cleanup cases and six negative source-pin cases successfully. Native harness
ASan/UBSan coverage retains its documented exclusions; it is not full backend
sanitization. Required-command success does not negate the independently observed
LOW policy failure.

Manual normal reproduction from repository root:

```sh
export CARGO_HOME="$PWD/.deps/cargo-home"
export CARGO_TARGET_DIR="$(mktemp -d /tmp/fp1-p2a-reaudit.XXXXXX)"
make build
make check
cargo test --offline --locked --release --manifest-path trusted-client/Cargo.toml
python3 scripts/check_rng_policy.py
git diff --check
```

## Findings

### CRITICAL

None found in this targeted scope.

### HIGH

None found in this targeted scope.

### MEDIUM

No remaining MEDIUM backend-substitution defect demonstrated. The original
P2A-A-M01 attack is blocked by actual check/build tests and defense in depth.

### LOW

**P2A-RA-L01 — Per-crate rustc arguments evade the blanket response-file policy.**

Affected: trusted-client/build.rs lines 110-118 (only Cargo-wide flag/cfg visibility),
.cargo/config.toml lines 10-17 (actual-argv launcher protects rustdoc only), and
scripts/check_rng_policy.py controls (no cargo rustc extra-argument cases).

Scenario: a developer/build configuration supplies an @file after cargo rustc's
`--`; the file adds a second approved cfg. The build script sees only the approved
Cargo-wide cfg. Rust cfg membership loses the duplicate, so the root guard passes.
The actual project library builds successfully. Equivalent direct duplicate
arguments also succeed. Build-argument control is required; no attacker-controlled
runtime object or secret is needed.

Consequence: the explicitly selected fail-closed response-file and duplicate-option
policy is incomplete, and the corrective documentation overstates its coverage.
The approved RNG backend did NOT change; hidden CUSTOM via the same route fails.
Severity is LOW, not a new MEDIUM randomness substitution.

Reproduction (public fixture, repository root):

```sh
export CARGO_HOME="$PWD/.deps/cargo-home"
export CARGO_TARGET_DIR="$(mktemp -d /tmp/fp1-rng-policy-gap.XXXXXX)"
response_file=$(mktemp /tmp/fp1-rng-duplicate.XXXXXX)
printf '%s\n' '--cfg=getrandom_backend="linux_getrandom"' > "$response_file"
cargo rustc --offline --locked --manifest-path trusted-client/Cargo.toml -- "@$response_file"
rm -- "$response_file"
```

Actual: exit 0 and libcadp_pqc.rlib produced. Required policy result: rejection.
The audit captured a concrete artifact SHA-256 and verbose compiler invocation.

Recommended correction: enforce the response-file/duplicate-argument policy at the
actual rustc argument boundary, including per-crate extra args, or reject that route
as part of an enforced supported-build entry point. Preserve the independent root
exclusivity guard. Do not add a recursive response-file parser. Merely documenting
an unenforced prohibition or adding another check of build.rs's current environment
would not fix this demonstrated gap.

Required regression: real full-crate cargo rustc commands with @duplicate, @custom,
@alternate, nested @file and literal duplicate arguments, from fresh and reused
targets, must reject; a normal approved invocation must still pass. Assert no new
accepted project artifact from rejected fresh builds. Include the equivalent
rustdoc route to preserve its working protection. No correction was implemented
in this audit.

### INFORMATIONAL

Cargo precedence removes/ignores lower-precedence RUSTFLAGS when encoded flags are
supplied. An ignored CUSTOM string is not an active custom backend. The separate
dependency build-cache experiment rebuilt the backend correctly before a normal
project build. Neither is an additional finding.

## Residual limitations

Trusted local source, toolchain, Cargo/rustc/rustdoc, Python and PATH assumptions
remain. Policy guards do not authenticate or sandbox those tools. Backend lists
must be revisited on a future pinned getrandom upgrade. No TSan, successful
LeakSanitizer, full-backend sanitizer coverage, AVX-512 hardware execution, true
fuzzing or physical-erasure guarantee is claimed. Existing M-01/L-19 fatal upstream
failure and offline time/rollback limitations remain open and unchanged.

## P2A-A-M01 status

Original MEDIUM exploit: FIX VERIFIED. Blanket response-file policy closure:
INCOMPLETE, tracked as LOW P2A-RA-L01. Do not mark the entire corrective acceptance
criteria satisfied until that small build-policy gap has been corrected and retested.

## Phase 2A gate recommendation

Approve the verified backend-exclusivity correction with the one required LOW
build-policy follow-up above. No broad redesign or repeat cryptographic audit is
required. Phase 2B and Phase 3 remain unapproved; stop for explicit direction.

PHASE 2A SECURITY RE-AUDIT: APPROVED WITH REQUIRED FIXES
