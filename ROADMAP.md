# Roadmap

Planned features that are not being worked on yet.

- Each item says what it is, why, and what it depends on.
- When an item becomes an addf Task (`addf/tasks/<name>/`), remove it from this file in the same change. From then on the Task is where its scope and decisions live.
- An item here is a plan, not a promise or a specification. Current behavior is specified in `addf/SPEC.md`.

## Refuse a terminal editor when there is no terminal

**What:** `gir amend`, `gir reword` and `gir squash` refuse to start git's editor when three conditions all hold:
- there is no controlling terminal (`/dev/tty` on Linux and macOS, `CONIN$` on Windows cannot be opened);
- git's editor (`git var GIT_EDITOR`) is a known terminal editor (vi, vim, nvim, nano, emacs -nw and similar);
- `GIR_INTERACTIVE` is not `1`.

The refusal names the editor and says how to fix it: a graphical `core.editor`, or running the command in a terminal.

**Why:** from a GUI client's custom action, git starts the terminal editor, which waits forever for a terminal that does not exist. Stopping it leaves `.git/index.lock` behind, and git then refuses every command in that repository until the file is deleted. See step C9 in `CLIENT-TESTING.md`.

**Depends on:** `gir amend`, `reword` and `squash` being merged to `main`.

## Report the environment gir runs in

**What:** a diagnostic listing (in `gir doctor` or a new `gir env`) of:
- whether stdin, stdout and stderr are terminals, and whether there is a controlling terminal;
- the parent process chain;
- `TERM_PROGRAM`, `GIT_ASKPASS` and the git editor;
- which git binary runs, and its version.

It only reports; gir makes no decision based on the process names.

**Why:** a tester following `CLIENT-TESTING.md` can paste this output instead of working these details out by hand, and the reports show over time how each client runs gir.

**Depends on:** `CLIENT-TESTING.md` being merged to `main`.

## Lint the new message of an `amend!` commit

**What:** the `commit-msg` hook lints the replacement message inside an `amend!` commit (created by `gir amend`, `gir reword` or `git commit --fixup=amend:`/`reword:`) when the commit is made.

**Why:** today an `amend!` subject returns before any rule runs. A replacement message that is not a Conventional Commit is only caught after `git rebase --autosquash` has folded it in. By then it is an ordinary commit, which `pre-push` or CI rejects, and fixing it takes another reword.

**Depends on:** nothing.
