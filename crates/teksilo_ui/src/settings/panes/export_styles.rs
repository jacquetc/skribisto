// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Settings ▸ Compile & Export ▸ **Export Formats** — the export-style manager.
//!
//! Two sections over the [`ExportStylesViewModel`]:
//! - **Built-in styles** — the shipped read-only presets, each badged and offering *Duplicate*
//!   (duplicate-to-edit, the only way to get an editable copy).
//! - **My styles** — the user's editable copies, each offering *Edit* (loads it into the form
//!   editor below), *Export…* (to a portable JSON file), and *Delete*; plus an *Import…* button.
//!
//! The **editor** below the lists is a `FormLayout` bound to the selected user style; each
//! control writes its field straight back through `vm.update`. It rebinds only on the *selection*
//! signal (never on the styles-changed bump), so editing a field never rebuilds the form under
//! the user's cursor — the lists, which *do* bind the bump, refresh names live.
//!
//! Like `panes::dictionaries`, it needs generic-closure widgets (`ListView`) the `teksu!` DSL
//! can't express, so it is a chained-builder module.

use teksilo::core::binding::BindingLevel;
use teksilo::core::widget::WidgetPlacement;
use teksilo::data::ListModel;
use teksilo::prelude::*;
use teksilo::widgets::{
    Badge, Button, ButtonVariant, ComboBox, FixedSize, FontPicker, FormLayout, HStack, ListView,
    MaxSize, Padding, Segment, SegmentedControl, Spacer, StandardListItem, TextInput, TextWidget,
    Toast, Toggle, VStack,
};

use skribisto_compiler::{
    DigitStyle, DirectionMode, EpigraphPlacement, ExportFormat, FootnoteNumbering, HeadingLanguage,
    HeadingScheme, ImageHandling, LineSpacing, PageSize, Preset, SceneBreak,
};
use skribisto_model::scene_break::SceneBreakTier;

use crate::settings::{field_label, group};
use crate::view_models::ExportStylesViewModel;

/// One list row (built-in or user), derived from a [`Preset`].
#[derive(Clone)]
struct StyleRow {
    id: String,
    name: String,
    /// A one-line summary (the font — plain data).
    subtitle: String,
}

fn row_of(p: &Preset) -> StyleRow {
    StyleRow {
        id: p.id.clone(),
        name: p.name.clone(),
        subtitle: p.font_family.clone(),
    }
}

/// The whole Export-Formats pane body. The caller wraps it in `pane_frame`.
pub fn export_styles_pane(ctx: &mut BuildContext, vm: &ExportStylesViewModel) -> impl Widget {
    // The user style currently loaded in the editor (by id).
    let selected: Signal<Option<String>> = Signal::new(None);

    // ── Built-in styles (static) ──
    let builtin_model = ListModel::from_vec(vm.builtin_presets().iter().map(row_of).collect());
    let bvm = vm.clone();
    // Captured once: the built-ins never change, so the per-row parameter sheet
    // does not need rebuilding them on every render.
    let builtin_sheets = vm.builtin_presets();
    let builtin_list = ListView::new(builtin_model, move |_i, row: &StyleRow, _sel| {
        let sheet = builtin_sheets.iter().find(|p| p.id == row.id).cloned();
        Box::new(builtin_row(&bvm, row, sheet))
    })
    .auto_item_height(52.0);

    // ── User styles (live: repopulated on every styles-changed bump) ──
    let user_model = ListModel::from_vec(vm.user_presets().iter().map(row_of).collect());
    {
        let model = user_model.clone();
        let vm = vm.clone();
        let selected = selected.clone();
        ctx.effect(&vm.changed_signal(), move |_| {
            let rows: Vec<StyleRow> = vm.user_presets().iter().map(row_of).collect();
            // If the edited style was deleted, drop the editor selection.
            if let Some(cur) = selected.get()
                && !rows.iter().any(|r| r.id == cur)
            {
                selected.set(None);
            }
            model.replace_all(rows);
        });
    }
    let uvm = vm.clone();
    let usel = selected.clone();
    let user_list = ListView::new(user_model, move |_i, row: &StyleRow, _sel| {
        // `user_preset(id)` looks one style up; `user_presets()` cloned the whole
        // vector per row, which is O(N²) `Preset` clones for an N-row list. Still
        // a live read, not a captured copy — a user style is editable, so a
        // snapshot would show stale values in the sheet after every edit.
        let sheet = uvm.user_preset(&row.id);
        Box::new(user_row(&uvm, row, usel.clone(), sheet))
    })
    .auto_item_height(52.0);

    // Import button (adds a user style from a JSON file).
    let import_vm = vm.clone();
    let import_sel = selected.clone();
    let toolbar = HStack::new().spacing(8.0).child(Spacer::new()).child(
        Button::new(tr!(settings_styles_import()))
            .variant(ButtonVariant::Tinted)
            .on_activate_fn(move |c| present_import(c, import_vm.clone(), import_sel.clone())),
    );

    VStack::new()
        .spacing(6.0)
        // Shared `group()` so section headers match every other settings pane
        // (SmallBold + Secondary), not a bare unstyled `GroupHeader::new`.
        .child(group(tr!(settings_styles_builtin())))
        // `MaxSize::height`, not `MinSize`: as a *minimum* the list grew to fit
        // its content, and at ten built-in styles it swallowed the whole pane,
        // pushing "My styles" and the editor below it out of the modal. A
        // height-only `FixedSize` is not the fix either — it proposes
        // `width: None`, which collapses the rows to the left. `MaxSize` caps
        // the height and leaves the width to fill; each list scrolls internally.
        .child(Padding::symmetric(4.0, 0.0).child(MaxSize::height(200.0).child(builtin_list)))
        .child(Padding::new(14.0, 0.0, 0.0, 0.0).child(group(tr!(settings_styles_user()))))
        .child(Padding::symmetric(6.0, 4.0).child(toolbar))
        .child(Padding::symmetric(4.0, 0.0).child(MaxSize::height(150.0).child(user_list)))
        .child(Padding::new(14.0, 0.0, 0.0, 0.0).child(group(tr!(settings_styles_editor_group()))))
        .child(StyleEditor::new(vm.clone(), selected))
}

