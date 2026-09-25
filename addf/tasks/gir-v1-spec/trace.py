"""Check that every requirement in delta/spec/ is traced to a test the suite contains.

usage: python trace.py [--tests <file>]

Without --tests it runs `cargo test -- --list` from the repository root.
A Test cell of `windows: name` names a test compiled only on that OS; it is
checked there and listed, not failed, elsewhere.
Exits 1 when a requirement is untraced, a trace row names an unknown
requirement, or a trace row names a test the suite does not list.
"""

import argparse
import re
import platform
import subprocess
import sys
from pathlib import Path

BUNDLE = Path(__file__).resolve().parent
REPO = BUNDLE.parents[2]
ANCHOR = re.compile(r'<a id="(req-[a-z0-9-]+)"></a>')
ROW = re.compile(r"^\|\s*`(req-[a-z0-9-]+)`\s*\|\s*`([^`]+)`\s*\|")
CITE = re.compile(r"`((?:src|tests)/[\w/.-]+\.rs):([\d,-]+)`")


def check_cite(path, lines):
    source = REPO / path
    if not source.is_file():
        return [f"cited file does not exist: {path}"]
    text = source.read_text(encoding="utf-8").splitlines()
    problems = []
    for part in lines.split(","):
        first, _, last = part.partition("-")
        span = text[int(first) - 1 : int(last or first)]
        if not span or not any("assert" in l for l in span):
            problems.append(f"no assert at {path}:{part}")
    return problems


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
    elsewhere = []
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
            os_name, _, gated = test.rpartition(": ")
            if os_name and os_name != platform.system().lower():
                elsewhere.append(f"{where}: {os_name}-only test not checked here: {gated}")
            elif (gated if os_name else test) not in tests:
                problems.append(f"{where}: test not in suite: {test}")
            for cite in CITE.finditer(line):
                problems.extend(f"{where}: {p}" for p in check_cite(*cite.groups()))
            traced.setdefault(req, []).append(test)

    for req, where in requirements.items():
        if req not in traced:
            problems.append(f"{where}: untraced requirement {req}")

    for line in problems + elsewhere:
        print(line)
    print(f"{len(requirements)} requirements, {sum(map(len, traced.values()))} trace rows, {len(problems)} problems, {len(elsewhere)} checked only on another OS")
    return 1 if problems else 0


if __name__ == "__main__":
    sys.exit(main())
