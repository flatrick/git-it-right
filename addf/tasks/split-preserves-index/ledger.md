# Ledger — split-preserves-index

## Taken from the thread `fixup-review-20261002`

Entries Q1, Q8 of `ledger/fixup-review-20261002.md`, copied unchanged.

- Q: Does `gir fixup --split` leave sparse-checkout, `skip-worktree` and intent-to-add index entries intact?
  A: No, reproduced.
  `src/cmd/fixup/split.rs:45` runs `git read-tree ORIG` on every round and on restore, which rebuilds the whole index.
  After a split, files outside the sparse set show as ` D`, a `skip-worktree` file with a local edit shows as ` M`, and a `git add -N` file becomes `??`.
  A later `git add -A` or `git commit -a` would then delete or commit those files.
  Suggested fix: build each round in a temporary index (`GIT_INDEX_FILE`, seeded from orig) and never touch the real index.
- Q: Does `--split` throw away the index stat cache?
  A: Likely, inferred from how `read-tree` works, not measured.
  Each round's full `read-tree` at `src/cmd/fixup/split.rs:45` would make the next `git commit` and `git status` re-hash every tracked file, about N+1 times for N targets.
  The temporary index from the first entry would also fix this.

## This Task

- Q: Which findings does this Task take?
  A: Q1 and Q8, chosen by the operator on 2026-10-02.
- Q: Isolated workspace and line of development?
  A: No new one: worktree `.worktrees/fixup-modes`, branch `feat/fixup-modes`.
- Q: This Task overlaps `diff-header-parsing` in `src/cmd/fixup.rs`. In which order do they publish?
  A: `diff-header-parsing` first, then `split-preserves-index`, which re-reads the current spec after the first terminalizes, before its own `VERIFY`.
- Q: Are the objective, success criteria and order viable and desirable?
  A: Agreed by the operator, as written in `TASK.md`.
