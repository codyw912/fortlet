use std::fs;
use std::path::{Path, PathBuf};

use sha2::{Digest, Sha256};

const CONTRACT: &str = "fip0013-2";

pub fn write_artifacts(root: &Path) {
    let runtime = runtime_manifest();
    write_json(&root.join("runtime/manifest.json"), &runtime);
    for (name, version) in [("codex", "0.147.0"), ("tact", "0.3.7")] {
        write_json(
            &root.join("harnesses").join(name).join("manifest.json"),
            &harness_manifest(name, version),
        );
    }
}

pub fn seed_prepared(data: &Path, project: &Path, artifacts: &Path, harness: &str) {
    write_artifacts(artifacts);
    let store = seed_runtime(data, project);
    seed_harness(&store, harness);
}

pub fn seed_runtime(data: &Path, project: &Path) -> PathBuf {
    let store = prepared_store(data, project);
    fs::create_dir_all(store.join("nix/store/runtime")).unwrap();
    write_json(&store.join("runtime.json"), &runtime_manifest());
    store
}

pub fn seed_harness(store: &Path, harness: &str) {
    let version = match harness {
        "codex" => "0.147.0",
        "tact" => "0.3.7",
        _ => panic!("test fixture has no version for {harness}"),
    };
    let executable = store
        .join("nix/store")
        .join(harness)
        .join("bin")
        .join(harness);
    fs::create_dir_all(executable.parent().unwrap()).unwrap();
    fs::write(executable, "fixture").unwrap();
    write_json(
        &store.join("harnesses").join(format!("{harness}.json")),
        &harness_manifest(harness, version),
    );
}

pub fn prepared_store(data: &Path, project: &Path) -> PathBuf {
    data.join("fortlet/project-stores")
        .join(project_identity(project))
}

fn project_identity(project: &Path) -> String {
    let canonical = project.canonicalize().unwrap();
    let digest = Sha256::digest(canonical.as_os_str().as_encoded_bytes());
    hex::encode(&digest[..8])
}

fn runtime_manifest() -> serde_json::Value {
    serde_json::json!({
        "schema": 1,
        "contract": CONTRACT,
        "system": guest_system(),
        "architecture": guest_architecture(),
        "image_reference": format!("fortlet-runtime:{CONTRACT}-{}", guest_system()),
        "image_digest": format!("sha256:{}", "0".repeat(64)),
        "image_archive_sha256": "0".repeat(64),
        "seed_archive_sha256": "0".repeat(64),
        "runtime_root": "/nix/store/runtime",
        "nix_version": "2.34.8",
        "nix": "/nix/store/runtime/bin/nix",
        "nix_store": "/nix/store/runtime/bin/nix-store",
        "shell": "/nix/store/runtime/bin/bash",
        "hold": "/nix/store/runtime/bin/fortlet-hold",
        "managed_bash_env": "/nix/store/runtime/etc/managed-bash-env",
        "runtime_library_path": "/nix/store/runtime/lib",
        "ca_bundle": "/etc/ssl/certs/ca-certificates.crt",
        "ca_bundle_source": "/nix/store/runtime/etc/ssl/certs/ca-bundle.crt",
        "store_paths": ["/nix/store/runtime"]
    })
}

fn harness_manifest(name: &str, version: &str) -> serde_json::Value {
    serde_json::json!({
        "schema": 1,
        "contract": CONTRACT,
        "name": name,
        "version": version,
        "system": guest_system(),
        "executable": format!("/nix/store/{name}/bin/{name}"),
        "archive_sha256": "0".repeat(64),
        "store_paths": [format!("/nix/store/{name}")]
    })
}

fn write_json(path: &Path, value: &serde_json::Value) {
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(path, serde_json::to_vec(value).unwrap()).unwrap();
}

#[cfg(target_arch = "aarch64")]
fn guest_system() -> &'static str {
    "aarch64-linux"
}

#[cfg(target_arch = "x86_64")]
fn guest_system() -> &'static str {
    "x86_64-linux"
}

#[cfg(target_arch = "aarch64")]
fn guest_architecture() -> &'static str {
    "arm64"
}

#[cfg(target_arch = "x86_64")]
fn guest_architecture() -> &'static str {
    "amd64"
}
