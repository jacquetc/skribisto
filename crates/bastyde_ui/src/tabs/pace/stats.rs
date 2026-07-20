// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The statistics row (streak, % done, days left, needed rate).

#[allow(unused_imports)]
use super::*;

/// One stat card: a big number over a small caption, inside a Panel.
pub(super) fn stat_card(
    big: &TextStyle,
    number: String,
    caption: LocalizedString,
    number_color: TextRole,
) -> impl Widget {
    Panel::new().child(
        Padding::uniform(14.0).child(
            VStack::new()
                .spacing(2.0)
                .child(
                    TextWidget::new(lit!(number))
                        .style(big.clone())
                        .color(number_color)
                        .single_line(),
                )
                .child(
                    TextWidget::new(caption)
                        .style(TextStyleRole::Small)
                        .color(TextRole::Secondary),
                ),
        ),
    )
}

/// The derived statistics, flowed through a [`ColumnFlow`] of cards — one to four columns
/// by width, height-balanced. Rebuilt only on the view-model's `version` (a data change);
/// the column count is `ColumnFlow`'s own responsibility, so no width-measuring here.
pub(super) struct StatCards {
    vm: PaceViewModel,
    today: NaiveDate,
    root: Option<WidgetId>,
}

impl StatCards {
    pub(super) fn new(vm: PaceViewModel, today: NaiveDate) -> Self {
        Self {
            vm,
            today,
            root: None,
        }
    }
}

impl std::fmt::Debug for StatCards {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StatCards").finish()
    }
}

impl Widget for StatCards {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        self.vm
            .version()
            .bind_to(ctx.self_id(), ctx.binding_registry(), BindingLevel::Rebuild);

        // A big, bold number style derived from the theme's body style.
        let big = TextStyle {
            size: 24.0,
            weight: FontWeight::BOLD,
            ..ctx.theme().typography.body_bold.clone()
        };
        let vm = &self.vm;
        let today = self.today;

        let mut cf = ColumnFlow::new()
            .min_column_width(150.0)
            .max_columns(4)
            .column_spacing(10.0)
            .item_spacing(10.0);

        cf = cf.child(stat_card(
            &big,
            commafy(vm.current_words()),
            tr!(pace_card_written()),
            TextRole::Accent,
        ));
        if let Some(p) = vm.percent_done() {
            cf = cf.child(stat_card(
                &big,
                format!("{}%", (p * 100.0).round() as i64),
                tr!(pace_card_of_goal()),
                TextRole::Accent,
            ));
        }
        if let Some(r) = vm.words_per_writing_day(today) {
            cf = cf.child(stat_card(
                &big,
                commafy(r),
                tr!(pace_card_rate()),
                TextRole::Accent,
            ));
        }
        if vm.end().get().is_some() {
            cf = cf.child(stat_card(
                &big,
                vm.writing_days_left(today).to_string(),
                tr!(pace_card_days_left()),
                TextRole::Accent,
            ));
        }
        cf = cf.child(stat_card(
            &big,
            vm.streak(today).to_string(),
            tr!(pace_card_streak()),
            TextRole::Accent,
        ));
        if let Some(d) = vm.ahead_behind(today) {
            let (num, caption, color) = if d >= 0 {
                (commafy(d), tr!(pace_card_ahead()), TextRole::Success)
            } else {
                (commafy(-d), tr!(pace_card_behind()), TextRole::Warning)
            };
            cf = cf.child(stat_card(&big, num, caption, color));
        }

        self.root = Some(ctx.add(cf));
        self.root.into_iter().collect()
    }

    fn layout_response(&self, proposal: SizeProposal, ctx: &LayoutContext) -> LayoutResponse {
        self.root
            .and_then(|id| ctx.child_size(id, proposal))
            .map(LayoutResponse::from)
            .unwrap_or_else(|| proposal.resolve(0.0, 0.0).into())
    }
}
