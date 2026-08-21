use std::fs::{self, File};
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use anyhow::{bail, Context, Result};
use fs2::FileExt;
use microsandbox::sandbox::PullPolicy;
use microsandbox::Sandbox;

use crate::harness::Harness;
use crate::nix_provider;
use crate::paths::AppPaths;
use crate::prepare_timing::{PreparePhase, PrepareTimings};
use crate::project::Project;
use crate::project_environment::{guest_platform, ProjectEnvironment, PublishedProjectEnvironment};
use crate::runtime_artifacts::{PreparedRuntime, RuntimeArtifacts};

pub struct EnvironmentStore<'a> {
    paths: &'a AppPaths,
}

pub struct EnvironmentLayers {
    pub runtime: PreparedRuntime,
    pub project: Option<PublishedProjectEnvironment>,
}

impl<'a> EnvironmentStore<'a> {
    pub fn new(paths: &'a AppPaths) -> Self {
        Self { paths }
    }

    pub async fn ensure(
        &self,
        harness: &dyn Harness,
        project: &Project,
        project_environment: Option<&ProjectEnvironment>,
    ) -> Result<EnvironmentLayers> {
        let mut timings = PrepareTimings::from_environment();
        let artifacts = RuntimeArtifacts::from_environment()?;
        let runtime = artifacts
            .ensure(self.paths, project, harness, &mut timings)
            .await?;
        let published_project = match project_environment {
            Some(environment) if environment.is_recipe() => {
                let project = self.ensure_project(environment, &runtime).await?;
                timings.record(PreparePhase::Schema1Layer);
                Some(project)
            }
            Some(environment) => {
                let project =
                    nix_provider::ensure(self.paths, project, environment, &runtime).await?;
                timings.record(PreparePhase::Schema2Provider);
                Some(project)
            }
            None => None,
        };
        artifacts.verify_prepared(self.paths, project, harness)?;
        timings.record(PreparePhase::FinalVerification);
        Ok(EnvironmentLayers {
            runtime,
            project: published_project,
        })
    }

    async fn ensure_project(
        &self,
        environment: &ProjectEnvironment,
        runtime: &PreparedRuntime,
    ) -> Result<PublishedProjectEnvironment> {
        let destination = self.paths.environments().join(environment.identity());
        if destination.exists() {
            environment.verify_published(&destination)?;
            return environment.published_recipe(destination);
        }
        let _lock = lock(
            &self
                .paths
                .locks()
                .join(format!("project-layer-{}.lock", environment.identity())),
        )?;
        if destination.exists() {
            environment.verify_published(&destination)?;
            return environment.published_recipe(destination);
        }
        if fs::symlink_metadata(&destination).is_ok() {
            bail!("published project environment is not a real directory");
        }

        fs::create_dir_all(self.paths.environments())?;
        let temporary = tempfile::Builder::new()
            .prefix(".project-environment-")
            .tempdir_in(self.paths.environments())?;
        let plan = project_provisioning_plan(temporary.path(), environment)?;
        let unique = SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos();
        let sandbox_name = format!(
            "fortlet-project-provision-{}-{unique:x}",
            std::process::id()
        );
        eprintln!("fortlet: preparing project environment (first use)");
        let sandbox = Sandbox::builder(&sandbox_name)
            .image(runtime.image_reference.clone())
            .pull_policy(PullPolicy::Never)
            .cpus(4)
            .memory(8192)
            .root_disk(8192)
            .volume("/out", |mount| mount.bind(&plan.output))
            .entrypoint([runtime.hold.clone()])
            .create()
            .await
            .context("cannot create project environment provisioning capsule")?;
        let provision = sandbox
            .exec_with(&runtime.shell, |options| {
                options
                    .args(["-eu", "-c", plan.recipe.as_str()])
                    .envs(plan.environment.clone())
            })
            .await
            .context("project environment recipe execution failed");
        cleanup_provisioning_capsule(&sandbox, &sandbox_name).await;
        let output = provision?;
        if !output.status().success {
            let diagnostic = sanitize_diagnostic(&output.stderr().unwrap_or_default());
            if diagnostic.is_empty() {
                bail!("project environment recipe exited {}", output.status().code);
            }
            bail!(
                "project environment recipe exited {}: {diagnostic}",
                output.status().code
            );
        }
        environment.validate_and_mark(temporary.path())?;
        let persisted = temporary.keep();
        fs::rename(&persisted, &destination)
            .context("cannot publish the prepared project environment")?;
        environment.verify_published(&destination)?;
        environment.published_recipe(destination)
    }
}

