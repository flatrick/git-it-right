# Adopt

Manual, reviewed instructions for adding this capsule to a repository. No
tool is required. Follow this file without reading the framework's own
design history or Task bundles.

## Preflight inventory

Before copying anything, inventory the destination repository:

- Does a framework root already exist at the path you intend to use?
- For any destination path that would collide with a capsule file, does the
  destination content already match the capsule source, or has it been
  locally modified?
- Does an entry-point file already exist for the harness you are adopting
  for (for example `CLAUDE.md`, `AGENTS.md`), and what does it currently
  say?
- Can the adopting process create and replace a disposable file beneath the
  selected framework root without an approval or sandbox failure?

Do not copy anything until this inventory is complete.

## Choose the destination

Choose one repository-relative path as the framework root. This is the only
configurable value in this procedure; the default is `addf/`, with no
leading dot, so default text-search tools do not silently skip active Tasks
and Knowledge. Every reference inside the capsule resolves relative to this
root, so record the chosen path before continuing.

## Detect collisions

For each generic capsule file (`CORE.md`, `REFERENCES.md`, `ADOPT.md`,
`RECOMMENDATIONS.md`, everything under `skills/`, `templates/`, and
`rules/`, and everything under `scripts/`), compare it against the matching
destination path:

- Destination does not exist: copy it.
- Destination exists and is byte-identical to the capsule source: skip it;
  it is already adopted.
- Destination exists and differs: stop. This is a collision. Do not
  overwrite it as part of this pass. Record the path and resolve it under
  Review before replacing.

## Review before replacing

Never silently replace a destination file that differs from the capsule
source. Show the difference and require explicit confirmation naming that
specific file before replacing a locally modified framework artifact. When
uncertain whether a difference is a local modification or drift from an
older capsule version, treat it as a local modification until a human
confirms otherwise.

## Copy boundaries

Copy verbatim only the generic capsule material that carries no
repository-specific fact: `CORE.md`, `REFERENCES.md`, `ADOPT.md`,
`RECOMMENDATIONS.md`, `skills/`, `templates/`, `rules/`, and `scripts/`. Do
not copy `SPEC.md`, `spec/`, `tasks/`, `knowledge/`, `open-claims/`,
`SELF-IMPROVEMENT/`, repository `evidence/`, or `.archive/`; each contains
source-repository instance data.

The source repository's `.archive/task-contract-v1.txt` is instance-specific
checker compatibility data. Do not copy it. A destination with its own frozen
version-1 Tasks creates an explicit baseline naming only those Task directories.

This file copies itself. A destination that does not hold `ADOPT.md` cannot
perform the convergent re-run described under Repeat adoption, and its
`INDEX.md` would name a file that is not there.

`INDEX.md` is capsule infrastructure by filename, but its content is always
this repository's own instance data, never the source capsule's. Do not copy
the source repository's populated `INDEX.md`. Create the destination's
`INDEX.md` fresh from `templates/INDEX.md`, listing the new specification and
only what this adoption actually copied (ordinarily the three responsibility
Skills) with every other category, including Rules, left in its explicit
empty state.

`SPEC.md` is also instance data. Create it fresh from `templates/SPEC.md` and
fill in the destination system's completed, accepted behavior. Do not copy this
repository's `SPEC.md` or its `spec/` modules. Leave `spec/` absent until the
destination specification is large enough to need a child module from
`templates/SPEC-MODULE.md`.

The same applies to `ledger/`: its threads hold this repository's own pre-Task exploration, never the source's.
Do not copy any thread; the destination creates its own from `templates/LEDGER-THREAD.md` as its exploration needs them.
A destination adopted before threads existed may still have a root `LEDGER.md`, which `scripts/check-capsule` refuses.
Move its entries unchanged into a thread under `ledger/`, or delete the file if it holds only the empty template, and list any new thread in `INDEX.md`.

Before copying `RECOMMENDATIONS.md`, check it for a repository-specific
narrative claim (a sentence naming "this repository" and a fact only true of
the source repository, such as an observed local-verification result).
Generic guidance copies verbatim; a repository-specific claim does not, and
should be flagged for correction in the source capsule rather than copied
into the destination.

Do not create `spec/`, `knowledge/`, `evidence/`, `open-claims/`, or
`SELF-IMPROVEMENT/` empty; they come into existence only when something is
actually promoted, archived, or added into them. `commands/` follows the
same rule. `rules/`
is copied non-empty, since this capsule now ships one generic Rule
(`rules/self-improvement-log.md`); a single-Skill Rule still lives inline
in its owning Skill instead (see `templates/SKILL.md`). A repository-specific
Rule the destination repository adds later, for its own use, is never part
of a capsule copy.

## Create the entry point

If no entry-point file exists yet for the target harness, add one (for
example `CLAUDE.md` for Claude Code, `AGENTS.md` for Codex) from
`templates/ENTRY-POINT.md`. Fill in the relative path from that
entry-point file's own location to the adopted `CORE.md`. Do not wrap the
instruction in text that duplicates Core's guidance or names an individual
Skill.

If an entry-point file already exists, its content is very likely
repository-specific and must not be overwritten. Show its current content
for explicit review, then append the linked-imperative instruction from
`templates/ENTRY-POINT.md` to it rather than replacing the file. Confirm the
existing content and the appended instruction can both be present without
contradiction; if the existing content already routes elsewhere in a way
that would prevent this instruction from being followed, stop and resolve
that conflict explicitly before treating adoption as complete.

## Check relative references

After copying, confirm every capsule-internal link resolves at the
destination path. This matters most when the framework root differs from
the default: a link written relative to `addf/` still resolves
correctly as long as the whole capsule moved together, but verify it rather
than assume it. A broken link is a fatal adoption defect; fix it before
treating adoption as complete.

Resolve every `framework:` reference from the selected root. Resolve a
`history:` reference only when its destination archive exists. The destination
specification must not depend on `history:` for current requirements.

If Python 3.11 or newer is available, run the copied optional checker after
filling the destination's instance files:

```console
python <framework-root>/scripts/check-capsule --root <framework-root>
```

The command must succeed from a directory outside the destination repository
as well as from its root. Do not treat the executable check as a substitute for
the reviewed collision, entry-point, or copy-boundary steps above.

## Check archive exclusion

If the destination repository will use a text-search tool covered by
`RECOMMENDATIONS.md`, follow that recommendation's own local-verification
step once the repository has an `.archive/` directory, rather than assuming
exclusion holds without checking it in that repository.

## Rollback

Before the first commit, adoption is trivially reversible: discard the
working-tree changes. After a commit, revert the adoption commit. Adoption
never changes a file outside the chosen framework root and the one
entry-point file it creates or edits; confirm this with a full repository
diff before committing, so rollback stays confined to those paths.

## Repeat adoption

Repeating this procedure is convergent when every capsule file already
matches byte-for-byte: it makes no changes. It stops at a collision when any
capsule file was modified locally since the last adoption. Do not repeat
past a collision without the explicit review above.

## Adoption invariants

- The framework root is the only configurable value.
- Reusable framework material is copied. `SPEC.md`, `spec/`, `INDEX.md`, and
  `ledger/` are created as destination instance data.
- `scripts/` is reusable framework material; archive compatibility baselines
  are repository instance data.
- Never overwrite a differing destination file without explicit review
  naming that file.
- Adoption changes no file outside the framework root and the one
  entry-point file it creates.
- A human can complete this procedure without this repository's design
  history, Task bundles, or maintainer documentation.
