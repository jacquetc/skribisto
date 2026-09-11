// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Reactive list model over the open Work's custom text-replacement lexicon
//! (`TextReplacementRule`) — the "btw → by the way" rules the writer can add,
//! toggled on a per-project basis via `Work.custom_replacement_rules_enabled`
//! (see [`SingleWork`](crate::singles::SingleWork)).
//!
//! The public surface is a `teksilo::data::ListModel<TextReplacementRuleRow>` a
//! `ListView` binds to (the Settings ▸ Text replacements pane), plus a
//! `version` signal for non-`ListView` consumers (the empty-state).
//!
//! Two `#[cfg]`-gated `mod imp` variants share one public surface: the real one
//! reads via `Work.text_replacement_rules` (scoped to *this* window's own open
//! Work — see `load_rows`, never the whole-store
//! `get_all_text_replacement_rule`, which would merge a second, simultaneously
//! open Work's lexicon into this one's list) and stays live on
//! `TextReplacementRule` events + project switches, with writes going through
//! the generated `text_replacement_rule_commands`; the mock one holds a
//! fabricated set and mutates it in place, since `--features mocks` has no real
//! `Work` to own a created rule.
//!
//! Rows are sorted case-insensitively by trigger, mirroring
//! [`work_tags_list_model`](crate::models::work_tags_list_model)'s ordering
//! convention.

/// One lexicon row: a trigger string that expands to a replacement while
/// typing, and whether the rule is currently active. Deactivated rows are kept
/// (not deleted) so a writer can temporarily disable a rule without losing it.
///
/// Declared by the matcher that consumes it
/// ([`skribisto_model::replacement`]) rather than here: this model's job is to
/// *produce* that shape from the store, and two definitions of it would drift.
pub use skribisto_model::replacement::TextReplacementRuleRow;

/// Sort key: case-insensitive, then exact, so equal-fold triggers keep a
/// deterministic order.
pub fn sort_rows(rows: &mut [TextReplacementRuleRow]) {
    rows.sort_by(|a, b| {
        a.trigger
            .to_lowercase()
            .cmp(&b.trigger.to_lowercase())
            .then_with(|| a.trigger.cmp(&b.trigger))
    });
}

/// The comparison key for "is this trigger already taken". Trimmed and
/// lowercased, matching the engine's own case-insensitive trigger matching —
/// two rows differing only in case could never both fire, so the UI treats
/// them as a collision the writer must resolve.
pub fn trigger_key(trigger: &str) -> String {
    trigger.trim().to_lowercase()
}

/// The row whose trigger collides with `candidate`, ignoring case and
/// surrounding space, excluding `exclude` (a rule being edited never collides
/// with itself).
pub fn colliding_trigger(
    rows: &[TextReplacementRuleRow],
    candidate: &str,
    exclude: Option<u64>,
) -> Option<String> {
    let key = trigger_key(candidate);
    if key.is_empty() {
        return None;
    }
    rows.iter()
        .find(|r| Some(r.id) != exclude && trigger_key(&r.trigger) == key)
        .map(|r| r.trigger.clone())
}

#[cfg(not(feature = "mocks"))]
mod imp {
    use std::rc::Rc;

    use teksilo::data::ListModel;
    use teksilo::prelude::*;

    use frontend::AppContext;
    use frontend::commands::{text_replacement_rule_commands, work_commands};
    use frontend::common::direct_access::work::WorkRelationshipField;
    use frontend::common::event::{
        DirectAccessEntity, EntityEvent, Event, Origin, WorkManagementEvent,
    };
    use frontend::direct_access::{CreateTextReplacementRuleDto, UpdateTextReplacementRuleDto};

    use crate::app_ids::AppIds;

    use super::{TextReplacementRuleRow, sort_rows};

    struct Inner {
        model: ListModel<TextReplacementRuleRow>,
        version: Signal<u64>,
        ctx: Rc<AppContext>,
        /// The open Work's own ids — `work_id` scopes every read to *this*
        /// Work's `text_replacement_rules` relationship (see [`load_rows`]); a
        /// second simultaneously-open Work's lexicon must never merge into this
        /// one's list.
        ids: AppIds,
    }

