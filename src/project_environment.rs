use std::collections::BTreeMap;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::os::unix::fs::{OpenOptionsExt, PermissionsExt};
use std::path::{Component, Path, PathBuf};

use anyhow::{bail, Context, Result};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::auth::{ACCESS_TOKEN_ENV, ACCOUNT_ID_ENV};
use crate::harness;
use crate::identity::{GIT_CONFIG_GLOBAL, GIT_CONFIG_NOSYSTEM, JJ_CONFIG};
use crate::project::Project;
use crate::runtime_artifacts::RUNTIME_CONTRACT;

pub const GUEST_ROOT: &str = "/opt/fortlet/project";
pub const NO_ENVIRONMENT: &str = "none";
pub const TERMINAL_ENVIRONMENT: &[&str] = &[
    "TERM",
    "COLORTERM",
    "TERM_PROGRAM",
    "TERM_PROGRAM_VERSION",
    "COLORFGBG",
    "NO_COLOR",
    "CLICOLOR",
    "CLICOLOR_FORCE",
    "FORCE_COLOR",
];

const MANIFEST_PATH: &str = ".fortlet/environment.json";
const RECIPE_PATH: &str = ".fortlet/environment.sh";
const FLAKE_PATH: &str = "flake.nix";
const LOCK_PATH: &str = "flake.lock";
const MARKER: &str = ".fortlet-project.json";
const RECIPE_SCHEMA: u64 = 1;
const NIX_SCHEMA: u64 = 2;
const DECLARATION_CONTRACT: &str = "fortlet-project-environment-v1";
const PUBLICATION_CONTRACT: &str = "fortlet-project-layer-v3";
pub const NIX_PROVIDER_CONTRACT: &str = "fortlet-nix-dev-shell-v2";
pub const NIX_VERSION: &str = "2.34.8";
const MAX_MANIFEST_BYTES: u64 = 64 * 1024;
const MAX_RECIPE_BYTES: u64 = 1024 * 1024;
const MAX_PATH_ENTRIES: usize = 32;
const MAX_ENVIRONMENT_ENTRIES: usize = 64;
const MAX_VALUE_BYTES: usize = 4096;
const MAX_OUTPUT_ENTRIES: u64 = 200_000;
const MAX_OUTPUT_BYTES: u64 = 4 * 1024 * 1024 * 1024;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectEnvironment {
    identity: String,
    provider: ProjectEnvironmentProvider,
    path: Vec<String>,
    environment: BTreeMap<String, String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum ProjectEnvironmentProvider {
    Recipe {
        recipe: String,
    },
    NixDevShell {
        name: String,
        project_root: PathBuf,
        project_identity: String,
        flake: Vec<u8>,
        lock: Vec<u8>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PublishedProjectEnvironment {
    pub identity: String,
    pub root: PathBuf,
    pub guest_root: String,
    pub path: Vec<String>,
    pub environment: BTreeMap<String, String>,
}

pub(crate) struct NixProjectEnvironment<'a> {
    pub name: &'a str,
    pub project_root: &'a Path,
    pub project_identity: &'a str,
    pub flake: &'a [u8],
    pub lock: &'a [u8],
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Manifest {
    schema: u64,
    path: Vec<String>,
    environment: BTreeMap<String, String>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct NixManifest {
    schema: u64,
    provider: NixProvider,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct NixProvider {
    kind: String,
    name: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct Marker {
    schema: u64,
    contract: String,
    identity: String,
    output: String,
    image: String,
    platform: String,
}

impl ProjectEnvironment {
    pub fn discover(project: &Project) -> Result<Option<Self>> {
        let manifest_path = project.root.join(MANIFEST_PATH);
        match fs::symlink_metadata(&manifest_path) {
            Ok(_) => {}
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(None),
            Err(error) => {
                return Err(error).with_context(|| {
                    format!(
                        "cannot inspect project environment {}",
                        manifest_path.display()
                    )
                });
            }
        }

        let manifest_bytes = snapshot_file(
            &project.root,
            &manifest_path,
            MAX_MANIFEST_BYTES,
            "manifest",
        )?;
        let platform = guest_platform()?;
        let schema = manifest_schema(&manifest_bytes)?;
        match schema {
            RECIPE_SCHEMA => {
                let recipe_bytes = snapshot_file(
                    &project.root,
                    &project.root.join(RECIPE_PATH),
                    MAX_RECIPE_BYTES,
                    "recipe",
                )?;
                let manifest: Manifest = serde_json::from_slice(&manifest_bytes)
                    .context("project environment manifest is not valid schema-1 JSON")?;
                validate_manifest(&manifest, project)?;
                let recipe = String::from_utf8(recipe_bytes.clone())
                    .context("project environment recipe is not valid UTF-8")?;
                if recipe.is_empty() {
                    bail!("project environment recipe is empty");
                }
                Ok(Some(Self {
                    identity: input_identity(&manifest_bytes, &recipe_bytes, platform),
                    provider: ProjectEnvironmentProvider::Recipe { recipe },
                    path: manifest.path,
                    environment: manifest.environment,
                }))
            }
            NIX_SCHEMA => {
                reject_adjacent_recipe(&project.root)?;
                let manifest: NixManifest = serde_json::from_slice(&manifest_bytes)
                    .context("project environment manifest is not valid schema-2 JSON")?;
                validate_nix_manifest(&manifest)?;
                let flake = snapshot_file(
                    &project.root,
                    &project.root.join(FLAKE_PATH),
                    MAX_RECIPE_BYTES,
                    "flake.nix",
                )?;
                let lock = snapshot_file(
                    &project.root,
                    &project.root.join(LOCK_PATH),
                    MAX_RECIPE_BYTES,
                    "flake.lock",
                )?;
                let identity = nix_input_identity(&manifest_bytes, &flake, &lock, platform);
                Ok(Some(Self {
                    identity,
                    provider: ProjectEnvironmentProvider::NixDevShell {
                        name: manifest.provider.name,
                        project_root: project.root.clone(),
                        project_identity: project.identity.as_str().to_owned(),
                        flake,
                        lock,
                    },
                    path: Vec::new(),
                    environment: BTreeMap::new(),
                }))
            }
            schema => bail!(
                "unsupported project environment schema {schema}; expected {RECIPE_SCHEMA} or {NIX_SCHEMA}"
            ),
        }
    }

    pub fn identity(&self) -> &str {
        &self.identity
    }

    pub fn recipe(&self) -> Option<&str> {
        match &self.provider {
            ProjectEnvironmentProvider::Recipe { recipe } => Some(recipe),
            ProjectEnvironmentProvider::NixDevShell { .. } => None,
        }
    }

    pub fn published_recipe(&self, root: PathBuf) -> Result<PublishedProjectEnvironment> {
        if !matches!(self.provider, ProjectEnvironmentProvider::Recipe { .. }) {
            bail!("Nix project environments do not publish recipe layers");
        }
        Ok(PublishedProjectEnvironment {
            identity: self.identity.clone(),
            root,
            guest_root: GUEST_ROOT.to_owned(),
            path: self
                .path
                .iter()
                .map(|entry| format!("{GUEST_ROOT}/{entry}"))
                .collect(),
            environment: self.environment.clone(),
        })
    }

    pub fn provider_name(&self) -> Option<&str> {
        match &self.provider {
            ProjectEnvironmentProvider::Recipe { .. } => None,
            ProjectEnvironmentProvider::NixDevShell { name, .. } => Some(name),
        }
    }

    pub fn activation_names(&self) -> Vec<&str> {
        self.environment.keys().map(String::as_str).collect()
    }

    pub fn project_identity(&self) -> Option<&str> {
        self.nix().map(|provider| provider.project_identity)
    }

    pub fn is_recipe(&self) -> bool {
        matches!(self.provider, ProjectEnvironmentProvider::Recipe { .. })
    }

    pub(crate) fn nix(&self) -> Option<NixProjectEnvironment<'_>> {
        match &self.provider {
            ProjectEnvironmentProvider::Recipe { .. } => None,
            ProjectEnvironmentProvider::NixDevShell {
                name,
                project_root,
                project_identity,
                flake,
                lock,
            } => Some(NixProjectEnvironment {
                name,
                project_root,
                project_identity,
                flake,
                lock,
            }),
        }
    }

    pub fn validate_and_mark(&self, root: &Path) -> Result<()> {
        match fs::symlink_metadata(root.join(MARKER)) {
            Ok(_) => bail!("project environment output uses reserved marker {MARKER}"),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            Err(error) => return Err(error.into()),
        }
        validate_declared_paths(root, &self.path)?;
        let output = output_digest(root)?;
        validate_output_modes(root, PublicationPhase::PreSeal)?;
        let marker = Marker {
            schema: RECIPE_SCHEMA,
            contract: PUBLICATION_CONTRACT.into(),
            identity: self.identity.clone(),
            output,
            image: RUNTIME_CONTRACT.into(),
            platform: guest_platform()?.into(),
        };
        let mut file = File::options()
            .write(true)
            .create_new(true)
            .mode(0o644)
            .open(root.join(MARKER))?;
        file.set_permissions(fs::Permissions::from_mode(0o644))?;
        serde_json::to_writer(&mut file, &marker)?;
        file.write_all(b"\n")?;
        file.sync_all()?;
        drop(file);
        seal_output(root)?;
        validate_output_modes(root, PublicationPhase::PostSeal)?;
        if output_digest(root)? != marker.output {
            bail!("project environment output changed while it was sealed");
        }
        Ok(())
    }

    pub fn verify_published(&self, root: &Path) -> Result<()> {
        let metadata = fs::symlink_metadata(root)
            .with_context(|| format!("cannot inspect project environment {}", root.display()))?;
        if !metadata.is_dir() || metadata.file_type().is_symlink() {
            bail!("published project environment is not a real directory");
        }
        validate_output_mode(
            metadata.permissions().mode() & 0o7777,
            true,
            PublicationPhase::PostSeal,
        )?;
        let marker_bytes = read_published_marker(root)?;
        let marker: Marker = serde_json::from_slice(&marker_bytes)
            .context("published project environment marker is invalid")?;
        if marker.schema != RECIPE_SCHEMA
            || marker.contract != PUBLICATION_CONTRACT
            || marker.identity != self.identity
            || marker.image != RUNTIME_CONTRACT
            || marker.platform != guest_platform()?
        {
            bail!("published project environment marker does not match its inputs");
        }
        validate_recorded_output_digest(&marker.output)?;
        Ok(())
    }
}

fn read_published_marker(root: &Path) -> Result<Vec<u8>> {
    let path = root.join(MARKER);
    let metadata = fs::symlink_metadata(&path)
        .context("cannot inspect published project environment marker")?;
    if !metadata.is_file() || metadata.file_type().is_symlink() {
        bail!("published project environment marker is not a regular file");
    }
    validate_output_mode(
        metadata.permissions().mode() & 0o7777,
        false,
        PublicationPhase::PostSeal,
    )?;
    if metadata.len() > MAX_MANIFEST_BYTES {
        bail!("project environment marker exceeds {MAX_MANIFEST_BYTES} bytes");
    }
    let file = File::open(path).context("cannot open published project environment marker")?;
    read_from(file, MAX_MANIFEST_BYTES, "marker")
}

fn validate_recorded_output_digest(output: &str) -> Result<()> {
    if output.len() != 64 || !output.bytes().all(|byte| byte.is_ascii_hexdigit()) {
        bail!("published project environment marker has an invalid output digest");
    }
    Ok(())
}

#[derive(Clone, Copy)]
enum PublicationPhase {
    PreSeal,
    PostSeal,
}

fn validate_output_modes(root: &Path, phase: PublicationPhase) -> Result<()> {
    fn validate(path: &Path, phase: PublicationPhase) -> Result<()> {
        let metadata = fs::symlink_metadata(path)?;
        if metadata.file_type().is_symlink() {
            return Ok(());
        }
        let actual = metadata.permissions().mode() & 0o7777;
        validate_output_mode(actual, metadata.is_dir(), phase)?;
        if metadata.is_dir() {
            for entry in fs::read_dir(path)? {
                validate(&entry?.path(), phase)?;
            }
        }
        Ok(())
    }
    validate(root, phase)
}

fn validate_output_mode(actual: u32, directory: bool, phase: PublicationPhase) -> Result<()> {
    let executable = directory || actual & 0o111 != 0;
    let expected = match (phase, executable) {
        (PublicationPhase::PreSeal, true) => 0o755,
        (PublicationPhase::PreSeal, false) => 0o644,
        (PublicationPhase::PostSeal, true) => 0o555,
        (PublicationPhase::PostSeal, false) => 0o444,
    };
    if actual != expected {
        bail!(
            "project environment output has noncanonical mode {actual:04o}; expected {expected:04o}"
        );
    }
    Ok(())
}

fn seal_output(root: &Path) -> Result<()> {
    fn seal(path: &Path) -> Result<()> {
        let metadata = fs::symlink_metadata(path)?;
        if metadata.is_dir() {
            for entry in fs::read_dir(path)? {
                seal(&entry?.path())?;
            }
        }
        if !metadata.file_type().is_symlink() {
            let mut permissions = metadata.permissions();
            permissions.set_mode(permissions.mode() & !0o222);
            fs::set_permissions(path, permissions)?;
        }
        Ok(())
    }
    seal(root)
}

pub(crate) fn restore_cleanup_permissions(root: &Path) -> Result<()> {
    fn restore(path: &Path) -> Result<()> {
        let metadata = fs::symlink_metadata(path)?;
        if metadata.file_type().is_symlink() {
            return Ok(());
        }
        let mut permissions = metadata.permissions();
        permissions.set_mode(if metadata.is_dir() { 0o700 } else { 0o600 });
        fs::set_permissions(path, permissions)?;
        if metadata.is_dir() {
            for entry in fs::read_dir(path)? {
                restore(&entry?.path())?;
            }
        }
        Ok(())
    }
    restore(root)
}

fn manifest_schema(bytes: &[u8]) -> Result<u64> {
    let value: serde_json::Value =
        serde_json::from_slice(bytes).context("project environment manifest is not valid JSON")?;
    value
        .as_object()
        .and_then(|object| object.get("schema"))
        .and_then(serde_json::Value::as_u64)
        .context("project environment manifest must contain an integer schema")
}

fn reject_adjacent_recipe(root: &Path) -> Result<()> {
    match fs::symlink_metadata(root.join(RECIPE_PATH)) {
        Ok(_) => bail!(
            "schema-2 project environment is ambiguous while {RECIPE_PATH} exists; remove the schema-1 recipe and retry"
        ),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(error) => Err(error).context("cannot inspect adjacent project environment recipe"),
    }
}

fn validate_nix_manifest(manifest: &NixManifest) -> Result<()> {
    if manifest.schema != NIX_SCHEMA {
        bail!("schema-2 project environment has an inconsistent schema");
    }
    if manifest.provider.kind != "nix-dev-shell" {
        bail!(
            "unsupported project environment provider {:?}; use \"nix-dev-shell\"",
            manifest.provider.kind
        );
    }
    let name = manifest.provider.name.as_bytes();
    let valid_start = name
        .first()
        .is_some_and(|byte| byte.is_ascii_alphabetic() || *byte == b'_');
    if name.is_empty()
        || name.len() > 64
        || !valid_start
        || !name
            .iter()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(*byte, b'_' | b'-' | b'\''))
    {
        bail!("Nix dev-shell name must be one bounded attribute component");
    }
    Ok(())
}

fn validate_manifest(manifest: &Manifest, project: &Project) -> Result<()> {
    if manifest.schema != RECIPE_SCHEMA {
        bail!(
            "unsupported project environment schema {}; expected {RECIPE_SCHEMA}",
            manifest.schema
        );
    }
    if manifest.path.len() > MAX_PATH_ENTRIES {
        bail!("project environment has too many PATH entries");
    }
    if manifest.environment.len() > MAX_ENVIRONMENT_ENTRIES {
        bail!("project environment has too many environment variables");
    }
    for entry in &manifest.path {
        validate_relative_path(entry)?;
    }
    for (key, value) in &manifest.environment {
        validate_environment_name(key)?;
        if value.len() > MAX_VALUE_BYTES || value.contains('\0') {
            bail!("project environment variable {key:?} has an invalid value");
        }
        if protected_environment_name(key, project)? {
            bail!("project environment variable {key:?} is reserved");
        }
    }
    Ok(())
}

pub(crate) fn protected_environment_name(name: &str, project: &Project) -> Result<bool> {
    if name == "HOME"
        || name == "PATH"
        || name == "BASH_ENV"
        || name == ACCESS_TOKEN_ENV
        || name == ACCOUNT_ID_ENV
        || matches!(name, GIT_CONFIG_GLOBAL | GIT_CONFIG_NOSYSTEM | JJ_CONFIG)
        || name.starts_with("FORTLET_")
        || name.starts_with("MSB_")
        || TERMINAL_ENVIRONMENT.contains(&name)
    {
        return Ok(true);
    }
    for harness_name in harness::names() {
        if harness::find(harness_name)?
            .environment(project)
            .iter()
            .any(|(key, _)| key == name)
        {
            return Ok(true);
        }
    }
    Ok(false)
}

pub(crate) fn validate_environment_name(name: &str) -> Result<()> {
    let mut characters = name.chars();
    let valid_start = characters
        .next()
        .is_some_and(|character| character == '_' || character.is_ascii_alphabetic());
    if !valid_start
        || !characters.all(|character| character == '_' || character.is_ascii_alphanumeric())
    {
        bail!("invalid project environment variable name {name:?}");
    }
    Ok(())
}

fn validate_relative_path(value: &str) -> Result<()> {
    if value.is_empty() || value.contains('\0') {
        bail!("invalid project environment PATH entry {value:?}");
    }
    let path = Path::new(value);
    if path
        .components()
        .any(|component| !matches!(component, Component::Normal(_)))
    {
        bail!("project environment PATH entry must be a normalized relative path: {value:?}");
    }
    Ok(())
}

fn snapshot_file(root: &Path, path: &Path, limit: u64, kind: &str) -> Result<Vec<u8>> {
    let canonical = path.canonicalize().with_context(|| {
        format!(
            "cannot resolve project environment {kind} {}",
            path.display()
        )
    })?;
    if !canonical.starts_with(root) {
        bail!("project environment {kind} resolves outside the project root");
    }
    let file = File::open(&canonical)
        .with_context(|| format!("cannot open project environment {kind} {}", path.display()))?;
    if !file.metadata()?.is_file() {
        bail!("project environment {kind} is not a regular file");
    }
    read_from(file, limit, kind)
}

fn read_from(file: File, limit: u64, kind: &str) -> Result<Vec<u8>> {
    let mut bytes = Vec::new();
    file.take(limit + 1).read_to_end(&mut bytes)?;
    if bytes.len() as u64 > limit {
        bail!("project environment {kind} exceeds {limit} bytes");
    }
    Ok(bytes)
}

fn input_identity(manifest: &[u8], recipe: &[u8], platform: &str) -> String {
    let mut digest = Sha256::new();
    hash_field(&mut digest, DECLARATION_CONTRACT.as_bytes());
    hash_field(&mut digest, PUBLICATION_CONTRACT.as_bytes());
    hash_field(&mut digest, RUNTIME_CONTRACT.as_bytes());
    hash_field(&mut digest, platform.as_bytes());
    hash_field(&mut digest, manifest);
    hash_field(&mut digest, recipe);
    hex::encode(digest.finalize())
}

fn nix_input_identity(manifest: &[u8], flake: &[u8], lock: &[u8], platform: &str) -> String {
    let mut digest = Sha256::new();
    hash_field(&mut digest, NIX_PROVIDER_CONTRACT.as_bytes());
    hash_field(&mut digest, NIX_VERSION.as_bytes());
    hash_field(&mut digest, RUNTIME_CONTRACT.as_bytes());
    hash_field(&mut digest, platform.as_bytes());
    hash_field(&mut digest, manifest);
    hash_field(&mut digest, flake);
    hash_field(&mut digest, lock);
    hex::encode(digest.finalize())
}

fn hash_field(digest: &mut Sha256, value: &[u8]) {
    digest.update((value.len() as u64).to_be_bytes());
    digest.update(value);
}

pub(crate) fn guest_platform() -> Result<&'static str> {
    match std::env::consts::ARCH {
        "aarch64" => Ok("linux/aarch64"),
        "x86_64" => Ok("linux/x86_64"),
        architecture => bail!("unsupported project environment architecture {architecture}"),
    }
}

pub(crate) fn guest_system() -> Result<&'static str> {
    match std::env::consts::ARCH {
        "aarch64" => Ok("aarch64-linux"),
        "x86_64" => Ok("x86_64-linux"),
        architecture => bail!("unsupported project environment architecture {architecture}"),
    }
}

fn validate_declared_paths(root: &Path, entries: &[String]) -> Result<()> {
    for entry in entries {
        let path = root.join(entry);
        if !path.metadata().is_ok_and(|metadata| metadata.is_dir()) {
            bail!("project environment did not create PATH directory {entry:?}");
        }
    }
    Ok(())
}

fn output_digest(root: &Path) -> Result<String> {
    let canonical_root = root.canonicalize()?;
    let mut entries = Vec::new();
    collect_entries(root, root, &mut entries)?;
    entries.sort_by(|left, right| left.0.cmp(&right.0));
    if entries.len() as u64 > MAX_OUTPUT_ENTRIES {
        bail!("project environment output has too many entries");
    }

    let mut total_bytes = 0_u64;
    let mut digest = Sha256::new();
    for (relative, path) in entries {
        if relative == MARKER {
            continue;
        }
        let metadata = fs::symlink_metadata(&path)?;
        hash_field(&mut digest, relative.as_bytes());
        digest.update([u8::from(metadata.permissions().mode() & 0o111 != 0)]);
        if metadata.is_file() {
            digest.update(b"file");
            total_bytes = total_bytes
                .checked_add(metadata.len())
                .context("project environment output size overflow")?;
            if total_bytes > MAX_OUTPUT_BYTES {
                bail!("project environment output exceeds {MAX_OUTPUT_BYTES} bytes");
            }
            digest.update(metadata.len().to_be_bytes());
            let mut file = File::open(path)?;
            let mut buffer = [0_u8; 64 * 1024];
            loop {
                let read = file.read(&mut buffer)?;
                if read == 0 {
                    break;
                }
                digest.update(&buffer[..read]);
            }
        } else if metadata.is_dir() {
            digest.update(b"directory");
        } else if metadata.file_type().is_symlink() {
            digest.update(b"symlink");
            let target = fs::read_link(&path)?;
            if target.is_absolute() {
                bail!("project environment output contains an absolute link");
            }
            let canonical = path
                .parent()
                .context("project environment link has no parent")?
                .join(&target)
                .canonicalize()
                .context("project environment output contains a broken link")?;
            if !canonical.starts_with(&canonical_root) {
                bail!("project environment output contains an escaping link");
            }
            hash_field(&mut digest, target.as_os_str().as_encoded_bytes());
        } else {
            bail!("project environment output contains an unsupported file type");
        }
    }
    Ok(hex::encode(digest.finalize()))
}

fn collect_entries(
    root: &Path,
    directory: &Path,
    entries: &mut Vec<(String, PathBuf)>,
) -> Result<()> {
    for entry in fs::read_dir(directory)? {
        let entry = entry?;
        let path = entry.path();
        let relative = path.strip_prefix(root)?;
        let relative = relative
            .to_str()
            .context("project environment output path is not valid UTF-8")?
            .to_owned();
        entries.push((relative, path.clone()));
        if fs::symlink_metadata(&path)?.is_dir() {
            collect_entries(root, &path, entries)?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::project::ProjectIdentity;

    fn project(root: &Path) -> Project {
        let root = root.canonicalize().unwrap();
        Project {
            identity: ProjectIdentity::from_local_root(&root),
            root: root.clone(),
            cwd: root,
            kind: "test",
            scratch: false,
        }
    }

    fn write_environment(root: &Path, manifest: &str, recipe: &str) {
        fs::create_dir_all(root.join(".fortlet")).unwrap();
        fs::write(root.join(MANIFEST_PATH), manifest).unwrap();
        fs::write(root.join(RECIPE_PATH), recipe).unwrap();
    }

    fn write_nix_environment(root: &Path, manifest: &str, flake: &str, lock: &str) {
        fs::create_dir_all(root.join(".fortlet")).unwrap();
        fs::write(root.join(MANIFEST_PATH), manifest).unwrap();
        fs::write(root.join(FLAKE_PATH), flake).unwrap();
        fs::write(root.join(LOCK_PATH), lock).unwrap();
    }

    fn set_mode(path: &Path, mode: u32) {
        fs::set_permissions(path, fs::Permissions::from_mode(mode)).unwrap();
    }

    fn rewrite_published_marker(root: &Path, marker: &Marker) {
        let marker_path = root.join(MARKER);
        set_mode(root, 0o755);
        set_mode(&marker_path, 0o644);
        let mut bytes = serde_json::to_vec(marker).unwrap();
        bytes.push(b'\n');
        fs::write(&marker_path, bytes).unwrap();
        set_mode(&marker_path, 0o444);
        set_mode(root, 0o555);
    }

    fn legacy_recipe_identity(manifest: &[u8], recipe: &[u8], platform: &str) -> String {
        let mut digest = Sha256::new();
        hash_field(&mut digest, DECLARATION_CONTRACT.as_bytes());
        hash_field(&mut digest, RUNTIME_CONTRACT.as_bytes());
        hash_field(&mut digest, platform.as_bytes());
        hash_field(&mut digest, manifest);
        hash_field(&mut digest, recipe);
        hex::encode(digest.finalize())
    }

    #[test]
    fn absence_is_opt_in_and_side_effect_free() {
        let temporary = tempfile::tempdir().unwrap();
        assert_eq!(
            ProjectEnvironment::discover(&project(temporary.path())).unwrap(),
            None
        );
        assert!(fs::read_dir(temporary.path()).unwrap().next().is_none());
    }

    #[test]
    fn snapshots_both_exact_inputs_into_identity() {
        let temporary = tempfile::tempdir().unwrap();
        let manifest = r#"{"schema":1,"path":["bin"],"environment":{"RUST_BACKTRACE":"1"}}"#;
        write_environment(temporary.path(), manifest, "mkdir -p /out/bin\n");
        let first = ProjectEnvironment::discover(&project(temporary.path()))
            .unwrap()
            .unwrap();

        write_environment(temporary.path(), manifest, "mkdir -p /out/tools\n");
        let recipe_changed = ProjectEnvironment::discover(&project(temporary.path()))
            .unwrap()
            .unwrap();
        write_environment(
            temporary.path(),
            &format!("{manifest}\n"),
            "mkdir -p /out/bin\n",
        );
        let manifest_changed = ProjectEnvironment::discover(&project(temporary.path()))
            .unwrap()
            .unwrap();

        assert_ne!(first.identity(), recipe_changed.identity());
        assert_ne!(first.identity(), manifest_changed.identity());
        assert_ne!(
            first.identity(),
            legacy_recipe_identity(
                manifest.as_bytes(),
                b"mkdir -p /out/bin\n",
                guest_platform().unwrap()
            )
        );
        assert_eq!(first.recipe(), Some("mkdir -p /out/bin\n"));
    }

    #[test]
    fn schema_two_is_explicit_locked_and_snapshot_identified() {
        let temporary = tempfile::tempdir().unwrap();
        let manifest = r#"{"schema":2,"provider":{"kind":"nix-dev-shell","name":"default"}}"#;
        write_nix_environment(temporary.path(), manifest, "{ outputs = _: {}; }\n", "{}\n");

        let first = ProjectEnvironment::discover(&project(temporary.path()))
            .unwrap()
            .unwrap();
        assert!(!first.is_recipe());
        assert_eq!(first.provider_name(), Some("default"));
        assert!(first.nix().is_some());

        fs::write(temporary.path().join(LOCK_PATH), "{\"version\": 7}\n").unwrap();
        let changed = ProjectEnvironment::discover(&project(temporary.path()))
            .unwrap()
            .unwrap();
        assert_ne!(first.identity(), changed.identity());
    }

    #[test]
    fn schema_two_rejects_ambiguity_missing_locks_and_unknown_provider_fields() {
        let cases = [
            (
                r#"{"schema":2,"provider":{"kind":"other","name":"default"}}"#,
                "unsupported",
            ),
            (
                r#"{"schema":2,"provider":{"kind":"nix-dev-shell","name":"bad.name"}}"#,
                "attribute component",
            ),
            (
                r#"{"schema":2,"provider":{"kind":"nix-dev-shell","name":"default","path":"x"}}"#,
                "unknown field",
            ),
        ];
        for (manifest, expected) in cases {
            let temporary = tempfile::tempdir().unwrap();
            write_nix_environment(temporary.path(), manifest, "{}\n", "{}\n");
            let error = ProjectEnvironment::discover(&project(temporary.path())).unwrap_err();
            assert!(format!("{error:#}").contains(expected), "{error:#}");
        }

        let temporary = tempfile::tempdir().unwrap();
        write_nix_environment(
            temporary.path(),
            r#"{"schema":2,"provider":{"kind":"nix-dev-shell","name":"default"}}"#,
            "{}\n",
            "{}\n",
        );
        fs::write(temporary.path().join(RECIPE_PATH), "true\n").unwrap();
        let error = ProjectEnvironment::discover(&project(temporary.path())).unwrap_err();
        assert!(error.to_string().contains("ambiguous"));

        fs::remove_file(temporary.path().join(RECIPE_PATH)).unwrap();
        fs::remove_file(temporary.path().join(LOCK_PATH)).unwrap();
        let error = ProjectEnvironment::discover(&project(temporary.path())).unwrap_err();
        assert!(format!("{error:#}").contains("flake.lock"));
    }

    #[test]
    fn rejects_unknown_schema_paths_variables_and_outside_recipe() {
        let temporary = tempfile::tempdir().unwrap();
        let cases = [
            (r#"{"schema":2,"path":[],"environment":{}}"#, "schema"),
            (
                r#"{"schema":1,"path":["../bin"],"environment":{}}"#,
                "relative path",
            ),
            (
                r#"{"schema":1,"path":[],"environment":{"HOME":"/tmp"}}"#,
                "reserved",
            ),
            (
                r#"{"schema":1,"path":[],"environment":{"BASH_ENV":"/tmp/hook"}}"#,
                "reserved",
            ),
            (
                r#"{"schema":1,"path":[],"environment":{},"extra":true}"#,
                "unknown field",
            ),
        ];
        for (manifest, expected) in cases {
            write_environment(temporary.path(), manifest, "true\n");
            let error = ProjectEnvironment::discover(&project(temporary.path())).unwrap_err();
            assert!(format!("{error:#}").contains(expected), "{error:#}");
        }

        let outside = tempfile::NamedTempFile::new().unwrap();
        write_environment(
            temporary.path(),
            r#"{"schema":1,"path":[],"environment":{}}"#,
            "true\n",
        );
        fs::remove_file(temporary.path().join(RECIPE_PATH)).unwrap();
        std::os::unix::fs::symlink(outside.path(), temporary.path().join(RECIPE_PATH)).unwrap();
        let error = ProjectEnvironment::discover(&project(temporary.path())).unwrap_err();
        assert!(error.to_string().contains("outside the project root"));
    }

    #[test]
    fn published_output_is_content_digested_and_sealed() {
        let project_root = tempfile::tempdir().unwrap();
        write_environment(
            project_root.path(),
            r#"{"schema":1,"path":["bin"],"environment":{}}"#,
            "true\n",
        );
        let environment = ProjectEnvironment::discover(&project(project_root.path()))
            .unwrap()
            .unwrap();
        let output = tempfile::tempdir().unwrap();
        fs::create_dir(output.path().join("bin")).unwrap();
        fs::create_dir(output.path().join("share")).unwrap();
        let tool = output.path().join("bin/tool");
        fs::write(&tool, "first").unwrap();
        set_mode(&tool, 0o755);
        let data = output.path().join("share/odd\n name");
        fs::write(&data, "data").unwrap();
        set_mode(&data, 0o644);
        std::os::unix::fs::symlink("../share/odd\n name", output.path().join("bin/data")).unwrap();
        set_mode(output.path(), 0o755);

        environment.validate_and_mark(output.path()).unwrap();
        environment.verify_published(output.path()).unwrap();
        assert_eq!(
            fs::metadata(output.path()).unwrap().permissions().mode() & 0o7777,
            0o555
        );
        assert_eq!(
            fs::metadata(output.path().join("bin"))
                .unwrap()
                .permissions()
                .mode()
                & 0o7777,
            0o555
        );
        assert_eq!(
            fs::metadata(&tool).unwrap().permissions().mode() & 0o7777,
            0o555
        );
        assert_eq!(
            fs::metadata(&data).unwrap().permissions().mode() & 0o7777,
            0o444
        );
        assert_eq!(
            fs::metadata(output.path().join(MARKER))
                .unwrap()
                .permissions()
                .mode()
                & 0o7777,
            0o444
        );
        let marker: Marker =
            serde_json::from_slice(&fs::read(output.path().join(MARKER)).unwrap()).unwrap();
        assert_eq!(marker.contract, PUBLICATION_CONTRACT);
        assert!(fs::write(&tool, "changed").is_err());

        set_mode(&tool, 0o755);
        fs::write(&tool, "changed").unwrap();
        set_mode(&tool, 0o555);
        assert_ne!(output_digest(output.path()).unwrap(), marker.output);
        environment.verify_published(output.path()).unwrap();

        set_mode(&tool, 0o500);
        environment.verify_published(output.path()).unwrap();
        restore_cleanup_permissions(output.path()).unwrap();
    }

    #[test]
    fn bounded_reuse_validates_only_the_root_and_marker() {
        let project_root = tempfile::tempdir().unwrap();
        write_environment(
            project_root.path(),
            r#"{"schema":1,"path":["bin"],"environment":{}}"#,
            "true\n",
        );
        let environment = ProjectEnvironment::discover(&project(project_root.path()))
            .unwrap()
            .unwrap();
        let output = tempfile::tempdir().unwrap();
        fs::create_dir(output.path().join("bin")).unwrap();
        fs::write(output.path().join("bin/tool"), "content").unwrap();
        set_mode(output.path(), 0o755);
        environment.validate_and_mark(output.path()).unwrap();

        set_mode(&output.path().join("bin"), 0o700);
        environment.verify_published(output.path()).unwrap();

        set_mode(output.path(), 0o755);
        let error = environment.verify_published(output.path()).unwrap_err();
        assert!(error.to_string().contains("expected 0555"));
        set_mode(output.path(), 0o555);

        let marker_path = output.path().join(MARKER);
        set_mode(&marker_path, 0o644);
        let error = environment.verify_published(output.path()).unwrap_err();
        assert!(error.to_string().contains("expected 0444"));
        set_mode(&marker_path, 0o444);

        let marker: Marker = serde_json::from_slice(&fs::read(&marker_path).unwrap()).unwrap();
        let mut malformed_digest = marker.clone();
        malformed_digest.output = "not-a-digest".into();
        rewrite_published_marker(output.path(), &malformed_digest);
        let error = environment.verify_published(output.path()).unwrap_err();
        assert!(error.to_string().contains("invalid output digest"));

        let mismatched_markers = [
            Marker {
                schema: RECIPE_SCHEMA + 1,
                ..marker.clone()
            },
            Marker {
                contract: "fortlet-project-layer-v2".into(),
                ..marker.clone()
            },
            Marker {
                identity: "0".repeat(64),
                ..marker.clone()
            },
            Marker {
                image: "other-runtime".into(),
                ..marker.clone()
            },
            Marker {
                platform: "linux/other".into(),
                ..marker.clone()
            },
        ];
        for mismatched in mismatched_markers {
            rewrite_published_marker(output.path(), &mismatched);
            let error = environment.verify_published(output.path()).unwrap_err();
            assert!(error.to_string().contains("does not match its inputs"));
        }

        rewrite_published_marker(output.path(), &marker);
        set_mode(output.path(), 0o755);
        fs::rename(&marker_path, output.path().join("saved-marker")).unwrap();
        std::os::unix::fs::symlink("saved-marker", &marker_path).unwrap();
        set_mode(output.path(), 0o555);
        let error = environment.verify_published(output.path()).unwrap_err();
        assert!(error.to_string().contains("not a regular file"));

        restore_cleanup_permissions(output.path()).unwrap();
    }

    #[test]
    fn publication_rejects_noncanonical_preseal_modes_and_special_files() {
        let project_root = tempfile::tempdir().unwrap();
        write_environment(
            project_root.path(),
            r#"{"schema":1,"path":[],"environment":{}}"#,
            "true\n",
        );
        let environment = ProjectEnvironment::discover(&project(project_root.path()))
            .unwrap()
            .unwrap();

        let legacy = tempfile::tempdir().unwrap();
        set_mode(legacy.path(), 0o500);
        let error = environment.validate_and_mark(legacy.path()).unwrap_err();
        assert!(error.to_string().contains("noncanonical mode 0500"));
        restore_cleanup_permissions(legacy.path()).unwrap();

        let error = validate_output_mode(0o4755, false, PublicationPhase::PreSeal).unwrap_err();
        assert!(error.to_string().contains("noncanonical mode 4755"));

        let special = tempfile::tempdir().unwrap();
        let socket_path = special.path().join("socket");
        let socket = std::os::unix::net::UnixListener::bind(&socket_path).unwrap();
        let error = environment.validate_and_mark(special.path()).unwrap_err();
        assert!(error.to_string().contains("unsupported file type"));
        drop(socket);
    }

    #[test]
    fn output_rejects_missing_paths_and_escaping_links() {
        let project_root = tempfile::tempdir().unwrap();
        write_environment(
            project_root.path(),
            r#"{"schema":1,"path":["bin"],"environment":{}}"#,
            "true\n",
        );
        let environment = ProjectEnvironment::discover(&project(project_root.path()))
            .unwrap()
            .unwrap();
        let output = tempfile::tempdir().unwrap();
        assert!(environment
            .validate_and_mark(output.path())
            .unwrap_err()
            .to_string()
            .contains("PATH directory"));

        fs::create_dir(output.path().join("bin")).unwrap();
        std::os::unix::fs::symlink("/tmp", output.path().join("bin/outside")).unwrap();
        assert!(environment
            .validate_and_mark(output.path())
            .unwrap_err()
            .to_string()
            .contains("absolute link"));
    }

    #[test]
    fn failed_update_preserves_the_last_verified_output() {
        let project_root = tempfile::tempdir().unwrap();
        write_environment(
            project_root.path(),
            r#"{"schema":1,"path":["bin"],"environment":{}}"#,
            "first\n",
        );
        let first = ProjectEnvironment::discover(&project(project_root.path()))
            .unwrap()
            .unwrap();
        let first_output = tempfile::tempdir().unwrap();
        fs::create_dir(first_output.path().join("bin")).unwrap();
        fs::write(first_output.path().join("bin/tool"), "working").unwrap();
        set_mode(first_output.path(), 0o755);
        first.validate_and_mark(first_output.path()).unwrap();

        write_environment(
            project_root.path(),
            r#"{"schema":1,"path":["bin"],"environment":{}}"#,
            "changed\n",
        );
        let update = ProjectEnvironment::discover(&project(project_root.path()))
            .unwrap()
            .unwrap();
        let failed_output = tempfile::tempdir().unwrap();
        assert_ne!(first.identity(), update.identity());
        assert!(update.validate_and_mark(failed_output.path()).is_err());
        first.verify_published(first_output.path()).unwrap();
        assert_eq!(
            fs::read_to_string(first_output.path().join("bin/tool")).unwrap(),
            "working"
        );
        restore_cleanup_permissions(first_output.path()).unwrap();
    }

    #[test]
    fn reserved_marker_cannot_redirect_host_writes() {
        let project_root = tempfile::tempdir().unwrap();
        write_environment(
            project_root.path(),
            r#"{"schema":1,"path":[],"environment":{}}"#,
            "true\n",
        );
        let environment = ProjectEnvironment::discover(&project(project_root.path()))
            .unwrap()
            .unwrap();
        let output = tempfile::tempdir().unwrap();
        let outside = tempfile::NamedTempFile::new().unwrap();
        fs::remove_file(outside.path()).unwrap();
        std::os::unix::fs::symlink(outside.path(), output.path().join(MARKER)).unwrap();

        assert!(environment
            .validate_and_mark(output.path())
            .unwrap_err()
            .to_string()
            .contains("reserved marker"));
        assert!(!outside.path().exists());
    }

    #[test]
    fn repository_fixture_pins_the_declared_toolchain() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .canonicalize()
            .unwrap();
        let environment = ProjectEnvironment::discover(&project(&root))
            .unwrap()
            .unwrap();

        assert_eq!(environment.path, vec!["bin", "zig"]);
        assert_eq!(
            environment.environment.get("CARGO_HOME").unwrap(),
            "/home/agent/.cargo"
        );
        assert_eq!(
            environment.environment.get("CARGO_TARGET_DIR").unwrap(),
            "target/fortlet-guest"
        );
        for pinned in [
            "rust_version=1.97.1",
            "jj_version=0.43.0",
            "zig_version=0.15.2",
            "${package}_0.8.3-1+b3_${debian_arch}.deb",
            "bsdtar -xOf \"$archive\" data.tar.xz",
            "temporary=\"$(mktemp -d \"$FORTLET_OUTPUT/.fortlet-work.XXXXXX\")\"",
            "trap 'rm -rf \"$temporary\"' EXIT",
            "9a7a2c336b4787f1b72f6bab7c35d5b7af2fd03cbd39b4fc721466a70d402a7d",
            "88f28fa9af20594179f85d6df67078dfd6fa93e2f6da5e1e9b0ac4997988ca4f",
            "289197b6bec60b4e57d47260624b617716f737eb02cdfd9155791b2576aa5862",
            "59e5588583ac82b623239929368c65b90735931c0f26b5a16c1f04d5bb97643d",
            "958ed7d1e00d0ea76590d27666efbf7a932281b3d7ba0c6b01b0ff26498f667f",
            "02aa270f183da276e5b5920b1dac44a63f1a49e55050ebde3aecc9eb82f93239",
        ] {
            assert!(environment.recipe().unwrap().contains(pinned), "{pinned}");
        }
    }

    #[test]
    fn repository_fixture_extracts_pinned_debian_artifacts_without_a_package_manager() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .canonicalize()
            .unwrap();
        let environment = ProjectEnvironment::discover(&project(&root))
            .unwrap()
            .unwrap();

        for required in [
            "debian_triplet=aarch64-linux-gnu",
            "debian_triplet=x86_64-linux-gnu",
            "debian_arch=arm64",
            "debian_arch=amd64",
            "24e74ad29a37d2a3940b8977d11298a7afc77379ef414b561d79c64147d740e0",
            "92ac2d723583ac9a34340f00c61adbf6a3ae613ec395541bc32d428f6c16c092",
            "b4b54769c77e4a71c8b33aee4d600ba28a9994a1c6f60d55d4ebe7fc44882e07",
            "50674ccc126009f8d640a9230db4600d6fe552b68077193f234ea892784db5d5",
            "bsdtar -xOf \"$archive\" data.tar.xz",
            "[ \"$link_target\" != \"$expected_target\" ] || [ ! -f \"$FORTLET_OUTPUT$expected_target\" ]",
            "ln -snf \"../../..$expected_target\" \"$link\"",
            "/lib/$debian_triplet/libcap-ng.so.0.0.0",
            "/lib/$debian_triplet/libdrop_ambient.so.0.0.0",
        ] {
            assert!(environment.recipe().unwrap().contains(required), "{required}");
        }
        for forbidden in ["apt-get", "dpkg-deb"] {
            assert!(!environment.recipe().unwrap().contains(forbidden));
        }
    }
}
