---
evidence:
- anchor: "scenarios `c1: autosquash of a fixup! of the merge` and `c1: autosquash of a fixup! of the side-branch commit`"
  role: behavioral-evidence
  source: "`addf/.archive/tasks/unfoldable-fixup-targets/logs/probe-02b4061-20261003-1623.log`"
- anchor: "`repo`, `autosquash`"
  role: behavioral-evidence
  source: "`addf/.archive/tasks/unfoldable-fixup-targets/probe.py`"
- anchor: "`=== merge commit as a target (r3)`"
  role: behavioral-evidence
  source: "`addf/.archive/tasks/autosquash-review-fixes/logs/probe-git-rules-20261003-1726.log`"
verification:
  method:
  - probe
  verified_at: "`2026-10-03T16:23:00+02:00`"
---

# Knowledge: what a plain autosquash rebase folds around a merge

## Claim

<a id="autosquash-merge-targets"></a>
-   Claim: On a branch holding a `--no-ff` merge of a side branch, `git rebase -i --autosquash <base>` without `--rebase-merges` drops the merge commit from its todo and keeps the side branch's commits, linearised; a `fixup!` of the merge is left unfolded on the rewritten branch, and a `fixup!` of a side-branch commit is folded into that commit.
-   State: `VERIFIED`
-   Scope: Linux, git 2.56.0; the side branch forked from the base branch or from a commit of the topic branch.
-   Consequence if false: gir refuses or hides merge commits as autosquash targets that would fold, or offers side-branch commits that would not.
-   Basis: `history:tasks/unfoldable-fixup-targets/TASK.md#verification-p2-autosquash-merge-targets`

## Derivation

`probe.py` builds `main` (`chore: base`) and `topic` with `feat: a`, a `--no-ff` merge of `side` (`feat: side`) and `feat: b`, once with `side` forked from `main` and once from `feat: a`.
For each target it stages a change, runs `git commit --fixup=<target>`, then `git rebase -q -i --autosquash main` with `GIT_SEQUENCE_EDITOR=true`, and prints the subjects of `main..HEAD` before and after.
In the log, with the merge as the target, the after list keeps `fixup! Merge branch 'side' into topic` and no longer holds the merge; with `feat: side` as the target, the after list holds no `fixup!` and `s.txt` at `HEAD` has the fixup's content.
The `autosquash-review-fixes` probe log shows the same for a merge.

## Limitations

-   Observed on Linux with git 2.56.0 only.
-   With `--rebase-merges` the merge is kept in the todo; that case was not probed.
-   A commit already cherry-picked onto the base branch is also skipped by a plain rebase, inferred and not probed (Ledger thread `fixup-review-20261002`, Q17).
