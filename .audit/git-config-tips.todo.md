# Architect workflow — git config tips

- [x] Ground — explain the current command and config-check architecture.
- [x] Sketch — compare at least two structurally distinct command/catalog designs and screen each for design red flags.
- [x] Agree — select the design; no human checkpoint was requested.
- [ ] Implement — fill in code against the selected sketch and reconcile deviations.
- [ ] Scrap — watch for repeated structural friction; redesign if the sketch is wrong.

## Feature throughput checkpoint

- [x] Blocking first steps — task state, investigation, and architecture decision complete before implementation.
- [x] Independent workstreams — one implementation owner has CLI/catalog/doctor/test files in a separate worktree; this worktree owns README and generated onboarding text.
- [x] Shared mutable state — writers use separate worktrees under `.worktrees/`; lifecycle state and specification publication remain serialized.
- [x] Smallest safe decomposition — one implementation owner keeps the code-coupled CLI and doctor contracts coherent; prose changes use disjoint files.
