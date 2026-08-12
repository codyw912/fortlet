use std::path::{Path, PathBuf};

use anyhow::{bail, Context, Result};
use clap::{Parser, Subcommand};
use microsandbox::setup;

use crate::auth::{require_outside_mounts, Credentials};
use crate::harness;
use crate::management::{self, StatusRequest, StopRequest};
use crate::native;
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
    Status {
        harness: Option<String>,
        #[arg(long)]
        project: Option<PathBuf>,
        #[arg(long)]
        allow_broad_mount: bool,
    },
    Stop {
        harness: String,
        #[arg(long)]
        project: Option<PathBuf>,
        #[arg(long)]
        allow_broad_mount: bool,
    },
    Native {
        harness: String,
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        arguments: Vec<String>,
    },
}

pub async fn run() -> Result<()> {
    match split_invocation(std::env::args().collect())? {
        Invocation::Fortlet(arguments) => run_command(Cli::parse_from(arguments).command).await,
        Invocation::Shim { harness, arguments } => launch(harness, arguments).await,
    }
}

async fn run_command(command: Command) -> Result<()> {
    match command {
        Command::Doctor => doctor(),
        Command::Run {
            harness,
            project,
            allow_broad_mount,
            arguments,
        } => {
            launch_with_request(LaunchRequest {
                harness,
                project,
                allow_broad_mount,
                arguments: strip_separator(arguments),
            })
            .await
        }
        Command::Native { harness, arguments } => {
            native::execute(&harness, &strip_separator(arguments))
        }
        Command::Status {
            harness,
            project,
            allow_broad_mount,
        } => {
            for line in management::status(StatusRequest {
                harness,
                project,
                allow_broad_mount,
            })
            .await?
            {
                println!("{line}");
            }
            Ok(())
        }
        Command::Stop {
            harness,
            project,
            allow_broad_mount,
        } => {
            println!(
                "{}",
                management::stop(StopRequest {
                    harness,
                    project,
                    allow_broad_mount,
                })
                .await?
            );
            Ok(())
        }
    }
}

async fn launch(harness: String, arguments: Vec<String>) -> Result<()> {
    launch_with_request(LaunchRequest {
        harness,
        project: None,
        allow_broad_mount: false,
        arguments,
    })
    .await
}

async fn launch_with_request(request: LaunchRequest) -> Result<()> {
    let code = session::launch(request).await?;
    if code != 0 {
        std::process::exit(code);
    }
    Ok(())
}

#[derive(Debug, PartialEq, Eq)]
enum Invocation {
    Fortlet(Vec<String>),
    Shim {
        harness: String,
        arguments: Vec<String>,
    },
}

fn split_invocation(arguments: Vec<String>) -> Result<Invocation> {
    let executable = arguments
        .first()
        .context("invocation has no executable name")?;
    let name = Path::new(executable)
        .file_name()
        .and_then(|name| name.to_str())
        .context("invocation executable name is not valid UTF-8")?;
    if name == "fortlet" {
        return Ok(Invocation::Fortlet(arguments));
    }
    if harness::find(name).is_ok() {
        return Ok(Invocation::Shim {
            harness: name.to_owned(),
            arguments: arguments.into_iter().skip(1).collect(),
        });
    }
    bail!("unsupported invocation name {name:?}; run as fortlet, codex, or tact")
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

    #[test]
    fn dispatches_registered_shims_without_consuming_arguments() {
        for harness in ["codex", "tact"] {
            assert_eq!(
                split_invocation(vec![
                    format!("/nix/store/example/shims/{harness}"),
                    "--".into(),
                    "--help".into(),
                ])
                .unwrap(),
                Invocation::Shim {
                    harness: harness.into(),
                    arguments: vec!["--".into(), "--help".into()],
                }
            );
        }
    }

    #[test]
    fn preserves_the_explicit_cli_invocation() {
        let arguments = vec!["/nix/store/example/bin/fortlet".into(), "doctor".into()];
        assert_eq!(
            split_invocation(arguments.clone()).unwrap(),
            Invocation::Fortlet(arguments)
        );
    }

    #[test]
    fn rejects_unknown_multicall_names() {
        let error = split_invocation(vec!["/tmp/not-fortlet".into()]).unwrap_err();
        assert!(error.to_string().contains("unsupported invocation name"));
    }

    #[test]
    fn parses_project_scoped_management_commands() {
        let status = Cli::try_parse_from([
            "fortlet",
            "status",
            "codex",
            "--project",
            "/tmp/project",
            "--allow-broad-mount",
        ])
        .unwrap();
        assert!(matches!(
            status.command,
            Command::Status {
                harness: Some(ref harness),
                project: Some(ref project),
                allow_broad_mount: true,
            } if harness == "codex" && project == Path::new("/tmp/project")
        ));

        let stop = Cli::try_parse_from(["fortlet", "stop", "tact"]).unwrap();
        assert!(matches!(
            stop.command,
            Command::Stop {
                harness,
                project: None,
                allow_broad_mount: false,
            } if harness == "tact"
        ));
    }
}
