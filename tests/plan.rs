#![cfg(unix)]

use std::fs;
use std::path::Path;
use std::process::{Command, Output};

fn run(project: &Path, home: &Path, state: &Path, data: &Path) -> Output {
    Command::new(env!("CARGO_BIN_EXE_fortlet"))
        .args(["plan", "--project", project.to_str().unwrap()])
        .env_clear()
        .env("HOME", home)
        .env("XDG_STATE_HOME", state)
        .env("XDG_DATA_HOME", data)
        .output()
        .unwrap()
}

#[test]
fn schema_two_plan_is_stable_redacted_and_side_effect_free() {
    let temporary = tempfile::tempdir().unwrap();
    let home = temporary.path().join("home");
    let project = temporary.path().join("public-project");
    let state = temporary.path().join("state");
    let data = temporary.path().join("data");
    fs::create_dir(&home).unwrap();
    fs::create_dir(&project).unwrap();
    fs::create_dir(project.join(".fortlet")).unwrap();
    fs::write(
        project.join(".fortlet/environment.json"),
        r#"{"schema":2,"provider":{"kind":"nix-dev-shell","name":"default"}}"#,
    )
    .unwrap();
    fs::write(project.join("flake.nix"), "{ outputs = _: {}; }\n").unwrap();
    fs::write(project.join("flake.lock"), "{}\n").unwrap();

    let output = run(&project, &home, &state, &data);

    assert!(output.status.success());
    assert!(output.stderr.is_empty());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("provider\tnix-dev-shell\n"));
    assert!(stdout.contains("selection\tdefault\n"));
    assert!(stdout.contains("preparation\tunprepared\n"));
    assert!(stdout.contains("identity-projection\tunchecked\n"));
    assert!(!stdout.contains(temporary.path().to_str().unwrap()));
    assert!(!state.exists());
    assert!(!data.exists());
}

#[test]
fn invalid_plan_fails_without_auth_state_or_runtime_mutation() {
    let temporary = tempfile::tempdir().unwrap();
    let home = temporary.path().join("home");
    let project = temporary.path().join("public-project");
    let state = temporary.path().join("state");
    let data = temporary.path().join("data");
    fs::create_dir(&home).unwrap();
    fs::create_dir(&project).unwrap();
    fs::create_dir(project.join(".fortlet")).unwrap();
    fs::write(
        project.join(".fortlet/environment.json"),
        r#"{"schema":2,"provider":{"kind":"unknown","name":"default"}}"#,
    )
    .unwrap();

    let output = run(&project, &home, &state, &data);

    assert!(!output.status.success());
    assert!(output.stdout.is_empty());
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(stderr.contains("project-environment stage failed"));
    assert!(stderr.contains("fix or remove .fortlet/environment.json and retry"));
    assert!(!state.exists());
    assert!(!data.exists());
}

#[test]
fn broad_root_plan_does_not_create_the_scratch_workspace() {
    let temporary = tempfile::tempdir().unwrap();
    let home = temporary.path().join("home");
    let state = temporary.path().join("state");
    let data = temporary.path().join("data");
    fs::create_dir(&home).unwrap();

    let output = run(&home, &home, &state, &data);

    assert!(output.status.success());
    assert!(String::from_utf8(output.stdout)
        .unwrap()
        .contains("project\tscratch\n"));
    assert!(!state.exists());
    assert!(!data.exists());
}
