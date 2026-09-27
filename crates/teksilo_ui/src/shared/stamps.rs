// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Rendering a stored timestamp for the writer to read.
//!
//! Every instant the app keeps is UTC, and stays UTC in the store and on disk: a
//! history entry, a backup's manifest, a trash record, a recent project. Every
//! instant it *shows* is on the writer's own clock. This module is the one place
//! that crosses from the first to the second, so no surface can label a moment
//! on one calendar while another surface, or a date filter, reads the same
//! moment on a different one.
//!
//! That is not hypothetical. The Versions dock, the Timeline, the Trash, the
//! Backups list, the project switcher, the Welcome screen and both restore
//! confirmations each formatted their `DateTime<Utc>` directly, and the backup
//! choice card and Settings ▸ Backup printed the stored RFC 3339 text as it was,
//! so a writer in Tokyo who saved at breakfast saw the version listed under the
//! previous day, at a time their own clock never showed.
//!
//! ## Today
//!
//! A writing plan is kept in calendar days, and [`today`] is the writer's. The
//! Pace planner's streak, days left and earliest deadline are measured against
//! it, and a progress snapshot is filed under it. Those days are *stored* as
//! midnight UTC (see `crate::date_convert`), but that is only how a calendar
//! day is written down: which day it is has to be the writer's, or a writer in
//! Los Angeles has their evening's words credited to the next day.
//!
//! ## Which zone
//!
//! [`Zone::writer`] is the machine's own time zone, whatever `TZ` or the system
//! setting names, read through jiff. jiff rather than `chrono::Local` because a
//! date filter also asks the reverse question, "when did this day begin here",
//! and jiff answers it with daylight saving handled: a day is not always 24 hours
//! long, and in a few zones it does not even begin at midnight.
//!
//! In this crate's own unit tests the writer's zone is UTC unless a test sets
//! another one with `override_writer_zone`, so no test depends on the zone of
//! the machine that runs it.

use chrono::{DateTime, Datelike, NaiveDate, NaiveDateTime, Timelike, Utc};
use jiff::tz::TimeZone;

/// A time zone a stored instant can be read in.
///
/// Surfaces take [`Zone::writer`]. Anything that groups or filters by calendar
/// day takes a `&Zone` instead of reaching for the writer's own, so it can be
/// tested against a zone whose days are not UTC's.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Zone(TimeZone);

impl Zone {
    /// The writer's own clock: the zone this machine is set to.
    ///
    /// Asked for on every call rather than fixed at launch. jiff keeps the
    /// machine's answer for five minutes, so a writer who changes zone with the
    /// app open sees the lists they open after that on the new clock. A surface
    /// that labels many rows asks once and reuses the answer.
    pub fn writer() -> Self {
        writer_zone()
    }

    /// Coordinated Universal Time: the clock everything is stored on.
    pub fn utc() -> Self {
        Zone(TimeZone::UTC)
    }

    /// The wall-clock reading of `t` in this zone.
    ///
    /// The conversion is the whole point, and it is easy to get wrong in a way
    /// that looks right: `DateTime<Utc>::naive_local` is a no-op, because chrono
    /// reads "local" as local to the `Tz` parameter and UTC's offset is zero by
    /// definition. Calling it reads like a conversion and performs none, which
    /// is how every timestamp in the app once came out in UTC while claiming to
    /// be the writer's own clock.
    ///
    /// Degrades to the UTC reading for an instant jiff cannot represent (a year
    /// beyond 9999 either way). That is a corrupt stamp rather than a moment
    /// anyone wrote, and showing something beats showing nothing.
    pub fn civil(&self, t: DateTime<Utc>) -> NaiveDateTime {
        // chrono carries a leap second as a nanosecond count past one billion;
        // jiff has no leap seconds, so it reads as the last instant before one.
        let nanos = i32::try_from(t.timestamp_subsec_nanos().min(999_999_999)).unwrap_or(0);
        let Ok(ts) = jiff::Timestamp::new(t.timestamp(), nanos) else {
            return t.naive_utc();
        };
        let dt = self.0.to_datetime(ts);
        naive_of(dt).unwrap_or_else(|| t.naive_utc())
    }

