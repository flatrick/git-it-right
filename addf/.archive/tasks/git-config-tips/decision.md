# Decision — `gir tips`

## Selected design

Use Candidate 1's private catalog in `src/cmd/tips.rs`. Its `Tip` records a
setting key, value, effect, and useful situation. `Catalog` has separate
presentation and behavior slices, rendered in that order. `page() -> String`
owns the copy and formatting; the existing CLI dispatcher validates the
zero-argument invocation and writes the result. The command does not inspect
the repository or Git configuration.

The safe-first contract is visible in the catalog shape. Every behavior entry
has both an effect and a use case. The command prints global `git config`
examples for developers to choose from, states that it changes nothing, and
notes that repository-local config can override global values.

The initial catalog contains twelve settings, in this order:

1. Presentation and editor context: `diff.colorMoved=plain`,
   `branch.sort=-committerdate`, `tag.sort=version:refname`, `column.ui=auto`,
   `diff.mnemonicPrefix=true`, `diff.context=10`, and `commit.verbose=true`.
2. Workflow and publishing choices: `help.autoCorrect=prompt`,
   `rebase.missingCommitsCheck=error`, `user.useConfigOnly=true`,
   `fetch.all=true`, and `push.followTags=true`.

Behavior descriptions will explain that `help.autoCorrect=prompt` executes a
suggested command only after confirmation; `rebase.missingCommitsCheck=error`
stops an interactive rebase when a todo entry disappears, while explicit
`drop` remains valid; `user.useConfigOnly=true` makes commits fail until the
identity is configured; `fetch.all=true` can fetch more remotes and make a
fetch take longer; and `push.followTags=true` can publish eligible annotated
tags without naming them explicitly. Each description will name when the
setting fits.

Add `diff.colorMoved=plain` to doctor's existing local recommendation list.
Reuse its effective-value check and local-only fix path, which preserves any
existing nonempty value. Keep the intentional overlap between global advice
and a repository-local doctor check; a shared registry would conflate their
different scope and mutation policies.

## Arena synthesis

Candidate 1 is the base. It scored 25/25 in the cross-judge review and has the
clearest boundary: catalog and formatting stay private, and the dispatcher
owns argument validation and output. Candidate 2 also uses two typed groups,
but writes stdout from the command and has less precise fetch and push
disclosures. Candidate 3 uses one list and an effect enum, but relies on
comments to keep presentation entries first and includes `pull.rebase`, a
team history policy excluded by the survey.

Carry two details from the other candidates into acceptance criteria: test
that tips rejects selectors and flags, and state that printed commands are
examples a person chooses to run. Retain the candidates' doctor checks for
unset, existing local, and inherited global values. No structural graft is
needed because all candidates agree on a dedicated, static, read-only command.

The design-red-flag screen found no shallow public module, pass-through layer,
split state writer, or competing command path. The single repeated key
`diff.colorMoved` is deliberate: tips teaches a global preference, while
doctor recommends a local repository value.

## Rejected catalog choices

- `pull.rebase=true` changes history behavior and can conflict with a team's
  merge policy, so it does not fit a general first catalog.
- `fetch.pruneTags=true` can delete local tags, and `rerere.autoUpdate=true`
  can stage remembered conflict resolutions. Both have higher impact than
  the initial catalog warrants.
- Platform-sensitive performance settings are deferred because their value
  depends on Git version, operating system, filesystem, and repository size.
- Settings already recommended by doctor are omitted, except for the
  explicitly requested `diff.colorMoved` entry.
- Profiles and selectors are omitted. `gir tips` prints the short complete
  catalog so users can compare choices before applying one.

## Scope and verification

Update the CLI, doctor, and new tips specifications at the successful terminal
checkpoint. Update the README and command usage during implementation. Test
the full catalog outside a repository, ordering and disclosures, rejected
arguments, and read-only behavior. Test doctor's unset, fix, existing-value,
and inherited-global cases. Run the real binary with Git unavailable on PATH
to prove that tips does not call Git.

The local environment has Git 2.56.0 on Linux. Other Git versions and
platforms are not available for direct verification; platform-sensitive
settings are excluded from this catalog.

## Alternatives

- An embedded Markdown page is easier to edit, but it cannot encode safe-first
  grouping or required effect and use-case fields.
- A shared doctor/tips registry would require scope and mutation metadata to
  make one of its consumers understand the other's policy.
- Profiles would add parsing and selection behavior without improving the
  initial short catalog.
