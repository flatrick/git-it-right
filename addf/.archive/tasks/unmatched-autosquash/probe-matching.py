#!/usr/bin/env python3
"""Probe: which autosquash subjects does `git rebase --autosquash <base>` fold, with the git on PATH?

Each case builds a throwaway repository, runs a non-interactive autosquash rebase onto `main`,
and reports whether any `fixup!`/`squash!`/`amend!` commit is left. Run: python3 probe-matching.py
"""
from __future__ import annotations

import os
import subprocess
import sys
import tempfile
from pathlib import Path

ENV = {**os.environ, "GIT_SEQUENCE_EDITOR": "true", "GIT_EDITOR": "true",
       "GIT_CONFIG_GLOBAL": os.devnull, "GIT_CONFIG_NOSYSTEM": "1"}


def git(repo: Path, *args: str, check: bool = True) -> str:
    r = subprocess.run(["git", *args], cwd=repo, env=ENV, capture_output=True, text=True)
    if check and r.returncode:
        raise RuntimeError(f"git {' '.join(args)}: {r.stderr}")
    return r.stdout.strip()


class Repo:
    def __init__(self, root: Path):
        self.path = root
        git(root, "init", "-q", "-b", "main")
        git(root, "config", "user.email", "p@x")
        git(root, "config", "user.name", "p")
        self.n = 0

    def commit(self, subject: str, body: str = "") -> str:
        self.n += 1
        (self.path / f"f{self.n}").write_text(f"{self.n}\n")
        git(self.path, "add", "-A")
        git(self.path, "commit", "-q", "--no-verify", "-m", subject, *(["-m", body] if body else []))
        return git(self.path, "rev-parse", "HEAD")

    def subjects(self) -> list[str]:
        return git(self.path, "log", "--format=%s", "--reverse", "main..HEAD").splitlines()

    def autosquash(self) -> tuple[int, str]:
        r = subprocess.run(["git", "rebase", "--autosquash", "main"], cwd=self.path, env=ENV,
                           capture_output=True, text=True)
        return r.returncode, (r.stdout + r.stderr).strip().replace("\n", " | ")


def case(name: str, build, detail: bool = False) -> None:
    with tempfile.TemporaryDirectory() as d:
        repo = Repo(Path(d))
        repo.commit("chore: init")
        git(repo.path, "switch", "-q", "-c", "feature")
        build(repo)
        before = repo.subjects()
        code, out = repo.autosquash()
        after = repo.subjects()
        left = [s for s in after if s.startswith(("fixup!", "squash!", "amend!"))]
        print(f"## {name}")
        print(f"before: {before}")
        print(f"rebase: exit {code}: {out}")
        print(f"after:  {after}")
        if detail:
            print("files:  " + git(repo.path, "log", "--reverse", "--format=%s:", "--name-only", "main..HEAD")
                  .replace("\n\n", " | ").replace("\n", " "))
        print(f"RESULT: {'LEFT ' + str(left) if left else 'folded'}\n")


def reword_target_then_fixup_left(r: Repo) -> None:
    t = r.commit("feat: add a")
    r.commit("feat: add b")
    r.commit("fixup! feat: add a")
    # Reword the target without folding the fixup, as a `git rebase -i` reword would.
    subprocess.run(["git", "rebase", "-q", "--no-autosquash", "-i", "main"], cwd=r.path, check=True,
                   env={**ENV, "GIT_SEQUENCE_EDITOR": f"sed -i '1s/^pick/reword/'",
                        "GIT_EDITOR": "sed -i '1s/.*/feat: introduce a/'"})
    del t


def hash_subject_then_reword(r: Repo) -> None:
    t = r.commit("feat: add a")
    r.commit(f"fixup! {t}")
    subprocess.run(["git", "rebase", "-q", "--no-autosquash", "-i", "main"], cwd=r.path, check=True,
                   env={**ENV, "GIT_SEQUENCE_EDITOR": f"sed -i '1s/^pick/reword/'",
                        "GIT_EDITOR": "sed -i '1s/.*/feat: introduce a/'"})


def main() -> int:
    print(subprocess.run(["git", "--version"], capture_output=True, text=True).stdout)
    case("p1: squash! with no matching subject",
         lambda r: (r.commit("feat: add a"), r.commit("squash! wip"), r.commit("squash! wip")))
    case("exact subject", lambda r: (r.commit("feat: add a"), r.commit("fixup! feat: add a")))
    case("subject prefix", lambda r: (r.commit("feat: add a thing"), r.commit("fixup! feat: add a")))
    case("prefix is a whole-word prefix only?", lambda r: (r.commit("feat: add alpha"), r.commit("fixup! feat: add al")))
    case("full hash", lambda r: r.commit(f"fixup! {r.commit('feat: add a')}"))
    case("7-char hash", lambda r: r.commit(f"fixup! {r.commit('feat: add a')[:7]}"))
    case("ref name HEAD", lambda r: (r.commit("feat: add a"), r.commit("fixup! HEAD")))
    case("ref name feature~1",
         lambda r: (r.commit("feat: add a"), r.commit("feat: add b"), r.commit("fixup! feature~1")))
    case("chained prefixes (git commit --fixup on a fixup)",
         lambda r: (r.commit("feat: add a"), r.commit("fixup! feat: add a"), r.commit("fixup! fixup! feat: add a")))
    case("amend! and fixup! of the same target in one rebase",
         lambda r: (r.commit("feat: add a"), r.commit("amend! feat: add a", "feat: add alpha"),
                    r.commit("fixup! feat: add a")))
    case("target is on the base branch, outside main..HEAD",
         lambda r: (git(r.path, "switch", "-q", "main"), r.commit("feat: on main"),
                    git(r.path, "switch", "-q", "feature"), git(r.path, "rebase", "-q", "main"),
                    r.commit("fixup! feat: on main")))
    case("fixup older than its target subject",
         lambda r: (r.commit("fixup! feat: later"), r.commit("feat: later")))
    case("two commits share the target subject",
         lambda r: (r.commit("feat: dup"), r.commit("feat: dup"), r.commit("fixup! feat: dup")))
    case("target reworded by an earlier rebase (subject form)", reword_target_then_fixup_left)
    case("target reworded by an earlier rebase (hash form)", hash_subject_then_reword)
    case("subject with trailing space after prefix only", lambda r: (r.commit("feat: a"), r.commit("fixup! ")))
    case("ref expression followed by a space and text",
         lambda r: (r.commit("feat: add a"), r.commit("feat: add b"), r.commit("fixup! feature~1 x")))
    case("no-space specifier that is not a rev, used as a title prefix",
         lambda r: (r.commit("feat: add a"), r.commit("fixup! feat:")))
    case("exact title only on a later commit, prefix on an earlier one",
         lambda r: (r.commit("feat: add a thing"), r.commit("fixup! feat: add a"), r.commit("feat: add a")),
         detail=True)
    case("mixed chained prefixes: squash! fixup!",
         lambda r: (r.commit("feat: add a"), r.commit("squash! fixup! feat: add a")))
    case("mixed chained prefixes: amend! fixup!",
         lambda r: (r.commit("feat: add a"), r.commit("amend! fixup! feat: add a", "feat: add a")))
    case("prefix without its space: fixup!feat",
         lambda r: (r.commit("feat: add a"), r.commit("fixup!feat: add a")))
    case("abbreviated hash of a commit outside the range",
         lambda r: (r.commit("feat: add a"), r.commit("fixup! " + git(r.path, "rev-parse", "--short", "main"))))
    return 0


if __name__ == "__main__":
    sys.exit(main())
