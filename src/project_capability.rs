use std::fs;
use std::path::PathBuf;

use anyhow::{Context, Result};
use serde::Deserialize;

use crate::paths::AppPaths;
use crate::project;
use crate::project_environment::{guest_system, ProjectEnvironment};

const RECIPE_MARKER: &str = ".fortlet-project.json";

pub struct PlanRequest {
    pub project: Option<PathBuf>,
    pub allow_broad_mount: bool,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ProjectCapabilityPlan {
    project_kind: &'static str,
    target: &'static str,
    provider: &'static str,
    declarations: Vec<&'static str>,
    selection: Option<String>,
    preparation: &'static str,
    activation_names: Vec<String>,
    cache_classes: Vec<&'static str>,
    unsupported: Vec<&'static str>,
}

#[derive(Deserialize)]
struct ResolvedRecordHeader {
    declaration_identity: String,
    environment: std::collections::BTreeMap<String, String>,
}

pub fn inspect(request: PlanRequest) -> Result<Vec<String>> {
    let paths = AppPaths::from_environment()
        .context("project stage failed; set HOME to a readable user directory and retry")?;
    let project = project::resolve_read_only(
        &paths,
        request.project.as_deref(),
        request.allow_broad_mount,
    )
    .context(
        "project stage failed; run from a readable project directory or pass --project <path>",
    )?;
    let environment = ProjectEnvironment::discover(&project).context(
        "project-environment stage failed; fix or remove .fortlet/environment.json and retry",
    )?;
    Ok(
        ProjectCapabilityPlan::from_environment(&paths, project.kind, environment.as_ref())
            .render(),
    )
}

impl ProjectCapabilityPlan {
    fn from_environment(
        paths: &AppPaths,
        project_kind: &'static str,
        environment: Option<&ProjectEnvironment>,
    ) -> Self {
        let Some(environment) = environment else {
            return Self {
                project_kind,
                target: guest_system().expect("supported build architecture"),
                provider: "none",
                declarations: Vec::new(),
                selection: None,
                preparation: "not-required",
                activation_names: Vec::new(),
                cache_classes: vec!["harness-home"],
                unsupported: Vec::new(),
            };
        };

        if environment.is_recipe() {
            let destination = paths.environments().join(environment.identity());
            Self {
                project_kind,
                target: guest_system().expect("supported build architecture"),
                provider: "isolated-recipe",
                declarations: vec![".fortlet/environment.json", ".fortlet/environment.sh"],
                selection: None,
                preparation: marker_state(&destination.join(RECIPE_MARKER)),
                activation_names: environment
                    .activation_names()
                    .into_iter()
                    .map(str::to_owned)
                    .collect(),
                cache_classes: vec!["immutable-project-layer", "harness-home"],
                unsupported: vec!["authenticated-input", "service"],
            }
        } else {
            let project_identity = environment
                .project_identity()
                .expect("Nix environment has a project identity");
            let (preparation, activation_names) = resolved_state(
                &paths.provider_record(project_identity),
                environment.identity(),
            );
            Self {
                project_kind,
                target: guest_system().expect("supported build architecture"),
                provider: "nix-dev-shell",
                declarations: vec![".fortlet/environment.json", "flake.nix", "flake.lock"],
                selection: environment.provider_name().map(str::to_owned),
                preparation,
                activation_names,
                cache_classes: vec!["project-nix-store", "harness-home"],
                unsupported: vec![
                    "executable-activation",
                    "authenticated-input",
                    "service",
                    "host-requirement",
                ],
            }
        }
    }

    fn render(&self) -> Vec<String> {
        vec![
            format!("project\t{}", self.project_kind),
            format!("target\t{}", self.target),
            format!("provider\t{}", self.provider),
            format!("declarations\t{}", joined(&self.declarations)),
            format!("selection\t{}", self.selection.as_deref().unwrap_or("none")),
            format!("preparation\t{}", self.preparation),
            format!("activation\t{}", joined(&self.activation_names)),
            format!("caches\t{}", joined(&self.cache_classes)),
            "identity-projection\tunchecked".to_owned(),
            format!("unsupported\t{}", joined(&self.unsupported)),
        ]
    }
}

fn marker_state(path: &std::path::Path) -> &'static str {
    match fs::symlink_metadata(path) {
        Ok(metadata) if metadata.is_file() && !metadata.file_type().is_symlink() => "prepared",
        Ok(_) => "invalid",
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => "unprepared",
        Err(_) => "invalid",
    }
}

fn resolved_state(path: &std::path::Path, identity: &str) -> (&'static str, Vec<String>) {
    let metadata = match fs::symlink_metadata(path) {
        Ok(metadata) => metadata,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            return ("unprepared", Vec::new())
        }
        Err(_) => return ("invalid", Vec::new()),
    };
    if !metadata.is_file() || metadata.file_type().is_symlink() {
        return ("invalid", Vec::new());
    }
    let Ok(bytes) = fs::read(path) else {
        return ("invalid", Vec::new());
    };
    let Ok(record) = serde_json::from_slice::<ResolvedRecordHeader>(&bytes) else {
        return ("invalid", Vec::new());
    };
    if record.declaration_identity == identity {
        ("prepared", record.environment.into_keys().collect())
    } else {
        ("stale", Vec::new())
    }
}

fn joined<T: AsRef<str>>(values: &[T]) -> String {
    if values.is_empty() {
        "none".to_owned()
    } else {
        values
            .iter()
            .map(AsRef::as_ref)
            .collect::<Vec<_>>()
            .join(",")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rendering_is_stable_and_contains_no_values_or_internal_paths() {
        let plan = ProjectCapabilityPlan {
            project_kind: "git",
            target: "aarch64-linux",
            provider: "nix-dev-shell",
            declarations: vec![".fortlet/environment.json", "flake.nix", "flake.lock"],
            selection: Some("default".into()),
            preparation: "unprepared",
            activation_names: vec!["GOFLAGS".into()],
            cache_classes: vec!["project-nix-store", "harness-home"],
            unsupported: vec!["service"],
        };

        let rendered = plan.render().join("\n");
        assert!(rendered.contains("selection\tdefault"));
        assert!(rendered.contains("activation\tGOFLAGS"));
        assert!(!rendered.contains("/Users/"));
        assert!(!rendered.contains('='));
    }
}
