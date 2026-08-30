// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The **Undo** button a toast offers for one specific operation.
//!
//! Not a view-model — a two-function module the eight destructive-op toasts
//! share so they cannot drift apart again.
//!
//! # Why this exists
//!
//! Every one of those toasts used to call `undo_redo_commands::undo(ctx, stack)`
//! — *pop whatever is on top*. But the toast is offered **for one operation**,
//! and the stack keeps moving underneath it: prose autosave used to push a
//! `Content::update` every few seconds, a second window on the same `Work`
//! shares the stack, and a rename committed on blur lands there too. So
//! *trash a folder → keep typing → press Undo* undid the autosave and left the
//! folder trashed. `story_bible/capture_flow.rs` documented the hazard and
//! bounded its toast to ten seconds to shrink the window, which narrows the
//! race without closing it.
//!
//! Naming the operation closes it. [`stamp`] records the sequence number the
//! command landed on; [`undo_action`] hands it back to `undo_if_head`, which
//! undoes that command **or refuses**, and the refusal is a message rather than
//! the wrong thing silently happening.

use std::rc::Rc;

use frontend::AppContext;
use frontend::commands::undo_redo_commands;
use frontend::common::undo_redo::UndoStatus;
use teksilo::i18n::LocalizedString;
use teksilo::prelude::*;
use teksilo::widgets::{Toast, ToastAction};

/// The sequence number the command that just ran landed on.
///
/// **Call it immediately after the call that pushed** — every later push
/// overwrites it. `None` means nothing was recorded (the write was untracked,
/// or it was folded into a composite that is still open), in which case there
/// is no operation to offer back and the caller should not build an Undo
/// button at all.
pub(crate) fn stamp(app_ctx: &AppContext) -> Option<u64> {
    undo_redo_commands::last_pushed_seq(app_ctx)
}

/// The handler behind a toast's **Undo**, bound to the one operation `seq`
/// names.
///
/// Returns the closure rather than a finished [`ToastAction`] so each site
/// keeps its own button style — the story-bible capture offers Undo as a
/// secondary beside "Open", where trash offers it as the primary.
///
/// `on_undone` runs only when the undo actually happened, for whatever
/// resyncing a given feature needs afterwards (reloading a list, re-running a
/// search).
pub(crate) fn undo_handler(
    app_ctx: Rc<AppContext>,
    stack: Option<u64>,
    seq: Option<u64>,
    on_undone: impl Fn(&mut EventContext) + 'static,
) -> impl Fn(&mut EventContext) + 'static {
    move |c: &mut EventContext| {
        let Some(seq) = seq else {
            // Nothing was recorded, so there is nothing to name and nothing to
            // take back. Saying so beats a button that quietly does nothing.
            superseded(c);
            return;
        };
        match undo_redo_commands::undo_if_head(&app_ctx, stack, seq) {
            Ok(UndoStatus::Undone) => on_undone(c),
            // Both non-undone outcomes say the same true thing to a writer:
            // this is no longer the step at the top, so taking it back is not
            // what Undo would do. Distinguishing "superseded" from "empty"
            // would be a distinction about our stack, not about their project.
            Ok(UndoStatus::Superseded | UndoStatus::Empty) => superseded(c),
            Err(e) => {
                c.show_toast(Toast::error(tr!(undo_failed(error = e.to_string()))));
            }
        }
    }
}

/// [`undo_handler`] wrapped as a toast's primary button — the common case.
pub(crate) fn undo_action(
    app_ctx: Rc<AppContext>,
    stack: Option<u64>,
    seq: Option<u64>,
    label: LocalizedString,
    on_undone: impl Fn(&mut EventContext) + 'static,
) -> ToastAction {
    ToastAction::primary(label, undo_handler(app_ctx, stack, seq, on_undone))
}

/// Say that the step has moved on. Never silently do nothing: a button that
/// appears to work and does not is worse than one that explains itself.
fn superseded(c: &mut EventContext) {
    c.show_toast(Toast::info(tr!(undo_superseded_title())).body(tr!(undo_superseded_body())));
}

#[cfg(all(test, not(feature = "mocks")))]
mod tests {
    use frontend::commands::{binder_commands, undo_redo_commands, work_commands};
    use frontend::direct_access::{CreateBinderDto, CreateWorkDto};

    use super::*;

    /// Two undoable commands on one stack, and the sequence of each.
    fn two_commands(ctx: &AppContext, stack: u64) -> (u64, u64) {
        let work = work_commands::create_orphan_work(ctx, None, &CreateWorkDto::default()).unwrap();
        let mut seqs = Vec::new();
        for name in ["first", "second"] {
            binder_commands::create_binder(
                ctx,
                Some(stack),
                &CreateBinderDto {
                    name: name.into(),
                    activated: true,
                    ..Default::default()
                },
                work.id,
                0,
            )
            .unwrap();
            seqs.push(stamp(ctx).expect("a recorded command has a sequence"));
        }
        (seqs[0], seqs[1])
    }

    /// The contract every one of the eight toasts rests on: the sequence taken
    /// straight after a command names *that* command, and stops naming it the
    /// moment anything else is pushed.
    ///
    /// This is the whole bug in miniature. Before it, a toast's Undo popped the
    /// top of the stack, so a project autosave, a rename committed on blur, or
    /// a second window on the same `Work` could slide in between the offer and
    /// the click — and the button would then reverse *that* instead, silently,
    /// while the writer believed they had taken back the thing the toast named.
    #[test]
    fn a_stamp_names_its_own_command_and_stops_when_another_lands() {
        let ctx = AppContext::new();
        let stack = undo_redo_commands::create_new_stack(&ctx);
        let (mine, theirs) = two_commands(&ctx, stack);
        assert_ne!(mine, theirs);

        assert_eq!(
            undo_redo_commands::undo_if_head(&ctx, Some(stack), mine).unwrap(),
            UndoStatus::Superseded,
            "the toast must refuse rather than undo the newer command"
        );
        assert_eq!(
            undo_redo_commands::get_stack_size(&ctx, stack),
            2,
            "and refusing must change nothing"
        );

        // Once the newer command is gone, the named one is reachable again.
        undo_redo_commands::undo(&ctx, Some(stack)).unwrap();
        assert_eq!(
            undo_redo_commands::undo_if_head(&ctx, Some(stack), mine).unwrap(),
            UndoStatus::Undone
        );
        assert_eq!(undo_redo_commands::get_stack_size(&ctx, stack), 0);
        assert_eq!(
            undo_redo_commands::undo_if_head(&ctx, Some(stack), mine).unwrap(),
            UndoStatus::Empty,
            "and an exhausted stack says so rather than silently doing nothing"
        );
    }

    /// A write routed to the untracked stack records nothing, so there is no
    /// sequence to offer and the caller must not build an Undo button at all.
    #[test]
    fn an_untracked_write_leaves_nothing_to_name() {
        let ctx = AppContext::new();
        let work =
            work_commands::create_orphan_work(&ctx, None, &CreateWorkDto::default()).unwrap();
        binder_commands::create_binder(
            &ctx,
            Some(frontend::common::undo_redo::UNTRACKED_STACK_ID),
            &CreateBinderDto {
                name: "mirror".into(),
                activated: true,
                ..Default::default()
            },
            work.id,
            0,
        )
        .unwrap();
        assert_eq!(stamp(&ctx), None);
    }
}
