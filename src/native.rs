use std::env;
use std::ffi::OsStr;
use std::fs;
use std::os::unix::fs::{MetadataExt, PermissionsExt};
use std::os::unix::process::CommandExt;
use std::path::{Path, PathBuf};
use std::process::Command;

use anyhow::{bail, Context, Result};

use crate::harness;

pub const SHIM_DIRECTORY_ENV: &str = "FORTLET_SHIM_DIR";

pub fn execute(harness_name: &str, arguments: &[String]) -> Result<()> {
    harness::find(harness_name)
        .with_context(|| "native stage failed; choose a registered harness: codex or tact")?;
    let path = env::var_os("PATH")
        .context("native stage failed; set PATH to include a host harness executable")?;
    let current_executable = env::current_exe()
        .context("native stage failed; reinstall Fortlet so its executable can be resolved")?;
    let shim_directory = env::var_os(SHIM_DIRECTORY_ENV).map(PathBuf::from);
    let executable = resolve(
        harness_name,
        &path,
        shim_directory.as_deref(),
        &current_executable,
    )?;
    let error = Command::new(&executable).args(arguments).exec();
    Err(error).with_context(|| {
        format!(
            "native stage failed; check that {} is executable",
            executable.display()
        )
    })
}

fn resolve(
    harness_name: &str,
    path: &OsStr,
    shim_directory: Option<&Path>,
    current_executable: &Path,
) -> Result<PathBuf> {
    let shim_directory = shim_directory
        .map(|directory| {
            directory.canonicalize().with_context(|| {
                format!(
                    "native stage failed; reinstall Fortlet because its shim directory cannot be resolved: {}",
                    directory.display()
                )
            })
        })
        .transpose()?;
    let current_executable = current_executable.canonicalize().with_context(|| {
        "native stage failed; reinstall Fortlet so its executable can be resolved"
    })?;

    for directory in env::split_paths(path) {
        let candidate = directory.join(harness_name);
        if !is_executable(&candidate) {
            continue;
        }
        let resolved_directory = directory.canonicalize().unwrap_or(directory);
        if shim_directory
            .as_ref()
            .is_some_and(|shim| resolved_directory == *shim)
        {
            continue;
        }
        let resolved = candidate
            .canonicalize()
            .with_context(|| format!("cannot resolve native candidate {}", candidate.display()))?;
        if same_file(&resolved, &current_executable)? {
            continue;
        }
        return Ok(resolved);
    }

    bail!(
        "native stage failed; add a host {harness_name} executable after Fortlet's shim directory on PATH"
    )
}

fn is_executable(path: &Path) -> bool {
    fs::metadata(path)
        .map(|metadata| metadata.is_file() && metadata.permissions().mode() & 0o111 != 0)
        .unwrap_or(false)
}

fn same_file(left: &Path, right: &Path) -> Result<bool> {
    let left = fs::metadata(left)?;
    let right = fs::metadata(right)?;
    Ok(left.dev() == right.dev() && left.ino() == right.ino())
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::os::unix::fs::{symlink, PermissionsExt};

    use super::*;

    #[test]
    fn skips_the_shim_directory_and_preserves_path_order() {
        let temporary = tempfile::tempdir().unwrap();
        let shim_directory = temporary.path().join("shims");
        let first_native = temporary.path().join("first");
        let second_native = temporary.path().join("second");
        fs::create_dir_all(&shim_directory).unwrap();
        fs::create_dir_all(&first_native).unwrap();
        fs::create_dir_all(&second_native).unwrap();
        let current = executable(&temporary.path().join("fortlet"));
        symlink(&current, shim_directory.join("codex")).unwrap();
        let expected = executable(&first_native.join("codex"));
        executable(&second_native.join("codex"));
        let path = env::join_paths([&shim_directory, &first_native, &second_native]).unwrap();

        assert_eq!(
            resolve("codex", &path, Some(&shim_directory), &current).unwrap(),
            expected.canonicalize().unwrap()
        );
    }

    #[test]
    fn skips_a_candidate_that_is_the_running_executable() {
        let temporary = tempfile::tempdir().unwrap();
        let first = temporary.path().join("first");
        let second = temporary.path().join("second");
        fs::create_dir_all(&first).unwrap();
        fs::create_dir_all(&second).unwrap();
        let current = executable(&temporary.path().join("fortlet"));
        symlink(&current, first.join("tact")).unwrap();
        let expected = executable(&second.join("tact"));
        let path = env::join_paths([&first, &second]).unwrap();

        assert_eq!(
            resolve("tact", &path, None, &current).unwrap(),
            expected.canonicalize().unwrap()
        );
    }

    #[test]
    fn missing_native_executable_has_one_action() {
        let temporary = tempfile::tempdir().unwrap();
        let current = executable(&temporary.path().join("fortlet"));
        let error = resolve("codex", OsStr::new(""), None, &current).unwrap_err();

        assert!(error
            .to_string()
            .contains("add a host codex executable after Fortlet's shim directory on PATH"));
    }

    fn executable(path: &Path) -> PathBuf {
        fs::write(path, "executable").unwrap();
        let mut permissions = fs::metadata(path).unwrap().permissions();
        permissions.set_mode(0o755);
        fs::set_permissions(path, permissions).unwrap();
        path.to_owned()
    }
}
