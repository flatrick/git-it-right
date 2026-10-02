# Recommendations

Optional conveniences. None of these is required: the framework's semantics
are fully expressed in tool-less Markdown, and ignoring every recommendation
here still leaves it usable. Nothing here is normative; `CORE.md` and the
three responsibility Skills are the only normative routing. Paths below are
relative to the framework root.

## Exclude `.archive/` from text-search tools

**Invariant supported:** archived material must not present as current truth
to a documented discovery path (`CORE.md`, `INDEX.md`, direct links) or to a
search integration the framework itself recommends.

**Integration steps:** once a repository has archived any material beneath
`.archive/`, add that path to the search tool's own ignore mechanism — for
`rg`/ripgrep, a repository-level `.rgignore` or `.ignore` file containing
`.archive/`; for another tool, its equivalent.

**Limits:** this excludes `.archive/` only from that one tool's default
search. A direct path read, a different tool, or a search invoked with
flags that override ignore files (for example `rg --no-ignore`) still
reaches archived content. Exclusion is a convenience against accidental
discovery, not an access control. The default framework root (`addf/`) has
no leading dot for exactly this reason: a dot-prefixed root is invisible to
a bare default invocation of most text-search tools, which would silently
hide active Tasks and Knowledge, not just `.archive/`. If a repository
chooses a dot-prefixed alternate root anyway, remember that most search
tools already exclude it from a bare default invocation regardless of this
configuration; this recommendation matters once a search explicitly reveals
hidden paths (for `rg`, the `--hidden` flag) but should still exclude
`.archive/` beneath it.

**Bypass path:** `rg --no-ignore` or an equivalent flag intentionally
searches archived material, which is sometimes the correct action (for
example, reconstructing a terminal Task's history).

**Local verification:** confirm the exclusion actually holds in your
repository and environment before relying on it: place a distinctive string
in a file beneath `.archive/`, run the search tool's default invocation for
that string, and confirm it does not appear. Re-verify after any change to
the repository's ignore configuration, and before relying on the
recommendation in a repository where you have not personally checked it.

**Tool-less alternative:** the framework's own discovery paths (`CORE.md`,
`INDEX.md`, and direct links from current artifacts) never route through
`.archive/` regardless of search-tool configuration. A human or agent that
only follows those paths sees no archived material without deliberately
navigating into `.archive/`.

## Run the structural checker in CI

**Invariant supported:** current specification, Task, reference, Claim, link,
placeholder, and Index structure remains mechanically coherent across changes.

**Integration steps:** when the optional checker exists beneath the selected
framework root, run its public CLI and test suite with Python 3.11 or newer. A
GitHub Actions repository can use a Linux and Windows matrix with these steps:

```console
python -m unittest discover -s <framework-root>/scripts/tests -v
python <framework-root>/scripts/check-capsule --root <framework-root>
```

Add `--include-archive` in a separate step when the repository maintains
versioned historical Tasks. Keep any version-1 compatibility baseline local to
that repository; adoption does not copy another repository's baseline.

**Limits:** a green structural check establishes only the invariants the tool
implements. It does not prove behavioral conformance to the specification and
does not replace the lifecycle's Verifications or the manual adoption review.
The checker is optional and cannot become a prerequisite for reading or using
the framework.

**Local verification:** run the same commands outside CI from the repository
root and from an unrelated current directory. When alternate roots are
supported, include one path containing a space. Compare repeated failure output
byte-for-byte before relying on diagnostics in automation.

**Tool-less alternative:** inspect the current routes and artifact contracts
manually. The Markdown contract remains complete when Python or CI is absent.

## Split large Claim sets into a `claims/` subfolder

**Invariant supported:** a Task document stays legible — a reader can find
and read a specific Claim without scrolling past a long run of near-identical
Claim records, and this repository's own file-organization guidance (small,
focused files) still applies to Task bundles, not only to source code.

**When to split:** once a Task's Material empirical premises would hold
roughly 5 or more Claims, or once inlining them would carry the Task
document past this repository's own file-hygiene bound (800 lines), move
Claims into `tasks/<task-name>/claims/`, one file per Claim (or a small
coherent group that always changes together), using
`templates/CLAIM.md`'s shape. A Task with only a few Claims should keep them
inline in Material empirical premises as usual — this is a scale
accommodation, not a new default.

**Naming and grouping:** prefix each filename by whatever grouping is
material for that Task — commonly, the section or theme of the source
material the Claim traces back to (for example `findings-*.md` vs.
`feasibility-*.md` when Claims are extracted from a research document with
named sections) — so the grouping is visible directly in a directory
listing, without opening files. Pick a stable dimension for the prefix
(source section, subsystem, risk category); do not reprefix files just
because Claim priority order changes, since priority is better expressed as
prose in the Task's own index of the folder (see below) than encoded into
every filename.

**Integration steps:** replace the Task's inline Claim records with a short
paragraph in Material empirical premises naming the convention in use,
followed by a per-file index (one line per Claim file, prefixed exactly as
the filenames are, linking to it) so a reader can still see the full set at
a glance from `TASK.md` alone. Each Claim file's own "Owning Task" section
links back to the Task for the objective/Constraints context a standalone
Claim file cannot restate on its own.

**A Claim that outlives the Task, not just the inline section, is a
different case:** `claims/` only scales an active Task's own inline Claim
set; a Claim still decision-relevant after the Task terminalizes moves to
`open-claims/` (`templates/OPEN-CLAIM.md`) instead, through Stewardship's
"Carry forward open Claims" step, never staying behind in `claims/` inside
an archived bundle.

**Local verification:** after splitting, run `check-capsule` and confirm it
reports the same or more Claims discovered as before the split (compare its
Claim-related failure/pass output, or add a deliberately broken Claim file
and confirm the checker catches it).

**Tool-less alternative:** a human reader can always navigate `claims/`
directly regardless of `check-capsule` support; only automated structural
validation of the split-out Claims depends on `check-capsule`'s `Claim:`
discovery.

## Recommendations invariants

- A recommendation never becomes required for correctness or discovery.
- Following a recommendation cannot silently hide current artifacts.
- Each recommendation states how a repository can verify its own
  configuration, rather than asserting the result for every adopter.
- Archive-exclusion claims are stated only for tools this repository actually
  checked; unchecked tools are not claimed to behave the same way.
