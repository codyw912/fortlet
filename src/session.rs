use std::ffi::OsStr;
use std::fmt;
use std::path::PathBuf;
use std::time::{Duration, Instant};

use anyhow::{anyhow, bail, Context, Result};

use crate::auth::{
    require_outside_mounts, require_source_outside_mounts, write_codex_projection,
    write_tact_projection, CredentialStore, Credentials,
};
use crate::environment::EnvironmentStore;
use crate::harness;
use crate::paths::AppPaths;
use crate::project;
use crate::project_environment::ProjectEnvironment;
use crate::runtime::{rotate_credentials, Capsule, MicroSandboxRuntime};

const TERMINAL_CORRECTION: &str = "retry interactively from a supported terminal";
const STARTUP_TIMINGS_ENV: &str = "FORTLET_STARTUP_TIMINGS";

pub struct LaunchRequest {
    pub harness: String,
    pub project: Option<PathBuf>,
    pub allow_broad_mount: bool,
    pub arguments: Vec<String>,
}

pub async fn launch(request: LaunchRequest) -> Result<i32> {
    let mut timings = StartupTimings::from_environment();
    let paths = stage(
        AppPaths::from_environment(),
        "project",
        "set HOME to a writable user directory and retry",
    )?;
    stage(
        paths.ensure_roots(),
        "project",
        "check the reported Fortlet state path and its permissions",
    )?;
    let harness = stage(
        harness::find(&request.harness),
        "harness",
        "choose a registered harness: codex or tact",
    )?;
    let project = project::resolve(
        &paths,
        request.project.as_deref(),
        request.allow_broad_mount,
    );
    let project = stage(
        project,
        "project",
        "run from a readable project directory or pass --project <path>",
    )?;
    if project.scratch {
        eprintln!(
            "fortlet: using persistent scratch workspace {}; pass --allow-broad-mount to expose this directory",
            project.root.display()
        );
    }
    let project_environment = stage(
        ProjectEnvironment::discover(&project),
        "project environment",
        "fix or remove .fortlet/environment.json and retry",
    )?;
    timings.record(StartupPhase::Resolve);
    let credential_store = if harness.name() == "codex" {
        let store = stage(
            CredentialStore::from_environment(&paths),
            "credentials",
            "run `codex login` on the host and retry",
        )?;
        stage(
            require_source_outside_mounts(
                store.source(),
                &[&project.root, &paths.data, &paths.state],
            ),
            "credentials",
            "move the host credential file outside every guest mount and retry",
        )?;
        Some(store)
    } else {
        None
    };
    let credentials = if let Some(store) = &credential_store {
        stage(
            store.renew().await,
            "credentials",
            "run `codex login` on the host or retry the bounded refresh",
        )?
    } else {
        stage(
            Credentials::read_default(),
            "credentials",
            "refresh the host Codex login and retry",
        )?
    };
    stage(
        require_outside_mounts(&credentials, &[&project.root, &paths.data]),
        "credentials",
        "move the host credential file outside every guest mount and retry",
    )?;
    timings.record(StartupPhase::Credentials);
    let runtime = MicroSandboxRuntime::new(&paths);
    let capsule = stage(
        runtime.capsule(&project, harness, project_environment.as_ref()),
        "capsule",
        "check the reported Fortlet state path and retry",
    )?;
    let projection = capsule.state.join(".codex/auth.json");
    let projection_result = if harness.name() == "codex" {
        write_codex_projection(&projection)
    } else {
        write_tact_projection(&projection)
    };
    stage(
        projection_result,
        "credentials",
        "remove the reported invalid guest projection and retry",
    )?;
    stage(
        require_outside_mounts(&credentials, &[&capsule.state]),
        "credentials",
        "move the host credential file outside every guest mount and retry",
    )?;
    timings.record(StartupPhase::CapsuleState);
    let layers = stage(
        EnvironmentStore::new(&paths)
            .ensure(harness, project_environment.as_ref())
            .await,
        "environment",
        "check network access or remove the reported incomplete layer and retry",
    )?;
    timings.record(StartupPhase::Environment);
    let sandbox = stage(
        runtime.ensure(&capsule, &layers, &credentials).await,
        "capsule",
        "run `fortlet doctor` and follow its reported correction",
    )?;
    timings.record(StartupPhase::Runtime);
    let result = if let Some(store) = credential_store {
        attach_codex_with_lease(
            &runtime,
            &capsule,
            &sandbox,
            &request.arguments,
            store,
            credentials,
        )
        .await
    } else {
        stage(
            runtime.attach(&capsule, &sandbox, &request.arguments).await,
            "terminal",
            TERMINAL_CORRECTION,
        )
    };
    timings.record(StartupPhase::Command);
    result
}

