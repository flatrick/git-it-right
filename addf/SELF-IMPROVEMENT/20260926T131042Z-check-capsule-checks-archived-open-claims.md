# Self-improvement: `check-capsule-checks-archived-open-claims`

**Date (UTC):** `2026-09-26T13:10:42Z`

## Trigger

Retiring `b001-b002-windows` and `gir-v1-windows-macos` to
`.archive/open-claims/`, as recorded in
[settling an open claim outside a Task](20260926T122449Z-settling-an-open-claim-outside-a-task.md),
left their Basis references unchecked. `--include-archive` loaded only
`.archive/tasks/`, so a retired claim's settled Basis could point at a
missing or unrelated Verification and the checker would still pass.

## What changed

With `--include-archive`, `check-capsule` now also loads and checks
`.archive/open-claims/`, so the same reference, placeholder and
Claim-Basis checks run there as on current open claims. The default run
is unchanged. The change is local to this repository and has not been
made upstream in `agent-driven-development-template`.

## Files touched

-   `scripts/check-capsule` — `archived_checked` decides which archived
    files `--include-archive` loads and checks, now including
    `.archive/open-claims/`.
-   `scripts/tests/test_check_capsule.py` —
    `test_include_archive_checks_retired_open_claims` added.

## Why

A retired open claim keeps its Basis as provenance, and the checker is
the only thing that re-derives it. Without the check, a broken reference
there is caught only by reading it.
