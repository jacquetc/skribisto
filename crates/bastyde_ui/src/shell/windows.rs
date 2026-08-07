// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Window-construction factories for the launcher-window model.
//!
//! Skribisto is **single-instance**: one process hosts the Launcher and every
//! open project window (see `main.rs` / `shell::instance`). A second launch
//! hands off to this process rather than forking. Multiple project windows —
//! different Works, or Work ▸ New Window onto the same Work — are normal.
//!
//! A **project window is only ever created once its project is already
//! known** — that's the fix for the window-geometry bug this module exists
//! to close: a project's persisted geometry keys on
//! [`window_id_for`]`(path)`, which can only be computed once a path exists.
//!
//! Two window shapes:
//!   * [`launcher_window_config`] — the Welcome UI hosted as a real window
//!     (not a modal). Opened on a bare launch (or when no argv path and no
//!     reachable recent project); closed the instant a project window opens.
//!   * [`ProjectWindowFactory`] — everything needed to build a **project**
//!     window (the editor shell: custom title bar + File menu, binder +
//!     split editor + status bar via [`App`]) — bundled once in `main` and
//!     registered as `app_state` so both the initial window and a runtime
//!     `ctx.open_window(...)` (from the Launcher once a project is
//!     picked/created) build an identical window.
//!
//! Path→window identity (`window_id_for`, [`resolve_project_window`],
//! [`open_or_focus_project`]) lives in [`super::window_ids`] and is re-exported
//! below so call sites keep a stable path.
//!
//! **Ordering invariant.** The process quits when its last window closes, so
//! every transition here opens the new window *before* closing the old one —
//! see `main.rs`'s module docs and [`crate::app::close_work_and_return_to_launcher`].
//! The one deliberate exception is `app.quit` (Ctrl+Q / File ▸ Quit —
//! registered in `app.rs`, not here): it force-closes this window with nothing
//! reopened, so the last-window-closes rule *is* the exit.

use std::rc::Rc;

use bastyde::core::menu_item_id::MenuItemId;
use bastyde::prelude::*;
use bastyde::res;
use bastyde::widgets::MenuModel;
use bastyde::widgets::primitives::icon_widget::IconMode;
use bastyde::widgets::{
    Center, CollapsePolicy, DeadZone, Expand, HStack, IconButtonSize, IconWidget, MenuBar, Padding,
    Slide, SlideEdge, TextWidget, TitleBar, VStack, WindowFrame, ZStack,
};

use frontend::AppContext;

use crate::app::{App, PendingAction, PendingExit, guard_unsaved_exit};
use crate::app_ids::AppIds;
use crate::export::split_button::ExportSplitButton;
use crate::models::{TreeExpansionService, WorkspaceLayoutService};
use crate::sessions::{WorkRegistry, WorkSession};
use crate::shell::project_switcher_button::ProjectSwitcherButton;
use crate::spellcheck::SpellcheckService;
use crate::spellcheck::toggle_button::SpellcheckToggleButton;
use crate::tabs::shared::editor::VisibleWhen;
use crate::view_models::{
    BackupSettingsViewModel, ExportViewModel, FormatViewModel, OutlineViewModel, SaveAsViewModel,
};

// Re-export path→window identity so `shell::windows::*` stays the
// stable public surface for call sites.
pub use super::window_ids::{
    LAUNCHER_WINDOW_ID, attached_window_id_for, open_or_focus_project, resolve_project_window,
    window_id_for,
};

/// Scope D — window titles. A reactive `"{Work title} — Skribisto"` (falling
/// back to plain `"Skribisto"` before a Work has finished loading/creating),
/// with a `" (Window N)"` suffix once `ordinal` says this window is not the
/// sole one showing its Work.
///
/// **Design goal: a STABLE, distinguishable string.** On Wayland a client
/// cannot position its own toplevel — pinning the binder window to a second
/// monitor is done with a KWin window rule keyed on the title text. So this
/// must (a) always name the Work the window shows, (b) never collide with a
/// sibling window on the *same* Work, and (c) never change identity out from
/// under an already-matched KWin rule just because some OTHER window closed.
/// `ordinal` (see `WorkRegistry::register_window`'s doc) is exactly that: a
/// number assigned once per window and never renumbered/reused for as long as
/// that window is open, so "Window 2" always means the same physical window
/// even after "Window 1" closes. `ordinal == 1` never shows a suffix at all —
/// the first window on a project is just the project.
///
/// Work ▸ New Window is what reaches the suffix
/// ([`attached_window_config`](ProjectWindowFactory::attached_window_config)).
/// Such a window knows its ordinal before it is built (its persistence id is
/// derived from it), so `window_ordinal` starts at the real value and the title
/// reads "(Window 2)" from the first frame rather than flickering through the
/// un-suffixed form; every other window starts at `1` and is corrected, if it
/// ever needs to be, by its own `LoadWork`/`NewWork` subscriber.
fn window_title_text(
    single_work: &crate::singles::SingleWork,
    ordinal: &Signal<usize>,
) -> Signal<String> {
    single_work.title().zip(ordinal).map(|(title, ord)| {
        let base = if title.trim().is_empty() {
            "Skribisto".to_string()
        } else {
            format!("{title} — Skribisto")
        };
        if *ord > 1 {
            format!("{base} (Window {ord})")
        } else {
            base
        }
    })
}

pub use super::launcher_window::launcher_window_config;

/// The fresh, per-window Tier-2 handles [`ProjectWindowFactory::window_config`]
/// just minted for its `WindowConfig`'s own `App` — handed back so `main.rs`
/// can (for the *initial* window only) seed the remaining `app_state`
/// registrations a few widgets still read Tier-2 state through. See
/// `window_config`'s doc for why this is a known, flagged Phase-2 gap rather
/// than a full fix.
#[derive(Clone)]
pub struct InitialWindowState {
    pub session: WorkSession,
    pub outline: OutlineViewModel,
}

#[derive(Clone)]
pub struct ProjectWindowFactory {
    app_ctx: Rc<AppContext>,
    registry: WorkRegistry,
    /// Tier-1 ingredients `WorkSession::new` needs — held here (not a
    /// pre-built session) so every call to [`Self::window_config`] can mint a
    /// **fresh** `WorkSession` for the Work that window is about to load/create
    /// (Phase 2: a second simultaneously-open Work must never share the first
    /// window's `AppIds`/singles/tag palette/dictionary/undo stack). Notably
    /// absent: `backup_mode`/`backup_context`/`unsaved` — Phase 3 moved those
    /// from a caller-supplied pair here to being minted *inside*
    /// `WorkSession::new` itself (see its module doc), so this factory no
    /// longer holds them at all; [`Self::window_config`] reads them straight
    /// off the freshly-built `session` instead. Also notably absent:
    /// `pending_exit` — the OPPOSITE fix, from a shared field here to a fresh
    /// `Signal::new(PendingExit::None)` minted per call, exactly like
    /// `scene_focused` below — it names a *window's* own close/quit
    /// resumption, not the Work's data, so it must never be shared even
    /// between two windows on the SAME Work.
    spellcheck: SpellcheckService,
    backup_settings: BackupSettingsViewModel,
    workspace_layout_service: WorkspaceLayoutService,
    tree_expansion_service: TreeExpansionService,
    autosave_menu: Signal<bool>,
    /// Plain mirror of the master spell-check switch — the title-bar toggle's icon and
    /// the View ▸ Check spelling checkmark read it. `App::build` keeps it in sync.
    spellcheck_menu: Signal<bool>,
    /// Plain mirror of the Tools ▸ Comments switch — that menu row's checkmark reads
    /// it. Shared process-wide exactly like `spellcheck_menu`: it mirrors one persisted
    /// app preference, not anything this window owns.
    comments_menu: Signal<bool>,
    /// The app-global quit sequencer, built **here** rather than per window and
    /// rather than in `main`: a quit spans every window, so two windows each
    /// running their own sequence over the same Works would prompt twice for
    /// each; and this factory already holds the only two things it needs (the
    /// registry and the autosave switch).
    quit: crate::view_models::QuitSequencer,
    // (Phase 4 removed `main_window_state`: one `Rc<RefCell<Option<WindowState>>>`
    // retargeted to the *freshest* project window, which IPC "raise" focused. It was
    // already the wrong answer with two project windows open, and single-instance
    // makes two windows the normal case. `main.rs`'s router resolves the target by
    // string id instead — which names the exact window and needs no side table.)
}

