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

use bastyde::prelude::*;
use bastyde::res;
use bastyde::widgets::primitives::icon_widget::IconMode;
use bastyde::widgets::{
    Center, CollapsePolicy, DeadZone, DockSide, Expand, HStack, IconButton, IconButtonSize,
    IconWidget, MenuBar, MenuEntry, MenuModel, Padding, TextWidget, TitleBar, VStack, WindowFrame,
};

use frontend::AppContext;
use frontend::common::entities::WorkShape;

use crate::app::{App, PendingAction, PendingExit, guard_unsaved_exit};
use crate::app_ids::AppIds;
use crate::export::split_button::ExportSplitButton;
use crate::intents::AppIntent;
use crate::models::{TreeExpansionService, WorkspaceLayoutService};
use crate::panels::welcome::WelcomePanel;
use crate::sessions::{WorkRegistry, WorkSession};
use crate::shell::project_switcher_button::ProjectSwitcherButton;
use crate::spellcheck::SpellcheckService;
use crate::spellcheck::toggle_button::SpellcheckToggleButton;
use crate::tabs::shared::editor::VisibleWhen;
use crate::view_models::{
    ALIGN_CENTER, ALIGN_LEFT, BackupSettingsViewModel, DIR_AUTO, DIR_LTR, DIR_RTL,
    ExportViewModel, FormatViewModel, OutlineViewModel, SaveAsViewModel, scope_label,
};
use export_management::ExportScopeKind;