fn builtin_row(vm: &ExportStylesViewModel, row: &StyleRow, sheet: Option<Preset>) -> impl Widget {
    let vm = vm.clone();
    let base_id = row.id.clone();
    let duplicate = Button::new(tr!(settings_styles_duplicate()))
        .variant(ButtonVariant::Plain)
        .on_activate_fn(move |_c| {
            if let Some(base) = vm.builtin_presets().into_iter().find(|p| p.id == base_id) {
                // A localized copy suffix is resolved at call time and baked into the new name.
                vm.duplicate(&base, &tr!(settings_styles_copy_suffix()).resolve_now());
            }
        });
    let trailing = HStack::new()
        .spacing(8.0)
        .child(Badge::new(tr!(settings_styles_builtin_badge())))
        .child(duplicate);
    let item = StandardListItem::new(lit!(row.name.clone()))
        .subtitle(lit!(row.subtitle.clone()))
        .trailing_slot(trailing);
    match sheet {
        Some(p) => item.composite_tooltip(preset_sheet(&p)),
        None => item,
    }
}

fn user_row(
    vm: &ExportStylesViewModel,
    row: &StyleRow,
    selected: Signal<Option<String>>,
    sheet: Option<Preset>,
) -> impl Widget {
    let id = row.id.clone();

    let edit = {
        let selected = selected.clone();
        let id = id.clone();
        Button::new(tr!(settings_styles_edit()))
            .variant(ButtonVariant::Plain)
            .on_activate_fn(move |_c| selected.set(Some(id.clone())))
    };
    let export = {
        let vm = vm.clone();
        let id = id.clone();
        let name = row.name.clone();
        Button::new(tr!(settings_styles_export()))
            .variant(ButtonVariant::Plain)
            .on_activate_fn(move |c| present_export(c, vm.clone(), &id, &name))
    };
    let delete = {
        let vm = vm.clone();
        let id = id.clone();
        Button::new(tr!(settings_styles_delete()))
            .variant(ButtonVariant::Plain)
            .on_activate_fn(move |_c| vm.remove(&id))
    };
    let trailing = HStack::new()
        .spacing(6.0)
        .child(edit)
        .child(export)
        .child(delete);

    // Highlight the row currently loaded in the editor.
    let id_hl = id.clone();
    let item = StandardListItem::new(lit!(row.name.clone()))
        .subtitle(lit!(row.subtitle.clone()))
        .selected(selected.map(move |s| s.as_deref() == Some(id_hl.as_str())))
        .trailing_slot(trailing);
    match sheet {
        Some(p) => item.composite_tooltip(preset_sheet(&p)),
        None => item,
    }
}

// ── JSON import / export (file dialogs) ──

fn present_import(
    ctx: &mut EventContext,
    vm: ExportStylesViewModel,
    selected: Signal<Option<String>>,
) {
    let req = crate::models::dialog_start_in(
        ctx,
        crate::models::FolderPurpose::DataInterchange,
        FileDialogRequest::pick_file()
            .title(tr!(settings_styles_import()))
            .add_filter(tr!(settings_styles_json_filter()).resolve_now(), &["json"]),
    );
    let _ = ctx.pick_file(req, move |res, ectx| {
        if let FileDialogResult::File(Some(path)) = res {
            crate::models::remember_dialog_file(
                ectx,
                crate::models::FolderPurpose::DataInterchange,
                &path,
            );
            match vm.import_from(&path) {
                Ok(p) => {
                    selected.set(Some(p.id.clone()));
                    // Broadcast: export-style presets are a shared user library
                    // (`ExportStylesViewModel` is a Tier-1 singleton, not
                    // per-Work — see its view-model), usable from any open
                    // project's Export panel.
                    ectx.show_toast(
                        Toast::info(tr!(settings_styles_imported()))
                            .body(lit!(p.name.clone()))
                            .auto_dismiss_after(std::time::Duration::from_secs(4))
                            .broadcast(),
                    );
                }
                Err(e) => {
                    ectx.show_toast(
                        Toast::error(tr!(settings_styles_import_failed()))
                            .body(lit!(format!("{e:#}")))
                            .auto_dismiss_after(std::time::Duration::from_secs(6))
                            .broadcast(),
                    );
                }
            }
        }
    });
}

fn present_export(ctx: &mut EventContext, vm: ExportStylesViewModel, id: &str, name: &str) {
    let id = id.to_string();
    let default_name = format!("{}.json", slugify(name));
    let req = crate::models::dialog_start_in(
        ctx,
        crate::models::FolderPurpose::DataInterchange,
        FileDialogRequest::save_file()
            .title(tr!(settings_styles_export()))
            .default_file_name(default_name)
            .add_filter(tr!(settings_styles_json_filter()).resolve_now(), &["json"]),
    );
    let _ = ctx.save_file(req, move |res, ectx| {
        if let FileDialogResult::Saved(Some(path)) = res {
            crate::models::remember_dialog_file(
                ectx,
                crate::models::FolderPurpose::DataInterchange,
                &path,
            );
            let mut target = path.clone();
            if target.extension().and_then(|e| e.to_str()) != Some("json") {
                target.set_extension("json");
            }
            // Origin-window default (not broadcast), unlike import just above:
            // exporting reads the shared style library out to an arbitrary
            // file — it changes no state another window's Export panel could
            // care about, so this is purely local "did my file save" feedback.
            match vm.export_to(&target, &id) {
                Ok(()) => {
                    ectx.show_toast(
                        Toast::info(tr!(settings_styles_exported()))
                            .body(lit!(target.to_string_lossy().into_owned()))
                            .auto_dismiss_after(std::time::Duration::from_secs(4)),
                    );
                }
                Err(e) => {
                    ectx.show_toast(
                        Toast::error(tr!(settings_styles_export_failed()))
                            .body(lit!(format!("{e:#}")))
                            .auto_dismiss_after(std::time::Duration::from_secs(6)),
                    );
                }
            }
        }
    });
}

