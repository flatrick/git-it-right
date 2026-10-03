# Testing gir with a git client

How to check whether gir works from a git client (Visual Studio, Visual Studio Code, SourceGit, SourceTree, GitKraken, Fork, and others), and how to report what you find.

gir is tested from terminals.
A git client differs from a terminal in ways that can break gir:

- It may run its own bundled git instead of the one on your `PATH`.
- It may not see your shell's `PATH`, so the hooks cannot find `gir`.
  This is common for apps started from the macOS Dock or Finder.
- It usually runs commands without a terminal.
  gir only asks questions in a terminal, and git can only open a terminal editor (vim, nano) in a terminal.
- It shows hook output its own way: in full, cut short, or as a generic "hook failed".

Each step below checks one of these.
You do not need to know Rust or gir's internals.
Record a result for every step, including the ones that could not be run, so the report shows where things break.

## Before you start

You need:

- gir on your `PATH` (`gir --version` works in a terminal).
- git 2.28 or newer.
- 30–45 minutes.

Write down, for the report:

- The client's name and version, and your OS and version.
- **Which git the client uses.** Many clients show this in their settings (often as "Git executable" or "embedded/system git").
  If it has a built-in terminal, run `git --version` there.
- **How you started the client**: from the Start menu, Dock or launcher, or from a terminal (`code .`).
  This changes which `PATH` it sees.
- `gir --version`.

## Create the test repository

Run one of these in a terminal, from a folder where a new `gir-client-test` folder may be created.
They use only the throwaway repository they create; nothing else is changed.

```sh
sh path/to/git-it-right/scripts/client-test-repo.sh
```

```powershell
pwsh path/to/git-it-right/scripts/client-test-repo.ps1
```

The script creates:

- `gir-client-test/work`: the repository to open in your client.
  gir's hooks are active in it, `main` holds `chore: base`, and branch `topic` (checked out) holds `feat: add a` (`a.txt`) and `feat: add b` (`b.txt`).
  Tag `client-test-start` marks this state.
- `gir-client-test/remote.git`: a local stand-in for a server, already set up as `origin`.

Open `gir-client-test/work` in your client.

To reset between steps, run this in `gir-client-test/work`.
It discards all changes there:

```sh
git reset --hard client-test-start
```

**Stage changes for two commits** is used several times below.
In the client:

1. In `a.txt`, change the line `two` to `TWO`.
   That line belongs to `feat: add a`.
2. In `b.txt`, change `b` to `B`.
   That line belongs to `feat: add b`.
3. Stage both files.

## Results

Record each step as one of:

- `PASS`: what happened matches **Expect**.
- `FAIL`: it did not.
  Note what you saw instead: the exact text, a screenshot, or "nothing happened".
- `PARTIAL`: the result was right but something was wrong along the way, such as a cut-short message or a window that stayed open.
- `N/A`: the client cannot do this step.
  Note why.

## Steps

### C1 Hooks find gir

**Do:** in the client, create a file `c1.txt`, stage it, and commit it with the message `feat: add c1`.

**Expect:** the commit is created with that message.
Any output the client shows contains no `gir: not installed`.

**If it fails:** `gir: not installed, commit-msg check skipped` means the client cannot find `gir` on its `PATH`.
Record how you started the client, then try again after starting it from a terminal.
Every later step depends on this one.

### C2 A rejected message is shown

**Do:** stage a change and commit it with the message `added stuff`.

**Expect:** the commit is refused, and the client shows gir's message:

```text
gir: commit rejected [type-missing] header must start with `<type>[(scope)][!]: `
  try: <type>: added stuff   types: feat fix docs style refactor perf test build ci chore revert
  more: gir explain type-missing
```

**Record:** whether the client showed all three lines, part of them, or only a generic error.
Also record whether your message was kept so you can edit it.

### C3 An automatic fix is shown and kept

**Do:** commit a staged change with the message `Feat: Add c3.`

**Expect:** the commit is created with the subject `feat: Add c3`.
gir prints `gir: fixed [type-case] Feat -> feat` and `gir: fixed [desc-period] Add c3. -> Add c3`.

**Record:** the subject the client's history shows after a refresh, and whether it showed the `gir: fixed` lines at all.

### C4 The client's own fixup, amend and squash commits

Skip this step (`N/A`) if the client has no fixup, squash or amend feature.

**Do:** stage a change to `a.txt` and use the client's feature to create a fixup, squash or amend commit for `feat: add a`.

**Expect:** the commit is created, with a subject starting `fixup! `, `squash! ` or `amend! `.
The commit-msg hook does not refuse it.

### C5 Push is refused while a fixup commit is unsquashed

**Do:** reset, then stage a change to `a.txt` and create a fixup commit for `feat: add a`, either with C4's feature or with `gir fixup` in a terminal.
Push `topic` from the client.

**Expect:** the push is refused, and the client shows:

```text
gir: <sha> push rejected [fixup-unsquashed] `fixup!` commit must be squashed before pushing
  try: git rebase --autosquash <base>
  more: gir explain fixup-unsquashed
```

### C6 Autosquash in the client

Skip this step (`N/A`) if the client has no interactive rebase.

