// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The footnotes feature's per-window view-model: minted once, wired to the
//! backend, and handed to the open-document store so every editor can insert a
//! reference and report its caret without holding the view-model itself.
//!
//! Extracted from `App::build` so the view-model's own construction, wiring
//! and renumber-on-edit hookup read as one unit instead of three steps
//! scattered across the god-function.

use std::rc::Rc;

use teksilo::prelude::*;

use frontend::AppContext;

use crate::app_ids::AppIds;
use crate::footnotes::FootnotesViewModel;
use crate::models::OpenDocsStore;

pub(in crate::app) fn install(
    // NOT pub(crate) — matches every sibling in app/wiring/
    ctx: &mut BuildContext,
    cell: &mut Option<FootnotesViewModel>,
    app_ctx: &Rc<AppContext>,
    ids: &AppIds,
    docs: &OpenDocsStore,
) -> FootnotesViewModel {
    // ── The footnotes feature's view-model ───────────────────────────────
    // One per window, on the same footing as `comments` above: the dock binds
    // it, and every open document is handed a per-`Content` binding through
    // the store, so an editor can insert a reference and report its caret
    // without ever holding the view-model itself.
    let footnotes = {
        let app_ctx = app_ctx.clone();
        let ids = ids.clone();
        let docs = docs.clone();
        cell.get_or_insert_with(|| {
            let model =
                crate::models::FootnotesListModel::new(app_ctx.clone(), ids.clone(), docs.clone());
            FootnotesViewModel::new(model, docs, ids.stack_id.clone())
        })
        .clone()
    };
    footnotes.wire(ctx);
    // Hand it to the open-document store, which back-fills every document
    // already open (a workspace restore opens tabs before this point) and
    // seeds each one opened later — including the marker map, without which a
    // reference paints its raw label.
    docs.set_footnotes(footnotes.clone());
    // Renumber when a document's references move. The model's own gate makes
    // this cheap: it asks the open documents which notes they name, and only
    // walks the manuscript when that answer has changed.
    {
        let f = footnotes.clone();
        ctx.effect(&docs.edited_any(), move |_| f.note_live_edit());
    }
    footnotes
}
