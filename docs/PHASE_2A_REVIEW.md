# Phase 2A — Deterministic CBOR and Security-Utility Foundation

Status: IMPLEMENTED / TESTED, ready for independent review; full Phase 2 NOT COMPLETE.
User authorized this foundation only. Phase 2B and Phase 3 were not implemented.
This separate adversarial self-review is not an independent Astra/organizational audit.
Actual command output: [PHASE_2A_VALIDATION.txt](PHASE_2A_VALIDATION.txt).
Historical Phase 0/1 and dependency-gate evidence is preserved unchanged.

## Implementation and dependencies

`cadp-pqc` retains the exact approved native backend and adds `foundation` utilities.
The ten D-26 pins/features are integrated in Cargo.toml/Cargo.lock, with default
features disabled. rpassword has no defined default features, so disabling its empty
default changes no resolved features. Rust minimum 1.89; actual Fedora Rust/Cargo
1.98.1. Full registry closure 34 / Linux 31, unchanged from approved owner lock apart
from the project root. There are no new build dependencies or proc macros.

Repository .cargo/config.toml exclusively replaces crates.io with ignored local
.deps/phase2-vendor, sets offline mode and dependency-wide Linux getrandom cfg.
Build guard rejects missing/alternate/conflicting/duplicate backend options. A crate
compile guard provides defense in depth. Rustdoc needs the matching rustdocflags
because it recompiles crate documentation independently. No automatic source download,
random-device fallback, host installation or liboqs configuration change.
`make check` includes vendor/feature/RNG/build/cleanup controls.

AES-GCM/Argon2/Blake2/rpassword are integrated dependencies, not implemented storage
or password workflows. Argon2 benchmarking/workspace cleanup remains later scope.
No certificate, CA issuance, PoP state machine, CRL/revocation or Phase 3 implementation.
Q-02 and Q-03 normative documents are byte-identical to this task's start.

## CBOR architecture and acceptance boundary

Private `cbor` module wraps minicbor's primitive codec. No raw Decoder escapes;
there is no recursive generic Value tree, derive serialization or unknown-field skip.
Structural preflight scans borrowed input with O(depth) stack, checks container
counts against aggregate budget before looping, rejects forbidden types, verifies
encoded-text-key order/duplicates, consumes all bytes and allocates no input tree.

Acceptance: bounded structural preflight -> borrowed typed Decode -> exact schema
map count/seen-field/type validation -> deterministic typed Encode -> whole original
byte comparison. Only `validated` returns an accepted typed value. The preflight
function alone is private and never a cryptographic acceptance result. Overwide
integers/container/string lengths are rejected by exact re-encoding comparison;
noncanonical signed input is not normalized and accepted. Duplicate alternate key
encodings cannot overwrite fields; the exact-schema seen set also rejects them.

Allowed types are unsigned U64, definite byte/text strings, arrays and text-key maps.
Negative integers, floats, tags, indefinite/break forms and simple/bool/null values
are outside the approved Phase 2 subset and reject. UTF-8 is checked by minicbor.
Field-specific exact algorithms/key lengths/ASCII/policy validation belongs to future
protocol types; Phase 2A tests a synthetic exact schema, not a production certificate.

Writer sorts bytewise lexicographic encoded keys, emits shortest forms through
minicbor, rejects duplicate schema keys and bounds its fallible Vec reservations.
Growth is geometric within byte limit, avoiding one exact reallocation per token.
Trusted schema key lists are fixed-size, capped at 64 fields/128 bytes per key; they
are not attacker-supplied recursive structures. Caller-implemented typed encoders
must remain bounded/simple. No unlimited generic serializer API is exported.

Limits: depth <=8; small certificate/PoP class <=16 KiB/256 items; bundle class <=32
KiB/256; CRL class <=16 MiB/1,000,000 items and array ceiling100,000. Root/container/
key/value each counts. Embedded canonical byte objects additionally inherit the
same remaining item budget and logical nesting depth, never a fresh allowance.
Checked count multiplication/conversion/addition precedes loops/reservations.
These are foundation limits, not completed protocol parsers or a CRL implementation.

## Identifiers and fixed domains

Public KemKeyId and SignatureKeyId are distinct fixed32 types. They hash unchanged
D([exact role label, public-key byte string]) using SHA-256, accepting only 1184-byte
KEM or 1952-byte signing public keys. No algorithm-selection argument. Public zero-byte
fixtures (not generated keys) produce these reproducible fingerprints:

- KEM: `1e41ffea04d255bc186b21eefa9039f6b92347bc005433f6cc11f552c9efbd6d`
- Signature: `7b2447cb726e5a16083ba09e8502cb9761ac6c2027b995024af4849c73462bf7`

They were independently checked with Python hashlib over the explicit public CBOR
fixture and locked in Rust tests. Length checks are not a claim that arbitrary
1184/1952-byte material is a valid generated public key.

