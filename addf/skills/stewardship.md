---
name: stewardship
description: Use before Complete, a Stop outcome, archival, Knowledge promotion, or correcting misleading current material.
---

# Stewardship

Own terminalization, archival, retention, promotion, and correction of
current truth. Paths below are relative to the framework root.

## Gate the terminal transition

Before `Complete`, confirm every success Claim in Success criteria shows
`State: VERIFIED` with a Basis naming an applicable Verification. A Claim
that is merely asserted, `UNVERIFIED`, `REFUTED`, or `DISPUTED` blocks
`Complete`; return the Task to Work control instead.

For Stop, confirm Work control has stopped phase work and either established
that no continuation is justified or recorded an authorized withdrawal. Budget
or KILL-condition exhaustion alone selects no outcome; reassess first.

Select the first applicable outcome in this order:

| Outcome | Required finding |
| --- | --- |
| `ABANDONED` | An authorized actor withdrew the objective. |
| `IMPOSSIBLE` | Evidence establishes that authoritative constraints cannot be satisfied. |
| `DEFERRED` | An authorized decision postpones a still-valid objective to a named condition or checkpoint. |
| `BLOCKED` | A required dependency or authority is absent and no authorized postponement exists. |
| `FAILED` | Work was attempted, the objective remains unmet, and permitted strategies or budget are exhausted without an earlier outcome applying. |

The order resolves overlaps. Stewardship selects the outcome from the recorded
finding; it does not decide whether phase work should continue.

## Write the terminal record

For `Complete`, write Terminal record: Summary and Gate basis, including the
status of every success Claim and any deferred obligation the Task still
carries.

For a Stop outcome, write Stop record: the prior active State, the trigger,
supporting evidence, budget or resources consumed, unresolved obligations and
uncertainty, learning, why continuing was unjustified, and any condition that
could justify a later traversal. A later traversal starts a new Task and
references the archived one; it never resumes the same bundle.

Before archival, publish one durable terminal checkpoint. For `Complete`, apply
the Task's verified specification delta and replace Terminal publication with
the published `framework:` references, or record `NONE` when the Task has no
delta. A Stop outcome publishes no proposed delta. In the same checkpoint, set
State to the selected outcome, append it to State path, set Resume at to `NONE`,
reconcile obligations, and update the current-artifact index. Do not move a
bundle while its State still names an active phase.

## Consider Learning

Read Learn. If no material technical or process learning exists, record that
conclusion explicitly in Retention and promotion rather than leaving the
section silent or inventing a learning to fill it. Route each material
Learning to the smallest destination that fits: tooling or executable
enforcement, a Command, a Skill, a Rule, Knowledge, Core only in an
exceptional case, or no permanent change. Do not promote an incident to
guidance automatically; apply the Promotion check in `templates/LEARNING.md`.

## Carry forward open Claims

Before terminalization (`Complete` or any Stop outcome, explicitly
including `DEFERRED` and `BLOCKED`), for every Success-criteria or
Material-empirical-premises Claim not ending `VERIFIED` and promoted to
Knowledge, ask explicitly per Claim: does this Claim's resolution remain
decision-relevant beyond this Task?

If yes, create or refresh a file under `open-claims/` from
`templates/OPEN-CLAIM.md` and list it in `INDEX.md`'s `## Open Claims`
category in the same pass. If no, the Claim archives with the Task bundle
as today; this is the common case and changes nothing about it.

## Promote Knowledge

Promote a Claim only when it is `VERIFIED` with an applicable Verification
and its validity is expected to outlive this Task and inform future
decisions. Do not promote an ordinary Task-scoped fact with no reuse value.

For each promoted Claim, create one file under `knowledge/` from
`templates/KNOWLEDGE.md`: one coherent subject, the Claim's State and Basis
pointing at the originating Verification, evidence anchors sufficient for
independent re-derivation, and `verified_at` when State is `VERIFIED`.
Promotion happens during terminalization, immediately before archival.

