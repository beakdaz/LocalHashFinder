@echo off
chcp 65001 >nul
cd /d "%~dp0"

echo README/SCRIPTS file watcher — auto-commit ~8s after save, then push
echo Close this window to stop.
echo.

powershell -NoProfile -ExecutionPolicy Bypass -File "%~dp0scripts\watch-readme-commit.ps1" -Push
pause
