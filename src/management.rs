use std::path::PathBuf;

use anyhow::{Context, Result};
use microsandbox::sandbox::{SandboxConfig, SandboxHandle, SandboxStatus};
use microsandbox::{MicrosandboxError, Sandbox};

use crate::harness::{self, Harness};
use crate::paths::AppPaths;
use crate::project::{self, Project};
use crate::runtime::{lock_capsule, CapsuleDescriptor};

pub struct StatusRequest {
    pub harness: Option<String>,
    pub project: Option<PathBuf>,
    pub allow_broad_mount: bool,
}

pub struct StopRequest {
    pub harness: String,
    pub project: Option<PathBuf>,
    pub allow_broad_mount: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum StopOutcome {
    Absent,
    AlreadyStopped,
    Stopped,
}

trait CapsuleControl {
    async fn inspect(&self, descriptor: &CapsuleDescriptor) -> Result<Option<SandboxStatus>>;

    async fn stop(&self, descriptor: &CapsuleDescriptor) -> Result<StopOutcome>;
}

struct LocalCapsuleControl<'a> {
    paths: &'a AppPaths,
}

impl CapsuleControl for LocalCapsuleControl<'_> {
    async fn inspect(&self, descriptor: &CapsuleDescriptor) -> Result<Option<SandboxStatus>> {
        let Some(handle) = get(descriptor).await? else {
            return Ok(None);
        };
        validate_handle(descriptor, &handle)?;
        Ok(Some(handle.status_snapshot()))
    }

    async fn stop(&self, descriptor: &CapsuleDescriptor) -> Result<StopOutcome> {
        if get(descriptor).await?.is_none() {
            return Ok(StopOutcome::Absent);
        }
        let _lock = lock_capsule(self.paths, descriptor)?;
        let Some(handle) = get(descriptor).await? else {
            return Ok(StopOutcome::Absent);
        };
        validate_handle(descriptor, &handle)?;
        if matches!(
            handle.status_snapshot(),
            SandboxStatus::Stopped | SandboxStatus::Crashed
        ) {
            return Ok(StopOutcome::AlreadyStopped);
        }
        handle.stop().await.context("cannot stop owned capsule")?;
        Ok(StopOutcome::Stopped)
    }
}

pub async fn status(request: StatusRequest) -> Result<Vec<String>> {
    let harnesses = select_harnesses(request.harness.as_deref())?;
    let (paths, project) = resolve_project(request.project, request.allow_broad_mount)?;
    status_with(&LocalCapsuleControl { paths: &paths }, &project, harnesses).await
}

pub async fn stop(request: StopRequest) -> Result<String> {
    let harness = stage(
        harness::find(&request.harness),
        "harness",
        "choose a registered harness: codex or tact",
    )?;
    let (paths, project) = resolve_project(request.project, request.allow_broad_mount)?;
    stop_with(&LocalCapsuleControl { paths: &paths }, &project, harness).await
}

async fn status_with<C: CapsuleControl>(
    control: &C,
    project: &Project,
    harnesses: Vec<&dyn Harness>,
) -> Result<Vec<String>> {
    let mut lines = Vec::with_capacity(harnesses.len());
    for harness in harnesses {
        let descriptor = CapsuleDescriptor::new(project, harness);
        let observed = capsule_stage(control.inspect(&descriptor).await)?;
        lines.push(format!(
            "{}\t{}",
            harness.name(),
            observed.map_or("absent", status_name)
        ));
    }
    Ok(lines)
}

async fn stop_with<C: CapsuleControl>(
    control: &C,
    project: &Project,
    harness: &dyn Harness,
) -> Result<String> {
    let descriptor = CapsuleDescriptor::new(project, harness);
    let outcome = capsule_stage(control.stop(&descriptor).await)?;
    let name = match outcome {
        StopOutcome::Absent => "absent",
        StopOutcome::AlreadyStopped => "already-stopped",
        StopOutcome::Stopped => "stopped",
    };
    Ok(format!("{}\t{name}", harness.name()))
}

fn select_harnesses(name: Option<&str>) -> Result<Vec<&'static dyn Harness>> {
    match name {
        Some(name) => stage(
            harness::find(name).map(|harness| vec![harness]),
            "harness",
            "choose a registered harness: codex or tact",
        ),
        None => harness::names()
            .map(harness::find)
            .collect::<Result<Vec<_>>>(),
    }
}

fn resolve_project(
    project: Option<PathBuf>,
    allow_broad_mount: bool,
) -> Result<(AppPaths, Project)> {
    let paths = stage(
        AppPaths::from_environment(),
        "project",
        "set HOME to a readable user directory and retry",
    )?;
    let project = stage(
        project::resolve(&paths, project.as_deref(), allow_broad_mount),
        "project",
        "run from a readable project directory or pass --project <path>",
    )?;
    if project.scratch {
        eprintln!(
            "fortlet: using persistent scratch workspace {}; pass --allow-broad-mount to expose this directory",
            project.root.display()
        );
    }
    Ok((paths, project))
}

async fn get(descriptor: &CapsuleDescriptor) -> Result<Option<SandboxHandle>> {
    match Sandbox::get(&descriptor.name).await {
        Ok(handle) => Ok(Some(handle)),
        Err(MicrosandboxError::SandboxNotFound(_)) => Ok(None),
        Err(error) => Err(error.into()),
    }
}

