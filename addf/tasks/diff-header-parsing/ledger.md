# Ledger — diff-header-parsing

## Taken from the thread `fixup-review-20261002`

Entries Q2 of `ledger/fixup-review-20261002.md`, copied unchanged.

- Q: Does `parse_hunks` handle a deleted line whose content starts with `-- `?
  A: No, reproduced.
  `src/cmd/fixup.rs:301` matches `--- ` on every line, so the deleted line `-- header` (Lua, SQL or Haskell comment) is read as a new file header.
  `gir fixup` then reports `cannot tell which commit header:5 belongs to`.
  A deleted `-- /dev/null` line makes every later hunk in that file skipped silently, and with several commits the fixup can go to the wrong commit with no error.
  Suggested fix: only match `---`/`+++` while still inside the diff header.

## This Task

- Q: Which findings does this Task take?
  A: Q2, chosen by the operator on 2026-10-02.
- Q: Isolated workspace and line of development?
  A: No new one: worktree `.worktrees/fixup-modes`, branch `feat/fixup-modes`.
- Q: This Task overlaps `split-preserves-index` in `src/cmd/fixup.rs`. In which order do they publish?
  A: `diff-header-parsing` first, then `split-preserves-index`, which re-reads the current spec after the first terminalizes, before its own `VERIFY`.
- Q: Are the objective, success criteria and order viable and desirable?
  A: Agreed by the operator, as written in `TASK.md`.