    /// The calendar day `t` falls on in this zone, as the date widgets speak it.
    ///
    /// `None` only for a year the widgets cannot hold, which is a corrupt stamp.
    pub fn date(&self, t: DateTime<Utc>) -> Option<jiff::civil::Date> {
        crate::date_convert::naive_to_jiff(self.civil(t).date())
    }

    /// Today, on this zone's calendar.
    pub fn today(&self) -> Option<jiff::civil::Date> {
        self.date(Utc::now())
    }

    /// Every instant from the start of `first` to the end of `last`, both days
    /// inclusive, on this zone's calendar.
    ///
    /// Both ends come from the zone's own rules rather than from 24-hour
    /// arithmetic: the day daylight saving starts is 23 hours long, the day it
    /// ends is 25, and in a zone that makes the change at midnight the day does
    /// not start at midnight at all. The end is the last nanosecond before the
    /// next day begins, because a history entry is stamped to the nanosecond and
    /// one saved in the last second of a day belongs to that day.
    ///
    /// `None` when either day is outside what an instant can hold.
    pub fn day_span(
        &self,
        first: jiff::civil::Date,
        last: jiff::civil::Date,
    ) -> Option<(DateTime<Utc>, DateTime<Utc>)> {
        let start = self.day_start(first)?;
        let next = self.day_start(last.tomorrow().ok()?)?;
        Some((start, next - chrono::Duration::nanoseconds(1)))
    }

    /// The first instant of `day` in this zone.
    ///
    /// `Date::to_zoned` resolves midnight with jiff's "compatible" rule: a
    /// midnight skipped by a spring-forward change moves to the first instant
    /// after the gap, and a midnight that happens twice takes the first of the
    /// two. Either way that is the moment the day began.
    fn day_start(&self, day: jiff::civil::Date) -> Option<DateTime<Utc>> {
        let ts = day.to_zoned(self.0.clone()).ok()?.timestamp();
        DateTime::from_timestamp(ts.as_second(), u32::try_from(ts.subsec_nanosecond()).ok()?)
    }

    /// `DD/MM/YYYY HH:MM` in this zone.
    pub fn stamp(&self, t: DateTime<Utc>) -> String {
        let t = self.civil(t);
        format!(
            "{:02}/{:02}/{:04} {:02}:{:02}",
            t.day(),
            t.month(),
            t.year(),
            t.hour(),
            t.minute()
        )
    }

    /// `YYYY-MM-DD HH:MM` in this zone: the shape every history surface uses.
    pub fn iso_stamp(&self, t: DateTime<Utc>) -> String {
        self.format(t, "%Y-%m-%d %H:%M")
    }

    /// `YYYY-MM-DD` in this zone.
    pub fn iso_day(&self, t: DateTime<Utc>) -> String {
        self.format(t, "%Y-%m-%d")
    }

    /// Any other shape, in this zone: `pattern` is a chrono format string.
    ///
    /// For a label whose shape belongs to its surface, such as a Timeline
    /// period's `%Y-%m`. `'static` because a pattern chrono cannot parse fails
    /// when the label is rendered rather than when it is written, and only a
    /// literal in the source can be checked by a test that renders it.
    pub fn format(&self, t: DateTime<Utc>, pattern: &'static str) -> String {
        self.civil(t).format(pattern).to_string()
    }
}

#[cfg(not(test))]
fn writer_zone() -> Zone {
    Zone(TimeZone::system())
}

#[cfg(test)]
fn writer_zone() -> Zone {
    WRITER_OVERRIDE
        .with(|o| o.borrow().clone())
        .unwrap_or_else(Zone::utc)
}

/// A jiff civil datetime as chrono's, for the formatting chrono already does.
fn naive_of(dt: jiff::civil::DateTime) -> Option<NaiveDateTime> {
    NaiveDate::from_ymd_opt(
        i32::from(dt.year()),
        u32::try_from(dt.month()).ok()?,
        u32::try_from(dt.day()).ok()?,
    )?
    .and_hms_nano_opt(
        u32::try_from(dt.hour()).ok()?,
        u32::try_from(dt.minute()).ok()?,
        u32::try_from(dt.second()).ok()?,
        u32::try_from(dt.subsec_nanosecond()).ok()?,
    )
}

