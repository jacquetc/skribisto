// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The interface strings Skribisto **stores**: the keys whose text is resolved
//! once, in the interface language, and written into a project as data.
//!
//! A status rung, a tag preset, a note template's headings, the binders a new
//! project or an import starts with: each is `tr!` at the moment it is seeded and a
//! literal from then on (see the i18n notes in the project guide). A fix to one of
//! these translations changes what Skribisto creates *from then on*, never what a
//! project already holds, and a translation report about one has to say so. That
//! is what this table is for: [`is_seeded`] is how the translation-report search
//! tells a seeded string from an ordinary label.
//!
//! **Why a table and not a file.** Classifying by catalogue file is wrong in both
//! directions: `story_bible.ftl` is all interface, `tags.ftl` and `templates.ftl`
//! mix presets with settings rows, and the status rungs, the create-row titles and
//! every importer's names live in `main.ftl`. One key is even both:
//! `create-book-end` labels a menu row *and* is the stored title of an end-of-book
//! marker.
//!
//! **Why it cannot rot.** Each [`SeedingSite`] names a function and lists the keys
//! its body resolves. `tests::every_site_lists_exactly_the_keys_it_resolves`
//! re-reads each function and fails on any key added to or dropped from it, and
//! another test fails if a listed key is missing from the en-US catalogue. A new
//! seeding *function* still needs a row here: say so in its doc comment.

/// One function that resolves interface strings into stored data, and the keys
/// it resolves.
pub(crate) struct SeedingSite {
    /// The source file, relative to `crates/teksilo_ui/src`. Read by the drift test,
    /// which is what keeps [`Self::keys`] true.
    #[cfg_attr(not(test), allow(dead_code))]
    pub file: &'static str,
    /// Text that locates the function: each marker is searched for after the
    /// previous one, and the function body is the brace block that follows the
    /// last. More than one marker where a name repeats in the file.
    #[cfg_attr(not(test), allow(dead_code))]
    pub markers: &'static [&'static str],
    /// The Fluent ids the body resolves, sorted.
    pub keys: &'static [&'static str],
}

