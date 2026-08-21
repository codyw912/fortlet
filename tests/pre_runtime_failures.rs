#![cfg(unix)]

use std::fs;
use std::io::{Read, Write};
use std::net::TcpListener;
use std::os::unix::fs::symlink;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::Engine;
use sha2::{Digest, Sha256};

const VALID_FAKE_EXPIRATION: u64 = 4_102_444_800;

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
        let auth = temporary.path().join("auth.json");
        fs::create_dir_all(&home).unwrap();
        fs::create_dir_all(&project).unwrap();
        Self {
            _temporary: temporary,
            home,
            state,
            data,
            project,
            auth,
        }
    }

    fn run(&self, harness: &str, project: &Path) -> Output {
        self.run_with_auth(harness, project, &self.auth)
    }

    fn run_with_auth(&self, harness: &str, project: &Path, auth: &Path) -> Output {
        self.command(harness, project, auth).output().unwrap()
    }

    fn run_with_refresh_endpoint(&self, harness: &str, endpoint: &str) -> Output {
        self.command(harness, &self.project, &self.auth)
            .env("CODEX_REFRESH_TOKEN_URL_OVERRIDE", endpoint)
            .output()
            .unwrap()
    }

    fn run_codex_shim(&self) -> Output {
        let shim = self._temporary.path().join("codex");
        symlink(env!("CARGO_BIN_EXE_fortlet"), &shim).unwrap();

        Command::new(shim)
            .arg("--version")
            .current_dir(&self.project)
            .env_clear()
            .env("HOME", &self.home)
            .env("XDG_STATE_HOME", &self.state)
            .env("XDG_DATA_HOME", &self.data)
            .env("FORTLET_AUTH_FILE", &self.auth)
            .output()
            .unwrap()
    }

    fn command(&self, harness: &str, project: &Path, auth: &Path) -> Command {
        let mut command = Command::new(env!("CARGO_BIN_EXE_fortlet"));
        command
            .args([
                "run",
                harness,
                "--project",
                project.to_str().unwrap(),
                "--",
                "--version",
            ])
            .env_clear()
            .env("HOME", &self.home)
            .env("XDG_STATE_HOME", &self.state)
            .env("XDG_DATA_HOME", &self.data)
            .env("FORTLET_AUTH_FILE", auth);
        command
    }

    fn write_auth(&self, expiration: u64) {
        write_auth(&self.auth, expiration);
    }

    fn write_valid_auth(&self) {
        self.write_auth(VALID_FAKE_EXPIRATION);
    }

    fn write_invalid_project_environment(&self) {
        fs::create_dir(self.project.join(".fortlet")).unwrap();
        fs::write(
            self.project.join(".fortlet/environment.json"),
            r#"{"schema":1,"path":["../outside"],"environment":{}}"#,
        )
        .unwrap();
        fs::write(self.project.join(".fortlet/environment.sh"), "exit 99\n").unwrap();
    }

    fn projects(&self) -> PathBuf {
        self.state.join("fortlet/projects")
    }

    fn tools(&self) -> PathBuf {
        self.data.join("fortlet/tools")
    }

    fn environments(&self) -> PathBuf {
        self.data.join("fortlet/environments")
    }

    fn seed_layers(&self, harness: &str) {
        seed_layer_marker(
            &self.tools().join("_base/bookworm-4"),
            ".fortlet-base.json",
            "_base",
            "bookworm-4",
        );
        let version = match harness {
            "codex" => "0.147.0",
            "tact" => "0.3.7",
            _ => panic!("unknown test harness"),
        };
        seed_layer_marker(
            &self.tools().join(harness).join(version),
            ".fortlet-tool.json",
            harness,
            version,
        );
    }

    fn capsule_state(&self, harness: &str) -> PathBuf {
        let project = self.project.canonicalize().unwrap();
        let digest = Sha256::digest(project.as_os_str().as_encoded_bytes());
        self.projects()
            .join(hex::encode(&digest[..8]))
            .join(harness)
    }
}

