//! Container tab for a `Folder/Chapter` (a chapter whose extent is its child
//! scenes). A `SegmentedControl` selects the view: **Synopsis** (shared with the
//! other folder tabs) and **Full Chapter** — a Scrivener-"Scrivenings"-style
//! continuous, editable manuscript of every scene belonging to the chapter, each
//! with its own header + options menu, plus an add-scene action. Corkboard and
//! Overview are 🚧 future segments (FEATURES.md), shown disabled.

use std::rc::Rc;

use bastyde::prelude::*;
use bastyde::widgets::{
    Button, Divider, Expand, HStack, IconButton, IconWidget, MenuItem, MenuList, PopoverIconButton,
    Repeater, ScrollArea, Segment, SegmentedControl, Spacer, Switcher, TextWidget, VStack,
};

use crate::models::SceneRow;
use crate::view_models::ChapterViewModel;

use super::{ContentTab, shared};

/// Pencil icon for the inline rename affordances (no built-in "edit" icon).
const EDIT_SVG: &str = include_str!("../../resources/icons/edit.svg");

pub fn render(tab: &ContentTab) -> Box<dyn Widget> {
    let bar = SegmentedControl::new(tab.segment.clone())
        .segment(Segment::new(tr!(synopsis())))
        .segment(Segment::new(tr!(full_chapter())))
        .segment(Segment::new(tr!(corkboard())).disabled(true))
        .segment(Segment::new(tr!(overview())).disabled(true));
    let content = Switcher::new(tab.segment.clone())
        .child(shared::folder_synopsis_pane(tab))
        .child(full_chapter_pane(tab));

    let col = VStack::new()
        .spacing(8.0)
        .child(shared::vspace(10.0))
        .child(shared::centered(bar, &tab.column_width))
        // Fill the remaining height so the selected segment (esp. the Full
        // Chapter ScrollArea) gets a bounded viewport to fill.
        .child(Expand::new().child(content));
    shared::tab_backdrop(col)
}

/// The scrollable manuscript: chapter header, one editor per scene, add-scene.
/// Structurally identical to the single-scene tab (an outer `ScrollArea` over a
/// flowing `VStack`) so the flowing writing columns size correctly; a zero-size
/// `WireOnBuild` child subscribes the view-model when this pane first mounts.
fn full_chapter_pane(tab: &ContentTab) -> impl Widget {
    let cw = tab.column_width.clone();
    let mut col = VStack::new().spacing(0.0);
    if let Some(vm) = tab.chapter.clone() {
        let mark_dirty: Rc<dyn Fn()> = Rc::new(tab.mark_dirty_fn());
        let factory = {
            let vm = vm.clone();
            let cw = cw.clone();
            let md = mark_dirty.clone();
            move |row: &SceneRow| -> Box<dyn Widget> {
                Box::new(scene_row(&vm, row.item_id, &cw, &md))
            }
        };
        col = col
            .child(WireOnBuild::new(vm.clone()))
            .child(shared::vspace(12.0))
            .child(shared::centered(chapter_header(&vm), &cw))
            .child(shared::vspace(4.0))
            .child(Repeater::new(vm.scenes(), factory))
            .child(shared::vspace(10.0))
            .child(shared::centered(add_scene_button(&vm), &cw))
            .child(shared::vspace(28.0));
    }
    ScrollArea::new().child(col)
}

/// Zero-size child that wires the chapter view-model on build (subscribing the
/// scene list + per-scene metadata) — the one place inside the pane's widget
/// tree that gets a `BuildContext`. `wire` is idempotent.
struct WireOnBuild {
    vm: ChapterViewModel,
}

impl WireOnBuild {
    fn new(vm: ChapterViewModel) -> Self {
        Self { vm }
    }
}

impl std::fmt::Debug for WireOnBuild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WireOnBuild").finish()
    }
}

impl Widget for WireOnBuild {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        self.vm.wire(ctx);
        Vec::new()
    }

    fn layout_response(&self, _proposal: SizeProposal, _ctx: &LayoutContext) -> LayoutResponse {
        Size::new(0.0, 0.0).into()
    }
}

