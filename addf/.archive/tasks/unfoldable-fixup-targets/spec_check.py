"""Check for c6-spec: the published spec/fixup.md text against this Task's Implement wording, the code and the tests.

Usage: python3 spec_check.py  (from the repository root)
Prints PASS/FAIL per check and exits 1 if any fails.
"""
import re
import sys
from pathlib import Path

spec = Path("addf/spec/fixup.md").read_text(encoding="utf-8")
code = Path("src/cmd/fixup.rs").read_text(encoding="utf-8")
tests = Path("tests/fixup.rs").read_text(encoding="utf-8")
task_dir = Path(__file__).resolve().parent
task = (task_dir / "TASK.md").read_text(encoding="utf-8")


def requirement(anchor):
    m = re.search(r'<a id="' + anchor + r'"></a>\n(.*?)\n', spec)
    return m.group(1) if m else ""


checks = []
ask = requirement("req-fixup-ask-branch-commit")
checks.append(("ask-branch-commit says non-merge", "up to 20 of the newest non-merge commits after the base commit" in ask))
checks.append(("code lists non-merge commits", '"rev-list", "--no-merges", "--max-count=20"' in code))

explicit = requirement("req-fixup-explicit-target-not-merge")
explicit_msg = "is a merge commit, which a rebase drops; pass the commit the change belongs to"
checks.append(("explicit-target-not-merge published", explicit_msg in explicit and "exit `2`" in explicit))
checks.append(("explicit-target-not-merge matches Implement", explicit in task.replace("New `explicit-target-not-merge`, after `explicit-target-after-base`: ", "**explicit-target-not-merge.** ")))
checks.append(("explicit message in code", explicit_msg in code))
checks.append(("explicit message asserted by a test", explicit_msg in tests))

merge = requirement("req-fixup-merge-limit")
merge_msg = "was last changed by the merge {}, which a rebase drops; commit it normally, or pass one: gir {} <commit>"
checks.append(("merge-limit published", "was last changed by the merge <sha>, which a rebase drops; commit it normally, or pass one: gir <subcommand> <commit>" in merge))
checks.append(("merge-limit insertion rule published", "none of its neighbouring lines was last changed by an eligible non-merge commit" in merge))
checks.append(("merge-limit matches Implement", merge.replace("**merge-limit.** Automatic", "automatic") in task))
checks.append(("merge message in code", merge_msg in code))
checks.append(("merge message asserted by a test", "was last changed by the merge {}, which a rebase drops; commit it normally, or pass one: gir {sub} <commit>" in tests))

failed = 0
for label, ok in checks:
    print(f"{'PASS' if ok else 'FAIL'} {label}")
    failed += not ok
print(f"{len(checks) - failed}/{len(checks)} passed")
sys.exit(1 if failed else 0)