/// `DD/MM/YYYY HH:MM` on the writer's clock.
///
/// This lived as two byte-identical copies in the outline card and the comment
/// card before it lived here.
pub fn stamp(t: DateTime<Utc>) -> String {
    Zone::writer().stamp(t)
}

/// `YYYY-MM-DD HH:MM` on the writer's clock.
pub fn iso_stamp(t: DateTime<Utc>) -> String {
    Zone::writer().iso_stamp(t)
}

/// `YYYY-MM-DD` on the writer's clock.
pub fn iso_day(t: DateTime<Utc>) -> String {
    Zone::writer().iso_day(t)
}

/// Today, on the writer's calendar.
///
/// For every "today" a plan is measured against: the day a progress snapshot
/// is filed under, and the Pace planner's streak, days left and earliest
/// deadline. A writing plan is kept in the writer's days, so on UTC's calendar a
/// writer in Los Angeles had their evening's words credited to the next day,
/// and one in Tokyo wrote all morning on a day the planner said had not begun.
pub fn today() -> NaiveDate {
    Zone::writer().civil(Utc::now()).date()
}

/// [`iso_stamp`] for an instant kept as RFC 3339 text, as a backup's manifest
/// and the backup settings file keep theirs.
///
/// `None` when the text is not RFC 3339. A caller then shows the text as it is:
/// it is still the best account of the moment there is, and dropping it would
/// say there was none.
pub fn iso_stamp_from_rfc3339(text: &str) -> Option<String> {
    DateTime::parse_from_rfc3339(text.trim())
        .ok()
        .map(|t| iso_stamp(t.with_timezone(&Utc)))
}

#[cfg(test)]
thread_local! {
    /// The zone [`Zone::writer`] answers with on this thread, when a test set one.
    ///
    /// Per thread because the test harness runs each test on a thread of its
    /// own: one test's zone can never reach another's.
    static WRITER_OVERRIDE: std::cell::RefCell<Option<Zone>> =
        const { std::cell::RefCell::new(None) };
}

/// Puts back the writer's zone a test replaced, when it goes out of scope.
#[cfg(test)]
#[must_use = "the override ends when the guard is dropped"]
pub(crate) struct WriterZoneGuard {
    previous: Option<Zone>,
}

#[cfg(test)]
impl Drop for WriterZoneGuard {
    fn drop(&mut self) {
        let previous = self.previous.take();
        WRITER_OVERRIDE.with(|o| *o.borrow_mut() = previous);
    }
}

/// Make [`Zone::writer`] answer `zone` on this thread until the guard drops.
///
/// Only ever needed where the code under test reads the writer's zone itself.
/// Code that takes a `&Zone` is simply handed one.
#[cfg(test)]
pub(crate) fn override_writer_zone(zone: Zone) -> WriterZoneGuard {
    let previous = WRITER_OVERRIDE.with(|o| o.borrow_mut().replace(zone));
    WriterZoneGuard { previous }
}

/// Zones for tests, spelled as fixed offsets or POSIX rules so they need no time
/// zone database: a test must give the same answer on a machine that has none.
#[cfg(test)]
impl Zone {
    /// Tokyo: nine hours ahead of UTC all year, so any evening in UTC is
    /// already the next morning there.
    pub(crate) fn tokyo() -> Self {
        Zone(TimeZone::fixed(jiff::tz::offset(9)))
    }

    /// More than a day ahead of UTC, so its date is never UTC's date: 25 hours,
    /// further than any real zone and still within what jiff accepts. For a test
    /// about "today" that must not depend on the hour it happens to run at.
    pub(crate) fn a_day_ahead() -> Self {
        Zone(TimeZone::fixed(jiff::tz::offset(25)))
    }

    /// New York: five hours behind UTC in winter, four in summer.
    pub(crate) fn new_york() -> Self {
        Self::posix("EST5EDT,M3.2.0,M11.1.0")
    }

