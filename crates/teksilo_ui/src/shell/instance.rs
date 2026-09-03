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
//! ## Bootstrap
//!
//! [`bootstrap`] is what `run()` actually calls, at the very top of `main`: it
//! reattaches the console (Windows), installs the panic hook, parses argv,
//! applies/dumps the settings-schema flags, then runs the election below and
//! hands off to a primary if one answers. `run()` never touches
//! `parse_args`/`elect`/`handoff` directly — [`Bootstrap::Exit`] tells it there
//! is nothing left to build (a dump printed, or a remote handed off);
//! [`Bootstrap::Continue`] carries the two things it still needs.
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
// Only [`primary_socket`] needs it, and that is Unix-only. See its own cfg.
#[cfg(unix)]
use std::path::PathBuf;
use std::time::Duration;

use interprocess::local_socket::prelude::*;
use interprocess::local_socket::{ListenerOptions, Stream};

use crate::cli;
use crate::crash_report;
use crate::shell::ipc::{self, InstanceReply, InstanceRequest};
use crate::shell::open_registry::{self, SocketId};

/// How long a remote waits for the primary to acknowledge its request before
/// giving up and launching standalone.
///
/// **Must exceed the primary's own patience** ([`crate::shell::ipc::UI_ACK_BUDGET`],
/// the window `serve_one` gives its UI thread to answer) — the two are the same
/// handshake seen from opposite ends, and getting the inequality backwards
/// produces a *duplicate launch*: the remote gives up and launches standalone
/// while the primary is still about to serve the very same request.
///
/// The absolute value only has to keep a user who double-clicked a `.skrib`
/// from staring at nothing for long; the *ordering* is what has to hold.
const ACK_TIMEOUT: Duration = Duration::from_secs(5);

/// The well-known socket every instance of this installation elects on — as a
/// *path*, which exists only on Unix (see `open_registry::socket_path`).
///
/// `#[cfg(unix)]` because its one caller is: `elect`, reaping a crashed
/// primary's stale socket file, itself under the same gate. A Windows named
/// pipe is not a filesystem object and cannot outlive its server, so there is
/// nothing to reap there and `open_registry::socket_path` answers `None` by
/// design. Without this gate the function still compiled on a Windows target
/// with no caller left, which is a `dead_code` warning that says nothing about
/// the code and everything about the platform. Matching the caller's cfg states
/// the portability fact instead.
#[cfg(unix)]
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
    // than a receive timeout on the socket itself: `set_recv_timeout` returns
    // `ErrorKind::Unsupported` on Windows named pipes, which would leave the
    // read unbounded there — a primary that accepted the connection and then
    // wedged would hang this process forever, with no window to show for it.
    //
    // The thread may outlive the timeout, still blocked in `read_line` —
    // bounded and harmless: it holds nothing but its own connection, and it
    // ends the moment the primary answers or dies.
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

    matches!(
        rx.recv_timeout(ACK_TIMEOUT),
        Ok(Some(InstanceReply::Accepted))
    )
}

/// What [`bootstrap`] decided, and what `run()` needs in order to keep going.
pub(crate) enum Bootstrap {
    /// Already handled — a remote handed its request off to the primary, or
    /// `--dump-config` printed and exited. `run()` returns immediately without
    /// building an `AppContext` or a window.
    Exit,
    /// A primary or standalone instance, free to build the rest of the app.
    Continue {
        /// The `.skrib` path from argv, if any.
        initial_project: Option<String>,
        /// Whether this process won the election (and so must also serve the
        /// well-known socket once the app is up, not just its own per-pid one).
        is_primary: bool,
        /// The `--translation-dev` directories to hot-reload, already checked
        /// against the filesystem. Empty in every ordinary launch.
        translation_dev: Vec<(teksilo::prelude::LanguageIdentifier, std::path::PathBuf)>,
    },
}

