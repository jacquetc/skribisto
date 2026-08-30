// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The two undo domains, behind one trait.
//!
//! Not a view-model: these are thin adapters that put a uniform face on two
//! engines that already exist. Neither holds state of its own — a third copy of
//! "can undo" would be a third thing to keep in step.

use std::cell::RefCell;
use std::rc::Rc;

use frontend::AppContext;
use frontend::commands::undo_redo_commands;
use frontend::common::undo_redo::UndoLabel;
use teksilo::prelude::*;

use crate::editors::EditorsViewModel;
use crate::format::FormatViewModel;
use crate::save::SaveStateViewModel;

/// Which history a command belongs to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DomainKind {
    /// The focused document's own prose history, word-coalesced.
    Prose,
    /// A focused plain text field's own history — a rename box, the search
    /// input.
    ///
    /// Its own domain rather than part of `Entity`, and the reason is a hazard
    /// rather than tidiness: registering Ctrl+Z globally means the app
    /// intercepts it *before* the field's own key handling, so without this a
    /// writer renaming a chapter and pressing Ctrl+Z out of habit would not get
    /// their typing back — they would un-trash a folder.
    TextField,
    /// The project's structural history — trash, rename, move, tags, Replace All.
    Entity,
}

/// One history, as the arbiter needs to see it.
pub trait UndoDomain {
    fn can_undo(&self) -> bool;
    fn can_redo(&self) -> bool;
    /// Refusing to step through history at all — as opposed to having nothing
    /// left to step through. The router must not fall through a refusal.
    fn frozen(&self) -> bool;
    fn undo(&self);
    fn redo(&self);
    /// A machine key naming what the next undo would take back, for the
    /// application to translate. `None` leaves the row generic.
    fn undo_label(&self) -> Option<UndoLabel>;
    fn redo_label(&self) -> Option<UndoLabel>;
}

/// The focused document's own history.
///
/// Delegates wholesale to [`FormatViewModel`], which already owns the editor
/// registry, the sticky latch and the `CommandFilter` gate. Duplicating any of
/// that here would give the Format dock and the Edit menu two answers to the
/// same question.
#[derive(Clone)]
pub struct ProseDomain {
    format: FormatViewModel,
}

impl ProseDomain {
    pub(crate) fn new(format: FormatViewModel) -> Self {
        Self { format }
    }

    /// Is a registered editor holding the caret right now?
    pub(crate) fn is_live(&self) -> bool {
        self.format.editor_focused()
    }
}

impl UndoDomain for ProseDomain {
    fn can_undo(&self) -> bool {
        self.format.can_undo().get()
    }

    fn can_redo(&self) -> bool {
        self.format.can_redo().get()
    }

    fn frozen(&self) -> bool {
        self.format.history_frozen()
    }

    fn undo(&self) {
        self.format.undo_editor();
    }

    fn redo(&self) {
        self.format.redo_editor();
    }

    // The document engine records no labels: every entry is the same act
    // ("typing"), so a key per entry would say nothing a fixed row does not.
    // The Edit menu names this domain, not its entries — see `undo_labels`.
    fn undo_label(&self) -> Option<UndoLabel> {
        None
    }

    fn redo_label(&self) -> Option<UndoLabel> {
        None
    }
}

/// The project's structural history — the per-`Work` Qleany stack.
#[derive(Clone)]
pub struct EntityDomain {
    app_ctx: Rc<AppContext>,
    stack: Signal<Option<u64>>,
    save_state: SaveStateViewModel,
    /// Filled in by `App::build`, not at construction.
    ///
    /// The window's menu bar — and therefore this domain — is built before
    /// `EditorsViewModel` exists, the same ordering `FormatViewModel::detached`
    /// and `attach` already work around. An entity undo with no editors
    /// attached simply skips the flush; there is nothing open to flush.
    editors: Rc<RefCell<Option<EditorsViewModel>>>,
}

impl EntityDomain {
    pub(crate) fn new(
        app_ctx: Rc<AppContext>,
        stack: Signal<Option<u64>>,
        save_state: SaveStateViewModel,
    ) -> Self {
        Self {
            app_ctx,
            stack,
            save_state,
            editors: Rc::new(RefCell::new(None)),
        }
    }

    /// Re-point at this window's editors. Called on every `App::build`;
    /// idempotent.
    pub(crate) fn attach_editors(&self, editors: EditorsViewModel) {
        *self.editors.borrow_mut() = Some(editors);
    }

