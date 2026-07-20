// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Which language is *this scene* written in?
//!
//! A manuscript is not monolingual. A novel can carry a Turkish chapter inside a French
//! book, and the language decides what folding **means**: in Turkish the dotted and dotless
//! `i` are different letters, so folding them together turns one word into another — and a
//! case-preserving rename that uppercased `i` to `I` instead of `İ` would silently rewrite
//! Turkish prose into different words.
//!
//! So the language is resolved **per item**, not per search. One pass of `run_search` folds
//! a French scene and a Turkish scene under different rules. The user's toggles
//! (`case_sensitive`, `diacritic_sensitive`) stay global — the language decides *how* to
//! fold, never *whether* to, or the same checkbox would mean different things in different
//! chapters of one book.
//!
//! ## The chain — two levels, no magic
//!
//! `BinderItem.dict_language` → `Work.dict_language`. That is the whole rule. An empty tag
//! means "use the Work's"; a tag that is unknown or **malformed** folds untailored rather
//! than failing, because it comes from a writer's project settings and a typo there must not
//! break searching.
//!
//! ## Why there is no implicit scope
//!
//! An earlier design let the nearest preceding `Book` supply a language to everything after
//! it. It was removed deliberately: the binder tree is **organisational only** and book
//! structure is a state machine over the flat item stream, so "the Book above" was a
//! stream-position rule that no other container could share. A language set on a *chapter*
//! folder silently did nothing, which is exactly the confusion a writer meets first — and the
//! rule could not be generalised, because `Work.chapter_mode` encodes a chapter either as a
//! real indent-nested folder (`Folder`) or as a bare positional marker with no nesting at all
//! (`Flat`).
//!
//! Propagation is now **explicit**: an item with a subtree offers "Apply to children", which
//! writes the value onto every descendant in one undo step (see
//! `OutlineViewModel::apply_dict_language_to_subtree`). Every item then means exactly what it
//! says, and what the Inspector shows on an item is what that item is checked against.
//!
//! Both `run_search` and `replace_in_project` resolve through this one function. Two copies
//! would drift, and the way a writer meets that drift is a rename that finds a word under
//! one set of rules and rewrites it under another.
//!
//! ## The tag *list* — one field, two readers
//!
//! `dict_language` is not a single tag but a **space-separated list** of BCP-47 tags; the
//! first is the *primary*. The field names *which languages the text is in* and nothing else
//! — mute state and dictionary availability live elsewhere. Two readers consume it, and they
//! read it differently:
//!
//! - **Search folds under [`primary`]** — a single language, because case-folding is
//!   inherently monolingual (Turkish `i` and French rules cannot both apply to one fold).
//! - **Spell-check accepts the union of [`all`] the tags** — a word is a mistake only when
//!   *every* listed dictionary rejects it (the Firefox model).
//!
//! A single-tag value is a one-element list, so every existing project keeps behaving
//! exactly as before.

use std::collections::{BTreeSet, HashMap};

use common::entities::BinderItem;
use common::types::EntityId;

/// The effective language tag of every item in one binder.
///
/// Each item resolves independently: its own `dict_language` if it has one, else the Work's.
/// Nothing an item's *neighbours* do can change its answer, so — unlike the Book-scope rule
/// this replaced — `items` need not be in any particular order, and a caller may resolve one
/// item without its siblings.
///
/// Items whose effective tag is empty are simply absent from the map; a caller reading a
/// missing entry as "untailored" is correct, and it keeps the map small (the overwhelmingly
/// common case is a manuscript with no language tags at all).
pub fn tags_in_binder(
    work_language: &[String],
    items: &[BinderItem],
    out: &mut HashMap<EntityId, Vec<String>>,
) {
    for item in items {
        let effective = if !item.dict_language.is_empty() {
            &item.dict_language
        } else {
            work_language
        };

        if !effective.is_empty() {
            out.insert(item.id, effective.to_vec());
        }
    }
}

/// The **primary** language of a tag list — the first tag, the one search folds under.
///
/// Empty when the list is empty. Skips blank entries: the list is a real `Vec` now, but a
/// writer can still leave an empty string in it through the UI, and a blank primary while a
/// real tag remains would silently untailor the fold.
pub fn primary(tags: &[String]) -> &str {
    tags.iter()
        .map(String::as_str)
        .find(|t| !t.trim().is_empty())
        .unwrap_or("")
}

