# Ledger handoff

Taken in full from `framework:ledger/sourcegit-commit-template.md`.

- Q1: Does the existing file need a different JSON format to work with SourceGit?
- A1: No. The user clarified that its current format is already supported by SourceGit; preserve that format.
- Q2: Should the generated file use built-in types or mirror project-specific configured types?
- A2: Use gir's built-in 11 types, regardless of custom `.girconfig` types.
- Q3: Where should this Task be implemented?
- A3: In a new isolated worktree and branch.
