---
name: work-control
description: Use when meaningful repository work starts or resumes, or when its lifecycle state, scope, strategy, or budget changes.
---

# Work control

Maintain one Task as the resumable execution cursor and gate lifecycle
transitions. Paths below are relative to the framework root.

## Activate

Use when work becomes meaningful: its outcome cannot be justified from the
request and immediate result alone, or material uncertainty, decision, risk,
deferred obligation, or reusable learning appears. Direct explanations,
inspections, and mechanical changes may remain informal while this stays false.

If the threshold becomes true, create the Task before further investigation or
change. Record the triggering observation under Material empirical premises.

## Isolate

**SHALL:** Before making any repository change once this Activate gate is
crossed — including a Ledger entry or creating a Task — ask the operator
whether this work should happen in a new isolated workspace and a new
isolated line of development, keeping the current one clean, unless the
operator already answered that question earlier in this conversation, in
which case use that answer instead of asking again.

**Reason:** An operator who did not ask for in-place changes should not
discover them mixed into their current workspace and line of development;
asking once, up front, is cheaper than an unwanted merge or a manual
cleanup later.

**Scope:** Any version-controlled repository, once this Activate gate is
crossed (Task-worthy work). Below that threshold, work stays informal per
Core and this constraint does not apply. In git, "isolated workspace"
means a new `git worktree` (or a fresh clone where worktrees are
unavailable) and "isolated line of development" means a new branch;
another VCS should use its closest equivalent (a second checkout and a new
named branch or bookmark).

If the operator answers yes, create the isolated workspace and line of
development before any other change. If the root `LEDGER.md` already holds
entries, carry its content into the new workspace as the first act after
creating it: an isolated workspace starts from a committed revision, so
uncommitted pre-Task exploration stays behind in the old one and the Ledger
handoff below would find nothing to move. If the operator answers no, or the
repository has no VCS, proceed in the current workspace. Either way, record
the decision (and, once created, the workspace/branch identity) in the
Ledger, or in the Task's Constraints once one exists — whichever is current
at that point.

## Ledger

Before a Task exists, or before its name and objective are settled, keep the
exploration that leads there from being lost. Append every question and its
answer — or its still-open state — that comes up while deciding whether work
is Task-worthy, or while shaping what a Task would even be, to `LEDGER.md` at
the framework root. It is append-only: never edit or delete a prior entry,
only add new ones. One entry per question:

- Q: `<question>`
  A: `<answer, or OPEN if still unresolved>`

Before raising a new question, check the current Ledger; do not duplicate one
it already holds an answer for.

When a Task is created (Start or resume, below), move the Ledger's
accumulated content into the new Task's bundle as an owned artifact (for
example `ledger.md`), and fold anything still material into Material
empirical premises, Open questions, or Constraints as appropriate. Then reset
the root `LEDGER.md` to its empty state from `templates/LEDGER.md`, so the
next pre-Task exploration starts clean. The copy moved into the Task bundle
is the durable record; nothing that mattered is lost by clearing the root
file.

## Define

Treat DEFINE as a conversation, not a solitary drafting pass. Its purpose is
to give the agent enough information to know where and what to investigate,
without guessing — operators routinely leave out details they assume are
obvious from context, and closing that gap is DEFINE's job, before any
Understand or Investigate effort is spent on the wrong problem.

Restate the request in your own words before acting on it. Actively look
for gaps, ambiguity, and missing constraints, and ask about them. Treat
what the operator says with default skepticism at the level of detail, even
when the general shape is almost certainly right: the agent's role is to
surface which specifics still need pressure-testing, not to accept them at
face value. Use the Ledger (above) to hold this dialogue's questions and
answers for as long as a Task's name and objective remain unsettled.

Read `SPEC.md` before settling the Task. Follow its ordered specification map
only as far as the objective requires. Record the current specification
references, the proposed delta or `NONE`, and a terminal publication value of
`PENDING` in the Task's Specification impact section. The current specification
contains completed truth; the active Task carries the proposal.

**SHALL:** Mark DEFINE's exit claim ("Outcome and bounds can guide
investigation and verification") `ESTABLISHED` only when the operator has
explicitly agreed, through this dialogue, that the objective, scope, and
success criteria are viable, actionable, and describe a desirable outcome.

**SHALL NOT:** Mark this gate `ESTABLISHED` from the agent's own assessment
alone, or satisfy it via `RISK_ACCEPTED` — unlike the general gate rules in
Work and transition, below, there is no standing authorization, in
conversation, in a durable instruction, or via `RISK_ACCEPTED`, that skips
this conversation.

**Reason:** Left unchecked, an agent guesses past ambiguity the operator
assumed was obvious from context, and the mismatch surfaces only after
investigation or implementation effort is already spent on the wrong
problem.

**Scope:** Every Task's DEFINE state, without exception. `NOT_APPLICABLE`
remains available only when DEFINE is skipped because none was needed, for
example when resuming an existing Task whose DEFINE already gated
`ESTABLISHED`.

## Start or resume

Read `INDEX.md` before either path. It is the only route by which a session
holding no conversation memory learns that a Task is already in flight;
without it, resuming depends on the agent guessing to look under `tasks/`.

