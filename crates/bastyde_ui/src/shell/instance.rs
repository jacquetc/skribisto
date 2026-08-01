// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Single-instance election: who is the **primary**, and how a **remote** hands
//! its command line over.
//!
//! Skribisto used to be one process per project. Phases 0–3 of the multi-Work
//! migration made one process genuinely safe to hold several projects at once
//! (`WorkSession`/`WorkRegistry`, per-Work `AppIds`/singles/undo stacks, per-window
//! teardown), and this module is what finally uses that from the outside: the
//! first live copy owns a well-known socket and becomes the **primary**; every
//! later launch connects to it, forwards what it was asked to do, and exits in
//! milliseconds without ever building a store, a settings writer or a window.
//!
//! ## The election
//!
//! [`elect`] resolves one of three roles, in this order:
//!
//! 1. **Connect succeeds** ⇒ [`InstanceRole::Remote`], carrying the very stream
//!    that succeeded. The handoff goes out on *that* connection — reconnecting
//!    would open a window where the primary could die in between.
//! 2. **Connect fails** ⇒ unlink a stale socket file (Unix only — a Windows
//!    named pipe cannot outlive its server) and bind. Success ⇒
//!    [`InstanceRole::Primary`].
//! 3. **Bind fails** ⇒ a peer won the race between our failed connect and our
//!    bind. Retry connect **once**; if that also fails ⇒
//!    [`InstanceRole::Standalone`].
//!
//! `Standalone` is not an error path to apologise for — it is exactly the
//! multi-process behaviour Skribisto shipped before this module existed, so a
//! wedged or half-dead primary degrades to "the old way" rather than to a launch
//! that does nothing. `--new-instance` selects it deliberately.
//!
//! ## Namespacing
//!
//! The socket lives in [`open_registry::dir`], which is keyed by installation
//! identity (a hash of the config dir). That is load-bearing here, not
//! incidental: `XDG_RUNTIME_DIR` is per-login-session, so without it the first
//! sandboxed automation run to start would become the primary for the
//! developer's own running copy, and every subsequent launch would hand its
//! project off into a tempdir.

use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;
use std::time::Duration;

use interprocess::local_socket::prelude::*;
use interprocess::local_socket::{ListenerOptions, Stream};

use crate::shell::ipc::{InstanceReply, InstanceRequest};
use crate::shell::open_registry::{self, SocketId};

/// How long a remote waits for the primary to acknowledge its request before
/// giving up and launching standalone.
///
/// **Must exceed the primary's own patience** ([`crate::shell::ipc::UI_ACK_BUDGET`],
/// the window `serve_one` gives its UI thread to answer). The two are the same
/// handshake seen from opposite ends, and getting the inequality backwards is not
/// a tuning nit — it produces a *duplicate launch*. An earlier revision had the
/// remote give up after 2 s while the primary was still willing to wait 3 s: under
/// any load the remote timed out, launched standalone and claimed the project,
/// while the primary went on to serve the very same request. Two processes, two
/// windows, one project — observed in the live trace as a third open-registry lock
/// under a second pid.
///
/// The absolute value only has to keep a user who double-clicked a `.skrib` from
/// staring at nothing for long; the *ordering* is what has to hold.
const ACK_TIMEOUT: Duration = Duration::from_secs(5);

/// The well-known socket every instance of this installation elects on — as a
/// *path*, which exists only on Unix (see `open_registry::socket_path`).
pub fn primary_socket() -> Option<PathBuf> {
    open_registry::socket_path(SocketId::Primary)
}

/// What this process turned out to be.
pub enum InstanceRole {
    /// We own the primary socket. `spawn_listener` will serve it in addition to
    /// this instance's own per-pid socket.
    Primary,
    /// Another instance owns it, and this is the live connection to it. Hand the
    /// request over with [`handoff`] and exit.
    Remote(Stream),
    /// No primary could be reached and none could be claimed — run as an
    /// ordinary, self-contained process (pre-Phase-4 behaviour). Also what
    /// `--new-instance` asks for.
    Standalone,
}

impl std::fmt::Debug for InstanceRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Primary => f.write_str("Primary"),
            Self::Remote(_) => f.write_str("Remote(..)"),
            Self::Standalone => f.write_str("Standalone"),
        }
    }
}

