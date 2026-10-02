# Commit message cheatsheet

A header is `<type>[(scope)][!]: <description>`.
Pick the type by asking what the change does for someone who uses the result, not by which files it touches.
Every section here is also available offline: `gir explain <type>` prints it, and `gir explain types` lists the types allowed in this repository.

## feat
Something users can do now that they could not do before.

Use it for:
- a new command, endpoint, screen, option or setting
- a new output or input format
- extending existing behavior in a way users can notice and rely on

Not for: making existing behavior match what was intended (`fix`), or internal changes nobody outside the code can observe (`refactor`).

Examples:
- `feat(cli): add --dry-run to the sync command`
- `feat: export reports as CSV`
- `feat(auth)!: require MFA for admin accounts`

## fix
Behavior that was wrong and is now right.

Use it for:
- crashes, hangs, wrong output, lost or corrupted data
- behavior that contradicts the docs, the spec or the obvious intent
- regressions introduced by an earlier commit

Not for: repairing a broken test (`test`), a broken pipeline (`ci`) or a typo in the docs (`docs`).

Examples:
- `fix(parser): accept tabs between key and value`
- `fix: stop the scheduler from running a job twice`
- `fix(ui): return focus to the dialog after a tooltip closes`

## perf
Same results, fewer resources: time, memory, disk, network.

Use it for:
- faster algorithms, queries or data structures
- caching, batching, streaming or lazy loading
- removing work that was done more often than needed

Put the measurement (before and after) in the body; a `perf` commit without numbers is a guess.

Examples:
- `perf(search): index titles to avoid full table scans`
- `perf: stream large uploads instead of buffering them`
- `perf(render): reuse row elements while scrolling`

## refactor
Code has a new shape; behavior is unchanged.

Use it for:
- moving, splitting, merging or renaming code
- extracting shared logic, removing duplication
- stricter types or null handling that changes no observable result
- deleting dead code

Not for: whitespace or formatting only (`style`), or anything a user could notice (`feat` or `fix`).

Examples:
- `refactor(billing): split the invoice builder into steps`
- `refactor: use the shared retry helper in the importer`
- `refactor(db): rename conn to connection`

## style
Formatting of source code only; the compiled or interpreted result is identical.

Use it for:
- running a formatter or a linter's automatic fixes
- whitespace, line wrapping, import order, quote style

Not for: visual styling of a UI such as colors or layout, which is `feat` or `fix` depending on intent.
"Style" here means code style.

Examples:
- `style: apply rustfmt to the workspace`
- `style(api): sort imports`
- `style: wrap long lines at 100 columns`

## test
Tests and the things only tests use.

Use it for:
- new or changed unit, integration or end-to-end tests
- fixtures, test data and test helpers
- fixing flaky or broken tests

Examples:
- `test(fixup): cover insertions at the start of a file`
- `test: replace sleeps with polling in the e2e suite`
- `test(api): add fixtures for expired tokens`

## docs
Documentation, in or out of the code.

Use it for:
- README files, guides, tutorials, decision records
- doc comments and code comments
- hand-written changelog or release notes text

Examples:
- `docs: explain how to install on macOS`
- `docs(api): describe the rate limit headers`
- `docs: fix broken links in the contributing guide`

## build
How the software is built, packaged or what it depends on at runtime.

Use it for:
- build scripts, compiler, bundler or linker settings
- container images and packaging
- adding, removing or upgrading dependencies that ship with the product

Not for: tools only developers use (`chore`), or pipeline definitions (`ci`).

Examples:
- `build(deps): bump serde to 1.0.210`
- `build: produce static binaries on Linux`
- `build(image): run the service as a non-root user`

## ci
The automation that checks, builds or releases on every push.

Use it for:
- pipeline and workflow definitions
- jobs, runners, caching and required checks
- release automation triggered by CI

Examples:
- `ci: run the tests on macOS and Windows`
- `ci: cache the cargo registry between runs`
- `ci(release): publish binaries when a tag is pushed`

## chore
Housekeeping that changes nothing in what ships.

Use it for:
- `.gitignore`, `.editorconfig`, editor and repository settings
- developer-only tools, scripts and their dependencies
- moving or renaming files that are not code

Not for: anything that ships; if users could be affected, another type fits better.

Examples:
- `chore: ignore editor swap files`
- `chore(deps-dev): bump the test runner to v3`
- `chore: add a script that resets the local database`

## revert
Undoes an earlier commit completely.

Create it with `git revert <commit>`, then reword the header to `revert: <original header>` and keep git's `This reverts commit <sha>.` line in the body:

```text
revert: feat(cli): add --dry-run to the sync command

This reverts commit 3f2a9c1d.
```

gir also accepts git's default `Revert "..."` header unless `gir.allowRevert = false`.

Examples:
- `revert: feat(cli): add --dry-run to the sync command`
- `revert: perf(search): index titles to avoid full table scans`

## breaking
Any type can carry a breaking change: something existing users must change on their side.
Mark it with `!` before the colon, a `BREAKING CHANGE:` footer, or both:

```text
feat(config)!: read settings from TOML instead of INI

BREAKING CHANGE: convert settings.ini with `app migrate-config`.
```

## scopes
A scope names the part of the project a commit touches: `fix(parser): ...`.
Keep them short nouns, reuse the same words, and leave the scope out when a change is cross-cutting.
`gir.scopes` in `.girconfig` can restrict the allowed list.

## fixup
Correcting a commit that is on your branch but not pushed yet, without an interactive rebase.
Pick the row that matches what should happen to the earlier commit:

| You want to | Commit with | After `git rebase --autosquash` |
|---|---|---|
| add forgotten changes, keep its message | `gir fixup` or `git commit --fixup=<commit>` | changes folded in, the `fixup!` message disappears |
| add changes and write a new message | `gir amend` or `git commit --fixup=amend:<commit>` | changes folded in, your `amend!` message replaces the old one |
| only change its message | `gir reword <commit>` or `git commit --fixup=reword:<commit>` | message replaced, content untouched |
| add changes and merge both messages | `gir squash` or `git commit --squash=<commit>` | changes folded in, git asks you to edit the combined message |

Then fold everything in:

```sh
git rebase --autosquash main
```

- `gir fixup`, `gir amend` and `gir squash` find the target themselves by looking at which commit last changed the staged lines.
  When gir finds the target itself, it refuses lines that belong to several commits, come from the base branch (`origin/HEAD`, `main`, `master` or the upstream), or are in a new file.
  Each accepts a target on the current branch after the base: `gir amend <commit>`.
  `gir reword` has no staged lines to trace, so it needs a commit and ignores anything staged.
- When the staged lines belong to several earlier commits, `--split` creates one commit per target, each with only its own lines.
  Or stage one target at a time with `git add -p`.
- In a terminal, gir asks instead of refusing: type a listed number, `s` to split, or press Enter to cancel.
  GUI clients, CI and scripts have no terminal and get the refusal; `GIR_INTERACTIVE=0` or `1` overrides the check.
- The `pre-push` hook rejects `fixup!`, `amend!` and `squash!` commits, so they cannot reach the remote by accident.
- `rebase.autoSquash = true` (set by `gir doctor --fix`) applies the same folding to every `git rebase -i`.
