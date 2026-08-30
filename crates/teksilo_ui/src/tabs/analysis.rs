// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The Analysis segment of a Book container: a bar of categories over one `analyze_book`
//! result, of which this application ships one — Shape — and any number may be contributed
//! from outside through `register_category`.
//!
//! ## What every view here promises
//!
//! **The manuscript is only ever compared to itself.** Not one figure is scored against a
//! genre norm, a target or an external corpus. There is no correct sentence length, dialogue
//! ratio or vocabulary richness, and a panel implying otherwise would be wrong about writing
//! rather than merely unhelpful. Where a bar is tinted, the threshold is the book's own
//! median.
//!
//! **Absence is stated, not drawn as zero.** A language with no curated dialogue convention,
//! a scene too short for a diversity index, a synopsis with no distinctive terms — each
//! shows an em dash and a reason, never a `0%` that reads as a finding.
//!
//! **Nothing is phrased as a fault.** The copy reports counts and comparisons. It never says
//! "too many", "weak" or "should".

use std::cell::RefCell;
use std::rc::Rc;

use teksilo::core::widget::WidgetPlacement;
use teksilo::data::{ChartDatum, ChartModel, ChartSeries};
use teksilo::prelude::*;
use teksilo::widgets::{
    Button, ButtonVariant, Expand, HStack, MaxSize, Padding, ScrollArea, Segment, SegmentedControl,
    Spacer, Switcher, TextWidget, Toggle, VStack,
};
use teksilo::widgets::{SegmentId, segmented_control};
use teksilo_charts::BarChart;
use teksilo_charts::reference_line::ReferenceLine;

use frontend::analysis_management::{BookAnalysisResultDto, SceneAnalyses, SceneAnalysis};

use super::shared::{CHART_HEIGHT, STRIP_HEIGHT, wide_chart};
use super::{Boxed, ContentTab};
use crate::analysis::{AnalysisCategory, AnalysisState, AnalysisViewModel};

// The bar and its `Switcher` are still matched by **position** — that is
// `SegmentedControl`'s contract — but neither is written out by hand: both are built from
// one pass over [`all_categories`], so a category cannot exist as a segment without its
// body or land at a different index in the two. A compile-time assert on the built-in
// count could never have seen that hazard, because the hazard only exists once a category
// is contributed; `the_bar_and_the_switcher_agree` pins it at runtime for exactly that
// case.

/// Builds a registered category's body from the view-model and the finished analysis.
pub type CategoryViewFn = Rc<dyn Fn(&AnalysisViewModel, &BookAnalysisResultDto) -> Box<dyn Widget>>;

/// One category on the Analysis bar: a stable id, its label, and its body.
///
/// The label is a closure rather than a stored `LocalizedString` because the list is
/// rebuilt per render: resolving it at registration would pin the string to whatever
/// locale happened to be active when the extension loaded, and it would never follow a
/// runtime language switch.
#[derive(Clone)]
pub struct AnalysisCategorySpec {
    /// Stable, namespaced for anything not built in (`"ext.style"`). Not shown to the
    /// writer — it exists so a category can be found again across a rebuild.
    pub id: String,
    pub label: crate::docks::LabelFn,
    /// Builds this category's body. Receives the view-model and the finished analysis, so
    /// a registered category can read the same measurements the built-ins do — or ignore
    /// them entirely and render from its own store.
    pub view: CategoryViewFn,
}

impl AnalysisCategorySpec {
    /// This category's stable segment identity, derived from [`Self::id`].
    ///
    /// Derived rather than stored so a registration cannot forget it, and so the same
    /// category keeps the same identity across rebuilds — which is the whole reason the
    /// bar is keyed. Hashed into the app-owned range: `SegmentId::fresh` allocates from
    /// 2^48 up, so folding into 48 bits can never collide with a framework-allocated id.
    ///
    /// The **same** derivation the container bar uses
    /// ([`crate::tabs::shared::segments::segment_id`]), and deliberately not a second one:
    /// this was a `DefaultHasher`, whose output std explicitly does not guarantee across
    /// Rust versions. Nothing persists an analysis category today — only the container
    /// segment is remembered — so it has cost nothing yet, and it is exactly the kind of
    /// thing that costs everything the first time something does.
    pub fn segment_id(&self) -> SegmentId {
        crate::tabs::shared::segments::segment_id(&self.id)
    }
}

struct RegisteredCategory {
    namespace: String,
    spec: AnalysisCategorySpec,
}

// Thread-local, not a `static RwLock`: a spec holds `Rc` closures that build widgets, and
// widgets are single-threaded by construction here. This is the same shape teksilo's own
// tooltip registry uses, and it is the honest one — a `Send + Sync` bound would force every
// extension to box its view builder behind a mutex for a value only the UI thread ever sees.
thread_local! {
    static EXTENSION_CATEGORIES: RefCell<Vec<RegisteredCategory>> = const { RefCell::new(Vec::new()) };
}

