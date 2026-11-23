// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The **Gallery** segment of a notes folder: the pictures in its notes, as thumbnails in a
//! waterfall grid.
//!
//! A header (where am I · search · First / All images · Nested / Flat · Notes without
//! images · order, over the Overview's tag and status chips and the image size slider)
//! sits above a virtualized [`GridView`] in its waterfall layout, so each thumbnail keeps
//! its picture's proportions instead of being cropped into a uniform cell.
//!
//! The search field, the chips and the order are the Overview's own signals
//! ([`crate::gallery`] explains why): narrowing one view narrows the other.
//!
//! A tile is opened by a double click or Enter: a picture in the image viewer, over the
//! pictures the Gallery shows, in its order (the arrow keys then walk them); a text note
//! in its tab; a subfolder, in Nested, by entering it. A right click offers the other
//! door (open the note a picture is in).
//!
//! A picture resource's tile says it is one (a badge under it, and the name a screen
//! reader hears), opens in the viewer with the others, and its right click offers what
//! the Resources dock offers for a resource, after the Gallery's own two doors.
//!
//! A Gallery with no tile says why ([`GalleryEmpty`]): *Clear the filters* when the
//! filters hide every picture, else *Add images…*, and a drop of pictures from outside,
//! which become resources of the folder shown.
//!
//! The thumbnails are decoded off the interface thread and cached
//! ([`crate::shared::thumbnail`]); a tile shows a placeholder of its final size until
//! its picture arrives, so the columns do not move when it does.

use std::rc::Rc;

use teksilo::canvas::EdgeInsets;
use teksilo::core::BindingLevel;
use teksilo::data::{ListModel, SortDirection, TreeDataSource};
use teksilo::prelude::*;
use teksilo::settings::SettingsExt;
use teksilo::tokens::HAlignment;
use teksilo::widgets::tooltip::TooltipContent;
use teksilo::widgets::{
    AspectRatio, Badge, Breadcrumb, BreadcrumbItem, Button, ComboBox, DropTarget, Expand,
    GridSizing, GridView, HStack, IconButton, MaxSize, MenuItem, MenuList, Padding, Panel,
    SearchField, Segment, SegmentedControl, Shrinkable, Slider, Spacer, TextWidget, TileContext,
    VStack,
};

use crate::gallery::{
    GalleryEmpty, GalleryOptions, GalleryPicture, GalleryTile, GalleryViewModel, pictures_among,
    tiles,
};
use crate::intents::AppIntent;
use crate::models::{COL_STATUS, COL_TITLE};
use crate::resources::file_kind::VIEWABLE_PICTURE_EXTENSIONS;
use crate::resources::row::{AttachedItem, DetachFrom, resource_actions};
use crate::resources::{AddTarget, ResourcesViewModel};
use crate::shared::thumbnail::{Thumb, thumb_edge, tile_ratio};
use crate::tabs::shared::flow_row::{FILTER_MIN_WIDTH, FlowRow, ROW_LINE_GAP};

/// The pane, or an empty column for a tab with no Overview (never reached: the segment
/// exists only on notes folders, which always have one).
pub fn gallery_pane(tab: &super::ContentTab) -> Box<dyn Widget> {
    let Some(overview) = tab.overview().cloned() else {
        return Box::new(VStack::new());
    };
    let resources = tab.resources();
    let vm = GalleryViewModel::new(
        tab.app_ctx(),
        tab.ids().work_id.clone(),
        overview,
        tab.docs(),
    )
    .with_resources(
        resources.session().model().clone(),
        tab.open_doc.tag_probe.title(),
    );
    Box::new(GalleryPane {
        vm,
        resources,
        tags: tab.tags(),
        statuses: tab.statuses(),
        root: None,
    })
}

/// The Gallery's settings, app-wide like the Corkboard's: what a writer chose in one
/// folder is how they want to look at pictures.
#[derive(Clone)]
struct GallerySettings {
    all_images: Signal<bool>,
    nested: Signal<bool>,
    text_notes: Signal<bool>,
    tile_size: Signal<f32>,
}

