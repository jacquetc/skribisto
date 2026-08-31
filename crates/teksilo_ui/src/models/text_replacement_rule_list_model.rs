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
    use std::cell::Cell;
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
        subscribed: Cell<bool>,
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
            let model = ListModel::from_vec(load_rows(&ctx, &ids));
            Self {
                inner: Rc::new(Inner {
                    model,
                    version: Signal::new(0),
                    subscribed: Cell::new(false),
                    ctx,
                    ids,
                }),
            }
        }

        /// Subscribe (once) so the list stays live: any `TextReplacementRule`
        /// mutation — from this pane, the editor's typing session, or an
        /// undo/redo — re-reads the set, and a project switch replaces it
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
        pub fn wire(&self, ctx: &mut BuildContext) {
            if self.inner.subscribed.replace(true) {
                return;
            }
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
                    if me.inner.ids.is_bootstrap_or_own(&event.ids) {
                        me.refresh()
                    }
                });
            }
            {
                let me = self.clone();
                ctx.subscribe_event(
                    Origin::WorkManagement(WorkManagementEvent::CloseWork),
                    move |event: &Event| {
                        if me.inner.ids.is_event_for_my_work(&event.ids) {
                            me.refresh()
                        }
                    },
                );
            }
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

        fn refresh(&self) {
            self.inner
                .model
                .reconcile_by_key(load_rows(&self.inner.ctx, &self.inner.ids), |r| r.id);
            let v = &self.inner.version;
            v.set(v.get().wrapping_add(1));
        }
    }

    /// Read this window's own open Work's `TextReplacementRule`s (via
    /// `Work.text_replacement_rules`), into sorted rows — **not**
    /// `get_all_text_replacement_rule`, which returns every rule in the whole
    /// shared store: with a second Work simultaneously open, that would merge
    /// both Works' lexicons into one list, and make one Work's private rules
    /// editable/deletable from the other's Settings pane and applicable to the
    /// other's typing session.
    fn load_rows(ctx: &AppContext, ids: &AppIds) -> Vec<TextReplacementRuleRow> {
        let Some(work_id) = ids.work_id.get() else {
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
