// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! A resource dragged from the dock and dropped on a text.
//!
//! [`ResourceDropHost`] wraps a writing editor in a `DropTarget` that takes the dock's
//! own rows (`RowDragData<ResourceRow>`) and nothing else. The editor takes files, text
//! and its own drags itself; a payload it does not take bubbles up to this target.
//!
//! The drop lands where it was released. The target reports the point in its own
//! coordinates, and the editor resolves offsets from window ones
//! (`EditorHandle::offset_at_point`), so the point used is the event's own pointer
//! position, which is in the window's. The caret is put there, then:
//!
//! - a web link is inserted as a link to its address;
//! - any other resource as a link `[title](resource:<uid>)`;
//! - a picture asks first, with a small menu at the drop point (keyboard-operable,
//!   Escape cancels): **Insert the picture** (a copy of its bytes becomes a picture of
//!   the text, through the same pipeline Insert image runs) or **Insert a link to the
//!   resource**.
//!
//! The picture half needs the project's pictures folder, an `Asset` row and the undo
//! stack, none of which an editor builder holds, so the choice is parked on the
//! window's [`FormatViewModel`] and the `resources.insert_dropped_picture` command picks
//! it up, as a dropped image file is picked up by `editor.insert_dropped_images`.
//!
//! ## Files from outside the app
//!
//! A file dropped from the desktop is the editor's own drop, not this target's. It is
//! parked as [`DroppedFiles`] with the editor it landed on and that text's item, as a
//! resource row's drop keeps its column's handle and item here, and the command writes
//! there. Never into the editor with focus nor onto the dock's item: a Full Book or Full
//! Chapter shows one editor per scene in one tab, a split view two panes, and a drop
//! moves no focus.

use std::cell::{Cell, RefCell};
use std::path::PathBuf;
use std::rc::Rc;

use teksilo::core::DragPayload;
use teksilo::core::overlay::{DismissBehavior, OverlayPlacement, OverlayRequest};
use teksilo::core::widget::WidgetPlacement;
use teksilo::prelude::*;
use teksilo::widgets::rich_text::{EditorHandle, WeakEditorHandle};
use teksilo::widgets::{DropTarget, MenuItem, MenuList, RowDragData};

use crate::format::FormatViewModel;
use crate::models::ResourceRow;
use crate::resources::{ResourcesViewModel, links};

/// A resource dropped on a text, waiting for the command that inserts it as a picture.
#[derive(Clone)]
pub struct DroppedResource {
    pub row: ResourceRow,
    pub handle: EditorHandle,
    pub at: usize,
}

impl std::fmt::Debug for DroppedResource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DroppedResource")
            .field("row", &self.row.id)
            .field("at", &self.at)
            .finish()
    }
}

/// Files from outside the app dropped on a text, waiting for `editor.insert_dropped_images`
/// (see the module docs).
#[derive(Clone)]
pub struct DroppedFiles {
    pub paths: Vec<PathBuf>,
    /// The editor they landed on, whose caret the drop put where they go.
    pub handle: EditorHandle,
    /// The item that text belongs to, which the files that are not pictures of it are
    /// attached to. `None` for a text with no item behind it.
    pub item: Option<u64>,
}

impl std::fmt::Debug for DroppedFiles {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DroppedFiles")
            .field("paths", &self.paths)
            .field("item", &self.item)
            .finish()
    }
}

/// The command that inserts a parked [`DroppedResource`] as a picture.
pub const INSERT_DROPPED_PICTURE: &str = "resources.insert_dropped_picture";

/// The rows a drag from the Resources dock carries, `None` for any other payload.
pub fn dragged_rows(payload: &mut DragPayload) -> Option<Vec<ResourceRow>> {
    payload
        .take_typed::<RowDragData<ResourceRow>>()
        .filter(RowDragData::is_export)
        .and_then(|d| d.items)
        .filter(|rows| !rows.is_empty())
}

/// What a drop of `rows` does once the caret is at the drop point.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DropAction {
    /// Ask between the picture and a link to it.
    AskPictureOrLink(ResourceRow),
    /// Insert these links: `(words, address)`.
    Links(Vec<(String, String)>),
}

/// Decide what a drop of `rows` does: one picture asks, anything else links.
pub fn drop_action(rows: &[ResourceRow]) -> DropAction {
    if let [row] = rows
        && ResourcesViewModel::offers_picture(row)
    {
        return DropAction::AskPictureOrLink(row.clone());
    }
    DropAction::Links(rows.iter().map(link_words).collect())
}