For new work, create `tasks/<task-name>/TASK.md` from
`templates/TASK-RECORD.md`. Set Contract version to `2`, and set State and State
path to `DEFINE`. Set Resume at to one concrete action, question, or artifact
reference. For every obligation, name its earliest blocking gate or explicit
deferred checkpoint. Fill the Specification impact section from the current
`SPEC.md` tree. Add the Task to `INDEX.md` under Active Tasks — path, objective
phrase, and current State — in the same pass. If a pre-Task Ledger exists,
perform its handoff (above) in the same pass, and update the index's Ledger
entry to match.

When resuming:

1. Read only Resume first. State must equal the final State path entry.
2. If they differ, stop and repair the cursor from task-local evidence. Never
   infer State from artifact presence.
3. Read the objective, success criteria, constraints, Specification impact,
   current-State section, and only references needed for material Decisions and
   open obligations. Do not scan every owned artifact.
4. Conservatively classify any ambiguous obligation to the earliest gate it
   may block.

A terminal Task does not resume the same traversal. Start a new Task and
reference the archived Task when its history is relevant.

## Work and transition

Work only in the current State. A durable checkpoint is persisted repository
state from which another session can continue without replaying completed work.
Refresh Resume at after each checkpoint and before interruption or handoff.

| State | Exit claim |
| --- | --- |
| `DEFINE` | Outcome and bounds can guide investigation and verification. |
| `UNDERSTAND` | The model is sufficient to avoid uninformed action. |
| `INVESTIGATE` | Every decision-relevant uncertainty has a disposition. |
| `DECIDE` | The path is justified, feasible enough, and verifiable. |
| `IMPLEMENT` | The intended change exists and can be evaluated. |
| `VERIFY` | Evidence establishes whether completion is justified. |
| `LEARN` | Material learning and its disposition were considered. |

Before a forward transition, record the gate result and its material basis in
the applicable Task section or owned artifact. `ESTABLISHED`, `NOT_APPLICABLE`,
and `RISK_ACCEPTED` permit progress; `NOT_SATISFIED` does not. Risk acceptance
records the authorized actor, uncertain claim, accepted consequence, and scope.
If authority is unclear, the gate is `NOT_SATISFIED`. Acceptance does not make
contradictory evidence or an unverified success-critical claim established and
cannot justify Complete. DEFINE's own gate narrows this: see Define, above —
`RISK_ACCEPTED` does not apply there.

Before destination-State work, update the cursor as one change: set State,
append it to State path, replace Resume at, reconcile obligations, and bring
the Task's `INDEX.md` entry to the new State. Close settled obligations, split
composite ones, reclassify changed blocking points, and retain the rest. When
skipping a normal-flow State, record its gate result; State path records
transitions, not their justification. The index is a derived view, so a
disagreement between it and the Task is a defect in the index; the Task is
authoritative.

**SHALL:** In a version-controlled repository, record every State change —
creating a Task in `DEFINE`, each forward or reassessment transition, and
the terminal transition — as its own commit, made as soon as the cursor
update above is written and before any destination-State work. After that
commit, nothing in the workspace is left uncommitted. Work done in the
previous State may be committed first in commits of its own, with the types
its content warrants; the transition commit then holds the cursor update and
names the Task and the transition in its message, for example
`docs(addf): fixup-modes-and-picker IMPLEMENT -> VERIFY`. Consecutive
transitions made in one pass, including skipped States, still get one
commit each.

**Reason:** A Task's State path records which transitions happened, not
what the repository looked like at each. Without one commit per transition,
a reader cannot check out the state a gate was decided on, and a whole
traversal can collapse into one commit made at the end.

**Scope:** Every Task in a repository under version control, in the
workspace and line of development chosen under Isolate. A repository
without version control is exempt.

## Reassess or stop

| Finding | Destination |
| --- | --- |
| Working model or context is inadequate | `UNDERSTAND` |
| A material uncertainty has a viable discriminating probe | `INVESTIGATE` |
| The strategy is invalid and evidence permits a new choice | `DECIDE` |
| Implementation is defective but the Decision remains justified | `IMPLEMENT` |
| A success claim is unsettled but the strategy remains justified | `VERIFY` |

Budget or KILL-condition exhaustion selects no destination. Stop the current
strategy, then use the table. If no continuation is justified, preserve the
active cursor only for the handoff to Stewardship. Stewardship selects the Stop
outcome and publishes the terminal cursor before archival. Also stop phase work
when the cursor cannot be repaired, the gate cannot be defined, required
authority or dependency is absent, an authorized actor withdraws the objective,
or contradiction invalidates the current path. Never report Complete in these
conditions.

## Responsibility handoff

Load [Evidence and verification](evidence-and-verification.md) when a claim
must be established, contradicted, or deferred. Load
[Stewardship](stewardship.md) before Complete, Stop, archival, promotion, or
another change to current truth. Work control owns the active cursor;
Stewardship owns its terminal publication.

## Exit check

- State equals the final State path entry.
- In a version-controlled repository, the latest State change is committed
  and nothing is left uncommitted in the workspace.
- Resume at is concrete, or `NONE` only in a terminal State.
- Each obligation names a blocking gate or deferred checkpoint.
- The next transition is justified and no contradiction is hidden.
- Only context needed for the current action was loaded.
- No pre-Task Ledger content was silently discarded: it is either still
  accumulating at the root, or was moved into the started Task's bundle. If
  an isolated workspace was created, the Ledger came with it.
- Isolation was asked about (or already answered earlier in this
  conversation) before this pass's first repository change.
- The Task names the current specification it consumed, its proposed delta or
  `NONE`, and its terminal publication state.
- `INDEX.md` names every active Task at its current State, and names no
  Task that is not active.
