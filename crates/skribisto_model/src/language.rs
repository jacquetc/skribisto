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
//! No container lends its language to descendants — the binder tree is organisational only,
//! so "the nearest Book above" would be a stream-position rule no other container could
//! share. Propagation is instead **explicit**: an item with a subtree offers "Apply to
//! children", writing the value onto every descendant in one undo step (see
//! `OutlineViewModel::apply_dict_language_to_subtree`). Every item then means exactly what
//! it says, and what the Inspector shows on an item is what that item is checked against.
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
        // "Has a language" must mean the same thing here as it does in the pill field and
        // the Inspector, both of which ignore blanks. Testing `Vec::is_empty()` instead let
        // an item holding `[""]` count as tagged, so it stopped inheriting the Work's
        // language while the UI still showed it as inheriting — the displayed language and
        // the one actually used for folding would disagree.
        let effective = if has_tags(&item.dict_language) {
            &item.dict_language
        } else {
            work_language
        };

        // Stored already filtered, so nothing downstream has to re-derive it and the map
        // never carries a blank that a later reader might trust.
        let tags: Vec<String> = all(effective).map(String::from).collect();
        if !tags.is_empty() {
            out.insert(item.id, tags);
        }
    }
}

/// Whether a list names at least one real language, ignoring blanks.
///
/// The one place "does this have a language?" is decided, so the pill field, the Inspector
/// and [`tags_in_binder`] cannot answer it differently — which they did, and which made an
/// item holding `[""]` stop inheriting while still *looking* like it inherited.
pub fn has_tags(tags: &[String]) -> bool {
    all(tags).next().is_some()
}

/// Split the pre-list, space-separated form into a real list.
///
/// The legacy grammar has exactly three homes left — the pre-v4 bundle deserializer, the
/// legacy SQLite loader, and this function they both call. Keeping it in one place is the
/// point: the whole change was about not restating "split on whitespace" everywhere.
pub fn parse_legacy_list(s: &str) -> Vec<String> {
    s.split_whitespace().map(String::from).collect()
}

