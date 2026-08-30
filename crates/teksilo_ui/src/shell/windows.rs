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

use teksilo::core::menu_item_id::MenuItemId;
use teksilo::prelude::*;
use teksilo::widgets::MenuModel;
use teksilo::widgets::{
    Center, CollapsePolicy, DeadZone, Expand, HStack, IconButtonSize, MenuBar, NativeMenuMode,
    Padding, Slide, SlideEdge, TextWidget, TitleBar, VStack, WindowFrame, ZStack,
};

use frontend::AppContext;

use crate::app::{App, PendingAction, PendingExit, guard_unsaved_exit};
use crate::app_ids::AppIds;
use crate::backup::BackupSettingsViewModel;
use crate::binder::OutlineViewModel;
use crate::export::ExportViewModel;
use crate::export::split_button::ExportSplitButton;
use crate::format::FormatViewModel;
use crate::models::{TreeExpansionService, WorkspaceLayoutService};
use crate::save::SaveAsViewModel;
use crate::sessions::{WorkRegistry, WorkSession};
use crate::shell::project_switcher_button::ProjectSwitcherButton;
use crate::spellcheck::SpellcheckService;
use crate::spellcheck::toggle_button::SpellcheckToggleButton;
use crate::tabs::shared::editor::VisibleWhen;

// Re-export path→window identity so `shell::windows::*` stays the
// stable public surface for call sites.
pub use super::window_ids::{
    LAUNCHER_WINDOW_ID, attached_window_id_for, open_or_focus_project, resolve_project_window,
    window_id_for,
};

