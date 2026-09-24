---
name: <skill-name, kebab-case, matching this file's name>
description: Use when <the one condition that should make a reader load this Skill>.
---

# <Skill name>

<One or two sentences naming the responsibility this Skill owns and the
outcome it produces. Add "Paths below are relative to the framework root."
when the Skill refers to repository paths.>

## <Responsibility section>

<One section per part of the responsibility, named for the action it governs
rather than for a generic phase label. Write the procedure as prose the reader
follows, not as a form to fill in. Delegate deterministic mechanics to a
Command instead of restating them here.>

## <Further responsibility sections as the work requires>

<Use a table when a reader must choose one row — a gate result, an outcome, a
destination State. Use a list when every item applies.>

## Responsibility handoff

Load [<other Skill>](<relative path>) when <the boundary condition that hands
control over>. State what this Skill owns and what it does not, so two Skills
never claim the same decision.

## Exit check

- <observable condition that must hold before leaving this Skill>
- <one line per condition; each must be checkable by reading the artifacts,
  not by recalling the conversation>

## Invariants

- Frontmatter carries `name` and a `description` that states the activation
  condition, so a reader can route without opening the file.
- One Skill owns one responsibility. When two Skills would both govern a
  decision, move the boundary rather than duplicating the procedure.
- Do not duplicate another Skill's procedure, schema, or rationale; link to
  it and name the boundary instead.
- Keep the Skill small enough to load for a single action. Detail that only
  one action needs belongs in that action's section, not in a preamble.
- Prefer executable enforcement over prose. When a constraint can be checked
  by tooling, the Skill should reference the check rather than restate it.
- A Rule belongs under `rules/` only once it is generic or spans more than
  one Skill or Command. A Rule tied to exactly one Skill lives inline in
  that Skill instead, as a `SHALL`/`SHALL NOT` callout with Reason and
  Scope (see `templates/RULE.md` for the shape), placed at the point in
  the responsibility section it constrains — never in a preamble, footer,
  or separate list. If a later change makes an inline Rule apply to more
  than one Skill or Command, move it out to `rules/` at that point.
- The following are optional sections, useful when the work is procedural
  enough to need them, and noise otherwise: `Prerequisites`, `Inputs`,
  `Outputs`, `Decision points`, `Progress gates`, `Stop / KILL conditions`,
  `Do not use when`. The three shipped Skills use none of them; add one only
  when a reader would otherwise get the procedure wrong.