Internal Separated encodes D([fixed Domain label, typed body]); labels are the exact
certificate/CRL/PoP/key-ID constants. Callers cannot supply an arbitrary label string.
RandomId generates B[32] through getrandom::fill only, discards partial failures and
returns a fixed typed error. No timestamp, counter, UUID or fallback. RandomId carries
public identifier material, so Copy/Debug is appropriate; it is not a secret owner.

## Crypto utilities and secret lifetime

SHA-256 returns32. HKDF explicitly extracts then expands32 with caller-supplied
protocol salt/info; the exposed PRK is explicitly zeroized before expansion.
Output lives in a private non-Clone heap Secret32 allocated fallibly before secrets
exist. No buffer growth after derivation. HMAC requires Secret32, returns public Tag32
without an equality trait, and verification uses full-length library verify_slice.
It rejects any non32-byte tag. No manual primitive or public raw-secret getter.

Secret32 has redacted Debug, no Display/Clone/Deref, crate-internal borrows only,
and zeroize on normal Drop before Box free. A cfg(test)-only observation callback
reads live wiped memory immediately before free; it cannot access freed memory and
adds no production hook. Omitting ONLY the wipe in a disposable source copy makes
that persistent test fail. Audited PQC secret types/FFI remain unchanged.

Dependency cleanup features cover owned hash/MAC/AES/GHASH state to the extent
already assessed at the gate. HKDF's internal expand output/previous-block copies
are not all wiped; project cannot promise complete library/register/stack cleanup.
No physical-erasure, swap/core defense or cleanup after fatal termination claim.
M-01/L-19 remains open/accepted, without abort interception or entropy fallback.

## Tests and evidence

Debug and release each: 41 unit tests (18 new foundation +23 PQC), 3 compile-fail
PQC doctests; 0 failures/ignored. Fresh-process concurrency:1 passed,40 filtered.
Build-script harness:2 tests (original SHA3 guard and new RNG flag guard) passed.
Native shim contracts:4 groups, Clang ASan/UBSan;2 forbidden SHA3 compilations
rejected;4 persistent partial-output Rust cleanup modes passed. Source pin:6 expected
rejection cases passed. Foundation controls: integrity/features,2 real syscall faults,
3 invalid Cargo configurations and1 omitted-Drop-wipe negative control passed.
Documentation/scope:6 passed,1 Mermaid skip (mmdc absent). Formatting, strict Clippy,
Make entry points and whitespace checks passed. Full outputs/counts in validation.

RFC4231 case1 HMAC vector uses its equivalent zero-extended32-byte key in the typed
owner: HMAC pads any <=64-byte key with zeros, so the public20-byte key plus12 zeros
has the same padded key. RFC5869 case1 checks the helper's direct32-byte output;
SHA-256 uses abc. Public fixture comparisons in tests are not authentication APIs.
Deterministic loops exercise256 typed round trips/byte mutations and1280 malformed
inputs, plus transition/boundary/depth/item/embedded and large CRL structural probes.
No downloaded fuzz/property dependency; true fuzzing remains unperformed.

Initial failures are recorded honestly: HMAC fixture typo, test-only Clippy warnings,
rustdoc lacking cfg, and an overly strict negative-control expectation when getrandom
rejected conflicting cfgs before the root guard. Fixed and rechecked; no backend patch.
Final builds started from fresh temporary native/target state with an initially empty
Cargo home. Later corrective reruns reuse that task-owned target; no claim every
repeated invocation started empty. All sources came from vendor, not a cache/download.

## Separate security self-review

Re-read production additions/build diff and tests after implementation. Reviewed:
alternate CBOR encodings, duplicates/unknowns/trailing, allocation/count/depth limits,
domain/role confusion, HMAC comparison, RNG cfg overrides/failure, secret ownership,
errors and future misuse. Verified unchanged native CMake options/source pin and
one extern block/ten unsafe call blocks in ffi.rs. New unsafe Rust: ZERO.
Production additions contain no unwrap/expect/panic/assert/TODO. Such operations
in test fixtures are assertion failures on public fixtures, not external-input paths.

| Finding | Severity / disposition | Regression / evidence |
|---|---|---|
| P2A-R01 | MEDIUM, FIXED before exit: exact per-token reserve could cause quadratic CRL re-encoding | Geometric capped growth; P2A-CBOR-010 capacity/growth/overflow boundary |
| P2A-R02 | LOW, FIXED: rustdoc lacks dependency-wide rustflags | Matching rustdocflags; actual three doctests now pass, guard retained |
| P2A-R03 | LOW, FIXED: native harness assumed dependency-free Cargo JSON and rustc search path | Filter project package_id, add dependency search path; all native/cleanup checks pass |
| P2A-R04 | LOW, FIXED: HMAC helper initially accepted an untyped key slice | Private Secret32 key and full Tag32; P2A-MAC-001 |
| P2A-I01 | INFORMATIONAL / retained | Future protocol schemas/embedded-byte choices require their own review; this is only infrastructure |
| P2A-I02 | INFORMATIONAL / retained | Scoped dead_code allowances keep unused internal future helpers private; no permissive API export to satisfy lint |