impl GallerySettings {
    fn read(ctx: &BuildContext) -> Self {
        Self {
            all_images: ctx.settings().signal(
                crate::GALLERY_ALL_IMAGES_KEY,
                crate::GALLERY_ALL_IMAGES_DEFAULT,
            ),
            nested: ctx
                .settings()
                .signal(crate::GALLERY_NESTED_KEY, crate::GALLERY_NESTED_DEFAULT),
            text_notes: ctx.settings().signal(
                crate::GALLERY_TEXT_NOTES_KEY,
                crate::GALLERY_TEXT_NOTES_DEFAULT,
            ),
            tile_size: ctx.settings().signal(
                crate::GALLERY_TILE_SIZE_KEY,
                crate::GALLERY_TILE_SIZE_DEFAULT,
            ),
        }
    }

    fn options(&self) -> GalleryOptions {
        GalleryOptions {
            all_images: self.all_images.get(),
            nested: self.nested.get(),
            text_notes: self.text_notes.get(),
        }
    }
}

struct GalleryPane {
    vm: GalleryViewModel,
    /// The window's Resources view-model: a resource tile's menu and its Open in another
    /// app in the viewer go through it.
    resources: ResourcesViewModel,
    tags: crate::tags::TagsViewModel,
    statuses: crate::statuses::StatusesViewModel,
    root: Option<WidgetId>,
}

impl std::fmt::Debug for GalleryPane {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GalleryPane").field("vm", &self.vm).finish()
    }
}

