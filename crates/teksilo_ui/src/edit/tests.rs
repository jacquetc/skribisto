// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The routing table, case by case.

use std::cell::Cell;
use std::rc::Rc;

use frontend::common::undo_redo::UndoLabel;
use teksilo::prelude::*;

use super::domains::{DomainKind, UndoDomain};
use super::undo_group_vm::{UndoGroupViewModel, UndoTarget};

/// A domain whose answers the test sets directly.
#[derive(Default)]
struct Fake {
    can_undo: Cell<bool>,
    can_redo: Cell<bool>,
    frozen: Cell<bool>,
    undone: Cell<u32>,
    redone: Cell<u32>,
    label: Cell<Option<UndoLabel>>,
    /// How many times the four *locking* questions have been asked. On the real
    /// [`EntityDomain`](super::domains::EntityDomain) each one takes the
    /// process-wide undo-manager mutex, and `refresh` runs on the frame tick.
    queries: Cell<u32>,
}

impl Fake {
    fn new(kind: DomainKind) -> Rc<Self> {
        let _ = kind;
        Rc::new(Self::default())
    }
}

impl UndoDomain for Fake {
    fn can_undo(&self) -> bool {
        self.queries.set(self.queries.get() + 1);
        self.can_undo.get()
    }
    fn can_redo(&self) -> bool {
        self.queries.set(self.queries.get() + 1);
        self.can_redo.get()
    }
    fn frozen(&self) -> bool {
        self.frozen.get()
    }
    fn undo(&self) {
        self.undone.set(self.undone.get() + 1);
    }
    fn redo(&self) {
        self.redone.set(self.redone.get() + 1);
    }
    fn undo_label(&self) -> Option<UndoLabel> {
        self.queries.set(self.queries.get() + 1);
        self.label.get()
    }
    fn redo_label(&self) -> Option<UndoLabel> {
        self.queries.set(self.queries.get() + 1);
        self.label.get()
    }
}

struct Rig {
    group: UndoGroupViewModel,
    prose: Rc<Fake>,
    entity: Rc<Fake>,
    /// Drives the prose domain's *liveness* — "is the caret in a text surface" —
    /// which is separate from whether it has history.
    caret_in_text: Rc<Cell<bool>>,
    seals: Rc<Cell<u32>>,
}

fn rig() -> Rig {
    let prose = Fake::new(DomainKind::Prose);
    let entity = Fake::new(DomainKind::Entity);
    let caret_in_text = Rc::new(Cell::new(false));
    let seals = Rc::new(Cell::new(0));
    let live = caret_in_text.clone();
    let counted = seals.clone();
    let group = UndoGroupViewModel::with_domains(
        prose.clone(),
        entity.clone(),
        Rc::new(move || live.get()),
        Rc::new(move || counted.set(counted.get() + 1)),
        Rc::new(|_| {}),
    );
    Rig {
        group,
        prose,
        entity,
        caret_in_text,
        seals,
    }
}

/// Every routing case from the design, in one table.
#[test]
fn ctrl_z_means_the_surface_the_writer_is_looking_at() {
    let r = rig();
    r.prose.can_undo.set(true);
    r.entity.can_undo.set(true);

    // Caret in prose → the document's own history.
    r.caret_in_text.set(true);
    r.group.refresh();
    assert_eq!(r.group.routed(), Some(DomainKind::Prose));
    assert_eq!(r.group.undo_target().get(), UndoTarget::Typing);

    // Focus in a claiming structural surface → the project's history.
    let claim = r.group.claim_entity(Signal::new(true));
    r.caret_in_text.set(false);
    r.group.refresh();
    assert_eq!(r.group.routed(), Some(DomainKind::Entity));
    assert!(matches!(r.group.undo_target().get(), UndoTarget::Entity(_)));

    // Focus nowhere (a menu overlay has it) → the latch keeps the last answer,
    // so the row does not change meaning as the writer reaches for it.
    drop(claim);
    r.group.refresh();
    assert_eq!(
        r.group.routed(),
        Some(DomainKind::Entity),
        "dropping the claim must not silently re-point the row at prose"
    );
}

/// The bounded fall-through: further back, never sideways.
#[test]
fn an_exhausted_domain_falls_through_to_the_other_one() {
    let r = rig();
    r.caret_in_text.set(true);
    r.prose.can_undo.set(false);
    r.entity.can_undo.set(true);
    r.group.refresh();

    assert_eq!(
        r.group.routed(),
        Some(DomainKind::Entity),
        "a scene with nothing left to undo must still reach the project's history"
    );
    assert!(
        matches!(r.group.undo_target().get(), UndoTarget::Entity(_)),
        "and the row must say so *before* the writer presses anything — the \
         label is computed from the route, not from the active domain"
    );
}

