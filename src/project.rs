use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{bail, Context, Result};
use sha2::{Digest, Sha256};

use crate::paths::AppPaths;

const MARKERS: &[(&str, &[&str])] = &[
    ("jj", &[".jj"]),
    ("git", &[".git"]),
    ("devenv", &["devenv.nix", "devenv.yaml", "devenv.yml"]),
    ("flake", &["flake.nix"]),
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectIdentity(String);

impl ProjectIdentity {
    pub fn from_local_root(root: &Path) -> Self {
        let digest = Sha256::digest(root.as_os_str().as_encoded_bytes());
        Self(hex::encode(&digest[..8]))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone)]
pub struct Project {
    pub identity: ProjectIdentity,
    pub root: PathBuf,
    pub cwd: PathBuf,
    pub kind: &'static str,
    pub scratch: bool,
}

pub fn resolve(
    paths: &AppPaths,
    explicit: Option<&Path>,
    allow_broad_mount: bool,
) -> Result<Project> {
    let mut cwd =
        canonical_directory(&env::current_dir().context("cannot read current directory")?)?;
    if let Some(root) = explicit {
        let root = canonical_directory(root)?;
        if !cwd.starts_with(&root) {
            cwd = root.clone();
        }
        return Ok(project(root, cwd, "explicit", false));
    }

    let home = env::var_os("HOME")
        .map(PathBuf::from)
        .context("HOME is not set")?;
    let home = canonical_directory(&home)?;
    if !allow_broad_mount && (cwd == home || cwd == Path::new("/")) {
        let root = paths.scratch();
        fs::create_dir_all(&root)
            .with_context(|| format!("cannot create scratch workspace {}", root.display()))?;
        let root = canonical_directory(&root)?;
        return Ok(project(root.clone(), root, "scratch", true));
    }

    for (kind, markers) in MARKERS {
        for candidate in cwd.ancestors() {
            if markers.iter().any(|marker| candidate.join(marker).exists()) {
                return Ok(project(candidate.to_path_buf(), cwd, kind, false));
            }
        }
    }
    Ok(project(cwd.clone(), cwd, "directory", false))
}

fn project(root: PathBuf, cwd: PathBuf, kind: &'static str, scratch: bool) -> Project {
    Project {
        identity: ProjectIdentity::from_local_root(&root),
        root,
        cwd,
        kind,
        scratch,
    }
}

fn canonical_directory(path: &Path) -> Result<PathBuf> {
    let resolved = path
        .canonicalize()
        .with_context(|| format!("cannot resolve project path {}", path.display()))?;
    if !resolved.is_dir() {
        bail!("project path is not a directory: {}", resolved.display());
    }
    Ok(resolved)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identity_is_stable_for_a_root() {
        let root = Path::new("/tmp/example");
        assert_eq!(
            ProjectIdentity::from_local_root(root),
            ProjectIdentity::from_local_root(root)
        );
    }
}
