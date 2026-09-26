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
