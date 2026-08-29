// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The weekday chips and the progression / words-per-day charts.

#[allow(unused_imports)]
use super::*;

// ── WeekdayChips: the seven counted-day toggles ─────────────────────────────

/// A wrapping row of seven weekday chips over the Pace's `weekday_mask`. Rebuilds
/// on any mask change (so a chip's fill reflects the current bit); activating a
/// chip toggles that day's bit through the view-model.
///
/// **Each chip is a real [`Button`], not a tinted `ZStack`.** It was the latter,
/// and that made the writing schedule the one setting in the app a keyboard
/// could not reach: a `ZStack` carrying only `.on_tap` is a pointer-only
/// control — no focus stop, no ring, no Enter/Space, and nothing but a promoted
/// text label in the accessibility tree. `WeekdayChips` is the only writer of
/// `weekday_mask` anywhere in the UI, so there was no second route to it.
///
/// A `Button` brings the focus ring, the hover and press chrome, Enter/Space
/// and `Action::Click` with it. The two `access_*` calls then say what kind of
/// button it is: accesskit has no `ToggleButton` role, so a multi-select chip
/// is a `CheckBox` carrying `set_toggled` — the same encoding `MenuItem` uses
/// for its check mode. Applied last, because they wrap the `Button` and every
/// `Button` method has to come above them (the idiom `welcome::panel`'s
/// `link_button` documents).
pub(super) struct WeekdayChips {
    vm: PaceViewModel,
    root: Option<WidgetId>,
}

impl WeekdayChips {
    pub(super) fn new(vm: PaceViewModel) -> Self {
        Self { vm, root: None }
    }
}

impl std::fmt::Debug for WeekdayChips {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WeekdayChips").finish()
    }
}

/// A chip's floor width. Measured, not guessed: with [`CHIP_PADDING_H`] the
/// widest en-US abbreviation ("Wed") asks for 43.7 dp and the narrowest ("Fri")
/// for 31.5, so 50 dp is above every one of them — which is what makes the seven
/// chips *uniform* — while 7 × 50 + 6 × 6 dp of spacing still fits one line of a
/// dashboard column at any width above roughly 390 dp. Narrower than that (the
/// `ColumnFlow`'s 300 dp floor, three columns in a small window) the `Wrap`
/// breaks the row onto two lines, which is the point of it: no label is ever
/// truncated to make the row fit.
const CHIP_MIN_WIDTH: f32 = 50.0;
const CHIP_HEIGHT: f32 = 30.0;
/// Breathing room around the label. Well under the stock button's 14 dp, which
/// is sized for "Cancel", not for "Wed".
const CHIP_PADDING_H: f32 = 8.0;

/// The chip's chrome: the stock recipes with one thing changed — their footprint.
///
/// **This is the fix for a real truncation bug, not a nicety.** The chip used to be
/// a `Button` inside `FixedSize::new().width(46.0)`, and a `Button` is rigid: it does
/// not shrink to a smaller proposal, it *truncates its label*. Every stock recipe
/// carries `padding: 14 dp each side`, so 46 dp left 18 dp for the text — enough for
/// "Fri" and nothing else, which is exactly what shipped: six chips reading "…" and
/// one reading "Fri".
///
/// Trimming the padding and moving the 46 dp from an outer `FixedSize` (a hard cap)
/// into the recipe's own `min_size` (a floor) fixes both halves. Seven three-letter
/// abbreviations now measure the same and stay aligned, and a locale whose weekday
/// names are longer gets a wider chip rather than a truncated one — the failure mode
/// degrades to ragged, never to unreadable.
///
/// Every variant is trimmed, not just the two in use: `RecipeButtonStyle` falls back
/// to `Plain` for variants it has no entry for, and a style whose footprint depended
/// on which variant a caller happened to pass would be a trap for the next edit.
fn weekday_chip_style() -> RecipeButtonStyle {
    let mut style = RecipeButtonStyle::intui();
    for recipe in style.recipes.values_mut() {
        // `EdgeInsets::symmetric` takes (horizontal, vertical).
        recipe.padding = EdgeInsets::symmetric(CHIP_PADDING_H, 0.0);
        recipe.min_size = Size::new(CHIP_MIN_WIDTH, CHIP_HEIGHT);
    }
    style
}

