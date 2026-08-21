use std::collections::BTreeMap;
use std::env;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use anyhow::{bail, Context, Result};
use fs2::FileExt;
use microsandbox::sandbox::PullPolicy;
use microsandbox::{Image, MicrosandboxError, Sandbox};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::harness::Harness;
use crate::paths::AppPaths;
use crate::prepare_timing::{PreparePhase, PrepareTimings};
use crate::project::Project;
use crate::project_environment::{guest_system, NIX_VERSION};
use crate::runtime::{effective_gid, effective_uid};

const ARTIFACTS_ENV: &str = "FORTLET_RUNTIME_ARTIFACTS";
pub const RUNTIME_CONTRACT: &str = "fip0013-1";
const MAX_MANIFEST_BYTES: u64 = 1024 * 1024;
const MAX_STORE_PATHS: usize = 1024;
const PROJECT_STORE_MIB: u32 = 16 * 1024;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RuntimeManifest {
    schema: u64,
    contract: String,
    system: String,
    architecture: String,
    image_reference: String,
    image_digest: String,
    image_archive_sha256: String,
    seed_archive_sha256: String,
    runtime_root: String,
    nix_version: String,
    nix: String,
    nix_store: String,
    shell: String,
    hold: String,
    managed_bash_env: String,
    runtime_library_path: String,
    ca_bundle: String,
    store_paths: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct HarnessManifest {
    schema: u64,
    contract: String,
    name: String,
    version: String,
    system: String,
    executable: String,
    archive_sha256: String,
    store_paths: Vec<String>,
}

#[derive(Clone, Debug)]
pub struct PreparedRuntime {
    pub store_volume: String,
    pub image_reference: String,
    pub runtime_root: String,
    pub nix_version: String,
    pub nix: String,
    pub shell: String,
    pub hold: String,
    pub managed_bash_env: String,
    pub runtime_library_path: String,
    pub harness_executable: String,
    pub runtime_identity: String,
    pub harness_identity: String,
}

pub struct RuntimeArtifacts {
    root: PathBuf,
    runtime: RuntimeManifest,
    harnesses: BTreeMap<String, HarnessManifest>,
}

impl RuntimeArtifacts {
    pub fn from_environment() -> Result<Self> {
        let root = env::var_os(ARTIFACTS_ENV)
            .map(PathBuf::from)
            .context("Fortlet runtime artifacts are not installed")?;
        Self::load(root)
    }

    fn load(root: PathBuf) -> Result<Self> {
        require_real_directory(&root, "runtime artifact root")?;
        let runtime: RuntimeManifest = read_manifest(&root.join("runtime/manifest.json"))?;
        runtime.validate()?;
        let mut harnesses = BTreeMap::new();
        for name in ["codex", "tact"] {
            let manifest: HarnessManifest =
                read_manifest(&root.join("harnesses").join(name).join("manifest.json"))?;
            manifest.validate(name)?;
            harnesses.insert(name.to_owned(), manifest);
        }
        Ok(Self {
            root,
            runtime,
            harnesses,
        })
    }

    pub async fn ensure(
        &self,
        paths: &AppPaths,
        project: &Project,
        harness: &dyn Harness,
        timings: &mut PrepareTimings,
    ) -> Result<PreparedRuntime> {
        let harness_manifest = self.harness(harness)?;
        let destination = paths.project_store_root(project.identity.as_str());
        if self.prepared(&destination, harness_manifest)? {
            return Ok(self.prepared_runtime(project, harness_manifest));
        }

        let _lock = lock(
            &paths
                .locks()
                .join(format!("project-store-{}.lock", project.identity.as_str())),
        )?;
        if self.prepared(&destination, harness_manifest)? {
            return Ok(self.prepared_runtime(project, harness_manifest));
        }
        if !destination.exists() {
            self.verify_runtime_archives()?;
            self.ensure_image().await?;
            timings.record(PreparePhase::Image);
            self.seed(paths, project).await?;
            timings.record(PreparePhase::RuntimeStore);
        }
        self.verify_runtime_store(&destination)?;
        if !self.harness_prepared(&destination, harness_manifest)? {
            self.import_harness(project, &destination, harness_manifest)
                .await?;
            timings.record(PreparePhase::HarnessClosure);
        }
        self.prepared(&destination, harness_manifest)?
            .then(|| self.prepared_runtime(project, harness_manifest))
            .context("prepared project runtime did not verify after publication")
    }

    pub fn verify_prepared(
        &self,
        paths: &AppPaths,
        project: &Project,
        harness: &dyn Harness,
    ) -> Result<()> {
        let harness_manifest = self.harness(harness)?;
        self.prepared(
            &paths.project_store_root(project.identity.as_str()),
            harness_manifest,
        )?
        .then_some(())
        .context("prepared project runtime did not verify")
    }

    fn harness(&self, harness: &dyn Harness) -> Result<&HarnessManifest> {
        let manifest = self
            .harnesses
            .get(harness.name())
            .context("selected harness closure is not installed")?;
        if manifest.version != harness.version() {
            bail!("selected harness closure version does not match its adapter");
        }
        Ok(manifest)
    }

    fn prepared(&self, root: &Path, harness: &HarnessManifest) -> Result<bool> {
        Ok(self.verify_runtime_store_if_present(root)? && self.harness_prepared(root, harness)?)
    }

    fn verify_runtime_store_if_present(&self, root: &Path) -> Result<bool> {
        let marker = root.join("runtime.json");
        match fs::symlink_metadata(&marker) {
            Ok(_) => {
                self.verify_runtime_store(root)?;
                Ok(true)
            }
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                if root.exists() {
                    bail!("incomplete project runtime store; remove it and retry");
                }
                Ok(false)
            }
            Err(error) => Err(error.into()),
        }
    }

    fn verify_runtime_store(&self, root: &Path) -> Result<()> {
        require_real_directory(root, "project runtime store")?;
        let marker: RuntimeManifest = read_manifest(&root.join("runtime.json"))?;
        if marker != self.runtime {
            bail!("project runtime store does not match installed runtime artifacts");
        }
        Ok(())
    }

    fn harness_prepared(&self, root: &Path, manifest: &HarnessManifest) -> Result<bool> {
        let marker = root
            .join("harnesses")
            .join(format!("{}.json", manifest.name));
        let metadata = match fs::symlink_metadata(&marker) {
            Ok(metadata) => metadata,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(false),
            Err(error) => return Err(error.into()),
        };
        if !metadata.is_file() || metadata.file_type().is_symlink() {
            bail!("prepared harness closure marker is not a regular file");
        }
        let actual: HarnessManifest = read_manifest(&marker)?;
        if actual != *manifest {
            bail!("prepared harness closure does not match installed artifacts");
        }
        Ok(true)
    }

    async fn seed(&self, paths: &AppPaths, project: &Project) -> Result<()> {
        let stores = paths.project_stores();
        fs::create_dir_all(&stores)?;
        let temporary = tempfile::Builder::new()
            .prefix(".project-store-")
            .tempdir_in(&stores)?;
        let artifact_stage = tempfile::Builder::new()
            .prefix(".runtime-artifact-")
            .tempdir_in(&stores)?;
        let staged_archive = artifact_stage.path().join("runtime.nar.gz");
        fs::copy(self.root.join("runtime/runtime.nar.gz"), &staged_archive)?;
        verify_sha256(&staged_archive, &self.runtime.seed_archive_sha256)?;

        eprintln!("fortlet: preparing runtime store (first use)");
        let unique = SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos();
        let sandbox_name = format!("fortlet-runtime-seed-{}-{unique:x}", std::process::id());
        let volume_name = project_store_volume(project.identity.as_str());
        let project_identity = project.identity.as_str().to_owned();
        let sandbox = Sandbox::builder(&sandbox_name)
            .image(self.runtime.image_reference.clone())
            .pull_policy(PullPolicy::Never)
            .cpus(2)
            .memory(2048)
            .volume("/seed/nix", |mount| {
                mount.named_with(volume_name.clone(), |volume| {
                    volume
                        .disk()
                        .size(PROJECT_STORE_MIB)
                        .ensure_exists()
                        .label("fortlet.managed", "true")
                        .label("fortlet.contract", RUNTIME_CONTRACT)
                        .label("fortlet.project", project_identity)
                })
            })
            .volume("/artifacts/runtime", |mount| {
                mount.bind(artifact_stage.path()).readonly()
            })
            .entrypoint([self.runtime.hold.clone()])
            .create()
            .await
            .context("cannot create runtime seed capsule")?;
        let declared_paths = self
            .runtime
            .store_paths
            .iter()
            .map(|path| shell_quote(path))
            .collect::<Vec<_>>()
            .join(" ");
        let script = format!(
            "set -eu\n{runtime_root}/bin/gzip -dc /artifacts/runtime/runtime.nar.gz | {nix_store} --store 'local?root=/seed' --import >/dev/null\nfor path in {declared_paths}; do test -e \"/seed$path\"; done\n{runtime_root}/bin/chown -R {uid}:{gid} /seed/nix\n",
            runtime_root = self.runtime.runtime_root,
            nix_store = self.runtime.nix_store,
            uid = effective_uid(),
            gid = effective_gid(),
        );
        let outcome = sandbox
            .exec(&self.runtime.shell, ["-c", script.as_str()])
            .await
            .context("runtime seed command could not start");
        cleanup(&sandbox, &sandbox_name).await;
        let output = outcome?;
        if !output.status().success {
            bail!(
                "runtime seed command exited {}: {}",
                output.status().code,
                bounded_diagnostic(&output.stderr().unwrap_or_default())
            );
        }
        write_manifest(&temporary.path().join("runtime.json"), &self.runtime)?;

        let destination = paths.project_store_root(project.identity.as_str());
        if destination.exists() {
            bail!("project runtime store appeared during seed publication");
        }
        let persisted = temporary.keep();
        fs::rename(&persisted, &destination)
            .context("cannot publish the prepared project runtime store")?;
        self.verify_runtime_store(&destination)
    }

    async fn import_harness(
        &self,
        project: &Project,
        root: &Path,
        manifest: &HarnessManifest,
    ) -> Result<()> {
        let bundle = self.root.join("harnesses").join(&manifest.name);
        verify_sha256(&bundle.join("closure.nar.gz"), &manifest.archive_sha256)?;
        let staging_root = root.parent().context("project store has no parent")?;
        let artifact_stage = tempfile::Builder::new()
            .prefix(".harness-artifact-")
            .tempdir_in(staging_root)?;
        let staged_archive = artifact_stage.path().join("closure.nar.gz");
        fs::copy(bundle.join("closure.nar.gz"), &staged_archive)?;
        verify_sha256(&staged_archive, &manifest.archive_sha256)?;
        eprintln!(
            "fortlet: preparing {} environment (first use)",
            manifest.name
        );
        let unique = SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos();
        let sandbox_name = format!(
            "fortlet-harness-{}-{}-{unique:x}",
            manifest.name,
            std::process::id()
        );
        let volume_name = project_store_volume(project.identity.as_str());
        let sandbox = Sandbox::builder(&sandbox_name)
            .image(self.runtime.image_reference.clone())
            .pull_policy(PullPolicy::Never)
            .cpus(2)
            .memory(2048)
            .volume("/seed/nix", |mount| mount.named(volume_name))
            .volume("/artifacts/harness", |mount| {
                mount.bind(artifact_stage.path()).readonly()
            })
            .entrypoint([self.runtime.hold.clone()])
            .create()
            .await
            .context("cannot create harness closure capsule")?;
        let declared_paths = manifest
            .store_paths
            .iter()
            .map(|path| shell_quote(path))
            .collect::<Vec<_>>()
            .join(" ");
        let script = format!(
            "set -eu\n{runtime_root}/bin/gzip -dc /artifacts/harness/closure.nar.gz | {nix_store} --store 'local?root=/seed' --import >/dev/null\nfor path in {declared_paths}; do test -e \"/seed$path\"; done\n{runtime_root}/bin/chroot /seed {executable} --version >/dev/null\n",
            runtime_root = self.runtime.runtime_root,
            nix_store = self.runtime.nix_store,
            executable = manifest.executable,
        );
        let outcome = sandbox
            .exec(&self.runtime.shell, ["-c", script.as_str()])
            .await
            .context("harness closure import could not start");
        cleanup(&sandbox, &sandbox_name).await;
        let output = outcome?;
        if !output.status().success {
            bail!(
                "harness closure import exited {}: {}",
                output.status().code,
                bounded_diagnostic(&output.stderr().unwrap_or_default())
            );
        }
        let marker = root
            .join("harnesses")
            .join(format!("{}.json", manifest.name));
        write_manifest(&marker, manifest)?;
        Ok(())
    }

    fn verify_runtime_archives(&self) -> Result<()> {
        verify_sha256(
            &self.root.join("runtime/image.oci.tar"),
            &self.runtime.image_archive_sha256,
        )?;
        verify_sha256(
            &self.root.join("runtime/runtime.nar.gz"),
            &self.runtime.seed_archive_sha256,
        )
    }

    async fn ensure_image(&self) -> Result<()> {
        match Image::get(&self.runtime.image_reference).await {
            Ok(image) => return self.verify_image(&image),
            Err(MicrosandboxError::ImageNotFound(_)) => {}
            Err(error) => return Err(error).context("cannot inspect the local runtime image"),
        }
        Image::load(
            &self.root.join("runtime/image.oci.tar"),
            vec![self.runtime.image_reference.clone()],
        )
        .await
        .context("cannot load the local runtime image archive")?;
        let image = Image::get(&self.runtime.image_reference)
            .await
            .context("loaded runtime image is not indexed")?;
        self.verify_image(&image)
    }

    fn verify_image(&self, image: &microsandbox::ImageHandle) -> Result<()> {
        if image.manifest_digest() != Some(self.runtime.image_digest.as_str())
            || image.architecture() != Some(self.runtime.architecture.as_str())
            || image.os() != Some("linux")
        {
            bail!("cached runtime image does not match its installed manifest");
        }
        Ok(())
    }

    fn prepared_runtime(&self, project: &Project, harness: &HarnessManifest) -> PreparedRuntime {
        PreparedRuntime {
            store_volume: project_store_volume(project.identity.as_str()),
            image_reference: self.runtime.image_reference.clone(),
            runtime_root: self.runtime.runtime_root.clone(),
            nix_version: self.runtime.nix_version.clone(),
            nix: self.runtime.nix.clone(),
            shell: self.runtime.shell.clone(),
            hold: self.runtime.hold.clone(),
            managed_bash_env: self.runtime.managed_bash_env.clone(),
            runtime_library_path: self.runtime.runtime_library_path.clone(),
            harness_executable: harness.executable.clone(),
            runtime_identity: format!(
                "{}:{}",
                self.runtime.image_digest, self.runtime.seed_archive_sha256
            ),
            harness_identity: harness.archive_sha256.clone(),
        }
    }
}

