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

use teksilo::core::BindingLevel;
use teksilo::core::overlay::OverlayPlacement;
use teksilo::core::widget::WidgetPlacement;
use teksilo::data::{ListModel, SelectionMode, SelectionModel};
use teksilo::platform::clipboard::ClipboardHandle;
use teksilo::prelude::*;
use teksilo::widgets::{
    ActivateOn, Center, DockOpenLocation, DockSide, DockWidget, DockWidgetId, DragTransferMode,
    DropTarget, Expand, FixedSize, FocusScope, HStack, IconButton, IconWidget, ImageFit,
    ImageWidget, ListView, MenuItem, MenuList, Padding, Panel, PopoverIconButton, ScrollBarMode,
    Switcher, TextWidget, ToolbarItem, TraversalScopePolicy, VStack,
};

use crate::models::ResourceRow;
use crate::resources::file_kind::{self, FileKind};
use crate::resources::{AddTarget, ResourcesViewModel};
use crate::shared::picture_cache::{PictureCache, PictureKey, PictureState};

/// The thumbnail's side, in logical pixels.
const THUMB: f32 = 32.0;

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
fn clipboard_address(ctx: &EventContext) -> Option<String> {
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

    fn selected(model: &ListModel<ResourceRow>, selection: &SelectionModel) -> Option<ResourceRow> {
        let index = *selection.selected_indices().first()?;
        model.with_item(index, |r| r.clone())
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
                Box::new(resource_row(row, vm.clone())) as Box<dyn Widget>
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
            let vm = self.vm.clone();
            let model = self.model.clone();
            let selection = self.selection.clone();
            move |ev: &WidgetEvent, ctx: &mut EventContext| match ev {
                WidgetEvent::KeyDown {
                    key: Key::Character('v'),
                    modifiers,
                    ..
                } if modifiers.command() => {
                    let text = ctx
                        .app_state::<ClipboardHandle>()
                        .and_then(|c| c.get_text().ok())
                        .unwrap_or_default();
                    if vm.add_dropped_text(ctx, &text) {
                        EventResponse::Handled
                    } else {
                        EventResponse::Ignored
                    }
                }
                WidgetEvent::KeyDown { key: Key::F2, .. } => {
                    match Self::selected(&model, &selection) {
                        Some(row) => {
                            super::edit_panel::present_edit(&vm, ctx, &row);
                            EventResponse::Handled
                        }
                        None => EventResponse::Ignored,
                    }
                }
                WidgetEvent::KeyDown {
                    key: Key::Delete, ..
                } => match Self::selected(&model, &selection) {
                    Some(row) => {
                        vm.remove(ctx, &row);
                        EventResponse::Handled
                    }
                    None => EventResponse::Ignored,
                },
                _ => EventResponse::Ignored,
            }
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

/// The family of `row`, for its icon and its name.
pub fn kind_of(row: &ResourceRow) -> FileKind {
    if row.is_web_link() {
        FileKind::WebLink
    } else {
        file_kind::kind_of(&row.file_name, &row.mime_type)
    }
}

/// The second line of a row: the address of a web link, else the file's size, with
/// "linked" for a file the project only points at.
pub fn detail_line(row: &ResourceRow) -> String {
    if row.is_web_link() {
        return row.location.clone();
    }
    let size = if row.is_stored() {
        row.byte_size
    } else {
        row.path
            .as_deref()
            .and_then(|p| std::fs::metadata(p).ok())
            .map_or(0, |m| m.len())
    };
    let size = (size > 0).then(|| crate::shared::units::byte_size(size));
    match (row.is_linked(), size) {
        (true, Some(size)) => tr!(resources_row_linked_size(size = size)).resolve_now(),
        (true, None) => tr!(resources_row_linked()).resolve_now(),
        (false, Some(size)) => size,
        (false, None) => row.file_name.clone(),
    }
}

/// What a screen reader hears for a row: its title, its type, its size or address, and
/// whether the file is missing.
pub fn row_name(row: &ResourceRow) -> String {
    let kind = kind_of(row).label().resolve_now();
    let detail = detail_line(row);
    let title = if row.title.trim().is_empty() {
        row.display_name().to_string()
    } else {
        row.title.clone()
    };
    if row.missing {
        tr!(resources_row_name_missing(
            title = title,
            kind = kind,
            detail = detail
        ))
        .resolve_now()
    } else {
        tr!(resources_row_name(
            title = title,
            kind = kind,
            detail = detail
        ))
        .resolve_now()
    }
}

/// One resource: its type, its title and its size or address, its badge, its menu.
fn resource_row(row: &ResourceRow, vm: ResourcesViewModel) -> impl Widget + use<> {
    let title = if row.title.trim().is_empty() {
        row.display_name().to_string()
    } else {
        row.title.clone()
    };
    let detail = detail_line(row);
    let name = row_name(row);
    let actions = tr!(resources_actions());
    let more = PopoverIconButton::new(
        IconButton::new(IconWidget::chevron_down(12.0)).tooltip(actions.clone()),
    )
    .bare()
    .show_disclosure_caret(false)
    .content(row_menu(row, vm))
    .placement(OverlayPlacement::BelowPreferred)
    .access_label(actions);

    let badge: Option<Box<dyn Widget>> = row.missing.then(|| {
        Box::new(
            TextWidget::new(tr!(resources_not_found()))
                .style(TextStyleRole::Tiny)
                .color(TextRole::Warning)
                .single_line(),
        ) as Box<dyn Widget>
    });

    let text = teksu!(
        VStack {
            spacing: 2.0
            TextWidget::new(lit!(title)) {
                style: TextStyleRole::Small
                single_line
                overflow: TextOverflow::Ellipsis(EllipsisMode::Trailing)
            }
            TextWidget::new(lit!(detail)) {
                style: TextStyleRole::Tiny
                color: TextRole::Secondary
                single_line
                overflow: TextOverflow::Ellipsis(EllipsisMode::Middle)
            }
        }
    );

    let mut line = HStack::new()
        .spacing(8.0)
        .child(ResourceIcon::new(row))
        .child(Expand::horizontal().child(text));
    if let Some(badge) = badge {
        line = line.child(crate::tabs::Boxed::new(badge));
    }
    let line = line.child(more);
    Padding::symmetric(8.0, 6.0).child(line).access_label(name)
}

/// The row's menu: every action, so each has a keyboard path.
fn row_menu(row: &ResourceRow, vm: ResourcesViewModel) -> MenuList {
    let mut menu = MenuList::new();
    {
        let (vm, row) = (vm.clone(), row.clone());
        menu = menu
            .item(MenuItem::new(tr!(resources_open())).on_activate_fn(move |c| vm.open(c, &row)));
    }
    if !row.is_web_link() {
        let (vm, row) = (vm.clone(), row.clone());
        menu = menu.item(
            MenuItem::new(tr!(resources_show_in_folder()))
                .enabled(!row.missing)
                .on_activate_fn(move |c| vm.reveal(c, &row)),
        );
    }
    if row.is_linked() && row.missing {
        let (vm, row) = (vm.clone(), row.clone());
        menu = menu.item(
            MenuItem::new(tr!(resources_locate())).on_activate_fn(move |c| vm.locate(c, &row)),
        );
    }
    menu = menu.separator();
    if ResourcesViewModel::offers_picture(row) {
        {
            let (vm, row) = (vm.clone(), row.clone());
            menu = menu.item(
                MenuItem::new(tr!(resources_insert_picture())).on_activate_fn(move |c| {
                    match vm.insertion_point() {
                        Some((handle, at)) => vm.insert_picture(c, &handle, at, &row),
                        None => vm.say_no_text(c),
                    }
                }),
            );
        }
        let (vm, row) = (vm.clone(), row.clone());
        menu = menu.item(
            MenuItem::new(tr!(resources_insert_link())).on_activate_fn(move |c| {
                match vm.insertion_point() {
                    Some((handle, at)) => vm.insert_link(&handle, at, &row),
                    None => vm.say_no_text(c),
                }
            }),
        );
    } else {
        let (vm, row) = (vm.clone(), row.clone());
        menu = menu.item(
            MenuItem::new(tr!(resources_insert_into_text())).on_activate_fn(move |c| {
                match vm.insertion_point() {
                    Some((handle, at)) => vm.insert_link(&handle, at, &row),
                    None => vm.say_no_text(c),
                }
            }),
        );
    }
    menu = menu.separator();
    {
        let (vm, row) = (vm.clone(), row.clone());
        menu = menu.item(
            MenuItem::new(tr!(resources_edit()))
                .on_activate_fn(move |c| super::edit_panel::present_edit(&vm, c, &row)),
        );
    }
    {
        let (vm, row) = (vm.clone(), row.clone());
        menu = menu.item(
            MenuItem::new(tr!(resources_detach())).on_activate_fn(move |c| vm.detach(c, &row)),
        );
    }
    let row = row.clone();
    menu.item(MenuItem::new(tr!(resources_remove())).on_activate_fn(move |c| vm.remove(c, &row)))
}

/// A row's leading square: a picture's thumbnail once decoded, the type's icon until then
/// and for everything else.
struct ResourceIcon {
    kind: FileKind,
    picture: Option<(PathBuf, String)>,
    root: Option<WidgetId>,
}

impl ResourceIcon {
    fn new(row: &ResourceRow) -> Self {
        let picture = (!row.missing
            && !row.content_hash.is_empty()
            && file_kind::is_viewable_picture(&row.file_name))
        .then(|| row.path.clone())
        .flatten()
        .map(|p| (p, row.content_hash.clone()));
        Self {
            kind: kind_of(row),
            picture,
            root: None,
        }
    }
}

impl std::fmt::Debug for ResourceIcon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ResourceIcon")
            .field("kind", &self.kind)
            .finish()
    }
}

