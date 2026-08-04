// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The mention index the Inspector reads: who is mentioned where, across the whole work.
//!
//! Owns the last completed `scan_mentions` result and hands out per-item views of it. One
//! instance, shared app-wide through `app_state` — every roster and backlink list in the app
//! resolves through this, so a second instance would mean a second scan and two answers.
//!
//! Shaped like [`ProgressRecorder`](super::ProgressRecorder): fire a long operation, guard
//! against overlap and against a project switch landing one project's answer on another,
//! throttle the save path.
//!
//! ## Two differences from that template, both deliberate
//!
//! **It scans on load, not only on save.** `ProgressRecorder` explicitly does *not* fire on
//! `LoadWork`, and is right not to: a historical word-count point should not be recorded for
//! a project that was merely opened. A mention index is not a history — it is live state the
//! Inspector must show something for the moment a project opens, or the roster is empty until
//! the writer happens to save.
//!
//! **The focused item is scanned live, in front of the batch.** A full-work scan is the only
//! way to know who mentions *this note* (the backlink direction reads every other item's
//! prose). But the roster of the scene being written depends on one string the app already
//! has in memory, and making the writer save to see a character appear would defeat the point
//! — the feature exists so the story bible fills itself in *while* you write. So
//! [`MentionIndex::roster_for`] rescans the focused item's own prose on demand, against the alias table the
//! last batch built, and layers those hits over the batch's. That costs one fold of one
//! scene, memoised by [`skribisto_model::mentions::cached_mentions`] on the prose itself, so
//! an idle Inspector repeats no work.

use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::rc::Rc;
use std::time::{Duration, Instant};

use bastyde::prelude::Signal;
use bastyde::text_document::matching::FoldLocale;
use frontend::AppContext;
use frontend::commands::mention_management_commands;
use frontend::common::event::Event;
use frontend::mention_management::{
    MentionEntity, MentionHit, MentionHits, MentionTable, ScanMentionsDto,
};
use skribisto_model::mentions::{self, DiscoverableEntity};

use crate::app_ids::AppIds;

use super::long_op::event_id;

/// Don't rescan more than once a minute on the save path — autosave fires every few seconds,
/// and the batch half of the index only needs to be roughly current (the half that must be
/// exact, the focused item's own roster, is rescanned live instead).
const THROTTLE: Duration = Duration::from_secs(60);

/// One row of the index, as the Inspector shows it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MentionRow {
    /// The item whose prose the name was found in.
    pub owner_id: u64,
    /// The story-bible item that was named.
    pub target_id: u64,
    pub title: String,
    /// Which of the target's names matched — its title, or one of its aliases.
    pub matched_name: String,
    pub is_title_match: bool,
    pub hit_count: i64,
    /// A persisted `references` entry, as opposed to a suggestion the scan derived.
    pub is_confirmed: bool,
    /// The sentence the first hit sits in. Empty for a confirmed reference whose name is
    /// never actually written.
    pub evidence: String,
}

struct Inner {
    app_ctx: Rc<AppContext>,
    ids: AppIds,
    /// The in-flight `scan_mentions` operation id, if one is running.
    active: RefCell<Option<String>>,
    /// The work this scan was fired for — a close or switch mid-scan must not land one
    /// project's index on another.
    fired_for: Cell<Option<u64>>,
    last_fire: Cell<Option<Instant>>,
    /// The last completed batch, keyed by owner for the roster direction.
    by_owner: RefCell<HashMap<u64, Vec<MentionRow>>>,
    /// …and by target for the backlink direction. Two indexes over one list, built once per
    /// scan so neither direction pays a scan of the whole vector per lookup.
    by_target: RefCell<HashMap<u64, Vec<MentionRow>>>,
    /// The alias table the last batch used, so the focused item can be rescanned against the
    /// same names without another full pass.
    table: RefCell<Vec<DiscoverableEntity>>,
    /// Bumped whenever a scan completes, so models rebuild.
    version: Signal<u64>,
}

/// Cloneable handle; one instance is shared app-wide via `app_state`.
#[derive(Clone)]
pub struct MentionIndex {
    inner: Rc<Inner>,
}

