[CmdletBinding()]
param(
    [string]$InstallDir = "$env:ProgramFiles\RustPluginSuite",
    [switch]$SkipBuild,
    [switch]$ForceRustup
)

$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest

function Write-Section([string]$Message) {
    Write-Host "`n=== $Message ===" -ForegroundColor Cyan
}

function Ensure-Command([string]$Name, [string]$InstallHint) {
    if (-not (Get-Command $Name -ErrorAction SilentlyContinue)) {
        throw "Required command '$Name' was not found. $InstallHint"
    }
}

function Ensure-Rust {
    if (Get-Command rustup -ErrorAction SilentlyContinue) {
        Write-Host "rustup found."
        return
    }

    Write-Section "Installing Rust toolchain"

    if (-not $ForceRustup) {
        if (Get-Command winget -ErrorAction SilentlyContinue) {
            Write-Host "Installing Rust via winget..."
            winget install -e --id Rustlang.Rustup --accept-source-agreements --accept-package-agreements
            if (Get-Command rustup -ErrorAction SilentlyContinue) { return }
        }

        if (Get-Command choco -ErrorAction SilentlyContinue) {
            Write-Host "Installing Rust via Chocolatey..."
            choco install rustup.install -y
            if (Get-Command rustup -ErrorAction SilentlyContinue) { return }
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

    Ensure-Command "rustup" "Install from https://rustup.rs and rerun this script."
}

function Invoke-Cargo([string]$Arguments) {
    Write-Host "> cargo $Arguments"
    $proc = Start-Process -FilePath "cargo" -ArgumentList $Arguments -Wait -PassThru -NoNewWindow
    if ($proc.ExitCode -ne 0) {
        throw "cargo $Arguments failed with exit code $($proc.ExitCode)."
    }
}

Write-Section "Preparing environment"
Ensure-Command "git" "Install Git for Windows: https://git-scm.com/download/win"
Ensure-Rust

Write-Section "Verifying Rust toolchain"
rustup default stable | Out-Null
Invoke-Cargo "--version"

$scriptRoot = Split-Path -Parent $MyInvocation.MyCommand.Path
$repoRoot = Resolve-Path (Join-Path $scriptRoot "..")
Set-Location $repoRoot

Write-Section "Running quality gates"
Invoke-Cargo "fmt --all -- --check"
Invoke-Cargo "clippy --all-targets --all-features -- -D warnings"
Invoke-Cargo "test --all-targets --all-features"

if (-not $SkipBuild) {
    Write-Section "Building release"
    Invoke-Cargo "build --release --all-targets"
}

Write-Section "Deploying artifacts"
New-Item -ItemType Directory -Path $InstallDir -Force | Out-Null
Copy-Item -Path (Join-Path $repoRoot "README.md") -Destination $InstallDir -Force
if (-not $SkipBuild) {
    $releaseDir = Join-Path $repoRoot "target\release"
    Get-ChildItem -Path $releaseDir -Filter "*.dll" -ErrorAction SilentlyContinue | ForEach-Object {
        Copy-Item -Path $_.FullName -Destination $InstallDir -Force
    }
}

$manifest = @{
    installed_at = (Get-Date).ToString("o")
    install_dir = $InstallDir
    repository = $repoRoot.Path
    skip_build = [bool]$SkipBuild
} | ConvertTo-Json

$manifestPath = Join-Path $InstallDir "install-manifest.json"
Set-Content -Path $manifestPath -Value $manifest -Encoding UTF8

Write-Host "`nInstallation complete. Files were deployed to: $InstallDir" -ForegroundColor Green
