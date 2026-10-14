// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! **Edit ▸ Undo / Redo** and the clipboard — the application's one Undo.
//!
//! # Who answers `Ctrl+Z`
//!
//! teksilo registers the seven standard edit commands in every widget tree
//! ([`StandardEdit`]: `edit.undo`, `edit.redo`, `edit.cut`, `edit.copy`,
//! `edit.paste`, `edit.paste_plain`, `edit.select_all`), and every text widget
//! and data view answers them with an `Action` on its own node. The chord fires
//! the intent, and the intent walks from the focused widget to the root, so the
//! widget holding the caret answers first: a scene undoes its own typing, a
//! rename box in the Overview its own, a list selects all its rows. The
//! actions here are what the walk reaches **when no widget took the intent** —
//! the focus is in the binder, a writing game refused the editor's Undo, or
//! the caret's own history is empty (`attach_surfaces` turns on teksilo's
//! `set_empty_history_passes_on` for exactly that fall-through) — and
//! what a menu row reaches, since a row's intent starts in the menu overlay and
//! never passes through the editor. Both go through
//! [`crate::edit::UndoGroupViewModel`], which asks teksilo's `TextSurfaces`
//! registry which widget holds the caret rather than keeping a list of its own.
//!
//! # Why the shortcuts are re-registered, and why they are not gated
//!
//! The registrations below are upserts over teksilo's own, under the same ids,
//! so there is still one binding per command: they only file the seven under
//! **Edit** beside the rest of the menu ([`ShortcutGroup::Edit`]) and name them
//! in this application's words. The chords are teksilo's (`StandardEdit::shortcut`),
//! so they cannot drift from what the widgets expect, Windows' `Ctrl+Y`-first
//! Redo included. A user's rebinding in **Settings ▸ Keymap** survives the upsert.
//!
//! They carry no `enabled_when`, and must not: a disabled shortcut hands the
//! raw key to the focused widget, and a text widget no longer matches `Ctrl+Z`
//! or `Ctrl+A` in its own key handling — it answers the intent instead. A gate
//! here would therefore make the chord dead wherever it was closed, `Ctrl+A` in
//! every list of the window first. The gates live on the **actions**: one that
//! is disabled does not take the intent, which then reaches nobody, and the key
//! goes on to the focused widget as if no shortcut had matched.

use teksilo::core::shortcut::Shortcut;
use teksilo::prelude::*;

use super::CommandDeps;
use crate::shared::shortcut_group::{ShortcutGroup, ShortcutGroupExt};

/// teksilo's standard shortcut for `command`, filed under Edit and named in
/// this application's words.
fn standard(command: StandardEdit, name: LocalizedString) -> Shortcut {
    let mut shortcut = command.shortcut().group(ShortcutGroup::Edit);
    shortcut.name = name.into();
    shortcut
}

pub(super) fn register(ctx: &mut BuildContext, deps: &CommandDeps) {
    register_with(ctx, &deps.undo_group);
}

