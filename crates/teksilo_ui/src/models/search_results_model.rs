// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Reactive list of the open project's [`SearchResult`](frontend::common::entities::SearchResult) rows — the data behind
//! the search dock's result list.
//!
//! One row per matching **field** (a scene's body, its synopsis, an item's title
//! or label), each carrying an `occurrence_count` and a snippet; per-occurrence
//! review happens in the bottom preview, recomputed live. The rows are produced
//! by the `run_search` use case, which drops the old set and writes the new one
//! in two bulk store writes and then publishes **one**
//! `SearchManagement(RunSearch)` event — so this model subscribes to that single
//! origin, not to the per-entity `Created`/`Removed` events, and rebuilds once
//! per search regardless of how many rows changed.
//!
//! The bound source of truth is a `teksilo::data::ListModel<SearchResultDto>` a
//! `ListView` binds to, kept in the **project-relative order** the `Search` owns
//! its results in (an `ordered_one_to_many`) — read back through
//! `get_search_relationship`, never `get_all` (which returns key order).
//!
//! Two `#[cfg]`-gated `mod imp` variants share one public surface (see the
//! convention note in [`crate::models`]): the real one reads the store; the mock
//! fabricates a handful of rows so the dock renders with fabricated data.

#[cfg(not(feature = "mocks"))]
mod imp {
    use std::rc::Rc;

    use teksilo::data::ListModel;
    use teksilo::prelude::*;

    use frontend::AppContext;
    use frontend::commands::{search_commands, search_result_commands, work_info_commands};
    use frontend::common::direct_access::search::SearchRelationshipField;
    use frontend::direct_access::SearchResultDto;

    struct Inner {
        /// Reactive rows a `ListView` binds to, in the `Search`'s own result order.
        /// The `ListView` observes this directly (its `replace_all` emits a data
        /// change), so no separate version signal is needed.
        model: ListModel<SearchResultDto>,
        ctx: Rc<AppContext>,
        /// The open project's `WorkInfo` id (ids-only global state) — it owns the
        /// `Search`, which owns these results.
        work_info_id: Signal<Option<u64>>,
    }

    #[derive(Clone)]
    pub struct SearchResultsModel {
        inner: Rc<Inner>,
    }

    impl SearchResultsModel {
        pub fn new(ctx: Rc<AppContext>, work_info_id: Signal<Option<u64>>) -> Self {
            Self {
                inner: Rc::new(Inner {
                    model: ListModel::from_vec(Vec::new()),
                    ctx,
                    work_info_id,
                }),
            }
        }

        /// Nothing to wire: the model is refreshed **explicitly** by the
        /// view-model right after a search runs (and on project restore), via
        /// [`reload`](Self::reload) — not through a backend-event subscription.
        ///
        /// The event path is a trap here: the search dock is a *switchable*
        /// leading tab, so its content is torn down and rebuilt as the writer
        /// flips between the binder and search. A `ctx.subscribe_event` registered
        /// in that content dies when the tab is parked, and a one-shot "subscribe
        /// once" guard then blocks re-subscribing on the next reveal — the list
        /// would silently stop updating. Driving `reload` from the view-model (a
        /// process-lifetime handle) sidesteps that entirely.
        pub fn wire(&self, _ctx: &mut BuildContext) {}

        /// The reactive model to bind a `ListView` to.
        pub fn list_model(&self) -> ListModel<SearchResultDto> {
            self.inner.model.clone()
        }

        /// Re-read the ordered result set from the store and swap it into the model.
        /// Called by the view-model after every search and on project restore.
        pub fn reload(&self) {
            self.inner
                .model
                .replace_all(query(&self.inner.ctx, &self.inner.work_info_id));
        }

        /// `Vec` snapshot of every current row, in order.
        ///
        /// Prefer [`for_each`](Self::for_each) / [`find`](Self::find) for anything that
        /// only reads: this clones every row, and a result set now runs to `RESULT_CAP`
        /// rows rather than the 300 it was once bounded to.
        pub fn items(&self) -> Vec<SearchResultDto> {
            (0..self.inner.model.len())
                .filter_map(|i| self.inner.model.with_item(i, |d| d.clone()))
                .collect()
        }

        /// Visit every row in order, **without cloning it**.
        ///
        /// The counting/filtering the view-model does after each search (which rows are
        /// comments, how many are still ticked, which items they touch) reads four fields
        /// and keeps none of them — and it runs on every settled keystroke. Cloning a whole
        /// manuscript's worth of rows to do that is per-keystroke garbage nobody reads.
        pub fn for_each(&self, mut f: impl FnMut(&SearchResultDto)) {
            for i in 0..self.inner.model.len() {
                self.inner.model.with_item(i, &mut f);
            }
        }

