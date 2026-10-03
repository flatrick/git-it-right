# Ledger — unfoldable-fixup-targets

## Taken from the thread `fixup-review-20261002`

Entry Q7 of `ledger/fixup-review-20261002.md`, copied unchanged.

- Q7: Can the commit picker offer merge commits as a fixup target?
- A7: Yes, reproduced up to the picker list.
  `pick_branch_commit` at `src/cmd/fixup.rs:158` lists `rev-list base..HEAD`, so after `git merge --no-ff side` it lists `Merge side` and the side branch's commits.
  That `fixup! Merge side` cannot be folded by a default `git rebase --autosquash` is inferred, not run.
  Suggested fix: add `--no-merges --first-parent`.

## This Task

- Q1: Isolated workspace and line of development?
- A1: Yes, decided by the operator on 2026-10-03: a new worktree `.worktrees/unfoldable-fixup-targets` and branch `unfoldable-fixup-targets`, started from `ledger-settle-open-entries` at `0f4aac1` so the thread's Q16 and D8 are present.
- Q2: Which commits should the picker leave out: merges only, or merges and side-branch commits (`--first-parent`)?
- A2: Decided after INVESTIGATE, by the operator on 2026-10-03.
  The agent noted, inferred and not run, that a flattening `git rebase -i --autosquash` probably keeps side-branch commits in its todo list, so `--first-parent` may hide commits that can be fixed up.
- Q3: Besides the picker list, which other places should leave out merge commits?
- A3: Explicit targets and automatic selection, chosen by the operator on 2026-10-03, in addition to the picker.
- Q4: What is the Task called?
- A4: `unfoldable-fixup-targets`, chosen by the operator on 2026-10-03, replacing the proposed `picker-first-parent`: the scope grew past the picker, and `--first-parent` is no longer presumed.
- Q5: Does this Task overlap another active Task?
- A5: No other Task is active.
- Q6: Are the objective, scope and success criteria c1–c6 viable, actionable and desirable, with `--rebase-merges` out of scope, the same filter when no base is found, and explicit unfoldable targets refused with exit `2`?
- A6: Agreed by the operator on 2026-10-03, as written in `TASK.md`.
- Q7: Which commits are left out, now that INVESTIGATE observed that a `fixup!` of a merge is left unfolded and a `fixup!` of a side-branch commit folds?
- A7: Merge commits only (`--no-merges`), following from the evidence, as the operator asked on 2026-10-03 (A2).
- Q8: What does automatic selection do with a staged line last changed by a merge?
- A8: Refuse it, like `base-limit`, chosen by the operator on 2026-10-03: `<place> was last changed by the merge <sha>, which a rebase drops; commit it normally, or pass one: gir <subcommand> <commit>`, exit `2`.
  An insertion is refused only when every neighbouring line was last changed by a merge; `--split` refuses the same way.
- Q9: Are commits already cherry-picked onto the base branch in scope?
- A9: No, chosen by the operator on 2026-10-03: recorded as Q17 of the thread `fixup-review-20261002` instead.
- Q10: How is the Windows half of `c5-no-regression` settled, given that CI runs only on `main` and on pull requests?
- A10: By the PR's CI run, decided by the operator on 2026-10-03: any failures are dealt with after the PR is created and CI has run.
