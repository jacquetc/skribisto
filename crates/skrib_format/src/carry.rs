// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Carry-through: preserving bundle files this build does not model.
//!
//! A `.skrib` write rebuilds the whole artifact from a [`WorkBundle`] — the zip
//! is repacked from a fresh staging directory, and the exploded folder is pruned
//! down to what the bundle lists. Anything on disk that the bundle does not
//! model is therefore **destroyed by the next save**, silently and with no
//! error.
//!
//! That is fine while one build writes every file. It stops being fine the
//! moment two builds share a format: a project written by a newer version — or
//! by an edition with features this one lacks — loses those features' data the
//! first time it is opened and saved here. The writer sees nothing; they simply
//! find their work gone later.
//!
//! So every unmodelled file is read as opaque bytes and written back untouched.
//! No parsing, no interpretation, no schema: preserving data whose *meaning*
//! this build cannot know is only safe if it never tries to know it.
//!
//! ## What counts as modelled
//!
//! `is_modelled` is a **structural** predicate over the bundle-relative path,
//! deliberately not "whatever the reader happened to consume". The difference
//! matters for garbage: an orphaned prose blob — modelled in shape, referenced
//! by nothing — must stay prunable, and a "did the reader read it?" rule would
//! reclassify every one of them as unmodelled and preserve them forever,
//! quietly disabling the format's whole GC story.
//!
//! The predicate is the one place the bundle layout is written down twice (the
//! other being `folder_io::write_folder`, which produces it). Adding a file kind
//! to the format means adding it here, and the test at the bottom of this module
//! walks a freshly written bundle to assert the two agree.

use std::collections::BTreeMap;

use crate::bundle::CarriedFile;
use crate::history::{HISTORY_DIR, HISTORY_INDEX};
use crate::shape::MANIFEST_NAME;
use crate::slug::{ASSETS_DIR, TEMPLATES_DIR};

/// The `.ron` manifests that live directly at the bundle root.
const ROOT_MANIFESTS: &[&str] = &[
    "tags.ron",
    "dictionary.ron",
    "replacements.ron",
    "trash.ron",
    "paces.ron",
    "snapshots.ron",
    "orphan_comments.ron",
    "orphan_footnotes.ron",
    "templates.ron",
    "assets.ron",
    "statuses.ron",
];

/// Does `rel` — a bundle-root-relative, `/`-separated path — name a file this
/// format writes itself?
///
/// `false` means "carry it through untouched". Being wrong in that direction
/// costs a preserved file nobody reads; being wrong in the other direction
/// deletes a writer's data, which is why anything unrecognised is carried.
/// `pub` because it is also the guard on the save hook: a bundle contributor
/// asking to write a modelled path is refused, so an extension can add to a
/// project but never rewrite the manuscript inside it.
pub fn is_modelled(rel: &str) -> bool {
    if rel == MANIFEST_NAME || ROOT_MANIFESTS.contains(&rel) {
        return true;
    }
    let segments: Vec<&str> = rel.split('/').collect();
    match segments.as_slice() {
        // templates/<slug>.djot — one note-template body each.
        [dir, name] if *dir == TEMPLATES_DIR => name.ends_with(".djot"),
        // assets/<hash>.<ext> — content-addressed image blobs, any extension.
        [dir, _name] if *dir == ASSETS_DIR => true,
        // history/index.ron plus one content-addressed blob per recorded state.
        [dir, name] if *dir == HISTORY_DIR => rel == HISTORY_INDEX || name.ends_with(".djot"),
        // binders/<dir>/items.ron
        ["binders", _bdir, "items.ron"] => true,
        // binders/<dir>/text/<name>.djot and its two sidecar kinds.
        ["binders", _bdir, "text", name] => {
            name.ends_with(".djot")
                || name.ends_with(".comments.ron")
                || name.ends_with(".footnotes.ron")
        }
        _ => false,
    }
}

/// Read the unmodelled files of the bundle at `path`, whatever its shape.
///
/// Mirrors [`crate::history::load`], and exists for the same reason: this is the
/// other part of a bundle that does not come from the store, so
/// [`crate::from_entities`] cannot produce it and every write path has to fetch
/// it from the bundle it is derived from. Without this, `save_work` would build
/// a bundle with an empty carry set and the write would delete exactly the files
/// carrying is meant to protect.
///
/// Only unmodelled entries are decompressed — the archive's directory is
/// consulted for names first — so the common case (a project with nothing
/// unmodelled in it) costs one central-directory read and no inflation.
///
/// A missing, unreadable or legacy bundle yields an empty map rather than an
/// error: there is nothing to carry, which is an ordinary state for a brand-new
/// project, not a failure.
pub fn load(path: &str) -> BTreeMap<String, CarriedFile> {
    use crate::shape::{SkribShape, detect_shape, folder_root};
    match detect_shape(path) {
        Ok(SkribShape::ExplodedFolder) => load_folder(&folder_root(path)),
        Ok(SkribShape::ZipFile) => load_zip(std::path::Path::new(path)),
        _ => BTreeMap::new(),
    }
}