impl RuntimeManifest {
    fn validate(&self) -> Result<()> {
        if self.schema != 1
            || self.contract != RUNTIME_CONTRACT
            || self.system != guest_system()?
            || self.nix_version != NIX_VERSION
        {
            bail!("runtime artifact manifest does not match this Fortlet build");
        }
        if self.architecture != expected_architecture()? {
            bail!("runtime artifact architecture does not match this Fortlet build");
        }
        if self.image_reference.is_empty()
            || !valid_sha256(&self.image_archive_sha256)
            || !valid_sha256(&self.seed_archive_sha256)
            || !self.image_digest.starts_with("sha256:")
        {
            bail!("runtime artifact manifest contains an invalid identity");
        }
        validate_store_paths(&self.store_paths)?;
        for path in [
            &self.runtime_root,
            &self.nix,
            &self.nix_store,
            &self.shell,
            &self.hold,
            &self.managed_bash_env,
            &self.ca_bundle,
        ] {
            validate_store_path(path)?;
            if !self
                .store_paths
                .iter()
                .any(|root| Path::new(path).starts_with(root))
            {
                bail!("runtime artifact path is outside its declared closure");
            }
        }
        for path in self.runtime_library_path.split(':') {
            validate_store_path(path)?;
            if !self
                .store_paths
                .iter()
                .any(|root| Path::new(path).starts_with(root))
            {
                bail!("runtime library path is outside its declared closure");
            }
        }
        Ok(())
    }
}

