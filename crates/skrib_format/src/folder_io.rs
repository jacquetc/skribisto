// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Exploded-folder read/write. Writes are atomic (tmp + rename), manifest-last
//! (the `project.skrib` is the commit point), and **diff-minimal**: a blob is
//! only touched when its bytes actually change, and orphaned blobs/dirs are
//! pruned — so an exploded project under git shows a tight diff.
//!
//! Every path a manifest names is resolved through [`super::locate`], on both the
//! read and the write side, and every prune compares names by NFC form: a folder
//! that crossed a Unicode-normalisation boundary (a sync client, HFS+) still opens,
//! and a save into it edits the files as they are spelled rather than writing twins.
//!
//! # Symbolic links
//!
//! A folder project can arrive from anywhere (a shared drive, an archive that keeps
//! links, a synchronised folder), and a link inside it can point at any file on the
//! disk. Whatever the reader takes in travels on: into the zip a Save As writes, into a
//! backup, into the copy a writer sends. So no file of the project is read through a
//! link, the file itself or any folder between the project folder and it
//! ([`crate::media::passes_through_link`]). The project folder itself is not judged:
//! the writer may well keep their books behind a link of their own.
//!
//! What a linked file gets depends on what leaving it out would cost:
//!
//! * **Refused, the project does not open**: `project.skrib`, every work-level `.ron`
//!   list, a binder's `items.ron`, a prose file, a footnote list, a note template's
//!   body. Each holds the writer's own words or the structure that places them. Left
//!   out, the book would open with a scene empty or a list gone, look like data loss,
//!   and the next autosave would write that loss into the project within seconds. A
//!   refusal names the file and keeps everything on disk as it is.
//! * **Left out, and said on stderr**: a comment list, the history index and its
//!   blobs, a stored resource's file, a picture, a file this build does not model.
//!   Each is something the format already reads as possibly absent (no comments yet, a
//!   thinned history, a resource shown as missing, a dropped picture row, nothing to
//!   carry), so the book opens whole. The link itself is never followed and nothing
//!   behind it is touched: a later save replaces it or prunes the link alone.
//!
//! The writer never writes through a link either. A link met on the way to a file it
//! writes, the file or a folder above it, is removed and a real folder or file written
//! in its place, by the same temporary file and rename as every write here; whatever
//! the link pointed at is left exactly as it was. A prune never lists a folder reached
//! through a link, so it cannot delete files elsewhere on the disk.

use anyhow::{Context, Result};
use serde::Serialize;
use serde::de::DeserializeOwned;
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use tempfile::NamedTempFile;

use super::bundle::*;
use super::history::{HISTORY_DIR, HISTORY_INDEX, blob_relpath};
use super::locate::{Locator, nfc};
use super::migration::COMMENT_BODIES_ARE_DJOT_FROM;
use super::shape::MANIFEST_NAME;
use super::slug::{ASSETS_DIR, TEMPLATES_DIR, binder_dir_name};
use super::version_gate::compute_min_read_version;
use super::writer::{discard, persist_durably};

fn to_ron<T: Serialize>(value: &T) -> Result<String> {
    let cfg = ron::ser::PrettyConfig::new().struct_names(true);
    let mut s = ron::ser::to_string_pretty(value, cfg)?;
    s.push('\n');
    Ok(s)
}

fn from_ron<T: DeserializeOwned>(text: &str, what: &str) -> Result<T> {
    ron::from_str(text).with_context(|| format!("parsing {what}"))
}

/// Make the way from the project folder `root` down to `path` the project's own before
/// anything is written there: the first symbolic link on it, a folder or `path`
/// itself, is removed, so the write that follows creates a real folder or file in its
/// place instead of following the link out of the project (see the module docs).
/// Removing a link touches nothing it points at. A `path` not under `root` is left to
/// the caller, which never builds one.
fn own_way(root: &Path, path: &Path) -> Result<()> {
    let Ok(rel) = path.strip_prefix(root) else {
        return Ok(());
    };
    let mut at = root.to_path_buf();
    for component in rel.components() {
        at.push(component);
        match fs::symlink_metadata(&at) {
            Ok(meta) if meta.file_type().is_symlink() => {
                eprintln!(
                    "skrib: {} is a link; it is replaced by the project's own, and what it \
                     points at is left alone",
                    at.display()
                );
                // `remove_file` takes a link to a folder on Unix; Windows wants
                // `remove_dir` for one. Neither follows the link.
                fs::remove_file(&at)
                    .or_else(|_| fs::remove_dir(&at))
                    .with_context(|| format!("removing the link {}", at.display()))?;
                // Everything below it is gone from the project's view, to be created.
                return Ok(());
            }
            Ok(_) => {}
            Err(_) => return Ok(()),
        }
    }
    Ok(())
}

/// [`own_way`], then create the folder `dir` of the project.
fn own_dir(root: &Path, dir: &Path) -> Result<()> {
    own_way(root, dir)?;
    fs::create_dir_all(dir).with_context(|| format!("creating {}", dir.display()))
}

/// Write `bytes` to `path`, under the project folder `root`, only if the on-disk
/// content differs. Returns whether a write happened. Uses a same-dir temp file +
/// atomic rename, in a folder of the project's own (see [`own_way`]).
fn write_if_changed(root: &Path, path: &Path, bytes: &[u8]) -> Result<bool> {
    own_way(root, path)?;
    if let Ok(existing) = fs::read(path)
        && existing == bytes
    {
        return Ok(false);
    }
    let parent = path
        .parent()
        .ok_or_else(|| anyhow::anyhow!("no parent dir for {}", path.display()))?;
    fs::create_dir_all(parent).with_context(|| format!("creating {}", parent.display()))?;
    let mut tmp = NamedTempFile::new_in(parent)
        .with_context(|| format!("temp file in {}", parent.display()))?;
    if let Err(e) = tmp.write_all(bytes) {
        discard(tmp);
        return Err(e).with_context(|| format!("writing {}", path.display()));
    }
    persist_durably(tmp, path)?;
    Ok(true)
}

/// Bundle-root-relative path of `path`, `/`-separated, or `None` if `path` is
/// not under `root` (or is not UTF-8). The key shape [`WorkBundle::carried`]
/// uses, and the one the prunes below compare against.
fn rel_key(root: &Path, path: &Path) -> Option<String> {
    let rel = path.strip_prefix(root).ok()?;
    Some(rel.to_str()?.replace('\\', "/"))
}

/// The set of paths a write is carrying through untouched, in the form the
/// prunes need: "is this file on disk one I must not delete?".
///
/// The prunes exist to remove *stale modelled* files — a renamed template's old
/// blob, a replaced image, a thinned history blob. A carried file is by
/// definition none of those: this build did not write it and cannot judge it, so
/// deleting it would be exactly the silent destruction carrying exists to stop.
struct Carried<'a> {
    root: &'a Path,
    paths: &'a BTreeMap<String, &'a CarriedFile>,
}

/// Where each carried file goes in this write, keyed as [`WorkBundle::carried`] is.
///
/// A binder's folder is named by its position and its name (`02-notes`), so a binder
/// renamed or moved gets another folder, and [`prune_binder_dirs`] removes the one it
/// had. A file carried from a binder's folder knows which binder that was
/// ([`CarriedFile::binder`]) and goes into that binder's folder of today:
/// `binders/02-notes/structure.ron` becomes `binders/01-notes/structure.ron`. One whose
/// binder is no longer in the bundle, because the writer deleted it, is not written: it
/// goes with its binder. Every other file is written where it was found.
///
/// Where a file is already found at the place another would move to, it keeps that
/// place: it is either an extension's current copy, written for the binder's folder of
/// today, or the file of the binder that holds the folder now.
///
/// Also returns the places files moved away from that nothing is written to. Most are
/// in a folder the write prunes whole, but not all: two binders of the same name that
/// swap places swap folders, and a file left behind would be read next time as the
/// other binder's.
fn place_carried(bundle: &WorkBundle) -> (BTreeMap<String, &CarriedFile>, Vec<&str>) {
    let folders: BTreeMap<uuid::Uuid, String> = bundle
        .binders
        .iter()
        .enumerate()
        .filter(|(_, bb)| !bb.binder.uid.is_nil())
        .map(|(index, bb)| (bb.binder.uid, binder_dir_name(index, &bb.binder.name)))
        .collect();
    let mut placed = BTreeMap::new();
    let mut moved = Vec::new();
    let mut left = Vec::new();
    for (rel, file) in &bundle.carried {
        let (Some(uid), Some((_, rest))) = (file.binder, super::carry::binder_folder(rel)) else {
            placed.insert(rel.clone(), file);
            continue;
        };
        let Some(folder) = folders.get(&uid) else {
            left.push(rel.as_str());
            continue;
        };
        let to = format!("binders/{folder}/{rest}");
        if nfc(&to) == nfc(rel) {
            placed.insert(rel.clone(), file);
        } else {
            moved.push((to, file));
            left.push(rel.as_str());
        }
    }
    for (to, file) in moved {
        placed.entry(to).or_insert(file);
    }
    let placed_nfc: BTreeSet<String> = placed.keys().map(|k| nfc(k)).collect();
    left.retain(|rel| !placed_nfc.contains(&nfc(rel)));
    (placed, left)
}