    #[derive(Clone)]
    pub struct TextReplacementRuleListModel {
        inner: Rc<Inner>,
    }

    impl TextReplacementRuleListModel {
        pub fn new(ctx: Rc<AppContext>, ids: AppIds) -> Self {
            let model = ListModel::from_vec(load_rows(&ctx, ids.work_id.get()));
            Self {
                inner: Rc::new(Inner {
                    model,
                    version: Signal::new(0),
                    ctx,
                    ids,
                }),
            }
        }

        /// Subscribe so the list stays live: any `TextReplacementRule`
        /// mutation — from this pane, the editor's typing session, or an
        /// undo/redo — re-reads the set, and a project boundary replaces it
        /// wholesale.
        ///
        /// `TextReplacementRule` entity events carry no `work_id`, only the
        /// changed entities' own ids — but `refresh` always re-derives the list
        /// from this model's own `ids.work_id` (see [`load_rows`]), so a sibling
        /// Work's event only ever costs a harmless, still-correct re-read here,
        /// never a wrong one. `LoadWork`/`NewWork`/`CloseWork` DO carry `work_id`
        /// and are guarded accordingly: a sibling Work's project boundary must
        /// not force a reload of a list that is (before or after) legitimately
        /// empty.
        ///
        /// **The `LoadWork`/`NewWork` arms fall back to the event's own work id,
        /// and `wire` ends in a catch-up.** This model is built with
        /// `WorkSession` and wired from `App::build`, which registers *before*
        /// `wiring::project_events`' lifecycle seed writes `ids.work_id`. Both
        /// answer the same `LoadWork`, and this one runs first, so a bare
        /// `refresh` read `None`, loaded nothing, and was never asked again:
        /// merely opening a project produces no further `TextReplacementRule`
        /// event. The lexicon then stayed empty for the whole session, the
        /// Settings pane showed none of it, and the typing session compiled an
        /// empty engine, so no rule fired — until the writer created one new
        /// rule, whose `Created` event brought every one of them back. That is
        /// how a reader reported it, on 3.0.1, on Linux and macOS alike. Same
        /// bug, same fix, as [`WorkTagsListModel::wire`](crate::models::WorkTagsListModel)
        /// and `WorkStatusesListModel::wire`.
        ///
        /// **Re-subscribes on every call**, like both of those: a `BuildContext`
        /// subscription is scoped to the current build and dropped on the next,
        /// so the one-shot guard this used to carry left the model permanently
        /// deaf after any rebuild while still reporting itself wired.
        pub fn wire(&self, ctx: &mut BuildContext) {
            for ev in [
                EntityEvent::Created,
                EntityEvent::Updated,
                EntityEvent::Removed,
            ] {
                let me = self.clone();
                ctx.subscribe_event(
                    Origin::DirectAccess(DirectAccessEntity::TextReplacementRule(ev)),
                    move |_event: &Event| me.refresh(),
                );
            }
            for wev in [WorkManagementEvent::LoadWork, WorkManagementEvent::NewWork] {
                let me = self.clone();
                ctx.subscribe_event(Origin::WorkManagement(wev), move |event: &Event| {
                    if !me.inner.ids.is_bootstrap_or_own(&event.ids) {
                        return;
                    }
                    // Prefer the already-seeded work id; fall back to the one the
                    // event carries, which is the only id available while the
                    // lifecycle seed is still queued behind this handler.
                    let work_id = me
                        .inner
                        .ids
                        .work_id
                        .get()
                        .or_else(|| event.ids.first().copied());
                    me.refresh_for(work_id);
                });
            }
            {
                let me = self.clone();
                ctx.subscribe_event(
                    Origin::WorkManagement(WorkManagementEvent::CloseWork),
                    move |event: &Event| {
                        if me.inner.ids.is_event_for_my_work(&event.ids) {
                            me.refresh_for(None)
                        }
                    },
                );
            }
            // Catch up when this window is already seeded (rebuild / late wire).
            self.refresh_for(self.inner.ids.work_id.get());
        }

        /// The reactive model to bind a `ListView` to (through the pane's
        /// `SortFilterListModel` search projection).
        pub fn list_model(&self) -> ListModel<TextReplacementRuleRow> {
            self.inner.model.clone()
        }

