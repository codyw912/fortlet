use std::env;
use std::ffi::{OsStr, OsString};
use std::fs::File;
use std::io::{self, Read, Write};
use std::os::fd::{AsRawFd, FromRawFd};
use std::os::unix::process::{CommandExt, ExitStatusExt};
use std::path::PathBuf;
use std::process::{Child, Command, ExitStatus, Stdio};
use std::thread;
use std::time::{Duration, Instant};

use anyhow::{bail, Context, Result};

struct CommandSpec {
    program: PathBuf,
    args: Vec<String>,
    cwd: Option<PathBuf>,
}

struct ObserverConfig {
    startup_timeout: Duration,
    resize: Option<TerminalSize>,
    resize_timeout: Duration,
    hold: Duration,
    exit_timeout: Duration,
    termination: TerminationAction,
}

#[derive(Debug)]
struct Observation {
    initial_bytes: usize,
    resized_bytes: usize,
    exit_code: Option<i32>,
    exit_signal: Option<i32>,
}

#[derive(Clone, Copy)]
struct TerminalSize {
    rows: u16,
    cols: u16,
}

#[derive(Clone, Copy)]
enum TerminationAction {
    SignalInterrupt,
    ExitCommand,
    TypedExitCommand,
    CtrlCKey,
}

enum Mode {
    Fixture,
    FixtureExit,
    FixtureTypedExit,
    FixtureCtrlC,
    Observe {
        program: PathBuf,
        cwd: PathBuf,
        hold: Duration,
    },
    ObserveExit {
        program: PathBuf,
        cwd: PathBuf,
        hold: Duration,
    },
    ObserveTypedExit {
        program: PathBuf,
        cwd: PathBuf,
        hold: Duration,
    },
    ObserveCtrlC {
        program: PathBuf,
        cwd: PathBuf,
        hold: Duration,
    },
}

fn parse_args(args: impl IntoIterator<Item = OsString>) -> Result<Mode> {
    let args: Vec<OsString> = args.into_iter().collect();
    match args.as_slice() {
        [mode] if mode == "fixture" => Ok(Mode::Fixture),
        [mode] if mode == "fixture-exit" => Ok(Mode::FixtureExit),
        [mode] if mode == "fixture-typed-exit" => Ok(Mode::FixtureTypedExit),
        [mode] if mode == "fixture-ctrl-c" => Ok(Mode::FixtureCtrlC),
        [mode, program, cwd, hold] if mode == "observe" => Ok(Mode::Observe {
            program: PathBuf::from(program),
            cwd: PathBuf::from(cwd),
            hold: parse_hold(hold)?,
        }),
        [mode, program, cwd, hold] if mode == "observe-exit" => Ok(Mode::ObserveExit {
            program: PathBuf::from(program),
            cwd: PathBuf::from(cwd),
            hold: parse_hold(hold)?,
        }),
        [mode, program, cwd, hold] if mode == "observe-typed-exit" => Ok(Mode::ObserveTypedExit {
            program: PathBuf::from(program),
            cwd: PathBuf::from(cwd),
            hold: parse_hold(hold)?,
        }),
        [mode, program, cwd, hold] if mode == "observe-ctrl-c" => Ok(Mode::ObserveCtrlC {
            program: PathBuf::from(program),
            cwd: PathBuf::from(cwd),
            hold: parse_hold(hold)?,
        }),
        _ => bail!(
            "usage: pty_observer fixture | fixture-exit | fixture-typed-exit | \
             fixture-ctrl-c | observe <program> <cwd> <hold-seconds> | observe-exit \
             <program> <cwd> <hold-seconds> | observe-typed-exit <program> <cwd> \
             <hold-seconds> | observe-ctrl-c <program> <cwd> <hold-seconds>"
        ),
    }
}

fn parse_hold(value: &OsStr) -> Result<Duration> {
    let seconds = value
        .to_str()
        .context("hold seconds must be valid UTF-8")?
        .parse::<u64>()
        .context("hold seconds must be an integer")?;
    Ok(Duration::from_secs(seconds))
}

#[cfg(test)]
fn observe(command: &CommandSpec, config: &ObserverConfig) -> Result<Observation> {
    observe_with_events(command, config, |_| {})
}

