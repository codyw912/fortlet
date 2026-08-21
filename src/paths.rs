use std::env;
use std::fs;
use std::path::PathBuf;

use anyhow::{bail, Context, Result};

pub const PRODUCT: &str = "fortlet";

#[derive(Debug, Clone)]
pub struct AppPaths {
    pub state: PathBuf,
    pub data: PathBuf,
}

impl AppPaths {
    pub fn from_environment() -> Result<Self> {
        let home = env::var_os("HOME")
            .map(PathBuf::from)
            .context("HOME is not set")?;
        let state_root = env::var_os("XDG_STATE_HOME")
            .map(PathBuf::from)
            .unwrap_or_else(|| home.join(".local/state"));
        let data_root = env::var_os("XDG_DATA_HOME")
            .map(PathBuf::from)
            .unwrap_or_else(|| home.join(".local/share"));
        Ok(Self {
            state: state_root.join(PRODUCT),
            data: data_root.join(PRODUCT),
        })
    }

    pub fn tools(&self) -> PathBuf {
        self.data.join("tools")
    }

    pub fn environments(&self) -> PathBuf {
        self.data.join("environments")
    }

    pub fn provider_root(&self, project_identity: &str) -> PathBuf {
        self.data.join("providers").join(project_identity)
    }

    pub fn nix_store(&self, project_identity: &str) -> PathBuf {
        self.provider_root(project_identity).join("nix")
    }

    pub fn provider_record(&self, project_identity: &str) -> PathBuf {
        self.provider_root(project_identity).join("resolved.json")
    }

    pub fn locks(&self) -> PathBuf {
        self.state.join("locks")
    }

    pub fn scratch(&self) -> PathBuf {
        self.state.join("scratch/default")
    }

    pub fn ensure_roots(&self) -> Result<()> {
        for root in [&self.state, &self.data] {
            fs::create_dir_all(root)
                .with_context(|| format!("cannot create Fortlet directory {}", root.display()))?;
            let metadata = fs::symlink_metadata(root)
                .with_context(|| format!("cannot inspect Fortlet directory {}", root.display()))?;
            if metadata.file_type().is_symlink() || !metadata.is_dir() {
                bail!(
                    "Fortlet directory is not a real directory: {}",
                    root.display()
                );
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_product_roots_before_boundary_validation() {
        let temporary = tempfile::tempdir().unwrap();
        let paths = AppPaths {
            state: temporary.path().join("state/fortlet"),
            data: temporary.path().join("data/fortlet"),
        };

        paths.ensure_roots().unwrap();

        assert!(paths.state.is_dir());
        assert!(paths.data.is_dir());
    }
}
