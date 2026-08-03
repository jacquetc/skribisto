// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! One-off tool that adds **descriptive metadata** to the bundled Starforgers example.
//!
//! Run with:
//! ```sh
//! cargo run -p skribisto-skrib-format --example enrich_example_fixture -- \
//!     resources/examples/Starforgers.skrib resources/examples/Starforgers-metadata.json
//! ```
//!
//! The metadata file is committed beside the bundle, so the enrichment is reproducible and
//! reviewable: what was added to the example is readable as JSON without unzipping anything.
//! It is **idempotent-unsafe** — running it twice appends a second set of synopses — so it
//! is a regeneration tool for a pristine bundle, not a migration.
//!
//! ## Why a tool and not a hand-edited `items.ron`
//!
//! The bundle is read and written through this crate's own `read_bundle` /
//! `write_bundle`, so the result is valid by construction. Hand-editing RON would produce a
//! file whose validity is only discovered when a user opens it.
//!
//! ## What it may and may not touch
//!
//! `Starforgers` is a real, in-print commercial novel used with its author's agreement (see
//! `resources/examples/NOTICE`). **The prose is never modified** — this tool only ever adds
//! synopses, tags, story-bible notes, aliases and point-of-view links, all of which are
//! editorial description *about* the work rather than part of it. It asserts that the scene
//! text it read back is byte-identical to what it writes out, so a future edit here cannot
//! quietly cross that line.
//!
//! Deliberate defects for testing repetition detection are **not** planted here; they belong
//! in a synthetic, project-owned fixture.

use std::collections::BTreeMap;

use serde::Deserialize;
use skrib_format::{
    BinderTagFile, BundledBinder, BundledItem, ProseRef, SkribShape, read_bundle, write_bundle,
};

/// One chapter's editorial metadata, as produced by the reading pass.
#[derive(Debug, Deserialize)]
struct ChapterMeta {
    file_id: u64,
    #[allow(dead_code)]
    title: String,
    synopsis: String,
    pov_character: String,
    characters_present: Vec<String>,
}

/// A story-bible entry: display name, the other names the prose uses for them, and a short
/// factual note. All original wording — none of it is lifted from the novel.
struct CastMember {
    name: &'static str,
    aliases: &'static [&'static str],
    note: &'static str,
}

/// The recurring cast, drawn from the reading pass's own tallies. Deliberately not
/// exhaustive: a story bible of every walk-on would demonstrate nothing except clutter.
const CAST: &[CastMember] = &[
    CastMember {
        name: "Devon Ardel",
        aliases: &["Devon"],
        note: "Stellar Ranger on the frontier moon Ocherva, later recruited into the \
               Starforgers. Daughter of Senator Gail Constantine.",
    },
    CastMember {
        name: "Rik Raider",
        aliases: &["Raider"],
        note: "Captain of the Tunnel Drive prototype starship Sokol.",
    },
    CastMember {
        name: "Neve Trimble",
        aliases: &["Trimble"],
        note: "Commander aboard the Sokol, serving under Captain Raider.",
    },
    CastMember {
        name: "Gail Constantine",
        aliases: &["Constantine", "Gail"],
        note: "Federation senator on Selene, arguing for Outer Rim defences. Devon's mother.",
    },
    CastMember {
        name: "Kantor",
        aliases: &[],
        note: "Chief Strategist of the Votainion Empire, pursuing the Federation after an \
               encounter over Ocherva.",
    },
    CastMember {
        name: "Varco",
        aliases: &[],
        note: "Votainion commander aboard the flagship VCF Krestor, serving under Kantor.",
    },
    CastMember {
        name: "Nykostra",
        aliases: &[],
        note: "Empress of the Votainion Empire.",
    },
    CastMember {
        name: "Morgan Blud",
        aliases: &["Blud"],
        note: "Pirate captain. Responsible for the death of Devon's husband.",
    },
    CastMember {
        name: "Sasha",
        aliases: &[],
        note: "Captain Blud's partner in the raid on the SS Kelley.",
    },
    CastMember {
        name: "Ganner",
        aliases: &[],
        note: "Federation admiral, pressing for the Starforgers to be built up.",
    },
    CastMember {
        name: "Seth",
        aliases: &[],
        note: "Stellar Ranger on Ocherva; takes command of Company H when Devon leaves.",
    },
    CastMember {
        name: "Gareth",
        aliases: &[],
        note: "Viewpoint character in the later chapters of the book.",
    },
    CastMember {
        name: "Hap",
        aliases: &[],
        note: "Devon's wingman, killed by an alien fighter over Ocherva.",
    },
    CastMember {
        name: "Thirty-seven",
        aliases: &["Slim"],
        note: "Android attached to the Rangers on Ocherva; one of Senator Constantine's \
               Silicant operatives.",
    },
    CastMember {
        name: "Eighty-eight",
        aliases: &[],
        note: "Silicant operative working for Senator Constantine.",
    },
    CastMember {
        name: "Aven",
        aliases: &[],
        note: "Ranger Control officer on Ocherva.",
    },
    CastMember {
        name: "Hoque",
        aliases: &[],
        note: "Federation senator opposing funding for Outer Rim defences.",
    },
];

