# TASK — `acceptance of the fixup header and split-index fixes`

## Resume

**Contract version:** `2`

**State:** `COMPLETED`

**State path:** `DEFINE -> UNDERSTAND -> INVESTIGATE -> DECIDE -> IMPLEMENT -> VERIFY -> LEARN -> COMPLETED`

**Resume at:** `NONE`

**Open obligations:** `NONE`

## Owned artifacts

-   `ledger.md` - the questions that shaped this Task, with the operator's answers.
-   `review-scripts/` - the review's `edge.py`, `edge2.py` and `edge3.py`, copied unchanged except that `edge.py` reads the gir binary from `GIR_BIN` when set.
-   `run_review_scripts.py` - runs those scripts against two builds, normalises commit IDs and temporary paths, and diffs.
-   `acceptance.py` - the end-to-end scenarios `a2`, `a3` and `a4` against one gir binary, one `PASS`/`FAIL` line per check.
-   `logs/acceptance-new-20261002-1107.log` - Evidence: `acceptance.py` on the new build (first run).
-   `logs/acceptance-old-20261002-1107.log` - Evidence: `acceptance.py` on the old build `d246ab2` (first run).
-   `logs/control-skip-worktree-rebase-20261002-1107.log` - Evidence: control with plain git and no gir: rebasing onto `main` with a `skip-worktree` file that has a private edit and was added on `topic`.
-   `logs/platform-paths-20261002-1121.log` - Evidence: the first platform check, which showed the Windows normaliser bug and the run-to-run ordering difference.
-   `logs/platform-paths-20261002-1122.log` - Evidence: after the fixes: the normaliser on Windows-style paths, temporary directories following `TMPDIR`, and the review-script comparison reproducible across runs and temporary directories.
-   `logs/acceptance-new-20261002-1130.log` - Evidence: `acceptance.py` on the new build after the scenario correction.
-   `logs/acceptance-old-20261002-1130.log` - Evidence: `acceptance.py` on the old build after the scenario correction.
-   `logs/tests-on-old-build-20261002-1131.log` - Evidence: this branch's `tests/fixup.rs` and `tests/fixup_modes.rs` run against the source of `d246ab2`.
-   `logs/test-final-20261002-1131.log` - Evidence: `cargo test --no-fail-fast` on the tree committed as `2c6a352`.
-   `logs/clippy-20261002-1131.log` - Evidence: `cargo clippy --all-targets -- -D warnings` on that tree.
-   `logs/edge-old.log`, `logs/edge2-old.log`, `logs/edge3-old.log`, `logs/edge-new.log`, `logs/edge2-new.log`, `logs/edge3-new.log`, `logs/review-scripts-diff.log` - Evidence: the review scripts on `d246ab2` and on `HEAD`, normalised, and their diff.

## Specification impact

- Current contract: `framework:spec/fixup.md#req-fixup-staged-line-target`, `framework:spec/fixup.md#req-fixup-split`, `framework:spec/fixup.md#req-fixup-split-rollback`
- Proposed delta: `NONE`. This Task tests behavior; it changes no requirement.
- Terminal publication: `NONE`

## Define

### Objective

Show, comparing the reviewed build `d246ab2` with this branch's `HEAD`, that `06c5ee4` and `3d8a04b` fix the reported scenarios end to end and change nothing else the review exercised.

### Success criteria

<a id="a1-review-scripts"></a>
#### `a1-review-scripts`