impl Widget for ResourceIcon {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        let state = match &self.picture {
            Some((path, hash)) => {
                let rt = ctx.app_state::<AsyncRuntimeHandle>().cloned();
                let key = PictureKey {
                    hash: hash.clone(),
                    edge: (THUMB * 2.0) as u32,
                };
                let slot = PictureCache::shared().request(rt.as_ref(), path.clone(), key);
                slot.bind_to(ctx.self_id(), ctx.binding_registry(), BindingLevel::Rebuild);
                slot.get()
            }
            None => PictureState::Failed,
        };
        let inner: Box<dyn Widget> = match state {
            PictureState::Ready(decoded) => Box::new(
                ImageWidget::from_raw(decoded.pixels.clone(), decoded.width, decoded.height)
                    .fit(ImageFit::Contain),
            ),
            _ => Box::new(
                Center::new().child(crate::icons::resources::for_kind(self.kind).icon_size(20.0)),
            ),
        };
        let id = ctx.add(
            FixedSize::new().width(THUMB).height(THUMB).child(
                Panel::new()
                    .background(SurfaceRole::Sunken)
                    .corner_radius(3.0)
                    .child(crate::tabs::Boxed::new(inner)),
            ),
        );
        self.root = Some(id);
        vec![id]
    }

    fn layout_response(&self, proposal: SizeProposal, ctx: &LayoutContext) -> LayoutResponse {
        self.root
            .and_then(|id| ctx.child_size(id, proposal))
            .map(LayoutResponse::from)
            .unwrap_or_else(|| proposal.resolve(THUMB, THUMB).into())
    }

    fn children(&self) -> Vec<WidgetId> {
        self.root.into_iter().collect()
    }
}

#[cfg(test)]
mod tests;
