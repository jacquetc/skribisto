// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Scrivener's dates: `2025-03-14 22:15:28 -0600`, and the days of its writing history.
//!
//! # One form, read strictly
//!
//! Every format 1.0, 1.5 and 2.0 file, on every platform and on iOS, writes an instant as
//! `YYYY-MM-DD HH:MM:SS ±HHMM`: over 42,000 dates in the measured corpus, every one of
//! them, with offsets from `-1000` to `+1300`. The one exception is a custom date field's
//! value, which Scrivener 3 for Mac writes with a fraction of a second
//! (`2026-10-05 15:02:00.00000 +0200`), and which is read through [`field_date`] alone. The
//! only other shapes ever seen (`2024-01-15T10:00:00Z`) come from a third-party tool's
//! hand-made fixtures. So the reader accepts exactly that grammar (surrounding white space
//! aside) and nothing close to it: a calendar date that exists, a time of day under
//! twenty-four hours with no leap second, an offset under a day, and an instant that is
//! still in the years 0 to 9999 in UTC, the only ones the app writes. Whatever else a file
//! holds keeps its text in the model, so the report can say a date was not read rather than
//! lose it quietly; the converter dates such a row with the import's own instant instead
//! (D6.9), since one unconverted date in a `.skrib` refuses the whole project on load.
//!
//! # Instants and days
//!
//! An instant is stored as the app stores every instant: RFC 3339 in UTC ([`rfc3339`]). A
//! *day* (a writing-history day, a deadline) is the calendar date the writer saw, in the
//! offset Scrivener wrote it with ([`local_date`]): converting through UTC first moves a
//! date written just after midnight east of Greenwich to the day before, which the corpus
//! does to one deadline in seven.

use chrono::{DateTime, Datelike, FixedOffset, NaiveDate, NaiveTime, TimeZone};

use crate::model::Stamp;

/// Read a date as Scrivener writes it.
pub(crate) fn stamp(raw: &str) -> Stamp {
    match parse(raw) {
        Some(at) => Stamp {
            raw: raw.to_string(),
            unix_seconds: Some(at.timestamp()),
            offset_minutes: i16::try_from(at.offset().local_minus_utc() / 60).ok(),
        },
        None => Stamp {
            raw: raw.to_string(),
            unix_seconds: None,
            offset_minutes: None,
        },
    }
}

/// Read an optional date attribute.
pub(crate) fn stamp_opt(raw: Option<&str>) -> Option<Stamp> {
    raw.map(stamp)
}

/// `YYYY-MM-DD HH:MM:SS ±HHMM`, and nothing else: the instant, in its own offset.
pub(crate) fn parse(raw: &str) -> Option<DateTime<FixedOffset>> {
    let b = raw.trim().as_bytes();
    if b.len() != 25 {
        return None;
    }
    let shape_holds = b.iter().enumerate().all(|(i, c)| match i {
        4 | 7 => *c == b'-',
        10 | 19 => *c == b' ',
        13 | 16 => *c == b':',
        20 => *c == b'+' || *c == b'-',
        _ => c.is_ascii_digit(),
    });
    if !shape_holds {
        return None;
    }
    let number = |from: usize, to: usize| -> u32 {
        b[from..to]
            .iter()
            .fold(0u32, |n, d| n * 10 + u32::from(d - b'0'))
    };
    let year = i32::try_from(number(0, 4)).ok()?;
    let date = NaiveDate::from_ymd_opt(year, number(5, 7), number(8, 10))?;
    let (hour, minute, second) = (number(11, 13), number(14, 16), number(17, 19));
    // `NaiveTime` would take a sixtieth second as a leap second; Scrivener never writes one.
    if hour > 23 || minute > 59 || second > 59 {
        return None;
    }
    let time = NaiveTime::from_hms_opt(hour, minute, second)?;
    let (offset_hours, offset_minutes) = (number(21, 23), number(23, 25));
    if offset_hours > 23 || offset_minutes > 59 {
        return None;
    }
    let seconds = i32::try_from(offset_hours * 3600 + offset_minutes * 60).ok()?;
    let offset = if b[20] == b'-' {
        FixedOffset::west_opt(seconds)?
    } else {
        FixedOffset::east_opt(seconds)?
    };
    let at = offset.from_local_datetime(&date.and_time(time)).single()?;
    // Stored, the instant is written in UTC with four digits of year: one that the offset
    // carries before year 0 or past 9999 would be written as no RFC 3339 reader takes it,
    // and would refuse the whole project on load.
    (0..=9999).contains(&at.naive_utc().year()).then_some(at)
}

