# Decision — one canonical SourceGit template

## Selected design

Keep `conventional-commits.json` at the repository root as the human-editable template and the single source of truth. Reorder its existing labels and descriptions to match `Config::DEFAULT_TYPES`, keep the SourceGit field names, and remove every `PrefillShortDesc` field so there is no placeholder content.

Embed the file in `src/cmd/templates.rs` with `include_str!("../../conventional-commits.json")`. Add the root JSON to `Cargo.toml`'s explicit package include list so installed builds retain the embedded template.

In `gir init`, write `conventional-commits.json` as a required file before `Config::load_at`. Route it through the existing writer, so matching files remain untouched, edits are kept with a conflict unless `--force` is set, and missing files are restored even in adopted repositories. Writing before config loading also lets a configured `gir.typesFile` point at this file during first-time setup.

Update the init spec, CLI usage, README setup and file table, and generated `GIT-IT-RIGHT.md` to identify the SourceGit helper file. Keep `cliff.toml` and `GIT-IT-RIGHT.md` optional.

## Rejected shapes

- Duplicating the JSON body in Rust would allow the checked-in file and generated file to drift.
- Deriving entries from effective project types would contradict the accepted built-in-only template and cannot produce the existing human labels and descriptions from `Config::DEFAULT_TYPES`.

## Verification plan

- Integration tests assert fresh init writes valid SourceGit fields in built-in order, with no placeholder prefill; the output matches the canonical JSON.
- Init tests cover required-file restoration in adopted repositories, edit preservation, `--force`, and CRLF-equivalent reruns.
- A configured `gir.typesFile = conventional-commits.json` test proves init writes the file before loading config.
- Run the relevant Rust test suites and verify `cargo package --list` includes the root JSON.
- Read the generated file and inspect the final diff directly.

## Gate

DECIDE gate: `ESTABLISHED` — the fixed template matches the accepted product intent, reuses the existing write semantics, and accounts for the crate's explicit package boundary.