// Re-export path→window identity so `shell::windows::*` stays the
// stable public surface for call sites.
pub use super::window_ids::{
    LAUNCHER_WINDOW_ID, attached_window_id_for, open_or_focus_project,
    resolve_project_window, window_id_for,
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
/// Work ▸ New Window is what reaches the suffix ([`attached_window_config`]).
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

/// The Launcher window: the Welcome UI hosted as a real top-level window
/// (not a modal), reusing [`WelcomePanel`]/`WelcomeViewModel` verbatim — no
/// duplicated recents/examples UI.
///
/// Skribisto's windows are all `DecorationsMode::CustomChrome` — the OS draws
/// no title bar at all, so `TitleBar` + `WindowFrame` *are* the window chrome
/// (a `Native`-decorated window would be the odd one out: no consistent
/// drag/resize/traffic-light behaviour with the rest of the app). "Appropriate
/// chrome" for the launcher means a **leaner** `TitleBar` — the drag region,
/// the window title, and the window controls (min/max/close), same as the
/// project window's — with no `MenuBar`/File menu and no `ProjectSwitcherButton`
/// (both assume an open project). Mirrors `ProjectWindowFactory::window_config`'s
/// `title_bar`/`WindowFrame` composition.
///
/// No close guard: the Launcher holds no unsaved state, so closing it (its
/// own title-bar close button, Alt+F4, or the panel's inline close button)
/// always succeeds — and since it is Skribisto's other-than-a-project window,
/// closing it while it is the only window quits the process. That is the
/// intended "close the launcher to exit" behaviour.
pub fn launcher_window_config(app_ctx: Rc<AppContext>) -> WindowConfig {
    // The Launcher is deliberately not resizable (min == max): `WelcomePanel`
    // fills it edge to edge, so this is the one place its proportions — the
    // 264 dp sidebar against the recents list — are decided.
    const W: u32 = 820;
    const H: u32 = 590;
    WindowConfig::new()
        .id(LAUNCHER_WINDOW_ID)
        // The OS-level title (taskbar, alt-tab, window list) stays the app's
        // name — that identifies the *process*. The visible custom title bar
        // below says "Welcome to Skribisto": that names the *screen*, and it
        // is the reason the Welcome content no longer carries a title strip of
        // its own.
        .title("Skribisto")
        .size(W, H)
        .min_size(W, H)
        .max_size(W, H)
        .decorations(DecorationsMode::CustomChrome)
        // Consume an xdg-activation startup token (set by the desktop, or by
        // another instance's "open in new window") so a bare launch comes up
        // focused on Wayland — same rationale as the project window.
        .activate_from_env(true)
        .root(move |tree, _state| {
            let theme = tree.theme().clone();
            // Leaner title bar: brand icon + window title, drag region, and
            // the platform's window controls — no menu, no project switcher.
            // Falls back to a plain label on any platform whose host is
            // unavailable (mirrors the project window's fallback).
            let title_bar = match tree.title_bar_host() {
                Some(host) => {
                    // Leading inset: the icon is the first thing in the title
                    // bar's centre slot, which starts at the window's left edge
                    // — bare, it sits flush against it. The project window has
                    // no such gap to close: its `MenuBar` hamburger leads, and
                    // an `IconButton` carries its own inset.
                    let brand_icon = tree.add(
                        Padding::new(0.0, 0.0, 0.0, 8.0).child(
                            IconWidget::from_raster(
                                res!("../../resources/icons/skribisto.png"),
                                25.0,
                            )
                            .mode(IconMode::FullColor),
                        ),
                    );
                    tree.add_boxed(Box::new(bati!(
                    TitleBar::new(host) {
                        background: SurfaceRole::Main
                        center: Expand::horizontal {
                            HStack {
                                spacing: 5.0
                                alignment: bastyde::tokens::VAlignment::Center
                                #{brand_icon}
                                Expand::horizontal {
                                    Center {
                                        // The window's title bar names the screen
                                        // ("Welcome to Skribisto"), so the Welcome
                                        // content below needs no title strip of its
                                        // own — the two together were a window
                                        // inside a window. Project windows keep the
                                        // bare app name here.
                                        TextWidget::new(tr!(welcome_title())) {
                                            style: theme.typography.body_bold.clone()
                                            color: TextRole::Primary
                                        }
                                    }
                                }
                            }
                        }
                        close_action: |ctx| ctx.close_window()
                    }
                    )))
                }
                None => tree.add(TextWidget::new(tr!(welcome_title()))),
            };
            // No `Center`: the Welcome content fills the window (see
            // `welcome_panel`'s module docs) — centring a fixed-size card in
            // here is what put a gutter down each side of it.
            let body = tree.add(Expand::new().child(WelcomePanel::new(app_ctx.clone())));
            let inner = tree.add(
                VStack::new()
                    .spacing(0.0)
                    .add_child(title_bar)
                    .add_child(body),
            );

            // Edge resize handles, same as the project window (skipped where
            // the host doesn't need them, e.g. macOS).
            match tree.title_bar_host() {
                Some(host) if host.needs_custom_resize_handles() => {
                    tree.add(WindowFrame::new(host).thickness(6.0).content_id(inner))
                }
                _ => inner,
            }
        })
}

/// Everything needed to build a fresh **project window** — shared,
/// process-wide state created once in `main`. Registered as `app_state` so
/// it is reachable both from `main`'s initial-window setup and from a
/// runtime `ctx.open_window(...)` call, which is how the Launcher opens the
/// project window once a project is picked, created, or opened via a Plume
/// import's "Open now".
/// A Format-menu row that reflects document state: a reflect-only checkmark
/// mirroring the same signal the dock's button binds, so the two surfaces
/// cannot disagree about whether the selection is bold.
///
/// `checked` and not `checkable`: the mark mirrors the document read-only, and
/// the command is what changes it. A checkable row would write the signal on
/// click and fight the value the editor reports back.
fn mark(
    vm: &FormatViewModel,
    label: bastyde::i18n::LocalizedString,
    enabled: Signal<bool>,
    state: Signal<bool>,
    run: fn(&FormatViewModel),
) -> MenuEntry {
    let vm = vm.clone();
    MenuEntry::new(label)
        .enabled(enabled)
        .checked(state)
        .on_activate(move |c| {
            run(&vm);
            c.request_frame();
            vm.refocus(c);
        })
}

/// A Format-menu row that just runs a command.
///
/// Like the dock's buttons it requests a frame: the pointer is on the menu
/// overlay and the editor is unfocused, so nothing else schedules the repaint
/// that shows the edit. It then puts focus back where the writer was typing —
/// reaching a menu item took it away, and the dock and context menu do not have
/// that problem (the dock's buttons are non-focusable, and dismissing the
/// context menu restores focus by itself).
fn command(
    vm: &FormatViewModel,
    label: bastyde::i18n::LocalizedString,
    enabled: Signal<bool>,
    run: fn(&FormatViewModel),
) -> MenuEntry {
    let vm = vm.clone();
    MenuEntry::new(label)
        .enabled(enabled)
        .on_activate(move |c| {
            run(&vm);
            c.request_frame();
            vm.refocus(c);
        })
}

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
        // Per WINDOW, not per process: unlike `spellcheck_menu` (a global
        // setting, correctly shared), this tracks which surface *this* window
        // has focused. A process-wide one would let a second project window
        // grey out this window's Format menu.
        let scene_focused = Signal::new(false);
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
                let title_bar = match tree.title_bar_host() {
                    Some(host) => {
                        // Model-style menu, collapsed to a hamburger (☰).
                        let menu_ctx = app_ctx_root.clone();
                        let export_for_menu = export.clone();
                        let menu_work = single_work.clone();
                        let menu_work_info = single_work_info.clone();
                        let menu_ids = session.ids.clone();
                        let menu_autosave = autosave_menu.clone();
        // The menu closure below is `move`, so give it its own clone — the title-bar toggle
        // still needs the original (same reason `menu_autosave` exists).
        let menu_spellcheck = spellcheck_menu.clone();
                        let menu_scene_focused = scene_focused.clone();
                        let menu_go = go.clone();
                        let menu_format_vm = format.clone();
                        let menu_save_as = save_as_vm.clone();
                        let menu_backup_mode = backup_mode.clone();
                        let menu_unsaved = unsaved.clone();
                        let menu = MenuModel::new().menu(tr!(menu_work()), move |m| {
                            let file_ctx = menu_ctx.clone();
                            let folder_ctx = menu_ctx.clone();
                            let file_ids = menu_ids.clone();
                            let folder_ids = menu_ids.clone();
                            let folder_work = menu_work.clone();
                            let save_as_file_vm = menu_save_as.clone();
                            let save_as_folder_vm = menu_save_as.clone();
                            // A work is open iff its WorkInfo shape is known.
                            let show_open = menu_work_info.shape().map(|s| s.is_some());
                            // Bug 2: offer only the *other* shape — a zip project
                            // shows "Save as folder", a folder project shows
                            // "Save as single file". Both collapse when no
                            // project is open (`shape` is `None`). Reactive via
                            // the overlay menu's `visible_when`.
                            let show_save_file =
                                menu_work_info.shape().map(|s| *s == Some(WorkShape::Folder));
                            let show_save_folder =
                                menu_work_info.shape().map(|s| *s == Some(WorkShape::Zip));
                            // Autosave hides the manual "Save" item (+ its Ctrl+S
                            // accelerator); the save then runs on the debounce timer.
                            // Also hidden in backup mode (Save is off there).
                            let show_manual_save = menu_autosave
                                .zip(&menu_backup_mode)
                                .map(|(a, bm)| !*a && !*bm);
                            // …and it greys out while there is nothing to save.
                            // Same signal as the `editor.save` action/shortcut
                            // (app.rs), so the item, Ctrl+S and the intent are
                            // enabled or disabled as one.
                            let can_save = crate::app::can_save(&menu_unsaved, &menu_backup_mode);
                            // "Back up now" shows only for an open, non-backup project.
                            let show_backup_now = show_open
                                .zip(&menu_backup_mode)
                                .map(|(o, bm)| *o && !*bm);
                            // New / Open route through the global `work.new` /
                            // `work.open` actions (registered in `App::build`), so
                            // the same code path serves the menu and the Ctrl+N /
                            // Ctrl+O shortcuts.
                            m.item(
                                MenuEntry::new(tr!(menu_new_work()))
                                    .intent("work.new")
                                    .shortcut("work.new"),
                            )
                            .item(
                                MenuEntry::new(tr!(menu_open_work()))
                                    .intent("work.open")
                                    .shortcut("work.open"),
                            )
                            // A second window onto the Work this one already
                            // shows — same project, same edits, its own desk.
                            // Hidden with no project open: there is nothing to
                            // open a second view of. Same `show_open` signal the
                            // Close/Backups entries below use, so the whole
                            // "needs a project" group appears and disappears
                            // together.
                            .item(
                                MenuEntry::new(tr!(menu_new_window()))
                                    .visible(show_open.clone())
                                    .intent("window.new")
                                    .shortcut("window.new"),
                            )
                            // Import from another writing app. A submenu so
                            // more importers can slot in later; each opens its
                            // own panel via a global action.
                            .submenu(tr!(menu_import_from()), |s| {
                                s.item(
                                    MenuEntry::new(tr!(menu_import_plume()))
                                        .intent("work.import_plume"),
                                )
                            })
                            // The same focus-adaptive quick-export list the title-bar
                            // Export split-button shows — one source (the view-model's
                            // `applicable` scopes), two surfaces. Each visible entry fires
                            // the data-bearing `export.scope` intent; the disabled hint keeps
                            // the submenu from ever being empty.
                            .submenu(tr!(menu_export()), {
                                let ex = export_for_menu.clone();
                                move |s| {
                                    let entry = |scope: ExportScopeKind| {
                                        let seen = scope.clone();
                                        let fire = scope.clone();
                                        MenuEntry::new(scope_label(&scope))
                                            .visible(
                                                ex.applicable_signal()
                                                    .map(move |v| v.contains(&seen)),
                                            )
                                            .on_activate(move |c| {
                                                c.send_intent(AppIntent::ExportScoped {
                                                    scope: fire.clone(),
                                                })
                                            })
                                    };
                                    s.item(entry(ExportScopeKind::CurrentBook))
                                        .item(entry(ExportScopeKind::CurrentPart))
                                        .item(entry(ExportScopeKind::CurrentChapter))
                                        .item(entry(ExportScopeKind::CurrentScene))
                                        .item(entry(ExportScopeKind::CurrentNote))
                                        .item(entry(ExportScopeKind::CurrentFolder))
                                        // Choose… — the checkbox-tree picker (always
                                        // available with a project open).
                                        .item(entry(ExportScopeKind::Custom))
                                        .item(
                                            MenuEntry::new(tr!(menu_export_none()))
                                                .visible(
                                                    ex.applicable_signal().map(|v| v.is_empty()),
                                                )
                                                .enabled(false),
                                        )
                                }
                            })
                            .separator()
                            // Flush editors to the store + write to disk (also Ctrl+S).
                            .item(
                                MenuEntry::new(tr!(menu_save()))
                                    .visible(show_manual_save)
                                    .enabled(can_save)
                                    .intent("editor.save")
                                    .shortcut("editor.save"),
                            )
                            // Convert the open project to a single zipped `.skrib`
                            // at a user-chosen location (native save dialog).
                            .item(MenuEntry::new(tr!(menu_save_as_file())).visible(show_save_file).on_activate(
                                move |ectx| {
                                    let ctx = file_ctx.clone();
                                    let vm = save_as_file_vm.clone();
                                    let req = FileDialogRequest::save_file()
                                        .title("Save as single .skrib file")
                                        .default_file_name(format!(
                                            "{}.skrib",
                                            crate::project_stem(&ctx, &file_ids)
                                        ))
                                        .add_filter("Skribisto work", &["skrib"]);
                                    let _ = ectx.save_file(req, move |res, ectx2| {
                                        if let FileDialogResult::Saved(Some(path)) = res {
                                            // `begin` flushes the live editor buffers into the
                                            // store first — the background op is read-only, so
                                            // without it we would write the pre-edit prose.
                                            vm.begin(ectx2, path.to_string_lossy().into_owned(), false);
                                        }
                                    });
                                },
                            ))
                            // Convert the open project to an exploded folder at a
                            // user-chosen directory (native folder picker).
                            .item(MenuEntry::new(tr!(menu_save_as_folder())).visible(show_save_folder).on_activate(
                                move |ectx| {
                                    let ctx = folder_ctx.clone();
                                    // Bug 1: the picked folder is the *parent* —
                                    // write into a subfolder named after the Work
                                    // title (sanitized), falling back to the
                                    // project file stem when the title is empty.
                                    let title = folder_work.title().get();
                                    let raw = if title.trim().is_empty() {
                                        crate::project_stem(&ctx, &folder_ids)
                                    } else {
                                        title
                                    };
                                    let name = crate::sanitize_folder_name(&raw);
                                    let req = FileDialogRequest::pick_folder()
                                        .title("Choose a parent folder for the work");
                                    let vm = save_as_folder_vm.clone();
                                    let _ = ectx.pick_folder(req, move |res, ectx2| {
                                        if let FileDialogResult::Folder(Some(path)) = res {
                                            let target = path
                                                .join(&name)
                                                .to_string_lossy()
                                                .into_owned();
                                            // Flushes the editors first — see the sibling item.
                                            vm.begin(ectx2, target, true);
                                        }
                                    });
                                },
                            ))
                            // Manual backup — routed through the guarded
                            // `backup.now` action in `App` (which flushes the
                            // editors into the store first, resolves the
                            // configured destinations, and drives the toast).
                            // Hidden while a project is open in backup mode.
                            .item(
                                MenuEntry::new(tr!(menu_backup()))
                                    .visible(show_backup_now)
                                    .intent("backup.now"),
                            )
                            // Browse the project's backup files (open / reveal / delete).
                            .item(
                                MenuEntry::new(tr!(menu_backups_list()))
                                    .visible(show_open.clone())
                                    .intent("backups.show"),
                            )
                            // Close the open work — routed through the guarded
                            // `work.close` action (unsaved-changes prompt /
                            // autosave-ensure live in `App`). Now returns to the
                            // Launcher instead of leaving an empty window.
                            .item(
                                MenuEntry::new(tr!(menu_close_work()))
                                    .visible(show_open)
                                    .intent("work.close")
                                    .shortcut("work.close"),
                            )
                            .separator()
                            .item(MenuEntry::new(tr!(menu_welcome())).intent("welcome.show"))
                            .item(
                                MenuEntry::new(tr!(menu_settings()))
                                    .intent("app.settings")
                                    .shortcut("app.settings"),
                            )
                            .separator()
                            .item(
                                MenuEntry::new(tr!(menu_quit()))
                                    .intent("app.quit")
                                    .shortcut("app.quit"),
                            )
                        })
                        .menu(tr!(menu_view()), {
                            // Reflect-only checkmarks: they mirror each dock's truth
                            // without writing it; the toggles are driven by the
                            // `outline.toggle` (F9) / `preview.toggle` (F10) intents.
                            let outline = outline.clone();
                            // Increment 1 of distraction-free (plain fullscreen): a
                            // reflect-only checkmark straight off THIS window's own
                            // `WindowState::placement()` — the same round-tripping
                            // signal `view.fullscreen`'s action writes and the OS
                            // reads back (see `FullscreenViewModel`'s doc), so the
                            // mark stays correct even if fullscreen is left behind
                            // the app's back (KDE's own shortcut, the titlebar).
                            let is_fullscreen = state
                                .placement()
                                .clone()
                                .map(|p| *p == WindowPlacement::Fullscreen);
                            // Increment 2 of distraction-free: this checkmark reflects
                            // `FocusViewModel::active_signal()` directly — unlike
                            // `is_fullscreen` above, this is this window's own live
                            // state, not something round-tripped off the OS, so there
                            // is no fresher source to read it from.
                            let is_focus_mode = focus.active_signal();
                            move |m| {
                                // The bottom band has no persistent reveal affordance
                                // of its own — a hidden top/bottom side collapses its
                                // rail with it (unlike leading/trailing, whose rail
                                // survives as the way back). So the menu entry is not
                                // a convenience here, it is the only way to bring the
                                // preview back once it is closed.
                                let preview_visible = outline
                                    .docking()
                                    .side_visible_signal(DockSide::Bottom);
                                m.item(
                                    MenuEntry::new(tr!(menu_outline()))
                                        .checked(outline.is_visible())
                                        .intent("outline.toggle")
                                        .shortcut("outline.toggle"),
                                )
                                // Reveal the leading search & replace dock. A plain
                                // action (not a reflect-only checkbox): the search
                                // dock is one of two switchable leading tabs, not a
                                // side that is simply shown or hidden.
                                .item(
                                    MenuEntry::new(tr!(menu_search()))
                                        .intent("search.show")
                                        .shortcut("search.show"),
                                )
                                // Reveal the trash dock — like Search, a plain action
                                // (a switchable leading tab, not a shown/hidden side).
                                .item(
                                    MenuEntry::new(tr!(menu_trash())).intent("trash.show"),
                                )
                                .item(
                                    MenuEntry::new(tr!(menu_search_preview()))
                                        .checked(preview_visible)
                                        .intent("preview.toggle")
                                        .shortcut("preview.toggle"),
                                )
                                .separator()
                                // Increment 1 of distraction-free: plain fullscreen,
                                // not the collapsed-chrome mode itself (that is a
                                // later increment). F11, the platform convention.
                                .item(
                                    MenuEntry::new(tr!(menu_fullscreen()))
                                        .checked(is_fullscreen)
                                        .intent("view.fullscreen")
                                        .shortcut("view.fullscreen"),
                                )
                                // Increment 2 of distraction-free: chrome
                                // collapse + docks disabled + fullscreen,
                                // together, as one per-window mode. Shift+F11.
                                .item(
                                    MenuEntry::new(tr!(menu_focus_mode()))
                                        .checked(is_focus_mode)
                                        .intent("view.focus_mode")
                                        .shortcut("view.focus_mode"),
                                )
                            }
                        })
                        // Format — marks the author places in the prose itself, as
                        // opposed to Tools, which processes the manuscript. A scene
                        // break lives here because it is something you *write*, not
                        // something the compiler infers from binder structure.
                        // `MenuEntry` carries no tooltip — the menubar model has no
                        // such affordance — so the two tiers are explained by the
                        // rich tooltips on their glyph pickers in
                        // Settings ▸ Compile & Export, which is where a writer
                        // decides what each one prints as.
                        .menu(tr!(menu_format()), {
                            // Enabled on the *sticky* target, not on live focus:
                            // opening this menu moves focus to the menu overlay,
                            // so an enablement keyed on focus would grey every
                            // row out at the instant the user reached for one.
                            // Scene breaks keep their own narrower gate — the
                            // same predicate the compiler uses, so the menu can
                            // never offer a mark the exporter would ignore.
                            //
                            // Rows stay visible when disabled: a greyed row still
                            // teaches that the feature exists and what its
                            // shortcut is, and still reaches the a11y tree. That
                            // is the opposite of the dock, which hides what does
                            // not apply — a menu is a map of what exists, a dock
                            // is a set of what applies right now.
                            //
                            // Text-only, with no glyphs: `MenuEntry` has no
                            // `.icon()`. Parity with the dock means the same
                            // commands and the same state, not the same look.
                            let on_scene = menu_scene_focused.clone();
                            let f = menu_format_vm.clone();
                            move |m| {
                                let on = f.has_target();
                                // Bold/Italic/Underline are handled inside
                                // `RichTextEditor`'s own key dispatch, not the
                                // shortcut registry, so there is no id to bind —
                                // the chord travels in the label instead.
                                let mut m = m
                                    .item(mark(&f, tr!(menu_format_marks_bold()), on.clone(),
                                        f.bold(), FormatViewModel::toggle_bold))
                                    .item(mark(&f, tr!(menu_format_marks_italic()), on.clone(),
                                        f.italic(), FormatViewModel::toggle_italic))
                                    .item(mark(&f, tr!(menu_format_marks_underline()), on.clone(),
                                        f.underline(), FormatViewModel::toggle_underline))
                                    .item(mark(&f, tr!(menu_format_marks_strike()), on.clone(),
                                        f.strikethrough(), FormatViewModel::toggle_strikethrough))
                                    .item(mark(&f, tr!(menu_format_marks_superscript()), on.clone(),
                                        f.superscript(), FormatViewModel::toggle_superscript))
                                    .item(mark(&f, tr!(menu_format_marks_subscript()), on.clone(),
                                        f.subscript(), FormatViewModel::toggle_subscript))
                                    .item(command(&f, tr!(menu_format_marks_clear()), on.clone(),
                                        FormatViewModel::clear_formatting))
                                    .separator();

                                // Seven levels, one exclusive choice — a radio
                                // group over the index the caret already reports.
                                m = m.submenu(tr!(menu_format_heading()), {
                                    let f = f.clone();
                                    let on = on.clone();
                                    move |s| {
                                        let mut s = s;
                                        for (level, label) in [
                                            (0usize, tr!(menu_format_heading_normal())),
                                            (1, tr!(menu_format_heading_1())),
                                            (2, tr!(menu_format_heading_2())),
                                            (3, tr!(menu_format_heading_3())),
                                            (4, tr!(menu_format_heading_4())),
                                            (5, tr!(menu_format_heading_5())),
                                            (6, tr!(menu_format_heading_6())),
                                        ] {
                                            let f = f.clone();
                                            s = s.item(
                                                MenuEntry::new(label)
                                                    .enabled(on.clone())
                                                    .radio(level, f.heading())
                                                    .on_activate(move |c| {
                                                        f.set_heading(level);
                                                        c.request_frame();
                                                        f.refocus(c);
                                                    }),
                                            );
                                        }
                                        s
                                    }
                                });

                                m = m.submenu(tr!(menu_format_alignment()), {
                                    let f = f.clone();
                                    let on = on.clone();
                                    move |s| {
                                        let mut s = s;
                                        for (idx, label) in [
                                            (ALIGN_LEFT, tr!(menu_format_align_left())),
                                            (ALIGN_CENTER, tr!(menu_format_align_center())),
                                        ] {
                                            let f = f.clone();
                                            s = s.item(
                                                MenuEntry::new(label)
                                                    .enabled(on.clone())
                                                    .radio(idx, f.alignment())
                                                    .on_activate(move |c| {
                                                        f.set_alignment(idx);
                                                        c.request_frame();
                                                        f.refocus(c);
                                                    }),
                                            );
                                        }
                                        s
                                    }
                                });

                                // Paragraph direction gets the full three-way
                                // radio the dock's single toggle cannot express:
                                // "automatic" is a real state, distinct from a
                                // pinned left-to-right, and worth reaching.
                                m = m.submenu(tr!(menu_format_direction()), {
                                    let f = f.clone();
                                    let on = on.clone();
                                    move |s| {
                                        let mut s = s;
                                        for (idx, label) in [
                                            (DIR_AUTO, tr!(menu_format_direction_auto())),
                                            (DIR_LTR, tr!(menu_format_direction_ltr())),
                                            (DIR_RTL, tr!(menu_format_direction_rtl())),
                                        ] {
                                            let f = f.clone();
                                            s = s.item(
                                                MenuEntry::new(label)
                                                    .enabled(on.clone())
                                                    .radio(idx, f.direction())
                                                    .on_activate(move |c| {
                                                        f.set_direction(idx);
                                                        c.request_frame();
                                                        f.refocus(c);
                                                    }),
                                            );
                                        }
                                        s
                                    }
                                });

                                m = m
                                    .item(mark(&f, tr!(menu_format_blockquote()), on.clone(),
                                        f.blockquote(), FormatViewModel::toggle_blockquote))
                                    .submenu(tr!(menu_format_lists()), {
                                        let f = f.clone();
                                        let on = on.clone();
                                        move |s| {
                                            s.item(command(&f, tr!(menu_format_list_bullet()),
                                                on.clone(), FormatViewModel::insert_bullet_list))
                                            .item(command(&f, tr!(menu_format_list_numbered()),
                                                on.clone(), FormatViewModel::insert_numbered_list))
                                            .separator()
                                            .item(command(&f, tr!(menu_format_indent()),
                                                on.clone(), FormatViewModel::indent))
                                            .item(command(&f, tr!(menu_format_outdent()),
                                                on.clone(), FormatViewModel::outdent))
                                        }
                                    })
                                    .submenu(tr!(menu_format_table()), {
                                        let f = f.clone();
                                        let on = on.clone();
                                        move |s| {
                                            // `insert_table` takes two runtime
                                            // numbers and `MenuEntry` has no
                                            // payload slot, so the sizes are
                                            // spelled out rather than prompted.
                                            let sizes = s.submenu(
                                                tr!(menu_format_table_insert()),
                                                {
                                                    let f = f.clone();
                                                    let on = on.clone();
                                                    move |t| {
                                                        let mut t = t;
                                                        for (n, label) in [
                                                            (2usize, tr!(menu_format_table_2x2())),
                                                            (3, tr!(menu_format_table_3x3())),
                                                            (4, tr!(menu_format_table_4x4())),
                                                        ] {
                                                            let f = f.clone();
                                                            t = t.item(
                                                                MenuEntry::new(label)
                                                                    .enabled(on.clone())
                                                                    .on_activate(move |c| {
                                                                        f.insert_table(n, n);
                                                                        c.request_frame();
                                                                        f.refocus(c);
                                                                    }),
                                                            );
                                                        }
                                                        t
                                                    }
                                                },
                                            );
                                            // The row/column commands are gated
                                            // on the caret actually being in a
                                            // table — the dock hides them, a menu
                                            // greys them.
                                            let in_table = f.in_table();
                                            sizes
                                                .separator()
                                                .item(command(&f, tr!(menu_format_table_row_above()),
                                                    in_table.clone(), FormatViewModel::insert_row_above))
                                                .item(command(&f, tr!(menu_format_table_row_below()),
                                                    in_table.clone(), FormatViewModel::insert_row_below))
                                                .item(command(&f, tr!(menu_format_table_col_before()),
                                                    in_table.clone(), FormatViewModel::insert_column_before))
                                                .item(command(&f, tr!(menu_format_table_col_after()),
                                                    in_table.clone(), FormatViewModel::insert_column_after))
                                                .separator()
                                                .item(command(&f, tr!(menu_format_table_row_delete()),
                                                    in_table.clone(), FormatViewModel::remove_row))
                                                .item(command(&f, tr!(menu_format_table_col_delete()),
                                                    in_table.clone(), FormatViewModel::remove_column))
                                                .item(command(&f, tr!(menu_format_table_remove()),
                                                    in_table, FormatViewModel::remove_table))
                                        }
                                    })
                                    .separator()
                                    .item(command(&f, tr!(menu_format_undo()), f.can_undo(),
                                        FormatViewModel::undo))
                                    .item(command(&f, tr!(menu_format_redo()), f.can_redo(),
                                        FormatViewModel::redo))
                                    .separator();

                                m.item(
                                    MenuEntry::new(tr!(menu_scene_break()))
                                        .enabled(on_scene.clone())
                                        .intent("format.scene_break")
                                        .shortcut("format.scene_break"),
                                )
                                .item(
                                    MenuEntry::new(tr!(menu_major_scene_break()))
                                        .enabled(on_scene.clone())
                                        .intent("format.major_scene_break")
                                        .shortcut("format.major_scene_break"),
                                )
                            }
                        })
                        // Go — Increment 4 of distraction-free: prev/next Scene/
                        // Chapter/Note, scoped to one binder, deliberately crossing
                        // Chapter/Part/Book boundaries within it (uninterrupted
                        // drafting), never wrapping around. Six STATIC rows, gated
                        // by `.enabled()` — never `.visible()` — for the same
                        // reason the Format menu's scene-break rows stay visible
                        // when disabled (see that menu's own comment above): a
                        // greyed row still teaches the feature exists and what its
                        // shortcut namespace is, and it still reaches the a11y
                        // tree. No individual shortcuts here — the generic
                        // `go.next`/`go.prev` pair (Alt+Down/Alt+Up), registered in
                        // `app::commands::go`, delegates to whichever of these six
                        // answers the focused tab's own kind resolves to; the
                        // distraction-free strip's own Next/Previous buttons fire
                        // that same generic pair, never a reimplementation.
                        .menu(tr!(menu_go()), {
                            let go = menu_go.clone();
                            move |m| {
                                use skribisto_model::{GoDirection::{Next, Previous}, GoKind::{Chapter, Note, Scene}};
                                m.item(
                                    MenuEntry::new(tr!(menu_go_next_scene()))
                                        .enabled(go.signal(Scene, Next))
                                        .intent("go.next_scene"),
                                )
                                .item(
                                    MenuEntry::new(tr!(menu_go_prev_scene()))
                                        .enabled(go.signal(Scene, Previous))
                                        .intent("go.prev_scene"),
                                )
                                .item(
                                    MenuEntry::new(tr!(menu_go_next_chapter()))
                                        .enabled(go.signal(Chapter, Next))
                                        .intent("go.next_chapter"),
                                )
                                .item(
                                    MenuEntry::new(tr!(menu_go_prev_chapter()))
                                        .enabled(go.signal(Chapter, Previous))
                                        .intent("go.prev_chapter"),
                                )
                                .item(
                                    MenuEntry::new(tr!(menu_go_next_note()))
                                        .enabled(go.signal(Note, Next))
                                        .intent("go.next_note"),
                                )
                                .item(
                                    MenuEntry::new(tr!(menu_go_prev_note()))
                                        .enabled(go.signal(Note, Previous))
                                        .intent("go.prev_note"),
                                )
                                .separator()
                                // "Go to…" — the six rows above step relative to
                                // where you are; this one jumps anywhere. Always
                                // enabled: unlike the stepping rows there is no
                                // "is there a target" question to answer, and an
                                // empty binder simply opens an empty list.
                                // The action behind it belongs to the `GoToButton`
                                // widget (`PopoverWidget::open_action`), because
                                // presenting a popover needs an `EventContext`.
                                .item(
                                    MenuEntry::new(tr!(menu_go_to()))
                                        .intent("go.to")
                                        .shortcut("go.to"),
                                )
                            }
                        })
                        // Tools — where every office suite keeps spell-check. Its own
                        // top-level section rather than a View entry: View toggles what a
                        // *dock* shows, whereas this changes how the manuscript is *processed*.
                        .menu(tr!(menu_tools()), move |m| {
                            // The master spell-check switch. `checked(..)` is Bastyde's
                            // **reflect-only** mark — it mirrors the setting read-only and the
                            // intent is what drives it. NOT `.checkable()`, which would write
                            // the signal on click and fight the store-backed value.
                            m.item(
                                MenuEntry::new(tr!(menu_spellcheck()))
                                    .checked(menu_spellcheck.clone())
                                    .intent("spellcheck.toggle")
                                    .shortcut("spellcheck.toggle"),
                            )
                        })
                        // Help sits last, as it does on every desktop platform.
                        // "About" is fired by name only (no payload), so it needs
                        // no `AppIntent` variant — just the global action that
                        // `App::build` registers.
                        .menu(tr!(menu_help()), |m| {
                            m.item(MenuEntry::new(tr!(menu_about())).intent("app.about"))
                        });
                        let menubar = MenuBar::from_model(menu)
                            .collapse_policy(CollapsePolicy::Always)
                            .hamburger_size(IconButtonSize::Large);

                        // Increment 2 of distraction-free: the menu bar, the title/
                        // switcher row and the trailing controls all collapse — and so
                        // does the enclosing `TitleBar`, below. `VisibleWhen` (not
                        // `Switcher`) throughout — dormant, not torn down, the same
                        // pattern the synopsis toggle already uses.
                        //
                        // The bar used to stay mounted so the window controls remained
                        // reachable, on the reasoning that a wedged Escape must never
                        // also take away the OS's own way to close the window. That
                        // reasoning predates the strip's Exit button being *mandatory*
                        // (no settings combination can remove it — `focus_strip`'s
                        // `exit_survives_every_combination_of_the_chrome_settings`
                        // pins it), and it left a min/max/close cluster floating over a
                        // fullscreen window, which no desktop convention does: macOS
                        // hides the traffic lights, Windows fullscreen has no caption
                        // buttons, browsers and editors hide their chrome outright.
                        // `WindowPlacement::Fullscreen`'s own doc says "title bar and
                        // all chrome hidden". Minimize and maximize are meaningless for
                        // a window with no frame.
                        //
                        // The replacement guarantee, which is strictly stronger:
                        //   * distraction-free → the strip's Exit button (always
                        //     present), Shift+F11, and Escape;
                        //   * plain fullscreen → the menu bar is still on screen, so
                        //     View ▸ Fullscreen and F11;
                        //   * either → Ctrl+W, Ctrl+Q, and the compositor's own
                        //     unfullscreen/close.
                        let chrome_visible = focus.active_signal().map(|active| !*active);
                        // Read from the WINDOW's own placement, not from
                        // `FullscreenViewModel`/`FocusViewModel`: those are two
                        // independent memories (F11 and Shift+F11 each keep their own),
                        // and an OS-initiated fullscreen goes through neither. The
                        // placement is the single fact all three agree on.
                        let controls_visible =
                            state.placement().map(|p| !p.is_fullscreen());
                        let menubar = VisibleWhen::new(chrome_visible.clone(), menubar);
                        let trailing_controls = VisibleWhen::new(
                            chrome_visible.clone(),
                            bati!(
                                HStack {
                                    spacing: 5.0
                                    alignment: bastyde::tokens::VAlignment::Center
                                    SpellcheckToggleButton::new(spellcheck_menu.clone())
                                    ExportSplitButton::new(export.clone())
                                }
                            ),
                        );
                        let center_content = VisibleWhen::new(
                            chrome_visible.clone(),
                            bati!(
                                HStack {
                                    spacing: 5.0
                                    alignment: bastyde::tokens::VAlignment::Center
                                    // The `center` slot lives inside the TitleBar's
                                    // DragRegion, which is published to the OS as the
                                    // window caption (on Windows: WM_NCHITTEST ->
                                    // HTCAPTION). The OS owns caption pixels outright,
                                    // so a bare button here would never see a click —
                                    // it would only drag the window. `DeadZone` carves
                                    // these two controls back out of the caption, and
                                    // (on every platform) stops a few px of pointer
                                    // jitter during a click from arming the window drag.
                                    DeadZone {
                                        HStack {
                                            spacing: 5.0
                                            alignment: bastyde::tokens::VAlignment::Center
                                            IconButton::new(IconWidget::from_raster(
                                                res!("../../resources/icons/skribisto.png"),
                                                25.0,
                                            )
                                            .mode(IconMode::FullColor)) {
                                                tooltip: tr!(tooltip_welcome())
                                                size: IconButtonSize::Large
                                                on_activate_fn: |ctx| ctx.send_intent(Intent::new("welcome.show"))
                                            }
                                            ProjectSwitcherButton::new(
                                                app_ctx_root.clone(),
                                                single_work.clone(),
                                                single_work_info.clone(),
                                            )
                                        }
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
                            ),
                        );

                        // The whole bar collapses in distraction-free: every slot inside
                        // it is already gated, so what would be left is an empty band
                        // ~40 px tall above the writing column. Wrapping it here
                        // reclaims that height for the manuscript, which is the point
                        // of the mode.
                        tree.add_boxed(Box::new(VisibleWhen::new(
                            chrome_visible,
                            bati!(

                            TitleBar::new(host) {
                                background: SurfaceRole::Main
                                leading: menubar
                                // Left of the window buttons: the master spell-check switch,
                                // then the focus-adaptive Export control. The `trailing` slot
                                // takes one widget, so they share an HStack.
                                trailing: trailing_controls
                                center: Expand::horizontal {
                                    child: center_content
                                }
                                // Hidden whenever this window is fullscreen — plain F11
                                // as well as distraction-free. See `chrome_visible`'s
                                // comment above for the exit guarantee that replaces
                                // "the window controls are always there".
                                controls_visible: controls_visible
                                close_action: |ctx| ctx.close_window()
                            }
                            ),
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
                    autosave_menu.clone(),
                    spellcheck_menu.clone(),
                    scene_focused.clone(),
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
                )));
                let inner =
                    tree.add(VStack::new().spacing(0.0).add_child(title_bar).add_child(body));

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
        let (second, _) = factory.attached_window_config(1, path).expect("second window");
        let (third, _) = factory.attached_window_config(1, path).expect("third window");

        assert_eq!(second.string_id.as_deref(), Some(attached_window_id_for(path, 2).as_str()));
        assert_eq!(third.string_id.as_deref(), Some(attached_window_id_for(path, 3).as_str()));
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

        assert_eq!(config.string_id.as_deref(), Some(window_id_for(path).as_str()));
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
    /// [`ProjectWindow::window_config`]. Add an entry to that menu, add its
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
