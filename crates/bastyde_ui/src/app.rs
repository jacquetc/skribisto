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
//! Plain builder calls rather than `bati!`: the docking/tab/editor widgets are
//! generic over closures, which the DSL doesn't express cleanly. See
//! `settings.rs` for the `bati!` style.

mod commands;
mod project_shell;
mod window_role;
mod wiring;

pub(crate) use window_role::WindowRole;

use std::rc::Rc;

use bastyde::core::DragPayload;
use bastyde::core::modal::{ModalCloseBehavior, ModalPresentation, ModalRequest};
use bastyde::core::widget::WidgetPlacement;
use bastyde::prelude::*;
use bastyde::settings::{Reloadable, SettingsExt, SettingsRegistry};
use bastyde::tokens::SurfaceRole::Hover;
use bastyde::widgets::{
    DockWidgetId, EventContextMessageBoxExt, MessageBox, MessageBoxButton, MessageBoxButtons,
    RowDragData, StandardButton, TabBarVisibility, TabWidget, ToastRegistry,
};

use frontend::AppContext;
use frontend::commands::{
    binder_commands, undo_redo_commands, work_commands, work_management_commands,
};
use frontend::common::direct_access::binder::BinderRelationshipField;
use frontend::common::direct_access::work::WorkRelationshipField;
use frontend::common::event::{DirectAccessEntity, EntityEvent, Event, Origin};
use frontend::work_management::{CloseWorkDto, LoadWorkDto, NewWorkDto};

use crate::app_ids::AppIds;
use crate::export::panel::ExportPanel;
use crate::models::TreeNode;
use crate::panels::new_work::NewWorkPanel;
use crate::sessions::{StackTeardown, WindowTeardown, WorkRegistry, WorkSession};
use crate::settings::SettingsPanel;
use crate::singles::SingleSmartPunctuation;
use crate::text_replacement::typography::SmartPunctuationFlags;
use crate::toast_scope::ToastWorkExt;

/// The punctuation rules in force for the open project — the two tiers resolved
/// into the one flag set the editor sessions run.
///
/// `override_app_default` off means "follow the application preference", which
/// is now a real thing to follow rather than a synonym for off. On means this
/// project departs from it, and the row's own five values win outright — not
/// merged with the app tier, because a house style is a whole system (Italian
/// picks one of three; French adds a space its neighbours do not) and a
/// half-inherited one belongs to no language at all.
fn punctuation_flags(
    sp: &SingleSmartPunctuation,
    app: &SettingsViewModel,
) -> SmartPunctuationFlags {
    if sp.override_app_default().get() {
        return SmartPunctuationFlags {
            dashes: sp.dashes().get(),
            ellipsis: sp.ellipsis().get(),
            quotes: sp.quotes().get(),
            quote_style: sp.quote_style().get(),
            pre_punctuation_spacing: sp.pre_punctuation_spacing().get(),
            dialogue_marker: sp.dialogue_marker().get(),
        };
    }
    SmartPunctuationFlags {
        dashes: app.punct_dashes().get(),
        ellipsis: app.punct_ellipsis().get(),
        quotes: app.punct_quotes().get(),
        quote_style: app.punct_quote_style().get(),
        pre_punctuation_spacing: app.punct_spacing().get(),
        dialogue_marker: app.punct_dialogue().get(),
    }
}
use crate::tabs::{ContentTab, tab_pane};
use crate::view_models::{
    BackupSchedulerViewModel, BackupSettingsViewModel, EditorsViewModel, ExportViewModel,
    OutlineViewModel, PendingSwitch, ProjectSwitchViewModel, SaveAsViewModel,
    SearchReplaceViewModel, SettingsViewModel, Side, SpinnerGate, UnsavedDecision,
    unsaved_decision,
};

/// Narrowest an editor tab may be squeezed, in dp — well above bastyde's 96 dp
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
    TabWidget::new(editors.selected(side))
        .dynamic_tab::<ContentTab>("editor", |_handle, state| tab_pane(state))
        .dynamic_model(editors.tabs(side))
        .on_close(move |tab_id, _ctx| close.close_in(side, tab_id))
        .on_tab_received(move |handle, _idx, _ctx| recv.receive_tab(side, handle))
        .on_transfer_out(move |tab_id, _ctx| out.transfer_out(side, tab_id))
        .reorderable(true)
        .accept_external_tabs(true)
        .bar_visibility(bar_visibility)
        .compact_bar()
        .min_tab_width(MIN_EDITOR_TAB_WIDTH)
        .selected_tab_background(SurfaceRole::Content)
        .hover_tab_background(Hover)
        .tab_dividers()
        .active_indicator(bastyde::widgets::TabIndicatorPosition::InnerEdge)
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
/// through [`QuitSequencer`](crate::view_models::QuitSequencer) and terminates the process. Both outcomes share
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
    /// [`QuitSequencer`](crate::view_models::QuitSequencer)'s continuation, run from
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
    New(NewWorkDto),
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
    },
}

