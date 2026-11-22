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
    paths: &'a BTreeMap<String, CarriedFile>,
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

    let carried = Carried {
        root,
        paths: &bundle.carried,
    };
    let locator = Locator::new();

    // Carried files go down **first**, so that anything this build also models
    // overwrites them with its own authoritative bytes rather than the other way
    // round. `write_if_changed` keeps the exploded shape diff-minimal: a carried
    // file whose bytes did not change is not rewritten, so it stays out of the
    // git diff exactly like modelled content.
    for (rel, file) in &bundle.carried {
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
    write_if_changed(
        root,
        &root.join("templates.ron"),
        to_ron(&bundle.note_templates)?.as_bytes(),
    )?;
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
        &root.join("assets.ron"),
        to_ron(&bundle.assets)?.as_bytes(),
    )?;
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
    prune_assets_dir(&assets_dir, &expected_assets, &carried)?;

    // Resources: the index, then each stored file in its own `resources/<uid>/`
    // folder under its own name. Placed like assets (a folder project saved in place
    // writes nothing), and the path always rebuilt from `uid` and `file_name` here,
    // never taken from the row, so a crafted `path` cannot steer a write.
    //
    // No prune, unlike assets: a resource removed during a session keeps its file
    // until the project is next opened (`media::prune_resources_dir`), which is what
    // lets Undo bring it back with its bytes. A zip is staged afresh and so never
    // carries a removed one anyway. A stored resource with no blob is written as a
    // row only: shown as missing, not dropped.
    write_if_changed(
        root,
        &root.join("resources.ron"),
        to_ron(&bundle.resources)?.as_bytes(),
    )?;
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
        write_if_changed(
            root,
            &root.join(HISTORY_INDEX),
            to_ron(&bundle.history.entries)?.as_bytes(),
        )?;
        let mut expected_blobs: BTreeSet<String> = BTreeSet::new();
        for hash in bundle.history.referenced_hashes() {
            let text = bundle.history.blobs.get(&hash).ok_or_else(|| {
                anyhow::anyhow!("history index references blob {hash} with no bytes in the bundle")
            })?;
            write_if_changed(root, &root.join(blob_relpath(&hash)), text.as_bytes())?;
            expected_blobs.insert(format!("{hash}.djot"));
        }
        prune_dir(&history_dir, &expected_blobs, "djot", &carried)?;
    }

    let mut expected_binder_dirs: BTreeSet<String> = BTreeSet::new();

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
        prune_dir(&tdir, &expected_prose, "djot", &carried)?;
        // Prunes a sidecar whose last comment or footnote was deleted, too —
        // `expected_sidecars` only holds the ones that still have content.
        // `items.ron` lives in the binder dir, not `text/`, so pruning "ron" here
        // cannot reach it.
        prune_dir(&tdir, &expected_sidecars, "ron", &carried)?;

        // items.ron (after its prose blobs exist).
        let items_file = ItemsFile {
            binder: bb.binder.clone(),
            items: bb.items.iter().map(|i| i.item.clone()).collect(),
        };
        write_if_changed(
            root,
            &bdir.join("items.ron"),
            to_ron(&items_file)?.as_bytes(),
        )?;
    }

    prune_binder_dirs(root, &binders_dir, &expected_binder_dirs)?;

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
/// Deliberately **not** carry-aware, unlike its two neighbours. A directory
/// disappears here only because the binder itself is gone from the bundle —
/// the writer deleted it — and an unmodelled sidecar that belonged to a deleted
/// binder has nothing left to annotate. Carrying it would resurrect a file for a
/// binder that no longer exists, growing the project forever. Carrying protects
/// data whose *meaning* this build cannot judge; it does not override a
/// deletion the writer actually asked for.
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
        for entry in entries.flatten() {
            let items_path = entry.path().join("items.ron");
            crate::media::refuse_link(root, &items_path, "the binder list")?;
            if items_path.is_file() {
                let text = fs::read_to_string(&items_path)
                    .with_context(|| format!("reading {}", items_path.display()))?;
                let itf: ItemsFile = from_ron(&text, "items.ron")?;
                by_id.insert(itf.binder.file_id, (itf, entry.path()));
            }
        }
    }

    let mut binders = Vec::with_capacity(manifest.binder_order.len());
    for bid in &manifest.binder_order {
        let (itf, _dir) = by_id.remove(bid).ok_or_else(|| {
            anyhow::anyhow!("binder {bid} listed in manifest but no items.ron found")
        })?;
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
                    if !linked && let Ok(ctext) = fs::read_to_string(&cpath) {
                        let mut list: Vec<CommentFile> = from_ron(&ctext, "comments.ron")?;
                        admit_comment_bodies(&manifest, &mut list, &comments_file_name(&pr.path))?;
                        if !list.is_empty() {
                            comments.insert(pr.file_id, list);
                        }
                    }
                    let fpath = locator.locate_child(dir, &footnotes_file_name(fname));
                    crate::media::refuse_link(root, &fpath, "the footnote list")?;
                    if let Ok(ftext) = fs::read_to_string(&fpath) {
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
    let mut carried: BTreeMap<String, CarriedFile> = BTreeMap::new();
    let live = super::carry::stored_uids(&resources);
    for entry in walkdir::WalkDir::new(root).sort_by_file_name() {
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
fn read_ron_vec<T: DeserializeOwned>(root: &Path, name: &str) -> Result<Vec<T>> {
    let path = root.join(name);
    crate::media::refuse_link(root, &path, "the list")?;
    match fs::read_to_string(&path) {
        Ok(text) => from_ron(&text, name),
        Err(_) => Ok(Vec::new()), // a missing optional manifest = empty
    }
}
