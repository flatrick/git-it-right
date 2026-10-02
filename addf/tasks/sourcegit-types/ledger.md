# Ledger thread — `sourcegit-types-20261002`

Shaping GitHub issue #4, support for SourceGit's Conventional Commit type definition file (JSON), from a conversation with the operator on 2026-10-02.

Append-only: never edit or delete a prior entry; append new ones at the end.
Number each entry by its position among the questions, `Q1` being the first; its answer takes the same number, as a separate list item.
Settle an entry with a numbered disposition line, `D1` being the first, that names the entries it settles.
See `skills/work-control.md`'s Ledger section for the full contract, including how a Task takes entries and when the thread is archived.

- Q1: What does issue #4 ask for?
- A1: That gir read the same JSON file SourceGit uses to define Conventional Commit types, so an agent, git on the CLI and a GUI client follow the same types.
  The file is an array of objects with `Name`, `Type`, `Description` and `PrefillShortDesc`; the issue says `PrefillShortDesc` need not be supported.
- Q2: Where is the file's location configured?
- A2: In a setting, `gir.typesFile`, so one file can serve every repository or a repository can have its own.
  The operator chose: readable from git config (system, global, repository) as well as `.girconfig`, the `.girconfig` value winning.
- Q3: How do the file's types combine with gir's?
- A3: The operator first said the file adds types gir does not know and overrules anything gir sets, with aliases staying in gir's own config.
  Asked to choose, the operator chose that the file replaces the type list: when set, only its types are allowed.
- Q4: What happens when the file is invalid?
- A4: gir refuses to do anything, naming the file and why it is invalid.
  The operator chose strict validation of the fields gir reads, ignoring `PrefillShortDesc` and unknown fields.
- Q5: What does the `Name` field do?
- A5: Nothing yet; it is the human-friendly label a future type picker will show.
  gir must still read and keep it, not discard it.
- Q6: What if `.girconfig` sets `gir.types` and a types file is also configured?
- A6: The types file wins and gir prints a warning.
- Q7: Which fields must each entry have?
- A7: `Type`, `Name` and `Description`.
- Q8: What happens to an alias whose target type the file leaves out?
- A8: The operator answered that a valid file dictates what is allowed.
  The agent's reading, not yet confirmed: such an alias stops applying, so the aliased type is rejected like any unknown type.
- Q9: Should the work happen in an isolated worktree and branch?
- A9: Yes, a new worktree on a new branch.