/// "Always forward" refuses, and must not become a way round itself.
#[test]
fn a_frozen_domain_blocks_the_route_instead_of_redirecting_it() {
    let r = rig();
    r.caret_in_text.set(true);
    r.prose.frozen.set(true);
    r.prose.can_undo.set(true);
    r.entity.can_undo.set(true);
    r.group.refresh();

    assert_eq!(r.group.routed(), None);
    assert_eq!(r.group.undo_target().get(), UndoTarget::Frozen);
    assert!(
        !r.group.can_undo().get(),
        "the row greys out rather than vanishing — a row that disappears \
         teaches nobody why"
    );

    r.group.undo();
    assert_eq!(r.prose.undone.get(), 0);
    assert_eq!(
        r.entity.undone.get(),
        0,
        "the writing game freezes drafting; it must not quietly start eating \
         the binder's history instead"
    );
}

/// A modal takes the whole group out of play, which is what lets the global
/// shortcut fall through to an editor inside the modal.
#[test]
fn a_modal_suspends_routing_while_it_is_presented() {
    let r = rig();
    r.caret_in_text.set(true);
    r.prose.can_undo.set(true);
    r.group.refresh();
    assert!(r.group.can_undo().get());

    let guard = r.group.suspend();
    r.group.refresh();
    assert_eq!(r.group.routed(), None);
    assert!(!r.group.can_undo().get());

    r.group.undo();
    assert_eq!(r.prose.undone.get(), 0);

    drop(guard);
    r.group.refresh();
    assert!(
        r.group.can_undo().get(),
        "and it comes back when the modal closes"
    );
}

/// Nested modals: the count, not a flag.
#[test]
fn suspension_nests() {
    let r = rig();
    r.caret_in_text.set(true);
    r.prose.can_undo.set(true);
    let outer = r.group.suspend();
    let inner = r.group.suspend();
    drop(inner);
    r.group.refresh();
    assert!(
        !r.group.can_undo().get(),
        "closing the inner modal must not un-suspend the outer one"
    );
    drop(outer);
    r.group.refresh();
    assert!(r.group.can_undo().get());
}

#[test]
fn a_structural_step_closes_the_focused_documents_typing_burst() {
    let r = rig();
    r.entity.can_undo.set(true);
    let _claim = r.group.claim_entity(Signal::new(true));
    r.group.refresh();

    r.group.undo();
    assert_eq!(r.entity.undone.get(), 1);
    assert_eq!(
        r.seals.get(),
        1,
        "otherwise a burst typed before the undone command and one typed after \
         merge into a single step that straddles it"
    );

    // A prose step is not a dividing line, so it seals nothing.
    r.caret_in_text.set(true);
    r.prose.can_undo.set(true);
    r.group.refresh();
    r.group.undo();
    assert_eq!(r.prose.undone.get(), 1);
    assert_eq!(r.seals.get(), 1);
}

#[test]
fn a_structural_command_landing_closes_the_burst_too() {
    // The half that matters more, and the half that was missing: `undo`/`redo`
    // are not the only dividing lines. *Type, rename a chapter, type* is the
    // case the merge rule actually gets wrong, and no undo happens in it at all
    // — the writer then presses Ctrl+Z once and loses text from before the
    // rename. The project shell drives this from `UndoRedoEvent::StackChanged`,
    // which fires once per entry the entity history gains.
    let r = rig();
    assert_eq!(r.seals.get(), 0);

    r.group.seal_prose_merge();
    assert_eq!(r.seals.get(), 1);

    // Independent of focus and of what either domain can currently undo: the
    // command landed, so the line was crossed wherever the caret happens to be.
    r.caret_in_text.set(true);
    r.group.refresh();
    r.group.seal_prose_merge();
    assert_eq!(r.seals.get(), 2);
}

#[test]
fn redo_routes_by_its_own_availability_not_undos() {
    let r = rig();
    r.caret_in_text.set(true);
    r.prose.can_undo.set(true);
    r.prose.can_redo.set(false);
    r.entity.can_redo.set(true);
    r.group.refresh();

    assert_eq!(r.group.undo_target().get(), UndoTarget::Typing);
    assert!(
        matches!(r.group.redo_target().get(), UndoTarget::Entity(_)),
        "Undo and Redo are answered separately: a document with edits to undo \
         and nothing to redo must not make Redo dead when the project has some"
    );
    assert!(r.group.can_redo().get());
}

#[test]
fn nothing_focused_and_nothing_recorded_offers_nothing() {
    let r = rig();
    r.group.refresh();
    assert_eq!(r.group.routed(), None);
    assert_eq!(r.group.undo_target().get(), UndoTarget::Nothing);
    assert!(!r.group.can_undo().get() && !r.group.can_redo().get());
}

