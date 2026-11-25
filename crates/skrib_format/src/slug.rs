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
        ContentRole::ScratchpadText => Some("scratchpad"),
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

/// Directory holding a bundle's stored resources, relative to its root: one folder
/// per resource, named by its `uid`, holding the file under its own name.
pub const RESOURCES_DIR: &str = "resources";

/// The longest a stored file's name may be, in characters and in bytes: well inside
/// what every file system accepts (255 bytes on Linux and macOS, 255 UTF-16 units on
/// Windows, which a name within 200 bytes never passes).
const MAX_NAME_CHARS: usize = 120;
const MAX_NAME_BYTES: usize = 200;

/// The longest suffix read as a file's extension, and so kept when a long name is cut.
const MAX_EXTENSION_CHARS: usize = 16;

/// Bundle-root-relative path of a stored resource's file: `resources/<uid>/<name>`.
///
/// The `uid` folder is what lets two resources keep the same file name, and what keeps
/// the path stable across a reopen (an `EntityId` is re-minted on every load, see
/// [`note_template_relpath`]). The name is the writer's own, made safe by
/// [`safe_file_name`], so a folder project shows the file as they know it.
pub fn resource_relpath(uid: Uuid, file_name: &str) -> String {
    format!(
        "{RESOURCES_DIR}/{}/{}",
        uid.hyphenated(),
        safe_file_name(file_name)
    )
}

/// [`resource_relpath`] under the name an earlier build gave the file, or `None` when it
/// is today's (see [`legacy_safe_file_name`]). For finding an existing file only.
pub fn legacy_resource_relpath(uid: Uuid, file_name: &str) -> Option<String> {
    let legacy = legacy_safe_file_name(file_name)?;
    Some(format!("{RESOURCES_DIR}/{}/{legacy}", uid.hyphenated()))
}

/// `name` as one safe file name: no separator, no character a file system refuses, no
/// name Windows reserves, not only dots, and not too long, in characters or in bytes. A
/// name cut to fit keeps its extension, so the file still opens with the application it
/// belongs to. NFC, so a name that came from macOS compares equal to the same name typed
/// elsewhere.
///
/// The one rule for a file name that came from outside and has to become a path here: a
/// resource's name comes from a shared project, and `..`, a separator or `CON` in it must
/// not be able to escape its folder or fail the write.
///
/// A name Windows takes for a device gets a `_` in front, which is enough: `_CON.pdf` and
/// `_COM¹ .tar.gz` are ordinary names everywhere. Those are `CON`, `PRN`, `AUX`, `NUL`,
/// `CONIN$`, `CONOUT$`, and `COM` or `LPT` followed by one digit or a superscript `¹`,
/// `²` or `³`, without case, with or without an extension, and with the spaces Windows
/// strips before it compares.
///
/// What this returns for a name is where a stored resource lives on disk
/// (`resources/<uid>/<this>`), so a change to it moves every existing file it names. The
/// rule has changed once (see [`legacy_safe_file_name`]), and the lookups of an existing
/// file fall back to the earlier name: [`crate::media::resource_file`] and the folder
/// reader.
pub fn safe_file_name(name: &str) -> String {
    let out = made_safe(name);
    if windows_device(&out) {
        format!("_{out}")
    } else {
        out
    }
}

/// [`safe_file_name`] as it was before it reserved every Windows device name, or `None`
/// when that gives the same name as today's rule. Only for finding a file a project
/// saved by an earlier build still holds under that name; nothing is written under it.
///
/// The earlier rule reserved `CON`, `PRN`, `AUX` and `NUL`, and `COM` or `LPT` followed by
/// one ASCII digit, compared without case on the part of the name before its first dot,
/// exactly as spelled. So it let through `COM¹` to `COM³` and `LPT¹` to `LPT³` (superscript
/// digits), `CONIN$` and `CONOUT$`, and a reserved name with spaces before a first dot
/// that does not start the extension (`CON .tar.gz`), which Windows strips before it
/// compares. (Before a lone extension, `CON .txt`, the trim of the stem already took
/// them, and the name was reserved.) A file stored under one of those
/// names is renamed to today's as the project opens ([`crate::media::prune_resources_dir`]);
/// until then, and wherever the rename could not be made, it is found under this one.
pub fn legacy_safe_file_name(name: &str) -> Option<String> {
    let out = made_safe(name);
    let stem = out
        .split('.')
        .next()
        .unwrap_or_default()
        .to_ascii_uppercase();
    let reserved = ["CON", "PRN", "AUX", "NUL"].contains(&stem.as_str())
        || ((stem.starts_with("COM") || stem.starts_with("LPT"))
            && stem.len() == 4
            && stem[3..].chars().all(|c| c.is_ascii_digit()));
    let legacy = if reserved { format!("_{out}") } else { out };
    (legacy != safe_file_name(name)).then_some(legacy)
}

