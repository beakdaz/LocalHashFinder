@echo off

chcp 65001 >nul

cd /d "%~dp0"



set "EXE=%~dp0engine\target\release\LocalHashFinder.exe"

if not exist "%EXE%" (

  echo LocalHashFinder.exe not found.

  echo Run BUILD.bat first.

  pause

  exit /b 1

)



if "%~1"=="" (

  echo Hash plaintext wordlist -^> hash:pass for LMDB import

  echo.

  echo Usage:

  echo   WORDLIST-HASH.bat "passwords.txt"

  echo   WORDLIST-HASH.bat "passwords.txt" sha1

  echo   WORDLIST-HASH.bat "passwords.txt" both 32

  echo.

  echo Default algo: md5. Creates {random}_{stem}_md5.txt next to source.

  echo Example: passwords.txt -^> 847291_passwords_md5.txt

  echo Then: IMPORT-DB.bat "847291_passwords_md5.txt" 4

  pause

  exit /b 1

)



set ALGO=md5

set THREADS=0

if not "%~2"=="" set ALGO=%~2

if not "%~3"=="" set THREADS=%~3



echo Wordlist: %~1

echo Algo: %ALGO%  Threads: %THREADS% ^(0=auto^)

"%EXE%" wordlist-hash "%~1" --algo %ALGO% --threads %THREADS%

if errorlevel 1 pause

exit /b %ERRORLEVEL%

