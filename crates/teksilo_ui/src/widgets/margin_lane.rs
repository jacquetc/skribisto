// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! MarginLane — a thin strip beside a scroll area that **maps** a document
//! rather than picturing it.
//!
//! A minimap shrinks the content. That works for code, which has a silhouette
//! at two pixels per line, and fails for prose, which is a uniform grey slab at
//! any scale. A margin lane draws *marks* instead: a search hit, a comment, the
//! boundary between two documents — each at the fraction of the content it sits
//! at, so the strip says **where** things are without pretending to say what
//! they look like.
//!
//! The widget knows nothing about any of that. It is handed [`LaneMark`]s
//! carrying a span, a column, a shape and a colour, and it draws, hit-tests and
//! speaks them. No document, no offsets, no comments, no search — the host does
//! every conversion. That is what makes it reusable, and it is why
//! [`LaneSpan`] is a *fraction* rather than an offset.
//!
//! ## The fraction is of laid-out pixels, not of characters
//!
//! Worth stating because the obvious reading is wrong and the failure is
//! invisible. `char_offset / char_count` is **not** proportional to vertical
//! position once a document has headings, images, blank lines and paragraphs of
//! unequal length — which is every real document. A lane fed character
//! fractions puts its marks in visibly wrong places, worst exactly where a
//! reader checks. The host must map through real laid-out geometry (for a text
//! editor, its position-to-rect query) and hand down the resulting pixel
//! fraction.
//!
//! ## Columns
//!
//! The mark area is divided into three columns, the way VS Code divides its
//! overview ruler. Two providers assigned different columns can never fight for
//! the same pixel, and [`LaneColumn::Full`] spans all three for the one thing
//! the user explicitly asked for.
//!
//! ## Texture
//!
//! An optional wider column draws one bar per unit of content — for prose, one
//! per paragraph, its length the word count and its filled portion some share of
//! it. It is a *column of this widget*, not a second widget: one border, one
//! hover target, one accessible region, and turning it off collapses the lane to
//! its mark columns with nothing else to change.
//!
//! ```rust
//! # use teksilo_ui::widgets::margin_lane::{MarginLane, LaneMark, LaneSpan, LaneColumn, LaneShape};
//! # use teksilo::core::signal::Signal;
//! # use teksilo::i18n::LocalizedString;
//! # use teksilo::tokens::Color;
//! let viewport_top = Signal::new(0.0_f32);
//! let viewport_span = Signal::new(0.25_f32);
//!
//! let marks = vec![LaneMark {
//!     id: 1,
//!     span: LaneSpan::at(0.42),
//!     column: LaneColumn::Full,
//!     shape: LaneShape::Bar,
//!     color: Color::from_hex("#BD8300"),
//!     label: LocalizedString::literal("search hit"),
//!     group: 0,
//! }];
//!
//! let _lane = MarginLane::new(marks, viewport_top, viewport_span).width(12.0);
//! ```

use std::cell::Cell;
use std::rc::Rc;

use teksilo::canvas::{Canvas, Point, Rect, Size, SizeProposal};
use teksilo::core::accessibility::{AccessNodeBuilder, SyntheticKind};
use teksilo::core::accesskit;
use teksilo::core::binding::BindingLevel;
use teksilo::core::event::{EventResponse, PointerButton};
use teksilo::core::gesture::DragPhase;
use teksilo::core::signal::{Prop, Signal};
use teksilo::core::widget::{EventContext, LayoutContext, LayoutResponse, PaintContext, Widget};
use teksilo::core::widget_builder::HandlerSet;
use teksilo::core::widget_id::WidgetId;
use teksilo::i18n::LocalizedString;
use teksilo::tokens::Color;

/// Where the caret rule is drawn, as an offset down the strip.
///
/// `viewport` is the box as it will actually be painted -- already floored -- and
/// `from`/`span` are what it truly represents.
///
/// A caret **inside** the viewport is placed inside the box at the same relative
/// position it holds in the span. Everywhere else it is placed straight on the map.
/// Where the box was not floored the two are the same arithmetic, so this only ever
/// changes the case the floor created.
fn caret_offset(
    at: f32,
    lane_height: f32,
    viewport: Option<(f32, f32)>,
    from: f32,
    span: f32,
) -> f32 {
    let at = at.clamp(0.0, 1.0);
    let plain = at * lane_height;
    let y = match viewport {
        Some((top, height)) if span > 0.0 => {
            let local = (at - from) / span;
            if (0.0..=1.0).contains(&local) {
                top + local * height
            } else {
                plain
            }
        }
        _ => plain,
    };
    // Kept inside the strip: the floor can push the box's foot past the bottom, and
    // a caret rule painted off the end is a rule nobody sees.
    y.clamp(0.0, (lane_height - CARET_HEIGHT).max(0.0))
}

/// The shortest the viewport box is ever drawn.
///
/// Over a whole Book the viewport's true span is about one pixel of the strip, and
/// a one-pixel box is not something a reader can find. The caret rule is drawn in
/// the floored box's own space so the two never disagree about where the writer is.
const MIN_VIEWPORT_HEIGHT: f32 = 6.0;

/// The caret rule's thickness.
const CARET_HEIGHT: f32 = 2.0;

/// Default lane width, matching a scroll bar's own default thickness.
///
/// Not an arbitrary number: VS Code sizes its overview ruler to exactly the
/// vertical scroll bar's width, and taking the same rule means the lane stays
/// the width of the bar it sits beside if that default ever moves.
pub const DEFAULT_LANE_WIDTH: f32 = 12.0;

/// Default width of the texture column when one is shown.
pub const DEFAULT_TEXTURE_WIDTH: f32 = 28.0;

/// Smallest height a mark may be drawn at, in logical pixels.
///
/// Below roughly this the mark stops being visible at all. VS Code enforces the
/// same floor for the same reason (six device pixels, re-centred on the mark's
/// true position rather than grown downward from it, so the mark does not drift
/// off what it points at).
pub const MIN_MARK_HEIGHT: f32 = 3.0;

/// How long a [`LaneShape::BarCurrent`] is drawn, at least.
///
/// Three times an ordinary mark's floor: enough to find at a glance among a hundred
/// siblings, without reaching for the width, which belongs to the column.
pub const CURRENT_MARK_HEIGHT: f32 = 9.0;

/// Where a mark sits along the lane, as a fraction of the mapped content.
///
/// `0.0` is the top of the content and `1.0` the bottom. `end == start` is a
/// point rather than a span, which is the common case.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LaneSpan {
    pub start: f32,
    pub end: f32,
}

impl LaneSpan {
    /// A point mark.
    pub fn at(fraction: f32) -> Self {
        Self {
            start: fraction,
            end: fraction,
        }
    }

    /// A span mark, normalised so `start <= end`.
    pub fn new(start: f32, end: f32) -> Self {
        if start <= end {
            Self { start, end }
        } else {
            Self {
                start: end,
                end: start,
            }
        }
    }

    /// Clamped to `0.0..=1.0`, which is what the lane actually draws.
    fn clamped(self) -> Self {
        Self {
            start: self.start.clamp(0.0, 1.0),
            end: self.end.clamp(0.0, 1.0),
        }
    }
}

/// Which of the three mark columns a mark belongs to.
///
/// Assigning providers to different columns is what keeps them from colliding.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LaneColumn {
    Left,
    Center,
    Right,
    /// All three — for the thing the user is actively looking for.
    Full,
}

/// A mark's shape class.
///
/// **Shape carries the distinction; colour is the third signal, never the
/// first.** A reader who cannot separate two hues still separates a bar from a
/// diamond, and a 3 px mark is small enough that hue alone is a weak channel
/// even with normal vision. The filled/hollow pairs mean one specific thing
/// throughout: filled is what a person put there, hollow is what the system
/// inferred.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LaneShape {
    /// Spanning the column's full width — something actively searched for.
    Bar,
    /// **The one of many that the reader is standing on**, and the only shape whose
    /// purpose is to be told apart from its own siblings.
    ///
    /// A [`Bar`](Self::Bar) in every respect but length: same column, same width,
    /// drawn longer and floored so a single-line hit is still findable among a
    /// hundred of them.
    ///
    /// Emphasis deliberately does **not** use width, and that is the whole reason
    /// this exists. A caller that widened its current mark instead -- by putting it
    /// in [`LaneColumn::Full`] while its siblings sat in a third
    /// of the area -- made the occupied width of the column a function of *how many
    /// matches there were and where the current one was*. Typing a query then looked
    /// like the column growing and shrinking, which is a thing a reader notices long
    /// before they notice which mark is current.
    ///
    /// Nor colour: the host decides that, so that no provider can ship a mark which
    /// fails contrast on a theme it never saw.
    BarCurrent,
    /// A note attached to a place.
    Square,
    /// The same, in a resolved or inactive state.
    SquareHollow,
    /// A structural position.
    Diamond,
    /// A structural position the system inferred rather than one that was set.
    DiamondHollow,
    /// A low-emphasis finding.
    Dot,
    /// A **division** rather than a finding — a hairline with a notch at the
    /// leading edge. A different shape class because it means a different kind
    /// of thing.
    Rule,
}

/// One mark on the lane.
#[derive(Clone)]
pub struct LaneMark {
    /// Stable within one provider, for accessibility node identity across
    /// repaints. Marks whose id changes every frame produce a tree that a
    /// screen reader cannot hold still.
    pub id: u64,
    pub span: LaneSpan,
    pub column: LaneColumn,
    pub shape: LaneShape,
    pub color: Color,
    /// What assistive technology says for this mark.
    ///
    /// A [`LocalizedString`] rather than a plain `String`, matching how every
    /// other widget here takes a label: it re-resolves when the locale changes,
    /// which a `String` captured at registration time would not.
    pub label: LocalizedString,
    /// Provider ordinal. Decides paint order (ascending, so a later group draws
    /// over an earlier one) and which marks may merge with which. Declared
    /// rather than incidental, so two providers cannot silently reorder
    /// depending on registration order.
    pub group: u16,
}

impl std::fmt::Debug for LaneMark {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LaneMark")
            .field("id", &self.id)
            .field("span", &self.span)
            .field("column", &self.column)
            .field("shape", &self.shape)
            .field("group", &self.group)
            .finish_non_exhaustive()
    }
}

/// One bar of the texture column.
///
/// Business-agnostic on purpose: a length and a fill. For prose `extent` is the
/// unit's word count against the longest in view and `filled` the share of it
/// that is dialogue, but the widget neither knows nor cares.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LaneBar {
    pub span: LaneSpan,
    /// Bar length as a fraction of the texture column's usable width, `0.0..=1.0`.
    pub extent: f32,
    /// Filled portion of that length, `0.0..=1.0`.
    pub filled: f32,
}

/// A mark resolved to lane pixels — what the lane actually drew, and what its
/// hit-testing and accessibility both read.
///
/// Exposed because synthetic accessibility nodes are not enumerable from the
/// public test API, so a test asserts against this instead. That is the same
/// thing `teksilo-charts` does for its per-datum marks, and for the same reason.
#[derive(Clone)]
pub struct ResolvedMark {
    pub id: u64,
    /// The provider ordinal this mark came from. Part of its accessibility node
    /// identity: two providers numbering their marks from 1 must not collide.
    pub group: u16,
    pub top: f32,
    pub height: f32,
    pub column: LaneColumn,
    pub shape: LaneShape,
    pub color: Color,
    pub label: LocalizedString,
    /// How many marks this one stands for after merging. `1` for a mark that
    /// merged with nothing.
    pub merged: usize,
}

impl std::fmt::Debug for ResolvedMark {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ResolvedMark")
            .field("id", &self.id)
            .field("group", &self.group)
            .field("top", &self.top)
            .field("height", &self.height)
            .field("column", &self.column)
            .field("shape", &self.shape)
            .field("merged", &self.merged)
            .finish_non_exhaustive()
    }
}

