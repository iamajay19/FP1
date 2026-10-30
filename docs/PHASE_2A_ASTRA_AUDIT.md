# Independent Phase 2A Astra Security Audit

Initial audit: 2026-10-08; evidence finalized: 2026-10-12.
Scope: Phase 2A foundations and regressions affecting Phase 1.
Reviewed implementation commit: `252eb63`; pre-Phase-2A baseline: `93b4671`.
This is an independent inspection and rerun, not adoption of the self-review.
Evidence: [PHASE_2A_ASTRA_VALIDATION.txt](PHASE_2A_ASTRA_VALIDATION.txt).

Production files, dependencies, protocol and historical evidence were not edited.
Pre-existing uncommitted changes to CRYPTO_PROTOCOL.md and KEY_MANAGEMENT.md, and
untracked conversation-history files, were left intact. Tests use ephemeral or
public synthetic data. No Phase 2B/3 implementation was performed.

## CBOR canonicality

The selected profile is RFC 8949 section 4.2.1 **core deterministic bytewise
ordering**, not section 4.2.3's alternative length-first ordering. The scanner
compares entire encoded key slices; the writer sorts shortest encoded keys with
the same bytewise comparator. This is correct. The project's keys are exact text
schema fields: integer/byte-string keys are rejected rather than given alternate
ordering semantics.

An independent fixture orders `a`, `b`, `z`, `aa`. Rust string ordering would put
`aa` before `z`, which is incorrect here. Equal-length keys use encoded bytes;
text payload prefixes such as `a`/`aa` differ in their CBOR length header. Complete
canonical CBOR items cannot be strict encoded prefixes of one another. A generic
core/length-first distinction is integer 256 (`19 01 00`) versus -1 (`20`): core
puts 256 first, length-first puts -1 first. Both are rejected as project map keys.
There is no legitimate requirement to replace this comparator with length-first.

Acceptance follows bounded allocation-free structural scanning, typed exact-schema
decode, deterministic encoding and exact whole-byte comparison. Non-shortest
integers/lengths cannot be normalized into acceptance. Forbidden indefinite forms,
floats, tags, simple/negative values, duplicate/unknown fields, bad ordering,
truncation and trailing bytes reject. Typed schema implementations remain trusted
project code, not an attacker-supplied generic value tree. Actual certificate and
bundle schemas do not exist yet and require their own subsequent review.

## CBOR bounds/allocation safety

The scanner checks byte limits before scanning and charges aggregate items before
recursion. Declared container lengths and checked map-pair multiplication are
validated before loops. minicbor uses checked conversion and slice bounds for
borrowed bytes/text, without allocating declared lengths. Depth is bounded to 8.
Embedded decoding shares the outer item budget and increases depth; embedded bytes
are already physically included in the enclosing byte limit.

Independent tests passed at limit-1, limit, limit+1 for 16 KiB/32 KiB/16 MiB encoded
objects, at 256/257 and 1,000,000/1,000,001 aggregate items, and at embedded depths
8/9. The existing 100,000-entry ceiling and overflow tests also passed. The
encoder checks addition, caps geometric reservation requests and uses fallible
reservation before growth. Allocator bookkeeping/rounding is not a physical-memory
cap. Trusted encoders must remain bounded; item/depth postvalidation does not make
an arbitrary recursive Encode implementation safe. No reachable input-driven
allocation/overflow defect was found in the current foundation.

## RNG enforcement

The normal configuration applies `getrandom_backend="linux_getrandom"` to dependency
and project compilation, tests and release builds, with matching rustdoc flags.
Fresh verbose compilation independently confirms getrandom receives the cfg.
Missing/alternate ordinary and encoded Rust flags, duplicate literal cfgs,
target-specific alternate settings and command-line conflicting config reject.
Whitespace variants are conservatively rejected. Real production RandomId returns
the typed error under forced ENOSYS and EPERM; no file fallback occurs.

