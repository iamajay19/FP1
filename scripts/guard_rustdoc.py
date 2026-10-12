#!/usr/bin/env python3
"""Reject unapproved RNG cfgs and @files before invoking the trusted rustdoc.

Cargo exposes no effective rustdocflags to build.rs. Inspect actual argv here;
never open response files or use a shell. This is not a compiler/toolchain sandbox.
"""
import os
import sys


def allowed(args):
    if any(arg.startswith("@") for arg in args):
        return False
    count = 0
    i = 0
    while i < len(args):
        arg = args[i]
        value = None
        if arg == "--cfg":
            i += 1
            if i == len(args):
                return False
            value = args[i]
        elif arg.startswith("--cfg="):
            value = arg[len("--cfg="):]
        if value is not None and "getrandom_backend" in value:
            if value != 'getrandom_backend="linux_getrandom"':
                return False
            count += 1
        i += 1
    return count == 1


def main():
    args = sys.argv[1:]
    # Cargo/toolchain may request identity without compiling any source.
    if args not in (["-V"], ["--version"], ["-vV"]) and not allowed(args):
        sys.exit("Phase 2A rustdoc policy rejected: exclusive linux_getrandom cfg; "
                 "no response files")
    os.execvp("rustdoc", ["rustdoc", *args])


if __name__ == "__main__":
    main()
