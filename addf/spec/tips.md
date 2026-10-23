# gir tips

## Subspecifications

`NONE`.

## Invocation and scope

<a id="req-tips-invocation"></a>
**invocation.** `gir tips` with no positional arguments or options SHALL print the complete Git settings catalog to stdout and exit `0`. It SHALL work inside or outside a Git repository. The common `-h` and `--help` behavior applies.

<a id="req-tips-read-only"></a>
**read-only.** `gir tips` SHALL NOT invoke Git, inspect repository state, or read or write Git configuration. It SHALL print suggestions only; it SHALL NOT apply any setting.

## Catalog

<a id="req-tips-order"></a>
**order.** The catalog SHALL list these presentation and editor settings first, in this order:

- `diff.colorMoved=plain`
- `branch.sort=-committerdate`
- `tag.sort=version:refname`
- `column.ui=auto`
- `diff.mnemonicPrefix=true`
- `diff.context=10`
- `commit.verbose=true`

It SHALL list these behavior-changing settings afterward, in this order:

- `help.autoCorrect=prompt`
- `rebase.missingCommitsCheck=error`
- `user.useConfigOnly=true`
- `fetch.all=true`
- `push.followTags=true`

<a id="req-tips-explanations"></a>
**explanations.** Each catalog entry SHALL print a `git config --global KEY VALUE` example, an `Effect` that describes what the setting changes, and a `Useful when` sentence that names a matching situation. The introduction SHALL state that `gir tips` changes nothing and that repository-local values can override the global examples.

<a id="req-tips-behavior-disclosures"></a>
**behavior-disclosures.** Each behavior-changing entry SHALL explain its effect and when it fits. The entries SHALL include these details:

- `help.autoCorrect=prompt` asks before running a suggested correction after a mistyped command. Accepting the suggestion runs it.
- `rebase.missingCommitsCheck=error` stops an interactive rebase when a commit disappears from its todo list. An explicit `drop` remains allowed.
- `user.useConfigOnly=true` stops Git from guessing author or committer identity. Commits fail until the required identity is configured.
- `fetch.all=true` makes a bare fetch update configured remotes except those with `remote.<name>.skipFetchAll=true`. It may add network requests and time.
- `push.followTags=true` also pushes eligible missing annotated tags reachable from the refs being pushed, without requiring the user to name those tags. Lightweight tags are excluded.

<a id="req-tips-presentation-disclosures"></a>
**presentation-disclosures.** Each presentation and editor entry SHALL explain its display or commit-editor effect and when that presentation is useful. The `branch.sort=-committerdate` entry SHALL say that it sorts by commit date, not branch creation date. The `diff.context=10` entry SHALL say that it shows more context and makes diffs longer. The `commit.verbose=true` entry SHALL say that the diff appears in the commit-message editor but is not saved in the commit message.

## Module invariants

- This file is reachable from `SPEC.md` through one ordered parent link.
- This file contains current requirements only.
- This file does not require archived material to define current behavior.
