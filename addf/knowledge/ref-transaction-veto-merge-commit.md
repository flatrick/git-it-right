---
evidence:
- anchor: "`f3-merge-commit-veto` cases"
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

# Knowledge: `git merge --no-ff` under a `reference-transaction` veto

## Claim

<a id="ref-transaction-veto-merge-commit"></a>
-   Claim: When a `reference-transaction` hook exits non-zero in the `prepared` state for the update of the checked-out branch, `git merge --no-ff` exits `128` with the branch unmoved, the merged changes staged beside any earlier uncommitted edits, and `MERGE_HEAD` left, so a merge stays in progress; when the hook accepts an update that leaves the branch where it is, `git merge --abort` ends the merge and restores the branch's tree while keeping the earlier uncommitted edits, and `git reset --hard HEAD` ends it while discarding them; when the hook refuses every update of the branch, both still restore the files as described but exit `128` and leave `MERGE_HEAD` in place.
-   State: `VERIFIED`
-   Scope: Linux, git 2.56.0, a shell hook in `.git/hooks`.
-   Consequence if false: a design that guards a branch with a `reference-transaction` hook misjudges what a refused update leaves behind.
-   Basis: `history:tasks/ref-transaction-veto-knowledge/TASK.md#verification-f3-merge-commit-veto`

## Derivation

`probe-veto.py` starts each case with `main` checked out, a tracked file `notes.txt` edited but not staged and an untracked file `scratch.txt`, installs a hook that exits `1` in `prepared` for `refs/heads/main` (hook `any`: every update; hook `moves`: only one whose old and new values differ), runs the operation, then a recovery command, and prints the exit codes, output, branch, `HEAD`, status, files, state files under `.git`, stash list and both files' content.
In the log, the four `k3: git merge --no-ff` cases: after the veto the state files include `MERGE_HEAD`; under `moves`, `git merge --abort` exits `0` keeping ` M notes.txt` and `git reset --hard HEAD` exits `0` leaving `?? scratch.txt` only, both with only `ORIG_HEAD` left; under `any`, both exit `128` with the same files and `MERGE_HEAD` still present.

## Limitations

-   Observed on Linux with git 2.56.0 only, with a shell hook in `.git/hooks`; Windows and a hook under `core.hooksPath` were not probed.
-   Two hook designs were probed: one refusing every update of the branch, one refusing only an update whose old and new values differ. A hook comparing the new value with the ref's current value was not probed.
-   gir installs no `reference-transaction` hook today; this records git's behaviour for a future decision.
