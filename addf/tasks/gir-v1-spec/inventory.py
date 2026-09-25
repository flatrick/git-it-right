"""List gir's user-visible identifiers that delta/spec/ never mentions.

usage: python inventory.py

Identifiers come from the source and the built binary: rule ids, config
keys, subcommand flags, doctor check ids and explain topics. Exits 1 when
any is missing from the spec text.
"""

import re
import subprocess
import sys
from pathlib import Path

BUNDLE = Path(__file__).resolve().parent
REPO = BUNDLE.parents[2]


def source(path):
    return (REPO / path).read_text(encoding="utf-8")


def identifiers():
    rules = re.findall(r'"([a-z-]+)"', source("src/cc/mod.rs").split("pub const RULES")[1].split("];")[0])
    rules += re.findall(r'rule: "([a-z-]+)"', source("src/cmd/lint.rs"))
    keys = re.findall(r'"gir\.([a-z]+)"', source("src/config.rs"))
    allowed = source("src/main.rs").split("let allowed")[1].split("};")[0]
    flags = [f for group in re.findall(r"&\[([^\]]*)\]", allowed) for f in re.findall(r'"([a-z-]+)"', group)] + ["range"]
    doctor = source("src/cmd/doctor.rs")
    checks = re.findall(r'\("([a-zA-Z]+\.[a-zA-Z]+)", "', doctor)
    checks += re.findall(r'check\(Level::\w+, "([^"]+)"', doctor)
    topics = subprocess.run(
        ["cargo", "run", "-q", "--", "explain"], cwd=REPO, capture_output=True, text=True, encoding="utf-8", check=True
    ).stdout.removeprefix("topics: ").split()
    groups = {
        "rule": rules,
        "config key": keys,
        "flag": [f"--{f}" for f in flags],
        "doctor check": checks,
        "explain topic": topics,
    }
    return {kind: sorted(set(values)) for kind, values in groups.items()}


def main():
    spec = "\n".join(p.read_text(encoding="utf-8") for p in sorted((BUNDLE / "delta" / "spec").glob("*.md")))
    lowered = spec.lower()
    missing = []
    total = 0
    for kind, values in identifiers().items():
        total += len(values)
        for value in values:
            if value.lower() not in lowered:
                missing.append(f"{kind} not in spec: {value}")
    for line in missing:
        print(line)
    print(f"{total} identifiers, {len(missing)} missing")
    return 1 if missing else 0


if __name__ == "__main__":
    sys.exit(main())
