// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! A read-only document showing one rendered comparison.
//!
//! Shared by the two places this app puts a diff in front of a writer: the Versions dock,
//! comparing a row against a backup of itself, and the import wizard's reconcile step,
//! comparing what the project holds against what a returning file brings. Both ask the same
//! question of the same renderer ([`crate::view_models::version_diff`]) and differ only in
//! where the two sides come from, so a second copy of this would be a second set of decisions
//! about scroll position, reload and failure — and they would drift.

use std::cell::{Cell, RefCell};
use std::rc::Rc;

use teksilo::text_document::TextDocument;

use crate::view_models::version_diff;

/// Where the comparison is shown. Owned by the panel so it survives a rebuild —
/// a fresh document each time would reset the scroll position on every keystroke
/// elsewhere in the app.
///
/// `set_djot_sync` is the right call *here* and the wrong one for a restore: it
/// clears undo history, which costs nothing on a throwaway view document with no
/// undo stack and no comment anchors, and would be destructive on a real one.
pub(crate) struct DiffPane {
    pub(crate) doc: TextDocument,
    /// The Djot currently loaded, so an unchanged rendering is not reloaded.
    pub(crate) loaded: RefCell<String>,
    /// Character offsets of each change in `doc`, for jump-to-next-change.
    pub(crate) offsets: Rc<RefCell<Vec<usize>>>,
    /// Which change the next jump goes to.
    pub(crate) cursor: Rc<Cell<usize>>,
    pub(crate) handle: RefCell<Option<teksilo::widgets::rich_text::EditorHandle>>,
}

impl DiffPane {
    pub(crate) fn new() -> Self {
        Self {
            doc: TextDocument::new(),
            loaded: RefCell::new(String::new()),
            offsets: Rc::new(RefCell::new(Vec::new())),
            cursor: Rc::new(Cell::new(0)),
            handle: RefCell::new(None),
        }
    }

    /// Load a rendering, if it is not the one already there.
    pub(crate) fn show(&self, rendered: &version_diff::Rendered) {
        if *self.loaded.borrow() == rendered.djot {
            return;
        }
        // A failed import leaves the previous text in place, which would be a lie.
        // Clearing is the honest fallback, and the summary line above still says
        // what changed.
        if self.doc.set_djot_sync(&rendered.djot).is_err() {
            let _ = self.doc.set_djot_sync("");
        }
        *self.loaded.borrow_mut() = rendered.djot.clone();
        *self.offsets.borrow_mut() = rendered.change_offsets.clone();
        self.cursor.set(0);
    }

    pub(crate) fn clear(&self) {
        if self.loaded.borrow().is_empty() {
            return;
        }
        let _ = self.doc.set_djot_sync("");
        self.loaded.borrow_mut().clear();
        self.offsets.borrow_mut().clear();
        self.cursor.set(0);
    }
}
