// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The app's **scriptable command surface** — every global action and shortcut, grouped by
//! the feature it drives.
//!
//! These ~520 lines used to sit inline in `App::build`, between the view-model wiring above
//! them and the widget tree below, which is what made that function 2 000 lines long. They
//! are pure registration: each block clones the handles it needs out of [`CommandDeps`] and
//! hands a closure to the framework. Nothing here builds a widget.
//!
//! ## Everything is `_global`
//!
//! Every registration uses `register_action_global` / `register_shortcut_global`, never the
//! plain forms. Intents walk source-widget → root, so a plain `register_action` only fires
//! when the registering widget is on that path — and the title-bar menu renders in an
//! **overlay**, a sibling of `App`, which never touches it. The global forms are consulted
//! as a dispatch *fallback* regardless of origin (menu overlay, global shortcut, content),
//! which is the only thing that makes the menu work. This is the classic
//! always-checked/dead-toggle bug; see the house rules.
//!
//! ## Why the bare function keys
//!
//! A Global shortcut is resolved *before* the focused widget sees the raw key. That is what
//! lets Ctrl+F reach the find banner instead of being eaten by the editor — and equally why
//! F9/F7/F10 are bare function keys rather than Ctrl+letter chords: any Ctrl+letter
//! registered here would shadow one of `RichTextEditor`'s built-in commands (Ctrl+B bold,
//! Ctrl+E centre-align, …).
//!
//! ## Grouping
//!
//! By the feature each command drives, not by where it happens to appear in a menu:
//!
//!   * [`view`] — docks and the find banner (outline, preview band, search, find/replace).
//!   * [`trash`] — the trash dock's own verbs.
//!   * [`editor`] — the editor surface: spell-check switch, open item, add-to-dictionary,
//!     save.
//!   * [`export`] — the two export entry points.
//!   * [`file`] — project and application lifecycle (new/open/import/close/settings/quit).
//!   * [`binder`] — the outline tree's verbs.

use std::rc::Rc;

use bastyde::prelude::*;
use bastyde::widgets::DockWidgetId;

use frontend::AppContext;

use crate::models::OpenDocsStore;
use crate::view_models::{
    BackupSchedulerViewModel, DictionariesViewModel, EditorsViewModel, OutlineViewModel,
    ProjectSwitchViewModel, SearchReplaceViewModel, TrashViewModel,
};

use super::PendingExit;

mod binder;
mod editor;
mod export;
mod file;
mod trash;
mod view;

/// The handles the command closures capture.
///
/// One bundle rather than fifteen parameters: every block clones only the fields it needs,
/// and adding a command that reaches a new view-model is one field here instead of a wider
/// signature on six functions. `App` builds it once per `build()` — the fields are all
/// cheap `Rc`/`Signal` clones of state that outlives any single build.
pub(super) struct CommandDeps {
    pub app_ctx: Rc<AppContext>,
    pub outline: OutlineViewModel,
    pub editors: EditorsViewModel,
    pub trash: TrashViewModel,
    pub search: SearchReplaceViewModel,
    pub project_switch: ProjectSwitchViewModel,
    pub backup_scheduler: BackupSchedulerViewModel,
    pub dictionaries: DictionariesViewModel,
    pub spell_docs: OpenDocsStore,
    /// Fixed dock ids (see [`crate::docks`]) — the reveal targets.
    pub search_dock: DockWidgetId,
    pub trash_dock: DockWidgetId,
    /// Derived: the work has edits not on disk.
    pub unsaved: Signal<bool>,
    /// A backup file is open here — Save is off.
    pub backup_mode: Signal<bool>,
    /// The close/quit deferred behind an in-flight save.
    pub pending_exit: Signal<PendingExit>,
    pub autosave: Signal<bool>,
}

/// Register every global action and shortcut.
///
/// Order is not significant — these are independent registrations keyed by name; the
/// framework resolves an intent against the whole set. Grouped calls, not one flat list, so
/// each feature's commands stay findable.
pub(super) fn register_all(ctx: &mut BuildContext, deps: &CommandDeps) {
    view::register(ctx, deps);
    trash::register(ctx, deps);
    editor::register(ctx, deps);
    export::register(ctx, deps);
    file::register(ctx, deps);
    binder::register(ctx, deps);
}
