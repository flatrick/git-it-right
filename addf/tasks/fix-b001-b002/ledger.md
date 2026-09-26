# Ledger

Append-only record of questions and answers arising before a Task exists, or
before its name and objective are settled. Never edit or delete a prior
entry; append new ones at the end. See `skills/work-control.md`'s Ledger
section for the full contract, including its handoff into a new Task's
bundle once one starts.

- Q: Should the BUGS.md fixes happen in an isolated workspace and line of development?
  A: Yes (operator request, 2026-09-26). Worktree `../git-it-right-bugfix`, branch `fix/b001-b002`, from `main` at `839a991`.
- Q: Does fixing B-002 change the current specification?
  A: Yes. `spec/doctor.md#req-doctor-gitattributes-missing` and `#req-doctor-gitattributes-autocrlf` say "cannot be read" means missing; the fix narrows that to "does not exist" and needs a new requirement for an unreadable file.
- Q: Does fixing B-001 change the current specification?
  A: Yes. `spec/cli.md` has no requirement for a closed stdout; the fix adds one.
- Q: What exit code and output should gir use when stdout closes early (B-001)?
  A: OPEN
- Q: How should `gir doctor` report a `.gitattributes` that exists but cannot be read (B-002)?
  A: OPEN

<!--
- Q: <question>
  A: <answer, or OPEN if still unresolved>
-->
- Q: What exit code and output should gir use when stdout closes early (B-001)? (answer to the OPEN entry above)
  A: Print nothing and exit `141` on every OS (operator, 2026-09-26).
- Q: How should `gir doctor` report a `.gitattributes` that exists but cannot be read (B-002)? (answer to the OPEN entry above)
  A: A `warn  .gitattributes: cannot read: <os error>` line with no fix; other checks still run (operator, 2026-09-26).
- Q: Are the objective, scope and success criteria agreed?
  A: Yes (operator, 2026-09-26): both bugs on one branch, failing test before each fix, spec deltas, Linux verified here and Windows deferred to the operator.
