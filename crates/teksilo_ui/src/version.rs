// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The app version shown in the UI, derived from git at build time.
//!
//! `build.rs` stamps `git describe --tags --always` into `SKRIBISTO_GIT_DESCRIBE`;
//! this module turns that into what a reader wants to see:
//!
//! | build sits on            | `git describe`                | displayed          |
//! |-------------------------|-------------------------------|--------------------|
//! | the tag `v3.0.0-alpha1` | `v3.0.0-alpha1`               | `3.0.0-alpha1`     |
//! | 31 commits past it      | `v3.0.0-alpha1-31-g5c62c8cac` | `3.0.0-alpha1+5c62c8cac` |
//! | a repo with no tags     | `5c62c8cac`                   | `5c62c8cac`        |
//! | no git checkout at all  | (empty)                       | `CARGO_PKG_VERSION`|
//!
//! The commit suffix is the point: a build made *after* a release must not
//! present itself as that release, or a bug report naming "3.0.0-alpha1" is
//! untraceable to the code that produced it.

/// The version string for this build — `3.0.0-alpha1` on a tag,
/// `3.0.0-alpha1+5c62c8cac` anywhere else.
pub fn app_version() -> String {
    display_version(env!("SKRIBISTO_GIT_DESCRIBE"))
}

/// Pure rendering of a `git describe` output. Separated from [`app_version`] so
/// the rule is testable without a build-time environment.
fn display_version(describe: &str) -> String {
    let describe = describe.trim();
    if describe.is_empty() {
        return env!("CARGO_PKG_VERSION").to_string();
    }
    match split_describe(describe) {
        Some((tag, hash)) => format!("{}+{}", strip_v(tag), hash),
        // Either HEAD *is* the tag, or the repo has no tags and this is a bare
        // commit hash. Both display as-is.
        None => strip_v(describe).to_string(),
    }
}

/// Split `v3.0.0-alpha1-31-g5c62c8cac` into (`v3.0.0-alpha1`, `5c62c8cac`), or
/// `None` when `describe` carries no `-<distance>-g<hash>` suffix.
///
/// Tags themselves contain `-` (every pre-release one does), so this matches the
/// *shape* of the two segments git appends — a decimal distance and a `g`-prefixed
/// hex hash — rather than cutting at the first dash, which would read the tag
/// `v3.0.0-alpha1` as version `3.0.0` and lose the pre-release entirely.
fn split_describe(describe: &str) -> Option<(&str, &str)> {
    let (head, hash) = describe.rsplit_once('-')?;
    let hash = hash.strip_prefix('g')?;
    if hash.is_empty() || !hash.chars().all(|c| c.is_ascii_hexdigit()) {
        return None;
    }
    let (tag, distance) = head.rsplit_once('-')?;
    if tag.is_empty() || distance.is_empty() || !distance.chars().all(|c| c.is_ascii_digit()) {
        return None;
    }
    Some((tag, hash))
}

/// Drop the `v` a release tag is written with (`v3.0.0` → `3.0.0`), but only
/// when it prefixes a number — a tag named `vision-2` keeps its name.
fn strip_v(tag: &str) -> &str {
    tag.strip_prefix('v')
        .filter(|rest| rest.starts_with(|c: char| c.is_ascii_digit()))
        .unwrap_or(tag)
}

/// The build this binary is: what a translation report carries in its machine block
/// and what About ▸ **Copy version info** puts on the clipboard.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct BuildInfo {
    /// `3.3.1`, `3.3.1+5c62c8cac`, or a bare hash in a clone with no tags.
    pub version: String,
    /// The crate's own version, the fallback the triage tries when `version` names
    /// no tag.
    pub crate_version: String,
    /// The teksilo-widgets version, or `None` when the build could not tell.
    pub framework: Option<String>,
    /// [`crate::updates::Channel::stamp`].
    pub channel: &'static str,
    /// `linux`, `macos`, `windows`.
    pub os: &'static str,
}

impl BuildInfo {
    /// This binary.
    pub(crate) fn current() -> Self {
        Self {
            version: crate::version::app_version(),
            crate_version: env!("CARGO_PKG_VERSION").to_string(),
            framework: Some(env!("SKRIBISTO_TEKSILO_VERSION").to_string())
                .filter(|v| !v.is_empty()),
            channel: crate::updates::Channel::current().stamp(),
            os: std::env::consts::OS,
        }
    }

