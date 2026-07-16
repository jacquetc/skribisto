//! Bridge between the entities' `chrono::DateTime<Utc>` (the Pace/Milestone/Holiday dates
//! round-trip through the store as datetimes) and the bastyde date widgets'
//! `jiff::civil::Date` (a calendar day, no time-of-day).
//!
//! Convention: a Pace date is semantically a *calendar day*, but the entity layer stores a
//! full moment, so [`from_jiff_date`] normalises to **midnight UTC** — round-tripping a day
//! through the entity layer is then lossless for the day part. Conversions to jiff are
//! fallible (`jiff` years are `i16`); a corrupt out-of-range date reads as `None` (treated
//! as "no date"), never a panic.

use chrono::{DateTime, Datelike, NaiveDate, Utc};
use jiff::civil::Date;

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
}
