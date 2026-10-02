"""End-to-end acceptance scenarios for the header-parsing and split-index fixes.

Usage: python3 acceptance.py GIR_BINARY
Prints one PASS or FAIL line per check, grouped by scenario, and exits 0 either way;
the verdicts are the result. Each scenario runs in a fresh temporary repository.
"""
import os
import subprocess
import sys
import tempfile
from pathlib import Path

GIR = str(Path(sys.argv[1]).resolve())
ENV = {
    **os.environ,
    "GIT_CONFIG_GLOBAL": os.devnull,
    "GIT_CONFIG_NOSYSTEM": "1",
    "GIT_AUTHOR_NAME": "t", "GIT_AUTHOR_EMAIL": "t@t",
    "GIT_COMMITTER_NAME": "t", "GIT_COMMITTER_EMAIL": "t@t",
    "GIR_INTERACTIVE": "0",
    "GIT_EDITOR": "true",
    "GIT_SEQUENCE_EDITOR": "true",
}
RESULTS = {"PASS": 0, "FAIL": 0}


def sh(cwd, *args, env=None, input=None):
    return subprocess.run(args, cwd=cwd, env=env or ENV, capture_output=True, text=True, input=input)


def git(cwd, *args):
    r = sh(cwd, "git", *args)
    if r.returncode:
        raise SystemExit(f"setup failed: git {' '.join(args)}: {r.stderr.strip()}")
    return r.stdout


def gir(cwd, *args, env=None, input=None):
    r = sh(cwd, GIR, *args, env=env, input=input)
    print(f"  $ gir {' '.join(args)} -> exit {r.returncode}: {(r.stdout + r.stderr).strip()!r}")
    return r


def check(label, ok, detail=""):
    verdict = "PASS" if ok else "FAIL"
    RESULTS[verdict] += 1
    print(f"  {verdict} {label}" + (f" ({detail})" if detail and not ok else ""))


def scenario(title):
    print(f"\n## {title}")


def repo(base=None):
    d = tempfile.mkdtemp(prefix="gir-accept-")
    git(d, "init", "-q", "-b", "main")
    for path, text in {"base.txt": "base\n", **(base or {})}.items():
        write(d, path, text)
    git(d, "add", "-A")
    git(d, "commit", "-q", "-m", "chore: base")
    git(d, "switch", "-q", "-c", "topic")
    return d


def write(d, path, text):
    p = Path(d, path)
    p.parent.mkdir(parents=True, exist_ok=True)
    p.write_text(text)


def commit(d, path, text, message):
    write(d, path, text)
    git(d, "add", "--", path)
    git(d, "commit", "-q", "-m", message)


def show(d, rev_path):
    r = sh(d, "git", "show", rev_path)
    return r.stdout if r.returncode == 0 else None


def subjects(d, rng="main..HEAD"):
    return git(d, "log", "--format=%s", rng).splitlines()


def rebase(d):
    r = sh(d, "git", "rebase", "-q", "-i", "--autosquash", "main")
    print(f"  $ git rebase -i --autosquash main -> exit {r.returncode}: {(r.stdout + r.stderr).strip()!r}")
    return r.returncode == 0


def view(d):
    return git(d, "status", "--short"), git(d, "ls-files", "-v")


def leftovers(d):
    gitdir = Path(git(d, "rev-parse", "--absolute-git-dir").strip())
    common = Path(git(d, "rev-parse", "--path-format=absolute", "--git-common-dir").strip())
    return sorted({p.name for g in (gitdir, common) for p in g.glob("gir-split-index-*")})


def refuse_hook(d, subject):
    hooks = Path(git(d, "rev-parse", "--path-format=absolute", "--git-path", "hooks").strip())
    hooks.mkdir(parents=True, exist_ok=True)
    hook = hooks / "commit-msg"
    hook.write_text(f"#!/bin/sh\n! grep -q '{subject}' \"$1\"\n")
    hook.chmod(0o755)


# ---------------------------------------------------------------- a2: header parsing, end to end

scenario("a2.1 delete '-- header', edit a later line; gir fixup; rebase --autosquash")
d = repo()
commit(d, "x.lua", "-- header\nlocal a = 1\nlocal b = 2\nlocal c = 3\nlocal d = 4\n", "feat: lua")
commit(d, "other.txt", "other\n", "feat: other")
write(d, "x.lua", "local a = 1\nlocal b = 2\nlocal c = 3\nlocal d = 5\n")
git(d, "add", "x.lua")
r = gir(d, "fixup")
check("gir fixup succeeds", r.returncode == 0)
check("rebase --autosquash succeeds", rebase(d))
check("history is feat: lua then feat: other", subjects(d) == ["feat: other", "feat: lua"], str(subjects(d)))
check("feat: lua holds the edited file", show(d, "HEAD~1:x.lua") == "local a = 1\nlocal b = 2\nlocal c = 3\nlocal d = 5\n", repr(show(d, "HEAD~1:x.lua")))

