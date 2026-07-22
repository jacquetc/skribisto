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
//! [`roster_for`] rescans the focused item's own prose on demand, against the alias table the
//! last batch built, and layers those hits over the batch's. That costs one fold of one
//! scene, memoised by [`skribisto_model::mentions::cached_mentions`] on the prose itself, so
//! an idle Inspector repeats no work.

use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::rc::Rc;
use std::time::{Duration, Instant};

use bastyde::prelude::Signal;
use frontend::AppContext;
use frontend::commands::mention_management_commands;
use frontend::common::event::Event;
use frontend::mention_management::{MentionEntity, MentionHit, MentionHits, MentionTable, ScanMentionsDto};
use skribisto_model::mentions::{self, DiscoverableEntity};
use bastyde::text_document::matching::FoldLocale;

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
        let a = sorted(vec![row(2, "Bea", false, true, 1), row(3, "Ada", false, true, 1)]);
        let b = sorted(vec![row(3, "Ada", false, true, 1), row(2, "Bea", false, true, 1)]);
        assert_eq!(a, b);
    }
}