/// **Every** non-blank tag in a list, in order — the set spell-check accepts.
pub fn all(tags: &[String]) -> impl Iterator<Item = &str> {
    tags.iter()
        .map(String::as_str)
        .map(str::trim)
        .filter(|t| !t.is_empty())
}

/// Best-effort *syntactic* canonicalisation of legacy tags toward BCP-47: underscores become
/// hyphens (`en_US` → `en-US`, `de_DE_frami` → `de-DE-frami`), applied per tag.
///
/// This is deliberately only the part that needs **no registry** — mapping an editorial
/// basename like `de-DE-frami` or a merged `fr-classique+reforme1990` to a real dictionary id
/// requires the registry's `system_basenames`, which lives in the UI layer. A caller that
/// wants the full resolution runs this first, then a registry lookup.
pub fn canonicalize(tags: &[String]) -> Vec<String> {
    all(tags).map(|t| t.replace('_', "-")).collect()
}

/// Every distinct language tag a project actually *uses*, for the missing-dictionary scan.
///
/// Resolves each item's effective list through [`tags_in_binder`] (so inheritance is
/// honoured) and unions them, plus the Work's own list. Operates on **one binder's** items —
/// a multi-binder caller extends one set across binders (`BTreeSet` unions for free).
pub fn effective_languages(work_language: &[String], items: &[BinderItem]) -> BTreeSet<String> {
    let mut map = HashMap::new();
    tags_in_binder(work_language, items, &mut map);

    let mut langs: BTreeSet<String> = BTreeSet::new();
    for list in map.values() {
        langs.extend(all(list).map(str::to_string));
    }
    // A Work language with no items still names a language worth having installed.
    langs.extend(all(work_language).map(str::to_string));
    langs
}

