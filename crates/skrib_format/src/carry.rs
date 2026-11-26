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

use anyhow::Context;

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
/// carried: it stays on disk as it is. The exceptions are a stray file in `assets/`,
/// which that folder's prune removes whatever its name, and a crash's leftover itself,
/// which a folder save removes from every folder it writes once it is old enough that
/// no write can still be using it (`folder_io::sweep_temp_leftovers`). The other prunes
/// match by extension, and a temporary name has none.
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
/// A missing or legacy bundle yields an empty map: there is nothing to carry, which is
/// an ordinary state for a brand-new project, not a failure. So does a file that
/// vanishes while it is read, as a sync client may remove one.
///
/// Any other failure to read is an error naming the file (a permission refused, a file
/// a sync client or an antivirus holds locked). A file left out of the map is a file
/// the write deletes: the prunes keep only what the map holds, and a binder's renamed
/// folder takes along only what the map lists. So a save that cannot read a carried
/// file fails, visibly, and deletes nothing; the project stays unsaved and the next
/// save tries again. The open path refuses such a file the same way.
pub fn load(path: &str) -> anyhow::Result<BTreeMap<String, CarriedFile>> {
    load_for_save(path, &[])
}

/// [`load`], knowing the live resources. Known live resources include files newly added
/// to a pre-v17 folder before its first save. They are streamed by the writer, never
/// buffered as legacy files.
pub fn load_for_save(
    path: &str,
    resources: &[ResourceFile],
) -> anyhow::Result<BTreeMap<String, CarriedFile>> {
    // Most projects need no extra manifest read to classify their files.
    let manifest = std::cell::LazyCell::new(|| crate::peek_manifest(path).ok());
    let live = stored_uids(resources);
    let modelled = |rel: &str| modelled_in(rel, || manifest.as_ref(), &live);
    use crate::shape::{SkribShape, detect_shape, folder_root};
    match detect_shape(path) {
        Ok(SkribShape::ExplodedFolder) => load_folder(&folder_root(path), modelled),
        Ok(SkribShape::ZipFile) => load_zip(std::path::Path::new(path), modelled),
        Ok(SkribShape::LegacySqlite) => Ok(BTreeMap::new()),
        // Nothing there, a folder with no manifest, a file of another kind: nothing to
        // carry. A file that is there and cannot be read is another matter: written
        // over, the files it carries would be gone.
        Err(e) => match e.chain().find_map(|c| c.downcast_ref::<std::io::Error>()) {
            Some(io) if io.kind() != std::io::ErrorKind::NotFound => {
                Err(e.context(format!("reading {path} for the files it carries")))
            }
            _ => Ok(BTreeMap::new()),
        },
    }
}

/// The error a save stops on for the I/O failure `e` met reading `what`, or `None` when
/// `e` says the file is simply not there (any more): see [`load`].
fn unless_gone(e: std::io::Error, what: &std::path::Path) -> Option<anyhow::Error> {
    (e.kind() != std::io::ErrorKind::NotFound).then(|| {
        anyhow::Error::new(e).context(format!(
            "reading {}, a file the project carries (nothing was saved, so nothing is lost)",
            what.display()
        ))
    })
}

/// The binder the folder `binders/<folder>/` of the project at `root` holds, read from
/// its `items.ron`: `None` when there is none, when it names none or does not parse, or
/// when it is reached through a link; an error when it is there and cannot be read.
/// Read as `None`, a renamed binder's carried files would stay in its old folder and be
/// pruned with it.
fn folder_binder(root: &std::path::Path, folder: &str) -> anyhow::Result<Option<uuid::Uuid>> {
    let items = root.join("binders").join(folder).join("items.ron");
    if crate::media::passes_through_link(root, &items) {
        return Ok(None);
    }
    match std::fs::read(&items) {
        Ok(bytes) => Ok(binder_uid_of(&bytes)),
        Err(e) => match unless_gone(e, &items) {
            Some(err) => Err(err),
            None => Ok(None),
        },
    }
}

