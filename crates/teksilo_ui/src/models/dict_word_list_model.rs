// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Reactive list model over the open Work's personal dictionary (`DictWord`).
//!
//! The public surface is a `teksilo::data::ListModel<DictWordRow>` a `ListView`
//! binds to (the Settings ▸ Personal-dictionary pane), plus a `version` signal
//! for non-`ListView` consumers (the empty-state), and the collection writes the
//! [`UserDictionaryViewModel`](crate::view_models::UserDictionaryViewModel)
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
    use std::cell::Cell;
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
        subscribed: Cell<bool>,
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
                    Origin::DirectAccess(DirectAccessEntity::DictWord(ev)),
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

        fn refresh(&self) {
            self.inner
                .model
                .reconcile_by_key(load_rows(&self.inner.ctx, &self.inner.ids), |r| r.id);
            let v = &self.inner.version;
            v.set(v.get().wrapping_add(1));
        }
    }

    /// Read this window's own open Work's `DictWord`s (via `Work.dict_words`), into
    /// sorted rows — **not** `get_all_dict_word`, which returns every `DictWord` in
    /// the whole shared store: with a second Work simultaneously open, that would
    /// merge both Works' personal dictionaries into one list, and make one Work's
    /// private words deletable from the other's Settings pane.
    fn load_rows(ctx: &AppContext, ids: &AppIds) -> Vec<DictWordRow> {
        let Some(work_id) = ids.work_id.get() else {
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