    /// Paris: one hour ahead of UTC in winter, two in summer, changing at
    /// 02:00 local time on the last Sunday of March and 03:00 on the last
    /// Sunday of October.
    pub(crate) fn paris() -> Self {
        Self::posix("CET-1CEST,M3.5.0,M10.5.0/3")
    }

    /// São Paulo under its 2018 rules, which started daylight saving *at
    /// midnight*: on 4 November 2018 the clocks went from 23:59:59 on the 3rd
    /// straight to 01:00 on the 4th, so that day had no midnight.
    pub(crate) fn sao_paulo_2018() -> Self {
        Self::posix("<-03>3<-02>,M11.1.0/0,M2.3.0/0")
    }

    /// The instant a wall-clock reading names in this zone, for building a
    /// test's moments on the writer's clock rather than on UTC's.
    pub(crate) fn instant_of(&self, civil: jiff::civil::DateTime) -> DateTime<Utc> {
        let ts = match civil.to_zoned(self.0.clone()) {
            Ok(z) => z.timestamp(),
            Err(e) => panic!("{civil} has no instant in {self:?}: {e}"),
        };
        match DateTime::from_timestamp(ts.as_second(), ts.subsec_nanosecond().unsigned_abs()) {
            Some(t) => t,
            None => panic!("{civil} is outside what chrono can hold"),
        }
    }

