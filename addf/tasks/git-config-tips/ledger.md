# Ledger thread — `git-config-tips`

Exploration of useful Git settings for gir doctor and a possible gir tips command, prompted by two articles and the request to teach developers about Git configuration.

Append-only: never edit or delete a prior entry; append new ones at the end.
Number each entry by its position among the questions, Q1 being the first; its answer takes the same number, as a separate list item.
Settle an entry with a numbered disposition line, D1 being the first, that names the entries it settles.
See `skills/work-control.md`'s Ledger section for the full contract, including how a Task takes entries and when the thread is archived.

- Q1: Which recommendations from Julia Evans's popular Git config options article might fit gir init or gir doctor --fix?
- A1: Evaluate fit against gir's current separation: init installs gir files and hooks; doctor reports and fixes repo-local hygiene and recommended config. Existing doctor settings already cover several options from the article; object checking was identified for possible consideration.
- Q2: What about the settings in GitButler's article on how Git core developers configure Git?
- A2: Review its settings against gir's existing checks and consider whether settings beyond doctor belong in a user-facing tips catalog.
- Q3: Should gir tips use profiles, one catalog, or both?
- A3: Do not decide presentation before surveying possible Git settings; first review the current options and then consider what to present.
- Q4: How should tips order and explain settings?
- A4: List safe settings first. Explicitly state any unexpected implications for Git behavior and when a developer might want the setting.
- Q5: What additional process constraint applies?
- A5: Follow `addf/CORE.md` and its lifecycle.
- Q6: Should the Task use an isolated workspace and branch?
- A6: Yes. Any worktree must be inside `.worktrees/`.
- Q7: What action should follow the proposed plan?
- A7: Implement it.
