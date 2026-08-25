// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Registered rich-tooltip content for the writing-model vocabulary.
//!
//! Skribisto's "＋ Create" affordances (the outline header
//! [`CreateSplitButton`](crate::docks::create_split_button), the right-click
//! **Add ▸** submenu) and the **Convert to ▸** menu (outline context menu +
//! Inspector) attach these by *key* via `.rich_tooltip(key)`. Binding by key
//! (rather than building inline `TooltipContent` at each call site) buys two
//! things:
//!
//! 1. a type's explainer reads identically wherever it is offered, and
//! 2. the type names cited *inside* one explainer become live cascade links.
//!    A Fluent body may embed `[label](:key)` markup; the tooltip widget
//!    resolves each `:key` against the installed
//!    [`TooltipRegistry`](teksilo::widgets::tooltip::TooltipRegistry) and opens
//!    that type's own rich tooltip as a nested child. A link only renders (and
//!    resolves) when its key is registered — which is why every writing type is
//!    registered here once, at boot, through
//!    `TeksiloAppBuilder::register_tooltips` in `main.rs`.
//!
//! Two webs live here: the writing-model vocabulary (`wm-*`) and the word/character
//! **target** vocabulary (`goal-*`, plus `pace-plan`). They cross-link freely — a target's
//! explainer cites scenes and chapters, and the manuscript-counting one cites notes and
//! paratexts — which is the point: what counts toward a book's length is a question about
//! the writing model, not a separate topic.
//!
//! These double as a lightweight, in-place substitute for a separate Help
//! document: the short `text` says what the type is; the `more` disclosure
//! teaches the distinctive writing model (dual text + synopsis, the two chapter
//! encodings, why a book needs an explicit end, that any item can be compiled).
//! Keep the copy model-accurate and em-dash-free; both locales live in
//! `locales/{en-US,fr-FR}/tooltips.ftl` under the `wm-*` keys. The
//! [`crate::binder::create_labels`] mappers turn a `CreateType` / `PromoteTarget` into
//! the matching key below, so the menu-binding side and this registration side
//! cannot drift.

use teksilo::prelude::*; // tr!
use teksilo::widgets::tooltip::TooltipContent;

/// Stable registry keys — the `:key` targets of the cascade links and the ids
/// each Create / Convert row binds with `.rich_tooltip(..)`.
pub const WM_BOOK: &str = "wm-book";
pub const WM_PART: &str = "wm-part";
pub const WM_CHAPTER: &str = "wm-chapter";
pub const WM_SCENE: &str = "wm-scene";
pub const WM_NOTE: &str = "wm-note";
pub const WM_NOTE_FOLDER: &str = "wm-note-folder";
pub const WM_FOLDER: &str = "wm-folder";
pub const WM_PARATEXT: &str = "wm-paratext";
pub const WM_PARATEXT_FOLDER: &str = "wm-paratext-folder";
pub const WM_END_OF_BOOK: &str = "wm-end-of-book";
/// `CreateType::StoryBibleEntry`'s own row: structurally a Note (see that variant's
/// doc), but explained on its own terms, as the creation ceremony a bible entry gets
/// (a location, tags, aliases, a template) rather than the bare row a Note is.
pub const WM_STORY_BIBLE_ENTRY: &str = "wm-story-bible-entry";
/// A concept, not a create/convert row: cited by Scene / Note / Note folder /
/// Folder, so it is a cascade target only.
pub const WM_SYNOPSIS: &str = "wm-synopsis";
/// The two scene-break tiers. Not create/convert rows — they are marks the
/// author places *in the prose*, bound by the Format menu's entries.
pub const SCENE_BREAK_MINOR: &str = "scene-break-minor";
pub const SCENE_BREAK_MAJOR: &str = "scene-break-major";
/// The `discoverable` flag on a tag, surfaced as the "Find in prose" switch.
///
/// Registered rather than written inline because it is the pane's one control
/// whose label cannot explain itself: "Find in prose" says what turning it on
/// *does*, not what the tag *joins*, and a writer still benefits from both
/// halves. It is offered in two places, the switch on every row of the Tags
/// settings pane and the same switch in the tag pill field's "New tag…" form,
/// and both must say the same thing.
pub const WM_FIND_IN_PROSE: &str = "wm-find-in-prose";
/// The Story bible place itself (C2): a card grid, one per notes folder, of
/// every item a [`WM_FIND_IN_PROSE`] tag has matched.
///
/// Was, until the checkbox's own rename, the id `WM_FIND_IN_PROSE` now holds:
/// the pane's tooltip for "does turning this on make the item searchable",
/// attached directly to the switch. Once the checkbox stopped being called
/// "Story bible", the phrase was free to name the place instead, and the
/// concept moved with it, keeping the *id* (nothing persisted depends on the
/// string "wm-story-bible" changing, only what it teaches) while the *text*
/// became a genuinely different explanation. Reachable from the Help window's
/// glossary and from [`WM_FIND_IN_PROSE`]'s own "more" disclosure, not hung on
/// any control of its own, since the place already names itself on its own
/// tab.
pub const WM_STORY_BIBLE: &str = "wm-story-bible";

