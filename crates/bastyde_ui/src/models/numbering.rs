// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Reading the open Work's structural numbering — one mapping, one policy, one default.
//!
//! Five row sources need the same two things: turn `BinderItemDto`s into the
//! [`ItemMeta`] stream [`skribisto_model::numbering`] counts, and decide whether this
//! manuscript numbers at all. Both used to be copied into each of them, which is how the
//! outline came to default numbering *on* after a failed `get_work` while the corkboard,
//! Overview, stream and export tree all defaulted *off* — the same transient error showing
//! "Chapter 3" in one dock and no badge at all in the next.
//!
//! The counting itself is not here. That is
//! [`skribisto_model::numbering::number_map`], the one function the exporter also calls;
//! this module only feeds it and gates it.

use std::collections::HashMap;

use frontend::AppContext;
use frontend::commands::work_commands;
use frontend::direct_access::BinderItemDto;
use skribisto_model::compile::ItemMeta;
use skribisto_model::numbering::{self, Numbered, NumberingRules};

/// One binder item as the numbering pass sees it.
///
/// The single place this seven-field mapping lives. It was written out by hand in five row
/// sources, so adding `exclude_from_numbering` meant editing all five in lockstep — and
/// missing one would have silently numbered that view against a stale view of the model,
/// with no compiler error, because every field of `ItemMeta` is default-able.
pub fn item_meta_of(it: &BinderItemDto) -> ItemMeta {
    ItemMeta {
        id: it.id,
        role: it.role.clone(),
        sub_role: it.sub_role.clone(),
        indent: it.indent as i32,
        activated: it.activated,
        is_exportable: it.is_exportable,
        exclude_from_numbering: it.exclude_from_numbering,
    }
}

/// Every structural row's ordinal for `work_id`, keyed by item id.
///
/// `metas` must be the **whole** Work's ordered stream — every binder, binder-major, in
/// each binder's stored relationship order. Handing this a scoped or filtered slice is the
/// one way to misuse it: a chapter's number would then depend on which binder is showing,
/// what is typed in a search box, or which container a tab happens to be open on.
///
/// Returns an empty map when the manuscript does not number (`Work.number_chapters`), and
/// **also when the Work cannot be read**. Numbering off is the safe failure default:
/// showing no badge for one rebuild is a smaller lie than showing a number computed from
/// guessed rules, and it is what every caller but one already did.
pub fn numbers_for_work(
    ctx: &AppContext,
    work_id: u64,
    metas: &[ItemMeta],
) -> HashMap<u64, Numbered> {
    match work_commands::get_work(ctx, &work_id) {
        Ok(Some(w)) if w.number_chapters => numbering::number_map(
            metas,
            NumberingRules {
                part_resets_chapter: w.part_resets_chapter,
            },
        ),
        _ => HashMap::new(),
    }
}

/// The name an untitled structural row falls back to — "Chapter 3", in the row's own
/// language — or `None` when it has no ordinal and so no generated name.
///
/// Built here rather than in each row source so the language resolution happens once: the
/// row's own `dict_language` tag wins, else the Work's, which is what the exporter's
/// `HeadingLanguage::Auto` does per row. The string itself comes from
/// `skribisto_compiler::headings::numbered`, the very function that writes the heading into
/// the exported file, so an untitled chapter reads the same in the binder and in the book.
///
/// `DigitStyle::Western` unconditionally: the digit style is an *export style*'s choice
/// (Mashriq vs. Maghreb), and the binder is not an export — a writer's tree should not
/// change shape because they picked a different preset in a dialog.
pub fn fallback_label_for(
    it: &BinderItemDto,
    numbered: Option<&Numbered>,
    work_langs: &[String],
) -> Option<String> {
    if !it.title.trim().is_empty() {
        return None; // it has a name of its own
    }
    // Structural rows only. A scene or a note has no generated name to fall back on, and
    // labelling one "Scene" would be noise rather than information.
    let level = skribisto_model::numbering::level_of(&it.sub_role)?;
    let tags: &[String] = if it.dict_language.iter().any(|t| !t.is_empty()) {
        &it.dict_language
    } else {
        work_langs
    };
    let lang = skribisto_model::language::primary(tags);
    Some(match numbered {
        // Numbered: the full heading, "Chapter 3".
        Some(n) => skribisto_compiler::headings::numbered(
            lang,
            level,
            n.number(),
            skribisto_compiler::DigitStyle::Western,
        ),
        // **Neither numbered nor titled** — an untitled prologue, the combination Tidy
        // chapter titles… made reachable. It has no ordinal and no name, and it used to
        // render as a completely blank row: not "an unnamed chapter", just nothing.
        // The bare structural word at least says what the row *is*.
        //
        // The export deliberately does not do this. A writer who has removed both the
        // number and the title has said what they want, and synthesising the word
        // "Chapter" as a heading would put a word in their book that they did not write;
        // there, the chapter's page break carries the boundary instead.
        None => skribisto_compiler::headings::word(lang, level).to_string(),
    })
}

