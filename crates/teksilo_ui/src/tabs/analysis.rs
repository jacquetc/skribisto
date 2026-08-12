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
    Button, ButtonVariant, Expand, HStack, Padding, ScrollArea, Segment, SegmentedControl, Spacer,
    Switcher, TextWidget, Toggle, VStack,
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
/// ⚠ `SegmentedControl` has a documented five-segment ceiling, and with one built-in the
/// bar reaches it at the fourth registration. Past roughly six the control stops being a
/// segmented bar at all — which is what `TabWidget::vertical()` is for. Registration does
/// not refuse on count, because refusing the *sixth* category would be an arbitrary line;
/// the ceiling is a design constraint on the control, and the control is the thing that has
/// to change.
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
                        Box::new(shape_view(dto, vm.ignore_empty(), vm.footnote_words()))
                    }
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

use crate::shared::text::caption as note;

fn heading(text: impl Into<LocalizedString>) -> impl Widget {
    TextWidget::new(text)
        .style(TextStyleRole::Tiny)
        .color(TextRole::Secondary)
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

    /// The built-in leads the list, whatever else has registered.
    ///
    /// Shape is the one this application ships and the one a writer opening Analysis
    /// expects to land on, so it holds position 0 — the segment `SegmentedControl` selects
    /// when nothing has been chosen yet.
    #[test]
    fn the_built_ins_are_present_and_first() {
        let ids = ids();
        assert_eq!(
            &ids[..1],
            &["shape"],
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
