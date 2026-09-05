@echo off
chcp 65001 >nul
cd /d "%~dp0"

set EXE=%~dp0engine\target\release\TextMerger.exe
if not exist "%EXE%" (
  echo Build first: BUILD.bat  ^(builds TextMerger too^)
  pause
  exit /b 1
)

if "%~1"=="" (
  echo Usage: MERGE-CLEAN.bat "input_folder" "output.txt" [recursive]
  echo.
  echo   Merges all .txt files into one plain-password wordlist.
  echo   Output: one password per line, deduplicated.
  echo.
  echo   REMOVES garbage:
  echo     - lines with :NULL  ^(:null, :Null — case insensitive^)
  echo     - lines shorter than 3 chars
  echo     - empty lines and comments ^(# ;^)
  echo     - hash lines: pure MD5/SHA1 hex, hash:pass, email:hash
  echo     - combo lines: ANY line containing ':' ^(login:pass, email:pass^)
  echo.
  echo   KEEPS only plain passwords: password123, qwerty, MyP@ss!2024
  echo.
  echo   Examples:
  echo     MERGE-CLEAN.bat "D:\wordlists" "D:\merged_clean.txt"
  echo     MERGE-CLEAN.bat "D:\wordlists" "merged.txt" recursive
  pause
  exit /b 1
)

if "%~2"=="" (
  echo Need input folder and output file.
  pause
  exit /b 1
)

set RECURSIVE=
if /I "%~3"=="recursive" set RECURSIVE=--recursive
if /I "%~3"=="--recursive" set RECURSIVE=--recursive

"%EXE%" merge --input "%~1" --output "%~2" %RECURSIVE% --min-len 3
pause
