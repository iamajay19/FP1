# scripts

Dependency-free environment inspection, documentation validation and PQC checks.

IMPLEMENTED: Phase 0 tools preserved; Phase 1 scope, native contract and source-pin
checks added. `validate_phase1.py` reuses applicable documentation checks without
rewriting the historical scaffold-only Phase 0 tests. `check_pqc_native.py` uses
Clang ASan/UBSan and disposable linker fault injection; the liboqs archive itself
is the normal noninstrumented build. `check_pqc_source_pin.py` changes only a
temporary local Git clone. `probe_liboqs_readiness.py` remains metadata-only.
The native check also runs PQC-BUILD-002 configuration rejection and PQC-SECRET-003
through the production Rust ownership path, using isolated synthetic partial outputs.
The new Rust/C cleanup fixture is not sanitizer-instrumented. All fixtures are removed;
no production failure hooks are installed. The script honors CARGO_TARGET_DIR for
fresh validation and obtains its actual build paths from Cargo JSON.
Follow the [implementation plan](../docs/IMPLEMENTATION_PLAN.md) and explicit phase approval.
