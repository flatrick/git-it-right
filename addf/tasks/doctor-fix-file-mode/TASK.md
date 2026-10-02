# TASK — `gir doctor --fix makes fixed scripts executable on disk`

## Resume

**Contract version:** `2`

**State:** `IMPLEMENT`

**State path:** `DEFINE -> UNDERSTAND -> INVESTIGATE -> DECIDE -> IMPLEMENT -> VERIFY -> IMPLEMENT`

**Resume at:** Repair the four Windows-invalid-name test fixtures by making their Git index insertion use the same Windows protection setting as the older passing test, and report Git stderr if it still fails. Then rerun local checks and return to VERIFY.

**Open obligations:** The reported Windows test failures need a discriminating rerun (blocks `VERIFY`); `c3-os-agnostic` waits for the manual Windows result and Windows CI, which requires a decision on the disabled workflow (blocks `VERIFY`); `c5-spec` is published in the terminal checkpoint.

## Owned artifacts

-   `ledger.md` - the thread entries this Task took, and the questions that shaped it with the operator's answers.
-   `acceptance.py` - Probe and acceptance: premise `p1` with git alone, then `gir doctor --fix` on a plain repository, a deleted hook, hooks outside a sparse checkout's cone, a script with an unstaged edit, and a script replaced by a symlink.
-   `logs/acceptance-start-98feea1-20261002-1359.log` - Evidence: `acceptance.py` on the build at this Task's start (`98feea1`), with the first status check (see Investigate).
-   `logs/probe-chmod-after-fix-20261002-1400.log` - Evidence: the start build's `--fix`, then `chmod a+x` by hand on the fixed files.
-   `logs/acceptance-start-98feea1-20261002-1401.log` - Evidence: `acceptance.py`, status check corrected, on the build at this Task's start.
-   `logs/tests-on-start-98feea1-20261002-1402.log` - Evidence: the new tests against the source at `98feea1`.
-   `logs/test-final-20261002-1402.log` - Evidence: `cargo test --no-fail-fast` on the tree committed as `7ff434a`.
-   `logs/clippy-20261002-1402.log` - Evidence: `cargo clippy --all-targets -- -D warnings` on that tree.
-   `logs/acceptance-old-20261002-1403.log` - Evidence: `acceptance.py` on the build of `98feea1`.
-   `logs/acceptance-fixed-20261002-1403.log` - Evidence: `acceptance.py` on the build of the tree committed as `7ff434a`.
-   `criteria.py` - Probe: compares this file's Specification impact, Success criteria and premise with an earlier commit, ignoring State and Basis lines.
-   `logs/inspection-windows-branch-20261002-1404.log` - Evidence: the code change at `7ff434a`, every `cfg(unix)`/`cfg(not(unix))` in the touched files, and the installed Rust targets.
-   `logs/criteria-unchanged-20261002-1405.log` - Evidence: the Success criteria, Specification impact and premise at `98feea1` against this checkpoint, ignoring State and Basis lines.
-   `logs/verification-checkpoint-20261002-1235Z.log` - Evidence: draft PR #5, the disabled CI workflow, and the local verification checks at `7926f97`.

## Specification impact

- Current contract: `framework:spec/doctor.md#req-doctor-exec-bit`
- Proposed delta: `exec-bit`: on systems with an executable bit, `--fix` SHALL also make each fixed file that exists in the working tree executable, without changing its content. Exact wording settled in `DECIDE`.
- Terminal publication: `PENDING`

## Define

### Objective

After `gir doctor --fix`, each fixed script is executable on disk as well as in git, so `git status` stays clean and hooks run.

### Success criteria

<a id="c1-reproduced"></a>
#### `c1-reproduced`

