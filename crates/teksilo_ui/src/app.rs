// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The application body: a `DockingLayout` whose leading dock is the binder
//! tree and whose center is a `TabWidget` of editor tabs, with a thin status
//! bar underneath. The window chrome (custom `TitleBar` + hamburger menu)
//! lives at the window root in `main.rs`.
//!
//! Clicking a binder item opens (or focuses) its editor tab via the tree's
//! `KeyedSelectionModel<NodeId>` selection signal.
//!
//! Plain builder calls rather than `teksu!`: the docking/tab/editor widgets are
//! generic over closures, which the DSL doesn't express cleanly. See
//! `settings.rs` for the `teksu!` style.

mod commands;
mod project_shell;
mod recreate_row;
mod restore_version;
mod view_model_setup;
mod window_role;
mod wiring;

pub(crate) use recreate_row::{DeletedRow, RecreateContext, recreate_row};
// Re-exported because the New Work wizard, which lives outside this module, arms it:
// its in-place path chooses a tag palette for a project that does not exist yet.
pub(crate) use restore_version::restore_version;
pub(crate) use wiring::project_events::PendingStarters;
// `pub`, not `pub(crate)`: it is a field of `PendingAction::New`, which is `pub`.
pub use wiring::project_events::ProjectStarters;

pub(crate) use window_role::WindowRole;

use std::rc::Rc;

use teksilo::core::DragPayload;
use teksilo::core::modal::{ModalCloseBehavior, ModalPresentation, ModalRequest};
use teksilo::core::widget::WidgetPlacement;
use teksilo::prelude::*;
use teksilo::settings::{Reloadable, SettingsRegistry};
use teksilo::tokens::SurfaceRole::Hover;
use teksilo::widgets::{
    DockWidgetId, EventContextMessageBoxExt, MessageBox, MessageBoxButton, MessageBoxButtons,
    RowDragData, StandardButton, TabBarVisibility, TabWidget, ToastRegistry,
};

use frontend::AppContext;
use frontend::commands::{
    binder_commands, comment_commands, undo_redo_commands, work_commands, work_management_commands,
};
use frontend::common::direct_access::binder::BinderRelationshipField;
use frontend::common::direct_access::comment::CommentRelationshipField;
use frontend::common::direct_access::work::WorkRelationshipField;
use frontend::common::event::{DirectAccessEntity, EntityEvent, Origin};
use frontend::work_management::{CloseWorkDto, LoadWorkDto, NewWorkDto};

use crate::app_ids::AppIds;
use crate::export::panel::ExportPanel;
use crate::models::TreeNode;
use crate::new_work::panel::NewWorkPanel;
use crate::sessions::{StackTeardown, WindowTeardown, WorkRegistry, WorkSession};
use crate::settings::SettingsPanel;
use crate::toast_scope::ToastWorkExt;

use crate::backup::{BackupSchedulerViewModel, BackupSettingsViewModel};
use crate::binder::OutlineViewModel;
use crate::editors::{EditorsViewModel, Side};
use crate::export::ExportViewModel;
use crate::project::{PendingSwitch, ProjectSwitchViewModel, UnsavedDecision, unsaved_decision};
use crate::save::{SaveAsViewModel, SpinnerGate};
use crate::search::SearchReplaceViewModel;
use crate::tabs::{ContentTab, tab_pane};

/// Narrowest an editor tab may be squeezed, in dp — well above teksilo's 96 dp
/// default. A writing project's tabs are near-identical by design ("Chapter 11",
/// "Chapter 12", "Chapter 13"), and the default truncates every one of them to
/// the same useless "Chap…" as soon as a handful are open: the strip stops
/// naming anything, which is the one job it has. This fits a two-word title
/// before the ellipsis; past it the bar scrolls (arrows + the overflow
/// dropdown) rather than squeezing further.
const MIN_EDITOR_TAB_WIDTH: f32 = 160.0;

/// Build one editor pane's `TabWidget`: dynamic tabs, cross-pane migration
/// (`accept_external_tabs` + `on_tab_received` dedup + `on_transfer_out`
/// collapse), close, and `trailing` in the tab-strip trailing slot. Shared by
/// both panes — main and side — so their chrome can't drift.
fn build_pane_tabs(
    editors: &EditorsViewModel,
    side: Side,
    trailing: impl Widget + 'static,
    bar_visibility: Signal<TabBarVisibility>,
) -> TabWidget {
    let close = editors.clone();
    let recv = editors.clone();
    let out = editors.clone();
    let reorder = editors.clone();
    TabWidget::new(editors.selected(side))
        .dynamic_tab::<ContentTab>("editor", |_handle, state| tab_pane(state))
        .dynamic_model(editors.tabs(side))
        .on_close(move |tab_id, _ctx| close.close_in(side, tab_id))
        .on_tab_received(move |handle, _idx, _ctx| recv.receive_tab(side, handle))
        .on_transfer_out(move |tab_id, _ctx| out.transfer_out(side, tab_id))
        // Not `.reorderable(true)`: installing a handler **replaces** teksilo's
        // default (a bare `model.move_item`), so `reorder_in` performs the move
        // itself. It is installed for one reason — a drag must not be able to
        // carry a tab across the pinned boundary, which is the invariant that
        // keeps model order equal to visual order and so keeps the workspace
        // capture and the arrow-key tab navigation honest.
        .on_reorder(move |tab_id, to, _ctx| reorder.reorder_in(side, tab_id, to))
        .accept_external_tabs(true)
        .bar_visibility(bar_visibility)
        .compact_bar()
        .min_tab_width(MIN_EDITOR_TAB_WIDTH)
        .selected_tab_background(SurfaceRole::Content)
        .hover_tab_background(Hover)
        .tab_dividers()
        .active_indicator(teksilo::widgets::TabIndicatorPosition::InnerEdge)
        .bar_trailing_slot(trailing)
}

/// Open every binder item in a dropped `RowDragData<TreeNode>` payload, routing
/// each `(item_id, title)` through `open` (which picks the pane / side). Binder
/// rows (no `item_id`) are ignored. Returns whether anything opened — the drop's
/// accept verdict.
fn drain_dropped(mut payload: DragPayload, mut open: impl FnMut(u64, &str)) -> bool {
    let Some(rd) = payload.take_typed::<RowDragData<TreeNode>>() else {
        return false;
    };
    let Some(items) = rd.items else {
        return false;
    };
    let mut opened = false;
    for node in items {
        if let Some(item_id) = node.item_id {
            open(item_id, &node.title);
            opened = true;
        }
    }
    opened
}

/// A close gesture deferred until the in-flight save finishes. The window's
/// close guard, the `work.close` action, `app.quit`'s action, and
/// `welcome.show` (aliased to `work.close`) all set this; `App` kicks the
/// save, and the SaveWork-completion event performs the action — so the
/// async save is awaited.
///
/// Two outcomes. Every guarded close that still goes through `close_window()`
/// (title-bar X, Alt+F4, Ctrl+W, File ▸ Close Work, File ▸ Welcome…) closes
/// that Work via [`close_work_and_return_to_launcher`] — which opens the
/// Launcher only when no other Work still has an open window. Ctrl+Q / File ▸
/// Quit never calls `close_window()` at all — `app.quit`'s action runs
/// through [`QuitSequencer`](crate::project::QuitSequencer) and terminates the process. Both outcomes share
/// the exact same branch order (`unsaved_decision`/`UnsavedDecision`); only
/// the terminal action differs.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum PendingExit {
    #[default]
    None,
    /// Release the open project once saved (and, if configured, once the
    /// on-close backup finishes). Opens the Launcher only when no other Work
    /// still has an open window — see [`close_work_and_return_to_launcher`].
    ReturnToLauncher,
    /// Release the open project and terminate the process entirely, once
    /// saved (and, if configured, once the on-close backup finishes) —
    /// [`QuitSequencer`](crate::project::QuitSequencer)'s continuation, run from
    /// `BackupSchedulerViewModel::do_close`. Unlike `ReturnToLauncher` this
    /// does NOT open a fresh Launcher window: the project window force-closes
    /// with nothing reopened, so `WindowManager::is_empty()` trips and the
    /// event loop exits.
    Quit,
}

/// A backend-mutating action deferred to a freshly-created project window's
/// first build (see `App::build`'s first-build logic below) — performed only
/// *after* that window's `LoadWork`/`NewWork` event subscriptions are live, so
/// the seeding they perform (`AppIds::seed`, `SingleWork::set_id`, the tree
/// reload, …) never races the event that would otherwise fire before anyone
/// is listening.
///
/// Mirrors the pre-existing "open the argv path once mounted" mechanism; the
/// Launcher's recents / file-picker / examples / New Work flows all build a
/// project window with one of these instead of touching the backend directly
/// from the Launcher's own (App-less) window.
#[derive(Clone, Debug)]
pub enum PendingAction {
    /// Load an existing `.skrib` at this path.
    Load(String),
    /// Create a brand-new work from this DTO (the Launcher's "New Work").
    ///
    /// `then_import` is the cold-start path: the Launcher's "From documents…"
    /// creates the project this flag rides with, and the Import documents wizard
    /// opens over it as soon as it exists (see the cold-start subscriber in
    /// `wiring::project_events`). The files themselves are chosen *there* — the
    /// door used to pick them first and then ask again in the wizard. `false`
    /// for every other New Work door.
    ///
    /// Carried on the action rather than parked on the factory: a factory field
    /// would mean "the next window this creates", which is an ordering
    /// assumption nothing enforces — this names the one window that asked.
    ///
    /// `starters` is what the form asked the project to start with beyond its template —
    /// a tag palette, a set of note templates — and rides here for the same reason
    /// `then_import` does: the project is created in the window this action opens, not
    /// the one whose form was filled in, so a one-shot armed on the presenting side
    /// would never be seen.
    New {
        dto: NewWorkDto,
        then_import: bool,
        starters: ProjectStarters,
    },
    /// Show a Work that is **already open** in another window of this process —
    /// Work ▸ New Window. The one action that performs no backend mutation at
    /// all: the `Work` is loaded, its `AppIds` are seeded, its singles point at
    /// it, and its undo stack is open, because a sibling window did all of that
    /// already. What is left is purely this window's own share of the seeding —
    /// see the `attach_seed` closure `wiring::project_events::install_lifecycle`
    /// returns, which stands in for the `LoadWork` event that will never fire
    /// here.
    ///
    /// `work_id` is resolved (and its session refcount bumped, via
    /// [`crate::sessions::WorkRegistry::attach`]) by
    /// `ProjectWindowFactory::attached_window_config` *before* the window is
    /// opened, so by the time this reaches `App` the Work cannot have gone away
    /// underneath it. `path`/`ordinal` are carried for the window's persistence
    /// id and its title suffix.
    AttachExisting {
        work_id: u64,
        /// The Work's own file path — the base its window-persistence id is
        /// derived from, exactly as [`Self::Load`]'s is.
        path: String,
        /// The ordinal reserved for this window on `work_id` (see
        /// [`crate::sessions::WorkRegistry::reserve_window_ordinal`]).
        ordinal: usize,
        /// The one `BinderItem` this window opens on arrival — the tab-strip
        /// menu's "Move into a new window". `None` for a plain Work ▸ New
        /// Window, which keeps its empty desk.
        ///
        /// A store `EntityId` rather than a durable `uid`, which does **not**
        /// break the "persist by uid" rule: nothing here is persisted. The
        /// action lives for milliseconds inside one process over the one live
        /// store, `ctx.open_window` builds the new window synchronously so no
        /// reload can re-mint ids in between, and the variant already carries a
        /// `work_id: u64` on exactly the same terms.
        ///
        /// Carried on the action rather than parked on the factory for the
        /// reason [`Self::New`]'s `then_import` gives: a field on the factory
        /// would mean "the next window this creates", an ordering assumption
        /// nothing enforces, where this names the one window that asked.
        open_item: Option<u64>,
    },
}

impl PendingAction {
    /// The on-disk path this action targets — used to derive the project
    /// window's persistence id ([`crate::shell::windows::window_id_for`]) before the
    /// action itself has run.
    pub fn target_path(&self) -> &str {
        match self {
            PendingAction::Load(path) => path,
            PendingAction::New { dto, .. } => &dto.file_name,
            PendingAction::AttachExisting { path, .. } => path,
        }
    }

    /// The `work_id` this action attaches to, when it attaches to one at all.
    /// `None` for [`Self::Load`]/[`Self::New`] — neither knows its Work's id
    /// until the backend mints it.
    pub fn attached_work_id(&self) -> Option<u64> {
        match self {
            PendingAction::AttachExisting { work_id, .. } => Some(*work_id),
            _ => None,
        }
    }
}