/// Every seeding function, by file.
pub(crate) const SEEDING_SITES: &[SeedingSite] = &[
    // The title a created row is stored with.
    SeedingSite {
        file: "binder/create_labels.rs",
        markers: &["pub fn default_title("],
        keys: &[
            "create-book-end",
            "new-item-book",
            "new-item-chapter",
            "new-item-folder",
            "new-item-note",
            "new-item-note-folder",
            "new-item-paratext",
            "new-item-paratext-folder",
            "new-item-part",
            "new-item-story-bible-entry",
            "new-scene-title",
        ],
    },
    // The status-ladder presets, stored as `BinderStatus` names.
    SeedingSite {
        file: "statuses/presets.rs",
        markers: &["pub fn rows(self)"],
        keys: &[
            "status-done",
            "status-draft",
            "status-final",
            "status-first-edit",
            "status-outline",
            "status-plume-draft-1",
            "status-plume-draft-2",
            "status-plume-draft-3",
            "status-plume-edit-1",
            "status-plume-edit-2",
            "status-plume-edit-3",
            "status-plume-finished",
            "status-plume-proofread",
            "status-revised",
            "status-second-edit",
            "status-todo",
        ],
    },
    // The tag presets, stored as `BinderTag` names.
    SeedingSite {
        file: "tags/presets.rs",
        markers: &["pub fn rows(self)"],
        keys: &[
            "tags-preset-artifact",
            "tags-preset-clue",
            "tags-preset-creature",
            "tags-preset-faction",
            "tags-preset-historical-figure",
            "tags-preset-magic-system",
            "tags-preset-organization",
            "tags-preset-period-detail",
            "tags-preset-planet",
            "tags-preset-realm",
            "tags-preset-red-herring",
            "tags-preset-source",
            "tags-preset-suspect",
            "tags-preset-vessel",
            "tags-preset-victim",
        ],
    },
    // The tags every preset starts with.
    SeedingSite {
        file: "tags/presets.rs",
        markers: &["fn basic_rows()"],
        keys: &[
            "tags-preset-character",
            "tags-preset-continuity-check",
            "tags-preset-item",
            "tags-preset-needs-research",
            "tags-preset-place",
            "tags-preset-plot-point",
        ],
    },
    // A note-template preset's name, stored as the template's name.
    SeedingSite {
        file: "note_templates/presets.rs",
        markers: &["impl Preset {", "pub fn label(self)"],
        keys: &[
            "note-template-preset-artifact",
            "note-template-preset-beat-sheet",
            "note-template-preset-character-sheet",
            "note-template-preset-faction",
            "note-template-preset-location",
            "note-template-preset-research-note",
        ],
    },
    // A note-template preset's headings and fields, stored in its body.
    SeedingSite {
        file: "note_templates/presets.rs",
        markers: &["fn body(self)"],
        keys: &[
            "note-template-field-age",
            "note-template-field-age-and-origin",
            "note-template-field-allies-and-enemies",
            "note-template-field-appearance",
            "note-template-field-build-and-features",
            "note-template-field-conflict",
            "note-template-field-ends-as",
            "note-template-field-exit-emotion",
            "note-template-field-fear",
            "note-template-field-flaw",
            "note-template-field-formative-event",
            "note-template-field-full-name",
            "note-template-field-goal",
            "note-template-field-habitual-bearing",
            "note-template-field-how-it-is-used",
            "note-template-field-key-facts",
            "note-template-field-known-as",
            "note-template-field-light-and-weather",
            "note-template-field-need",
            "note-template-field-page-or-link",
            "note-template-field-pov",
            "note-template-field-purpose",
            "note-template-field-relationships",
            "note-template-field-resources",
            "note-template-field-role-in-story",
            "note-template-field-scenes-set-here",
            "note-template-field-sound-and-smell",
            "note-template-field-speech-patterns",
            "note-template-field-starts-as",
            "note-template-field-time-and-place",
            "note-template-field-turn",
            "note-template-field-want",
            "note-template-field-what-happened-here",
            "note-template-field-what-it-does",
            "note-template-field-what-it-wants-now",
            "note-template-field-what-they-never-say",
            "note-template-field-what-you-notice-first",
            "note-template-field-where-from",
            "note-template-field-who-holds-it",
            "note-template-field-who-leads-it",
            "note-template-field-who-lives-or-works-here",
            "note-template-field-who-wants-it",
            "note-template-field-why-it-matters",
            "note-template-section-appearance",
            "note-template-section-arc",
            "note-template-section-first-impression",
            "note-template-section-history",
            "note-template-section-identity",
            "note-template-section-in-the-story",
            "note-template-section-psychology",
            "note-template-section-source",
            "note-template-section-the-scene",
            "note-template-section-the-shape",
            "note-template-section-the-thing-itself",
            "note-template-section-voice",
            "note-template-section-what-it-is",
            "note-template-section-what-it-says",
        ],
    },
    // The binder and folder names a new project is created with.
    SeedingSite {
        file: "new_work/new_work_vm.rs",
        markers: &["pub(crate) fn new_work_dto("],
        keys: &[
            "new-work-back-matter",
            "new-work-characters",
            "new-work-front-matter",
            "new-work-manuscript",
            "new-work-note",
            "new-work-notebook",
            "new-work-notes",
            "new-work-places",
            "new-work-research",
            "new-work-scene",
        ],
    },
    // The binders and groups a Manuskript import creates.
    SeedingSite {
        file: "import_manuskript/import_manuskript_vm.rs",
        markers: &["fn dto(&self"],
        keys: &[
            "import-manuskript-characters-group",
            "import-manuskript-importance-main",
            "import-manuskript-importance-minor",
            "import-manuskript-importance-secondary",
            "import-manuskript-manuscript-binder",
            "import-manuskript-plots-group",
            "import-manuskript-project-info-note",
            "import-manuskript-story-bible-binder",
            "import-manuskript-summary-note",
            "import-manuskript-world-group",
        ],
    },
    // The section headings a Manuskript import writes into notes.
    SeedingSite {
        file: "import_manuskript/import_manuskript_vm.rs",
        markers: &["fn section_name("],
        keys: &[
            "import-manuskript-section-description",
            "import-manuskript-section-email",
            "import-manuskript-section-epiphany",
            "import-manuskript-section-full",
            "import-manuskript-section-full-summary",
            "import-manuskript-section-genre",
            "import-manuskript-section-license",
            "import-manuskript-section-meta",
            "import-manuskript-section-motivation",
            "import-manuskript-section-notes",
            "import-manuskript-section-page",
            "import-manuskript-section-paragraph",
            "import-manuskript-section-paragraph-summary",
            "import-manuskript-section-passion",
            "import-manuskript-section-result",
            "import-manuskript-section-sentence",
            "import-manuskript-section-series",
            "import-manuskript-section-situation",
            "import-manuskript-section-summary",
            "import-manuskript-section-volume",
            "import-manuskript-untitled-project",
            "note-template-field-conflict",
            "note-template-field-goal",
        ],
    },
    // The binders a Plume Creator import creates.
    SeedingSite {
        file: "import_plume/import_plume_vm.rs",
        markers: &["fn dto(&self"],
        keys: &[
            "import-plume-manuscript-binder",
            "import-plume-story-bible-binder",
        ],
    },
    // The names a Plume Creator import seeds.
    SeedingSite {
        file: "import_plume/import_plume_vm.rs",
        markers: &["fn seeded_name("],
        keys: &[
            "import-plume-content-of",
            "import-plume-group-characters",
            "import-plume-group-items",
            "import-plume-group-places",
            "import-plume-untitled-project",
        ],
    },
    // The names and phrases a Scrivener import stores.
    SeedingSite {
        file: "import_scrivener/import_scrivener_vm.rs",
        markers: &["fn resolve() -> Self"],
        keys: &[
            "import-scrivener-label-details",
            "import-scrivener-label-tag",
            "import-scrivener-list-separator",
            "import-scrivener-manuscript-binder",
            "import-scrivener-notes-binder",
            "import-scrivener-notes-of",
            "import-scrivener-project-notes",
            "import-scrivener-recovered",
            "import-scrivener-status-details",
            "import-scrivener-unnamed-label",
            "import-scrivener-unnamed-status",
            "import-scrivener-untitled-tag",
            "import-scrivener-untitled-template",
        ],
    },
    // The metadata words a Scrivener import stores.
    SeedingSite {
        file: "import_scrivener/import_scrivener_vm.rs",
        markers: &["fn metadata_word("],
        keys: &[
            "import-scrivener-abbreviated-title",
            "import-scrivener-collection-details",
            "import-scrivener-collection-tag",
            "import-scrivener-company",
            "import-scrivener-deadline",
            "import-scrivener-deadline-day",
            "import-scrivener-draft-target",
            "import-scrivener-field-details",
            "import-scrivener-field-tag",
            "import-scrivener-field-value",
            "import-scrivener-from-scrivener",
            "import-scrivener-in-own-note",
            "import-scrivener-keyword-details",
            "import-scrivener-keyword-tag",
            "import-scrivener-label-value",
            "import-scrivener-replacement-off",
            "import-scrivener-replacement-regex",
            "import-scrivener-search-all",
            "import-scrivener-search-any",
            "import-scrivener-search-exact",
            "import-scrivener-search-in",
            "import-scrivener-search-in-all",
            "import-scrivener-search-in-custom-metadata",
            "import-scrivener-search-in-keywords",
            "import-scrivener-search-in-label",
            "import-scrivener-search-in-notes",
            "import-scrivener-search-in-section-type",
            "import-scrivener-search-in-status",
            "import-scrivener-search-in-synopsis",
            "import-scrivener-search-in-text",
            "import-scrivener-search-in-title",
            "import-scrivener-search-operator",
            "import-scrivener-search-query",
            "import-scrivener-session-target",
            "import-scrivener-session-target-calculated",
            "import-scrivener-subjects",
            "import-scrivener-unit-characters",
            "import-scrivener-unit-pages",
            "import-scrivener-unit-words",
            "import-scrivener-unnamed-keyword",
            "import-scrivener-weekday-friday",
            "import-scrivener-weekday-monday",
            "import-scrivener-weekday-saturday",
            "import-scrivener-weekday-sunday",
            "import-scrivener-weekday-thursday",
            "import-scrivener-weekday-tuesday",
            "import-scrivener-weekday-wednesday",
            "import-scrivener-writing-days",
        ],
    },
];

