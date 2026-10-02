# TASK — `gir fixup traces files whatever their names`

## Resume

**Contract version:** `2`

**State:** `LEARN`

**State path:** `DEFINE -> UNDERSTAND -> INVESTIGATE -> DECIDE -> IMPLEMENT -> VERIFY -> LEARN`

**Resume at:** Record Learn, Retention and promotion and Archive readiness, then complete.

**Open obligations:** `NONE`

## Owned artifacts

-   `ledger.md` - the thread entries this Task took, and the questions that shaped it with the operator's answers.
-   `probe_names.py` - Probe: how git writes six kinds of file name in diff headers and `--name-status`, and what `gir fixup --dry-run` does with each; names Windows cannot hold are skipped there.
-   `logs/probe-fixed-20261002-1146.log` - Evidence: `probe_names.py` with the fix (the tree committed as `4f0fd88`).
-   `logs/test-final-20261002-1146.log` - Evidence: `cargo test --no-fail-fast` on that tree.
-   `logs/clippy-20261002-1146.log` - Evidence: `cargo clippy --all-targets -- -D warnings` on that tree.
-   `logs/tests-on-start-d6b61b1-20261002-1147.log` - Evidence: the new end-to-end tests run against the committed source before the fix (`d6b61b1`).
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
-   State: `VERIFIED`
-   Scope: Linux, git 2.56.0, this branch.
-   Consequence if false: files with spaces in their names cannot be fixed up automatically.
-   Basis: [Verification](#verification-c1-space-in-name).

<a id="c2-quoted-name"></a>
#### `c2-quoted-name`

-   Claim: For a file whose name git quotes in diff headers (one containing a double quote, a backslash or a tab), `gir fixup` selects the right commit, and `gir fixup --split` on such a file creates one commit per target with the right content.
-   State: `VERIFIED`
-   Scope: Linux, git 2.56.0, this branch.
-   Consequence if false: such files cannot be fixed up or split.
-   Basis: [Verification](#verification-c2-quoted-name).

<a id="c3-end-to-end"></a>
#### `c3-end-to-end`

-   Claim: For both kinds of name, `gir fixup` and `gir fixup --split` followed by `git rebase --autosquash` give the intended history, and the build at the start of this Task fails the same checks.
-   State: `VERIFIED`
-   Scope: Linux, git 2.56.0, this branch.
-   Consequence if false: the fix looks right in `--dry-run` but the folded history is wrong.
-   Basis: [Verification](#verification-c3-end-to-end).

<a id="c4-regression-tests"></a>
#### `c4-regression-tests`

-   Claim: Integration tests cover `c1-space-in-name` and `c2-quoted-name`, they fail on the code at this Task's start, and `cargo test` and `cargo clippy --all-targets -- -D warnings` pass on the final revision.
-   State: `VERIFIED`
-   Scope: Linux, final revision of this branch.
-   Consequence if false: the fix regresses unnoticed.
-   Basis: [Verification](#verification-c4-regression-tests).

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

-   `4f0fd88` adds `header_path` and `unquote` to `src/cmd/fixup.rs` and uses `header_path` for both `---` and `+++` lines in `parse_hunks`; `trace` reads `git diff --cached --name-status -z`.
-   Tests in the same commit: unit tests `header_paths_drop_the_tab_terminator_and_decode_quoting` and `new_files_with_quoted_names_need_an_explicit_target`; integration tests `file_name_with_a_space_is_traced_and_split` (all platforms) and `quoted_file_names_are_traced_and_split` (`#[cfg(unix)]`, with the reason in a comment), both through `gir fixup --dry-run`, `gir fixup --split` and `git rebase --autosquash` via the helper `trace_and_split_two_commits_in`.
-   Before the fix both integration tests failed (exit `2` where `0` was expected), and they fail again when run against the committed source without the fix (`logs/tests-on-start-d6b61b1-20261002-1147.log`).
-   No deviation from Decide.

## Verify

<a id="verification-p1-space-name-fails"></a>
### Verification: `p1-space-name-fails`

- Claim: [p1-space-name-fails](#p1-space-name-fails)
- Method: `probe_names.py` with the build at `8a2a60c`.
- Evidence considered: `logs/probe-head-8a2a60c-20261002-1145.log`: `'my file.txt'` gives `gir: cannot tell which commit my file.txt belongs to`, exit `2`; the three quoted names fail the same way.
- Conclusion: `VERIFIED`.
- Limitations: Linux, git 2.56.0.

<a id="verification-c1-space-in-name"></a>
### Verification: `c1-space-in-name`

- Claim: [c1-space-in-name](#c1-space-in-name)
- Method: `probe_names.py` before and after the fix; `file_name_with_a_space_is_traced_and_split`.
- Evidence considered: `logs/probe-fixed-20261002-1146.log`: `'my file.txt'` selects `feat: add file`, where `logs/probe-head-8a2a60c-20261002-1145.log` refused. `logs/test-final-20261002-1146.log`: the integration test passes, and it fails on the unfixed source (`logs/tests-on-start-d6b61b1-20261002-1147.log`).
- Conclusion: `VERIFIED`.
- Limitations: Linux, git 2.56.0. The test also runs on Windows in CI, not observed here.

<a id="verification-c2-quoted-name"></a>
### Verification: `c2-quoted-name`

- Claim: [c2-quoted-name](#c2-quoted-name)
- Method: `probe_names.py` before and after the fix; `quoted_file_names_are_traced_and_split` and the two unit tests.
- Evidence considered: `logs/probe-fixed-20261002-1146.log`: `say "hi".txt`, `back\slash.txt` and `tab\there.txt` each select `feat: add file`, where before all three were refused. `logs/test-final-20261002-1146.log`: the integration test (four names, including one with a space and quotes) and both unit tests pass; the integration test fails on the unfixed source (`logs/tests-on-start-d6b61b1-20261002-1147.log`).
- Conclusion: `VERIFIED`.
- Limitations: Unix only by nature: Windows cannot hold these names, so the integration test is `#[cfg(unix)]`; the unit tests run on every platform.

<a id="verification-c3-end-to-end"></a>
### Verification: `c3-end-to-end`

- Claim: [c3-end-to-end](#c3-end-to-end)
- Method: both integration tests run `gir fixup --dry-run`, then `gir fixup --split`, then `git rebase -i --autosquash main`, and check the history and both commits' file contents; run on the fixed and the unfixed source.
- Evidence considered: `logs/test-final-20261002-1146.log` (both pass); `logs/tests-on-start-d6b61b1-20261002-1147.log` (both fail on the unfixed source).
- Conclusion: `VERIFIED`.
- Limitations: Linux, git 2.56.0.

<a id="verification-c4-regression-tests"></a>
### Verification: `c4-regression-tests`

- Claim: [c4-regression-tests](#c4-regression-tests)
- Method: read the tests in `4f0fd88`; ran them on the unfixed source; ran the full suite and clippy.
- Evidence considered: `logs/tests-on-start-d6b61b1-20261002-1147.log`; `logs/test-final-20261002-1146.log` (211 passed, 0 failed); `logs/clippy-20261002-1146.log` (no warnings).
- Conclusion: `VERIFIED`.
- Limitations: run on Linux only here.

VERIFY gate: `ESTABLISHED`; every success Claim is `VERIFIED`.

## Learn

### Technical

git's text output quotes unusual file names (C-style, and a trailing tab after names with spaces in patch headers). Where gir only needs names, `-z` output avoids quoting entirely; where it must read patch text, the header has to be decoded. The unit test `header_paths_drop_the_tab_terminator_and_decode_quoting` enforces the decoding.

### Process

The Investigate section was written and committed together with Understand, before the `UNDERSTAND -> INVESTIGATE` transition, which is work ahead of the current State. No content depended on it, and the gate result stands; noted, no framework change.

LEARN gate: `ESTABLISHED`.

## Retention and promotion

The technical Learning is enforced by tests; no other permanent change.

### Promotion: success Claims

-   Claims: `c1-space-in-name`, `c2-quoted-name`, `c3-end-to-end`, `c4-regression-tests`.
-   Will this Claim's validity outlive this Task and inform a future decision? `no`; the behavior is required by `spec/fixup.md` and guarded by tests.
-   Disposition: not promoted — Task-scoped only.

### Promotion: `p1-space-name-fails`

-   Claim: [p1-space-name-fails](#p1-space-name-fails)
-   Will this Claim's validity outlive this Task and inform a future decision? `no`; it described the code before the fix.
-   Disposition: not promoted — Task-scoped only.

No Claim promoted to Knowledge, and no Claim carried forward to `open-claims/`.

## Archive readiness

The bundle holds its ledger, its probe and every log it cites under `logs/`; internal links are relative.
References to source files, commits and `spec/fixup.md` are supplemental.

## Terminal record

### Summary

`PENDING`

### Gate basis

`PENDING`