/// Scope D — window titles. A reactive `"{Work title} — {app name}"` (falling
/// back to the bare app name before a Work has finished loading/creating), with
/// a `" (Window N)"` suffix once `ordinal` says this window is not the sole one
/// showing its Work.
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
///
/// **The application name is [`crate::identity::display_name`], not a literal.**
/// An edition that registered its own identity must name *itself* in the title
/// bar, or every window claims to be the community build.
///
/// **The join is a Fluent key, not `format!`.** It used to be
/// `format!("{title} — Skribisto")` plus `format!("{base} (Window {ord})")`,
/// which left "(Window 2)" untranslated in every locale — a French window read
/// "(Window 2)". The product name inside it stays `lit!`-shaped data (a name is
/// not translated, the same rule that keeps entity titles out of Fluent); only
/// the framing around it is a message.
///
/// ⚠ Consequence for the KWin workflow described above: a rule written against
/// the French title will not match the English one. That is inherent in
/// translating the string at all, and worth knowing before writing a rule.
///
/// Resolved eagerly with `resolve_now`, so the title re-renders whenever the Work
/// title or the window ordinal changes. A language switch *mid-session* does not
/// by itself re-title already-open windows — a real if small gap, and the one
/// case where a stable KWin identity happens to be the friendlier behaviour.
///
/// Takes the Work's title **signal** rather than the `SingleWork` it came from:
/// the two inputs are all this needs, and a function that reaches into a backend
/// handle for one of them cannot be exercised without standing a backend up. Same
/// reasoning as `ActiveContext::for_window` — narrow the input, and the wiring
/// becomes testable instead of merely inspectable.
fn window_title_text(work_title: Signal<String>, ordinal: &Signal<usize>) -> Signal<String> {
    work_title.zip(ordinal).map(|(title, ord)| {
        let app = crate::identity::display_name();
        if title.trim().is_empty() {
            tr!(window_title_empty(app = app)).resolve_now()
        } else if *ord > 1 {
            tr!(window_title_numbered(
                title = title.clone(),
                app = app,
                n = *ord as i64
            ))
            .resolve_now()
        } else {
            tr!(window_title(title = title.clone(), app = app)).resolve_now()
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
    /// Plain mirror of the margin-lane switch, read by View ▸ Margin marks.
    /// Shared process-wide like `comments_menu` above: one persisted app
    /// preference, not anything a window owns.
    margin_lane_menu: Signal<bool>,
    /// The app-global quit sequencer, built **here** rather than per window and
    /// rather than in `main`: a quit spans every window, so two windows each
    /// running their own sequence over the same Works would prompt twice for
    /// each; and this factory already holds the only two things it needs (the
    /// registry and the autosave switch).
    quit: crate::project::QuitSequencer,
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
        margin_lane_menu: Signal<bool>,
    ) -> Self {
        Self {
            quit: crate::project::QuitSequencer::new(
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
            margin_lane_menu,
        }
    }

    /// The app-global [`QuitSequencer`](crate::project::QuitSequencer) — one per
    /// process, minted in [`Self::new`].
    ///
    /// Handed out because the **Launcher** needs it too. Quitting is not a
    /// window's business: single-instance means the Launcher can be on screen
    /// beside any number of project windows, so a Quit that only closed the
    /// window it was fired from would leave the app running and the command
    /// lying about what it does. Both windows therefore register `app.quit` over
    /// this one sequencer — which is also why it must be shared and not rebuilt:
    /// two sequencers would each walk the same open Works and prompt twice for
    /// each.
    pub fn quit(&self) -> crate::project::QuitSequencer {
        self.quit.clone()
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
    /// state through (`tags::tag_chip`, `overview`'s tree-
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
    ///     teksilo's window map and the key its geometry is saved under.
    ///
    /// The refcount `attach` bumps is released symmetrically by
    /// `WorkRegistry::remove_window`, driven by this window's own `on_removed`
    /// hook — the same path every other window's release already takes.
    /// `open_item` is the one `BinderItem` the new window opens on arrival — the
    /// tab-strip menu's "Move into a new window". `None` for a plain Work ▸ New
    /// Window, which keeps the empty desk an attached window starts with.
    pub fn attached_window_config(
        &self,
        work_id: u64,
        path: &str,
        open_item: Option<u64>,
    ) -> Option<(WindowConfig, InitialWindowState)> {
        let session = self.registry.attach(work_id)?;
        let ordinal = self.registry.reserve_window_ordinal(work_id);
        Some(self.build_window(
            PendingAction::AttachExisting {
                work_id,
                path: path.to_string(),
                ordinal,
                open_item,
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
            crate::import_document::ImportDocumentViewModel::new(app_ctx_root.clone(), ids.clone());
        let registry = self.registry.clone();
        let quit = self.quit.clone();
        // Let this Work's on-close backup hand control back to the quit sequencer
        // when it lands (see `BackupSchedulerViewModel::do_close`'s
        // `PendingExit::Quit` arm).
        session.backup_scheduler.set_quit_sequencer(quit.clone());
        // Per WINDOW, for the same reason `format` is: "which surface has the
        // caret" is a property of a window, and two windows on one project can
        // have it in different places. The entity half is Tier 2 underneath —
        // one structural history per `Work`, shared by its windows — which is
        // exactly right: undoing a rename in either window undoes the rename.
        let undo_group = crate::edit::UndoGroupViewModel::new(
            format.clone(),
            crate::edit::EntityDomain::new(
                app_ctx_root.clone(),
                session.ids.stack_id.clone(),
                session.save_state.clone(),
            ),
        );
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
        let title_text = window_title_text(single_work.title(), &window_ordinal);
        let autosave_menu = self.autosave_menu.clone();
        let spellcheck_menu = self.spellcheck_menu.clone();
        let comments_menu = self.comments_menu.clone();
        let margin_lane_menu = self.margin_lane_menu.clone();
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
        let templates_submenu_id = teksilo::core::menu_item_id::MenuItemId::next();
        // Increment 4 (the Go menu). Per WINDOW, same rationale as `scene_focused`
        // just above: it mirrors *this* window's own focused item, so a second
        // simultaneously-open project window's Go menu never reflects the wrong
        // window's answer. See `GoAvailability`'s module doc.
        let go = crate::go::GoAvailability::new();
        // The "jump to any item" popup's state. Per WINDOW, same rationale as
        // `go` just above and for a sharper reason: it owns its own binder tree
        // model and selection, so a process-wide instance would let one window's
        // popup drive another window's editor. See `GoToViewModel`'s module doc.
        let go_to = crate::go::GoToViewModel::new(app_ctx_root.clone(), ids.clone());
        // Increment 1 of distraction-free (plain fullscreen). Per WINDOW,
        // same rationale as `scene_focused` just above: this remembers
        // *this* window's own pre-fullscreen placement, so a second
        // simultaneously-open project window's F11 never restores (or
        // clobbers) the wrong window's memory. See `FullscreenViewModel`'s
        // module doc.
        let fullscreen = crate::shared::FullscreenViewModel::new();
        // Increment 2 of distraction-free (chrome collapse). Per WINDOW, same
        // rationale as `fullscreen` just above — see `FocusViewModel`'s
        // module doc for why it keeps its own independent placement memory
        // rather than sharing `fullscreen`'s.
        let focus = crate::shared::FocusViewModel::new();
        // The surface the mode actually shows. Per window for the same reason
        // `focus` is; minted empty because everything it needs beyond `focus`,
        // `go_to` and `ids` is built inside `App::build`, which hands it over
        // there (see `DistractionFreeSurfaceViewModel`'s module doc).
        let df_surface = crate::distraction_free::DistractionFreeSurfaceViewModel::new(
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
        let restore_vm = crate::backup::BackupRestoreViewModel::new(
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
        let project_switch = crate::project::ProjectSwitchViewModel::new(
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
            // The window-teardown hook (see `teksilo::core::window::WindowConfig::
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

                // Custom Teksilo title bar with a model-driven hamburger menu
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
                                undo_group: undo_group.clone(),
                                export: export.clone(),
                                single_work: single_work.clone(),
                                single_work_info: single_work_info.clone(),
                                ids: session.ids.clone(),
                                autosave_menu: autosave_menu.clone(),
                                spellcheck_menu: spellcheck_menu.clone(),
                                comments_menu: comments_menu.clone(),
                                margin_lane_menu: margin_lane_menu.clone(),
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
                                pace_available: session.pace_summary_available.clone(),
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
                        //
                        // `Suppress`, not `Coexist`: on macOS the same `MenuModel`
                        // is mirrored into the global bar at the top of the screen
                        // (see `install_native_menu` in `lib.rs`) and the in-window
                        // hamburger disappears, leaving the brand mark and the
                        // window controls in the title bar. Two copies of one menu
                        // — one of them where no Mac user looks — is what `Coexist`
                        // would buy. The flag is inert off macOS: the hamburger and
                        // its dropdowns render exactly as before, which is why this
                        // is one line and not a `cfg`.
                        let menubar = MenuBar::from_model(menu)
                            .native_on_macos(NativeMenuMode::Suppress)
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
                        let brand_icon = Padding::new(0.0, 0.0, 0.0, 8.0)
                            .child(crate::identity::brand_mark().widget(25.0));
                        let leading = teksu!(
                            HStack {
                                spacing: 5.0
                                alignment: teksilo::tokens::VAlignment::Center
                                child: brand_icon
                                child: menubar
                            }
                        );
                        let trailing_controls = teksu!(
                            HStack {
                                spacing: 5.0
                                alignment: teksilo::tokens::VAlignment::Center
                                SpellcheckToggleButton::new(spellcheck_menu.clone())
                                ExportSplitButton::new(export.clone())
                            }
                        );
                        let center_content = teksu!(
                            HStack {
                                    spacing: 5.0
                                    alignment: teksilo::tokens::VAlignment::Center
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
                                            TextWidget::new(lit!(crate::identity::display_name())) {
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

                        tree.add_boxed(Box::new(teksu!(
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
                    None => tree.add(
                        TextWidget::new(lit!(crate::identity::display_name()))
                            .text(title_text.clone()),
                    ),
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
                    margin_lane_menu.clone(),
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
                    undo_group.clone(),
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
mod tests;
