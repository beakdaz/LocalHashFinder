@echo off
chcp 65001 >nul
cd /d "%~dp0"

set "RELEASE=%~dp0engine\target\release"
set "EXE=%RELEASE%\LocalHashFinder.exe"
set "CFG=%RELEASE%\LocalHashFinder.cfg"

if not exist "%EXE%" (
  echo LocalHashFinder.exe not found.
  echo Run BUILD.bat first.
  pause
  exit /b 1
)

if not exist "%CFG%" (
  echo # LocalHashFinder — settings> "%CFG%"
  echo lmdb_path=%RELEASE%\data\hashdb.lmdb>> "%CFG%"
)

set "DATA=%RELEASE%\data"
if not exist "%DATA%" (
  mkdir "%DATA%"
  echo Created: %DATA%
)

echo LocalHashFinder — GUI ^(Lookup, Merge, SQL, Regex, Combo, ULP^)
echo Project: %~dp0
echo Config: %CFG%
echo.
echo Launching:
echo   %EXE%
for %%F in ("%EXE%") do echo   Modified: %%~tF  Size: %%~zF bytes
echo.
start "" /D "%RELEASE%" "%EXE%"
exit /b 0
