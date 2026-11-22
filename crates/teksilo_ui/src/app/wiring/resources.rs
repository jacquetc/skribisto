// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The Resources feature's per-window view-model: minted once, connected to what it
//! reaches through the window, and wired to the events it answers.
//!
//! - The project's resources model re-reads on its backend events (every window wires
//!   it; a subscription is scoped to the build that made it).
//! - A store run reports through the shared long-operation events, filtered by the
//!   view-model's own operation id.
//! - An edit another application made is picked up when this window regains focus
//!   (and before each save, by the hook `WorkSession` installs), and the notice it owes
//!   is raised by the window that handles the `Resource` update the pickup caused: the
//!   pickup itself runs where there is no `EventContext` to raise one with.

use std::rc::Rc;

use teksilo::prelude::*;

use frontend::AppContext;
use frontend::common::event::{DirectAccessEntity, EntityEvent, Event, LongOperationEvent, Origin};

use crate::editors::EditorsViewModel;
use crate::format::FormatViewModel;
use crate::resources::{ResourcesViewModel, SaveAsFolderFn};
use crate::sessions::WorkSession;

pub(in crate::app) fn install(
    // NOT pub(crate), matching every sibling in app/wiring/
    ctx: &mut BuildContext,
    cell: &mut Option<ResourcesViewModel>,
    app_ctx: &Rc<AppContext>,
    session: &WorkSession,
    editors: &EditorsViewModel,
    format: &FormatViewModel,
    save_as_folder: SaveAsFolderFn,
) -> ResourcesViewModel {
    let vm = cell
        .get_or_insert_with(|| {
            ResourcesViewModel::new(
                app_ctx.clone(),
                session.ids.clone(),
                session.resources.clone(),
                editors.active_item(),
                session.single_work_info.shape(),
                session.single_work.unique_id(),
            )
        })
        .clone();
    {
        let format = format.clone();
        let docs = session.open_docs.clone();
        vm.connect(
            Rc::new(move || format.writable_handle()),
            Rc::new(move || docs.media_dir()),
            save_as_folder,
        );
    }
    session.resources.model().wire(ctx);

    for (event, handler) in [
        (
            LongOperationEvent::Progress,
            ResourcesViewModel::on_long_op_progress
                as fn(&ResourcesViewModel, &mut EventContext, &Event),
        ),
        (
            LongOperationEvent::Completed,
            ResourcesViewModel::on_long_op_completed,
        ),
        (
            LongOperationEvent::Cancelled,
            ResourcesViewModel::on_long_op_cancelled,
        ),
        (
            LongOperationEvent::Failed,
            ResourcesViewModel::on_long_op_failed,
        ),
    ] {
        let vm = vm.clone();
        ctx.subscribe_event_with_ctx(Origin::LongOperation(event), move |e: &Event, c| {
            handler(&vm, c, e)
        });
    }

    {
        let vm = vm.clone();
        ctx.subscribe_event_with_ctx(
            Origin::DirectAccess(DirectAccessEntity::Resource(EntityEvent::Updated)),
            move |_e: &Event, c| vm.raise_change_notices(c),
        );
    }

    // The focus-regain edge: the writer comes back from the application they edited
    // the file in.
    {
        let resources = session.resources.clone();
        let was_active = std::cell::Cell::new(true);
        let active = ctx.window_active_signal();
        ctx.effect(&active, move |active| {
            let regained = *active && !was_active.get();
            was_active.set(*active);
            if regained {
                resources.pickup_changes();
            }
        });
    }
    vm
}
