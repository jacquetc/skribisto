//! Per-instance IPC: a tiny local socket other instances use to ask this one to
//! raise its window (the ProjectSwitcher "switch to an already-open project").
//!
//! Each process binds a socket named by its pid ([`open_registry::my_ipc_socket`])
//! in `on_ready`, on a background thread that blocks on accept (zero idle CPU).
//! An incoming one-line JSON [`RaiseMainWindow`] is posted onto the UI thread via
//! [`AppEventProxy::send_external`]; `main.rs`'s `on_app_event` handler then
//! focuses the captured main-window `WindowState` (optionally with the forwarded
//! xdg-activation token, for a real cross-surface raise on Wayland).

use std::io::{BufRead, BufReader, Write};

use bastyde::app::AppEventProxy;
use interprocess::local_socket::prelude::*;
use interprocess::local_socket::{GenericFilePath, ListenerOptions, Stream};
use serde::{Deserialize, Serialize};

use crate::open_registry;

/// Wire message + app-side "please raise" event (same shape).
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct RaiseMainWindow {
    /// Optional xdg-activation token (Wayland) minted by the focused requester.
    #[serde(default)]
    pub activation_token: Option<String>,
}

/// Bind this instance's socket and serve raise requests until the process exits.
/// Call once from `BastydeAppBuilder::on_ready`. Failures are best-effort no-ops
/// (window switching simply won't reach this instance).
pub fn spawn_listener(proxy: AppEventProxy) {
    let Some(sock) = open_registry::my_ipc_socket() else {
        return;
    };
    let _ = std::thread::Builder::new()
        .name("skribisto-ipc".into())
        .spawn(move || {
            // Remove a stale socket left by a prior run that reused our pid.
            #[cfg(unix)]
            let _ = std::fs::remove_file(&sock);
            let Ok(name) = sock.to_fs_name::<GenericFilePath>() else {
                return;
            };
            let Ok(listener) = ListenerOptions::new().name(name).create_sync() else {
                return;
            };
            for conn in listener.incoming().flatten() {
                let mut line = String::new();
                if BufReader::new(conn).read_line(&mut line).is_ok() && !line.trim().is_empty() {
                    let msg =
                        serde_json::from_str::<RaiseMainWindow>(line.trim()).unwrap_or_default();
                    proxy.send_external(msg);
                }
            }
        });
}

/// Ask the instance owning `pid` to raise its window, forwarding an optional
/// activation `token`. Best-effort — a dead / unreachable peer just errors.
pub fn send_raise(pid: u32, token: Option<String>) -> std::io::Result<()> {
    let sock = open_registry::ipc_socket_for_pid(pid)
        .ok_or_else(|| std::io::Error::other("no runtime dir"))?;
    let name = sock.to_fs_name::<GenericFilePath>()?;
    let mut conn = Stream::connect(name)?;
    let mut line = serde_json::to_string(&RaiseMainWindow {
        activation_token: token,
    })
    .unwrap_or_default();
    line.push('\n');
    conn.write_all(line.as_bytes())?;
    conn.flush()
}