fn load_folder(root: &std::path::Path) -> BTreeMap<String, CarriedFile> {
    let mut out = BTreeMap::new();
    for entry in walkdir::WalkDir::new(root).sort_by_file_name() {
        let Ok(entry) = entry else { continue };
        if !entry.file_type().is_file() {
            continue;
        }
        let Some(rel) = entry
            .path()
            .strip_prefix(root)
            .ok()
            .and_then(|r| r.to_str())
            .map(|r| r.replace('\\', "/"))
        else {
            continue;
        };
        if is_modelled(&rel) {
            continue;
        }
        // The same filter the zip side applies, for the same reason and with the
        // same consequence: `write_folder` checks every carried path before
        // writing it, so a name carried here that the writer would refuse turns
        // every later save of this project into a failure — over a file nothing
        // reads. A directory walk cannot produce an escaping path, so in
        // practice this only ever catches a control character in a filename.
        if let Err(e) = crate::safe_path::bundle_relative(&rel) {
            eprintln!("skrib: not carrying {e}");
            continue;
        }
        if let Ok(bytes) = std::fs::read(entry.path()) {
            out.insert(rel, CarriedFile::new(bytes));
        }
    }
    out
}

fn load_zip(path: &std::path::Path) -> BTreeMap<String, CarriedFile> {
    use std::io::Read;
    let Ok(file) = std::fs::File::open(path) else {
        return BTreeMap::new();
    };
    let Ok(mut archive) = zip::ZipArchive::new(file) else {
        return BTreeMap::new();
    };
    // Names first, from the central directory, so the decision of what to
    // inflate is made without inflating anything.
    // The names are taken verbatim — that is the point of carrying — so they are
    // filtered here rather than trusted. An entry that would not survive
    // `bundle_relative` is not carried at all: it cannot be written back out
    // (`write_folder` checks the same predicate), so keeping it would only turn
    // a refusal at read time into a failed save later, on a file the writer
    // never asked for. Silently dropping is right for exactly this set, because
    // by construction this build does not model it and nothing downstream reads
    // it.
    let unmodelled: Vec<String> = archive
        .file_names()
        .filter(|n| !n.ends_with('/') && !is_modelled(n))
        .filter(|n| match crate::safe_path::bundle_relative(n) {
            Ok(_) => true,
            Err(e) => {
                eprintln!("skrib: not carrying {e}");
                false
            }
        })
        .map(str::to_string)
        .collect();

    let mut out = BTreeMap::new();
    for name in unmodelled {
        let Ok(mut entry) = archive.by_name(&name) else {
            continue;
        };
        let mut bytes = Vec::new();
        if entry.read_to_end(&mut bytes).is_ok() {
            out.insert(name, CarriedFile::new(bytes));
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_file_a_write_produces_is_modelled() {
        // The anti-drift check. `is_modelled` restates the layout that
        // `write_folder` produces, so the two can disagree — and a disagreement
        // in this direction is the expensive one: a modelled file classed as
        // unmodelled gets carried, which means it is never pruned and never
        // garbage-collected again.
        //
        // Rather than trust the list, write a real bundle with one of
        // everything and assert the predicate accepts every file in it.
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        let bundle = crate::tests::build_bundle(crate::bundle::ShapeTag::Folder);
        crate::folder_io::write_folder(root, &bundle).unwrap();

        let mut unmodelled = Vec::new();
        for entry in walkdir::WalkDir::new(root) {
            let entry = entry.unwrap();
            if !entry.file_type().is_file() {
                continue;
            }
            let rel = entry
                .path()
                .strip_prefix(root)
                .unwrap()
                .to_str()
                .unwrap()
                .replace('\\', "/");
            if !is_modelled(&rel) {
                unmodelled.push(rel);
            }
        }
        assert!(
            unmodelled.is_empty(),
            "write_folder produced files `is_modelled` does not recognise, so carrying \
             would preserve them forever and the prunes would never collect them: {unmodelled:?}"
        );
    }

    #[test]
    fn unknown_paths_are_not_modelled() {
        // The shapes a future build (or another edition) would plausibly add.
        for rel in [
            "structure.ron",
            "structure/beats.ron",
            "binders/0-main/structure.ron",
            "binders/0-main/text/12-scene.beats.ron",
            "templates/readme.txt",
            "history/notes.txt",
        ] {
            assert!(!is_modelled(rel), "`{rel}` must be carried, not modelled");
        }
    }

    #[test]
    fn modelled_shapes_are_recognised() {
        for rel in [
            "project.skrib",
            "tags.ron",
            "assets.ron",
            "statuses.ron",
            "templates/ab12-character.djot",
            "assets/deadbeef.png",
            "history/index.ron",
            "history/cafe1234.djot",
            "binders/0-main/items.ron",
            "binders/0-main/text/12-the-lamp.djot",
            "binders/0-main/text/12-the-lamp.comments.ron",
            "binders/0-main/text/12-the-lamp.footnotes.ron",
        ] {
            assert!(is_modelled(rel), "`{rel}` is written by this format");
        }
    }
}
