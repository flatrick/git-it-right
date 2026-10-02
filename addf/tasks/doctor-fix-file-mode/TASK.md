# TASK — `gir doctor --fix makes fixed scripts executable on disk`

## Resume

**Contract version:** `2`

**State:** `VERIFY`

**State path:** `DEFINE -> UNDERSTAND -> INVESTIGATE -> DECIDE -> IMPLEMENT -> VERIFY -> IMPLEMENT -> VERIFY -> IMPLEMENT -> VERIFY -> IMPLEMENT -> VERIFY`

**Resume at:** Push the fixture correction to PR #5 and inspect the hosted macOS, Windows, and Ubuntu jobs.

**Open obligations:** Correct the macOS non-UTF-8 test fixture and obtain a green hosted run (blocks `VERIFY`); `c5-spec` is published in the terminal checkpoint.

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
-   `logs/windows-fixture-local-20261002-1255Z.log` - Evidence: the reported Windows failures, the focused fixture change, and local checks.
-   `logs/windows-hooks-local-20261002-1311Z.log` - Evidence: the reported Windows hook-test failure, the Git-driven test change, and local checks.
-   `logs/windows-manual-report-20261002-1320Z.log` - Evidence: the operator's report of passing Windows tests and `acceptance.py` at `c017481`; no raw output was archived.
-   `logs/ci-first-run-20261002.log` - Evidence: hosted CI run at `b8175a8`, including passing Windows jobs and the macOS fixture failure.

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
-   State: `VERIFIED`
-   Scope: this branch; the Windows branch by inspection and CI.
-   Consequence if false: the change breaks or misbehaves on Windows.
-   Basis: [Verification](#verification-c3-os-agnostic).

<a id="c4-tests"></a>
#### `c4-tests`

-   Claim: Regression tests fail on the code at this Task's start and pass after; `cargo test` and `cargo clippy --all-targets -- -D warnings` pass on the final revision.
-   State: `VERIFIED`
-   Scope: Linux, final revision of this branch.
-   Consequence if false: the fix regresses unnoticed.
-   Basis: [Verification](#verification-c4-tests).

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
-   `public-ci-safeguards` now runs on the same branch and must finish before this Task's hosted Windows CI verification. It has no product specification delta; this Task publishes the doctor delta after its own verification.

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
-   After the operator reported four Windows test failures, `add_index_entries` now passes `core.protectNTFS=false` to `git update-index`, matching the older test that inserts Windows-unsafe names. It also captures Git stderr so another failure names the rejected path or other cause. Local tests and clippy pass; Windows remains unobserved (`logs/windows-fixture-local-20261002-1255Z.log`).
-   After the operator reported two hook-test failures on Windows, `missing_gir_script` now invokes the installed `commit-msg` and `pre-push` shims through Git, with `gir` removed from PATH. This removes the test's direct dependency on a `sh` command in PATH. The two affected tests, the full Linux suite, and clippy pass (`logs/windows-hooks-local-20261002-1311Z.log`).

### IMPLEMENT gate after hook-test reassessment

`ESTABLISHED`: the hook tests' direct shell launch was replaced in `ba625c5`, and the installed hooks can be evaluated through Git on Windows.

### IMPLEMENT gate after reassessment

`ESTABLISHED`: the focused test-fixture change is committed as `3e69631`, and the Windows rerun can distinguish a remaining Git insertion failure from a doctor report failure.

### IMPLEMENT gate after macOS fixture reassessment

`58bea3b` replaces the non-UTF-8 working-tree filename in `doctor_fix_sets_exec_bit_on_a_non_utf8_hook_name` with an index-only entry. The test still checks that `gir doctor --fix` changes its index mode to `100755`. The focused test, `cargo test --no-fail-fast`, and `cargo clippy --all-targets -- -D warnings` pass on Linux. The capsule checker, including the archive, passes after correcting the archived safeguard Task's terminal obligation marker at `8fbddcd`.

`ESTABLISHED`: the intended fixture correction exists and passes local checks. Hosted macOS CI is needed to verify it on the failing system.

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
- Method: inspect both platform branches; consider the operator's manual Windows test and acceptance report; inspect hosted Windows CI.
- Evidence considered: `logs/inspection-windows-branch-20261002-1404.log` shows the Windows body of `make_executable_on_disk` is a no-op. `logs/windows-manual-report-20261002-1320Z.log` records the operator's report of passing Windows tests and `acceptance.py` with 32 PASS, 0 FAIL at `c017481`. `logs/ci-first-run-20261002.log` records hosted `test (windows-latest)` and `capsule (windows-latest)` passing at `b8175a8`; the test job includes clippy and the full Rust suite.
- Conclusion: `VERIFIED` for the stated Windows inspection and CI scope.
- Limitations: the manual run's raw output was not archived; the hosted run tests this branch at `b8175a8`.

<a id="verification-c4-tests"></a>
### Verification: `c4-tests`

- Claim: [c4-tests](#c4-tests)
- Method: the new tests against the source at `98feea1`; `cargo test --no-fail-fast` and clippy on the tree committed as `7ff434a`.
- Evidence considered: `logs/tests-on-start-98feea1-20261002-1402.log`: 40 passed, 4 failed; the failures are the four tests of the new behavior (no unstaged change, executable on disk, hook runs, edited script), and the symlink guard passes there, as planned in Decide. `logs/test-final-20261002-1402.log`: every test binary reports `ok`, 240 passed, 0 failed in total, the 44 in `tests/doctor.rs` among them. `logs/clippy-20261002-1402.log`: no warnings. After the test-fixture change at `3e69631`, `logs/windows-fixture-local-20261002-1255Z.log` records 240 passing Linux tests and clean clippy; after the hook-test change at `ba625c5`, `logs/windows-hooks-local-20261002-1311Z.log` records 7 passing hook tests, 240 passing tests total, and clean clippy on Linux. The Rust source of `gir` is unchanged.
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

### Verification after fixture correction

`3e69631` changes only the four failing tests' shared index fixture and its failure output. Linux tests and clippy pass (`logs/windows-fixture-local-20261002-1255Z.log`). The Windows tests have not been rerun, so their result and `c3-os-agnostic` remain unverified. The VERIFY gate is `NOT_SATISFIED`.

### Windows hook test report and reassessment

The operator also reported that `installed_hooks_warn_when_gir_is_missing` and `installed_hooks_block_when_gir_is_missing_and_configured_to_fail` fail on Windows at `tests/hooks.rs:29`, where `missing_gir_script` calls `repo.cmd("sh").output().unwrap()`: Windows returns `NotFound` before either hook runs. The test helper depends on a shell executable being on PATH, although installed hooks are invoked by Git in normal use. Return to IMPLEMENT to use Git as the test driver; the existing hook behavior expectations remain unchanged. The Windows rerun is still required.

### Verification after hook-test correction

`ba625c5` changes the two affected tests to drive the installed hooks through Git. All seven hook tests, the full Linux suite, and clippy pass (`logs/windows-hooks-local-20261002-1311Z.log`). At this checkpoint the Windows tests had not been rerun, and the VERIFY gate remained `NOT_SATISFIED`.

### Manual Windows report

The operator reports that all tests now pass on Windows and that `acceptance.py` ends with 32 PASS, 0 FAIL at `c017481` (`logs/windows-manual-report-20261002-1320Z.log`). This settles the requested manual rerun as reported. Hosted Windows CI remains absent, so `c3-os-agnostic` stays `UNVERIFIED` and the VERIFY gate stays `NOT_SATISFIED`.

The operator subsequently made securing the public repository against untrusted workflow runs and protecting `main` against force pushes or deletion prerequisites to enabling Actions. Hosted CI remains blocked on those safeguards; this Task stays in VERIFY.

### Hosted CI and macOS fixture reassessment

The prerequisite `public-ci-safeguards` Task completed and Actions ran PR #5 at `b8175a8` (`logs/ci-first-run-20261002.log`). Both Windows and both Ubuntu jobs passed. The macOS test job failed only `doctor_fix_sets_exec_bit_on_a_non_utf8_hook_name`: `commit_files_named` calls `std::fs::write` on a path with byte `0xe9`, and macOS returns `Illegal byte sequence` at `tests/doctor.rs:519`. Gir does not run in that test before the failure. The separate index-only non-UTF-8 doctor test passed on macOS.

The test should place the non-UTF-8 name directly in Git's index, as the passing test does, and verify that `doctor --fix` changes its index mode without needing a working-tree file. This preserves the test's intent and avoids a macOS-invalid fixture. Return to `IMPLEMENT` for that correction. `VERIFY` is `NOT_SATISFIED` until the hosted run is green, despite `c3-os-agnostic` now being `VERIFIED`.

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
