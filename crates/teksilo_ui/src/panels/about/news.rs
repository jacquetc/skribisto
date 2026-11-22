// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! This version's release notes, read from the `NEWS.yml` compiled into the binary.
//!
//! The file is the one `scripts/news-to-metainfo.sh` turns into the software
//! centres' release notes, so About ▸ **What's new in this version** says what
//! Flathub says, and works offline. It is written in English and stays so: there is
//! no translation of it to show.
//!
//! The format is fixed by that script and by every block written so far: blocks
//! separated by `---`, each a `Version:`, a `Date:`, a `Type:`, then `Description:`
//! and one `- ` line per entry, most of them in double quotes. The newest block is on
//! top, and between two releases it is the *next* release, not the running one, so a
//! block is found by its version and never by its position.

/// The whole of `NEWS.yml`, as the build saw it.
const NEWS: &str = include_str!("../../../../../NEWS.yml");

/// The first year Skribisto was worked on: the date of the repository's first
/// commit, 2015-04-16.
pub(crate) const FIRST_YEAR: i32 = 2015;

/// One release's block.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct ReleaseNotes {
    pub version: String,
    /// `YYYY-MM-DD`, as written.
    pub date: String,
    pub entries: Vec<String>,
}

impl ReleaseNotes {
    /// The year of [`ReleaseNotes::date`], `None` for a date that does not start
    /// with one.
    pub(crate) fn year(&self) -> Option<i32> {
        self.date.get(..4)?.parse().ok()
    }

    /// The entries as the reader is shown them: one paragraph each, with a bullet,
    /// closed with the full stop the file leaves out (the software centres add their
    /// own).
    pub(crate) fn text(&self) -> String {
        self.entries
            .iter()
            .map(|entry| {
                let stop = if entry.ends_with(['.', '!', '?', '…', ':']) {
                    ""
                } else {
                    "."
                };
                format!("•\u{a0}{entry}{stop}")
            })
            .collect::<Vec<_>>()
            .join("\n\n")
    }
}

/// Every block of `news`, newest first. A block missing its version is skipped.
pub(crate) fn parse(news: &str) -> Vec<ReleaseNotes> {
    let mut releases = Vec::new();
    for block in news.split("\n---") {
        let mut version = None;
        let mut date = String::new();
        let mut entries = Vec::new();
        for line in block.lines() {
            let line = line.trim_end();
            if let Some(v) = line.strip_prefix("Version:") {
                version = Some(v.trim().to_string());
            } else if let Some(d) = line.strip_prefix("Date:") {
                date = d.trim().to_string();
            } else if let Some(entry) = line.strip_prefix("- ") {
                entries.push(unquote(entry.trim()));
            }
        }
        if let Some(version) = version.filter(|v| !v.is_empty()) {
            releases.push(ReleaseNotes {
                version,
                date,
                entries,
            });
        }
    }
    releases
}

/// An entry without the double quotes it is written in, and with YAML's two
/// escapes undone. An unquoted entry (the oldest blocks have one) is kept as is.
fn unquote(entry: &str) -> String {
    match entry
        .strip_prefix('"')
        .and_then(|rest| rest.strip_suffix('"'))
    {
        Some(inner) => inner.replace("\\\"", "\"").replace("\\\\", "\\"),
        None => entry.to_string(),
    }
}

/// The notes of `release` in `news`, if it has a block with at least one entry.
pub(crate) fn notes_in(news: &str, release: &str) -> Option<ReleaseNotes> {
    parse(news)
        .into_iter()
        .find(|notes| notes.version == release && !notes.entries.is_empty())
}

/// The notes of the running version, or `None` for a build that names no release
/// (a bare commit hash) or one `NEWS.yml` has no block for.
pub(crate) fn current_notes() -> Option<ReleaseNotes> {
    let version = crate::version::app_version();
    notes_in(NEWS, crate::version::release_of(&version)?)
}