impl Carried<'_> {
    fn holds(&self, path: &Path) -> bool {
        rel_key(self.root, path).is_some_and(|k| self.paths.contains_key(&k))
    }
}

pub fn write_folder(root: &Path, bundle: &WorkBundle) -> Result<()> {
    write_folder_placing(root, bundle, Placing::Keep)
}

/// [`write_folder`] into a folder that is thrown away once the write is done (a zip's
/// staging): pictures and stored files are hard-linked rather than copied where the
/// file system allows, which is what keeps a zip save of a large project affordable.
pub(crate) fn write_folder_staged(root: &Path, bundle: &WorkBundle) -> Result<()> {
    write_folder_placing(root, bundle, Placing::Throwaway)
}

/// Whether the folder written outlives the write. See [`crate::Blob::place_at`].
#[derive(Clone, Copy)]
enum Placing {
    Keep,
    Throwaway,
}

/// Put `blob` at `dest`, under the project folder `root`, never through a link (see
/// [`own_way`]): a link left at `dest` would otherwise be kept whenever what it points
/// at already holds the bytes.
fn place(root: &Path, blob: &crate::Blob, dest: &Path, placing: Placing) -> Result<bool> {
    own_way(root, dest)?;
    Ok(match placing {
        Placing::Keep => blob.place_at(dest)?,
        Placing::Throwaway => blob.link_or_place_at(dest)?,
    })
}

