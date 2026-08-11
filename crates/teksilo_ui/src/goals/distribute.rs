// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Spreading a container's target across its immediate children.
//!
//! Nothing else on the market does this. Every writing tool surveyed either sums child
//! targets upward (Scrivener, and its fifteen-year-old complaint thread about the
//! double-counting that causes) or has no per-child target at all; the one tool that
//! distributes anything distributes it across *days*, not across the manuscript. So the
//! shape here is chosen rather than copied, and two of the choices are load-bearing:
//!
//! * **The numbers it writes are ordinary, independent targets.** Distribution is an
//!   action, not a standing relationship. The moment it finishes, every number it wrote
//!   behaves exactly like one typed by hand, and they are free to drift apart. Nothing in
//!   the UI may imply otherwise — no reconciliation warning, no "derived" styling.
//! * **The parts sum to the whole, exactly**, via
//!   [`apportion`](super::apportion::apportion). The preview's footer states that sum, so
//!   the guarantee is something the writer can see rather than something the code claims.
//!
//! Pure: [`plan`] takes the children a [`measure`](super::measure) walk found and returns
//! what would be written. The confirm modal renders a plan; only pressing OK writes one.

use super::apportion::apportion;
use super::measure::Child;

/// What a child's share is proportional to.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Weighting {
    /// By what is already written under each child. The default whenever anything is:
    /// a writer rebalancing a book's budget cares about the manuscript's actual shape.
    ///
    /// Falls back to [`Rows`](Weighting::Rows) when nothing is written yet, because a
    /// fresh outline carries no signal to weight by.
    #[default]
    Length,
    /// Equal shares.
    Even,
    /// By how many manuscript rows each child contains — a part with nine chapters gets
    /// nine times what a part with one does.
    Rows,
}

/// One row of the preview: what a child has, and what it would get.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Proposal {
    pub id: u64,
    pub title: String,
    /// The child's current target (`0` = none).
    pub current: i64,
    /// What this plan would write. Equal to `current` for a child the plan leaves alone.
    pub proposed: i64,
}

impl Proposal {
    /// Whether this row would actually change, so the preview can mark the untouched ones
    /// and the write can skip them.
    pub fn changes(&self) -> bool {
        self.proposed != self.current
    }
}

/// A distribution, ready to preview and then commit.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Plan {
    pub rows: Vec<Proposal>,
    /// What the rows add up to. Equals the parent's target whenever the plan is valid.
    pub total: i64,
    /// Set when hand-set child targets already exceed the parent's, in fill-empty mode:
    /// how far over. A plan carrying this must not be committed.
    ///
    /// Reported rather than clamped. Silently shrinking the writer's own numbers to fit
    /// would be the app quietly overruling a decision it was not asked about.
    pub over_budget: Option<i64>,
}

impl Plan {
    /// Whether committing this plan is meaningful: nothing over budget, and at least one
    /// row that would actually change.
    pub fn is_committable(&self) -> bool {
        self.over_budget.is_none() && self.rows.iter().any(Proposal::changes)
    }
}

fn weights(children: &[Child], weighting: Weighting) -> Vec<u64> {
    let by_length: Vec<u64> = children.iter().map(|c| c.subtree.words as u64).collect();
    match weighting {
        Weighting::Length if by_length.iter().any(|w| *w > 0) => by_length,
        // Nothing written yet: fall through to the structural weight rather than handing
        // `apportion` an all-zero vector and getting an even split it did not choose.
        Weighting::Length | Weighting::Rows => {
            children.iter().map(|c| c.subtree_rows as u64).collect()
        }
        Weighting::Even => vec![1; children.len()],
    }
}

/// Work out what distributing `parent_goal` across `children` would write.
///
/// With `overwrite` false (the default, and the non-destructive one), children that
/// already carry a target keep it and only the rest share what is left over. With it true,
/// every child is recomputed from scratch and whatever was there is discarded.
pub fn plan(parent_goal: i64, children: &[Child], weighting: Weighting, overwrite: bool) -> Plan {
    if children.is_empty() || parent_goal <= 0 {
        return Plan::default();
    }
    if overwrite {
        let shares = apportion(parent_goal, &weights(children, weighting));
        let rows: Vec<Proposal> = children
            .iter()
            .zip(shares)
            .map(|(c, proposed)| Proposal {
                id: c.id,
                title: c.title.clone(),
                current: c.goal,
                proposed,
            })
            .collect();
        let total = rows.iter().map(|r| r.proposed).sum();
        return Plan {
            rows,
            total,
            over_budget: None,
        };
    }

    let kept: i64 = children.iter().filter(|c| c.goal > 0).map(|c| c.goal).sum();
    let remaining = parent_goal - kept;
    if remaining < 0 {
        // Every row stays as it is; the caller shows the shortfall and refuses to commit.
        let rows: Vec<Proposal> = children
            .iter()
            .map(|c| Proposal {
                id: c.id,
                title: c.title.clone(),
                current: c.goal,
                proposed: c.goal,
            })
            .collect();
        return Plan {
            rows,
            total: kept,
            over_budget: Some(-remaining),
        };
    }

    let empties: Vec<&Child> = children.iter().filter(|c| c.goal <= 0).collect();
    let shares = apportion(remaining, &weights_of(&empties, weighting));
    let mut next = shares.into_iter();
    let rows: Vec<Proposal> = children
        .iter()
        .map(|c| {
            let proposed = if c.goal > 0 {
                c.goal
            } else {
                next.next().unwrap_or(0)
            };
            Proposal {
                id: c.id,
                title: c.title.clone(),
                current: c.goal,
                proposed,
            }
        })
        .collect();
    let total = rows.iter().map(|r| r.proposed).sum();
    Plan {
        rows,
        total,
        over_budget: None,
    }
}