// ── Word / character targets ──────────────────────────────────────────────────
//
// A second cascade web, registered here for the same two reasons as the writing-model one:
// a target explains itself identically wherever it is offered, and the concepts each
// explainer cites become live links to their own entries. It is also, in practice, the
// app's only documentation of what does and does not get counted — `FEATURES.md` records
// that there is no help surface anywhere else.
/// The per-item target itself: the Inspector's field, the Overview's column header.
pub const GOAL_TARGET: &str = "goal-target";
/// Words vs characters, on the New Work picker and the Settings one.
pub const GOAL_UNIT: &str = "goal-unit";
/// The bar and its reading. A concept, not a control: a cascade target only.
pub const GOAL_PROGRESS: &str = "goal-progress";
/// What the manuscript admits — the single gate every count in this app applies.
pub const GOAL_MANUSCRIPT_WORDS: &str = "goal-manuscript-words";
/// Why a row's own length can show while it counts toward nothing.
pub const GOAL_EXPORTABLE: &str = "goal-exportable";
/// The Distribute action and its preview.
pub const GOAL_DISTRIBUTE: &str = "goal-distribute";
/// The informational "what the targets inside add up to" line, which must never be
/// mistaken for a target.
pub const GOAL_SUBTREE_TOTAL: &str = "goal-subtree-total";
/// The two milestone kinds.
pub const GOAL_MILESTONE: &str = "goal-milestone";
/// The Book's Pace plan, cited by the milestone explainer and citing it back.
pub const PACE_PLAN: &str = "pace-plan";

/// Every registered writing-model key. Consumed by the headless test that
/// asserts every menu row's key and every `[..](:key)` cascade link in the
/// Fluent bodies resolves to something registered here.
pub const WM_KEYS: &[&str] = &[
    WM_BOOK,
    WM_PART,
    WM_CHAPTER,
    WM_SCENE,
    WM_NOTE,
    WM_NOTE_FOLDER,
    WM_FOLDER,
    WM_PARATEXT,
    WM_PARATEXT_FOLDER,
    WM_END_OF_BOOK,
    WM_STORY_BIBLE_ENTRY,
    WM_SYNOPSIS,
    SCENE_BREAK_MINOR,
    SCENE_BREAK_MAJOR,
    WM_FIND_IN_PROSE,
    WM_STORY_BIBLE,
    GOAL_TARGET,
    GOAL_UNIT,
    GOAL_PROGRESS,
    GOAL_MANUSCRIPT_WORDS,
    GOAL_EXPORTABLE,
    GOAL_DISTRIBUTE,
    GOAL_SUBTREE_TOTAL,
    GOAL_MILESTONE,
    PACE_PLAN,
];

