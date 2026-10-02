# TASK — `gir doctor --fix makes fixed scripts executable on disk`

## Resume

**Contract version:** `2`

**State:** `UNDERSTAND`

**State path:** `DEFINE -> UNDERSTAND`

**Resume at:** Reproduce `c1-reproduced` and `p1-git-ignores-non-executable-hook` on the current build, checking each part rather than assuming it.

**Open obligations:** Every success criterion below is `UNVERIFIED` and blocks `VERIFY`.

## Owned artifacts

-   `ledger.md` - the thread entries this Task took, and the questions that shaped it with the operator's answers.

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
-   State: `UNVERIFIED`
-   Scope: Linux, git 2.56.0, this branch.
-   Consequence if false: the fix targets a defect that is not there, or misses part of it.
-   Basis: pending check.

<a id="c2-fixed"></a>
#### `c2-fixed`

-   Claim: After the change, on Unix: each fixed script that exists on disk is executable; `git status` shows no mode change for it; `git add` keeps `100755`; the hook runs. Files missing from the working tree (deleted, or outside a sparse checkout's cone) stay absent; file content and unstaged edits are untouched.
-   State: `UNVERIFIED`
-   Scope: Linux, git 2.56.0, this branch.
-   Consequence if false: fixed hooks still do not run, or `git add` undoes the fix.
-   Basis: pending check.

<a id="c3-os-agnostic"></a>
#### `c3-os-agnostic`

-   Claim: Windows has no executable bit: the on-disk step is Unix-only, written with both platform branches per `rules/os-agnostic-code.md`, and nothing changes on Windows.
-   State: `UNVERIFIED`
-   Scope: this branch; the Windows branch by inspection and CI.
-   Consequence if false: the change breaks or misbehaves on Windows.
-   Basis: pending check.

<a id="c4-tests"></a>
#### `c4-tests`

-   Claim: Regression tests fail on the code at this Task's start and pass after; `cargo test` and `cargo clippy --all-targets -- -D warnings` pass on the final revision.
-   State: `UNVERIFIED`
-   Scope: Linux, final revision of this branch.
-   Consequence if false: the fix regresses unnoticed.
-   Basis: pending check.

<a id="c5-spec"></a>
#### `c5-spec`

-   Claim: `spec/doctor.md#req-doctor-exec-bit` adds the on-disk part, and the delta is published at completion.
-   State: `UNVERIFIED`
-   Scope: this branch.
-   Consequence if false: the specification does not describe `--fix`.
-   Basis: pending check.

<a id="c6-stops-for-decisions"></a>
#### `c6-stops-for-decisions`

-   Claim: The sub-agent stops and reports instead of deciding anything that changes these criteria or the specification beyond `c5-spec`.
-   State: `UNVERIFIED`
-   Scope: this Task.
-   Consequence if false: a delegated decision is made without the operator.
-   Basis: pending check.

### Constraints

-   Work in worktree `.worktrees/fixup-modes` on branch `feat/fixup-modes` (operator's answer in `ledger.md`).
-   Code and tests follow `rules/os-agnostic-code.md`.
-   Delegated to a sub-agent from `UNDERSTAND` on; see `c6-stops-for-decisions`.
-   No other Task is active.

### Material empirical premises

<a id="p1-git-ignores-non-executable-hook"></a>
#### `p1-git-ignores-non-executable-hook`

-   Claim: With `core.hooksPath` pointing at `.githooks/`, git does not run a hook file that is not executable on disk, and says so.
-   State: `UNVERIFIED`
-   Scope: Linux, git 2.56.0.
-   Consequence if false: Q15's impact is only a dirty `git status`, not hooks failing to run.
-   Basis: pending check.

### DEFINE gate

`ESTABLISHED`: on 2026-10-02 the operator explicitly agreed that the objective, success criteria and order are right, as written here (`ledger.md`).

## Understand

### Relevant context

`PENDING`

### Assumptions

-   `NONE` yet.

### Open questions

-   `NONE` yet.

### Deferred verification

-   `NONE` yet.

## Investigate

`PENDING`

## Decide

`PENDING`

## Implement

`PENDING`

## Verify

`PENDING`

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