/// Chapter title + a flat rename (pencil) button.
fn chapter_header(vm: &ChapterViewModel) -> impl Widget {
    let rename_vm = vm.clone();
    HStack::new()
        .child(
            TextWidget::new(lit!(""))
                .text(vm.chapter_title())
                .style(TextStyleRole::BodyBold)
                .color(TextRole::Primary),
        )
        .child(Spacer::new())
        .child(
            IconButton::new(IconWidget::from_svg(EDIT_SVG).icon_size(14.0))
                .embedded()
                .tooltip(tr!(rename_chapter()))
                .on_activate_fn(move |ctx| rename_vm.begin_rename_chapter(ctx)),
        )
}

fn add_scene_button(vm: &ChapterViewModel) -> impl Widget {
    let vm = vm.clone();
    HStack::new()
        .child(Spacer::new())
        .child(Button::new(tr!(add_scene())).on_activate_fn(move |ctx| vm.begin_add_scene(ctx)))
}

/// One scene: a separator, a header (name / label / options menu), and the
/// scene's flowing editor (reuses the single-scene writing column).
fn scene_row(
    vm: &ChapterViewModel,
    id: u64,
    column_width: &Signal<f32>,
    mark_dirty: &Rc<dyn Fn()>,
) -> impl Widget {
    let scene = vm.scene(id);
    let md = mark_dirty.clone();
    let on_change = move || md();
    // "Split scene" reads the caret and splits this scene there.
    let split: shared::SplitFn = {
        let vm = vm.clone();
        Rc::new(move |ctx: &mut EventContext, caret: usize| vm.split_scene(ctx, id, caret))
    };
    VStack::new()
        .spacing(4.0)
        .child(shared::vspace(10.0))
        .child(shared::centered(scene_header(vm, id), column_width))
        .child(shared::writing_column(
            &scene.main_doc(),
            column_width,
            on_change,
            Some(split),
        ))
}

fn scene_header(vm: &ChapterViewModel, id: u64) -> impl Widget {
    let scene = vm.scene(id);
    HStack::new()
        .spacing(8.0)
        .child(
            TextWidget::new(lit!(""))
                .text(scene.title())
                .style(TextStyleRole::SmallBold)
                .color(TextRole::Secondary),
        )
        // Scene label (blank when unset). Tags: placeholder for a future
        // BinderTag chip strip.
        .child(
            TextWidget::new(lit!(""))
                .text(scene.label())
                .color(TextRole::Secondary),
        )
        .child(Expand::horizontal().child(Divider::new()))
        .child(scene_menu(vm, id))
}

fn scene_menu(vm: &ChapterViewModel, id: u64) -> impl Widget {
    // Move/merge stay enabled and no-op at the boundaries (the row is reused, not
    // rebuilt, on reorder, so a build-time bool would go stale).
    let mk = |f: fn(&ChapterViewModel, &mut EventContext, u64)| {
        let vm = vm.clone();
        move |ctx: &mut EventContext| f(&vm, ctx, id)
    };
    PopoverIconButton::new(IconButton::more()).bare().content(
        MenuList::new()
            .item(
                MenuItem::new(tr!(rename()))
                    .icon(IconWidget::from_svg(EDIT_SVG))
                    .on_activate_fn(mk(|v, c, id| v.begin_rename_scene(c, id))),
            )
            .item(
                MenuItem::new(tr!(set_label()))
                    .on_activate_fn(mk(|v, c, id| v.begin_set_label(c, id))),
            )
            .separator()
            .item(
                MenuItem::new(tr!(insert_scene()))
                    .on_activate_fn(mk(|v, c, id| v.begin_insert_scene_after(c, id))),
            )
            .item(MenuItem::new(tr!(move_up())).on_activate_fn(mk(|v, c, id| v.move_scene_up(c, id))))
            .item(
                MenuItem::new(tr!(move_down()))
                    .on_activate_fn(mk(|v, c, id| v.move_scene_down(c, id))),
            )
            .item(
                MenuItem::new(tr!(merge_with_previous()))
                    .on_activate_fn(mk(|v, c, id| v.merge_into_previous(c, id))),
            )
            .separator()
            .item(
                MenuItem::new(tr!(move_to_trash()))
                    .text_role(TextRole::Error)
                    .on_activate_fn(mk(|v, c, id| v.trash_scene(c, id))),
            ),
    )
}
