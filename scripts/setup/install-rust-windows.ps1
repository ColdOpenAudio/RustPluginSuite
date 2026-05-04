[CmdletBinding()]
param(
    [switch]$ForceRustup
)

$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest

function Ensure-Command([string]$Name) {
    return [bool](Get-Command $Name -ErrorAction SilentlyContinue)
}

if (Ensure-Command "rustup") {
    rustup default stable | Out-Null
    Write-Host "Rust already installed: $(rustc --version)"
    exit 0
}

if (-not $ForceRustup) {
    if (Ensure-Command "winget") {
        Write-Host "Installing Rust via winget..."
        winget install -e --id Rustlang.Rustup --accept-source-agreements --accept-package-agreements
        if (Ensure-Command "rustup") { rustup default stable | Out-Null; Write-Host "Rust installed via winget."; exit 0 }
    }

    if (Ensure-Command "choco") {
        Write-Host "Installing Rust via Chocolatey..."
        choco install rustup.install -y
        if (Ensure-Command "rustup") { rustup default stable | Out-Null; Write-Host "Rust installed via Chocolatey."; exit 0 }
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

if (-not (Ensure-Command "rustup")) {
    throw "Rust install failed. Install manually from https://rustup.rs"
}

rustup default stable | Out-Null
Write-Host "Rust installed: $(rustc --version)"