-   Claim: On the build at this Task's start, after `gir doctor --fix` a fixed script is `100755` in the index but not executable on disk; `git status` shows it modified; `git add` stages `100644` again; and git does not run the hook. Each part is observed, not assumed.
-   State: `VERIFIED`
-   Scope: Linux, git 2.56.0, this branch.
-   Consequence if false: the fix targets a defect that is not there, or misses part of it.
-   Basis: [Verification](#verification-c1-reproduced).

<a id="c2-fixed"></a>
#### `c2-fixed`

-   Claim: After the change, on Unix: each fixed script that exists on disk is executable; `git status` shows no mode change for it; `git add` keeps `100755`; the hook runs. Files missing from the working tree (deleted, or outside a sparse checkout's cone) stay absent; file content and unstaged edits are untouched.
-   State: `VERIFIED`
-   Scope: Linux, git 2.56.0, this branch.
-   Consequence if false: fixed hooks still do not run, or `git add` undoes the fix.
-   Basis: [Verification](#verification-c2-fixed).

<a id="c3-os-agnostic"></a>
#### `c3-os-agnostic`

-   Claim: Windows has no executable bit: the on-disk step is Unix-only, written with both platform branches per `rules/os-agnostic-code.md`, and nothing changes on Windows.
-   State: `UNVERIFIED`
-   Scope: this branch; the Windows branch by inspection and CI.
-   Consequence if false: the change breaks or misbehaves on Windows.
-   Basis: [Verification](#verification-c3-os-agnostic): both branches exist and the Windows one is a no-op by inspection; the Windows CI part of the Scope needs a push and is not observed.

<a id="c4-tests"></a>
#### `c4-tests`

-   Claim: Regression tests fail on the code at this Task's start and pass after; `cargo test` and `cargo clippy --all-targets -- -D warnings` pass on the final revision.
-   State: `UNVERIFIED`
-   Scope: Linux, final revision of this branch.
-   Consequence if false: the fix regresses unnoticed.
-   Basis: The four Windows-invalid-name fixtures are being repaired after the operator reported their failures; the final test revision needs a new Verification. The earlier Linux result remains in [Verification](#verification-c4-tests).

<a id="c5-spec"></a>
#### `c5-spec`

-   Claim: `spec/doctor.md#req-doctor-exec-bit` adds the on-disk part, and the delta is published at completion.
-   State: `UNVERIFIED`
-   Scope: this branch.
-   Consequence if false: the specification does not describe `--fix`.
-   Basis: the delta is settled in Decide; publication belongs to the terminal checkpoint, which this sub-agent did not reach (see Verify).

<a id="c6-stops-for-decisions"></a>
#### `c6-stops-for-decisions`

-   Claim: The sub-agent stops and reports instead of deciding anything that changes these criteria or the specification beyond `c5-spec`.
-   State: `VERIFIED`
-   Scope: this Task.
-   Consequence if false: a delegated decision is made without the operator.
-   Basis: [Verification](#verification-c6-stops-for-decisions).

### Constraints

-   Work in worktree `.worktrees/fixup-modes` on branch `feat/fixup-modes` (operator's answer in `ledger.md`).
-   Code and tests follow `rules/os-agnostic-code.md`.
-   Delegated to a sub-agent from `UNDERSTAND` on; see `c6-stops-for-decisions`.
-   No other Task is active.

### Material empirical premises

<a id="p1-git-ignores-non-executable-hook"></a>
#### `p1-git-ignores-non-executable-hook`

-   Claim: With `core.hooksPath` pointing at `.githooks/`, git does not run a hook file that is not executable on disk, and says so.
-   State: `VERIFIED`
-   Scope: Linux, git 2.56.0.
-   Consequence if false: Q15's impact is only a dirty `git status`, not hooks failing to run.
-   Basis: [Verification](#verification-p1-git-ignores-non-executable-hook).

### DEFINE gate

`ESTABLISHED`: on 2026-10-02 the operator explicitly agreed that the objective, success criteria and order are right, as written here (`ledger.md`).

## Understand

### Relevant context

Observed on the build of `98feea1` (`logs/acceptance-start-98feea1-20261002-1359.log`, 33 PASS, 13 FAIL; each FAIL is a part of the defect):

-   **p1 holds.** With `core.hooksPath=.githooks` and the hook at mode `0644` on disk, `git commit` succeeds, does not run the hook, and prints `hint: The '.githooks/pre-commit' hook was ignored because it's not set as executable.`; the same hook at `0755` runs.
-   **Every part of c1 reproduces** with `core.filemode=true` (git's default on Linux): after `--fix` each fixed script is `100755` in the index and `0644` on disk; `git status --porcelain` shows `MM` for it; `git add` stages `100644` again; and the next commit ignores the fixed hook with the same hint.
-   **The same holds with an unstaged edit:** the edit stays unstaged, but `git diff` also shows `old mode 100755` / `new mode 100644`.
-   **Missing files are already handled:** a deleted hook stays absent and hooks outside a sparse checkout's cone stay absent and `skip-worktree`; the present `tools/run.sh` in those repositories stays `0644`.
-   **A tracked script replaced by a symlink** in the working tree is flagged (the index still says `100644`); the current build does not touch the symlink's target, which the change must keep.
-   `Fix::Chmod` in `src/cmd/doctor.rs` already carries each flagged path's bytes and `skip_worktree`; `apply` gets the repository root, so the on-disk step can work from those.

### Assumptions

-   `NONE`.

### Open questions

-   `NONE`.

### Deferred verification

-   `c3-os-agnostic` on Windows: no Windows machine or Windows Rust target is available here, and pushing to CI is outside this Task's authority; the Windows branch is checked by inspection (Claim's scope), and CI on Windows is not observed.

### UNDERSTAND gate

`ESTABLISHED`: p1 and every part of c1 are observed on the start build, and the code path the change touches is identified.

## Investigate

One probe settled the only design-relevant uncertainty, whether making the file executable after the index fix is enough (`logs/probe-chmod-after-fix-20261002-1400.log`, umask `0022`):

-   After the start build's `--fix` and a manual `chmod a+x`, the files are `755`, `git status --porcelain` shows `M ` (the mode change staged by `--fix`, nothing unstaged), `git diff` is empty, `git add` keeps both entries `100755`, and the next commit runs the hook.
-   `git` needs no index refresh or other extra step after the `chmod`.

The probe also showed that `acceptance.py`'s status check was wrong: it expected `git status` to show nothing, but `--fix` stages the mode change by design, so the right expectation is `M ` (staged, nothing unstaged).
The check now compares the whole porcelain line with `M  <path>`; the rerun on the start build gives the same 33 PASS, 13 FAIL (`logs/acceptance-start-98feea1-20261002-1401.log`).

### INVESTIGATE gate

`ESTABLISHED`: no decision-relevant uncertainty remains; the remaining choices are design choices for `DECIDE`.

## Decide

-   **Mechanism.** After `Fix::Chmod`'s index update and skip-worktree restore, `apply` calls a new `make_executable_on_disk(root, paths)` for the same entries.
    On Unix (`#[cfg(unix)]`) it builds each file's path from its stored bytes (`OsStr::from_bytes`), reads it with `symlink_metadata`, skips it if that fails or it is not a regular file, and otherwise adds an execute bit for each class (user, group, other) that has the read bit, the way `0644` becomes `0755`, writing the mode only when it changes.
    On other platforms (`#[cfg(not(unix))]`) it does nothing: Windows has no executable bit and git keeps the mode in the index only.
-   **What stays untouched:** content (only the mode is set); missing files (deleted, or outside a sparse cone) are skipped, never created; a symlink in place of a script is skipped, so its target is never changed; unmerged paths are already not flagged.
-   **Skip-worktree entries whose file happens to exist** are made executable too, since `c2-fixed` asks for every fixed script that exists on disk.
-   **Errors:** a failure to set the mode on an existing regular file is returned as `cannot make <path> executable: <error>`, like the other fixes' write errors; the index change made just before it stays.
-   **Spec delta** (published at completion), appended to `exec-bit`: "On systems with an executable bit, `--fix` SHALL also make each of those files that exists in the working tree as a regular file executable, without changing its content, and SHALL NOT create a missing file or change the target of a symbolic link; on Windows it SHALL NOT change the working tree."
-   **Rejected:** `chmod` before the index update, which would leave files executable if the index update fails; following symlinks (`fs::metadata`), which would make an unrelated target executable; reading the process umask, which needs `libc` for no observable gain over mirroring the read bits.
-   **Verification strategy.** Tests first in `tests/doctor.rs`: one cross-platform test that `--fix` leaves no unstaged change for fixed scripts (it exercises the Windows branch on Windows CI), and `#[cfg(unix)]` tests for the on-disk bit, `git add` keeping `100755`, the hook running, an unstaged edit, and a symlink.
    They run against the source at `98feea1` (extracted with `git archive`) and must fail there except the symlink guard; then `cargo test --no-fail-fast`, clippy, and `acceptance.py` on the old and new builds.
    `c3-os-agnostic` by inspection of both branches; Windows CI is not observable here.

### DECIDE gate

`ESTABLISHED`: the design is probed (`logs/probe-chmod-after-fix-20261002-1400.log`), stays inside `exec-bit`'s fix, and each success Claim has a planned check.

## Implement

-   Tests first, appended to `tests/doctor.rs`: `doctor_fix_leaves_no_unstaged_change_for_fixed_scripts` (all platforms; on Windows it exercises the no-op branch), and, `#[cfg(unix)]` with a one-line reason each, `doctor_fix_makes_fixed_scripts_executable_on_disk` (also `git add` keeps `100755`, and a non-script stays non-executable), `doctor_fix_lets_git_run_the_fixed_hook`, `doctor_fix_makes_an_edited_script_executable_and_keeps_the_edit_unstaged` and `doctor_fix_does_not_change_the_target_of_a_symlink_in_place_of_a_script`; plus a `disk_mode` helper.
-   Against the source at `98feea1` (`git archive` into a scratch directory, this test file copied in), the first four fail and the symlink guard passes, as planned: 40 passed, 4 failed (`logs/tests-on-start-98feea1-20261002-1402.log`).
-   `src/cmd/doctor.rs`: `apply`'s `Fix::Chmod` arm now ends with `make_executable_on_disk(root, paths)`, with a `#[cfg(unix)]` body as in Decide and a `#[cfg(not(unix))]` body that returns `Ok(())`.
-   Committed as `7ff434a` after the checks under Verify; the committed tree is the tree those checks ran on (`git status` was clean apart from the new logs).

## Verify

<a id="verification-p1-git-ignores-non-executable-hook"></a>
### Verification: `p1-git-ignores-non-executable-hook`

- Claim: [p1-git-ignores-non-executable-hook](#p1-git-ignores-non-executable-hook)
- Method: `acceptance.py`'s p1 section: git alone, with `core.hooksPath=.githooks`, a committed `pre-commit` hook at mode `0644` on disk that writes a marker file, then the same hook at `0755` as a control.
- Evidence considered: `logs/acceptance-start-98feea1-20261002-1359.log`: the commit exits `0`, no marker is written, and git prints `hint: The '.githooks/pre-commit' hook was ignored because it's not set as executable.`; with `0755` the marker is written.
- Conclusion: `VERIFIED`.
- Limitations: Linux, git 2.56.0; `advice.ignoredHook` at its default.

<a id="verification-c1-reproduced"></a>
### Verification: `c1-reproduced`

- Claim: [c1-reproduced](#c1-reproduced)
- Method: `acceptance.py` on the build of `98feea1`, `core.filemode=true`.
- Evidence considered: `logs/acceptance-start-98feea1-20261002-1359.log` and, with the corrected status check, `logs/acceptance-start-98feea1-20261002-1401.log`, section "plain repository": each of the three fixed scripts is `100755` in the index and `0o644` on disk, `git status --porcelain` shows `MM` for each, `git add` leaves `tools/run.sh` and `.githooks/pre-commit` at `100644`, and the following `git commit` prints the ignored-hook hint for both hooks and writes no marker.
- Conclusion: `VERIFIED`.
- Limitations: Linux, git 2.56.0.

<a id="verification-c2-fixed"></a>
### Verification: `c2-fixed`

- Claim: [c2-fixed](#c2-fixed)
- Method: `acceptance.py` on the build of `7ff434a` and, in the same minute, on the build of `98feea1`; the new integration tests.
- Evidence considered: `logs/acceptance-fixed-20261002-1403.log`: 46 PASS, 0 FAIL. In the plain repository each fixed script is `100755` in the index and executable on disk, `git status --porcelain` shows only `M  <path>` (the staged mode change, nothing unstaged), content is unchanged, `README.md` stays `100644` and non-executable, `git add` keeps `100755`, and the next commit runs the fixed `pre-commit` hook with no ignored-hook hint. A deleted hook stays absent; hooks outside a sparse cone stay absent and `skip-worktree`; an unstaged edit stays byte for byte on disk and unstaged, with no mode change in `git diff` and the committed content still staged; a symlink in place of a script is left a symlink and its target stays `0644`. `logs/acceptance-old-20261002-1403.log`: the start build fails 13 of the same 46 checks, all on-disk, status, `git add`, hook or mode-diff checks. `logs/test-final-20261002-1402.log`: the four Unix tests and the cross-platform test for this behavior pass.
- Conclusion: `VERIFIED`.
- Limitations: Linux, git 2.56.0, umask `0022`; one user, so a file the user cannot `chmod` was not exercised.

<a id="verification-c3-os-agnostic"></a>
### Verification: `c3-os-agnostic`

- Claim: [c3-os-agnostic](#c3-os-agnostic)
- Method: inspection of the change; a search for the installed targets that could compile the Windows branch here.
- Evidence considered: `logs/inspection-windows-branch-20261002-1404.log`: `make_executable_on_disk` has a `#[cfg(unix)]` body and a `#[cfg(not(unix))]` body that only returns `Ok(())`, so on Windows `--fix` runs the same index update as before and nothing more; the one test that runs on Windows, `doctor_fix_leaves_no_unstaged_change_for_fixed_scripts`, exercises that branch there; every Unix-only test carries a one-line reason. Only `x86_64-unknown-linux-gnu` is installed, so the Windows branch was not compiled here, and CI runs only after a push, which this sub-agent may not do.
- Conclusion: `UNVERIFIED`. The inspection part of the Scope supports the Claim; the CI part is not observed.
- Limitations: inferred, not observed: that the Windows branch compiles without warnings under clippy and that the cross-platform test passes on Windows (it relies on git setting `core.filemode=false` there).

<a id="verification-c4-tests"></a>
### Verification: `c4-tests`

- Claim: [c4-tests](#c4-tests)
- Method: the new tests against the source at `98feea1`; `cargo test --no-fail-fast` and clippy on the tree committed as `7ff434a`.
- Evidence considered: `logs/tests-on-start-98feea1-20261002-1402.log`: 40 passed, 4 failed; the failures are the four tests of the new behavior (no unstaged change, executable on disk, hook runs, edited script), and the symlink guard passes there, as planned in Decide. `logs/test-final-20261002-1402.log`: every test binary reports `ok`, 240 passed, 0 failed in total, the 44 in `tests/doctor.rs` among them. `logs/clippy-20261002-1402.log`: no warnings. Later commits in this Task change only `addf/`.
- Conclusion: `VERIFIED`.
- Limitations: Linux only.

<a id="verification-c6-stops-for-decisions"></a>
### Verification: `c6-stops-for-decisions`

- Claim: [c6-stops-for-decisions](#c6-stops-for-decisions)
- Method: comparison of the Success criteria, Specification impact and premise between `98feea1` and this checkpoint, ignoring State and Basis lines; the record of where work stopped.
- Evidence considered: `logs/criteria-unchanged-20261002-1405.log`: no Claim, Scope or Consequence line changed; the Specification impact is unchanged. The spec wording chosen in Decide only adds the on-disk part to `exec-bit`. When `c3-os-agnostic` turned out to need a push to observe, the sub-agent stopped here instead of re-scoping the Claim or completing without it.
- Conclusion: `VERIFIED` for the sub-agent's part of this Task, which ends at this checkpoint.
- Limitations: covers only the work up to this checkpoint.

### Stopped here

The sub-agent stopped in `VERIFY` at this checkpoint: `c3-os-agnostic`'s Scope names Windows CI, which runs only after a push, and pushing is outside its authority.
`VERIFY`'s gate is `NOT_SATISFIED` until the operator settles `c3-os-agnostic`, for example by pushing and reading the Windows CI result, or by narrowing its Scope.
After that, LEARN and the terminal checkpoint remain, including publishing the delta under Specification impact as worded in Decide.

### Checkpoint on 2026-10-02

Draft PR [#5](https://github.com/flatrick/git-it-right/pull/5) is open at `7926f97`. GitHub reports no checks or runs because workflow `ci` is `disabled_manually`. The operator was asked whether to re-enable it and for the manual Windows test result. Local tests, clippy, capsule tests and the archive check pass at this revision (`logs/verification-checkpoint-20261002-1235Z.log`). None of these observations establishes `c3-os-agnostic`; its State and the VERIFY gate remain unchanged.

### Windows test report and reassessment

The operator reported that `cargo test` on a Windows checkout failed only `doctor_compares_names_as_stored_for_case_collisions`, `doctor_flags_control_characters_and_non_utf8_names_in_git_quoted_form`, `doctor_reports_exec_bit_for_hook_names_git_quotes`, and `doctor_reports_windows_unsafe_names_as_stored`. The failure output is unavailable until the Windows computer is accessible again. All four use `add_index_entries` to insert paths Windows cannot hold. The older `doctor_reports_windows_unsafe_index_names_without_renaming` test, which the operator says passed with the rest, disables `core.protectNTFS` for its index insertion; the shared helper does not. This is a strong test-fixture hypothesis, not a verified Windows diagnosis.

Reassessment: the test implementation needs a focused correction while the on-disk fix Decision remains justified. Return to IMPLEMENT to align the helper with the existing test and expose Git stderr. The rerun on Windows will distinguish a fixture failure from a doctor output failure; until then `c3-os-agnostic` and the VERIFY gate remain unsettled.

## Learn

### Technical

`PENDING`

### Process

`PENDING`

## Retention and promotion

`PENDING`

## Archive readiness

`PENDING`

## Terminal record

### Summary

`PENDING`

### Gate basis

`PENDING`