/// A filesystem-safe base name for the default export file name.
fn slugify(name: &str) -> String {
    let s: String = name
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '-' })
        .collect();
    let trimmed = s.trim_matches('-').to_string();
    if trimmed.is_empty() {
        "export-style".to_string()
    } else {
        trimmed
    }
}

// ── The style editor (reactive on the selection) ──

/// The `FormLayout` editor for the selected user style. A widget (not a plain builder) so it can
/// rebind on the selection signal at `Rebuild` and reseed its controls from the chosen preset.
struct StyleEditor {
    vm: ExportStylesViewModel,
    selected: Signal<Option<String>>,
    child_id: Option<WidgetId>,
}

impl StyleEditor {
    fn new(vm: ExportStylesViewModel, selected: Signal<Option<String>>) -> Self {
        Self {
            vm,
            selected,
            child_id: None,
        }
    }
}

impl std::fmt::Debug for StyleEditor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StyleEditor").finish()
    }
}

/// Register a write-back effect: when `sig` changes, apply it to the selected user preset and
/// persist — but only when it actually changes a field (so the effect's initial fire is a no-op).
fn bind_field<T: Clone + PartialEq + 'static>(
    ctx: &mut BuildContext,
    vm: &ExportStylesViewModel,
    id: &str,
    sig: &Signal<T>,
    apply: impl Fn(&mut Preset, T) + 'static,
) {
    let vm = vm.clone();
    let id = id.to_string();
    ctx.effect(sig, move |v: &T| {
        if let Some(mut p) = vm.user_preset(&id) {
            let before = p.clone();
            apply(&mut p, v.clone());
            if p != before {
                vm.update(&p);
            }
        }
    });
}

