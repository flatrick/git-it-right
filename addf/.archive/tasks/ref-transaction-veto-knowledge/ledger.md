# Ledger — `ref-transaction-veto-knowledge`

The exploration that shaped this Task and its DEFINE dialogue.
It came from a phase-0 investigation of WIP commits on 2026-10-03, whose memo and probe logs lived under `.agents/` (never committed, since removed).

Append-only: never edit or delete a prior entry; append new ones at the end.

- Q1: When a `reference-transaction` hook vetoes, in its `prepared` phase, the update of `refs/heads/main` during `git merge --ff-only`, what state is the repository left in?
- A1: In the phase-0 probe (git 2.56.0, Linux, a shell hook): `main` did not move and the merge exited `128` with `fatal: in 'prepared' phase, update aborted by the reference-transaction hook`, but the working tree and index already held the incoming commit's changes, staged (`git status --short` showed `A  a`). A direct `git commit --no-verify` vetoed the same way also left its changes staged.
  Repro: a repository with one commit on `main`; a branch `feature` with one more commit; a `reference-transaction` hook that exits `1` in `prepared` for `refs/heads/main`; then `git switch main && git merge --ff-only feature`.
- Q2: Where should this finding be documented?
- A2: As addf Knowledge (operator answer, 2026-10-03), which needs its own Task: a Verification, then promotion at Complete.
- Q3: Should that Task run in a new isolated workspace and line of development?
- A3: Yes: a separate Task in its own worktree (operator answer, 2026-10-03). Worktree `.worktrees/knowledge-ref-transaction-veto`, branch `knowledge-ref-transaction-veto`, from `3e10963`.

DEFINE dialogue with the operator, 2026-10-03.

- Q4: Which git operations should the Knowledge cover, one Knowledge file each?
- A4: `git merge --ff-only`, a direct `git commit`, `git merge --no-ff`, `git pull` and `git rebase`.
- Q5: addf's Knowledge template is for claims about the current repository or system, and gir has no `reference-transaction` hook. Use Knowledge anyway, or an open claim?
- A5: Knowledge: a verified fact about git, the system gir runs on, that informs a future design decision; each file says gir does not use the hook today.
- Q6: Should each file include an observed recovery, such as what `git reset --hard HEAD` does afterwards?
- A6: Yes, stated as an observed fact, not advice, including its effect on uncommitted changes that existed before the operation.
- Q7: Are the objective and success criteria k1–k5 in `TASK.md` viable, actionable and the outcome the operator wants?
- A7: Yes: "Agreed, proceed".
