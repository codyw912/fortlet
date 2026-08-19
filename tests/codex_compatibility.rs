#![cfg(unix)]

use std::fs;
use std::io::{Read, Write};
use std::net::TcpListener;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

const ACCOUNT_PLACEHOLDER: &str = "$MSB_FORTLET_CHATGPT_ACCOUNT_ID";
const ACCESS_TOKEN_PLACEHOLDER: &str = "$MSB_FORTLET_CHATGPT_ACCESS_TOKEN";

#[test]
#[ignore = "requires FORTLET_CODEX_COMPAT_BINARY pointing to stock Codex 0.147.0"]
fn stock_codex_disable_is_authoritative_and_preserves_configured_mcp() {
    let binary = PathBuf::from(
        std::env::var_os("FORTLET_CODEX_COMPAT_BINARY")
            .expect("set FORTLET_CODEX_COMPAT_BINARY to stock Codex 0.147.0"),
    );
    assert_stock_version(&binary);

    let temporary = tempfile::tempdir().unwrap();
    let codex_home = temporary.path().join("codex-home");
    fs::create_dir(&codex_home).unwrap();
    fs::write(
        codex_home.join("config.toml"),
        r#"[features]
apps = true

[mcp_servers.sample]
command = "/usr/bin/true"
"#,
    )
    .unwrap();

    let features = Command::new(&binary)
        .args([
            "--disable",
            "apps",
            "--enable",
            "apps",
            "-c",
            "features.apps=true",
            "features",
            "list",
        ])
        .env_clear()
        .env("HOME", temporary.path())
        .env("CODEX_HOME", &codex_home)
        .env("PATH", "/usr/bin:/bin")
        .output()
        .unwrap();
    assert!(
        features.status.success(),
        "stock Codex feature query failed: {}",
        String::from_utf8_lossy(&features.stderr)
    );
    let features = String::from_utf8(features.stdout).unwrap();
    let apps = features
        .lines()
        .find(|line| line.starts_with("apps "))
        .expect("stock Codex did not report the apps feature");
    assert!(
        apps.ends_with("false"),
        "Apps disable was not authoritative: {apps}"
    );

    let servers = Command::new(&binary)
        .args(["--disable", "apps", "mcp", "list", "--json"])
        .env_clear()
        .env("HOME", temporary.path())
        .env("CODEX_HOME", &codex_home)
        .env("PATH", "/usr/bin:/bin")
        .output()
        .unwrap();
    assert!(
        servers.status.success(),
        "stock Codex MCP query failed: {}",
        String::from_utf8_lossy(&servers.stderr)
    );
    let servers: serde_json::Value = serde_json::from_slice(&servers.stdout).unwrap();
    assert_eq!(servers.as_array().unwrap().len(), 1);
    assert_eq!(servers[0]["name"], "sample");
    assert_eq!(servers[0]["enabled"], true);
}

