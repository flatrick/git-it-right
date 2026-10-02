# Ledger — diff-path-names

## Taken from the thread `fixup-review-20261002`

Entries Q11 of `ledger/fixup-review-20261002.md`, copied unchanged.

- Q: Can `gir fixup` trace a staged change to a file whose name contains a space?
  A: No, reproduced on `d246ab2` and on the fixed build (review script `edge.py` case 2; `tasks/fixup-fixes-acceptance/logs/edge-old.log` and `edge-new.log` while that Task is active).
  `gir fixup` reports `cannot tell which commit my file.txt belongs to`.
  For such a name git writes the `---`/`+++` header with a trailing tab (`--- a/my file.txt` then TAB), and `parse_hunks` keeps the tab in the path, so the path never matches.
  Found by the acceptance run of the header and split-index fixes; not caused by either fix.

## This Task

- Q: Which finding does this Task take?
  A: Q11, chosen by the operator on 2026-10-02 as the first of the *NIX fixes.
- Q: Isolated workspace and line of development?
  A: No new one: worktree `.worktrees/fixup-modes`, branch `feat/fixup-modes`.
- Q: Does this Task overlap another active Task?
  A: No other Task is active.
- Q: Are the objective and success criteria viable and desirable?
  A: Agreed by the operator, as written in `TASK.md`.
