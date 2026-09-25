# gir cli

## Subspecifications

`NONE`.

## Invocation

<a id="req-cli-version"></a>
**version.** `gir --version` and `gir -V` SHALL print `gir` followed by the crate version on stdout and exit `0`.

<a id="req-cli-help"></a>
**help.** `gir` with no arguments, `gir -h`, `gir --help`, and `-h` or `--help` after any subcommand SHALL print the usage text on stdout and exit `0`.

<a id="req-cli-usage-lists-commands"></a>
**usage-lists-commands.** The usage text SHALL list `init`, `lint`, `fixup`, `doctor`, `explain` and `hook` with their accepted arguments and flags.

## Argument errors

<a id="req-cli-unknown-long-option"></a>
**unknown-long-option.** A long flag the subcommand does not accept SHALL print `gir: unknown option --FLAG for` followed by the subcommand on stderr and exit `2`.
The accepted flags are `init --force`, `lint --fix --json --range`, `fixup --dry-run` and `doctor --fix`.

<a id="req-cli-range-lint-only"></a>
**range-lint-only.** `--range` SHALL be accepted only by `gir lint`; any other subcommand given `--range` SHALL fail as an unknown option with exit `2`.

<a id="req-cli-unknown-short-option"></a>
**unknown-short-option.** A short flag other than `-h` and `-V` SHALL print `gir: unknown option -C` (with the given letter) on stderr and exit `2`.

<a id="req-cli-bad-arguments"></a>
**bad-arguments.** An unknown subcommand, or a known subcommand with the wrong number of positional arguments, SHALL print `gir: bad arguments for` followed by the subcommand and the usage text on stderr, and exit `2`.

## Exit codes

<a id="req-cli-runtime-error"></a>
**runtime-error.** Any error that stops a command (an unreadable file, an invalid `.girconfig`, a failing git call) SHALL print one message prefixed `gir: ` on stderr and exit `2`.

## Module invariants

- This file is reachable from `SPEC.md` through one ordered parent link.
- This file contains current requirements only.
- This file does not require archived material to define current behavior.
