"""Acceptance for doctor-fix-file-mode: --fix makes fixed scripts executable on disk as well as in git.

Usage: python3 acceptance.py GIR_BINARY
Prints PASS/FAIL per check and exits 0 either way.
Each check states the behavior wanted after the fix; on the build before it, the FAILs are the defect.
On Windows, which has no executable bit, the on-disk checks become "nothing changed on disk".
"""
import os
import stat
import subprocess
import sys
import tempfile
from pathlib import Path

GIR = str(Path(sys.argv[1]).resolve())
UNIX = os.name != "nt"
ENV = {
    **os.environ,
    "GIT_CONFIG_GLOBAL": os.devnull, "GIT_CONFIG_NOSYSTEM": "1",
    "GIT_AUTHOR_NAME": "t", "GIT_AUTHOR_EMAIL": "t@t", "GIT_COMMITTER_NAME": "t", "GIT_COMMITTER_EMAIL": "t@t",
}
RESULTS = {"PASS": 0, "FAIL": 0}
HOOK = b"#!/bin/sh\n: > .git/pre-commit-ran\n"
SCRIPT = b"#!/bin/sh\necho run\n"


def sh(d, *args):
    return subprocess.run(args, cwd=d, env=ENV, capture_output=True)


def git(d, *args):
    r = sh(d, "git", *args)
    if r.returncode:
        raise SystemExit(f"setup failed: git {args}: {r.stderr.decode(errors='replace')}")
    return r.stdout.decode(errors="replace")


def check(label, ok, detail=""):
    verdict = "PASS" if ok else "FAIL"
    RESULTS[verdict] += 1
    print(f"  {verdict} {label}" + (f" ({detail})" if detail and not ok else ""))


def repo():
    d = tempfile.mkdtemp(prefix="gir-filemode-")
    git(d, "init", "-q", "-b", "main")
    files = {".githooks/pre-commit": HOOK, ".githooks/commit-msg": b"#!/bin/sh\n", "tools/run.sh": SCRIPT, "README.md": b"readme\n"}
    for name, content in files.items():
        p = Path(d, name)
        p.parent.mkdir(parents=True, exist_ok=True)
        p.write_bytes(content)
        if UNIX:
            p.chmod(0o644)
    git(d, "add", "-A")
    git(d, "commit", "-q", "-m", "chore: files")
    return d


def mode(d, path):
    return git(d, "ls-files", "-s", "--", path).split(" ")[0]


def disk_mode(d, path):
    return stat.S_IMODE(os.lstat(Path(d, path)).st_mode)


def executable(d, path):
    return bool(disk_mode(d, path) & stat.S_IXUSR)


def status(d, path):
    return git(d, "status", "--porcelain", "--", path).rstrip("\n")


def run_fix(d, title):
    print(f"\n## {title}")
    r = sh(d, GIR, "doctor", "--fix")
    out = (r.stdout + r.stderr).decode(errors="replace")
    print("  $ gir doctor --fix -> exit", r.returncode)
    print("    " + "\n    ".join(l for l in out.splitlines() if "exec-bit" in l or "fatal" in l or "error" in l))
    check("no fatal error", "fatal" not in out and "error:" not in out)
    check("finished with the summary line", out.rstrip().splitlines()[-1].startswith("gir doctor:"))
    return out


print(f"git: {git(tempfile.gettempdir(), 'version').strip()}; platform: {sys.platform}; gir: {GIR}")

print("\n## premise p1: git and a hook that is not executable on disk (no gir involved)")
if UNIX:
    d = repo()
    git(d, "config", "core.hooksPath", ".githooks")
    r = sh(d, "git", "commit", "-q", "--allow-empty", "-m", "chore: probe")
    err = r.stderr.decode(errors="replace")
    print("  $ git commit with .githooks/pre-commit at mode", oct(disk_mode(d, ".githooks/pre-commit")), "-> exit", r.returncode)
    print("    " + "\n    ".join(err.splitlines()))
    check("p1: the commit succeeds", r.returncode == 0)
    check("p1: the hook did not run", not Path(d, ".git/pre-commit-ran").exists())
    check("p1: git says it ignored the hook", "hook was ignored because it's not set as executable" in err, err)
    d = repo()
    git(d, "config", "core.hooksPath", ".githooks")
    os.chmod(Path(d, ".githooks/pre-commit"), 0o755)
    sh(d, "git", "commit", "-q", "--allow-empty", "-m", "chore: probe")
    check("p1 control: the same hook made executable does run", Path(d, ".git/pre-commit-ran").exists())
else:
    print("  SKIP p1 is about Unix file modes")