/// Add a category to the Analysis bar.
///
/// Refuses an `id` a built-in already uses, or one another namespace registered: the id is
/// how a category is identified across rebuilds, so two claimants make that lookup
/// ambiguous rather than merely crowded.
///
/// ⚠ `SegmentedControl` has a documented five-segment ceiling, and with **two**
/// built-ins the bar reaches it at the third registration — which the downstream
/// edition already makes, so the bar is at its ceiling today. Past roughly six the
/// control stops being a segmented bar at all, which is what `TabWidget::vertical()` is
/// for. Registration does not refuse on count, because refusing the *sixth* category
/// would be an arbitrary line; the ceiling is a design constraint on the control, and the
/// control is the thing that has to change.
///
/// The returned handle unregisters on drop; re-registering a namespace replaces its entry.
pub fn register_category(
    namespace: impl Into<String>,
    spec: AnalysisCategorySpec,
) -> Result<CategoryHandle, String> {
    let namespace = namespace.into();
    if builtin_categories().iter().any(|c| c.id == spec.id) {
        return Err(format!("category id '{}' is a built-in", spec.id));
    }
    EXTENSION_CATEGORIES.with(|reg| {
        let mut reg = reg.borrow_mut();
        if let Some(other) = reg
            .iter()
            .find(|r| r.spec.id == spec.id && r.namespace != namespace)
        {
            return Err(format!(
                "category id '{}' is already registered by '{}'",
                spec.id, other.namespace
            ));
        }
        reg.retain(|r| r.namespace != namespace);
        reg.push(RegisteredCategory {
            namespace: namespace.clone(),
            spec,
        });
        Ok(CategoryHandle {
            namespace: namespace.clone(),
        })
    })
}

/// Unregisters its category when dropped.
#[derive(Debug)]
pub struct CategoryHandle {
    namespace: String,
}

impl Drop for CategoryHandle {
    fn drop(&mut self) {
        // `try_with`: a handle dropped during thread teardown must not panic.
        let _ = EXTENSION_CATEGORIES.try_with(|reg| {
            reg.borrow_mut().retain(|r| r.namespace != self.namespace);
        });
    }
}

/// The categories this application ships, as specs.
fn builtin_categories() -> Vec<AnalysisCategorySpec> {
    AnalysisCategory::ALL
        .iter()
        .map(|c| {
            let c = *c;
            AnalysisCategorySpec {
                id: c.id().to_string(),
                label: Rc::new(move || c.label()),
                view: Rc::new(move |vm, dto| match c {
                    AnalysisCategory::Shape => {
                        // Read once per build, never per bar: a generated name depends on
                        // every item before it, so this walks the manuscript.
                        let names = vm
                            .ids()
                            .work_id
                            .get()
                            .map(|id| crate::models::NameContext::read(&vm.app_ctx(), id));
                        Box::new(shape_view(
                            dto,
                            vm.ignore_empty(),
                            vm.footnote_words(),
                            names.as_ref(),
                        ))
                    }
                    // Reads nothing from `dto`: this category measures how text
                    // *arrived*, which no analysis run produces and no scope
                    // narrows. It is here rather than on a dock because it is a
                    // measurement about the writing, and this is where a writer
                    // comes to read those.
                    AnalysisCategory::Arrivals => Box::new(arrivals_view(
                        &common::arrival::shared()
                            .session_total(&vm.work_unique_id().unwrap_or_default()),
                    )),
                }),
            }
        })
        .collect()
}

/// Built-ins first, then anything registered — the order the bar renders in.
pub fn all_categories() -> Vec<AnalysisCategorySpec> {
    EXTENSION_CATEGORIES.with(|reg| {
        builtin_categories()
            .into_iter()
            .chain(reg.borrow().iter().map(|r| r.spec.clone()))
            .collect()
    })
}

pub fn analysis_pane(tab: &ContentTab) -> Box<dyn Widget> {
    let Some(vm) = tab.analysis() else {
        // Not a Book. Structurally unreachable through `folder_book`, but a pane that
        // renders nothing is better than one that panics if the gate ever widens.
        return Box::new(VStack::new());
    };
    Box::new(AnalysisPane {
        vm: vm.clone(),
        backdrop: tab.backdrop_role(),
        root: None,
    })
}

struct AnalysisPane {
    vm: AnalysisViewModel,
    /// The tab's own page colour, so switching between Pace and Analysis does not change
    /// the surface under the charts.
    backdrop: SurfaceRole,
    root: Option<WidgetId>,
}

impl std::fmt::Debug for AnalysisPane {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AnalysisPane").finish_non_exhaustive()
    }
}

impl Widget for AnalysisPane {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        use teksilo::core::BindingLevel;

        self.vm
            .state()
            .bind_to(ctx.self_id(), ctx.binding_registry(), BindingLevel::Rebuild);
        // Shape reads this while building its charts, so flipping it has to rebuild the
        // pane — the whole result is already in hand, nothing is re-analysed.
        self.vm.ignore_empty().bind_to(
            ctx.self_id(),
            ctx.binding_registry(),
            BindingLevel::Rebuild,
        );
        // The footnote-word figure lands from its own, separately-timed long operation
        // (see `AnalysisViewModel`'s module doc), so it can settle *after* the main
        // result has already put the pane in `Ready` — this binding is what makes that
        // late arrival repaint the Shape line rather than sitting stale until some
        // unrelated rebuild happens to pick it up.
        self.vm.footnote_words().bind_to(
            ctx.self_id(),
            ctx.binding_registry(),
            BindingLevel::Rebuild,
        );

        // The one automatic run: the writer navigated here to see a report, and Pace's own
        // empty state is the precedent for not making them click first.
        self.vm.run_once_on_open();

        // Long-operation events reach the panel here rather than through the app's global
        // wiring, because the view-model is per-tab: a Book and a Part open side by side
        // each filter for their own operation id. All three terminal variants share one
        // handler, so they are subscribed in a loop — the idiom `app/wiring/long_ops.rs`
        // already uses for the same shape.
        use frontend::common::event::LongOperationEvent as L;
        for event in [L::Completed, L::Failed, L::Cancelled] {
            let vm = self.vm.clone();
            ctx.subscribe_event(
                frontend::common::event::Origin::LongOperation(event),
                move |e| vm.on_long_op_event(e),
            );
        }