fn validate_handle(descriptor: &CapsuleDescriptor, handle: &SandboxHandle) -> Result<()> {
    validate_stored_config(descriptor, handle.config_json())
}

fn validate_stored_config(descriptor: &CapsuleDescriptor, stored: &str) -> Result<()> {
    let config: SandboxConfig =
        serde_json::from_str(stored).context("cannot read stored capsule configuration")?;
    descriptor.validate_management(&config.spec.name, &config.spec.labels)
}

fn status_name(status: SandboxStatus) -> &'static str {
    match status {
        SandboxStatus::Created => "created",
        SandboxStatus::Starting => "starting",
        SandboxStatus::Running => "running",
        SandboxStatus::Draining => "draining",
        SandboxStatus::Paused => "paused",
        SandboxStatus::Stopped => "stopped",
        SandboxStatus::Crashed => "crashed",
    }
}

fn capsule_stage<T>(result: Result<T>) -> Result<T> {
    stage(
        result,
        "capsule",
        "run `fortlet doctor` and follow its reported correction",
    )
}

fn stage<T>(result: Result<T>, name: &str, action: &str) -> Result<T> {
    result.with_context(|| format!("{name} stage failed; {action}"))
}

#[cfg(test)]
mod tests {
    use anyhow::anyhow;

    use super::*;
    use crate::project::ProjectIdentity;

    struct FakeControl {
        status: Result<Option<SandboxStatus>, &'static str>,
        stop: Result<StopOutcome, &'static str>,
    }

    impl CapsuleControl for FakeControl {
        async fn inspect(&self, _descriptor: &CapsuleDescriptor) -> Result<Option<SandboxStatus>> {
            self.status.map_err(|message| anyhow!(message))
        }

        async fn stop(&self, _descriptor: &CapsuleDescriptor) -> Result<StopOutcome> {
            self.stop.map_err(|message| anyhow!(message))
        }
    }

    fn project() -> Project {
        Project {
            identity: ProjectIdentity::from_local_root(PathBuf::from("/tmp/management").as_path()),
            root: "/tmp/management".into(),
            cwd: "/tmp/management".into(),
            kind: "test",
            scratch: false,
        }
    }

    fn fake(status: Option<SandboxStatus>, stop: StopOutcome) -> FakeControl {
        FakeControl {
            status: Ok(status),
            stop: Ok(stop),
        }
    }

    #[tokio::test]
    async fn status_maps_absence_and_every_sdk_state() {
        let harness = harness::find("codex").unwrap();
        let cases = [
            (None, "absent"),
            (Some(SandboxStatus::Created), "created"),
            (Some(SandboxStatus::Starting), "starting"),
            (Some(SandboxStatus::Running), "running"),
            (Some(SandboxStatus::Draining), "draining"),
            (Some(SandboxStatus::Paused), "paused"),
            (Some(SandboxStatus::Stopped), "stopped"),
            (Some(SandboxStatus::Crashed), "crashed"),
        ];

        for (status, expected) in cases {
            let control = fake(status, StopOutcome::Stopped);
            let lines = status_with(&control, &project(), vec![harness])
                .await
                .unwrap();
            assert_eq!(lines, [format!("codex\t{expected}")]);
        }
    }

    #[tokio::test]
    async fn status_without_selector_uses_registry_order() {
        let control = fake(None, StopOutcome::Stopped);
        let harnesses = select_harnesses(None).unwrap();

        let lines = status_with(&control, &project(), harnesses).await.unwrap();

        assert_eq!(lines, ["codex\tabsent", "tact\tabsent"]);
    }

    #[tokio::test]
    async fn stop_renders_each_idempotent_outcome() {
        for (outcome, expected) in [
            (StopOutcome::Absent, "codex\tabsent"),
            (StopOutcome::AlreadyStopped, "codex\talready-stopped"),
            (StopOutcome::Stopped, "codex\tstopped"),
        ] {
            let control = fake(None, outcome);
            let line = stop_with(&control, &project(), harness::find("codex").unwrap())
                .await
                .unwrap();
            assert_eq!(line, expected);
        }
    }

    #[tokio::test]
    async fn runtime_failures_keep_stage_action_and_cause() {
        let control = FakeControl {
            status: Err("lookup failed"),
            stop: Err("stop failed"),
        };

        for error in [
            status_with(&control, &project(), vec![harness::find("codex").unwrap()])
                .await
                .unwrap_err(),
            stop_with(&control, &project(), harness::find("codex").unwrap())
                .await
                .unwrap_err(),
        ] {
            let message = format!("{error:#}");
            assert!(message.starts_with(
                "capsule stage failed; run `fortlet doctor` and follow its reported correction"
            ));
            assert!(message.ends_with("failed"));
        }
    }

    #[test]
    fn malformed_stored_configuration_fails_before_ownership_use() {
        let descriptor = CapsuleDescriptor::new(&project(), harness::find("codex").unwrap());

        let error = validate_stored_config(&descriptor, "not json").unwrap_err();

        assert!(
            format!("{error:#}").starts_with("cannot read stored capsule configuration"),
            "{error:#}"
        );
    }
}
