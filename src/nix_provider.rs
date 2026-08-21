use std::collections::BTreeMap;
use std::fs::{self, File};
use std::io::Write;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use anyhow::{bail, Context, Result};
use fs2::FileExt;
use microsandbox::Sandbox;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::environment::BASE_IMAGE;
use crate::paths::AppPaths;
use crate::project::Project;
use crate::project_environment::{
    guest_system, protected_environment_name, validate_environment_name, NixProjectEnvironment,
    ProjectEnvironment, PublishedProjectEnvironment, NIX_PROVIDER_CONTRACT, NIX_VERSION,
};
use crate::runtime::{effective_gid, effective_uid};

const RECORD_SCHEMA: u64 = 1;
const GUEST_STORE: &str = "/nix";
const MAX_VARIABLES: usize = 256;
const MAX_VALUE_BYTES: usize = 16 * 1024;
const HOLD_SCRIPT: &str = "trap 'exit 0' TERM INT; while :; do sleep 3600 & wait $!; done";

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct ResolvedRecord {
    schema: u64,
    contract: String,
    declaration_identity: String,
    identity: String,
    nix_version: String,
    system: String,
    source: String,
    derivation: String,
    path: Vec<String>,
    environment: BTreeMap<String, String>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct StructuredEnvironment {
    variables: BTreeMap<String, StructuredVariable>,
    #[serde(rename = "bashFunctions")]
    bash_functions: BTreeMap<String, String>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct StructuredVariable {
    #[serde(rename = "type")]
    kind: String,
    value: serde_json::Value,
}

pub async fn ensure(
    paths: &AppPaths,
    project: &Project,
    environment: &ProjectEnvironment,
) -> Result<PublishedProjectEnvironment> {
    let declaration = environment
        .nix()
        .context("project environment is not a Nix dev shell")?;
    let record_path = paths.provider_record(declaration.project_identity);
    let store = paths.nix_store(declaration.project_identity);
    if let Some(record) = verified_record(&record_path, &store, environment.identity())? {
        return Ok(published(record, store));
    }

    let _lock = lock(&paths.locks().join(format!(
        "nix-provider-{}.lock",
        declaration.project_identity
    )))?;
    if let Some(record) = verified_record(&record_path, &store, environment.identity())? {
        return Ok(published(record, store));
    }
    verify_live_inputs(&declaration)?;
    reject_authenticated_lock(declaration.lock)?;
    ensure_owned_directory(&paths.provider_root(declaration.project_identity))?;
    ensure_owned_directory(&store)?;

    eprintln!("fortlet: preparing Nix project environment (first use)");
    let record = provision(&store, project, environment, &declaration).await?;
    write_record(&record_path, &record)?;
    let record = verified_record(&record_path, &store, environment.identity())?
        .context("resolved Nix project environment record was not published")?;
    Ok(published(record, store))
}

async fn provision(
    store: &Path,
    project: &Project,
    environment: &ProjectEnvironment,
    declaration: &NixProjectEnvironment<'_>,
) -> Result<ResolvedRecord> {
    let provider_root = store.parent().context("Nix provider store has no parent")?;
    let home = tempfile::Builder::new()
        .prefix(".nix-home-")
        .tempdir_in(provider_root)?;
    let result = tempfile::Builder::new()
        .prefix(".nix-result-")
        .tempdir_in(provider_root)?;
    let unique = SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos();
    let sandbox_name = format!("fortlet-nix-provision-{}-{unique:x}", std::process::id());
    let project_path = project.root.display().to_string();
    let sandbox = Sandbox::builder(&sandbox_name)
        .image(BASE_IMAGE)
        .cpus(4)
        .memory(8192)
        .root_disk(16384)
        .volume(GUEST_STORE, |mount| mount.bind(store))
        .volume("/home/agent", |mount| mount.bind(home.path()))
        .volume("/result", |mount| mount.bind(result.path()))
        .volume(&project_path, |mount| mount.bind(&project.root).readonly())
        .script("hold", HOLD_SCRIPT)
        .entrypoint(["hold"])
        .create()
        .await
        .context("cannot create Nix project-environment provisioning capsule")?;

    let outcome = provision_in_sandbox(&sandbox, project, environment, declaration).await;
    cleanup(&sandbox, &sandbox_name).await;
    outcome
}

async fn provision_in_sandbox(
    sandbox: &Sandbox,
    project: &Project,
    environment: &ProjectEnvironment,
    declaration: &NixProjectEnvironment<'_>,
) -> Result<ResolvedRecord> {
    let (url, digest) = nix_distribution()?;
    let bootstrap = format!(
        r#"set -eu
export DEBIAN_FRONTEND=noninteractive
apt-get update -qq
apt-get install -y -qq ca-certificates curl xz-utils
curl --fail --location --silent --show-error {url} -o /tmp/nix.tar.xz
printf '%s  %s\n' '{digest}' /tmp/nix.tar.xz | sha256sum -c -
tar -xJf /tmp/nix.tar.xz -C /tmp
getent group {gid} >/dev/null || groupadd -g {gid} fortlet-agent
getent passwd {uid} >/dev/null || useradd -u {uid} -g {gid} -d /home/agent -M fortlet-agent
chown {uid}:{gid} /nix /home/agent /result
"#,
        uid = effective_uid(),
        gid = effective_gid(),
    );
    checked_exec(sandbox, "/bin/sh", &["-c", &bootstrap], None).await?;

    let install = r#"set -eu
set -- /tmp/nix-*/install
test "$#" -eq 1
"$1" --no-daemon --yes --no-channel-add --no-modify-profile
"#;
    checked_exec(
        sandbox,
        "/bin/sh",
        &["-c", install],
        Some(agent_environment()),
    )
    .await?;

    let nix = "/home/agent/.nix-profile/bin/nix";
    let common = nix_arguments();
    let flake_reference = format!("path:{}", project.root.display());
    let mut archive_arguments = common.clone();
    archive_arguments.extend([
        "flake".to_owned(),
        "archive".to_owned(),
        "--json".to_owned(),
        "--no-write-lock-file".to_owned(),
        flake_reference,
    ]);
    let archive = checked_exec_owned(sandbox, nix, &archive_arguments, agent_environment()).await?;
    let source = archived_source(&archive)?;
    let installable = format!(
        "{source}#devShells.{}.{}",
        guest_system()?,
        declaration.name
    );

    let mut derivation_arguments = common.clone();
    derivation_arguments.extend([
        "eval".to_owned(),
        "--raw".to_owned(),
        format!("{installable}.drvPath"),
    ]);
    let derivation = checked_exec_owned(sandbox, nix, &derivation_arguments, agent_environment())
        .await?
        .trim()
        .to_owned();
    require_store_path(&derivation)?;

    let mut environment_arguments = common;
    environment_arguments.extend([
        "print-dev-env".to_owned(),
        "--json".to_owned(),
        "--no-write-lock-file".to_owned(),
        installable,
    ]);
    let structured =
        checked_exec_owned(sandbox, nix, &environment_arguments, agent_environment()).await?;
    let (path, activation) = parse_activation(&structured, project)?;

    let mut record = ResolvedRecord {
        schema: RECORD_SCHEMA,
        contract: NIX_PROVIDER_CONTRACT.to_owned(),
        declaration_identity: environment.identity().to_owned(),
        identity: String::new(),
        nix_version: NIX_VERSION.to_owned(),
        system: guest_system()?.to_owned(),
        source,
        derivation,
        path,
        environment: activation,
    };
    record.identity = resolved_identity(&record);
    Ok(record)
}

fn nix_arguments() -> Vec<String> {
    vec![
        "--extra-experimental-features".into(),
        "nix-command flakes".into(),
        "--option".into(),
        "accept-flake-config".into(),
        "false".into(),
        "--option".into(),
        "flake-registry".into(),
        "".into(),
    ]
}

fn agent_environment() -> Vec<(String, String)> {
    vec![
        ("HOME".into(), "/home/agent".into()),
        ("USER".into(), "fortlet-agent".into()),
        ("LOGNAME".into(), "fortlet-agent".into()),
        ("TMPDIR".into(), "/tmp".into()),
        (
            "NIX_CONFIG".into(),
            "experimental-features = nix-command flakes\naccept-flake-config = false\nflake-registry ="
                .into(),
        ),
    ]
}

async fn checked_exec(
    sandbox: &Sandbox,
    executable: &str,
    arguments: &[&str],
    environment: Option<Vec<(String, String)>>,
) -> Result<String> {
    let output = sandbox
        .exec_with(executable, |options| {
            let options = options.args(arguments.iter().copied());
            match environment {
                Some(environment) => options
                    .user(format!("{}:{}", effective_uid(), effective_gid()))
                    .envs(environment),
                None => options,
            }
        })
        .await
        .context("Nix provider command could not start")?;
    if !output.status().success {
        bail!(
            "Nix provider command exited {}; use a locked public scalar-only dev shell and retry",
            output.status().code
        );
    }
    Ok(output.stdout().unwrap_or_default())
}

async fn checked_exec_owned(
    sandbox: &Sandbox,
    executable: &str,
    arguments: &[String],
    environment: Vec<(String, String)>,
) -> Result<String> {
    let arguments = arguments.iter().map(String::as_str).collect::<Vec<_>>();
    checked_exec(sandbox, executable, &arguments, Some(environment)).await
}

fn parse_activation(
    value: &str,
    project: &Project,
) -> Result<(Vec<String>, BTreeMap<String, String>)> {
    let structured: StructuredEnvironment =
        serde_json::from_str(value).context("Nix structured activation is invalid")?;
    let _discarded_functions = structured.bash_functions;
    if structured.variables.len() > MAX_VARIABLES {
        bail!("Nix structured activation has too many variables");
    }
    let mut path = Vec::new();
    let mut environment = BTreeMap::new();
    for (name, variable) in structured.variables {
        if name == "shellHook" {
            if variable
                .value
                .as_str()
                .is_some_and(|value| !value.is_empty())
            {
                bail!("Nix dev shell requires executable shellHook activation; use schema 1");
            }
            continue;
        }
        if variable.kind != "exported" {
            continue;
        }
        let value = variable
            .value
            .as_str()
            .with_context(|| format!("exported Nix variable {name:?} is not scalar"))?;
        validate_environment_name(&name)?;
        if value.len() > MAX_VALUE_BYTES || value.contains('\0') {
            bail!("exported Nix variable {name:?} exceeds activation bounds");
        }
        if name == "PATH" {
            path = validate_provider_path(value)?;
            continue;
        }
        if protected_environment_name(&name, project)? {
            continue;
        }
        if value.contains("/Users/") || value.contains(&project.root.display().to_string()) {
            bail!("exported Nix variable {name:?} contains a host-only path");
        }
        environment.insert(name, value.to_owned());
    }
    if path.is_empty() {
        bail!("Nix dev shell did not export a usable store-only PATH");
    }
    Ok((path, environment))
}

fn validate_provider_path(value: &str) -> Result<Vec<String>> {
    let entries = value.split(':').map(str::to_owned).collect::<Vec<_>>();
    if entries.is_empty() || entries.len() > 256 {
        bail!("Nix dev-shell PATH exceeds activation bounds");
    }
    for entry in &entries {
        require_store_path(entry)?;
    }
    Ok(entries)
}

fn archived_source(value: &str) -> Result<String> {
    let value: serde_json::Value =
        serde_json::from_str(value).context("Nix archive result is invalid")?;
    let source = value
        .get("path")
        .and_then(serde_json::Value::as_str)
        .context("Nix archive did not return a source path")?;
    require_store_path(source)?;
    Ok(source.to_owned())
}

fn require_store_path(value: &str) -> Result<()> {
    let path = Path::new(value);
    if !path.is_absolute()
        || !path.starts_with("/nix/store")
        || path
            .components()
            .any(|component| matches!(component, std::path::Component::ParentDir))
    {
        bail!("Nix provider returned a path outside its project store");
    }
    Ok(())
}

fn verify_live_inputs(declaration: &NixProjectEnvironment<'_>) -> Result<()> {
    for (name, expected) in [
        ("flake.nix", declaration.flake),
        ("flake.lock", declaration.lock),
    ] {
        let actual = fs::read(declaration.project_root.join(name))
            .with_context(|| format!("cannot reread project {name}"))?;
        if actual != expected {
            bail!("project {name} changed after discovery; retry preparation");
        }
    }
    Ok(())
}

fn reject_authenticated_lock(bytes: &[u8]) -> Result<()> {
    let value: serde_json::Value =
        serde_json::from_slice(bytes).context("flake.lock is not valid JSON")?;
    fn visit(value: &serde_json::Value) -> bool {
        match value {
            serde_json::Value::String(value) => {
                let lower = value.to_ascii_lowercase();
                lower.contains("ssh://")
                    || lower.contains("git+ssh://")
                    || lower.starts_with("file:")
                    || lower.starts_with("path:")
                    || lower.contains("token=")
                    || lower.contains("access_token")
            }
            serde_json::Value::Array(values) => values.iter().any(visit),
            serde_json::Value::Object(values) => values.values().any(visit),
            _ => false,
        }
    }
    if visit(&value) {
        bail!("flake.lock requires a private, authenticated, or host-local input");
    }
    Ok(())
}

fn resolved_identity(record: &ResolvedRecord) -> String {
    let mut digest = Sha256::new();
    for field in [
        record.contract.as_bytes(),
        record.declaration_identity.as_bytes(),
        record.nix_version.as_bytes(),
        record.system.as_bytes(),
        record.source.as_bytes(),
        record.derivation.as_bytes(),
    ] {
        digest.update((field.len() as u64).to_be_bytes());
        digest.update(field);
    }
    for entry in &record.path {
        digest.update(entry.as_bytes());
    }
    for (name, value) in &record.environment {
        digest.update(name.as_bytes());
        digest.update(value.as_bytes());
    }
    hex::encode(digest.finalize())
}

fn write_record(path: &Path, record: &ResolvedRecord) -> Result<()> {
    let parent = path.parent().context("provider record has no parent")?;
    ensure_owned_directory(parent)?;
    let mut temporary = tempfile::NamedTempFile::new_in(parent)?;
    temporary
        .as_file()
        .set_permissions(fs::Permissions::from_mode(0o600))?;
    serde_json::to_writer(&mut temporary, record)?;
    temporary.write_all(b"\n")?;
    temporary.as_file().sync_all()?;
    temporary.persist(path).map_err(|error| error.error)?;
    Ok(())
}

fn verified_record(
    path: &Path,
    store: &Path,
    declaration_identity: &str,
) -> Result<Option<ResolvedRecord>> {
    let metadata = match fs::symlink_metadata(path) {
        Ok(metadata) => metadata,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(error) => return Err(error.into()),
    };
    if !metadata.is_file() || metadata.file_type().is_symlink() {
        bail!("resolved Nix project environment record is not a regular file");
    }
    let record: ResolvedRecord = serde_json::from_slice(&fs::read(path)?)
        .context("resolved Nix project environment record is invalid")?;
    if record.declaration_identity != declaration_identity {
        return Ok(None);
    }
    if resolved_identity(&record) != record.identity {
        bail!("resolved Nix project environment identity does not match its record");
    }
    if record.schema != RECORD_SCHEMA
        || record.contract != NIX_PROVIDER_CONTRACT
        || record.nix_version != NIX_VERSION
        || record.system != guest_system()?
    {
        bail!("resolved Nix project environment record does not match its provider contract");
    }
    let metadata =
        fs::symlink_metadata(store).context("resolved Nix project environment store is missing")?;
    if !metadata.is_dir() || metadata.file_type().is_symlink() {
        bail!("resolved Nix project environment store is not a real directory");
    }
    for entry in record
        .path
        .iter()
        .chain([&record.source, &record.derivation])
    {
        require_store_path(entry)?;
        let host = store.join(entry.trim_start_matches("/nix/"));
        if !host.exists() {
            bail!("resolved Nix project environment store is incomplete");
        }
    }
    Ok(Some(record))
}

fn published(record: ResolvedRecord, store: PathBuf) -> PublishedProjectEnvironment {
    PublishedProjectEnvironment {
        identity: record.identity,
        root: store,
        guest_root: GUEST_STORE.to_owned(),
        path: record.path,
        environment: record.environment,
    }
}

fn ensure_owned_directory(path: &Path) -> Result<()> {
    fs::create_dir_all(path)?;
    let metadata = fs::symlink_metadata(path)?;
    if !metadata.is_dir() || metadata.file_type().is_symlink() {
        bail!("Nix provider path is not a real directory");
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

fn nix_distribution() -> Result<(&'static str, &'static str)> {
    match std::env::consts::ARCH {
        "aarch64" => Ok((
            "https://releases.nixos.org/nix/nix-2.35.2/nix-2.35.2-aarch64-linux.tar.xz",
            "4d0302a2910f5eec1c33b8deef634f04899a75737e7001ec49908d003ae5efda",
        )),
        "x86_64" => Ok((
            "https://releases.nixos.org/nix/nix-2.35.2/nix-2.35.2-x86_64-linux.tar.xz",
            "0c3960a9792331a22081c3c7a5d8465db9b17c50b3acdf18587fa4c6f2cb1158",
        )),
        architecture => bail!("unsupported Nix provider architecture {architecture}"),
    }
}

async fn cleanup(sandbox: &Sandbox, name: &str) {
    let _ = sandbox.stop_and_wait().await;
    let _ = Sandbox::remove(name).await;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::project::ProjectIdentity;

    fn project(root: &Path) -> Project {
        Project {
            identity: ProjectIdentity::from_local_root(root),
            root: root.to_owned(),
            cwd: root.to_owned(),
            kind: "test",
            scratch: false,
        }
    }

    #[test]
    fn structured_activation_keeps_exported_scalars_and_discards_inert_metadata() {
        let root = Path::new("/tmp/public-project");
        let value = serde_json::json!({
            "variables": {
                "PATH": {"type": "exported", "value": "/nix/store/abc-tool/bin"},
                "GOFLAGS": {"type": "exported", "value": "-mod=readonly"},
                "HOME": {"type": "exported", "value": "/homeless-shelter"},
                "internal": {"type": "var", "value": "discarded"},
                "hooks": {"type": "array", "value": ["discarded"]}
            },
            "bashFunctions": {"stdenvHook": "discarded"}
        });

        let (path, environment) = parse_activation(&value.to_string(), &project(root)).unwrap();

        assert_eq!(path, vec!["/nix/store/abc-tool/bin"]);
        assert_eq!(
            environment.get("GOFLAGS").map(String::as_str),
            Some("-mod=readonly")
        );
        assert!(!environment.contains_key("HOME"));
        assert!(!environment.contains_key("internal"));
    }

    #[test]
    fn executable_activation_and_non_store_paths_fail_closed() {
        for value in [
            serde_json::json!({
                "variables": {
                    "PATH": {"type": "exported", "value": "/usr/bin"}
                },
                "bashFunctions": {}
            }),
            serde_json::json!({
                "variables": {
                    "PATH": {"type": "exported", "value": "/nix/store/abc/bin"},
                    "shellHook": {"type": "exported", "value": "run something"}
                },
                "bashFunctions": {}
            }),
        ] {
            assert!(
                parse_activation(&value.to_string(), &project(Path::new("/tmp/project"))).is_err()
            );
        }
    }

    #[test]
    fn distribution_is_versioned_and_digest_pinned() {
        let (url, digest) = nix_distribution().unwrap();
        assert!(url.contains(NIX_VERSION));
        assert_eq!(digest.len(), 64);
    }
}
