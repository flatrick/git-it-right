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

- OS: windows=untested linux=seen macos=untested
- Area: cli
- Status: open
- Reproduced: yes
- Found: 2026-09-25

`gir explain | head -c 0` prints `failed printing to stdout: Broken pipe (os error 32)` from a Rust panic and exits `101`.
Any command that writes to stdout through `println!` is affected when the reader closes the pipe first.
It should exit quietly, as other command-line tools do when their output pipe closes.

### B-002 `gir doctor` reports an unreadable `.gitattributes` as missing

- OS: windows=untested linux=seen macos=untested
- Area: doctor
- Status: open
- Reproduced: yes
- Found: 2026-09-25

With `.gitattributes` present but unreadable (`chmod 000`), `gir doctor` prints `warn  .gitattributes: missing`.
`gir doctor --fix` then tries to write the template and fails with `gir: cannot write .gitattributes: Permission denied (os error 13)`, exit `2`; the file was not changed in that run.
`src/cmd/doctor.rs` treats any read error as a missing file; only `NotFound` should mean missing, and other errors should be reported as they are.
