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

use std::cell::{Cell, RefCell};
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
    root: Option<WidgetId>,
    menu: Option<WidgetId>,
}

impl ResourceDropHost {
    pub fn new(child: Box<dyn Widget>, drop: Option<(WeakEditorHandle, FormatViewModel)>) -> Self {
        Self {
            child: Some(child),
            drop,
            root: None,
            menu: None,
        }
    }
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

        let as_picture = {
            let (pending, format, menu_id) = (pending.clone(), format.clone(), menu_id.clone());
            move |c: &mut EventContext| {
                if let Some(id) = menu_id.get() {
                    c.dismiss_overlay_by_content(id);
                }
                if let Some(dropped) = pending.borrow_mut().take() {
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
                }
            }
        };
        let menu = MenuList::new()
            .item(MenuItem::new(tr!(resources_drop_insert_picture())).on_activate_fn(as_picture))
            .item(MenuItem::new(tr!(resources_drop_insert_link())).on_activate_fn(as_link));
        let menu = ctx.add(menu);
        ctx.set_dormant(menu);
        menu_id.set(Some(menu));
        self.menu = Some(menu);

        let anchor = ctx.self_id();
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
                    }
                    DropAction::AskPictureOrLink(row) => {
                        *pending.borrow_mut() = Some(DroppedResource { row, handle, at });
                        c.materialize_now(menu);
                        c.activate(menu);
                        c.show_overlay(OverlayRequest::new(
                            menu,
                            anchor,
                            OverlayPlacement::AtPointer(window),
                            DismissBehavior::EscapeOrClickOutside,
                        ));
                        c.request_focus_into(menu);
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
