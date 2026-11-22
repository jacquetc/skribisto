// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Resources: the files and web links a writer attaches to binder items. Research, not
//! book content: a map of the village, a photograph of the lighthouse, the recording of
//! an interview, a page on the web.
//!
//! [`ResourcesSession`] is what one open project holds (Tier 2), [`ResourcesViewModel`]
//! the per-window logic, [`dock`] its surface on the trailing rail, and [`text_drop`] the
//! drop of a resource from the dock onto the text. The rest is plain support code, each
//! module testable without a window: what a file is ([`file_kind`]), which files open
//! without a question ([`open_safety`]), the two sizes that ask ([`sizes`]), links to a
//! resource in the text ([`links`]), and the files handed to other applications
//! ([`opened_files`]).

pub mod dock;
pub mod edit_panel;
pub mod file_kind;
pub mod links;
pub mod open_safety;
pub mod opened_files;
mod resources_vm;
pub mod session;
pub mod sizes;
pub mod text_drop;

pub use resources_vm::{
    AddTarget, EditorFn, LargeFileChoice, MediaDirFn, ResourcesViewModel, SaveAsFolderFn,
    declined_for, is_web_address, large_file_choice, move_stored_folders, siblings_to_relink,
    sorted_by_title, web_addresses_in, with_declined,
};
pub use session::ResourcesSession;
