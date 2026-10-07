# Phase 2 — Dependency Gate Review

Current dependency gate: READY (2026-10-07), after final refreshed-bundle verification.
The final section below supersedes historical BLOCKED status/source-absence reports.
This is an environment/dependency gate report, not a completed Phase 2 security review.
Phase 3 remains unapproved. No Phase 2 cryptographic or storage code was created.

## Authorization and preserved Phase 1 gate

The user explicitly approved Private-Key Storage and Offline Post-Quantum PKI.
The independent [Astra re-audit](PHASE_1_ASTRA_REAUDIT.txt) approved Phase 1 after
H-01/L-01 corrections. Earlier statements that re-audit/Phase 2 approval were pending
are historical. M-01/L-19 remains an accepted backend limitation, not this blocker.
The approved PQ API, liboqs source pin, build configuration, unsafe boundary and
all Phase 0/1 evidence remain unchanged.

The requested authoritative documents and Phase 1 review/correction/re-audit evidence
were read completely before changes. The existing Cargo manifests and safe PQ/secret
API were inspected. No protocol or PoP decision was changed.

## Confirmed dependency blocker

The user's Phase 2 section 12 requires: "If a suitable reviewed AES-GCM Rust
dependency is not available offline: STOP and report the dependency blocker."
The same task requires Argon2id and prohibits custom primitives, cipher substitution
and arbitrary dependency downloads. This inspection found no offline `aes-gcm` or
`argon2` package in either the project Cargo home or the user Cargo registry cache.

Four isolated `cargo generate-lockfile --offline` probes failed with exit 101:
each package against the project cache, and each against a disposable copy of the
user registry cache. Each reported `no matching package named ... found`; none
created a lockfile. Wildcard requirements were used only to test whether any cached
version could resolve; they are not proposed production pins or maintenance approval.
The user cache was copied into /tmp to avoid modifying a home outside the workspace.
Only registry files were copied, not credentials. Temporary probes/cache copy were
removed. No package was downloaded, compiled or executed by these probes.

Searches of the repository, /tmp, /opt, /var/cache and /usr/share/cargo found no
alternative Cargo vendor bundle for these dependencies. The user cache contains
terminal-support crates, not the required crypto crates. Availability is scoped to
inspected paths and this execution surface; it is not proof about every host path.
Evidence: [PHASE_2_VALIDATION.txt](PHASE_2_VALIDATION.txt).

## Required recovery

Supply a complete offline Cargo source/cache or vendor bundle containing reviewed,
exactly pinned implementations of AES-256-GCM and Argon2id v0x13, with all transitive
dependencies, checksums and license metadata. SHA-256, PoP authentication and protected
terminal input dependencies also need offline availability and review before selection.
The repository does not yet approve particular Phase 2 crate versions. Source and
maintenance review, exact locking and an actual offline build must precede adoption.
Do not interpret an availability-only wildcard probe as approval of an old version.

No fallback to native FFI, subprocess key handling, custom crypto or another cipher
was attempted. No new unsafe code or secret export was added to bypass this gate.
Phase 2 authorization remains recorded; the dependency bundle, not another phase
authorization, is needed to resume within the approved scope.

## Results and limitations

No recipient or CA keys, passwords, bundle, certificate, PoP transcript or CRL exists
from this task. No Argon2 benchmark ran. All 19 existing Phase 2 runtime IDs remain
PLANNED, with no pass claimed. No Phase 2 security exit gate is satisfied.
Documentation/scope checks and whitespace checks are recorded separately from
dependency failures; historical Phase 1 test results are not presented as fresh runs.

## Manual dependency verification

From the repository root, this checks availability without downloading packages or
modifying the production manifest. Each probe currently exits 101. The shell continues
to check both packages and removes only its own disposable directory.

```sh
export CARGO_HOME="$PWD/.deps/cargo-home"
phase2_probe=$(mktemp -d /tmp/fp1-phase2-dependency-check.XXXXXX)
trap 'rm -rf -- "$phase2_probe"' EXIT
for dependency in aes-gcm argon2; do
    mkdir -p "$phase2_probe/$dependency/src"
    printf '// Resolution only; no crypto operation.\n' > "$phase2_probe/$dependency/src/lib.rs"
    cat > "$phase2_probe/$dependency/Cargo.toml" <<EOF
[package]
name = "phase2-dependency-probe"
version = "0.0.0"
edition = "2024"
[dependencies]
$dependency = "*"
EOF
    cargo generate-lockfile --offline --manifest-path "$phase2_probe/$dependency/Cargo.toml"
    printf 'dependency=%s exit=%s\n' "$dependency" "$?"
done
python3 scripts/validate_phase1.py
git diff --check
```

Recommendation: PHASE 2 BLOCKED on offline cryptographic dependencies. Stop before
implementation; do not begin Phase 3.

## Offline vendor audit — 2026-10-07 (current dependency gate)

The earlier unavailable-source result above is historical. The host owner supplied
`.deps/phase2-vendor` and four bootstrap/evidence files. This task is dependency-gate
verification only; it does not resume the earlier Phase 2 implementation authorization.
No production manifest, lockfile, API, protocol, liboqs source or implementation was
changed. All 19 Phase 2 runtime tests remain PLANNED. Phase 3 remains unapproved.

**Current result: PHASE 2 DEPENDENCY GATE BLOCKED**, narrowed to the missing reviewed
cleanup dependency. AES-GCM/Argon2/SHA-256/OS RNG/terminal source availability and
vendor-only compilation are established; the original absence of those crates is
no longer the blocker. Their exact versions below are technically suitable candidates
subject to completing the cleanup feature closure, not unconditional production adoption.

### Integrity, offline resolution and exact pins

All 34 registry `(name, version)` pairs agree across the supplied lockfile, metadata,
vendor directory and normalized Cargo manifests. Each package checksum in the
lockfile agrees with `.cargo-checksum.json`; every listed source file was independently
hashed against Cargo's file metadata. Exact file sets also agree with the owner's
SHA-256 inventory: 1,421 Cargo-checksummed files plus 34 checksum metadata files,
1,455 inventory entries total, zero mismatches, zero symlinks. No supplied input was
modified. Supporting file SHA-256 values are recorded in PHASE_2_VALIDATION.txt.

This establishes internal integrity/consistency, not independent registry authenticity.
No original `.crate` archives were provided to recompute package archive checksums.
The owner inventory and vendor checksum files could both be changed by a malicious
supplier. No upstream signature, fresh advisory database, current maintenance status
or exact-release independent security audit was verified; network use was prohibited.
Local repository metadata identifies RustCrypto projects for AES/Argon2/SHA-256 and
rust-random/getrandom and rpassword upstreams. The AES README's historical audit
claim is not evidence that every version in this new dependency graph was audited.

Disposable Cargo projects used an initially empty task-owned CARGO_HOME, explicit
crates.io directory-source replacement, `net.offline=true`, `--offline`, a fresh
isolated target and no registry cache. Cargo/environment wrapper and flag overrides
were removed for these runs; no credentials were copied. The supplied default-feature
graph compiled with the original lockfile and `--locked`; its lockfile remained
byte-for-byte unchanged. Fedora/Linux resolves 31 registry crates (5 direct + 26
transitive); the cross-platform lockfile has 34 (5 direct + 29 transitive).

