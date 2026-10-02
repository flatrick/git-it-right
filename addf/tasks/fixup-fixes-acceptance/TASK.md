# TASK — `acceptance of the fixup header and split-index fixes`

## Resume

**Contract version:** `2`

**State:** `DECIDE`

**State path:** `DEFINE -> UNDERSTAND -> INVESTIGATE -> DECIDE`

**Resume at:** Record the acceptance plan under Decide, then transition to `IMPLEMENT`.

**Open obligations:** Every success criterion below is `UNVERIFIED` and blocks `VERIFY`.

## Owned artifacts

-   `ledger.md` - the questions that shaped this Task, with the operator's answers.
-   `review-scripts/` - the review's `edge.py`, `edge2.py` and `edge3.py`, copied unchanged except that `edge.py` reads the gir binary from `GIR_BIN` when set.
-   `run_review_scripts.py` - runs those scripts against two builds, normalises commit IDs and temporary paths, and diffs.
-   `logs/edge-old.log`, `logs/edge2-old.log`, `logs/edge3-old.log`, `logs/edge-new.log`, `logs/edge2-new.log`, `logs/edge3-new.log`, `logs/review-scripts-diff.log` - Evidence: the review scripts on `d246ab2` and on `HEAD`, normalised, and their diff.

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

`PENDING`

## Implement

`PENDING`

## Verify

<a id="verification-p1-old-build-fails"></a>
### Verification: `p1-old-build-fails`

- Claim: [p1-old-build-fails](#p1-old-build-fails)
- Method: `run_review_scripts.py` with the binary built from `d246ab2`.
- Evidence considered: `logs/edge-old.log` cases 1 and 1b (`cannot tell which commit header:5 belongs to`) and the intent-to-add case (`status after: '?? ita'`); `logs/edge2-old.log` cases 15 (` D out/c`) and 16 (` M cfg`).
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
