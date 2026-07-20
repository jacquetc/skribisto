// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Window-construction factories for the launcher-window model.
//!
//! Skribisto is one process per project (see `main.rs`'s module docs), and a
//! **project window is only ever created once its project is already
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
//! **Ordering invariant.** The process quits when its last window closes, so
//! every transition here opens the new window *before* closing the old one —
//! see `main.rs`'s module docs and [`crate::app::close_work_and_return_to_launcher`].
//! The one deliberate exception is `app.quit` (Ctrl+Q / File ▸ Quit —
//! registered in `app.rs`, not here): it force-closes this window with
//! nothing reopened, so the last-window-closes rule *is* the exit — see
//! [`crate::app::quit_app`]/`PendingExit::Quit`.

use std::cell::RefCell;
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
use crate::export::split_button::ExportSplitButton;
use crate::intents::AppIntent;
use crate::panels::welcome::WelcomePanel;
use crate::shell::project_switcher_button::ProjectSwitcherButton;
use crate::singles::{SingleWork, SingleWorkInfo};
use crate::spellcheck::toggle_button::SpellcheckToggleButton;
use crate::view_models::{
    ALIGN_CENTER, ALIGN_LEFT, BackupSchedulerViewModel, ExportViewModel, FormatViewModel,
    OutlineViewModel, SaveAsViewModel, scope_label,
};
use export_management::ExportScopeKind;

/// Stable window-persistence string_id for the Launcher window. One process
/// only ever shows one Launcher at a time (it closes the moment a project
/// opens), so a fixed id — not per-project hashing — is correct here; see
/// [`window_id_for`] for project windows.
pub const LAUNCHER_WINDOW_ID: &str = "launcher";

/// Per-project window-persistence id: `work-{hash(canonical_path)}`.
///
/// Canonicalizing first collapses different spellings of the same path
/// (symlinks, `..`, relative vs. absolute, a trailing slash) onto one id, so
/// a project's remembered geometry keys on the file it actually is, not the
/// string a particular launch path happened to spell it as — the bug this
/// replaces: per-project ids only worked when the path came from argv,
/// because a bare launch used to fix the window's id *before* any project
/// was chosen.
///
/// Falls back to the raw string when the path doesn't exist yet (a New Work
/// target that hasn't been written to disk) — still stable and still
/// collision-free in practice, since every caller hashes the exact target
/// path it already computed.
///
/// Hashed with **blake3**, not `std::hash::DefaultHasher`: the result is
/// *persisted* (as the key of a `window_state.toml` row), and `DefaultHasher`'s
/// algorithm is explicitly not guaranteed stable across Rust releases — a
/// toolchain bump would silently orphan every saved geometry. blake3 is
/// already resolved in this workspace (a transitive dependency via
/// `skrib_format`, which documents the identical rationale in
/// `crates/skrib_format/src/fingerprint.rs`), so this adds no new crate to
/// the dependency graph. Truncated to 16 hex chars to match the previous
/// `work-{:016x}` id width (`window_state.toml`'s existing well-formed rows
/// stay visually consistent); the full 32-char digest would be equally safe,
/// this is just cosmetic.
pub fn window_id_for(project: &str) -> String {
    let canon = std::fs::canonicalize(project)
        .map(|p| p.to_string_lossy().into_owned())
        .unwrap_or_else(|_| project.to_string());
    format!("work-{}", &blake3::hash(canon.as_bytes()).to_hex()[..16])
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
        })
}

/// A Format-menu row that just runs a command.
///
/// Like the dock's buttons it requests a frame: the pointer is on the menu
/// overlay and the editor is unfocused, so nothing else schedules the repaint
/// that shows the edit.
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
        })
}

#[derive(Clone)]
pub struct ProjectWindowFactory {
    /// The formatting view-model, created detached here because the menu bar is
    /// built alongside `App` rather than inside it — the Format menu binds these
    /// signals at that moment. `App::build` attaches the editors on every build.
    format: FormatViewModel,
    app_ctx: Rc<AppContext>,
    outline: OutlineViewModel,
    export: ExportViewModel,
    single_work: SingleWork,
    single_work_info: SingleWorkInfo,
    autosave_menu: Signal<bool>,
    /// Plain mirror of the master spell-check switch — the title-bar toggle's icon and
    /// the View ▸ Check spelling checkmark read it. `App::build` keeps it in sync.
    spellcheck_menu: Signal<bool>,
    save_as_vm: SaveAsViewModel,
    backup_mode: Signal<bool>,
    backup_context: Signal<Option<crate::backup::BackupContext>>,
    unsaved: Signal<bool>,
    pending_exit: Signal<PendingExit>,
    backup_scheduler: BackupSchedulerViewModel,
    /// Shared handle to the *current* project window's `WindowState`, so IPC
    /// "raise" events (see `ipc.rs`) can focus it directly without a
    /// `WindowManager` id lookup (which misses while that window is
    /// dispatching its own events). Retargeted to the freshest project
    /// window each time one opens.
    main_window_state: Rc<RefCell<Option<WindowState>>>,
}