impl MentionIndex {
    pub fn new(app_ctx: Rc<AppContext>, ids: AppIds) -> Self {
        Self {
            inner: Rc::new(Inner {
                app_ctx,
                ids,
                active: RefCell::new(None),
                fired_for: Cell::new(None),
                last_fire: Cell::new(None),
                by_owner: RefCell::new(HashMap::new()),
                by_target: RefCell::new(HashMap::new()),
                table: RefCell::new(Vec::new()),
                version: Signal::new(0),
            }),
        }
    }

    /// Bumped when a scan lands. Bind at `Rebuild` to refresh a roster.
    pub fn changed_signal(&self) -> Signal<u64> {
        self.inner.version.clone()
    }

    /// Scan now, skipping only if one is already running.
    ///
    /// Used for the events that change *who is discoverable* — a tag gaining or losing its
    /// story-bible flag, an item's tags or aliases being edited. Those are rare, deliberate
    /// actions, and they are exactly the moment a writer expects the roster to appear, so
    /// they are not throttled: the save-path throttle below exists for autosave storms, and
    /// applying it here meant marking a tag discoverable did nothing visible for a minute.
    pub fn rescan(&self) {
        if self.inner.active.borrow().is_some() {
            return;
        }
        self.fire();
    }

    /// Scan on a save, unless one is running or we scanned within [`THROTTLE`].
    pub fn rescan_throttled(&self) {
        if self.inner.active.borrow().is_some() {
            return;
        }
        if let Some(last) = self.inner.last_fire.get()
            && last.elapsed() < THROTTLE
        {
            return;
        }
        self.fire();
    }

    fn fire(&self) {
        // No open project → nothing to scan (this also keeps the mock build inert, since it
        // never seeds a real Work).
        let Some(work_id) = self.inner.ids.work_id.get() else {
            return;
        };
        let dto = ScanMentionsDto { work_id };
        if let Ok(op_id) = mention_management_commands::scan_mentions(&self.inner.app_ctx, &dto) {
            *self.inner.active.borrow_mut() = Some(op_id);
            self.inner.fired_for.set(Some(work_id));
            self.inner.last_fire.set(Some(Instant::now()));
        }
    }

    /// A long operation completed — if it is our in-flight scan, take its result.
    pub fn on_completed(&self, event: &Event) {
        let Some(op_id) = self.take_if_ours(event) else {
            return;
        };
        // Session guard: only accept while the same project is still open.
        let fired_for = self.inner.fired_for.get();
        if fired_for.is_none() || self.inner.ids.work_id.get() != fired_for {
            return;
        }
        let Ok(Some(res)) =
            mention_management_commands::get_scan_mentions_result(&self.inner.app_ctx, &op_id)
        else {
            return;
        };

        let mut by_owner: HashMap<u64, Vec<MentionRow>> = HashMap::new();
        let mut by_target: HashMap<u64, Vec<MentionRow>> = HashMap::new();
        if let MentionHits::Found(hits) = res.hits {
            for h in hits {
                let MentionHit::Found {
                    owner_id,
                    target_id,
                    title,
                    matched_name,
                    is_title_match,
                    hit_count,
                    is_confirmed,
                    evidence,
                } = h
                else {
                    continue;
                };
                let row = MentionRow {
                    owner_id,
                    target_id,
                    title,
                    matched_name,
                    is_title_match,
                    hit_count,
                    is_confirmed,
                    evidence,
                };
                by_owner.entry(owner_id).or_default().push(row.clone());
                by_target.entry(target_id).or_default().push(row);
            }
        }
        // The alias table the scan used, so the live rescan of the focused item matches
        // against exactly the same names. Without this the two halves of the index would
        // disagree about who is discoverable.
        if let MentionTable::Entities(entities) = res.table {
            *self.inner.table.borrow_mut() = entities
                .into_iter()
                .filter_map(|e| match e {
                    MentionEntity::Discoverable { id, title, aliases } => {
                        Some(DiscoverableEntity { id, title, aliases })
                    }
                    MentionEntity::Empty => None,
                })
                .collect();
        }
        *self.inner.by_owner.borrow_mut() = by_owner;
        *self.inner.by_target.borrow_mut() = by_target;
        self.inner.version.set(self.inner.version.get() + 1);
    }