/// Release the open project: fires `CloseWork` (releases the open-registry
/// claim and clears `AppIds`/the tree/the singles via the subscriber below),
/// force-closes every window showing that Work, and — **only when no other
/// Work still has an open window** — opens (or focuses) the Launcher so the
/// process is never briefly windowless (which would quit it; see `main.rs`).
///
/// When another Work is still open elsewhere, the Launcher stays closed and
/// the first surviving project window is focused instead.
///
/// Callers invoking this from inside an `on_close_requested` guard must return
/// `CloseResponse::Veto` afterward: this function performs the actual close
/// itself, via `close_window_forced`, rather than deferring to the guard's own
/// return value.
///
/// **This closes the WORK, so it closes every window showing it** (Work ▸ New
/// Window can open several on one Work, via `WorkRegistry::windows_for`) —
/// `close_work` tears out the one backend subtree they all read, so a sibling
/// left open would be a window onto nothing. Closing *a window* rather than
/// the project is the narrower gesture (title-bar X / Alt+F4 with a sibling
/// still open); it never reaches here — see the close guard in
/// `shell::windows`, which resolves that case first.
pub fn close_work_and_return_to_launcher(
    app_ctx: &Rc<AppContext>,
    ids: &AppIds,
    workspace_layout: &crate::workspace_layout::WorkspaceLayoutViewModel,
    ctx: &mut EventContext,
) {
    // Capture the desk (open tabs + docks) while the store is still alive —
    // `close_work` tears the Work subtree out *before* publishing `CloseWork`, so a
    // subscriber could no longer translate a tab into its persistable ordinal.
    // `workspace_layout`/`ids` are the **calling window's own** instances, passed
    // in explicitly: `ctx.app_state::<T>()` can only ever answer with whichever
    // window built `main`'s bootstrap session — reading it here with a second Work
    // open in a second window would close the WRONG Work.
    capture_workspace_layout(workspace_layout);
    // `close_work` takes a `CloseWorkDto{work_id}` resolved from `ids` (see above);
    // skipped entirely when none is open.
    //
    // The sibling windows on this same Work are collected *before* the close, so
    // the list is read while the registry bindings are still intact, and closed
    // *after* it, so each one's `CloseWork` subscriber has already run in its own
    // window (clearing its tabs and unpointing its tree) before its window goes
    // away. `close_window_by_id` is deliberately the unconditional close: their
    // own close guards must not run, and letting each one open its own Launcher
    // would spawn one per window.
    //
    // `WorkRegistry` is genuinely one registry per process, so unlike
    // `AppIds`/`WorkspaceLayoutViewModel` this `app_state` lookup is the right
    // way to reach it.
    let registry = ctx.app_state::<crate::sessions::WorkRegistry>().cloned();
    let my_work_id = ids.work_id.get();
    let me = ctx.window().map(|w| w.id());
    let siblings = my_work_id
        .map(|work_id| {
            registry
                .as_ref()
                .map(|reg| reg.windows_for(work_id))
                .unwrap_or_default()
                .into_iter()
                .filter(|id| Some(*id) != me)
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    // Another Work still has at least one open window? Then Welcome must not
    // open — only the *last* project Work's close returns to the Launcher.
    // Read before `close_work` tears this Work's session out of the registry.
    let survivor = registry.as_ref().and_then(|reg| {
        reg.open_work_ids()
            .into_iter()
            .filter(|&wid| Some(wid) != my_work_id)
            .find_map(|wid| reg.windows_for(wid).into_iter().next())
    });
    if let Some(work_id) = my_work_id {
        let _ = work_management_commands::close_work(app_ctx, &CloseWorkDto { work_id });
    }
    for window in siblings {
        ctx.close_window_by_id(window);
    }
    if let Some(survivor) = survivor {
        // Other project windows remain: raise one of them so focus does not
        // die with the Work we just closed.
        ctx.focus_window(survivor);
    } else {
        // Last open Work: open (or reuse) the Launcher *before* force-closing
        // this window so the process is never briefly windowless mid-transition
        // (that would quit it — see `main.rs`'s module docs).
        //
        // A different window's own last-Work close may already have opened the
        // Launcher — `WindowManager::create_window` has no string-id dedup of its
        // own, so a second `open_window` with the same `.id(LAUNCHER_WINDOW_ID)`
        // would spawn a second, orphaned Launcher window. Reuse and focus the
        // existing one if it's already open.
        match ctx.find_window(crate::shell::windows::LAUNCHER_WINDOW_ID) {
            Some(existing) => ctx.focus_window(existing),
            None => {
                ctx.open_window(crate::shell::windows::launcher_window_config(
                    app_ctx.clone(),
                ));
            }
        }
    }
    ctx.close_window_forced();
}

/// **May this window replace its own project in place?** `false` means it must
/// open the incoming project in a *new* window instead — the answer every one of
/// the four switch doors (File ▸ New Work, File ▸ Open Work…, the switcher's
/// "Open here", the import toast's "Open now") consults before doing anything.
/// Delegates to [`WindowRole::may_switch_in_place`] — the one home for
/// attach/sibling policy.
pub(crate) fn may_switch_project_in_place(
    registry: &WorkRegistry,
    ids: &AppIds,
    role: WindowRole,
) -> bool {
    role.may_switch_in_place(registry, ids)
}

/// Persist the open project's workspace layout (open tabs + dock arrangement)
/// through `workspace_layout` — the **calling window's own** [`WorkspaceLayoutViewModel`](crate::workspace_layout::WorkspaceLayoutViewModel)
/// handle, passed in explicitly rather than resolved via
/// `ctx.app_state::<WorkspaceLayoutViewModel>()` (multi-Work migration: that slot is
/// one process-wide registration fixed at builder time from the *first* window's
/// session — reading it here would capture window 1's desk again, on every window's
/// close/quit, and never the closing/quitting window's own). Called at each "leave
/// the project" door while its store is still alive.
///
/// `workspace_layout.capture()` and `workspace_layout.capture_tree_expansion()`
/// both read entirely off this Tier-2 view-model's own injected/held state;
/// neither needs an `EventContext`.
pub(crate) fn capture_workspace_layout(
    workspace_layout: &crate::workspace_layout::WorkspaceLayoutViewModel,
) {
    workspace_layout.capture();
    workspace_layout.capture_tree_expansion();
}

/// Close `work_id`'s backend subtree — the window's own OUTGOING Work — right
/// before an in-place replace (New Work / Open Work / the switcher's "Open
/// here" / the import toast's "Open now") swaps in a different one. `work_id`
/// is `None` when nothing was open yet (a no-op).
///
/// `load_work`/`new_work` no longer close other open Works themselves (two
/// DIFFERENT Works can coexist in two windows), so nothing else closes THIS
/// window's own outgoing Work on an in-place replace — without this call its
/// whole backend subtree (`Work`/`WorkInfo`/`Binder`/`BinderItem`/`Content`/
/// `DictWord`/…) and its `SpellcheckService` personal-word/mute entry leak for
/// the rest of the process's life, since only a real `CloseWork` event drives
/// `ProjectLifecycleViewModel::on_close`.
///
/// **The caller must resolve `work_id` from the window's OWN `AppIds`
/// (`ids.work_id.get()`), captured before anything re-points it** — never
/// `ctx.app_state::<AppIds>()`, which is one process-wide slot fixed at
/// builder time from the *first* window's session: with a second Work open in
/// a second window, that slot answers with the WRONG window's `work_id`, and
/// closing it would tear a sibling window's live, untouched Work out from
/// under it.
///
/// `WorkRegistry`'s session/undo-stack bookkeeping is untouched by this (by
/// design — see `on_close`'s own doc): that half already runs from
/// `register_window`'s replace path once the new `LoadWork`/`NewWork` lands.
pub(crate) fn close_outgoing_work(app_ctx: &Rc<AppContext>, work_id: Option<u64>) {
    if let Some(work_id) = work_id {
        let _ = work_management_commands::close_work(app_ctx, &CloseWorkDto { work_id });
    }
}

/// Perform `outcome` immediately — [`close_work_and_return_to_launcher`] for
/// `ReturnToLauncher`; `Quit` is a no-op here (see the match arm below).
/// Shared by every "user picked Discard" branch in [`guard_unsaved_exit`], so
/// discarding unsaved edits always skips the on-close backup (the
/// last-saved state is what's kept).
fn perform_exit(
    outcome: PendingExit,
    app_ctx: &Rc<AppContext>,
    ids: &AppIds,
    workspace_layout: &crate::workspace_layout::WorkspaceLayoutViewModel,
    ctx: &mut EventContext,
) {
    match outcome {
        PendingExit::ReturnToLauncher => {
            close_work_and_return_to_launcher(app_ctx, ids, workspace_layout, ctx)
        }
        // Never reached: `guard_unsaved_exit` only ever carries
        // `ReturnToLauncher` now. `Quit` belongs to `QuitSequencer`, whose
        // continuation runs in `BackupSchedulerViewModel::do_close` instead of
        // here — quitting spans every window, so it cannot be expressed as one
        // window's exit. A silent no-op rather than a panic: an unreachable
        // state is not worth taking the app down for.
        PendingExit::Quit => {}
        PendingExit::None => unreachable!("guard_unsaved_exit is never invoked with outcome=None"),
    }
}

/// **The** unsaved-changes guard shared by `work.close`'s action, `app.quit`'s
/// action, and the project window's `on_close_requested` (`windows.rs`) — the
/// single branch order every exit path in the app uses, mirroring
/// [`ProjectSwitchViewModel::request`]'s four switch doors, which already
/// share [`unsaved_decision`]. `outcome` is `ReturnToLauncher` or `Quit`
/// (never `None` — that would mean nothing to guard, which none of the three
/// callers ever ask for).
///
/// * [`UnsavedDecision::Proceed`] — nothing to protect: hand straight to
///   `scheduler.on_close_flow`, which itself takes the on-close backup (if
///   configured and not suppressed by backup mode) before performing
///   `outcome`.
/// * [`UnsavedDecision::SaveThenProceed`] — autosave is on: just arm
///   `pending_exit = outcome`. The existing save-then-close machinery (the
///   `pending_exit` effect + the `LongOperation::Completed` handler, both in
///   `App::build`, unchanged) resumes the outcome once the save lands.
/// * [`UnsavedDecision::PromptDiscardOnly`] — a backup file is open here
///   (Save is off): Discard performs `outcome` at once (bypassing
///   `on_close_flow` — discarding a backup's edits never backs anything up);
///   Cancel does nothing.
/// * [`UnsavedDecision::PromptSaveDiscardCancel`] — Save arms
///   `pending_exit = outcome` (same resumption as `SaveThenProceed`); Discard
///   performs `outcome` at once; Cancel does nothing.
///
/// The dialog copy differs by `outcome` (quitting says so, rather than
/// reusing "closing the work") — see `quit-question` /
/// `quit-backup-discard-title` / `quit-backup-discard-text` in the locales.
#[allow(clippy::too_many_arguments)]
pub(crate) fn guard_unsaved_exit(
    ctx: &mut EventContext,
    app_ctx: &Rc<AppContext>,
    ids: &AppIds,
    unsaved: bool,
    backup_mode: bool,
    autosave: bool,
    pending: &Signal<PendingExit>,
    scheduler: &BackupSchedulerViewModel,
    outcome: PendingExit,
) {
    match unsaved_decision(unsaved, backup_mode, autosave) {
        UnsavedDecision::Proceed => scheduler.on_close_flow(ctx, outcome),
        UnsavedDecision::SaveThenProceed => pending.set(outcome),
        UnsavedDecision::PromptDiscardOnly => {
            let app_ctx = app_ctx.clone();
            let ids = ids.clone();
            let workspace_layout = scheduler.workspace_layout().clone();
            let (title, text) = match outcome {
                PendingExit::Quit => (
                    tr!(quit_backup_discard_title()),
                    tr!(quit_backup_discard_text()),
                ),
                _ => (
                    tr!(close_backup_discard_title()),
                    tr!(close_backup_discard_text()),
                ),
            };
            ctx.present_message_box(
                MessageBox::question(title)
                    .text(text)
                    .buttons(MessageBoxButtons::Custom(vec![
                        MessageBoxButton::standard(StandardButton::Discard),
                        MessageBoxButton::standard(StandardButton::Cancel),
                    ]))
                    .default_button(StandardButton::Cancel)
                    .escape_button(StandardButton::Cancel)
                    .on_result(move |r, ctx| {
                        if r.button == StandardButton::Discard {
                            perform_exit(outcome, &app_ctx, &ids, &workspace_layout, ctx);
                        }
                    }),
            );
        }
        UnsavedDecision::PromptSaveDiscardCancel => {
            let app_ctx = app_ctx.clone();
            let ids = ids.clone();
            let workspace_layout = scheduler.workspace_layout().clone();
            let pe = pending.clone();
            let title = match outcome {
                PendingExit::Quit => tr!(quit_question()),
                _ => tr!(close_work_question()),
            };
            ctx.present_message_box(
                MessageBox::question(title)
                    .text(tr!(unsaved_changes()))
                    .buttons(MessageBoxButtons::SaveDiscardCancel)
                    .default_button(StandardButton::Save)
                    .escape_button(StandardButton::Cancel)
                    .on_result(move |r, ctx| match r.button {
                        StandardButton::Save => pe.set(outcome),
                        StandardButton::Discard => {
                            perform_exit(outcome, &app_ctx, &ids, &workspace_layout, ctx)
                        }
                        _ => {}
                    }),
            );
        }
    }
}

/// Convert a theme colour role to the `text_document` colour a highlight span carries. Used to
/// paint spell-check squiggles in the theme's `text_error` role — a semantic role, not a hex
/// literal, so light/dark both work.
fn spell_underline_color(c: teksilo::tokens::Color) -> teksilo::text_document::Color {
    let [r, g, b, _] = c.to_array();
    let to_u8 = |x: f32| (x.clamp(0.0, 1.0) * 255.0).round() as u8;
    teksilo::text_document::Color::rgb(to_u8(r), to_u8(g), to_u8(b))
}

/// After a project becomes live, offer to install any dictionary its declared languages need
/// but the machine lacks — one aggregated toast (never one per language), whose action opens
/// Settings ▸ Dictionaries with the missing set highlighted. Purely additive and dismissible,
/// so a toast, not a modal.
pub(crate) fn offer_missing_dictionaries(
    docs: &crate::models::OpenDocsStore,
    dictionaries: &crate::spellcheck::DictionariesViewModel,
    session: &WorkSession,
    undo: &crate::edit::UndoGroupViewModel,
    ctx: &mut EventContext,
) {
    let undo = undo.clone();
    let missing = dictionaries.missing_for(&docs.project_languages());
    if missing.is_empty() {
        return;
    }
    dictionaries.set_highlight(missing.clone());
    let n = missing.len() as i64;
    // Work-scoped: this Work's own declared languages, so only this
    // window/bell needs to hear about it — not every open project.
    let work_id = session.ids.work_id.get();
    let session = session.clone();
    ctx.show_toast(
        teksilo::widgets::Toast::info(tr!(dict_missing_toast(count = n)))
            // Work-scoped (F1): a bare "dict.missing" shared by every window
            // would let a second Work's own nudge find THIS Work's still-live
            // toast and silently steal/retarget it.
            .scoped_id("dict.missing", work_id)
            .target_work(work_id)
            .action(teksilo::widgets::ToastAction::primary(
                tr!(dict_missing_action()),
                move |c| {
                    let session = session.clone();
                    let undo = undo.clone();
                    crate::settings::present(c, move || {
                        SettingsPanel::open_to_dictionaries(session, &undo)
                    });
                },
            )),
    );
}

/// After a comment is created with nobody's name on it, say so once and offer the
/// page that fixes it.
///
/// Fires only when the resolved signature is anonymous — with a name set, from
/// either source, this never appears. A toast rather than a modal or a
/// pre-flight prompt, for the same reason the missing-dictionary nudge is one:
/// the comment has already been written and is perfectly usable unsigned, so
/// this is information, not a decision to block on. Nothing is retroactive —
/// filling the name in signs the *next* comment, never the ones already stored
/// (see `comments::signature`), which is why the string says so.
pub(crate) fn warn_unsigned_comments(
    comments: &crate::comments::CommentsViewModel,
    session: &WorkSession,
    undo: &crate::edit::UndoGroupViewModel,
    ctx: &mut EventContext,
) {
    let undo = undo.clone();
    if !comments.signature().is_anonymous() {
        return;
    }
    let work_id = session.ids.work_id.get();
    let session = session.clone();
    ctx.show_toast(
        teksilo::widgets::Toast::info(tr!(comments_unsigned_toast()))
            // Work-scoped for the same reason `dict.missing` is: a bare id shared
            // by every window would let a second Work's nudge find and retarget
            // this one's still-live toast. It also means comment after comment
            // replaces the same toast instead of stacking one per remark.
            .scoped_id("comments.unsigned", work_id)
            .target_work(work_id)
            .action(teksilo::widgets::ToastAction::primary(
                tr!(comments_unsigned_action()),
                move |c| {
                    let session = session.clone();
                    let undo = undo.clone();
                    crate::settings::present(c, move || {
                        SettingsPanel::open_to_user(session, &undo)
                    });
                },
            )),
    );
}

pub struct App {
    app_ctx: Rc<AppContext>,
    /// The Tier-2 per-open-Work bundle (see `sessions::WorkSession`'s module
    /// doc) — built **fresh for this window** by `ProjectWindowFactory::window_config`
    /// (a second simultaneously-open Work gets its own independent
    /// `WorkSession`, not a second handle onto the first window's) and handed to
    /// `App::new` the way `outline` already is. `App::build` reads its fields
    /// straight off `self.session` rather than `ctx.app_state::<T>()` per field
    /// — that lookup is one process-wide slot, last-write-wins across windows,
    /// and only still used by a handful of residual consumers. Prefer
    /// `session` over `app_state` for any new code.
    ///
    /// Most of the fields below share this same "minted fresh per window in
    /// the factory, never `ctx.app_state`" shape, for the identical reason:
    /// two simultaneously-open windows — even on the same Work — must never
    /// share UI state that names one window's own placement/selection/target.
    session: WorkSession,
    /// Built fresh alongside `session` — each simultaneously-open Work gets
    /// its own outline/tree, never a second window's.
    outline: OutlineViewModel,
    /// This window's own "was I maximized/floating before I went fullscreen"
    /// memory (Increment 1 of distraction-free — plain fullscreen). See
    /// `FullscreenViewModel`'s doc.
    fullscreen: crate::shared::FullscreenViewModel,
    /// This window's own distraction-free state (Increment 2 — chrome
    /// collapse), with its own independent placement memory from
    /// [`Self::fullscreen`] — see `FocusViewModel`'s module doc for why the
    /// two toggles never share one. Reset on Close-Work/Load-Work so a stale
    /// "mode was on" never leaks into the next project this window shows.
    focus: crate::shared::FocusViewModel,
    /// Bound to this window's own `ids`, so an export from this window scopes
    /// to *this* Work. See `app::commands::CommandDeps::export`'s doc.
    export: crate::export::ExportViewModel,
    /// The Import documents wizard's state, bound to this window's own `ids` —
    /// so a manuscript imported from this window lands in *this* project. Also
    /// the subscriber the analysis's long-operation events are routed to (see
    /// `app::wiring::long_ops`), which is why it must outlive the modal: the
    /// panel is built and destroyed around it, not the other way round.
    import_document: crate::import_document::ImportDocumentViewModel,
    /// The Tier-1 registry every open Work registers into once its own
    /// `LoadWork`/`NewWork` resolves a real `work_id` (see the `LoadWork`/
    /// `NewWork` subscribers in `build`, which also bind this window's id to
    /// that `work_id` — `WorkRegistry::register_window`). Unregistered not on
    /// `CloseWork` but either once teksilo's `on_removed` hook confirms this
    /// window is really gone (`WorkRegistry::remove_window`, wired in
    /// `shell::windows`), or the instant this same window's own
    /// `register_window` call supersedes its own previous binding on an
    /// in-place Work switch (see that method's doc).
    registry: WorkRegistry,
    /// The app-global quit sequencer (`app.quit`). Shared across every window,
    /// unlike almost everything else on this struct: a quit spans them all.
    quit: crate::project::QuitSequencer,
    /// Built fresh alongside `session`, bound to *this* window's own `ids`/
    /// `single_work`.
    save_as_vm: SaveAsViewModel,
    restore_vm: crate::backup::BackupRestoreViewModel,
    /// This window's own formatting surfaces (dock + menu + editor registry).
    /// Never shared — a shared instance made the last-built window win the
    /// Format dock's live target.
    format: crate::format::FormatViewModel,
    /// Which history Ctrl+Z means, and what it would take back. Tier 3 for the
    /// same reason `format` is — see [`crate::edit`].
    undo_group: crate::edit::UndoGroupViewModel,
    /// Holds this window's structural undo claim alive for as long as the shell
    /// is mounted. Dropped and re-made on every rebuild, which is correct: the
    /// claim names a `focus_within` signal, and a rebuild mints a new one.
    undo_claim: std::cell::RefCell<Option<crate::edit::UndoClaim>>,
    /// This window's own in-place project-switch guard, with *this* Work's
    /// `unsaved`/`backup_mode`.
    project_switch: ProjectSwitchViewModel,
    /// Scope D — window titles. The live `"{Work title} — Skribisto"` string
    /// (see `shell::windows::window_title_text`'s doc), built once in
    /// `ProjectWindowFactory::window_config` alongside `session` and already
    /// bound to the drawn custom title bar there. `App::build` pushes every
    /// change to `ctx.window()`'s OS-level title too (see the effect near the
    /// top of `build`), so the taskbar/Alt-Tab/KWin-visible title and the
    /// drawn one never drift apart.
    title_text: Signal<String>,
    /// Scope D — this window's own "which window on its Work am I" ordinal
    /// (see `sessions::WorkRegistry::register_window`'s doc). Starts at `1`
    /// (or the reserved ordinal for Work ▸ New Window) and is written with the
    /// real assigned value by the `LoadWork`/`NewWork`/attach subscribers in
    /// `build`, the moment `work_id` — and so this window's place among any
    /// siblings on it — becomes known.
    window_ordinal: Signal<usize>,
    /// This window's distraction-free surface. Mounted at the *window* root (so
    /// it can cover the title bar), but its dependencies are built here — see
    /// `DistractionFreeSurfaceViewModel`'s module doc — so `build` hands them
    /// over with `attach`.
    df_surface: crate::distraction_free::DistractionFreeSurfaceViewModel,
    /// Plain mirror of the persisted autosave setting, read by the title-bar menu
    /// (outside `App`) to hide the manual "Save" item. `App::build` mirrors the
    /// store-backed setting into it.
    autosave_menu: Signal<bool>,
    /// Plain mirror of the persisted master spell-check switch, read by the title-bar
    /// (outside `App`) for the toggle's icon + the View ▸ Check spelling checkmark.
    /// `App::build` mirrors the store-backed setting into it.
    spellcheck_menu: Signal<bool>,
    /// Plain mirror of the persisted Tools ▸ Comments switch, read by the title-bar's
    /// menu (built outside `App`) for its checkmark. `App::build` mirrors the
    /// store-backed setting into it — the menu row never writes it back.
    comments_menu: Signal<bool>,
    margin_lane_menu: Signal<bool>,
    /// Live "a scene's prose is the active surface" flag, shared with the
    /// title-bar's Format menu so its entries grey out off a scene. Written by
    /// `EditorsViewModel`, which is the only thing that can compute it.
    scene_focused: Signal<bool>,
    binder_has_selection: Signal<bool>,
    /// This window's menu model plus the id of its "Insert template" submenu, so
    /// `build` can refill that submenu as the catalogue changes. `None` where the
    /// platform gives no title-bar host and there is no model to refill.
    templates_menu: Option<(
        teksilo::widgets::MenuModel,
        teksilo::core::menu_item_id::MenuItemId,
    )>,
    /// Pre-allocated id for the **Image** menu, which is inserted and removed as
    /// the selection changes (see `project_menus::sync_image_menu`). Minted here
    /// rather than by the insertion, because a menu that will be removed again
    /// has to be nameable before it exists.
    image_menu_id: teksilo::core::menu_item_id::MenuItemId,
    /// Live "is there a target" mirrors for the title-bar's Go menu (Increment 4 —
    /// six Next/Previous × Scene/Chapter/Note rows), the same shape as
    /// `scene_focused` just above: minted in `shell/windows.rs` (which builds the
    /// menu before this `App`/its `EditorsViewModel` exist), forwarded to
    /// `EditorsViewModel::new` here so it can write the live answer, and read
    /// straight from the window-chrome closure's own clone for the menu's
    /// `.enabled(..)` bindings. See [`crate::go::GoAvailability`]'s doc.
    go: crate::go::GoAvailability,
    /// This window's "jump to any item" popup state — see `GoToViewModel`.
    go_to: crate::go::GoToViewModel,
    /// `true` while the open work has edits not yet written to disk. Read by the
    /// close guard, `work.close` and the switch guard to decide whether to prompt,
    /// and by `can_save` for the Save affordances.
    ///
    /// **Derived**, not set by hand: `dirty_seq > saved_seq`, both read off the
    /// shared [`crate::save::SaveStateViewModel`] (Work-scoped, not owned
    /// here — see its module docs). A plain flag cleared on save completion
    /// would lie while a save is in flight: typing during that save would be
    /// marked clean the moment it finished, even though its snapshot never
    /// contained those edits.
    unsaved: Signal<bool>,
    /// A deferred close: performed once the save it asked for actually covers the
    /// edits (see `exit_seq`). Shared with `main`'s window close guard.
    pending_exit: Signal<PendingExit>,
    /// The edit sequence [`Self::pending_exit`] is waiting to see on disk.
    exit_seq: Rc<std::cell::Cell<Option<u64>>>,
    /// The save indicator's "saving…" hysteresis, and the gate's answer. Held here,
    /// not in the widget: `build` constructs a fresh `SaveIndicator` on every run, so
    /// a gate owned by the widget would have a slow save's delay / min-display timers
    /// reset by any unrelated App rebuild.
    save_spinner: Rc<std::cell::RefCell<SpinnerGate>>,
    save_spinner_visible: Signal<bool>,
    /// True while a *backup file* is open here (Save + auto-backup off; content
    /// still editable). Shared with `main`'s title-bar menu.
    backup_mode: Signal<bool>,
    /// The open backup's details (drives the permanent banner + restore), or
    /// `None` for a normal project.
    backup_context: Signal<Option<crate::backup::BackupContext>>,
    /// The backend mutation this window's project performs once mounted (load
    /// an existing path, or create a brand-new work) — taken on first build,
    /// after the `LoadWork`/`NewWork` subscriptions are live so the full
    /// seed flow runs. See [`PendingAction`].
    initial_action: Option<PendingAction>,
    /// One-shot guard so `initial_action` runs only on the first build.
    initial_loaded: bool,
    /// How this window reached its Work — [`WindowRole::Owner`] vs
    /// [`WindowRole::Attached`]. Fixed for the window's whole life (derived
    /// once in [`App::new`]); see that type for desk ownership, switch policy,
    /// and project-switch hook installation.
    role: WindowRole,
    /// Created once on first build (its column-width signal needs `ctx.settings()`).
    editors: Option<EditorsViewModel>,
    /// Stable id for the trailing Inspector dock (created once so a rebuild keeps
    /// the same dock in the `DockingModel`).
    inspector_dock: DockWidgetId,
    /// Stable id for the trailing Format dock (Inspector's neighbour).
    format_dock: DockWidgetId,
    /// Stable id for the bottom search-preview dock.
    preview_dock: DockWidgetId,
    /// Stable id for the leading search & replace dock.
    search_dock: DockWidgetId,
    /// The search feature's shared view-model, created once on first build (like
    /// [`editors`](Self::editors) — its debounce/signals want a live context).
    search: Option<SearchReplaceViewModel>,
    /// Stable id for the leading trash dock (third rail tab).
    trash_dock: DockWidgetId,
    /// Stable id for the leading writing-games dock.
    games_dock: DockWidgetId,
    comments_dock: DockWidgetId,
    doc_comments_dock: DockWidgetId,
    footnotes_dock: DockWidgetId,
    versions_dock: DockWidgetId,
    timeline_dock: DockWidgetId,
    /// This row's recorded past. Tier 3 — the timeline is a view of what one
    /// window is focused on, and two windows on the same Work legitimately look
    /// at different rows.
    versions: crate::versions::VersionsViewModel,
    /// The whole project's past. Tier 3 for the same reason `versions` is: the
    /// selected moment and the change list are one window's place in the
    /// history, not the project's.
    timeline: crate::timeline::TimelineViewModel,
    /// The trash feature's shared view-model, created once on first build.
    trash: Option<crate::trash::TrashViewModel>,
    comments: Option<crate::comments::CommentsViewModel>,
    footnotes: Option<crate::footnotes::FootnotesViewModel>,
    /// Keeps this window's `SearchSettingsService` `Reloadable` registration alive
    /// in the shared `SettingsRegistry`, so a peer process's `search.toml` writes
    /// are picked up live — the same story as [`backup_settings_reloadable`](Self::backup_settings_reloadable).
    search_settings_reloadable: Option<Rc<dyn Reloadable>>,
    root_child: Option<WidgetId>,
    /// Keeps `backup_settings`'s `Reloadable` registration alive in the app's
    /// shared `SettingsRegistry` (only a `Weak` is held internally — see
    /// `SettingsRegistry::register`'s docs) so the settings-file watcher keeps
    /// applying a peer process's `backup.toml` writes to this handle for as
    /// long as `App` lives. Registered once, on first build.
    backup_settings_reloadable: Option<Rc<dyn Reloadable>>,
    /// Keeps the dictionary accepted-licence store's `Reloadable` registration alive in the
    /// shared `SettingsRegistry` (mirrors `backup_settings_reloadable`).
    dictionary_settings_reloadable: Option<Rc<dyn Reloadable>>,
    /// Keeps the user export-styles store's `Reloadable` registration alive in the shared
    /// `SettingsRegistry` (mirrors `dictionary_settings_reloadable`).
    export_styles_reloadable: Option<Rc<dyn Reloadable>>,
    /// This window's editor tab-strip context menu (Tier 3).
    ///
    /// An `Rc` created **once per window** in [`App::new`], not per build: every
    /// open tab's menu factory holds a `Weak` to it, so a handle re-minted on a
    /// rebuild would leave every tab built before it with a menu that silently
    /// declines to open.
    tab_menu: Rc<crate::editors::TabMenuViewModel>,
}

impl App {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        app_ctx: Rc<AppContext>,
        session: WorkSession,
        outline: OutlineViewModel,
        fullscreen: crate::shared::FullscreenViewModel,
        focus: crate::shared::FocusViewModel,
        export: crate::export::ExportViewModel,
        import_document: crate::import_document::ImportDocumentViewModel,
        autosave_menu: Signal<bool>,
        spellcheck_menu: Signal<bool>,
        comments_menu: Signal<bool>,
        margin_lane_menu: Signal<bool>,
        scene_focused: Signal<bool>,
        binder_has_selection: Signal<bool>,
        templates_menu: Option<(
            teksilo::widgets::MenuModel,
            teksilo::core::menu_item_id::MenuItemId,
        )>,
        go: crate::go::GoAvailability,
        go_to: crate::go::GoToViewModel,
        unsaved: Signal<bool>,
        pending_exit: Signal<PendingExit>,
        backup_mode: Signal<bool>,
        backup_context: Signal<Option<crate::backup::BackupContext>>,
        initial_action: PendingAction,
        registry: WorkRegistry,
        quit: crate::project::QuitSequencer,
        save_as_vm: SaveAsViewModel,
        restore_vm: crate::backup::BackupRestoreViewModel,
        format: crate::format::FormatViewModel,
        undo_group: crate::edit::UndoGroupViewModel,
        project_switch: ProjectSwitchViewModel,
        title_text: Signal<String>,
        window_ordinal: Signal<usize>,
        df_surface: crate::distraction_free::DistractionFreeSurfaceViewModel,
    ) -> Self {
        let tab_menu = Rc::new(crate::editors::TabMenuViewModel::new(
            app_ctx.clone(),
            session.ids.clone(),
        ));
        Self {
            tab_menu,
            df_surface,
            app_ctx,
            session,
            outline,
            fullscreen,
            focus,
            export,
            import_document,
            registry,
            quit,
            save_as_vm,
            restore_vm,
            format,
            undo_group,
            undo_claim: std::cell::RefCell::new(None),
            project_switch,
            title_text,
            window_ordinal,
            autosave_menu,
            spellcheck_menu,
            comments_menu,
            margin_lane_menu,
            scene_focused,
            binder_has_selection,
            templates_menu,
            image_menu_id: teksilo::core::menu_item_id::MenuItemId::next(),
            go,
            go_to,
            unsaved,
            pending_exit,
            exit_seq: Rc::new(std::cell::Cell::new(None)),
            save_spinner: Rc::new(std::cell::RefCell::new(SpinnerGate::default())),
            save_spinner_visible: Signal::new(false),
            backup_mode,
            backup_context,
            role: WindowRole::from_action(&initial_action),
            initial_action: Some(initial_action),
            initial_loaded: false,
            editors: None,
            // *Stable* ids (not `fresh()`) so the per-work dock-layout restore
            // can match these docks across launches — see `crate::docks` docs.
            inspector_dock: DockWidgetId::from_raw(crate::docks::INSPECTOR_DOCK_ID),
            format_dock: DockWidgetId::from_raw(crate::docks::FORMAT_DOCK_ID),
            preview_dock: DockWidgetId::from_raw(crate::docks::PREVIEW_DOCK_ID),
            search_dock: DockWidgetId::from_raw(crate::docks::SEARCH_DOCK_ID),
            search: None,
            comments: None,
            footnotes: None,
            trash_dock: DockWidgetId::from_raw(crate::docks::TRASH_DOCK_ID),
            games_dock: DockWidgetId::from_raw(crate::docks::GAMES_DOCK_ID),
            comments_dock: DockWidgetId::from_raw(crate::docks::COMMENTS_DOCK_ID),
            doc_comments_dock: DockWidgetId::from_raw(crate::docks::DOC_COMMENTS_DOCK_ID),
            footnotes_dock: DockWidgetId::from_raw(crate::docks::FOOTNOTES_DOCK_ID),
            versions_dock: DockWidgetId::from_raw(crate::docks::VERSIONS_DOCK_ID),
            timeline_dock: DockWidgetId::from_raw(crate::docks::TIMELINE_DOCK_ID),
            versions: crate::versions::VersionsViewModel::new(),
            timeline: crate::timeline::TimelineViewModel::new(),
            trash: None,
            search_settings_reloadable: None,
            root_child: None,
            backup_settings_reloadable: None,
            dictionary_settings_reloadable: None,
            export_styles_reloadable: None,
        }
    }
}

/// Open the persistent search-settings service (`<config_dir>/search.toml`),
/// degrading to a throwaway per-process temp file if the config dir is
/// unavailable — so the app still runs, search preferences just won't persist.
fn open_search_settings() -> crate::models::SearchSettingsService {
    use crate::models::SearchSettingsService;
    match crate::identity::app_paths() {
        Some(paths) => SearchSettingsService::open(&paths).unwrap_or_else(|e| {
            eprintln!("search settings: open failed ({e}); using an in-memory fallback");
            SearchSettingsService::in_memory_default()
        }),
        None => SearchSettingsService::in_memory_default(),
    }
}

/// The backend mutation events that mark the work "unsaved" (and reschedule the
/// autosave debounce). Editor *typing* is caught separately via the editors'
/// `edited` signal; `Content` events (which fire only on flush) are excluded so a
/// save's own flush doesn't loop the debounce.
///
/// `Comment`/`CommentReply`/`Footnote` are here because their bodies are edited
/// in surfaces that are **not** the manuscript editors — the margin card and the
/// footnotes dock commit through the generated `update_*` commands, so their
/// events are the only signal an edit happened at all. They were missing at
/// first, and a comment edited then closed was silently discarded with no
/// prompt. A kind may only join this list once its write paths are checked for
/// flush- or open-driven **unconditional** writes (the `Content` failure mode):
/// `CommentsViewModel::persist_live_anchors`/`reanchor` and
/// `FootnotesListModel::set_body` all skip no-op writes for exactly this reason.
fn mutation_origins() -> Vec<Origin> {
    use DirectAccessEntity::{
        Binder, BinderItem, BinderStatus, BinderTag, Comment, CommentReply, DictWord, Footnote,
        NoteTemplate, Work,
    };
    let mut v = Vec::new();
    for ent in [
        Work(EntityEvent::Updated),
        BinderItem(EntityEvent::Created),
        BinderItem(EntityEvent::Updated),
        BinderItem(EntityEvent::Removed),
        Binder(EntityEvent::Created),
        Binder(EntityEvent::Updated),
        Binder(EntityEvent::Removed),
        BinderTag(EntityEvent::Created),
        BinderTag(EntityEvent::Updated),
        BinderTag(EntityEvent::Removed),
        // Renaming, reordering or deleting a rung is an edit to the project like any
        // other. Missing this list is the gap that shipped three times — Comment,
        // CommentReply and Footnote each landed with their edits silently discarded by
        // Close, because `unsaved` is derived from `dirty_seq` and only this whitelist
        // bumps it. Assigning a status to an ITEM is already covered: that writes through
        // `BinderItem(Updated)` above.
        BinderStatus(EntityEvent::Created),
        BinderStatus(EntityEvent::Updated),
        BinderStatus(EntityEvent::Removed),
        DictWord(EntityEvent::Created),
        DictWord(EntityEvent::Updated),
        DictWord(EntityEvent::Removed),
        Comment(EntityEvent::Created),
        Comment(EntityEvent::Updated),
        Comment(EntityEvent::Removed),
        CommentReply(EntityEvent::Created),
        CommentReply(EntityEvent::Updated),
        CommentReply(EntityEvent::Removed),
        Footnote(EntityEvent::Created),
        Footnote(EntityEvent::Updated),
        Footnote(EntityEvent::Removed),
        // The fourth kind to arrive missing, and the same failure each time: a preset
        // applied from Settings ▸ Work ▸ Templates created six rows, `unsaved` stayed
        // false, and Ctrl+W discarded them with no prompt. Templates are edited only
        // from the settings pane and from Document ▸ Save as template — never from a
        // manuscript editor — so like the three above, their events are the only signal
        // that an edit happened at all.
        //
        // Checked against the precondition this doc sets: the three write paths
        // (`WorkNoteTemplatesListModel::{create, update, remove_all}`) are reached only
        // from those two writer-driven surfaces plus the `NewWork` starter set; nothing
        // writes a template at flush or on open, so there is no unconditional write to
        // loop the debounce. `load_work`'s own rows are dispatched before this window's
        // `work_id` is seeded and are dropped by the guard below — see the note where
        // the status-ladder heal used to live in `wiring::project_events`.
        NoteTemplate(EntityEvent::Created),
        NoteTemplate(EntityEvent::Updated),
        NoteTemplate(EntityEvent::Removed),
    ] {
        v.push(Origin::DirectAccess(ent));
    }
    v
}

/// "Is this `DirectAccess` mutation event about *my* Work?" — the guard the
/// autosave `mutation_origins()` loop needs, and the harder half of "guard every
/// subscriber": unlike `LoadWork`/`NewWork`/`CloseWork`, none of these entity
/// kinds' events carry a `work_id` — only the changed entities' own ids. Answering
/// requires walking the relationship each entity actually has back to a `Work`:
///
/// * `Work(Updated)` — the entity id IS the work id; a direct comparison.
/// * `Binder`/`BinderTag`/`DictWord`/`Comment`/`Footnote` — each is a direct
///   `Work` one-to-many child (`qleany.yaml`'s `Work.binders`/`.tags`/
///   `.dict_words`/`.comments`/`.footnotes`); one relationship read answers it.
/// * `BinderItem` — one hop further (`Work` → `Binder` → `BinderItem`): first the
///   Work's own binder ids, then each binder's item ids.
/// * `CommentReply` — the same shape one door over (`Work` → `Comment` →
///   `CommentReply`).
///
/// Deliberately re-queried per event rather than cached: these are edits a hand
/// makes (create/rename/move/trash, a comment or footnote body committing on
/// change), not per-keystroke *prose* edits (`Content` events are excluded from
/// `mutation_origins` for exactly that reason), so even the busiest of them —
/// typing in a comment card, one `Comment(Updated)` per committed change — costs
/// a handful of in-memory relationship reads, nothing an autosave-timer debounce
/// would notice.
///
/// A `Removed` event can legitimately fail this walk: the generated cascade
/// reconciles the owner's junction in the same transaction, so by the time the
/// handler runs the removed id is no longer in the owner's list. That is fine —
/// the same commit emits the owner's own `Updated` event (`Work(Updated)` for a
/// removed `Comment`, `Comment(Updated)` for a removed reply), and *that* one
/// passes. The walk only has to never claim a **sibling** Work's mutation.
///
/// An event with no ids at all (shouldn't happen for these kinds, but no
/// generated event type guarantees it) is treated as in-scope — swallowing a
/// mutation this Work's autosave should have reacted to is worse than an
/// occasional spurious wake.
fn mutation_ids_belong_to_work(
    ctx: &AppContext,
    my_work_id: u64,
    entity: DirectAccessEntity,
    event_ids: &[u64],
) -> bool {
    if event_ids.is_empty() {
        return true; // unattributed — safer to assume it's mine than to swallow it
    }
    match entity {
        DirectAccessEntity::Work(_) => event_ids.contains(&my_work_id),
        DirectAccessEntity::Binder(_) => {
            let mine = work_commands::get_work_relationship(
                ctx,
                &my_work_id,
                &WorkRelationshipField::Binders,
            )
            .unwrap_or_default();
            event_ids.iter().any(|id| mine.contains(id))
        }
        DirectAccessEntity::BinderTag(_) => {
            let mine = work_commands::get_work_relationship(
                ctx,
                &my_work_id,
                &WorkRelationshipField::Tags,
            )
            .unwrap_or_default();
            event_ids.iter().any(|id| mine.contains(id))
        }
        // A rung is a direct `Work` child, so one relationship read answers it — the same
        // shape `BinderTag` above has. Without this arm the fallback below would decide,
        // and a status edit in one open project would mark every other open project dirty.
        DirectAccessEntity::BinderStatus(_) => {
            let mine = work_commands::get_work_relationship(
                ctx,
                &my_work_id,
                &WorkRelationshipField::Statuses,
            )
            .unwrap_or_default();
            event_ids.iter().any(|id| mine.contains(id))
        }
        DirectAccessEntity::DictWord(_) => {
            let mine = work_commands::get_work_relationship(
                ctx,
                &my_work_id,
                &WorkRelationshipField::DictWords,
            )
            .unwrap_or_default();
            event_ids.iter().any(|id| mine.contains(id))
        }
        // Same shape again, and needed for the same reason `BinderStatus` states: without
        // this arm the `_ => true` fallback would decide, and applying a template preset
        // in one open project would mark every other open project dirty.
        DirectAccessEntity::NoteTemplate(_) => {
            let mine = work_commands::get_work_relationship(
                ctx,
                &my_work_id,
                &WorkRelationshipField::NoteTemplates,
            )
            .unwrap_or_default();
            event_ids.iter().any(|id| mine.contains(id))
        }
        DirectAccessEntity::BinderItem(_) => {
            let my_binders = work_commands::get_work_relationship(
                ctx,
                &my_work_id,
                &WorkRelationshipField::Binders,
            )
            .unwrap_or_default();
            my_binders.iter().any(|binder_id| {
                let items = binder_commands::get_binder_relationship(
                    ctx,
                    binder_id,
                    &BinderRelationshipField::BinderItems,
                )
                .unwrap_or_default();
                event_ids.iter().any(|id| items.contains(id))
            })
        }
        DirectAccessEntity::Comment(_) => {
            let mine = work_commands::get_work_relationship(
                ctx,
                &my_work_id,
                &WorkRelationshipField::Comments,
            )
            .unwrap_or_default();
            event_ids.iter().any(|id| mine.contains(id))
        }
        DirectAccessEntity::CommentReply(_) => {
            let my_comments = work_commands::get_work_relationship(
                ctx,
                &my_work_id,
                &WorkRelationshipField::Comments,
            )
            .unwrap_or_default();
            my_comments.iter().any(|comment_id| {
                let replies = comment_commands::get_comment_relationship(
                    ctx,
                    comment_id,
                    &CommentRelationshipField::Replies,
                )
                .unwrap_or_default();
                event_ids.iter().any(|id| replies.contains(id))
            })
        }
        DirectAccessEntity::Footnote(_) => {
            let mine = work_commands::get_work_relationship(
                ctx,
                &my_work_id,
                &WorkRelationshipField::Footnotes,
            )
            .unwrap_or_default();
            event_ids.iter().any(|id| mine.contains(id))
        }
        _ => true, // not one of `mutation_origins`'s kinds — never reached
    }
}

/// "Is there anything to save right now?" — the single truth the Save affordances
/// (menu item, `editor.save` action, Ctrl+S) all gate on.
///
/// True iff the work has edits not yet on disk **and** this window is not in backup
/// mode. Backup mode is excluded because [`EditorsViewModel::save_to_disk`] is inert
/// there (the backup file is read-only; edits leave only through Save As / Restore),
/// so a Save command would be a silent no-op — better to show it disabled.
///
/// Derived, so it stays reactive: it recomputes whenever either input changes.
pub fn can_save(unsaved: &Signal<bool>, backup_mode: &Signal<bool>) -> Signal<bool> {
    unsaved.and(&backup_mode.not())
}

/// Build this Work's [`StackTeardown`] — run either once
/// `WorkRegistry::remove_window` confirms this was the *last* window on the
/// Work (a real close, driven by teksilo's `on_removed` hook — see
/// `shell::windows`'s wiring), or the instant this same window's own
/// `register_window` call supersedes its own previous binding (an in-place
/// File > New Work / Open Work / ProjectSwitcher "Open here", which never
/// destroys the window, so `on_removed` never fires for the Work being left —
/// see `WorkRegistry::register_window`'s doc for why that path exists).
///
/// `stack_id` is a **snapshot**, not a live read of `AppIds`: by the time this
/// runs on a real close, the window's own `CloseWork` subscriber has already
/// called `ids.clear()` (see `ProjectLifecycleViewModel::on_close`), so
/// reading `ids.stack_id` live here would always see `None`. It must be
/// captured at registration time, right after `AppIds::open_stack` minted it.
///
/// `is_last` — whether this was the last window on this Work, decided by
/// [`WorkRegistry`] itself, never `WindowRemovedEvent::remaining_windows`
/// (which counts every window in the process, Launcher included).
pub(crate) fn build_stack_teardown(
    app_ctx: Rc<AppContext>,
    stack_id: Option<u64>,
) -> StackTeardown {
    Rc::new(move |is_last: bool| {
        if is_last && let Some(stack_id) = stack_id {
            let _ = undo_redo_commands::delete_stack(&app_ctx, stack_id);
        }
    })
}

/// Build this window's [`WindowTeardown`] — what `WorkRegistry::remove_window`
/// runs once teksilo's `on_removed` hook confirms this window is really gone
/// (see `shell::windows`'s wiring): release this window's own `OpenDoc` refs
/// and retire its backup flush-hook registration.
///
/// Unlike [`StackTeardown`], this must run **only** on a real close, never on
/// an in-place Load/New switching this window to a different Work: this
/// window's `EditorsViewModel` and backup flush hook both survive the switch
/// (the very same window keeps showing tabs, and keeps needing its buffers
/// flushed, for whatever Work it shows next) — see
/// `WorkRegistry::register_window`'s doc.
///
/// **F3.** Also tells the toast registry to forget this window
/// (`ToastRegistry::forget_window`) — `set_window_audience(id, None)` alone
/// only clears the audience *signal's value*, leaving the map entry (and its
/// `Signal` allocation) alive in `window_audiences`/`window_versions`
/// forever, so every window ever opened over a session would otherwise leak
/// one entry in each map. `None` only in a headless/off-screen build context
/// (no `install_toast_default()` ever ran) — a safe no-op there.
///
/// **And, on the last window standing, the project's own close-out.**
/// [`ProjectLifecycleViewModel::on_close`](crate::project::ProjectLifecycleViewModel::on_close)
/// is subscribed to `CloseWork` with `subscribe_event_with_ctx`, and that event
/// is only *queued* by `close_work`: every path that closes a project
/// force-closes its windows in the same dispatch, so by the time the event is
/// delivered there is no live tree to build an `EventContext` from and the
/// handler is skipped. (The plain `subscribe_event` handlers on the same event
/// still run, which is why this went unnoticed.) The close-out therefore never
/// happened on a real close — leaving, most visibly, the **open-registry claim
/// held**: a project closed back to the Launcher stayed advertised as open to
/// every peer instance, which would then offer to raise a window that no longer
/// exists. It also left this Work's `SpellcheckService` personal-word/mute entry
/// resident and its ids un-cleared.
///
/// Here instead, where it is guaranteed to run — a window really being gone is
/// exactly what `on_removed` reports — and to run **once per Work**: `is_last`
/// is [`WorkRegistry`]'s own refcount answer, so with several windows on one
/// Work only the last one performs it. The in-place-switch path is untouched
/// and still reaches `on_close` through the subscriber, because there the
/// window survives; this teardown does not run at all for it (see
/// [`WindowTeardown`]).
#[allow(clippy::too_many_arguments)]
pub(crate) fn build_window_teardown(
    editors: EditorsViewModel,
    backup_scheduler: BackupSchedulerViewModel,
    toast_registry: Option<ToastRegistry>,
    window_id: TeksiloWindowId,
    stack_id: Option<u64>,
    close_out: ProjectCloseOut,
) -> WindowTeardown {
    Rc::new(move |is_last: bool| {
        editors.release_own_open_docs(stack_id);
        backup_scheduler.unregister_flush_hook(window_id);
        if let Some(reg) = &toast_registry {
            reg.forget_window(window_id);
        }
        // Ordered after the release above, which reads this window's tab list —
        // the close-out empties it.
        if is_last {
            close_out();
        }
    })
}

/// The project's own close-out, as one callable — what
/// [`build_window_teardown`] runs on the last window standing.
///
/// A closure rather than the [`ProjectLifecycleViewModel`](crate::project::ProjectLifecycleViewModel)
/// itself so the seam can be exercised for what it is: "does the teardown call
/// this exactly when it is the last window, and never otherwise" is a question
/// about the teardown, and answering it should not require standing up an
/// outline, a trash dock and a spell-checker first.
pub(crate) type ProjectCloseOut = Rc<dyn Fn()>;

/// The real close-out: `ProjectLifecycleViewModel::on_close`, the handler that
/// `CloseWork` can no longer deliver on a real close (see
/// [`build_window_teardown`]'s doc for why).
pub(crate) fn build_project_close_out(
    lifecycle: crate::project::ProjectLifecycleViewModel,
) -> ProjectCloseOut {
    Rc::new(move || lifecycle.on_close())
}

/// Present the shared Export modal over an already-`prepare`d [`ExportViewModel`]. Both the
/// `export.scope` (quick scope) and `work.export` (Choose…) actions funnel through here so
/// the modal chrome stays identical.
fn present_export_panel(ctx: &mut EventContext, vm: ExportViewModel) {
    ctx.present_modal(
        ModalRequest::deferred(move |t| t.add(ExportPanel::new(vm)))
            .presentation(ModalPresentation::InTree)
            .title("Export")
            .close_behavior(ModalCloseBehavior::EscapeOrClickOutside)
            .size(760, 640),
    );
}

impl std::fmt::Debug for App {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("App").finish()
    }
}