// ── Feature concepts ─────────────────────────────────────────────────────────
//
// A third web beside the writing-model and target ones. These name the things a
// project *has* rather than the things a book is made of: what a tag is, what a
// backup protects you from, why a returning DOCX is recognised. They cross-link to
// each other and to the two webs above, and each is browsable as a glossary entry in
// the Help window (`crate::help`) as well as hoverable on its own control.
pub const CONCEPT_TAG: &str = "concept-tag";
pub const CONCEPT_LABEL: &str = "concept-label";
pub const CONCEPT_POINT_OF_VIEW: &str = "concept-point-of-view";
pub const CONCEPT_EPIGRAPH: &str = "concept-epigraph";
pub const CONCEPT_FOOTNOTE: &str = "concept-footnote";
pub const CONCEPT_CHAPTER_MODE: &str = "concept-chapter-mode";
pub const CONCEPT_COMMENT: &str = "concept-comment";
pub const CONCEPT_BACKUP: &str = "concept-backup";
pub const CONCEPT_VERSION: &str = "concept-version";
pub const CONCEPT_TRASH: &str = "concept-trash";
pub const CONCEPT_SPELLCHECK: &str = "concept-spellcheck";
pub const CONCEPT_SEARCH_REPLACE: &str = "concept-search-replace";
pub const CONCEPT_NOTE_TEMPLATE: &str = "concept-note-template";
pub const CONCEPT_TEXT_REPLACEMENT: &str = "concept-text-replacement";
pub const CONCEPT_SMART_PUNCTUATION: &str = "concept-smart-punctuation";
pub const CONCEPT_EXPORT_STYLE: &str = "concept-export-style";
pub const CONCEPT_ROUND_TRIP_MARKS: &str = "concept-round-trip-marks";

/// Every registered feature-concept key.
pub const CONCEPT_KEYS: &[&str] = &[
    CONCEPT_TAG,
    CONCEPT_LABEL,
    CONCEPT_POINT_OF_VIEW,
    CONCEPT_EPIGRAPH,
    CONCEPT_FOOTNOTE,
    CONCEPT_CHAPTER_MODE,
    CONCEPT_COMMENT,
    CONCEPT_BACKUP,
    CONCEPT_VERSION,
    CONCEPT_TRASH,
    CONCEPT_SPELLCHECK,
    CONCEPT_SEARCH_REPLACE,
    CONCEPT_NOTE_TEMPLATE,
    CONCEPT_TEXT_REPLACEMENT,
    CONCEPT_SMART_PUNCTUATION,
    CONCEPT_EXPORT_STYLE,
    CONCEPT_ROUND_TRIP_MARKS,
];

/// Every key this module registers, across all three webs.
///
/// The drift tests and the Help window's glossary both read this rather than either
/// array, so a key added to one and forgotten by the other is a build failure rather
/// than a concept that quietly stops being reachable.
pub fn all_keys() -> impl Iterator<Item = &'static str> {
    WM_KEYS.iter().copied().chain(CONCEPT_KEYS.iter().copied())
}