/// The Work's own language tags, for [`fallback_label_for`]'s per-row resolution.
pub fn work_language_tags(ctx: &AppContext, work_id: u64) -> Vec<String> {
    work_commands::get_work(ctx, &work_id)
        .ok()
        .flatten()
        .map(|w| skribisto_model::language::parse_legacy_list(&w.dict_language.join(" ")))
        .unwrap_or_default()
}

/// What a row is called on screen, and whether its ordinal badge shows beside it.
///
/// One rule, because it has to hold identically in the outline, its three pickers, the
/// Overview, the stream, the corkboard and the export tree — a writer who clears a chapter's
/// title must not find it named in one dock and a bare "3." in the next.
///
/// * **Titled** → the writer's title, with the badge beside it.
/// * **Untitled, with a generated name** → that name ("Chapter 3" when numbered, "Chapter"
///   when not), and **no** badge: showing both gives "3. Chapter 3", the exact duplication
///   this feature removes. [`fallback_label_for`] decides which.
/// * **Untitled, with no generated name** → the empty string. Only non-structural rows
///   reach this: a scene or a note has nothing to be called, and inventing "Scene" would be
///   noise on every untitled row in the tree.
///
/// This mirrors the exporter's own `NumberAndTitle` composition, which renders an untitled
/// chapter as its number alone and a titled one as number-then-title.
pub fn label_and_badge(
    title: &str,
    fallback: Option<&str>,
    number: Option<usize>,
) -> (String, Option<usize>) {
    if title.trim().is_empty() {
        match fallback {
            Some(f) => (f.to_string(), None),
            None => (String::new(), number),
        }
    } else {
        (title.to_string(), number)
    }
}

/// [`numbers_for_work`] straight from a slice of DTOs, for the callers that hold one.
pub fn numbers_for_items(
    ctx: &AppContext,
    work_id: u64,
    items: &[BinderItemDto],
) -> HashMap<u64, Numbered> {
    let metas: Vec<ItemMeta> = items.iter().map(item_meta_of).collect();
    numbers_for_work(ctx, work_id, &metas)
}

#[cfg(test)]
mod tests {
    use super::*;
    use frontend::common::entities::BinderItemSubRole;
    use skribisto_model::compile::StreamLevel;

    fn dto(title: &str, sub_role: BinderItemSubRole) -> BinderItemDto {
        BinderItemDto {
            title: title.to_string(),
            sub_role,
            ..Default::default()
        }
    }

    fn numbered(level: StreamLevel, n: usize) -> Numbered {
        Numbered {
            level,
            book: n,
            part: n,
            chapter: n,
        }
    }

    /// A titled row is named by its writer, whatever else is true of it.
    #[test]
    fn a_titled_row_has_no_fallback() {
        let it = dto("The Storm", BinderItemSubRole::ChapterScene);
        let n = numbered(StreamLevel::Chapter, 3);
        assert_eq!(fallback_label_for(&it, Some(&n), &["en".into()]), None);
    }

    /// Untitled but numbered: the full heading, exactly what the export prints.
    #[test]
    fn an_untitled_numbered_chapter_is_named_by_its_heading() {
        let it = dto("", BinderItemSubRole::ChapterScene);
        let n = numbered(StreamLevel::Chapter, 3);
        assert_eq!(
            fallback_label_for(&it, Some(&n), &["en".into()]).as_deref(),
            Some("Chapter 3")
        );
        assert_eq!(
            fallback_label_for(&it, Some(&n), &["fr".into()]).as_deref(),
            Some("Chapitre 3")
        );
    }

    /// **Untitled *and* unnumbered — an untitled prologue.** It has no ordinal and no name,
    /// and it used to render as a completely blank row. The bare structural word at least
    /// says what it is.
    #[test]
    fn an_untitled_unnumbered_chapter_still_says_what_it_is() {
        let it = dto("", BinderItemSubRole::ChapterScene);
        assert_eq!(
            fallback_label_for(&it, None, &["en".into()]).as_deref(),
            Some("Chapter")
        );
        assert_eq!(
            fallback_label_for(&it, None, &["fr".into()]).as_deref(),
            Some("Chapitre")
        );
        // And a Part says "Part", not "Chapter".
        let part = dto("", BinderItemSubRole::Part);
        assert_eq!(
            fallback_label_for(&part, None, &["en".into()]).as_deref(),
            Some("Part")
        );
    }

    /// A scene or a note has no generated name, so it stays blank rather than acquiring a
    /// label like "Scene" that would be noise on every untitled row in the binder.
    #[test]
    fn a_non_structural_row_gets_no_fallback() {
        for sr in [BinderItemSubRole::Scene, BinderItemSubRole::Note] {
            let it = dto("", sr);
            assert_eq!(fallback_label_for(&it, None, &["en".into()]), None);
        }
    }

    /// The display rule: a badge accompanies a real title, and never a generated name —
    /// "3. Chapter 3" is the duplication the whole feature removes.
    #[test]
    fn the_badge_never_doubles_a_generated_name() {
        assert_eq!(
            label_and_badge("The Storm", None, Some(3)),
            ("The Storm".to_string(), Some(3))
        );
        assert_eq!(
            label_and_badge("", Some("Chapter 3"), Some(3)),
            ("Chapter 3".to_string(), None)
        );
        assert_eq!(label_and_badge("", None, None), (String::new(), None));
    }
}
