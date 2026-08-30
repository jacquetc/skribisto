// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! `UndoGroupViewModel` — which history Ctrl+Z means, and what it will take back.
//!
//! Modelled on Qt's `QUndoGroup`: several stacks, one of them active, and a menu
//! that mirrors whichever it is. Qt's own rule is that the programmer sets the
//! active stack on focus change, and that is what happens here.
//!
//! # Why not one merged history
//!
//! Because a single chronological Ctrl+Z spanning every document is Excel's
//! design, and Excel is the cautionary tale: users report undo silently
//! reversing edits in a workbook they were not looking at, discovered only after
//! it had been saved. The HCI work says the same — Seifried et al. (CHI 2012)
//! found people expect *regional* undo scoped to what they can see, and the
//! global condition in *Just Undo It* (CHI 2024) produced exactly the
//! interference you would predict. So undo is scoped to the surface with the
//! caret, and the menu **names its target** so the scope is never a guess.
//!
//! # The three states this has to survive
//!
//! 1. **Opening the Edit menu blurs the editor.** Focus moves to the menu
//!    overlay, which is inside no claim. The [`latch`](Inner::latch) answers
//!    instead, so the row does not change meaning under the writer's hand at the
//!    exact moment they reach for it.
//! 2. **A modal.** Here the latch is *wrong*: Ctrl+Z inside Settings must not
//!    reach the manuscript. A modal holds an [`UndoSuspend`], and while one is
//!    alive the group reports nothing to undo — which makes the global shortcut
//!    fall through to the focused widget, so an editor inside the modal still
//!    undoes itself.
//! 3. **"Always forward".** A frozen prose domain **blocks** the route rather
//!    than falling through to the entity one; otherwise the writing game would
//!    quietly redirect Ctrl+Z into the binder's history instead of refusing it.

use std::cell::{Cell, RefCell};
use std::rc::Rc;

use frontend::common::undo_redo::UndoLabel;
use teksilo::prelude::*;

use super::domains::{DomainKind, EntityDomain, ProseDomain, UndoDomain};
use crate::format::FormatViewModel;

/// What the next Undo (or Redo) would act on — the typed state a menu row
/// renders. The view-model publishes the answer; naming it in the reader's
/// language is the view's job.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum UndoTarget {
    /// Nothing to take back anywhere the writer is looking.
    Nothing,
    /// The focused document's own prose.
    Typing,
    /// A structural command, with the backend's machine key for it when the
    /// command named itself.
    Entity(Option<UndoLabel>),
    /// Refused: the writing game is on in this surface.
    Frozen,
}