fn write_folder_placing(root: &Path, bundle: &WorkBundle, placing: Placing) -> Result<()> {
    fs::create_dir_all(root).with_context(|| format!("creating {}", root.display()))?;
    let binders_dir = root.join("binders");
    // A link here is replaced, not followed (see the module docs); a failure to is met
    // again, and reported, by the first write below it.
    own_dir(root, &binders_dir).ok();

    let (placed, left) = place_carried(bundle);
    let carried = Carried {
        root,
        paths: &placed,
    };
    let locator = Locator::new();

    // Carried files go down **first**, so that anything this build also models
    // overwrites them with its own authoritative bytes rather than the other way
    // round. `write_if_changed` keeps the exploded shape diff-minimal: a carried
    // file whose bytes did not change is not rewritten, so it stays out of the
    // git diff exactly like modelled content.
    for (rel, file) in &placed {
        // A carried path is a zip entry name this build did not recognise, so it
        // is the least trusted string in the whole bundle — `carry` keeps it
        // verbatim by design. `read_zip` already refuses an escaping entry, but
        // a `WorkBundle` can also be built by hand (both project importers do),
        // so the write side checks rather than assuming.
        let target = locator.locate(root, rel, "carried file")?;
        write_if_changed(root, &target, &file.bytes)
            .with_context(|| format!("writing carried file {rel}"))?;
    }

    // Work-level manifests.
    write_if_changed(
        root,
        &root.join("tags.ron"),
        to_ron(&bundle.tags)?.as_bytes(),
    )?;
    write_if_changed(
        root,
        &root.join("dictionary.ron"),
        to_ron(&bundle.dict_words)?.as_bytes(),
    )?;
    write_if_changed(
        root,
        &root.join("replacements.ron"),
        to_ron(&bundle.text_replacement_rules)?.as_bytes(),
    )?;
    write_if_changed(
        root,
        &root.join("trash.ron"),
        to_ron(&bundle.trash_infos)?.as_bytes(),
    )?;
    write_if_changed(
        root,
        &root.join("paces.ron"),
        to_ron(&bundle.paces)?.as_bytes(),
    )?;
    write_if_changed(
        root,
        &root.join("snapshots.ron"),
        to_ron(&bundle.progress_snapshots)?.as_bytes(),
    )?;
    // Only materialise the orphanage when it has something in it, so a project that
    // has never lost a comment's anchor carries no extra file at all. When it empties
    // again, remove the file rather than leaving an empty list behind.
    let orphans_path = root.join("orphan_comments.ron");
    if bundle.orphan_comments.is_empty() {
        fs::remove_file(&orphans_path).ok();
    } else {
        write_if_changed(
            root,
            &orphans_path,
            to_ron(&bundle.orphan_comments)?.as_bytes(),
        )?;
    }

    // The same for footnotes whose annotated content is gone. Kept, not dropped:
    // a comment's loss costs a remark, a footnote's costs words from the book.
    let orphan_notes_path = root.join("orphan_footnotes.ron");
    if bundle.orphan_footnotes.is_empty() {
        fs::remove_file(&orphan_notes_path).ok();
    } else {
        write_if_changed(
            root,
            &orphan_notes_path,
            to_ron(&bundle.orphan_footnotes)?.as_bytes(),
        )?;
    }

    // Note templates: an index plus one Djot blob each, mirroring the prose split.
    //
    // The prune is not optional bookkeeping. A rename changes the slug and therefore the
    // blob's filename, and a delete drops the row entirely — without this, every rename
    // and every delete would leave an orphan `.djot` behind for good, growing the
    // git-tracked tree and undoing the whole reason the bodies are separate files.
    //
    // Bodies first, then the index naming them, then the prune: a save cut short at any
    // point (a crash, a full disk) leaves an index whose every body is on disk, since a
    // new body is there before the index names it and an old one goes only once no
    // index does. The same order holds for every index and its files below.
    let templates_dir = root.join(TEMPLATES_DIR);
    let mut expected_templates: BTreeSet<String> = BTreeSet::new();
    if !bundle.note_templates.is_empty() {
        own_dir(root, &templates_dir)?;
    }
    for t in &bundle.note_templates {
        let rel = Path::new(&t.path);
        let fname = rel
            .file_name()
            .and_then(|n| n.to_str())
            .ok_or_else(|| anyhow::anyhow!("bad note-template path '{}'", t.path))?
            .to_string();
        let body = bundle
            .note_template_bodies
            .get(&t.file_id)
            .ok_or_else(|| anyhow::anyhow!("missing body blob for note template {}", t.file_id))?;
        write_if_changed(
            root,
            &locator.locate(root, &t.path, "note-template body")?,
            body.as_bytes(),
        )?;
        expected_templates.insert(nfc(&fname));
    }
    write_if_changed(
        root,
        &root.join("templates.ron"),
        to_ron(&bundle.note_templates)?.as_bytes(),
    )?;
    prune_dir(&templates_dir, &expected_templates, "djot", &carried)?;

    // Assets: an index plus one blob each, the same split templates use — but
    // placed as files, not text. `Blob::place_at` leaves a file that already holds
    // the bytes alone (a folder project saved in place reads its blobs from the
    // very files it would write), so an unchanged image is not rewritten and the
    // exploded shape stays diff-minimal even with photographs in it.
    //
    // The prune matters more here than anywhere else: assets are content-
    // addressed, so *replacing* an image writes a new name and leaves the old
    // blob behind. Without this a project would accumulate every version of
    // every picture a writer ever swapped out, forever.
    write_if_changed(
        root,
        &root.join("statuses.ron"),
        to_ron(&bundle.statuses)?.as_bytes(),
    )?;
    let assets_dir = root.join(ASSETS_DIR);
    let mut expected_assets: BTreeSet<String> = BTreeSet::new();
    if !bundle.assets.is_empty() {
        own_dir(root, &assets_dir)?;
    }
    for a in &bundle.assets {
        let rel = Path::new(&a.path);
        let fname = rel
            .file_name()
            .and_then(|n| n.to_str())
            .ok_or_else(|| anyhow::anyhow!("bad asset path '{}'", a.path))?
            .to_string();
        let blob = bundle.asset_blobs.get(&a.content_hash).ok_or_else(|| {
            anyhow::anyhow!(
                "missing bytes for asset {} ({})",
                a.file_name,
                a.content_hash
            )
        })?;
        let dest = locator.locate(root, &a.path, "asset")?;
        place(root, blob, &dest, placing)
            .with_context(|| format!("writing asset {} to {}", a.file_name, dest.display()))?;
        expected_assets.insert(nfc(&fname));
    }
    // After the pictures it names: the reader refuses a project whose index names a
    // picture that is not there.
    write_if_changed(
        root,
        &root.join("assets.ron"),
        to_ron(&bundle.assets)?.as_bytes(),
    )?;
    prune_assets_dir(&assets_dir, &expected_assets, &carried)?;

    // Resources: each stored file in its own `resources/<uid>/` folder under its own
    // name, then the index. Placed like assets (a folder project saved in place
    // writes nothing), and the path always rebuilt from `uid` and `file_name` here,
    // never taken from the row, so a crafted `path` cannot steer a write.
    //
    // No prune, unlike assets: a resource removed during a session keeps its file
    // until the project is next opened (`media::prune_resources_dir`), which is what
    // lets Undo bring it back with its bytes. A zip is staged afresh and so never
    // carries a removed one anyway. A stored resource with no blob is written as a
    // row only: shown as missing, not dropped.
    for r in &bundle.resources {
        if r.kind != common::entities::ResourceKind::StoredFile {
            continue;
        }
        let Some(blob) = bundle.resource_blobs.get(&r.uid.hyphenated().to_string()) else {
            continue;
        };
        let rel = crate::slug::resource_relpath(r.uid, &r.file_name);
        let dest = locator.locate(root, &rel, "resource")?;
        place(root, blob, &dest, placing)
            .with_context(|| format!("writing resource {} to {}", r.file_name, dest.display()))?;
    }
    write_if_changed(
        root,
        &root.join("resources.ron"),
        to_ron(&bundle.resources)?.as_bytes(),
    )?;

    // The history log: an index plus one content-addressed blob per recorded
    // state, the same split assets use. Written only when the log is non-empty, so
    // a project that predates the feature (or one whose log was thinned to nothing)
    // grows no directory at all.
    //
    // The prune is the blob GC: `history::thin` drops index entries, and the blobs
    // they referenced have to follow, or the directory only ever grows.
    let history_dir = root.join(HISTORY_DIR);
    if bundle.history.is_empty() {
        // Nothing to keep — remove any index left from a previous save so the
        // on-disk state matches the bundle rather than resurrecting a stale log.
        // Not through a linked `history/`: the index it would remove is elsewhere.
        if !crate::media::passes_through_link(root, &root.join(HISTORY_INDEX)) {
            let _ = fs::remove_file(root.join(HISTORY_INDEX));
        }
        prune_dir(&history_dir, &BTreeSet::new(), "djot", &carried)?;
    } else {
        own_dir(root, &history_dir)?;
        let mut expected_blobs: BTreeSet<String> = BTreeSet::new();
        for hash in bundle.history.referenced_hashes() {
            let text = bundle.history.blobs.get(&hash).ok_or_else(|| {
                anyhow::anyhow!("history index references blob {hash} with no bytes in the bundle")
            })?;
            write_if_changed(root, &root.join(blob_relpath(&hash)), text.as_bytes())?;
            expected_blobs.insert(format!("{hash}.djot"));
        }
        write_if_changed(
            root,
            &root.join(HISTORY_INDEX),
            to_ron(&bundle.history.entries)?.as_bytes(),
        )?;
        prune_dir(&history_dir, &expected_blobs, "djot", &carried)?;
    }

    // Binders, in three steps across all of them, so that a save cut short anywhere
    // leaves every item named by some binder's `items.ron`, with its prose on disk:
    //
    // 1. every binder's prose and sidecars, deleting nothing;
    // 2. every binder's `items.ron` ([`write_binder_lists`]);
    // 3. after the manifest, the prunes of every `text/` and of the binder folders.
    //
    // Binder by binder, an item moved from the first binder to the second was dropped
    // from the first `items.ron` and its prose pruned before the second `items.ron`
    // named it: a save stopped between the two lost the item.
    let mut expected_binder_dirs: BTreeSet<String> = BTreeSet::new();
    let mut lists: Vec<(PathBuf, ItemsFile)> = Vec::with_capacity(bundle.binders.len());
    let mut text_prunes: Vec<(PathBuf, BTreeSet<String>, BTreeSet<String>)> =
        Vec::with_capacity(bundle.binders.len());

    for (index, bb) in bundle.binders.iter().enumerate() {
        let dir_name = binder_dir_name(index, &bb.binder.name);
        expected_binder_dirs.insert(nfc(&dir_name));
        // The directory as it is already spelled on disk (see `locate`): a twin
        // beside it would be pruned as stale on a byte-exact filesystem and would
        // collide with it on a normalisation-insensitive one.
        let bdir = locator.locate_child(&binders_dir, &dir_name);
        let tdir = bdir.join("text");
        own_dir(root, &tdir)?;

        // Prose blobs + the set of expected `.djot` file names, and beside each one
        // its `.comments.ron` sidecar (written only when that content actually has
        // comments, so an uncommented project grows no files at all).
        let mut expected_prose: BTreeSet<String> = BTreeSet::new();
        // ONE expected-set for every `.ron` sidecar in this directory, not one per
        // kind. `prune_dir` matches by bare extension, so a second call carrying only
        // the footnote names would delete every `.comments.ron` beside them, and a
        // reciprocal call would delete every `.footnotes.ron`. The union is the only
        // shape that is right for both.
        let mut expected_sidecars: BTreeSet<String> = BTreeSet::new();
        for item in &bb.items {
            for pr in &item.item.prose_refs {
                let rel = Path::new(&pr.path);
                let fname = rel
                    .file_name()
                    .and_then(|n| n.to_str())
                    .ok_or_else(|| anyhow::anyhow!("bad prose path '{}'", pr.path))?
                    .to_string();
                let data = item.prose.get(&pr.file_id).ok_or_else(|| {
                    anyhow::anyhow!("missing prose blob for content {}", pr.file_id)
                })?;
                write_if_changed(
                    root,
                    &locator.locate(root, &pr.path, "prose")?,
                    data.as_bytes(),
                )?;
                expected_prose.insert(nfc(&fname));

                if let Some(comments) = item.comments.get(&pr.file_id)
                    && !comments.is_empty()
                {
                    let cname = comments_file_name(&fname);
                    write_if_changed(
                        root,
                        &locator.locate_child(&tdir, &cname),
                        to_ron(comments)?.as_bytes(),
                    )?;
                    expected_sidecars.insert(nfc(&cname));
                }

                if let Some(footnotes) = item.footnotes.get(&pr.file_id)
                    && !footnotes.is_empty()
                {
                    let fnname = footnotes_file_name(&fname);
                    write_if_changed(
                        root,
                        &locator.locate_child(&tdir, &fnname),
                        to_ron(footnotes)?.as_bytes(),
                    )?;
                    expected_sidecars.insert(nfc(&fnname));
                }
            }
        }
        let items_file = ItemsFile {
            binder: bb.binder.clone(),
            items: bb.items.iter().map(|i| i.item.clone()).collect(),
            moving_out: Vec::new(),
        };
        lists.push((bdir, items_file));
        text_prunes.push((tdir, expected_prose, expected_sidecars));
    }
    // Every `items.ron` after all the prose exists, and before any prune: until it is
    // written the `items.ron` on disk still names the old files (a renamed scene's file
    // name changes, a moved scene's folder does), and a save cut short with those gone
    // would leave a project that does not open, or opens without the scene.
    write_binder_lists(root, &binders_dir, &lists)?;

    // Commit point — written last.
    //
    // The content-derived read floor is stamped **here**, not by whoever built the
    // bundle. This is the single site every write path funnels through — `save_work` /
    // `save_as` / `backup_now` via `from_entities`, the Plume importer via its own
    // hand-written manifest literal, `mark_existing_as_backup` via read-modify-write,
    // and the zip shape via `write_zip`'s staging dir — so a producer cannot forget it,
    // and a future producer gets it for free. Doing it at the construction sites instead
    // would mean one more place to wire up (and to notice was missing) per producer.
    //
    // Only the manifest is cloned, so this costs nothing measurable next to the prose
    // blobs already written above.
    let mut manifest = bundle.manifest.clone();
    manifest.format_min_read_version = Some(compute_min_read_version(bundle));
    manifest.legacy_resource_dirs = bundle
        .carried
        .keys()
        .filter_map(|rel| super::carry::resource_owner(rel))
        .collect();
    write_if_changed(
        root,
        &root.join(MANIFEST_NAME),
        to_ron(&manifest)?.as_bytes(),
    )?;

    // After the commit point: every `items.ron` is written by now, so no file pruned
    // below is one an `items.ron` on disk still names.
    for (tdir, expected_prose, expected_sidecars) in &text_prunes {
        prune_dir(tdir, expected_prose, "djot", &carried)?;
        // Prunes a sidecar whose last comment or footnote was deleted, too —
        // `expected_sidecars` only holds the ones that still have content.
        // `items.ron` lives in the binder dir, not `text/`, so pruning "ron" here
        // cannot reach it.
        prune_dir(tdir, expected_sidecars, "ron", &carried)?;
    }
    // Until the manifest is written, the one on disk still lists a binder the writer
    // deleted, and the reader refuses a project whose manifest names a binder with no
    // folder. This also removes the lists [`write_binder_lists`] set aside.
    prune_binder_dirs(root, &binders_dir, &expected_binder_dirs)?;
    // What a carried file left behind where it moved from (see `place_carried`), once
    // its new copy is down. Never through a link, like every prune here.
    for rel in left {
        if let Ok(old) = locator.locate(root, rel, "carried file")
            && !crate::media::passes_through_link(root, &old)
        {
            fs::remove_file(&old).ok();
        }
    }

    // A staging folder is new, so it holds no leftover from an earlier write.
    if let Placing::Keep = placing {
        let mut written: BTreeSet<PathBuf> = [
            root.to_path_buf(),
            root.join(TEMPLATES_DIR),
            assets_dir,
            history_dir,
            binders_dir,
        ]
        .into_iter()
        .collect();
        for (bdir, _) in &lists {
            written.insert(bdir.join("text"));
            written.insert(bdir.clone());
        }
        for r in &bundle.resources {
            let rel = crate::slug::resource_relpath(r.uid, &r.file_name);
            if let Ok(file) = locator.locate(root, &rel, "resource")
                && let Some(parent) = file.parent()
            {
                written.insert(parent.to_path_buf());
            }
        }
        for rel in placed.keys() {
            if let Ok(file) = locator.locate(root, rel, "carried file")
                && let Some(parent) = file.parent()
            {
                written.insert(parent.to_path_buf());
            }
        }
        for dir in &written {
            sweep_temp_leftovers(root, dir);
        }
    }
    Ok(())
}

