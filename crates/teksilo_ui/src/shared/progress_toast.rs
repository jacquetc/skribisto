// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! A long operation's loading notice, and the notice that ends it.
//!
//! Not a view-model: a handle on one toast, held by the view-model whose operation
//! it reports (the export, the project importers, the backups).
//!
//! A progress notice is a [`Toast::loading`] raised under an id and updated in place
//! on every tick. teksilo keeps a toast's leading widget through an update in place
//! that brings none, so long as the severity stays the same: that is what keeps the
//! spinner turning from one tick to the next. A loading toast is `Info`, so a final
//! notice that is `Info` too (an operation cancelled, or one that ended with nothing
//! to read) keeps the spinner as well, and says the work is over while it still
//! spins. teksilo drops the spinner only when the severity changes, which is why a
//! success, a warning or a failure can update the notice in place.
//!
//! So an `Info` ending takes the loading notice down and raises the final one under
//! the same id: the notification log keeps the operation's one row, and nothing
//! spins once the operation is over.

use std::cell::RefCell;
use std::rc::Rc;

use teksilo::prelude::*;
use teksilo::widgets::{Toast, ToastHandle};

/// The loading notice of the operation running now. Cheap to clone; clones share
/// the notice.
#[derive(Clone, Default)]
pub(crate) struct ProgressToast {
    /// The notice on screen, from its last raise or update. `None` once the
    /// operation has ended.
    shown: Rc<RefCell<Option<ToastHandle>>>,
}

impl ProgressToast {
    /// Raise the loading notice, or update it in place: `toast` carries the
    /// operation's id and its spinner.
    pub(crate) fn show(&self, ctx: &mut EventContext, toast: Toast) {
        let handle = ctx.show_toast(toast);
        *self.shown.borrow_mut() = Some(handle);
    }

    /// End the operation with `toast`, a success, a warning or a failure under the
    /// operation's id: it takes over the loading notice in place, and the change of
    /// severity takes the spinner away.
    pub(crate) fn end_in_place(&self, ctx: &mut EventContext, toast: Toast) {
        self.shown.borrow_mut().take();
        ctx.show_toast(toast);
    }

    /// End the operation with `toast`, an `Info` notice under the operation's id:
    /// the loading notice goes, then `toast` is raised in its place. Updated in
    /// place, it would keep the spinner (see the module doc).
    pub(crate) fn end_replacing(&self, ctx: &mut EventContext, toast: Toast) {
        self.dismiss(ctx);
        ctx.show_toast(toast);
    }

    /// End the operation with no notice of its own, because a dialog raised in
    /// the same breath says how it ended: the loading notice goes, and nothing
    /// takes its place. A spinner left under that dialog would say the work is
    /// still going on.
    pub(crate) fn dismiss(&self, ctx: &mut EventContext) {
        let shown = self.shown.borrow_mut().take();
        if let Some(shown) = shown {
            shown.dismiss(ctx);
        }
    }
}
