import os, subprocess, sys, tempfile, shutil

GIR = os.environ.get("GIR_BIN", "/home/flatrick/src/github/flatrick/git-it-right/.worktrees/fixup-modes/target/debug/gir")
ENV = dict(os.environ, GIT_AUTHOR_NAME="t", GIT_AUTHOR_EMAIL="t@t", GIT_COMMITTER_NAME="t",
           GIT_COMMITTER_EMAIL="t@t", GIR_INTERACTIVE="0", GIT_EDITOR="true", GIT_CONFIG_GLOBAL="/dev/null")


def sh(cwd, *args, check=True, env=None):
    r = subprocess.run(args, cwd=cwd, capture_output=True, text=True, env=env or ENV)
    if check and r.returncode:
        raise SystemExit(f"FAILED {args}: {r.stdout}{r.stderr}")
    return r


def repo():
    d = tempfile.mkdtemp()
    sh(d, "git", "init", "-q", "-b", "main")
    open(f"{d}/base", "w").write("base\n")
    sh(d, "git", "add", "-A"); sh(d, "git", "commit", "-qm", "chore: base")
    sh(d, "git", "switch", "-qc", "topic")
    return d


def commit(d, path, content, msg):
    os.makedirs(os.path.dirname(f"{d}/{path}") or d, exist_ok=True)
    open(f"{d}/{path}", "w").write(content)
    sh(d, "git", "add", "--", path); sh(d, "git", "commit", "-qm", msg)


def gir(d, *args, env=None):
    r = sh(d, GIR, *args, check=False, env=env)
    return r.returncode, (r.stdout + r.stderr).strip()


def log(d, n=4):
    return sh(d, "git", "log", f"-{n}", "--format=%s").stdout.strip().replace("\n", " | ")


def case(name):
    print(f"\n=== {name}")