    fn posix(rule: &str) -> Self {
        match TimeZone::posix(rule) {
            Ok(tz) => Zone(tz),
            Err(e) => panic!("the test zone '{rule}' does not parse: {e}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn at(rfc3339: &str) -> DateTime<Utc> {
        DateTime::parse_from_rfc3339(rfc3339)
            .expect("a valid test instant")
            .with_timezone(&Utc)
    }

    fn day(y: i16, m: i8, d: i8) -> jiff::civil::Date {
        jiff::civil::date(y, m, d)
    }

    /// **The defect this module exists for.** A version saved at 08:30 in Tokyo
    /// is stamped 23:30 the previous evening in UTC, and read without a
    /// conversion it was listed under the wrong day.
    #[test]
    fn a_stamp_reads_on_the_writers_clock_not_on_utc() {
        let saved = at("2026-03-03T23:30:00Z");
        let tokyo = Zone::tokyo();
        assert_eq!(tokyo.iso_stamp(saved), "2026-03-04 08:30");
        assert_eq!(tokyo.iso_day(saved), "2026-03-04");
        assert_eq!(tokyo.stamp(saved), "04/03/2026 08:30");
        assert_eq!(Zone::utc().iso_stamp(saved), "2026-03-03 23:30");
    }

    /// A fixed offset would get half the year wrong: the same wall-clock hour
    /// sits at a different UTC distance either side of a daylight-saving change.
    #[test]
    fn a_stamp_follows_daylight_saving() {
        let paris = Zone::paris();
        assert_eq!(
            paris.iso_stamp(at("2026-01-15T12:00:00Z")),
            "2026-01-15 13:00"
        );
        assert_eq!(
            paris.iso_stamp(at("2026-07-15T12:00:00Z")),
            "2026-07-15 14:00"
        );
        // Either side of the spring change, 29 March 2026 at 01:00 UTC.
        assert_eq!(
            paris.iso_stamp(at("2026-03-29T00:59:00Z")),
            "2026-03-29 01:59"
        );
        assert_eq!(
            paris.iso_stamp(at("2026-03-29T01:00:00Z")),
            "2026-03-29 03:00"
        );
    }

    /// The free functions read the writer's zone, which a test can set, and
    /// which is UTC in this crate's tests unless one does.
    #[test]
    fn the_free_functions_read_the_writers_zone() {
        let saved = at("2026-03-03T23:30:00Z");
        assert_eq!(
            iso_stamp(saved),
            "2026-03-03 23:30",
            "UTC by default in tests"
        );
        {
            let _tokyo = override_writer_zone(Zone::tokyo());
            assert_eq!(iso_stamp(saved), "2026-03-04 08:30");
            assert_eq!(iso_day(saved), "2026-03-04");
            assert_eq!(stamp(saved), "04/03/2026 08:30");
        }
        assert_eq!(
            iso_stamp(saved),
            "2026-03-03 23:30",
            "the override ends with its guard",
        );
    }

    /// A writing plan's "today" is the writer's day. In a zone more than a day
    /// ahead of UTC it is never UTC's day, whatever the hour.
    #[test]
    fn today_is_the_writers_day() {
        let _ahead = override_writer_zone(Zone::a_day_ahead());
        assert!(today() > Utc::now().date_naive());
    }

    /// A backup's manifest and the backup settings keep RFC 3339 text, and that
    /// text used to reach the screen as it was stored: offset, seconds,
    /// nanoseconds and all.
    #[test]
    fn stored_rfc3339_text_is_shown_on_the_writers_clock() {
        let _tokyo = override_writer_zone(Zone::tokyo());
        assert_eq!(
            iso_stamp_from_rfc3339("2026-03-03T23:30:12.123456789+00:00").as_deref(),
            Some("2026-03-04 08:30"),
        );
        assert_eq!(iso_stamp_from_rfc3339("not a date"), None);
    }

    #[test]
    fn a_calendar_day_is_the_writers_day() {
        let evening_utc = at("2026-03-03T20:00:00Z");
        assert_eq!(Zone::tokyo().date(evening_utc), Some(day(2026, 3, 4)));
        assert_eq!(Zone::new_york().date(evening_utc), Some(day(2026, 3, 3)));
        assert_eq!(Zone::utc().date(evening_utc), Some(day(2026, 3, 3)));
    }

    /// A day's span starts at the zone's own midnight, not UTC's.
    #[test]
    fn a_day_span_runs_from_local_midnight_to_local_midnight() {
        let (start, end) = Zone::tokyo()
            .day_span(day(2026, 3, 4), day(2026, 3, 4))
            .expect("an ordinary day");
        assert_eq!(start, at("2026-03-03T15:00:00Z"));
        assert_eq!(
            end,
            at("2026-03-04T15:00:00Z") - chrono::Duration::nanoseconds(1),
            "the last nanosecond before the next day begins, so a stamp in the \
             last second of the day still belongs to it",
        );
    }

    /// The day daylight saving starts is 23 hours long and the day it ends is
    /// 25. Adding 24 hours to a midnight gets both of them wrong.
    #[test]
    fn a_day_span_honours_the_short_and_the_long_day() {
        let paris = Zone::paris();
        let (start, end) = paris
            .day_span(day(2026, 3, 29), day(2026, 3, 29))
            .expect("the spring change");
        assert_eq!(start, at("2026-03-28T23:00:00Z"));
        assert_eq!(
            end + chrono::Duration::nanoseconds(1),
            at("2026-03-29T22:00:00Z")
        );
        assert_eq!((end - start).num_minutes(), 23 * 60 - 1);

        let (start, end) = paris
            .day_span(day(2026, 10, 25), day(2026, 10, 25))
            .expect("the autumn change");
        assert_eq!(start, at("2026-10-24T22:00:00Z"));
        assert_eq!(
            end + chrono::Duration::nanoseconds(1),
            at("2026-10-25T23:00:00Z")
        );
        assert_eq!((end - start).num_minutes(), 25 * 60 - 1);
    }

    /// Where daylight saving starts at midnight, the day begins at 01:00. A
    /// filter that asked for midnight would either fail or start an hour late.
    #[test]
    fn a_day_with_no_midnight_starts_when_its_first_hour_does() {
        let (start, _) = Zone::sao_paulo_2018()
            .day_span(day(2018, 11, 4), day(2018, 11, 4))
            .expect("a day with no midnight still has a start");
        assert_eq!(start, at("2018-11-04T03:00:00Z"), "01:00 at UTC-2");
        assert_eq!(Zone::sao_paulo_2018().iso_stamp(start), "2018-11-04 01:00",);
    }

    /// **The drift guard.** Every timestamp the writer reads goes through this
    /// module; no other file formats one itself, and no code outside a test
    /// reads a moment or a "today" on UTC's clock.
    ///
    /// A site that formats a `DateTime<Utc>` on its own does not fail loudly. It
    /// prints a plausible date and time, in UTC, and it looks right to anyone who
    /// lives there, so it survives review and ships: nine surfaces did exactly
    /// that. A directory walk rather than a fixed list, for the reason
    /// `identity`'s drift guards use one: the case to catch is a new surface in
    /// a file nobody thought to check.
    #[test]
    fn no_module_formats_a_timestamp_for_display_itself() {
        use std::path::{Path, PathBuf};

        fn walk(dir: &Path, out: &mut Vec<PathBuf>) {
            let Ok(entries) = std::fs::read_dir(dir) else {
                return;
            };
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    walk(&path, out);
                } else if path.extension().is_some_and(|e| e == "rs") {
                    out.push(path);
                }
            }
        }

        // A file name, not a label: the safety copy a restore leaves behind is
        // stamped in UTC like every backup, and is read back by
        // `retention::parse_stamp_from_filename` rather than by a writer.
        const FILE_NAME_STAMP: &str = ".format(\"%Y%m%d-%H%M%S\")";

        let src = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
        let mut files = Vec::new();
        walk(&src, &mut files);

        // Reading on UTC's clock without formatting anything: a site handed
        // `Zone::utc()`, or a "today" taken from UTC's calendar, is the same
        // defect in a spelling the formatting needles do not match. Tests read
        // on UTC on purpose, so these are looked for outside them only.
        const UTC_READS: [&str; 4] = [
            "Zone::utc()",
            "now().date_naive()",
            "now.date_naive()",
            "today_utc()",
        ];
        // The two places UTC's today is the point: its definition, and the
        // update check's once-a-day gate, which must not move with the zone.
        const UTC_TODAY_ON_PURPOSE: [&str; 2] = ["/date_convert.rs", "/updates/update_vm.rs"];

        let mut offenders = Vec::new();
        for file in files {
            // This module is the one place allowed to, so it necessarily holds
            // the scanner's own needles.
            if file.ends_with("shared/stamps.rs") {
                continue;
            }
            let Ok(text) = std::fs::read_to_string(&file) else {
                continue;
            };
            let mut flag = |index: usize| {
                let line = text[..index].matches('\n').count() + 1;
                offenders.push(format!("{}:{line}", file.display()));
            };
            for needle in [".format(\"%", ".naive_local()", ".strftime("] {
                for (index, _) in text.match_indices(needle) {
                    if !text[index..].starts_with(FILE_NAME_STAMP) {
                        flag(index);
                    }
                }
            }
            let path = file.to_string_lossy().replace('\\', "/");
            // A file that is tests throughout, or else the items in it that only
            // a test build compiles.
            if path.ends_with("tests.rs") {
                continue;
            }
            let test_only = test_only_ranges(&text);
            for needle in UTC_READS {
                if needle == "today_utc()" && UTC_TODAY_ON_PURPOSE.iter().any(|p| path.ends_with(p))
                {
                    continue;
                }
                for (index, _) in text.match_indices(needle) {
                    if !test_only.iter().any(|r| r.contains(&index)) {
                        flag(index);
                    }
                }
            }
        }

        assert!(
            offenders.is_empty(),
            "these sites format a timestamp themselves instead of going through \
             `shared::stamps`, or read it on UTC's clock, so they show it on \
             UTC's clock rather than the writer's:\n  {}\n\n\
             Use `stamps::iso_stamp`, `stamps::iso_day` or `stamps::stamp`, \
             `stamps::today` for a day a plan is measured against, or take a \
             `&stamps::Zone` where the code groups or filters by calendar day.",
            offenders.join("\n  ")
        );
    }

