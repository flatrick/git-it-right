# TASK — `gir fixup --split commits exactly the staged bytes`

## Resume

**Contract version:** `2`

**State:** `LEARN`

**State path:** `DEFINE -> UNDERSTAND -> INVESTIGATE -> DECIDE -> IMPLEMENT -> VERIFY -> LEARN`

**Resume at:** Record Learn, Retention and promotion and Archive readiness, then complete.

**Open obligations:** The Windows branch of `os_path` is unverified until the operator's Windows testing (Deferred verification); it does not block completion, since no success Claim covers Windows.

## Owned artifacts

-   `ledger.md` - the thread entries this Task took, and the questions that shaped it with the operator's answers.
-   `acceptance.py` - Probe and acceptance: each byte-sensitive case through `--split` and `git rebase --autosquash`, comparing each commit's bytes; the non-UTF-8 name case is Unix-only.
-   `logs/acceptance-fixed-20261002-1156.log` - Evidence: `acceptance.py` with the fix (the tree committed as `0dac9db`).
-   `logs/test-final-20261002-1156.log` - Evidence: `cargo test --no-fail-fast` on that tree.
-   `logs/clippy-20261002-1156.log` - Evidence: `cargo clippy --all-targets -- -D warnings` on that tree.
-   `logs/tests-on-start-a6ae550-20261002-1156.log` - Evidence: the new integration tests against the committed source before the fix (`a6ae550`).
-   `logs/regression-sweep-20261002-1157.log` - Evidence: the previous acceptance script, the review scripts (`d246ab2` against this build), and `gir fixup --dry-run` on a non-UTF-8 name.
-   `logs/acceptance-head-7b5b2d2-20261002-1153.log` - Evidence: `acceptance.py` at the Task's start.

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
-   State: `VERIFIED`
-   Scope: Linux, git 2.56.0, this branch.
-   Consequence if false: Windows-style files cannot be split.
-   Basis: [Verification](#verification-c1-crlf).

<a id="c2-non-utf8-content"></a>
#### `c2-non-utf8-content`

-   Claim: A file whose content is not UTF-8 (Latin-1) splits into the right commits with its bytes unchanged.
-   State: `VERIFIED`
-   Scope: Linux, git 2.56.0, this branch.
-   Consequence if false: files in legacy encodings cannot be split, or are corrupted.
-   Basis: [Verification](#verification-c2-non-utf8-content).

<a id="c3-whitespace"></a>
#### `c3-whitespace`

-   Claim: A staged last line ending in spaces, and lines with trailing whitespace, split into the right commits with their bytes unchanged.
-   State: `VERIFIED`
-   Scope: Linux, git 2.56.0, this branch.
-   Consequence if false: the split refuses or changes whitespace.
-   Basis: [Verification](#verification-c3-whitespace).

<a id="c4-settings"></a>
#### `c4-settings`

-   Claim: With `apply.whitespace=error`, with `apply.whitespace=fix`, and with a `textconv` driver configured for the file, `--split` succeeds and commits the staged bytes unchanged.
-   State: `VERIFIED`
-   Scope: Linux, git 2.56.0, this branch.
-   Consequence if false: a user setting blocks or silently alters the split.
-   Basis: [Verification](#verification-c4-settings).

<a id="c5-end-to-end"></a>
#### `c5-end-to-end`

-   Claim: For `c1-crlf` to `c4-settings` and `c7-non-utf8-name`, `--split` followed by `git rebase --autosquash` leaves exactly the staged bytes in the right commits, and the build at this Task's start fails those checks.
-   State: `VERIFIED`
-   Scope: Linux, git 2.56.0, this branch.
-   Consequence if false: the fix looks right but the folded history differs from what was staged.
-   Basis: [Verification](#verification-c5-end-to-end).

<a id="c6-regression-tests"></a>
#### `c6-regression-tests`

-   Claim: Integration tests cover `c1-crlf` to `c4-settings` and `c7-non-utf8-name` and fail on the code at this Task's start; `cargo test` and `cargo clippy --all-targets -- -D warnings` pass on the final revision.
-   State: `VERIFIED`
-   Scope: Linux, final revision of this branch.
-   Consequence if false: the fix regresses unnoticed.
-   Basis: [Verification](#verification-c6-regression-tests).

<a id="c7-non-utf8-name"></a>
#### `c7-non-utf8-name`

-   Claim: A file whose name is not valid UTF-8 is traced by `gir fixup` and split by `gir fixup --split` into the right commits.
-   State: `VERIFIED`
-   Scope: Unix only, since Windows file names cannot hold such bytes; Linux, git 2.56.0, this branch.
-   Consequence if false: such files cannot be fixed up or split.
-   Basis: [Verification](#verification-c7-non-utf8-name).

### Constraints

-   Work in worktree `.worktrees/fixup-modes` on branch `feat/fixup-modes` (operator's answer in `ledger.md`).
-   Code and test scripts follow `rules/os-agnostic-code.md`; tests for non-UTF-8 names are Unix-only with the reason stated.
-   Out of scope: thread Q5 and Q6 (next Task).
-   No other Task is active.

### Material empirical premises

<a id="p1-split-loses-bytes"></a>
#### `p1-split-loses-bytes`

-   Claim: At this Task's starting revision, `gir fixup --split` fails for a CRLF file, a Latin-1 file, a staged last line ending in spaces, and with `apply.whitespace=error`.
-   State: `VERIFIED`
-   Scope: Linux, git 2.56.0, `7b5b2d2`.
-   Consequence if false: the review's reproductions do not hold here.
-   Basis: [Verification](#verification-p1-split-loses-bytes).

### DEFINE gate

`ESTABLISHED`: on 2026-10-02 the operator explicitly agreed that the objective, success criteria and order are right, as written here (`ledger.md`).

## Understand

### Relevant context

-   At `7b5b2d2`, `acceptance.py` (`logs/acceptance-head-7b5b2d2-20261002-1153.log`) fails CRLF, Latin-1, a staged last line ending in spaces, `apply.whitespace=error` and `=fix`, a textconv driver, and a non-UTF-8 file name; a last line ending in spaces with no final newline passes, because git's `\ No newline` line follows it.
-   Bytes are lost in four places: `git::run` decodes stdout lossily as UTF-8 and trims trailing whitespace; `parse_hunks` splits with `str::lines`, which drops `\r`; `git apply --cached` follows `apply.whitespace`; `git diff` applies textconv by default.
-   Paths are `String` throughout `trace`, `parse_hunks`, `blame` and the `--name-status -z` cross-check, so a non-UTF-8 name never matches and cannot be passed to `git blame`.
-   `split.rs` builds the patch from each hunk's `header` and `changes` strings and passes it to `TempIndex::run_with_stdin` as `&str`; `git::run_with_stdin` has no other caller.

### Assumptions

-   On Windows, git writes paths as UTF-8, so converting path bytes to an OS string through UTF-8 there loses nothing for names Windows can hold. Source: Git for Windows stores paths as UTF-8; not verified here, since Windows is not available.

### Open questions

-   `NONE`.

### Deferred verification

-   The Windows path conversion branch runs in CI on `windows-latest`; this Task cannot observe it. Earliest checkpoint: the operator's later Windows testing. Consequence if false: paths with non-ASCII characters fail on Windows.

### UNDERSTAND gate

`ESTABLISHED`: every case reproduces, and every place that loses bytes is identified.

## Investigate

No further probe is needed: `acceptance.py` already reproduces every case, and the causes were read from the code.

### INVESTIGATE gate

`ESTABLISHED`.

## Decide

-   **Raw output.** `git::run_raw` returns git's stdout as bytes, neither decoded nor trimmed, and accepts any `AsRef<OsStr>` arguments. `trace` reads the staged diff and `--name-status -z` through it, and passes `--no-textconv` to `git diff`.
-   **Bytes in hunks.** `Hunk.path`, `Hunk.header` and `Hunk.changes` become `Vec<u8>`. `parse_hunks` takes `&[u8]` and splits with `split_inclusive(b'\n')`, so every line keeps its exact ending, including `\r`; header and `@@` lines are recognised on their bytes. `header_path` and `unquote` work on bytes.
-   **Paths to git.** A path's bytes become an `OsString` only where it is passed to `git blame`: on Unix directly from the bytes; elsewhere through UTF-8, which is how git writes paths on Windows. Both branches are written (`rules/os-agnostic-code.md`); the Unix one is exercised by the non-UTF-8 name test, the other by CI on Windows. Messages show paths through lossy UTF-8.
-   **Applying.** `split.rs` builds the patch as `Vec<u8>` and runs `git apply --cached --unidiff-zero --whitespace=nowarn`; `TempIndex::run_with_stdin` and the private helper take `&[u8]` input, and `git::run_with_stdin` passes its `&str` as bytes.
-   **Spec.** `NONE`: `split` already requires `HEAD`'s tree to equal the staged tree, and `staged-line-target` already applies to every staged line, whatever the file's name.
-   **Rejected:** keeping `String` and re-adding `\r` or trailing spaces, which would patch one symptom at a time; `OsString` for paths everywhere, which would need platform branches in the parser too.
-   **Verification strategy.** Integration tests first, each case as in `acceptance.py` (CRLF, Latin-1, trailing whitespace with and without a final newline, both `apply.whitespace` settings, textconv, and a Unix-only non-UTF-8 name), and see them fail; then the change; then `acceptance.py` on the new build, the new tests against the unfixed source, `cargo test` and clippy.

### DECIDE gate

`ESTABLISHED`: the design covers every place bytes are lost, and each success Claim has a planned test.

## Implement

-   `0dac9db` implements Decide: `git::run_raw`; `Hunk.path`, `header` and `changes` as `Vec<u8>`; `parse_hunks` over `split_inclusive(b'\n')`; `header_path` and `unquote` on bytes; `os_path` with a Unix and a non-Unix branch for `git blame`; `--no-textconv` on `git diff`; `--whitespace=nowarn` on `git apply`; byte input for `TempIndex::run_with_stdin`.
-   Existing unit tests changed only in types: `&str` inputs passed as bytes, and expected strings compared as bytes.
-   New integration tests in `tests/fixup_modes.rs`, through the helper `split_and_fold_keeps_bytes` (split, `rebase --autosquash`, compare each commit's bytes): `split_keeps_crlf_line_endings`, `split_keeps_non_utf8_content`, `split_keeps_trailing_whitespace` (with and without a final newline), `split_ignores_apply_whitespace_and_textconv_settings`, and `split_traces_non_utf8_file_names` (`#[cfg(unix)]`, with the reason in a comment). All five fail before the fix (`logs/tests-on-start-a6ae550-20261002-1156.log`).
-   No deviation from Decide.
-   In the review-script comparison (`logs/regression-sweep-20261002-1157.log`), `edge.py` case 3b now reports `nothing staged`. That is the script, not gir: it reuses case 3's repository, and case 3 now succeeds and commits everything. `apply.whitespace=fix` itself is covered by `acceptance.py` and `split_ignores_apply_whitespace_and_textconv_settings`, which pass.

## Verify

<a id="verification-p1-split-loses-bytes"></a>
### Verification: `p1-split-loses-bytes`

- Claim: [p1-split-loses-bytes](#p1-split-loses-bytes)
- Method: `acceptance.py` with the build at `7b5b2d2`.
- Evidence considered: `logs/acceptance-head-7b5b2d2-20261002-1153.log`: CRLF and Latin-1 fail with `patch does not apply`, a last line ending in spaces with `do not add up`, `apply.whitespace=error` with `1 line adds whitespace errors`.
- Conclusion: `VERIFIED`.
- Limitations: Linux, git 2.56.0.

<a id="verification-c1-crlf"></a>
### Verification: `c1-crlf`

- Claim: [c1-crlf](#c1-crlf)
- Method: `acceptance.py` and the integration tests, on the build at `0dac9db` and at the start.
- Evidence considered: `acceptance.py` case c1 passes on the fixed build (`logs/acceptance-fixed-20261002-1156.log`), where it failed with `patch does not apply` at the start (`logs/acceptance-head-7b5b2d2-20261002-1153.log`); `split_keeps_crlf_line_endings` passes (`logs/test-final-20261002-1156.log`) and failed on the unfixed source (`logs/tests-on-start-a6ae550-20261002-1156.log`).
- Conclusion: `VERIFIED`.
- Limitations: Linux, git 2.56.0.

<a id="verification-c2-non-utf8-content"></a>
### Verification: `c2-non-utf8-content`

- Claim: [c2-non-utf8-content](#c2-non-utf8-content)
- Method: `acceptance.py` and the integration tests, on the build at `0dac9db` and at the start.
- Evidence considered: `acceptance.py` case c2 passes (logs/acceptance-fixed-20261002-1156.log; before: `patch does not apply`); `split_keeps_non_utf8_content` passes (`logs/test-final-20261002-1156.log`) and failed before (`logs/tests-on-start-a6ae550-20261002-1156.log`); review script `edge3.py` case 21 now splits (`logs/regression-sweep-20261002-1157.log`).
- Conclusion: `VERIFIED`.
- Limitations: Linux, git 2.56.0.

<a id="verification-c3-whitespace"></a>
### Verification: `c3-whitespace`

- Claim: [c3-whitespace](#c3-whitespace)
- Method: `acceptance.py` and the integration tests, on the build at `0dac9db` and at the start.
- Evidence considered: `acceptance.py` cases c3a and c3b pass (logs/acceptance-fixed-20261002-1156.log; c3a failed before with `do not add up`); `split_keeps_trailing_whitespace` passes (`logs/test-final-20261002-1156.log`) and failed before (`logs/tests-on-start-a6ae550-20261002-1156.log`); review script `edge3.py` case 20 now splits (`logs/regression-sweep-20261002-1157.log`).
- Conclusion: `VERIFIED`.
- Limitations: Linux, git 2.56.0.

<a id="verification-c4-settings"></a>
### Verification: `c4-settings`

- Claim: [c4-settings](#c4-settings)
- Method: `acceptance.py` and the integration tests, on the build at `0dac9db` and at the start.
- Evidence considered: `acceptance.py` cases for `apply.whitespace=error`, `=fix` and the textconv driver pass (logs/acceptance-fixed-20261002-1156.log; all three failed before); `split_ignores_apply_whitespace_and_textconv_settings` passes (`logs/test-final-20261002-1156.log`) and failed before (`logs/tests-on-start-a6ae550-20261002-1156.log`).
- Conclusion: `VERIFIED`.
- Limitations: Linux, git 2.56.0.

<a id="verification-c5-end-to-end"></a>
### Verification: `c5-end-to-end`

- Claim: [c5-end-to-end](#c5-end-to-end)
- Method: `acceptance.py` and the integration tests, on the build at `0dac9db` and at the start.
- Evidence considered: Every `acceptance.py` case and every new integration test runs `--split`, then `git rebase -i --autosquash main`, and compares both commits' bytes; all pass on the fixed build (logs/acceptance-fixed-20261002-1156.log: 48 PASS, 0 FAIL; logs/test-final-20261002-1156.log) and fail at the start (logs/acceptance-head-7b5b2d2-20261002-1153.log: 7 of 8 cases fail; logs/tests-on-start-a6ae550-20261002-1156.log).
- Conclusion: `VERIFIED`.
- Limitations: Linux, git 2.56.0.

<a id="verification-c6-regression-tests"></a>
### Verification: `c6-regression-tests`

- Claim: [c6-regression-tests](#c6-regression-tests)
- Method: `acceptance.py` and the integration tests, on the build at `0dac9db` and at the start.
- Evidence considered: The five integration tests fail on the unfixed source (`logs/tests-on-start-a6ae550-20261002-1156.log`) and pass on `0dac9db` (`logs/test-final-20261002-1156.log: 216 passed, 0 failed`); clippy reports no warnings (`logs/clippy-20261002-1156.log`).
- Conclusion: `VERIFIED`.
- Limitations: Linux only here.

<a id="verification-c7-non-utf8-name"></a>
### Verification: `c7-non-utf8-name`

- Claim: [c7-non-utf8-name](#c7-non-utf8-name)
- Method: `acceptance.py` and the integration tests, on the build at `0dac9db` and at the start.
- Evidence considered: `acceptance.py` case c7 passes (logs/acceptance-fixed-20261002-1156.log; before: `cannot tell which commit caf\ufffd.txt:1 belongs to`); `split_traces_non_utf8_file_names` passes (`logs/test-final-20261002-1156.log`) and failed before (`logs/tests-on-start-a6ae550-20261002-1156.log`); plain `gir fixup --dry-run` selects the commit (`logs/regression-sweep-20261002-1157.log`).
- Conclusion: `VERIFIED`.
- Limitations: Unix only by nature; the non-Unix `os_path` branch is exercised only by CI on Windows, not observed here.

VERIFY gate: `ESTABLISHED`; every success Claim is `VERIFIED`. The Windows path conversion stays a deferred verification for the operator's Windows testing.

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