    /// Push every open buffer into the store before touching the history.
    ///
    /// An open editor holds prose the store has not seen, and an undo that
    /// restores `Content` rows underneath it would otherwise be overwritten by
    /// the next autosave of that stale buffer.
    fn flush_editors(&self) {
        if let Some(editors) = self.editors.borrow().as_ref() {
            editors.flush_all();
        }
    }

    fn stack(&self) -> Option<u64> {
        self.stack.get()
    }

    /// Close the focused document's merge chain, so a burst of typing before a
    /// structural command and one after it stay two undo steps.
    ///
    /// Called when a structural command lands. Merging decides on shape alone —
    /// adjacent, close in time — and cannot see that the writer renamed a
    /// chapter in between; without this, one Ctrl+Z in the editor afterwards
    /// takes back text typed on both sides of that line.
    pub(crate) fn seal_prose_merge(&self, format: &FormatViewModel) {
        if let Some(handle) = format.handle_for_commands() {
            handle.break_undo_merge();
        }
    }
}

impl UndoDomain for EntityDomain {
    fn can_undo(&self) -> bool {
        undo_redo_commands::can_undo(&self.app_ctx, self.stack())
    }

    fn can_redo(&self) -> bool {
        undo_redo_commands::can_redo(&self.app_ctx, self.stack())
    }

    fn frozen(&self) -> bool {
        // The writing game freezes *drafting*, not the project: restoring a
        // trashed chapter adds text back, which is forward, not regressive.
        // FEATURES.md's own copy says the game covers "the surfaces you choose",
        // and the binder is not one of them.
        false
    }

    fn undo(&self) {
        self.flush_editors();
        if let Err(e) = undo_redo_commands::undo(&self.app_ctx, self.stack()) {
            eprintln!("edit: undo failed: {e}");
            return;
        }
        self.mark_dirty();
    }

    fn redo(&self) {
        self.flush_editors();
        if let Err(e) = undo_redo_commands::redo(&self.app_ctx, self.stack()) {
            eprintln!("edit: redo failed: {e}");
            return;
        }
        self.mark_dirty();
    }

    fn undo_label(&self) -> Option<UndoLabel> {
        undo_redo_commands::undo_label(&self.app_ctx, self.stack())
    }

    fn redo_label(&self) -> Option<UndoLabel> {
        undo_redo_commands::redo_label(&self.app_ctx, self.stack())
    }
}

impl EntityDomain {
    /// An undo **is** an edit, and Close has to know it.
    ///
    /// `unsaved` is derived from `dirty_seq`, which only `App::mutation_origins`
    /// bumps — and `Content` is deliberately absent from that list, because it
    /// fires on every autosave flush. So an undo that reverts prose bumps
    /// nothing, `unsaved` stays false, and closing the project discards it with
    /// no prompt. That is the gap `mutation_origins`' own comment records
    /// shipping three times (Comment, CommentReply, Footnote); saying it here
    /// keeps it from shipping a fourth.
    fn mark_dirty(&self) {
        self.save_state.bump_dirty();
    }
}

/// A focused text widget's own history — whatever kind of widget it is.
///
/// Built on demand around whichever surface holds the caret, from the
/// framework's own registry, so there is no list here to keep in step with the
/// widgets that exist. That is the difference between "every text surface the
/// application remembered to register" and "every text surface", and it is what
/// makes taking `Ctrl+Z` globally safe.
#[derive(Clone)]
pub struct FieldDomain {
    surface: Rc<dyn teksilo::core::text_surface::TextSurface>,
}

impl FieldDomain {
    pub(crate) fn new(surface: Rc<dyn teksilo::core::text_surface::TextSurface>) -> Self {
        Self { surface }
    }
}

impl UndoDomain for FieldDomain {
    fn can_undo(&self) -> bool {
        self.surface.can_undo()
    }

    fn can_redo(&self) -> bool {
        self.surface.can_redo()
    }

    /// The surface's own answer: a host-imposed mode — a writing game, a
    /// read-only review pass — must not be escapable through a menu.
    fn frozen(&self) -> bool {
        self.surface.history_frozen()
    }

    fn undo(&self) {
        self.surface.undo();
    }

    fn redo(&self) {
        self.surface.redo();
    }

    // Every entry in a text widget's history is the same act — typing — so the
    // row names the domain, not the entry.
    fn undo_label(&self) -> Option<UndoLabel> {
        None
    }

    fn redo_label(&self) -> Option<UndoLabel> {
        None
    }
}
