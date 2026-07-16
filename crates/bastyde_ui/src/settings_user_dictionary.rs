// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Settings ▸ Work ▸ **Personal dictionary** — the per-project word-list manager.
//!
//! A toolbar (add a word · Import… · Export…), a search field, and a scrollable
//! list of the project's words — each row an editable field (rename on Enter)
//! with a Remove button — over the [`UserDictionaryViewModel`]. The list binds
//! the view-model's reactive [`DictWordListModel`](crate::models::DictWordListModel)
//! directly through a `SortFilterListModel` search projection, so a word added
//! here or from the editor's "Add to dictionary" appears live either way.
//!
//! Like `settings_dictionaries`, it needs generic-closure widgets (`ListView`)
//! the `bati!` DSL can't express, so it is a chained-builder module.

use bastyde::data::SortFilterListModel;
use bastyde::prelude::*;
use bastyde::widgets::{
    Button, ButtonVariant, Expand, HStack, ListView, MinSize, Padding, SearchField, Spacer,
    TextInput, TextWidget, Toast, VStack,
};

use crate::models::DictWordRow;
use crate::view_models::UserDictionaryViewModel;

const WORD_COL: &str = "word";

/// The whole personal-dictionary pane body. The caller wraps it in `pane_frame`.
pub fn user_dictionary_pane(ctx: &mut BuildContext, vm: &UserDictionaryViewModel) -> impl Widget {
    let toolbar = toolbar(ctx, vm);

    // Search box → a filter projection over the reactive words model.
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
    let search = SearchField::new(query).placeholder(tr!(settings_user_dict_search()));

    let list_vm = vm.clone();
    let list = ListView::from_source(filtered, move |_i, row: &DictWordRow, _selected| {
        Box::new(word_row(&list_vm, row))
    })
    .auto_item_height(40.0);

    // Empty-state hint (only when there are no words at all). A widget-prop bind
    // to a mapped signal is fine (only `ctx.effect`/`observe` panics on derived).
    let empty_vm = vm.clone();
    let empty_text = vm.changed_signal().map(move |_| {
        if empty_vm.len() == 0 {
            tr!(settings_user_dict_empty()).resolve_now()
        } else {
            String::new()
        }
    });

    VStack::new()
        .spacing(0.0)
        .child(Padding::symmetric(8.0, 10.0).child(
            TextWidget::new(tr!(settings_user_dict_desc())).color(TextRole::Secondary),
        ))
        .child(Padding::symmetric(8.0, 6.0).child(toolbar))
        .child(Padding::symmetric(8.0, 6.0).child(search))
        .child(
            Padding::symmetric(4.0, 0.0)
                .child(MinSize::new(0.0, 300.0).child(list)),
        )
        .child(
            Padding::symmetric(10.0, 8.0)
                .child(TextWidget::new(lit!("")).text(empty_text).color(TextRole::Secondary)),
        )
}

/// The add-a-word field + button, followed by Import… / Export….
fn toolbar(_ctx: &mut BuildContext, vm: &UserDictionaryViewModel) -> impl Widget {
    let text = Signal::new(String::new());

    // Add on Enter / on the button — both attempt the add (a blank / duplicate is
    // a no-op) and clear the field, toasting on a real add.
    let commit = {
        let vm = vm.clone();
        let text = text.clone();
        move |ctx: &mut EventContext| {
            let raw = text.get();
            let word = raw.trim().to_string();
            let ids = vm.add_word(&word);
            vm.added_toast(ctx, ids, Some(word));
            text.set(String::new());
        }
    };

    let field = {
        let commit = commit.clone();
        TextInput::new(text.clone())
            .placeholder(tr!(settings_user_dict_add_placeholder()))
            .on_submit_fn(move |ctx| commit(ctx))
    };

    // Enabled only for a valid, non-duplicate word — recomputed as the text or
    // the word set changes.
    let can_add = {
        let vm = vm.clone();
        text.zip(&vm.changed_signal())
            .map(move |(t, _)| vm.can_add(t))
    };
    let add_btn = {
        let mut commit = commit;
        Button::new(tr!(settings_user_dict_add()))
            .variant(ButtonVariant::Tinted)
            .enabled(can_add)
            .on_activate_fn(move |ctx| commit(ctx))
    };

    let import_btn = {
        let vm = vm.clone();
        Button::new(tr!(settings_user_dict_import()))
            .variant(ButtonVariant::Plain)
            .on_activate_fn(move |ctx| present_import(ctx, vm.clone()))
    };
    let export_btn = {
        let vm = vm.clone();
        Button::new(tr!(settings_user_dict_export()))
            .variant(ButtonVariant::Plain)
            .on_activate_fn(move |ctx| present_export(ctx, vm.clone()))
    };

    HStack::new()
        .spacing(8.0)
        .child(Expand::horizontal().child(field))
        .child(add_btn)
        .child(import_btn)
        .child(export_btn)
}

/// One word row: an editable field (rename on Enter) + a Remove button.
fn word_row(vm: &UserDictionaryViewModel, row: &DictWordRow) -> impl Widget {
    let id = row.id;
    let value = Signal::new(row.word.clone());

    let field = {
        let vm = vm.clone();
        let value = value.clone();
        TextInput::new(value.clone()).on_submit_fn(move |_ctx| {
            let _ = vm.rename(id, &value.get());
        })
    };
    let remove = {
        let vm = vm.clone();
        Button::new(tr!(settings_user_dict_remove()))
            .variant(ButtonVariant::Plain)
            .on_activate_fn(move |_ctx| vm.remove(id))
    };

    Padding::symmetric(4.0, 2.0).child(
        HStack::new()
            .spacing(8.0)
            .child(Expand::horizontal().child(field))
            .child(remove),
    )
}

/// Import a `.txt` word list (merge). Toasts the count on success, the error on
/// failure. The added-toast (with Undo) is folded into the success toast here.
fn present_import(ctx: &mut EventContext, vm: UserDictionaryViewModel) {
    let req = FileDialogRequest::pick_file()
        .title(tr!(settings_user_dict_import()))
        .add_filter(&tr!(settings_user_dict_txt_filter()).resolve_now(), &["txt"]);
    let _ = ctx.pick_file(req, move |res, ectx| {
        if let FileDialogResult::File(Some(path)) = res {
            match vm.import_from(&path) {
                Ok(summary) => {
                    ectx.show_toast(
                        Toast::info(tr!(settings_user_dict_imported(
                            count = summary.added as i64,
                            duplicates = summary.duplicates as i64
                        )))
                        .auto_dismiss_after(std::time::Duration::from_secs(5)),
                    );
                }
                Err(e) => {
                    ectx.show_toast(
                        Toast::error(tr!(settings_user_dict_import_failed(
                            error = format!("{e:#}")
                        )))
                        .auto_dismiss_after(std::time::Duration::from_secs(6)),
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
        .add_filter(&tr!(settings_user_dict_txt_filter()).resolve_now(), &["txt"]);
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
                            .auto_dismiss_after(std::time::Duration::from_secs(4)),
                    );
                }
                Err(e) => {
                    ectx.show_toast(
                        Toast::error(tr!(settings_user_dict_export_failed(
                            error = format!("{e:#}")
                        )))
                        .auto_dismiss_after(std::time::Duration::from_secs(6)),
                    );
                }
            }
        }
    });
}
