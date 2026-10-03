# Self-improvement: `promotion-needs-factual-claim`

**Date (UTC):** `2026-10-03T15:55:00Z`

## Trigger

Completing the Task `unfoldable-fixup-targets`, its success criterion `c1-autosquash-behaviour` was worded as an observation to make ("it is observed whether ... folds"), not as the fact it would establish.
When its result was promoted, `scripts/check-capsule` failed with `CLAIM_BASIS: settled Basis must resolve to an explicit Verification conclusion`: a Knowledge Claim must match, in its Claim and Scope text, a Task Claim that the Basis Verification concludes on (`same_claim`).
The fix was a new premise `p2-autosquash-merge-targets` stating the result, with its own Verification, added at the terminal checkpoint.

## What changed

`NONE`. This entry records friction only.

## Why

Neither `skills/stewardship.md`'s Promote Knowledge section nor `templates/KNOWLEDGE.md` says that the Knowledge Claim must repeat a Task Claim word for word; only the checker does.
A criterion written as "observe X" is natural in DEFINE, when the result is unknown, so this will recur for any investigative criterion whose result is worth keeping.
See `history:tasks/unfoldable-fixup-targets/TASK.md`, Retention and promotion.