        let id = ctx.add_boxed(self.body());
        self.root = Some(id);
        vec![id]
    }

    fn layout_response(&self, proposal: SizeProposal, ctx: &LayoutContext) -> LayoutResponse {
        self.root
            .and_then(|id| ctx.child_size(id, proposal))
            .map(LayoutResponse::from)
            .unwrap_or_else(|| proposal.resolve(0.0, 0.0).into())
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

    // Reported here as well as from `build`: a filling child that is not listed is never
    // placed by the layout pass, and the whole pane silently vanishes.
    fn children(&self) -> Vec<WidgetId> {
        self.root.into_iter().collect()
    }
}

impl AnalysisPane {
    fn body(&self) -> Box<dyn Widget> {
        // Same backdrop as every other container segment, so switching between Pace and
        // Analysis does not change the page under the charts.
        let mut col = VStack::new()
            .spacing(10.0)
            .child(super::shared::vspace(18.0))
            .child(Padding::symmetric(0.0, 24.0).child(self.header()));

        match self.vm.state().get() {
            AnalysisState::Idle => {
                col = col.child(Padding::symmetric(0.0, 24.0).child(note(tr!(analysis_not_run()))));
            }
            AnalysisState::Running => {
                col = col.child(Padding::symmetric(0.0, 24.0).child(note(tr!(analysis_running()))));
            }
            AnalysisState::Failed(msg) => {
                // A failure is said out loud. An empty report here would read as "your book
                // is fine", which is the one thing it does not mean.
                col = col.child(note(tr!(analysis_failed())));
                if !msg.is_empty() {
                    col = col.child(note(lit!(msg)));
                }
            }
            AnalysisState::Ready(dto) => {
                // Resolved once and shared: the bar and the switcher are paired by
                // index, so computing the list twice would let a registration landing
                // between the two calls shift one and not the other.
                let cats = all_categories();
                col = col
                    .child(Padding::symmetric(0.0, 24.0).child(self.category_bar(&cats)))
                    .child(Expand::new().child(self.categories(&cats, &dto)));
            }
        }
        super::shared::tab_backdrop(self.backdrop, col)
    }

    fn header(&self) -> impl Widget {
        let vm = self.vm.clone();
        let running = self.vm.is_running();
        let stale = self.vm.is_stale();
        teksu!(
            HStack {
                spacing: 8.0
                TextWidget::new(tr!(analysis_scope_book())) {
                    style: TextStyleRole::Small
                    color: TextRole::Secondary
                }
                Spacer
                // Staleness is a plain statement of fact, not a warning: the result is
                // still the one that was computed, and the writer decides whether it is
                // worth re-running.
                if stale {
                    TextWidget::new(tr!(analysis_stale())) {
                        style: TextStyleRole::Tiny
                        color: TextRole::Secondary
                    }
                }
                Button::new(tr!(analysis_run())) {
                    variant: ButtonVariant::Plain
                    enabled: !running
                    on_activate_fn: move |_| vm.run()
                }
            }
        )
    }

    fn category_bar(&self, cats: &[AnalysisCategorySpec]) -> impl Widget {
        let mut bar = SegmentedControl::new(self.vm.category());
        for c in cats {
            bar = bar.segment(Segment::new((c.label)()).id(c.segment_id()));
        }
        bar
    }

    /// The `Switcher`'s children sit at the same positions as the bar's segments — matched
    /// by index, not by name, which is the trap `tabs.rs` documents at length. Both are
    /// built from the **same slice** in the same order, by the same caller, which is what
    /// makes that pairing true rather than merely intended.
    fn categories(
        &self,
        cats: &[AnalysisCategorySpec],
        dto: &BookAnalysisResultDto,
    ) -> impl Widget {
        let ids: Vec<SegmentId> = cats.iter().map(|c| c.segment_id()).collect();
        let mut sw = Switcher::new(segmented_control::index_signal(&self.vm.category(), &ids));
        for c in cats {
            sw = sw.child_boxed(scrolled_boxed((c.view)(&self.vm, dto)));
        }
        sw
    }
}

/// A padded, scrollable wrapper for an already-boxed body — what a category
/// spec hands back. (It had an unboxed sibling, deleted as unused.)
fn scrolled_boxed(inner: Box<dyn Widget>) -> Box<dyn Widget> {
    Box::new(teksu!(
        ScrollArea {
            Padding::symmetric(0.0, 24.0) {
                Boxed::new(inner)
            }
        }
    ))
}

use crate::shared::text::caption as note;

fn heading(text: impl Into<LocalizedString>) -> impl Widget {
    teksu!(
        TextWidget::new(text) {
            style: TextStyleRole::Tiny
            color: TextRole::Secondary
        }
    )
}

/// The width a paragraph of explanatory prose is held to.
///
/// About eighty characters at the body size, which is the top of the range a reader can
/// track from the end of one line to the start of the next.
const PROSE_MEASURE: f32 = 640.0;

/// A paragraph explaining what a section measures, held to [`PROSE_MEASURE`].
///
/// ⚠ Not the same thing as [`note`], and the difference is the whole reason this exists.
/// `TextWidget` wraps by default, but it only wraps against a width its parent proposes,
/// and nothing above these sections proposes one: the pane hands its children the whole
/// content column. On a maximised window that column is around 1,450 logical pixels, so a
/// paragraph laid out through `note` alone becomes a single line of roughly two hundred
/// characters. It is not a wrapping bug; there is simply no width to wrap at until
/// something names one.
fn prose(text: impl Into<LocalizedString>) -> impl Widget {
    teksu!(
        MaxSize::width(PROSE_MEASURE) {
            child: note(text)
        }
    )
}