impl Widget for GalleryPane {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        // The Overview's model is what tells this pane the folder changed (a note added,
        // renamed, moved, trashed, its text saved). Wiring it is idempotent and costs
        // nothing when the Overview segment has already done it.
        self.vm.overview().wire(ctx);
        let settings = GallerySettings::read(ctx);
        let header = gallery_header(
            &self.vm,
            &settings,
            self.tags.clone(),
            self.statuses.clone(),
        );
        let id = teksu!(ctx => VStack {
            spacing: 0.0
            child: header
            Expand::new() {
                child: (GalleryGrid {
                    vm: self.vm.clone(),
                    resources: self.resources.clone(),
                    settings,
                    root: None,
                })
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

fn gallery_header(
    vm: &GalleryViewModel,
    settings: &GallerySettings,
    tags: crate::tags::TagsViewModel,
    statuses: crate::statuses::StatusesViewModel,
) -> impl Widget {
    let overview = vm.overview();
    let top = FlowRow::new(10.0, ROW_LINE_GAP)
        .child(GalleryTrail {
            vm: vm.clone(),
            root: None,
        })
        .fill(
            teksu!(
                SearchField::new(overview.search_query_signal()) {
                    label: tr!(gallery_search_label())
                    placeholder: tr!(overview_search_placeholder())
                }
            ),
            FILTER_MIN_WIDTH,
        )
        .child(BoolSegments {
            value: settings.all_images.clone(),
            off: tr!(gallery_mode_first()),
            on: tr!(gallery_mode_all()),
            off_tip: tr!(gallery_mode_first_hint()),
            on_tip: tr!(gallery_mode_all_hint()),
            true_first: false,
            index: Signal::new(0),
            root: None,
        })
        .child(BoolSegments {
            value: settings.nested.clone(),
            off: tr!(corkboard_view_flat()),
            on: tr!(corkboard_view_nested()),
            off_tip: tr!(gallery_view_flat_hint()),
            on_tip: tr!(gallery_view_nested_hint()),
            true_first: true,
            index: Signal::new(0),
            root: None,
        })
        // An icon toggle with a rich tooltip, as the search panel's options are: the
        // header's other choices are segments, and a labelled checkbox among them would be
        // the only one in a toolbar anywhere in the app.
        .child(
            IconButton::new(crate::icons::gallery::text_notes())
                .toolbar()
                .toggle(settings.text_notes.clone())
                .rich_tooltip_content(TooltipContent::new(
                    "gallery-text-notes-hint",
                    tr!(gallery_text_notes_hint()),
                )),
        )
        .child(GallerySortControl {
            sort: overview.sort_signal(),
            selected: Signal::new(None),
            root: None,
        });

    let size = teksu!(
        HStack {
            spacing: 10.0
            TextWidget::new(tr!(gallery_tile_size())) {
                color: TextRole::Secondary
            }
            Shrinkable::new() {
                min_width: 80.0
                MaxSize::width(170.0) {
                    Slider::new(settings.tile_size.clone(), crate::GALLERY_TILE_SIZE_MIN, crate::GALLERY_TILE_SIZE_MAX) {
                        step: crate::GALLERY_TILE_SIZE_STEP
                        label: tr!(gallery_tile_size())
                        tooltip: tr!(gallery_scope_hint())
                    }
                }
            }
        }
    );
    let bottom = FlowRow::new(10.0, ROW_LINE_GAP).child(size);

    teksu!(
        Panel {
            background: SurfaceRole::Raised
            Padding::symmetric(14.0, 8.0) {
                VStack {
                    spacing: 0.0
                    child: top
                    child: crate::tabs::Boxed::new(super::overview::tag_filter_row(overview, tags))
                    child: crate::tabs::Boxed::new(super::overview::status_filter_row(overview, statuses))
                    child: bottom
                }
            }
        }
    )
}

/// Where the Gallery is, as a breadcrumb: shown only once a subfolder has been entered.
struct GalleryTrail {
    vm: GalleryViewModel,
    root: Option<WidgetId>,
}

impl std::fmt::Debug for GalleryTrail {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GalleryTrail").finish()
    }
}

impl Widget for GalleryTrail {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        let trail = self.vm.trail();
        trail.bind_to(ctx.self_id(), ctx.binding_registry(), BindingLevel::Rebuild);
        let trail = trail.get();
        if trail.is_empty() {
            self.root = None;
            return Vec::new();
        }
        let back = self.vm.clone();
        let mut crumbs = Breadcrumb::new()
            .label(tr!(gallery_breadcrumb_label()))
            .item(
                BreadcrumbItem::new(tr!(gallery_breadcrumb_root()))
                    .on_activate_fn(move |_| back.leave_to(0)),
            );
        let last = trail.len() - 1;
        for (i, (_, title)) in trail.iter().enumerate() {
            if i == last {
                crumbs = crumbs.item(BreadcrumbItem::current(lit!(title.clone())));
            } else {
                let vm = self.vm.clone();
                crumbs = crumbs.item(
                    BreadcrumbItem::new(lit!(title.clone()))
                        .on_activate_fn(move |_| vm.leave_to(i + 1)),
                );
            }
        }
        let id = ctx.add(crumbs);
        self.root = Some(id);
        vec![id]
    }

    fn layout_response(&self, p: SizeProposal, ctx: &LayoutContext) -> LayoutResponse {
        self.root
            .and_then(|id| ctx.child_size(id, p))
            .unwrap_or(Size::ZERO)
            .into()
    }

    fn children(&self) -> Vec<WidgetId> {
        self.root.into_iter().collect()
    }
}

/// A two-segment control over a `bool` setting.
struct BoolSegments {
    value: Signal<bool>,
    off: teksilo::i18n::LocalizedString,
    on: teksilo::i18n::LocalizedString,
    /// What each choice shows, as its tooltip.
    off_tip: teksilo::i18n::LocalizedString,
    on_tip: teksilo::i18n::LocalizedString,
    /// Show the `true` choice first, as the Corkboard puts Nested before Flat.
    true_first: bool,
    index: Signal<usize>,
    root: Option<WidgetId>,
}

impl std::fmt::Debug for BoolSegments {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BoolSegments").finish()
    }
}

impl Widget for BoolSegments {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        let true_first = self.true_first;
        let index_of = move |v: bool| usize::from(v != true_first);
        self.index.set(index_of(self.value.get()));
        {
            let index = self.index.clone();
            ctx.effect(&self.value, move |v| {
                index.set_if_changed(index_of(*v));
            });
        }
        {
            let value = self.value.clone();
            ctx.effect(&self.index, move |i| {
                value.set_if_changed((*i == 0) == true_first);
            });
        }
        let off = Segment::new(self.off.clone()).tooltip(self.off_tip.clone());
        let on = Segment::new(self.on.clone()).tooltip(self.on_tip.clone());
        let (first, second) = if true_first { (on, off) } else { (off, on) };
        let ctrl = SegmentedControl::indexed(self.index.clone())
            .segment(first)
            .segment(second);
        let id = ctx.add(ctrl);
        self.root = Some(id);
        vec![id]
    }

