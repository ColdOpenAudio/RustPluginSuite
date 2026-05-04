[CmdletBinding()]
param(
    [string]$InstallDir = "$env:ProgramFiles\RustPluginSuite",
    [switch]$SkipBuild
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

function Invoke-Process([string]$Label, [string]$FilePath, [string[]]$Arguments) {
    Write-Host "> $Label"
    $proc = Start-Process -FilePath $FilePath -ArgumentList $Arguments -Wait -PassThru -NoNewWindow
    if ($proc.ExitCode -ne 0) {
        throw "Step failed: $Label (exit code $($proc.ExitCode))."
    }
}

Write-Section "Preparing environment"
Ensure-Command "git" "Install Git for Windows: https://git-scm.com/download/win"
Ensure-Command "rustup" "Run scripts\install-windows-bootstrap.ps1 to bootstrap Rust."
Ensure-Command "cargo" "Run scripts\install-windows-bootstrap.ps1 to bootstrap Rust."

Write-Section "Verifying Rust toolchain"
rustup default stable | Out-Null
Invoke-Process "cargo --version" "cargo" @("--version")

$scriptRoot = Split-Path -Parent $MyInvocation.MyCommand.Path
$repoRoot = Resolve-Path (Join-Path $scriptRoot "..")
Set-Location $repoRoot

Write-Section "Running quality gates"
Invoke-Process "cargo fmt --all -- --check" "cargo" @("fmt", "--all", "--", "--check")
Invoke-Process "cargo clippy --all-targets --all-features -- -D warnings" "cargo" @("clippy", "--all-targets", "--all-features", "--", "-D", "warnings")
Invoke-Process "cargo test --all-targets --all-features" "cargo" @("test", "--all-targets", "--all-features")

if (-not $SkipBuild) {
    Write-Section "Building release"
    Invoke-Process "cargo build --release --all-targets" "cargo" @("build", "--release", "--all-targets")
}

Write-Section "Deploying artifacts"
New-Item -ItemType Directory -Path $InstallDir -Force | Out-Null
Copy-Item -Path (Join-Path $repoRoot "README.md") -Destination $InstallDir -Force
if (-not $SkipBuild) {
    $releaseDir = Join-Path $repoRoot "target\release"
    $artifacts = @(Get-ChildItem -Path $releaseDir -Filter "*.dll" -ErrorAction SilentlyContinue)
    if ($artifacts.Count -eq 0) {
        throw "No Windows DLL artifacts were found in $releaseDir after release build."
    }

    $artifacts | ForEach-Object {
        Copy-Item -Path $_.FullName -Destination $InstallDir -Force
    }
}

$manifest = @{
    installed_at = (Get-Date).ToString("o")
    install_dir = $InstallDir
    repository = $repoRoot.Path
    powershell_edition = $PSVersionTable.PSEdition
    powershell_version = $PSVersionTable.PSVersion.ToString()
    skip_build = [bool]$SkipBuild
} | ConvertTo-Json

$manifestPath = Join-Path $InstallDir "install-manifest.json"
Set-Content -Path $manifestPath -Value $manifest -Encoding UTF8

Write-Host "`nInstallation complete. Files were deployed to: $InstallDir" -ForegroundColor Green