    /// A long operation failed or was cancelled — clear our marker if it was ours.
    pub fn on_failed_or_cancelled(&self, event: &Event) {
        let _ = self.take_if_ours(event);
    }

    /// Discoverable entities the last batch built — the story-bible catalogue for Add-to-cast
    /// and write-path validation. Empty until the first scan lands.
    pub fn discoverable_table(&self) -> Vec<DiscoverableEntity> {
        self.inner.table.borrow().clone()
    }

    /// Whether `target_id` is currently a valid cast target (present in the alias table).
    pub fn is_valid_cast_target(&self, target_id: u64) -> bool {
        self.inner.table.borrow().iter().any(|e| e.id == target_id)
    }

    /// Keep only discoverable, non-self targets, preserving order and dropping duplicates.
    ///
    /// When the alias table is still empty (scan not landed yet), only self and
    /// duplicates are dropped — stripping against an empty table would wipe a
    /// writer's cast the first time they pin before the first scan returns.
    pub fn filter_cast_targets(&self, owner_id: u64, ids: &[u64]) -> Vec<u64> {
        let table = self.inner.table.borrow();
        let mut out = Vec::with_capacity(ids.len());
        for &id in ids {
            if id == owner_id {
                continue;
            }
            if !table.is_empty() && !table.iter().any(|e| e.id == id) {
                continue;
            }
            if !out.contains(&id) {
                out.push(id);
            }
        }
        out
    }

