#!/usr/bin/env python3
"""Cargo rustc-wrapper: validate actual argv, then exec Cargo's trusted tool.

No response-file reads, shell or argument rewriting. The existing rustdoc parser
enforces the same literal cfg policy; compiler/toolchain/PATH remain trusted.
"""
import os
import shutil
import sys
from pathlib import Path

from guard_rustdoc import allowed

APPROVED = '--cfg=getrandom_backend="linux_getrandom"'


def rejected():
    sys.exit("Phase 2A rustc policy rejected: exclusive linux_getrandom cfg; "
             "no response files; guarded compiler required")


def executable(name):
    found = shutil.which(name)
    if found is None:
        rejected()
    path = Path(found).resolve()
    if path == Path(__file__).resolve():
        rejected()  # Never recurse if RUSTC points back at this wrapper.
    return path


def main():
    command = sys.argv[1:]
    if not command or any(arg.startswith("@") for arg in command):
        rejected()
    # Clippy appends this separate flag channel after its Cargo-supplied argv.
    # It may add warnings, but never @files or another backend selection. Reject
    # markers conservatively without parsing its private separator/language.
    clippy_args = os.environ.get("CLIPPY_ARGS", "")
    if "@" in clippy_args or "getrandom_backend" in clippy_args:
        rejected()
    executable(command[0])
    args = command[1:]
    # Cargo nests its workspace wrapper inside rustc-wrapper. Only the normal
    # trusted Clippy driver is supported; retain its real-rustc argv unchanged.
    workspace = os.environ.get("RUSTC_WORKSPACE_WRAPPER")
    if workspace:
        if executable(workspace).name != "clippy-driver":
            rejected()
        if executable(workspace) == executable(command[0]):
            if not args:
                rejected()
            executable(args[0])
            args = args[1:]
    identity = args in (["-V"], ["--version"], ["-vV"])
    # Explicit --target builds omit target cfgs from host build-script binaries.
    # Those tools do not use runtime RNG. Permit zero selections only there;
    # validation still rejects every @file, duplicate or alternate selection.
    host_script = any(args[i:i + 2] == ["--crate-name", "build_script_build"]
                      for i in range(len(args))) and not any(
                          a == "--target" or a.startswith("--target=") for a in args)
    # Cargo also queries the host target without target rustflags. --print
    # without --emit does not compile an artifact; reject bad cfgs even there.
    query = any(a == "--print" or a.startswith("--print=") for a in args) and not any(
        a == "--emit" or a.startswith("--emit=") for a in args)
    zero_permitted = (host_script or query) and allowed([*args, APPROVED])
    if not (identity or allowed(args) or zero_permitted):
        rejected()
    try:
        os.execvp(command[0], command)
    except OSError:
        rejected()


if __name__ == "__main__":
    main()
