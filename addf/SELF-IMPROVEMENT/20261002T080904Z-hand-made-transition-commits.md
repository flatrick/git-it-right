# Self-improvement: `hand-made-transition-commits`

**Date (UTC):** `2026-10-02T08:09:04Z`

## Trigger

While running the Task `ledger-threads-and-concurrent-tasks`, an agent wrote each State change and commit by hand with shell commands, and two of them went wrong.

-   `3aab4e9` committed an empty `INDEX.md`. A script truncated the file before reading it; `scripts/check-capsule` then failed, but the next command in the `&&` chain was `cat` of the log, which succeeded, so the commit ran.
-   `922e673` committed only evidence logs under a message saying it recorded verifications. The script that should have written them failed, and `set -e` did not stop the commit that followed.

Both were corrected in later commits (`12d7283`, `fd87e22`), since history is not rewritten.
Separately, this repository's `commit-msg` hook (gir's `header-length`, 72 characters) rejected `docs(addf): ledger-threads-and-concurrent-tasks UNDERSTAND -> INVESTIGATE`.
The example form in `skills/work-control.md` (`docs(addf): <task> <from> -> <to>`) does not fit a Task name longer than about 34 characters for that transition.

## What changed

`NONE`.
The agent switched to a local script that edits the cursor and index, runs `scripts/check-capsule`, and commits only when it passes, writing `<from>-><to>` without spaces when the header would exceed 72 characters.
That script was not added to `scripts/`; whether to add one, and whether to shorten the recommended message form, is the operator's call.

## Files touched

-   `NONE`

## Why

Work and transition requires one commit per State change, with nothing left uncommitted.
Doing that by hand means many small edit-check-commit sequences, and each sequence is a chance to commit despite a failing check.
A transition helper in `scripts/` would make "commit only if the checker passes" structural instead of something every agent has to get right in shell.
