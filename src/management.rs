use std::collections::HashSet;
use std::fs::File;
use std::path::{Path, PathBuf};

use anyhow::{bail, Context, Result};
use microsandbox::sandbox::{SandboxConfig, SandboxHandle, SandboxStatus, VolumeMount};
use microsandbox::{MicrosandboxError, Sandbox};

use crate::harness::{self, Harness};
use crate::paths::AppPaths;
use crate::project::{self, Project};
use crate::runtime::{lock_capsule, CapsuleDescriptor};

pub struct StatusRequest {
    pub harness: Option<String>,
    pub project: Option<PathBuf>,
    pub allow_broad_mount: bool,
}

pub struct StopRequest {
    pub harness: String,
    pub project: Option<PathBuf>,
    pub allow_broad_mount: bool,
}

pub struct ResetRequest {
    pub harness: String,
    pub project: Option<PathBuf>,
    pub allow_broad_mount: bool,
}

const INVENTORY_PAGE_SIZE: u32 = 100;
const MAX_INVENTORY_CAPSULES: usize = 10_000;

struct InventorySnapshot {
    name: String,
    config: String,
    status: SandboxStatus,
}

struct InventoryPage {
    capsules: Vec<InventorySnapshot>,
    next_cursor: Option<String>,
}

#[derive(Debug, PartialEq, Eq)]
struct InventoryRecord {
    project: PathBuf,
    harness: String,
    status: SandboxStatus,
}

trait InventoryBackend {
    async fn page(&self, cursor: Option<&str>, label: (&str, &str)) -> Result<InventoryPage>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum StopOutcome {
    Absent,
    AlreadyStopped,
    Stopped,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ResetOutcome {
    Absent,
    Reset,
    StopFirst(SandboxStatus),
}

trait CapsuleControl {
    async fn inspect(&self, descriptor: &CapsuleDescriptor) -> Result<Option<SandboxStatus>>;

    async fn stop(&self, descriptor: &CapsuleDescriptor) -> Result<StopOutcome>;

    async fn reset(&self, descriptor: &CapsuleDescriptor) -> Result<ResetOutcome>;
}

trait ResetBackend {
    type Handle;
    type Lock;

    async fn lookup(&self, descriptor: &CapsuleDescriptor) -> Result<Option<Self::Handle>>;
    fn lock(&self, descriptor: &CapsuleDescriptor) -> Result<Self::Lock>;
    fn config_json<'a>(&self, handle: &'a Self::Handle) -> &'a str;
    fn status(&self, handle: &Self::Handle) -> SandboxStatus;
    async fn remove(&self, handle: &Self::Handle) -> Result<()>;
}

struct LocalCapsuleControl<'a> {
    paths: &'a AppPaths,
}

struct LocalInventoryBackend;

impl InventoryBackend for LocalInventoryBackend {
    async fn page(&self, cursor: Option<&str>, label: (&str, &str)) -> Result<InventoryPage> {
        let page = Sandbox::list_with(|builder| {
            let mut builder = builder.limit(INVENTORY_PAGE_SIZE).label(label.0, label.1);
            if let Some(cursor) = cursor {
                builder = builder.cursor(cursor);
            }
            builder
        })
        .await?;
        Ok(InventoryPage {
            capsules: page
                .sandboxes
                .into_iter()
                .map(|handle| InventorySnapshot {
                    name: handle.name().to_owned(),
                    config: handle.config_json().to_owned(),
                    status: handle.status_snapshot(),
                })
                .collect(),
            next_cursor: page.next_cursor,
        })
    }
}

impl CapsuleControl for LocalCapsuleControl<'_> {
    async fn inspect(&self, descriptor: &CapsuleDescriptor) -> Result<Option<SandboxStatus>> {
        let Some(handle) = get(descriptor).await? else {
            return Ok(None);
        };
        validate_handle(descriptor, &handle)?;
        Ok(Some(handle.status_snapshot()))
    }

    async fn stop(&self, descriptor: &CapsuleDescriptor) -> Result<StopOutcome> {
        if get(descriptor).await?.is_none() {
            return Ok(StopOutcome::Absent);
        }
        let _lock = lock_capsule(self.paths, descriptor)?;
        let Some(handle) = get(descriptor).await? else {
            return Ok(StopOutcome::Absent);
        };
        validate_handle(descriptor, &handle)?;
        if matches!(
            handle.status_snapshot(),
            SandboxStatus::Stopped | SandboxStatus::Crashed
        ) {
            return Ok(StopOutcome::AlreadyStopped);
        }
        handle.stop().await.context("cannot stop owned capsule")?;
        Ok(StopOutcome::Stopped)
    }

    async fn reset(&self, descriptor: &CapsuleDescriptor) -> Result<ResetOutcome> {
        reset_capsule(self, descriptor).await
    }
}

