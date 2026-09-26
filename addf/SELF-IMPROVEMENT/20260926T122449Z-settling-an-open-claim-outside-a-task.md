# Self-improvement: `settling-an-open-claim-outside-a-task`

**Date (UTC):** `2026-09-26T12:24:49Z`

## Trigger

Settling the open claims `b001-b002-windows` and `gir-v1-windows-macos`
after a Windows run raised three questions addf does not answer:

-   Where a standalone Verification of an open claim lives when no Task
    owns it. `templates/VERIFICATION.md` allows a standalone artifact but
    names no folder; `evidence/` was chosen because `check-capsule` checks
    it.
-   What retiring a settled open claim means. `templates/OPEN-CLAIM.md`
    says to retire it "under Stewardship's normal archive rules", which
    cover Task bundles and Knowledge but not an open claim with no Task.
    It was moved to `.archive/open-claims/`.
-   How to record an operator decision to stop verifying part of a claim
    (macOS here) without presenting it as a Verification. It went into
    the narrowed Verification's Remaining uncertainty and the retired
    claim's Basis.

## What changed

`NONE`. Friction only.

## Why

Every open claim is eventually settled or dropped, usually without a Task,
so each settlement will face the same three placement choices.