/// One weekday chip: a toggle button over a single bit of the `weekday_mask`.
///
/// Split out of the loop so its keyboard and accessibility contract can be
/// tested without standing up a whole `PaceViewModel` — the contract is the
/// point of it, and it is the half that was missing.
fn weekday_chip(
    label: LocalizedString,
    on: bool,
    toggle: impl Fn(&mut EventContext) + 'static,
) -> impl Widget + 'static {
    Button::new(label)
        // Filled reads as "counted", Plain as "skipped" — the same accent-fill /
        // neutral pair the hand-drawn chip painted, now resolved by the theme so
        // it follows light and dark, and picks up the button's own hover, press
        // and disabled states with it.
        .variant(if on {
            ButtonVariant::Filled
        } else {
            ButtonVariant::Plain
        })
        // The chip's own footprint — see `weekday_chip_style`. Nothing wraps this
        // button: the floor lives in the recipe, so a label that needs more room
        // gets it.
        .style(weekday_chip_style())
        .on_activate_fn(toggle)
        // Last: these wrap the `Button`, so every `Button` method is above them.
        .access_role(teksilo::core::accesskit::Role::CheckBox)
        .access_customize(move |b| b.set_toggled(on))
}

impl Widget for WeekdayChips {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        self.vm.weekday_mask().bind_to(
            ctx.self_id(),
            ctx.binding_registry(),
            BindingLevel::Rebuild,
        );
        let mask = self.vm.weekday_mask().get();

        let mut row = Wrap::new().spacing(6.0).line_spacing(6.0);
        for (bit, label) in weekdays() {
            let on = mask & bit != 0;
            let vm = self.vm.clone();
            row = row.child(weekday_chip(label, on, move |_c| {
                let m = vm.weekday_mask().get();
                vm.set_weekday_mask(m ^ bit);
            }));
        }
        self.root = Some(ctx.add(row));
        self.root.into_iter().collect()
    }

    fn layout_response(&self, proposal: SizeProposal, ctx: &LayoutContext) -> LayoutResponse {
        self.root
            .and_then(|id| ctx.child_size(id, proposal))
            .map(LayoutResponse::from)
            .unwrap_or_else(|| proposal.resolve(0.0, 0.0).into())
    }
}

// ── PaceCharts: progression (actual vs target) + words-per-day ──────────────

/// The two progress charts, rebuilt on the view-model's `version` (a new snapshot
/// or a schedule edit): a line chart of the recorded cumulative words against the
/// even-pace target, and a bar chart of words written per day. Until any progress
/// is recorded there is nothing to plot, so it shows a hint instead.
pub(super) struct PaceCharts {
    vm: PaceViewModel,
    root: Option<WidgetId>,
}

impl PaceCharts {
    pub(super) fn new(vm: PaceViewModel) -> Self {
        Self { vm, root: None }
    }
}

impl std::fmt::Debug for PaceCharts {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PaceCharts").finish()
    }
}

impl Widget for PaceCharts {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        self.vm
            .version()
            .bind_to(ctx.self_id(), ctx.binding_registry(), BindingLevel::Rebuild);

        let actual = self.vm.actual_series();
        if actual.is_empty() {
            self.root =
                Some(ctx.add(TextWidget::new(tr!(pace_charts_empty())).color(TextRole::Secondary)));
            return self.root.into_iter().collect();
        }

        // Progression: recorded cumulative vs the even-pace target.
        let mut actual_series = ChartSeries::new(tr!(pace_series_actual()).resolve_now());
        for (date, words) in &actual {
            actual_series.push(day_label(*date), *words as f32);
        }
        let mut line_series = vec![actual_series];
        // Only plot the target once a start/end/goal exist.
        if actual.iter().any(|(d, _)| self.vm.target_for(*d).is_some()) {
            let mut target = ChartSeries::new(tr!(pace_series_target()).resolve_now());
            for (date, _) in &actual {
                target.push(
                    day_label(*date),
                    self.vm.target_for(*date).unwrap_or(0) as f32,
                );
            }
            line_series.push(target);
        }
        let line = LineChart::new(ChartModel::from_series_vec(line_series))
            .points(true)
            .grid(true)
            .legend(true);

