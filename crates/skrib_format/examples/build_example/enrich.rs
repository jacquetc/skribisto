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

use anyhow::{Result, anyhow, bail, ensure};
use common::entities::{BinderItemRole as Role, BinderItemSubRole as SubRole, ContentRole};
use skrib_format::{
    BinderFile, BinderItemFile, BinderStatusFile, BinderTagFile, BundledBinder, BundledItem,
    ProseRef, SkribShape, WorkBundle, binder_dir_name, markdown_to_djot, prose_file_name,
    prose_relpath, read_bundle, write_bundle,
};

use crate::convert::letters;
use crate::spec::{Entry, Spec, uid_for};

pub fn run(spec_path: &str, bundle_path: &str) -> Result<()> {
    let spec = Spec::load(spec_path)?;
    let mut bundle = read_bundle(bundle_path)?;
    let done = enrich(&mut bundle, &spec)?;
    write_bundle(bundle_path, SkribShape::ZipFile, &bundle)?;
    println!(
        "enrich: {bundle_path} — {} chapters annotated, {} story-bible notes, {} tags, \
         {} statuses, {} rows no longer addressed cleared",
        done.chapters, done.notes, done.tags, done.statuses, done.abandoned
    );
    Ok(())
}

/// What one run did, for the summary line.
struct Done {
    chapters: usize,
    notes: usize,
    tags: usize,
    statuses: usize,
    abandoned: usize,
}

/// The whole subcommand, in memory, so the tests can hold a shipped bundle to it.
fn enrich(bundle: &mut WorkBundle, spec: &Spec) -> Result<Done> {
    let before = prose_snapshot(bundle);

    let tags = ensure_tags(bundle, spec);
    let statuses = ensure_statuses(bundle, spec);
    // `bible` is keyed by name *and* by every alias, so its length is not the number
    // of notes written — count the entries themselves.
    let notes = spec.entries().count();

    // The order of these steps is load-bearing: **everything this tool wrote last
    // time is removed before anything new is numbered.** Strip the synopses, drop
    // and rebuild the bible, then write. Getting it wrong does not corrupt the
    // bundle — it just makes every content id climb by a few dozen on every run, so
    // the same TOML produces a different file each time and no diff of an example is
    // readable.
    let targets = resolve_targets(bundle, spec)?;
    strip_synopses(bundle, &targets);
    let abandoned = clear_abandoned(bundle, spec, &targets);
    let bible = build_bible(bundle, spec, &tags)?;
    let chapters = write_chapters(bundle, spec, &tags, &statuses, &bible, &targets)?;

    let after = prose_snapshot(bundle);
    ensure!(
        before == after,
        "the author's prose changed — this subcommand may only add metadata"
    );
    Ok(Done {
        chapters,
        notes,
        tags: tags.len(),
        statuses: statuses.len(),
        abandoned,
    })
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

    // Prune what the spec no longer declares, and sweep the dangling references it leaves.
    //
    // Without this the builder only ever *adds*: deleting a `[[tag]]` from the TOML left
    // the row in the artifact for good, so a spec and the bundle it supposedly describes
    // could disagree indefinitely with nothing saying so. Found when the four `statut/…`
    // tags outlived their own removal — they had become a status axis, and the artifact
    // went on shipping them as tags beside it.
    //
    // The TOML owns this vocabulary (the loop above already overwrites colour, details and
    // `discoverable` from it), so pruning is the same ownership carried to its end. The
    // sweep is not optional: an item's `tag_ids` are `file_id`s, and leaving one pointing
    // at a removed row is a dangling reference the format has no way to represent.
    let declared: BTreeSet<u64> = spec
        .tag
        .iter()
        .filter_map(|d| by_name.get(&d.name).copied())
        .collect();
    bundle.tags.retain(|t| declared.contains(&t.file_id));
    for bundled in bundle.binders.iter_mut().flat_map(|b| b.items.iter_mut()) {
        bundled.item.tag_ids.retain(|id| declared.contains(id));
    }
    by_name.retain(|_, id| declared.contains(id));

    let ids: BTreeSet<u64> = bundle.tags.iter().map(|t| t.file_id).collect();
    bundle.manifest.work.tag_ids = ids.into_iter().collect();
    by_name
}

