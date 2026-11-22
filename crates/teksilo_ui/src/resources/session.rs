// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! What the Resources feature holds per open project, shared by every window on it.
//!
//! Tier 2 (see [`crate::sessions::WorkSession`]): the resources model, the files handed
//! to other applications ([`OpenedFiles`]), and the notices a change picked up while no
//! handler was running still owes the writer. A second window on the same project reads
//! the same rows and picks up the same edits; a second project keeps its own.
//!
//! ## Picking up an edit made in another application
//!
//! [`ResourcesSession::pickup_changes`] runs when a window regains focus, before each
//! save, and before a close or a quit decides whether anything is unsaved. A copy a
//! single-file project opened ([`OpenedWhere::Copy`]) that changed replaces the stored
//! file, and the previous bytes go to the scratch folder first, so the notice's Undo can
//! put them back; a folder project's file edited in place ([`OpenedWhere::InPlace`]) is
//! already the project's, so only its row moves and there is nothing to undo. Either way
//! the row's size, hash and picture size are brought up to date, which is the edit that
//! reaches the next save.
//!
//! Only a change taken in that way is settled ([`OpenedFiles::settle`]). One that fails
//! on the way (the previous bytes could not be kept, the stored file could not be
//! replaced, the row could not be written, the row is not found) stays unseen: it is
//! tried again at the next pickup, and the close keeps its copy rather than deleting the
//! only place the edit is.
//!
//! The copies themselves are made by [`ResourcesSession::open_copy`], under the
//! project's own folder in `<data>/open/` (see [`crate::resources::opened_files`]). An
//! earlier copy found there holding bytes the stored file does not (an edit saved after
//! the last close took its look, or one that could not be taken in) is never
//! overwritten: it is taken in at once, as if the writer had just come back from it,
//! with the same notice and Undo, and then handed over as it is.
//!
//! Undo ([`ResourcesSession::undo_change`]) finds the stored file where it is at that
//! moment, not where it was at the pickup, since a Save As in between moves it. It
//! writes the previous bytes back there and into the copy, and settles the copy on
//! them, so the other application's next save is a new change.
//!
//! Neither caller has an `EventContext` to raise the notice with (a focus change is an
//! effect, a save request a plain call), so the notice is queued ([`ChangeNotice`]) and
//! raised by the window that handles the `Resource` update the pickup itself caused.
//! Its toast is routed to the project's audience, every window on the project, so which
//! window raises it does not change where it shows. A binder drop's attach notice is
//! not routed that way and is one window's own: it waits on that window's
//! [`ResourcesViewModel`](crate::resources::ResourcesViewModel), not here.
//!
//! Not a view-model: per-project state and the file work around it.

use std::cell::RefCell;
use std::path::{Path, PathBuf};
use std::rc::Rc;

use frontend::AppContext;

use crate::app_ids::AppIds;
use crate::models::{FileFacts, ResourceRow, ResourcesListModel};
use crate::resources::opened_files::{self, OpenCopy, OpenedFiles, OpenedWhere};

/// A change picked up, waiting for a window to say so.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ChangeNotice {
    pub row_id: u64,
    /// The name of the file, as the notice says it.
    pub file_name: String,
    /// What Undo puts back; `None` for a folder project's file, which is the project's.
    pub undo: Option<UndoChange>,
}

/// The previous version of a stored file a copy replaced.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UndoChange {
    /// The previous bytes, in the scratch folder.
    pub kept: PathBuf,
    /// The row as it was.
    pub previous: FileFacts,
    /// The version this action is allowed to undo.
    applied_hash: String,
}

struct Inner {
    app_ctx: Rc<AppContext>,
    model: ResourcesListModel,
    opened: OpenedFiles,
    notices: RefCell<Vec<ChangeNotice>>,
}

/// The Resources feature's per-project state. Cheap to clone; clones share it.
#[derive(Clone)]
pub struct ResourcesSession {
    inner: Rc<Inner>,
}

