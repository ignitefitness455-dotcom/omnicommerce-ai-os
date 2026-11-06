@echo off
setlocal

echo ========================================================
echo   OmniCommerce AI OS - GitHub Quick Push & Deploy
echo ========================================================
echo.

where git >nul 2>nul
if %errorlevel% neq 0 (
    echo [ERROR] Git is not installed or not found in PATH!
    echo Please download and install Git from: https://git-scm.com/download/win
    echo Then run this script again.
    pause
    exit /b 1
)

set /p REPO_URL="Enter your GitHub Repository URL (e.g. https://github.com/your-username/omnicommerce-ai-os.git): "

if "%REPO_URL%"=="" (
    echo [ERROR] Repository URL cannot be empty.
    pause
    exit /b 1
)

echo.
echo [1/4] Initializing local Git repository...
if not exist .git (
    git init -b main
)

echo [2/4] Staging all files...
git add .

echo [3/4] Creating initial commit...
git commit -m "feat: complete Phase 1-4 OmniCommerce AI OS, Fraud Shield, Webhooks, AI Haggling & CI/CD"

echo [4/4] Pushing to GitHub (%REPO_URL%)...
git remote remove origin 2>nul
git remote add origin %REPO_URL%
git push -u origin main --force

echo.
echo ========================================================
echo [SUCCESS] Your code is pushed to GitHub!
echo GitHub Actions will now automatically:
echo  1. Verify Rust workspace compilation
echo  2. Deploy the Interactive Command Center to GitHub Pages!
echo ========================================================
pause
