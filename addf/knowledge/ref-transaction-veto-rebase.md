---
evidence:
- anchor: "`f5-rebase-veto` cases"
  role: behavioral-evidence
  source: "`addf/.archive/tasks/ref-transaction-veto-knowledge/logs/probe-veto-20261003-1945.log`"
- anchor: "`case`, `HOOKS`"
  role: behavioral-evidence
  source: "`addf/.archive/tasks/ref-transaction-veto-knowledge/probe-veto.py`"
verification:
  method:
  - probe
  verified_at: "`2026-10-03T15:35:54+02:00`"
---

# Knowledge: `git rebase` under a `reference-transaction` veto

## Claim

<a id="ref-transaction-veto-rebase"></a>
-   Claim: When a `reference-transaction` hook exits non-zero in the `prepared` state for the update of the checked-out branch, `git rebase` exits `128` with the branch unmoved, `HEAD` detached at the rebased commit and the rebase still in progress; `git rebase --abort` then returns the working tree and index to the branch's tree but exits `128` with `HEAD` still detached and the rebase in progress, because it updates the branch with an all-zero old value, which a hook comparing old and new values treats as a change; `git reset --hard HEAD` leaves `HEAD` detached and the rebase in progress; `git rebase --quit` ends the rebase with `HEAD` still detached and moves an `--autostash` stash into the stash list.
-   State: `VERIFIED`
-   Scope: Linux, git 2.56.0, a shell hook in `.git/hooks`.
-   Consequence if false: a design that guards a branch with a `reference-transaction` hook misjudges what a refused update leaves behind.
-   Basis: `history:tasks/ref-transaction-veto-knowledge/TASK.md#verification-f5-rebase-veto`

## Derivation

`probe-veto.py` starts each case with `main` checked out, a tracked file `notes.txt` edited but not staged and an untracked file `scratch.txt`, installs a hook that exits `1` in `prepared` for `refs/heads/main` (hook `any`: every update; hook `moves`: only one whose old and new values differ), runs the operation, then a recovery command, and prints the exit codes, output, branch, `HEAD`, status, files, state files under `.git`, stash list and both files' content.
In the log, the ten `k5` cases: after the veto, `HEAD=detached` and the state files include `rebase-merge`; `git rebase --abort` exits `128` under both hooks, the hook having received `0000000000000000000000000000000000000000 <main's ID> refs/heads/main`, and leaves `HEAD` detached, `rebase-merge` present and status `D  incoming.txt`; `git reset --hard HEAD` exits `0` with `HEAD` detached and `rebase-merge` present; `git rebase --quit` exits `0`, prints `Autostash exists; creating a new stash entry.`, and leaves `HEAD` detached, no `rebase-merge` and `stash@{0}: autostash`.

## Limitations

-   Observed on Linux with git 2.56.0 only, with a shell hook in `.git/hooks`; Windows and a hook under `core.hooksPath` were not probed.
-   Two hook designs were probed: one refusing every update of the branch, one refusing only an update whose old and new values differ. A hook comparing the new value with the ref's current value was not probed.
-   gir installs no `reference-transaction` hook today; this records git's behaviour for a future decision.
