"""Probe for unfoldable-fixup-targets: what a plain autosquash folds, and what gir targets today.

Usage: python3 probe.py GIR_BINARY
Prints observations per scenario and exits 0. Picker runs use GIR_INTERACTIVE=1 with an
empty answer (cancel) on stdin; other runs use GIR_INTERACTIVE=0.
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
    "GIR_INTERACTIVE": "0", "GIT_EDITOR": "true", "GIT_SEQUENCE_EDITOR": "true",
}


def sh(d, *args, env=None, input=None):
    return subprocess.run(args, cwd=d, env=env or ENV, capture_output=True, input=input)


def git(d, *args, ok=(0,)):
    r = sh(d, "git", *args)
    if r.returncode not in ok:
        raise SystemExit(f"setup failed: git {args}: {r.stderr.decode(errors='replace')}")
    return r.stdout.decode(errors="replace").strip()


def gir(d, *args, answers=None):
    env = {**ENV, "GIR_INTERACTIVE": "1" if answers is not None else "0"}
    r = sh(d, GIR, *args, env=env, input=(answers or "").encode())
    text = (r.stdout + r.stderr).decode(errors="replace")
    print(f"  $ {'GIR_INTERACTIVE=1 ' if answers is not None else ''}gir {' '.join(args)}"
          f"{' <<< ' + repr(answers) if answers is not None else ''} -> exit {r.returncode}:\n    " + text.strip().replace("\n", "\n    "))
    return r.returncode, text


def write(d, name, text):
    Path(d, name).write_text(text)


def commit(d, name, text, msg):
    write(d, name, text)
    git(d, "add", name)
    git(d, "commit", "-q", "-m", msg)
    return git(d, "rev-parse", "HEAD")


def subjects(d):
    return git(d, "log", "--format=%s", "main..HEAD").splitlines()


def scenario(title):
    print(f"\n## {title}")


def repo(side_from):
    """main: chore: base. topic: feat: a, then a --no-ff merge of side (feat: side), then feat: b.
    side forks from main's tip or from topic's feat: a."""
    d = tempfile.mkdtemp(prefix="gir-merge-")
    git(d, "init", "-q", "-b", "main")
    commit(d, "base.txt", "base\n", "chore: base")
    git(d, "switch", "-q", "-c", "topic")
    commit(d, "a.txt", "a\n", "feat: a")
    git(d, "switch", "-q", "-c", "side", "main" if side_from == "main" else "topic")
    side = commit(d, "s.txt", "s\n", "feat: side")
    git(d, "switch", "-q", "topic")
    git(d, "merge", "-q", "--no-ff", "--no-edit", "side")
    merge = git(d, "rev-parse", "HEAD")
    commit(d, "b.txt", "b\n", "feat: b")
    return d, side, merge


def autosquash(d, path, text, target, label):
    write(d, path, text)
    git(d, "add", path)
    git(d, "commit", "-q", f"--fixup={target}")
    print(f"  before: {subjects(d)}")
    r = sh(d, "git", "rebase", "-q", "-i", "--autosquash", "main")
    print(f"  git rebase -i --autosquash main -> exit {r.returncode} {r.stderr.decode(errors='replace').strip()}")
    after = subjects(d)
    print(f"  after:  {after}")
    left = any(s.startswith("fixup!") for s in after)
    print(f"  OBSERVED {label}: {'LEFT unfolded' if left else 'FOLDED'}; {path} at HEAD = {git(d, 'show', 'HEAD:' + path)!r}")


for side_from in ("main", "topic"):
    scenario(f"p1: picker after a --no-ff merge (side forked from {side_from})")
    d, side, merge = repo(side_from)
    print(f"  merge {merge[:10]}, side commit {side[:10]}")
    code, text = gir(d, "reword", answers="\n")
    print(f"  OBSERVED picker lists merge: {merge[:10] in text}; lists side commit: {side[:10] in text}")

    scenario(f"c1: autosquash of a fixup! of the merge (side forked from {side_from})")
    d, side, merge = repo(side_from)
    autosquash(d, "a.txt", "A\n", merge, "fixup! of merge")

    scenario(f"c1: autosquash of a fixup! of the side-branch commit (side forked from {side_from})")
    d, side, merge = repo(side_from)
    autosquash(d, "s.txt", "S\n", side, "fixup! of side commit")

scenario("explicit target: gir fixup --dry-run <merge>")
d, side, merge = repo("main")
write(d, "a.txt", "A\n")
git(d, "add", "a.txt")
gir(d, "fixup", "--dry-run", merge)
print(f"  (merge is {merge[:10]})")

scenario("automatic selection: a staged line last changed by a conflict-resolving merge")
d = tempfile.mkdtemp(prefix="gir-evil-")
git(d, "init", "-q", "-b", "main")
commit(d, "f.txt", "one\ntwo\nthree\n", "chore: base")
git(d, "switch", "-q", "-c", "topic")
commit(d, "f.txt", "one\ntwo-topic\nthree\n", "feat: topic")
git(d, "switch", "-q", "-c", "side", "main")
commit(d, "f.txt", "one\ntwo-side\nthree\n", "feat: side")
git(d, "switch", "-q", "topic")
git(d, "merge", "-q", "--no-ff", "--no-edit", "side", ok=(0, 1))
write(d, "f.txt", "one\ntwo-merged\nthree\n")
git(d, "add", "f.txt")
git(d, "commit", "-q", "--no-edit")
merge = git(d, "rev-parse", "HEAD")
print(f"  merge {merge[:10]}; git blame -L 2,2: {git(d, 'blame', '-l', '-s', '-L', '2,2', 'HEAD', '--', 'f.txt')}")
write(d, "f.txt", "one\nTWO\nthree\n")
git(d, "add", "f.txt")
code, text = gir(d, "fixup", "--dry-run")
print(f"  OBSERVED automatic selection picks the merge: {merge[:10] in text}")
code, text = gir(d, "fixup", "--split", "--dry-run")
print(f"  OBSERVED --split picks the merge: {merge[:10] in text}")
