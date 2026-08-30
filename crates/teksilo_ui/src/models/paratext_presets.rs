// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Paratext presets — the starting front and back matter a new project can be given.
//!
//! A preset is a TOML file naming a publishing tradition and listing the pages it opens
//! and closes a book with. It is **data, never code**: adding one is a file, and a writer
//! can add their own.
//!
//! ## Two things that are deliberately not translated
//!
//! Item titles are **verbatim**, in the language of the tradition they come from. *Achevé
//! d'imprimer* has no English translation that means anything, and a writer who wants it
//! renamed knows better than we do what to call it. They are content bound for the book,
//! and like every other created title they are resolved once and then owned by the writer
//! — switching the interface language later must never retitle them.
//!
//! The preset's own `name` is the same: "Roman français" names itself.
//!
//! What *is* translated is the pair of folders the items land in ("Front matter" / "Back
//! matter"), because those are organisational scaffolding common to every preset — and
//! because a `Folder/Paratext` emits nothing into the export, so its name never reaches
//! the book. Neither, in fact, does a leaf's *title*: `render.rs` gives a paratext no
//! heading of its own — "its title is a binder label, not a line of the book" — and prints
//! only its `ParatextText` prose. So the reason to keep the titles verbatim is not that
//! they are printed; it is that *Achevé d'imprimer* has no English that means anything, and
//! a writer who wants it renamed knows better than we do what to call it.
//!
//! ## `[front]` and `[back]` are not a model concept
//!
//! They decide **where these items are created, and nothing more**. There is no front
//! matter or back matter in the model, in the binder or in the compiler: once the project
//! exists these are ordinary rows, and the writer moves them wherever their tradition,
//! their publisher or their taste puts them. This is the whole reason the model has no
//! zones — the French *table des matières* goes at the back and the American one at the
//! front, so any ordering baked into the compiler would be wrong for half the writers.
//!
//! ## Storage
//!
//! Bundled presets are compiled in (`include_str!`), so they cannot go missing in a
//! Flatpak or an AppImage. User presets live in one settings file as raw TOML strings —
//! the same envelope `export_styles.toml` uses for the same reason: one file to lock and
//! migrate, one editable blob per entry.
//!
//! A malformed user preset is **skipped, not fatal**. A New Work dialog that refused to
//! open because someone fat-fingered a bracket in an optional file would be a bad trade;
//! `skrib_format`'s hard-failing `read_ron_vec` is the anti-pattern this avoids.

use std::path::PathBuf;
use std::rc::Rc;

use serde::{Deserialize, Serialize};
use teksilo::settings::{
    AppPaths, Migrator, Reloadable, SettingsFile, SettingsFileError, Versioned,
};

/// The four traditions shipped with the app. Data, not code — each is one file.
const BUNDLED: &[(&str, &str)] = &[
    (
        "us-trade",
        include_str!("../../../../resources/paratext/us-trade.toml"),
    ),
    (
        "roman-francais",
        include_str!("../../../../resources/paratext/roman-francais.toml"),
    ),
    (
        "uk-trade",
        include_str!("../../../../resources/paratext/uk-trade.toml"),
    ),
    (
        "deutscher-roman",
        include_str!("../../../../resources/paratext/deutscher-roman.toml"),
    ),
];

/// One preset, parsed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParatextPreset {
    /// Stable identity: the bundled file stem, or a minted id for a user preset.
    pub id: String,
    /// Shown as-is in the picker. Names itself, in its own language.
    pub name: String,
    /// ISO country codes this tradition serves, for preselecting from the interface
    /// locale. Several, because "UK trade" serves more than one country.
    pub country: Vec<String>,
    /// What the item titles are written in. Offered as the new work's language, never
    /// forced — an American may want the French structure in English.
    pub language: String,
    pub front: Vec<String>,
    pub back: Vec<String>,
    /// `false` for a bundled preset, which cannot be edited or deleted in place — the
    /// Settings pane offers *duplicate and edit* instead.
    pub editable: bool,
}

impl ParatextPreset {
    /// Parse one preset's TOML source. The error is the raw parse message, which the
    /// Settings pane shows inline so the writer can see what they broke.
    pub fn parse(id: &str, source: &str, editable: bool) -> Result<Self, String> {
        let raw: RawPreset = toml::from_str(source).map_err(|e| e.to_string())?;
        if raw.paratext.name.trim().is_empty() {
            return Err("a preset needs a `name`".to_string());
        }
        Ok(ParatextPreset {
            id: id.to_string(),
            name: raw.paratext.name,
            country: raw.paratext.country,
            language: raw.paratext.language,
            front: clean(raw.front.items),
            back: clean(raw.back.items),
            editable,
        })
    }

    /// Whether this preset serves `locale` — matched on the country subtag, so `fr-FR`
    /// finds the French tradition and `fr-CA` does not accidentally.
    pub fn serves_locale(&self, locale: &str) -> bool {
        let country = locale
            .split(['-', '_'])
            .nth(1)
            .unwrap_or_default()
            .to_ascii_uppercase();
        !country.is_empty()
            && self
                .country
                .iter()
                .any(|c| c.eq_ignore_ascii_case(&country))
    }
}