/// Everything that has to happen before an `AppContext` exists: reattach the
/// console (Windows), install the panic hook, parse argv, apply/dump the
/// settings-schema flags, then run the election and hand off to a primary if
/// one answers.
///
/// Call once, at the very top of `run()` — before the event hub, the
/// window-state prune, `initialize_app` or any settings handle exists. See the
/// module doc for why: a remote must touch none of them.
pub(crate) fn bootstrap() -> Bootstrap {
    // ── Windows: rejoin the launching terminal, if any ────────────────────────
    //
    // The binaries are linked for the GUI subsystem (`#![windows_subsystem =
    // "windows"]` in `src/bin/skribisto.rs`), which also detaches stdout/stderr.
    // Reattach to the parent's console so `--dump-config`, pin-validation errors
    // and panic reports still print when launched from a terminal. Failure just
    // means no console to join (a double-click) — the normal GUI case. Redirected
    // handles (`> file`) are set via STARTF_USESTDHANDLES and survive the attach.
    #[cfg(windows)]
    unsafe {
        use ::windows::Win32::System::Console::{ATTACH_PARENT_PROCESS, AttachConsole};
        let _ = AttachConsole(ATTACH_PARENT_PROCESS);
    }

    // ── Panic diagnostics — before anything at all ────────────────────────────
    //
    // Ahead of even the election, so a panic while parsing arguments or binding
    // the instance socket is still recorded. The hook chains to libstd's, takes
    // no application locks, and writes one file; see `crash_report`'s module doc
    // for why it deliberately does not try to dump prose.
    crash_report::install();

    // ── Single-instance election — FIRST, before anything is built ────────────
    //
    // A remote must not construct an `AppContext`, start the event-dispatch
    // thread, run `initialize_app`, prune `window_state.toml`, or open a settings
    // handle: all of those touch state the primary is concurrently using, on
    // behalf of a process that is about to exit. Everything below this block is
    // therefore reachable only by a primary or a standalone instance.
    //
    // `--new-instance`, `--config`, `--dump-config`, `--style`,
    // `--translation-dev` and a bare `.skrib` path are the whole argument
    // surface; `parse_args` is a pure function so that surface is unit-tested.
    let args = parse_args(std::env::args().skip(1));
    let initial_project = args.project.clone();
    if let Some(error) = &args.error {
        eprintln!("skribisto: {error}");
        std::process::exit(2);
    }

    // ── The design language, before anything at all is built ─────────────────
    //
    // Earlier than the settings flags below and earlier than the election, for
    // the same reason `identity` is: widget chrome is resolved when a widget is
    // built, so the style has to be in place before the first `AppContext`, let
    // alone the first window. Nothing here reads settings, so it is free to run
    // this early — and `--dump-config` exits below without ever using it.
    if let Some(style) = args.style {
        crate::style::install(style);
    }

    // ── The settings-schema flags, before anything else touches settings ──────
    //
    // Both are debug-only and both run ahead of the election: `--config` must
    // land its pins on disk before `read_prefs` (below) reads the very keys it
    // may be pinning, and `--dump-config` exits without building anything.
    //
    // Pins are applied *before* a dump, so `--config pins.toml --dump-config`
    // reads as "apply these, then show me what I get" — a validate-and-preview
    // pass that needs no window. Ordering the other way would print the state a
    // launch was about to leave behind, which is a strictly less useful answer to
    // the question the pair asks.
    if let Some(path) = &args.config {
        cli::apply_config_pins(path);
    }
    if args.dump_config {
        cli::run_dump_config();
        return Bootstrap::Exit;
    }

    // ── The translator's hot-reload directories ───────────────────────────────
    //
    // After the settings flags because it is fatal on a bad path and there is no
    // reason to have written pins first; before the election because it decides
    // the role below. Ordinary launches pass none and this is a no-op.
    let translation_dev = cli::resolve_translation_dev(&args.translation_dev);

    // The desktop's own startup token (a file-manager double-click sets it), so
    // whichever window the primary ends up showing can actually come forward on
    // Wayland — a process cannot raise itself unprompted.
    let launch_token = std::env::var("XDG_ACTIVATION_TOKEN").ok();

    // `--config` implies standalone. A remote hands its command line to the
    // primary and exits, and the primary's windows are already running against
    // their own settings — so an elected-away `--config` run would pin nothing
    // and silently observe a differently-configured app, which is precisely the
    // failure this flag exists to remove.
    //
    // `--style` implies it for the same reason and one worse: the primary built
    // its widgets in whatever style *it* was launched with, and a style cannot
    // be applied to a tree that already exists. An elected-away `--style` run
    // would hand its project to a window in the previous design language and
    // exit reporting success.
    //
    // `--translation-dev` implies it for a third reason: bundles are built once,
    // in `run()`, from the config this process assembles. A run elected away to a
    // primary would hand over its project and exit, leaving the translator
    // editing `.ftl` files against an app that never registered a watcher, the
    // exact silence the flag exists to break.
    let role = if args.new_instance
        || args.config.is_some()
        || args.style.is_some()
        || !translation_dev.is_empty()
    {
        InstanceRole::Standalone
    } else {
        elect()
    };
    let is_primary = matches!(role, InstanceRole::Primary);
    if let InstanceRole::Remote(stream) = role {
        let request = match &initial_project {
            Some(path) => ipc::InstanceRequest::Open {
                path: path.clone(),
                activation_token: launch_token,
            },
            // A bare second launch asks for the Launcher rather than a raise:
            // the Launcher is *how* a further project gets opened, so raising an
            // existing project window would leave a desktop-icon user with no
            // route to one. See `InstanceRequest::ShowLauncher`.
            None => ipc::InstanceRequest::ShowLauncher {
                activation_token: launch_token,
            },
        };
        if handoff(stream, &request) {
            return Bootstrap::Exit;
        }
        // Not acknowledged — a primary that accepted the connection and then
        // wedged, or died mid-handshake. Fall through and launch normally: a
        // duplicate window is a far better outcome than a launch that silently
        // did nothing. This instance does NOT claim the primary socket (the
        // election already resolved), so it behaves as a standalone peer.
    }

    Bootstrap::Continue {
        initial_project,
        is_primary,
        translation_dev,
    }
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

/// Build the app in a design language other than the default IntUI preset.
///
/// Takes a value the same two ways `--config` does (`--style=fluent` or
/// `--style fluent`); the legal names are [`crate::style::names`]. Deliberately
/// not a settings key — see [`crate::style`] for why a design language is a
/// launch decision and light/dark is not.
pub const STYLE_FLAG: &str = "--style";

/// Watch a locale's `.ftl` directory and hot-reload it on every save (debug
/// builds only). Repeatable, once per locale.
///
/// The value is `<locale>=<path>`, and the path is the locale's **directory**,
/// not one file inside it. See [`cli::resolve_translation_dev`] for why a file
/// is refused outright here.
pub const TRANSLATION_DEV_FLAG: &str = "--translation-dev";

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
    /// `--style <name>`: the design language to build this run in. `None` →
    /// [`crate::style::AppStyle::IntUi`], the default. Validated here rather
    /// than at the use site, so an unknown name is a startup error naming the
    /// legal set instead of a silent fall back to the default.
    pub style: Option<crate::style::AppStyle>,
    /// `--translation-dev <locale>=<path>`, repeatable: the `.ftl` directories
    /// to watch and hot-reload for the duration of this run. Shape and locale
    /// are validated here, so an unsupported tag is a startup error naming the
    /// legal set, rather than a watcher that quietly observes nothing.
    pub translation_dev: Vec<TranslationOverride>,
    /// A malformed argument. Reported by `main`, which exits rather than launching
    /// — a probe that asked to pin settings and silently got none is worse than one
    /// that does not start, since it goes on to assert against the wrong state.
    pub error: Option<String>,
}

