# Self-improvement: `ledger-threads-and-several-active-tasks`

**Date (UTC):** `2026-10-02T07:59:40Z`

## Trigger

A code review of `feat/fixup-modes` found ten edge cases, and the operator wanted the two most serious fixed as two Tasks.
The root `LEDGER.md` could not take that shape.
Its handoff moved every entry into the next Task's bundle and reset the file, so the other eight findings would have been buried in a Task they do not belong to, or the root could not be reset.
The Ledger was doing three jobs: pre-Task exploration, a backlog of findings nobody was working on yet, and state shared by every Task and worktree.
`skills/work-control.md` also assumed a single Task ("Maintain one Task as the resumable execution cursor"), although `INDEX.md` already listed Active Tasks as a list.
The operator noted that two Tasks at once is a case they keep running into.

## What changed

The root `LEDGER.md` is replaced by Ledger threads: one append-only file per exploration under `ledger/`.
An entry is settled by appending a `D:` line naming it.
A Task that takes a whole thread moves the file into its bundle; a Task that takes part copies those entries and appends a `D:` line.
Stewardship archives a settled thread to `.archive/ledger/`, and `INDEX.md` lists every open thread.
Up to three Tasks may be active at once.
Resuming asks which Task when more than one is active, overlapping Tasks record the overlap and their order of spec publication, and the Isolate question is asked once per Task instead of once per conversation.
`scripts/check-capsule` enforces the parts that can be checked from files: it refuses a root `LEDGER.md`, requires `INDEX.md` to list exactly the threads under `ledger/`, refuses more than three active Tasks, and checks threads in prose mode so they can quote output such as `<commit>` in inline code.

## Files touched

-   `CORE.md` — exploration goes to a Ledger thread; routing names Ledger threads.
-   `skills/work-control.md` — each active Task is its own cursor; Isolate per Task; the Ledger section describes threads, dispositions and the handoff; the three-Task limit, overlap order and the resume question; Exit check items.
-   `skills/stewardship.md` — archive a settled thread; index ownership of threads; an Exit check item.
-   `ADOPT.md` — `ledger/` is instance data; a destination with an old root `LEDGER.md` moves it into a thread.
-   `templates/LEDGER.md` — renamed to `templates/LEDGER-THREAD.md` and rewritten as a thread.
-   `templates/INDEX.md` — the Ledger section lists open threads; invariants name `ledger/`.
-   `templates/SELF-IMPROVEMENT.md`, `rules/self-improvement-log.md` — Scope names `ledger/` threads instead of `LEDGER.md` entries.
-   `scripts/check-capsule`, `scripts/tests/test_check_capsule.py` — the four checks above, each with a test.

## Why

The decisions and the operator's answers are in the Task that made the change, `ledger-threads-and-concurrent-tasks` (`history:tasks/ledger-threads-and-concurrent-tasks/ledger.md`).
Rejected alternatives: tagging entries in one root file, which keeps one shared file that conflicts across worktrees; creating the Task early instead of threads, which fails when one exploration leads to several Tasks or none; and `ROADMAP.md` or a new `backlog/` for findings nobody is working on yet.
