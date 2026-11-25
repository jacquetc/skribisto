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

use std::collections::{BTreeMap, BTreeSet};

use crate::bundle::{CarriedFile, ProjectManifest, ResourceFile};
use crate::history::{HISTORY_DIR, HISTORY_INDEX};
use crate::shape::MANIFEST_NAME;
use crate::slug::{ASSETS_DIR, RESOURCES_DIR, TEMPLATES_DIR};

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
    "resources.ron",
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
        // resources/<uid>/<name> — one stored resource's file, any name.
        [dir, uid, _name] if *dir == RESOURCES_DIR => crate::media::resource_dir_uid(uid).is_some(),
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

/// The UID in a path occupying a stored file's position in the bundle.
pub(crate) fn resource_owner(rel: &str) -> Option<uuid::Uuid> {
    let rest = rel.strip_prefix("resources/")?;
    let (dir, file) = rest.split_once('/')?;
    if file.contains('/') {
        return None;
    }
    crate::media::resource_dir_uid(dir)
}

pub(crate) fn modelled_in<'a>(
    rel: &str,
    manifest: impl FnOnce() -> Option<&'a ProjectManifest>,
    live: &BTreeSet<uuid::Uuid>,
) -> bool {
    if let Some(uid) = resource_owner(rel) {
        return live.contains(&uid)
            || manifest()
                .is_some_and(|m| m.format_version >= 17 && !m.legacy_resource_dirs.contains(&uid));
    }
    is_modelled(rel)
}

pub(crate) fn stored_uids(resources: &[ResourceFile]) -> BTreeSet<uuid::Uuid> {
    resources
        .iter()
        .filter(|r| r.kind == common::entities::ResourceKind::StoredFile)
        .map(|r| r.uid)
        .collect()
}

/// A bundle that ended up **inside** this one — neither modelled nor carried.
///
/// A `.skrib` at the bundle root is a backup an older default wrote "next to
/// the project" while the project was addressed by its `project.skrib`, so that
/// "next to" resolved to the bundle itself. Carrying it would copy every such
/// backup into every later save and every later backup — each of which would
/// then hold all the earlier ones — and modelling it would prune it. It stays
/// on disk, untouched, where the writer can still find it. Only the root is
/// special: deeper down, an unknown file is exactly what carrying exists for.
pub fn is_stray_bundle(rel: &str) -> bool {
    !rel.contains('/') && rel != MANIFEST_NAME && rel.ends_with(".skrib")
}

/// Whether the bundle-relative `rel` is, or lies under, a hidden name: one starting
/// with a dot.
///
/// The format writes no such name, and none is the writer's: a `.git` folder kept
/// beside the book, a `.DS_Store` or `._` file a Mac leaves, a sync client's own
/// files, the `.tmp…` and `.blob…` temporary files a write cut short by a crash leaves
/// beside its target (`tempfile`'s names, which every atomic write here uses). Carried,
/// a repository would be read on every save, copied into every backup and archive, and
/// written back, stale, over the one beside the book; a crash's leftover would travel
/// with the project for good. So none is carried. Nor is one deleted for not being
/// carried: it stays on disk as it is, unless it sits in a folder the format fills and
/// prunes itself (`assets/`, a binder's `text/`), where it goes like any stray file
/// there, which is how a crash's leftover in such a folder is cleaned up.
pub fn is_hidden(rel: &str) -> bool {
    rel.split('/').any(|part| part.starts_with('.'))
}

/// Whether a walk of a project folder enters `entry`: not a hidden one (see
/// [`is_hidden`]), so a `.git` folder is not even listed. The project folder itself is
/// always entered, whatever its name.
pub(crate) fn walk_enters(entry: &walkdir::DirEntry) -> bool {
    entry.depth() == 0 || !entry.file_name().as_encoded_bytes().starts_with(b".")
}

/// The binder folder `rel` lies in and its path inside that folder, for a path under
/// `binders/<folder>/`: `binders/02-notes/text/x.ron` gives `("02-notes", "text/x.ron")`.
pub(crate) fn binder_folder(rel: &str) -> Option<(&str, &str)> {
    let (folder, rest) = rel.strip_prefix("binders/")?.split_once('/')?;
    (!folder.is_empty() && !rest.is_empty()).then_some((folder, rest))
}

/// The binder `uid` an `items.ron` names, or `None` when it does not parse or names
/// none (a project written before binders had one).
pub(crate) fn binder_uid_of(items_ron: &[u8]) -> Option<uuid::Uuid> {
    let text = std::str::from_utf8(items_ron).ok()?;
    let file: crate::bundle::ItemsFile = ron::from_str(text).ok()?;
    Some(file.binder.uid).filter(|uid| !uid.is_nil())
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
    load_for_save(path, &[])
}