        /// Bumped on each refresh; for consumers that observe rather than bind
        /// the model (the pane's empty-state).
        pub fn version_signal(&self) -> Signal<u64> {
            self.inner.version.clone()
        }

        /// Current rows (sorted), materialized — for the engine rebuild, export
        /// and dedup.
        pub fn rows(&self) -> Vec<TextReplacementRuleRow> {
            snapshot(&self.inner.model)
        }

        pub fn len(&self) -> usize {
            self.inner.model.len()
        }

        pub fn is_empty(&self) -> bool {
            self.len() == 0
        }

        /// The trigger a candidate collides with, ignoring case and surrounding
        /// space, excluding `exclude`.
        pub fn colliding_trigger(&self, candidate: &str, exclude: Option<u64>) -> Option<String> {
            super::colliding_trigger(&self.rows(), candidate, exclude)
        }

        /// Create one rule under `owner_id`. `None` when no project is open or
        /// the write failed. The backend's `Created` event drives the refresh.
        pub fn create(
            &self,
            trigger: &str,
            replacement: &str,
            enabled: bool,
            owner_id: Option<u64>,
            stack_id: Option<u64>,
        ) -> Option<u64> {
            let owner = owner_id?;
            let now = chrono::Utc::now();
            let dto = CreateTextReplacementRuleDto {
                created_at: now,
                updated_at: now,
                trigger: trigger.trim().to_string(),
                replacement: replacement.to_string(),
                enabled,
            };
            match text_replacement_rule_commands::create_text_replacement_rule(
                &self.inner.ctx,
                stack_id,
                &dto,
                owner,
                -1,
            ) {
                Ok(created) => Some(created.id),
                Err(e) => {
                    eprintln!("text replacements: create failed: {e}");
                    None
                }
            }
        }

        /// Patch one rule. Every field is sent, so callers pass the row's
        /// current values for whatever they are not changing (`update` replaces
        /// all scalars). Read-modify-write off the live entity rather than off a
        /// `TextReplacementRuleRow`, the same reasoning as
        /// `WorkTagsListModel::update` — the row does not carry `created_at`.
        pub fn update(
            &self,
            id: u64,
            trigger: &str,
            replacement: &str,
            enabled: bool,
            stack_id: Option<u64>,
        ) {
            let existing = match text_replacement_rule_commands::get_text_replacement_rule(
                &self.inner.ctx,
                &id,
            ) {
                Ok(Some(r)) => r,
                Ok(None) => return,
                Err(e) => {
                    eprintln!("text replacements: update failed to read {id}: {e}");
                    return;
                }
            };
            let dto = UpdateTextReplacementRuleDto {
                id,
                created_at: existing.created_at,
                updated_at: chrono::Utc::now(),
                trigger: trigger.trim().to_string(),
                replacement: replacement.to_string(),
                enabled,
            };
            if let Err(e) = text_replacement_rule_commands::update_text_replacement_rule(
                &self.inner.ctx,
                stack_id,
                &dto,
            ) {
                eprintln!("text replacements: update failed: {e}");
            }
        }

        /// Remove the given ids in one undoable step. Missing ids are a safe
        /// no-op (a stale toast Undo after a manual delete).
        pub fn remove_all(&self, ids: &[u64], stack_id: Option<u64>) {
            let existing: std::collections::HashSet<u64> =
                self.rows().into_iter().map(|r| r.id).collect();
            let present: Vec<u64> = ids
                .iter()
                .copied()
                .filter(|id| existing.contains(id))
                .collect();
            if present.is_empty() {
                return;
            }
            if let Err(e) = text_replacement_rule_commands::remove_text_replacement_rule_multi(
                &self.inner.ctx,
                stack_id,
                &present,
            ) {
                eprintln!("text replacements: remove failed: {e}");
            }
        }

