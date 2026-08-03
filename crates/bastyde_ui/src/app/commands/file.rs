// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Project and application lifecycle: New / Open / Open-a-given-path / Import / Close /
//! Settings / Quit / Welcome.
//!
//! **Every door that replaces this window's project goes through the guard.** New Work and
//! Open Work both close the open `Work` first, so both ask [`ProjectSwitchViewModel`] rather
//! than calling the backend — which is what they used to do, destroying unsaved edits
//! outright. The guard performs the switch itself, now or once the deferred save lands;
//! these actions only *ask* for it.

use bastyde::core::modal::{ModalCloseBehavior, ModalPresentation, ModalRequest};
use bastyde::prelude::*;

use crate::intents::AppIntent;
use crate::panels::import_plume::ImportPlumePanel;
use crate::panels::new_work::NewWorkPanel;
use crate::settings::SettingsPanel;
use crate::view_models::{ImportPlumeViewModel, PendingSwitch};

use super::super::open_work_flow;
use super::CommandDeps;

pub(super) fn register(ctx: &mut BuildContext, deps: &CommandDeps) {
    ctx.register_shortcut_global(
        Shortcut::new("work.new")
            .name("New Work")
            .primary(KeyStroke::ctrl(Key::N))
            .build(),
    );
    {
        let switch = deps.project_switch.clone();
        let ids = deps.ids.clone();
        let registry = deps.registry.clone();
        let app_ctx = deps.app_ctx.clone();
        let role = deps.role;
        ctx.register_action_global(Action::new("work.new").on_invoke(move |_i, c| {
            // A window that may not switch in place (attached, or a live
            // sibling) presents the form in "create beside this window" mode.
            if !crate::app::may_switch_project_in_place(&registry, &ids, role) {
                let Some(factory) = c
                    .app_state::<crate::shell::windows::ProjectWindowFactory>()
                    .cloned()
                else {
                    return;
                };
                let app_ctx = app_ctx.clone();
                c.present_modal(
                    ModalRequest::deferred(move |t| {
                        t.add(NewWorkPanel::new_beside_current(app_ctx, factory))
                    })
                    .presentation(ModalPresentation::InTree)
                    .title("New Work")
                    .close_behavior(ModalCloseBehavior::EscapeOrClickOutside)
                    .size(600, 680),
                );
                return;
            }
            switch.request(c, PendingSwitch::NewWork, ids.work_id.get())
        }));
    }

    // Open Work (Ctrl+O): native picker for an existing `.skrib`, then the guard, then load.
    // The guard runs *after* the pick, so cancelling the picker — or choosing a backup file,
    // which opens in its own window and leaves this project untouched — never prompts about
    // unsaved changes.
    ctx.register_shortcut_global(
        Shortcut::new("work.open")
            .name("Open Work")
            .primary(KeyStroke::ctrl(Key::O))
            .build(),
    );
    {
        let switch = deps.project_switch.clone();
        let ids = deps.ids.clone();
        let registry = deps.registry.clone();
        let role = deps.role;
        ctx.register_action_global(Action::new("work.open").on_invoke(move |_i, c| {
            open_work_flow(switch.clone(), ids.clone(), registry.clone(), role, c)
        }));
    }
    // Open an already-chosen path (payload in the intent) — the switcher popover's "Open
    // here" and the import toast's "Open now", both of which live outside `App` and pick the
    // path themselves. Same guard, no picker.
    {
        let switch = deps.project_switch.clone();
        let ids = deps.ids.clone();
        let registry = deps.registry.clone();
        let role = deps.role;
        ctx.register_action_global(Action::new("work.open_path").on_invoke(move |i, c| {
            if let Some(AppIntent::OpenWorkPath { path }) = AppIntent::from_intent(i) {
                if !crate::app::may_switch_project_in_place(&registry, &ids, role) {
                    crate::shell::windows::open_or_focus_project(c, &path);
                    return;
                }
                switch.request(c, PendingSwitch::OpenWork(path.clone()), ids.work_id.get());
            }
        }));
    }

    // Work ▸ New Window (Ctrl+Shift+N): a SECOND window onto the Work this
    // window is already showing.
    //
    // The one project command that mutates nothing. Both windows share the live
    // `WorkSession` — one store view, one undo stack, one set of open documents,
    // one dirty state — so a scene typed in either is the same edit, and only
    // the last of them to close ends the project. What each window keeps to
    // itself is its desk: its own tabs, split editor, docks and outline. That is
    // the point of the command (two chapters side by side, or the binder pinned
    // to a second monitor), and it is why this is not "open the file again":
    // loading the same `.skrib` twice would give two independent, silently
    // diverging copies racing each other onto one path.
    //
    // Resolution and refcounting happen in
    // `ProjectWindowFactory::attached_window_config`, which needs both this
    // window's `work_id` and its project path — the latter for the new window's
    // persistence id. A `None` from it means the Work stopped being open between
    // the menu opening and the click (the menu item is hidden without a project,
    // so this is a race, not a normal path); doing nothing is the right answer,
    // since the thing the user asked for a second view of no longer exists.
    ctx.register_shortcut_global(
        Shortcut::new("window.new")
            .name("New Window")
            .primary(KeyStroke::new(Key::N, Modifiers::CTRL | Modifiers::SHIFT))
            .build(),
    );
    {
        let app_ctx = deps.app_ctx.clone();
        let ids = deps.ids.clone();
        ctx.register_action_global(Action::new("window.new").on_invoke(move |_i, c| {
            let Some(work_id) = ids.work_id.get() else {
                return;
            };
            // The Work's own path, resolved through this window's `work_info_id`
            // — never `get_all_work_info`'s first entry, which with several Works
            // open answers about somebody else's project (see
            // `current_project_path`'s doc).
            let Some(path) = crate::current_project_path(&app_ctx, &ids) else {
                return;
            };
            let Some(factory) = c
                .app_state::<crate::shell::windows::ProjectWindowFactory>()
                .cloned()
            else {
                return;
            };
            if let Some((config, _state)) = factory.attached_window_config(work_id, &path) {
                c.open_window(config);
            }
        }));
    }

    // Import from Plume Creator: present the Import Plume modal (menu-only, no shortcut).
    // Global so the title-bar overlay menu reaches it — like work.new. The panel is built
    // over the shared, app-state `ImportPlumeViewModel` (the same instance the
    // long-operation events are routed to), reset first so a previous session's paths don't
    // linger.
    ctx.register_action_global(Action::new("work.import_plume").on_invoke(move |_i, c| {
        let Some(vm) = c.app_state::<ImportPlumeViewModel>().cloned() else {
            return;
        };
        vm.reset_form();
        c.present_modal(
            ModalRequest::deferred(move |t| t.add(ImportPlumePanel::new(vm)))
                .presentation(ModalPresentation::InTree)
                .title("Import Plume Creator project")
                .close_behavior(ModalCloseBehavior::EscapeOrClickOutside)
                .size(600, 500),
        );
    }));

    // Close Work (Ctrl+W): the `work.close` *action* is registered in `App::build` (it shares
    // the unsaved-changes guard with the window close); here we only add its global shortcut.
    ctx.register_shortcut_global(
        Shortcut::new("work.close")
            .name("Close Work")
            .primary(KeyStroke::ctrl(Key::W))
            .build(),
    );

    // Settings (Ctrl+,): present the settings modal.
    ctx.register_shortcut_global(
        Shortcut::new("app.settings")
            .name("Settings")
            .primary(KeyStroke::ctrl(Key::Character(',')))
            .build(),
    );
    {
        let session = deps.session.clone();
        ctx.register_action_global(Action::new("app.settings").on_invoke(move |_i, c| {
            let session = session.clone();
            c.present_modal(
                ModalRequest::deferred(move |t| t.add(SettingsPanel::new(session)))
                    .presentation(ModalPresentation::InTree)
                    .title("Settings")
                    .size(920, 620)
                    // Not easily dismissable — like a critical MessageBox. Only the panel's own
                    // close button / Cancel / OK close it (each calls `ctx.dismiss_modal()`);
                    // Escape and outside clicks do not, so a stray click never discards a
                    // settings session.
                    .close_behavior(ModalCloseBehavior::Manual),
            );
        }));
    }

    // Quit (Ctrl+Q): really terminates the process, accounting for **every** open Work
    // first — see `QuitSequencer`'s module doc. Single-instance means several project
    // windows in one process is the ordinary shape, so Quit must guard all of them, not
    // just the invoking window's.
    //
    // Deliberately still NOT `close_window()`: that lands on the project window's close
    // guard, which always returns to the Launcher rather than terminating. The title-bar
    // X / Alt+F4 path is unchanged (still `close_window()`, still returns to the Launcher) —
    // closing *a window* and quitting *the app* are different requests.
    ctx.register_shortcut_global(
        Shortcut::new("app.quit")
            .name("Quit")
            .primary(KeyStroke::ctrl(Key::Q))
            .build(),
    );
    {
        let quit = deps.quit.clone();
        ctx.register_action_global(Action::new("app.quit").on_invoke(move |_i, ctx| {
            quit.begin(ctx);
        }));
    }

    // "Welcome" now means "close this work and go back to the Launcher" (the launcher-window
    // model — Welcome is a real window, not a modal any more). Fired from File ▸ Welcome….
    // A pure alias for `work.close`, dispatched by name, so it shares
    // that action's exact guard (unsaved-changes prompt, the on-close backup, backup-mode
    // handling) rather than bypassing it — do NOT inline a second copy of that logic here.
    // Global so the title-bar overlay menu reaches it (house rule).
    ctx.register_action_global(
        Action::new("welcome.show").on_invoke(|_i, c| c.send_intent(Intent::new("work.close"))),
    );
}
