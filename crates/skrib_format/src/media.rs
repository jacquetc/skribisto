// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Where a project's image bytes live while it is open.
//!
//! Assets are metadata-only rows in the store (see the `Asset` entity); their
//! bytes are files. This module decides *which* files, and it has to answer
//! differently for the two shapes a `.skrib` takes:
//!
//! * **Exploded folder** — the project already *is* a directory, so `assets/`
//!   inside it is the media directory. Nothing is copied on save, the bytes are
//!   already where they belong, and `git` sees them next to the prose that
//!   references them.
//! * **Zip** — a project file cannot be written into piecemeal, so the bytes are
//!   extracted to a working directory under the app's data root on load and read
//!   back from there when the archive is rewritten.
//!
//! The zip case is a *working directory*, not a cache: bytes land there the
//! moment an image is inserted, before any save, which is what makes an
//! unsaved image survive a crash. It is keyed by the Work's `unique_id` so two
//! projects open at once never share one.

use std::path::{Path, PathBuf};

/// Directory name for assets inside an exploded-folder project.
///
/// Re-exported from [`crate::slug`], which owns the bundle's directory names,
/// rather than declared again here: this module builds the `assets/…` paths the
/// prose and the UI parse, while `folder_io` writes and prunes the directory
/// itself. Two independent constants that happened to agree would let a rename
/// of one silently break the correspondence, with nothing to catch it.
pub use crate::slug::ASSETS_DIR;

/// Resolve the media directory for a project.
///
/// `project_path` is the `.skrib` file or folder; `unique_id` is the Work's
/// durable id; `data_root` is the app's per-user data directory.
///
/// A project with no usable `unique_id` — one that has never been saved — gets a
/// directory keyed by `fallback_key` instead. The caller supplies that (a
/// session-scoped id held on the Work's session) rather than this function
/// inventing one, because the same project must resolve to the same directory
/// for as long as it is open, and a value minted here could not do that.
pub fn media_dir(
    project_path: &Path,
    unique_id: &str,
    data_root: &Path,
    fallback_key: &str,
) -> PathBuf {
    if is_folder_project(project_path) {
        return project_path.join(ASSETS_DIR);
    }
    let key = if unique_id.trim().is_empty() {
        fallback_key
    } else {
        unique_id
    };
    data_root.join("media").join(sanitize_key(key))
}

/// Whether `path` is an exploded-folder project rather than a zip.
///
/// Decided by what is on disk, not by the extension: both shapes use `.skrib`,
/// which is the whole point of the format's "one name, two layouts" design.
/// A path that does not exist yet is treated as a zip, because that is what
/// `new_work` produces unless the writer asked otherwise.
pub fn is_folder_project(path: &Path) -> bool {
    path.is_dir()
}

/// Reduce a key to something safe to use as a single directory name.
///
/// A `unique_id` is a UUID in every project this ships with, but it is a plain
/// string in the schema, and a project file is user-supplied data — so a value
/// containing `/` or `..` must not be able to steer the media directory
/// somewhere else.
fn sanitize_key(key: &str) -> String {
    let cleaned: String = key
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || c == '-' || c == '_' {
                c
            } else {
                '_'
            }
        })
        .collect();
    // An all-illegal key would collapse to an empty name and silently place the
    // directory at `media/` itself, shared by every such project.
    if cleaned.is_empty() {
        "unnamed".to_string()
    } else {
        cleaned
    }
}

/// Relative path of an asset inside a bundle, from its content hash and type.
///
/// The hash *is* the name: prose references `assets/<hash>.<ext>`, so
/// re-inserting the same picture costs nothing and an unchanged asset can be
/// recognised without reading its bytes.
pub fn asset_relpath(content_hash: &str, extension: &str) -> String {
    let ext = if extension.is_empty() {
        "bin"
    } else {
        extension
    };
    format!("{ASSETS_DIR}/{}.{ext}", sanitize_key(content_hash))
}

/// Escape a string for use as an image's alt text in Djot.
///
/// The alt runs inside `![…]`, so `]` would close it early and turn the rest of
/// the text into stray markup — a book called *The Lighthouse \[Revised\]* is not
/// exotic. `\` is escaped too, or it would escape whatever follows it into the
/// document. `[` needs nothing: only `]` can end the span.
///
/// Escaped, never *stripped*: a title is the writer's, and silently deleting
/// characters from it to make it safe to embed loses the text while looking like
/// it worked. Lives here rather than in either caller because both the editor
/// (writing prose) and the compiler (writing a cover) must agree — two
/// same-named copies had already drifted into escaping and deleting.
pub fn escape_djot_alt(alt: &str) -> String {
    let mut out = String::with_capacity(alt.len());
    for c in alt.chars() {
        if c == ']' || c == '\\' {
            out.push('\\');
        }
        out.push(c);
    }
    out
}

/// Every `assets/…` path an image in this Djot references, in first-seen order
/// and without repeats.
///
/// A deliberately narrow scan rather than a parse. It runs on every document
/// open and on every export, and the only thing either needs is the set of
/// names to resolve — parsing the prose to learn them would cost the whole
/// document for a list that is almost always empty.
///
/// The `assets/` prefix is the filter: an ordinary Markdown link, an `http://`
/// image, or a path the writer typed pointing outside the project is not
/// something this project stores bytes for, so none of them are returned.
pub fn referenced_paths(djot: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut rest = djot;
    let prefix = format!("{ASSETS_DIR}/");
    while let Some(open) = rest.find("![") {
        rest = &rest[open + 2..];
        let Some(close) = rest.find("](") else { break };
        let after = &rest[close + 2..];
        let Some(end) = after.find(')') else { break };
        let path = &after[..end];
        if path.starts_with(&prefix) && !out.iter().any(|p| p == path) {
            out.push(path.to_string());
        }
        rest = &after[end..];
    }
    out
}

