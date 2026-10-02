"""Run the review's scripts against the old and the new gir, normalise, and diff.

Usage, from the repository root: python3 addf/tasks/fixup-fixes-acceptance/run_review_scripts.py OLD_GIR NEW_GIR OUT_DIR
Writes OUT_DIR/<script>-old.log, -new.log (normalised) and review-scripts-diff.log.
"""
import difflib
import os
import re
import subprocess
import sys
import tempfile
from pathlib import Path

HERE = Path(__file__).parent / "review-scripts"
# split orders commits by ID; fixed dates make IDs, and so the order, the same on every run
FIXED_DATES = {"GIT_AUTHOR_DATE": "2026-01-01T00:00:00+0000", "GIT_COMMITTER_DATE": "2026-01-01T00:00:00+0000"}


def normalise(text: str) -> str:
    tmp = r"[\\/]".join(re.escape(part) for part in re.split(r"[\\/]", tempfile.gettempdir()))
    text = re.sub(tmp + r"[\\/][^\s'\":]+", "<tmp>", text)
    return re.sub(r"\b[0-9a-f]{7,40}\b", "<sha>", text)


def main(old: str, new: str, out: str) -> int:
    out_dir = Path(out)
    report = []
    for script in ("edge.py", "edge2.py", "edge3.py"):
        runs = {}
        for name, binary in (("old", old), ("new", new)):
            r = subprocess.run([sys.executable, script], cwd=HERE, capture_output=True, text=True,
                               env={**os.environ, "GIR_BIN": binary, **FIXED_DATES})
            text = normalise(r.stdout + r.stderr + f"\n[script exit {r.returncode}]\n")
            (out_dir / f"{script[:-3]}-{name}.log").write_text(text)
            runs[name] = text.splitlines(keepends=True)
        diff = list(difflib.unified_diff(runs["old"], runs["new"], f"{script} old", f"{script} new", n=4))
        report.append(f"##### {script}: {'identical' if not diff else 'differs'}\n")
        report.extend(diff)
    (out_dir / "review-scripts-diff.log").write_text("".join(report))
    print("".join(report))
    return 0


if __name__ == "__main__":
    sys.exit(main(*sys.argv[1:4]))
