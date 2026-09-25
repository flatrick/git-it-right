# gir lint

## Subspecifications

`NONE`.

## Message preparation

<a id="req-lint-strip-comments"></a>
**strip-comments.** Before linting a message file or stdin, gir SHALL set aside every line that starts with the comment string, and the scissors line with everything after it, the way `git commit` does.

<a id="req-lint-comment-string"></a>
**comment-string.** The comment string SHALL be `core.commentString`, else `core.commentChar`, else `#`; the value `auto` SHALL mean `#`.

<a id="req-lint-crlf"></a>
**crlf.** CRLF line endings in a message file or stdin SHALL be read as LF.

<a id="req-lint-normalize"></a>
**normalize.** gir SHALL strip trailing whitespace from every line, drop leading and trailing blank lines, and collapse runs of blank lines before checking, without reporting this as a fix.

## Messages exempt from Conventional Commits

<a id="req-lint-autosquash-exempt"></a>
**autosquash-exempt.** A message whose subject starts with `fixup! `, `squash! ` or `amend! ` SHALL pass `gir lint` for a file or stdin without further checks.

<a id="req-lint-merge"></a>
**merge.** A subject starting with `Merge ` SHALL pass without further checks, unless `gir.allowMerge` is false, in which case it SHALL be rejected with rule `merge-commit`.

<a id="req-lint-revert"></a>
**revert.** A subject starting with `Revert "` SHALL pass without further checks, unless `gir.allowRevert` is false, in which case it SHALL be rejected with rule `revert-commit`.

## Rejections

<a id="req-lint-empty"></a>
**empty.** A message that is empty after preparation SHALL be rejected with rule `empty`.

<a id="req-lint-type-missing"></a>
**type-missing.** A subject that is not shaped `type[(scope)][!]: description` SHALL be rejected with rule `type-missing` and a hint starting `<type>: ` followed by the subject, then the allowed types.

<a id="req-lint-type-unknown"></a>
**type-unknown.** A type that is not allowed after case and alias fixes SHALL be rejected with rule `type-unknown`, and gir SHALL NOT replace it.
When an allowed type is within edit distance 2 of it, and that distance is less than the allowed type's length, the message SHALL say `did you mean` the closest such type and the hint SHALL show the header with it.

<a id="req-lint-scope-required"></a>
**scope-required.** With `gir.scopeRequired` true, a header without a scope SHALL be rejected with rule `scope-required`.

<a id="req-lint-scope-unknown"></a>
**scope-unknown.** With `gir.scopes` set, each comma-separated scope not in that list SHALL be rejected with rule `scope-unknown`; listed scopes SHALL pass.

<a id="req-lint-desc-empty"></a>
**desc-empty.** An empty description SHALL be rejected with rule `desc-empty`.

<a id="req-lint-header-length"></a>
**header-length.** A header longer than `gir.subjectMax` characters after fixes SHALL be rejected with rule `header-length`.

<a id="req-lint-spec"></a>
**spec.** A message with no other rejection that still fails Conventional Commits 1.0.0 parsing SHALL be rejected with rule `spec`.

<a id="req-lint-valid-unchanged"></a>
**valid-unchanged.** A valid Conventional Commits message SHALL pass with no fixes and its text unchanged.

## Safe fixes

<a id="req-lint-header-spacing"></a>
**header-spacing.** Spaces around the type, `(scope)`, `!` and `:` SHALL be normalized to `type(scope)!: description`, reported as rule `header-spacing`.

<a id="req-lint-type-case"></a>
**type-case.** A type whose lowercase form is an allowed type or an alias SHALL be lowercased, reported as rule `type-case`.

<a id="req-lint-type-alias"></a>
**type-alias.** A type that is an alias SHALL be replaced by its target, reported as rule `type-alias`.

<a id="req-lint-scope-empty"></a>
**scope-empty.** An empty `()` scope SHALL be removed, reported as rule `scope-empty`.

<a id="req-lint-desc-period"></a>
**desc-period.** Trailing periods on the description SHALL be removed, reported as rule `desc-period`; a description ending in `..` SHALL be left alone.

<a id="req-lint-desc-case"></a>
**desc-case.** With `gir.descCase` `lower`, an uppercase first letter of the description SHALL be lowercased, reported as rule `desc-case`, unless the second letter is also uppercase; with `any` the description SHALL be left alone.

<a id="req-lint-body-separator"></a>
**body-separator.** A missing blank line between header and body SHALL be inserted, reported as rule `body-separator`.

<a id="req-lint-breaking-footer"></a>
**breaking-footer.** In the footer block, any case or underscore variant of `BREAKING CHANGE` before a colon SHALL be rewritten to `BREAKING CHANGE: value`, reported as rule `breaking-footer`; `BREAKING-CHANGE: value` SHALL be left alone.

<a id="req-lint-fix-idempotent"></a>
**fix-idempotent.** Linting an already fixed message SHALL produce no further fixes and the same text.

<a id="req-lint-fixes-do-not-fail"></a>
**fixes-do-not-fail.** For a message file, stdin and the commit-msg hook, fixes alone SHALL NOT make a message fail; only rejections SHALL.

## Output

<a id="req-lint-fix-line"></a>
**fix-line.** Each applied fix SHALL be reported on stderr as `gir: fixed [RULE] FROM -> TO`, with newlines shown as `\n`.

<a id="req-lint-rejection-lines"></a>
**rejection-lines.** Each rejection SHALL be reported on stderr in at most three lines: `gir: LABEL [RULE] MESSAGE`, then `  try: HINT` when a hint exists, then `  more: gir explain RULE`.

<a id="req-lint-json"></a>
**json.** With `--json`, gir SHALL print on stdout one JSON array with one object per linted message, holding `commit` (the full SHA, or `null` for a file or stdin), `ok`, `message` (the fixed text), `fixes` (objects with `rule`, `from`, `to`) and `violations` (objects with `rule`, `message`, `hint`, `explain`), and SHALL print no fix or rejection lines.

<a id="req-lint-json-escaping"></a>
**json-escaping.** JSON strings SHALL escape `"`, `\`, newline, carriage return, tab, and other control characters as `\u00XX`.

## The lint command

<a id="req-lint-file"></a>
**file.** `gir lint FILE` SHALL lint the file with label `rejected`, leave the file unchanged, and exit `1` when any rejection exists, else `0`.

<a id="req-lint-file-fix"></a>
**file-fix.** `gir lint FILE --fix` SHALL rewrite the file with the fixed message when a fix applies, keeping the set-aside comment lines after a blank line, and report each fix line.

<a id="req-lint-stdin"></a>
**stdin.** `gir lint` and `gir lint -` SHALL read the message from stdin; with `--fix` and without `--json` they SHALL print the fixed message on stdout.

<a id="req-lint-range"></a>
**range.** `gir lint --range A..B` SHALL lint every commit `git log A..B` selects, labelling each rejection with the first 10 characters of its SHA and `rejected`, and exit `1` when any commit is rejected.

<a id="req-lint-range-unsquashed"></a>
**range-unsquashed.** `gir lint --range` SHALL reject a commit whose subject starts with `fixup! `, `squash! ` or `amend! ` with rule `fixup-unsquashed` and a hint `git rebase --autosquash <base>`.

<a id="req-lint-range-fix-pending"></a>
**range-fix-pending.** `gir lint --range` SHALL reject a commit whose message a safe fix would change with rule `fix-pending`, a message naming the rule id of every pending fix, and a hint showing the fixed subject line.

## Module invariants

- This file is reachable from `SPEC.md` through one ordered parent link.
- This file contains current requirements only.
- This file does not require archived material to define current behavior.