-   Claim: With commit IDs and temporary paths normalised, the review's `edge.py`, `edge2.py` and `edge3.py` produce the same output on both builds except for the cases of findings #1 (index entries) and #2 (`-- ` lines), which on the new build behave as the fixes intend.
-   State: `VERIFIED`
-   Scope: Linux, git 2.56.0, old build `d246ab2`, new build this branch's `HEAD`.
-   Consequence if false: a fix changed behavior the review saw as correct, or did not fix a reported case.
-   Basis: [Verification](#verification-a1-review-scripts).

<a id="a2-header-end-to-end"></a>
#### `a2-header-end-to-end`

-   Claim: For the `-- ` deletion cases, `gir fixup` (or `--split`) followed by `git rebase --autosquash` leaves each original commit holding the intended lines on the new build, and not on the old one.
-   State: `VERIFIED`
-   Scope: Linux, git 2.56.0, old build `d246ab2`, new build this branch's `HEAD`.
-   Consequence if false: the fix looks right in `--dry-run` but the folded history is wrong.
-   Basis: [Verification](#verification-a2-header-end-to-end).

<a id="a3-split-end-to-end"></a>
#### `a3-split-end-to-end`

-   Claim: For a sparse checkout, a `skip-worktree` file with a local edit and a `git add -N` file, `gir fixup --split`, `git rebase --autosquash`, then `git add -A` and `git commit -a` delete no hidden file, commit no private edit and leave the intent-to-add file intent-to-add, on the new build; the old build fails this.
-   State: `VERIFIED`
-   Scope: Linux, git 2.56.0, old build `d246ab2`, new build this branch's `HEAD`.
-   Consequence if false: the harm the review described still happens after the fix.
-   Basis: [Verification](#verification-a3-split-end-to-end).

<a id="a4-setups"></a>
#### `a4-setups`

-   Claim: On the new build, a linked worktree, a sparse checkout in cone mode with a sparse index, running from a subdirectory, `gir amend --split`, `gir squash --split` and the interactive picker's `s` each keep index entries the split does not commit, and no `gir-split-index-*` file remains after a successful or a failed split.
-   State: `VERIFIED`
-   Scope: Linux, git 2.56.0, this branch's `HEAD`.
-   Consequence if false: the temporary index breaks or leaks in a setup the fix was not tested in.
-   Basis: [Verification](#verification-a4-setups).

<a id="a5-regression-tests"></a>
#### `a5-regression-tests`

-   Claim: The regression-relevant cases of `a2-header-end-to-end` to `a4-setups` are cargo integration tests, and `cargo test` and `cargo clippy --all-targets -- -D warnings` pass on the final revision.
-   State: `VERIFIED`
-   Scope: Linux, final revision of this branch.
-   Consequence if false: the scenarios regress unnoticed.
-   Basis: [Verification](#verification-a5-regression-tests).

### Constraints

-   Work in worktree `.worktrees/fixup-modes` on branch `feat/fixup-modes` (operator's answer in `ledger.md`).
-   If any scenario fails on the new build: stop, record it, and report to the operator before any fix; a fix is a separate Task.
-   No active Task overlaps this one.

### Material empirical premises

<a id="p1-old-build-fails"></a>
#### `p1-old-build-fails`

-   Claim: On the old build `d246ab2`, the review's scripts still show the #1 and #2 failures.
-   State: `VERIFIED`
-   Scope: Linux, git 2.56.0.
-   Consequence if false: the before side of every comparison is unsound.
-   Basis: [Verification](#verification-p1-old-build-fails).

### DEFINE gate

`ESTABLISHED`: on 2026-10-02 the operator explicitly agreed that the objective, success criteria and order are right, as written here (`ledger.md`).

## Understand

### Relevant context

-   The old binary is built from `git archive d246ab2` in a scratch directory, so no worktree or branch is added to the repository; the new binary is `target/debug/gir` at this branch's `HEAD`.
-   The review scripts run every case in a fresh temporary repository with `GIR_INTERACTIVE=0`, `GIT_EDITOR=true` and an empty global config. Commit IDs differ between runs, so `run_review_scripts.py` replaces them and temporary paths before diffing.
-   Their diff (`logs/review-scripts-diff.log`) differs only in `edge.py` cases 1 and 1b (finding #2) and the intent-to-add case, and in `edge2.py` cases 15 (sparse checkout) and 16 (`skip-worktree`) (finding #1); `edge3.py` is identical.
-   `edge.py` case 2, a file name with a space, fails identically on both builds. git writes a trailing tab after such a name in `---`/`+++` lines, and `parse_hunks` keeps it in the path. It is a new finding, recorded as Q11 in `ledger/fixup-review-20261002.md`, and outside this Task.

### Assumptions

-   `NONE`.

### Open questions

-   `NONE`. What remains is building the end-to-end scenarios, not an uncertainty.

### Deferred verification

-   `NONE`.

### UNDERSTAND gate

`ESTABLISHED`: both builds exist, the before side is confirmed, and the review scripts' comparison is in `logs/`.

## Investigate

No probe is needed: the success Claims are themselves the observations to make, and no design choice depends on an unknown.

### INVESTIGATE gate

`ESTABLISHED`: no decision-relevant uncertainty is open.

## Decide

-   **`acceptance.py`** in this bundle runs the end-to-end scenarios against a given gir binary and prints one `PASS` or `FAIL` line per check, so the same script runs on the old and the new build:
    -   `a2`: the `-- header` deletion and the `-- /dev/null` two-commit case through `gir fixup` or `gir fixup --split`, then `GIT_SEQUENCE_EDITOR=true git rebase -i --autosquash`, checking each original commit's file content afterwards.
    -   `a3`: a sparse checkout, a `skip-worktree` file with a local edit and a `git add -N` file through `gir fixup --split` and the rebase, then `git add -A` and `git commit -a`, checking that the hidden file is still in `HEAD`, the private edit is not, and the intent-to-add file is still intent-to-add.
    -   `a4` (new build): a linked worktree, cone mode with a sparse index, a subdirectory, `gir amend --split` and `gir squash --split` with `GIT_EDITOR=true`, and the picker's `s` with `GIR_INTERACTIVE=1`, each checking `git status` and `git ls-files -v`, and that no `gir-split-index-*` file remains in the git directory after a successful and a hook-refused split.
-   **Integration tests** for the regression-relevant cases: the two-commit `-- /dev/null` case folded by rebase (`tests/fixup.rs`); `git add -A` and `git commit -a` after a split in a sparse checkout and with `skip-worktree`, a linked worktree, cone mode with a sparse index, a subdirectory, and no leftover temporary index after success and failure (`tests/fixup_modes.rs`). Amend, squash and the picker share the split code path already covered there, so they stay in `acceptance.py` only.
-   **Verification strategy.** `acceptance.py` on both builds, logs committed; then `cargo test` and `cargo clippy --all-targets -- -D warnings`.
-   **On a failure** on the new build: stop, record it here, and report to the operator (Constraints).

### DECIDE gate

`ESTABLISHED`: every success Claim has a planned check.

## Implement

-   First run of `acceptance.py` (`logs/acceptance-new-20261002-1107.log`, `logs/acceptance-old-20261002-1107.log`): the new build passes 48 checks and fails 5; the old build passes 22 and fails 31.
-   All 5 new-build failures are `git rebase -i --autosquash main` in `a3.2`, `a4.4` and `a4.5` (and the history check that depends on it), each with `error: Your local changes to the following files would be overwritten by checkout: cfg.txt`.
-   Cause: the scenario, not gir. In those scenarios `cfg.txt`, the `skip-worktree` file with a private edit, is added on `topic`, so the rebase must check out `main`, where it does not exist. The control `logs/control-skip-worktree-rebase-20261002-1107.log` reproduces the same error with plain git and no gir, and shows the rebase succeeding when `cfg.txt` is committed on `main` instead.
-   Per Constraints, work stopped here and the operator was asked before changing the scenario.
-   At the operator's request, the scripts were made platform-neutral. Repositories already came from `tempfile.mkdtemp()`, which follows `TMPDIR`/`TEMP`/`TMP` (on Windows, `$env:TEMP`). `acceptance.py` now uses `os.devnull` for `GIT_CONFIG_GLOBAL` instead of `/dev/null`. `run_review_scripts.py` builds its path pattern from `tempfile.gettempdir()` and accepts either slash, instead of a hard-coded `/tmp/`.
-   That check found a flaw in the first `a1-review-scripts` comparison. `--split` orders commits by ID, and the review scripts do not fix commit dates, so IDs and the order changed between runs; the first comparison matched cases 9, 10 and 13 only by chance. `run_review_scripts.py` now sets `GIT_AUTHOR_DATE` and `GIT_COMMITTER_DATE`. The rerun (`logs/review-scripts-diff.log`, replaced) is identical across two runs and with another temporary directory, and still differs only in the #1 and #2 cases (`logs/platform-paths-20261002-1122.log`).
-   Not run on Windows: the Windows path handling is checked only by substituting a Windows temporary directory into the normaliser.
-   With the operator's approval, every `skip-worktree` scenario now commits `cfg.txt` in the base commit on `main` (`repo(base=...)`), and `a4.4`/`a4.5` expect the history `feat: a`, `feat: b`. Under the new rule `os-agnostic-code`, `review-scripts/edge.py` now uses `os.devnull` and requires `GIR_BIN`; the review-script comparison is byte-for-byte the same as before that change.
-   Second run: the new build passes all 53 checks (`logs/acceptance-new-20261002-1130.log`); the old build fails 31 (`logs/acceptance-old-20261002-1130.log`), across every `a2`, `a3` and `a4` scenario except `a4.7` (temporary index files did not exist before the fix).
-   `2c6a352` adds the integration tests: `dev_null_comment_split_folds_into_the_right_commits` (`tests/fixup.rs`); in `tests/fixup_modes.rs`, a cone-mode sparse-index setup in the setup table, a no-leftover-temporary-index assertion in both split tests, `add_all_and_commit_all_after_split_keep_hidden_and_private_files`, `split_in_a_linked_worktree_keeps_its_index_entries` and `split_from_a_subdirectory_keeps_index_entries`. `index_view` now reads `git ls-files --sparse -v`, so a sparse index's directory entries are compared too.
-   The same two test files run against the source of `d246ab2` fail 9 tests (4 in `tests/fixup.rs`, 5 in `tests/fixup_modes.rs`), every new or changed one (`logs/tests-on-old-build-20261002-1131.log`).
-   `amend`/`squash --split` and the picker's `s` share the split code path the integration tests cover, so they stay in `acceptance.py` only, as decided.

## Verify

<a id="verification-p1-old-build-fails"></a>
### Verification: `p1-old-build-fails`

- Claim: [p1-old-build-fails](#p1-old-build-fails)
- Method: `run_review_scripts.py` with the binary built from `d246ab2`.
- Evidence considered: `logs/edge-old.log` cases 1 and 1b (`cannot tell which commit header:5 belongs to`) and the intent-to-add case (`status after: '?? ita'`); `logs/edge2-old.log` cases 15 (` D out/c`) and 16 (` M cfg`).
- Conclusion: `VERIFIED`.
- Limitations: Linux, git 2.56.0.

<a id="verification-a1-review-scripts"></a>
### Verification: `a1-review-scripts`

- Claim: [a1-review-scripts](#a1-review-scripts)
- Method: `run_review_scripts.py` with the binary built from `d246ab2` and this branch's binary, with fixed commit dates; repeated in a second temporary directory.
- Evidence considered: `logs/review-scripts-diff.log`: `edge.py` differs only in cases 1 and 1b (finding #2: now `feat: lua` and a correct two-commit split) and the intent-to-add case (finding #1: now ` A ita`); `edge2.py` only in cases 15 (` D out/c` now clean) and 16 (` M cfg` now clean); `edge3.py` is identical. `logs/platform-paths-20261002-1122.log`: the comparison is identical across two runs and with another `TMPDIR`.
- Conclusion: `VERIFIED`.
- Limitations: Linux, git 2.56.0. The copies differ from the review's originals only in `edge.py`'s `GIR_BIN` and `os.devnull`, which change no output. The first comparison, before commit dates were fixed, was unsound; it is replaced.

<a id="verification-a2-header-end-to-end"></a>
### Verification: `a2-header-end-to-end`

- Claim: [a2-header-end-to-end](#a2-header-end-to-end)
- Method: `acceptance.py` scenarios `a2.1` to `a2.3` on both builds.
- Evidence considered: `logs/acceptance-new-20261002-1130.log`: all checks in `a2.1` to `a2.3` pass, including the history and file contents after `git rebase -i --autosquash`. `logs/acceptance-old-20261002-1130.log`: the old build fails every one of those checks (`a2.1` refused, `a2.2` silently commits to `feat: A`, `a2.3` cannot split).
- Conclusion: `VERIFIED`.
- Limitations: Linux, git 2.56.0.

<a id="verification-a3-split-end-to-end"></a>
### Verification: `a3-split-end-to-end`

- Claim: [a3-split-end-to-end](#a3-split-end-to-end)
- Method: `acceptance.py` scenarios `a3.1` to `a3.3` on both builds.
- Evidence considered: `logs/acceptance-new-20261002-1130.log`: all checks pass. After the split, the rebase, `git add -A` and `git commit -a`, `out/c.txt` is still in `HEAD` and still `skip-worktree`; `cfg.txt`'s private edit is neither committed nor lost; `ita.txt` stays intent-to-add. `logs/acceptance-old-20261002-1130.log`: the old build fails the status checks after the split in all three scenarios and refuses the rebase in `a3.1` and `a3.2`.
- Conclusion: `VERIFIED`.
- Limitations: Linux, git 2.56.0. `a3.3` checks the intent-to-add state after the split only: `git rebase` refuses to run while an intent-to-add entry exists, on any build. In `a3.1` the old build's `git add -A` ran after a refused rebase, so its later checks show the split's damage, not a rebase result.

<a id="verification-a4-setups"></a>
### Verification: `a4-setups`

- Claim: [a4-setups](#a4-setups)
- Method: `acceptance.py` scenarios `a4.1` to `a4.7` on the new build (and the old for comparison).
- Evidence considered: `logs/acceptance-new-20261002-1130.log`: all checks pass for a linked worktree, cone mode with a sparse index (the index stays sparse with the same entries), a subdirectory, `gir amend --split` and `gir squash --split` through the rebase, and the picker's `s`; no `gir-split-index-*` file remains after a successful or a hook-refused split, in the worktree's git directory or the common one. `logs/acceptance-old-20261002-1130.log` fails `a4.1` to `a4.6`.
- Conclusion: `VERIFIED`.
- Limitations: Linux, git 2.56.0. An interrupted gir (for example Ctrl+C mid-split) may still leave a temporary index file, because `Drop` does not run when the process is killed; not tested.

<a id="verification-a5-regression-tests"></a>
### Verification: `a5-regression-tests`

- Claim: [a5-regression-tests](#a5-regression-tests)
- Method: read the tests added in `2c6a352`; ran them against the source of `d246ab2`; ran `cargo test --no-fail-fast` and clippy on `2c6a352`.
- Evidence considered: `logs/tests-on-old-build-20261002-1131.log` (9 failures on the old source, every new or changed test); `logs/test-final-20261002-1131.log` (207 passed, 0 failed); `logs/clippy-20261002-1131.log` (no warnings).
- Conclusion: `VERIFIED`.
- Limitations: run on Linux only here; CI runs the same suite on Windows, which this Task did not observe.

VERIFY gate: `ESTABLISHED`; every success Claim is `VERIFIED`, on Linux.

## Learn

### Technical

-   When the behavior under test orders things by commit ID, as `--split` does, comparisons between runs need fixed commit dates; otherwise identical behavior looks different, or a difference can hide.
-   When a scenario fails, a control with plain git and no gir separates the tool's behavior from git's own refusals. Here it showed that the first `skip-worktree` rebase failures came from the scenario.

### Process

-   `diff-header-parsing` and `split-preserves-index` were completed on evidence from `--dry-run` output, `git status` and index flags. The operator had to ask for testing of the end state a user meets: the folded history, and `git add -A` / `git commit -a` after a split. Success criteria for a fix should include that end state from the start.
-   A question to the operator named the scenario's file (`cfg.txt`) without saying it existed only inside the test, and was unclear until explained.
-   Neither is addf friction; no framework change.

LEARN gate: `ESTABLISHED`.

## Retention and promotion

The technical Learnings are applied in `run_review_scripts.py` and `acceptance.py`, and the regression cases are enforced by the integration tests; no other permanent change.

### Promotion: success Claims

-   Claims: `a1-review-scripts`, `a2-header-end-to-end`, `a3-split-end-to-end`, `a4-setups`, `a5-regression-tests`.
-   Will this Claim's validity outlive this Task and inform a future decision? `no`; the behavior is specified in `spec/fixup.md` and guarded by integration tests.
-   Disposition: not promoted — Task-scoped only.

### Promotion: `p1-old-build-fails`

-   Claim: [p1-old-build-fails](#p1-old-build-fails)
-   Will this Claim's validity outlive this Task and inform a future decision? `no`; it describes the old build.
-   Disposition: not promoted — Task-scoped only.

No Claim promoted to Knowledge, and no Claim carried forward to `open-claims/`. The possible leftover temporary index after an interrupted split is recorded as Q12 in `ledger/fixup-review-20261002.md`.

## Archive readiness

The bundle holds its ledger, the review-script copies, `acceptance.py`, `run_review_scripts.py` and every log it cites under `logs/`; internal links are relative.
The old binary was built in a scratch directory and is not kept; `git archive d246ab2` rebuilds it.
References to source files, commits and `spec/fixup.md` are supplemental.

## Terminal record

### Summary

The header-parsing fix (`06c5ee4`) and the split-index fix (`3d8a04b`) are accepted, comparing the reviewed build `d246ab2` with this branch.
The review's own scripts differ only in the cases of findings #1 and #2.
End to end, through `git rebase --autosquash` and `git add -A` / `git commit -a`, the new build passes all 53 acceptance checks and the old build fails 31.
Regression cases are integration tests (`2c6a352`), which fail on the old source.
Two new findings are recorded in `ledger/fixup-review-20261002.md` (Q11, Q12).

### Gate basis

All five success Claims are `VERIFIED` through the Verifications above, on Linux with git 2.56.0. No Claim is carried forward.
