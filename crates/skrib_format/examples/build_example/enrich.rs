// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! `enrich` — adds the **editorial metadata** to a bundled example.
//!
//! Synopses, the story bible, the tag vocabulary, points of view and the pinned
//! cast/place/object references. None of it is part of the book: it is description
//! *about* the work, written by the Skribisto project, and each example's `NOTICE`
//! says so. The one line this must not cross is the prose, so it snapshots every
//! scene and paratext page on the way in and asserts them byte-identical on the way
//! out — a future edit here cannot quietly start rewriting an author's sentences.
//!
//! ## Re-runnable, unlike its predecessor
//!
//! The JSON-driven tool this replaces was documented "idempotent-unsafe — running it
//! twice appends a second set of synopses". That is a trap with no upside: the fix
//! is simply to *replace* rather than append, which is what happens here. Every
//! synopsis row it does not recognise is dropped before the new one is written, and
//! the story-bible binder is rebuilt from nothing each run. Regenerating an example
//! after a one-word change to its TOML is therefore a one-line diff, not a decision
//! about whether the bundle is still pristine.

use std::collections::{BTreeMap, BTreeSet};

use anyhow::{Result, bail, ensure};
use common::entities::{BinderItemRole as Role, BinderItemSubRole as SubRole, ContentRole};
use skrib_format::{
    BinderFile, BinderItemFile, BinderTagFile, BundledBinder, BundledItem, ProseRef, SkribShape,
    WorkBundle, binder_dir_name, markdown_to_djot, prose_file_name, prose_relpath, read_bundle,
    write_bundle,
};

use crate::spec::{Entry, Spec, uid_for};

pub fn run(spec_path: &str, bundle_path: &str) -> Result<()> {
    let spec = Spec::load(spec_path)?;
    let mut bundle = read_bundle(bundle_path)?;
    let before = prose_snapshot(&bundle);

    let tags = ensure_tags(&mut bundle, &spec);
    // `bible` is keyed by name *and* by every alias, so its length is not the number
    // of notes written — count the entries themselves.
    let notes = spec.entries().count();

    // The order of these four is load-bearing: **everything this tool wrote last
    // time is removed before anything new is numbered.** Strip the synopses, drop
    // and rebuild the bible, then write. Getting it wrong does not corrupt the
    // bundle — it just makes every content id climb by a few dozen on every run, so
    // the same TOML produces a different file each time and no diff of an example is
    // readable.
    let targets = resolve_targets(&bundle, &spec)?;
    strip_synopses(&mut bundle, &targets);
    let bible = build_bible(&mut bundle, &spec, &tags)?;
    let chapters = write_chapters(&mut bundle, &spec, &tags, &bible, &targets)?;

    let after = prose_snapshot(&bundle);
    ensure!(
        before == after,
        "the author's prose changed — this subcommand may only add metadata"
    );

    write_bundle(bundle_path, SkribShape::ZipFile, &bundle)?;
    println!(
        "enrich: {bundle_path} — {chapters} chapters annotated, {notes} story-bible \
         notes, {} tags",
        tags.len()
    );
    Ok(())
}

/// Every scene, paratext page and epigraph, keyed by (item uid, content id).
///
/// Deliberately *not* keyed by `file_id` alone: this subcommand renumbers nothing,
/// but a comparison that could be satisfied by two different rows swapping ids would
/// not be the guarantee it claims to be.
fn prose_snapshot(bundle: &WorkBundle) -> BTreeMap<(uuid::Uuid, u64), String> {
    let mut out = BTreeMap::new();
    for binder in &bundle.binders {
        for item in &binder.items {
            for r in &item.item.prose_refs {
                if matches!(
                    r.role,
                    ContentRole::SceneText | ContentRole::ParatextText | ContentRole::EpigraphText
                ) && let Some(text) = item.prose.get(&r.file_id)
                {
                    out.insert((item.item.uid, r.file_id), text.clone());
                }
            }
        }
    }
    out
}