/// One `--translation-dev <locale>=<path>` pairing, already checked for shape
/// and for a locale this build actually ships strings for.
///
/// Still strings: the path is not touched by [`parse_args`], which is pure so
/// the whole argument surface stays unit-testable without a filesystem. Turning
/// these into a watchable `(LanguageIdentifier, PathBuf)`, and rejecting a path
/// that is missing or is a single file, is [`cli::resolve_translation_dev`]'s
/// job.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TranslationOverride {
    /// A tag from [`crate::startup::SUPPORTED_LOCALES`].
    pub locale: String,
    /// The directory of `.ftl` files to watch, exactly as typed.
    pub path: String,
}

/// The `--translation-dev` rejections, in one place so the attached and
/// separated forms cannot drift apart.
fn bad_translation_dev(value: &str) -> String {
    match value.split_once('=') {
        None => format!(
            "{TRANSLATION_DEV_FLAG}: `{value}` is missing the locale (expected {TRANSLATION_DEV_FLAG} <locale>=<path>)"
        ),
        Some((locale, path)) if locale.trim().is_empty() || path.trim().is_empty() => format!(
            "{TRANSLATION_DEV_FLAG}: `{value}` needs both a locale and a path (expected {TRANSLATION_DEV_FLAG} <locale>=<path>)"
        ),
        Some((locale, _)) => format!(
            "{TRANSLATION_DEV_FLAG}: unknown locale `{locale}` (this build ships: {})",
            crate::startup::SUPPORTED_LOCALES.join(", ")
        ),
    }
}

