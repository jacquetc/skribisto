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

#[cfg(test)]
mod tests {
    use super::*;

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
