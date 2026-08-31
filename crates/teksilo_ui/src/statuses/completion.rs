// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Where the manuscript stands, by rung — the completion readout.
//!
//! **This module owns the content and nothing else.** [`readout`] returns a bare widget
//! with no modal chrome, no title bar and no dismiss, so the two places it appears cannot
//! drift apart:
//!
//! * folded into the Pace summary that opens with a project (`pace::panel`), under the
//!   plan it already shows, and
//! * as its own panel, opened on demand from the menu bar (`statuses::panel`).
//!
//! One builder rather than two similar ones, because the failure mode here is specific and
//! quiet: two readouts of the same number that disagree after one of them is changed is
//! worse than having only one of them, and nothing in a test would notice.
//!
//! # It is a gadget, and it counts rows
//!
//! Deliberately **rows, not words**. "Nine of twelve scenes are final" is a sentence a
//! writer can act on; the same fraction weighted by length is a different and much
//! wobblier claim — one long unfinished scene would swamp eleven short finished ones and
//! the bar would say something untrue about how much is left to *do*. The word counts are
//! already answered, honestly, by the Pace plan this sits under and by the Overview's own
//! columns.
//!
//! Counted over rows that can actually carry a status *and* hold prose
//! ([`skribisto_model::counts_prose`]), so a Book folder and a notes folder do not dilute
//! the manuscript's own figure.

use std::rc::Rc;

use common::entities::StatusCategory;
use frontend::AppContext;
use frontend::commands::binder_item_commands;
use teksilo::prelude::*;
use teksilo::widgets::{HStack, Spacer, TextWidget, VStack};

use crate::app_ids::AppIds;
use crate::statuses::{StatusRung, StatusesViewModel};

/// One line of the readout: a rung, and how many prose rows sit on it.
pub struct Tally {
    pub rung: Option<StatusRung>,
    pub count: usize,
}

/// The whole reading, ready to render.
pub struct Completion {
    /// Ladder order, with the unset bucket **first** — the ladder starts before its first
    /// rung, and the row reads top-to-bottom as progress.
    pub tallies: Vec<Tally>,
    pub total: usize,
}

impl Completion {
    /// How many rows are on the last rung of the ladder — the closest thing to "done".
    ///
    /// Positional, not `StatusCategory::Final`: several rungs may share that bucket (an
    /// eight-rung imported ladder collapses onto five glyphs), and the writer's own last
    /// rung is what they mean by finished, whatever its category.
    pub fn finished(&self) -> usize {
        self.tallies
            .last()
            .map_or(0, |t| if t.rung.is_some() { t.count } else { 0 })
    }

    pub fn is_empty(&self) -> bool {
        self.total == 0
    }
}

/// Count the project's prose rows by rung.
pub fn measure(app_ctx: &AppContext, ids: &AppIds, statuses: &StatusesViewModel) -> Completion {
    let ladder = statuses.ladder();
    let mut counts: Vec<usize> = vec![0; ladder.len()];
    let mut unset = 0usize;
    let mut total = 0usize;

    let Some(work_id) = ids.work_id.get() else {
        return Completion {
            tallies: Vec::new(),
            total: 0,
        };
    };
    for item in crate::models::binder_stream::ordered_binder_items(app_ctx, work_id) {
        let Ok(Some(dto)) = binder_item_commands::get_binder_item(app_ctx, &item.id) else {
            continue;
        };
        // Trashed rows are not part of the manuscript and must not dilute the figure —
        // `activated = !trashed` is the model's own spelling of that.
        if !dto.activated || !skribisto_model::counts_prose(&dto.role, &dto.sub_role) {
            continue;
        }
        total += 1;
        // A rung that no longer resolves counts as unset, which is what every other reader
        // does with it: the reference is weak so that deleting a rung leaves the prose.
        match dto
            .status
            .and_then(|id| ladder.iter().position(|r| r.id == id))
        {
            Some(i) => counts[i] += 1,
            None => unset += 1,
        }
    }

    let mut tallies = vec![Tally {
        rung: None,
        count: unset,
    }];
    for (rung, count) in ladder.into_iter().zip(counts) {
        tallies.push(Tally {
            rung: Some(rung),
            count,
        });
    }
    Completion { tallies, total }
}

/// The readout as a widget — **content only**, so both hosts render the same thing.
///
/// An empty project renders a single line saying so rather than a table of zeros: the same
/// "no chrome for a question nobody can ask yet" discipline `pace::panel` follows when it
/// declines to open at all.
pub fn readout(c: &Completion) -> Box<dyn Widget> {
    if c.is_empty() {
        return Box::new(
            TextWidget::new(tr!(status_completion_empty()))
                .style(TextStyleRole::Small)
                .color(TextRole::Secondary),
        );
    }

    let mut col = VStack::new().spacing(8.0);
    col = col.child(
        TextWidget::new(tr!(status_completion_headline(
            done = c.finished() as i64,
            total = c.total as i64
        )))
        .style(TextStyleRole::BodyBold),
    );

    for t in &c.tallies {
        // A rung nobody is on is left out. A ladder is a plan, not a report, and listing
        // its empty rungs buries the two or three that carry the manuscript.
        if t.count == 0 {
            continue;
        }
        let (label, glyph): (LocalizedString, Option<teksilo::widgets::IconWidget>) = match &t.rung
        {
            Some(r) => (
                lit!(r.name.clone()),
                Some(crate::statuses::status_glyph(&r.category)),
            ),
            None => (tr!(status_none()), None),
        };
        let mut line = HStack::new().spacing(6.0);
        if let Some(g) = glyph {
            line = line.child(g);
        } else {
            // The unset bucket keeps the glyph column's width so the names stay aligned.
            line = line.child(
                crate::statuses::glyph::category_icon(&StatusCategory::Planned)
                    .color(TextRole::Disabled),
            );
        }
        col = col.child(
            line.child(TextWidget::new(label).single_line())
                .child(Spacer::new())
                .child(
                    TextWidget::new(lit!(t.count.to_string()))
                        .color(TextRole::Secondary)
                        .single_line(),
                ),
        );
    }
    Box::new(col)
}

