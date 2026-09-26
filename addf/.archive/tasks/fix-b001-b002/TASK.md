# TASK — `B-001 and B-002 are fixed`

## Resume

**Contract version:** `2`

**State:** `COMPLETED`

**State path:** `DEFINE -> UNDERSTAND -> INVESTIGATE -> DECIDE -> IMPLEMENT -> VERIFY -> LEARN -> COMPLETED`

**Resume at:** `NONE`

**Open obligations:** `NONE`

## Owned artifacts

-   `ledger.md` - pre-Task questions and the operator's DEFINE answers.
-   Codex reviews (supplemental, outside the bundle, not committed): main
    checkout `.scratch/git-it-right-bugfix/codex/plan-review-20260926-121403/`
    and `code-review-20260926-121637/`.

## Specification impact

- Current contract: `framework:spec/cli.md#req-cli-runtime-error`,
  `framework:spec/doctor.md#req-doctor-gitattributes-missing`,
  `framework:spec/doctor.md#req-doctor-gitattributes-autocrlf`
- Proposed delta: three changes to the `cli` and `doctor` modules.
  - cli: add `req-cli-closed-stdout`. When the reader closes stdout before gir
    finishes writing, gir SHALL print nothing more and exit `141`.
  - doctor: `req-doctor-gitattributes-missing` and
    `req-doctor-gitattributes-autocrlf` apply when `.gitattributes` does not
    exist, not whenever it cannot be read. Add
    `req-doctor-gitattributes-unreadable`: an existing `.gitattributes` that
    cannot be read SHALL produce `warn  .gitattributes: cannot read: ` and the
    OS error; `--fix` SHALL NOT write the file.
  - cli: `req-cli-runtime-error` excludes a closed stdout (Codex plan review
    finding 4), and `req-cli-closed-stdout` covers only gir's own writes, not
    a git child with inherited stdio (Codex finding 1).
- Terminal publication: `framework:spec/cli.md#req-cli-runtime-error`,
  `framework:spec/cli.md#req-cli-closed-stdout`,
  `framework:spec/doctor.md#req-doctor-gitattributes-missing`,
  `framework:spec/doctor.md#req-doctor-gitattributes-autocrlf`,
  `framework:spec/doctor.md#req-doctor-gitattributes-unreadable`

## Define

### Objective

`gir` exits quietly with `141` when its stdout closes early, and `gir doctor`
reports an unreadable `.gitattributes` as unreadable and never overwrites it.

### Success criteria

<a id="closed-stdout-quiet"></a>
#### `closed-stdout-quiet`

-   Claim: Every gir command that writes to stdout prints nothing on stderr
    and exits `141` when its stdout pipe has no reader.