/// Where the viewport box sits, and what a pointer at some height means for it.
///
/// Its own type because three things need these answers and must not derive them
/// separately: the paint, the hit test that decides whether a press grabs the box,
/// and the arithmetic that moves it. A hit rect computed apart from the paint is a
/// handle a writer can see and cannot grab, in exactly the places where the two
/// computations round differently.
///
/// Holds the signals rather than their values: the host rewrites them on every
/// scroll, and a handler must answer from where the box is *now*, not from where
/// it was when the lane was built.
#[derive(Clone)]
struct LaneGeometry {
    viewport_top: Signal<f32>,
    viewport_span: Signal<f32>,
}

impl LaneGeometry {
    fn viewport_top(&self) -> f32 {
        self.viewport_top.get().clamp(0.0, 1.0)
    }

    fn span(&self) -> f32 {
        self.viewport_span.get().clamp(0.0, 1.0)
    }

    /// `(top, height)` in lane-local pixels, or `None` when there is no box.
    ///
    /// `None` when the viewport covers the whole map, or nothing is mapped yet: a
    /// box around the entire strip says nothing worth the ink, and there is
    /// nothing to drag it along.
    ///
    /// The height is **floored** to [`MIN_VIEWPORT_HEIGHT`]. Over a whole Book the
    /// true span really is about a pixel, and a one-pixel box is neither a "you are
    /// here" nor anything a pointer can hit.
    fn drawn(&self, lane_height: f32) -> Option<(f32, f32)> {
        let span = self.span();
        if !(span > 0.0 && span < 1.0) || lane_height <= 0.0 {
            return None;
        }
        Some((
            self.viewport_top() * lane_height,
            (span * lane_height).max(MIN_VIEWPORT_HEIGHT),
        ))
    }

    /// Whether a lane-local `y` lands on the box as drawn.
    fn box_contains(&self, lane_height: f32, y: f32) -> bool {
        self.drawn(lane_height)
            .is_some_and(|(top, height)| y >= top && y <= top + height)
    }

    /// How far down the box its own centre is, as a fraction of the map.
    ///
    /// Measured on the box as **drawn**, because that is the box the writer is
    /// aiming at. Where the floor is not in play the two are the same number.
    fn centre_grab(&self, lane_height: f32) -> f32 {
        match self.drawn(lane_height) {
            Some((_, height)) if lane_height > 0.0 => height / 2.0 / lane_height,
            _ => 0.0,
        }
    }

    /// The viewport top a pointer at `y` asks for, holding the box `grab`
    /// fractions below its own top.
    ///
    /// Clamped against the **true** span, not the drawn one: flooring the box is a
    /// visibility affordance, and letting it decide how far a writer may scroll
    /// would put the end of a long book out of reach by however much the floor
    /// added.
    fn top_for(&self, y: f32, lane_height: f32, grab: f32) -> f32 {
        let pointed = (y / lane_height).clamp(0.0, 1.0);
        (pointed - grab).clamp(0.0, (1.0 - self.span()).max(0.0))
    }
}

/// A thin strip beside a scroll area, mapping marks onto the content's extent.
pub struct MarginLane {
    marks: Prop<Vec<LaneMark>>,
    bars: Prop<Vec<LaneBar>>,
    /// Where the viewport's top sits **on the map**, `0.0..=1.0`.
    viewport_top: Signal<f32>,
    /// How much of the map the viewport covers, `0.0..=1.0`. `1.0` means all of
    /// it, and the box is then not drawn at all.
    viewport_span: Signal<f32>,
    caret: Prop<Option<f32>>,
    width: Prop<f32>,
    texture_width: Prop<f32>,
    /// Whether [`MarginLane::texture_width`] was called. See [`MarginLane::texture`].
    texture_width_set: bool,
    on_jump: Option<Rc<dyn Fn(f32)>>,
    /// What the strip offers on a right-click, if anything.
    ///
    /// Held here rather than wrapped around the lane from outside, and the
    /// reason is the accessibility tree: a node advertises `ShowContextMenu`
    /// only where it owns the factory ITSELF, and the node a screen reader
    /// lands on is the one this widget emits. A factory on a wrapper works for
    /// a pointer and is invisible to everyone else -- which is the same shape
    /// of gap as a control that only exists on hover.
    #[allow(clippy::type_complexity)]
    context_menu: Option<Rc<dyn Fn(Point, &mut EventContext) -> Option<Box<dyn Widget>>>>,
    access_label: Option<LocalizedString>,
    /// Bounds as of the last **layout**, not the last paint.
    ///
    /// Captured in `place_children` because the accessibility walk can run
    /// without a paint having happened first — on the very first tree sync, or
    /// on a window that is laid out but occluded. Reading paint-time bounds
    /// there meant the AT tree was built against `Rect::ZERO`, which collapses
    /// every mark to the same position and merges them all into one.
    cached_bounds: Rc<Cell<Rect>>,
    /// Which mark the keyboard is on, as an index into `resolve_marks`.
    selected: Signal<Option<usize>>,
    /// Whether the viewport box is being dragged, the way a scroll bar's thumb is.
    dragging: Signal<bool>,
    /// Where in the box the writer took hold of it, as a fraction of the map.
    ///
    /// The whole of what makes a drag a *handle* drag rather than a repeated jump:
    /// the box keeps the offset it was grabbed at, so it travels with the pointer
    /// instead of re-centring under it on every move. Grabbing the bottom edge and
    /// pulling down must not first snap the box up by half its height.
    grab: Rc<Cell<f32>>,
}

impl std::fmt::Debug for MarginLane {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MarginLane").finish_non_exhaustive()
    }
}

impl MarginLane {
    /// A lane over `marks`, told where the viewport sits **on the map**.
    ///
    /// Both signals are fractions of the lane, not pixels, and that is the whole
    /// contract: *the map is `0.0..=1.0`; say where the viewport is on it*.
    ///
    /// It used to take a `ScrollArea`'s own `scroll`, `max_scroll` and
    /// `viewport_ratio` and do the division here, which was less work for a
    /// caller and quietly assumed the one thing a lane may not: that a fraction
    /// of the scrolled pixels is a fraction of the map. That holds only where the
    /// content is laid out uniformly. It is false for a manuscript, whose scenes
    /// are mapped by how much of the book they are rather than by how tall they
    /// rendered, and it was false in a way nobody could see -- the box simply
    /// pointed a little to the side of the marks it was meant to be over.
    ///
    /// A caller with a plain uniform scroll area passes
    /// `scroll / content` and `viewport / content`, which is what this used to
    /// compute for it.
    pub fn new(
        marks: impl Into<Prop<Vec<LaneMark>>>,
        viewport_top: Signal<f32>,
        viewport_span: Signal<f32>,
    ) -> Self {
        Self {
            marks: marks.into(),
            bars: Prop::from(Vec::new()),
            viewport_top,
            viewport_span,
            caret: Prop::from(None),
            width: Prop::from(DEFAULT_LANE_WIDTH),
            texture_width: Prop::from(0.0),
            texture_width_set: false,
            on_jump: None,
            context_menu: None,
            access_label: None,
            cached_bounds: Rc::new(Cell::new(Rect::ZERO)),
            selected: Signal::new(None),
            dragging: Signal::new(false),
            grab: Rc::new(Cell::new(0.0)),
        }
    }

    /// Width of the mark area. Defaults to [`DEFAULT_LANE_WIDTH`].
    pub fn width(mut self, dp: impl Into<Prop<f32>>) -> Self {
        self.width = dp.into();
        self
    }

    /// The texture column's bars, **and the switch that turns the column on**.
    ///
    /// Sets [`texture_width`](Self::texture_width) to [`DEFAULT_TEXTURE_WIDTH`]
    /// unless that was called explicitly, in either order. Pass `0.0` there to keep
    /// the column off while still handing over bars.
    pub fn texture(mut self, bars: impl Into<Prop<Vec<LaneBar>>>) -> Self {
        self.bars = bars.into();
        // **Giving the lane bars is what turns the column on.** It used to take two
        // calls, and forgetting the second was silent: the bars were computed, at
        // the cost of walking a document, and drawn into a column zero pixels wide.
        // A host shipped exactly that — a writer could turn the texture on in
        // Settings and nothing would ever appear.
        //
        // The flag rather than a check on the current value, so the two builders are
        // order-independent: an explicit `texture_width` still wins whether it was
        // called before this or after.
        if !self.texture_width_set {
            self.texture_width = Prop::from(DEFAULT_TEXTURE_WIDTH);
        }
        self
    }

    /// Width of the texture column. `0.0` — the default — hides it, collapsing
    /// the lane to its mark columns.
    pub fn texture_width(mut self, dp: impl Into<Prop<f32>>) -> Self {
        self.texture_width = dp.into();
        self.texture_width_set = true;
        self
    }

    /// Where the caret sits, as a fraction of the content. Drawn as a full-width
    /// accent rule so the writer can see where they are relative to everything
    /// else the lane is showing.
    pub fn caret(mut self, at: impl Into<Prop<Option<f32>>>) -> Self {
        self.caret = at.into();
        self
    }

    /// What the strip offers on a right-click.
    ///
    /// The widget knows nothing about what the items mean -- the factory is
    /// called with an `EventContext` and builds whatever the host wants. It
    /// lives on the widget rather than on a wrapper so the menu is announced:
    /// `ShowContextMenu` is advertised on the node that owns the factory, and
    /// that has to be the node carrying this widget's role and name.
    ///
    /// Right-click is free here by construction: the tap recogniser accepts the
    /// primary button only, and the box drag matches Primary too.
    pub fn context_menu(
        mut self,
        factory: impl Fn(Point, &mut EventContext) -> Option<Box<dyn Widget>> + 'static,
    ) -> Self {
        self.context_menu = Some(Rc::new(factory));
        self
    }

    /// Called with the fraction of the map the **viewport's top** should move to.
    ///
    /// Not the fraction the writer pointed at, and the difference is the whole of
    /// what a lane is for. A click puts the box's *middle* where the pointer is,
    /// so the mark someone aimed at ends up on screen rather than on its first
    /// line with everything they were looking for below the fold. A drag of the
    /// box keeps the offset it was grabbed at, the way a scroll bar's thumb does.
    /// What arrives here is the answer to all of that, already clamped so the box
    /// stays on the strip.
    ///
    /// A host maps it back through whatever mapping it laid the marks out with —
    /// which is the reason this is a fraction of the map and not a pixel: the two
    /// are only the same number where the content is laid out uniformly, and a
    /// manuscript is not.
    pub fn on_jump(mut self, f: impl Fn(f32) + 'static) -> Self {
        self.on_jump = Some(Rc::new(f));
        self
    }

    /// Where the viewport box is, in a form the handlers can keep.
    ///
    /// A closure cannot hold `&self`, and the two numbers it needs are signals the
    /// host writes on every scroll, so what it holds is the signals — the answer
    /// is derived at the moment of the click, from what the writer is looking at.
    fn geometry(&self) -> LaneGeometry {
        LaneGeometry {
            viewport_top: self.viewport_top.clone(),
            viewport_span: self.viewport_span.clone(),
        }
    }

    /// Where the viewport box is **drawn**, in lane-local pixels: `(top, height)`.
    fn drawn_viewport(&self, lane_height: f32) -> Option<(f32, f32)> {
        self.geometry().drawn(lane_height)
    }

    /// The lane's own accessible name. Defaults to "Margin marks, N marks".
    pub fn access_label(mut self, label: impl Into<LocalizedString>) -> Self {
        self.access_label = Some(label.into());
        self
    }

    /// Total width the lane occupies, texture column included.
    fn total_width(&self) -> f32 {
        let tex = self.texture_width.get().max(0.0);
        let marks = self.width.get().max(0.0);
        if tex > 0.0 {
            tex + marks + TEXTURE_DIVIDER
        } else {
            marks
        }
    }

