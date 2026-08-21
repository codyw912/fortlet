#![cfg(unix)]

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

struct Fixture {
    _temporary: tempfile::TempDir,
    home: PathBuf,
    state: PathBuf,
    data: PathBuf,
    project: PathBuf,
    auth: PathBuf,
}

impl Fixture {
    fn new() -> Self {
        let temporary = tempfile::tempdir().unwrap();
        let home = temporary.path().join("home");
        let state = temporary.path().join("state");
        let data = temporary.path().join("data");
        let project = temporary.path().join("project");
        let auth = project.join("invalid-auth.json");
        fs::create_dir_all(&home).unwrap();
        fs::create_dir_all(&project).unwrap();
        fs::write(&auth, "not a credential").unwrap();
        Self {
            _temporary: temporary,
            home,
            state,
            data,
            project,
            auth,
        }
    }

    fn command(&self, arguments: &[&str]) -> Output {
        self.command_from(&self.project, arguments)
    }

    fn command_from(&self, current_directory: &Path, arguments: &[&str]) -> Output {
        Command::new(env!("CARGO_BIN_EXE_fortlet"))
            .args(arguments)
            .current_dir(current_directory)
            .env_clear()
            .env("HOME", &self.home)
            .env("XDG_STATE_HOME", &self.state)
            .env("XDG_DATA_HOME", &self.data)
            .env("FORTLET_AUTH_FILE", &self.auth)
            .output()
            .unwrap()
    }

    fn prepare(&self, harness: &str) -> Output {
        self.command(&[
            "prepare",
            harness,
            "--project",
            self.project.to_str().unwrap(),
        ])
    }

    fn tools(&self) -> PathBuf {
        self.data.join("fortlet/tools")
    }

    fn seed_base(&self) {
        seed_marker(&self.tools().join("_base/bookworm-4"), ".fortlet-base.json");
    }

    fn seed_harness(&self, harness: &str) {
        let version = match harness {
            "codex" => "0.147.0",
            "tact" => "0.3.7",
            _ => panic!("test fixture has no version for {harness}"),
        };
        seed_marker(
            &self.tools().join(harness).join(version),
            ".fortlet-tool.json",
        );
    }

    fn assert_no_runtime_state(&self) {
        assert!(!self.state.join("fortlet/projects").exists());
        assert!(!self.data.join("fortlet/environments").exists());
    }
}

fn seed_marker(root: &Path, marker: &str) {
    fs::create_dir_all(root).unwrap();
    let (name, version) = if marker == ".fortlet-base.json" {
        ("_base", "bookworm-4")
    } else {
        let name = root
            .parent()
            .unwrap()
            .file_name()
            .unwrap()
            .to_str()
            .unwrap();
        let version = root.file_name().unwrap().to_str().unwrap();
        (name, version)
    };
    fs::write(
        root.join(marker),
        serde_json::to_vec(&serde_json::json!({
            "name": name,
            "version": version,
            "image": "node:24-bookworm",
        }))
        .unwrap(),
    )
    .unwrap();
}

fn assert_failure(output: Output, stage: &str, action: &str, cause: &str) {
    assert!(!output.status.success());
    assert!(output.stdout.is_empty());
    let stderr = String::from_utf8(output.stderr).unwrap();
    let expected = format!("fortlet: {stage} stage failed; {action}: {cause}");
    assert!(stderr.starts_with(&expected), "{stderr}");
}

#[test]
fn unknown_harness_fails_before_project_or_state_creation() {
    let fixture = Fixture::new();

    let output = fixture.prepare("unknown");

    assert_failure(
        output,
        "harness",
        "choose a registered harness: codex or tact",
        "unsupported harness \"unknown\"; expected codex or tact",
    );
    assert!(!fixture.state.join("fortlet").exists());
    assert!(!fixture.data.join("fortlet").exists());
}

#[test]
fn missing_project_fails_without_reading_credentials_or_creating_runtime_state() {
    let fixture = Fixture::new();
    let missing = fixture.project.join("missing");

    let output = fixture.command(&["prepare", "codex", "--project", missing.to_str().unwrap()]);

    assert_failure(
        output,
        "project",
        "run from a readable project directory or pass --project <path>",
        "cannot resolve project path",
    );
    fixture.assert_no_runtime_state();
    assert!(!fixture.tools().exists());
}

#[test]
fn invalid_project_environment_fails_before_layer_or_runtime_creation() {
    let fixture = Fixture::new();
    fs::create_dir(fixture.project.join(".fortlet")).unwrap();
    fs::write(
        fixture.project.join(".fortlet/environment.json"),
        r#"{"schema":1,"path":["../outside"],"environment":{}}"#,
    )
    .unwrap();
    fs::write(fixture.project.join(".fortlet/environment.sh"), "exit 99\n").unwrap();

    let output = fixture.prepare("codex");

    assert_failure(
        output,
        "project-environment",
        "fix or remove .fortlet/environment.json and retry",
        "project environment PATH entry must be a normalized relative path",
    );
    fixture.assert_no_runtime_state();
    assert!(!fixture.tools().exists());
}

#[test]
fn cache_hits_are_silent_idempotent_and_credential_free_for_both_harnesses() {
    for harness in ["codex", "tact"] {
        let fixture = Fixture::new();
        fixture.seed_base();
        fixture.seed_harness(harness);
        let before = fs::read(fixture.tools().join("_base/bookworm-4/.fortlet-base.json")).unwrap();

        for _ in 0..2 {
            let output = fixture.prepare(harness);
            assert!(output.status.success());
            assert_eq!(
                String::from_utf8(output.stdout).unwrap(),
                format!("{harness}\tready\n")
            );
            assert!(output.stderr.is_empty());
        }

        assert_eq!(
            fs::read(fixture.tools().join("_base/bookworm-4/.fortlet-base.json")).unwrap(),
            before
        );
        fixture.assert_no_runtime_state();
    }
}

#[test]
fn implicit_selection_discovers_the_project_from_a_nested_directory() {
    let fixture = Fixture::new();
    fixture.seed_base();
    fixture.seed_harness("codex");
    fs::create_dir(fixture.project.join(".jj")).unwrap();
    let nested = fixture.project.join("src/nested");
    fs::create_dir_all(&nested).unwrap();

    let output = fixture.command_from(&nested, &["prepare", "codex"]);

    assert!(output.status.success());
    assert_eq!(String::from_utf8(output.stdout).unwrap(), "codex\tready\n");
    assert!(output.stderr.is_empty());
    fixture.assert_no_runtime_state();
}

#[test]
fn incomplete_harness_layer_preserves_the_verified_base() {
    let fixture = Fixture::new();
    fixture.seed_base();
    let base_marker = fixture.tools().join("_base/bookworm-4/.fortlet-base.json");
    let before = fs::read(&base_marker).unwrap();
    fs::create_dir_all(fixture.tools().join("codex/0.147.0")).unwrap();

    let output = fixture.prepare("codex");

    assert_failure(
        output,
        "environment",
        "check network access or remove the reported incomplete layer and retry",
        "incomplete environment layer at",
    );
    assert_eq!(fs::read(base_marker).unwrap(), before);
    fixture.assert_no_runtime_state();
}
