// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The instance-to-instance protocol, and the listener that serves it.
//!
//! Two sockets carry the same [`InstanceRequest`] enum:
//!
//! * **the per-pid socket** ([`SocketId::Pid`]) — bound by every instance. It is
//!   how a *peer* process is reached: the ProjectSwitcher asking a
//!   `--new-instance` sibling to raise its window.
//! * **the primary socket** ([`SocketId::Primary`]) — bound only by the instance
//!   that won the election. It is how a **remote** launch hands its command line
//!   over instead of becoming a second process (see [`crate::shell::instance`]).
//!
//! Both are addressed through [`open_registry::socket_name`], which is a
//! filesystem path on Unix and a `\\.\pipe\` name on Windows — those are not
//! interchangeable, and assuming they were is why single-instance never engaged
//! on Windows in the first revision.
//!
//! Both are served on one background thread that blocks on accept (zero idle
//! CPU). An incoming one-line JSON request is posted onto the UI thread via
//! [`AppEventProxy::send_external`], where `main.rs`'s
//! `on_external_with_ctx` handler opens, finds or focuses the window it names —
//! that hook exists because opening a window needs an `EventContext`, which the
//! plain `on_app_event` hook does not have.
//!
//! The reply goes back on the same connection so the remote knows whether it may
//! exit; a remote that is not acknowledged relaunches standalone rather than
//! disappearing having done nothing.
//!
//! Socket files are unlinked on a clean exit by [`cleanup_own_sockets`] (called
//! alongside `open_registry::release_all()`); a crash instead leaves the per-pid
//! one for `open_registry::scan()` to reap once the pid is dead, and the primary
//! one for the next launch's election to unlink (it can prove nothing is
//! listening before it does).

use std::io::{BufRead, BufReader, Write};

use interprocess::local_socket::prelude::*;
use interprocess::local_socket::{ListenerOptions, Stream};
use serde::{Deserialize, Serialize};
use teksilo::app::AppEventProxy;

use crate::shell::open_registry::{self, SocketId};

/// What one instance asks another to do.
///
/// Every variant carries an optional `xdg_activation_v1` token: on Wayland a
/// process cannot raise itself unprompted, so the *requesting* side (which has
/// the focus, or was handed a startup token by the desktop) mints one and the
/// answering side applies it before focusing. Without it the compositor treats
/// the raise as an unsolicited pop-up and, on KWin, leaves the window behind.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum InstanceRequest {
    /// Bring a window forward without changing what is open.
    ///
    /// `path` names *which* window — the project whose window should come
    /// forward. It is `Option` because a bare "raise this app" (no particular
    /// project in mind) is still meaningful, and resolves to the focused or
    /// primary window.
    ///
    /// **`path` is not decoration.** Before it existed, the handler focused a
    /// single `main_window_state` slot that was retargeted to whichever project
    /// window opened most recently — so with two projects open, "raise
    /// Starforgers" would raise the other one.
    Raise {
        #[serde(default)]
        path: Option<String>,
        #[serde(default)]
        activation_token: Option<String>,
    },
    /// Open `path`, or focus its window if it is already open. The remote
    /// launch's whole reason for existing.
    Open {
        path: String,
        #[serde(default)]
        activation_token: Option<String>,
    },
    /// Show the Launcher — what a bare second `skribisto` (no argv) asks for.
    ///
    /// Deliberately *not* "raise whatever is open": the Launcher is how a
    /// second project gets opened, so a desktop-icon user who already has a
    /// project up would otherwise have no way to reach one.
    ShowLauncher {
        #[serde(default)]
        activation_token: Option<String>,
    },
}

/// The answer a remote waits for before it exits.
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum InstanceReply {
    /// The request was handled (or will be, on the next event-loop turn).
    Accepted,
    /// The request could not be served; the remote should launch normally.
    Rejected,
}

/// How long `serve_one` gives this instance's UI thread to answer a request
/// before telling the caller to launch on its own.
///
/// Strictly less than `instance::ACK_TIMEOUT`, the remote's
/// matching patience — see that constant for what goes wrong when the inequality
/// is reversed. `the_primary_gives_up_before_the_remote_does` pins the ordering.
pub const UI_ACK_BUDGET: std::time::Duration = std::time::Duration::from_secs(3);

/// A request that arrived over a socket, plus the channel its acknowledgement
/// goes back on.
///
/// Posted to the UI thread as an `AppEvent::External` payload. The `ack` is a
/// one-shot: the handler sends [`InstanceReply::Accepted`] once it has resolved
/// the window, and dropping it without sending is read by the serving thread as
/// a rejection — so a handler that panics or forgets cannot strand a remote for
/// the full timeout.
pub struct IncomingRequest {
    pub request: InstanceRequest,
    ack: std::sync::mpsc::SyncSender<InstanceReply>,
}

