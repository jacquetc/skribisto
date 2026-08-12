// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Answering what another instance asked this one to do.
//!
//! `shell::ipc` carries an [`ipc::InstanceRequest`] over the socket; this is the
//! primary's side of it — raise a project's window, open one, show the launcher.
//! Kept out of `shell` because every variant ends in
//! [`windows::open_or_focus_project`] or the launcher, which is app-level
//! composition rather than transport.
//!
//! Every variant carries an activation token: on Wayland a process cannot raise
//! itself unprompted, so the token has to come from whoever was clicked.

use std::rc::Rc;

use frontend::AppContext;

use crate::shell::{ipc, windows};

/// Serve one [`ipc::InstanceRequest`] — the primary's whole answer to a remote
/// launch, and to a peer's raise.
///
/// Every arm is the multi-window "document window" pattern: a window's **string
/// id** is its identity, so `find_window(window_id_for(path))` answers "is this
/// project already open here?" exactly, and `open_window` is reached only when it
/// is not. Nothing keeps a side table of paths to windows — the id *is* the table,
/// and it is the same id the window's persisted geometry is keyed by.
///
/// The activation token is applied to the resolved window before focusing it. On
/// Wayland a process cannot raise itself unprompted; the token the *requester*
/// minted (or the desktop handed its launch) is the compositor's evidence that
/// this raise was asked for. Skipping it leaves the window behind the current one
/// on KWin — the raise silently doing nothing.
pub(crate) fn serve_instance_request(
    app_ctx: &Rc<AppContext>,
    request: &ipc::InstanceRequest,
    ctx: &mut teksilo::prelude::EventContext,
) {
    match request {
        ipc::InstanceRequest::Open {
            path,
            activation_token,
        } => {
            if let Some(id) = windows::open_or_focus_project(ctx, path) {
                focus_with_token(ctx, id, activation_token.clone());
            }
        }
        ipc::InstanceRequest::Raise {
            path,
            activation_token,
        } => {
            // A raise never opens anything: it is "come forward", not "open".
            // With no path (a bare "raise this app"), the focused/primary window
            // the context was minted from is already the right answer, so there
            // is nothing to resolve.
            if let Some(path) = path
                && let Some(id) = windows::resolve_project_window(ctx, path)
            {
                focus_with_token(ctx, id, activation_token.clone());
            }
        }
        ipc::InstanceRequest::ShowLauncher { activation_token } => {
            let id = match ctx.find_window(windows::LAUNCHER_WINDOW_ID) {
                Some(id) => id,
                None => ctx.open_window(windows::launcher_window_config(app_ctx.clone())),
            };
            focus_with_token(ctx, id, activation_token.clone());
        }
    }
}

/// Raise `id`, handing the compositor the activation token that authorises it.
pub(crate) fn focus_with_token(
    ctx: &mut teksilo::prelude::EventContext,
    id: teksilo::prelude::TeksiloWindowId,
    token: Option<String>,
) {
    if let Some(token) = token
        && let Some(state) = ctx.window_state(id)
    {
        state.set_activation_token(token);
    }
    ctx.focus_window(id);
}