        /// Bulk-create as ONE undoable step (an import), skipping triggers
        /// already present. Returns the triggers that were skipped, for the
        /// summary.
        pub fn import(
            &self,
            rows: &[TextReplacementRuleRow],
            owner_id: Option<u64>,
            stack_id: Option<u64>,
        ) -> Vec<String> {
            if rows.is_empty() {
                return Vec::new();
            }
            let Some(owner) = owner_id else {
                return rows.iter().map(|r| r.trigger.clone()).collect();
            };
            let existing = self.rows();
            let mut seen: std::collections::HashSet<String> = existing
                .iter()
                .map(|r| super::trigger_key(&r.trigger))
                .collect();
            let mut skipped = Vec::new();
            let now = chrono::Utc::now();
            let mut dtos = Vec::new();
            for r in rows {
                let key = super::trigger_key(&r.trigger);
                if key.is_empty() || !seen.insert(key) {
                    skipped.push(r.trigger.clone());
                    continue;
                }
                dtos.push(CreateTextReplacementRuleDto {
                    created_at: now,
                    updated_at: now,
                    trigger: r.trigger.trim().to_string(),
                    replacement: r.replacement.clone(),
                    enabled: r.enabled,
                });
            }
            if !dtos.is_empty()
                && let Err(e) = text_replacement_rule_commands::create_text_replacement_rule_multi(
                    &self.inner.ctx,
                    stack_id,
                    &dtos,
                    owner,
                    -1,
                )
            {
                eprintln!("text replacements: import failed: {e}");
                return rows.iter().map(|r| r.trigger.clone()).collect();
            }
            skipped
        }

        /// Re-read the lexicon for the currently seeded Work (`None` → empty).
        fn refresh(&self) {
            self.refresh_for(self.inner.ids.work_id.get());
        }

        /// Re-read for an explicitly named Work, which is what lets a `LoadWork`
        /// handler load the project whose id `ids.work_id` does not carry yet
        /// (see [`Self::wire`]).
        pub(crate) fn refresh_for(&self, work_id: Option<u64>) {
            let rows = load_rows(&self.inner.ctx, work_id);
            // **The version bump is conditional, and that is load-bearing.** It
            // means "the lexicon changed", never "somebody re-read it". `wire`
            // now ends in a catch-up `refresh_for` that runs inside `build()`,
            // and the Settings pane binds this version: an unconditional bump
            // would be a write a widget's own build made, which dirties it,
            // which rebuilds, which builds, which bumps, at frame rate, forever.
            // The identical bump cost `WorkStatusesListModel` 205 rebuilds of an
            // idle window in six seconds before it was made conditional there.
            //
            // The typing session keys its engine off this same signal
            // (`session::refresh_engine` compiles on `(version, enabled)`), so
            // comparing first also spares it a recompile per unrelated event.
            let changed = snapshot(&self.inner.model) != rows;
            self.inner.model.reconcile_by_key(rows, |r| r.id);
            if changed {
                let v = &self.inner.version;
                v.set(v.get().wrapping_add(1));
            }
        }
    }

    /// Read `work_id`'s `TextReplacementRule`s (via
    /// `Work.text_replacement_rules`), into sorted rows. The id is passed rather
    /// than read off `AppIds` so a `LoadWork` handler can name the Work the seed
    /// has not written yet (see [`TextReplacementRuleListModel::wire`]) — **not**
    /// `get_all_text_replacement_rule`, which returns every rule in the whole
    /// shared store: with a second Work simultaneously open, that would merge
    /// both Works' lexicons into one list, and make one Work's private rules
    /// editable/deletable from the other's Settings pane and applicable to the
    /// other's typing session.
    fn load_rows(ctx: &AppContext, work_id: Option<u64>) -> Vec<TextReplacementRuleRow> {
        let Some(work_id) = work_id else {
            return Vec::new(); // no project open
        };
        let rule_ids = work_commands::get_work_relationship(
            ctx,
            &work_id,
            &WorkRelationshipField::TextReplacementRules,
        )
        .unwrap_or_default();
        let mut rows: Vec<TextReplacementRuleRow> =
            text_replacement_rule_commands::get_text_replacement_rule_multi(ctx, &rule_ids)
                .unwrap_or_default()
                .into_iter()
                .flatten()
                .map(|r| TextReplacementRuleRow {
                    id: r.id,
                    trigger: r.trigger,
                    replacement: r.replacement,
                    enabled: r.enabled,
                })
                .collect();
        sort_rows(&mut rows);
        rows
    }

