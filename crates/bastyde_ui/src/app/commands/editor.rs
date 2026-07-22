// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Commands over the editor surface: the master spell-check switch, opening an item into a
//! pane, adding a word to the personal dictionary, and saving to disk.

use bastyde::prelude::*;

use crate::intents::AppIntent;
use crate::view_models::SettingsViewModel;

use super::super::{can_save, offer_missing_dictionaries};
use super::CommandDeps;

pub(super) fn register(ctx: &mut BuildContext, deps: &CommandDeps) {
    // F7 — the spell-check key every office suite has used for thirty years. A bare function
    // key for the same reason F9 is: a Global shortcut resolves before the focused widget sees
    // the raw key, so any Ctrl+letter here would shadow one of `RichTextEditor`'s built-in
    // commands.
    ctx.register_shortcut_global(
        Shortcut::new("spellcheck.toggle")
            .name("Check Spelling")
            .primary(KeyStroke::new(Key::F7, Modifiers::NONE))
            .build(),
    );
    // The master spell-check switch. Flips the *setting* and nothing else — the effect in
    // `App::build` owns the engine, so there is exactly one path from the key to
    // `set_enabled` no matter which surface fired. The toast lives here rather than in that
    // effect because only an action gets an `EventContext`.
    {
        let enabled = SettingsViewModel::new(ctx.settings()).spellcheck_enabled();
        let docs = deps.spell_docs.clone();
        let dicts = deps.dictionaries.clone();
        let session = deps.session.clone();
        ctx.register_action_global(Action::new("spellcheck.toggle").on_invoke(
            move |_i, c: &mut EventContext| {
                let now_on = !enabled.get();
                enabled.set(now_on);
                // Turning it back on with no dictionary installed reproduces the exact
                // symptom this switch exists to end: a silent absence of squiggles.
                // `offer_missing_dictionaries` otherwise only ever fires on Load/New.
                if now_on {
                    offer_missing_dictionaries(&docs, &dicts, &session, c);
                }
            },
        ));
    }

    {
        let editors = deps.editors.clone();
        ctx.register_action_global(Action::new("editor.open_item").on_invoke(move |i, _c| {
            if let Some(AppIntent::OpenItem { item_id, title }) = AppIntent::from_intent(i) {
                editors.open_or_focus(*item_id, title);
            }
        }));
    }
    {
        let editors = deps.editors.clone();
        ctx.register_action_global(Action::new("editor.open_item_to_side").on_invoke(
            move |i, _c| {
                if let Some(AppIntent::OpenItemToSide { item_id, title }) = AppIntent::from_intent(i)
                {
                    editors.open_to_side(*item_id, title);
                }
            },
        ));
    }

    // Add the resolved selection/caret word(s) to the personal dictionary, fired from the
    // editor's "Add to dictionary" context-menu item. The menu mounts at the arena root, so
    // only a **global** action reaches it. The resulting `DictWord(Created)` event drives the
    // live squiggle refresh (the subscription in `App::build`); here we just create the
    // entities and toast.
    {
        let vm = deps.user_dictionary.clone();
        ctx.register_action_global(Action::new("editor.add_to_dictionary").on_invoke(
            move |i, c| {
                let Some(AppIntent::AddWordsToDictionary { words }) = AppIntent::from_intent(i)
                else {
                    return;
                };
                let sample = words.first().cloned();
                let ids = vm.add_words(words);
                vm.added_toast(c, ids, sample);
            },
        ));
    }

    // Ctrl+S: flush every editor to the store, then save the project to disk. Gated on
    // `can_save` (dirty && !backup mode) at *both* ends: the shortcut stops matching the
    // keystroke, and the action stops matching the intent — so nothing to save means the menu
    // item greys out (same signal, in `main`), Ctrl+S is inert, and a scripted `editor.save`
    // intent is a no-op instead of a pointless disk write. The exit guards call
    // `save_to_disk()` directly, not through the intent, so save-then-close still works.
    let can_save = can_save(&deps.unsaved, &deps.backup_mode);
    ctx.register_shortcut_global(
        Shortcut::new("editor.save")
            .name("Save")
            .primary(KeyStroke::ctrl(Key::S))
            .enabled_when(can_save.clone())
            .build(),
    );
    {
        let editors = deps.editors.clone();
        ctx.register_action_global(
            Action::new("editor.save")
                .enabled_when(can_save)
                .on_invoke(move |_i, _c| editors.save_to_disk()),
        );
    }
}
