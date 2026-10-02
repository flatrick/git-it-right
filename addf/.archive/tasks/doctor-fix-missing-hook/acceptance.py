"""Acceptance for doctor-fix-missing-hook: --fix sets the exec bit without needing the hook file.

Usage: python3 acceptance.py GIR_BINARY
Prints PASS/FAIL per check and exits 0 either way.
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
}
RESULTS = {"PASS": 0, "FAIL": 0}


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
    d = tempfile.mkdtemp(prefix="gir-hookfix-")
    git(d, "init", "-q", "-b", "main")
    for name in (".githooks/pre-commit", ".githooks/commit-msg", "tools/run.sh", "README.md"):
        p = Path(d, name)
        p.parent.mkdir(parents=True, exist_ok=True)
        p.write_text("#!/bin/sh\n" if name != "README.md" else "readme\n")
    git(d, "add", "-A")
    git(d, "commit", "-q", "-m", "chore: files")
    return d


def mode(d, path):
    return git(d, "ls-files", "-s", "--", path).split(" ")[0]


def run_fix(d, title):
    print(f"\n## {title}")
    before_tree = git(d, "status", "--porcelain", "--ignored")
    r = sh(d, GIR, "doctor", "--fix")
    out = (r.stdout + r.stderr).decode(errors="replace")
    print("  $ gir doctor --fix -> exit", r.returncode)
    print("    " + "\n    ".join(l for l in out.splitlines() if "exec-bit" in l or "fatal" in l or "error" in l))
    check("no fatal error", "fatal" not in out and "error:" not in out)
    check("finished with the summary line", out.rstrip().splitlines()[-1].startswith("gir doctor:"))
    check(".gitattributes was written (a fix after the hooks would have been reached)", Path(d, ".gitattributes").exists())
    return out


d = repo()
os.remove(Path(d, ".githooks/pre-commit"))
out = run_fix(d, "a hook deleted from the working tree")
check(".githooks/pre-commit is 100755 in the index", mode(d, ".githooks/pre-commit") == "100755", mode(d, ".githooks/pre-commit"))
check(".githooks/commit-msg (present) is 100755", mode(d, ".githooks/commit-msg") == "100755")
check("tools/run.sh (present) is 100755", mode(d, "tools/run.sh") == "100755")
check("the deleted file was not restored", not Path(d, ".githooks/pre-commit").exists())

d = repo()
git(d, "sparse-checkout", "set", "--no-cone", "/*", "!/.githooks/")
out = run_fix(d, "hooks outside a sparse checkout's cone")
check(".githooks/pre-commit is 100755 in the index", mode(d, ".githooks/pre-commit") == "100755", mode(d, ".githooks/pre-commit"))
check(".githooks/ is still not in the working tree", not Path(d, ".githooks").exists())
check("hooks are still skip-worktree", git(d, "ls-files", "-v", ".githooks/pre-commit").startswith("S "), git(d, "ls-files", "-v", ".githooks/pre-commit").strip())
check("tools/run.sh (present) is 100755", mode(d, "tools/run.sh") == "100755")

d = repo()
Path(d, ".githooks/commit-msg").write_text("#!/bin/sh\n# local edit, not staged\n")
out = run_fix(d, "a hook with an unstaged local edit")
check(".githooks/commit-msg is 100755", mode(d, ".githooks/commit-msg") == "100755")
check("the local edit stays unstaged", "+# local edit, not staged" in git(d, "diff", "--", ".githooks/commit-msg"), git(d, "diff", "--", ".githooks/commit-msg"))
staged = sh(d, "git", "show", ":.githooks/commit-msg").stdout
check("the staged content is the committed content", staged == b"#!/bin/sh\n", repr(staged))

print(f"\n{RESULTS['PASS']} PASS, {RESULTS['FAIL']} FAIL")
