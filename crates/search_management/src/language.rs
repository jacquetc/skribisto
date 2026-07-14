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

use std::collections::HashMap;

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
}
