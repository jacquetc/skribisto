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
//! ## The chain
//!
//! `BinderItem.dict_language` → the nearest **Book** above it → `Work.dict_language`. An
//! empty tag at any level means "inherit"; a tag that is unknown or **malformed** folds
//! untailored rather than failing, because it comes from a writer's project settings and a
//! typo there must not break searching.
//!
//! ## "Nearest Book" is a scan, not a climb
//!
//! The binder tree is **organisational only**. Book structure is a state machine over the
//! flat, ordered item stream, so the book an item belongs to is simply *the most recent
//! `Book` item before it* — no parent pointers, no recursion. That is also why the scope
//! resets at each binder: two binders are two streams.
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

use common::entities::{BinderItem, BinderItemSubRole};
use common::types::EntityId;

/// The effective language tag of every item in one binder, in stream order.
///
/// `items` **must** be in document order — the order `BinderItems` is stored in, which is
/// the order the writer sees. Out of order, an item would inherit from whichever Book
/// happened to be visited last, which is not a language it is written in.
///
/// Items whose effective tag is empty are simply absent from the map; a caller reading a
/// missing entry as "untailored" is correct, and it keeps the map small (the overwhelmingly
/// common case is a manuscript with no language tags at all).
pub fn tags_in_binder(
    work_language: &str,
    items: &[BinderItem],
    out: &mut HashMap<EntityId, String>,
) {
    // The book currently in scope. Reset at the start of every binder: a second binder is a
    // second stream, not a continuation of the first.
    let mut book_language = String::new();

    for item in items {
        // A `Book` item opens a new book, and its tag governs everything after it until the
        // next one. An *untagged* Book deliberately clears the scope rather than leaving the
        // previous book's language in place — it inherits from the Work, and so does
        // everything under it.
        if item.sub_role == BinderItemSubRole::Book {
            book_language = item.dict_language.clone();
        }

        let effective = if !item.dict_language.is_empty() {
            item.dict_language.as_str()
        } else if !book_language.is_empty() {
            book_language.as_str()
        } else {
            work_language
        };

        if !effective.is_empty() {
            out.insert(item.id, effective.to_string());
        }
    }
}

/// The **primary** language of a tag list — the first tag, the one search folds under.
///
/// Empty when the list is empty. Whitespace-tolerant (a stray double space or a trailing
/// space never yields an empty primary while a real tag remains).
pub fn primary(tags: &str) -> &str {
    tags.split_whitespace().next().unwrap_or("")
}

/// **Every** tag in a list, in order, skipping empty gaps — the set spell-check accepts.
///
/// A single-tag value yields one element; the empty string yields none.
pub fn all(tags: &str) -> impl Iterator<Item = &str> {
    tags.split_whitespace()
}

/// Best-effort *syntactic* canonicalisation of one legacy tag toward BCP-47: underscores
/// become hyphens (`en_US` → `en-US`, `de_DE_frami` → `de-DE-frami`), applied per token so a
/// list is canonicalised whole.
///
/// This is deliberately only the part that needs **no registry** — mapping an editorial
/// basename like `de-DE-frami` or a merged `fr-classique+reforme1990` to a real dictionary id
/// requires the registry's `system_basenames`, which lives in the UI layer. A caller that
/// wants the full resolution runs this first, then a registry lookup.
pub fn canonicalize(tags: &str) -> String {
    all(tags)
        .map(|t| t.replace('_', "-"))
        .collect::<Vec<_>>()
        .join(" ")
}