/// [`register`] over the one thing it needs, so a headless test can mount it.
fn register_with(ctx: &mut BuildContext, group: &crate::edit::UndoGroupViewModel) {
    let group = group.clone();
    let can_undo = group.can_undo();
    let can_redo = group.can_redo();

    ctx.register_shortcut_global(standard(StandardEdit::Undo, tr!(shortcut_name_edit_undo())));
    ctx.register_shortcut_global(standard(StandardEdit::Redo, tr!(shortcut_name_edit_redo())));

    // Gated with the group's own answer, the way `editor.save` is, so a menu row
    // and a scripted intent can never disagree about whether there is anything
    // to undo. The gate also keeps "Always forward" a refusal: the editor
    // declines its Undo, the intent climbs to here, and the group reports the
    // frozen domain as nothing to do rather than reaching the project's history.
    {
        let group = group.clone();
        ctx.register_action_global(
            Action::new(StandardEdit::Undo.id())
                .enabled_when(can_undo)
                .on_invoke(move |_i, c| {
                    group.undo();
                    // The undo may have rewritten prose behind an open tab, and
                    // `set_djot` only queues a document event.
                    c.request_frame();
                }),
        );
    }
    {
        let group = group.clone();
        ctx.register_action_global(
            Action::new(StandardEdit::Redo.id())
                .enabled_when(can_redo)
                .on_invoke(move |_i, c| {
                    group.redo();
                    c.request_frame();
                }),
        );
    }

    // ── Clipboard ─────────────────────────────────────────────────────────
    //
    // The same split: the focused widget answers the chord, these answer the
    // menu rows. Each gate is deliberately live rather than latched:
    // `UndoGroupViewModel` answers "is there a text surface to act on" from the
    // focused surface (or a rich editor that actually holds the caret), never
    // from the Format dock's sticky target — otherwise Edit ▸ Paste would stay
    // enabled for the rest of the session after one click in a scene, and paste
    // into a scene the writer is not looking at.
    let clip = [
        (
            StandardEdit::Cut,
            tr!(shortcut_name_edit_cut()),
            group.can_cut(),
        ),
        (
            StandardEdit::Copy,
            tr!(shortcut_name_edit_copy()),
            group.can_copy(),
        ),
        (
            StandardEdit::Paste,
            tr!(shortcut_name_edit_paste()),
            group.can_paste(),
        ),
        (
            StandardEdit::PastePlain,
            tr!(shortcut_name_edit_paste_plain()),
            group.can_paste(),
        ),
        (
            StandardEdit::SelectAll,
            tr!(shortcut_name_edit_select_all()),
            group.can_select_all(),
        ),
    ];
    for (command, name, enabled) in clip {
        ctx.register_shortcut_global(standard(command, name));
        let group = group.clone();
        ctx.register_action_global(Action::new(command.id()).enabled_when(enabled).on_invoke(
            move |_i, c| {
                match command {
                    StandardEdit::Cut => group.cut(c),
                    StandardEdit::Copy => group.copy(c),
                    StandardEdit::Paste => group.paste(c),
                    StandardEdit::PastePlain => group.paste_plain(c),
                    _ => group.select_all(),
                }
                c.request_frame();
            },
        ));
    }
}

#[cfg(test)]
mod tests {
    use std::cell::Cell;
    use std::rc::Rc;

    use frontend::common::undo_redo::UndoLabel;
    use teksilo::canvas::SizeProposal;
    use teksilo::core::widget::WidgetPlacement;
    use teksilo::core::widget_tree::WidgetTree;
    use teksilo::prelude::*;
    use teksilo::widgets::TextInputField;

    use super::register_with;
    use crate::edit::UndoGroupViewModel;
    use crate::edit::domains::UndoDomain;
    use crate::shared::shortcut_group::ShortcutGroup;

    #[derive(Default)]
    struct Fake {
        can_undo: Cell<bool>,
        undone: Cell<u32>,
    }

    impl UndoDomain for Fake {
        fn can_undo(&self) -> bool {
            self.can_undo.get()
        }
        fn can_redo(&self) -> bool {
            false
        }
        fn frozen(&self) -> bool {
            false
        }
        fn undo(&self) {
            self.undone.set(self.undone.get() + 1);
        }
        fn redo(&self) {}
        fn undo_label(&self) -> Option<UndoLabel> {
            None
        }
        fn redo_label(&self) -> Option<UndoLabel> {
            None
        }
    }

    /// The application root as far as these commands go: registers them, and
    /// holds one text field.
    struct Host {
        group: Option<UndoGroupViewModel>,
        field: Option<TextInputField>,
    }

