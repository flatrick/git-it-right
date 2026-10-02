"""Acceptance: the type file from GitHub issue #4, used through a global `gir.typesFile`.

Usage: acceptance.py <path to gir binary>
Fetches the issue body with `gh`, extracts its JSON example unchanged, and drives gir
in a throwaway repository with its own global git config.
"""
import json
import os
import re
import subprocess
import sys
import tempfile
from pathlib import Path

results = []


def check(name, ok, detail=""):
    results.append(ok)
    print(f"{'PASS' if ok else 'FAIL'}  {name}")
    if detail:
        print("      " + detail.replace("\n", "\n      "))


def main():
    gir = str(Path(sys.argv[1]).resolve())
    body = json.loads(subprocess.run(["gh", "issue", "view", "4", "--json", "body"], capture_output=True, text=True, check=True).stdout)["body"]
    example = re.search(r"```json\n(.*?)```", body, re.S).group(1)
    print(f"issue example: {len(example)} chars, contains a tab: {chr(9) in example}")

    with tempfile.TemporaryDirectory() as tmp:
        tmp = Path(tmp)
        home, repo = tmp / "home", tmp / "repo"
        home.mkdir()
        (home / "sourcegit-types.json").write_text(example, encoding="utf-8")
        env = dict(os.environ, HOME=str(home), GIT_CONFIG_GLOBAL=str(home / ".gitconfig"), GIT_CONFIG_NOSYSTEM="1")
        for k in ("GIT_DIR", "GIT_WORK_TREE", "GIT_CONFIG_COUNT"):
            env.pop(k, None)

        def run(*args, cwd=repo):
            r = subprocess.run(list(args), cwd=cwd, env=env, capture_output=True, text=True)
            return r.returncode, r.stdout, r.stderr

        run("git", "init", "-q", str(repo), cwd=tmp)
        run("git", "config", "--global", "gir.typesFile", "~/sourcegit-types.json")

        def lint(msg):
            (repo / "MSG").write_text(msg)
            return run(gir, "lint", "MSG")

        for ty in ["build", "chore", "ci", "docs", "feat", "fix", "perf", "refactor", "revert", "style", "test"]:
            code, _, err = lint(f"{ty}: change x\n")
            check(f"lint accepts `{ty}` from the issue's file", code == 0, err.strip())
        code, _, err = lint("wip: change x\n")
        check("lint rejects `wip`, which the file does not define", code == 1 and "`wip` is not an allowed type" in err, err.strip())

        code, out, err = run(gir, "explain", "types")
        check("explain types names the file and shows its descriptions",
              code == 0 and "sourcegit-types.json" in out and "Changes that affect the build system or external dependencies" in out, out + err)
        print(out)

        (home / "sourcegit-types.json").write_text(example.replace('"Type": "fix",', '"Type": "fix"'), encoding="utf-8")
        code, _, err = lint("feat: change x\n")
        check("a syntax error refuses with path, origin and line", code == 2 and "sourcegit-types.json (gir.typesFile in " in err and "line " in err, err.strip())
        print(err)

    print(f"{results.count(True)} PASS, {results.count(False)} FAIL")
    return 0 if all(results) else 1


if __name__ == "__main__":
    sys.exit(main())
