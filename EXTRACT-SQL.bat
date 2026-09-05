@echo off
chcp 65001 >nul
cd /d "%~dp0"

set EXE=%~dp0engine\target\release\LocalHashFinder.exe
if not exist "%EXE%" (
  echo LocalHashFinder.exe not found.
  echo Run BUILD.bat first.
  pause
  exit /b 1
)

if "%~1"=="" (
  echo Usage: EXTRACT-SQL.bat "dump.sql" [output.txt]
  echo.
  echo   Parses .sql via regex, writes only:
  echo     email@domain.com:32hex_md5
  echo     email@domain.com:40hex_sha1
  pause
  exit /b 1
)

if "%~2"=="" (
  "%EXE%" extract-sql "%~1"
) else (
  "%EXE%" extract-sql "%~1" -o "%~2"
)
pause
