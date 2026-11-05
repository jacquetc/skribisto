// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Where the keyboard goes when what held it is parked by something the writer just did.
//!
//! Hiding a side parks its content once the collapse has run, and Teksilo documents what
//! follows: a focused widget that is parked leaves focus on nothing until something takes
//! it. Nothing did, so a screen reader in the binder pressed F9, heard the window's title
//! and was left on the bare window, with every key going nowhere. Go to did the same from
//! a scene's text to a book showing *Pace*: raising the tab parked the editor that held
//! the keyboard, and the new tab had no text to take it.
//!
//! [`SideFocusKeeper`] is the something. **It watches for the loss and not for the
//! command**, because a side hides by more doors than one (F9, the View menu, the status
//! bar's buttons, the command palette), and because a window has no way to ask which
//! side holds the focus: `DockingLayout` keeps its per-side `focus_within` to itself. Focus
//! falling to nothing just after one of those gestures is what they all share, and it is
//! true only when the keyboard really was in what went away.
//!
//! The keyboard goes to the text of the focused pane's active tab, as Go ▸ Next scene
//! leaves it. A tab with no text there (a corkboard, *Pace*) takes it in its body, and
//! with no tab open the main area does. Landing there is what a reader hears: the control
//! says itself.
//!
//! Leaving distraction-free mode comes through here too, without the loss: the surface
//! that held the keyboard is gone and the desk it covered is back, so the text is where
//! the writer carries on.
//!
//! It takes the focus from a post-mount action on a zero-size child rebuilt by the
//! request, because an effect has no `EventContext`, and only an `EventContext` can move
//! focus. It is the shape `DockingLayout` uses for `focus_dock`.

use std::cell::Cell;
use std::rc::Rc;
use std::time::Instant;

use teksilo::core::BindingLevel;
use teksilo::prelude::*;
use teksilo::widgets::{DockSide, DockingModel};

use crate::editors::EditorsViewModel;

/// The sides a writer can hide. `Top` is not in use.
const SIDES: [DockSide; 3] = [DockSide::Leading, DockSide::Trailing, DockSide::Bottom];

/// How far the search for a place to land has got.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Step {
    Idle,
    /// The keyboard fell to nothing: try the active tab's text, unless something has
    /// taken it since.
    Lost,
    /// Distraction-free mode just ended: the text, whatever holds the keyboard now. The
    /// framework's fallback may have put it on the window's first control, the menu.
    Back,
    /// The text did not take it: the tab's body, or the main area.
    Page,
}

/// See the module doc. Mount it once in the shell, beside the layout whose sides it
/// watches.
pub struct SideFocusKeeper {
    docking: DockingModel,
    editors: EditorsViewModel,
    /// Whether this window's distraction-free surface is up.
    surface_up: Signal<bool>,
    root: Option<WidgetId>,
}

impl SideFocusKeeper {
    pub fn new(docking: DockingModel, editors: EditorsViewModel, surface_up: Signal<bool>) -> Self {
        Self {
            docking,
            editors,
            surface_up,
            root: None,
        }
    }
}

impl std::fmt::Debug for SideFocusKeeper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("SideFocusKeeper")
    }
}

impl Widget for SideFocusKeeper {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        for side in SIDES {
            let editors = self.editors.clone();
            ctx.effect(&self.docking.side_visible_signal(side), move |&visible| {
                if !visible {
                    editors.expect_focus_loss(Instant::now());
                }
            });
        }

        let step = Rc::new(Cell::new(Step::Idle));
        let request = Signal::new(0u64);
        let ask = {
            let (step, request) = (step.clone(), request.clone());
            move |at: Step| {
                step.set(at);
                request.set(request.get().wrapping_add(1));
            }
        };
        {
            let focused = ctx.focused_signal();
            let had_focus = Cell::new(focused.get().is_some());
            let editors = self.editors.clone();
            let ask = ask.clone();
            ctx.effect(&focused, move |now| {
                let lost = had_focus.replace(now.is_some()) && now.is_none();
                if lost && editors.focus_loss_expected(Instant::now()) {
                    ask(Step::Lost);
                }
            });
        }
        ctx.effect(&self.surface_up, move |&up| {
            if !up {
                ask(Step::Back);
            }
        });

        let courier = ctx.add(Courier {
            editors: self.editors.clone(),
            step,
            request,
        });
        self.root = Some(courier);
        vec![courier]
    }

    fn layout_response(&self, _proposal: SizeProposal, _ctx: &LayoutContext) -> LayoutResponse {
        Size::new(0.0, 0.0).into()
    }

    fn children(&self) -> Vec<WidgetId> {
        self.root.into_iter().collect()
    }
}

/// Rebuilt alone by each request, for the `EventContext` its post-mount action gets.
struct Courier {
    editors: EditorsViewModel,
    step: Rc<Cell<Step>>,
    request: Signal<u64>,
}