// Id ranges, kept clear of the existing rows (items and contents both run 1..=37).
const TAG_ID: u64 = 500;
const SYNOPSIS_CONTENT_BASE: u64 = 600;
const CAST_ITEM_BASE: u64 = 700;
const CAST_CONTENT_BASE: u64 = 800;
const STORY_BIBLE_BINDER_ID: u64 = 900;

fn main() -> anyhow::Result<()> {
    let mut args = std::env::args().skip(1);
    let bundle_path = args.next().expect("usage: <bundle.skrib> <synopses.json>");
    let meta_path = args.next().expect("usage: <bundle.skrib> <synopses.json>");

    let metas: Vec<ChapterMeta> = serde_json::from_str(&std::fs::read_to_string(&meta_path)?)?;
    let mut bundle = read_bundle(&bundle_path)?;

    // Everything the prose said, before anything is touched. Compared again at the end.
    let prose_before = snapshot_prose(&bundle);

    let now = bundle.manifest.work.updated_at.clone();

    // ── the discoverable tag ──────────────────────────────────────────────────
    // `discoverable` is what puts an item's title and aliases into the mention index; a
    // story-bible note without it is invisible to the scan and to the point-of-view picker.
    bundle.tags.push(BinderTagFile {
        file_id: TAG_ID,
        created_at: now.clone(),
        updated_at: now.clone(),
        name: "Character".to_string(),
        color: "#4477aa".to_string(),
        details: "Story-bible entry for a person in the book.".to_string(),
        discoverable: true,
    });

    // ── story-bible notes, in their own binder ────────────────────────────────
    // A second binder rather than notes appended to the manuscript: binders exist to
    // separate the book from the material about the book, and burying character notes in
    // the manuscript stream would put them in the analysis scope.
    let manuscript = bundle
        .binders
        .first()
        .expect("the fixture has a manuscript binder");
    let binder_dir_index = bundle.binders.len();
    let mut cast_items = Vec::new();
    // Keyed by canonical name **and** by every alias: the reading pass names people the way
    // the prose does ("Devon"), while the story bible titles them fully ("Devon Ardel"). A
    // canonical-only map silently matched neither the points of view nor the cast links, and
    // the only symptom was a lower count in this tool's own summary line.
    let mut name_to_item: BTreeMap<&str, u64> = BTreeMap::new();

    for (i, member) in CAST.iter().enumerate() {
        let item_id = CAST_ITEM_BASE + i as u64;
        let content_id = CAST_CONTENT_BASE + i as u64;
        name_to_item.insert(member.name, item_id);
        for alias in member.aliases {
            name_to_item.insert(alias, item_id);
        }

        let mut item = manuscript.items[1].item.clone();
        item.file_id = item_id;
        item.uid = uuid::Uuid::new_v4();
        item.title = member.name.to_string();
        item.sub_title = String::new();
        item.label = String::new();
        item.role = common::entities::BinderItemRole::Item;
        item.sub_role = common::entities::BinderItemSubRole::Note;
        item.indent = 0;
        item.activated = true;
        item.is_favorite = false;
        item.is_exportable = false;
        item.aliases = member.aliases.iter().map(|s| s.to_string()).collect();
        item.inline_contents = Vec::new();
        item.reference_ids = Vec::new();
        item.point_of_view_ids = Vec::new();
        item.tag_ids = vec![TAG_ID];
        item.prose_refs = vec![ProseRef {
            file_id: content_id,
            created_at: now.clone(),
            updated_at: now.clone(),
            activated: true,
            role: common::entities::ContentRole::NoteText,
            path: skrib_format::prose_relpath(
                &skrib_format::binder_dir_name(binder_dir_index, "Story bible"),
                &skrib_format::prose_file_name(
                    content_id,
                    &item.title,
                    &common::entities::ContentRole::NoteText,
                )
                .expect("NoteText is a prose role"),
            ),
        }];

        let mut prose = BTreeMap::new();
        prose.insert(content_id, member.note.to_string());
        // No comments on a generated cast note: this fixture writes story-bible
        // entries, and a comment is something a reader leaves on prose.
        cast_items.push(BundledItem {
            item,
            prose,
            comments: Default::default(),
        });
    }

    let mut story_binder = manuscript.binder.clone();
    story_binder.file_id = STORY_BIBLE_BINDER_ID;
    story_binder.uid = uuid::Uuid::new_v4();
    story_binder.name = "Story bible".to_string();
    story_binder.item_order = cast_items.iter().map(|b| b.item.file_id).collect();

    // ── synopses and point of view on the manuscript ──────────────────────────
    let by_id: BTreeMap<u64, &ChapterMeta> = metas.iter().map(|m| (m.file_id, m)).collect();
    let manuscript_dir = skrib_format::binder_dir_name(0, &bundle.binders[0].binder.name);
    let mut synopses_added = 0usize;
    let mut pov_added = 0usize;

    for bundled in &mut bundle.binders[0].items {
        let Some(meta) = by_id.get(&bundled.item.file_id) else {
            continue;
        };
        let content_id = SYNOPSIS_CONTENT_BASE + bundled.item.file_id;
        bundled.item.prose_refs.push(ProseRef {
            file_id: content_id,
            created_at: now.clone(),
            updated_at: now.clone(),
            activated: true,
            role: common::entities::ContentRole::SynopsisText,
            path: skrib_format::prose_relpath(
                &manuscript_dir,
                &skrib_format::prose_file_name(
                    content_id,
                    &bundled.item.title,
                    &common::entities::ContentRole::SynopsisText,
                )
                .expect("SynopsisText is a prose role"),
            ),
        });
        bundled.prose.insert(content_id, meta.synopsis.clone());
        synopses_added += 1;

        // Point of view only where the reading pass was confident. Two dozen chapters of
        // this book are genuinely multi-thread, and inventing a viewpoint for them would be
        // worse than leaving them unassigned — the unassigned bucket exists for exactly
        // this, and a fixture that never exercises it would hide it.
        if let Some(&target) = name_to_item.get(meta.pov_character.as_str()) {
            bundled.item.point_of_view_ids = vec![target];
            pov_added += 1;
        }

        // The cast a chapter names, as confirmed references — the same relationship the
        // Inspector's Cast section pins by hand.
        bundled.item.reference_ids = meta
            .characters_present
            .iter()
            .filter_map(|n| name_to_item.get(n.as_str()).copied())
            .collect();
    }

    bundle.binders.push(BundledBinder {
        binder: story_binder,
        items: cast_items,
    });
    bundle.manifest.binder_order.push(STORY_BIBLE_BINDER_ID);
    bundle.manifest.work.tag_ids.push(TAG_ID);

    // ── the line this tool must not cross ─────────────────────────────────────
    let prose_after = snapshot_prose(&bundle);
    for (key, before) in &prose_before {
        let after = prose_after.get(key).expect("a scene disappeared");
        assert_eq!(
            before, after,
            "the author's prose was modified for {key:?} — this tool may only add metadata"
        );
    }

    write_bundle(&bundle_path, SkribShape::ZipFile, &bundle)?;
    println!(
        "enriched {bundle_path}: {synopses_added} synopses, {pov_added} points of view, \
         {} story-bible notes, 1 discoverable tag",
        CAST.len()
    );
    Ok(())
}

/// Every existing scene's text, keyed by (item id, content id).
fn snapshot_prose(bundle: &skrib_format::WorkBundle) -> BTreeMap<(u64, u64), String> {
    let mut out = BTreeMap::new();
    for b in &bundle.binders {
        for item in &b.items {
            for r in &item.item.prose_refs {
                if r.role == common::entities::ContentRole::SceneText
                    && let Some(text) = item.prose.get(&r.file_id)
                {
                    out.insert((item.item.file_id, r.file_id), text.clone());
                }
            }
        }
    }
    out
}
