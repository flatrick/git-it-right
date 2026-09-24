# References

This file defines durable references beneath a selected framework root.

## Stable anchors

Place an explicit HTML anchor immediately before an item when another artifact
needs to reference that item.

```html
<a id="claim-name"></a>
```

Use lowercase ASCII words separated by hyphens. The anchor must be unique in
its file. A display heading may change without changing the anchor. Create a
new anchor when the item's meaning or material scope changes.

## Task-bundle references

Use an ordinary source-relative Markdown link when both artifacts belong to
one Task bundle.

```markdown
[Claim](../TASK.md#claim-name)
```

Archival moves the whole bundle without changing its internal layout, so the
link keeps the same meaning.

## Current framework references

Use `framework:` for a target resolved from the selected framework root.

```text
framework:SPEC.md
framework:skills/work-control.md
framework:knowledge/example.md#claim-name
```

The path after `framework:` is relative to the framework root. It cannot start
with `/`, contain `..`, or resolve beneath `.archive/`.

## Historical references

Use `history:` only for a direct provenance target beneath `.archive/`.

```text
history:tasks/example/verifications/outcome.md#conclusion
```

The path after `history:` is relative to `.archive/`. It cannot start with `/`
or contain `..`. During the terminal checkpoint that publishes a reference to
the current Task, a resolver may accept the matching path under `tasks/`; after
archival it must resolve beneath `.archive/`.

`history:` never supplies normative specification content or ordinary current
discovery. A current Knowledge Basis may use it to identify the Verification
that established a promoted Claim.

## Invariants

- Normalize paths before comparing identity.
- Do not use line numbers or editable heading text as durable item identity.
- A record references material that it consumes. Do not add reciprocal links
  only for navigation.
- A broken target or anchor makes the reference invalid.
