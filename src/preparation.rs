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
    ensure_layers(&paths, harness, project_environment.as_ref()).await?;

    Ok(format!("{}\tready", harness.name()))
}

async fn ensure_layers(
    paths: &AppPaths,
    harness: &dyn Harness,
    project_environment: Option<&ProjectEnvironment>,
) -> Result<()> {
    stage(
        EnvironmentStore::new(paths)
            .ensure(harness, project_environment)
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
    use std::fs;

    use anyhow::anyhow;

    use super::*;
    use crate::project::{Project, ProjectIdentity};

    fn seed_marker(root: &std::path::Path, marker: &str) {
        fs::create_dir_all(root).unwrap();
        let (name, version) = if marker == ".fortlet-base.json" {
            ("_base", "bookworm-1")
        } else {
            let name = root
                .parent()
                .unwrap()
                .file_name()
                .unwrap()
                .to_str()
                .unwrap();
            let version = root.file_name().unwrap().to_str().unwrap();
            (name, version)
        };
        fs::write(
            root.join(marker),
            serde_json::to_vec(&serde_json::json!({
                "name": name,
                "version": version,
                "image": crate::environment::BASE_IMAGE,
            }))
            .unwrap(),
        )
        .unwrap();
    }

    fn configured_environment(root: &std::path::Path) -> ProjectEnvironment {
        fs::create_dir(root.join(".fortlet")).unwrap();
        fs::write(
            root.join(".fortlet/environment.json"),
            r#"{"schema":1,"path":["bin"],"environment":{"RUST_BACKTRACE":"1"}}"#,
        )
        .unwrap();
        fs::write(root.join(".fortlet/environment.sh"), "mkdir -p /out/bin\n").unwrap();
        let root = root.canonicalize().unwrap();
        let project = Project {
            identity: ProjectIdentity::from_local_root(&root),
            root: root.clone(),
            cwd: root,
            kind: "test",
            scratch: false,
        };
        ProjectEnvironment::discover(&project).unwrap().unwrap()
    }

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

    #[tokio::test]
    async fn configured_cache_hit_verifies_content_for_both_harnesses() {
        let temporary = tempfile::tempdir().unwrap();
        let paths = AppPaths {
            state: temporary.path().join("state"),
            data: temporary.path().join("data"),
        };
        let project_root = temporary.path().join("project");
        fs::create_dir(&project_root).unwrap();
        let environment = configured_environment(&project_root);
        let published = paths.environments().join(environment.identity());
        fs::create_dir_all(published.join("bin")).unwrap();
        fs::write(published.join("bin/tool"), "verified").unwrap();
        environment.validate_and_mark(&published).unwrap();
        let marker_before = fs::read(published.join(".fortlet-project.json")).unwrap();

        seed_marker(
            &paths.tools().join("_base/bookworm-1"),
            ".fortlet-base.json",
        );
        for (name, version) in [("codex", "0.147.0"), ("tact", "0.3.7")] {
            seed_marker(
                &paths.tools().join(name).join(version),
                ".fortlet-tool.json",
            );
            ensure_layers(&paths, harness::find(name).unwrap(), Some(&environment))
                .await
                .unwrap();
        }

        assert_eq!(
            fs::read(published.join(".fortlet-project.json")).unwrap(),
            marker_before
        );
        assert!(!paths.state.join("projects").exists());

        fs::write(published.join("bin/tool"), "corrupted").unwrap();
        let error = ensure_layers(&paths, harness::find("codex").unwrap(), Some(&environment))
            .await
            .unwrap_err();
        assert!(format!("{error:#}").contains("content digest"));
    }
}
