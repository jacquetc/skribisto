// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Is the release named by the feed newer than the build that is running?
//!
//! Pure, store-free and window-free, so every rule below is a table test rather
//! than something only a release could disprove.
//!
//! ## The four answers, and why there are four
//!
//! Three of them are obvious. The fourth, [`Verdict::Unknown`], is the one that
//! keeps this honest: a build whose own version does not parse cannot be
//! compared, and reporting "you are up to date" for it would be a surface
//! claiming something it cannot prove. It says nothing instead.
//!
//! ## The case that decides the design
//!
//! [`crate::version::app_version`] renders a build made *past* a tag as
//! `3.0.1+5c62c8cac`. Semver's precedence rules say **build metadata is
//! ignored**, so `3.0.1+5c62c8cac` compares *equal* to `3.0.1`. That is exactly
//! the behaviour wanted here and it falls out for free: a developer running a
//! build newer than the last release is told nothing, because their version is
//! not less than the feed's.
//!
//! It also gets the neighbouring case right without a special rule. A build at
//! `3.0.0+31commits` against a feed at `3.0.1` really is behind a release it
//! does not contain, and semver says so.
//!
//! ## Pre-releases
//!
//! A tester running `3.1.0-rc1` is *ahead* of a stable `3.0.1`, because semver
//! ranks a pre-release below its own final version but above every earlier one.
//! Nothing here has to know what "rc" means.

use semver::Version;

/// What the comparison can conclude.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Verdict {
    /// A newer release exists. Carries it, parsed.
    Behind(Version),
    /// The running build is the newest release, or newer than it.
    Current,
    /// One of the two versions could not be read, so nothing can be said.
    ///
    /// A build from a checkout with no tags renders as a bare commit hash, and a
    /// feed could be served by something that is not this project's website at
    /// all (a captive portal answering every request with a login page).
    Unknown,
}

impl Verdict {
    /// The newer release, when there is one.
    pub fn newer(&self) -> Option<&Version> {
        match self {
            Verdict::Behind(v) => Some(v),
            _ => None,
        }
    }
}

/// Compare the feed's version string against this build's.
///
/// Both are taken as strings rather than parsed values because both arrive as
/// strings from places that can be wrong: one from a file on a web server, the
/// other from `git describe` by way of a build script.
pub fn verdict(running: &str, latest: &str) -> Verdict {
    let (Some(running), Some(latest)) = (parse(running), parse(latest)) else {
        return Verdict::Unknown;
    };
    if latest > running {
        Verdict::Behind(latest)
    } else {
        Verdict::Current
    }
}

/// Read a version the way this project writes them.
///
/// Tolerates the leading `v` a tag carries, since the feed publishes both the
/// tag and the bare version and a hand-written one could use either.
fn parse(text: &str) -> Option<Version> {
    let text = text.trim();
    let text = text.strip_prefix('v').unwrap_or(text);
    Version::parse(text).ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn behind(running: &str, latest: &str) -> bool {
        matches!(verdict(running, latest), Verdict::Behind(_))
    }

    #[test]
    fn a_newer_release_is_reported() {
        assert!(behind("3.0.1", "3.0.2"));
        assert!(behind("3.0.1", "3.1.0"));
        assert!(behind("3.0.1", "4.0.0"));
        assert_eq!(
            verdict("3.0.1", "3.0.2").newer().map(ToString::to_string),
            Some("3.0.2".to_string())
        );
    }

    #[test]
    fn the_current_release_is_not_a_newer_one() {
        assert_eq!(verdict("3.0.1", "3.0.1"), Verdict::Current);
    }

    #[test]
    fn a_build_past_the_tag_is_never_told_it_is_behind() {
        // THE case this module exists for. `app_version()` renders a build made
        // after the v3.0.1 tag as "3.0.1+5c62c8cac"; semver ignores build
        // metadata in precedence, so it is equal to 3.0.1, not less than it.
        assert_eq!(verdict("3.0.1+5c62c8cac", "3.0.1"), Verdict::Current);
        assert!(!behind("3.0.1+5c62c8cac", "3.0.1"));
    }

    #[test]
    fn a_build_past_an_older_tag_is_still_behind_the_newer_release() {
        // The neighbouring case, which must NOT be swallowed by the rule above:
        // commits made after 3.0.0 do not contain 3.0.1.
        assert!(behind("3.0.0+31commits", "3.0.1"));
    }

    #[test]
    fn a_prerelease_ranks_below_its_own_final_version() {
        assert!(behind("3.0.0-rc1", "3.0.0"));
        assert!(behind("3.0.0-alpha7", "3.0.0-rc1"));
    }

    #[test]
    fn a_tester_ahead_of_the_stable_line_is_left_alone() {
        // Running a 3.1.0 candidate while the feed still says 3.0.1.
        assert_eq!(verdict("3.1.0-rc1", "3.0.1"), Verdict::Current);
    }

    #[test]
    fn an_unreadable_version_says_nothing_rather_than_guessing() {
        // A checkout with no tags renders as a bare commit hash.
        assert_eq!(verdict("5c62c8cac", "3.0.1"), Verdict::Unknown);
        // A captive portal answering with HTML that happened to parse as JSON.
        assert_eq!(verdict("3.0.1", "<!DOCTYPE html>"), Verdict::Unknown);
        assert_eq!(verdict("", "3.0.1"), Verdict::Unknown);
        assert_eq!(verdict("3.0.1", ""), Verdict::Unknown);
    }

    #[test]
    fn a_tag_shaped_version_is_read_the_same_as_a_bare_one() {
        assert_eq!(verdict("3.0.1", "v3.0.2"), verdict("3.0.1", "3.0.2"));
        assert_eq!(verdict("v3.0.1", "3.0.1"), Verdict::Current);
    }

    /// The real string this build produces has to be comparable, or the whole
    /// feature is silent in production and no unit test would show it.
    #[test]
    fn this_build_reports_a_version_the_comparison_can_read() {
        let running = crate::version::app_version();
        // A tagless checkout legitimately yields Unknown; anything else must not.
        if parse(&running).is_some() {
            assert_ne!(
                verdict(&running, "999.0.0"),
                Verdict::Unknown,
                "running version {running} parsed, so the comparison must reach a verdict"
            );
            assert!(behind(&running, "999.0.0"));
        }
    }
}
