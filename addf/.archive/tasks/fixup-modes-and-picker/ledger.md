# Ledger — fixup-modes-and-picker

Questions raised while shaping this Task, with the operator's answers (2026-09-27).
The root `LEDGER.md` held no entries when the Task started.

- Q: Git's third mode was named as `--fixup=squash:SHA`; does git have it?
  A: No. `git help commit` (git 2.55.0) lists `--fixup=[(amend|reword):]<commit>` and a separate `--squash=<commit>`. `gir squash` maps to `--squash=`.
- Q: Should the new modes be flags on `gir fixup` or separate subcommands?
  A: Separate subcommands: `gir amend`, `gir reword`, `gir squash`.
- Q: How can gir know whether the program running it can handle a picker (SourceTree, SourceGit, Visual Studio (Code), other clients)?
  A: It does not identify the client. The picker appears only when stdin and stderr are both terminals; `GIR_INTERACTIVE=0|1` overrides detection. Accepted with the plan.
- Q: When staged hunks belong to several commits, what should the picker offer?
  A: Pick one commit (with a warning) or split into one autosquash commit per target.
- Q: What kind of picker?
  A: A numbered prompt on stderr reading stdin; no new dependency.
- Q: Isolated workspace and line of development?
  A: Yes: worktree `.worktrees/feat-fixup-modes`, branch `feat/fixup-modes`.
- Q: Are the objective, scope and success criteria (the approved plan) viable and desirable?
  A: Agreed by the operator.
