// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The two export entry points: a focus-derived quick scope, and the Choose… picker.
//!
//! Both flush the editors and read the anchor **here**, before presenting, so the panel's
//! preview and the committed export see current prose. The panel is modal, so no edit can
//! slip in behind it.

use bastyde::prelude::*;

use export_management::ExportScopeKind;

use crate::intents::AppIntent;
use crate::view_models::ExportViewModel;

use super::super::present_export_panel;
use super::CommandDeps;

pub(super) fn register(ctx: &mut BuildContext, deps: &CommandDeps) {
    // Ctrl+Shift+E → the Export Choose… picker (Ctrl+E is the editor's centre-align).
    ctx.register_shortcut_global(
        Shortcut::new("work.export")
            .name("Export…")
            .primary(KeyStroke::new(Key::E, Modifiers::CTRL | Modifiers::SHIFT))
            .build(),
    );

    // Export a quick scope resolved from the current focus. Fired (with the scope as payload)
    // by the title-bar Export split-button and the File ▸ Export submenu.
    {
        let editors = deps.editors.clone();
        ctx.register_action_global(Action::new("export.scope").on_invoke(move |i, c| {
            let Some(AppIntent::ExportScoped { scope }) = AppIntent::from_intent(i) else {
                return;
            };
            let scope = scope.clone();
            let Some(vm) = c.app_state::<ExportViewModel>().cloned() else {
                return;
            };
            editors.flush_all();
            let anchor = editors.active_item().get();
            vm.prepare(scope, anchor);
            present_export_panel(c, vm);
        }));
    }
    // The Choose… entry point (File ▸ Export ▸ Choose…, the split-button dropdown, and
    // Ctrl+Shift+E): open the panel straight into the checkbox tree (Custom scope),
    // independent of focus. Not Ctrl+E — that is the editor's centre-align.
    {
        let editors = deps.editors.clone();
        ctx.register_action_global(Action::new("work.export").on_invoke(move |_i, c| {
            let Some(vm) = c.app_state::<ExportViewModel>().cloned() else {
                return;
            };
            editors.flush_all();
            let anchor = editors.active_item().get();
            vm.prepare(ExportScopeKind::Custom, anchor);
            present_export_panel(c, vm);
        }));
    }
}