impl UndoTarget {
    /// Is there something to do? `Frozen` is deliberately *not* actionable — the
    /// row stays visible and greys out, because a row that vanishes teaches
    /// nobody why.
    pub(crate) fn is_actionable(&self) -> bool {
        !matches!(self, Self::Nothing | Self::Frozen)
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Step {
    Undo,
    Redo,
}

struct Claim {
    id: u64,
    focused: Signal<bool>,
}

struct Inner {
    // Held as trait objects so the routing table below can be exercised
    // headlessly against fakes. The table is the whole design — "what does
    // Ctrl+Z mean here" in a dozen focus contexts — and it would otherwise be
    // testable only by standing up an editor, a save queue and a live project.
    prose: Rc<dyn UndoDomain>,
    entity: Rc<dyn UndoDomain>,
    /// Is a rich editor holding the caret? The prose domain's liveness, which
    /// is a different question from whether it has history.
    prose_live: Rc<dyn Fn() -> bool>,
    /// The framework's registry of text-editing widgets in this window.
    ///
    /// Asked, rather than keeping a list here — and that is what makes the
    /// global chords safe to register. A list maintained in the application is
    /// correct the day it is written and wrong the first time someone adds a
    /// field, and it fails *silently*: Ctrl+Z in the field it forgot would undo
    /// a structural command instead of the writer's typing. teksilo knows every
    /// text widget in the tree because each registers itself on build, so the
    /// answer is complete by construction.
    surfaces: RefCell<Option<teksilo::core::text_surface::TextSurfaces>>,
    /// Close the focused document's merge chain — run after a structural step.
    on_entity_step: Rc<dyn Fn()>,
    /// Hand this window's editors to the entity domain, once they exist.
    attach_editors: Rc<dyn Fn(crate::editors::EditorsViewModel)>,
    /// The rich editor holding the caret, for the clipboard commands. Same
    /// resolver the Format menu uses, so the two can never disagree about which
    /// surface a command acts on.
    editor: Rc<dyn Fn() -> Option<teksilo::widgets::rich_text::EditorHandle>>,
    /// Structural surfaces' focus flags. Any one live means the writer is in the
    /// project, not in a document.
    claims: RefCell<Vec<Claim>>,
    next_claim_id: Cell<u64>,
    /// The last domain that genuinely held focus — see the module doc.
    latch: Cell<Option<DomainKind>>,
    /// Non-zero while a modal is presented.
    suspended: Cell<usize>,
    undo_target: Signal<UndoTarget>,
    redo_target: Signal<UndoTarget>,
    can_undo: Signal<bool>,
    can_redo: Signal<bool>,
    /// Mirrors for the clipboard rows. Polled once a frame beside the rest,
    /// because "is anything selected" has no event a menu could subscribe to.
    can_cut: Signal<bool>,
    can_copy: Signal<bool>,
    can_paste: Signal<bool>,
    can_select_all: Signal<bool>,
    /// Which domain the next Undo will actually reach. What the label describes
    /// — never merely the active one, because of the fall-through below.
    routed: Cell<Option<DomainKind>>,
}

/// A structural surface's registration. Removing it on drop is not tidiness:
/// a claim outliving its widget would keep routing Ctrl+Z at a panel that is no
/// longer on screen.
pub struct UndoClaim {
    group: UndoGroupViewModel,
    id: u64,
}

impl Drop for UndoClaim {
    fn drop(&mut self) {
        self.group.release(self.id);
    }
}

/// Held for as long as a modal is presented. While any is alive the group has
/// nothing to offer, so the global shortcut falls through to the focused widget
/// — which is how an editor inside the modal keeps its own Ctrl+Z.
pub struct UndoSuspend {
    group: UndoGroupViewModel,
}

impl Drop for UndoSuspend {
    fn drop(&mut self) {
        self.group.resume();
    }
}

#[derive(Clone)]
pub struct UndoGroupViewModel {
    inner: Rc<Inner>,
}

impl UndoGroupViewModel {
    pub fn new(format: FormatViewModel, entity: EntityDomain) -> Self {
        let prose = ProseDomain::new(format.clone());
        let live = prose.clone();
        let seal_format = format.clone();
        let seal_entity = entity.clone();
        let attach_entity = entity.clone();
        let editor_of = format.clone();
        Self::with_domains_and_editor(
            Rc::new(move || editor_of.handle_for_commands()),
            Rc::new(prose),
            Rc::new(entity),
            Rc::new(move || live.is_live()),
            Rc::new(move || seal_entity.seal_prose_merge(&seal_format)),
            Rc::new(move |editors| attach_entity.attach_editors(editors)),
        )
    }

    /// The constructor the tests use. It and [`new`](Self::new) both funnel into
    /// `with_domains_and_editor`, so what the tests exercise is the shipped
    /// routing, not a parallel copy of it. No rich editor to reach.
    #[cfg(test)]
    pub(crate) fn with_domains(
        prose: Rc<dyn UndoDomain>,
        entity: Rc<dyn UndoDomain>,
        prose_live: Rc<dyn Fn() -> bool>,
        on_entity_step: Rc<dyn Fn()>,
        attach_editors: Rc<dyn Fn(crate::editors::EditorsViewModel)>,
    ) -> Self {
        Self::with_domains_and_editor(
            Rc::new(|| None),
            prose,
            entity,
            prose_live,
            on_entity_step,
            attach_editors,
        )
    }

    #[allow(clippy::too_many_arguments)]
    fn with_domains_and_editor(
        editor: Rc<dyn Fn() -> Option<teksilo::widgets::rich_text::EditorHandle>>,
        prose: Rc<dyn UndoDomain>,
        entity: Rc<dyn UndoDomain>,
        prose_live: Rc<dyn Fn() -> bool>,
        on_entity_step: Rc<dyn Fn()>,
        attach_editors: Rc<dyn Fn(crate::editors::EditorsViewModel)>,
    ) -> Self {
        Self {
            inner: Rc::new(Inner {
                prose,
                entity,
                prose_live,
                on_entity_step,
                attach_editors,
                editor,
                claims: RefCell::new(Vec::new()),
                surfaces: RefCell::new(None),
                next_claim_id: Cell::new(1),
                latch: Cell::new(None),
                suspended: Cell::new(0),
                undo_target: Signal::new(UndoTarget::Nothing),
                redo_target: Signal::new(UndoTarget::Nothing),
                can_undo: Signal::new(false),
                can_redo: Signal::new(false),
                can_cut: Signal::new(false),
                can_copy: Signal::new(false),
                can_paste: Signal::new(false),
                can_select_all: Signal::new(false),
                routed: Cell::new(None),
            }),
        }
    }

    /// Declare that a surface edits the **project**, so Ctrl+Z in it means the
    /// structural history.
    ///
    /// `focused` is the surface's own focus flag — `focus_within` for a panel,
    /// or a field's `focused_signal`. Every surface with undoable verbs must
    /// claim, or Ctrl+Z there keeps reaching whatever was focused before it,
    /// which is the stale-latch bug this design exists to avoid. Today one
    /// claim covers the whole project surface (`App::project_shell` binds it to
    /// the content root's `focus_within`), so a panel added inside that subtree
    /// inherits it; a surface mounted *outside* it needs its own claim, and
    /// nothing checks that for you.
    pub(crate) fn claim_entity(&self, focused: Signal<bool>) -> UndoClaim {
        let id = self.inner.next_claim_id.get();
        self.inner.next_claim_id.set(id + 1);
        self.inner.claims.borrow_mut().push(Claim { id, focused });
        UndoClaim {
            group: self.clone(),
            id,
        }
    }

    /// Re-point the entity domain at this window's editors, so an entity undo
    /// flushes open buffers before it touches the store. Called on every
    /// `App::build`; idempotent.
    pub(crate) fn attach_editors(&self, editors: crate::editors::EditorsViewModel) {
        self.inner.attach_editors.as_ref()(editors);
    }

    /// Point the group at this window's registry of text-editing widgets.
    /// Called once from `App::build`; idempotent.
    pub(crate) fn attach_surfaces(&self, surfaces: teksilo::core::text_surface::TextSurfaces) {
        *self.inner.surfaces.borrow_mut() = Some(surfaces);
    }

    fn release(&self, id: u64) {
        self.inner.claims.borrow_mut().retain(|c| c.id != id);
    }

    /// The text-editing widget holding the caret, whatever kind it is.
    fn focused_surface(&self) -> Option<Rc<dyn teksilo::core::text_surface::TextSurface>> {
        self.inner.surfaces.borrow().as_ref()?.focused()
    }

    /// Suspend routing for as long as the returned guard lives — for a modal.
    ///
    /// Load-bearing, because the chords *are* registered globally: without a
    /// suspension, Ctrl+Z over a Settings checkbox would fall back to whatever
    /// the writer was last doing in the project behind the modal. Held by
    /// `SettingsPanel` for the panel's life; exercised by `edit::tests`.
    pub(crate) fn suspend(&self) -> UndoSuspend {
        self.inner.suspended.set(self.inner.suspended.get() + 1);
        UndoSuspend {
            group: self.clone(),
        }
    }

    fn resume(&self) {
        self.inner
            .suspended
            .set(self.inner.suspended.get().saturating_sub(1));
    }

    /// The domain the writer is in: a focused editor, else a claiming
    /// structural surface, else whichever it last was.
    fn active(&self) -> Option<DomainKind> {
        if (self.inner.prose_live)() {
            self.inner.latch.set(Some(DomainKind::Prose));
            return Some(DomainKind::Prose);
        }
        // Before the structural claim, because a text widget lives *inside* a
        // claiming surface: a rename box in the outline is focus-within the
        // outline, and the caret is what decides.
        if self.focused_surface().is_some() {
            self.inner.latch.set(Some(DomainKind::TextField));
            return Some(DomainKind::TextField);
        }
        if self.inner.claims.borrow().iter().any(|c| c.focused.get()) {
            self.inner.latch.set(Some(DomainKind::Entity));
            return Some(DomainKind::Entity);
        }
        self.inner.latch.get()
    }

    /// The domain for `kind`, if it can be resolved right now.
    ///
    /// `TextField` is built on demand around whichever field holds the caret,
    /// so it exists only while one does — which is exactly when it can be
    /// routed to.
    fn domain(&self, kind: DomainKind) -> Option<Rc<dyn UndoDomain>> {
        match kind {
            DomainKind::Prose => Some(self.inner.prose.clone()),
            DomainKind::Entity => Some(self.inner.entity.clone()),
            DomainKind::TextField => self
                .focused_surface()
                .map(|s| Rc::new(super::domains::FieldDomain::new(s)) as Rc<dyn UndoDomain>),
        }
    }

    /// Where a step would actually land.
    ///
    /// **The one deliberate deviation from `QUndoGroup`**, whose active stack is
    /// a single pointer and which therefore stops dead when that stack is empty.
    /// Stopping dead would strand a trashed chapter forever the moment the
    /// writer put the caret back in a scene. The fall-through is bounded by
    /// three properties, each tested:
    ///
    /// 1. it only reaches *further back*, never sideways — with prose active and
    ///    prose exhausted, the entity history is the only other place a command
    ///    can be;
    /// 2. it can never take back something the writer was not told about,
    ///    because the label is computed from **this** answer, so the row already
    ///    reads "Undo trashing «Chapter 3»" before anything is pressed;
    /// 3. a **frozen** domain blocks it entirely — no escape hatch past the
    ///    writing game.
    fn route(&self, step: Step) -> Option<DomainKind> {
        if self.inner.suspended.get() > 0 {
            return None;
        }
        let active = self.active()?;
        let d = self.domain(active)?;
        if d.frozen() {
            return None;
        }
        let can = |d: &dyn UndoDomain| match step {
            Step::Undo => d.can_undo(),
            Step::Redo => d.can_redo(),
        };
        if can(&*d) {
            return Some(active);
        }
        // Only ever *further back*, never sideways: the project's history is
        // the one place a command can be that a text surface is not. A field
        // with nothing left to undo therefore reaches the project, and a
        // document with nothing left does too — but neither ever reaches the
        // other, which would be undoing somewhere the writer is not looking.
        let other = DomainKind::Entity;
        if active == other {
            return None;
        }
        let o = self.domain(other)?;
        (!o.frozen() && can(&*o)).then_some(other)
    }

    /// Where a step would land **and** what to call it, resolved in one pass.
    ///
    /// One pass because each half takes the process-wide undo-manager lock:
    /// `can_undo`, `can_redo` and both `*_label`s on the entity domain all do,
    /// and this runs on the frame tick. Answering "which domain" and "what is
    /// it called" in separate passes routed three times per frame instead of
    /// twice, for five lock acquisitions instead of four in the worst case.
    ///
    /// The lock is uncontended by construction rather than by luck: every
    /// caller of `undo_redo_commands` runs on the UI thread, and a long
    /// operation takes `long_operation_manager`, never this one. So what the
    /// collapse saved is the acquisitions themselves — four of them measure
    /// ~600 ns in a debug build and ~80 ns in release — never a wait on a
    /// worker. That is small enough to keep the poll (see
    /// [`refresh`](Self::refresh)), and small only because it is bounded, which
    /// is what `one_refresh_asks_the_entity_history_at_most_four_times` exists
    /// to keep true.
    fn step_state(&self, step: Step) -> (UndoTarget, Option<DomainKind>) {
        if self.inner.suspended.get() > 0 {
            return (UndoTarget::Nothing, None);
        }
        // A frozen *active* domain is reported as such even when the other one
        // has history: the writer is being told why the row is grey, not
        // offered a way round the game.
        if let Some(active) = self.active()
            && self.domain(active).is_some_and(|d| d.frozen())
        {
            return (UndoTarget::Frozen, None);
        }
        let routed = self.route(step);
        let target = match routed {
            None => UndoTarget::Nothing,
            Some(DomainKind::Prose | DomainKind::TextField) => UndoTarget::Typing,
            Some(DomainKind::Entity) => UndoTarget::Entity(match step {
                Step::Undo => self.inner.entity.undo_label(),
                Step::Redo => self.inner.entity.redo_label(),
            }),
        };
        (target, routed)
    }

    /// Recompute the published state, from the frame tick — like
    /// `FormatViewModel::refresh`, and for the same reason.
    ///
    /// # Why this polls, and goes on polling
    ///
    /// Raised twice in review, so the answer lives here rather than in a review
    /// thread. What one refresh depends on, and what each dependency offers a
    /// subscriber:
    ///
    /// | Dependency | Something to subscribe to? |
    /// |---|---|
    /// | the project's history | **yes** — `UndoRedoEvent::{StackChanged, Undone, Redone}`, and `AppIds::stack_id` is a `Signal` |
    /// | the focused document's own history | no — read through `FormatViewModel`'s mirrors, which are themselves refreshed from this same tick, because reading editor state from inside the editor's own change notification borrows a cell it is already holding |
    /// | which surface has focus | partly — `TextSurfaces::focus_signal` exists, but "is a registered editor focused" and "is a claiming panel focused" are `any()` over two collections that grow and shrink as widgets and panels come and go, not one signal to observe |
    /// | the command filter — "Always forward" | no — read off the editor handle; turning the game on writes no signal |
    /// | the suspend count | no — a `Cell`, written by [`suspend`](Self::suspend) and by `UndoSuspend`'s `Drop` |
    /// | the four clipboard rows | no — `TextSurface::{has_selection, is_read_only, allows_copy}` are plain `bool` methods, and a selection changes on every caret move in every text widget in the window |
    ///
    /// One of the six has an event, and it is the cheap one. The entity queries
    /// are bounded at four `HashMap` lookups behind an uncontended mutex per
    /// frame (see [`step_state`](Self::step_state)), and the frame that
    /// actually repeats sixty times a second — mid-burst, caret in a scene —
    /// costs one or two of the four, not the full budget: Undo is answered by
    /// the document and never leaves it, while Redo, whose prose stack the last
    /// keystroke cleared, falls through and asks the project once, plus a label
    /// if the project has one to give. Zero is reachable but rarer than it
    /// looks: it needs both prose steps live, which is the moment just after an
    /// undo. Driving that quarter from events would leave the tick in place for
    /// the other five and buy back a few hundred nanoseconds of a 16.7 ms frame.
    ///
    /// It would also cost something. These signals gate the **global**
    /// `Ctrl+Z`, `Ctrl+X` and `Ctrl+C`, not merely menu rows, and a disabled
    /// shortcut falls through to the focused widget — so a briefly stale value
    /// does not just look wrong, it changes what a keystroke does. A cache fed
    /// by events is stale across exactly the window between a command landing
    /// and its event being delivered back to the UI thread, which is where a
    /// keystroke lands.
    ///
    /// `StackChanged` therefore has a job here, and it is not this one: it
    /// seals the focused document's merge chain when a structural command lands
    /// (see [`seal_prose_merge`](Self::seal_prose_merge)). That is an *edge*,
    /// not a state — the one thing a poll genuinely cannot reconstruct.
    ///
    /// Bounded on the other side as well: `set_if_changed` means a frame in
    /// which nothing moved writes no signal, so the poll cannot itself provoke
    /// a rebuild.
    pub(crate) fn refresh(&self) {
        let (undo, routed) = self.step_state(Step::Undo);
        let (redo, _) = self.step_state(Step::Redo);
        self.inner.routed.set(routed);
        set_if_changed(&self.inner.can_undo, undo.is_actionable());
        set_if_changed(&self.inner.can_redo, redo.is_actionable());

        // A modal's own field is a legitimate clipboard target, so these are
        // *not* gated on suspension the way undo is: suspending exists to keep
        // Ctrl+Z out of the manuscript, not to make Copy stop working in a
        // dialog.
        let target = self.has_text_target();
        let selection = target && self.has_selection();
        // `allows_copy` is a separate question from `has_selection`: a masked
        // secure field has a selection and still refuses to yield it, and the
        // widget enforces that itself — so a row that lit up here would light
        // up over a command that cannot run.
        let copyable = selection && self.allows_copy();
        set_if_changed(&self.inner.can_cut, copyable && self.accepts_paste());
        set_if_changed(&self.inner.can_copy, copyable);
        set_if_changed(&self.inner.can_paste, target && self.accepts_paste());
        set_if_changed(&self.inner.can_select_all, target);
        if self.inner.undo_target.get() != undo {
            self.inner.undo_target.set(undo);
        }
        if self.inner.redo_target.get() != redo {
            self.inner.redo_target.set(redo);
        }
    }

    pub(crate) fn undo(&self) {
        let Some(kind) = self.route(Step::Undo) else {
            return;
        };
        let Some(d) = self.domain(kind) else { return };
        d.undo();
        self.after_step(kind);
        self.refresh();
    }

    pub(crate) fn redo(&self) {
        let Some(kind) = self.route(Step::Redo) else {
            return;
        };
        let Some(d) = self.domain(kind) else { return };
        d.redo();
        self.after_step(kind);
        self.refresh();
    }

    /// A structural step is a dividing line the document engine cannot see, so
    /// the focused document's typing burst is closed against it — otherwise one
    /// Ctrl+Z in the editor afterwards takes back text typed on both sides of
    /// the command that was just undone.
    fn after_step(&self, kind: DomainKind) {
        if kind == DomainKind::Entity {
            (self.inner.on_entity_step)();
        }
    }

    /// The same dividing line, drawn when a structural command **lands** rather
    /// than when one is taken back.
    ///
    /// This is the case the merge rule actually gets wrong, and for a while it
    /// was the case nothing called: `after_step` fires only from `undo`/`redo`,
    /// so a writer who typed, renamed a chapter, and typed again still got one
    /// entry spanning both bursts — the scenario `break_undo_merge`'s own doc
    /// comment describes. Wired from the project shell to
    /// `UndoRedoEvent::StackChanged`, which is emitted once per entry: a group
    /// of fifty imported tags seals once, when the group closes, not fifty times.
    pub(crate) fn seal_prose_merge(&self) {
        (self.inner.on_entity_step)();
    }

    // ── Clipboard ─────────────────────────────────────────────────────────
    //
    // Not undo, but the same question: *which* text surface does this act on?
    // Answering it twice, in two places, is how the Edit menu and the keyboard
    // would come to disagree. The order is the same — a focused field first,
    // because a rename box lives inside a panel and the caret decides.

    /// The rich editor to act on, and **only while it holds the caret**.
    ///
    /// `FormatViewModel`'s resolver is deliberately *sticky*: it keeps its last
    /// target so a Format command survives the trip through the menu bar, which
    /// blurs the editor. That latch is exactly wrong here. These chords are
    /// registered globally, so a gate built on the latch stays true for the rest
    /// of the session once any editor has been focused — and `Ctrl+V` pressed
    /// with the caret in the outline would then paste into a scene the writer is
    /// not looking at, while `Ctrl+A` would take select-all away from every
    /// multi-selection list in the window.
    ///
    /// A focused rich editor registers itself as a text surface like any other
    /// widget, so `focused_surface()` already answers for it; this is the
    /// belt beside those braces, for the frame before the registry is attached.
    fn live_editor(&self) -> Option<teksilo::widgets::rich_text::EditorHandle> {
        if !(self.inner.prose_live)() {
            return None;
        }
        (self.inner.editor)()
    }

    /// Is there a text surface to act on at all?
    pub(crate) fn has_text_target(&self) -> bool {
        self.focused_surface().is_some() || self.live_editor().is_some()
    }

    /// Is anything selected in the focused text surface?
    pub(crate) fn has_selection(&self) -> bool {
        if let Some(surface) = self.focused_surface() {
            return surface.has_selection();
        }
        self.live_editor().is_some_and(|h| h.has_selection().get())
    }

    /// May the focused surface's content be copied at all?
    ///
    /// A masked secure field says no, and the row must grey out rather than
    /// light up and do nothing: the widget itself refuses the copy
    /// (`clipboard_copy` bails on `!copy_allowed()`), so a menu that offered it
    /// would simply be lying about what the keyboard can do.
    pub(crate) fn allows_copy(&self) -> bool {
        if let Some(surface) = self.focused_surface() {
            return surface.allows_copy();
        }
        self.live_editor().is_some()
    }

    /// May the focused surface accept a paste? A read-only field may not.
    pub(crate) fn accepts_paste(&self) -> bool {
        if let Some(surface) = self.focused_surface() {
            return !surface.is_read_only();
        }
        self.live_editor().is_some()
    }

    pub(crate) fn cut(&self, ctx: &EventContext) {
        if let Some(surface) = self.focused_surface() {
            surface.cut(ctx);
        } else if let Some(h) = self.live_editor() {
            h.cut(ctx);
        }
    }

    pub(crate) fn copy(&self, ctx: &EventContext) {
        if let Some(surface) = self.focused_surface() {
            surface.copy(ctx);
        } else if let Some(h) = self.live_editor() {
            h.copy(ctx);
        }
    }

    pub(crate) fn paste(&self, ctx: &EventContext) {
        if let Some(surface) = self.focused_surface() {
            surface.paste(ctx);
        } else if let Some(h) = self.live_editor() {
            h.paste(ctx);
        }
    }

    pub(crate) fn paste_plain(&self, ctx: &EventContext) {
        if let Some(surface) = self.focused_surface() {
            surface.paste_plain(ctx);
        } else if let Some(h) = self.live_editor() {
            h.paste_unformatted(ctx);
        }
    }

    pub(crate) fn select_all(&self) {
        if let Some(surface) = self.focused_surface() {
            surface.select_all();
        } else if let Some(h) = self.live_editor() {
            h.select_all();
        }
    }

    pub(crate) fn can_undo(&self) -> Signal<bool> {
        self.inner.can_undo.clone()
    }

    pub(crate) fn can_redo(&self) -> Signal<bool> {
        self.inner.can_redo.clone()
    }

    pub(crate) fn can_cut(&self) -> Signal<bool> {
        self.inner.can_cut.clone()
    }

    pub(crate) fn can_copy(&self) -> Signal<bool> {
        self.inner.can_copy.clone()
    }

    pub(crate) fn can_paste(&self) -> Signal<bool> {
        self.inner.can_paste.clone()
    }

    pub(crate) fn can_select_all(&self) -> Signal<bool> {
        self.inner.can_select_all.clone()
    }

    pub(crate) fn undo_target(&self) -> Signal<UndoTarget> {
        self.inner.undo_target.clone()
    }

    pub(crate) fn redo_target(&self) -> Signal<UndoTarget> {
        self.inner.redo_target.clone()
    }

    /// Which domain the next Undo would reach — for tests and diagnostics.
    #[cfg(test)]
    pub(crate) fn routed(&self) -> Option<DomainKind> {
        self.inner.routed.get()
    }
}

/// Write only on a real change: a `Signal` notifies unconditionally, and these
/// are recomputed every frame.
fn set_if_changed(signal: &Signal<bool>, value: bool) {
    if signal.get() != value {
        signal.set(value);
    }
}