/// How long a temporary file must have been left alone before a save takes it for a
/// crash's leftover. A write in progress keeps touching its file, and every write here
/// renames its file into place the moment it is done, so a file this old belongs to no
/// write still running, in this process or another.
const TEMP_LEFTOVER_AGE: std::time::Duration = std::time::Duration::from_secs(15 * 60);

/// Whether `name` is one this crate gives a temporary file beside its target: `.tmp`
/// (`tempfile`'s default, which [`write_if_changed`] and the zip writer use) or `.blob`
/// ([`crate::Blob::place_at`]), then six letters or digits. Nothing else is ever taken
/// for a leftover, however it is named.
fn is_temp_leftover_name(name: &str) -> bool {
    [".tmp", ".blob"].iter().any(|prefix| {
        name.strip_prefix(prefix)
            .is_some_and(|rest| rest.len() == 6 && rest.bytes().all(|b| b.is_ascii_alphanumeric()))
    })
}

/// Remove from the folder `dir` of the project at `root` the temporary files a write cut
/// short (a crash, a power cut) left there: names [`is_temp_leftover_name`] accepts, on
/// plain files not modified for [`TEMP_LEFTOVER_AGE`]. The prunes match by extension and
/// a temporary name has none, and since such names are hidden they are not carried
/// either (see [`super::carry::is_hidden`]), so without this they stayed for good.
///
/// The age is the modification time: the temporary files written into a project folder
/// are filled as they are written. The one kind that is not, the hard link
/// [`crate::Blob::link_or_place_at`] makes under a temporary name, is made only in a
/// zip's staging folder or outside the project folder, never in a folder swept here.
///
/// Best effort: a leftover that cannot be removed now is met again by the next save.
/// Never through a link, like every prune here.
fn sweep_temp_leftovers(root: &Path, dir: &Path) {
    if crate::media::passes_through_link(root, dir) {
        return;
    }
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };
    let now = std::time::SystemTime::now();
    for entry in entries.flatten() {
        if !entry
            .file_name()
            .to_str()
            .is_some_and(is_temp_leftover_name)
        {
            continue;
        }
        // `DirEntry::metadata` does not follow a link, so a link is never a file here.
        let Ok(meta) = entry.metadata() else {
            continue;
        };
        let old_enough = meta
            .modified()
            .ok()
            .and_then(|touched| now.duration_since(touched).ok())
            .is_some_and(|age| age >= TEMP_LEFTOVER_AGE);
        if meta.is_file() && old_enough {
            fs::remove_file(entry.path()).ok();
        }
    }
}

/// Whether two binder rows name the same binder: by `uid` when both have one, else by
/// `file_id` (a project written before binders had a uid).
fn same_binder(a: &BinderFile, b: &BinderFile) -> bool {
    if a.uid.is_nil() || b.uid.is_nil() {
        a.file_id == b.file_id
    } else {
        a.uid == b.uid
    }
}

/// The name, in the folder `binders/`, of the folder that keeps, until the next manifest
/// is written, a binder list a save had to write over in the folder `folder`. Hidden, so
/// it is neither carried nor read before a binder's own folder.
fn set_aside_name(folder: &std::ffi::OsStr) -> std::ffi::OsString {
    let mut name = std::ffi::OsString::from(".set-aside-");
    name.push(folder);
    name
}

/// The binder list in the folder `bdir` of the project at `root` as it is on disk: its
/// bytes and what they hold. `None` when there is none, when it is reached through a
/// link, or when it does not parse, which the reader could not take either. One that
/// cannot be read is an error: the save stops before it writes over it.
fn list_on_disk(root: &Path, bdir: &Path) -> Result<Option<(Vec<u8>, ItemsFile)>> {
    let path = bdir.join("items.ron");
    if crate::media::passes_through_link(root, &path) {
        return Ok(None);
    }
    match fs::read(&path) {
        Ok(bytes) => {
            let parsed = std::str::from_utf8(&bytes)
                .ok()
                .and_then(|text| ron::from_str::<ItemsFile>(text).ok());
            Ok(parsed.map(|list| (bytes, list)))
        }
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(None),
        Err(e) => Err(e).with_context(|| format!("reading {}", path.display())),
    }
}