fn load_folder(
    root: &std::path::Path,
    modelled: impl Fn(&str) -> bool,
) -> anyhow::Result<BTreeMap<String, CarriedFile>> {
    let mut out = BTreeMap::new();
    // The binder each binder folder holds, read from its `items.ron` when a carried
    // file is first found in it (see `CarriedFile::binder`).
    let mut binders: BTreeMap<String, Option<uuid::Uuid>> = BTreeMap::new();
    let walk = walkdir::WalkDir::new(root)
        .sort_by_file_name()
        .into_iter()
        .filter_entry(walk_enters);
    for entry in walk {
        // A folder that cannot be listed may hold carried files: skipped, they would be
        // pruned or left behind by a renamed binder, so the save stops instead.
        let entry = match entry {
            Ok(entry) => entry,
            Err(e) => {
                let at = e.path().unwrap_or(root).to_path_buf();
                match e.into_io_error() {
                    Some(io) => match unless_gone(io, &at) {
                        Some(err) => return Err(err),
                        None => continue,
                    },
                    None => anyhow::bail!("walking {}: it loops back on itself", at.display()),
                }
            }
        };
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
        let bytes = match std::fs::read(entry.path()) {
            Ok(bytes) => bytes,
            Err(e) => match unless_gone(e, entry.path()) {
                Some(err) => return Err(err),
                None => continue,
            },
        };
        let binder = match binder_folder(&rel) {
            Some((folder, _)) => match binders.get(folder) {
                Some(uid) => *uid,
                None => {
                    let uid = folder_binder(root, folder)?;
                    binders.insert(folder.to_string(), uid);
                    uid
                }
            },
            None => None,
        };
        out.insert(rel, CarriedFile::new(bytes).in_binder(binder));
    }
    Ok(out)
}

/// The zip side of [`load`]. The archive is replaced whole by the save, so a member
/// left out here is gone from the project: an archive that cannot be opened or read
/// stops the save, as it would refuse the open. Only a member past the size budget is
/// dropped, see below.
fn load_zip(
    path: &std::path::Path,
    modelled: impl Fn(&str) -> bool,
) -> anyhow::Result<BTreeMap<String, CarriedFile>> {
    let file = match std::fs::File::open(path) {
        Ok(file) => file,
        Err(e) => {
            return match unless_gone(e, path) {
                Some(err) => Err(err),
                None => Ok(BTreeMap::new()),
            };
        }
    };
    let (mut archive, mut guard) = crate::zip_guard::ZipGuard::open(crate::zip_io::LIMITS, file)
        .with_context(|| {
            format!(
                "reading {} for the files it carries (nothing was saved, so nothing is lost)",
                path.display()
            )
        })?;
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
    // Bounded by the shared guard: a carried file this build does not model is still a
    // zip member from an untrusted bundle, so it inflates under the same ratio and
    // ceiling as any other. A member past its budget stops the carrying, along with the
    // archive's running total, exactly as it would stop an extraction: a bundle whose
    // unmodelled members are a bomb is one `read_zip` refuses to open anyway, and its
    // bytes are no writer's. Any other failure (a damaged member, a read error) stops
    // the save, as it refuses the open.
    let mut read = |archive: &mut zip::ZipArchive<std::fs::File>,
                    name: &str|
     -> anyhow::Result<Option<Option<Vec<u8>>>> {
        match guard.read_named(archive, name) {
            Ok(bytes) => Ok(Some(bytes)),
            Err(e) if e.is::<crate::zip_guard::ZipRefused>() => Ok(None),
            Err(e) => Err(e.context(format!(
                "reading {} for the files it carries (nothing was saved, so nothing is lost)",
                path.display()
            ))),
        }
    };
    'members: for name in unmodelled {
        let bytes = match read(&mut archive, &name)? {
            Some(Some(bytes)) => bytes,
            Some(None) => continue,
            None => break,
        };
        let folder = binder_folder(&name).map(|(folder, _)| folder.to_string());
        let binder = match folder {
            Some(folder) => match binders.get(&folder) {
                Some(uid) => *uid,
                None => {
                    let items = format!("binders/{folder}/items.ron");
                    let uid = match read(&mut archive, &items)? {
                        Some(Some(b)) => binder_uid_of(&b),
                        Some(None) => None,
                        None => break 'members,
                    };
                    binders.insert(folder, uid);
                    uid
                }
            },
            None => None,
        };
        out.insert(name, CarriedFile::new(bytes).in_binder(binder));
    }
    Ok(out)
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
