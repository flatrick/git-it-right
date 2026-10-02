# Self-improvement: `delegated-claim-needs-unavailable-check`

**Date (UTC):** `2026-10-02T12:05:26Z`

## Trigger

The Task `doctor-fix-file-mode` was delegated to a sub-agent from `UNDERSTAND` on, with the rule that it may not push.
Its Claim `c3-os-agnostic` names "CI" in its Scope, and CI runs only after a push, so the sub-agent could not settle the Claim and stopped in `VERIFY`.
DEFINE agreed the Claims without checking that each one's settling observation is available to whoever will carry the Task.

## What changed

`NONE`.

## Why

A Claim whose check needs authority the executor lacks blocks `Complete` late, after all other work is done.
See `tasks/doctor-fix-file-mode/TASK.md`, Verify, "Stopped here".