/// Whether the interface string `key` is stored into projects when they are
/// created, imported or given a preset.
pub(crate) fn is_seeded(key: &str) -> bool {
    SEEDING_SITES
        .iter()
        .any(|site| site.keys.binary_search(&key).is_ok())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;
    use std::path::Path;

    /// The brace block after `markers`, each searched for after the previous one.
    fn function_body<'a>(source: &'a str, markers: &[&str]) -> Option<&'a str> {
        let mut at = 0;
        for marker in markers {
            at += source[at..].find(marker)?;
        }
        let open = at + source[at..].find('{')?;
        let mut depth = 0usize;
        for (i, c) in source[open..].char_indices() {
            match c {
                '{' => depth += 1,
                '}' => {
                    depth -= 1;
                    if depth == 0 {
                        return Some(&source[open..=open + i]);
                    }
                }
                _ => {}
            }
        }
        None
    }

    /// The Fluent ids a body resolves: every `tr!(some_key(` call, its name turned
    /// from the macro's snake case back to the catalogue's kebab case.
    fn resolved_keys(body: &str) -> BTreeSet<String> {
        let mut keys = BTreeSet::new();
        let mut rest = body;
        while let Some(i) = rest.find("tr!(") {
            rest = &rest[i + 4..];
            let name: String = rest
                .trim_start()
                .chars()
                .take_while(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || *c == '_')
                .collect();
            let after = rest.trim_start()[name.len()..].trim_start();
            if !name.is_empty() && after.starts_with('(') {
                keys.insert(name.replace('_', "-"));
            }
        }
        keys
    }

    fn source_root() -> std::path::PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR")).join("src")
    }

    #[test]
    fn every_site_lists_exactly_the_keys_it_resolves() {
        for site in SEEDING_SITES {
            let path = source_root().join(site.file);
            let source = std::fs::read_to_string(&path)
                .unwrap_or_else(|e| panic!("{}: {e}", path.display()));
            let body = function_body(&source, site.markers)
                .unwrap_or_else(|| panic!("{}: no function after {:?}", site.file, site.markers));
            let found = resolved_keys(body);
            let listed: BTreeSet<String> = site.keys.iter().map(|k| k.to_string()).collect();
            assert_eq!(
                found, listed,
                "{} {:?}: the keys this seeding function resolves changed; update its row",
                site.file, site.markers
            );
        }
    }

    /// `is_seeded` searches each row with a binary search.
    #[test]
    fn every_site_lists_its_keys_sorted_and_once() {
        for site in SEEDING_SITES {
            assert!(
                site.keys.windows(2).all(|w| w[0] < w[1]),
                "{} {:?}: keys must be sorted and unique",
                site.file,
                site.markers
            );
        }
    }

    /// A marker that matched a second, unrelated function would pin the wrong body,
    /// and the drift test would then guard the wrong keys. A row's first marker must
    /// therefore occur once in its file; a later marker only narrows inside it.
    #[test]
    fn every_site_starts_from_a_unique_marker() {
        for site in SEEDING_SITES {
            let path = source_root().join(site.file);
            let source = std::fs::read_to_string(&path)
                .unwrap_or_else(|e| panic!("{}: {e}", path.display()));
            let first = site.markers.first().copied().unwrap_or_default();
            assert_eq!(
                source.matches(first).count(),
                1,
                "{}: {first:?} must occur exactly once; add a marker before it",
                site.file
            );
        }
    }

    #[test]
    fn every_listed_key_is_in_the_source_catalogue() {
        let en_us = crate::startup::app_locales()
            .iter()
            .find(|(tag, _)| *tag == "en-US")
            .map(|(_, files)| files.join("\n"))
            .unwrap_or_default();
        for site in SEEDING_SITES {
            for key in site.keys {
                let defined = en_us.lines().any(|line| {
                    line.strip_prefix(key)
                        .is_some_and(|r| r.trim_start().starts_with('='))
                });
                assert!(
                    defined,
                    "{key} ({}) is not in the en-US catalogue",
                    site.file
                );
            }
        }
    }

    #[test]
    fn a_status_rung_is_seeded_and_a_menu_label_is_not() {
        assert!(is_seeded("status-draft"));
        assert!(is_seeded("create-book-end"));
        assert!(!is_seeded("menu-help-report"));
    }
}
