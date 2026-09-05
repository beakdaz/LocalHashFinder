@echo off
chcp 65001 >nul
cd /d "%~dp0"

set "TASK=LocalHashFinder-README-AutoCommit"
set "RUNNER=%~dp0scripts\run-auto-commit-readme.bat"

echo Installing Windows scheduled task: %TASK%
echo   Every 3 minutes — commit README.md if changed, then push
echo.

schtasks /Query /TN "%TASK%" >nul 2>&1
if not errorlevel 1 (
  echo Removing old task...
  schtasks /Delete /TN "%TASK%" /F >nul
)

schtasks /Create /TN "%TASK%" /TR "\"%RUNNER%\"" /SC MINUTE /MO 3 /F
if errorlevel 1 (
  echo FAILED. Run this bat as Administrator if access denied.
  pause
  exit /b 1
)

powershell -NoProfile -ExecutionPolicy Bypass -File "%~dp0scripts\setup-cursor-readme-hook.ps1"

echo.
echo Done.
echo   Task: %TASK%  ^(every 3 min^)
echo   Cursor hook: .cursor\hooks.json  ^(after agent/Tab edits^)
echo.
echo Optional — instant watcher after save:
echo   WATCH-README-AUTO-COMMIT.bat
echo.
echo Remove: UNINSTALL-README-AUTO-COMMIT-TASK.bat
echo Update hook only: UPDATE-README-HOOK.bat
echo.
pause
