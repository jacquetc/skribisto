// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Settings ▸ Work ▸ **Personal dictionary** — the per-project word-list manager.
//!
//! A description, a prominent add-a-word row (leading `+`, filled accent button), a filter +
//! live count + Import…/Export… toolbar, and a bordered word list — each row the word as an
//! inline field (rename on Enter) with an X remove button — over the [`UserDictionaryViewModel`].
//! The list binds the view-model's reactive
//! [`DictWordListModel`](crate::models::DictWordListModel) through a `SortFilterListModel` search
//! projection, so a word added here or from the editor's "Add to dictionary" appears live either way.
//!
//! Like `settings_dictionaries`, it needs generic-closure widgets (`ListView`)
//! the `bati!` DSL can't express, so it is a chained-builder module.

use bastyde::core::styles::TextInputVariant;
use bastyde::data::SortFilterListModel;
use bastyde::prelude::*;
use bastyde::res;
use bastyde::tokens::{BorderRole, SurfaceRole};
use bastyde::widgets::{
    BuiltInIcons, Button, ButtonVariant, Center, Divider, Expand, HStack, IconButton, IconLocation,
    IconWidget, ListView, MaxSize, MinSize, Padding, Panel, SearchField, Spacer, Switcher,
    TextInput, TextWidget, Toast, VStack,
};

use crate::app_ids::HasWorkId;
use crate::models::DictWordRow;
use crate::toast_scope::ToastWorkExt;
use crate::view_models::UserDictionaryViewModel;

const WORD_COL: &str = "word";

/// The filter field caps here so it reads as a compact search box, not a second full-width field.
const FILTER_FIELD_MAX_WIDTH: f32 = 260.0;

/// The list box's minimum height — enough to show several rows and scroll internally rather than
/// collapsing inside the settings pane's own scroll.
const LIST_MIN_HEIGHT: f32 = 320.0;

fn add_glyph() -> IconWidget {
    (BuiltInIcons::defaults().add)().icon_size(15.0)
}
fn import_glyph() -> IconWidget {
    IconWidget::from_svg_icon(res!("assets/icons/settings/import.svg")).icon_size(15.0)
}
fn export_glyph() -> IconWidget {
    IconWidget::from_svg_icon(res!("assets/icons/settings/export.svg")).icon_size(15.0)
}

/// The whole personal-dictionary pane body. The caller wraps it in `pane_frame`.
///
/// Top to bottom: a one-line description, the prominent add-a-word row, a filter + live count +
/// Import/Export toolbar, and a bordered word list. The list binds the reactive
/// [`DictWordListModel`](crate::models::DictWordListModel) through a `SortFilterListModel` search
/// projection, so a word added here or from the editor's "Add to dictionary" appears live either way.
pub fn user_dictionary_pane(ctx: &mut BuildContext, vm: &UserDictionaryViewModel) -> impl Widget {
    // Filter box → a filter projection over the reactive words model.
    let filtered = SortFilterListModel::new(vm.list_model()).with_predicate(WORD_COL, |text| {
        let needle = text.trim().to_lowercase();
        Box::new(move |row: &DictWordRow| {
            needle.is_empty() || row.word.to_lowercase().contains(&needle)
        })
    });
    let query = Signal::new(String::new());
    {
        // Push imperatively — `filters_signal` would `observe` a derived signal.
        let filter_view = filtered.clone();
        ctx.effect(&query, move |q| filter_view.set_filter(WORD_COL, q));
    }

    let list_vm = vm.clone();
    let list = ListView::from_source(filtered, move |i, row: &DictWordRow, _selected| {
        Box::new(word_row(&list_vm, row, i))
    })
    .auto_item_height(46.0);

    // The list lives in a bordered card; with no words at all it shows a centered hint instead —
    // an empty bordered box reads as "broken", a hint reads as "nothing here yet".
    let empty_idx = {
        let vm = vm.clone();
        vm.changed_signal().map(move |_| usize::from(vm.len() == 0))
    };
    let list_card = Panel::new()
        .background(SurfaceRole::Content)
        .border_color(BorderRole::Default)
        .border_width(1.0)
        .corner_radius(8.0)
        .padding(0.0)
        .child(
            // MinSize wraps the whole Switcher (not just the list child): a `Switcher` reports
            // its active child's size, and a virtualised `ListView` given unbounded height in the
            // settings pane's own scroll reports ~nothing — so the floor must be imposed on the
            // card, giving the list a bounded height to fill.
            MinSize::new(0.0, LIST_MIN_HEIGHT).child(
                Switcher::new(empty_idx)
                    .child(Expand::vertical().child(list))
                    .child(empty_state()),
            ),
        );

    VStack::new()
        .spacing(16.0)
        .child(TextWidget::new(tr!(settings_user_dict_desc())).color(TextRole::Secondary))
        .child(add_row(vm))
        .child(toolbar_row(vm, query))
        // Fill the content width — a `Panel` sizes to its child, so without this the card would
        // shrink to its widest row rather than span the pane like the rows above it.
        .child(Expand::horizontal().child(list_card))
}