/// Whether a BCP-47 language tag is written right-to-left.
///
/// The export compiler sets each block's text direction from this so an Arabic or
/// Hebrew scene lays out correctly (and a book that mixes LTR and RTL scenes is handled
/// per block). An **explicit script subtag wins** — `az-Arab` is RTL, `ku-Latn` and
/// romanised `ar-Latn` are LTR — matching how [`WritingSystem`] detection treats scripts.
/// With no script subtag the primary language decides. Whitespace/casing tolerant, `_`
/// accepted as a separator; an empty or unknown tag is LTR (never fail on a writer's typo).
///
/// [`WritingSystem`]: https://en.wikipedia.org/wiki/Writing_system
pub fn is_rtl(tag: &str) -> bool {
    let lower = tag.trim().replace('_', "-").to_ascii_lowercase();
    if lower.is_empty() {
        return false;
    }
    let mut subtags = lower.split('-');
    let language = subtags.next().unwrap_or("");
    // BCP-47 places the script (if any) immediately after the language, as 4 letters.
    let script = subtags
        .next()
        .filter(|s| s.len() == 4 && s.bytes().all(|b| b.is_ascii_alphabetic()));
    if let Some(script) = script {
        // The script overrides the language default in both directions.
        return matches!(
            script,
            "arab" | "hebr" | "syrc" | "thaa" | "nkoo" | "samr" | "mand" | "rohg" | "yezi"
                | "adlm" | "mend" | "phlp"
        );
    }
    matches!(
        language,
        "ar" | "he" | "iw" | "fa" | "prs" | "ur" | "ps" | "sd" | "ug" | "yi" | "ji" | "dv"
            | "ckb" | "ku" | "ks" | "syr" | "arc" | "nqo" | "sam" | "rhg"
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::entities::BinderItemSubRole;

    /// The tests still *read* as space-separated lists, which is how a writer thinks of
    /// them; only the storage changed. This is the one place that translation happens.
    fn tags(s: &str) -> Vec<String> {
        s.split_whitespace().map(String::from).collect()
    }

    fn item(id: EntityId, sub_role: BinderItemSubRole, dict_language: &str) -> BinderItem {
        BinderItem {
            id,
            sub_role,
            dict_language: tags(dict_language),
            ..BinderItem::default()
        }
    }

    fn scene(id: EntityId, dict_language: &str) -> BinderItem {
        item(id, BinderItemSubRole::Scene, dict_language)
    }

    fn book(id: EntityId, dict_language: &str) -> BinderItem {
        item(id, BinderItemSubRole::Book, dict_language)
    }

    fn resolve(work: &str, items: &[BinderItem]) -> HashMap<EntityId, Vec<String>> {
        let mut out = HashMap::new();
        tags_in_binder(&tags(work), items, &mut out);
        out
    }

    /// The item's own tag wins over everything.
    #[test]
    fn an_items_own_tag_wins() {
        let got = resolve("fr-FR", &[book(1, "de-DE"), scene(2, "tr-TR")]);
        assert_eq!(got[&2], tags("tr-TR"));
    }

    /// **A Book no longer supplies a language to anything but itself.** An untagged scene
    /// takes the *Work's* language even when it sits after a tagged Book — the writer
    /// propagates a language with "Apply to children", which writes a real tag onto each
    /// descendant, and this function then simply reads it back.
    ///
    /// This is the deliberate replacement for the old Book-scope rule; it is what makes a
    /// language set on a *chapter* folder behave the same as one set on a Book (neither
    /// reaches a descendant on its own).
    #[test]
    fn a_book_does_not_lend_its_language_to_what_follows_it() {
        let got = resolve(
            "fr-FR",
            &[
                scene(1, ""),     // the Work's language
                book(2, "tr-TR"), // a Turkish book — Turkish for ITSELF only
                scene(3, ""),     //   …still the Work's language, not Turkish
                scene(4, "tr-TR"),//   …Turkish only because it says so (post "apply to children")
            ],
        );
        assert_eq!(got[&1], tags("fr-FR"));
        assert_eq!(got[&2], tags("tr-TR"), "the Book's own tag is its own");
        assert_eq!(got[&3], tags("fr-FR"), "no implicit scope: the Work's language wins");
        assert_eq!(got[&4], tags("tr-TR"), "an explicit tag is honoured");
    }

    /// Every container behaves alike. A chapter folder's tag reaches nothing on its own —
    /// exactly as a Book's does not — so the Inspector never shows a language that silently
    /// does nothing for one container type but works for another.
    #[test]
    fn a_chapter_folder_and_a_book_scope_identically_which_is_to_say_not_at_all() {
        let chapter = item(2, BinderItemSubRole::ChapterScene, "de-DE");
        let got = resolve("fr-FR", &[book(1, "tr-TR"), chapter, scene(3, "")]);
        assert_eq!(got[&1], tags("tr-TR"));
        assert_eq!(got[&2], tags("de-DE"));
        assert_eq!(
            got[&3], tags("fr-FR"),
            "neither the Book nor the chapter folder reaches the scene"
        );
    }

    /// Resolution is per-item, so order carries no meaning — the same items shuffled resolve
    /// identically. (Under the old Book-scope rule this was false, which is why the caller
    /// had to promise document order.)
    #[test]
    fn order_does_not_change_any_items_answer() {
        let forward = resolve("fr-FR", &[book(1, "tr-TR"), scene(2, ""), scene(3, "la")]);
        let shuffled = resolve("fr-FR", &[scene(3, "la"), scene(2, ""), book(1, "tr-TR")]);
        assert_eq!(forward, shuffled);
    }

    /// Two binders resolve independently — trivially so now, but pinned because the caller
    /// (`OpenDocsStore::build_language_map`) still folds several binders into one map.
    #[test]
    fn separate_binders_fold_into_one_map_without_interfering() {
        let mut out = HashMap::new();
        tags_in_binder(&tags("fr-FR"), &[book(1, "tr-TR"), scene(2, "")], &mut out);
        tags_in_binder(&tags("fr-FR"), &[scene(3, "")], &mut out);
        assert_eq!(out[&1], tags("tr-TR"));
        assert_eq!(out[&2], tags("fr-FR"));
        assert_eq!(out[&3], tags("fr-FR"));
    }

    /// No tags anywhere: nothing to record, and a caller reading a missing entry as
    /// "untailored" is right.
    #[test]
    fn an_untagged_project_records_nothing() {
        let got = resolve("", &[book(1, ""), scene(2, ""), scene(3, "")]);
        assert!(got.is_empty());
    }

    /// A Work language with no item or book tags reaches every item.
    #[test]
    fn the_work_language_reaches_every_item() {
        let got = resolve("tr", &[scene(1, ""), book(2, ""), scene(3, "")]);
        assert_eq!(got.len(), 3);
        assert!(got.values().all(|t| *t == tags("tr")));
    }

    /// The hazard the list shape introduces that the string never could: a real `Vec` can
    /// hold an empty element. The UI's pill field can leave one behind, and a blank primary
    /// while a real tag remains would silently untailor the fold — the exact class of bug the
    /// old `split_whitespace` grammar made impossible by construction.
    #[test]
    fn a_blank_entry_never_becomes_the_primary() {
        let list = vec![String::new(), "  ".to_string(), "tr-TR".to_string()];
        assert_eq!(primary(&list), "tr-TR");
        assert_eq!(all(&list).collect::<Vec<_>>(), vec!["tr-TR"]);
        assert_eq!(canonicalize(&list), vec!["tr-TR"]);
    }

    /// The primary is the first tag; a single-tag list is its own primary.
    #[test]
    fn primary_is_the_first_tag() {
        assert_eq!(primary(&tags("fr-FR")), "fr-FR");
        assert_eq!(primary(&tags("fr-FR en-US la")), "fr-FR");
        assert_eq!(primary(&tags("")), "");
        assert_eq!(primary(&tags("  fr-FR  en-US ")), "fr-FR", "whitespace-tolerant");
    }

    /// `all` yields every tag, and nothing for the empty string.
    #[test]
    fn all_yields_every_tag() {
        assert_eq!(all(&tags("fr-FR en-US la")).collect::<Vec<_>>(), ["fr-FR", "en-US", "la"]);
        assert_eq!(all(&tags("fr-FR")).collect::<Vec<_>>(), ["fr-FR"]);
        assert!(all(&tags("")).next().is_none());
        assert!(all(&tags("   ")).next().is_none());
    }

    /// Syntactic canonicalisation flips underscores to hyphens, per token.
    #[test]
    fn canonicalize_hyphenates_underscores() {
        assert_eq!(canonicalize(&tags("en_US")), tags("en-US"));
        assert_eq!(canonicalize(&tags("de_DE_frami")), tags("de-DE-frami"));
        assert_eq!(canonicalize(&tags("fr-FR en_US")), tags("fr-FR en-US"));
        assert_eq!(canonicalize(&tags("")), tags(""));
    }

    /// The scan collects the distinct union across a project's whole list per item.
    #[test]
    fn effective_languages_unions_the_lists() {
        let got = effective_languages(&tags("fr-FR"),
            &[
                scene(1, ""),               // the Work's fr-FR
                book(2, "de-DE en-US"),     // a bilingual book — for itself
                scene(3, ""),               // the Work's fr-FR (the Book lends nothing)
                scene(4, "la"),             // its own Latin
            ],
        );
        let want: BTreeSet<String> = ["fr-FR", "de-DE", "en-US", "la"]
            .into_iter()
            .map(str::to_string)
            .collect();
        assert_eq!(got, want);
    }

    /// An untagged project with a Work language still names that language for install.
    #[test]
    fn effective_languages_includes_the_bare_work_language() {
        let got = effective_languages(&tags("fr-FR"), &[scene(1, ""), scene(2, "")]);
        assert_eq!(got, ["fr-FR"].into_iter().map(str::to_string).collect());
    }

    /// RTL languages are RTL; Latin-script European languages are not.
    #[test]
    fn is_rtl_by_primary_language() {
        for t in ["ar", "ar-EG", "he", "he-IL", "iw", "fa", "fa-IR", "ur", "ps", "ckb", "yi", "dv"] {
            assert!(is_rtl(t), "{t} should be RTL");
        }
        for t in ["", "en", "en-US", "fr-FR", "de", "es-419", "tr", "ru", "zh-Hans"] {
            assert!(!is_rtl(t), "{t} should be LTR");
        }
    }

    /// An explicit script subtag overrides the language default, both ways.
    #[test]
    fn is_rtl_script_subtag_wins() {
        assert!(is_rtl("az-Arab"), "Azerbaijani in Arabic script is RTL");
        assert!(!is_rtl("ku-Latn"), "Kurdish in Latin script is LTR");
        assert!(!is_rtl("ar-Latn"), "romanised Arabic is LTR");
        assert!(is_rtl("sr-Arab"), "any language in the Arabic script is RTL");
    }

    /// Casing and the `_` separator are tolerated; a region subtag is not a script.
    #[test]
    fn is_rtl_is_separator_and_case_tolerant() {
        assert!(is_rtl("AR_eg"));
        assert!(is_rtl("HE"));
        assert!(!is_rtl("EN_us"), "US is a region, not an RTL script");
    }
}
