"""Acceptance for split-patch-fidelity: --split must commit exactly the staged bytes.

Usage: python3 acceptance.py GIR_BINARY
Every case commits a file in two commits, stages a change to a line of each, runs
`gir fixup --split` and `git rebase -i --autosquash main`, and compares each commit's
file bytes with the expected bytes. Prints PASS/FAIL per check; exits 0 either way.
The non-UTF-8 file name case runs only on Unix: Windows file names cannot hold such bytes.
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


def run(d, *args):
    return subprocess.run(args, cwd=d, env=ENV, capture_output=True)


def git(d, *args):
    r = run(d, "git", *args)
    if r.returncode:
        raise SystemExit(f"setup failed: git {args}: {r.stderr.decode(errors='replace')}")
    return r.stdout


def check(label, ok, detail=""):
    verdict = "PASS" if ok else "FAIL"
    RESULTS[verdict] += 1
    print(f"  {verdict} {label}" + (f" ({detail})" if detail and not ok else ""))


def case(title, name, first, second, staged, expect_first, expect_second, base_files=None, config=()):
    """first/second/staged/expect_*: file bytes. name: str (may carry surrogate escapes)."""
    print(f"\n## {title}")
    d = tempfile.mkdtemp(prefix="gir-bytes-")
    git(d, "init", "-q", "-b", "main")
    for key, value in config:
        git(d, "config", key, value)
    for path, data in {"base.txt": b"base\n", **(base_files or {})}.items():
        Path(d, path).write_bytes(data)
    git(d, "add", "-A")
    git(d, "commit", "-q", "-m", "chore: base")
    git(d, "switch", "-q", "-c", "topic")
    for data, msg in ((first, "feat: first"), (second, "feat: second")):
        Path(d, name).write_bytes(data)
        git(d, "add", "-A")
        git(d, "commit", "-q", "-m", msg)
    Path(d, name).write_bytes(staged)
    git(d, "add", "-A")
    r = run(d, GIR, "fixup", "--split")
    print(f"  $ gir fixup --split -> exit {r.returncode}: {(r.stdout + r.stderr).decode(errors='replace').strip()!r}")
    check("gir fixup --split succeeds", r.returncode == 0)
    if r.returncode:
        return
    check("HEAD's tree equals the staged tree", git(d, "status", "--porcelain") == b"", git(d, "status", "--porcelain"))
    rb = run(d, "git", "rebase", "-q", "-i", "--autosquash", "main")
    check("rebase --autosquash succeeds", rb.returncode == 0, rb.stderr.decode(errors="replace").strip())
    subjects = git(d, "log", "--format=%s", "main..HEAD").decode().splitlines()
    check("history is feat: first, feat: second", subjects == ["feat: second", "feat: first"], str(subjects))
    for rev, want in (("HEAD~1", expect_first), ("HEAD", expect_second)):
        got = run(d, "git", "show", f"{rev}:{name}").stdout
        check(f"{rev} holds exactly the expected bytes", got == want, f"{got!r} != {want!r}")


lines = lambda *ls, sep=b"\n", end=True: sep.join(ls) + (sep if end else b"")

case("c1 CRLF line endings (* -text)", "f.txt",
     lines(b"one", b"two", b"three", b"four", sep=b"\r\n"),
     lines(b"one", b"two", b"three", b"FOUR", sep=b"\r\n"),
     lines(b"ONE", b"two", b"three", b"FOURx", sep=b"\r\n"),
     lines(b"ONE", b"two", b"three", b"four", sep=b"\r\n"),
     lines(b"ONE", b"two", b"three", b"FOURx", sep=b"\r\n"),
     base_files={".gitattributes": b"* -text\n"})

case("c2 Latin-1 content", "f.txt",
     lines(b"caf\xe9 1", b"l2", b"l3", b"l4"),
     lines(b"caf\xe9 1", b"l2", b"l3", b"L4 \xe9"),
     lines(b"CAF\xc9 1", b"l2", b"l3", b"L4x \xe9"),
     lines(b"CAF\xc9 1", b"l2", b"l3", b"l4"),
     lines(b"CAF\xc9 1", b"l2", b"l3", b"L4x \xe9"))

case("c3a trailing whitespace, last staged line ending in spaces", "f.txt",
     lines(b"one", b"two", b"three", b"four"),
     lines(b"one", b"two", b"three", b"FOUR"),
     lines(b"ONE  ", b"two", b"three", b"FOURx   "),
     lines(b"ONE  ", b"two", b"three", b"four"),
     lines(b"ONE  ", b"two", b"three", b"FOURx   "))

case("c3b last staged line ending in spaces, no newline at end of file", "f.txt",
     lines(b"one", b"two", b"three", b"four"),
     lines(b"one", b"two", b"three", b"FOUR"),
     lines(b"ONE", b"two", b"three", b"FOURx   ", end=False),
     lines(b"ONE", b"two", b"three", b"four"),
     lines(b"ONE", b"two", b"three", b"FOURx   ", end=False))

for setting in ("error", "fix"):
    case(f"c4 apply.whitespace={setting} with trailing whitespace", "f.txt",
         lines(b"one", b"two", b"three", b"four"),
         lines(b"one", b"two", b"three", b"FOUR"),
         lines(b"ONE  ", b"two", b"three", b"FOURx   "),
         lines(b"ONE  ", b"two", b"three", b"four"),
         lines(b"ONE  ", b"two", b"three", b"FOURx   "),
         config=(("apply.whitespace", setting),))

case("c4 a textconv driver for the file", "f.txt",
     lines(b"one", b"two", b"three", b"four"),
     lines(b"one", b"two", b"three", b"FOUR"),
     lines(b"ONE", b"two", b"three", b"FOURx"),
     lines(b"ONE", b"two", b"three", b"four"),
     lines(b"ONE", b"two", b"three", b"FOURx"),
     base_files={".gitattributes": b"*.txt diff=shout\n"},
     config=(("diff.shout.textconv", "sed s/^/SHOUT:/"),))

if os.name != "nt":
    case("c7 non-UTF-8 file name (Unix only)", os.fsdecode(b"caf\xe9.txt"),
         lines(b"one", b"two", b"three", b"four"),
         lines(b"one", b"two", b"three", b"FOUR"),
         lines(b"ONE", b"two", b"three", b"FOURx"),
         lines(b"ONE", b"two", b"three", b"four"),
         lines(b"ONE", b"two", b"three", b"FOURx"))

print(f"\n{RESULTS['PASS']} PASS, {RESULTS['FAIL']} FAIL")