/// Write each binder's `items.ron` (`lists`: the binder's folder and its list), so that
/// at every moment of the write every item is named by some `items.ron` on disk, and
/// every binder the manifest on disk lists is found.
///
/// Two things would break that, written one binder after the other:
///
/// * **An item moved between binders.** Its old binder's list drops it, its new
///   binder's list names it, and whichever is written first, there is a moment when
///   the other is not, or when both are. So it goes in three steps. First every list on
///   disk that holds items now another binder's is rewritten where it is, the same
///   items, with those marked as moving out ([`ItemsFile::moving_out`]). Then every
///   binder gets its new list, a binder that loses items *with those items still in
///   it*, still marked. Last, those binders get their lists without them. Cut short
///   anywhere, a moved item is in its old binder's list or its new one, often both, and
///   whenever it is in both, the copy in the old binder's is marked: the reader keeps
///   only the other ([`drop_interrupted_moves`]).
/// * **A folder that holds another binder's list.** A binder's folder is named by its
///   position and its name, so two binders of the same name that swap places swap
///   folders, and a binder made where a deleted one was takes its folder. Before
///   writing over another binder's list, the save keeps a copy of it in a hidden
///   folder beside ([`set_aside_name`]), which the reader takes only for a binder no
///   other folder holds, and which the prune of binder folders removes once the
///   manifest is written.
///
/// The prose these lists name is already on disk, and nothing is pruned until the
/// manifest is written, so every list on disk, old or new, names files that are there.
fn write_binder_lists(
    root: &Path,
    binders_dir: &Path,
    lists: &[(PathBuf, ItemsFile)],
) -> Result<()> {
    // The binder each item now belongs to, by its position in `lists`.
    let mut home: BTreeMap<uuid::Uuid, usize> = BTreeMap::new();
    for (index, (_, list)) in lists.iter().enumerate() {
        for item in &list.items {
            if !item.uid.is_nil() {
                home.insert(item.uid, index);
            }
        }
    }
    // Every list on disk now, by its folder's name, before anything is written. No two
    // binders of `lists` share a folder, so each folder's entry is still what is on disk
    // when its binder is written below. The set-aside folders of an earlier save cut
    // short are left out: their binders' own folders are read instead.
    let mut on_disk: BTreeMap<std::ffi::OsString, (Vec<u8>, ItemsFile)> = BTreeMap::new();
    if let Ok(entries) = fs::read_dir(binders_dir) {
        for entry in entries.flatten() {
            let folder = entry.file_name();
            if folder.as_encoded_bytes().starts_with(b".") {
                continue;
            }
            if let Some(list) = list_on_disk(root, &entry.path())? {
                on_disk.insert(folder, list);
            }
        }
    }

    // Step one: mark, in place, every list on disk holding items that now belong to
    // another binder, before any list names one of them anew. A binder the writer
    // deleted is marked too: until the manifest is written, it is still read.
    for (folder, (bytes, old)) in on_disk.iter_mut() {
        let moving: Vec<uuid::Uuid> = old
            .items
            .iter()
            .filter(|item| {
                home.get(&item.uid)
                    .is_some_and(|&to| !same_binder(&lists[to].1.binder, &old.binder))
            })
            .map(|item| item.uid)
            .filter(|uid| !old.moving_out.contains(uid))
            .collect();
        if moving.is_empty() {
            continue;
        }
        old.moving_out.extend(moving);
        let marked = to_ron(&*old)?.into_bytes();
        write_if_changed(root, &binders_dir.join(folder).join("items.ron"), &marked)?;
        *bytes = marked;
    }

    // Step two: every binder's new list, still holding, marked, what it loses.
    let mut second_pass = Vec::new();
    for (index, (bdir, list)) in lists.iter().enumerate() {
        // The items this binder's list on disk holds that are now another binder's.
        let mut seen = BTreeSet::new();
        let moved_out: Vec<BinderItemFile> = on_disk
            .values()
            .map(|(_, old)| old)
            .filter(|old| same_binder(&old.binder, &list.binder))
            .flat_map(|old| &old.items)
            .filter(|item| home.get(&item.uid).is_some_and(|&to| to != index))
            .filter(|item| seen.insert(item.uid))
            .cloned()
            .collect();
        let first = if moved_out.is_empty() {
            to_ron(list)?
        } else {
            second_pass.push(index);
            // Marked, so that a reader meeting a copy in the item's new binder too knows
            // this one for the leftover (see `drop_interrupted_moves`).
            to_ron(&ItemsFile {
                binder: list.binder.clone(),
                moving_out: moved_out.iter().map(|item| item.uid).collect(),
                items: list.items.iter().cloned().chain(moved_out).collect(),
            })?
        };
        // Another binder's list in this folder is copied aside before it is written over.
        if let Some(folder) = bdir.file_name()
            && let Some((bytes, old)) = on_disk.get(folder)
            && !same_binder(&old.binder, &list.binder)
        {
            let aside = binders_dir.join(set_aside_name(folder));
            own_dir(root, &aside)?;
            write_if_changed(root, &aside.join("items.ron"), bytes)?;
        }
        write_if_changed(root, &bdir.join("items.ron"), first.as_bytes())?;
    }
    // Step three: the lists without what moved out, now that its new binders name it.
    for index in second_pass {
        let (bdir, list) = &lists[index];
        write_if_changed(root, &bdir.join("items.ron"), to_ron(list)?.as_bytes())?;
    }
    Ok(())
}

/// The comment sidecar's name for a prose blob: `12-the-lamp.djot` →
/// `12-the-lamp.comments.ron`. Derived from the blob's own file name so the two
/// always sort together and a reader can find one from the other without a lookup
/// table — the same reason the prose name already carries its content `file_id`.
///
/// Only the suffix changes, so a blob's bundle-relative path gives the sidecar's,
/// which is how a refusal names it.
pub(crate) fn comments_file_name(prose_file_name: &str) -> String {
    let stem = prose_file_name
        .strip_suffix(".djot")
        .unwrap_or(prose_file_name);
    format!("{stem}.comments.ron")
}

/// The footnote sidecar beside a prose blob: `<stem>.footnotes.ron`.
pub(crate) fn footnotes_file_name(prose_file_name: &str) -> String {
    let stem = prose_file_name
        .strip_suffix(".djot")
        .unwrap_or(prose_file_name);
    format!("{stem}.footnotes.ron")
}

/// Whether a prune may list `dir`, under the project folder `root`: not when it is
/// reached through a link, whose listing is a folder elsewhere on the disk, and whose
/// files the prune would delete. A write into it replaces the link first (see
/// [`own_way`]), so this only meets one the save had nothing to write into. An entry
/// that is itself a link is removed as one, which removes only the link.
fn prunable(root: &Path, dir: &Path) -> bool {
    if crate::media::passes_through_link(root, dir) {
        eprintln!(
            "skrib: not pruning {}: it is a link, not a folder of the project",
            dir.display()
        );
        return false;
    }
    true
}

/// Remove files in `dir` with extension `ext` whose name is not in `keep` — and
/// which this write is not carrying through (see [`Carried`]).
///
/// `keep` holds NFC names and the comparison is by NFC form, like every prune here:
/// a file whose name reached this machine in another normalisation is the file the
/// bundle means, not a stale one (see [`super::locate`]).
fn prune_dir(dir: &Path, keep: &BTreeSet<String>, ext: &str, carried: &Carried) -> Result<()> {
    if !prunable(carried.root, dir) {
        return Ok(());
    }
    let Ok(entries) = fs::read_dir(dir) else {
        return Ok(());
    };
    for entry in entries.flatten() {
        let p = entry.path();
        if p.extension().and_then(|e| e.to_str()) == Some(ext)
            && let Some(name) = p.file_name().and_then(|n| n.to_str())
            && !keep.contains(&nfc(name))
            && !carried.holds(&p)
        {
            fs::remove_file(&p).ok();
        }
    }
    Ok(())
}

/// Delete any file in the assets directory that the bundle no longer lists.
///
/// Unlike [`prune_dir`] this does not filter by extension: assets are `.png`,
/// `.jpg`, `.webp` — whatever the writer inserted — so the keep-set is the only
/// thing that can decide. It matters more than the other prunes, too: assets are
/// content-addressed, so *replacing* an image writes a new filename and orphans
/// the old blob. Without this a project would keep every version of every
/// picture ever swapped out.
fn prune_assets_dir(dir: &Path, keep: &BTreeSet<String>, carried: &Carried) -> Result<()> {
    if !prunable(carried.root, dir) {
        return Ok(());
    }
    let Ok(entries) = fs::read_dir(dir) else {
        return Ok(());
    };
    for entry in entries.flatten() {
        let p = entry.path();
        if p.is_file()
            && let Some(name) = p.file_name().and_then(|n| n.to_str())
            && !keep.contains(&nfc(name))
            && !carried.holds(&p)
        {
            fs::remove_file(&p).ok();
        }
    }
    Ok(())
}

