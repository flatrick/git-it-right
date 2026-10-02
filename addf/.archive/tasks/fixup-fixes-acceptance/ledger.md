# Ledger — fixup-fixes-acceptance

No Ledger thread entries taken; the Task came from the operator's request after `diff-header-parsing` and `split-preserves-index` completed (2026-10-02).

- Q: Are the two fixes done?
  A: Not yet: the operator asked for proper testing that they fix the scenarios. Earlier evidence was mostly `--dry-run`, status and flags; no `git rebase --autosquash`, no `git add -A` / `git commit -a`, no linked worktree, no cone mode with a sparse index through gir, and the review's scripts were not rerun.
- Q: Which scenarios?
  A: All four offered: the review's scripts, end to end to the folded history, the step that causes harm, and the untested setups.
- Q: In what form?
  A: Cargo integration tests for regression-relevant cases, plus a before/after script whose logs are committed here.
- Q: Isolated workspace and line of development?
  A: No new one: worktree `.worktrees/fixup-modes`, branch `feat/fixup-modes`.
- Q: What if a scenario fails?
  A: Stop, record, report to the operator; a fix is a separate Task.
- Q: Are the objective and success criteria viable and desirable?
  A: Agreed by the operator, as written in `TASK.md`.
- Q: In `a3.2`, `a4.4` and `a4.5`, the stand-in `cfg.txt` is added on `topic`, so git refuses the rebase with or without gir. May it be committed on `main` instead?
  A: Yes (2026-10-02), after asking what was being asked; the question had first been unclear.
- Q: The copied review script `edge.py` uses `/dev/null` and a Linux-only default gir path. How should it meet the new rule `os-agnostic-code`?
  A: Make it OS-agnostic: `os.devnull`, and `GIR_BIN` required with no machine-specific default.
