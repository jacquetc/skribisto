// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Prose and synopsis side by side, scrolling as one.
//!
//! Deliberately one module and **not** subdivided. `WidthProbe` decides whether
//! there is room for two columns, `PageScrollPort` owns the scroll offset the
//! pair shares, `SideSync` keeps the two heights and offsets in step, and
//! `SynopsisPaneEffects` drives the transitions between the two arrangements.
//! They read as four types and behave as one state machine: each reacts to
//! values the others write, within the same frame, and a boundary drawn between
//! any two of them would be a boundary the frame does not respect.
//!
//! The hysteresis on the breakpoint is part of that: without it a tab parked at
//! the threshold flips arrangement every frame, because switching arrangements
//! is itself what changes the measured width.

use super::*;

/// Minimum width the **prose** column keeps when the synopsis sits beside it.
///
/// Side placement is a preference the layout cannot always honour. The outer
/// editor split already floors a pane at `PANE_MIN_WIDTH` (320px) — that is the
/// entire budget a secondary pane gets — so subtracting a ~280px synopsis from it
/// would leave a prose column too narrow to write in. Below
/// `side_width + PROSE_MIN_WIDTH` of available width, [`WidthProbe`] renders the
/// Top layout instead, rather than honouring the setting into unusability.
pub const PROSE_MIN_WIDTH: f32 = 320.0;

/// Dead band around the Top/Side breakpoint, in px.
///
/// Without it a window left to rest exactly on the threshold flips layout on
/// every pixel of jitter — and since each flip is a relayout that feeds the next
/// decision, the two can chase each other indefinitely. The band makes the
/// crossing points asymmetric (enter Side higher than you leave it), so a width
/// has to move a real distance to change the answer.
pub(super) const SIDE_BREAKPOINT_HYSTERESIS: f32 = 24.0;

pub(super) const MODE_TOP: usize = 0;
pub(super) const MODE_SIDE: usize = 1;

/// Picks the **Top** or **Side** synopsis layout from the width actually
/// available, and shows the chosen one.
///
/// Placement is a global setting, but whether it can be *honoured* is local: the
/// same preference has to produce a side-by-side scene in a maximised window and
/// a stacked one in a 320px secondary pane. Only layout knows which, so the
/// decision is made in [`place_children`](Widget::place_children) — where the
/// resolved width is finally known — and published to a `Signal<usize>` that a
/// [`Switcher`] consumes as its page index.
///
/// **Writing a signal from inside layout** is the
/// [`MenuBar`](teksilo::widgets::MenuBar)-collapse idiom, and
/// it is safe for the same two reasons: the write is guarded by a plain `Cell`
/// shadow so it only happens when the answer actually changes (no churn, no
/// oscillation), and the consumer is a `bind_to`/`visible_when` binding, which the
/// framework *defers* to the next pass rather than running re-entrantly. Nothing
/// downstream of `mode` may use `ctx.effect`: observers fire synchronously inside
/// `Signal::set`, which would re-enter layout from the middle of a layout pass.
///
/// The two branches are `Switcher` pages, so each is built at most once and
/// thereafter only shown or hidden — crossing the breakpoint back and forth keeps
/// the editors' carets, scroll offsets and spell sessions intact instead of
/// rebuilding a scene's worth of widgets on a window drag.
pub(crate) struct WidthProbe {
    /// Whether Side is wanted at all (the placement setting). Derived signals are
    /// fine here: `bind_to` walks a derived signal's mutable roots, and only
    /// `observe()` rejects them.
    side_enabled: Signal<bool>,
    /// Current width of the synopsis pane, so the breakpoint tracks the divider.
    side_width: Signal<f32>,
    mode: Signal<usize>,
    /// Non-reactive mirror of the last value written to `mode` — the guard that
    /// makes the layout-time write idempotent.
    last_mode: Cell<usize>,
    switcher_id: Option<WidgetId>,
    pending: Option<(Box<dyn Widget>, Box<dyn Widget>)>,
}

impl WidthProbe {
    pub fn new(
        side_enabled: Signal<bool>,
        side_width: Signal<f32>,
        top: Box<dyn Widget>,
        side: Box<dyn Widget>,
    ) -> Self {
        Self {
            side_enabled,
            side_width,
            // Start on Top: the available width is unknown until the first
            // layout pass, and Top is the layout that fits every width. A tab
            // that should be Side flips on that first pass, before paint.
            mode: Signal::new(MODE_TOP),
            last_mode: Cell::new(MODE_TOP),
            switcher_id: None,
            pending: Some((top, side)),
        }
    }