| Direct package | Exact candidate pin | Minimal functional probe | Proposed storage configuration after missing source is reviewed |
|---|---|---|---|
| aes-gcm | =0.11.1 | defaults off; aes | defaults off; aes, zeroize |
| argon2 | =0.6.0 | defaults off; caller-provided memory | defaults off; zeroize; caller-owned, drop-cleansed memory |
| sha2 | =0.11.0 | defaults off | defaults off; Sha256 for public key IDs |
| getrandom | =0.4.3 | defaults off | defaults off; enforce Linux syscall-only backend |
| rpassword | =7.5.4 | defaults off (no features) | hidden default /dev/tty; immediate secret ownership |

The minimal functional graph independently resolved offline to 30 cross-platform
registry crates / 27 Linux crates (5 direct + 22 Linux transitive). It removes
const-oid, password-hash, phc and rand_core; base64ct remains an unconditional Argon2
dependency. Both supplied and minimal feature graphs have no build-dependency edges
and no proc-macro crates. These counts are not the eventual cleanup-enabled graph.
The highest declared Rust minimum is 1.89 (`aes 0.9.3`), despite aes-gcm itself declaring
1.85. Host rustc/Cargo 1.98.1 works. The Phase 1 crate's 1.85 claim remains unchanged;
future adoption must update the applicable component's declared minimum to at least
1.89 or retest/review another graph. No old crate substitution is approved here.

### AES-256-GCM

`Aes256Gcm` selects AES-256, U12 nonce and default U16 tag: key 32, nonce 12, tag 16
bytes. `AeadInOut` detached operations accept explicit AAD and caller-owned buffers;
no alloc/getrandom/hazmat feature is needed. No AES-GCM-SIV or custom cipher is used.
`decrypt_inout_detached` computes and constant-time compares authentication before
applying the keystream; an authentication error leaves the supplied ciphertext buffer
unchanged in this API. The public error is generic. Phase 2 must still bound object
sizes, enforce nonce freshness and generic user-facing failure before using these APIs.

The isolated probe passed a published all-zero AES-256-GCM fixture, AAD round trip,
wrong-AAD rejection and modified-tag rejection, including unchanged output on failures.
These are dependency API checks, not bundle tests. Fixed public keys/nonces in these
fixtures must never become production nonce policy.

Without `zeroize`, AES expanded key schedules and GHASH secret state have no cleanup
Drop implementation. aes-gcm's zeroize feature forwards to AES and GHASH/Polyval and
cleans a temporary GHASH key. Cleaning only the original KEK cannot clean these copies.
Even that feature is best effort, not a guarantee about registers, stack temporaries,
compiler copies, swapped pages or physical erasure. Cipher types support Clone; future
project owners must not expose or unnecessarily clone them.

### Argon2id

`Argon2::new(Algorithm::Argon2id, Version::V0x13, Params::new(m,t,p,Some(32)))`
and `hash_password_into_with_memory` provide exactly the required mode/version and
raw 32-byte KEK output. PHC/password-hash, automatic salt generation, kdf and Rayon
features are unnecessary. The p parameter describes algorithm lanes; without the
parallel feature those lanes are processed sequentially, not by p OS threads.

Library parameter bounds are much broader than project bounds. The future parser must
validate total bundle size, memory/time/lanes and relationships before Params, workspace
allocation or KDF execution. The protocol's 64..256 MiB, 3..6 passes and 1..4 lanes
remain authoritative. The probe used only a 32 KiB public fixture to test API operation,
repeatability and invalid-salt rejection; it did NOT benchmark or select production
parameters. The 64 MiB / 3 / 4 candidate still requires a real implementation benchmark.

Password/salt/output are borrowed, but password-derived hashes and the entire memory
workspace contain sensitive material. `Block` is Copy/Clone/Debug and has no cleansing
Drop. The `alloc` convenience `hash_password_into` allocates before validating inputs,
and its internal `Blocks::Drop` deallocates without cleansing, EVEN WITH zeroize enabled.
Do not assume enabling zeroize solves workspace cleanup. Prefer fallible caller-owned
bounded `Vec<Block>` with a safe drop-cleanup owner on every return path. zeroize enables
Block::zeroize and selected temporary wipes; library/hash/register copies remain a
limitation. No new unsafe project code is authorized to work around missing sources.

### SHA-256 and OS randomness

`sha2::Sha256`/Digest implements SHA-256 with 32-byte output; the abc known vector
passed with defaults disabled. Public-key ID hashing needs neither alloc nor OID
support. There is no manual SHA implementation in the project. Future secret hash
inputs require their own lifecycle review; no broad zeroization guarantee is made.

`getrandom::fill` returns Result and uses Fedora/Linux OS entropy. On this target its
DEFAULT backend can fall back from getrandom to /dev/urandom (still OS entropy, not a
custom PRNG). To satisfy the user's no-fallback policy the probes explicitly compiled
with `--cfg getrandom_backend="linux_getrandom"`, selecting src/backends/getrandom.rs.
It retries EINTR/short reads and returns other syscall errors, without device fallback.
Future production build configuration must enforce this selection and review overrides;
no such configuration was installed by this task. No rdrand/custom/alternate backend
is approved. On any error callers must reject/discard the whole output, which may have
been partially filled. A task-local LD_PRELOAD fixture forcing libc getrandom to EPERM
confirmed typed failure rather than successful fallback. This is a dependency-only
failure injection, not a production hook or alternative entropy source.

### Protected terminal input

Default `prompt_password`/`read_password` opens /dev/tty and hides input with termios;
no password argv/environment/config is necessary. A real controlling-PTY test verified
ECHO off during public synthetic input, no echo of the sentinel, correct returned value
and restoration of original termios on normal return. Default mode must be retained;
the crate's optional file/data reader configurations are inappropriate production
password sources. The library returns an ordinary String, not an automatically wiped
secret. Future code must transfer its owned allocation promptly into a drop-cleansed
secret owner, never format/log/clone it, and set application input policy.

Internal rtoolbox SafeString wipes its current logical bytes on Drop, but supports
Clone/Debug; growth/deletion/into_inner can leave previous copies or transfer ownership.
Input growth is not bounded by a built-in maximum. Terminal/kernel and UTF-8 temporaries,
allocator reuse, signal/process termination and failed termios restoration are residual
limitations. Normal Drop attempts restoration but cannot report a restoration error;
Ctrl-C may raise SIGINT and bypass cleanup. No password hardening implementation or
signal recovery was introduced here.

### Supply-chain scope and license inventory

All rows below have source `registry+https://github.com/rust-lang/crates.io-index`;
actual acquisition for this audit was ONLY the owner's local vendored directory.
Checksums are lockfile package/archive checksums, not hashes of unpacked directories.
License expressions are exact Cargo metadata declarations, checked against manifests;
this is an inventory for project review, not legal approval or choice of license.
31 crates offer MIT/Apache-2.0 alternatives, two are Apache-2.0 only, and r-efi also
lists LGPL-2.1-or-later as an alternative. Dev/test dependencies in dependency crates'
own lockfiles are not part of this production graph and were not downloaded/executed.

