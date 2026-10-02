# Ledger — doctor-fix-missing-hook

## Taken from the thread `fixup-review-20261002`

Entries Q14 of `ledger/fixup-review-20261002.md`, copied unchanged.

- Q: Does `gir doctor --fix` cope with a tracked hook file that is missing from the working tree?
  A: No, observed while testing the Task `doctor-path-names`: `git update-index --chmod=+x` needs the file, so `--fix` stops with `fatal: ... does not exist and --remove not passed` after applying the earlier fixes.
  This can happen when the hook is outside a sparse checkout's cone, or deleted locally.
  Possible fix: set the mode from the index entry (`git update-index --cacheinfo 100755,<object>,<path>`), which needs no working-tree file. Not tested.

## This Task

- Q: Which finding does this Task take?
  A: Q14, chosen by the operator on 2026-10-02 as the most serious open finding.
- Q: Who implements it?
  A: Codex, through the `harness-driver` skill in worker mode, in a throwaway clone in the supervisor's scratchpad, sandbox `workspace-write` only. If cargo cannot build there, stop and ask the operator rather than loosening the sandbox. The supervisor owns this Task and verifies Codex's diff before committing it.
- Q: Isolated workspace and line of development?
  A: No new one for the Task: worktree `.worktrees/fixup-modes`, branch `feat/fixup-modes`.
- Q: Does this Task overlap another active Task?
  A: No other Task is active.
- Q: Are the objective and success criteria viable and desirable?
  A: Agreed by the operator, as written in `TASK.md`.
- Q: Does the sparse-checkout case in Q14 reproduce?
  A: No. At `3ab3c09`, `--fix` handles hooks outside the cone correctly; Q14 inferred that case wrongly. The deleted-hook case reproduces, and `--fix` was also found to stage unstaged edits to hooks.
- Q: How should the criteria change, given that the sparse case does not reproduce and the staging problem does?
  A: Narrow `c1-reproduced` to the deleted hook and the staging problem; `c2-fixed` also checks that unstaged edits stay unstaged and sparse hooks stay `skip-worktree`.
- Q: How should exec-bit treat a file with an unresolved merge conflict?
  A: Skip it: no exec-bit warning and no fix while it has conflict entries; included in this Task, with a spec rule and a regression test. Found while reviewing Codex's first diff: both the old and the new `--fix` mark such a hook resolved with the wrong content.
