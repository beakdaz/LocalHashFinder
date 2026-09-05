@echo off
chcp 65001 >nul
cd /d "%~dp0"

echo === Step 1: run MOVE-PRIVATE-FOR-GITHUB.bat first if not done ===
echo.

if exist ".git" (
  echo Removing old .git history ^(bad commit with build artifacts^)...
  rmdir /s /q ".git"
)

echo Initializing fresh git repo...
git init
git add .
echo.
echo === git status — verify NO ext, wordlists, target-build ===
git status
echo.
echo If OK, run:
echo   git commit -m "Initial release: LocalHashFinder offline toolkit"
echo   git branch -M main
echo   git remote add origin https://github.com/YOUR_USER/LocalHashFinder.git
echo   git push -u origin main
echo.
pause