| Package | Version | License expression | Package SHA-256 |
|---|---|---|---|
| aead | 0.6.1 | MIT OR Apache-2.0 | `1973cfbc1a2daf9cf550e74e1f088c28e7f7d8c1e1418fb6c9dc5184b7e84c99` |
| aes | 0.9.3 | MIT OR Apache-2.0 | `35f0f96ce78e38c3dc6d8948aa8163d06385be74000f3c7a95bf1eef35d3ea32` |
| aes-gcm | 0.11.1 | Apache-2.0 OR MIT | `7f2b8006a0c83f52b62ba44a97b58bf76fe2f70a329e588f67f89691d93d498f` |
| argon2 | 0.6.0 | MIT OR Apache-2.0 | `134c52ddac6d63c576bef8168db10c83c49c26444ecbc68060fef078925a901c` |
| base64ct | 1.8.3 | Apache-2.0 OR MIT | `2af50177e190e07a26ab74f8b1efbfe2ef87da2116221318cb1c2e82baf7de06` |
| blake2 | 0.11.0 | MIT OR Apache-2.0 | `5b5d4d889834ee8ecfc0f8426ad30faf7cdcb10f741a8e6d7224d95325479f6f` |
| block-buffer | 0.12.1 | MIT OR Apache-2.0 | `d2f6c7dbe95a6ed67ad9f18e57daf93a2f034c524b99fd2b76d18fdfeb6660aa` |
| cfg-if | 1.0.5 | MIT OR Apache-2.0 | `4e7648175b45a9a48536d676f68d918270699102aa8dab5496df06904c914600` |
| cipher | 0.5.2 | MIT OR Apache-2.0 | `e8cf2a2c93cd704877c0858356ed03480ff301ee950b43f1cbe4573b088bfa6c` |
| cmov | 0.5.4 | Apache-2.0 OR MIT | `0c9ea0ac24bc397ab3c98583a3c9ba74fa56b09a4449bbe172b9b1ddb016027a` |
| const-oid | 0.10.2 | Apache-2.0 OR MIT | `a6ef517f0926dd24a1582492c791b6a4818a4d94e789a334894aa15b0d12f55c` |
| cpubits | 0.1.1 | MIT OR Apache-2.0 | `15b85f9c39137c3a891689859392b1bd49812121d0d61c9caf00d46ed5ce06ae` |
| cpufeatures | 0.3.1 | MIT OR Apache-2.0 | `5ca28b0ae3115b884660db4118d803791fd6756b6e88f39c0f3f7859060d7566` |
| crypto-common | 0.2.2 | MIT OR Apache-2.0 | `ce6e4c961d6cd6c9a86db418387425e8bdeaf05b3c8bc1411e6dca4c252f1453` |
| ctr | 0.10.1 | MIT OR Apache-2.0 | `baaca1c4b237092596f64d571e9db6ce4109c4ef9742e27590f1709594461f21` |
| ctutils | 0.4.3 | Apache-2.0 OR MIT | `03bb0e1cc970d482d121d9a1744999169b69a07470b3d644a7894e53fcaf4574` |
| digest | 0.11.3 | MIT OR Apache-2.0 | `f1dd6dbb5841937940781866fa1281a1ff7bd3bf827091440879f9994983d5c2` |
| getrandom | 0.4.3 | MIT OR Apache-2.0 | `300e883d756b2e4ec94e02791f39b04b522276138852cfc41d9fb7e904106099` |
| ghash | 0.6.0 | Apache-2.0 OR MIT | `2eecf2d5dc9b66b732b97707a0210906b1d30523eb773193ab777c0c84b3e8d5` |
| hybrid-array | 0.4.15 | MIT OR Apache-2.0 | `27f864f10dfb56725ce5ce5472bc52252c8f93a4ab86327122cebf62c5f59a17` |
| inout | 0.2.2 | MIT OR Apache-2.0 | `4250ce6452e92010fdf7268ccc5d14faa80bb12fc741938534c58f16804e03c7` |
| libc | 0.2.190 | MIT OR Apache-2.0 | `ce5d3ddc6d3fa000eb1536d85e147bfe31aacaba692ed6a876f95cb7c855be78` |
| password-hash | 0.6.1 | MIT OR Apache-2.0 | `aab41826031698d6ffcd9cff78ef56ef998e39dc7e5067cdfebe373842d4723b` |
| phc | 0.6.1 | Apache-2.0 OR MIT | `44dc769b75f93afdddd8c7fa12d685292ddeff1e66f7f0f3a234cf1818afe892` |
| polyval | 0.7.3 | Apache-2.0 OR MIT | `f0fa31d631f2b2cb2a544d0aa321ce847a94764d701ca2becc411138b93d49cd` |
| r-efi | 6.0.0 | MIT OR Apache-2.0 OR LGPL-2.1-or-later | `f8dcc9c7d52a811697d2151c701e0d08956f92b0e24136cf4cf27b57a6a0d9bf` |
| rand_core | 0.10.1 | MIT OR Apache-2.0 | `63b8176103e19a2643978565ca18b50549f6101881c443590420e4dc998a3c69` |
| rpassword | 7.5.4 | Apache-2.0 | `2da316a15f47e3d053de9cb2c439650bd8fa4aaeb9365f2e5f27f492ff73c196` |
| rtoolbox | 0.0.6 | Apache-2.0 | `9a1efe12a1469752d0e6ff5ebec0b6ef4924cc5c4c71046b0ec730040535819d` |
| sha2 | 0.11.0 | MIT OR Apache-2.0 | `446ba717509524cb3f22f17ecc096f10f4822d76ab5c0b9822c5f9c284e825f4` |
| typenum | 1.20.1 | MIT OR Apache-2.0 | `b6f5e870be6c3b371b77fe0ee0bafb859fa4964b4404c27de1d380043c4dda20` |
| universal-hash | 0.6.1 | MIT OR Apache-2.0 | `f4987bdc12753382e0bec4a65c50738ffaabc998b9cdd1f952fb5f39b0048a96` |
| windows-link | 0.2.1 | MIT OR Apache-2.0 | `f0805222e57f7521d6a62e36fa9163bc891acd422f971defe97d64e70d0a4fe5` |
| windows-sys | 0.61.2 | MIT OR Apache-2.0 | `ae137229bcbd6cdf0f7b80a31df61766145077ddf49416a728b02cb3921ff3fc` |

Custom build scripts: libc and getrandom only; no registry build dependencies.
getrandom's script sets sanitizer cfg; libc invokes rustc for version/target decisions
(and has platform-specific FreeBSD/emcc queries). No network command or downloaded
native build was identified. No C/C++ compiler, bindgen or proc macro is required by
this Rust graph; libc/rpassword link platform libc and terminal APIs. The existing
Phase 1 native liboqs build is outside this new dependency graph and unchanged.
Windows-only windows-sys/windows-link and UEFI r-efi are locked but not compiled on
Fedora; other target-specific source branches in getrandom/libc/crypto are not tested.

An unsafe-syntax scan found Rust unsafe operations/declarations in: aes, argon2,
base64ct, blake2, block-buffer, cmov, const-oid, cpufeatures, getrandom, hybrid-array,
inout, libc, polyval, r-efi, rpassword, rtoolbox, sha2 and windows-sys (18 crates).
This is audit triage across vendored sources, including non-host/test code, not a full
line-by-line memory-safety review or a count of executed unsafe sites. CPU intrinsics,
assembly/backend selection, buffer casts, allocation and OS FFI warrant dependency
review; unsafe alone is not a rejection criterion. No new project unsafe code exists.

### Remaining gate and design decisions

The cleanup feature probe (`aes-gcm` aes+zeroize; `argon2` zeroize, defaults off)
failed offline with exit 101: no matching package named zeroize. Supply a reviewed,
exact-pinned zeroize 1.x release and its complete intended feature closure, checksums
and license evidence; repeat integrity/offline compilation before storage adoption.
Do not enable derive/proc-macro features without demonstrated need. This is the
specific remaining dependency blocker, not a reason to replace AES or Argon2.

