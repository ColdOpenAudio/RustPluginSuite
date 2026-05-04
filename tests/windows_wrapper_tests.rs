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
fn powershell_installer_contains_rust_bootstrap_paths() {
    let script = fs::read_to_string("scripts/install-windows.ps1")
        .expect("expected scripts/install-windows.ps1 to exist");

    assert!(script.contains("winget install -e --id Rustlang.Rustup"));
    assert!(script.contains("choco install rustup.install -y"));
    assert!(script.contains("https://win.rustup.rs/x86_64"));
}

#[test]
fn batch_wrapper_invokes_powershell_installer() {
    let script = fs::read_to_string("scripts/install-windows.bat")
        .expect("expected scripts/install-windows.bat to exist");

    assert!(script.contains("ExecutionPolicy Bypass"));
    assert!(script.contains("install-windows.ps1"));
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
