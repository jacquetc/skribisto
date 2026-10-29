// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The Go menu's commands: prev/next Scene, Chapter and Note, scoped to the focused
//! item's own binder — Increment 4 of distraction-free.
//!
//! Eight bare-named global actions, none with an `AppIntent` variant — the same
//! "fixed behaviour, keystroke/menu only" shape `editor.find_next` and
//! `format.scene_break` already use. Six are kind-specific (menu rows only, no
//! individual shortcut — see the Go menu built in `shell/windows.rs`); the other two,
//! `go.next`/`go.prev`, are the generic pair the distraction-free strip's own
//! Next/Previous buttons fire, bound to Alt+Down/Alt+Up while a writing editor
//! holds the focus (see [`register_step_pair`] for why only then), leaving
//! Alt+Left/Alt+Right free for a possible future history feature. They read the
//! *focused tab's own kind* at dispatch time and delegate to the matching
//! kind-specific answer.
//!
//! Every handler resolves its target through [`EditorsViewModel::go`], which is the
//! **existing** `open_or_focus` path — never a bespoke `OpenDocsStore::open` — so
//! autosave, dirty tracking and `StatsModel::active_item` all see a Go jump exactly
//! like any ordinary binder navigation. A target that does not exist (nothing
//! focused, or nothing of that kind in that direction — no wraparound) is a quiet
//! no-op, the same shape `format.scene_break`'s own internal guard already uses.

use std::rc::Rc;

use skribisto_model::{GoDirection, GoKind};
use teksilo::prelude::*;

use crate::editors::EditorsViewModel;

use super::CommandDeps;
use crate::shared::shortcut_group::{ShortcutGroup, ShortcutGroupExt};

pub(super) fn register(ctx: &mut BuildContext, deps: &CommandDeps) {
    register_kind(ctx, deps, "go.next_scene", GoKind::Scene, GoDirection::Next);
    register_kind(
        ctx,
        deps,
        "go.prev_scene",
        GoKind::Scene,
        GoDirection::Previous,
    );
    register_kind(
        ctx,
        deps,
        "go.next_chapter",
        GoKind::Chapter,
        GoDirection::Next,
    );
    register_kind(
        ctx,
        deps,
        "go.prev_chapter",
        GoKind::Chapter,
        GoDirection::Previous,
    );
    register_kind(ctx, deps, "go.next_note", GoKind::Note, GoDirection::Next);
    register_kind(
        ctx,
        deps,
        "go.prev_note",
        GoKind::Note,
        GoDirection::Previous,
    );

    // The generic pair: keyboard/strip-only, never a menu row (the six kind-specific
    // rows above already cover the menu). Alt+Down steps forward, Alt+Up backward —
    // Down/Up rather than Left/Right so a future history feature can still claim
    // Alt+Left/Alt+Right, matching the prior-art shape one axis over.
    {
        let editors = deps.editors.clone();
        register_step_pair(
            ctx,
            deps.format.writing_focused(),
            Rc::new(move |direction, ctx| {
                if let Some(kind) = editors.focused_go_kind() {
                    editors.go(kind, direction, ctx);
                }
            }),
        );
    }

    // `go.to` — the shortcut only. Its **action** is registered by the
    // `GoToButton` widget itself, through `PopoverWidget::open_action`, because
    // opening a popover needs the `EventContext` that only the widget's own
    // toggle closure has (see that widget's module doc). Registering a second
    // action here under the same name would shadow it with something that could
    // not actually present the overlay.
    ctx.register_shortcut_global(
        Shortcut::new("go.to")
            .group(ShortcutGroup::Go)
            .name(tr!(shortcut_name_go_to()))
            .primary(KeyStroke::ctrl(Key::G))
            .build(),
    );
    // `go.to` is the name the menu and the shortcut use; it forwards to
    // whichever `GoToButton` is actually on screen. There are two per window —
    // the status bar's and the distraction-free strip's — and only one is ever
    // visible. Letting both claim the same action meant Ctrl+G could be
    // answered by the *hidden* one, whose trigger has no live bounds, so the
    // popover opened anchored to nothing in the top-left corner.
    {
        let focus = deps.focus.clone();
        ctx.register_action_global(Action::new("go.to").on_invoke(move |_i, c| {
            let target = if focus.active_signal().get() {
                crate::statusbar::go_to_button::GO_TO_FOCUS
            } else {
                crate::statusbar::go_to_button::GO_TO_MAIN
            };
            c.send_intent(Intent::new(target));
        }));
    }
}

/// What the generic pair does when it fires: step in `direction` from the focused
/// tab, with the context it needs to put the caret in the item it opens.
type Step = Rc<dyn Fn(GoDirection, &mut EventContext)>;