No deterministic-CBOR crate is supplied or approved. Before serialization work,
choose/review either the already-authorized small bounded RFC 8949 subset or a
separately supplied/reviewed library. Absence of a CBOR library alone does not prove
that the approved subset route is impossible. No custom format or serializer-default
canonicalization is approved. Q-02's exact PoP transcript, replay state and required
primitives remain undecided; no HMAC/HKDF crate is present in this locked bundle,
and none was invented or selected. Their dependency requirement depends on the
future documented design. Time/host-rollback assumptions and CA recovery remain
open under existing requirements; no trusted time was fabricated.

No PKI, CA, identity generation, certificate, PoP, bundle, CRL, filesystem rewrite,
Argon2 production benchmark or Phase 2 runtime test was implemented. The vendor bundle
remains ignored, production Cargo files unchanged, and the audit's disposable files
will be removed after evidence is recorded. No fresh Phase 1 crypto checks are claimed.

### Manual offline availability reproduction

This reproduces supplied-graph compilation and the missing-cleanup negative check.
Run from the repository root; each project is disposable. No password/key is used.
The second Cargo command is expected to exit 101 until zeroize is supplied.
Full API probe source and actual command output are in PHASE_2_VALIDATION.txt.

```sh
set -eu
repo_dir=$PWD
dependency_probe=$(mktemp -d /tmp/fp1-phase2-vendor-check.XXXXXX)
trap 'rm -rf -- "$dependency_probe"' EXIT
mkdir -p "$dependency_probe/src" "$dependency_probe/.cargo"
cat > "$dependency_probe/Cargo.toml" <<'EOF'
[package]
name="fp1-phase2-dependency-bootstrap"
version="0.0.0"
edition="2024"
[dependencies]
aes-gcm="=0.11.1"
argon2="=0.6.0"
sha2="=0.11.0"
getrandom="=0.4.3"
rpassword="=7.5.4"
EOF
printf 'fn main() {}\n' > "$dependency_probe/src/main.rs"
cp .deps/phase2-bootstrap-Cargo.lock "$dependency_probe/Cargo.lock"
cat > "$dependency_probe/.cargo/config.toml" <<EOF
[source.crates-io]
replace-with="vendored-sources"
[source.vendored-sources]
directory="$repo_dir/.deps/phase2-vendor"
[net]
offline=true
[build]
rustflags=["--cfg", 'getrandom_backend="linux_getrandom"']
EOF
unset RUSTFLAGS CARGO_ENCODED_RUSTFLAGS RUSTC_WRAPPER RUSTC_WORKSPACE_WRAPPER
(cd "$dependency_probe" && CARGO_HOME="$dependency_probe/cargo-home"  CARGO_TARGET_DIR="$dependency_probe/target" cargo build --offline --locked)
sha256sum -c .deps/phase2-vendor-SHA256SUMS.txt
# In the disposable manifest only, request the missing cleanup features.
python3 - "$dependency_probe/Cargo.toml" <<'PYCODE'
import sys
from pathlib import Path
p = Path(sys.argv[1])
s = p.read_text().replace('aes-gcm="=0.11.1"',
    'aes-gcm={version="=0.11.1",default-features=false,features=["aes","zeroize"]}')
s = s.replace('argon2="=0.6.0"',
    'argon2={version="=0.6.0",default-features=false,features=["zeroize"]}')
p.write_text(s)
PYCODE
(cd "$dependency_probe" && CARGO_HOME="$dependency_probe/cargo-home"  cargo generate-lockfile --offline)
```


## Final pre-implementation gate review — 2026-10-07

Scope: refreshed cleanup graph, library-assisted deterministic CBOR, exact Q-02 PoP
transcript and Q-03 time boundary only. Production code and all 19 Phase 2 runtime
statuses remain unchanged. Earlier inventories/failures/manual examples above are
historical and superseded for the refreshed graph; do not reproduce their obsolete
missing-zeroize expectations. No Phase 2 implementation or Argon2 benchmark occurred.

### Refreshed source and offline build proof (D-22)

The six exact direct candidates now include zeroize 1.9.1. Lock, bootstrap metadata,
manifest versions, vendor directory set, Cargo package/file checksums and the owner's
SHA-256 inventory agree: 31 registry packages across platforms, 1367 source-file
checksums, 1398 inventory files, zero mismatches and zero symlinks. Compared with the
historical graph, const-oid/password-hash/phc/rand_core sources are no longer included;
zeroize is added. Local consistency is not supplier authenticity, archive-signature
verification or a full dependency/advisory audit. No package archives were supplied.

A disposable project copied the refreshed lock byte-for-byte and used only the vendor
replacement with net.offline=true. A newly empty CARGO_HOME and target, sanitized
Cargo/Rust flags/wrapper environment and --offline --locked produced a successful
build: 28 Linux registry crates plus the fixture root. Metadata/features and a public
API probe ran successfully. Only synthetic public fixtures were processed. No password
terminal operation or production key/secret was used. Exact evidence is preserved in
PHASE_2_VALIDATION.txt; disposable directories are removed after recording it.

Minimal supplied direct settings: defaults OFF for all; aes-gcm 0.11.1 features
[aes,zeroize], argon2 0.6.0 [zeroize], sha2 0.11.0 [], getrandom 0.4.3 [], rpassword
7.5.4 [], zeroize 1.9.1 [alloc]. Required Fedora cfg remains linux_getrandom; no
fallback RNG is introduced. The current 31-package graph has no build-dependency
edges or proc-macro targets; prior per-source build-script/unsafe inventory remains
applicable to retained crates. Zeroize introduces reviewed volatile/unsafe cleanup
internals, not a new project unsafe boundary; no derive macro is enabled.

### AES/GHASH cleanup source assessment

Local aes 0.9.3 src/lib.rs common_impls Drop uses zeroize_flat_type for the owned
expanded-key state when zeroize is selected and implements ZeroizeOnDrop. aes-gcm
0.11.1 forwards zeroize to aes and ghash and wipes its GHASH key temporary. GHash
owns Polyval; ghash 0.6.0 forwards zeroize to polyval 0.7.3, whose Drop clears owned
state. Cargo metadata confirms those features actually resolve, not merely appear in
a suggested manifest. The public probe compile-checks AES's ZeroizeOnDrop trait.
The internal polyval hazmat feature required by GHASH is not permission to enable
short AES-GCM tags or use an alternate application cipher.

This covers owned state on normal Drop, not every conversion/stack/register copy,
process-abort cleanup or physical erasure. Project code must separately own/cleanse
the original 32-byte KEK, password, decrypted plaintext private keys, temporary output
and failures. Default-free in-place/detached AES-256-GCM API needs no alloc/getrandom
feature; project code supplies bounded buffers and OS randomness. No cipher change.

### Argon2 and digest cleanup source assessment

Local argon2 0.6.0 zeroize makes Block implement Zeroize and clears selected initial
hash/final-block temporaries on success. Its convenience allocated Blocks Drop frees
without wiping; Block is Copy/Clone/Debug and has no cleansing Drop. The selected
no-alloc-feature API accepts caller-owned workspace, demonstrated safely as
Zeroizing<Vec<Block>>. Future code must validate explicit memory/pass/lane/output
bounds first, allocate fallibly without secret-time reallocations, and retain workspace,
password and output owners on ALL error paths. Existing 64 MiB/3/4 is a benchmark
candidate only, not a measured or finalized profile. Explicit probe output cleansing
was observed; backend Drop memory was not inspected and no physical-erasure claim made.

