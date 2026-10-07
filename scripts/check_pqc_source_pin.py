"""PQC-BUILD-001: source rejection tests in a disposable local Git clone."""

import os
import shlex
import subprocess
import tempfile
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
SOURCE = ROOT / ".deps/liboqs-0.16.0"


def run(args):
    print("$ " + shlex.join(map(str, args)), flush=True)
    subprocess.run(args, check=True)


def rejected(source, target, diagnostic):
    env = os.environ.copy()
    env["CADP_LIBOQS_SOURCE"] = str(source)
    env["CARGO_HOME"] = str(ROOT / ".deps/cargo-home")
    env["CARGO_TARGET_DIR"] = str(target)
    command = ["cargo", "build", "--offline", "--locked", "--manifest-path",
               str(ROOT / "trusted-client/Cargo.toml")]
    print(f"$ CADP_LIBOQS_SOURCE={shlex.quote(str(source))} "
          f"CARGO_TARGET_DIR={shlex.quote(str(target))} " + shlex.join(command), flush=True)
    result = subprocess.run(command, env=env, capture_output=True, text=True, check=False)
    print(result.stdout, end="")
    print(result.stderr, end="")
    print(f"exit={result.returncode}", flush=True)
    if result.returncode != 101 or diagnostic not in result.stderr:
        raise RuntimeError("source guard did not reject for the expected reason")


def main():
    with tempfile.TemporaryDirectory(prefix="fp1-pin-test-") as temporary:
        work = Path(temporary)
        clone = work / "source"
        target = work / "target"
        rejected(work / "missing", target, "approved local liboqs source missing")
        run(["git", "clone", "--quiet", "--shared", SOURCE, clone])
        # Supplied source can be shallow. Create a harmless local empty commit
        # instead of assuming a parent commit exists; only the clone is changed.
        run(["git", "-C", clone, "-c", "user.name=Phase 1 guard test",
             "-c", "user.email=phase1-test@example.invalid", "commit", "--quiet",
             "--allow-empty", "-m", "Temporary source-pin rejection fixture"])
        rejected(clone, target, "approved exact commit")
        run(["git", "-C", clone, "checkout", "--quiet", "0.16.0"])
        cmake = clone / "CMakeLists.txt"
        original = cmake.read_text()
        cmake.write_text(original + "\n# deliberate guard-test change\n")
        rejected(clone, target, "source must be clean")
        cmake.write_text(original)
        run(["git", "-C", clone, "update-index", "--assume-unchanged", "CMakeLists.txt"])
        cmake.write_text(original + "\n# concealed guard-test change\n")
        rejected(clone, target, "source index must not hide tracked files")
        cmake.write_text(original)
        run(["git", "-C", clone, "update-index", "--no-assume-unchanged", "CMakeLists.txt"])
        untracked = clone / "untracked-guard-test.c"
        untracked.write_text("/* no implementation; source guard fixture */\n")
        rejected(clone, target, "source must be clean")
        untracked.unlink()
        (clone / "build").mkdir()
        (clone / "build/ignored.txt").write_text("source guard fixture\n")
        rejected(clone, target, "source must be clean")
    run(["git", "-C", SOURCE, "status", "--porcelain", "--untracked-files=all"])
    print("PQC-BUILD-001 PASS: 6 rejected cases; original source untouched; clone removed")


if __name__ == "__main__":
    main()
