[CmdletBinding()]
param(
    [string]$InstallDir = "$env:ProgramFiles\RustPluginSuite",
    [switch]$SkipBuild,
    [switch]$ForceRustup,
    [switch]$UseCurrentPowerShell
)

$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest

function Write-Section([string]$Message) {
    Write-Host "`n=== $Message ===" -ForegroundColor Cyan
}

function Ensure-Command([string]$Name, [string]$InstallHint) {
    $command = Get-Command $Name -ErrorAction SilentlyContinue
    if (-not $command) {
        throw "Required command '$Name' was not found. $InstallHint"
    }

    return $command.Source
}

function Find-Pwsh {
    $command = Get-Command "pwsh" -ErrorAction SilentlyContinue
    if ($command) {
        return $command.Source
    }

    $candidates = @(
        "$env:ProgramFiles\PowerShell\7\pwsh.exe",
        "${env:ProgramFiles(x86)}\PowerShell\7\pwsh.exe"
    )

    foreach ($candidate in $candidates) {
        if ($candidate -and (Test-Path $candidate)) {
            return $candidate
        }
    }

    return $null
}

function Install-PowerShellCore {
    if (Get-Command "winget" -ErrorAction SilentlyContinue) {
        Write-Host "Installing PowerShell 7 via winget..."
        winget install --id Microsoft.PowerShell --exact --source winget --accept-source-agreements --accept-package-agreements
        $pwsh = Find-Pwsh
        if ($pwsh) {
            return $pwsh
        }
    }

    if (Get-Command "choco" -ErrorAction SilentlyContinue) {
        Write-Host "Installing PowerShell 7 via Chocolatey..."
        choco install powershell-core -y
        $pwsh = Find-Pwsh
        if ($pwsh) {
            return $pwsh
        }
    }

    throw "PowerShell 7 was not found and could not be installed automatically. Install Microsoft.PowerShell with winget or the official MSI, then rerun this script."
}

function Ensure-PowerShellCore {
    $pwsh = Find-Pwsh
    if ($pwsh) {
        Write-Host "PowerShell 7 found: $pwsh"
        return $pwsh
    }

    Write-Section "Installing PowerShell 7"
    return Install-PowerShellCore
}

function Ensure-Rust {
    if (Get-Command "rustup" -ErrorAction SilentlyContinue) {
        Write-Host "rustup found."
        return
    }

    Write-Section "Installing Rust toolchain"

    if (-not $ForceRustup) {
        if (Get-Command "winget" -ErrorAction SilentlyContinue) {
            Write-Host "Installing Rust via winget..."
            winget install -e --id Rustlang.Rustup --accept-source-agreements --accept-package-agreements
            if (Get-Command "rustup" -ErrorAction SilentlyContinue) {
                return
            }
        }

        if (Get-Command "choco" -ErrorAction SilentlyContinue) {
            Write-Host "Installing Rust via Chocolatey..."
            choco install rustup.install -y
            if (Get-Command "rustup" -ErrorAction SilentlyContinue) {
                return
            }
        }
    }

    Write-Host "Installing Rust via official rustup-init.exe..."
    $tmp = Join-Path $env:TEMP "rustup-init.exe"
    Invoke-WebRequest -Uri "https://win.rustup.rs/x86_64" -OutFile $tmp
    Start-Process -FilePath $tmp -ArgumentList "-y" -Wait -NoNewWindow

    $cargoBin = Join-Path $env:USERPROFILE ".cargo\bin"
    if (Test-Path $cargoBin) {
        $env:PATH = "$cargoBin;$env:PATH"
    }

    Ensure-Command "rustup" "Install from https://rustup.rs and rerun this script." | Out-Null
}

function Invoke-Script([string]$PowerShellPath, [string]$ScriptPath, [string[]]$ScriptArguments) {
    $arguments = @("-NoLogo", "-NoProfile", "-ExecutionPolicy", "Bypass", "-File", $ScriptPath) + $ScriptArguments
    $argumentLine = ($arguments | ForEach-Object { ConvertTo-ProcessArgument $_ }) -join " "
    $proc = Start-Process -FilePath $PowerShellPath -ArgumentList $argumentLine -Wait -PassThru -NoNewWindow
    if ($proc.ExitCode -ne 0) {
        throw "Script failed: $ScriptPath (exit code $($proc.ExitCode))."
    }
}

function ConvertTo-ProcessArgument([string]$Value) {
    if ($Value -notmatch '[\s"]') {
        return $Value
    }

    return '"' + ($Value -replace '"', '\"') + '"'
}

function New-BootstrapArguments([bool]$IncludeUseCurrentPowerShell) {
    $arguments = @("-InstallDir", $InstallDir)
    if ($SkipBuild) {
        $arguments += "-SkipBuild"
    }
    if ($ForceRustup) {
        $arguments += "-ForceRustup"
    }
    if ($IncludeUseCurrentPowerShell) {
        $arguments += "-UseCurrentPowerShell"
    }

    return $arguments
}

function New-InstallerArguments {
    $arguments = @("-InstallDir", $InstallDir)
    if ($SkipBuild) {
        $arguments += "-SkipBuild"
    }

    return $arguments
}

$scriptRoot = Split-Path -Parent $MyInvocation.MyCommand.Path
$installer = Join-Path $scriptRoot "install-windows.ps1"

Write-Section "Preparing Windows host"
$pwsh = Ensure-PowerShellCore

if (-not $UseCurrentPowerShell -and $PSVersionTable.PSEdition -ne "Core") {
    Write-Host "Re-entering bootstrap under PowerShell 7..."
    Invoke-Script $pwsh $MyInvocation.MyCommand.Path (New-BootstrapArguments $true)
    exit 0
}

Ensure-Command "git" "Install Git for Windows: https://git-scm.com/download/win" | Out-Null
Ensure-Rust

Write-Section "Verifying Rust toolchain"
rustup default stable | Out-Null
Ensure-Command "cargo" "Install Rust from https://rustup.rs and rerun this script." | Out-Null

Write-Section "Running product installer"
Invoke-Script $pwsh $installer (New-InstallerArguments)