impl Widget for App {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        let view_model_setup::LayerBViewModels {
            settings,
            window_id,
            session,
            save_state,
            editors,
            workspace_layout,
        } = self.build_layer_b_view_models(ctx);

        let wiring::spellcheck::SpellcheckWiring {
            spell_docs,
            spellcheck,
            dictionaries,
            dictionary_settings_reloadable,
        } = wiring::spellcheck::wire(
            ctx,
            &self.app_ctx,
            &session.ids,
            &session.open_docs,
            self.dictionary_settings_reloadable.is_some(),
        );
        if let Some(r) = dictionary_settings_reloadable {
            self.dictionary_settings_reloadable = Some(r);
        }
        // Live cross-process reload for `export_styles.toml` (user styles), pulled from
        // app-state since `App` doesn't own the view-model.
        if self.export_styles_reloadable.is_none()
            && let Some(registry) = ctx.app_state::<SettingsRegistry>().cloned()
            && let Some(styles) = ctx
                .app_state::<crate::export::ExportStylesViewModel>()
                .cloned()
        {
            self.export_styles_reloadable = Some(registry.register(styles.settings_reloadable()));
        }

        // "Unsaved" is *derived*: the work has edits not on disk iff more mutations
        // have happened than the last completed save covered. Recomputed whenever
        // either side moves — a mutation (typing, tree edit) or a save landing.
        // Both sequences are read off the shared `SaveStateViewModel`, not owned
        // here — every window derives the identical `unsaved` from the identical
        // pair.
        {
            let unsaved = self.unsaved.clone();
            let dirty_seq = save_state.dirty_seq();
            let saved_seq = save_state.saved_seq();
            let recompute = Rc::new(move || {
                let is_unsaved = dirty_seq.get() > saved_seq.get();
                if unsaved.get() != is_unsaved {
                    unsaved.set(is_unsaved);
                }
            });
            {
                let r = recompute.clone();
                ctx.effect(&save_state.dirty_seq(), move |_| r());
            }
            ctx.effect(&save_state.saved_seq(), move |_| recompute());
        }