    /// References-first cast for the focused item: confirmed pins first, then scan
    /// suggestions (batch + optional chapter-union owners + optional live prose).
    ///
    /// `live_prose` must be supplied only from a **debounced** path — never from the
    /// editor key handler. When `None`, suggestions come from the batch index alone.
    /// `is_confirmed` is always taken from `confirmed` (the focused item's `references`),
    /// never from a child owner's pins when `extra_owners` is used for a chapter union.
    pub fn cast_for(
        &self,
        owner_id: u64,
        live_prose: Option<&str>,
        confirmed: &[u64],
        extra_owners: &[u64],
    ) -> Vec<MentionRow> {
        let table = self.inner.table.borrow();
        // When the table is empty, still surface raw confirmed ids (titles may be
        // blank until the first scan); never drop the writer's pins.
        let confirmed_set: HashMap<u64, ()> = confirmed
            .iter()
            .copied()
            .filter(|&id| id != owner_id && (table.is_empty() || table.iter().any(|e| e.id == id)))
            .map(|id| (id, ()))
            .collect();

        // Suggestions: batch for owner (unless non-empty live prose replaces it) + extra
        // owners (chapter union). Live prose is never computed here — the caller debounces
        // it. Empty `Some("")` must not drop the owner batch: that is the same as no live
        // overlay yet (new scene, cleared body).
        let mut by_target: HashMap<u64, MentionRow> = HashMap::new();
        let live = live_prose.filter(|p| !p.is_empty());

        let merge_suggestion = |map: &mut HashMap<u64, MentionRow>, row: MentionRow| {
            if row.target_id == owner_id {
                return;
            }
            if !table.iter().any(|e| e.id == row.target_id) {
                return;
            }
            match map.get_mut(&row.target_id) {
                Some(existing) => {
                    existing.hit_count = existing.hit_count.max(row.hit_count);
                    if existing.evidence.is_empty() && !row.evidence.is_empty() {
                        existing.evidence = row.evidence;
                        existing.matched_name = row.matched_name;
                        existing.is_title_match = row.is_title_match;
                    } else if row.is_title_match && !existing.is_title_match {
                        existing.is_title_match = true;
                        existing.matched_name = row.matched_name;
                    }
                }
                None => {
                    map.insert(
                        row.target_id,
                        MentionRow {
                            // Cast rows are owned by the focused item for pin/unpin.
                            owner_id,
                            is_confirmed: false,
                            ..row
                        },
                    );
                }
            }
        };

        {
            let by_owner = self.inner.by_owner.borrow();
            // Non-empty live prose replaces the owner's batch half (same as
            // `roster_for`); still merge extra_owners from the batch only.
            let batch_owners: Box<dyn Iterator<Item = u64>> = if live.is_some() {
                Box::new(extra_owners.iter().copied())
            } else {
                Box::new(std::iter::once(owner_id).chain(extra_owners.iter().copied()))
            };
            for id in batch_owners {
                if let Some(rows) = by_owner.get(&id) {
                    for row in rows {
                        // Child batch `is_confirmed` is ignored — only `confirmed` below matters.
                        merge_suggestion(&mut by_target, row.clone());
                    }
                }
            }
        }

        if let Some(prose) = live
            && !table.is_empty()
        {
            let fingerprint = mentions::fingerprint_alias_table(&table);
            let hits = mentions::cached_mentions(prose, &table, fingerprint, FoldLocale::default());
            let mut live_counts: HashMap<u64, MentionRow> = HashMap::new();
            for h in hits.iter() {
                if h.entity_id == owner_id {
                    continue;
                }
                let Some(entity) = table.iter().find(|e| e.id == h.entity_id) else {
                    continue;
                };
                let row = live_counts
                    .entry(h.entity_id)
                    .or_insert_with(|| MentionRow {
                        owner_id,
                        target_id: h.entity_id,
                        title: entity.title.clone(),
                        matched_name: h.matched_name(entity).to_string(),
                        is_title_match: h.is_title_match,
                        hit_count: 0,
                        is_confirmed: false,
                        evidence: mentions::evidence_sentence(prose, h),
                    });
                row.hit_count += 1;
            }
            for row in live_counts.into_values() {
                merge_suggestion(&mut by_target, row);
            }
        }

        // Apply confirmed flags; inject pure-planning pins with no prose hits.
        for &target_id in confirmed_set.keys() {
            match by_target.get_mut(&target_id) {
                Some(row) => row.is_confirmed = true,
                None => {
                    let title = table
                        .iter()
                        .find(|e| e.id == target_id)
                        .map(|e| e.title.clone())
                        .unwrap_or_default();
                    by_target.insert(
                        target_id,
                        MentionRow {
                            owner_id,
                            target_id,
                            title: title.clone(),
                            matched_name: title,
                            is_title_match: true,
                            hit_count: 0,
                            is_confirmed: true,
                            evidence: String::new(),
                        },
                    );
                }
            }
        }

        // Drop suggestions that somehow kept a child-confirmed flag without being in
        // `confirmed` — merge_suggestion already forces false, but be explicit.
        for row in by_target.values_mut() {
            row.is_confirmed = confirmed_set.contains_key(&row.target_id);
        }

        sorted(by_target.into_values().collect())
    }

    /// Who this item mentions — the batch's answer, or a live rescan of `prose` when the
    /// caller has the item's text in hand (it is the focused one, so its prose is already
    /// loaded and may be newer than the last batch).
    pub fn roster_for(&self, item_id: u64, prose: Option<&str>) -> Vec<MentionRow> {
        let batch = self
            .inner
            .by_owner
            .borrow()
            .get(&item_id)
            .cloned()
            .unwrap_or_default();
        let Some(prose) = prose else {
            return sorted(batch);
        };
        let table = self.inner.table.borrow();
        if table.is_empty() {
            return sorted(batch);
        }

        // Live hits for the focused item, from prose the batch may not have seen.
        let fingerprint = mentions::fingerprint_alias_table(&table);
        let hits = mentions::cached_mentions(prose, &table, fingerprint, FoldLocale::default());
        let mut live: HashMap<u64, MentionRow> = HashMap::new();
        for h in hits.iter() {
            if h.entity_id == item_id {
                continue;
            }
            let Some(entity) = table.iter().find(|e| e.id == h.entity_id) else {
                continue;
            };
            let row = live.entry(h.entity_id).or_insert_with(|| MentionRow {
                owner_id: item_id,
                target_id: h.entity_id,
                title: entity.title.clone(),
                matched_name: h.matched_name(entity).to_string(),
                is_title_match: h.is_title_match,
                hit_count: 0,
                is_confirmed: false,
                evidence: mentions::evidence_sentence(prose, h),
            });
            row.hit_count += 1;
        }

        // The live pass sees the prose but not the `references` table, so carry `is_confirmed`
        // across from the batch. A confirmed row the live pass found no text for must also
        // survive — that is the whole point of pinning something the prose never names.
        for b in batch {
            match live.get_mut(&b.target_id) {
                Some(row) => row.is_confirmed = b.is_confirmed,
                None if b.is_confirmed => {
                    live.insert(b.target_id, b);
                }
                None => {}
            }
        }
        sorted(live.into_values().collect())
    }

