# Ledger — `autosquash-review-fixes`

The review of branch `fix-unmatched-autosquash` (`3e10963..8465c1d`) that shaped this Task, and its DEFINE dialogue.
The review was a cold-context code review by a sub-agent on 2026-10-03, with experiments in throwaway repositories (scripts `exp.py` and `perf.py`, kept in the session scratchpad, not shared).
Finding 1 was reproduced independently in the parent session; the others are as the reviewer reported them.

Append-only: never edit or delete a prior entry; append new ones at the end.

- Q1: Does `fixup-unmatched` advise correctly when the target blame finds is the root commit inside the linted range?
- A1: No, reproduced twice. `git blame` prints root-commit lines with `^` and a 39-character hash, so `blame_at`'s result never equals the full ID in the list and the target is reported as already published.
  Repro: `feat: one` (root), `feat: two`, then `fixup! feat: uno` changing a line of `feat: one`; `gir lint --range HEAD` prints `target 4879170621 feat: one is already published` and `try: git rebase -i <base>: reword it into a normal commit`.
  It also affects pre-push on a repository's first push.
- Q2: Does gir read a commit's subject as git does when the first paragraph wraps onto several lines?
- A2: No, confirmed by the reviewer. git's subject is the first paragraph joined with spaces; `lint::title` takes the first line. `fixup! feat: one` / `two` against an older `feat: one three` is predicted to fold, and git leaves it (the old loop). A wrapped target title is the reverse case: git folds, gir says it will not.
- Q3: Does gir strip the prefix as git does when more than one whitespace character follows it?
- A3: No, confirmed by the reviewer. git skips any whitespace after `fixup!`; gir needs exactly one space, so `fixup!  feat: one` is reported as unmatched although git folds it.
- Q4: Can a merge commit in the range be a fold target?
- A4: Not for a plain `git rebase`, which drops merges from its todo; gir counts them, so `fixup! Merge branch 'side'` is predicted to fold and git leaves it. Confirmed by the reviewer.
- Q5: How expensive is the blame step?
- A5: One `git blame` per changed line: an unmatched fixup rewriting 3000 lines took 6.28 s in a 3-commit repository (0.01 s when it matched). Confirmed by the reviewer.
- Q6: Which specifiers resolve against the wrong commit?
- A6: Plausible, not tested: a rev specifier such as `HEAD~2` resolves against the checked-out HEAD, not the tip being pushed or linted.
- Q7: Are the published-target wording and the base hint right for `gir lint --range`?
- A7: For `lint --range`, "already published" really means "before the range", which may be unpushed. When the oldest linted commit is the root, the hint says `git rebase -i <base>`, which cannot be run; git needs `--root`.
- Q8: Is the `amend!` message check right for a wrapped `amend!` subject?
- A8: Plausible, not tested: the second line of a wrapped subject counts as a message body, so `fixup -C` could be advised for an `amend!` with no new message.
- Q9: Which findings does the operator want fixed, and where?
- A9: All of them (1–8) plus tests for each gap the review listed (root and merge in the range, wrapped subjects, extra whitespace, a large unmatched fixup, a first push), in a new Task on the same branch and worktree, before the branch is merged.
