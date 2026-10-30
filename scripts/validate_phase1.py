"""Current Phase 1 documentation/scope checks; preserve historical Phase 0 evidence."""

import unittest
from pathlib import Path

from validate_phase0 import PhaseZero

ROOT = Path(__file__).resolve().parents[1]


class PhaseOneScope(unittest.TestCase):
    def test_phase1_scope(self):
        """PQC-SCOPE-001: approved PQC/foundation/storage only; other phases stay absent."""
        for directory in (
            "pki", "publisher", "ledger/network", "ledger/chaincode", "forensic",
            "tests/integration", "tests/adversarial", "tests/fixtures",
        ):
            files = [p for p in (ROOT / directory).rglob("*") if p.is_file()]
            self.assertTrue(all(p.name == "README.md" for p in files), files)
        client = ROOT / "trusted-client"
        expected = {
            "README.md", "Cargo.toml", "Cargo.lock", "build.rs",
            "native/oqs_shim.c", "native/oqs_shim.h", "src/lib.rs",
            "src/pqc/mod.rs", "src/pqc/ffi.rs", "src/pqc/kem.rs",
            "src/pqc/signature.rs", "src/pqc/error.rs", "src/pqc/secret.rs",
            "src/pqc/tests.rs", "tests/native_contract.c",
            "tests/support/partial_output.c", "tests/support/partial_output.rs",
            # Explicitly authorized Phase 2A foundation; no PKI/storage workflow.
            "src/foundation/mod.rs", "src/foundation/error.rs",
            "src/foundation/cbor.rs", "src/foundation/crypto.rs",
            "src/foundation/identifiers.rs", "src/foundation/secret.rs",
            "src/foundation/tests.rs",
            # Explicitly authorized Phase 2B storage; no CA/PoP/CRL workflow.
            "src/storage/mod.rs", "src/storage/error.rs", "src/storage/secret.rs",
            "src/storage/kdf.rs", "src/storage/schema.rs", "src/storage/aead.rs",
            "src/storage/persistence.rs", "src/storage/tests.rs",
            "src/bin/argon2_bench.rs",
        }
        actual = {
            str(p.relative_to(client)) for p in client.rglob("*")
            if p.is_file() and "target" not in p.relative_to(client).parts
        }
        self.assertEqual(actual, expected)
        for filename in ("Cargo.toml", "go.mod", "pyproject.toml"):
            self.assertFalse((ROOT / filename).exists(), filename)


if __name__ == "__main__":
    suite = unittest.defaultTestLoader.loadTestsFromTestCase(PhaseOneScope)
    # Reuse still-applicable artifact checks. The original scaffold-only and
    # unavailable-build checks remain untouched as historical Phase 0 tests;
    # Phase 1 scope and Cargo build/tests replace those assumptions now.
    for method in (
        "test_01_documents", "test_03_links", "test_04_fences_and_examples",
        "test_06_static_and_format", "test_07_matrix", "test_08_mermaid_render",
    ):
        suite.addTest(PhaseZero(method))
    result = unittest.TextTestRunner(verbosity=2).run(suite)
    raise SystemExit(0 if result.wasSuccessful() else 1)
