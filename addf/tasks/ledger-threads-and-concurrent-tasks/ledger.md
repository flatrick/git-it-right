# Ledger — ledger-threads-and-concurrent-tasks

Questions raised while shaping this Task, with the operator's answers (2026-10-02).

## Moved from the root `LEDGER.md`

The root `LEDGER.md` held these entries when the Task started; they are moved here unchanged, as Work control's handoff requires.
They are not about this Task: success criterion `c5-findings-thread` moves them on into their own thread.

Entries 1–10 come from a `/code-review high` of `feat/fixup-modes` at `d246ab2`, run on 2026-10-02 against `target/debug/gir` with git 2.56.0 in throwaway repos.
Its repro scripts and logs are kept locally in `.scratch/code-review-edgecases-20261002-0933/` (gitignored, so not shared).
For each one, whether to fix it on this branch is still OPEN.

- Q: Does `gir fixup --split` leave sparse-checkout, `skip-worktree` and intent-to-add index entries intact?
  A: No, reproduced.
  `src/cmd/fixup/split.rs:45` runs `git read-tree ORIG` on every round and on restore, which rebuilds the whole index.
  After a split, files outside the sparse set show as ` D`, a `skip-worktree` file with a local edit shows as ` M`, and a `git add -N` file becomes `??`.
  A later `git add -A` or `git commit -a` would then delete or commit those files.
  Suggested fix: build each round in a temporary index (`GIT_INDEX_FILE`, seeded from orig) and never touch the real index.
- Q: Does `parse_hunks` handle a deleted line whose content starts with `-- `?
  A: No, reproduced.
  `src/cmd/fixup.rs:301` matches `--- ` on every line, so the deleted line `-- header` (Lua, SQL or Haskell comment) is read as a new file header.
  `gir fixup` then reports `cannot tell which commit header:5 belongs to`.
  A deleted `-- /dev/null` line makes every later hunk in that file skipped silently, and with several commits the fixup can go to the wrong commit with no error.
  Suggested fix: only match `---`/`+++` while still inside the diff header.
- Q: Does `--split` rebuild a patch that matches the bytes in the index?
  A: No, reproduced.
  `src/cmd/fixup.rs:292` builds it from `diff.lines()`, which drops `\r`, and `git::run` uses `from_utf8_lossy` and trims the output.
  A CRLF file (`* -text`) and a Latin-1 file both fail with `patch does not apply`.
  A staged last line ending in spaces fails with `the split commits do not add up to the staged changes`.
  HEAD and the index are restored each time.
  Suggested fix: split raw bytes with `split_inclusive('\n')`, through a git runner that does not trim.
- Q: Is `--split` affected by the user's `apply.whitespace` setting?
  A: Yes, reproduced.
  `git apply --cached` at `src/cmd/fixup/split.rs:46` follows it.
  With `error`, a staged line with trailing whitespace aborts the split.
  With `fix`, apply strips the whitespace, the round has nothing to commit, and the user sees `no changes added to commit` and then `git commit --fixup=... failed`.
  Suggested fix: pass `--whitespace=nowarn`, and consider `--no-textconv` on the `diff --cached` call.
- Q: What does `--split` do when some staged change cannot be traced, such as a new file?
  A: It is ignored, reproduced.
  At `src/cmd/fixup.rs:82`, with `GIR_INTERACTIVE=1` and changes to a.txt (commit A), b.txt (commit B) and a new file, picking 1 gives one `fixup! feat: b` holding all three.
  Without a terminal, the error says `pass one: gir fixup COMMIT`, but `gir fixup --split COMMIT` is refused with `--split finds each commit itself; drop the commit argument`.
- Q: Which commit does a pure insertion between lines from two different commits belong to?
  A: Both, reproduced, so it cannot be split.
  At `src/cmd/fixup/split.rs:10`, inserting `mid` between `one` (commit 1) and `two` (commit 2) makes the picker offer both commits and `s) split`.
  Choosing `s` gives `f:1 spans several commits; split it with git add -p`, which cannot split one inserted line, and blocks the split for every other file too.
  Suggested fix: attribute the insertion to one neighbour, or advise passing the commit, and do not offer `s` when split will refuse.
- Q: Can the commit picker offer merge commits as a fixup target?
  A: Yes, reproduced up to the picker list.
  `pick_branch_commit` at `src/cmd/fixup.rs:158` lists `rev-list base..HEAD`, so after `git merge --no-ff side` it lists `Merge side` and the side branch's commits.
  That `fixup! Merge side` cannot be folded by a default `git rebase --autosquash` is inferred, not run.
  Suggested fix: add `--no-merges --first-parent`.
- Q: Does `--split` throw away the index stat cache?
  A: Likely, inferred from how `read-tree` works, not measured.
  Each round's full `read-tree` at `src/cmd/fixup/split.rs:45` would make the next `git commit` and `git status` re-hash every tracked file, about N+1 times for N targets.
  The temporary index from the first entry would also fix this.
- Q: How many git processes does the picker start to show its list?
  A: About 21 for 20 commits, read from the code.
  `subject(sha)` at `src/cmd/fixup.rs:162` runs `git log -1` once per commit, and split prints subjects the same way.
  Suggested fix: one `git log --max-count=20 --format='%H %s' RANGE` call, with `targets` carrying the subjects.
- Q: Does the branch's new Markdown use semantic line breaks?
  A: No, read from the files.
  `ROADMAP.md` lines 6, 7 and 40 and many lines of `CLIENT-TESTING.md` hold several sentences each.
  `addf/SELF-IMPROVEMENT/20260927T112030Z-one-commit-per-task-state-change.md` is wrapped at a fixed width.

## This Task

- Q: Which two findings should become Tasks, and how should the root Ledger hand off ten findings to two Tasks?
  A: OPEN for the findings. The operator judged that the root Ledger is a poor home for data about one Task, and that two Tasks at once is a case addf keeps running into, so addf needs an amendment first.
- Q: What is wrong with the root Ledger?
  A: It does three jobs: pre-Task exploration, a backlog of unscheduled findings, and state shared by every Task and worktree. Its all-or-nothing handoff cannot split one exploration across several Tasks or none, and one shared append-only file conflicts across worktrees.
- Q: Could the two fixes be one Task under the current rules instead?
  A: Yes, but the eight untasked findings would still have no home; that is the main reason for the amendment.
- Q: Where should pre-Task exploration live?
  A: Ledger threads: `ledger/<thread>.md`, one append-only file per exploration. A Task copies the entries it takes, and the thread records which Task took each.
- Q: Where should findings nobody is working on yet live?
  A: As open entries in their thread, until each is taken by a Task, rejected, or moved elsewhere; the thread is archived only when every entry is settled.
- Q: Which rules for several active Tasks should addf adopt?
  A: All four offered: ask which Task when resuming with more than one active; Tasks that overlap in spec modules or files declare it and their order; the Isolate question is asked per Task; and a cap on active Tasks.
- Q: What is the cap?
  A: 3.
- Q: Should the amendment come before the two fixes?
  A: Yes, as its own Task under the current rules; the fix Tasks then start under the new ones.
- Q: Isolated workspace and line of development?
  A: No new one: worktree `.worktrees/fixup-modes`, branch `feat/fixup-modes`. The earlier ledger commit `e3e3c3b` was made before this question was asked, which the Isolate rule did not allow.
- Q: Are the objective, scope and success criteria viable and desirable?
  A: Agreed by the operator, as written in `TASK.md`.
