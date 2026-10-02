"""Compare a TASK.md's DEFINE content between a commit and the working tree, ignoring State and Basis lines."""
import difflib
import re
import subprocess
import sys
from pathlib import Path

rev, path = sys.argv[1], sys.argv[2]
old = subprocess.run(["git", "show", f"{rev}:{path}"], capture_output=True, text=True, check=True).stdout
new = Path(path).read_text()


def define(text):
    spec = text[text.index("## Specification impact"):text.index("## Define")]
    crit = text[text.index("### Success criteria"):text.index("### Constraints")]
    prem = text[text.index("### Material empirical premises"):text.index("### DEFINE gate")]
    return [l for l in (spec + crit + prem).splitlines() if not re.match(r"-   (State|Basis):", l)]


a, b = define(old), define(new)
print(f"compared {rev}:{path} with the working tree; {len(a)} and {len(b)} lines after dropping State and Basis lines")
diff = list(difflib.unified_diff(a, b, rev, "working tree", lineterm=""))
print("\n".join(diff) if diff else "no difference")
