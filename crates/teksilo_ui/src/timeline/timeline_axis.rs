// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! What the Timeline band's axis actually draws: either one bar per recorded
//! moment, or — when there are too many for the band to hold — one bar per
//! calendar period, which the writer opens to see the moments inside it.
//!
//! ## Why bucketing rather than scrolling
//!
//! A band is a fixed, short, wide surface. The rest of the app answers "too many
//! data points" by giving each one a real slot and scrolling — `tabs::shared::
//! charts::wide_chart`, used by Pace and Analysis. That is right for a chart you
//! *read*, and wrong for one you *aim at*: 300 backups at that pitch is 8,400 dp
//! of scrolling to reach a date, with no way to skip. Two years of history is a
//! calendar question — "what did this look like last March" — and a calendar
//! answers it in two steps rather than a scroll.
//!
//! The units come from [`BucketUnit`], which is retention's own GFS bucket key.
//! The band groups a project's past exactly the way the backup sweep thins it,
//! so a bar and a retention tier can never disagree about what "the same week"
//! means.
//!
//! ## Why there is a terminal state
//!
//! Below [`MAX_BARS`] the axis stops bucketing and draws the moments themselves.
//! Without that floor "open this period" would recurse forever on a period
//! holding one backup, and the writer could never reach a version at all.

use chrono::{DateTime, Utc};

use skrib_format::retention::BucketUnit;

use super::timeline_vm::Moment;

/// The most bars a band this shape can carry.
///
/// Not a guess: the axis is roughly 450 dp wide, and a bar needs about 6 dp of
/// its own plus the chart's minimum gap to read as a bar rather than a hairline.
/// Above this the previous, hand-rolled row did not degrade — its `HStack`
/// spacing alone exceeded the available width and every bar laid out to zero, so
/// 300 backups drew an empty band.
pub const MAX_BARS: usize = 40;

/// One bar of the axis.
#[derive(Debug, Clone, PartialEq)]
pub struct Bar {
    /// What the x-axis calls it — a date, or a period like `2026-03`.
    pub label: String,
    /// Height: the prose the project held at this point.
    pub bytes: u64,
    /// The moment this bar stands for — for a period, its newest.
    ///
    /// A period's height is the size the project had reached by the *end* of it,
    /// which is what "how much your project held then" means for a span. The mean
    /// of a period would be a number the project never actually was.
    pub at: DateTime<Utc>,
    /// Index into the visible moments of the moment this bar stands for.
    ///
    /// Always present, including on a period: a period is represented by its
    /// newest moment, so the change list beside the axis has something real to
    /// compare against at either level. Whether selecting the bar *opens* it is
    /// [`Axis::opens`]'s business, not this field's.
    pub moment: usize,
    /// The span a period covers, for opening it. `None` on a moment bar.
    pub span: Option<(DateTime<Utc>, DateTime<Utc>)>,
    /// How many moments this bar stands for. Always 1 for a moment bar.
    pub count: usize,
}

/// The axis as a whole: what its bars mean, and therefore what selecting one does.
#[derive(Debug, Clone, PartialEq)]
pub struct Axis {
    pub bars: Vec<Bar>,
    /// `Some(unit)` when the bars are periods, `None` when they are moments.
    ///
    /// The one flag the dock branches on: a period is opened, a moment is chosen.
    pub unit: Option<BucketUnit>,
}

impl Axis {
    pub fn is_empty(&self) -> bool {
        self.bars.is_empty()
    }

    /// Whether selecting a bar opens a period rather than picking a version.
    pub fn opens(&self) -> bool {
        self.unit.is_some()
    }
}

/// Build the axis for `moments` (oldest first, already narrowed to the window).
pub fn axis_for(moments: &[Moment]) -> Axis {
    if moments.len() <= MAX_BARS {
        return Axis {
            bars: moments
                .iter()
                .enumerate()
                .map(|(i, m)| Bar {
                    label: m.at.format("%Y-%m-%d %H:%M").to_string(),
                    bytes: m.bytes,
                    at: m.at,
                    moment: i,
                    span: None,
                    count: 1,
                })
                .collect(),
            unit: None,
        };
    }
    let unit = choose_unit(moments);
    Axis {
        bars: group(moments, unit),
        unit: Some(unit),
    }
}

