// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

// One read-only pass over the open Work, finding where the story bible is mentioned in the
// prose. Every item carrying a **discoverable** tag contributes its title and aliases; those
// names are matched against every activated Scene/Synopsis/Note, and each surviving hit is
// reported as `(owner = the item whose prose it was found in, target = the item named)`.
//
// A scene's **roster** is the hits whose owner is that scene; an item's **backlinks** are the
// hits whose target is that item. One list, two views — the direction is the reader's, not
// the data's.
//
// **This writes nothing.** Not the entities it reads, not a dirty flag, not an undo entry:
// `undoable: false`, `read_only: true`, and a `QueryUnitOfWork`. A suggestion only becomes
// persisted when the writer pins it, which goes through the `references` relationship from
// the UI — never from here. `scan_writes_no_entities` guards that.
//
// Modelled on `count_words_uc`: same `gather`, same `TreeReader` impl, same long-operation
// shape. The real logic is `fold_mentions`, kept free of the store so it is unit-testable.
use crate::MentionScanResultDto;
use crate::dtos::{MentionEntity, MentionHit, MentionHits, MentionTable};
use anyhow::{Result, anyhow};
use common::database::QueryUnitOfWork;
use common::entities::{Binder, BinderItem, BinderTag, Content, ContentRole, Work};
use common::long_operation::{LongOperation, OperationProgress};
use common::direct_access::binder::BinderRelationshipField;
use common::direct_access::binder_item::BinderItemRelationshipField;
use common::direct_access::work::WorkRelationshipField;
use common::types::EntityId;
use skrib_format::{TreeReader, gather};
use skribisto_model::language;
use skribisto_model::mentions::{self, DiscoverableEntity};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use text_document::matching::FoldLocale;
use text_document::{DjotImportOptions, djot_to_plain_text};

pub trait ScanMentionsUnitOfWorkFactoryTrait: Send + Sync {
    fn create(&self) -> Box<dyn ScanMentionsUnitOfWorkTrait>;
}

// Read-only unit of work — the same read surface as `count_words` (it drives the same
// `gather`). The identical macro set must appear on the impl in
// ../units_of_work/scan_mentions_uow.rs.
#[macros::uow_action(entity = "Work", action = "GetAllRO")]
#[macros::uow_action(entity = "Work", action = "GetRelationshipRO")]
#[macros::uow_action(entity = "Binder", action = "GetMultiRO")]
#[macros::uow_action(entity = "Binder", action = "GetRelationshipRO")]
#[macros::uow_action(entity = "BinderItem", action = "GetMultiRO")]
#[macros::uow_action(entity = "BinderItem", action = "GetRelationshipRO")]
#[macros::uow_action(entity = "BinderTag", action = "GetMultiRO")]
#[macros::uow_action(entity = "Content", action = "GetMultiRO")]
pub trait ScanMentionsUnitOfWorkTrait: QueryUnitOfWork + Send + Sync {
    fn publish_scan_mentions_event(&self, ids: Vec<EntityId>, data: Option<String>);
}

// Map the generated read methods onto the shared `TreeReader`. The defaulted methods
// (work_info / trash / dict / pace / snapshot) are omitted — a scan reads none of them.
impl<'a> TreeReader for dyn ScanMentionsUnitOfWorkTrait + 'a {
    fn all_work(&self) -> Result<Vec<Work>> {
        self.get_all_work()
    }
    fn work_rel(&self, id: &EntityId, field: &WorkRelationshipField) -> Result<Vec<EntityId>> {
        self.get_work_relationship(id, field)
    }
    fn binder_multi(&self, ids: &[EntityId]) -> Result<Vec<Option<Binder>>> {
        self.get_binder_multi(ids)
    }
    fn binder_rel(&self, id: &EntityId, field: &BinderRelationshipField) -> Result<Vec<EntityId>> {
        self.get_binder_relationship(id, field)
    }
    fn item_multi(&self, ids: &[EntityId]) -> Result<Vec<Option<BinderItem>>> {
        self.get_binder_item_multi(ids)
    }
    fn item_rel(
        &self,
        id: &EntityId,
        field: &BinderItemRelationshipField,
    ) -> Result<Vec<EntityId>> {
        self.get_binder_item_relationship(id, field)
    }
    fn tag_multi(&self, ids: &[EntityId]) -> Result<Vec<Option<BinderTag>>> {
        self.get_binder_tag_multi(ids)
    }
    fn content_multi(&self, ids: &[EntityId]) -> Result<Vec<Option<Content>>> {
        self.get_content_multi(ids)
    }
}