/// A row's link: its title (or its name) to its address.
fn link_words(row: &ResourceRow) -> (String, String) {
    let words = if row.title.trim().is_empty() {
        row.display_name().to_string()
    } else {
        row.title.clone()
    };
    let href = if row.is_web_link() {
        row.location.clone()
    } else {
        links::resource_href(row.uid)
    };
    (words, href)
}

/// The offset a drop released at `window_point` lands on in `handle`'s text: the
/// character there, or the end of the text when the point falls outside its lines.
pub fn drop_offset(handle: &EditorHandle, window_point: Point) -> usize {
    handle
        .offset_at_point(window_point)
        .unwrap_or_else(|| handle.to_plain_text().chars().count())
}

/// A writing editor that also takes resources dragged from the dock. See the module
/// docs.
pub struct ResourceDropHost {
    child: Option<Box<dyn Widget>>,
    /// The editor and the window's formatting registry, for a surface that takes
    /// drops; `None` for one that does not (a trashed item's text, a test's bare
    /// column), where the host only passes its child through.
    drop: Option<(WeakEditorHandle, FormatViewModel)>,
    /// The binder item this text belongs to: a resource dropped on it is attached to
    /// it as well as put into the text. `None` for a text with no item behind it.
    item: Option<u64>,
    root: Option<WidgetId>,
    /// The picture-or-link menu, and whether it is open (its activation gate).
    menu: Option<WidgetId>,
    open: Signal<bool>,
    /// Where `build` leaves the function that opens the picture-or-link menu, for a
    /// test to call: a drag cannot be made headless (its source view's id cannot be).
    opener_sink: Option<Rc<RefCell<Option<OpenMenuFn>>>>,
}

/// Open the picture-or-link menu for `dropped` at the window point.
pub(crate) type OpenMenuFn = Rc<dyn Fn(&mut EventContext, DroppedResource, Point)>;

impl ResourceDropHost {
    pub fn new(
        child: Box<dyn Widget>,
        drop: Option<(WeakEditorHandle, FormatViewModel)>,
        item: Option<u64>,
    ) -> Self {
        Self {
            child: Some(child),
            drop,
            item,
            root: None,
            menu: None,
            open: Signal::new(false),
            opener_sink: None,
        }
    }

    /// Leave the menu's opener in `sink` once built. For tests.
    #[cfg(test)]
    pub(crate) fn expose_opener(mut self, sink: Rc<RefCell<Option<OpenMenuFn>>>) -> Self {
        self.opener_sink = Some(sink);
        self
    }
}

/// Attach `rows` to `item`, the text's own item, after a drop put them into it: a
/// resource dropped on a text is filed on that item too (`resources.attach`, which
/// says so with an Undo and skips what the item already carries).
fn attach_to_item(c: &mut EventContext, item: Option<u64>, rows: &[ResourceRow]) {
    let Some(item_id) = item else {
        return;
    };
    c.send_intent(crate::intents::AppIntent::AttachResources {
        item_id,
        resource_ids: rows.iter().map(|r| r.id).collect(),
    });
}

impl std::fmt::Debug for ResourceDropHost {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ResourceDropHost").finish_non_exhaustive()
    }
}

impl Widget for ResourceDropHost {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        let child = self
            .child
            .take()
            .unwrap_or_else(|| Box::new(teksilo::widgets::Spacer::new()));
        let child = ctx.add_boxed(child);
        let Some((handle, format)) = self.drop.clone() else {
            self.root = Some(child);
            return vec![child];
        };
        // The picture dropped last, waiting for the writer's choice in the menu.
        let pending: Rc<RefCell<Option<DroppedResource>>> = Rc::new(RefCell::new(None));
        let menu_id: Rc<Cell<Option<WidgetId>>> = Rc::new(Cell::new(None));

