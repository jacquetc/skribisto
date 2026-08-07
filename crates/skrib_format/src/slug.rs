// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Path naming for the exploded form: binder directory names and prose file
//! names. Slugs are cosmetic and diff-friendly, and keep the title's actual
//! (non-ASCII) characters — titles are visible, git-tracked file names, so
//! folding them away would make non-Latin-script books unreadable on disk.
//! Uniqueness is guaranteed by the [`short_id`] prefix, not the slug.
//!
//! ## Why the prefix is derived from the item's `uid`
//!
//! A prose file name must survive a save → load → save cycle unchanged, or the
//! exploded shape's whole reason for existing — a git history where editing one
//! scene touches one file — collapses into a wholesale rename on every reopen.
//! `EntityId`s cannot carry that weight: the loader remaps every `file_id` to a
//! fresh store id, and `next_id` is a process-lifetime counter that is never
//! reset, so closing a project and reopening it in the same session hands every
//! row a different number. `BinderItem.uid` is the project's durable identity
//! (minted once on create, never on update) and is what everything else
//! persisted about an item already keys on; the file name now follows that rule
//! too.
//!
//! The slug is chosen so that *reordering* renames nothing either: it comes from
//! the item's own title, or — for the untitled scenes that make up most of a
//! continuous manuscript — from its nearest titled **ancestor**, found by
//! crossing indent levels rather than by list position.

use common::entities::ContentRole;
use unicode_normalization::UnicodeNormalization;
use uuid::Uuid;

/// lowercase alphanumeric runs (any script) joined by `-`, trimmed; `"item"` when empty.
///
/// Input is NFC-normalised first, so a decomposed (NFD) and precomposed (NFC)
/// encoding of the same title always produce the same slug.
pub fn slugify(input: &str) -> String {
    let mut out = String::new();
    let mut prev_dash = false;
    for ch in input.nfc() {
        if ch.is_alphanumeric() {
            out.extend(ch.to_lowercase());
            prev_dash = false;
        } else if !prev_dash && !out.is_empty() {
            out.push('-');
            prev_dash = true;
        }
    }
    while out.ends_with('-') {
        out.pop();
    }
    if out.chars().count() > 40 {
        out = out.chars().take(40).collect();
        while out.ends_with('-') {
            out.pop();
        }
    }
    if out.is_empty() {
        "item".to_string()
    } else {
        out
    }
}

/// `binders/NN-slug` directory name for the binder at ordinal `index` (0-based).
pub fn binder_dir_name(index: usize, binder_name: &str) -> String {
    format!("{:02}-{}", index + 1, slugify(binder_name))
}

/// The `.djot` suffix for a prose content role, or `None` for inline/title roles.
pub fn prose_kind(role: &ContentRole) -> Option<&'static str> {
    match role {
        ContentRole::SceneText => Some("scene"),
        ContentRole::NoteText => Some("note"),
        ContentRole::SynopsisText => Some("synopsis"),
        ContentRole::EpigraphText => Some("epigraph"),
        ContentRole::ParatextText => Some("paratext"),
        ContentRole::BookTitle
        | ContentRole::BookSubtitle
        | ContentRole::PartTitle
        | ContentRole::ChapterTitle => None,
    }
}

/// A short, stable, collision-resistant tag derived from a durable `uid`.
///
/// **Hashed, never a substring of the uid.** `common::uid::fixture_uid(n)` is
/// `Uuid::from_u128(n)`, the house test-fixture generator: for every realistic
/// `n` its entropy lives in the *low* bytes, so the leading hex digits are all
/// zero and a raw prefix would make every fixture row collide. Hashing spreads
/// any id source's bits uniformly, and stays correct if the uid scheme is ever
/// changed for one that orders by time (which would concentrate the entropy at
/// the other end again).
pub fn short_id(uid: Uuid) -> String {
    blake3::hash(uid.as_bytes()).to_hex()[..8].to_string()
}

/// The nearest titled ancestor of `items[idx]`, given `(indent, title)` pairs in
/// binder order — or `None` when nothing above it is titled.
///
/// Walks backwards, and each time it crosses to a strictly shallower indent it
/// either takes that row's title or, if that ancestor is itself untitled, keeps
/// climbing from the new, shallower ceiling. Deliberately **not** the preceding
/// sibling: siblings share an indent, so consulting one would make the file name
/// depend on list position, and reordering a scene would rename its neighbours.
///
/// O(depth) for one query, but a caller resolving every row of a binder should
/// reach for [`nearest_titled_ancestors`] instead — one query per row here adds
/// up to O(n²) over a long run of untitled siblings, which is the common shape
/// (most scenes in a continuous manuscript have no title).
pub fn nearest_titled_ancestor<'a>(items: &[(i64, &'a str)], idx: usize) -> Option<&'a str> {
    let mut ceiling = items.get(idx)?.0;
    let mut j = idx;
    while j > 0 {
        j -= 1;
        let (indent, title) = items[j];
        if indent < ceiling {
            if !title.trim().is_empty() {
                return Some(title);
            }
            ceiling = indent;
        }
    }
    None
}

