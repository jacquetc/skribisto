// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Reading the small JSON file that says which release is current.
//!
//! Blocking, so it runs on a `spawn_blocking` worker exactly like the dictionary
//! download does. Nothing here touches a `Signal`, a widget or the store, which
//! is what lets it be `Send` and run off the UI thread.
//!
//! ## Why not the GitHub API
//!
//! The obvious source is `api.github.com/repos/.../releases/latest`, and it is
//! the wrong one on four counts, three of them measured against this repository:
//!
//! - **It would have served a release candidate as the current release.** Every
//!   release of this repository is flagged `"prerelease": false`, `v3.0.0-rc1`
//!   and `v3.0.0-rc2` included, so for the two days those tags were newest the
//!   endpoint named them as the latest stable version. The website generator
//!   already knows this and reads the tag instead.
//! - **Sixty requests an hour, per IP.** That is shared by everyone behind one
//!   corporate address, and a conditional request does not help: a `304` still
//!   costs one of the sixty.
//! - **Twelve kilobytes** of release, author and asset objects, against roughly
//!   three hundred bytes here.
//! - **It sends the reader's address to a third party.** The project's own site
//!   is a party the reader already has a relationship with; the privacy page can
//!   describe it truthfully, and it says so.
//!
//! GitHub stays where the release *lives*. It is simply not what the application
//! asks.

use std::collections::BTreeMap;
use std::time::Duration;

use serde::Deserialize;

/// Highest feed schema this build understands.
///
/// A feed declaring a higher number is refused rather than parsed leniently: the
/// number is only ever raised for a change an older client could not survive, so
/// guessing is exactly what it exists to prevent.
const SUPPORTED_SCHEMA: u32 = 1;

/// Refuse a body larger than this. The real feed is a few hundred bytes; a
/// megabyte of it is a captive portal's login page or a misconfigured host, and
/// neither should be buffered, let alone parsed.
const MAX_BODY: u64 = 64 * 1024;

/// The whole request, start to finish. Short on purpose: nothing waits on this,
/// no reader is watching it, and a check that fails silently today is retried
/// tomorrow at no cost.
const TIMEOUT: Duration = Duration::from_secs(10);

/// What the feed says. Every field the application does not know is ignored, so
/// the server can add one without stranding an installed build.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct Feed {
    /// Schema number; see [`SUPPORTED_SCHEMA`].
    pub feed: u32,
    /// The current release, as a bare version (`3.0.1`).
    pub version: String,
    /// The tag it was cut from (`v3.0.1`). Display and diagnostics only.
    #[serde(default)]
    pub tag: String,
    /// Publication date, `YYYY-MM-DD`.
    #[serde(default)]
    pub date: String,
    /// Release-notes page, by language code.
    #[serde(default)]
    pub notes: BTreeMap<String, String>,
    /// Download page, by language code.
    #[serde(default)]
    pub download: BTreeMap<String, String>,
}

impl Feed {
    /// The download page in `language`, falling back to English and then to
    /// whatever the map holds first.
    ///
    /// A reader sent to a page in a language they do not read has not been
    /// helped, but sending them nowhere is worse, so this never returns `None`
    /// for a feed that carries any link at all.
    pub fn download_for(&self, language: &str) -> Option<&str> {
        pick(&self.download, language)
    }

    /// The release-notes page in `language`. Same fallback as
    /// [`Self::download_for`].
    pub fn notes_for(&self, language: &str) -> Option<&str> {
        pick(&self.notes, language)
    }
}

/// Look up a language, then its base subtag, then English, then anything.
///
/// The interface locale is a full tag (`fr-FR`), while the site publishes by
/// base language (`fr`), so the base subtag is the case that actually matches.
fn pick<'a>(map: &'a BTreeMap<String, String>, language: &str) -> Option<&'a str> {
    let base = language.split(['-', '_']).next().unwrap_or(language);
    map.get(language)
        .or_else(|| map.get(base))
        .or_else(|| map.get("en"))
        .or_else(|| map.values().next())
        .map(String::as_str)
}

/// Parse a feed body, refusing anything this build cannot trust.
///
/// Separated from [`fetch`] so every rejection below is a unit test rather than
/// something only a broken server could demonstrate.
pub fn parse(body: &str) -> Result<Feed, String> {
    let feed: Feed = serde_json::from_str(body).map_err(|e| format!("unreadable feed: {e}"))?;
    if feed.feed > SUPPORTED_SCHEMA {
        return Err(format!(
            "feed schema {} is newer than this build understands ({SUPPORTED_SCHEMA})",
            feed.feed
        ));
    }
    if feed.version.trim().is_empty() {
        return Err("the feed names no version".to_string());
    }
    // Every link is handed to the browser eventually, and `open_external_link`
    // would refuse a foreign scheme at the door. Refusing here as well means a
    // tampered feed cannot put a live-looking button in front of the reader that
    // only fails when clicked.
    for url in feed.notes.values().chain(feed.download.values()) {
        if !url.starts_with("https://") {
            return Err(format!("the feed offers a link that is not https: {url}"));
        }
    }
    Ok(feed)
}