/// **What this row is called on screen**, which is not always its title.
///
/// An untitled chapter is "Chapter 3" in the outline, generated from its position in the
/// manuscript, and the axis of a chart of that same manuscript has to agree: a column of
/// bars all labelled with the same blank is not an axis. [`crate::models::NameContext`] is the one place
/// that question is answered, and the outline, the search tree and this chart all ask it.
///
/// Falls back to the row's own title when there is no context to ask (a test, or a project
/// still loading) and when the row has no generated name to offer: only structural rows
/// are numbered, so a genuinely untitled leaf scene keeps whatever it had.
fn bar_label(
    names: Option<&crate::models::NameContext>,
    item_id: common::types::EntityId,
    title: &str,
) -> String {
    if !title.trim().is_empty() {
        return title.to_string();
    }
    names
        .and_then(|n| n.item(item_id).and_then(|it| n.generated_name(it)))
        .unwrap_or_else(|| title.to_string())
}

fn scenes_of(dto: &BookAnalysisResultDto) -> Vec<&SceneAnalysis> {
    match &dto.scenes {
        SceneAnalyses::Measured(rows) => rows.iter().collect(),
        SceneAnalyses::Empty => vec![],
    }
}

/// A text with no prose in it — never measured, or measured to nothing.
///
/// Part headings, chapter folders and scenes that exist only as a title all land here. They
/// are real rows of the binder, which is why they are counted and offered rather than
/// dropped outright, but in a bar chart each one is a gap where the eye expects a bar. A
/// book outlined ahead of its drafting is mostly gaps.
fn is_empty_text(scene: &SceneAnalysis) -> bool {
    match scene {
        SceneAnalysis::Empty => true,
        SceneAnalysis::Measured { words, .. } => *words == 0,
    }
}

/// The control for [`is_empty_text`] filtering, drawn above the charts it governs.
fn empty_toggle(ignore_empty: Signal<bool>) -> impl Widget {
    teksu!(
        HStack {
            Toggle::new(ignore_empty) {
                label: tr!(analysis_ignore_empty())
            }
            Spacer
        }
    )
}

// ── Shape ─────────────────────────────────────────────────────────────────────

/// Word count along the stream, plus dialogue share where the language admits one.
///
/// Bars are tinted against the book's **own median**, which is the whole point: there is no
/// correct scene length, only a scene that is unlike its neighbours. Using the median rather
/// than the mean keeps one 6,000-word chapter from moving the line everything else is judged
/// against.
fn shape_view(
    dto: &BookAnalysisResultDto,
    ignore_empty: Signal<bool>,
    footnote_words: Signal<Option<i64>>,
    names: Option<&crate::models::NameContext>,
) -> impl Widget {
    // Read once, up front, so every return path below — including the two early "no
    // scenes"/"all texts empty" ones — carries the same footnote line. It does not
    // depend on `scenes_of` at all: a Book folder with a note on its own synopsis and
    // not one scene written yet still has a real footnote figure to show.
    let footnote_section = footnote_words_section(footnote_words.get());
    let all = scenes_of(dto);
    if all.is_empty() {
        return teksu!(
            VStack {
                child: note(tr!(analysis_no_scenes()))
                child: footnote_section
            }
        );
    }
    let hidden = all.iter().filter(|s| is_empty_text(s)).count();
    let hiding = ignore_empty.get();
    let scenes: Vec<&SceneAnalysis> = if hiding {
        all.into_iter().filter(|s| !is_empty_text(s)).collect()
    } else {
        all
    };
    if scenes.is_empty() {
        // Every text is empty and they are all hidden — say which, or the pane reads as
        // "this book has no scenes" when in fact it has scenes with nothing written yet.
        return teksu!(
            VStack {
                spacing: 10.0
                child: empty_toggle(ignore_empty)
                child: note(tr!(analysis_all_texts_empty()))
                child: footnote_section
            }
        );
    }

    let words: Vec<i64> = scenes
        .iter()
        .filter_map(|s| match s {
            SceneAnalysis::Measured { words, .. } => Some(*words),
            SceneAnalysis::Empty => None,
        })
        .collect();
    // The shared definition, so the tint threshold here and any mean/stddev shown
    // elsewhere in this panel cannot be computed two different ways.
    let median = skribisto_model::analysis::stats::median(
        &words.iter().map(|&w| w as f64).collect::<Vec<_>>(),
    )
    .unwrap_or(0.0);

    let mut points: Vec<ChartDatum<String>> = Vec::new();
    let mut dialogue_points: Vec<ChartDatum<String>> = Vec::new();
    for s in &scenes {
        let SceneAnalysis::Measured {
            item_id,
            title,
            words,
            dialogue,
            ..
        } = s
        else {
            continue;
        };
        let title = &bar_label(names, *item_id, title);
        // One colour for the whole series, and the median drawn as a line below. The
        // earlier design tinted below-median bars `SurfaceRole::Raised` to set them apart,
        // which in the light theme is `#FFFFFF` — the same value as the `Content` page they
        // sit on. Every below-median bar was painted white on white, so a book whose scenes
        // mostly run under its own median showed a chart with most of its bars missing, and
        // the ones that survived were exactly the ones being compared *against* an invisible
        // threshold. The line says the same thing and cannot disappear into the page.
        points.push(ChartDatum::new(title.clone(), *words as f32).with_color(SurfaceRole::Accent));
        if let Some(d) = dialogue {
            dialogue_points.push(
                ChartDatum::new(title.clone(), (*d * 100.0) as f32).with_color(SurfaceRole::Accent),
            );
        }
    }

    // The dialogue strip is one heading followed by one of two bodies — hence the
    // heading hoisted out of the branch and a `child_opt` pair below it: a `teksu!`
    // `if/else` arm holds a single element, and the heading is common to both.
    let no_dialogue = dialogue_points.is_empty();
    teksu!(
        VStack {
            spacing: 10.0
            child: empty_toggle(ignore_empty)
            child_opt: (hiding && hidden > 0)
            .then(|| note(tr!(analysis_empty_hidden(count = hidden as i64))))
            child: heading(tr!(analysis_words_per_scene()))
            child: wide_chart(
                points.len(),
                CHART_HEIGHT,
                BarChart::new(ChartModel::from_series_vec(vec![
                    ChartSeries::new(tr!(analysis_words_per_scene()).resolve_now()).data(points),
                ]))
                .grid(true)
                .legend(false)
                // The comparison the panel is built on, drawn rather than described. A
                // caption saying "the median is 2,495" asks the reader to hold a number
                // in their head and eyeball every bar against it.
                .reference_line(ReferenceLine::new(
                    median as f32,
                    tr!(analysis_median_line(count = median.round() as i64)),
                )),
            )
            child: note(tr!(analysis_median_words(count = median.round() as i64)))
            child: heading(tr!(analysis_dialogue()))
            // Not "0% dialogue" — the language has no curated convention, which is a
            // different and honest statement.
            child_opt: no_dialogue.then(|| note(tr!(analysis_dialogue_unsupported())))
            child_opt: (!no_dialogue).then(|| {
                wide_chart(
                    dialogue_points.len(),
                    STRIP_HEIGHT,
                    BarChart::new(ChartModel::from_series_vec(vec![
                        ChartSeries::new(tr!(analysis_dialogue()).resolve_now())
                            .data(dialogue_points),
                    ]))
                    .grid(true)
                    .legend(false),
                )
            })
            child: footnote_section
        }
    )
}