/// Register the generic `go.next`/`go.prev` pair: Alt+Down and Alt+Up, and the
/// actions the distraction-free strip's own buttons send.
///
/// The two shortcuts are live only while a writing editor holds the keyboard
/// focus (`writing`). Teksilo gives the same chords to its widgets: a binder row
/// moves with Alt+Up/Alt+Down, its only keyboard alternative to a drag, and a
/// combo box or a date field opens with Alt+Down. A global shortcut resolves
/// before the focused widget sees the key, so the ungated pair took the chord
/// from every one of them. A shortcut that is not live counts as unregistered,
/// and the key reaches the focused widget.
///
/// The actions stay ungated: a strip button sends its intent with the focus on
/// the button, not in the text.
fn register_step_pair(ctx: &mut BuildContext, writing: Signal<bool>, step: Step) {
    for (id, name, key, direction) in [
        (
            "go.next",
            tr!(shortcut_name_go_next()),
            Key::ArrowDown,
            GoDirection::Next,
        ),
        (
            "go.prev",
            tr!(shortcut_name_go_prev()),
            Key::ArrowUp,
            GoDirection::Previous,
        ),
    ] {
        ctx.register_shortcut_global(
            Shortcut::new(id)
                .group(ShortcutGroup::Go)
                .name(name)
                .primary(KeyStroke::new(key, Modifiers::ALT))
                .enabled_when(writing.clone())
                .build(),
        );
        let step = step.clone();
        ctx.register_action_global(Action::new(id).on_invoke(move |_i, c| step(direction, c)));
    }
}

/// Register one kind-specific Go action: fixed `kind`/`direction`, no payload, no
/// shortcut — reached only via its Go-menu row (`shell/windows.rs`) or a scripted
/// intent by name.
fn register_kind(
    ctx: &mut BuildContext,
    deps: &CommandDeps,
    name: &'static str,
    kind: GoKind,
    direction: GoDirection,
) {
    let editors: EditorsViewModel = deps.editors.clone();
    ctx.register_action_global(Action::new(name).on_invoke(move |_i, c| {
        editors.go(kind, direction, c);
    }));
}

#[cfg(test)]
mod tests {
    use std::cell::{Cell, RefCell};

    use teksilo::core::widget::{LayoutResponse, WidgetPlacement};
    use teksilo::core::widget_tree::WidgetTree;
    use teksilo::widgets::Spacer;

    use super::*;

    /// Registers the pair the way `App` does, over one focusable widget that
    /// stands for a binder row: it counts the Alt+Down presses it is handed.
    struct Host {
        writing: Signal<bool>,
        steps: Rc<RefCell<Vec<GoDirection>>>,
        row_saw: Rc<Cell<u32>>,
    }

    impl std::fmt::Debug for Host {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str("Host")
        }
    }

    impl Widget for Host {
        fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
            let steps = self.steps.clone();
            register_step_pair(
                ctx,
                self.writing.clone(),
                Rc::new(move |direction, _ctx| steps.borrow_mut().push(direction)),
            );
            let row_saw = self.row_saw.clone();
            let row = Spacer::new().focusable(true).on_key(move |event, _ctx| {
                if matches!(
                    event,
                    WidgetEvent::KeyDown {
                        key: Key::ArrowDown,
                        modifiers: Modifiers::ALT,
                        ..
                    }
                ) {
                    row_saw.set(row_saw.get() + 1);
                    return EventResponse::Handled;
                }
                EventResponse::Ignored
            });
            vec![ctx.add(row)]
        }

        fn layout_response(&self, proposal: SizeProposal, _ctx: &LayoutContext) -> LayoutResponse {
            Size::new(
                proposal.width.unwrap_or(300.0),
                proposal.height.unwrap_or(40.0),
            )
            .into()
        }

        fn place_children(
            &self,
            bounds: Rect,
            _proposal: SizeProposal,
            children: &mut [WidgetPlacement],
            _ctx: &LayoutContext,
        ) {
            for child in children {
                child.origin = Point::new(bounds.x, bounds.y);
                child.size = Size::new(bounds.width, bounds.height);
            }
        }
    }

    /// Alt+Down is the binder row's own way to move without a drag. With no
    /// writing editor focused it must reach the row; with one focused it steps to
    /// the next item, and the row never sees it.
    #[test]
    fn alt_down_reaches_the_focused_widget_unless_a_writing_editor_holds_the_focus() {
        let writing = Signal::new(false);
        let steps = Rc::new(RefCell::new(Vec::new()));
        let row_saw = Rc::new(Cell::new(0));
        let mut tree = WidgetTree::new();
        let host = tree.add(Host {
            writing: writing.clone(),
            steps: steps.clone(),
            row_saw: row_saw.clone(),
        });
        tree.layout(SizeProposal::exact(300.0, 40.0));
        let row = *tree
            .children(host)
            .first()
            .expect("the row is the host's one child");
        tree.focus(row);

        tree.press_key(Key::ArrowDown, Modifiers::ALT);
        assert_eq!(row_saw.get(), 1, "the focused row must be handed Alt+Down");
        assert!(
            steps.borrow().is_empty(),
            "with no writing editor focused, Go must not take the chord"
        );

        writing.set(true);
        tree.press_key(Key::ArrowDown, Modifiers::ALT);
        tree.press_key(Key::ArrowUp, Modifiers::ALT);
        assert_eq!(
            *steps.borrow(),
            vec![GoDirection::Next, GoDirection::Previous],
            "from a writing editor, Alt+Down and Alt+Up step through the items"
        );
        assert_eq!(
            row_saw.get(),
            1,
            "the chord the shortcut took is not the row's"
        );
    }
}