    fn snapshot(model: &ListModel<TextReplacementRuleRow>) -> Vec<TextReplacementRuleRow> {
        (0..model.len())
            .filter_map(|i| model.with_item(i, |r| r.clone()))
            .collect()
    }
}

#[cfg(feature = "mocks")]
mod imp {
    use std::cell::Cell;
    use std::rc::Rc;

    use teksilo::data::ListModel;
    use teksilo::prelude::*;

    use frontend::AppContext;

    use crate::app_ids::AppIds;

    use super::{TextReplacementRuleRow, sort_rows, trigger_key};

    struct Inner {
        model: ListModel<TextReplacementRuleRow>,
        version: Signal<u64>,
        next_id: Cell<u64>,
    }

    #[derive(Clone)]
    pub struct TextReplacementRuleListModel {
        inner: Rc<Inner>,
    }

    fn row(id: u64, trigger: &str, replacement: &str, enabled: bool) -> TextReplacementRuleRow {
        TextReplacementRuleRow {
            id,
            trigger: trigger.to_string(),
            replacement: replacement.to_string(),
            enabled,
        }
    }

    impl TextReplacementRuleListModel {
        pub fn new(_ctx: Rc<AppContext>, _ids: AppIds) -> Self {
            let mut rows = vec![
                row(1, "--", "—", true),
                row(2, "btw", "by the way", true),
                row(3, "teh", "the", false),
            ];
            sort_rows(&mut rows);
            Self {
                inner: Rc::new(Inner {
                    model: ListModel::from_vec(rows),
                    version: Signal::new(0),
                    next_id: Cell::new(4),
                }),
            }
        }

        pub fn wire(&self, _ctx: &mut BuildContext) {}

        pub fn list_model(&self) -> ListModel<TextReplacementRuleRow> {
            self.inner.model.clone()
        }

        pub fn version_signal(&self) -> Signal<u64> {
            self.inner.version.clone()
        }

        pub fn rows(&self) -> Vec<TextReplacementRuleRow> {
            (0..self.inner.model.len())
                .filter_map(|i| self.inner.model.with_item(i, |r| r.clone()))
                .collect()
        }

        pub fn len(&self) -> usize {
            self.inner.model.len()
        }

        pub fn is_empty(&self) -> bool {
            self.len() == 0
        }

        pub fn colliding_trigger(&self, candidate: &str, exclude: Option<u64>) -> Option<String> {
            super::colliding_trigger(&self.rows(), candidate, exclude)
        }

        pub fn create(
            &self,
            trigger: &str,
            replacement: &str,
            enabled: bool,
            _owner_id: Option<u64>,
            _stack_id: Option<u64>,
        ) -> Option<u64> {
            let id = self.inner.next_id.get();
            self.inner.next_id.set(id + 1);
            let mut rows = self.rows();
            rows.push(row(id, trigger.trim(), replacement, enabled));
            self.replace(rows);
            Some(id)
        }

        pub fn update(
            &self,
            id: u64,
            trigger: &str,
            replacement: &str,
            enabled: bool,
            _stack_id: Option<u64>,
        ) {
            let mut rows = self.rows();
            if let Some(r) = rows.iter_mut().find(|r| r.id == id) {
                r.trigger = trigger.trim().to_string();
                r.replacement = replacement.to_string();
                r.enabled = enabled;
            }
            self.replace(rows);
        }

        pub fn remove_all(&self, ids: &[u64], _stack_id: Option<u64>) {
            let keep: Vec<TextReplacementRuleRow> = self
                .rows()
                .into_iter()
                .filter(|r| !ids.contains(&r.id))
                .collect();
            self.replace(keep);
        }

        pub fn import(
            &self,
            rows: &[TextReplacementRuleRow],
            _owner_id: Option<u64>,
            _stack_id: Option<u64>,
        ) -> Vec<String> {
            let mut current = self.rows();
            let mut skipped = Vec::new();
            for r in rows {
                if r.trigger.trim().is_empty()
                    || current
                        .iter()
                        .any(|e| trigger_key(&e.trigger) == trigger_key(&r.trigger))
                {
                    skipped.push(r.trigger.clone());
                    continue;
                }
                let id = self.inner.next_id.get();
                self.inner.next_id.set(id + 1);
                current.push(row(id, r.trigger.trim(), &r.replacement, r.enabled));
            }
            self.replace(current);
            skipped
        }

