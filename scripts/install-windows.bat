@echo off
setlocal

set "SCRIPT_DIR=%~dp0"
set "PS1=%SCRIPT_DIR%install-windows.ps1"

if not exist "%PS1%" (
  echo ERROR: Missing installer script: "%PS1%"
  exit /b 1
)

where powershell >nul 2>&1
if %errorlevel% neq 0 (
  echo ERROR: Windows PowerShell is required but was not found.
  exit /b 1
)

echo Running RustPluginSuite Windows installer...
powershell -NoLogo -NoProfile -ExecutionPolicy Bypass -File "%PS1%" %*
set "EXITCODE=%ERRORLEVEL%"

if %EXITCODE% neq 0 (
  echo Installer failed with exit code %EXITCODE%.
  exit /b %EXITCODE%
)

echo Installer completed successfully.
exit /b 0