        // Words written per day, each bar tinted red when that day fell below the
        // even-pace daily target (and accent otherwise).
        let rate = self.vm.target_daily_rate();
        let points: Vec<ChartDatum<String>> = self
            .vm
            .words_per_day()
            .into_iter()
            .map(|(date, words)| {
                let datum = ChartDatum::new(day_label(date), words as f32);
                match rate {
                    Some(r) if words < r => datum.with_color(SurfaceRole::StatusError),
                    _ => datum.with_color(SurfaceRole::Accent),
                }
            })
            .collect();
        let per_day_len = points.len();
        let per_day = ChartSeries::new(tr!(pace_series_words_per_day()).resolve_now()).data(points);
        let mut bars = BarChart::new(ChartModel::from_series_vec(vec![per_day]))
            .grid(true)
            .legend(false);
        // The rate the bars are tinted against, drawn. The progression chart above plots its
        // target as a series and so has always shown it; this one compared every bar to a
        // number that appeared nowhere on it.
        if let Some(r) = rate {
            bars = bars.reference_line(ReferenceLine::new(
                r as f32,
                tr!(pace_daily_target_line(count = r)),
            ));
        }

        let col = VStack::new()
            .spacing(12.0)
            .child(
                TextWidget::new(tr!(pace_chart_progression()))
                    .style(TextStyleRole::Small)
                    .color(TextRole::Secondary),
            )
            // Sized from the day count, not the viewport: a project running for a year
            // squeezed a line chart into a thumbnail, and its companion bar chart below hit
            // `BarChart`'s 4px bar floor and drew past its own plot. Same helper the
            // Analysis charts use, so the two panes render the same manuscript at the same
            // scale.
            .child(crate::tabs::shared::wide_chart(
                actual.len(),
                crate::tabs::shared::CHART_HEIGHT,
                line,
            ))
            .child(
                TextWidget::new(tr!(pace_chart_words_per_day()))
                    .style(TextStyleRole::Small)
                    .color(TextRole::Secondary),
            )
            .child(crate::tabs::shared::wide_chart(
                per_day_len,
                crate::tabs::shared::STRIP_HEIGHT,
                bars,
            ));
        self.root = Some(ctx.add(col));
        self.root.into_iter().collect()
    }

    fn layout_response(&self, proposal: SizeProposal, ctx: &LayoutContext) -> LayoutResponse {
        self.root
            .and_then(|id| ctx.child_size(id, proposal))
            .map(LayoutResponse::from)
            .unwrap_or_else(|| proposal.resolve(0.0, 0.0).into())
    }
}

