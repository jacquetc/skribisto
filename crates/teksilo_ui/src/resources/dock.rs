// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The **Resources** dock: the files and web links attached to the item in front of the
//! writer.
//!
//! Trailing rail, after Versions: the side that is about *whatever is in front of me
//! right now*, following the same `EditorsViewModel::active_item()` signal as the
//! Inspector, the Footnotes and the Versions docks.
//!
//! A row is its type at a glance (a picture's thumbnail, else an icon of the app's own),
//! its title, its size or its address, and a "Not found" badge for a file that is not
//! where the project says. A screen reader hears all of it in the row's name: the
//! badge's colour is not the only thing that says a file is missing. Double-click or
//! Enter opens it; its menu holds everything else.
//!
//! The dock is a drop target for files and web addresses from outside, and its rows
//! can be dragged into a text (see [`super::text_drop`]). The keyboard path to each is
//! here too: the header's two buttons add files and links, Ctrl+V on the list adds the
//! web addresses on the clipboard, F2 renames, Delete removes, and the row menu inserts
//! into the text.

use std::path::PathBuf;
use std::rc::Rc;

use teksilo::core::widget::WidgetPlacement;
use teksilo::data::{ListModel, SelectionMode, SelectionModel};
use teksilo::platform::clipboard::ClipboardHandle;
use teksilo::prelude::*;
use teksilo::widgets::{
    ActivateOn, DockOpenLocation, DockSide, DockWidget, DockWidgetId, DragTransferMode, DropTarget,
    FocusScope, IconButton, ListView, Padding, ScrollBarMode, Switcher, TextWidget, ToolbarItem,
    TraversalScopePolicy,
};

use crate::models::ResourceRow;
use crate::resources::row::{DetachFrom, list_keys, resource_row};
use crate::resources::{AddTarget, ResourcesViewModel};

/// Package the Resources panel as a trailing `DockWidget`.
pub fn resources_dock(vm: ResourcesViewModel, dock_id: DockWidgetId) -> DockWidget {
    let header_vm = vm.clone();
    DockWidget::new(dock_id, tr!(resources_title()), move |_id| {
        FocusScope::new(TraversalScopePolicy::Continue).child(resources_panel(vm.clone()))
    })
    .icon(crate::icons::activity::resources_icon)
    .show_header(true)
    .header_actions(move |_id| {
        let files_vm = header_vm.clone();
        let link_vm = header_vm.clone();
        vec![
            ToolbarItem::custom(
                IconButton::add()
                    .toolbar()
                    .tooltip(tr!(resources_add_files()))
                    .enabled(header_vm.focus().map(Option::is_some))
                    .on_activate_fn(move |ctx| files_vm.pick_files(ctx)),
            ),
            ToolbarItem::custom(
                IconButton::new(crate::icons::resources::add_link())
                    .toolbar()
                    .tooltip(tr!(resources_add_link()))
                    .enabled(header_vm.focus().map(Option::is_some))
                    .on_activate_fn(move |ctx| {
                        let url = clipboard_address(ctx).unwrap_or_default();
                        super::edit_panel::present_add_link(&link_vm, ctx, &url);
                    }),
            ),
        ]
    })
    .default_location(DockOpenLocation::side(DockSide::Trailing))
}

/// The web address on the clipboard, if it holds one: what Add a web link starts with.
pub(crate) fn clipboard_address(ctx: &EventContext) -> Option<String> {
    let text = ctx.app_state::<ClipboardHandle>()?.get_text().ok()?;
    crate::resources::web_addresses_in(&text).into_iter().next()
}

/// The panel: the list, or why there is none, inside a drop target for files and links
/// from outside.
pub fn resources_panel(vm: ResourcesViewModel) -> impl Widget {
    let drop_vm = vm.clone();
    let targeted = Signal::new(false);
    DropTarget::new()
        .accept_external()
        .targeted_signal(targeted)
        .hint(TextWidget::new(tr!(resources_drop_hint())))
        .on_drop(move |payload, _pos, ctx| {
            let files: Vec<PathBuf> = payload.files().to_vec();
            if !files.is_empty() {
                drop_vm.add_paths(ctx, files, AddTarget::Dock);
                return true;
            }
            let mut text = payload.uris().join("\n");
            if let Some(t) = payload.text() {
                text.push('\n');
                text.push_str(t);
            }
            drop_vm.add_dropped_text(ctx, &text)
        })
        .child(ResourcesList::new(vm))
}