d = repo()
filemode = git(d, "config", "core.filemode").strip()
run_fix(d, f"plain repository, core.filemode={filemode}")
for path in (".githooks/pre-commit", ".githooks/commit-msg", "tools/run.sh"):
    check(f"{path} is 100755 in the index", mode(d, path) == "100755", mode(d, path))
    if UNIX:
        check(f"{path} is executable on disk", executable(d, path), oct(disk_mode(d, path)))
    else:
        check(f"{path} still exists on disk (Windows: nothing to change)", Path(d, path).is_file())
    s = status(d, path)
    check(f"git status shows the mode change staged and nothing unstaged for {path}", s == f"M  {path}", s)
check("tools/run.sh content is unchanged", Path(d, "tools/run.sh").read_bytes() == SCRIPT)
check("the pre-commit hook content is unchanged", Path(d, ".githooks/pre-commit").read_bytes() == HOOK)
check("README.md (not a script) is untouched in the index", mode(d, "README.md") == "100644")
if UNIX:
    check("README.md (not a script) is not executable on disk", not executable(d, "README.md"), oct(disk_mode(d, "README.md")))
git(d, "add", "--", "tools/run.sh", ".githooks/pre-commit")
check("git add keeps tools/run.sh 100755", mode(d, "tools/run.sh") == "100755", mode(d, "tools/run.sh"))
check("git add keeps .githooks/pre-commit 100755", mode(d, ".githooks/pre-commit") == "100755", mode(d, ".githooks/pre-commit"))
if UNIX:
    r = sh(d, "git", "commit", "-q", "-m", "chore: fixed modes")
    err = r.stderr.decode(errors="replace")
    print("  $ git commit -> exit", r.returncode, ("; stderr: " + " | ".join(err.splitlines())) if err else "")
    check("git runs the fixed pre-commit hook", Path(d, ".git/pre-commit-ran").exists(), err.strip())

d = repo()
os.remove(Path(d, ".githooks/pre-commit"))
run_fix(d, "a hook deleted from the working tree")
check(".githooks/pre-commit is 100755 in the index", mode(d, ".githooks/pre-commit") == "100755", mode(d, ".githooks/pre-commit"))
check("the deleted file was not restored", not Path(d, ".githooks/pre-commit").exists())
if UNIX:
    check("tools/run.sh (present) is executable on disk", executable(d, "tools/run.sh"), oct(disk_mode(d, "tools/run.sh")))

d = repo()
git(d, "sparse-checkout", "set", "--no-cone", "/*", "!/.githooks/")
run_fix(d, "hooks outside a sparse checkout's cone")
check(".githooks/pre-commit is 100755 in the index", mode(d, ".githooks/pre-commit") == "100755", mode(d, ".githooks/pre-commit"))
check(".githooks/ is still not in the working tree", not Path(d, ".githooks").exists())
check("hooks are still skip-worktree", git(d, "ls-files", "-v", ".githooks/pre-commit").startswith("S "), git(d, "ls-files", "-v", ".githooks/pre-commit").strip())
if UNIX:
    check("tools/run.sh (present) is executable on disk", executable(d, "tools/run.sh"), oct(disk_mode(d, "tools/run.sh")))

d = repo()
edited = b"#!/bin/sh\necho run\n# local edit, not staged\n"
Path(d, "tools/run.sh").write_bytes(edited)
run_fix(d, "a script with an unstaged local edit")
check("tools/run.sh is 100755 in the index", mode(d, "tools/run.sh") == "100755")
check("the file on disk still holds the edit, byte for byte", Path(d, "tools/run.sh").read_bytes() == edited)
diff = git(d, "diff", "--", "tools/run.sh")
check("the local edit stays unstaged", "+# local edit, not staged" in diff, diff)
check("git diff shows no mode change", "old mode" not in diff, diff)
staged = sh(d, "git", "show", ":tools/run.sh").stdout
check("the staged content is the committed content", staged == SCRIPT, repr(staged))
if UNIX:
    check("tools/run.sh is executable on disk", executable(d, "tools/run.sh"), oct(disk_mode(d, "tools/run.sh")))

if UNIX:
    d = repo()
    outside = Path(tempfile.mkdtemp(prefix="gir-filemode-outside-"), "target.txt")
    outside.write_bytes(b"not a script\n")
    outside.chmod(0o644)
    os.remove(Path(d, "tools/run.sh"))
    os.symlink(outside, Path(d, "tools/run.sh"))
    run_fix(d, "a tracked script replaced by a symlink in the working tree")
    check("tools/run.sh is 100755 in the index", mode(d, "tools/run.sh") == "100755")
    check("the symlink's target was not made executable", stat.S_IMODE(outside.stat().st_mode) == 0o644, oct(stat.S_IMODE(outside.stat().st_mode)))
    check("tools/run.sh is still a symlink", Path(d, "tools/run.sh").is_symlink())

print(f"\n{RESULTS['PASS']} PASS, {RESULTS['FAIL']} FAIL")