    /// The byte ranges of `text` that only a test build compiles: every item
    /// under `#[cfg(test)]` or `#[cfg(all(test, …))]`, from its attribute to
    /// its end.
    ///
    /// The item, and not the rest of the file from there on. A test module is
    /// often declared near the top of a file (`#[cfg(test)] mod tests;`, the
    /// tests living in a file of their own) with the file's production code
    /// after it, and an inline one can be followed by production code too.
    /// Cutting at the first test module left all of that unchecked.
    fn test_only_ranges(text: &str) -> Vec<std::ops::Range<usize>> {
        const TEST_ONLY: [&str; 2] = ["#[cfg(test)]", "#[cfg(all(test,"];
        let b = text.as_bytes();
        let mut ranges = Vec::new();
        let mut i = 0;
        while i < b.len() {
            if let Some(next) = past_literal_or_comment(b, i) {
                i = next;
            } else if TEST_ONLY.iter().any(|a| b[i..].starts_with(a.as_bytes())) {
                let end = item_end(b, i);
                ranges.push(i..end);
                i = end;
            } else {
                i += 1;
            }
        }
        ranges
    }

    /// Where the item whose first attribute starts at `start` ends: after its
    /// `;`, after its `{ … }` body, after the `,` that ends a field or a match
    /// arm, or before the bracket that closes the scope it sits in.
    ///
    /// A `,` between angle brackets (`-> Result<(), String>`) is a generic
    /// argument's, not the item's end. Angle brackets are only counted up to
    /// the item's first `=`: after one they are comparisons.
    fn item_end(b: &[u8], start: usize) -> usize {
        let mut depth = 0usize;
        let mut angles = 0usize;
        let mut signature = true;
        let mut i = start;
        while i < b.len() {
            if let Some(next) = past_literal_or_comment(b, i) {
                i = next;
                continue;
            }
            match b[i] {
                b'{' if depth == 0 => return block_end(b, i),
                b'(' | b'[' | b'{' => depth += 1,
                b')' | b']' | b'}' if depth == 0 => return i,
                b')' | b']' | b'}' => depth -= 1,
                b'<' if depth == 0 && signature => angles += 1,
                // Not the `>` of an `->`.
                b'>' if depth == 0 && signature && b[i - 1] != b'-' => {
                    angles = angles.saturating_sub(1);
                }
                b'=' if depth == 0 => {
                    signature = false;
                    angles = 0;
                }
                b';' | b',' if depth == 0 && angles == 0 => return i + 1,
                _ => {}
            }
            i += 1;
        }
        b.len()
    }

