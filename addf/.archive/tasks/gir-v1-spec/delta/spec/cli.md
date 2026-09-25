# gir cli

## Subspecifications

`NONE`.

## Invocation

<a id="req-cli-version"></a>
**version.** `gir --version` and `gir -V` SHALL print `gir` followed by the crate version on stdout and exit `0`, ignoring any arguments after them.

<a id="req-cli-help"></a>
**help.** `gir` with no arguments, `gir -h`, `gir --help`, and `-h` or `--help` after any subcommand SHALL print the usage text on stdout and exit `0`, ignoring any other arguments except an unknown short flag before it, which fails as below.

<a id="req-cli-usage-lists-commands"></a>
**usage-lists-commands.** The usage text SHALL list `init`, `lint`, `fixup`, `doctor`, `explain` and `hook` with their accepted arguments and flags.

## Argument errors

<a id="req-cli-unknown-long-option"></a>
**unknown-long-option.** A long flag the subcommand does not accept SHALL print `gir: unknown option --FLAG for` followed by the subcommand on stderr and exit `2`.
The accepted flags are `init --force`, `lint --fix --json --range`, `fixup --dry-run` and `doctor --fix`.

<a id="req-cli-range-lint-only"></a>
**range-lint-only.** `--range` SHALL be accepted only by `gir lint`; any other subcommand given `--range` SHALL fail as an unknown option with exit `2`.

<a id="req-cli-unknown-short-option"></a>
**unknown-short-option.** Before a subcommand, a short flag other than `-h` and `-V` SHALL print `gir: invalid option '-C'` (with the given letter) on stderr and exit `2`; after a subcommand, a short flag other than `-h` SHALL print `gir: unknown option -C` on stderr and exit `2`.

<a id="req-cli-bad-arguments"></a>
**bad-arguments.** An unknown subcommand, `gir hook` without a hook name, or a known subcommand other than `lint` and `hook` with the wrong number of positional arguments, SHALL print `gir: bad arguments for` followed by the subcommand and the usage text on stderr, and exit `2`.

<a id="req-cli-hook-arguments"></a>
**hook-arguments.** `gir hook commit-msg` SHALL take exactly one file and `gir hook pre-push` SHALL take the remote and ignore any further arguments; any other hook name or argument count SHALL print ``gir: unknown hook `NAME` `` on stderr and exit `2`.

<a id="req-cli-lint-arguments"></a>
**lint-arguments.** `gir lint` with more than one positional argument, or with a positional argument and `--range`, SHALL print ``gir: gir lint takes one file, `-`, or --range`` on stderr and exit `2`.

## Exit codes

<a id="req-cli-runtime-error"></a>
**runtime-error.** Any error that stops a command (an unreadable file, an invalid `.girconfig`, a failing git call) SHALL print one message prefixed `gir: ` on stderr and exit `2`.
Output from a git command that gir runs with inherited stdio (the `git commit` of `gir fixup`) SHALL appear before that message.

<a id="req-cli-repository-root"></a>
**repository-root.** `gir init`, `gir doctor` and `gir fixup` SHALL behave the same when run from any directory inside the repository as from its root.

## Module invariants

- This file is reachable from `SPEC.md` through one ordered parent link.
- This file contains current requirements only.
- This file does not require archived material to define current behavior.
