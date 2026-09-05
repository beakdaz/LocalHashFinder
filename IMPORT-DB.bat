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
  echo Usage: IMPORT-DB.bat "D:\path\to\hash_pass.txt" [map_gb]
  echo.
  echo   First-time import of hash:pass lines into LMDB.
  echo   Target: engine\target\release\data\hashdb.lmdb
  echo   map_gb default 280 for ~200 GB source file
  echo.
  echo   Close LocalHashFinder GUI before import!
  pause
  exit /b 1
)

set MAP_GB=280
if not "%~2"=="" set MAP_GB=%~2

if not exist "%DATA%" mkdir "%DATA%"

echo Importing %~1 into %DATA%\hashdb.lmdb ^(map ~%MAP_GB% GB^)
echo This can take many hours for large files. Do not close the window.
"%EXE%" --db "%DATA%" import "%~1" --map-gb %MAP_GB%
if errorlevel 1 pause
exit /b %ERRORLEVEL%
