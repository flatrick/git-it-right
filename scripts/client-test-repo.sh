#!/bin/sh
# Creates the throwaway repository that CLIENT-TESTING.md tests against.
# Usage: sh scripts/client-test-repo.sh [directory]   (default: gir-client-test)
set -eu

dir=${1:-gir-client-test}
if [ -e "$dir" ]; then
	echo "$dir already exists; pass another directory" >&2
	exit 1
fi
if ! command -v gir >/dev/null 2>&1; then
	echo "gir is not on PATH; install it first" >&2
	exit 1
fi

mkdir -p "$dir"
root=$(cd "$dir" && pwd)
git init -q --bare "$root/remote.git"
git init -q "$root/work"
cd "$root/work"
git symbolic-ref HEAD refs/heads/main
if ! git config user.email >/dev/null; then
	git config user.name "gir client test"
	git config user.email "client-test@example.invalid"
fi

gir init
printf 'base\n' > base.txt
git add -A
git commit -q -m "chore: base"
git remote add origin "$root/remote.git"
git push -q -u origin main

git switch -q -c topic
printf 'one\ntwo\nthree\n' > a.txt
git add a.txt
git commit -q -m "feat: add a"
printf 'b\n' > b.txt
git add b.txt
git commit -q -m "feat: add b"
git tag client-test-start

echo "ready: open $root/work in your client"