/// The prominent add-a-word row: a field with a leading `+`, and a filled accent button. Enter in
/// the field or the button both add (a blank / duplicate is a no-op) and clear the field, toasting
/// on a real add.
fn add_row(vm: &UserDictionaryViewModel) -> impl Widget {
    let text = Signal::new(String::new());

    let commit = {
        let vm = vm.clone();
        let text = text.clone();
        move |ctx: &mut EventContext| {
            let word = text.get().trim().to_string();
            let ids = vm.add_word(&word);
            vm.added_toast(ctx, ids, Some(word));
            text.set(String::new());
        }
    };

    let field = {
        let commit = commit.clone();
        TextInput::new(text.clone())
            .leading_slot(add_glyph().color(TextRole::Secondary))
            .placeholder(tr!(settings_user_dict_add_placeholder()))
            .on_submit_fn(move |ctx| commit(ctx))
    };

    // Enabled only for a valid, non-duplicate word — recomputed as the text or the word set changes.
    let can_add = {
        let vm = vm.clone();
        text.zip(&vm.changed_signal())
            .map(move |(t, _)| vm.can_add(t))
    };
    let add_btn = {
        Button::new(tr!(settings_user_dict_add()))
            .variant(ButtonVariant::Filled)
            .enabled(can_add)
            .on_activate_fn(move |ctx| commit(ctx))
    };

    HStack::new()
        .spacing(10.0)
        .child(Expand::horizontal().child(field))
        .child(add_btn)
}

/// The filter + live count on the left, Import… / Export… (with glyphs) pushed to the right.
fn toolbar_row(vm: &UserDictionaryViewModel, query: Signal<String>) -> impl Widget {
    let search = MaxSize::width(FILTER_FIELD_MAX_WIDTH)
        .child(SearchField::new(query).placeholder(tr!(settings_user_dict_search())));

    let count = {
        let vm = vm.clone();
        let text = vm
            .changed_signal()
            .map(move |_| tr!(settings_user_dict_count(count = vm.len() as i64)).resolve_now());
        TextWidget::new(lit!(""))
            .text(text)
            .style(TextStyleRole::Small)
            .color(TextRole::Secondary)
    };

    let import_btn = {
        let vm = vm.clone();
        Button::new(tr!(settings_user_dict_import()))
            .variant(ButtonVariant::Plain)
            .icon(import_glyph(), IconLocation::Leading)
            .tooltip(tr!(settings_user_dict_import_tip()))
            .on_activate_fn(move |ctx| present_import(ctx, vm.clone()))
    };
    let export_btn = {
        let vm = vm.clone();
        Button::new(tr!(settings_user_dict_export()))
            .variant(ButtonVariant::Plain)
            .icon(export_glyph(), IconLocation::Leading)
            .tooltip(tr!(settings_user_dict_export_tip()))
            .on_activate_fn(move |ctx| present_export(ctx, vm.clone()))
    };

    HStack::new()
        .spacing(10.0)
        .child(search)
        .child(count)
        .child(Spacer::new())
        .child(import_btn)
        .child(export_btn)
}

