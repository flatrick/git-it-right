# Ledger — split-patch-fidelity

## Taken from the thread `fixup-review-20261002`

Entries Q3, Q4 of `ledger/fixup-review-20261002.md`, copied unchanged.

- Q: Does `--split` rebuild a patch that matches the bytes in the index?
  A: No, reproduced.
  `src/cmd/fixup.rs:292` builds it from `diff.lines()`, which drops `\r`, and `git::run` uses `from_utf8_lossy` and trims the output.
  A CRLF file (`* -text`) and a Latin-1 file both fail with `patch does not apply`.
  A staged last line ending in spaces fails with `the split commits do not add up to the staged changes`.
  HEAD and the index are restored each time.
  Suggested fix: split raw bytes with `split_inclusive('\n')`, through a git runner that does not trim.
- Q: Is `--split` affected by the user's `apply.whitespace` setting?
  A: Yes, reproduced.
  `git apply --cached` at `src/cmd/fixup/split.rs:46` follows it.
  With `error`, a staged line with trailing whitespace aborts the split.
  With `fix`, apply strips the whitespace, the round has nothing to commit, and the user sees `no changes added to commit` and then `git commit --fixup=... failed`.
  Suggested fix: pass `--whitespace=nowarn`, and consider `--no-textconv` on the `diff --cached` call.

## This Task

- Q: Which findings does this Task take?
  A: Q3 and Q4, as the operator ordered on 2026-10-02 (thread entry Q13).
- Q: Are non-UTF-8 file names part of this Task?
  A: Yes, the operator chose to include them, which adds `c7-non-utf8-name`; they are possible on Linux but not on Windows, and need paths handled as bytes.
- Q: Isolated workspace and line of development?
  A: No new one: worktree `.worktrees/fixup-modes`, branch `feat/fixup-modes`.
- Q: Does this Task overlap another active Task?
  A: No other Task is active.
- Q: Are the objective and success criteria viable and desirable?
  A: Agreed by the operator, as written in `TASK.md`, with `c7-non-utf8-name` from the scope answer above.