/// A `FontPicker` bound to a typeface `Signal<String>` (bridged to the picker's `Option<String>`
/// selection), self-populating from the shared typesetter's real font database.
fn font_picker(ctx: &mut BuildContext, persisted: Signal<String>) -> impl Widget {
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

fn heading_label(scheme: &HeadingScheme) -> LocalizedString {
    match scheme {
        HeadingScheme::None => tr!(settings_styles_heading_none()),
        HeadingScheme::Numbered => tr!(settings_styles_heading_numbered()),
        HeadingScheme::TitleOnly => tr!(settings_styles_heading_title()),
        HeadingScheme::NumberAndTitle => tr!(settings_styles_heading_both()),
    }
}

/// One tier's glyph picker: a curated set, plus the preset's own current glyph
/// when it is not already in it (a regional built-in may use `＊`, `◇` or `…`),
/// bound through [`bind_field`] and carrying `tip` as a rich tooltip.
///
/// Both tiers are identical but for which `Preset` field they write, so they
/// share this rather than duplicating the list-and-fallback logic.
fn break_combo(
    ctx: &mut BuildContext,
    vm: &ExportStylesViewModel,
    id: &str,
    current: SceneBreak,
    tier: SceneBreakTier,
    apply: impl Fn(&mut Preset, SceneBreak) + 'static,
    tip: &'static str,
) -> ComboBox<SceneBreak> {
    // Each tier offers its own glyphs. Offering the canonical major mark on the
    // minor picker would invite a preset whose ordinary break prints *stronger*
    // than its major one — an inverted hierarchy nothing would warn about.
    let mut breaks = vec![SceneBreak::BlankLine];
    breaks.extend(
        match tier {
            SceneBreakTier::Minor => ["#", "* * *", "*"].as_slice(),
            SceneBreakTier::Major => ["# # #", "* * *", "⁂"].as_slice(),
        }
        .iter()
        .map(|g| SceneBreak::Glyph((*g).to_string())),
    );
    breaks.push(SceneBreak::None);
    if !breaks.contains(&current) {
        breaks.insert(1, current.clone());
    }
    let sig = Signal::new(Some(current));
    bind_field(ctx, vm, id, &sig, move |p, v| {
        if let Some(v) = v {
            apply(p, v);
        }
    });
    ComboBox::from_items(breaks, sig, scene_break_label).rich_tooltip(tip)
}

/// The full parameter sheet for a preset, as a composite-tooltip body.
///
/// Every list row carries one. A style is ~20 settings and the row can only show
/// its name and font, so choosing between "Standard Manuscript (Shunn)" and
/// "Manuscrit (français)" otherwise means duplicating one and opening the editor
/// just to read it.
///
/// Laid out as **two label/value pairs per line** in `Tiny` type: one pair per
/// line made a tooltip taller than the list it described. Colours come from the
/// tooltip roles, not the surface roles — the body paints on the tooltip's own
/// dark chrome, where `Primary`/`Secondary` are the wrong contrast pair.
fn preset_sheet(p: &Preset) -> impl Widget + 'static {
    // Rust has no `%g`: render 12.0 as "12" and 0.5 as "0.5".
    fn num(v: f32) -> String {
        let t = format!("{v:.2}");
        t.trim_end_matches('0').trim_end_matches('.').to_string()
    }
    let yes_no = |b: bool| {
        if b {
            tr!(settings_styles_sheet_yes())
        } else {
            tr!(settings_styles_sheet_no())
        }
    };

    let pairs: Vec<(LocalizedString, LocalizedString)> = vec![
        (
            tr!(settings_styles_sheet_font()),
            lit!(format!("{} {} pt", p.font_family, num(p.font_size_pt))),
        ),
        (
            tr!(settings_styles_field_spacing()),
            spacing_label(p.line_spacing),
        ),
        (
            tr!(settings_styles_sheet_indent()),
            lit!(format!("{}\u{2033}", num(p.first_line_indent_in))),
        ),
        (
            tr!(settings_styles_sheet_para_spacing()),
            lit!(format!("{} pt", num(p.paragraph_spacing_pt))),
        ),
        (tr!(settings_styles_field_justify()), yes_no(p.justify)),
        (
            tr!(settings_styles_sheet_page()),
            page_size_label(p.page_size),
        ),
        (
            tr!(settings_styles_sheet_margins()),
            lit!(format!(
                "{} / {} / {} / {}\u{2033}",
                num(p.margin.top_in),
                num(p.margin.right_in),
                num(p.margin.bottom_in),
                num(p.margin.left_in)
            )),
        ),
        (
            tr!(settings_styles_sheet_title_page()),
            yes_no(p.book_title_page),
        ),
        (
            tr!(settings_styles_field_chapters()),
            heading_label(&p.chapter_heading),
        ),
        (
            tr!(settings_styles_field_parts()),
            heading_label(&p.part_heading),
        ),
        (
            tr!(settings_styles_field_scene_break()),
            scene_break_label(&p.scene_break),
        ),
        (
            tr!(settings_styles_field_major_scene_break()),
            scene_break_label(&p.major_scene_break),
        ),
        (tr!(settings_styles_field_notes()), yes_no(p.include_notes)),
        (
            tr!(settings_styles_field_synopses()),
            yes_no(p.include_synopses),
        ),
        (
            tr!(settings_styles_field_scene_titles()),
            yes_no(p.include_scene_titles),
        ),
        (
            tr!(settings_styles_field_epigraphs()),
            yes_no(p.include_epigraphs),
        ),
        (
            tr!(settings_styles_field_epigraph_placement()),
            epigraph_placement_label(p.epigraph_placement),
        ),
        (
            tr!(settings_styles_field_footnotes()),
            yes_no(p.include_footnotes),
        ),
        (
            tr!(settings_styles_field_footnote_numbering()),
            footnote_numbering_label(&p.footnote_numbering),
        ),
        (
            tr!(settings_styles_field_paratexts()),
            yes_no(p.include_paratexts),
        ),
        (
            tr!(settings_styles_field_images()),
            image_handling_label(p.image_handling),
        ),
        (tr!(settings_styles_field_cover()), yes_no(p.book_cover)),
        (
            tr!(settings_styles_field_word_count()),
            yes_no(p.title_page_word_count),
        ),
        (
            tr!(settings_styles_field_page_books()),
            yes_no(p.book_starts_page),
        ),
        (
            tr!(settings_styles_field_page_parts()),
            yes_no(p.part_starts_page),
        ),
        (
            tr!(settings_styles_field_page_chapters()),
            yes_no(p.chapter_starts_page),
        ),
        (
            tr!(settings_styles_field_page_paratexts()),
            yes_no(p.paratext_starts_page),
        ),
        (
            tr!(settings_styles_sheet_heading_language()),
            match &p.heading_language {
                HeadingLanguage::Auto => tr!(settings_styles_sheet_auto()),
                HeadingLanguage::Fixed(l) => lit!(l.clone()),
            },
        ),
        (
            tr!(settings_styles_sheet_digits()),
            digit_style_label(p.digit_style),
        ),
        (
            tr!(settings_styles_sheet_direction()),
            direction_label(p.direction),
        ),
        (
            tr!(settings_styles_sheet_formats()),
            if p.formats.is_empty() {
                tr!(settings_styles_sheet_all_formats())
            } else {
                lit!(
                    p.formats
                        .iter()
                        .map(|f| format_label(*f).resolve_now())
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            },
        ),
    ];

    let label_of = |t: LocalizedString| {
        TextWidget::new(t)
            .style(TextStyleRole::Tiny)
            .color(TextRole::TooltipShortcut)
    };
    let value_of = |t: LocalizedString| {
        TextWidget::new(t)
            .style(TextStyleRole::Tiny)
            .color(TextRole::TooltipText)
    };

    // Two `FormLayout` columns rather than one: a single column of ~19 rows made
    // the tooltip taller than the list it describes. `FormLayout` still owns the
    // label/value alignment within each column — the same widget the pane's own
    // editor uses, so the sheet and the form it summarises line up the same way.
    let mid = pairs.len().div_ceil(2);
    let mut left = FormLayout::new().label_gap(10.0).row_spacing(2.0);
    let mut right = FormLayout::new().label_gap(10.0).row_spacing(2.0);
    for (i, (label, value)) in pairs.into_iter().enumerate() {
        if i < mid {
            left = left.line(label_of(label), value_of(value));
        } else {
            right = right.line(label_of(label), value_of(value));
        }
    }
    let columns = HStack::new().spacing(22.0).child(left).child(right);

    VStack::new()
        .spacing(5.0)
        .child(
            TextWidget::new(lit!(p.name.clone()))
                .style(TextStyleRole::SmallBold)
                .color(TextRole::TooltipText),
        )
        .child(columns)
}

fn page_size_label(p: PageSize) -> LocalizedString {
    match p {
        // Paper names are proper nouns, the same everywhere.
        PageSize::A4 => lit!("A4"),
        PageSize::A5 => lit!("A5"),
        PageSize::Letter => tr!(settings_styles_page_letter()),
    }
}

fn epigraph_placement_label(p: EpigraphPlacement) -> LocalizedString {
    match p {
        EpigraphPlacement::AfterHeading => tr!(settings_styles_epigraph_after()),
        EpigraphPlacement::BeforeHeading => tr!(settings_styles_epigraph_before()),
    }
}

fn image_handling_label(h: ImageHandling) -> LocalizedString {
    match h {
        ImageHandling::CopyBeside => tr!(settings_styles_images_beside()),
        ImageHandling::Embed => tr!(settings_styles_images_embed()),
        ImageHandling::Omit => tr!(settings_styles_images_omit()),
    }
}

fn footnote_numbering_label(n: &FootnoteNumbering) -> LocalizedString {
    match n {
        FootnoteNumbering::Continuous => tr!(settings_styles_footnote_numbering_continuous()),
        FootnoteNumbering::PerChapter => tr!(settings_styles_footnote_numbering_per_chapter()),
        FootnoteNumbering::PerBook => tr!(settings_styles_footnote_numbering_per_book()),
    }
}

fn digit_style_label(d: DigitStyle) -> LocalizedString {
    match d {
        DigitStyle::Western => tr!(settings_styles_digits_western()),
        DigitStyle::EasternArabic => tr!(settings_styles_digits_eastern_arabic()),
    }
}

fn direction_label(d: DirectionMode) -> LocalizedString {
    match d {
        DirectionMode::Auto => tr!(settings_styles_sheet_auto()),
        DirectionMode::ForceLtr => tr!(settings_styles_direction_ltr()),
        DirectionMode::ForceRtl => tr!(settings_styles_direction_rtl()),
    }
}

fn format_label(f: ExportFormat) -> LocalizedString {
    // The format *names* are proper nouns (DOCX, EPUB, PDF…), so they stay
    // literal — unlike the enums above, whose variants are English words.
    lit!(match f {
        ExportFormat::PlainText => "TXT",
        ExportFormat::Markdown => "Markdown",
        ExportFormat::Html => "HTML",
        ExportFormat::Djot => "Djot",
        ExportFormat::Latex => "LaTeX",
        ExportFormat::Docx => "DOCX",
        ExportFormat::Epub => "EPUB",
        ExportFormat::Pdf => "PDF",
    })
}

fn spacing_label(s: LineSpacing) -> LocalizedString {
    match s {
        LineSpacing::Single => tr!(settings_styles_spacing_single()),
        LineSpacing::OneAndHalf => tr!(settings_styles_spacing_onehalf()),
        LineSpacing::Double => tr!(settings_styles_spacing_double()),
    }
}

fn scene_break_label(sb: &SceneBreak) -> LocalizedString {
    match sb {
        SceneBreak::Glyph(g) => lit!(g.clone()),
        SceneBreak::BlankLine => tr!(settings_styles_break_blank()),
        SceneBreak::None => tr!(settings_styles_break_none()),
    }
}

impl Widget for StyleEditor {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        self.selected
            .bind_to(ctx.self_id(), ctx.binding_registry(), BindingLevel::Rebuild);

        let Some(id) = self.selected.get() else {
            let hint = Padding::symmetric(20.0, 4.0).child(
                TextWidget::new(tr!(settings_styles_editor_none()))
                    .style(TextStyleRole::Small)
                    .color(TextRole::Secondary),
            );
            let child = ctx.add(hint);
            self.child_id = Some(child);
            return vec![child];
        };
        let Some(preset) = self.vm.user_preset(&id) else {
            // The selected style vanished under us (deleted in another window,
            // or the id outlived a reload). Rendering an empty `TextWidget`
            // here was indistinguishable from the editor silently failing to
            // open — say what happened instead.
            let child = ctx.add(
                Padding::symmetric(20.0, 4.0).child(
                    TextWidget::new(tr!(settings_styles_editor_missing()))
                        .style(TextStyleRole::Small)
                        .color(TextRole::Secondary),
                ),
            );
            self.child_id = Some(child);
            return vec![child];
        };

        // Name.
        let name = Signal::new(preset.name.clone());
        bind_field(ctx, &self.vm, &id, &name, |p, v| p.name = v);

        // Typeface.
        let font = Signal::new(preset.font_family.clone());
        bind_field(ctx, &self.vm, &id, &font, |p, v| p.font_family = v);

        // Chapter / Part heading schemes.
        let schemes = vec![
            HeadingScheme::None,
            HeadingScheme::Numbered,
            HeadingScheme::TitleOnly,
            HeadingScheme::NumberAndTitle,
        ];
        let chapter = Signal::new(Some(preset.chapter_heading));
        bind_field(ctx, &self.vm, &id, &chapter, |p, v| {
            if let Some(v) = v {
                p.chapter_heading = v;
            }
        });
        let chapter_box = ComboBox::from_items(schemes.clone(), chapter, heading_label);
        let part = Signal::new(Some(preset.part_heading));
        bind_field(ctx, &self.vm, &id, &part, |p, v| {
            if let Some(v) = v {
                p.part_heading = v;
            }
        });
        let part_box = ComboBox::from_items(schemes, part, heading_label);

        // Scene break, per tier. The author decides *where* a break goes by
        // marking it in the prose; these decide only what each tier prints as.
        // Both carry the rich tooltip that explains the minor/major distinction,
        // since this is where a writer meets it.
        let break_box = break_combo(
            ctx,
            &self.vm,
            &id,
            preset.scene_break.clone(),
            SceneBreakTier::Minor,
            |p, v| p.scene_break = v,
            crate::tooltip_registry::SCENE_BREAK_MINOR,
        );
        let major_break_box = break_combo(
            ctx,
            &self.vm,
            &id,
            preset.major_scene_break.clone(),
            SceneBreakTier::Major,
            |p, v| p.major_scene_break = v,
            crate::tooltip_registry::SCENE_BREAK_MAJOR,
        );

        // Line spacing (a segmented control).
        let spacing_idx = Signal::new(match preset.line_spacing {
            LineSpacing::Single => 0usize,
            LineSpacing::OneAndHalf => 1,
            LineSpacing::Double => 2,
        });
        bind_field(ctx, &self.vm, &id, &spacing_idx, |p, v| {
            p.line_spacing = match v {
                0 => LineSpacing::Single,
                1 => LineSpacing::OneAndHalf,
                _ => LineSpacing::Double,
            };
        });
        let spacing = SegmentedControl::new(spacing_idx)
            .segment(Segment::new(tr!(settings_styles_spacing_single())))
            .segment(Segment::new(tr!(settings_styles_spacing_onehalf())))
            .segment(Segment::new(tr!(settings_styles_spacing_double())));

        // Boolean inclusions.
        let justify = Signal::new(preset.justify);
        bind_field(ctx, &self.vm, &id, &justify, |p, v| p.justify = v);
        let notes = Signal::new(preset.include_notes);
        bind_field(ctx, &self.vm, &id, &notes, |p, v| p.include_notes = v);
        let synopses = Signal::new(preset.include_synopses);
        bind_field(ctx, &self.vm, &id, &synopses, |p, v| p.include_synopses = v);
        let scene_titles = Signal::new(preset.include_scene_titles);
        bind_field(ctx, &self.vm, &id, &scene_titles, |p, v| {
            p.include_scene_titles = v
        });
        let epigraphs = Signal::new(preset.include_epigraphs);
        bind_field(ctx, &self.vm, &id, &epigraphs, |p, v| {
            p.include_epigraphs = v
        });
        // Where the epigraph sits, as a two-way segmented control: it is a choice between
        // two conventions, not a switch that is on or off.
        let placement_idx = Signal::new(match preset.epigraph_placement {
            EpigraphPlacement::AfterHeading => 0usize,
            EpigraphPlacement::BeforeHeading => 1,
        });
        bind_field(ctx, &self.vm, &id, &placement_idx, |p, v| {
            p.epigraph_placement = match v {
                1 => EpigraphPlacement::BeforeHeading,
                _ => EpigraphPlacement::AfterHeading,
            };
        });
        let placement = SegmentedControl::new(placement_idx)
            .segment(Segment::new(tr!(settings_styles_epigraph_after())))
            .segment(Segment::new(tr!(settings_styles_epigraph_before())));

        // Footnotes. `footnote_placement` (page-bottom / chapter-end / book-end) gets no
        // control here on purpose: nothing in the compiler reads it yet — `rg
        // footnote_placement crates/` turns up only its own declaration in
        // `skribisto_compiler::preset` — so every value renders an identical footnote
        // today. A selector a writer can turn without anything in the export changing is
        // worse than no selector at all, so this stays absent until some backend actually
        // branches on it (a real page-bottom note for DOCX/PDF/LaTeX, a `noteref`/`aside`
        // pop-up for EPUB/HTML, an end-of-book list for Markdown/plain text — see the
        // field's own doc comment in `preset.rs` for the intended split). Add the control
        // the same way as `epigraph_placement` above, the day that lands.
        let footnotes = Signal::new(preset.include_footnotes);
        bind_field(ctx, &self.vm, &id, &footnotes, |p, v| {
            p.include_footnotes = v
        });
        let footnote_numberings = vec![
            FootnoteNumbering::Continuous,
            FootnoteNumbering::PerChapter,
            FootnoteNumbering::PerBook,
        ];
        let footnote_numbering = Signal::new(Some(preset.footnote_numbering));
        bind_field(ctx, &self.vm, &id, &footnote_numbering, |p, v| {
            if let Some(v) = v {
                p.footnote_numbering = v;
            }
        });
        // Only meaningful with footnotes included, same reasoning as `word_count`/
        // `page_books` below being gated on the title page: disabled, not hidden, so the
        // setting stays visible as something that exists.
        let footnote_numbering_box = ComboBox::from_items(
            footnote_numberings,
            footnote_numbering,
            footnote_numbering_label,
        )
        .enabled(footnotes.clone());

        let paratexts = Signal::new(preset.include_paratexts);
        bind_field(ctx, &self.vm, &id, &paratexts, |p, v| {
            p.include_paratexts = v
        });

        // What the referencing formats do with the manuscript's pictures. Three
        // choices rather than a switch, because "beside the document" and "inside
        // it" are both ways of keeping them — the difference is what the reader
        // ends up with, not whether the images survive.
        let images_idx = Signal::new(match preset.image_handling {
            ImageHandling::CopyBeside => 0usize,
            ImageHandling::Embed => 1,
            ImageHandling::Omit => 2,
        });
        bind_field(ctx, &self.vm, &id, &images_idx, |p, v| {
            p.image_handling = match v {
                1 => ImageHandling::Embed,
                2 => ImageHandling::Omit,
                _ => ImageHandling::CopyBeside,
            };
        });
        let images = SegmentedControl::new(images_idx)
            .segment(Segment::new(tr!(settings_styles_images_beside())))
            .segment(Segment::new(tr!(settings_styles_images_embed())))
            .segment(Segment::new(tr!(settings_styles_images_omit())));

        let cover = Signal::new(preset.book_cover);
        bind_field(ctx, &self.vm, &id, &cover, |p, v| p.book_cover = v);

        // Title page + pagination.
        let title_page = Signal::new(preset.book_title_page);
        bind_field(ctx, &self.vm, &id, &title_page, |p, v| {
            p.book_title_page = v
        });
        let word_count = Signal::new(preset.title_page_word_count);
        bind_field(ctx, &self.vm, &id, &word_count, |p, v| {
            p.title_page_word_count = v
        });
        let page_books = Signal::new(preset.book_starts_page);
        bind_field(ctx, &self.vm, &id, &page_books, |p, v| {
            p.book_starts_page = v
        });
        let page_parts = Signal::new(preset.part_starts_page);
        bind_field(ctx, &self.vm, &id, &page_parts, |p, v| {
            p.part_starts_page = v
        });
        let page_chapters = Signal::new(preset.chapter_starts_page);
        bind_field(ctx, &self.vm, &id, &page_chapters, |p, v| {
            p.chapter_starts_page = v
        });
        let page_paratexts = Signal::new(preset.paratext_starts_page);
        bind_field(ctx, &self.vm, &id, &page_paratexts, |p, v| {
            p.paratext_starts_page = v
        });
        // Two switches only mean anything with a title page: the word count is printed on
        // it, and a Book's own break is what the title page already performs. Disabled
        // rather than hidden, so the setting stays visible as something that exists.
        let has_title_page = title_page.clone();
        let word_count_on = has_title_page.clone();
        let books_break_on = title_page.map(|on| !on);

        let form = FormLayout::new()
            .label(tr!(settings_styles_editor_title()))
            .label_gap(16.0)
            .row_spacing(14.0)
            .line(
                field_label(tr!(settings_styles_field_name())),
                TextInput::new(name).placeholder(tr!(settings_styles_field_name())),
            )
            .line(
                field_label(tr!(settings_field_typeface())),
                font_picker(ctx, font),
            )
            .line(
                field_label(tr!(settings_styles_field_chapters())),
                chapter_box,
            )
            .line(field_label(tr!(settings_styles_field_parts())), part_box)
            .line(
                field_label(tr!(settings_styles_field_scene_break())),
                break_box,
            )
            .line(
                field_label(tr!(settings_styles_field_major_scene_break())),
                major_break_box,
            )
            .line(field_label(tr!(settings_styles_field_spacing())), spacing)
            // Back in the label column with every other field. `FormLayout::line`
            // already wires `access_labelled_by`, so these were never actually
            // nameless — `labelled_externally` just tells the toggle's assertion
            // so, since the relation is pushed after `accessibility()` runs.
            .line(
                field_label(tr!(settings_styles_field_justify())),
                Toggle::new(justify).labelled_externally(),
            )
            .line(
                field_label(tr!(settings_styles_field_notes())),
                Toggle::new(notes).labelled_externally(),
            )
            .line(
                field_label(tr!(settings_styles_field_synopses())),
                Toggle::new(synopses).labelled_externally(),
            )
            .line(
                field_label(tr!(settings_styles_field_scene_titles())),
                Toggle::new(scene_titles).labelled_externally(),
            )
            .line(
                field_label(tr!(settings_styles_field_epigraphs())),
                Toggle::new(epigraphs).labelled_externally(),
            )
            .line(
                field_label(tr!(settings_styles_field_epigraph_placement())),
                placement,
            )
            .line(
                field_label(tr!(settings_styles_field_footnotes())),
                Toggle::new(footnotes).labelled_externally(),
            )
            .line(
                field_label(tr!(settings_styles_field_footnote_numbering())),
                footnote_numbering_box,
            )
            .line(
                field_label(tr!(settings_styles_field_paratexts())),
                Toggle::new(paratexts).labelled_externally(),
            )
            .line(field_label(tr!(settings_styles_field_images())), images)
            // Spanning both columns, with the file's own section-header idiom and its
            // breathing room above.
            .full_width(
                Padding::new(14.0, 0.0, 0.0, 0.0).child(group(tr!(settings_styles_group_pages()))),
            )
            .line(
                field_label(tr!(settings_styles_field_cover())),
                Toggle::new(cover).labelled_externally(),
            )
            .line(
                field_label(tr!(settings_styles_sheet_title_page())),
                Toggle::new(title_page).labelled_externally(),
            )
            .line(
                field_label(tr!(settings_styles_field_word_count())),
                Toggle::new(word_count)
                    .enabled(word_count_on)
                    .labelled_externally(),
            )
            .line(
                field_label(tr!(settings_styles_field_page_books())),
                Toggle::new(page_books)
                    .enabled(books_break_on)
                    .labelled_externally(),
            )
            .line(
                field_label(tr!(settings_styles_field_page_parts())),
                Toggle::new(page_parts).labelled_externally(),
            )
            .line(
                field_label(tr!(settings_styles_field_page_chapters())),
                Toggle::new(page_chapters).labelled_externally(),
            )
            .line(
                field_label(tr!(settings_styles_field_page_paratexts())),
                Toggle::new(page_paratexts).labelled_externally(),
            );

        let child = ctx.add(Padding::new(16.0, 0.0, 0.0, 0.0).child(form));
        self.child_id = Some(child);
        vec![child]
    }

    fn layout_response(&self, proposal: SizeProposal, ctx: &LayoutContext) -> LayoutResponse {
        self.child_id
            .and_then(|id| ctx.child_size(id, proposal))
            .map(LayoutResponse::from)
            .unwrap_or_else(|| proposal.resolve(0.0, 0.0).into())
    }

    fn place_children(
        &self,
        bounds: Rect,
        _proposal: SizeProposal,
        children: &mut [WidgetPlacement],
        _ctx: &LayoutContext,
    ) {
        for child in children.iter_mut() {
            child.origin = bounds.origin();
            child.size = bounds.size();
        }
    }

    fn children(&self) -> Vec<WidgetId> {
        self.child_id.into_iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::ExportStylesService;
    use teksilo::core::widget_tree::WidgetTree;

    /// A private styles file per call, so the tests do not race each other's writes.
    static NEXT: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);

    fn vm_with_a_user_style() -> (ExportStylesViewModel, String, std::path::PathBuf) {
        let n = NEXT.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let dir = std::env::temp_dir().join(format!("skrib-styles-{}-{n}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        let svc = ExportStylesService::open_at(dir.join("styles.toml")).unwrap();
        let vm = ExportStylesViewModel::new(svc);
        let base = vm.builtin_presets().into_iter().next().expect("a built-in");
        let mine = vm.duplicate(&base, "copy").expect("duplicate");
        (vm, mine.id, dir)
    }

    /// The editor builds and lays out with every row, including the pagination group. A
    /// `FormLayout` row that fails to build takes the whole settings page with it, and the
    /// page is only reachable two clicks deep.
    #[test]
    fn the_style_editor_builds_and_lays_out() {
        let (vm, id, dir) = vm_with_a_user_style();
        let mut tree = WidgetTree::new();
        let w = tree.add_boxed(Box::new(StyleEditor::new(vm, Signal::new(Some(id)))));
        tree.layout(SizeProposal::exact(600.0, 900.0));
        assert!(tree.bounds(w).height > 0.0, "the editor must lay out");
        let _ = std::fs::remove_dir_all(&dir);
    }

    /// Every new switch must actually write through to the saved style.
    ///
    /// This is the failure this pane is prone to: the toggle is bound to a plain `Signal`,
    /// and forgetting its `bind_field` call still compiles, still renders a switch that
    /// moves, and silently discards the writer's choice. The pane's own signals are locals
    /// inside `build`, so the harness re-binds the same closures through the same
    /// `bind_field` in a throwaway widget — which is exactly the machinery under test.
    #[test]
    fn the_pagination_switches_write_through_to_the_saved_style() {
        let (vm, id, dir) = vm_with_a_user_style();

        // Seeded from the built-in: pagination on, word count off.
        let before = vm.user_preset(&id).expect("the user style");
        assert!(before.chapter_starts_page);
        assert!(!before.title_page_word_count);

        let signals: Vec<(&str, Signal<bool>)> = FIELDS
            .iter()
            .map(|(name, _)| (*name, Signal::new(field_of(&before, name))))
            .collect();

        let mut tree = WidgetTree::new();
        tree.add_boxed(Box::new(Binder {
            vm: vm.clone(),
            id: id.clone(),
            signals: signals.clone(),
        }));
        tree.layout(SizeProposal::exact(100.0, 100.0));

        for (name, sig) in &signals {
            sig.set(!sig.get());
            let after = vm.user_preset(&id).expect("the user style");
            assert_eq!(
                field_of(&after, name),
                !field_of(&before, name),
                "flipping {name:?} must reach the saved style"
            );
        }
        let _ = std::fs::remove_dir_all(&dir);
    }

    /// The epigraph placement is a two-way choice rather than a switch, so it rides its own
    /// signal type — and it still has to reach the saved style.
    #[test]
    fn the_epigraph_placement_writes_through_to_the_saved_style() {
        let (vm, id, dir) = vm_with_a_user_style();
        assert_eq!(
            vm.user_preset(&id).unwrap().epigraph_placement,
            EpigraphPlacement::AfterHeading,
            "the convention is what a new style starts from"
        );

        let idx = Signal::new(0usize);
        let mut tree = WidgetTree::new();
        tree.add_boxed(Box::new(PlacementBinder {
            vm: vm.clone(),
            id: id.clone(),
            idx: idx.clone(),
        }));
        tree.layout(SizeProposal::exact(100.0, 100.0));

        idx.set(1);
        assert_eq!(
            vm.user_preset(&id).unwrap().epigraph_placement,
            EpigraphPlacement::BeforeHeading
        );
        idx.set(0);
        assert_eq!(
            vm.user_preset(&id).unwrap().epigraph_placement,
            EpigraphPlacement::AfterHeading
        );
        let _ = std::fs::remove_dir_all(&dir);
    }

    struct PlacementBinder {
        vm: ExportStylesViewModel,
        id: String,
        idx: Signal<usize>,
    }

    impl std::fmt::Debug for PlacementBinder {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("PlacementBinder").finish()
        }
    }

    impl Widget for PlacementBinder {
        fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
            bind_field(ctx, &self.vm, &self.id, &self.idx, |p, v| {
                p.epigraph_placement = match v {
                    1 => EpigraphPlacement::BeforeHeading,
                    _ => EpigraphPlacement::AfterHeading,
                };
            });
            Vec::new()
        }

        fn layout_response(&self, p: SizeProposal, _ctx: &LayoutContext) -> LayoutResponse {
            p.resolve(0.0, 0.0).into()
        }
    }

    /// The footnote-numbering restart is a three-way choice (unlike the two-way epigraph
    /// placement above), so it rides an `Option<FootnoteNumbering>` like the pane's own
    /// `ComboBox` — and it still has to reach the saved style.
    #[test]
    fn the_footnote_numbering_writes_through_to_the_saved_style() {
        let (vm, id, dir) = vm_with_a_user_style();
        assert_eq!(
            vm.user_preset(&id).unwrap().footnote_numbering,
            FootnoteNumbering::Continuous,
            "the built-in's own convention is what a new style starts from"
        );

        let sig = Signal::new(Some(FootnoteNumbering::Continuous));
        let mut tree = WidgetTree::new();
        tree.add_boxed(Box::new(FootnoteNumberingBinder {
            vm: vm.clone(),
            id: id.clone(),
            sig: sig.clone(),
        }));
        tree.layout(SizeProposal::exact(100.0, 100.0));

        sig.set(Some(FootnoteNumbering::PerChapter));
        assert_eq!(
            vm.user_preset(&id).unwrap().footnote_numbering,
            FootnoteNumbering::PerChapter
        );
        sig.set(Some(FootnoteNumbering::PerBook));
        assert_eq!(
            vm.user_preset(&id).unwrap().footnote_numbering,
            FootnoteNumbering::PerBook
        );
        let _ = std::fs::remove_dir_all(&dir);
    }

    struct FootnoteNumberingBinder {
        vm: ExportStylesViewModel,
        id: String,
        sig: Signal<Option<FootnoteNumbering>>,
    }

    impl std::fmt::Debug for FootnoteNumberingBinder {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("FootnoteNumberingBinder").finish()
        }
    }

    impl Widget for FootnoteNumberingBinder {
        fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
            bind_field(ctx, &self.vm, &self.id, &self.sig, |p, v| {
                if let Some(v) = v {
                    p.footnote_numbering = v;
                }
            });
            Vec::new()
        }

        fn layout_response(&self, p: SizeProposal, _ctx: &LayoutContext) -> LayoutResponse {
            p.resolve(0.0, 0.0).into()
        }
    }

    /// Every switch the pane added, with the write it performs. Adding a knob to the pane
    /// without adding it here leaves it untested — but it leaves it *visibly* untested,
    /// next to its six neighbours.
    #[allow(clippy::type_complexity)]
    const FIELDS: &[(&str, fn(&mut Preset, bool))] = &[
        ("book", |p, v| p.book_starts_page = v),
        ("part", |p, v| p.part_starts_page = v),
        ("chapter", |p, v| p.chapter_starts_page = v),
        ("paratext", |p, v| p.paratext_starts_page = v),
        ("title_page", |p, v| p.book_title_page = v),
        ("word_count", |p, v| p.title_page_word_count = v),
        ("include_paratexts", |p, v| p.include_paratexts = v),
        ("include_footnotes", |p, v| p.include_footnotes = v),
    ];

    /// A widget whose only job is to run `bind_field` over `FIELDS` — `bind_field` needs a
    /// `BuildContext`, and a `BuildContext` only exists inside a `build`.
    struct Binder {
        vm: ExportStylesViewModel,
        id: String,
        signals: Vec<(&'static str, Signal<bool>)>,
    }

    impl std::fmt::Debug for Binder {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("Binder").finish()
        }
    }

    impl Widget for Binder {
        fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
            for ((_, sig), (_, apply)) in self.signals.iter().zip(FIELDS.iter()) {
                bind_field(ctx, &self.vm, &self.id, sig, *apply);
            }
            Vec::new()
        }

        fn layout_response(&self, p: SizeProposal, _ctx: &LayoutContext) -> LayoutResponse {
            p.resolve(0.0, 0.0).into()
        }
    }

    fn field_of(p: &Preset, name: &str) -> bool {
        match name {
            "book" => p.book_starts_page,
            "part" => p.part_starts_page,
            "chapter" => p.chapter_starts_page,
            "paratext" => p.paratext_starts_page,
            "title_page" => p.book_title_page,
            "word_count" => p.title_page_word_count,
            "include_paratexts" => p.include_paratexts,
            "include_footnotes" => p.include_footnotes,
            other => panic!("unknown field {other}"),
        }
    }
}
