use std::collections::BTreeMap;
use std::env;
use std::fs::{self, File};
use std::io::{IsTerminal, Write};
use std::path::{Path, PathBuf};
use std::time::Duration;

use anyhow::{bail, Context, Result};
use fs2::FileExt;
use microsandbox::sandbox::{PullPolicy, SandboxStatus};
use microsandbox::{
    ExecEvent, ExecHandle, MicrosandboxError, ModificationDisposition, PlannedChange, Sandbox,
    SecretSource,
};

use crate::auth::{Credentials, ACCESS_TOKEN_ENV, ACCOUNT_ID_ENV};
use crate::environment::EnvironmentLayers;
use crate::harness::Harness;
use crate::identity::{
    IdentityProjection, GIT_CONFIG_GLOBAL, GIT_CONFIG_NOSYSTEM, GUEST_IDENTITY_ROOT, JJ_CONFIG,
};
use crate::paths::{AppPaths, PRODUCT};
use crate::project::Project;
use crate::project_environment::{
    PublishedProjectEnvironment, NO_ENVIRONMENT, TERMINAL_ENVIRONMENT,
};

const SCHEMA_VERSION: &str = "1";
const DEFAULT_IDLE_TIMEOUT_SECS: u64 = 4 * 60 * 60;
const EXEC_KILL_CLEANUP_TIMEOUT: Duration = Duration::from_secs(5);
const MANAGED_PATH_ENV: &str = "FORTLET_MANAGED_PATH";
const SECRET_HOSTS: &[&str] = &["chatgpt.com", "*.chatgpt.com", "openai.com", "*.openai.com"];
pub struct Capsule<'a> {
    pub descriptor: CapsuleDescriptor,
    pub state: PathBuf,
    pub project: &'a Project,
    pub harness: &'a dyn Harness,
    pub identity: Option<&'a IdentityProjection>,
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
            name: capsule_name(harness.name(), &project.identity),
            tool: harness.name().to_owned(),
            labels,
        }
    }

    fn for_launch(
        project: &Project,
        harness: &dyn Harness,
        environment: Option<&PublishedProjectEnvironment>,
        identity: Option<&IdentityProjection>,
        runtime: Option<&crate::runtime_artifacts::PreparedRuntime>,
    ) -> Self {
        let mut descriptor = Self::new(project, harness);
        descriptor.labels.insert(
            label("environment"),
            environment
                .map(|environment| environment.identity.as_str())
                .unwrap_or(NO_ENVIRONMENT)
                .to_owned(),
        );
        descriptor.labels.insert(
            label("identity"),
            identity
                .map(|projection| projection.identity.as_str())
                .unwrap_or("none")
                .to_owned(),
        );
        descriptor.labels.insert(
            label("runtime"),
            runtime
                .map(|runtime| runtime.runtime_identity.as_str())
                .unwrap_or("legacy")
                .to_owned(),
        );
        descriptor.labels.insert(
            label("harness-closure"),
            runtime
                .map(|runtime| runtime.harness_identity.as_str())
                .unwrap_or("legacy")
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

    pub fn validate_inventory(
        stored_name: &str,
        labels: &BTreeMap<String, String>,
        project_root: &Path,
    ) -> Result<String> {
        let required = |suffix| {
            labels
                .get(&label(suffix))
                .with_context(|| format!("stored capsule is missing fortlet.{suffix}"))
        };
        if required("managed")? != "true" {
            bail!("stored capsule is not marked as Fortlet-managed");
        }
        if required("schema")? != SCHEMA_VERSION {
            bail!("stored capsule uses an unsupported Fortlet schema");
        }

        let tool = required("tool")?;
        if tool.is_empty()
            || tool.len() > 64
            || !tool
                .bytes()
                .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-')
        {
            bail!("stored capsule has an invalid harness name");
        }

        let identity = crate::project::ProjectIdentity::from_local_root(project_root);
        if required("project")? != identity.as_str() {
            bail!("stored capsule project path does not match its identity");
        }
        let expected_name = capsule_name(tool, &identity);
        if stored_name != expected_name {
            bail!("stored capsule name does not match its ownership labels");
        }
        Ok(tool.clone())
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
        let stored_identity = labels
            .get(&label("identity"))
            .map(String::as_str)
            .unwrap_or("none");
        let expected_identity = self
            .labels
            .get(&label("identity"))
            .map(String::as_str)
            .unwrap_or("none");
        let stored_runtime = labels
            .get(&label("runtime"))
            .map(String::as_str)
            .unwrap_or("legacy");
        let expected_runtime = self
            .labels
            .get(&label("runtime"))
            .map(String::as_str)
            .unwrap_or("legacy");
        let stored_harness_closure = labels
            .get(&label("harness-closure"))
            .map(String::as_str)
            .unwrap_or("legacy");
        let expected_harness_closure = self
            .labels
            .get(&label("harness-closure"))
            .map(String::as_str)
            .unwrap_or("legacy");
        if labels.get(&label("version")) != self.labels.get(&label("version"))
            || stored_environment != expected_environment
            || stored_identity != expected_identity
            || stored_runtime != expected_runtime
            || stored_harness_closure != expected_harness_closure
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

fn capsule_name(tool: &str, identity: &crate::project::ProjectIdentity) -> String {
    format!("fortlet-{}-{tool}-{}", effective_uid(), identity.as_str())
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
        environment: Option<&PublishedProjectEnvironment>,
        identity: Option<&'b IdentityProjection>,
        layers: &EnvironmentLayers,
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
            descriptor: CapsuleDescriptor::for_launch(
                project,
                harness,
                environment,
                identity,
                Some(&layers.runtime),
            ),
            state,
            project,
            harness,
            identity,
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
        layers: &EnvironmentLayers,
        requested_arguments: &[String],
    ) -> Result<i32> {
        let arguments = effective_launch_arguments(capsule.harness, requested_arguments);
        if !std::io::stdin().is_terminal() || !std::io::stdout().is_terminal() {
            let mut session = sandbox
                .exec_stream_with(tool_executable(layers), |options| {
                    options
                        .args(arguments.iter().cloned())
                        .cwd(capsule.project.cwd.display().to_string())
                        // MicroSandbox 0.6.8 streaming does not send EOF for
                        // StdinMode::Null. Close an explicit pipe below so
                        // non-terminal harnesses cannot block reading stdin.
                        .stdin_pipe()
                })
                .await?;
            return forward_non_interactive(
                &mut session,
                &mut std::io::stdout(),
                &mut std::io::stderr(),
                capsule.harness.non_interactive_idle_timeout(),
            )
            .await;
        }

        let terminal = terminal_environment();
        sandbox
            .attach_with(tool_executable(layers), |options| {
                options
                    .args(arguments.iter().cloned())
                    .cwd(capsule.project.cwd.display().to_string())
                    .envs(terminal)
            })
            .await
            .map_err(Into::into)
    }
}

enum ProcessEvent {
    Started,
    Stdout(Vec<u8>),
    Stderr(Vec<u8>),
    Exited(i32),
}

trait ExecutionSession {
    async fn close_stdin(&mut self) -> Result<()>;
    async fn next_event(&mut self) -> Result<Option<ProcessEvent>>;
    async fn kill(&self) -> Result<()>;
}

impl ExecutionSession for ExecHandle {
    async fn close_stdin(&mut self) -> Result<()> {
        self.take_stdin()
            .context("non-interactive exec has no stdin pipe")?
            .close()
            .await
            .context("cannot close non-interactive command stdin")
    }

    async fn next_event(&mut self) -> Result<Option<ProcessEvent>> {
        loop {
            let event = match self.recv().await {
                Some(ExecEvent::Started { .. }) => ProcessEvent::Started,
                Some(ExecEvent::Stdout(bytes)) => ProcessEvent::Stdout(bytes.to_vec()),
                Some(ExecEvent::Stderr(bytes)) => ProcessEvent::Stderr(bytes.to_vec()),
                Some(ExecEvent::Exited { code }) => ProcessEvent::Exited(code),
                Some(ExecEvent::Failed(failure)) => {
                    return Err(MicrosandboxError::ExecFailed(failure).into());
                }
                Some(ExecEvent::StdinError(_)) => continue,
                None => return Ok(None),
            };
            return Ok(Some(event));
        }
    }

    async fn kill(&self) -> Result<()> {
        ExecHandle::kill(self).await.map_err(Into::into)
    }
}

async fn forward_non_interactive(
    session: &mut impl ExecutionSession,
    stdout: &mut impl Write,
    stderr: &mut impl Write,
    idle_timeout: Option<Duration>,
) -> Result<i32> {
    session.close_stdin().await?;
    loop {
        let event = match idle_timeout {
            Some(duration) => match tokio::time::timeout(duration, session.next_event()).await {
                Ok(event) => event?,
                Err(_) => {
                    terminate_inactive_session(session, stdout, stderr).await?;
                    bail!(
                        "non-interactive command exceeded its {}-second inactivity ceiling",
                        duration.as_secs_f64()
                    );
                }
            },
            None => session.next_event().await?,
        };
        let Some(event) = event else {
            bail!("exec session ended without exit event");
        };
        if let Some(code) = forward_process_event(event, stdout, stderr)? {
            return Ok(code);
        }
    }
}

async fn terminate_inactive_session(
    session: &mut impl ExecutionSession,
    stdout: &mut impl Write,
    stderr: &mut impl Write,
) -> Result<()> {
    session
        .kill()
        .await
        .context("cannot kill inactive command")?;
    tokio::time::timeout(EXEC_KILL_CLEANUP_TIMEOUT, async {
        loop {
            let Some(event) = session.next_event().await? else {
                bail!("exec session ended without exit event after kill");
            };
            if forward_process_event(event, stdout, stderr)?.is_some() {
                return Ok(());
            }
        }
    })
    .await
    .context("inactive command did not exit after kill")?
}

fn forward_process_event(
    event: ProcessEvent,
    stdout: &mut impl Write,
    stderr: &mut impl Write,
) -> Result<Option<i32>> {
    match event {
        ProcessEvent::Started => Ok(None),
        ProcessEvent::Stdout(bytes) => {
            stdout.write_all(&bytes)?;
            stdout.flush()?;
            Ok(None)
        }
        ProcessEvent::Stderr(bytes) => {
            stderr.write_all(&bytes)?;
            stderr.flush()?;
            Ok(None)
        }
        ProcessEvent::Exited(code) => Ok(Some(code)),
    }
}

fn effective_launch_arguments(
    harness: &dyn Harness,
    requested_arguments: &[String],
) -> Vec<String> {
    harness.launch_arguments(requested_arguments)
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
    let (passwd, group) = guest_accounts(capsule, layers)?;
    let mut builder = Sandbox::builder(&capsule.descriptor.name)
        .image(layers.runtime.image_reference.clone())
        .pull_policy(PullPolicy::Never)
        .cpus(4)
        .memory(8192)
        .root_disk(8192)
        .detached(true)
        .idle_timeout(DEFAULT_IDLE_TIMEOUT_SECS)
        .user(format!("{}:{}", effective_uid(), effective_gid()))
        .workdir(capsule.project.cwd.display().to_string())
        .volume(&project_path, |mount| mount.bind(&capsule.project.root))
        .volume(guest_home, |mount| mount.bind(&capsule.state))
        .volume("/etc/passwd", |mount| mount.bind(&passwd).readonly())
        .volume("/etc/group", |mount| mount.bind(&group).readonly())
        .volume("/nix", |mount| {
            mount.named(layers.runtime.store_volume.clone()).readonly()
        })
        .env("HOME", guest_home)
        .entrypoint([layers.runtime.hold.clone()])
        .labels(capsule.descriptor.labels());

    if let Some(project) = &layers.project {
        if project.guest_root != "/nix" {
            builder = builder.volume(&project.guest_root, |mount| {
                mount.bind(&project.root).readonly()
            });
        }
    }
    if let Some(identity) = capsule.identity {
        builder = builder.volume(GUEST_IDENTITY_ROOT, |mount| {
            mount.bind(&identity.root).readonly()
        });
    }
    for (key, value) in capsule_environment(capsule, layers) {
        builder = builder.env(key, value);
    }
    for (key, value) in managed_shell_environment(layers) {
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

fn guest_accounts(capsule: &Capsule<'_>, layers: &EnvironmentLayers) -> Result<(PathBuf, PathBuf)> {
    let parent = capsule
        .state
        .parent()
        .context("capsule state has no parent")?;
    let root = parent.join(format!(".{}-accounts", capsule.harness.name()));
    match fs::symlink_metadata(&root) {
        Ok(metadata) if metadata.is_dir() && !metadata.file_type().is_symlink() => {}
        Ok(_) => bail!("capsule account projection is not a real directory"),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => fs::create_dir(&root)?,
        Err(error) => return Err(error.into()),
    }
    let uid = effective_uid();
    let gid = effective_gid();
    let passwd = root.join("passwd");
    let group = root.join("group");
    write_account_file(
        &passwd,
        format!(
            "root:x:0:0:root:/root:/bin/sh\nfortlet-agent:x:{uid}:{gid}:Fortlet agent:/home/agent:{}\n",
            layers.runtime.shell
        )
        .as_bytes(),
    )?;
    write_account_file(
        &group,
        format!("root:x:0:\nfortlet-agent:x:{gid}:\n").as_bytes(),
    )?;
    Ok((passwd, group))
}

fn write_account_file(path: &Path, contents: &[u8]) -> Result<()> {
    match fs::symlink_metadata(path) {
        Ok(metadata) if metadata.is_file() && !metadata.file_type().is_symlink() => {}
        Ok(_) => bail!("capsule account projection is not a regular file"),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
        Err(error) => return Err(error.into()),
    }
    let parent = path.parent().context("account projection has no parent")?;
    let mut temporary = tempfile::NamedTempFile::new_in(parent)?;
    temporary.as_file_mut().write_all(contents)?;
    temporary.as_file_mut().sync_all()?;
    temporary.persist(path)?;
    Ok(())
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
    environment.extend([
        (
            GIT_CONFIG_GLOBAL.to_owned(),
            capsule
                .identity
                .map(|_| format!("{GUEST_IDENTITY_ROOT}/gitconfig"))
                .unwrap_or_else(|| "/dev/null".to_owned()),
        ),
        (GIT_CONFIG_NOSYSTEM.to_owned(), "1".to_owned()),
        (
            JJ_CONFIG.to_owned(),
            capsule
                .identity
                .map(|_| format!("{GUEST_IDENTITY_ROOT}/jjconfig.toml"))
                .unwrap_or_else(|| "/dev/null".to_owned()),
        ),
    ]);
    environment.extend(capsule.harness.environment(capsule.project));
    match environment
        .iter_mut()
        .find(|(key, _)| key == "LD_LIBRARY_PATH")
    {
        Some((_, value)) => {
            value.push(':');
            value.push_str(&layers.runtime.runtime_library_path);
        }
        None => environment.push((
            "LD_LIBRARY_PATH".into(),
            layers.runtime.runtime_library_path.clone(),
        )),
    }
    environment
}

fn managed_shell_environment(layers: &EnvironmentLayers) -> [(String, String); 3] {
    let path = guest_path(layers);
    [
        ("PATH".into(), path.clone()),
        (MANAGED_PATH_ENV.into(), path),
        ("BASH_ENV".into(), layers.runtime.managed_bash_env.clone()),
    ]
}

fn guest_path(layers: &EnvironmentLayers) -> String {
    let mut entries = layers
        .project
        .as_ref()
        .map(|project| project.path.clone())
        .unwrap_or_default();
    entries.extend([
        Path::new(&layers.runtime.harness_executable)
            .parent()
            .expect("validated harness executable has a parent")
            .display()
            .to_string(),
        format!("{}/bin", layers.runtime.runtime_root),
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

fn tool_executable(layers: &EnvironmentLayers) -> String {
    layers.runtime.harness_executable.clone()
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
pub(crate) fn effective_uid() -> u32 {
    unsafe { libc::geteuid() }
}

#[cfg(unix)]
pub(crate) fn effective_gid() -> u32 {
    unsafe { libc::getegid() }
}

#[cfg(test)]
mod tests {
    use std::collections::VecDeque;
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::Arc;

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
    fn attachment_arguments_pass_through_the_harness_adapter() {
        let requested = vec!["exec".to_owned(), "hello".to_owned()];

        assert_eq!(
            effective_launch_arguments(harness::find("codex").unwrap(), &requested),
            vec![
                "--disable".to_owned(),
                "apps".to_owned(),
                "exec".to_owned(),
                "hello".to_owned(),
            ]
        );
        assert_eq!(
            effective_launch_arguments(harness::find("tact").unwrap(), &requested),
            requested
        );
    }

    #[tokio::test]
    async fn non_interactive_stream_forwards_output_and_exact_exit_status() {
        let mut session = FakeExecutionSession::new([
            ProcessEvent::Started,
            ProcessEvent::Stdout(b"visible stdout".to_vec()),
            ProcessEvent::Stderr(b"visible stderr".to_vec()),
            ProcessEvent::Exited(7),
        ]);
        let mut stdout = RecordingWriter::default();
        let mut stderr = RecordingWriter::default();

        let code = forward_non_interactive(&mut session, &mut stdout, &mut stderr, None)
            .await
            .unwrap();

        assert_eq!(stdout.bytes, b"visible stdout");
        assert_eq!(stderr.bytes, b"visible stderr");
        assert_eq!(stdout.flushes, 1);
        assert_eq!(stderr.flushes, 1);
        assert_eq!(code, 7);
    }

    #[tokio::test]
    async fn non_interactive_stream_returns_zero_exit_status() {
        let mut session = FakeExecutionSession::new([ProcessEvent::Exited(0)]);

        let code = forward_non_interactive(&mut session, &mut Vec::new(), &mut Vec::new(), None)
            .await
            .unwrap();

        assert_eq!(code, 0);
    }

    #[tokio::test]
    async fn non_interactive_stream_closes_stdin_before_reading_events() {
        let mut session = FakeExecutionSession::new([ProcessEvent::Exited(0)]);

        forward_non_interactive(&mut session, &mut Vec::new(), &mut Vec::new(), None)
            .await
            .unwrap();

        assert!(session.stdin_closed);
        assert!(!session.event_read_before_stdin_closed);
    }

    #[tokio::test]
    async fn non_interactive_stream_rejects_a_missing_exit_event() {
        let mut session = FakeExecutionSession::new([]);

        let error = forward_non_interactive(&mut session, &mut Vec::new(), &mut Vec::new(), None)
            .await
            .unwrap_err();

        assert_eq!(error.to_string(), "exec session ended without exit event");
    }

    #[tokio::test(start_paused = true)]
    async fn output_activity_renews_the_non_interactive_deadline() {
        let nine_minutes = std::time::Duration::from_secs(9 * 60);
        let mut session = DelayedExecutionSession::new([
            (nine_minutes, ProcessEvent::Stdout(b"still".to_vec())),
            (nine_minutes, ProcessEvent::Stderr(b" working".to_vec())),
            (nine_minutes, ProcessEvent::Exited(0)),
        ]);
        let mut stdout = Vec::new();
        let mut stderr = Vec::new();

        let code = forward_non_interactive(
            &mut session,
            &mut stdout,
            &mut stderr,
            Some(std::time::Duration::from_secs(10 * 60)),
        )
        .await
        .unwrap();

        assert_eq!(stdout, b"still");
        assert_eq!(stderr, b" working");
        assert_eq!(code, 0);
    }

    #[tokio::test]
    async fn non_interactive_inactivity_kills_the_command_and_fails_boundedly() {
        let killed = Arc::new(AtomicBool::new(false));
        let mut session = FakeExecutionSession::stalled(killed.clone(), 137);
        let mut stdout = Vec::new();
        let mut stderr = Vec::new();

        let result = tokio::time::timeout(
            std::time::Duration::from_millis(100),
            forward_non_interactive(
                &mut session,
                &mut stdout,
                &mut stderr,
                Some(std::time::Duration::from_millis(10)),
            ),
        )
        .await
        .expect("inactivity handling must be bounded");
        let error = result.unwrap_err();

        assert!(killed.load(Ordering::SeqCst));
        assert!(error
            .to_string()
            .contains("exceeded its 0.01-second inactivity ceiling"));
    }

    #[tokio::test(start_paused = true)]
    async fn non_interactive_kill_cleanup_has_a_short_bound() {
        let killed = Arc::new(AtomicBool::new(false));
        let mut session = FakeExecutionSession::never_exits(killed.clone());

        let error = forward_non_interactive(
            &mut session,
            &mut Vec::new(),
            &mut Vec::new(),
            Some(std::time::Duration::from_secs(10 * 60)),
        )
        .await
        .unwrap_err();

        assert!(killed.load(Ordering::SeqCst));
        assert!(error
            .to_string()
            .contains("inactive command did not exit after kill"));
    }

    #[derive(Default)]
    struct RecordingWriter {
        bytes: Vec<u8>,
        flushes: usize,
    }

    impl Write for RecordingWriter {
        fn write(&mut self, buffer: &[u8]) -> std::io::Result<usize> {
            self.bytes.extend_from_slice(buffer);
            Ok(buffer.len())
        }

        fn flush(&mut self) -> std::io::Result<()> {
            self.flushes += 1;
            Ok(())
        }
    }

    struct FakeExecutionSession {
        events: VecDeque<ProcessEvent>,
        stdin_closed: bool,
        event_read_before_stdin_closed: bool,
        killed: Option<Arc<AtomicBool>>,
        exit_after_kill: Option<i32>,
        stall_after_kill: bool,
    }

    impl FakeExecutionSession {
        fn new(events: impl IntoIterator<Item = ProcessEvent>) -> Self {
            Self {
                events: events.into_iter().collect(),
                stdin_closed: false,
                event_read_before_stdin_closed: false,
                killed: None,
                exit_after_kill: None,
                stall_after_kill: false,
            }
        }

        fn stalled(killed: Arc<AtomicBool>, exit_after_kill: i32) -> Self {
            Self {
                events: VecDeque::from([ProcessEvent::Started]),
                stdin_closed: false,
                event_read_before_stdin_closed: false,
                killed: Some(killed),
                exit_after_kill: Some(exit_after_kill),
                stall_after_kill: false,
            }
        }

        fn never_exits(killed: Arc<AtomicBool>) -> Self {
            Self {
                events: VecDeque::from([ProcessEvent::Started]),
                stdin_closed: false,
                event_read_before_stdin_closed: false,
                killed: Some(killed),
                exit_after_kill: None,
                stall_after_kill: true,
            }
        }
    }

    impl ExecutionSession for FakeExecutionSession {
        async fn close_stdin(&mut self) -> Result<()> {
            self.stdin_closed = true;
            Ok(())
        }

        async fn next_event(&mut self) -> Result<Option<ProcessEvent>> {
            self.event_read_before_stdin_closed |= !self.stdin_closed;
            if let Some(event) = self.events.pop_front() {
                return Ok(Some(event));
            }
            if self
                .killed
                .as_ref()
                .is_some_and(|killed| killed.load(Ordering::SeqCst))
            {
                if self.stall_after_kill {
                    return std::future::pending().await;
                }
                return Ok(self.exit_after_kill.take().map(ProcessEvent::Exited));
            }
            if self.killed.is_some() {
                std::future::pending().await
            } else {
                Ok(None)
            }
        }

        async fn kill(&self) -> Result<()> {
            if let Some(killed) = &self.killed {
                killed.store(true, Ordering::SeqCst);
            }
            Ok(())
        }
    }

    struct DelayedExecutionSession {
        events: VecDeque<(Duration, ProcessEvent)>,
    }

    impl DelayedExecutionSession {
        fn new(events: impl IntoIterator<Item = (Duration, ProcessEvent)>) -> Self {
            Self {
                events: events.into_iter().collect(),
            }
        }
    }

    impl ExecutionSession for DelayedExecutionSession {
        async fn close_stdin(&mut self) -> Result<()> {
            Ok(())
        }

        async fn next_event(&mut self) -> Result<Option<ProcessEvent>> {
            let Some((delay, event)) = self.events.pop_front() else {
                return Ok(None);
            };
            tokio::time::sleep(delay).await;
            Ok(Some(event))
        }

        async fn kill(&self) -> Result<()> {
            Ok(())
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

        CapsuleDescriptor::for_launch(&project, harness, None, None, None)
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
            guest_root: "/opt/fortlet/project".into(),
            path: vec![
                "/opt/fortlet/project/cargo/bin".into(),
                "/opt/fortlet/project/jj/bin".into(),
            ],
            environment: BTreeMap::from([("RUST_BACKTRACE".into(), "1".into())]),
        };
        let layers = EnvironmentLayers {
            runtime: crate::runtime_artifacts::PreparedRuntime {
                store_volume: "fortlet-store-501-fip0013-1-project".into(),
                image_reference: "fortlet-runtime:fip0013-1-aarch64-linux".into(),
                runtime_root: "/nix/store/runtime".into(),
                nix_version: "2.34.8".into(),
                nix: "/nix/store/runtime/bin/nix".into(),
                shell: "/nix/store/runtime/bin/bash".into(),
                hold: "/nix/store/runtime/bin/fortlet-hold".into(),
                managed_bash_env: "/nix/store/runtime/etc/managed-bash-env".into(),
                runtime_library_path: "/nix/store/glibc/lib:/nix/store/zlib/lib".into(),
                harness_executable: "/nix/store/codex/bin/codex".into(),
                runtime_identity: "sha256:runtime:seed".into(),
                harness_identity: "harness-closure".into(),
            },
            project: Some(published),
        };

        assert!(guest_path(&layers)
            .starts_with("/opt/fortlet/project/cargo/bin:/opt/fortlet/project/jj/bin:"));
        let managed_path = guest_path(&layers);
        let managed_environment = BTreeMap::from(managed_shell_environment(&layers));
        assert_eq!(managed_environment.get("PATH"), Some(&managed_path));
        assert_eq!(
            managed_environment.get(MANAGED_PATH_ENV),
            Some(&managed_path)
        );
        assert_eq!(
            managed_environment.get("BASH_ENV").map(String::as_str),
            Some("/nix/store/runtime/etc/managed-bash-env")
        );
        for harness_name in ["codex", "tact"] {
            let harness = harness::find(harness_name).unwrap();
            let capsule = Capsule {
                descriptor: CapsuleDescriptor::new(&project, harness),
                state: "/state".into(),
                project: &project,
                harness,
                identity: None,
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
