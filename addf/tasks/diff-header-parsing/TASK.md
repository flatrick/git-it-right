# TASK — `gir fixup reads ---/+++ only as file headers`

## Resume

**Contract version:** `2`

**State:** `LEARN`

**State path:** `DEFINE -> UNDERSTAND -> INVESTIGATE -> DECIDE -> IMPLEMENT -> VERIFY -> LEARN`

**Resume at:** Record Learn, Retention and promotion and Archive readiness, then hand over to Stewardship for `COMPLETED`.

**Open obligations:** `NONE`

## Owned artifacts

-   `ledger.md` - the thread entries this Task took, and the questions that shaped it with the operator's answers.
-   `probe_dash_dash.py` - Probe: four staged changes whose hunk lines look like `---`/`+++` headers, run with `gir fixup --dry-run`.
-   `logs/probe-head-afd58e8-20261002-1015.log` - Evidence: the probe at `afd58e8`, before any change.
-   `logs/probe-fixed-20261002-1017.log` - Evidence: the probe with the fix (the tree committed as `06c5ee4`).
-   `logs/probe-split-20261002-1018.log` - Evidence: `--split` on probe case 2 with the fix.
-   `logs/test-final-20261002-1017.log` - Evidence: `cargo test` with the fix (the tree committed as `06c5ee4`).
-   `logs/clippy-20261002-1017.log` - Evidence: `cargo clippy --all-targets -- -D warnings` with the fix.

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
-   State: `VERIFIED`
-   Scope: Linux, this branch.
-   Consequence if false: a fixup goes to the wrong commit, or is refused, for Lua, SQL or Haskell comments.
-   Basis: [Verification](#verification-dh-dash-dash-deletion).

<a id="dh-plus-plus-addition"></a>
#### `dh-plus-plus-addition`

-   Claim: When a staged change adds a line whose content starts with `++ b/`, `gir fixup` does not report it as a new file.
-   State: `VERIFIED`
-   Scope: Linux, this branch.
-   Consequence if false: an ordinary addition is refused as a new file.
-   Basis: [Verification](#verification-dh-plus-plus-addition).

<a id="dh-gates-green"></a>
#### `dh-gates-green`

-   Claim: Tests cover both cases above, and `cargo test` and `cargo clippy --all-targets -- -D warnings` pass on the final revision.
-   State: `VERIFIED`
-   Scope: Linux, final revision of this branch.
-   Consequence if false: the fix regresses unnoticed.
-   Basis: [Verification](#verification-dh-gates-green).

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

No further probe is needed. Dispositions:

-   Whether the defect is real at `HEAD`: resolved by `p1-dash-dash-reproduces` and the other three probe cases.
-   Whether a real diff can reach `parse_hunks` without a `diff --git` line: irrelevant to the fix if the parser treats the start of its input as header, which also keeps the existing unit test valid.

### INVESTIGATE gate

`ESTABLISHED`: every decision-relevant uncertainty has a disposition.

## Decide

-   **Fix.** In `parse_hunks`, `in_header` starts `true`, and the `--- ` and `+++ b/` tests run only while `in_header` is true. Inside a hunk body every line is content, so `-- x` and `++ b/x` reach `changes` like any other line.
-   **Rejected:** a separate `in_body` flag, which would track the same state twice; changing the unit test to add a `diff --git` line while keeping `in_header` starting `false`, which would leave the parser misreading header-less input.
-   **Verification strategy.** Write the tests first and see them fail at the current code: unit tests in `src/cmd/fixup.rs` for a deleted `-- x` line, a deleted `-- /dev/null` line followed by another hunk, and an added `++ b/x` line after it; integration tests in `tests/fixup.rs` for probe cases 1, 2 and 4. Then rerun the probe, `cargo test` and `cargo clippy --all-targets -- -D warnings`.
-   Residual uncertainty: none known.

### DECIDE gate

`ESTABLISHED`: the change is three lines in one function, and each success Claim has a test planned.

## Implement

-   `06c5ee4` changes `parse_hunks` in `src/cmd/fixup.rs` as decided: `in_header` starts `true`, and the `--- ` and `+++ b/` tests run only while it is true.
-   Tests added in the same commit: the unit test `hunk_lines_that_look_like_file_headers_are_content`, and the integration tests `deleted_line_starting_with_dashes_is_traced_like_any_other`, `deleted_dev_null_comment_does_not_hide_later_hunks` and `added_line_starting_with_plus_b_is_not_a_new_file`.
-   Before the fix, all four new tests failed: the unit test traced the later hunks to path `other`; the integration tests got exit `2`, `0` and `2` where `0`, `2` and `0` were expected.
-   No deviation from Decide.
-   Not a success Claim, but checked because `--split` rebuilds patches from the same hunks: `--split` on probe case 2 creates one `fixup!` per commit with the right content (`logs/probe-split-20261002-1018.log`).

## Verify

<a id="verification-p1-dash-dash-reproduces"></a>
### Verification: `p1-dash-dash-reproduces`

- Claim: [p1-dash-dash-reproduces](#p1-dash-dash-reproduces)
- Method: built `target/debug/gir` at `afd58e8` and ran `probe_dash_dash.py`.
- Evidence considered: `logs/probe-head-afd58e8-20261002-1015.log`, case 1: `gir: cannot tell which commit header:5 belongs to; pass one: gir fixup <commit>`, exit `2`.
- Conclusion: `VERIFIED`.
- Limitations: Linux, git 2.56.0.

<a id="verification-dh-dash-dash-deletion"></a>
### Verification: `dh-dash-dash-deletion`

- Claim: [dh-dash-dash-deletion](#dh-dash-dash-deletion)
- Method: the unit and integration tests added in `06c5ee4`, and the probe rerun with the fix.
- Evidence considered: `logs/test-final-20261002-1017.log`: `hunk_lines_that_look_like_file_headers_are_content` (deleted `--- /dev/null` and `--- a/other` lines stay in `x.lua`, later hunks keep path `x.lua`), `deleted_line_starting_with_dashes_is_traced_like_any_other` and `deleted_dev_null_comment_does_not_hide_later_hunks` pass. `logs/probe-fixed-20261002-1017.log`: case 1 selects `feat: lua`; case 2 reports both commits A (`x.lua:1`) and B (`x.lua:5`). Before the fix (`logs/probe-head-afd58e8-20261002-1015.log`) case 1 was refused and case 2 silently selected A.
- Conclusion: `VERIFIED`.
- Limitations: Linux, git 2.56.0.

<a id="verification-dh-plus-plus-addition"></a>
### Verification: `dh-plus-plus-addition`

- Claim: [dh-plus-plus-addition](#dh-plus-plus-addition)
- Method: the unit test and `added_line_starting_with_plus_b_is_not_a_new_file` from `06c5ee4`, and probe cases 3 and 4.
- Evidence considered: `logs/test-final-20261002-1017.log` shows both tests passing; `logs/probe-fixed-20261002-1017.log` case 4 selects `feat: z`, where before the fix it reported `foo is a new file`. Case 3 already passed before the fix, so for an `++ b/` line on its own the Claim guards against a regression rather than fixing one.
- Conclusion: `VERIFIED`.
- Limitations: Linux, git 2.56.0.

<a id="verification-dh-gates-green"></a>
### Verification: `dh-gates-green`

- Claim: [dh-gates-green](#dh-gates-green)
- Method: `cargo test` and `cargo clippy --all-targets -- -D warnings` on the tree committed as `06c5ee4`.
- Evidence considered: `logs/test-final-20261002-1017.log` (every target `ok`, 201 tests, 0 failed); `logs/clippy-20261002-1017.log` (finished, no warnings).
- Conclusion: `VERIFIED`.
- Limitations: Linux only.

VERIFY gate: `ESTABLISHED`; every success Claim is `VERIFIED`.

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
