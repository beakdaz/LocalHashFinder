@echo off
chcp 65001 >nul
cd /d "%~dp0"

where node >nul 2>&1
if errorlevel 1 (
  echo Node.js not installed.
  echo   winget install OpenJS.NodeJS.LTS
  echo   or https://nodejs.org
  pause
  exit /b 1
)

if not exist "server.js" (
  echo server.js not found in project root.
  echo.
  echo Usage: START-WEB.bat
  echo   Legacy web UI ^(Node.js^). Requires server.js in project root.
  echo   For LMDB use LocalHashFinder.exe ^(START-LOCAL-HASH.bat^).
  pause
  exit /b 1
)

echo Local Hash Finder — web UI ^(Node, legacy^)
echo Open: http://127.0.0.1:8787
echo.
echo For LMDB use LocalHashFinder.exe ^(START-LOCAL-HASH.bat^)
node server.js
if errorlevel 1 pause
exit /b %ERRORLEVEL%
