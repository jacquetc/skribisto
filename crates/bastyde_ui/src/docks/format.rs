// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The trailing **Format** dock: the manuscript's formatting controls, as a
//! flowing grid that reflows with the dock's width.
//!
//! Groups that do not apply to what the caret is in are **hidden**, not greyed.
//! A synopsis has no headings and no tables, and a dozen dead buttons teach a
//! writer nothing. The two highest-frequency groups — history and character
//! marks — stay wherever there is anywhere to type, so moving between a scene
//! and its synopsis never makes the dock flicker; only a real change of surface
//! reflows it.
//!
//! Layout is one `Wrap` per group inside a `ScrollArea`. `Wrap` needs a bounded
//! width proposal to break lines at all, and a `ScrollArea`'s content slot
//! supplies exactly that — bounded width, unconstrained height. That is why the
//! two are paired rather than the dock body holding the `Wrap` directly.
//!
//! Every button is built by [`command_button`], [`toggle_button`] or
//! [`intent_button`], which run the command *and* request a frame. That pairing
//! is not optional: an edit made while the pointer is on the dock leaves the
//! editor unfocused, nothing schedules the repaint, and the formatting silently
//! fails to appear. It shipped that way once in the editor's context menu.

use bastyde::core::widget::WidgetPlacement;
use bastyde::prelude::*;
use bastyde::widgets::{
    Center, DockOpenLocation, DockSide, DockWidget, DockWidgetId, GroupHeader, IconButton,
    IconWidget, MenuItem, MenuList, Padding, PopoverIconButton, ScrollArea, TextWidget, VStack,
    Wrap,
};

use crate::icons::format as glyph;
use crate::tabs::shared::editor::VisibleWhen;
use crate::view_models::{ALIGN_CENTER, ALIGN_LEFT, FormatViewModel};

/// Gap between buttons, and between wrapped rows.
const BUTTON_GAP: f32 = 4.0;
/// Gap between one group and the next.
const GROUP_GAP: f32 = 10.0;
/// Inset from the dock's edges.
const DOCK_PADDING: f32 = 8.0;

/// Package the format controls as a trailing `DockWidget`.
pub fn format_dock(vm: FormatViewModel, dock_id: DockWidgetId) -> DockWidget {
    DockWidget::new(dock_id, tr!(format_dock_title()), move |_id| {
        FormatDock::new(vm.clone())
    })
    .icon(crate::icons::activity::format_icon)
    .default_location(DockOpenLocation::side(DockSide::Trailing))
}

/// A momentary control: run it, then schedule the frame that shows the result.
fn command_button(
    icon: IconWidget,
    tooltip: impl Into<bastyde::i18n::LocalizedString>,
    vm: FormatViewModel,
    run: fn(&FormatViewModel),
) -> IconButton {
    IconButton::new(icon)
        .toolbar()
        // Tab-order only. AccessKit emission and `Action::Click` are unaffected,
        // so a screen reader still reaches every button; what this buys is that
        // clicking one leaves the editor's caret and selection where they were.
        .focusable(false)
        .tooltip(tooltip)
        .on_activate_fn(move |ctx| {
            run(&vm);
            ctx.request_frame();
        })
}

/// A control that reflects document state.
///
/// The signal comes from the view-model and is shared with the Format menu's
/// checkmark, so the two surfaces cannot disagree. The view-model re-reads state
/// from the editor after each command rather than leaving `IconButton::toggle`'s
/// optimistic flip standing: over a mixed selection "toggle bold" is not a
/// negation, and the button would show the wrong thing.
fn toggle_button(
    icon: IconWidget,
    tooltip: impl Into<bastyde::i18n::LocalizedString>,
    state: Signal<bool>,
    vm: FormatViewModel,
    run: fn(&FormatViewModel),
) -> IconButton {
    IconButton::new(icon)
        .toolbar()
        .focusable(false)
        .tooltip(tooltip)
        .toggle(state)
        .on_activate_fn(move |ctx| {
            run(&vm);
            ctx.request_frame();
        })
}

