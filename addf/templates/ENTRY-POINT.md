# Entry point: `<repository-chosen file, e.g. CLAUDE.md, AGENTS.md>`

Read and follow [Core](<relative path to CORE.md from this file>)
before repository work.

## Invariants

-   Resolve the relative path from this file's own location, not the
    process working directory.
-   Add no other framework guidance or Skill routing here; Core owns
    that.
-   A repository may wrap this sentence in syntax its harness requires
    (frontmatter, a heading, surrounding prose), but the wrapper must
    not duplicate framework behavior or route to an individual Skill
    directly.
-   One entry point loads one Core. Do not point multiple entry points
    at different Core copies within the same repository.
