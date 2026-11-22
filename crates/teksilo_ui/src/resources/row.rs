// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! One resource as every Resources surface shows it: the row of the dock and of a
//! folder's Resources page, its menu, and its leading square.
//!
//! The dock and the folder page differ in one thing only, which items the resource is
//! taken off by "Detach" ([`DetachFrom`]): the dock's item is the one in front of the
//! writer, a folder page names every item of its subtree the resource is on, and also
//! says so under the row, each name a link that opens the item.
//!
//! The menu holds every action, so each has a keyboard path; the Gallery appends the
//! same actions to its own tile menu ([`resource_actions`]).

use std::path::PathBuf;
use std::rc::Rc;

use teksilo::core::BindingLevel;
use teksilo::core::overlay::OverlayPlacement;
use teksilo::data::{ListModel, SelectionModel};
use teksilo::platform::clipboard::ClipboardHandle;
use teksilo::prelude::*;
use teksilo::widgets::{
    Center, Expand, FixedSize, HStack, IconButton, IconWidget, ImageFit, ImageWidget, Link,
    MenuItem, MenuList, Padding, Panel, PopoverIconButton, TextWidget, VStack, Wrap,
};

use crate::intents::AppIntent;
use crate::models::ResourceRow;
use crate::resources::ResourcesViewModel;
use crate::resources::file_kind::{self, FileKind};
use crate::shared::picture_cache::{PictureCache, PictureKey, PictureState};

/// The thumbnail's side in a row, in logical pixels.
const THUMB: f32 = 32.0;

/// How many items a row names before it says how many more there are.
pub const NAMED_ITEMS: usize = 3;

/// An item a resource is attached to, by name.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AttachedItem {
    pub id: u64,
    pub title: String,
}

impl AttachedItem {
    /// The item's title, or "Untitled" for one the writer never named.
    pub fn label(&self) -> String {
        if self.title.trim().is_empty() {
            tr!(untitled()).resolve_now()
        } else {
            self.title.clone()
        }
    }
}

/// Which items "Detach" takes a resource off.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DetachFrom {
    /// The item in front of the writer, read when the writer chooses it: the dock.
    Focus,
    /// These items, each by name: a folder's Resources page, the Gallery.
    Items(Vec<AttachedItem>),
}

/// The family of `row`, for its icon and its name.
pub fn kind_of(row: &ResourceRow) -> FileKind {
    if row.is_web_link() {
        FileKind::WebLink
    } else {
        file_kind::kind_of(&row.file_name, &row.mime_type)
    }
}

