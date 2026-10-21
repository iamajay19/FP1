# Final Phase 2A Astra Confirmation

Date: 2026-10-21 (local date); independent tests began 2026-10-13 UTC.
Reviewed commit: `ce67343cff345dd74052b2103c8dc3f725b32d9c`, compared with
`4f265d9cde54cedbb1a78eb4933dad2400927656`. Scope: P2A-FV-L01 and RNG-policy
regressions directly caused by its correction only. This does not repeat the full
Phase 2A audit. No production code was modified. Only this report and
[PHASE_2A_ASTRA_CONFIRMATION_VALIDATION.txt](PHASE_2A_ASTRA_CONFIRMATION_VALIDATION.txt)
were added; public disposable fixtures/logs remain ignored under
.deps/p2a-astra-confirmation. Existing owner edits and historical evidence were preserved.

## Cache-failure reproduction

Using the unmodified corrected checkout, an existing empty target, initially empty
Cargo home and default cache behavior, the formerly problematic command
`cargo clippy --offline --locked --manifest-path trusted-client/Cargo.toml -- @duplicate.rsp`
returned 101 at an actual compilation invocation. The public response file contains
another approved linux_getrandom cfg. Immediately running the ordinary all-targets
strict-Clippy command on the SAME target returned 0. No .rustc_info.json, target
metadata or target directory was removed between calls; caching was not disabled.

The failure-before-correction remains independently documented in the historical
final verification. This confirmation did not patch production back to the old
implementation. The primary corrected closure test is reject -> accept, not an
assertion that the old failure should still occur.

## Same-target recovery sequences

An independently written runner, without importing the production test helpers,
executed all five sequences on one retained target:

| Sequence | Actual exits |
|---|---|
| A: response-file invalid -> valid strict | 101 -> 0 |
| B: valid -> CUSTOM invalid -> valid | 0 -> 101 -> 0 |
| C: response-file invalid -> valid -> CUSTOM invalid -> valid | 101 -> 0 -> 101 -> 0 |
| D: duplicate split literal -> valid | 101 -> 0 |
| E: repeated strict/lint-only invocations | 0 -> 0 -> 0 |

All 14 steps passed their expectations: five actual compilation rejections and nine
valid invocations. After EVERY step the runner saved the compiler-information cache
and verified every cached query result was successful. No cache deletion, target reset
or CARGO_CACHE_RUSTC_INFO bypass occurred. Persistent P2A-BUILD-006 was also rerun.

## Exact query-exception review

The wrapper first rejects literal @arguments and checks the trusted compiler/Clippy
chain. Only exact -V/--version/-vV and the enumerated stdin/___ target-information
argv shapes bypass invocation-specific CLIPPY_ARGS checking. The metadata request
contains file-names, six fixed crate types, sysroot/split-debuginfo/crate-name/cfg
prints and -Wwarnings, with only the reviewed optional single Linux cfg and native
target. Unknown Cargo placeholder-query shapes fail closed even without bad flags.

Independent actual clippy-driver/rustc tests accepted 18 exact handshakes with
otherwise prohibited CLIPPY_ARGS, including native explicit-target and host forms.
The supplied query stdin deliberately contained compile_error; it was not compiled.
Metadata output did not contain CUSTOM. Thirty-five altered/forbidden requests
rejected with the wrapper diagnostic: generic print, source/output/emission additions,
unknown/missing query parts, response files, duplicate cfg and CUSTOM. Actual artifact
calls with an otherwise approved cfg also rejected the bad extra channel.

There is no generic --print or --emit exemption. A direct plain-rustc public fixture
with --print=link-args produced an executable (positive sensitivity control); the
equivalent guarded call without the required cfg rejected and produced no forbidden
artifact. Appending link-args to the exact metadata shape also rejected. The former
P2A-FV-I01 broad-comment observation is addressed by this narrowed policy.

## Artifact-compilation enforcement

CLIPPY_ARGS @/backend markers remain rejected for compilation, including host build
scripts. No query matcher admits source paths, arbitrary output/emit flags, duplicate
cfgs or alternate backends. Existing actual-argv checking, build.rs wrapper identity,
encoded Rust flag validation and root compile-time exclusivity remain unchanged.
Forwarding uses os.execvp with original argv; no shell or response-file reads.

The persistent suite passed 23 final-crate cases in fresh/reused stages, seven
wrapper-override controls, explicit-native-target checking and thirteen Clippy
negative cases with valid recovery after each. Rejected final-crate invocations
leave no new/changed accepted project rlib; the fresh stage also checks rmeta absence.
All eight recognized alternatives remain blocked.

## Negative-test independence

P2A-BUILD-006 and the thirteen existing/expanded Clippy negatives require the actual
policy diagnostic plus an artifact-producing --crate-name invocation in stderr.
A cached -vV/___ metadata failure cannot satisfy this assertion. Successful valid
sentinels follow each independent Clippy negative. My independent runner additionally
checked retained cache contents after every step, rather than relying only on exit
codes or the corrective report. These controls address the previous false-confidence
risk from a chain of cached negative failures.

## Existing RNG protections

Pinned getrandom 0.4.3 independently exposes exactly linux_getrandom plus custom,
linux_raw, rdrand, rndr, efi_rng, windows_legacy, unsupported and extern_impl.
The root guard rejects all eight alternatives. Source/configuration comparison and
actual P2A-BUILD-003/004/005 reruns establish that P2A-A-M01 and P2A-RA-L01 remain
closed: hidden CUSTOM, response-file and duplicate routes still reject. The 36 Cargo
controls and 18 independent rustc/rustdoc cfg controls passed, including the accepted
omitted-exclusivity sensitivity controls. Normal doctests and rustdoc rejection
routes passed. Production RandomId ENOSYS and EPERM injection tests passed with typed
failure and no fallback. No runtime RNG change was made by this correction.

