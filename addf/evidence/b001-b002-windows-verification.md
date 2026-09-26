# Verification: `B-001 and B-002 are fixed on Windows`

## Claim

**Reference:** `history:open-claims/b001-b002-windows.md#b001-b002-windows`

## Method

Built `gir` (debug) at `42b84ad` on this branch; `git diff 86fb64b 42b84ad
-- src` is empty, so the build is the claimed `src/`. As a control, built
the pre-fix `839a991` from `git archive` the same way.

-   B-001: in Git Bash (`C:\Program Files\Git\bin\bash.exe`, not the WSL
    `bash.exe` first on `PATH`), `(sleep 0.5; gir explain) 2>file | true`,
    recording `${PIPESTATUS[0]}` and the stderr byte count. Five runs on the
    fixed build, three on the control, and one `gir explain >/dev/null` run
    with an open reader.
-   B-002: in a fresh `git init` repository per case, write
    `.gitattributes`, hash it, deny read with `icacls .gitattributes /deny
    "%USERNAME%:(R)"` or `(RD)`, confirm .NET cannot read it, run `gir
    doctor` then `gir doctor --fix`, restore the ACL with `icacls /reset`,
    and hash again. Each deny mode ran on the fixed build and the control.

## Expected observations

-   Fixed build: B-001 exits `141` with 0 stderr bytes; B-002 prints `warn
    .gitattributes: cannot read: ` and the hash is unchanged after `--fix`.
-   Control: B-001 panics with `os error 232` and exits `101`; B-002 reports
    `missing`. Without this, a `141` could come from Git Bash rather than
    from `gir`.

## Observed results

### `b001`

-   Fact: fixed build, 5 of 5 runs `pipestatus=141 stderr_bytes=0`; open
    reader `exit=0 stderr_bytes=0`. Control, 3 of 3 runs `pipestatus=101`
    with `failed printing to stdout: Denna pipe håller på att stängas. (os
    error 232)`.
-   Source and method: main checkout `.scratch/fix-b001-b002/b001-20260926-1411.log`
    and `b001-prefix-20260926-1411.log` (local, not committed).
-   Context: Windows 11 Home 10.0.26200, Swedish locale, Git for Windows
    2.55.0.windows.5, rustc 1.94.0, 2026-09-26.
-   Limitations: only `gir explain` was run by hand. The other seven
    invocations are covered on Windows by `tests/args.rs`
    `closed_stdout_exits_141_without_output`, which passed in the same
    session's `cargo test`.

### `b002`

-   Fact: fixed build, both `(R)` and `(RD)`: .NET read denied; `gir
    doctor` and `gir doctor --fix` both print `warn  .gitattributes: cannot
    read: Åtkomst nekad. (os error 5)` and exit `1`; hash
    `13CB7F5E…4262222` before and after. Control `(R)`: `missing`, `--fix`
    exits `2` with `cannot write .gitattributes`, hash unchanged. Control
    `(RD)`: `missing`, `--fix` prints `fixed .gitattributes` and the hash
    changes to the template's `D94A9286…ECF3602`.
-   Source and method: main checkout
    `.scratch/fix-b001-b002/b002-rerun-20260926-1412.log` (local, not
    committed).
-   Context: as above. An earlier run, `b002-20260926-1412.log`, is void:
    `Set-Content -NoNewline` failed in that shell and `.gitattributes` was
    never created.
-   Limitations: one machine, run by hand. Since `67f5be6`, the same `(R)`
    and `(RD)` cases run on Windows in `cargo test` as
    `doctor_reports_read_denied_gitattributes_and_does_not_fix_it` and
    `doctor_fix_keeps_read_data_denied_gitattributes`; both fail against
    `3d95b90`'s parent.

## Evidence considered

-   [b001](#b001) SUPPORTS: exit `141` and empty stderr on every run, and
    the control shows the old panic under the same shell, so the `141`
    comes from `gir`.
-   [b002](#b002) SUPPORTS: both deny modes give `cannot read` and an
    unchanged file. The `(RD)` control overwrote the file, so the fixed
    build's unchanged hash shows that `--fix` declined to write, not that it
    failed to.

## Contradictory and inconclusive evidence

`NONE`.

## Conclusion

<a id="conclusion"></a>

**Result:** `VERIFIED`

Both halves hold on Windows at `86fb64b`'s `src/`: B-001 exits `141`
silently in Git Bash, and B-002 reports an unreadable `.gitattributes`
and leaves it unchanged under `(R)` and `(RD)`.

## Remaining uncertainty

`NONE` blocking. The Windows regression tests added in `67f5be6` have
passed locally but not yet on the `windows-latest` CI runner.
