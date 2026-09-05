@echo off
chcp 65001 >nul

set "TASK=LocalHashFinder-README-AutoCommit"

schtasks /Delete /TN "%TASK%" /F >nul 2>&1
if errorlevel 1 (
  echo Task was not installed: %TASK%
  echo Nothing to remove. To enable auto-commit, run:
  echo   INSTALL-README-AUTO-COMMIT-TASK.bat
) else (
  echo Removed scheduled task: %TASK%
)

pause