#[derive(Debug, PartialEq, Eq)]
struct ProjectProvisioningPlan {
    output: PathBuf,
    recipe: String,
    environment: Vec<(String, String)>,
}

fn project_provisioning_plan(
    output: &Path,
    project: &ProjectEnvironment,
) -> Result<ProjectProvisioningPlan> {
    Ok(ProjectProvisioningPlan {
        output: output.to_owned(),
        recipe: project
            .recipe()
            .context("project environment is not an isolated recipe")?
            .to_owned(),
        environment: vec![
            ("FORTLET_OUTPUT".into(), "/out".into()),
            ("FORTLET_TARGET".into(), guest_platform()?.into()),
        ],
    })
}

fn sanitize_diagnostic(value: &str) -> String {
    value
        .chars()
        .take(2048)
        .map(|character| {
            if character == '\n' || character == '\t' || !character.is_control() {
                character
            } else {
                '?'
            }
        })
        .collect::<String>()
        .trim()
        .to_owned()
}

async fn cleanup_provisioning_capsule(sandbox: &Sandbox, name: &str) {
    let _ = sandbox.stop_and_wait().await;
    let _ = Sandbox::remove(name).await;
}

fn lock(path: &Path) -> Result<File> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let file = File::options().create(true).append(true).open(path)?;
    file.lock_exclusive()?;
    Ok(file)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::project::{Project, ProjectIdentity};
    use std::sync::mpsc;
    use std::time::Duration;

    fn project_environment() -> ProjectEnvironment {
        let root = tempfile::tempdir().unwrap().keep().canonicalize().unwrap();
        fs::create_dir(root.join(".fortlet")).unwrap();
        fs::write(
            root.join(".fortlet/environment.json"),
            r#"{"schema":1,"path":[],"environment":{}}"#,
        )
        .unwrap();
        fs::write(root.join(".fortlet/environment.sh"), "mkdir -p /out/bin\n").unwrap();
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
    fn project_provisioning_receives_only_recipe_output_and_fixed_values() {
        let environment = project_environment();
        let plan = project_provisioning_plan(Path::new("/owned/output"), &environment).unwrap();

        assert_eq!(plan.output, Path::new("/owned/output"));
        assert_eq!(plan.recipe, "mkdir -p /out/bin\n");
        assert_eq!(
            plan.environment,
            vec![
                ("FORTLET_OUTPUT".into(), "/out".into()),
                ("FORTLET_TARGET".into(), guest_platform().unwrap().into())
            ]
        );
        assert!(!plan.recipe.contains("/Users/"));
    }

    #[test]
    fn diagnostics_are_bounded_and_strip_control_characters() {
        let diagnostic = format!("bad\u{1b}[31m{}tail", "x".repeat(4096));
        let sanitized = sanitize_diagnostic(&diagnostic);
        assert!(!sanitized.contains('\u{1b}'));
        assert!(sanitized.len() <= 2048);
    }

    #[test]
    fn identity_lock_serializes_concurrent_project_builds() {
        let temporary = tempfile::tempdir().unwrap();
        let lock_path = temporary.path().join("same-identity.lock");
        let first = lock(&lock_path).unwrap();
        let (acquired_sender, acquired_receiver) = mpsc::channel();
        let worker = std::thread::spawn(move || {
            let _second = lock(&lock_path).unwrap();
            acquired_sender.send(()).unwrap();
        });

        assert!(acquired_receiver
            .recv_timeout(Duration::from_millis(50))
            .is_err());
        drop(first);
        acquired_receiver
            .recv_timeout(Duration::from_secs(1))
            .unwrap();
        worker.join().unwrap();
    }
}