        // Export: keep the focus-adaptive quick-scope list in step with the focused editor
        // item, so the title-bar Export split-button and the File ▸ Export submenu both
        // re-derive whenever the selection changes. (A reactive trigger can't dispatch an
        // intent, but it can query + set a signal — which is all this does.)
        //
        // `self.export`, not `ctx.app_state::<ExportViewModel>()` — see the field's doc.
        {
            let export_vm = self.export.clone();
            let active = editors.active_item();
            export_vm.recompute_applicable(active.get()); // seed for the current focus
            ctx.effect(&active, move |id| export_vm.recompute_applicable(*id));
        }

        let outline = self.outline.clone();

        let wiring::shared_view_models::SharedFeatureViewModels {
            search,
            trash,
            comments,
        } = wiring::shared_view_models::get_or_create(
            &self.app_ctx,
            &self.outline,
            &session.open_docs,
            self.search_dock,
            self.preview_dock,
            self.trash_dock,
            &mut self.search,
            &mut self.trash,
            &mut self.comments,
        );
        let ids = self.outline.ids();
        let docs = session.open_docs.clone();
        let footnotes =
            wiring::footnotes::install(ctx, &mut self.footnotes, &self.app_ctx, &ids, &docs);

        // Record every marked copy that leaves. Tier 1 service, per-project rows
        // keyed by the payload's own `Work.unique_id` — the title is display
        // only, so a window whose project has not loaded yet simply records an
        // empty one and the next export fills it in.
        // Cloned out of the borrow first: `app_state` hands back a reference tied
        // to `ctx`, and `wire` needs `ctx` mutably.
        let exchange = ctx.app_state::<crate::models::ExchangeService>().cloned();
        if let Some(exchange) = exchange {
            wiring::exchange::wire(ctx, &exchange, session.single_work.title());
        }

