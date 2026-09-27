# Open Claim: `gui-clients-have-no-tty`

## Claim

<a id="gui-clients-have-no-tty"></a>
-   Claim: GUI git clients' custom actions (SourceTree, SourceGit, Visual Studio) run gir with stdin or stderr not a terminal, so `gir fixup`, `amend`, `reword` and `squash` never show their prompt there.
-   State: `UNVERIFIED`
-   Scope: those clients on Windows and macOS.
-   Consequence if false: `such a client could wait on gir's prompt; end of input cancels it and GIR_INTERACTIVE=0 disables it.`
-   Basis: none; the clients were not available when the Task ran on Linux.

## Originating Task

-   `history:tasks/fixup-modes-and-picker/TASK.md#gui-clients-have-no-tty`

## Next action

Run step C7 of [CLIENT-TESTING.md](../../CLIENT-TESTING.md) in each client: a custom action or external tool runs `gir fixup` in a repository whose staged changes belong to two commits.
The action ending at once with gir's `staged changes belong to several commits:` refusal supports the Claim for that client; showing or waiting on `pick [1-2, s, q]:` refutes it.
Record each client's report in a Verification, and split the Claim per client if the results differ.
