// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Path naming for the exploded form: binder directory names and prose file
//! names. Slugs are cosmetic and diff-friendly, and keep the title's actual
//! (non-ASCII) characters — titles are visible, git-tracked file names, so
//! folding them away would make non-Latin-script books unreadable on disk.
//! Uniqueness is guaranteed by the numeric `file_id` prefix, not the slug.

use common::entities::ContentRole;
use unicode_normalization::UnicodeNormalization;

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
        ContentRole::BookTitle
        | ContentRole::BookSubtitle
        | ContentRole::PartTitle
        | ContentRole::ChapterTitle => None,
    }
}

/// Prose file name `<content_id>-<item-slug>.<kind>.djot`, or `None` if `role`
/// is not a prose role.
pub fn prose_file_name(content_id: u64, item_title: &str, role: &ContentRole) -> Option<String> {
    let kind = prose_kind(role)?;
    Some(format!("{content_id}-{}.{kind}.djot", slugify(item_title)))
}

/// Bundle-root-relative path of a prose file within a binder directory.
pub fn prose_relpath(binder_dir: &str, file_name: &str) -> String {
    format!("binders/{binder_dir}/text/{file_name}")
}

/// The directory holding the note-template bodies, relative to the bundle root.
pub const TEMPLATES_DIR: &str = "templates";

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
    use super::slugify;

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
}
