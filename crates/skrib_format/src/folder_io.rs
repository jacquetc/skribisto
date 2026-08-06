// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Exploded-folder read/write. Writes are atomic (tmp + rename), manifest-last
//! (the `project.skrib` is the commit point), and **diff-minimal**: a blob is
//! only touched when its bytes actually change, and orphaned blobs/dirs are
//! pruned — so an exploded project under git shows a tight diff.

use anyhow::{Context, Result};
use serde::Serialize;
use serde::de::DeserializeOwned;
use std::collections::BTreeSet;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use tempfile::NamedTempFile;

use super::bundle::*;
use super::shape::MANIFEST_NAME;
use super::slug::{ASSETS_DIR, TEMPLATES_DIR, binder_dir_name};
use super::version_gate::compute_min_read_version;
use super::writer::persist_durably;

fn to_ron<T: Serialize>(value: &T) -> Result<String> {
    let cfg = ron::ser::PrettyConfig::new().struct_names(true);
    let mut s = ron::ser::to_string_pretty(value, cfg)?;
    s.push('\n');
    Ok(s)
}

fn from_ron<T: DeserializeOwned>(text: &str, what: &str) -> Result<T> {
    ron::from_str(text).with_context(|| format!("parsing {what}"))
}

/// Write `bytes` to `path` only if the on-disk content differs. Returns whether
/// a write happened. Uses a same-dir temp file + atomic rename.
fn write_if_changed(path: &Path, bytes: &[u8]) -> Result<bool> {
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
    tmp.write_all(bytes)
        .with_context(|| format!("writing {}", path.display()))?;
    persist_durably(tmp, path)?;
    Ok(true)
}

pub fn write_folder(root: &Path, bundle: &WorkBundle) -> Result<()> {
    fs::create_dir_all(root).with_context(|| format!("creating {}", root.display()))?;
    let binders_dir = root.join("binders");
    fs::create_dir_all(&binders_dir).ok();

    // Work-level manifests.
    write_if_changed(&root.join("tags.ron"), to_ron(&bundle.tags)?.as_bytes())?;
    write_if_changed(
        &root.join("dictionary.ron"),
        to_ron(&bundle.dict_words)?.as_bytes(),
    )?;
    write_if_changed(
        &root.join("replacements.ron"),
        to_ron(&bundle.text_replacement_rules)?.as_bytes(),
    )?;
    write_if_changed(
        &root.join("trash.ron"),
        to_ron(&bundle.trash_infos)?.as_bytes(),
    )?;
    write_if_changed(&root.join("paces.ron"), to_ron(&bundle.paces)?.as_bytes())?;
    write_if_changed(
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
        write_if_changed(&orphans_path, to_ron(&bundle.orphan_comments)?.as_bytes())?;
    }

    // The same for footnotes whose annotated content is gone. Kept, not dropped:
    // a comment's loss costs a remark, a footnote's costs words from the book.
    let orphan_notes_path = root.join("orphan_footnotes.ron");
    if bundle.orphan_footnotes.is_empty() {
        fs::remove_file(&orphan_notes_path).ok();
    } else {
        write_if_changed(
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
        &root.join("templates.ron"),
        to_ron(&bundle.note_templates)?.as_bytes(),
    )?;
    let templates_dir = root.join(TEMPLATES_DIR);
    let mut expected_templates: BTreeSet<String> = BTreeSet::new();
    if !bundle.note_templates.is_empty() {
        fs::create_dir_all(&templates_dir)
            .with_context(|| format!("creating {}", templates_dir.display()))?;
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
        write_if_changed(&root.join(rel), body.as_bytes())?;
        expected_templates.insert(fname);
    }
    prune_dir(&templates_dir, &expected_templates, "djot")?;

    // Assets: an index plus one blob each, the same split templates use — but
    // written as bytes, not text. `write_if_changed` already takes `&[u8]` and
    // diffs against what is on disk, so an unchanged image is not rewritten and
    // the exploded shape stays diff-minimal even with photographs in it.
    //
    // The prune matters more here than anywhere else: assets are content-
    // addressed, so *replacing* an image writes a new name and leaves the old
    // blob behind. Without this a project would accumulate every version of
    // every picture a writer ever swapped out, forever.
    write_if_changed(&root.join("assets.ron"), to_ron(&bundle.assets)?.as_bytes())?;
    let assets_dir = root.join(ASSETS_DIR);
    let mut expected_assets: BTreeSet<String> = BTreeSet::new();
    if !bundle.assets.is_empty() {
        fs::create_dir_all(&assets_dir)
            .with_context(|| format!("creating {}", assets_dir.display()))?;
    }
    for a in &bundle.assets {
        let rel = Path::new(&a.path);
        let fname = rel
            .file_name()
            .and_then(|n| n.to_str())
            .ok_or_else(|| anyhow::anyhow!("bad asset path '{}'", a.path))?
            .to_string();
        let bytes = bundle.asset_bytes.get(&a.content_hash).ok_or_else(|| {
            anyhow::anyhow!(
                "missing bytes for asset {} ({})",
                a.file_name,
                a.content_hash
            )
        })?;
        write_if_changed(&root.join(rel), bytes)?;
        expected_assets.insert(fname);
    }
    prune_assets_dir(&assets_dir, &expected_assets)?;

    let mut expected_binder_dirs: BTreeSet<String> = BTreeSet::new();

    for (index, bb) in bundle.binders.iter().enumerate() {
        let dir_name = binder_dir_name(index, &bb.binder.name);
        expected_binder_dirs.insert(dir_name.clone());
        let bdir = binders_dir.join(&dir_name);
        let tdir = bdir.join("text");
        fs::create_dir_all(&tdir).with_context(|| format!("creating {}", tdir.display()))?;

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
                write_if_changed(&root.join(rel), data.as_bytes())?;
                expected_prose.insert(fname.clone());

                if let Some(comments) = item.comments.get(&pr.file_id)
                    && !comments.is_empty()
                {
                    let cname = comments_file_name(&fname);
                    write_if_changed(&tdir.join(&cname), to_ron(comments)?.as_bytes())?;
                    expected_sidecars.insert(cname);
                }

                if let Some(footnotes) = item.footnotes.get(&pr.file_id)
                    && !footnotes.is_empty()
                {
                    let fnname = footnotes_file_name(&fname);
                    write_if_changed(&tdir.join(&fnname), to_ron(footnotes)?.as_bytes())?;
                    expected_sidecars.insert(fnname);
                }
            }
        }
        prune_dir(&tdir, &expected_prose, "djot")?;
        // Prunes a sidecar whose last comment or footnote was deleted, too —
        // `expected_sidecars` only holds the ones that still have content.
        // `items.ron` lives in the binder dir, not `text/`, so pruning "ron" here
        // cannot reach it.
        prune_dir(&tdir, &expected_sidecars, "ron")?;

        // items.ron (after its prose blobs exist).
        let items_file = ItemsFile {
            binder: bb.binder.clone(),
            items: bb.items.iter().map(|i| i.item.clone()).collect(),
        };
        write_if_changed(&bdir.join("items.ron"), to_ron(&items_file)?.as_bytes())?;
    }

    prune_binder_dirs(&binders_dir, &expected_binder_dirs)?;

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
    write_if_changed(&root.join(MANIFEST_NAME), to_ron(&manifest)?.as_bytes())?;
    Ok(())
}

