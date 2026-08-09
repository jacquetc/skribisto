// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The Analysis segment of a Book container: four views over one `analyze_book` result.
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
    ActivateOn, Button, ButtonVariant, Expand, FixedSize, HStack, Padding, ScrollArea,
    ScrollBarMode, Segment, SegmentedControl, Spacer, StandardTreeItem, Switcher, TextWidget,
    Toggle, TreeRow, TreeView, VStack,
};
use teksilo_charts::BarChart;
use teksilo_charts::reference_line::ReferenceLine;

use frontend::analysis_management::{
    BookAnalysisResultDto, BookDiversity, DriftRow, DriftRows, DuplicateRow, DuplicateRows,
    EchoRow, EchoRows, SceneAnalyses, SceneAnalysis,
};

use super::{Boxed, ContentTab};
use super::shared::{CHART_HEIGHT, STRIP_HEIGHT, wide_chart};
use crate::intents::AppIntent;
use crate::models::RepetitionNode;
use crate::view_models::{AnalysisCategory, AnalysisState, AnalysisViewModel};

/// The bar and its `Switcher` are still matched by **position** — that is
/// `SegmentedControl`'s contract — but neither is written out by hand any more: both are
/// built from one pass over [`all_categories`], so a category cannot exist as a segment
/// without its body or land at a different index in the two. The compile-time
/// `ALL.len() == 4` assert this file used to carry was guarding a hazard that the shared
/// list removes by construction; `the_bar_and_the_switcher_agree` pins it at runtime for
/// the registered case, which no `const` assert could see.

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
    pub label: Rc<dyn Fn() -> LocalizedString>,
    /// Builds this category's body. Receives the view-model and the finished analysis, so
    /// a registered category can read the same measurements the built-ins do — or ignore
    /// them entirely and render from its own store.
    pub view: Rc<dyn Fn(&AnalysisViewModel, &BookAnalysisResultDto) -> Box<dyn Widget>>,
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
/// ⚠ `SegmentedControl` has a documented five-segment ceiling and the four built-ins
/// already sit just under it. A registered category takes the bar past that, and past
/// roughly six the control stops being a segmented bar at all — which is what
/// `TabWidget::vertical()` is for. Registration does not refuse on count, because refusing
/// the *fifth* category would be an arbitrary line; the ceiling is a design constraint on
/// the control, and the control is the thing that has to change.
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

