# Auto-commit README.md when it has unstaged or staged changes.
param(
    [switch]$Push,
    [string]$RepoRoot = (Resolve-Path (Join-Path $PSScriptRoot "..")).Path
)

$ErrorActionPreference = "Stop"
Set-Location $RepoRoot

$Doc = "README.md"
$StateFile = Join-Path $PSScriptRoot ".readme-autocommit-state.json"
$CooldownSec = 45

function Test-DocChanged([string]$Path) {
    git diff --quiet -- $Path 2>$null
    $unstaged = $LASTEXITCODE -ne 0
    git diff --cached --quiet -- $Path 2>$null
    $staged = $LASTEXITCODE -ne 0
    return ($unstaged -or $staged)
}

if (-not (Test-Path $Doc)) { exit 0 }
if (-not (Test-DocChanged $Doc)) { exit 0 }

if (Test-Path $StateFile) {
    try {
        $state = Get-Content $StateFile -Raw | ConvertFrom-Json
        $last = [datetime]::Parse($state.lastRun)
        if (((Get-Date) - $last).TotalSeconds -lt $CooldownSec) { exit 0 }
    } catch { }
}

git add -- $Doc

git diff --cached --quiet
if ($LASTEXITCODE -eq 0) { exit 0 }

$stamp = Get-Date -Format "yyyy-MM-dd HH:mm"
$msg = "docs: auto-commit README ($stamp)"
git commit -m $msg
if ($LASTEXITCODE -ne 0) { exit 1 }

@{ lastRun = (Get-Date).ToString("o"); message = $msg } | ConvertTo-Json | Set-Content $StateFile -Encoding UTF8

if ($Push) {
    git push 2>&1 | Out-Null
    if ($LASTEXITCODE -ne 0) {
        Write-Warning "Commit OK, push failed. Run: git push"
        exit 2
    }
}

exit 0