/// Fetch and parse the feed at `url`. Blocking; call from a worker.
///
/// Errors are strings because nothing acts on their variant: every failure has
/// the same consequence, which is that the application says nothing and tries
/// again tomorrow.
pub fn fetch(url: &str) -> Result<Feed, String> {
    let body = ureq::get(url)
        .config()
        // One global cap rather than the connect/receive pair the dictionary
        // download uses: the body here is a few hundred bytes, so there is no
        // legitimate slow transfer to protect.
        .timeout_global(Some(TIMEOUT))
        .https_only(true)
        .build()
        .header("User-Agent", USER_AGENT)
        .call()
        .map_err(|e| format!("request failed: {e}"))?
        .body_mut()
        .with_config()
        .limit(MAX_BODY)
        .read_to_string()
        .map_err(|e| format!("read failed: {e}"))?;
    parse(&body)
}

/// Names the client and nothing else.
///
/// Deliberately **without** a version, unlike the dictionary downloader's. The
/// comparison happens on the reader's machine, so the server never needs to know
/// which version is asking, and a version here would turn an access log into a
/// census of installed versions. The privacy page says the request carries no
/// such thing; this is the line that has to stay true for it.
const USER_AGENT: &str = "Skribisto (update check)";

#[cfg(test)]
mod tests {
    use super::*;

    /// The **exact bytes** `scripts/sync_release_data.py` in the website
    /// repository produces, pasted verbatim rather than hand-written.
    ///
    /// The two repositories cannot import from each other, so this literal is
    /// the only thing standing between a renamed field on the generator side and
    /// an application that silently stops seeing releases. Regenerate it with
    /// `python3 scripts/sync_release_data.py` and paste the file in.
    const GOOD: &str = r#"{
        "date": "2026-09-03",
        "download": {
          "en": "https://www.skribisto.eu/download/",
          "fr": "https://www.skribisto.eu/fr/download/"
        },
        "feed": 1,
        "notes": {
          "en": "https://www.skribisto.eu/news/",
          "fr": "https://www.skribisto.eu/fr/news/"
        },
        "tag": "v3.0.1",
        "version": "3.0.1"
      }"#;

    #[test]
    fn the_published_feed_shape_parses() {
        let feed = parse(GOOD).expect("the real feed shape must parse");
        assert_eq!(feed.version, "3.0.1");
        assert_eq!(feed.tag, "v3.0.1");
        assert_eq!(feed.date, "2026-09-03");
    }

    #[test]
    fn a_full_locale_tag_finds_its_base_language() {
        let feed = parse(GOOD).unwrap();
        // The interface locale is `fr-FR`; the site publishes `fr`.
        assert_eq!(
            feed.download_for("fr-FR"),
            Some("https://www.skribisto.eu/fr/download/")
        );
        assert_eq!(
            feed.notes_for("fr-FR"),
            Some("https://www.skribisto.eu/fr/news/")
        );
        assert_eq!(
            feed.download_for("en-US"),
            Some("https://www.skribisto.eu/download/")
        );
    }

    #[test]
    fn an_unpublished_language_falls_back_to_english() {
        let feed = parse(GOOD).unwrap();
        assert_eq!(
            feed.download_for("eo"),
            Some("https://www.skribisto.eu/download/")
        );
    }

    #[test]
    fn a_feed_with_no_links_asks_for_nothing_impossible() {
        let feed = parse(r#"{"feed":1,"version":"3.0.1"}"#).unwrap();
        assert_eq!(feed.download_for("en-US"), None);
        assert_eq!(feed.notes_for("en-US"), None);
    }

    #[test]
    fn a_newer_schema_is_refused_rather_than_guessed_at() {
        let err = parse(r#"{"feed":2,"version":"9.9.9"}"#).unwrap_err();
        assert!(err.contains("newer than this build"), "{err}");
    }

    #[test]
    fn unknown_fields_are_ignored_so_the_server_can_grow() {
        let feed = parse(r#"{"feed":1,"version":"3.0.1","urgency":"security","x":[1,2]}"#)
            .expect("an added field must not strand an installed build");
        assert_eq!(feed.version, "3.0.1");
    }

    #[test]
    fn a_feed_naming_no_version_is_refused() {
        assert!(parse(r#"{"feed":1,"version":""}"#).is_err());
        assert!(parse(r#"{"feed":1,"version":"   "}"#).is_err());
    }

    #[test]
    fn a_link_in_another_scheme_is_refused_at_parse_time() {
        // A button that looks live and dies on click teaches the reader nothing.
        let err = parse(r#"{"feed":1,"version":"3.0.1","download":{"en":"file:///etc/passwd"}}"#)
            .unwrap_err();
        assert!(err.contains("not https"), "{err}");
        assert!(
            parse(r#"{"feed":1,"version":"3.0.1","notes":{"en":"http://example.com"}}"#).is_err(),
            "plain http is refused too"
        );
    }

    #[test]
    fn a_captive_portal_login_page_is_not_a_feed() {
        assert!(parse("<!DOCTYPE html><html><body>Sign in</body></html>").is_err());
        assert!(parse("").is_err());
    }

    #[test]
    fn the_user_agent_carries_no_version() {
        // The privacy page states the request does not disclose which version is
        // asking. This is the only place that could break it.
        assert!(!USER_AGENT.contains(env!("CARGO_PKG_VERSION")));
        assert!(!USER_AGENT.contains(char::is_numeric));
    }
}
