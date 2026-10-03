#!/usr/bin/env python3
"""Probe: what a `reference-transaction` hook that exits 1 in the `prepared` state for
`refs/heads/main` leaves behind, for five operations, and what a recovery command then does.

Two hooks: `any` vetoes every update of `refs/heads/main`; `moves` vetoes only an update whose old
and new values differ, so a no-op update (as `git reset --hard HEAD` makes) passes.
Each case starts with `main` checked out, a tracked file `notes.txt` edited but not staged, and an
untracked file `scratch.txt`, so the effect on pre-existing uncommitted changes is visible.
Run: python3 probe-veto.py
"""
from __future__ import annotations

import os
import subprocess
import tempfile
from pathlib import Path

ENV = {**os.environ, "GIT_EDITOR": "true", "GIT_CONFIG_GLOBAL": os.devnull, "GIT_CONFIG_NOSYSTEM": "1"}
HOOKS = {
    "any": """#!/bin/sh
[ "$1" = prepared ] || exit 0
while read -r old new ref; do
  if [ "$ref" = refs/heads/main ]; then echo "hook: veto $old $new $ref" >&2; exit 1; fi
done
exit 0
""",
    "moves": """#!/bin/sh
[ "$1" = prepared ] || exit 0
while read -r old new ref; do
  if [ "$ref" = refs/heads/main ] && [ "$old" != "$new" ]; then echo "hook: veto $old $new $ref" >&2; exit 1; fi
done
exit 0
""",
}


def run(repo: Path, *args: str) -> tuple[int, str]:
    r = subprocess.run(["git", *args], cwd=repo, env=ENV, capture_output=True, text=True)
    return r.returncode, (r.stdout + r.stderr).strip()


def git(repo: Path, *args: str) -> str:
    code, out = run(repo, *args)
    if code:
        raise RuntimeError(f"git {' '.join(args)}: {out}")
    return out


def init(path: Path, bare: bool = False) -> Path:
    path.mkdir(parents=True, exist_ok=True)
    git(path, "init", "-q", "-b", "main", *(["--bare"] if bare else []))
    if not bare:
        git(path, "config", "user.email", "p@x")
        git(path, "config", "user.name", "p")
    return path


def commit(repo: Path, message: str, path: str, content: str) -> str:
    (repo / path).write_text(content)
    git(repo, "add", path)
    git(repo, "commit", "-q", "-m", message)
    return git(repo, "rev-parse", "HEAD")


def arm(repo: Path, dirty: bool, kind: str) -> None:
    """Install the vetoing hook and leave pre-existing uncommitted changes."""
    hook = repo / ".git" / "hooks" / "reference-transaction"
    hook.parent.mkdir(exist_ok=True)
    hook.write_text(HOOKS[kind])
    hook.chmod(0o755)
    if dirty:
        (repo / "notes.txt").write_text("notes edited before the operation\n")
        (repo / "scratch.txt").write_text("untracked before the operation\n")


def state(repo: Path, label: str) -> None:
    head = run(repo, "symbolic-ref", "-q", "HEAD")[1] or "detached at " + git(repo, "rev-parse", "--short", "HEAD")
    status = git(repo, "status", "--porcelain=v1", "--untracked-files=all").replace("\n", " | ")
    leftovers = [p for p in ["MERGE_HEAD", "rebase-merge", "rebase-apply", "AUTO_MERGE", "ORIG_HEAD"]
                 if (repo / ".git" / p).exists()]
    stashes = git(repo, "stash", "list").replace("\n", " | ")
    print(f"  {label}: main={git(repo, 'rev-parse', '--short', 'main')} HEAD={head}")
    print(f"    status: {status or '(clean)'}")
    print(f"    files: {sorted(p.name for p in repo.iterdir() if p.name != '.git')}")
    print(f"    state files: {leftovers}; stash: {stashes or '(none)'}")
    notes = (repo / "notes.txt").read_text().strip() if (repo / "notes.txt").exists() else "(missing)"
    scratch = (repo / "scratch.txt").exists()
    print(f"    notes.txt: {notes!r}; scratch.txt exists: {scratch}")


def case(name: str, setup, operation: list[str], kind: str, recovery: list[str], dirty: bool = True) -> None:
    with tempfile.TemporaryDirectory() as d:
        repo = setup(Path(d))
        arm(repo, dirty, kind)
        print(f"## {name} [hook: {kind}] then git {' '.join(recovery)}")
        state(repo, "before")
        code, out = run(repo, *operation)
        print(f"  $ git {' '.join(operation)} -> exit {code}")
        for line in out.splitlines():
            print(f"    | {line}")
        state(repo, "after veto")
        code, out = run(repo, *recovery)
        print(f"  $ git {' '.join(recovery)} -> exit {code}")
        for line in out.splitlines():
            print(f"    | {line}")
        state(repo, "after recovery")
        print()


def base_repo(d: Path) -> Path:
    repo = init(d / "repo")
    commit(repo, "chore: init", "notes.txt", "notes\n")
    return repo


def with_feature(d: Path) -> Path:
    repo = base_repo(d)
    git(repo, "switch", "-q", "-c", "feature")
    commit(repo, "feat: incoming", "incoming.txt", "incoming\n")
    git(repo, "switch", "-q", "main")
    return repo


def diverged(d: Path) -> Path:
    repo = with_feature(d)
    commit(repo, "feat: local", "local.txt", "local\n")
    return repo


def with_staged(d: Path) -> Path:
    repo = base_repo(d)
    (repo / "new.txt").write_text("new\n")
    git(repo, "add", "new.txt")
    return repo


def remote_ahead(d: Path) -> Path:
    remote = init(d / "remote.git", bare=True)
    other = init(d / "other")
    commit(other, "chore: init", "notes.txt", "notes\n")
    git(other, "remote", "add", "origin", str(remote))
    git(other, "push", "-q", "origin", "main")
    git(d, "clone", "-q", str(remote), "repo")
    repo = d / "repo"
    git(repo, "config", "user.email", "p@x")
    git(repo, "config", "user.name", "p")
    commit(other, "feat: incoming", "incoming.txt", "incoming\n")
    git(other, "push", "-q", "origin", "main")
    return repo


def main() -> None:
    print(git(Path("."), "--version") + "\n")
    reset = ["reset", "--hard", "HEAD"]
    for kind in ["any", "moves"]:
        case("k1: git merge --ff-only", with_feature, ["merge", "--ff-only", "feature"], kind, reset)
        case("k2: git commit", with_staged, ["commit", "-q", "-m", "feat: new"], kind, reset)
        case("k3: git merge --no-ff", diverged, ["merge", "--no-ff", "--no-edit", "feature"], kind, reset)
        case("k3: git merge --no-ff", diverged, ["merge", "--no-ff", "--no-edit", "feature"], kind, ["merge", "--abort"])
        case("k4: git pull (fast-forward)", remote_ahead, ["pull", "--ff-only", "-q"], kind, reset)
        case("k5: git rebase, clean tree", diverged, ["rebase", "feature"], kind, reset, dirty=False)
        case("k5: git rebase, clean tree", diverged, ["rebase", "feature"], kind, ["rebase", "--abort"], dirty=False)
        case("k5: git rebase --autostash, dirty tree", diverged, ["rebase", "--autostash", "feature"], kind, reset)
        case("k5: git rebase --autostash, dirty tree", diverged, ["rebase", "--autostash", "feature"], kind,
             ["rebase", "--abort"])
        case("k5: git rebase --autostash, dirty tree", diverged, ["rebase", "--autostash", "feature"], kind,
             ["rebase", "--quit"])


if __name__ == "__main__":
    main()
