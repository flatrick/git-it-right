# Ledger thread — `fixup-review-20261002`

Edge cases in `gir fixup` and `--split` found by a `/code-review high` of `feat/fixup-modes` on 2026-10-02.
Copied here unchanged from the `ledger.md` of the Task `ledger-threads-and-concurrent-tasks`, which received them only because the retired root Ledger handed off all its entries at once.

Append-only: never edit or delete a prior entry; append new ones at the end.
An entry is named by its position among the `Q:` entries, `Q1` being the first.
Settle an entry with a `D:` line that names it.
See `skills/work-control.md`'s Ledger section for the full contract, including how a Task takes entries and when the thread is archived.

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
- D: Q2 -> taken by Task diff-header-parsing
- D: Q1, Q8 -> taken by Task split-preserves-index
- Q: Can `gir fixup` trace a staged change to a file whose name contains a space?
  A: No, reproduced on `d246ab2` and on the fixed build (review script `edge.py` case 2; `tasks/fixup-fixes-acceptance/logs/edge-old.log` and `edge-new.log` while that Task is active).
  `gir fixup` reports `cannot tell which commit my file.txt belongs to`.
  For such a name git writes the `---`/`+++` header with a trailing tab (`--- a/my file.txt` then TAB), and `parse_hunks` keeps the tab in the path, so the path never matches.
  Found by the acceptance run of the header and split-index fixes; not caused by either fix.
- Q: Can an interrupted `gir fixup --split` leave its temporary index file behind?
  A: Likely, inferred from the code, not tested.
  `git::TempIndex` removes `gir-split-index-<pid>` in the git directory when it is dropped, which does not happen when the process is killed (for example Ctrl+C during a commit hook or editor).
  The leftover file is harmless to git but is never cleaned up.
  Found while verifying the Task `fixup-fixes-acceptance`.
- Q: In what order, and in which Tasks, are the open *NIX issues fixed?
  A: Decided by the operator on 2026-10-02: Q11 first, as its own Task; then Q3 and Q4 together (the split patch reaching git exactly as staged); then Q5 and Q6 together (split refusals and advice that cannot work). Each Task is created when its turn comes.
- D: Q11 -> taken by Task diff-path-names
- D: Q3, Q4 -> taken by Task split-patch-fidelity
- D: Q5, Q6, Q13 -> taken by Task split-refusal-advice
