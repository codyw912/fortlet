use std::fs::{self, File};
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use anyhow::{bail, Context, Result};
use fs2::FileExt;
use microsandbox::Sandbox;

use crate::harness::Harness;
use crate::paths::AppPaths;

pub const BASE_IMAGE: &str = "node:24-bookworm";
const BASE_TOOLS_VERSION: &str = "bookworm-1";
const BUBBLEWRAP_VERSION: &str = "0.8.0-2+deb12u1";
const HOLD_SCRIPT: &str = "trap 'exit 0' TERM INT; while :; do sleep 3600 & wait $!; done";

pub struct EnvironmentStore<'a> {
    paths: &'a AppPaths,
}

pub struct EnvironmentLayers {
    pub harness: PathBuf,
    pub base: PathBuf,
}

impl<'a> EnvironmentStore<'a> {
    pub fn new(paths: &'a AppPaths) -> Self {
        Self { paths }
    }

    pub async fn ensure(&self, harness: &dyn Harness) -> Result<EnvironmentLayers> {
        let base_script = format!(
            r#"set -eu
temporary="$(mktemp -d)"
trap 'rm -rf "$temporary"' EXIT
chmod 777 "$temporary"
cd "$temporary"
apt-get update -qq
apt-get download "bubblewrap={BUBBLEWRAP_VERSION}"
set -- bubblewrap_*.deb
test "$#" -eq 1
dpkg-deb -x "$1" /out
/out/usr/bin/bwrap --version
"#
        );
        let base = self
            .ensure_layer(
                "_base",
                BASE_TOOLS_VERSION,
                ".fortlet-base.json",
                &base_script,
            )
            .await?;
        let harness_path = self
            .ensure_layer(
                harness.name(),
                harness.version(),
                ".fortlet-tool.json",
                &harness.provision_script(),
            )
            .await?;
        Ok(EnvironmentLayers {
            harness: harness_path,
            base,
        })
    }

    async fn ensure_layer(
        &self,
        name: &str,
        version: &str,
        marker: &str,
        script: &str,
    ) -> Result<PathBuf> {
        let destination = self.paths.tools().join(name).join(version);
        if destination.join(marker).is_file() {
            return Ok(destination);
        }
        let _lock = lock(&self.paths.locks().join(format!("layer-{name}.lock")))?;
        if destination.join(marker).is_file() {
            return Ok(destination);
        }
        if destination.exists() {
            bail!(
                "incomplete environment layer at {}; remove it and retry",
                destination.display()
            );
        }
        fs::create_dir_all(self.paths.tools())?;
        let temporary = tempfile::Builder::new()
            .prefix(&format!(".{name}-{version}-"))
            .tempdir_in(self.paths.tools())?;
        let unique = SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos();
        let sandbox_name = format!("fortlet-provision-{}-{unique:x}", std::process::id());
        eprintln!("fortlet: preparing {name} {version} (first use)");
        let sandbox = Sandbox::builder(&sandbox_name)
            .image(BASE_IMAGE)
            .cpus(2)
            .memory(2048)
            .volume("/out", |mount| mount.bind(temporary.path()))
            .script("hold", HOLD_SCRIPT)
            .entrypoint(["hold"])
            .create()
            .await
            .with_context(|| format!("cannot create provisioning capsule {sandbox_name}"))?;
        let provision = sandbox
            .exec("/bin/sh", ["-c", script])
            .await
            .context("environment provisioning failed");
        cleanup_provisioning_capsule(&sandbox, &sandbox_name).await;
        let output = provision?;
        if !output.status().success {
            let stderr = output.stderr().unwrap_or_default();
            bail!(
                "environment provisioning exited {}: {stderr}",
                output.status().code
            );
        }
        fs::write(
            temporary.path().join(marker),
            format!(
                "{}\n",
                serde_json::json!({
                    "name": name,
                    "version": version,
                    "image": BASE_IMAGE,
                })
            ),
        )?;
        fs::create_dir_all(destination.parent().context("layer has no parent")?)?;
        let persisted = temporary.keep();
        fs::rename(&persisted, &destination).with_context(|| {
            format!(
                "cannot publish environment layer {} to {}",
                persisted.display(),
                destination.display()
            )
        })?;
        Ok(destination)
    }
}

async fn cleanup_provisioning_capsule(sandbox: &Sandbox, name: &str) {
    let _ = sandbox.stop_and_wait().await;
    let _ = Sandbox::remove(name).await;
}

fn lock(path: &Path) -> Result<File> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let file = File::options().create(true).append(true).open(path)?;
    file.lock_exclusive()?;
    Ok(file)
}
