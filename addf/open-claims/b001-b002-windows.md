# Open Claim: `b001-b002-windows`

## Claim

<a id="b001-b002-windows"></a>
-   Claim: On Windows, `(sleep 0.5; gir explain) | true` in Git Bash exits
    `141` with nothing on stderr, and with `.gitattributes` denied read by
    `icacls .gitattributes /deny "%USERNAME%:(R)"` or `(RD)`, `gir doctor`
    prints `warn  .gitattributes: cannot read: ` and `gir doctor --fix` leaves
    the file unchanged.
-   State: `UNVERIFIED`
-   Scope: `src/` at `86fb64b`, Windows.
-   Consequence if false: `B-001 or B-002 remains open on Windows, where both were first seen.`
-   Basis: `DEFERRED_VERIFICATION`; no Windows host in the originating
    session. Rust std maps `ERROR_NO_DATA` (232) to `ErrorKind::BrokenPipe`,
    which supports but does not settle the B-001 half.

## Originating Task

-   `history:tasks/fix-b001-b002/TASK.md#windows-behavior`

## Next action

On Windows, build `gir` at or after `86fb64b` and run the commands in the
Claim, recording exit codes, stderr and the file content before and after
`--fix`. Settle the two bugs separately if only one passes, and reopen the
matching `BUGS.md` entry if one fails.
