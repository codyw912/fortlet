use std::collections::BTreeMap;
use std::env;
use std::fs::{self, File};
use std::io::{IsTerminal, Write};
use std::path::{Path, PathBuf};

use anyhow::{bail, Context, Result};
use fs2::FileExt;
use microsandbox::sandbox::SandboxStatus;
use microsandbox::{
    MicrosandboxError, ModificationDisposition, PlannedChange, Sandbox, SecretSource,
};

use crate::auth::{Credentials, ACCESS_TOKEN_ENV, ACCOUNT_ID_ENV};
use crate::environment::{EnvironmentLayers, BASE_IMAGE};
use crate::harness::Harness;
use crate::paths::{AppPaths, PRODUCT};
use crate::project::Project;
use crate::project_environment::{
    ProjectEnvironment, GUEST_ROOT, NO_ENVIRONMENT, TERMINAL_ENVIRONMENT,
};

const SCHEMA_VERSION: &str = "1";
const DEFAULT_IDLE_TIMEOUT_SECS: u64 = 4 * 60 * 60;
const HOLD_SCRIPT: &str = "trap 'exit 0' TERM INT; while :; do sleep 3600 & wait $!; done";
const SECRET_HOSTS: &[&str] = &["chatgpt.com", "*.chatgpt.com", "openai.com", "*.openai.com"];
pub struct Capsule<'a> {
    pub descriptor: CapsuleDescriptor,
    pub state: PathBuf,
    pub project: &'a Project,
    pub harness: &'a dyn Harness,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CapsuleDescriptor {
    pub name: String,
    tool: String,
    labels: BTreeMap<String, String>,
}

impl CapsuleDescriptor {
    pub fn new(project: &Project, harness: &dyn Harness) -> Self {
        let labels = [
            (label("managed"), "true".to_owned()),
            (label("schema"), SCHEMA_VERSION.to_owned()),
            (label("project"), project.identity.as_str().to_owned()),
            (label("tool"), harness.name().to_owned()),
            (label("version"), harness.version().to_owned()),
        ]
        .into_iter()
        .collect();
        Self {
            name: format!(
                "fortlet-{}-{}-{}",
                effective_uid(),
                harness.name(),
                project.identity.as_str()
            ),
            tool: harness.name().to_owned(),
            labels,
        }
    }

    fn for_launch(
        project: &Project,
        harness: &dyn Harness,
        environment: Option<&ProjectEnvironment>,
    ) -> Self {
        let mut descriptor = Self::new(project, harness);
        descriptor.labels.insert(
            label("environment"),
            environment
                .map(ProjectEnvironment::identity)
                .unwrap_or(NO_ENVIRONMENT)
                .to_owned(),
        );
        descriptor
    }

    fn labels(&self) -> BTreeMap<String, String> {
        self.labels.clone()
    }

    pub fn validate_management(
        &self,
        stored_name: &str,
        labels: &BTreeMap<String, String>,
    ) -> Result<()> {
        let identity_keys = [
            label("managed"),
            label("schema"),
            label("project"),
            label("tool"),
        ];
        let identity_matches = stored_name == self.name
            && identity_keys
                .iter()
                .all(|key| labels.get(key) == self.labels.get(key));
        if !identity_matches {
            bail!(
                "expected capsule is not owned by Fortlet for this project and harness; inspect it with msb and retry"
            );
        }
        Ok(())
    }

    fn validate_launch(&self, stored_name: &str, labels: &BTreeMap<String, String>) -> Result<()> {
        self.validate_management(stored_name, labels)?;
        let stored_environment = labels
            .get(&label("environment"))
            .map(String::as_str)
            .unwrap_or(NO_ENVIRONMENT);
        let expected_environment = self
            .labels
            .get(&label("environment"))
            .map(String::as_str)
            .unwrap_or(NO_ENVIRONMENT);
        if labels.get(&label("version")) != self.labels.get(&label("version"))
            || stored_environment != expected_environment
        {
            bail!(
                "capsule has stale configuration; run `fortlet stop {}` followed by `fortlet reset {}` and retry",
                self.tool,
                self.tool
            );
        }
        Ok(())
    }
}

pub struct MicroSandboxRuntime<'a> {
    paths: &'a AppPaths,
}

impl<'a> MicroSandboxRuntime<'a> {
    pub fn new(paths: &'a AppPaths) -> Self {
        Self { paths }
    }

