import os
import shutil
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path


CHECKER = Path(__file__).resolve().parents[1] / "check-capsule"
CLEAN_OUTPUT = "capsule is consistent\n"
TASK = """# Task
## Resume
**Contract version:** `2`
**State:** `IMPLEMENT`
**State path:** `DEFINE -> IMPLEMENT`
**Resume at:** Run the checker.
**Open obligations:** `NONE`
## Specification impact
- Current contract: `framework:SPEC.md`
- Proposed delta: `NONE`
- Terminal publication: `PENDING`
## Define
### Success criteria
<a id="works"></a>
#### works
- Claim: The checker works.
- State: `UNVERIFIED`
- Scope: This repository.
- Consequence if false: Broken checks.
- Basis: Pending execution.
### Material empirical premises
`NONE`.
## Verify
"""
VERIFICATION = """<a id="verification-works"></a>
### Verification: works
- Claim: [works](#works).
- Method: Run the checker.
- Evidence considered: The observed output.
- Conclusion: `VERIFIED`. Output matches.
- Limitations: This fixture only.
"""
STANDALONE = """# Verification: works
## Claim
[works](../TASK.md#works).
## Method
Run the checker.
## Evidence considered
The observed output.
## Conclusion
<a id="conclusion"></a>
**Result:** `VERIFIED`
Output matches.
## Remaining uncertainty
This fixture only.
"""