impl ResetBackend for LocalCapsuleControl<'_> {
    type Handle = SandboxHandle;
    type Lock = File;

    async fn lookup(&self, descriptor: &CapsuleDescriptor) -> Result<Option<Self::Handle>> {
        get(descriptor).await
    }

    fn lock(&self, descriptor: &CapsuleDescriptor) -> Result<Self::Lock> {
        lock_capsule(self.paths, descriptor)
    }

    fn config_json<'a>(&self, handle: &'a Self::Handle) -> &'a str {
        handle.config_json()
    }

    fn status(&self, handle: &Self::Handle) -> SandboxStatus {
        handle.status_snapshot()
    }

    async fn remove(&self, handle: &Self::Handle) -> Result<()> {
        handle.remove().await.context("cannot remove owned capsule")
    }
}

pub async fn status(request: StatusRequest) -> Result<Vec<String>> {
    let harnesses = select_harnesses(request.harness.as_deref())?;
    let (paths, project) = resolve_project(request.project, request.allow_broad_mount)?;
    status_with(&LocalCapsuleControl { paths: &paths }, &project, harnesses).await
}

pub async fn list() -> Result<Vec<String>> {
    inventory_with(&LocalInventoryBackend).await
}

pub async fn stop(request: StopRequest) -> Result<String> {
    let harness = select_harness(&request.harness)?;
    let (paths, project) = resolve_project(request.project, request.allow_broad_mount)?;
    stop_with(&LocalCapsuleControl { paths: &paths }, &project, harness).await
}

pub async fn reset(request: ResetRequest) -> Result<String> {
    let harness = select_harness(&request.harness)?;
    let (paths, project) = resolve_project(request.project, request.allow_broad_mount)?;
    reset_with(&LocalCapsuleControl { paths: &paths }, &project, harness).await
}

async fn status_with<C: CapsuleControl>(
    control: &C,
    project: &Project,
    harnesses: Vec<&dyn Harness>,
) -> Result<Vec<String>> {
    let mut lines = Vec::with_capacity(harnesses.len());
    for harness in harnesses {
        let descriptor = CapsuleDescriptor::new(project, harness);
        let observed = capsule_stage(control.inspect(&descriptor).await)?;
        lines.push(format!(
            "{}\t{}",
            harness.name(),
            observed.map_or("absent", status_name)
        ));
    }
    Ok(lines)
}

async fn inventory_with<B: InventoryBackend>(backend: &B) -> Result<Vec<String>> {
    let records = capsule_stage(load_inventory(backend).await)?;
    if records.is_empty() {
        return Ok(vec!["no capsules".into()]);
    }
    Ok(records
        .iter()
        .map(|record| {
            format!(
                "{}\t{}\t{}",
                display_path(&record.project),
                record.harness,
                status_name(record.status)
            )
        })
        .collect())
}

async fn load_inventory<B: InventoryBackend>(backend: &B) -> Result<Vec<InventoryRecord>> {
    let mut records = Vec::new();
    let mut cursor = None;
    let mut seen_cursors = HashSet::new();

    loop {
        let page = backend
            .page(cursor.as_deref(), ("fortlet.managed", "true"))
            .await?;
        if records.len() + page.capsules.len() > MAX_INVENTORY_CAPSULES {
            bail!("owned capsule inventory exceeds the supported bound");
        }
        for capsule in page.capsules {
            records.push(inventory_record(
                &capsule.name,
                &capsule.config,
                capsule.status,
            )?);
        }
        let Some(next) = page.next_cursor else {
            break;
        };
        if next.is_empty() || !seen_cursors.insert(next.clone()) {
            bail!("MicroSandbox returned an invalid capsule inventory cursor");
        }
        cursor = Some(next);
    }

    records.sort_by(|left, right| {
        left.project
            .as_os_str()
            .as_encoded_bytes()
            .cmp(right.project.as_os_str().as_encoded_bytes())
            .then_with(|| left.harness.cmp(&right.harness))
    });
    Ok(records)
}

fn inventory_record(
    handle_name: &str,
    stored: &str,
    status: SandboxStatus,
) -> Result<InventoryRecord> {
    let config: SandboxConfig =
        serde_json::from_str(stored).context("cannot read stored capsule configuration")?;
    if handle_name != config.spec.name {
        bail!("stored capsule handle and configuration names do not match");
    }
    let project = project_mount(&config)?;
    let harness =
        CapsuleDescriptor::validate_inventory(&config.spec.name, &config.spec.labels, &project)?;
    Ok(InventoryRecord {
        project,
        harness,
        status,
    })
}