/// A holiday's date span, compact (`7/16` or `7/16–7/20`).
pub(super) fn holiday_span(start: NaiveDate, end: NaiveDate) -> String {
    if start == end {
        day_label(start)
    } else {
        format!("{}\u{2013}{}", day_label(start), day_label(end))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell as StdCell;
    use std::rc::Rc as StdRc;
    use teksilo::core::accesskit::{Role, Toggled};
    use teksilo::core::event::{Key, Modifiers};
    use teksilo::core::widget_tree::WidgetTree;

    fn tree() -> WidgetTree {
        WidgetTree::new().with_theme(teksilo::presets::intui::light())
    }

    /// Build one chip and hand back the tree plus a "was it toggled" cell.
    fn chip(on: bool) -> (WidgetTree, WidgetId, StdRc<StdCell<bool>>) {
        let fired = StdRc::new(StdCell::new(false));
        let f = fired.clone();
        let mut t = tree();
        let id = t.add_boxed(Box::new(weekday_chip(lit!("Mon"), on, move |_c| {
            f.set(true)
        })));
        t.layout(SizeProposal::exact(200.0, 60.0));
        (t, id, fired)
    }

    /// The regression this replaced: the chip was a `ZStack` carrying `.on_tap`,
    /// which is a pointer-only control — no focus stop at all. `WeekdayChips` is
    /// the only writer of `weekday_mask` in the whole UI, so a keyboard-only
    /// writer could not choose which days their schedule counts.
    #[test]
    fn a_chip_is_reachable_and_operable_from_the_keyboard() {
        let (mut t, id, fired) = chip(false);

        let button = t
            .first_focusable_descendant(id)
            .expect("a weekday chip must be a focus stop");
        t.focus(button);
        t.press_key(Key::Enter, Modifiers::NONE);
        assert!(fired.get(), "Enter must toggle the day");
    }

    /// A tree with a real text metric — without one the fallback reports a rigid
    /// size and a truncated label measures exactly as a fitting one does, so the
    /// two tests below would pass against the bug they exist for.
    fn measuring_tree() -> WidgetTree {
        tree().with_text_backend(StdRc::new(std::cell::RefCell::new(
            teksilo::canvas::MockTextBackend::new(),
        )))
    }

    /// Lay one chip out on its own and report the width it took.
    fn chip_width(label: &str) -> f32 {
        let mut t = measuring_tree();
        let id = t.add_boxed(Box::new(weekday_chip(lit!(label), false, |_c| {})));
        // `unspecified`, never `exact`: the root of a tree is placed at whatever the
        // proposal says, so an exact box would report the box back and measure nothing.
        // Unspecified asks the chip what it actually wants.
        t.layout(SizeProposal::unspecified());
        t.bounds(id).size().width
    }

    /// **The truncation bug.** The chip used to be a `Button` capped by
    /// `FixedSize::new().width(46.0)`, and a `Button` does not shrink — it
    /// truncates. Every weekday but "Fri" rendered as "…".
    ///
    /// The guard is that a chip is a *floor*, not a cap: a label wider than the
    /// floor must make the chip wider, never make the text shorter. A reinstated
    /// `FixedSize` fails this immediately.
    #[test]
    fn a_chip_grows_for_a_label_too_wide_for_its_floor() {
        let short = chip_width("Fri");
        let long = chip_width("Wednesday-ish");
        assert_eq!(
            short, CHIP_MIN_WIDTH,
            "a short label sits at the floor, so every abbreviation lines up"
        );
        assert!(
            long > CHIP_MIN_WIDTH,
            "a label past the floor must widen the chip (got {long} for a floor of \
             {CHIP_MIN_WIDTH}) — a cap would truncate it instead"
        );
    }

    /// …and the floor is above every weekday the app ships, in every locale it
    /// ships, so the seven chips are the same width and stay aligned.
    #[test]
    fn every_shipped_weekday_fits_the_floor() {
        for label in [
            // en-US
            "Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun", // fr-FR
            "Lun", "Mar", "Mer", "Jeu", "Ven", "Sam", "Dim",
        ] {
            assert_eq!(
                chip_width(label),
                CHIP_MIN_WIDTH,
                "{label:?} must fit the floor, or the row goes ragged"
            );
        }
    }

    /// A multi-select chip is a check box carrying its state, not a bare button:
    /// accesskit has no `ToggleButton` role, so `CheckBox` + `toggled` is the
    /// encoding (the same one `MenuItem`'s check mode uses).
    #[test]
    fn a_chip_announces_its_name_role_and_state() {
        for on in [false, true] {
            let (mut t, id, _) = chip(on);
            let _ = t.render();
            let update = t.sync_accessibility();
            let node = update
                .nodes
                .iter()
                .map(|(_, n)| n)
                .find(|n| n.role() == Role::CheckBox)
                .unwrap_or_else(|| panic!("no CheckBox node for on={on}"));

            assert!(
                node.label().is_some_and(|l| l == "Mon"),
                "the chip announces the weekday, got {:?}",
                node.label()
            );
            assert_eq!(
                node.toggled(),
                Some(if on { Toggled::True } else { Toggled::False }),
                "the chip announces whether the day is counted (on={on})"
            );
            assert!(
                node.supports_action(teksilo::core::accesskit::Action::Click),
                "assistive tech must be able to activate the chip"
            );
            let _ = id;
        }
    }
}
