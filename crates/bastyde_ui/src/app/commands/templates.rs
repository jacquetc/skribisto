// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Commands over the per-project note templates: inserting one, and capturing the note you
//! are in as a new one.
//!
//! Both are gated on the caret being in a **note's prose**, which
//! [`FormatViewModel::note_focused`] answers. A note's *synopsis* resolves to
//! `FormatSurface::Synopsis`, so neither command can reach one — that falls out of the
//! existing classification rather than needing a guard here.
//!
//! **Insert puts the body at the caret**, replacing the selection if there is one. It does
//! not clear the note first and does not ask: `TextCursor::insert_fragment` already brackets
//! the whole thing in a composite, so one Ctrl+Z takes it back, and a modal on every
//! non-empty note would train the writer to dismiss it unread. That is what every
//! comparable tool does too — Obsidian inserts at the cursor, Scrivener spawns a new
//! document rather than overwriting one.

use bastyde::core::modal::{ModalCloseBehavior, ModalPresentation, ModalRequest};
use bastyde::prelude::*;
use bastyde::widgets::Toast;

use crate::app_ids::HasWorkId;
use crate::intents::AppIntent;
use crate::toast_scope::ToastWorkExt;

use super::CommandDeps;

pub(super) fn register(ctx: &mut BuildContext, deps: &CommandDeps) {
    {
        let format = deps.format.clone();
        let templates = deps.session.note_templates.clone();
        ctx.register_action_global(Action::new("editor.insert_template").on_invoke(
            move |intent, c| {
                let Some(AppIntent::InsertTemplate { template_id }) =
                    AppIntent::from_intent(intent)
                else {
                    return;
                };
                let Some(row) = templates.rows().into_iter().find(|r| r.id == *template_id) else {
                    return; // deleted between the menu being built and clicked
                };
                let Some(handle) = format.handle_for_commands() else {
                    return;
                };
                handle.insert_djot(&row.body);
                // The menu overlay took focus when it opened; without this the writer is
                // left with no caret and has to click back into the prose before typing.
                // The dock never needed it — its buttons are `focusable(false)` — but the
                // menu is now the only surface these commands have.
                format.refocus(c);
                c.show_toast(
                    Toast::info(tr!(template_inserted(name = row.name.clone())))
                        .scoped_id("templates.inserted", templates.work_id())
                        .target_work(templates.work_id()),
                );
            },
        ));
    }

    {
        let format = deps.format.clone();
        let templates = deps.session.note_templates.clone();
        let editors = deps.editors.clone();
        ctx.register_action_global(Action::new("templates.save_as").on_invoke(move |_i, c| {
            // The prose comes from the *document*, not from the editor handle — `EditorHandle`
            // deliberately exposes no content reader, and the document is where the Djot
            // actually lives.
            let Some(body) = editors.focused_note_djot() else {
                return;
            };
            if body.trim().is_empty() {
                c.show_toast(
                    Toast::warning(tr!(save_as_template_empty_note()))
                        .scoped_id("templates.empty", templates.work_id())
                        .target_work(templates.work_id()),
                );
                return;
            }
            let vm = templates.clone();
            c.present_modal(
                ModalRequest::deferred(move |t| {
                    t.add(crate::note_templates::SaveAsTemplatePanel::new(vm, body))
                })
                .presentation(ModalPresentation::InTree)
                .title("Save as template")
                .close_behavior(ModalCloseBehavior::EscapeOrClickOutside)
                .size(460, 260),
            );
            let _ = &format;
        }));
    }
}