impl HarnessManifest {
    fn validate(&self, expected_name: &str) -> Result<()> {
        if self.schema != 1
            || self.contract != RUNTIME_CONTRACT
            || self.system != guest_system()?
            || self.name != expected_name
            || self.version.is_empty()
            || !valid_sha256(&self.archive_sha256)
        {
            bail!("harness artifact manifest does not match this Fortlet build");
        }
        validate_store_paths(&self.store_paths)?;
        validate_store_path(&self.executable)?;
        if !self
            .store_paths
            .iter()
            .any(|root| Path::new(&self.executable).starts_with(root))
        {
            bail!("harness executable is outside its declared closure");
        }
        Ok(())
    }
}

fn project_store_volume(project_identity: &str) -> String {
    format!(
        "fortlet-store-{}-{RUNTIME_CONTRACT}-{project_identity}",
        effective_uid()
    )
}

fn expected_architecture() -> Result<&'static str> {
    match guest_system()? {
        "aarch64-linux" => Ok("arm64"),
        "x86_64-linux" => Ok("amd64"),
        _ => bail!("unsupported runtime artifact architecture"),
    }
}

fn validate_store_paths(paths: &[String]) -> Result<()> {
    if paths.is_empty() || paths.len() > MAX_STORE_PATHS {
        bail!("runtime artifact closure exceeds its path bounds");
    }
    for path in paths {
        validate_store_path(path)?;
    }
    Ok(())
}

