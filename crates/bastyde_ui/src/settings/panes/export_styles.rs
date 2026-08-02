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
//! Like `settings_dictionaries`, it needs generic-closure widgets (`ListView`) the `bati!` DSL
//! can't express, so it is a chained-builder module.

use bastyde::core::binding::BindingLevel;
use bastyde::core::widget::WidgetPlacement;
use bastyde::data::ListModel;
use bastyde::prelude::*;
use bastyde::widgets::{
    Badge, Button, ButtonVariant, ComboBox, FixedSize, FontPicker, FormLayout, HStack, ListView,
    MaxSize, Padding, Segment, SegmentedControl, Spacer, StandardListItem, TextInput, TextWidget,
    Toast, Toggle, VStack,
};

use skribisto_compiler::{
    DigitStyle, DirectionMode, ExportFormat, HeadingLanguage, HeadingScheme, LineSpacing, PageSize,
    Preset, SceneBreak,
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
    let req = FileDialogRequest::pick_file()
        .title(tr!(settings_styles_import()))
        .add_filter(tr!(settings_styles_json_filter()).resolve_now(), &["json"]);
    let _ = ctx.pick_file(req, move |res, ectx| {
        if let FileDialogResult::File(Some(path)) = res {
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
    let req = FileDialogRequest::save_file()
        .title(tr!(settings_styles_export()))
        .default_file_name(default_name)
        .add_filter(tr!(settings_styles_json_filter()).resolve_now(), &["json"]);
    let _ = ctx.save_file(req, move |res, ectx| {
        if let FileDialogResult::Saved(Some(path)) = res {
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