        /// The first row matching `pred`, cloned — the only one the caller keeps.
        pub fn find(&self, pred: impl Fn(&SearchResultDto) -> bool) -> Option<SearchResultDto> {
            (0..self.inner.model.len()).find_map(|i| {
                self.inner
                    .model
                    .with_item(i, |d| pred(d).then(|| d.clone()))?
            })
        }
    }

    /// The open project's `SearchResult` rows, in the `Search`'s own result order.
    ///
    /// Reads the `Search` id off the single open `WorkInfo`, then its ordered
    /// `Results` relationship — never `get_all_search_result`, whose "database key
    /// order" note (see the generated command) would scramble the order the
    /// `run_search` use case carefully built.
    fn query(ctx: &AppContext, work_info_id: &Signal<Option<u64>>) -> Vec<SearchResultDto> {
        let Some(work_info_id) = work_info_id.get() else {
            return Vec::new(); // no project open
        };
        let Ok(Some(work_info)) = work_info_commands::get_work_info(ctx, &work_info_id) else {
            return Vec::new();
        };
        let ids = search_commands::get_search_relationship(
            ctx,
            &work_info.search,
            &SearchRelationshipField::Results,
        )
        .unwrap_or_default();
        search_result_commands::get_search_result_multi(ctx, &ids)
            .unwrap_or_default()
            .into_iter()
            .flatten()
            .collect()
    }
}

#[cfg(feature = "mocks")]
mod imp {
    use std::rc::Rc;

    use teksilo::data::ListModel;
    use teksilo::prelude::*;

    use frontend::AppContext;
    use frontend::common::entities::MatchField;
    use frontend::direct_access::SearchResultDto;

    #[derive(Clone)]
    pub struct SearchResultsModel {
        model: ListModel<SearchResultDto>,
    }

    impl SearchResultsModel {
        pub fn new(_ctx: Rc<AppContext>, _work_info_id: Signal<Option<u64>>) -> Self {
            Self {
                model: ListModel::from_vec(mock_rows()),
            }
        }

        pub fn wire(&self, _ctx: &mut BuildContext) {}

        pub fn list_model(&self) -> ListModel<SearchResultDto> {
            self.model.clone()
        }

        /// No backend in the mock build — the fabricated rows are fixed.
        pub fn reload(&self) {}

        pub fn items(&self) -> Vec<SearchResultDto> {
            (0..self.model.len())
                .filter_map(|i| self.model.with_item(i, |d| d.clone()))
                .collect()
        }

        pub fn for_each(&self, mut f: impl FnMut(&SearchResultDto)) {
            for i in 0..self.model.len() {
                self.model.with_item(i, &mut f);
            }
        }

        pub fn find(&self, pred: impl Fn(&SearchResultDto) -> bool) -> Option<SearchResultDto> {
            (0..self.model.len())
                .find_map(|i| self.model.with_item(i, |d| pred(d).then(|| d.clone()))?)
        }
    }

    /// A few fabricated rows so the search dock renders with `--features mocks`,
    /// mirroring the mock binder tree (`Chapitre 7`, a note, a title hit).
    fn mock_rows() -> Vec<SearchResultDto> {
        vec![
            SearchResultDto {
                id: 1,
                binder_item_id: 3,
                item_title: "Chapitre 7".to_string(),
                match_field: MatchField::Body,
                occurrence_count: 5,
                snippet_before: "Elle appela « ".to_string(),
                snippet_match: "Aurélien".to_string(),
                snippet_after: " ! » — sa voix se perdit.".to_string(),
                trashed: false,
                ..Default::default()
            },
            SearchResultDto {
                id: 2,
                binder_item_id: 4,
                item_title: "Notes de personnage".to_string(),
                match_field: MatchField::Synopsis,
                occurrence_count: 1,
                snippet_before: "La promesse d'".to_string(),
                snippet_match: "Aurélien".to_string(),
                snippet_after: " qu'il n'a jamais tenue.".to_string(),
                trashed: false,
                ..Default::default()
            },
            SearchResultDto {
                id: 3,
                binder_item_id: 5,
                item_title: "Aurélien".to_string(),
                match_field: MatchField::Title,
                occurrence_count: 1,
                snippet_before: String::new(),
                snippet_match: "Aurélien".to_string(),
                snippet_after: String::new(),
                trashed: false,
                ..Default::default()
            },
        ]
    }
}

pub use imp::SearchResultsModel;