/// A button that fires one of the app's registered intents rather than calling
/// the view-model.
///
/// Scene breaks belong to `EditorsViewModel` — they are gated on the *tab*
/// carrying a scene, knowledge this dock has no business holding — and a peer
/// view-model must not be imported here. The intent bus is the sanctioned route
/// for exactly that link, and it means the dock button, the Format menu item and
/// Ctrl+Shift+Enter all reach one command instead of three copies of it.
///
/// Unlike the other two constructors this one does **not** request a frame.
/// `send_intent` only queues — the framework dispatches after this handler
/// returns — so a frame requested here could be spent before the edit lands.
/// The repaint belongs with the edit: `insert_scene_break` focuses the editor
/// once its paragraph is in, which is what schedules the draw.
fn intent_button(
    icon: IconWidget,
    tooltip: impl Into<bastyde::i18n::LocalizedString>,
    intent: &'static str,
) -> IconButton {
    IconButton::new(icon)
        .toolbar()
        .focusable(false)
        .tooltip(tooltip)
        .on_activate_fn(move |ctx| ctx.send_intent(Intent::new(intent)))
}

/// The heading picker: a glyph that opens the seven levels.
///
/// Not the `ComboBox` bastyde's example toolbar uses — a combo has a minimum
/// width that would force a wrap in a 300px rail of 30dp buttons, and reads as a
/// form field among icons. A popover keeps the row's rhythm.
fn heading_picker(vm: &FormatViewModel) -> PopoverIconButton {
    let mut list = MenuList::new();
    for (level, label) in [
        (0usize, tr!(format_heading_normal())),
        (1, tr!(format_heading_1())),
        (2, tr!(format_heading_2())),
        (3, tr!(format_heading_3())),
        (4, tr!(format_heading_4())),
        (5, tr!(format_heading_5())),
        (6, tr!(format_heading_6())),
    ] {
        let vm2 = vm.clone();
        list = list.item(
            // The levels are mutually exclusive, and `heading` is already
            // mirrored as the index the caret sits at — so the picker shows
            // where you are, not just where you can go.
            MenuItem::new(label)
                .radio(level, vm.heading())
                .on_activate_fn(move |ctx| {
                    vm2.set_heading(level);
                    ctx.request_frame();
                }),
        );
    }
    PopoverIconButton::new(
        IconButton::new(glyph::heading())
            .toolbar()
            .focusable(false)
            .tooltip(tr!(format_heading())),
    )
    .bare()
    .content(list)
}

/// One labelled group: a header over a flowing row, gated as a unit.
///
/// The gap to the next group is *inside* the gate, as bottom padding, rather
/// than `spacing` on the enclosing `VStack`. A `VStack` reserves its spacing
/// between every registered child including the dormant ones, so putting the
/// gap out there left a 10px void for each hidden group — measured at 92px of
/// content for an empty state whose placeholder and padding account for ~36.
/// Carried by the group, the gap disappears exactly when the group does.
fn group(
    visible: Signal<bool>,
    header: impl Into<bastyde::i18n::LocalizedString>,
    controls: Wrap,
) -> VisibleWhen {
    VisibleWhen::new(
        visible,
        Padding::new(0.0, 0.0, GROUP_GAP, 0.0).child(
            VStack::new()
                .spacing(BUTTON_GAP)
                .child(GroupHeader::new(header))
                .child(controls),
        ),
    )
}

/// An empty flowing row.
fn row() -> Wrap {
    Wrap::new().spacing(BUTTON_GAP).line_spacing(BUTTON_GAP)
}

struct FormatDock {
    vm: FormatViewModel,
    root: Option<WidgetId>,
}

impl FormatDock {
    fn new(vm: FormatViewModel) -> Self {
        Self { vm, root: None }
    }
}

impl std::fmt::Debug for FormatDock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FormatDock").finish_non_exhaustive()
    }
}