/// One word row: the word as an inline field (rename on Enter — click it and type), with an X
/// remove button. A hairline above every row but the first gives the list its structure.
fn word_row(vm: &UserDictionaryViewModel, row: &DictWordRow, index: usize) -> impl Widget {
    let id = row.id;
    let value = Signal::new(row.word.clone());

    let field = {
        let vm = vm.clone();
        let value = value.clone();
        // `Bare` = no field chrome, so the word reads as plain text until clicked — a rename in
        // place, without every row looking like a form input.
        TextInput::new(value.clone())
            .variant(TextInputVariant::Bare)
            .on_submit_fn(move |_ctx| {
                let _ = vm.rename(id, &value.get());
            })
    };
    let remove = {
        let vm = vm.clone();
        IconButton::clear()
            .embedded()
            .tooltip(tr!(settings_user_dict_remove()))
            .on_activate_fn(move |_ctx| vm.remove(id))
    };

    let body = Padding::symmetric(6.0, 12.0).child(
        HStack::new()
            .spacing(8.0)
            .child(Expand::horizontal().child(field))
            .child(remove),
    );

    let mut col = VStack::new().spacing(0.0);
    if index > 0 {
        col = col.child(Divider::new());
    }
    col.child(body)
}

/// The centered "no words yet" hint shown inside the list card when the dictionary is empty.
fn empty_state() -> impl Widget {
    Center::new().child(
        TextWidget::new(tr!(settings_user_dict_empty()))
            .style(TextStyleRole::Small)
            .color(TextRole::Secondary),
    )
}

/// Import a `.txt` word list (merge). Toasts the count on success, the error on
/// failure. The added-toast (with Undo) is folded into the success toast here.
fn present_import(ctx: &mut EventContext, vm: UserDictionaryViewModel) {
    let req = FileDialogRequest::pick_file()
        .title(tr!(settings_user_dict_import()))
        .add_filter(tr!(settings_user_dict_txt_filter()).resolve_now(), &["txt"]);
    let _ = ctx.pick_file(req, move |res, ectx| {
        if let FileDialogResult::File(Some(path)) = res {
            match vm.import_from(&path) {
                Ok(summary) => {
                    ectx.show_toast(
                        Toast::info(tr!(settings_user_dict_imported(
                            count = summary.added as i64,
                            duplicates = summary.duplicates as i64
                        )))
                        .auto_dismiss_after(std::time::Duration::from_secs(5))
                        // Work-scoped: this project's own personal dictionary.
                        .target_work(vm.work_id()),
                    );
                }
                Err(e) => {
                    ectx.show_toast(
                        Toast::error(tr!(settings_user_dict_import_failed(
                            error = format!("{e:#}")
                        )))
                        .auto_dismiss_after(std::time::Duration::from_secs(6))
                        .target_work(vm.work_id()),
                    );
                }
            }
        }
    });
}

/// Export the word list to a `.txt` file (one word per line), forcing the `.txt`
/// extension.
fn present_export(ctx: &mut EventContext, vm: UserDictionaryViewModel) {
    let req = FileDialogRequest::save_file()
        .title(tr!(settings_user_dict_export()))
        .default_file_name("dictionary.txt".to_string())
        .add_filter(tr!(settings_user_dict_txt_filter()).resolve_now(), &["txt"]);
    let _ = ctx.save_file(req, move |res, ectx| {
        if let FileDialogResult::Saved(Some(path)) = res {
            let mut target = path.clone();
            if target.extension().and_then(|e| e.to_str()) != Some("txt") {
                target.set_extension("txt");
            }
            match vm.export_to(&target) {
                Ok(count) => {
                    ectx.show_toast(
                        Toast::info(tr!(settings_user_dict_exported(count = count as i64)))
                            .body(lit!(target.to_string_lossy().into_owned()))
                            .auto_dismiss_after(std::time::Duration::from_secs(4))
                            .target_work(vm.work_id()),
                    );
                }
                Err(e) => {
                    ectx.show_toast(
                        Toast::error(tr!(settings_user_dict_export_failed(
                            error = format!("{e:#}")
                        )))
                        .auto_dismiss_after(std::time::Duration::from_secs(6))
                        .target_work(vm.work_id()),
                    );
                }
            }
        }
    });
}
