//! The compiled-in catalogue of downloadable Hunspell dictionaries.
//!
//! This is static, curated data — one entry per installable dictionary *variant* (French
//! alone is three: Classique, Réforme 1990, Toutes variantes) — so, like
//! [`crate::models::examples_list_model`], it has **no real/mock split**: the bytes are
//! identical in both builds. It lives here in the UI layer, not `skribisto_model`, because it
//! is app-local catalogue data (URLs, licence assets, on-disk basenames), not a domain rule.
//!
//! ## Why compiled-in, not fetched
//!
//! The registry ships in the binary (`include_str!`) and every licence text is bundled
//! alongside it. So the "Get more" list and every licence modal work **offline, on first
//! run, with no network** — the network is touched only when the user clicks Download, and
//! only for that one dictionary's files. That also sidesteps GitHub's REST rate limit
//! entirely: there is no "list dictionaries" API call, unlike the old app which scraped
//! `api.github.com` on every wizard open.
//!
//! The cost is that the catalogue changes only between Skribisto releases — the right
//! trade-off for a v1 with no server infrastructure. A dead URL or a new language is an
//! ordinary reviewed PR, which is also where each licence is vetted.
//!
//! ## Legacy `dict_language` resolution
//!
//! A `.skrib` written by the old C++ app (or its SQLite upgrader) can hold a value that is
//! not a registry id: an underscore form (`en_US`), an editorial basename (`de_DE_frami`), or
//! a merged label (`fr-classique+reforme1990`). [`resolve_token`] maps such a token to a
//! registry id via [`skribisto_model::language::canonicalize`] (the syntactic part) plus a
//! `system_basenames` lookup (the part that needs *this* catalogue). Unresolvable tokens are
//! surfaced as-is, never silently rewritten — see [`crate::language_pill_field`].

use std::sync::OnceLock;

use serde::Deserialize;

/// Where a dictionary's `.aff`/`.dic` bytes come from.
#[derive(Debug, Clone, Deserialize)]
pub enum Source {
    /// One URL per file — the LibreOffice/dictionaries shape (the primary source).
    DirectFiles { aff_url: String, dic_url: String },
    /// One member each out of a shared zip — the grammalecte.net shape, where all three
    /// French variants live in one archive (so the zip is fetched once and cached).
    ZipMember {
        zip_url: String,
        aff_member: String,
        dic_member: String,
    },
}

/// One installable dictionary variant.
#[derive(Debug, Clone, Deserialize)]
pub struct DictionaryEntry {
    /// The `dict_language` value this dictionary satisfies: a BCP-47 tag, optionally with a
    /// private-use variant (`fr-FR`, `fr-FR-x-1990`, `de-DE-1996`, `en-US`).
    pub id: String,
    /// Human name, ideally in the language's own tongue (`"Français — classique"`).
    pub display_name: String,
    /// The basename(s) a system-installed copy is known by under `/usr/share/hunspell`
    /// (`"en_US"`, `"de_DE_frami"`, `"fr"`), tried in order to match a discovered file.
    pub system_basenames: Vec<String>,
    /// Where to fetch the files.
    pub source: Source,
    /// The bundled licence text this maps to (a key into [`license_text`], e.g.
    /// `"mpl-2.0"`).
    pub license_asset: String,
    /// Human licence name + SPDX id, shown as a badge.
    pub license_name: String,
    /// Approximate size of the `.dic` in bytes, for the download UI.
    pub approx_size_bytes: u64,
}

const REGISTRY_RON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/assets/dictionaries/registry.ron"
));

/// Every bundled licence text, keyed by `license_asset`. Hard-coded rather than
/// directory-scanned so a missing file is a **compile** error, not a runtime dead link. The
/// set is exactly the licences the verified registry entries reference — no more, no less.
macro_rules! license_assets {
    ($($key:literal),+ $(,)?) => {
        &[$(
            ($key, include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/dictionaries/licenses/", $key, ".txt"
            ))),
        )+]
    };
}
const LICENSES: &[(&str, &str)] = license_assets![
    "gpl-2.0",         // English ×4, pt-PT
    "gpl-3.0",         // German ×3, it-IT
    "mpl-2.0",         // French ×3 (grammalecte)
    "mpl-1.1",         // es-ES (triple-licensed; MPL-1.1 chosen)
    "lgpl-3.0",        // pt-BR, sv-SE
    "bsd-revised",     // nl-NL (BSD-3-Clause or CC-BY-3.0)
    "cc-by-4.0",       // pl-PL (multi-licensed; CC-BY-4.0 chosen)
    "ru-lebedev",      // ru-RU (a non-standard 4-clause BSD — bundled verbatim)
    "ca-unspecified",  // ca (source states "GPL, LGPL" with no version)
];