## Preservation check

The complete correction commit changes only the two reviewed Python scripts and
seven documentation/evidence files. Runtime foundation/PQC/native files, dependency
pins/features, Cargo config, build.rs and root compile guard are unchanged from the
pre-correction commit. Q-02/Q-03 and protocol bytes are unchanged by this correction.
The owner's preexisting protocol/key-management edits are preserved, not attributed
to this audit. All 96 start-snapshotted files, including historical Astra evidence,
remain byte-identical during this confirmation.

Unsafe inventory remains one extern block and ten unsafe call blocks, confined to
Phase 1 ffi.rs. All 19 PKI runtime rows remain byte-identical and PLANNED.
liboqs source remains clean at 5a1a854b0dc9f2141bdc771c555ee60c37950183 (0.16.0).
Six fresh debug/release generated/installed configuration headers retain AVX-512 SHA3
and OpenSSL disabled, pthreads enabled and the two required algorithms enabled.
Static linkage and the Phase 1 FFI are unchanged; core primitive tests passed.

## Tests executed

All ten requested commands completed independently with exit 0. Core commands used
an initially empty Cargo home and fresh target, then retained them. The three longer
policy suites had interrupted earlier execution attempts; those partial logs are
explicitly INCOMPLETE, not passes. The standalone policy command completed in
final-policy-4.log; the two aggregate commands completed in verified-5.log and
verified-8.log, each with a fresh outer target. The final two aggregate runs used
GIT_OPTIONAL_LOCKS=0 to avoid optional Git index refresh writes repeatedly invalidating Cargo source-directory tracking. This leaves
source revision/dirty-tree checks active and does not disable or clear Cargo caches.
The separate independent closure sequences and completed standalone policy suite
used the default Git setting. No cache was cleared within any same-target test
sequence. All Cargo builds used the supplied local vendor/source configuration and offline/locked mode.

| Command | Result |
|---|---|
| `cargo fmt --manifest-path trusted-client/Cargo.toml --check` | PASS; exit 0 |
| `cargo clippy --offline --locked --manifest-path trusted-client/Cargo.toml --all-targets -- -D warnings` | PASS; exit 0 |
| `cargo test --offline --locked --manifest-path trusted-client/Cargo.toml` | PASS; exit 0; 41 unit + 3 doctests, 0 failed/ignored |
| `cargo test --offline --locked --manifest-path trusted-client/Cargo.toml --release` | PASS; exit 0; 41 unit + 3 doctests, 0 failed/ignored |
| `python3 scripts/check_rng_policy.py` | PASS; exit 0 |
| `python3 scripts/validate_phase2a.py` | PASS; exit 0 |
| `python3 scripts/validate_phase1.py` | PASS; exit 0; 6 passed, 1 Mermaid-render skip (mmdc unavailable) |
| `make build` | PASS; exit 0 |
| `make check` | PASS; exit 0 |
| `git diff --check` | PASS; exit 0 |

make check also reran native/configuration/partial-output-cleanup and source-pin
controls successfully. No TSan, LeakSanitizer, full-backend sanitizer or AVX-512
hardware execution is claimed. Exact commands, environments, outputs, fixture
sources, cache snapshots and preservation hashes are in the companion evidence.

Manual confirmation from checkout root with approved ignored sources:

```sh
export CARGO_HOME="$PWD/.deps/cargo-home"
export GIT_OPTIONAL_LOCKS=0
python3 scripts/check_rng_policy.py
python3 scripts/validate_phase2a.py
make check
cargo test --offline --locked --release --manifest-path trusted-client/Cargo.toml
git diff --check
```

The policy runner demonstrates P2A-BUILD-006's same-target recovery and the independent
Clippy rejection/recovery pairs without requiring manual cache deletion.

## Findings

### CRITICAL

None found in this targeted scope.

### HIGH

None found in this targeted scope.

### MEDIUM

None found in this targeted scope. No RNG/backend downgrade demonstrated.

### LOW

No remaining LOW correction required. P2A-FV-L01 is CLOSED by independent reproduction
and retained-cache sequence testing; P2A-RA-L01 remains CLOSED.

### INFORMATIONAL

Exact query matching intentionally depends on the reviewed Cargo/rustc 1.98.1,
Clippy 0.1.98 native Linux x86_64 toolchain. New query shapes or Rust flags changing
that handshake can fail closed and require review. This is documented compatibility
scope, not a backend-policy bypass. Compiler/Cargo/Python/PATH/local source remain
trusted; these wrappers do not sandbox malicious build tools. Existing backend
abort/zeroization, third-party-copy, offline-time and host-rollback limits remain.

## P2A-FV-L01 status

**CLOSED / INDEPENDENTLY VERIFIED.** Invalid Clippy no longer poisons subsequent
valid invocations on the same target under the reviewed supported toolchain. The
correction does not weaken actual-compilation RNG/backend policy.

## Phase 2A final gate recommendation

Approve Phase 2A's final security confirmation. The original broad audit's sole
required finding and both narrow follow-up findings are now closed. This is scoped
prototype review, not production certification or organizational security approval.
Phase 2B requires explicit authorization; Phase 2B and Phase 3 were not begun.

PHASE 2A SECURITY FINAL CONFIRMATION: APPROVED
