mod auth;
mod cli;
mod environment;
mod harness;
mod identity;
mod management;
mod native;
mod nix_provider;
mod paths;
mod preparation;
mod project;
mod project_capability;
mod project_environment;
mod runtime;
mod runtime_artifacts;
mod session;

#[tokio::main]
async fn main() {
    if let Err(error) = cli::run().await {
        eprintln!("fortlet: {error:#}");
        std::process::exit(1);
    }
}
