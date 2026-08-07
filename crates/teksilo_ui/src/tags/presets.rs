// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Named tag presets, **generated in code rather than shipped as data**.
//!
//! That is the whole point: a preset built here goes through `tr!`, so a French writer
//! applying "Basic" gets `personnage` / `lieu`, not English strings imported verbatim.
//! A data file would have to pick one language at authoring time.
//!
//! A new project starts with an *empty* palette — nothing is seeded — and the writer picks
//! a preset if they want one. Genre presets **extend** Basic rather than standing alone,
//! so applying Sci-fi to an empty palette yields Basic's tags too, and applying it after
//! Basic adds only what is new (`import_tags` skips names already present, so re-applying
//! is a no-op rather than a doubling).

use teksilo::i18n::LocalizedString;
use teksilo::prelude::*;

use crate::models::TagRow;

/// Colours drawn from `ColorPicker::DEFAULT_SWATCHES`' hue range, **minus its near-black
/// and near-white**: a tag's colour is theme-constant, so either extreme disappears
/// against one of the two surfaces. Text colour is never stored — it is derived from these
/// at paint time.
mod hue {
    pub const SLATE: &str = "#607d8b";
    pub const GREY: &str = "#95a5a6";
    pub const GREEN: &str = "#27ae60";
    pub const AMBER: &str = "#f39c12";
    pub const ORANGE: &str = "#d35400";
    pub const BLUE: &str = "#2980b9";
    pub const PURPLE: &str = "#8e44ad";
    pub const TEAL: &str = "#16a085";
    pub const RED: &str = "#c0392b";
    pub const PINK: &str = "#c2185b";
    pub const BROWN: &str = "#795548";
}

/// Which preset the writer picked.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Preset {
    Basic,
    SciFi,
    Fantasy,
    Mystery,
    Historical,
}

impl Preset {
    /// Every preset, in menu order — Basic first, then genres alphabetically.
    pub const ALL: [Preset; 5] = [
        Preset::Basic,
        Preset::SciFi,
        Preset::Fantasy,
        Preset::Mystery,
        Preset::Historical,
    ];

    /// The menu label.
    pub fn label(self) -> LocalizedString {
        match self {
            Preset::Basic => tr!(tags_preset_basic()),
            Preset::SciFi => tr!(tags_preset_scifi()),
            Preset::Fantasy => tr!(tags_preset_fantasy()),
            Preset::Mystery => tr!(tags_preset_mystery()),
            Preset::Historical => tr!(tags_preset_historical()),
        }
    }

    /// The rows to hand to `import_tags`, resolved in the active locale.
    ///
    /// Ids are zero — `import_tags` assigns real ones. The `status/…` prefix is a naming
    /// convention, not a hierarchy: no parent/child entity, no tree UI, nothing parses it.
    /// Alphabetical ordering alone makes those four cluster in every list, which is the
    /// entire reason the prefix is worth having.
    pub fn rows(self) -> Vec<TagRow> {
        let mut rows = basic_rows();
        match self {
            Preset::Basic => {}
            Preset::SciFi => rows.extend(genre_rows(&[
                (tr!(tags_preset_vessel()), hue::TEAL),
                (tr!(tags_preset_planet()), hue::BLUE),
                (tr!(tags_preset_organization()), hue::PURPLE),
            ])),
            Preset::Fantasy => rows.extend(genre_rows(&[
                (tr!(tags_preset_creature()), hue::TEAL),
                (tr!(tags_preset_faction()), hue::PURPLE),
                (tr!(tags_preset_artifact()), hue::AMBER),
                (tr!(tags_preset_realm()), hue::BLUE),
                (tr!(tags_preset_magic_system()), hue::PINK),
            ])),
            Preset::Mystery => {
                rows.extend(genre_rows(&[
                    (tr!(tags_preset_suspect()), hue::RED),
                    (tr!(tags_preset_victim()), hue::BROWN),
                    (tr!(tags_preset_clue()), hue::TEAL),
                ]));
                // A red herring is a plot device, not a named entity anyone writes into
                // prose — indexing it would only generate noise, exactly like `plot point`.
                rows.push(row(tr!(tags_preset_red_herring()), hue::ORANGE, false));
            }
            Preset::Historical => {
                rows.extend(genre_rows(&[(
                    tr!(tags_preset_historical_figure()),
                    hue::BROWN,
                )]));
                rows.push(row(tr!(tags_preset_source()), hue::GREY, false));
                rows.push(row(tr!(tags_preset_period_detail()), hue::AMBER, false));
            }
        }
        rows
    }
}

