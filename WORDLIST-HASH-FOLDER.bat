@echo off
setlocal EnableDelayedExpansion

chcp 65001 >nul
cd /d "%~dp0"

set EXE=%~dp0engine\target\release\LocalHashFinder.exe
if not exist "%EXE%" (
  echo Build first: BUILD.bat
  pause
  exit /b 1
)

set "FOLDER=%~1"
if "%FOLDER%"=="" set "FOLDER=%~dp0wordlists"

if not exist "%FOLDER%\" (
  echo Folder not found: %FOLDER%
  echo.
  echo Usage:
  echo   WORDLIST-HASH-FOLDER.bat "D:\wordlists"
  echo   WORDLIST-HASH-FOLDER.bat "D:\wordlists" sha1
  echo   WORDLIST-HASH-FOLDER.bat "D:\wordlists" both 32
  echo   WORDLIST-HASH-FOLDER.bat "D:\wordlists" md5 32
  echo.
  echo No arg uses default: wordlists\ next to this bat. Algo default: md5.
  echo Output per file: {random}_{stem}_md5.txt ^(new random prefix each run^).
  echo Skips: *_md5.txt, *_sha1.txt, hash:pass files, prior hash outputs.
  pause
  exit /b 1
)

set ALGO=md5
set THREADS=0
if not "%~2"=="" set ALGO=%~2
if not "%~3"=="" set THREADS=%~3

set COUNT=0
for %%F in ("%FOLDER%\*.txt") do (
  call :ShouldSkip "%%F" "%%~nF"
  if !SKIP!==0 set /a COUNT+=1
)

if !COUNT!==0 (
  echo No .txt files to process in: %FOLDER%
  echo ^(skips hash outputs, *_md5.txt, *_sha1.txt, hash:pass content^)
  pause
  exit /b 0
)

echo Folder: %FOLDER%
echo Algo: %ALGO%  Threads: %THREADS% ^(0=auto^)
echo Output: {random}_{stem}_md5.txt / _sha1.txt per file
echo Files: !COUNT!
echo.

set IDX=0
set FAILED=0
for %%F in ("%FOLDER%\*.txt") do (
  call :ShouldSkip "%%F" "%%~nF"
  if !SKIP!==0 (
    set /a IDX+=1
    echo [!IDX!/!COUNT!] processing %%~nxF...
    "%EXE%" wordlist-hash "%%F" --algo %ALGO% --threads %THREADS%
    if errorlevel 1 (
      echo FAILED: %%~nxF
      set /a FAILED+=1
    )
    echo.
  )
)

echo Done. Processed !IDX! file^(s^), !FAILED! failed.
if !FAILED! gtr 0 pause
exit /b !FAILED!

:ShouldSkip
set "SKIP=0"
set "CHKFILE=%~1"
set "CHKSTEM=%~2"

if /i "!CHKSTEM:~-4!"=="_md5" set "SKIP=1" & goto :ShouldSkipDone
if /i "!CHKSTEM:~-5!"=="_sha1" set "SKIP=1" & goto :ShouldSkipDone

powershell -NoProfile -ExecutionPolicy Bypass -Command "if ('!CHKSTEM!' -match '^\d+_.+_(md5|sha1)$') { exit 0 } else { exit 1 }" >nul 2>&1
if not errorlevel 1 set "SKIP=1" & goto :ShouldSkipDone

powershell -NoProfile -ExecutionPolicy Bypass -Command "$p='!CHKFILE!'; $line = Get-Content -LiteralPath $p -TotalCount 20 -ErrorAction SilentlyContinue | Where-Object { $t = $_.Trim(); $t -ne '' -and -not $t.StartsWith('#') -and -not $t.StartsWith(';') } | Select-Object -First 1; if ($line -match '^(?:[0-9a-fA-F]{32}|[0-9a-fA-F]{40}):') { exit 0 } else { exit 1 }" >nul 2>&1
if not errorlevel 1 set "SKIP=1"

:ShouldSkipDone
exit /b 0