/// Parse one `<locale>=<path>` value, or describe why it cannot be.
///
/// The locale is checked against [`crate::startup::SUPPORTED_LOCALES`] rather
/// than merely parsed as a language tag: `set_locale` silently no-ops on an
/// unsupported target, so `--translation-dev de-DE=...` would otherwise start a
/// watcher on a bundle nothing can ever display and report nothing wrong.
fn parse_translation_dev(value: &str) -> Result<TranslationOverride, String> {
    let (locale, path) = value
        .split_once('=')
        .ok_or_else(|| bad_translation_dev(value))?;
    let (locale, path) = (locale.trim(), path.trim());
    if locale.is_empty() || path.is_empty() {
        return Err(bad_translation_dev(value));
    }
    if !crate::startup::SUPPORTED_LOCALES.contains(&locale) {
        return Err(bad_translation_dev(value));
    }
    Ok(TranslationOverride {
        locale: locale.to_string(),
        path: path.to_string(),
    })
}

/// The `--style` rejection, in one place so the attached and separated forms
/// cannot drift apart.
fn unknown_style(value: &str) -> String {
    format!(
        "{STYLE_FLAG}: unknown style `{value}` (known: {})",
        crate::style::names().join(", ")
    )
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
        } else if let Some(value) = arg.strip_prefix("--style=") {
            match crate::style::AppStyle::from_name(value) {
                Some(style) => out.style = Some(style),
                None => out.error = Some(unknown_style(value)),
            }
        } else if arg == STYLE_FLAG {
            // Peek, never take — same reasoning as `--config` below: consuming a
            // value that turned out to be the next flag disables that flag too.
            let is_value = rest
                .peek()
                .is_some_and(|v| !v.as_ref().trim().is_empty() && !v.as_ref().starts_with("--"));
            if is_value {
                let value = rest.next().expect("just peeked");
                match crate::style::AppStyle::from_name(value.as_ref()) {
                    Some(style) => out.style = Some(style),
                    None => out.error = Some(unknown_style(value.as_ref())),
                }
            } else {
                out.error = Some(format!(
                    "{STYLE_FLAG} needs a name ({})",
                    crate::style::names().join(", ")
                ));
            }
        } else if let Some(value) = arg.strip_prefix("--translation-dev=") {
            match parse_translation_dev(value) {
                Ok(entry) => out.translation_dev.push(entry),
                Err(e) => out.error = Some(e),
            }
        } else if arg == TRANSLATION_DEV_FLAG {
            // Peek, never take: same reasoning as `--config` below.
            let is_value = rest
                .peek()
                .is_some_and(|v| !v.as_ref().trim().is_empty() && !v.as_ref().starts_with("--"));
            if is_value {
                let value = rest.next().expect("just peeked");
                match parse_translation_dev(value.as_ref()) {
                    Ok(entry) => out.translation_dev.push(entry),
                    Err(e) => out.error = Some(e),
                }
            } else {
                out.error = Some(format!(
                    "{TRANSLATION_DEV_FLAG} needs <locale>=<path> (locales: {})",
                    crate::startup::SUPPORTED_LOCALES.join(", ")
                ));
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

    /// Both spellings, same as `--config`.
    #[test]
    fn style_takes_its_value_attached_or_separated() {
        use crate::style::AppStyle;
        assert_eq!(parse_args(["--style=fluent"]).style, Some(AppStyle::Fluent));
        assert_eq!(
            parse_args(["--style", "fluent"]).style,
            Some(AppStyle::Fluent)
        );
        assert_eq!(
            parse_args(["--style", "m3"]).style,
            Some(AppStyle::Material3)
        );
    }

    /// The same trap `--config` fell into: the value must not become the project.
    #[test]
    fn a_style_value_is_not_mistaken_for_the_project() {
        let args = parse_args(["--style", "macos", "/tmp/a.skrib"]);
        assert_eq!(args.style, Some(crate::style::AppStyle::MacOs));
        assert_eq!(args.project.as_deref(), Some("/tmp/a.skrib"));
        assert_eq!(args.error, None);
    }

    /// An unknown style is a hard error naming the legal set — never a silent
    /// fall back to the default, which would launch the app in the style the
    /// operator was trying to leave and report nothing.
    #[test]
    fn an_unknown_style_is_an_error_that_names_the_legal_set() {
        for args in [
            parse_args(["--style", "fluid"]),
            parse_args(["--style=fluid"]),
        ] {
            let error = args.error.expect("an unknown style must be reported");
            assert!(
                error.contains("fluid"),
                "{error} does not name the offender"
            );
            for known in crate::style::names() {
                assert!(error.contains(known), "{error} does not list `{known}`");
            }
            assert_eq!(args.style, None);
        }
    }

    /// A forgotten name must not swallow the next flag — same reasoning as
    /// `--config`, and the error still has to list what was allowed.
    #[test]
    fn style_without_a_value_is_an_error_not_a_swallowed_flag() {
        let args = parse_args(["--style", "--new-instance"]);
        assert!(args.error.is_some(), "a missing name must be reported");
        assert_eq!(args.style, None);
        assert!(
            args.new_instance,
            "and the flag it would have swallowed still applies"
        );

        assert!(parse_args(["--style"]).error.is_some());
        assert!(parse_args(["--style="]).error.is_some());
    }

    /// The default is no style at all, which `run` reads as IntUI — the app's
    /// behaviour before the flag existed.
    #[test]
    fn no_style_flag_leaves_the_default() {
        assert_eq!(parse_args(["/tmp/a.skrib"]).style, None);
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
            "--style",
            "fluent",
        ]);
        assert_eq!(args.project.as_deref(), Some("/tmp/a.skrib"));
        assert!(args.new_instance);
        assert_eq!(args.config.as_deref(), Some("/tmp/pins.toml"));
        assert!(args.dump_config);
        assert_eq!(args.style, Some(crate::style::AppStyle::Fluent));
        assert_eq!(args.error, None);
    }

    #[test]
    fn translation_dev_takes_its_value_attached_or_separated() {
        for args in [
            parse_args(["--translation-dev=fr-FR=/tmp/fr"]),
            parse_args(["--translation-dev", "fr-FR=/tmp/fr"]),
        ] {
            assert_eq!(args.error, None);
            assert_eq!(
                args.translation_dev,
                vec![TranslationOverride {
                    locale: "fr-FR".into(),
                    path: "/tmp/fr".into(),
                }]
            );
        }
    }

    /// Repeatable: one flag per locale, all of them kept.
    #[test]
    fn translation_dev_accumulates_across_locales() {
        let args = parse_args([
            "--translation-dev",
            "fr-FR=/tmp/fr",
            "--translation-dev",
            "en-US=/tmp/en",
        ]);
        assert_eq!(args.error, None);
        let seen: Vec<&str> = args
            .translation_dev
            .iter()
            .map(|o| o.locale.as_str())
            .collect();
        assert_eq!(seen, ["fr-FR", "en-US"]);
    }

    /// A path holding an `=` (perfectly legal on Unix) splits at the *first*
    /// one, so the locale is the part before it and the whole remainder is the
    /// path.
    #[test]
    fn translation_dev_splits_at_the_first_equals_only() {
        let args = parse_args(["--translation-dev=fr-FR=/tmp/a=b/fr-FR"]);
        assert_eq!(args.error, None);
        assert_eq!(
            args.translation_dev.first().map(|o| o.path.as_str()),
            Some("/tmp/a=b/fr-FR")
        );
    }

    /// An unsupported tag is refused rather than watched: `set_locale` no-ops
    /// on one, so the watcher would feed a bundle nothing can ever display.
    #[test]
    fn an_unsupported_translation_locale_is_an_error_naming_the_shipped_set() {
        let args = parse_args(["--translation-dev=de-DE=/tmp/de"]);
        assert!(args.translation_dev.is_empty());
        let error = args.error.expect("an unsupported locale must be refused");
        assert!(error.contains("de-DE"), "{error}");
        for locale in crate::startup::SUPPORTED_LOCALES {
            assert!(error.contains(locale), "{error} should name {locale}");
        }
    }

    /// Both halves are required, and neither may be blank.
    #[test]
    fn a_malformed_translation_dev_value_is_an_error() {
        for value in ["fr-FR", "=/tmp/fr", "fr-FR=", "fr-FR=   "] {
            let args = parse_args([format!("--translation-dev={value}")]);
            assert!(
                args.error.is_some(),
                "`{value}` should not parse as an override"
            );
            assert!(args.translation_dev.is_empty());
        }
    }

    /// The peek-never-take rule: a missing value must not swallow the flag
    /// that follows it.
    #[test]
    fn translation_dev_without_a_value_is_an_error_not_a_swallowed_flag() {
        let args = parse_args(["--translation-dev", "--new-instance"]);
        assert!(args.error.is_some());
        assert!(
            args.new_instance,
            "the following flag must still have been parsed"
        );
        assert!(args.translation_dev.is_empty());
    }

    /// Its value is a flag value, not the project path.
    #[test]
    fn a_translation_dev_value_is_not_mistaken_for_the_project() {
        let args = parse_args(["--translation-dev", "fr-FR=/tmp/fr", "/tmp/a.skrib"]);
        assert_eq!(args.error, None);
        assert_eq!(args.project.as_deref(), Some("/tmp/a.skrib"));
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