    /// The lines **Copy version info** puts on the clipboard, under `app`, the
    /// edition's display name: what a bug report needs to name the build, and
    /// nothing about the writer. In English whatever the interface language, like
    /// the report it is pasted into; the framework line is left out when the build
    /// could not tell it.
    pub(crate) fn summary(&self, app: &str) -> String {
        let mut lines = vec![format!("{app} {}", self.version)];
        if let Some(framework) = &self.framework {
            lines.push(format!("Teksilo {framework}"));
        }
        lines.push(format!("Channel: {}", self.channel));
        lines.push(format!("OS: {} {}", self.os, std::env::consts::ARCH));
        lines.join("\n")
    }
}

/// The release a displayed version belongs to: `3.3.1` for `3.3.1` and for a build
/// past it (`3.3.1+5c62c8cac`), `None` for a bare commit hash, which names no release.
pub(crate) fn release_of(version: &str) -> Option<&str> {
    let release = version.split_once('+').map_or(version, |(tag, _)| tag);
    release
        .starts_with(|c: char| c.is_ascii_digit())
        .then_some(release)
        .filter(|r| r.contains('.'))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_build_past_a_release_belongs_to_that_release() {
        assert_eq!(release_of("3.3.1"), Some("3.3.1"));
        assert_eq!(release_of("3.3.1+5c62c8cac"), Some("3.3.1"));
        assert_eq!(release_of("3.0.0-alpha1+5c62c8cac"), Some("3.0.0-alpha1"));
        // A clone with no tags shows a bare hash, which names no release, even one
        // that happens to start with a digit.
        assert_eq!(release_of("5c62c8cac"), None);
        assert_eq!(release_of("deadbee"), None);
    }

    #[test]
    fn the_version_summary_names_the_build_and_nothing_else() {
        let build = BuildInfo {
            version: "3.3.1+5c62c8cac".to_string(),
            crate_version: "3.0.0".to_string(),
            framework: Some("0.28.1".to_string()),
            channel: "flathub",
            os: "linux",
        };
        let arch = std::env::consts::ARCH;
        assert_eq!(
            build.summary("Skribisto"),
            format!(
                "Skribisto 3.3.1+5c62c8cac\nTeksilo 0.28.1\nChannel: flathub\nOS: linux {arch}"
            )
        );
        let unknown = BuildInfo {
            framework: None,
            ..build
        };
        assert!(
            !unknown.summary("Skribisto").contains("Teksilo"),
            "a framework the build could not tell is left out, not printed empty"
        );
    }

    #[test]
    fn on_a_tag_shows_the_tag_alone() {
        assert_eq!(display_version("v2.0.7"), "2.0.7");
        assert_eq!(display_version("v3.0.0-alpha1"), "3.0.0-alpha1");
        // A tag written without the conventional `v`.
        assert_eq!(display_version("2.0.7"), "2.0.7");
    }

    #[test]
    fn past_a_tag_appends_the_commit() {
        assert_eq!(display_version("v2.0.7-3-gdeadbee"), "2.0.7+deadbee");
        // The pre-release part of the tag survives: only the trailing
        // `-<distance>-g<hash>` is peeled off.
        assert_eq!(
            display_version("v3.0.0-alpha1-31-g5c62c8cac"),
            "3.0.0-alpha1+5c62c8cac"
        );
    }

    #[test]
    fn a_tagless_repo_shows_the_bare_hash() {
        // `git describe --always` with no tags reachable.
        assert_eq!(display_version("5c62c8cac"), "5c62c8cac");
    }

    #[test]
    fn a_tag_that_merely_looks_like_a_suffix_is_not_split() {
        // Not a `-<digits>-g<hex>` tail: the `g`-segment isn't hex...
        assert_eq!(display_version("v1.0.0-2-gzzz"), "1.0.0-2-gzzz");
        // ...and the distance isn't a number.
        assert_eq!(display_version("v1.0.0-rc-gabc"), "1.0.0-rc-gabc");
    }

    #[test]
    fn no_git_falls_back_to_the_cargo_version() {
        assert_eq!(display_version(""), env!("CARGO_PKG_VERSION"));
        assert_eq!(display_version("  \n"), env!("CARGO_PKG_VERSION"));
    }

    #[test]
    fn the_built_binary_carries_a_usable_version() {
        let v = app_version();
        assert!(!v.is_empty());
        assert!(
            !v.contains(char::is_whitespace),
            "displayed as one token: {v}"
        );
    }
}
