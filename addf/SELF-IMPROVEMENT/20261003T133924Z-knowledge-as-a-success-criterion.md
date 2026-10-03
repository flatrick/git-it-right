# Self-improvement: `knowledge-as-a-success-criterion`

**Date (UTC):** `2026-10-03T13:39:24Z`

## Trigger

The Task `ref-transaction-veto-knowledge` exists to publish Knowledge, so its success criteria are "a Knowledge file states X".
Three framework rules met awkwardly:

-   Stewardship publishes Knowledge only in the terminal checkpoint, after VERIFY, so a success Claim about a Knowledge file cannot be observed before the gate that requires it `VERIFIED`; the Task verified it "with its publication in the terminal checkpoint", as `split-refusal-advice` did for a spec delta, which no Skill describes.
-   `check-capsule` requires a promoted Knowledge Claim's text and Scope to equal the Task Claim its Basis Verification names. The success criteria could not carry the concrete findings, so the Task added one premise per finding (`f1`–`f5`) to be promoted instead. The rule lives only in the checker (`same_claim`), not in `skills/stewardship.md` or `templates/KNOWLEDGE.md`.
-   `templates/KNOWLEDGE.md` describes a Claim "about the current repository/system"; a fact about git that informs a future gir design fit only after an operator decision (`ledger.md` A5 of that Task).

## What changed

`NONE`. This entry records friction only.

## Why

A Task whose deliverable is Knowledge has to discover these constraints by trial against the checker.
See `history:tasks/ref-transaction-veto-knowledge/TASK.md`, Decide and Verify.
