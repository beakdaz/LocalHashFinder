@echo off
chcp 65001 >nul
cd /d "%~dp0engine"

where cargo >nul 2>&1
if errorlevel 1 (
  echo Rust not installed.
  echo   winget install Rustlang.Rustup
  echo   or https://rustup.rs
  pause
  exit /b 1
)

echo Building LocalHashFinder + TextMerger release...
echo Path: %CD%
cargo build --release --bin LocalHashFinder --bin TextMerger
if errorlevel 1 (
  pause
  exit /b 1
)

rem Prefer newest deps exe (avoids stale hash artifact when multiple exist)
set "DEPS_EXE="
for /f "delims=" %%F in ('dir /b /o-d "target\release\deps\LocalHashFinder-*.exe" 2^>nul') do (
  set "DEPS_EXE=target\release\deps\%%F"
  goto :deps_found
)
:deps_found
if defined DEPS_EXE (
  copy /Y "%DEPS_EXE%" "target\release\LocalHashFinder.exe" >nul
  echo Copied: %DEPS_EXE%
)

if not exist "target\release\LocalHashFinder.exe" (
  echo ERROR: LocalHashFinder.exe not found after build.
  pause
  exit /b 1
)

if not exist "target\release\data" mkdir "target\release\data"

echo.
set "OUT=%~dp0engine\target\release\LocalHashFinder.exe"
set "MERGE=%~dp0engine\target\release\TextMerger.exe"
echo OK: %OUT%
for %%F in ("%OUT%") do echo Built: %%~tF  size=%%~zF bytes
if exist "%MERGE%" (
  echo OK: %MERGE%
  for %%F in ("%MERGE%") do echo Built: %%~tF  size=%%~zF bytes
)
echo Data: engine\target\release\data\  ^(LMDB created on first run^)
echo Run: START-LOCAL-HASH.bat
echo Merge wordlists: MERGE-CLEAN.bat "folder" "output.txt"
pause
