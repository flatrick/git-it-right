# Ledger - public CI safeguards

- Q: Where should this Task run?
  A: The operator chose the current worktree and branch, `.worktrees/fixup-modes` on `feat/fixup-modes`, on 2026-10-02.
- Q: Which fork PRs should require approval before workflows run?
  A: The operator chose every external PR, rather than only first-time contributors, on 2026-10-02.
- Q: Should this Task enable Actions after the safeguards are verified?
  A: OPEN. The operator was asked to confirm the scope and choose whether to enable CI or leave it disabled.
- D: Q3 -> deferred to `doctor-fix-file-mode`; this Task leaves Actions disabled. The operator's original request puts security before enabling CI, and the choice does not block the protection work.
- Q: Should Actions be enabled after the safeguards are verified?
  A: Yes. The operator explicitly confirmed the objective and success criteria and chose to enable CI afterward on 2026-10-02. This supersedes Q3's provisional disposition.
- D: Q3 -> taken by `public-ci-safeguards` as answered by Q4.
