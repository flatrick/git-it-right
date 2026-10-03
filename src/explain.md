## spec
Conventional Commits 1.0.0 (https://www.conventionalcommits.org/en/v1.0.0/):

    <type>[optional scope][!]: <description>

    [optional body]

    [optional footer(s)]

- `type` is a noun such as `feat` or `fix`; run `gir explain types` for the allowed list.
- `scope` is a noun in parentheses naming the area touched: `feat(parser): ...`.
- `!` before the colon, or a `BREAKING CHANGE: <text>` footer, marks a breaking change.
- The body starts one blank line after the header.
- Footers are `Token: value` or `Token #value`, one blank line after the body, for example `Refs: #123`.

If gir reports `[spec]`, the header passed gir's own rules but the whole message did not parse (usually a footer or body issue).
Run `gir lint --json` on the message to see the text that was checked.

## empty
The commit message is empty after comments are stripped.
Write a header: `<type>: <description>`.

## type-missing
The header does not start with `<type>[(scope)][!]: `.
Wrong:  `Update README.md`
Right:  `docs: update install steps in README`
Run `gir explain types` for the allowed types.

## type-case
The type was not lowercase (`Feat`), so gir lowercased it.
Automatic fix; nothing to do.

## type-alias
A common synonym (`feature`, `bugfix`, `doc`, ...) was mapped to its canonical type.
Aliases live in `.girconfig` under `[gir "alias"]`.

## type-unknown
The type is not in `gir.types`.
Pick one from `gir explain types`, or add the type to `gir.types` in `.girconfig` if the project really needs it.
gir never guesses a misspelled type for you, because a wrong guess changes meaning (and the changelog section).

## scope-empty
`feat(): x` had empty parentheses; gir removed them.

## scope-unknown
`gir.scopes` in `.girconfig` restricts the allowed scopes, and this one is not listed.
Use a listed scope, or omit the scope if the change is cross-cutting (unless `gir.scopeRequired` is set).
Several scopes: `feat(api,cli): ...`.

## scope-required
`gir.scopeRequired = true` in `.girconfig`: every header needs `(scope)`.

## header-spacing
gir normalised whitespace: `feat :x` and `feat ( api ):x` become `feat: x` and `feat(api): x`.

## header-length
The header is longer than `gir.subjectMax` (default 72).
gir cannot shorten it for you.
Keep the header to *what* changed; move *why* and *how* into the body:

    fix(auth): refresh token before expiry

    Tokens were refreshed only after a 401, which failed uploads
    that outlived the token.

## desc-empty
Nothing follows `<type>: `. Describe the change in imperative mood: `fix: handle empty input`.

## desc-period
The description ended with `.`; gir removed it. Headers are titles, not sentences.

## desc-case
`gir.descCase = lower`: gir lowercased the first letter of the description.
It skips words that look like acronyms (`API`).

## body-separator
The line after the header was not blank, so git would treat both lines as the subject.
gir inserted the blank line.

## breaking-footer
A breaking-change footer had the wrong case or spacing (`breaking change: x`).
gir rewrote it as `BREAKING CHANGE: x`. `BREAKING-CHANGE: x` is also valid and left alone.

## merge-commit
`gir.allowMerge = false`: merge commits are rejected.
Rebase onto the base branch instead: `git rebase <base>`.

## revert-commit
`gir.allowRevert = false`: git's default `Revert "..."` subject is rejected.
Reword it: `revert: <original description>`, and keep the `This reverts commit <sha>.` line in the body.

## fixup-unsquashed
A `fixup!`, `squash!` or `amend!` commit is about to be pushed.
Fold it into its target first: `git rebase --autosquash <base>`, then push again.
A commit that matches no earlier commit, or whose target is already published, is reported as `fixup-unmatched` instead: move it below its target in `git rebase -i <base>`, or reword it into a normal commit (`gir explain fixup-unmatched`).
The whole workflow: `gir explain fixup`.

## fixup-unmatched
A `fixup!`, `squash!` or `amend!` commit matches no earlier commit among those being pushed or linted, so `git rebase --autosquash` reports success and leaves it where it is.
git matches the text after the prefix against the title of an earlier commit (all of it, or its start), or resolves it as a commit ID; a fixup made before its target was reworded no longer matches.
When gir can tell which commit the change belongs to, `try:` names it: in `git rebase -i <base>`, move the line below that commit and change `pick` to `fixup` (`squash` for `squash!`, and `fixup -C` for an `amend!` that carries a new message).
When that commit is already published, folding into it rewrites published history: reword the commit into a normal Conventional Commit instead (`reword` in `git rebase -i <base>`).
Pushing it as it is stays your call: `git push --no-verify` skips the `pre-push` check.

## fix-pending
A recorded commit has a message the `commit-msg` hook would have fixed, so it was made with `--no-verify` or without the hook installed.
The `try:` line shows the fixed subject; the message lists every pending fix by rule id.
Reword the commit: `git commit --amend` for the last commit, or `git rebase -i <base>` and mark it `reword`.

## config
`.girconfig` at the repo root, git-config syntax (`git config --file .girconfig gir.subjectMax 100`):

    [gir]
        types = feat fix docs style refactor perf test build ci chore revert
        scopes = api cli            # empty = any scope allowed
        scopeRequired = false
        subjectMax = 72
        descCase = any              # or lower
        allowMerge = true
        allowRevert = true
        hookMissing = warn          # or fail: hooks block when gir is not installed
    [gir "alias"]
        feature = feat
        bugfix = fix

`gir.typesFile` names a SourceGit type definition file (JSON: `Name`, `Type`, `Description` per type).
Its types replace `types`, and an alias to a type it leaves out stops applying.
Set it in `.girconfig`, or in git config (`git config --global gir.typesFile ~/cc-types.json`) for every repository.
A relative path is relative to the file that sets it; an empty value in `.girconfig` turns off one from git config.
An invalid file stops `lint`, the hooks and `init` until it is fixed.

## hooks
`gir init` installs `.githooks/commit-msg` and `.githooks/pre-push` and sets `core.hooksPath = .githooks`.
Each clone needs `gir init` (or `git config core.hooksPath .githooks`) once, because git does not trust hook paths from a clone.
Once `.girconfig` is tracked, `gir init` does not recreate a deleted `cliff.toml` or `GIT-IT-RIGHT.md`; `gir init --optional` does, without overwriting edited files.
`git commit --no-verify` skips the hooks; `pre-push` and CI (`gir lint --range origin/main..HEAD`) catch those commits.

## doctor
`gir doctor` checks repo hygiene: `.gitattributes`, `.gitignore`, `.editorconfig`, recommended git config, and index problems (case collisions, Windows-reserved names, scripts without the executable bit).
`gir doctor --fix` applies only repo-local changes; it prints global suggestions without applying them.
