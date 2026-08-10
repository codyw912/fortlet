use std::path::PathBuf;

use anyhow::{bail, Result};
use clap::{Parser, Subcommand};
use microsandbox::setup;

use crate::auth::{require_outside_mounts, Credentials};
use crate::harness;
use crate::paths::AppPaths;
use crate::project;
use crate::session::{self, LaunchRequest};

#[derive(Parser)]
#[command(
    name = "fortlet",
    about = "Project-scoped isolation for coding-agent CLIs"
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Doctor,
    Run {
        harness: String,
        #[arg(long)]
        project: Option<PathBuf>,
        #[arg(long)]
        allow_broad_mount: bool,
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        arguments: Vec<String>,
    },
}

pub async fn run() -> Result<()> {
    match Cli::parse().command {
        Command::Doctor => doctor(),
        Command::Run {
            harness,
            project,
            allow_broad_mount,
            arguments,
        } => {
            let code = session::launch(LaunchRequest {
                harness,
                project,
                allow_broad_mount,
                arguments: strip_separator(arguments),
            })
            .await?;
            if code != 0 {
                std::process::exit(code);
            }
            Ok(())
        }
    }
}

fn doctor() -> Result<()> {
    let diagnosis = setup::diagnose();
    if !diagnosis.is_healthy() {
        for problem in diagnosis.problems {
            eprintln!("microsandbox: {}", problem.headline);
            for hint in problem.hints {
                eprintln!("  {hint}");
            }
        }
        bail!("MicroSandbox host prerequisites are not healthy");
    }
    let paths = AppPaths::from_environment()?;
    paths.ensure_roots()?;
    let credentials = Credentials::read_default()?;
    let project = project::resolve(&paths, None, false)?;
    require_outside_mounts(&credentials, &[&project.root, &paths.data, &paths.state])?;
    println!("microsandbox: SDK 0.6.8 host ready");
    println!("auth: valid ChatGPT token with at least one hour remaining");
    println!("project: {} ({})", project.root.display(), project.kind);
    println!("boundary: host credential file is outside guest mounts");
    println!(
        "harnesses: {}",
        harness::names().collect::<Vec<_>>().join(", ")
    );
    Ok(())
}

fn strip_separator(mut values: Vec<String>) -> Vec<String> {
    if values.first().is_some_and(|value| value == "--") {
        values.remove(0);
    }
    values
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strips_optional_separator() {
        assert_eq!(
            strip_separator(vec!["--".into(), "--help".into()]),
            vec!["--help"]
        );
    }
}