    /// Where this item is mentioned. Only the batch can answer this: it means reading every
    /// other item's prose, which the Inspector has no access to.
    pub fn backlinks_for(&self, item_id: u64) -> Vec<MentionRow> {
        sorted(
            self.inner
                .by_target
                .borrow()
                .get(&item_id)
                .cloned()
                .unwrap_or_default(),
        )
    }

    fn take_if_ours(&self, event: &Event) -> Option<String> {
        let ours = {
            let active = self.inner.active.borrow();
            match active.as_deref() {
                Some(id) => event_id(event).as_deref() == Some(id),
                None => false,
            }
        };
        if ours {
            self.inner.active.borrow_mut().take()
        } else {
            None
        }
    }
}

/// Confirmed first, then title matches, then by how often the name appears, then by name so
/// two equal rows never swap places between identical scans.
fn sorted(mut rows: Vec<MentionRow>) -> Vec<MentionRow> {
    rows.sort_by(|a, b| {
        b.is_confirmed
            .cmp(&a.is_confirmed)
            .then(b.is_title_match.cmp(&a.is_title_match))
            .then(b.hit_count.cmp(&a.hit_count))
            .then(a.title.cmp(&b.title))
    });
    rows
}

#[cfg(test)]
mod tests {
    use super::*;

    fn row(target: u64, title: &str, confirmed: bool, title_match: bool, hits: i64) -> MentionRow {
        MentionRow {
            owner_id: 1,
            target_id: target,
            title: title.to_string(),
            matched_name: title.to_string(),
            is_title_match: title_match,
            hit_count: hits,
            is_confirmed: confirmed,
            evidence: String::new(),
        }
    }

    /// A pinned reference is a decision the writer already made; a suggestion is a guess.
    /// The decision goes first, however faint the guess's evidence.
    #[test]
    fn confirmed_rows_sort_above_suggestions() {
        let out = sorted(vec![
            row(2, "Suggested", false, true, 99),
            row(3, "Pinned", true, false, 1),
        ]);
        assert_eq!(out[0].title, "Pinned");
    }

    /// Within a tier: a title match is a stronger signal than an alias, and a name written
    /// nine times is a stronger signal than one written once.
    #[test]
    fn a_title_match_outranks_an_alias_and_then_frequency_decides() {
        let out = sorted(vec![
            row(2, "AliasOften", false, false, 9),
            row(3, "TitleOnce", false, true, 1),
            row(4, "AliasOnce", false, false, 1),
        ]);
        let order: Vec<&str> = out.iter().map(|r| r.title.as_str()).collect();
        assert_eq!(order, vec!["TitleOnce", "AliasOften", "AliasOnce"]);
    }

    /// Two otherwise-equal rows must not swap places between identical scans — a roster that
    /// reshuffles on every save reads as if something changed.
    #[test]
    fn equal_rows_keep_a_stable_order() {
        let a = sorted(vec![
            row(2, "Bea", false, true, 1),
            row(3, "Ada", false, true, 1),
        ]);
        let b = sorted(vec![
            row(3, "Ada", false, true, 1),
            row(2, "Bea", false, true, 1),
        ]);
        assert_eq!(a, b);
    }

