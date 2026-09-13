@echo off
setlocal
cd /d "%~dp0\.."
echo Starting ThunderStrike from: %CD%
call npm run tauri dev
if errorlevel 1 pause