/// The controls themselves, without the surrounding `ScrollArea`.
///
/// Split out because the `ScrollArea` is *greedy* — it fills the dock side and
/// reports that height whatever it contains — so measuring the dock says
/// nothing about whether hiding a group actually collapsed it. This is the
/// widget whose height the hide model moves.
fn controls(vm: &FormatViewModel) -> Padding {
    let g = vm.groups().clone();

    let history = group(
        g.history.clone(),
        tr!(format_group_history()),
        row()
            .child(command_button(
                glyph::undo(),
                tr!(format_undo()),
                vm.clone(),
                |vm| vm.undo(),
            ))
            .child(command_button(
                glyph::redo(),
                tr!(format_redo()),
                vm.clone(),
                |vm| vm.redo(),
            )),
    );

    let marks = group(
        g.marks.clone(),
        tr!(format_group_marks()),
        row()
            .child(toggle_button(
                glyph::bold(),
                tr!(format_bold()),
                vm.bold(),
                vm.clone(),
                |vm| vm.toggle_bold(),
            ))
            .child(toggle_button(
                glyph::italic(),
                tr!(format_italic()),
                vm.italic(),
                vm.clone(),
                |vm| vm.toggle_italic(),
            ))
            .child(toggle_button(
                glyph::underline(),
                tr!(format_underline()),
                vm.underline(),
                vm.clone(),
                |vm| vm.toggle_underline(),
            ))
            .child(toggle_button(
                glyph::strikethrough(),
                tr!(format_strikethrough()),
                vm.strikethrough(),
                vm.clone(),
                |vm| vm.toggle_strikethrough(),
            ))
            .child(toggle_button(
                glyph::superscript(),
                tr!(format_superscript()),
                vm.superscript(),
                vm.clone(),
                |vm| vm.toggle_superscript(),
            ))
            .child(toggle_button(
                glyph::subscript(),
                tr!(format_subscript()),
                vm.subscript(),
                vm.clone(),
                |vm| vm.toggle_subscript(),
            ))
            .child(command_button(
                glyph::clear_formatting(),
                tr!(format_clear()),
                vm.clone(),
                |vm| vm.clear_formatting(),
            )),
    );

    let block = group(
        g.block.clone(),
        tr!(format_group_block()),
        row()
            .child(heading_picker(vm))
            .child(toggle_button(
                glyph::align_left(),
                tr!(format_align_left()),
                vm.align_left(),
                vm.clone(),
                |vm| vm.set_alignment(ALIGN_LEFT),
            ))
            .child(toggle_button(
                glyph::align_center(),
                tr!(format_align_center()),
                vm.align_center(),
                vm.clone(),
                |vm| vm.set_alignment(ALIGN_CENTER),
            ))
            .child(toggle_button(
                glyph::blockquote(),
                tr!(format_blockquote()),
                vm.blockquote(),
                vm.clone(),
                |vm| vm.toggle_blockquote(),
            )),
    );

    let lists = group(
        g.lists.clone(),
        tr!(format_group_lists()),
        row()
            .child(command_button(
                glyph::list_bullet(),
                tr!(format_list_bullet()),
                vm.clone(),
                |vm| vm.insert_bullet_list(),
            ))
            .child(command_button(
                glyph::list_numbered(),
                tr!(format_list_numbered()),
                vm.clone(),
                |vm| vm.insert_numbered_list(),
            ))
            .child(command_button(
                glyph::indent(),
                tr!(format_indent()),
                vm.clone(),
                |vm| vm.indent(),
            ))
            .child(command_button(
                glyph::outdent(),
                tr!(format_outdent()),
                vm.clone(),
                |vm| vm.outdent(),
            )),
    );

    // Insert is offered wherever a table could live; the seven row/column
    // operations only mean something with the caret inside one, so they hide
    // rather than sit dead under the insert button.
    let table_ops = row()
        .child(command_button(
            glyph::table_row_above(),
            tr!(format_table_row_above()),
            vm.clone(),
            |vm| vm.insert_row_above(),
        ))
        .child(command_button(
            glyph::table_row_below(),
            tr!(format_table_row_below()),
            vm.clone(),
            |vm| vm.insert_row_below(),
        ))
        .child(command_button(
            glyph::table_col_before(),
            tr!(format_table_col_before()),
            vm.clone(),
            |vm| vm.insert_column_before(),
        ))
        .child(command_button(
            glyph::table_col_after(),
            tr!(format_table_col_after()),
            vm.clone(),
            |vm| vm.insert_column_after(),
        ))
        .child(command_button(
            glyph::table_row_delete(),
            tr!(format_table_row_delete()),
            vm.clone(),
            |vm| vm.remove_row(),
        ))
        .child(command_button(
            glyph::table_col_delete(),
            tr!(format_table_col_delete()),
            vm.clone(),
            |vm| vm.remove_column(),
        ))
        .child(command_button(
            glyph::table_remove(),
            tr!(format_table_remove()),
            vm.clone(),
            |vm| vm.remove_table(),
        ));

    let tables = VisibleWhen::new(
        g.tables.clone(),
        VStack::new()
            .spacing(BUTTON_GAP)
            .child(GroupHeader::new(tr!(format_group_tables())))
            .child(row().child(command_button(
                glyph::table_insert(),
                tr!(format_table_insert()),
                vm.clone(),
                |vm| vm.insert_table(3, 3),
            )))
            .child(VisibleWhen::new(vm.in_table(), table_ops)),
    );

    let breaks = group(
        g.scene_breaks.clone(),
        tr!(format_group_breaks()),
        row()
            .child(intent_button(
                glyph::scene_break_minor(),
                tr!(menu_scene_break()),
                "format.scene_break",
            ))
            .child(intent_button(
                glyph::scene_break_major(),
                tr!(menu_major_scene_break()),
                "format.major_scene_break",
            )),
    );

    // The placeholder replaces the controls rather than joining them: with
    // nothing formattable focused, every group above is hidden anyway.
    let empty = VisibleWhen::new(
        g.empty.clone(),
        Center::new().child(TextWidget::new(tr!(format_panel_empty())).color(TextRole::Secondary)),
    );

    Padding::uniform(DOCK_PADDING).child(
        VStack::new()
            .spacing(0.0)
            .child(empty)
            .child(history)
            .child(marks)
            .child(block)
            .child(lists)
            .child(tables)
            .child(breaks),
    )
}

