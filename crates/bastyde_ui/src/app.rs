// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The application body: a `DockingLayout` whose leading dock is the binder
//! tree and whose center is a `TabWidget` of editor tabs (Phase 3), with a thin
//! status bar underneath. The window chrome (custom `TitleBar` + hamburger menu)
//! lives at the window root in `main.rs`.
//!
//! Clicking a binder item opens (or focuses) its editor tab via the tree's
//! `KeyedSelectionModel<NodeId>` selection signal.
//!
//! Plain builder calls rather than `bati!`: the docking/tab/editor widgets are
//! generic over closures, which the DSL doesn't express cleanly. See
//! `settings_panel.rs` for the `bati!` style.

mod commands;
mod wiring;

use std::rc::Rc;

use bastyde::core::DragPayload;
use bastyde::core::modal::{ModalCloseBehavior, ModalPresentation, ModalRequest};
use bastyde::core::widget::WidgetPlacement;
use bastyde::prelude::*;
use bastyde::settings::{Reloadable, SettingsExt, SettingsRegistry};
use bastyde::tokens::SurfaceRole::Hover;
use bastyde::widgets::{
    Divider, DockCorner, DockOpenLocation, DockRail, DockRailItemSize, DockSide, DockWidgetId,
    DockingLayout, DropRegion, DropTarget, DropTargetVariant, EventContextMessageBoxExt, Expand,
    HStack, IconButton, IconButtonSize, MessageBox, MessageBoxButton, MessageBoxButtons,
    NotificationArchiveModel, RowDragData, Spacer, Splitter,
    StandardButton, StatusBar, TabBarVisibility, TabWidget, TextWidget, Toast, ToastAction,
    ToastAudience, ToastRegistry, VStack,
};

use frontend::AppContext;
use frontend::commands::{
    binder_commands, undo_redo_commands, work_commands, work_management_commands,
};
use frontend::common::direct_access::binder::BinderRelationshipField;
use frontend::common::direct_access::work::WorkRelationshipField;
use frontend::common::event::{
    DirectAccessEntity, EntityEvent, Event, LongOperationEvent, Origin, WorkManagementEvent,
};
use frontend::work_management::{CloseWorkDto, LoadWorkDto, NewWorkDto};

use crate::app_ids::AppIds;
use crate::export::panel::ExportPanel;
use crate::models::TreeNode;
use crate::panels::new_work::NewWorkPanel;
use crate::sessions::{StackTeardown, WindowTeardown, WorkRegistry, WorkSession};
use crate::settings::SettingsPanel;
use crate::singles::SingleSmartPunctuation;
use crate::text_replacement::typography::SmartPunctuationFlags;
use crate::toast_scope::{ToastWorkExt, work_scoped_toast_id};

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
    BackupSchedulerViewModel, BackupSettingsViewModel, DeferredResume, EditorsViewModel,
    ExportViewModel, OutlineViewModel, PendingSwitch, ProjectSwitchViewModel, SaveAsViewModel,
    SearchReplaceViewModel, SettingsViewModel, Side, SpinnerGate, UnsavedDecision,
    unsaved_decision,
};

/// Build one editor pane's `TabWidget`: dynamic tabs, cross-pane migration
/// (`accept_external_tabs` + `on_tab_received` dedup + `on_transfer_out`
/// collapse), close, and `trailing` in the tab-strip trailing slot. Shared by
/// both panes so their chrome can't drift.
fn build_pane_tabs(
    editors: &EditorsViewModel,
    side: Side,
    trailing: impl Widget + 'static,
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
        .bar_visibility(TabBarVisibility::Always)
        .compact_bar()
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
/// `welcome.show` (aliased to `work.close` — see its action below) all set
/// this; `App` kicks the save, and the SaveWork-completion event performs the
/// action — so the async save is awaited.
///
/// Two outcomes today. Every guarded close of a project window that still
/// goes through `close_window()` (title-bar X, Alt+F4, Ctrl+W, File ▸ Close
/// Work, the brand icon / File ▸ Welcome…) returns to the Launcher — see
/// [`close_work_and_return_to_launcher`]. Ctrl+Q / File ▸ Quit is the one
/// exception: it never calls `close_window()` at all — `app.quit`'s action
/// runs the same unsaved-changes guard ([`guard_unsaved_exit`]) but ends in
/// [`quit_app`], which really terminates the process instead of reopening the
/// Launcher. Both outcomes share the exact same branch order
/// (`unsaved_decision`/`UnsavedDecision`); only the terminal action differs.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum PendingExit {
    #[default]
    None,
    /// Release the open project and return to the Launcher, once saved (and,
    /// if configured, once the on-close backup finishes).
    ReturnToLauncher,
    /// Release the open project and terminate the process entirely, once
    /// saved (and, if configured, once the on-close backup finishes).
    /// Unlike `ReturnToLauncher`, this does NOT open a fresh Launcher
    /// window — `quit_app` force-closes the project window with nothing
    /// reopened, so `WindowManager::is_empty()` trips and the event loop
    /// exits (bastyde-app/src/app.rs, `maybe_exit`/`event_loop.exit()`).
    /// Only valid when this project window is the sole open window, which
    /// is Skribisto's steady state.
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
}

impl PendingAction {
    /// The on-disk path this action targets — used to derive the project
    /// window's persistence id ([`crate::shell::windows::window_id_for`]) before the
    /// action itself has run.
    pub fn target_path(&self) -> &str {
        match self {
            PendingAction::Load(path) => path,
            PendingAction::New(dto) => &dto.file_name,
        }
    }
}

/// Release the open project and return to the Launcher: fires `CloseWork`
/// (releases the open-registry claim and clears `AppIds`/the tree/the
/// singles via the subscriber below), opens a fresh Launcher window, **then**
/// force-closes this project window.
///
/// Order matters: opening the Launcher before closing this window means the
/// process is never briefly windowless mid-transition — which would quit it
/// (see `main.rs`'s module docs). Callers invoking this from inside a
/// `on_close_requested` guard must return `CloseResponse::Veto` afterward:
/// this function performs the actual close itself, via `close_window_forced`,
/// rather than deferring to the guard's own return value.
pub fn close_work_and_return_to_launcher(
    app_ctx: &Rc<AppContext>,
    ids: &AppIds,
    workspace_layout: &crate::view_models::WorkspaceLayoutViewModel,
    ctx: &mut EventContext,
) {
    // Capture the desk (open tabs + docks) while the store is still alive —
    // `close_work` tears the Work subtree out *before* publishing `CloseWork`, so a
    // subscriber could no longer translate a tab into its persistable ordinal.
    // `workspace_layout` is the **calling window's own** instance, passed in
    // explicitly for the identical reason `ids` is (see the comment just below):
    // `ctx.app_state::<WorkspaceLayoutViewModel>()` can only ever answer with
    // whichever window built `main`'s bootstrap session.
    capture_workspace_layout(workspace_layout);
    // Phase 0 (backend): `close_work` now takes a `CloseWorkDto{work_id}` — the
    // caller names which Work to close rather than the backend picking one.
    // Resolved from `ids` — the **calling window's own** `AppIds`, passed in
    // explicitly rather than `ctx.app_state::<AppIds>()` (multi-Work migration:
    // `app_state` is one process-wide slot fixed at builder time, before any
    // window exists — it can never answer "this window's Work" once a second
    // Work's window is open; reading it here would close the WRONG Work).
    // Skipped entirely when none is open, the same no-op today's unconditional
    // call silently was against an empty store.
    if let Some(work_id) = ids.work_id.get() {
        let _ = work_management_commands::close_work(app_ctx, &CloseWorkDto { work_id });
    }
    // Scope E fix: with M Works open, a DIFFERENT window's own close-to-launcher
    // may already have opened the Launcher (bastyde's `WindowManager::create_window`
    // has no string-id dedup of its own — it just overwrites `string_to_id`, so a
    // second `open_window` with the same `.id(LAUNCHER_WINDOW_ID)` would spawn a
    // SECOND, orphaned Launcher window, violating "the Launcher is the one
    // singleton window"). Reuse and focus the existing one if it's already open.
    match ctx.find_window(crate::shell::windows::LAUNCHER_WINDOW_ID) {
        Some(existing) => ctx.focus_window(existing),
        None => {
            ctx.open_window(crate::shell::windows::launcher_window_config(
                app_ctx.clone(),
            ));
        }
    }
    ctx.close_window_forced();
}

