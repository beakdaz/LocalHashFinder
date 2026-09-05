@echo off
chcp 65001 >nul

set "TASK=LocalHashFinder-README-AutoCommit"

schtasks /Delete /TN "%TASK%" /F 2>nul
if errorlevel 1 (
  echo Task not found or already removed: %TASK%
) else (
  echo Removed scheduled task: %TASK%
)

pause