/// The finest unit whose buckets still fit the band.
///
/// Finest, not coarsest: a writer looking for last Tuesday is better served by
/// forty days than by two months, and every step still lands inside [`MAX_BARS`].
/// Falls back to the coarsest when even months are too many — a decade-old
/// project with a backup an hour — because drawing 200 invisible bars is the
/// failure this whole module exists to prevent.
fn choose_unit(moments: &[Moment]) -> BucketUnit {
    for unit in BucketUnit::ASCENDING {
        if distinct_buckets(moments, unit) <= MAX_BARS {
            return unit;
        }
    }
    BucketUnit::Month
}

fn distinct_buckets(moments: &[Moment], unit: BucketUnit) -> usize {
    let mut last: Option<i64> = None;
    let mut n = 0;
    // `moments` is sorted, so distinct keys are consecutive runs — no set needed.
    for m in moments {
        let key = unit.key(&m.at);
        if last != Some(key) {
            n += 1;
            last = Some(key);
        }
    }
    n
}

/// Collapse `moments` into one bar per bucket, oldest first.
fn group(moments: &[Moment], unit: BucketUnit) -> Vec<Bar> {
    let mut bars: Vec<Bar> = Vec::new();
    let mut key: Option<i64> = None;
    for (i, m) in moments.iter().enumerate() {
        let k = unit.key(&m.at);
        match bars.last_mut() {
            Some(bar) if key == Some(k) => {
                // Later moments in the same bucket: the bucket's height, date and
                // representative moment are its newest, and `moments` is
                // oldest-first, so each one in turn wins.
                bar.bytes = m.bytes;
                bar.at = m.at;
                bar.moment = i;
                bar.count += 1;
                bar.span = bar.span.map(|(start, _)| (start, m.at));
            }
            _ => {
                key = Some(k);
                bars.push(Bar {
                    label: label_for(unit, m.at),
                    bytes: m.bytes,
                    at: m.at,
                    moment: i,
                    // Inclusive of both ends, and widened as the bucket fills.
                    // Built from the moments actually present rather than from
                    // the calendar bucket's true edges: zooming has to land on a
                    // span that contains something, and an empty February would
                    // otherwise be openable.
                    span: Some((m.at, m.at)),
                    count: 1,
                });
            }
        }
    }
    bars
}