/// Remove whole binder directories the bundle no longer lists.
///
/// Deliberately **not** carry-aware, unlike its two neighbours. A directory is left
/// over here for one of two reasons. Its binder was renamed or moved, and has a new
/// folder: the files carried from the old one were already written into the new one
/// ([`place_carried`]), so removing the old folder loses nothing. Or its binder is gone
/// from the bundle, because the writer deleted it: an unmodelled sidecar that belonged
/// to a deleted binder has nothing left to annotate, and carrying it would resurrect a
/// file for a binder that no longer exists, growing the project forever. Carrying
/// protects data whose *meaning* this build cannot judge; it does not override a
/// deletion the writer actually asked for.
///
/// Run after the manifest is written, never before (see `write_folder_placing`).
fn prune_binder_dirs(root: &Path, binders_dir: &Path, keep: &BTreeSet<String>) -> Result<()> {
    if !prunable(root, binders_dir) {
        return Ok(());
    }
    let Ok(entries) = fs::read_dir(binders_dir) else {
        return Ok(());
    };
    for entry in entries.flatten() {
        let p = entry.path();
        if p.is_dir()
            && let Some(name) = p.file_name().and_then(|n| n.to_str())
            && !keep.contains(&nfc(name))
        {
            fs::remove_dir_all(&p).ok();
        }
    }
    Ok(())
}

pub fn read_folder(root: &Path) -> Result<WorkBundle> {
    read_folder_kept(root, None)
}

