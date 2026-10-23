# Architect workflow — git config tips

- [x] Ground — explain the current command and config-check architecture.
- [x] Sketch — compare at least two structurally distinct command/catalog designs and screen each for design red flags.
- [x] Agree — select the design; no human checkpoint was requested.
- [ ] Implement — fill in code against the selected sketch and reconcile deviations.
- [ ] Scrap — watch for repeated structural friction; redesign if the sketch is wrong.

## Feature throughput checkpoint

- [x] Blocking first steps — task state, investigation, and architecture decision complete before implementation.
- [ ] Independent workstreams — CLI/catalog/doctor source implementation is code-coupled; docs and tests will follow the selected contract.
- [ ] Shared mutable state — writers use separate worktrees under `.worktrees/`; lifecycle state and specification publication remain serialized.
- [ ] Smallest safe decomposition — one implementation owner keeps the cross-file CLI and doctor contracts coherent; architecture candidates and cross-judge were independent and read-only.
