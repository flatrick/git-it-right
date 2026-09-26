# Ledger — init-adopted-repo

Pre-Task and DEFINE questions with the operator's answers.
The root `LEDGER.md` held no entries when this Task started.

- Q: Where did this Task come from?
  A: A `/rubberduck` session (main checkout, `.agents/rubberduck/contributor-onboarding/2026-09-26-181810.md`, untracked).
  It found that `gir init` recreates every missing template file, so a maintainer's committed deletion of `cliff.toml` is undone by each contributor's `gir init`.
  The operator decided `gir init` SHALL also generate `GIT-IT-RIGHT.md`, and that maintainers remove optional files they don't want.
- Q: Isolate the work in a new worktree and branch?
  A: Yes (operator's request). Worktree `.worktrees/init-activate-only`, branch `feat/init-activate-only`, from `main` at `04f9cee`.
- Q: Scope — only the skip-when-adopted fix, or also generating `GIT-IT-RIGHT.md`?
  A: Both, in this Task.
- Q: What signal says a repository has already adopted gir?
  A: `.girconfig` is in the index (`git ls-files .girconfig`), which covers committed and staged files and an unborn `HEAD`.
- Q: In an adopted repository, does `gir init` restore missing required files (`.githooks/*`, `.girconfig`)?
  A: Yes. Only optional files (`cliff.toml`, `GIT-IT-RIGHT.md`) are skipped.
- Q: When an optional file is skipped, does `gir init` say so?
  A: No, it is silent. `--force` restores it.
- Q: `--force` also overwrites an edited `.girconfig` and hooks; a maintainer who only wants `cliff.toml` back would lose custom types. Accept?
  A: No. Add a narrower flag.
- Q: Name and behavior of the narrower flag?
  A: `gir init --optional`: a normal init that also writes each missing optional file and never overwrites an existing file.
- Q: What does `GIT-IT-RIGHT.md` contain?
  A: What gir is and why; a contributor quick-start (install, `gir init` once per clone, `gir explain <rule>` on rejection); a git-cliff section (why `cliff.toml` exists, optional and for maintainers, the two commands, deleting it is fine); a no-CI maintainer check (`gir lint --range main..<branch>` before merging) plus the CI one-liner.
- Q: Static text, or generated from `.girconfig`?
  A: Static; it points at `gir explain types` for the live list.
- Q: Should this repository get its own `GIT-IT-RIGHT.md` in this Task?
  A: Yes, by running the new `gir init` here. Because `.girconfig` is tracked here, that goes through `gir init --optional`.
