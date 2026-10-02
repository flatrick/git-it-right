# Ledger — doctor-fix-file-mode

## Taken from the thread `fixup-review-20261002`

Entries Q15 of `ledger/fixup-review-20261002.md`, copied unchanged.

- Q: After `gir doctor --fix` sets a script's exec bit, does the file on disk match?
  A: No, observed while verifying the Task `doctor-fix-missing-hook`, on both the old (`--chmod=+x`) and the new code: the index entry becomes `100755`, but the file stays `100644`. With `core.filemode=true`, `git status` then shows every fixed script as modified, and a later `git add` would stage `100644` again, undoing the fix.
  Possible fix: also set the executable bit on the file when it exists (Unix only; Windows has no such bit). Not tested.

## This Task

- Q: Which finding does this Task take, and who does the work?
  A: Q15, handed by the operator on 2026-10-02 to a sub-agent with a fresh context. The supervising session wrote this Task's DEFINE with the operator, then delegated from `UNDERSTAND` on; the sub-agent stops and reports instead of deciding anything that changes the criteria or the specification beyond `c5-spec`.
- Q: Isolated workspace and line of development?
  A: No new one: worktree `.worktrees/fixup-modes`, branch `feat/fixup-modes`.
- Q: Does this Task overlap another active Task?
  A: No other Task is active.
- Q: Are the objective and success criteria viable and desirable?
  A: Agreed by the operator, as written in `TASK.md`.
