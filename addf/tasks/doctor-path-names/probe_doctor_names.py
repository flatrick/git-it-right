"""Probe: what gir doctor reports for file names git quotes or cannot decode.

Usage: python3 probe_doctor_names.py GIR_BINARY
Each case creates a repository, runs `gir doctor` and prints the report lines for the checks
in question. Names Windows cannot hold (double quote, backslash, tab, control characters,
non-UTF-8 bytes) are created only on Unix.
"""
import os
import subprocess
import sys
import tempfile
from pathlib import Path

GIR = str(Path(sys.argv[1]).resolve())
ENV = {
    **os.environ,
    "GIT_CONFIG_GLOBAL": os.devnull, "GIT_CONFIG_NOSYSTEM": "1",
    "GIT_AUTHOR_NAME": "t", "GIT_AUTHOR_EMAIL": "t@t", "GIT_COMMITTER_NAME": "t", "GIT_COMMITTER_EMAIL": "t@t",
}
UNIX = os.name != "nt"
IDS = ("case-collision", "windows-names", "exec-bit", ".gitignore")


def sh(d, *args):
    return subprocess.run(args, cwd=d, env=ENV, capture_output=True)


def repo(files):
    d = tempfile.mkdtemp(prefix="gir-doctor-")
    sh(d, "git", "init", "-q", "-b", "main")
    for name, data in files:
        p = Path(d, name)
        p.parent.mkdir(parents=True, exist_ok=True)
        p.write_bytes(data)
    sh(d, "git", "add", "-A")
    sh(d, "git", "commit", "-q", "-m", "chore: files")
    return d


def doctor(d, *args):
    r = sh(d, GIR, "doctor", *args)
    lines = (r.stdout + r.stderr).decode("utf-8", errors="backslashreplace").splitlines()
    return [l for l in lines if any(f" {i}:" in l for i in IDS)]


def case(title, files, fix=False):
    if files is None:
        print(f"\n## {title}: skipped (Windows cannot hold these names)")
        return None
    print(f"\n## {title}")
    d = repo(files)
    for line in doctor(d):
        print("  " + line)
    if fix:
        print("  --- gir doctor --fix")
        for line in doctor(d, "--fix"):
            print("  " + line)
        print("  index modes after --fix:", sh(d, "git", "ls-files", "-s", "-z").stdout.split(b"\0"))
    return d


u = lambda files: files if UNIX else None

case("a1 exec-bit control: .githooks/plain", [(".githooks/plain", b"#!/bin/sh\n")], fix=True)
case("a1 exec-bit: .githooks/pre \"x\" (quoted by git)", u([('.githooks/pre "x"', b"#!/bin/sh\n")]), fix=True)
case("a1 exec-bit: .githooks/my hook (space, not quoted)", [(".githooks/my hook", b"#!/bin/sh\n")], fix=True)
case("a2 windows-names: say \"hi\".txt", u([('say "hi".txt', b"x\n")]))
case("a2 windows-names: back\\slash.txt", u([("back\\slash.txt", b"x\n")]))
case("a2 windows-names: tab<TAB>here.txt", u([("tab\there.txt", b"x\n")]))
case("a3 case-collision control: A.txt / a.txt", u([("A.txt", b"x\n"), ("a.txt", b"y\n")]))
case("a3 case-collision: Q \"a\".txt / q \"a\".txt", u([('Q "a".txt', b"x\n"), ('q "a".txt', b"y\n")]))
case("a3 case-collision: Å.txt / å.txt (non-ASCII)", u([("Å.txt", b"x\n"), ("å.txt", b"y\n")]))
case("b ignore rules control: Angstrom.csproj", [("Angstrom.csproj", b"<Project/>\n")])
case("b ignore rules: Ångström.csproj (non-ASCII)", [("Ångström.csproj", b"<Project/>\n")])
case("c control character: bell<BEL>.txt", u([("bell\x07.txt", b"x\n")]))
case("d non-UTF-8 name: caf<E9>.txt", u([(os.fsdecode(b"caf\xe9.txt"), b"x\n")]))
case("d non-UTF-8 name in .githooks: hook<E9>", u([(os.fsdecode(b".githooks/hook\xe9"), b"#!/bin/sh\n")]), fix=True)