    /// The mark area's x range within `bounds`.
    fn mark_area(&self, bounds: Rect) -> (f32, f32) {
        let tex = self.texture_width.get().max(0.0);
        let marks = self.width.get().max(0.0);
        if tex > 0.0 {
            (bounds.x + tex + TEXTURE_DIVIDER, marks)
        } else {
            (bounds.x, marks)
        }
    }

    /// Marks resolved to lane pixels, merged, in paint order.
    ///
    /// The single source of truth for what the lane draws, what a click hits and
    /// what a screen reader is told — deliberately one function, so those three
    /// cannot describe different geometry.
    pub fn resolve_marks(&self, bounds: Rect) -> Vec<ResolvedMark> {
        let mut marks = self.marks.get();
        // Ascending group, then position, so paint order is declared rather
        // than dependent on the order providers happened to register in.
        marks.sort_by(|a, b| {
            a.group.cmp(&b.group).then(
                a.span
                    .start
                    .partial_cmp(&b.span.start)
                    .unwrap_or(std::cmp::Ordering::Equal),
            )
        });

        let mut out: Vec<ResolvedMark> = Vec::with_capacity(marks.len());
        // Index of the last mark emitted for each (group, column, shape), so a
        // merge candidate is compared against the nearest prior mark *of its own
        // kind* rather than against whatever happened to be emitted last. Sorting
        // by (group, start) alone is not enough: a mark from another column
        // sorting between two mergeable ones would otherwise split them.
        let mut last_of: Vec<((u16, LaneColumn, LaneShape), usize)> = Vec::new();

        for m in marks {
            let span = m.span.clamped();
            let raw_top = bounds.y + span.start * bounds.height;
            let raw_height = (span.end - span.start) * bounds.height;
            // Re-centre rather than grow downward: a mark that grew from its top
            // would drift below the thing it points at, and at the floor height
            // that drift is the whole mark. Then clamp, so a mark at 0.0 or 1.0
            // does not hang outside the lane it belongs to.
            let height = raw_height.max(MIN_MARK_HEIGHT);
            let top = (raw_top - (height - raw_height) / 2.0)
                .clamp(bounds.y, (bounds.y + bounds.height - height).max(bounds.y));

            let key = (m.group, m.column, m.shape);
            if let Some(&(_, idx)) = last_of.iter().find(|(k, _)| *k == key) {
                let prev = &mut out[idx];
                if top <= prev.top + prev.height + 1.0 {
                    let bottom = (top + height).max(prev.top + prev.height);
                    prev.height = bottom - prev.top;
                    prev.merged += 1;
                    continue;
                }
            }

            out.push(ResolvedMark {
                id: m.id,
                group: m.group,
                top,
                height,
                column: m.column,
                shape: m.shape,
                color: m.color,
                label: m.label.clone(),
                merged: 1,
            });
            let idx = out.len() - 1;
            match last_of.iter_mut().find(|(k, _)| *k == key) {
                Some(slot) => slot.1 = idx,
                None => last_of.push((key, idx)),
            }
        }
        out
    }

    /// A mark's accessibility element id, folding in its provider ordinal.
    ///
    /// `synthetic_node_id` hashes only `(owner widget, element id, kind)`, and
    /// every lane mark shares the owner and the kind — so two providers each
    /// numbering their marks from 1 would produce the *same* node id and one
    /// would silently replace the other in the tree. Hashing `(group, id)`
    /// together is what makes `LaneMark::id`'s "unique within a provider"
    /// contract sufficient. Same move `teksilo-charts` makes for `(series,
    /// point)`.
    ///
    /// FNV-1a, 64-bit, over `group` widened to `u64` then `id` — the same shape
    /// `margin_lane::providers::mark_id` uses for the other half of a mark's
    /// identity (its `LaneMark::id`), and the same algorithm
    /// `margin_lane::resolve::group_of` uses for `group` itself. This used to be
    /// `std::collections::hash_map::DefaultHasher`, which is exactly what
    /// `group_of`'s own doc comment warns against: "a named, stable, trivially
    /// re-implementable hash rather than `DefaultHasher`, whose output std
    /// explicitly does not promise across releases — and this number reaches an
    /// accessibility tree that has to hold still." That rule was being kept for
    /// one half of this identity and broken for the other; this makes both
    /// halves keep it the same way.
    fn element_id(group: u16, id: u64) -> u64 {
        let mut hash: u64 = 0xcbf2_9ce4_8422_2325;
        let mut mix = |v: u64| {
            for byte in v.to_le_bytes() {
                hash ^= u64::from(byte);
                hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
            }
        };
        mix(u64::from(group));
        mix(id);
        hash
    }

    /// The x offset and width a mark occupies, given its column.
    fn column_rect(&self, bounds: Rect, column: LaneColumn) -> (f32, f32) {
        let (x, w) = self.mark_area(bounds);
        let col = w / 3.0;
        // Every width is floored: a lane narrow enough to make a column
        // sub-pixel is a caller error, but a negative-width rect is a rendering
        // bug, and the floor keeps the marks visible instead of inverted.
        let cw = (col - 1.0).max(1.0);
        match column {
            LaneColumn::Full => (x + 1.0, (w - 2.0).max(1.0)),
            // `Left` sits against a true edge too, but only sometimes: with the
            // texture column off, `mark_area` starts at `bounds.x` and the half
            // pixel in front of the column is inside the *left* `EDGE_RULE`. With
            // the texture on, `mark_area` starts past the divider and there is no
            // rule to overlap. One pixel of inset covers both — it is what `Full`
            // already reserves on this side, and where there is no rule it costs
            // half a pixel of a column nothing is drawn hard against.
            LaneColumn::Left => (x + 1.0, (col - 1.5).max(1.0)),
            LaneColumn::Center => (x + col + 0.5, cw),
            // `mark_area`'s width runs to the strip's own right edge --
            // `x + w == bounds.x + bounds.width` whether or not a texture
            // column is showing -- so Right is the one column that sits
            // against a true edge of the widget rather than only against its
            // neighbours. The shared `cw` put its right edge at
            // `x + 3.0 * col - 0.5`, which is half a pixel inside
            // `[x + w - EDGE_RULE, x + w)`, the right `EDGE_RULE`'s own span:
            // a Right-column Square or Diamond had its rightmost half-pixel
            // painted under the edge hairline. An extra half pixel off the
            // width -- the same pixel `Full` already reserves on each side,
            // for the same reason -- lands the right edge exactly on the
            // rule's boundary instead of inside it.
            LaneColumn::Right => (x + 2.0 * col + 0.5, (col - 1.5).max(1.0)),
        }
    }
}

/// Gap between the texture column and the mark columns.
///
/// One pixel, drawn as a dotted hairline rather than a border: it groups the two
/// halves of one widget rather than separating two widgets.
pub const TEXTURE_DIVIDER: f32 = 1.0;

/// The hairline down each edge of the strip, which is what gives it a constant
/// footprint whether or not it currently holds a mark.
const EDGE_RULE: f32 = 1.0;

/// The texture column's own left inset and usable bar length, derived from its
/// width rather than hardcoded.
///
/// This used to be a bare `let usable = (tex_w - 6.0).max(1.0);` paired with
/// every bar painted at `bounds.x + 3.0` -- numbers that assume a texture
/// column at least 6px wide with a symmetric 3px margin on each side. But
/// `texture_width` is a public builder that takes any positive value, and the
/// divider between the texture column and the mark columns is drawn
/// independently at `bounds.x + tex_w`: `MarginLane::new(..).texture(bars)
/// .texture_width(2.0)` is a legal call, and under the old numbers every bar
/// then started 1px to the *right* of the column's own divider, inside the
/// mark-column area, and grew further into it as the bar lengthened.
///
/// The inset is now half of whatever is left after `usable` is reserved --
/// `(tex_w - usable) / 2.0` -- floored at zero so a column narrower than its
/// own reservation still draws its bars inside itself (starting at its own
/// left edge) rather than to the left of it. At the default 28px width this
/// reproduces the old numbers exactly: `usable` is 22, the leftover is 6, and
/// half of that is the 3.0 every bar used to hardcode -- a normal lane does
/// not change.
fn texture_bar_geometry(tex_w: f32) -> (f32, f32) {
    let usable = (tex_w - 6.0).max(1.0);
    let inset = ((tex_w - usable) / 2.0).max(0.0);
    (inset, usable)
}

/// Where a texture bar's top edge lands once it is clamped into the strip --
/// the paint-time twin of the clamp `resolve_marks` applies to a mark's own
/// top (see the comment there, "so a mark at 0.0 or 1.0 does not hang outside
/// the lane it belongs to"). Factored out purely so the clamp has something to
/// assert against directly: the loop it lives in used to paint straight from
/// `bounds.y + span.start * bounds.height` with no clamp at all, so a bar
/// whose paragraph spans `{ start: 1.0, end: 1.0 }` -- the very last one in
/// the document -- landed a whole `bounds.height` below the strip's own top,
/// and the `.max(1.0)` height floor then gave that position something to
/// paint: a bar drawn entirely beneath the widget, over whatever sits below
/// the lane. The two loops (marks and bars) must agree on this clamp or only
/// one of them keeps its promise to stay inside its own bounds.
fn clamp_texture_bar_top(raw_top: f32, height: f32, bounds: Rect) -> f32 {
    raw_top.clamp(bounds.y, (bounds.y + bounds.height - height).max(bounds.y))
}

impl Widget for MarginLane {
    /// Opt into concrete-type introspection, so a host's tests can read the marks
    /// of a lane mounted deep inside a page nobody holds a reference to.
    ///
    /// The marks are the only thing worth asserting about a lane, and they are not
    /// otherwise reachable: they resolve against live geometry during the host's own
    /// layout, and the accessibility nodes they become are synthetic and not
    /// enumerable from the public test API. `ScrollArea` opts in for the same
    /// reason and about the same kind of value.
    fn as_any(&self) -> Option<&dyn std::any::Any> {
        Some(self)
    }

