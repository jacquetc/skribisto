// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The **Resources** segment of every folder: the files and web links of the folder and
//! of everything inside it ([`FolderResourcesViewModel`] says which, and in what order).
//!
//! A header (how many · List / Grid · Add files · Add a web link) over the resources, in
//! one of two layouts the writer picks once for every folder
//! ([`crate::RESOURCES_FOLDER_VIEW_KEY`]):
//!
//! - **List**: the dock's own rows, each with a line naming the items it is on, every
//!   name a link that opens its item.
//! - **Grid**: a waterfall [`GridView`], a picture's thumbnail laid out from its stored
//!   size before it decodes, a file-type icon and the title for anything else.
//!
//! Both are bound to a data model (never built by a loop), open a resource on a double
//! click or Enter, carry the dock's menu (on the row's button, or a right click on a
//! tile), and can be dragged into a text. The page is a drop target for files and web
//! addresses from outside, which, like the two header buttons, attach to the folder.

use std::path::PathBuf;
use std::rc::Rc;

use teksilo::canvas::EdgeInsets;
use teksilo::core::BindingLevel;
use teksilo::data::{ListModel, SelectionMode, SelectionModel};
use teksilo::prelude::*;
use teksilo::settings::SettingsExt;
use teksilo::widgets::{
    ActivateOn, AspectRatio, Button, ButtonVariant, Center, DragTransferMode, DropTarget, Expand,
    GridSizing, GridView, ListView, Padding, Panel, ScrollBarMode, Segment, SegmentedControl,
    Spacer, TextWidget, TileContext, VStack,
};

use crate::models::ResourceRow;
use crate::resources::FolderResourcesViewModel;
use crate::resources::folder_vm::{FolderResource, FolderView};
use crate::resources::row::{
    self, AttachedItem, DetachFrom, ResourceIcon, list_keys, picture_of, resource_row, row_menu,
};
use crate::shared::thumbnail::{Thumb, thumb_edge, tile_ratio};
use crate::tabs::shared::flow_row::{FlowRow, ROW_LINE_GAP};

/// The width a grid tile starts from, in logical pixels.
const TILE: f32 = 170.0;

/// The pane of `tab`'s folder.
pub fn resources_pane(tab: &crate::tabs::ContentTab) -> Box<dyn Widget> {
    let vm = FolderResourcesViewModel::new(
        tab.app_ctx(),
        tab.ids().work_id.clone(),
        tab.item_id(),
        tab.open_doc.tag_probe.title(),
        tab.resources(),
    );
    Box::new(FolderResourcesPane::new(vm))
}

/// The page: its header, and the resources under it inside a drop target.
pub struct FolderResourcesPane {
    vm: FolderResourcesViewModel,
    root: Option<WidgetId>,
}

impl FolderResourcesPane {
    pub fn new(vm: FolderResourcesViewModel) -> Self {
        Self { vm, root: None }
    }
}

impl std::fmt::Debug for FolderResourcesPane {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FolderResourcesPane")
            .field("vm", &self.vm)
            .finish()
    }
}

/// The remembered layout, as the setting's string.
fn view_setting(ctx: &BuildContext) -> Signal<String> {
    ctx.settings().signal::<String>(
        crate::RESOURCES_FOLDER_VIEW_KEY,
        crate::RESOURCES_FOLDER_VIEW_DEFAULT.to_string(),
    )
}

impl Widget for FolderResourcesPane {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        let setting = view_setting(ctx);
        // The segmented control's index, bridged both ways to the stored string by two
        // guarded effects, so neither writes the other back for ever.
        let index = Signal::new(FolderView::from_setting(&setting.get()).index());
        {
            let index = index.clone();
            ctx.effect(&setting, move |s: &String| {
                index.set_if_changed(FolderView::from_setting(s).index());
            });
        }
        {
            let setting = setting.clone();
            ctx.effect(&index, move |i| {
                let want = FolderView::from_index(*i).as_setting();
                if setting.get() != want {
                    setting.set(want.to_string());
                }
            });
        }

