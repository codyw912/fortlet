#![cfg(unix)]

use std::fs;
use std::os::unix::fs::{symlink, PermissionsExt};
use std::process::Command;

#[test]
fn native_execution_skips_the_fortlet_shim_and_preserves_arguments() {
    let temporary = tempfile::tempdir().unwrap();
    let shim_directory = temporary.path().join("shims");
    let native_directory = temporary.path().join("native");
    fs::create_dir_all(&shim_directory).unwrap();
    fs::create_dir_all(&native_directory).unwrap();
    let fortlet = env!("CARGO_BIN_EXE_fortlet");
    symlink(fortlet, shim_directory.join("codex")).unwrap();
    let native = native_directory.join("codex");
    fs::write(&native, "#!/bin/sh\nprintf '%s\\n' \"$@\"\n").unwrap();
    let mut permissions = fs::metadata(&native).unwrap().permissions();
    permissions.set_mode(0o755);
    fs::set_permissions(&native, permissions).unwrap();
    let path = std::env::join_paths([&shim_directory, &native_directory]).unwrap();

    let output = Command::new(fortlet)
        .args(["native", "codex", "--", "--flag", "two words"])
        .env("PATH", path)
        .env("FORTLET_SHIM_DIR", &shim_directory)
        .output()
        .unwrap();

    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        "--flag\ntwo words\n"
    );
}

#[test]
fn native_execution_fails_actionably_when_only_the_shim_exists() {
    let temporary = tempfile::tempdir().unwrap();
    let shim_directory = temporary.path().join("shims");
    fs::create_dir_all(&shim_directory).unwrap();
    let fortlet = env!("CARGO_BIN_EXE_fortlet");
    symlink(fortlet, shim_directory.join("tact")).unwrap();

    let output = Command::new(fortlet)
        .args(["native", "tact"])
        .env("PATH", &shim_directory)
        .env("FORTLET_SHIM_DIR", &shim_directory)
        .output()
        .unwrap();

    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr)
        .contains("add a host tact executable after Fortlet's shim directory on PATH"));
}