Argon2's zeroize feature does not forward cleanup to Blake2. Therefore the planned
manifest must also directly pin ALREADY TRANSITIVE blake2 =0.11.0, defaults off,
features=[zeroize], to unify digest/core cleanup. sha2 =0.11.0 must likewise select
zeroize for future keyed SHA-256/HMAC state. Both additions were separately resolved,
compiled and run offline from the current vendor tree; they require no new package
sources. Blake2 is still the internal Argon2 hash, not a new application hash choice.
Digest/block-buffer feature cleanup and hash-core Drop are source-verified. Some
internal helper copies/early-return temporaries remain outside project control.

Zeroizing supplies best-effort Drop, but supports Clone/Deref: keep it behind private
role-specific owners with no public raw-secret export/Clone. Explicit exposed HKDF PRK,
PoP key and output cleanup is required; hkdf's proposed implementation has internal
PRK/expand-block copies without universal automatic wiping. Do not claim otherwise.
Use narrowly scoped safe internal operations for the opaque Phase 1 SharedSecret;
this task does not add a getter, storage importer, new unsafe or modified PQ backend.

### Deterministic CBOR route (D-23)

Proposed exact minicbor 2.3.0, default-features=false, features=[]. This is a maintained
low-level library candidate, not a canonical validator. The project's existing wire
format remains RFC 8949 core deterministic CBOR. No custom format, serde defaults,
derive macro, half-float, std/full or convenience Value layer is needed. The library
has no enabled transitive dependencies under that selection. Its license is
BlueOak-1.0.0, to report for project/owner review rather than claim legal approval.

Upstream Decoder accepts overwide integers, tags, floats, simple values and indefinite
containers in relevant methods; Encoder emits shortest integers but does not sort map
keys or enforce exact schemas. Feature omission does not turn those APIs off. Therefore
use: bounded typed decode -> strict exact-schema validation -> deterministic re-encode
-> equality with the WHOLE original byte string. Verify signatures only after equality,
over the original canonical signed bytes. Never normalize a noncanonical attacker
object and accept its signature on the normalized result.

Required planned implementation constraints:

- Precheck total sizes: existing certificate 16 KiB, bundle 32 KiB, CRL 16 MiB and
  100000-entry limit; PoP challenge/response 16 KiB. Bound before expensive allocation.
- Maximum nesting 8. Charge containers, keys, values and decoded embedded signed byte
  strings against one aggregate budget: 256 items for certificate/bundle/PoP;
  1000000 for CRLs. Checked arithmetic and bounded loops precede allocation. Avoid
  recursive generic Value decoding and unknown-field skip paths.
- Accept only schema-authorized unsigned U64, text/bytes, definite arrays/maps. All
  tags are unauthorized in these version-1 schemas. Reject negative integers,
  floats, indefinite/break forms and unrequested simple/null/bool values. Validate
  UTF-8/field-specific ASCII and exact lengths/algorithm labels.
- Exact field count and seen-field tracking reject duplicates before overwriting,
  including identical duplicates; unknown fields reject immediately. No normalization
  can repair these checks. Sort CRL entries per protocol and reject duplicate IDs.
- Require full outer AND embedded-object consumption. Emit shortest integers/lengths,
  definite containers and map keys sorted by bytewise lexicographic order of their
  encoded keys (RFC core profile, not alternate length-first ordering or Rust map order).
  Re-encode into bounded fallible buffers via the low-level Encoder and compare exactly.

This design adds schema code, not a hand-written cryptographic or CBOR primitive.
Local source/compile/integrity and positive/negative/property coverage are still gates.
Minicbor's permissive API is safe only with those restrictions. Planned tests include
integer/length transitions 23/24, 255/256, 65535/65536, overlong encodings, duplicate/
unknown keys, wrong ordering, all forbidden types, UTF-8, trailing bytes, nested embedded
objects and depth/item/entry/byte limits. None of these runtime tests ran in this task.

### Provisioning and local time decisions (D-24/D-25)

The exact normative design is in KEY_MANAGEMENT.md. One fresh CA transaction binds
both recipient public keys/IDs, recipient ID, intended CA, complete certificate body,
CA nonce and KEM ciphertext. Recipient confirms the decapsulated secret via full
HKDF-SHA-256 extract+expand and HMAC-SHA-256; recipient ML-DSA pure/empty-context
signature covers challenge AND MAC. CA verifies against retained state and consumes
one authenticated-session attempt atomically. Domains, size/time bounds and all error/
expiry/restart/substitution cases are specified. No shared secret or expected MAC is
sent; decapsulation status is never possession. Authenticated administrative enrollment
is an explicit assumption; relaying to a willing key holder and a dishonest CA are not
solved by PoP. No transcript runtime, HMAC/HKDF or unsafe operation was implemented.

Q-02's PoP design is resolved; benchmark/recovery/lifecycle remains open. Q-03's
Phase 2 semantics are resolved: zero-skew local-clock certificate/CRL intervals,
fail-closed missing/stale CRLs and cumulative sequence persistence with explicit
bootstrap, lower/conflicting-sequence rejection and atomic persistence. Whole-host
snapshots can roll back clock and retained state. No trusted time/host anti-rollback
or deployment import-cadence guarantee is invented. These residuals do not require an
online service; implementing/tested conditional checks is distinct from solving them.

### Smallest remaining dependency source delta

| Crate / exact proposed pin | Features (defaults OFF) | Purpose | Enabled transitive impact | License |
|---|---|---|---|---|
| minicbor =2.3.0 | none | Auditable CBOR codec for bounded schema/canonical comparison | No enabled dependencies; one new package | BlueOak-1.0.0 |
| hmac =0.13.0 | zeroize | Standard full-tag HMAC-SHA-256 confirmation, constant-time verify | digest ^0.11.2 fits current 0.11.3; adds no other missing source | MIT OR Apache-2.0 |
| hkdf =0.13.0 | none | Standard extract+expand SHA-256 derivation with transcript-bound info | hmac ^0.13.0 above; do not enable optional kdf crate | MIT OR Apache-2.0 |

Only THREE new sources are requested, plus existing sha2/blake2 feature changes.
Existing aes-gcm/argon2/sha2/getrandom/rpassword/zeroize cannot implement a reviewed
CBOR codec, HKDF or HMAC without hand-written primitives/format code; that workaround
is not approved. With these features the predicted refreshed graph is 34 registry
packages across platforms and 31 on this host, not yet resolved/built. No new proc
macros/native dependency/build-dependency edges are expected from their manifests;
confirm against actual supplied sources. Crate archive checksums are not yet available
locally and are NOT invented. Candidate maintenance/source inspection is not a complete
advisory or security audit; strict exact pins, source integrity/license inventory and a
vendor-only empty-home locked build must precede implementation.

### Primary upstream references inspected read-only

