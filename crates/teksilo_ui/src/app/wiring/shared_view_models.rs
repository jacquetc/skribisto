// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Get-or-create for the three feature view-models the docking rail shares
//! across builds: search/replace, trash and comments. Each is minted once
//! (held in an `App` field cell) and re-fetched on every rebuild, which is
//! why the three lookups were grouped here rather than living next to the
//! docks that use them.

use std::rc::Rc;

use teksilo::widgets::DockWidgetId;

use frontend::AppContext;

use crate::binder::OutlineViewModel;
use crate::models::OpenDocsStore;
use crate::search::SearchReplaceViewModel;

use super::super::open_search_settings;

/// The three shared view-models `get_or_create` resolves.
pub(in crate::app) struct SharedFeatureViewModels {
    pub search: SearchReplaceViewModel,
    pub trash: crate::trash::TrashViewModel,
    pub comments: crate::comments::CommentsViewModel,
}

#[allow(clippy::too_many_arguments)]
pub(in crate::app) fn get_or_create(
    app_ctx: &Rc<AppContext>,
    outline: &OutlineViewModel,
    docs: &OpenDocsStore,
    search_dock: DockWidgetId,
    preview_dock: DockWidgetId,
    trash_dock: DockWidgetId,
    search_slot: &mut Option<SearchReplaceViewModel>,
    trash_slot: &mut Option<crate::trash::TrashViewModel>,
    comments_slot: &mut Option<crate::comments::CommentsViewModel>,
) -> SharedFeatureViewModels {
    // ── The search feature's shared view-model (both docks clone it) ──────
    // Created once; it holds the results model, the persisted `search.toml`
    // service, the shared open-docs store (so its preview edits the same
    // document a tab does), and the DockingModel (to reveal the bottom band).
    let search = {
        let app_ctx = app_ctx.clone();
        let ids = outline.ids();
        let docs = docs.clone();
        let docking = outline.docking();
        search_slot
            .get_or_insert_with(|| {
                let settings_svc = open_search_settings();
                let results = crate::models::SearchResultsModel::new(
                    app_ctx.clone(),
                    ids.work_info_id.clone(),
                );
                SearchReplaceViewModel::new(
                    app_ctx,
                    ids,
                    results,
                    settings_svc,
                    docs,
                    docking,
                    preview_dock,
                    search_dock,
                )
            })
            .clone()
    };

    // ── The trash feature's shared view-model ────────────────────────────
    // Shares the outline's DockingModel (the leading rail hosts several tabs)
    // and a distinct dock id; created once, not registered as app_state.
    let trash = {
        let app_ctx = app_ctx.clone();
        let ids = outline.ids();
        let docking = outline.docking();
        trash_slot
            .get_or_insert_with(|| {
                let model =
                    crate::models::TrashTreeModel::new(app_ctx.clone(), ids.work_id.clone());
                crate::trash::TrashViewModel::new(app_ctx, ids, model, docking, trash_dock)
            })
            .clone()
    };
    // ── The comments feature's shared view-model ─────────────────────────
    // One instance for both docks: the project-wide dock binds the whole list,
    // the per-document dock the same handle narrowed to the focused item. Two
    // models over one entity set would drift the moment a thread was resolved
    // in one and not the other.
    let comments = {
        let app_ctx = app_ctx.clone();
        let ids = outline.ids();
        comments_slot
            .get_or_insert_with(|| {
                let model = crate::models::CommentsListModel::new(app_ctx.clone(), ids.clone());
                crate::comments::CommentsViewModel::new(model, app_ctx, ids.stack_id.clone())
            })
            .clone()
    };

    SharedFeatureViewModels {
        search,
        trash,
        comments,
    }
}
