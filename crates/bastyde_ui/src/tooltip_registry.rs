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
//!    [`TooltipRegistry`](bastyde::widgets::tooltip::TooltipRegistry) and opens
//!    that type's own rich tooltip as a nested child. A link only renders (and
//!    resolves) when its key is registered — which is why every writing type is
//!    registered here once, at boot, through
//!    `BastydeAppBuilder::register_tooltips` in `main.rs`.
//!
//! These double as a lightweight, in-place substitute for a separate Help
//! document: the short `text` says what the type is; the `more` disclosure
//! teaches the distinctive writing model (dual text + synopsis, the two chapter
//! encodings, why a book needs an explicit end, that any item can be compiled).
//! Keep the copy model-accurate and em-dash-free; both locales live in
//! `locales/{en-US,fr-FR}.ftl` under the `wm-*` keys. The
//! [`crate::create_labels`] mappers turn a `CreateType` / `PromoteTarget` into
//! the matching key below, so the menu-binding side and this registration side
//! cannot drift.

use bastyde::prelude::*; // tr!
use bastyde::widgets::tooltip::TooltipContent;

/// Stable registry keys — the `:key` targets of the cascade links and the ids
/// each Create / Convert row binds with `.rich_tooltip(..)`.
pub const WM_BOOK: &str = "wm-book";
pub const WM_PART: &str = "wm-part";
pub const WM_CHAPTER: &str = "wm-chapter";
pub const WM_SCENE: &str = "wm-scene";
pub const WM_NOTE: &str = "wm-note";
pub const WM_NOTE_FOLDER: &str = "wm-note-folder";
pub const WM_FOLDER: &str = "wm-folder";
pub const WM_END_OF_BOOK: &str = "wm-end-of-book";
/// A concept, not a create/convert row: cited by Scene / Note / Note folder /
/// Folder, so it is a cascade target only.
pub const WM_SYNOPSIS: &str = "wm-synopsis";

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
    WM_END_OF_BOOK,
    WM_SYNOPSIS,
];

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
        TooltipContent::new(WM_END_OF_BOOK, tr!(wm_end_of_book()))
            .with_more(tr!(wm_end_of_book_more())),
        TooltipContent::new(WM_SYNOPSIS, tr!(wm_synopsis())).with_more(tr!(wm_synopsis_more())),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Extract every `(:key)` cascade target from a Fluent source blob.
    fn cascade_targets(ftl: &str) -> Vec<String> {
        let mut out = Vec::new();
        let mut rest = ftl;
        while let Some(i) = rest.find("(:") {
            rest = &rest[i + 2..];
            let end = rest.find(')').unwrap_or(rest.len());
            out.push(rest[..end].to_string());
            rest = &rest[end..];
        }
        out
    }

    #[test]
    fn registration_covers_every_key_exactly_once() {
        let tips = writing_model_tooltips();
        let keys: Vec<&str> = tips.iter().map(|t| t.key.as_str()).collect();
        // Every declared key is registered, and nothing extra is.
        for k in WM_KEYS {
            assert!(keys.contains(k), "key {k} declared but not registered");
        }
        assert_eq!(keys.len(), WM_KEYS.len(), "registered set != WM_KEYS");
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
            ("en-US", include_str!("../locales/en-US.ftl")),
            ("fr-FR", include_str!("../locales/fr-FR.ftl")),
        ] {
            let targets = cascade_targets(ftl);
            assert!(
                !targets.is_empty(),
                "{locale}: no cascade links found — the wm-* bodies lost their [label](:key) markup"
            );
            for t in &targets {
                assert!(
                    WM_KEYS.contains(&t.as_str()),
                    "{locale}: cascade link (:{t}) points at an unregistered key"
                );
            }
        }
    }
}
