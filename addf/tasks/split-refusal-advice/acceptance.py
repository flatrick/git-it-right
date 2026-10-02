"""Acceptance for split-refusal-advice: --split asks what it cannot decide, or refuses with advice that works.

Usage: python3 acceptance.py GIR_BINARY
Prints PASS/FAIL per check and exits 0 either way. "Terminal" runs use GIR_INTERACTIVE=1
with answers piped on stdin; "no terminal" runs use GIR_INTERACTIVE=0.
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
RESULTS = {"PASS": 0, "FAIL": 0}


def sh(d, *args, env=None, input=None):
    return subprocess.run(args, cwd=d, env=env or ENV, capture_output=True, input=input)


def git(d, *args):
    r = sh(d, "git", *args)
    if r.returncode:
        raise SystemExit(f"setup failed: git {args}: {r.stderr.decode(errors='replace')}")
    return r.stdout.decode(errors="replace")


def gir(d, *args, answers=None):
    env = {**ENV, "GIR_INTERACTIVE": "1" if answers is not None else "0"}
    r = sh(d, GIR, *args, env=env, input=(answers or "").encode())
    text = (r.stdout + r.stderr).decode(errors="replace")
    print(f"  $ {'GIR_INTERACTIVE=1 ' if answers is not None else ''}gir {' '.join(args)}"
          f"{' <<< ' + repr(answers) if answers is not None else ''} -> exit {r.returncode}:\n    " + text.strip().replace("\n", "\n    "))
    return r.returncode, text


def check(label, ok, detail=""):
    verdict = "PASS" if ok else "FAIL"
    RESULTS[verdict] += 1
    print(f"  {verdict} {label}" + (f" ({detail})" if detail and not ok else ""))


def scenario(title):
    print(f"\n## {title}")


def repo():
    d = tempfile.mkdtemp(prefix="gir-advice-")
    git(d, "init", "-q", "-b", "main")
    Path(d, "base.txt").write_text("base\n")
    git(d, "add", "-A")
    git(d, "commit", "-q", "-m", "chore: base")
    git(d, "switch", "-q", "-c", "topic")
    for name, msg in (("a.txt", "feat: a"), ("b.txt", "feat: b")):
        Path(d, name).write_text(name[0] + "\n")
        git(d, "add", name)
        git(d, "commit", "-q", "-m", msg)
    return d


def stage_ab(d):
    Path(d, "a.txt").write_text("A\n")
    Path(d, "b.txt").write_text("B\n")
    git(d, "add", "a.txt", "b.txt")


def rebase(d):
    r = sh(d, "git", "rebase", "-q", "-i", "--autosquash", "main")
    return r.returncode == 0


def show(d, spec):
    r = sh(d, "git", "show", spec)
    return r.stdout.decode(errors="replace") if r.returncode == 0 else None


def untraceable(kind, d):
    """Stages a change gir cannot trace; returns its path."""
    if kind == "new file":
        Path(d, "new.txt").write_text("new\n")
        git(d, "add", "new.txt")
        return "new.txt"
    if kind == "binary file":
        Path(d, "bin.dat").write_bytes(b"\x00\x01old")
        git(d, "add", "bin.dat")
        git(d, "commit", "-q", "-m", "feat: bin")
        Path(d, "bin.dat").write_bytes(b"\x00\x01new")
        git(d, "add", "bin.dat")
        return "bin.dat"
    if kind == "mode-only change":
        git(d, "update-index", "--chmod=+x", "a.txt")
        return "a.txt"
    raise ValueError(kind)


for kind in ("new file", "binary file", "mode-only change"):
    scenario(f"c1 no terminal, --split, {kind}")
    d = repo()
    path = untraceable(kind, d)
    if kind != "mode-only change":
        stage_ab(d)
    else:
        Path(d, "b.txt").write_text("B\n")
        git(d, "add", "b.txt")
    head = git(d, "rev-parse", "HEAD")
    code, text = gir(d, "fixup", "--split")
    check("exit 2", code == 2)
    check("nothing committed", git(d, "rev-parse", "HEAD") == head)
    check(f"the message names {path}", path in text)
    check("the message mentions --split", "--split" in text)
    check("no 'pass one: gir fixup <commit>' advice", "pass one" not in text)

scenario("c2 terminal, --split, a new file: asked for that file only, then split")
d = repo()
untraceable("new file", d)
stage_ab(d)
# the question lists the branch's newest commits first: 1) feat: b, 2) feat: a
code, text = gir(d, "fixup", "--split", answers="2\n")
check("exit 0", code == 0)
check("asked once, about new.txt", text.count("pick [") == 1 and "new.txt" in text, text.count("pick ["))
check("two fixup! commits were created", git(d, "log", "--format=%s", "-2").count("fixup!") == 2)
check("rebase --autosquash succeeds", rebase(d))
check("history is feat: a, feat: b", git(d, "log", "--format=%s", "main..HEAD").split() == ["feat:", "b", "feat:", "a"])
check("new.txt is in feat: a", show(d, "HEAD~1:new.txt") == "new\n", repr(show(d, "HEAD~1:new.txt")))
check("feat: a holds A, feat: b holds B", show(d, "HEAD~1:a.txt") == "A\n" and show(d, "HEAD:b.txt") == "B\n")

scenario("c2 terminal, --split, a binary file: asked for that file, then split")
d = repo()
untraceable("binary file", d)
stage_ab(d)
# 1) feat: bin, 2) feat: b, 3) feat: a
code, text = gir(d, "fixup", "--split", answers="1\n")
check("exit 0", code == 0)
check("rebase --autosquash succeeds", rebase(d))
check("bin.dat's staged bytes are in feat: bin", sh(d, "git", "show", "HEAD:bin.dat").stdout == b"\x00\x01new")
check("feat: a holds A, feat: b holds B", show(d, "HEAD~2:a.txt") == "A\n" and show(d, "HEAD~1:b.txt") == "B\n")


def insertion_repo():
    d = repo()
    Path(d, "f.txt").write_text("one\n")
    git(d, "add", "f.txt"); git(d, "commit", "-q", "-m", "feat: one")
    Path(d, "f.txt").write_text("one\ntwo\n")
    git(d, "add", "f.txt"); git(d, "commit", "-q", "-m", "feat: two")
    Path(d, "f.txt").write_text("one\nmid\ntwo\n")
    Path(d, "a.txt").write_text("A\n")
    git(d, "add", "f.txt", "a.txt")
    return d


scenario("c3 no terminal, --split, an insertion between two commits' lines")
d = insertion_repo()
head = git(d, "rev-parse", "HEAD")
code, text = gir(d, "fixup", "--split")
check("exit 2", code == 2)
check("nothing committed", git(d, "rev-parse", "HEAD") == head)
check("names the hunk and both commits", "f.txt:1" in text and "feat: one" in text and "feat: two" in text)
check("no git add -p advice", "add -p" not in text)

scenario("c3 terminal, --split, an insertion between two commits' lines: asked, then split")
d = insertion_repo()
# the question for f.txt:1 lists the line above's commit first; 2 = the line below's, feat: two
code, text = gir(d, "fixup", "--split", answers="2\n")
check("exit 0", code == 0)
check("asked about f.txt:1", "pick [" in text and "f.txt:1" in text)
check("two fixup! commits were created", git(d, "log", "--format=%s", "-2").count("fixup!") == 2)
check("rebase --autosquash succeeds", rebase(d))
log = git(d, "log", "--format=%s", "main..HEAD").splitlines()
check("history keeps four commits", log == ["feat: two", "feat: one", "feat: b", "feat: a"], str(log))
mid_in = [c for c, rev in (("feat: one", "HEAD~1"), ("feat: two", "HEAD")) if "mid" in (show(d, f"{rev}:f.txt") or "")]
check("mid went into exactly one commit, and stays in HEAD", len(mid_in) >= 1 and show(d, "HEAD:f.txt") == "one\nmid\ntwo\n", str(mid_in))
check("feat: a holds A", show(d, "HEAD~3:a.txt") == "A\n")

scenario("c3 terminal, picker s, an insertion between two commits' lines: asked, then split")
d = insertion_repo()
code, text = gir(d, "fixup", answers="s\n2\n")
check("exit 0", code == 0)
check("the picker offered s", "s) split" in text)
check("asked about f.txt:1 after s", text.count("pick [") == 2 and "f.txt:1" in text.split("s) split", 1)[-1])
check("the split went ahead: one fixup! each for feat: a and the chosen commit", git(d, "log", "--format=%s", "-3").count("fixup!") == 2)

scenario("c4 the picker does not offer s when the split cannot go ahead")
d = repo()
Path(d, "f.txt").write_text("one\n")
git(d, "add", "f.txt"); git(d, "commit", "-q", "-m", "feat: one")
Path(d, "f.txt").write_text("one\ntwo\n")
git(d, "add", "f.txt"); git(d, "commit", "-q", "-m", "feat: two")
Path(d, "f.txt").write_text("ONE\nTWO\n")
Path(d, "a.txt").write_text("A\n")
git(d, "add", "f.txt", "a.txt")
code, text = gir(d, "fixup", answers="q\n")
check("the picker was shown", "pick [" in text)
check("s is not offered (f.txt:1 changes lines of two commits in one hunk)", "s) split" not in text)

print(f"\n{RESULTS['PASS']} PASS, {RESULTS['FAIL']} FAIL")
