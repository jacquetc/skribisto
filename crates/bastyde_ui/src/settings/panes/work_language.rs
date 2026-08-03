// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Work: `<name>` ▸ Language — the open project's default spell-check language(s).

use bastyde::prelude::*;

#[allow(unused_imports)]
use super::super::*;

/// Work: `<name>` ▸ Language — the project's default spell-check language(s), edited with
/// the shared [`LanguagePillField`](crate::spellcheck::language_pill_field::LanguagePillField) over the
/// live `SingleWork::dict_language` signal. Adding a language persists it and saves.
pub(in crate::settings) fn work_language_pane(
    ctx: &mut BuildContext,
    vm: &WorkSettingsViewModel,
    work_title: String,
    open_docs: crate::models::OpenDocsStore,
) -> impl Widget {
    // Build the whole form per branch so the pill field is added through FormLayout's own
    // `full_width(widget)` **deferred insertion**, which parents it to the FormLayout —
    // parenting it to this build context instead orphans it into an arena root that the
    // layout pass then places at the window origin with the full window size.
    let base = FormLayout::new()
        .label(tr!(settings_page_language()))
        .label_gap(16.0)
        .row_spacing(14.0)
        .full_width(group(tr!(settings_field_dict_language())));
    // `open_docs` is the OPENING WINDOW's own `WorkSession::open_docs`, threaded in by
    // `SettingsPanel::build` — never `ctx.app_state::<OpenDocsStore>()`, which would
    // silently resolve to whichever Work's session registered it first. `SpellcheckService`
    // stays resolved via `ctx.app_state`: it is genuinely process-wide (internally
    // partitioned per `work_id` — see its own module doc), not per-Work state.
    let form = match (
        ctx.app_state::<crate::spellcheck::SpellcheckService>()
            .cloned(),
        Some(open_docs),
    ) {
        (Some(spell), Some(open_docs)) => {
            let value = vm.dict_language();
            let set: crate::spellcheck::language_pill_field::SetLanguages = {
                let vm = vm.clone();
                Rc::new(move |new: Vec<String>, _c| vm.set_dict_language(new))
            };
            // The Work is the root of the inheritance chain — nothing to inherit from.
            base.full_width(
                crate::spellcheck::language_pill_field::LanguagePillField::new(
                    value, set, spell, None, open_docs,
                ),
            )
        }
        _ => base.full_width(TextWidget::new(tr!(settings_field_dict_language()))),
    };

    pane_frame(
        crumb(
            Some(lit!(format!(
                "{}: {}",
                tr!(settings_sec_work()).resolve_now(),
                work_title
            ))),
            tr!(settings_page_language()),
        ),
        form,
    )
}