        let item = self.item;
        let as_picture = {
            let (pending, format, menu_id) = (pending.clone(), format.clone(), menu_id.clone());
            move |c: &mut EventContext| {
                if let Some(id) = menu_id.get() {
                    c.dismiss_overlay_by_content(id);
                }
                if let Some(dropped) = pending.borrow_mut().take() {
                    attach_to_item(c, item, std::slice::from_ref(&dropped.row));
                    format.park_dropped_resource(dropped);
                    c.send_intent(Intent::new(INSERT_DROPPED_PICTURE));
                }
            }
        };
        let as_link = {
            let (pending, menu_id) = (pending.clone(), menu_id.clone());
            move |c: &mut EventContext| {
                if let Some(id) = menu_id.get() {
                    c.dismiss_overlay_by_content(id);
                }
                if let Some(dropped) = pending.borrow_mut().take() {
                    links::insert_links(&dropped.handle, dropped.at, &[link_words(&dropped.row)]);
                    attach_to_item(c, item, std::slice::from_ref(&dropped.row));
                }
            }
        };
        let menu = MenuList::new()
            .item(MenuItem::new(tr!(resources_drop_insert_picture())).on_activate_fn(as_picture))
            .item(MenuItem::new(tr!(resources_drop_insert_link())).on_activate_fn(as_link));
        // The popover's own shape (`PopoverWidget`): the panel is built the first time
        // it opens (`add_deferred`, materialized in the opener), kept as a child gated
        // on `open` so the host's activation never wakes it inline, and `open` is
        // cleared however the menu is dismissed. Built eagerly and parked dormant, as
        // it first was, the overlay opened on an empty panel.
        let open = self.open.clone();
        let menu = ctx.add_deferred(open.clone(), menu);
        ctx.set_dormant(menu);
        ctx.visible_when(menu, open.clone());
        menu_id.set(Some(menu));
        self.menu = Some(menu);

        let anchor = ctx.self_id();
        let open_menu: OpenMenuFn = {
            let pending = pending.clone();
            let open = open.clone();
            Rc::new(
                move |c: &mut EventContext, dropped: DroppedResource, window: Point| {
                    *pending.borrow_mut() = Some(dropped);
                    open.set(true);
                    c.materialize_now(menu);
                    c.activate(menu);
                    let closed = open.clone();
                    c.show_overlay(
                        OverlayRequest::new(
                            menu,
                            anchor,
                            OverlayPlacement::AtPointer(window),
                            DismissBehavior::EscapeOrClickOutside,
                        )
                        .on_dismiss(Rc::new(move |_, _| closed.set(false))),
                    );
                    c.request_focus_into(menu);
                },
            )
        };
        if let Some(sink) = &self.opener_sink {
            *sink.borrow_mut() = Some(open_menu.clone());
        }
        let target = DropTarget::new()
            .accept_when(|p| {
                p.get_typed::<RowDragData<ResourceRow>>()
                    .is_some_and(RowDragData::is_export)
            })
            .on_drop(move |mut payload, local, c| {
                let Some(rows) = dragged_rows(&mut payload) else {
                    return false;
                };
                let Some(handle) = handle.upgrade() else {
                    return false;
                };
                if !crate::shared::editor_writes::accepts_writes(&handle) {
                    return false;
                }
                // The event's pointer is in window coordinates, which is what the
                // editor resolves; `local` is the target's own, kept only for a drop
                // that carries no pointer sample.
                let window = c.pointer_position().unwrap_or(local);
                let at = drop_offset(&handle, window);
                handle.select_range(at, at);
                match drop_action(&rows) {
                    DropAction::Links(links) => {
                        links::insert_links(&handle, at, &links);
                        attach_to_item(c, item, &rows);
                    }
                    DropAction::AskPictureOrLink(row) => {
                        open_menu(c, DroppedResource { row, handle, at }, window);
                    }
                }
                true
            })
            // `Expand`, because the target lays its content out in a stack that would
            // otherwise give the editor its natural width and centre it: the column
            // must stay as wide as it was without the target around it.
            .child(teksilo::widgets::Expand::horizontal().child(child));
        let root = ctx.add(target);
        self.root = Some(root);
        vec![root, menu]
    }

    fn layout_response(&self, proposal: SizeProposal, ctx: &LayoutContext) -> LayoutResponse {
        // The child's whole response, not just its size: an editor in fill mode says
        // so in it, and a parent laying the column out reads that.
        self.root
            .and_then(|id| ctx.child_layout_response(id, proposal))
            .unwrap_or_else(|| proposal.resolve(0.0, 0.0).into())
    }

    fn place_children(
        &self,
        bounds: Rect,
        _proposal: SizeProposal,
        children: &mut [WidgetPlacement],
        _ctx: &LayoutContext,
    ) {
        // The editor fills the host; the menu's bounds are the overlay manager's.
        for child in children.iter_mut() {
            if Some(child.id) == self.menu {
                child.size = Size::ZERO;
                continue;
            }
            child.origin = bounds.origin();
            child.size = bounds.size();
        }
    }

    fn children(&self) -> Vec<WidgetId> {
        self.root.into_iter().chain(self.menu).collect()
    }
}

#[cfg(test)]
mod tests;
