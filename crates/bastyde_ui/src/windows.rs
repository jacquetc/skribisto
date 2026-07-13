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

use std::cell::RefCell;
use std::rc::Rc;

use bastyde::prelude::*;
use bastyde::res;
use bastyde::widgets::primitives::icon_widget::IconMode;
use bastyde::widgets::{
    Center, CollapsePolicy, EventContextMessageBoxExt, Expand, HStack, IconButton, IconButtonSize,
    IconWidget, MenuBar, MenuEntry, MenuModel, MessageBox, MessageBoxButton, MessageBoxButtons,
    StandardButton, TextWidget, TitleBar, VStack, WindowFrame,
};

use frontend::AppContext;
use frontend::common::entities::WorkShape;

use crate::app::{App, PendingAction, PendingExit};
use crate::project_switcher_button::ProjectSwitcherButton;
use crate::singles::{SingleWork, SingleWorkInfo};
use crate::view_models::{BackupSchedulerViewModel, OutlineViewModel, SaveAsViewModel};
use crate::welcome_panel::WelcomePanel;

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
pub fn window_id_for(project: &str) -> String {
    use std::hash::{Hash, Hasher};
    let canon = std::fs::canonicalize(project)
        .map(|p| p.to_string_lossy().into_owned())
        .unwrap_or_else(|_| project.to_string());
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    canon.hash(&mut hasher);
    format!("work-{:016x}", hasher.finish())
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
    // Roughly `WelcomePanel`'s fixed 780×548 card plus breathing room; the
    // card is `Center`-ed so a larger window just adds margin instead of
    // stretching or clipping it.
    const W: u32 = 820;
    const H: u32 = 590;
    WindowConfig::new()
        .id(LAUNCHER_WINDOW_ID)
        .title("Skribisto")
        .size(W, H)
        .min_size(W, H)
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
                    let brand_icon = tree.add(
                        IconWidget::from_raster(res!("../../resources/icons/skribisto.png"), 25.0)
                            .mode(IconMode::FullColor),
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
            let body = tree
                .add(Expand::new().child(Center::new().child(WelcomePanel::new(app_ctx.clone()))));
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
#[derive(Clone)]
pub struct ProjectWindowFactory {
    app_ctx: Rc<AppContext>,
    outline: OutlineViewModel,
    single_work: SingleWork,
    single_work_info: SingleWorkInfo,
    autosave_menu: Signal<bool>,
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
        single_work: SingleWork,
        single_work_info: SingleWorkInfo,
        autosave_menu: Signal<bool>,
        save_as_vm: SaveAsViewModel,
        backup_mode: Signal<bool>,
        backup_context: Signal<Option<crate::backup::BackupContext>>,
        unsaved: Signal<bool>,
        pending_exit: Signal<PendingExit>,
        backup_scheduler: BackupSchedulerViewModel,
        main_window_state: Rc<RefCell<Option<WindowState>>>,
    ) -> Self {
        Self {
            app_ctx,
            outline,
            single_work,
            single_work_info,
            autosave_menu,
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
        let single_work = self.single_work.clone();
        let single_work_info = self.single_work_info.clone();
        let autosave_menu = self.autosave_menu.clone();
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
            .decorations(DecorationsMode::CustomChrome)
            // Consume an xdg-activation startup token (set by the desktop, or
            // by another instance's "open in new window") so this window comes
            // up focused on Wayland.
            .activate_from_env(true)
            // Unsaved-changes guard for every interactive close (title-bar X,
            // Alt+F4, and Ctrl+Q — all route through `close_window()`). In the
            // launcher-window model, closing a project window never quits the
            // process by itself: it always opens a fresh Launcher window
            // first, then force-closes this one (`close_work_and_return_to_
            // launcher`) — the process only actually exits when the Launcher
            // itself is closed. Autosave on: just ensure the save runs, then
            // return to the Launcher (no prompt). Autosave off + unsaved:
            // Save / Discard / Cancel. The save is async, so we veto now and
            // `App` re-issues the close on SaveWork.
            .on_close_requested({
                let unsaved = unsaved.clone();
                let autosave = autosave_menu.clone();
                let pending = pending_exit.clone();
                let scheduler = backup_scheduler.clone();
                let backup_mode = backup_mode.clone();
                let app_ctx_guard = app_ctx_root.clone();
                move |ctx| {
                    // Backup window: Save is off (the file is read-only), so the
                    // normal save-then-close path doesn't apply. Clean → return
                    // to the Launcher; dirty → offer to discard (Save As keeps
                    // edits — via the banner). No on-close backup (never back
                    // up a backup).
                    if backup_mode.get() {
                        if !unsaved.get() {
                            crate::app::close_work_and_return_to_launcher(&app_ctx_guard, ctx);
                            return CloseResponse::Veto;
                        }
                        let app_ctx_inner = app_ctx_guard.clone();
                        ctx.present_message_box(
                            MessageBox::question(tr!(close_backup_discard_title()))
                                .text(tr!(close_backup_discard_text()))
                                .buttons(MessageBoxButtons::Custom(vec![
                                    MessageBoxButton::standard(StandardButton::Discard),
                                    MessageBoxButton::standard(StandardButton::Cancel),
                                ]))
                                .default_button(StandardButton::Cancel)
                                .escape_button(StandardButton::Cancel)
                                .on_result(move |r, ctx| {
                                    if r.button == StandardButton::Discard {
                                        crate::app::close_work_and_return_to_launcher(
                                            &app_ctx_inner,
                                            ctx,
                                        );
                                    }
                                }),
                        );
                        return CloseResponse::Veto;
                    }
                    if !unsaved.get() {
                        // Clean project: still take an on-close backup (if the
                        // policy asks) before actually leaving — `on_close_flow`
                        // performs the Launcher-return once the backup finishes
                        // (or immediately if there's nothing to do).
                        if scheduler.wants_on_close() {
                            scheduler.on_close_flow(ctx, PendingExit::ReturnToLauncher);
                            return CloseResponse::Veto;
                        }
                        crate::app::close_work_and_return_to_launcher(&app_ctx_guard, ctx);
                        return CloseResponse::Veto;
                    }
                    if autosave.get() {
                        // Save first; the SaveWork-completion handler then runs
                        // the on-close backup and returns to the Launcher.
                        pending.set(PendingExit::ReturnToLauncher);
                        return CloseResponse::Veto;
                    }
                    let pe = pending.clone();
                    let app_ctx_discard = app_ctx_guard.clone();
                    ctx.present_message_box(
                        MessageBox::question(tr!(close_question()))
                            .text(tr!(unsaved_changes()))
                            .buttons(MessageBoxButtons::SaveDiscardCancel)
                            .default_button(StandardButton::Save)
                            .escape_button(StandardButton::Cancel)
                            .on_result(move |r, ctx| match r.button {
                                StandardButton::Save => pe.set(PendingExit::ReturnToLauncher),
                                // Discarding unsaved edits skips the backup (the
                                // last-saved state is what's kept).
                                StandardButton::Discard => {
                                    crate::app::close_work_and_return_to_launcher(
                                        &app_ctx_discard,
                                        ctx,
                                    );
                                }
                                _ => {}
                            }),
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
                        let menu_work = single_work.clone();
                        let menu_work_info = single_work_info.clone();
                        let menu_autosave = autosave_menu.clone();
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
                            // Reflect-only checkmark: mirrors the dock's truth
                            // (`is_visible`) without writing it; the toggle is
                            // driven by the `outline.toggle` intent (F9).
                            let outline = outline.clone();
                            move |m| {
                                m.item(
                                    MenuEntry::new(tr!(menu_outline()))
                                        .checked(outline.is_visible())
                                        .intent("outline.toggle")
                                        .shortcut("outline.toggle"),
                                )
                            }
                        });
                        let menubar = MenuBar::from_model(menu)
                            .collapse_policy(CollapsePolicy::Always)
                            .hamburger_size(IconButtonSize::Large);

                        tree.add_boxed(Box::new(bati!(

                            TitleBar::new(host) {
                                background: SurfaceRole::Main
                                leading: menubar
                                center: Expand::horizontal {
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
}
