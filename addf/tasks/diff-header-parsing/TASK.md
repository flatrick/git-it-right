# TASK — `gir fixup reads ---/+++ only as file headers`

## Resume

**Contract version:** `2`

**State:** `DEFINE`

**State path:** `DEFINE`

**Resume at:** Confirm at the current `HEAD` that `p1-dash-dash-reproduces` still holds, then read `parse_hunks` and its callers in `src/cmd/fixup.rs`.

**Open obligations:** Every success criterion below is `UNVERIFIED` and blocks `VERIFY`; `p1-dash-dash-reproduces` blocks `UNDERSTAND`.

## Owned artifacts

-   `ledger.md` - the thread entries this Task took, and the questions that shaped it with the operator's answers.

## Specification impact

- Current contract: `framework:spec/fixup.md#req-fixup-staged-line-target`
- Proposed delta: `NONE`. The requirement already says gir selects the commit that last changed the staged lines; this Task brings the implementation in line with it.
- Terminal publication: `PENDING`

## Define

### Objective

`gir fixup` reads `---` and `+++` lines only as file headers, never as hunk content.

### Success criteria

<a id="dh-dash-dash-deletion"></a>
#### `dh-dash-dash-deletion`

-   Claim: When a staged change deletes a line whose content starts with `-- `, including `-- /dev/null`, `gir fixup` without a commit argument creates its `fixup!` for the commit that last changed that line, and later hunks in the same file are still traced.
-   State: `UNVERIFIED`
-   Scope: Linux, this branch.
-   Consequence if false: a fixup goes to the wrong commit, or is refused, for Lua, SQL or Haskell comments.
-   Basis: pending check.

<a id="dh-plus-plus-addition"></a>
#### `dh-plus-plus-addition`

-   Claim: When a staged change adds a line whose content starts with `++ b/`, `gir fixup` does not report it as a new file.
-   State: `UNVERIFIED`
-   Scope: Linux, this branch.
-   Consequence if false: an ordinary addition is refused as a new file.
-   Basis: pending check.

<a id="dh-gates-green"></a>
#### `dh-gates-green`

-   Claim: Tests cover both cases above, and `cargo test` and `cargo clippy --all-targets -- -D warnings` pass on the final revision.
-   State: `UNVERIFIED`
-   Scope: Linux, final revision of this branch.
-   Consequence if false: the fix regresses unnoticed.
-   Basis: pending check.

### Constraints

-   Work in worktree `.worktrees/fixup-modes` on branch `feat/fixup-modes` (operator's answer in `ledger.md`).
-   Overlaps `split-preserves-index` in `src/cmd/fixup.rs`. Order: this Task publishes and terminalizes first.
-   Only `parse_hunks`' header detection changes; the other findings in the thread are out of scope.

### Material empirical premises

<a id="p1-dash-dash-reproduces"></a>
#### `p1-dash-dash-reproduces`

-   Claim: At this Task's starting revision, deleting `-- header` from a committed file and editing a later line makes `gir fixup` report `cannot tell which commit header:5 belongs to`.
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
