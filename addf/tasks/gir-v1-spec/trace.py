"""Check that every requirement in delta/spec/ is traced to a test the suite contains.

usage: python trace.py [--tests <file>]

Without --tests it runs `cargo test -- --list` from the repository root.
Exits 1 when a requirement is untraced, a trace row names an unknown
requirement, or a trace row names a test the suite does not list.
"""

import argparse
import re
import subprocess
import sys
from pathlib import Path

BUNDLE = Path(__file__).resolve().parent
REPO = BUNDLE.parents[2]
ANCHOR = re.compile(r'<a id="(req-[a-z0-9-]+)"></a>')
ROW = re.compile(r"^\|\s*`(req-[a-z0-9-]+)`\s*\|\s*`([^`]+)`\s*\|")


def listed_tests(path):
    if path:
        text = Path(path).read_text(encoding="utf-8")
    else:
        text = subprocess.run(
            ["cargo", "test", "-q", "--", "--list"],
            cwd=REPO, capture_output=True, text=True, encoding="utf-8", check=True,
        ).stdout
    return {line[: -len(": test")] for line in text.splitlines() if line.endswith(": test")}


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--tests")
    args = parser.parse_args()

    requirements = {}
    for spec in sorted((BUNDLE / "delta" / "spec").glob("*.md")):
        for n, line in enumerate(spec.read_text(encoding="utf-8").splitlines(), 1):
            for anchor in ANCHOR.findall(line):
                if anchor in requirements:
                    print(f"{spec.name}:{n}: duplicate requirement {anchor}")
                    return 1
                requirements[anchor] = f"{spec.name}:{n}"

    traced = {}
    problems = []
    tests = listed_tests(args.tests)
    for table in sorted((BUNDLE / "trace").glob("*.md")):
        for n, line in enumerate(table.read_text(encoding="utf-8").splitlines(), 1):
            row = ROW.match(line)
            if not row:
                continue
            req, test = row.groups()
            where = f"trace/{table.name}:{n}"
            if req not in requirements:
                problems.append(f"{where}: unknown requirement {req}")
            if test not in tests:
                problems.append(f"{where}: test not in suite: {test}")
            traced.setdefault(req, []).append(test)

    for req, where in requirements.items():
        if req not in traced:
            problems.append(f"{where}: untraced requirement {req}")

    for problem in problems:
        print(problem)
    print(f"{len(requirements)} requirements, {sum(map(len, traced.values()))} trace rows, {len(problems)} problems")
    return 1 if problems else 0


if __name__ == "__main__":
    sys.exit(main())
