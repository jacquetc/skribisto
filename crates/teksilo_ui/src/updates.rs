// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Telling a writer, quietly, that a newer version of the application exists.
//!
//! # The shape, and what it deliberately is not
//!
//! One line, in the places the application already prints its own version: the
//! Launcher's sidebar and the About box. Nothing floats, nothing blocks, nothing
//! appears over a manuscript, and there is no badge to clear.
//!
//! That follows from what the fact *is*. "A newer version exists" is a standing
//! condition, not an event, and it stops being true the moment the reader
//! updates. Rendering it from state means it is always accurate, needs no
//! read/unread bookkeeping, and disappears on its own. See
//! [`update_vm`]'s module docs for the three concrete ways the
//! notification archive gets this wrong, none of which are bugs in the archive.
//!
//! Three surfaces were considered and rejected:
//!
//! - **The status bar, beside the save glyph.** That corner reports the state of
//!   *this document*: whether the words just typed are on disk. The application's
//!   own version is a different subject, and putting it there would also put app
//!   chrome on the writing surface, which is the one place this feature must
//!   never reach.
//! - **A toast.** `Toast::loading` is persistent by construction, `.broadcast()`
//!   delivers to every open window, and a completion whose window has closed is
//!   dropped silently by teksilo. Together those can strand a spinner reading
//!   "Checking…" over a freshly opened manuscript, and archive it to
//!   `notifications.toml` where it survives the restart. A short toast is used
//!   for the *hand-driven* check, where the reader is watching and asked for it.
//! - **A banner across the Launcher.** Reserved, by both the GNOME guidelines and
//!   ordinary taste, for information that is ongoing and important. A routine
//!   release is neither, and a banner in the accent colour reads as a problem.
//!
//! # Where the answer comes from
//!
//! A small static JSON file on the project's own site, not the GitHub API. See
//! [`feed`] for the four reasons, three of them measured against this repository.
//!
//! # When it speaks, and when it does not
//!
//! [`channel`] holds the policy. Two channels are silent because somebody else
//! is already responsible for keeping that copy current, and a build that does
//! not know what it is stays silent too. The failure direction is always silence.
//!
//! # The reader's control over it
//!
//! On by default, on the channels that check at all, and turned off in one click
//! in Settings under Notifications. Turning it off clears what was already found
//! rather than merely hiding it, so it cannot come back without a new check.
//!
//! The request carries no identifier and not even the running version: the
//! comparison happens on the reader's machine. That is what lets the published
//! privacy page describe it in one honest paragraph.

pub mod channel;
pub mod compare;
pub mod feed;
pub mod update_line;
pub mod update_vm;

pub use channel::{Channel, ManagedBy};

/// Whether this build offers any update surface at all.
///
/// One name for a condition three unrelated places have to agree on: the Help
/// menu row, the action that row fires, and the Settings toggle. A row whose
/// action was never registered is a dead menu entry, and a toggle for a check
/// that never runs is a lie, so the three must not be able to drift.
pub fn shows_update_state() -> bool {
    Channel::current().shows_update_state()
}
pub use compare::Verdict;
pub use update_line::UpdateLine;
pub use update_vm::{Available, CheckOutcome, UpdateViewModel, view_model};