fn project_mount(config: &SandboxConfig) -> Result<PathBuf> {
    let mut matches = config.spec.mounts.iter().filter_map(|mount| match mount {
        VolumeMount::Bind {
            host,
            guest,
            options,
            ..
        } if !options.readonly && host.is_absolute() && host == Path::new(guest) => {
            Some(host.clone())
        }
        _ => None,
    });
    let project = matches
        .next()
        .context("stored capsule has no unambiguous project mount")?;
    if matches.next().is_some() {
        bail!("stored capsule has multiple possible project mounts");
    }
    Ok(project)
}

fn display_path(path: &Path) -> String {
    let bytes = path.as_os_str().as_encoded_bytes();
    match std::str::from_utf8(bytes) {
        Ok(text) => {
            let mut escaped = String::new();
            for character in text.chars() {
                push_escaped_character(&mut escaped, character);
            }
            escaped
        }
        Err(_) => {
            let mut escaped = String::new();
            for byte in bytes {
                push_escaped_byte(&mut escaped, *byte);
            }
            escaped
        }
    }
}

fn push_escaped_character(escaped: &mut String, character: char) {
    match character {
        '\\' => escaped.push_str("\\\\"),
        '\t' => escaped.push_str("\\t"),
        '\r' => escaped.push_str("\\r"),
        '\n' => escaped.push_str("\\n"),
        character if character.is_control() => {
            use std::fmt::Write;
            let _ = write!(escaped, "\\u{{{:x}}}", u32::from(character));
        }
        character => escaped.push(character),
    }
}

fn push_escaped_byte(escaped: &mut String, byte: u8) {
    if byte.is_ascii_graphic() || byte == b' ' {
        if byte == b'\\' {
            escaped.push_str("\\\\");
        } else {
            escaped.push(char::from(byte));
        }
    } else {
        use std::fmt::Write;
        let _ = write!(escaped, "\\x{byte:02x}");
    }
}

async fn stop_with<C: CapsuleControl>(
    control: &C,
    project: &Project,
    harness: &dyn Harness,
) -> Result<String> {
    let descriptor = CapsuleDescriptor::new(project, harness);
    let outcome = capsule_stage(control.stop(&descriptor).await)?;
    let name = match outcome {
        StopOutcome::Absent => "absent",
        StopOutcome::AlreadyStopped => "already-stopped",
        StopOutcome::Stopped => "stopped",
    };
    Ok(format!("{}\t{name}", harness.name()))
}

async fn reset_with<C: CapsuleControl>(
    control: &C,
    project: &Project,
    harness: &dyn Harness,
) -> Result<String> {
    let descriptor = CapsuleDescriptor::new(project, harness);
    let outcome = capsule_stage(control.reset(&descriptor).await)?;
    let name = match outcome {
        ResetOutcome::Absent => "absent",
        ResetOutcome::Reset => "reset",
        ResetOutcome::StopFirst(status) => {
            anyhow::bail!(
                "capsule stage failed; run `fortlet stop {}` and retry reset: capsule is {}",
                harness.name(),
                status_name(status)
            );
        }
    };
    Ok(format!("{}\t{name}", harness.name()))
}

async fn reset_capsule<B: ResetBackend>(
    backend: &B,
    descriptor: &CapsuleDescriptor,
) -> Result<ResetOutcome> {
    if backend.lookup(descriptor).await?.is_none() {
        return Ok(ResetOutcome::Absent);
    }

    let _lock = backend.lock(descriptor)?;
    let Some(handle) = backend.lookup(descriptor).await? else {
        return Ok(ResetOutcome::Absent);
    };
    validate_stored_config(descriptor, backend.config_json(&handle))?;
    let status = backend.status(&handle);
    if !matches!(status, SandboxStatus::Stopped | SandboxStatus::Crashed) {
        return Ok(ResetOutcome::StopFirst(status));
    }
    backend.remove(&handle).await?;
    Ok(ResetOutcome::Reset)
}

fn select_harnesses(name: Option<&str>) -> Result<Vec<&'static dyn Harness>> {
    match name {
        Some(name) => select_harness(name).map(|harness| vec![harness]),
        None => harness::names()
            .map(harness::find)
            .collect::<Result<Vec<_>>>(),
    }
}

fn select_harness(name: &str) -> Result<&'static dyn Harness> {
    stage(
        harness::find(name),
        "harness",
        "choose a registered harness: codex or tact",
    )
}

