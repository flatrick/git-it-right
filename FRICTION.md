# Friction log

Every bit of friction we hit while building or using gir goes here, including friction from our own tools, shells, and agents.
`cargo test` checks this file's format (`tests/friction.rs`).

## Format

Each entry is a `### F-NNN <title>` heading followed by these fields, then free text (what happened, workaround, fix):

- `OS:` all three keys in this order: `windows=<state> linux=<state> macos=<state>`.
  A state is `seen` (hit it there), `not-seen` (checked there, did not happen) or `untested`.
- `Shell:` optional, when the shell matters (for example `PowerShell 7`, `Git Bash`, `zsh`).
- `Area:` one lowercase word: `agent`, `harness`, `hooks`, `install`, `lint`, `fixup`, `doctor`, `ci`, `tooling`.
- `Status:` `open`, `workaround`, `fixed` or `wontfix`.
- `Found:` date as `YYYY-MM-DD`.

IDs are never reused or renumbered; append new entries at the end.

## Entries

### F-001 A failed commit leaves files staged, and the next commit sweeps them in

- OS: windows=seen linux=untested macos=untested
- Shell: PowerShell 7
- Area: agent
- Status: workaround
- Found: 2026-09-24

An agent wrote `git commit -F - @'...'@`, so the here-string became an argument (a pathspec), not stdin, and the commit aborted.
Its next command staged more files and committed everything under the wrong message.
Workaround: write the message to a file and use `git commit -F <file>`; check `git status` after any failed commit.
Idea: gir could warn when a commit carries files staged before a failed commit.

### F-002 `gh repo create --source=.` fails inside a git worktree

- OS: windows=seen linux=untested macos=untested
- Area: tooling
- Status: workaround
- Found: 2026-09-24

gh reports "current directory is not a git repository" because a worktree has a `.git` file, not a directory.
Workaround: `gh repo create <owner>/<name> --private` without `--source`, then `git remote add origin <url>`.

### F-003 Claude Code worktree sandbox blocks writes to the main checkout's `.scratch/`

- OS: windows=seen linux=untested macos=untested
- Area: harness
- Status: workaround
- Found: 2026-09-24

The convention is `.scratch/<worktree>/` under the main checkout, but a worktree-isolated session may only write inside its worktree.
Workaround: use the worktree's own `.scratch/` (gitignored by the same rule).

### F-004 Claude Code worktree guard refuses commands it cannot prove stay in the worktree

- OS: windows=seen linux=untested macos=untested
- Area: harness
- Status: workaround
- Found: 2026-09-24

`cargo add git-conventional` was refused because an argument contains "git".
A Bash timing loop with a subshell was refused as too complex to verify.
Workaround: edit `Cargo.toml` directly; split shell pipelines into plain single commands.

### F-005 tree-sitter-gitcommit 0.5 misses `squash!` and gives unstable trees for a non-blank second line

- OS: windows=seen linux=untested macos=untested
- Area: lint
- Status: fixed
- Found: 2026-09-24

`squash! feat: a` produced no `subject_prefix` node, and a non-blank line 2 produced three differently shaped ERROR trees.
Fixed by dropping tree-sitter for line checks before the first gir commit.

### F-006 Hooks run whatever `gir` is on PATH, so a changed gir needs a manual reinstall

- OS: windows=seen linux=untested macos=untested
- Area: install
- Status: open
- Found: 2026-09-24

After changing gir, the hooks keep running the old binary until `cargo install --path . --locked` runs again.
When gir is missing entirely, the shim prints a warning and lets the commit through (unless `gir.hookMissing=fail`).

### F-007 `core.hooksPath` is shared by all worktrees, but `.githooks/` exists only on some branches

- OS: windows=seen linux=untested macos=untested
- Area: hooks
- Status: open
- Found: 2026-09-24

`gir init` in a worktree sets `core.hooksPath=.githooks` in the shared repo config.
Checkouts of branches without `.githooks/` (here: `main` before the merge) silently run no hooks.

### F-008 `gir doctor --fix` output describes the state before the fix

- OS: windows=seen linux=untested macos=untested
- Area: doctor
- Status: open
- Found: 2026-09-24

Fixed lines still read "unset; `true` recommended", and the summary counts ok checks from before the fixes.