    /// Just past the `}` that closes the `{` at `open`.
    fn block_end(b: &[u8], open: usize) -> usize {
        let mut depth = 0usize;
        let mut i = open;
        while i < b.len() {
            if let Some(next) = past_literal_or_comment(b, i) {
                i = next;
                continue;
            }
            match b[i] {
                b'{' => depth += 1,
                b'}' => {
                    depth -= 1;
                    if depth == 0 {
                        return i + 1;
                    }
                }
                _ => {}
            }
            i += 1;
        }
        b.len()
    }

    /// Just past the comment, string or character literal that starts at `i`,
    /// if one does: a brace or a `;` inside one is not the code's.
    fn past_literal_or_comment(b: &[u8], i: usize) -> Option<usize> {
        let at = |j: usize| b.get(j).copied();
        let ident = |c: u8| c.is_ascii_alphanumeric() || c == b'_';
        match b[i] {
            b'/' if at(i + 1) == Some(b'/') => Some(
                b[i..]
                    .iter()
                    .position(|&c| c == b'\n')
                    .map_or(b.len(), |n| i + n),
            ),
            b'/' if at(i + 1) == Some(b'*') => {
                // Block comments nest.
                let mut depth = 0usize;
                let mut j = i;
                while j + 1 < b.len() {
                    if b[j] == b'/' && b[j + 1] == b'*' {
                        depth += 1;
                        j += 2;
                    } else if b[j] == b'*' && b[j + 1] == b'/' {
                        depth -= 1;
                        j += 2;
                        if depth == 0 {
                            return Some(j);
                        }
                    } else {
                        j += 1;
                    }
                }
                Some(b.len())
            }
            b'"' => {
                let mut j = i + 1;
                while j < b.len() {
                    match b[j] {
                        b'\\' => j += 2,
                        b'"' => return Some(j + 1),
                        _ => j += 1,
                    }
                }
                Some(b.len())
            }
            // `r"…"`, `r#"…"#` and their `br` forms, but not the `r` ending an
            // identifier, nor a raw identifier such as `r#type`.
            b'r' if i == 0
                || !ident(b[i - 1])
                || (b[i - 1] == b'b' && (i < 2 || !ident(b[i - 2]))) =>
            {
                let hashes = b[i + 1..].iter().take_while(|&&c| c == b'#').count();
                if at(i + 1 + hashes) != Some(b'"') {
                    return None;
                }
                let mut close = vec![b'"'];
                close.extend(std::iter::repeat_n(b'#', hashes));
                let body = i + 2 + hashes;
                Some(
                    b[body..]
                        .windows(close.len())
                        .position(|w| w == close.as_slice())
                        .map_or(b.len(), |n| body + n + close.len()),
                )
            }
            // A character literal, as opposed to a lifetime or a loop label:
            // `'{'`, `'\''` and `'é'` are literals, `'a` and `'outer` are not.
            b'\'' => {
                if at(i + 1) == Some(b'\\') {
                    let mut j = i + 3;
                    while j < b.len() && b[j] != b'\'' {
                        j += 1;
                    }
                    return Some(j + 1);
                }
                let width = match at(i + 1)? {
                    0x00..=0x7F => 1,
                    0xC0..=0xDF => 2,
                    0xE0..=0xEF => 3,
                    _ => 4,
                };
                (at(i + 1 + width) == Some(b'\'')).then_some(i + 2 + width)
            }
            _ => None,
        }
    }

