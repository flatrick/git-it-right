"""Probe: does gir fixup --split leave index entries it does not commit alone?

Run from the repository root after `cargo build`: python3 addf/tasks/split-preserves-index/probe_split_index.py
For each setup, prints `git status --short` and `git ls-files -v` flags before and after a
successful split and a split that fails and restores, then a verdict:
`kept` when the only status lines that changed are the staged paths the split committed.
"""
import os
import subprocess
import tempfile
from pathlib import Path

GIR = Path("target/debug/gir").resolve()
ENV = {**os.environ, "GIR_INTERACTIVE": "0"}
COMMITTED = ("a", "b")


def sh(cwd, *args):
    return subprocess.run(args, cwd=cwd, env=ENV, capture_output=True, text=True)


def commit(d, path, text, message):
    p = Path(d, path)
    p.parent.mkdir(parents=True, exist_ok=True)
    p.write_text(text)
    sh(d, "git", "add", path)
    sh(d, "git", "commit", "-q", "-m", message)


def repo():
    d = tempfile.mkdtemp(prefix="gir-split-")
    sh(d, "git", "init", "-q", "-b", "main")
    sh(d, "git", "config", "user.name", "probe")
    sh(d, "git", "config", "user.email", "probe@example.invalid")
    commit(d, "base.txt", "base\n", "chore: base")
    sh(d, "git", "switch", "-q", "-c", "topic")
    commit(d, "a", "a\n", "feat: a")
    commit(d, "b", "b\n", "feat: b")
    return d


def sparse(d):
    commit(d, "out/c", "c\n", "feat: c")
    sh(d, "git", "sparse-checkout", "set", "--no-cone", "/*", "!/out/")


def skip_worktree(d):
    commit(d, "cfg", "c\n", "feat: cfg")
    sh(d, "git", "update-index", "--skip-worktree", "cfg")
    Path(d, "cfg").write_text("private local edit\n")


def intent_to_add(d):
    Path(d, "ita").write_text("not staged yet\n")
    sh(d, "git", "add", "-N", "ita")


def failing_hook(d):
    hook = Path(d, ".git/hooks/commit-msg")
    hook.write_text("#!/bin/sh\ngrep -q 'fixup! feat: b' \"$1\" && { echo 'probe: hook refuses feat: b' >&2; exit 1; }\nexit 0\n")
    hook.chmod(0o755)


def state(d):
    status = sh(d, "git", "status", "--short").stdout.splitlines()
    flags = sh(d, "git", "ls-files", "-v").stdout.splitlines()
    return status, flags


def verdict(before, after, committed):
    drop = lambda lines: sorted(l for l in lines if l[3:] not in committed)
    return "kept" if drop(before[0]) == drop(after[0]) and before[1] == after[1] else "CHANGED"


for name, setup in (("sparse checkout", sparse), ("skip-worktree with a local edit", skip_worktree), ("git add -N", intent_to_add)):
    for outcome in ("success", "failure"):
        d = repo()
        setup(d)
        Path(d, "a").write_text("A\n")
        Path(d, "b").write_text("B\n")
        sh(d, "git", "add", "a", "b")
        if outcome == "failure":
            failing_hook(d)
        head = sh(d, "git", "rev-parse", "HEAD").stdout.strip()
        before = state(d)
        r = sh(d, str(GIR), "fixup", "--split")
        after = state(d)
        print(f"## {name}, {outcome}: gir fixup --split exit {r.returncode}")
        print(r.stderr.rstrip())
        print("before status:", before[0])
        print("after status: ", after[0])
        print("before flags: ", before[1])
        print("after flags:  ", after[1])
        moved = sh(d, "git", "rev-parse", "HEAD").stdout.strip() != head
        committed = COMMITTED if outcome == "success" else ()
        print(f"HEAD moved: {moved}; verdict: {verdict(before, after, committed)}\n")
