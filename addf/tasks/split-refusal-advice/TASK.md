# TASK — `--split asks or refuses with advice that works`

## Resume

**Contract version:** `2`

**State:** `LEARN`

**State path:** `DEFINE -> UNDERSTAND -> INVESTIGATE -> DECIDE -> IMPLEMENT -> VERIFY -> LEARN`

**Resume at:** Record Learn, Retention and promotion and Archive readiness, then publish the delta and complete.

**Open obligations:** Publish the specification delta in the terminal checkpoint (blocks `COMPLETED`).

## Owned artifacts

-   `ledger.md` - the thread entries this Task took, and the questions that shaped it with the operator's answers.
-   `acceptance.py` - Probe and acceptance: each criterion's cases with and without a terminal (`GIR_INTERACTIVE`), through `git rebase --autosquash` where a split happens.
-   `logs/acceptance-fixed-20261002-1209.log` - Evidence: the first acceptance run on the change, with one wrong check (see Implement).
-   `logs/acceptance-fixed-20261002-120919.log` - Evidence: the run that exposed the empty-patch failure (see Implement).
-   `logs/acceptance-fixed-x5-20261002-1209.log` - Evidence: `acceptance.py` five times on the final change (the tree committed as `45174c6`).
-   `logs/test-final-20261002-1209.log` - Evidence: `cargo test` before the empty-patch fix.
-   `logs/test-final-20261002-1209-run1.log`, `logs/test-final-20261002-1209-run2.log`, `logs/test-final-20261002-1209-run3.log` - Evidence: `cargo test --no-fail-fast` three times on the tree committed as `45174c6`.
-   `logs/clippy-20261002-1209.log` - Evidence: `cargo clippy --all-targets -- -D warnings` on that tree.
-   `logs/tests-on-start-6dcb42e-20261002-1210.log` - Evidence: the new tests against the source before the change (`6dcb42e`).
-   `logs/regression-sweep-20261002-1209.log` - Evidence: the earlier acceptance scripts and the review scripts on this build.
-   `logs/acceptance-head-f46e828-20261002-1205.log` - Evidence: `acceptance.py` at the Task's start.

## Specification impact

- Current contract: `framework:spec/fixup.md#req-fixup-split`, `framework:spec/fixup.md#req-fixup-split-refusals`, `framework:spec/fixup.md#req-fixup-ask-several`, `framework:spec/fixup.md#req-fixup-ask-branch-commit`, `framework:spec/fixup.md#req-fixup-insertion-target`
- Proposed delta: `split-refusals`, `ask-several` and `split-flag-hint` reworded, and a new `split-ask`, as written under Decide.
- Terminal publication: `PENDING`

## Define

### Objective

`gir fixup --split` never gives advice that cannot work: at a terminal it asks what it cannot decide itself; without one it refuses with advice that works.

### Success criteria

<a id="c1-untraced-no-terminal"></a>
#### `c1-untraced-no-terminal`