        let header = header(&self.vm, index);
        let drop_vm = self.vm.clone();
        let body = DropTarget::new()
            .accept_external()
            .hint(TextWidget::new(tr!(resources_folder_drop_hint())))
            .on_drop(move |payload, _pos, c| {
                let files: Vec<PathBuf> = payload.files().to_vec();
                if !files.is_empty() {
                    drop_vm.add_paths(c, files);
                    return true;
                }
                let mut text = payload.uris().join("\n");
                if let Some(t) = payload.text() {
                    text.push('\n');
                    text.push_str(t);
                }
                drop_vm.add_dropped_text(c, &text)
            })
            .child(FolderResourcesBody {
                vm: self.vm.clone(),
                view: setting,
                selection: SelectionModel::new(SelectionMode::Single),
                root: None,
            });
        let id = teksu!(ctx => VStack {
            spacing: 0.0
            child: header
            Expand::new() {
                child: body
            }
        });
        self.root = Some(id);
        vec![id]
    }

    fn layout_response(&self, proposal: SizeProposal, ctx: &LayoutContext) -> LayoutResponse {
        self.root
            .and_then(|id| ctx.child_size(id, proposal))
            .map(LayoutResponse::from)
            .unwrap_or_else(|| proposal.resolve(0.0, 0.0).into())
    }

    fn children(&self) -> Vec<WidgetId> {
        self.root.into_iter().collect()
    }
}

/// How many · List / Grid · Add files · Add a web link.
fn header(vm: &FolderResourcesViewModel, index: Signal<usize>) -> impl Widget + use<> {
    let count = {
        let vm = vm.clone();
        vm.version().map(move |_| {
            tr!(resources_folder_count(count = vm.entries().len() as i64)).resolve_now()
        })
    };
    let layout = SegmentedControl::indexed(index)
        .label(tr!(resources_view_label()))
        .segment(Segment::new(tr!(resources_view_list())))
        .segment(Segment::new(tr!(resources_view_grid())));
    let files_vm = vm.clone();
    let link_vm = vm.clone();
    let add_files = Button::new(tr!(resources_add_files()))
        .variant(ButtonVariant::Plain)
        .on_activate_fn(move |c| files_vm.pick_files(c));
    let add_link = Button::new(tr!(resources_add_link()))
        .variant(ButtonVariant::Plain)
        .on_activate_fn(move |c| {
            let url = super::dock::clipboard_address(c).unwrap_or_default();
            link_vm.add_link(c, &url);
        });
    let row = FlowRow::new(10.0, ROW_LINE_GAP)
        .child(
            TextWidget::new(lit!(""))
                .text(count)
                .color(TextRole::Secondary),
        )
        .child(layout)
        .fill(Spacer::new(), 0.0)
        .child(add_files)
        .child(add_link);
    teksu!(
        Panel {
            background: SurfaceRole::Raised
            Padding::symmetric(14.0, 8.0) {
                child: row
            }
        }
    )
}

/// The resources in the layout chosen, rebuilt when a resource, an attachment or the
/// folder's subtree changes, or the layout does.
struct FolderResourcesBody {
    vm: FolderResourcesViewModel,
    view: Signal<String>,
    /// Kept across rebuilds, so a change elsewhere does not lose the row the keyboard
    /// was on.
    selection: SelectionModel,
    root: Option<WidgetId>,
}

impl std::fmt::Debug for FolderResourcesBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FolderResourcesBody").finish()
    }
}

impl Widget for FolderResourcesBody {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        let sid = ctx.self_id();
        let reg = ctx.binding_registry();
        self.vm.version().bind_to(sid, reg, BindingLevel::Rebuild);
        self.vm.work_id().bind_to(sid, reg, BindingLevel::Rebuild);
        self.view.bind_to(sid, reg, BindingLevel::Rebuild);