/// Release the open project and terminate the process — the `Quit` sibling
/// of [`close_work_and_return_to_launcher`]. Deliberately does NOT open a
/// fresh Launcher window: once this (normally sole) window force-closes,
/// `WindowManager::is_empty()` trips and the event loop exits for real — the
/// framework's only process-exit mechanism (there is no
/// `EventContext::quit()`/`terminate()`).
///
/// Callers invoking this from inside a close guard must return
/// `CloseResponse::Veto` afterward, exactly like its Launcher-returning
/// sibling: this function performs the actual close itself, via
/// `close_window_forced`, rather than deferring to the guard's own return
/// value.
pub fn quit_app(
    app_ctx: &Rc<AppContext>,
    ids: &AppIds,
    workspace_layout: &crate::view_models::WorkspaceLayoutViewModel,
    ctx: &mut EventContext,
) {
    // Persist the desk before the store is torn down — see
    // [`close_work_and_return_to_launcher`].
    capture_workspace_layout(workspace_layout);
    // See `close_work_and_return_to_launcher`'s identical comment above.
    if let Some(work_id) = ids.work_id.get() {
        let _ = work_management_commands::close_work(app_ctx, &CloseWorkDto { work_id });
    }
    ctx.close_window_forced();
}

/// Persist the open project's workspace layout (open tabs + dock arrangement)
/// through `workspace_layout` — the **calling window's own** [`WorkspaceLayoutViewModel`]
/// handle, passed in explicitly rather than resolved via
/// `ctx.app_state::<WorkspaceLayoutViewModel>()` (multi-Work migration: that slot is
/// one process-wide registration fixed at builder time from the *first* window's
/// session — reading it here would capture window 1's desk again, on every window's
/// close/quit, and never the closing/quitting window's own). Called at each "leave
/// the project" door while its store is still alive.
///
/// **Scope C fix — no longer takes `ctx`.** `workspace_layout.capture()` and
/// `workspace_layout.capture_tree_expansion()` (see the latter's own doc for why
/// it moved here from a free function) both read entirely off this Tier-2
/// view-model's own injected/held state; neither ever needed an `EventContext`.
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
/// Phase 2 deliberately removed `load_work`'s/`new_work`'s own "close every
/// other open Work" sweep from the backend (`load_work_uc.rs`/`new_work_uc.rs`)
/// so two DIFFERENT Works can coexist in two windows. Nothing then closed the
/// SAME window's own outgoing Work for an in-place replace — every ordinary
/// New-Work-after-a-project-is-open / Open-Work-after-a-project-is-open action
/// silently leaked that Work's whole backend subtree (`Work`/`WorkInfo`/
/// `Binder`/`BinderItem`/`Content`/`DictWord`/…, never freed for the rest of
/// the process's life) and its `SpellcheckService` personal-word/mute map
/// entry (dropped only by `ProjectLifecycleViewModel::on_close`, which only a
/// real `CloseWork` event drives — an in-place switch fires none on its own).
///
/// Calling `close_work` here is what supplies that missing `CloseWork`.
/// **The caller must resolve `work_id` from the window's OWN `AppIds`
/// (`ids.work_id.get()`), captured before anything re-points it** — never
/// `ctx.app_state::<AppIds>()`, which is one process-wide slot fixed at
/// builder time from the *first* window's session (see
/// `close_work_and_return_to_launcher`'s identical warning about
/// `ctx.app_state::<AppIds>()`): with a second Work open in a second window,
/// that slot answers with the WRONG window's `work_id`,
/// and closing it would tear a sibling window's live, untouched Work out from
/// under it. Every call site today reaches this from inside that window's own
/// `App::build` (`commands::CommandDeps::ids` / `NewWorkViewModel`'s own
/// `ids`), where the correct, per-window `AppIds` is already at hand.
///
/// While `work_id` still names the outgoing Work, the ordinary guarded
/// `CloseWork` subscriber (`is_event_for_my_work`, in `App::build`) matches
/// and runs `ProjectLifecycleViewModel::on_close` exactly as a real close
/// would — releasing the open-registry claim, clearing this Work's
/// spell-check entry, and unpointing the tree/tabs/singles — before the
/// caller's own `load_work`/`new_work` seeds them again for the new Work.
/// `WorkRegistry`'s session/undo-stack bookkeeping is untouched by this (by
/// design — see `on_close`'s own doc): that half already runs from
/// `register_window`'s replace path once the new `LoadWork`/`NewWork` lands.
pub(crate) fn close_outgoing_work(app_ctx: &Rc<AppContext>, work_id: Option<u64>) {
    if let Some(work_id) = work_id {
        let _ = work_management_commands::close_work(app_ctx, &CloseWorkDto { work_id });
    }
}

/// Perform `outcome` immediately — `close_work_and_return_to_launcher` for
/// `ReturnToLauncher`, `quit_app` for `Quit`. Shared by every "user picked
/// Discard" branch in [`guard_unsaved_exit`], so discarding unsaved edits
/// always skips the on-close backup (the last-saved state is what's kept),
/// exactly as it did before the guard's three call sites were unified.
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
        PendingExit::Quit => quit_app(app_ctx, ids, workspace_layout, ctx),
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

