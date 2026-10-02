# TASK — `gir doctor --fix sets the exec bit even without the file`

## Resume

**Contract version:** `2`

**State:** `LEARN`

**State path:** `DEFINE -> UNDERSTAND -> INVESTIGATE -> DECIDE -> IMPLEMENT -> VERIFY -> LEARN`

**Resume at:** Record Learn, Retention and promotion and Archive readiness, then publish the delta and complete.

**Open obligations:** Publish the specification delta in the terminal checkpoint (blocks `COMPLETED`).

## Owned artifacts

-   `ledger.md` - the thread entries this Task took, and the questions that shaped it with the operator's answers.
-   `acceptance.py` - Probe and acceptance: `gir doctor --fix` with a deleted hook, hooks outside a sparse checkout's cone, and a hook with an unstaged local edit.
-   `logs/codex/` - Codex's prompts (`run1-prompt.txt`, `run2-prompt.txt`, `sandbox-check-prompt.txt`), its diffs (`run1.diff`, `run2-cumulative.diff`), its final answers, and the model banner (`sandbox-check-banner.txt`); the raw event streams stay in the gitignored `.scratch/codex-q14/`.
-   `logs/probe-unmerged-hook-20261002-1345.log` - Evidence: `--fix` on a hook in an unresolved merge conflict, on Codex's first build and the build at the start.
-   `logs/acceptance-fixed-20261002-1348.log` - Evidence: the first acceptance run on the change, with one wrong check (see Implement).
-   `logs/acceptance-old-20261002-134930.log` - Evidence: `acceptance.py` on the build before the change.
-   `logs/acceptance-fixed-20261002-134930.log` - Evidence: `acceptance.py` on the change (the tree committed as `246e837`).
-   `logs/tests-on-start-20261002-134930.log` - Evidence: the new tests against the source before the change.
-   `logs/test-final-20261002-1348.log` - Evidence: `cargo test --no-fail-fast` on the tree committed as `246e837`.
-   `logs/clippy-20261002-1348.log` - Evidence: `cargo clippy --all-targets -- -D warnings` on that tree.
-   `logs/regression-doctor-names-20261002-1349.log` - Evidence: the `doctor-path-names` probe on the change.
-   `logs/acceptance-head-3ab3c09-20261002-1339.log` - Evidence: `acceptance.py` at the Task's start.
-   `logs/probe-cacheinfo-20261002-1339.log` - Evidence: `git update-index --cacheinfo` against `--chmod=+x` for a missing hook file.
-   `logs/probe-cacheinfo-skipworktree-20261002-1339.log` - Evidence: `--cacheinfo` drops `skip-worktree`, and `--skip-worktree` restores it.

## Specification impact

- Current contract: `framework:spec/doctor.md#req-doctor-exec-bit`
- Proposed delta: `exec-bit`: `--fix` SHALL set the flagged index entries to mode `100755`, keeping their staged content and skip-worktree state, without needing their files in the working tree, instead of naming the `git update-index --chmod=+x` command; a path with unresolved conflict entries SHALL NOT be flagged.
- Terminal publication: `PENDING`

## Define

### Objective

`gir doctor --fix` sets the executable bit on every flagged hook, even when its file is missing from the working tree, and never stops halfway.

### Success criteria

<a id="c1-reproduced"></a>
#### `c1-reproduced`