/// Blank entries are dropped rather than becoming untitled rows: a trailing comma in a
/// hand-edited list should not create a nameless page in someone's binder.
fn clean(items: Vec<String>) -> Vec<String> {
    items
        .into_iter()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect()
}

#[derive(Deserialize)]
struct RawPreset {
    paratext: RawMeta,
    #[serde(default)]
    front: RawItems,
    #[serde(default)]
    back: RawItems,
}

#[derive(Deserialize)]
struct RawMeta {
    name: String,
    #[serde(default)]
    country: Vec<String>,
    #[serde(default)]
    language: String,
}

#[derive(Deserialize, Default)]
struct RawItems {
    #[serde(default)]
    items: Vec<String>,
}

/// The persisted user presets: a version stamp + each preset's raw TOML source.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ParatextPresetsFile {
    #[serde(default = "default_version")]
    pub version: u32,
    #[serde(default)]
    pub presets: Vec<String>,
}

fn default_version() -> u32 {
    ParatextPresetsFile::CURRENT_VERSION
}

impl Default for ParatextPresetsFile {
    fn default() -> Self {
        ParatextPresetsFile {
            version: ParatextPresetsFile::CURRENT_VERSION,
            presets: Vec::new(),
        }
    }
}

impl Versioned for ParatextPresetsFile {
    const CURRENT_VERSION: u32 = 1;
    fn version(&self) -> u32 {
        self.version
    }
    fn set_version(&mut self, v: u32) {
        self.version = v;
    }
}

fn migrator() -> Migrator<ParatextPresetsFile> {
    Migrator::new()
}

/// Reads the bundled presets and the user's, and writes the user's.
#[derive(Clone)]
pub struct ParatextPresetsService {
    file: SettingsFile<ParatextPresetsFile>,
}

impl ParatextPresetsService {
    pub fn open(paths: &AppPaths) -> Result<Self, SettingsFileError> {
        let file = SettingsFile::load(paths.config_file("paratext_presets"), migrator())?;
        Ok(Self { file })
    }

    #[cfg_attr(not(test), allow(dead_code))]
    pub fn open_at(path: PathBuf) -> Result<Self, SettingsFileError> {
        let file = SettingsFile::load(path, migrator())?;
        Ok(Self { file })
    }

    /// Graceful fallback when the config dir is unavailable, mirroring the export styles:
    /// the app still runs, user presets just do not persist.
    pub fn in_memory_default() -> Self {
        let path = std::env::temp_dir().join(format!(
            "skribisto-paratext-presets-{}.toml",
            std::process::id()
        ));
        SettingsFile::load(path, migrator())
            .map(|file| Self { file })
            .unwrap_or_else(|_| {
                let file = SettingsFile::load(
                    PathBuf::from(".skribisto-paratext-presets.toml"),
                    migrator(),
                )
                .expect("in-memory paratext-presets settings fallback");
                Self { file }
            })
    }

    pub fn as_reloadable(&self) -> Rc<dyn Reloadable> {
        Rc::new(self.file.clone())
    }

    /// The bundled presets. Always parse — a failure here is a build-time mistake, and the
    /// test below is what catches it, so a broken one is simply skipped at runtime rather
    /// than taking the dialog down with it.
    pub fn bundled() -> Vec<ParatextPreset> {
        BUNDLED
            .iter()
            .filter_map(|(id, src)| ParatextPreset::parse(id, src, false).ok())
            .collect()
    }

    /// A bundled preset's TOML source, for *duplicate and edit*. Bundled presets are
    /// read-only, and this is how a writer bases their own on one instead of retyping it.
    pub fn bundled_source(id: &str) -> Option<&'static str> {
        BUNDLED.iter().find(|(k, _)| *k == id).map(|(_, src)| *src)
    }

    /// The user's presets, skipping any that no longer parse.
    pub fn user_presets(&self) -> Vec<ParatextPreset> {
        self.raw_user_presets()
            .into_iter()
            .enumerate()
            .filter_map(|(i, src)| ParatextPreset::parse(&user_id(i), &src, true).ok())
            .collect()
    }

    /// Every preset, bundled first. What the picker and the Settings pane both list.
    pub fn all(&self) -> Vec<ParatextPreset> {
        let mut v = Self::bundled();
        v.extend(self.user_presets());
        v
    }

    /// The raw sources, for the Settings pane's editor.
    pub fn raw_user_presets(&self) -> Vec<String> {
        self.file.borrow().presets.clone()
    }

    /// The preset the interface locale suggests, if any — the dialog's initial pick. A
    /// user preset wins over a bundled one serving the same country, since someone who
    /// wrote their own meant it.
    pub fn preselect_for_locale(&self, locale: &str) -> Option<ParatextPreset> {
        let all = self.all();
        all.iter().rev().find(|p| p.serves_locale(locale)).cloned()
    }

    pub fn add_user_preset(&self, source: &str) -> Result<(), SettingsFileError> {
        let mut presets = self.raw_user_presets();
        presets.push(source.to_string());
        self.file.mutate(|f| f.presets = presets)
    }

    pub fn replace_user_preset(&self, index: usize, source: &str) -> Result<(), SettingsFileError> {
        let mut presets = self.raw_user_presets();
        if let Some(slot) = presets.get_mut(index) {
            *slot = source.to_string();
        }
        self.file.mutate(|f| f.presets = presets)
    }

    pub fn remove_user_preset(&self, index: usize) -> Result<(), SettingsFileError> {
        let mut presets = self.raw_user_presets();
        if index < presets.len() {
            presets.remove(index);
        }
        self.file.mutate(|f| f.presets = presets)
    }
}

