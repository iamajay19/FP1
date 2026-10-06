"""Offline validation of Phase 0 artifacts; never a runtime security test suite."""

import ast
import re
import shutil
import subprocess
import tempfile
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
DOCS = (
    "PROJECT_SPEC IMPLEMENTATION_PLAN ARCHITECTURE THREAT_MODEL CRYPTOGRAPHIC_SCOPE "
    "CRYPTO_PROTOCOL PACKAGE_FORMAT LEDGER_PROTOCOL WATERMARK_DESIGN "
    "SECURITY_ASSUMPTIONS ATTACK_SURFACE TEST_MATRIX KNOWN_LIMITATIONS DECISIONS "
    "OFFLINE_DEPLOYMENT USER_GUIDE ENVIRONMENT KEY_MANAGEMENT PHASE_0_REVIEW"
).split()
DIRECTORIES = (
    "pki publisher trusted-client ledger/network ledger/chaincode forensic "
    "tests/integration tests/adversarial tests/fixtures scripts"
).split()
UNAVAILABLE = ("build", "encrypt", "decrypt", "pki", "ledger", "forensic")


def markdown_files():
    return sorted(ROOT.glob("*.md")) + sorted((ROOT / "docs").glob("*.md")) + [
        ROOT / directory / "README.md" for directory in DIRECTORIES
    ]


def code_blocks(path):
    blocks = []
    opened = None
    lines = []
    for number, line in enumerate(path.read_text().splitlines(), 1):
        if line.startswith("```"):
            if opened is None:
                opened = line[3:].strip()
                lines = []
            else:
                if line != "```":
                    raise ValueError(f"{path}:{number}: malformed closing fence")
                blocks.append((opened, "\n".join(lines)))
                opened = None
        elif opened is not None:
            lines.append(line)
    if opened is not None:
        raise ValueError(f"{path}: unclosed code fence")
    return blocks


class PhaseZero(unittest.TestCase):
    def test_01_documents(self):
        """P0-DOC-001: required documentation exists and is substantive."""
        for name in DOCS:
            with self.subTest(document=name):
                path = ROOT / "docs" / f"{name}.md"
                self.assertTrue(path.is_file(), str(path))
                self.assertGreater(len(path.read_text().split()), 40)
        for name in ("AGENTS.md", "README.md", "LICENSE", ".gitignore", "Makefile"):
            self.assertTrue((ROOT / name).is_file(), name)

    def test_02_structure_and_scope(self):
        """P0-STRUCT-001: documented component paths remain scaffold-only."""
        for directory in DIRECTORIES:
            path = ROOT / directory
            self.assertTrue(path.is_dir(), directory)
            self.assertTrue((path / "README.md").is_file(), directory)
        for directory in DIRECTORIES[:-1]:
            files = [p for p in (ROOT / directory).rglob("*") if p.is_file()]
            self.assertTrue(all(p.name == "README.md" for p in files), files)
        for manifest in ("Cargo.toml", "go.mod", "pyproject.toml"):
            self.assertFalse((ROOT / manifest).exists(), manifest)

    def test_03_links(self):
        """P0-LINK-001: local Markdown links resolve inside the repository."""
        for path in markdown_files():
            for target in re.findall(r"\]\(([^)]+)\)", path.read_text()):
                if "://" in target or target.startswith("#"):
                    continue
                target = target.split("#", 1)[0]
                resolved = (path.parent / target).resolve()
                self.assertTrue(resolved.is_relative_to(ROOT), (path, target))
                self.assertTrue(resolved.exists(), (path, target))

    def test_04_fences_and_examples(self):
        """P0-SYNTAX-001: balanced fences and parseable shell/Python examples."""
        for path in markdown_files():
            for language, block in code_blocks(path):
                if language == "python":
                    ast.parse(block, filename=str(path))
                if language in ("sh", "bash"):
                    result = subprocess.run(
                        ["sh", "-n"], input=block, text=True, capture_output=True,
                        check=False, timeout=10,
                    )
                    self.assertEqual(result.returncode, 0, (path, result.stderr))

    def test_05_unavailable_commands(self):
        """P0-CLI-001: future runtime targets fail clearly, without outputs."""
        with tempfile.TemporaryDirectory(prefix="cadp-make-test-") as temporary:
            for target in UNAVAILABLE:
                result = subprocess.run(
                    ["make", "--no-print-directory", "-f", str(ROOT / "Makefile"), target],
                    cwd=temporary, capture_output=True, text=True, check=False,
                    timeout=10,
                )
                self.assertNotEqual(result.returncode, 0, target)
                self.assertIn("NOT IMPLEMENTED: Phase 0", result.stderr)
                self.assertEqual(list(Path(temporary).iterdir()), [], target)

    def test_06_static_and_format(self):
        """P0-STATIC-001: Python syntax and simple formatting policy."""
        paths = markdown_files() + list((ROOT / "scripts").glob("*.py")) + [
            ROOT / "Makefile", ROOT / ".gitignore", ROOT / "LICENSE"
        ]
        for path in paths:
            content = path.read_text()
            self.assertTrue(content.endswith("\n"), str(path))
            self.assertNotIn("\r", content, str(path))
            for number, line in enumerate(content.splitlines(), 1):
                self.assertEqual(line, line.rstrip(), f"{path}:{number}")
            if path.suffix == ".py":
                ast.parse(content, filename=str(path))
                for number, line in enumerate(content.splitlines(), 1):
                    self.assertLessEqual(len(line), 100, f"{path}:{number}")

    def test_07_matrix(self):
        """P0-MATRIX-001: planned test IDs are unique and cover plan IDs."""
        matrix = (ROOT / "docs/TEST_MATRIX.md").read_text()
        ids = re.findall(r"^\| ([A-Z][A-Z0-9-]+-\d{3}) \|", matrix, re.MULTILINE)
        self.assertEqual(len(ids), len(set(ids)), "Duplicate test ID")
        plan = (ROOT / "docs/IMPLEMENTATION_PLAN.md").read_text()
        # Only test-list entries count; AES-256-GCM is an algorithm, not a test ID.
        required = set(re.findall(
            r"^([A-Z][A-Z0-9-]+-\d{3})(?:[ \t]|$)", plan, re.MULTILINE
        ))
        self.assertFalse(required - set(ids), sorted(required - set(ids)))

    @unittest.skipUnless(shutil.which("mmdc"), "mmdc unavailable; Mermaid render untested")
    def test_08_mermaid_render(self):
        """P0-MERMAID-001: actual Mermaid rendering when tooling is installed."""
        with tempfile.TemporaryDirectory(prefix="cadp-mermaid-") as temporary:
            for path in markdown_files():
                for language, block in code_blocks(path):
                    if language != "mermaid":
                        continue
                    source = Path(temporary) / "diagram.mmd"
                    output = Path(temporary) / "diagram.svg"
                    source.write_text(block)
                    result = subprocess.run(
                        ["mmdc", "-i", str(source), "-o", str(output)],
                        capture_output=True, text=True, check=False, timeout=30,
                    )
                    self.assertEqual(result.returncode, 0, result.stderr)
                    self.assertTrue(output.is_file())


if __name__ == "__main__":
    unittest.main(verbosity=2)