        // Who new threads and replies get signed by: Settings ▸ User if it is
        // filled in, else this book's byline (see `comments::signature`).
        //
        // An **effect over all three sources**, not the one-shot
        // `set_default_author(&…author_name().get())` this replaced. That read
        // registered no dependency — `Signal::get` is a plain clone — and ran
        // during the first `App::build`, which happens before `load_work` has
        // populated `SingleWork`. The captured value was therefore `""` for the
        // life of the window no matter what the project or the settings said, and
        // every comment reached disk with `author_name: ""`. `ctx.effect` fires
        // once with the current value and again on every change, so it covers
        // both the seed and a name typed into Settings mid-session.
        {
            let user_name = settings.user_name();
            let user_initials = settings.user_initials();
            let work_author = session.single_work.author_name();
            let recompute = {
                let comments = comments.clone();
                let user_name = user_name.clone();
                let user_initials = user_initials.clone();
                let work_author = work_author.clone();
                move || {
                    comments.set_signature(crate::comments::signature::resolve(
                        &user_name.get(),
                        &user_initials.get(),
                        &work_author.get(),
                    ));
                }
            };
            {
                let r = recompute.clone();
                ctx.effect(&user_name, move |_: &String| r());
            }
            {
                let r = recompute.clone();
                ctx.effect(&user_initials, move |_: &String| r());
            }
            ctx.effect(&work_author, move |_: &String| recompute());
        }
        comments.model().wire(ctx);
        // Hand the view-model to the open-document store so every document — the
        // ones already open from a workspace restore, and every one opened later —
        // gets its highlight layer seeded from the persisted anchors.
        session.open_docs.set_comments(comments.clone());
        // The comment palette follows the theme: its light-page values are a glare
        // on a dark one. Re-resolved on every build, which is when a theme change
        // lands (the same cadence the spell squiggle's colour uses).
        comments.set_dark(ctx.theme().is_dark());

