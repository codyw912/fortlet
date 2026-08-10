use std::env;
use std::fs::{self, File};
use std::io::{IsTerminal, Write};
use std::path::{Path, PathBuf};

use anyhow::{bail, Context, Result};
use fs2::FileExt;
use microsandbox::sandbox::SandboxStatus;
use microsandbox::{MicrosandboxError, Sandbox, SecretSource};

use crate::auth::{Credentials, ACCESS_TOKEN_ENV, ACCOUNT_ID_ENV};
use crate::environment::{EnvironmentLayers, BASE_IMAGE};
use crate::harness::Harness;
use crate::paths::{AppPaths, PRODUCT};
use crate::project::Project;

const SCHEMA_VERSION: &str = "1";
const DEFAULT_IDLE_TIMEOUT_SECS: u64 = 4 * 60 * 60;
const HOLD_SCRIPT: &str = "trap 'exit 0' TERM INT; while :; do sleep 3600 & wait $!; done";
const SECRET_HOSTS: &[&str] = &["chatgpt.com", "*.chatgpt.com", "openai.com", "*.openai.com"];
const TERMINAL_ENVIRONMENT: &[&str] = &[
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

pub struct Capsule<'a> {
    pub name: String,
    pub state: PathBuf,
    pub project: &'a Project,
    pub harness: &'a dyn Harness,
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
            name: format!(
                "fortlet-{}-{}-{}",
                effective_uid(),
                harness.name(),
                project.identity.as_str()
            ),
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
        let _lock = lock(
            &self
                .paths
                .locks()
                .join(format!("capsule-{}.lock", capsule.name)),
        )?;
        credentials.expose_to_broker();
        match Sandbox::get(&capsule.name).await {
            Ok(handle) => {
                validate_labels(capsule, &handle.config()?.spec.labels)?;
                let fingerprint = credentials.fingerprint();
                let fingerprint_changed =
                    read_fingerprint(capsule).as_deref() != Some(fingerprint.as_str());
                let running = matches!(
                    handle.status_snapshot(),
                    SandboxStatus::Running | SandboxStatus::Draining
                );
                let sandbox = if running && fingerprint_changed {
                    handle.stop().await?;
                    let sandbox = handle.start_detached().await?;
                    write_fingerprint(capsule, credentials)?;
                    sandbox
                } else if running {
                    handle.connect().await?
                } else {
                    let sandbox = handle.start_detached().await?;
                    write_fingerprint(capsule, credentials)?;
                    sandbox
                };
                sandbox.touch().await?;
                Ok(sandbox)
            }
            Err(MicrosandboxError::SandboxNotFound(_)) => {
                let sandbox = create_capsule(capsule, layers).await?;
                write_fingerprint(capsule, credentials)?;
                Ok(sandbox)
            }
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

async fn create_capsule(capsule: &Capsule<'_>, layers: &EnvironmentLayers) -> Result<Sandbox> {
    let guest_home = "/home/agent";
    let project_path = capsule.project.root.display().to_string();
    let mut builder = Sandbox::builder(&capsule.name)
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
        .env(
            "PATH",
            format!(
                "/opt/{PRODUCT}/base/usr/bin:/opt/{PRODUCT}/tool/bin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin"
            ),
        )
        .script("hold", HOLD_SCRIPT)
        .entrypoint(["hold"])
        .label(format!("{PRODUCT}.managed"), "true")
        .label(format!("{PRODUCT}.schema"), SCHEMA_VERSION)
        .label(
            format!("{PRODUCT}.project"),
            capsule.project.identity.as_str(),
        )
        .label(format!("{PRODUCT}.tool"), capsule.harness.name())
        .label(format!("{PRODUCT}.version"), capsule.harness.version());

    for (key, value) in capsule.harness.environment(capsule.project) {
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
        .with_context(|| format!("cannot create capsule {}", capsule.name))
}

fn validate_labels(
    capsule: &Capsule<'_>,
    labels: &std::collections::BTreeMap<String, String>,
) -> Result<()> {
    let expected = [
        (format!("{PRODUCT}.managed"), "true"),
        (format!("{PRODUCT}.schema"), SCHEMA_VERSION),
        (
            format!("{PRODUCT}.project"),
            capsule.project.identity.as_str(),
        ),
        (format!("{PRODUCT}.tool"), capsule.harness.name()),
        (format!("{PRODUCT}.version"), capsule.harness.version()),
    ];
    if expected
        .iter()
        .any(|(key, value)| labels.get(key).map(String::as_str) != Some(*value))
    {
        bail!(
            "capsule {} has stale configuration; remove it with msb and retry",
            capsule.name
        );
    }
    Ok(())
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

fn read_fingerprint(capsule: &Capsule<'_>) -> Option<String> {
    fs::read_to_string(capsule.state.join(".fortlet-credential-fingerprint"))
        .ok()
        .map(|value| value.trim().to_owned())
}

fn write_fingerprint(capsule: &Capsule<'_>, credentials: &Credentials) -> Result<()> {
    let destination = capsule.state.join(".fortlet-credential-fingerprint");
    let temporary = destination.with_extension(format!("tmp-{}", std::process::id()));
    fs::write(&temporary, format!("{}\n", credentials.fingerprint()))?;
    fs::rename(temporary, destination)?;
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

#[cfg(unix)]
fn effective_uid() -> u32 {
    unsafe { libc::geteuid() }
}

#[cfg(unix)]
fn effective_gid() -> u32 {
    unsafe { libc::getegid() }
}