if __name__ == "__main__":
    case("1 deleting a '-- comment' line then another hunk")
    d = repo()
    commit(d, "x.lua", "-- header\nlocal a = 1\nlocal b = 2\nlocal c = 3\nlocal d = 4\n", "feat: lua")
    open(f"{d}/x.lua", "w").write("local a = 1\nlocal b = 2\nlocal c = 3\nlocal d = 5\n")
    sh(d, "git", "add", "x.lua")
    print(gir(d, "fixup", "--dry-run"))
    print(gir(d, "fixup", "--split", "--dry-run"))

    case("1b split with '-- comment' deletion across two targets")
    d = repo()
    commit(d, "x.lua", "-- header\nlocal a = 1\n", "feat: lua one")
    commit(d, "x.lua", "-- header\nlocal a = 1\n-- second\nlocal b = 2\n", "feat: lua two")
    open(f"{d}/x.lua", "w").write("local a = 1\nlocal b = 3\n")
    sh(d, "git", "add", "x.lua")
    print(gir(d, "fixup", "--split"))
    print(log(d))

    case("2 file name with a space")
    d = repo()
    commit(d, "my file.txt", "one\ntwo\n", "feat: spaced")
    open(f"{d}/my file.txt", "w").write("one\nTWO\n")
    sh(d, "git", "add", "-A")
    print(gir(d, "fixup", "--dry-run"))

    case("3 apply.whitespace=error and trailing whitespace in split")
    d = repo()
    commit(d, "a", "a1\na2\n", "feat: a")
    commit(d, "b", "b1\nb2\n", "feat: b")
    open(f"{d}/a", "w").write("a1 \na2\n"); open(f"{d}/b", "w").write("b1\nb2 \n")
    sh(d, "git", "add", "-A"); sh(d, "git", "config", "apply.whitespace", "error")
    print(gir(d, "fixup", "--split"))
    print(log(d))

    case("3b apply.whitespace=fix")
    sh(d, "git", "config", "apply.whitespace", "fix")
    print(gir(d, "fixup", "--split"))
    print(log(d))

    case("4 insertion between two commits' lines with --split")
    d = repo()
    commit(d, "f", "one\n", "feat: one")
    commit(d, "f", "one\ntwo\n", "feat: two")
    commit(d, "g", "g\n", "feat: g")
    open(f"{d}/f", "w").write("one\nmid\ntwo\n"); open(f"{d}/g", "w").write("G\n")
    sh(d, "git", "add", "-A")
    print(gir(d, "fixup", "--split"))

    case("5 new file + --split, non-interactive")
    d = repo()
    commit(d, "f", "one\n", "feat: one")
    open(f"{d}/new", "w").write("n\n"); open(f"{d}/f", "w").write("ONE\n")
    sh(d, "git", "add", "-A")
    print(gir(d, "fixup", "--split"))

    case("6 intent-to-add entry survives split?")
    d = repo()
    commit(d, "a", "a\n", "feat: a")
    commit(d, "b", "b\n", "feat: b")
    open(f"{d}/a", "w").write("A\n"); open(f"{d}/b", "w").write("B\n")
    sh(d, "git", "add", "-A")
    open(f"{d}/ita", "w").write("i\n"); sh(d, "git", "add", "-N", "ita")
    print(sh(d, "git", "status", "--short").stdout)
    print(gir(d, "fixup", "--split"))
    print(log(d))
    print("status after:", repr(sh(d, "git", "status", "--short").stdout))

    case("7 reword with staged changes")
    d = repo()
    commit(d, "a", "a\n", "feat: a")
    open(f"{d}/a", "w").write("A\n"); sh(d, "git", "add", "-A")
    print(gir(d, "reword", "HEAD"))
    print(log(d), sh(d, "git", "show", "--stat", "--format=%s", "HEAD").stdout)
    print("status after:", repr(sh(d, "git", "status", "--short").stdout))

    case("8 deleted line content '-- /dev/null'-like and '--- a/'")
    d = repo()
    commit(d, "s.sql", "-- /dev/null\nselect 1;\nselect 2;\n", "feat: sql")
    open(f"{d}/s.sql", "w").write("select 1;\nselect 3;\n")
    sh(d, "git", "add", "-A")
    print(gir(d, "fixup", "--dry-run"))

    case("9 split when a hunk deletes the whole file of one target")
    d = repo()
    commit(d, "a", "a\n", "feat: a")
    commit(d, "b", "b\n", "feat: b")
    os.remove(f"{d}/a"); open(f"{d}/b", "w").write("B\n")
    sh(d, "git", "add", "-A")
    print(gir(d, "fixup", "--split"))
    print(log(d))

    case("10 split, file mode + content change")
    d = repo()
    commit(d, "a", "a\n", "feat: a")
    commit(d, "b", "b\n", "feat: b")
    open(f"{d}/a", "w").write("A\n"); os.chmod(f"{d}/a", 0o755); open(f"{d}/b", "w").write("B\n")
    sh(d, "git", "add", "-A")
    print(gir(d, "fixup", "--split"))
    print(log(d))

    case("11 split with pre-commit hook that fails on 2nd commit -> state")
    d = repo()
    commit(d, "a", "a\n", "feat: a")
    commit(d, "b", "b\n", "feat: b")
    open(f"{d}/a", "w").write("A\n"); open(f"{d}/b", "w").write("B\n")
    sh(d, "git", "add", "-A")
    os.makedirs(f"{d}/.git/hooks", exist_ok=True)
    open(f"{d}/.git/hooks/pre-commit", "w").write("#!/bin/sh\nc=$(cat .git/n 2>/dev/null || echo 0); echo $((c+1)) > .git/n; [ $c -lt 1 ]\n")
    os.chmod(f"{d}/.git/hooks/pre-commit", 0o755)
    print(gir(d, "fixup", "--split"))
    print(log(d)); print("status after:", repr(sh(d, "git", "status", "--short").stdout))

    case("12 picker offers merge commit / main commits when no base (on main, no upstream)")
    d = tempfile.mkdtemp()
    sh(d, "git", "init", "-q", "-b", "main")
    commit(d, "a", "a\n", "feat: a")
    open(f"{d}/new", "w").write("n\n"); sh(d, "git", "add", "-A")
    r = subprocess.run([GIR, "fixup"], cwd=d, input="1\n", capture_output=True, text=True, env=dict(ENV, GIR_INTERACTIVE="1"))
    print(r.returncode, r.stdout, r.stderr)

    case("13 squash --split opens editor per target; message check")
    d = repo()
    commit(d, "a", "a\n", "feat: a")
    commit(d, "b", "b\n", "feat: b")
    open(f"{d}/a", "w").write("A\n"); open(f"{d}/b", "w").write("B\n")
    sh(d, "git", "add", "-A")
    print(gir(d, "squash", "--split"))
    print(log(d))