        // Live cross-process reload for `search.toml` — register this window's
        // service into the shared `SettingsRegistry` once (same story as
        // `backup_settings` above).
        if self.search_settings_reloadable.is_none()
            && let Some(registry) = ctx.app_state::<SettingsRegistry>().cloned()
        {
            self.search_settings_reloadable = Some(registry.register(search.settings_reloadable()));
        }
        // Re-seed the search inputs from the opened project's saved preferences,
        // and drop any preview held for the previous project. Guarded: with a
        // second Work open in a second window, its Load/New/Close must not reset
        // or wipe *this* window's own live search box/preview.
        {
            let s = search.clone();
            wiring::project_events::on_own_load_or_new(ctx, &session.ids, move |_e| {
                s.restore_for_project();
            });
            let s = search.clone();
            wiring::project_events::on_own_close(ctx, &session.ids, move |_e| {
                s.clear_preview();
            });
        }

        // Forget this window's distraction-free state on Close-Work/Load-Work
        // — exactly like `AppIds` (see `FocusViewModel`'s module doc).
        {
            let focus = self.focus.clone();
            wiring::project_events::on_own_load_or_new(ctx, &session.ids, move |_e| {
                focus.reset();
            });
            let focus = self.focus.clone();
            wiring::project_events::on_own_close(ctx, &session.ids, move |_e| {
                focus.reset();
            });
        }

        // Persist live theme / interface-language changes into the keys the
        // startup restore reads. The Settings window drives these via the
        // framework's `ThemeSwitcher` / `LanguageSwitcher`, which apply the change
        // live (`EventContext::set_theme` / `set_locale`) but do not themselves
        // persist; these effects mirror the live value into `DARK_KEY` /
        // `LOCALE_KEY` so it restores next launch (see `main::read_prefs`).
        {
            let dark = settings.dark();
            // Mirrored beside `ui.dark`, not instead of it: `ui.dark` is what an
            // install written before `ui.theme_mode` existed still boots from, so
            // it has to keep tracking the live theme for ever. The *mode* is the
            // finer answer — it is the only one that can say "follow the desktop"
            // — and without this half it would be written solely by an explicit
            // `set_theme_mode`, so a pick made in the ThemeSwitcher would come
            // back next launch as a frozen light/dark rather than the mode it was.
            //
            // The signal is fetched inside the closure, never at build time:
            // `store.signal()` seeds a missing key, and seeding this one on every
            // launch would hand the desktop a vote nobody cast (see
            // `SettingsViewModel::theme_mode`). Here it is only ever reached with
            // a theme in hand, which is the moment the key is about to be written.
            let mode_owner = settings.clone();
            let theme_sig = ctx.theme_signal().clone();
            ctx.effect(&theme_sig, move |t| {
                let is_dark = t.is_dark();
                if dark.get() != is_dark {
                    dark.set(is_dark);
                }
                let mode = mode_owner.theme_mode();
                let m = crate::settings_keys::theme_mode_of(t);
                if mode.get() != m {
                    mode.set(m.to_string());
                }
            });
        }
        if let Some(locale_sig) = teksilo::i18n::current_locale() {
            let persisted = settings.locale();
            ctx.effect(&locale_sig, move |l| {
                let tag = l.to_string();
                if persisted.get() != tag {
                    persisted.set(tag);
                }
            });
        }

        // The framework's toast registry — used below (LoadWork/NewWork) to bind
        // this window's toast/bell audience to whatever Work it is showing (see
        // `crate::toast_scope`'s module doc). `None` only in a headless/off-screen
        // build context where `install_toast_default()` was never called; every
        // real window has it (`main.rs`'s builder chain installs it once, up
        // front, before any window exists).
        let toast_registry = ctx.app_state::<ToastRegistry>().cloned();

        // ── Layer-A singles: id-only per-Work state + reactive entity handles ──
        // Held on `session` (created in `main`, shared with every window onto this
        // Work). `wire` installs each single's event subscriptions on this
        // (process-lifetime) widget; they are re-pointed on `LoadWork` below.
        let ids = session.ids.clone();
        let single_work = session.single_work.clone();
        let statuses = session.statuses.clone();
        let single_work_info = session.single_work_info.clone();
        // Resolved off THIS window's own `session`, same as every other
        // Layer-A single above — a second, simultaneously-open Work always
        // gets its own punctuation handle (see `WorkSession::smart_punctuation`'s
        // own doc).
        let smart_punctuation = session.smart_punctuation.clone();
        single_work.wire(ctx);
        single_work_info.wire(ctx);
        smart_punctuation.wire(ctx);

        wiring::punctuation::install(
            ctx,
            &ids,
            &single_work,
            &smart_punctuation,
            &spell_docs,
            &settings,
        );

        // The project-lifecycle view-model: the shared Load/New/Close sequence, which was
        // three hand-kept-in-step closures here. Built fresh each build — it holds only
        // clones of handles that are themselves stable across builds, so re-creating it is
        // idempotent; the subscribers below capture their own clone.
        let lifecycle = crate::project::ProjectLifecycleViewModel::new(
            self.app_ctx.clone(),
            ids.clone(),
            outline.clone(),
            editors.clone(),
            trash.clone(),
            single_work.clone(),
            single_work_info.clone(),
            spell_docs.clone(),
            spellcheck.clone(),
            save_state.clone(),
            self.backup_mode.clone(),
            self.backup_context.clone(),
            workspace_layout.clone(),
        );
        // The custom replacement lexicon — one wired instance behind the Settings pane
        // and the editor's typing session — resolved off `session`, same as
        // `smart_punctuation` above.
        //
        // Tags and the personal dictionary are wired *after* the LoadWork seed below:
        // their Load/New handlers re-read Work-scoped relationships, and must not race
        // the lifecycle seed that writes `ids.work_id` (see WorkTagsListModel::wire).
        session.text_replacements.wire(ctx);
        // Backup scheduler (on `session`) + settings (registered in `main`). The
        // scheduler drives every trigger and holds the singles; the settings VM
        // tracks the active project for the per-project settings pane.
        let backup_scheduler = session.backup_scheduler.clone();
        let backup_settings = ctx
            .app_state::<BackupSettingsViewModel>()
            .cloned()
            .expect("BackupSettingsViewModel registered in main");
        // `self.restore_vm`/`self.save_as_vm`, not `ctx.app_state::<T>()` — see
        // those fields' doc: each window now gets its own, bound to its own
        // `ids`/`single_work`.
        let restore_vm = self.restore_vm.clone();
        let save_as_vm = self.save_as_vm.clone();
        // Live cross-process reload for `backup.toml` (T1-6 continued): register
        // this window's `BackupSettingsService` into the app's shared
        // `SettingsRegistry` once, so the settings-file watcher (installed by
        // `TeksiloAppBuilder` whenever a settings bundle is configured — see
        // `main.rs`) reloads it in place the moment a peer window — another
        // project's window in this process, or a `--new-instance` process —
        // writes an override / policy / bookkeeping change. Without this,
        // `backup_settings`'s reads would only ever see this process's own
        // last write. Absent registry (e.g. a headless/test build with no
        // settings bundle) is a silent no-op: the service still works, peer
        // writes just aren't picked up live until this process next writes
        // something itself.
        if self.backup_settings_reloadable.is_none()
            && let Some(registry) = ctx.app_state::<SettingsRegistry>().cloned()
        {
            self.backup_settings_reloadable =
                Some(registry.register(backup_settings.service().as_reloadable()));
        }
        // T1-2: install the real flush hook — every backup trigger
        // (`backup_now` / `on_open` / `interval_tick` / `on_close_flow`) flushes
        // the live editor buffers into the store *before* it reads it, so a
        // long typing session in an unfocused tab is never lost to
        // skip-if-unchanged. Registered per-`WindowId` (multi-Work migration):
        // the scheduler's hook map is shared across every clone already handed
        // out (the window close guard in `windows.rs`, `App`'s own exit-guard
        // effect below), so registering here is visible everywhere at once —
        // but keyed, so a Work with two windows flushes *both* rather than the
        // last one registered silently displacing the first (see
        // `BackupSchedulerViewModel::register_flush_hook`'s doc). `window_id`
        // is `None` only in a headless/off-screen build context (never a real
        // project window), in which case there is nothing to key the hook on.
        if let Some(window_id) = window_id {
            backup_scheduler.register_flush_hook(
                window_id,
                Rc::new({
                    let editors = editors.clone();
                    move || editors.flush_all()
                }),
            );
        }
        // The same invariant for the *other* two paths that serialize the store on
        // a background thread: Save As and Restore. Typing only marks a doc dirty
        // (`OpenDoc::mark_dirty_fn`) — the prose reaches `Content` only on a flush
        // — so a read-only background op that starts without one writes the
        // pre-edit text. `save_work` gets this via `EditorsViewModel::save_to_disk`
        // and the backups via the hook above; these two had no flush at all, which
        // is worst in backup mode, where Save is off and Save As / Restore are the
        // only ways the edits can be kept at all.
        save_as_vm.set_flush_hook(Rc::new({
            let editors = editors.clone();
            move || editors.flush_all()
        }));
        restore_vm.set_flush_hook(Rc::new({
            let editors = editors.clone();
            move || editors.flush_all()
        }));
        // The project-switch guard (New Work / Open Work / "Open here" / the import
        // toast — every command that replaces this window's project in place) needs
        // two things only the view layer has: a way to write the project to disk
        // (returning the op id, so a *failed* save drops the parked switch instead
        // of stranding it), and a way to put the New Work form on screen. Installed
        // here, once — the guard itself is created in `main` (it holds the same
        // `unsaved`/`backup_mode`/autosave signals as the close guard) and reached
        // from outside `App` via app-state.
        //
        // Per-window `ProjectSwitchViewModel` (minted in the factory with this
        // window's own `unsaved`/`backup_mode`). Only the desk owner installs
        // hooks — see [`WindowRole::installs_project_switch_hooks`].
        // Built here rather than beside `cold_start_import` below, because the New Work
        // form hook installed just under this line has to capture it: the in-place
        // creation path arms it from inside the wizard, and the arm has to reach the
        // same window's own one-shot.
        let pending_starters = wiring::project_events::PendingStarters::default();
        let project_switch = self.project_switch.clone();
        if self.role.installs_project_switch_hooks() {
            project_switch.set_save_hook(Rc::new({
                let editors = editors.clone();
                move || editors.request_save()
            }));
            project_switch.set_new_work_form_hook(Rc::new({
                let app_ctx = self.app_ctx.clone();
                let ids = ids.clone();
                let starters = pending_starters.clone();
                move |c: &mut EventContext| {
                    let app_ctx = app_ctx.clone();
                    let ids = ids.clone();
                    let starters = starters.clone();
                    c.present_modal(
                        ModalRequest::deferred(move |t| {
                            t.add(NewWorkPanel::new(app_ctx, ids, starters))
                        })
                        .presentation(ModalPresentation::InTree)
                        .title("New Work")
                        .close_behavior(ModalCloseBehavior::EscapeOrClickOutside)
                        .size(640, 620),
                    );
                }
            }));
        }
        // Keep the outline tree reactive to *all* structural mutations (incl. the
        // manuscript streams' rename/merge/split/add), not just the outline's own.
        self.outline.wire(ctx);
        // The trash model must be wired here too — at `App` (a stable root), not
        // lazily from its dock content — so it stays subscribed even while the
        // trash tab is a background rail tab and survives hide/reveal cycles.
        trash.wire(ctx);