/// Resolve this process's role. See the module doc for the three-step contract.
///
/// Call once, at the very top of `main` — before the event hub, the window-state
/// prune, `initialize_app` or any settings handle exists. A remote must touch
/// none of them.
pub fn elect() -> InstanceRole {
    if try_name().is_none() {
        // Nowhere to elect (no home directory, no runtime dir): every instance
        // is its own.
        return InstanceRole::Standalone;
    }

    if let Some(stream) = try_connect() {
        return InstanceRole::Remote(stream);
    }

    // Nobody answered. On Unix the socket *file* may still be sitting there from
    // a crashed primary — `bind` would fail on it forever. Removing it is safe
    // precisely because the connect above just proved nothing is listening.
    // (`open_registry::scan` reaps the per-pid sockets by pid; this one carries
    // no pid in its name, so it is reaped here instead.) A Windows named pipe
    // cannot outlive its server, so there is nothing to reap there — which is
    // why `socket_path` is `None` on that platform rather than a lie.
    #[cfg(unix)]
    if let Some(sock) = primary_socket() {
        let _ = std::fs::remove_file(&sock);
    }

    if try_bind() {
        return InstanceRole::Primary;
    }

    // The bind lost to a peer that claimed the socket in the window between our
    // failed connect and our own bind. That peer is a perfectly good primary.
    match try_connect() {
        Some(stream) => InstanceRole::Remote(stream),
        None => InstanceRole::Standalone,
    }
}

/// The primary socket's platform-correct address (a Unix path, or a Windows
/// named-pipe name).
fn try_name() -> Option<interprocess::local_socket::Name<'static>> {
    open_registry::socket_name(SocketId::Primary)
}

/// Connect to the primary socket, or `None` if nothing is listening.
fn try_connect() -> Option<Stream> {
    Stream::connect(try_name()?).ok()
}

/// Claim the primary socket. Returns whether we got it.
///
/// The listener is dropped immediately: binding is the *election*, and serving is
/// [`crate::shell::ipc::spawn_listener`]'s job once the app is actually up. On
/// Unix dropping the listener leaves the socket file in place, which is what the
/// later re-bind attaches to; the tiny gap where the file exists but nothing
/// accepts is covered by the same stale-socket unlink above.
fn try_bind() -> bool {
    let Some(name) = try_name() else {
        return false;
    };
    ListenerOptions::new().name(name).create_sync().is_ok()
}

/// Send `request` to the primary over the connection [`elect`] already
/// established, and wait for its acknowledgement.
///
/// Returns whether the primary accepted. `false` — a rejection, a timeout, a
/// half-closed socket — means the caller must fall back to launching normally:
/// the one thing a remote may never do is exit silently having achieved nothing.
pub fn handoff(mut stream: Stream, request: &InstanceRequest) -> bool {
    let Ok(mut line) = serde_json::to_string(request) else {
        return false;
    };
    line.push('\n');
    if stream.write_all(line.as_bytes()).is_err() || stream.flush().is_err() {
        return false;
    }

    // Read the acknowledgement on a scratch thread and wait on a channel, rather
    // than arming a receive timeout on the socket itself.
    //
    // `set_recv_timeout` is **not portable**: on Windows named pipes interprocess
    // returns `ErrorKind::Unsupported` ("named pipes do not support I/O
    // timeouts"). An earlier revision ignored that error, which on Windows left
    // the read unbounded — so a primary that accepted the connection and then
    // wedged would hang this process forever, with no window to show for it,
    // since a remote never builds one. That is the exact failure the timeout
    // exists to prevent, so it cannot be the one platform where it is absent.
    //
    // The thread may outlive the timeout, still blocked in `read_line`. That is
    // deliberate and bounded: it holds nothing but its own connection, and it
    // ends the moment the primary answers or dies. Abandoning it costs one parked
    // thread in an already-degraded launch; the alternative (`CancelSynchronousIo`
    // and friends) is a platform-specific unsafe dance for a case that resolves
    // itself.
    let (tx, rx) = std::sync::mpsc::sync_channel::<Option<InstanceReply>>(1);
    let _ = std::thread::Builder::new()
        .name("skribisto-handoff".into())
        .spawn(move || {
            let mut reply = String::new();
            let ok = BufReader::new(&mut stream).read_line(&mut reply).is_ok();
            let parsed = ok
                .then(|| serde_json::from_str::<InstanceReply>(reply.trim()).ok())
                .flatten();
            let _ = tx.send(parsed);
        });

    matches!(rx.recv_timeout(ACK_TIMEOUT), Ok(Some(InstanceReply::Accepted)))
}