/// The parsed registry, built once on first access.
pub fn entries() -> &'static [DictionaryEntry] {
    static PARSED: OnceLock<Vec<DictionaryEntry>> = OnceLock::new();
    PARSED
        .get_or_init(|| {
            ron::from_str(REGISTRY_RON)
                .expect("assets/dictionaries/registry.ron is malformed (compiled-in data)")
        })
        .as_slice()
}

/// The entry with this exact registry `id`, if any.
pub fn by_id(id: &str) -> Option<&'static DictionaryEntry> {
    entries().iter().find(|e| e.id == id)
}

/// The registry id whose `system_basenames` includes `basename` (a discovered on-disk file's
/// stem) — how a system-found dictionary is matched to a catalogue entry.
pub fn id_for_basename(basename: &str) -> Option<&'static str> {
    entries()
        .iter()
        .find(|e| e.system_basenames.iter().any(|b| b == basename))
        .map(|e| e.id.as_str())
}

/// Resolve one raw `dict_language` token to a registry id, tolerating legacy shapes.
///
/// Order: an exact id wins; otherwise the token is syntactically canonicalised
/// (`en_US` → `en-US`) and tried again; otherwise it is treated as a system basename
/// (`de_DE_frami`, `fr`). `None` means "unrecognised" — the caller surfaces it as-is and
/// never rewrites the stored value.
pub fn resolve_token(raw: &str) -> Option<&'static str> {
    if let Some(e) = by_id(raw) {
        return Some(e.id.as_str());
    }
    let canon = skribisto_model::language::canonicalize(raw);
    if let Some(e) = by_id(&canon) {
        return Some(e.id.as_str());
    }
    id_for_basename(raw).or_else(|| id_for_basename(&canon))
}

/// The bundled text of a licence, by its `license_asset` key. `None` for an unknown key
/// (which the registry test below forbids for any entry actually in use).
pub fn license_text(asset: &str) -> Option<&'static str> {
    LICENSES
        .iter()
        .find(|(k, _)| *k == asset)
        .map(|(_, text)| *text)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The compiled-in RON parses, and there is at least one dictionary.
    #[test]
    fn registry_parses_and_is_nonempty() {
        assert!(!entries().is_empty(), "registry.ron produced no entries");
    }

    /// No two entries share an id (a duplicate would make `by_id` ambiguous).
    #[test]
    fn ids_are_unique() {
        let mut ids: Vec<&str> = entries().iter().map(|e| e.id.as_str()).collect();
        ids.sort_unstable();
        let before = ids.len();
        ids.dedup();
        assert_eq!(before, ids.len(), "duplicate dictionary id in registry.ron");
    }

    /// Every entry's `license_asset` resolves to a bundled licence text — the guarantee that
    /// the licence modal can never hit a missing file for a listed dictionary.
    #[test]
    fn every_license_asset_is_bundled() {
        for e in entries() {
            assert!(
                license_text(&e.license_asset).is_some(),
                "dictionary {:?} references unbundled licence {:?}",
                e.id,
                e.license_asset
            );
        }
    }

    /// The French set and the base English are present. Bare `fr-FR` is the tolerant
    /// toutes-variantes default; classique and réforme-1990 are the explicit variants.
    #[test]
    fn the_motivating_entries_exist() {
        for id in ["en-US", "fr-FR", "fr-FR-x-classique", "fr-FR-x-1990"] {
            assert!(by_id(id).is_some(), "missing expected dictionary {id}");
        }
    }

    /// Legacy underscore and basename forms resolve to a registry id.
    #[test]
    fn resolve_token_tolerates_legacy_shapes() {
        assert_eq!(resolve_token("en-US"), Some("en-US"));
        assert_eq!(resolve_token("en_US"), Some("en-US"), "underscore form");
        // A bare system basename maps through `system_basenames`.
        assert_eq!(resolve_token("fr"), Some("fr-FR"), "system basename");
        assert_eq!(resolve_token("kl-KL"), None, "unknown stays unresolved");
    }
}