    pub fn capsule<'b>(
        &self,
        project: &'b Project,
        harness: &'b dyn Harness,
        environment: Option<&ProjectEnvironment>,
    ) -> Result<Capsule<'b>> {
        let state = self
            .paths
            .state
            .join("projects")
            .join(project.identity.as_str())
            .join(harness.name());
        fs::create_dir_all(&state)
            .with_context(|| format!("cannot create harness state {}", state.display()))?;
        let state = state.canonicalize()?;
        Ok(Capsule {
            descriptor: CapsuleDescriptor::for_launch(project, harness, environment),
            state,
            project,
            harness,
        })
    }

    pub async fn ensure(
        &self,
        capsule: &Capsule<'_>,
        layers: &EnvironmentLayers,
        credentials: &Credentials,
    ) -> Result<Sandbox> {
        let _lock = lock_capsule(self.paths, &capsule.descriptor)?;
        credentials.expose_to_broker();
        match Sandbox::get(&capsule.descriptor.name).await {
            Ok(handle) => {
                let config = handle.config()?;
                capsule
                    .descriptor
                    .validate_launch(&config.spec.name, &config.spec.labels)?;
                let running = matches!(
                    handle.status_snapshot(),
                    SandboxStatus::Running | SandboxStatus::Draining
                );
                let sandbox = if running {
                    let sandbox = handle.connect().await?;
                    rotate_credentials(&sandbox, credentials).await?;
                    sandbox
                } else {
                    handle.start_detached().await?
                };
                sandbox.touch().await?;
                Ok(sandbox)
            }
            Err(MicrosandboxError::SandboxNotFound(_)) => create_capsule(capsule, layers).await,
            Err(error) => Err(error.into()),
        }
    }

    pub async fn attach(
        &self,
        capsule: &Capsule<'_>,
        sandbox: &Sandbox,
        args: &[String],
    ) -> Result<i32> {
        if !std::io::stdin().is_terminal() || !std::io::stdout().is_terminal() {
            let output = sandbox
                .exec_with(tool_executable(capsule), |options| {
                    options
                        .args(args.iter().cloned())
                        .cwd(capsule.project.cwd.display().to_string())
                })
                .await?;
            std::io::stdout().write_all(output.stdout_bytes())?;
            std::io::stderr().write_all(output.stderr_bytes())?;
            return Ok(output.status().code);
        }

        let terminal = terminal_environment();
        sandbox
            .attach_with(tool_executable(capsule), |options| {
                options
                    .args(args.iter().cloned())
                    .cwd(capsule.project.cwd.display().to_string())
                    .envs(terminal)
            })
            .await
            .map_err(Into::into)
    }
}

pub async fn rotate_credentials(sandbox: &Sandbox, credentials: &Credentials) -> Result<()> {
    credentials.expose_to_broker();
    let plan = sandbox
        .modify()
        .secret(|secret| {
            SECRET_HOSTS.iter().fold(
                secret.env(ACCESS_TOKEN_ENV).source(SecretSource::Env {
                    var: ACCESS_TOKEN_ENV.into(),
                }),
                |secret, host| secret.allow_host(*host),
            )
        })
        .secret(|secret| {
            SECRET_HOSTS.iter().fold(
                secret.env(ACCOUNT_ID_ENV).source(SecretSource::Env {
                    var: ACCOUNT_ID_ENV.into(),
                }),
                |secret, host| secret.allow_host(*host),
            )
        })
        .apply()
        .await?;
    if !plan.applied {
        bail!("credential rotation was planned but not applied");
    }
    validate_live_rotation(&plan.changes)?;
    Ok(())
}

fn validate_live_rotation(changes: &[PlannedChange]) -> Result<()> {
    let mut rotated = 0;
    for change in changes {
        let PlannedChange::Secret(secret) = change else {
            continue;
        };
        if secret.name != ACCESS_TOKEN_ENV && secret.name != ACCOUNT_ID_ENV {
            continue;
        }
        rotated += 1;
        if secret.disposition != ModificationDisposition::Live {
            bail!(
                "credential rotation for {} was not applied live",
                secret.name
            );
        }
    }
    if rotated != 2 {
        bail!("credential rotation did not update both broker secrets live");
    }
    Ok(())
}