fn seed_layer_marker(root: &Path, marker: &str, name: &str, version: &str) {
    fs::create_dir_all(root).unwrap();
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

fn write_auth(path: &Path, expiration: u64) {
    let header = URL_SAFE_NO_PAD.encode(r#"{"alg":"none","typ":"JWT"}"#);
    let claims = URL_SAFE_NO_PAD.encode(format!(r#"{{"exp":{expiration}}}"#));
    let access_token = format!("{header}.{claims}.signature");
    let id_claims = URL_SAFE_NO_PAD.encode(
        r#"{"https://api.openai.com/auth":{"chatgpt_account_id":"00000000-0000-0000-0000-000000000000"}}"#,
    );
    let id_token = format!("{header}.{id_claims}.signature");
    let document = serde_json::json!({
        "auth_mode": "chatgpt",
        "tokens": {
            "id_token": id_token,
            "access_token": access_token,
            "refresh_token": "host-refresh-token",
            "account_id": "00000000-0000-0000-0000-000000000000"
        }
    });
    fs::write(path, serde_json::to_vec(&document).unwrap()).unwrap();
}

fn rejecting_refresh_server() -> (String, std::thread::JoinHandle<()>) {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let address = listener.local_addr().unwrap();
    let server = std::thread::spawn(move || {
        let (mut stream, _) = listener.accept().unwrap();
        let mut request = [0_u8; 4096];
        let _ = stream.read(&mut request).unwrap();
        stream
            .write_all(
                b"HTTP/1.1 401 Unauthorized\r\nContent-Length: 0\r\nConnection: close\r\n\r\n",
            )
            .unwrap();
    });
    (format!("http://{address}/oauth/token"), server)
}

fn assert_failure(output: Output, stage: &str, action: &str, cause: &str) {
    assert!(!output.status.success());
    assert!(output.stdout.is_empty());
    let stderr = String::from_utf8(output.stderr).unwrap();
    let expected = format!("fortlet: {stage} stage failed; {action}: {cause}");
    assert!(stderr.starts_with(&expected), "{stderr}");
}

#[test]
fn unsupported_harness_fails_before_project_or_runtime_artifacts() {
    let fixture = Fixture::new();

    let output = fixture.run("unknown", &fixture.project);

    assert_failure(
        output,
        "harness",
        "choose a registered harness: codex or tact",
        "unsupported harness \"unknown\"; expected codex or tact",
    );
    assert!(!fixture.projects().exists());
    assert!(!fixture.tools().exists());
}

#[test]
fn missing_project_fails_before_credentials_or_runtime_artifacts() {
    let fixture = Fixture::new();
    let missing = fixture.project.join("missing");

    let output = fixture.run("codex", &missing);

    assert_failure(
        output,
        "project",
        "run from a readable project directory or pass --project <path>",
        "cannot resolve project path",
    );
    assert!(!fixture.projects().exists());
    assert!(!fixture.tools().exists());
}

#[test]
fn missing_auth_fails_before_capsule_or_environment_artifacts() {
    let fixture = Fixture::new();

    let output = fixture.run("codex", &fixture.project);

    assert_failure(
        output,
        "credentials",
        "run `codex login` on the host and retry",
        "cannot read Codex ChatGPT auth",
    );
    assert!(!fixture.projects().exists());
    assert!(!fixture.tools().exists());
}

#[test]
fn codex_shim_missing_auth_fails_before_capsule_or_environment_artifacts() {
    let fixture = Fixture::new();

    let output = fixture.run_codex_shim();

    assert_failure(
        output,
        "credentials",
        "run `codex login` on the host and retry",
        "cannot read Codex ChatGPT auth",
    );
    assert!(!fixture.projects().exists());
    assert!(!fixture.tools().exists());
}

#[test]
fn missing_refresh_token_fails_before_capsule_or_environment_artifacts() {
    let fixture = Fixture::new();
    fixture.write_valid_auth();
    let mut document: serde_json::Value =
        serde_json::from_slice(&fs::read(&fixture.auth).unwrap()).unwrap();
    document["tokens"]
        .as_object_mut()
        .unwrap()
        .remove("refresh_token");
    fs::write(&fixture.auth, serde_json::to_vec(&document).unwrap()).unwrap();

    let output = fixture.run("codex", &fixture.project);

    assert_failure(
        output,
        "credentials",
        "run `codex login` on the host or retry the bounded refresh",
        "Codex ChatGPT auth has no refresh token; run `codex login` and retry",
    );
    assert!(!fixture.projects().exists());
    assert!(!fixture.tools().exists());
}

#[test]
fn invalid_project_environment_fails_before_credentials_or_runtime_artifacts() {
    let fixture = Fixture::new();
    fixture.write_invalid_project_environment();

    let output = fixture.run("codex", &fixture.project);

    assert_failure(
        output,
        "project environment",
        "fix or remove .fortlet/environment.json and retry",
        "project environment PATH entry must be a normalized relative path",
    );
    assert!(!fixture.projects().exists());
    assert!(!fixture.tools().exists());
    assert!(!fixture.environments().exists());
}

#[test]
fn codex_shim_invalid_project_environment_fails_before_credentials_or_runtime_artifacts() {
    let fixture = Fixture::new();
    fixture.write_invalid_project_environment();

    let output = fixture.run_codex_shim();

    assert_failure(
        output,
        "project environment",
        "fix or remove .fortlet/environment.json and retry",
        "project environment PATH entry must be a normalized relative path",
    );
    assert!(!fixture.projects().exists());
    assert!(!fixture.tools().exists());
    assert!(!fixture.environments().exists());
}

#[test]
fn malformed_auth_fails_before_capsule_or_environment_artifacts() {
    let fixture = Fixture::new();
    fs::write(&fixture.auth, "not json").unwrap();

    let output = fixture.run("codex", &fixture.project);

    assert_failure(
        output,
        "credentials",
        "run `codex login` on the host or retry the bounded refresh",
        "Codex auth is invalid",
    );
    assert!(!fixture.projects().exists());
    assert!(!fixture.tools().exists());
}

#[test]
fn expired_auth_fails_before_capsule_or_environment_artifacts() {
    let fixture = Fixture::new();
    fixture.write_auth(1);
    let (endpoint, server) = rejecting_refresh_server();

    let output = fixture.run_with_refresh_endpoint("codex", &endpoint);
    server.join().unwrap();

    assert_failure(
        output,
        "credentials",
        "run `codex login` on the host or retry the bounded refresh",
        "Codex login refresh was rejected",
    );
    assert!(!fixture.projects().exists());
    assert!(!fixture.tools().exists());
}

#[test]
fn project_mounted_auth_fails_before_capsule_or_environment_artifacts() {
    let fixture = Fixture::new();
    let exposed_auth = fixture.project.join("auth.json");
    write_auth(&exposed_auth, VALID_FAKE_EXPIRATION);

    let output = fixture.run_with_auth("codex", &fixture.project, &exposed_auth);

    assert_failure(
        output,
        "credentials",
        "move the host credential file outside every guest mount and retry",
        "credential file would be exposed by guest mount",
    );
    assert!(!fixture.projects().exists());
    assert!(!fixture.tools().exists());
}

#[test]
fn invalid_capsule_state_fails_before_projection_or_environment_artifacts() {
    let fixture = Fixture::new();
    fixture.write_valid_auth();
    fixture.seed_layers("codex");
    let capsule_state = fixture.capsule_state("codex");
    fs::create_dir_all(capsule_state.parent().unwrap()).unwrap();
    fs::write(&capsule_state, "not a directory").unwrap();

    let output = fixture.run("codex", &fixture.project);

    assert_failure(
        output,
        "capsule",
        "check the reported Fortlet state path and retry",
        "cannot create harness state",
    );
    assert!(capsule_state.is_file());
    assert!(!fixture.environments().exists());
}

#[test]
fn invalid_guest_projection_fails_before_environment_or_runtime_artifacts() {
    let fixture = Fixture::new();
    fixture.write_valid_auth();
    fixture.seed_layers("codex");
    let capsule_state = fixture.capsule_state("codex");
    let projection_target = fixture.state.join("projection-target");
    fs::create_dir_all(&capsule_state).unwrap();
    fs::create_dir_all(&projection_target).unwrap();
    symlink(&projection_target, capsule_state.join(".codex")).unwrap();

    let output = fixture.run("codex", &fixture.project);

    assert_failure(
        output,
        "credentials",
        "remove the reported invalid guest projection and retry",
        "guest auth directory is not a real directory",
    );
    assert!(!projection_target.join("auth.json").exists());
    assert!(!capsule_state
        .join(".fortlet-credential-fingerprint")
        .exists());
    assert!(!fixture.environments().exists());
}

#[test]
fn incomplete_base_layer_fails_before_provisioning_or_runtime_artifacts() {
    let fixture = Fixture::new();
    fixture.write_valid_auth();
    let incomplete_base = fixture.tools().join("_base/bookworm-4");
    fs::create_dir_all(&incomplete_base).unwrap();

    let output = fixture.run("codex", &fixture.project);

    assert_failure(
        output,
        "environment",
        "check network access or remove the reported incomplete layer and retry",
        "incomplete environment layer at",
    );
    let capsule_state = fixture.capsule_state("codex");
    assert!(!capsule_state.exists());
    assert!(!capsule_state
        .join(".fortlet-credential-fingerprint")
        .exists());
    assert!(!incomplete_base.join(".fortlet-base.json").exists());
    assert!(!fixture.tools().join("codex").exists());
}

#[test]
fn incomplete_harness_layer_fails_before_provisioning_or_runtime_artifacts() {
    let fixture = Fixture::new();
    fixture.write_valid_auth();
    let base = fixture.tools().join("_base/bookworm-4");
    let incomplete_harness = fixture.tools().join("codex/0.147.0");
    fs::create_dir_all(&base).unwrap();
    fs::write(
        base.join(".fortlet-base.json"),
        r#"{"name":"_base","version":"bookworm-4","image":"node:24-bookworm"}"#,
    )
    .unwrap();
    fs::create_dir_all(&incomplete_harness).unwrap();

    let output = fixture.run("codex", &fixture.project);

    assert_failure(
        output,
        "environment",
        "check network access or remove the reported incomplete layer and retry",
        "incomplete environment layer at",
    );
    let capsule_state = fixture.capsule_state("codex");
    assert!(!capsule_state.exists());
    assert!(!capsule_state
        .join(".fortlet-credential-fingerprint")
        .exists());
    assert!(!incomplete_harness.join(".fortlet-tool.json").exists());
}
