use std::env;
use std::fs::{self, OpenOptions};
use std::io::Write;
#[cfg(unix)]
use std::os::unix::fs::OpenOptionsExt;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use anyhow::{bail, Context, Result};
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::Engine;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};

pub const ACCESS_TOKEN_ENV: &str = "FORTLET_CHATGPT_ACCESS_TOKEN";
pub const ACCOUNT_ID_ENV: &str = "FORTLET_CHATGPT_ACCOUNT_ID";
const MINIMUM_TOKEN_LIFETIME: u64 = 60 * 60;
const MAX_AUTH_SIZE: u64 = 1024 * 1024;
const FAKE_ID_TOKEN_HEADER: &str = r#"{"alg":"none","typ":"JWT"}"#;
const FAKE_ID_TOKEN_CLAIMS: &str = r#"{"exp":4102444800,"https://api.openai.com/auth":{"chatgpt_account_id":"00000000-0000-0000-0000-000000000000","chatgpt_plan_type":"plus"},"sub":"fortlet-broker"}"#;

#[derive(Clone)]
pub struct Credentials {
    access_token: String,
    account_id: String,
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
        let source = path
            .canonicalize()
            .with_context(|| format!("cannot read Codex ChatGPT auth at {}", path.display()))?;
        let metadata = fs::metadata(&source)
            .with_context(|| format!("cannot inspect Codex auth at {}", source.display()))?;
        if !metadata.is_file() {
            bail!("Codex auth is not a regular file: {}", source.display());
        }
        if metadata.len() > MAX_AUTH_SIZE {
            bail!("Codex auth is unexpectedly large: {}", source.display());
        }
        let document: Value = serde_json::from_slice(
            &fs::read(&source)
                .with_context(|| format!("cannot read Codex auth at {}", source.display()))?,
        )
        .with_context(|| format!("Codex auth is invalid: {}", source.display()))?;
        let object = document
            .as_object()
            .context("Codex auth is not a JSON object")?;
        if object
            .get("auth_mode")
            .and_then(Value::as_str)
            .is_some_and(|mode| mode != "chatgpt")
        {
            bail!("Codex is not logged in with ChatGPT");
        }
        let tokens = object.get("tokens").and_then(Value::as_object);
        let access_token = tokens
            .and_then(|value| value.get("access_token"))
            .or_else(|| object.get("access_token"))
            .and_then(Value::as_str)
            .filter(|value| !value.is_empty())
            .context("Codex ChatGPT auth has no access token")?;
        let account_id = tokens
            .and_then(|value| value.get("account_id"))
            .or_else(|| object.get("account_id"))
            .and_then(Value::as_str)
            .filter(|value| !value.is_empty())
            .context("Codex ChatGPT auth has no account ID")?;
        validate_expiration(access_token)?;
        Ok(Self {
            access_token: access_token.into(),
            account_id: account_id.into(),
            source,
        })
    }

    pub fn expose_to_broker(&self) {
        env::set_var(ACCESS_TOKEN_ENV, &self.access_token);
        env::set_var(ACCOUNT_ID_ENV, &self.account_id);
    }

    pub fn fingerprint(&self) -> String {
        let mut digest = Sha256::new();
        digest.update(self.access_token.as_bytes());
        digest.update([0]);
        digest.update(self.account_id.as_bytes());
        hex::encode(digest.finalize())
    }
}

pub fn require_outside_mounts(credentials: &Credentials, mounts: &[&Path]) -> Result<()> {
    for mount in mounts {
        let mount = mount
            .canonicalize()
            .with_context(|| format!("cannot resolve guest mount {}", mount.display()))?;
        if credentials.source.starts_with(&mount) {
            bail!(
                "credential file would be exposed by guest mount: {}",
                credentials.source.display()
            );
        }
    }
    Ok(())
}

pub fn write_public_projection(path: &Path) -> Result<()> {
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
    let document = json!({
        "OPENAI_API_KEY": Value::Null,
        "auth_mode": "chatgpt",
        "tokens": {
            "id_token": fake_id_token(),
            "access_token": format!("$MSB_{ACCESS_TOKEN_ENV}"),
            "refresh_token": "not-a-real-refresh-token",
            "account_id": format!("$MSB_{ACCOUNT_ID_ENV}"),
        }
    });
    let temporary = path.with_extension(format!("tmp-{}", std::process::id()));
    let mut options = OpenOptions::new();
    options.write(true).create_new(true);
    #[cfg(unix)]
    options.mode(0o600);
    let mut file = options
        .open(&temporary)
        .with_context(|| format!("cannot create {}", temporary.display()))?;
    let result = (|| -> Result<()> {
        file.write_all(&serde_json::to_vec(&document)?)?;
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

fn validate_expiration(token: &str) -> Result<()> {
    let payload = token
        .split('.')
        .nth(1)
        .context("Codex access token is not a valid JWT")?;
    let claims: Value = serde_json::from_slice(
        &URL_SAFE_NO_PAD
            .decode(payload)
            .context("Codex access token is not a valid JWT")?,
    )
    .context("Codex access token is not a valid JWT")?;
    let expiration = claims
        .get("exp")
        .and_then(Value::as_u64)
        .context("Codex access token has no expiration")?;
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .context("system clock is before the Unix epoch")?
        .as_secs();
    if expiration < now + MINIMUM_TOKEN_LIFETIME {
        bail!("Codex access token expires too soon; refresh Codex login");
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn projection_contains_placeholders_only() {
        let temporary = tempfile::tempdir().unwrap();
        let path = temporary.path().join("state/auth.json");
        write_public_projection(&path).unwrap();
        let content = fs::read_to_string(path).unwrap();
        assert!(content.contains("$MSB_FORTLET_CHATGPT_ACCESS_TOKEN"));
        assert!(!content.contains("real-access-token"));
    }
}