    /// The live page index (`0` = Top, `1` = Side). Read it before handing the
    /// probe to the tree.
    // Read only by `width_probe_tests`, which asserts the settle/hysteresis
    // behaviour directly rather than through a laid-out tab.
    #[cfg(test)]
    pub fn mode_signal(&self) -> Signal<usize> {
        self.mode.clone()
    }

    /// The breakpoint decision, factored out so it can be tested without a tree.
    pub(super) fn resolve_mode(&self, available: f32) -> usize {
        if !self.side_enabled.get() {
            return MODE_TOP;
        }
        let threshold = self.side_width.get() + PROSE_MIN_WIDTH;
        // Asymmetric crossing points: harder to enter Side than to stay in it.
        let limit = if self.last_mode.get() == MODE_SIDE {
            threshold - SIDE_BREAKPOINT_HYSTERESIS
        } else {
            threshold + SIDE_BREAKPOINT_HYSTERESIS
        };
        if available >= limit {
            MODE_SIDE
        } else {
            MODE_TOP
        }
    }
}

impl std::fmt::Debug for WidthProbe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WidthProbe")
            .field("mode", &self.mode.get())
            .finish()
    }
}

impl Widget for WidthProbe {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        let self_id = ctx.self_id();
        // A placement or width change must re-run `place_children` so the
        // breakpoint is re-evaluated. `Relayout` (not `Rebuild`) — the widget
        // tree is unchanged, only the decision it feeds.
        self.side_enabled
            .bind_to(self_id, ctx.binding_registry(), BindingLevel::Relayout);
        self.side_width
            .bind_to(self_id, ctx.binding_registry(), BindingLevel::Relayout);

        if let Some((top, side)) = self.pending.take() {
            let switcher = Switcher::new(self.mode.clone())
                .child_boxed(top)
                .child_boxed(side);
            self.switcher_id = Some(ctx.add(switcher));
        }
        self.switcher_id.into_iter().collect()
    }

    fn layout_response(&self, proposal: SizeProposal, ctx: &LayoutContext) -> LayoutResponse {
        self.switcher_id
            .and_then(|id| ctx.child_size(id, proposal))
            .unwrap_or_else(|| proposal.resolve(0.0, 0.0))
            .into()
    }

    fn place_children(
        &self,
        bounds: Rect,
        _proposal: SizeProposal,
        children: &mut [WidgetPlacement],
        _ctx: &LayoutContext,
    ) {
        let next = self.resolve_mode(bounds.width);
        if self.last_mode.get() != next {
            self.last_mode.set(next);
            self.mode.set(next);
        }
        for child in children.iter_mut() {
            child.origin = Point::new(bounds.x, bounds.y);
            child.size = bounds.size();
        }
    }

    fn children(&self) -> Vec<WidgetId> {
        self.switcher_id.into_iter().collect()
    }
}

/// The synopsis as the **left column** of a Side-placed dual-pane editor: no box
/// of its own, filling its splitter pane and scrolling inside it.
///
/// A thin call onto [`synopsis_editor`] with [`SynopsisFit::Side`] — the wiring
/// (context menu, handle re-attach, live typography, spell, replace-while-typing,
/// caret band, format registration) is identical to the other two placements and
/// must stay that way; only the sizing and the chrome differ, which is exactly
/// what `SynopsisFit` decides.
///
/// No `split` action and no typewriter: splitting a scene is an operation on the
/// manuscript, and pinning a line means nothing in a box that scrolls itself.
#[allow(clippy::too_many_arguments)]
pub fn side_synopsis_editor(
    doc: &TextDocument,
    typo: &EditorTypography,
    on_change: impl Fn() + 'static,
    spell: Option<Rc<SpellSession>>,
    replacement: Option<Rc<TextReplacementSession>>,
    handle_sink: Option<Rc<RefCell<Option<EditorHandle>>>>,
    format: Option<FormatViewModel>,
    caret: Option<crate::shared::CaretBand>,
    // The writing games this project is playing (currently "Always forward"),
    // which may freeze this surface while one is on. `None` on the surfaces
    // built with no app around them (the widget tests). Which surfaces a game
    // covers is the game's own decision, taken against this editor's kind.
    games: Option<crate::writing_session::WritingGamesViewModel>,
    // Threaded like every other synopsis placement. Left at `None` this column
    // would be the one surface in the app where a synopsis quietly cannot be
    // commented on — and it is a *placement* of the same `Content` row, not a
    // different thing, so the annotations must follow it across the fold.
    comments: Option<crate::comments::binding::CommentBinding>,
    // Where this editor fetches an image it meets but its document does not
    // have — a picture pasted in from another editor, or brought back by an
    // undo. `None` on the surfaces built without a project around them.
    images: Option<crate::shared::images::ImageSource>,
    // Whether this surface may be typed into — see `writing_column`.
    read_only: bool,
    // Forwarded straight to [`synopsis_editor`] — see its own note.
    item: Option<common::types::EntityId>,
    // Forwarded straight to [`synopsis_editor`] — see its own note.
    estimate_height: bool,
) -> impl Widget {
    synopsis_editor(
        doc,
        typo,
        SynopsisFit::Side,
        on_change,
        None,
        spell,
        replacement,
        handle_sink,
        format,
        None,
        caret,
        games,
        comments,
        images,
        read_only,
        item,
        estimate_height,
    )
}

