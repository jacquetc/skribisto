// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The punctuation house style, pushed from the Work's row (and the app-level
//! fallback) into every open editor.
//!
//! Two hops, because the row is reached through the Work: re-point the handle
//! whenever the open project changes, then push the resolved flags whenever any
//! of the app- or project-tier signals moves. Also carries
//! [`punctuation_flags`], the tier-merge the push step calls — kept beside its
//! one call site rather than at the top of `app.rs`.

use teksilo::prelude::*;

use crate::app_ids::AppIds;
use crate::models::OpenDocsStore;
use crate::singles::{SingleSmartPunctuation, SingleWork};
use crate::text_replacement::typography::SmartPunctuationFlags;
use crate::view_models::SettingsViewModel;

/// The punctuation rules in force for the open project — the two tiers resolved
/// into the one flag set the editor sessions run.
///
/// `override_app_default` off means "follow the application preference", which
/// is now a real thing to follow rather than a synonym for off. On means this
/// project departs from it, and the row's own five values win outright — not
/// merged with the app tier, because a house style is a whole system (Italian
/// picks one of three; French adds a space its neighbours do not) and a
/// half-inherited one belongs to no language at all.
pub(in crate::app) fn punctuation_flags(
    sp: &SingleSmartPunctuation,
    app: &SettingsViewModel,
) -> SmartPunctuationFlags {
    if sp.override_app_default().get() {
        return SmartPunctuationFlags {
            dashes: sp.dashes().get(),
            ellipsis: sp.ellipsis().get(),
            quotes: sp.quotes().get(),
            quote_style: sp.quote_style().get(),
            pre_punctuation_spacing: sp.pre_punctuation_spacing().get(),
            dialogue_marker: sp.dialogue_marker().get(),
        };
    }
    SmartPunctuationFlags {
        dashes: app.punct_dashes().get(),
        ellipsis: app.punct_ellipsis().get(),
        quotes: app.punct_quotes().get(),
        quote_style: app.punct_quote_style().get(),
        pre_punctuation_spacing: app.punct_spacing().get(),
        dialogue_marker: app.punct_dialogue().get(),
    }
}

pub(in crate::app) fn install(
    ctx: &mut BuildContext,
    ids: &AppIds,
    single_work: &SingleWork,
    smart_punctuation: &SingleSmartPunctuation,
    spell_docs: &OpenDocsStore,
    settings: &SettingsViewModel,
) {
    // ── The punctuation house style, from the Work's row to every editor ──
    //
    // Two hops, because the row is reached through the Work: re-point the
    // handle whenever the open project changes, then push the flags whenever
    // any of them does. The second hop is what makes a switch flipped in
    // Settings reach the scene the writer is looking at without reopening it.
    {
        let sp = smart_punctuation.clone();
        ctx.effect(&single_work.smart_punctuation(), move |id| {
            // 0 is "no project open", never "this project has no row" — the
            // row is minted before its Work, so a live project always has one.
            sp.set_id((*id != 0).then_some(*id));
        });
    }
    // The project's default language, from the Work to the open documents.
    //
    // `OpenDocsStore` caches it (every item that has no tag of its own
    // inherits it, and re-resolving per keystroke would be wasteful), and
    // until this effect existed only `ProjectLifecycleViewModel` ever wrote
    // that cache — on load. So editing Settings ▸ Work ▸ Language updated
    // the entity, re-attached every document, and re-resolved them all
    // against the *stale* cached language: the change did not reach the
    // editor until the project was reopened. That affected spell-check as
    // much as punctuation.
    {
        let docs = spell_docs.clone();
        let ids = ids.clone();
        ctx.effect(&single_work.dict_language(), move |langs| {
            docs.set_project_language(ids.work_id.get(), langs.clone());
            docs.attach_all();
        });
    }

    {
        let docs = spell_docs.clone();
        let sp = smart_punctuation.clone();
        let app = settings.clone();
        let push = move || {
            docs.set_punctuation(Some(punctuation_flags(&sp, &app)));
        };
        // One effect per flag: `ctx.effect` takes a single signal, and these
        // are independent switches rather than one compound value.
        //
        // Both tiers are observed. The app-level ones matter even while a
        // project holds the override: it can be dropped at any moment, and
        // the flags it falls back to have to be current when it is.
        //
        // This list MUST name every signal `punctuation_flags` reads, on
        // both tiers — a flag observed here but not read there is harmless,
        // but one read there and not observed here silently fails to
        // propagate (that is exactly how `dialogue_marker` was inert until a
        // second setting changed). The bool signals of each tier:
        for signal in [
            smart_punctuation.override_app_default(),
            smart_punctuation.dashes(),
            smart_punctuation.ellipsis(),
            smart_punctuation.quotes(),
            smart_punctuation.pre_punctuation_spacing(),
            smart_punctuation.dialogue_marker(),
            settings.punct_dashes(),
            settings.punct_ellipsis(),
            settings.punct_quotes(),
            settings.punct_spacing(),
            settings.punct_dialogue(),
        ] {
            let push = push.clone();
            ctx.effect(&signal, move |_| push());
        }
        // …and the quote-style enum on each tier, which is a different type.
        for signal in [
            smart_punctuation.quote_style(),
            settings.punct_quote_style(),
        ] {
            let push = push.clone();
            ctx.effect(&signal, move |_| push());
        }
    }
}
