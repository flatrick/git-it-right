# gir explain

## Subspecifications

`NONE`.

## Topic list

<a id="req-explain-list-format"></a>
**list-format.** `gir explain` SHALL print one stdout line beginning `topics: `, with topic names separated by spaces, and exit `0`.

<a id="req-explain-unique-topics"></a>
**unique-topics.** `gir explain` SHALL list each topic at most once.

## Topic pages

<a id="req-explain-listed-pages"></a>
**listed-pages.** `gir explain <topic>` SHALL exit `0` and print nonempty stdout for every topic printed by `gir explain`.

<a id="req-explain-config-page-aliases"></a>
**config-page-aliases.** The `config` page's sample SHALL show the aliases `feature = feat` and `bugfix = fix`, matching the `.girconfig` that `gir init` writes.

<a id="req-explain-type-examples-valid"></a>
**type-examples-valid.** Every example listed on a default type's page SHALL pass `gir lint` with no fixes and start with that type, and each default type's page SHALL list at least two.

<a id="req-explain-known-pages"></a>
**known-pages.** `gir explain <topic>` SHALL provide a nonempty page for each default type, lint rule, and the topics `fixup-unsquashed`, `fix-pending`, `fixup`, `breaking`, `scopes`, `config`, `hooks`, `doctor`, and `types`.

<a id="req-explain-single-section"></a>
**single-section.** A topic page other than `types` SHALL print the text below its matching `## ` heading, omit that heading and subsequent sections, and end with one newline.

<a id="req-explain-no-next-heading"></a>
**no-next-heading.** A default type or rule page SHALL NOT include a subsequent `## ` heading.

<a id="req-explain-case-sensitive"></a>
**case-sensitive.** `gir explain` SHALL match topic names exactly, including case.

<a id="req-explain-unknown-topic"></a>
**unknown-topic.** For an unknown topic, `gir explain <topic>` SHALL exit `2` and print `gir: no topic` with the requested topic and `topics: ` on stderr.

## Types page

<a id="req-explain-types-config"></a>
**types-config.** `gir explain types` SHALL print `Allowed types (gir.types in .girconfig):` followed by the configured `gir.types` in their configured order, or the default types when no usable configuration is loaded.

<a id="req-explain-types-summaries"></a>
**types-summaries.** Each type on the `types` page SHALL have the first line of its cheatsheet section as its summary, or `changelog group: ` followed by its group title when no such section exists.

<a id="req-explain-types-aliases"></a>
**types-aliases.** `gir explain types` SHALL print `Auto-mapped aliases: ` followed by space-separated `alias->type` mappings for its aliases.

<a id="req-explain-types-footer"></a>
**types-footer.** `gir explain types` SHALL end with `When to use each, with examples: gir explain <type>. Also: gir explain breaking, scopes, fixup.`

## Arguments and effects

<a id="req-explain-read-only"></a>
**read-only.** `gir explain` SHALL NOT create or modify files, change git configuration, or run a git command that changes the repository.

## Module invariants

- This file is reachable from `SPEC.md` through one ordered parent link.
- This file contains current requirements only.
- This file does not require archived material to define current behavior.