    fn layout_response(&self, p: SizeProposal, ctx: &LayoutContext) -> LayoutResponse {
        self.root
            .and_then(|id| ctx.child_size(id, p))
            .unwrap_or_else(|| p.resolve(0.0, 0.0))
            .into()
    }

    fn children(&self) -> Vec<WidgetId> {
        self.root.into_iter().collect()
    }
}

/// The orders the Gallery offers, each one the Overview's sort under another name.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum GallerySort {
    Binder,
    TitleAscending,
    TitleDescending,
    Status,
}

impl GallerySort {
    const ALL: [Self; 4] = [
        Self::Binder,
        Self::TitleAscending,
        Self::TitleDescending,
        Self::Status,
    ];

    /// The Overview's sort as one of these, `None` for a column the Gallery has no name
    /// for (a word count sorted from the table): the control then says the order is the
    /// Overview's.
    fn of(sort: &Option<(String, SortDirection)>) -> Option<Self> {
        match sort {
            None => Some(Self::Binder),
            Some((col, SortDirection::Ascending)) if col == COL_TITLE => Some(Self::TitleAscending),
            Some((col, SortDirection::Descending)) if col == COL_TITLE => {
                Some(Self::TitleDescending)
            }
            Some((col, SortDirection::Ascending)) if col == COL_STATUS => Some(Self::Status),
            Some(_) => None,
        }
    }

    fn sort(self) -> Option<(String, SortDirection)> {
        match self {
            Self::Binder => None,
            Self::TitleAscending => Some((COL_TITLE.to_string(), SortDirection::Ascending)),
            Self::TitleDescending => Some((COL_TITLE.to_string(), SortDirection::Descending)),
            Self::Status => Some((COL_STATUS.to_string(), SortDirection::Ascending)),
        }
    }

    fn label(&self) -> teksilo::i18n::LocalizedString {
        match self {
            Self::Binder => tr!(gallery_sort_binder()),
            Self::TitleAscending => tr!(gallery_sort_title_ascending()),
            Self::TitleDescending => tr!(gallery_sort_title_descending()),
            Self::Status => tr!(gallery_sort_status()),
        }
    }
}

/// The order, bound both ways to the Overview's sort signal.
struct GallerySortControl {
    sort: Signal<Option<(String, SortDirection)>>,
    selected: Signal<Option<GallerySort>>,
    root: Option<WidgetId>,
}

impl std::fmt::Debug for GallerySortControl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GallerySortControl").finish()
    }
}

impl Widget for GallerySortControl {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        self.selected.set(GallerySort::of(&self.sort.get()));
        {
            let selected = self.selected.clone();
            ctx.effect(&self.sort, move |sort| {
                selected.set_if_changed(GallerySort::of(sort));
            });
        }
        {
            let sort = self.sort.clone();
            ctx.effect(&self.selected, move |chosen| {
                if let Some(chosen) = chosen
                    && GallerySort::of(&sort.get()) != Some(*chosen)
                {
                    sort.set(chosen.sort());
                }
            });
        }
        let combo = ComboBox::from_items(
            GallerySort::ALL,
            self.selected.clone(),
            |s: &GallerySort| s.label(),
        )
        .label(tr!(gallery_sort_label()))
        .placeholder(tr!(gallery_sort_overview()));
        let id = ctx.add(combo);
        self.root = Some(id);
        vec![id]
    }

    fn layout_response(&self, p: SizeProposal, ctx: &LayoutContext) -> LayoutResponse {
        self.root
            .and_then(|id| ctx.child_size(id, p))
            .unwrap_or_else(|| p.resolve(0.0, 0.0))
            .into()
    }

    fn children(&self) -> Vec<WidgetId> {
        self.root.into_iter().collect()
    }
}

