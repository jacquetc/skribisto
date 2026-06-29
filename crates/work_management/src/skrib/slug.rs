//! Path naming for the exploded form: binder directory names and prose file
//! names. Slugs are cosmetic and diff-friendly; uniqueness is guaranteed by the
//! numeric `file_id` prefix, so a plain ASCII fold (no Unicode-normalisation
//! dependency) is enough.

use common::entities::ContentRole;

/// lowercase, ASCII-alphanumeric runs joined by `-`, trimmed; `"item"` when empty.
pub fn slugify(input: &str) -> String {
    let mut out = String::new();
    let mut prev_dash = false;
    for ch in input.chars() {
        if ch.is_ascii_alphanumeric() {
            out.push(ch.to_ascii_lowercase());
            prev_dash = false;
        } else if !prev_dash && !out.is_empty() {
            out.push('-');
            prev_dash = true;
        }
    }
    while out.ends_with('-') {
        out.pop();
    }
    if out.len() > 40 {
        out.truncate(40);
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