#[derive(Clone, Copy)]
enum StartupPhase {
    Resolve,
    Credentials,
    CapsuleState,
    Environment,
    Runtime,
    Command,
}

impl StartupPhase {
    const fn as_str(self) -> &'static str {
        match self {
            Self::Resolve => "resolve",
            Self::Credentials => "credentials",
            Self::CapsuleState => "capsule-state",
            Self::Environment => "environment",
            Self::Runtime => "runtime",
            Self::Command => "command",
        }
    }
}

struct StartupTimings {
    enabled: bool,
    started: Instant,
    previous: Instant,
}

impl StartupTimings {
    fn from_environment() -> Self {
        let started = Instant::now();
        Self {
            enabled: startup_timings_enabled(std::env::var_os(STARTUP_TIMINGS_ENV).as_deref()),
            started,
            previous: started,
        }
    }

    fn record(&mut self, phase: StartupPhase) {
        if let Some(event) = self.record_at(phase, Instant::now()) {
            eprintln!("{event}");
        }
    }

    fn record_at(&mut self, phase: StartupPhase, now: Instant) -> Option<StartupTimingEvent> {
        if !self.enabled {
            return None;
        }
        let event = StartupTimingEvent {
            phase,
            delta: now.saturating_duration_since(self.previous),
            total: now.saturating_duration_since(self.started),
        };
        self.previous = now;
        Some(event)
    }
}

fn startup_timings_enabled(value: Option<&OsStr>) -> bool {
    value == Some(OsStr::new("1"))
}

struct StartupTimingEvent {
    phase: StartupPhase,
    delta: Duration,
    total: Duration,
}

impl fmt::Display for StartupTimingEvent {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "fortlet: startup-timing phase={} delta_ms={} total_ms={}",
            self.phase.as_str(),
            self.delta.as_millis(),
            self.total.as_millis()
        )
    }
}

async fn attach_codex_with_lease(
    runtime: &MicroSandboxRuntime<'_>,
    capsule: &Capsule<'_>,
    sandbox: &microsandbox::Sandbox,
    arguments: &[String],
    store: CredentialStore,
    credentials: Credentials,
) -> Result<i32> {
    let renewal_sandbox = sandbox.clone();
    let renewal = tokio::spawn(async move {
        let mut current = credentials;
        loop {
            tokio::time::sleep(current.refresh_delay()?).await;
            current = store.renew().await?;
            rotate_credentials(&renewal_sandbox, &current).await?;
        }
        #[allow(unreachable_code)]
        Ok::<(), anyhow::Error>(())
    });
    match await_attachment_with_renewal(runtime.attach(capsule, sandbox, arguments), renewal).await
    {
        AttachmentOutcome::Attached(attached) => stage(attached, "terminal", TERMINAL_CORRECTION),
        AttachmentOutcome::RenewalEnded(error) => {
            bail!(
                "credentials stage failed; run `codex login` on the host or retry the bounded refresh: {error:#}"
            )
        }
    }
}

enum AttachmentOutcome<T> {
    Attached(Result<T>),
    RenewalEnded(anyhow::Error),
}

async fn await_attachment_with_renewal<T>(
    attachment: impl std::future::Future<Output = Result<T>>,
    mut renewal: tokio::task::JoinHandle<Result<()>>,
) -> AttachmentOutcome<T> {
    tokio::select! {
        attached = attachment => {
            cancel_renewal(renewal).await;
            AttachmentOutcome::Attached(attached)
        }
        renewed = &mut renewal => {
            let error = match renewed {
                Ok(Err(error)) => error,
                Ok(Ok(())) => anyhow!("Codex credential lease ended unexpectedly"),
                Err(error) => anyhow!("Codex credential lease task failed: {error}"),
            };
            AttachmentOutcome::RenewalEnded(error)
        }
    }
}