/// The four categories this application ships, as specs.
fn builtin_categories() -> Vec<AnalysisCategorySpec> {
    AnalysisCategory::ALL
        .iter()
        .map(|c| {
            let c = *c;
            AnalysisCategorySpec {
                id: c.id().to_string(),
                label: Rc::new(move || c.label()),
                view: Rc::new(move |vm, dto| match c {
                    AnalysisCategory::Shape => Box::new(shape_view(
                        dto,
                        vm.ignore_empty(),
                        vm.footnote_words(),
                    )),
                    AnalysisCategory::Repetition => Box::new(repetition_view(vm, dto)),
                    AnalysisCategory::Synopsis => Box::new(synopsis_view(dto)),
                    AnalysisCategory::Voice => Box::new(voice_view(dto)),
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

/// How many rows a finder list shows before it stops.
///
/// A punch-list past this length is not a punch-list, and the ranking already puts the most
/// surprising findings first. The count of what was left out is shown rather than silently
/// dropped — a truncated list that does not say so reads as "that's all of them".
const MAX_ROWS: usize = 50;

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
        let mut row = HStack::new()
            .spacing(8.0)
            .child(
                TextWidget::new(tr!(analysis_scope_book()))
                    .style(TextStyleRole::Small)
                    .color(TextRole::Secondary),
            )
            .child(Spacer::new());

        // Staleness is a plain statement of fact, not a warning: the result is still the
        // one that was computed, and the writer decides whether it is worth re-running.
        if self.vm.is_stale() {
            row = row.child(
                TextWidget::new(tr!(analysis_stale()))
                    .style(TextStyleRole::Tiny)
                    .color(TextRole::Secondary),
            );
        }
        row.child(
            Button::new(tr!(analysis_run()))
                .variant(ButtonVariant::Plain)
                .enabled(!running)
                .on_activate_fn(move |_| vm.run()),
        )
    }

    fn category_bar(&self, cats: &[AnalysisCategorySpec]) -> impl Widget {
        let mut bar = SegmentedControl::new(self.vm.category());
        for c in cats {
            bar = bar.segment(Segment::new((c.label)()));
        }
        bar
    }

    /// The `Switcher`'s children sit at the same positions as the bar's segments — matched
    /// by index, not by name, which is the trap `tabs.rs` documents at length. Both are
    /// built from the **same slice** in the same order, by the same caller, which is what
    /// makes that pairing true rather than merely intended.
    fn categories(&self, cats: &[AnalysisCategorySpec], dto: &BookAnalysisResultDto) -> impl Widget {
        let mut sw = Switcher::new(self.vm.category());
        for c in cats {
            sw = sw.child_boxed(scrolled_boxed((c.view)(&self.vm, dto)));
        }
        sw
    }
}

/// One category's body, in the same shell Pace uses.
///
/// A stats dashboard, not a prose column: it fills the viewport's width rather than hugging
/// a centred reading column, with horizontal breathing room at the sides. Matching Pace
/// matters beyond consistency — both panes show charts of the same manuscript, and two
/// different gutters made the same book look like two different shapes.
fn scrolled(inner: impl Widget + 'static) -> Box<dyn Widget> {
    scrolled_boxed(Box::new(inner))
}

/// [`scrolled`] for an already-boxed body — what a category spec hands back.
fn scrolled_boxed(inner: Box<dyn Widget>) -> Box<dyn Widget> {
    Box::new(ScrollArea::new().child(Padding::symmetric(0.0, 24.0).child(Boxed::new(inner))))
}

fn note(text: impl Into<LocalizedString>) -> impl Widget {
    TextWidget::new(text)
        .style(TextStyleRole::Small)
        .color(TextRole::Secondary)
}

fn heading(text: impl Into<LocalizedString>) -> impl Widget {
    TextWidget::new(text)
        .style(TextStyleRole::Tiny)
        .color(TextRole::Secondary)
}

/// The leading paragraph of a section: what the reader is looking at, in plain language.
///
/// Body size rather than [`note`]'s small, because this is the text that has to be read for
/// the figures under it to mean anything — giving it a caveat's typography would say the
/// opposite of what it is for.
fn explainer(text: impl Into<LocalizedString>) -> impl Widget {
    TextWidget::new(text).color(TextRole::Secondary)
}

/// Findings, indented under the scene they belong to.
fn indented(inner: impl Widget + 'static) -> impl Widget {
    Padding::new(0.0, 0.0, 6.0, 16.0).child(inner)
}

/// The scene a group of findings belongs to, as a control that opens it.
///
/// This is the difference between a report and a punch list. Every finding here names a
/// place in the manuscript, and the only useful next move is to go and look at it — so the
/// scene's name is the way there, not a label printed above the numbers.
fn scene_link(item_id: u64, title: String) -> impl Widget {
    let open = title.clone();
    Button::new(lit!(title))
        .variant(ButtonVariant::Link)
        .on_activate_fn(move |ctx| {
            ctx.send_intent(crate::intents::AppIntent::OpenItem {
                item_id,
                title: open.clone(),
            })
        })
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
    HStack::new()
        .child(Toggle::new(ignore_empty).label(tr!(analysis_ignore_empty())))
        .child(Spacer::new())
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
) -> impl Widget {
    // Read once, up front, so every return path below — including the two early "no
    // scenes"/"all texts empty" ones — carries the same footnote line. It does not
    // depend on `scenes_of` at all: a Book folder with a note on its own synopsis and
    // not one scene written yet still has a real footnote figure to show.
    let footnote_section = footnote_words_section(footnote_words.get());
    let all = scenes_of(dto);
    if all.is_empty() {
        return VStack::new()
            .child(note(tr!(analysis_no_scenes())))
            .child(footnote_section);
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
        return VStack::new()
            .spacing(10.0)
            .child(empty_toggle(ignore_empty))
            .child(note(tr!(analysis_all_texts_empty())))
            .child(footnote_section);
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
            title,
            words,
            dialogue,
            ..
        } = s
        else {
            continue;
        };
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

    let mut col = VStack::new()
        .spacing(10.0)
        .child(empty_toggle(ignore_empty));
    if hiding && hidden > 0 {
        col = col.child(note(tr!(analysis_empty_hidden(count = hidden as i64))));
    }
    col = col
        .child(heading(tr!(analysis_words_per_scene())))
        .child(wide_chart(
            points.len(),
            CHART_HEIGHT,
            BarChart::new(ChartModel::from_series_vec(vec![
                ChartSeries::new(tr!(analysis_words_per_scene()).resolve_now()).data(points),
            ]))
            .grid(true)
            .legend(false)
            // The comparison the panel is built on, drawn rather than described. A caption
            // saying "the median is 2,495" asks the reader to hold a number in their head
            // and eyeball every bar against it.
            .reference_line(ReferenceLine::new(
                median as f32,
                tr!(analysis_median_line(count = median.round() as i64)),
            )),
        ))
        .child(note(tr!(analysis_median_words(
            count = median.round() as i64
        ))));

    if dialogue_points.is_empty() {
        // Not "0% dialogue" — the language has no curated convention, which is a different
        // and honest statement.
        col = col
            .child(heading(tr!(analysis_dialogue())))
            .child(note(tr!(analysis_dialogue_unsupported())));
    } else {
        col = col
            .child(heading(tr!(analysis_dialogue())))
            .child(wide_chart(
                dialogue_points.len(),
                STRIP_HEIGHT,
                BarChart::new(ChartModel::from_series_vec(vec![
                    ChartSeries::new(tr!(analysis_dialogue()).resolve_now()).data(dialogue_points),
                ]))
                .grid(true)
                .legend(false),
            ));
    }
    col.child(footnote_section)
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
fn footnote_words_section(value: Option<i64>) -> impl Widget {
    let line = match value {
        Some(n) => note(tr!(analysis_footnote_words_count(count = n))),
        None => note(tr!(analysis_footnote_words_pending())),
    };
    VStack::new()
        .spacing(6.0)
        .child(heading(tr!(analysis_footnote_words())))
        .child(line)
}

// ── Repetition ────────────────────────────────────────────────────────────────

/// Where each scene sits, and what it is called — the context every finding needs.
///
/// A flat list of `"glanced" appears 4 times` says nothing a writer can act on: they cannot
/// go and look at it. Findings are grouped under the scene they were found in, and scenes
/// under their chapter where that adds anything.
struct SceneIndex {
    /// item id → (scene title, chapter title)
    by_id: std::collections::HashMap<u64, (String, String)>,
    /// Scene ids in stream order, so groups read in the order the book does rather than in
    /// whatever order the findings happened to rank.
    order: Vec<u64>,
}

impl SceneIndex {
    fn build(dto: &BookAnalysisResultDto) -> Self {
        let mut by_id = std::collections::HashMap::new();
        let mut order = Vec::new();
        for s in scenes_of(dto) {
            if let SceneAnalysis::Measured {
                item_id,
                title,
                chapter_title,
                ..
            } = s
            {
                by_id.insert(*item_id, (title.clone(), chapter_title.clone()));
                order.push(*item_id);
            }
        }
        Self { by_id, order }
    }

    fn title(&self, id: u64) -> String {
        self.by_id
            .get(&id)
            .map(|(t, _)| t.clone())
            .unwrap_or_default()
    }

    /// The chapter heading to draw above a scene, or `None` when it would say nothing.
    ///
    /// Suppressed when the chapter *is* the scene — a flat `Item/ChapterScene` is its own
    /// chapter, so a heading above it would simply repeat the row underneath.
    fn chapter_of(&self, id: u64) -> Option<&str> {
        let (title, chapter) = self.by_id.get(&id)?;
        (!chapter.is_empty() && chapter != title).then_some(chapter.as_str())
    }
}

/// Findings for one scene, in the book's own order.
fn grouped_by_scene<T: Clone>(index: &SceneIndex, rows: &[(u64, T)]) -> Vec<(u64, Vec<T>)> {
    let mut out: Vec<(u64, Vec<T>)> = Vec::new();
    for id in &index.order {
        let mine: Vec<T> = rows
            .iter()
            .filter(|(owner, _)| owner == id)
            .map(|(_, r)| r.clone())
            .collect();
        if !mine.is_empty() {
            out.push((*id, mine));
        }
    }
    out
}

/// Repeated words, as a tree of texts you can open.
///
/// The echoes half is a `TreeView` — one row per text that echoes, its words beneath,
/// **collapsed**. The flat list this replaces answered "what did you find" and left the
/// writer to read every finding to learn which scenes were worth opening; the parent rows
/// answer "where do I look" in one screen, and the numbers stay one click away.
///
/// Near-duplicate scenes stay a flat list below it, and deliberately: a duplicate pair
/// belongs to *two* texts at once, so nesting it under one of them would misstate what was
/// found. It already names both, which is the context that finding needs.
fn repetition_view(vm: &AnalysisViewModel, dto: &BookAnalysisResultDto) -> impl Widget {
    let index = SceneIndex::build(dto);

    let echoes: Vec<(u64, (String, i64, i64))> = match &dto.echoes {
        EchoRows::Found(rows) => rows
            .iter()
            .filter_map(|r| match r {
                EchoRow::Found {
                    item_id,
                    word,
                    occurrences,
                    closest_gap,
                    ..
                } => Some((*item_id, (word.clone(), *occurrences, *closest_gap))),
                EchoRow::Empty => None,
            })
            .collect(),
        EchoRows::Empty => vec![],
    };
    let duplicates: Vec<&DuplicateRow> = match &dto.duplicates {
        DuplicateRows::Found(rows) => rows.iter().collect(),
        DuplicateRows::Empty => vec![],
    };

    // Book order for the texts, the analyser's own rank order for the words inside each —
    // the first is where the reader is, the second is what is worth reading first.
    let tree_model = vm.repetition_tree();
    tree_model.set_groups(
        grouped_by_scene(&index, &echoes)
            .into_iter()
            .map(|(item_id, found)| (item_id, index.title(item_id), found))
            .collect(),
    );

    let mut col = VStack::new()
        .spacing(6.0)
        .child(heading(tr!(analysis_echoes())))
        // "4 times, 12 words apart at the closest" is a measurement, not a sentence. What
        // the writer needs first is what an echo *is* here, and that the list is already
        // filtered down to words worth hearing about.
        .child(explainer(tr!(analysis_echoes_explainer())));
    if echoes.is_empty() {
        col = col.child(note(tr!(analysis_no_echoes())));
    } else {
        // No `MAX_ROWS` cap here, unlike every other section. The cap exists because a flat
        // list of a thousand text widgets is both unusable and expensive; a collapsed tree
        // is neither — it shows one row per text and the `TreeView` realises only what is
        // on screen. Truncating it would hide whole scenes behind a "N more not shown"
        // note, which is precisely the "that's all of them" misreading the note was added
        // to prevent.
        let activate = tree_model.clone();
        let tree = TreeView::from_source_keyed(
            tree_model.source(),
            vm.repetition_selection(),
            move |node: &RepetitionNode, row: &TreeRow, selected: bool| {
                let item = StandardTreeItem::new(lit!(node.label().to_string()))
                    .depth(row.depth)
                    .has_children(row.has_children)
                    .is_expanded(row.is_expanded)
                    .selected(selected)
                    .on_toggle_rc(row.toggle_callback());
                let item = match node {
                    RepetitionNode::Item { words, .. } => item
                        .trailing_slot(
                            TextWidget::new(lit!(words.to_string()))
                                .style(TextStyleRole::Small)
                                .color(TextRole::Secondary),
                        )
                        .tooltip(tr!(analysis_repetition_text_tooltip(count = *words as i64))),
                    RepetitionNode::Word {
                        word,
                        occurrences,
                        closest_gap,
                        ..
                    } => item
                        .trailing_slot(numbers(*occurrences, *closest_gap))
                        .tooltip(tr!(analysis_repetition_word_tooltip(
                            word = word.clone(),
                            count = *occurrences,
                            gap = *closest_gap
                        ))),
                };
                Box::new(item) as Box<dyn Widget>
            },
        )
        .auto_item_height(28.0)
        .scroll_bar_style(ScrollBarMode::Overlay)
        // A click on the row opens its text; the disclosure triangle is what expands. Both
        // gestures are wanted here and would otherwise fight — `row_click_expands` on a
        // parent would make "show me the words" and "take me there" the same click.
        .row_click_expands(false)
        .activate_on(ActivateOn::SingleClick)
        .on_activate(move |idx, ctx| {
            // A word row opens the text it was found in — the same destination as its
            // parent, because the finding is only actionable in the prose.
            if let Some(key) = activate.source().key_at(idx)
                && let Some(node) = activate.node_of(&key)
                && let Some(title) = activate.title_of(node.item_id())
            {
                ctx.send_intent(AppIntent::OpenItem {
                    item_id: node.item_id(),
                    title,
                });
            }
        });
        // The tree sits inside the pane's own vertical `ScrollArea`, which proposes an
        // unbounded height and asks each child what it wants — and a scrollable answers
        // with a constant. Left to that, the tree collapses into a small window with its
        // own scrollbar inside the page's. A fixed height sized to the collapsed report
        // keeps one scrollbar on the page. (The same trap the Shape charts hit.)
        let rows = tree_model.source().visible_count().max(1);
        col = col.child(
            FixedSize::new()
                .height((rows as f32 * TREE_ROW_HEIGHT).clamp(TREE_ROW_HEIGHT, TREE_MAX_HEIGHT))
                .child(tree),
        );
    }

    col = col
        .child(heading(tr!(analysis_similar_scenes())))
        .child(explainer(tr!(analysis_similar_explainer())));
    if duplicates.is_empty() {
        col = col.child(note(tr!(analysis_no_similar_scenes())));
    }
    // A near-duplicate pair belongs to two scenes at once, so it is not grouped under
    // either — it already names both, which is the context this finding needs.
    for row in duplicates.iter().take(MAX_ROWS) {
        if let DuplicateRow::Found {
            a_title,
            b_title,
            containment,
            ..
        } = row
        {
            col = col.child(TextWidget::new(tr!(analysis_similar_row(
                a = a_title.clone(),
                b = b_title.clone(),
                percent = (containment * 100.0).round() as i64
            ))));
        }
    }
    if duplicates.len() > MAX_ROWS {
        col = col.child(note(tr!(analysis_more_rows(
            count = (duplicates.len() - MAX_ROWS) as i64
        ))));
    }
    col
}

/// One row's height, and the tallest the tree may grow before it scrolls itself.
const TREE_ROW_HEIGHT: f32 = 28.0;
const TREE_MAX_HEIGHT: f32 = 520.0;

/// The two numbers on a word row: how many uses read as an echo, and how close the
/// closest two are.
///
/// Right-aligned in fixed columns rather than run together in a sentence, so a reader
/// scanning down the tree compares like with like — which is the whole reason they are
/// numbers here and prose in the tooltip.
fn numbers(occurrences: i64, closest_gap: i64) -> impl Widget {
    // Takes a `LocalizedString` rather than a resolved `String`: resolving here would
    // freeze the row in the locale it was built in, and the gap cell is translated.
    let cell = |text: LocalizedString| {
        FixedSize::new().width(44.0).child(
            HStack::new().child(Spacer::new()).child(
                TextWidget::new(text)
                    .style(TextStyleRole::Small)
                    .color(TextRole::Secondary),
            ),
        )
    };
    HStack::new()
        .spacing(4.0)
        .child(cell(lit!(occurrences.to_string())))
        .child(cell(tr!(analysis_repetition_gap_short(gap = closest_gap))))
}

// ── Synopsis ──────────────────────────────────────────────────────────────────

/// Scenes whose prose tracks their synopsis less closely than the rest of this book's do.
///
/// The finding names the *missing terms*, not the score: "your synopsis mentions the locket
/// and the prose does not" is something a writer can act on, where a coverage percentage is
/// not.
fn synopsis_view(dto: &BookAnalysisResultDto) -> impl Widget {
    let drifts: Vec<&DriftRow> = match &dto.drifts {
        DriftRows::Found(rows) => rows.iter().collect(),
        DriftRows::Empty => vec![],
    };
    let any_synopsis = scenes_of(dto).iter().any(
        |s| matches!(s, SceneAnalysis::Measured { synopsis_words, .. } if *synopsis_words > 0),
    );

    let mut col = VStack::new()
        .spacing(6.0)
        .child(heading(tr!(analysis_synopsis_drift())));
    if !any_synopsis {
        // Distinct from "no findings": there is nothing to compare against, which is not
        // the same as everything matching.
        return col.child(note(tr!(analysis_no_synopses())));
    }
    if drifts.is_empty() {
        return col.child(note(tr!(analysis_no_drift())));
    }
    for row in drifts.iter().take(MAX_ROWS) {
        if let DriftRow::Found { title, missing, .. } = row {
            col = col.child(TextWidget::new(tr!(analysis_drift_row(
                title = title.clone(),
                terms = missing.join(", ")
            ))));
        }
    }
    if drifts.len() > MAX_ROWS {
        col = col.child(note(tr!(analysis_more_rows(
            count = (drifts.len() - MAX_ROWS) as i64
        ))));
    }
    col
}

// ── Voice ─────────────────────────────────────────────────────────────────────

/// Lexical diversity for the book as a whole.
///
/// Shown with its sample size, and withheld entirely below the reliability floor: a
/// diversity figure over a few hundred words is a coin toss wearing a decimal point.
fn voice_view(dto: &BookAnalysisResultDto) -> impl Widget {
    let mut col = VStack::new()
        .spacing(6.0)
        .child(heading(tr!(analysis_vocabulary())))
        // What the figure is, before the figure — a bare 0.742 on a page teaches the reader
        // nothing except that something was measured.
        .child(explainer(tr!(analysis_vocabulary_explainer())));
    match &dto.diversity {
        BookDiversity::Measured {
            words,
            distinct_words,
            mattr,
            reliable,
            ..
        } => {
            col = col.child(note(tr!(analysis_words_measured(
                words = *words,
                distinct = *distinct_words
            ))));
            match (reliable, mattr) {
                (true, Some(m)) => {
                    col = col
                        .child(TextWidget::new(tr!(analysis_mattr(
                            value = format!("{:.3}", m)
                        ))))
                        // The two ends of the scale, so the number has somewhere to sit.
                        // Deliberately not a verdict: there is no good value.
                        .child(note(tr!(analysis_mattr_scale())));
                }
                _ => {
                    col = col.child(note(tr!(analysis_not_enough_text())));
                }
            }
        }
        BookDiversity::Empty => {
            col = col.child(note(tr!(analysis_not_enough_text())));
        }
    }
    col.child(note(tr!(analysis_vocabulary_caveat())))
}

#[cfg(test)]
mod tests {

    /// The tint threshold is the shared median, not a local one — pinned here because the
    /// property it buys is what this panel depends on: a single very long chapter must not
    /// drag the comparison line up with it.
    use super::SceneIndex;
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
            synopsis_words: 0,
            synopsis_coverage: None,
        }
    }

    fn dto(scenes: Vec<SceneAnalysis>) -> BookAnalysisResultDto {
        BookAnalysisResultDto {
            scenes: SceneAnalyses::Measured(scenes),
            ..Default::default()
        }
    }

    /// A flat `Item/ChapterScene` is its own chapter, so a heading above it would just
    /// repeat the row underneath. Suppressed — which is what the whole bundled example
    /// looks like.
    #[test]
    fn a_scene_that_is_its_own_chapter_gets_no_redundant_heading() {
        let index = SceneIndex::build(&dto(vec![scene(1, "Chapter 6", "Chapter 6")]));
        assert_eq!(index.chapter_of(1), None);
    }

    #[test]
    fn a_scene_under_a_real_chapter_is_grouped_under_it() {
        let index = SceneIndex::build(&dto(vec![scene(1, "The attic", "Chapter 6")]));
        assert_eq!(index.chapter_of(1), Some("Chapter 6"));
        assert_eq!(index.title(1), "The attic");
    }

    #[test]
    fn a_scene_with_no_enclosing_chapter_is_not_grouped() {
        let index = SceneIndex::build(&dto(vec![scene(1, "Prologue", "")]));
        assert_eq!(index.chapter_of(1), None);
    }

    /// Groups follow the book's order, not the findings' ranking — a report that jumps
    /// about the manuscript is harder to work through than one that reads front to back.
    #[test]
    fn groups_follow_stream_order_not_finding_order() {
        let index = SceneIndex::build(&dto(vec![scene(10, "First", ""), scene(20, "Second", "")]));
        // Findings arrive worst-first, i.e. the later scene leads.
        let rows = vec![(20u64, "b"), (10u64, "a"), (20u64, "c")];
        let grouped = super::grouped_by_scene(&index, &rows);
        assert_eq!(grouped[0].0, 10, "the earlier scene is shown first");
        assert_eq!(grouped[1].0, 20);
        assert_eq!(
            grouped[1].1,
            vec!["b", "c"],
            "its own findings keep their ranking"
        );
    }

    #[test]
    fn a_scene_with_no_findings_gets_no_group() {
        let index = SceneIndex::build(&dto(vec![scene(10, "First", ""), scene(20, "Second", "")]));
        let grouped = super::grouped_by_scene(&index, &[(20u64, "only")]);
        assert_eq!(grouped.len(), 1);
        assert_eq!(grouped[0].0, 20);
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

    /// The Repetition pane mounts, and its tree opens **closed**.
    ///
    /// The pane had no widget test at all — every test above it exercises a pure helper,
    /// so nothing would have noticed the tree failing to build or coming up expanded.
    /// "Collapsed by default" is the whole reason it is a tree, and it is one flag away
    /// from being wrong (`set_expand_new_nodes(true)`, which the binder tree does set).
    #[test]
    fn the_repetition_tree_mounts_with_every_text_closed() {
        use crate::app_ids::AppIds;
        use crate::view_models::AnalysisViewModel;
        use teksilo::prelude::{Signal, SizeProposal};
        use frontend::analysis_management::{EchoRow, EchoRows};

        let ctx = std::rc::Rc::new(frontend::AppContext::new());
        let vm = AnalysisViewModel::new(ctx.clone(), AppIds::new(), 1, Signal::new(0));

        let echo = |item_id: u64, word: &str| EchoRow::Found {
            item_id,
            word: word.to_string(),
            occurrences: 3,
            closest_gap: 12,
            score: 1.0,
            first_at: 0,
            first_len: 0,
        };
        let mut d = dto(vec![scene(10, "First", ""), scene(20, "Second", "")]);
        d.echoes = EchoRows::Found(vec![
            echo(10, "glanced"),
            echo(10, "suddenly"),
            echo(20, "carnelian"),
        ]);

        let mut tree = crate::test_support::tree_with_events(&ctx);
        let id = tree.add_boxed(Box::new(super::repetition_view(&vm, &d)));
        tree.layout(SizeProposal::exact(700.0, 900.0));
        assert!(tree.bounds(id).height > 0.0, "the pane laid out to nothing");

        // Two texts echo, so two rows — and only two: the three findings underneath them
        // are hidden until a row is opened.
        assert_eq!(
            vm.repetition_tree().source().visible_count(),
            2,
            "the tree came up expanded — every finding is visible without a click"
        );
        assert_eq!(vm.repetition_tree().finding_count(), 3);
    }

    /// Expanding one text reveals its own findings and nobody else's.
    #[test]
    fn opening_one_text_reveals_only_its_own_words() {
        use teksilo::data::TreeDataSource;

        use crate::models::RepetitionTreeKey;

        let model = crate::models::RepetitionTreeModel::new();
        model.set_groups(vec![
            (
                10,
                "First".into(),
                vec![("glanced".into(), 3, 12), ("suddenly".into(), 2, 40)],
            ),
            (20, "Second".into(), vec![("carnelian".into(), 2, 8)]),
        ]);
        let src = model.source();
        assert_eq!(src.visible_count(), 2, "closed to start");

        src.set_expanded(&RepetitionTreeKey::Item(10), true);
        assert_eq!(
            src.visible_count(),
            4,
            "two texts plus the two words of the one that was opened"
        );
    }

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
        )));
        tree.layout(SizeProposal::exact(700.0, 900.0));
        assert!(tree.bounds(id).height > 0.0, "the pane laid out to nothing");
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

    /// The four built-ins are the list until something registers.
    #[test]
    fn the_built_ins_are_present_and_first() {
        let ids = ids();
        assert_eq!(
            &ids[..4],
            &["shape", "repetition", "synopsis", "voice"],
            "the built-in order is what every existing writer's muscle memory keys on"
        );
    }

    /// A registered category joins the list, after the built-ins.
    #[test]
    fn a_registered_category_lands_after_the_built_ins() {
        let _h = register_category("test.after", spec("ext.after")).expect("register");
        let ids = ids();
        let pos = ids.iter().position(|i| i == "ext.after").expect("present");
        assert!(
            pos >= 4,
            "an extension must not displace a built-in from its position"
        );
    }

    /// The invariant the deleted `const _: () = assert!(ALL.len() == 4)` used to guard.
    ///
    /// It could only ever see the built-ins, so it said nothing about a registered
    /// category — the case where a bar/switcher mismatch would actually be introduced.
    /// Both are now built from one pass over this list, so what has to hold is that the
    /// list itself is coherent: every entry has a label and a body, and no two share an id.
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
            cats.len() >= 5,
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