fn validate_store_path(value: &str) -> Result<()> {
    let path = Path::new(value);
    if !path.is_absolute()
        || !path.starts_with("/nix/store")
        || path.components().any(|component| {
            matches!(
                component,
                std::path::Component::ParentDir | std::path::Component::CurDir
            )
        })
    {
        bail!("runtime artifact contains a path outside the project store");
    }
    Ok(())
}

#[cfg(test)]
fn verify_store_paths(store: &Path, paths: &[String]) -> Result<()> {
    for guest in paths {
        validate_store_path(guest)?;
        let relative = guest
            .strip_prefix("/nix")
            .context("store path has no /nix prefix")?;
        let host = store.join(relative.strip_prefix("/").unwrap_or(relative));
        if fs::symlink_metadata(&host).is_err() {
            bail!("prepared project store is missing a declared closure path");
        }
    }
    Ok(())
}

fn valid_sha256(value: &str) -> bool {
    value.len() == 64 && value.bytes().all(|byte| byte.is_ascii_hexdigit())
}

fn shell_quote(value: &str) -> String {
    format!("'{}'", value.replace('\'', "'\\''"))
}

fn bounded_diagnostic(value: &str) -> String {
    value
        .chars()
        .filter(|character| !character.is_control() || *character == '\n' || *character == '\t')
        .take(2048)
        .collect::<String>()
        .trim()
        .to_owned()
}

