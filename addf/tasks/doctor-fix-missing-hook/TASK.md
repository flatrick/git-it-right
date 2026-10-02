# TASK — `gir doctor --fix sets the exec bit even without the file`

## Resume

**Contract version:** `2`

**State:** `INVESTIGATE`

**State path:** `DEFINE -> UNDERSTAND -> INVESTIGATE`

**Resume at:** Record that the probes settled every uncertainty, then transition to `DECIDE`.

**Open obligations:** Every success criterion below is `UNVERIFIED` and blocks `VERIFY`.

## Owned artifacts

-   `ledger.md` - the thread entries this Task took, and the questions that shaped it with the operator's answers.
-   `acceptance.py` - Probe and acceptance: `gir doctor --fix` with a deleted hook, hooks outside a sparse checkout's cone, and a hook with an unstaged local edit.
-   `logs/acceptance-head-3ab3c09-20261002-1339.log` - Evidence: `acceptance.py` at the Task's start.
-   `logs/probe-cacheinfo-20261002-1339.log` - Evidence: `git update-index --cacheinfo` against `--chmod=+x` for a missing hook file.
-   `logs/probe-cacheinfo-skipworktree-20261002-1339.log` - Evidence: `--cacheinfo` drops `skip-worktree`, and `--skip-worktree` restores it.

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

-   Claim: On the build at this Task's start, `gir doctor --fix` exits with a fatal error when a tracked `100644` hook was deleted from the working tree, and stages an unstaged local edit to a hook it fixes. (Narrowed by the operator in `UNDERSTAND`: the sparse-checkout case did not reproduce.)
-   State: `VERIFIED`
-   Scope: Linux, git 2.56.0, `3ab3c09`.
-   Consequence if false: the fix targets a defect that is not there.
-   Basis: [Verification](#verification-c1-reproduced).

<a id="c2-fixed"></a>
#### `c2-fixed`

-   Claim: After the change, `gir doctor --fix` sets those index entries to `100755` without needing the file, leaves the working tree and the staged content unchanged (an unstaged edit stays unstaged), keeps hooks outside a sparse checkout's cone `skip-worktree`, and finishes normally; hooks present in the working tree are still fixed.
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

`PENDING`

## Decide

`PENDING`

## Implement

`PENDING`

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
