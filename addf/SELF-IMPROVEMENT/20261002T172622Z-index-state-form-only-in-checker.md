# Self-improvement: `index-state-form-only-in-checker`

**Date (UTC):** `2026-10-02T17:26:22Z`

## Trigger

During `sourcegit-types`, the active Task's `INDEX.md` entry ended with its State as `` `VERIFY` ``.
`skills/work-control.md` asks for the entry's "path, objective phrase, and current State", with no form.
`scripts/check-capsule` only accepts `` State: `<state>` `` within the entry's first three lines, so the hosted `capsule` job failed on Ubuntu and Windows with `INDEX_STATE: ... index says ?`.
The check was not run locally before pushing, so the mismatch surfaced only in CI.

## What changed

`NONE`. This entry records friction only.

## Why

It cost a push and a hosted CI run for a format the Skill does not state.
See `history:tasks/sourcegit-types/TASK.md`, Verification `c6-no-regression`.
