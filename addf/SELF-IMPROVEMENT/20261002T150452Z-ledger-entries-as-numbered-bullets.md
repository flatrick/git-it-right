# Self-improvement: `ledger-entries-as-numbered-bullets`

**Date (UTC):** `2026-10-02T15:04:52Z`

## Trigger

A Ledger entry was a `- Q:` list item with its `A:` on an indented continuation line.
Rendered as Markdown, the answer joins the question's paragraph, so a thread is very hard to read.
Entries were also named only by their position, which a reader had to count.
The operator converted `ledger/fixup-review-20261002.md` by hand to numbered, separate bullets (`- Q1:` / `- A1:`) and asked that this format be used from now on.

## What changed

The Ledger entry format is now a numbered question and a numbered answer, each its own list item.
The number is still the entry's position among the thread's questions, so existing `D:` lines keep their meaning.

## Files touched

-   `skills/work-control.md` — the entry example and its naming sentence use `Q<n>` / `A<n>` as separate list items; "settled" now refers to every question rather than every `Q:` entry.
-   `templates/LEDGER-THREAD.md` — the same format in the preamble and the commented entry example.
-   `skills/stewardship.md` — "settled" refers to every question rather than every `Q:` entry.

## Why

The operator's instruction on 2026-10-02, after the `doctor-fix-file-mode` Task completed.
Threads already archived and Task `ledger.md` files keep their old format; they are history.
The checker tests in `scripts/tests/test_check_capsule.py` still use the old format in fixtures that test placeholder detection, which does not depend on the entry format.
