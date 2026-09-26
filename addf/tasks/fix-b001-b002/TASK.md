# TASK — `B-001 and B-002 are fixed`

## Resume

**Contract version:** `2`

**State:** `IMPLEMENT`

**State path:** `DEFINE -> UNDERSTAND -> INVESTIGATE -> DECIDE -> IMPLEMENT`

**Resume at:** Commit the failing regression tests, then the fixes, per [Decide](#decide).

**Open obligations:**

-   Windows behavior of both fixes; deferred checkpoint: the operator's Windows run, carried forward as an open claim at LEARN.
-   Publish the spec delta; blocks `COMPLETED`.

## Owned artifacts

-   `ledger.md` - pre-Task questions and the operator's DEFINE answers.

## Specification impact

- Current contract: `framework:spec/cli.md#req-cli-runtime-error`,
  `framework:spec/doctor.md#req-doctor-gitattributes-missing`,
  `framework:spec/doctor.md#req-doctor-gitattributes-autocrlf`
- Proposed delta:
  - cli: add `req-cli-closed-stdout`. When the reader closes stdout before gir
    finishes writing, gir SHALL print nothing more and exit `141`.
  - doctor: `req-doctor-gitattributes-missing` and
    `req-doctor-gitattributes-autocrlf` apply when `.gitattributes` does not
    exist, not whenever it cannot be read. Add
    `req-doctor-gitattributes-unreadable`: an existing `.gitattributes` that
    cannot be read SHALL produce `warn  .gitattributes: cannot read: ` and the
    OS error; `--fix` SHALL NOT write the file.
- Terminal publication: `PENDING`

## Define

### Objective

`gir` exits quietly with `141` when its stdout closes early, and `gir doctor`
reports an unreadable `.gitattributes` as unreadable and never overwrites it.

### Success criteria

<a id="closed-stdout-quiet"></a>
#### `closed-stdout-quiet`

-   Claim: Every gir command that writes to stdout prints nothing on stderr
    and exits `141` when its stdout pipe has no reader.
-   State: `UNVERIFIED`
-   Scope: Linux, this branch.
-   Consequence if false: B-001 stays open.
-   Basis: pending check.

<a id="unreadable-gitattributes-reported"></a>
#### `unreadable-gitattributes-reported`

-   Claim: With `.gitattributes` at mode `000`, `gir doctor` prints
    `warn  .gitattributes: cannot read: `, and `gir doctor --fix` leaves the
    file unchanged and does not fail on it.
-   State: `UNVERIFIED`
-   Scope: Linux, non-root user, this branch.
-   Consequence if false: B-002 stays open.
-   Basis: pending check.

<a id="write-only-gitattributes-kept"></a>
#### `write-only-gitattributes-kept`

-   Claim: With `.gitattributes` at mode `200`, `gir doctor --fix` leaves its
    content unchanged.
-   State: `UNVERIFIED`
-   Scope: Linux, non-root user, this branch.
-   Consequence if false: user data loss remains.
-   Basis: pending check.

### Constraints

-   Work happens in worktree `../git-it-right-bugfix` on branch
    `fix/b001-b002`, from `main` at `839a991` (operator request).
-   Each failing regression test is committed before its fix.
-   Windows is not available in this session.

### Material empirical premises

<a id="b001-reproduces"></a>
#### `b001-reproduces`

-   Claim: `(sleep 0.3; gir explain) | true` panics with
    `failed printing to stdout: Broken pipe (os error 32)` and exits `101`.
-   State: `VERIFIED`
-   Scope: Linux, `839a991`, debug build, 3 of 3 runs.
-   Consequence if false: nothing to fix on Linux.
-   Basis: [Investigate](#investigate).

<a id="b002-reproduces"></a>
#### `b002-reproduces`

-   Claim: Mode `000` gives `warn  .gitattributes: missing` and `--fix` exits
    `2`; mode `200` makes `--fix` report `fixed .gitattributes` and replace the
    user's content with the template.
-   State: `VERIFIED`
-   Scope: Linux, `839a991`, non-root user.
-   Consequence if false: nothing to fix on Linux.
-   Basis: [Investigate](#investigate).

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
    source: std's Windows error decoding; unverified here; if false, Windows
    keeps a message and a non-141 exit.

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

`PENDING`.

## Verify

`PENDING`.

## Learn

### Technical

`PENDING`.

### Process

`PENDING`.

## Retention and promotion

`PENDING`.

## Archive readiness

`PENDING`.

## Terminal record

### Summary

`PENDING`.

### Gate basis

`PENDING`.

## Stop record

`NONE`.