/// Index of the synopsis pane in a tab's Side splitter.
pub(crate) const SYNOPSIS_PANE: usize = 0;

/// Publishes one scroll area's offset/max to the tab's view-state ports — but only
/// while the branch it sits in is the one on screen.
///
/// A tab's remembered caret and page scroll live in a single slot, last write
/// wins. That was fine while a prose tab had exactly one scrolling page; the Top
/// and Side layouts each have their own, so whichever was *constructed* last would
/// otherwise own the slot regardless of which is *displayed* — and the tab would
/// restore, and report, the scroll of a page nobody is looking at.
///
/// Re-attaching on activation makes the answer "the visible one" by construction,
/// including on the way back to a branch that was built earlier and will not build
/// again (a `Switcher` keeps its pages, so a second `build()` never comes).
pub(crate) struct PageScrollPort {
    ports: Rc<crate::shared::ViewStatePorts>,
    offset: Signal<f32>,
    max: Signal<f32>,
    /// This page's main editor handle, staged by the column that built it. Promoted
    /// into `ports` beside the scroll, and gated the same way, so the caret the tab
    /// captures and the editor a click focuses both belong to the page on screen.
    page_editor: Rc<RefCell<Option<EditorHandle>>>,
}

impl PageScrollPort {
    pub fn new(
        ports: Rc<crate::shared::ViewStatePorts>,
        offset: Signal<f32>,
        max: Signal<f32>,
        page_editor: Rc<RefCell<Option<EditorHandle>>>,
    ) -> Self {
        Self {
            ports,
            offset,
            max,
            page_editor,
        }
    }
}

impl std::fmt::Debug for PageScrollPort {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PageScrollPort").finish()
    }
}

impl Widget for PageScrollPort {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        let active = ctx.activation_signal(ctx.self_id());
        let attach: Rc<dyn Fn()> = {
            let ports = self.ports.clone();
            let offset = self.offset.clone();
            let max = self.max.clone();
            let page_editor = self.page_editor.clone();
            let active = active.clone();
            Rc::new(move || {
                if active.get() {
                    ports.attach_page_scroll(offset.clone(), max.clone());
                    if let Some(handle) = page_editor.borrow().clone() {
                        ports.attach_editor(handle);
                    }
                }
            })
        };
        attach();
        let f = attach.clone();
        ctx.effect(&active, move |_| f());
        // The after-mount half of a restore, enqueued from here because this is the
        // one widget every writing page mounts and the only one on the page with a
        // `BuildContext`. It needs a laid-out editor (a caret with no geometry cannot
        // be revealed) and `run_after_mount` is drained after the redraw pass that
        // lays the tree out. A no-op unless something armed one of the one-shots,
        // which a tab the writer is simply typing in never does.
        //
        // Guarded on activation for the same reason the attach above is: a page that
        // is not the one on screen must not act on a request meant for the page that
        // is, and `ports.editor()` would answer with whichever handle was attached
        // last. Today only the mounted page builds, so this is a belt on top of
        // braces rather than a live bug being fixed.
        if active.get() {
            self.ports.finish_restore_after_mount(ctx);
        }
        Vec::new()
    }

    fn layout_response(&self, _proposal: SizeProposal, _ctx: &LayoutContext) -> LayoutResponse {
        Size::new(0.0, 0.0).into()
    }

    fn place_children(
        &self,
        _bounds: Rect,
        _proposal: SizeProposal,
        _children: &mut [WidgetPlacement],
        _ctx: &LayoutContext,
    ) {
    }

    fn children(&self) -> Vec<WidgetId> {
        Vec::new()
    }
}

