use std::path::PathBuf;
use std::time::Duration;

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

pub struct LaunchRequest {
    pub harness: String,
    pub project: Option<PathBuf>,
    pub allow_broad_mount: bool,
    pub arguments: Vec<String>,
}

pub async fn launch(request: LaunchRequest) -> Result<i32> {
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
    let layers = stage(
        EnvironmentStore::new(&paths)
            .ensure(harness, project_environment.as_ref())
            .await,
        "environment",
        "check network access or remove the reported incomplete layer and retry",
    )?;
    let sandbox = stage(
        runtime.ensure(&capsule, &layers, &credentials).await,
        "capsule",
        "run `fortlet doctor` and follow its reported correction",
    )?;
    if let Some(store) = credential_store {
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
