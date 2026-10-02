---
name: work-control
description: Use when meaningful repository work starts or resumes, or when its lifecycle state, scope, strategy, or budget changes.
---

# Work control

Maintain each active Task as its own resumable execution cursor and gate lifecycle transitions.
Paths below are relative to the framework root.

## Activate

Use when work becomes meaningful: its outcome cannot be justified from the
request and immediate result alone, or material uncertainty, decision, risk,
deferred obligation, or reusable learning appears. Direct explanations,
inspections, and mechanical changes may remain informal while this stays false.

If the threshold becomes true, create the Task before further investigation or
change. Record the triggering observation under Material empirical premises.

## Isolate

**SHALL:** Before a new Task's first repository change, including creating the Task, ask the operator whether this Task should happen in a new isolated workspace and a new isolated line of development, keeping the current one clean.
Ask once per Task: an answer given for another Task, in this conversation or earlier, does not carry over.

**Reason:** An operator who did not ask for in-place changes should not
discover them mixed into their current workspace and line of development;
asking once, up front, is cheaper than an unwanted merge or a manual
cleanup later.
Several Tasks can be active at once, and each may belong in a different place, so one Task's answer cannot stand for another's.

**Scope:** Every new Task in a version-controlled repository.
Writing to a Ledger thread does not trigger this question: a thread is pre-Task material, committed in the workspace where it is written.
Below the Activate threshold, work stays informal per Core and this constraint does not apply.
In git, "isolated workspace"
means a new `git worktree` (or a fresh clone where worktrees are
unavailable) and "isolated line of development" means a new branch;
another VCS should use its closest equivalent (a second checkout and a new
named branch or bookmark).

If the operator answers yes, first commit every Ledger thread the Task draws on, then create the isolated workspace and line of development before any other change.
An isolated workspace starts from a committed revision, so an uncommitted thread would stay behind and the Ledger handoff below would find nothing to take.
If the operator answers no, or the repository has no VCS, proceed in the current workspace.
Either way, record the decision, and once created the workspace and branch identity, in the Task's Constraints.

## Ledger

Before a Task exists, or before its name and objective are settled, keep the exploration that leads there from being lost.
Write it to a Ledger thread: one file per exploration under `ledger/`, named with a short kebab-case slug and created from `templates/LEDGER-THREAD.md`.
One exploration, such as a code review, can lead to several Tasks or to none, so a thread belongs to its exploration, not to a Task.
Append every question and its answer, or its still-open state, that comes up while deciding whether work is Task-worthy or while shaping what a Task would be.
A thread is append-only: never edit or delete a prior entry, only add new ones at the end.
One entry per question:

- Q`<n>`: `<question>`
- A`<n>`: `<answer, or OPEN if still unresolved>`

Number each entry by its position among the thread's questions, `Q1` being the first, and give its answer the same number; append-only keeps that position stable.
Write the question and the answer as separate list items, so a rendered answer does not run into its question's paragraph.
Settle an entry by appending a disposition line that names it, never by editing the entry:

- D: `<Q positions, or all>` -> `<taken by Task name | rejected: reason | moved to place>`

Before raising a new question, check the open threads `INDEX.md` lists; do not duplicate one a thread already answers.
Add a question to the thread whose exploration it belongs to, or start a new thread and list it in `INDEX.md` in the same pass.

When a Task is created (Start or resume, below), it takes the thread entries it was shaped from:

- If it takes every entry of a thread, move the thread file into the Task's bundle as `ledger.md` and remove the thread from `INDEX.md`. No disposition line is needed.
- Otherwise, copy the entries it takes into the bundle's `ledger.md`, unchanged and naming the thread and their positions, and append a disposition line to the thread naming the Task. The copy keeps the bundle self-contained after archival.

Fold anything still material into Material empirical premises, Open questions, or Constraints.
Once the Task exists, its own `ledger.md` holds the rest of its DEFINE dialogue.

A thread whose every question has a disposition is settled; load [Stewardship](stewardship.md) to archive it.
Entries no Task has taken stay open in their thread, listed through `INDEX.md`, until each is taken, rejected, or moved.

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
face value.
Hold this dialogue's questions and answers in a Ledger thread (above) until the Task exists, then in the Task's own `ledger.md`.

Read `SPEC.md` before settling the Task. Follow its ordered specification map
only as far as the objective requires. Record the current specification
references, the proposed delta or `NONE`, and a terminal publication value of
`PENDING` in the Task's Specification impact section. The current specification
contains completed truth; each active Task carries its own proposal.

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

For new work, first count the active Tasks `INDEX.md` lists.
At most three Tasks may be active at once; with three active, do not create another.
Tell the operator, and let them choose which active Task to finish or stop first.

Otherwise, create `tasks/<task-name>/TASK.md` from
`templates/TASK-RECORD.md`. Set Contract version to `2`, and set State and State
path to `DEFINE`. Set Resume at to one concrete action, question, or artifact
reference. For every obligation, name its earliest blocking gate or explicit
deferred checkpoint. Fill the Specification impact section from the current
`SPEC.md` tree. Add the Task to `INDEX.md` under Active Tasks — path, objective
phrase, and current State — in the same pass.
Perform the Ledger handoff (above) for the thread entries the Task takes in the same pass, and update the index's Ledger entries to match.

Compare the new Task's Specification impact, and the files it expects to change, with every other active Task.
Where they overlap, record the overlap and the order in both Tasks' Constraints.
The order is the order of spec publication: the earlier Task publishes its delta and terminalizes first.
The later Task may work in parallel, but re-reads the current specification after the earlier one terminalizes, before its own `VERIFY`.

When resuming:

1. If more than one Task is active and the operator has not named one, list the active Tasks from `INDEX.md` and ask which to resume.
2. Read only Resume first. State must equal the final State path entry.
3. If they differ, stop and repair the cursor from task-local evidence. Never
   infer State from artifact presence.
4. Read the objective, success criteria, constraints, Specification impact,
   current-State section, and only references needed for material Decisions and
   open obligations. Do not scan every owned artifact.
5. Conservatively classify any ambiguous obligation to the earliest gate it
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
another change to current truth. Work control owns each active cursor;
Stewardship owns its terminal publication.

## Exit check

- State equals the final State path entry.
- In a version-controlled repository, the latest State change is committed
  and nothing is left uncommitted in the workspace.
- Resume at is concrete, or `NONE` only in a terminal State.
- Each obligation names a blocking gate or deferred checkpoint.
- The next transition is justified and no contradiction is hidden.
- Only context needed for the current action was loaded.
- No Ledger entry was silently discarded: each is still open in its thread, settled by a disposition line, or moved with its whole thread into a Task's bundle.
- Every thread a new isolated workspace drew on was committed before the workspace was created.
- Each new Task's Isolate question was asked before that Task's first repository change.
- At most three Tasks are active, and active Tasks that overlap record the overlap and their order in their Constraints.
- The Task names the current specification it consumed, its proposed delta or
  `NONE`, and its terminal publication state.
- `INDEX.md` names every active Task at its current State, and names no
  Task that is not active.
- `INDEX.md` lists every thread under `ledger/`, and no other file as a thread.