/// Drives a tab's Side divider from the same boolean that decides whether the
/// synopsis is on screen at all.
///
/// Showing and hiding a splitter pane is not just a visibility flip: a `Splitter`
/// folds **every** pane's `min_size` into its own intrinsic minimum, visible or
/// not, so a hidden pane left holding a real minimum keeps setting a floor under
/// the whole tab's width. The minimum is therefore raised only while the pane is
/// up and dropped to zero on the way down — the same dance
/// `EditorsViewModel::set_split` performs for the editor's own side pane, and in
/// the same order (minimum first, both ways).
#[derive(Clone)]
pub(crate) struct SideSync {
    model: SplitterModel,
    /// Where a user-dragged width is persisted, and what the next tab seeds from.
    width: Signal<f32>,
    /// Set while *this* code is mutating the model, so the width write-back can
    /// tell the app's own bookkeeping apart from a real drag. See
    /// [`SideSync::watch_width`].
    suppress: Rc<Cell<bool>>,
}

impl SideSync {
    pub fn new(model: SplitterModel, width: Signal<f32>) -> Self {
        Self {
            model,
            width,
            suppress: Rc::new(Cell::new(false)),
        }
    }

    /// Whether the *setting* puts a synopsis column in this tab at all.
    ///
    /// Distinct from folding it away, and both states are used: hidden removes the
    /// pane **and its gutter**, which is right when there is no synopsis to reach;
    /// folded keeps the gutter, which is what lets the writer pull it back.
    pub(super) fn set_visible(&self, visible: bool) {
        // Idempotent, and that is load-bearing rather than an optimisation: the
        // dormancy effect re-runs on the model's own `version`, so a `set_visible`
        // that mutated unconditionally would bump the version it is reacting to and
        // spin. Re-entrancy here is a hang, not a wasted call.
        if self.model.is_pane_visible(SYNOPSIS_PANE) == visible {
            return;
        }
        self.suppress.set(true);
        if visible {
            self.model
                .set_min_size(SYNOPSIS_PANE, crate::SYNOPSIS_SIDE_WIDTH_MIN);
            self.model.set_pane_visible(SYNOPSIS_PANE, true);
        } else {
            self.model.set_min_size(SYNOPSIS_PANE, 0.0);
            self.model.set_pane_visible(SYNOPSIS_PANE, false);
        }
        self.suppress.set(false);
    }

    /// Fold the column away, leaving the divider behind as the way back.
    pub fn fold(&self) {
        self.model.set_collapsed(SYNOPSIS_PANE, true);
    }

    pub(super) fn is_folded(&self) -> bool {
        self.model.is_collapsed(SYNOPSIS_PANE)
    }

    /// Whether this handle is part-way through its own mutation of the model.
    ///
    /// The model bumps `version` from *inside* `set_pane_visible`, before the flag
    /// it is setting has landed — so a version observer that re-entered here would
    /// read the old value, mutate again, and recurse until the framework's
    /// nesting limit killed it. Reading the flag is not enough; the observer has
    /// to know the write is still in flight.
    pub(super) fn is_settling(&self) -> bool {
        self.suppress.get()
    }

    pub(super) fn version(&self) -> Signal<u64> {
        self.model.version()
    }

    /// Persist the synopsis column's width when the **writer** drags the divider.
    ///
    /// `SplitterModel::version()` is one coarse signal bumped by every mutation
    /// there is, so it cannot be taken at face value: this widget's own show/hide
    /// dance bumps it twice on every toggle, and a drag past the minimum bumps it
    /// while collapsing the pane to nothing. Persisting either would quietly
    /// rewrite the writer's chosen width with a number they never chose — a zero,
    /// in the collapse case.
    ///
    /// Three filters, in order of what they exclude: the suppression flag (our own
    /// mutations, set synchronously around them because observers run inside
    /// `Signal::set`), the pane's own state (a hidden or collapsed pane's width is
    /// not a width anyone picked), and a last-written shadow (so an unrelated bump
    /// is not a write).
    pub(super) fn watch_width(&self, ctx: &mut BuildContext) {
        let model = self.model.clone();
        let target = self.width.clone();
        let suppress = self.suppress.clone();
        let last = Rc::new(Cell::new(model.stored_size(SYNOPSIS_PANE)));
        ctx.effect(&model.version(), move |_| {
            if suppress.get() {
                return;
            }
            if !model.is_pane_visible(SYNOPSIS_PANE) || model.is_collapsed(SYNOPSIS_PANE) {
                return;
            }
            let width = model.stored_size(SYNOPSIS_PANE).clamp(
                crate::SYNOPSIS_SIDE_WIDTH_MIN,
                crate::SYNOPSIS_SIDE_WIDTH_MAX,
            );
            if (width - last.get()).abs() > 0.5 {
                last.set(width);
                target.set(width);
            }
        });
    }
}