/// The comment sidecar's name for a prose blob: `12-the-lamp.djot` →
/// `12-the-lamp.comments.ron`. Derived from the blob's own file name so the two
/// always sort together and a reader can find one from the other without a lookup
/// table — the same reason the prose name already carries its content `file_id`.
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

/// Remove files in `dir` with extension `ext` whose name is not in `keep`.
fn prune_dir(dir: &Path, keep: &BTreeSet<String>, ext: &str) -> Result<()> {
    let Ok(entries) = fs::read_dir(dir) else {
        return Ok(());
    };
    for entry in entries.flatten() {
        let p = entry.path();
        if p.extension().and_then(|e| e.to_str()) == Some(ext)
            && let Some(name) = p.file_name().and_then(|n| n.to_str())
            && !keep.contains(name)
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
fn prune_assets_dir(dir: &Path, keep: &BTreeSet<String>) -> Result<()> {
    let Ok(entries) = fs::read_dir(dir) else {
        return Ok(());
    };
    for entry in entries.flatten() {
        let p = entry.path();
        if p.is_file()
            && let Some(name) = p.file_name().and_then(|n| n.to_str())
            && !keep.contains(name)
        {
            fs::remove_file(&p).ok();
        }
    }
    Ok(())
}

fn prune_binder_dirs(binders_dir: &Path, keep: &BTreeSet<String>) -> Result<()> {
    let Ok(entries) = fs::read_dir(binders_dir) else {
        return Ok(());
    };
    for entry in entries.flatten() {
        let p = entry.path();
        if p.is_dir()
            && let Some(name) = p.file_name().and_then(|n| n.to_str())
            && !keep.contains(name)
        {
            fs::remove_dir_all(&p).ok();
        }
    }
    Ok(())
}

pub fn read_folder(root: &Path) -> Result<WorkBundle> {
    let manifest_text = fs::read_to_string(root.join(MANIFEST_NAME))
        .with_context(|| format!("reading {}", root.join(MANIFEST_NAME).display()))?;
    let manifest: ProjectManifest = from_ron(&manifest_text, "project.skrib")?;

    let tags = read_ron_vec(&root.join("tags.ron"), "tags.ron")?;
    let dict_words = read_ron_vec(&root.join("dictionary.ron"), "dictionary.ron")?;
    // Additive: a pre-feature bundle has no `replacements.ron`; `read_ron_vec` treats a
    // missing file as an empty vec, so old projects load with zero rules (no version bump).
    let text_replacement_rules = read_ron_vec(&root.join("replacements.ron"), "replacements.ron")?;
    let trash_infos = read_ron_vec(&root.join("trash.ron"), "trash.ron")?;
    // Additive: a pre-Pace bundle has no `paces.ron`; `read_ron_vec` treats a missing
    // file as an empty vec, so old projects load with zero paces (no version bump).
    let paces = read_ron_vec(&root.join("paces.ron"), "paces.ron")?;
    let progress_snapshots = read_ron_vec(&root.join("snapshots.ron"), "snapshots.ron")?;
    // Additive: a bundle written before comments existed has no orphanage, which
    // `read_ron_vec` reads back as an empty vec — no `format_version` bump needed.
    let orphan_comments = read_ron_vec(&root.join("orphan_comments.ron"), "orphan_comments.ron")?;
    let orphan_footnotes =
        read_ron_vec(&root.join("orphan_footnotes.ron"), "orphan_footnotes.ron")?;
    // Additive like its neighbours: a pre-v5 bundle has no `templates.ron` and reads back
    // as zero templates. A *malformed* one is a hard error, matching every sibling here —
    // and for a sharper reason than consistency. Degrading to "no templates" would let
    // autosave rewrite the file from that empty state within seconds of opening, so a
    // typo in a hand-edited manifest would cost the writer every template permanently.
    // Refusing to open keeps the bytes on disk and is recoverable.
    let note_templates: Vec<NoteTemplateFile> =
        read_ron_vec(&root.join("templates.ron"), "templates.ron")?;
    let mut note_template_bodies = std::collections::BTreeMap::new();
    for t in &note_templates {
        let text = fs::read_to_string(root.join(&t.path))
            .with_context(|| format!("reading note-template body {}", t.path))?;
        note_template_bodies.insert(t.file_id, text);
    }

    // Assets. `fs::read`, not `read_to_string` — this is the one part of a
    // bundle that is not UTF-8, and every other reader here would reject it.
    let assets: Vec<AssetFile> = read_ron_vec(&root.join("assets.ron"), "assets.ron")?;
    let mut asset_bytes = std::collections::BTreeMap::new();
    for a in &assets {
        let bytes = fs::read(root.join(&a.path))
            .with_context(|| format!("reading asset {} ({})", a.file_name, a.path))?;
        asset_bytes.insert(a.content_hash.clone(), bytes);
    }

    // Index every binder by its file id (dir names are cosmetic).
    let mut by_id: std::collections::HashMap<u64, (ItemsFile, PathBuf)> =
        std::collections::HashMap::new();
    let binders_dir = root.join("binders");
    if let Ok(entries) = fs::read_dir(&binders_dir) {
        for entry in entries.flatten() {
            let items_path = entry.path().join("items.ron");
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
                let prose_path = root.join(&pr.path);
                let text = fs::read_to_string(&prose_path)
                    .with_context(|| format!("reading prose {}", pr.path))?;
                prose.insert(pr.file_id, text);

                // Additive and optional: a bundle written before comments existed —
                // or any content the writer never annotated — simply has no sidecar,
                // which reads back as "no comments" rather than as an error. That is
                // what lets this ship without a `format_version` bump.
                if let (Some(dir), Some(fname)) = (
                    prose_path.parent(),
                    prose_path.file_name().and_then(|n| n.to_str()),
                ) {
                    let cpath = dir.join(comments_file_name(fname));
                    if let Ok(ctext) = fs::read_to_string(&cpath) {
                        let list: Vec<CommentFile> = from_ron(&ctext, "comments.ron")?;
                        if !list.is_empty() {
                            comments.insert(pr.file_id, list);
                        }
                    }
                    let fpath = dir.join(footnotes_file_name(fname));
                    if let Ok(ftext) = fs::read_to_string(&fpath) {
                        let list: Vec<FootnoteFile> = from_ron(&ftext, "footnotes.ron")?;
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

    Ok(WorkBundle {
        manifest,
        tags,
        dict_words,
        text_replacement_rules,
        note_templates,
        note_template_bodies,
        assets,
        asset_bytes,
        trash_infos,
        paces,
        progress_snapshots,
        orphan_comments,
        orphan_footnotes,
        binders,
    })
}

fn read_ron_vec<T: DeserializeOwned>(path: &Path, what: &str) -> Result<Vec<T>> {
    match fs::read_to_string(path) {
        Ok(text) => from_ron(&text, what),
        Err(_) => Ok(Vec::new()), // a missing optional manifest = empty
    }
}
