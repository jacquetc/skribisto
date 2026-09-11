// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Reactive list model over the open Work's personal dictionary (`DictWord`).
//!
//! The public surface is a `teksilo::data::ListModel<DictWordRow>` a `ListView`
//! binds to (the Settings ▸ Personal-dictionary pane), plus a `version` signal
//! for non-`ListView` consumers (the empty-state), and the collection writes the
//! [`UserDictionaryViewModel`](crate::spellcheck::UserDictionaryViewModel)
//! drives: [`add_words`](imp::DictWordListModel::add_words) and
//! [`remove_all`](imp::DictWordListModel::remove_all). Per-entity **rename** is
//! [`SingleDictWord`](crate::singles::SingleDictWord)'s job.
//!
//! Two `#[cfg]`-gated `mod imp` variants share one public surface: the real one
//! reads via `get_all_dict_word` and stays live on `DictWord` events + project
//! switches, and its writes go through `dict_word_commands` (which fire those
//! events); the mock one holds a fabricated list and mutates it in place, since
//! `--features mocks` has no real `Work` to own a created `DictWord`.
//!
//! Rows are sorted case-insensitively for a stable display order.

/// One personal-dictionary row: the entity id and the word as the user typed it.
/// The word is stored/shown verbatim; spell-check matching is exact-case (see
/// `crate::spellcheck`), so `"the"` and `"The"` are distinct rows.
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct DictWordRow {
    pub id: u64,
    pub word: String,
}

/// Sort key: case-insensitive, then exact, so equal-fold words keep a
/// deterministic order.
fn sort_rows(rows: &mut [DictWordRow]) {
    rows.sort_by(|a, b| {
        a.word
            .to_lowercase()
            .cmp(&b.word.to_lowercase())
            .then_with(|| a.word.cmp(&b.word))
    });
}

#[cfg(not(feature = "mocks"))]
mod imp {
    use std::rc::Rc;

    use teksilo::data::ListModel;
    use teksilo::prelude::*;

    use frontend::AppContext;
    use frontend::commands::{dict_word_commands, work_commands};
    use frontend::common::direct_access::work::WorkRelationshipField;
    use frontend::common::event::{
        DirectAccessEntity, EntityEvent, Event, Origin, WorkManagementEvent,
    };
    use frontend::direct_access::CreateDictWordDto;

    use crate::app_ids::AppIds;

    use super::{DictWordRow, sort_rows};

    struct Inner {
        model: ListModel<DictWordRow>,
        version: Signal<u64>,
        ctx: Rc<AppContext>,
        /// The open Work's own ids — `work_id` scopes every read to *this*
        /// Work's `dict_words` relationship (see [`load_rows`]); a second
        /// simultaneously-open Work's personal dictionary must never merge into
        /// this one's list.
        ids: AppIds,
    }

    #[derive(Clone)]
    pub struct DictWordListModel {
        inner: Rc<Inner>,
    }

    impl DictWordListModel {
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