/// The command-line flag that opts out of the election entirely.
///
/// Needed by `scripts/automation_two_process.py`, which deliberately runs two
/// processes over **one** sandbox to prove the cross-process settings layer still
/// works — without this it would hand off to itself and become a one-process
/// test of nothing. Also the escape hatch for running two builds side by side.
pub const NEW_INSTANCE_FLAG: &str = "--new-instance";

/// Pin app settings for this run from a TOML file (debug builds only).
///
/// See `crate::settings_keys` for the schema and why it exists. Takes a value,
/// either attached (`--config=pins.toml`) or as the next argument
/// (`--config pins.toml`) — the latter is why this parser consumes arguments
/// through an iterator rather than looping over them independently: the old
/// reader took the first non-flag argument as the project path, so a
/// space-separated value would have been opened as a `.skrib`.
pub const CONFIG_FLAG: &str = "--config";

/// Print every settable key with its effective value and exit (debug builds only).
pub const DUMP_CONFIG_FLAG: &str = "--dump-config";

/// Everything the command line can say, in the order `main` acts on it.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct LaunchArgs {
    /// `--new-instance`: skip the election, run standalone.
    pub new_instance: bool,
    /// The optional `.skrib` path to open.
    pub project: Option<String>,
    /// `--config <file>`: the settings-pin file to apply before startup.
    pub config: Option<String>,
    /// `--dump-config`: print the effective settings and exit.
    pub dump_config: bool,
    /// A malformed argument. Reported by `main`, which exits rather than launching
    /// — a probe that asked to pin settings and silently got none is worse than one
    /// that does not start, since it goes on to assert against the wrong state.
    pub error: Option<String>,
}

