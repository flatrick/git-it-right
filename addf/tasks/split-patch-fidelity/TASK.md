# TASK — `gir fixup --split commits exactly the staged bytes`

## Resume

**Contract version:** `2`

**State:** `UNDERSTAND`

**State path:** `DEFINE -> UNDERSTAND`

**Resume at:** Confirm `p1-split-loses-bytes` at the current `HEAD`, then read how the diff text reaches `parse_hunks` and `patch`.

**Open obligations:** Every success criterion below is `UNVERIFIED` and blocks `VERIFY`; `p1-split-loses-bytes` blocks `UNDERSTAND`.

## Owned artifacts

-   `ledger.md` - the thread entries this Task took, and the questions that shaped it with the operator's answers.

## Specification impact

- Current contract: `framework:spec/fixup.md#req-fixup-split`, `framework:spec/fixup.md#req-fixup-staged-line-target`
- Proposed delta: `NONE` expected: `split` already requires `HEAD`'s tree to equal the staged tree. To be confirmed in `DECIDE` if non-UTF-8 names need a new requirement.
- Terminal publication: `PENDING`

## Define

### Objective

`gir fixup --split` commits exactly the bytes that were staged, whatever the file's line endings, encoding, whitespace, name, or the user's apply and diff settings.

### Success criteria

<a id="c1-crlf"></a>
#### `c1-crlf`

-   Claim: A file with CRLF line endings (`* -text`) splits into the right commits, and the folded commits keep its CRLF bytes.
-   State: `UNVERIFIED`
-   Scope: Linux, git 2.56.0, this branch.
-   Consequence if false: Windows-style files cannot be split.
-   Basis: pending check.

<a id="c2-non-utf8-content"></a>
#### `c2-non-utf8-content`

-   Claim: A file whose content is not UTF-8 (Latin-1) splits into the right commits with its bytes unchanged.
-   State: `UNVERIFIED`
-   Scope: Linux, git 2.56.0, this branch.
-   Consequence if false: files in legacy encodings cannot be split, or are corrupted.
-   Basis: pending check.

<a id="c3-whitespace"></a>
#### `c3-whitespace`

-   Claim: A staged last line ending in spaces, and lines with trailing whitespace, split into the right commits with their bytes unchanged.
-   State: `UNVERIFIED`
-   Scope: Linux, git 2.56.0, this branch.
-   Consequence if false: the split refuses or changes whitespace.
-   Basis: pending check.

<a id="c4-settings"></a>
#### `c4-settings`

-   Claim: With `apply.whitespace=error`, with `apply.whitespace=fix`, and with a `textconv` driver configured for the file, `--split` succeeds and commits the staged bytes unchanged.
-   State: `UNVERIFIED`
-   Scope: Linux, git 2.56.0, this branch.
-   Consequence if false: a user setting blocks or silently alters the split.
-   Basis: pending check.

<a id="c5-end-to-end"></a>
#### `c5-end-to-end`

-   Claim: For `c1-crlf` to `c4-settings` and `c7-non-utf8-name`, `--split` followed by `git rebase --autosquash` leaves exactly the staged bytes in the right commits, and the build at this Task's start fails those checks.
-   State: `UNVERIFIED`
-   Scope: Linux, git 2.56.0, this branch.
-   Consequence if false: the fix looks right but the folded history differs from what was staged.
-   Basis: pending check.

<a id="c6-regression-tests"></a>
#### `c6-regression-tests`

-   Claim: Integration tests cover `c1-crlf` to `c4-settings` and `c7-non-utf8-name` and fail on the code at this Task's start; `cargo test` and `cargo clippy --all-targets -- -D warnings` pass on the final revision.
-   State: `UNVERIFIED`
-   Scope: Linux, final revision of this branch.
-   Consequence if false: the fix regresses unnoticed.
-   Basis: pending check.

<a id="c7-non-utf8-name"></a>
#### `c7-non-utf8-name`

-   Claim: A file whose name is not valid UTF-8 is traced by `gir fixup` and split by `gir fixup --split` into the right commits.
-   State: `UNVERIFIED`
-   Scope: Unix only, since Windows file names cannot hold such bytes; Linux, git 2.56.0, this branch.
-   Consequence if false: such files cannot be fixed up or split.
-   Basis: pending check.

### Constraints

-   Work in worktree `.worktrees/fixup-modes` on branch `feat/fixup-modes` (operator's answer in `ledger.md`).
-   Code and test scripts follow `rules/os-agnostic-code.md`; tests for non-UTF-8 names are Unix-only with the reason stated.
-   Out of scope: thread Q5 and Q6 (next Task).
-   No other Task is active.

### Material empirical premises

<a id="p1-split-loses-bytes"></a>
#### `p1-split-loses-bytes`

-   Claim: At this Task's starting revision, `gir fixup --split` fails for a CRLF file, a Latin-1 file, a staged last line ending in spaces, and with `apply.whitespace=error`.
-   State: `UNVERIFIED`
-   Scope: Linux, git 2.56.0, this branch at the Task's first commit.
-   Consequence if false: the review's reproductions do not hold here.
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
