// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Which generation of the Manuskript format a project is, and the ceiling above
//! which this importer refuses to guess.
//!
//! The version is a **file, not a field**: a root member named `MANUSKRIPT` (or,
//! for a few weeks in 2016, `VERSION`) whose whole content is a bare decimal
//! number. No data file carries a version of its own.
//!
//! Manuskript's own dispatcher (`loadSave.py`) reads that number and branches
//! `if version == 0` to its format-0 loader, `else` to format 1 — with no upper
//! bound at all, so a marker holding `2` or `9999` is handed to the version-1
//! loader without a word. This importer does not copy that: a number above
//! [`TERMINAL`] is refused by name, because reading a schema we have never seen
//! with a reader written for an older one produces a project that looks imported
//! and is quietly wrong.
//!
//! Everything at or below the ceiling is read leniently. A missing marker means
//! format 0 (that is Manuskript's own rule for a zip with no marker), and a marker
//! that is not a number at all is treated the same way rather than refused — those
//! are the cases where being strict would reject a real project.

use anyhow::{Result, bail};

/// The newest format generation this importer understands.
///
/// `1` since Manuskript 0.3.0, March 2016, and unchanged through 0.17.0 — ten
/// years and nineteen releases. Every field added since (`customIcon` in 0.5.0,
/// `charCount` and `Character.pov` in 0.12.0) shipped without moving it, which is
/// why the parsers below sniff for a key rather than branch on this number.
pub const TERMINAL: u32 = 1;

/// Which generation a project is.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FormatVersion {
    /// Manuskript 0.1.0–0.2.0: one zip of XML, plus a `settings.pickle` this
    /// importer never deserializes.
    V0,
    /// Manuskript 0.3.0 onward: plain text and XML, as a folder or a zip.
    V1,
}

/// Resolve the marker's content to a generation.
///
/// `None` is "there was no marker", which format 0 is: Manuskript wrote none
/// before 0.3.0.
pub fn resolve(marker: Option<&str>) -> Result<FormatVersion> {
    let Some(raw) = marker else {
        return Ok(FormatVersion::V0);
    };
    let trimmed = raw.trim();
    let Ok(n) = trimmed.parse::<u32>() else {
        // Not a number. Manuskript raised an unhandled `ValueError` here until
        // 0.17.0 hardened it; there is nothing to gain from refusing a project
        // over a marker whose only job is to pick a reader, so fall back to the
        // reader a marker-less project would get.
        return Ok(FormatVersion::V0);
    };
    match n {
        0 => Ok(FormatVersion::V0),
        1 => Ok(FormatVersion::V1),
        _ => bail!(
            "this Manuskript project's format is version {n} — newer than this importer \
             supports ({TERMINAL})"
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn an_absent_marker_is_format_zero() {
        assert_eq!(resolve(None).ok(), Some(FormatVersion::V0));
    }

    #[test]
    fn the_two_real_markers_resolve() {
        assert_eq!(resolve(Some("0")).ok(), Some(FormatVersion::V0));
        assert_eq!(resolve(Some("1")).ok(), Some(FormatVersion::V1));
        // The marker file carries no trailing newline, but a hand-edited one might.
        assert_eq!(resolve(Some("1\n")).ok(), Some(FormatVersion::V1));
        assert_eq!(resolve(Some(" 1 ")).ok(), Some(FormatVersion::V1));
    }

    #[test]
    fn a_newer_numeric_marker_is_refused_by_name() {
        let err = match resolve(Some("2")) {
            Ok(v) => panic!("expected a refusal, got {v:?}"),
            Err(e) => e.to_string(),
        };
        assert!(err.contains('2'), "{err}");
        assert!(resolve(Some("9999")).is_err());
    }

    /// Unlike a newer number, an unreadable marker is not a reason to refuse a
    /// project — it is the case Manuskript itself crashed on until 0.17.0.
    #[test]
    fn an_unparsable_marker_falls_back_rather_than_failing() {
        assert_eq!(resolve(Some("")).ok(), Some(FormatVersion::V0));
        assert_eq!(resolve(Some("wat")).ok(), Some(FormatVersion::V0));
        assert_eq!(resolve(Some("1.0")).ok(), Some(FormatVersion::V0));
    }
}