/// Keeps a tab's synopsis **plumbing** in step with whether the synopsis is
/// actually on screen: the spell session's dormancy, and (under Side placement)
/// the divider.
///
/// A widget rather than a call in `prose()` because both jobs need a
/// `BuildContext` to register effects on, and `prose()` is a plain builder
/// function. It draws nothing and occupies nothing — it is mounted purely so that
/// its lifetime, and the framework's own activation gate, can be borrowed.
///
/// Used by **both** placements, so "is the synopsis being shown?" has exactly one
/// answer in the codebase rather than one per layout. Under Top the divider half
/// is simply absent.
pub(crate) struct SynopsisPaneEffects {
    doc: Rc<crate::models::OpenDoc>,
    /// The window's or the mode's "show synopsis" flag.
    show: Signal<bool>,
    side: Option<SideSync>,
    guard: Rc<RefCell<Option<crate::models::SynopsisViewerGuard>>>,
}

impl SynopsisPaneEffects {
    pub fn new(
        doc: Rc<crate::models::OpenDoc>,
        show: Signal<bool>,
        side: Option<SideSync>,
    ) -> Self {
        Self {
            doc,
            show,
            side,
            guard: Rc::new(RefCell::new(None)),
        }
    }
}

impl std::fmt::Debug for SynopsisPaneEffects {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SynopsisPaneEffects")
            .field("item", &self.doc.item_id)
            .finish()
    }
}

impl Widget for SynopsisPaneEffects {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        let self_id = ctx.self_id();
        // Activation, not just the two flags. A `TabWidget` pre-mounts every open
        // tab and a `Switcher` keeps the branch it switched away from alive, so
        // effects keep firing for panes nobody can see — and a synopsis session
        // pinned awake by a background tab is exactly the cost this refcount
        // exists to avoid. Same gate `wire_spell`/`wire_replacements` already use.
        let active = ctx.activation_signal(self_id);

        let apply: Rc<dyn Fn()> = {
            let doc = self.doc.clone();
            let guard = self.guard.clone();
            let side = self.side.clone();
            let show = self.show.clone();
            let active = active.clone();
            Rc::new(move || {
                let shown = active.get() && show.get();
                if let Some(side) = &side {
                    side.set_visible(shown);
                }
                // A folded column is on screen in name only — the framework parks
                // its content dormant behind the divider — so it must not hold the
                // spell session awake either. Asked of the splitter rather than
                // mirrored into a second flag: the divider can also be dragged
                // shut, and a mirror would go stale the moment the writer did that.
                let reachable = shown && !side.as_ref().is_some_and(SideSync::is_folded);
                let mut slot = guard.borrow_mut();
                match (reachable, slot.is_some()) {
                    // Dropping the guard is what puts the session to sleep, and
                    // it only reaches a `Cell` on the doc — no re-entry into the
                    // borrow held here.
                    (true, false) => *slot = Some(doc.acquire_synopsis_viewer()),
                    (false, true) => *slot = None,
                    _ => {}
                }
            })
        };

        // Seed: `ctx.effect` fires on later changes only, and the pane may well be
        // built with the synopsis already showing.
        apply();
        if let Some(side) = &self.side {
            side.watch_width(ctx);
            // Folding and unfolding are splitter mutations, so this is how a fold
            // reaches dormancy — including one done by dragging the divider shut
            // or double-clicking it, not just by pressing the button.
            let f = apply.clone();
            let guard = side.clone();
            ctx.effect(&side.version(), move |_| {
                if !guard.is_settling() {
                    f();
                }
            });
        }
        for src in [&self.show, &active] {
            let f = apply.clone();
            ctx.effect(src, move |_| f());
        }
        Vec::new()
    }

    fn layout_response(&self, _proposal: SizeProposal, _ctx: &LayoutContext) -> LayoutResponse {
        Size::new(0.0, 0.0).into()
    }

    fn place_children(
        &self,
        _bounds: Rect,
        _proposal: SizeProposal,
        _children: &mut [WidgetPlacement],
        _ctx: &LayoutContext,
    ) {
    }

    fn children(&self) -> Vec<WidgetId> {
        Vec::new()
    }
}
