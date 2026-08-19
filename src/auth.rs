use std::env;
use std::fs::{self, File, OpenOptions};
use std::io::{Read, Write};
#[cfg(unix)]
use std::os::unix::fs::{MetadataExt, OpenOptionsExt, PermissionsExt};
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use anyhow::{anyhow, bail, Context, Result};
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::Engine;
use chrono::{SecondsFormat, Utc};
use fs2::FileExt;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::{json, Map, Value};
use sha2::{Digest, Sha256};

use crate::paths::AppPaths;

pub const ACCESS_TOKEN_ENV: &str = "FORTLET_CHATGPT_ACCESS_TOKEN";
pub const ACCOUNT_ID_ENV: &str = "FORTLET_CHATGPT_ACCOUNT_ID";
pub const TOKEN_SAFETY_WINDOW: Duration = Duration::from_secs(60 * 60);

const MAX_AUTH_SIZE: u64 = 1024 * 1024;
const MAX_REFRESH_RESPONSE_SIZE: u64 = 64 * 1024;
const REFRESH_TOKEN_URL: &str = "https://auth.openai.com/oauth/token";
const REFRESH_TOKEN_URL_OVERRIDE: &str = "CODEX_REFRESH_TOKEN_URL_OVERRIDE";
const CLIENT_ID_OVERRIDE: &str = "CODEX_APP_SERVER_LOGIN_CLIENT_ID";
const CLIENT_ID: &str = "app_EMoamEEZ73f0CkXaXp7hrann";
const FAKE_ID_TOKEN_HEADER: &str = r#"{"alg":"none","typ":"JWT"}"#;
const FAKE_ID_TOKEN_CLAIMS: &str = r#"{"exp":4102444800,"https://api.openai.com/auth":{"chatgpt_account_id":"00000000-0000-0000-0000-000000000000","chatgpt_plan_type":"plus"},"sub":"fortlet-broker"}"#;

#[derive(Clone)]
pub struct Credentials {
    access_token: String,
    account_id: String,
    expires_at: u64,
    pub source: PathBuf,
}

impl std::fmt::Debug for Credentials {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("Credentials([REDACTED])")
    }
}

impl Credentials {
    pub fn read_default() -> Result<Self> {
        Self::read(&default_auth_file()?)
    }

    pub fn read(path: &Path) -> Result<Self> {
        let source = resolve_source(path)?;
        let snapshot = read_snapshot(&source, false)?;
        require_lifetime(snapshot.credentials.expires_at, TOKEN_SAFETY_WINDOW)?;
        Ok(snapshot.credentials)
    }

    pub fn expose_to_broker(&self) {
        env::set_var(ACCESS_TOKEN_ENV, &self.access_token);
        env::set_var(ACCOUNT_ID_ENV, &self.account_id);
    }

    pub fn refresh_delay(&self) -> Result<Duration> {
        let now = unix_time()?;
        Ok(Duration::from_secs(self.expires_at.saturating_sub(
            now.saturating_add(TOKEN_SAFETY_WINDOW.as_secs()),
        )))
    }
}

#[derive(Clone)]
pub struct CredentialStore {
    source: PathBuf,
    lock: PathBuf,
    endpoint: String,
    client_id: String,
    client: Client,
}

impl CredentialStore {
    pub fn from_environment(paths: &AppPaths) -> Result<Self> {
        Self::new(default_auth_file()?, paths.locks())
    }

    fn new(source: PathBuf, locks: PathBuf) -> Result<Self> {
        let endpoint = env::var(REFRESH_TOKEN_URL_OVERRIDE)
            .ok()
            .filter(|value| !value.trim().is_empty())
            .unwrap_or_else(|| REFRESH_TOKEN_URL.to_owned());
        reqwest::Url::parse(&endpoint).context("Codex refresh endpoint is invalid")?;
        let client_id = env::var(CLIENT_ID_OVERRIDE)
            .ok()
            .filter(|value| !value.trim().is_empty())
            .unwrap_or_else(|| CLIENT_ID.to_owned());
        Self::with_config(source, locks, endpoint, client_id)
    }

