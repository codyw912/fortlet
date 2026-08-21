use std::fs::{self, File};
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use anyhow::{bail, Context, Result};
use fs2::FileExt;
use microsandbox::Sandbox;

use crate::harness::Harness;
use crate::nix_provider;
use crate::paths::AppPaths;
use crate::project::Project;
use crate::project_environment::{guest_platform, ProjectEnvironment, PublishedProjectEnvironment};

pub const BASE_IMAGE: &str = "node:24-bookworm";
const BASE_TOOLS_VERSION: &str = "bookworm-4";
const BUBBLEWRAP_VERSION: &str = "0.8.0-2+deb12u1";
const HOLD_SCRIPT: &str = "trap 'exit 0' TERM INT; while :; do sleep 3600 & wait $!; done";
pub const MANAGED_BASH_ENV: &str = "/opt/fortlet/base/etc/fortlet/bash-env";
const CERTIFICATE_BUNDLE_FUNCTION: &str = r#"build_certificate_bundle() {
  root="$1"
  mkdir -p "$root/etc/ssl/certs"
  bundle="$root/etc/ssl/certs/ca-certificates.crt"
  : > "$bundle"
  while IFS= read -r certificate || [ -n "$certificate" ]; do
    case "$certificate" in
      ''|'#'*|'!'*) continue ;;
      /*|..|../*|*/../*|*/..) printf 'invalid CA certificate path: %s\n' "$certificate" >&2; return 1 ;;
    esac
    cat "$root/usr/share/ca-certificates/$certificate" >> "$bundle"
    printf '\n' >> "$bundle"
  done < "$root/etc/ca-certificates.conf"
  test -s "$bundle"
}"#;

pub struct EnvironmentStore<'a> {
    paths: &'a AppPaths,
}