/// The title a row shows: its own, else the name of its file or its address.
pub fn title_of(row: &ResourceRow) -> String {
    if row.title.trim().is_empty() {
        row.display_name().to_string()
    } else {
        row.title.clone()
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
    let title = title_of(row);
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

/// [`row_name`], with the items the resource is on: a folder page's row.
pub fn row_name_with_items(row: &ResourceRow, items: &[AttachedItem]) -> String {
    if items.is_empty() {
        return row_name(row);
    }
    tr!(resources_row_name_attached(
        name = row_name(row),
        items = items
            .iter()
            .map(AttachedItem::label)
            .collect::<Vec<_>>()
            .join(&format!(
                "{} ",
                tr!(resources_attached_separator()).resolve_now()
            ))
    ))
    .resolve_now()
}

/// One resource: its type, its title and its size or address, its badge, its menu, and
/// on a folder page the items it is on.
pub fn resource_row(
    row: &ResourceRow,
    vm: ResourcesViewModel,
    detach: DetachFrom,
) -> impl Widget + use<> {
    let title = title_of(row);
    let detail = detail_line(row);
    let attached = match &detach {
        DetachFrom::Focus => Vec::new(),
        DetachFrom::Items(items) => items.clone(),
    };
    let name = row_name_with_items(row, &attached);
    let actions = tr!(resources_actions());
    let more = PopoverIconButton::new(
        IconButton::new(IconWidget::chevron_down(12.0)).tooltip(actions.clone()),
    )
    .bare()
    .show_disclosure_caret(false)
    .content(row_menu(row, vm, detach))
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
    let on_items: Option<Box<dyn Widget>> =
        (!attached.is_empty()).then(|| Box::new(attached_line(&attached)) as Box<dyn Widget>);

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
            child_opt: on_items
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

/// "On: A, B, C and 2 more", each name a link that opens its item.
fn attached_line(items: &[AttachedItem]) -> impl Widget + use<> {
    let mut wrap = Wrap::new().spacing(6.0).line_spacing(2.0).child(
        TextWidget::new(tr!(resources_attached_to()))
            .style(TextStyleRole::Tiny)
            .color(TextRole::Secondary)
            .single_line(),
    );
    let named = items.len().min(NAMED_ITEMS);
    for (i, item) in items.iter().take(NAMED_ITEMS).enumerate() {
        let (item_id, title) = (item.id, item.title.clone());
        let link = Link::new(lit!(item.label()))
            .tooltip(tr!(resources_open_item(title = item.label())))
            .on_activate_fn(move |c| {
                c.send_intent(AppIntent::OpenItem {
                    item_id,
                    title: title.clone(),
                })
            });
        // A separator glued to every name but the last one named: "A, B, C and 2 more".
        let mut pair = HStack::new().spacing(0.0).child(link);
        if i + 1 < named {
            pair = pair.child(
                TextWidget::new(tr!(resources_attached_separator()))
                    .style(TextStyleRole::Tiny)
                    .color(TextRole::Secondary)
                    .single_line(),
            );
        }
        wrap = wrap.child(pair);
    }
    if items.len() > NAMED_ITEMS {
        wrap = wrap.child(
            TextWidget::new(tr!(resources_attached_more(
                count = (items.len() - NAMED_ITEMS) as i64
            )))
            .style(TextStyleRole::Tiny)
            .color(TextRole::Secondary)
            .single_line(),
        );
    }
    wrap
}

/// The row's menu: every action, so each has a keyboard path.
pub fn row_menu(row: &ResourceRow, vm: ResourcesViewModel, detach: DetachFrom) -> MenuList {
    resource_actions(MenuList::new(), row, vm, detach)
}

/// Every action on a resource, appended to `menu`: open, show in folder, locate, the
/// ways into the text, rename and describe, detach, remove.
pub fn resource_actions(
    mut menu: MenuList,
    row: &ResourceRow,
    vm: ResourcesViewModel,
    detach: DetachFrom,
) -> MenuList {
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
    match detach {
        DetachFrom::Focus => {
            let (vm, row) = (vm.clone(), row.clone());
            menu = menu.item(
                MenuItem::new(tr!(resources_detach())).on_activate_fn(move |c| vm.detach(c, &row)),
            );
        }
        DetachFrom::Items(items) => {
            // The keyboard path to each item the row's links open, then one Detach per item.
            for item in &items {
                let (item_id, title) = (item.id, item.title.clone());
                menu = menu.item(
                    MenuItem::new(tr!(resources_open_item(title = item.label()))).on_activate_fn(
                        move |c| {
                            c.send_intent(AppIntent::OpenItem {
                                item_id,
                                title: title.clone(),
                            })
                        },
                    ),
                );
            }
            for item in items {
                let (vm, row) = (vm.clone(), row.clone());
                menu = menu.item(
                    MenuItem::new(tr!(resources_detach_from(title = item.label())))
                        .on_activate_fn(move |c| vm.detach_from(c, item.id, &row)),
                );
            }
        }
    }
    let row = row.clone();
    menu.item(MenuItem::new(tr!(resources_remove())).on_activate_fn(move |c| vm.remove(c, &row)))
}

/// What pasting a text on a list does with it: add its web addresses somewhere. Answers
/// whether it added any.
pub type PasteFn = Rc<dyn Fn(&mut EventContext, &str) -> bool>;

/// The keys a list of resources answers: Ctrl+V adds the web addresses on the clipboard
/// (through `paste`), F2 renames and describes the selected row, Delete removes it.
pub fn list_keys(
    vm: ResourcesViewModel,
    model: ListModel<ResourceRow>,
    selection: SelectionModel,
    paste: PasteFn,
) -> impl Fn(&WidgetEvent, &mut EventContext) -> EventResponse + 'static {
    let selected = move || -> Option<ResourceRow> {
        let index = *selection.selected_indices().first()?;
        model.with_item(index, |r| r.clone())
    };
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
            if paste(ctx, &text) {
                EventResponse::Handled
            } else {
                EventResponse::Ignored
            }
        }
        WidgetEvent::KeyDown { key: Key::F2, .. } => match selected() {
            Some(row) => {
                super::edit_panel::present_edit(&vm, ctx, &row);
                EventResponse::Handled
            }
            None => EventResponse::Ignored,
        },
        WidgetEvent::KeyDown {
            key: Key::Delete, ..
        } => match selected() {
            Some(row) => {
                vm.remove(ctx, &row);
                EventResponse::Handled
            }
            None => EventResponse::Ignored,
        },
        _ => EventResponse::Ignored,
    }
}