    fn with_config(
        source: PathBuf,
        locks: PathBuf,
        endpoint: String,
        client_id: String,
    ) -> Result<Self> {
        let source = resolve_source(&source)?;
        let mut digest = Sha256::new();
        digest.update(source.as_os_str().as_encoded_bytes());
        let lock = locks.join(format!(
            "codex-credential-{}.lock",
            hex::encode(&digest.finalize()[..16])
        ));
        let client = Client::builder()
            .connect_timeout(Duration::from_secs(5))
            .read_timeout(Duration::from_secs(10))
            .timeout(Duration::from_secs(15))
            .build()
            .context("cannot create Codex refresh client")?;
        Ok(Self {
            source,
            lock,
            endpoint,
            client_id,
            client,
        })
    }

    pub async fn renew(&self) -> Result<Credentials> {
        let lock_path = self.lock.clone();
        let _guard = tokio::task::spawn_blocking(move || lock_source(&lock_path))
            .await
            .context("Codex credential lock task failed")??;
        let snapshot = read_snapshot(&self.source, true)?;
        if require_lifetime(snapshot.credentials.expires_at, TOKEN_SAFETY_WINDOW).is_ok() {
            return Ok(snapshot.credentials);
        }

        let response = self.request_refresh(&snapshot.refresh_token).await?;
        let refreshed = validate_refresh(response, &snapshot.credentials.account_id)?;
        persist_refresh(&snapshot, &refreshed)?;
        let persisted = read_snapshot(&self.source, true)?.credentials;
        require_lifetime(persisted.expires_at, TOKEN_SAFETY_WINDOW)
            .context("refreshed Codex access token does not cover the safety window")?;
        Ok(persisted)
    }

    pub fn source(&self) -> &Path {
        &self.source
    }

    async fn request_refresh(&self, refresh_token: &str) -> Result<RefreshResponse> {
        let request = RefreshRequest {
            client_id: &self.client_id,
            grant_type: "refresh_token",
            refresh_token,
        };
        let mut response = self
            .client
            .post(&self.endpoint)
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
            .map_err(|_| anyhow!("Codex login refresh transport failed; retry"))?;
        let status = response.status();
        if response
            .content_length()
            .is_some_and(|size| size > MAX_REFRESH_RESPONSE_SIZE)
        {
            bail!("Codex login refresh response is unexpectedly large");
        }
        let mut bytes = Vec::new();
        while let Some(chunk) = response
            .chunk()
            .await
            .map_err(|_| anyhow!("Codex login refresh response failed; retry"))?
        {
            if bytes.len().saturating_add(chunk.len()) > MAX_REFRESH_RESPONSE_SIZE as usize {
                bail!("Codex login refresh response is unexpectedly large");
            }
            bytes.extend_from_slice(&chunk);
        }
        if !status.is_success() {
            if status == reqwest::StatusCode::UNAUTHORIZED || refresh_requires_login(&bytes) {
                bail!("Codex login refresh was rejected; run `codex login` and retry");
            }
            bail!("Codex login refresh failed with HTTP {status}; retry");
        }
        serde_json::from_slice(&bytes).context("Codex login refresh response is invalid")
    }
}

fn refresh_requires_login(body: &[u8]) -> bool {
    let Ok(Value::Object(object)) = serde_json::from_slice(body) else {
        return false;
    };
    let code = match object.get("error") {
        Some(Value::Object(error)) => error.get("code").and_then(Value::as_str),
        Some(Value::String(code)) => Some(code.as_str()),
        _ => object.get("code").and_then(Value::as_str),
    };
    matches!(
        code,
        Some("refresh_token_expired" | "refresh_token_reused" | "refresh_token_invalidated")
    )
}

pub fn require_outside_mounts(credentials: &Credentials, mounts: &[&Path]) -> Result<()> {
    require_source_outside_mounts(&credentials.source, mounts)
}

pub fn require_source_outside_mounts(source: &Path, mounts: &[&Path]) -> Result<()> {
    for mount in mounts {
        let mount = mount
            .canonicalize()
            .with_context(|| format!("cannot resolve guest mount {}", mount.display()))?;
        if source.starts_with(&mount) {
            bail!(
                "credential file would be exposed by guest mount: {}",
                source.display()
            );
        }
    }
    Ok(())
}

