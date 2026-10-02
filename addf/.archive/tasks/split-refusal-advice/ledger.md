# Ledger — split-refusal-advice

## Taken from the thread `fixup-review-20261002`

Entries Q5, Q6, Q13 of `ledger/fixup-review-20261002.md`, copied unchanged.

- Q: What does `--split` do when some staged change cannot be traced, such as a new file?
  A: It is ignored, reproduced.
  At `src/cmd/fixup.rs:82`, with `GIR_INTERACTIVE=1` and changes to a.txt (commit A), b.txt (commit B) and a new file, picking 1 gives one `fixup! feat: b` holding all three.
  Without a terminal, the error says `pass one: gir fixup COMMIT`, but `gir fixup --split COMMIT` is refused with `--split finds each commit itself; drop the commit argument`.
- Q: Which commit does a pure insertion between lines from two different commits belong to?
  A: Both, reproduced, so it cannot be split.
  At `src/cmd/fixup/split.rs:10`, inserting `mid` between `one` (commit 1) and `two` (commit 2) makes the picker offer both commits and `s) split`.
  Choosing `s` gives `f:1 spans several commits; split it with git add -p`, which cannot split one inserted line, and blocks the split for every other file too.
  Suggested fix: attribute the insertion to one neighbour, or advise passing the commit, and do not offer `s` when split will refuse.
- Q: In what order, and in which Tasks, are the open *NIX issues fixed?
  A: Decided by the operator on 2026-10-02: Q11 first, as its own Task; then Q3 and Q4 together (the split patch reaching git exactly as staged); then Q5 and Q6 together (split refusals and advice that cannot work). Each Task is created when its turn comes.

## This Task

- Q: Which entries does this Task take?
  A: Q5 and Q6, the last pair in the order the operator set (Q13, also taken, since it is settled once this Task starts).
- Q: Without a terminal, what should `--split` do with a file it cannot trace?
  A: Refuse with advice that works: nothing committed, name the file, say to commit or unstage it and run `--split` again.
- Q: At a terminal, what should `--split` do with a file it cannot trace?
  A: Ask which commit that file belongs to, put its staged version in that commit's autosquash commit, and split the rest as usual.
- Q: What should happen to an insertion between lines from two different commits?
  A: At a terminal, ask which of the two commits it belongs to; without one, refuse naming the hunk and both commits, without `git add -p` advice.
- Q: Isolated workspace and line of development?
  A: No new one: worktree `.worktrees/fixup-modes`, branch `feat/fixup-modes`.
- Q: Does this Task overlap another active Task?
  A: No other Task is active.
- Q: Are the objective and success criteria viable and desirable?
  A: Agreed by the operator, as written in `TASK.md`.
