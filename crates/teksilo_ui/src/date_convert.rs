// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Bridge between the entities' `chrono::DateTime<Utc>` (the Pace/Milestone/Holiday dates
//! round-trip through the store as datetimes) and the teksilo date widgets'
//! `jiff::civil::Date` (a calendar day, no time-of-day).
//!
//! Convention: a Pace date is semantically a *calendar day*, but the entity layer stores a
//! full moment, so [`from_jiff_date`] normalises to **midnight UTC** - round-tripping a day
//! through the entity layer is then lossless for the day part. Conversions to jiff are
//! fallible (`jiff` years are `i16`); a corrupt out-of-range date reads as `None` (treated
//! as "no date"), never a panic.

use chrono::{DateTime, Datelike, NaiveDate, Utc};
use jiff::civil::Date;
use teksilo::widgets::DateRange;

/// Today, as the version surfaces date their rows.
///
/// **UTC**, deliberately, because that is the clock those surfaces already show:
/// a `Change.at` is a `DateTime<Utc>` and is formatted without conversion, so a
/// backup made at 23:00 in Berlin is listed under the previous day. Reading
/// "today" off the local clock here would filter by one calendar and label by
/// another, and a writer would watch a row they can see fall out of range.
/// (Whether the surfaces should show local time at all is a real question, and a
/// larger one than this preset.)
pub fn today_utc() -> Option<Date> {
    to_jiff_date(Utc::now())
}

/// A chrono UTC instant → a calendar date (drops time-of-day). `None` if the year is
/// outside jiff's `i16` range.
pub fn to_jiff_date(dt: DateTime<Utc>) -> Option<Date> {
    let year = i16::try_from(dt.year()).ok()?;
    Date::new(year, dt.month() as i8, dt.day() as i8).ok()
}

pub fn to_jiff_date_opt(dt: Option<DateTime<Utc>>) -> Option<Date> {
    dt.and_then(to_jiff_date)
}

/// A calendar date → a chrono UTC instant at **midnight**.
pub fn from_jiff_date(d: Date) -> DateTime<Utc> {
    NaiveDate::from_ymd_opt(d.year() as i32, d.month() as u32, d.day() as u32)
        .and_then(|nd| nd.and_hms_opt(0, 0, 0))
        .map(|ndt| ndt.and_utc())
        // A jiff `Date` is always a valid Gregorian day, so this never falls back.
        .unwrap_or_else(|| DateTime::<Utc>::from_timestamp(0, 0).unwrap())
}

pub fn from_jiff_date_opt(d: Option<Date>) -> Option<DateTime<Utc>> {
    d.map(from_jiff_date)
}

/// A `chrono::NaiveDate` → a jiff calendar date. `None` if the year is outside
/// jiff's `i16` range. (The Pace view-model speaks `NaiveDate`; the date widgets
/// speak jiff - this is the direct day-to-day bridge, no datetime hop.)
pub fn naive_to_jiff(nd: NaiveDate) -> Option<Date> {
    let year = i16::try_from(nd.year()).ok()?;
    Date::new(year, nd.month() as i8, nd.day() as i8).ok()
}

pub fn naive_to_jiff_opt(nd: Option<NaiveDate>) -> Option<Date> {
    nd.and_then(naive_to_jiff)
}

/// A jiff calendar date → a `chrono::NaiveDate` (always valid - a jiff `Date` is
/// a valid Gregorian day).
pub fn jiff_to_naive(d: Date) -> NaiveDate {
    NaiveDate::from_ymd_opt(d.year() as i32, d.month() as u32, d.day() as u32)
        .expect("a jiff Date is a valid Gregorian day")
}