/// The writing-model rich tooltips, registered once at boot. Each carries a
/// short `text` (what it is) plus a `more` disclosure (the teaching body, whose
/// cited types cascade to their own entries here).
pub fn writing_model_tooltips() -> Vec<TooltipContent> {
    vec![
        TooltipContent::new(WM_BOOK, tr!(wm_book())).with_more(tr!(wm_book_more())),
        TooltipContent::new(WM_PART, tr!(wm_part())).with_more(tr!(wm_part_more())),
        TooltipContent::new(WM_CHAPTER, tr!(wm_chapter())).with_more(tr!(wm_chapter_more())),
        TooltipContent::new(WM_SCENE, tr!(wm_scene())).with_more(tr!(wm_scene_more())),
        TooltipContent::new(WM_NOTE, tr!(wm_note())).with_more(tr!(wm_note_more())),
        TooltipContent::new(WM_NOTE_FOLDER, tr!(wm_note_folder()))
            .with_more(tr!(wm_note_folder_more())),
        TooltipContent::new(WM_FOLDER, tr!(wm_folder())).with_more(tr!(wm_folder_more())),
        TooltipContent::new(WM_PARATEXT, tr!(wm_paratext())).with_more(tr!(wm_paratext_more())),
        TooltipContent::new(WM_PARATEXT_FOLDER, tr!(wm_paratext_folder()))
            .with_more(tr!(wm_paratext_folder_more())),
        TooltipContent::new(WM_END_OF_BOOK, tr!(wm_end_of_book()))
            .with_more(tr!(wm_end_of_book_more())),
        TooltipContent::new(WM_STORY_BIBLE_ENTRY, tr!(wm_story_bible_entry()))
            .with_more(tr!(wm_story_bible_entry_more())),
        TooltipContent::new(WM_SYNOPSIS, tr!(wm_synopsis())).with_more(tr!(wm_synopsis_more())),
        // The shortcut chip tracks a rebind, so the accelerator shown here can
        // never drift from the one actually registered.
        TooltipContent::new(SCENE_BREAK_MINOR, tr!(scene_break_minor()))
            .with_more(tr!(scene_break_minor_more()))
            .for_shortcut("format.scene_break"),
        TooltipContent::new(SCENE_BREAK_MAJOR, tr!(scene_break_major()))
            .with_more(tr!(scene_break_major_more()))
            .for_shortcut("format.major_scene_break"),
        TooltipContent::new(WM_FIND_IN_PROSE, tr!(wm_find_in_prose()))
            .with_more(tr!(wm_find_in_prose_more())),
        TooltipContent::new(WM_STORY_BIBLE, tr!(wm_story_bible()))
            .with_more(tr!(wm_story_bible_more())),
        TooltipContent::new(GOAL_TARGET, tr!(goal_target())).with_more(tr!(goal_target_more())),
        TooltipContent::new(GOAL_UNIT, tr!(goal_unit())).with_more(tr!(goal_unit_more())),
        // The `more` key is `-explained` rather than `-more`: `goal-progress-words` and
        // friends already live in `main.ftl` as the printed readout, and a
        // `goal-progress-more` beside them would read as one of that family.
        TooltipContent::new(GOAL_PROGRESS, tr!(goal_progress()))
            .with_more(tr!(goal_progress_explained())),
        TooltipContent::new(GOAL_MANUSCRIPT_WORDS, tr!(goal_manuscript_words()))
            .with_more(tr!(goal_manuscript_words_more())),
        TooltipContent::new(GOAL_EXPORTABLE, tr!(goal_exportable()))
            .with_more(tr!(goal_exportable_more())),
        TooltipContent::new(GOAL_DISTRIBUTE, tr!(goal_distribute()))
            .with_more(tr!(goal_distribute_more())),
        TooltipContent::new(GOAL_SUBTREE_TOTAL, tr!(goal_subtree_total()))
            .with_more(tr!(goal_subtree_total_more())),
        TooltipContent::new(GOAL_MILESTONE, tr!(goal_milestone()))
            .with_more(tr!(goal_milestone_more())),
        TooltipContent::new(PACE_PLAN, tr!(pace_plan())).with_more(tr!(pace_plan_more())),
        TooltipContent::new(CONCEPT_TAG, tr!(concept_tag())).with_more(tr!(concept_tag_more())),
        TooltipContent::new(CONCEPT_LABEL, tr!(concept_label()))
            .with_more(tr!(concept_label_more())),
        TooltipContent::new(CONCEPT_POINT_OF_VIEW, tr!(concept_point_of_view()))
            .with_more(tr!(concept_point_of_view_more())),
        TooltipContent::new(CONCEPT_EPIGRAPH, tr!(concept_epigraph()))
            .with_more(tr!(concept_epigraph_more())),
        TooltipContent::new(CONCEPT_FOOTNOTE, tr!(concept_footnote()))
            .with_more(tr!(concept_footnote_more())),
        TooltipContent::new(CONCEPT_CHAPTER_MODE, tr!(concept_chapter_mode()))
            .with_more(tr!(concept_chapter_mode_more())),
        TooltipContent::new(CONCEPT_COMMENT, tr!(concept_comment()))
            .with_more(tr!(concept_comment_more())),
        TooltipContent::new(CONCEPT_BACKUP, tr!(concept_backup()))
            .with_more(tr!(concept_backup_more())),
        TooltipContent::new(CONCEPT_VERSION, tr!(concept_version()))
            .with_more(tr!(concept_version_more())),
        TooltipContent::new(CONCEPT_TRASH, tr!(concept_trash()))
            .with_more(tr!(concept_trash_more())),
        TooltipContent::new(CONCEPT_SPELLCHECK, tr!(concept_spellcheck()))
            .with_more(tr!(concept_spellcheck_more())),
        TooltipContent::new(CONCEPT_SEARCH_REPLACE, tr!(concept_search_replace()))
            .with_more(tr!(concept_search_replace_more())),
        TooltipContent::new(CONCEPT_NOTE_TEMPLATE, tr!(concept_note_template()))
            .with_more(tr!(concept_note_template_more())),
        TooltipContent::new(CONCEPT_TEXT_REPLACEMENT, tr!(concept_text_replacement()))
            .with_more(tr!(concept_text_replacement_more())),
        TooltipContent::new(CONCEPT_SMART_PUNCTUATION, tr!(concept_smart_punctuation()))
            .with_more(tr!(concept_smart_punctuation_more())),
        TooltipContent::new(CONCEPT_EXPORT_STYLE, tr!(concept_export_style()))
            .with_more(tr!(concept_export_style_more())),
        TooltipContent::new(CONCEPT_ROUND_TRIP_MARKS, tr!(concept_round_trip_marks()))
            .with_more(tr!(concept_round_trip_marks_more())),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    /// Keys deliberately not hung on any control, with the reason.
    ///
    /// A concept reaches the reader two ways: in place, on the control it describes, and
    /// cold, as a glossary entry in the Help window. The second is free once registered;
    /// the first is a decision per concept. This list is where "no control" is *stated*
    /// rather than left as an absence nobody can tell from an oversight.
    const BROWSE_ONLY: &[(&str, &str)] = &[
        (
            WM_SYNOPSIS,
            "a concept cited by other entries, not a thing you create; the synopsis box \
             itself is labelled and needs no explainer",
        ),
        (
            WM_STORY_BIBLE,
            "the place already names itself on its own notes-folder tab, so nothing in \
             the app needs a '?' pointing at it; reached instead through the Help \
             window's glossary and through wm-find-in-prose-more's own cascade link",
        ),
    ];

    /// Every `.rich_tooltip(..)` / `RichTip::new(..)` argument appearing anywhere under
    /// `src/`, as raw source text.
    ///
    /// A directory walk rather than a file list, for the reason `settings_keys`' own
    /// drift test gives: the case worth catching is a call site in a file nobody thought
    /// to enumerate.
    fn attachment_sources() -> String {
        fn walk(dir: &Path, out: &mut String) {
            let Ok(entries) = std::fs::read_dir(dir) else {
                return;
            };
            for entry in entries.flatten() {
                let path = entry.path();
                // Three files must not be scanned, or the test passes by construction
                // and proves nothing. Both of these were live bugs in this very test
                // before the exclusion was added, which is why they are named here
                // rather than assumed:
                //
                //  * `tooltip_registry.rs` DECLARES every constant, so a key would
                //    always "appear in sources" via its own `pub const` line;
                //  * `help.rs` and its module list every key in the glossary table, so
                //    every concept would look attached the moment it was browsable.
                let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
                if name == "help.rs" || name == "help" || name == "tooltip_registry.rs" {
                    continue;
                }
                if path.is_dir() {
                    walk(&path, out);
                } else if path.extension().is_some_and(|e| e == "rs")
                    && let Ok(text) = std::fs::read_to_string(&path)
                {
                    out.push_str(&text);
                }
            }
        }
        let mut out = String::new();
        walk(&Path::new(env!("CARGO_MANIFEST_DIR")).join("src"), &mut out);
        out
    }

    #[test]
    fn every_registered_concept_is_hung_on_a_control_or_declared_browse_only() {
        let sources = attachment_sources();
        let excused: Vec<&str> = BROWSE_ONLY.iter().map(|(k, _)| *k).collect();

        // The constant's *name*, not its value: attachment sites bind by constant
        // (`.rich_tooltip(WM_SCENE)`), and a literal string would be the drift this test
        // exists to catch.
        let missing: Vec<&str> = all_keys()
            .filter(|key| !excused.contains(key))
            .filter(|key| {
                // `wm-scene` -> `WM_SCENE`
                let ident = key.replace('-', "_").to_uppercase();
                !sources.contains(&ident)
            })
            .collect();

        assert!(
            missing.is_empty(),
            "these registered concepts are hung on no control and are not declared \
             browse-only: {missing:?}\nAttach each with `.rich_tooltip(KEY)` or \
             `RichTip::new(KEY, ..)`, or add it to BROWSE_ONLY with the reason."
        );
    }

    #[test]
    fn browse_only_names_only_registered_keys() {
        for (key, _) in BROWSE_ONLY {
            assert!(
                all_keys().any(|k| k == *key),
                "BROWSE_ONLY names '{key}', which is not a registered key"
            );
        }
    }

    /// Extract every `(:key)` cascade target from a Fluent source blob,
    /// skipping comment lines (`#` / `##` / `###`) — a comment may carry a
    /// literal `[label](:key)` example that is not a real cascade link.
    fn cascade_targets(ftl: &str) -> Vec<String> {
        let mut out = Vec::new();
        for line in ftl.lines() {
            if line.trim_start().starts_with('#') {
                continue;
            }
            let mut rest = line;
            while let Some(i) = rest.find("(:") {
                rest = &rest[i + 2..];
                let end = rest.find(')').unwrap_or(rest.len());
                out.push(rest[..end].to_string());
                rest = &rest[end..];
            }
        }
        out
    }

    #[test]
    fn registration_covers_every_key_exactly_once() {
        let tips = writing_model_tooltips();
        let keys: Vec<&str> = tips.iter().map(|t| t.key.as_str()).collect();
        // Every declared key is registered, and nothing extra is.
        for k in all_keys() {
            assert!(keys.contains(&k), "key {k} declared but not registered");
        }
        assert_eq!(
            keys.len(),
            all_keys().count(),
            "registered set != the declared keys"
        );
        // Every registered entry teaches (has a `more` disclosure).
        for t in &tips {
            assert!(t.has_more(), "tooltip {} is missing its `more` body", t.key);
        }
    }

    /// The cascade only works if every `[label](:key)` link in either locale
    /// points at a key we actually register. A typo (`:wm-scenes`) would render
    /// as dead text and silently break the "cited type shows its own tooltip"
    /// contract, so pin it in both locales.
    #[test]
    fn every_cascade_link_resolves_in_both_locales() {
        for (locale, ftl) in [
            ("en-US", include_str!("../locales/en-US/tooltips.ftl")),
            ("fr-FR", include_str!("../locales/fr-FR/tooltips.ftl")),
        ] {
            let targets = cascade_targets(ftl);
            assert!(
                !targets.is_empty(),
                "{locale}: no cascade links found — the wm-* bodies lost their [label](:key) markup"
            );
            let known: Vec<&str> = all_keys().collect();
            for t in &targets {
                assert!(
                    known.contains(&t.as_str()),
                    "{locale}: cascade link (:{t}) points at an unregistered key"
                );
            }
        }
    }
}
