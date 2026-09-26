# Verification: `B-001 and B-002 fixes re-verified on Linux`

## Claim

**Reference:** `history:tasks/fix-b001-b002/TASK.md#closed-stdout-quiet`,
`history:tasks/fix-b001-b002/TASK.md#unreadable-gitattributes-reported`,
`history:tasks/fix-b001-b002/TASK.md#write-only-gitattributes-kept`,
`history:tasks/fix-b001-b002/TASK.md#b001-reproduces`,
`history:tasks/fix-b001-b002/TASK.md#b002-reproduces`

The archived Task verified these on Linux, but its raw output was never
committed, so its Verifications record summaries only. This Verification
repeats the observations and commits the output.

## Method

In WSL2 Arch Linux as a non-root user (uid `1000`), clone the repository
into the Linux filesystem, so that `chmod` has its Linux meaning, and check
out `72457d0`. Build the pre-fix `839a991` in a second worktree as a control.

-   Run `cargo clippy --all-targets -- -D warnings`, `cargo test
    --no-fail-fast` and `check-capsule --root addf --include-archive` at
    `72457d0`, writing each exit code into the log.
-   B-001: on each build, in a repository with one commit and a staged
    change, run `(sleep 0.3; printf INPUT | gir ARGS) 2>err | true` three
    times for each of the 8 invocations in `tests/args.rs`
    `closed_stdout_exits_141_without_output`, recording `${PIPESTATUS[0]}`
    and the stderr bytes.
-   B-002: on each build, in a fresh repository with `.gitattributes`
    holding `# mine`, `chmod 000` or `chmod 200`, run `gir doctor` and
    `gir doctor --fix`, restore mode `600`, and compare the SHA-256 before
    and after.

## Expected observations

-   The fixed build exits `141` with 0 stderr bytes for all 8 invocations,
    reports `cannot read` in both modes, and leaves the file unchanged.
-   The control panics with `Broken pipe (os error 32)` and exits `101`,
    reports `missing` in both modes, and under mode `200` its `--fix`
    replaces the file.

## Observed results

### `linux-gate`

-   Fact: `clippy exit=0`, `test exit=0`, `capsule exit=0`. Every test
    binary reports `ok`, including `closed_stdout_exits_141_without_output`,
    `doctor_reports_unreadable_gitattributes_and_does_not_fix_it` and
    `doctor_fix_keeps_write_only_gitattributes`, and no `skipped: mode`
    notice appears.
-   Source and method: [gate log](logs-20260926/linux/gate-linux-72457d0-20260926-1600.log).
-   Context: Linux 6.18.33.2-microsoft-standard-WSL2 x86_64, uid `1000`,
    rustc 1.98.0, git 2.55.0, 2026-09-26.
-   Limitations: one machine, one run.

### `linux-repro`

-   Fact: fixed build, 24 of 24 runs `exit=141 stderr_bytes=0`; `chmod 000`
    and `chmod 200` both print `warn  .gitattributes: cannot read:
    Permission denied (os error 13)` from `doctor` and `doctor --fix`, and
    the hash `49cf24c707d61fd3` is unchanged. Control, 24 of 24 runs
    `exit=101 stderr_bytes=249` with `failed printing to stdout: Broken pipe
    (os error 32)`; both modes report `missing and core.autocrlf=input`;
    mode `000` `--fix` exits `2` with `gir: cannot write .gitattributes:
    Permission denied (os error 13)` and the hash is unchanged; mode `200`
    `--fix` prints `fixed .gitattributes` and the hash becomes
    `d94a9286cc06a0be`.
-   Source and method: [repro log](logs-20260926/linux/repro-linux-20260926-1601.log).
-   Context: as above; both builds are debug builds.
-   Limitations: `sleep 0.3` makes the early close likely, not certain;
    every run closed early.

## Evidence considered

-   [linux-gate](#linux-gate) SUPPORTS `closed-stdout-quiet`,
    `unreadable-gitattributes-reported` and `write-only-gitattributes-kept`:
    their regression tests ran and passed as non-root.
-   [linux-repro](#linux-repro) SUPPORTS all five. The fixed build behaves
    as the success Claims require. The control reproduces `b001-reproduces`
    and `b002-reproduces` exactly, which also shows the fixed build's
    results come from the fix and not from the environment.

## Contradictory and inconclusive evidence

`NONE`.

## Conclusion

<a id="conclusion"></a>

**Result:** `VERIFIED`

All five Claims hold on Linux at `72457d0` and `839a991`, with the raw
output committed.

## Remaining uncertainty

`NONE` blocking. The archived Task's own Linux runs remain summaries; this
Verification supplies the committed output.
