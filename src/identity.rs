use std::fs::{self, File};
use std::io::Write;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use anyhow::{bail, Context, Result};
use sha2::{Digest, Sha256};

use crate::paths::AppPaths;
use crate::project::Project;

pub const GIT_CONFIG_GLOBAL: &str = "GIT_CONFIG_GLOBAL";
pub const GIT_CONFIG_NOSYSTEM: &str = "GIT_CONFIG_NOSYSTEM";
pub const JJ_CONFIG: &str = "JJ_CONFIG";
pub const GUEST_IDENTITY_ROOT: &str = "/opt/fortlet/identity";

const MAX_IDENTITY_BYTES: usize = 512;

#[derive(Debug, Clone)]
pub struct IdentityProjection {
    pub identity: String,
    pub root: PathBuf,
}

pub fn prepare(paths: &AppPaths, project: &Project) -> Result<Option<IdentityProjection>> {
    let Some((name, email)) = read_effective_identity()? else {
        return Ok(None);
    };
    let identity = identity_digest(&name, &email);
    let root = paths
        .state
        .join("projects")
        .join(project.identity.as_str())
        .join("identity")
        .join(&identity);
    if projection_matches(&root, &name, &email)? {
        return Ok(Some(IdentityProjection { identity, root }));
    }
    if root.exists() {
        bail!("generated VCS identity projection is incomplete; remove it and retry");
    }
    fs::create_dir_all(&root)?;
    fs::set_permissions(&root, fs::Permissions::from_mode(0o700))?;
    write_atomic(
        &root.join("gitconfig"),
        git_config(&name, &email).as_bytes(),
    )?;
    write_atomic(
        &root.join("jjconfig.toml"),
        jj_config(&name, &email).as_bytes(),
    )?;
    Ok(Some(IdentityProjection { identity, root }))
}

fn read_effective_identity() -> Result<Option<(String, String)>> {
    let Some(home) = std::env::var_os("HOME") else {
        return Ok(None);
    };
    let home = PathBuf::from(home);
    let Some(name) = read_git_value(&home, "user.name")? else {
        return Ok(None);
    };
    let Some(email) = read_git_value(&home, "user.email")? else {
        return Ok(None);
    };
    validate_value(&name)?;
    validate_value(&email)?;
    Ok(Some((name, email)))
}

fn read_git_value(home: &Path, key: &str) -> Result<Option<String>> {
    let output = Command::new("/usr/bin/git")
        .args(["config", "--global", "--no-includes", "--get", key])
        .env_clear()
        .env("HOME", home)
        .env(GIT_CONFIG_NOSYSTEM, "1")
        .stdin(Stdio::null())
        .stderr(Stdio::null())
        .output();
    let output = match output {
        Ok(output) => output,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(error) => return Err(error).context("cannot read effective Git identity"),
    };
    if !output.status.success() {
        return Ok(None);
    }
    let value = String::from_utf8(output.stdout).context("effective Git identity is not UTF-8")?;
    Ok(Some(value.trim_end_matches(['\r', '\n']).to_owned()))
}

fn validate_value(value: &str) -> Result<()> {
    if value.is_empty()
        || value.len() > MAX_IDENTITY_BYTES
        || value
            .chars()
            .any(|character| character.is_control() || character == '\0')
    {
        bail!("effective Git identity is invalid");
    }
    Ok(())
}

fn identity_digest(name: &str, email: &str) -> String {
    let mut digest = Sha256::new();
    for value in [name.as_bytes(), email.as_bytes()] {
        digest.update((value.len() as u64).to_be_bytes());
        digest.update(value);
    }
    hex::encode(digest.finalize())
}

fn quote(value: &str) -> String {
    value.replace('\\', "\\\\").replace('"', "\\\"")
}

fn git_config(name: &str, email: &str) -> String {
    format!(
        "[user]\n\tname = \"{}\"\n\temail = \"{}\"\n[commit]\n\tgpgSign = false\n[tag]\n\tgpgSign = false\n",
        quote(name),
        quote(email)
    )
}

fn jj_config(name: &str, email: &str) -> String {
    format!(
        "[user]\nname = \"{}\"\nemail = \"{}\"\n[signing]\nbehavior = \"drop\"\n",
        quote(name),
        quote(email)
    )
}

fn projection_matches(root: &Path, name: &str, email: &str) -> Result<bool> {
    let expected = [
        ("gitconfig", git_config(name, email)),
        ("jjconfig.toml", jj_config(name, email)),
    ];
    for (file, expected) in expected {
        let path = root.join(file);
        let metadata = match fs::symlink_metadata(&path) {
            Ok(metadata) => metadata,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(false),
            Err(error) => return Err(error.into()),
        };
        if !metadata.is_file()
            || metadata.file_type().is_symlink()
            || metadata.permissions().mode() & 0o777 != 0o600
            || fs::read_to_string(path)? != expected
        {
            return Ok(false);
        }
    }
    Ok(true)
}

fn write_atomic(path: &Path, contents: &[u8]) -> Result<()> {
    let parent = path.parent().context("identity artifact has no parent")?;
    let mut temporary = tempfile::NamedTempFile::new_in(parent)?;
    temporary
        .as_file()
        .set_permissions(fs::Permissions::from_mode(0o600))?;
    temporary.write_all(contents)?;
    temporary.as_file().sync_all()?;
    temporary.persist(path).map_err(|error| error.error)?;
    let file = File::open(parent)?;
    file.sync_all()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generated_configs_contain_only_literal_identity_and_disabled_signing() {
        let git = git_config("Public Example", "example@example.invalid");
        let jj = jj_config("Public Example", "example@example.invalid");

        for value in [&git, &jj] {
            assert!(value.contains("Public Example"));
            assert!(value.contains("example@example.invalid"));
            assert!(!value.contains("helper"));
            assert!(!value.contains("ssh"));
            assert!(!value.contains("include"));
        }
        assert!(git.contains("gpgSign = false"));
        assert!(jj.contains("behavior = \"drop\""));
    }

    #[test]
    fn invalid_or_multiline_values_are_rejected() {
        for value in ["", "line\nbreak", "control\u{1b}"] {
            assert!(validate_value(value).is_err());
        }
    }
}