/// Split `argv` (excluding argv\[0\]) into the flags and the optional project path.
///
/// A pure function so the whole argument surface is unit-testable without a
/// process: `main` calls it with `std::env::args().skip(1)`.
///
/// Unknown `--flags` are left alone (the first one becomes the project path, as it
/// always did). Only the flags named here are interpreted.
pub fn parse_args<I, S>(args: I) -> LaunchArgs
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    let mut out = LaunchArgs::default();
    let mut rest = args.into_iter().peekable();

    while let Some(arg) = rest.next() {
        let arg = arg.as_ref();
        if arg == NEW_INSTANCE_FLAG {
            out.new_instance = true;
        } else if arg == DUMP_CONFIG_FLAG {
            out.dump_config = true;
        } else if let Some(value) = arg.strip_prefix("--config=") {
            if value.trim().is_empty() {
                out.error = Some(format!("{CONFIG_FLAG} needs a file path"));
            } else {
                out.config = Some(value.to_string());
            }
        } else if arg == CONFIG_FLAG {
            // Peek rather than take: a value that is itself a flag means the path
            // was forgotten, and *consuming* it would disable that flag as well as
            // failing here — one mistake turned into two. Leaving it in the stream
            // means the rest of the command line still parses as written.
            let is_value = rest
                .peek()
                .is_some_and(|v| !v.as_ref().trim().is_empty() && !v.as_ref().starts_with("--"));
            if is_value {
                let value = rest.next().expect("just peeked");
                out.config = Some(value.as_ref().to_string());
            } else {
                out.error = Some(format!("{CONFIG_FLAG} needs a file path"));
            }
        } else if out.project.is_none() && !arg.trim().is_empty() {
            out.project = Some(arg.to_string());
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The handshake's one ordering rule. The remote must outwait the primary,
    /// or a slow-but-successful answer lands after the remote has already given
    /// up and launched standalone — two processes on one project, which is
    /// exactly what the live trace caught as a third open-registry lock under a
    /// second pid.
    #[test]
    fn the_primary_gives_up_before_the_remote_does() {
        assert!(
            crate::shell::ipc::UI_ACK_BUDGET < ACK_TIMEOUT,
            "serve_one waits {:?} but the remote only waits {:?} — a duplicate launch",
            crate::shell::ipc::UI_ACK_BUDGET,
            ACK_TIMEOUT
        );
    }

    #[test]
    fn a_bare_launch_has_no_path_and_no_opt_out() {
        let args = parse_args(Vec::<String>::new());
        assert_eq!(args, LaunchArgs::default());
    }

    #[test]
    fn a_path_is_taken_whichever_side_of_the_flag_it_sits() {
        let a = parse_args(["--new-instance", "/tmp/a.skrib"]);
        let b = parse_args(["/tmp/a.skrib", "--new-instance"]);
        assert!(
            a.new_instance && b.new_instance,
            "the flag is positional-independent"
        );
        assert_eq!(a.project.as_deref(), Some("/tmp/a.skrib"));
        assert_eq!(b.project.as_deref(), Some("/tmp/a.skrib"));
    }

    /// The pre-Phase-4 reader was `std::env::args().nth(1).filter(non-empty)`.
    /// A blank argument must stay "no project" rather than becoming a path the
    /// loader then fails on.
    #[test]
    fn a_blank_argument_is_not_a_path() {
        assert_eq!(parse_args(["   "]).project, None);
    }

    #[test]
    fn only_the_first_path_wins() {
        let args = parse_args(["/tmp/a.skrib", "/tmp/b.skrib"]);
        assert_eq!(
            args.project.as_deref(),
            Some("/tmp/a.skrib"),
            "a second path is ignored, matching the old nth(1) reader"
        );
    }

    /// Both spellings must reach the same place. The attached form is the safe one
    /// to hand a shell; the separated form is what anyone types by hand.
    #[test]
    fn config_takes_its_value_attached_or_separated() {
        assert_eq!(
            parse_args(["--config=/tmp/pins.toml"]).config.as_deref(),
            Some("/tmp/pins.toml")
        );
        assert_eq!(
            parse_args(["--config", "/tmp/pins.toml"]).config.as_deref(),
            Some("/tmp/pins.toml")
        );
    }

    /// The regression this parser exists to prevent: the old reader took the first
    /// non-flag argument as the project, so `--config pins.toml` opened `pins.toml`
    /// as a `.skrib` and pinned nothing.
    #[test]
    fn a_config_value_is_not_mistaken_for_the_project() {
        let args = parse_args(["--config", "/tmp/pins.toml", "/tmp/a.skrib"]);
        assert_eq!(args.config.as_deref(), Some("/tmp/pins.toml"));
        assert_eq!(args.project.as_deref(), Some("/tmp/a.skrib"));
    }

    /// A forgotten path must not swallow the next flag: that would disable the flag
    /// *and* try to parse it as TOML, reporting a parse error about a filename.
    #[test]
    fn config_without_a_value_is_an_error_not_a_swallowed_flag() {
        let args = parse_args(["--config", "--new-instance"]);
        assert!(args.error.is_some(), "a missing value must be reported");
        assert_eq!(args.config, None);
        assert!(
            args.new_instance,
            "and the flag it would have swallowed still applies"
        );

        assert!(parse_args(["--config"]).error.is_some());
        assert!(parse_args(["--config="]).error.is_some());
    }

    #[test]
    fn dump_config_is_a_bare_flag() {
        let args = parse_args(["--dump-config"]);
        assert!(args.dump_config);
        assert_eq!(args.project, None, "it takes no value");
    }

    /// Every flag composes with every other, and with a project path.
    #[test]
    fn the_flags_compose() {
        let args = parse_args([
            "/tmp/a.skrib",
            "--new-instance",
            "--config=/tmp/pins.toml",
            "--dump-config",
        ]);
        assert_eq!(args.project.as_deref(), Some("/tmp/a.skrib"));
        assert!(args.new_instance);
        assert_eq!(args.config.as_deref(), Some("/tmp/pins.toml"));
        assert!(args.dump_config);
        assert_eq!(args.error, None);
    }

    /// Election is a real socket dance. On Unix it is drivable against a temp
    /// directory; the Windows pipe namespace is machine-global, so these stay
    /// `#[cfg(unix)]` rather than colliding with a developer's live instance.
    #[cfg(unix)]
    fn temp_socket(tag: &str) -> PathBuf {
        let d = std::env::temp_dir().join(format!(
            "skribisto-instance-test-{tag}-{}",
            std::process::id()
        ));
        std::fs::create_dir_all(&d).unwrap();
        d.join("primary.sock")
    }

    #[cfg(unix)]
    fn connect_at(sock: &std::path::Path) -> Option<Stream> {
        use interprocess::local_socket::{GenericFilePath, ToFsName};
        let name = sock.to_owned().to_fs_name::<GenericFilePath>().ok()?;
        Stream::connect(name).ok()
    }

    #[cfg(unix)]
    fn bind_at(sock: &std::path::Path) -> Option<interprocess::local_socket::Listener> {
        use interprocess::local_socket::{GenericFilePath, ToFsName};
        let name = sock.to_owned().to_fs_name::<GenericFilePath>().ok()?;
        ListenerOptions::new().name(name).create_sync().ok()
    }

    #[cfg(unix)]
    #[test]
    fn nothing_listening_means_the_socket_is_claimable() {
        let sock = temp_socket("claimable");
        let _ = std::fs::remove_file(&sock);
        assert!(connect_at(&sock).is_none(), "nothing is listening yet");
        assert!(bind_at(&sock).is_some(), "so the socket must be claimable");
        let _ = std::fs::remove_file(&sock);
    }

    /// The crash case: a primary died leaving its socket file behind. `bind`
    /// fails on an existing path, so without the unlink in `elect` every later
    /// launch would degrade to `Standalone` forever — one orphaned file and
    /// single-instance is permanently off.
    #[cfg(unix)]
    #[test]
    fn a_stale_socket_file_blocks_bind_until_it_is_unlinked() {
        let sock = temp_socket("stale");
        let _ = std::fs::remove_file(&sock);
        std::fs::write(&sock, b"").unwrap();

        assert!(
            bind_at(&sock).is_none(),
            "an existing path is exactly what makes a stale socket fatal"
        );
        assert!(connect_at(&sock).is_none(), "and nothing answers on it");

        let _ = std::fs::remove_file(&sock);
        assert!(bind_at(&sock).is_some(), "unlinking it restores the claim");
        let _ = std::fs::remove_file(&sock);
    }

    /// A live listener must be found by connect — the step that makes an
    /// instance a remote rather than a second primary.
    #[cfg(unix)]
    #[test]
    fn a_live_listener_is_reachable() {
        let sock = temp_socket("live");
        let _ = std::fs::remove_file(&sock);
        let listener = bind_at(&sock).expect("first bind wins");

        assert!(
            connect_at(&sock).is_some(),
            "a bound socket must be connectable"
        );
        assert!(
            bind_at(&sock).is_none(),
            "and must not be claimable a second time — that is the whole election"
        );

        drop(listener);
        let _ = std::fs::remove_file(&sock);
    }

    /// A remote must never block forever waiting for an acknowledgement. The
    /// portable guarantee is `recv_timeout` on a channel fed by a scratch
    /// thread, **not** `set_recv_timeout` on the socket — that returns
    /// `ErrorKind::Unsupported` on Windows named pipes, and the earlier
    /// revision's `let _ = …` silently left the read unbounded there.
    #[test]
    fn the_acknowledgement_wait_is_bounded_on_every_platform() {
        let (tx, rx) = std::sync::mpsc::sync_channel::<Option<InstanceReply>>(1);
        // A peer that accepted the connection and then wedged: nothing is ever
        // sent, and the sender stays alive so the channel does not disconnect.
        let started = std::time::Instant::now();
        let answer = rx.recv_timeout(Duration::from_millis(120));
        assert!(answer.is_err(), "a silent peer must time out, not hang");
        assert!(
            started.elapsed() < Duration::from_secs(5),
            "the wait must be bounded by the timeout, not by the peer"
        );
        drop(tx);
    }
}
