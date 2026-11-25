// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The Scratchpad's per-window view-model: minted once, and told on every build what is
//! in front of the writer.
//!
//! - **The view in front**: the focused pane, its active tab item and the segment its
//!   bar shows. Pushed on the two edges at which the active tab can change (its item,
//!   and the pane with focus), and then on every switch of that tab's segment bar,
//!   through an observer re-pointed at each new tab. Those edges fire on every click
//!   into a pane; the view-model ignores a push that names the view already in front.
//! - **The caret**: the window formatting view-model's live `caret_item`, from which the
//!   view-model keeps the stream row the writer was last in, when the editor belongs to
//!   the tab in front.
//! - **The binder moving**: a rename, a renumbering, a row trashed, deleted or moved out
//!   of the stream in front. The name in the header is re-read, and a row no longer in
//!   the stream is let go. The same structural events the open tabs re-caption on
//!   ([`crate::editors::structural_origins`]).
//! - **A Promote**: the store says when it rebuilt a document, and the view-model
//!   observes that itself.
//! - **The project closing** in this window gives its reference back. The window going
//!   away does too, through `build_window_teardown`, on the window's own stack.

use std::cell::RefCell;
use std::rc::Rc;

use teksilo::core::ObserverHandle;
use teksilo::prelude::*;

use frontend::AppContext;
use frontend::common::event::{
    BinderItemManagementEvent, DirectAccessEntity, EntityEvent, Event, Origin,
};

use crate::editors::EditorsViewModel;
use crate::format::FormatViewModel;
use crate::scratchpad::{FrontView, ScratchpadViewModel};
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
                // Re-pointed at this tab's bar; the previous tab's observer is dropped.
                *segment_watch.borrow_mut() = segment.as_ref().map(|s| {
                    let vm = vm.clone();
                    s.observe(move |seg| vm.segment_changed(*seg))
                });
                let view = FrontView {
                    active: editors.active_context().get().map(|item| item.id),
                    pane: editors.focused_side(),
                    segment: segment.and_then(|s| s.get()),
                };
                vm.view_changed(view, caret.get());
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
        ctx.effect(&format.caret_item(), move |caret| vm.caret_moved(*caret));
    }

    // The binder moving: the name in the header, and whether the row is still there.
    {
        let vm = vm.clone();
        ctx.subscribe_event(
            Origin::DirectAccess(DirectAccessEntity::BinderItem(EntityEvent::Updated)),
            move |e: &Event| vm.items_updated(&e.ids),
        );
    }
    {
        let vm = vm.clone();
        ctx.subscribe_event(
            Origin::DirectAccess(DirectAccessEntity::BinderItem(EntityEvent::Removed)),
            move |_e: &Event| vm.structure_changed(),
        );
    }
    for origin in crate::editors::structural_origins() {
        let vm = vm.clone();
        ctx.subscribe_event(origin, move |_e: &Event| vm.structure_changed());
    }
    // A merge names the scene that stays and the one merged into it, in that order.
    {
        let vm = vm.clone();
        ctx.subscribe_event(
            Origin::BinderItemManagement(BinderItemManagementEvent::MergeTwoScenes),
            move |e: &Event| {
                if let [survivor, merged, ..] = e.ids[..] {
                    vm.scenes_merged(survivor, merged);
                }
            },
        );
    }

    // This window's own project closing: nothing of it is shown any more.
    {
        let vm = vm.clone();
        super::project_events::on_own_close(ctx, &session.ids, move |_e| vm.release());
    }
    vm
}
