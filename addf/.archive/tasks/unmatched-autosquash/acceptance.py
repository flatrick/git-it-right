#!/usr/bin/env python3
"""Acceptance for c1 and c2: run `gir lint --range`, follow its `try:` hint literally, and check that
no autosquash commit is left and the tree is unchanged.

Run from the repository root after `cargo build`: python3 addf/tasks/unmatched-autosquash/acceptance.py
The script is also its own GIT_SEQUENCE_EDITOR (--edit).
"""
from __future__ import annotations

import os
import re
import subprocess
import sys
import tempfile
from pathlib import Path

GIR = str(Path("target/debug/gir").resolve())
ENV = {**os.environ, "GIT_EDITOR": "true", "GIT_CONFIG_GLOBAL": os.devnull, "GIT_CONFIG_NOSYSTEM": "1"}
PREFIXES = ("fixup!", "squash!", "amend!")


def git(repo: Path, *args: str, env: dict | None = None) -> str:
    r = subprocess.run(["git", *args], cwd=repo, env=env or ENV, capture_output=True, text=True)
    if r.returncode:
        raise RuntimeError(f"git {' '.join(args)}: {r.stderr}")
    return r.stdout.strip()


def edit(todo: str, sha: str, below: str, action: str) -> None:
    """Give the line for `sha` the action `action` and, unless `below` is `-`, move it below `below`."""
    lines = Path(todo).read_text().splitlines()
    i = next(i for i, l in enumerate(lines) if l.split()[1:2] and sha.startswith(l.split()[1]))
    line = action + " " + lines.pop(i).split(" ", 1)[1]
    at = i if below == "-" else next(j for j, l in enumerate(lines) if l.split()[1:2] and below.startswith(l.split()[1])) + 1
    lines.insert(at, line)
    Path(todo).write_text("\n".join(lines) + "\n")


def commit(repo: Path, subject: str, content: str, path: str = "a") -> str:
    (repo / path).write_text(content)
    git(repo, "add", path)
    git(repo, "commit", "-q", "--no-verify", "-m", subject)
    return git(repo, "rev-parse", "HEAD")


def setup(repo: Path) -> str:
    git(repo, "init", "-q", "-b", "main")
    git(repo, "config", "user.email", "p@x")
    git(repo, "config", "user.name", "p")
    return commit(repo, "chore: init", "1\n2\n3\n")


def subjects(repo: Path, base: str) -> list[str]:
    out = git(repo, "log", "-z", "--format=%s", "--reverse", f"{base}..HEAD")
    return out.rstrip("\0").split("\0") if out else []


def follow(name: str, build, expected: list[str]) -> bool:
    """`build` returns the start of the range to lint, or None for the first commit."""
    with tempfile.TemporaryDirectory() as d:
        repo = Path(d)
        base = setup(repo)
        base = build(repo) or base
        tree = git(repo, "rev-parse", "HEAD^{tree}")
        before = subjects(repo, base)
        r = subprocess.run([GIR, "lint", "--range", f"{base}..HEAD"], cwd=repo, env=ENV, capture_output=True, text=True)
        tries = [l.strip()[len("try: "):] for l in r.stderr.splitlines() if l.strip().startswith("try: ")]
        auto = [l for l in r.stderr.splitlines() if "[fixup-unmatched]" in l or "[fixup-unsquashed]" in l]
        print(f"## {name}\nbefore: {before}\ngir:    {auto}\ntry:    {tries}")
        hint = next((t for t in tries if t.startswith("git rebase")), None)
        sha = next(l.split()[1] for l in auto)
        if hint is None:
            print("RESULT: FAIL no rebase hint\n")
            return False
        if hint.startswith("git rebase --autosquash "):
            cmd, env = ["git", "rebase", "--autosquash", hint.split()[-1]], {**ENV, "GIT_SEQUENCE_EDITOR": "true"}
        else:
            m = re.match(r"git rebase -i (\S+): (?:move it below (\S+) .*, change pick to (fixup -C|fixup|squash)|reword it into a normal commit)", hint)
            if not m:
                # Generic advice: follow its literal alternative, reword.
                m = re.match(r"git rebase -i (\S+): move it below the commit it belongs to", hint)
                below, action = "-", "reword"
            else:
                below, action = (m[2], m[3]) if m[2] else ("-", "reword")
            editor = f"{sys.executable} {Path(__file__).resolve()} --edit {sha} {below} {action.replace(' ', '_')}"
            cmd = ["git", "rebase", "-i", m[1]]
            env = {**ENV, "GIT_SEQUENCE_EDITOR": editor, "GIT_EDITOR": "sed -i '1s/.*/fix: correct a/'" if action == "reword" else "true"}
        rebase = subprocess.run(cmd, cwd=repo, env=env, capture_output=True, text=True)
        after = subjects(repo, base)
        left = [s for s in after if s.startswith(PREFIXES)]
        same = git(repo, "rev-parse", "HEAD^{tree}") == tree
        ok = rebase.returncode == 0 and not left and same and after == expected
        print(f"ran:    {' '.join(cmd)} -> exit {rebase.returncode} {rebase.stderr.strip()[:200]!r}")
        print(f"after:  {after}; expected: {expected}; tree unchanged: {same}")
        print(f"RESULT: {'PASS' if ok else 'FAIL'}\n")
        return ok


def reworded(prefix: str, body: str = ""):
    def build(repo: Path) -> None:
        commit(repo, "feat: add a", "1\nA\n3\n")
        commit(repo, "feat: add b", "b\n", "b")
        (repo / "a").write_text("1\nAA\n3\n")
        git(repo, "commit", "-qa", "--no-verify", "-m", f"{prefix} wip", *(["-m", body] if body else []))
    return build


def published(repo: Path) -> str:
    a = commit(repo, "feat: add a", "1\nA\n3\n")
    commit(repo, "fixup! feat: add a", "1\nAA\n3\n")
    return a


def new_file_only(repo: Path) -> None:
    commit(repo, "feat: add a", "1\nA\n3\n")
    commit(repo, "fixup! wip", "n\n", "new")


def foldable(repo: Path) -> None:
    commit(repo, "feat: add a", "1\nA\n3\n")
    commit(repo, "feat: add b", "b\n", "b")
    commit(repo, "fixup! feat: add a", "1\nAA\n3\n")


def main() -> int:
    if sys.argv[1:2] == ["--edit"]:
        sha, below, action, todo = sys.argv[2:6]
        edit(todo, sha, below, action.replace("_", " "))
        return 0
    print(subprocess.run(["git", "--version"], capture_output=True, text=True).stdout)
    ab = ["feat: add a", "feat: add b"]
    results = [
        follow("foldable fixup!", foldable, ab),
        follow("unmatched fixup! with its target in the range", reworded("fixup!"), ab),
        follow("unmatched squash! with its target in the range", reworded("squash!"), ab),
        follow("unmatched amend! with a new message, target in the range", reworded("amend!", "feat: add alpha"),
               ["feat: add alpha", "feat: add b"]),
        follow("unmatched amend! without a new message, target in the range", reworded("amend!"), ab),
        follow("unmatched fixup! with a published target", published, ["fix: correct a"]),
        follow("unmatched fixup! that only adds a file (generic advice)", new_file_only,
               ["feat: add a", "fix: correct a"]),
    ]
    print(f"{sum(results)}/{len(results)} passed")
    return 0 if all(results) else 1


if __name__ == "__main__":
    sys.exit(main())