    /// The lines of `text` where the guard looks for `needle`: every one that
    /// holds it, less those inside an item only a test build compiles.
    fn checked_lines<'a>(text: &'a str, needle: &str) -> Vec<&'a str> {
        let test_only = test_only_ranges(text);
        text.match_indices(needle)
            .filter(|(index, _)| !test_only.iter().any(|r| r.contains(index)))
            .map(|(index, _)| {
                let start = text[..index].rfind('\n').map_or(0, |n| n + 1);
                let end = text[index..].find('\n').map_or(text.len(), |n| index + n);
                text[start..end].trim()
            })
            .collect()
    }

    /// The guard reads a file's production code to its last line. It used to
    /// stop at the file's first test module, and in the Inspector, the Help
    /// window and the crate root that is a `mod tests;` declaration near the
    /// top, with hundreds of lines of production code after it that the guard
    /// never looked at. The same went for the production code a file keeps
    /// after an inline test module, such as the real Corkboard cards model.
    #[test]
    fn the_guard_reads_the_production_code_either_side_of_a_test_module() {
        let text = r##"use super::*;

#[cfg(test)]
mod tests;

fn after_a_declared_module() -> Zone { Zone::utc() }

#[cfg(test)]
mod inline {
    // A brace in a literal or a comment is not the module's: }
    fn t() { let _ = ('{', "}}", r#"{"#, '\'', '}', 'é'); Zone::utc(); }
}

fn after_an_inline_module() -> Zone { Zone::utc() }

#[cfg(all(test, feature = "mocks"))]
mod mocks_only { fn t() -> Zone { Zone::utc() } }

#[cfg(not(test))]
fn production_only() -> Zone { Zone::utc() }

fn one_arm_is_a_tests(x: u8) -> Zone {
    match x {
        #[cfg(test)]
        0 => Zone::utc(),
        #[cfg(test)]
        n if n < 3 => Zone::utc(),
        _ => Zone::utc(),
    }
}

#[cfg(test)]
fn a_helper() -> Result<(), String> { Zone::utc(); Ok(()) }

fn after_a_helper() -> Zone { Zone::utc() }
"##;
        assert_eq!(
            checked_lines(text, "Zone::utc()"),
            [
                "fn after_a_declared_module() -> Zone { Zone::utc() }",
                "fn after_an_inline_module() -> Zone { Zone::utc() }",
                "fn production_only() -> Zone { Zone::utc() }",
                "_ => Zone::utc(),",
                "fn after_a_helper() -> Zone { Zone::utc() }",
            ],
        );
    }
}