/// Whether Windows takes the file name `name` for a device rather than a file: `CON`,
/// `PRN`, `AUX`, `NUL`, `CONIN$`, `CONOUT$`, and `COM` or `LPT` followed by one digit, `0`
/// to `9` or a superscript `¹`, `²` or `³`. Without case, with or without an extension,
/// and as Windows compares: on the part before the first dot, with the spaces it strips
/// at the end of that part (`CON .tar.gz`). Dots and spaces at the very end of a name,
/// and before a lone extension (`CON .txt`, `nul..txt`), are already gone
/// ([`made_safe`]).
fn windows_device(name: &str) -> bool {
    let base = name
        .split('.')
        .next()
        .unwrap_or_default()
        .trim_end_matches(' ')
        .to_uppercase();
    if ["CON", "PRN", "AUX", "NUL", "CONIN$", "CONOUT$"].contains(&base.as_str()) {
        return true;
    }
    let Some(digit) = base
        .strip_prefix("COM")
        .or_else(|| base.strip_prefix("LPT"))
    else {
        return false;
    };
    let mut chars = digit.chars();
    matches!(
        (chars.next(), chars.next()),
        (Some('0'..='9' | '¹' | '²' | '³'), None)
    )
}

/// The part of [`safe_file_name`] that both its rules share: every character made safe,
/// the ends trimmed, the length cut, never empty. Not yet checked for a device name.
fn made_safe(name: &str) -> String {
    use unicode_normalization::UnicodeNormalization;
    let mut out: String = name
        .nfc()
        .map(|c| match c {
            '/' | '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|' => '_',
            c if c.is_control() => '_',
            c => c,
        })
        .collect();
    out = trim_name(&out);
    let (stem, extension) = match out.rsplit_once('.') {
        Some((stem, extension))
            if !stem.is_empty()
                && !extension.is_empty()
                && extension.chars().count() <= MAX_EXTENSION_CHARS
                && extension.chars().all(char::is_alphanumeric) =>
        {
            (stem.to_string(), format!(".{extension}"))
        }
        _ => (out.clone(), String::new()),
    };
    let chars_left = MAX_NAME_CHARS.saturating_sub(extension.chars().count());
    let bytes_left = MAX_NAME_BYTES.saturating_sub(extension.len());
    let mut kept = String::new();
    for c in stem.chars().take(chars_left) {
        if kept.len() + c.len_utf8() > bytes_left {
            break;
        }
        kept.push(c);
    }
    // A cut can leave a space or a dot at the end, which Windows refuses.
    let kept = trim_name(&kept);
    out = if kept.is_empty() && !extension.is_empty() {
        format!("_{extension}")
    } else {
        format!("{kept}{extension}")
    };
    if out.is_empty() {
        out = "_".to_string();
    }
    out
}

/// `name` without the spaces and dots a file system drops or refuses at its ends.
fn trim_name(name: &str) -> String {
    name.trim().trim_matches('.').trim().to_string()
}

/// Bundle-root-relative path of one note template's Djot body.
///
/// `slugify` is what makes this safe for a name the writer typed: a template called
/// `Fiche/perso` or `CON` or `..` still lands on a legal, non-escaping single path
/// segment. The [`short_id`] prefix keeps two same-slug templates from colliding, exactly
/// as it does for prose blobs.
///
/// **Keyed by `uid`, not by the row's `EntityId`.** It used to take a `file_id`, and that
/// was the same bug the prose blobs had: `load_work` re-mints every `EntityId`, so the
/// name changed on every reopen, `prune_dir` deleted the previous one, and a folder-shape
/// project rewrote its entire `templates/` directory each session. A renamed template
/// still renames its own blob — that is correct, and it is one file.
pub fn note_template_relpath(uid: Uuid, name: &str) -> String {
    format!("{TEMPLATES_DIR}/{}-{}.djot", short_id(uid), slugify(name))
}