/// The `days`-long range ending on `today`, **inclusive of both ends**.
///
/// `last_days(today, 30)` is today and the twenty-nine before it — thirty days,
/// counted the way someone reading "last 30 days" counts them, not
/// `today - 30` (which is thirty-one).
///
/// `DateRangeEdit` carries no open-ended range, so a preset like this is the
/// only way to express "recently" through it: the caller computes both ends and
/// sets them. `today` is a parameter rather than read from the clock here so the
/// arithmetic is testable; callers pass the same UTC day the version surfaces
/// date their rows by.
///
/// Saturates rather than wraps at the start of the calendar, which only matters
/// to a corrupt timestamp but must not panic.
pub fn last_days(today: Date, days: u32) -> DateRange {
    let back = jiff::Span::new().days(i64::from(days.saturating_sub(1)));
    let start = today.checked_sub(back).unwrap_or(today);
    DateRange::new(start, today)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn utc(y: i32, m: u32, d: u32, h: u32, min: u32) -> DateTime<Utc> {
        NaiveDate::from_ymd_opt(y, m, d)
            .unwrap()
            .and_hms_opt(h, min, 0)
            .unwrap()
            .and_utc()
    }

    #[test]
    fn round_trip_preserves_the_day_and_normalises_to_midnight() {
        let d = to_jiff_date(utc(2026, 7, 16, 14, 30)).unwrap();
        assert_eq!((d.year(), d.month(), d.day()), (2026, 7, 16));
        // Back to chrono: same day, midnight (the time-of-day is intentionally dropped).
        let back = from_jiff_date(d);
        assert_eq!(back, utc(2026, 7, 16, 0, 0));
    }

    #[test]
    fn options_thread_none() {
        assert_eq!(to_jiff_date_opt(None), None);
        assert_eq!(from_jiff_date_opt(None), None);
        assert!(to_jiff_date_opt(Some(utc(2020, 1, 1, 0, 0))).is_some());
    }

    #[test]
    fn out_of_range_year_is_none_not_a_panic() {
        // jiff years are i16; a corrupt far-future date can't convert.
        assert_eq!(to_jiff_date(utc(40000, 1, 1, 0, 0)), None);
    }

    /// Thirty days means thirty, counted the way the label is read.
    #[test]
    fn last_days_counts_today_as_one_of_them() {
        let today = Date::new(2026, 8, 7).unwrap();
        let range = last_days(today, 30);
        assert_eq!(range.end, today, "the range ends today, not yesterday");
        assert_eq!(
            range.start,
            Date::new(2026, 7, 9).unwrap(),
            "today plus the twenty-nine before it",
        );
        // …and the arithmetic says so too: both ends inclusive.
        let days = (range.start.until(range.end).unwrap().get_days()) + 1;
        assert_eq!(days, 30);
    }

    #[test]
    fn last_days_crosses_a_month_and_a_year_boundary() {
        let range = last_days(Date::new(2026, 1, 3).unwrap(), 30);
        assert_eq!(range.start, Date::new(2025, 12, 5).unwrap());
    }

    #[test]
    fn one_day_is_today_alone() {
        let today = Date::new(2026, 8, 7).unwrap();
        let range = last_days(today, 1);
        assert_eq!(range.start, today);
        assert_eq!(range.end, today);
    }

    /// A zero-day window is meaningless but must not underflow into a range that
    /// ends before it starts.
    #[test]
    fn zero_days_does_not_invert_the_range() {
        let today = Date::new(2026, 8, 7).unwrap();
        let range = last_days(today, 0);
        assert!(range.start <= range.end);
    }

    #[test]
    fn naive_jiff_round_trip() {
        let nd = NaiveDate::from_ymd_opt(2026, 7, 16).unwrap();
        let jd = naive_to_jiff(nd).unwrap();
        assert_eq!((jd.year(), jd.month(), jd.day()), (2026, 7, 16));
        assert_eq!(jiff_to_naive(jd), nd);
        assert_eq!(naive_to_jiff_opt(None), None);
        assert_eq!(
            naive_to_jiff(NaiveDate::from_ymd_opt(40000, 1, 1).unwrap()),
            None
        );
    }
}