        let entries = self.vm.entries();
        if entries.is_empty() {
            let id = ctx.add(
                Center::new().child(
                    Padding::symmetric(24.0, 24.0).child(
                        TextWidget::new(tr!(resources_folder_empty()))
                            .style(TextStyleRole::Body)
                            .color(TextRole::Secondary),
                    ),
                ),
            );
            self.root = Some(id);
            return vec![id];
        }
        let items: Rc<std::collections::HashMap<u64, Vec<AttachedItem>>> = Rc::new(
            entries
                .iter()
                .map(|e| (e.row.id, e.items.clone()))
                .collect(),
        );
        let rows: Vec<ResourceRow> = entries.into_iter().map(|e: FolderResource| e.row).collect();
        let model = ListModel::from_vec(rows);
        let view = FolderView::from_setting(&self.view.get());
        let id = match view {
            FolderView::List => {
                let list = self.list(model, items);
                ctx.add(list)
            }
            FolderView::Grid => {
                let grid = self.grid(ctx, model, items);
                ctx.add(grid)
            }
        };
        self.root = Some(id);
        vec![id]
    }

    fn layout_response(&self, proposal: SizeProposal, ctx: &LayoutContext) -> LayoutResponse {
        self.root
            .and_then(|id| ctx.child_size(id, proposal))
            .map(LayoutResponse::from)
            .unwrap_or_else(|| proposal.resolve(0.0, 0.0).into())
    }

    fn children(&self) -> Vec<WidgetId> {
        self.root.into_iter().collect()
    }
}

/// The items a resource is on, for its row's menu.
fn detach_from(items: &std::collections::HashMap<u64, Vec<AttachedItem>>, id: u64) -> DetachFrom {
    DetachFrom::Items(items.get(&id).cloned().unwrap_or_default())
}

impl FolderResourcesBody {
    fn list(
        &self,
        model: ListModel<ResourceRow>,
        items: Rc<std::collections::HashMap<u64, Vec<AttachedItem>>>,
    ) -> impl Widget + use<> {
        let vm = self.vm.resources().clone();
        let open_vm = vm.clone();
        let open_model = model.clone();
        let list = ListView::new(model.clone(), move |_i, row: &ResourceRow, _sel| {
            Box::new(resource_row(row, vm.clone(), detach_from(&items, row.id))) as Box<dyn Widget>
        })
        .selection(self.selection.clone())
        .auto_item_height(64.0)
        .scroll_bar_style(ScrollBarMode::Overlay)
        .activate_on(ActivateOn::DoubleClick)
        // A row dragged into a text becomes a link, or a picture, there.
        .exportable(DragTransferMode::Copy)
        .type_ahead_label(|row: &ResourceRow| row::title_of(row))
        .on_activate(move |idx, ctx| {
            if let Some(row) = open_model.with_item(idx, |r| r.clone()) {
                open_vm.open(ctx, &row);
            }
        });
        let paste_vm = self.vm.clone();
        let keys = list_keys(
            self.vm.resources().clone(),
            model,
            self.selection.clone(),
            Rc::new(move |c, text| paste_vm.add_dropped_text(c, text)),
        );
        Padding::symmetric(12.0, 8.0).child(list.on_key(keys))
    }