-   State: `VERIFIED`
-   Scope: Linux, this branch.
-   Consequence if false: B-001 stays open.
-   Basis: [Verification](#verification-closed-stdout-quiet).

<a id="unreadable-gitattributes-reported"></a>
#### `unreadable-gitattributes-reported`

-   Claim: With `.gitattributes` at mode `000`, `gir doctor` prints
    `warn  .gitattributes: cannot read: `, and `gir doctor --fix` leaves the
    file unchanged and does not fail on it.
-   State: `VERIFIED`
-   Scope: Linux, non-root user, this branch.
-   Consequence if false: B-002 stays open.
-   Basis: [Verification](#verification-unreadable-gitattributes-reported).

<a id="write-only-gitattributes-kept"></a>
#### `write-only-gitattributes-kept`

-   Claim: With `.gitattributes` at mode `200`, `gir doctor --fix` leaves its
    content unchanged.
-   State: `VERIFIED`
-   Scope: Linux, non-root user, this branch.
-   Consequence if false: user data loss remains.
-   Basis: [Verification](#verification-write-only-gitattributes-kept).

### Constraints

-   Work happens in worktree `../git-it-right-bugfix` on branch
    `fix/b001-b002`, from `main` at `839a991` (operator request).
-   Each failing regression test is committed before its fix.
-   Windows is not available in this session.

### Material empirical premises

<a id="windows-behavior"></a>
#### `windows-behavior`

-   Claim: On Windows, B-001's repro exits `141` silently and B-002's
    `icacls` cases print `cannot read` and leave the file unchanged.
-   State: `UNVERIFIED`
-   Scope: Windows, this branch.
-   Consequence if false: both bugs stay open on Windows.
-   Basis: `DEFERRED_VERIFICATION`; no Windows host in this session. Rust std
    source maps `ERROR_NO_DATA` (232) to `ErrorKind::BrokenPipe` (Codex plan
    review finding 3), which supports but does not settle it.

<a id="b001-reproduces"></a>
#### `b001-reproduces`

-   Claim: `(sleep 0.3; gir explain) | true` panics with
    `failed printing to stdout: Broken pipe (os error 32)` and exits `101`.
-   State: `VERIFIED`
-   Scope: Linux, `839a991`, debug build, 3 of 3 runs.
-   Consequence if false: nothing to fix on Linux.
-   Basis: [Verification](#verification-b001-reproduces).

<a id="b002-reproduces"></a>
#### `b002-reproduces`

-   Claim: Mode `000` gives `warn  .gitattributes: missing` and `--fix` exits
    `2`; mode `200` makes `--fix` report `fixed .gitattributes` and replace the
    user's content with the template.
-   State: `VERIFIED`
-   Scope: Linux, `839a991`, non-root user.
-   Consequence if false: nothing to fix on Linux.
-   Basis: [Verification](#verification-b002-reproduces).

## Understand

### Relevant context

Rust ignores `SIGPIPE`, so a write to a closed pipe returns `EPIPE`, and
`println!`/`print!` panic on any write error. stdout writers: `src/main.rs`
(version, help, explain), `src/cmd/doctor.rs` (report, summary),
`src/cmd/lint.rs` (fixed message, `--json`), `src/cmd/fixup.rs` (`--dry-run`).

`src/cmd/doctor.rs` `file_checks` matches `Err(_)` on the `.gitattributes`
read and attaches `Fix::WriteFile`, so any read error means "missing" and
`--fix` writes the template.

Gate: `ESTABLISHED`; the causes are located in code and match the observed
output.

### Assumptions

-   Rust maps Windows `ERROR_NO_DATA` (232) to `ErrorKind::BrokenPipe`;
    source: std `sys/io/error/windows.rs`, read by Codex; no runtime check;
    if false, Windows keeps a message and a non-141 exit.

### Open questions

`NONE`.

### Deferred verification

-   Both fixes on Windows; no Windows host here; checkpoint: operator's
    Windows run; settling observation: `(sleep 0.5; gir explain) | true` in
    Git Bash exits `141` silently, and the `icacls` cases in `BUGS.md` show
    `cannot read` with the file unchanged; consequence if false: bugs stay
    open on Windows; blocks nothing on Linux.

## Investigate

Reproduced at `839a991` on Linux before any change:

```text
$ (sleep 0.3; gir explain) | true      # 3 runs
failed printing to stdout: Broken pipe (os error 32)
exit=101
$ chmod 000 .gitattributes; gir doctor | grep gitattributes
warn  .gitattributes: missing and core.autocrlf=input, so line endings depend on each clone
$ gir doctor --fix
gir: cannot write .gitattributes: Permission denied (os error 13)   exit=2
$ chmod 200 .gitattributes; gir doctor --fix | grep gitattributes
fixed .gitattributes: missing and core.autocrlf=input, ...          # content replaced
```

The `chmod 200` overwrite that `BUGS.md` listed as untested on Linux is
confirmed. Gate: `ESTABLISHED`.

## Decide

-   B-001: route every stdout write through one crate function that returns
    on success, exits `141` on `BrokenPipe`, and exits `2` with
    `gir: cannot write stdout: <e>` on any other error (`req-cli-runtime-error`).
    `main` flushes stdout before exiting and applies the same mapping, so a
    buffered `print!` without a newline is covered.
    Rejected: a panic hook matching the message text (brittle); restoring
    `SIGPIPE` (needs `unsafe`/libc and does nothing on Windows).
-   B-002: split the read result on `ErrorKind::NotFound`. Only `NotFound`
    keeps the missing warning and its fix; any other error becomes a `warn`
    line with no fix.
-   Verification: integration tests in `tests/cli.rs` and `tests/doctor.rs`,
    committed failing first, plus the manual repro commands above.

Gate: `ESTABLISHED`; each step has a failing observation that the fix must
turn green.

## Implement

-   `c8af95b` failing regressions; `3d95b90` B-002 fix; `9333a1f` B-001 fix
    (`src/stdout.rs`, `out!`/`outln!` at every stdout write);
    `86fb64b` makes the root skip in the doctor regressions visible.
-   Deviation: the planned final `flush()` in `main` was dropped. Every stdout
    write ends in a newline (`message::join` appends one), so `LineWriter`
    surfaces the error inside `write_fmt`; no input reached the flush.
-   Codex review findings not taken: a git child with inherited stdout
    (narrowed in the spec instead); `doctor --fix` stopping at its first
    report line on a closed stdout (same point the old panic stopped; fixes
    are idempotent, so a rerun converges); the check-then-write race in
    `--fix` (pre-existing, unreproduced, filed as B-003).

## Verify

<a id="verification-b001-reproduces"></a>
### Verification: `b001-reproduces`

- Claim: [b001-reproduces](#b001-reproduces)
- Method: `(sleep 0.3; gir explain) | true` on a debug build of `839a991`,
  3 runs, recording stderr and the exit code, as shown in
  [Investigate](#investigate).
- Evidence considered: all 3 runs print `failed printing to stdout: Broken
  pipe (os error 32)` and exit `101`.
- Conclusion: `VERIFIED` on Linux.
- Limitations: Linux only; Windows was reproduced separately in `BUGS.md`.

<a id="verification-b002-reproduces"></a>
### Verification: `b002-reproduces`

- Claim: [b002-reproduces](#b002-reproduces)
- Method: `chmod 000` and then `chmod 200` on `.gitattributes`, running
  `gir doctor` and `gir doctor --fix` on `839a991` as a non-root user, as
  shown in [Investigate](#investigate).
- Evidence considered: mode `000` reports `missing` and `--fix` exits `2`
  with `Permission denied (os error 13)`; mode `200` makes `--fix` report
  `fixed .gitattributes` and replace the content with the template.
- Conclusion: `VERIFIED` on Linux, non-root.
- Limitations: Linux only; Windows was reproduced separately in `BUGS.md`.

<a id="verification-closed-stdout-quiet"></a>
### Verification: `closed-stdout-quiet`

- Claim: [closed-stdout-quiet](#closed-stdout-quiet)
- Method: `tests/args.rs` `closed_stdout_exits_141_without_output` gives
  each of 8 invocations (`--version`, `--help`, `explain`, `explain doctor`,
  `doctor`, `lint --json -`, `lint --fix -`, `fixup --dry-run`) a pipe whose
  reader is already closed. Manual `(sleep .3; gir ARGS) | true` for the same
  8 on the binary before and after the fix.
- Evidence considered: before, all 8 exit `101` with 251 bytes of panic on
  stderr, and the test fails; after, all 8 exit `141` with 0 stderr bytes,
  and `cargo test` passes all 14 test binaries.
- Conclusion: `VERIFIED` on Linux.
- Limitations: Windows is `windows-behavior`, deferred.

<a id="verification-unreadable-gitattributes-reported"></a>
### Verification: `unreadable-gitattributes-reported`

- Claim: [unreadable-gitattributes-reported](#unreadable-gitattributes-reported)
- Method: `tests/doctor.rs`
  `doctor_reports_unreadable_gitattributes_and_does_not_fix_it`, non-root.
- Evidence considered: fails at `c8af95b` (`missing` reported), passes after
  `3d95b90`; `--fix` has empty stderr and the content stays `# mine`.
- Conclusion: `VERIFIED` on Linux, non-root.
- Limitations: skipped with a notice when run as root.

<a id="verification-write-only-gitattributes-kept"></a>
### Verification: `write-only-gitattributes-kept`

- Claim: [write-only-gitattributes-kept](#write-only-gitattributes-kept)
- Method: `tests/doctor.rs` `doctor_fix_keeps_write_only_gitattributes`,
  non-root.
- Evidence considered: fails at `c8af95b` (file replaced by the template),
  passes after `3d95b90`.
- Conclusion: `VERIFIED` on Linux, non-root.
- Limitations: as above.

## Learn

### Technical

A Rust CLI that prints with `println!` panics with exit `101` on a closed
pipe, because Rust ignores `SIGPIPE`. Routing stdout through one checked
writer is portable; restoring `SIGPIPE` is not.

### Process

No material process learning. Codex reviews produced two accepted findings
(spec precedence, visible root skip) and one new bug (B-003).

## Retention and promotion

### Promotion: `closed-stdout-quiet`

-   Claim: [closed-stdout-quiet](#closed-stdout-quiet)
-   Will this Claim's validity outlive this Task and inform a future
    decision? `no`; it is now `req-cli-closed-stdout` and a regression test.
-   Disposition: not promoted — Task-scoped only.

### Promotion: `unreadable-gitattributes-reported`, `write-only-gitattributes-kept`

-   Claim: [unreadable-gitattributes-reported](#unreadable-gitattributes-reported),
    [write-only-gitattributes-kept](#write-only-gitattributes-kept)
-   Will this Claim's validity outlive this Task and inform a future
    decision? `no`; it is now `req-doctor-gitattributes-unreadable` and tests.
-   Disposition: not promoted — Task-scoped only.

### Promotion: `windows-behavior`

-   Claim: [windows-behavior](#windows-behavior)
-   Will this Claim's validity outlive this Task and inform a future
    decision? `yes`; whether B-001 and B-002 are fixed on Windows decides
    whether they reopen.
-   Disposition: carried forward to `open-claims/b001-b002-windows.md`.

## Archive readiness

All internal references are same-file anchors or `ledger.md`. The Codex
evidence directories and `framework:` spec references are supplemental.

## Terminal record

### Summary

B-001 and B-002 are fixed and verified on Linux. The spec delta is
published, and Windows verification is carried forward as an open claim.

### Gate basis

All three success Claims are `VERIFIED` via their Verifications above.
`windows-behavior` stays `UNVERIFIED`, is agreed in DEFINE as deferred, and
is carried forward to `history:open-claims/b001-b002-windows.md`, since
settled and retired.

## Stop record

`NONE`.