        fn replace(&self, mut rows: Vec<TextReplacementRuleRow>) {
            sort_rows(&mut rows);
            self.inner.model.replace_all(rows);
            let v = &self.inner.version;
            v.set(v.get().wrapping_add(1));
        }
    }
}

pub use imp::TextReplacementRuleListModel;

/// What only the real half can get wrong: which Work a refresh reads, and when the
/// version signal is allowed to move. The mock half fabricates its rows and never
/// re-reads anything, so both questions are answered for it by construction.
#[cfg(all(test, not(feature = "mocks")))]
mod store_tests {
    use super::*;
    use crate::app_ids::AppIds;
    use frontend::AppContext;
    use frontend::commands::{smart_punctuation_commands, work_commands};
    use frontend::common::entities::QuoteStyle;
    use frontend::direct_access::{CreateSmartPunctuationDto, CreateWorkDto};
    use std::rc::Rc;

    fn now() -> chrono::DateTime<chrono::Utc> {
        chrono::Utc::now()
    }

    /// A Work in the store and a model pointed at **nothing** — `AppIds` exactly as it
    /// stands while `LoadWork` is being dispatched and the lifecycle seed is still queued.
    fn model_and_unseeded_work() -> (TextReplacementRuleListModel, AppIds, u64) {
        let ctx = Rc::new(AppContext::new());
        // A Work owns exactly one SmartPunctuation (one_to_one, strong), so it has to
        // exist before the Work that points at it.
        let smart_punctuation = smart_punctuation_commands::create_orphan_smart_punctuation(
            &ctx,
            None,
            &CreateSmartPunctuationDto {
                created_at: now(),
                updated_at: now(),
                override_app_default: false,
                dashes: false,
                ellipsis: false,
                quotes: false,
                quote_style: QuoteStyle::LocaleDefault,
                pre_punctuation_spacing: false,
                dialogue_marker: false,
            },
        )
        .expect("create smart_punctuation")
        .id;
        let work = work_commands::create_orphan_work(
            &ctx,
            None,
            &CreateWorkDto {
                statuses: Vec::new(),
                created_at: now(),
                updated_at: now(),
                title: "W".into(),
                smart_punctuation,
                ..Default::default()
            },
        )
        .expect("create work")
        .id;
        let ids = AppIds::new();
        (
            TextReplacementRuleListModel::new(ctx, ids.clone()),
            ids,
            work,
        )
    }

    /// **The reported bug, as a unit test.** `App::build` wires this model before the
    /// lifecycle seed writes `ids.work_id`, and both answer the same `LoadWork` with this
    /// one first — so the handler has nothing but the event's own id to go on. It must be
    /// able to load that Work anyway.
    ///
    /// The event itself cannot be driven headlessly (`test_support`'s source never fires),
    /// so what is pinned here is the capability the handler depends on.
    #[test]
    fn a_lexicon_loads_for_a_work_the_ids_have_not_been_seeded_with() {
        let (model, ids, work) = model_and_unseeded_work();
        assert_eq!(ids.work_id.get(), None, "the seed has not run yet");
        model.create("btw", "by the way", true, Some(work), None);
        model.create("teh", "the", true, Some(work), None);

        model.refresh_for(Some(work));

        let triggers: Vec<String> = model.rows().into_iter().map(|r| r.trigger).collect();
        assert_eq!(triggers, vec!["btw".to_string(), "teh".to_string()]);
    }

    /// The other half of the same story: a refresh that reads the unseeded `AppIds` finds
    /// no project and empties the list. Correct in itself, and exactly what left the
    /// lexicon empty for a whole session when it was the only thing the handler did.
    #[test]
    fn a_refresh_that_reads_the_unseeded_ids_finds_no_project() {
        let (model, _ids, work) = model_and_unseeded_work();
        model.create("btw", "by the way", true, Some(work), None);
        model.refresh_for(Some(work));
        assert_eq!(model.len(), 1);

        model.refresh_for(None);

        assert!(
            model.is_empty(),
            "no open Work means no lexicon, not the previous Work's"
        );
    }