impl std::fmt::Debug for ResourcesSession {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ResourcesSession")
            .field("opened", &self.inner.opened)
            .finish()
    }
}

impl ResourcesSession {
    pub fn new(app_ctx: Rc<AppContext>, ids: AppIds) -> Self {
        Self {
            inner: Rc::new(Inner {
                app_ctx: app_ctx.clone(),
                model: ResourcesListModel::new(app_ctx, ids),
                opened: OpenedFiles::new(),
                notices: RefCell::new(Vec::new()),
            }),
        }
    }

    pub fn model(&self) -> &ResourcesListModel {
        &self.inner.model
    }

    /// The backend this project lives in: what a surface outside any window's
    /// view-model (the Work's Resources settings page) builds its own with.
    pub fn app_ctx(&self) -> Rc<AppContext> {
        self.inner.app_ctx.clone()
    }

    pub fn opened(&self) -> &OpenedFiles {
        &self.inner.opened
    }

    /// Bring back what other applications changed in the files this project handed
    /// them, and say whether that changed the store. See the module docs.
    pub fn pickup_changes(&self) -> bool {
        let mut picked = false;
        for changed in self.inner.opened.changed() {
            // Not settled: tried again at the next pickup, and kept by the close.
            let Some(row) = self.inner.model.by_uid(changed.uid) else {
                continue;
            };
            // The external application may still be editing the original after Save As.
            if changed.place == OpenedWhere::InPlace
                && !row
                    .path
                    .as_deref()
                    .is_some_and(|path| super::resources_vm::same_place(path, &changed.path))
            {
                self.inner.opened.stop_tracking_in_place(&changed.path);
                continue;
            }
            let notice = match changed.place {
                OpenedWhere::Copy => {
                    self.take_copy(&row, &changed.path, changed.len, &changed.hash)
                }
                OpenedWhere::InPlace => {
                    let facts = facts_of(&changed.path, &row, changed.len, &changed.hash);
                    match self.inner.model.set_file_facts(row.id, &facts) {
                        Ok(()) => Some(ChangeNotice {
                            row_id: row.id,
                            file_name: row.file_name.clone(),
                            undo: None,
                        }),
                        Err(e) => {
                            eprintln!("resources: could not update {}: {e}", row.file_name);
                            None
                        }
                    }
                }
            };
            if let Some(notice) = notice {
                // Taken in: only now is the change seen.
                self.inner
                    .opened
                    .settle(&changed.path, changed.stamp, &changed.hash);
                self.inner.notices.borrow_mut().push(notice);
                picked = true;
            }
        }
        picked
    }

    /// The copy `row`'s stored file opens through, made under `data_dir` and the
    /// project's own folder there, and remembered so that an edit in it comes back.
    ///
    /// An earlier copy holding bytes the stored file does not is left as it is and taken
    /// in now, with the notice and its Undo: it is an edit the writer saved that the
    /// project never got (see the module docs).
    pub fn open_copy(&self, row: &ResourceRow, data_dir: &Path) -> std::io::Result<PathBuf> {
        let stored = row
            .path
            .clone()
            .ok_or_else(|| std::io::Error::other("no stored file"))?;
        let dir = self.inner.model.resources_dir();
        let key = dir
            .file_name()
            .and_then(|n| n.to_str())
            .ok_or_else(|| std::io::Error::other("no project to open a copy for"))?;
        let held = if row.content_hash.is_empty() {
            opened_files::hash_file(&stored)?.1
        } else {
            row.content_hash.clone()
        };
        match opened_files::write_open_copy(&stored, data_dir, key, row.uid, &row.file_name)? {
            OpenCopy::Fresh(copy) => {
                self.inner
                    .opened
                    .remember(row.uid, &copy, OpenedWhere::Copy, &held);
                Ok(copy)
            }
            OpenCopy::Pending(copy) => {
                self.inner
                    .opened
                    .remember_unsettled(row.uid, &copy, OpenedWhere::Copy, &held);
                self.pickup_changes();
                Ok(copy)
            }
        }
    }

