# TASK — `gir doctor --fix sets the exec bit even without the file`

## Resume

**Contract version:** `2`

**State:** `DEFINE`

**State path:** `DEFINE`

**Resume at:** Reproduce `c1-reproduced` on the current build, then prepare Codex's throwaway clone and check that cargo can build in its sandbox.

**Open obligations:** Every success criterion below is `UNVERIFIED` and blocks `VERIFY`.

## Owned artifacts

-   `ledger.md` - the thread entries this Task took, and the questions that shaped it with the operator's answers.

## Specification impact

- Current contract: `framework:spec/doctor.md#req-doctor-exec-bit`
- Proposed delta: `exec-bit`: `--fix` SHALL set the flagged index entries to mode `100755` without needing their files in the working tree, instead of naming the `git update-index --chmod=+x` command.
- Terminal publication: `PENDING`

## Define

### Objective

`gir doctor --fix` sets the executable bit on every flagged hook, even when its file is missing from the working tree, and never stops halfway.

### Success criteria

<a id="c1-reproduced"></a>
#### `c1-reproduced`

-   Claim: On the build at this Task's start, `gir doctor --fix` exits with a fatal error after applying earlier fixes when a tracked `100644` hook is missing from the working tree, both when deleted locally and when outside a sparse checkout's cone.
-   State: `UNVERIFIED`
-   Scope: Linux, git 2.56.0, this branch.
-   Consequence if false: the fix targets a defect that is not there.
-   Basis: pending check.

<a id="c2-fixed"></a>
#### `c2-fixed`

-   Claim: After the change, `gir doctor --fix` sets those index entries to `100755` without needing the file, leaves the working tree and file contents unchanged, and finishes normally; hooks present in the working tree are still fixed as before.
-   State: `UNVERIFIED`
-   Scope: Linux, git 2.56.0, this branch.
-   Consequence if false: `--fix` still leaves repositories half-changed.
-   Basis: pending check.

<a id="c3-tests"></a>
#### `c3-tests`

-   Claim: Regression tests fail on the code at this Task's start and pass after; `cargo test` and `cargo clippy --all-targets -- -D warnings` pass on the final revision; the tests follow `rules/os-agnostic-code.md`.
-   State: `UNVERIFIED`
-   Scope: Linux, final revision of this branch.
-   Consequence if false: the fix regresses unnoticed.
-   Basis: pending check.

<a id="c4-spec"></a>
#### `c4-spec`

-   Claim: `spec/doctor.md#req-doctor-exec-bit` states the outcome instead of the command, and the delta is published at completion.
-   State: `UNVERIFIED`
-   Scope: this branch.
-   Consequence if false: the specification describes a mechanism that no longer exists.
-   Basis: pending check.

<a id="c5-verified-here"></a>
#### `c5-verified-here`

-   Claim: The diff Codex produces is reviewed and verified by the supervisor in this worktree before it is committed; Codex's own report is not used as evidence.
-   State: `UNVERIFIED`
-   Scope: this Task.
-   Consequence if false: unverified delegated work lands on the branch.
-   Basis: pending check.

### Constraints

-   Work in worktree `.worktrees/fixup-modes` on branch `feat/fixup-modes` (operator's answer in `ledger.md`).
-   Codex runs only in a throwaway clone, with `-s workspace-write`; never in this worktree.
-   Code and tests follow `rules/os-agnostic-code.md`.
-   No other Task is active.

### Material empirical premises

<a id="p1-cacheinfo-works"></a>
#### `p1-cacheinfo-works`

-   Claim: `git update-index --cacheinfo 100755,<object>,<path>` sets an index entry's mode when the file is missing from the working tree, including outside a sparse checkout's cone.
-   State: `UNVERIFIED`
-   Scope: Linux, git 2.56.0.
-   Consequence if false: the suggested fix does not work and another is needed.
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