/// The years of the copyright line in `news` for `release`: from [`FIRST_YEAR`] to
/// the year of the release's block, or of the newest block when there is none for it.
/// The year follows the release, not the clock, so a build says the same thing
/// whenever it is made.
pub(crate) fn copyright_years_in(news: &str, release: Option<&str>) -> String {
    let releases = parse(news);
    let last = release
        .and_then(|r| releases.iter().find(|notes| notes.version == r))
        .or_else(|| releases.first())
        .and_then(ReleaseNotes::year)
        .filter(|year| *year > FIRST_YEAR);
    match last {
        Some(year) => format!("{FIRST_YEAR}–{year}"),
        None => FIRST_YEAR.to_string(),
    }
}

/// [`copyright_years_in`] for the running version.
pub(crate) fn copyright_years() -> String {
    let version = crate::version::app_version();
    copyright_years_in(NEWS, crate::version::release_of(&version))
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r#"Version: 3.4.0
Date: 2027-01-10
Type: stable
Description:
- "Not released yet"
---
Version: 3.3.3
Date: 2026-11-19
Type: stable
Description:
- "Letters are as dark as in a web browser"
- "Settings has a new choice, \"Text contrast\""
---
Version: 1.0.0
Date: 2014-05-01
Type: stable
Description:
- First release on FlatHub
"#;

    #[test]
    fn the_running_release_is_found_by_its_version_not_its_place() {
        let notes = notes_in(SAMPLE, "3.3.3").expect("3.3.3 has a block");
        assert_eq!(notes.date, "2026-11-19");
        assert_eq!(
            notes.entries,
            vec![
                "Letters are as dark as in a web browser".to_string(),
                "Settings has a new choice, \"Text contrast\"".to_string(),
            ]
        );
    }

    #[test]
    fn an_unquoted_entry_is_kept_as_written() {
        let notes = notes_in(SAMPLE, "1.0.0").expect("1.0.0 has a block");
        assert_eq!(notes.entries, vec!["First release on FlatHub".to_string()]);
    }

    #[test]
    fn a_release_with_no_block_has_no_notes() {
        assert_eq!(notes_in(SAMPLE, "3.3.2"), None);
        assert_eq!(notes_in("", "3.3.3"), None);
    }

    #[test]
    fn each_entry_becomes_a_bulleted_paragraph() {
        let notes = notes_in(SAMPLE, "3.3.3").expect("3.3.3 has a block");
        assert_eq!(
            notes.text(),
            "•\u{a0}Letters are as dark as in a web browser.\n\n\
             •\u{a0}Settings has a new choice, \"Text contrast\"."
        );
    }

    #[test]
    fn the_copyright_runs_to_the_year_of_the_release() {
        assert_eq!(copyright_years_in(SAMPLE, Some("3.3.3")), "2015–2026");
        // A build that names no release takes the newest block's year.
        assert_eq!(copyright_years_in(SAMPLE, None), "2015–2027");
        assert_eq!(copyright_years_in(SAMPLE, Some("9.9.9")), "2015–2027");
        // No date to go by, or one before the first year: the first year alone.
        assert_eq!(copyright_years_in("", None), "2015");
        assert_eq!(copyright_years_in(SAMPLE, Some("1.0.0")), "2015");
    }

    /// The real file, so a block written in a shape the reader does not know shows
    /// up here rather than as an empty window: every block has a version, a date
    /// and at least one entry.
    #[test]
    fn every_block_of_the_bundled_news_reads() {
        let releases = parse(NEWS);
        assert!(
            releases.len() > 10,
            "NEWS.yml read as {} blocks",
            releases.len()
        );
        for notes in &releases {
            assert!(
                notes.year().is_some(),
                "{} has no dated block",
                notes.version
            );
            assert!(
                !notes.entries.is_empty(),
                "{} has no entries",
                notes.version
            );
            for entry in &notes.entries {
                assert!(
                    !entry.starts_with('"') && !entry.ends_with('"'),
                    "{}: an entry kept its quotes: {entry}",
                    notes.version
                );
            }
        }
        assert_eq!(
            releases.len(),
            NEWS.lines().filter(|l| l.starts_with("Version:")).count(),
            "a block was skipped"
        );
    }
}
