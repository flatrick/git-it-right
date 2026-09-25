# Self-improvement: `check-capsule-writes-utf-8`

**Date (UTC):** `2026-09-24T19:45:00Z`

## Trigger

Fix for the friction recorded in
[check-capsule output encoding on Windows](20260924T192735Z-check-capsule-output-encoding-on-windows.md).

## What changed

`check-capsule` now reconfigures stdout and stderr to UTF-8 before any
output, and the suite gained a regression test that forces a `cp1252`
output encoding. Both files are byte-identical to
`agent-driven-development-template` commit `3944c24` on branch
`fix/check-capsule-utf8-output`, where the fix was made first.

## Files touched

-   `scripts/check-capsule` — `main()` reconfigures both streams to UTF-8.
-   `scripts/tests/test_check_capsule.py` —
    `test_output_is_utf8_regardless_of_locale_encoding` added.

## Why

With the fix in the checker, the CI `capsule` job no longer needs
`PYTHONUTF8=1`, and the suite passes on Windows without any caller-side
setting.