/// Tag name → `file_id`, creating whatever the TOML declares and the bundle lacks.
///
/// Reuse is by **name**, case-sensitively, because that is what the writer sees and
/// what a `[[chapter]]` line spells. Creating a second "character" tag beside an
/// existing one would split the mention index in half with no visible cause.
fn ensure_tags(bundle: &mut WorkBundle, spec: &Spec) -> BTreeMap<String, u64> {
    let mut next = bundle.tags.iter().map(|t| t.file_id).max().unwrap_or(0);
    let mut by_name: BTreeMap<String, u64> = bundle
        .tags
        .iter()
        .map(|t| (t.name.clone(), t.file_id))
        .collect();
    let now = spec.book.timestamp.clone();

    for declared in &spec.tag {
        if let Some(id) = by_name.get(&declared.name) {
            // Keep the row's identity, refresh what the TOML owns.
            if let Some(row) = bundle.tags.iter_mut().find(|t| t.file_id == *id) {
                row.color = declared.color.clone();
                row.details = declared.details.clone();
                row.discoverable = declared.discoverable;
            }
            continue;
        }
        next += 1;
        bundle.tags.push(BinderTagFile {
            file_id: next,
            uid: uid_for(&spec.book.unique_id, "tag", &declared.name),
            created_at: now.clone(),
            updated_at: now.clone(),
            name: declared.name.clone(),
            color: declared.color.clone(),
            details: declared.details.clone(),
            discoverable: declared.discoverable,
            creates_in: None,
            note_template: None,
        });
        by_name.insert(declared.name.clone(), next);
    }

    let ids: BTreeSet<u64> = bundle.tags.iter().map(|t| t.file_id).collect();
    bundle.manifest.work.tag_ids = ids.into_iter().collect();
    by_name
}

/// Story-bible name **or alias** → the note item's `file_id`.
///
/// Keyed by both because the two halves of the TOML speak differently on purpose:
/// a `[[chapter]]` names people the way the prose does ("Fogg"), while the bible
/// titles them fully ("Phileas Fogg"). A canonical-only map matched neither the
/// points of view nor the cast links in the predecessor, and the only symptom was a
/// smaller number in its own summary line.
type Bible = BTreeMap<String, u64>;

fn build_bible(
    bundle: &mut WorkBundle,
    spec: &Spec,
    tags: &BTreeMap<String, u64>,
) -> Result<Bible> {
    let mut bible = Bible::new();
    if spec.bible.binder.trim().is_empty() {
        return Ok(bible);
    }

    // Rebuilt from nothing every run, which is what makes the whole subcommand
    // re-runnable. Dropping it first also means a note deleted from the TOML
    // actually disappears, rather than lingering in the bundle for ever.
    if let Some(pos) = bundle
        .binders
        .iter()
        .position(|b| b.binder.name == spec.bible.binder)
    {
        let gone = bundle.binders.remove(pos).binder.file_id;
        bundle.manifest.binder_order.retain(|id| *id != gone);
    }

    let now = spec.book.timestamp.clone();
    let project = &spec.book.unique_id;
    let binder_id = bundle
        .binders
        .iter()
        .map(|b| b.binder.file_id)
        .max()
        .unwrap_or(0)
        + 1;
    let mut next_item = max_item_id(bundle);
    let mut next_content = max_content_id(bundle);
    let binder_dir = binder_dir_name(bundle.binders.len(), &spec.bible.binder);

    let kinds: [(&str, &[Entry], &str, &str); 3] = [
        (
            "character",
            &spec.character,
            &spec.bible.character_folder,
            &spec.bible.character_tag,
        ),
        (
            "place",
            &spec.place,
            &spec.bible.place_folder,
            &spec.bible.place_tag,
        ),
        (
            "object",
            &spec.object,
            &spec.bible.object_folder,
            &spec.bible.object_tag,
        ),
    ];

    let mut items: Vec<BundledItem> = Vec::new();
    for (kind, entries, folder, kind_tag) in kinds {
        if entries.is_empty() {
            continue;
        }
        let indent = if folder.is_empty() {
            0
        } else {
            next_item += 1;
            items.push(BundledItem {
                item: BinderItemFile {
                    title: folder.to_string(),
                    role: Role::Folder,
                    sub_role: SubRole::Note,
                    indent: 0,
                    is_exportable: false,
                    ..blank(next_item, uid_for(project, "bible-folder", kind), &now)
                },
                prose: BTreeMap::new(),
                comments: BTreeMap::new(),
                footnotes: BTreeMap::new(),
            });
            1
        };

        for entry in entries {
            next_item += 1;
            next_content += 1;
            let uid = uid_for(project, "bible", &entry.name);
            let mut tag_ids: Vec<u64> = entry
                .tags
                .iter()
                .chain((!kind_tag.is_empty()).then_some(&kind_tag.to_string()))
                .filter_map(|t| tags.get(t).copied())
                .collect();
            tag_ids.sort_unstable();
            tag_ids.dedup();

            let mut prose = BTreeMap::new();
            prose.insert(next_content, markdown_to_djot(&entry.note)?);

            items.push(BundledItem {
                item: BinderItemFile {
                    title: entry.name.clone(),
                    role: Role::Item,
                    sub_role: SubRole::Note,
                    indent,
                    // A story-bible note is not part of the book. Left exportable it
                    // would land in the manuscript of every export whose scope is the
                    // whole project.
                    is_exportable: false,
                    aliases: entry.aliases.clone(),
                    tag_ids,
                    prose_refs: vec![ProseRef {
                        file_id: next_content,
                        uid: uid_for(project, "bible-content", &entry.name),
                        created_at: now.clone(),
                        updated_at: now.clone(),
                        activated: true,
                        role: ContentRole::NoteText,
                        path: prose_relpath(
                            &binder_dir,
                            &prose_file_name(uid, &entry.name, &ContentRole::NoteText)
                                .expect("NoteText is a prose role"),
                        ),
                    }],
                    ..blank(next_item, uid, &now)
                },
                prose,
                comments: BTreeMap::new(),
                footnotes: BTreeMap::new(),
            });

            for name in std::iter::once(&entry.name).chain(&entry.aliases) {
                bible.insert(name.clone(), next_item);
            }
        }
    }

    if items.is_empty() {
        return Ok(bible);
    }
    bundle.binders.push(BundledBinder {
        binder: BinderFile {
            file_id: binder_id,
            uid: uid_for(project, "binder", &spec.bible.binder),
            created_at: now.clone(),
            updated_at: now,
            name: spec.bible.binder.clone(),
            activated: true,
            item_order: items.iter().map(|b| b.item.file_id).collect(),
        },
        items,
    });
    bundle.manifest.binder_order.push(binder_id);
    Ok(bible)
}