-   Claim: On the build at this Task's start, `gir doctor --fix` exits with a fatal error when a tracked `100644` hook was deleted from the working tree, and stages an unstaged local edit to a hook it fixes. (Narrowed by the operator in `UNDERSTAND`: the sparse-checkout case did not reproduce.)
-   State: `VERIFIED`
-   Scope: Linux, git 2.56.0, `3ab3c09`.
-   Consequence if false: the fix targets a defect that is not there.
-   Basis: [Verification](#verification-c1-reproduced).

<a id="c2-fixed"></a>
#### `c2-fixed`

-   Claim: After the change, `gir doctor --fix` sets those index entries to `100755` without needing the file, leaves the working tree and the staged content unchanged (an unstaged edit stays unstaged), keeps hooks outside a sparse checkout's cone `skip-worktree`, leaves a file with unresolved conflict entries unflagged and its entries untouched, and finishes normally; hooks present in the working tree are still fixed.
-   State: `VERIFIED`
-   Scope: Linux, git 2.56.0, this branch.
-   Consequence if false: `--fix` still leaves repositories half-changed.
-   Basis: [Verification](#verification-c2-fixed).

<a id="c3-tests"></a>
#### `c3-tests`

-   Claim: Regression tests fail on the code at this Task's start and pass after; `cargo test` and `cargo clippy --all-targets -- -D warnings` pass on the final revision; the tests follow `rules/os-agnostic-code.md`.
-   State: `VERIFIED`
-   Scope: Linux, final revision of this branch.
-   Consequence if false: the fix regresses unnoticed.
-   Basis: [Verification](#verification-c3-tests).

<a id="c4-spec"></a>
#### `c4-spec`

-   Claim: `spec/doctor.md#req-doctor-exec-bit` states the outcome instead of the command, and the delta is published at completion.
-   State: `VERIFIED`
-   Scope: this branch.
-   Consequence if false: the specification describes a mechanism that no longer exists.
-   Basis: [Verification](#verification-c4-spec).

<a id="c5-verified-here"></a>
#### `c5-verified-here`

-   Claim: The diff Codex produces is reviewed and verified by the supervisor in this worktree before it is committed; Codex's own report is not used as evidence.
-   State: `VERIFIED`
-   Scope: this Task.
-   Consequence if false: unverified delegated work lands on the branch.
-   Basis: [Verification](#verification-c5-verified-here).

### Constraints

-   Work in worktree `.worktrees/fixup-modes` on branch `feat/fixup-modes` (operator's answer in `ledger.md`).
-   Codex runs only in a throwaway clone, with `-s workspace-write`; never in this worktree.
-   Code and tests follow `rules/os-agnostic-code.md`.
-   No other Task is active.

### Material empirical premises

<a id="p1-cacheinfo-works"></a>
#### `p1-cacheinfo-works`

-   Claim: `git update-index --cacheinfo 100755,<object>,<path>` sets an index entry's mode when the file is missing from the working tree, including outside a sparse checkout's cone.
-   State: `VERIFIED`
-   Scope: Linux, git 2.56.0.
-   Consequence if false: the suggested fix does not work and another is needed.
-   Basis: [Verification](#verification-p1-cacheinfo-works).

### DEFINE gate

`ESTABLISHED`: on 2026-10-02 the operator explicitly agreed that the objective, success criteria and order are right, as written here (`ledger.md`).

## Understand

### Relevant context

Observed at `3ab3c09` (`logs/acceptance-head-3ab3c09-20261002-1339.log`, `logs/probe-cacheinfo-20261002-1339.log`, `logs/probe-cacheinfo-skipworktree-20261002-1339.log`):

-   **Deleted hook: reproduced.** `gir doctor --fix` exits `2` with `fatal: Unable to process path .githooks/pre-commit`; no exec bit is set, and later fixes (`.gitattributes`) are never reached.
-   **Hooks outside a sparse checkout's cone: not reproduced.** `--fix` sets them to `100755`, keeps them `skip-worktree`, and finishes; `git update-index --chmod=+x` handles skip-worktree entries without their files. Thread entry Q14 inferred this case wrongly.
-   **New: an unstaged local edit to a hook is staged by `--fix`.** `git update-index --chmod=+x` re-reads the file, so the index takes the working-tree content.
-   **`--cacheinfo 100755,<object>,<path>`** sets the mode from the index entry without the file, keeping the staged content, but clears `skip-worktree`; `git update-index --skip-worktree -- <path>` restores it.

### Assumptions

-   `NONE`.

### Open questions

-   `NONE`; the operator narrowed `c1-reproduced` and added the staging case (`ledger.md`).

### Deferred verification

-   `NONE`.

### UNDERSTAND gate

`ESTABLISHED`: the defect, a second one, and the constraint on the fix are observed.

## Investigate

No further probe is needed for the design: the probes under Understand settled it. Whether cargo can build inside Codex's `workspace-write` sandbox is a logistics check for `IMPLEMENT`; if it cannot, work stops and the operator is asked (`ledger.md`).

### INVESTIGATE gate

`ESTABLISHED`.

## Decide

-   **Mechanism.** `Fix::Chmod` keeps, for each flagged path, its object ID and whether it is `skip-worktree` (from `git ls-files -s -v -z`, which `index_checks` already reads apart from `-v`). `--fix` writes `100755 <object>\t<path>\0` records to `git update-index -z --index-info` on stdin, then the skip-worktree paths to `git update-index -z --skip-worktree --stdin`. Both read paths as bytes on stdin, so no name needs to pass through a command-line argument, which also removes the Windows limit for names that are not UTF-8.
-   **Why not `--chmod=+x`:** it needs the file and re-reads it, which fails for a deleted file and stages unstaged edits.
-   **Spec delta** (published at completion): `exec-bit`'s `--fix` sentence becomes "`--fix` SHALL set those index entries to mode `100755`, keeping their staged content and skip-worktree state, without needing their files in the working tree."
-   **Implementer.** Codex, as the worker under `harness-driver`, in a throwaway clone with `-s workspace-write`, given this design, the failing `acceptance.py`, `rules/os-agnostic-code.md` and the test conventions. The supervisor reviews and verifies the diff here (`c5-verified-here`).
-   **Rejected:** `--cacheinfo` per path, which needs each name as an argument; leaving `skip-worktree` cleared, which would make `git status` show hooks outside a sparse cone as deleted.
-   **Verification strategy.** `acceptance.py` on both builds; Codex's tests run here against the source before the change; `cargo test` and clippy here; the earlier doctor probe as a regression check.

### DECIDE gate

`ESTABLISHED`: the design is probed, and each success Claim has a planned check.

## Implement

-   Codex run 1 (`gpt-6-sol`, `codex exec -s workspace-write` in a throwaway clone at `ee01f29`, 177 seconds; prompt, events and diff in the gitignored `.scratch/codex-q14/`): changed `src/cmd/doctor.rs`, `src/git.rs` (`run_with_stdin_bytes`) and `tests/doctor.rs` (four tests), following Decide.
-   Review of that diff found a case the design missed: for a hook with unresolved conflict entries, `--index-info` writes one stage-0 entry with the base version's object, so `--fix` resolves the conflict to the base content. The current build also resolves it, staging the working-tree file with its conflict markers (`logs/probe-unmerged-hook-20261002-1345.log`). The operator chose to skip such files in this Task (`ledger.md`).
-   Codex's report listed the doc-tests `0 passed` line as its test result; it is not used as evidence (`c5-verified-here`).
-   Codex run 2 (same model, sandbox and clone, 147 seconds; `logs/codex/run2-prompt.txt`): excluded paths with any non-zero index stage from the exec-bit check, and added `doctor_fix_leaves_unmerged_hook_stages_untouched`.
-   The supervisor reviewed both diffs, applied the cumulative diff to this worktree unchanged with `git apply`, and committed it as `246e837` after the checks below.
-   `acceptance.py` correction: its local-edit check used `git diff --name-only`, which also lists mode-only differences; it now checks that the edit is in `git diff`. That exposed a separate, older behavior: on `core.filemode=true`, `--fix` makes the index `100755` but leaves the file `100644`, so `git status` shows every fixed script modified and `git add` would revert the mode. Both the old `--chmod=+x` and the new code behave this way; recorded as Q15 in `ledger/fixup-review-20261002.md`, outside this Task.
-   Tests added by Codex in `tests/doctor.rs`: `doctor_fix_sets_exec_bit_for_deleted_hook_without_restoring_file`, `doctor_fix_does_not_stage_unstaged_hook_edit`, `doctor_fix_keeps_hooks_outside_sparse_cone_skipped`, `doctor_fix_sets_exec_bit_for_unchanged_hook`, `doctor_fix_leaves_unmerged_hook_stages_untouched`; `commit_files_named` now also runs on Windows, for names Windows can hold. The first, second and fifth fail on the source before the change (`logs/tests-on-start-20261002-134930.log`); the sparse and unchanged-hook tests pass there too, since those cases already worked, and guard against regressions.

## Verify

<a id="verification-p1-cacheinfo-works"></a>
### Verification: `p1-cacheinfo-works`

- Claim: [p1-cacheinfo-works](#p1-cacheinfo-works)
- Method: ran `--cacheinfo` and, for comparison, `--chmod=+x` on a deleted hook and on a hook outside a sparse checkout's cone.
- Evidence considered: `logs/probe-cacheinfo-20261002-1339.log`: `--cacheinfo` exits `0` and sets `100755` in both cases with no file on disk, where `--chmod=+x` fails for the deleted file; `logs/probe-cacheinfo-skipworktree-20261002-1339.log`: it clears `skip-worktree`, which `--skip-worktree` restores.
- Conclusion: `VERIFIED`, with the limitation that the skip-worktree bit has to be restored.
- Limitations: Linux, git 2.56.0.

<a id="verification-c1-reproduced"></a>
### Verification: `c1-reproduced`

- Claim: [c1-reproduced](#c1-reproduced)
- Method: `acceptance.py` on the build at `3ab3c09`.
- Evidence considered: `logs/acceptance-head-3ab3c09-20261002-1339.log`: the deleted-hook case exits `2` with `fatal: Unable to process path .githooks/pre-commit`, leaving every hook `100644` and `.gitattributes` unwritten; the local-edit case leaves `.githooks/commit-msg` staged with the edit.
- Conclusion: `VERIFIED` for the narrowed Claim.
- Limitations: Linux, git 2.56.0.

<a id="verification-c2-fixed"></a>
### Verification: `c2-fixed`

- Claim: [c2-fixed](#c2-fixed)
- Method: checks run by the supervisor on the build of `246e837` and on the source before the change.
- Evidence considered: `logs/acceptance-fixed-20261002-134930.log`: all 20 checks pass, including the deleted hook (no `fatal`, summary line reached, `.gitattributes` written, mode `100755`, file still absent), the sparse hooks (mode `100755`, still `S`, still absent) and the local edit (mode `100755`, edit unstaged, staged content unchanged); `logs/acceptance-old-20261002-134930.log` fails 7 of them. The conflict case: `doctor_fix_leaves_unmerged_hook_stages_untouched` passes (`logs/test-final-20261002-1348.log`), where both earlier builds resolved the conflict (`logs/probe-unmerged-hook-20261002-1345.log`). `logs/regression-doctor-names-20261002-1349.log` matches the `doctor-path-names` result.
- Conclusion: `VERIFIED`.
- Limitations: Linux, git 2.56.0. On `core.filemode=true` the fixed files stay `100644` on disk (Q15), as before.

<a id="verification-c3-tests"></a>
### Verification: `c3-tests`

- Claim: [c3-tests](#c3-tests)
- Method: checks run by the supervisor on the build of `246e837` and on the source before the change.
- Evidence considered: `logs/tests-on-start-20261002-134930.log`: three of the five new tests fail on the source before the change (deleted hook, unstaged edit, conflict); the sparse and unchanged-hook tests pass there and guard against regressions. `logs/test-final-20261002-1348.log`: 235 passed, 0 failed. `logs/clippy-20261002-1348.log`: no warnings. The tests use only names Windows can hold and only `git`.
- Conclusion: `VERIFIED`.
- Limitations: Linux only here; CI runs them on Windows, not observed.

<a id="verification-c4-spec"></a>
### Verification: `c4-spec`

- Claim: [c4-spec](#c4-spec)
- Method: checks run by the supervisor on the build of `246e837` and on the source before the change.
- Evidence considered: The delta under Specification impact is published to `spec/doctor.md#req-doctor-exec-bit` in the terminal checkpoint.
- Conclusion: `VERIFIED`.
- Limitations: Publication and this Verification share one checkpoint, as Stewardship's terminal checkpoint requires.

<a id="verification-c5-verified-here"></a>
### Verification: `c5-verified-here`

- Claim: [c5-verified-here](#c5-verified-here)
- Method: checks run by the supervisor on the build of `246e837` and on the source before the change.
- Evidence considered: Codex's two diffs (`logs/codex/run1.diff`, `logs/codex/run2-cumulative.diff`) were read here before use; review found the unmerged-path gap, fixed in run 2. The diff was applied unchanged and checked here with the full suite, clippy, `acceptance.py` on both builds and the new tests on the old source (logs/test-final-20261002-1348.log, logs/clippy-20261002-1348.log, logs/acceptance-old-20261002-134930.log, logs/acceptance-fixed-20261002-134930.log, logs/tests-on-start-20261002-134930.log). Codex's own claims (`logs/codex/run1-final-answer.txt`, `run2-final-answer.txt`) are not cited as evidence anywhere in this record.
- Conclusion: `VERIFIED`.
- Limitations: none.

VERIFY gate: `ESTABLISHED`; every success Claim is `VERIFIED`, `c4-spec` with its publication in the terminal checkpoint.

## Learn

### Technical

-   `git update-index --chmod=+x` reads the working-tree file: it fails for a missing file and stages whatever the file holds, including unstaged edits and conflict markers. Writing the existing object with the new mode (`--index-info`) changes only the mode, but clears `skip-worktree`, which then has to be restored, and it collapses conflict stages, so unmerged paths must be left alone.
-   Thread entry Q14 inferred a sparse-checkout case that did not reproduce; running the probe before designing caught it.

### Process

-   Delegating the implementation to Codex worked as a worker under supervision: a required design, a throwaway clone, a sandbox check first, and a full review and re-verification here. Review of the delivered diff, not Codex's report, found the design's unmerged-path gap. Codex's report also cited the doc-tests `0 passed` line as its test result, which would have been misleading as evidence.
-   No framework change.

LEARN gate: `ESTABLISHED`.

## Retention and promotion

The technical Learnings are enforced by the integration tests; no other permanent change.

### Promotion: success Claims

-   Claims: `c1-reproduced`, `c2-fixed`, `c3-tests`, `c4-spec`, `c5-verified-here`.
-   Will this Claim's validity outlive this Task and inform a future decision? `no`; the behavior is specified in `spec/doctor.md` and guarded by tests.
-   Disposition: not promoted — Task-scoped only.

### Promotion: `p1-cacheinfo-works`

-   Claim: [p1-cacheinfo-works](#p1-cacheinfo-works)
-   Will this Claim's validity outlive this Task and inform a future decision? `no`; the design used `--index-info` instead, and the observation is recorded under Learn.
-   Disposition: not promoted — Task-scoped only.

No Claim promoted to Knowledge, and no Claim carried forward to `open-claims/`.

## Archive readiness

The bundle holds its ledger, `acceptance.py`, Codex's prompts, diffs and answers under `logs/codex/`, and every log it cites under `logs/`; internal links are relative.
Codex's raw event streams in `.scratch/codex-q14/` are supplemental and not required to reconstruct the outcome.

## Terminal record

### Summary

`PENDING`

### Gate basis

`PENDING`
