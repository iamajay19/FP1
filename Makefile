PYTHON ?= python3
.DEFAULT_GOAL := help
.PHONY: help check test lint fmt-check environment build encrypt decrypt pki ledger forensic

help:
	@printf '%s\n' 'Phase 0: documentation and scaffold only.' 'make check       Validate Phase 0 (no runtime security tests).' 'make environment Inspect tools without installing dependencies.' 'build/encrypt/decrypt/pki/ledger/forensic: unavailable until approved phases.'

check test lint fmt-check:
	@$(PYTHON) scripts/validate_phase0.py

environment:
	@$(PYTHON) scripts/inspect_environment.py

build encrypt decrypt pki ledger forensic:
	@printf '%s\n' 'NOT IMPLEMENTED: Phase 0 scaffold only; later phases require explicit approval.' >&2
	@exit 2