    /// A single-file project's copy changed: it becomes the stored file.
    fn take_copy(
        &self,
        row: &ResourceRow,
        copy: &Path,
        len: u64,
        hash: &str,
    ) -> Option<ChangeNotice> {
        if !row.is_stored() {
            return None;
        }
        let stored = row.path.clone()?;
        // Keep what is there before replacing it. A stored file already missing has
        // nothing to keep, and the change is still the writer's latest version.
        let kept = if stored.is_file() {
            match self.inner.opened.keep_previous(row.uid, &stored) {
                Ok(kept) => Some(kept),
                Err(e) => {
                    eprintln!("resources: could not keep the previous version: {e}");
                    return None;
                }
            }
        } else {
            None
        };
        if let Err(e) = opened_files::replace_file(copy, &stored) {
            eprintln!("resources: could not take back {}: {e}", copy.display());
            return None;
        }
        let facts = facts_of(&stored, row, len, hash);
        if let Err(e) = self.inner.model.set_file_facts(row.id, &facts) {
            eprintln!("resources: could not update {}: {e}", row.file_name);
            return None;
        }
        Some(ChangeNotice {
            row_id: row.id,
            file_name: row.file_name.clone(),
            undo: kept.map(|kept| UndoChange {
                kept,
                previous: FileFacts::of(row),
                applied_hash: hash.to_owned(),
            }),
        })
    }

    /// The notices waiting to be raised, taken: each is raised once, by one window.
    pub fn take_notices(&self) -> Vec<ChangeNotice> {
        std::mem::take(&mut *self.inner.notices.borrow_mut())
    }

    /// Put back the version a copy replaced: the stored file, the row, and the copy the
    /// other application has open, settled on those bytes (see the module docs).
    pub fn undo_change(&self, row_id: u64, undo: &UndoChange) -> Result<(), String> {
        // Where the stored file is now: a Save As since the pickup moved it.
        let Some((row, stored)) = self
            .inner
            .model
            .by_id(row_id)
            .and_then(|row| row.path.clone().map(|path| (row, path)))
        else {
            // Removed since: its own Undo brings it back as it was when removed.
            eprintln!("resources: the resource to undo the change of is gone");
            return Ok(());
        };
        let copies = self.inner.opened.copy_paths(row.uid);
        // Check every destination before changing any of them. The other application
        // can save again without a focus change or another pickup.
        for path in std::iter::once(&stored).chain(&copies) {
            opened_files::require_version(path, &undo.applied_hash).map_err(|e| e.to_string())?;
        }
        opened_files::replace_version(&undo.kept, &stored, &undo.applied_hash)
            .map_err(|e| e.to_string())?;
        self.inner.model.set_file_facts(row_id, &undo.previous)?;
        self.inner
            .opened
            .rewrite_copies(&copies, &stored, &undo.applied_hash)
            .map_err(|e| e.to_string())?;
        Ok(())
    }
}

/// A changed file's facts: its size and hash as just read, and its picture size when it
/// is one the editor decodes (0 by 0 otherwise, as the store run records it).
fn facts_of(path: &Path, row: &ResourceRow, len: u64, hash: &str) -> FileFacts {
    let (width, height) = if crate::resources::file_kind::is_viewable_picture(&row.file_name)
        && len <= document_ingest::pictures::MAX_PICTURE_BYTES
    {
        std::fs::read(path)
            .ok()
            .and_then(|bytes| document_ingest::pictures::displayable_size(&bytes).ok())
            .map_or((0, 0), |(w, h)| (u64::from(w), u64::from(h)))
    } else {
        (0, 0)
    };
    FileFacts {
        byte_size: len,
        content_hash: hash.to_string(),
        width,
        height,
    }
}
