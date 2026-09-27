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
//! The units come from [`BucketUnit`], which is retention's own GFS bucket key:
//! one definition of an hour, a day, a rolling week and a month, not a second
//! copy that could drift from the first.
//!
//! ## Whose calendar
//!
//! The writer's. Every bar is *labelled* on the writer's clock, and so is every
//! moment inside it once it is opened, so the band has to *group* on that clock
//! too. Grouped on UTC's, a save at 21:00 on the 3rd in New York (02:00 on the
//! 4th in UTC) landed in a bar whichever label it carried: called the 4th, it
//! held a moment stamped the 3rd; called the 3rd, it held one stamped the 4th.
//! So each moment is keyed by its wall-clock reading in the writer's zone, fed
//! to [`BucketUnit::key`] as if that reading were UTC, which keys it by the
//! writer's hours, days, weeks and months with retention's own arithmetic.
//!
//! Retention itself still buckets on UTC, and that is deliberate: it decides
//! which backup *files* survive a sweep and labels nothing, while the band
//! labels everything and decides nothing. The two therefore agree on what a day
//! *is* and may disagree on where a writer's evening falls between two of them,
//! which a writer can only ever see through the band.
//!
//! ## Why there is a terminal state
//!
//! Below [`MAX_BARS`] the axis stops bucketing and draws the moments themselves.
//! Without that floor "open this period" would recurse forever on a period
//! holding one backup, and the writer could never reach a version at all.

use chrono::{DateTime, Utc};

use skrib_format::retention::BucketUnit;

use super::timeline_vm::Moment;
use crate::shared::stamps::Zone;

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

