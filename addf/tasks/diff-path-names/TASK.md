# TASK — `gir fixup traces files whatever their names`

## Resume

**Contract version:** `2`

**State:** `IMPLEMENT`

**State path:** `DEFINE -> UNDERSTAND -> INVESTIGATE -> DECIDE -> IMPLEMENT`

**Resume at:** Write the planned tests, see them fail, then add the header-path decoder and switch `trace` to `--name-status -z`.

**Open obligations:** Every success criterion below is `UNVERIFIED` and blocks `VERIFY`; `p1-space-name-fails` blocks `UNDERSTAND`.

## Owned artifacts

-   `ledger.md` - the thread entries this Task took, and the questions that shaped it with the operator's answers.
-   `probe_names.py` - Probe: how git writes six kinds of file name in diff headers and `--name-status`, and what `gir fixup --dry-run` does with each; names Windows cannot hold are skipped there.
-   `logs/probe-head-8a2a60c-20261002-1145.log` - Evidence: that probe at the Task's start.

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
-   State: `VERIFIED`
-   Scope: Linux, git 2.56.0, `8a2a60c`.
-   Consequence if false: the defect needs re-establishing.
-   Basis: [Verification](#verification-p1-space-name-fails).

### DEFINE gate

`ESTABLISHED`: on 2026-10-02 the operator explicitly agreed that the objective, success criteria and order are right, as written here (`ledger.md`).

## Understand

### Relevant context

-   The probe (`logs/probe-head-8a2a60c-20261002-1145.log`) shows how git 2.56.0 writes names with `core.quotePath=false`. A name with a space ends its `---`/`+++` line with a tab. A name with a double quote, a backslash or a tab is C-quoted (`"a/say \"hi\".txt"`, `"a/back\\slash.txt"`, `"a/tab\there.txt"`), with a trailing tab when it also has a space. `café.txt` is written as is and already works.
-   `parse_hunks` (`src/cmd/fixup.rs`) keeps the trailing tab and only strips the outer quotes, without decoding escapes. Its new-file check matches only an unquoted `+++ b/`.
-   `trace` then compares the parsed paths with `git diff --cached --name-status`, which quotes the same names. Even correct header parsing would still fail that comparison.
-   `--split` reuses each hunk's header text unchanged, and `git apply` reads quoted headers itself, so the patch needs no change.
-   Outside this Task: `src/cmd/doctor.rs` also reads names from `git ls-files` output without `-z`.

### Assumptions

-   A trailing tab on a `---`/`+++` line is always git's terminator: a name that itself contains a tab is quoted, so its tab is written as `\t`. Source: the probe's `tab\there.txt` case.

### Open questions

-   `NONE`.

### Deferred verification

-   `NONE`.

### UNDERSTAND gate

`ESTABLISHED`: the defect reproduces, and both places that mis-read names are identified.

## Investigate

No further probe is needed: the probe already shows git's exact output for every kind of name in scope.

### INVESTIGATE gate

`ESTABLISHED`.

## Decide

-   **Header paths.** A new function in `src/cmd/fixup.rs` turns the text after `--- ` or `+++ ` into a path. It drops one trailing tab, decodes a C-quoted name (`\"`, `\\`, `\t`, `\n` and the other single-letter escapes, and `\ooo` octal bytes), strips the `a/` or `b/` prefix, and gives no path for `/dev/null`. `parse_hunks` uses it for both lines, so the new-file check also handles quoted names.
-   **Cross-check.** `trace` reads `git diff --cached --name-status -z`, which is NUL-separated and never quoted, instead of parsing quoted lines.
-   **Rejected:** decoding the quoted `--name-status` lines too, which would be a second decoder for output that `-z` makes unnecessary; asking git for the paths some other way, which would not change how `--split` reuses the headers.
-   **OS-agnostic rule.** Names with a double quote, a backslash or a tab cannot exist on Windows, so tests for them run only on Unix (`#[cfg(unix)]`) and the reason is stated where they are. Space-in-name tests run everywhere. The decoder's unit tests run everywhere, since they need no files.
-   **Verification strategy.** Tests first, and see them fail: decoder unit tests; integration tests for a space and for each quoted kind, through `gir fixup` and `gir fixup --split` and `git rebase --autosquash`. Then rerun `probe_names.py`, `cargo test` and clippy.
-   Out of scope, noted for the operator: `src/cmd/doctor.rs` reads `git ls-files` output without `-z`.

### DECIDE gate

`ESTABLISHED`: the design is small and each success Claim has a planned test.

## Implement

`PENDING`

## Verify

<a id="verification-p1-space-name-fails"></a>
### Verification: `p1-space-name-fails`

- Claim: [p1-space-name-fails](#p1-space-name-fails)
- Method: `probe_names.py` with the build at `8a2a60c`.
- Evidence considered: `logs/probe-head-8a2a60c-20261002-1145.log`: `'my file.txt'` gives `gir: cannot tell which commit my file.txt belongs to`, exit `2`; the three quoted names fail the same way.
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
