@echo off
chcp 65001 >nul
cd /d "%~dp0"

set "RELEASE=%~dp0engine\target\release"
set "EXE=%RELEASE%\LocalHashFinder.exe"
set "DATA=%RELEASE%\data"
if not exist "%EXE%" (
  echo LocalHashFinder.exe not found.
  echo Run BUILD.bat first.
  pause
  exit /b 1
)

if "%~1"=="" (
  echo Usage: APPEND-DB.bat "D:\new_hash_pass.txt" [map_gb]
  echo.
  echo   Adds new hash:pass lines to existing LMDB.
  echo   Target: engine\target\release\data\hashdb.lmdb
  echo   Duplicate hashes are SKIPPED ^(old password kept^)
  echo   map_gb default 280
  echo.
  echo   Close LocalHashFinder GUI before append!
  pause
  exit /b 1
)

set MAP_GB=280
if not "%~2"=="" set MAP_GB=%~2

if not exist "%DATA%" (
  echo LMDB folder not found: %DATA%
  echo Run IMPORT-DB.bat first for initial import.
  pause
  exit /b 1
)

echo Appending %~1 to %DATA%\hashdb.lmdb ^(map ~%MAP_GB% GB^)
"%EXE%" --db "%DATA%" append "%~1" --map-gb %MAP_GB%
if errorlevel 1 pause
exit /b %ERRORLEVEL%
