// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Settings ▸ Spelling ▸ **Dictionaries** — the dictionary management pane.
//!
//! Two tabs:
//! - **Installed** — the dictionaries found on this machine (our downloads + system copies).
//!   Downloaded ones are removable; system ones are read-only (a distro package isn't ours to
//!   delete), badged accordingly.
//! - **Get more** — the catalogue, searchable, each row offering "View licence" and a Download
//!   (gated behind the licence-accept modal). A row already installed says so instead.
//!
//! Needs generic-closure widgets (`ListView`, `TabWidget`) the `bati!` DSL can't express, so —
//! like `panes::backup` — it is a chained-builder module rather than a `bati!` tree.

use bastyde::data::{ListModel, SortFilterListModel};
use bastyde::prelude::*;
use bastyde::widgets::{
    Badge, Button, ButtonVariant, HStack, ListView, MinSize, Padding, SearchField, Spacer,
    StandardListItem, Switcher, TabWidget, TextWidget, VStack,
};

use crate::models::{DictOrigin, InstalledDictionaryRow};
use crate::panels::license;
use crate::spellcheck::dictionary_registry;
use crate::view_models::DictionariesViewModel;

/// The whole Dictionaries pane body (a two-tab widget). The caller wraps it in `pane_frame`.
pub fn dictionaries_pane(ctx: &mut BuildContext, vm: &DictionariesViewModel) -> impl Widget {
    let selected = Signal::new(None);
    TabWidget::new(selected)
        .tab(tr!(settings_dict_tab_installed()), installed_tab(vm))
        .tab(tr!(settings_dict_tab_get_more()), get_more_tab(ctx, vm))
}

// ── Installed tab ──

fn installed_tab(vm: &DictionariesViewModel) -> impl Widget {
    let model = vm.installed_model().list_model();
    let list_vm = vm.clone();
    let list = ListView::new(model, move |_i, row: &InstalledDictionaryRow, selected| {
        Box::new(installed_row(&list_vm, row, selected))
    })
    .auto_item_height(52.0);

    // A trailing "Add dictionary…" button opens the sideload modal (name + code + local
    // `.aff`/`.dic`). Cloning the view-model into the handler lets it present over this window.
    let add_vm = vm.clone();
    let toolbar = HStack::new().spacing(8.0).child(Spacer::new()).child(
        Button::new(tr!(dict_add_button()))
            .variant(ButtonVariant::Tinted)
            .on_activate_fn(move |c| {
                crate::spellcheck::add_dictionary_panel::present_add_dictionary(c, add_vm.clone())
            }),
    );

    // A bounded height so the virtualised list actually shows rows and scrolls internally —
    // an `Expand::vertical` here would collapse to zero inside the settings pane's scroll.
    VStack::new()
        .spacing(0.0)
        .child(Padding::symmetric(8.0, 8.0).child(toolbar))
        .child(Padding::symmetric(4.0, 0.0).child(MinSize::new(0.0, 340.0).child(list)))
}

fn installed_row(
    vm: &DictionariesViewModel,
    row: &InstalledDictionaryRow,
    selected: bool,
) -> impl Widget {
    // A trailing slot: a Remove button for our downloads, a read-only badge for system copies.
    let trailing: Box<dyn Widget> = match row.origin {
        DictOrigin::Downloaded => {
            let vm = vm.clone();
            let id = row.id.clone();
            Box::new(
                Button::new(tr!(dict_remove()))
                    .variant(ButtonVariant::Plain)
                    .on_activate_fn(move |c| vm.remove(&id, c)),
            )
        }
        DictOrigin::System => {
            let label = if row.encoding.is_some() && !row.matched {
                tr!(dict_unusable_badge())
            } else {
                tr!(dict_system_badge())
            };
            Box::new(Badge::new(label))
        }
    };

    let subtitle = if row.also_system {
        tr!(dict_system_badge())
    } else {
        lit!(row.id.clone())
    };

    StandardListItem::new(lit!(row.display_name.clone()))
        .subtitle(subtitle)
        .trailing_slot_boxed(trailing)
        .selected(selected)
}

// ── Get more tab ──

#[derive(Clone)]
struct CatalogRow {
    id: String,
    display_name: String,
    size: String,
    license_name: String,
}

const NAME_COL: &str = "name";