pub fn write_codex_projection(path: &Path) -> Result<()> {
    write_projection(
        path,
        "chatgptAuthTokens",
        "",
        Some(Utc::now().to_rfc3339_opts(SecondsFormat::Secs, true)),
    )
}

pub fn write_tact_projection(path: &Path) -> Result<()> {
    write_projection(path, "chatgpt", "not-a-real-refresh-token", None)
}

fn write_projection(
    path: &Path,
    auth_mode: &str,
    refresh_token: &str,
    last_refresh: Option<String>,
) -> Result<()> {
    let parent = path.parent().context("guest auth path has no parent")?;
    fs::create_dir_all(parent)
        .with_context(|| format!("cannot create guest auth directory {}", parent.display()))?;
    if fs::symlink_metadata(parent)?.file_type().is_symlink() || !parent.is_dir() {
        bail!(
            "guest auth directory is not a real directory: {}",
            parent.display()
        );
    }
    if fs::symlink_metadata(path)
        .map(|metadata| metadata.file_type().is_symlink())
        .unwrap_or(false)
    {
        bail!(
            "guest auth projection must not be a symlink: {}",
            path.display()
        );
    }
    let mut document = json!({
        "OPENAI_API_KEY": Value::Null,
        "auth_mode": auth_mode,
        "tokens": {
            "id_token": fake_id_token(),
            "access_token": format!("$MSB_{ACCESS_TOKEN_ENV}"),
            "refresh_token": refresh_token,
            "account_id": format!("$MSB_{ACCOUNT_ID_ENV}"),
        }
    });
    if let Some(last_refresh) = last_refresh {
        document["last_refresh"] = Value::String(last_refresh);
    }
    atomic_write(path, &serde_json::to_vec(&document)?)
}

#[derive(Clone, Copy)]
struct SourceIdentity {
    #[cfg(unix)]
    device: u64,
    #[cfg(unix)]
    inode: u64,
}

impl SourceIdentity {
    fn from_metadata(metadata: &fs::Metadata) -> Self {
        Self {
            #[cfg(unix)]
            device: metadata.dev(),
            #[cfg(unix)]
            inode: metadata.ino(),
        }
    }

    fn matches(self, metadata: &fs::Metadata) -> bool {
        #[cfg(unix)]
        {
            self.device == metadata.dev() && self.inode == metadata.ino()
        }
        #[cfg(not(unix))]
        {
            let _ = metadata;
            true
        }
    }
}

struct CredentialSnapshot {
    credentials: Credentials,
    refresh_token: String,
    document: Value,
    identity: SourceIdentity,
}

fn read_snapshot(source: &Path, require_refresh: bool) -> Result<CredentialSnapshot> {
    let metadata = fs::symlink_metadata(source)
        .with_context(|| format!("cannot inspect Codex auth at {}", source.display()))?;
    if metadata.file_type().is_symlink() {
        bail!("Codex auth must not be a symlink: {}", source.display());
    }
    let mut options = OpenOptions::new();
    options.read(true);
    #[cfg(unix)]
    options.custom_flags(libc::O_NOFOLLOW);
    let mut file = options
        .open(source)
        .with_context(|| format!("cannot read Codex ChatGPT auth at {}", source.display()))?;
    let metadata = file
        .metadata()
        .with_context(|| format!("cannot inspect Codex auth at {}", source.display()))?;
    if !metadata.is_file() {
        bail!("Codex auth is not a regular file: {}", source.display());
    }
    if metadata.len() > MAX_AUTH_SIZE {
        bail!("Codex auth is unexpectedly large: {}", source.display());
    }
    let mut bytes = Vec::with_capacity(metadata.len() as usize);
    (&mut file)
        .take(MAX_AUTH_SIZE + 1)
        .read_to_end(&mut bytes)?;
    if bytes.len() as u64 > MAX_AUTH_SIZE {
        bail!("Codex auth is unexpectedly large: {}", source.display());
    }
    let document: Value = serde_json::from_slice(&bytes)
        .with_context(|| format!("Codex auth is invalid: {}", source.display()))?;
    let object = document
        .as_object()
        .context("Codex auth is not a JSON object")?;
    if object.get("auth_mode").and_then(Value::as_str) != Some("chatgpt") {
        bail!("Codex is not logged in with file-backed ChatGPT authentication");
    }
    let tokens = object.get("tokens").and_then(Value::as_object);
    let access_token = token_value(tokens, object, "access_token").context(
        "Codex file-backed ChatGPT auth has no access token; keyring-backed login is unsupported",
    )?;
    let account_id = token_value(tokens, object, "account_id")
        .context("Codex ChatGPT auth has no account ID")?;
    let refresh_token = token_value(tokens, object, "refresh_token").unwrap_or_default();
    if require_refresh && refresh_token.is_empty() {
        bail!("Codex ChatGPT auth has no refresh token; run `codex login` and retry");
    }
    let expires_at = token_expiration(access_token)?;
    if let Some(bound) = token_account_id(access_token)? {
        if bound != account_id {
            bail!("Codex ChatGPT auth account binding does not match");
        }
    }
    if let Some(id_token) = token_value(tokens, object, "id_token") {
        let bound =
            token_account_id(id_token)?.context("Codex ChatGPT ID token has no account binding")?;
        if bound != account_id {
            bail!("Codex ChatGPT auth account binding does not match");
        }
    } else if require_refresh {
        bail!("Codex ChatGPT auth has no ID token; run `codex login` and retry");
    }
    Ok(CredentialSnapshot {
        credentials: Credentials {
            access_token: access_token.to_owned(),
            account_id: account_id.to_owned(),
            expires_at,
            source: source.to_owned(),
        },
        refresh_token: refresh_token.to_owned(),
        document,
        identity: SourceIdentity::from_metadata(&metadata),
    })
}