/// [`weights`] over borrowed children, for the fill-empty path's filtered subset.
fn weights_of(children: &[&Child], weighting: Weighting) -> Vec<u64> {
    let owned: Vec<Child> = children.iter().map(|c| (*c).clone()).collect();
    weights(&owned, weighting)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn child(id: u64, goal: i64, words: usize, rows: usize) -> Child {
        Child {
            id,
            title: format!("Item {id}"),
            goal,
            subtree: super::super::measure::Counts {
                words,
                chars: words * 6,
            },
            subtree_rows: rows,
        }
    }

    #[test]
    fn fill_empty_leaves_hand_set_targets_alone_and_shares_the_rest() {
        let kids = vec![
            child(1, 10_000, 500, 1),
            child(2, 0, 500, 1),
            child(3, 0, 500, 1),
        ];
        let p = plan(90_000, &kids, Weighting::Length, false);
        assert_eq!(p.rows[0].proposed, 10_000, "kept verbatim");
        assert_eq!(p.rows[1].proposed + p.rows[2].proposed, 80_000);
        assert_eq!(p.total, 90_000);
        assert!(p.over_budget.is_none());
        assert!(p.is_committable());
    }

    #[test]
    fn overwrite_recomputes_every_child_from_scratch() {
        let kids = vec![child(1, 10_000, 1, 1), child(2, 0, 1, 1)];
        let p = plan(90_000, &kids, Weighting::Even, true);
        assert_eq!(p.rows[0].proposed, 45_000);
        assert_eq!(p.rows[1].proposed, 45_000);
        assert_eq!(p.total, 90_000);
    }

    /// The case the design refuses to paper over: hand-set targets already exceed the
    /// parent's, so there is nothing to share and clamping would silently rewrite the
    /// writer's own numbers.
    #[test]
    fn hand_set_targets_over_the_parent_report_the_shortfall_and_change_nothing() {
        let kids = vec![
            child(1, 60_000, 0, 1),
            child(2, 50_000, 0, 1),
            child(3, 0, 0, 1),
        ];
        let p = plan(90_000, &kids, Weighting::Length, false);
        assert_eq!(p.over_budget, Some(20_000));
        assert!(p.rows.iter().all(|r| !r.changes()));
        assert!(!p.is_committable());
    }

    #[test]
    fn length_weighting_follows_what_is_already_written() {
        let kids = vec![child(1, 0, 8_000, 1), child(2, 0, 2_000, 1)];
        let p = plan(50_000, &kids, Weighting::Length, false);
        assert_eq!(p.rows[0].proposed, 40_000);
        assert_eq!(p.rows[1].proposed, 10_000);
    }

    /// A fresh outline has no length to weight by, so length weighting has to mean
    /// something rather than collapsing to an even split by accident.
    #[test]
    fn length_weighting_falls_back_to_row_counts_when_nothing_is_written() {
        let kids = vec![child(1, 0, 0, 4), child(2, 0, 0, 9), child(3, 0, 0, 2)];
        let p = plan(90_000, &kids, Weighting::Length, false);
        assert_eq!(
            p.rows.iter().map(|r| r.proposed).collect::<Vec<_>>(),
            vec![24_000, 54_000, 12_000]
        );
    }

    #[test]
    fn no_children_or_no_parent_goal_plans_nothing() {
        assert!(plan(90_000, &[], Weighting::Even, false).rows.is_empty());
        assert!(
            plan(0, &[child(1, 0, 0, 1)], Weighting::Even, false)
                .rows
                .is_empty()
        );
    }

    /// Every child already carrying exactly the parent's total is a valid, finished state,
    /// not an error — and there is nothing left to commit.
    #[test]
    fn an_exactly_budgeted_outline_is_valid_but_not_committable() {
        let kids = vec![child(1, 45_000, 0, 1), child(2, 45_000, 0, 1)];
        let p = plan(90_000, &kids, Weighting::Length, false);
        assert_eq!(p.over_budget, None);
        assert_eq!(p.total, 90_000);
        assert!(!p.is_committable(), "nothing would change");
    }
}