/// A row's leading square: a picture's thumbnail once decoded, the type's icon until then
/// and for everything else.
pub struct ResourceIcon {
    kind: FileKind,
    picture: Option<(PathBuf, String)>,
    side: f32,
    root: Option<WidgetId>,
}

impl ResourceIcon {
    pub fn new(row: &ResourceRow) -> Self {
        Self {
            kind: kind_of(row),
            picture: picture_of(row),
            side: THUMB,
            root: None,
        }
    }

    /// The square's side, in logical pixels: a grid tile's is larger than a row's.
    pub fn side(mut self, side: f32) -> Self {
        self.side = side;
        self
    }
}

/// The file and content hash a row's thumbnail is decoded from: a picture the editor
/// can paint, present, with a known hash. `None` for everything else.
pub fn picture_of(row: &ResourceRow) -> Option<(PathBuf, String)> {
    (!row.missing && !row.content_hash.is_empty() && file_kind::is_viewable_picture(&row.file_name))
        .then(|| row.path.clone())
        .flatten()
        .map(|p| (p, row.content_hash.clone()))
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
                    edge: (self.side * 2.0) as u32,
                };
                let slot = PictureCache::shared().request(rt.as_ref(), path.clone(), key);
                slot.bind_to(ctx.self_id(), ctx.binding_registry(), BindingLevel::Rebuild);
                slot.get()
            }
            None => PictureState::Failed,
        };
        let inner: Box<dyn Widget> = match state {
            PictureState::Ready(decoded) => Box::new(
                // Decorative: the row's accessible name already says the resource's
                // title and kind, and teksilo refuses an image with neither alt text nor
                // this (it panicked the moment a dropped picture's thumbnail decoded).
                ImageWidget::from_raw(decoded.pixels.clone(), decoded.width, decoded.height)
                    .fit(ImageFit::Contain)
                    .a11y_hidden(),
            ),
            _ => Box::new(
                // 20 px in a row's 32 px square, and in step with it in a larger one.
                Center::new().child(
                    crate::icons::resources::for_kind(self.kind).icon_size(self.side * 0.625),
                ),
            ),
        };
        let id = ctx.add(
            FixedSize::new().width(self.side).height(self.side).child(
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
            .unwrap_or_else(|| proposal.resolve(self.side, self.side).into())
    }

    fn children(&self) -> Vec<WidgetId> {
        self.root.into_iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::ResourceKind;

    /// A dropped picture's thumbnail, decoded, reaches the accessibility tree without
    /// tripping teksilo's alt-text check. It panicked the moment the first one decoded:
    /// no test reached the decoded state, which needs a real picture on disk.
    #[test]
    fn a_decoded_picture_thumbnail_is_decorative_and_does_not_panic() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("Lighthouse.png");
        let icon = teksilo::canvas::RasterIcon::from_raw(vec![200; 4 * 4 * 4], 4, 4);
        std::fs::write(
            &path,
            crate::shared::images::encode_png(&icon).unwrap_or_default(),
        )
        .unwrap();
        let row = ResourceRow {
            kind: ResourceKind::StoredFile,
            file_name: "Lighthouse.png".into(),
            content_hash: "a-hash-only-this-test-uses".into(),
            path: Some(path),
            ..Default::default()
        };
        assert!(picture_of(&row).is_some(), "the row must take a thumbnail");

        let mut tree = teksilo::core::widget_tree::WidgetTree::new();
        tree.add(ResourceIcon::new(&row));
        // No async runtime in a test tree: the cache decodes on the spot, so the
        // thumbnail is built in its decoded state.
        tree.layout(SizeProposal::exact(64.0, 64.0));
        tree.layout(SizeProposal::exact(64.0, 64.0));
        let _ = tree.accessibility_tree_snapshot();
    }
}
