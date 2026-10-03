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
