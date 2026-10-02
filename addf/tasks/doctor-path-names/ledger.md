# Ledger — doctor-path-names

No Ledger thread entries taken; the suspicion came up while fixing `diff-path-names` (2026-10-02), where `src/cmd/doctor.rs` was noticed reading `git ls-files` output without `-z`.

- Q: What should the Task do?
  A: Investigate each suspicion, then fix what is confirmed, in this Task.
- Q: Isolated workspace and line of development?
  A: No new one: worktree `.worktrees/fixup-modes`, branch `feat/fixup-modes`, to reuse this branch's byte-level git helpers.
- Q: Does this Task overlap another active Task?
  A: No other Task is active.
- Q: Are the objective and success criteria viable and desirable?
  A: Agreed by the operator, as written in `TASK.md`.
