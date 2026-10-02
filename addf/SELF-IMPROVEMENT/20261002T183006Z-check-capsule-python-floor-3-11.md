# Self-improvement: `check-capsule-python-floor-3-11`

**Date (UTC):** `2026-10-02T18:30:06Z`

## Trigger

While moving CI to Node 24 actions (#7), `actions/setup-python` could still install Python 3.9 on Linux, but the newest 3.9 with a Windows build was 3.9.13.
Python 3.9 reached end of life in October 2025, and 3.10 does so in October 2026.
The operator chose to raise the minimum Python version for `check-capsule` to 3.11.

## What changed

`check-capsule` and the documents that state its minimum Python version now say 3.11 or newer instead of 3.9.
The CI capsule job tests that minimum, so its pin moved to `"3.11"` as well.

## Files touched

-   `scripts/check-capsule` — the docstring states Python 3.11 or newer.
-   `ADOPT.md` — the optional checker step requires Python 3.11 or newer.
-   `RECOMMENDATIONS.md` — the checker's test suite runs with Python 3.11 or newer.

## Why

The operator's decision on 2026-10-02, on branch `ci/node24-release-binaries`.