/// Syntactic canonicalisation of **one** tag toward BCP-47: `en_US` → `en-US`.
///
/// Split out from [`canonicalize`] because the dictionary registry resolves one token at a
/// time and had started inlining the rule rather than routing a single string through the
/// list helper — two copies of the same rule, one of them silent.
pub fn canonicalize_tag(tag: &str) -> String {
    tag.trim().replace('_', "-")
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
    all(tags).map(canonicalize_tag).collect()
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

/// Whether a BCP-47 tag is written in the **Arabic script** specifically.
///
/// [`is_rtl`] deliberately answers a layout question and so lumps together every
/// right-to-left script — Arabic, Hebrew, Syriac, Thaana, N'Ko. Punctuation is not a
/// layout question: Arabic-script languages substitute their own comma, semicolon and
/// question mark (`،` `؛` `؟`), whereas Hebrew keeps the ASCII ones outright. Mirroring
/// punctuation off `is_rtl` would therefore corrupt Hebrew prose, so this is a separate,
/// narrower test — the same reason [`counting`] carries its own `is_cjk_language` rather
/// than reusing a broader script check.
///
/// Same resolution order as [`is_rtl`]: an explicit script subtag wins in both directions
/// (`az-Arab` qualifies, `ar-Latn` does not), otherwise the primary language decides.
/// Whitespace/casing tolerant, `_` accepted as a separator, unknown tags answer `false`.
///
/// ## Two deliberate divergences from [`is_rtl`]'s language list
///
/// - **Bare `ku` is excluded.** Unqualified Kurdish means Kurmanji, which is written in a
///   Latin alphabet and takes ASCII punctuation; Sorani (`ckb`) is the Arabic-script one
///   and is included. `is_rtl` treats bare `ku` as RTL, which is arguable for layout but
///   would be plainly wrong here.
/// - **`rhg` is excluded.** Rohingya's modern orthography is Hanifi, its own script with
///   its own marks, not Arabic.
///
/// [`counting`]: crate::counting
pub fn uses_arabic_script(tag: &str) -> bool {
    let lower = tag.trim().replace('_', "-").to_ascii_lowercase();
    if lower.is_empty() {
        return false;
    }
    let mut subtags = lower.split('-');
    let language = subtags.next().unwrap_or("");
    let script = subtags
        .next()
        .filter(|s| s.len() == 4 && s.bytes().all(|b| b.is_ascii_alphabetic()));
    if let Some(script) = script {
        // `aran` is Nastaliq — a style of the Arabic script, not a different one, and the
        // usual way Urdu is tagged when the style is worth recording.
        return matches!(script, "arab" | "aran");
    }
    matches!(
        language,
        "ar" | "fa" | "prs" | "ur" | "ps" | "sd" | "ug" | "ckb" | "ks" | "pnb" | "bal" | "lrc"
            | "mzn" | "glk" | "arz" | "ary" | "acm" | "apc" | "ajp" | "aeb" | "afb" | "ars"
            | "ayl" | "kby"
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::entities::BinderItemSubRole;

    /// The tests still *read* as space-separated lists, which is how a writer thinks of
    /// them; only the storage changed.
    use super::parse_legacy_list as tags;

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

    /// A Book supplies a language to nothing but itself. An untagged scene takes the
    /// *Work's* language even when it sits after a tagged Book — the writer propagates one
    /// explicitly with "Apply to children".
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
    /// identically.
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

    /// An item holding only a blank tag must still inherit — `Vec::is_empty()` would count
    /// `[""]` as tagged and stop it inheriting, while the pill field and Inspector (which
    /// ignore blanks) would keep showing it as inherited.
    #[test]
    fn an_item_holding_only_a_blank_still_inherits() {
        let mut blank = scene(1, "");
        blank.dict_language = vec![String::new(), "  ".to_string()];
        let got = resolve("fr-FR", &[blank]);
        assert_eq!(
            got[&1],
            tags("fr-FR"),
            "a blank is not a language, so the Work's must still reach the item"
        );
    }

    /// The resolved map is stored already filtered, so no later reader can trust a blank.
    #[test]
    fn the_resolved_map_never_carries_a_blank() {
        let mut mixed = scene(1, "");
        mixed.dict_language = vec!["fr-FR".to_string(), String::new()];
        let got = resolve("en-US", &[mixed]);
        assert_eq!(got[&1], tags("fr-FR"));
    }

    /// `has_tags` is the single answer to "does this have a language", so the three call
    /// sites cannot drift apart again.
    #[test]
    fn has_tags_ignores_blanks() {
        assert!(!has_tags(&[]));
        assert!(!has_tags(&[String::new(), "   ".to_string()]));
        assert!(has_tags(&tags("fr-FR")));
    }

    /// A real `Vec` can hold an empty element (the UI's pill field can leave one behind); a
    /// blank primary while a real tag remains would silently untailor the fold.
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

    // ── uses_arabic_script ───────────────────────────────────────────────────

    #[test]
    fn arabic_script_languages_are_detected() {
        for t in [
            "ar", "ar-EG", "arz", "ary", "fa", "fa-IR", "prs", "ur", "ur-PK", "ps", "sd", "ug",
            "ckb", "ks", "pnb",
        ] {
            assert!(uses_arabic_script(t), "{t} is written in the Arabic script");
        }
        for t in ["", "en", "en-US", "fr-FR", "tr", "ru", "zh-Hans", "ja"] {
            assert!(!uses_arabic_script(t), "{t} is not Arabic-script");
        }
    }

    /// **The reason this function exists at all.** Every one of these is right-to-left, so
    /// `is_rtl` says yes to all of them — but none takes Arabic punctuation. Mirroring a
    /// typed `?` to `؟` off `is_rtl` would silently corrupt Hebrew, Yiddish and Divehi prose.
    #[test]
    fn other_rtl_scripts_are_not_arabic_script() {
        for t in ["he", "he-IL", "iw", "yi", "ji", "dv", "syr", "arc", "nqo", "rhg"] {
            assert!(is_rtl(t), "{t} is RTL — that is precisely the trap");
            assert!(
                !uses_arabic_script(t),
                "{t} is RTL but must NOT take Arabic punctuation"
            );
        }
    }

    /// An explicit script subtag overrides the language default, both ways — so a
    /// romanised Arabic tag keeps ASCII punctuation, and a Latin-script language written
    /// in Arabic script takes Arabic punctuation.
    #[test]
    fn arabic_script_subtag_wins_over_the_language() {
        assert!(uses_arabic_script("az-Arab"), "Azerbaijani in Arabic script");
        assert!(uses_arabic_script("ur-Aran"), "Nastaliq is the Arabic script");
        assert!(!uses_arabic_script("ar-Latn"), "romanised Arabic is not");
        assert!(!uses_arabic_script("ku-Latn"), "Kurmanji in Latin script");
        assert!(
            !uses_arabic_script("he-Hebr"),
            "an explicit Hebrew script is still not Arabic"
        );
    }

    /// Bare `ku` means Kurmanji, written in a Latin alphabet with ASCII punctuation —
    /// even though `is_rtl` calls it RTL. Sorani, the Arabic-script one, has its own tag.
    #[test]
    fn bare_kurdish_is_kurmanji_and_takes_ascii_punctuation() {
        assert!(is_rtl("ku"), "is_rtl treats bare ku as RTL");
        assert!(
            !uses_arabic_script("ku"),
            "…but Kurmanji is Latin-script, so no punctuation mirroring"
        );
        assert!(uses_arabic_script("ckb"), "Sorani is the Arabic-script one");
    }

    #[test]
    fn arabic_script_is_separator_and_case_tolerant() {
        assert!(uses_arabic_script("AR_eg"));
        assert!(uses_arabic_script("FA"));
        assert!(
            !uses_arabic_script("EN_us"),
            "US is a region subtag, not a script"
        );
        assert!(uses_arabic_script("  ar-SA  "), "whitespace is trimmed");
    }
}