impl std::fmt::Debug for Courier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("SideFocusCourier")
    }
}

impl Widget for Courier {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        self.request
            .bind_to(ctx.self_id(), ctx.binding_registry(), BindingLevel::Rebuild);
        let at = self.step.replace(Step::Idle);
        if at == Step::Idle {
            return Vec::new();
        }
        let (editors, step, request) = (
            self.editors.clone(),
            self.step.clone(),
            self.request.clone(),
        );
        ctx.run_after_mount(move |c| {
            // Taken by the time this runs, by a rebuild putting focus back in the subtree
            // it replaced or a dialog opening: then nothing was lost.
            if at != Step::Back && c.focused().is_some() {
                return;
            }
            if at != Step::Page && editors.focus_active_text(c) {
                // An editor can answer and still be parked with a page the tab has left,
                // and its request is then dropped at the next layout pass. Look again
                // after that pass.
                step.set(Step::Page);
                request.set(request.get().wrapping_add(1));
            } else {
                editors.focus_active_page(c);
            }
        });
        Vec::new()
    }

    fn layout_response(&self, _proposal: SizeProposal, _ctx: &LayoutContext) -> LayoutResponse {
        Size::new(0.0, 0.0).into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    use crate::editors::Side;
    use crate::editors::test_support::{editors, push_tab};
    use teksilo::core::NoopWindowOps;
    use teksilo::core::widget_tree::WidgetTree;
    use teksilo::text_document::TextDocument;
    use teksilo::widgets::rich_text::{EditorHandle, RichTextEditor};
    use teksilo::widgets::{
        Button, DockOpenLocation, DockWidget, DockWidgetId, DockingLayout, Expand, VStack,
    };

    use crate::tabs::shared::editor::VisibleWhen;

    fn settle(tree: &mut WidgetTree) {
        let size = SizeProposal::exact(1000.0, 700.0);
        for _ in 0..3 {
            tree.layout(size);
            tree.tick_animations(Duration::from_millis(600));
            tree.layout(size);
            tree.run_mount_actions(&mut NoopWindowOps);
            tree.layout(size);
        }
    }

    /// A desk: a left and a right panel with a control each, and a page in the centre with
    /// two controls and, when asked, an editor that is shown or parked.
    struct Desk {
        tree: WidgetTree,
        docking: DockingModel,
        handle: Option<EditorHandle>,
        surface_up: Signal<bool>,
        vm: EditorsViewModel,
    }

    fn desk(editor_shown: Option<bool>) -> Desk {
        let docking = DockingModel::new();
        let surface_up = Signal::new(false);
        let vm = editors();
        let tab = push_tab(&vm, Side::Primary, 42);
        vm.selected(Side::Primary).set(Some(tab));

        let mut page = VStack::new()
            .child(Button::new(lit!("First on the page")))
            .child(Button::new(lit!("Second on the page")));
        let mut handle = None;
        if let Some(shown) = editor_shown {
            let editor = RichTextEditor::editor(TextDocument::new());
            handle = Some(editor.handle());
            page = page.child(VisibleWhen::new(Signal::new(shown), editor));
        }
        let (left, right) = (DockWidgetId::from_raw(9001), DockWidgetId::from_raw(9002));
        let layout = DockingLayout::new(docking.clone())
            .center(page)
            .dock(DockWidget::new(left, lit!("Left"), |_| {
                Button::new(lit!("In the left panel"))
            }))
            .dock(DockWidget::new(right, lit!("Right"), |_| {
                Button::new(lit!("In the right panel"))
            }));
        let mut tree = WidgetTree::new();
        tree.add(
            VStack::new()
                .child(Expand::new().child(layout))
                .child(SideFocusKeeper::new(
                    docking.clone(),
                    vm.clone(),
                    surface_up.clone(),
                )),
        );
        docking.open_dock(left, DockOpenLocation::side(DockSide::Leading));
        docking.open_dock(right, DockOpenLocation::side(DockSide::Trailing));
        settle(&mut tree);

        let ports = vm.active_ports().expect("the tab is selected");
        // The page's node: its first control is what going "into" it reaches.
        let first = tree.find_by_label("First on the page").expect("the page");
        ports.attach_page(first);
        if let Some(handle) = &handle {
            ports.attach_editor(handle.clone());
        }
        Desk {
            tree,
            docking,
            handle,
            surface_up,
            vm,
        }
    }

    impl Desk {
        fn focus_on(&mut self, label: &str) {
            let id = self.tree.find_by_label(label).expect("the control");
            self.tree.focus(id);
            settle(&mut self.tree);
            assert_eq!(self.tree.focused(), Some(id), "precondition: {label}");
        }

        fn focused_label(&mut self) -> Option<String> {
            let id = self.tree.focused()?;
            self.tree
                .accessibility_node(id)
                .name()
                .map(ToOwned::to_owned)
        }
    }

    /// What NVDA measured: F9 in the binder left the writer on the bare window.
    #[test]
    fn a_side_hiding_with_the_focus_in_it_hands_it_to_the_page() {
        let mut d = desk(None);
        d.focus_on("In the left panel");
        d.docking.set_side_visible(DockSide::Leading, false);
        settle(&mut d.tree);
        assert_eq!(
            d.focused_label().as_deref(),
            Some("First on the page"),
            "the keyboard must land in the main area, not on nothing"
        );
    }

    #[test]
    fn the_right_side_is_watched_like_the_left() {
        let mut d = desk(None);
        d.focus_on("In the right panel");
        d.docking.set_side_visible(DockSide::Trailing, false);
        settle(&mut d.tree);
        assert_eq!(d.focused_label().as_deref(), Some("First on the page"));
    }

    /// The text is where Go ▸ Next scene leaves the caret, so it comes before the page.
    #[test]
    fn the_text_of_the_active_tab_comes_first() {
        let mut d = desk(Some(true));
        d.focus_on("In the left panel");
        d.docking.set_side_visible(DockSide::Leading, false);
        settle(&mut d.tree);
        assert!(
            d.handle.as_ref().is_some_and(|h| h.focused_signal().get()),
            "the editor must hold the focus, it is on {:?}",
            d.focused_label()
        );
    }

    /// A container on its Corkboard still holds the editor of a page it showed earlier. A
    /// request to it is dropped, and the keyboard must not be left on nothing by that.
    #[test]
    fn a_parked_editor_does_not_swallow_the_landing() {
        let mut d = desk(Some(false));
        d.focus_on("In the left panel");
        d.docking.set_side_visible(DockSide::Leading, false);
        settle(&mut d.tree);
        assert_eq!(d.focused_label().as_deref(), Some("First on the page"));
    }

    /// Hiding a side the writer is not in must not take them anywhere.
    #[test]
    fn a_side_hiding_without_the_focus_moves_nothing() {
        let mut d = desk(None);
        d.focus_on("Second on the page");
        d.docking.set_side_visible(DockSide::Leading, false);
        settle(&mut d.tree);
        assert_eq!(d.focused_label().as_deref(), Some("Second on the page"));

        d.focus_on("In the right panel");
        d.docking.set_side_visible(DockSide::Leading, true);
        settle(&mut d.tree);
        d.docking.set_side_visible(DockSide::Leading, false);
        settle(&mut d.tree);
        assert_eq!(
            d.focused_label().as_deref(),
            Some("In the right panel"),
            "the other side's control keeps the keyboard"
        );
    }

    /// A focus lost with nothing the writer did behind it is not this keeper's to catch: a
    /// widget going away on its own must not pull the writer into the text.
    #[test]
    fn a_loss_nobody_expected_is_left_alone() {
        let mut d = desk(None);
        d.focus_on("Second on the page");
        d.tree.clear_focus();
        settle(&mut d.tree);
        assert_eq!(d.tree.focused(), None);
    }

    /// Raising a tab with no text parks the editor that held the keyboard: what Go to did
    /// from a scene to a book showing *Pace*.
    #[test]
    fn an_activation_that_parks_the_focus_lands_on_the_page() {
        let shown = Signal::new(true);
        let mut d = desk(None);
        let vm = d.vm.clone();
        let root = d.tree.add(VisibleWhen::new(
            shown.clone(),
            Button::new(lit!("On the page the tab covers")),
        ));
        d.tree.layout(SizeProposal::exact(1000.0, 700.0));
        let _ = root;
        d.focus_on("On the page the tab covers");

        vm.expect_focus_loss(Instant::now());
        shown.set(false);
        settle(&mut d.tree);
        assert_eq!(d.focused_label().as_deref(), Some("First on the page"));
    }

    /// Leaving distraction-free mode: the surface that held the keyboard is gone, whatever
    /// it did with it, and the writer carries on in the text.
    #[test]
    fn leaving_the_surface_goes_back_to_the_text() {
        let mut d = desk(Some(true));
        d.surface_up.set(true);
        settle(&mut d.tree);
        d.focus_on("Second on the page");
        d.surface_up.set(false);
        settle(&mut d.tree);
        assert!(
            d.handle.as_ref().is_some_and(|h| h.focused_signal().get()),
            "the editor must hold the focus, it is on {:?}",
            d.focused_label()
        );
    }

    /// Showing a side again leaves the keyboard where it is: a writer typing with the
    /// binder hidden is not pulled out of the text by bringing it back.
    #[test]
    fn a_side_coming_back_moves_nothing() {
        let mut d = desk(None);
        d.focus_on("In the left panel");
        d.docking.set_side_visible(DockSide::Leading, false);
        settle(&mut d.tree);
        d.focus_on("Second on the page");
        d.docking.set_side_visible(DockSide::Leading, true);
        settle(&mut d.tree);
        assert_eq!(d.focused_label().as_deref(), Some("Second on the page"));
    }
}
