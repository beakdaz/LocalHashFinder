# Creates .cursor/hooks.json + hook script (local only; .cursor/ is gitignored).
param(
    [string]$RepoRoot = (Resolve-Path (Join-Path $PSScriptRoot "..")).Path
)

$cursorDir = Join-Path $RepoRoot ".cursor"
$hooksDir = Join-Path $cursorDir "hooks"
New-Item -ItemType Directory -Force -Path $hooksDir | Out-Null

$hookScript = Join-Path $hooksDir "auto-commit-readme-hook.ps1"
@'
$ErrorActionPreference = "SilentlyContinue"
$raw = [Console]::In.ReadToEnd()
if ($raw -notmatch "README\.md") { exit 0 }

$repo = (Resolve-Path (Join-Path $PSScriptRoot "..\..")).Path

$ps1 = Join-Path $repo "scripts\auto-commit-readme.ps1"
if (-not (Test-Path $ps1)) { exit 0 }

Start-Process powershell.exe -WindowStyle Hidden -ArgumentList @(
    "-NoProfile", "-ExecutionPolicy", "Bypass", "-File", "`"$ps1`"", "-Push"
) -WorkingDirectory $repo | Out-Null
exit 0
'@ | Set-Content $hookScript -Encoding UTF8

$hooksJson = Join-Path $cursorDir "hooks.json"
@'
{
  "version": 1,
  "hooks": {
    "afterFileEdit": [
      {
        "command": "powershell -NoProfile -ExecutionPolicy Bypass -File .cursor/hooks/auto-commit-readme-hook.ps1"
      }
    ]
  }
}
'@ | Set-Content $hooksJson -Encoding UTF8

Write-Host "Cursor hook installed:"
Write-Host "  $hooksJson"
Write-Host "Restart Cursor if hooks do not load immediately."
