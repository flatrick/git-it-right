# Self-improvement: `check-capsule output encoding on Windows`

**Date (UTC):** `2026-09-24T19:27:35Z`

## Trigger

While adopting addf into this repository on Windows 11 (Python 3.14.3),
`python -m unittest discover -s addf/scripts/tests -v` failed with 1 error
out of 57 tests: `test_malformed_anchors_remain_errors`, sub-test
`<a id="cläim-name"></a>`.

`check-capsule` writes its findings with bare `print()`, so when stdout is a
pipe it encodes them with the locale's preferred encoding (`cp1252` here).
The finding echoes the non-ASCII anchor, so `ä` is written as byte `0xe4`.
`run_checker()` in `scripts/tests/test_check_capsule.py` decodes the child's
output as UTF-8, which raises `UnicodeDecodeError` in the reader thread,
leaves `result.stdout` as `None`, and then fails with
`TypeError: unsupported operand type(s) for +: 'NoneType' and 'str'`.

The same test fails the same way in the source repository
(`agent-driven-development-template` at `87f671d`), so this is not an
adoption defect. It passes with `PYTHONUTF8=1`; the full suite then runs 57
tests with `OK`.

## What changed

`NONE`. Friction only; the capsule copy stays byte-identical to its source.

## Why

The checker's output bytes depend on the host locale, so any finding that
quotes a non-ASCII path or anchor is encoded differently on Windows than on
Linux, and the suite is not portable. A likely fix is to make the checker's
output encoding explicit (for example
`sys.stdout.reconfigure(encoding="utf-8")` at startup), rather than having
callers set `PYTHONUTF8=1`. That fix is unverified.