fn observe_with_events(
    command: &CommandSpec,
    config: &ObserverConfig,
    mut emit: impl FnMut(&'static str),
) -> Result<Observation> {
    let (mut master, slave) = open_pty(24, 80)?;
    set_nonblocking(&master)?;
    let mut child = spawn_child(command, slave)?;
    emit("started");

    let mut initial_bytes = read_until_activity(&mut master, config.startup_timeout)?;
    emit("activity_initial");
    let resized_bytes = if let Some(size) = config.resize {
        initial_bytes += drain_until_quiet(
            &mut master,
            Duration::from_millis(100),
            config.startup_timeout,
        )?;
        resize_pty(&master, size)?;
        emit("resized");
        let bytes = read_until_activity(&mut master, config.resize_timeout)?;
        emit("activity_resized");
        bytes
    } else {
        0
    };
    emit("concurrent_window");
    thread::sleep(config.hold);
    match config.termination {
        TerminationAction::SignalInterrupt => {
            child.signal(libc::SIGINT)?;
            emit("signal");
        }
        TerminationAction::ExitCommand => {
            write_exit_command(&mut master)?;
            emit("exit_command");
        }
        TerminationAction::TypedExitCommand => {
            write_typed_exit_command(&mut master)?;
            emit("typed_exit_command");
        }
        TerminationAction::CtrlCKey => {
            write_ctrl_c_key(&mut master)?;
            emit("ctrl_c_key");
        }
    }
    let status = child.wait(config.exit_timeout)?;
    emit("exited");

    Ok(Observation {
        initial_bytes,
        resized_bytes,
        exit_code: status.code(),
        exit_signal: status.signal(),
    })
}

fn write_exit_command(output: &mut impl Write) -> Result<()> {
    output
        .write_all(b"/exit\r")
        .context("cannot write exit command to PTY")
}

fn write_ctrl_c_key(output: &mut impl Write) -> Result<()> {
    output
        .write_all(&[0x03])
        .context("cannot write Ctrl-C key to PTY")
}

const TYPED_EXIT_KEY_DELAY: Duration = Duration::from_millis(20);

fn write_typed_exit_command(output: &mut impl Write) -> Result<()> {
    write_typed_exit_command_with_delay(output, thread::sleep)
}

fn write_typed_exit_command_with_delay(
    output: &mut impl Write,
    mut delay: impl FnMut(Duration),
) -> Result<()> {
    for byte in b"/exit" {
        output
            .write_all(std::slice::from_ref(byte))
            .context("cannot write typed exit key to PTY")?;
        delay(TYPED_EXIT_KEY_DELAY);
    }
    output
        .write_all(b"\r")
        .context("cannot write typed exit Enter key to PTY")
}

fn open_pty(rows: u16, cols: u16) -> Result<(File, File)> {
    let mut master = -1;
    let mut slave = -1;
    let mut size = libc::winsize {
        ws_row: rows,
        ws_col: cols,
        ws_xpixel: 0,
        ws_ypixel: 0,
    };
    let result = unsafe {
        libc::openpty(
            &mut master,
            &mut slave,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            &mut size,
        )
    };
    if result == -1 {
        return Err(io::Error::last_os_error()).context("cannot open PTY");
    }
    let master = unsafe { File::from_raw_fd(master) };
    let slave = unsafe { File::from_raw_fd(slave) };
    Ok((master, slave))
}

fn set_nonblocking(file: &File) -> Result<()> {
    let descriptor = file.as_raw_fd();
    let flags = unsafe { libc::fcntl(descriptor, libc::F_GETFL) };
    if flags == -1 {
        return Err(io::Error::last_os_error()).context("cannot read PTY flags");
    }
    if unsafe { libc::fcntl(descriptor, libc::F_SETFL, flags | libc::O_NONBLOCK) } == -1 {
        return Err(io::Error::last_os_error()).context("cannot make PTY nonblocking");
    }
    Ok(())
}

fn resize_pty(file: &File, size: TerminalSize) -> Result<()> {
    let dimensions = libc::winsize {
        ws_row: size.rows,
        ws_col: size.cols,
        ws_xpixel: 0,
        ws_ypixel: 0,
    };
    if unsafe { libc::ioctl(file.as_raw_fd(), libc::TIOCSWINSZ as _, &dimensions) } == -1 {
        return Err(io::Error::last_os_error()).context("cannot resize PTY");
    }
    Ok(())
}

fn spawn_child(command: &CommandSpec, slave: File) -> Result<OwnedChild> {
    let stdin = slave.try_clone().context("cannot clone PTY slave")?;
    let stdout = slave.try_clone().context("cannot clone PTY slave")?;
    let controlling_terminal = slave.as_raw_fd();
    let mut process = Command::new(&command.program);
    process
        .args(&command.args)
        .stdin(Stdio::from(stdin))
        .stdout(Stdio::from(stdout))
        .stderr(Stdio::from(slave));
    if let Some(cwd) = &command.cwd {
        process.current_dir(cwd);
    }
    unsafe {
        process.pre_exec(move || {
            if libc::setsid() == -1 {
                return Err(io::Error::last_os_error());
            }
            if libc::ioctl(controlling_terminal, libc::TIOCSCTTY as _, 0) == -1 {
                return Err(io::Error::last_os_error());
            }
            Ok(())
        });
    }
    let child = process.spawn().with_context(|| {
        format!(
            "cannot launch PTY child {}",
            command.program.to_string_lossy()
        )
    })?;
    OwnedChild::new(child)
}

fn read_until_activity(master: &mut File, timeout: Duration) -> Result<usize> {
    let deadline = Instant::now() + timeout;
    let mut buffer = [0_u8; 4096];
    loop {
        match master.read(&mut buffer) {
            Ok(0) => bail!("PTY closed before producing output"),
            Ok(count) => return Ok(count),
            Err(error) if error.kind() == io::ErrorKind::WouldBlock => {}
            Err(error) => return Err(error).context("cannot read PTY output"),
        }
        if Instant::now() >= deadline {
            bail!("PTY produced no output before timeout");
        }
        thread::sleep(Duration::from_millis(10));
    }
}

fn drain_until_quiet(
    master: &mut File,
    quiet_period: Duration,
    timeout: Duration,
) -> Result<usize> {
    let deadline = Instant::now() + timeout;
    let mut quiet_deadline = Instant::now() + quiet_period;
    let mut total = 0;
    let mut buffer = [0_u8; 4096];
    loop {
        match master.read(&mut buffer) {
            Ok(0) => bail!("PTY closed while startup output was settling"),
            Ok(count) => {
                total += count;
                quiet_deadline = Instant::now() + quiet_period;
            }
            Err(error) if error.kind() == io::ErrorKind::WouldBlock => {
                if Instant::now() >= quiet_deadline {
                    return Ok(total);
                }
            }
            Err(error) => return Err(error).context("cannot drain PTY output"),
        }
        if Instant::now() >= deadline {
            bail!("PTY startup output did not settle before timeout");
        }
        thread::sleep(Duration::from_millis(10));
    }
}

struct OwnedChild {
    child: Child,
    process_group: libc::pid_t,
    exited: bool,
}

impl OwnedChild {
    fn new(child: Child) -> Result<Self> {
        let pid = child.id() as libc::pid_t;
        let process_group = unsafe { libc::getpgid(pid) };
        if process_group == -1 {
            return Err(io::Error::last_os_error()).context("cannot inspect child process group");
        }
        if process_group != pid {
            bail!("PTY child does not own its process group");
        }
        Ok(Self {
            child,
            process_group,
            exited: false,
        })
    }

    fn signal(&self, signal: libc::c_int) -> Result<()> {
        if unsafe { libc::kill(-self.process_group, signal) } == -1 {
            return Err(io::Error::last_os_error()).context("cannot signal PTY child group");
        }
        Ok(())
    }

    fn wait(&mut self, timeout: Duration) -> Result<ExitStatus> {
        let deadline = Instant::now() + timeout;
        loop {
            if let Some(status) = self.child.try_wait().context("cannot wait for PTY child")? {
                self.exited = true;
                return Ok(status);
            }
            if Instant::now() >= deadline {
                bail!("PTY child did not exit before timeout");
            }
            thread::sleep(Duration::from_millis(10));
        }
    }

    fn reap_within(&mut self, timeout: Duration) -> bool {
        let deadline = Instant::now() + timeout;
        while Instant::now() < deadline {
            match self.child.try_wait() {
                Ok(Some(_)) => {
                    self.exited = true;
                    return true;
                }
                Ok(None) => thread::sleep(Duration::from_millis(10)),
                Err(_) => return false,
            }
        }
        false
    }
}

impl Drop for OwnedChild {
    fn drop(&mut self) {
        if self.exited || matches!(self.child.try_wait(), Ok(Some(_))) {
            return;
        }
        let _ = unsafe { libc::kill(-self.process_group, libc::SIGTERM) };
        if self.reap_within(Duration::from_millis(500)) {
            return;
        }
        let _ = self.child.kill();
        let _ = self.reap_within(Duration::from_millis(500));
    }
}

fn main() -> Result<()> {
    let mode = parse_args(env::args_os().skip(1))?;
    let fixture = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/pty-observer.sh");
    let (command, config) = match mode {
        Mode::Fixture => (
            CommandSpec {
                program: PathBuf::from("/bin/sh"),
                args: vec![fixture.display().to_string(), "resize".into()],
                cwd: None,
            },
            ObserverConfig {
                startup_timeout: Duration::from_secs(2),
                resize: Some(TerminalSize {
                    rows: 40,
                    cols: 120,
                }),
                resize_timeout: Duration::from_secs(2),
                hold: Duration::ZERO,
                exit_timeout: Duration::from_secs(2),
                termination: TerminationAction::SignalInterrupt,
            },
        ),
        Mode::FixtureExit => (
            CommandSpec {
                program: PathBuf::from("/bin/sh"),
                args: vec![fixture.display().to_string(), "exit-command".into()],
                cwd: None,
            },
            ObserverConfig {
                startup_timeout: Duration::from_secs(2),
                resize: Some(TerminalSize {
                    rows: 40,
                    cols: 120,
                }),
                resize_timeout: Duration::from_secs(2),
                hold: Duration::ZERO,
                exit_timeout: Duration::from_secs(2),
                termination: TerminationAction::ExitCommand,
            },
        ),
        Mode::FixtureTypedExit => (
            CommandSpec {
                program: PathBuf::from("/bin/sh"),
                args: vec![fixture.display().to_string(), "exit-command".into()],
                cwd: None,
            },
            ObserverConfig {
                startup_timeout: Duration::from_secs(2),
                resize: Some(TerminalSize {
                    rows: 40,
                    cols: 120,
                }),
                resize_timeout: Duration::from_secs(2),
                hold: Duration::ZERO,
                exit_timeout: Duration::from_secs(2),
                termination: TerminationAction::TypedExitCommand,
            },
        ),
        Mode::FixtureCtrlC => (
            CommandSpec {
                program: PathBuf::from("/bin/sh"),
                args: vec![fixture.display().to_string(), "ctrl-c-key".into()],
                cwd: None,
            },
            ObserverConfig {
                startup_timeout: Duration::from_secs(2),
                resize: Some(TerminalSize {
                    rows: 40,
                    cols: 120,
                }),
                resize_timeout: Duration::from_secs(2),
                hold: Duration::ZERO,
                exit_timeout: Duration::from_secs(2),
                termination: TerminationAction::CtrlCKey,
            },
        ),
        Mode::Observe { program, cwd, hold } => (
            CommandSpec {
                program,
                args: Vec::new(),
                cwd: Some(cwd),
            },
            ObserverConfig {
                startup_timeout: Duration::from_secs(90),
                resize: Some(TerminalSize {
                    rows: 40,
                    cols: 120,
                }),
                resize_timeout: Duration::from_secs(15),
                hold,
                exit_timeout: Duration::from_secs(15),
                termination: TerminationAction::SignalInterrupt,
            },
        ),
        Mode::ObserveExit { program, cwd, hold } => (
            CommandSpec {
                program,
                args: Vec::new(),
                cwd: Some(cwd),
            },
            ObserverConfig {
                startup_timeout: Duration::from_secs(90),
                resize: Some(TerminalSize {
                    rows: 40,
                    cols: 120,
                }),
                resize_timeout: Duration::from_secs(15),
                hold,
                exit_timeout: Duration::from_secs(15),
                termination: TerminationAction::ExitCommand,
            },
        ),
        Mode::ObserveTypedExit { program, cwd, hold } => (
            CommandSpec {
                program,
                args: Vec::new(),
                cwd: Some(cwd),
            },
            ObserverConfig {
                startup_timeout: Duration::from_secs(90),
                resize: Some(TerminalSize {
                    rows: 40,
                    cols: 120,
                }),
                resize_timeout: Duration::from_secs(15),
                hold,
                exit_timeout: Duration::from_secs(15),
                termination: TerminationAction::TypedExitCommand,
            },
        ),
        Mode::ObserveCtrlC { program, cwd, hold } => (
            CommandSpec {
                program,
                args: Vec::new(),
                cwd: Some(cwd),
            },
            ObserverConfig {
                startup_timeout: Duration::from_secs(90),
                resize: Some(TerminalSize {
                    rows: 40,
                    cols: 120,
                }),
                resize_timeout: Duration::from_secs(15),
                hold,
                exit_timeout: Duration::from_secs(15),
                termination: TerminationAction::CtrlCKey,
            },
        ),
    };

    let stdout = io::stdout();
    let mut output = stdout.lock();
    let mut output_error = None;
    let observation = observe_with_events(&command, &config, |event| {
        if output_error.is_none() {
            let line = serde_json::json!({ "event": event });
            if let Err(error) = writeln!(output, "{line}").and_then(|_| output.flush()) {
                output_error = Some(error);
            }
        }
    })?;
    if let Some(error) = output_error {
        return Err(error).context("cannot write observer event");
    }
    let summary = serde_json::json!({
        "event": "summary",
        "initial_bytes": observation.initial_bytes,
        "resized_bytes": observation.resized_bytes,
        "exit_code": observation.exit_code,
        "exit_signal": observation.exit_signal,
    });
    writeln!(output, "{summary}").context("cannot write observer summary")?;
    output.flush().context("cannot flush observer summary")?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::OsString;
    use std::fs;

    #[derive(Default)]
    struct RecordingWriter {
        writes: Vec<Vec<u8>>,
    }

    impl Write for RecordingWriter {
        fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
            self.writes.push(bytes.to_vec());
            Ok(bytes.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    fn fixture() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/pty-observer.sh")
    }

    #[test]
    fn fixture_emits_activity_and_terminates_on_sigint() {
        let fixture = fixture();
        let observation = observe(
            &CommandSpec {
                program: PathBuf::from("/bin/sh"),
                args: vec![fixture.display().to_string()],
                cwd: None,
            },
            &ObserverConfig {
                startup_timeout: Duration::from_secs(2),
                resize: None,
                resize_timeout: Duration::from_secs(2),
                hold: Duration::ZERO,
                exit_timeout: Duration::from_secs(2),
                termination: TerminationAction::SignalInterrupt,
            },
        )
        .expect("fixture observation should succeed");

        assert!(observation.initial_bytes > 0);
        assert_eq!(observation.exit_code, None);
        assert_eq!(observation.exit_signal, Some(libc::SIGINT));
    }

    #[test]
    fn fixture_emits_activity_after_terminal_resize() {
        let fixture = fixture();
        let observation = observe(
            &CommandSpec {
                program: PathBuf::from("/bin/sh"),
                args: vec![fixture.display().to_string(), "resize".into()],
                cwd: None,
            },
            &ObserverConfig {
                startup_timeout: Duration::from_secs(2),
                resize: Some(TerminalSize {
                    rows: 40,
                    cols: 120,
                }),
                resize_timeout: Duration::from_secs(2),
                hold: Duration::ZERO,
                exit_timeout: Duration::from_secs(2),
                termination: TerminationAction::SignalInterrupt,
            },
        )
        .expect("fixture observation should succeed");

        assert!(observation.initial_bytes > 0);
        assert!(observation.resized_bytes > 0);
        assert_eq!(observation.exit_code, None);
        assert_eq!(observation.exit_signal, Some(libc::SIGINT));
    }

    #[test]
    fn startup_timeout_removes_the_owned_fixture_process() {
        let fixture = fixture();
        let temporary = tempfile::tempdir().expect("temporary directory should exist");
        let pid_file = temporary.path().join("fixture.pid");
        let started = Instant::now();
        let error = observe(
            &CommandSpec {
                program: PathBuf::from("/bin/sh"),
                args: vec![
                    fixture.display().to_string(),
                    "quiet".into(),
                    pid_file.display().to_string(),
                ],
                cwd: None,
            },
            &ObserverConfig {
                startup_timeout: Duration::from_millis(50),
                resize: None,
                resize_timeout: Duration::from_secs(2),
                hold: Duration::ZERO,
                exit_timeout: Duration::from_secs(2),
                termination: TerminationAction::SignalInterrupt,
            },
        )
        .expect_err("quiet fixture should time out");

        assert!(error.to_string().contains("produced no output"));
        assert!(started.elapsed() < Duration::from_secs(1));
        let pid: libc::pid_t = fs::read_to_string(pid_file)
            .expect("fixture should record its pid")
            .parse()
            .expect("fixture pid should be numeric");
        assert_eq!(unsafe { libc::kill(pid, 0) }, -1);
        assert_eq!(io::Error::last_os_error().raw_os_error(), Some(libc::ESRCH));
    }

    #[test]
    fn fixture_emits_the_structured_observer_sequence() {
        let fixture = fixture();
        let mut events = Vec::new();
        observe_with_events(
            &CommandSpec {
                program: PathBuf::from("/bin/sh"),
                args: vec![fixture.display().to_string(), "resize".into()],
                cwd: None,
            },
            &ObserverConfig {
                startup_timeout: Duration::from_secs(2),
                resize: Some(TerminalSize {
                    rows: 40,
                    cols: 120,
                }),
                resize_timeout: Duration::from_secs(2),
                hold: Duration::ZERO,
                exit_timeout: Duration::from_secs(2),
                termination: TerminationAction::SignalInterrupt,
            },
            |event| events.push(event),
        )
        .expect("fixture observation should succeed");

        assert_eq!(
            events,
            [
                "started",
                "activity_initial",
                "resized",
                "activity_resized",
                "concurrent_window",
                "signal",
                "exited",
            ]
        );
    }

    #[test]
    fn command_interface_accepts_fixture_and_live_observation_modes() {
        assert!(matches!(
            parse_args([OsString::from("fixture")]),
            Ok(Mode::Fixture)
        ));
        assert!(matches!(
            parse_args([OsString::from("fixture-exit")]),
            Ok(Mode::FixtureExit)
        ));
        assert!(matches!(
            parse_args([OsString::from("fixture-typed-exit")]),
            Ok(Mode::FixtureTypedExit)
        ));
        assert!(matches!(
            parse_args([OsString::from("fixture-ctrl-c")]),
            Ok(Mode::FixtureCtrlC)
        ));
        let mode = parse_args([
            OsString::from("observe"),
            OsString::from("/immutable/shim"),
            OsString::from("/project"),
            OsString::from("20"),
        ])
        .expect("live observer arguments should parse");
        match mode {
            Mode::Observe { program, cwd, hold } => {
                assert_eq!(program, PathBuf::from("/immutable/shim"));
                assert_eq!(cwd, PathBuf::from("/project"));
                assert_eq!(hold, Duration::from_secs(20));
            }
            Mode::Fixture
            | Mode::FixtureExit
            | Mode::FixtureTypedExit
            | Mode::FixtureCtrlC
            | Mode::ObserveExit { .. }
            | Mode::ObserveTypedExit { .. }
            | Mode::ObserveCtrlC { .. } => {
                panic!("expected live observation mode")
            }
        }

        let mode = parse_args([
            OsString::from("observe-exit"),
            OsString::from("/immutable/shim"),
            OsString::from("/project"),
            OsString::from("20"),
        ])
        .expect("live exit-command observer arguments should parse");
        assert!(matches!(mode, Mode::ObserveExit { .. }));

        let mode = parse_args([
            OsString::from("observe-typed-exit"),
            OsString::from("/immutable/shim"),
            OsString::from("/project"),
            OsString::from("20"),
        ])
        .expect("live typed-exit observer arguments should parse");
        assert!(matches!(mode, Mode::ObserveTypedExit { .. }));

        let mode = parse_args([
            OsString::from("observe-ctrl-c"),
            OsString::from("/immutable/shim"),
            OsString::from("/project"),
            OsString::from("5"),
        ])
        .expect("live Ctrl-C-key observer arguments should parse");
        assert!(matches!(mode, Mode::ObserveCtrlC { .. }));
    }

    #[test]
    fn buffered_startup_output_does_not_count_as_resize_activity() {
        let fixture = fixture();
        let error = observe(
            &CommandSpec {
                program: PathBuf::from("/bin/sh"),
                args: vec![fixture.display().to_string(), "no-redraw".into()],
                cwd: None,
            },
            &ObserverConfig {
                startup_timeout: Duration::from_secs(2),
                resize: Some(TerminalSize {
                    rows: 40,
                    cols: 120,
                }),
                resize_timeout: Duration::from_millis(100),
                hold: Duration::ZERO,
                exit_timeout: Duration::from_secs(2),
                termination: TerminationAction::SignalInterrupt,
            },
        )
        .expect_err("fixture without redraw should fail resize observation");

        assert!(error.to_string().contains("produced no output"));
    }

    #[test]
    fn exit_command_writes_exact_terminal_bytes() {
        let mut bytes = Vec::new();
        write_exit_command(&mut bytes).expect("exit command should be writable");
        assert_eq!(bytes, b"/exit\r");
    }

    #[test]
    fn typed_exit_command_writes_separate_keys_with_fixed_delays() {
        let mut output = RecordingWriter::default();
        let mut delays = Vec::new();

        write_typed_exit_command_with_delay(&mut output, |delay| delays.push(delay))
            .expect("typed exit command should be writable");

        assert_eq!(
            output.writes,
            [b"/".as_slice(), b"e", b"x", b"i", b"t", b"\r"]
        );
        assert_eq!(delays, [Duration::from_millis(20); 5]);
    }

    #[test]
    fn ctrl_c_key_writes_one_control_byte_once() {
        let mut output = RecordingWriter::default();

        write_ctrl_c_key(&mut output).expect("Ctrl-C key should be writable");

        assert_eq!(output.writes, [vec![0x03]]);
    }

    #[test]
    fn fixture_exit_command_preserves_distinctive_status_and_events() {
        let fixture = fixture();
        let mut events = Vec::new();
        let observation = observe_with_events(
            &CommandSpec {
                program: PathBuf::from("/bin/sh"),
                args: vec![fixture.display().to_string(), "exit-command".into()],
                cwd: None,
            },
            &ObserverConfig {
                startup_timeout: Duration::from_secs(2),
                resize: Some(TerminalSize {
                    rows: 40,
                    cols: 120,
                }),
                resize_timeout: Duration::from_secs(2),
                hold: Duration::ZERO,
                exit_timeout: Duration::from_secs(2),
                termination: TerminationAction::ExitCommand,
            },
            |event| events.push(event),
        )
        .expect("fixture should accept the exit command");

        assert!(observation.initial_bytes > 0);
        assert!(observation.resized_bytes > 0);
        assert_eq!(observation.exit_code, Some(23));
        assert_eq!(observation.exit_signal, None);
        assert_eq!(
            events,
            [
                "started",
                "activity_initial",
                "resized",
                "activity_resized",
                "concurrent_window",
                "exit_command",
                "exited",
            ]
        );
    }

    #[test]
    fn fixture_typed_exit_preserves_distinctive_status_and_events() {
        let fixture = fixture();
        let mut events = Vec::new();
        let observation = observe_with_events(
            &CommandSpec {
                program: PathBuf::from("/bin/sh"),
                args: vec![fixture.display().to_string(), "exit-command".into()],
                cwd: None,
            },
            &ObserverConfig {
                startup_timeout: Duration::from_secs(2),
                resize: Some(TerminalSize {
                    rows: 40,
                    cols: 120,
                }),
                resize_timeout: Duration::from_secs(2),
                hold: Duration::ZERO,
                exit_timeout: Duration::from_secs(2),
                termination: TerminationAction::TypedExitCommand,
            },
            |event| events.push(event),
        )
        .expect("fixture should accept the typed exit command");

        assert!(observation.initial_bytes > 0);
        assert!(observation.resized_bytes > 0);
        assert_eq!(observation.exit_code, Some(23));
        assert_eq!(observation.exit_signal, None);
        assert_eq!(
            events,
            [
                "started",
                "activity_initial",
                "resized",
                "activity_resized",
                "concurrent_window",
                "typed_exit_command",
                "exited",
            ]
        );
    }

    #[test]
    fn fixture_ctrl_c_key_preserves_distinctive_status_and_events() {
        let fixture = fixture();
        let mut events = Vec::new();
        let observation = observe_with_events(
            &CommandSpec {
                program: PathBuf::from("/bin/sh"),
                args: vec![fixture.display().to_string(), "ctrl-c-key".into()],
                cwd: None,
            },
            &ObserverConfig {
                startup_timeout: Duration::from_secs(2),
                resize: Some(TerminalSize {
                    rows: 40,
                    cols: 120,
                }),
                resize_timeout: Duration::from_secs(2),
                hold: Duration::ZERO,
                exit_timeout: Duration::from_secs(2),
                termination: TerminationAction::CtrlCKey,
            },
            |event| events.push(event),
        )
        .expect("fixture should accept the Ctrl-C key");

        assert!(observation.initial_bytes > 0);
        assert!(observation.resized_bytes > 0);
        assert_eq!(observation.exit_code, Some(23));
        assert_eq!(observation.exit_signal, None);
        assert_eq!(
            events,
            [
                "started",
                "activity_initial",
                "resized",
                "activity_resized",
                "concurrent_window",
                "ctrl_c_key",
                "exited",
            ]
        );
    }

    #[test]
    fn exit_timeout_removes_the_owned_fixture_process() {
        let fixture = fixture();
        let temporary = tempfile::tempdir().expect("temporary directory should exist");
        let pid_file = temporary.path().join("fixture.pid");
        let error = observe(
            &CommandSpec {
                program: PathBuf::from("/bin/sh"),
                args: vec![
                    fixture.display().to_string(),
                    "ignore-exit".into(),
                    pid_file.display().to_string(),
                ],
                cwd: None,
            },
            &ObserverConfig {
                startup_timeout: Duration::from_secs(2),
                resize: None,
                resize_timeout: Duration::from_secs(2),
                hold: Duration::ZERO,
                exit_timeout: Duration::from_millis(50),
                termination: TerminationAction::ExitCommand,
            },
        )
        .expect_err("fixture that ignores the exit command should time out");

        assert!(error.to_string().contains("did not exit"));
        let pid: libc::pid_t = fs::read_to_string(pid_file)
            .expect("fixture should record its pid")
            .parse()
            .expect("fixture pid should be numeric");
        assert_eq!(unsafe { libc::kill(pid, 0) }, -1);
        assert_eq!(io::Error::last_os_error().raw_os_error(), Some(libc::ESRCH));
    }

    #[test]
    fn typed_exit_timeout_removes_the_owned_fixture_process() {
        let fixture = fixture();
        let temporary = tempfile::tempdir().expect("temporary directory should exist");
        let pid_file = temporary.path().join("fixture.pid");
        let error = observe(
            &CommandSpec {
                program: PathBuf::from("/bin/sh"),
                args: vec![
                    fixture.display().to_string(),
                    "ignore-exit".into(),
                    pid_file.display().to_string(),
                ],
                cwd: None,
            },
            &ObserverConfig {
                startup_timeout: Duration::from_secs(2),
                resize: None,
                resize_timeout: Duration::from_secs(2),
                hold: Duration::ZERO,
                exit_timeout: Duration::from_millis(50),
                termination: TerminationAction::TypedExitCommand,
            },
        )
        .expect_err("fixture that ignores typed exit should time out");

        assert!(error.to_string().contains("did not exit"));
        let pid: libc::pid_t = fs::read_to_string(pid_file)
            .expect("fixture should record its pid")
            .parse()
            .expect("fixture pid should be numeric");
        assert_eq!(unsafe { libc::kill(pid, 0) }, -1);
        assert_eq!(io::Error::last_os_error().raw_os_error(), Some(libc::ESRCH));
    }

    #[test]
    fn ctrl_c_key_timeout_removes_the_owned_fixture_process() {
        let fixture = fixture();
        let temporary = tempfile::tempdir().expect("temporary directory should exist");
        let pid_file = temporary.path().join("fixture.pid");
        let error = observe(
            &CommandSpec {
                program: PathBuf::from("/bin/sh"),
                args: vec![
                    fixture.display().to_string(),
                    "ignore-ctrl-c".into(),
                    pid_file.display().to_string(),
                ],
                cwd: None,
            },
            &ObserverConfig {
                startup_timeout: Duration::from_secs(2),
                resize: None,
                resize_timeout: Duration::from_secs(2),
                hold: Duration::ZERO,
                exit_timeout: Duration::from_millis(50),
                termination: TerminationAction::CtrlCKey,
            },
        )
        .expect_err("fixture that ignores Ctrl-C key should time out");

        assert!(error.to_string().contains("did not exit"));
        let pid: libc::pid_t = fs::read_to_string(pid_file)
            .expect("fixture should record its pid")
            .parse()
            .expect("fixture pid should be numeric");
        assert_eq!(unsafe { libc::kill(pid, 0) }, -1);
        assert_eq!(io::Error::last_os_error().raw_os_error(), Some(libc::ESRCH));
    }
}
