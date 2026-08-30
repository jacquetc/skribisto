// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Work: `<name>` ▸ Author — the writer's name for this project.

use teksilo::widgets::TextInput;
use teksilo::widgets::tooltip::TooltipContent;

#[allow(unused_imports)]
use super::super::*;

/// Work: `<name>` ▸ Author — the name that appears on the compiled title page and
/// in the exported metadata. Optional: a blank field is the ordinary "not set"
/// state, and the compiler omits the line entirely rather than printing a gap.
///
/// The field is a **local** `Signal` mirrored from the model, not `vm.author_name()`
/// bound directly. `TextInput` is two-way, so binding the model signal would write
/// every keystroke straight into the entity — bypassing
/// [`WorkSettingsViewModel::set_author_name`], and with it the `save()` that puts
/// the change on disk and the guard that keeps an unedited pass through the field
/// from queuing an empty undo step. Committing on Enter *and* on blur, like the tag
/// fields, because nothing about a bare inline field announces that it needs
/// confirming — typing a name and clicking away must not discard it.
pub(in crate::settings) fn work_author_pane(
    ctx: &mut BuildContext,
    vm: &WorkSettingsViewModel,
    crumbs: &Crumbs,
) -> impl Widget {
    let typed = Signal::new(vm.author_name().get());

    // Keep the field in step when the Work changes underneath it — loading another
    // project, or an undo of a previous edit to this very field.
    {
        let typed = typed.clone();
        ctx.effect(&vm.author_name(), move |from_model| {
            if typed.get() != *from_model {
                typed.set(from_model.clone());
            }
        });
    }

    let field = {
        let vm = vm.clone();
        let sig = typed.clone();
        // `set_author_name` trims and compares before writing, so a blur that
        // changed nothing costs nothing. Blank IS meaningful here — it clears the
        // name — which is why there is no non-empty guard.
        let commit = move || vm.set_author_name(sig.get());
        let on_blur = commit.clone();
        TextInput::new(typed.clone())
            .placeholder(tr!(settings_field_author_placeholder()))
            .rich_tooltip_content(TooltipContent::new(
                "settings.author_name",
                tr!(settings_field_author_hint()),
            ))
            .on_submit_fn(move |_c| commit())
            .on_blur_fn(move |_c| on_blur())
    };

    let form = FormLayout::new()
        .label(tr!(settings_page_author()))
        .label_gap(16.0)
        .row_spacing(14.0)
        .line(field_label(tr!(settings_field_author_name())), field);

    pane_frame(crumbs.of(Pane::WorkAuthor), form)
}
