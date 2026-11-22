// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Settings ▸ Work ▸ **Resources** — the project's resources that no item carries
//! ([`UnattachedResourcesViewModel`] says why such a page exists).
//!
//! Shaped like the Templates page: a description, then the size the project's stored
//! files take (with, for a large single-file project, the advice to keep it as a folder
//! and the button that does it), a toolbar acting on the selection, and a bordered list.
//! Every action is also on each row's menu, so one resource needs no selection first.
//!
//! The list is a `ListView` bound to a `ListModel` the view-model's rows are reconciled
//! into on every change, several rows selectable at once (Ctrl+click, Shift+click).
//! Plain builders under the shell, like the Templates page beside it.

use teksilo::core::overlay::OverlayPlacement;
use teksilo::data::{ListModel, SelectionMode, SelectionModel};
use teksilo::prelude::*;
use teksilo::tokens::{BorderRole, SurfaceRole, TextRole};
use teksilo::widgets::{
    ActivateOn, Button, ButtonVariant, Expand, HStack, IconButton, IconWidget, ListView, MenuItem,
    MenuList, MinSize, Padding, Panel, PopoverIconButton, ScrollBarMode, Spacer, Switcher,
    TextWidget, VStack,
};

use crate::models::ResourceRow;
use crate::resources::ResourcesViewModel;
use crate::resources::row::{ResourceIcon, detail_line, row_name, title_of};
use crate::resources::unattached::{SAVE_AS_FOLDER, UnattachedResourcesViewModel};
use crate::sessions::WorkSession;
use crate::settings::fields::group;

/// The page over `session`'s project.
pub fn work_resources_pane(ctx: &mut BuildContext, session: &WorkSession) -> impl Widget + use<> {
    let resources = session.resources.clone();
    let vm = ResourcesViewModel::new(
        resources.app_ctx(),
        session.ids.clone(),
        resources.clone(),
        // The page follows no item: what it acts on is the rows the writer picks.
        Signal::new(None),
        session.single_work_info.shape(),
        session.single_work.unique_id(),
    );
    let page = UnattachedResourcesViewModel::new(session.ids.work_id.clone(), vm);
    work_resources_body(ctx, &page)
}

/// The page's body over `vm`: what [`work_resources_pane`] frames, and what a test builds.
pub(crate) fn work_resources_body(
    ctx: &mut BuildContext,
    vm: &UnattachedResourcesViewModel,
) -> impl Widget + use<> {
    let model = ListModel::from_vec(vm.rows());
    let selection = SelectionModel::new(SelectionMode::Multi);
    {
        let vm = vm.clone();
        let model = model.clone();
        let selection = selection.clone();
        ctx.effect(&vm.version(), move |_| {
            let rows = vm.rows();
            // A row that left the list (attached, deleted) must not stay selected under
            // the index it had.
            selection.clear();
            model.reconcile_by_key(rows, |r| r.id);
        });
    }

    let list = {
        let row_vm = vm.clone();
        let open_vm = vm.resources().clone();
        let open_model = model.clone();
        ListView::new(model.clone(), move |_i, row: &ResourceRow, _sel| {
            Box::new(unattached_row(row, &row_vm)) as Box<dyn Widget>
        })
        .selection(selection.clone())
        .auto_item_height(52.0)
        .scroll_bar_style(ScrollBarMode::Overlay)
        .activate_on(ActivateOn::DoubleClick)
        .type_ahead_label(title_of)
        .on_activate(move |idx, c| {
            if let Some(row) = open_model.with_item(idx, |r| r.clone()) {
                open_vm.open(c, &row);
            }
        })
    };

    let empty_idx = {
        let vm = vm.clone();
        vm.version().map(move |_| usize::from(vm.rows().is_empty()))
    };
    let list_card = Panel::new()
        .background(SurfaceRole::Content)
        .border_color(BorderRole::Default)
        .border_width(1.0)
        .corner_radius(8.0)
        .padding(0.0)
        .child(
            MinSize::new(0.0, crate::settings::fields::LIST_MIN_HEIGHT).child(
                Switcher::new(empty_idx)
                    .child(Expand::vertical().child(list))
                    .child(empty_state()),
            ),
        );

    let size = size_line(ctx, vm);
    VStack::new()
        .spacing(16.0)
        .child(TextWidget::new(tr!(settings_resources_description())).color(TextRole::Secondary))
        .child(group(tr!(settings_resources_group_storage())))
        .child(size)
        .child(group(tr!(settings_resources_group_unattached())))
        .child(toolbar(vm, model, selection))
        .child(crate::settings::fields::list_box(
            Expand::horizontal().child(list_card),
        ))
}

/// "Files stored in this project: 120 MB", and for a single-file project past the notice
/// size the advice to keep it as a folder, with the button that does it.
fn size_line(ctx: &mut BuildContext, vm: &UnattachedResourcesViewModel) -> impl Widget + use<> {
    let total = {
        let vm = vm.clone();
        vm.version().map(move |_| {
            tr!(settings_resources_stored(
                size = crate::shared::units::byte_size(vm.stored_bytes())
            ))
            .resolve_now()
        })
    };
    // A plain signal kept in step, not a derived one: the show/hide gate observes it,
    // and observing a derived signal panics.
    let advice_shown = Signal::new(vm.suggests_folder());
    {
        let (vm, shown) = (vm.clone(), advice_shown.clone());
        ctx.effect(&vm.version(), move |_| {
            shown.set_if_changed(vm.suggests_folder());
        });
    }
    {
        let (vm, shown) = (vm.clone(), advice_shown.clone());
        ctx.effect(&vm.shape(), move |_| {
            shown.set_if_changed(vm.suggests_folder());
        });
    }
    let advice = HStack::new()
        .spacing(10.0)
        .child(Expand::horizontal().child(
            TextWidget::new(tr!(settings_resources_folder_advice())).color(TextRole::Secondary),
        ))
        .child(
            Button::new(tr!(resources_folder_switch()))
                .variant(ButtonVariant::Plain)
                .on_activate_fn(|c| c.send_intent(Intent::new(SAVE_AS_FOLDER))),
        );
    VStack::new()
        .spacing(6.0)
        .child(TextWidget::new(lit!("")).text(total))
        .child(crate::tabs::shared::editor::VisibleWhen::new(
            advice_shown,
            advice,
        ))
}

