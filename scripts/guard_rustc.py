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


def cargo_metadata_query(args):
    # Observed Cargo 1.98.1 target-information handshake. Every print here is
    # metadata-only; no source path, output, emission or arbitrary print option
    # may join the exemption. In particular --print=link-args can compile/link.
    prefix = ["-", "--crate-name", "___", "--print=file-names"]
    suffix = ["--crate-type", "bin", "--crate-type", "rlib", "--crate-type", "dylib",
              "--crate-type", "cdylib", "--crate-type", "staticlib", "--crate-type",
              "proc-macro", "--print=sysroot", "--print=split-debuginfo",
              "--print=crate-name", "--print=cfg", "-Wwarnings"]
    # Only the reviewed native target is supported; new toolchain query shapes
    # require review rather than widening this to generic --print handling.
    for cfg in ([], ["--cfg", 'getrandom_backend="linux_getrandom"'], [APPROVED]):
        for target in ([], ["--target", "x86_64-unknown-linux-gnu"]):
            if args == prefix + cfg + target + suffix:
                return True
    return False


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
    metadata = cargo_metadata_query(args)
    # Cargo reserves this stdin/placeholder invocation for cached target info.
    # An unfamiliar shape is unsupported even with valid CLIPPY_ARGS; otherwise
    # it could pass normally yet cache an extra-channel failure after bad flags.
    if args[:3] == ["-", "--crate-name", "___"] and not metadata:
        rejected()
    handshake = identity or metadata
    # Cargo caches compiler identity/target information without CLIPPY_ARGS in
    # the key. The real driver ignores that channel for these exact handshakes.
    # Rejecting it here would cache failure across later valid invocations.
    # Artifact-producing calls still check the channel before exec, including
    # host build scripts. No parsing of Clippy's private separator is needed.
    clippy_args = os.environ.get("CLIPPY_ARGS", "")
    if not handshake and ("@" in clippy_args or "getrandom_backend" in clippy_args):
        rejected()
    # Explicit --target builds omit target cfgs from host build-script binaries.
    # Those tools do not use runtime RNG. Permit zero selections only there;
    # validation still rejects every @file, duplicate or alternate selection.
    host_script = any(args[i:i + 2] == ["--crate-name", "build_script_build"]
                      for i in range(len(args))) and not any(
                          a == "--target" or a.startswith("--target=") for a in args)
    zero_permitted = host_script and allowed([*args, APPROVED])
    if not (handshake or allowed(args) or zero_permitted):
        rejected()
    try:
        os.execvp(command[0], command)
    except OSError:
        rejected()


if __name__ == "__main__":
    main()
