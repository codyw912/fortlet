use std::path::PathBuf;

use anyhow::{Context, Result};

use crate::auth::{require_outside_mounts, write_public_projection, Credentials};
use crate::environment::EnvironmentStore;
use crate::harness;
use crate::paths::AppPaths;
use crate::project;
use crate::runtime::MicroSandboxRuntime;

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
    let credentials = stage(
        Credentials::read_default(),
        "credentials",
        "refresh the host Codex login and retry",
    )?;
    stage(
        require_outside_mounts(&credentials, &[&project.root, &paths.data]),
        "credentials",
        "move the host credential file outside every guest mount and retry",
    )?;
    let runtime = MicroSandboxRuntime::new(&paths);
    let capsule = stage(
        runtime.capsule(&project, harness),
        "capsule",
        "check the reported Fortlet state path and retry",
    )?;
    stage(
        write_public_projection(&capsule.state.join(".codex/auth.json")),
        "credentials",
        "remove the reported invalid guest projection and retry",
    )?;
    stage(
        require_outside_mounts(&credentials, &[&capsule.state]),
        "credentials",
        "move the host credential file outside every guest mount and retry",
    )?;
    let layers = stage(
        EnvironmentStore::new(&paths).ensure(harness).await,
        "environment",
        "check network access or remove the reported incomplete layer and retry",
    )?;
    let sandbox = stage(
        runtime.ensure(&capsule, &layers, &credentials).await,
        "capsule",
        "run `fortlet doctor` and follow its reported correction",
    )?;
    stage(
        runtime.attach(&capsule, &sandbox, &request.arguments).await,
        "terminal",
        "retry from a supported terminal or use a non-interactive harness command",
    )
}

fn stage<T>(result: Result<T>, name: &str, action: &str) -> Result<T> {
    result.with_context(|| format!("{name} stage failed; {action}"))
}

#[cfg(test)]
mod tests {
    use anyhow::anyhow;

    use super::*;

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
}