However, rustc response files bypass the literal-token validator. **P2A-A-M01 is
open and required before Phase 2B.** The default configuration is correct; the claim
that conflicting/alternate build settings always fail closed is not correct.

Rustdoc with missing/alternate cfg rejects. Rustdoc with both approved and custom
cfg succeeds: its guard checks presence, not exclusivity. This does not recompile
or redirect the already-approved getrandom dependency and is not a separate runtime
RNG finding; it is a boundary on the claimed flag-validation coverage.

## Key-ID verification

Distinct fixed 32-byte KemKeyId/SignatureKeyId types retain the exact public-key
lengths 1184/1952 and exact domain labels. Encoding is an array of text label and
byte-string key. Python hashlib over independently hand-constructed CBOR reproduced
both public all-zero-key fixtures:

- KEM: `1e41ffea04d255bc186b21eefa9039f6b92347bc005433f6cc11f552c9efbd6d`
- Signature: `7b2447cb726e5a16083ba09e8502cb9761ac6c2027b995024af4849c73462bf7`

Public key length checks do not claim to validate mathematical key material.

## SHA/HKDF/HMAC review

SHA-256 returns 32 bytes. HKDF uses SHA-256 extract and expand with fixed 32-byte
output, explicit salt/info and explicit returned-PRK wiping. HMAC takes Secret32,
returns a full 32-byte tag and uses library verify_slice. The dependency rejects
length mismatch and uses ct_eq for the authentication comparison. No ordinary
tag equality or alternate algorithm exists in production. Rust RFC 4231/5869 tests
passed; independent Python hashlib/hmac computations reproduced their expected
outputs and the SHA-256 abc vector. The RFC 4231 padded 32-byte key is equivalent
to its 20-byte test key under HMAC's standard block padding.

## Secret-lifetime review

Secret32 allocates its fixed heap buffer before sensitive material exists, does not
reallocate after filling, has no Clone/Display/Deref/public getter, and redacts
Debug. Normal Drop wipes the buffer before Box frees it. The live-memory observer
is cfg(test)-only; the omitted-wipe negative control actually fails. Partial-output
Phase 1 ownership/Drop/cleanse/free regressions still pass.

Dependency cleanup is partial: SHA-256 core state and enabled digest block buffers
are wiped on Drop, including corresponding owned HMAC state and its clones.
HKDF expand's raw output/previous-block arrays are not universally wiped. HMAC's
padded-key temporary in hmac/src/block_api.rs and raw intermediate digest arrays
also lack an explicit local wipe; the zeroize feature is not a guarantee that all
key-derived temporaries disappear. Caller-owned HKDF input is borrowed and must
retain its own cleanup owner. Moves, compiler/register/stack copies, swap/dumps
and fatal exits remain outside these normal-Drop guarantees. This refines the
already-recorded dependency-copy limitation; no physical erasure is claimed.

## Error/panic review

No new production unsafe, unwrap, expect, panic, assert, unreachable, todo or
unimplemented invocation was found in Phase 2A runtime paths. Matches in the
foundation are test assertions. Build-script exit is an intentional build rejection,
not an application abort. Production indexing has bounded internal invariants;
errors contain fixed messages without secret data. Dependency HKDF expect paths
use invariant PRK/output sizes and are not reachable failures with this fixed
SHA-256/32-byte wrapper. Upstream fatal liboqs errors remain accepted M-01/L-19.

## Unsafe-boundary verification

Phase 2A production adds zero project-owned unsafe Rust. The Phase 1 boundary is
byte-for-byte unchanged: one extern block and ten unsafe call blocks in ffi.rs.
Third-party minicbor unsafe remains dependency scope. The isolated audit-only
custom-symbol fixture uses an unsafe no_mangle attribute, outside production code;
it neither modifies production code nor dereferences its raw pointer argument.

## Phase 1 regression review