pub struct ScanMentionsUseCase {
    uow_factory: Box<dyn ScanMentionsUnitOfWorkFactoryTrait>,
}

impl ScanMentionsUseCase {
    pub fn new(uow_factory: Box<dyn ScanMentionsUnitOfWorkFactoryTrait>) -> Self {
        ScanMentionsUseCase { uow_factory }
    }
}

impl LongOperation for ScanMentionsUseCase {
    type Output = MentionScanResultDto;

    fn execute(
        &self,
        progress_callback: Box<dyn Fn(OperationProgress) + Send>,
        cancel_flag: Arc<AtomicBool>,
    ) -> Result<Self::Output> {
        use std::sync::atomic::Ordering;

        progress_callback(OperationProgress::new(0.0, Some("Starting...".to_string())));

        let uow = self.uow_factory.create();
        uow.begin_transaction()?;
        let scanned = run_scan(uow.as_ref(), &*progress_callback, &cancel_flag);
        if cancel_flag.load(Ordering::Relaxed) {
            uow.end_transaction()?;
            return Err(anyhow!("Operation was cancelled"));
        }
        uow.end_transaction()?;
        let (work_id, dto) = scanned?;

        uow.publish_scan_mentions_event(vec![work_id], None);
        progress_callback(OperationProgress::new(100.0, Some("completed".to_string())));
        Ok(dto)
    }
}

/// Which content roles carry prose a name can be mentioned in.
///
/// Synopses and notes count, not just scene text: a synopsis is where a writer records who a
/// scene is *about* before writing it, and note↔note links are half of what the story bible
/// is for.
fn is_prose(role: &ContentRole) -> bool {
    matches!(
        role,
        ContentRole::SceneText | ContentRole::SynopsisText | ContentRole::NoteText
    )
}

