// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Keeping the live spell-check honest.
//!
//! The checker's state has three inputs that change *while the app runs* and none of which
//! it can observe for itself: the installed dictionaries, the project's personal word list,
//! and the theme (the squiggle colour). Each needs a subscription that ends in
//! `attach_all()` — re-running the check over every open document so squiggles update
//! immediately rather than at the next keystroke.
//!
//! The re-attach *policy* is the subtle part and is documented per block below; the engine
//! itself lives in [`crate::spellcheck`], and the caret-aware session wiring in
//! `tabs::shared::editor`.

use teksilo::prelude::*;

use std::rc::Rc;

use frontend::AppContext;
use frontend::common::event::{DirectAccessEntity, EntityEvent, Event, Origin};

use crate::app_ids::AppIds;
use crate::models::OpenDocsStore;
use crate::spellcheck::SpellcheckService;
use crate::view_models::DictionariesViewModel;

use super::super::spell_underline_color;

pub(in crate::app) fn install(
    ctx: &mut BuildContext,
    app_ctx: &Rc<AppContext>,
    ids: &AppIds,
    docs: &OpenDocsStore,
    spell: &SpellcheckService,
    dictionaries: &DictionariesViewModel,
) {
    // Squiggle colour from the theme's error role (re-attaches only on a real change, e.g. a
    // light/dark switch).
    docs.set_squiggle_color(spell_underline_color(ctx.theme().colors.text_error));

    // A dictionary installed or removed → drop the engine's per-id cache (so a cached miss
    // can't hide a fresh install, nor a cached `Arc` keep a removed dictionary alive), then
    // re-attach every open document (install paints new squiggles; remove degrades
    // gracefully, never rewriting `dict_language`).
    {
        let docs = docs.clone();
        let spell = spell.clone();
        ctx.effect(&dictionaries.changed_signal(), move |_| {
            spell.invalidate_dictionaries();
            docs.attach_all();
        });
    }

    // A personal-dictionary change — a word added/removed from the Settings pane or the
    // editor's "Add to dictionary", including its undo/redo — re-runs the live spell-check:
    // reload the personal words and re-attach every open document so squiggles update
    // immediately. This is the single place a `DictWord` mutation touches the checker; the
    // pane and the context menu both just create/remove the entity.
    //
    // Guarded — `DictWord` events carry no `work_id`, only the changed entity's own id, so a
    // sibling window's DictWord edit (a *different* open Work) would otherwise reload *this*
    // window's personal words from the wrong Work's `DictWord` set.
    // `mutation_ids_belong_to_work` is the same relationship-walk guard `App`'s autosave
    // `mutation_origins()` loop uses for the identical problem.
    for dict_word_event in [
        EntityEvent::Created,
        EntityEvent::Updated,
        EntityEvent::Removed,
    ] {
        let app_ctx = app_ctx.clone();
        let ids = ids.clone();
        let docs = docs.clone();
        let spell = spell.clone();
        ctx.subscribe_event(
            Origin::DirectAccess(DirectAccessEntity::DictWord(dict_word_event)),
            move |event: &Event| {
                let Origin::DirectAccess(entity) = event.origin.clone() else {
                    return;
                };
                let Some(my_work_id) = ids.work_id.get() else {
                    return; // nothing open here — cannot be my mutation
                };
                if !super::super::mutation_ids_belong_to_work(
                    &app_ctx, my_work_id, entity, &event.ids,
                ) {
                    return;
                }
                crate::view_models::reload_personal_words(&app_ctx, &spell, Some(my_work_id));
                docs.attach_all();
            },
        );
    }

    // Cross-process staleness: a peer window may have installed/removed a dictionary while
    // this one was unfocused. Re-scan on the focus-regain edge — `rescan()` bumps `changed`,
    // whose effect (above) drops the engine cache and re-attaches every document, so this
    // must NOT invalidate/attach again or each focus-regain would do all that work twice.
    {
        let dictionaries = dictionaries.clone();
        let was_active = std::cell::Cell::new(true);
        let wsig = ctx.window_active_signal();
        ctx.effect(&wsig, move |active| {
            let regained = *active && !was_active.get();
            was_active.set(*active);
            if regained {
                dictionaries.rescan();
            }
        });
    }
}