/// A custom date field's value: Scrivener's one form, with an optional fraction of a second
/// after the seconds (`2026-10-05 15:02:00.00000 +0200`), which is dropped.
pub(crate) fn field_date(raw: &str) -> Option<DateTime<FixedOffset>> {
    let raw = raw.trim();
    match (raw.get(..19), raw.get(19..)) {
        (Some(head), Some(rest)) if rest.starts_with('.') => {
            let (fraction, offset) = rest[1..].split_once(' ')?;
            if fraction.is_empty() || !fraction.bytes().all(|b| b.is_ascii_digit()) {
                return None;
            }
            parse(&format!("{head} {offset}"))
        }
        _ => parse(raw),
    }
}

/// The instant a readable stamp names, as the app stores instants: RFC 3339, in UTC.
/// `None` for an instant outside the years the app writes, whatever produced the stamp.
pub(crate) fn rfc3339(stamp: &Stamp) -> Option<String> {
    DateTime::from_timestamp(stamp.unix_seconds?, 0)
        .filter(|at| (0..=9999).contains(&at.year()))
        .map(|at| at.to_rfc3339())
}

/// The calendar date a readable stamp names in the offset it was written with: the day the
/// writer saw, never the day it was in UTC.
pub(crate) fn local_date(stamp: &Stamp) -> Option<NaiveDate> {
    let offset = FixedOffset::east_opt(i32::from(stamp.offset_minutes?) * 60)?;
    let at = DateTime::from_timestamp(stamp.unix_seconds?, 0)?;
    Some(at.with_timezone(&offset).date_naive())
}