scenario("a2.2 delete '-- /dev/null' (commit A), edit a line of commit B; gir fixup without --split")
d = repo()
commit(d, "x.lua", "-- /dev/null\nl1\nl2\nl3\nl4\n", "feat: A")
commit(d, "x.lua", "-- /dev/null\nl1\nl2\nl3\nL4\n", "feat: B")
head = git(d, "rev-parse", "HEAD")
write(d, "x.lua", "l1\nl2\nl3\nL4x\n")
git(d, "add", "x.lua")
r = gir(d, "fixup")
check("gir refuses: the changes belong to two commits", r.returncode == 2 and "several commits" in r.stderr, r.stderr.strip())
check("no commit was created", git(d, "rev-parse", "HEAD") == head)

scenario("a2.3 the same case with --split; rebase --autosquash")
r = gir(d, "fixup", "--split")
check("gir fixup --split succeeds", r.returncode == 0)
check("rebase --autosquash succeeds", rebase(d))
check("history is feat: A then feat: B", subjects(d) == ["feat: B", "feat: A"], str(subjects(d)))
check("feat: A lost only its '-- /dev/null' line", show(d, "HEAD~1:x.lua") == "l1\nl2\nl3\nl4\n", repr(show(d, "HEAD~1:x.lua")))
check("feat: B holds the edit", show(d, "HEAD:x.lua") == "l1\nl2\nl3\nL4x\n", repr(show(d, "HEAD:x.lua")))

# ---------------------------------------------------------------- a3: split index entries, end to end


def two_targets(d):
    commit(d, "a.txt", "a\n", "feat: a")
    commit(d, "b.txt", "b\n", "feat: b")


def stage_two(d, prefix=""):
    write(d, prefix + "a.txt", "A\n")
    write(d, prefix + "b.txt", "B\n")
    git(d, "add", prefix + "a.txt", prefix + "b.txt")


scenario("a3.1 sparse checkout: split, rebase, then git add -A and git commit -a")
d = repo()
two_targets(d)
commit(d, "out/c.txt", "c\n", "feat: c")
git(d, "sparse-checkout", "set", "--no-cone", "/*", "!/out/")
stage_two(d)
r = gir(d, "fixup", "--split")
check("gir fixup --split succeeds", r.returncode == 0)
check("status is clean after the split", git(d, "status", "--short") == "", repr(git(d, "status", "--short")))
check("rebase --autosquash succeeds", rebase(d))
sh(d, "git", "add", "-A")
sh(d, "git", "commit", "-q", "-a", "-m", "chore: everything")
check("out/c.txt is still in HEAD", show(d, "HEAD:out/c.txt") == "c\n", repr(show(d, "HEAD:out/c.txt")))
check("out/c.txt is still skip-worktree", git(d, "ls-files", "-v", "out/c.txt").startswith("S "), git(d, "ls-files", "-v", "out/c.txt").strip())
check("history is feat: a, feat: b, feat: c", subjects(d) == ["feat: c", "feat: b", "feat: a"], str(subjects(d)))

scenario("a3.2 skip-worktree file with a private edit: split, rebase, then git add -A and git commit -a")
d = repo({"cfg.txt": "shared\n"})
two_targets(d)
git(d, "update-index", "--skip-worktree", "cfg.txt")
write(d, "cfg.txt", "private local edit\n")
stage_two(d)
r = gir(d, "fixup", "--split")
check("gir fixup --split succeeds", r.returncode == 0)
check("status is clean after the split", git(d, "status", "--short") == "", repr(git(d, "status", "--short")))
check("rebase --autosquash succeeds", rebase(d))
sh(d, "git", "add", "-A")
sh(d, "git", "commit", "-q", "-a", "-m", "chore: everything")
check("the private edit is not committed", show(d, "HEAD:cfg.txt") == "shared\n", repr(show(d, "HEAD:cfg.txt")))
check("the private edit is still on disk", Path(d, "cfg.txt").read_text() == "private local edit\n")
check("cfg.txt is still skip-worktree", git(d, "ls-files", "-v", "cfg.txt").startswith("S "), git(d, "ls-files", "-v", "cfg.txt").strip())

scenario("a3.3 intent-to-add file: split, then git add -A and git commit")
d = repo()
two_targets(d)
write(d, "ita.txt", "new, not staged\n")
git(d, "add", "-N", "ita.txt")
stage_two(d)
r = gir(d, "fixup", "--split")
check("gir fixup --split succeeds", r.returncode == 0)
check("ita.txt is still intent-to-add", git(d, "status", "--short") == " A ita.txt\n", repr(git(d, "status", "--short")))
check("ita.txt is not in HEAD", show(d, "HEAD:ita.txt") is None)
print("  note: git rebase refuses while an intent-to-add entry exists (unstaged change), on any build; the rebase is not part of this case")

# ---------------------------------------------------------------- a4: setups the fix was not tested in