Baseline comparison shows no changes to src/pqc, native shim, native fixtures or
the source-pin test script. The build's only substantive new policy is the RNG
guard. Fresh generated/installed liboqs headers agree: 0.16.0, pinned commit
`5a1a854b0dc9f2141bdc771c555ee60c37950183`, SHA3 AVX512VL undefined, OpenSSL undefined,
pthreads enabled, only ML-KEM-768/ML-DSA-65 application algorithms enabled. Checked
static archives contain no AVX512 members or SHA3/Keccak AVX512 definitions.
Static linkage, metadata sizes, pure/empty-context ML-DSA, implicit rejection and
concurrency/cleanup regressions pass. No AVX512 hardware execution is claimed.

## Dependency/build review

Exact ten pins and requested feature sets match the approved graph: 34 locked
registry packages, 31 Linux-resolved packages. Integrity checks verified 1,435
Cargo-listed files and 1,469 owner hashes, exact sets and no vendor symlinks.
An explicitly checked empty Cargo home and fresh target built offline/locked.
MSRV 1.89 is justified by approved aes 0.9.3. The relative vendor path has no
host-specific absolute path. Commands must run from the checkout or otherwise
supply its Cargo configuration. Offline settings are reproducibility controls,
not a network sandbox or authenticity proof. Toolchain/build environment remains
trusted; this is not a complete third-party or current-advisory audit.

## Test-quality assessment

All 23 Phase 2A IDs were traced to their implementations/runners. Positive and
negative canonical tests are meaningful; exact re-encoding has a deliberately
defective-schema negative control. The 256 round trips/mutations and 1,280 small
malformed inputs are deterministic coverage, not coverage-guided fuzzing. The
original large CRL test uses 900,001 items with a reduced-budget rejection; this
audit additionally exercised the actual one-million boundary. Live Drop observation
and its negative control establish sensitivity without reading freed memory.

The required correction exposes a coverage gap in P2A-BUILD-001/002: literal-token
tests and three environment controls do not cover rustc response-file expansion.
Their historical successful executions remain true; their broad assurance claim
must not be interpreted as universal override rejection. Promote the audit's
additional boundary/order cases to persistent tests when convenient.

## Tests independently executed

All required top-level commands exited 0 with a fresh audit target:

- cargo fmt --manifest-path trusted-client/Cargo.toml --check
- cargo clippy --offline --locked --manifest-path trusted-client/Cargo.toml --all-targets -- -D warnings
- cargo test --offline --locked --manifest-path trusted-client/Cargo.toml
- cargo test --offline --locked --release --manifest-path trusted-client/Cargo.toml
- cargo test --offline --locked --manifest-path trusted-client/Cargo.toml pqc_conc_001 -- --test-threads=1
- python3 scripts/validate_phase2a.py
- python3 scripts/check_pqc_native.py
- python3 scripts/check_pqc_source_pin.py
- python3 scripts/validate_phase1.py
- make build
- make check
- git diff --check

Debug and release: each 41 unit tests and 3 compile-fail doctests passed, zero
failed/ignored. Fresh-process concurrency: 1 passed, 40 filtered. Native checks:
2 build-script tests, 4 native contract groups, 2 expected forbidden-header compile
rejections, 4 Rust partial-output cleanup paths. Source pin: 6 expected rejections.
Foundation controls: vendor/features, 2 syscall faults, 3 ordinary override
rejections, 1 expected omitted-wipe failure. Documentation: 6 passed, 1 Mermaid
skip because mmdc is absent. make check repeats applicable checks, not new IDs.

Additional audit tests: 4 CBOR tests passed after correcting an audit-fixture-only
compile mistake (minimal minicbor does not implement Write for Vec). Seven ordinary
RNG overrides reject; three response-file cases unexpectedly succeed. A linked
external fixture confirms actual custom-backend dispatch. Rustdoc missing/alternate
cfg reject; conflicting cfg succeeds without redirecting compiled dependencies.
Independent SHA/HMAC/HKDF/key-ID vectors match. Initial fixture failure and final
results are distinguished in evidence, not presented as project failures.

## Findings

### CRITICAL

