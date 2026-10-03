---
evidence:
- anchor: "`f4-pull-veto` cases"
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

# Knowledge: A fast-forward `git pull` under a `reference-transaction` veto

## Claim

<a id="ref-transaction-veto-pull"></a>
-   Claim: When a `reference-transaction` hook exits non-zero in the `prepared` state for the update of the checked-out branch, a fast-forward `git pull` exits `128` with the branch unmoved, but the index and working tree already hold the pulled changes, staged, beside any uncommitted edits made before; `git reset --hard HEAD` then returns the index and tracked files to the branch's commit, discarding those earlier tracked edits and keeping untracked files, and when the hook also refuses an update that leaves the branch where it is, the reset still does this but exits `128`.
-   State: `VERIFIED`
-   Scope: Linux, git 2.56.0, a shell hook in `.git/hooks`.
-   Consequence if false: a design that guards a branch with a `reference-transaction` hook misjudges what a refused update leaves behind.
-   Basis: `history:tasks/ref-transaction-veto-knowledge/TASK.md#verification-f4-pull-veto`

## Derivation

`probe-veto.py` starts each case with `main` checked out, a tracked file `notes.txt` edited but not staged and an untracked file `scratch.txt`, installs a hook that exits `1` in `prepared` for `refs/heads/main` (hook `any`: every update; hook `moves`: only one whose old and new values differ), runs the operation, then a recovery command, and prints the exit codes, output, branch, `HEAD`, status, files, state files under `.git`, stash list and both files' content.
In the log, the two `k4: git pull (fast-forward)` cases: every line matches the `k1` cases: veto `exit 128`, `main` unchanged, `A  incoming.txt` beside ` M notes.txt`; `git reset --hard HEAD` exits `0` under `moves` and `128` under `any`, leaving `?? scratch.txt` only.

## Limitations

-   Observed on Linux with git 2.56.0 only, with a shell hook in `.git/hooks`; Windows and a hook under `core.hooksPath` were not probed.
-   Two hook designs were probed: one refusing every update of the branch, one refusing only an update whose old and new values differ. A hook comparing the new value with the ref's current value was not probed.
-   gir installs no `reference-transaction` hook today; this records git's behaviour for a future decision.