- [Minicbor 2.3.0 release](https://docs.rs/crate/minicbor/2.3.0),
  [exact manifest](https://docs.rs/crate/minicbor/2.3.0/source/Cargo.toml.orig),
  [decoder](https://docs.rs/minicbor/latest/src/minicbor/decode/decoder.rs.html),
  [encoder](https://docs.rs/minicbor/latest/src/minicbor/encode/encoder.rs.html).
  Latest source pages reported 2.3.0 during inspection; no local source proof claimed.
- [HMAC 0.13.0 tagged manifest](https://raw.githubusercontent.com/RustCrypto/MACs/hmac-v0.13.0/hmac/Cargo.toml)
  and its adjacent src/lib.rs / src/block_api.rs; digest verifier source is already local.
- [HKDF 0.13.0 tagged manifest](https://raw.githubusercontent.com/RustCrypto/KDFs/hkdf-v0.13.0/hkdf/Cargo.toml)
  and its adjacent src/lib.rs; [RFC 5869](https://www.rfc-editor.org/rfc/rfc5869.html).
- [RFC 8949 core deterministic profile](https://www.rfc-editor.org/rfc/rfc8949.html#section-4.2.1).

This task used read-only web research, no dependency downloads, Cargo networking or
vendoring. Candidate sources must still be supplied locally. All three actual vendor-only
resolution attempts failed with exit 101/no matching package. Recommendation:
PHASE 2 PRE-IMPLEMENTATION GATES BLOCKED until those additions pass the same checks.
No production Phase 2 authorization is exercised by this report.

### Refreshed 31-package license/checksum inventory

All rows below have source registry+https://github.com/rust-lang/crates.io-index.
This is a license inventory for owner review, not legal approval. Checksums identify
supplied locked packages; no independent archive/signature authenticity claim.

| Package | Exact version | License expression | Locked package checksum |
|---|---|---|---|
| aead | 0.6.1 | MIT OR Apache-2.0 | 1973cfbc1a2daf9cf550e74e1f088c28e7f7d8c1e1418fb6c9dc5184b7e84c99 |
| aes | 0.9.3 | MIT OR Apache-2.0 | 35f0f96ce78e38c3dc6d8948aa8163d06385be74000f3c7a95bf1eef35d3ea32 |
| aes-gcm | 0.11.1 | Apache-2.0 OR MIT | 7f2b8006a0c83f52b62ba44a97b58bf76fe2f70a329e588f67f89691d93d498f |
| argon2 | 0.6.0 | MIT OR Apache-2.0 | 134c52ddac6d63c576bef8168db10c83c49c26444ecbc68060fef078925a901c |
| base64ct | 1.8.3 | Apache-2.0 OR MIT | 2af50177e190e07a26ab74f8b1efbfe2ef87da2116221318cb1c2e82baf7de06 |
| blake2 | 0.11.0 | MIT OR Apache-2.0 | 5b5d4d889834ee8ecfc0f8426ad30faf7cdcb10f741a8e6d7224d95325479f6f |
| block-buffer | 0.12.1 | MIT OR Apache-2.0 | d2f6c7dbe95a6ed67ad9f18e57daf93a2f034c524b99fd2b76d18fdfeb6660aa |
| cfg-if | 1.0.5 | MIT OR Apache-2.0 | 4e7648175b45a9a48536d676f68d918270699102aa8dab5496df06904c914600 |
| cipher | 0.5.2 | MIT OR Apache-2.0 | e8cf2a2c93cd704877c0858356ed03480ff301ee950b43f1cbe4573b088bfa6c |
| cmov | 0.5.4 | Apache-2.0 OR MIT | 0c9ea0ac24bc397ab3c98583a3c9ba74fa56b09a4449bbe172b9b1ddb016027a |
| cpubits | 0.1.1 | MIT OR Apache-2.0 | 15b85f9c39137c3a891689859392b1bd49812121d0d61c9caf00d46ed5ce06ae |
| cpufeatures | 0.3.1 | MIT OR Apache-2.0 | 5ca28b0ae3115b884660db4118d803791fd6756b6e88f39c0f3f7859060d7566 |
| crypto-common | 0.2.2 | MIT OR Apache-2.0 | ce6e4c961d6cd6c9a86db418387425e8bdeaf05b3c8bc1411e6dca4c252f1453 |
| ctr | 0.10.1 | MIT OR Apache-2.0 | baaca1c4b237092596f64d571e9db6ce4109c4ef9742e27590f1709594461f21 |
| ctutils | 0.4.3 | Apache-2.0 OR MIT | 03bb0e1cc970d482d121d9a1744999169b69a07470b3d644a7894e53fcaf4574 |
| digest | 0.11.3 | MIT OR Apache-2.0 | f1dd6dbb5841937940781866fa1281a1ff7bd3bf827091440879f9994983d5c2 |
| getrandom | 0.4.3 | MIT OR Apache-2.0 | 300e883d756b2e4ec94e02791f39b04b522276138852cfc41d9fb7e904106099 |
| ghash | 0.6.0 | Apache-2.0 OR MIT | 2eecf2d5dc9b66b732b97707a0210906b1d30523eb773193ab777c0c84b3e8d5 |
| hybrid-array | 0.4.15 | MIT OR Apache-2.0 | 27f864f10dfb56725ce5ce5472bc52252c8f93a4ab86327122cebf62c5f59a17 |
| inout | 0.2.2 | MIT OR Apache-2.0 | 4250ce6452e92010fdf7268ccc5d14faa80bb12fc741938534c58f16804e03c7 |
| libc | 0.2.190 | MIT OR Apache-2.0 | ce5d3ddc6d3fa000eb1536d85e147bfe31aacaba692ed6a876f95cb7c855be78 |
| polyval | 0.7.3 | Apache-2.0 OR MIT | f0fa31d631f2b2cb2a544d0aa321ce847a94764d701ca2becc411138b93d49cd |
| r-efi | 6.0.0 | MIT OR Apache-2.0 OR LGPL-2.1-or-later | f8dcc9c7d52a811697d2151c701e0d08956f92b0e24136cf4cf27b57a6a0d9bf |
| rpassword | 7.5.4 | Apache-2.0 | 2da316a15f47e3d053de9cb2c439650bd8fa4aaeb9365f2e5f27f492ff73c196 |
| rtoolbox | 0.0.6 | Apache-2.0 | 9a1efe12a1469752d0e6ff5ebec0b6ef4924cc5c4c71046b0ec730040535819d |
| sha2 | 0.11.0 | MIT OR Apache-2.0 | 446ba717509524cb3f22f17ecc096f10f4822d76ab5c0b9822c5f9c284e825f4 |
| typenum | 1.20.1 | MIT OR Apache-2.0 | b6f5e870be6c3b371b77fe0ee0bafb859fa4964b4404c27de1d380043c4dda20 |
| universal-hash | 0.6.1 | MIT OR Apache-2.0 | f4987bdc12753382e0bec4a65c50738ffaabc998b9cdd1f952fb5f39b0048a96 |
| windows-link | 0.2.1 | MIT OR Apache-2.0 | f0805222e57f7521d6a62e36fa9163bc891acd422f971defe97d64e70d0a4fe5 |
| windows-sys | 0.61.2 | MIT OR Apache-2.0 | ae137229bcbd6cdf0f7b80a31df61766145077ddf49416a728b02cb3921ff3fc |
| zeroize | 1.9.1 | Apache-2.0 OR MIT | e13084392c5e4bc371903e2935a5eaeed24905a7511356b883835e18a78f6879 |


### Current manual offline verification commands

Run from the repository root. This replaces the earlier historical missing-zeroize
example. It uses disposable files, public fixtures, an empty home and local source
replacement. The integrity verifier in PHASE_2_VALIDATION.txt can also be copied to a
disposable file to recheck exact Cargo/owner file sets (sha256sum alone checks only
listed paths). No dependency download is involved. New dependency checks must remain
negative until the owner refreshes the bundle; that expected absence is not a crypto test.

```sh
sha256sum -c .deps/phase2-vendor-SHA256SUMS.txt
python3 scripts/validate_phase1.py
git diff --check
```

```python
import os
from pathlib import Path
import subprocess
import tempfile

repo = Path.cwd()
manifest = '[package]\nname="fp1-phase2-dependency-bootstrap"\nversion="0.0.0"\nedition="2024"\n[dependencies]\naes-gcm={version="=0.11.1",default-features=false,features=["aes","zeroize"]}\nargon2={version="=0.6.0",default-features=false,features=["zeroize"]}\nsha2={version="=0.11.0",default-features=false}\ngetrandom={version="=0.4.3",default-features=false}\nrpassword={version="=7.5.4",default-features=false}\nzeroize={version="=1.9.1",default-features=false,features=["alloc"]}\n'
source = 'use aes_gcm::{Aes256Gcm, KeyInit, Nonce, aead::AeadInOut};\nuse argon2::{Algorithm, Argon2, Block, Params, Version};\nuse zeroize::{Zeroize, ZeroizeOnDrop, Zeroizing};\nfn needs_drop_clean<T: ZeroizeOnDrop>() {}\nfn main() {\n    // Public dependency fixtures only; not production keys/passwords.\n    needs_drop_clean::<aes_gcm::aes::Aes256>();\n    let cipher=Aes256Gcm::new_from_slice(&[0u8;32]).expect("fixture");\n    let nonce=Nonce::from([0u8;12]);\n    let mut data=[0u8;16];\n    let tag=cipher.encrypt_inout_detached(&nonce,b"public aad",(&mut data[..]).into()).expect("fixture encrypt");\n    cipher.decrypt_inout_detached(&nonce,b"public aad",(&mut data[..]).into(),&tag).expect("fixture decrypt");\n    assert_eq!(data,[0u8;16]);\n    let params=Params::new(32,3,4,Some(32)).expect("small fixture");\n    let mut workspace=Zeroizing::new(vec![Block::default();params.block_count()]);\n    let mut output=Zeroizing::new([0u8;32]);\n    Argon2::new(Algorithm::Argon2id,Version::V0x13,params).hash_password_into_with_memory(b"PUBLIC FIXTURE",b"PUBLIC SALT 1234!",&mut output[..],&mut workspace[..]).expect("fixture KDF");\n    workspace.zeroize();\n    output.zeroize();\n    assert!(output.iter().all(|v|*v==0));\n    let mut bytes=vec![0x5au8;32];bytes.zeroize();assert!(bytes.is_empty());\n    println!("Cleanup-enabled AES-256-GCM/Argon2id API probe PASS; safe Zeroizing<Vec<Block>> available; explicit output cleansing observed. No physical-erasure or backend Drop-memory observation claimed.");\n}\n'
with tempfile.TemporaryDirectory(prefix="fp1-p2-cleanup-", dir="/tmp") as temp:
    probe = Path(temp)
    (probe / "src").mkdir()
    (probe / ".cargo").mkdir()
    (probe / "cargo-home").mkdir()  # Initially empty.
    (probe / "Cargo.toml").write_text(manifest)
    (probe / "Cargo.lock").write_bytes(
        (repo / ".deps/phase2-bootstrap-Cargo.lock").read_bytes())
    (probe / "src/main.rs").write_text(source)
    (probe / ".cargo/config.toml").write_text(
        '[source.crates-io]\nreplace-with="vendored-sources"\n'
        '[source.vendored-sources]\ndirectory=' +
        repr(str(repo / ".deps/phase2-vendor")) + '\n'
        '[net]\noffline=true\n[build]\n'
        'rustflags=["--cfg", \'getrandom_backend="linux_getrandom"\']\n')
    env = {k: v for k, v in os.environ.items() if not k.startswith("CARGO_")
           and k not in ("RUSTFLAGS", "RUSTC_WRAPPER", "RUSTC_WORKSPACE_WRAPPER")}
    env.update(CARGO_HOME=str(probe / "cargo-home"),
               CARGO_TARGET_DIR=str(probe / "target"))
    for args in (("build", "--offline", "--locked"),
                 ("run", "--offline", "--locked"),
                 ("tree", "--offline", "--locked", "-e", "features")):
        subprocess.run(["cargo", *args], cwd=probe, env=env, check=True)
```

To reproduce the additionally tested cleanup feature profile, change only this
disposable manifest: sha2 features=[zeroize], and add blake2 =0.11.0 defaults off/
zeroize; run cargo generate-lockfile --offline before the locked run. It needs no new
source packages. Do not apply changes to production Cargo files under this gate task.


## FINAL Phase 2 dependency gate — 2026-10-07

Current verdict: **PHASE 2 DEPENDENCY GATE READY**. Earlier missing-source reports,
counts, negative probes and manual examples are historical. This latest task verifies
dependencies ONLY; no production implementation or Phase 2 runtime test was started.
The approved protocol, Q-02/Q-03 documents and Phase 1 code/evidence are unchanged.

### Integrity and exact graph

The refreshed lock, owner metadata, vendor package manifests/Cargo checksum files and
owner SHA-256 inventory agree exactly: 34 registry packages, 1435 source checksums,
1469 inventory entries, zero mismatches, zero missing/unexpected files and no symlinks.
Source replacement config points only at the supplied vendor tree. A reusable verifier
and input fingerprints are recorded in PHASE_2_VALIDATION.txt. This proves consistency,
not authenticity against an owner/supplier controlling all evidence; original package
archives were not provided to recompute archive checksums independently.

An independently constructed exact root manifest mirrors the owner's ten direct
requirements and uses the supplied lock unchanged. Full resolved packages, features
and dependency edges match owner metadata. Linux resolves 31 registry packages plus
the disposable root: 10 direct, 21 transitive; full lock has 24 transitive.

| Package | Exact pin | Direct default features | Actual resolved features |
|---|---|---|---|
| aes-gcm | =0.11.1 | OFF | aes, zeroize |
| argon2 | =0.6.0 | OFF | zeroize |
| sha2 | =0.11.0 | OFF | zeroize |
| blake2 | =0.11.0 | OFF | zeroize |
| getrandom | =0.4.3 | OFF | none |
| rpassword | =7.5.4 | ON in owner root; no feature definitions | none |
| zeroize | =1.9.1 | OFF | alloc |
| minicbor | =2.3.0 | OFF | none |
| hmac | =0.13.0 | OFF | zeroize |
| hkdf | =0.13.0 | OFF | none |

rpassword defines no features, so the owner's defaults flag introduces no feature
change. HMAC also has no default feature definition: Cargo tree's transitive HKDF
request for hmac/default activates no extra named feature/dependency. HMAC resolves
only zeroize. No minicbor alloc/derive/half/std/full or HKDF kdf feature is enabled.
AES/GHASH/POLYVAL, digest/block-buffer, SHA-2 and Blake2 cleanup features resolve;
project-owned workspace/secret cleanup and software-erasure limitations are retained.

### Independent build and minimal API evidence

All commands used a task-created initially empty CARGO_HOME, a fresh target,
net.offline=true, --offline --locked and exclusive directory-source replacement.
Ambient CARGO_* flags/source overrides and Rust flag/wrapper variables were removed;
no user config/credentials/cache copied. Probe configured Linux syscall-only getrandom
cfg; future production integration must enforce it. No network access occurred.

Commands executed successfully in the disposable project:

```sh
cargo build --offline --locked
cargo check --offline --locked
cargo run --offline --locked
cargo metadata --offline --locked --format-version 1
cargo metadata --offline --locked --format-version 1 --filter-platform x86_64-unknown-linux-gnu
cargo tree --offline --locked -e features --target x86_64-unknown-linux-gnu
```

Build compiled 31 registry crates plus root; lock bytes unchanged after all commands.
Minicbor's minimal Decoder/Encoder constructor types compile with zero features; no
CBOR input was encoded/decoded and no project schema/canonical validator implemented.
HMAC-SHA-256 passed RFC 4231 case 1 and rejected changed/31-byte tags. HKDF-SHA-256
passed RFC 5869 case 1, including PRK, 42-byte output and the first 32 bytes as direct
output. Fixtures are public constants, not real secrets or Q-02 transcripts. The probe
calls library APIs only, not manually implemented primitives. Exposed fixture output/
PRK cleanup uses existing zeroize; no universal internal-copy erasure claim.

D-23 remains: bounded typed decode -> strict exact-schema validation -> deterministic
re-encode -> exact original-byte comparison -> cryptographic verification. Minicbor
availability is NOT proof of project canonical validation. D-24 remains full certificate/
ciphertext/nonce/transaction binding, ML-KEM-768 + HKDF/HMAC confirmation and recipient
ML-DSA-65 pure/empty-context signature over challenge AND proof. Decapsulation status
is never possession. D-25 remains local/configured-time intervals and monotonic retained
CRL checks only; host clock/snapshot rollback is NOT prevented. KEY_MANAGEMENT.md and
CRYPTO_PROTOCOL.md are byte-identical to this task's starting files.

### Supply-chain delta and local-source triage

Exactly three package sources were added relative to the previous 31-package graph;
retained package versions/checksums are unchanged. All are crates.io registry sources.
The table gives supplied locked package checksums, not independent archive authenticity.
License expressions are inventory for owner review, not legal approval.

| Package | Version | License | Locked SHA-256 |
|---|---|---|---|
| hkdf | 0.13.0 | MIT OR Apache-2.0 | 4aaa26c720c68b866f2c96ef5c1264b3e6f473fe5d4ce61cd44bbe913e553018 |
| hmac | 0.13.0 | MIT OR Apache-2.0 | 6303bc9732ae41b04cb554b844a762b4115a61bfaa81e3e83050991eeb56863f |
| minicbor | 2.3.0 | BlueOak-1.0.0 | c12b4033ffaa92fbf9df03df38d19324f52bad130dd223f811734a8006dd2d69 |

The new manifests have build=false, no build dependencies, no proc-macro targets and
no native source files. Normal enabled edges: minicbor none; hmac -> existing digest
0.11.3; hkdf -> hmac 0.13.0. Optional/dev dependencies (half, minicbor-derive, kdf,
hex-literal, test hashes etc.) are not enabled or acquired. Full graph's only custom
build targets remain getrandom/libc; build-dependency edges and proc macros are zero.
Three non-Linux registry packages remain locked: r-efi, windows-link, windows-sys.
Only Fedora Linux compiled; other platforms and a new advisory database not checked.

Unsafe syntax triage of newly added Rust sources:

- minicbor src/bytes.rs:37,45: shared/mutable ByteSlice casts over repr(transparent)
  byte slices, preserving input lifetime/ownership. These are library internals.
- minicbor src/decode.rs:470,497: MaybeUninit array conversion only after all N slots
  initialized; success forgets the temporary owner, failure Drop visits initialized
  prefix only. Generic array decoding requires normal dependency review; the selected
  future route remains primitive typed schema parsing rather than generic value trees.
- hmac/hkdf: no unsafe syntax in src/*.rs; normalized manifests forbid unsafe_code.
  Their generic hash/digest internals remain existing dependency scope. No new project
  unsafe, native FFI, secret getter, algorithm agility or production hook was added.

These sites were inspected for gate triage; this is not a complete upstream security
or memory-safety audit, constant-time proof, FIPS module validation or legal clearance.
No CRITICAL/HIGH gate issue identified. HKDF intermediate-copy limitations remain open.

### Gate disposition and remaining work

Dependency availability/integrity/API gate CLOSED. Phase 2 runtime remains unimplemented,
its 19 rows remain PLANNED, and no completion/production-readiness claim follows.
Argon2 benchmarking, secret-owner integration, bounded schema/PoP/PKI/storage/CRL tests,
security review and accepted time/rollback/deployment limitations remain their approved
future work. User's current scope stops here. Phase 3 remains unapproved.
Exact commands/source/results and final documentation checks: PHASE_2_VALIDATION.txt.


### Final completion and current reproduction — 2026-10-08

Both independent fresh verification runs passed; the first run's disposable /tmp
files were lost during an environment refresh, so the second run's complete logs/source
were captured in PHASE_2_VALIDATION.txt before cleanup. The final dependency gate
remains READY. This is not Phase 2 implementation or permission to begin Phase 3.

Earlier manual examples used previous owner graphs and must not be mixed with the
refreshed ten-pin lock. The following current example reproduces compilation only;
copy the public API probe source from the final validation section to rerun its
HMAC/HKDF fixtures. Run from the repository root; no production files are changed.

```python
from pathlib import Path
import json
import os
import subprocess
import tempfile

repo = Path.cwd()
manifest = '[package]\nname="fp1-phase2-dependency-bootstrap"\nversion="0.0.0"\nedition="2024"\n[dependencies]\naes-gcm={version="=0.11.1",default-features=false,features=["aes", "zeroize"]}\nargon2={version="=0.6.0",default-features=false,features=["zeroize"]}\nblake2={version="=0.11.0",default-features=false,features=["zeroize"]}\ngetrandom={version="=0.4.3",default-features=false,features=[]}\nhkdf={version="=0.13.0",default-features=false,features=[]}\nhmac={version="=0.13.0",default-features=false,features=["zeroize"]}\nminicbor={version="=2.3.0",default-features=false,features=[]}\nrpassword={version="=7.5.4",default-features=true,features=[]}\nsha2={version="=0.11.0",default-features=false,features=["zeroize"]}\nzeroize={version="=1.9.1",default-features=false,features=["alloc"]}\n'
with tempfile.TemporaryDirectory(prefix="fp1-p2-final-manual-", dir="/tmp") as name:
    probe = Path(name)
    (probe / "src").mkdir()
    (probe / ".cargo").mkdir()
    (probe / "cargo-home").mkdir()
    (probe / "Cargo.toml").write_text(manifest)
    (probe / "Cargo.lock").write_bytes(
        (repo / ".deps/phase2-bootstrap-Cargo.lock").read_bytes())
    (probe / "src/main.rs").write_text("fn main() {}\n")
    (probe / ".cargo/config.toml").write_text(
        '[source.crates-io]\nreplace-with="vendored-sources"\n'
        '[source.vendored-sources]\ndirectory=' +
        json.dumps(str(repo / ".deps/phase2-vendor")) + '\n'
        '[net]\noffline=true\n[build]\n'
        'rustflags=["--cfg", \'getrandom_backend="linux_getrandom"\']\n')
    env = {k: v for k, v in os.environ.items() if not k.startswith("CARGO_")
           and k not in ("RUSTFLAGS", "RUSTC_WRAPPER", "RUSTC_WORKSPACE_WRAPPER")}
    env.update(CARGO_HOME=str(probe / "cargo-home"),
               CARGO_TARGET_DIR=str(probe / "target"))
    subprocess.run(["cargo", "build", "--offline", "--locked"],
                   cwd=probe, env=env, check=True)
```

Additional read-only commands:

```sh
sha256sum -c .deps/phase2-vendor-SHA256SUMS.txt
python3 scripts/validate_phase1.py
git diff --check
```

The final validation section's verifier additionally checks exact sets and Cargo
metadata; sha256sum alone cannot reject an unlisted extra file. This current manual
example was syntax-checked; actual build/API evidence is the separately captured
fresh verification above, not a claim of new Phase 2 runtime tests.
