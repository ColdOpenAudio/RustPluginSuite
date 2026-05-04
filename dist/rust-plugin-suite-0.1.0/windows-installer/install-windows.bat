@echo off
setlocal

set "SCRIPT_DIR=%~dp0"
set "BOOTSTRAP=%SCRIPT_DIR%install-windows-bootstrap.ps1"

if not exist "%BOOTSTRAP%" (
  echo ERROR: Missing bootstrap script: "%BOOTSTRAP%"
  exit /b 1
)

set "PSHOST="

where pwsh >nul 2>&1
if %errorlevel% equ 0 (
  set "PSHOST=pwsh"
)

if not defined PSHOST (
  where powershell >nul 2>&1
  if %errorlevel% equ 0 (
    set "PSHOST=powershell"
  )
)

if not defined PSHOST (
  where winget >nul 2>&1
  if %errorlevel% equ 0 (
    echo PowerShell was not found. Installing PowerShell 7 via winget...
    winget install --id Microsoft.PowerShell --exact --source winget --accept-source-agreements --accept-package-agreements
    if exist "%ProgramFiles%\PowerShell\7\pwsh.exe" (
      set "PSHOST=%ProgramFiles%\PowerShell\7\pwsh.exe"
    )
    if not defined PSHOST (
      if exist "%ProgramFiles(x86)%\PowerShell\7\pwsh.exe" (
        set "PSHOST=%ProgramFiles(x86)%\PowerShell\7\pwsh.exe"
      )
    )
  )
)

if not defined PSHOST (
  echo ERROR: No usable PowerShell host was found.
  echo Install PowerShell 7 with: winget install --id Microsoft.PowerShell --source winget
  exit /b 1
)

echo Running RustPluginSuite Windows bootstrap...
"%PSHOST%" -NoLogo -NoProfile -ExecutionPolicy Bypass -File "%BOOTSTRAP%" %*
set "EXITCODE=%ERRORLEVEL%"

if %EXITCODE% neq 0 (
  echo Bootstrap failed with exit code %EXITCODE%.
  exit /b %EXITCODE%
)

echo Bootstrap and installer completed successfully.
exit /b 0
