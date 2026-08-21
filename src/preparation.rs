use std::path::PathBuf;

use anyhow::{Context, Result};

use crate::environment::EnvironmentStore;
use crate::harness::{self, Harness};
use crate::paths::AppPaths;
use crate::project;
use crate::project_environment::ProjectEnvironment;

pub struct PrepareRequest {
    pub harness: String,
    pub project: Option<PathBuf>,
    pub allow_broad_mount: bool,
}

pub async fn prepare(request: PrepareRequest) -> Result<String> {
    let harness = stage(
        harness::find(&request.harness),
        "harness",
        "choose a registered harness: codex or tact",
    )?;
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
    let project = stage(
        project::resolve(
            &paths,
            request.project.as_deref(),
            request.allow_broad_mount,
        ),
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
        "project-environment",
        "fix or remove .fortlet/environment.json and retry",
    )?;
    ensure_layers(&paths, harness, &project, project_environment.as_ref()).await?;

    Ok(format!("{}\tready", harness.name()))
}

async fn ensure_layers(
    paths: &AppPaths,
    harness: &dyn Harness,
    project: &crate::project::Project,
    project_environment: Option<&ProjectEnvironment>,
) -> Result<()> {
    stage(
        EnvironmentStore::new(paths)
            .ensure(harness, project, project_environment)
            .await,
        "environment",
        "check network access or remove the reported incomplete layer and retry",
    )?;
    Ok(())
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
            "project-environment",
            "fix the project environment and retry",
        )
        .unwrap_err();

        assert_eq!(
            format!("{error:#}"),
            "project-environment stage failed; fix the project environment and retry: root cause"
        );
    }
}
