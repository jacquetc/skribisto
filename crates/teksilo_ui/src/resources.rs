// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Resources: the files and web links a writer attaches to binder items. Research, not
//! book content: a map of the village, a photograph of the lighthouse, the recording of
//! an interview, a page on the web.
//!
//! [`ResourcesSession`] is what one open project holds (Tier 2), [`ResourcesViewModel`]
//! the per-window logic, [`dock`] its surface on the trailing rail, [`folder_pane`] the
//! Resources page of every folder (over [`FolderResourcesViewModel`]), [`unattached`] the
//! logic of the Work's Resources settings page, and [`text_drop`] the drop of a resource
//! onto the text. The dock and the folder page show a resource with the same row
//! ([`row`]). The rest is plain support code, each module testable without a window:
//! what a file is ([`file_kind`]), which files open without a question
//! ([`open_safety`]), the two sizes that ask ([`sizes`]), links to a resource in the text
//! ([`links`]), the files handed to other applications ([`opened_files`]), and the copies
//! the settings page exports ([`export`]).

pub mod dock;
pub mod edit_panel;
pub mod export;
pub mod file_kind;
pub mod folder_pane;
mod folder_vm;
pub mod links;
pub mod open_safety;
pub mod opened_files;
mod resources_vm;
pub(crate) use resources_vm::name_to_judge;
pub mod row;
pub mod session;
pub mod sizes;
pub mod text_drop;
pub mod unattached;

use std::cell::RefCell;
use std::rc::Rc;

pub use folder_vm::{FolderResource, FolderResourcesViewModel, FolderView, collect_in_subtree};
pub use resources_vm::{
    AddTarget, EditorFn, LargeFileChoice, MediaDirFn, ResourcesViewModel, SaveAsFolderFn,
    declined_for, is_web_address, large_file_choice, move_stored_folders, siblings_to_relink,
    sorted_by_title, web_addresses_in, with_declined,
};
pub use session::ResourcesSession;

/// Where a window's [`ResourcesViewModel`] is handed to the tabs that need it. Cheap to
/// clone; clones share the one slot.
#[derive(Clone, Default)]
pub struct ResourcesSlot(Rc<RefCell<Option<ResourcesViewModel>>>);

impl ResourcesSlot {
    /// Put `vm` in the slot, replacing whatever was there.
    pub fn set(&self, vm: ResourcesViewModel) {
        *self.0.borrow_mut() = Some(vm);
    }

    /// The view-model in the slot, if one was handed over.
    pub fn get(&self) -> Option<ResourcesViewModel> {
        self.0.borrow().clone()
    }

    /// The view-model in the slot, or the one `make` builds, kept there.
    pub fn get_or_insert_with(
        &self,
        make: impl FnOnce() -> ResourcesViewModel,
    ) -> ResourcesViewModel {
        if let Some(vm) = self.get() {
            return vm;
        }
        let vm = make();
        self.set(vm.clone());
        vm
    }
}

impl std::fmt::Debug for ResourcesSlot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ResourcesSlot")
            .field("set", &self.0.borrow().is_some())
            .finish()
    }
}