    /// **The rebuild loop, as a unit test.** `wire` ends in a catch-up `refresh_for` that
    /// runs inside `build()`, and the Settings pane binds this version at rebuild level: a
    /// bump on an unchanged re-read is a write the widget's own build made, which dirties
    /// it, which rebuilds, which builds, which bumps. Measured on the identical bug in
    /// `WorkStatusesListModel`: 205 rebuilds of an idle window in six seconds.
    #[test]
    fn re_reading_an_unchanged_lexicon_does_not_move_the_version() {
        let (model, ids, work) = model_and_unseeded_work();
        ids.work_id.set(Some(work));
        model.create("btw", "by the way", true, Some(work), None);
        model.refresh_for(Some(work));
        let settled = model.version_signal().get();

        for _ in 0..20 {
            model.refresh_for(Some(work));
        }

        assert_eq!(
            model.version_signal().get(),
            settled,
            "a re-read that changes nothing must not ask anyone to repaint"
        );
        assert_eq!(model.len(), 1, "and it must not lose the row either");
    }

    /// The bump still happens when the lexicon really moves, or the Settings pane's
    /// empty-state and the typing session's engine would both keep a stale answer.
    #[test]
    fn a_real_change_does_move_the_version() {
        let (model, ids, work) = model_and_unseeded_work();
        ids.work_id.set(Some(work));
        model.refresh_for(Some(work));
        let before = model.version_signal().get();

        model.create("btw", "by the way", true, Some(work), None);
        model.refresh_for(Some(work));

        assert_ne!(model.version_signal().get(), before);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn r(id: u64, trigger: &str) -> TextReplacementRuleRow {
        TextReplacementRuleRow {
            id,
            trigger: trigger.to_string(),
            ..Default::default()
        }
    }

    #[test]
    fn sorting_is_case_insensitive_but_deterministic() {
        let mut rows = vec![r(1, "beta"), r(2, "Alpha"), r(3, "alpha")];
        sort_rows(&mut rows);
        let triggers: Vec<&str> = rows.iter().map(|r| r.trigger.as_str()).collect();
        assert_eq!(triggers, vec!["Alpha", "alpha", "beta"]);
    }

    #[test]
    fn trigger_key_ignores_case_and_surrounding_space() {
        assert_eq!(trigger_key("  BTW  "), "btw");
        assert_eq!(trigger_key("btw"), trigger_key("Btw"));
        assert_eq!(trigger_key("   "), "");
    }

    fn rules() -> Vec<TextReplacementRuleRow> {
        vec![r(1, "btw"), r(2, "--"), r(3, "very long trigger word")]
    }

    #[test]
    fn an_exact_trigger_collides() {
        assert_eq!(
            colliding_trigger(&rules(), "btw", None).as_deref(),
            Some("btw")
        );
    }

    #[test]
    fn a_differently_cased_trigger_collides_and_reports_the_existing_spelling() {
        assert_eq!(
            colliding_trigger(&rules(), "BTW", None).as_deref(),
            Some("btw"),
            "the warning names the trigger as it is actually spelled, not as it was typed"
        );
    }

    #[test]
    fn a_rule_never_collides_with_itself() {
        assert_eq!(colliding_trigger(&rules(), "btw", Some(1)), None);
        let mut two = rules();
        two.push(r(4, "BTW"));
        assert_eq!(
            colliding_trigger(&two, "btw", Some(1)).as_deref(),
            Some("BTW")
        );
    }

    #[test]
    fn a_novel_trigger_does_not_collide() {
        assert_eq!(colliding_trigger(&rules(), "xyz", None), None);
    }

    #[test]
    fn a_blank_candidate_never_collides() {
        assert_eq!(colliding_trigger(&rules(), "", None), None);
        assert_eq!(colliding_trigger(&rules(), "   ", None), None);
    }

    #[test]
    fn an_empty_lexicon_has_nothing_to_collide_with() {
        assert_eq!(colliding_trigger(&[], "anything", None), None);
    }
}
