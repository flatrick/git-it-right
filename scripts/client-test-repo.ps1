# Creates the throwaway repository that CLIENT-TESTING.md tests against.
# Usage: pwsh scripts/client-test-repo.ps1 [-Directory <path>]   (default: gir-client-test)
param([string]$Directory = "gir-client-test")
$ErrorActionPreference = "Stop"

function Invoke-Git {
    git @args
    if ($LASTEXITCODE -ne 0) { throw "git $args failed" }
}

if (Test-Path $Directory) { throw "$Directory already exists; pass another directory" }
if (-not (Get-Command gir -ErrorAction SilentlyContinue)) { throw "gir is not on PATH; install it first" }

$root = (New-Item -ItemType Directory -Path $Directory).FullName
Invoke-Git init -q --bare (Join-Path $root "remote.git")
Invoke-Git init -q (Join-Path $root "work")
Set-Location (Join-Path $root "work")
Invoke-Git symbolic-ref HEAD refs/heads/main
git config user.email *> $null
if ($LASTEXITCODE -ne 0) {
    Invoke-Git config user.name "gir client test"
    Invoke-Git config user.email "client-test@example.invalid"
}

gir init
if ($LASTEXITCODE -ne 0) { throw "gir init failed" }
Set-Content -Path base.txt -Value "base" -NoNewline:$false
Invoke-Git add -A
Invoke-Git commit -q -m "chore: base"
Invoke-Git remote add origin (Join-Path $root "remote.git")
Invoke-Git push -q -u origin main

Invoke-Git switch -q -c topic
Set-Content -Path a.txt -Value "one", "two", "three"
Invoke-Git add a.txt
Invoke-Git commit -q -m "feat: add a"
Set-Content -Path b.txt -Value "b"
Invoke-Git add b.txt
Invoke-Git commit -q -m "feat: add b"
Invoke-Git tag client-test-start

Write-Output "ready: open $(Join-Path $root 'work') in your client"
