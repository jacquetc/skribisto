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
//! [`ResourcesSession::pickup_changes`] runs when a window regains focus and before
//! each save. A copy a single-file project opened ([`OpenedWhere::Copy`]) that changed
//! replaces the stored file, and the previous bytes go to the scratch folder first, so
//! the notice's Undo can put them back; a folder project's file edited in place
//! ([`OpenedWhere::InPlace`]) is already the project's, so only its row moves and there
//! is nothing to undo. Either way the row's size, hash and picture size are brought up
//! to date, which is the edit that reaches the next save.
//!
//! Neither caller has an `EventContext` to raise the notice with (a focus change is an
//! effect, a save request a plain call), so the notice is queued ([`ChangeNotice`]) and
//! raised by the window that handles the `Resource` update the pickup itself caused.
//!
//! Not a view-model: per-project state and the file work around it.

use std::cell::RefCell;
use std::path::{Path, PathBuf};
use std::rc::Rc;

use frontend::AppContext;

use crate::app_ids::AppIds;
use crate::models::{FileFacts, ResourceRow, ResourcesListModel};
use crate::resources::opened_files::{self, OpenedFiles, OpenedWhere};

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
    /// The stored file they go back to.
    pub stored: PathBuf,
    /// The row as it was.
    pub previous: FileFacts,
}

/// Resources attached by a drop that had no event context to say so with (a drop on a
/// binder row lands in the tree's data source), waiting for a window to raise it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AttachNotice {
    /// The undo step the attach landed on, for the toast's Undo.
    pub seq: Option<u64>,
    /// The resource's title when one was attached, empty for several.
    pub title: String,
    pub count: usize,
    /// The item they were attached to.
    pub item_title: String,
}

struct Inner {
    app_ctx: Rc<AppContext>,
    model: ResourcesListModel,
    opened: OpenedFiles,
    notices: RefCell<Vec<ChangeNotice>>,
    attach_notices: RefCell<Vec<AttachNotice>>,
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
                attach_notices: RefCell::new(Vec::new()),
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
    /// them. See the module docs.
    pub fn pickup_changes(&self) {
        for changed in self.inner.opened.changed() {
            let Some(row) = self.inner.model.by_uid(changed.uid) else {
                self.inner.opened.forget(changed.uid);
                continue;
            };
            let notice = match changed.place {
                OpenedWhere::Copy => {
                    self.take_copy(&row, &changed.path, changed.len, &changed.hash)
                }
                OpenedWhere::InPlace => {
                    let facts = facts_of(&changed.path, &row, changed.len, &changed.hash);
                    self.inner
                        .model
                        .set_file_facts(row.id, &facts)
                        .ok()
                        .map(|()| ChangeNotice {
                            row_id: row.id,
                            file_name: row.file_name.clone(),
                            undo: None,
                        })
                }
            };
            if let Some(notice) = notice {
                self.inner.notices.borrow_mut().push(notice);
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
        self.inner.model.set_file_facts(row.id, &facts).ok()?;
        Some(ChangeNotice {
            row_id: row.id,
            file_name: row.file_name.clone(),
            undo: kept.map(|kept| UndoChange {
                kept,
                stored,
                previous: FileFacts::of(row),
            }),
        })
    }

    /// Queue a drop's attach notice for the next window event to raise.
    pub fn push_attach_notice(&self, notice: AttachNotice) {
        self.inner.attach_notices.borrow_mut().push(notice);
    }

    /// The attach notices waiting to be raised, taken: each is raised once.
    pub fn take_attach_notices(&self) -> Vec<AttachNotice> {
        std::mem::take(&mut *self.inner.attach_notices.borrow_mut())
    }

    /// The notices waiting to be raised, taken: each is raised once, by one window.
    pub fn take_notices(&self) -> Vec<ChangeNotice> {
        std::mem::take(&mut *self.inner.notices.borrow_mut())
    }

    /// Put back the version a copy replaced, bytes and row both.
    pub fn undo_change(&self, row_id: u64, undo: &UndoChange) -> Result<(), String> {
        opened_files::replace_file(&undo.kept, &undo.stored).map_err(|e| e.to_string())?;
        self.inner.model.set_file_facts(row_id, &undo.previous)
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
