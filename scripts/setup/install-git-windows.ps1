[CmdletBinding()]
param(
    [switch]$ForceGitInstaller
)

$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest

function Ensure-Command([string]$Name) {
    return [bool](Get-Command $Name -ErrorAction SilentlyContinue)
}

if (Ensure-Command "git") {
    Write-Host "Git already installed: $(git --version)"
    exit 0
}

if (-not $ForceGitInstaller) {
    if (Ensure-Command "winget") {
        Write-Host "Installing Git via winget..."
        winget install -e --id Git.Git --accept-source-agreements --accept-package-agreements
        if (Ensure-Command "git") { Write-Host "Git installed via winget."; exit 0 }
    }

    if (Ensure-Command "choco") {
        Write-Host "Installing Git via Chocolatey..."
        choco install git -y
        if (Ensure-Command "git") { Write-Host "Git installed via Chocolatey."; exit 0 }
    }
}

Write-Host "Installing Git via official installer..."
$tmp = Join-Path $env:TEMP "Git-setup.exe"
Invoke-WebRequest -Uri "https://github.com/git-for-windows/git/releases/latest/download/Git-64-bit.exe" -OutFile $tmp
Start-Process -FilePath $tmp -ArgumentList "/VERYSILENT /NORESTART" -Wait -NoNewWindow

if (-not (Ensure-Command "git")) {
    throw "Git install failed. Install manually from https://git-scm.com/download/win"
}

Write-Host "Git installed: $(git --version)"