pub struct EnvironmentLayers {
    pub harness: PathBuf,
    pub base: PathBuf,
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
        let base_script = base_provision_script();
        let base = self
            .ensure_layer(
                "_base",
                BASE_TOOLS_VERSION,
                ".fortlet-base.json",
                &base_script,
                "base environment",
            )
            .await?;
        let harness_label = format!("{} environment", harness.name());
        let harness_path = self
            .ensure_layer(
                harness.name(),
                harness.version(),
                ".fortlet-tool.json",
                &harness.provision_script(),
                &harness_label,
            )
            .await?;
        let project = match project_environment {
            Some(environment) if environment.is_recipe() => {
                Some(self.ensure_project(environment).await?)
            }
            Some(environment) => {
                Some(nix_provider::ensure(self.paths, project, environment).await?)
            }
            None => None,
        };
        Ok(EnvironmentLayers {
            harness: harness_path,
            base,
            project,
        })
    }

    async fn ensure_project(
        &self,
        environment: &ProjectEnvironment,
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
            .image(BASE_IMAGE)
            .cpus(4)
            .memory(8192)
            .root_disk(8192)
            .volume("/out", |mount| mount.bind(&plan.output))
            .script("hold", HOLD_SCRIPT)
            .entrypoint(["hold"])
            .create()
            .await
            .context("cannot create project environment provisioning capsule")?;
        let provision = sandbox
            .exec_with("/bin/sh", |options| {
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
        fs::rename(temporary.path(), &destination).with_context(|| {
            format!(
                "cannot publish project environment into {}",
                self.paths.environments().display()
            )
        })?;
        environment.verify_published(&destination)?;
        environment.published_recipe(destination)
    }

    async fn ensure_layer(
        &self,
        name: &str,
        version: &str,
        marker: &str,
        script: &str,
        label: &str,
    ) -> Result<PathBuf> {
        let destination = self.paths.tools().join(name).join(version);
        if layer_marker_matches(&destination, marker, name, version, label)? {
            return Ok(destination);
        }
        let _lock = lock(&self.paths.locks().join(format!("layer-{name}.lock")))?;
        if layer_marker_matches(&destination, marker, name, version, label)? {
            return Ok(destination);
        }
        if destination.exists() {
            bail!(
                "incomplete environment layer at {}; remove it and retry",
                destination.display()
            );
        }
        fs::create_dir_all(self.paths.tools())?;
        let temporary = tempfile::Builder::new()
            .prefix(&format!(".{name}-{version}-"))
            .tempdir_in(self.paths.tools())?;
        let unique = SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos();
        let sandbox_name = format!("fortlet-provision-{}-{unique:x}", std::process::id());
        eprintln!("fortlet: preparing {label} (first use)");
        let sandbox = Sandbox::builder(&sandbox_name)
            .image(BASE_IMAGE)
            .cpus(2)
            .memory(2048)
            .volume("/out", |mount| mount.bind(temporary.path()))
            .script("hold", HOLD_SCRIPT)
            .entrypoint(["hold"])
            .create()
            .await
            .with_context(|| format!("cannot create {label} provisioning capsule"))?;
        let provision = sandbox
            .exec("/bin/sh", ["-c", script])
            .await
            .context("environment provisioning failed");
        cleanup_provisioning_capsule(&sandbox, &sandbox_name).await;
        let output = provision?;
        if !output.status().success {
            let diagnostic = sanitize_diagnostic(&output.stderr().unwrap_or_default());
            if diagnostic.is_empty() {
                bail!("environment provisioning exited {}", output.status().code);
            }
            bail!(
                "environment provisioning exited {}: {diagnostic}",
                output.status().code
            );
        }
        fs::write(
            temporary.path().join(marker),
            format!(
                "{}\n",
                serde_json::json!({
                    "name": name,
                    "version": version,
                    "image": BASE_IMAGE,
                })
            ),
        )?;
        fs::create_dir_all(destination.parent().context("layer has no parent")?)?;
        let persisted = temporary.keep();
        fs::rename(&persisted, &destination).with_context(|| {
            format!(
                "cannot publish environment layer {} to {}",
                persisted.display(),
                destination.display()
            )
        })?;
        Ok(destination)
    }
}

fn base_provision_script() -> String {
    format!(
        r#"set -eu
{CERTIFICATE_BUNDLE_FUNCTION}
step=temporary-directory
temporary="$(mktemp -d)"
cleanup() {{
  status=$?
  trap - EXIT
  rm -rf "$temporary"
  if [ "$status" -ne 0 ]; then
    printf 'fortlet: base provisioning step failed: %s\n' "$step" >&2
  fi
  exit "$status"
}}
trap cleanup EXIT
chmod 777 "$temporary"
cd "$temporary"
step=package-index
apt-get update -qq
step=package-download
apt-get download "bubblewrap={BUBBLEWRAP_VERSION}" git ca-certificates curl tar xz-utils
step=package-extraction
for package in ./*.deb; do
  dpkg-deb -x "$package" /out
done
step=bubblewrap-validation
/out/usr/bin/bwrap --version
step=git-validation
/out/usr/bin/git --version
step=curl-validation
/out/usr/bin/curl --version
step=tar-validation
/out/bin/tar --version
step=xz-validation
/out/usr/bin/xz --version
step=certificate-bundle
build_certificate_bundle /out
step=managed-shell-activation
mkdir -p /out/etc/fortlet
cat > /out/etc/fortlet/bash-env <<'FORTLET_BASH_ENV'
case ":$PATH:" in
  *:/.msb/scripts:*) PATH="/.msb/scripts:$FORTLET_MANAGED_PATH" ;;
  *) PATH="$FORTLET_MANAGED_PATH" ;;
esac
export PATH
FORTLET_BASH_ENV
chmod 0444 /out/etc/fortlet/bash-env
"#
    )
}

fn layer_marker_matches(
    destination: &Path,
    marker: &str,
    name: &str,
    version: &str,
    label: &str,
) -> Result<bool> {
    let marker_path = destination.join(marker);
    let metadata = match fs::symlink_metadata(&marker_path) {
        Ok(metadata) => metadata,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(false),
        Err(error) => return Err(error.into()),
    };
    if !metadata.is_file() || metadata.file_type().is_symlink() {
        bail!("published {label} marker is not a regular file");
    }
    let actual: serde_json::Value = serde_json::from_slice(&fs::read(&marker_path)?)
        .with_context(|| format!("published {label} marker is invalid"))?;
    let expected = serde_json::json!({
        "name": name,
        "version": version,
        "image": BASE_IMAGE,
    });
    if actual != expected {
        bail!("published {label} marker does not match its inputs");
    }
    Ok(true)
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
    use std::process::Command;
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
    fn base_layer_installs_the_managed_bash_environment() {
        let script = base_provision_script();

        assert_eq!(BASE_TOOLS_VERSION, "bookworm-4");
        for required in [
            "printf 'fortlet: base provisioning step failed: %s\\n' \"$step\" >&2",
            "step=certificate-bundle",
            "build_certificate_bundle /out",
            "cat > /out/etc/fortlet/bash-env <<'FORTLET_BASH_ENV'",
            "*:/.msb/scripts:*) PATH=\"/.msb/scripts:$FORTLET_MANAGED_PATH\" ;;",
            "*) PATH=\"$FORTLET_MANAGED_PATH\" ;;",
            "chmod 0444 /out/etc/fortlet/bash-env",
            "/out/bin/tar --version",
        ] {
            assert!(script.contains(required), "{required}");
        }
        assert!(!script.contains("test -f /out/etc/ssl/certs/ca-certificates.crt"));
    }

    #[test]
    fn certificate_bundle_uses_enabled_entries_and_rejects_traversal() {
        let root = tempfile::tempdir().unwrap();
        let certificates = root.path().join("usr/share/ca-certificates/mozilla");
        fs::create_dir_all(&certificates).unwrap();
        fs::create_dir_all(root.path().join("etc")).unwrap();
        fs::write(certificates.join("alpha.crt"), "alpha").unwrap();
        fs::write(certificates.join("disabled.crt"), "disabled").unwrap();
        fs::write(certificates.join("omega.crt"), "omega").unwrap();
        fs::write(
            root.path().join("etc/ca-certificates.conf"),
            "# trusted certificates\nmozilla/alpha.crt\n!mozilla/disabled.crt\nmozilla/omega.crt\n",
        )
        .unwrap();

        let script = format!("{CERTIFICATE_BUNDLE_FUNCTION}\nbuild_certificate_bundle \"$1\"\n");
        let output = Command::new("/bin/sh")
            .args(["-eu", "-c", &script, "fortlet-test"])
            .arg(root.path())
            .output()
            .unwrap();
        assert!(output.status.success(), "{:?}", output.stderr);
        assert_eq!(
            fs::read(root.path().join("etc/ssl/certs/ca-certificates.crt")).unwrap(),
            b"alpha\nomega\n"
        );

        fs::write(
            root.path().join("etc/ca-certificates.conf"),
            "../outside.crt\n",
        )
        .unwrap();
        let output = Command::new("/bin/sh")
            .args(["-eu", "-c", &script, "fortlet-test"])
            .arg(root.path())
            .output()
            .unwrap();
        assert!(!output.status.success());
        assert!(String::from_utf8_lossy(&output.stderr).contains("invalid CA certificate path"));
    }

    #[test]
    fn diagnostics_are_bounded_and_strip_control_characters() {
        let diagnostic = format!("bad\u{1b}[31m{}tail", "x".repeat(4096));
        let sanitized = sanitize_diagnostic(&diagnostic);
        assert!(!sanitized.contains('\u{1b}'));
        assert!(sanitized.len() <= 2048);
    }

    #[test]
    fn tool_layer_markers_are_regular_and_bound_to_their_inputs() {
        let temporary = tempfile::tempdir().unwrap();
        let destination = temporary.path().join("codex/0.147.0");
        fs::create_dir_all(&destination).unwrap();
        fs::write(
            destination.join(".fortlet-tool.json"),
            serde_json::to_vec(&serde_json::json!({
                "name": "codex",
                "version": "0.147.0",
                "image": BASE_IMAGE,
            }))
            .unwrap(),
        )
        .unwrap();

        assert!(layer_marker_matches(
            &destination,
            ".fortlet-tool.json",
            "codex",
            "0.147.0",
            "codex environment"
        )
        .unwrap());

        fs::write(destination.join(".fortlet-tool.json"), "{}").unwrap();
        let error = layer_marker_matches(
            &destination,
            ".fortlet-tool.json",
            "codex",
            "0.147.0",
            "codex environment",
        )
        .unwrap_err();
        assert!(error.to_string().contains("does not match its inputs"));
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