/// [`nearest_titled_ancestor`] for every row at once, in one forward pass.
///
/// Same answers, computed differently: rather than each row independently
/// walking back over everything before it, this keeps a stack of the ancestor
/// chain currently open, one entry per indent level, each already carrying its
/// *own* resolved answer (its own title if it has one, otherwise whatever it
/// inherited from its own nearest titled ancestor). A new row then only has to
/// look at the top of the stack — the climbing has already happened once, when
/// each ancestor was pushed — so the whole binder resolves in one O(n) pass
/// instead of the O(n²) a long run of untitled containers would otherwise cost
/// (a chapter of a thousand untitled scenes is exactly this run).
pub fn nearest_titled_ancestors<'a>(items: &[(i64, &'a str)]) -> Vec<Option<&'a str>> {
    // (indent, what a child of this entry should resolve to).
    let mut stack: Vec<(i64, Option<&'a str>)> = Vec::new();
    let mut out = Vec::with_capacity(items.len());
    for &(indent, title) in items {
        while stack.last().is_some_and(|&(ind, _)| ind >= indent) {
            stack.pop();
        }
        let ancestor = stack.last().and_then(|&(_, resolved)| resolved);
        out.push(ancestor);
        let resolved_here = if title.trim().is_empty() {
            ancestor
        } else {
            Some(title)
        };
        stack.push((indent, resolved_here));
    }
    out
}

/// Prose file name `<short_id>-<slug>.<kind>.djot`, or `None` if `role` is not a
/// prose role.
///
/// `slug_source` is the already-resolved name to slug: the item's own title, or
/// its [`nearest_titled_ancestor`]'s when it has none. When it is blank the
/// content kind is used, so an untitled scene lands on `…-scene.scene.djot`
/// rather than the anonymous `item` every untitled row used to share.
pub fn prose_file_name(item_uid: Uuid, slug_source: &str, role: &ContentRole) -> Option<String> {
    let kind = prose_kind(role)?;
    let source = if slug_source.trim().is_empty() {
        kind
    } else {
        slug_source
    };
    Some(format!(
        "{}-{}.{kind}.djot",
        short_id(item_uid),
        slugify(source)
    ))
}

/// Bundle-root-relative path of a prose file within a binder directory.
pub fn prose_relpath(binder_dir: &str, file_name: &str) -> String {
    format!("binders/{binder_dir}/text/{file_name}")
}

/// The directory holding the note-template bodies, relative to the bundle root.
pub const TEMPLATES_DIR: &str = "templates";

/// Directory holding a bundle's binary assets, relative to its root.
pub const ASSETS_DIR: &str = "assets";

/// Bundle-root-relative path of one note template's Djot body.
///
/// `slugify` is what makes this safe for a name the writer typed: a template called
/// `Fiche/perso` or `CON` or `..` still lands on a legal, non-escaping single path
/// segment. The `file_id` prefix keeps two same-slug templates from colliding, exactly
/// as it does for prose blobs.
pub fn note_template_relpath(file_id: u64, name: &str) -> String {
    format!("{TEMPLATES_DIR}/{file_id}-{}.djot", slugify(name))
}

#[cfg(test)]
mod tests {
    use super::{nearest_titled_ancestor, nearest_titled_ancestors, slugify};

    #[test]
    fn empty_input_falls_back_to_item() {
        assert_eq!(slugify(""), "item");
    }

    #[test]
    fn no_alphanumeric_chars_falls_back_to_item() {
        assert_eq!(slugify("!!! ... ???"), "item");
    }

    #[test]
    fn ascii_is_lowercased_and_separator_collapsed() {
        assert_eq!(slugify("  Hello   World!  "), "hello-world");
    }

    #[test]
    fn accented_latin_is_preserved_not_dropped() {
        assert_eq!(slugify("Café"), "café");
        assert_eq!(slugify("Écriture"), "écriture");
    }

    #[test]
    fn non_latin_scripts_are_preserved() {
        assert_eq!(slugify("日本語のタイトル"), "日本語のタイトル");
        assert_eq!(slugify("Заголовок"), "заголовок");
    }

    #[test]
    fn long_multibyte_input_truncates_on_char_boundary() {
        let title = "日".repeat(50);
        let slug = slugify(&title);
        assert_eq!(slug.chars().count(), 40);
    }

    #[test]
    fn nfd_and_nfc_forms_of_the_same_title_produce_the_same_slug() {
        let nfc = "Café"; // U+00E9 LATIN SMALL LETTER E WITH ACUTE
        let nfd = "Cafe\u{0301}"; // 'e' + U+0301 COMBINING ACUTE ACCENT
        assert_eq!(slugify(nfc), slugify(nfd));
        assert_eq!(slugify(nfd), "café");
    }

    /// [`nearest_titled_ancestors`] must agree with [`nearest_titled_ancestor`]
    /// for every row — it is a faster way to compute the exact same answers, not
    /// a different rule. Covers an untitled chapter over untitled scenes (must
    /// keep climbing past the chapter to the book), a titled chapter (must stop
    /// there), and a row with nothing above it (must be `None`).
    #[test]
    fn the_batch_form_agrees_with_the_per_row_form_on_every_row() {
        let items: Vec<(i64, &str)> = vec![
            (0, "The Book"),    // 0
            (1, ""),            // 1: untitled chapter
            (2, ""),            // 2: untitled scene under it -> "The Book"
            (2, ""),            // 3: sibling scene -> also "The Book"
            (1, "Chapter Two"), // 4: titled chapter
            (2, ""),            // 5: untitled scene under it -> "Chapter Two"
            (0, ""),            // 6: untitled second book -> None
            (1, ""),            // 7: untitled chapter under it -> None
            (2, ""),            // 8: untitled scene -> None
        ];

        let batch = nearest_titled_ancestors(&items);
        assert_eq!(batch.len(), items.len());
        for (idx, expected) in batch.iter().enumerate() {
            assert_eq!(
                *expected,
                nearest_titled_ancestor(&items, idx),
                "row {idx} disagrees between the batch and per-row forms"
            );
        }

        assert_eq!(batch[2], Some("The Book"));
        assert_eq!(batch[3], Some("The Book"));
        assert_eq!(batch[5], Some("Chapter Two"));
        assert_eq!(batch[8], None);
    }

    #[test]
    fn the_batch_form_on_an_empty_list_is_empty() {
        assert!(nearest_titled_ancestors(&[]).is_empty());
    }
}
