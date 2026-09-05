@echo off

setlocal EnableDelayedExpansion

chcp 65001 >nul

cd /d "%~dp0"



rem ========== EDIT YOUR GITHUB USERNAME ==========

set "GITHUB_USER=beakdaz"

rem ==============================================



set "PROJ=%~dp0"

set "BACKUP=%~dp0..\LocalHashFinder-PRIVATE-DATA"

set "REPO=LocalHashFinder"



echo.

echo ========================================

echo  LocalHashFinder - GitHub Publish (all-in-one)

echo ========================================

echo.



if /i "%GITHUB_USER%"=="YOUR_USER" (

  echo ERROR: Edit this file and set GITHUB_USER=your_github_login

  echo   Line: set "GITHUB_USER=YOUR_USER"

  pause

  exit /b 1

)



echo [1/7] Moving private data to backup...

mkdir "%BACKUP%" 2>nul

if exist "ext" (

  if exist "%BACKUP%\ext" rmdir /s /q "%BACKUP%\ext"

  move "ext" "%BACKUP%\ext" >nul

  echo   moved ext\

)

if exist "wordlists" (

  if exist "%BACKUP%\wordlists" rmdir /s /q "%BACKUP%\wordlists"

  move "wordlists" "%BACKUP%\wordlists" >nul

  echo   moved wordlists\

)

if exist "engine\target-build" (

  if exist "%BACKUP%\engine-target-build" rmdir /s /q "%BACKUP%\engine-target-build"

  move "engine\target-build" "%BACKUP%\engine-target-build" >nul

  echo   moved engine\target-build\

)

if exist "engine\target" (

  if exist "%BACKUP%\engine-target" rmdir /s /q "%BACKUP%\engine-target"

  move "engine\target" "%BACKUP%\engine-target" >nul

  echo   moved engine\target\

)

mkdir "wordlists" 2>nul

echo   backup: %BACKUP%

echo.



echo [2/7] Removing old .git (if any)...

if exist ".git" rmdir /s /q ".git"

echo.



echo [3/7] git init + add...

git init

if errorlevel 1 goto :fail

git add .

if errorlevel 1 goto :fail

echo.



echo [4/7] git status — check: NO ext, wordlists, target-build

git status

echo.

echo Press any key if status looks OK (only source + docs + bats)...

pause >nul

echo.



echo [5/7] git commit...

git commit -m "Initial release: LocalHashFinder offline toolkit"

if errorlevel 1 (

  echo Nothing to commit or commit failed.

  goto :fail

)

echo.



echo [6/7] git branch + remote...

git branch -M main

git remote remove origin 2>nul

git remote add origin https://github.com/%GITHUB_USER%/%REPO%.git

echo   origin: https://github.com/%GITHUB_USER%/%REPO%.git

echo.



echo [7/7] git push -u origin main

echo (use gh auth or PAT if prompted)

git push -u origin main

if errorlevel 1 (

  echo.

  echo Push failed. Create repo on GitHub first:

  echo   https://github.com/new  name: %REPO%

  echo Or run: gh repo create %REPO% --public --source=. --remote=origin --push

  pause

  exit /b 1

)



echo.

echo ========================================

echo  DONE: https://github.com/%GITHUB_USER%/%REPO%

echo  Private data: %BACKUP%

echo ========================================

pause

exit /b 0



:fail

echo FAILED. See messages above.

pause

exit /b 1