No unresolved CRITICAL/HIGH Phase 2A finding identified by self-review. This is
not security certification or independent review approval. Existing backend/copy/
clock/rollback/deployment/supply-chain limitations remain.

## Files and preservation

Created: .cargo/config.toml; trusted-client/src/foundation/{mod,error,cbor,crypto,
identifiers,secret,tests}.rs; scripts/validate_phase2a.py; this review/validation.
Modified: trusted-client/{Cargo.toml,Cargo.lock,build.rs,src/lib.rs,README.md};
scripts/{check_pqc_native,validate_phase1}.py; Makefile;
docs/{ENVIRONMENT,DECISIONS,KNOWN_LIMITATIONS,TEST_MATRIX}.md.
The scope validator explicitly admits ONLY these approved foundation files alongside
PQC; other components remain scaffolds. All19 PKI rows are byte-identical/PLANNED.
AGENTS, specification/plan, protocol/key-management bytes, native/PQC sources,
Phase0/1 history and previous Phase2 dependency evidence are preserved from start.
Pre-existing owner modifications/history files are not part of this task's changes.

## Manual verification and demonstration

Run from repository root, with approved ignored source/vendor bundle present:

```sh
export CARGO_HOME="$PWD/.deps/cargo-home"
cargo fmt --manifest-path trusted-client/Cargo.toml --check
cargo clippy --offline --locked --manifest-path trusted-client/Cargo.toml --all-targets -- -D warnings
cargo test --offline --locked --manifest-path trusted-client/Cargo.toml
cargo test --offline --locked --release --manifest-path trusted-client/Cargo.toml
cargo test --offline --locked --manifest-path trusted-client/Cargo.toml pqc_conc_001 -- --test-threads=1
python3 scripts/validate_phase2a.py
python3 scripts/check_pqc_native.py
python3 scripts/check_pqc_source_pin.py
make build
make check
git diff --check
```

Fresh-target reproduction without deleting any existing build:

```sh
p2a_target=$(mktemp -d /tmp/fp1-p2a-manual.XXXXXX)
export CARGO_TARGET_DIR="$p2a_target"
cargo test --offline --locked --manifest-path trusted-client/Cargo.toml
cargo test --offline --locked --release --manifest-path trusted-client/Cargo.toml
```

Demonstrate `cargo test ... p2a_` and foundation-check control PASS lines using only
public fixtures. No production identities or persistent secret files are generated.
Show the unchanged backend macro evidence and PQC tests. All original19 PKI rows
remain PLANNED;23 new foundation/control IDs have actual evidence. Propose review,
then an explicitly scoped Phase2B decision; do not begin it automatically.


## Independent review and required correction — 2026-10-12

Independent Astra returned APPROVED WITH REQUIRED FIXES, with one required MEDIUM
P2A-A-M01: response-file RNG cfg bypass. The earlier literal-only override assurance
above describes the original tests, not universal flag-route protection. Do not
interpret this self-review as independent approval. Astra's original report and
validation are unchanged.

Authorized correction: reject @response files, validate effective exclusive cfg,
independently compile-reject every pinned recognized alternate, and guard actual
rustdoc argv through the required launcher. New P2A-BUILD-003/004 complement existing
IDs and preserved tests. Status: FIXED / READY FOR TARGETED ASTRA RE-AUDIT, not yet
independently approved. Scope, architecture, runtime crypto/CBOR and Phase 1 boundary
are preserved; all 19 PKI tests PLANNED. Separate assessment and actual evidence:
[PHASE_2A_CORRECTIONS_REVIEW.md](PHASE_2A_CORRECTIONS_REVIEW.md),
[PHASE_2A_CORRECTIONS_VALIDATION.txt](PHASE_2A_CORRECTIONS_VALIDATION.txt).


## Final narrow P2A-RA-L01 correction — 2026-10-12

Astra's targeted re-audit verified backend exclusivity but retained a LOW per-crate
response-file/duplicate-argument policy gap. The historical independent finding and
verdict remain unchanged. D-31 adds only Cargo actual-rustc-argv enforcement, required
wrapper identity and P2A-BUILD-005 controls. Cryptography/CBOR/Phase 1 are unchanged.
Status: FIXED / READY FOR FINAL TARGETED ASTRA VERIFICATION; no independent approval
or Phase 2B/3 authorization. Evidence and corrective assessment:
[PHASE_2A_L01_CORRECTIONS_REVIEW.md](PHASE_2A_L01_CORRECTIONS_REVIEW.md),
[PHASE_2A_L01_CORRECTIONS_VALIDATION.txt](PHASE_2A_L01_CORRECTIONS_VALIDATION.txt).
