#!/usr/bin/env python3
"""Probe: for an autosquash commit that will not fold, can its intended target be found by blaming the
lines it changes, and does advice built from that leave no autosquash commit when followed?

Run: python3 probe-advice.py   (the script also serves as its own GIT_SEQUENCE_EDITOR with --edit)
"""
from __future__ import annotations

import os
import re
import subprocess
import sys
import tempfile
from pathlib import Path

ENV = {**os.environ, "GIT_EDITOR": "true", "GIT_CONFIG_GLOBAL": os.devnull, "GIT_CONFIG_NOSYSTEM": "1"}
PREFIXES = ("fixup!", "squash!", "amend!")


def git(repo: Path, *args: str, env: dict | None = None) -> str:
    r = subprocess.run(["git", *args], cwd=repo, env=env or ENV, capture_output=True, text=True)
    if r.returncode:
        raise RuntimeError(f"git {' '.join(args)}: {r.stderr}")
    return r.stdout.strip()


def edit_todo(path: str, mover: str, target: str, action: str) -> None:
    """--edit mode: move the line for `mover` right below `target` and give it `action`;
    when both name the same commit, only change its action."""
    lines = Path(path).read_text().splitlines()
    moved = next(l for l in lines if l.split()[1:2] and mover.startswith(l.split()[1]))
    if mover == target:
        lines[lines.index(moved)] = action + " " + moved.split(" ", 1)[1]
        Path(path).write_text("\n".join(lines) + "\n")
        return
    lines.remove(moved)
    at = next(i for i, l in enumerate(lines) if l.split()[1:2] and target.startswith(l.split()[1]))
    lines.insert(at + 1, action + " " + moved.split(" ", 1)[1])
    Path(path).write_text("\n".join(lines) + "\n")


def blame_targets(repo: Path, commit: str) -> set[str]:
    """Commits that last changed, before `commit`, the lines `commit` changes (removed or modified lines)."""
    diff = git(repo, "diff", "-U0", f"{commit}^", commit)
    found, path = set(), None
    for line in diff.splitlines():
        if line.startswith("--- "):
            path = None if line == "--- /dev/null" else line[6:]
        m = re.match(r"@@ -(\d+)(?:,(\d+))? ", line)
        if m and path:
            start, count = int(m[1]), int(m[2] if m[2] is not None else 1)
            if count == 0:
                continue
            out = git(repo, "blame", "--porcelain", "-L", f"{start},{start + count - 1}", f"{commit}^", "--", path)
            found |= {l.split()[0] for l in out.splitlines() if re.match(r"^[0-9a-f]{40} ", l)}
    return found


def subjects(repo: Path) -> list[str]:
    return git(repo, "log", "--format=%s", "--reverse", "main..HEAD").splitlines()


def scenario(name: str, build, advice: str) -> None:
    with tempfile.TemporaryDirectory() as d:
        repo = Path(d)
        git(repo, "init", "-q", "-b", "main")
        git(repo, "config", "user.email", "p@x")
        git(repo, "config", "user.name", "p")
        (repo / "a").write_text("1\n2\n3\n")
        git(repo, "add", "a")
        git(repo, "commit", "-q", "-m", "chore: init")
        git(repo, "switch", "-q", "-c", "feature")
        fixup = build(repo)
        tree = git(repo, "rev-parse", "HEAD^{tree}")
        print(f"## {name}")
        print(f"before: {subjects(repo)}")
        targets = blame_targets(repo, fixup)
        named = {t[:10]: git(repo, "log", "-1", "--format=%s", t) for t in targets}
        on_base = [t[:10] for t in targets
                   if subprocess.run(["git", "merge-base", "--is-ancestor", t, "main"], cwd=repo).returncode == 0]
        print(f"blame:  {named or 'no target found'}; on base: {on_base}")
        if advice == "move" and len(targets) == 1 and not on_base:
            editor = f"{sys.executable} {Path(__file__).resolve()} --edit {fixup} {targets.pop()} fixup"
        elif advice == "reword":
            editor = f"{sys.executable} {Path(__file__).resolve()} --edit {fixup} {fixup} reword"
        else:
            print("advice: none applicable\n")
            return
        env = {**ENV, "GIT_SEQUENCE_EDITOR": editor, "GIT_EDITOR": "sed -i '1s/.*/fix: correct a/'"}
        r = subprocess.run(["git", "rebase", "-q", "-i", "main"], cwd=repo, env=env, capture_output=True, text=True)
        after = subjects(repo)
        left = [s for s in after if s.split(" ", 1)[0] in PREFIXES]
        print(f"advice: {advice}; rebase exit {r.returncode} {r.stderr.strip()}")
        print(f"after:  {after}; tree unchanged: {git(repo, 'rev-parse', 'HEAD^{tree}') == tree}")
        print(f"RESULT: {'LEFT ' + str(left) if left else 'no autosquash commit left'}\n")


def commit(repo: Path, subject: str, content: str, path: str = "a") -> str:
    (repo / path).write_text(content)
    git(repo, "add", path)
    git(repo, "commit", "-q", "--no-verify", "-m", subject)
    return git(repo, "rev-parse", "HEAD")


def reword_first(repo: Path, title: str) -> None:
    env = {**ENV, "GIT_SEQUENCE_EDITOR": "sed -i '1s/^pick/reword/'", "GIT_EDITOR": f"sed -i '1s/.*/{title}/'"}
    git(repo, "rebase", "-q", "--no-autosquash", "-i", "main", env=env)


def reworded_target(repo: Path) -> str:
    commit(repo, "feat: add a", "1\nA\n3\n")
    commit(repo, "feat: add b", "B\n", "b")
    commit(repo, "fixup! feat: add a", "1\nAA\n3\n")
    reword_first(repo, "feat: introduce a")
    return git(repo, "rev-parse", "HEAD")


def hand_written_wip(repo: Path) -> str:
    commit(repo, "feat: add a", "1\nA\n3\n")
    return commit(repo, "squash! wip", "1\nA2\n3\n")


def published_target(repo: Path) -> str:
    git(repo, "switch", "-q", "main")
    commit(repo, "feat: add a", "1\nA\n3\n")
    git(repo, "switch", "-q", "feature")
    git(repo, "rebase", "-q", "main")
    return commit(repo, "fixup! feat: add a", "1\nAA\n3\n")


def new_file_only(repo: Path) -> str:
    commit(repo, "feat: add a", "1\nA\n3\n")
    return commit(repo, "fixup! feat: gone", "new\n", "c")


def two_targets(repo: Path) -> str:
    commit(repo, "feat: add a", "1\nA\n3\n")
    commit(repo, "feat: add c", "1\nA\nC\n")
    return commit(repo, "fixup! feat: gone", "1\nAX\nCX\n")


def main() -> int:
    if sys.argv[1:2] == ["--edit"]:
        mover, target, action, todo = sys.argv[2:6]
        edit_todo(todo, mover, target, action)
        return 0
    print(subprocess.run(["git", "--version"], capture_output=True, text=True).stdout)
    scenario("target reworded by an earlier rebase", reworded_target, "move")
    scenario("hand-written squash! wip changing a line of feat: add a", hand_written_wip, "move")
    scenario("target already on the base branch: move is impossible", published_target, "move")
    scenario("target already on the base branch: reword into a real commit", published_target, "reword")
    scenario("fixup only adds a new file", new_file_only, "move")
    scenario("fixup changes lines of two commits", two_targets, "move")
    return 0


if __name__ == "__main__":
    sys.exit(main())
