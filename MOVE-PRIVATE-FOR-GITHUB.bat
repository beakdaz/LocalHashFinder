@echo off

chcp 65001 >nul

cd /d "%~dp0"



set "BACKUP=%~dp0..\LocalHashFinder-PRIVATE-DATA"

echo Backup folder: %BACKUP%

mkdir "%BACKUP%" 2>nul



if exist "ext" (

  echo Moving ext\ ...

  if exist "%BACKUP%\ext" rmdir /s /q "%BACKUP%\ext"

  move "ext" "%BACKUP%\ext"

)

if exist "wordlists" (

  echo Moving wordlists\ ...

  if exist "%BACKUP%\wordlists" rmdir /s /q "%BACKUP%\wordlists"

  move "wordlists" "%BACKUP%\wordlists"

)

if exist "engine\target-build" (

  echo Moving engine\target-build\ ...

  if exist "%BACKUP%\engine-target-build" rmdir /s /q "%BACKUP%\engine-target-build"

  move "engine\target-build" "%BACKUP%\engine-target-build"

)

if exist "engine\target" (

  echo Moving engine\target\ ...

  if exist "%BACKUP%\engine-target" rmdir /s /q "%BACKUP%\engine-target"

  move "engine\target" "%BACKUP%\engine-target"

)



echo.

echo Done. Private data moved to:

echo   %BACKUP%

echo.

echo Recreate wordlists folder for local use:

mkdir "wordlists" 2>nul

echo.

pause