/// User preset ids are positional. They are not persisted anywhere and exist only to key
/// a list row within one session.
fn user_id(index: usize) -> String {
    format!("user-{index}")
}

/// A starting point for a writer creating their own, offered by the Settings pane's
/// "New preset" button. Deliberately in English and deliberately minimal: it is a
/// scaffold to overwrite, not a tradition to follow.
pub const NEW_PRESET_TEMPLATE: &str = r#"[paratext]
name = "My structure"
country = []
language = ""

[front]
items = [
    "Title page",
]

[back]
items = [
]
"#;

#[cfg(test)]
mod tests {
    use super::*;

    /// Every shipped preset parses and names itself. A broken one is skipped at runtime,
    /// so without this test it would ship silently missing from the picker.
    #[test]
    fn every_bundled_preset_parses() {
        let all = ParatextPresetsService::bundled();
        assert_eq!(
            all.len(),
            BUNDLED.len(),
            "one or more bundled presets failed to parse"
        );
        for p in &all {
            assert!(!p.name.trim().is_empty(), "{} has no name", p.id);
            assert!(
                !p.front.is_empty() || !p.back.is_empty(),
                "{} lists nothing at all",
                p.id
            );
            assert!(!p.editable, "a bundled preset is never editable in place");
        }
    }

    /// The two traditions that motivated the whole design differ in the way that proves
    /// no canonical order exists: the contents page changes ends, and "also by" moves
    /// from the front of a French book to the back of an American one.
    #[test]
    fn the_french_and_american_traditions_genuinely_differ() {
        let all = ParatextPresetsService::bundled();
        let fr = all.iter().find(|p| p.id == "roman-francais").unwrap();
        let us = all.iter().find(|p| p.id == "us-trade").unwrap();

        assert!(fr.back.iter().any(|t| t.starts_with("Table des")));
        assert!(us.front.iter().any(|t| t == "Contents"));
        assert!(fr.front.iter().any(|t| t == "Du même auteur"));
        assert!(us.back.iter().any(|t| t.starts_with("Also by")));
    }

    #[test]
    fn a_preset_is_matched_to_a_locale_by_country() {
        let all = ParatextPresetsService::bundled();
        let fr = all.iter().find(|p| p.id == "roman-francais").unwrap();
        assert!(fr.serves_locale("fr-FR"));
        assert!(fr.serves_locale("fr_BE"));
        assert!(!fr.serves_locale("fr-CA"), "Canada is not on its list");
        assert!(!fr.serves_locale("fr"), "a bare language names no country");
    }

    /// A malformed user preset is skipped, and the good ones around it still load. The
    /// alternative — one bad bracket taking the New Work dialog down — is the failure this
    /// whole design avoids.
    #[test]
    fn a_malformed_user_preset_is_skipped_not_fatal() {
        let dir = std::env::temp_dir().join(format!("skrib-paratext-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        let svc = ParatextPresetsService::open_at(dir.join("p.toml")).unwrap();

        svc.add_user_preset("this is not toml [[[").unwrap();
        svc.add_user_preset(NEW_PRESET_TEMPLATE).unwrap();

        let user = svc.user_presets();
        assert_eq!(user.len(), 1, "the good one survives the bad one");
        assert_eq!(user[0].name, "My structure");
        assert_eq!(
            svc.all().len(),
            ParatextPresetsService::bundled().len() + 1,
            "and the bundled ones are unaffected"
        );
        let _ = std::fs::remove_dir_all(&dir);
    }

    /// A preset with no `name` is refused rather than listed as a blank row.
    #[test]
    fn a_nameless_preset_is_refused() {
        let err = ParatextPreset::parse("x", "[paratext]\nname = \"\"\n", true).unwrap_err();
        assert!(err.contains("name"), "got {err}");
    }

    /// Blank entries never become untitled pages.
    #[test]
    fn blank_items_are_dropped() {
        let p = ParatextPreset::parse(
            "x",
            "[paratext]\nname = \"X\"\n[front]\nitems = [\"A\", \"  \", \"B\"]\n",
            true,
        )
        .unwrap();
        assert_eq!(p.front, vec!["A", "B"]);
    }
}
