"""Probe: how gir fixup reads hunk lines that look like ---/+++ headers.

Run from the repository root after `cargo build`: python3 addf/tasks/diff-header-parsing/probe_dash_dash.py
"""
import os
import subprocess
import sys
import tempfile
from pathlib import Path

GIR = Path("target/debug/gir").resolve()
ENV = {**os.environ, "GIR_INTERACTIVE": "0", "GIT_AUTHOR_DATE": "2026-01-01T00:00:00", "GIT_COMMITTER_DATE": "2026-01-01T00:00:00"}


def sh(cwd, *args):
    return subprocess.run(args, cwd=cwd, env=ENV, capture_output=True, text=True)


def repo():
    d = tempfile.mkdtemp(prefix="gir-probe-")
    sh(d, "git", "init", "-q", "-b", "main")
    sh(d, "git", "config", "user.name", "probe")
    sh(d, "git", "config", "user.email", "probe@example.invalid")
    commit(d, "base.txt", "base\n", "chore: base")
    sh(d, "git", "switch", "-q", "-c", "topic")
    return d


def commit(d, path, text, message):
    Path(d, path).write_text(text)
    sh(d, "git", "add", path)
    sh(d, "git", "commit", "-q", "-m", message)


def stage(d, path, text):
    Path(d, path).write_text(text)
    sh(d, "git", "add", path)


def run(d, *args):
    r = sh(d, str(GIR), *args)
    print(f"$ gir {' '.join(args)}  (exit {r.returncode})")
    for stream in (r.stdout, r.stderr):
        if stream.strip():
            print(stream.rstrip())


def case(title):
    print(f"\n## {title}")


case("1 delete '-- header', edit a later line from the same commit")
d = repo()
commit(d, "x.lua", "-- header\nlocal a = 1\nlocal b = 2\nlocal c = 3\nlocal d = 4\n", "feat: lua")
stage(d, "x.lua", "local a = 1\nlocal b = 2\nlocal c = 3\nlocal d = 5\n")
run(d, "fixup", "--dry-run")

case("2 delete '-- /dev/null' (commit A), edit a line from commit B")
d = repo()
commit(d, "x.lua", "-- /dev/null\nl1\nl2\nl3\nl4\n", "feat: A")
commit(d, "x.lua", "-- /dev/null\nl1\nl2\nl3\nL4\n", "feat: B")
stage(d, "x.lua", "l1\nl2\nl3\nL4x\n")
run(d, "fixup", "--dry-run")
print("expected: the hunks trace to two commits (A and B), so no single target")

case("3 add a line '++ b/foo' to a file from one commit")
d = repo()
commit(d, "y.txt", "one\ntwo\n", "feat: y")
stage(d, "y.txt", "one\n++ b/foo\ntwo\n")
run(d, "fixup", "--dry-run")

case("4 delete '-- /dev/null' and add '++ b/foo' later in the same file")
d = repo()
commit(d, "z.lua", "-- /dev/null\nm1\nm2\nm3\nm4\n", "feat: z")
stage(d, "z.lua", "m1\nm2\nm3\n++ b/foo\nm4\n")
run(d, "fixup", "--dry-run")
sys.exit(0)