fn token_value<'a>(
    tokens: Option<&'a Map<String, Value>>,
    object: &'a Map<String, Value>,
    key: &str,
) -> Option<&'a str> {
    tokens
        .and_then(|value| value.get(key))
        .or_else(|| object.get(key))
        .and_then(Value::as_str)
        .filter(|value| !value.is_empty())
}

#[derive(Serialize)]
struct RefreshRequest<'a> {
    client_id: &'a str,
    grant_type: &'static str,
    refresh_token: &'a str,
}

#[derive(Deserialize)]
struct RefreshResponse {
    id_token: Option<String>,
    access_token: Option<String>,
    refresh_token: Option<String>,
}

struct CompleteRefresh {
    id_token: String,
    access_token: String,
    refresh_token: String,
}

fn validate_refresh(response: RefreshResponse, expected_account: &str) -> Result<CompleteRefresh> {
    let id_token = nonempty(response.id_token).context("Codex refresh returned no ID token")?;
    let access_token =
        nonempty(response.access_token).context("Codex refresh returned no access token")?;
    let refresh_token =
        nonempty(response.refresh_token).context("Codex refresh returned no refresh token")?;
    let account =
        token_account_id(&id_token)?.context("Codex refresh ID token has no account binding")?;
    if account != expected_account {
        bail!("Codex refresh changed the ChatGPT account; run `codex login` and retry");
    }
    if let Some(access_account) = token_account_id(&access_token)? {
        if access_account != expected_account {
            bail!("Codex refresh access token has a different account binding");
        }
    }
    let expires_at = token_expiration(&access_token)?;
    require_lifetime(expires_at, TOKEN_SAFETY_WINDOW)
        .context("Codex refresh returned an access token that expires too soon")?;
    Ok(CompleteRefresh {
        id_token,
        access_token,
        refresh_token,
    })
}

fn nonempty(value: Option<String>) -> Option<String> {
    value.filter(|value| !value.is_empty())
}

fn persist_refresh(snapshot: &CredentialSnapshot, refreshed: &CompleteRefresh) -> Result<()> {
    let mut document = snapshot.document.clone();
    let object = document
        .as_object_mut()
        .context("Codex auth is not a JSON object")?;
    let tokens = object
        .get_mut("tokens")
        .and_then(Value::as_object_mut)
        .context("Codex ChatGPT auth has no token object")?;
    tokens.insert("id_token".into(), Value::String(refreshed.id_token.clone()));
    tokens.insert(
        "access_token".into(),
        Value::String(refreshed.access_token.clone()),
    );
    tokens.insert(
        "refresh_token".into(),
        Value::String(refreshed.refresh_token.clone()),
    );
    object.insert(
        "last_refresh".into(),
        Value::String(Utc::now().to_rfc3339_opts(SecondsFormat::Secs, true)),
    );
    atomic_replace_source(
        &snapshot.credentials.source,
        snapshot.identity,
        &serde_json::to_vec(&document)?,
    )
}

