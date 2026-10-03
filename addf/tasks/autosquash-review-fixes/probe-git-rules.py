#!/usr/bin/env python3
"""Probe: the git 2.56.0 behaviours r1-r7 depend on. Run: python3 probe-git-rules.py"""
from __future__ import annotations

import os
import subprocess
import tempfile
import time
from pathlib import Path

ENV = {**os.environ, "GIT_EDITOR": "true", "GIT_SEQUENCE_EDITOR": "true",
       "GIT_CONFIG_GLOBAL": os.devnull, "GIT_CONFIG_NOSYSTEM": "1"}


def git(repo: Path, *args: str, check: bool = True) -> str:
    r = subprocess.run(["git", *args], cwd=repo, env=ENV, capture_output=True, text=True)
    if check and r.returncode:
        raise RuntimeError(f"git {' '.join(args)}: {r.stderr}")
    return r.stdout


def new_repo(d: str) -> Path:
    repo = Path(d)
    git(repo, "init", "-q", "-b", "main")
    git(repo, "config", "user.email", "p@x")
    git(repo, "config", "user.name", "p")
    return repo


def commit(repo: Path, message: str, path: str, content: str) -> str:
    (repo / path).write_text(content)
    git(repo, "add", path)
    git(repo, "commit", "-q", "--no-verify", "--cleanup=verbatim", "-m", message)
    return git(repo, "rev-parse", "HEAD").strip()


def subjects(repo: Path, rng: str) -> list[str]:
    return git(repo, "log", "-z", "--reverse", "--format=%s", rng).rstrip("\0").split("\0")


def fold_case(name: str, target: str, fixup: str) -> None:
    with tempfile.TemporaryDirectory() as d:
        repo = new_repo(d)
        base = commit(repo, "chore: init", "i", "i\n")
        commit(repo, target, "a", "a\n")
        commit(repo, fixup, "b", "b\n")
        before = subjects(repo, f"{base}..HEAD")
        git(repo, "rebase", "-q", "--autosquash", base)
        after = subjects(repo, f"{base}..HEAD")
        print(f"## {name}\n%s before: {before}\n%s after:  {after}\nRESULT: {'folded' if len(after) < len(before) else 'LEFT'}\n")


def main() -> None:
    print(git(Path("."), "--version"))
    print("=== subject rule (r2)")
    with tempfile.TemporaryDirectory() as d:
        repo = new_repo(d)
        commit(repo, "feat: one\n  two  \nthree\n\nbody line\n", "a", "a\n")
        print(f"%s of 'feat: one\\n  two  \\nthree\\n\\nbody': {git(repo, 'log', '-1', '--format=%s')!r}\n")
    fold_case("wrapped fixup subject vs older 'feat: one three'", "feat: one three", "fixup! feat: one\ntwo")
    fold_case("wrapped target title, fixup names the joined subject", "feat: one\ntwo", "fixup! feat: one two")
    fold_case("two spaces after the prefix", "feat: one", "fixup!  feat: one")
    fold_case("tab after the prefix", "feat: one", "fixup!\tfeat: one")
    fold_case("prefix chain with two spaces inside", "feat: one", "fixup!  squash!  feat: one")
    fold_case("wrapped amend! subject, no message paragraph", "feat: one", "amend! feat:\none")

    print("=== merge commit as a target (r3)")
    with tempfile.TemporaryDirectory() as d:
        repo = new_repo(d)
        base = commit(repo, "chore: init", "i", "i\n")
        git(repo, "switch", "-q", "-c", "side")
        commit(repo, "feat: side", "s", "s\n")
        git(repo, "switch", "-q", "main")
        commit(repo, "feat: main", "m", "m\n")
        git(repo, "merge", "-q", "--no-ff", "--no-edit", "side")
        commit(repo, "fixup! Merge branch 'side'", "f", "f\n")
        before = subjects(repo, f"{base}..HEAD")
        git(repo, "rebase", "-q", "--autosquash", base)
        after = subjects(repo, f"{base}..HEAD")
        print(f"before: {before}\nafter:  {after}\n")

    print("=== HEAD during `git rebase <upstream> <branch>` from another checkout (r5)")
    with tempfile.TemporaryDirectory() as d:
        repo = new_repo(d)
        base = commit(repo, "chore: init", "i", "i\n")
        git(repo, "switch", "-q", "-c", "feature")
        commit(repo, "feat: a", "a", "a\n")
        commit(repo, "feat: b", "b", "b\n")
        commit(repo, "fixup! HEAD~2", "c", "c\n")
        git(repo, "switch", "-q", "main")
        git(repo, "rebase", "-q", "--autosquash", base, "feature")
        after = subjects(repo, f"{base}..feature")
        files = git(repo, "log", "--reverse", "--format=%s:", "--name-only", f"{base}..feature").split()
        print(f"after: {after}\nfiles: {files}\n")

    print("=== HEAD-relative specifiers with the branch checked out (r5)")
    for spec in ["HEAD~2", "HEAD~1", "@~2", "feature~2"]:
        with tempfile.TemporaryDirectory() as d:
            repo = new_repo(d)
            base = commit(repo, "chore: init", "i", "i\n")
            git(repo, "switch", "-q", "-c", "feature")
            commit(repo, "feat: a", "a", "a\n")
            commit(repo, "feat: b", "b", "b\n")
            commit(repo, f"fixup! {spec}", "c", "c\n")
            git(repo, "rebase", "-q", "--autosquash", base)
            print(f"fixup! {spec}: {subjects(repo, f'{base}..HEAD')}")
    print()

    print("=== git blame on root lines, and with --root (r1)")
    with tempfile.TemporaryDirectory() as d:
        repo = new_repo(d)
        root = commit(repo, "feat: one", "a", "1\n2\n3\n")
        commit(repo, "feat: two", "b", "b\n")
        plain = git(repo, "blame", "-l", "-s", "-L", "2,2", "HEAD", "--", "a").split()[0]
        rooted = git(repo, "blame", "--root", "-l", "-s", "-L", "2,2", "HEAD", "--", "a").split()[0]
        print(f"root id:        {root} ({len(root)})\nblame:          {plain} ({len(plain)})\nblame --root:   {rooted} ({len(rooted)})\n")

    print("=== git blame cost: per line vs one range (r4)")
    with tempfile.TemporaryDirectory() as d:
        repo = new_repo(d)
        commit(repo, "feat: big", "a", "".join(f"{i}\n" for i in range(3000)))
        commit(repo, "feat: other", "b", "b\n")
        t = time.monotonic()
        for line in range(1, 3001):
            git(repo, "blame", "--root", "-l", "-s", "-L", f"{line},{line}", "HEAD", "--", "a")
        per_line = time.monotonic() - t
        t = time.monotonic()
        out = git(repo, "blame", "--root", "-l", "-s", "-L", "1,3000", "HEAD", "--", "a")
        one = time.monotonic() - t
        print(f"3000 single-line blames: {per_line:.2f} s; one 1,3000 blame: {one:.3f} s, {len(out.splitlines())} lines\n")


if __name__ == "__main__":
    main()