/// The book's footnote-word figure, kept visually apart from the words-per-scene chart
/// above it for the same reason `progress_management::count_words_uc` keeps the two
/// totals apart in the data: a footnote is authored prose, but showing it as one more
/// bar in "words per scene" would credit a heavily annotated scene with story progress
/// it did not make.
///
/// `None` is shown as "still counting" rather than as `0` — the figure comes from its
/// own long operation (see `AnalysisViewModel`'s module doc) that can still be in
/// flight even once the rest of Shape is ready to show.
/// **How text reached this project while it has been open.**
///
/// ## The two scopes, and why they are said out loud
///
/// Everything else on this bar is about **one Book** and about **the whole of
/// its text**. This is about the whole **project** and only about **this
/// session**, and both departures are stated in the pane rather than left for a
/// reader to assume the narrower meaning the rest of the bar has taught them.
///
/// Neither is fixable here. The count is taken where text is inserted, and an
/// editor knows which project it belongs to and not which Book — a project can
/// hold several, and resolving one per inserted character would mean walking the
/// binder on every keystroke. And nothing persists it: the tally lives in memory
/// and goes when the project closes, so there is no earlier session to add.
///
/// ## What it must never become
///
/// A fact about **input**, never about authorship, and there is nothing here to
/// score. No route is weighed against another, none is ordered above another,
/// there is no total to reach and no threshold anywhere in it. A writer who
/// drafts elsewhere and pastes chapters in has pasted; one who dictates has
/// dictated; neither says anything about who wrote the words.
///
/// A route that contributed nothing says so, rather than being left out: a
/// missing line reads as "not measured", which would be a different claim.
fn arrivals_view(counts: &common::arrival::Counts) -> impl Widget {
    use common::arrival::Arrival;

    let total: u64 = counts.values().copied().sum();
    // Fixed order, from `Arrival::ALL` — the declaration order, which is not
    // a ranking and is not sorted by size. Sorting by count would put the
    // largest route first and invite reading it as the finding.
    let routes: Vec<(LocalizedString, LocalizedString)> = if total == 0 {
        Vec::new()
    } else {
        Arrival::ALL
            .into_iter()
            .map(|route| {
                let n = counts.get(&route).copied().unwrap_or(0);
                let label = match route {
                    Arrival::Typed => tr!(analysis_arrivals_typed()),
                    Arrival::Pasted => tr!(analysis_arrivals_pasted()),
                    Arrival::Dictated => tr!(analysis_arrivals_dictated()),
                    Arrival::Imported => tr!(analysis_arrivals_imported()),
                    Arrival::Programmatic => tr!(analysis_arrivals_programmatic()),
                };
                let value = if n == 0 {
                    tr!(analysis_arrivals_none())
                } else {
                    tr!(analysis_arrivals_count(count = n as i64))
                };
                (label, value)
            })
            .collect()
    };

    let rows = teksu!(
        VStack {
            spacing: 6.0
            child_opt: (total == 0).then(|| note(tr!(analysis_arrivals_nothing())))
            for (label, value) in routes.into_iter() {
                HStack {
                    spacing: 8.0
                    TextWidget::new(label)
                    child: note(value)
                }
            }
        }
    );

    teksu!(
        VStack {
            spacing: 10.0
            child: heading(tr!(analysis_arrivals()))
            child: prose(tr!(analysis_arrivals_explainer()))
            child: rows
            // Below the figures, not above: they are caveats on what was just read,
            // and a reader who takes nothing else from this pane should still take
            // these two.
            child: note(tr!(analysis_arrivals_scope()))
            child: note(tr!(analysis_arrivals_session()))
        }
    )
}

fn footnote_words_section(value: Option<i64>) -> impl Widget {
    let line = match value {
        Some(n) => note(tr!(analysis_footnote_words_count(count = n))),
        None => note(tr!(analysis_footnote_words_pending())),
    };
    teksu!(
        VStack {
            spacing: 6.0
            child: heading(tr!(analysis_footnote_words()))
            child: line
        }
    )
}