impl IncomingRequest {
    /// Acknowledge the request. Best-effort: a remote that already timed out and
    /// exited leaves a disconnected channel, which is not an error here.
    pub fn accept(&self) {
        let _ = self.ack.send(InstanceReply::Accepted);
    }
}

impl std::fmt::Debug for IncomingRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("IncomingRequest")
            .field("request", &self.request)
            .finish()
    }
}

/// Bind this instance's sockets and serve requests until the process exits.
///
/// Call once from `TeksiloAppBuilder::on_ready`. `is_primary` decides whether the
/// well-known primary socket is served in addition to this instance's own per-pid
/// one — a `--new-instance` or `Standalone` process serves only the latter, so it
/// stays reachable by the switcher without claiming an election it did not win.
///
/// Failures are best-effort no-ops: the worst case is that this instance cannot
/// be reached, which degrades to the pre-Phase-4 multi-process behaviour rather
/// than breaking anything.
pub fn spawn_listener(proxy: AppEventProxy, is_primary: bool) {
    serve(SocketId::Pid(open_registry::my_pid()), proxy.clone(), true);
    if is_primary {
        // `instance::elect` already unlinked any stale file and proved the claim
        // by binding once; it dropped that listener so this is the bind that
        // actually serves. Do not unlink again here — between the election and
        // now, this is *our* socket.
        serve(SocketId::Primary, proxy, false);
    }
}

/// Bind `sock` on a background thread and post every well-formed request.
///
/// `unlink_first` removes a socket left by a prior run that reused our pid — true
/// for the per-pid socket, false for the primary one, whose staleness was already
/// resolved (provably, by a failed connect) during the election.
fn serve(id: SocketId, proxy: AppEventProxy, unlink_first: bool) {
    let _ = std::thread::Builder::new()
        .name("skribisto-ipc".into())
        .spawn(move || {
            // Unix only: a Windows named pipe is not a filesystem object and
            // cannot outlive its server, so there is never a stale one to clear.
            #[cfg(unix)]
            if unlink_first && let Some(sock) = open_registry::socket_path(id) {
                let _ = std::fs::remove_file(&sock);
            }
            #[cfg(not(unix))]
            let _ = unlink_first;
            let Some(name) = open_registry::socket_name(id) else {
                return;
            };
            let Ok(listener) = ListenerOptions::new().name(name).create_sync() else {
                return;
            };
            for conn in listener.incoming().flatten() {
                serve_one(conn, &proxy);
            }
        });
}

/// Read one request off `conn`, hand it to the UI thread, and write back what
/// the UI thread decided.
fn serve_one(mut conn: Stream, proxy: &AppEventProxy) {
    let mut line = String::new();
    if BufReader::new(&mut conn).read_line(&mut line).is_err() || line.trim().is_empty() {
        return;
    }
    let Ok(request) = serde_json::from_str::<InstanceRequest>(line.trim()) else {
        let _ = write_reply(&mut conn, InstanceReply::Rejected);
        return;
    };

    // Bounded to 1: exactly one acknowledgement is ever sent, and a bounded
    // channel means a handler that somehow acked twice cannot grow a queue.
    let (tx, rx) = std::sync::mpsc::sync_channel(1);
    proxy.send_external(IncomingRequest { request, ack: tx });

    // Bounded, and deliberately SHORTER than the remote's own patience
    // (`instance::ACK_TIMEOUT`) — see that constant's doc. If this side outwaited
    // the remote, a slow-but-successful answer would arrive after the remote had
    // already given up and launched standalone: two processes on one project.
    // Falling through with `Rejected` is the safe answer; the remote then launches
    // standalone, which is a duplicate window at worst, never a launch that
    // silently did nothing.
    let reply = rx
        .recv_timeout(UI_ACK_BUDGET)
        .unwrap_or(InstanceReply::Rejected);
    let _ = write_reply(&mut conn, reply);
}

fn write_reply(conn: &mut Stream, reply: InstanceReply) -> std::io::Result<()> {
    let mut line = serde_json::to_string(&reply).unwrap_or_default();
    line.push('\n');
    conn.write_all(line.as_bytes())?;
    conn.flush()
}

/// Remove this instance's socket files, if present. Call once at shutdown,
/// alongside `open_registry::release_all()`, so a clean exit never leaves a
/// socket behind for a later `scan()` — or a later election — to have to reap.
pub fn cleanup_own_sockets(was_primary: bool) {
    // A no-op on Windows, where `socket_path` is `None`: a named pipe disappears
    // with the process that served it, so there is nothing left behind to clean.
    let mut ids = vec![SocketId::Pid(open_registry::my_pid())];
    if was_primary {
        ids.push(SocketId::Primary);
    }
    for id in ids {
        if let Some(sock) = open_registry::socket_path(id) {
            let _ = std::fs::remove_file(sock);
        }
    }
}