fn resolve_project(
    project: Option<PathBuf>,
    allow_broad_mount: bool,
) -> Result<(AppPaths, Project)> {
    let paths = stage(
        AppPaths::from_environment(),
        "project",
        "set HOME to a readable user directory and retry",
    )?;
    let project = stage(
        project::resolve(&paths, project.as_deref(), allow_broad_mount),
        "project",
        "run from a readable project directory or pass --project <path>",
    )?;
    if project.scratch {
        eprintln!(
            "fortlet: using persistent scratch workspace {}; pass --allow-broad-mount to expose this directory",
            project.root.display()
        );
    }
    Ok((paths, project))
}

async fn get(descriptor: &CapsuleDescriptor) -> Result<Option<SandboxHandle>> {
    match Sandbox::get(&descriptor.name).await {
        Ok(handle) => Ok(Some(handle)),
        Err(MicrosandboxError::SandboxNotFound(_)) => Ok(None),
        Err(error) => Err(error.into()),
    }
}

fn validate_handle(descriptor: &CapsuleDescriptor, handle: &SandboxHandle) -> Result<()> {
    validate_stored_config(descriptor, handle.config_json())
}

fn validate_stored_config(descriptor: &CapsuleDescriptor, stored: &str) -> Result<()> {
    let config: SandboxConfig =
        serde_json::from_str(stored).context("cannot read stored capsule configuration")?;
    descriptor.validate_management(&config.spec.name, &config.spec.labels)
}

fn status_name(status: SandboxStatus) -> &'static str {
    match status {
        SandboxStatus::Created => "created",
        SandboxStatus::Starting => "starting",
        SandboxStatus::Running => "running",
        SandboxStatus::Draining => "draining",
        SandboxStatus::Paused => "paused",
        SandboxStatus::Stopped => "stopped",
        SandboxStatus::Crashed => "crashed",
    }
}

fn capsule_stage<T>(result: Result<T>) -> Result<T> {
    stage(
        result,
        "capsule",
        "run `fortlet doctor` and follow its reported correction",
    )
}

