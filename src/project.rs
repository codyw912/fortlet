use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{bail, Context, Result};
use sha2::{Digest, Sha256};

use crate::paths::AppPaths;

const MARKERS: &[(&str, &[&str])] = &[
    ("jj", &[".jj"]),
    ("git", &[".git"]),
    ("devenv", &["devenv.nix", "devenv.yaml", "devenv.yml"]),
    ("flake", &["flake.nix"]),
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectIdentity(String);

impl ProjectIdentity {
    pub fn from_local_root(root: &Path) -> Self {
        let digest = Sha256::digest(root.as_os_str().as_encoded_bytes());
        Self(hex::encode(&digest[..8]))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone)]
pub struct Project {
    pub identity: ProjectIdentity,
    pub root: PathBuf,
    pub cwd: PathBuf,
    pub kind: &'static str,
    pub scratch: bool,
}

pub fn resolve(
    paths: &AppPaths,
    explicit: Option<&Path>,
    allow_broad_mount: bool,
) -> Result<Project> {
    resolve_with_scratch(paths, explicit, allow_broad_mount, true)
}

pub fn resolve_read_only(
    paths: &AppPaths,
    explicit: Option<&Path>,
    allow_broad_mount: bool,
) -> Result<Project> {
    resolve_with_scratch(paths, explicit, allow_broad_mount, false)
}

fn resolve_with_scratch(
    paths: &AppPaths,
    explicit: Option<&Path>,
    allow_broad_mount: bool,
    create_scratch: bool,
) -> Result<Project> {
    let cwd = env::current_dir().context("cannot read current directory")?;
    let home = env::var_os("HOME")
        .map(PathBuf::from)
        .context("HOME is not set")?;
    resolve_from_mode(
        paths,
        &cwd,
        &home,
        explicit,
        allow_broad_mount,
        create_scratch,
    )
}

#[cfg(test)]
fn resolve_from(
    paths: &AppPaths,
    cwd: &Path,
    home: &Path,
    explicit: Option<&Path>,
    allow_broad_mount: bool,
) -> Result<Project> {
    resolve_from_mode(paths, cwd, home, explicit, allow_broad_mount, true)
}

fn resolve_from_mode(
    paths: &AppPaths,
    cwd: &Path,
    home: &Path,
    explicit: Option<&Path>,
    allow_broad_mount: bool,
    create_scratch: bool,
) -> Result<Project> {
    let cwd = canonical_directory(cwd)?;
    let home = canonical_directory(home)?;
    let (root, cwd, kind) = if let Some(root) = explicit {
        let root = canonical_directory(root)?;
        let cwd = if cwd.starts_with(&root) {
            cwd
        } else {
            root.clone()
        };
        (root, cwd, "explicit")
    } else {
        let (root, kind) = discover_root(&cwd);
        (root, cwd, kind)
    };

    if !allow_broad_mount && is_broad_root(&root, &home) {
        return scratch_project(paths, create_scratch);
    }
    Ok(project(root, cwd, kind, false))
}

fn project(root: PathBuf, cwd: PathBuf, kind: &'static str, scratch: bool) -> Project {
    Project {
        identity: ProjectIdentity::from_local_root(&root),
        root,
        cwd,
        kind,
        scratch,
    }
}

fn discover_root(cwd: &Path) -> (PathBuf, &'static str) {
    for (kind, markers) in MARKERS {
        for candidate in cwd.ancestors() {
            if markers.iter().any(|marker| candidate.join(marker).exists()) {
                return (candidate.to_path_buf(), kind);
            }
        }
    }
    (cwd.to_path_buf(), "directory")
}

fn is_broad_root(root: &Path, home: &Path) -> bool {
    root == home || root.parent().is_none()
}

fn scratch_project(paths: &AppPaths, create: bool) -> Result<Project> {
    let root = paths.scratch();
    if create {
        fs::create_dir_all(&root)
            .with_context(|| format!("cannot create scratch workspace {}", root.display()))?;
    }
    let root = if root.exists() {
        canonical_directory(&root)?
    } else {
        root
    };
    Ok(project(root.clone(), root, "scratch", true))
}

fn canonical_directory(path: &Path) -> Result<PathBuf> {
    let resolved = path
        .canonicalize()
        .with_context(|| format!("cannot resolve project path {}", path.display()))?;
    if !resolved.is_dir() {
        bail!("project path is not a directory: {}", resolved.display());
    }
    Ok(resolved)
}

#[cfg(test)]
mod tests {
    use std::os::unix::fs::symlink;

    use super::*;

    fn test_paths(root: &Path) -> AppPaths {
        AppPaths {
            state: root.join("state"),
            data: root.join("data"),
        }
    }

    fn create_marker(root: &Path, marker: &str) {
        let path = root.join(marker);
        if marker.starts_with('.') {
            fs::create_dir_all(path).unwrap();
        } else {
            fs::write(path, "").unwrap();
        }
    }

    #[test]
    fn unmarked_directory_resolves_to_itself() {
        let temporary = tempfile::tempdir().unwrap();
        let cwd = temporary.path().join("workspace");
        let home = temporary.path().join("home");
        fs::create_dir_all(&cwd).unwrap();
        fs::create_dir_all(&home).unwrap();
        let paths = test_paths(temporary.path());

        let resolved = resolve_from(&paths, &cwd, &home, None, false).unwrap();

        assert_eq!(resolved.root, cwd.canonicalize().unwrap());
        assert_eq!(resolved.cwd, cwd.canonicalize().unwrap());
        assert_eq!(resolved.kind, "directory");
        assert!(!resolved.scratch);
    }

    #[test]
    fn explicit_root_preserves_a_nested_working_directory() {
        let temporary = tempfile::tempdir().unwrap();
        let root = temporary.path().join("workspace");
        let cwd = root.join("nested");
        let home = temporary.path().join("home");
        fs::create_dir_all(&cwd).unwrap();
        fs::create_dir_all(&home).unwrap();

        let resolved = resolve_from(
            &test_paths(temporary.path()),
            &cwd,
            &home,
            Some(&root),
            false,
        )
        .unwrap();

        assert_eq!(resolved.root, root.canonicalize().unwrap());
        assert_eq!(resolved.cwd, cwd.canonicalize().unwrap());
        assert_eq!(resolved.kind, "explicit");
        assert!(!resolved.scratch);
    }

    #[test]
    fn explicit_root_replaces_an_unrelated_working_directory() {
        let temporary = tempfile::tempdir().unwrap();
        let root = temporary.path().join("workspace");
        let cwd = temporary.path().join("elsewhere");
        let home = temporary.path().join("home");
        for directory in [&root, &cwd, &home] {
            fs::create_dir_all(directory).unwrap();
        }

        let resolved = resolve_from(
            &test_paths(temporary.path()),
            &cwd,
            &home,
            Some(&root),
            false,
        )
        .unwrap();

        assert_eq!(resolved.root, root.canonicalize().unwrap());
        assert_eq!(resolved.cwd, root.canonicalize().unwrap());
        assert_eq!(resolved.kind, "explicit");
    }

    #[test]
    fn discovery_uses_the_nearest_root_in_each_marker_class() {
        for (marker, kind) in [
            (".jj", "jj"),
            (".git", "git"),
            ("devenv.nix", "devenv"),
            ("devenv.yaml", "devenv"),
            ("devenv.yml", "devenv"),
            ("flake.nix", "flake"),
        ] {
            let temporary = tempfile::tempdir().unwrap();
            let outer = temporary.path().join("outer");
            let inner = outer.join("inner");
            let cwd = inner.join("nested");
            let home = temporary.path().join("home");
            fs::create_dir_all(&cwd).unwrap();
            fs::create_dir_all(&home).unwrap();
            create_marker(&outer, marker);
            create_marker(&inner, marker);

            let resolved =
                resolve_from(&test_paths(temporary.path()), &cwd, &home, None, false).unwrap();

            assert_eq!(resolved.root, inner.canonicalize().unwrap(), "{marker}");
            assert_eq!(resolved.cwd, cwd.canonicalize().unwrap(), "{marker}");
            assert_eq!(resolved.kind, kind, "{marker}");
        }
    }

    #[test]
    fn discovery_applies_marker_class_priority_before_proximity() {
        for (preferred_marker, preferred_kind, nearer_marker) in [
            (".jj", "jj", ".git"),
            (".git", "git", "devenv.nix"),
            ("devenv.nix", "devenv", "flake.nix"),
        ] {
            let temporary = tempfile::tempdir().unwrap();
            let preferred_root = temporary.path().join("outer");
            let nearer_root = preferred_root.join("inner");
            let cwd = nearer_root.join("nested");
            let home = temporary.path().join("home");
            fs::create_dir_all(&cwd).unwrap();
            fs::create_dir_all(&home).unwrap();
            create_marker(&preferred_root, preferred_marker);
            create_marker(&nearer_root, nearer_marker);

            let resolved =
                resolve_from(&test_paths(temporary.path()), &cwd, &home, None, false).unwrap();

            assert_eq!(
                resolved.root,
                preferred_root.canonicalize().unwrap(),
                "{preferred_marker} before {nearer_marker}"
            );
            assert_eq!(resolved.kind, preferred_kind);
        }
    }

    #[test]
    fn canonical_root_and_working_directory_define_identity() {
        let temporary = tempfile::tempdir().unwrap();
        let real_root = temporary.path().join("real-workspace");
        let real_cwd = real_root.join("nested");
        let linked_root = temporary.path().join("linked-workspace");
        let linked_cwd = linked_root.join("nested");
        let home = temporary.path().join("home");
        fs::create_dir_all(&real_cwd).unwrap();
        fs::create_dir_all(&home).unwrap();
        symlink(&real_root, &linked_root).unwrap();

        let resolved = resolve_from(
            &test_paths(temporary.path()),
            &linked_cwd,
            &home,
            Some(&linked_root),
            false,
        )
        .unwrap();
        let canonical_root = real_root.canonicalize().unwrap();

        assert_eq!(resolved.root, canonical_root);
        assert_eq!(resolved.cwd, real_cwd.canonicalize().unwrap());
        assert_eq!(
            resolved.identity,
            ProjectIdentity::from_local_root(&canonical_root)
        );
    }

    #[test]
    fn explicit_home_root_uses_persistent_scratch_without_override() {
        let temporary = tempfile::tempdir().unwrap();
        let home = temporary.path().join("home");
        let cwd = home.join("nested");
        fs::create_dir_all(&cwd).unwrap();
        let paths = test_paths(temporary.path());

        let resolved = resolve_from(&paths, &cwd, &home, Some(&home), false).unwrap();
        let scratch = paths.scratch().canonicalize().unwrap();

        assert_eq!(resolved.root, scratch);
        assert_eq!(resolved.cwd, scratch);
        assert_eq!(resolved.kind, "scratch");
        assert!(resolved.scratch);
        assert_eq!(
            resolved.identity,
            ProjectIdentity::from_local_root(&scratch)
        );
    }

    #[test]
    fn marker_selected_home_root_uses_persistent_scratch() {
        let temporary = tempfile::tempdir().unwrap();
        let home = temporary.path().join("home");
        let cwd = home.join("project/nested");
        fs::create_dir_all(&cwd).unwrap();
        create_marker(&home, ".jj");
        let paths = test_paths(temporary.path());

        let resolved = resolve_from(&paths, &cwd, &home, None, false).unwrap();
        let scratch = paths.scratch().canonicalize().unwrap();

        assert_eq!(resolved.root, scratch);
        assert_eq!(resolved.cwd, scratch);
        assert_eq!(resolved.kind, "scratch");
        assert!(resolved.scratch);
    }

    #[test]
    fn direct_home_and_filesystem_root_use_persistent_scratch() {
        let temporary = tempfile::tempdir().unwrap();
        let home = temporary.path().join("home");
        fs::create_dir_all(&home).unwrap();

        for cwd in [&home, Path::new("/")] {
            let case_root = tempfile::tempdir().unwrap();
            let paths = test_paths(case_root.path());

            let resolved = resolve_from(&paths, cwd, &home, None, false).unwrap();
            let scratch = paths.scratch().canonicalize().unwrap();

            assert_eq!(resolved.root, scratch, "{}", cwd.display());
            assert_eq!(resolved.cwd, scratch, "{}", cwd.display());
            assert!(resolved.scratch, "{}", cwd.display());
        }
    }

    #[test]
    fn read_only_resolution_does_not_create_broad_root_scratch() {
        let temporary = tempfile::tempdir().unwrap();
        let paths = test_paths(temporary.path());
        let home = temporary.path().join("home");
        fs::create_dir(&home).unwrap();

        let resolved = resolve_from_mode(&paths, &home, &home, None, false, false).unwrap();

        assert!(resolved.scratch);
        assert_eq!(resolved.root, paths.scratch());
        assert!(!paths.scratch().exists());
    }

    #[test]
    fn explicit_filesystem_root_uses_persistent_scratch_without_override() {
        let temporary = tempfile::tempdir().unwrap();
        let cwd = temporary.path().join("workspace");
        let home = temporary.path().join("home");
        fs::create_dir_all(&cwd).unwrap();
        fs::create_dir_all(&home).unwrap();
        let paths = test_paths(temporary.path());

        let resolved = resolve_from(&paths, &cwd, &home, Some(Path::new("/")), false).unwrap();
        let scratch = paths.scratch().canonicalize().unwrap();

        assert_eq!(resolved.root, scratch);
        assert_eq!(resolved.cwd, scratch);
        assert!(resolved.scratch);
    }

    #[test]
    fn broad_mount_override_preserves_explicit_root_and_working_directory() {
        let temporary = tempfile::tempdir().unwrap();
        let home = temporary.path().join("home");
        let cwd = home.join("project/nested");
        fs::create_dir_all(&cwd).unwrap();
        let paths = test_paths(temporary.path());

        for root in [&home, Path::new("/")] {
            let resolved = resolve_from(&paths, &cwd, &home, Some(root), true).unwrap();

            assert_eq!(resolved.root, root.canonicalize().unwrap());
            assert_eq!(resolved.cwd, cwd.canonicalize().unwrap());
            assert_eq!(resolved.kind, "explicit");
            assert!(!resolved.scratch);
        }
    }

    #[test]
    fn broad_mount_override_preserves_marker_selected_home_root() {
        let temporary = tempfile::tempdir().unwrap();
        let home = temporary.path().join("home");
        let cwd = home.join("project/nested");
        fs::create_dir_all(&cwd).unwrap();
        create_marker(&home, ".jj");

        let resolved =
            resolve_from(&test_paths(temporary.path()), &cwd, &home, None, true).unwrap();

        assert_eq!(resolved.root, home.canonicalize().unwrap());
        assert_eq!(resolved.cwd, cwd.canonicalize().unwrap());
        assert_eq!(resolved.kind, "jj");
        assert!(!resolved.scratch);
    }

    #[test]
    fn scratch_creation_failure_is_fail_closed_with_path_context() {
        let temporary = tempfile::tempdir().unwrap();
        let home = temporary.path().join("home");
        fs::create_dir_all(&home).unwrap();
        let paths = test_paths(temporary.path());
        fs::create_dir_all(&paths.state).unwrap();
        fs::write(paths.state.join("scratch"), "not a directory").unwrap();

        let error = resolve_from(&paths, &home, &home, None, false).unwrap_err();
        let message = format!("{error:#}");

        assert!(message.contains("cannot create scratch workspace"));
        assert!(message.contains(&paths.scratch().display().to_string()));
    }

    #[test]
    fn selected_path_canonicalization_failure_is_fail_closed_with_context() {
        let temporary = tempfile::tempdir().unwrap();
        let cwd = temporary.path().join("workspace");
        let home = temporary.path().join("home");
        let missing = temporary.path().join("missing-project");
        fs::create_dir_all(&cwd).unwrap();
        fs::create_dir_all(&home).unwrap();

        let error = resolve_from(
            &test_paths(temporary.path()),
            &cwd,
            &home,
            Some(&missing),
            false,
        )
        .unwrap_err();
        let message = format!("{error:#}");

        assert!(message.contains("cannot resolve project path"));
        assert!(message.contains(&missing.display().to_string()));
    }

    #[test]
    fn identity_is_stable_for_a_root() {
        let root = Path::new("/tmp/example");
        assert_eq!(
            ProjectIdentity::from_local_root(root),
            ProjectIdentity::from_local_root(root)
        );
    }
}