/// [`read_folder`], for a folder that is a scratch extraction: every blob the bundle
/// points into it keeps `keep` alive, so the folder outlives the read for exactly as
/// long as a blob still needs it.
pub(crate) fn read_folder_kept(
    root: &Path,
    keep: Option<std::sync::Arc<tempfile::TempDir>>,
) -> Result<WorkBundle> {
    crate::media::refuse_link(root, &root.join(MANIFEST_NAME), "the manifest")?;
    let manifest_text = fs::read_to_string(root.join(MANIFEST_NAME))
        .with_context(|| format!("reading {}", root.join(MANIFEST_NAME).display()))?;
    let mut manifest: ProjectManifest = from_ron(&manifest_text, "project.skrib")?;
    let locator = Locator::new();

    let tags = read_ron_vec(root, "tags.ron")?;
    let dict_words = read_ron_vec(root, "dictionary.ron")?;
    // Additive: a pre-feature bundle has no `replacements.ron`; `read_ron_vec` treats a
    // missing file as an empty vec, so old projects load with zero rules (no version bump).
    let text_replacement_rules = read_ron_vec(root, "replacements.ron")?;
    let trash_infos = read_ron_vec(root, "trash.ron")?;
    // Additive: a pre-Pace bundle has no `paces.ron`; `read_ron_vec` treats a missing
    // file as an empty vec, so old projects load with zero paces (no version bump).
    let paces = read_ron_vec(root, "paces.ron")?;
    let progress_snapshots = read_ron_vec(root, "snapshots.ron")?;
    // Additive: a bundle written before comments existed has no orphanage, which
    // `read_ron_vec` reads back as an empty vec — no `format_version` bump needed.
    //
    // The bodies are Djot the comment and footnote cards parse, so they are held to
    // the same ceiling as the prose below, and refused the same way.
    let mut orphan_comments: Vec<CommentFile> = read_ron_vec(root, "orphan_comments.ron")?;
    admit_comment_bodies(&manifest, &mut orphan_comments, "orphan_comments.ron")?;
    let mut orphan_footnotes: Vec<FootnoteFile> = read_ron_vec(root, "orphan_footnotes.ron")?;
    crate::djot_depth::admit_footnotes(&mut orphan_footnotes)
        .context("footnotes orphan_footnotes.ron")?;
    // Additive like its neighbours: a pre-v5 bundle has no `templates.ron` and reads back
    // as zero templates. A *malformed* one is a hard error, matching every sibling here —
    // and for a sharper reason than consistency. Degrading to "no templates" would let
    // autosave rewrite the file from that empty state within seconds of opening, so a
    // typo in a hand-edited manifest would cost the writer every template permanently.
    // Refusing to open keeps the bytes on disk and is recoverable.
    let note_templates: Vec<NoteTemplateFile> = read_ron_vec(root, "templates.ron")?;
    let mut note_template_bodies = std::collections::BTreeMap::new();
    for t in &note_templates {
        let path = locator.locate(root, &t.path, "note-template body")?;
        crate::media::refuse_link(root, &path, "the note template")?;
        let text = fs::read_to_string(&path)
            .with_context(|| format!("reading note-template body {}", t.path))?;
        let text = crate::djot_depth::admit(text)
            .with_context(|| format!("note-template body {}", t.path))?;
        note_template_bodies.insert(t.file_id, text);
    }

    // The history log.
    //
    // **Deliberately not `read_ron_vec`**, whose contract is a hard error on
    // malformed input. Every sibling above holds manuscript content, where
    // refusing to open is the recoverable outcome. This file holds a convenience,
    // and a truncated index is not a reason to lock a writer out of their book —
    // so it degrades to an empty log.
    //
    // Blobs are loaded **eagerly**, not on demand, and that is load-bearing for
    // the zip shape: `read_zip` extracts into a tempdir that is gone by the time
    // `write_zip` packs a *fresh* staging directory, so a blob left on disk rather
    // than carried in memory would simply not be in the next archive. Thinning is
    // what bounds the cost; a blob whose file has vanished is skipped rather than
    // failing the open, and the next write prunes its index entry's claim on it.
    let history = {
        let index_path = root.join(HISTORY_INDEX);
        // A link is left out, like an unreadable index: the log is a convenience.
        let linked = crate::media::passes_through_link(root, &index_path);
        if linked {
            eprintln!(
                "skrib: ignoring history index {}: it is a link, not a file of the project",
                index_path.display()
            );
        }
        let read = if linked {
            Err(std::io::ErrorKind::NotFound.into())
        } else {
            fs::read_to_string(&index_path)
        };
        let entries: Vec<super::history::HistoryEntry> = match read {
            Err(_) => Vec::new(),
            Ok(text) => ron::from_str(&text).unwrap_or_else(|e| {
                eprintln!(
                    "skrib: ignoring unreadable history index {}: {e}",
                    index_path.display()
                );
                Vec::new()
            }),
        };
        let mut blobs = std::collections::BTreeMap::new();
        for hash in entries.iter().map(|e| e.hash.clone()) {
            if blobs.contains_key(&hash) {
                continue;
            }
            // Bounded like every other prose blob: the Versions pane parses
            // these, so an unbounded one aborts the process from there.
            // Dropping the blob rather than failing the open matches the rest of
            // the history reader, which treats a missing blob as a thinned entry
            // — a convenience file must not lock a writer out of their book.
            let blob_path = root.join(blob_relpath(&hash));
            if crate::media::passes_through_link(root, &blob_path) {
                eprintln!(
                    "skrib: ignoring history blob {}: it is a link, not a file of the project",
                    blob_path.display()
                );
                continue;
            }
            if let Ok(text) = fs::read_to_string(&blob_path)
                && let Ok(text) = crate::djot_depth::admit(text)
            {
                blobs.insert(hash, text);
            }
        }
        // Drop entries whose blob is gone, so the in-memory log never claims prose
        // it cannot produce — which is exactly what `write_folder` asserts.
        let entries = entries
            .into_iter()
            .filter(|e| blobs.contains_key(&e.hash))
            .collect();
        super::history::HistoryLog { entries, blobs }
    };

    // Assets. `fs::read`, not `read_to_string` — this is the one part of a
    // bundle that is not UTF-8, and every other reader here would reject it.
    let assets: Vec<AssetFile> = read_ron_vec(root, "assets.ron")?;
    // Absent before v14, and an absent ladder is a legitimate "no statuses" state, so
    // `read_ron_vec` returning empty for a missing file is exactly right here.
    let statuses: Vec<BinderStatusFile> = read_ron_vec(root, "statuses.ron")?;
    // Absent before v17, and an absent list is the "nothing attached" state.
    let resources: Vec<ResourceFile> = read_ron_vec(root, "resources.ron")?;
    // Each stored file pointed at, never read. A missing one is not an error, unlike a
    // missing asset: the resource's card loads and shows its file as missing, and the
    // writer can locate it or remove it. Its path is rebuilt from `uid` and `file_name`,
    // the same rule the writer follows, rather than trusted from the row. One reached
    // through a symbolic link is left out the same way, and said: its bytes are not the
    // project's, and a blob over it would copy them into the next archive or backup
    // (see `media::passes_through_link`).
    let mut resource_blobs = std::collections::BTreeMap::new();
    for r in &resources {
        if r.kind != common::entities::ResourceKind::StoredFile {
            continue;
        }
        let rel = crate::slug::resource_relpath(r.uid, &r.file_name);
        let Ok(mut path) = locator.locate(root, &rel, "resource") else {
            continue;
        };
        // Under the name an earlier build gave it, when today's is not there: see
        // `slug::legacy_safe_file_name` for the rules that changed. Opening the project
        // renames it (`media::prune_resources_dir`), and a save writes today's name.
        if fs::symlink_metadata(&path).is_err()
            && let Some(legacy) = crate::slug::legacy_resource_relpath(r.uid, &r.file_name)
            && let Ok(found) = locator.locate(root, &legacy, "resource")
            && fs::symlink_metadata(&found).is_ok_and(|m| m.is_file() || m.is_symlink())
        {
            path = found;
        }
        if crate::media::passes_through_link(root, &path) {
            eprintln!(
                "skrib: ignoring resource {}: it is a link, not a file of the project",
                path.display()
            );
            continue;
        }
        if !path.is_file() {
            continue;
        }
        let blob = match &keep {
            Some(keep) => crate::Blob::file_in(path, std::sync::Arc::clone(keep)),
            None => crate::Blob::file(path),
        };
        resource_blobs.insert(r.uid.hyphenated().to_string(), blob);
    }
    // Each asset is checked to be there, never read: its blob points at the file,
    // and only a writer that copies it somewhere opens it.
    let mut asset_blobs = std::collections::BTreeMap::new();
    for a in &assets {
        let path = locator.locate(root, &a.path, "asset")?;
        // A picture reached through a link is left out, like a stored resource above,
        // rather than refused: the project still opens, and its next save drops the row
        // whose bytes it never had (`from_entities` keeps only assets with a blob).
        if crate::media::passes_through_link(root, &path) {
            eprintln!(
                "skrib: ignoring asset {}: it is a link, not a file of the project",
                path.display()
            );
            continue;
        }
        let meta = fs::metadata(&path)
            .with_context(|| format!("reading asset {} ({})", a.file_name, a.path))?;
        if !meta.is_file() {
            anyhow::bail!("reading asset {} ({}): not a file", a.file_name, a.path);
        }
        let blob = match &keep {
            Some(keep) => crate::Blob::file_in(path, std::sync::Arc::clone(keep)),
            None => crate::Blob::file(path),
        };
        asset_blobs.insert(a.content_hash.clone(), blob);
    }

    // Index every binder by its file id (dir names are cosmetic).
    let mut by_id: std::collections::HashMap<u64, (ItemsFile, PathBuf)> =
        std::collections::HashMap::new();
    let binders_dir = root.join("binders");
    if let Ok(entries) = fs::read_dir(&binders_dir) {
        // A hidden folder holds a list a save cut short had set aside (see
        // `write_binder_lists`): read last, and taken only for a binder no other folder
        // holds, since the binder's own folder, once written, is the newer list.
        let mut entries: Vec<fs::DirEntry> = entries.flatten().collect();
        entries.sort_by_key(|e| e.file_name().as_encoded_bytes().starts_with(b"."));
        for entry in entries {
            let set_aside = entry.file_name().as_encoded_bytes().starts_with(b".");
            let items_path = entry.path().join("items.ron");
            crate::media::refuse_link(root, &items_path, "the binder list")?;
            if items_path.is_file() {
                let text = fs::read_to_string(&items_path)
                    .with_context(|| format!("reading {}", items_path.display()))?;
                let itf: ItemsFile = from_ron(&text, "items.ron")?;
                if set_aside && by_id.contains_key(&itf.binder.file_id) {
                    continue;
                }
                by_id.insert(itf.binder.file_id, (itf, entry.path()));
            }
        }
    }

    let mut lists = Vec::with_capacity(manifest.binder_order.len());
    for bid in &manifest.binder_order {
        let (itf, _dir) = by_id.remove(bid).ok_or_else(|| {
            anyhow::anyhow!("binder {bid} listed in manifest but no items.ron found")
        })?;
        lists.push(itf);
    }
    // Before any prose is read: a leftover copy is not read at all.
    drop_interrupted_moves(&mut lists);

    let mut binders = Vec::with_capacity(lists.len());
    for itf in lists {
        let mut items = Vec::with_capacity(itf.items.len());
        for item in itf.items {
            let mut prose = std::collections::BTreeMap::new();
            let mut comments = std::collections::BTreeMap::new();
            let mut footnotes = std::collections::BTreeMap::new();
            for pr in &item.prose_refs {
                let prose_path = locator.locate(root, &pr.path, "prose")?;
                crate::media::refuse_link(root, &prose_path, "the prose file")?;
                let text = fs::read_to_string(&prose_path)
                    .with_context(|| format!("reading prose {}", pr.path))?;
                // Refuse before the prose can reach an editor or an exporter: the
                // Djot parser's recursion is unbounded and a stack overflow is an
                // abort, not a panic, so there is no later place to catch this.
                let text =
                    crate::djot_depth::admit(text).with_context(|| format!("prose {}", pr.path))?;
                prose.insert(pr.file_id, text);

                // Additive and optional: a bundle written before comments existed —
                // or any content the writer never annotated — simply has no sidecar,
                // which reads back as "no comments" rather than as an error. That is
                // what lets this ship without a `format_version` bump.
                //
                // Every body in a sidecar is Djot, parsed by the comment and footnote
                // cards and by the exporter, so it is refused here for the same reason
                // and in the same words as the prose beside it, naming the sidecar.
                if let (Some(dir), Some(fname)) = (
                    prose_path.parent(),
                    prose_path.file_name().and_then(|n| n.to_str()),
                ) {
                    // A comment list reached through a link is left out, a footnote list
                    // refused: a remark can be missing, words of the book cannot (see
                    // the module docs).
                    let cpath = locator.locate_child(dir, &comments_file_name(fname));
                    let linked = crate::media::passes_through_link(root, &cpath);
                    if linked {
                        eprintln!(
                            "skrib: ignoring comments {}: it is a link, not a file of the project",
                            cpath.display()
                        );
                    }
                    let ctext = if linked {
                        None
                    } else {
                        read_if_present(&cpath).with_context(|| {
                            format!("reading comments {}", comments_file_name(&pr.path))
                        })?
                    };
                    if let Some(ctext) = ctext {
                        let mut list: Vec<CommentFile> = from_ron(&ctext, "comments.ron")?;
                        admit_comment_bodies(&manifest, &mut list, &comments_file_name(&pr.path))?;
                        if !list.is_empty() {
                            comments.insert(pr.file_id, list);
                        }
                    }
                    let fpath = locator.locate_child(dir, &footnotes_file_name(fname));
                    crate::media::refuse_link(root, &fpath, "the footnote list")?;
                    let ftext = read_if_present(&fpath).with_context(|| {
                        format!("reading footnotes {}", footnotes_file_name(&pr.path))
                    })?;
                    if let Some(ftext) = ftext {
                        let mut list: Vec<FootnoteFile> = from_ron(&ftext, "footnotes.ron")?;
                        crate::djot_depth::admit_footnotes(&mut list).with_context(|| {
                            format!("footnotes {}", footnotes_file_name(&pr.path))
                        })?;
                        if !list.is_empty() {
                            footnotes.insert(pr.file_id, list);
                        }
                    }
                }
            }
            items.push(BundledItem {
                item,
                footnotes,
                prose,
                comments,
            });
        }
        binders.push(BundledBinder {
            binder: itf.binder,
            items,
        });
    }

    // Everything on disk the reader above did not claim. Walking the tree once,
    // *after* the modelled reads, is what makes this self-maintaining: a file
    // kind added to the format later needs no entry in a list here, and a file
    // kind this build has never heard of is preserved rather than dropped.
    //
    // Directories are not carried — only files. An empty directory holds no
    // data, and `write_if_changed` recreates any parent a carried file needs.
    //
    // Nor is anything under a hidden name, which is not entered at all: a `.git`
    // folder, a crash's leftover temporary file (see `carry::is_hidden`).
    let mut carried: BTreeMap<String, CarriedFile> = BTreeMap::new();
    let live = super::carry::stored_uids(&resources);
    let walk = walkdir::WalkDir::new(root)
        .sort_by_file_name()
        .into_iter()
        .filter_entry(super::carry::walk_enters);
    for entry in walk {
        let entry = entry.with_context(|| format!("walking {}", root.display()))?;
        // The walk does not follow links (`WalkDir`'s default; only the project folder
        // itself is), so neither a linked file nor anything under a linked folder is
        // carried: its bytes are not the project's. A link is neither a file nor a
        // folder to the walk, and is said once here when nothing above reported it.
        let linked = entry.depth() > 0 && entry.path_is_symlink();
        if !linked && !entry.file_type().is_file() {
            continue;
        }
        let Some(key) = rel_key(root, entry.path()) else {
            continue;
        };
        if super::carry::modelled_in(&key, || Some(&manifest), &live)
            || super::carry::is_stray_bundle(&key)
        {
            continue;
        }
        if linked {
            eprintln!("skrib: not carrying {key}: it is a link, not a file of the project");
            continue;
        }
        let bytes = fs::read(entry.path())
            .with_context(|| format!("reading unmodelled bundle file {key}"))?;
        // Not told which binder it came from (`CarriedFile::binder`). Every save the
        // application makes reads what it carries from disk again, binder and all
        // (`carry::load_for_save`), and none uses this map. A tool that writes back a
        // bundle read here after renaming or moving a binder has to do the same, or
        // the files carried from that binder's folder go with the old folder.
        carried.insert(key, CarriedFile::new(bytes));
    }

    manifest.legacy_resource_dirs = carried
        .keys()
        .filter_map(|rel| super::carry::resource_owner(rel))
        .collect();
    Ok(WorkBundle {
        manifest,
        tags,
        dict_words,
        text_replacement_rules,
        note_templates,
        statuses,
        note_template_bodies,
        assets,
        asset_blobs,
        resources,
        resource_blobs,
        trash_infos,
        paces,
        progress_snapshots,
        orphan_comments,
        orphan_footnotes,
        history,
        binders,
        carried,
    })
}

