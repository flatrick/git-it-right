# TASK — `gir fixup traces files whatever their names`

## Resume

**Contract version:** `2`

**State:** `UNDERSTAND`

**State path:** `DEFINE -> UNDERSTAND`

**Resume at:** Confirm `p1-space-name-fails` at the current `HEAD`, then read how `parse_hunks` takes the path from `---`/`+++` lines.

**Open obligations:** Every success criterion below is `UNVERIFIED` and blocks `VERIFY`; `p1-space-name-fails` blocks `UNDERSTAND`.

## Owned artifacts

-   `ledger.md` - the thread entries this Task took, and the questions that shaped it with the operator's answers.

## Specification impact

- Current contract: `framework:spec/fixup.md#req-fixup-staged-line-target`, `framework:spec/fixup.md#req-fixup-split`
- Proposed delta: `NONE`. The requirements already apply to every staged line; this Task makes the implementation meet them for these file names.
- Terminal publication: `PENDING`

## Define

### Objective

`gir fixup` traces staged changes to files whatever their names, as git writes them in diff headers.

### Success criteria

<a id="c1-space-in-name"></a>
#### `c1-space-in-name`

-   Claim: For a file whose name contains a space (git ends its `---`/`+++` header line with a tab), `gir fixup` without a commit argument selects the commit that last changed the staged line.
-   State: `UNVERIFIED`
-   Scope: Linux, git 2.56.0, this branch.
-   Consequence if false: files with spaces in their names cannot be fixed up automatically.
-   Basis: pending check.

<a id="c2-quoted-name"></a>
#### `c2-quoted-name`

-   Claim: For a file whose name git quotes in diff headers (one containing a double quote, a backslash or a tab), `gir fixup` selects the right commit, and `gir fixup --split` on such a file creates one commit per target with the right content.
-   State: `UNVERIFIED`
-   Scope: Linux, git 2.56.0, this branch.
-   Consequence if false: such files cannot be fixed up or split.
-   Basis: pending check.

<a id="c3-end-to-end"></a>
#### `c3-end-to-end`

-   Claim: For both kinds of name, `gir fixup` and `gir fixup --split` followed by `git rebase --autosquash` give the intended history, and the build at the start of this Task fails the same checks.
-   State: `UNVERIFIED`
-   Scope: Linux, git 2.56.0, this branch.
-   Consequence if false: the fix looks right in `--dry-run` but the folded history is wrong.
-   Basis: pending check.

<a id="c4-regression-tests"></a>
#### `c4-regression-tests`

-   Claim: Integration tests cover `c1-space-in-name` and `c2-quoted-name`, they fail on the code at this Task's start, and `cargo test` and `cargo clippy --all-targets -- -D warnings` pass on the final revision.
-   State: `UNVERIFIED`
-   Scope: Linux, final revision of this branch.
-   Consequence if false: the fix regresses unnoticed.
-   Basis: pending check.

### Constraints

-   Work in worktree `.worktrees/fixup-modes` on branch `feat/fixup-modes` (operator's answer in `ledger.md`).
-   Code and test scripts follow `rules/os-agnostic-code.md`.
-   Out of scope: non-UTF-8 file names and the bytes of `--split` patches (thread Q3, next Task).
-   No other Task is active.

### Material empirical premises

<a id="p1-space-name-fails"></a>
#### `p1-space-name-fails`

-   Claim: At this Task's starting revision, `gir fixup` on a staged change to `my file.txt` reports `cannot tell which commit my file.txt belongs to`.
-   State: `UNVERIFIED`
-   Scope: Linux, git 2.56.0, this branch at the Task's first commit.
-   Consequence if false: the defect needs re-establishing.
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
