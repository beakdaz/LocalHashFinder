# File watcher: commit README/SCRIPTS ~8s after save (Ctrl+C to stop).
param(
    [switch]$Push,
    [string]$RepoRoot = (Resolve-Path (Join-Path $PSScriptRoot "..")).Path
)

$ErrorActionPreference = "Stop"
Set-Location $RepoRoot

$AutoScript = Join-Path $PSScriptRoot "auto-commit-readme.ps1"
$Docs = @("README.md")
$lastSeen = @{}
$pending = $false
$pendingAt = $null
$debounceSec = 8

Write-Host "Watching README.md in:"
Write-Host "  $RepoRoot"
Write-Host "Auto-commit ~${debounceSec}s after save. Ctrl+C to stop."
Write-Host ""

while ($true) {
    foreach ($doc in $Docs) {
        $path = Join-Path $RepoRoot $doc
        if (-not (Test-Path $path)) { continue }
        $write = (Get-Item $path).LastWriteTimeUtc
        if ($lastSeen.ContainsKey($doc) -and $write -gt $lastSeen[$doc]) {
            $pending = $true
            $pendingAt = Get-Date
        }
        $lastSeen[$doc] = $write
    }

    if ($pending -and $pendingAt -and (((Get-Date) - $pendingAt).TotalSeconds -ge $debounceSec)) {
        $pending = $false
        $pendingAt = $null
        if ($Push) {
            & $AutoScript -Push
        } else {
            & $AutoScript
        }
        $code = $LASTEXITCODE
        if ($code -eq 0) {
            Write-Host "[$(Get-Date -Format HH:mm:ss)] No changes or already committed."
        } elseif ($code -eq 2) {
            Write-Host "[$(Get-Date -Format HH:mm:ss)] Committed; push failed — run git push"
        } else {
            Write-Host "[$(Get-Date -Format HH:mm:ss)] Committed and pushed."
        }
    }

    Start-Sleep -Seconds 2
}
