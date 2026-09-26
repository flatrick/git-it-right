# Known bugs

Every known, unfixed defect in gir goes here until it is fixed, so it is not lost in a chat or a commit message.
`cargo test` checks this file's format (`tests/logs.rs`).

## Format

Each entry is a `### B-NNN <title>` heading followed by these fields, then free text (how to reproduce, what happens, what should happen):

- `OS:` all three keys in this order: `windows=<state> linux=<state> macos=<state>`.
  A state is `seen` (reproduced there), `not-seen` (checked there, did not happen) or `untested`.
- `Area:` the spec module it breaks: `cli`, `config`, `lint`, `hooks`, `fixup`, `doctor`, `init` or `explain`.
- `Status:` `open`, `fixed` or `wontfix`.
- `Reproduced:` `yes` when someone ran it and saw the bug, `no` when it is only suspected from reading the code.
- `Found:` date as `YYYY-MM-DD`.

IDs are never reused or renumbered; append new entries at the end.
When a bug is fixed, set `Status: fixed` and name the commit in the text.

## Entries

### B-001 gir panics when stdout is closed early

- OS: windows=seen linux=seen macos=untested
- Area: cli
- Status: fixed
- Reproduced: yes
- Found: 2026-09-25

`gir explain | head -c 0` prints `failed printing to stdout: Broken pipe (os error 32)` from a Rust panic and exits `101`.
Any command that writes to stdout through `println!` is affected when the reader closes the pipe first.
It should exit quietly, as other command-line tools do when their output pipe closes.
On Windows the panic reads `failed printing to stdout: The pipe is being closed. (os error 232)`, also exit `101`.
`gir explain` writes only 365 bytes, so `| head -c 0` races and fired 1 run in 10 there; `(sleep 0.5; gir explain) | true` in Git Bash fires every time.
Fixed in `9333a1f`: every stdout write goes through `src/stdout.rs`, which exits `141` silently on a broken pipe (`tests/args.rs` `closed_stdout_exits_141_without_output`).
Windows is not yet re-checked after the fix.

### B-002 `gir doctor` reports an unreadable `.gitattributes` as missing

- OS: windows=seen linux=seen macos=untested
- Area: doctor
- Status: fixed
- Reproduced: yes
- Found: 2026-09-25

With `.gitattributes` present but unreadable (`chmod 000`), `gir doctor` prints `warn  .gitattributes: missing`.
`gir doctor --fix` then tries to write the template and fails with `gir: cannot write .gitattributes: Permission denied (os error 13)`, exit `2`; the file was not changed in that run.
`src/cmd/doctor.rs` treats any read error as a missing file; only `NotFound` should mean missing, and other errors should be reported as they are.
On Windows, denying read with `icacls .gitattributes /deny "%USERNAME%:(R)"` gives the same `missing` warning, and `--fix` fails with `os error 5`, exit `2`, file unchanged.
Denying only read-data with `(RD)` leaves the file writable: `--fix` then reports `fixed .gitattributes` and replaces the user's file with the template.
The same overwrite happens on Linux with a write-only file (`chmod 200`): `--fix` reported `fixed .gitattributes` and replaced the content (reproduced 2026-09-26).
Fixed in `3d95b90`: only `NotFound` means missing; any other read error prints `warn  .gitattributes: cannot read: <error>` with no fix (`tests/doctor.rs`).
Windows is not yet re-checked after the fix.

### B-003 `gir doctor --fix` can overwrite a file created after its check

- OS: windows=untested linux=untested macos=untested
- Area: doctor
- Status: open
- Reproduced: no
- Found: 2026-09-26

`gir doctor` checks for `.gitattributes` and `.editorconfig`, then `--fix` writes the template with `std::fs::write` in `src/cmd/doctor.rs` `apply`.
If another process creates the file between the check and the write, `--fix` truncates it and replaces it with the template.
It should create the file only if it still does not exist (`OpenOptions::create_new`) and report the file as present otherwise.
Suspected from a Codex review of the B-002 fix; not reproduced.

### B-004 `gir init` overwrites a file it cannot read, without `--force`

- OS: windows=untested linux=seen macos=untested
- Area: init
- Status: open
- Reproduced: yes
- Found: 2026-09-26

`src/cmd/init.rs` `write` falls through to `std::fs::write` on any read error, not only `NotFound`.
With `.girconfig` containing `subjectMax = 50` at mode `200`, `gir init` prints `gir: wrote .girconfig`, replaces the content with the template, then fails with `gir: .girconfig: warning: unable to access ... Permission denied`, exit `2`.
`cliff.toml` and `.githooks/*` take the same path.
It should report the read error and leave the file alone, as `gir doctor` does since B-002.

### B-005 `gir fixup` reads a removed `-- ` line as a file header

- OS: windows=untested linux=seen macos=untested
- Area: fixup
- Status: open
- Reproduced: yes
- Found: 2026-09-26

`src/cmd/fixup.rs` `parse_hunks` takes any `--- ` line as a file header.
With `-U0`, removing the line `-- note` (an SQL, Lua or Haskell comment) shows as `--- note` inside a hunk, so the later hunks of that file are blamed against a file named `note`.
Repro: commit `q.sql` with a `-- note` line and 8 lines in all, then on a branch stage a change that removes `-- note` and edits the last line; `gir fixup --dry-run` prints `gir: cannot tell which commit note:8 belongs to`, exit `2`.
Only a `--- ` line between `diff --git` and the first `@@` should be a header.