#[cfg(test)]
mod tests {
    use super::{nearest_titled_ancestor, nearest_titled_ancestors, safe_file_name, slugify};

    #[test]
    fn a_file_name_is_made_safe_and_never_a_path() {
        assert_eq!(safe_file_name("../../etc/passwd"), "_.._etc_passwd");
        assert_eq!(safe_file_name("Tide: tables?.pdf"), "Tide_ tables_.pdf");
        assert_eq!(safe_file_name("CON.pdf"), "_CON.pdf");
        assert_eq!(safe_file_name("  "), "_");
        assert_eq!(safe_file_name("map.desktop"), "map.desktop");
    }

    /// Every name Windows takes for a device, as Windows matches it: without case, with
    /// or without an extension, and with the spaces it strips before the extension.
    #[test]
    fn every_windows_device_name_is_reserved() {
        for (name, safe) in [
            ("COM¹.pdf", "_COM¹.pdf"),
            ("com².pdf", "_com².pdf"),
            ("LPT³", "_LPT³"),
            ("lpt¹.tar.gz", "_lpt¹.tar.gz"),
            ("CONIN$.txt", "_CONIN$.txt"),
            ("conout$", "_conout$"),
            // The spaces and dots before a lone extension go with the cut's trim.
            ("CON .txt", "_CON.txt"),
            ("Com1  .pdf", "_Com1.pdf"),
            ("NUL..txt", "_NUL.txt"),
            ("aux. .txt", "_aux.txt"),
            // Before a first dot that is not the extension's, they stay, and Windows
            // still strips them to compare.
            ("CON .tar.gz", "_CON .tar.gz"),
            ("nul .back-up", "_nul .back-up"),
            ("PRN. ", "_PRN"),
            ("nul . ", "_nul"),
            ("LPT0.txt", "_LPT0.txt"),
        ] {
            assert_eq!(super::safe_file_name(name), safe, "{name:?}");
        }
        for name in [
            "COM10.pdf",
            "COM⁴.pdf",
            "LPT.txt",
            "CONSOLE.txt",
            "CONIN.txt",
            "CON-notes.txt",
            "_CON.pdf",
        ] {
            assert_eq!(super::safe_file_name(name), name, "{name:?}");
        }
    }

    /// The earlier rule, kept for finding a file an earlier build stored, gives a name
    /// only where today's rule gives another.
    #[test]
    fn the_earlier_rule_answers_only_where_today_s_differs() {
        use super::legacy_safe_file_name as legacy;
        assert_eq!(legacy("COM¹.pdf").as_deref(), Some("COM¹.pdf"));
        assert_eq!(legacy("CONIN$.txt").as_deref(), Some("CONIN$.txt"));
        assert_eq!(legacy("CON .tar.gz").as_deref(), Some("CON .tar.gz"));
        assert_eq!(legacy("CON .txt"), None);
        assert_eq!(legacy("CON.pdf"), None);
        assert_eq!(legacy("NUL..txt"), None);
        assert_eq!(legacy("Village map.pdf"), None);
        assert_eq!(legacy("Tide: tables?.pdf"), None);
        let uid = uuid::Uuid::nil();
        assert_eq!(
            super::legacy_resource_relpath(uid, "COM¹.pdf").as_deref(),
            Some("resources/00000000-0000-0000-0000-000000000000/COM¹.pdf")
        );
    }

    #[test]
    fn a_long_file_name_is_cut_and_keeps_its_extension() {
        let long = format!("{}.pdf", "é".repeat(300));
        let safe = safe_file_name(&long);
        assert!(safe.ends_with(".pdf"));
        assert!(safe.len() <= 200);
    }

    #[test]
    fn a_resource_lives_in_its_own_folder() {
        let uid = uuid::Uuid::nil();
        assert_eq!(
            super::resource_relpath(uid, "Carte/du village.png"),
            "resources/00000000-0000-0000-0000-000000000000/Carte_du village.png"
        );
    }

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