/// Lay down the workflow ladder, and return `name -> file_id`.
///
/// Mirrors [`ensure_tags`] deliberately, including its "keep the row's identity, refresh
/// what the TOML owns" rule: a rung matched by name keeps its `file_id` and `uid`, so
/// re-running the builder does not re-point every chapter that wears it.
///
/// One real difference: **the order is the data.** `Work.statuses` is an
/// `ordered_one_to_many`, so the sequence of `bundle.statuses` IS the ladder — which is
/// what makes one rung "lower" than another for merge and for the Overview's sort. There
/// is no `status_ids` field on the manifest to carry it (there is none for
/// `note_templates` either, for the same reason), so this rebuilds the vector in spec
/// order rather than appending to it.
fn ensure_statuses(bundle: &mut WorkBundle, spec: &Spec) -> BTreeMap<String, u64> {
    let mut next = bundle
        .statuses
        .iter()
        .map(|s| s.file_id)
        .chain(bundle.tags.iter().map(|t| t.file_id))
        .max()
        .unwrap_or(0);
    let existing: BTreeMap<String, BinderStatusFile> = bundle
        .statuses
        .drain(..)
        .map(|s| (s.name.clone(), s))
        .collect();
    let now = spec.book.timestamp.clone();
    let mut by_name: BTreeMap<String, u64> = BTreeMap::new();

    for declared in &spec.status {
        let row = match existing.get(&declared.name) {
            Some(prev) => BinderStatusFile {
                // Identity kept; only what the TOML owns is refreshed.
                file_id: prev.file_id,
                uid: prev.uid,
                created_at: prev.created_at.clone(),
                updated_at: now.clone(),
                name: declared.name.clone(),
                category: declared.category.clone(),
                details: declared.details.clone(),
            },
            None => {
                next += 1;
                BinderStatusFile {
                    file_id: next,
                    uid: uid_for(&spec.book.unique_id, "status", &declared.name),
                    created_at: now.clone(),
                    updated_at: now.clone(),
                    name: declared.name.clone(),
                    category: declared.category.clone(),
                    details: declared.details.clone(),
                }
            }
        };
        by_name.insert(row.name.clone(), row.file_id);
        bundle.statuses.push(row);
    }

    // A rung the spec has dropped is gone from the ladder, and every item that wore it
    // reads as "no status" — which is exactly what the weak reference is for, and why
    // removing a rung here does not have to touch a single chapter.
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
        let row = bundle.binders[0]
            .items
            .iter()
            .find(|b| b.item.file_id == id)
            .ok_or_else(|| {
                anyhow!("[[chapter]] addresses row {id}, which is not in the manuscript")
            })?;
        // An id is only a number, and the bundle's author can renumber every row by
        // adding a front-matter page. Starforgers shipped exactly that way: its ids
        // were carried over from an older bundle, landed five rows early, and wrote
        // Chapter 1's synopsis and cast onto the dedication.
        let names = row_names(row);
        ensure!(
            declared.title.is_empty()
                || names.iter().any(|n| letters(n) == letters(&declared.title)),
            "[[chapter]] {:?} addresses row {id}, which calls itself {names:?}",
            declared.title
        );
        targets.push((id, declared));
    }
    Ok(targets)
}

