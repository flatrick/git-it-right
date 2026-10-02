# TASK — `acceptance of the fixup header and split-index fixes`

## Resume

**Contract version:** `2`

**State:** `UNDERSTAND`

**State path:** `DEFINE -> UNDERSTAND`

**Resume at:** Build the old (`d246ab2`) and new binaries side by side, then confirm `p1-old-build-fails` with the review's scripts.

**Open obligations:** Every success criterion below is `UNVERIFIED` and blocks `VERIFY`.

## Owned artifacts

-   `ledger.md` - the thread entries this Task took, and the questions that shaped it with the operator's answers.

## Specification impact

- Current contract: `framework:spec/fixup.md#req-fixup-staged-line-target`, `framework:spec/fixup.md#req-fixup-split`, `framework:spec/fixup.md#req-fixup-split-rollback`
- Proposed delta: `NONE`. This Task tests behavior; it changes no requirement.
- Terminal publication: `PENDING`

## Define

### Objective

Show, comparing the reviewed build `d246ab2` with this branch's `HEAD`, that `06c5ee4` and `3d8a04b` fix the reported scenarios end to end and change nothing else the review exercised.

### Success criteria

<a id="a1-review-scripts"></a>
#### `a1-review-scripts`

-   Claim: With commit IDs and temporary paths normalised, the review's `edge.py`, `edge2.py` and `edge3.py` produce the same output on both builds except for the cases of findings #1 (index entries) and #2 (`-- ` lines), which on the new build behave as the fixes intend.
-   State: `UNVERIFIED`
-   Scope: Linux, git 2.56.0, old build `d246ab2`, new build this branch's `HEAD`.
-   Consequence if false: a fix changed behavior the review saw as correct, or did not fix a reported case.
-   Basis: pending check.

<a id="a2-header-end-to-end"></a>
#### `a2-header-end-to-end`

-   Claim: For the `-- ` deletion cases, `gir fixup` (or `--split`) followed by `git rebase --autosquash` leaves each original commit holding the intended lines on the new build, and not on the old one.
-   State: `UNVERIFIED`
-   Scope: Linux, git 2.56.0, old build `d246ab2`, new build this branch's `HEAD`.
-   Consequence if false: the fix looks right in `--dry-run` but the folded history is wrong.
-   Basis: pending check.

<a id="a3-split-end-to-end"></a>
#### `a3-split-end-to-end`

-   Claim: For a sparse checkout, a `skip-worktree` file with a local edit and a `git add -N` file, `gir fixup --split`, `git rebase --autosquash`, then `git add -A` and `git commit -a` delete no hidden file, commit no private edit and leave the intent-to-add file intent-to-add, on the new build; the old build fails this.
-   State: `UNVERIFIED`
-   Scope: Linux, git 2.56.0, old build `d246ab2`, new build this branch's `HEAD`.
-   Consequence if false: the harm the review described still happens after the fix.
-   Basis: pending check.

<a id="a4-setups"></a>
#### `a4-setups`

-   Claim: On the new build, a linked worktree, a sparse checkout in cone mode with a sparse index, running from a subdirectory, `gir amend --split`, `gir squash --split` and the interactive picker's `s` each keep index entries the split does not commit, and no `gir-split-index-*` file remains after a successful or a failed split.
-   State: `UNVERIFIED`
-   Scope: Linux, git 2.56.0, this branch's `HEAD`.
-   Consequence if false: the temporary index breaks or leaks in a setup the fix was not tested in.
-   Basis: pending check.

<a id="a5-regression-tests"></a>
#### `a5-regression-tests`

-   Claim: The regression-relevant cases of `a2-header-end-to-end` to `a4-setups` are cargo integration tests, and `cargo test` and `cargo clippy --all-targets -- -D warnings` pass on the final revision.
-   State: `UNVERIFIED`
-   Scope: Linux, final revision of this branch.
-   Consequence if false: the scenarios regress unnoticed.
-   Basis: pending check.

### Constraints

-   Work in worktree `.worktrees/fixup-modes` on branch `feat/fixup-modes` (operator's answer in `ledger.md`).
-   If any scenario fails on the new build: stop, record it, and report to the operator before any fix; a fix is a separate Task.
-   No active Task overlaps this one.

### Material empirical premises

<a id="p1-old-build-fails"></a>
#### `p1-old-build-fails`

-   Claim: On the old build `d246ab2`, the review's scripts still show the #1 and #2 failures.
-   State: `UNVERIFIED`
-   Scope: Linux, git 2.56.0.
-   Consequence if false: the before side of every comparison is unsound.
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
