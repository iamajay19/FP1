"""Phase 2B public probes: terminal no-echo, AAD reference, Drop sensitivity.

No network, real identities/passwords or production failure hooks. Cargo uses the
approved project source configuration. Temporary files contain ciphertext/public
fixture source only; the typed password probe does not print its input.
"""
import hashlib
import json
import os
from pathlib import Path
import pty
import select
import shutil
import subprocess
import tempfile
import termios
import time

from check_pqc_native import build_artifacts, run

ROOT = Path(__file__).resolve().parents[1]


def deterministic_public(value):
    """Independent tiny fixture encoder, never used by production or on secrets."""
    def head(major, size):
        if size < 24:
            return bytes([major * 32 + size])
        n = 1 if size < 256 else 2 if size < 65536 else 4 if size < 2**32 else 8
        return bytes([major * 32 + {1: 24, 2: 25, 4: 26, 8: 27}[n]]) + size.to_bytes(n, 'big')
    if isinstance(value, int):
        return head(0, value)
    if isinstance(value, bytes):
        return head(2, len(value)) + value
    if isinstance(value, str):
        data = value.encode()
        return head(3, len(data)) + data
    if isinstance(value, list):
        return head(4, len(value)) + b''.join(map(deterministic_public, value))
    if isinstance(value, dict):
        pairs = sorted((deterministic_public(k), deterministic_public(v)) for k, v in value.items())
        return head(5, len(pairs)) + b''.join(k + v for k, v in pairs)
    raise AssertionError('unsupported public fixture type')


def vectors():
    header = dict(schema_version=1, recipient_id=bytes([1])*32,
                  kem_algorithm='ML-KEM-768', kem_key_id=bytes([2])*32,
                  signature_algorithm='ML-DSA-65', signature_key_id=bytes([3])*32,
                  aead_algorithm='AES-256-GCM',
                  kdf=dict(algorithm='Argon2id', version=19, memory_kib=65536,
                           time_cost=3, parallelism=4, output_len=32, salt=bytes([4])*16))
    expected = {'kem': (391, 'c5ef77df93fe9fe2f1ae03e98f1a2fe89f6de6d6e258d1d8fce9ddd7eadaf4d4'),
                'signature': (397,
                    '6767117d497f73ae84b680522b50a53c615875e3f826dd919916800799464116')}
    tests = (ROOT / 'trusted-client/src/storage/tests.rs').read_text()
    for role, (length, digest) in expected.items():
        data = deterministic_public(['CADP-KEY-BUNDLE-AAD-v1', dict(header=header, role=role)])
        assert len(data) == length and hashlib.sha256(data).hexdigest() == digest
        assert digest in tests
    print('P2B-AAD-001 independent public AAD vectors PASS', flush=True)


def terminal(binary, env):
    pid, fd = pty.fork()
    if pid == 0:
        os.execve(binary, [str(binary)], env)
    original = termios.tcgetattr(fd)
    received = b''
    sent = False
    # Public synthetic fixture; never echo/log this value or include it in errors.
    fixture = b'PUBLIC-PTY-FIXTURE-ONLY'
    deadline = time.monotonic() + 20
    try:
        while time.monotonic() < deadline:
            ready, _, _ = select.select([fd], [], [], 0.2)
            if ready:
                try:
                    data = os.read(fd, 4096)
                except OSError:
                    break
                if not data:
                    break
                received += data
            if b'Bundle password: ' in received and not sent:
                assert not (termios.tcgetattr(fd)[3] & termios.ECHO), 'terminal echo enabled'
                os.write(fd, fixture + b'\n')
                sent = True
            if b'terminal-input-ok' in received:
                break
        assert sent and b'terminal-input-ok' in received, 'protected terminal probe failed'
        assert fixture not in received, 'terminal input was echoed'
        assert termios.tcgetattr(fd)[3] == original[3], 'terminal flags not restored'
        waited, status = os.waitpid(pid, 0)
        assert waited == pid and os.waitstatus_to_exitcode(status) == 0
        print('P2B-PASSWORD-001 controlling PTY ECHO off / no echo / restoration PASS', flush=True)
    finally:
        os.close(fd)
        # Never leave a stuck disposable child on a failed terminal assertion.
        try:
            waited, _ = os.waitpid(pid, os.WNOHANG)
            if waited == 0:
                os.kill(pid, 9)
                os.waitpid(pid, 0)
        except ChildProcessError:
            pass


def controls(env):
    _, library = build_artifacts(env)
    with tempfile.TemporaryDirectory(prefix='fp1-p2b-controls-') as name:
        work = Path(name)
        source = work / 'terminal.rs'
        source.write_text('#![deny(unsafe_code)]\nuse cadp_pqc::storage::Password;\n'
                          'fn main(){let p=Password::read_from_terminal()'
                          '.expect("protected input");'
                          'drop(p);println!("terminal-input-ok");}\n')
        binary = work / 'terminal'
        run(['rustc', '--edition=2024', '-D', 'warnings', source, '--extern',
             f'cadp_pqc={library}', '-L',
             f'dependency={library.parent / "deps"}', '-o', binary], env)
        terminal(binary, env)
        copy = work / 'without-wipe'
        shutil.copytree(ROOT / 'trusted-client', copy, ignore=shutil.ignore_patterns('target'))
        (work / 'scripts').symlink_to(ROOT / 'scripts', target_is_directory=True)
        secret = copy / 'src/storage/secret.rs'
        contents = secret.read_text()
        marker = '        self.bytes.as_mut_slice().zeroize();'
        assert contents.count(marker) == 1
        secret.write_text(contents.replace(
            marker, '        // Negative control: omit only storage Drop wipe.'))
        altered_env = dict(env, CADP_LIBOQS_SOURCE=str(ROOT / '.deps/liboqs-0.16.0'),
                           CARGO_TARGET_DIR=str(work / 'negative-target'))
        command = ['cargo', 'test', '--offline', '--locked', '--release', '--manifest-path',
                   copy / 'Cargo.toml', 'p2b_secret_001']
        result = subprocess.run(command, cwd=ROOT, env=altered_env, capture_output=True, text=True)
        assert result.returncode == 101, 'missing-wipe negative control unexpectedly accepted'
        assert 'p2b_secret_001_redaction_password_and_kek_drop ... FAILED' in result.stdout
        assert 'assertion failed' in result.stdout + result.stderr
        print('P2B-SECRET-007 omitted-Drop-wipe negative control rejected exit=101 PASS',
              flush=True)
    print('Disposable public fixtures removed; no production hook or new unsafe', flush=True)


def main():
    env = os.environ.copy()
    env.setdefault('CARGO_HOME', str(ROOT / '.deps/cargo-home'))
    vectors()
    command = ['cargo', 'test', '--offline', '--locked', '--release', '--manifest-path',
               ROOT / 'trusted-client/Cargo.toml', 'p2b_']
    run(command, env)
    controls(env)
    command = ['cargo', 'run', '--offline', '--locked', '--release', '--manifest-path',
               ROOT / 'trusted-client/Cargo.toml', '--bin', 'argon2_bench']
    result = subprocess.run(command, cwd=ROOT, env=env, capture_output=True, text=True, check=True)
    lines = [s for s in result.stdout.splitlines() if s.startswith('memory_kib=')]
    assert len(lines) == 15
    assert all(0 < float(line.split('wall_ms=')[1]) < 60000 for line in lines)
    print(result.stdout, end='', flush=True)
    print('P2B-BENCH-001 five synthetic/public runs per bounded profile PASS', flush=True)


if __name__ == '__main__':
    main()
