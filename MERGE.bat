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
  echo Usage: MERGE.bat "mail_hash.txt" "dehash_good.txt"
  echo.
  echo   mail file:   user@gmail.com:md5hash
  echo   dehash file: md5hash:plainpass  ^(_good.txt^)
  echo   output:      mail_plain.txt  +  mail_plain_nohash.txt
  pause
  exit /b 1
)

if "%~2"=="" (
  echo Need two files: mail list and dehash/good file
  pause
  exit /b 1
)

"%EXE%" merge --mail "%~1" --dehash "%~2"
pause
