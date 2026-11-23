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
//! At its top the writer chooses what it lists ([`DockView`]): *This item*, or
//! *Unattached*, the project's resources that no item carries. The second is where
//! research collected before it belongs anywhere lives: what is added there, by the
//! buttons, a drop or Ctrl+V, goes on no item, and each row offers *Attach to items…*.
//! Dragging such a row into a text, or putting a link to it there from its menu, files
//! it on that text's item.
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
    Expand, FocusScope, IconButton, ListView, Padding, ScrollBarMode, Segment, SegmentedControl,
    Switcher, TextWidget, ToolbarItem, TraversalScopePolicy, VStack,
};

use crate::models::ResourceRow;
use crate::resources::row::{DetachFrom, list_keys, resource_row};
use crate::resources::{AddTarget, DockView, ResourcesViewModel};

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
                    .enabled(header_vm.can_add())
                    .on_activate_fn(move |ctx| files_vm.pick_files(ctx)),
            ),
            ToolbarItem::custom(
                IconButton::new(crate::icons::resources::add_link())
                    .toolbar()
                    .tooltip(tr!(resources_add_link()))
                    .enabled(header_vm.can_add())
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

/// The panel: the choice of what it lists, and the list or why there is none, inside a
/// drop target for files and links from outside.
pub fn resources_panel(vm: ResourcesViewModel) -> impl Widget {
    let drop_vm = vm.clone();
    let targeted = Signal::new(false);
    // Says where a drop goes, which the choice at the top decides.
    let hint = vm.view().map(|view| match view {
        DockView::Item => tr!(resources_drop_hint()).resolve_now(),
        DockView::Unattached => tr!(resources_drop_hint_unattached()).resolve_now(),
    });
    DropTarget::new()
        .accept_external()
        .targeted_signal(targeted)
        .hint(TextWidget::new(lit!("")).text(hint))
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

/// The choice and the list, wrapped in a widget so its effects have a context: the rows
/// depend on the model's version, on the item in front of the writer and on the choice.
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

/// Which page the panel shows: no item open, nothing on it, the list, nothing
/// unattached.
fn page(view: DockView, has_item: bool, has_rows: bool) -> usize {
    match (view, has_item, has_rows) {
        (DockView::Item, false, _) => 0,
        (DockView::Item, true, false) => 1,
        (_, _, true) => 2,
        // An unattached resource needs no item open to be listed.
        (DockView::Unattached, _, false) => 3,
    }
}

/// The page for `vm`'s choice and rows now.
fn page_of(vm: &ResourcesViewModel, rows: &[ResourceRow]) -> usize {
    page(
        vm.view().get(),
        vm.focus().get().is_some(),
        !rows.is_empty(),
    )
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
        let none_loose = teksu!(
            Padding::symmetric(16.0, 24.0) {
                TextWidget::new(tr!(resources_unattached_empty())) {
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
                // Read when the row is made: the two lists share no resource, so a change
                // of choice makes every row again.
                let detach = match vm.view().get() {
                    DockView::Item => DetachFrom::Focus,
                    DockView::Unattached => DetachFrom::Nothing,
                };
                Box::new(resource_row(row, vm.clone(), detach)) as Box<dyn Widget>
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
            self.vm.view().get(),
            self.vm.focus().get().is_some(),
            !self.model.is_empty(),
        ));
        let switcher = Switcher::new(state.clone())
            .child(no_item)
            .child(empty)
            .child(list)
            .child(none_loose);

        let keys = {
            let paste_vm = self.vm.clone();
            list_keys(
                self.vm.clone(),
                self.model.clone(),
                self.selection.clone(),
                Rc::new(move |c, text| paste_vm.add_dropped_text(c, text)),
            )
        };
        let choice = view_choice(ctx, self.vm.view());
        let id = teksu!(ctx => VStack {
            spacing: 0.0
            Padding::symmetric(10.0, 6.0) {
                child: choice
            }
            Expand::new() {
                child: switcher.on_key(keys)
            }
        });
        self.child = Some(id);

        let rederive = {
            let vm = self.vm.clone();
            let model = self.model.clone();
            let state = state.clone();
            move || {
                let rows = vm.rows();
                state.set_if_changed(page_of(&vm, &rows));
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
        {
            let f = rederive.clone();
            ctx.effect(&self.vm.view(), move |_| f());
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

/// *This item* · *Unattached*, bound both ways to `view` by two guarded effects, so
/// neither writes the other back for ever.
fn view_choice(ctx: &mut BuildContext, view: Signal<DockView>) -> SegmentedControl {
    let index = Signal::new(view.get().index());
    {
        let index = index.clone();
        ctx.effect(&view, move |v: &DockView| {
            index.set_if_changed(v.index());
        });
    }
    {
        let view = view.clone();
        ctx.effect(&index, move |i| {
            view.set_if_changed(DockView::from_index(*i));
        });
    }
    SegmentedControl::indexed(index)
        .label(tr!(resources_dock_view_label()))
        .fill_width(true)
        .segment(
            Segment::new(tr!(resources_dock_view_item()))
                .tooltip(tr!(resources_dock_view_item_hint())),
        )
        .segment(
            Segment::new(tr!(resources_dock_view_unattached()))
                .tooltip(tr!(resources_dock_view_unattached_hint())),
        )
}

#[cfg(test)]
mod tests;
