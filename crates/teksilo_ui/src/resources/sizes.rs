// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The two sizes the Resources feature asks about.
//!
//! A stored file travels in the project, in every backup and in every copy of it; a
//! linked one stays where it is. Neither is wrong, and the writer cannot see the
//! difference at the moment of dropping a recording on a scene, so a large file asks
//! first ([`LARGE_FILE_BYTES`]).
//!
//! A project kept as one file is rewritten whole at each save, resources included, so
//! past [`ZIP_NOTICE_BYTES`] of stored files it is offered once the shape that does not
//! rewrite them: a folder project, which saves only what changed.
//!
//! Binary units, so the figure the question shows (`shared::units::byte_size`, also
//! binary) is the round number the threshold is.
//!
//! Not a view-model: two numbers and the tests that pin them.

/// A file larger than this asks whether to store it or link to it: 10 MB.
pub const LARGE_FILE_BYTES: u64 = 10 * 1024 * 1024;

/// Stored resources in a single-file project past this get the folder-project notice:
/// 100 MB.
pub const ZIP_NOTICE_BYTES: u64 = 100 * 1024 * 1024;

/// Whether a file of `bytes` is large enough to ask about.
pub fn is_large(bytes: u64) -> bool {
    bytes > LARGE_FILE_BYTES
}

/// Whether adding stored files took a single-file project from `before` to `after`
/// bytes of stored resources across the notice's threshold.
///
/// Crossing, not "is past": a project already past it when the writer said Not now is
/// not asked again on every later file. It is asked again only if removing files takes
/// it back under and a later one takes it over once more.
pub fn crosses_zip_notice(before: u64, after: u64) -> bool {
    before <= ZIP_NOTICE_BYTES && after > ZIP_NOTICE_BYTES
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ten_megabytes_is_not_large_and_one_byte_more_is() {
        assert!(!is_large(LARGE_FILE_BYTES));
        assert!(is_large(LARGE_FILE_BYTES + 1));
        assert!(!is_large(0));
        assert_eq!(LARGE_FILE_BYTES, 10_485_760);
    }

    #[test]
    fn the_notice_fires_on_crossing_a_hundred_megabytes_only() {
        assert_eq!(ZIP_NOTICE_BYTES, 104_857_600);
        assert!(crosses_zip_notice(90 << 20, 101 << 20));
        assert!(crosses_zip_notice(ZIP_NOTICE_BYTES, ZIP_NOTICE_BYTES + 1));
        assert!(!crosses_zip_notice(50 << 20, ZIP_NOTICE_BYTES));
        assert!(
            !crosses_zip_notice(120 << 20, 130 << 20),
            "already past: no second notice for every later file"
        );
    }
}
