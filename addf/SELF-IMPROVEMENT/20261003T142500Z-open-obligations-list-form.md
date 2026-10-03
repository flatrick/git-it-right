# Self-improvement: `open-obligations-list-form`

**Date (UTC):** `2026-10-03T14:25:00Z`

## Trigger

Creating the Task `unfoldable-fixup-targets`, its three Open obligations were written as a bullet list under `**Open obligations:**`, and `scripts/check-capsule` failed with `TASK_OBLIGATIONS: Open obligations field is missing`.
The checker reads the field's value from its own line, so a list that starts on the next line counts as empty.
`templates/TASK-RECORD.md` shows only a one-line placeholder and does not say how to write several obligations; the archived Tasks all end at `NONE`, so none shows a form either.

## What changed

`NONE`. This entry records friction only.
The Task now starts the value on the label line, numbering the items `(1)`, `(2)`, `(3)` on continuation lines.

## Why

Like `20261003T121151Z-index-state-form-recurred.md`, a format the checker enforces is stated nowhere else.
See `history:tasks/unfoldable-fixup-targets/TASK.md`, Resume.