/// The grid, rebuilt when the folder's rows change, an open note's pictures change, a
/// resource or an attachment changes, a subfolder is entered or left, or a switch flips.
struct GalleryGrid {
    vm: GalleryViewModel,
    resources: ResourcesViewModel,
    settings: GallerySettings,
    root: Option<WidgetId>,
}

impl std::fmt::Debug for GalleryGrid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GalleryGrid").finish()
    }
}

impl Widget for GalleryGrid {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        let sid = ctx.self_id();
        let reg = ctx.binding_registry();
        self.vm
            .overview()
            .rows()
            .version_signal()
            .bind_to(sid, reg, BindingLevel::Rebuild);
        self.vm.revision().bind_to(sid, reg, BindingLevel::Rebuild);
        if let Some(version) = self.vm.resources_version() {
            version.bind_to(sid, reg, BindingLevel::Rebuild);
        }
        self.vm.trail().bind_to(sid, reg, BindingLevel::Rebuild);
        self.vm.work_id().bind_to(sid, reg, BindingLevel::Rebuild);
        self.settings
            .all_images
            .bind_to(sid, reg, BindingLevel::Rebuild);
        self.settings
            .nested
            .bind_to(sid, reg, BindingLevel::Rebuild);
        self.settings
            .text_notes
            .bind_to(sid, reg, BindingLevel::Rebuild);
        // The empty page reads the filters themselves, not only the rows they shape: it
        // says whether one is on.
        let filters = self.vm.overview().filters();
        filters.query.bind_to(sid, reg, BindingLevel::Rebuild);
        filters.tag_filter.bind_to(sid, reg, BindingLevel::Rebuild);
        filters
            .status_filter
            .bind_to(sid, reg, BindingLevel::Rebuild);

        let options = self.settings.options();
        let notes = self.vm.notes();
        let tiles = tiles::build_tiles(&notes, options);
        if tiles.is_empty() {
            let reason = self.vm.empty_reason(&notes, options);
            let id = ctx.add_boxed(empty_page(reason, &self.vm, &self.resources));
            self.root = Some(id);
            return vec![id];
        }

        let (entries, entry_of) = self.vm.viewer_entries(&tiles);
        let entries = Rc::new(entries);
        let entry_of = Rc::new(entry_of);
        let model = ListModel::from_vec(tiles);
        let media_dir = self.vm.media_dir();
        let rt = ctx.app_state::<AsyncRuntimeHandle>().cloned();
        let size = self.settings.tile_size.clone();