/// What a row calls itself: its binder title, and the opening line of its scene
/// text. Both are needed. Starforgers' chapter rows have no binder title, only a
/// `## Chapter 1` heading in the prose, while its Prologue has a title and opens
/// straight into the text.
fn row_names(row: &BundledItem) -> Vec<&str> {
    let opening = row
        .item
        .prose_refs
        .iter()
        .filter(|r| r.role == ContentRole::SceneText)
        .filter_map(|r| row.prose.get(&r.file_id))
        .filter_map(|text| text.lines().map(str::trim).find(|l| !l.is_empty()));
    std::iter::once(row.item.title.as_str())
        .filter(|t| !t.is_empty())
        .chain(opening)
        .collect()
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

/// Clear what an earlier run wrote on a row the TOML no longer addresses, and return
/// how many rows that was.
///
/// Such a row is recognised by its synopsis: [`write_chapters`] gives each one a uid
/// derived from the row's id, which no synopsis written in the app can carry. Without
/// this, a `[[chapter]]` that moves or goes away leaves its old synopsis, cast and
/// point of view behind on a row nothing describes any more. Starforgers was
/// corrected exactly that way, and its front matter kept Chapter 1's cast.
fn clear_abandoned(
    bundle: &mut WorkBundle,
    spec: &Spec,
    targets: &[(u64, &crate::spec::Chapter)],
) -> usize {
    let addressed: BTreeSet<u64> = targets.iter().map(|(id, _)| *id).collect();
    let project = &spec.book.unique_id;
    let mut cleared = 0;
    for bundled in &mut bundle.binders[0].items {
        let id = bundled.item.file_id;
        if addressed.contains(&id) {
            continue;
        }
        let ours = uid_for(project, "synopsis", &id.to_string());
        let Some(pos) = bundled
            .item
            .prose_refs
            .iter()
            .position(|r| r.role == ContentRole::SynopsisText && r.uid == ours)
        else {
            continue;
        };
        let gone = bundled.item.prose_refs.remove(pos);
        bundled.prose.remove(&gone.file_id);
        bundled.item.point_of_view_ids.clear();
        bundled.item.reference_ids.clear();
        bundled.item.tag_ids.clear();
        bundled.item.status_id = None;
        cleared += 1;
    }
    cleared
}

/// Write each chapter's synopsis, point of view, references, tags, status and label.
fn write_chapters(
    bundle: &mut WorkBundle,
    spec: &Spec,
    tags: &BTreeMap<String, u64>,
    statuses: &BTreeMap<String, u64>,
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
        // A hard error, not a `filter_map` like the tags above: a status is single-valued,
        // so a typo does not merely lose one of several marks — it silently leaves the row
        // unmarked, and "unmarked" is a legitimate state no reader can tell from a mistake.
        // The spec's own header sets this discipline for cast names; it applies more here.
        bundled.item.status_id = if declared.status.is_empty() {
            None
        } else {
            Some(*statuses.get(&declared.status).ok_or_else(|| {
                anyhow!(
                    "chapter {:?} names status {:?}, which no [[status]] entry declares",
                    declared.title,
                    declared.status
                )
            })?)
        };
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
        status_id: None,
        cover_id: None,
        resource_ids: Vec::new(),
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

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLES: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../resources/examples");

    /// Each shipped bundle must be exactly what its TOML builds: enriching it again
    /// changes nothing. That also runs the title check over every `[[chapter]]`, so
    /// a spec whose rows drifted, as Starforgers' did, fails here rather than in
    /// front of a writer opening Chapter 1.
    #[test]
    fn every_shipped_example_is_what_its_spec_builds() {
        for (spec, bundle) in [
            (
                "starforgers/starforgers.toml",
                "starforgers/Starforgers.skrib",
            ),
            (
                "le_tour_du_monde_en_80_jours/le-tour-du-monde-en-quatre-vingts-jours.toml",
                "le_tour_du_monde_en_80_jours/le-tour-du-monde-en-quatre-vingts-jours.skrib",
            ),
        ] {
            let spec = Spec::load(&format!("{EXAMPLES}/{spec}")).unwrap();
            let shipped = read_bundle(&format!("{EXAMPLES}/{bundle}")).unwrap();
            let mut rebuilt = shipped.clone();
            enrich(&mut rebuilt, &spec).unwrap();
            assert!(
                rebuilt == shipped,
                "{bundle} is stale: rerun `build_example enrich` on it"
            );
        }
    }

    /// The check that would have caught Starforgers: an id one row off is refused.
    #[test]
    fn a_chapter_addressing_the_wrong_row_is_refused() {
        let mut spec = Spec::load(&format!("{EXAMPLES}/starforgers/starforgers.toml")).unwrap();
        let bundle = read_bundle(&format!("{EXAMPLES}/starforgers/Starforgers.skrib")).unwrap();
        // Chapter 1 is row 9; row 4 is the dedication.
        let chapter_1 = spec
            .chapter
            .iter_mut()
            .find(|c| c.title == "Chapter 1")
            .unwrap();
        chapter_1.file_id = Some(4);
        let err = resolve_targets(&bundle, &spec).unwrap_err().to_string();
        assert!(err.contains("Dedication"), "{err}");
    }

    /// A row the TOML stops addressing loses what the last run wrote on it, and a
    /// synopsis the writer typed in the app is left alone.
    #[test]
    fn a_row_no_longer_addressed_is_cleared_of_our_synopsis_only() {
        let mut spec = Spec::load(&format!("{EXAMPLES}/starforgers/starforgers.toml")).unwrap();
        let mut bundle = read_bundle(&format!("{EXAMPLES}/starforgers/Starforgers.skrib")).unwrap();
        let row = |b: &WorkBundle, id: u64| {
            b.binders[0]
                .items
                .iter()
                .find(|i| i.item.file_id == id)
                .unwrap()
                .clone()
        };
        let synopses = |i: &BundledItem| {
            i.item
                .prose_refs
                .iter()
                .filter(|r| r.role == ContentRole::SynopsisText)
                .count()
        };
        assert_eq!(synopses(&row(&bundle, 9)), 1);
        assert!(!row(&bundle, 9).item.reference_ids.is_empty());

        spec.chapter.retain(|c| c.title != "Chapter 1");
        enrich(&mut bundle, &spec).unwrap();
        let chapter_1 = row(&bundle, 9);
        assert_eq!(synopses(&chapter_1), 0);
        assert!(chapter_1.item.reference_ids.is_empty());
        assert!(chapter_1.item.point_of_view_ids.is_empty());
        // Chapter 2 is still addressed and keeps everything.
        assert_eq!(synopses(&row(&bundle, 10)), 1);
    }
}
