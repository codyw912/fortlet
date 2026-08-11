mod auth;
mod cli;
mod environment;
mod harness;
mod native;
mod paths;
mod project;
mod runtime;
mod session;

#[tokio::main]
async fn main() {
    if let Err(error) = cli::run().await {
        eprintln!("fortlet: {error:#}");
        std::process::exit(1);
    }
}
