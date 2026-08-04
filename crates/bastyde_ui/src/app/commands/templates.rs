// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Commands over the per-project note templates: inserting one, and capturing the note you
//! are in as a new one.
//!
//! Both are gated only on there being **an editor to act on**, which
//! [`FormatViewModel::has_target`](crate::view_models::FormatViewModel::has_target) answers. Templates were note-only at first; that
//! restriction is gone, so a scene, a synopsis box, a corkboard card and a stream row are
//! all fair game. `has_target` is also *sticky*, so the gate survives the focus loss that
//! opening the menu causes.
//!
//! **Insert puts the body at the caret**, replacing the selection if there is one. It does
//! not clear the note first and does not ask: `TextCursor::insert_fragment` already brackets
//! the whole thing in a composite, so one Ctrl+Z takes it back, and a modal on every
//! non-empty note would train the writer to dismiss it unread.

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
        ctx.register_action_global(Action::new("templates.save_as").on_invoke(move |_i, c| {
            // Read through the **handle**, not through the tab's `OpenDoc`.
            //
            // The tab route only ever sees editors the tab itself mounted, so it can reach
            // a scene or a note's main prose and nothing else. Now that any editor can be
            // captured, that would quietly do nothing on a synopsis box, a corkboard card
            // or a stream row — surfaces whose editors are registered but whose documents
            // the tab does not own. `EditorHandle::to_djot` is the same resolution the
            // insert command already writes through, so read and write agree on what "the
            // editor you are in" means.
            let Some(handle) = format.handle_for_commands() else {
                return;
            };
            let body = handle.to_djot();
            if body.trim().is_empty() {
                c.show_toast(
                    Toast::warning(tr!(save_as_template_empty_editor()))
                        .scoped_id("templates.empty", templates.work_id())
                        .target_work(templates.work_id()),
                );
                return;
            }
            present_save_as_template(&templates, format.clone(), body, c);
        }));
    }
}

/// Present the name prompt for **Save as template**, and create the template on OK.
///
/// An `InputDialog`, not a hand-built modal: this captures exactly one short string,
/// which is the case its own doc reserves it for ("forms longer than a single field
/// belong in a custom `Dialog`"). It is also what `binder.rename` uses, so the two name
/// prompts in the app look and behave the same.
///
/// The duplicate check is a **live validator**, so a colliding name greys OK out and
/// explains itself as it is typed, rather than being accepted and then apologised for.
/// That is the whole reason `InputDialog::validate` exists — it was added for this.
fn present_save_as_template(
    templates: &crate::view_models::NoteTemplatesViewModel,
    format: crate::view_models::FormatViewModel,
    body: String,
    ctx: &mut EventContext,
) {
    let for_validate = templates.clone();
    let templates = templates.clone();
    InputDialog::new(tr!(save_as_template_title()))
        .prompt(tr!(save_as_template_explain()))
        .placeholder(tr!(save_as_template_placeholder()))
        .ok_label(tr!(save_as_template_confirm()))
        .validate(move |name| {
            if name.trim().is_empty() {
                // Nothing to say: the writer has not finished typing a name, they have
                // not made a mistake. The greyed OK carries it.
                return Err(None);
            }
            match for_validate.duplicate_name(name, None) {
                Some(clash) => Err(Some(tr!(save_as_template_duplicate(name = clash)))),
                None => Ok(()),
            }
        })
        .on_result(move |result, c| {
            // Focus went to the menu overlay, then to the dialog. Put it back in the note
            // either way — including on Cancel, which is the case where the writer most
            // clearly meant to carry on where they were. Same fix, same reason, as the
            // insert command's own `refocus`.
            format.refocus(c);
            // The validator has already refused every name that cannot be used, so this
            // only ever sees a usable one — or `None`, for Cancel.
            let Some(name) = result else { return };
            match templates.save_as_template(&name, &body) {
                Ok(_) => {
                    c.show_toast(
                        Toast::info(tr!(save_as_template_saved(name = name.trim().to_string())))
                            .scoped_id("templates.saved", templates.work_id())
                            .target_work(templates.work_id()),
                    );
                }
                // Reachable only for the failures the validator cannot see — no project
                // open, or a body over the size cap.
                Err(e) => {
                    c.show_toast(
                        Toast::warning(lit!(format!("{e:#}")))
                            .scoped_id("templates.error", templates.work_id())
                            .target_work(templates.work_id()),
                    );
                }
            }
        })
        .present(ctx);
}