### B-006 A `0x1e` byte in a commit message hides it from `gir lint --range` and pre-push

- OS: windows=untested linux=seen macos=untested
- Area: lint
- Status: open
- Reproduced: yes
- Found: 2026-09-26

`src/cmd/lint.rs` `commits` splits `git log --format=%H%x1f%B%x1e` on `0x1e`, which a message can contain.
A commit `feat: x` followed directly by a body line gives `rejected [fix-pending]`, exit `1`; the same message with `0x1e` after `feat: x` passes with exit `0`, because only the part before the byte is linted.
The pre-push hook uses the same function.
Records should be split on a byte a message cannot contain (`git log -z`), or each body read separately.

### B-007 pre-push to a URL lints every reachable commit

- OS: windows=untested linux=seen macos=untested
- Area: hooks
- Status: open
- Reproduced: yes
- Found: 2026-09-26

When a push names a URL instead of a remote, git passes the URL as the remote name, and `--remotes=<url>` matches no remote-tracking ref.
For a new branch, `gir hook pre-push` then lints the whole history: pushing to a bare repository by path rejected `old bad message`, a commit that repository already had, exit `1`; the same push through a named remote exited `0`.
This follows `spec/hooks.md#req-hooks-pre-push-scope` as written, so the fix changes the spec.
Decided (operator, 2026-09-26): ask the remote. For a new branch, or when `remote-sha..local-sha` fails, run `git ls-remote REMOTE` (a name or a URL) and lint `local-sha --not` every returned SHA that exists locally, for named remotes as well as URLs, replacing the remote-tracking-ref lookup.
Git connects and reads the remote's refs before it runs pre-push, so an unreachable remote fails `git push` (exit `128`) before the hook runs; checked with git 2.55.0.
Still open for the fixing Task's DEFINE: `git ls-remote` is a second connection, so it can ask for credentials again (an SSH passphrase, or HTTPS without a credential helper) or fail if the remote goes away in between; decide what the hook does then.

### B-008 `gir fixup` cannot trace paths with a space, quote, backslash, tab or non-UTF-8 byte

- OS: windows=untested linux=seen macos=untested
- Area: fixup
- Status: open
- Reproduced: yes
- Found: 2026-09-26

`src/cmd/fixup.rs` `parse_hunks` takes the path from the `--- a/` header line as it is.
Git ends that line with a tab when the name contains a space, and quotes it with C escapes when it contains `"`, `\`, a tab or another control byte, even with `core.quotePath=false`.
`src/git.rs` `run` also decodes output lossily, so a non-UTF-8 byte becomes U+FFFD.
Repro: on a branch after the commit that added the file, change one line of it, stage it and run `gir fixup --dry-run`.
`plain.txt` prints the commit, exit `0`.
`sp ace.txt` prints `gir: cannot tell which commit sp ace.txt belongs to`, exit `2`; its header is `--- a/sp ace.txt` followed by a tab.
A name with byte `0xff` (`x\377.txt` in `printf`) prints `x` U+FFFD `.txt:2`, exit `2`.
`a<TAB>b.txt`, `q"uote.txt` and `b\s.txt` print the quoted name, exit `2`.
Paths should come from `-z` output, which needs no unquoting, and be kept as bytes.

### B-009 `gir doctor` mishandles non-UTF-8 index paths

- OS: windows=untested linux=seen macos=untested
- Area: doctor
- Status: open
- Reproduced: yes
- Found: 2026-09-26

`src/cmd/doctor.rs` `index_checks` reads `git ls-files -s` through lossy UTF-8 decoding, so distinct names can decode to the same string.
Staged names `A` + byte `0x80` and `a` + byte `0x81` give `warn  case-collision: paths differ only in case`, although they differ in more than case.
Staged names `a` + `0x80` and `a` + `0x81` decode to the same adjacent string, so the unmerged-stage dedup drops the second and no warning appears; that is right only by accident.
Staged scripts `x` + `0x80` + `.sh` and `x` + `0x81` + `.sh` at mode `100644` give an `exec-bit` warning that names one lossy path.
`gir doctor --fix` then fails with `gir: error: x` U+FFFD `.sh: does not exist and --remove not passed`, exit `2`, and neither script becomes executable.
Paths should be read with `-z` and kept as bytes for comparison and for the `git update-index` fix.

### B-010 The fixup-table test never reads `CHEATSHEET.md`

- OS: windows=untested linux=seen macos=untested
- Area: explain
- Status: open
- Reproduced: yes
- Found: 2026-09-26

`tests/cli.rs` `cheatsheet_fixup_table_matches_git` says it checks every row of the fixup table in `CHEATSHEET.md` against real git, but it only runs git with hard-coded expectations and never opens the file.
A false edit to a table row, which `gir explain` shows to users, still passes: swapping row 3 to `content replaced, message untouched` left `cargo test --test cli cheatsheet_fixup` green.
The test should parse the table rows it claims to check.