/// A focused text widget answers for its own typing, ahead of the project.
///
/// This is the reflex the design panel flagged: rename a chapter, press Ctrl+Z
/// out of habit, and without a text domain the chord would reach the project's
/// history and un-trash a folder instead of restoring the typed name.
///
/// The surface comes from teksilo's own registry — every text widget registers
/// itself on build — so the answer covers widgets this application never knew
/// about, which is what makes taking the chord globally safe. The registry
/// itself is proven in `teksilo-widgets/tests/text_surface_registry.rs`; what is
/// pinned here is the *ordering*: a text widget inside a claiming structural
/// surface still wins, because the caret decides.
#[test]
fn a_focused_text_widget_answers_ahead_of_the_project() {
    let r = rig();
    r.entity.can_undo.set(true);
    let _claim = r.group.claim_entity(Signal::new(true));
    r.group.refresh();
    assert_eq!(
        r.group.routed(),
        Some(DomainKind::Entity),
        "with no text widget focused, the structural surface answers"
    );

    // A real tree, a real field, real focus — the registry is the framework's,
    // so there is nothing to fake.
    let mut tree = teksilo::core::widget_tree::WidgetTree::new();
    let field = teksilo::widgets::TextInputField::new(Signal::new(String::new()));
    let id = tree.add(field);
    tree.layout(teksilo::canvas::SizeProposal::exact(200.0, 40.0));
    r.group.attach_surfaces(tree.text_surfaces());

    r.group.refresh();
    assert_eq!(
        r.group.routed(),
        Some(DomainKind::Entity),
        "an unfocused field must not steal the chord"
    );

    tree.focus(id);
    r.group.refresh();
    // The field is live but empty, so it has nothing to undo and the route falls
    // through to the project — the honest answer, and announced by the label.
    assert_eq!(r.group.routed(), Some(DomainKind::Entity));
    assert_eq!(
        r.group.undo_target().get(),
        UndoTarget::Entity(None),
        "falling through must be described, not hidden"
    );
}

#[test]
fn a_dropped_claim_stops_answering() {
    let r = rig();
    r.entity.can_undo.set(true);
    {
        let _claim = r.group.claim_entity(Signal::new(true));
        r.group.refresh();
        assert_eq!(r.group.routed(), Some(DomainKind::Entity));
    }
    // The panel is gone. Nothing is focused, so the latch answers — but the
    // claim itself must not still be consulted, or the group would keep routing
    // at a surface that is no longer on screen.
    r.prose.can_undo.set(true);
    r.caret_in_text.set(true);
    r.group.refresh();
    assert_eq!(r.group.routed(), Some(DomainKind::Prose));
}

/// What one frame of polling actually costs, pinned so it cannot creep back.
///
/// The tick is the design (see [`UndoGroupViewModel::refresh`]), but its budget
/// is not open-ended: the four questions counted here are the ones
/// `EntityDomain` answers by taking the process-wide undo-manager mutex. An
/// earlier shape asked them five times a frame because "which domain" and "what
/// is it called" were resolved in separate passes — three routings per frame
/// instead of two. `step_state` collapsed that, and this is what keeps it
/// collapsed.
#[test]
fn one_refresh_asks_the_entity_history_at_most_four_times() {
    let r = rig();
    r.entity.can_undo.set(true);
    r.entity.can_redo.set(true);

    // Both prose steps live — the moment just after an undo. The route never
    // leaves prose for either step, so the mutex is not touched at all.
    r.caret_in_text.set(true);
    r.prose.can_undo.set(true);
    r.prose.can_redo.set(true);
    r.entity.queries.set(0);
    r.group.refresh();
    assert_eq!(
        r.entity.queries.get(),
        0,
        "a document that can answer both steps must not poll the project at all"
    );

    // The frame that actually repeats while someone writes: prose has a burst
    // to undo and nothing to redo, because the last keystroke cleared the redo
    // stack. Undo stays in the document; only Redo falls through, for one
    // availability query and one label.
    r.prose.can_redo.set(false);
    r.entity.queries.set(0);
    r.group.refresh();
    assert_eq!(
        r.entity.queries.get(),
        2,
        "typing costs the project one step's worth of questions, not two"
    );

    // The worst case: the project answers for both steps, so each costs one
    // availability query and one label — two per step, four per frame, no more.
    let _claim = r.group.claim_entity(Signal::new(true));
    r.caret_in_text.set(false);
    r.entity.queries.set(0);
    r.group.refresh();
    assert_eq!(r.entity.queries.get(), 4);

    // The fall-through costs the same: an exhausted document reaches the
    // project once per step, not once per step per pass.
    r.caret_in_text.set(true);
    r.prose.can_undo.set(false);
    r.prose.can_redo.set(false);
    r.entity.queries.set(0);
    r.group.refresh();
    assert_eq!(r.entity.queries.get(), 4);
}
