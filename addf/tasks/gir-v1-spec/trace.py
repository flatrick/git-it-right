"""Check that every requirement in delta/spec/ is traced to a test the suite contains.

usage: python trace.py [--tests <file>] [--follow <rev>]

Without --tests it runs `cargo test -- --list` from the repository root.
A Test cell of `windows: name` names a test compiled only on that OS; it is
checked there and listed, not failed, elsewhere.
--follow REV rewrites every cited line number in trace/ through the diff of
each cited file between REV and the working tree, then checks as usual. A
citation inside a changed hunk cannot be mapped and is reported instead.
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


def test_span(text, test):
    name = test.rpartition(": ")[2].rpartition("::")[2]
    start = next((n for n, l in enumerate(text) if re.match(rf"\s*fn {name}\(", l)), None)
    if start is None:
        return None
    indent = len(text[start]) - len(text[start].lstrip())
    end = next(n for n in range(start + 1, len(text)) if text[n] == " " * indent + "}")
    return start + 1, end + 1


def line_map(rev, path):
    diff = subprocess.run(
        ["git", "diff", "-U0", rev, "--", path], cwd=REPO, capture_output=True, text=True, encoding="utf-8", check=True
    ).stdout
    hunks = [tuple(int(x or 1) for x in m) for m in re.findall(r"^@@ -(\d+)(?:,(\d+))? \+(\d+)(?:,(\d+))? @@", diff, re.M)]

    def remap(n):
        shift = 0
        for old, old_len, new, new_len in hunks:
            if n < old or (old_len == 0 and n == old):
                break
            if n < old + old_len:
                return None
            shift = (new + new_len if new_len else new + 1) - (old + old_len if old_len else old + 1)
        return n + shift

    return remap


def follow(rev):
    cite_spec = re.compile(r"`((?:src|tests)/[\w/.-]+\.rs):([\d,-]+)`")
    maps = {}
    unmapped = []
    for table in sorted((BUNDLE / "trace").glob("*.md")):
        text = table.read_text(encoding="utf-8")

        def rewrite(m):
            path, spec = m.groups()
            remap = maps.setdefault(path, line_map(rev, path))
            parts = []
            for part in spec.split(","):
                ends = [remap(int(x)) for x in part.split("-")]
                if None in ends:
                    unmapped.append(f"trace/{table.name}: {path}:{part} is inside a changed hunk")
                    return m.group()
                parts.append("-".join(map(str, ends)))
            return f"`{path}:{','.join(parts)}`"

        table.write_text(cite_spec.sub(rewrite, text), encoding="utf-8")
    return unmapped


def check_cite(path, lines, test):
    source = REPO / path
    if not source.is_file():
        return [f"cited file does not exist: {path}"]
    text = source.read_text(encoding="utf-8").splitlines()
    span_of_test = test_span(text, test)
    problems = []
    for part in lines.split(","):
        first, _, last = part.partition("-")
        first, last = int(first), int(last or first)
        span = text[first - 1 : last]
        if not span or not any("assert" in l for l in span):
            problems.append(f"no assert at {path}:{part}")
        elif span_of_test and not span_of_test[0] <= first <= last <= span_of_test[1]:
            problems.append(f"{path}:{part} is outside {test}")
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
    parser.add_argument("--follow")
    args = parser.parse_args()
    unmapped = follow(args.follow) if args.follow else []

    requirements = {}
    for spec in sorted((BUNDLE / "delta" / "spec").glob("*.md")):
        for n, line in enumerate(spec.read_text(encoding="utf-8").splitlines(), 1):
            for anchor in ANCHOR.findall(line):
                if anchor in requirements:
                    print(f"{spec.name}:{n}: duplicate requirement {anchor}")
                    return 1
                requirements[anchor] = f"{spec.name}:{n}"

    traced = {}
    problems = list(unmapped)
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
                problems.extend(f"{where}: {p}" for p in check_cite(*cite.groups(), test))
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