fn atomic_replace_source(path: &Path, identity: SourceIdentity, bytes: &[u8]) -> Result<()> {
    let parent = path.parent().context("Codex auth path has no parent")?;
    let mut temporary = tempfile::Builder::new()
        .prefix(".fortlet-auth-")
        .tempfile_in(parent)
        .with_context(|| format!("cannot create temporary Codex auth in {}", parent.display()))?;
    temporary.write_all(bytes)?;
    temporary.as_file().sync_all()?;
    #[cfg(unix)]
    {
        let metadata = temporary.as_file().metadata()?;
        if metadata.permissions().mode() & 0o777 != 0o600
            || metadata.uid() != unsafe { libc::geteuid() }
        {
            bail!("temporary Codex auth has unsafe ownership or permissions");
        }
    }
    let current = fs::symlink_metadata(path)
        .with_context(|| format!("cannot recheck Codex auth at {}", path.display()))?;
    if current.file_type().is_symlink() || !identity.matches(&current) {
        bail!("Codex auth changed during refresh; retry");
    }
    temporary
        .persist(path)
        .map_err(|error| error.error)
        .with_context(|| format!("cannot replace Codex auth at {}", path.display()))?;
    File::open(parent)?.sync_all()?;
    Ok(())
}

fn atomic_write(path: &Path, bytes: &[u8]) -> Result<()> {
    let temporary = path.with_extension(format!("tmp-{}", std::process::id()));
    let mut options = OpenOptions::new();
    options.write(true).create_new(true);
    #[cfg(unix)]
    options.mode(0o600);
    let mut file = options
        .open(&temporary)
        .with_context(|| format!("cannot create {}", temporary.display()))?;
    let result = (|| -> Result<()> {
        file.write_all(bytes)?;
        file.sync_all()?;
        drop(file);
        fs::rename(&temporary, path)?;
        Ok(())
    })();
    if result.is_err() {
        let _ = fs::remove_file(&temporary);
    }
    result
}

fn lock_source(path: &Path) -> Result<File> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let mut options = OpenOptions::new();
    options.create(true).append(true);
    #[cfg(unix)]
    options.mode(0o600);
    let file = options.open(path)?;
    file.lock_exclusive()?;
    Ok(file)
}

fn resolve_source(path: &Path) -> Result<PathBuf> {
    let metadata = fs::symlink_metadata(path)
        .with_context(|| format!("cannot read Codex ChatGPT auth at {}", path.display()))?;
    if metadata.file_type().is_symlink() {
        bail!("Codex auth must not be a symlink: {}", path.display());
    }
    path.canonicalize()
        .with_context(|| format!("cannot read Codex ChatGPT auth at {}", path.display()))
}

fn fake_id_token() -> String {
    format!(
        "{}.{}.signature",
        URL_SAFE_NO_PAD.encode(FAKE_ID_TOKEN_HEADER),
        URL_SAFE_NO_PAD.encode(FAKE_ID_TOKEN_CLAIMS)
    )
}

fn default_auth_file() -> Result<PathBuf> {
    if let Some(path) = env::var_os("FORTLET_AUTH_FILE") {
        return Ok(PathBuf::from(path));
    }
    if let Some(path) = env::var_os("CODEX_HOME") {
        return Ok(PathBuf::from(path).join("auth.json"));
    }
    Ok(env::var_os("HOME")
        .map(PathBuf::from)
        .context("HOME is not set")?
        .join(".codex/auth.json"))
}

fn token_expiration(token: &str) -> Result<u64> {
    jwt_claims(token)?
        .get("exp")
        .and_then(Value::as_u64)
        .context("Codex access token has no expiration")
}

fn token_account_id(token: &str) -> Result<Option<String>> {
    let claims = jwt_claims(token)?;
    let account = claims
        .get("https://api.openai.com/auth")
        .and_then(Value::as_object)
        .and_then(|auth| auth.get("chatgpt_account_id"))
        .and_then(Value::as_str)
        .map(str::to_owned);
    Ok(account)
}