/// Build the axis for `moments` (oldest first, already narrowed to the window),
/// labelled and grouped on `zone`'s calendar. The band passes the writer's.
pub fn axis_for(moments: &[Moment], zone: &Zone) -> Axis {
    if moments.len() <= MAX_BARS {
        return Axis {
            bars: moments
                .iter()
                .enumerate()
                .map(|(i, m)| Bar {
                    label: zone.iso_stamp(m.at),
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
    let unit = choose_unit(moments, zone);
    Axis {
        bars: group(moments, unit, zone),
        unit: Some(unit),
    }
}

/// The bucket `at` falls in on `zone`'s calendar.
///
/// The wall-clock reading, re-read as if it were UTC, is what makes
/// [`BucketUnit::key`] count the writer's hours and days rather than UTC's. See
/// the module docs.
fn bucket(unit: BucketUnit, zone: &Zone, at: DateTime<Utc>) -> i64 {
    unit.key(&zone.civil(at).and_utc())
}

/// The finest unit whose buckets still fit the band.
///
/// Finest, not coarsest: a writer looking for last Tuesday is better served by
/// forty days than by two months, and every step still lands inside [`MAX_BARS`].
/// Falls back to the coarsest when even months are too many — a decade-old
/// project with a backup an hour — because drawing 200 invisible bars is the
/// failure this whole module exists to prevent.
fn choose_unit(moments: &[Moment], zone: &Zone) -> BucketUnit {
    for unit in BucketUnit::ASCENDING {
        if distinct_buckets(moments, unit, zone) <= MAX_BARS {
            return unit;
        }
    }
    BucketUnit::Month
}

fn distinct_buckets(moments: &[Moment], unit: BucketUnit, zone: &Zone) -> usize {
    let mut last: Option<i64> = None;
    let mut n = 0;
    // `moments` is sorted, so distinct keys are consecutive runs — no set needed.
    // (On the writer's clock too: an autumn change repeats the hour the clock
    // has just left, so both passes through it are one run.)
    for m in moments {
        let key = bucket(unit, zone, m.at);
        if last != Some(key) {
            n += 1;
            last = Some(key);
        }
    }
    n
}

/// Collapse `moments` into one bar per bucket, oldest first.
fn group(moments: &[Moment], unit: BucketUnit, zone: &Zone) -> Vec<Bar> {
    let mut bars: Vec<Bar> = Vec::new();
    let mut key: Option<i64> = None;
    for (i, m) in moments.iter().enumerate() {
        let k = bucket(unit, zone, m.at);
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
                    label: label_for(unit, zone, m.at),
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

/// What a period is called on the axis, read off its first moment on the same
/// calendar the period was grouped on.
fn label_for(unit: BucketUnit, zone: &Zone, at: DateTime<Utc>) -> String {
    match unit {
        BucketUnit::Hour => zone.format(at, "%m-%d %H:00"),
        BucketUnit::Day => zone.iso_day(at),
        // The week's own first recorded moment, not an ISO week number: a writer
        // knows what "the week of the 3rd" means and does not know what week 14 is.
        BucketUnit::Week => zone.iso_day(at),
        BucketUnit::Month => zone.format(at, "%Y-%m"),
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
        let axis = axis_for(&spread(12, 30), &Zone::utc());
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
        let axis = axis_for(&moments, &Zone::utc());
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
        let axis = axis_for(&spread(300, 730), &Zone::utc());
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
            let axis = axis_for(&moments, &Zone::utc());
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
        assert_eq!(choose_unit(&spread(200, 10), &Zone::utc()), BucketUnit::Day);
        // Two years: months.
        assert_eq!(
            choose_unit(&spread(300, 730), &Zone::utc()),
            BucketUnit::Month
        );
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
        let bars = group(&moments, BucketUnit::Day, &Zone::utc());
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
        let axis = axis_for(&moments, &Zone::utc());
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
        let axis = axis_for(&[], &Zone::utc());
        assert!(axis.is_empty());
        assert!(!axis.opens());
    }

    /// The band counts hours, days and weeks with retention's own arithmetic,
    /// so the two can never disagree about what a week *is*, only about which
    /// clock it is read on (see the module docs).
    #[test]
    fn the_units_are_retentions_own_bucket_keys() {
        let a = DateTime::from_timestamp(1_700_000_000, 0).unwrap();
        let b = a + chrono::Duration::hours(1);
        assert_ne!(BucketUnit::Hour.key(&a), BucketUnit::Hour.key(&b));
        assert_eq!(BucketUnit::Day.key(&a), BucketUnit::Day.key(&b));
        assert_eq!(
            bucket(BucketUnit::Day, &Zone::utc(), a),
            BucketUnit::Day.key(&a),
            "on UTC's own clock the band's bucket is retention's, exactly",
        );
    }

    // ── the writer's calendar ──────────────────────────────────────────────

    fn at(rfc3339: &str) -> DateTime<Utc> {
        DateTime::parse_from_rfc3339(rfc3339)
            .expect("a valid test instant")
            .with_timezone(&Utc)
    }

    /// Each bar with the moments it stands for: bars are consecutive runs of the
    /// oldest-first moments, each ending on the moment it names.
    fn members<'a>(axis: &'a Axis, moments: &'a [Moment]) -> Vec<(&'a Bar, &'a [Moment])> {
        let mut first = 0;
        axis.bars
            .iter()
            .map(|bar| {
                let run = &moments[first..=bar.moment];
                first = bar.moment + 1;
                (bar, run)
            })
            .collect()
    }

    /// Every moment inside a period, read on `zone`'s clock the way the band
    /// shows it once the period is opened, has to belong to the period its
    /// label names.
    fn assert_every_bar_holds_only_its_own_label(
        axis: &Axis,
        moments: &[Moment],
        zone: &Zone,
        read: impl Fn(&Zone, DateTime<Utc>) -> String,
    ) {
        for (bar, run) in members(axis, moments) {
            for m in run {
                assert_eq!(
                    read(zone, m.at),
                    bar.label,
                    "the bar labelled '{}' holds a moment the band shows as {}",
                    bar.label,
                    zone.iso_stamp(m.at),
                );
            }
        }
    }

    /// **The defect.** A moment bar was labelled on UTC's clock, so a version
    /// saved at breakfast in Tokyo sat on the axis at 23:30 the evening before.
    #[test]
    fn a_moment_bar_reads_on_the_writers_clock() {
        let moments = vec![moment(at("2026-03-03T23:30:00Z"), 100)];
        let axis = axis_for(&moments, &Zone::tokyo());
        assert!(!axis.opens());
        assert_eq!(axis.bars[0].label, "2026-03-04 08:30");
    }

    /// A day bar is the writer's day. New York is five hours behind UTC in
    /// March, so its evenings are the next day in UTC: grouped by UTC's day, each
    /// bar held one evening and the following morning, and whichever of the two
    /// days it was labelled with, the other moment was wrong.
    #[test]
    fn a_day_bar_holds_the_writers_day_where_utc_has_already_moved_on() {
        let zone = Zone::new_york();
        // Thirty days of three saves: 10:00, 21:00 and 22:30, New York time. The
        // two evening saves are after midnight in UTC.
        let first_morning = at("2026-01-05T15:00:00Z"); // 10:00 EST
        let mut moments = Vec::new();
        for d in 0..30 {
            let morning = first_morning + chrono::Duration::days(d);
            moments.push(moment(morning, 100));
            moments.push(moment(morning + chrono::Duration::hours(11), 200));
            moments.push(moment(
                morning + chrono::Duration::minutes(12 * 60 + 30),
                300,
            ));
        }
        let axis = axis_for(&moments, &zone);
        assert_eq!(axis.unit, Some(BucketUnit::Day), "precondition: days fit");
        assert_eq!(axis.bars.len(), 30, "one bar per day the writer wrote on");
        assert_eq!(axis.bars[0].label, "2026-01-05");
        assert!(axis.bars.iter().all(|b| b.count == 3));
        assert_every_bar_holds_only_its_own_label(&axis, &moments, &zone, Zone::iso_day);
    }

    /// Across a daylight-saving change the offset moves, and a moment half an
    /// hour after midnight in Paris summer time is still the previous evening on
    /// a winter clock. The spring change of 2026 is on 29 March.
    #[test]
    fn a_day_bar_follows_the_writers_calendar_across_a_daylight_saving_change() {
        let zone = Zone::paris();
        // 00:30, 12:00 and 23:30 Paris time on each day from 20 March to 8 April.
        let mut moments = Vec::new();
        let mut day = jiff::civil::date(2026, 3, 20);
        for _ in 0..20 {
            for (h, m) in [(0, 30), (12, 0), (23, 30)] {
                moments.push(moment(zone.instant_of(day.at(h, m, 0, 0)), 100));
            }
            day = day.tomorrow().expect("a later day");
        }
        // The fixture has to be the writer's clock too, or it proves nothing.
        assert_eq!(zone.iso_stamp(moments[3 * 10].at), "2026-03-30 00:30");
        assert_eq!(
            moments[3 * 10].at,
            at("2026-03-29T22:30:00Z"),
            "past the change, 00:30 in Paris is 22:30 the evening before in UTC",
        );

        let axis = axis_for(&moments, &zone);
        assert_eq!(axis.unit, Some(BucketUnit::Day));
        assert_eq!(axis.bars.len(), 20);
        assert_eq!(axis.bars[10].label, "2026-03-30");
        assert_every_bar_holds_only_its_own_label(&axis, &moments, &zone, Zone::iso_day);
    }

    /// An hour bar is the writer's hour, and the hour the clocks go back is one
    /// bar: every moment in it reads 02:xx, so a label for either pass through
    /// it names both. On 25 October 2026 Paris runs 02:00 to 03:00 twice.
    #[test]
    fn an_hour_bar_is_the_writers_hour_across_the_autumn_change() {
        let zone = Zone::paris();
        // Every twenty minutes from 22:00 UTC on the 24th (midnight in Paris)
        // for fourteen real hours.
        let start = at("2026-10-24T22:00:00Z");
        let moments: Vec<Moment> = (0..42)
            .map(|i| moment(start + chrono::Duration::minutes(20 * i), 100))
            .collect();
        let axis = axis_for(&moments, &zone);
        assert_eq!(axis.unit, Some(BucketUnit::Hour), "precondition: hours fit");
        assert_eq!(
            axis.bars.len(),
            13,
            "fourteen real hours, one of them repeated on the wall clock",
        );
        let repeated = axis
            .bars
            .iter()
            .find(|b| b.label == "10-25 02:00")
            .expect("the repeated hour has a bar");
        assert_eq!(repeated.count, 6, "both passes through 02:00 land in it");
        assert_every_bar_holds_only_its_own_label(&axis, &moments, &zone, |z, t| {
            z.format(t, "%m-%d %H:00")
        });
    }

    /// Months are the writer's months: the last evening of January in New York
    /// is already February in UTC.
    #[test]
    fn a_month_bar_is_the_writers_month() {
        let zone = Zone::new_york();
        // Twelve years of one save a month, each at 21:00 on the last day of
        // the month in New York, which is the first of the next month in UTC.
        let mut moments = Vec::new();
        let mut month = jiff::civil::date(2014, 1, 1);
        for _ in 0..144 {
            let evening = month.last_of_month().at(21, 0, 0, 0);
            moments.push(moment(zone.instant_of(evening), 100));
            month = month
                .checked_add(jiff::ToSpan::months(1))
                .expect("a later month");
        }
        let axis = axis_for(&moments, &zone);
        assert_eq!(axis.unit, Some(BucketUnit::Month));
        assert_eq!(axis.bars[0].label, "2014-01");
        assert_every_bar_holds_only_its_own_label(&axis, &moments, &zone, |z, t| {
            z.format(t, "%Y-%m")
        });
    }
}
