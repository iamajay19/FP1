PYTHON ?= python3
CARGO ?= cargo
CARGO_HOME ?= $(CURDIR)/.deps/cargo-home
export CARGO_HOME
PQC_ARGS := --offline --locked --manifest-path trusted-client/Cargo.toml
.DEFAULT_GOAL := help
.PHONY: help check docs-check test lint fmt-check native-check source-pin-check environment build encrypt decrypt pki ledger forensic

help:
	@printf '%s\n' 'Phase 1: ML-KEM-768 / ML-DSA-65 primitive library only.' 'make check       Documentation, scope, Rust, native and source-pin checks.' 'make build       Build pinned liboqs and safe Rust primitives offline.' 'make environment Inspect tools without installing dependencies.' 'encrypt/decrypt/pki/ledger/forensic: unavailable until approved phases.'

check: docs-check fmt-check lint test native-check source-pin-check

docs-check:
	@$(PYTHON) scripts/validate_phase1.py

test:
	$(CARGO) test $(PQC_ARGS)

lint:
	$(CARGO) clippy $(PQC_ARGS) --all-targets -- -D warnings

fmt-check:
	$(CARGO) fmt --manifest-path trusted-client/Cargo.toml --check

native-check:
	$(PYTHON) scripts/check_pqc_native.py

source-pin-check:
	$(PYTHON) scripts/check_pqc_source_pin.py

environment:
	@$(PYTHON) scripts/inspect_environment.py

build:
	$(CARGO) build $(PQC_ARGS)

encrypt decrypt pki ledger forensic:
	@printf '%s\n' 'NOT IMPLEMENTED: Phase 1 primitives only; later phases require explicit approval.' >&2
	@exit 2