fn jwt_claims(token: &str) -> Result<Value> {
    let payload = token
        .split('.')
        .nth(1)
        .context("Codex access token is not a valid JWT")?;
    serde_json::from_slice(
        &URL_SAFE_NO_PAD
            .decode(payload)
            .context("Codex access token is not a valid JWT")?,
    )
    .context("Codex access token is not a valid JWT")
}

fn require_lifetime(expires_at: u64, window: Duration) -> Result<()> {
    if expires_at < unix_time()?.saturating_add(window.as_secs()) {
        bail!("Codex access token expires too soon");
    }
    Ok(())
}

fn unix_time() -> Result<u64> {
    Ok(SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .context("system clock is before the Unix epoch")?
        .as_secs())
}

#[cfg(test)]
mod tests {
    use std::io::{Read as _, Write as _};
    use std::net::TcpListener;
    #[cfg(unix)]
    use std::os::unix::fs::symlink;

    use super::*;

    const ACCOUNT: &str = "00000000-0000-0000-0000-000000000000";

    fn jwt(expiration: Option<u64>, account: Option<&str>) -> String {
        let header = URL_SAFE_NO_PAD.encode(FAKE_ID_TOKEN_HEADER);
        let mut claims = Map::new();
        if let Some(expiration) = expiration {
            claims.insert("exp".into(), Value::from(expiration));
        }
        if let Some(account) = account {
            claims.insert(
                "https://api.openai.com/auth".into(),
                json!({"chatgpt_account_id": account}),
            );
        }
        let claims = URL_SAFE_NO_PAD.encode(serde_json::to_vec(&claims).unwrap());
        format!("{header}.{claims}.signature")
    }

    fn auth_document(expiration: u64, refresh_token: &str) -> Value {
        json!({
            "auth_mode": "chatgpt",
            "unrelated": {"preserved": true},
            "tokens": {
                "id_token": jwt(None, Some(ACCOUNT)),
                "access_token": jwt(Some(expiration), Some(ACCOUNT)),
                "refresh_token": refresh_token,
                "account_id": ACCOUNT,
            }
        })
    }

    fn write_auth(path: &Path, expiration: u64, refresh_token: &str) {
        fs::write(
            path,
            serde_json::to_vec(&auth_document(expiration, refresh_token)).unwrap(),
        )
        .unwrap();
    }

    fn store(path: &Path, locks: &Path, endpoint: String) -> CredentialStore {
        CredentialStore::with_config(
            path.to_owned(),
            locks.to_owned(),
            endpoint,
            "test-client".into(),
        )
        .unwrap()
    }

    fn refresh_response(expiration: u64, account: &str) -> String {
        serde_json::to_string(&json!({
            "id_token": jwt(None, Some(account)),
            "access_token": jwt(Some(expiration), Some(account)),
            "refresh_token": "new-refresh",
        }))
        .unwrap()
    }

    fn serve_once(status: &str, body: String) -> (String, std::thread::JoinHandle<String>) {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let address = listener.local_addr().unwrap();
        let status = status.to_owned();
        let server = std::thread::spawn(move || {
            let (mut stream, _) = listener.accept().unwrap();
            stream
                .set_read_timeout(Some(Duration::from_secs(2)))
                .unwrap();
            let mut request = Vec::new();
            let mut chunk = [0_u8; 4096];
            loop {
                let count = stream.read(&mut chunk).unwrap();
                if count == 0 {
                    break;
                }
                request.extend_from_slice(&chunk[..count]);
                let Some(header_end) = request.windows(4).position(|part| part == b"\r\n\r\n")
                else {
                    continue;
                };
                let header = String::from_utf8_lossy(&request[..header_end]);
                let content_length = header
                    .lines()
                    .find_map(|line| {
                        let (name, value) = line.split_once(':')?;
                        name.eq_ignore_ascii_case("content-length")
                            .then(|| value.trim().parse::<usize>().ok())
                            .flatten()
                    })
                    .unwrap_or(0);
                if request.len() >= header_end + 4 + content_length {
                    break;
                }
            }
            let response = format!(
                "HTTP/1.1 {status}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
                body.len()
            );
            stream.write_all(response.as_bytes()).unwrap();
            String::from_utf8(request).unwrap()
        });
        (format!("http://{address}/oauth/token"), server)
    }