async fn create_capsule(capsule: &Capsule<'_>, layers: &EnvironmentLayers) -> Result<Sandbox> {
    let guest_home = "/home/agent";
    let project_path = capsule.project.root.display().to_string();
    let mut builder = Sandbox::builder(&capsule.descriptor.name)
        .image(BASE_IMAGE)
        .cpus(4)
        .memory(8192)
        .root_disk(8192)
        .detached(true)
        .idle_timeout(DEFAULT_IDLE_TIMEOUT_SECS)
        .user(format!("{}:{}", effective_uid(), effective_gid()))
        .workdir(capsule.project.cwd.display().to_string())
        .volume(&project_path, |mount| mount.bind(&capsule.project.root))
        .volume(guest_home, |mount| mount.bind(&capsule.state))
        .volume(format!("/opt/{PRODUCT}/tool"), |mount| {
            mount.bind(&layers.harness).readonly()
        })
        .volume(format!("/opt/{PRODUCT}/base"), |mount| {
            mount.bind(&layers.base).readonly()
        })
        .env("HOME", guest_home)
        .env("PATH", guest_path(layers))
        .script("hold", HOLD_SCRIPT)
        .entrypoint(["hold"])
        .labels(capsule.descriptor.labels());

    if let Some(project) = &layers.project {
        builder = builder.volume(GUEST_ROOT, |mount| mount.bind(&project.root).readonly());
    }
    for (key, value) in capsule_environment(capsule, layers) {
        builder = builder.env(key, value);
    }
    builder = builder
        .secret(|secret| {
            SECRET_HOSTS.iter().fold(
                secret.env(ACCESS_TOKEN_ENV).source(SecretSource::Env {
                    var: ACCESS_TOKEN_ENV.into(),
                }),
                |secret, host| secret.allow_host(*host),
            )
        })
        .secret(|secret| {
            SECRET_HOSTS.iter().fold(
                secret.env(ACCOUNT_ID_ENV).source(SecretSource::Env {
                    var: ACCOUNT_ID_ENV.into(),
                }),
                |secret, host| secret.allow_host(*host),
            )
        });
    builder
        .create_detached()
        .await
        .with_context(|| format!("cannot create capsule {}", capsule.descriptor.name))
}

