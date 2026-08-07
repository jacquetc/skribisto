// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Editor ▸ Scene / Synopsis / Notes — one typography page per editor kind.
//!
//! All three are the *same* form over a different [`EditorTypography`] bundle, so the page
//! title is a parameter rather than three near-identical bodies.

use teksilo::prelude::*;

#[allow(unused_imports)]
use super::super::*;

/// A `FontPicker` bound to a persisted typeface `Signal<String>` (bridged to
/// the picker's `Option<String>` selection; the effect mirrors external
/// changes — Reset — back into the picker). Self-populates from the shared
/// typesetter's real font database, including the bundled writing serifs, so
/// every offered name renders.
pub(in crate::settings) fn font_picker(
    ctx: &mut BuildContext,
    persisted: Signal<String>,
) -> impl Widget {
    let selection: Signal<Option<String>> = Signal::new(Some(persisted.get()));
    {
        let selection = selection.clone();
        ctx.effect(&persisted, move |p: &String| {
            if selection.get().as_deref() != Some(p.as_str()) {
                selection.set(Some(p.clone()));
            }
        });
    }
    let write_back = persisted.clone();
    FixedSize::new().width(240.0).child(
        FontPicker::new(selection)
            .placeholder(tr!(settings_field_typeface()))
            .on_select(move |f: &str, _ctx| write_back.set(f.to_string())),
    )
}

/// Append the Typography group's rows to `form`, bound to `typo`'s live signals.
///
/// Extracted from [`typography_pane`] because the distraction-free page needs
/// the *same rows* under a page that also carries a column width and the
/// control-strip toggles, and the strip's quick-access popover needs them again
/// in a third place. `FormLayout` is a row-accumulating builder — which is why
/// the settings pane bodies are chained builders rather than `teksu!` — so
/// "append to a form" is the shape that composes, not "return a widget".
pub(crate) fn typography_rows(
    ctx: &mut BuildContext,
    form: FormLayout,
    typo: &EditorTypography,
) -> FormLayout {
    form.full_width(group(tr!(settings_group_typography())))
        .line(
            field_label(tr!(settings_field_typeface())),
            font_picker(ctx, typo.font_family.clone()),
        )
        .line(
            field_label(tr!(settings_field_size())),
            slider_field(typo.size.clone(), 0.7, 1.6, 0.05, |v| {
                format!("{:.0}%", v * 100.0)
            }),
        )
        .line(
            field_label(tr!(settings_field_line_height())),
            slider_field(typo.line_height.clone(), 1.0, 2.4, 0.02, |v| {
                format!("{v:.2}")
            }),
        )
        .line(
            field_label(tr!(settings_field_first_line_indent())),
            slider_field(typo.first_line_indent.clone(), 0.0, 60.0, 2.0, |v| {
                format!("{} px", v.round() as i32)
            }),
        )
        .line(
            field_label(tr!(settings_field_paragraph_spacing_before())),
            slider_field(typo.para_spacing_before.clone(), 0.0, 40.0, 2.0, |v| {
                format!("{} px", v.round() as i32)
            }),
        )
        .line(
            field_label(tr!(settings_field_paragraph_spacing_after())),
            slider_field(typo.para_spacing_after.clone(), 0.0, 40.0, 2.0, |v| {
                format!("{} px", v.round() as i32)
            }),
        )
}

/// One per-editor-type typography page (Scene / Synopsis / Notes): nothing but
/// [`typography_rows`] under its own page heading. Distraction-free appends the same
/// rows into a page of its own alongside other fields; Corkboard reimplements them
/// by hand rather than sharing the helper.
pub(in crate::settings) fn typography_pane(
    ctx: &mut BuildContext,
    page: LocalizedString,
    typo: &EditorTypography,
) -> impl Widget {
    let form = FormLayout::new()
        .label(page.clone())
        .label_gap(16.0)
        .row_spacing(14.0);
    let form = typography_rows(ctx, form, typo);
    pane_frame(crumb(Some(tr!(settings_sec_editor())), page), form)
}