/// The manuscript row each `[[chapter]]` addresses, resolved once.
///
/// `number` counts prose rows in the manuscript binder's own stream order, which is
/// the order the writer sees. `file_id` names an existing row outright, which is how
/// a bundle this tool did not build — Starforgers — is addressed.
fn resolve_targets<'a>(
    bundle: &WorkBundle,
    spec: &'a Spec,
) -> Result<Vec<(u64, &'a crate::spec::Chapter)>> {
    let ordinals: BTreeMap<u32, u64> = bundle.binders[0]
        .items
        .iter()
        .filter(|b| {
            b.item
                .prose_refs
                .iter()
                .any(|r| r.role == ContentRole::SceneText)
        })
        .enumerate()
        .map(|(i, b)| (i as u32 + 1, b.item.file_id))
        .collect();

    let mut targets = Vec::new();
    for declared in &spec.chapter {
        let id = match (declared.number, declared.file_id) {
            (Some(n), _) => *ordinals.get(&n).ok_or_else(|| {
                anyhow::anyhow!(
                    "[[chapter]] number = {n} addresses prose row {n}, but the \
                     manuscript has only {}",
                    ordinals.len()
                )
            })?,
            (_, Some(id)) => id,
            _ => bail!("[[chapter]] must set `number` or `file_id`"),
        };
        ensure!(
            bundle.binders[0].items.iter().any(|b| b.item.file_id == id),
            "[[chapter]] addresses row {id}, which is not in the manuscript"
        );
        targets.push((id, declared));
    }
    Ok(targets)
}

/// Drop every synopsis on an addressed row. Replacing rather than appending is what
/// makes this subcommand re-runnable at all — see the module header.
fn strip_synopses(bundle: &mut WorkBundle, targets: &[(u64, &crate::spec::Chapter)]) {
    for (id, _) in targets {
        let Some(bundled) = bundle.binders[0]
            .items
            .iter_mut()
            .find(|b| b.item.file_id == *id)
        else {
            continue;
        };
        for r in bundled
            .item
            .prose_refs
            .iter()
            .filter(|r| r.role == ContentRole::SynopsisText)
        {
            bundled.prose.remove(&r.file_id);
        }
        bundled
            .item
            .prose_refs
            .retain(|r| r.role != ContentRole::SynopsisText);
    }
}

