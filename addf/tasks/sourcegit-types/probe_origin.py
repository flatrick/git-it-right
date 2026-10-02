"""Probe: how `git config --show-origin` reports where `gir.typesFile` was set.

Runs in a throwaway home and repository; never reads the caller's git config.
"""
import os
import subprocess
import sys
import tempfile
from pathlib import Path


def git(*args, cwd, env):
    r = subprocess.run(["git", *args], cwd=cwd, env=env, capture_output=True)
    print(f"$ git {' '.join(args)}")
    print(f"  exit={r.returncode} stdout={r.stdout!r} stderr={r.stderr!r}")
    return r


def main():
    with tempfile.TemporaryDirectory() as tmp:
        tmp = Path(tmp)
        home = tmp / "home"
        repo = tmp / "repo"
        sub = repo / "sub"
        home.mkdir()
        env = dict(os.environ)
        env["HOME"] = str(home)
        env["GIT_CONFIG_GLOBAL"] = str(home / ".gitconfig")
        env["GIT_CONFIG_NOSYSTEM"] = "1"
        for k in ("GIT_DIR", "GIT_WORK_TREE", "GIT_CONFIG_PARAMETERS", "GIT_CONFIG_COUNT"):
            env.pop(k, None)
        git("--version", cwd=tmp, env=env)
        git("init", "-q", str(repo), cwd=tmp, env=env)
        sub.mkdir()
        get = ("config", "--show-origin", "--show-scope", "-z", "--get", "gir.typesFile")

        print("## unset")
        git(*get, cwd=repo, env=env)
        print("## global relative")
        git("config", "--global", "gir.typesFile", "types.json", cwd=repo, env=env)
        git(*get, cwd=sub, env=env)
        print("## global ~/ with --type=path")
        git("config", "--global", "gir.typesFile", "~/x.json", cwd=repo, env=env)
        git(*get, cwd=repo, env=env)
        git("config", "--show-origin", "--show-scope", "-z", "--type=path", "--get", "gir.typesFile", cwd=repo, env=env)
        print("## local overrides global")
        git("config", "--local", "gir.typesFile", "local.json", cwd=repo, env=env)
        git(*get, cwd=repo, env=env)
        print("## -c on the command line")
        r = subprocess.run(["git", "-c", "gir.typesFile=cli.json", *get], cwd=repo, env=env, capture_output=True)
        print(f"  exit={r.returncode} stdout={r.stdout!r}")
        print("## empty local value")
        git("config", "--local", "gir.typesFile", "", cwd=repo, env=env)
        git(*get, cwd=repo, env=env)
        print("## included file")
        (home / "inc.gitconfig").write_text("[gir]\n\ttypesFile = inc.json\n")
        git("config", "--local", "--unset", "gir.typesFile", cwd=repo, env=env)
        git("config", "--global", "include.path", "inc.gitconfig", cwd=repo, env=env)
        git(*get, cwd=repo, env=env)
        print("## path with spaces")
        git("config", "--global", "--unset", "include.path", cwd=repo, env=env)
        git("config", "--global", "gir.typesFile", "my dir/t.json", cwd=repo, env=env)
        git(*get, cwd=repo, env=env)
        print("## .girconfig via --file")
        (repo / ".girconfig").write_text("[gir]\n\ttypesFile = rel/t.json\n")
        git("config", "--file", ".girconfig", "--show-origin", "-z", "--get", "gir.typesFile", cwd=sub, env=env)
        git("config", "--file", str(repo / ".girconfig"), "--show-origin", "-z", "--get", "gir.typesFile", cwd=sub, env=env)
        print("## local origin seen from a subdirectory and a linked worktree")
        git("config", "--global", "--unset", "gir.typesFile", cwd=repo, env=env)
        git("config", "--local", "gir.typesFile", "local.json", cwd=repo, env=env)
        git(*get, cwd=sub, env=env)
        git("-c", "user.name=p", "-c", "user.email=p@p", "commit", "-q", "--allow-empty", "-m", "init", cwd=repo, env=env)
        wt = tmp / "wt"
        git("worktree", "add", "-q", str(wt), cwd=repo, env=env)
        git(*get, cwd=wt, env=env)
        git("rev-parse", "--git-common-dir", cwd=wt, env=env)
    return 0


if __name__ == "__main__":
    sys.exit(main())