impl Widget for FormatDock {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        // Pull the editor's state into the mirrors once per frame.
        //
        // Deliberately not an effect on the editor's `format_version`: that
        // signal is written from inside the editor's own `state.borrow_mut()`
        // and observers fire synchronously there, so reading the state back
        // would panic on an already-borrowed cell. A frame tick fires outside
        // any borrow, and `refresh` short-circuits when nothing has moved.
        {
            let vm = self.vm.clone();
            let tick = ctx.frame_tick();
            ctx.effect(&tick, move |_| vm.refresh());
        }

        // The ScrollArea is what gives `Wrap` its bounded width; without it the
        // rows would report their widest single line and never break.
        let padded = ctx.add(controls(&self.vm));
        let root = ctx.add(ScrollArea::from_id(padded));
        self.root = Some(root);
        vec![root]
    }

    fn layout_response(&self, proposal: SizeProposal, ctx: &LayoutContext) -> LayoutResponse {
        if let Some(root) = self.root
            && let Some(size) = ctx.child_size(root, proposal)
        {
            return size.into();
        }
        proposal.resolve(0.0, 0.0).into()
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
        self.root.into_iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::view_models::FormatSurface;
    use bastyde::core::widget_tree::WidgetTree;
    use std::rc::Rc;

    /// A view-model with nothing focused — enough to build the dock, since the
    /// dock reads groups and signals, never the editor directly.
    fn vm() -> FormatViewModel {
        FormatViewModel::new(Rc::new(|| None))
    }

    /// The dock builds and lays out at the trailing side's real width.
    #[test]
    fn the_dock_lays_out_at_the_trailing_sides_width() {
        let vm = vm();
        vm.set_surface(FormatSurface::Scene);
        let mut tree = WidgetTree::new();
        let id = tree.add(FormatDock::new(vm));
        // 300px is what App sets the trailing side to; 48 of that is the rail.
        tree.layout(SizeProposal::exact(300.0, 700.0));
        let bounds = tree.bounds(id);
        assert!(
            bounds.width > 0.0 && bounds.height > 0.0,
            "the dock must occupy its side, got {bounds:?}"
        );
    }

    /// Hiding a group must actually collapse it. If a hidden group still
    /// reserved its row the "hide, don't grey" decision would buy nothing — the
    /// gap would just be blank instead of full of dead buttons.
    ///
    /// Measured on [`controls`], not on the dock: the dock's `ScrollArea` is
    /// greedy and reports the side's height whatever it holds, so a dock-level
    /// assertion would pass against any layout at all. (It did, at a constant
    /// 200px, before this test was pointed at the right widget.)
    #[test]
    fn hiding_a_group_collapses_it() {
        fn height_at(surface: FormatSurface) -> f32 {
            let vm = vm();
            vm.set_surface(surface);
            let mut tree = WidgetTree::new();
            let id = tree.add(controls(&vm));
            // Unconstrained height so the content reports what it wants; the
            // width is bounded because `Wrap` needs that to break lines.
            tree.layout(SizeProposal {
                width: Some(300.0),
                height: None,
            });
            tree.bounds(id).height
        }

        let scene = height_at(FormatSurface::Scene);
        let synopsis = height_at(FormatSurface::Synopsis);
        let empty = height_at(FormatSurface::None);

        assert!(
            synopsis < scene,
            "a synopsis hides block, tables and scene breaks, so it must be \
             shorter than a scene: {synopsis} vs {scene}"
        );
        assert!(
            empty < synopsis,
            "with nothing focused every group hides and only the placeholder \
             remains: {empty} vs {synopsis}"
        );
    }

    /// A hidden group must leave no trace, not merely no buttons.
    ///
    /// The gap between groups is bottom padding *inside* each gate rather than
    /// `spacing` on the enclosing `VStack`, because a `VStack` reserves spacing
    /// between every registered child — dormant ones included. With the gap
    /// outside, the empty state measured 92px for a placeholder and padding
    /// worth ~32: six hidden groups were each still holding a 10px void open.
    /// This pins the tight value so that regression is visible.
    #[test]
    fn a_hidden_group_leaves_no_gap_behind() {
        let vm = vm();
        vm.set_surface(FormatSurface::None);
        let mut tree = WidgetTree::new();
        let id = tree.add(controls(&vm));
        tree.layout(SizeProposal {
            width: Some(300.0),
            height: None,
        });
        let h = tree.bounds(id).height;
        assert!(
            h < 2.0 * DOCK_PADDING + 30.0,
            "the empty state is one line of text inside {DOCK_PADDING}px padding; \
             {h}px means hidden groups are still reserving their spacing"
        );
    }

    /// The group gates follow the surface, and the two high-frequency groups
    /// stay put across the switch a writer makes most often — scene to synopsis
    /// and back. That is what keeps the dock from flickering under the cursor.
    #[test]
    fn history_and_marks_survive_the_scene_to_synopsis_switch() {
        let vm = vm();
        let g = vm.groups().clone();

        vm.set_surface(FormatSurface::Scene);
        assert!(g.history.get() && g.marks.get() && g.block.get() && g.scene_breaks.get());

        vm.set_surface(FormatSurface::Synopsis);
        assert!(
            g.history.get() && g.marks.get(),
            "the most-used groups must not blink out when focus moves to the synopsis"
        );
        assert!(
            !g.block.get() && !g.tables.get() && !g.scene_breaks.get(),
            "a synopsis is not chapter-structured, so those groups go"
        );
        assert!(!g.empty.get());

        vm.set_surface(FormatSurface::None);
        assert!(g.empty.get());
        assert!(!g.history.get() && !g.marks.get() && !g.lists.get());
    }
}