-   Claim: Without a terminal, `gir fixup --split` with a staged file it cannot trace (a new file, a binary file, or a mode-only change) exits `2`, commits nothing, and its message names the file and says to commit or unstage it and run `--split` again, without `pass one: gir fixup <commit>`.
-   State: `VERIFIED`
-   Scope: Linux, git 2.56.0, this branch.
-   Consequence if false: the user is told to do something `--split` refuses.
-   Basis: [Verification](#verification-c1-untraced-no-terminal).

<a id="c2-untraced-terminal"></a>
#### `c2-untraced-terminal`

-   Claim: At a terminal, `gir fixup --split`, and the several-targets picker's `s`, ask only which commit each untraceable file belongs to; that file's whole staged version goes into the chosen commit's autosquash commit, and every other hunk is split as before.
-   State: `VERIFIED`
-   Scope: Linux, git 2.56.0, this branch.
-   Consequence if false: `--split` is silently ignored and everything lands in one commit.
-   Basis: [Verification](#verification-c2-untraced-terminal).

<a id="c3-ambiguous-insertion"></a>
#### `c3-ambiguous-insertion`

-   Claim: For a staged insertion between lines last changed by two different commits, at a terminal gir asks which of the two it belongs to and then splits; without a terminal it refuses, naming the hunk and both commits, without `git add -p` advice, and commits nothing.
-   State: `VERIFIED`
-   Scope: Linux, git 2.56.0, this branch.
-   Consequence if false: the split is blocked by advice that cannot split one line.
-   Basis: [Verification](#verification-c3-ambiguous-insertion).

<a id="c4-picker-offers-split"></a>
#### `c4-picker-offers-split`

-   Claim: The several-targets picker offers `s` only when the split can go ahead.
-   State: `VERIFIED`
-   Scope: Linux, git 2.56.0, this branch.
-   Consequence if false: the user picks `s` and gets a refusal.
-   Basis: [Verification](#verification-c4-picker-offers-split).

<a id="c5-end-to-end"></a>
#### `c5-end-to-end`

-   Claim: For `c2-untraced-terminal` and `c3-ambiguous-insertion`, the split followed by `git rebase --autosquash` gives the intended history; the build at this Task's start fails `c1-untraced-no-terminal` to `c4-picker-offers-split`.
-   State: `VERIFIED`
-   Scope: Linux, git 2.56.0, this branch.
-   Consequence if false: the questions look right but the folded history is wrong.
-   Basis: [Verification](#verification-c5-end-to-end).

<a id="c6-spec-and-tests"></a>
#### `c6-spec-and-tests`

-   Claim: The specification delta is published at completion; integration tests cover `c1-untraced-no-terminal` to `c4-picker-offers-split` and fail on the code at this Task's start; `cargo test` and `cargo clippy --all-targets -- -D warnings` pass on the final revision.
-   State: `VERIFIED`
-   Scope: Linux, final revision of this branch.
-   Consequence if false: the behavior is unspecified or regresses unnoticed.
-   Basis: [Verification](#verification-c6-spec-and-tests).

### Constraints

-   Work in worktree `.worktrees/fixup-modes` on branch `feat/fixup-modes` (operator's answer in `ledger.md`).
-   Code and test scripts follow `rules/os-agnostic-code.md`.
-   Plain `gir fixup` without `--split` keeps its current refusals and questions, apart from the picker's `s` (`c4-picker-offers-split`).
-   No other Task is active.

### Material empirical premises

<a id="p1-advice-fails"></a>
#### `p1-advice-fails`

-   Claim: At this Task's starting revision, `gir fixup --split` without a terminal and with a new file staged advises `pass one: gir fixup <commit>`; with `GIR_INTERACTIVE=1` it puts everything into one commit; and an insertion between two commits' lines is refused with `split it with git add -p`.
-   State: `VERIFIED`
-   Scope: Linux, git 2.56.0, `f46e828`.
-   Consequence if false: the review's reproductions do not hold here.
-   Basis: [Verification](#verification-p1-advice-fails).

### DEFINE gate

`ESTABLISHED`: on 2026-10-02 the operator explicitly agreed that the objective, success criteria and order are right, as written here (`ledger.md`).

## Understand

### Relevant context

-   At `f46e828` (`logs/acceptance-head-f46e828-20261002-1205.log`): without a terminal, `--split` with a new, binary or mode-only file ends with `pass one: gir fixup <commit>`; at a terminal it asks for one commit and puts everything there; an insertion between two commits' lines is refused with `split it with git add -p`, with or without a terminal; the picker offers `s` even when a hunk changes lines of two commits and the split would refuse.
-   `trace` returns `Trace::Unattributed` at the first file it cannot trace, so `run` never reaches `split` with such a file, and `--split` is lost. `parse_hunks` returns an error for the first new file; the `--name-status -z` cross-check finds the rest (binary files and mode-only changes have no `---`/`+++` lines). A hunk whose lines `git blame` cannot name also ends the trace.
-   `split` refuses any hunk with several commits before committing, insertion or not. `split::commit_each` rebuilds each round from patches only, so a whole-file change has no way in.
-   `pick::choose` reads each answer from the shared stdin buffer, so several questions in one run read consecutive lines.

### Assumptions

-   `git update-index --cacheinfo <mode> <object> <path>` in the temporary index places a file's staged version exactly, including mode-only changes and binary content; source: git documentation; checked by `acceptance.py` after implementation.

### Open questions

-   `NONE`.

### Deferred verification

-   `NONE`.

### UNDERSTAND gate

`ESTABLISHED`: every failure reproduces, and the code paths that cause them are known.

## Investigate

No probe is needed: `acceptance.py` reproduces every case, and the one assumption (`update-index --cacheinfo`) is checked by the same script once implemented.

### INVESTIGATE gate

`ESTABLISHED`.

## Decide

-   **Trace result.** `trace` returns every hunk it can trace and every file it cannot (new, binary, mode-only, or a hunk `git blame` cannot name), each with the reason already used today, instead of stopping at the first such file. `parse_hunks` no longer errors on a new file; the `--name-status -z` cross-check reports it with the same message. Plain `gir fixup` keeps its behavior: it uses the first untraced file's reason, new files first, exactly as before.
-   **Split order of checks.** Before any question or commit: a hunk that changes lines of several commits is refused as today. Then, without a terminal, the first untraced file is refused with `gir: <reason>; --split cannot place it: commit it on its own or unstage it (git restore --staged -- <path>), then run gir <subcommand> --split again`, and an insertion between two commits' lines with `gir: <path>:<line> is an insertion between lines of <sha> <subject> and <sha> <subject>; run gir <subcommand> --split in a terminal to choose, or stage it on its own and run gir <subcommand> <commit>`; both exit `2`.
-   **Questions.** At a terminal, gir asks for each untraced file `gir: <reason>; pick the commit it belongs to:` with the branch's newest commits (as `ask-branch-commit` does), then for each such insertion `gir: <path>:<line> is an insertion between lines of two commits; pick the one it belongs to:` with those two commits. A cancel exits `1` with nothing committed.
-   **Whole-file placement.** In each round of the temporary index, after the patch, every untraced file assigned to a done target gets its staged entry from the staged tree (`git update-index --cacheinfo <mode> <object> <path>`), or is removed when the staged tree has none. `TempIndex` gains `run_raw` for byte paths.
-   **When split can go ahead.** No hunk changes lines of several commits (insertions can be asked about). Only then does the picker offer `s`, and only then does the several-targets refusal print `or: gir <subcommand> --split ...`. The second follows from the objective, though no criterion names it.
-   **Spec delta**, published at completion:
    -   `split-refusals`: the hunk refusal applies to a hunk that changes lines last changed by several commits; without a terminal, the two refusals above.
    -   new `split-ask`: the two questions above, where the answers go, and that a cancel exits `1` with no commit.
    -   `ask-several` and `split-flag-hint`: `s` and the `--split` hint only when the split can go ahead.
-   **Rejected:** splitting what can be traced and leaving the rest staged (operator chose to refuse); attributing an insertion to the line above (operator chose to ask).
-   **Verification strategy.** Integration tests first for each `acceptance.py` case, and see them fail; then the change; `acceptance.py` on the new build; the new tests against the unfixed source; the earlier acceptance scripts; `cargo test` and clippy.

### DECIDE gate

`ESTABLISHED`: every operator decision is reflected, and each success Claim has a planned test.

## Implement

-   `45174c6` implements Decide in `src/cmd/fixup.rs` (`Trace` with `hunks` and `untraced`, `trace(base, all)`, `parse_hunks` without the new-file error, the picker's `s` and the `--split` hint only when `split::can_split`), `src/cmd/fixup/split.rs` (`can_split`, the refusals, the two questions, `neighbours`, `place_whole_file`), and `src/git.rs` (`TempIndex::run_raw`).
-   Deviation from Decide: the insertion question lists the two commits in file order, the line above's first, each labelled `(line above)` or `(line below)`, instead of by commit ID; the commit-ID order is effectively random, which made the question harder to answer and the tests unable to pick a commit.
-   Two unit tests that expected `parse_hunks` to fail on a new file now assert that it returns no hunks, and are renamed `new_files_have_no_lines_to_trace` and `new_files_with_quoted_names_have_no_lines_to_trace`; the new-file message now comes from `trace`, unchanged in wording.
-   Found during implementation: a target that receives only whole files has an empty patch, and `git apply` rejects empty input. Whether it showed depended on commit-ID order, so the binary-file case passed once and failed the next run (`logs/acceptance-fixed-20261002-120919.log`). Fixed by skipping `git apply` for an empty patch; `split_places_a_file_in_a_commit_with_no_hunks_of_its_own` reproduces it every time on the intermediate code. That test passes on the source before this Task too, where a new file makes everything one commit, so it guards the new code rather than separating old from new.
-   `acceptance.py` corrections: the insertion answers became `2` (the line below) after the order change, and one check expected three `fixup!` commits where two is correct.
-   New integration tests in `tests/fixup_modes.rs`: `split_without_a_terminal_refuses_an_untraceable_file_with_split_advice`, `split_at_a_terminal_asks_where_an_untraceable_file_goes`, `split_at_a_terminal_places_a_binary_file_whole`, `split_without_a_terminal_refuses_an_ambiguous_insertion_naming_both_commits`, `split_at_a_terminal_asks_where_an_ambiguous_insertion_goes` (through `--split` and through the picker's `s`), `split_is_not_offered_when_a_hunk_changes_lines_of_several_commits`, `split_places_a_file_in_a_commit_with_no_hunks_of_its_own`. The first six fail on the source before the change (`logs/tests-on-start-6dcb42e-20261002-1210.log`).

## Verify

<a id="verification-p1-advice-fails"></a>
### Verification: `p1-advice-fails`

- Claim: [p1-advice-fails](#p1-advice-fails)
- Method: `acceptance.py` with the build at `f46e828`.
- Evidence considered: `logs/acceptance-head-f46e828-20261002-1205.log`: every `c1` case ends with `pass one: gir fixup <commit>`; the terminal `c2` new-file case creates one `fixup!` for everything; both `c3` cases end with `split it with git add -p`.
- Conclusion: `VERIFIED`.
- Limitations: Linux, git 2.56.0.

<a id="verification-c1-untraced-no-terminal"></a>
### Verification: `c1-untraced-no-terminal`

- Claim: [c1-untraced-no-terminal](#c1-untraced-no-terminal)
- Method: `acceptance.py` (five runs) and the integration tests (three runs), on the build of `45174c6` and on the source before the change.
- Evidence considered: `acceptance.py` `c1` cases for a new file, a binary file and a mode-only change pass in all five runs (`logs/acceptance-fixed-x5-20261002-1209.log`), and failed at the start (`logs/acceptance-head-f46e828-20261002-1205.log`); `split_without_a_terminal_refuses_an_untraceable_file_with_split_advice` passes (`logs/test-final-20261002-1209-run1.log`) and fails before the change (`logs/tests-on-start-6dcb42e-20261002-1210.log`).
- Conclusion: `VERIFIED`.
- Limitations: Linux, git 2.56.0.

<a id="verification-c2-untraced-terminal"></a>
### Verification: `c2-untraced-terminal`

- Claim: [c2-untraced-terminal](#c2-untraced-terminal)
- Method: `acceptance.py` (five runs) and the integration tests (three runs), on the build of `45174c6` and on the source before the change.
- Evidence considered: `acceptance.py` `c2` cases pass in all five runs, through `git rebase --autosquash` (`logs/acceptance-fixed-x5-20261002-1209.log`); `split_at_a_terminal_asks_where_an_untraceable_file_goes`, `split_at_a_terminal_places_a_binary_file_whole` and `split_places_a_file_in_a_commit_with_no_hunks_of_its_own` pass three times (`logs/test-final-20261002-1209-run1.log`, `logs/test-final-20261002-1209-run2.log`, `logs/test-final-20261002-1209-run3.log`); the first two fail before the change (`logs/tests-on-start-6dcb42e-20261002-1210.log`).
- Conclusion: `VERIFIED`.
- Limitations: Linux, git 2.56.0. Through the picker's `s` an untraceable file cannot occur: plain `gir fixup` asks for one commit first, as before.

<a id="verification-c3-ambiguous-insertion"></a>
### Verification: `c3-ambiguous-insertion`

- Claim: [c3-ambiguous-insertion](#c3-ambiguous-insertion)
- Method: `acceptance.py` (five runs) and the integration tests (three runs), on the build of `45174c6` and on the source before the change.
- Evidence considered: `acceptance.py` `c3` cases pass in all five runs (`logs/acceptance-fixed-x5-20261002-1209.log`); `split_without_a_terminal_refuses_an_ambiguous_insertion_naming_both_commits` and `split_at_a_terminal_asks_where_an_ambiguous_insertion_goes` pass (`logs/test-final-20261002-1209-run1.log`) and fail before the change (`logs/tests-on-start-6dcb42e-20261002-1210.log`); the review script's case 4 now names both commits (`logs/regression-sweep-20261002-1209.log`).
- Conclusion: `VERIFIED`.
- Limitations: Linux, git 2.56.0. Choosing the line above's commit can make the later `rebase --autosquash` conflict, because both changes then touch the same place; that is git's behavior for that history, and the tests choose the line below.

<a id="verification-c4-picker-offers-split"></a>
### Verification: `c4-picker-offers-split`

- Claim: [c4-picker-offers-split](#c4-picker-offers-split)
- Method: `acceptance.py` (five runs) and the integration tests (three runs), on the build of `45174c6` and on the source before the change.
- Evidence considered: `acceptance.py` `c4` passes (`logs/acceptance-fixed-x5-20261002-1209.log`); `split_is_not_offered_when_a_hunk_changes_lines_of_several_commits` passes (`logs/test-final-20261002-1209-run1.log`) and fails before the change (`logs/tests-on-start-6dcb42e-20261002-1210.log`); it also checks that the no-terminal refusal leaves out the `--split` hint.
- Conclusion: `VERIFIED`.
- Limitations: Linux, git 2.56.0.

<a id="verification-c5-end-to-end"></a>
### Verification: `c5-end-to-end`

- Claim: [c5-end-to-end](#c5-end-to-end)
- Method: `acceptance.py` (five runs) and the integration tests (three runs), on the build of `45174c6` and on the source before the change.
- Evidence considered: Every terminal case in `acceptance.py` and the integration tests runs `git rebase -i --autosquash` and checks the history and file contents: all pass (`logs/acceptance-fixed-x5-20261002-1209.log`, `logs/test-final-20261002-1209-run1.log`); at the start, `acceptance.py` failed 24 checks across `c1` to `c4` (`logs/acceptance-head-f46e828-20261002-1205.log`). The earlier acceptance scripts still pass, 53/53 and 48/48 (`logs/regression-sweep-20261002-1209.log`).
- Conclusion: `VERIFIED`.
- Limitations: Linux, git 2.56.0.

<a id="verification-c6-spec-and-tests"></a>
### Verification: `c6-spec-and-tests`

- Claim: [c6-spec-and-tests](#c6-spec-and-tests)
- Method: `acceptance.py` (five runs) and the integration tests (three runs), on the build of `45174c6` and on the source before the change.
- Evidence considered: The new tests fail before the change (`logs/tests-on-start-6dcb42e-20261002-1210.log`) and pass three times on `45174c6` (`logs/test-final-20261002-1209-run1.log`, `logs/test-final-20261002-1209-run2.log`, `logs/test-final-20261002-1209-run3.log`: 223 passed, 0 failed each); clippy reports no warnings (`logs/clippy-20261002-1209.log`). The specification delta recorded under Decide is published to `spec/fixup.md` in the terminal checkpoint, the same commit that marks this Task `COMPLETED`.
- Conclusion: `VERIFIED`.
- Limitations: Linux only here. Publication and this Verification share one checkpoint, as Stewardship's terminal checkpoint requires the delta to be applied there.

VERIFY gate: `ESTABLISHED`; every success Claim is `VERIFIED`, `c6-spec-and-tests` with its publication in the terminal checkpoint.

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