        /// Subscribe (once) so the list stays live: any `DictWord` mutation — from
        /// this pane, the editor's "Add to dictionary", or an undo/redo — re-reads
        /// the set, and a project switch replaces it wholesale.
        ///
        /// `DictWord` entity events carry no `work_id`, only the changed entities'
        /// own ids — but `refresh` always re-derives the list from this model's
        /// own `ids.work_id` (see [`load_rows`]), so a sibling Work's `DictWord`
        /// event only ever costs a harmless, still-correct re-read here, never a
        /// wrong one. `LoadWork`/`NewWork`/`CloseWork` DO carry `work_id` and are
        /// guarded accordingly: a sibling Work's project boundary must not force
        /// a reload of a list that is (before or after) legitimately empty.
        ///
        /// **The `LoadWork`/`NewWork` arms fall back to the event's own work id, and
        /// `wire` ends in a catch-up**, so this model no longer depends on being wired
        /// *after* `wiring::project_events`' lifecycle seed — which is the only reason it
        /// ever loaded anything, its call site in `App::build` happening to sit below the
        /// seed's. Wired above it, a bare `refresh` would read `None` and the personal
        /// dictionary would stay empty for the whole session, since merely opening a
        /// project produces no `DictWord` event. Same bug, same fix, as
        /// `TextReplacementRuleListModel::wire`, which shipped it.
        ///
        /// **Re-subscribes on every call.** A `BuildContext` subscription is scoped to
        /// the current build and dropped on the next, so the one-shot guard this used to
        /// carry left the model deaf after any rebuild while still reporting itself wired.
        pub fn wire(&self, ctx: &mut BuildContext) {
            for ev in [
                EntityEvent::Created,
                EntityEvent::Updated,
                EntityEvent::Removed,
            ] {
                let me = self.clone();
                ctx.subscribe_event(
                    Origin::DirectAccess(DirectAccessEntity::DictWord(ev)),
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
        pub fn list_model(&self) -> ListModel<DictWordRow> {
            self.inner.model.clone()
        }

        /// Bumped on each refresh; for consumers that observe rather than bind the
        /// model (the pane's empty-state).
        pub fn version_signal(&self) -> Signal<u64> {
            self.inner.version.clone()
        }

        /// Current rows (sorted), materialized — for export and dedup.
        pub fn rows(&self) -> Vec<DictWordRow> {
            snapshot(&self.inner.model)
        }

        /// Exact-case membership (the dedup + validation test).
        pub fn contains(&self, word: &str) -> bool {
            (0..self.inner.model.len()).any(|i| {
                self.inner
                    .model
                    .with_item(i, |r| r.word == word)
                    .unwrap_or(false)
            })
        }

        pub fn len(&self) -> usize {
            self.inner.model.len()
        }

        pub fn is_empty(&self) -> bool {
            self.len() == 0
        }

        /// Create the given words under `owner_id` in a single undoable step
        /// (`create_dict_word_multi`), returning the new ids. No-op (empty) when
        /// `words` is empty or no project is open. The backend's `Created` event
        /// drives the list refresh — callers rely on that, not a manual reload.
        pub fn add_words(
            &self,
            words: &[String],
            owner_id: Option<u64>,
            stack_id: Option<u64>,
        ) -> Vec<u64> {
            let Some(owner) = owner_id else {
                return Vec::new();
            };
            if words.is_empty() {
                return Vec::new();
            }
            let now = chrono::Utc::now();
            let dtos: Vec<CreateDictWordDto> = words
                .iter()
                .map(|w| CreateDictWordDto {
                    created_at: now,
                    updated_at: now,
                    word: w.clone(),
                })
                .collect();
            match dict_word_commands::create_dict_word_multi(
                &self.inner.ctx,
                stack_id,
                &dtos,
                owner,
                -1,
            ) {
                Ok(created) => created.into_iter().map(|d| d.id).collect(),
                Err(e) => {
                    eprintln!("dictionary: add words failed: {e}");
                    Vec::new()
                }
            }
        }

        /// Remove the given ids in one undoable step. Missing ids are a safe no-op
        /// (a stale toast Undo after a manual delete).
        pub fn remove_all(&self, ids: &[u64], stack_id: Option<u64>) {
            let present: Vec<u64> = {
                let existing: std::collections::HashSet<u64> = snapshot(&self.inner.model)
                    .into_iter()
                    .map(|r| r.id)
                    .collect();
                ids.iter()
                    .copied()
                    .filter(|id| existing.contains(id))
                    .collect()
            };
            if present.is_empty() {
                return;
            }
            if let Err(e) =
                dict_word_commands::remove_dict_word_multi(&self.inner.ctx, stack_id, &present)
            {
                eprintln!("dictionary: remove failed: {e}");
            }
        }

        /// Re-read the dictionary for the currently seeded Work (`None` → empty).
        fn refresh(&self) {
            self.refresh_for(self.inner.ids.work_id.get());
        }

        /// Re-read for an explicitly named Work, which is what lets a `LoadWork`
        /// handler load the project whose id `ids.work_id` does not carry yet
        /// (see [`Self::wire`]).
        pub(crate) fn refresh_for(&self, work_id: Option<u64>) {
            let rows = load_rows(&self.inner.ctx, work_id);
            // Conditional, because `wire` now ends in a catch-up that runs inside
            // `build()`: an unconditional bump would be a write a widget's own build
            // made, which dirties it, which rebuilds, which builds, which bumps. See
            // `WorkStatusesListModel::refresh_for`, where that cost 205 rebuilds of an
            // idle window in six seconds.
            let changed = snapshot(&self.inner.model) != rows;
            self.inner.model.reconcile_by_key(rows, |r| r.id);
            if changed {
                let v = &self.inner.version;
                v.set(v.get().wrapping_add(1));
            }
        }
    }

    /// Read this window's own open Work's `DictWord`s (via `Work.dict_words`), into
    /// sorted rows — **not** `get_all_dict_word`, which returns every `DictWord` in
    /// the whole shared store: with a second Work simultaneously open, that would
    /// merge both Works' personal dictionaries into one list, and make one Work's
    /// private words deletable from the other's Settings pane.
    fn load_rows(ctx: &AppContext, work_id: Option<u64>) -> Vec<DictWordRow> {
        let Some(work_id) = work_id else {
            return Vec::new(); // no project open
        };
        let word_ids =
            work_commands::get_work_relationship(ctx, &work_id, &WorkRelationshipField::DictWords)
                .unwrap_or_default();
        let mut rows: Vec<DictWordRow> = dict_word_commands::get_dict_word_multi(ctx, &word_ids)
            .unwrap_or_default()
            .into_iter()
            .flatten()
            .map(|d| DictWordRow {
                id: d.id,
                word: d.word,
            })
            .collect();
        sort_rows(&mut rows);
        rows
    }

    fn snapshot(model: &ListModel<DictWordRow>) -> Vec<DictWordRow> {
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

    use super::{DictWordRow, sort_rows};

    struct Inner {
        model: ListModel<DictWordRow>,
        version: Signal<u64>,
        next_id: Cell<u64>,
    }

    #[derive(Clone)]
    pub struct DictWordListModel {
        inner: Rc<Inner>,
    }

    impl DictWordListModel {
        pub fn new(_ctx: Rc<AppContext>, _ids: AppIds) -> Self {
            let mut rows = vec![
                DictWordRow {
                    id: 1,
                    word: "Teksilo".to_string(),
                },
                DictWordRow {
                    id: 2,
                    word: "Gandalf".to_string(),
                },
                DictWordRow {
                    id: 3,
                    word: "Skribisto".to_string(),
                },
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

        pub fn list_model(&self) -> ListModel<DictWordRow> {
            self.inner.model.clone()
        }

        pub fn version_signal(&self) -> Signal<u64> {
            self.inner.version.clone()
        }

        pub fn rows(&self) -> Vec<DictWordRow> {
            (0..self.inner.model.len())
                .filter_map(|i| self.inner.model.with_item(i, |r| r.clone()))
                .collect()
        }

        pub fn contains(&self, word: &str) -> bool {
            self.rows().iter().any(|r| r.word == word)
        }

        pub fn len(&self) -> usize {
            self.inner.model.len()
        }

        pub fn is_empty(&self) -> bool {
            self.len() == 0
        }

        /// Fabricated add: no backend, so mutate the in-memory list directly (and
        /// bump `version`), returning the assigned ids.
        pub fn add_words(
            &self,
            words: &[String],
            _owner_id: Option<u64>,
            _stack_id: Option<u64>,
        ) -> Vec<u64> {
            let mut ids = Vec::new();
            let mut rows = self.rows();
            for w in words {
                let id = self.inner.next_id.get();
                self.inner.next_id.set(id + 1);
                rows.push(DictWordRow {
                    id,
                    word: w.clone(),
                });
                ids.push(id);
            }
            sort_rows(&mut rows);
            self.inner.model.replace_all(rows);
            self.bump();
            ids
        }

        pub fn remove_all(&self, ids: &[u64], _stack_id: Option<u64>) {
            let keep: Vec<DictWordRow> = self
                .rows()
                .into_iter()
                .filter(|r| !ids.contains(&r.id))
                .collect();
            self.inner.model.replace_all(keep);
            self.bump();
        }

        fn bump(&self) {
            let v = &self.inner.version;
            v.set(v.get().wrapping_add(1));
        }
    }
}

pub use imp::DictWordListModel;

/// What only the real half can get wrong: which Work a refresh reads, and when the
/// version signal is allowed to move.
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
    fn model_and_unseeded_work() -> (DictWordListModel, AppIds, u64) {
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
        (DictWordListModel::new(ctx, ids.clone()), ids, work)
    }

    /// The capability the `LoadWork` handler depends on: load the Work the event names,
    /// not the one `AppIds` has not been told about yet. This model used to be correct
    /// only because its call site in `App::build` happened to sit below the seed's.
    #[test]
    fn a_dictionary_loads_for_a_work_the_ids_have_not_been_seeded_with() {
        let (model, ids, work) = model_and_unseeded_work();
        assert_eq!(ids.work_id.get(), None, "the seed has not run yet");
        model.add_words(
            &["azerty".to_string(), "qwerty".to_string()],
            Some(work),
            None,
        );

        model.refresh_for(Some(work));

        assert_eq!(model.len(), 2);
        assert!(model.contains("azerty"));
    }

    /// The rebuild loop, as a unit test: `wire` ends in a catch-up that runs inside
    /// `build()`, so a bump on an unchanged re-read would dirty the widget that made it.
    #[test]
    fn re_reading_an_unchanged_dictionary_does_not_move_the_version() {
        let (model, ids, work) = model_and_unseeded_work();
        ids.work_id.set(Some(work));
        model.add_words(&["azerty".to_string()], Some(work), None);
        model.refresh_for(Some(work));
        let settled = model.version_signal().get();

        for _ in 0..20 {
            model.refresh_for(Some(work));
        }

        assert_eq!(model.version_signal().get(), settled);
        assert_eq!(model.len(), 1, "and it must not lose the row either");
    }

    /// No open Work means no dictionary, never the previous Work's.
    #[test]
    fn closing_the_work_empties_the_dictionary() {
        let (model, _ids, work) = model_and_unseeded_work();
        model.add_words(&["azerty".to_string()], Some(work), None);
        model.refresh_for(Some(work));
        assert_eq!(model.len(), 1);

        model.refresh_for(None);

        assert!(model.is_empty());
    }
}
