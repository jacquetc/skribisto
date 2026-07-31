// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The **manuscript stream** pane — one body, two flavours, three containers.
//!
//! A Scrivener-"Scrivenings"-style continuous, editable view of everything inside a
//! container: the chapter's scenes (Full Chapter), a part's chapters and their scenes
//! (Full Part), or a whole book's parts, chapters and scenes (Full Book). Each row
//! carries its own header + options menu; an "Add" action closes the pane.
//!
//! The **flavour** ([`SplitFlavour`]) picks which of a row's two writing surfaces the
//! stream shows: its prose (`Prose`) or its synopsis (`Synopsis`). Everything else —
//! the rows, the headings, the menus, the split/merge gating — is identical, which is
//! why the Full Synopsis stream is this same function with one argument flipped.
//!
//! **Which rows get which editor is decided by the constraint matrix**, never by a
//! hardcoded sub_role list: `is_prose_bearing` / `is_synopsis_bearing` ask
//! `skribisto_model::content_allowed`. So a chapter folder shows a prose editor (it
//! carries its own `SceneText`, exactly like the flat chapter it promotes to), while a
//! part heading shows one only in the Synopsis flavour. Nothing here special-cases a
//! row kind for content — only for *chrome*.

use std::rc::Rc;

use bastyde::prelude::*;
use bastyde::widgets::{
    Button, Divider, Expand, FocusScope, HStack, IconButton, IconWidget, MenuItem, MenuList,
    PopoverIconButton, Repeater, ScrollArea, Spacer, TextWidget, TraversalScopePolicy, VStack,
};

use skribisto_model::{CreateType, SubRoleExt};

use crate::models::{StreamLevel, StreamRow};
use crate::view_models::{
    EditorTypography, SplitFlavour, StreamViewModel, is_prose_bearing, is_synopsis_bearing,
};

use super::{
    HEADING_PROSE_MIN_LINES, MAIN_MIN_LINES, SplitFn, centered, synopsis_column, vspace,
    writing_column,
};

/// Pencil icon for the inline rename affordances (no built-in "edit" icon).
const EDIT_SVG: &str = include_str!("../../../resources/icons/edit.svg");

/// The scrollable manuscript: container header, the container's own writing surface,
/// one editor per row, and the add action.
///
/// Structurally identical to the single-scene tab (an outer `ScrollArea` over a flowing
/// `VStack`) so the flowing writing columns size correctly; a zero-size [`WireOnBuild`]
/// child subscribes the view-model when this pane first mounts.
pub fn stream_pane(tab: &super::super::ContentTab, flavour: SplitFlavour) -> impl Widget {
    let cw = tab.column_width.clone();
    let mut col = VStack::new().spacing(0.0);

    if let Some(vm) = tab.stream().cloned() {
        let mark_dirty: Rc<dyn Fn()> = Rc::new(tab.mark_dirty_fn());
        // A row's editor is a scene's, whichever flavour: prose uses the Scene bundle,
        // synopses use the Synopsis bundle.
        let typo = match flavour {
            SplitFlavour::Prose => tab.typography.scene.clone(),
            SplitFlavour::Synopsis => tab.typography.synopsis.clone(),
        };
        let format = tab.format.clone();
        let factory = {
            let vm = vm.clone();
            let cw = cw.clone();
            let md = mark_dirty.clone();
            let typo = typo.clone();
            let format = format.clone();
            move |row: &StreamRow| -> Box<dyn Widget> {
                Box::new(stream_row(&vm, row, &cw, &typo, flavour, &md, &format))
            }
        };

        col = col
            .child(WireOnBuild::new(vm.clone()))
            .child(vspace(12.0))
            .child(centered(container_header(&vm), &cw))
            .child(vspace(4.0));

        // The container's **own** content — it is the pane header, never a row. In the
        // Prose flavour that is a chapter folder's own `SceneText` (present after
        // promoting a flat chapter, and where that prose stays editable); a Part or a
        // Book has none. In the Synopsis flavour every container has one.
        let own = match flavour {
            SplitFlavour::Prose => tab.main(),
            SplitFlavour::Synopsis => tab.synopsis(),
        };
        if let Some(field) = own {
            col = match flavour {
                SplitFlavour::Prose => col.child(writing_column(
                    &field.doc,
                    &cw,
                    &typo,
                    MAIN_MIN_LINES,
                    tab.mark_dirty_fn(),
                    Option::None,
                    Option::None,
                    tab.open_doc.spell_main(),
                    tab.open_doc.replacement_main(),
                    Some(tab.format.clone()),
                )),
                SplitFlavour::Synopsis => col.child(synopsis_column(
                    &field.doc,
                    &cw,
                    &typo,
                    tab.mark_dirty_fn(),
                    Option::None,
                    tab.open_doc.spell_synopsis(),
                    tab.open_doc.replacement_synopsis(),
                    // No per-tab sink: a stream shows one synopsis per row, so
                    // the last row built would win it. These rows reach the
                    // formatting surfaces through the editor registry instead
                    // (see `FormatViewModel`), which resolves by focus and so
                    // can name the row the caret is actually in.
                    Option::None,
                    Some(tab.format.clone()),
                )),
            };
            col = col.child(vspace(6.0));
        }

        col = col
            .child(Repeater::new(vm.list(), factory))
            .child(vspace(10.0))
            .child(centered(add_button(&vm), &cw))
            .child(vspace(28.0));
    }
    ScrollArea::new().child(col)
}