#[cfg(test)]
mod arrivals_tests {
    use common::arrival::{Arrival, Counts};

    /// **Every route is named, including the ones that contributed nothing.**
    /// A missing line reads as "not measured", which is a different claim from
    /// "none arrived that way" — and the second is the true one.
    #[test]
    fn every_route_is_named_even_at_zero() {
        let mut counts = Counts::new();
        counts.insert(Arrival::Typed, 1200);

        // The view's own lookup, on a tally where four of the five routes are
        // simply absent from the map. Each has to resolve to a number the view
        // can render as "none" — an absent key that yielded no row at all is how
        // "none arrived that way" would silently become "not measured".
        let rendered: Vec<u64> = Arrival::ALL
            .iter()
            .map(|route| counts.get(route).copied().unwrap_or(0))
            .collect();
        assert_eq!(rendered, vec![1200, 0, 0, 0, 0]);
        assert_eq!(rendered.len(), 5, "five routes, all of them shown");
    }

    /// **The order is the declaration order, and is never a ranking.** Sorting
    /// by size would put the largest route first and invite reading it as the
    /// finding — which is exactly the verdict this pane must not deliver.
    #[test]
    fn the_route_order_does_not_depend_on_the_counts() {
        let mut lopsided = Counts::new();
        lopsided.insert(Arrival::Programmatic, 900_000);
        lopsided.insert(Arrival::Typed, 1);
        // `Arrival::ALL` is what the view iterates, and it is a const array.
        assert_eq!(
            Arrival::ALL,
            [
                Arrival::Typed,
                Arrival::Pasted,
                Arrival::Dictated,
                Arrival::Imported,
                Arrival::Programmatic,
            ],
            "the order is fixed in the type, so no distribution of counts can reorder it"
        );
        assert_eq!(lopsided.get(&Arrival::Typed), Some(&1));
    }

    /// A project nothing has been typed into yet says so, rather than showing
    /// five zeros — which would read as a measurement of an empty book rather
    /// than as a session that has not started.
    #[test]
    fn an_untouched_session_has_nothing_to_report() {
        let counts = Counts::new();
        assert_eq!(counts.values().copied().sum::<u64>(), 0);
    }
}

#[cfg(test)]
mod tests {
    use frontend::analysis_management::{BookAnalysisResultDto, SceneAnalyses, SceneAnalysis};

    fn scene(item_id: u64, title: &str, chapter: &str) -> SceneAnalysis {
        SceneAnalysis::Measured {
            item_id,
            title: title.to_string(),
            chapter_title: chapter.to_string(),
            words: 100,
            sentence_mean: None,
            sentence_stddev: None,
            paragraph_mean: None,
            punctuation_per_1k: 0.0,
            dialogue: None,
        }
    }

    fn dto(scenes: Vec<SceneAnalysis>) -> BookAnalysisResultDto {
        BookAnalysisResultDto {
            scenes: SceneAnalyses::Measured(scenes),
            ..Default::default()
        }
    }

    /// Both ways an unwritten text can be recorded count as empty. The `Empty` variant is
    /// the analyser declining to measure; a `Measured` zero is it measuring nothing — and a
    /// filter that caught only one of them would leave half the gaps in the chart.
    #[test]
    fn an_unwritten_text_is_empty_however_it_was_recorded() {
        assert!(super::is_empty_text(&SceneAnalysis::Empty));
        let mut blank = scene(1, "Chapter 12", "");
        if let SceneAnalysis::Measured { words, .. } = &mut blank {
            *words = 0;
        }
        assert!(super::is_empty_text(&blank));
        assert!(
            !super::is_empty_text(&scene(2, "Chapter 13", "")),
            "100 words is not empty"
        );
    }

    /// The tint threshold is the shared median, not a local one — pinned here because the
    /// property it buys is what this panel depends on: a single very long chapter must not
    /// drag the comparison line up with it.
    #[test]
    fn the_tint_threshold_is_a_median_not_a_mean() {
        use skribisto_model::analysis::stats;
        assert_eq!(stats::median(&[1.0, 2.0, 3.0, 4.0, 100.0]), Some(3.0));
        assert_eq!(stats::mean(&[1.0, 2.0, 3.0, 4.0, 100.0]), Some(22.0));
    }

    // ── the footnote-word line ───────────────────────────────────────────────

    /// Shape must still lay out once a real footnote figure is known — the line joins
    /// the words-per-scene chart rather than replacing it.
    #[test]
    fn shape_lays_out_with_a_known_footnote_figure() {
        use teksilo::core::widget_tree::WidgetTree;
        use teksilo::prelude::{Signal, SizeProposal};

        let d = dto(vec![scene(1, "Chapter 1", "")]);
        let mut tree = WidgetTree::new();
        let id = tree.add_boxed(Box::new(super::shape_view(
            &d,
            Signal::new(true),
            Signal::new(Some(420)),
            None,
        )));
        tree.layout(SizeProposal::exact(700.0, 900.0));
        assert!(tree.bounds(id).height > 0.0, "the pane laid out to nothing");
    }

    // ── the chart's axis labels ──────────────────────────────────────────────

