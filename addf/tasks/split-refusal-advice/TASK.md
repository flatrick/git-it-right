# TASK — `--split asks or refuses with advice that works`

## Resume

**Contract version:** `2`

**State:** `INVESTIGATE`

**State path:** `DEFINE -> UNDERSTAND -> INVESTIGATE`

**Resume at:** Record that no probe is needed, then transition to `DECIDE`.

**Open obligations:** Every success criterion below is `UNVERIFIED` and blocks `VERIFY`; `p1-advice-fails` blocks `UNDERSTAND`.

## Owned artifacts

-   `ledger.md` - the thread entries this Task took, and the questions that shaped it with the operator's answers.
-   `acceptance.py` - Probe and acceptance: each criterion's cases with and without a terminal (`GIR_INTERACTIVE`), through `git rebase --autosquash` where a split happens.
-   `logs/acceptance-head-f46e828-20261002-1205.log` - Evidence: `acceptance.py` at the Task's start.

## Specification impact

- Current contract: `framework:spec/fixup.md#req-fixup-split`, `framework:spec/fixup.md#req-fixup-split-refusals`, `framework:spec/fixup.md#req-fixup-ask-several`, `framework:spec/fixup.md#req-fixup-ask-branch-commit`, `framework:spec/fixup.md#req-fixup-insertion-target`
- Proposed delta: `split-refusals`: without a terminal, an untraceable file is refused with advice to commit or unstage it and run `--split` again, and an insertion between two commits' lines is refused naming both commits, without `git add -p` advice. New requirements: at a terminal, `--split` asks which commit each untraceable file and each such insertion belongs to, and splits accordingly; `ask-several` offers `s` only when the split can go ahead. Exact wording settled in `DECIDE`.
- Terminal publication: `PENDING`

## Define

### Objective

`gir fixup --split` never gives advice that cannot work: at a terminal it asks what it cannot decide itself; without one it refuses with advice that works.

### Success criteria

<a id="c1-untraced-no-terminal"></a>
#### `c1-untraced-no-terminal`

-   Claim: Without a terminal, `gir fixup --split` with a staged file it cannot trace (a new file, a binary file, or a mode-only change) exits `2`, commits nothing, and its message names the file and says to commit or unstage it and run `--split` again, without `pass one: gir fixup <commit>`.
-   State: `UNVERIFIED`
-   Scope: Linux, git 2.56.0, this branch.
-   Consequence if false: the user is told to do something `--split` refuses.
-   Basis: pending check.

<a id="c2-untraced-terminal"></a>
#### `c2-untraced-terminal`

-   Claim: At a terminal, `gir fixup --split`, and the several-targets picker's `s`, ask only which commit each untraceable file belongs to; that file's whole staged version goes into the chosen commit's autosquash commit, and every other hunk is split as before.
-   State: `UNVERIFIED`
-   Scope: Linux, git 2.56.0, this branch.
-   Consequence if false: `--split` is silently ignored and everything lands in one commit.
-   Basis: pending check.

<a id="c3-ambiguous-insertion"></a>
#### `c3-ambiguous-insertion`

-   Claim: For a staged insertion between lines last changed by two different commits, at a terminal gir asks which of the two it belongs to and then splits; without a terminal it refuses, naming the hunk and both commits, without `git add -p` advice, and commits nothing.
-   State: `UNVERIFIED`
-   Scope: Linux, git 2.56.0, this branch.
-   Consequence if false: the split is blocked by advice that cannot split one line.
-   Basis: pending check.

<a id="c4-picker-offers-split"></a>
#### `c4-picker-offers-split`

-   Claim: The several-targets picker offers `s` only when the split can go ahead.
-   State: `UNVERIFIED`
-   Scope: Linux, git 2.56.0, this branch.
-   Consequence if false: the user picks `s` and gets a refusal.
-   Basis: pending check.

<a id="c5-end-to-end"></a>
#### `c5-end-to-end`

-   Claim: For `c2-untraced-terminal` and `c3-ambiguous-insertion`, the split followed by `git rebase --autosquash` gives the intended history; the build at this Task's start fails `c1-untraced-no-terminal` to `c4-picker-offers-split`.
-   State: `UNVERIFIED`
-   Scope: Linux, git 2.56.0, this branch.
-   Consequence if false: the questions look right but the folded history is wrong.
-   Basis: pending check.

<a id="c6-spec-and-tests"></a>
#### `c6-spec-and-tests`

-   Claim: The specification delta is published at completion; integration tests cover `c1-untraced-no-terminal` to `c4-picker-offers-split` and fail on the code at this Task's start; `cargo test` and `cargo clippy --all-targets -- -D warnings` pass on the final revision.
-   State: `UNVERIFIED`
-   Scope: Linux, final revision of this branch.
-   Consequence if false: the behavior is unspecified or regresses unnoticed.
-   Basis: pending check.

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

`PENDING`

## Implement

`PENDING`

## Verify

<a id="verification-p1-advice-fails"></a>
### Verification: `p1-advice-fails`

- Claim: [p1-advice-fails](#p1-advice-fails)
- Method: `acceptance.py` with the build at `f46e828`.
- Evidence considered: `logs/acceptance-head-f46e828-20261002-1205.log`: every `c1` case ends with `pass one: gir fixup <commit>`; the terminal `c2` new-file case creates one `fixup!` for everything; both `c3` cases end with `split it with git add -p`.
- Conclusion: `VERIFIED`.
- Limitations: Linux, git 2.56.0.

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
