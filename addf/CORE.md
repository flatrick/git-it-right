---
name: core
description: Load first, every session. Routes to the responsibility that governs the current action.
---

# Core

Paths below are relative to the framework root.

## Is this Task-worthy?

Work becomes meaningful when its outcome cannot be justified from the
request and immediate result alone, or when material uncertainty, a
decision, a risk, a deferred obligation, or reusable learning appears. Below
that threshold, work stays informal: answer, inspect, or make the mechanical
change directly, without loading a responsibility Skill. At or above it,
load [Work control](skills/work-control.md) before further investigation or
change.

Before the threshold is crossed, or before a Task's name and objective are
settled, do not let that exploration evaporate: append every question and
answer it raises to the pre-Task Ledger (`LEDGER.md`, at the framework
root). See Work control's Ledger section for the append-only contract and
what happens to it once a Task starts.

## The lifecycle

A Task moves through `DEFINE -> UNDERSTAND -> INVESTIGATE -> DECIDE ->
IMPLEMENT -> VERIFY -> LEARN`, ending at `COMPLETED` or a Stop outcome
(`BLOCKED`, `IMPOSSIBLE`, `DEFERRED`, `ABANDONED`, `FAILED`). A Task's own
State is the only authoritative cursor; never infer it from artifact
presence or conversation memory.

## Route to a responsibility

Load exactly the Skill that governs the current action, and nothing a
current action does not need:

- [Work control](skills/work-control.md) — starting or resuming a Task (read
  [the index](INDEX.md) first; work may already be in flight, and a Task's
  State is not discoverable any other way), transitioning its lifecycle
  State, reading the current specification, deciding whether work meets the
  Task-worthy threshold above, maintaining the pre-Task Ledger, or deciding
  whether to isolate work in a new workspace and line of development.
- [Evidence and verification](skills/evidence-and-verification.md) — a
  material Claim must be established, contradicted, or deferred.
- [Stewardship](skills/stewardship.md) — `Complete`, a Stop outcome,
  archival, Knowledge promotion, or correcting misleading current material.

Each Skill hands off to the others at its own boundary. Do not duplicate
their procedures, schemas, or rationale here.

## Invariant

An assumption, a green result, and an accepted risk are never a verified
Claim. Only a Verification changes Claim State.