    /// Seed a MentionIndex as if a batch scan just landed — table + by_owner only.
    fn seeded_index(
        table: Vec<DiscoverableEntity>,
        by_owner: HashMap<u64, Vec<MentionRow>>,
    ) -> MentionIndex {
        // AppContext is only needed for fire/rescan; cast_for is pure over Inner maps.
        let app_ctx = frontend::AppContext::new();
        let ids = crate::app_ids::AppIds::new();
        let index = MentionIndex::new(std::rc::Rc::new(app_ctx), ids);
        *index.inner.table.borrow_mut() = table;
        *index.inner.by_owner.borrow_mut() = by_owner;
        index
    }

    fn entity(id: u64, title: &str) -> DiscoverableEntity {
        DiscoverableEntity {
            id,
            title: title.to_string(),
            aliases: vec![],
        }
    }

    fn suggestion(owner: u64, target: u64, title: &str, hits: i64, confirmed: bool) -> MentionRow {
        MentionRow {
            owner_id: owner,
            target_id: target,
            title: title.to_string(),
            matched_name: title.to_string(),
            is_title_match: true,
            hit_count: hits,
            is_confirmed: confirmed,
            evidence: if hits > 0 {
                format!("{title} walked in.")
            } else {
                String::new()
            },
        }
    }

    #[test]
    fn cast_for_shows_confirmed_pins_without_prose_hits() {
        let index = seeded_index(
            vec![entity(10, "Elena"), entity(11, "Dock")],
            HashMap::new(),
        );
        let cast = index.cast_for(1, None, &[10], &[]);
        assert_eq!(cast.len(), 1);
        assert!(cast[0].is_confirmed);
        assert_eq!(cast[0].target_id, 10);
        assert_eq!(cast[0].hit_count, 0);
    }

    /// Empty live prose must not wipe the owner's batch suggestions (review issue 2).
    #[test]
    fn cast_for_empty_live_prose_keeps_owner_batch() {
        let mut by_owner = HashMap::new();
        by_owner.insert(1, vec![suggestion(1, 10, "Grace", 2, false)]);
        let index = seeded_index(vec![entity(10, "Grace")], by_owner);
        let cast = index.cast_for(1, Some(""), &[], &[]);
        assert_eq!(
            cast.len(),
            1,
            "batch suggestion must survive empty live prose"
        );
        assert_eq!(cast[0].hit_count, 2);
        assert!(!cast[0].is_confirmed);
    }

    #[test]
    fn cast_for_chapter_union_ignores_child_pins() {
        // Child scene has Grace confirmed on the *scene*; chapter did not pin her.
        let mut by_owner = HashMap::new();
        by_owner.insert(
            2,
            vec![suggestion(2, 10, "Grace", 3, true)], // child-confirmed
        );
        let index = seeded_index(vec![entity(10, "Grace")], by_owner);
        let cast = index.cast_for(1, None, &[], &[2]);
        assert_eq!(cast.len(), 1);
        assert!(
            !cast[0].is_confirmed,
            "chapter cast must not inherit a child's pin"
        );
        assert_eq!(cast[0].hit_count, 3);
    }

    #[test]
    fn cast_for_chapter_pin_is_independent_of_children() {
        let mut by_owner = HashMap::new();
        by_owner.insert(2, vec![suggestion(2, 10, "Grace", 1, false)]);
        let index = seeded_index(vec![entity(10, "Grace"), entity(11, "Will")], by_owner);
        // Chapter pins Will only; Grace is a child suggestion.
        let cast = index.cast_for(1, None, &[11], &[2]);
        assert_eq!(cast.len(), 2);
        assert!(cast[0].is_confirmed && cast[0].target_id == 11);
        assert!(!cast[1].is_confirmed && cast[1].target_id == 10);
    }

    #[test]
    fn filter_cast_targets_drops_self_and_unknown() {
        let index = seeded_index(
            vec![entity(10, "Elena"), entity(11, "Dock")],
            HashMap::new(),
        );
        assert_eq!(
            index.filter_cast_targets(1, &[10, 1, 10, 99, 11]),
            vec![10, 11]
        );
    }
}