fn capsule_environment(capsule: &Capsule<'_>, layers: &EnvironmentLayers) -> Vec<(String, String)> {
    let mut environment = layers
        .project
        .as_ref()
        .map(|project| {
            project
                .environment
                .iter()
                .map(|(key, value)| (key.clone(), value.clone()))
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    environment.extend(capsule.harness.environment(capsule.project));
    environment
}

fn guest_path(layers: &EnvironmentLayers) -> String {
    let mut entries = layers
        .project
        .as_ref()
        .map(|project| {
            project
                .path
                .iter()
                .map(|entry| format!("{GUEST_ROOT}/{entry}"))
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    entries.extend([
        format!("/opt/{PRODUCT}/base/usr/bin"),
        format!("/opt/{PRODUCT}/tool/bin"),
        "/usr/local/sbin".into(),
        "/usr/local/bin".into(),
        "/usr/sbin".into(),
        "/usr/bin".into(),
        "/sbin".into(),
        "/bin".into(),
    ]);
    entries.join(":")
}

fn label(suffix: &str) -> String {
    format!("{PRODUCT}.{suffix}")
}

fn tool_executable(capsule: &Capsule<'_>) -> String {
    format!("/opt/{PRODUCT}/tool/bin/{}", capsule.harness.executable())
}

fn terminal_environment() -> Vec<(String, String)> {
    TERMINAL_ENVIRONMENT
        .iter()
        .filter_map(|key| env::var(key).ok().map(|value| ((*key).into(), value)))
        .collect()
}

fn lock(path: &Path) -> Result<File> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let file = File::options().create(true).append(true).open(path)?;
    file.lock_exclusive()?;
    Ok(file)
}

pub fn lock_capsule(paths: &AppPaths, descriptor: &CapsuleDescriptor) -> Result<File> {
    lock(
        &paths
            .locks()
            .join(format!("capsule-{}.lock", descriptor.name)),
    )
}

#[cfg(unix)]
fn effective_uid() -> u32 {
    unsafe { libc::geteuid() }
}

#[cfg(unix)]
fn effective_gid() -> u32 {
    unsafe { libc::getegid() }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::harness;
    use crate::project::ProjectIdentity;
    use crate::project_environment::PublishedProjectEnvironment;
    use microsandbox::{SecretChangeKind, SecretPlannedChange};

    fn project() -> Project {
        Project {
            identity: ProjectIdentity::from_local_root(Path::new("/tmp/fortlet-descriptor-test")),
            root: "/tmp/fortlet-descriptor-test".into(),
            cwd: "/tmp/fortlet-descriptor-test".into(),
            kind: "test",
            scratch: false,
        }
    }

    #[test]
    fn descriptor_is_shared_identity_and_complete_creation_metadata() {
        let project = project();
        let harness = harness::find("codex").unwrap();
        let descriptor = CapsuleDescriptor::new(&project, harness);

        assert_eq!(
            descriptor.name,
            format!(
                "fortlet-{}-codex-{}",
                effective_uid(),
                project.identity.as_str()
            )
        );
        assert_eq!(descriptor.labels.get(&label("managed")).unwrap(), "true");
        assert_eq!(
            descriptor.labels.get(&label("version")).unwrap(),
            harness.version()
        );
    }

    #[test]
    fn management_accepts_version_skew_but_launch_does_not() {
        let descriptor = CapsuleDescriptor::new(&project(), harness::find("codex").unwrap());
        let mut labels = descriptor.labels();
        labels.insert(label("version"), "older-release".into());

        descriptor
            .validate_management(&descriptor.name, &labels)
            .unwrap();
        let error = descriptor
            .validate_launch(&descriptor.name, &labels)
            .unwrap_err();
        assert_eq!(
            error.to_string(),
            "capsule has stale configuration; run `fortlet stop codex` followed by `fortlet reset codex` and retry"
        );
        assert!(!error.to_string().contains(&descriptor.name));
    }

    #[test]
    fn management_rejects_name_and_ownership_mismatches() {
        let descriptor = CapsuleDescriptor::new(&project(), harness::find("codex").unwrap());
        assert!(descriptor
            .validate_management("fortlet-collision", &descriptor.labels())
            .is_err());

        for key in ["managed", "schema", "project", "tool"] {
            let mut labels = descriptor.labels();
            labels.insert(label(key), "wrong".into());
            assert!(
                descriptor
                    .validate_management(&descriptor.name, &labels)
                    .is_err(),
                "{key}"
            );
        }
    }

    #[test]
    fn launch_identity_tracks_project_environment_but_management_does_not() {
        let project = project();
        let harness = harness::find("codex").unwrap();
        let mut configured = CapsuleDescriptor::new(&project, harness);
        configured
            .labels
            .insert(label("environment"), "configured".into());
        let legacy_labels = CapsuleDescriptor::new(&project, harness).labels();

        CapsuleDescriptor::for_launch(&project, harness, None)
            .validate_launch(&configured.name, &legacy_labels)
            .unwrap();
        configured
            .validate_management(&configured.name, &legacy_labels)
            .unwrap();
        assert!(configured
            .validate_launch(&configured.name, &legacy_labels)
            .unwrap_err()
            .to_string()
            .contains("stale configuration"));
    }

    #[test]
    fn project_paths_and_variables_reach_both_harnesses() {
        let project = project();
        let published = PublishedProjectEnvironment {
            identity: "environment-id".into(),
            root: "/project-layer".into(),
            path: vec!["cargo/bin".into(), "jj/bin".into()],
            environment: BTreeMap::from([("RUST_BACKTRACE".into(), "1".into())]),
        };
        let layers = EnvironmentLayers {
            harness: "/harness-layer".into(),
            base: "/base-layer".into(),
            project: Some(published),
        };

        assert!(guest_path(&layers)
            .starts_with("/opt/fortlet/project/cargo/bin:/opt/fortlet/project/jj/bin:"));
        for harness_name in ["codex", "tact"] {
            let harness = harness::find(harness_name).unwrap();
            let capsule = Capsule {
                descriptor: CapsuleDescriptor::new(&project, harness),
                state: "/state".into(),
                project: &project,
                harness,
            };
            let environment = capsule_environment(&capsule, &layers);
            assert!(environment.contains(&("RUST_BACKTRACE".into(), "1".into())));
            for (key, _) in harness.environment(&project) {
                assert!(environment.iter().any(|(candidate, _)| candidate == &key));
            }
        }
    }

    fn secret_change(name: &str, disposition: ModificationDisposition) -> PlannedChange {
        PlannedChange::Secret(SecretPlannedChange {
            field: "secret".into(),
            name: name.into(),
            change: SecretChangeKind::Rotated,
            before_ref: Some(format!("$MSB_{name}")),
            after_ref: Some(format!("$MSB_{name}")),
            disposition,
            allow_hosts: SECRET_HOSTS.iter().map(|host| (*host).into()).collect(),
            reason: None,
        })
    }

    #[test]
    fn credential_rotation_requires_both_changes_to_be_live() {
        let live = vec![
            secret_change(ACCESS_TOKEN_ENV, ModificationDisposition::Live),
            secret_change(ACCOUNT_ID_ENV, ModificationDisposition::Live),
        ];
        validate_live_rotation(&live).unwrap();

        let restart = vec![
            secret_change(ACCESS_TOKEN_ENV, ModificationDisposition::RequiresRestart),
            secret_change(ACCOUNT_ID_ENV, ModificationDisposition::Live),
        ];
        assert!(validate_live_rotation(&restart)
            .unwrap_err()
            .to_string()
            .contains("was not applied live"));

        assert!(validate_live_rotation(&live[..1])
            .unwrap_err()
            .to_string()
            .contains("both broker secrets"));
    }
}