/// Every distinct language tag a project actually *uses*, for the missing-dictionary scan.
///
/// Resolves each item's effective tag through [`tags_in_binder`] (so inheritance is honoured)
/// and unions the individual tags across every item's whole list, plus the Work's own list.
/// Operates on **one binder's** items in document order — a multi-binder caller extends one
/// set across binders (`BTreeSet` unions for free), exactly as the search use case walks
/// Work → Binders → BinderItems.
pub fn effective_languages(work_language: &str, items: &[BinderItem]) -> BTreeSet<String> {
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

#[cfg(test)]
mod tests {
    use super::*;

    fn item(id: EntityId, sub_role: BinderItemSubRole, dict_language: &str) -> BinderItem {
        BinderItem {
            id,
            sub_role,
            dict_language: dict_language.to_string(),
            ..BinderItem::default()
        }
    }

    fn scene(id: EntityId, dict_language: &str) -> BinderItem {
        item(id, BinderItemSubRole::Scene, dict_language)
    }

    fn book(id: EntityId, dict_language: &str) -> BinderItem {
        item(id, BinderItemSubRole::Book, dict_language)
    }

    fn resolve(work: &str, items: &[BinderItem]) -> HashMap<EntityId, String> {
        let mut out = HashMap::new();
        tags_in_binder(work, items, &mut out);
        out
    }

    /// The item's own tag wins over everything.
    #[test]
    fn an_items_own_tag_wins() {
        let got = resolve("fr-FR", &[book(1, "de-DE"), scene(2, "tr-TR")]);
        assert_eq!(got[&2], "tr-TR");
    }

    /// With no tag of its own, a scene takes the book it is in — which is the most recent
    /// `Book` *before it in the stream*, because containment is organisational only.
    #[test]
    fn a_scene_takes_the_book_it_is_in() {
        let got = resolve(
            "fr-FR",
            &[
                scene(1, ""),     // before any book: the Work's language
                book(2, "tr-TR"), // a Turkish book opens
                scene(3, ""),     //   …so this scene is Turkish
                scene(4, ""),     //   …and so is this one
                book(5, "de-DE"), // a German book opens
                scene(6, ""),     //   …the scope changed
            ],
        );
        assert_eq!(got[&1], "fr-FR", "before the first book: the Work");
        assert_eq!(got[&3], "tr-TR");
        assert_eq!(got[&4], "tr-TR");
        assert_eq!(got[&6], "de-DE", "the next Book replaces the scope");
    }

    /// An untagged Book clears the scope rather than leaving the previous book's language
    /// standing. A book with no language of its own is written in the Work's language — not
    /// in whatever the book before it happened to use.
    #[test]
    fn an_untagged_book_falls_back_to_the_work_rather_than_the_previous_book() {
        let got = resolve(
            "fr-FR",
            &[book(1, "tr-TR"), scene(2, ""), book(3, ""), scene(4, "")],
        );
        assert_eq!(got[&2], "tr-TR");
        assert_eq!(
            got[&4], "fr-FR",
            "the second book is untagged, so it is in the Work's language — NOT Turkish"
        );
    }

    /// Two binders are two streams. A book in one must not leak into the other.
    #[test]
    fn the_book_scope_does_not_leak_across_binders() {
        let mut out = HashMap::new();
        tags_in_binder("fr-FR", &[book(1, "tr-TR"), scene(2, "")], &mut out);
        tags_in_binder("fr-FR", &[scene(3, "")], &mut out);
        assert_eq!(out[&2], "tr-TR");
        assert_eq!(
            out[&3], "fr-FR",
            "a second binder starts a fresh stream — no Turkish book is in scope"
        );
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
        assert!(got.values().all(|t| t == "tr"));
    }

    /// The primary is the first tag; a single-tag list is its own primary.
    #[test]
    fn primary_is_the_first_tag() {
        assert_eq!(primary("fr-FR"), "fr-FR");
        assert_eq!(primary("fr-FR en-US la"), "fr-FR");
        assert_eq!(primary(""), "");
        assert_eq!(primary("  fr-FR  en-US "), "fr-FR", "whitespace-tolerant");
    }

    /// `all` yields every tag, and nothing for the empty string.
    #[test]
    fn all_yields_every_tag() {
        assert_eq!(all("fr-FR en-US la").collect::<Vec<_>>(), ["fr-FR", "en-US", "la"]);
        assert_eq!(all("fr-FR").collect::<Vec<_>>(), ["fr-FR"]);
        assert!(all("").next().is_none());
        assert!(all("   ").next().is_none());
    }

    /// Syntactic canonicalisation flips underscores to hyphens, per token.
    #[test]
    fn canonicalize_hyphenates_underscores() {
        assert_eq!(canonicalize("en_US"), "en-US");
        assert_eq!(canonicalize("de_DE_frami"), "de-DE-frami");
        assert_eq!(canonicalize("fr-FR en_US"), "fr-FR en-US");
        assert_eq!(canonicalize(""), "");
    }

    /// The scan collects the distinct union across a project's whole list per item.
    #[test]
    fn effective_languages_unions_the_lists() {
        let got = effective_languages(
            "fr-FR",
            &[
                scene(1, ""),               // inherits fr-FR
                book(2, "de-DE en-US"),     // a bilingual book
                scene(3, ""),               // inherits "de-DE en-US"
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
        let got = effective_languages("fr-FR", &[scene(1, ""), scene(2, "")]);
        assert_eq!(got, ["fr-FR"].into_iter().map(str::to_string).collect());
    }
}