    fn grid(
        &self,
        ctx: &BuildContext,
        model: ListModel<ResourceRow>,
        items: Rc<std::collections::HashMap<u64, Vec<AttachedItem>>>,
    ) -> impl Widget + use<> {
        let rt = ctx.app_state::<AsyncRuntimeHandle>().cloned();
        let delegate = move |tc: &TileContext<'_, ResourceRow>| -> Box<dyn Widget> {
            Box::new(ResourceTile {
                row: tc.item.clone(),
                rt: rt.clone(),
                root: None,
            })
        };
        let vm = self.vm.resources().clone();
        let open_vm = vm.clone();
        let (a11y_model, type_model, open_model, menu_model) =
            (model.clone(), model.clone(), model.clone(), model.clone());
        let a11y_items = items.clone();
        GridView::new(model, delegate)
            .sizing(GridSizing::Adaptive {
                min_width: TILE,
                max_width: Some(TILE * 1.5),
                height: TILE,
            })
            .waterfall(TILE)
            .spacing(12.0)
            .content_inset(EdgeInsets::uniform(12.0))
            .selection(self.selection.clone())
            .exportable(DragTransferMode::Copy)
            .a11y_label(tr!(resources_grid_label()).resolve_now())
            .tile_a11y_label(move |i| {
                a11y_model
                    .with_item(i, |r| {
                        row::row_name_with_items(r, a11y_items.get(&r.id).map_or(&[], |v| v))
                    })
                    .unwrap_or_default()
            })
            .type_ahead_label(move |i| type_model.with_item(i, row::title_of).unwrap_or_default())
            .on_tile_activate(move |i, c: &mut EventContext| {
                if let Some(row) = open_model.with_item(i, Clone::clone) {
                    open_vm.open(c, &row);
                }
            })
            .tile_context_menu(move |i, _at, _c| {
                let row = menu_model.with_item(i, Clone::clone)?;
                Some(
                    Box::new(row_menu(&row, vm.clone(), detach_from(&items, row.id)))
                        as Box<dyn Widget>,
                )
            })
    }
}

/// One tile of the grid: a picture's thumbnail, or the type's icon, then the title.
struct ResourceTile {
    row: ResourceRow,
    rt: Option<AsyncRuntimeHandle>,
    root: Option<WidgetId>,
}

impl std::fmt::Debug for ResourceTile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ResourceTile")
            .field("id", &self.row.id)
            .finish()
    }
}

impl Widget for ResourceTile {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        let title = row::title_of(&self.row);
        let detail = row::detail_line(&self.row);
        let badge: Option<TextWidget> = self.row.missing.then(|| {
            TextWidget::new(tr!(resources_not_found()))
                .style(TextStyleRole::Tiny)
                .color(TextRole::Warning)
                .single_line()
        });
        let id = match picture_of(&self.row) {
            Some((path, hash)) => {
                let size = (self.row.width > 0 && self.row.height > 0).then(|| {
                    (
                        u32::try_from(self.row.width).unwrap_or(u32::MAX),
                        u32::try_from(self.row.height).unwrap_or(u32::MAX),
                    )
                });
                let thumb =
                    Thumb::of_file(Some(path), &hash, &title, self.rt.clone(), thumb_edge(TILE));
                teksu!(ctx => VStack {
                    spacing: 4.0
                    AspectRatio::new(tile_ratio(size)) {
                        child: thumb
                    }
                    TextWidget::new(lit!(title)) {
                        style: TextStyleRole::Small
                        single_line
                        overflow: TextOverflow::Ellipsis(EllipsisMode::Trailing)
                    }
                })
            }
            None => teksu!(ctx => Panel {
                background: SurfaceRole::Sunken
                corner_radius: 4.0
                Padding::uniform(10.0) {
                    VStack {
                        spacing: 6.0
                        Center {
                            child: (ResourceIcon::new(&self.row).side(40.0))
                        }
                        TextWidget::new(lit!(title)) {
                            style: TextStyleRole::BodyBold
                            max_lines: 2
                            overflow: TextOverflow::Ellipsis(EllipsisMode::Trailing)
                        }
                        TextWidget::new(lit!(detail)) {
                            style: TextStyleRole::Tiny
                            color: TextRole::Secondary
                            single_line
                            overflow: TextOverflow::Ellipsis(EllipsisMode::Middle)
                        }
                        child_opt: badge
                    }
                }
            }),
        };
        self.root = Some(id);
        vec![id]
    }

    fn layout_response(&self, proposal: SizeProposal, ctx: &LayoutContext) -> LayoutResponse {
        self.root
            .and_then(|id| ctx.child_size(id, proposal))
            .map(LayoutResponse::from)
            .unwrap_or_else(|| proposal.resolve(0.0, 0.0).into())
    }

    fn children(&self) -> Vec<WidgetId> {
        self.root.into_iter().collect()
    }
}

#[cfg(test)]
mod tests;
