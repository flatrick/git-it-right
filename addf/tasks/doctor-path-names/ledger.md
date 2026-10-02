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
- Q: Should `windows-names` flag control characters (bytes 1 to 31)?
  A: Yes, added to `windows-names`.
- Q: Should `windows-names` flag names whose bytes are not valid UTF-8?
  A: Yes; the Windows side stays unverified for the operator's Windows testing.
- Q: How should a report show a name with a control character or bytes that are not UTF-8?
  A: In git's quoted form, for those names only; other names, including ones with `"` or `\`, are shown as they are.
