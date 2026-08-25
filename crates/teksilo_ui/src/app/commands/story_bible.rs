// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! "Add as note": the editor context-menu row that opens the story-bible
//! creation modal, pre-filled from the current selection. The menu mounts at
//! the arena root, so it can only reach a **global** action; this is that
//! action, and the only thing it does is resolve the pre-fill and hand off to
//! [`crate::story_bible::modal::present_create`], which is the one place that
//! actually creates anything (and only once Create is pressed there).

use teksilo::prelude::*;

use crate::intents::AppIntent;
use crate::story_bible;

use super::CommandDeps;

pub(super) fn register(ctx: &mut BuildContext, deps: &CommandDeps) {
    let modal_deps = story_bible::modal::ModalDeps {
        app_ctx: deps.app_ctx.clone(),
        ids: deps.ids.clone(),
        tags: deps.session.tags.clone(),
        templates: deps.session.note_templates.clone(),
        editors: deps.editors.clone(),
    };
    ctx.register_action_global(
        Action::new("story_bible.add_as_note").on_invoke(move |i, c| {
            if let Some(AppIntent::AddAsNote {
                item_id,
                selected_text,
            }) = AppIntent::from_intent(i)
                && let Some(prefill) = story_bible::modal::prefill_from_selection(
                    &modal_deps.app_ctx,
                    &modal_deps.ids,
                    *item_id,
                    selected_text,
                )
            {
                story_bible::modal::present_create(modal_deps.clone(), prefill, c);
            }
        }),
    );
}