impl ProjectWindowFactory {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        app_ctx: Rc<AppContext>,
        outline: OutlineViewModel,
        export: ExportViewModel,
        single_work: SingleWork,
        single_work_info: SingleWorkInfo,
        autosave_menu: Signal<bool>,
        spellcheck_menu: Signal<bool>,
        save_as_vm: SaveAsViewModel,
        backup_mode: Signal<bool>,
        backup_context: Signal<Option<crate::backup::BackupContext>>,
        unsaved: Signal<bool>,
        pending_exit: Signal<PendingExit>,
        backup_scheduler: BackupSchedulerViewModel,
        main_window_state: Rc<RefCell<Option<WindowState>>>,
        format: FormatViewModel,
    ) -> Self {
        Self {
            format,
            app_ctx,
            outline,
            export,
            single_work,
            single_work_info,
            autosave_menu,
            spellcheck_menu,
            save_as_vm,
            backup_mode,
            backup_context,
            unsaved,
            pending_exit,
            backup_scheduler,
            main_window_state,
        }
    }

    /// Build the `WindowConfig` for a project window that performs `action`
    /// once its [`App`] mounts (see `App::build`'s first-build logic) — the
    /// same "seed only after the `LoadWork`/`NewWork` subscription is live"
    /// mechanism the argv launch path has always used, now shared with the
    /// Launcher's recents / new-work / examples flows. Doing the backend
    /// mutation here instead (before the window exists) would race that
    /// subscription and silently skip seeding `AppIds`/the singles/the tree.
    pub fn window_config(&self, action: PendingAction) -> WindowConfig {
        let id = window_id_for(action.target_path());

        let app_ctx_root = self.app_ctx.clone();
        let outline = self.outline.clone();
        let format = self.format.clone();
        let export = self.export.clone();
        let single_work = self.single_work.clone();
        let single_work_info = self.single_work_info.clone();
        let autosave_menu = self.autosave_menu.clone();
        let spellcheck_menu = self.spellcheck_menu.clone();
        // Per WINDOW, not per process: unlike `spellcheck_menu` (a global
        // setting, correctly shared), this tracks which surface *this* window
        // has focused. A process-wide one would let a second project window
        // grey out this window's Format menu.
        let scene_focused = Signal::new(false);
        let save_as_vm = self.save_as_vm.clone();
        let backup_mode = self.backup_mode.clone();
        let backup_context = self.backup_context.clone();
        let unsaved = self.unsaved.clone();
        let pending_exit = self.pending_exit.clone();
        let backup_scheduler = self.backup_scheduler.clone();
        let main_window_state = self.main_window_state.clone();

        WindowConfig::new()
            .id(id)
            .title("Skribisto")
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
            .on_close_requested({
                let unsaved = unsaved.clone();
                let autosave = autosave_menu.clone();
                let pending = pending_exit.clone();
                let scheduler = backup_scheduler.clone();
                let backup_mode = backup_mode.clone();
                let app_ctx_guard = app_ctx_root.clone();
                move |ctx| {
                    guard_unsaved_exit(
                        ctx,
                        &app_ctx_guard,
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
            .root(move |tree, state| {
                // Publish this window's handle so IPC "raise" events (handled
                // in `on_app_event`) can focus it directly.
                *main_window_state.borrow_mut() = Some(state.clone());
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
                        let menu_autosave = autosave_menu.clone();
        // The menu closure below is `move`, so give it its own clone — the title-bar toggle
        // still needs the original (same reason `menu_autosave` exists).
        let menu_spellcheck = spellcheck_menu.clone();
                        let menu_scene_focused = scene_focused.clone();
                        let menu_format_vm = format.clone();
                        let menu_save_as = save_as_vm.clone();
                        let menu_backup_mode = backup_mode.clone();
                        let menu_unsaved = unsaved.clone();
                        let menu = MenuModel::new().menu(tr!(menu_file()), move |m| {
                            let file_ctx = menu_ctx.clone();
                            let folder_ctx = menu_ctx.clone();
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
                                        .default_file_name(format!("{}.skrib", crate::project_stem(&ctx)))
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
                                        crate::project_stem(&ctx)
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
                        });
                        let menubar = MenuBar::from_model(menu)
                            .collapse_policy(CollapsePolicy::Always)
                            .hamburger_size(IconButtonSize::Large);

                        tree.add_boxed(Box::new(bati!(

                            TitleBar::new(host) {
                                background: SurfaceRole::Main
                                leading: menubar
                                // Left of the window buttons: the master spell-check switch,
                                // then the focus-adaptive Export control. The `trailing` slot
                                // takes one widget, so they share an HStack.
                                trailing: HStack {
                                    spacing: 5.0
                                    alignment: bastyde::tokens::VAlignment::Center
                                    SpellcheckToggleButton::new(spellcheck_menu.clone())
                                    ExportSplitButton::new(export.clone())
                                }
                                center: Expand::horizontal {
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
                                                ProjectSwitcherButton::new(app_ctx_root.clone())
                                            }
                                        }
                                        Expand::horizontal {
                                            Center {
                                                TextWidget::new(lit!("Skribisto")) {
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
                    None => tree.add(TextWidget::new(lit!("Skribisto"))),
                };

                let body = tree.add(Expand::new().child(App::new(
                    app_ctx_root.clone(),
                    outline.clone(),
                    autosave_menu.clone(),
                    spellcheck_menu.clone(),
                    scene_focused.clone(),
                    unsaved.clone(),
                    pending_exit.clone(),
                    backup_mode.clone(),
                    backup_context.clone(),
                    action,
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
            })
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
            &["menu-file", "menu-view", "menu-format", "menu-tools"],
        ),
        (
            "File",
            &[
                "menu-new-work",
                "menu-open-work",
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
        ("File > Import from", &["menu-import-plume"]),
        // The export scopes are labelled from `ExportScopeKind` at runtime and
        // deliberately carry no mnemonics; they are listed so the table stays a
        // complete picture of the menu, and the uniqueness check skips them.
        (
            "File > Export",
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
        ("Tools", &["menu-spellcheck"]),
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
