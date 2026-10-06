"""Inspect Phase 0 prerequisites without installing or changing the host."""

import importlib.util
import json
import platform
import shutil
import subprocess
import tempfile
from pathlib import Path


def probe(argv):
    """Bound probes so an inaccessible daemon cannot block validation forever."""
    if shutil.which(argv[0]) is None:
        return {"command": argv, "status": "MISSING"}
    try:
        result = subprocess.run(
            argv, capture_output=True, text=True, timeout=20, check=False
        )
    except (OSError, subprocess.TimeoutExpired) as error:
        return {"command": argv, "status": "FAILED", "detail": str(error)}
    return {
        "command": argv,
        "status": "AVAILABLE" if result.returncode == 0 else "FAILED",
        "exit_code": result.returncode,
        "output": (result.stdout + result.stderr).strip(),
    }


def main():
    commands = [
        ["rustc", "--version"],
        ["rustup", "show"],
        ["cargo", "--version"],
        ["python3", "--version"],
        ["python3", "-m", "pip", "--version"],
        ["go", "version"],
        ["docker", "--version"],
        ["docker", "compose", "version"],
        ["docker", "info", "--format", "{{.ServerVersion}}"],
        ["git", "--version"],
        ["git", "status", "--short"],
        ["cc", "--version"],
        ["c++", "--version"],
        ["cmake", "--version"],
        ["pkg-config", "--version"],
        ["pkg-config", "--modversion", "liboqs"],
        ["make", "--version"],
        ["ruff", "--version"],
        ["markdownlint", "--version"],
        ["mmdc", "--version"],
        ["sh", "-c", "ulimit -l; ulimit -c"],
    ]
    report = {
        "os": platform.platform(),
        "architecture": platform.machine(),
        "os_release": Path("/etc/os-release").read_text(),
        "probes": [probe(command) for command in commands],
        "python_venv_module": importlib.util.find_spec("venv") is not None,
        "python_oqs_module": importlib.util.find_spec("oqs") is not None,
        "swap": Path("/proc/swaps").read_text(),
    }
    # Probe actual creation, not just importability; remove the isolated environment.
    with tempfile.TemporaryDirectory(prefix="cadp-phase0-") as temporary:
        report["venv_creation"] = probe(
            ["python3", "-m", "venv", str(Path(temporary) / "venv")]
        )
    cache = probe(["ldconfig", "-p"])
    if cache["status"] == "AVAILABLE":
        cache["output"] = "\n".join(
            line for line in cache["output"].splitlines() if "oqs" in line.lower()
        ) or "No liboqs entry in dynamic linker cache"
    report["liboqs_linker_cache"] = cache
    print(json.dumps(report, indent=2))


if __name__ == "__main__":
    main()