        let delegate = {
            let size = size.clone();
            move |tc: &TileContext<'_, GalleryTile>| -> Box<dyn Widget> {
                Box::new(TileView {
                    tile: tc.item.clone(),
                    media_dir: media_dir.clone(),
                    rt: rt.clone(),
                    edge: thumb_edge(size.get()),
                    root: None,
                })
            }
        };
        let sizing = size.map(|w| GridSizing::Adaptive {
            min_width: *w,
            max_width: Some(*w * 1.5),
            height: *w,
        });

        let a11y_model = model.clone();
        let type_model = model.clone();
        let activate_model = model.clone();
        let menu_model = model.clone();
        let activate_vm = self.vm.clone();
        let activate_entries = entries.clone();
        let activate_at = entry_of.clone();
        let menu_entries = entries.clone();
        let menu_at = entry_of.clone();
        let menu_dir = self.vm.media_dir();
        let view_dir = self.vm.media_dir();
        let menu_resources = self.resources.clone();
        // A picture resource in the viewer opens elsewhere through the Resources feature.
        let opener = self.resources.viewer_opener();
        let menu_opener = opener.clone();

        let grid = GridView::new(model, delegate)
            .sizing(sizing)
            .waterfall(size.get())
            .spacing(12.0)
            .content_inset(EdgeInsets::uniform(12.0))
            .a11y_label(tr!(gallery_grid_label()).resolve_now())
            .tile_a11y_label(move |i| a11y_model.with_item(i, tile_a11y_label).unwrap_or_default())
            .type_ahead_label(move |i| {
                type_model
                    .with_item(i, |t| t.title().to_string())
                    .unwrap_or_default()
            })
            .on_tile_activate(move |i, ectx: &mut EventContext| {
                let Some(tile) = activate_model.with_item(i, Clone::clone) else {
                    return;
                };
                match tile {
                    GalleryTile::Picture { .. } => {
                        if let Some(Some(start)) = activate_at.get(i) {
                            crate::picture_viewer::present_with_opener(
                                ectx,
                                activate_entries.as_ref().clone(),
                                *start,
                                view_dir.clone(),
                                opener.clone(),
                            );
                        }
                    }
                    GalleryTile::Text { item_id, title, .. } => {
                        ectx.send_intent(AppIntent::OpenItem { item_id, title });
                    }
                    GalleryTile::Folder { item_id, title, .. } => {
                        activate_vm.enter(item_id, title);
                    }
                }
            })
            .tile_context_menu(move |i, _at, _ectx| {
                let tile = menu_model.with_item(i, Clone::clone)?;
                let item_id = tile.item_id();
                let title = tile.title().to_string();
                let mut menu = MenuList::new();
                if let Some(Some(start)) = menu_at.get(i) {
                    let entries = menu_entries.clone();
                    let start = *start;
                    let dir = menu_dir.clone();
                    let opener = menu_opener.clone();
                    menu = menu.item(MenuItem::new(tr!(image_menu_view())).on_activate_fn(
                        move |c| {
                            crate::picture_viewer::present_with_opener(
                                c,
                                entries.as_ref().clone(),
                                start,
                                dir.clone(),
                                opener.clone(),
                            )
                        },
                    ));
                }
                if !matches!(tile, GalleryTile::Folder { .. }) {
                    let open_title = title.clone();
                    menu = menu.item(
                        MenuItem::new(tr!(picture_viewer_open_note())).on_activate_fn(move |c| {
                            c.send_intent(AppIntent::OpenItem {
                                item_id,
                                title: open_title.clone(),
                            })
                        }),
                    );
                }
                // What the dock offers for a resource, taken off the item this tile is
                // filed under.
                if let Some(row) = tile.resource() {
                    menu = resource_actions(
                        menu.separator(),
                        row,
                        menu_resources.clone(),
                        DetachFrom::Items(vec![AttachedItem { id: item_id, title }]),
                    );
                }
                Some(Box::new(menu) as Box<dyn Widget>)
            });
        let id = ctx.add(grid);
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

/// What a Gallery with no tile shows, for `reason`: a line saying why, and the way out.
///
/// Filtered, the way out is *Clear the filters*. Otherwise it is *Add images…*, and the page
/// takes pictures dropped from outside too, each attached to the folder shown.
fn empty_page(
    reason: GalleryEmpty,
    vm: &GalleryViewModel,
    resources: &ResourcesViewModel,
) -> Box<dyn Widget> {
    let message = match reason {
        GalleryEmpty::NoMatch => tr!(gallery_no_match()),
        GalleryEmpty::TextNotesHidden => tr!(gallery_empty_text_notes_hint()),
        GalleryEmpty::NoPictures => tr!(gallery_empty()),
    };
    let action = if reason.offers_adding() {
        let (vm, resources) = (vm.clone(), resources.clone());
        Button::new(tr!(gallery_add_images()))
            .tooltip(tr!(gallery_add_images_hint()))
            .on_activate_fn(move |c| resources.pick_pictures_for(c, vec![vm.current_folder()]))
    } else {
        let vm = vm.clone();
        Button::new(tr!(gallery_clear_filters())).on_activate_fn(move |_| vm.clear_filters())
    };
    // A column, not a `Center`: a `Center` proposes no width, so the line would never
    // wrap and would run off a narrow pane on both sides.
    let page = teksu!(
        VStack {
            alignment: HAlignment::Center
            Spacer::new()
            Padding::symmetric(24.0, 24.0) {
                VStack {
                    spacing: 12.0
                    alignment: HAlignment::Center
                    TextWidget::new(message) {
                        style: TextStyleRole::Body
                        color: TextRole::Secondary
                    }
                    child: action
                }
            }
            Spacer::new()
        }
    );
    if !reason.offers_adding() {
        return Box::new(page);
    }
    let (vm, resources) = (vm.clone(), resources.clone());
    Box::new(
        DropTarget::new()
            .accept_external_extensions(VIEWABLE_PICTURE_EXTENSIONS)
            .hint(TextWidget::new(tr!(gallery_drop_hint())))
            .on_drop(move |payload, _pos, c| {
                let pictures = pictures_among(payload.files().to_vec());
                if pictures.is_empty() {
                    return false;
                }
                resources.add_paths_to(c, vec![vm.current_folder()], pictures, AddTarget::Dock);
                true
            })
            .child(page),
    )
}

