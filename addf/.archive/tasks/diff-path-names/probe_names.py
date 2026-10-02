"""Probe: how git writes awkward file names in diff headers, and what gir fixup does with them.

Usage: python3 probe_names.py GIR_BINARY
Names with a double quote, a backslash or a tab cannot exist on Windows; those cases are skipped there.
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
    "GIR_INTERACTIVE": "0",
}
NAMES = ["plain.txt", "my file.txt", "café.txt"]
if os.name != "nt":
    NAMES += ['say "hi".txt', "back\\slash.txt", "tab\there.txt"]


def sh(d, *args):
    return subprocess.run(args, cwd=d, env=ENV, capture_output=True, text=True)


for name in NAMES:
    d = tempfile.mkdtemp(prefix="gir-names-")
    sh(d, "git", "init", "-q", "-b", "main")
    Path(d, "base.txt").write_text("base\n")
    sh(d, "git", "add", "-A"); sh(d, "git", "commit", "-q", "-m", "chore: base")
    sh(d, "git", "switch", "-q", "-c", "topic")
    Path(d, name).write_text("one\ntwo\n")
    sh(d, "git", "add", "-A"); sh(d, "git", "commit", "-q", "-m", "feat: add file")
    Path(d, name).write_text("one\nTWO\n")
    sh(d, "git", "add", "-A")
    diff = sh(d, "git", "-c", "core.quotePath=false", "diff", "--cached", "-U0", "--no-color", "--no-renames", "--src-prefix=a/", "--dst-prefix=b/").stdout
    status = sh(d, "git", "-c", "core.quotePath=false", "diff", "--cached", "--name-status", "--no-renames").stdout
    r = sh(d, GIR, "fixup", "--dry-run")
    print(f"## {name!r}")
    print("  headers:", [l for l in diff.splitlines() if l.startswith(("diff --git", "--- ", "+++ "))])
    print("  name-status:", repr(status))
    print(f"  gir fixup --dry-run -> exit {r.returncode}: {(r.stdout + r.stderr).strip()!r}")