/// What a period is called on the axis.
fn label_for(unit: BucketUnit, at: DateTime<Utc>) -> String {
    match unit {
        BucketUnit::Hour => at.format("%m-%d %H:00").to_string(),
        BucketUnit::Day => at.format("%Y-%m-%d").to_string(),
        // The week's own first recorded moment, not an ISO week number: a writer
        // knows what "the week of the 3rd" means and does not know what week 14 is.
        BucketUnit::Week => at.format("%Y-%m-%d").to_string(),
        BucketUnit::Month => at.format("%Y-%m").to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use skrib_format::versions::{SourceKind, VersionRef};
    use std::path::PathBuf;

    fn moment(at: DateTime<Utc>, bytes: u64) -> Moment {
        Moment {
            at,
            source: SourceKind::Backup,
            from: VersionRef {
                path: PathBuf::from("/x.skrib"),
                taken_at: at,
                source: SourceKind::Backup,
            },
            bytes,
        }
    }

    /// `n` moments spread evenly over `days`, oldest first.
    fn spread(n: usize, days: i64) -> Vec<Moment> {
        let start = DateTime::from_timestamp(1_700_000_000, 0).unwrap();
        (0..n)
            .map(|i| {
                let at =
                    start + chrono::Duration::seconds(days * 86_400 * i as i64 / n.max(1) as i64);
                moment(at, 1000 + i as u64)
            })
            .collect()
    }

    #[test]
    fn a_short_history_draws_its_moments_and_nothing_is_collapsed() {
        let axis = axis_for(&spread(12, 30));
        assert_eq!(axis.bars.len(), 12);
        assert!(!axis.opens(), "twelve moments are choosable directly");
        assert_eq!(
            axis.bars.iter().map(|b| b.moment).collect::<Vec<_>>(),
            (0..12).collect::<Vec<_>>(),
        );
    }

    /// Every bar — period or moment — has to name a real moment, or the change
    /// list beside the axis has nothing to compare against while zoomed out.
    #[test]
    fn a_period_is_represented_by_its_newest_moment() {
        let moments = spread(300, 730);
        let axis = axis_for(&moments);
        assert!(axis.opens(), "precondition: these are periods");
        for bar in &axis.bars {
            let m = moments.get(bar.moment).expect("a bar names a real moment");
            assert_eq!(m.at, bar.at, "'{}' points at the wrong moment", bar.label);
            let (_, end) = bar.span.expect("a period carries its span");
            assert_eq!(
                end, m.at,
                "a period ends on the moment it is represented by"
            );
        }
    }

    /// **The bug this exists for.** 300 backups over two years drew an empty band:
    /// the row's spacing alone exceeded its width, so every bar laid out to zero.
    #[test]
    fn two_years_of_backups_fit_the_band() {
        let axis = axis_for(&spread(300, 730));
        assert!(
            axis.bars.len() <= MAX_BARS,
            "{} bars is more than the band can draw",
            axis.bars.len(),
        );
        assert!(axis.opens(), "at that density a bar has to be a period");
        assert_eq!(
            axis.bars.iter().map(|b| b.count).sum::<usize>(),
            300,
            "every moment has to be inside exactly one period",
        );
    }

    /// The floor that makes "open this period" terminate.
    #[test]
    fn opening_a_period_eventually_reaches_the_versions_themselves() {
        let mut moments = spread(300, 730);
        let mut guard = 0;
        loop {
            let axis = axis_for(&moments);
            if !axis.opens() {
                break;
            }
            guard += 1;
            assert!(guard < 10, "zooming never bottomed out");
            let (start, end) = axis.bars[0].span.expect("a period carries its span");
            moments.retain(|m| m.at >= start && m.at <= end);
            assert!(!moments.is_empty(), "a period must contain what it counted");
        }
        assert!(
            moments.len() <= MAX_BARS,
            "the terminal state is one bar per moment",
        );
    }

    /// The finest unit that fits, so a writer gets the most resolution the band
    /// can actually draw.
    #[test]
    fn the_unit_is_the_finest_one_that_fits() {
        // Ten days, several backups a day: days fit, hours would not.
        assert_eq!(choose_unit(&spread(200, 10)), BucketUnit::Day);
        // Two years: months.
        assert_eq!(choose_unit(&spread(300, 730)), BucketUnit::Month);
    }

    /// A period's height is what the project had reached by its end — a number it
    /// really was — never a mean of what it passed through.
    #[test]
    fn a_periods_height_is_the_state_it_ended_on() {
        let day = DateTime::from_timestamp(1_700_000_000, 0).unwrap();
        let moments = vec![
            moment(day, 100),
            moment(day + chrono::Duration::minutes(10), 500),
            moment(day + chrono::Duration::minutes(20), 300),
        ];
        let bars = group(&moments, BucketUnit::Day);
        assert_eq!(bars.len(), 1);
        assert_eq!(
            bars[0].bytes, 300,
            "the last state of the day, not the peak"
        );
        assert_eq!(bars[0].count, 3);
    }

    /// A period's span has to contain every moment counted into it, or zooming
    /// lands somewhere with nothing in it.
    #[test]
    fn a_periods_span_covers_everything_it_counted() {
        let moments = spread(300, 730);
        let axis = axis_for(&moments);
        for bar in &axis.bars {
            let (start, end) = bar.span.expect("a period carries its span");
            let inside = moments
                .iter()
                .filter(|m| m.at >= start && m.at <= end)
                .count();
            assert_eq!(
                inside, bar.count,
                "'{}' counted {} but spans {}",
                bar.label, bar.count, inside
            );
        }
    }

    #[test]
    fn an_empty_history_produces_an_empty_axis() {
        let axis = axis_for(&[]);
        assert!(axis.is_empty());
        assert!(!axis.opens());
    }

    /// The band and the backup sweep must agree about what a week is, or a bar
    /// and a retention tier describe different sets.
    #[test]
    fn the_units_are_retentions_own_bucket_keys() {
        let a = DateTime::from_timestamp(1_700_000_000, 0).unwrap();
        let b = a + chrono::Duration::hours(1);
        assert_ne!(BucketUnit::Hour.key(&a), BucketUnit::Hour.key(&b));
        assert_eq!(BucketUnit::Day.key(&a), BucketUnit::Day.key(&b));
    }
}
