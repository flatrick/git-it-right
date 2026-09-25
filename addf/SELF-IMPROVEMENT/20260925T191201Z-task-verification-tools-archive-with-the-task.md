# Self-improvement: `task-verification-tools-archive-with-the-task`

**Date (UTC):** `2026-09-25T19:12:01Z`

## Trigger

The `gir-v1-spec` Task built two reusable checkers inside its bundle:
`trace.py`, which checks that every spec requirement is traced to a test
that asserts it, and `inventory.py`, which lists user-visible identifiers
missing from the spec. Both keep the published specification honest after
the Task ends. Stewardship archives them with the bundle, and they locate the
repository from their own path, so after archival they neither run against
the current `spec/` nor appear in any current discovery path. addf has no
destination for a Task-built verification tool whose value outlives the
Task. `scripts/` holds generic capsule mechanics only, and Knowledge holds
Claims, not tools.

The same Task also hit a smaller friction: `check-capsule` rejected an
`Open obligations` value written as a bulleted list under the field, with
`TASK_OBLIGATIONS: Open obligations field is missing`. It accepts only a value
on the field's own line, which `templates/TASK-RECORD.md` does not say.

## What changed

`NONE`. Friction only.

## Why

A requirement-to-test trace is the evidence that the specification is
verified. Losing the tool at archival means the next Task that changes
`spec/` has to rebuild it or skip the check. See
`history:tasks/gir-v1-spec/TASK.md`, sections Learn and Retention and
promotion.
