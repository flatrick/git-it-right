# Self-improvement: `numbered-disposition-lines`

**Date (UTC):** `2026-10-02T15:10:26Z`

## Trigger

After Ledger entries were numbered (`20261002T150452Z-ledger-entries-as-numbered-bullets.md`), a disposition line such as `- D: Q1, Q8 -> ...` began with the same `Q1` as the entry line `- Q1: ...`.
The operator read the two lines as two items with the same ID.

## What changed

Disposition lines are numbered by their position among the thread's dispositions and name the entries they settle after the word `settles`: `- D2: settles Q1, Q8 -> taken by Task split-preserves-index`.
Entry IDs are unchanged, so references to them from Task ledgers, Task records and `INDEX.md` keep their meaning.

## Files touched

-   `skills/work-control.md` — disposition lines are numbered `D<n>` and name the entries they settle.
-   `templates/LEDGER-THREAD.md` — the same format in the preamble and the commented example.

## Why

The operator's instruction on 2026-10-02.
`ledger/fixup-review-20261002.md` was converted in the same pass; its seven dispositions are now `D1` to `D7`.