/// Leave every binder item in one binder's list only, for `lists` read in manifest order.
///
/// A save cut short while it moved an item between binders leaves it in both lists: the
/// old binder's list still holds it, marked as moving out ([`ItemsFile::moving_out`]),
/// and the new binder's may name it already (see `write_binder_lists`). Read as it is,
/// the project would hold two items with one uid, and everything that finds an item by
/// its uid (an open tab, a mention, the version history, an extension) would find either
/// one. So a marked copy is dropped wherever another copy of the item is: the one in the
/// new binder is the one the writer last saw. A marked copy that is the only one is kept,
/// since the new binder's list does not name the item yet, and the marks themselves are
/// never kept past the read.
///
/// An item that is still in two lists after that was not put there by a save cut short
/// (a hand edit, say), and nothing tells which copy is stale. Both are kept, so no words
/// are lost, and each later copy gets a uid of its own, derived from its binder, its
/// place and its old uid (`bundle::derived_uid`), so that reading the same lists again
/// gives the same uid until a save writes it. Only the item's own uid is given anew.
///
/// A pure function of the lists: a second read of the same folder gives the same result.
fn drop_interrupted_moves(lists: &mut [ItemsFile]) {
    let mut copies: BTreeMap<uuid::Uuid, usize> = BTreeMap::new();
    for item in lists.iter().flat_map(|list| &list.items) {
        if !item.uid.is_nil() {
            *copies.entry(item.uid).or_default() += 1;
        }
    }
    for list in lists.iter_mut() {
        let moving: BTreeSet<uuid::Uuid> =
            std::mem::take(&mut list.moving_out).into_iter().collect();
        if moving.is_empty() {
            continue;
        }
        list.items.retain(|item| {
            let left_over =
                moving.contains(&item.uid) && copies.get(&item.uid).is_some_and(|&count| count > 1);
            if left_over && let Some(count) = copies.get_mut(&item.uid) {
                *count -= 1;
            }
            !left_over
        });
    }

    let mut seen: BTreeSet<uuid::Uuid> = BTreeSet::new();
    for list in lists.iter_mut() {
        for (row, item) in list.items.iter_mut().enumerate() {
            if item.uid.is_nil() || seen.insert(item.uid) {
                continue;
            }
            let old = item.uid;
            item.uid = (0u32..)
                .map(|attempt| {
                    super::bundle::derived_uid(
                        "skribisto binder item uid heal 2026-11",
                        &[
                            list.binder.uid.as_bytes(),
                            &list.binder.file_id.to_le_bytes(),
                            &(row as u64).to_le_bytes(),
                            old.as_bytes(),
                            &attempt.to_le_bytes(),
                        ],
                    )
                })
                .find(|uid| !copies.contains_key(uid) && !seen.contains(uid))
                .unwrap_or_else(uuid::Uuid::new_v4);
            seen.insert(item.uid);
            eprintln!(
                "skrib: the item {old} is in more than one binder; the copy in binder {} is \
                 given the uid {}",
                list.binder.file_id, item.uid
            );
        }
    }
}

/// Hold the bodies in one file of comment threads to what the Djot parser can be given
/// (`djot_depth::admit`, which may join a body's lines in place), naming `file` when one
/// cannot be, if `manifest`'s bundle stores them as Djot.
///
/// A bundle stamped before [`COMMENT_BODIES_ARE_DJOT_FROM`] stores them as plain text,
/// which nothing parses as it stands: `migrate_bundle` rewrites each one as the Djot that
/// reads back as the same words, and that Djot is within the ceiling whatever the words
/// were (`plain_text_to_djot_verbatim`). Refusing such a body for looking nested would lock
/// a writer out of an older project over a remark that could never have reached the
/// parser as nesting.
fn admit_comment_bodies(
    manifest: &ProjectManifest,
    threads: &mut [CommentFile],
    file: &str,
) -> Result<()> {
    if manifest.format_version < COMMENT_BODIES_ARE_DJOT_FROM {
        return Ok(());
    }
    crate::djot_depth::admit_comments(threads).with_context(|| format!("comments {file}"))
}

/// The list in the work-level manifest `name` of the project folder `root`, empty when
/// the file is not there. One reached through a link is refused like every manifest
/// (see the module docs): read as missing, its list would be saved back empty.
///
/// Only a file that is not there reads as empty. One that is there and cannot be read
/// (see [`read_if_present`]) refuses the project, like one that does not parse.
fn read_ron_vec<T: DeserializeOwned>(root: &Path, name: &str) -> Result<Vec<T>> {
    let path = root.join(name);
    crate::media::refuse_link(root, &path, "the list")?;
    match read_if_present(&path).with_context(|| format!("reading {name}"))? {
        Some(text) => from_ron(&text, name),
        None => Ok(Vec::new()), // a missing optional manifest = empty
    }
}

/// The text of the file at `path`, or `None` when there is no file there.
///
/// Every other failure is an error: a byte that is not UTF-8, a permission refused, a
/// file a sync client holds locked. Read as missing, the list in such a file would open
/// empty, and the next save, seeing nothing to keep, would delete the file and the
/// comments, footnotes or tags in it. Refused, the file stays on disk as it is, the
/// same as a file that does not parse.
fn read_if_present(path: &Path) -> std::io::Result<Option<String>> {
    match fs::read_to_string(path) {
        Ok(text) => Ok(Some(text)),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(None),
        Err(e) => Err(e),
    }
}
