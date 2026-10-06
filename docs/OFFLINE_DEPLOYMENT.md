# Offline Deployment

Status: PLANNED runtime deployment; Phase 0 validation runs without network access.
An air-gapped LAN between local client, peers and orderer is permitted. Offline does
not mean the ledger is reachable without a local network or that revocation is always fresh.

## Phase 0

Required: Python 3.12+ and GNU Make. No package installation required.

```sh
cd /home/iamajay/Projects/FP1
make check
make environment
```

The environment probe queries the local Docker daemon read-only; it does not pull
images/start services. Phase 0 check needs neither Docker nor network access.
See [ENVIRONMENT.md](ENVIRONMENT.md) for actual readiness, not assumed prerequisites.

## Later deployment manifest (Phase 13)

Record exact versions, platforms, checksums, provenance, licenses and build flags for:
Rust toolchain, locked/vendored crates (including oqs/oqs-sys); Python interpreter and
pinned wheelhouse; Go toolchain and vendored modules; liboqs source/build and C/C++/
CMake toolchain; Fabric binaries, native/external chaincode dependencies, Compose
configuration and every container image by digest. Preserve platform-specific native
libraries and shared-library search configuration. Runtime auto-fetch is forbidden.

Connected preparation may download dependencies only during approved development.
Transfer a verified manifest and artifacts through controlled media. Plan `cargo
vendor`/locked offline builds, pip `--no-index` with local wheelhouse, Go vendored
builds and `docker save`/`docker load`; exact executable commands await pinned
versions. No installation command in Phase 0 pretends an unselected version is tested.

Provision trust anchors, MSP identities, application certificates, CRLs and local
time policy without public CA/identity services. Protect ledger volumes, configuration,
backups and cross-org checkpoints. Bootstrap permissions must match chaincode ACLs.
No telemetry, CDN, cloud KMS, SaaS, public blockchain or external API dependencies.

## Acceptance evidence required

On a clean supported host with external networking disabled: install from transferred
artifacts, build, start the local Fabric network, create CA/recipients, register a
single document, encrypt once, release Alice/Bob attributed outputs and independently
extract evidence. Record local networking rules, commands, versions and outputs.
Test service restart, stale CRL, dependency absence, outbound connection attempts,
peer/orderer outage and ledger-finalization failure. No output on unknown commit.
Capture evidence for OFFLINE-001 and E2E-001; none of this acceptance has run yet.
