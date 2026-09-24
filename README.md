# gir

`gir` keeps git usage honest for humans and coding agents alike.
It is one small Rust binary that works the same on Windows, Linux and macOS.

- **Conventional Commits lint.** Messages are checked against [Conventional Commits 1.0.0](https://www.conventionalcommits.org/en/v1.0.0/).
  Safe mistakes are fixed automatically; anything else is rejected with a short, actionable message.
- **Fixup workflow.** `gir fixup` finds which commit your staged change belongs to and creates a `fixup!` commit for `git rebase --autosquash`.
- **Repo hygiene.** `gir doctor` checks `.gitattributes`, `.gitignore`, `.editorconfig`, recommended git settings and file names that break on some platforms.

## Install

You need `git` and a Rust toolchain (`rustup`).

```sh
git clone git@github.com:flatrick/git-it-right.git
cd git-it-right
cargo install --path . --locked
```

This puts `gir` in `~/.cargo/bin`, which `rustup` adds to your `PATH`.
Check it with `gir --version`.

Rebuild with the same `cargo install` command after pulling changes; the hooks always run the `gir` on your `PATH`.

## Set up a repository

```sh
gir init      # hooks, .girconfig, cliff.toml, core.hooksPath
gir doctor    # report what else is missing
gir doctor --fix
```

`gir init` writes:

| File | Purpose |
|---|---|
| `.githooks/commit-msg` | Lints and fixes each commit message. |
| `.githooks/pre-push` | Blocks unsquashed `fixup!` commits and messages that skipped the hook. |
| `.girconfig` | Project settings (types, scopes, limits). |
| `cliff.toml` | [git-cliff](https://git-cliff.org) changelog config that matches your types. |

Commit these files.
`gir init` stages the hooks as executable and sets `core.hooksPath=.githooks` for your clone.

Git does not activate hooks from a fresh clone, so everyone who clones the repository runs `gir init` once (or `git config core.hooksPath .githooks`).
If `gir` is not installed, the hooks print a warning and let the commit through; set `gir.hookMissing = fail` in `.girconfig` to block instead.

## Writing commit messages

The format is `<type>[(scope)][!]: <description>`, an optional body after a blank line, and optional footers:

```text
feat(api)!: return 404 for unknown users

Previously the API returned 200 with an empty body.

BREAKING CHANGE: clients must handle 404.
Refs: #42
```

Run `gir explain types` for the allowed types and what each one means.

### What gets fixed automatically

The `commit-msg` hook rewrites these and tells you what it changed:

```text
$ git commit -m "Feature(api) :Add retry."
gir: fixed [header-spacing] Feature(api) :Add retry. -> Feature(api): Add retry.
gir: fixed [type-case] Feature -> feature
gir: fixed [type-alias] feature -> feat
gir: fixed [desc-period] Add retry. -> Add retry
```

- type casing (`Feat` → `feat`) and aliases (`feature` → `feat`, `bugfix` → `fix`)
- spacing around `(scope)`, `!` and `:`
- empty scope `()`
- a trailing period on the description
- a missing blank line between header and body
- `breaking change:` footers → `BREAKING CHANGE:`

### What gets rejected

Anything that needs a human (or agent) decision is rejected in at most three lines: what is wrong, an example, and where to read more.

```text
$ git commit -m "feet: add retry"
gir: commit rejected [type-unknown] `feet` is not an allowed type (did you mean `feat`?)
  try: feat: add retry   types: feat fix docs style refactor perf test build ci chore revert
  more: gir explain type-unknown
```

gir never guesses a misspelled type for you, because a wrong guess changes the meaning and the changelog section.
`gir explain <rule>` prints the full explanation for any rule id shown in brackets; `gir explain` lists every topic.

## Fixing an earlier commit

Use this instead of an interactive rebase when a change belongs in an earlier commit on your branch:

```sh
git add path/to/fix
gir fixup                        # finds the target from the staged lines
git rebase --autosquash main     # folds the fixup into its target
```

- `gir fixup <commit>` names the target explicitly; `gir fixup --dry-run` only prints the target it found.
- gir refuses when the staged lines belong to several commits, come from the base branch, or are in a new file.
  It tells you which, so you can stage per commit with `git add -p`.
- `rebase.autoSquash = true` (set by `gir doctor --fix`) makes every `git rebase -i` autosquash too.
- The `pre-push` hook refuses `fixup!`, `squash!` and `amend!` commits, so they never reach the remote.

## Commands

| Command | What it does |
|---|---|
| `gir init [--force]` | Install hooks and config. Re-running changes nothing; `--force` overwrites edited files. |
| `gir lint [<file> \| -] [--fix] [--json]` | Lint a message from a file or stdin. `--fix` rewrites the file (or prints the fixed message for stdin). |
| `gir lint --range <A..B> [--json]` | Lint existing commits, for example in CI. Also rejects unsquashed fixups. |
| `gir fixup [<commit>] [--dry-run]` | Create a `fixup!` commit for the commit the staged change belongs to. |
| `gir doctor [--fix]` | Check repo hygiene; `--fix` applies repo-local fixes only and never touches global git config. |
| `gir explain [<topic>]` | Explain a rule, `types`, `config`, `fixup`, `hooks` or `doctor`. |

Exit codes: `0` success, `1` rejected message or doctor warnings, `2` usage or runtime error.

## Configuration

`.girconfig` uses git-config syntax, so you can edit it with `git config --file .girconfig`:

```ini
[gir]
	types = feat fix docs style refactor perf test build ci chore revert
	subjectMax = 72         # longest allowed header
	scopes = api cli        # empty or unset: any scope
	scopeRequired = false
	descCase = any          # lower: lowercase the first letter of the description
	allowMerge = true       # git's default "Merge ..." subjects
	allowRevert = true      # git's default "Revert ..." subjects
	hookMissing = warn      # fail: block commits when gir is not installed
[gir "alias"]
	feature = feat
	bugfix = fix
```

`gir explain config` shows the same reference.

## Using gir in CI

Lint every commit in a pull request:

```sh
gir lint --range origin/main..HEAD
```

This catches commits made with `git commit --no-verify`.

## Using gir with coding agents

gir's output is designed to cost an agent as few tokens as possible:

- Each problem is at most three lines, with a stable rule id in brackets.
- `gir explain <rule>` holds the long form, so it is only read when needed.
- `--json` returns machine-readable results for `gir lint`.

A line like this in your `AGENTS.md` or `CLAUDE.md` is enough:

```text
Commits must follow Conventional Commits; the commit-msg hook (gir) fixes what it can and explains the rest. Run `gir explain <rule>` for any rejection.
```

## Changelog

`cliff.toml` is ready for [git-cliff](https://git-cliff.org), which gir does not replace:

```sh
git-cliff -o CHANGELOG.md
git-cliff --bumped-version
```

## Contributing

- `cargo test` runs unit tests, end-to-end tests with real git repositories, and a 750 ms latency budget for the commit-msg hook.
- `cargo clippy --all-targets -- -D warnings` must pass.
- Log any friction you hit, with the OS it happened on, in [FRICTION.md](FRICTION.md).
