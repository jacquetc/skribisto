// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Work: `<name>` ▸ Language — the open project's default spell-check language(s).

use bastyde::prelude::*;

#[allow(unused_imports)]
use super::super::*;

/// Work: `<name>` ▸ Language — the project's default spell-check language(s), edited with
/// the shared [`LanguagePillField`](crate::spellcheck::language_pill_field::LanguagePillField) over the
/// live `SingleWork::dict_language` signal. Adding a language persists it and saves; a hint
/// states the multi-language trade-off.
pub(in crate::settings) fn work_language_pane(
    ctx: &mut BuildContext,
    vm: &WorkSettingsViewModel,
    work_title: String,
) -> impl Widget {
    // Build the whole form per branch so the pill field is added through FormLayout's own
    // `full_width(widget)` **deferred insertion** (which parents it to the FormLayout). The
    // earlier `ctx.add_boxed(field)` + `full_width_id` route parented the field to *this*
    // build context instead, orphaning it into an arena root — which the layout pass then
    // placed at the window origin (0,0) with the full window size, leaking a stray pill row
    // to the top-left that even survived closing Settings.
    let base = FormLayout::new()
        .label(tr!(settings_page_language()))
        .label_gap(16.0)
        .row_spacing(12.0)
        .full_width(group(tr!(settings_field_dict_language())));
    let form = match ctx
        .app_state::<crate::spellcheck::SpellcheckService>()
        .cloned()
    {
        Some(spell) => {
            let value = vm.dict_language();
            let set: crate::spellcheck::language_pill_field::SetLanguages = {
                let vm = vm.clone();
                Rc::new(move |new: Vec<String>, _c| vm.set_dict_language(new))
            };
            // The Work is the root of the inheritance chain — nothing to inherit from.
            base.full_width(
                crate::spellcheck::language_pill_field::LanguagePillField::new(
                    value, set, spell, None,
                ),
            )
        }
        None => base.full_width(TextWidget::new(tr!(settings_field_dict_language()))),
    }
    .full_width(hint(tr!(dict_tradeoff_hint())));

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