async fn cancel_renewal(mut renewal: tokio::task::JoinHandle<Result<()>>) {
    renewal.abort();
    let _ = tokio::time::timeout(Duration::from_secs(1), &mut renewal).await;
}

fn stage<T>(result: Result<T>, name: &str, action: &str) -> Result<T> {
    result.with_context(|| format!("{name} stage failed; {action}"))
}

#[cfg(test)]
mod tests {
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::Arc;

    use anyhow::anyhow;

    use super::*;

    struct CancellationFlag(Arc<AtomicBool>);

    impl Drop for CancellationFlag {
        fn drop(&mut self) {
            self.0.store(true, Ordering::SeqCst);
        }
    }

    fn pending_renewal(cancelled: Arc<AtomicBool>) -> tokio::task::JoinHandle<Result<()>> {
        let guard = CancellationFlag(cancelled);
        tokio::spawn(async move {
            let _guard = guard;
            std::future::pending::<()>().await;
            Ok(())
        })
    }

    #[test]
    fn staged_errors_name_the_stage_and_one_action() {
        let error = stage::<()>(
            Err(anyhow!("root cause")),
            "credentials",
            "refresh the host login and retry",
        )
        .unwrap_err();
        let message = format!("{error:#}");

        assert!(message.starts_with("credentials stage failed; refresh the host login and retry"));
        assert!(message.ends_with("root cause"));
    }

    #[test]
    fn startup_timings_require_an_exact_opt_in() {
        assert!(startup_timings_enabled(Some(OsStr::new("1"))));
        assert!(!startup_timings_enabled(None));
        assert!(!startup_timings_enabled(Some(OsStr::new("true"))));
        assert!(!startup_timings_enabled(Some(OsStr::new("0"))));
    }

    #[test]
    fn startup_timing_output_contains_only_bounded_phase_and_durations() {
        let started = Instant::now();
        let mut timings = StartupTimings {
            enabled: true,
            started,
            previous: started,
        };

        let first = timings
            .record_at(StartupPhase::Resolve, started + Duration::from_millis(17))
            .unwrap();
        let second = timings
            .record_at(StartupPhase::Runtime, started + Duration::from_millis(41))
            .unwrap();

        assert_eq!(
            first.to_string(),
            "fortlet: startup-timing phase=resolve delta_ms=17 total_ms=17"
        );
        assert_eq!(
            second.to_string(),
            "fortlet: startup-timing phase=runtime delta_ms=24 total_ms=41"
        );
    }

    #[test]
    fn disabled_startup_timings_emit_no_event() {
        let started = Instant::now();
        let mut timings = StartupTimings {
            enabled: false,
            started,
            previous: started,
        };

        assert!(timings
            .record_at(StartupPhase::Resolve, started + Duration::from_secs(1))
            .is_none());
    }

    #[tokio::test]
    async fn renewal_cancellation_is_bounded() {
        let renewal = tokio::spawn(async {
            tokio::time::sleep(Duration::from_secs(60)).await;
            Ok(())
        });
        let started = std::time::Instant::now();

        cancel_renewal(renewal).await;

        assert!(started.elapsed() < Duration::from_secs(1));
    }

    #[tokio::test]
    async fn attachment_completion_cancels_the_renewal_task_before_returning() {
        let cancelled = Arc::new(AtomicBool::new(false));
        let renewal = pending_renewal(cancelled.clone());

        let outcome = await_attachment_with_renewal(async { Ok(7) }, renewal).await;

        assert!(matches!(outcome, AttachmentOutcome::Attached(Ok(7))));
        assert!(cancelled.load(Ordering::SeqCst));
    }

    #[tokio::test]
    async fn attachment_failure_cancels_the_renewal_task_before_returning() {
        let cancelled = Arc::new(AtomicBool::new(false));
        let renewal = pending_renewal(cancelled.clone());

        let outcome = await_attachment_with_renewal(
            async { Err::<i32, _>(anyhow!("inactivity timeout")) },
            renewal,
        )
        .await;

        assert!(matches!(outcome, AttachmentOutcome::Attached(Err(_))));
        assert!(cancelled.load(Ordering::SeqCst));
    }
}