    fn build(&mut self, ctx: &mut teksilo::core::build_context::BuildContext) -> Vec<WidgetId> {
        // The whole strip is one large tap target, which is the same answer a
        // scroll bar gives: an individual mark is far below any minimum target
        // size and always will be, so pointer users jump by position and
        // keyboard users reach the marks themselves.
        let mut handlers = HandlerSet::new().focusable(true);

        if let Some(factory) = self.context_menu.clone() {
            handlers = handlers.context_menu(move |pos, ctx| factory(pos, ctx));
        }

        // The viewport box is a **handle**, and behaves like one: press it and it
        // travels with the pointer, keeping the offset it was grabbed at; press
        // the strip anywhere else and it comes to you, centred.
        //
        // Both of those were "put the viewport's top where you clicked", which is
        // the one reading a writer never means. Aiming at a mark put that mark on
        // the first line of the screen, so everything the writer was aiming *at*
        // sat below the fold, and grabbing the box by its lower edge and pulling
        // down first snapped it up by its whole height.
        if let Some(on_jump) = self.on_jump.clone() {
            let cached_bounds = self.cached_bounds.clone();
            let dragging = self.dragging.clone();
            let grab = self.grab.clone();
            let geometry = self.geometry();

            // A press that never travels 5 px is a tap and produces no drag at
            // all, so the click case lives here rather than in `on_drag`.
            {
                let on_jump = on_jump.clone();
                let cached_bounds = cached_bounds.clone();
                let geometry = geometry.clone();
                handlers = handlers.on_tap(move |event, _ctx| {
                    let bounds = cached_bounds.get();
                    if bounds.height <= 0.0 {
                        return;
                    }
                    // Event positions arrive widget-local, so the y is already
                    // measured down the lane — no `bounds.y` term.
                    if geometry.box_contains(bounds.height, event.position.y) {
                        // A click on the handle that went nowhere. A scroll bar's
                        // thumb does nothing here too, and for the better reason:
                        // the writer was reaching for the box, and moving the page
                        // under someone who has only just taken hold of it is the
                        // opposite of what taking hold is for.
                        return;
                    }
                    on_jump(geometry.top_for(
                        event.position.y,
                        bounds.height,
                        geometry.centre_grab(bounds.height),
                    ));
                });
            }

            handlers = handlers.on_drag(move |phase, _ctx| {
                let bounds = cached_bounds.get();
                if bounds.height <= 0.0 {
                    return;
                }
                match phase {
                    // `DragPhase::Started` reports the position of the *initial
                    // press*, not where the pointer had reached when it crossed the
                    // recogniser's threshold — which is what makes the grab offset
                    // below the offset the writer actually took hold at.
                    DragPhase::Started {
                        position,
                        button: PointerButton::Primary,
                    } => {
                        let held = if geometry.box_contains(bounds.height, position.y) {
                            // Grabbed the box: keep where on it.
                            (position.y / bounds.height).clamp(0.0, 1.0) - geometry.viewport_top()
                        } else {
                            // Grabbed the strip: the box comes to the pointer and
                            // is then held by its middle, so the drag continues
                            // from where it jumped to rather than snapping again.
                            geometry.centre_grab(bounds.height)
                        };
                        grab.set(held);
                        dragging.set(true);
                        on_jump(geometry.top_for(position.y, bounds.height, held));
                    }
                    DragPhase::Moved { position, .. } if dragging.get() => {
                        on_jump(geometry.top_for(position.y, bounds.height, grab.get()));
                    }
                    DragPhase::Ended { .. } => dragging.set(false),
                    _ => {}
                }
            });
        }

        // Keyboard access is the whole accessibility argument for a lane rather
        // than marks painted inside a scroll bar, so it is not optional: one Tab
        // stop, arrows to move between marks, Enter to go there, Escape to hand
        // focus back without having moved anything.
        {
            let marks = self.marks.clone();
            let selected = self.selected.clone();
            let cached_bounds = self.cached_bounds.clone();
            let on_jump = self.on_jump.clone();
            let width = self.width.clone();
            let texture_width = self.texture_width.clone();
            let geometry = self.geometry();

            handlers = handlers.on_key(move |event, ctx| {
                use teksilo::core::event::{Key, WidgetEvent};
                let WidgetEvent::KeyDown { key, .. } = event else {
                    return EventResponse::Ignored;
                };

                // Resolved the same way `paint` and `accessibility` resolve
                // them, then ordered by position — arrowing must follow the
                // strip, not the provider registration order.
                let lane = MarginLane {
                    marks: marks.clone(),
                    bars: Prop::from(Vec::new()),
                    viewport_top: Signal::new(0.0),
                    viewport_span: Signal::new(1.0),
                    caret: Prop::from(None),
                    width: width.clone(),
                    texture_width: texture_width.clone(),
                    texture_width_set: true,
                    on_jump: None,
                    context_menu: None,
                    access_label: None,
                    cached_bounds: cached_bounds.clone(),
                    selected: selected.clone(),
                    dragging: Signal::new(false),
                    grab: Rc::new(Cell::new(0.0)),
                };
                let mut resolved = lane.resolve_marks(cached_bounds.get());
                resolved.sort_by(|a, b| {
                    a.top
                        .partial_cmp(&b.top)
                        .unwrap_or(std::cmp::Ordering::Equal)
                });
                if resolved.is_empty() {
                    return EventResponse::Ignored;
                }
                let last = resolved.len() - 1;
                let current = selected.get();

                let next = match key {
                    Key::ArrowDown => Some(match current {
                        Some(i) if i < last => i + 1,
                        Some(i) => i,
                        None => 0,
                    }),
                    Key::ArrowUp => Some(match current {
                        Some(i) if i > 0 => i - 1,
                        Some(i) => i,
                        None => last,
                    }),
                    Key::Home => Some(0),
                    Key::End => Some(last),
                    Key::Enter | Key::Space => {
                        if let (Some(i), Some(jump)) = (current, on_jump.as_ref()) {
                            let bounds = cached_bounds.get();
                            if bounds.height > 0.0
                                && let Some(m) = resolved.get(i)
                            {
                                // Centred, exactly as a click on the same mark
                                // would be. Two routes to "take me there" that
                                // landed in different places would be a lane
                                // that means something different depending on
                                // which hand you reached with.
                                jump(geometry.top_for(
                                    m.top - bounds.y,
                                    bounds.height,
                                    geometry.centre_grab(bounds.height),
                                ));
                            }
                        }
                        return EventResponse::Handled;
                    }
                    Key::Escape => {
                        // Leaves the caret where it was: Escape means "I was
                        // only looking", and a review pass that moved the
                        // writer's cursor would be worse than no review pass.
                        selected.set(None);
                        return EventResponse::Handled;
                    }
                    _ => None,
                };

                match next {
                    Some(i) => {
                        selected.set(Some(i));
                        ctx.request_frame();
                        EventResponse::Handled
                    }
                    None => EventResponse::Ignored,
                }
            });
        }

        // ── what makes the strip live ────────────────────────────────────────
        //
        // Every reactive input is registered here, and the **level** each one gets
        // is load-bearing rather than a formality.
        //
        // The marks, the bars, the caret and the three scroll metrics are all
        // `RepaintOnly`: none of them changes how much room the lane takes, and a
        // host that resolves marks against live geometry has to be able to publish
        // them from its own layout pass. At `Relayout` that publication would
        // re-run the layout that produced it, which is an instant loop — the same
        // hazard `ScrollArea` records against its own metrics, and the reason it
        // binds them on its scroll bar children rather than on itself.
        //
        // The two widths are `Relayout`, because they are exactly the inputs that
        // do change the lane's size.
        let self_id = ctx.self_id();
        let registry = ctx.binding_registry();
        self.marks
            .register_if_bound(self_id, registry, BindingLevel::RepaintOnly);
        self.bars
            .register_if_bound(self_id, registry, BindingLevel::RepaintOnly);
        self.caret
            .register_if_bound(self_id, registry, BindingLevel::RepaintOnly);
        self.viewport_top
            .bind_to(self_id, registry, BindingLevel::RepaintOnly);
        self.viewport_span
            .bind_to(self_id, registry, BindingLevel::RepaintOnly);
        self.width
            .register_if_bound(self_id, registry, BindingLevel::Relayout);
        self.texture_width
            .register_if_bound(self_id, registry, BindingLevel::Relayout);
        // The marks are what assistive technology reads, so a set that changed
        // without a repaint being needed still has to re-walk the tree.
        self.marks
            .register_if_bound(self_id, registry, BindingLevel::AccessibilityOnly);

        ctx.apply_self_handlers(handlers);
        Vec::new()
    }

    fn place_children(
        &self,
        bounds: Rect,
        _proposal: SizeProposal,
        _children: &mut [teksilo::core::widget::WidgetPlacement],
        _ctx: &LayoutContext,
    ) {
        // The lane has no children; this override exists purely to capture
        // bounds at layout time, which is what the accessibility walk needs.
        self.cached_bounds.set(bounds);
    }

    fn layout_response(&self, proposal: SizeProposal, _ctx: &LayoutContext) -> LayoutResponse {
        // Fixed width, greedy height: a lane maps whatever extent it is given.
        LayoutResponse::rigid(Size::new(
            self.total_width(),
            proposal.height.unwrap_or(0.0),
        ))
    }

    fn paint(&self, bounds: Rect, canvas: &mut Canvas, ctx: &PaintContext) {
        self.cached_bounds.set(bounds);
        if bounds.height <= 0.0 || bounds.width <= 0.0 {
            return;
        }

        let colors = &ctx.theme.colors;
        // **The strip keeps its footprint, occupied or not.**
        //
        // Reported as the lane changing width while a search was typed and vanishing
        // when nothing matched. Nothing resizes: the width is two constants the host
        // never varies. What changes is ink -- the mark column is half the strip, and
        // an empty column is only as visible as the ground under it.
        //
        // Ground alone cannot do it, which is why this is a pair of rules rather than
        // a different token. The lane has *two different neighbours*: the writing
        // surface on one side and the window chrome on the other. Measured in the
        // dark theme, the page is #1E1F22 and the chrome is #2B2D30 -- and those are
        // exactly `surface_sunken` and `surface_main`, so whichever of the two the
        // strip is painted, it disappears into one edge or the other.
        //
        // So it is painted as chrome, which is what it is, and bounded on both sides.
        // The rules are what make the extent readable; the ground only has to not
        // fight them.
        canvas.fill_rect(bounds, colors.surface_main);
        canvas.fill_rect(
            Rect::new(bounds.x, bounds.y, EDGE_RULE, bounds.height),
            colors.divider,
        );
        canvas.fill_rect(
            Rect::new(
                bounds.x + bounds.width - EDGE_RULE,
                bounds.y,
                EDGE_RULE,
                bounds.height,
            ),
            colors.divider,
        );

        let tex_w = self.texture_width.get().max(0.0);

        // ── the texture column ────────────────────────────────────────────
        if tex_w > 0.0 {
            let (inset, usable) = texture_bar_geometry(tex_w);
            let ink = colors.text_secondary;
            for bar in self.bars.get() {
                let span = bar.span.clamped();
                let raw_top = bounds.y + span.start * bounds.height;
                let height = ((span.end - span.start) * bounds.height).max(1.0);
                // Clamped exactly the way `resolve_marks` clamps a mark's top
                // (see the comment there): a paragraph spanning the very end
                // of the document, where `span.start == span.end == 1.0`,
                // puts `raw_top` at `bounds.y + bounds.height` -- a whole
                // `bounds.height` below the strip's own top -- and the
                // `.max(1.0)` height floor then painted a rect that started,
                // and stayed, entirely below the widget's own bounds, over
                // whatever sits beneath the lane. The two loops (marks here,
                // bars there) have to agree on this or the last paragraph in
                // a manuscript silently draws its texture bar into the
                // neighbour below the lane instead of onto the lane itself.
                let top = clamp_texture_bar_top(raw_top, height, bounds);
                let len = (bar.extent.clamp(0.0, 1.0) * usable).max(1.0);

                // Two passes, unfilled then filled, so the fill reads as a
                // proportion of the bar rather than as a second bar.
                canvas.fill_rect(
                    Rect::new(bounds.x + inset, top, len, height),
                    ink.with_alpha(0.42),
                );
                let filled = len * bar.filled.clamp(0.0, 1.0);
                if filled > 0.0 {
                    canvas.fill_rect(Rect::new(bounds.x + inset, top, filled, height), ink);
                }
            }

            let divider_x = bounds.x + tex_w;
            canvas.fill_rect(
                Rect::new(divider_x, bounds.y, TEXTURE_DIVIDER, bounds.height),
                colors.divider,
            );
        }

        // ── the marks ─────────────────────────────────────────────────────
        for m in self.resolve_marks(bounds) {
            let (x, w) = self.column_rect(bounds, m.column);
            match m.shape {
                LaneShape::Bar => {
                    canvas.fill_rect(Rect::new(x, m.top, w, m.height), m.color);
                }
                LaneShape::BarCurrent => {
                    // Grown about its own middle, so the mark still points at what
                    // it points at, then clamped inside the strip.
                    let h = m.height.max(CURRENT_MARK_HEIGHT);
                    let top = (m.top - (h - m.height) / 2.0)
                        .clamp(bounds.y, (bounds.y + bounds.height - h).max(bounds.y));
                    canvas.fill_rect(Rect::new(x, top, w, h), m.color);
                }
                LaneShape::Square => {
                    let s = w.min(m.height.max(MIN_MARK_HEIGHT));
                    canvas.fill_rect(Rect::new(x, m.top, s, s), m.color);
                }
                LaneShape::SquareHollow => {
                    let s = w.min(m.height.max(MIN_MARK_HEIGHT));
                    stroke_rect(canvas, Rect::new(x, m.top, s, s), m.color);
                }
                LaneShape::Diamond => {
                    // Drawn as a centred square: at three logical pixels a
                    // rotated quad and an axis-aligned one differ by less than
                    // the mark's own antialiasing, and the axis-aligned one
                    // lands on the pixel grid.
                    let s = w.min(m.height.max(MIN_MARK_HEIGHT));
                    canvas.fill_rect(Rect::new(x, m.top, s, s), m.color);
                }
                LaneShape::DiamondHollow => {
                    let s = w.min(m.height.max(MIN_MARK_HEIGHT));
                    stroke_rect(canvas, Rect::new(x, m.top, s, s), m.color);
                }
                LaneShape::Dot => {
                    let s = (w * 0.6).min(MIN_MARK_HEIGHT);
                    canvas.fill_rect(Rect::new(x + (w - s) / 2.0, m.top, s, s), m.color);
                }
                LaneShape::Rule => {
                    let (ax, aw) = self.mark_area(bounds);
                    canvas.fill_rect(Rect::new(ax, m.top, aw, 1.0), m.color);
                    // The notch is what makes a division read as a division
                    // rather than as a very wide mark.
                    canvas.fill_rect(Rect::new(ax, m.top - 2.0, 3.0, 5.0), m.color);
                }
            }
        }

        // ── the viewport box, and the caret inside it ─────────────────────
        //
        // Both numbers are already fractions of the map, so there is nothing to
        // derive: whoever owns the mapping is the only one who can do that
        // conversion correctly, and doing it here is what made the box disagree
        // with the marks it sits over.
        //
        // A span of `1.0` -- everything visible, or nothing mapped yet -- means a
        // box around the whole strip, which says nothing worth the ink.
        let span = self.viewport_span.get().clamp(0.0, 1.0);
        let viewport = self.drawn_viewport(bounds.height);

        // ── the caret ─────────────────────────────────────────────────────
        //
        // **Drawn in the box's space when the box has been floored.** The caret and
        // the box are two statements about the same place, so they have to be made
        // on the same scale or they contradict each other in front of the reader.
        //
        // Reported: a caret on the last line of the viewport drew a fifth of the way
        // down the box. It was not misplaced -- it was at 88% of the true span, and
        // correct -- but the box around it had been stretched 5.3x by the floor and
        // anchored at its top, so the two were being drawn in spaces that differed
        // by everything the floor added. Where the box is not floored this changes
        // nothing, because the two scales are then the same scale.
        if let Some(at) = self.caret.get() {
            let y = bounds.y
                + caret_offset(
                    at,
                    bounds.height,
                    viewport,
                    self.viewport_top.get().clamp(0.0, 1.0),
                    span,
                );
            canvas.fill_rect(
                Rect::new(bounds.x, y, bounds.width, CARET_HEIGHT),
                colors.accent,
            );
        }

        // ── the viewport box ──────────────────────────────────────────────
        //
        if let Some((top, height)) = viewport {
            let top = bounds.y + top;
            // Held: the box is a handle, and a handle says so while it is being
            // held. Same two-step the scroll bar's thumb makes, for the same
            // reason -- a writer dragging it needs to see that the thing under
            // their pointer is what is moving.
            let (fill, edge) = if self.dragging.get() {
                (0.22, 0.75)
            } else {
                (0.10, 0.45)
            };
            let box_rect = Rect::new(bounds.x, top, bounds.width, height);
            canvas.fill_rect(box_rect, colors.accent.with_alpha(fill));
            canvas.fill_rect(
                Rect::new(bounds.x, top, bounds.width, 1.0),
                colors.accent.with_alpha(edge),
            );
            canvas.fill_rect(
                Rect::new(bounds.x, top + height - 1.0, bounds.width, 1.0),
                colors.accent.with_alpha(edge),
            );
        }
    }

