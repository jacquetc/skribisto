// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The weekday chips and the progression / words-per-day charts.

#[allow(unused_imports)]
use super::*;

// ── WeekdayChips: the seven counted-day toggles ─────────────────────────────

/// A wrapping row of seven weekday chips over the Pace's `weekday_mask`. Rebuilds
/// on any mask change (so a chip's fill reflects the current bit); a click
/// toggles that day's bit through the view-model.
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
            let (bg, fg) = if on {
                (SurfaceRole::Accent, TextRole::OnAccent)
            } else {
                (SurfaceRole::Container, TextRole::Secondary)
            };
            let vm = self.vm.clone();
            let chip = ZStack::new()
                .child(
                    RectWidget::new()
                        .background(bg)
                        .corner_radius(CornerRadius::uniform(8.0)),
                )
                .child(Center::new().child(TextWidget::new(label).color(fg).single_line()))
                .on_tap(move |_e, _c| {
                    let m = vm.weekday_mask().get();
                    vm.set_weekday_mask(m ^ bit);
                });
            row = row.child(FixedSize::new().width(46.0).height(30.0).child(chip));
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
                tr!(pace_daily_target_line(count = r as i64)),
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