scenario("a4.1 linked worktree with a skip-worktree file")
d = repo({"cfg.txt": "shared\n"})
two_targets(d)
wt = d + "-wt"
git(d, "worktree", "add", "-q", "-b", "wt", wt, "topic")
git(wt, "update-index", "--skip-worktree", "cfg.txt")
write(wt, "cfg.txt", "private local edit\n")
stage_two(wt)
before = view(wt)
r = gir(wt, "fixup", "--split")
check("gir fixup --split succeeds in the linked worktree", r.returncode == 0)
check("status and flags kept (apart from the committed files)", view(wt)[1] == before[1] and git(wt, "status", "--short") == "", repr(view(wt)))
check("no temporary index left in either git directory", leftovers(wt) == [], str(leftovers(wt)))
check("main worktree's index untouched (topic still clean)", git(d, "status", "--short") == "", repr(git(d, "status", "--short")))

scenario("a4.2 sparse checkout, cone mode, sparse index")
d = repo()
commit(d, "in/a.txt", "a\n", "feat: a")
commit(d, "in/b.txt", "b\n", "feat: b")
commit(d, "out/c.txt", "c\n", "feat: c")
git(d, "sparse-checkout", "set", "--cone", "--sparse-index", "in")
sparse_before = git(d, "ls-files", "--sparse")
stage_two(d, "in/")
r = gir(d, "fixup", "--split")
check("gir fixup --split succeeds", r.returncode == 0)
check("status is clean after the split", git(d, "status", "--short") == "", repr(git(d, "status", "--short")))
check("the index is still sparse with the same entries", git(d, "ls-files", "--sparse") == sparse_before, repr(git(d, "ls-files", "--sparse")))
check("rebase --autosquash succeeds", rebase(d))
check("out/c.txt is still in HEAD", show(d, "HEAD:out/c.txt") == "c\n")

scenario("a4.3 gir run from a subdirectory, skip-worktree file at the top")
d = repo({"cfg.txt": "shared\n"})
commit(d, "sub/a.txt", "a\n", "feat: a")
commit(d, "sub/b.txt", "b\n", "feat: b")
git(d, "update-index", "--skip-worktree", "cfg.txt")
write(d, "cfg.txt", "private local edit\n")
stage_two(d, "sub/")
flags = git(d, "ls-files", "-v")
r = gir(str(Path(d, "sub")), "fixup", "--split")
check("gir fixup --split succeeds from sub/", r.returncode == 0)
check("status clean and flags kept", git(d, "status", "--short") == "" and git(d, "ls-files", "-v") == flags, repr(view(d)))

for mode, prefix in (("amend", "amend!"), ("squash", "squash!")):
    scenario(f"a4.{4 if mode == 'amend' else 5} gir {mode} --split with a skip-worktree file")
    d = repo({"cfg.txt": "shared\n"})
    two_targets(d)
    git(d, "update-index", "--skip-worktree", "cfg.txt")
    write(d, "cfg.txt", "private local edit\n")
    stage_two(d)
    flags = git(d, "ls-files", "-v")
    r = gir(d, mode, "--split")
    check(f"gir {mode} --split succeeds", r.returncode == 0)
    check(f"two {prefix} commits", sorted(s.split(" ", 1)[0] for s in subjects(d, "-2")) == [prefix, prefix], str(subjects(d, "-2")))
    check("status clean and flags kept", git(d, "status", "--short") == "" and git(d, "ls-files", "-v") == flags, repr(view(d)))
    check("rebase --autosquash succeeds", rebase(d))
    check("history is feat: a, feat: b", subjects(d) == ["feat: b", "feat: a"], str(subjects(d)))

scenario("a4.6 the interactive picker's s, with a skip-worktree file")
d = repo({"cfg.txt": "shared\n"})
two_targets(d)
git(d, "update-index", "--skip-worktree", "cfg.txt")
write(d, "cfg.txt", "private local edit\n")
stage_two(d)
flags = git(d, "ls-files", "-v")
r = gir(d, "fixup", env={**ENV, "GIR_INTERACTIVE": "1"}, input="s\n")
check("the picker's s splits", r.returncode == 0 and len([s for s in subjects(d, "-2") if s.startswith("fixup!")]) == 2, str(subjects(d, "-2")))
check("status clean and flags kept", git(d, "status", "--short") == "" and git(d, "ls-files", "-v") == flags, repr(view(d)))

scenario("a4.7 no temporary index left after a successful and a failed split")
d = repo()
two_targets(d)
stage_two(d)
r = gir(d, "fixup", "--split")
check("successful split", r.returncode == 0)
check("no gir-split-index-* file after success", leftovers(d) == [], str(leftovers(d)))
d = repo()
two_targets(d)
stage_two(d)
refuse_hook(d, "fixup! feat: b")
r = gir(d, "fixup", "--split")
check("hook-refused split fails and restores", r.returncode == 2 and "restored HEAD and the index" in r.stderr, r.stderr.strip())
check("no gir-split-index-* file after failure", leftovers(d) == [], str(leftovers(d)))

print(f"\n{RESULTS['PASS']} PASS, {RESULTS['FAIL']} FAIL")