fn run_scan(
    uow: &dyn ScanMentionsUnitOfWorkTrait,
    progress: &(dyn Fn(OperationProgress) + Send),
    cancel: &AtomicBool,
) -> Result<(EntityId, MentionScanResultDto)> {
    let g = gather(uow, progress, cancel)?;
    let work_id = g.work.id;

    // Which tags mark story-bible material.
    let discoverable: HashSet<EntityId> = g
        .tags
        .iter()
        .filter(|t| t.discoverable)
        .map(|t| t.id)
        .collect();

    // The alias table: every live item carrying one of those tags. Trashed items are
    // excluded here rather than filtered later — a deleted character must not appear in a
    // roster at all, and excluding them at the source also shrinks the matching work.
    let mut table: Vec<DiscoverableEntity> = Vec::new();
    let mut titles: HashMap<EntityId, String> = HashMap::new();
    for b in &g.binders {
        for iwc in &b.items {
            let it = &iwc.item;
            if !it.activated || !it.tags.iter().any(|t| discoverable.contains(t)) {
                continue;
            }
            titles.insert(it.id, it.title.clone());
            table.push(DiscoverableEntity {
                id: it.id,
                title: it.title.clone(),
                aliases: it.aliases.clone(),
            });
        }
    }
    let fingerprint = mentions::fingerprint_alias_table(&table);

    // Per-item language, for folding. The work's own language is the fallback; this is the
    // house resolver, already used by search and word count.
    let all_items: Vec<BinderItem> = g
        .binders
        .iter()
        .flat_map(|b| b.items.iter().map(|i| i.item.clone()))
        .collect();
    let mut lang: HashMap<EntityId, Vec<String>> = HashMap::new();
    language::tags_in_binder(&g.work.dict_language, &all_items, &mut lang);

    // (owner, target) -> the hits found, so repeated mentions of one name collapse to one
    // row carrying a count rather than N rows.
    let mut rows: HashMap<(EntityId, EntityId), Row> = HashMap::new();

    for b in &g.binders {
        for iwc in &b.items {
            let owner = &iwc.item;
            if !owner.activated {
                continue;
            }
            let locale = FoldLocale::from_tag(language::primary(
                lang.get(&owner.id).map(Vec::as_slice).unwrap_or_default(),
            ));
            for c in iwc.contents.iter().filter(|c| c.activated && is_prose(&c.role)) {
                if c.data.is_empty() {
                    continue;
                }
                // Plain text, not Djot: matching raw markup finds names inside link syntax
                // and attribute blocks, which are not mentions.
                let plain = djot_to_plain_text(&c.data, &DjotImportOptions::default());
                let hits = mentions::cached_mentions(&plain, &table, fingerprint, locale);
                for h in hits.iter() {
                    // An item mentioning its own name is not a mention of itself.
                    if h.entity_id == owner.id {
                        continue;
                    }
                    let entity = table.iter().find(|e| e.id == h.entity_id);
                    let entry = rows.entry((owner.id, h.entity_id)).or_insert_with(Row::default);
                    entry.hit_count += 1;
                    // Keep the first hit's evidence and name: cut once, here, where the
                    // prose is already in hand. Nothing downstream has access to another
                    // item's text, so this is the only place it can be done at all.
                    if entry.evidence.is_empty() {
                        entry.evidence = mentions::evidence_sentence(&plain, h);
                        entry.matched_name = entity
                            .map(|e| h.matched_name(e).to_string())
                            .unwrap_or_default();
                        entry.is_title_match = h.is_title_match;
                    }
                }
            }
        }
    }

    // Persisted references join the same list. A reference the writer pinned by hand must
    // appear even when the name is never written — a scene told in deep POV may name nobody
    // — so this is a union, not a filter over the textual hits.
    for b in &g.binders {
        for iwc in &b.items {
            let owner = &iwc.item;
            if !owner.activated {
                continue;
            }
            for target in &owner.references {
                if !titles.contains_key(target) {
                    continue;
                }
                rows.entry((owner.id, *target)).or_insert_with(Row::default).is_confirmed = true;
            }
        }
    }

    let mut hits: Vec<MentionHit> = rows
        .into_iter()
        .map(|((owner_id, target_id), r)| MentionHit::Found {
            owner_id,
            target_id,
            title: titles.get(&target_id).cloned().unwrap_or_default(),
            matched_name: if r.matched_name.is_empty() {
                titles.get(&target_id).cloned().unwrap_or_default()
            } else {
                r.matched_name
            },
            is_title_match: r.is_title_match,
            hit_count: r.hit_count,
            is_confirmed: r.is_confirmed,
            evidence: r.evidence,
        })
        .collect();
    // Deterministic order so the UI does not reshuffle between identical scans.
    hits.sort_by(|a, b| key_of(a).cmp(&key_of(b)));

    // The table rides back with the hits so the UI can rescan the focused item's own prose
    // against exactly the same names, live, without a second full pass.
    let entities: Vec<MentionEntity> = table
        .iter()
        .map(|e| MentionEntity::Discoverable {
            id: e.id,
            title: e.title.clone(),
            aliases: e.aliases.clone(),
        })
        .collect();

    Ok((
        work_id,
        MentionScanResultDto {
            row: MentionHit::Empty,
            hits: MentionHits::Found(hits),
            entity: MentionEntity::Empty,
            table: MentionTable::Entities(entities),
        },
    ))
}

#[derive(Default)]
struct Row {
    hit_count: i64,
    is_confirmed: bool,
    is_title_match: bool,
    matched_name: String,
    evidence: String,
}

fn key_of(h: &MentionHit) -> (EntityId, EntityId) {
    match h {
        MentionHit::Found {
            owner_id,
            target_id,
            ..
        } => (*owner_id, *target_id),
        MentionHit::Empty => (0, 0),
    }
}