fn stage<T>(result: Result<T>, name: &str, action: &str) -> Result<T> {
    result.with_context(|| format!("{name} stage failed; {action}"))
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;
    use std::collections::VecDeque;
    use std::ffi::OsString;
    use std::os::unix::ffi::OsStringExt;
    use std::sync::Mutex;

    use anyhow::anyhow;
    use microsandbox::sandbox::{HostPermissions, MountOptions, StatVirtualization};

    use super::*;
    use crate::project::ProjectIdentity;

    struct FakeControl {
        status: Result<Option<SandboxStatus>, &'static str>,
        stop: Result<StopOutcome, &'static str>,
        reset: Result<ResetOutcome, &'static str>,
    }

    struct FakeInventoryBackend {
        pages: Mutex<VecDeque<Result<InventoryPage, &'static str>>>,
        requests: Mutex<Vec<(Option<String>, String, String)>>,
    }

    impl InventoryBackend for FakeInventoryBackend {
        async fn page(&self, cursor: Option<&str>, label: (&str, &str)) -> Result<InventoryPage> {
            self.requests.lock().unwrap().push((
                cursor.map(str::to_owned),
                label.0.to_owned(),
                label.1.to_owned(),
            ));
            self.pages
                .lock()
                .unwrap()
                .pop_front()
                .expect("unexpected inventory page")
                .map_err(|message| anyhow!(message))
        }
    }

    impl CapsuleControl for FakeControl {
        async fn inspect(&self, _descriptor: &CapsuleDescriptor) -> Result<Option<SandboxStatus>> {
            self.status.map_err(|message| anyhow!(message))
        }

        async fn stop(&self, _descriptor: &CapsuleDescriptor) -> Result<StopOutcome> {
            self.stop.map_err(|message| anyhow!(message))
        }

        async fn reset(&self, _descriptor: &CapsuleDescriptor) -> Result<ResetOutcome> {
            self.reset.map_err(|message| anyhow!(message))
        }
    }

    fn project() -> Project {
        Project {
            identity: ProjectIdentity::from_local_root(PathBuf::from("/tmp/management").as_path()),
            root: "/tmp/management".into(),
            cwd: "/tmp/management".into(),
            kind: "test",
            scratch: false,
        }
    }

    fn fake(status: Option<SandboxStatus>, stop: StopOutcome) -> FakeControl {
        FakeControl {
            status: Ok(status),
            stop: Ok(stop),
            reset: Ok(ResetOutcome::Absent),
        }
    }

    fn inventory_handle(root: &Path, harness: &str, status: SandboxStatus) -> InventorySnapshot {
        let identity = ProjectIdentity::from_local_root(root);
        let name = format!(
            "fortlet-{}-{harness}-{}",
            unsafe { libc::geteuid() },
            identity.as_str()
        );
        let mut config = SandboxConfig::default();
        config.spec.name.clone_from(&name);
        config.spec.labels = BTreeMap::from([
            ("fortlet.managed".into(), "true".into()),
            ("fortlet.schema".into(), "1".into()),
            ("fortlet.project".into(), identity.as_str().into()),
            ("fortlet.tool".into(), harness.into()),
            ("fortlet.version".into(), "older-release".into()),
        ]);
        config.spec.mounts = vec![bind_mount(root, root, false)];
        InventorySnapshot {
            name,
            config: serde_json::to_string(&config).unwrap(),
            status,
        }
    }

    fn bind_mount(host: &Path, guest: &Path, readonly: bool) -> VolumeMount {
        VolumeMount::Bind {
            host: host.into(),
            guest: guest.display().to_string(),
            options: MountOptions {
                readonly,
                ..MountOptions::default()
            },
            stat_virtualization: StatVirtualization::Strict,
            host_permissions: HostPermissions::Mirror,
            follow_root_symlinks: false,
            quota_mib: None,
        }
    }

    fn inventory_backend(
        pages: impl IntoIterator<Item = Result<InventoryPage, &'static str>>,
    ) -> FakeInventoryBackend {
        FakeInventoryBackend {
            pages: Mutex::new(pages.into_iter().collect()),
            requests: Mutex::new(Vec::new()),
        }
    }

    #[tokio::test]
    async fn inventory_reads_every_managed_page_and_sorts_stably() {
        let backend = inventory_backend([
            Ok(InventoryPage {
                capsules: vec![inventory_handle(
                    Path::new("/tmp/z-project"),
                    "tact",
                    SandboxStatus::Stopped,
                )],
                next_cursor: Some("next-page".into()),
            }),
            Ok(InventoryPage {
                capsules: vec![
                    inventory_handle(Path::new("/tmp/a-project"), "tact", SandboxStatus::Running),
                    inventory_handle(Path::new("/tmp/a-project"), "codex", SandboxStatus::Crashed),
                ],
                next_cursor: None,
            }),
        ]);

        assert_eq!(
            inventory_with(&backend).await.unwrap(),
            [
                "/tmp/a-project\tcodex\tcrashed",
                "/tmp/a-project\ttact\trunning",
                "/tmp/z-project\ttact\tstopped",
            ]
        );
        assert_eq!(
            *backend.requests.lock().unwrap(),
            [
                (None, "fortlet.managed".into(), "true".into()),
                (
                    Some("next-page".into()),
                    "fortlet.managed".into(),
                    "true".into(),
                ),
            ]
        );
    }

    #[tokio::test]
    async fn inventory_renders_empty_and_every_lifecycle_state() {
        let empty = inventory_backend([Ok(InventoryPage {
            capsules: vec![],
            next_cursor: None,
        })]);
        assert_eq!(inventory_with(&empty).await.unwrap(), ["no capsules"]);

        let states = [
            (SandboxStatus::Created, "created"),
            (SandboxStatus::Starting, "starting"),
            (SandboxStatus::Running, "running"),
            (SandboxStatus::Draining, "draining"),
            (SandboxStatus::Paused, "paused"),
            (SandboxStatus::Stopped, "stopped"),
            (SandboxStatus::Crashed, "crashed"),
        ];
        for (index, (status, expected)) in states.into_iter().enumerate() {
            let root = PathBuf::from(format!("/tmp/state-{index}"));
            let backend = inventory_backend([Ok(InventoryPage {
                capsules: vec![inventory_handle(&root, "codex", status)],
                next_cursor: None,
            })]);
            assert_eq!(
                inventory_with(&backend).await.unwrap(),
                [format!("{}\tcodex\t{expected}", root.display())]
            );
        }
    }

    #[tokio::test]
    async fn inventory_rejects_page_failure_and_repeated_cursor_without_partial_results() {
        let page_failure = inventory_backend([
            Ok(InventoryPage {
                capsules: vec![inventory_handle(
                    Path::new("/tmp/valid"),
                    "codex",
                    SandboxStatus::Running,
                )],
                next_cursor: Some("next".into()),
            }),
            Err("list failed"),
        ]);
        assert!(inventory_with(&page_failure).await.is_err());

        let repeated = inventory_backend([
            Ok(InventoryPage {
                capsules: vec![],
                next_cursor: Some("same".into()),
            }),
            Ok(InventoryPage {
                capsules: vec![],
                next_cursor: Some("same".into()),
            }),
        ]);
        let error = inventory_with(&repeated).await.unwrap_err();
        assert!(format!("{error:#}").contains("invalid capsule inventory cursor"));
    }

    #[test]
    fn inventory_validates_complete_ownership_and_project_mount() {
        let valid = inventory_handle(
            Path::new("/tmp/inventory"),
            "future-harness",
            SandboxStatus::Stopped,
        );
        assert_eq!(
            inventory_record(&valid.name, &valid.config, valid.status).unwrap(),
            InventoryRecord {
                project: "/tmp/inventory".into(),
                harness: "future-harness".into(),
                status: SandboxStatus::Stopped,
            }
        );

        let mut invalid = Vec::new();
        invalid.push(("different-name".into(), valid.config.clone()));
        invalid.push((valid.name.clone(), "not json".into()));

        for mutation in [
            |value: &mut serde_json::Value| value["labels"]["fortlet.managed"] = "false".into(),
            |value: &mut serde_json::Value| value["labels"]["fortlet.schema"] = "2".into(),
            |value: &mut serde_json::Value| value["labels"]["fortlet.project"] = "wrong".into(),
            |value: &mut serde_json::Value| value["labels"]["fortlet.tool"] = "BAD\n".into(),
        ] {
            let mut value: serde_json::Value = serde_json::from_str(&valid.config).unwrap();
            mutation(&mut value);
            invalid.push((valid.name.clone(), value.to_string()));
        }

        let mut missing_mount: serde_json::Value = serde_json::from_str(&valid.config).unwrap();
        missing_mount["mounts"] = serde_json::json!([]);
        invalid.push((valid.name.clone(), missing_mount.to_string()));

        let mut duplicate_mount: serde_json::Value = serde_json::from_str(&valid.config).unwrap();
        let mount = duplicate_mount["mounts"][0].clone();
        duplicate_mount["mounts"]
            .as_array_mut()
            .unwrap()
            .push(mount);
        invalid.push((valid.name.clone(), duplicate_mount.to_string()));

        for (name, config) in invalid {
            assert!(
                inventory_record(&name, &config, SandboxStatus::Stopped).is_err(),
                "accepted invalid inventory config: {config}"
            );
        }
    }

    #[test]
    fn inventory_path_rendering_is_single_line_and_preserves_printable_utf8() {
        assert_eq!(
            display_path(Path::new("/tmp/naïve\\line\n\t\r\u{1b}")),
            "/tmp/naïve\\\\line\\n\\t\\r\\u{1b}"
        );
        let invalid = PathBuf::from(OsString::from_vec(vec![
            b'/', b't', b'm', b'p', b'/', 0xff, b'\n',
        ]));
        assert_eq!(display_path(&invalid), "/tmp/\\xff\\x0a");
    }

    #[tokio::test]
    async fn status_maps_absence_and_every_sdk_state() {
        let harness = harness::find("codex").unwrap();
        let cases = [
            (None, "absent"),
            (Some(SandboxStatus::Created), "created"),
            (Some(SandboxStatus::Starting), "starting"),
            (Some(SandboxStatus::Running), "running"),
            (Some(SandboxStatus::Draining), "draining"),
            (Some(SandboxStatus::Paused), "paused"),
            (Some(SandboxStatus::Stopped), "stopped"),
            (Some(SandboxStatus::Crashed), "crashed"),
        ];

        for (status, expected) in cases {
            let control = fake(status, StopOutcome::Stopped);
            let lines = status_with(&control, &project(), vec![harness])
                .await
                .unwrap();
            assert_eq!(lines, [format!("codex\t{expected}")]);
        }
    }

    #[tokio::test]
    async fn status_without_selector_uses_registry_order() {
        let control = fake(None, StopOutcome::Stopped);
        let harnesses = select_harnesses(None).unwrap();

        let lines = status_with(&control, &project(), harnesses).await.unwrap();

        assert_eq!(lines, ["codex\tabsent", "tact\tabsent"]);
    }

    #[tokio::test]
    async fn stop_renders_each_idempotent_outcome() {
        for (outcome, expected) in [
            (StopOutcome::Absent, "codex\tabsent"),
            (StopOutcome::AlreadyStopped, "codex\talready-stopped"),
            (StopOutcome::Stopped, "codex\tstopped"),
        ] {
            let control = fake(None, outcome);
            let line = stop_with(&control, &project(), harness::find("codex").unwrap())
                .await
                .unwrap();
            assert_eq!(line, expected);
        }
    }

    #[tokio::test]
    async fn reset_renders_idempotent_outcomes_and_active_correction() {
        let harness = harness::find("codex").unwrap();
        for (outcome, expected) in [
            (ResetOutcome::Absent, "codex\tabsent"),
            (ResetOutcome::Reset, "codex\treset"),
        ] {
            let mut control = fake(None, StopOutcome::Stopped);
            control.reset = Ok(outcome);
            assert_eq!(
                reset_with(&control, &project(), harness).await.unwrap(),
                expected
            );
        }

        for status in [
            SandboxStatus::Created,
            SandboxStatus::Starting,
            SandboxStatus::Running,
            SandboxStatus::Draining,
            SandboxStatus::Paused,
        ] {
            let mut control = fake(None, StopOutcome::Stopped);
            control.reset = Ok(ResetOutcome::StopFirst(status));
            let error = reset_with(&control, &project(), harness).await.unwrap_err();
            assert_eq!(
                error.to_string(),
                format!(
                    "capsule stage failed; run `fortlet stop codex` and retry reset: capsule is {}",
                    status_name(status)
                )
            );
        }
    }

    #[tokio::test]
    async fn runtime_failures_keep_stage_action_and_cause() {
        let control = FakeControl {
            status: Err("lookup failed"),
            stop: Err("stop failed"),
            reset: Err("reset failed"),
        };

        for error in [
            status_with(&control, &project(), vec![harness::find("codex").unwrap()])
                .await
                .unwrap_err(),
            stop_with(&control, &project(), harness::find("codex").unwrap())
                .await
                .unwrap_err(),
            reset_with(&control, &project(), harness::find("codex").unwrap())
                .await
                .unwrap_err(),
        ] {
            let message = format!("{error:#}");
            assert!(message.starts_with(
                "capsule stage failed; run `fortlet doctor` and follow its reported correction"
            ));
            assert!(message.ends_with("failed"));
        }
    }

    #[test]
    fn malformed_stored_configuration_fails_before_ownership_use() {
        let descriptor = CapsuleDescriptor::new(&project(), harness::find("codex").unwrap());

        let error = validate_stored_config(&descriptor, "not json").unwrap_err();

        assert!(
            format!("{error:#}").starts_with("cannot read stored capsule configuration"),
            "{error:#}"
        );
    }

    #[derive(Clone)]
    struct FakeHandle {
        config: String,
        status: SandboxStatus,
    }

    struct FakeResetBackend {
        lookups: Mutex<VecDeque<Result<Option<FakeHandle>, &'static str>>>,
        lock_error: Option<&'static str>,
        remove_error: Option<&'static str>,
        events: Mutex<Vec<&'static str>>,
    }

    impl ResetBackend for FakeResetBackend {
        type Handle = FakeHandle;
        type Lock = ();

        async fn lookup(&self, _descriptor: &CapsuleDescriptor) -> Result<Option<Self::Handle>> {
            self.events.lock().unwrap().push("lookup");
            self.lookups
                .lock()
                .unwrap()
                .pop_front()
                .expect("unexpected lookup")
                .map_err(|message| anyhow!(message))
        }

        fn lock(&self, _descriptor: &CapsuleDescriptor) -> Result<Self::Lock> {
            self.events.lock().unwrap().push("lock");
            self.lock_error
                .map_or(Ok(()), |message| Err(anyhow!(message)))
        }

        fn config_json<'a>(&self, handle: &'a Self::Handle) -> &'a str {
            self.events.lock().unwrap().push("config");
            &handle.config
        }

        fn status(&self, handle: &Self::Handle) -> SandboxStatus {
            self.events.lock().unwrap().push("status");
            handle.status
        }

        async fn remove(&self, _handle: &Self::Handle) -> Result<()> {
            self.events.lock().unwrap().push("remove");
            self.remove_error
                .map_or(Ok(()), |message| Err(anyhow!(message)))
        }
    }

    fn stored_config(descriptor: &CapsuleDescriptor) -> String {
        serde_json::json!({
            "name": descriptor.name,
            "labels": {
                "fortlet.managed": "true",
                "fortlet.schema": "1",
                "fortlet.project": project().identity.as_str(),
                "fortlet.tool": "codex",
                "fortlet.version": "older-release"
            }
        })
        .to_string()
    }

    fn reset_backend(handle: FakeHandle) -> FakeResetBackend {
        FakeResetBackend {
            lookups: Mutex::new(VecDeque::from([Ok(Some(handle.clone())), Ok(Some(handle))])),
            lock_error: None,
            remove_error: None,
            events: Mutex::new(Vec::new()),
        }
    }

    fn reset_handle(status: SandboxStatus) -> (CapsuleDescriptor, FakeHandle) {
        let descriptor = CapsuleDescriptor::new(&project(), harness::find("codex").unwrap());
        let handle = FakeHandle {
            config: stored_config(&descriptor),
            status,
        };
        (descriptor, handle)
    }

    #[tokio::test]
    async fn absent_reset_returns_before_lock_or_state_work() {
        let descriptor = CapsuleDescriptor::new(&project(), harness::find("codex").unwrap());
        let backend = FakeResetBackend {
            lookups: Mutex::new(VecDeque::from([Ok(None)])),
            lock_error: None,
            remove_error: None,
            events: Mutex::new(Vec::new()),
        };

        assert_eq!(
            reset_capsule(&backend, &descriptor).await.unwrap(),
            ResetOutcome::Absent
        );
        assert_eq!(*backend.events.lock().unwrap(), ["lookup"]);
    }

    #[tokio::test]
    async fn reset_refetches_under_lock_and_removes_only_terminal_states() {
        for status in [
            SandboxStatus::Created,
            SandboxStatus::Starting,
            SandboxStatus::Running,
            SandboxStatus::Draining,
            SandboxStatus::Paused,
            SandboxStatus::Stopped,
            SandboxStatus::Crashed,
        ] {
            let (descriptor, handle) = reset_handle(status);
            let backend = reset_backend(handle);

            let outcome = reset_capsule(&backend, &descriptor).await.unwrap();
            let terminal = matches!(status, SandboxStatus::Stopped | SandboxStatus::Crashed);
            assert_eq!(
                outcome,
                if terminal {
                    ResetOutcome::Reset
                } else {
                    ResetOutcome::StopFirst(status)
                }
            );
            assert_eq!(
                *backend.events.lock().unwrap(),
                if terminal {
                    vec!["lookup", "lock", "lookup", "config", "status", "remove"]
                } else {
                    vec!["lookup", "lock", "lookup", "config", "status"]
                }
            );
        }
    }

    #[tokio::test]
    async fn reset_accepts_version_skew_but_rejects_bad_ownership_and_config() {
        let (descriptor, handle) = reset_handle(SandboxStatus::Stopped);
        let backend = reset_backend(handle.clone());
        assert_eq!(
            reset_capsule(&backend, &descriptor).await.unwrap(),
            ResetOutcome::Reset
        );

        let mut wrong_name: serde_json::Value = serde_json::from_str(&handle.config).unwrap();
        wrong_name["name"] = "fortlet-collision".into();
        let mut wrong_tool: serde_json::Value = serde_json::from_str(&handle.config).unwrap();
        wrong_tool["labels"]["fortlet.tool"] = "tact".into();
        for config in [
            wrong_name.to_string(),
            wrong_tool.to_string(),
            "not json".into(),
        ] {
            let backend = reset_backend(FakeHandle {
                config,
                status: SandboxStatus::Stopped,
            });
            assert!(reset_capsule(&backend, &descriptor).await.is_err());
            assert!(!backend.events.lock().unwrap().contains(&"remove"));
        }
    }

    #[tokio::test]
    async fn reset_propagates_lookup_lock_and_removal_failures() {
        let (descriptor, handle) = reset_handle(SandboxStatus::Stopped);
        let cases = [
            (
                FakeResetBackend {
                    lookups: Mutex::new(VecDeque::from([Err("initial lookup failed")])),
                    lock_error: None,
                    remove_error: None,
                    events: Mutex::new(Vec::new()),
                },
                "initial lookup failed",
            ),
            (
                FakeResetBackend {
                    lookups: Mutex::new(VecDeque::from([Ok(Some(handle.clone()))])),
                    lock_error: Some("lock failed"),
                    remove_error: None,
                    events: Mutex::new(Vec::new()),
                },
                "lock failed",
            ),
            (
                FakeResetBackend {
                    lookups: Mutex::new(VecDeque::from([
                        Ok(Some(handle.clone())),
                        Err("refetch failed"),
                    ])),
                    lock_error: None,
                    remove_error: None,
                    events: Mutex::new(Vec::new()),
                },
                "refetch failed",
            ),
            (
                FakeResetBackend {
                    lookups: Mutex::new(VecDeque::from([
                        Ok(Some(handle.clone())),
                        Ok(Some(handle.clone())),
                    ])),
                    lock_error: None,
                    remove_error: Some("remove failed"),
                    events: Mutex::new(Vec::new()),
                },
                "remove failed",
            ),
        ];

        for (backend, expected) in cases {
            assert_eq!(
                reset_capsule(&backend, &descriptor)
                    .await
                    .unwrap_err()
                    .to_string(),
                expected
            );
        }
    }
}
