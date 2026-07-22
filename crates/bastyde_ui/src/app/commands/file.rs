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
use bastyde::widgets::{
    EventContextMessageBoxExt, MessageBox, MessageBoxButton, MessageBoxButtons, StandardButton,
};

use crate::panels::import_plume::ImportPlumePanel;
use crate::intents::AppIntent;
use crate::settings::SettingsPanel;
use crate::view_models::{ImportPlumeViewModel, PendingSwitch};

use super::super::{PendingExit, guard_unsaved_exit, open_work_flow};
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
        ctx.register_action_global(Action::new("work.new").on_invoke(move |_i, c| {
            switch.request(c, PendingSwitch::NewWork, ids.work_id.get())
        }));
    }

    // Open Work (Ctrl+O): native picker for an existing `.skrib`, then the guard, then load.
    // The guard runs *after* the pick, so cancelling the picker — or choosing a backup file,
    // which opens in its own process and leaves this project untouched — never prompts about
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
        ctx.register_action_global(Action::new("work.open").on_invoke(move |_i, c| {
            open_work_flow(switch.clone(), ids.clone(), c)
        }));
    }
    // Open an already-chosen path (payload in the intent) — the switcher popover's "Open
    // here" and the import toast's "Open now", both of which live outside `App` and pick the
    // path themselves. Same guard, no picker.
    {
        let switch = deps.project_switch.clone();
        let ids = deps.ids.clone();
        ctx.register_action_global(Action::new("work.open_path").on_invoke(move |i, c| {
            if let Some(AppIntent::OpenWorkPath { path }) = AppIntent::from_intent(i) {
                switch.request(c, PendingSwitch::OpenWork(path.clone()), ids.work_id.get());
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

    // Quit (Ctrl+Q): really terminates the process (see `PendingExit::Quit`'s docs), after
    // the same unsaved-changes guard as every other exit path — `guard_unsaved_exit`, shared
    // with `work.close`'s action and the project window's own `on_close_requested` guard
    // (`windows.rs`). Deliberately does NOT go through `close_window()`: that would only ever
    // land back on the project window's close guard, which always returns to the Launcher —
    // never terminates. The title-bar X / Alt+F4 path is unchanged (still `close_window()`,
    // still returns to the Launcher).
    //
    // With M Works open, Quit first accounts for every OTHER open Work's dirty state
    // (`other_dirty_work_titles`) — see that function's own doc for why it REFUSES
    // (naming them) rather than attempting a cross-window save-then-close-all. THIS
    // window's own Work always still goes through the unchanged single-Work guard below.
    ctx.register_shortcut_global(
        Shortcut::new("app.quit")
            .name("Quit")
            .primary(KeyStroke::ctrl(Key::Q))
            .build(),
    );
    {
        let app_ctx = deps.app_ctx.clone();
        let ids = deps.ids.clone();
        let unsaved = deps.unsaved.clone();
        let autosave = deps.autosave.clone();
        let pending = deps.pending_exit.clone();
        let scheduler = deps.backup_scheduler.clone();
        let backup_mode = deps.backup_mode.clone();
        let registry = deps.registry.clone();
        ctx.register_action_global(Action::new("app.quit").on_invoke(move |_i, ctx| {
            let others = super::super::other_dirty_work_titles(&registry, ids.work_id.get());
            if !others.is_empty() {
                ctx.present_message_box(
                    MessageBox::warning(tr!(quit_other_works_dirty_title()))
                        .text(tr!(quit_other_works_dirty_text(list = others.join(", "))))
                        .buttons(MessageBoxButtons::Custom(vec![MessageBoxButton::standard(
                            StandardButton::Ok,
                        )]))
                        .default_button(StandardButton::Ok)
                        .escape_button(StandardButton::Ok),
                );
                return;
            }
            guard_unsaved_exit(
                ctx,
                &app_ctx,
                &ids,
                unsaved.get(),
                backup_mode.get(),
                autosave.get(),
                &pending,
                &scheduler,
                PendingExit::Quit,
            );
        }));
    }

    // "Welcome" now means "close this work and go back to the Launcher" (the launcher-window
    // model — Welcome is a real window, not a modal any more). Fired from File ▸ Welcome… and
    // the brand icon button. A pure alias for `work.close`, dispatched by name, so it shares
    // that action's exact guard (unsaved-changes prompt, the on-close backup, backup-mode
    // handling) rather than bypassing it — do NOT inline a second copy of that logic here.
    // Global so the title-bar overlay menu/button reach it (house rule).
    ctx.register_action_global(
        Action::new("welcome.show").on_invoke(|_i, c| c.send_intent(Intent::new("work.close"))),
    );
}