fn get_more_tab(ctx: &mut BuildContext, vm: &DictionariesViewModel) -> impl Widget {
    let rows: Vec<CatalogRow> = dictionary_registry::entries()
        .iter()
        .map(|e| CatalogRow {
            id: e.id.clone(),
            display_name: e.display_name.clone(),
            size: human_size(e.approx_size_bytes),
            license_name: e.license_name.clone(),
        })
        .collect();
    let model = ListModel::from_vec(rows);
    let filtered = SortFilterListModel::new(model).with_predicate(NAME_COL, |text| {
        let needle = text.trim().to_lowercase();
        Box::new(move |row: &CatalogRow| {
            needle.is_empty()
                || row.display_name.to_lowercase().contains(&needle)
                || row.id.to_lowercase().contains(&needle)
        })
    });

    // The search box drives a plain mutable `query`; push it into the filter through an effect
    // (ties the observer to the widget's lifetime — `filters_signal` would panic on a *derived*
    // signal, which is why the push is imperative).
    let query = Signal::new(String::new());
    let search = SearchField::new(query.clone()).placeholder(tr!(dict_get_more_search()));
    {
        let filter_view = filtered.clone();
        ctx.effect(&query, move |q| filter_view.set_filter(NAME_COL, q));
    }

    let vm = vm.clone();
    let list = ListView::from_source(filtered, move |_i, row: &CatalogRow, selected| {
        Box::new(catalog_row(&vm, row, selected))
    })
    .auto_item_height(56.0);

    VStack::new()
        .spacing(0.0)
        .child(Padding::symmetric(8.0, 8.0).child(search))
        .child(Padding::symmetric(4.0, 0.0).child(MinSize::new(0.0, 340.0).child(list)))
}

fn catalog_row(vm: &DictionariesViewModel, row: &CatalogRow, selected: bool) -> impl Widget {
    let id = row.id.clone();

    // Row state → which trailing widget to show: 0 Download, 1 Downloading, 2 Installed.
    let state_idx = {
        let vm_i = vm.clone();
        let _vm_d = vm.clone();
        let id_i = id.clone();
        let id_d = id.clone();
        let installed = vm.changed_signal().map(move |_| vm_i.is_installed(&id_i));
        let downloading = vm.downloading_signal().map(move |s| s.contains(&id_d));
        installed.zip(&downloading).map(move |(inst, dl)| {
            if *dl {
                1usize
            } else if *inst {
                2usize
            } else {
                0usize
            }
        })
    };

    // Download button: gate behind the licence-accept modal, then download.
    let download_btn = {
        let vm = vm.clone();
        let id = id.clone();
        Button::new(tr!(dict_download_button()))
            .variant(ButtonVariant::Tinted)
            .on_activate_fn(move |c| {
                if vm.has_accepted(&id) {
                    vm.download(&id, c);
                } else {
                    let vm2 = vm.clone();
                    let id2 = id.clone();
                    license::present_license_accept(c, &id, move |c2| {
                        vm2.accept_license(&id2, &now_rfc3339());
                        vm2.download(&id2, c2);
                    });
                }
            })
    };
    let trailing = Switcher::new(state_idx)
        .child(download_btn)
        .child(TextWidget::new(tr!(dict_downloading())).color(TextRole::Secondary))
        .child(TextWidget::new(tr!(dict_installed_label())).color(TextRole::Secondary));

    // "View licence" + the state widget.
    let view_btn = {
        let id = id.clone();
        Button::new(tr!(dict_view_license()))
            .variant(ButtonVariant::Plain)
            .on_activate_fn(move |c| license::present_license_view(c, &id))
    };
    let actions = HStack::new().spacing(6.0).child(view_btn).child(trailing);

    StandardListItem::new(lit!(row.display_name.clone()))
        .subtitle(lit!(format!("{}  ·  {}", row.size, row.license_name)))
        .trailing_slot(actions)
        .selected(selected)
}

/// RFC3339 "now" for an acceptance record.
fn now_rfc3339() -> String {
    chrono::Utc::now().to_rfc3339()
}

/// A compact human size, e.g. `1.2 MB`.
fn human_size(bytes: u64) -> String {
    const KB: f64 = 1024.0;
    const MB: f64 = KB * 1024.0;
    let b = bytes as f64;
    if b >= MB {
        format!("{:.1} MB", b / MB)
    } else if b >= KB {
        format!("{:.0} KB", b / KB)
    } else {
        format!("{bytes} B")
    }
}