fn row(name: LocalizedString, color: &str, discoverable: bool) -> TagRow {
    TagRow {
        id: 0,
        name: name.resolve_now(),
        color: color.to_string(),
        details: String::new(),
        discoverable,
    }
}

/// Genre additions are all discoverable taxonomy — the things a story bible is *about*.
fn genre_rows(specs: &[(LocalizedString, &str)]) -> Vec<TagRow> {
    specs
        .iter()
        .map(|(name, color)| row(name.clone(), color, true))
        .collect()
}

fn basic_rows() -> Vec<TagRow> {
    vec![
        // The workflow ladder: one hue at rising saturation, so progress along it reads at
        // a glance without anyone having to read the label.
        row(tr!(tags_preset_status_outline()), hue::GREY, false),
        row(tr!(tags_preset_status_draft()), hue::SLATE, false),
        row(tr!(tags_preset_status_to_review()), hue::AMBER, false),
        row(tr!(tags_preset_status_finished()), hue::GREEN, false),
        // Flags, deliberately *without* the `status/` prefix: an item can be a draft AND
        // need research at once, so prefixing these would falsely imply they belong to the
        // same mutually-exclusive ladder.
        row(tr!(tags_preset_needs_research()), hue::ORANGE, false),
        row(tr!(tags_preset_continuity_check()), hue::RED, false),
        // A structural marker, not a named entity — not discoverable, for the same reason
        // as `red herring`.
        row(tr!(tags_preset_plot_point()), hue::PINK, false),
        // The taxonomy the mention index actually scans prose for.
        row(tr!(tags_preset_character()), hue::BLUE, true),
        row(tr!(tags_preset_place()), hue::PURPLE, true),
        row(tr!(tags_preset_item()), hue::BROWN, true),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::name_key;

    #[test]
    fn every_genre_preset_extends_basic() {
        let basic: Vec<String> = Preset::Basic
            .rows()
            .iter()
            .map(|r| r.name.clone())
            .collect();
        for p in Preset::ALL {
            let names: Vec<String> = p.rows().iter().map(|r| r.name.clone()).collect();
            for b in &basic {
                assert!(
                    names.contains(b),
                    "{p:?} must include Basic's {b:?} — genre presets extend, they do not replace"
                );
            }
        }
    }

    #[test]
    fn no_preset_repeats_a_name() {
        for p in Preset::ALL {
            let rows = p.rows();
            let mut keys: Vec<String> = rows.iter().map(|r| name_key(&r.name)).collect();
            keys.sort();
            let before = keys.len();
            keys.dedup();
            assert_eq!(before, keys.len(), "{p:?} repeats a tag name");
        }
    }

    #[test]
    fn every_preset_row_has_a_name_and_a_colour() {
        for p in Preset::ALL {
            for r in p.rows() {
                assert!(!r.name.trim().is_empty(), "{p:?} has a blank tag name");
                assert!(
                    r.color.starts_with('#') && r.color.len() == 7,
                    "{p:?}/{}: colour {:?} is not #rrggbb",
                    r.name,
                    r.color
                );
            }
        }
    }

    /// The taxonomy is discoverable; workflow states and plot devices are not. Getting
    /// this backwards would either bury the roster in noise or leave it empty.
    #[test]
    fn only_the_taxonomy_is_discoverable() {
        let rows = Preset::Basic.rows();
        let discoverable: Vec<&str> = rows
            .iter()
            .filter(|r| r.discoverable)
            .map(|r| r.name.as_str())
            .collect();
        assert_eq!(
            discoverable.len(),
            3,
            "Basic's discoverable set should be character/place/item, got {discoverable:?}"
        );
        for r in rows.iter().filter(|r| r.name.starts_with("status/")) {
            assert!(!r.discoverable, "{:?} is a workflow state", r.name);
        }
    }

    /// Alphabetical ordering is what makes the prefix worth having.
    #[test]
    fn the_status_prefix_clusters_when_sorted() {
        let mut names: Vec<String> = Preset::Basic
            .rows()
            .iter()
            .map(|r| r.name.clone())
            .collect();
        names.sort_by_key(|n| n.to_lowercase());
        let first = names.iter().position(|n| n.starts_with("status/")).unwrap();
        let count = names.iter().filter(|n| n.starts_with("status/")).count();
        for n in names.iter().skip(first).take(count) {
            assert!(
                n.starts_with("status/"),
                "the status run is interrupted by {n:?}"
            );
        }
    }
}