class CheckCapsuleTest(unittest.TestCase):
    def setUp(self):
        temporary = tempfile.TemporaryDirectory()
        self.addCleanup(temporary.cleanup)
        self.repo = Path(temporary.name)
        self.root = self.repo / "addf"
        self.checker = self.root / "scripts/check-capsule"
        self.checker.parent.mkdir(parents=True)
        shutil.copy2(CHECKER, self.checker)
        self.write("SPEC.md", "# Specification\n## Specification map\n`NONE`.\n")
        self.write("INDEX.md", "# Index\n## Active Tasks\n`NONE`.\n## Knowledge\n")

    def write(self, relative, text):
        path = self.root / relative
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(text, encoding="utf-8")
        return path

    def run_checker(self, *args, cwd=None):
        result = subprocess.run(
            [sys.executable, str(self.checker), *map(str, args)],
            cwd=cwd or self.repo, capture_output=True, text=True,
            encoding="utf-8", timeout=15,
        )
        self.assertNotIn("Traceback", result.stdout + result.stderr)
        return result

    def clean(self, *args, cwd=None):
        result = self.run_checker(*args, cwd=cwd)
        self.assertEqual((result.returncode, result.stdout, result.stderr), (0, CLEAN_OUTPUT, ""))

    def finding(self, expected, *args):
        result = self.run_checker(*args)
        self.assertEqual(result.returncode, 1, result.stdout + result.stderr)
        self.assertEqual(result.stderr, "")
        self.assertIn(expected + "\n", result.stdout)
        for line in result.stdout.splitlines():
            self.assertRegex(line, r"^.+:[1-9][0-9]*: [A-Z][A-Z0-9_]*: .+$")
        return result

    def task(self, text=TASK, archived=False, name="example"):
        path = (".archive/" if archived else "") + "tasks/" + name + "/TASK.md"
        self.write(path, text)
        if not archived and "**State:** `IMPLEMENT`" in text:
            self.write("INDEX.md", "# Index\n## Active Tasks\n- [Example](tasks/" + name + "/TASK.md), State: `IMPLEMENT`.\n## Knowledge\n")
        return path

    def terminal(self):
        return TASK.replace("**State:** `IMPLEMENT`", "**State:** `COMPLETED`").replace(
            "DEFINE -> IMPLEMENT", "DEFINE -> IMPLEMENT -> COMPLETED"
        ).replace("Run the checker.", "`NONE`", 1).replace("`PENDING`", "`NONE`").replace(
            "- State: `UNVERIFIED`", "- State: `VERIFIED`"
        ).replace("Pending execution.", "[Conclusion](#verification-works).") + VERIFICATION

    def test_default_root_and_non_root_cwd(self):
        self.clean(cwd=self.checker.parent)

    def test_explicit_root_absolute_relative_and_spaces(self):
        original = self.root
        self.root = self.repo / "project framework"
        shutil.copytree(original, self.root)
        self.clean("--root", self.root)
        self.clean("--root", "../project framework", cwd=original)

    def test_default_space_root(self):
        destination = self.repo / "project framework"
        shutil.copytree(self.root, destination)
        self.checker = destination / "scripts/check-capsule"
        self.clean(cwd=self.repo)

    def test_help_unknown_arguments_and_bad_roots(self):
        result = self.run_checker("--help")
        self.assertEqual(result.returncode, 0)
        self.assertIn("--root", result.stdout)
        self.assertIn("--include-archive", result.stdout)
        for args in (("--bogus",), ("--root",), ("--root", "absent"), ("--root", self.root / "SPEC.md")):
            with self.subTest(args=args):
                result = self.run_checker(*args)
                self.assertEqual(result.returncode, 2)
                self.assertIn("error:", result.stderr)

    def test_durable_anchor_is_not_an_unfilled_placeholder(self):
        for relative in ("knowledge/claim.md", "LEDGER.md"):
            with self.subTest(path=relative):
                artifact = self.write(relative, '<a id="claim-name"></a>\n')
                self.clean()
                artifact.unlink()

    def test_placeholder_after_anchor_keeps_its_line_number(self):
        self.write("knowledge/claim.md", '<a id="claim-name"></a>\n<claim-name>\n')
        result = self.finding("addf/knowledge/claim.md:2: PLACEHOLDER: unfilled placeholder `<claim-name>`")
        self.assertEqual(result.stdout, "addf/knowledge/claim.md:2: PLACEHOLDER: unfilled placeholder `<claim-name>`\n")

    def test_malformed_anchors_remain_errors(self):
        for anchor in ('<a id="Claim-name"></a>', '<a id="cläim-name"></a>', '<a id="claim_name"></a>', '<a id="claim--name"></a>', '<a id="claim-name-"></a>', '<a id="claim-name">', '<a id="claim-name">text</a>'):
            with self.subTest(anchor=anchor):
                self.write("knowledge/claim.md", anchor + "\n")
                result = self.run_checker()
                self.assertEqual(result.returncode, 1, result.stdout + result.stderr)
                self.assertIn("PLACEHOLDER:", result.stdout)

    def test_output_is_utf8_regardless_of_locale_encoding(self):
        self.write("knowledge/claim.md", '<a id="cläim-name"></a>\n')
        env = {**os.environ, "PYTHONIOENCODING": "cp1252", "PYTHONUTF8": "0"}
        result = subprocess.run([sys.executable, str(self.checker)], cwd=self.repo, capture_output=True, env=env, timeout=15)
        self.assertEqual(result.returncode, 1, result.stderr)
        self.assertIn("cläim-name", result.stdout.decode("utf-8"))

    def test_conversion_debris_and_strict_instance_placeholders(self):
        self.write("CORE.md", "{=html}\n\\<oops\\>\n")
        self.write("LEDGER.md", "`<question>`\n")
        self.finding("addf/CORE.md:1: CONVERSION: pandoc artifact `{=html}`")
        self.finding("addf/CORE.md:2: CONVERSION: escaped angle bracket from conversion")
        self.finding("addf/LEDGER.md:1: PLACEHOLDER: unfilled placeholder `<question>`")

    def test_relative_links_and_explicit_and_heading_anchors(self):
        self.write("knowledge/a.md", "[explicit](b.md#durable)\n[heading](b.md#some-heading)\n")
        self.write("knowledge/b.md", '# Some heading\n<a id="durable"></a>\n')
        self.clean()
        self.write("knowledge/a.md", "[bad](b.md#missing)\n[absent](missing.md)\n")
        self.finding("addf/knowledge/a.md:1: LINK_ANCHOR: anchor does not exist: b.md#missing")
        self.finding("addf/knowledge/a.md:2: LINK_TARGET: target does not exist: missing.md")

    def test_links_with_spaces_and_code_examples(self):
        self.write("knowledge/with space.md", "# Target\n")
        self.write("knowledge/a.md", "[space](<with space.md#target>)\n`[example](absent.md)`\n```md\n[example](missing.md)\n```\n")
        self.clean()

    def test_task_cursor_and_index(self):
        self.task()
        self.clean()
        self.task(TASK.replace("DEFINE -> IMPLEMENT", "DEFINE -> VERIFY"))
        self.finding("addf/tasks/example/TASK.md:5: TASK_CURSOR: State `IMPLEMENT` is not the final State path entry `VERIFY`")
        self.write("INDEX.md", "## Active Tasks\n`NONE`.\n")
        self.finding("addf/INDEX.md:1: INDEX_TASK: active Task is not listed: tasks/example/TASK.md")

    def test_index_state_duplicate_and_terminal_entries(self):
        self.task()
        self.write("INDEX.md", "## Active Tasks\n- [Example](tasks/example/TASK.md), State: `VERIFY`.\n")
        self.finding("addf/INDEX.md:2: INDEX_STATE: tasks/example/TASK.md is `IMPLEMENT`, index says `VERIFY`")
        self.write("INDEX.md", "## Active Tasks\n- [Example](tasks/example/TASK.md), State: `IMPLEMENT`.\n- [Again](tasks/example/TASK.md), State: `IMPLEMENT`.\n")
        self.finding("addf/INDEX.md:3: INDEX_TASK: duplicate Task entry: tasks/example/TASK.md")
        self.write("tasks/example/TASK.md", self.terminal())
        self.finding("addf/INDEX.md:2: INDEX_TASK: terminal Task still listed: tasks/example/TASK.md")

    def test_required_spec_and_reachable_unique_child_tree(self):
        (self.root / "SPEC.md").unlink()
        self.finding("addf/SPEC.md:1: SPEC_ROOT: SPEC.md is missing")
        self.write("SPEC.md", "# Spec\n## Specification map\n1. [Child](spec/child.md)\n")
        self.finding("addf/SPEC.md:3: SPEC_CHILD: child specification does not exist: spec/child.md")
        self.write("spec/child.md", "# Child\n## Subspecifications\n1. [Grandchild](grandchild.md)\n")
        self.write("spec/grandchild.md", "# Grandchild\n## Subspecifications\n`NONE`.\n")
        self.clean()
        self.write("spec/orphan.md", "# Orphan\n")
        self.finding("addf/spec/orphan.md:1: SPEC_UNREACHABLE: specification is not reachable from SPEC.md")
        (self.root / "spec/orphan.md").unlink()
        self.write("SPEC.md", "# Spec\n## Specification map\n1. [Child](spec/child.md)\n2. [Again](spec/./child.md)\n")
        self.finding("addf/SPEC.md:4: SPEC_DUPLICATE: child specification is already reachable: spec/./child.md")

    def test_spec_history_target_is_not_normative(self):
        self.write("SPEC.md", "# Spec\n## Specification map\n`NONE`.\n[Old](history:notes.md#old)\n")
        self.write(".archive/notes.md", '<a id="old"></a>\n')
        self.finding("addf/SPEC.md:4: SPEC_HISTORY: current specification cannot target history:notes.md#old")
        self.write("SPEC.md", "# Spec\n## Specification map\n`NONE`.\nThe `history:` prefix and `history:notes.md#old` are examples.\n")
        self.clean()

    def test_durable_record_values_and_examples(self):
        self.write("knowledge/a.md", "# Knowledge\n- Basis: `framework:SPEC.md`\n")
        self.clean()
        self.write("knowledge/a.md", "# Knowledge\n- Basis: `framework:missing.md`\n")
        self.finding("addf/knowledge/a.md:2: REF_TARGET: target does not exist: framework:missing.md")
        self.write("knowledge/a.md", "# Knowledge\nExamples `framework:missing.md` and `history:absent.md`.\n```md\n- Basis: `framework:missing.md`\n```\n")
        self.write("templates/example.md", "- Basis: `framework:<target>`\n[Example](missing.md)\n")
        self.clean()

    def test_namespace_paths_reject_absolute_parent_and_archive_escape(self):
        for ref in ("framework:../SPEC.md", "framework:/SPEC.md", "framework:C:/SPEC.md", "framework:.archive/notes.md", "history:../SPEC.md", "history:/notes.md", "history:C:\\notes.md"):
            with self.subTest(ref=ref):
                self.write("knowledge/a.md", "- Basis: `" + ref + "`\n")
                self.finding("addf/knowledge/a.md:1: REF_PATH: invalid durable path: " + ref)

    def test_history_anchor_and_terminal_checkpoint_fallback(self):
        self.write(".archive/notes.md", '<a id="old"></a>\n')
        self.write("knowledge/a.md", "- Basis: `history:notes.md#old`\n")
        self.clean()
        self.write("knowledge/a.md", "- Basis: `history:notes.md#missing`\n")
        self.finding("addf/knowledge/a.md:1: REF_ANCHOR: anchor does not exist: history:notes.md#missing")
        self.task(self.terminal())
        self.write("knowledge/a.md", "- Basis: `history:tasks/example/TASK.md#verification-works`\n")
        self.clean()
        self.task()
        self.finding("addf/knowledge/a.md:1: REF_TARGET: target does not exist: history:tasks/example/TASK.md#verification-works")

    def test_current_task_version_and_v2_fields(self):
        self.task(TASK.replace("**Contract version:** `2`\n", ""))
        self.finding("addf/tasks/example/TASK.md:1: TASK_VERSION: Task must declare Contract version `2`")
        self.task(TASK.replace("**Open obligations:** `NONE`\n", ""))
        self.finding("addf/tasks/example/TASK.md:1: TASK_OBLIGATIONS: Open obligations field is missing")
        self.task(TASK.replace("- Proposed delta: `NONE`\n", ""))
        self.finding("addf/tasks/example/TASK.md:8: TASK_SPEC: Specification impact requires Proposed delta")

    def test_terminal_obligations_and_publication(self):
        self.task(self.terminal().replace("**Open obligations:** `NONE`", "**Open obligations:** Run a probe before VERIFY."))
        self.finding("addf/tasks/example/TASK.md:7: TASK_OBLIGATIONS: terminal Task must have Open obligations `NONE`")
        self.task(self.terminal().replace("- Terminal publication: `NONE`", "- Terminal publication: `PENDING`"))
        self.finding("addf/tasks/example/TASK.md:11: TASK_SPEC: terminal Task cannot have Terminal publication `PENDING`")

    def test_claim_fields_anchor_uniqueness_and_state(self):
        self.task(TASK.replace("- Scope: This repository.\n", ""))
        self.finding("addf/tasks/example/TASK.md:16: CLAIM_FIELD: Claim requires Scope")
        self.task(TASK.replace('<a id="works"></a>\n', ""))
        self.finding("addf/tasks/example/TASK.md:15: CLAIM_ANCHOR: Claim requires an explicit stable local anchor")
        self.task(TASK + '\n<a id="works"></a>\n')
        self.finding("addf/tasks/example/TASK.md:25: ANCHOR_DUPLICATE: duplicate explicit anchor: works")
        self.task(TASK.replace("- State: `UNVERIFIED`", "- State: `PARTIALLY_VERIFIED`"))
        self.finding("addf/tasks/example/TASK.md:17: CLAIM_STATE: invalid Claim State: PARTIALLY_VERIFIED")

    def test_settled_claim_basis_embedded_and_standalone(self):
        settled = TASK.replace("- State: `UNVERIFIED`", "- State: `VERIFIED`").replace("Pending execution.", "[Conclusion](#verification-works).")
        self.task(settled + VERIFICATION)
        self.clean()
        self.task(settled.replace("#verification-works", "verifications/works.md#conclusion"))
        self.write("tasks/example/verifications/works.md", STANDALONE)
        self.clean()
        self.write("tasks/example/verifications/works.md", '# Notes\n<a id="conclusion"></a>\nA command passed.\n')
        self.finding("addf/tasks/example/TASK.md:20: CLAIM_BASIS: settled Basis must resolve to an explicit Verification conclusion")

    def test_verification_requires_fields_matching_claim_and_state(self):
        settled = TASK.replace("- State: `UNVERIFIED`", "- State: `VERIFIED`").replace("Pending execution.", "[Conclusion](#verification-works).")
        self.task(settled + VERIFICATION.replace("- Method: Run the checker.\n", ""))
        self.finding("addf/tasks/example/TASK.md:20: CLAIM_BASIS: settled Basis must resolve to an explicit Verification conclusion")
        self.task(settled + VERIFICATION.replace("- Conclusion: `VERIFIED`", "- Conclusion: `REFUTED`"))
        self.finding("addf/tasks/example/TASK.md:20: CLAIM_BASIS: Verification conclusion does not supply Claim State `VERIFIED`")

    def test_arbitrary_claim_bullets_are_not_records(self):
        self.task(TASK + "\n## Notes\n- Claim: This word is discussed here.\n")
        self.clean()

    def test_default_archive_checks_only_terminality(self):
        legacy = "# Task\n**State:** `COMPLETED`\n**State path:** `DEFINE -> COMPLETED`\n**Resume at:** `NONE`\n<unfilled>\n[broken](missing.md)\n{=html}\n"
        self.task(legacy, archived=True)
        self.write(".archive/task-contract-v1.txt", "bad entry\nbad entry\n")
        self.clean()
        self.task(legacy.replace("**Resume at:** `NONE`", "**Resume at:** Continue."), archived=True)
        self.finding("addf/.archive/tasks/example/TASK.md:4: TASK_CURSOR: terminal State `COMPLETED` must have Resume at `NONE`")

    def test_archive_active_cursor_is_always_rejected(self):
        self.task(archived=True)
        self.finding("addf/.archive/tasks/example/TASK.md:4: TASK_ARCHIVE: archived Task retains active State `IMPLEMENT`")

    def test_include_archive_v2_and_exact_v1_baseline(self):
        self.clean("--include-archive")
        self.task(self.terminal(), archived=True)
        self.clean("--include-archive")
        self.task(self.terminal().replace("**Contract version:** `2`\n", ""), archived=True)
        self.finding("addf/.archive/tasks/example/TASK.md:1: TASK_VERSION: Task must declare Contract version `2`", "--include-archive")
        self.write(".archive/task-contract-v1.txt", "example\n")
        self.clean("--include-archive")
        self.write(".archive/tasks/example/notes.md", "[broken](missing.md)\n<unfilled>\n")
        self.clean("--include-archive")
        self.task(self.terminal(), archived=True)
        self.finding("addf/.archive/task-contract-v1.txt:1: BASELINE_STALE: entry does not name an archived v1 Task: example", "--include-archive")

    def test_include_archive_checks_retired_open_claims(self):
        claim = "# Open Claim: works\n## Claim\n<a id=\"works\"></a>\n- Claim: The checker works.\n- State: `VERIFIED`\n- Scope: This repository.\n- Consequence if false: Broken checks.\n- Basis: `framework:evidence/works.md#conclusion`\n"
        self.write(".archive/open-claims/works.md", claim)
        self.write("evidence/works.md", STANDALONE.replace("[works](../TASK.md#works).", "**Reference:** `history:open-claims/works.md#works`"))
        self.write("evidence/other.md", STANDALONE.replace("[works](../TASK.md#works).", "**Reference:** `history:open-claims/other.md#other`"))
        self.write(".archive/open-claims/other.md", claim.replace("\"works\"", "\"other\"").replace("works.", "other."))
        self.clean("--include-archive")
        self.write(".archive/open-claims/works.md", claim.replace("#conclusion", "#missing"))
        self.clean()
        self.finding("addf/.archive/open-claims/works.md:8: REF_ANCHOR: anchor does not exist: framework:evidence/works.md#missing", "--include-archive")
        self.write(".archive/open-claims/works.md", claim.replace("evidence/works.md", "evidence/other.md"))
        self.finding("addf/.archive/open-claims/works.md:8: CLAIM_BASIS: settled Basis must resolve to an explicit Verification conclusion", "--include-archive")

    def test_archive_v2_defects_are_opt_in(self):
        self.task(self.terminal(), archived=True)
        self.write(".archive/tasks/example/notes.md", "[broken](missing.md)\n")
        self.clean()
        self.finding("addf/.archive/tasks/example/notes.md:1: LINK_TARGET: target does not exist: missing.md", "--include-archive")

    def test_baseline_malformed_duplicate_stale_and_not_current_exemption(self):
        self.write(".archive/task-contract-v1.txt", "../example\nmissing\nmissing\n")
        self.finding("addf/.archive/task-contract-v1.txt:1: BASELINE_ENTRY: invalid v1 Task name: ../example", "--include-archive")
        self.finding("addf/.archive/task-contract-v1.txt:2: BASELINE_STALE: entry does not name an archived v1 Task: missing", "--include-archive")
        self.finding("addf/.archive/task-contract-v1.txt:3: BASELINE_DUPLICATE: duplicate v1 Task name: missing", "--include-archive")
        self.task(TASK.replace("**Contract version:** `2`\n", ""))
        self.write(".archive/task-contract-v1.txt", "example\n")
        self.finding("addf/tasks/example/TASK.md:1: TASK_VERSION: Task must declare Contract version `2`", "--include-archive")

    def test_invalid_utf8_and_multiple_failures_are_stable(self):
        path = self.write("knowledge/bad.md", "")
        path.write_bytes(b"\xff")
        self.write("knowledge/z.md", "[broken](missing.md)\n")
        self.write("knowledge/a.md", "<unfilled>\n")
        first = self.finding("addf/knowledge/bad.md:1: READ: cannot read UTF-8 file")
        second = self.run_checker(cwd=self.root)
        self.assertEqual(first.stdout, second.stdout)
        self.assertEqual(first.stdout.splitlines(), sorted(first.stdout.splitlines()))

    def test_indented_prose_does_not_extend_a_previous_record(self):
        text = self.terminal().replace("## Specification impact", "## Owned artifacts\n- A retained observation\n  explaining the outcome.\n## Specification impact")
        self.task(text)
        self.clean()

    def test_archive_v1_exemption_excludes_unreadable_owned_artifacts(self):
        self.task(self.terminal().replace("**Contract version:** `2`\n", ""), archived=True)
        self.write(".archive/task-contract-v1.txt", "example\n")
        self.write(".archive/tasks/example/unreadable.md", "").write_bytes(b"\xff")
        self.clean("--include-archive")

    def test_duplicate_resume_fields_are_not_overwritten(self):
        self.task(TASK.replace("**State:** `IMPLEMENT`", "**State:** `IMPLEMENT`\n**State:** `IMPLEMENT`"))
        self.finding("addf/tasks/example/TASK.md:5: TASK_CURSOR: duplicate Resume field: State")

    def test_duplicate_claim_fields_are_not_overwritten(self):
        self.task(TASK.replace("- State: `UNVERIFIED`", "- State: `REFUTED`\n- State: `UNVERIFIED`"))
        self.finding("addf/tasks/example/TASK.md:18: CLAIM_FIELD: duplicate Claim field: State")

    def test_code_in_method_and_claim_prose_is_illustrative(self):
        self.task(TASK.replace("The checker works.", "The checker rejects `framework:missing.md`.") + "\n### Probe\n- Method: Reject `history:absent.md`.\n")
        self.clean()

    def test_multiline_basis_and_verification_claim(self):
        settled = TASK.replace("- State: `UNVERIFIED`", "- State: `VERIFIED`").replace("- Basis: Pending execution.", "- Basis:\n  [Conclusion](#verification-works).")
        self.task(settled + VERIFICATION.replace("- Claim: [works](#works).", "- Claim:\n  [works](#works)."))
        self.clean()

    def test_v2_requires_resume_section_and_impact_value_contract(self):
        self.task(TASK.replace("## Resume\n", ""))
        self.finding("addf/tasks/example/TASK.md:1: TASK_CURSOR: Task requires a Resume section")
        self.task(TASK.replace("- Current contract: `framework:SPEC.md`", "- Current contract: Read the docs."))
        self.finding("addf/tasks/example/TASK.md:9: TASK_SPEC: Current contract must name current specification references or NONE")
        self.task(TASK.replace("- Terminal publication: `PENDING`", "- Terminal publication: Almost ready."))
        self.finding("addf/tasks/example/TASK.md:11: TASK_SPEC: active Task must have Terminal publication `PENDING`")

    def test_completed_claim_must_be_verified(self):
        self.task(self.terminal().replace("- State: `VERIFIED`", "- State: `UNVERIFIED`", 1))
        self.finding("addf/tasks/example/TASK.md:17: CLAIM_STATE: completed Task requires VERIFIED success Claims")

    def test_cursor_malformed_paths_and_missing_resume(self):
        for old, new in (("DEFINE -> IMPLEMENT", "IMPLEMENT"), ("DEFINE -> IMPLEMENT", "DEFINE -> ALMOST -> IMPLEMENT"), ("DEFINE -> IMPLEMENT", "DEFINE -> FAILED -> IMPLEMENT")):
            with self.subTest(new=new):
                self.task(TASK.replace(old, new))
                self.finding("addf/tasks/example/TASK.md:5: TASK_CURSOR: State path must start at DEFINE and contain only valid transitions before its final State")
        self.task(TASK.replace("**Resume at:** Run the checker.", "**Resume at:** `NONE`"))
        self.finding("addf/tasks/example/TASK.md:6: TASK_CURSOR: active State `IMPLEMENT` needs a concrete Resume at")

    def test_spec_cycle_and_escaped_history_target(self):
        self.write("SPEC.md", "# Spec\n## Specification map\n1. [Child](spec/child.md)\n")
        self.write("spec/child.md", "# Child\n## Subspecifications\n1. [Root](../SPEC.md)\n")
        self.finding("addf/spec/child.md:3: SPEC_DUPLICATE: child specification is already reachable: ../SPEC.md")
        self.write("spec/child.md", "# Child\n- Basis: `history:notes.md`\n")
        self.write(".archive/notes.md", "# Notes\n")
        self.finding("addf/spec/child.md:2: SPEC_HISTORY: current specification cannot target history:notes.md")

    def test_percent_encoded_namespace_escape_and_normalized_identity(self):
        self.write("knowledge/a.md", "- Basis: `framework:%2e%2e/SPEC.md`\n")
        self.finding("addf/knowledge/a.md:1: REF_PATH: invalid durable path: framework:%2e%2e/SPEC.md")
        self.write("knowledge/a.md", "- Basis: `framework:./SPEC.md`\n")
        self.clean()

    def test_wrong_claim_verification_is_rejected(self):
        settled = TASK.replace("- State: `UNVERIFIED`", "- State: `VERIFIED`").replace("Pending execution.", "[Conclusion](#verification-works).")
        self.task(settled + VERIFICATION.replace("#works", "#verification-works"))
        self.finding("addf/tasks/example/TASK.md:20: CLAIM_BASIS: settled Basis must resolve to an explicit Verification conclusion")

    def test_default_archive_ignores_invalid_owned_utf8(self):
        self.task(self.terminal(), archived=True)
        self.write(".archive/tasks/example/bad.md", "").write_bytes(b"\xff")
        self.clean()
        self.finding("addf/.archive/tasks/example/bad.md:1: READ: cannot read UTF-8 file", "--include-archive")

    def test_code_formatted_headings_preserve_claim_structure_and_link_anchors(self):
        self.task(TASK.replace("#### works", "#### `works`"))
        self.write("knowledge/a.md", "[Heading](b.md#some-code-heading)\n")
        self.write("knowledge/b.md", "# Some `code` heading\n")
        self.clean()

    def test_multiline_malformed_state_still_produces_one_line_diagnostics(self):
        self.task(TASK.replace("**State:** `IMPLEMENT`", "**State:** `IMPLEMENT\n  ALMOST`"))
        result = self.run_checker()
        self.assertEqual(result.returncode, 1)
        for line in result.stdout.splitlines():
            self.assertRegex(line, r"^.+:[1-9][0-9]*: [A-Z][A-Z0-9_]*: .+$")

    def test_numbered_links_outside_specification_map_are_ordinary_references(self):
        self.write("SPEC.md", "# Spec\n## Specification map\n`NONE`.\n## Requirements\n1. Read [the specification](SPEC.md).\n")
        self.clean()

    def test_bulleted_maps_traverse_children_in_document_order(self):
        self.write("SPEC.md", "# Spec\n## Specification map\n- [Child](spec/child.md)\n")
        self.write("spec/child.md", "# Child\n## Subspecifications\n- [Grandchild](grandchild.md)\n")
        self.write("spec/grandchild.md", "# Grandchild\n## Subspecifications\n`NONE`.\n")
        self.clean()
        self.write("SPEC.md", "# Spec\n## Specification map\n- [Child](spec/child.md)\n- [Again](spec/./child.md)\n")
        result = self.finding("addf/SPEC.md:4: SPEC_DUPLICATE: child specification is already reachable: spec/./child.md")
        self.assertEqual(result.stdout, "addf/SPEC.md:4: SPEC_DUPLICATE: child specification is already reachable: spec/./child.md\n")

    def test_declared_specification_maps_require_children_or_explicit_none(self):
        self.write("SPEC.md", "# Spec\n## Specification map\nNo decision recorded.\n")
        self.finding("addf/SPEC.md:2: SPEC_MAP: Specification map requires child Markdown links or explicit NONE")
        self.write("SPEC.md", "# Spec\n## Specification map\n1. [Child](spec/child.md)\n")
        self.write("spec/child.md", "# Child\n## Subspecifications\n")
        self.finding("addf/spec/child.md:2: SPEC_MAP: Subspecifications requires child Markdown links or explicit NONE")

    def test_specification_map_sections_are_required(self):
        self.write("SPEC.md", "# Spec\n")
        self.finding("addf/SPEC.md:1: SPEC_MAP: Specification map section is missing")
        self.write("SPEC.md", "# Spec\n## Specification map\n- [Child](spec/child.md)\n")
        self.write("spec/child.md", "# Child\n")
        self.finding("addf/spec/child.md:1: SPEC_MAP: Subspecifications section is missing")

    def test_declared_map_children_conflict_with_none(self):
        self.write("SPEC.md", "# Spec\n## Specification map\n`NONE`.\n1. [Child](spec/child.md)\n")
        self.write("spec/child.md", "# Child\n## Subspecifications\n`NONE`.\n")
        self.finding("addf/SPEC.md:2: SPEC_MAP: Specification map cannot combine child links with NONE")

    def test_malformed_explicit_anchors_are_rejected_in_framework_root_documents(self):
        for anchor in ('<a id="Claim-name"></a>', '<a id="cläim-name"></a>', '<a id="claim_name"></a>', '<a id="claim--name"></a>', '<a id="claim-name-"></a>', '<a id="claim-name">', '<a id="claim-name">text</a>', "<a id='claim-name'></a>", '<a\n id="claim-name"></a>', '<a id = "claim-name"></a>', '<A ID="claim-name"></A>'):
            with self.subTest(anchor=anchor):
                self.write("CORE.md", "# Core\n" + anchor + "\n")
                self.finding("addf/CORE.md:2: ANCHOR_FORMAT: explicit anchor must use an empty lowercase ASCII kebab anchor")
        self.write("CORE.md", '# Core\n<a id="claim-name"></a>\n`<a id="Bad"></a>`\n```html\n<a id="Bad"></a>\n```\n')
        self.clean()

    def test_current_knowledge_settled_basis_requires_a_verification(self):
        knowledge = """# Knowledge
## Claim
<a id="known"></a>
- Claim: The result is known.
- State: `VERIFIED`
- Scope: This fixture.
- Consequence if false: Current retrieval is misleading.
- Basis: A command passed.
"""
        self.write("knowledge/known.md", knowledge)
        self.finding("addf/knowledge/known.md:8: CLAIM_BASIS: settled Basis must resolve to an explicit Verification conclusion")

        self.write(
            "knowledge/known.md",
            knowledge.replace(
                "A command passed.",
                "[Conclusion](history:tasks/source/verifications/known.md#conclusion).",
            ),
        )
        self.write(
            ".archive/tasks/source/verifications/known.md",
            """# Verification: known
## Claim
**Reference:** `framework:knowledge/known.md#known`
## Method
Inspect the result.
## Evidence considered
The observed fixture.
## Conclusion
<a id="conclusion"></a>
**Result:** `VERIFIED`
The result matches.
## Remaining uncertainty
`NONE`.
""",
        )
        self.clean()

    def test_promoted_knowledge_accepts_unchanged_originating_verification(self):
        self.task(self.terminal(), archived=True)
        knowledge = """# Knowledge: checker result
## Claim
<a id="known"></a>
- Claim: The checker works.
- State: `VERIFIED`
- Scope: This repository.
- Consequence if false: Current retrieval is misleading.
- Basis: [Conclusion](history:tasks/example/TASK.md#verification-works).
"""
        self.write(
            "knowledge/works.md",
            knowledge,
        )
        self.clean()
        for changed in (
            knowledge.replace("The checker works.", "A different result holds."),
            knowledge.replace("This repository.", "Every repository."),
        ):
            with self.subTest(changed=changed):
                self.write("knowledge/works.md", changed)
                self.finding("addf/knowledge/works.md:8: CLAIM_BASIS: settled Basis must resolve to an explicit Verification conclusion")

    def test_decision_and_local_verification_claims_require_verified_basis(self):
        self.task()
        self.write(
            "tasks/example/decisions/choice.md",
            """# Decision
## Verification claims
<a id="choice-works"></a>
### choice-works
- Claim: The choice works.
- State: `VERIFIED`
- Scope: This fixture.
- Consequence if false: The decision is unsupported.
- Basis: A command passed.
""",
        )
        result = self.run_checker()
        self.assertEqual(result.returncode, 1, result.stdout + result.stderr)
        self.assertIn("addf/tasks/example/decisions/choice.md:9: CLAIM_BASIS:", result.stdout)

        (self.root / "tasks/example/decisions/choice.md").unlink()
        self.write(
            "tasks/example/verifications/local.md",
            """# Verification: local
## Claim
**Reference:** `LOCAL`
<a id="local-result"></a>
### local-result
- Claim: The local result holds.
- State: `VERIFIED`
- Scope: This fixture.
- Consequence if false: The result is unsupported.
- Basis: A command passed.
## Method
Inspect the result.
## Evidence considered
The observed fixture.
## Conclusion
<a id="conclusion"></a>
**Result:** `VERIFIED`
The result matches.
## Remaining uncertainty
`NONE`.
""",
        )
        result = self.run_checker()
        self.assertEqual(result.returncode, 1, result.stdout + result.stderr)
        self.assertIn("addf/tasks/example/verifications/local.md:10: CLAIM_BASIS:", result.stdout)
        local = self.root / "tasks/example/verifications/local.md"
        local.write_text(
            local.read_text(encoding="utf-8").replace("A command passed.", "[Conclusion](#conclusion)."),
            encoding="utf-8",
        )
        self.clean()

    def test_retention_and_promotion_placeholder_is_caught(self):
        placeholder = """
## Retention and promotion

For each Claim considered for Knowledge promotion:

### Promotion: `<claim anchor>`

- Claim: <link to the evaluated Claim>
- Will this Claim's validity outlive this Task and inform a future decision? `<yes | no>`, with reason
- Disposition: `<promoted to knowledge/<file> | carried forward to open-claims/<file> | not promoted — Task-scoped only>`
"""
        self.task(TASK + placeholder)
        result = self.run_checker()
        self.assertEqual(result.returncode, 1, result.stdout + result.stderr)
        self.assertIn("PLACEHOLDER:", result.stdout)

        filled = """
## Retention and promotion

For each Claim considered for Knowledge promotion:

### Promotion: `works`

- Claim: [works](#works).
- Will this Claim's validity outlive this Task and inform a future decision? `no`, scoped to this fixture.
- Disposition: `not promoted — Task-scoped only`
"""
        self.task(TASK + filled)
        self.clean()

    def test_standalone_claim_file_is_discovered_and_checked(self):
        self.task()
        claim = """# Claim: split
## Claim
<a id="split"></a>
- Claim: The split Claim is discovered.
- State: `UNVERIFIED`
- Scope: This fixture.
- Consequence if false: A split Claim set goes unchecked.
- Basis: Pending execution.
## Owning Task
- [Example](../TASK.md#works).
"""
        self.write("tasks/example/claims/split.md", claim)
        self.clean()
        self.write("tasks/example/claims/split.md", claim.replace("- Scope: This fixture.\n", ""))
        self.finding("addf/tasks/example/claims/split.md:4: CLAIM_FIELD: Claim requires Scope")

    def test_specification_rejects_relative_archive_dependency(self):
        self.write(
            "SPEC.md",
            "# Specification\n## Specification map\n`NONE`.\nFollow [the old contract](.archive/contract.md).\n",
        )
        self.write(".archive/contract.md", "# Old contract\n")
        self.finding("addf/SPEC.md:4: SPEC_HISTORY: current specification cannot target .archive/contract.md")

    def test_namespaced_markdown_link_with_spaces_is_parsed_once(self):
        self.write("knowledge/with space.md", "# Target\n")
        self.write(
            "knowledge/reference.md",
            "- Basis: [Target](<framework:knowledge/with space.md>)\n",
        )
        self.clean()


if __name__ == "__main__":
    unittest.main()