impl PendingAction {
    /// The on-disk path this action targets — used to derive the project
    /// window's persistence id ([`crate::shell::windows::window_id_for`]) before the
    /// action itself has run.
    pub fn target_path(&self) -> &str {
        match self {
            PendingAction::Load(path) => path,
            PendingAction::New(dto) => &dto.file_name,
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
    workspace_layout: &crate::view_models::WorkspaceLayoutViewModel,
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
/// through `workspace_layout` — the **calling window's own** [`WorkspaceLayoutViewModel`](crate::view_models::WorkspaceLayoutViewModel)
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
    workspace_layout: &crate::view_models::WorkspaceLayoutViewModel,
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
    workspace_layout: &crate::view_models::WorkspaceLayoutViewModel,
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
fn spell_underline_color(c: bastyde::tokens::Color) -> bastyde::text_document::Color {
    let [r, g, b, _] = c.to_array();
    let to_u8 = |x: f32| (x.clamp(0.0, 1.0) * 255.0).round() as u8;
    bastyde::text_document::Color::rgb(to_u8(r), to_u8(g), to_u8(b))
}

/// After a project becomes live, offer to install any dictionary its declared languages need
/// but the machine lacks — one aggregated toast (never one per language), whose action opens
/// Settings ▸ Dictionaries with the missing set highlighted. Purely additive and dismissible,
/// so a toast, not a modal.
pub(crate) fn offer_missing_dictionaries(
    docs: &crate::models::OpenDocsStore,
    dictionaries: &crate::view_models::DictionariesViewModel,
    session: &WorkSession,
    ctx: &mut EventContext,
) {
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
        bastyde::widgets::Toast::info(tr!(dict_missing_toast(count = n)))
            // Work-scoped (F1): a bare "dict.missing" shared by every window
            // would let a second Work's own nudge find THIS Work's still-live
            // toast and silently steal/retarget it.
            .scoped_id("dict.missing", work_id)
            .target_work(work_id)
            .action(bastyde::widgets::ToastAction::primary(
                tr!(dict_missing_action()),
                move |c| {
                    let session = session.clone();
                    c.present_modal(
                        ModalRequest::deferred(move |t| {
                            t.add(SettingsPanel::open_to_dictionaries(session))
                        })
                        .presentation(ModalPresentation::InTree)
                        .title("Settings")
                        .size(920, 620)
                        .close_behavior(ModalCloseBehavior::Manual),
                    );
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
    fullscreen: crate::view_models::FullscreenViewModel,
    /// This window's own distraction-free state (Increment 2 — chrome
    /// collapse), with its own independent placement memory from
    /// [`Self::fullscreen`] — see `FocusViewModel`'s module doc for why the
    /// two toggles never share one. Reset on Close-Work/Load-Work so a stale
    /// "mode was on" never leaks into the next project this window shows.
    focus: crate::view_models::FocusViewModel,
    /// Bound to this window's own `ids`, so an export from this window scopes
    /// to *this* Work. See `app::commands::CommandDeps::export`'s doc.
    export: crate::view_models::ExportViewModel,
    /// The Tier-1 registry every open Work registers into once its own
    /// `LoadWork`/`NewWork` resolves a real `work_id` (see the `LoadWork`/
    /// `NewWork` subscribers in `build`, which also bind this window's id to
    /// that `work_id` — `WorkRegistry::register_window`). Unregistered not on
    /// `CloseWork` but either once bastyde's `on_removed` hook confirms this
    /// window is really gone (`WorkRegistry::remove_window`, wired in
    /// `shell::windows`), or the instant this same window's own
    /// `register_window` call supersedes its own previous binding on an
    /// in-place Work switch (see that method's doc).
    registry: WorkRegistry,
    /// The app-global quit sequencer (`app.quit`). Shared across every window,
    /// unlike almost everything else on this struct: a quit spans them all.
    quit: crate::view_models::QuitSequencer,
    /// Built fresh alongside `session`, bound to *this* window's own `ids`/
    /// `single_work`.
    save_as_vm: SaveAsViewModel,
    restore_vm: crate::view_models::BackupRestoreViewModel,
    /// This window's own formatting surfaces (dock + menu + editor registry).
    /// Never shared — a shared instance made the last-built window win the
    /// Format dock's live target.
    format: crate::view_models::FormatViewModel,
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
    df_surface: crate::view_models::DistractionFreeSurfaceViewModel,
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
    /// Live "a scene's prose is the active surface" flag, shared with the
    /// title-bar's Format menu so its entries grey out off a scene. Written by
    /// `EditorsViewModel`, which is the only thing that can compute it.
    scene_focused: Signal<bool>,
    binder_has_selection: Signal<bool>,
    /// This window's menu model plus the id of its "Insert template" submenu, so
    /// `build` can refill that submenu as the catalogue changes. `None` where the
    /// platform gives no title-bar host and there is no model to refill.
    templates_menu: Option<(
        bastyde::widgets::MenuModel,
        bastyde::core::menu_item_id::MenuItemId,
    )>,
    /// Pre-allocated id for the **Image** menu, which is inserted and removed as
    /// the selection changes (see `project_menus::sync_image_menu`). Minted here
    /// rather than by the insertion, because a menu that will be removed again
    /// has to be nameable before it exists.
    image_menu_id: bastyde::core::menu_item_id::MenuItemId,
    /// Live "is there a target" mirrors for the title-bar's Go menu (Increment 4 —
    /// six Next/Previous × Scene/Chapter/Note rows), the same shape as
    /// `scene_focused` just above: minted in `shell/windows.rs` (which builds the
    /// menu before this `App`/its `EditorsViewModel` exist), forwarded to
    /// `EditorsViewModel::new` here so it can write the live answer, and read
    /// straight from the window-chrome closure's own clone for the menu's
    /// `.enabled(..)` bindings. See [`crate::view_models::GoAvailability`]'s doc.
    go: crate::view_models::GoAvailability,
    /// This window's "jump to any item" popup state — see `GoToViewModel`.
    go_to: crate::view_models::GoToViewModel,
    /// `true` while the open work has edits not yet written to disk. Read by the
    /// close guard, `work.close` and the switch guard to decide whether to prompt,
    /// and by `can_save` for the Save affordances.
    ///
    /// **Derived**, not set by hand: `dirty_seq > saved_seq`, both read off the
    /// shared [`crate::view_models::SaveStateViewModel`] (Work-scoped, not owned
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
    comments_dock: DockWidgetId,
    doc_comments_dock: DockWidgetId,
    footnotes_dock: DockWidgetId,
    /// The trash feature's shared view-model, created once on first build.
    trash: Option<crate::view_models::TrashViewModel>,
    comments: Option<crate::view_models::CommentsViewModel>,
    footnotes: Option<crate::view_models::FootnotesViewModel>,
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
}

impl App {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        app_ctx: Rc<AppContext>,
        session: WorkSession,
        outline: OutlineViewModel,
        fullscreen: crate::view_models::FullscreenViewModel,
        focus: crate::view_models::FocusViewModel,
        export: crate::view_models::ExportViewModel,
        autosave_menu: Signal<bool>,
        spellcheck_menu: Signal<bool>,
        comments_menu: Signal<bool>,
        scene_focused: Signal<bool>,
        binder_has_selection: Signal<bool>,
        templates_menu: Option<(
            bastyde::widgets::MenuModel,
            bastyde::core::menu_item_id::MenuItemId,
        )>,
        go: crate::view_models::GoAvailability,
        go_to: crate::view_models::GoToViewModel,
        unsaved: Signal<bool>,
        pending_exit: Signal<PendingExit>,
        backup_mode: Signal<bool>,
        backup_context: Signal<Option<crate::backup::BackupContext>>,
        initial_action: PendingAction,
        registry: WorkRegistry,
        quit: crate::view_models::QuitSequencer,
        save_as_vm: SaveAsViewModel,
        restore_vm: crate::view_models::BackupRestoreViewModel,
        format: crate::view_models::FormatViewModel,
        project_switch: ProjectSwitchViewModel,
        title_text: Signal<String>,
        window_ordinal: Signal<usize>,
        df_surface: crate::view_models::DistractionFreeSurfaceViewModel,
    ) -> Self {
        Self {
            df_surface,
            app_ctx,
            session,
            outline,
            fullscreen,
            focus,
            export,
            registry,
            quit,
            save_as_vm,
            restore_vm,
            format,
            project_switch,
            title_text,
            window_ordinal,
            autosave_menu,
            spellcheck_menu,
            comments_menu,
            scene_focused,
            binder_has_selection,
            templates_menu,
            image_menu_id: bastyde::core::menu_item_id::MenuItemId::next(),
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
            comments_dock: DockWidgetId::from_raw(crate::docks::COMMENTS_DOCK_ID),
            doc_comments_dock: DockWidgetId::from_raw(crate::docks::DOC_COMMENTS_DOCK_ID),
            footnotes_dock: DockWidgetId::from_raw(crate::docks::FOOTNOTES_DOCK_ID),
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
    match bastyde::settings::AppPaths::new("eu", "skribisto", "Skribisto") {
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
fn mutation_origins() -> Vec<Origin> {
    use DirectAccessEntity::{Binder, BinderItem, BinderTag, DictWord, Work};
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
        DictWord(EntityEvent::Created),
        DictWord(EntityEvent::Updated),
        DictWord(EntityEvent::Removed),
    ] {
        v.push(Origin::DirectAccess(ent));
    }
    v
}

/// "Is this `DirectAccess` mutation event about *my* Work?" — the guard the
/// autosave `mutation_origins()` loop needs, and the harder half of "guard every
/// subscriber": unlike `LoadWork`/`NewWork`/`CloseWork`, none of these five entity
/// kinds' events carry a `work_id` — only the changed entities' own ids. Answering
/// requires walking the relationship each entity actually has back to a `Work`:
///
/// * `Work(Updated)` — the entity id IS the work id; a direct comparison.
/// * `Binder`/`BinderTag`/`DictWord` — each is a direct `Work` one-to-many child
///   (`qleany.yaml`'s `Work.binders`/`.tags`/`.dict_words`); one relationship read
///   answers it.
/// * `BinderItem` — one hop further (`Work` → `Binder` → `BinderItem`): first the
///   Work's own binder ids, then each binder's item ids.
///
/// Deliberately re-queried per event rather than cached: these are structural
/// mutations (create/rename/move/trash), not per-keystroke prose edits (`Content`
/// events are excluded from `mutation_origins` for exactly that reason), so they
/// are rare enough that a live relationship read costs nothing an autosave-timer
/// debounce would notice.
///
/// An event with no ids at all (shouldn't happen for these five kinds, but no
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
        DirectAccessEntity::DictWord(_) => {
            let mine = work_commands::get_work_relationship(
                ctx,
                &my_work_id,
                &WorkRelationshipField::DictWords,
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
        _ => true, // not one of `mutation_origins`'s five kinds — never reached
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
/// Work (a real close, driven by bastyde's `on_removed` hook — see
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
/// runs once bastyde's `on_removed` hook confirms this window is really gone
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
pub(crate) fn build_window_teardown(
    editors: EditorsViewModel,
    backup_scheduler: BackupSchedulerViewModel,
    toast_registry: Option<ToastRegistry>,
    window_id: BastydeWindowId,
    stack_id: Option<u64>,
) -> WindowTeardown {
    Rc::new(move || {
        editors.release_own_open_docs(stack_id);
        backup_scheduler.unregister_flush_hook(window_id);
        if let Some(reg) = &toast_registry {
            reg.forget_window(window_id);
        }
    })
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
        // ── Layer-B view-models: created once, then shared by clone ──────────
        let settings = SettingsViewModel::new(ctx.settings());

        // This window's own id, read once per build — `None` only in a
        // headless/off-screen build context (never a real project window; see
        // the backup flush-hook registration below, this crate's first consumer
        // of `ctx.window()`). Used both there and by the window-teardown
        // registration further down (`WorkRegistry::register_window`), which is
        // what bastyde's `on_removed` hook (wired in `shell::windows`) looks
        // this same window up by once it is confirmed gone.
        let window_id = ctx.window().map(|w| w.id());

        // Scope D — window titles. Push every change of this window's live
        // title text (`Self::title_text`, already bound to the drawn custom
        // title bar in `shell::windows`) to the OS-level title too, via
        // `WindowState::title()` — its own doc: an app-side `.set()` here
        // round-trips through the window manager into a real OS window-title
        // call, the half a KWin rule (matching a window by its title text)
        // actually needs. A no-op in a headless/off-screen build context
        // (`ctx.window()` is `None` there — same guard as `window_id` above).

        // The Tier-2 per-open-Work bundle — see `sessions::WorkSession`'s module
        // doc and this struct's own field doc for why `App::build` reads these
        // straight off `session` instead of doing its own `ctx.app_state::<T>()`
        // lookup per field, the way the rest of this function used to.
        let session = self.session.clone();

        if let Some(window) = ctx.window() {
            let os_title = window.title().clone();
            // Observe the two MUTABLE sources, never `self.title_text` itself:
            // that is `single_work.title().zip(ordinal).map(..)`
            // (`shell::windows::window_title_text`), and a zip/map signal is
            // lazy and read-only — `ctx.effect` observes, and `observe()`
            // panics on a derived signal. Reading its value inside the closure
            // is fine; only observing it is not. Both arms recompute the whole
            // title, so either source changing pushes the same correct text.
            let title_text = self.title_text.clone();
            let title = session.single_work.title();
            {
                let os_title = os_title.clone();
                let title_text = title_text.clone();
                ctx.effect(&title, move |_: &String| {
                    os_title.set(title_text.get());
                });
            }
            ctx.effect(&self.window_ordinal, move |_: &usize| {
                os_title.set(title_text.get());
            });
        }
        let app_ctx = self.app_ctx.clone();
        let column_width = settings.column_width();
        let show_synopsis = settings.synopsis_pane();
        let synopsis_placement = settings.synopsis_placement();
        let synopsis_side_width = settings.synopsis_side_width();
        let typography = settings.editor_typography();
        // Typewriter scrolling: the two source signals, straight from the store,
        // so a settings change reaches every open tab's editors and its page's
        // scroll range together.
        let typewriter = crate::view_models::TypewriterSettings::new(
            settings.typewriter(),
            settings.typewriter_anchor(),
        );
        // The caret band. Its colour is a *resolved* theme colour, because it crosses into the
        // document as a `HighlightFormat` field rather than staying a paintable role — the same
        // trip the spell-check squiggle colour makes — so the bundle owns a source signal kept
        // current by a theme effect. Built through the shared constructor, which the Search &
        // Replace preview dock also uses; duplicating it here is how one of the two ends up not
        // following a light/dark switch.
        let caret_highlight = crate::view_models::CaretHighlightSettings::from_context(ctx);
        let view_memory = crate::view_models::EditorViewMemory::new(ctx.settings());
        let corkboard_defaults = settings.corkboard_defaults();
        let ids = self.outline.ids();
        let docs = session.open_docs.clone();
        // Work-scoped, not per-window: created once in `main` (inside the
        // `WorkSession` bundle) and shared by every window onto this project —
        // see `SaveStateViewModel`'s module docs for why a per-window copy of
        // `dirty_seq`/`saved_seq`/`saving`/the `SaveQueue` is a bug the moment a
        // second window exists. Unlike every other Tier-2 field, this one is
        // *not* also reachable via `ctx.app_state::<SaveStateViewModel>()` —
        // `main.rs` deliberately stopped registering it there once `App` itself
        // started carrying the session (see `main.rs`'s comment at its
        // construction site).
        let save_state = session.save_state.clone();
        let backup_mode_for_editors = self.backup_mode.clone();
        let save_state_for_editors = save_state.clone();
        let scene_focused_for_editors = self.scene_focused.clone();
        // A **pane** tab is never distraction-free — a constant `false`, not
        // `self.focus.active_signal()`. `TabWidget` memoizes its panes and
        // `DockingLayout` preserves its centre across rebuilds, so a signal here
        // would never actually be re-read once a pane is built: a scene opened
        // before entering the mode would keep its normal typeface/column for the
        // whole session. The mode instead mounts its own surface with its own
        // tab, whose flag is a constant `true` (`EditorsViewModel::open_surface_tab`).
        let distraction_free_for_editors = Signal::new(false);
        let distraction_free_width = settings.distraction_free_width();
        let go_for_editors = self.go.clone();
        let format_for_editors = self.format.clone();
        let editors = self
            .editors
            .get_or_insert_with(|| {
                EditorsViewModel::new(
                    app_ctx,
                    column_width,
                    show_synopsis,
                    synopsis_placement,
                    synopsis_side_width,
                    typography,
                    typewriter,
                    caret_highlight,
                    view_memory,
                    corkboard_defaults,
                    ids,
                    docs,
                    backup_mode_for_editors,
                    save_state_for_editors,
                    scene_focused_for_editors,
                    session.tree_expansion.clone(),
                    distraction_free_for_editors,
                    distraction_free_width,
                    go_for_editors,
                    format_for_editors,
                )
            })
            .clone();

        // This window's own format VM (minted in the factory with the menu bar).
        // Re-pointed at the editors here, on every build — idempotent.
        let format = self.format.clone();
        {
            let target = editors.clone();
            format.attach(Rc::new(move || {
                use crate::view_models::FormatSurface;
                // One walk answers both halves, at deliberately different
                // strictnesses.
                //
                // The *target* is sticky: it stays the tab's editor whether or
                // not it holds focus this instant. Opening the Format menu moves
                // focus to the menu overlay, so a target gated on live focus
                // would vanish exactly when the user reached for a command.
                //
                // The *surface* is live: click into the binder and there is
                // genuinely nothing to format, so the dock drops to its empty
                // state rather than offering controls for a caret that is no
                // longer anywhere. (The dock's own buttons are
                // `focusable(false)`, so pressing one never blurs the editor out
                // from under itself.)
                let Some((handle, is_synopsis, focused)) = target.format_target() else {
                    return (None, FormatSurface::None);
                };
                if !focused {
                    return (Some(handle), FormatSurface::None);
                }
                if is_synopsis {
                    return (Some(handle), FormatSurface::Synopsis);
                }
                // The same predicate the compiler uses to decide what it scans,
                // so the dock cannot offer a scene break where the exporter
                // would ignore one.
                let surface = if target.focused_carries_scene() {
                    FormatSurface::Scene
                } else {
                    FormatSurface::Note
                };
                (Some(handle), surface)
            }));
        }

        // Pull the focused editor's formatting into the mirrors once per frame,
        // here rather than in the dock: the Format menu binds the same signals,
        // and the dock is only one of two trailing rail tabs — driven from
        // there, the menu's checkmarks would freeze whenever the user switched
        // the rail to the Inspector.
        //
        // Deliberately not an effect on the editor's `format_version`: that
        // signal is written from inside the editor's own `state.borrow_mut()`
        // and observers fire synchronously there, so reading the state back
        // would panic on an already-borrowed cell. A frame tick fires outside
        // any borrow, and `refresh` short-circuits when nothing has moved.
        {
            let format = format.clone();
            let tick = ctx.frame_tick();
            ctx.effect(&tick, move |_| format.refresh());
        }

        // Hand the editors to the per-work workspace-layout restore. It was created
        // in `main` (inside the `WorkSession` bundle, before any `ctx.settings()`),
        // so it starts editor-less and is wired here, on every build — idempotent
        // (`set_editors` just re-points). Kept as a local `Option` (rather than
        // `session.workspace_layout` directly) so the Load/New subscribers below,
        // which pre-date this field always being present, don't need reshaping.
        //
        // **`None` for an attached window** — see [`WindowRole::owns_desk`].
        let workspace_layout = self
            .role
            .owns_desk()
            .then(|| session.workspace_layout.clone());
        if let Some(layout) = &workspace_layout {
            layout.set_editors(editors.clone());
            // Same idempotent re-point, for `capture_tree_expansion`'s own use of
            // this window's outline (Scope C fix — see that method's doc).
            layout.set_outline(self.outline.clone());
        }

        // ── Spell-checking wiring (Step 6). `docs` above was moved into the editors VM, so
        // re-fetch the shared handle for the attach loop. ──
        let spell_docs = session.open_docs.clone();
        // Keep the store's cached language map honest: an item's `dict_language` or
        // `sub_role` edited in place leaves the binder's shape unchanged, so only the
        // entity event can invalidate it. Every build — the subscription is scoped to
        // this one (see `OpenDocsStore::wire`).
        spell_docs.wire(ctx);
        let spellcheck = ctx
            .app_state::<crate::spellcheck::SpellcheckService>()
            .cloned()
            .expect("SpellcheckService registered in main");
        let dictionaries = ctx
            .app_state::<crate::view_models::DictionariesViewModel>()
            .cloned()
            .expect("DictionariesViewModel registered in main");
        // Dictionary / personal-word / theme changes → re-attach every open document.
        // See `app::wiring::spellcheck` for the re-attach policy.
        wiring::spellcheck::install(
            ctx,
            &self.app_ctx,
            &session.ids,
            &spell_docs,
            &spellcheck,
            &dictionaries,
        );
        // Live cross-process reload for `dictionaries.toml` (accepted licences), mirroring the
        // backup-settings registration below.
        if self.dictionary_settings_reloadable.is_none()
            && let Some(registry) = ctx.app_state::<SettingsRegistry>().cloned()
        {
            self.dictionary_settings_reloadable =
                Some(registry.register(dictionaries.settings_reloadable()));
        }
        // Live cross-process reload for `export_styles.toml` (user styles), pulled from
        // app-state since `App` doesn't own the view-model.
        if self.export_styles_reloadable.is_none()
            && let Some(registry) = ctx.app_state::<SettingsRegistry>().cloned()
            && let Some(styles) = ctx
                .app_state::<crate::view_models::ExportStylesViewModel>()
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

        // ── The search feature's shared view-model (both docks clone it) ──────
        // Created once; it holds the results model, the persisted `search.toml`
        // service, the shared open-docs store (so its preview edits the same
        // document a tab does), and the DockingModel (to reveal the bottom band).
        let search = {
            let app_ctx = self.app_ctx.clone();
            let ids = self.outline.ids();
            let docs = session.open_docs.clone();
            let docking = outline.docking();
            let search_dock = self.search_dock;
            let preview_dock = self.preview_dock;
            self.search
                .get_or_insert_with(|| {
                    let settings_svc = open_search_settings();
                    let results = crate::models::SearchResultsModel::new(
                        app_ctx.clone(),
                        ids.work_info_id.clone(),
                    );
                    SearchReplaceViewModel::new(
                        app_ctx,
                        ids,
                        results,
                        settings_svc,
                        docs,
                        docking,
                        preview_dock,
                        search_dock,
                    )
                })
                .clone()
        };

        // ── The trash feature's shared view-model ────────────────────────────
        // Shares the outline's DockingModel (the leading rail hosts several tabs)
        // and a distinct dock id; created once, not registered as app_state.
        let trash = {
            let app_ctx = self.app_ctx.clone();
            let ids = self.outline.ids();
            let docking = outline.docking();
            let trash_dock = self.trash_dock;
            self.trash
                .get_or_insert_with(|| {
                    let model =
                        crate::models::TrashTreeModel::new(app_ctx.clone(), ids.work_id.clone());
                    crate::view_models::TrashViewModel::new(
                        app_ctx, ids, model, docking, trash_dock,
                    )
                })
                .clone()
        };
        // ── The comments feature's shared view-model ─────────────────────────
        // One instance for both docks: the project-wide dock binds the whole list,
        // the per-document dock the same handle narrowed to the focused item. Two
        // models over one entity set would drift the moment a thread was resolved
        // in one and not the other.
        let comments = {
            let app_ctx = self.app_ctx.clone();
            let ids = self.outline.ids();
            self.comments
                .get_or_insert_with(|| {
                    let model = crate::models::CommentsListModel::new(app_ctx.clone(), ids.clone());
                    crate::view_models::CommentsViewModel::new(model, app_ctx, ids.stack_id.clone())
                })
                .clone()
        };
        // ── The footnotes feature's view-model ───────────────────────────────
        // One per window, on the same footing as `comments` above: the dock binds
        // it, and every open document is handed a per-`Content` binding through
        // the store, so an editor can insert a reference and report its caret
        // without ever holding the view-model itself.
        let footnotes = {
            let app_ctx = self.app_ctx.clone();
            let ids = self.outline.ids();
            let docs = session.open_docs.clone();
            self.footnotes
                .get_or_insert_with(|| {
                    let model = crate::models::FootnotesListModel::new(
                        app_ctx.clone(),
                        ids.clone(),
                        docs.clone(),
                    );
                    crate::view_models::FootnotesViewModel::new(model, docs, ids.stack_id.clone())
                })
                .clone()
        };
        footnotes.wire(ctx);
        // Hand it to the open-document store, which back-fills every document
        // already open (a workspace restore opens tabs before this point) and
        // seeds each one opened later — including the marker map, without which a
        // reference paints its raw label.
        session.open_docs.set_footnotes(footnotes.clone());
        // Renumber when a document's references move. The model's own gate makes
        // this cheap: it asks the open documents which notes they name, and only
        // walks the manuscript when that answer has changed.
        {
            let f = footnotes.clone();
            ctx.effect(&session.open_docs.edited_any(), move |_| f.note_live_edit());
        }

        // The default author for new threads is the book's byline — the only name
        // the app knows. There is no identity system and this does not invent one.
        comments.set_default_author(&session.single_work.author_name().get());
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
            let theme_sig = ctx.theme_signal().clone();
            ctx.effect(&theme_sig, move |t| {
                let is_dark = t.is_dark();
                if dark.get() != is_dark {
                    dark.set(is_dark);
                }
            });
        }
        if let Some(locale_sig) = bastyde::i18n::current_locale() {
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
        let single_work_info = session.single_work_info.clone();
        // Resolved off THIS window's own `session`, same as every other
        // Layer-A single above — a second, simultaneously-open Work always
        // gets its own punctuation handle (see `WorkSession::smart_punctuation`'s
        // own doc).
        let smart_punctuation = session.smart_punctuation.clone();
        single_work.wire(ctx);
        single_work_info.wire(ctx);
        smart_punctuation.wire(ctx);

        // ── The punctuation house style, from the Work's row to every editor ──
        //
        // Two hops, because the row is reached through the Work: re-point the
        // handle whenever the open project changes, then push the flags whenever
        // any of them does. The second hop is what makes a switch flipped in
        // Settings reach the scene the writer is looking at without reopening it.
        {
            let sp = smart_punctuation.clone();
            ctx.effect(&single_work.smart_punctuation(), move |id| {
                // 0 is "no project open", never "this project has no row" — the
                // row is minted before its Work, so a live project always has one.
                sp.set_id((*id != 0).then_some(*id));
            });
        }
        // The project's default language, from the Work to the open documents.
        //
        // `OpenDocsStore` caches it (every item that has no tag of its own
        // inherits it, and re-resolving per keystroke would be wasteful), and
        // until this effect existed only `ProjectLifecycleViewModel` ever wrote
        // that cache — on load. So editing Settings ▸ Work ▸ Language updated
        // the entity, re-attached every document, and re-resolved them all
        // against the *stale* cached language: the change did not reach the
        // editor until the project was reopened. That affected spell-check as
        // much as punctuation.
        {
            let docs = spell_docs.clone();
            let ids = ids.clone();
            ctx.effect(&single_work.dict_language(), move |langs| {
                docs.set_project_language(ids.work_id.get(), langs.clone());
                docs.attach_all();
            });
        }

        {
            let docs = spell_docs.clone();
            let sp = smart_punctuation.clone();
            let app = settings.clone();
            let push = move || {
                docs.set_punctuation(Some(punctuation_flags(&sp, &app)));
            };
            // One effect per flag: `ctx.effect` takes a single signal, and these
            // are independent switches rather than one compound value.
            //
            // Both tiers are observed. The app-level ones matter even while a
            // project holds the override: it can be dropped at any moment, and
            // the flags it falls back to have to be current when it is.
            //
            // This list MUST name every signal `punctuation_flags` reads, on
            // both tiers — a flag observed here but not read there is harmless,
            // but one read there and not observed here silently fails to
            // propagate (that is exactly how `dialogue_marker` was inert until a
            // second setting changed). The bool signals of each tier:
            for signal in [
                smart_punctuation.override_app_default(),
                smart_punctuation.dashes(),
                smart_punctuation.ellipsis(),
                smart_punctuation.quotes(),
                smart_punctuation.pre_punctuation_spacing(),
                smart_punctuation.dialogue_marker(),
                settings.punct_dashes(),
                settings.punct_ellipsis(),
                settings.punct_quotes(),
                settings.punct_spacing(),
                settings.punct_dialogue(),
            ] {
                let push = push.clone();
                ctx.effect(&signal, move |_| push());
            }
            // …and the quote-style enum on each tier, which is a different type.
            for signal in [
                smart_punctuation.quote_style(),
                settings.punct_quote_style(),
            ] {
                let push = push.clone();
                ctx.effect(&signal, move |_| push());
            }
        }

        // The project-lifecycle view-model: the shared Load/New/Close sequence, which was
        // three hand-kept-in-step closures here. Built fresh each build — it holds only
        // clones of handles that are themselves stable across builds, so re-creating it is
        // idempotent; the subscribers below capture their own clone.
        let lifecycle = crate::view_models::ProjectLifecycleViewModel::new(
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
        // `BastydeAppBuilder` whenever a settings bundle is configured — see
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
        let project_switch = self.project_switch.clone();
        if self.role.installs_project_switch_hooks() {
            project_switch.set_save_hook(Rc::new({
                let editors = editors.clone();
                move || editors.request_save()
            }));
            project_switch.set_new_work_form_hook(Rc::new({
                let app_ctx = self.app_ctx.clone();
                let ids = ids.clone();
                move |c: &mut EventContext| {
                    let app_ctx = app_ctx.clone();
                    let ids = ids.clone();
                    c.present_modal(
                        ModalRequest::deferred(move |t| t.add(NewWorkPanel::new(app_ctx, ids)))
                            .presentation(ModalPresentation::InTree)
                            .title("New Work")
                            .close_behavior(ModalCloseBehavior::EscapeOrClickOutside)
                            .size(600, 680),
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
            fullscreen: self.fullscreen.clone(),
            focus: self.focus.clone(),
            editors: editors.clone(),
            trash: trash.clone(),
            search: search.clone(),
            project_switch: project_switch.clone(),
            backup_scheduler: backup_scheduler.clone(),
            dictionaries: dictionaries.clone(),
            spell_docs: spell_docs.clone(),
            user_dictionary: session.user_dictionary.clone(),
            export: self.export.clone(),
            search_dock: self.search_dock,
            trash_dock: self.trash_dock,
            footnotes_dock: self.footnotes_dock,
            unsaved: self.unsaved.clone(),
            backup_mode: self.backup_mode.clone(),
            pending_exit: self.pending_exit.clone(),
            autosave: settings.autosave(),
        };
        commands::register_all(ctx, &command_deps);

        // On project load/new/close/attach: lifecycle seed, backup sniff, dict offer.
        // Binder-item tab sync is not lifecycle — it rides every edit, not the boundaries —
        // so it is the editors' own `wire` (rebuild on retype, close on remove, re-caption
        // on anything that renumbers), subscribed here for this window's whole build.
        editors.wire(ctx);

        let attach_seed = wiring::project_events::install_lifecycle(
            ctx,
            wiring::project_events::LifecycleDeps {
                app_ctx: self.app_ctx.clone(),
                session: session.clone(),
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
        // The framework's answer (bastyde/docs/native-menu.md, "Dynamic structure") is a
        // pre-allocated submenu id plus runtime mutation, which is what this drives — each
        // refill bumps `MenuModel::version`, and the bar re-derives its dropdowns from it.
        // The Image menu appears while a picture is selected and goes away
        // when it is not. Driven off the same signal the Document menu's image
        // rows used to gate on, so the menu and the commands cannot disagree
        // about whether there is an image in hand.
        if let Some((menu, _)) = self.templates_menu.clone() {
            let image_menu_id = self.image_menu_id;
            let active = self.format.active_image();
            ctx.effect(&active, move |image| {
                crate::shell::project_menus::sync_image_menu(&menu, image_menu_id, image.is_some());
            });
        }

        if let Some((menu, submenu_id)) = self.templates_menu.clone() {
            let templates = session.note_templates.clone();
            let has_editor = self.format.has_target();
            ctx.effect(&templates.changed_signal(), move |_| {
                crate::shell::project_menus::sync_insert_template_submenu(
                    &menu,
                    submenu_id,
                    &templates,
                    &has_editor,
                );
            });
        }

        // Long-operation routing: every background job (import, export, save-as, backup,
        // restore, the progress recorder) reports through the same four events and filters
        // by its own op id. Grouped in `app::wiring::long_ops`; the editors' own save
        // routing stays below, since it also drives the deferred close/switch resumption.
        wiring::long_ops::install(
            ctx,
            &save_as_vm,
            &backup_scheduler,
            &restore_vm,
            &self.export,
            &self.session.mention_index,
            &self.session.progress_recorder,
        );

        // Keep the Document menu's per-item gate in step with this window's binder
        // selection. `MenuEntry::enabled` wants a concrete `Signal<bool>`, so the set is
        // projected into one here rather than handed over as a lazy `.map()`.
        {
            let has_selection = self.binder_has_selection.clone();
            let sig = self.outline.selection_signal();
            ctx.effect(&sig, move |set: &std::collections::HashSet<_>| {
                let now = !set.is_empty();
                if has_selection.get() != now {
                    has_selection.set(now);
                }
            });
        }

        // App mediates the two peer view-models: *activating* a binder item
        // (click or Enter — NOT arrow navigation, which only moves the selection)
        // opens (or focuses) its editor tab. The tree fires this via
        // `TreeView::on_activate`; App supplies the open callback so neither
        // view-model imports the other.
        let on_open: crate::docks::outline::OpenItemFn = {
            let editors = editors.clone();
            Rc::new(move |item_id, title| editors.open_or_focus(item_id, &title))
        };

        // Keep the "open document" id in sync with each pane's active tab (open,
        // close, or a tab-bar click), so the binder's open-item marker tracks the
        // focused pane. A selection change in a pane also marks it focused and
        // flushes pending edits to the store — autosave on a natural boundary
        // (changed fields only; clean tabs are a no-op). One effect per pane.
        //
        // Flush the *whole* store, not only the leaving tab: a shared
        // `OpenDoc` can be open in both panes (or in a stream row plus a
        // tab), and a title/prose edit on the tab we leave must land
        // before the next tab paints. Clean docs are a field-level no-op
        // (`is_modified` / title probe), so the cost stays proportional
        // to *dirty* open docs, not to how fast the user clicks tabs.
        for side in [Side::Primary, Side::Secondary] {
            let editors = editors.clone();
            ctx.effect(&editors.selected(side), move |_| {
                editors.flush_all();
                editors.set_focused(side);
            });
        }

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
        // ── Tools ▸ Comments ─────────────────────────────────────────────────
        // Same shape as the spell-check switch above, and deliberately the same scope: one
        // persisted app-wide key, mirrored into the menu's checkmark and pushed into the
        // store, which owns both halves of the hide (the view-model flag the margin binds,
        // and every open document's highlight layer).
        //
        // Presentation only. The docks keep listing every thread and the AccessKit
        // annotations keep announcing them — a writer who decluttered the page must still
        // be able to act on the notes they hid, and a screen-reader user gets nothing from
        // losing them.
        {
            self.comments_menu.set(settings.comments_visible().get());
            let menu = self.comments_menu.clone();
            let docs = spell_docs.clone();
            // Seeded before the first document opens, so a launch with comments hidden
            // never paints a frame of ochre before the effect catches up.
            docs.set_comments_visible(settings.comments_visible().get());
            ctx.effect(&settings.comments_visible(), move |on| {
                menu.set(*on);
                docs.set_comments_visible(*on);
            });
        }
        // Synopsis spell dormancy is not wired here: a doc can be on screen
        // several times at once (split pane, distraction-free), so "may this
        // session sleep?" is answered by counting the views that actually show
        // it — see `OpenDoc::acquire_synopsis_viewer`, held by the mounted pane.
        // Dirty tracking + debounced autosave-to-disk. Every mutation (editor
        // typing via the editors' `edited` signal, plus tree/metadata events)
        // marks the work `unsaved` and — when autosave is on — (re)schedules a
        // one-shot wake ~1.5 s out. `wake_at` keeps the loop asleep until the
        // deadline (no 60 fps drain); the `frame_tick` effect only runs on the
        // frames that actually pump, and fires the save when the deadline passes.
        // The countdown policy lives in `view_models::timers` (pure, `now`-injected,
        // unit-tested); `App` keeps only the two effects it must own — arming the
        // framework's `wake_at` and actually writing to disk.
        {
            use std::time::Instant;
            let countdown = Rc::new(crate::view_models::AutosaveCountdown::new());
            let wake = ctx.wake_at_handle();
            let autosave = settings.autosave();

            let on_mutation = {
                let countdown = countdown.clone();
                let wake = wake.clone();
                let autosave = autosave.clone();
                let save_state = save_state.clone();
                Rc::new(move || {
                    // Bump the shared edit sequence: this mutation is now ahead of
                    // whatever the last save covered, so the derived `unsaved` goes
                    // true for every window onto this project — and stays true if
                    // the save in flight (if any) predates it.
                    save_state.bump_dirty();
                    if let Some(at) = countdown.on_mutation(Instant::now(), autosave.get()) {
                        wake.set(Some(at));
                    }
                })
            };
            {
                let oc = on_mutation.clone();
                ctx.effect(&editors.edited_signal(), move |_| oc());
            }
            // Guarded: a sibling Work's mutation must not mark THIS window's Work
            // dirty or re-arm its autosave timer. Unlike `LoadWork`/`NewWork`/
            // `CloseWork`, none of these five entity kinds' events carry a
            // `work_id` — see `mutation_ids_belong_to_work`'s docs for the
            // relationship walk each one needs.
            for origin in mutation_origins() {
                let oc = on_mutation.clone();
                let app_ctx = self.app_ctx.clone();
                let my_ids = session.ids.clone();
                ctx.subscribe_event(origin, move |e: &Event| {
                    let Origin::DirectAccess(entity) = e.origin.clone() else {
                        return;
                    };
                    let Some(my_work_id) = my_ids.work_id.get() else {
                        return; // nothing open here — cannot be my mutation
                    };
                    if mutation_ids_belong_to_work(&app_ctx, my_work_id, entity, &e.ids) {
                        oc();
                    }
                });
            }
            {
                let editors = editors.clone();
                let autosave = autosave.clone();
                let tick = ctx.frame_tick();
                ctx.effect(&tick, move |_| {
                    let (save, resleep) = countdown.tick(Instant::now(), autosave.get());
                    if save {
                        editors.save_to_disk();
                    }
                    if let Some(at) = resleep {
                        wake.set(Some(at));
                    }
                });
            }
        }

        // Periodic "every N hours" backup while a project is open. Mirrors the
        // autosave timer: a `wake_at` deadline keeps the loop asleep; the
        // `frame_tick` effect fires `interval_tick` when the deadline passes and
        // re-arms. The interval (and whether it's on) comes from the open project's
        // effective policy via the scheduler; `None` disarms it.
        {
            use crate::view_models::IntervalTick;
            use std::time::{Duration, Instant};
            let countdown =
                crate::view_models::IntervalCountdown::new(backup_scheduler.completed_epoch());
            let wake = ctx.wake_at_handle();
            let scheduler = backup_scheduler.clone();
            let tick = ctx.frame_tick();
            ctx.effect(&tick, move |_| {
                let interval = scheduler.interval_secs().map(Duration::from_secs);
                match countdown.tick(Instant::now(), interval, scheduler.completed_epoch()) {
                    IntervalTick::Disarmed => {}
                    IntervalTick::Sleep(at) => wake.set(Some(at)),
                    IntervalTick::Fire(next) => {
                        scheduler.interval_tick();
                        wake.set(Some(next));
                    }
                }
            });
        }

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
                        let toast = crate::view_models::open_failure_toast(&path, &e);
                        eprintln!("skribisto: could not open '{path}': {e:#}");
                        ctx.run_after_mount(move |ctx| {
                            ctx.show_toast(toast);
                        });
                    }
                }
                Some(PendingAction::New(dto)) => {
                    let target = dto.file_name.clone();
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
                    work_id, ordinal, ..
                }) => attach_seed(work_id, ordinal),
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
    let req = FileDialogRequest::pick_file()
        .title("Open Skribisto work")
        .add_filter("Skribisto work", &["skrib"]);
    let _ = ctx.pick_file(req, move |res, ectx| {
        if let FileDialogResult::File(Some(path)) = res {
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
mod tests {
    use super::*;

    // ── Work ▸ New Window: may this window replace its project in place? ────
    //
    // The predicate every switch door consults. It is the whole protection for
    // the shared `AppIds`: get it wrong in the permissive direction and File ▸
    // Open Work from either of two windows on one Work deletes the backend
    // subtree the other one is displaying.

    /// A window alone on its project switches in place, exactly as before this
    /// feature existed — the common case must not regress.
    #[test]
    fn a_window_alone_on_its_project_may_switch_in_place() {
        let registry = WorkRegistry::new();
        let ids = AppIds::new();
        registry.register(1, crate::sessions::WorkSession::for_test());
        ids.work_id.set(Some(1));

        assert!(may_switch_project_in_place(
            &registry,
            &ids,
            WindowRole::Owner
        ));
    }

    /// With no project open there is nothing to replace and nothing to protect.
    #[test]
    fn a_window_with_no_project_may_switch_in_place() {
        let registry = WorkRegistry::new();
        let ids = AppIds::new();
        assert!(may_switch_project_in_place(
            &registry,
            &ids,
            WindowRole::Owner
        ));
    }

    /// The live condition: a sibling window is showing this very Work, and the
    /// two share one `AppIds`.
    #[test]
    fn a_window_sharing_its_work_with_a_sibling_may_not_switch_in_place() {
        let registry = WorkRegistry::new();
        let ids = AppIds::new();
        registry.register(1, crate::sessions::WorkSession::for_test());
        registry.attach(1).expect("a second window on Work 1");
        ids.work_id.set(Some(1));

        assert!(!may_switch_project_in_place(
            &registry,
            &ids,
            WindowRole::Owner
        ));
    }

    /// …and it becomes permitted again once that sibling closes: the reason was
    /// the sharing, not a one-way door.
    #[test]
    fn the_last_window_left_on_a_work_may_switch_in_place_again() {
        let registry = WorkRegistry::new();
        let ids = AppIds::new();
        registry.register(1, crate::sessions::WorkSession::for_test());
        registry.attach(1).expect("a second window on Work 1");
        ids.work_id.set(Some(1));
        assert!(!may_switch_project_in_place(
            &registry,
            &ids,
            WindowRole::Owner
        ));

        registry.unregister(1); // the sibling closed
        assert!(may_switch_project_in_place(
            &registry,
            &ids,
            WindowRole::Owner
        ));
    }

    /// The durable condition, and the one that is easy to miss: a window opened
    /// BY Work ▸ New Window may never switch in place, even once it is the only
    /// window left on the Work. It does not own the project's saved desk — the
    /// shared `workspace_layout` still holds the *original* window's
    /// `DockingModel` and editors — so an in-place switch there would persist a
    /// dead window's desk under the incoming project's key.
    #[test]
    fn an_attached_window_may_never_switch_in_place_even_when_left_alone() {
        let registry = WorkRegistry::new();
        let ids = AppIds::new();
        registry.register(1, crate::sessions::WorkSession::for_test());
        ids.work_id.set(Some(1));
        // No sibling at all: only `attached` stands in the way.
        assert_eq!(registry.window_count_for(1), 1);

        assert!(!may_switch_project_in_place(
            &registry,
            &ids,
            WindowRole::Attached
        ));
    }

    /// Work ▸ New Window carries the ordinal that decides both the window's
    /// title suffix and its persistence id, so `PendingAction` has to surface
    /// the Work it attaches to — and must not claim one for the actions that
    /// have no `work_id` until the backend mints it.
    #[test]
    fn only_an_attaching_action_names_a_work_up_front() {
        let attach = PendingAction::AttachExisting {
            work_id: 7,
            path: "/tmp/x.skrib".into(),
            ordinal: 2,
        };
        assert_eq!(attach.attached_work_id(), Some(7));
        assert_eq!(attach.target_path(), "/tmp/x.skrib");

        let load = PendingAction::Load("/tmp/y.skrib".into());
        assert_eq!(load.attached_work_id(), None);
        assert_eq!(load.target_path(), "/tmp/y.skrib");
    }

    /// The two-tier punctuation resolution, which is the whole point of
    /// `override_app_default` being a stored flag rather than an implied one.
    ///
    /// Driven through real handles rather than a reimplementation of the rule:
    /// a test that restated the `if` would pass no matter which way round it
    /// was written.
    #[test]
    fn a_project_without_an_override_follows_the_application_preference() {
        let ctx = std::rc::Rc::new(frontend::AppContext::new());
        let sp = SingleSmartPunctuation::new(ctx);
        let app = settings_vm_for_test();

        // The app tier says dashes on, spacing off.
        app.punct_dashes().set(true);
        app.punct_spacing().set(false);
        app.punct_quote_style()
            .set(frontend::common::entities::QuoteStyle::Guillemets);

        // The project disagrees on every count — and must be ignored while its
        // override is off.
        sp.set_override_app_default(false);
        sp.set_dashes(false);
        sp.set_pre_punctuation_spacing(true);
        sp.set_quote_style(frontend::common::entities::QuoteStyle::LowHigh);

        let f = punctuation_flags(&sp, &app);
        assert!(f.dashes, "the app tier decides");
        assert!(!f.pre_punctuation_spacing);
        assert_eq!(
            f.quote_style,
            frontend::common::entities::QuoteStyle::Guillemets
        );
    }

    /// And with the override on, the project's own row wins outright — not
    /// merged with the app tier. A house style is a whole system; a
    /// half-inherited one belongs to no language at all.
    #[test]
    fn an_overriding_project_wins_outright() {
        let ctx = std::rc::Rc::new(frontend::AppContext::new());
        let sp = SingleSmartPunctuation::new(ctx);
        let app = settings_vm_for_test();

        app.punct_dashes().set(true);
        app.punct_ellipsis().set(true);

        sp.set_override_app_default(true);
        sp.set_dashes(false);
        sp.set_ellipsis(false);

        let f = punctuation_flags(&sp, &app);
        assert!(!f.dashes, "the project decides, even to switch a rule OFF");
        assert!(
            !f.ellipsis,
            "an app-level rule is not inherited through an override"
        );
    }

    /// A throwaway store, so these read the real signal plumbing rather than a
    /// stand-in for it.
    fn settings_vm_for_test() -> SettingsViewModel {
        use std::sync::atomic::{AtomicU32, Ordering};
        static N: AtomicU32 = AtomicU32::new(0);
        let n = N.fetch_add(1, Ordering::Relaxed);
        let path = std::env::temp_dir().join(format!(
            "skribisto_punct_tier_{}_{n}.toml",
            std::process::id()
        ));
        let _ = std::fs::remove_file(&path);
        let store = bastyde::settings::SettingsStore::open(path).expect("open temp store");
        SettingsViewModel::new(&store)
    }

    /// The four states the Save affordances gate on. Nothing to save, or a
    /// read-only backup window → disabled.
    #[test]
    fn can_save_only_when_dirty_and_not_in_backup_mode() {
        for (dirty, backup, want) in [
            (false, false, false), // clean project — nothing to write
            (true, false, true),   // the only case that saves
            (false, true, false),  // backup window, clean
            (true, true, false),   // backup window with edits: Save As / Restore, not Save
        ] {
            let unsaved = Signal::new(dirty);
            let backup_mode = Signal::new(backup);
            assert_eq!(
                can_save(&unsaved, &backup_mode).get(),
                want,
                "dirty={dirty} backup_mode={backup}"
            );
        }
    }

    /// The signal is *derived*, not sampled: flipping either input after the fact
    /// must move it (otherwise the menu item would freeze at its build-time value).
    #[test]
    fn can_save_tracks_later_input_changes() {
        let unsaved = Signal::new(false);
        let backup_mode = Signal::new(false);
        let can = can_save(&unsaved, &backup_mode);
        assert!(!can.get());

        unsaved.set(true); // an edit lands
        assert!(can.get());

        backup_mode.set(true); // …in a backup window: still nothing Save can do
        assert!(!can.get());

        backup_mode.set(false); // Save As turned it back into a normal project
        assert!(can.get());

        unsaved.set(false); // saved to disk
        assert!(!can.get());
    }

    /// Phase 3 regression: `backup_mode` used to be one process-wide `Signal`
    /// shared by every open Work (see `sessions::WorkSession`'s module doc's
    /// "Phase 3 correction" section). Entering backup mode in Work A's window
    /// must leave Work B's own `can_save` (hence its Save menu item, Ctrl+S,
    /// and `editor.save`) fully enabled — this is the "backup-mode on Work A
    /// leaves Work B writable" guarantee the migration exists to restore.
    #[test]
    fn backup_mode_on_one_work_never_disables_saving_on_another() {
        let session_a = crate::sessions::WorkSession::for_test();
        let session_b = crate::sessions::WorkSession::for_test();
        // Both Works have unsaved edits. `unsaved` is ALSO per-Work now (Scope
        // E's own fix, `WorkSession::unsaved`) — using plain local `Signal`s
        // here instead keeps this test isolated to what it actually names:
        // `backup_mode`.
        let unsaved_a = Signal::new(true);
        let unsaved_b = Signal::new(true);

        let can_save_a = can_save(&unsaved_a, &session_a.backup_mode);
        let can_save_b = can_save(&unsaved_b, &session_b.backup_mode);
        assert!(can_save_a.get(), "Work A starts writable");
        assert!(can_save_b.get(), "Work B starts writable");

        // Work A opens a backup file — enters backup mode.
        session_a.backup_mode.set(true);

        assert!(!can_save_a.get(), "Work A's Save must now be disabled");
        assert!(
            can_save_b.get(),
            "Work B must stay writable — a sibling Work's backup mode must never disable it"
        );
    }

    /// The shortcut and the action must *both* follow the signal — the keystroke
    /// and the intent are two independent entry points into `save_to_disk`.
    #[test]
    fn save_shortcut_and_action_follow_can_save() {
        let unsaved = Signal::new(false);
        let backup_mode = Signal::new(false);
        let can = can_save(&unsaved, &backup_mode);

        let shortcut = Shortcut::new("editor.save")
            .name("Save")
            .primary(KeyStroke::ctrl(Key::S))
            .enabled_when(can.clone())
            .build();
        let action = Action::new("editor.save")
            .enabled_when(can)
            .on_invoke(|_i, _c| {});

        assert!(!shortcut.is_enabled(), "Ctrl+S inert on a clean project");
        assert!(!action.is_enabled(), "editor.save inert on a clean project");

        unsaved.set(true);
        assert!(shortcut.is_enabled());
        assert!(action.is_enabled());

        backup_mode.set(true);
        assert!(!shortcut.is_enabled(), "Ctrl+S inert in backup mode");
        assert!(!action.is_enabled(), "editor.save inert in backup mode");
    }

    /// F3: `build_window_teardown` — the closure `WorkRegistry::remove_window`
    /// runs once bastyde's `on_removed` hook confirms a window is really
    /// gone — must tell the toast registry to forget that window too, or
    /// `window_audiences`/`window_versions` leak one entry per window ever
    /// opened over the process's lifetime (see the function's own doc).
    /// `set_window_audience(id, None)` alone is NOT the fix (it only clears
    /// the signal's *value*, leaving the map entry) — this proves the actual
    /// teardown path removes the entry, by observing the framework's own
    /// documented post-`forget_window` behaviour: re-deriving the audience
    /// signal for a forgotten window starts fresh (`None`), not merely
    /// "whatever it was last set to".
    #[test]
    fn window_teardown_forgets_the_toast_registry_entry_too() {
        use bastyde::widgets::{ToastAudience, ToastInstallOptions};

        let app_ctx = Rc::new(frontend::AppContext::new());
        let session = crate::sessions::WorkSession::for_test();
        let editors = test_editors_view_model(&app_ctx);
        let window_id = BastydeWindowId::new(1);

        let registry = ToastRegistry::new(ToastInstallOptions {
            archive: None,
            ..ToastInstallOptions::default()
        });
        registry.set_window_audience(window_id, Some(ToastAudience::new(42)));
        assert_eq!(
            registry.window_audience_signal(window_id).get(),
            Some(ToastAudience::new(42)),
            "sanity: the audience is really set before teardown runs"
        );

        let teardown = build_window_teardown(
            editors,
            session.backup_scheduler.clone(),
            Some(registry.clone()),
            window_id,
            None,
        );
        teardown();

        assert_eq!(
            registry.window_audience_signal(window_id).get(),
            None,
            "a forgotten window's audience must start fresh, not resurrect whatever \
             `set_window_audience` last wrote — proving the map entry (not just the \
             signal's value) was actually removed"
        );
    }

    /// A minimal `EditorsViewModel` for wiring-only tests that just need a real
    /// instance to pass through `build_window_teardown` — mirrors
    /// `editors.rs`'s own private test helper (not reachable from here), kept
    /// deliberately small since nothing here exercises editor behaviour.
    fn test_editors_view_model(app_ctx: &Rc<frontend::AppContext>) -> EditorsViewModel {
        let bundle = || crate::view_models::EditorTypography {
            font_family: Signal::new("Literata".to_string()),
            size: Signal::new(1.0),
            line_height: Signal::new(1.5),
            first_line_indent: Signal::new(0.0),
            para_spacing_before: Signal::new(0.0),
            para_spacing_after: Signal::new(0.0),
        };
        let typography = crate::view_models::EditorTypographySet {
            scene: bundle(),
            synopsis: bundle(),
            notes: bundle(),
            corkboard: bundle(),
            distraction_free: bundle(),
        };
        let ids = crate::app_ids::AppIds::new();
        let save_state = crate::view_models::SaveStateViewModel::new(app_ctx.clone(), ids.clone());
        EditorsViewModel::new(
            app_ctx.clone(),
            Signal::new(700.0),
            Signal::new(true),
            Signal::new(crate::view_models::SynopsisPlacement::default()),
            Signal::new(crate::SYNOPSIS_SIDE_WIDTH_DEFAULT),
            typography,
            crate::view_models::TypewriterSettings::off(),
            crate::view_models::CaretHighlightSettings::off(),
            crate::view_models::EditorViewMemory::detached(false),
            crate::view_models::CorkboardDefaults::detached(),
            ids.clone(),
            crate::models::OpenDocsStore::new(app_ctx.clone()),
            Signal::new(false),
            save_state,
            Signal::new(false),
            crate::view_models::TreeExpansionViewModel::new(
                app_ctx.clone(),
                ids.clone(),
                crate::models::TreeExpansionService::in_memory_default(),
            ),
            Signal::new(false),
            Signal::new(620.0),
            crate::view_models::GoAvailability::new(),
            crate::view_models::FormatViewModel::detached(),
        )
    }
}