/// Known live resources include files newly added to a pre-v17 folder before its
/// first save. They are streamed by the writer, never buffered as legacy files.
pub fn load_for_save(path: &str, resources: &[ResourceFile]) -> BTreeMap<String, CarriedFile> {
    // Most projects need no extra manifest read to classify their files.
    let manifest = std::cell::LazyCell::new(|| crate::peek_manifest(path).ok());
    let live = stored_uids(resources);
    let modelled = |rel: &str| modelled_in(rel, || manifest.as_ref(), &live);
    use crate::shape::{SkribShape, detect_shape, folder_root};
    match detect_shape(path) {
        Ok(SkribShape::ExplodedFolder) => load_folder(&folder_root(path), modelled),
        Ok(SkribShape::ZipFile) => load_zip(std::path::Path::new(path), modelled),
        _ => BTreeMap::new(),
    }
}

fn load_folder(
    root: &std::path::Path,
    modelled: impl Fn(&str) -> bool,
) -> BTreeMap<String, CarriedFile> {
    let mut out = BTreeMap::new();
    // The binder each binder folder holds, read from its `items.ron` when a carried
    // file is first found in it (see `CarriedFile::binder`).
    let mut binders: BTreeMap<String, Option<uuid::Uuid>> = BTreeMap::new();
    let walk = walkdir::WalkDir::new(root)
        .sort_by_file_name()
        .into_iter()
        .filter_entry(walk_enters);
    for entry in walk {
        let Ok(entry) = entry else { continue };
        // A link is not a file to a walk that does not follow links (`WalkDir`'s
        // default), and a linked folder is not descended: nothing reached through one
        // is carried, as `folder_io::read_folder_kept` carries none.
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
        if modelled(&rel) || is_stray_bundle(&rel) {
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
        let Ok(bytes) = std::fs::read(entry.path()) else {
            continue;
        };
        let binder = binder_folder(&rel).and_then(|(folder, _)| {
            *binders.entry(folder.to_string()).or_insert_with(|| {
                let items = root.join("binders").join(folder).join("items.ron");
                if crate::media::passes_through_link(root, &items) {
                    return None;
                }
                std::fs::read(&items).ok().and_then(|b| binder_uid_of(&b))
            })
        });
        out.insert(rel, CarriedFile::new(bytes).in_binder(binder));
    }
    out
}

fn load_zip(
    path: &std::path::Path,
    modelled: impl Fn(&str) -> bool,
) -> BTreeMap<String, CarriedFile> {
    let Ok(file) = std::fs::File::open(path) else {
        return BTreeMap::new();
    };
    let Ok((mut archive, mut guard)) =
        crate::zip_guard::ZipGuard::open(crate::zip_io::LIMITS, file)
    else {
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
    // it. A name zip cannot decode is in it too: it cannot be read back by name.
    let unmodelled: Vec<String> = archive
        .file_names()
        .filter_map(Result::ok)
        .filter(|n| !n.ends_with('/') && !modelled(n) && !is_stray_bundle(n) && !is_hidden(n))
        .filter(|n| match crate::safe_path::bundle_relative(n) {
            Ok(_) => true,
            Err(e) => {
                eprintln!("skrib: not carrying {e}");
                false
            }
        })
        .map(std::borrow::Cow::into_owned)
        .collect();

    let mut out = BTreeMap::new();
    let mut binders: BTreeMap<String, Option<uuid::Uuid>> = BTreeMap::new();
    for name in unmodelled {
        // Bounded by the shared guard: a carried file this build does not model is
        // still a zip member from an untrusted bundle, so it inflates under the same
        // ratio and ceiling as any other. One past its budget is dropped along with
        // the archive's running total, exactly as it would be on extraction.
        match guard.read_named(&mut archive, &name) {
            Ok(Some(bytes)) => {
                let folder = binder_folder(&name).map(|(folder, _)| folder.to_string());
                let binder = match folder {
                    Some(folder) => match binders.get(&folder) {
                        Some(uid) => *uid,
                        None => {
                            let items = format!("binders/{folder}/items.ron");
                            let uid = match guard.read_named(&mut archive, &items) {
                                Ok(Some(b)) => binder_uid_of(&b),
                                Ok(None) => None,
                                Err(_) => break,
                            };
                            binders.insert(folder, uid);
                            uid
                        }
                    },
                    None => None,
                };
                out.insert(name, CarriedFile::new(bytes).in_binder(binder));
            }
            Ok(None) => {}
            // A member past the budget: stop carrying. Carry is best-effort by
            // design (a missing carry set is a normal state), and a bundle whose
            // unmodelled members are a bomb is one this build will refuse to open
            // through `read_zip` anyway.
            Err(_) => break,
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