**Do:** continuing from C5, use the client's interactive rebase onto `main` with autosquash, or with the fixup commit moved under `feat: add a` and marked fixup.
Then push `topic` again.

**Expect:** `topic` shows `feat: add a` and `feat: add b`, with no `fixup!` commit, and `a.txt` keeps your change.
The push succeeds.

### C7 `gir fixup` without a terminal does not wait for an answer

This is the most important step for gir's picker.
Run gir the way the client runs external commands, not in a terminal window.
Most clients have one of the following; use whichever yours has, and record which:

- **Custom actions.** SourceTree and SourceGit can run a command on the current repository.
  Set the command to `gir`, the arguments to `fixup`, and turn on showing the full output if offered.
- **External tools.** Visual Studio's Tools > External Tools.
  Set the command to `gir`, the arguments to `fixup`, the initial directory to the repository, and use the Output window.
- **Visual Studio Code.** A task in `.vscode/tasks.json` runs in the integrated terminal, which counts as a terminal; test it in C10 instead.
  For this step, mark `N/A` unless an extension runs commands without the terminal.

Menu names differ between versions; note in the report where you found the feature.

**Do:** reset, **stage changes for two commits**, then run the action.

**Expect:** the action finishes at once with exit code `2`, and shows:

```text
gir: staged changes belong to several commits:
  <sha> feat: add a  <- a.txt:2
  <sha> feat: add b  <- b.txt:1
  split: git restore --staged . && git add -p, then one gir fixup per commit
  or: gir fixup --split creates one fixup! per commit
```

**It fails if** the action shows `pick [1-2, s, q]:`, or seems to hang.
In that case, stop the action, then set the environment variable `GIR_INTERACTIVE=0` for the client (or the action) and try again.
Record both results.

### C8 `gir fixup --split` without a terminal

**Do:** reset, **stage changes for two commits**, then run the action from C7 with the arguments `fixup --split`.

**Expect:** two new commits, `fixup! feat: add a` changing only `a.txt` and `fixup! feat: add b` changing only `b.txt`, and nothing left staged.
Output ends with a `fold: git rebase --autosquash <sha>` line.

### C9 Commands that open an editor, without a terminal

`gir amend`, `gir reword` and `gir squash` let git open its editor for the new message.
Without a terminal, only a graphical editor can open.

**Do:** first run `git var GIT_EDITOR` in a terminal and record its output.
Reset, then run the action from C7 with the arguments `reword client-test-start~1`.

**Expect**, depending on the editor:

- **Graphical editor** (for example `code --wait`, `notepad`, or a GUI editor set in the client): it opens with the message of `feat: add a`.
  Save and close it; the action creates `amend! feat: add a` and prints `gir: created amend! for <sha> feat: add a`.
  Record `PASS`.
- **Terminal editor** (vim, vi, nano, emacs -nw), or nothing set: the action may fail with `error: there was a problem with the editor` followed by `gir: git commit --quiet --fixup=reword:<sha> failed`.
  Record `FAIL` with the text.
  It may also hang: a terminal editor waits for a terminal that does not exist.
  Record `FAIL` and "hangs".

**If it hangs:** stop the action.
If the client does not stop it, end the `vim`/`vi`/`nano` or `git` process.
A stopped commit can leave `.git/index.lock` behind; git then refuses everything with `Unable to create '.../.git/index.lock': File exists`.
Delete that file once no git process is running for the repository.

### C10 The picker in the client's built-in terminal

Skip this step (`N/A`) if the client has no built-in terminal.

**Do:** reset, **stage changes for two commits**, then run `gir fixup` in the client's terminal: VS Code's integrated terminal, Visual Studio's Developer PowerShell, or the client's "open terminal" option.
At `pick [1-2, s, q]:`, press Enter.

**Expect:** the numbered list with `s) split: one fixup! per commit`, then `gir: cancelled; nothing committed`, and no new commit.
Run it again and answer `s`: two `fixup!` commits, as in C8.

**Record:** whether the prompt appeared, and whether typing an answer worked.

## Report

Open an issue at <https://github.com/flatrick/git-it-right/issues> titled `Client test: <client> <version> on <OS>`, with this filled in:

```text
Client:            <name and version>
OS:                <name and version>
Git used:          <path or "bundled"/"system", and git --version>
Started from:      <Start menu / Dock / launcher / terminal>
gir:               <gir --version>
GIT_EDITOR:        <output of git var GIT_EDITOR>

C1  hooks find gir:              PASS | FAIL | PARTIAL | N/A  <notes>
C2  rejection shown:             ...
C3  automatic fix kept:          ...
C4  client's own fixup commits:  ...
C5  push refused:                ...
C6  autosquash in client:        ...
C7  gir fixup, no terminal:      ...  <which feature you used>
C8  --split, no terminal:        ...
C9  editor, no terminal:         ...
C10 picker in built-in terminal: ...

Anything else that surprised you:
```

Paste the exact text of any failure; a screenshot helps when the client shows output in a dialog.

## Reported results

Add a row here, in a pull request, once a report's issue exists.

| Client | Version | OS | Date | Failed steps | Report |
|---|---|---|---|---|---|
| NONE | | | | | No client has been reported yet. |
