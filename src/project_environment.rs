use std::collections::BTreeMap;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::os::unix::fs::PermissionsExt;
use std::path::{Component, Path, PathBuf};

use anyhow::{bail, Context, Result};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::auth::{ACCESS_TOKEN_ENV, ACCOUNT_ID_ENV};
use crate::environment::BASE_IMAGE;
use crate::harness;
use crate::project::Project;

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
const MARKER: &str = ".fortlet-project.json";
const SCHEMA: u64 = 1;
const CONTRACT: &str = "fortlet-project-environment-v1";
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
    recipe: String,
    path: Vec<String>,
    environment: BTreeMap<String, String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PublishedProjectEnvironment {
    pub identity: String,
    pub root: PathBuf,
    pub path: Vec<String>,
    pub environment: BTreeMap<String, String>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Manifest {
    schema: u64,
    path: Vec<String>,
    environment: BTreeMap<String, String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct Marker {
    schema: u64,
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
        let platform = guest_platform()?;
        let identity = input_identity(&manifest_bytes, &recipe_bytes, platform);

        Ok(Some(Self {
            identity,
            recipe,
            path: manifest.path,
            environment: manifest.environment,
        }))
    }

    pub fn identity(&self) -> &str {
        &self.identity
    }

    pub fn recipe(&self) -> &str {
        &self.recipe
    }

    pub fn published(&self, root: PathBuf) -> PublishedProjectEnvironment {
        PublishedProjectEnvironment {
            identity: self.identity.clone(),
            root,
            path: self.path.clone(),
            environment: self.environment.clone(),
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
        let marker = Marker {
            schema: SCHEMA,
            identity: self.identity.clone(),
            output,
            image: BASE_IMAGE.into(),
            platform: guest_platform()?.into(),
        };
        let mut file = File::create(root.join(MARKER))?;
        serde_json::to_writer(&mut file, &marker)?;
        file.write_all(b"\n")?;
        Ok(())
    }

    pub fn verify_published(&self, root: &Path) -> Result<()> {
        let metadata = fs::symlink_metadata(root)
            .with_context(|| format!("cannot inspect project environment {}", root.display()))?;
        if !metadata.is_dir() || metadata.file_type().is_symlink() {
            bail!("published project environment is not a real directory");
        }
        let marker_bytes = read_bounded(&root.join(MARKER), MAX_MANIFEST_BYTES, "marker")?;
        let marker: Marker = serde_json::from_slice(&marker_bytes)
            .context("published project environment marker is invalid")?;
        if marker.schema != SCHEMA
            || marker.identity != self.identity
            || marker.image != BASE_IMAGE
            || marker.platform != guest_platform()?
        {
            bail!("published project environment marker does not match its inputs");
        }
        validate_declared_paths(root, &self.path)?;
        if output_digest(root)? != marker.output {
            bail!("published project environment content digest does not match its marker");
        }
        Ok(())
    }
}

fn validate_manifest(manifest: &Manifest, project: &Project) -> Result<()> {
    if manifest.schema != SCHEMA {
        bail!(
            "unsupported project environment schema {}; expected {SCHEMA}",
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

fn protected_environment_name(name: &str, project: &Project) -> Result<bool> {
    if name == "HOME"
        || name == "PATH"
        || name == ACCESS_TOKEN_ENV
        || name == ACCOUNT_ID_ENV
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

fn validate_environment_name(name: &str) -> Result<()> {
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

fn read_bounded(path: &Path, limit: u64, kind: &str) -> Result<Vec<u8>> {
    let metadata = fs::symlink_metadata(path).with_context(|| {
        format!(
            "cannot inspect project environment {kind} {}",
            path.display()
        )
    })?;
    if !metadata.is_file() || metadata.file_type().is_symlink() {
        bail!("project environment {kind} is not a regular file");
    }
    let file = File::open(path)
        .with_context(|| format!("cannot open project environment {kind} {}", path.display()))?;
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
    hash_field(&mut digest, CONTRACT.as_bytes());
    hash_field(&mut digest, BASE_IMAGE.as_bytes());
    hash_field(&mut digest, platform.as_bytes());
    hash_field(&mut digest, manifest);
    hash_field(&mut digest, recipe);
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
            if !canonical.starts_with(root) {
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
        assert_eq!(first.recipe(), "mkdir -p /out/bin\n");
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
    fn published_output_is_content_verified() {
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
        let tool = output.path().join("bin/tool");
        fs::write(&tool, "first").unwrap();
        fs::set_permissions(&tool, fs::Permissions::from_mode(0o755)).unwrap();

        environment.validate_and_mark(output.path()).unwrap();
        environment.verify_published(output.path()).unwrap();
        fs::write(&tool, "changed").unwrap();
        assert!(environment
            .verify_published(output.path())
            .unwrap_err()
            .to_string()
            .contains("content digest"));
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

        assert_eq!(environment.path, vec!["bin"]);
        assert_eq!(
            environment.environment.get("CARGO_HOME").unwrap(),
            "/home/agent/.cargo"
        );
        for pinned in [
            "rust_version=1.97.1",
            "jj_version=0.43.0",
            "libcap-ng-dev=0.8.3-1+b3",
            "temporary=\"$(mktemp -d \"$FORTLET_OUTPUT/.fortlet-work.XXXXXX\")\"",
            "trap 'rm -rf \"$temporary\"' EXIT",
            "9a7a2c336b4787f1b72f6bab7c35d5b7af2fd03cbd39b4fc721466a70d402a7d",
            "88f28fa9af20594179f85d6df67078dfd6fa93e2f6da5e1e9b0ac4997988ca4f",
            "289197b6bec60b4e57d47260624b617716f737eb02cdfd9155791b2576aa5862",
            "59e5588583ac82b623239929368c65b90735931c0f26b5a16c1f04d5bb97643d",
        ] {
            assert!(environment.recipe.contains(pinned), "{pinned}");
        }
    }
}