/// Zero-size child that wires the stream view-model on build (subscribing the row list
/// and the per-row metadata) — the one place inside the pane's widget tree that gets a
/// `BuildContext`. `wire` is idempotent.
struct WireOnBuild {
    vm: StreamViewModel,
}

impl WireOnBuild {
    fn new(vm: StreamViewModel) -> Self {
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

/// The container's title + a flat rename (pencil) button.
fn container_header(vm: &StreamViewModel) -> impl Widget {
    let rename_vm = vm.clone();
    HStack::new()
        .child(
            TextWidget::new(lit!(""))
                .text(vm.container_title())
                .style(TextStyleRole::BodyBold)
                .color(TextRole::Primary),
        )
        .child(Spacer::new())
        .child(
            IconButton::new(IconWidget::from_svg(EDIT_SVG).icon_size(14.0))
                .embedded()
                .tooltip(tr!(rename()))
                .on_activate_fn(move |ctx| rename_vm.begin_rename_container(ctx)),
        )
}

/// The trailing "Add …" action. What it *creates* is the model's own default
/// recommendation for this container — a Scene inside a chapter, a Chapter inside a
/// part or a book — so the label names that type rather than hardcoding "scene".
fn add_button(vm: &StreamViewModel) -> impl Widget {
    let label = match vm.level() {
        StreamLevel::Chapter => tr!(add_scene()),
        StreamLevel::Part | StreamLevel::Book => tr!(add_chapter()),
    };
    let vm = vm.clone();
    HStack::new()
        .child(Spacer::new())
        .child(Button::new(label).on_activate_fn(move |ctx| vm.begin_add_row(ctx)))
}

/// One row: a header, and — when the matrix allows it for this flavour — the row's
/// editor. A chapter or part heading gets heavier chrome than a scene, so a Full Book
/// reads as a manuscript rather than a flat pile of prose.
fn stream_row(
    vm: &StreamViewModel,
    row: &StreamRow,
    column_width: &Signal<f32>,
    typo: &EditorTypography,
    flavour: SplitFlavour,
    mark_dirty: &Rc<dyn Fn()>,
    format: &crate::view_models::FormatViewModel,
) -> impl Widget {
    let id = row.item_id;
    let is_heading = row.sub_role.opens_chapter() || row.sub_role.opens_part();

    let mut col = VStack::new()
        .spacing(4.0)
        // Structure headings breathe more than the scenes under them.
        .child(vspace(if is_heading { 22.0 } else { 10.0 }))
        .child(centered(row_header(vm, row), column_width));

    // Does this row have a surface in *this* flavour? The constraint matrix answers —
    // a part heading has no prose, but it does have a synopsis.
    let shows_editor = match flavour {
        SplitFlavour::Prose => is_prose_bearing(&row.role, &row.sub_role),
        SplitFlavour::Synopsis => is_synopsis_bearing(&row.role, &row.sub_role),
    };
    if shows_editor && let Some(doc) = vm.row_doc(id) {
        let md = mark_dirty.clone();
        let on_change = move || md();

        // "Split scene" reads the caret and splits *this* editor's text there; the other
        // role stays whole on the source. Only offered where the backend accepts it.
        let split: Option<SplitFn> = vm.can_split(id).then(|| {
            let vm = vm.clone();
            Rc::new(move |ctx: &mut EventContext, caret: usize| {
                vm.split_row(ctx, id, flavour, caret)
            }) as SplitFn
        });

        match flavour {
            SplitFlavour::Prose => {
                if let Some(field) = doc.main.as_ref() {
                    // A chapter heading's own prose is subordinate to its scenes — one
                    // line, growing with content, instead of an empty ten-line box under
                    // every chapter of a Full Book.
                    let min_lines = if is_heading {
                        HEADING_PROSE_MIN_LINES
                    } else {
                        MAIN_MIN_LINES
                    };
                    col = col.child(writing_column(
                        &field.doc,
                        column_width,
                        typo,
                        min_lines,
                        on_change,
                        split,
                        Option::None,
                        doc.spell_main(),
                        doc.replacement_main(),
                        Some(format.clone()),
                    ));
                }
            }
            SplitFlavour::Synopsis => {
                if let Some(field) = doc.synopsis.as_ref() {
                    col = col.child(synopsis_column(
                        &field.doc,
                        column_width,
                        typo,
                        on_change,
                        split,
                        doc.spell_synopsis(),
                        doc.replacement_synopsis(),
                        // One synopsis per stream row — see the sibling call.
                        // Formatting reaches it through the editor registry.
                        Option::None,
                        Some(format.clone()),
                    ));
                }
            }
        }
    }
    col
}

/// A row's header: its title, its label, a rule, and the options menu. A structure
/// heading is `BodyBold`/Primary above a full-width rule; a scene stays `SmallBold`/
/// Secondary, so the hierarchy reads at a glance. (`TextStyleRole` has no dedicated
/// heading variant — the prominence comes from weight, colour and spacing.)
fn row_header(vm: &StreamViewModel, row: &StreamRow) -> impl Widget {
    let id = row.item_id;
    let is_heading = row.sub_role.opens_chapter() || row.sub_role.opens_part();
    let (style, color) = if is_heading {
        (TextStyleRole::BodyBold, TextRole::Primary)
    } else {
        (TextStyleRole::SmallBold, TextRole::Secondary)
    };

    HStack::new()
        .spacing(8.0)
        .child(crate::binder::icons::sub_role_icon(&row.sub_role).icon_size(14.0))
        .child(
            TextWidget::new(lit!(""))
                .text(vm.row_title(id))
                .style(style)
                .color(color),
        )
        // The row's free-text label (blank when unset).
        .child(
            TextWidget::new(lit!(""))
                .text(vm.row_label(id))
                .color(TextRole::Secondary),
        )
        // Tag dots, between the label and the rule. Takes no space when the row is untagged.
        .child(crate::tags::TagDotsRow::new(
            vm.row_tags(id),
            {
                let vm = vm.clone();
                std::rc::Rc::new(move |ids: Vec<u64>, _c: &mut EventContext| {
                    vm.set_row_tags(id, &ids);
                })
            },
            crate::tags::tag_chip::MAX_VISIBLE_STREAM,
        ))
        .child(Expand::horizontal().child(Divider::new()))
        .child(row_menu(vm, row))
}

/// The per-row options menu. Merge and split are offered only where they are legal:
/// `can_merge_into_previous` refuses to merge a row away across a chapter or part
/// boundary, and `can_split` only offers a split on a prose-bearing row. Move stays
/// enabled and no-ops at the boundaries (the row is reused, not rebuilt, on reorder, so
/// a build-time bool would go stale).
fn row_menu(vm: &StreamViewModel, row: &StreamRow) -> impl Widget {
    let id = row.item_id;
    let mk = |f: fn(&StreamViewModel, &mut EventContext, u64)| {
        let vm = vm.clone();
        move |ctx: &mut EventContext| f(&vm, ctx, id)
    };

    let mut list = MenuList::new()
        .item(
            MenuItem::new(tr!(rename()))
                .icon(IconWidget::from_svg(EDIT_SVG))
                .on_activate_fn(mk(|v, c, id| v.begin_rename_row(c, id))),
        )
        .item(
            MenuItem::new(tr!(set_label())).on_activate_fn(mk(|v, c, id| v.begin_set_label(c, id))),
        )
        .separator()
        .item(
            MenuItem::new(insert_label(row))
                .on_activate_fn(mk(|v, c, id| v.begin_insert_after(c, id))),
        )
        .item(MenuItem::new(tr!(move_up())).on_activate_fn(mk(|v, c, id| v.move_row_up(c, id))))
        .item(
            MenuItem::new(tr!(move_down())).on_activate_fn(mk(|v, c, id| v.move_row_down(c, id))),
        );

    if vm.can_merge_into_previous(id) {
        list = list.item(
            MenuItem::new(tr!(merge_with_previous()))
                .on_activate_fn(mk(|v, c, id| v.merge_into_previous(c, id))),
        );
    }

    list = list.separator().item(
        MenuItem::new(tr!(move_to_trash()))
            .text_role(TextRole::Error)
            .on_activate_fn(mk(|v, c, id| v.trash_row(c, id))),
    );

    PopoverIconButton::new(IconButton::more())
        .bare()
        // Trap Tab inside the anchored overlay, as every popover must.
        .content(FocusScope::new(TraversalScopePolicy::Cycle).child(list))
}

/// What "Insert …" on this row will actually create — the model's default
/// recommendation for it. A scene (or a chapter head) recommends a Scene; a *part*
/// heading recommends a Chapter, so the label must say so rather than lie about a scene.
fn insert_label(row: &StreamRow) -> LocalizedString {
    let recommended = skribisto_model::recommendations(&row.role, &row.sub_role)
        .first()
        .map(|r| r.create_type);
    match recommended {
        Some(CreateType::Chapter) => tr!(insert_chapter()),
        _ => tr!(insert_scene()),
    }
}