Answer the promotion test at `templates/TASK-RECORD.md`'s "Retention and
promotion" structure directly, per Claim, at the point of decision — not
from a recalled paraphrase of it.

## Correct current truth

When evidence contradicts or can no longer establish a current Knowledge
Claim, update its State (`REFUTED`, `DISPUTED`, or `UNVERIFIED` pending
re-check) and Basis to the applicable Verification. Remove it from current
retrieval only when no accepted Claim remains to justify it there. Never
leave a `VERIFIED` Knowledge file beside evidence that contradicts it.

When a current Skill, Rule, Command, or other non-Task artifact no longer
matches observed repository behavior, correct it, archive it, or delete it.
Do not leave incorrect material presented as current truth beside a warning;
a reader following ordinary discovery must not be misled.

When an accepted product requirement changes, update `SPEC.md` or one of its
reachable specification documents. Archived Tasks remain provenance. Do not
append current corrections to them or require a reader to replay their order.

## Archive or delete

Archive anything that was current truth a reader could reasonably expect to
reconstruct later: terminal Task bundles, superseded Decisions, and retracted
Knowledge with real provenance. Move it beneath `.archive/` at the matching
relative path; its logical identity does not change.

Delete only material with no independent informational value beyond what an
archived or canonical copy and VCS history already retain: an exact duplicate
superseded by a moved canonical copy, a disposable fixture, or scratch
material. When in doubt, archive rather than delete.

## Confirm archive readiness

Before moving a Task bundle beneath `.archive/`, confirm it is
self-contained: every Task-internal reference is an ordinary relative link
per the [reference contract](../REFERENCES.md), and any reference to material
outside the bundle is supplemental evidence, not required for reconstructing
the Task's outcome. Do not archive a bundle that silently depends on a
current-only artifact for its own reasoning to make sense.

## Update the index

Remove a Task from the current-artifact index the same time it becomes
terminal, regardless of which terminal outcome it reached. Add or remove
promoted Knowledge, carried-forward or retired open Claims, corrected
Skills, Rules, or Commands, and any other current-artifact change in the
same pass. The index is a derived view; a stale entry is a discovery
defect; the archived or corrected source artifact remains authoritative.

The index has two writers, and they do not overlap. Work control adds an
active Task and keeps its State current while the Task lives; Stewardship
removes it at terminalization and owns every promotion and correction entry.

## Responsibility handoff

Load [Work control](work-control.md) to resume execution after a Stewardship
pass, or when a terminal or Stop decision itself is still unsettled. Load
[Evidence and verification](evidence-and-verification.md) when a Claim named
here (a success Claim, a Knowledge Claim, or a contradiction) still needs to
be established, reevaluated, or deferred before Stewardship can act on it.
Stewardship does not itself decide Claim state or active lifecycle traversal.

## Exit check

- `Complete` occurred only with every success Claim `VERIFIED` through an
  applicable Verification.
- A Stop outcome has a Stop record naming its re-entry condition, or `NONE`.
- A stopped Task's State is its selected outcome, State path ends with that
  outcome, Resume at is `NONE`, and no unresolved obligation is hidden.
- No Task terminalizes while carrying a still-relevant `UNVERIFIED` or
  `DISPUTED` Claim that was neither resolved nor carried forward to
  `open-claims/`.
- Retention and promotion records an explicit disposition for Learning and
  for every durable Claim considered for promotion, including "no material Learning" or "no
  Claim promoted" when true.
- No current Knowledge, Skill, Rule, or Command remains beside evidence that
  contradicts it.
- A completed Task published its specification delta in the terminal
  checkpoint, or recorded that it had no delta. A stopped Task published none.
- Every archived bundle is self-contained; nothing was deleted that a future
  reader would need to reconstruct history.
- The current-artifact index reflects every change made in this pass.
