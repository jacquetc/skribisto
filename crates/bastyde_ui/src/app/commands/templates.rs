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

use bastyde::prelude::*;
use bastyde::widgets::{InputDialog, Toast};

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
            present_save_as_template(&templates, body, String::new(), c);
            let _ = &format;
        }));
    }
}

/// Present the name prompt for **Save as template**, and create the template on OK.
///
/// An `InputDialog`, not a hand-built modal: this captures exactly one short string,
/// which is the case its own doc reserves it for ("forms longer than a single field
/// belong in a custom `Dialog`"). It is also what `binder.rename` uses, so the two
/// name prompts in the app look and behave the same.
///
/// `InputDialog` has no live validation and no disable-until-valid OK, so the duplicate
/// check happens on confirm — and on a clash the dialog is **re-presented with the name
/// still in it** rather than simply refused. Losing what you typed to a toast would be
/// the worse half of every trade here: the writer's next move is to adjust the name they
/// just chose, not to type it again from nothing.
fn present_save_as_template(
    templates: &crate::view_models::NoteTemplatesViewModel,
    body: String,
    seed: String,
    ctx: &mut EventContext,
) {
    let templates = templates.clone();
    InputDialog::new(tr!(save_as_template_title()))
        .prompt(tr!(save_as_template_explain()))
        .placeholder(tr!(save_as_template_placeholder()))
        .default_text(seed)
        .ok_label(tr!(save_as_template_confirm()))
        .on_result(move |result, c| {
            // Cancel, or an empty name: nothing to do, and nothing worth saying — the
            // same silence `begin_rename` gives.
            let Some(name) = result else { return };
            if name.trim().is_empty() {
                return;
            }
            // The duplicate case is checked here rather than left to the error string
            // `save_as_template` returns: that string is a bare `bail!`, so it would reach
            // the writer untranslated. Every other failure is genuinely exceptional and
            // falls through to the generic message.
            if let Some(clash) = templates.duplicate_name(&name, None) {
                c.show_toast(
                    Toast::warning(tr!(save_as_template_duplicate(name = clash)))
                        .scoped_id("templates.error", templates.work_id())
                        .target_work(templates.work_id()),
                );
                present_save_as_template(&templates, body.clone(), name, c);
                return;
            }
            match templates.save_as_template(&name, &body) {
                Ok(_) => {
                    c.show_toast(
                        Toast::info(tr!(save_as_template_saved(name = name.trim().to_string())))
                            .scoped_id("templates.saved", templates.work_id())
                            .target_work(templates.work_id()),
                    );
                }
                Err(e) => {
                    c.show_toast(
                        Toast::warning(lit!(format!("{e:#}")))
                            .scoped_id("templates.error", templates.work_id())
                            .target_work(templates.work_id()),
                    );
                    // Straight back to the prompt, carrying what was typed.
                    present_save_as_template(&templates, body.clone(), name, c);
                }
            }
        })
        .present(ctx);
}