/// Measure and render in one call — what both hosts actually use.
pub fn readout_for(
    app_ctx: &Rc<AppContext>,
    ids: &AppIds,
    statuses: &StatusesViewModel,
) -> Box<dyn Widget> {
    readout(&measure(app_ctx, ids, statuses))
}

#[cfg(test)]
mod tests {
    use super::*;

    use teksilo::core::widget_tree::WidgetTree;

    /// Count the `TextWidget`s below `id` — the readout is text plus glyphs, so this
    /// counts its lines without depending on how they are stacked.
    fn texts(tree: &WidgetTree, id: WidgetId) -> usize {
        let mine = usize::from(
            tree.widget_type_name(id)
                .is_some_and(|n| n.ends_with("::TextWidget")),
        );
        tree.children(id)
            .into_iter()
            .map(|c| texts(tree, c))
            .sum::<usize>()
            + mine
    }

    fn rung(id: u64, name: &str, category: StatusCategory) -> StatusRung {
        StatusRung {
            id,
            uid: common::uid::fixture_uid(id),
            name: name.into(),
            category,
            details: String::new(),
        }
    }

    /// The populated readout has to be laid out somewhere, and the `mocks` build cannot do
    /// it: `measure` scans the real store through `binder_item_commands`, while the mock
    /// binder tree lives in `BinderBinderItemsTreeModel`'s own `mocks` arm and never
    /// reaches the store. So the running app can only ever show this branch's *empty*
    /// state. Laid out headlessly here instead, which is the project's stated alternative
    /// to a visual check.
    #[test]
    fn a_populated_readout_lays_out() {
        let c = Completion {
            tallies: vec![
                Tally {
                    rung: None,
                    count: 4,
                },
                Tally {
                    rung: Some(rung(1, "Draft", StatusCategory::Drafting)),
                    count: 3,
                },
                Tally {
                    // Empty rungs are skipped, so this one must not appear.
                    rung: Some(rung(2, "Needs work", StatusCategory::NeedsWork)),
                    count: 0,
                },
                Tally {
                    rung: Some(rung(3, "Final", StatusCategory::Final)),
                    count: 2,
                },
            ],
            total: 9,
        };
        assert_eq!(c.finished(), 2);

        // One line per *non-empty* rung, plus the headline: the empty "Needs work" rung
        // must not appear, which is the behaviour a table of zeros would lose.
        let mut tree = teksilo::core::widget_tree::WidgetTree::new();
        let root = tree.add_boxed(readout(&c));
        tree.layout(SizeProposal::with_width(360.0));
        // 1 headline + 3 populated rungs x (name + count). The empty "Needs work" rung
        // contributes nothing — with it the count would be 9, which is precisely the
        // table-of-zeros this skips.
        assert_eq!(
            texts(&tree, root),
            7,
            "headline + three populated rungs, and nothing for the empty one"
        );
    }

    /// The empty branch is the one the running app *can* show, and it must not render a
    /// table of zeros.
    #[test]
    fn an_empty_readout_lays_out_as_one_line() {
        let c = Completion {
            tallies: Vec::new(),
            total: 0,
        };
        assert!(c.is_empty());
        let mut tree = teksilo::core::widget_tree::WidgetTree::new();
        let root = tree.add_boxed(readout(&c));
        tree.layout(SizeProposal::with_width(360.0));
        assert_eq!(texts(&tree, root), 1, "one line, not a table of zeros");
    }

    /// "Finished" is the ladder's **last rung**, not the `Final` category: an imported
    /// eight-rung ladder puts several rungs in that bucket, and the writer's own last one
    /// is what they mean.
    #[test]
    fn finished_counts_the_last_rung_not_the_final_category() {
        let c = Completion {
            tallies: vec![
                Tally {
                    rung: None,
                    count: 4,
                },
                Tally {
                    // Shares the Final bucket, but is not the last rung.
                    rung: Some(rung(1, "Proofread", StatusCategory::Final)),
                    count: 3,
                },
                Tally {
                    rung: Some(rung(2, "Published", StatusCategory::Final)),
                    count: 2,
                },
            ],
            total: 9,
        };
        assert_eq!(c.finished(), 2, "only the last rung counts as done");
    }

    /// A ladder with nothing on its last rung still reports honestly rather than reaching
    /// backwards for the nearest non-empty one.
    #[test]
    fn nothing_finished_reports_zero() {
        let c = Completion {
            tallies: vec![
                Tally {
                    rung: None,
                    count: 5,
                },
                Tally {
                    rung: Some(rung(1, "Final", StatusCategory::Final)),
                    count: 0,
                },
            ],
            total: 5,
        };
        assert_eq!(c.finished(), 0);
        assert!(!c.is_empty(), "rows exist; none of them are done");
    }

    /// The unset bucket is never mistaken for a rung — `finished` reads the last *tally*,
    /// and a ladder-less project's only tally is the unset one.
    #[test]
    fn a_project_with_no_ladder_has_nothing_finished() {
        let c = Completion {
            tallies: vec![Tally {
                rung: None,
                count: 7,
            }],
            total: 7,
        };
        assert_eq!(c.finished(), 0);
    }
}