/// The count, and the three actions on the selected rows.
fn toolbar(
    vm: &UnattachedResourcesViewModel,
    model: ListModel<ResourceRow>,
    selection: SelectionModel,
) -> impl Widget + use<> {
    let count = {
        let vm = vm.clone();
        vm.version().map(move |_| {
            tr!(settings_resources_count(count = vm.rows().len() as i64)).resolve_now()
        })
    };
    let any = selection.selection_signal().map(|s| !s.is_empty());
    let selected = move || -> Vec<ResourceRow> {
        selection
            .selected_indices()
            .into_iter()
            .filter_map(|i| model.with_item(i, |r| r.clone()))
            .collect()
    };
    let attach = {
        let (vm, selected) = (vm.clone(), selected.clone());
        Button::new(tr!(resources_attach_to_items()))
            .variant(ButtonVariant::Plain)
            .enabled(any.clone())
            .on_activate_fn(move |c| vm.pick_items_and_attach(c, selected()))
    };
    let export = {
        let (vm, selected) = (vm.clone(), selected.clone());
        Button::new(tr!(resources_export_copies()))
            .variant(ButtonVariant::Plain)
            .enabled(any.clone())
            .on_activate_fn(move |c| vm.export(c, selected()))
    };
    let delete = {
        let vm = vm.clone();
        Button::new(tr!(resources_remove()))
            .variant(ButtonVariant::Plain)
            .enabled(any)
            .on_activate_fn(move |c| vm.delete(c, &selected()))
    };
    HStack::new()
        .spacing(10.0)
        .child(
            TextWidget::new(lit!(""))
                .text(count)
                .color(TextRole::Secondary),
        )
        .child(Expand::horizontal().child(Spacer::new()))
        .child(attach)
        .child(export)
        .child(delete)
}

/// Shown instead of the list when every resource is on an item.
fn empty_state() -> impl Widget {
    teksilo::widgets::Center::new().child(
        Padding::symmetric(24.0, 24.0)
            .child(TextWidget::new(tr!(settings_resources_empty())).color(TextRole::Secondary)),
    )
}

/// One resource: its type, its title, its size or address, and its menu.
fn unattached_row(row: &ResourceRow, vm: &UnattachedResourcesViewModel) -> impl Widget + use<> {
    let actions = tr!(resources_actions());
    let more = PopoverIconButton::new(
        IconButton::new(IconWidget::chevron_down(12.0)).tooltip(actions.clone()),
    )
    .bare()
    .show_disclosure_caret(false)
    .content(row_menu(row, vm))
    .placement(OverlayPlacement::BelowPreferred)
    .access_label(actions);
    let text = VStack::new()
        .spacing(2.0)
        .child(
            TextWidget::new(lit!(title_of(row)))
                .style(TextStyleRole::Small)
                .single_line()
                .overflow(TextOverflow::Ellipsis(EllipsisMode::Trailing)),
        )
        .child(
            TextWidget::new(lit!(detail_line(row)))
                .style(TextStyleRole::Tiny)
                .color(TextRole::Secondary)
                .single_line()
                .overflow(TextOverflow::Ellipsis(EllipsisMode::Middle)),
        );
    let mut line = HStack::new()
        .spacing(8.0)
        .child(ResourceIcon::new(row))
        .child(Expand::horizontal().child(text));
    if row.missing {
        line = line.child(
            TextWidget::new(tr!(resources_not_found()))
                .style(TextStyleRole::Tiny)
                .color(TextRole::Warning)
                .single_line(),
        );
    }
    Padding::symmetric(6.0, 10.0)
        .child(line.child(more))
        .access_label(row_name(row))
}

/// A row's menu: open it, show it in its folder, and the page's three actions on it alone.
fn row_menu(row: &ResourceRow, vm: &UnattachedResourcesViewModel) -> MenuList {
    let mut menu = MenuList::new();
    {
        let (r, row) = (vm.resources().clone(), row.clone());
        menu = menu
            .item(MenuItem::new(tr!(resources_open())).on_activate_fn(move |c| r.open(c, &row)));
    }
    if !row.is_web_link() {
        let (r, row) = (vm.resources().clone(), row.clone());
        menu = menu.item(
            MenuItem::new(tr!(resources_show_in_folder()))
                .enabled(!row.missing)
                .on_activate_fn(move |c| r.reveal(c, &row)),
        );
    }
    menu = menu.separator();
    {
        let (vm, row) = (vm.clone(), row.clone());
        menu = menu.item(
            MenuItem::new(tr!(resources_attach_to_items()))
                .on_activate_fn(move |c| vm.pick_items_and_attach(c, vec![row.clone()])),
        );
    }
    {
        let (vm, row) = (vm.clone(), row.clone());
        menu = menu.item(
            MenuItem::new(tr!(resources_export_copy()))
                .on_activate_fn(move |c| vm.export(c, vec![row.clone()])),
        );
    }
    let (vm, row) = (vm.clone(), row.clone());
    menu.item(
        MenuItem::new(tr!(resources_remove()))
            .on_activate_fn(move |c| vm.delete(c, std::slice::from_ref(&row))),
    )
}