    /// **A titled row keeps its title, and an untitled one does not become blank.**
    ///
    /// A chapter-folder book keeps its prose in unnamed rows, so on a real manuscript most
    /// of this chart's bars have no title of their own. The outline names them by their
    /// position and this axis has to agree, or one book is labelled two ways in two panes
    /// three inches apart.
    ///
    /// The generated-name path itself needs a store to number against; what is asserted
    /// here is that the fallbacks do not lose a name that already exists.
    #[test]
    fn a_bar_keeps_its_own_title_and_never_falls_back_over_one() {
        assert_eq!(super::bar_label(None, 7, "Prologue"), "Prologue");
        assert_eq!(
            super::bar_label(None, 7, "  Low tide  "),
            "  Low tide  ",
            "a title with surrounding space is still a title"
        );
        assert_eq!(
            super::bar_label(None, 7, ""),
            "",
            "with nothing to ask, the row keeps exactly what it had"
        );
    }

    // ── the prose measure ────────────────────────────────────────────────────

    /// A paragraph of the length these sections actually carry.
    const LONG: &str = "How text reached this project while it has been open. It says which \
                        route the characters came down, and nothing at all about who wrote \
                        them: a writer who drafts elsewhere and pastes has pasted, and one \
                        who dictates has dictated. There is no number to aim for here.";

    /// **A paragraph must wrap, and a wide pane is what stops it.**
    ///
    /// `TextWidget` wraps by default, but only against a width its parent proposes. Given
    /// the whole content column of a maximised window it has no width to wrap at and lays
    /// out as one very long line. This asserts the opposite: offered far more room than the
    /// measure, the paragraph takes the measure and grows downwards instead.
    ///
    /// ⚠ Measured on the **child**, never on the root. A root is handed
    /// `SizeProposal::exact` and reports exactly that whatever it wanted, so a root-level
    /// assertion here passes and fails for reasons that have nothing to do with wrapping.
    #[test]
    fn an_explainer_paragraph_is_held_to_a_measure_however_wide_the_pane_is() {
        use super::prose;
        use teksilo::core::widget_tree::WidgetTree;
        use teksilo::prelude::{SizeProposal, lit};
        use teksilo::widgets::VStack;

        let mut tree = WidgetTree::new();
        // A literal rather than the real key: this asserts a layout rule, and it must not
        // start passing because a translator shortened a sentence.
        let para = tree.add_boxed(Box::new(prose(lit!(LONG))));
        let _root = tree.add_boxed(Box::new(VStack::new().add_child(para)));
        // Wider than any measure: a maximised window on a large display.
        tree.layout(SizeProposal::exact(1450.0, 900.0));

        let b = tree.bounds(para);
        assert!(
            b.width <= super::PROSE_MEASURE + 0.5,
            "the paragraph took {}px of a 1450px pane; it should stop at {}",
            b.width,
            super::PROSE_MEASURE
        );
        assert!(
            b.height > 0.0,
            "the paragraph laid out to nothing, so the width assertion above proves nothing"
        );
    }

    /// And it is the cap doing it, not the sentence running out.
    ///
    /// The same paragraph without the cap takes the whole column. Without this, the test
    /// above would pass just as well on a sentence that happened to be short.
    ///
    /// ⚠ This asserts **width**, not line count, and that is a limit of the harness rather
    /// than a choice. A bare `WidgetTree` has no typesetter attached, so line breaking is
    /// never exercised here and both paragraphs report a single line whatever width they
    /// are given. What the cap does to the *text* has to be seen with a real text backend.
    #[test]
    fn the_same_paragraph_uncapped_takes_the_whole_pane() {
        use super::{note, prose};
        use teksilo::core::widget_tree::WidgetTree;
        use teksilo::prelude::{SizeProposal, lit};
        use teksilo::widgets::VStack;

        let mut tree = WidgetTree::new();
        let capped = tree.add_boxed(Box::new(prose(lit!(LONG))));
        let bare = tree.add_boxed(Box::new(note(lit!(LONG))));
        let _root = tree.add_boxed(Box::new(VStack::new().add_child(capped).add_child(bare)));
        tree.layout(SizeProposal::exact(1450.0, 900.0));

        let (c, u) = (tree.bounds(capped), tree.bounds(bare));
        assert!(
            u.width > super::PROSE_MEASURE,
            "the uncapped paragraph was {}px wide, so this pane is not wide enough to \
             demonstrate anything",
            u.width
        );
        assert!(
            c.width < u.width,
            "capped width {} is not less than uncapped width {}: the cap did nothing",
            c.width,
            u.width
        );
    }

    /// The measure is a reading constraint, not a resizing one: a pane narrower than the
    /// measure must still get the whole of its own width rather than a cropped column.
    #[test]
    fn a_narrow_pane_keeps_its_full_width() {
        use super::prose;
        use teksilo::core::widget_tree::WidgetTree;
        use teksilo::prelude::{SizeProposal, lit};
        use teksilo::widgets::VStack;

        let mut tree = WidgetTree::new();
        let para = tree.add_boxed(Box::new(prose(lit!(LONG))));
        let _root = tree.add_boxed(Box::new(VStack::new().add_child(para)));
        tree.layout(SizeProposal::exact(320.0, 900.0));
        assert!(
            tree.bounds(para).width <= 320.0,
            "a cap must never make a child wider than the space it was offered"
        );
    }

    /// Before the companion `count_words` operation has landed, the figure is `None` —
    /// Shape must still lay out (as "still counting"), never panic on the missing value.
    #[test]
    fn shape_lays_out_while_the_footnote_figure_is_still_pending() {
        use teksilo::core::widget_tree::WidgetTree;
        use teksilo::prelude::{Signal, SizeProposal};

        let d = dto(vec![scene(1, "Chapter 1", "")]);
        let mut tree = WidgetTree::new();
        let id = tree.add_boxed(Box::new(super::shape_view(
            &d,
            Signal::new(true),
            Signal::new(None),
            None,
        )));
        tree.layout(SizeProposal::exact(700.0, 900.0));
        assert!(tree.bounds(id).height > 0.0, "the pane laid out to nothing");
    }

