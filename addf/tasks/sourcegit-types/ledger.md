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
- Q10: Does an alias whose target the file leaves out stop applying, or is it a configuration error?
- A10: It stops applying; the aliased type is rejected like any unknown type. This confirms the agent's reading in A8.
- Q11: How does a relative `gir.typesFile` resolve?
- A11: Against the directory of the file that set it, as git resolves `include.path`.
  `~/` expands to the home directory, and an empty value in `.girconfig` turns off a value from git config.
- Q12: Do `gir explain` and `gir doctor` tolerate an invalid types file?
- A12: Yes, as they tolerate an invalid `.girconfig` today: `explain` falls back to the defaults and `doctor` prints a `warn` line; `lint`, `hook`, `fixup` and `init` refuse.
- Q13: What does `gir init` write as `types` when a types file is configured?
- A13: The `types` line commented out, so a new repository does not trigger the warning from A6.
- Q14: Which `Type` values are valid?
- A14: Nonempty, only ASCII letters, digits and `-`.
- Q15: Do the Objective and Success criteria in `TASK.md` describe the desired outcome?
- A15: Yes, the operator agreed and DEFINE may close.
- Q16: Which JSON parser does gir use?
- A16: `serde_json`, parsed to `serde_json::Value` and validated by hand, without `serde_derive`.
  Measured before asking: four transitive crates and about 38 KB on a release build, with line and column in syntax errors.
- Q17: What does a relative `gir.typesFile` set with `git -c` resolve against?
- A17: The repository root, as a `.girconfig` value does.