impl ProjectWindowFactory {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        app_ctx: Rc<AppContext>,
        registry: WorkRegistry,
        spellcheck: SpellcheckService,
        backup_settings: BackupSettingsViewModel,
        workspace_layout_service: WorkspaceLayoutService,
        tree_expansion_service: TreeExpansionService,
        autosave_menu: Signal<bool>,
        spellcheck_menu: Signal<bool>,
        comments_menu: Signal<bool>,
    ) -> Self {
        Self {
            quit: crate::view_models::QuitSequencer::new(
                app_ctx.clone(),
                registry.clone(),
                autosave_menu.clone(),
            ),
            app_ctx,
            registry,
            spellcheck,
            backup_settings,
            workspace_layout_service,
            tree_expansion_service,
            autosave_menu,
            spellcheck_menu,
            comments_menu,
        }
    }

    /// Build the `WindowConfig` for a project window that performs `action`
    /// once its [`App`] mounts (see `App::build`'s first-build logic) — the
    /// same "seed only after the `LoadWork`/`NewWork` subscription is live"
    /// mechanism the argv launch path has always used, now shared with the
    /// Launcher's recents / new-work / examples flows. Doing the backend
    /// mutation here instead (before the window exists) would race that
    /// subscription and silently skip seeding `AppIds`/the singles/the tree.
    ///
    /// **Phase 2**: mints a brand-new `AppIds`/`OutlineViewModel`/
    /// `ExportViewModel`/`WorkSession` for *this* call — never the previous
    /// call's — so a second simultaneously-open Work gets its own independent
    /// ids, tree, tag palette, personal dictionary and undo stack, never a
    /// second handle onto the first window's. The fresh session is registered
    /// into `WorkRegistry` once its own `LoadWork`/`NewWork` subscriber (in
    /// `App::build`) resolves a real `work_id` — not here, since the id does
    /// not exist yet at this point for a `Load`/`New` action. A **second window
    /// on an already-open Work** is the one case where it does: see
    /// [`Self::attached_window_config`].
    ///
    /// Returns the freshly-built [`InitialWindowState`] alongside the
    /// `WindowConfig`: `main.rs`'s *initial* window uses it to seed the handful
    /// of remaining `app_state` registrations a few widgets still read Tier-2
    /// state through (`tags::tag_chip`, `view_models::overview`'s tree-
    /// expansion restore, the Settings ▸ Work panes — a known, flagged Phase-2
    /// gap; see the migration report) rather than a constructor-threaded
    /// handle. Every other caller (the Welcome/New-Work flows opening a
    /// genuinely new window) discards it: `app_state` cannot be re-registered
    /// after the builder runs, so only the first window's session can ever
    /// satisfy those few lookups.
    pub fn window_config(&self, action: PendingAction) -> (WindowConfig, InitialWindowState) {
        self.build_window(action, None)
    }

    /// Build a **second window onto a Work that is already open** in this
    /// process — Work ▸ New Window. `None` when `work_id` names no
    /// currently-open Work, which is the one thing the caller cannot rule out
    /// on its own (a `work_id` read from a window whose project was closed
    /// between the menu opening and the click).
    ///
    /// This is the create-**or-share** half of the registry's resolution
    /// mechanism finally being used: the live [`WorkSession`] is resolved
    /// through [`WorkRegistry::attach`], not minted, so both windows share one
    /// `AppIds`, one set of singles, one undo stack, one `OpenDocsStore` and one
    /// save/dirty state. Typing in either window is the same edit to the same
    /// document, and only the *last* window to close tears any of it down.
    ///
    /// Two things are decided here rather than in `App`, because both must be
    /// known before the window exists:
    ///
    ///   * the **ordinal** ([`WorkRegistry::reserve_window_ordinal`]) — the
    ///     window's fixed "which window on this Work am I" number, which its
    ///     title suffix and its persistence id are both derived from;
    ///   * the **string id** ([`attached_window_id_for`]) — its identity in
    ///     bastyde's window map and the key its geometry is saved under.
    ///
    /// The refcount `attach` bumps is released symmetrically by
    /// `WorkRegistry::remove_window`, driven by this window's own `on_removed`
    /// hook — the same path every other window's release already takes.
    pub fn attached_window_config(
        &self,
        work_id: u64,
        path: &str,
    ) -> Option<(WindowConfig, InitialWindowState)> {
        let session = self.registry.attach(work_id)?;
        let ordinal = self.registry.reserve_window_ordinal(work_id);
        Some(self.build_window(
            PendingAction::AttachExisting {
                work_id,
                path: path.to_string(),
                ordinal,
            },
            Some(session),
        ))
    }

    /// The shared body of [`Self::window_config`] and
    /// [`Self::attached_window_config`]. `attached` is the already-open Work's
    /// live session (Work ▸ New Window) or `None` to mint a fresh one.
    fn build_window(
        &self,
        action: PendingAction,
        attached: Option<WorkSession>,
    ) -> (WindowConfig, InitialWindowState) {
        let app_ctx_root = self.app_ctx.clone();
        // Per WINDOW: the menu bar binds these signals before `EditorsViewModel`
        // exists; `App::build` attaches the editors. Never shared — a shared
        // instance made the last-built window win the Format dock's target.
        let format = FormatViewModel::detached();
        // A second window on an already-open Work is numbered — and so
        // *identified* — the moment it is built; every other window is the
        // first on its Work until its own Load/New says otherwise.
        let ordinal = match &action {
            PendingAction::AttachExisting { ordinal, .. } => *ordinal,
            _ => 1,
        };
        // Fixed for this window's whole life — see [`crate::app::WindowRole`].
        let role = crate::app::WindowRole::from_action(&action);
        let id = attached_window_id_for(action.target_path(), ordinal);
        // Tier 2 (the Work's own state) is either shared with the sibling
        // window that already shows this Work, or minted fresh for the Work
        // this window is about to load/create. Tier 3 — the outline and its
        // `DockingModel` — is always this window's own, even when the Work is
        // shared: two windows on one project each arrange their own desk.
        let (session, ids, outline) = match attached {
            Some(session) => {
                let ids = session.ids.clone();
                let outline = OutlineViewModel::new_default(app_ctx_root.clone(), ids.clone());
                (session, ids, outline)
            }
            None => {
                let ids = AppIds::new();
                let outline = OutlineViewModel::new_default(app_ctx_root.clone(), ids.clone());
                let session = WorkSession::new(
                    app_ctx_root.clone(),
                    ids.clone(),
                    self.spellcheck.clone(),
                    outline.docking(),
                    self.backup_settings.clone(),
                    self.workspace_layout_service.clone(),
                    self.tree_expansion_service.clone(),
                );
                (session, ids, outline)
            }
        };
        let export = ExportViewModel::new(app_ctx_root.clone(), ids.clone());
        // Tier 3, like `export` and `outline` above: bound to this window's own
        // `ids`, so the wizard opened here imports into the project *this*
        // window shows. Built out here rather than inside the modal because it
        // is also what the analysis's long-operation events are routed to (see
        // `app::wiring::long_ops`), and the modal comes and goes.
        let import_document =
            crate::view_models::ImportDocumentViewModel::new(app_ctx_root.clone(), ids.clone());
        let registry = self.registry.clone();
        let quit = self.quit.clone();
        // Let this Work's on-close backup hand control back to the quit sequencer
        // when it lands (see `BackupSchedulerViewModel::do_close`'s
        // `PendingExit::Quit` arm).
        session.backup_scheduler.set_quit_sequencer(quit.clone());
        let single_work = session.single_work.clone();
        let single_work_info = session.single_work_info.clone();
        // Scope D — window titles. For a Load/New window this is `1` until
        // `App::build`'s own `LoadWork`/`NewWork` subscriber calls
        // `WorkRegistry::register_window` and writes back whatever ordinal it
        // was actually assigned (see `window_title_text`'s doc for why `1`
        // never shows a suffix, and `App::new`'s `window_ordinal` parameter for
        // the write-back). A Work ▸ New Window window already knows its number
        // — it was reserved before the window was built, precisely so its
        // string id could be derived from it — so it starts with the real value
        // and its title reads "(Window 2)" from the very first frame rather
        // than flickering through the un-suffixed form.
        let window_ordinal: Signal<usize> = Signal::new(ordinal);
        let title_text = window_title_text(&single_work, &window_ordinal);
        let autosave_menu = self.autosave_menu.clone();
        let spellcheck_menu = self.spellcheck_menu.clone();
        let comments_menu = self.comments_menu.clone();
        // Per WINDOW, not per process: unlike `spellcheck_menu` (a global
        // setting, correctly shared), this tracks which surface *this* window
        // has focused. A process-wide one would let a second project window
        // grey out this window's Format menu.
        let scene_focused = Signal::new(false);
        // The Document menu's per-item rows (Rename / Duplicate / Indent / Outdent /
        // Move to Trash) grey out with nothing selected. Per WINDOW for the same reason
        // `scene_focused` is: it mirrors *this* window's own binder selection, and a
        // process-wide one would let a second project window enable this one's rows.
        // Kept concrete rather than mapped off the selection set — `MenuEntry::enabled`
        // wants a real `Signal<bool>`; `App::build` keeps it in step.
        let binder_has_selection = Signal::new(false);
        // Pre-allocated so the Document menu's "Insert template" submenu stays addressable
        // after the model is built — its contents are DATA and have to be refilled as the
        // catalogue changes. Per WINDOW, like everything else here: two windows each own a
        // menu model, and one window's id must not address the other's submenu.
        let templates_submenu_id = bastyde::core::menu_item_id::MenuItemId::next();
        // Increment 4 (the Go menu). Per WINDOW, same rationale as `scene_focused`
        // just above: it mirrors *this* window's own focused item, so a second
        // simultaneously-open project window's Go menu never reflects the wrong
        // window's answer. See `GoAvailability`'s module doc.
        let go = crate::view_models::GoAvailability::new();
        // The "jump to any item" popup's state. Per WINDOW, same rationale as
        // `go` just above and for a sharper reason: it owns its own binder tree
        // model and selection, so a process-wide instance would let one window's
        // popup drive another window's editor. See `GoToViewModel`'s module doc.
        let go_to = crate::view_models::GoToViewModel::new(app_ctx_root.clone(), ids.clone());
        // Increment 1 of distraction-free (plain fullscreen). Per WINDOW,
        // same rationale as `scene_focused` just above: this remembers
        // *this* window's own pre-fullscreen placement, so a second
        // simultaneously-open project window's F11 never restores (or
        // clobbers) the wrong window's memory. See `FullscreenViewModel`'s
        // module doc.
        let fullscreen = crate::view_models::FullscreenViewModel::new();
        // Increment 2 of distraction-free (chrome collapse). Per WINDOW, same
        // rationale as `fullscreen` just above — see `FocusViewModel`'s
        // module doc for why it keeps its own independent placement memory
        // rather than sharing `fullscreen`'s.
        let focus = crate::view_models::FocusViewModel::new();
        // The surface the mode actually shows. Per window for the same reason
        // `focus` is; minted empty because everything it needs beyond `focus`,
        // `go_to` and `ids` is built inside `App::build`, which hands it over
        // there (see `DistractionFreeSurfaceViewModel`'s module doc).
        let df_surface = crate::view_models::DistractionFreeSurfaceViewModel::new(
            app_ctx_root.clone(),
            focus.clone(),
            go_to.clone(),
            ids.clone(),
        );
        // Sourced from `session`, never a `self` field (Phase 3): a second
        // simultaneously-open Work must never share this Work's backup-mode
        // flag/details — see `WorkSession`'s module doc.
        let backup_mode = session.backup_mode.clone();
        let backup_context = session.backup_context.clone();
        // Fresh per window (Phase 2), like `session`/`outline`/`export` above:
        // bound to *this* window's own `ids`/`single_work`, so a Save-As or
        // backup-restore from this window always targets this window's own
        // Work, never whichever window happened to build most recently.
        let save_as_vm = SaveAsViewModel::new(
            app_ctx_root.clone(),
            ids.clone(),
            single_work.clone(),
            backup_mode.clone(),
            backup_context.clone(),
        );
        let restore_vm = crate::view_models::BackupRestoreViewModel::new(
            app_ctx_root.clone(),
            ids.clone(),
            single_work.clone(),
            backup_mode.clone(),
            backup_context.clone(),
        );
        // Sourced from `session`, never a `self` field (Scope E, same fix as
        // `backup_mode`/`backup_context` above): a second, simultaneously-open
        // Work must never share this Work's dirty-state flag — see
        // `WorkSession::unsaved`'s doc.
        let unsaved = session.unsaved.clone();
        // Per WINDOW, bound to *this* Work's unsaved/backup signals — never a
        // process-wide switch guard whose hooks the last owner overwrote.
        let project_switch = crate::view_models::ProjectSwitchViewModel::new(
            app_ctx_root.clone(),
            unsaved.clone(),
            backup_mode.clone(),
            self.autosave_menu.clone(),
        );
        // Fresh per WINDOW, never a `self`/`session` field: this names a
        // *window's* own deferred close/quit, not the Work's data — even two
        // windows on the SAME Work must each resolve their own close
        // independently. See `ProjectWindowFactory`'s field doc.
        let pending_exit: Signal<PendingExit> = Signal::new(PendingExit::None);
        let backup_scheduler = session.backup_scheduler.clone();
        // Clones for the caller (see this method's doc) — the originals are
        // moved into the `.root(...)` closure below.
        let initial_state = InitialWindowState {
            session: session.clone(),
            outline: outline.clone(),
        };

        let config = WindowConfig::new()
            .id(id)
            // A snapshot baseline only: at this instant the Work hasn't
            // finished loading/creating yet, so `title_text` (its live value)
            // is still the generic fallback anyway. `App::build`'s own effect
            // (see its `window_ordinal`/`title_text` fields) pushes every
            // later change to `state.title()` reactively — see
            // `window_title_text`'s doc.
            .title(title_text.get())
            .size(1200, 800)
            .min_size(800, 600)
            .decorations(DecorationsMode::CustomChrome)
            // Consume an xdg-activation startup token (set by the desktop, or
            // by another instance's "open in new window") so this window comes
            // up focused on Wayland.
            .activate_from_env(true)
            // Unsaved-changes guard for every interactive close that still
            // routes through `close_window()` — the title-bar X and Alt+F4
            // (Ctrl+Q no longer does: `app.quit`'s action calls
            // `guard_unsaved_exit` directly and ends in a real process exit —
            // see `app::PendingExit::Quit` — instead of `close_window()`,
            // which would only ever land back here and return to the
            // Launcher). In the launcher-window model, closing a project
            // window *this* way never quits the process by itself: it always
            // opens a fresh Launcher window first, then force-closes this one
            // (`close_work_and_return_to_launcher`) — the process only
            // actually exits via `app.quit`, or by closing the Launcher
            // itself.
            //
            // Delegates entirely to `guard_unsaved_exit` (shared with
            // `work.close`'s action and `app.quit`'s action, both in
            // `app.rs`) instead of re-deriving the branch order here — this
            // is the ONE hand-written copy of it left in the crate. See that
            // function's docs for what each `UnsavedDecision` arm does; this
            // closure only supplies the outcome (`ReturnToLauncher`) and
            // always vetoes the framework's own close, regardless of branch —
            // the guard performs the actual transition itself, either at once
            // or once a deferred save lands.
            //
            // **Except when a sibling window still shows this Work** (Work ▸ New
            // Window). Then closing this window is closing a *view*, not the
            // project: the Work stays open, its edits stay exactly where they
            // are, and the sibling goes on showing them. So there is nothing to
            // guard — prompting "you have unsaved changes" for a project that is
            // not going anywhere would be a lie, an on-close backup would fire
            // for a project still being edited, and returning to the Launcher
            // would be answering a question nobody asked. The framework's own
            // close is allowed through instead, and `on_removed` below does the
            // rest: this window's `OpenDoc` refs and flush hook released, the
            // Work's refcount dropped by one, its undo stack left alone because
            // `WorkRegistry` reports this was not the last window on it.
            .on_close_requested({
                let unsaved = unsaved.clone();
                let autosave = autosave_menu.clone();
                let pending = pending_exit.clone();
                let scheduler = backup_scheduler.clone();
                let backup_mode = backup_mode.clone();
                let app_ctx_guard = app_ctx_root.clone();
                let ids = session.ids.clone();
                let registry_guard = registry.clone();
                let layout_guard = session.workspace_layout.clone();
                move |ctx| {
                    let shared_with_a_sibling = ids
                        .work_id
                        .get()
                        .is_some_and(|work_id| registry_guard.window_count_for(work_id) > 1);
                    if shared_with_a_sibling {
                        // The desk belongs to the window that loaded the project
                        // (see `WindowRole::owns_desk`), and this is its last
                        // chance to write it: the Work lives on in the sibling,
                        // so no `CloseWork` — and none of the capture that hangs
                        // off it — will ever run for this window.
                        if role.owns_desk() {
                            crate::app::capture_workspace_layout(&layout_guard);
                        }
                        return CloseResponse::Close;
                    }
                    guard_unsaved_exit(
                        ctx,
                        &app_ctx_guard,
                        &ids,
                        unsaved.get(),
                        backup_mode.get(),
                        autosave.get(),
                        &pending,
                        &scheduler,
                        PendingExit::ReturnToLauncher,
                    );
                    CloseResponse::Veto
                }
            })
            // The window-teardown hook (see `bastyde::core::window::WindowConfig::
            // on_removed`'s doc for exactly when this fires and what it may/may not
            // touch — no `EventContext`, no live tree). Built fresh here, inside
            // this per-call closure, never hoisted onto a `ProjectWindowFactory`
            // field: `on_removed` may be attached to several windows (the doc's own
            // "Rc-cloned config" case), and a closure built once in `new` would
            // fire against every window with the SAME captured window/session
            // state — exactly the mistake `on_close_requested` above already
            // avoids by being rebuilt per call.
            //
            // `registry.remove_window` is the one thing this needs: it looks up
            // (by `event.id`) the two teardowns `App::build`'s `LoadWork`/
            // `NewWork` subscribers registered for THIS window (see
            // `build_stack_teardown`/`build_window_teardown` in `app.rs`), runs
            // the window-scoped one unconditionally (its `OpenDoc` refs/backup
            // flush hook — the window really is gone now), then decides — from
            // Skribisto's own window→Work bookkeeping, never
            // `event.remaining_windows` (which counts every window in the
            // process, Launcher included) — whether this was the last window on
            // its Work, and runs the stack-scoped one with that answer. A safe
            // no-op for the rare window that force-closed before its own
            // Load/New ever registered one.
            .on_removed({
                let registry = registry.clone();
                move |event| registry.remove_window(event.id)
            })
            .root(move |tree, state| {
                let theme = tree.theme().clone();

                // Custom Bastyde title bar with a model-driven hamburger menu
                // in the leading slot (falls back to a plain label on any
                // platform whose host is unavailable).
                // A clone of the menu model, kept alive past the match so `App::build` can
                // refill the template submenu as the catalogue changes. `None` on a
                // platform with no title-bar host, where there is no model to refill.
                let mut templates_menu: Option<(MenuModel, MenuItemId)> = None;
                let title_bar = match tree.title_bar_host() {
                    Some(host) => {
                        // Model-style menu, collapsed to a hamburger (☰).
                        let menu = super::project_menus::build_project_menu(
                            super::project_menus::ProjectMenuParts {
                                app_ctx: app_ctx_root.clone(),
                                export: export.clone(),
                                single_work: single_work.clone(),
                                single_work_info: single_work_info.clone(),
                                ids: session.ids.clone(),
                                autosave_menu: autosave_menu.clone(),
                                spellcheck_menu: spellcheck_menu.clone(),
                                comments_menu: comments_menu.clone(),
                                scene_focused: scene_focused.clone(),
                                binder_has_selection: binder_has_selection.clone(),
                                templates_submenu_id,
                                go: go.clone(),
                                format: format.clone(),
                                save_as: save_as_vm.clone(),
                                backup_mode: backup_mode.clone(),
                                unsaved: unsaved.clone(),
                                outline: outline.clone(),
                                focus: focus.clone(),
                                placement: state.placement().clone(),
                            },
                        );
                        // Fill the template submenu once now, so a window opened on a
                        // project that already has templates shows them before anything
                        // changes. `App::build` keeps it in step from there.
                        super::project_menus::sync_insert_template_submenu(
                            &menu,
                            templates_submenu_id,
                            &session.note_templates,
                            &format.has_target(),
                        );

                        templates_menu = Some((menu.clone(), templates_submenu_id));

                        // `Toolbar` (30 dp), not `Large` (40): the bar is
                        // `TITLE_BAR_HEIGHT` tall and does not grow for an oversized
                        // child — a `Large` hamburger simply overflows the strip.
                        let menubar = MenuBar::from_model(menu)
                            .collapse_policy(CollapsePolicy::Always)
                            .hamburger_size(IconButtonSize::Toolbar);

                        // Distraction-free gates nothing in here any more: the
                        // whole title bar is parked dormant along with the rest of
                        // the shell, by the one `VisibleWhen` at the window root.
                        // What is left below is about *fullscreen*, which is a
                        // different thing and still needs its own answer.
                        //
                        // The exit guarantee the collapsed bar used to owe:
                        //   * distraction-free → the strip's Exit button (always
                        //     present), Shift+F11, and Escape;
                        //   * plain fullscreen → the menu bar is still on screen, so
                        //     View ▸ Fullscreen and F11;
                        //   * either → Ctrl+W, Ctrl+Q, and the compositor's own
                        //     unfullscreen/close.
                        //
                        // Read from the WINDOW's own placement, not from
                        // `FullscreenViewModel`/`FocusViewModel`: those are two
                        // independent memories (F11 and Shift+F11 each keep their own),
                        // and an OS-initiated fullscreen goes through neither. The
                        // placement is the single fact all three agree on.
                        let controls_visible = state.placement().map(|p| !p.is_fullscreen());
                        // Brand mark first (simple icon, same treatment as the
                        // Launcher title bar), then the hamburger. Leading inset
                        // on the icon: it sits at the window's left edge, so
                        // without padding it would flush against the frame —
                        // matching `launcher_window`'s brand icon.
                        let brand_icon = Padding::new(0.0, 0.0, 0.0, 8.0).child(
                            IconWidget::from_raster(
                                res!("../../resources/icons/skribisto.png"),
                                25.0,
                            )
                            .mode(IconMode::FullColor),
                        );
                        let leading = bati!(
                            HStack {
                                spacing: 5.0
                                alignment: bastyde::tokens::VAlignment::Center
                                child: brand_icon
                                child: menubar
                            }
                        );
                        let trailing_controls = bati!(
                            HStack {
                                spacing: 5.0
                                alignment: bastyde::tokens::VAlignment::Center
                                SpellcheckToggleButton::new(spellcheck_menu.clone())
                                ExportSplitButton::new(export.clone())
                            }
                        );
                        let center_content = bati!(
                            HStack {
                                    spacing: 5.0
                                    alignment: bastyde::tokens::VAlignment::Center
                                    // The `center` slot lives inside the TitleBar's
                                    // DragRegion, which is published to the OS as the
                                    // window caption (on Windows: WM_NCHITTEST ->
                                    // HTCAPTION). The OS owns caption pixels outright,
                                    // so a bare button here would never see a click —
                                    // it would only drag the window. `DeadZone` carves
                                    // the project switcher back out of the caption, and
                                    // (on every platform) stops a few px of pointer
                                    // jitter during a click from arming the window drag.
                                    DeadZone {
                                        ProjectSwitcherButton::new(app_ctx_root.clone(), single_work.clone(), single_work_info.clone())
                                    }
                                    Expand::horizontal {
                                        Center {
                                            TextWidget::new(lit!("Skribisto")) {
                                                // Scope D — live, per-Work, sibling-disambiguating
                                                // title (see `window_title_text`'s doc).
                                                text: title_text.clone()
                                                style: theme.typography.body_bold.clone()
                                                color: TextRole::Primary
                                            }
                                        }
                                    }
                                }
                        );

                        tree.add_boxed(Box::new(bati!(
                            TitleBar::new(host) {
                                height: super::TITLE_BAR_HEIGHT
                                background: SurfaceRole::Main
                                leading: leading
                                // Left of the window buttons: the master spell-check switch,
                                // then the focus-adaptive Export control. The `trailing` slot
                                // takes one widget, so they share an HStack.
                                trailing: trailing_controls
                                center: Expand::horizontal {
                                    child: center_content
                                }
                                // Hidden whenever this window is fullscreen — plain F11
                                // as well as distraction-free. See the exit guarantee
                                // noted above.
                                controls_visible: controls_visible
                                close_action: |ctx| ctx.close_window()
                            }
                        )))
                    }
                    None => tree.add(TextWidget::new(lit!("Skribisto")).text(title_text.clone())),
                };

                let body = tree.add(Expand::new().child(App::new(
                    app_ctx_root.clone(),
                    session.clone(),
                    outline.clone(),
                    fullscreen.clone(),
                    focus.clone(),
                    export.clone(),
                    import_document.clone(),
                    autosave_menu.clone(),
                    spellcheck_menu.clone(),
                    comments_menu.clone(),
                    scene_focused.clone(),
                    binder_has_selection.clone(),
                    templates_menu,
                    go.clone(),
                    go_to.clone(),
                    unsaved.clone(),
                    pending_exit.clone(),
                    backup_mode.clone(),
                    backup_context.clone(),
                    action,
                    registry.clone(),
                    quit.clone(),
                    save_as_vm.clone(),
                    restore_vm.clone(),
                    format.clone(),
                    project_switch.clone(),
                    title_text.clone(),
                    window_ordinal.clone(),
                    df_surface.clone(),
                )));
                // The project shell, and the distraction-free surface that
                // replaces it, as siblings in a `ZStack` — the surface is added
                // second so it paints on top, and it sits *outside* the stack's
                // first child so it covers the title bar too.
                //
                // The main tree's gate is `VisibleWhen`, not merely `Slide`:
                // `Slide` translates its child but leaves it painted, in the
                // accessibility tree and in the Tab order, whereas
                // `ctx.visible_when` parks a subtree dormant — no layout, no
                // paint, no AT node, no tab order — without destroying it, so
                // the writer's tabs, carets and scroll positions all survive
                // untouched behind the surface. Being able to Tab into an
                // invisible menu bar is exactly what this app's accessibility
                // posture rules out.
                //
                // The surface itself is wrapped in *both*: `VisibleWhen` for
                // presence and dormancy, `Slide` for the motion. `Slide`'s own
                // progress animation keeps observing the same signal while its
                // parent gate is parked, so it is already settled the moment the
                // gate lets it through.
                let chrome_visible = focus.active_signal().map(|active| !*active);
                let shell = tree.add(VisibleWhen::new(
                    chrome_visible,
                    VStack::new()
                        .spacing(0.0)
                        .add_child(title_bar)
                        .add_child(body),
                ));
                // `VisibleWhen` **inside** `Slide`, not around it. Both read the
                // same signal, so they turn on and off together — but the nesting
                // decides which one can animate. `Slide` drives its progress with
                // an animation, and an animation whose owner is parked dormant is
                // paused: with the gate on the outside the surface stayed at
                // progress 0, i.e. fully translated off the trailing edge —
                // present in the tree (its Exit button was reachable) and
                // invisible on screen. Slide stays live; the gate parks only the
                // content, which is all that was ever wanted from it (no layout,
                // no paint, no accessibility node, no tab stop while the mode is
                // off).
                // `Slide` translates its child by the child's own **natural
                // size**, so the surface reports the full window while the mode
                // is on (a window's width to travel) and nothing at all while it
                // is off — see `DistractionFreeSurface::build`, which is also
                // where the "nothing at all" keeps this sibling from covering the
                // shell with an invisible click target.
                //
                // No `VisibleWhen` around the `Slide`: `Slide` drives its
                // progress with an animation, and an animation whose owner is
                // parked dormant is paused — gated from outside, the surface
                // stayed at progress 0, translated fully off the trailing edge,
                // present in the tree and invisible on screen.
                let surface = tree.add(
                    Slide::new(focus.active_signal())
                        .from(SlideEdge::Trailing)
                        .child(crate::distraction_free::DistractionFreeSurface::new(
                            df_surface.clone(),
                        )),
                );
                // The distraction-free theme, applied as a **token override on
                // this subtree** rather than as colours on any widget.
                //
                // That is what keeps a paper-and-ink theme inside the house rule:
                // every widget under here still asks only for
                // `SurfaceRole::Content` / `TextRole::Primary`, and this decides
                // what those *mean* on the surface — light and dark keep working,
                // and no raw hex ever reaches a call site. The four axes land on
                // four roles the widgets already tell apart:
                //
                //   surface_content → the page (and the strip, which is themed as
                //                     the page's own footer)
                //   text_primary    → the prose
                //   text_secondary  → the strip's readouts and the item name
                //   surface_main    → whatever is not the page: a container tab's
                //                     segmented bar and the gaps around it
                //
                // The closure is consulted on every resolve, so it reads the
                // current theme live; the surface marks its own subtree dirty
                // when the choice or the library changes (see its `build`).
                {
                    let vm = df_surface.clone();
                    tree.set_theme_override(surface, move |theme| {
                        if let Some(t) = vm.theme() {
                            crate::distraction_free::theme::apply_to(theme, &t);
                        }
                    });
                }
                // `Expand` around the stack, not inside it: `ZStack` sizes itself
                // to the *intrinsic* max of its children and explicitly claims no
                // growth slack, so left bare at the window root it would report a
                // content-sized box instead of filling the window. Its
                // `place_children` does hand each child the full bounds, so one
                // `Expand` on the outside is all it takes.
                let stack = tree.add(ZStack::new().add_child(shell).add_child(surface));
                let inner = tree.add(Expand::new().child_id(stack));

                // Add edge resize handles only where the host needs the app
                // to drive them (skipped on macOS — NSWindow handles edges).
                // App-global commands reach the menu/shortcut via
                // `register_action_global` (no root wrapper needed).
                match tree.title_bar_host() {
                    Some(host) if host.needs_custom_resize_handles() => {
                        tree.add(WindowFrame::new(host).thickness(6.0).content_id(inner))
                    }
                    _ => inner,
                }
            });
        (config, initial_state)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn window_id_for_a_missing_path_is_stable_and_not_shared() {
        // A New Work target that hasn't been written yet falls back to
        // hashing the raw string — still deterministic, still distinct from
        // the launcher's fixed id.
        let a = window_id_for("/tmp/skribisto-window-id-test-does-not-exist-a.skrib");
        let b = window_id_for("/tmp/skribisto-window-id-test-does-not-exist-a.skrib");
        let c = window_id_for("/tmp/skribisto-window-id-test-does-not-exist-b.skrib");
        assert_eq!(a, b, "the same path always yields the same id");
        assert_ne!(a, c, "different paths yield different ids");
        assert_ne!(a, LAUNCHER_WINDOW_ID);
        assert!(a.starts_with("work-"));
    }

    #[test]
    fn window_id_for_collapses_different_spellings_of_one_path() {
        let dir = std::env::temp_dir().join(format!("sk-window-id-test-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let file = dir.join("project.skrib");
        std::fs::write(&file, b"x").unwrap();

        let direct = window_id_for(file.to_str().unwrap());
        let via_dotdot = window_id_for(&format!(
            "{}/../{}/project.skrib",
            dir.to_str().unwrap(),
            dir.file_name().unwrap().to_str().unwrap()
        ));
        assert_eq!(
            direct, via_dotdot,
            "canonicalization must collapse a `..`-spelled path onto the same id"
        );

        let _ = std::fs::remove_dir_all(&dir);
    }

    /// F4(a): the persisted id must not depend on `std::hash::DefaultHasher`
    /// (whose algorithm std explicitly does not guarantee stable across Rust
    /// releases) — swapped for blake3. This can't directly prove
    /// cross-release stability (that would require pinning a specific
    /// toolchain), but it does pin the two properties a caller actually
    /// relies on: the id is deterministic for a given input in this process,
    /// and its shape is the fixed `work-{16 hex chars}` this module's other
    /// callers (and `window_state.toml`'s existing rows) expect.
    #[test]
    fn window_id_for_is_deterministic_and_16_hex_chars() {
        let path = "/tmp/skribisto-window-id-format-test-does-not-exist.skrib";
        let a = window_id_for(path);
        let b = window_id_for(path);
        assert_eq!(a, b, "hashing the same path twice must agree");

        let hex = a.strip_prefix("work-").expect("id must start with work-");
        assert_eq!(hex.len(), 16, "id must be work- + exactly 16 hex chars");
        assert!(
            hex.chars().all(|c| c.is_ascii_hexdigit()),
            "id suffix must be lowercase/uppercase hex, got {hex:?}"
        );
    }

    // ── Second windows on one project (Work ▸ New Window) ────────────────

    /// The first window on a project must keep the id its geometry has always
    /// been saved under — anything else silently orphans every existing
    /// `window_state.toml` row the day this ships.
    #[test]
    fn the_first_window_on_a_project_keeps_the_plain_id() {
        let path = "/tmp/skribisto-attached-id-test-does-not-exist.skrib";
        assert_eq!(attached_window_id_for(path, 1), window_id_for(path));
        // Defensive: an ordinal of 0 is not reachable (they start at 1), but it
        // must degrade to the base id rather than producing `-w0`.
        assert_eq!(attached_window_id_for(path, 0), window_id_for(path));
    }

    /// Every further window is a distinct identity — distinct from the first
    /// and from each other. bastyde's `WindowManager` overwrites its
    /// `string_to_id` entry rather than rejecting a duplicate, so a collision
    /// here would not fail loudly: the newer window would silently steal the
    /// older one's identity, and `find_window` (hence `open_or_focus_project`,
    /// the IPC raise path and the switcher) would resolve to the wrong one.
    #[test]
    fn every_further_window_on_one_project_gets_its_own_id() {
        let path = "/tmp/skribisto-attached-id-test-does-not-exist.skrib";
        let base = window_id_for(path);
        let second = attached_window_id_for(path, 2);
        let third = attached_window_id_for(path, 3);

        assert_eq!(second, format!("{base}-w2"));
        assert_ne!(
            second, base,
            "the second window must not claim the first's id"
        );
        assert_ne!(third, second, "two further windows must not share one id");
        assert!(
            second.starts_with(&base),
            "a second window's id must stay recognisably derived from its project's"
        );
    }

    /// Two different projects' second windows must not collide either — the
    /// suffix disambiguates *within* a project, never across them.
    #[test]
    fn second_windows_of_different_projects_do_not_collide() {
        let a = attached_window_id_for("/tmp/skribisto-attached-a-does-not-exist.skrib", 2);
        let b = attached_window_id_for("/tmp/skribisto-attached-b-does-not-exist.skrib", 2);
        assert_ne!(a, b);
    }

    /// The id is a pure function of (project, ordinal): reopening a second
    /// window on the same project at the same ordinal must land on the same
    /// remembered geometry.
    #[test]
    fn an_attached_window_id_is_deterministic() {
        let path = "/tmp/skribisto-attached-id-test-does-not-exist.skrib";
        assert_eq!(
            attached_window_id_for(path, 2),
            attached_window_id_for(path, 2)
        );
    }

    // ── `attached_window_config` (the Work ▸ New Window factory path) ──────

    /// A factory over a caller-supplied registry, so a test can register a Work
    /// and then ask for a second window on it. (`view_models::welcome`'s own
    /// helper builds its registry internally, which is fine there and useless
    /// here.)
    fn test_factory(app_ctx: Rc<AppContext>, registry: WorkRegistry) -> ProjectWindowFactory {
        use crate::models::{BackupSettingsService, TreeExpansionService, WorkspaceLayoutService};
        use crate::spellcheck::SpellcheckService;
        ProjectWindowFactory::new(
            app_ctx,
            registry,
            SpellcheckService::new(),
            BackupSettingsViewModel::new(BackupSettingsService::in_memory_default()),
            WorkspaceLayoutService::in_memory_default(),
            TreeExpansionService::in_memory_default(),
            Signal::new(false),
            Signal::new(true),
            Signal::new(true),
        )
    }

    /// The race the menu item cannot rule out on its own: the Work was closed
    /// between the menu opening and the click. No window, rather than a window
    /// onto nothing — and, critically, rather than falling back to *loading the
    /// file again*, which would give two independent `Work`s for one project.
    #[test]
    fn attaching_to_a_work_that_is_not_open_yields_no_window() {
        let app_ctx = Rc::new(AppContext::new());
        let registry = WorkRegistry::new();
        let factory = test_factory(app_ctx, registry);

        assert!(
            factory
                .attached_window_config(404, "/tmp/skribisto-attach-test.skrib")
                .is_none()
        );
    }

    /// The heart of the feature: a second window on an open Work shares that
    /// Work's live session — one `AppIds`, one set of singles, one undo stack —
    /// rather than minting a second one, and takes its own identity (ordinal,
    /// string id) so the two windows never collide in bastyde's window map.
    #[test]
    fn a_second_window_shares_the_works_session_and_takes_its_own_identity() {
        let app_ctx = Rc::new(AppContext::new());
        let registry = WorkRegistry::new();
        let session = crate::sessions::WorkSession::for_test();
        session.ids.work_id.set(Some(1));
        registry.register(1, session.clone());
        // The Work's first window, exactly as its own `LoadWork` subscriber
        // binds it — the state Work ▸ New Window is always invoked from.
        registry.register_window(
            bastyde::prelude::BastydeWindowId::new(1),
            1,
            None,
            Rc::new(|_| {}),
            Rc::new(|| {}),
        );
        let factory = test_factory(app_ctx, registry.clone());

        let path = "/tmp/skribisto-attach-test.skrib";
        let (config, state) = factory
            .attached_window_config(1, path)
            .expect("the Work is open, so a second window on it must be buildable");

        assert_eq!(
            config.string_id.as_deref(),
            Some(attached_window_id_for(path, 2).as_str()),
            "the second window must carry its own persistence id, never the first's"
        );
        // Shared, not copied: a write through the registered session is visible
        // through the new window's — they are one object.
        session.ids.work_info_id.set(Some(99));
        assert_eq!(
            state.session.ids.work_info_id.get(),
            Some(99),
            "the attached window must share the Work's live session, not a fresh one"
        );
        assert_eq!(
            registry.window_count_for(1),
            2,
            "attaching must take a reference the new window's close will drop"
        );
    }

    /// Each further window gets its own ordinal and its own id — the mechanism
    /// is not limited to a second window, and none of them may collide.
    #[test]
    fn a_third_window_does_not_reuse_the_seconds_identity() {
        let app_ctx = Rc::new(AppContext::new());
        let registry = WorkRegistry::new();
        registry.register(1, crate::sessions::WorkSession::for_test());
        registry.register_window(
            bastyde::prelude::BastydeWindowId::new(1),
            1,
            None,
            Rc::new(|_| {}),
            Rc::new(|| {}),
        );
        let factory = test_factory(app_ctx, registry.clone());

        let path = "/tmp/skribisto-attach-test.skrib";
        let (second, _) = factory
            .attached_window_config(1, path)
            .expect("second window");
        let (third, _) = factory
            .attached_window_config(1, path)
            .expect("third window");

        assert_eq!(
            second.string_id.as_deref(),
            Some(attached_window_id_for(path, 2).as_str())
        );
        assert_eq!(
            third.string_id.as_deref(),
            Some(attached_window_id_for(path, 3).as_str())
        );
        assert_ne!(second.string_id, third.string_id);
        assert_eq!(registry.window_count_for(1), 3);
    }

    /// A window that *loads* a project keeps the plain id its geometry has
    /// always been saved under — the suffix is only ever an addition.
    #[test]
    fn a_loading_window_keeps_the_plain_project_id() {
        let app_ctx = Rc::new(AppContext::new());
        let factory = test_factory(app_ctx, WorkRegistry::new());

        let path = "/tmp/skribisto-attach-test.skrib";
        let (config, _) = factory.window_config(PendingAction::Load(path.to_string()));

        assert_eq!(
            config.string_id.as_deref(),
            Some(window_id_for(path).as_str())
        );
    }

    // ── Menu mnemonics ────────────────────────────────────────────────────
    //
    // A mnemonic must be unique *within* one keyboard namespace, and each open
    // menu is its own namespace — `Alt+F` opens File, and `F` may then address
    // an item inside it without ambiguity. So the check is per-scope, not
    // global, and the same letter may be reused freely across scopes.
    //
    // The menu bar itself is the scope that bites hardest: `MenuBar::build`
    // fills a `HashMap<char, usize>` behind a `debug_assert!`, so a duplicate
    // there is a debug-build panic, and in release the later entry silently
    // wins and the earlier menu becomes unreachable by keyboard. That is not
    // hypothetical — fr-FR shipped `F&ormat` against `&Outils` (both `O`)
    // until this test was written.
    //
    // Translators pick mnemonics per language, so a locale that reads clean in
    // English can collide in French. Every supported locale is checked.

    /// Menu scopes, mirroring the `MenuModel` built in
    /// [`ProjectWindowFactory::window_config`]. Add an entry to that menu, add its
    /// key here — an unlisted key is simply unchecked, which is the one
    /// failure mode this table has.
    const MENU_MNEMONIC_SCOPES: &[(&str, &[&str])] = &[
        (
            "menu bar",
            &[
                "menu-work",
                "menu-view",
                "menu-format",
                "menu-go",
                "menu-tools",
                "menu-help",
            ],
        ),
        (
            "Work",
            &[
                "menu-new-work",
                "menu-open-work",
                "menu-new-window",
                "menu-import-from",
                "menu-export",
                "menu-save",
                "menu-save-as-file",
                "menu-save-as-folder",
                "menu-backup",
                "menu-backups-list",
                "menu-close-work",
                "menu-welcome",
                "menu-settings",
                "menu-quit",
            ],
        ),
        ("Work > Import from", &["menu-import-plume"]),
        // The export scopes are labelled from `ExportScopeKind` at runtime and
        // deliberately carry no mnemonics; they are listed so the table stays a
        // complete picture of the menu, and the uniqueness check skips them.
        (
            "Work > Export",
            &[
                "menu-export-book",
                "menu-export-part",
                "menu-export-chapter",
                "menu-export-scene",
                "menu-export-note",
                "menu-export-folder",
                "menu-export-choose",
                "menu-export-none",
            ],
        ),
        (
            "View",
            &[
                "menu-outline",
                "menu-search",
                "menu-trash",
                "menu-search-preview",
                "menu-fullscreen",
                "menu-focus-mode",
            ],
        ),
        (
            "Format",
            &[
                "menu-format-marks-bold",
                "menu-format-marks-italic",
                "menu-format-marks-underline",
                "menu-format-marks-strike",
                "menu-format-marks-superscript",
                "menu-format-marks-subscript",
                "menu-format-marks-clear",
                "menu-format-heading",
                "menu-format-alignment",
                "menu-format-blockquote",
                "menu-format-lists",
                "menu-format-table",
                "menu-format-undo",
                "menu-format-redo",
                "menu-scene-break",
                "menu-major-scene-break",
            ],
        ),
        (
            "Format > Heading",
            &[
                "menu-format-heading-normal",
                "menu-format-heading-1",
                "menu-format-heading-2",
                "menu-format-heading-3",
                "menu-format-heading-4",
                "menu-format-heading-5",
                "menu-format-heading-6",
            ],
        ),
        (
            "Format > Alignment",
            &["menu-format-align-left", "menu-format-align-center"],
        ),
        (
            "Format > Lists",
            &[
                "menu-format-list-bullet",
                "menu-format-list-numbered",
                "menu-format-indent",
                "menu-format-outdent",
            ],
        ),
        (
            "Format > Table",
            &[
                "menu-format-table-insert",
                "menu-format-table-row-above",
                "menu-format-table-row-below",
                "menu-format-table-col-before",
                "menu-format-table-col-after",
                "menu-format-table-row-delete",
                "menu-format-table-col-delete",
                "menu-format-table-remove",
            ],
        ),
        (
            "Format > Table > Insert",
            &[
                "menu-format-table-2x2",
                "menu-format-table-3x3",
                "menu-format-table-4x4",
            ],
        ),
        (
            "Go",
            &[
                "menu-go-next-scene",
                "menu-go-prev-scene",
                "menu-go-next-chapter",
                "menu-go-prev-chapter",
                "menu-go-next-note",
                "menu-go-prev-note",
            ],
        ),
        ("Tools", &["menu-spellcheck"]),
        ("Help", &["menu-about"]),
    ];

    /// Every locale whose menu labels carry mnemonics, as the `.ftl` source.
    /// Mirrors the `compile_in` list in `main.rs`.
    const MENU_LOCALES: &[(&str, &str)] = &[
        ("en-US", include_str!("../../locales/en-US/main.ftl")),
        ("fr-FR", include_str!("../../locales/fr-FR/main.ftl")),
    ];

    /// The mnemonic a label declares, lower-cased to match `MenuBar`'s own
    /// `key_lower` table. `&&` is an escaped literal ampersand and is skipped,
    /// per the convention documented at the top of each `.ftl`.
    fn mnemonic_of(label: &str) -> Option<char> {
        let mut chars = label.chars().peekable();
        while let Some(c) = chars.next() {
            if c != '&' {
                continue;
            }
            match chars.peek() {
                Some('&') => {
                    chars.next();
                }
                Some(&marked) => return marked.to_lowercase().next().or(Some(marked)),
                None => return None,
            }
        }
        None
    }

    /// `key = value` pairs from one `.ftl`. Continuation lines are indented and
    /// comments start with `#`, so both are skipped; menu labels are always
    /// single-line, which is all this needs to see.
    fn ftl_labels(ftl: &str) -> std::collections::HashMap<&str, &str> {
        ftl.lines()
            .filter(|line| !line.starts_with('#') && !line.starts_with(char::is_whitespace))
            .filter_map(|line| line.split_once(" = "))
            .map(|(key, value)| (key.trim(), value.trim()))
            .collect()
    }

    #[test]
    fn menu_mnemonics_are_unique_within_every_scope_and_locale() {
        let mut collisions = Vec::new();

        for (locale, ftl) in MENU_LOCALES {
            let labels = ftl_labels(ftl);

            for (scope, keys) in MENU_MNEMONIC_SCOPES {
                let mut claimed: std::collections::HashMap<char, &str> =
                    std::collections::HashMap::new();

                for key in *keys {
                    let label = labels.get(key).unwrap_or_else(|| {
                        panic!(
                            "{locale}: `{key}` is listed in MENU_MNEMONIC_SCOPES \
                             but absent from main.ftl"
                        )
                    });
                    // No mnemonic is legitimate (see the export scopes above).
                    let Some(mnemonic) = mnemonic_of(label) else {
                        continue;
                    };
                    if let Some(previous) = claimed.insert(mnemonic, key) {
                        collisions.push(format!(
                            "{locale} / {scope}: '{mnemonic}' is claimed by both \
                             `{previous}` and `{key}`"
                        ));
                    }
                }
            }
        }

        assert!(
            collisions.is_empty(),
            "menu mnemonics must be unique within a scope:\n  {}",
            collisions.join("\n  ")
        );
    }

    #[test]
    fn every_menu_bar_entry_declares_a_mnemonic() {
        // A top-level menu with no mnemonic is unreachable by `Alt+letter`,
        // which for the Format menu is the whole keyboard path to the
        // formatting commands.
        let bar = MENU_MNEMONIC_SCOPES
            .iter()
            .find(|(scope, _)| *scope == "menu bar")
            .expect("the menu-bar scope is listed")
            .1;

        for (locale, ftl) in MENU_LOCALES {
            let labels = ftl_labels(ftl);
            for key in bar {
                let label = labels[key];
                assert!(
                    mnemonic_of(label).is_some(),
                    "{locale}: menu-bar entry `{key}` = {label:?} declares no mnemonic"
                );
            }
        }
    }

    #[test]
    fn mnemonic_of_reads_markers_and_skips_escaped_ampersands() {
        assert_eq!(mnemonic_of("&Fichier"), Some('f'));
        assert_eq!(mnemonic_of("Fo&rmat"), Some('r'));
        assert_eq!(mnemonic_of("E&xporter"), Some('x'));
        // An escaped ampersand is literal text, not a marker.
        assert_eq!(mnemonic_of("Search && Replace"), None);
        assert_eq!(mnemonic_of("Search && &Replace"), Some('r'));
        assert_eq!(mnemonic_of("no marker here"), None);
        // A trailing lone '&' marks nothing.
        assert_eq!(mnemonic_of("dangling &"), None);
    }
}