/// A day as the writing history writes it: `YYYY-MM-DD`, a date that exists, nothing else.
pub(crate) fn parse_day(raw: &str) -> Option<NaiveDate> {
    let b = raw.trim().as_bytes();
    if b.len() != 10
        || !b.iter().enumerate().all(|(i, c)| match i {
            4 | 7 => *c == b'-',
            _ => c.is_ascii_digit(),
        })
    {
        return None;
    }
    let text = std::str::from_utf8(b).ok()?;
    NaiveDate::parse_from_str(text, "%Y-%m-%d").ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_scrivener_date_keeps_its_instant_and_its_offset() {
        let at = stamp("2025-03-14 22:15:28 -0600");
        assert_eq!(at.unix_seconds, Some(1_742_012_128));
        assert_eq!(at.offset_minutes, Some(-360));
        assert_eq!(
            rfc3339(&at).as_deref(),
            Some("2025-03-15T04:15:28+00:00"),
            "stored as the app stores an instant: UTC"
        );
    }

    #[test]
    fn every_offset_the_corpus_holds_reads() {
        for (raw, minutes) in [
            ("2019-10-21 23:16:18 -0500", -300),
            ("2019-10-21 23:16:18 +0530", 330),
            ("2019-10-21 23:16:18 +1300", 780),
            ("2019-10-21 23:16:18 -1000", -600),
            ("2019-10-21 23:16:18 +0000", 0),
            ("2019-10-21 23:16:18 -0000", 0),
        ] {
            assert_eq!(stamp(raw).offset_minutes, Some(minutes), "{raw}");
        }
    }

    #[test]
    fn an_unreadable_date_keeps_its_text() {
        let at = stamp("yesterday");
        assert_eq!(at.raw, "yesterday");
        assert_eq!(at.unix_seconds, None);
        assert_eq!(rfc3339(&at), None);
    }

    /// The forms a third-party tool or a damaged file writes, and dates that do not exist:
    /// none is taken for a date, so none reaches the project unconverted.
    #[test]
    fn only_scriveners_own_form_is_read() {
        for raw in [
            "2024-01-15T10:00:00Z",
            "2024-01-15 10:00:00",
            "2024-01-15 10:00:00 +01:00",
            "2024-01-15 10:00:00 GMT",
            "2024-01-15  10:00:00 +0100",
            "2024-1-15 10:00:00 +0100",
            "2024-02-30 10:00:00 +0100",
            "2023-02-29 10:00:00 +0100",
            "2024-13-01 10:00:00 +0100",
            "2024-01-15 24:00:00 +0100",
            "2024-01-15 10:60:00 +0100",
            "2024-01-15 23:59:60 +0100",
            "2024-01-15 10:00:00 +2400",
            "2024-01-15 10:00:00 +0160",
            "２０２４-01-15 10:00:00 +0100",
            "",
        ] {
            assert_eq!(parse(raw), None, "{raw:?}");
        }
        assert!(parse("2024-02-29 10:00:00 +0100").is_some(), "a leap day");
        assert!(
            parse("  2024-01-15 10:00:00 +0100\n").is_some(),
            "white space around it"
        );
    }

    /// The first and the last years Scrivener's four digits can write, in an offset that
    /// carries the instant past them in UTC: stored, the instant would read year -1 or 10000,
    /// which no RFC 3339 reader takes, and one such date refuses the whole project on load.
    /// Such a date is not read; everything inside the years stays read.
    #[test]
    fn a_date_whose_instant_the_app_cannot_store_is_not_read() {
        for raw in [
            "0000-01-01 00:30:00 +0100",
            "0000-01-01 00:00:00 +0001",
            "9999-12-31 23:30:00 -0100",
            "9999-12-31 23:59:59 -0001",
        ] {
            assert_eq!(parse(raw), None, "{raw}");
            assert_eq!(rfc3339(&stamp(raw)), None, "{raw}");
        }
        // A stamp the model carries from elsewhere is held to the same years.
        let before_year_zero = Stamp {
            raw: String::new(),
            unix_seconds: Some(-62_167_219_201),
            offset_minutes: Some(0),
        };
        assert_eq!(rfc3339(&before_year_zero), None);
        for raw in [
            "0000-01-01 00:00:00 +0000",
            "0000-01-01 00:30:00 -0100",
            "9999-12-31 23:59:59 +0000",
            "9999-12-31 23:30:00 +0100",
        ] {
            let stored = rfc3339(&stamp(raw));
            assert!(
                stored
                    .as_deref()
                    .is_some_and(|s| DateTime::parse_from_rfc3339(s).is_ok()),
                "{raw} is stored as {stored:?}"
            );
        }
    }

    /// A custom date field's value carries a fraction of a second (Scrivener 3.5.2 for Mac,
    /// M10); nothing else does, and a fraction that is not digits is no date.
    #[test]
    fn a_date_fields_value_reads_with_its_fraction_of_a_second() {
        let day = |raw: &str| field_date(raw).map(|at| at.date_naive());
        assert_eq!(
            day("2026-10-05 15:02:00.00000 +0200"),
            NaiveDate::from_ymd_opt(2026, 10, 5)
        );
        assert_eq!(
            day("2026-10-23 15:52:00 +0200"),
            NaiveDate::from_ymd_opt(2026, 10, 23)
        );
        assert_eq!(
            field_date("2026-10-05 23:30:00.5 -0700").map(|at| at.timestamp()),
            parse("2026-10-05 23:30:00 -0700").map(|at| at.timestamp())
        );
        for unread in [
            "2026-10-05 15:02:00. +0200",
            "2026-10-05 15:02:00.12x +0200",
            "2026-10-05",
            "next spring",
        ] {
            assert_eq!(field_date(unread), None, "{unread}");
        }
        // The stamps every other file writes stay as strict as ever.
        assert_eq!(parse("2026-10-05 15:02:00.00000 +0200"), None);
    }

    /// Just after midnight in Paris is still the day before in UTC: the day the writer saw is
    /// the one kept.
    #[test]
    fn a_day_is_the_one_the_writer_saw() {
        let at = stamp("2019-10-21 00:30:00 +0200");
        assert_eq!(local_date(&at), NaiveDate::from_ymd_opt(2019, 10, 21));
        assert_eq!(
            rfc3339(&at).as_deref(),
            Some("2019-10-20T22:30:00+00:00"),
            "the instant itself is the previous day in UTC"
        );
        let west = stamp("2019-10-21 23:30:00 -0700");
        assert_eq!(local_date(&west), NaiveDate::from_ymd_opt(2019, 10, 21));
        assert_eq!(local_date(&stamp("not a date")), None);
    }

    #[test]
    fn a_writing_history_day_is_a_date_and_nothing_else() {
        assert_eq!(
            parse_day("2021-05-19"),
            NaiveDate::from_ymd_opt(2021, 5, 19)
        );
        for raw in [
            "2021-5-19",
            "2021-02-30",
            "19/05/2021",
            "2021-05-19 00:00",
            "",
        ] {
            assert_eq!(parse_day(raw), None, "{raw:?}");
        }
    }

    proptest::proptest! {
        /// Any instant Scrivener could write reads back to itself, in its own offset, and
        /// its RFC 3339 form is the same instant.
        #[test]
        fn every_written_instant_reads_back(
            secs in -2_000_000_000i64..4_000_000_000i64,
            offset_quarters in -48i32..=56,
        ) {
            let offset = FixedOffset::east_opt(offset_quarters * 15 * 60).unwrap();
            let at = DateTime::from_timestamp(secs, 0).unwrap().with_timezone(&offset);
            let raw = at.format("%Y-%m-%d %H:%M:%S %z").to_string();
            let read = stamp(&raw);
            proptest::prop_assert_eq!(read.unix_seconds, Some(at.timestamp()));
            proptest::prop_assert_eq!(read.offset_minutes, i16::try_from(offset_quarters * 15).ok());
            proptest::prop_assert_eq!(local_date(&read), Some(at.date_naive()));
            let stored = rfc3339(&read).unwrap_or_default();
            let back = DateTime::parse_from_rfc3339(&stored).map(|d| d.timestamp()).ok();
            proptest::prop_assert_eq!(back, Some(at.timestamp()));
        }
    }
}