    #[test]
    fn codex_projection_contains_external_placeholders_only() {
        let temporary = tempfile::tempdir().unwrap();
        let path = temporary.path().join("state/auth.json");
        write_codex_projection(&path).unwrap();
        let document: Value = serde_json::from_slice(&fs::read(path).unwrap()).unwrap();
        let mut fixture: Value =
            serde_json::from_str(include_str!("../tests/fixtures/codex-external-auth.json"))
                .unwrap();
        let last_refresh = document["last_refresh"].as_str().unwrap();
        chrono::DateTime::parse_from_rfc3339(last_refresh).unwrap();
        fixture["last_refresh"] = document["last_refresh"].clone();
        assert_eq!(document, fixture);
        assert_eq!(document["auth_mode"], "chatgptAuthTokens");
        assert_eq!(document["tokens"]["refresh_token"], "");
        assert_eq!(
            document["tokens"]["access_token"],
            "$MSB_FORTLET_CHATGPT_ACCESS_TOKEN"
        );
    }

    #[test]
    fn tact_projection_preserves_managed_mode() {
        let temporary = tempfile::tempdir().unwrap();
        let path = temporary.path().join("state/auth.json");
        write_tact_projection(&path).unwrap();
        let document: Value = serde_json::from_slice(&fs::read(path).unwrap()).unwrap();
        assert_eq!(document["auth_mode"], "chatgpt");
        assert_eq!(
            document["tokens"]["refresh_token"],
            "not-a-real-refresh-token"
        );
    }

    #[tokio::test]
    async fn renewal_reuses_a_token_that_covers_the_safety_window() {
        let temporary = tempfile::tempdir().unwrap();
        let path = temporary.path().join("auth.json");
        write_auth(&path, unix_time().unwrap() + 7200, "old-refresh");
        let before = fs::read(&path).unwrap();

        let credentials = store(
            &path,
            &temporary.path().join("locks"),
            "http://127.0.0.1:9/oauth/token".into(),
        )
        .renew()
        .await
        .unwrap();

        assert!(credentials.refresh_delay().unwrap() >= Duration::from_secs(3500));
        assert_eq!(fs::read(path).unwrap(), before);
    }