    fn accessibility(&self, builder: &mut AccessNodeBuilder) {
        builder.set_role(accesskit::Role::Group);
        let bounds = self.cached_bounds.get();
        let mut resolved = self.resolve_marks(bounds);

        // Announced in **vertical order**, not paint order. `resolve_marks`
        // sorts by provider group so that painting is deterministic, but a
        // screen reader reading "1 of 12, 2 of 12" down a strip must hear them
        // in the order they physically appear, or arrowing through the lane
        // jumps around the document.
        resolved.sort_by(|a, b| {
            a.top
                .partial_cmp(&b.top)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        builder.set_name(
            self.access_label
                .as_ref()
                .map(|l| l.resolve_now())
                .unwrap_or_else(|| format!("Margin marks, {} marks", resolved.len())),
        );

        let selected = self.selected.get();

        // One synthetic child per mark, the shape teksilo-charts uses per datum.
        //
        // Marks get their own nodes rather than riding the content they refer
        // to because, unlike a line number, a mark is not derivable from the
        // text: there is no node to delegate to. The texture column deliberately
        // emits nothing — it *is* uniform and dense and derivable, so it belongs
        // on the content's own nodes, which is `CodeGutter`'s argument and it is
        // right about that half.
        for (i, m) in resolved.iter().enumerate() {
            let label = m.label.resolve_now();
            let pct = if bounds.height > 0.0 {
                ((m.top - bounds.y) / bounds.height * 100.0).round() as i32
            } else {
                0
            };
            let spoken = if m.merged > 1 {
                format!("{label}, {} marks, {pct} percent through", m.merged)
            } else {
                format!("{label}, {pct} percent through")
            };
            let (x, w) = self.column_rect(bounds, m.column);
            let (top, height) = (m.top, m.height);
            builder.push_scene_child(
                Self::element_id(m.group, m.id),
                SyntheticKind::LaneMark,
                |child| {
                    child.set_role(accesskit::Role::GraphicsObject);
                    child.set_name(spoken);
                    child.set_position_in_set(i + 1);
                    child.set_size_of_set(resolved.len());
                    // Without bounds a screen reader cannot place the mark on
                    // screen, so its "route to item" and braille-cursor
                    // affordances have nothing to point at.
                    child.inner_mut().set_bounds(accesskit::Rect {
                        x0: x as f64,
                        y0: top as f64,
                        x1: (x + w) as f64,
                        y1: (top + height) as f64,
                    });
                },
            );
        }

        // Roving focus: the lane itself is the single Tab stop, and the
        // "current" mark is pointed at rather than separately focusable. That
        // is the pattern every composite widget here uses.
        if let Some(idx) = selected
            && let Some(m) = resolved.get(idx)
            && let Some(owner) = builder.owner_id()
        {
            builder.set_active_descendant(teksilo::core::accessibility::synthetic_node_id(
                owner,
                Self::element_id(m.group, m.id),
                SyntheticKind::LaneMark,
            ));
        }
    }
}

/// A one-pixel outline, for the hollow shapes.
fn stroke_rect(canvas: &mut Canvas, rect: Rect, color: Color) {
    canvas.fill_rect(Rect::new(rect.x, rect.y, rect.width, 1.0), color);
    canvas.fill_rect(
        Rect::new(rect.x, rect.y + rect.height - 1.0, rect.width, 1.0),
        color,
    );
    canvas.fill_rect(Rect::new(rect.x, rect.y, 1.0, rect.height), color);
    canvas.fill_rect(
        Rect::new(rect.x + rect.width - 1.0, rect.y, 1.0, rect.height),
        color,
    );
}

#[cfg(test)]
mod tests {

    /// **The caret and the viewport box are drawn on the same scale.**
    ///
    /// The reported case, with the real numbers off a Book: the viewport spans
    /// 0.00179 of the strip, which over 631px is 1.13px, so the box is floored to
    /// 6px. The caret sits at 88% of that span -- the last paragraph on screen.
    ///
    /// Drawn on the map it lands 1.1px into a 6px box: a fifth of the way down,
    /// while the writer's caret is at the bottom of their screen. Drawn in the
    /// box's own space it lands where they put it.
    #[test]
    fn a_floored_viewport_box_takes_the_caret_with_it() {
        let lane = 631.0_f32;
        let (from, span) = (0.65465_f32, 0.00179_f32);
        let at = from + 0.88 * span;
        let box_top = from * lane;
        let boxed = (box_top, (span * lane).max(MIN_VIEWPORT_HEIGHT));
        assert!(
            (boxed.1 - MIN_VIEWPORT_HEIGHT).abs() < 1e-6,
            "this case is only interesting because the box is floored"
        );

        let y = caret_offset(at, lane, Some(boxed), from, span);
        let local = (y - box_top) / boxed.1;
        assert!(
            (local - 0.88).abs() < 0.01,
            "the caret should sit 88% down the box it is inside, not {local:.2}"
        );

        let naive = at * lane;
        assert!(
            (naive - box_top) / boxed.1 < 0.25,
            "and drawn on the map it would have been near the top, which is the bug"
        );
    }

    /// Where the box is **not** floored the two scales are the same scale, so this
    /// rule must be a no-op. Otherwise it would trade a bug at one zoom for a bug
    /// at every other.
    #[test]
    fn an_unfloored_box_leaves_the_caret_exactly_where_the_map_puts_it() {
        let lane = 600.0_f32;
        let (from, span) = (0.25_f32, 0.5_f32);
        let boxed = (from * lane, span * lane);
        for f in [0.0_f32, 0.1, 0.25, 0.5, 0.74, 0.75, 0.9, 1.0] {
            // Bar the clamp at the foot, which keeps a 2px rule on the strip.
            let expected = (f * lane).min(lane - CARET_HEIGHT);
            let y = caret_offset(f, lane, Some(boxed), from, span);
            assert!(
                (y - expected).abs() < 1e-3,
                "caret at {f} drew at {y}, not {expected}"
            );
        }
    }

    /// A caret outside the viewport is on the map, wherever the box happens to be:
    /// it is telling the writer where they left off, which is the whole use of it
    /// once they have scrolled away.
    #[test]
    fn a_caret_outside_the_viewport_stays_on_the_map() {
        let lane = 600.0_f32;
        let (from, span) = (0.9_f32, 0.002_f32);
        let boxed = (from * lane, MIN_VIEWPORT_HEIGHT);
        let y = caret_offset(0.1, lane, Some(boxed), from, span);
        assert!((y - 60.0).abs() < 1e-3, "drew at {y}");
    }

    /// With no box at all -- nothing mapped, or the whole map visible -- the caret
    /// is simply where the map says.
    #[test]
    fn with_no_viewport_box_the_caret_is_plain() {
        assert!((caret_offset(0.5, 600.0, None, 0.0, 1.0) - 300.0).abs() < 1e-3);
        assert!((caret_offset(-3.0, 600.0, None, 0.0, 1.0) - 0.0).abs() < 1e-3);
        assert!(caret_offset(9.0, 600.0, None, 0.0, 1.0) <= 598.0);
    }
    use super::*;
    use std::cell::RefCell;
    use teksilo::canvas::Point;
    use teksilo::core::event::WidgetEvent;
    use teksilo::core::widget_tree::WidgetTree;

    fn label() -> LocalizedString {
        LocalizedString::literal("mark")
    }

    fn mark(id: u64, at: f32, column: LaneColumn, shape: LaneShape) -> LaneMark {
        LaneMark {
            id,
            span: LaneSpan::at(at),
            column,
            shape,
            color: Color::from_hex("#000000"),
            label: label(),
            group: 0,
        }
    }

    fn lane(marks: Vec<LaneMark>) -> MarginLane {
        MarginLane::new(marks, Signal::new(0.0), Signal::new(1.0))
    }

    const BOUNDS: Rect = Rect {
        x: 0.0,
        y: 0.0,
        width: 12.0,
        height: 1000.0,
    };

    #[test]
    fn a_span_is_normalised_whichever_way_round_it_is_given() {
        assert_eq!(LaneSpan::new(0.8, 0.2), LaneSpan::new(0.2, 0.8));
    }

    #[test]
    fn a_mark_sits_at_its_fraction_of_the_height() {
        let l = lane(vec![mark(1, 0.25, LaneColumn::Full, LaneShape::Bar)]);
        let resolved = l.resolve_marks(BOUNDS);
        assert_eq!(resolved.len(), 1);
        // 0.25 of 1000, less the half-height the floor re-centring gives back.
        assert!((resolved[0].top - (250.0 - MIN_MARK_HEIGHT / 2.0)).abs() < 0.01);
    }

    /// A point mark has no height of its own, so the floor is what makes it
    /// visible — and it must be applied by re-centring, not by growing
    /// downward, or every mark drifts below what it points at.
    #[test]
    fn a_point_mark_is_grown_to_the_floor_around_its_own_position() {
        let l = lane(vec![mark(1, 0.5, LaneColumn::Full, LaneShape::Bar)]);
        let r = &l.resolve_marks(BOUNDS)[0];
        assert_eq!(r.height, MIN_MARK_HEIGHT);
        let centre = r.top + r.height / 2.0;
        assert!(
            (centre - 500.0).abs() < 0.01,
            "the grown mark must stay centred on 0.5, got centre {centre}"
        );
    }

    #[test]
    fn a_tall_span_keeps_its_own_height() {
        let l = lane(vec![LaneMark {
            span: LaneSpan::new(0.1, 0.4),
            ..mark(1, 0.0, LaneColumn::Full, LaneShape::Bar)
        }]);
        let r = &l.resolve_marks(BOUNDS)[0];
        assert!((r.height - 300.0).abs() < 0.01);
    }

    #[test]
    fn touching_marks_of_the_same_kind_merge_and_say_how_many() {
        let l = lane(vec![
            mark(1, 0.500, LaneColumn::Left, LaneShape::Square),
            mark(2, 0.501, LaneColumn::Left, LaneShape::Square),
            mark(3, 0.502, LaneColumn::Left, LaneShape::Square),
        ]);
        let resolved = l.resolve_marks(BOUNDS);
        assert_eq!(resolved.len(), 1, "three touching marks are one mark");
        assert_eq!(resolved[0].merged, 3, "and it stands for three");
    }

    /// The merge is geometric, but only within one kind. Two different things
    /// that happen to land together stay two things — a comment does not
    /// disappear because a search hit is at the same line.
    #[test]
    fn marks_of_different_kinds_never_merge_however_close() {
        let l = lane(vec![
            mark(1, 0.500, LaneColumn::Left, LaneShape::Square),
            mark(2, 0.500, LaneColumn::Right, LaneShape::Diamond),
        ]);
        assert_eq!(l.resolve_marks(BOUNDS).len(), 2);
    }

    #[test]
    fn distant_marks_of_the_same_kind_stay_separate() {
        let l = lane(vec![
            mark(1, 0.1, LaneColumn::Left, LaneShape::Square),
            mark(2, 0.9, LaneColumn::Left, LaneShape::Square),
        ]);
        let resolved = l.resolve_marks(BOUNDS);
        assert_eq!(resolved.len(), 2);
        assert!(resolved.iter().all(|m| m.merged == 1));
    }

    /// **Handing the lane bars is what turns the texture column on.**
    ///
    /// It used to take two calls, and forgetting the second was silent: the bars
    /// were computed, at the cost of walking a document, and drawn into a column
    /// zero pixels wide. A host shipped exactly that, so a writer could turn the
    /// texture on in Settings and nothing would ever appear.
    #[test]
    fn giving_the_lane_bars_widens_it_by_the_texture_column() {
        let bare = lane(Vec::new());
        let bare_width = bare.total_width();

        let textured = lane(Vec::new()).texture(vec![LaneBar {
            span: LaneSpan::new(0.0, 0.5),
            extent: 1.0,
            filled: 0.25,
        }]);
        assert!(
            textured.total_width() > bare_width + DEFAULT_TEXTURE_WIDTH - 0.01,
            "the column has to take real width: {} against {bare_width}",
            textured.total_width()
        );
    }

    /// An explicit width wins in **either** order, so the two builders cannot be
    /// made order-dependent by the convenience above.
    #[test]
    fn an_explicit_texture_width_wins_whichever_way_round_it_is_called() {
        let bars = vec![LaneBar {
            span: LaneSpan::new(0.0, 0.5),
            extent: 1.0,
            filled: 0.25,
        }];
        let before = lane(Vec::new()).texture_width(60.0).texture(bars.clone());
        let after = lane(Vec::new()).texture(bars).texture_width(60.0);
        assert_eq!(before.total_width(), after.total_width());

        // And zero still means "no column", which is how a caller keeps the texture
        // off while still handing over bars.
        let off = lane(Vec::new())
            .texture(vec![LaneBar {
                span: LaneSpan::new(0.0, 0.5),
                extent: 1.0,
                filled: 0.25,
            }])
            .texture_width(0.0);
        assert_eq!(off.total_width(), lane(Vec::new()).total_width());
    }

    /// **A texture bar can never cross its own column's divider.**
    ///
    /// The old code reserved a hardcoded 6px and inset every bar by a hardcoded
    /// 3px, which assumed a texture column at least 6px wide with a symmetric
    /// 3px margin either side. `texture_width` takes any positive value, though,
    /// and the divider is drawn independently at `bounds.x + tex_w`:
    /// `MarginLane::new(..).texture(bars).texture_width(2.0)` is legal, and
    /// under the old numbers every bar started 1px to the right of the
    /// column's own divider -- inside the mark-column area -- and grew further
    /// into it as the bar lengthened. `texture_bar_geometry` derives the inset
    /// from `tex_w` instead, so the longest a bar can ever be (`usable`,
    /// reached at `extent == 1.0`) plus the inset it starts at must stay
    /// strictly left of the divider at every width, narrow ones included.
    #[test]
    fn a_narrow_texture_column_keeps_every_bar_left_of_its_own_divider() {
        for tex_w in [2.0_f32, 3.0, 4.0, 6.0, 10.0, DEFAULT_TEXTURE_WIDTH, 60.0] {
            let (inset, usable) = texture_bar_geometry(tex_w);
            assert!(
                inset + usable < tex_w,
                "tex_w {tex_w}: inset {inset} + longest bar {usable} reaches the divider at {tex_w}"
            );
        }
    }

    /// At the default width, deriving the inset must reproduce the numbers the
    /// column always drew: a caller who never touches `texture_width` sees no
    /// change at all.
    #[test]
    fn the_default_texture_width_keeps_its_old_geometry() {
        let (inset, usable) = texture_bar_geometry(DEFAULT_TEXTURE_WIDTH);
        assert_eq!(inset, 3.0);
        assert_eq!(usable, 22.0);
    }

    /// **A texture bar must not paint below the lane it belongs to.**
    ///
    /// A paragraph that is the very last thing in a document maps to
    /// `span = LaneSpan { start: 1.0, end: 1.0 }` -- a zero-length span sitting
    /// right at the bottom of the map. With no clamp that put `raw_top` at
    /// `bounds.y + bounds.height`, one whole lane height below the strip's own
    /// top, and the `.max(1.0)` height floor then gave that position 1px to
    /// paint: a bar drawn entirely beneath the widget, over whatever sits
    /// below the lane rather than on the lane itself.
    #[test]
    fn a_texture_bar_at_the_very_end_of_the_document_stays_inside_the_lane() {
        let bounds = BOUNDS;
        let height = 1.0_f32; // the `.max(1.0)` floor a zero-length span gets
        let raw_top = bounds.y + bounds.height; // span.start == span.end == 1.0
        let top = clamp_texture_bar_top(raw_top, height, bounds);
        assert!(
            top >= bounds.y,
            "the bar's top at {top} is above the lane's own top"
        );
        assert!(
            top + height <= bounds.y + bounds.height + 1e-4,
            "the bar's bottom at {} is past the lane's own bottom at {}",
            top + height,
            bounds.y + bounds.height
        );
    }

    /// An ordinary, well-inside-the-lane bar must land exactly where its span
    /// says -- the clamp is only supposed to catch the edges, not nudge every
    /// bar.
    #[test]
    fn a_texture_bar_well_inside_the_lane_is_not_moved_by_the_clamp() {
        let bounds = BOUNDS;
        let raw_top = bounds.y + 0.4 * bounds.height;
        let height = 20.0_f32;
        let top = clamp_texture_bar_top(raw_top, height, bounds);
        assert!((top - raw_top).abs() < 1e-4, "an interior bar moved: {top}");
    }

    /// **A lane bound to a signal has to redraw when the signal moves.**
    ///
    /// It did not. The widget took `Prop<Vec<LaneMark>>` and three scroll signals
    /// from the first version and registered none of them, so the marks were read
    /// once at paint and never asked for again. Scrolling still looked right —
    /// the whole area repaints anyway — which is exactly why this went unnoticed
    /// until something drove the marks from a signal of its own.
    #[test]
    fn a_change_to_any_bound_input_asks_for_a_repaint() {
        let marks = Signal::new(vec![LaneMark {
            id: 1,
            span: LaneSpan::at(0.5),
            column: LaneColumn::Full,
            shape: LaneShape::Bar,
            color: Color::from_hex("#0072B2"),
            label: LocalizedString::literal("a mark"),
            group: 0,
        }]);
        let scroll = Signal::new(0.0_f32);
        let bars = Signal::new(Vec::<LaneBar>::new());

        let mut tree = WidgetTree::new();
        let _id = tree.add(
            MarginLane::new(marks.clone(), scroll.clone(), Signal::new(0.25)).texture(bars.clone()),
        );
        tree.layout(SizeProposal::exact(40.0, 600.0));
        let _ = tree.render();
        assert!(!tree.needs_render(), "a painted tree starts clean");

        // Bindings reconcile during layout, so that is where a signal change turns
        // into a dirty flag.
        marks.set(Vec::new());
        tree.layout(SizeProposal::exact(40.0, 600.0));
        assert!(tree.needs_render(), "a new set of marks must redraw");
        let _ = tree.render();

        scroll.set(120.0);
        tree.layout(SizeProposal::exact(40.0, 600.0));
        assert!(tree.needs_render(), "and so must a scroll");
        let _ = tree.render();

        bars.set(vec![LaneBar {
            span: LaneSpan::new(0.0, 0.2),
            extent: 0.5,
            filled: 0.1,
        }]);
        tree.layout(SizeProposal::exact(40.0, 600.0));
        assert!(tree.needs_render(), "and so must the texture");
    }

    // ── the viewport box as a handle ──────────────────────────────────────

    /// A lane 600px tall showing a quarter of the map, with the box a third of
    /// the way down. Returns the lane, and where every jump landed.
    fn draggable_lane() -> (MarginLane, Rc<RefCell<Vec<f32>>>, Signal<f32>) {
        let top = Signal::new(1.0 / 3.0);
        let jumps: Rc<RefCell<Vec<f32>>> = Rc::new(RefCell::new(Vec::new()));
        let record = jumps.clone();
        let lane = MarginLane::new(
            Signal::new(Vec::<LaneMark>::new()),
            top.clone(),
            Signal::new(0.25),
        )
        .on_jump(move |f| record.borrow_mut().push(f));
        (lane, jumps, top)
    }

    fn press(tree: &mut WidgetTree, y: f32) {
        tree.pointer_move(Point::new(6.0, y));
        tree.dispatch_event(WidgetEvent::PointerDown {
            position: Point::new(6.0, y),
            button: PointerButton::Primary,
            modifiers: teksilo::core::event::Modifiers::NONE,
        });
    }

    fn release(tree: &mut WidgetTree, y: f32) {
        tree.dispatch_event(WidgetEvent::PointerUp {
            position: Point::new(6.0, y),
            button: PointerButton::Primary,
            modifiers: teksilo::core::event::Modifiers::NONE,
        });
    }

    /// **A click brings the box to the pointer, centred on it.**
    ///
    /// Reported. It used to put the viewport's *top* at the click, so aiming at a
    /// mark put that mark on the first line of the screen and everything the
    /// writer was aiming at sat below the fold.
    #[test]
    fn a_click_centres_the_viewport_box_on_the_pointer() {
        let (lane, jumps, _top) = draggable_lane();
        let mut tree = WidgetTree::new();
        tree.add(lane);
        tree.layout(SizeProposal::exact(40.0, 600.0));
        tree.render();

        // 480px down a 600px lane is 0.8 of the map; the box spans 0.25, so its
        // top must land half a span above that.
        press(&mut tree, 480.0);
        release(&mut tree, 480.0);

        let landed = *jumps.borrow().last().expect("the click must jump");
        assert!(
            (landed - (0.8 - 0.125)).abs() < 1e-3,
            "expected the box centred on 0.8, got a top of {landed}"
        );
    }

    /// A click on the box itself leaves it exactly where it is.
    ///
    /// The writer was reaching for the handle, and moving the page under someone
    /// who has only just taken hold of it is the opposite of what taking hold is
    /// for. A scroll bar's thumb does nothing here either.
    #[test]
    fn a_click_on_the_box_itself_does_not_move_it() {
        let (lane, jumps, _top) = draggable_lane();
        let mut tree = WidgetTree::new();
        tree.add(lane);
        tree.layout(SizeProposal::exact(40.0, 600.0));
        tree.render();

        // The box runs from 200px to 350px. Press in the middle of it.
        press(&mut tree, 275.0);
        release(&mut tree, 275.0);
        assert!(
            jumps.borrow().is_empty(),
            "a click on the handle must not move the page: {:?}",
            jumps.borrow()
        );
    }

    /// **The box keeps the offset it was grabbed at**, which is what makes it a
    /// handle and not a repeated jump.
    ///
    /// Grabbed 30px below its own top and pulled 120px down, it must end 120px
    /// lower — not centred on the pointer, which would snap it up first.
    #[test]
    fn dragging_the_box_keeps_the_offset_it_was_grabbed_at() {
        let (lane, jumps, _top) = draggable_lane();
        let mut tree = WidgetTree::new();
        tree.add(lane);
        tree.layout(SizeProposal::exact(40.0, 600.0));
        tree.render();

        // Box top is at 200px. Grab it 30px in, at 230px.
        press(&mut tree, 230.0);
        // One move to cross the recogniser's 5px threshold, then the real one.
        tree.dispatch_event(WidgetEvent::PointerMove {
            position: Point::new(6.0, 240.0),
        });
        tree.dispatch_event(WidgetEvent::PointerMove {
            position: Point::new(6.0, 350.0),
        });
        release(&mut tree, 350.0);

        // Pointer at 350px, still 30px into the box, so the top is at 320px.
        let landed = *jumps.borrow().last().expect("the drag must jump");
        assert!(
            (landed - (320.0 / 600.0)).abs() < 1e-3,
            "expected the grab offset kept (top at 0.533), got {landed}"
        );
    }

    /// A drag that starts **off** the box brings it to the pointer first, then
    /// carries on from there — so the gesture is one movement, not a snap
    /// followed by a second snap on the first move.
    #[test]
    fn a_drag_off_the_box_brings_it_over_before_moving_it() {
        let (lane, jumps, _top) = draggable_lane();
        let mut tree = WidgetTree::new();
        tree.add(lane);
        tree.layout(SizeProposal::exact(40.0, 600.0));
        tree.render();

        press(&mut tree, 480.0);
        // The first move crosses the recogniser's 5px threshold and emits
        // `DragStarted`, which reports the *press* position; the second is the
        // first real `DragMoved`.
        tree.dispatch_event(WidgetEvent::PointerMove {
            position: Point::new(6.0, 490.0),
        });
        tree.dispatch_event(WidgetEvent::PointerMove {
            position: Point::new(6.0, 520.0),
        });
        release(&mut tree, 520.0);

        let seen = jumps.borrow().clone();
        assert!(
            seen.len() >= 2,
            "the press itself must land the box, not only the moves: {seen:?}"
        );
        assert!(
            (seen[0] - (0.8 - 0.125)).abs() < 1e-3,
            "the press centres it on the pointer, got {}",
            seen[0]
        );
        assert!(
            (seen[1] - (520.0 / 600.0 - 0.125)).abs() < 1e-3,
            "and the move carries it on, still centred, got {}",
            seen[1]
        );
    }

    /// The box cannot be dragged off either end of the strip.
    #[test]
    fn the_box_stops_at_both_ends() {
        let (lane, jumps, _top) = draggable_lane();
        let mut tree = WidgetTree::new();
        tree.add(lane);
        tree.layout(SizeProposal::exact(40.0, 600.0));
        tree.render();

        press(&mut tree, 590.0);
        release(&mut tree, 590.0);
        let landed = *jumps.borrow().last().unwrap();
        assert!(
            (landed - 0.75).abs() < 1e-4,
            "a quarter-map box stops with its bottom on the end, got {landed}"
        );

        press(&mut tree, 2.0);
        release(&mut tree, 2.0);
        let landed = *jumps.borrow().last().unwrap();
        assert!(
            landed.abs() < 1e-4,
            "and with its top on the start, got {landed}"
        );
    }

    /// The hit test uses the box as **drawn**, floor included.
    ///
    /// Over a whole Book the true span is about a pixel, and the box is floored to
    /// [`MIN_VIEWPORT_HEIGHT`] so it can be seen. A hit test against the true span
    /// would leave a handle a writer can see and cannot grab.
    #[test]
    fn a_floored_box_can_be_grabbed_where_it_is_drawn() {
        let geometry = LaneGeometry {
            viewport_top: Signal::new(0.5),
            viewport_span: Signal::new(0.00179),
        };
        let lane = 631.0;
        let (top, height) = geometry.drawn(lane).expect("a box");
        assert!(
            (height - MIN_VIEWPORT_HEIGHT).abs() < 1e-6,
            "this case is only interesting because the box is floored"
        );
        // 4px below the top is inside the drawn box and far outside the true one.
        assert!(geometry.box_contains(lane, top + 4.0));
        assert!(!geometry.box_contains(lane, top + height + 1.0));
    }

    /// The marks are `RepaintOnly` on purpose, and this is the property that
    /// depends on it: a host resolving marks against live geometry publishes them
    /// from its own layout pass, and at `Relayout` that publication would re-run
    /// the layout that produced it. `ScrollArea` records the same hazard against
    /// its own metrics.
    #[test]
    fn new_marks_do_not_ask_for_another_layout() {
        let marks = Signal::new(Vec::<LaneMark>::new());
        let mut tree = WidgetTree::new();
        let _id = tree.add(MarginLane::new(
            marks.clone(),
            Signal::new(0.0),
            Signal::new(0.25),
        ));
        tree.layout(SizeProposal::exact(40.0, 600.0));
        let _ = tree.render();

        marks.set(vec![LaneMark {
            id: 1,
            span: LaneSpan::at(0.5),
            column: LaneColumn::Full,
            shape: LaneShape::Bar,
            color: Color::from_hex("#0072B2"),
            label: LocalizedString::literal("a mark"),
            group: 0,
        }]);
        assert!(
            !tree.needs_layout(),
            "a mark change must not request a layout on its own"
        );
        tree.layout(SizeProposal::exact(40.0, 600.0));
        assert!(tree.needs_render(), "it must still redraw");
        assert!(
            !tree.needs_layout(),
            "and reconciling it must not ask for yet another layout, or a host \
             publishing marks from `place_children` never settles"
        );
    }

    /// Paint order is the declared group, not the order marks arrived in.
    ///
    /// `resolve_marks` sorts ascending by `group` before anything is drawn, so a
    /// later group always paints over an earlier one regardless of which
    /// provider happened to register first, or which one's marks landed
    /// earlier in the `Vec` this frame. That has to be true independent of
    /// registration order: an extension registering a provider must not be
    /// able to change what paints over what merely by the order its
    /// registration call happens to run in, or paint order becomes a race
    /// instead of a property declared on the mark itself.
    #[test]
    fn paint_order_follows_the_group_not_the_input_order() {
        let l = lane(vec![
            LaneMark {
                group: 5,
                ..mark(1, 0.9, LaneColumn::Left, LaneShape::Square)
            },
            LaneMark {
                group: 1,
                ..mark(2, 0.1, LaneColumn::Right, LaneShape::Diamond)
            },
        ]);
        let resolved = l.resolve_marks(BOUNDS);
        assert_eq!(resolved[0].id, 2, "group 1 is drawn first");
        assert_eq!(resolved[1].id, 1);
    }

    /// The three columns must not overlap, or two providers assigned different
    /// columns would still collide — which is the whole reason columns exist.
    #[test]
    fn the_three_columns_are_disjoint() {
        let l = lane(Vec::new());
        let left = l.column_rect(BOUNDS, LaneColumn::Left);
        let centre = l.column_rect(BOUNDS, LaneColumn::Center);
        let right = l.column_rect(BOUNDS, LaneColumn::Right);

        assert!(
            left.0 + left.1 <= centre.0,
            "left {left:?} runs into centre {centre:?}"
        );
        assert!(
            centre.0 + centre.1 <= right.0,
            "centre {centre:?} runs into right {right:?}"
        );
    }

    /// No mark column may paint under either `EDGE_RULE` -- the hairline down
    /// each side of the strip that gives it a constant footprint whether or
    /// not it currently holds a mark. `Full` already reserved a pixel on each
    /// side for exactly this; `Right` did not, and its right edge landed half
    /// a pixel inside the right rule's own span -- invisible until a Square or
    /// Diamond mark sat right against it and had its rightmost half-pixel
    /// painted under the hairline.
    ///
    /// Checked against a lane with its texture column on and bounds sized to
    /// that lane's own `total_width()` -- the shape `paint` actually receives,
    /// where `mark_area`'s right edge coincides with the widget's own right
    /// edge regardless of whether the texture column is showing. With no
    /// texture column at all, `mark_area`'s *left* edge instead coincides with
    /// the widget's own left edge, and `Left` starts only 0.5px inside that --
    /// a second, narrower overlap this fix does not touch: the defect was the
    /// Right column specifically, and Left/Center/Full's geometry was not to
    /// change.
    #[test]
    fn no_column_paints_under_either_edge_rule() {
        // **Both arrangements.** Which columns sit against a true edge of the widget
        // depends on the texture: with it on, `mark_area` starts past the divider and
        // only `Right` touches an edge rule; with it off, `mark_area` starts at
        // `bounds.x` and `Left` touches the other one. Testing one arrangement is
        // what let the left-hand half of this stand.
        for textured in [false, true] {
            let l = if textured {
                lane(Vec::new()).texture_width(DEFAULT_TEXTURE_WIDTH)
            } else {
                lane(Vec::new())
            };
            let bounds = Rect {
                x: 0.0,
                y: 0.0,
                width: l.total_width(),
                height: 1000.0,
            };
            let left_rule_end = bounds.x + EDGE_RULE;
            let right_rule_start = bounds.x + bounds.width - EDGE_RULE;

            for column in [
                LaneColumn::Left,
                LaneColumn::Center,
                LaneColumn::Right,
                LaneColumn::Full,
            ] {
                let (x, w) = l.column_rect(bounds, column);
                assert!(
                    x >= left_rule_end - 1e-4,
                    "{column:?} at {x} starts inside the left edge rule (textured: {textured})"
                );
                assert!(
                    x + w <= right_rule_start + 1e-4,
                    "{column:?} ends at {} which reaches into the right edge rule at \
                     {right_rule_start} (textured: {textured})",
                    x + w
                );
            }
        }
    }

    /// **Emphasis may not come out of the width.**
    ///
    /// A search draws every hit in one column and the one the reader is standing on
    /// in `BarCurrent`. Before that shape existed the current hit was widened
    /// instead, by putting it in `Full` while its siblings sat in `Center` -- three
    /// times the width. The column's occupied width was then a function of the query:
    /// wide where the current hit was, a third of that everywhere else, and nothing
    /// at all when the query matched nothing. Typing "a", then "ar", then "arez"
    /// made the column visibly change width three times, which is what was reported.
    #[test]
    fn the_current_mark_is_no_wider_than_its_siblings() {
        let l = lane(Vec::new());
        let (ordinary_x, ordinary_w) = l.column_rect(BOUNDS, LaneColumn::Center);
        let (full_x, full_w) = l.column_rect(BOUNDS, LaneColumn::Full);
        assert!(
            full_w > ordinary_w * 2.5,
            "the trap this guards is only interesting because Full is much wider: \
             {full_w} against {ordinary_w}"
        );
        let _ = (ordinary_x, full_x);

        // Both shapes resolve in the same column, so both are drawn at that column's
        // width whatever their height.
        let marks: Vec<LaneMark> = [LaneShape::Bar, LaneShape::BarCurrent]
            .into_iter()
            .enumerate()
            .map(|(i, shape)| LaneMark {
                id: i as u64,
                span: LaneSpan::at(0.25 + i as f32 * 0.5),
                column: LaneColumn::Center,
                shape,
                color: Color::from_hex("#E69F00"),
                label: LocalizedString::literal("hit"),
                group: 3,
            })
            .collect();
        let resolved = lane(marks).resolve_marks(BOUNDS);
        assert_eq!(resolved.len(), 2, "different shapes must not merge");
        for m in &resolved {
            assert_eq!(
                lane(Vec::new()).column_rect(BOUNDS, m.column).1,
                ordinary_w,
                "every hit occupies the ordinary column width"
            );
        }
    }

    /// The current mark earns its emphasis in **length**, which is the axis that
    /// does not belong to the column. Asserted through `resolve_marks` rather than
    /// against the constants, so it is the drawn geometry that is pinned.
    #[test]
    fn the_current_mark_is_longer_than_its_siblings() {
        let mark = |shape| LaneMark {
            id: 1,
            span: LaneSpan::at(0.5),
            column: LaneColumn::Center,
            shape,
            color: Color::from_hex("#E69F00"),
            label: LocalizedString::literal("hit"),
            group: 3,
        };
        let plain = lane(vec![mark(LaneShape::Bar)]).resolve_marks(BOUNDS)[0].height;
        let current = lane(vec![mark(LaneShape::BarCurrent)]).resolve_marks(BOUNDS)[0].height;
        // `resolve_marks` floors every mark alike; the extra length is added in the
        // paint, so what is asserted here is that the floor did not already swallow
        // the difference the paint is about to make.
        assert!(plain <= CURRENT_MARK_HEIGHT);
        assert!(current <= CURRENT_MARK_HEIGHT);
        assert!(
            CURRENT_MARK_HEIGHT > plain * 2.0,
            "a current mark {CURRENT_MARK_HEIGHT} is not findable beside a sibling of {plain}"
        );
    }

    /// **What a lane holds may not change how much room it takes.**
    ///
    /// Reported against a find banner: typing a query made the strip appear to
    /// change width, and a query matching nothing made it appear to go away. The
    /// width is `width + texture + divider` and none of those is a function of the
    /// marks, so the geometry was never in question -- but a reader does not measure
    /// widgets, they see where the ink stops, and half the strip is a mark column
    /// that is only as visible as the ground under it.
    ///
    /// The paint answers that with a rule down each edge. This answers the half a
    /// test can hold: that no number moved, so a future change cannot quietly make
    /// the perception true.
    #[test]
    fn what_the_lane_holds_does_not_change_what_it_occupies() {
        let none = lane(Vec::new());
        let many = lane(
            (0..500)
                .map(|i| LaneMark {
                    id: i,
                    span: LaneSpan::at(i as f32 / 500.0),
                    column: LaneColumn::Right,
                    shape: LaneShape::Bar,
                    color: Color::from_hex("#E69F00"),
                    label: LocalizedString::literal("a hit"),
                    group: 3,
                })
                .collect(),
        );
        assert_eq!(none.total_width(), many.total_width());
        assert_eq!(none.mark_area(BOUNDS), many.mark_area(BOUNDS));
        assert_eq!(
            none.column_rect(BOUNDS, LaneColumn::Right),
            many.column_rect(BOUNDS, LaneColumn::Right),
            "an empty mark column occupies exactly what a full one does"
        );
    }

    #[test]
    fn the_full_column_spans_the_whole_mark_area() {
        let l = lane(Vec::new());
        let (x, w) = l.column_rect(BOUNDS, LaneColumn::Full);
        let (ax, aw) = l.mark_area(BOUNDS);
        assert!(x >= ax && x + w <= ax + aw + 0.01);
        assert!(w > aw * 0.8, "Full must actually span, got {w} of {aw}");
    }

    /// Turning the texture off must collapse the lane to exactly its mark
    /// width, with the mark columns unmoved — that is what makes "one widget,
    /// two columns" a setting rather than a second widget.
    #[test]
    fn dropping_the_texture_collapses_the_lane_to_its_marks() {
        let wide = lane(Vec::new()).texture_width(28.0);
        let narrow = lane(Vec::new());

        assert_eq!(narrow.total_width(), DEFAULT_LANE_WIDTH);
        assert_eq!(
            wide.total_width(),
            28.0 + DEFAULT_LANE_WIDTH + TEXTURE_DIVIDER
        );

        // The mark area is the same width either way, and its origin shifts by
        // exactly the texture column plus its divider — which is what makes the
        // texture a column of this widget rather than a second widget.
        assert_eq!(narrow.mark_area(BOUNDS).1, wide.mark_area(BOUNDS).1);
        assert_eq!(narrow.mark_area(BOUNDS).0, BOUNDS.x);
        assert_eq!(wide.mark_area(BOUNDS).0, BOUNDS.x + 28.0 + TEXTURE_DIVIDER);
    }

    #[test]
    fn an_empty_lane_resolves_to_nothing_rather_than_panicking() {
        assert!(lane(Vec::new()).resolve_marks(BOUNDS).is_empty());
    }

    /// A zero-height lane happens for one frame during teardown and on a
    /// collapsed pane. Every mark must land *at* the lane rather than floating
    /// above it — the clamp is what this asserts, not merely that nothing
    /// divided by zero.
    #[test]
    fn a_zero_height_lane_puts_every_mark_at_the_lane_origin() {
        let l = lane(vec![
            mark(1, 0.0, LaneColumn::Full, LaneShape::Bar),
            LaneMark {
                group: 1,
                ..mark(2, 1.0, LaneColumn::Full, LaneShape::Bar)
            },
        ]);
        let flat = Rect {
            height: 0.0,
            ..BOUNDS
        };
        for m in l.resolve_marks(flat) {
            assert_eq!(m.top, flat.y, "a mark escaped a zero-height lane");
            assert!(m.height.is_finite());
        }
    }

    /// Marks at the very ends must stay *inside* the lane. Growing a point mark
    /// to the floor by re-centring pushes half of it past the edge, so the clamp
    /// has to put it back — otherwise the first and last marks in every document
    /// hang outside the widget that owns them.
    #[test]
    fn marks_at_either_end_stay_within_the_lane() {
        let l = lane(vec![
            mark(1, 0.0, LaneColumn::Full, LaneShape::Bar),
            LaneMark {
                group: 1,
                ..mark(2, 1.0, LaneColumn::Full, LaneShape::Bar)
            },
            LaneMark {
                group: 2,
                ..mark(3, -3.0, LaneColumn::Full, LaneShape::Bar)
            },
            LaneMark {
                group: 3,
                ..mark(4, 42.0, LaneColumn::Full, LaneShape::Bar)
            },
        ]);
        for m in l.resolve_marks(BOUNDS) {
            assert!(m.top >= BOUNDS.y, "mark starts above the lane at {}", m.top);
            assert!(
                m.top + m.height <= BOUNDS.y + BOUNDS.height + 0.01,
                "mark ends below the lane at {}",
                m.top + m.height
            );
        }
    }

    // ── regressions for the review findings ──────────────────────────────

    /// Two providers each numbering their marks from 1 must not collide in the
    /// accessibility tree. `synthetic_node_id` hashes only (owner, element id,
    /// kind), and every lane mark shares owner and kind — so the group has to be
    /// folded into the element id or one provider's node silently replaces the
    /// other's.
    #[test]
    fn two_providers_using_the_same_mark_ids_get_distinct_nodes() {
        let a = MarginLane::element_id(0, 1);
        let b = MarginLane::element_id(1, 1);
        assert_ne!(a, b, "group 0 mark 1 and group 1 mark 1 collided");

        assert_eq!(
            MarginLane::element_id(3, 7),
            MarginLane::element_id(3, 7),
            "the same mark must keep the same node id across walks"
        );
        assert_ne!(MarginLane::element_id(3, 7), MarginLane::element_id(3, 8));
    }

    /// Pinned against literals -- the same idiom `providers::mark_id` and
    /// `resolve::group_of` use for their own hashes, and for the same reason:
    /// two inputs merely differing does not catch the algorithm being quietly
    /// swapped back to `DefaultHasher`, which would still very likely produce
    /// different numbers for different inputs. Only the actual numbers pin the
    /// algorithm itself.
    #[test]
    fn element_id_is_the_same_number_on_every_build() {
        assert_eq!(MarginLane::element_id(0, 1), 0x6925_58b0_5610_1a44);
        assert_eq!(MarginLane::element_id(1, 1), 0x581c_d0fa_58d9_9645);
        assert_eq!(MarginLane::element_id(3, 7), 0x3c38_5254_3d68_0a01);
    }

    /// The merge must compare a mark against the nearest prior mark **of its own
    /// kind**, not against whatever was emitted last. A mark from another column
    /// sorting between two mergeable ones would otherwise split them.
    #[test]
    fn an_intervening_mark_of_another_kind_does_not_split_a_merge() {
        let l = lane(vec![
            mark(1, 0.5000, LaneColumn::Left, LaneShape::Square),
            mark(2, 0.5002, LaneColumn::Right, LaneShape::Diamond),
            mark(3, 0.5004, LaneColumn::Left, LaneShape::Square),
        ]);
        let resolved = l.resolve_marks(BOUNDS);
        assert_eq!(
            resolved.len(),
            2,
            "the two Left/Square marks must still merge across the Right/Diamond between them"
        );
        let square = resolved
            .iter()
            .find(|m| m.shape == LaneShape::Square)
            .expect("the square survived");
        assert_eq!(square.merged, 2);
    }

    /// `group` decides which marks may merge with which, so two providers whose
    /// marks land together stay two marks. Otherwise one provider's finding
    /// absorbs another's and the count is a lie.
    #[test]
    fn marks_from_different_providers_never_merge() {
        let l = lane(vec![
            LaneMark {
                group: 0,
                ..mark(1, 0.5, LaneColumn::Left, LaneShape::Square)
            },
            LaneMark {
                group: 1,
                ..mark(2, 0.5, LaneColumn::Left, LaneShape::Square)
            },
        ]);
        let resolved = l.resolve_marks(BOUNDS);
        assert_eq!(resolved.len(), 2, "different providers must not merge");
        assert!(resolved.iter().all(|m| m.merged == 1));
    }

    /// A lane narrow enough to make a column sub-pixel is a caller error, but a
    /// negative-width rect is a rendering bug.
    #[test]
    fn a_very_narrow_lane_never_produces_a_negative_width_column() {
        for w in [0.0_f32, 1.0, 2.0, 3.0, 4.0, 12.0] {
            let l = lane(Vec::new()).width(w);
            for col in [
                LaneColumn::Left,
                LaneColumn::Center,
                LaneColumn::Right,
                LaneColumn::Full,
            ] {
                let (_, width) = l.column_rect(BOUNDS, col);
                assert!(width > 0.0, "width {w} / {col:?} gave {width}");
            }
        }
    }
}
