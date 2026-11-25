// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The Scratchpad's per-window view-model: minted once, and told on every build what is
//! in front of the writer.
//!
//! - **The view in front**: the focused pane's active tab item and whether that tab shows
//!   a stream. Pushed on the two edges at which the active tab can change (its item, and
//!   the pane with focus), and then on every switch of that tab's segment bar, through an
//!   observer re-pointed at each new tab.
//! - **The caret**: the window formatting view-model's live `caret_item`, from which the
//!   view-model keeps the stream row the writer was last in.
//! - **The name**: re-read on every binder item update (a rename, a renumbering).
//! - **A Promote**: polled once a frame, since the document is rebuilt by other handlers
//!   of the same event, in an order nothing here can rely on.
//! - **The project closing** in this window gives its reference back. The window going
//!   away does too, through `build_window_teardown`.

use std::cell::RefCell;
use std::rc::Rc;

use teksilo::core::ObserverHandle;
use teksilo::prelude::*;

use frontend::AppContext;
use frontend::common::event::{DirectAccessEntity, EntityEvent, Event, Origin};

use crate::editors::EditorsViewModel;
use crate::format::FormatViewModel;
use crate::scratchpad::{ScratchpadViewModel, is_stream_segment};
use crate::sessions::WorkSession;

pub(in crate::app) fn install(
    // NOT pub(crate), matching every sibling in app/wiring/
    ctx: &mut BuildContext,
    cell: &mut Option<ScratchpadViewModel>,
    app_ctx: &Rc<AppContext>,
    session: &WorkSession,
    editors: &EditorsViewModel,
    format: &FormatViewModel,
) -> ScratchpadViewModel {
    let vm = cell
        .get_or_insert_with(|| {
            ScratchpadViewModel::new(
                app_ctx.clone(),
                session.open_docs.clone(),
                session.ids.clone(),
            )
        })
        .clone();

    // The view in front, and the segment bar of the tab showing it.
    {
        let segment_watch: Rc<RefCell<Option<ObserverHandle>>> = Rc::new(RefCell::new(None));
        let view_changed: Rc<dyn Fn()> = {
            let vm = vm.clone();
            let editors = editors.clone();
            let caret = format.caret_item();
            Rc::new(move || {
                let segment = editors.focused_segment_signal();
                let in_stream = segment.as_ref().is_some_and(|s| is_stream_segment(s.get()));
                // Re-pointed at this tab's bar; the previous tab's observer is dropped.
                *segment_watch.borrow_mut() = segment.map(|s| {
                    let vm = vm.clone();
                    s.observe(move |seg| vm.segment_changed(is_stream_segment(*seg)))
                });
                vm.view_changed(
                    editors.active_context().get().map(|item| item.id),
                    in_stream,
                    caret.get(),
                );
            })
        };
        view_changed();
        {
            let f = view_changed.clone();
            ctx.effect(&editors.active_context(), move |_| f());
        }
        ctx.effect(&editors.focused_side_signal(), move |_| view_changed());
    }

    // Where the caret is.
    {
        let vm = vm.clone();
        ctx.effect(&format.caret_item(), move |item| vm.caret_moved(*item));
    }

    // The name in the header.
    {
        let vm = vm.clone();
        ctx.subscribe_event(
            Origin::DirectAccess(DirectAccessEntity::BinderItem(EntityEvent::Updated)),
            move |_e: &Event| vm.refresh_title(),
        );
    }

    // A Promote's rebuilt document.
    {
        let vm = vm.clone();
        let tick = ctx.frame_tick();
        ctx.effect(&tick, move |_| vm.poll_document());
    }

    // This window's own project closing: nothing of it is shown any more.
    {
        let vm = vm.clone();
        super::project_events::on_own_close(ctx, &session.ids, move |_e| vm.release());
    }
    vm
}
