---
name: evidence-and-verification
description: Use when a material empirical claim is unverified, disputed, contradictory, decision-relevant, success-critical, or due for deferred verification.
---

# Evidence and verification

Establish only what the available evidence proves. Never turn an assumption,
green result, or accepted risk into a verified Claim. Paths below are relative
to the framework root. Use the current [reference contract](../REFERENCES.md)
for durable targets.

## Define the Claim

Record one scoped, falsifiable proposition with State, material Scope,
Consequence if false, and Basis. The initial State is `UNVERIFIED`; Basis states
why no Verification exists. A reported behavior remains unverified until you
establish it. Requirements, preferences, and authorized Decisions are inputs,
not empirical Claims.

Split a Claim when its parts can receive different states and tracking them
separately changes a decision, obligation, or conclusion. Do not use a partial
state. Partial support leaves the broader Claim `UNVERIFIED`. Adequate disproof
of a required part makes the broader Claim `REFUTED`.

## Choose the check

1. State what observation would settle the Claim within its scope.
2. For a material consequence, name plausible competing explanations and the
   observations each predicts.
3. Inspect existing evidence when it is adequate and reproducible. Otherwise,
   use the smallest Probe that distinguishes the material explanations. Use
   `templates/PROBE.md` when the method, budget, or reasoning needs a record.
4. Define the expected observations, limitations, and budget before execution.

Stop the current method when its budget or KILL condition is reached. A command
that ran successfully produced an Observation; it did not prove the expected
mechanism caused the result.

## Record evidence

Keep these parts distinct:

| Part | Record |
| --- | --- |
| Observation | The fact directly observed, source, method, inputs, material context, and limitations. |
| Relationship | `SUPPORTS`, `CONTRADICTS`, or `INCONCLUSIVE`, with reason and scope. |
| Interpretation | What the Observation implies and which alternatives remain. |
| Verification | The conclusion after considering all relevant relationships. |

Keep Evidence inline when it has one use, is cheap to reproduce, and remains
readable. Use `templates/EVIDENCE.md` for an expensive, destructive, external,
hard-to-reproduce, large, reused, or materially contradictory Observation set.
One Evidence artifact covers one acquisition event or coherent Observation set.

Treat archived material as historical context even when search returns it.
Reverify its empirical Claims before current use. Never use retracted or
known-false material as Evidence.

## Defer when the Claim cannot yet be checked

Record `DEFERRED_VERIFICATION`, why verification is unavailable, the earliest
checkpoint, the settling observation, the consequence if false, and the work
blocked at that checkpoint. Keep the Claim `UNVERIFIED` and add an open Task
obligation.

Risk acceptance is separate. It permits only the work named in its scope; do
not infer permission to release or Complete. It does not change Claim state or
permit Complete with an unverified success-critical Claim.

## Conclude and publish the state

Use the shape in `templates/VERIFICATION.md` when a material Claim is evaluated.
The Verification may be a standalone artifact or a structured section beneath
the Task's `## Verify` heading. Both forms name the Claim, method, Evidence
considered, conclusion, and limitations. Consider all relevant supporting,
contradicting, and inconclusive Evidence, including unexpected mechanisms and
evidence against the preferred result.

Choose exactly one conclusion:

| State | Use when |
| --- | --- |
| `VERIFIED` | Adequate evidence establishes the Claim within its scope. |
| `REFUTED` | Adequate evidence establishes that the Claim does not hold. |
| `DISPUTED` | Material supporting and contradicting Evidence remains unresolved. |
| `UNVERIFIED` | The available Evidence does not establish either result. |

Before a change or reevaluation can invalidate a non-`UNVERIFIED` state, set
the Claim to `UNVERIFIED` and put the pending reason or check in Basis. Then
write the Verification. Publish its conclusion by updating the Claim's State
and Basis together; Basis points to the Verification's explicit conclusion
anchor. A command, changed file, authority statement, risk acceptance, or
manual inspection is Evidence or input, not a direct Basis for a settled
Claim. If Basis is broken or the Verification cannot independently support
re-derivation, treat the Claim as `UNVERIFIED`. Do not reuse the Claim anchor
after changing its proposition or material scope.

Return the conclusion, remaining uncertainty, and blocking or deferred
obligations to [Work control](work-control.md). Work control owns the cursor.
Load [Stewardship](stewardship.md) for promotion, retraction, archival, or
another change to current repository truth.

## Exit check

- The Claim is one proposition and its scope matches the observations.
- Observations contain no interpretation or conclusion.
- Each relationship states its direction, reason, and scope.
- The Verification explains contradictory and inconclusive Evidence.
- State and Basis agree with the current applicable Verification.
- No green result, authority statement, or accepted risk is presented as proof.