/// What a screen reader hears for a tile: what it is and whose.
fn tile_a11y_label(tile: &GalleryTile) -> String {
    match tile {
        GalleryTile::Picture {
            title,
            picture,
            more,
            ..
        } => {
            let alt = picture.image.alt.trim();
            let what = if picture.is_resource() {
                tr!(gallery_tile_resource(
                    title = title.clone(),
                    description = alt.to_string()
                ))
                .resolve_now()
            } else if alt.is_empty() {
                tr!(gallery_tile_picture(title = title.clone())).resolve_now()
            } else {
                tr!(gallery_tile_picture_described(
                    title = title.clone(),
                    description = alt.to_string()
                ))
                .resolve_now()
            };
            if *more > 0 {
                format!(
                    "{what}, {}",
                    tr!(gallery_more_images(count = *more as i64)).resolve_now()
                )
            } else {
                what
            }
        }
        GalleryTile::Text { title, .. } => {
            tr!(gallery_tile_text(title = title.clone())).resolve_now()
        }
        GalleryTile::Folder {
            title, pictures, ..
        } => tr!(gallery_tile_folder(
            title = title.clone(),
            count = *pictures as i64
        ))
        .resolve_now(),
    }
}

/// One tile, sized height-for-width so the waterfall can stack it.
struct TileView {
    tile: GalleryTile,
    media_dir: std::path::PathBuf,
    rt: Option<AsyncRuntimeHandle>,
    edge: u32,
    root: Option<WidgetId>,
}

impl std::fmt::Debug for TileView {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TileView")
            .field("item_id", &self.tile.item_id())
            .finish()
    }
}

impl TileView {
    fn thumb(&self, picture: &GalleryPicture, backdrop: bool) -> Thumb {
        let thumb = match &picture.resource {
            Some(row) => Thumb::of_file(
                (!row.missing).then(|| row.path.clone()).flatten(),
                &row.content_hash,
                &picture.image.alt,
                self.rt.clone(),
                self.edge,
            ),
            None => Thumb::of_picture(&self.media_dir, &picture.image, self.rt.clone(), self.edge),
        };
        thumb.backdrop(backdrop)
    }
}

