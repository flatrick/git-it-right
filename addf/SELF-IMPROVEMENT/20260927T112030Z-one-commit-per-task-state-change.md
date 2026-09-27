# Self-improvement: `one-commit-per-task-state-change`

**Date (UTC):** `2026-09-27T11:20:30Z`

## Trigger

The Task `fixup-modes-and-picker` went through
`DEFINE -> UNDERSTAND -> INVESTIGATE -> DECIDE -> IMPLEMENT -> VERIFY -> LEARN -> COMPLETED`
without a single commit until it was complete. Its whole traversal then landed in three commits:
the feature (`f7e2de1`), a client guide (`64fa419`), and the archived Task with its spec delta
(`60502e5`). The State path lists eight States, but git holds none of the intermediate
repository states: nothing shows the Task as it stood when DEFINE was agreed, when DECIDE
chose the split design, or when IMPLEMENT's deviation from that design was found. The
operator asked that every State change be a full commit, so each step can be traced.

## What changed

`skills/work-control.md` now requires, in Work and transition, one commit per State change:
creating a Task in `DEFINE`, every transition, and the terminal one. The commit comes right
after the cursor update and before destination-State work, and leaves nothing uncommitted.
Work from the previous State may be committed first under its own commit types, so
Conventional Commit types and the changelog stay meaningful; the transition commit holds the
cursor update and names the Task and transition. Its Exit check gained a matching item.
`skills/stewardship.md` states that the terminal checkpoint is committed as that transition,
with archival in the same commit or the next.

**Still missing: enforcement.** The rule exists only as text instructions in two Skills.
Nothing checks it: `check-capsule` validates files, not history; no git hook, test or CI job
looks at it; and nothing stops an agent from skipping it. An agent working from these same
instructions produced the unrecorded traversal in the Trigger. Until something checks it, the
rule holds only as long as whoever runs the Task remembers it. Candidates, not decided: have
`check-capsule --include-archive` compare each archived Task's State path with the commits that
changed its `TASK.md`, or have a `pre-push` check refuse a push that leaves a Task's latest
State change uncommitted.

## Files touched

-   `skills/work-control.md` — a SHALL block in Work and transition requiring one commit per
    State change; an Exit check item.
-   `skills/stewardship.md` — the terminal checkpoint is committed as the terminal transition.

## Why

State path says which transitions happened; only commits preserve what the repository looked
like at each of them, which is what a reviewer needs to judge a gate after the fact. The
operator's request in the conversation that completed `fixup-modes-and-picker`
(`history:tasks/fixup-modes-and-picker/TASK.md`) is the source.
