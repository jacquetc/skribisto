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
use bastyde::tokens::HAlignment;
use bastyde::widgets::{
    DockOpenLocation, DockSide, DockWidget, DockWidgetId, FocusScope, GroupHeader, IconButton,
    IconWidget, MenuItem, MenuList, Padding, PopoverIconButton, ScrollArea, TextWidget,
    TraversalScopePolicy, VStack, Wrap,
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
///
/// The one control here that `focusable(false)` cannot protect. Opening the
/// popover moves keyboard focus into its list — it must, or the levels would be
/// unreachable from the keyboard — which blurs the editor and, before
/// [`FormatViewModel::set_dock_overlay_open`] existed, dropped the dock to its
/// empty state: every group hid, this button among them, and the list died
/// dormant with the subtree that held it. Telling the view-model who took the
/// focus is what keeps the surface alive for as long as the list is up. Putting
/// it back is not this module's job — the overlay manager restores the
/// pre-overlay focus on every dismiss path, pick or Escape alike.
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
    // A dock rebuilt while its popover was up would otherwise carry the latch
    // in forever; the fresh trigger starts closed, so say so.
    vm.set_dock_overlay_open(false);
    let opened = vm.clone();
    let closed = vm.clone();
    PopoverIconButton::new(
        IconButton::new(glyph::heading())
            .toolbar()
            .focusable(false)
            .tooltip(tr!(format_heading())),
    )
    .bare()
    .on_open(move || opened.set_dock_overlay_open(true))
    .on_close(move || closed.set_dock_overlay_open(false))
    // Trap Tab inside the anchored overlay, as every popover must.
    .content(FocusScope::new(TraversalScopePolicy::Cycle).child(list))
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
                glyph::direction_rtl(),
                tr!(format_direction_rtl()),
                vm.dir_rtl(),
                vm.clone(),
                |vm| vm.toggle_direction(),
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
    //
    // Centred by a `VStack`'s cross-axis alignment, **not** by `Center`. The
    // hint is a whole sentence in a ~236px column, so it has to wrap — and
    // `TextWidget` only wraps when something proposes it a bounded width.
    // `Center` proposes `unspecified` on both axes (that is what lets it
    // shrink-wrap an open one), so the label measured as a single 592px line
    // and was then placed at half its overhang — x = -170 — running off the
    // dock on the left and the right at once. A `VStack` hands its children the
    // width it was given, which is exactly the wrap basis that was missing, and
    // still centres the wrapped block.
    let empty = VisibleWhen::new(
        g.empty.clone(),
        VStack::new()
            .alignment(HAlignment::Center)
            .child(TextWidget::new(tr!(format_panel_empty())).color(TextRole::Secondary)),
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
        // No refresh tick here: it lives in `App`, because the Format menu binds
        // the same mirrors and this dock is only one of two trailing rail tabs.
        // Driven from here, every checkmark in the menu would freeze the moment
        // the user switched the rail to the Inspector.
        //
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
        FormatViewModel::new(Rc::new(|| (None, FormatSurface::None)))
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
    /// assertion would pass against any layout at all.
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

    /// Clicking the heading picker must leave the dock standing.
    ///
    /// Driven through a real pointer tap rather than by poking the view-model,
    /// because the bug lived in the wiring between the two: the popover takes
    /// keyboard focus, the resolver reports the blur honestly, and the dock used
    /// to answer by hiding every group — the picker's own included, which
    /// dormants the subtree the just-opened list hangs off. Pressing the button
    /// made the dock look like there was no editor at all.
    #[test]
    fn opening_the_heading_picker_leaves_the_dock_standing() {
        use bastyde::text_document::TextDocument;
        use bastyde::widgets::rich_text::RichTextEditor;

        let doc = TextDocument::new();
        doc.set_markdown("scene prose")
            .expect("parse")
            .wait()
            .expect("import");
        let editor = RichTextEditor::editor(doc);
        let handle = editor.handle();
        let focused = handle.focused_signal();
        // `App`'s resolver: the target stays, the surface follows live focus.
        let (resolved, live) = (handle.clone(), focused.clone());
        let vm = FormatViewModel::new(Rc::new(move || {
            let surface = if live.get() {
                FormatSurface::Scene
            } else {
                FormatSurface::None
            };
            (Some(resolved.clone()), surface)
        }));

        focused.set(true);
        vm.refresh();
        assert!(vm.groups().block.get(), "the picker's group starts visible");

        let mut tree = WidgetTree::new();
        let id = tree.add(heading_picker(&vm));
        tree.layout(SizeProposal::exact(300.0, 40.0));
        tree.click(id);
        // What the click costs: the popover's list now holds the focus.
        focused.set(false);
        // The per-frame refresh `App` drives off the frame tick.
        vm.refresh();

        assert!(
            vm.groups().block.get(),
            "the group holding the open picker must survive its own popover"
        );
        assert!(
            !vm.groups().empty.get(),
            "the dock must not fall to its 'nothing to format' placeholder"
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

    /// Widths the dock body has to survive.
    ///
    /// The trailing side is **user-resizable** — a `DockingLayout` splitter, not
    /// a fixed 300px. `App` only sets the opening size; from there the writer
    /// drags it anywhere between the docking model's `min_size` floor for a
    /// horizontal-axis side and whatever the window allows. Both numbers are
    /// *content* thicknesses — `SideLayout` adds the 48px activity rail on top
    /// rather than carving it out — so these are the widths the body itself
    /// gets. The narrow end is where a layout that measures itself unbounded
    /// gives itself away.
    const WIDTHS: [f32; 4] = [
        120.0, // the docking model's floor for a leading/trailing side
        300.0, // what `App` opens the trailing side at
        420.0, // dragged out wide
        600.0,
    ];

    /// A tree that measures text the way the app does.
    ///
    /// With no backend `TextWidget` falls back to 8px/char on a **single line**
    /// and merely clamps that to the proposal — it never breaks a line, so a
    /// bare `WidgetTree` cannot tell wrapped text from truncated text.
    /// `MockTextBackend` runs the real paragraph path (8px/char, 16px lines,
    /// word-broken), which is what makes the line count below meaningful.
    fn measuring_tree() -> WidgetTree {
        WidgetTree::new().with_text_backend(std::rc::Rc::new(std::cell::RefCell::new(
            bastyde::canvas::MockTextBackend::new(),
        )))
    }

    /// The worst horizontal overflow anywhere under `id`, measured against the
    /// dock's own `box`, plus the widget that owns it — "something overflows" is
    /// useless without "what". Positive means it escapes. Dormant zero-size
    /// nodes are skipped; they sit at the origin and would read as an overflow
    /// on the leading edge.
    fn worst_overflow(tree: &WidgetTree, id: WidgetId, r#box: Rect) -> (f32, String) {
        let b = tree.bounds(id);
        let mut worst = if b.width > 0.0 {
            (
                (r#box.x - b.x).max(b.right() - r#box.right()),
                tree.widget_type_name(id).unwrap_or("?").to_string(),
            )
        } else {
            (f32::NEG_INFINITY, String::new())
        };
        for child in tree.children(id) {
            let got = worst_overflow(tree, child, r#box);
            if got.0 > worst.0 {
                worst = got;
            }
        }
        worst
    }

    /// Every laid-out `TextWidget` under `id`, skipping the dormant ones (a
    /// hidden group's header is still in the arena, sized to nothing).
    fn visible_labels(tree: &WidgetTree, id: WidgetId) -> Vec<WidgetId> {
        let mut found = Vec::new();
        let mut stack = vec![id];
        while let Some(node) = stack.pop() {
            if tree
                .widget_type_name(node)
                .is_some_and(|n| n.ends_with("::TextWidget"))
                && tree.bounds(node).width > 0.0
            {
                found.push(node);
            }
            stack.extend(tree.children(node));
        }
        found
    }

    /// **The empty state's hint must wrap to the dock, not run off both edges.**
    ///
    /// It is a whole sentence, and no width the splitter can reach fits it on
    /// one line. `TextWidget` wraps by default — but only when something
    /// proposes it a bounded width, and `Center` proposes `unspecified` on both
    /// axes (that is what lets it shrink-wrap an open one). Under it the label
    /// measured as a single 592px line, and `Center` placed that at half its
    /// overhang — x = -170 in the 252px default — so the sentence bled past the
    /// dock on the left and the right at once, at every width.
    #[test]
    fn the_placeholder_wraps_to_the_docks_width() {
        for width in WIDTHS {
            let vm = vm();
            vm.set_surface(FormatSurface::None);
            let mut tree = measuring_tree();
            let id = tree.add(controls(&vm));
            tree.layout(SizeProposal {
                width: Some(width),
                height: None,
            });

            let labels = visible_labels(&tree, id);
            assert_eq!(
                labels.len(),
                1,
                "with nothing focused the placeholder is the only live label"
            );
            let text = tree.bounds(labels[0]);
            let dock = tree.bounds(id);
            assert!(
                text.x >= dock.x + DOCK_PADDING - 0.5
                    && text.right() <= dock.right() - DOCK_PADDING + 0.5,
                "at {width}px the hint must stay inside the {DOCK_PADDING}px \
                 inset: {text:?} in {dock:?}"
            );
            // Wrapped, not clipped. Compared against the sentence's own natural
            // extent rather than a pixel constant, which would make this a
            // translation test — fr-FR's string is longer, and both are free to
            // change. Where there is less room than the sentence needs, the
            // label has to be taller than the single line it would be otherwise.
            let (natural, line) = natural_extent();
            if width - 2.0 * DOCK_PADDING < natural - 1.0 {
                assert!(
                    text.height > line + 1.0,
                    "at {width}px there is less room than the hint's {natural}px \
                     natural width, so it must wrap onto more than one {line}px \
                     line; got {}px tall",
                    text.height
                );
            }
        }
    }

    /// The empty-state sentence laid out with nothing constraining it: its
    /// single-line width, and the height of that one line.
    fn natural_extent() -> (f32, f32) {
        let mut tree = measuring_tree();
        let id = tree.add(TextWidget::new(tr!(format_panel_empty())));
        tree.layout(SizeProposal {
            width: None,
            height: None,
        });
        let b = tree.bounds(id);
        (b.width, b.height)
    }

    /// **Nothing in the dock overflows the side, at any width it can be dragged
    /// to.** The controls already flow — `Wrap` breaks the button rows and
    /// `GroupHeader` ellipsizes — so this guards the whole surface, including
    /// the empty state, against the next widget added without a wrap basis.
    #[test]
    fn no_surface_overflows_the_side_at_any_width() {
        for surface in [
            FormatSurface::Scene,
            FormatSurface::Synopsis,
            FormatSurface::Note,
            FormatSurface::None,
        ] {
            for width in WIDTHS {
                let vm = vm();
                vm.set_surface(surface);
                let mut tree = measuring_tree();
                let id = tree.add(controls(&vm));
                tree.layout(SizeProposal {
                    width: Some(width),
                    height: None,
                });

                // Against the dock's own box, not the padded inset: the outer
                // `Padding` widget spans the full width by design, and it is
                // its *child* that has to respect the inset.
                let (over, who) = worst_overflow(&tree, id, tree.bounds(id));
                assert!(
                    over <= 0.5,
                    "{surface:?} at {width}px overflows the dock by {over}px, \
                     worst offender {who}"
                );
            }
        }
    }
}
