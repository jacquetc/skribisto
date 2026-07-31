// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Whether a project window **owns** the Work it shows, or only **attaches** to
//! one a sibling already loaded.
//!
//! Work ▸ New Window opens a second desk onto a live Work. The two windows share
//! `AppIds`, the undo stack, open documents and dirty state — but only the window
//! that *loaded* (or created) the Work owns the saved desk layout. An attached
//! window must never inject into / capture / restore that layout, and must never
//! replace the project in place (it shares `AppIds` with the sibling).
//!
//! Encoding those rules as a boolean (`attached`) scattered through `App::build`
//! and the close guard invited drift. [`WindowRole`] is the one type; every policy
//! question is a method on it.

use crate::app_ids::AppIds;
use crate::sessions::WorkRegistry;

use super::PendingAction;

/// How this window reached the Work it shows.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum WindowRole {
    /// Loaded or created the Work — owns desk persistence and may switch in place
    /// when it is alone on the Work.
    Owner,
    /// Work ▸ New Window onto a Work a sibling already shows. Never owns the desk;
    /// never switches in place, even after the sibling closes.
    Attached,
}

impl WindowRole {
    /// Derive the role from the action that will seed this window on first build.
    /// Fixed for the window's whole life — do not re-derive after attach.
    pub fn from_action(action: &PendingAction) -> Self {
        match action {
            PendingAction::AttachExisting { .. } => Self::Attached,
            PendingAction::Load(_) | PendingAction::New(_) => Self::Owner,
        }
    }

    /// This window may inject editors/outline into `WorkSession::workspace_layout`,
    /// restore/capture the desk, and snapshot default docks.
    pub fn owns_desk(self) -> bool {
        matches!(self, Self::Owner)
    }

    /// Install the process-wide `ProjectSwitchViewModel` hooks bound to *this*
    /// window's editors/`AppIds`. Only the owner may; an attached install would
    /// overwrite the owner's hooks with a set nothing should fire.
    pub fn installs_project_switch_hooks(self) -> bool {
        self.owns_desk()
    }

    /// May this window replace its project in place (File ▸ Open / New Work, …)?
    ///
    /// False when attached (durable: even alone on the Work later, desk handles
    /// still point at the original owner) or when a live sibling still shows the
    /// same Work (`window_count_for > 1`).
    pub fn may_switch_in_place(self, registry: &WorkRegistry, ids: &AppIds) -> bool {
        if !self.owns_desk() {
            return false;
        }
        !ids.work_id
            .get()
            .is_some_and(|work_id| registry.window_count_for(work_id) > 1)
    }
}
