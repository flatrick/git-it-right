# TASK — `gir fixup reads ---/+++ only as file headers`

## Resume

**Contract version:** `2`

**State:** `UNDERSTAND`

**State path:** `DEFINE -> UNDERSTAND`

**Resume at:** Confirm `p1-dash-dash-reproduces` at the current `HEAD`, then read `parse_hunks` and its callers in `src/cmd/fixup.rs`.

**Open obligations:** Every success criterion below is `UNVERIFIED` and blocks `VERIFY`; `p1-dash-dash-reproduces` blocks `UNDERSTAND`.

## Owned artifacts

-   `ledger.md` - the thread entries this Task took, and the questions that shaped it with the operator's answers.
-   `probe_dash_dash.py` - Probe: four staged changes whose hunk lines look like `---`/`+++` headers, run with `gir fixup --dry-run`.
-   `logs/probe-head-afd58e8-20261002-1015.log` - Evidence: the probe at `afd58e8`, before any change.

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
-   State: `VERIFIED`
-   Scope: Linux, git 2.56.0, `afd58e8`.
-   Consequence if false: the review's reproduction does not hold here, and the defect needs re-establishing.
-   Basis: [Verification](#verification-p1-dash-dash-reproduces).

### DEFINE gate

`ESTABLISHED`: on 2026-10-02 the operator explicitly agreed that the objective, success criteria and order are right, as written here (`ledger.md`).

## Understand

### Relevant context

-   `trace` in `src/cmd/fixup.rs` runs `git diff --cached -U0` and passes the text to `parse_hunks`; `split.rs` rebuilds patches from the `Hunk`s it returns.
-   `parse_hunks` tracks `in_header` (from `diff --git` to the first `@@`), but tests `--- ` and `+++ b/` on every line. In a hunk body, a deleted line `-- x` reads `--- x` and an added line `++ b/x` reads `+++ b/x`.
-   `--- x` sets the current path to `x`, or to none for `/dev/null`; hunks under no path are dropped. `+++ b/x` with no path is reported as a new file.
-   The probe at `afd58e8` (`logs/probe-head-afd58e8-20261002-1015.log`) shows three outcomes: case 1 refuses with `header:5`; case 2 silently picks commit A although the staged changes trace to A and B; case 4 calls an added line a new file. Case 3, `++ b/foo` alone, is already correct because a path is set.
-   The unit test `new_files_need_an_explicit_target` passes `parse_hunks` a diff without a `diff --git` line, so `in_header` is false throughout it.

### Assumptions

-   `git diff` output always starts each file with `diff --git`; source: git's diff format; not verified beyond the probe; if false, header lines before it would be read as body lines.

### Open questions

-   `NONE`.

### Deferred verification

-   `NONE`.

### UNDERSTAND gate

`ESTABLISHED`: the defect reproduces at `HEAD`, the code path is known, and the one test that depends on header detection is identified.

## Investigate

`PENDING`

## Decide

`PENDING`

## Implement

`PENDING`

## Verify

<a id="verification-p1-dash-dash-reproduces"></a>
### Verification: `p1-dash-dash-reproduces`

- Claim: [p1-dash-dash-reproduces](#p1-dash-dash-reproduces)
- Method: built `target/debug/gir` at `afd58e8` and ran `probe_dash_dash.py`.
- Evidence considered: `logs/probe-head-afd58e8-20261002-1015.log`, case 1: `gir: cannot tell which commit header:5 belongs to; pass one: gir fixup <commit>`, exit `2`.
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
