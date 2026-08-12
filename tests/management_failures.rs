#![cfg(unix)]

use std::fs;
use std::path::PathBuf;
use std::process::{Command, Output};

struct Fixture {
    _temporary: tempfile::TempDir,
    home: PathBuf,
    state: PathBuf,
    data: PathBuf,
    project: PathBuf,
}

impl Fixture {
    fn new() -> Self {
        let temporary = tempfile::tempdir().unwrap();
        let home = temporary.path().join("home");
        let state = temporary.path().join("state");
        let data = temporary.path().join("data");
        let project = temporary.path().join("project");
        fs::create_dir_all(&home).unwrap();
        fs::create_dir_all(&project).unwrap();
        Self {
            _temporary: temporary,
            home,
            state,
            data,
            project,
        }
    }

    fn command(&self, arguments: &[&str]) -> Output {
        Command::new(env!("CARGO_BIN_EXE_fortlet"))
            .args(arguments)
            .env_clear()
            .env("HOME", &self.home)
            .env("XDG_STATE_HOME", &self.state)
            .env("XDG_DATA_HOME", &self.data)
            .output()
            .unwrap()
    }

    fn assert_no_fortlet_state(&self) {
        assert!(!self.state.join("fortlet").exists());
        assert!(!self.data.join("fortlet").exists());
    }
}

fn assert_failure(output: Output, stage: &str, action: &str, cause: &str) {
    assert!(!output.status.success());
    assert!(output.stdout.is_empty());
    let stderr = String::from_utf8(output.stderr).unwrap();
    let expected = format!("fortlet: {stage} stage failed; {action}: {cause}");
    assert!(stderr.starts_with(&expected), "{stderr}");
}

#[test]
fn status_rejects_unknown_harness_before_project_or_runtime_state() {
    let fixture = Fixture::new();
    let output = fixture.command(&[
        "status",
        "unknown",
        "--project",
        fixture.project.to_str().unwrap(),
    ]);

    assert_failure(
        output,
        "harness",
        "choose a registered harness: codex or tact",
        "unsupported harness \"unknown\"; expected codex or tact",
    );
    fixture.assert_no_fortlet_state();
}

#[test]
fn stop_rejects_missing_project_without_credentials_or_runtime_state() {
    let fixture = Fixture::new();
    let missing = fixture.project.join("missing");
    let output = fixture.command(&["stop", "codex", "--project", missing.to_str().unwrap()]);

    assert_failure(
        output,
        "project",
        "run from a readable project directory or pass --project <path>",
        "cannot resolve project path",
    );
    fixture.assert_no_fortlet_state();
}