/// Every OTHER currently-open Work's title that still has unsaved edits — the
/// input to `app.quit`'s "you have other unsaved projects open" refusal.
///
/// **Scope E — accounting for every dirty Work on Quit, per the design doc's
/// §9 q1 recommendation of ONE dialog naming every dirty Work, not just the
/// quitting window's own.** With M Works open, `app.quit` used to guard only
/// the ONE window it was invoked from (`guard_unsaved_exit`, unchanged, still
/// below) — a sibling window's own unsaved edits were never asked about, and
/// closing only the invoking window doesn't even terminate the process while
/// another remains open. Actually orchestrating a save-then-close across
/// EVERY open window is a much bigger feature this phase does not build (it
/// would need forcing an arbitrary *other* window closed, which
/// `EventContext::close_window_by_id`'s own doc says is guarded — "equivalent
/// to `close_window` when `id` is the current window's id" — for any other
/// id, so it would just re-open THAT window's own close guard/dialog, not
/// collapse into the one aggregated dialog the design doc asks for; a real
/// fix needs a `close_window_forced`-by-id bastyde does not expose today).
/// So instead: Quit safely REFUSES and names every other dirty Work when one
/// exists, rather than silently discarding it or silently doing nothing —
/// accounting for it by making it impossible to lose unnoticed. The user
/// switches to that Work's own window and saves/closes it there (where the
/// existing, correct single-Work guard already applies), then quits again.
/// THIS window's own Work is unaffected and still goes through
/// `guard_unsaved_exit` exactly as before.
///
/// A backup-mode Work is excluded: Save is off there, so its "unsaved" can
/// never reach disk regardless — the same reasoning `unsaved_decision`'s own
/// `PromptDiscardOnly` branch already applies to THIS window's Work.
fn other_dirty_work_titles(registry: &WorkRegistry, my_work_id: Option<u64>) -> Vec<String> {
    registry
        .open_work_ids()
        .into_iter()
        .filter(|&id| Some(id) != my_work_id)
        .filter_map(|id| registry.session_for(id))
        .filter(|s| s.unsaved.get() && !s.backup_mode.get())
        .map(|s| s.single_work.title().get())
        .collect()
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
fn offer_missing_dictionaries(
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
            .id(work_scoped_toast_id("dict.missing", work_id))
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
    /// (Phase 2: a second simultaneously-open Work gets its own independent
    /// `WorkSession`, not a second handle onto the first window's) and handed to
    /// `App::new` the way `outline` already is. `App::build` reads its fields
    /// straight off `self.session` instead of doing its own
    /// `ctx.app_state::<T>()` lookup for each one — the concrete piece of the
    /// migration's "resolution mechanism" (design doc §2). Every field here is
    /// *also* still registered as its own `app_state` entry (save_state
    /// excepted — see its own doc); that registration is now this window's
    /// *last write wins* — correct only because Phase 2 does not yet ship
    /// AttachExisting (a second window on an *already* open Work), so at most
    /// one window is ever mid-build with a not-yet-superseded registration.
    session: WorkSession,
    /// Built **fresh for this window** alongside `session` (see its doc) —
    /// each simultaneously-open Work gets its own outline/tree, never a second
    /// window's.
    outline: OutlineViewModel,
    /// Built **fresh for this window** alongside `session`/`outline`: bound to
    /// this window's own `ids`, so an export from this window scopes to *this*
    /// Work, not whichever Work's `App` constructed the shared registration
    /// last. See `app::commands::CommandDeps::export`'s doc.
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
    /// Built fresh alongside `session`, bound to *this* window's own `ids`/
    /// `single_work` — see `shell::windows::ProjectWindowFactory::window_config`'s
    /// doc for why these can no longer be the single `ctx.app_state`-registered
    /// instances every window used to share.
    save_as_vm: SaveAsViewModel,
    restore_vm: crate::view_models::BackupRestoreViewModel,
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
    /// (the only value reachable today — Phase 3 does not yet ship
    /// `AttachExisting`) and is written with the real assigned value by the
    /// `LoadWork`/`NewWork` subscribers in `build`, the moment `work_id` — and
    /// so this window's place among any siblings on it — becomes known.
    window_ordinal: Signal<usize>,
    /// Plain mirror of the persisted autosave setting, read by the title-bar menu
    /// (outside `App`) to hide the manual "Save" item. `App::build` mirrors the
    /// store-backed setting into it.
    autosave_menu: Signal<bool>,
    /// Plain mirror of the persisted master spell-check switch, read by the title-bar
    /// (outside `App`) for the toggle's icon + the View ▸ Check spelling checkmark.
    /// `App::build` mirrors the store-backed setting into it.
    spellcheck_menu: Signal<bool>,
    /// Live "a scene's prose is the active surface" flag, shared with the
    /// title-bar's Format menu so its entries grey out off a scene. Written by
    /// `EditorsViewModel`, which is the only thing that can compute it.
    scene_focused: Signal<bool>,
    /// `true` while the open work has edits not yet written to disk. Read by the
    /// close guard, `work.close` and the switch guard to decide whether to prompt,
    /// and by `can_save` for the Save affordances.
    ///
    /// **Derived**, not set by hand: `dirty_seq > saved_seq`, both read off the
    /// shared [`crate::view_models::SaveStateViewModel`] (Work-scoped, not owned
    /// here — see its module docs). It used to be a flag set on mutation and
    /// cleared whenever *a* save landed — which lied while a save was in flight,
    /// because typing during that save was marked clean the moment it finished,
    /// even though its snapshot never contained those edits. Save then greyed out
    /// on prose that was on no disk anywhere.
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
    /// The trash feature's shared view-model, created once on first build.
    trash: Option<crate::view_models::TrashViewModel>,
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
        export: crate::view_models::ExportViewModel,
        autosave_menu: Signal<bool>,
        spellcheck_menu: Signal<bool>,
        scene_focused: Signal<bool>,
        unsaved: Signal<bool>,
        pending_exit: Signal<PendingExit>,
        backup_mode: Signal<bool>,
        backup_context: Signal<Option<crate::backup::BackupContext>>,
        initial_action: PendingAction,
        registry: WorkRegistry,
        save_as_vm: SaveAsViewModel,
        restore_vm: crate::view_models::BackupRestoreViewModel,
        title_text: Signal<String>,
        window_ordinal: Signal<usize>,
    ) -> Self {
        Self {
            app_ctx,
            session,
            outline,
            export,
            registry,
            save_as_vm,
            restore_vm,
            title_text,
            window_ordinal,
            autosave_menu,
            spellcheck_menu,
            scene_focused,
            unsaved,
            pending_exit,
            exit_seq: Rc::new(std::cell::Cell::new(None)),
            save_spinner: Rc::new(std::cell::RefCell::new(SpinnerGate::default())),
            save_spinner_visible: Signal::new(false),
            backup_mode,
            backup_context,
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
            trash_dock: DockWidgetId::from_raw(crate::docks::TRASH_DOCK_ID),
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
            let mine =
                work_commands::get_work_relationship(ctx, &my_work_id, &WorkRelationshipField::Binders)
                    .unwrap_or_default();
            event_ids.iter().any(|id| mine.contains(id))
        }
        DirectAccessEntity::BinderTag(_) => {
            let mine =
                work_commands::get_work_relationship(ctx, &my_work_id, &WorkRelationshipField::Tags)
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
            let my_binders =
                work_commands::get_work_relationship(ctx, &my_work_id, &WorkRelationshipField::Binders)
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
fn build_stack_teardown(app_ctx: Rc<AppContext>, stack_id: Option<u64>) -> StackTeardown {
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
/// (`ToastRegistry::forget_window`) — the counterpart `set_window_audience`
/// above this window's `LoadWork`/`NewWork` subscriber calls has no forget
/// half of its own: `set_window_audience(id, None)` only clears the audience
/// *signal's value*, leaving the map entry (and its `Signal` allocation)
/// alive in `window_audiences` forever, and `window_versions` (bumped on
/// every toast) accumulates the same way. Without this, every window ever
/// opened over a session leaks one entry in each map — unbounded over a long
/// session that opens and closes many windows, the exact class of leak
/// `unregister_flush_hook` just above closes for the backup scheduler's own
/// map. `None` only in a headless/off-screen build context (no
/// `install_toast_default()` ever ran) — a safe no-op there, same guard as
/// `toast_registry`'s own doc.
fn build_window_teardown(
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
        if let Some(window) = ctx.window() {
            let os_title = window.title().clone();
            ctx.effect(&self.title_text, move |t: &String| {
                os_title.set(t.clone());
            });
        }

        // The Tier-2 per-open-Work bundle — see `sessions::WorkSession`'s module
        // doc and this struct's own field doc for why `App::build` reads these
        // straight off `session` instead of doing its own `ctx.app_state::<T>()`
        // lookup per field, the way the rest of this function used to.
        let session = self.session.clone();
        let app_ctx = self.app_ctx.clone();
        let column_width = settings.column_width();
        let show_synopsis = settings.synopsis_pane();
        let typography = settings.editor_typography();
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
        let editors = self
            .editors
            .get_or_insert_with(|| {
                EditorsViewModel::new(
                    app_ctx,
                    column_width,
                    show_synopsis,
                    typography,
                    view_memory,
                    corkboard_defaults,
                    ids,
                    docs,
                    backup_mode_for_editors,
                    save_state_for_editors,
                    scene_focused_for_editors,
                    session.tree_expansion.clone(),
                )
            })
            .clone();

        // The formatting view-model is created in `main` (the menu bar needs its
        // signals before `EditorsViewModel` exists) and re-pointed at the
        // editors here, on every build — idempotent, exactly like the
        // workspace-layout view-model's `set_editors`.
        let format = ctx
            .app_state::<crate::view_models::FormatViewModel>()
            .cloned()
            .expect("FormatViewModel registered in main");
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
        let workspace_layout = Some(session.workspace_layout.clone());
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
            let my_ids = session.ids.clone();
            ctx.subscribe_event(
                Origin::WorkManagement(WorkManagementEvent::LoadWork),
                move |e: &Event| {
                    if my_ids.is_bootstrap_or_own(&e.ids) {
                        s.restore_for_project();
                    }
                },
            );
        }
        {
            let s = search.clone();
            let my_ids = session.ids.clone();
            ctx.subscribe_event(
                Origin::WorkManagement(WorkManagementEvent::NewWork),
                move |e: &Event| {
                    if my_ids.is_bootstrap_or_own(&e.ids) {
                        s.restore_for_project();
                    }
                },
            );
        }
        {
            let s = search.clone();
            let my_ids = session.ids.clone();
            ctx.subscribe_event(
                Origin::WorkManagement(WorkManagementEvent::CloseWork),
                move |e: &Event| {
                    if my_ids.is_event_for_my_work(&e.ids) {
                        s.clear_preview();
                    }
                },
            );
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
        // The personal-dictionary view-model — wire its held list-model + single so
        // the Settings pane stays live and the editor's "Add to dictionary" reaches
        // a wired handle.
        session.user_dictionary.wire(ctx);
        // The tag palette, same reasoning: one wired instance behind the Inspector's tag
        // section, the Settings pane and every chip in the app.
        session.tags.wire(ctx);
        // The custom replacement lexicon, same reasoning: one wired instance behind the
        // Settings pane and the editor's typing session — resolved off `session`, same
        // as `smart_punctuation` above.
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
        // `main.rs`) reloads it in place the moment a peer window (another
        // Skribisto process, one per project) writes an override / policy /
        // bookkeeping change. Without this, `backup_settings`'s reads would
        // only ever see this process's own last write (reads no longer poll —
        // see `models::backup_settings_file`'s module docs). Absent registry
        // (e.g. a headless/test build with no settings bundle) is a silent
        // no-op: the service still works, peer writes just aren't picked up
        // live until this process next writes something itself.
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
        let project_switch = ctx
            .app_state::<ProjectSwitchViewModel>()
            .cloned()
            .expect("ProjectSwitchViewModel registered in main");
        project_switch.set_save_hook(Rc::new({
            let editors = editors.clone();
            move || editors.request_save()
        }));
        project_switch.set_new_work_form_hook(Rc::new({
            let app_ctx = self.app_ctx.clone();
            // THIS window's own `AppIds` — so "Create Work" (`NewWorkViewModel::create`)
            // closes THIS window's own outgoing Work, never `ctx.app_state::<AppIds>()`'s
            // stale, first-window-wins slot. See `close_outgoing_work`'s doc.
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
            session: session.clone(),
            registry: self.registry.clone(),
            outline: outline.clone(),
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
            unsaved: self.unsaved.clone(),
            backup_mode: self.backup_mode.clone(),
            pending_exit: self.pending_exit.clone(),
            autosave: settings.autosave(),
        };
        commands::register_all(ctx, &command_deps);

        // On project load: hand off to the lifecycle view-model (seed the ids, open the
        // per-Work undo stack, re-point the singles, rebuild the tree, drop stale tabs).
        // The two `BinderItem` subscribers below are not lifecycle — they run for every
        // edit, not just at project boundaries — so they stay here.
        {
            // An open tab does not follow its item by itself: `TabInfo::title` is a plain
            // string baked in at open time, and the `ContentTab` payload is built once for
            // the item's `(role, sub_role)`. So a rename must push the new caption, and a
            // Promote must rebuild the tab — otherwise it keeps showing a chapter's
            // segments and editors for what is now a Part.
            {
                let editors = editors.clone();
                ctx.subscribe_event(
                    Origin::DirectAccess(DirectAccessEntity::BinderItem(EntityEvent::Updated)),
                    move |event: &Event| editors.items_updated(&event.ids),
                );
            }
            // A hard-removed item (Delete Forever / Empty Trash of an open item)
            // must not leave a tab pointing at a vanished entity — close it.
            {
                let editors = editors.clone();
                ctx.subscribe_event(
                    Origin::DirectAccess(DirectAccessEntity::BinderItem(EntityEvent::Removed)),
                    move |event: &Event| editors.items_removed(&event.ids),
                );
            }

            // NOTE: the on-open backup trigger is fired from the SECOND `LoadWork`
            // subscriber below (T2-4), once `backup_mode` is known — firing it here,
            // before that sniff runs, could pump a freshly-opened *backup* file into the
            // real project's retention pool before anyone knew it was a backup.
            //
            // The workspace-layout restore (open tabs + docks) is likewise driven from
            // that SECOND subscriber: it already sniffs whether the file is a backup, and
            // restore needs that answer (a backup gets a clean default desk, not the
            // source project's) — so it is supplied there rather than re-sniffed here.
            // Guarded (loose form): with a second Work open in a second window, its
            // own LoadWork must not re-seed *this* window's ids/tree/singles — but
            // THIS window's own bootstrap/in-place-switch load must still seed
            // itself, at the instant its own `ids.work_id` is still `None` (see
            // `AppIds::is_bootstrap_or_own`'s docs).
            let lifecycle_load = lifecycle.clone();
            let my_ids = session.ids.clone();
            let my_session = session.clone();
            let registry_for_load = self.registry.clone();
            let toast_registry_for_load = toast_registry.clone();
            // Ingredients for this window's own `on_removed`-driven teardown —
            // see `build_window_teardown`'s doc. `editors`/`app_ctx`/
            // `backup_scheduler` are cheap `Rc`-backed clones; `window_id` is
            // `Copy`.
            let editors_for_teardown = editors.clone();
            let app_ctx_for_teardown = self.app_ctx.clone();
            let backup_scheduler_for_teardown = backup_scheduler.clone();
            // Scope D — window titles: written with whatever ordinal
            // `register_window` actually assigns below, so `window_title_text`
            // (built in `shell::windows`, over this very `Signal`) recomputes
            // reactively the instant this window's Work — and its place among
            // any siblings on it — is known.
            let window_ordinal_for_load = self.window_ordinal.clone();
            ctx.subscribe_event(
                Origin::WorkManagement(WorkManagementEvent::LoadWork),
                move |event: &Event| {
                    if !my_ids.is_bootstrap_or_own(&event.ids) {
                        return;
                    }
                    if let Some(&work_id) = event.ids.first() {
                        lifecycle_load.on_load(work_id);
                        // Advertise this Work as open (Phase 2's window-to-Work
                        // binding — see `sessions::WorkRegistry`'s module doc).
                        // Idempotent: a re-registration for a `work_id` this
                        // window already registered just bumps a refcount it
                        // will symmetrically drop once bastyde's `on_removed`
                        // hook confirms this window is gone (see
                        // `WorkRegistry::remove_window`, wired in
                        // `shell::windows`).
                        registry_for_load.register(work_id, my_session.clone());
                        // Bind THIS window (if it is a real, on-screen window —
                        // never true in a headless/off-screen build context) to
                        // the Work it just loaded, with its stack- and
                        // window-scoped teardowns. An in-place re-load/switch
                        // supersedes the previous registration and tears down
                        // whatever Work it was showing — see
                        // `register_window`'s doc.
                        if let Some(window_id) = window_id {
                            let stack_teardown = build_stack_teardown(
                                app_ctx_for_teardown.clone(),
                                my_ids.stack_id.get(),
                            );
                            let window_teardown = build_window_teardown(
                                editors_for_teardown.clone(),
                                backup_scheduler_for_teardown.clone(),
                                toast_registry_for_load.clone(),
                                window_id,
                                my_ids.stack_id.get(),
                            );
                            let ordinal = registry_for_load.register_window(
                                window_id,
                                work_id,
                                stack_teardown,
                                window_teardown,
                            );
                            window_ordinal_for_load.set(ordinal);
                            // Bind this window's toast/bell audience to the Work
                            // it just loaded — the same `work_id` every Work-scoped
                            // toast in this crate routes on
                            // (`crate::toast_scope::ToastWorkExt`), so a toast
                            // raised for this Work always lands in exactly the
                            // window(s) showing it. Superseding a previous
                            // audience (an in-place Load/New/switch) is exactly
                            // right: this window no longer shows the old Work, so
                            // its toasts/bell must stop matching here too.
                            if let Some(reg) = &toast_registry_for_load {
                                reg.set_window_audience(
                                    window_id,
                                    Some(ToastAudience::new(work_id)),
                                );
                            }
                        }
                    }
                },
            );
        }

        // Detect "a backup file was opened" and enter backup mode. A separate
        // `subscribe_event_with_ctx` (needs an `EventContext` to present the choice
        // modal) reads the just-loaded path and sniffs its manifest. Opening a
        // backup always happens in its own process (the redirect in the open entry
        // points), so this only ever fires in a window dedicated to that backup.
        {
            let app_ctx = self.app_ctx.clone();
            let ids = ids.clone();
            let tree_expansion = session.tree_expansion.clone();
            let backup_mode = self.backup_mode.clone();
            let backup_context = self.backup_context.clone();
            let restore_vm = restore_vm.clone();
            let single_work = single_work.clone();
            let backup_settings = backup_settings.clone();
            let backup_scheduler = backup_scheduler.clone();
            let workspace_layout = workspace_layout.clone();
            // A saved per-work layout serialized before the trash dock existed
            // won't contain it; re-mounting it after restore keeps the trash panel
            // reachable in every project (and it re-saves with trash thereafter).
            let trash_docking = outline.docking();
            // The outline's tree model, for re-applying its remembered chevrons below.
            let outline_model = outline.model();
            let trash_dock = self.trash_dock;
            let outline_dock = outline.dock_id();
            let session_for_nudge = session.clone();
            ctx.subscribe_event_with_ctx(
                Origin::WorkManagement(WorkManagementEvent::LoadWork),
                move |e: &Event, c: &mut EventContext| {
                    // Guarded (strict form): by the time this fires, the first
                    // `LoadWork` subscriber above (registered earlier in this same
                    // `build`, so it always runs first) has already re-seeded
                    // `ids.work_id` for THIS window's own load — so a sibling
                    // window's Load, which never touches this window's `ids`,
                    // reliably fails this check instead of popping this window's
                    // backup-choice modal / nudge toast for someone else's project.
                    if !ids.is_event_for_my_work(&e.ids) {
                        return;
                    }
                    // Resolved through the Phase-1 seam (`ids.work_info_id`, already
                    // re-seeded by the first `LoadWork` subscriber above — registered
                    // earlier in this same `build`, so it always runs first),
                    // rather than `get_all_work_info(&app_ctx)`'s first entry: see
                    // `main::current_project_path`'s doc for why that stopped being
                    // a safe stand-in for "this window's project" once the backend
                    // scoped `WorkInfo` to support more than one open `Work`.
                    let path = ids.work_info_id.get().and_then(|id| {
                        frontend::commands::work_info_commands::get_work_info(&app_ctx, &id)
                            .ok()
                            .flatten()
                            .and_then(|wi| wi.file_name)
                    });
                    // Sniff the manifest once: drives both the backup-mode branch
                    // below and the workspace-layout restore.
                    let backup = path.as_deref().and_then(crate::backup::backup_context_for);
                    // Restore the desk now that backup-ness is known: a backup gets a
                    // clean default desk (its saved layout is the *source's*), a normal
                    // project its saved tabs + docks. The first `LoadWork` subscriber
                    // has already re-seeded the ids/singles and cleared the old tabs.
                    if let Some(layout) = &workspace_layout {
                        layout.restore(backup.is_some());
                    }
                    // …and the outline's remembered chevrons, from the same moment and
                    // for the same reason: a backup shares its source project's uid, so
                    // its saved expansion is the *source's* and it gets the default tree
                    // instead. Keyed by durable uid, so what was written is what is read
                    // — no translation against the freshly re-minted store ids.
                    if backup.is_none() {
                        let remembered = tree_expansion.outline_expanded();
                        if !remembered.is_empty() {
                            outline_model.set_expanded_keys(&remembered);
                        }
                    }
                    // Ensure the trash dock survives restoring a pre-trash layout.
                    // `open_dock` selects the new tab, so re-reveal the outline to
                    // keep it the foreground leading panel on launch (the app default).
                    if !trash_docking.dock_open_signal(trash_dock).get() {
                        trash_docking.open_dock(
                            trash_dock,
                            DockOpenLocation::side(DockSide::Leading).new_tab(),
                        );
                        trash_docking.reveal_dock(outline_dock);
                    }
                    match backup {
                        Some(bc) => {
                            backup_mode.set(true);
                            backup_context.set(Some(bc.clone()));
                            let restore = restore_vm.clone();
                            let backup_mode_for_panel = backup_mode.clone();
                            let backup_context_for_panel = backup_context.clone();
                            c.present_modal(
                                ModalRequest::deferred(move |t| {
                                    t.add(crate::backup::choice_panel::BackupChoicePanel::new(
                                        restore.clone(),
                                        bc.clone(),
                                        backup_mode_for_panel.clone(),
                                        backup_context_for_panel.clone(),
                                    ))
                                })
                                .presentation(ModalPresentation::InTree)
                                .title(tr!(backup_choice_title()))
                                .close_behavior(ModalCloseBehavior::Manual)
                                .size(560, 320),
                            );
                        }
                        None => {
                            // A normal project — never in backup mode.
                            backup_mode.set(false);
                            backup_context.set(None);
                            // Take an on-open backup now (T2-4) — `backup_mode` is
                            // known false at this point, so a freshly-opened
                            // *backup* file (the `Some(bc)` arm above) can never be
                            // pumped into the real project's retention pool before
                            // anyone knew it was a backup.
                            backup_scheduler.on_open();
                            // One-time "no backups configured" nudge (only when the
                            // effective policy has every trigger off — a project on
                            // defaults still backs up on close, so it never nags).
                            let uid = single_work.unique_id().get();
                            if crate::models::uid_is_usable(&uid)
                                && backup_settings.effective_for(&uid).is_effectively_off()
                                && !backup_settings.was_nudged(&uid)
                            {
                                if let Some(p) = path.as_deref() {
                                    backup_settings.mark_nudged(&uid, p);
                                }
                                let session_for_action = session_for_nudge.clone();
                                // Work-scoped: this project's own backup policy.
                                c.show_toast(
                                    Toast::warning(tr!(backup_nudge_text()))
                                        .target_work(ids.work_id.get())
                                        .action(ToastAction::primary(
                                            tr!(backup_nudge_action()),
                                            move |c| {
                                                let session_for_action =
                                                    session_for_action.clone();
                                                c.present_modal(
                                                    ModalRequest::deferred(move |t| {
                                                        t.add(SettingsPanel::open_to_backup(
                                                            session_for_action,
                                                        ))
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
                        }
                    }
                },
            );
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

        // On new work: same seeding as load (a project is now open), then write
        // the freshly-created project to the chosen path immediately — a
        // create-and-save. `save_to_disk` resolves the target + shape from the
        // `WorkInfo` the use case just set (from the picker path + is_folder).
        // The new project isn't on disk yet, so it starts `unsaved = true`; the
        // async save is a long op, and the SaveWork-completion handler clears
        // `unsaved` only once the write actually lands — so an exit/close during
        // the in-flight write is caught by the guards instead of dropping the file.
        {
            // Guarded (loose form) — same reasoning as the LoadWork subscriber above:
            // a sibling window's NewWork must not reseed this window, but THIS
            // window's own bootstrap/in-place New must still seed itself while its
            // own `ids.work_id` is still `None`.
            let lifecycle_new = lifecycle.clone();
            let my_ids = session.ids.clone();
            let my_session = session.clone();
            let registry_for_new = self.registry.clone();
            let toast_registry_for_new = toast_registry.clone();
            // Same teardown ingredients as the LoadWork subscriber above — see
            // `build_window_teardown`'s doc.
            let editors_for_teardown = editors.clone();
            let app_ctx_for_teardown = self.app_ctx.clone();
            let backup_scheduler_for_teardown = backup_scheduler.clone();
            // Scope D — window titles: see the identical LoadWork subscriber's
            // `window_ordinal_for_load` doc above.
            let window_ordinal_for_new = self.window_ordinal.clone();
            ctx.subscribe_event(
                Origin::WorkManagement(WorkManagementEvent::NewWork),
                move |event: &Event| {
                    if !my_ids.is_bootstrap_or_own(&event.ids) {
                        return;
                    }
                    if let Some(&work_id) = event.ids.first() {
                        lifecycle_new.on_new(work_id);
                        registry_for_new.register(work_id, my_session.clone());
                        if let Some(window_id) = window_id {
                            let stack_teardown = build_stack_teardown(
                                app_ctx_for_teardown.clone(),
                                my_ids.stack_id.get(),
                            );
                            let window_teardown = build_window_teardown(
                                editors_for_teardown.clone(),
                                backup_scheduler_for_teardown.clone(),
                                toast_registry_for_new.clone(),
                                window_id,
                                my_ids.stack_id.get(),
                            );
                            let ordinal = registry_for_new.register_window(
                                window_id,
                                work_id,
                                stack_teardown,
                                window_teardown,
                            );
                            window_ordinal_for_new.set(ordinal);
                            // See the identical LoadWork subscriber above for why.
                            if let Some(reg) = &toast_registry_for_new {
                                reg.set_window_audience(
                                    window_id,
                                    Some(ToastAudience::new(work_id)),
                                );
                            }
                        }
                    }
                },
            );
        }

        // After a project becomes live (Load or New), offer any missing dictionaries its
        // declared languages need. Registered *after* the spell-wiring subscribers above, which
        // set the project language `offer_missing_dictionaries` reads — so it runs once those
        // have populated it. Needs an `EventContext` (to raise the toast), hence a separate
        // `subscribe_event_with_ctx` per event.
        //
        // Guarded (strict form): by the time either event fires, this window's own
        // `on_load`/`on_new` subscriber above (registered earlier in this same
        // `build`) has already seeded `ids.work_id` — so a sibling window's Load/New
        // reliably fails this check instead of popping a duplicate "install a
        // dictionary" toast in every other open window.
        for event in [WorkManagementEvent::LoadWork, WorkManagementEvent::NewWork] {
            let docs = spell_docs.clone();
            let dictionaries = dictionaries.clone();
            let my_ids = session.ids.clone();
            let session_for_toast = session.clone();
            ctx.subscribe_event_with_ctx(
                Origin::WorkManagement(event),
                move |e: &Event, c: &mut EventContext| {
                    if my_ids.is_event_for_my_work(&e.ids) {
                        offer_missing_dictionaries(&docs, &dictionaries, &session_for_toast, c)
                    }
                },
            );
        }

        // The search corpus cache holds the parsed, folded prose of every scene of the
        // project that is open. When a project is **replaced or closed**, that prose is
        // gone from the store — and because the cache is content-addressed, its keys are
        // strings nothing will ever ask for again. Left alone it is dead weight: ~19 MB
        // for a 300k-word novel, freed only when the 128 MB ceiling eventually trips.
        //
        // This is *not* a correctness hook. The cache cannot go stale (edited prose is a
        // different key), which is the whole point of keying it on the text. It is purely
        // about not carrying the previous manuscript around.
        //
        // All three events, because all three replace or drop the open project: New Work
        // and Open Work (and the switcher's "Open here", and the import toast's "Open
        // now") both go through `load_work`/`new_work`, which close the previous Work in
        // the backend rather than via a UI-side `close_work`.
        //
        // Deliberately left **unguarded** (multi-Work migration): this cache is Tier-1
        // (process-wide) by nature, not per-Work — with a second Work now able to be
        // open at the same time, a sibling Work's Load/New/Close still safely clears
        // it, because the cache is content-addressed and cannot go stale. Over-clearing
        // on a sibling's event only costs this Work's next search a re-parse; it is not
        // a correctness bug, so it gets no `is_event_for_my_work` guard.
        for event in [
            WorkManagementEvent::LoadWork,
            WorkManagementEvent::NewWork,
            WorkManagementEvent::CloseWork,
        ] {
            ctx.subscribe_event(Origin::WorkManagement(event), move |_event: &Event| {
                frontend::search_management::corpus_cache::clear();
            });
        }

        // On work close: forget the ids, empty the tree, drop the tabs, and clear
        // the singles (the store no longer holds the work).
        //
        // Guarded (strict form): a sibling window's CloseWork must never tear down
        // THIS window's own live Work. `subscribe_event_with_ctx` (not the plain
        // form the single-Work version used) so this can also force-dismiss any
        // modal this window has open — the on_close modal-dismissal gap named in
        // the migration design doc: without it, a Settings pane (or any other
        // InTree modal) left open over the outgoing Work would keep a live handle
        // into a session whose backing rows this teardown just freed (`ids.clear()`,
        // the singles unpointing below).
        //
        // Deliberately does **not** touch `WorkRegistry`'s session/window
        // bookkeeping or delete the undo stack any more — both moved to
        // `WorkRegistry::remove_window` (driven by bastyde's `on_removed`
        // window-teardown hook — see `shell::windows`'s wiring and
        // `build_stack_teardown`'s doc) for a real window close, and to
        // `WorkRegistry::register_window`'s own replace path for an in-place
        // switch (`CloseWork` never fires for that case — see
        // `ProjectSwitchViewModel`'s doc). Those two are the only reliable
        // answer to "am I the last window on this Work" once Phase 3 ships
        // `AttachExisting` (a second window sharing one Work) — deciding it
        // here, at `CloseWork` time, would already be too early for a window
        // that isn't closing itself (a sibling's own `CloseWork` subscriber
        // firing the same event).
        {
            let lifecycle_close = lifecycle.clone();
            let my_ids = session.ids.clone();
            ctx.subscribe_event_with_ctx(
                Origin::WorkManagement(WorkManagementEvent::CloseWork),
                move |event: &Event, c: &mut EventContext| {
                    if !my_ids.is_event_for_my_work(&event.ids) {
                        return;
                    }
                    c.dismiss_top_overlay();
                    lifecycle_close.on_close();
                },
            );
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
        // A hidden synopsis pane should not cost a full re-tokenise of its (often huge) text on
        // every re-attach. Mirror the global synopsis-pane setting into the store, which puts each
        // open doc's synopsis spell session to sleep while the pane is hidden and wakes it (with
        // one catch-up rebuild) when it returns. Seed it before the first document opens.
        {
            let docs = spell_docs.clone();
            docs.set_synopsis_visible(settings.synopsis_pane().get());
            ctx.effect(&settings.synopsis_pane(), move |v| {
                docs.set_synopsis_visible(*v)
            });
        }
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
        // The window close guard (in `main`) and the `work.close` action set
        // `pending_exit`; that asks for a disk save and remembers the edit sequence
        // it will cover. The close is performed only once *that* sequence is on disk
        // (below) — so the async write is awaited, never raced.
        {
            let editors = editors.clone();
            let exit_seq = self.exit_seq.clone();
            let pending = self.pending_exit.clone();
            ctx.effect(&self.pending_exit, move |pe| {
                if *pe == PendingExit::None {
                    return;
                }
                match editors.request_save() {
                    Some(covers) => exit_seq.set(Some(covers)),
                    // The command could not be issued, so no operation exists — no
                    // completion and no failure event will ever arrive. Leaving the
                    // close armed on a write that will never happen would make
                    // Ctrl+W / Ctrl+Q / the window's X do nothing at all, forever.
                    // Disarm instead: the window stays open and usable, and the next
                    // close attempt retries. (No toast: an `effect` has no
                    // `EventContext`. `save_work` can only fail to *start* on a
                    // poisoned store lock, at which point this log line is the least
                    // of it — every other path that can reach a context does toast.)
                    None => {
                        exit_seq.set(None);
                        pending.set(PendingExit::None);
                        eprintln!("skribisto: could not start the save for a deferred close");
                    }
                }
            });
        }
        // Both deferred flows — the close above and a parked project switch (New
        // Work / Open Work / "Open here" / the import toast, where the user chose
        // "Save") — resume here, off the **long-operation** events.
        //
        // They wait on the edit *sequence* their save covers, not on "a save
        // finished": `SaveQueue` runs one `save_work` at a time and coalesces, so
        // the op that finally carries these edits may be a follow-up issued when an
        // already-in-flight save landed. That in-flight save's snapshot can predate
        // our flush, and resuming on it would wipe the store while the last sentence
        // typed was still unwritten.
        {
            let editors = editors.clone();
            let pending = self.pending_exit.clone();
            let exit_seq = self.exit_seq.clone();
            let scheduler = backup_scheduler.clone();
            let switch = project_switch.clone();
            let workspace_layout = workspace_layout.clone();
            let ids = ids.clone();
            ctx.subscribe_event_with_ctx(
                Origin::LongOperation(LongOperationEvent::Completed),
                move |e: &Event, c| {
                    // Ours? (A backup's, an import's or a Save As's completion is
                    // their own view-model's business.) This also issues the
                    // follow-up save when edits arrived while that one was running.
                    let Some(landed) = editors.on_save_completed(e) else {
                        return;
                    };
                    // The store now matches disk — keep the persisted desk fresh and
                    // ordinal-aligned with the file, so a later hard exit (crash /
                    // kill, with no graceful close to capture at) still restores. A
                    // no-op if a follow-up save is due (still dirty): `capture` self-
                    // gates on `is_unsaved`, so only the final clean save persists.
                    if let Some(layout) = &workspace_layout {
                        layout.capture();
                    }
                    // The precedence rule (a close outranks a switch, even an
                    // uncovered one) lives in `view_models::save_queue` as a pure
                    // decision, so it is unit-tested rather than only reachable
                    // through a real async save.
                    let saved = landed.saved_seq;
                    let pe = pending.get();
                    match crate::view_models::resume_deferred(
                        landed.follow_up_failed,
                        saved,
                        pe != PendingExit::None,
                        exit_seq.get(),
                    ) {
                        DeferredResume::Abandon => abandon_deferred(
                            c,
                            &pending,
                            &exit_seq,
                            &switch,
                            None,
                            ids.work_id.get(),
                        ),
                        DeferredResume::Close => {
                            pending.set(PendingExit::None);
                            exit_seq.set(None);
                            switch.cancel();
                            // Saved and consistent — now take the on-close backup (if
                            // configured) and then perform the deferred close. When
                            // no on-close backup applies, `on_close_flow` closes at
                            // once.
                            scheduler.on_close_flow(c, pe);
                        }
                        DeferredResume::Wait => {}
                        DeferredResume::Switch => switch.on_saved(c, saved),
                    }
                },
            );
        }
        // **A failed save is always a toast.** The write is asynchronous, so the only
        // other way the user could learn of it is the Save affordance staying live —
        // which is invisible under autosave, where Save is hidden entirely. It used
        // to be reported nowhere at all.
        //
        // Exactly one toast, and it says what was lost *besides* the write: if a
        // close or a project switch was parked behind that save, it is dropped and
        // the message says so (the deferred command silently never happening is the
        // more confusing half). The project itself is untouched — still open, still
        // dirty — so nothing is lost by staying put. Only *our* save's failure
        // counts here; a failing backup / import / Save As is reported by its own
        // view-model.
        {
            let editors = editors.clone();
            let pending = self.pending_exit.clone();
            let exit_seq = self.exit_seq.clone();
            let switch = project_switch.clone();
            let ids = ids.clone();
            ctx.subscribe_event_with_ctx(
                Origin::LongOperation(LongOperationEvent::Failed),
                move |e: &Event, c| {
                    let Some(error) = editors.on_save_failed(e) else {
                        return;
                    };
                    abandon_deferred(
                        c,
                        &pending,
                        &exit_seq,
                        &switch,
                        Some(&error),
                        ids.work_id.get(),
                    );
                },
            );
        }
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
            ctx.register_action_global(Action::new("backups.show").on_invoke(move |_i, c| {
                let uid = single_work.unique_id().get();
                let Some(path) = single_work_info.file_name().get() else {
                    return;
                };
                let dirs = backup_settings.effective_for(&uid).destinations;
                let work_id = single_work.id();
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
        let active_item = editors.active_item();
        let split_active = editors.split_active();

        // ── Center: split editor — two panes in a Splitter ───────────────────
        // Each pane is a zoned `DropTarget` wrapping a `TabWidget`, so a binder
        // row dragged from the outline opens on the pane it's dropped over. The
        // primary pane's `Trailing` (right-edge) zone opens to the side; it
        // deactivates once split (`enabled(split_active.not())`). Closing a tab
        // saves it first (`on_close` is a pre-close intercept). Tabs migrate
        // between panes (`accept_external_tabs` + `on_tab_received` dedup).
        let split_button = {
            let editors = editors.clone();
            IconButton::new(crate::icons::editor::split())
                .tooltip(tr!(split_editor()))
                .icon_role(split_active.map(|on| {
                    if *on {
                        TextRole::Accent
                    } else {
                        TextRole::Primary
                    }
                }))
                .on_activate_fn(move |_ctx| editors.toggle_split())
        };
        let close_split_button = {
            let editors = editors.clone();
            IconButton::new(crate::icons::editor::close_split())
                .tooltip(tr!(close_split_view()))
                .on_activate_fn(move |_ctx| editors.close_split())
        };

        // Follow keyboard focus, not just tab selection: when focus enters a
        // pane's content (e.g. clicking into its editor), mark that pane focused so
        // the Inspector + open-item marker track the pane you're actually working
        // in. `focus_within` is set by the framework when a descendant has focus.
        let primary_focus = Signal::new(false);
        let secondary_focus = Signal::new(false);
        for (sig, side) in [
            (&primary_focus, Side::Primary),
            (&secondary_focus, Side::Secondary),
        ] {
            let editors = editors.clone();
            ctx.effect(sig, move |&focused| {
                if focused {
                    editors.set_focused(side);
                }
            });
        }

        let primary_pane = {
            let e = editors.clone();
            DropTarget::new()
                .variant(DropTargetVariant::Prominent)
                .zone_size_factor(0.3)
                .accept_when(|p| {
                    p.get_typed::<RowDragData<TreeNode>>()
                        .is_some_and(|d| d.is_export())
                })
                .region(DropRegion::Center, |z| {
                    z.hint(TextWidget::new(tr!(drop_open_here())))
                })
                .region(DropRegion::Trailing, |z| {
                    z.hint(TextWidget::new(tr!(drop_open_to_side())))
                        .enabled(split_active.not())
                })
                .on_region_drop(move |region, payload, _pos, _ctx| {
                    drain_dropped(payload, |item_id, title| match region {
                        DropRegion::Trailing => e.open_to_side(item_id, title),
                        _ => e.open_in(Side::Primary, item_id, title),
                    })
                })
                .child(build_pane_tabs(&editors, Side::Primary, split_button))
                .focus_within(primary_focus.clone())
        };

        let secondary_pane = {
            let e = editors.clone();
            DropTarget::new()
                .variant(DropTargetVariant::Prominent)
                .accept_when(|p| {
                    p.get_typed::<RowDragData<TreeNode>>()
                        .is_some_and(|d| d.is_export())
                })
                .region(DropRegion::Center, |z| {
                    z.hint(TextWidget::new(tr!(drop_open_here())))
                })
                .on_region_drop(move |_region, payload, _pos, _ctx| {
                    drain_dropped(payload, |item_id, title| {
                        e.open_in(Side::Secondary, item_id, title)
                    })
                })
                .child(build_pane_tabs(
                    &editors,
                    Side::Secondary,
                    close_split_button,
                ))
                .focus_within(secondary_focus.clone())
        };

        let center = Splitter::new(editors.splitter())
            .pane(primary_pane)
            .pane(secondary_pane);

        // ── Leading dock: the binder tree, fronted by a VS Code-style activity
        //    bar (icon rail). The OutlineViewModel owns the DockingModel; the
        //    dock content itself lives in `docks::outline`. ───────────────────
        // The trailing side hosts the context Inspector (a rail dock, like the
        // outline), sized + rail-fronted on the shared DockingModel.
        //
        // These are the DEFAULT dock config + arrangement, established **once** (on
        // first build): the shared `DockingModel` persists across widget rebuilds,
        // and the per-work restore below imports each project's saved sizes /
        // selected side-tab on `LoadWork`, so re-running these on every rebuild would
        // stomp a restored (or user-adjusted) layout. The `.dock(...)` registrations
        // on `DockingLayout::new(...)` still run every build — they rebuild the dock
        // *content*, not the arrangement.
        if !self.initial_loaded {
            let docking = outline.docking();
            docking.set_side_size(DockSide::Trailing, 300.0);
            docking.set_side_rail(DockSide::Trailing, 48.0);
            // The bottom search-preview band. The bottom-LEADING corner belongs to
            // the Leading side, so the binder column runs full height and the preview
            // spans only the width beside it. (Default is `Bottom`, i.e. a full-width
            // band under everything.)
            docking.set_side_size(DockSide::Bottom, 180.0);
            docking.set_corner(DockCorner::BottomLeading, DockSide::Leading);
            // An activity bar, not a tab strip: `set_side_rail` switches the side's
            // presentation from tabs to a rail of activity glyphs, and `Compact` keeps
            // them at the standard icon-button size so the band spends its height on
            // prose rather than on chrome.
            docking.set_side_rail(DockSide::Bottom, 36.0);
            docking.set_side_rail_size(DockSide::Bottom, DockRailItemSize::Compact);
        }
        let layout = DockingLayout::new(outline.docking())
            .rail(
                DockRail::new(DockSide::Leading)
                    .background(SurfaceRole::Main)
                    .divider(),
            )
            .rail(
                DockRail::new(DockSide::Trailing)
                    .background(SurfaceRole::Main)
                    .divider(),
            )
            .rail(
                DockRail::new(DockSide::Bottom)
                    .background(SurfaceRole::Main)
                    .divider(),
            )
            .center(center)
            .dock(crate::docks::outline::outline_dock(
                outline.clone(),
                self.app_ctx.clone(),
                on_open.clone(),
                active_item.clone(),
            ))
            .dock(crate::docks::inspector::inspector_dock(
                self.app_ctx.clone(),
                outline.clone(),
                active_item,
                self.inspector_dock,
                session.tags.clone(),
                session.mention_index.clone(),
                session.open_docs.clone(),
            ))
            .dock(crate::docks::format::format_dock(
                format.clone(),
                self.format_dock,
            ))
            .dock(crate::docks::search::search_dock(
                search.clone(),
                self.search_dock,
            ))
            .dock(crate::docks::search_preview::search_preview_dock(
                search.clone(),
                self.preview_dock,
            ))
            .dock(crate::docks::trash::trash_dock(
                trash.clone(),
                self.trash_dock,
                on_open,
            ));
        // First-build-only default arrangement (see the config block above on why
        // it must not re-run on rebuilds).
        if !self.initial_loaded {
            // The leading side hosts TWO activity docks (binder + search) as separate
            // switchable rail tabs — VS Code style: the rail shows both glyphs, and
            // selecting one shows only its panel. `.new_tab()` is what makes them
            // distinct tabs; the default `side()` placement *stacks* (a vertical
            // split showing both at once, which starves the binder). The binder is
            // revealed last so it is the selected leading panel on launch.
            let docking = outline.docking();
            docking.open_dock(
                outline.dock_id(),
                DockOpenLocation::side(DockSide::Leading).new_tab(),
            );
            docking.open_dock(
                self.search_dock,
                DockOpenLocation::side(DockSide::Leading).new_tab(),
            );
            docking.open_dock(
                self.trash_dock,
                DockOpenLocation::side(DockSide::Leading).new_tab(),
            );
            docking.reveal_dock(outline.dock_id());
            // Mount the inspector on the trailing side (otherwise the side shows the
            // empty "drop a panel here" placeholder).
            docking.open_dock(
                self.inspector_dock,
                DockOpenLocation::side(DockSide::Trailing),
            );
            // Format joins it as a second rail tab rather than a second side.
            // Inspector answers "what is this item", Format answers "how does
            // this text read" — same trailing rail, one visible at a time,
            // because a writer wants one question answered at a time and the
            // 300px side has no room to stack both.
            docking.open_dock(
                self.format_dock,
                DockOpenLocation::side(DockSide::Trailing).new_tab(),
            );
            // Inspector is the one that starts showing: it is the older habit,
            // and Format is reachable in one click on the rail.
            docking.reveal_dock(self.inspector_dock);
            // Mount the bottom preview band, then hide it: it is the transient
            // search-preview band, always hidden at start (a result click reveals it
            // thereafter). `open_dock` makes its side visible as a side effect, so the
            // hide must follow the mount — and it is *immediate* to avoid an
            // opening-then-closing flash on launch.
            docking.open_dock(self.preview_dock, DockOpenLocation::side(DockSide::Bottom));
            docking.set_side_visible_immediate(DockSide::Bottom, false);
            // Snapshot this pristine arrangement as the reset target for a project
            // that has no saved layout (so an in-place switch to an unconfigured
            // project doesn't inherit the previous one's docks).
            session
                .workspace_layout
                .set_default_docks(docking.export_state());
        }

        // ── Status bar (thin) with the notification bell ─────────────────────
        let archive = ctx
            .app_state::<Rc<NotificationArchiveModel>>()
            .cloned()
            .expect("install_toast_default registers the notification archive");
        // Status-bar dock toggles: hide/show the leading (binder) and trailing
        // (inspector) sides — like Bastyde's `docking` example.
        let dock_lead = outline.docking();
        let dock_trail = outline.docking();
        // The save indicator sits right after the binder toggle: the quiet, always-
        // there answer to "is my last paragraph on disk?" — the one thing autosave
        // mode had no way to tell you (it hides Save + Ctrl+S). Failures are toasts;
        // this is only the steady state.
        let save_indicator = crate::statusbar::save_indicator::SaveIndicator::new(
            editors.clone(),
            self.unsaved.clone(),
            settings.autosave(),
            self.backup_mode.clone(),
            // A work is open iff its `WorkInfo` shape is known (same test the File
            // menu uses to collapse its project-only items).
            single_work_info.shape().map(|s| s.is_some()),
            self.save_spinner.clone(),
            self.save_spinner_visible.clone(),
        );
        // The focused item's live word count sits right after the save glyph — the
        // quiet "how many words in this scene" a writer glances at. Counts the open
        // document's live text (so it tracks typing), off `StatsModel` over the shared
        // `OpenDocsStore`.
        let stats = crate::models::StatsModel::new(
            session.open_docs.clone(),
            editors.active_item(),
            settings.counting_method(),
        );
        let word_count_indicator = crate::statusbar::word_count_indicator::WordCountIndicator::new(
            stats.clone(),
            single_work_info.shape().map(|s| s.is_some()),
            settings.show_characters(),
        );
        // The writing session: a play/pause sprint timer + word tracker (ephemeral —
        // only its targets persist). Sits on the right of the status bar.
        let session_vm =
            crate::view_models::WritingSessionViewModel::new(stats.clone(), ctx.settings());
        let session_item = crate::statusbar::session_status_item::SessionStatusItem::new(
            session_vm.clone(),
            single_work_info.shape().map(|s| s.is_some()),
        );
        // `session.toggle` — scriptable start/pause (palette / automation); the play
        // button is the primary control. Global so it fires regardless of focus.
        {
            let vm = session_vm.clone();
            ctx.register_action_global(
                Action::new("session.toggle").on_invoke(move |_i, _c| vm.toggle()),
            );
        }
        let status = StatusBar::new().background(SurfaceRole::Main).child(
            HStack::new()
                .spacing(8.0)
                // Leading (binder) toggle on the left; trailing (inspector) toggle
                // pushed to the right next to the notification bell.
                .child(
                    IconButton::new(crate::icons::activity::sidebar_icon())
                        .size(IconButtonSize::Compact)
                        .tooltip(tr!(statusbar_toggle_outline()))
                        .on_activate_fn(move |_| dock_lead.toggle_side_visible(DockSide::Leading)),
                )
                .child(save_indicator)
                .child(word_count_indicator)
                .child(Spacer::new())
                .child(session_item)
                .child(
                    IconButton::new(crate::icons::activity::inspector_icon())
                        .size(IconButtonSize::Compact)
                        .tooltip(tr!(statusbar_toggle_inspector()))
                        .on_activate_fn(move |_| {
                            dock_trail.toggle_side_visible(DockSide::Trailing)
                        }),
                )
                .child(
                    crate::statusbar::notification_bell::NotificationBell::new(
                        archive,
                        ids.work_id.clone(),
                    )
                    .size(IconButtonSize::Compact),
                ),
        );

        // The permanent backup banner sits above everything while a backup file is
        // open (zero height otherwise).
        let backup_banner = crate::backup::banner::BackupBanner::new(
            self.backup_context.clone(),
            restore_vm.clone(),
            save_as_vm.clone(),
            single_work.clone(),
        );

        let root = ctx.add(
            VStack::new()
                .spacing(0.0)
                .child(backup_banner)
                .child(Divider::new())
                .child(Expand::new().child(layout))
                .child(status),
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
                            file_name: path.clone(),
                        },
                    ) {
                        eprintln!("skribisto: could not open '{path}': {e}");
                    }
                }
                Some(PendingAction::New(dto)) => {
                    let target = dto.file_name.clone();
                    if let Err(e) = work_management_commands::new_work(&self.app_ctx, &dto) {
                        eprintln!("skribisto: could not create '{target}': {e}");
                    }
                }
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

/// The disk write a deferred flow was waiting on will never land — it failed, or a
/// follow-up save could not even be started. Drop whatever was parked on it and say
/// so, rather than leave the user with a command that silently never happens.
///
/// **Exactly one toast**, naming what was lost *besides* the write, because the
/// deferred command not happening is the more confusing half. A close outranks a
/// switch (it is where the project was headed); with neither parked, this is a plain
/// autosave / Ctrl+S and only the write itself is reported. `error` is `None` when
/// the operation never started, so there is no message from the backend to quote.
///
/// The project is untouched — still open, still dirty — so nothing is lost by
/// staying put; the edits are exactly where the user left them.
///
/// `work_id` is this window's own `AppIds.work_id` at the moment the deferred
/// save's outcome landed — the toast is squarely about *this* window's own
/// save, so it routes here (`crate::toast_scope::ToastWorkExt`) rather than
/// to every open project.
fn abandon_deferred(
    c: &mut EventContext,
    pending: &Signal<PendingExit>,
    exit_seq: &Rc<std::cell::Cell<Option<u64>>>,
    switch: &ProjectSwitchViewModel,
    error: Option<&str>,
    work_id: Option<u64>,
) {
    if pending.get() != PendingExit::None {
        pending.set(PendingExit::None);
        exit_seq.set(None);
        switch.cancel();
        c.show_toast(
            Toast::error(match error {
                Some(e) => tr!(close_save_failed(error = e.to_string())),
                None => tr!(close_save_not_started()),
            })
            .target_work(work_id),
        );
        return;
    }
    if switch.on_save_failed(c, error) {
        return; // it toasted "…so it wasn't replaced"
    }
    // Nothing was waiting: report just the failed write.
    if let Some(e) = error {
        c.show_toast(Toast::error(tr!(save_error(error = e.to_string()))).target_work(work_id));
    } else {
        c.show_toast(Toast::error(tr!(save_not_started())).target_work(work_id));
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
/// straight to `load_work` — which is what used to throw away the open project's
/// unsaved edits without a word. The guard runs *after* the pick and *after* the
/// backup sniff, so neither cancelling the picker nor choosing a backup (which
/// opens in its own process, leaving this project alone) prompts about anything.
///
/// `ids` is THIS window's own `AppIds` — read (`.work_id.get()`) only at the
/// final `switch.request` call below, not snapshotted here, so the outgoing Work
/// it names is whatever is actually open in this window at that (later, async)
/// moment, not whatever was open when the picker was first invoked.
fn open_work_flow(switch: ProjectSwitchViewModel, ids: AppIds, ctx: &mut EventContext) {
    let req = FileDialogRequest::pick_file()
        .title("Open Skribisto work")
        .add_filter("Skribisto work", &["skrib"]);
    let _ = ctx.pick_file(req, move |res, ectx| {
        if let FileDialogResult::File(Some(path)) = res {
            let file = path.to_string_lossy().into_owned();
            let switch = switch.clone();
            let ids = ids.clone();
            let file_for_check = file.clone();
            ectx.spawn_local_with(
                async move {
                    spawn_blocking(move || crate::backup::is_backup_path(&file_for_check))
                        .await
                        .unwrap_or(false)
                },
                move |is_backup, ectx2| {
                    // A backup always opens in its own instance (never replacing
                    // the project in this window) — see the backup-mode invariant.
                    // Nothing here is destroyed, so there is nothing to guard.
                    if is_backup {
                        ectx2.request_activation_token_self(Box::new(move |tok| {
                            crate::shell::process::spawn_new_process(&file, tok);
                        }));
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

    // ── `other_dirty_work_titles` (Scope E — app.quit's multi-Work accounting) ──

    #[test]
    fn other_dirty_work_titles_is_empty_with_only_this_window_open() {
        let reg = WorkRegistry::new();
        let mine = crate::sessions::WorkSession::for_test();
        mine.ids.work_id.set(Some(1));
        mine.unsaved.set(true);
        reg.register(1, mine);

        assert!(
            other_dirty_work_titles(&reg, Some(1)).is_empty(),
            "this window's own dirty Work must never appear in the OTHER-Works list"
        );
    }

    #[test]
    fn other_dirty_work_titles_excludes_mine_clean_and_backup_mode_works() {
        let reg = WorkRegistry::new();

        let mine = crate::sessions::WorkSession::for_test();
        mine.ids.work_id.set(Some(1));
        mine.unsaved.set(true); // dirty, but it's MY OWN Work — must never appear
        reg.register(1, mine);

        let clean = crate::sessions::WorkSession::for_test();
        clean.ids.work_id.set(Some(2)); // unsaved stays false
        reg.register(2, clean);

        let backup = crate::sessions::WorkSession::for_test();
        backup.ids.work_id.set(Some(3));
        backup.unsaved.set(true);
        backup.backup_mode.set(true); // dirty, but Save is off here — excluded
        reg.register(3, backup);

        let dirty_other = crate::sessions::WorkSession::for_test();
        dirty_other.ids.work_id.set(Some(4));
        dirty_other.unsaved.set(true);
        reg.register(4, dirty_other);

        let others = other_dirty_work_titles(&reg, Some(1));
        assert_eq!(
            others.len(),
            1,
            "only Work 4 qualifies: not mine, dirty, and not in backup mode"
        );
    }

    #[test]
    fn other_dirty_work_titles_with_no_work_of_my_own_still_finds_others() {
        // This window hasn't finished its own Load/New yet (`my_work_id = None`) —
        // a sibling Work's own dirty edits must still be reported.
        let reg = WorkRegistry::new();
        let sibling = crate::sessions::WorkSession::for_test();
        sibling.ids.work_id.set(Some(9));
        sibling.unsaved.set(true);
        reg.register(9, sibling);

        assert_eq!(other_dirty_work_titles(&reg, None).len(), 1);
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
        };
        let ids = crate::app_ids::AppIds::new();
        let save_state = crate::view_models::SaveStateViewModel::new(app_ctx.clone(), ids.clone());
        EditorsViewModel::new(
            app_ctx.clone(),
            Signal::new(700.0),
            Signal::new(true),
            typography,
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
        )
    }
}