None identified in scope.

### HIGH

None identified in scope.

### MEDIUM

**P2A-A-M01 — response files bypass exclusive Linux RNG configuration. OPEN.**

- Affected: trusted-client/build.rs:11-25, 100-108; presence-only defense at
  trusted-client/src/lib.rs:3-5; missing cases in scripts/validate_phase2a.py:115-134.
- Problem: rng_flags_valid inspects literal unit-separated arguments. A token
  `@/path/to/flags.rsp` contains no getrandom_backend text and is ignored. rustc
  expands it into additional cfg flags. An approved literal cfg keeps both the
  build validator and the root presence check satisfied.
- Reproduction: a response file containing `--cfg=getrandom_backend="custom"`,
  together with literal `--cfg=getrandom_backend="linux_getrandom"`, passes both
  offline/locked cargo check and cargo build on the unchanged repository. The
  getrandom 0.4.3 cfg_if selects custom before linux_getrandom. An external audit
  symbol fixture linked to that rlib confirms RandomId calls custom, not Linux.
  Hidden duplicate and unsupported settings also pass instead of rejecting.
- Reachability/attacker requirement: build flag/response-file control is required;
  untrusted CBOR or a document alone cannot trigger this. A final custom-backend
  executable additionally needs the custom external symbol; without it the link
  fails. No claim of an automatic default-build RNG downgrade is made.
- Consequence: an unapproved backend can be built despite the mandatory exclusive
  syscall policy. An embedding environment supplying custom randomness can determine
  identifier entropy. Current checked-in default builds still use Linux correctly.
- Recommended correction: fail closed on response-file arguments unless their
  expansion and cache dependencies are rigorously validated; rejecting them is the
  smallest option. Also validate Cargo's effective getrandom backend cfg is exactly
  the approved value. Do not add a custom RNG or weaken the selected policy.
- Required regression: hidden custom/duplicate/conflicting cfgs via response files
  must make ordinary check and build fail; cover nested response files and encoded
  Rust flags. Preserve the successful approved configuration and ENOSYS/EPERM tests.
  Keep compiler-wrapper/toolchain trust outside the claimed protection boundary.

### LOW

No additional required security correction identified.

### INFORMATIONAL

- Dependency key-derived temporary arrays are not completely wiped; see the precise
  HMAC/HKDF lifetime assessment above. Retain explicit limitations.
- Schema-specific validation and embedded-object choices remain Phase 2B work;
  infrastructure tests do not certify a future certificate/bundle parser.
- Boundary/order coverage is stronger in this audit than in the persistent suite;
  retaining the additional cases would improve regression resistance.
- Rustdoc cfg exclusivity is not enforced independently, although normal flags are
  present and built dependencies remain fixed in the tested override scenario.

## Residual limitations

No TSan, LeakSanitizer, full-backend sanitizer, AVX512 hardware, formal verification
or new fuzz campaign was performed. Native ASan/UBSan covers the shim/test harness,
not all upstream liboqs code. M-01/L-19 process termination bypassing Drop remains
open/accepted; no fallback entropy, interception or backend patch was added. No
trusted offline time, host rollback protection, FIPS module validation or complete
supplier authenticity/advisory assurance is claimed. All 19 Phase 2 PKI runtime
rows remain PLANNED. Q-02/Q-03 and protocol bytes were not changed.

## Required corrections

Close P2A-A-M01 with a small build-policy correction and persistent negative tests,
then rerun the relevant build/RNG controls and request targeted verification. No
cryptographic redesign or Phase 1 boundary change is needed. Do not begin Phase 2B
on the strength of the self-review's successful tests alone.

## Phase 2A gate recommendation

The current normal-build codec/cryptographic foundation passes this audit's scoped
correctness checks, but its mandatory RNG configuration gate has a demonstrated
bypass. Hold Phase 2B authorization until the required correction is verified.

PHASE 2A SECURITY AUDIT: APPROVED WITH REQUIRED FIXES
