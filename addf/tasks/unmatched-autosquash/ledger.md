# Ledger thread — `unmatched-autosquash-20261003`

Why the `fixup-unsquashed` hint can loop when an autosquash commit matches no target, found during a phase-0 investigation of WIP commits on 2026-10-03.

Append-only: never edit or delete a prior entry; append new ones at the end.
Number each entry by its position among the questions, `Q1` being the first; its answer takes the same number, as a separate list item.
Settle an entry with a numbered disposition line, `D1` being the first, that names the entries it settles.
See `skills/work-control.md`'s Ledger section for the full contract, including how a Task takes entries and when the thread is archived.

The probe ran with gir 0.2.1 and git 2.56.0 in a throwaway repository.
Its script and log are local only and not shared, so the repro is copied in A1.

- Q1: Does the `fixup-unsquashed` hint, `git rebase --autosquash <base>`, fold an autosquash commit whose subject matches no commit on the branch?
- A1: No, reproduced.
  On a branch `main..feature` of `feat: add a`, `squash! wip`, `squash! wip`, the command `GIT_SEQUENCE_EDITOR=true git rebase --autosquash main` prints `Successfully rebased and updated refs/heads/feature.` and exits 0.
  Afterwards the branch is unchanged, with both `squash! wip` commits still on it.
  `gir lint --range main..HEAD` then rejects both as `fixup-unsquashed` with the same `try: git rebase --autosquash <base>` hint.
  Following the hint again does nothing, so the user loops between the rejection and a rebase that reports success.
- Q2: Which realistic workflows produce an autosquash commit with no matching target?
- A2: OPEN.
  Unverified candidates:
  - The target was reworded (`amend!`, `reword`) after the fixup was made.
  - The target was dropped or split in a rebase.
  - The subject was written by hand (`squash! wip`) rather than by `gir fixup` or `git commit --fixup`.
  - The fixup's target is already on the base branch, outside `<base>..HEAD`.
- Q3: What would make the fixup workflow reliable when a target cannot be matched?
- A3: OPEN.
  Unverified ideas:
  - `gir lint --range` and pre-push detect an unmatched autosquash commit and reject it with a distinct rule id and its own instructions.
  - Make `gir fixup`, `amend` and `squash` produce subjects that survive a reword. Git's autosquash also matches a commit hash after the prefix, but whether git 2.56.0 accepts that form needs a probe.
  - Explain text for `fixup-unsquashed` says that an unmatched commit survives the rebase, and how to point it at its target.