impl Widget for TileView {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        let id = match &self.tile {
            GalleryTile::Picture {
                title,
                picture,
                more,
                ..
            } => {
                let badge: Option<Badge> =
                    (*more > 0).then(|| Badge::new(tr!(gallery_more_badge(count = *more as i64))));
                // A resource says so, under its picture: it is filed on the note, not in it.
                let resource: Option<Badge> = picture
                    .is_resource()
                    .then(|| Badge::new(tr!(gallery_resource_badge())));
                let thumb = self.thumb(picture, false);
                teksu!(ctx => VStack {
                    spacing: 4.0
                    AspectRatio::new(tile_ratio(picture.image.size)) {
                        child: thumb
                    }
                    HStack {
                        spacing: 6.0
                        Expand::horizontal() {
                            TextWidget::new(lit!(title.clone())) {
                                style: TextStyleRole::Small
                                single_line
                            }
                        }
                        child_opt: resource
                        child_opt: badge
                    }
                })
            }
            GalleryTile::Text { title, excerpt, .. } => teksu!(ctx => Panel {
                background: SurfaceRole::Sunken
                Padding::uniform(10.0) {
                    VStack {
                        spacing: 4.0
                        TextWidget::new(lit!(title.clone())) {
                            style: TextStyleRole::BodyBold
                            single_line
                        }
                        TextWidget::new(lit!(excerpt.clone())) {
                            style: TextStyleRole::Small
                            color: TextRole::Secondary
                            max_lines: 8
                        }
                    }
                }
            }),
            GalleryTile::Folder {
                title,
                previews,
                pictures,
                ..
            } => {
                let cells: Vec<Box<dyn Widget>> = (0..tiles::FOLDER_PREVIEWS)
                    .map(|i| -> Box<dyn Widget> {
                        match previews.get(i) {
                            Some(picture) => Box::new(self.thumb(picture, true)),
                            None => Box::new(Panel::new().background(SurfaceRole::Sunken)),
                        }
                    })
                    .collect();
                let quad = QuadGrid {
                    pending: cells,
                    ids: Vec::new(),
                };
                teksu!(ctx => Panel {
                    background: SurfaceRole::Raised
                    Padding::uniform(6.0) {
                        VStack {
                            spacing: 4.0
                            child: quad
                            TextWidget::new(lit!(title.clone())) {
                                style: TextStyleRole::BodyBold
                                single_line
                            }
                            TextWidget::new(tr!(gallery_folder_count(count = *pictures as i64))) {
                                style: TextStyleRole::Tiny
                                color: TextRole::Secondary
                                single_line
                            }
                        }
                    }
                })
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

/// Four cells in a square, two by two: a folder tile's previews.
///
/// Laid out by hand, height from width, because stacks of `Expand` and `AspectRatio`
/// inside a tile the waterfall measures height-for-width collapse to no height at all.
struct QuadGrid {
    pending: Vec<Box<dyn Widget>>,
    ids: Vec<WidgetId>,
}

/// The gap between a folder tile's previews.
const QUAD_GAP: f32 = 2.0;

impl std::fmt::Debug for QuadGrid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("QuadGrid").finish()
    }
}

impl Widget for QuadGrid {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        self.ids = std::mem::take(&mut self.pending)
            .into_iter()
            .map(|cell| ctx.add_boxed(cell))
            .collect();
        self.ids.clone()
    }

    fn layout_response(&self, proposal: SizeProposal, _ctx: &LayoutContext) -> LayoutResponse {
        let w = proposal.width.unwrap_or(160.0).max(0.0);
        Size::new(w, w).into()
    }

    fn place_children(
        &self,
        bounds: Rect,
        _proposal: SizeProposal,
        children: &mut [teksilo::core::widget::WidgetPlacement],
        _ctx: &LayoutContext,
    ) {
        let side = ((bounds.width - QUAD_GAP) / 2.0).max(0.0);
        for (i, child) in children.iter_mut().enumerate() {
            let (col, row) = ((i % 2) as f32, (i / 2) as f32);
            child.origin = teksilo::canvas::Point::new(
                bounds.x + col * (side + QUAD_GAP),
                bounds.y + row * (side + QUAD_GAP),
            );
            child.size = Size::new(side, side);
        }
    }

    fn children(&self) -> Vec<WidgetId> {
        self.ids.clone()
    }

    fn clips_children(&self) -> bool {
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_gallery_orders_are_the_overviews_sorts() {
        for order in GallerySort::ALL {
            assert_eq!(GallerySort::of(&order.sort()), Some(order));
        }
        // A sort the Gallery has no name for is shown as the Overview's.
        assert_eq!(
            GallerySort::of(&Some((
                crate::models::COL_OWN_WORDS.to_string(),
                SortDirection::Ascending
            ))),
            None
        );
    }
}