/// Filename extension for a media type, used when naming an asset's blob.
///
/// Falls back to `bin` rather than guessing from the original filename: a
/// wrong extension inside the bundle would mislabel the bytes for every reader
/// that dispatches on it, and a visible `bin` is easier to diagnose than a
/// `.png` that is not a PNG.
pub fn extension_for(mime_type: &str) -> String {
    match mime_type {
        "image/png" => "png",
        "image/jpeg" => "jpg",
        "image/webp" => "webp",
        "image/gif" => "gif",
        "image/svg+xml" => "svg",
        _ => "bin",
    }
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_folder_project_keeps_its_assets_beside_its_prose() {
        let dir = tempfile::tempdir().unwrap();
        let project = dir.path().join("Novel.skrib");
        std::fs::create_dir(&project).unwrap();
        let got = media_dir(&project, "uid-1", Path::new("/data"), "session");
        assert_eq!(got, project.join("assets"));
    }

    #[test]
    fn a_zip_project_uses_a_working_directory_keyed_by_its_uid() {
        let dir = tempfile::tempdir().unwrap();
        let project = dir.path().join("Novel.skrib");
        std::fs::write(&project, b"zip").unwrap();
        let got = media_dir(&project, "uid-1", Path::new("/data"), "session");
        assert_eq!(got, Path::new("/data/media/uid-1"));
    }

    #[test]
    fn a_never_saved_project_falls_back_to_the_session_key() {
        // No file on disk and no uid yet — the shape `new_work` starts from.
        let got = media_dir(
            Path::new("/nowhere/Untitled.skrib"),
            "",
            Path::new("/data"),
            "session-abc",
        );
        assert_eq!(got, Path::new("/data/media/session-abc"));
    }

    #[test]
    fn a_hostile_uid_cannot_escape_the_media_root() {
        // `unique_id` is a plain string in the schema and arrives from a project
        // file, so traversal has to be impossible rather than unlikely.
        let got = media_dir(
            Path::new("/nowhere/P.skrib"),
            "../../etc",
            Path::new("/data"),
            "session",
        );
        assert_eq!(got, Path::new("/data/media/______etc"));
        assert!(got.starts_with("/data/media"));
    }

    #[test]
    fn an_empty_key_does_not_collapse_onto_the_shared_root() {
        // Both keys empty is the only way a name collapses to nothing; without
        // the guard the directory would be `media/` itself, shared by every
        // such project.
        let got = media_dir(Path::new("/nowhere/P.skrib"), "", Path::new("/data"), "");
        assert_eq!(got, Path::new("/data/media/unnamed"));
    }

    #[test]
    fn every_referenced_image_is_found_once() {
        let djot =
            "a ![one](assets/a.png) b ![two](assets/b.jpg){width=4} c ![again](assets/a.png)";
        assert_eq!(
            referenced_paths(djot),
            vec!["assets/a.png".to_string(), "assets/b.jpg".to_string()]
        );
    }

    #[test]
    fn a_link_is_not_an_image_and_an_outside_path_is_not_ours() {
        // `[text](assets/x.png)` is a link, not an image: no bytes to resolve.
        // An absolute or remote path names something this project does not store.
        let djot = "[a link](assets/x.png) ![web](https://example.com/y.png) ![abs](/etc/z.png)";
        assert!(referenced_paths(djot).is_empty());
    }

    #[test]
    fn malformed_markup_does_not_hang_or_panic() {
        // The scan advances on every branch; an unterminated image must end it.
        assert!(referenced_paths("![unclosed").is_empty());
        assert!(referenced_paths("![alt](assets/a.png").is_empty());
        assert_eq!(referenced_paths("![](assets/a.png)").len(), 1);
    }

    #[test]
    fn an_asset_is_named_by_its_content_hash() {
        assert_eq!(asset_relpath("abc123", "png"), "assets/abc123.png");
        assert_eq!(asset_relpath("abc123", ""), "assets/abc123.bin");
        // Four illegal characters in `a/../b` → four underscores.
        assert_eq!(asset_relpath("a/../b", "jpg"), "assets/a____b.jpg");
    }

    /// A title's brackets must survive into the alt text, escaped rather than
    /// deleted. Stripping them loses the writer's words while looking like it
    /// worked — and the two copies of this function that used to exist disagreed
    /// on exactly this.
    #[test]
    fn escape_djot_alt_escapes_rather_than_strips() {
        assert_eq!(
            escape_djot_alt("The Lighthouse [Revised]"),
            "The Lighthouse [Revised\\]"
        );
        assert_eq!(escape_djot_alt("back\\slash"), "back\\\\slash");
        // `[` cannot end the span, so it is left exactly as the writer typed it.
        assert_eq!(escape_djot_alt("a [ b"), "a [ b");
        assert_eq!(escape_djot_alt("plain title"), "plain title");
    }
}
