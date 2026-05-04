use std::fs;

#[test]
fn powershell_installer_contains_required_quality_gates() {
    let script = fs::read_to_string("scripts/install-windows.ps1")
        .expect("expected scripts/install-windows.ps1 to exist");

    assert!(script.contains("fmt --all -- --check"));
    assert!(script.contains("clippy --all-targets --all-features -- -D warnings"));
    assert!(script.contains("test --all-targets --all-features"));
    assert!(script.contains("build --release --all-targets"));
}

#[test]
fn powershell_bootstrap_contains_powershell_core_ladder() {
    let script = fs::read_to_string("scripts/install-windows-bootstrap.ps1")
        .expect("expected scripts/install-windows-bootstrap.ps1 to exist");

    assert!(script.contains("Microsoft.PowerShell"));
    assert!(script.contains("powershell-core"));
    assert!(script.contains("PowerShell\\7\\pwsh.exe"));
    assert!(script.contains("Re-entering bootstrap under PowerShell 7"));
}

#[test]
fn powershell_bootstrap_contains_rust_bootstrap_paths() {
    let script = fs::read_to_string("scripts/install-windows-bootstrap.ps1")
        .expect("expected scripts/install-windows-bootstrap.ps1 to exist");

    assert!(script.contains("winget install -e --id Rustlang.Rustup"));
    assert!(script.contains("choco install rustup.install -y"));
    assert!(script.contains("https://win.rustup.rs/x86_64"));
}

#[test]
fn batch_wrapper_invokes_powershell_installer() {
    let script = fs::read_to_string("scripts/install-windows.bat")
        .expect("expected scripts/install-windows.bat to exist");

    assert!(script.contains("where pwsh"));
    assert!(script.contains("where powershell"));
    assert!(script.contains("winget install --id Microsoft.PowerShell"));
    assert!(script.contains("install-windows-bootstrap.ps1"));
    assert!(script.contains("ExecutionPolicy Bypass"));
}

#[test]
fn release_package_includes_windows_bootstrap() {
    let script = fs::read_to_string("scripts/release/package-release.sh")
        .expect("expected scripts/release/package-release.sh to exist");

    assert!(script.contains("install-windows-bootstrap.ps1"));
}

#[test]
fn setup_install_scripts_include_primary_install_paths() {
    let git_script = fs::read_to_string("scripts/setup/install-git-windows.ps1")
        .expect("expected scripts/setup/install-git-windows.ps1 to exist");
    assert!(git_script.contains("winget install -e --id Git.Git"));
    assert!(git_script.contains("choco install git -y"));
    assert!(git_script.contains("Git-64-bit.exe"));

    let rust_script = fs::read_to_string("scripts/setup/install-rust-windows.ps1")
        .expect("expected scripts/setup/install-rust-windows.ps1 to exist");
    assert!(rust_script.contains("winget install -e --id Rustlang.Rustup"));
    assert!(rust_script.contains("choco install rustup.install -y"));
    assert!(rust_script.contains("https://win.rustup.rs/x86_64"));
}