    impl std::fmt::Debug for Host {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str("Host")
        }
    }

    impl Widget for Host {
        fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
            if let Some(group) = self.group.take() {
                register_with(ctx, &group);
            }
            let field = self.field.take().map(|f| ctx.add(f));
            field.into_iter().collect()
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

    fn mount(entity: Rc<Fake>) -> (WidgetTree, UndoGroupViewModel, WidgetId) {
        let group = UndoGroupViewModel::with_domains(
            Rc::new(Fake::default()),
            entity,
            Rc::new(|| false),
            Rc::new(|| {}),
            Rc::new(|_| {}),
        );
        let mut tree = WidgetTree::new();
        let host = tree.add(Host {
            group: Some(group.clone()),
            field: Some(TextInputField::new(Signal::new("Chapter one".to_string()))),
        });
        tree.layout(SizeProposal::exact(300.0, 40.0));
        let field = tree
            .children(host)
            .into_iter()
            .next()
            .expect("the field is the host's one child");
        (tree, group, field)
    }

    /// The group answers "no text target" for any surface it cannot see — a
    /// list, or here a field whose registry it was never handed. A gate on the
    /// *shortcut* would then hand the raw key to the widget, and since teksilo
    /// 0.17 no text widget or data view matches `Ctrl+A` in its own key
    /// handling: the chord would do nothing at all.
    #[test]
    fn a_closed_gate_never_takes_the_chord_from_the_focused_widget() {
        let (mut tree, group, field) = mount(Rc::new(Fake::default()));
        group.refresh();
        assert!(
            !group.can_select_all().get(),
            "the premise: the gate is closed"
        );

        tree.focus(field);
        // Focusing a field may select its text; start from a bare caret.
        tree.press_key(Key::End, Modifiers::NONE);
        let surface = tree
            .text_surfaces()
            .focused()
            .expect("the field holds the caret");
        assert!(!surface.has_selection(), "the premise: nothing selected");

        // `COMMAND`, not `CTRL`: a declared Ctrl chord is registered as ⌘ on macOS,
        // where a literal Ctrl+A would match no shortcut at all.
        tree.press_key(Key::A, Modifiers::COMMAND);
        assert!(
            surface.has_selection(),
            "Ctrl+A must still reach the field's own select-all"
        );
    }

    /// With no text widget focused, the walk ends at the application's action,
    /// which reaches the project's history.
    #[test]
    fn ctrl_z_outside_any_text_reaches_the_project() {
        let entity = Rc::new(Fake::default());
        entity.can_undo.set(true);
        let (mut tree, group, _field) = mount(entity.clone());
        let _claim = group.claim_entity(Signal::new(true));
        group.refresh();

        tree.press_key(Key::Z, Modifiers::COMMAND);
        assert_eq!(entity.undone.get(), 1);
    }

    /// A text widget with nothing to undo passes Ctrl+Z on, so the group's
    /// fall-through to the project is reachable from the keyboard and not only
    /// from the Edit menu; one with something to undo keeps it.
    #[test]
    fn ctrl_z_in_a_field_with_nothing_to_undo_reaches_the_project() {
        let entity = Rc::new(Fake::default());
        entity.can_undo.set(true);
        let (mut tree, group, field) = mount(entity.clone());
        group.attach_surfaces(tree.text_surfaces());
        let _claim = group.claim_entity(Signal::new(true));
        tree.focus(field);
        group.refresh();

        tree.press_key(Key::Z, Modifiers::COMMAND);
        assert_eq!(entity.undone.get(), 1, "the field had nothing to undo");

        tree.press_key(Key::End, Modifiers::NONE);
        tree.type_text(field, "!");
        tree.layout(SizeProposal::exact(300.0, 40.0));
        group.refresh();
        tree.press_key(Key::Z, Modifiers::COMMAND);
        assert_eq!(
            entity.undone.get(),
            1,
            "the field undid its own typing; the project was not touched"
        );
    }

    /// The upsert keeps one binding per command, teksilo's chords, and files it
    /// under the application's Edit group.
    #[test]
    fn the_standard_commands_are_filed_under_edit() {
        let (tree, _group, _field) = mount(Rc::new(Fake::default()));
        for command in StandardEdit::ALL {
            let effective = tree
                .shortcut_registry()
                .effective(command.id())
                .expect("registered");
            assert_eq!(
                effective.shortcut.category,
                Some(ShortcutGroup::Edit.key()),
                "{}",
                command.id()
            );
            assert_eq!(
                effective.shortcut.primary,
                command.shortcut().primary,
                "{}",
                command.id()
            );
        }
    }
}