fn verify_sha256(path: &Path, expected: &str) -> Result<()> {
    let metadata = fs::symlink_metadata(path).context("installed artifact is missing")?;
    if !metadata.is_file() || metadata.file_type().is_symlink() {
        bail!("installed artifact is not a regular file");
    }
    let mut file = File::open(path)?;
    let mut digest = Sha256::new();
    let mut buffer = [0_u8; 64 * 1024];
    loop {
        let read = file.read(&mut buffer)?;
        if read == 0 {
            break;
        }
        digest.update(&buffer[..read]);
    }
    if hex::encode(digest.finalize()) != expected {
        bail!("installed artifact digest does not match its manifest");
    }
    Ok(())
}

fn read_manifest<T: for<'de> Deserialize<'de>>(path: &Path) -> Result<T> {
    let metadata = fs::symlink_metadata(path).context("installed manifest is missing")?;
    if !metadata.is_file()
        || metadata.file_type().is_symlink()
        || metadata.len() > MAX_MANIFEST_BYTES
    {
        bail!("installed manifest is not a bounded regular file");
    }
    serde_json::from_slice(&fs::read(path)?).context("installed manifest is invalid")
}

fn write_manifest<T: Serialize>(path: &Path, value: &T) -> Result<()> {
    let parent = path.parent().context("manifest has no parent")?;
    fs::create_dir_all(parent)?;
    let mut temporary = tempfile::NamedTempFile::new_in(parent)?;
    temporary
        .as_file()
        .set_permissions(fs::Permissions::from_mode(0o600))?;
    serde_json::to_writer(&mut temporary, value)?;
    temporary.write_all(b"\n")?;
    temporary.as_file().sync_all()?;
    temporary.persist(path).map_err(|error| error.error)?;
    Ok(())
}

fn require_real_directory(path: &Path, label: &str) -> Result<()> {
    let metadata = fs::symlink_metadata(path).with_context(|| format!("{label} is missing"))?;
    if !metadata.is_dir() || metadata.file_type().is_symlink() {
        bail!("{label} is not a real directory");
    }
    Ok(())
}

fn lock(path: &Path) -> Result<File> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let file = File::options().create(true).append(true).open(path)?;
    file.lock_exclusive()?;
    Ok(file)
}

async fn cleanup(sandbox: &Sandbox, name: &str) {
    let _ = sandbox.stop_and_wait().await;
    let _ = Sandbox::remove(name).await;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn store_paths_are_absolute_and_contained() {
        assert!(validate_store_path("/nix/store/abc-tool").is_ok());
        assert!(validate_store_path("/nix/store/abc-tool/bin/tool").is_ok());
        assert!(validate_store_path("/opt/tool").is_err());
        assert!(validate_store_path("/nix/store/../escape").is_err());
    }

    #[test]
    fn artifact_hashes_are_exact_lower_or_upper_hex() {
        assert!(valid_sha256(&"a".repeat(64)));
        assert!(valid_sha256(&"A".repeat(64)));
        assert!(!valid_sha256(&"g".repeat(64)));
        assert!(!valid_sha256(&"a".repeat(63)));
    }

    #[test]
    fn store_verification_maps_only_beneath_physical_nix_root() {
        let temporary = tempfile::tempdir().unwrap();
        let path = temporary.path().join("store/abc-tool/bin");
        fs::create_dir_all(&path).unwrap();
        fs::write(path.join("tool"), "ok").unwrap();

        verify_store_paths(temporary.path(), &["/nix/store/abc-tool/bin/tool".into()]).unwrap();
        assert!(verify_store_paths(temporary.path(), &["/nix/store/missing".into()]).is_err());
    }
}
