const WORKFLOW: &str = include_str!("../.github/workflows/verify.yml");
const PULL_REQUEST_TEMPLATE: &str = include_str!("../.github/pull_request_template.md");

#[test]
fn hosted_verification_is_read_only_and_targets_main_pull_requests() {
    assert!(WORKFLOW.contains("name: Rust verification"));
    assert!(WORKFLOW.contains("pull_request:\n    branches:\n      - main"));
    assert!(WORKFLOW.contains("permissions:\n  contents: read"));
    assert!(WORKFLOW.contains("runs-on: ubuntu-24.04"));
    assert!(WORKFLOW.contains("timeout-minutes: 30"));
    assert!(!WORKFLOW.contains("push:"));
    assert!(!WORKFLOW.contains("pull_request_target:"));
    assert!(!WORKFLOW.contains("secrets."));
    assert!(!WORKFLOW.contains("microsandbox"));
    assert!(!WORKFLOW.contains("fortlet run"));
    assert!(!WORKFLOW.contains("actions/cache"));
    assert!(!WORKFLOW.contains("container:"));
    assert!(!WORKFLOW.contains("nix"));
}

#[test]
fn hosted_verification_pins_tools_and_runs_exact_rust_gates() {
    assert!(WORKFLOW.contains("actions/checkout@3d3c42e5aac5ba805825da76410c181273ba90b1 # v7.0.1"));
    assert!(WORKFLOW.contains("persist-credentials: false"));
    assert!(WORKFLOW
        .contains("rustup toolchain install 1.97.1 --profile minimal --component clippy,rustfmt"));

    let commands = [
        "sudo apt-get update",
        "sudo apt-get install --yes --no-install-recommends libcap-ng-dev",
        "rustup toolchain install 1.97.1 --profile minimal --component clippy,rustfmt",
        "cargo +1.97.1 test",
        "cargo +1.97.1 fmt --all -- --check",
        "cargo +1.97.1 clippy --all-targets --all-features -- -D warnings",
        "cargo +1.97.1 test --test conformance",
    ];
    let actual = WORKFLOW
        .lines()
        .filter_map(|line| {
            let trimmed = line.trim();
            trimmed
                .strip_prefix("run: ")
                .filter(|command| *command != "|")
                .or_else(|| trimmed.starts_with("sudo apt-get ").then_some(trimmed))
        })
        .collect::<Vec<_>>();
    assert_eq!(actual, commands);
    assert_eq!(WORKFLOW.matches("run:").count(), 6);

    let checkout = WORKFLOW
        .find("uses: actions/checkout@")
        .expect("checkout step");
    let linker_dependency = WORKFLOW
        .find("- name: Install Linux linker dependency")
        .expect("Linux dependency step");
    let rust_toolchain = WORKFLOW
        .find("- name: Install pinned Rust toolchain")
        .expect("Rust toolchain step");
    assert!(checkout < linker_dependency);
    assert!(linker_dependency < rust_toolchain);
}

#[test]
fn pull_request_template_requests_the_publication_contract() {
    for required in [
        "## Scope",
        "GOAL:",
        "Governing FIPs:",
        "Included:",
        "Excluded / unchanged:",
        "## Evidence",
        "Reviewed Jujutsu revision:",
        "Local Rust gates:",
        "Local `nix flake check` host and result:",
        "Hosted Rust verification:",
        "## Experiments",
        "Experiment records and outcomes:",
        "## Known limitations",
    ] {
        assert!(PULL_REQUEST_TEMPLATE.contains(required), "{required}");
    }
}
