"""Probe: the split loop run by hand with GIT_INDEX_FILE pointing at a temporary index.

Run from anywhere: python3 addf/tasks/split-preserves-index/probe_temp_index.py
For each setup it hashes .git/index, runs read-tree, apply --cached and commit --fixup
against a temporary index for two targets, then checks the real index's bytes, the final
HEAD tree, and git status. The failure variant has a commit-msg hook refuse the second target.
"""
import hashlib
import os
import subprocess
import tempfile
from pathlib import Path

ENV = {**os.environ}


def sh(cwd, *args, env=None, stdin=None):
    return subprocess.run(args, cwd=cwd, env=env or ENV, capture_output=True, text=True, input=stdin)


def commit(d, path, text, message):
    p = Path(d, path)
    p.parent.mkdir(parents=True, exist_ok=True)
    p.write_text(text)
    sh(d, "git", "add", path)
    sh(d, "git", "commit", "-q", "-m", message)


def repo():
    d = tempfile.mkdtemp(prefix="gir-tmpidx-")
    sh(d, "git", "init", "-q", "-b", "main")
    sh(d, "git", "config", "user.name", "probe")
    sh(d, "git", "config", "user.email", "probe@example.invalid")
    commit(d, "base.txt", "base\n", "chore: base")
    commit(d, "a", "a\n", "feat: a")
    commit(d, "b", "b\n", "feat: b")
    return d


def sparse_cone_sparse_index(d):
    commit(d, "out/c", "c\n", "feat: c")
    sh(d, "git", "sparse-checkout", "set", "--cone", "--sparse-index", "in")
    Path(d, "in").mkdir(exist_ok=True)


def sparse_no_cone(d):
    commit(d, "out/c", "c\n", "feat: c")
    sh(d, "git", "sparse-checkout", "set", "--no-cone", "/*", "!/out/")


def skip_worktree(d):
    commit(d, "cfg", "c\n", "feat: cfg")
    sh(d, "git", "update-index", "--skip-worktree", "cfg")
    Path(d, "cfg").write_text("private local edit\n")


def intent_to_add(d):
    Path(d, "ita").write_text("x\n")
    sh(d, "git", "add", "-N", "ita")


def digest(d):
    return hashlib.sha256(Path(d, ".git/index").read_bytes()).hexdigest()[:16]


for name, setup in (("sparse cone + sparse index", sparse_cone_sparse_index), ("sparse no-cone", sparse_no_cone), ("skip-worktree", skip_worktree), ("git add -N", intent_to_add)):
    for fail in (False, True):
        d = repo()
        setup(d)
        Path(d, "a").write_text("A\n")
        Path(d, "b").write_text("B\n")
        sh(d, "git", "add", "a", "b")
        if fail:
            hook = Path(d, ".git/hooks/commit-msg")
            hook.write_text("#!/bin/sh\ngrep -q 'fixup! feat: b' \"$1\" && exit 1\nexit 0\n")
            hook.chmod(0o755)
        orig = sh(d, "git", "rev-parse", "HEAD").stdout.strip()
        goal = sh(d, "git", "write-tree").stdout.strip()
        status_before = sh(d, "git", "status", "--short").stdout.splitlines()
        before = digest(d)
        tmp = Path(sh(d, "git", "rev-parse", "--absolute-git-dir").stdout.strip(), "gir-probe-index")
        env = {**ENV, "GIT_INDEX_FILE": str(tmp)}
        diffs = {p: sh(d, "git", "diff", "--cached", "-U0", "--", p).stdout for p in ("a", "b")}
        shas = {p: sh(d, "git", "rev-parse", f"HEAD~{1 if p == 'b' else 2}" if name in ("sparse cone + sparse index", "sparse no-cone", "skip-worktree") else f"HEAD~{0 if p == 'b' else 1}").stdout.strip() for p in ("a", "b")}
        errors = []
        done = []
        for p in ("a", "b"):
            done.append(p)
            for step in (
                lambda: sh(d, "git", "read-tree", orig, env=env),
                lambda: sh(d, "git", "apply", "--cached", "--unidiff-zero", "-", env=env, stdin="".join(diffs[q] for q in done)),
                lambda: sh(d, "git", "commit", "--quiet", f"--fixup={shas[p]}", env=env),
            ):
                r = step()
                if r.returncode:
                    errors.append(f"{p}: {r.stderr.strip()}")
                    break
            if errors:
                break
        if errors:
            sh(d, "git", "reset", "-q", "--soft", orig)
        tmp.unlink(missing_ok=True)
        after = digest(d)
        tree = sh(d, "git", "rev-parse", "HEAD^{tree}").stdout.strip()
        status_after = sh(d, "git", "status", "--short").stdout.splitlines()
        print(f"## {name}, {'failure' if fail else 'success'}")
        print("errors:", errors)
        print("real index bytes unchanged:", before == after)
        print("HEAD tree == goal:", tree == goal, "| HEAD == orig:", sh(d, "git", "rev-parse", "HEAD").stdout.strip() == orig)
        print("status before:", status_before)
        print("status after: ", status_after)
        print("subjects:", sh(d, "git", "log", "--format=%s", "-3").stdout.splitlines(), "\n")
