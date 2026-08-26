// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! "Add as note": the editor context-menu submenu that files the current selection
//! into the story bible under one chosen tag. The menu mounts at the arena root, so
//! it can only reach a **global** action; this is that action.
//!
//! The submenu is built in [`crate::tabs::shared::editor`] and ordered by
//! [`crate::story_bible::capture`]; everything that happens after the writer picks a
//! tag is [`crate::story_bible::capture_flow`]. Nothing is created here.

use teksilo::prelude::*;

use crate::intents::AppIntent;
use crate::story_bible::capture_flow::{self, CaptureDeps};

use super::CommandDeps;

pub(super) fn register(ctx: &mut BuildContext, deps: &CommandDeps) {
    let capture_deps = CaptureDeps {
        app_ctx: deps.app_ctx.clone(),
        ids: deps.ids.clone(),
        tags: deps.session.tags.clone(),
        templates: deps.session.note_templates.clone(),
        editors: deps.editors.clone(),
        capture: ctx
            .app_state::<crate::models::NoteCaptureService>()
            .cloned(),
    };
    ctx.register_action_global(
        Action::new("story_bible.add_as_note").on_invoke(move |i, c| {
            if let Some(AppIntent::AddAsNote {
                item_id,
                selected_text,
                tag_id,
            }) = AppIntent::from_intent(i)
            {
                capture_flow::capture(&capture_deps, *item_id, selected_text, *tag_id, c);
            }
        }),
    );
}
