// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The app's **scriptable command surface** — every global action and shortcut, grouped by
//! the feature it drives. Pure registration: each block clones the handles it needs out of
//! [`CommandDeps`] and hands a closure to the framework. Nothing here builds a widget.
//! Extracted out of `App::build` so that function is wiring + composition, not a wall of
//! action registrations.
//!
//! ## Everything is `_global`
//!
//! Every registration uses `register_action_global` / `register_shortcut_global`, never the
//! plain forms. Intents walk source-widget → root, so a plain `register_action` only fires
//! when the registering widget is on that path — and the title-bar menu renders in an
//! **overlay**, a sibling of `App`, which never touches it. The global forms are consulted
//! as a dispatch *fallback* regardless of origin (menu overlay, global shortcut, content),
//! which is the only thing that makes the menu work.
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
//! [`view`] (docks + find banner), [`trash`], [`editor`] (spell-check, open item,
//! add-to-dictionary, save), [`export`], [`mod@file`] (project/app lifecycle), [`comments`],
//! [`mod@format`], [`templates`], [`binder`] (outline tree verbs), `go` (prev/next
//! Scene/Chapter/Note, scoped to the focused item's own binder).

use std::rc::Rc;

use bastyde::prelude::*;
use bastyde::widgets::DockWidgetId;

use frontend::AppContext;

use crate::models::OpenDocsStore;
use crate::view_models::{
    BackupSchedulerViewModel, DictionariesViewModel, EditorsViewModel, ExportViewModel,
    FocusViewModel, FullscreenViewModel, OutlineViewModel, ProjectSwitchViewModel,
    SearchReplaceViewModel, TrashViewModel, UserDictionaryViewModel,
};

use super::PendingExit;

mod binder;
mod comments;
mod editor;
mod export;
mod file;
mod footnotes;
mod format;
mod go;
mod images;
mod templates;
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
    /// This window's own id-only Work state — threaded so `app.quit`'s guard
    /// (and any future close-adjacent command) resolves the *right* Work via
    /// `guard_unsaved_exit`, never `ctx.app_state::<AppIds>()` (see
    /// `close_work_and_return_to_launcher`'s doc for why that lookup is unsound
    /// the moment a second Work's window exists).
    pub ids: crate::app_ids::AppIds,
    /// The whole Tier-2 bundle for this window's own Work — threaded so
    /// `app.settings` can build `SettingsPanel` over the RIGHT Work's session
    /// instead of `ctx.app_state`'s stale, first-window-wins slot (see
    /// `SettingsPanel`'s own `session` field doc).
    pub session: crate::sessions::WorkSession,
    /// The app-global Work registry.
    pub registry: crate::sessions::WorkRegistry,
    /// How this window reached its Work — see [`crate::app::WindowRole`].
    /// Paired with `registry`/`ids` by `crate::app::may_switch_project_in_place`,
    /// which the New Work / Open Work doors in [`mod@file`] consult before
    /// replacing this window's project.
    pub role: crate::app::WindowRole,
    /// The app-global quit sequencer — `app.quit`'s whole implementation. Shared
    /// (not per-window) precisely because a quit spans every window: two windows
    /// running their own sequence over the same Works would prompt twice for each.
    pub quit: crate::view_models::QuitSequencer,
    pub outline: OutlineViewModel,
    /// This window's formatting resolver — the only thing that can answer "which editor has
    /// the caret". The template commands gate on its `has_target` and read/write through
    /// the handle it resolves, so they reach every registered editor rather than only the
    /// ones a tab happens to own.
    pub format: crate::view_models::FormatViewModel,
    /// This window's own "was I maximized/floating before I went fullscreen"
    /// memory — minted fresh per window (never a `ctx.app_state` lookup, see
    /// `FullscreenViewModel`'s own doc for why a shared instance would answer
    /// with the wrong window's memory the moment a second project window
    /// exists).
    pub fullscreen: FullscreenViewModel,
    /// This window's own distraction-free state (Increment 2) — minted fresh
    /// per window, never a `ctx.app_state` lookup, for the same reason as
    /// `fullscreen` above (see `FocusViewModel`'s own doc).
    pub focus: FocusViewModel,
    pub editors: EditorsViewModel,
    pub trash: TrashViewModel,
    pub search: SearchReplaceViewModel,
    pub project_switch: ProjectSwitchViewModel,
    pub backup_scheduler: BackupSchedulerViewModel,
    pub dictionaries: DictionariesViewModel,
    pub spell_docs: OpenDocsStore,
    /// Tier-2 (per-open-Work), threaded from `sessions::WorkSession` — never via
    /// `ctx.app_state::<T>()`: that slot is one process-wide value, so with a
    /// second Work open in a second window the lookup would silently resolve
    /// to whichever Work's session registered it first, and "Add to
    /// dictionary" fired from this window would write into *that* Work's
    /// personal dictionary instead of this window's own.
    pub user_dictionary: UserDictionaryViewModel,
    /// Tier-2 (per-open-Work, bound to this window's own `ids`) — threaded
    /// rather than looked up via `ctx.app_state::<ExportViewModel>()`: see
    /// `user_dictionary`'s doc above for why an `app_state` lookup would be
    /// wrong the moment a second Work opens in a second window.
    pub export: ExportViewModel,
    /// Fixed dock ids (see [`crate::docks`]) — the reveal targets.
    pub search_dock: DockWidgetId,
    pub trash_dock: DockWidgetId,
    /// The footnotes dock, for `footnotes.show` — the way back when a saved desk
    /// has lost it. See that command for why a dock needs one at all.
    pub footnotes_dock: DockWidgetId,
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
    comments::register(ctx, deps);
    footnotes::register(ctx, deps);
    format::register(ctx, deps);
    templates::register(ctx, deps);
    binder::register(ctx, deps);
    go::register(ctx, deps);
    images::register(ctx, deps);
}