        // ── The scriptable command surface ───────────────────────────────────
        // Every global action + shortcut, grouped by feature in `app::commands`. They are
        // pure registration — each closure clones what it needs out of `CommandDeps` — so
        // they lived here only by accident of history and made this function unreadable.
        let command_deps = commands::CommandDeps {
            app_ctx: self.app_ctx.clone(),
            ids: session.ids.clone(),
            role: self.role,
            session: session.clone(),
            registry: self.registry.clone(),
            quit: self.quit.clone(),
            outline: outline.clone(),
            format: self.format.clone(),
            undo_group: self.undo_group.clone(),
            fullscreen: self.fullscreen.clone(),
            focus: self.focus.clone(),
            editors: editors.clone(),
            comments: comments.clone(),
            trash: trash.clone(),
            search: search.clone(),
            project_switch: project_switch.clone(),
            dictionaries: dictionaries.clone(),
            spell_docs: spell_docs.clone(),
            user_dictionary: session.user_dictionary.clone(),
            export: self.export.clone(),
            import_document: self.import_document.clone(),
            search_dock: self.search_dock,
            trash_dock: self.trash_dock,
            footnotes_dock: self.footnotes_dock,
            timeline_dock: self.timeline_dock,
            unsaved: self.unsaved.clone(),
            backup_mode: self.backup_mode.clone(),
        };
        commands::register_all(ctx, &command_deps);

        // Anything an extension registered, onto **App's own** context, right
        // beside the app's own commands and for exactly that reason: a global
        // action belongs to the widget whose `build()` registered it, and is torn
        // down when that widget rebuilds or is destroyed. An extension has no
        // always-mounted widget of its own, so registering here — the window's
        // stable root — is the only way its shortcut keeps working once its panel
        // is closed. Its Tools-menu rows are assembled with the menu itself, in
        // `project_menus`; see `commands_ext`'s module docs.
        //
        // A snapshot, taken as this window builds — see `commands_ext::register_command`.
        //
        // Built **once** per build and shared with the dock roster below (through
        // `ShellParts::seam`): `ActiveContext` bridges three of this window's
        // signals by observation, so a second instance would be a second set of
        // observers doing identical work — and two objects obliged to agree about
        // the writer's focus with nothing making them.
        let seam = crate::docks::DockContext {
            app_ctx: self.app_ctx.clone(),
            ids: session.ids.clone(),
            // Tier 2: the Work's own save state, shared with every window on it,
            // so an extension's edit gates this project's close exactly like a
            // manuscript edit.
            work: session.save_state.handle(),
            // Tier 3: *this* window's focus. Built from this window's own
            // view-models, never `ctx.app_state` — a second window on the same
            // Work is looking somewhere else. Through `for_window` rather than
            // picking the signals here, so this call site cannot drift from what
            // `active_context`'s own test exercises.
            active: crate::active_context::ActiveContext::for_window(&editors, &self.outline),
            // Tier 2: **the Work's own** index, the same handle every tab is given.
            // Never `ctx.app_state::<MentionIndex>()` — that slot answers with the
            // bootstrap session's, which for an app started without a project is an
            // index bound to no Work and empty forever. See `DockContext` for the
            // whole account.
            mention_index: session.mention_index.clone(),
            // Tier 3 again, and for the same reason: *this* window's editors. The handle is
            // resolved per call and never held, because `RichTextEditor::construct` mints a
            // fresh one.
            live_prose: {
                let format = self.format.clone();
                Rc::new(move |item| {
                    // `document_view`, not `handle_for_item`: this reads the row's text
                    // and its version, both of which every view of that row shares, and a
                    // dock has no surface to name a scope with.
                    let handle = format.document_view(item, crate::format::EditorKind::Prose)?;
                    Some(crate::docks::LiveProse {
                        text: handle.to_plain_text(),
                        version: handle.document_version(),
                    })
                })
            },
        };
        crate::commands_ext::register_all_extension_commands(ctx, &seam);
        // …and anything an extension wants done with this context that is not a
        // verb. Same reason, same place, same re-run-per-build contract: an
        // extension's *state* has no always-mounted widget to hang on either, and
        // this is the only context from which a preference can be mirrored to
        // where its own save hook, on a worker thread, can read it.
        crate::app_wiring::run_all_wiring(ctx);

        // On project load/new/close/attach: lifecycle seed, backup sniff, dict offer.
        // Binder-item tab sync is not lifecycle — it rides every edit, not the boundaries —
        // so it is the editors' own `wire` (rebuild on retype, close on remove, re-caption
        // on anything that renumbers), subscribed here for this window's whole build.
        editors.wire(ctx);

        // The Launcher's "New from documents…" hand-off (see `ColdStartImport`).
        // Armed by the `PendingAction::New` arm at the end of this build, taken
        // by the subscriber `install_lifecycle` registers right below.
        let cold_start_import = wiring::project_events::ColdStartImport::default();
        let attach_seed = wiring::project_events::install_lifecycle(
            ctx,
            wiring::project_events::LifecycleDeps {
                app_ctx: self.app_ctx.clone(),
                session: session.clone(),
                undo_group: self.undo_group.clone(),
                ids: session.ids.clone(),
                registry: self.registry.clone(),
                lifecycle: lifecycle.clone(),
                editors: editors.clone(),
                outline: outline.clone(),
                trash: trash.clone(),
                search: search.clone(),
                backup_scheduler: backup_scheduler.clone(),
                toast_registry: toast_registry.clone(),
                window_id,
                window_ordinal: self.window_ordinal.clone(),
                spell_docs: spell_docs.clone(),
                dictionaries: dictionaries.clone(),
                tree_expansion: session.tree_expansion.clone(),
                backup_mode: self.backup_mode.clone(),
                backup_context: self.backup_context.clone(),
                restore_vm: restore_vm.clone(),
                single_work: single_work.clone(),
                backup_settings: backup_settings.clone(),
                workspace_layout: workspace_layout.clone(),
                trash_dock: self.trash_dock,
                import_document: self.import_document.clone(),
                cold_start_import: cold_start_import.clone(),
                starters: pending_starters.clone(),
            },
        );

        // Tag palette + personal dictionary: wire **after** the LoadWork seed so their
        // project-boundary handlers see a seeded `work_id` when possible. They also fall
        // back to the event's work id (see each model's wire docs) if registration order
        // ever races seed again.
        session.user_dictionary.wire(ctx);
        session.tags.wire(ctx);
        // Same window, same reasons: without this the note-template list never subscribes
        // to anything, so it holds whatever `load_rows` returned at session-construction
        // time — which is nothing, because `work_id` is not seeded yet — and applying a
        // preset writes rows the pane never hears about.
        session.note_templates.wire(ctx);
        // Refill Document ▸ Insert template whenever the catalogue changes.
        //
        // The submenu's own contents cannot be produced where the menu is declared: that
        // builder is `FnOnce` and runs at window construction, before a project exists.
        // The framework's answer (teksilo/docs/native-menu.md, "Dynamic structure") is a
        // pre-allocated submenu id plus runtime mutation, which is what this drives — each
        // refill bumps `MenuModel::version`, and the bar re-derives its dropdowns from it.
        // The Image menu appears while a picture is selected and goes away
        // when it is not. Driven off the same signal the Document menu's image
        // rows used to gate on, so the menu and the commands cannot disagree
        // about whether there is an image in hand.
        let on_open = wiring::focus_sync::install(
            ctx,
            wiring::focus_sync::FocusSyncDeps {
                templates_menu: self.templates_menu.clone(),
                image_menu_id: self.image_menu_id,
                active_image: self.format.active_image(),
                note_templates: session.note_templates.clone(),
                has_target: self.format.has_target(),
                save_as_vm: save_as_vm.clone(),
                backup_scheduler: backup_scheduler.clone(),
                restore_vm: restore_vm.clone(),
                export_vm: self.export.clone(),
                import_document: self.import_document.clone(),
                mention_index: self.session.mention_index.clone(),
                progress_recorder: self.session.progress_recorder.clone(),
                binder_has_selection: self.binder_has_selection.clone(),
                outline_selection: self.outline.selection_signal(),
                editors: editors.clone(),
            },
        );

        // ── Autosave ─────────────────────────────────────────────────────────
        // Mirror the persisted setting into the menu's plain signal (the title-bar
        // menu lives outside `App` and can't read `ctx.settings()`).
        {
            self.autosave_menu.set(settings.autosave().get());
            let menu = self.autosave_menu.clone();
            ctx.effect(&settings.autosave(), move |a| menu.set(*a));
        }
        // ── The master spell-check switch ────────────────────────────────────
        // One setting, four surfaces (title-bar toggle, View ▸ Check spelling, F7,
        // Settings ▸ Spelling) — they all write `SPELLCHECK_ENABLED_KEY`, and this effect is
        // the only place that reads it into the engine. `set_enabled` gates `build_checker`,
        // so `attach_all` then rebuilds every open document through the existing degrade
        // path: off clears the squiggles, on paints them back.
        //
        // The *toast* for "you turned it on but have no dictionary" cannot live here —
        // `ctx.effect` gets no `EventContext` — so it hangs off the `spellcheck.toggle`
        // action below, which does. That covers the title bar, the menu and F7; the
        // Settings ▸ Spelling row writes the signal directly and shows no toast, which is
        // tolerable precisely there: Dictionaries is the next page down the same tree.
        {
            self.spellcheck_menu
                .set(settings.spellcheck_enabled().get());
            let menu = self.spellcheck_menu.clone();
            let spell = spellcheck.clone();
            let docs = spell_docs.clone();
            // Seed the engine before the first attach, or a launch with the switch off
            // would paint one frame of squiggles before the effect caught up.
            spell.set_enabled(settings.spellcheck_enabled().get());
            ctx.effect(&settings.spellcheck_enabled(), move |on| {
                menu.set(*on);
                // Not a real flip → don't re-attach every open document for nothing.
                if spell.set_enabled(*on) {
                    docs.attach_all();
                }
            });
        }
        wiring::autosave::install(
            ctx,
            &wiring::autosave::AutosaveDeps {
                comments_menu: self.comments_menu.clone(),
                margin_lane_menu: self.margin_lane_menu.clone(),
                margin_lane_enabled: ctx.settings().signal(
                    crate::MARGIN_LANE_ENABLED_KEY,
                    crate::MARGIN_LANE_ENABLED_DEFAULT,
                ),
                comments_visible: settings.comments_visible(),
                spell_docs: spell_docs.clone(),
                editors: editors.clone(),
                save_state: save_state.clone(),
                autosave: settings.autosave(),
                app_ctx: self.app_ctx.clone(),
                ids: session.ids.clone(),
                backup_scheduler: backup_scheduler.clone(),
            },
        );

        // ── Exit guards (Close Work / Quit / window close) ───────────────────
        wiring::save_and_exit::install(
            ctx,
            &wiring::save_and_exit::SaveAndExitDeps {
                editors: editors.clone(),
                pending_exit: self.pending_exit.clone(),
                exit_seq: self.exit_seq.clone(),
                backup_scheduler: backup_scheduler.clone(),
                project_switch: project_switch.clone(),
                workspace_layout: workspace_layout.clone(),
                ids: ids.clone(),
                quit: self.quit.clone(),
                save_state: save_state.clone(),
            },
        );