/// Write each chapter's synopsis, point of view, references, tags and label.
fn write_chapters(
    bundle: &mut WorkBundle,
    spec: &Spec,
    tags: &BTreeMap<String, u64>,
    bible: &Bible,
    targets: &[(u64, &crate::spec::Chapter)],
) -> Result<usize> {
    if targets.is_empty() {
        return Ok(0);
    }
    let now = spec.book.timestamp.clone();
    let project = spec.book.unique_id.clone();
    let binder_dir = binder_dir_name(0, &bundle.binders[0].binder.name);
    let mut next_content = max_content_id(bundle);

    for (id, declared) in targets {
        let Some(bundled) = bundle.binders[0]
            .items
            .iter_mut()
            .find(|b| b.item.file_id == *id)
        else {
            continue;
        };
        if !declared.synopsis.trim().is_empty() {
            next_content += 1;
            bundled
                .prose
                .insert(next_content, markdown_to_djot(&declared.synopsis)?);
            bundled.item.prose_refs.push(ProseRef {
                file_id: next_content,
                uid: uid_for(&project, "synopsis", &id.to_string()),
                created_at: now.clone(),
                updated_at: now.clone(),
                activated: true,
                role: ContentRole::SynopsisText,
                path: prose_relpath(
                    &binder_dir,
                    &prose_file_name(
                        bundled.item.uid,
                        &bundled.item.title,
                        &ContentRole::SynopsisText,
                    )
                    .expect("SynopsisText is a prose role"),
                ),
            });
        }

        bundled.item.point_of_view_ids = resolve(&declared.pov, bible);
        // The cast, places and things a chapter names, pinned as `References` — the
        // same relationship the Inspector's Cast section pins by hand, and the one
        // the mention scan unions its own hits into.
        bundled.item.reference_ids = resolve(
            &declared
                .cast
                .iter()
                .chain(&declared.places)
                .chain(&declared.objects)
                .cloned()
                .collect::<Vec<_>>(),
            bible,
        );
        bundled.item.tag_ids = declared
            .tags
            .iter()
            .filter_map(|t| tags.get(t).copied())
            .collect();
        if !declared.label.is_empty() {
            bundled.item.label = declared.label.clone();
        }
    }
    Ok(targets.len())
}

/// Names to note ids, deduplicated and ordered so the bundle is byte-stable.
///
/// A name the bible does not know is impossible here: [`Spec::validate`] refused the
/// file before anything was read.
fn resolve(names: &[String], bible: &Bible) -> Vec<u64> {
    let mut ids: Vec<u64> = names.iter().filter_map(|n| bible.get(n).copied()).collect();
    ids.sort_unstable();
    ids.dedup();
    ids
}

fn max_item_id(bundle: &WorkBundle) -> u64 {
    bundle
        .binders
        .iter()
        .flat_map(|b| &b.items)
        .map(|b| b.item.file_id)
        .max()
        .unwrap_or(0)
}

fn max_content_id(bundle: &WorkBundle) -> u64 {
    bundle
        .binders
        .iter()
        .flat_map(|b| &b.items)
        .flat_map(|b| b.item.prose_refs.iter().map(|r| r.file_id))
        .chain(
            bundle
                .binders
                .iter()
                .flat_map(|b| &b.items)
                .flat_map(|b| b.item.inline_contents.iter().map(|c| c.file_id)),
        )
        .max()
        .unwrap_or(0)
}

fn blank(file_id: u64, uid: uuid::Uuid, now: &str) -> BinderItemFile {
    BinderItemFile {
        file_id,
        uid,
        created_at: now.to_string(),
        updated_at: now.to_string(),
        title: String::new(),
        sub_title: String::new(),
        role: Role::Item,
        sub_role: SubRole::Note,
        label: String::new(),
        activated: true,
        is_favorite: false,
        is_exportable: true,
        exclude_from_numbering: false,
        indent: 0,
        word_count_goal: 0,
        char_count_goal: 0,
        dict_language: Vec::new(),
        aliases: Vec::new(),
        inline_contents: Vec::new(),
        prose_refs: Vec::new(),
        reference_ids: Vec::new(),
        point_of_view_ids: Vec::new(),
        book_ids: Vec::new(),
        tag_ids: Vec::new(),
    }
}