    #[tokio::test]
    async fn renewal_is_atomic_preserves_fields_and_uses_codex_parameters() {
        let temporary = tempfile::tempdir().unwrap();
        let path = temporary.path().join("auth.json");
        write_auth(&path, unix_time().unwrap() + 30, "old-refresh");
        let (endpoint, server) = serve_once(
            "200 OK",
            refresh_response(unix_time().unwrap() + 7200, ACCOUNT),
        );

        let credentials = store(&path, &temporary.path().join("locks"), endpoint)
            .renew()
            .await
            .unwrap();
        let request = server.join().unwrap();
        let document: Value = serde_json::from_slice(&fs::read(&path).unwrap()).unwrap();

        assert!(credentials.refresh_delay().unwrap() >= Duration::from_secs(3500));
        assert_eq!(document["unrelated"]["preserved"], true);
        assert_eq!(document["tokens"]["refresh_token"], "new-refresh");
        assert!(document["last_refresh"].as_str().is_some());
        assert!(request.contains(r#""client_id":"test-client""#));
        assert!(request.contains(r#""grant_type":"refresh_token""#));
        assert!(request.contains(r#""refresh_token":"old-refresh""#));
        #[cfg(unix)]
        assert_eq!(
            fs::metadata(path).unwrap().permissions().mode() & 0o777,
            0o600
        );
    }

    #[tokio::test]
    async fn account_mismatch_leaves_the_source_unchanged() {
        let temporary = tempfile::tempdir().unwrap();
        let path = temporary.path().join("auth.json");
        write_auth(&path, unix_time().unwrap() + 30, "old-refresh");
        let before = fs::read(&path).unwrap();
        let (endpoint, server) = serve_once(
            "200 OK",
            refresh_response(unix_time().unwrap() + 7200, "different-account"),
        );

        let error = store(&path, &temporary.path().join("locks"), endpoint)
            .renew()
            .await
            .unwrap_err();
        server.join().unwrap();

        assert!(format!("{error:#}").contains("changed the ChatGPT account"));
        assert_eq!(fs::read(path).unwrap(), before);
    }

    #[tokio::test]
    async fn malformed_response_leaves_the_source_unchanged() {
        let temporary = tempfile::tempdir().unwrap();
        let path = temporary.path().join("auth.json");
        write_auth(&path, unix_time().unwrap() + 30, "old-refresh");
        let before = fs::read(&path).unwrap();
        let (endpoint, server) = serve_once("200 OK", "not-json".into());

        let error = store(&path, &temporary.path().join("locks"), endpoint)
            .renew()
            .await
            .unwrap_err();
        server.join().unwrap();

        assert!(format!("{error:#}").contains("refresh response is invalid"));
        assert_eq!(fs::read(path).unwrap(), before);
    }

    #[tokio::test]
    async fn provider_failure_body_is_not_reported() {
        let temporary = tempfile::tempdir().unwrap();
        let path = temporary.path().join("auth.json");
        write_auth(&path, unix_time().unwrap() + 30, "old-refresh");
        let (endpoint, server) = serve_once(
            "500 Internal Server Error",
            r#"{"error":"provider-body-secret"}"#.into(),
        );

        let error = store(&path, &temporary.path().join("locks"), endpoint)
            .renew()
            .await
            .unwrap_err();
        server.join().unwrap();
        let message = format!("{error:#}");

        assert!(message.contains("HTTP 500"));
        assert!(!message.contains("provider-body-secret"));
        assert!(!message.contains("old-refresh"));
    }

    #[tokio::test]
    async fn concurrent_renewals_converge_on_one_refresh_request() {
        let temporary = tempfile::tempdir().unwrap();
        let path = temporary.path().join("auth.json");
        write_auth(&path, unix_time().unwrap() + 30, "old-refresh");
        let (endpoint, server) = serve_once(
            "200 OK",
            refresh_response(unix_time().unwrap() + 7200, ACCOUNT),
        );
        let store = store(&path, &temporary.path().join("locks"), endpoint);

        let (first, second) = tokio::join!(store.renew(), store.renew());
        first.unwrap();
        second.unwrap();
        server.join().unwrap();

        let document: Value = serde_json::from_slice(&fs::read(path).unwrap()).unwrap();
        assert_eq!(document["tokens"]["refresh_token"], "new-refresh");
    }

    #[test]
    fn source_identity_change_blocks_atomic_replacement() {
        let temporary = tempfile::tempdir().unwrap();
        let path = temporary.path().join("auth.json");
        write_auth(&path, unix_time().unwrap() + 30, "old-refresh");
        let snapshot = read_snapshot(&path, true).unwrap();
        fs::remove_file(&path).unwrap();
        write_auth(&path, unix_time().unwrap() + 7200, "external-refresh");
        let external = fs::read(&path).unwrap();
        let refreshed = CompleteRefresh {
            id_token: jwt(None, Some(ACCOUNT)),
            access_token: jwt(Some(unix_time().unwrap() + 7200), Some(ACCOUNT)),
            refresh_token: "fortlet-refresh".into(),
        };

        let error = persist_refresh(&snapshot, &refreshed).unwrap_err();

        assert!(format!("{error:#}").contains("changed during refresh"));
        assert_eq!(fs::read(path).unwrap(), external);
    }

    #[cfg(unix)]
    #[test]
    fn symlinked_source_is_rejected() {
        let temporary = tempfile::tempdir().unwrap();
        let target = temporary.path().join("target.json");
        let link = temporary.path().join("auth.json");
        write_auth(&target, unix_time().unwrap() + 7200, "refresh");
        symlink(target, &link).unwrap();

        let error = Credentials::read(&link).unwrap_err();

        assert!(format!("{error:#}").contains("must not be a symlink"));
    }

    #[test]
    fn keyring_backed_login_is_rejected_actionably() {
        let temporary = tempfile::tempdir().unwrap();
        let path = temporary.path().join("auth.json");
        fs::write(&path, r#"{"auth_mode":"chatgpt"}"#).unwrap();

        let error = Credentials::read(&path).unwrap_err();

        assert!(format!("{error:#}").contains("keyring-backed login is unsupported"));
    }
}
