# TASK — `gir fixup --split leaves untouched index entries alone`

## Resume

**Contract version:** `2`

**State:** `UNDERSTAND`

**State path:** `DEFINE -> UNDERSTAND`

**Resume at:** Confirm `p1-split-drops-flags` at the current `HEAD`, then read `src/cmd/fixup/split.rs`; this Task's `VERIFY` waits for `diff-header-parsing` to terminalize.

**Open obligations:** Every success criterion below is `UNVERIFIED` and blocks `VERIFY`; `p1-split-drops-flags` blocks `UNDERSTAND`; re-reading the spec after `diff-header-parsing` terminalizes blocks `VERIFY`.

## Owned artifacts

-   `ledger.md` - the thread entries this Task took, and the questions that shaped it with the operator's answers.

## Specification impact

- Current contract: `framework:spec/fixup.md#req-fixup-split`, `framework:spec/fixup.md#req-fixup-split-rollback`
- Proposed delta: split and split-rollback also leave the `skip-worktree` and intent-to-add state of every index entry as it was before the command, and change no entry the split does not commit.
- Terminal publication: `PENDING`

## Define

### Objective

`gir fixup --split` leaves alone every index entry it does not commit.

### Success criteria

<a id="sp-entries-kept"></a>
#### `sp-entries-kept`

-   Claim: After a successful `gir fixup --split`, and after a failed one that restores, `skip-worktree` bits, intent-to-add entries and the sparse-checkout view are as they were before the command, and `git status` differs from before only by the effect of the new commits.
-   State: `UNVERIFIED`
-   Scope: Linux, this branch.
-   Consequence if false: a later `git add -A` or `git commit -a` deletes sparse files, commits private edits, or loses intent-to-add entries.
-   Basis: pending check.

<a id="sp-tests"></a>
#### `sp-tests`

-   Claim: Integration tests cover a sparse checkout, a `skip-worktree` file with a local edit, and a `git add -N` file, each for a successful split and for a restored failure.
-   State: `UNVERIFIED`
-   Scope: this branch.
-   Consequence if false: the fix regresses unnoticed.
-   Basis: pending check.

<a id="sp-gates-green"></a>
#### `sp-gates-green`

-   Claim: `cargo test` and `cargo clippy --all-targets -- -D warnings` pass on the final revision.
-   State: `UNVERIFIED`
-   Scope: Linux, final revision of this branch.
-   Consequence if false: regressions ship.
-   Basis: pending check.

### Constraints

-   Work in worktree `.worktrees/fixup-modes` on branch `feat/fixup-modes` (operator's answer in `ledger.md`).
-   Overlaps `diff-header-parsing` in `src/cmd/fixup.rs`. Order: `diff-header-parsing` publishes and terminalizes first; this Task re-reads the current specification after that, before its own `VERIFY`.
-   Not promised: the stat-cache cost (Q8). If the fix also cures it, record that under Verify.

### Material empirical premises

<a id="p1-split-drops-flags"></a>
#### `p1-split-drops-flags`

-   Claim: At this Task's starting revision, after a successful `gir fixup --split` in a sparse checkout, `git status` shows the files outside the sparse set as deleted.
-   State: `UNVERIFIED`
-   Scope: Linux, this branch at the Task's first commit.
-   Consequence if false: the review's reproduction does not hold here, and the defect needs re-establishing.
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
