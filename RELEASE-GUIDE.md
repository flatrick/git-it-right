# Release guide

How to cut a gir release: pick the version, write the changelog with git-cliff, tag the release commit on `main`, and publish a GitHub release.

Commands are PowerShell.
Replace `vX.Y.Z` with the new version throughout.

## Before you start

- You need `git-cliff`, `gh` (logged in), and a Rust toolchain.
- Merge pull requests into `main` with a merge commit, not a squash.
  git-cliff reads individual commits from `main`; a squash leaves it one line per pull request.
  v0.0.1 was squashed, so only the `CHANGELOG.md` committed at that tag lists its changes.
  The repository settings allow only merge commits, so GitHub offers no other method.
- CI on `main` must be green.

## 1. Pick the version

```powershell
git fetch origin --tags
git-cliff --bumped-version
```

git-cliff suggests the next version from the commit types since the last tag.
Treat it as a suggestion; you decide the number.

## 2. Prepare the release branch

Branch from `origin/main`, not from a feature branch.
The changelog must cover every commit the tag will contain, including ones committed straight to `main`.

```powershell
git worktree add .worktrees/release-vX.Y.Z -b release/vX.Y.Z origin/main
cd .worktrees/release-vX.Y.Z
```

Set `version = "X.Y.Z"` in `Cargo.toml`, then update `Cargo.lock` and check the build:

```powershell
cargo build
cargo test
```

## 3. Write the changelog

```powershell
git-cliff --unreleased --tag vX.Y.Z --prepend CHANGELOG.md
git-cliff --unreleased --tag vX.Y.Z --strip all -o ../../.scratch/release-notes-vX.Y.Z.md
```

The first command adds the new version's section to the top of `CHANGELOG.md`.
The second writes the same section to a scratch file for the GitHub release.
Run both before committing, so the release commit itself stays out of the notes.

Read the new section.
Fix a wrong entry by rewording its commit before release, not by editing the changelog by hand.

## 4. Commit and merge

```powershell
git add Cargo.toml Cargo.lock CHANGELOG.md
git commit -m "chore(release): vX.Y.Z"
git push -u origin release/vX.Y.Z
gh pr create --fill
```

Merge the pull request with a merge commit once CI passes.

## 5. Tag

Run the rest of the steps from the main checkout.
Tag the merge commit on `main` with an annotated tag:

```powershell
git fetch origin
git tag -a vX.Y.Z -m vX.Y.Z origin/main
git push origin vX.Y.Z
```

Check that `origin/main` is the release merge before you tag it.

## 6. Draft the GitHub release

```powershell
gh release create vX.Y.Z --draft --verify-tag --title vX.Y.Z --notes-file .scratch/release-notes-vX.Y.Z.md --latest
```

`--verify-tag` stops `gh` from creating the tag if the push in step 5 failed.
A draft's URL shows `untagged-…` until it is published.
Review the draft on the repository's Releases page.

## 7. Publish

```powershell
gh release edit vX.Y.Z --draft=false
```

Publishing makes the release public and notifies watchers.
When you publish several drafts, publish the oldest first so the newest ends up as Latest.

## Clean up

```powershell
git worktree remove .worktrees/release-vX.Y.Z
git branch -d release/vX.Y.Z
```
