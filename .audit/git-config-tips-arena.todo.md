# Arena workflow — tips command architecture

- [x] Frame — define the artifact, rubric, candidates, and isolated output paths.
- [x] Fan out — collect three independent design sketches.
- [x] Cross-judge — score all sketches against the rubric.
- [x] Pick — compare every sketch with the judge and select a base.
- [x] Graft — incorporate only coherent, high-value ideas from the other sketches.
- [x] Verify — check that the synthesized design meets the requirements and red-flag screen.

Selected candidate 1 (25/25); no structural graft was needed. CLI rejection cases and explicit user-selected examples are carried into implementation acceptance. See `addf/tasks/git-config-tips/decision.md`.
