use std::path::PathBuf;

use anyhow::Result;

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
    let paths = AppPaths::from_environment()?;
    paths.ensure_roots()?;
    let harness = harness::find(&request.harness)?;
    let project = project::resolve(
        &paths,
        request.project.as_deref(),
        request.allow_broad_mount,
    )?;
    if project.scratch {
        eprintln!(
            "fortlet: using persistent scratch workspace {}; pass --allow-broad-mount to expose this directory",
            project.root.display()
        );
    }
    let credentials = Credentials::read_default()?;
    let runtime = MicroSandboxRuntime::new(&paths);
    let capsule = runtime.capsule(&project, harness)?;
    write_public_projection(&capsule.state.join(".codex/auth.json"))?;
    require_outside_mounts(&credentials, &[&project.root, &capsule.state, &paths.data])?;
    let layers = EnvironmentStore::new(&paths).ensure(harness).await?;
    let sandbox = runtime.ensure(&capsule, &layers, &credentials).await?;
    runtime.attach(&capsule, &sandbox, &request.arguments).await
}
