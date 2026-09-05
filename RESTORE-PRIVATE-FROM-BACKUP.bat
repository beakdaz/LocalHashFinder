@echo off

chcp 65001 >nul

cd /d "%~dp0"

set "BACKUP=%~dp0..\LocalHashFinder-PRIVATE-DATA"

echo Restore from: %BACKUP%
echo.

if not exist "%BACKUP%" (
  echo ERROR: backup folder not found:
  echo   %BACKUP%
  echo.
  echo Run MOVE-PRIVATE-FOR-GITHUB.bat first to create a backup.
  pause
  exit /b 1
)

if exist "%BACKUP%\ext" (
  if exist "ext" (
    echo SKIP ext\ — already exists in project
  ) else (
    echo Restoring ext\ ...
    move "%BACKUP%\ext" "ext"
  )
)

if exist "%BACKUP%\wordlists" (
  if exist "wordlists" (
    echo SKIP wordlists\ — already exists in project
  ) else (
    echo Restoring wordlists\ ...
    move "%BACKUP%\wordlists" "wordlists"
  )
)

if exist "%BACKUP%\engine-target-build" (
  if exist "engine\target-build" (
    echo SKIP engine\target-build\ — already exists in project
  ) else (
    echo Restoring engine\target-build\ ...
    move "%BACKUP%\engine-target-build" "engine\target-build"
  )
)

if exist "%BACKUP%\engine-target" (
  if exist "engine\target" (
    echo SKIP engine\target\ — already exists in project
  ) else (
    echo Restoring engine\target\ ...
    move "%BACKUP%\engine-target" "engine\target"
  )
)

echo.
echo Done. Check project folder and backup leftovers:
echo   %BACKUP%
echo.
pause