        // `work.close` — the Close Work menu command (Ctrl+W), and the target of the
        // `welcome.show` alias. Every terminal path ends at
        // [`close_work_and_return_to_launcher`] — there is no more "close in place,
        // keep this (now-empty) window" outcome.
        //
        // Delegates to [`guard_unsaved_exit`] — the same guard `app.quit`'s action
        // and the project window's `on_close_requested` (`windows.rs`) call — rather
        // than re-deriving the branch order: what happens to your unsaved chapter
        // must not depend on which command is about to discard it. Only the
        // *outcome* differs — this one leaves for the Launcher instead of quitting.
        {
            let app_ctx2 = self.app_ctx.clone();
            let my_ids = session.ids.clone();
            let unsaved = self.unsaved.clone();
            let autosave = settings.autosave();
            let pending = self.pending_exit.clone();
            let scheduler = backup_scheduler.clone();
            let backup_mode = self.backup_mode.clone();
            ctx.register_action_global(Action::new("work.close").on_invoke(move |_i, ctx| {
                guard_unsaved_exit(
                    ctx,
                    &app_ctx2,
                    &my_ids,
                    unsaved.get(),
                    backup_mode.get(),
                    autosave.get(),
                    &pending,
                    &scheduler,
                    PendingExit::ReturnToLauncher,
                );
            }));
        }
        // `backup.now` — the manual "Back up now" command. Runs a forced backup
        // to the configured destinations; `backup_now` itself flushes in-widget
        // edits into the store first (T1-2 — the flush hook installed above, at
        // the top of every trigger, not just this one). Registered globally (the
        // title-bar menu is an overlay).
        {
            let scheduler = backup_scheduler.clone();
            ctx.register_action_global(Action::new("backup.now").on_invoke(move |_i, ctx| {
                scheduler.backup_now(ctx);
            }));
        }
        // `statuses.completion` — "Where the book stands", on demand.
        //
        // The same readout the Pace summary shows on the way in, asked for rather than
        // offered: a writer with no deadline set never sees that card, and this is how
        // they reach the reading anyway. `register_action_global` and not
        // `register_action`, like every other app command — the title-bar menu renders in
        // an overlay that is a sibling of `App`, so a plain registration is never on the
        // intent's source-to-root path and the row would be dead.
        {
            let app_ctx = self.app_ctx.clone();
            let ids = ids.clone();
            let statuses = statuses.clone();
            ctx.register_action_global(Action::new("statuses.completion").on_invoke(
                move |_i, c| {
                    crate::statuses::panel::present(
                        c,
                        app_ctx.clone(),
                        ids.clone(),
                        statuses.clone(),
                    );
                },
            ));
        }
        // `pace.summary` — the writing-plan summary, on demand.
        //
        // The same card the load wiring greets a planned project with, opened because the
        // writer asked. Without this row the card had exactly one door — the opening of a
        // project — and its own "Do not show this when opening" checkbox bricked it: there
        // was no way back. The Work menu greys the row out (never hides it) when the
        // project has no active plan, off `session.pace_summary_available`.
        //
        // The setting signal is read here rather than threaded in so that the checkbox in
        // the card writes the same key whichever door opened it; `ctx.settings()` hands
        // back one shared store, so the two `signal(..)` calls are the same signal.
        {
            let app_ctx = self.app_ctx.clone();
            let ids = ids.clone();
            let statuses = statuses.clone();
            let editors = editors.clone();
            let show_on_open = ctx.settings().signal(crate::PACE_SUMMARY_ON_OPEN_KEY, true);
            ctx.register_action_global(Action::new("pace.summary").on_invoke(move |_i, c| {
                let editors = editors.clone();
                crate::pace::panel::present(
                    c,
                    app_ctx.clone(),
                    ids.clone(),
                    show_on_open.clone(),
                    statuses.clone(),
                    Rc::new(move |book_item_id, _c: &mut EventContext| {
                        // The card's one forward action, identical to the on-open path's:
                        // open the Book, where every number it showed can be edited. The
                        // empty title only seeds the tab caption — the editors model
                        // re-reads the item's own name as it opens.
                        editors.open_or_focus(book_item_id, "");
                    }),
                );
            }));
        }
        // `backups.show` — open the browsable list of this project's backup files.
        {
            let single_work = single_work.clone();
            let single_work_info = single_work_info.clone();
            let backup_settings = backup_settings.clone();
            let ids = ids.clone();
            ctx.register_action_global(Action::new("backups.show").on_invoke(move |_i, c| {
                let uid = single_work.unique_id().get();
                let Some(path) = single_work_info.file_name().get() else {
                    return;
                };
                let dirs = backup_settings.effective_for(&uid).destinations;
                // F2 — `ids.work_id` is the authoritative "current Work" id
                // (never `single_work.id()`): this is threaded into
                // `BackupsListPanel`/`BackupsListViewModel` for the
                // delete-failure toast's `scoped_id`/`target_work`, which
                // must route on the same source every other toast in this
                // window does.
                let work_id = ids.work_id.get();
                c.present_modal(
                    ModalRequest::deferred(move |t| {
                        t.add(crate::backup::list_panel::BackupsListPanel::new(
                            uid.clone(),
                            path.clone(),
                            dirs.clone(),
                            work_id,
                        ))
                    })
                    .presentation(ModalPresentation::InTree)
                    .title(tr!(backups_title()))
                    .size(700, 540)
                    .close_behavior(ModalCloseBehavior::EscapeOrClickOutside),
                );
            }));
        }
        // `app.about` — Help ▸ About. Global (not `register_action`) because the
        // title-bar menu renders in an overlay that is a *sibling* of `App`, so a
        // plain action would never be on the intent's source-widget → root path.
        // Fired by name only, so it needs no `AppIntent` variant.
        ctx.register_action_global(
            Action::new("app.about").on_invoke(|_i, c| crate::panels::about::present_about(c)),
        );
        let root = self.build_shell(
            ctx,
            project_shell::ShellParts {
                versions: self.versions.clone(),
                timeline: self.timeline.clone(),
                editors: editors.clone(),
                comments: comments.clone(),
                footnotes: footnotes.clone(),
                outline: outline.clone(),
                search: search.clone(),
                trash: trash.clone(),
                format: self.format.clone(),
                settings: settings.clone(),
                session: session.clone(),
                ids: ids.clone(),
                on_open: on_open.clone(),
                single_work: single_work.clone(),
                single_work_info: single_work_info.clone(),
                restore_vm: restore_vm.clone(),
                save_as_vm: save_as_vm.clone(),
                seam,
            },
        );
        self.root_child = Some(root);

        // Perform this project window's one backend mutation (load the argv
        // path / a Launcher-picked recent, or create a brand-new work)
        // exactly once — now that the `LoadWork`/`NewWork` subscriptions above
        // are live, so the handler runs the full seed flow (ids, tree,
        // singles). There is no "show the Welcome modal" branch any more:
        // whether this process opens a Launcher or a project window at all is
        // decided in `main` before any window (hence any `App`) exists — see
        // `windows.rs` and the launcher-window model in `main.rs`'s module docs.
        if !self.initial_loaded {
            self.initial_loaded = true;
            match self.initial_action.take() {
                Some(PendingAction::Load(path)) => {
                    if let Err(e) = work_management_commands::load_work(
                        &self.app_ctx,
                        &LoadWorkDto {
                            media_root: crate::media_paths::media_root_string(),
                            file_name: path.clone(),
                        },
                    ) {
                        // A toast, not the `eprintln!` this used to be: an argv launch
                        // is the one open path with no dialog behind it, so a failure
                        // here left the writer looking at an empty window with the
                        // explanation on a terminal nobody is watching. `build` has only
                        // a `BuildContext`, which cannot present anything — but
                        // `run_after_mount` hands back a real `EventContext` once this
                        // window exists, which is exactly what a toast needs. The
                        // enclosing `initial_loaded` guard already makes this a genuine
                        // one-shot, so the per-enqueue caveat on `run_after_mount`
                        // (a rebuilding widget enqueuing twice) cannot apply.
                        let toast = crate::project::open_failure_toast(&path, &e);
                        eprintln!("skribisto: could not open '{path}': {e:#}");
                        ctx.run_after_mount(move |ctx| {
                            ctx.show_toast(toast);
                        });
                    }
                }
                Some(PendingAction::New {
                    dto,
                    then_import,
                    starters,
                }) => {
                    let target = dto.file_name.clone();
                    // Arm the cold-start import BEFORE creating the work: the
                    // `NewWork` subscriber that consumes it is already live, and
                    // arming afterwards would be a race whose losing side is
                    // silent (the wizard simply never opens).
                    if then_import {
                        cold_start_import.arm();
                    }
                    // Armed for the same reason and at the same moment: the
                    // subscriber that applies them is already live, and there is no
                    // palette or template list to write into until the call below has
                    // returned.
                    pending_starters.arm(starters);
                    if let Err(e) = work_management_commands::new_work(&self.app_ctx, &dto) {
                        eprintln!("skribisto: could not create '{target}': {e}");
                    }
                }
                // Work ▸ New Window — the one action with no backend call at
                // all. The Work is already loaded and this window's `AppIds`
                // already point at it (they are the sibling window's, shared);
                // what is missing is this window's own share of becoming live.
                // See `attach_seed`'s own comment for the per-Work/per-window
                // split it implements.
                Some(PendingAction::AttachExisting {
                    work_id,
                    ordinal,
                    open_item,
                    ..
                }) => attach_seed(work_id, ordinal, open_item),
                None => {}
            }
        }

        vec![root]
    }

    fn layout_response(&self, proposal: SizeProposal, ctx: &LayoutContext) -> LayoutResponse {
        self.root_child
            .and_then(|id| ctx.child_size(id, proposal))
            .map(LayoutResponse::from)
            .unwrap_or_else(|| proposal.resolve(0.0, 0.0).into())
    }

    fn place_children(
        &self,
        bounds: Rect,
        _proposal: SizeProposal,
        children: &mut [WidgetPlacement],
        _ctx: &LayoutContext,
    ) {
        for child in children.iter_mut() {
            child.origin = bounds.origin();
            child.size = bounds.size();
        }
    }

    fn children(&self) -> Vec<WidgetId> {
        self.root_child.into_iter().collect()
    }
}

/// Present the native picker for an existing `.skrib` and load it. Backs the
/// global `work.open` command (File ▸ Open Work… and Ctrl+O) — the most-used
/// file path in the app, so the backup sniff below (a blocking `File::open` +
/// zip parse with no timeout — see `crate::backup::is_backup_path`) must never
/// run on the UI thread: a recent entry on a disconnected network/FUSE mount
/// would otherwise hang the whole app on an ordinary click (T2-3).
///
/// Loading replaces this window's project **in place**, so the load itself goes
/// through the unsaved-changes guard (`ProjectSwitchViewModel`) rather than
/// straight to `load_work`, which would throw away the open project's unsaved
/// edits without a word. The guard runs *after* the pick and *after* the
/// backup sniff, so neither cancelling the picker nor choosing a backup (which
/// opens in its own window, leaving this project alone) prompts about anything.
///
/// `ids` is THIS window's own `AppIds` — read (`.work_id.get()`) only at the
/// final `switch.request` call below, not snapshotted here, so the outgoing Work
/// it names is whatever is actually open in this window at that (later, async)
/// moment, not whatever was open when the picker was first invoked.
///
/// …unless this window may not replace its project in place at all
/// ([`may_switch_project_in_place`] — a Work ▸ New Window sibling shares its
/// `AppIds`). Then the picked project takes the same route a backup already
/// does: its own window, this one untouched. Deciding that *here*, after the
/// pick, rather than at the menu is deliberate — the answer can change while the
/// native picker is up (a sibling window can be opened or closed under it), and
/// the state that matters is the one at the moment the load would actually
/// happen.
fn open_work_flow(
    switch: ProjectSwitchViewModel,
    ids: AppIds,
    registry: WorkRegistry,
    role: WindowRole,
    ctx: &mut EventContext,
) {
    let req = crate::models::dialog_start_in(
        ctx,
        crate::models::FolderPurpose::OpenProject,
        FileDialogRequest::pick_file()
            .title("Open Skribisto work")
            .add_filter("Skribisto work", &["skrib"]),
    );
    let _ = ctx.pick_file(req, move |res, ectx| {
        if let FileDialogResult::File(Some(path)) = res {
            crate::models::remember_dialog_file(
                ectx,
                crate::models::FolderPurpose::OpenProject,
                &path,
            );
            let file = path.to_string_lossy().into_owned();
            let switch = switch.clone();
            let ids = ids.clone();
            let registry = registry.clone();
            let file_for_check = file.clone();
            ectx.spawn_local_with(
                async move {
                    spawn_blocking(move || crate::backup::is_backup_path(&file_for_check))
                        .await
                        .unwrap_or(false)
                },
                move |is_backup, ectx2| {
                    // A backup always opens in its own WINDOW. A window that
                    // may not switch in place ([`WindowRole`]) takes the same
                    // route — the picked project gets a window of its own.
                    if is_backup || !may_switch_project_in_place(&registry, &ids, role) {
                        crate::shell::windows::open_or_focus_project(ectx2, &file);
                        return;
                    }
                    switch.request(
                        ectx2,
                        PendingSwitch::OpenWork(file.clone()),
                        ids.work_id.get(),
                    );
                },
            )
            .detach();
        }
    });
}

#[cfg(test)]
mod tests;

/// Attribution tests for [`mutation_ids_belong_to_work`], against a real store:
/// the guard's one job is to bump *this* window's Work and never a sibling's.
/// Real commands rather than fixtures because the walk is nothing but
/// relationship reads — a faked store would test the fake.
#[cfg(all(test, not(feature = "mocks")))]
mod dirty_guard_tests;