    /// The footnote line does not depend on `scenes_of` at all — a Book folder with a
    /// note on its own synopsis and not one scene written yet still has a real figure
    /// to show, so the "no scenes yet" early return must still carry it (and still lay
    /// out rather than panicking on a `VStack` built from two different branches).
    #[test]
    fn the_footnote_line_survives_the_no_scenes_early_return() {
        use teksilo::core::widget_tree::WidgetTree;
        use teksilo::prelude::{Signal, SizeProposal};

        let d = dto(vec![]);
        let mut tree = WidgetTree::new();
        let id = tree.add_boxed(Box::new(super::shape_view(
            &d,
            Signal::new(true),
            Signal::new(Some(12)),
            None,
        )));
        tree.layout(SizeProposal::exact(700.0, 900.0));
        assert!(tree.bounds(id).height > 0.0, "the pane laid out to nothing");
    }
}

#[cfg(test)]
mod category_registry_tests {
    use super::*;

    // Scoped to ids this test owns, never to `all_categories().len()`: the registry is
    // per-thread but `cargo test` may still run these on one thread in any order, and a
    // count assertion would couple them to each other.
    fn spec(id: &str) -> AnalysisCategorySpec {
        AnalysisCategorySpec {
            id: id.to_string(),
            label: Rc::new(|| lit!("Style".to_string())),
            view: Rc::new(|_vm, _dto| Box::new(TextWidget::new(lit!("body".to_string())))),
        }
    }

    fn ids() -> Vec<String> {
        all_categories().into_iter().map(|c| c.id).collect()
    }

    /// The built-in leads the list, whatever else has registered.
    ///
    /// Shape is the one this application ships and the one a writer opening Analysis
    /// expects to land on, so it holds position 0 — the segment `SegmentedControl` selects
    /// when nothing has been chosen yet.
    #[test]
    fn the_built_ins_are_present_and_first() {
        let ids = ids();
        assert_eq!(
            &ids[..2],
            &["shape", "arrivals"],
            "the built-in order is what every existing writer's muscle memory keys on, \
             and Shape stays at position 0 — the segment the bar selects before anything \
             has been chosen"
        );
    }

    /// ⚠ **The bar is now at its ceiling with this edition's three registrations.**
    /// `SegmentedControl` documents five segments; two built-ins plus three
    /// contributed is exactly five. A sixth category is not refused — see
    /// `register_category` for why counting is the wrong place to draw that line
    /// — but it is the point at which the *control* has to change, and this test
    /// is where somebody adding a third built-in will be told so.
    #[test]
    fn the_built_ins_leave_room_for_three_registrations() {
        assert_eq!(
            AnalysisCategory::ALL.len(),
            2,
            "a third built-in leaves room for only two contributed categories, and this \
             edition already contributes three"
        );
    }

    /// A registered category joins the list, after the built-ins.
    #[test]
    fn a_registered_category_lands_after_the_built_ins() {
        let _h = register_category("test.after", spec("ext.after")).expect("register");
        let ids = ids();
        let pos = ids.iter().position(|i| i == "ext.after").expect("present");
        assert!(
            pos >= AnalysisCategory::ALL.len(),
            "an extension must not displace a built-in from its position"
        );
    }

    /// The invariant a `const _: () = assert!(ALL.len() == N)` could never have guarded.
    ///
    /// Such an assert could only ever see the built-ins, so it said nothing about a
    /// registered category — the case where a bar/switcher mismatch would actually be
    /// introduced, and the only case there is now that the bar is mostly contributed.
    /// Both are built from one pass over this list, so what has to hold is that the list
    /// itself is coherent: every entry has a label and a body, and no two share an id.
    #[test]
    fn the_bar_and_the_switcher_agree() {
        let _h = register_category("test.pairing", spec("ext.pairing")).expect("register");
        let cats = all_categories();

        let mut seen = std::collections::HashSet::new();
        for c in &cats {
            assert!(
                seen.insert(c.id.clone()),
                "duplicate category id '{}' — the bar would show two segments the \
                 switcher cannot tell apart",
                c.id
            );
            // A segment is built from `label` and its pane from `view`; a spec missing
            // either would put a labelled segment over someone else's body.
            let _ = (c.label)();
        }
        assert!(
            cats.len() > AnalysisCategory::ALL.len(),
            "the registered category must actually be in the list under test"
        );
    }

    /// A built-in id cannot be claimed.
    #[test]
    fn a_built_in_id_is_refused() {
        let err = register_category("test.shadow", spec("shape")).expect_err("must refuse");
        assert!(err.contains("built-in"), "unhelpful message: {err}");
    }

    /// Two extensions cannot claim one id, and the error names the holder.
    #[test]
    fn a_taken_id_is_refused_and_names_its_holder() {
        let _first = register_category("test.one", spec("ext.contested")).expect("register");
        let err = register_category("test.two", spec("ext.contested")).expect_err("must refuse");
        assert!(err.contains("test.one"), "unhelpful message: {err}");
    }

    /// Dropping the handle removes the category; re-registering replaces.
    #[test]
    fn drop_unregisters_and_re_registration_replaces() {
        {
            let _a = register_category("test.scoped", spec("ext.first")).expect("a");
            assert!(ids().contains(&"ext.first".to_string()));
            let _b = register_category("test.scoped", spec("ext.second")).expect("b");
            assert!(ids().contains(&"ext.second".to_string()));
            assert!(
                !ids().contains(&"ext.first".to_string()),
                "re-registering a namespace must replace, not stack"
            );
        }
        assert!(
            !ids().contains(&"ext.second".to_string()),
            "a dropped handle must leave no category behind"
        );
    }
}