#[test]
#[ignore = "requires FORTLET_CODEX_COMPAT_BINARY pointing to stock Codex 0.147.0"]
fn stock_codex_loads_external_tokens_without_oauth_refresh() {
    let binary = PathBuf::from(
        std::env::var_os("FORTLET_CODEX_COMPAT_BINARY")
            .expect("set FORTLET_CODEX_COMPAT_BINARY to stock Codex 0.147.0"),
    );
    assert_stock_version(&binary);

    let temporary = tempfile::tempdir().unwrap();
    let codex_home = temporary.path().join("codex-home");
    let project = temporary.path().join("project");
    fs::create_dir(&codex_home).unwrap();
    fs::create_dir(&project).unwrap();
    let auth = codex_home.join("auth.json");
    fs::copy(
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/codex-external-auth.json"),
        &auth,
    )
    .unwrap();
    fs::set_permissions(&auth, fs::Permissions::from_mode(0o600)).unwrap();

    let (model_url, model_request) = serve_model_request();
    let (refresh_url, refresh_seen, refresh_done, refresh_server) = serve_refresh_tripwire();
    fs::write(
        codex_home.join("config.toml"),
        format!(
            r#"model = "gpt-5.3-codex"
model_provider = "fortlet-fixture"
approval_policy = "never"
sandbox_mode = "read-only"
disable_response_storage = true

[features]
plugins = false

[model_providers.fortlet-fixture]
name = "Fortlet compatibility fixture"
base_url = "{model_url}/v1"
wire_api = "responses"
requires_openai_auth = true
request_max_retries = 0
stream_max_retries = 0
"#,
        ),
    )
    .unwrap();

    let mut child = Command::new(&binary)
        .args([
            "exec",
            "--json",
            "--ephemeral",
            "--ignore-rules",
            "--skip-git-repo-check",
            "-C",
            project.to_str().unwrap(),
            "Return exactly compatibility-ok without using tools.",
        ])
        .env_clear()
        .env("HOME", temporary.path())
        .env("CODEX_HOME", &codex_home)
        .env("CODEX_REFRESH_TOKEN_URL_OVERRIDE", &refresh_url)
        .env("PATH", "/usr/bin:/bin")
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    let deadline = Instant::now() + Duration::from_secs(15);
    loop {
        if child.try_wait().unwrap().is_some() {
            break;
        }
        if Instant::now() >= deadline {
            child.kill().unwrap();
            panic!("stock Codex did not finish the bounded compatibility request");
        }
        std::thread::sleep(Duration::from_millis(20));
    }
    let output = child.wait_with_output().unwrap();
    let request = model_request.join().unwrap();
    refresh_done.store(true, Ordering::SeqCst);
    refresh_server.join().unwrap();

    assert!(
        request.contains(&format!("chatgpt-account-id: {ACCOUNT_PLACEHOLDER}")),
        "stock Codex did not send the external account placeholder: {request}"
    );
    assert!(
        request.contains(&format!("authorization: Bearer {ACCESS_TOKEN_PLACEHOLDER}")),
        "stock Codex did not send the external access-token placeholder: {request}"
    );
    assert!(
        !refresh_seen.load(Ordering::SeqCst),
        "stock Codex attempted the OAuth refresh endpoint"
    );
    assert!(
        !output.status.success(),
        "the fake backend deliberately returns an error"
    );
}

fn assert_stock_version(binary: &Path) {
    let version = Command::new(binary)
        .args(["--disable", "apps", "--version"])
        .output()
        .unwrap();
    assert!(version.status.success());
    assert_eq!(
        String::from_utf8(version.stdout).unwrap().trim(),
        "codex-cli 0.147.0"
    );
}

fn serve_model_request() -> (String, std::thread::JoinHandle<String>) {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let address = listener.local_addr().unwrap();
    let server = std::thread::spawn(move || {
        let (mut stream, _) = listener.accept().unwrap();
        stream
            .set_read_timeout(Some(Duration::from_secs(5)))
            .unwrap();
        let mut request = [0_u8; 64 * 1024];
        let count = stream.read(&mut request).unwrap();
        let body = r#"{"error":{"message":"fortlet compatibility stop"}}"#;
        let response = format!(
            "HTTP/1.1 500 Internal Server Error\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
            body.len()
        );
        stream.write_all(response.as_bytes()).unwrap();
        String::from_utf8_lossy(&request[..count]).into_owned()
    });
    (format!("http://{address}"), server)
}

fn serve_refresh_tripwire() -> (
    String,
    Arc<AtomicBool>,
    Arc<AtomicBool>,
    std::thread::JoinHandle<()>,
) {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    listener.set_nonblocking(true).unwrap();
    let address = listener.local_addr().unwrap();
    let seen = Arc::new(AtomicBool::new(false));
    let server_seen = seen.clone();
    let done = Arc::new(AtomicBool::new(false));
    let server_done = done.clone();
    let server = std::thread::spawn(move || {
        let deadline = Instant::now() + Duration::from_secs(15);
        while Instant::now() < deadline && !server_done.load(Ordering::SeqCst) {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    server_seen.store(true, Ordering::SeqCst);
                    stream
                        .write_all(
                            b"HTTP/1.1 500 Internal Server Error\r\nContent-Length: 0\r\nConnection: close\r\n\r\n",
                        )
                        .unwrap();
                    return;
                }
                Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => {
                    std::thread::sleep(Duration::from_millis(20));
                }
                Err(error) => panic!("refresh tripwire failed: {error}"),
            }
        }
    });
    (format!("http://{address}/oauth/token"), seen, done, server)
}