/// Ask the instance owning `pid` to raise the window showing `path`, forwarding
/// an optional activation `token`.
///
/// Peer-to-peer, over the per-pid socket — the path used when the ProjectSwitcher
/// finds an entry belonging to a *different* process (a `--new-instance` sibling).
/// Within one process the switcher focuses the window directly instead; going out
/// over a socket to ourselves would work but would route through the same
/// resolution for no reason.
///
/// Best-effort — a dead or unreachable peer just errors.
pub fn send_raise(pid: u32, path: Option<String>, token: Option<String>) -> std::io::Result<()> {
    let name = open_registry::socket_name(SocketId::Pid(pid))
        .ok_or_else(|| std::io::Error::other("no addressable socket for this installation"))?;
    let mut conn = Stream::connect(name)?;
    let mut line = serde_json::to_string(&InstanceRequest::Raise {
        path,
        activation_token: token,
    })
    .unwrap_or_default();
    line.push('\n');
    conn.write_all(line.as_bytes())?;
    conn.flush()?;
    // The peer replies on the same connection (see `serve_one`), but nothing here
    // needs the answer — a raise is advisory. Deliberately *not* read: bounding a
    // blocking read portably needs the scratch-thread dance `instance::handoff`
    // documents (`set_recv_timeout` is `Unsupported` on Windows named pipes), and
    // that is a lot of machinery to discard a reply. Dropping the connection is
    // enough; the peer's write either lands in the buffer or fails harmlessly,
    // and either way it has already queued the raise on its UI thread.
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The wire is JSON over a line-delimited socket, shared by two binaries
    /// that are always the same build — but the shapes still have to survive a
    /// round trip, and `#[serde(default)]` has to keep an absent token from
    /// being a parse error.
    #[test]
    fn every_request_variant_round_trips() {
        let cases = vec![
            InstanceRequest::Raise {
                path: Some("/tmp/a.skrib".into()),
                activation_token: Some("tok".into()),
            },
            InstanceRequest::Raise {
                path: None,
                activation_token: None,
            },
            InstanceRequest::Open {
                path: "/tmp/b.skrib".into(),
                activation_token: None,
            },
            InstanceRequest::ShowLauncher {
                activation_token: Some("tok".into()),
            },
        ];
        for case in cases {
            let json = serde_json::to_string(&case).unwrap();
            let back: InstanceRequest = serde_json::from_str(&json).unwrap();
            assert_eq!(
                format!("{case:?}"),
                format!("{back:?}"),
                "round trip changed the request"
            );
        }
    }

    #[test]
    fn a_request_without_an_activation_token_parses() {
        // The desktop does not always hand out a token (X11, or a bare CLI
        // launch), so the field must be optional on the wire, not merely
        // nullable.
        let r: InstanceRequest = serde_json::from_str(r#"{"Open":{"path":"/tmp/a.skrib"}}"#)
            .expect("an absent token must not be a parse error");
        match r {
            InstanceRequest::Open {
                path,
                activation_token,
            } => {
                assert_eq!(path, "/tmp/a.skrib");
                assert_eq!(activation_token, None);
            }
            other => panic!("wrong variant: {other:?}"),
        }
    }

    #[test]
    fn replies_round_trip() {
        for reply in [InstanceReply::Accepted, InstanceReply::Rejected] {
            let json = serde_json::to_string(&reply).unwrap();
            assert_eq!(serde_json::from_str::<InstanceReply>(&json).unwrap(), reply);
        }
    }

    /// An unacknowledged request must read as a rejection, not as an accept —
    /// a remote told "accepted" by a handler that never ran would exit having
    /// opened nothing at all.
    #[test]
    fn a_dropped_acknowledgement_is_not_an_accept() {
        let (tx, rx) = std::sync::mpsc::sync_channel::<InstanceReply>(1);
        let incoming = IncomingRequest {
            request: InstanceRequest::ShowLauncher {
                activation_token: None,
            },
            ack: tx,
        };
        drop(incoming);
        assert!(
            rx.recv_timeout(std::time::Duration::from_millis(50))
                .is_err(),
            "a dropped request yields no reply, so `serve_one` falls through to Rejected"
        );
    }

    #[test]
    fn accept_delivers_exactly_one_acknowledgement() {
        let (tx, rx) = std::sync::mpsc::sync_channel::<InstanceReply>(1);
        let incoming = IncomingRequest {
            request: InstanceRequest::Open {
                path: "/tmp/a.skrib".into(),
                activation_token: None,
            },
            ack: tx,
        };
        incoming.accept();
        assert_eq!(rx.recv().unwrap(), InstanceReply::Accepted);
    }
}