/// The list, wrapped in a widget so its re-derivation effects have a context: the rows
/// depend on the model's version and on the item in front of the writer.
struct ResourcesList {
    vm: ResourcesViewModel,
    model: ListModel<ResourceRow>,
    selection: SelectionModel,
    child: Option<WidgetId>,
}

impl ResourcesList {
    fn new(vm: ResourcesViewModel) -> Self {
        Self {
            model: ListModel::from_vec(vm.rows()),
            selection: SelectionModel::new(SelectionMode::Single),
            vm,
            child: None,
        }
    }
}

impl std::fmt::Debug for ResourcesList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ResourcesList").finish_non_exhaustive()
    }
}

/// Which page the panel shows.
fn page(has_item: bool, has_rows: bool) -> usize {
    match (has_item, has_rows) {
        (false, _) => 0,
        (true, false) => 1,
        (true, true) => 2,
    }
}

impl Widget for ResourcesList {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        let no_item = teksu!(
            Padding::symmetric(16.0, 24.0) {
                TextWidget::new(tr!(resources_no_item())) {
                    style: TextStyleRole::Small
                    color: TextRole::Secondary
                }
            }
        );
        let empty = teksu!(
            Padding::symmetric(16.0, 24.0) {
                TextWidget::new(tr!(resources_empty())) {
                    style: TextStyleRole::Small
                    color: TextRole::Secondary
                }
            }
        );

        let list = {
            let vm = self.vm.clone();
            let model = self.model.clone();
            let open_vm = self.vm.clone();
            ListView::new(self.model.clone(), move |_i, row: &ResourceRow, _sel| {
                Box::new(resource_row(row, vm.clone(), DetachFrom::Focus)) as Box<dyn Widget>
            })
            .selection(self.selection.clone())
            .auto_item_height(48.0)
            .scroll_bar_style(ScrollBarMode::Overlay)
            .activate_on(ActivateOn::DoubleClick)
            // A row dragged into a text becomes a link, or a picture, there.
            .exportable(DragTransferMode::Copy)
            .type_ahead_label(|row: &ResourceRow| row.title.clone())
            .on_activate(move |idx, ctx| {
                if let Some(row) = model.with_item(idx, |r| r.clone()) {
                    open_vm.open(ctx, &row);
                }
            })
        };

        let state = Signal::new(page(
            self.vm.focus().get().is_some(),
            !self.model.is_empty(),
        ));
        let switcher = Switcher::new(state.clone())
            .child(no_item)
            .child(empty)
            .child(list);

        let keys = {
            let paste_vm = self.vm.clone();
            list_keys(
                self.vm.clone(),
                self.model.clone(),
                self.selection.clone(),
                Rc::new(move |c, text| paste_vm.add_dropped_text(c, text)),
            )
        };
        let id = ctx.add(switcher.on_key(keys));
        self.child = Some(id);

        let rederive = {
            let vm = self.vm.clone();
            let model = self.model.clone();
            let state = state.clone();
            move || {
                let rows = vm.rows();
                state.set_if_changed(page(vm.focus().get().is_some(), !rows.is_empty()));
                model.reconcile_by_key(rows, |r| r.id);
            }
        };
        rederive();
        {
            let f = rederive.clone();
            ctx.effect(&self.vm.version(), move |_| f());
        }
        {
            let f = rederive.clone();
            ctx.effect(&self.vm.focus(), move |_| f());
        }
        vec![id]
    }

    fn layout_response(&self, proposal: SizeProposal, ctx: &LayoutContext) -> LayoutResponse {
        self.child
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
            child.origin = Point::new(bounds.x, bounds.y);
            child.size = bounds.size();
        }
    }

    fn children(&self) -> Vec<WidgetId> {
        self.child.into_iter().collect()
    }
}

#[cfg(test)]
mod tests;
