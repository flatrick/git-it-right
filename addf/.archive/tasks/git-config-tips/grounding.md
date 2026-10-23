# CLI architecture grounding

## Overview

`gir` has one CLI dispatcher in `src/main.rs`; command code is exposed from `src/lib.rs` and organized in `src/cmd/`. `doctor` is a repository inspection command with an optional write phase. `explain` is an embedded Markdown topic reader. A static, read-only `tips` command would enter through the dispatcher and print a curated catalog without needing doctor's repository entry or `--fix` path.

## Key concepts

- `run()` in `src/main.rs` parses arguments with `lexopt`, validates flags per subcommand, then matches `(sub, positional.as_slice())` to call a command implementation. `main()` maps returned `Ok(i32)` to a process exit and `Err(String)` to `gir: ...` on stderr with exit 2.
- `doctor::Check` carries `Level`, a message, and an optional `Fix`. `LOCAL_CONFIG` in `src/cmd/doctor.rs` is a static `(key, wanted value, why)` list. `Fix::LocalConfig` is its Git config mutation path.
- `explain::SOURCES` in `src/explain.rs` embeds `CHEATSHEET.md` and `src/explain.md` with `include_str!`; `## ` headings name topics. `types` is generated from `Config` rather than Markdown.

## How it works

`run()` handles global help and version before parsing remaining values and long flags. Its `allowed` match constrains each command's flags. The final match checks positional arity. `doctor::doctor(fix)` calls `git::enter_toplevel()` and runs `config_checks`, `file_checks`, and `index_checks`. `config_checks` reads effective values through `git::get_config`. For `LOCAL_CONFIG`, a matching value is OK, a nonempty different value is informational and has no fix, and an unset value is a warning with `Fix::LocalConfig`. `pull.ff` is considered OK when `pull.rebase` is set. If `--fix` is given, `apply()` executes each fix, and `Fix::LocalConfig` calls `git config --local key value`. Identity and default-branch checks suggest global config without changing it.

For `explain`, the dispatcher prints `topics()` on bare `gir explain`, or loads `Config` with a default fallback and calls `page(topic, &cfg)` for one topic. `section()` finds the exact `## {topic}` heading and slices through the next heading. Unknown topics become exit-2 errors listing valid topics.

## Where things live

- `src/main.rs`: usage, parser, dispatch, exit behavior.
- `src/cmd/mod.rs` and `src/lib.rs`: command module exposure.
- `src/cmd/doctor.rs` and `src/git.rs`: recommendations, checks, fixes, and Git config process calls.
- `src/explain.rs`, `src/explain.md`, `CHEATSHEET.md`: topic extraction and embedded content.
- `tests/args.rs`, `tests/doctor.rs`, `tests/explain.rs`: corresponding CLI contracts.

## Gotchas

`git::get_config` reads effective config from the repository context, while doctor fixes use `--local`; a global matching value counts as OK and gets no local copy. `doctor` requires a repository and enters its root; a static `tips` command need not. Explain topics are exact, case-sensitive Markdown headings, and `types` is special. The argument tests cover help, flags, and arity for commands.
