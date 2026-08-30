// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Bringing a **deleted** row back: the guarded sequence around the create.
//!
//! The sibling of [`super::restore_version`], and deliberately not part of it.
//! A restore overwrites text the writer still has; this adds a row they no
//! longer do. The two differ at every step that matters, so folding them into
//! one function would mean a chain of "unless we are recreating" branches
//! through the part of the app least allowed to be subtle:
//!
//! ```text
//!   a backup window?  ─┐  yes?               → refuse; this is not their project
//!   is the uid live?  ─┤  yes?               → say so; never two rows on one uid
//!   resolve the place ─┤  nowhere to put it? → refuse, by name
//!   confirm           ─┤  cancelled?         → nothing happened
//!   read the prose    ─┤  unreadable?        → refuse, by name; create nothing
//!   create + write    ─┘  one composite      → then: one Undo takes it all back
//! ```
//!
//! **No safety copy.** [`super::restore_version`] takes one because it destroys
//! text; this only adds, inside one `begin_composite`/`end_composite` pair, so
//! the whole thing comes back out in one step. Asking for a backup here would
//! make a purely additive action fail on the three conditions `backup_now`
//! returns early on, for nothing.
//!
//! ⚠ **That one step is the toast's Undo, not Ctrl+Z**, and the difference is
//! the confirmation's to state honestly. `restore_version` may promise Ctrl+Z
//! because what it does is a *document* edit inside a composite edit block, and
//! the focused editor's undo reverses it. A create is an **entity** write on the
//! Work's undo stack, which no keystroke in this app is bound to — every other
//! entity operation (trash, comments, footnotes) offers it as a toast action for
//! exactly that reason, and so does this. Saying "Ctrl+Z" here would be a
//! promise the keyboard does not keep.
//!
//! ## Why the uid is reused
//!
//! A recreated row keeps the uid the version recorded, so it comes back as
//! *itself*: its own recorded past reappears in the Versions dock, and anything
//! else keyed to that uid points at a row again instead of at nothing.
//! `with_identity` mints only over a nil uid, so a supplied one survives.
//!
//! What that costs if it goes wrong is why [`live_uids`] runs first. Two rows
//! sharing a uid is not a cosmetic clash: `VersionIndex::row` takes the first
//! match, `history` keys every entry on it, and comment sidecars are addressed by
//! it — so the second row would silently inherit the first's whole past. A stale
//! change list is enough to reach that (a row trashed, then restored from the
//! trash while the band still showed it removed), so the check is against the
//! live tree at the moment of the click, not against what the scan believed.
//!
//! ## Why the prose is read inline
//!
//! It is a zip open on the UI thread, which `open_past` was moved off — but the
//! cost profile is not the same one. That read fires on **every click** in the
//! change list, while this one happens once, after a writer has read the text
//! and confirmed a dialog, on a bundle the reader opened seconds earlier. Doing
//! it here is also what lets an unreadable archive refuse *before* anything is
//! created: a resumption path would have the row already made, and would then
//! have to unmake it. The destructive sibling writes inline for the same reason.

use std::collections::HashSet;
use std::rc::Rc;

use teksilo::prelude::*;
use teksilo::widgets::{MessageBox, MessageBoxButtons, StandardButton, Toast};

use frontend::AppContext;
use frontend::commands::{
    binder_commands, binder_item_commands, undo_redo_commands, work_commands,
};
use frontend::common::direct_access::binder::BinderRelationshipField;
use frontend::common::direct_access::work::WorkRelationshipField;
use frontend::common::entities::{BinderItemRole, BinderItemSubRole, ContentRole};
use frontend::direct_access::CreateBinderItemDto;
use frontend::trash_management::DropPosition;
use skribisto_model::Relation;

use skrib_format::versions::{BackupVersions, VersionRef, VersionSource};

use crate::app_ids::AppIds;
use crate::binder::placement::{self, ItemMeta};
use crate::editors::EditorsViewModel;
use crate::models::OpenDocsStore;
use crate::toast_scope::ToastWorkExt;
use crate::versions::ProjectHandle;
use crate::versions::version_restore::{self, RecreateRequest};
use crate::widgets::destination_picker::BinderDestination;

/// One toast per feature, replacing its own rather than stacking.
const RECREATE_TOAST_ID: &str = "versions.recreated";

/// How long the "Brought back — Undo" snackbar stays up.
const UNDO_GRACE: std::time::Duration = std::time::Duration::from_secs(10);

/// Why a row is not coming back.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RecreateRefusal {
    /// This window is showing a **backup**, not the writer's project.
    ///
    /// The same refusal [`super::restore_version`] makes through
    /// `SafetyBlocker::BackupFileOpen`, and for a sharper reason: a restore into
    /// a backup window merely edits a copy, while a *create* there mints a row
    /// with the recorded uid inside a bundle that is about to be swept by
    /// retention — and the project the writer meant still has nothing.
    InBackupFile,
    /// A live row already carries this uid — it is not gone after all.
    AlreadyHere,
    /// The chosen destination does not resolve to a place in a binder.
    NoDestination,
    /// The bundle holding the prose could not be read.
    Unreadable,
    /// The create itself failed.
    CreateFailed { reason: String },
}

/// The row to bring back, as one recorded moment described it.
///
/// Everything here comes off a **backup**'s own `items.ron`. The project's
/// history log records prose keyed by uid and nothing else — no title, no type,
/// no depth — so a row it alone remembers cannot be reconstructed at all,
/// whatever the code does. `timeline_vm::compare` only ever calls a row removed
/// when the moment was structural, i.e. a backup, so this can be built for every
/// removed row there is.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeletedRow {
    pub uid: uuid::Uuid,
    pub role: BinderItemRole,
    pub sub_role: BinderItemSubRole,
    /// The name it had at that moment. There is no live row to take one from.
    pub title: String,
    /// A Book's subtitle, empty for every other kind.
    pub sub_title: String,
    /// Whether it was included in exports when it was recorded — see
    /// [`skrib_format::versions::VersionRow::is_exportable`].
    pub is_exportable: bool,
    /// The bundle its prose is read out of.
    pub from: VersionRef,
    /// Every text it carried, as `(recorded content role, bundle-relative path)`.
    pub blobs: Vec<(ContentRole, String)>,
    pub taken_at: chrono::DateTime<chrono::Utc>,
}

/// Everything the sequence needs that it cannot reach on its own.
///
/// A struct rather than five positional arguments, because each is a handle that
/// has to be cloned into the confirmation's result closure — and a positional
/// list of clones is how the wrong undo stack gets passed.
#[derive(Clone)]
pub struct RecreateContext {
    pub app_ctx: Rc<AppContext>,
    pub docs: OpenDocsStore,
    pub editors: EditorsViewModel,
    pub ids: AppIds,
    /// Whether this window is showing a backup rather than the project itself.
    pub backup_mode: Signal<bool>,
    /// Where this project's past is kept — the destinations a backup is read from.
    pub project: ProjectHandle,
}

/// Ask to bring `row` back at `destination`, then — if everything holds — do it.
///
/// Every guard runs **twice**: once here so a doomed request costs a sentence
/// rather than a dialog, and once inside `on_result` because that is the only
/// place their answers are still true. A `MessageBox` is per *window* and a
/// `Work` can span several (Work ▸ New Window), so between the question and the
/// answer the row can be back — brought back from the other window, or restored
/// out of the trash — the anchor can be trashed, and this window can enter
/// backup mode. Deciding before the question and writing on that decision is how
/// two rows end up sharing one uid.
pub fn recreate_row(
    ctx: &mut EventContext,
    cx: RecreateContext,
    row: DeletedRow,
    destination: BinderDestination,
) {
    if let Some(why) = blocked(&cx, &row, &destination) {
        return refuse(ctx, cx.ids.work_id.get(), why);
    }

    let when = row.taken_at.format("%Y-%m-%d %H:%M").to_string();
    let shown_title = display_title(&row.title);
    let where_to = destination.title.clone();
    MessageBox::question(tr!(versions_recreate_confirm_title(
        item = shown_title.clone()
    )))
    .text(tr!(versions_recreate_confirm_text(
        date = when,
        destination = where_to
    )))
    .informative_text(tr!(versions_recreate_confirm_undo_note()))
    .buttons(MessageBoxButtons::Custom(vec![
        StandardButton::Ok.into(),
        StandardButton::Cancel.into(),
    ]))
    .default_button(StandardButton::Ok)
    .escape_button(StandardButton::Cancel)
    .on_result(move |r, c| {
        if r.button != StandardButton::Ok {
            return;
        }
        // Re-read, never reuse — see this function's own docs.
        let work = cx.ids.work_id.get();
        if let Some(why) = blocked(&cx, &row, &destination) {
            return refuse(c, work, why);
        }
        let Some(place) = resolve_place(&cx.app_ctx, &destination) else {
            return refuse(c, work, RecreateRefusal::NoDestination);
        };
        // Read first, create second. An unreadable archive must not leave a row
        // behind full of empty prose: a bundle on an unplugged drive would then
        // be indistinguishable from a scene the writer had emptied themselves.
        let Some(prose) = read_all(&cx.project, &row.from, &row.blobs) else {
            return refuse(c, work, RecreateRefusal::Unreadable);
        };
        let req = RecreateRequest {
            uid: row.uid,
            role: row.role.clone(),
            sub_role: row.sub_role.clone(),
            title: row.title.clone(),
            sub_title: row.sub_title.clone(),
            is_exportable: row.is_exportable,
            prose,
            taken_at: row.taken_at,
        };
        commit(c, &cx, &req, place, &shown_title);
    })
    .present(ctx);
}

/// Why this recreation cannot go ahead **right now**, or `None`.
///
/// Everything here is read live and nothing is cached: that is the whole point
/// of it being a function rather than three checks written once at the top.
fn blocked(
    cx: &RecreateContext,
    row: &DeletedRow,
    destination: &BinderDestination,
) -> Option<RecreateRefusal> {
    if cx.backup_mode.get() {
        return Some(RecreateRefusal::InBackupFile);
    }
    // Against the live tree, never against the scan the change list was built
    // from: see the module docs for what a second row on one uid costs.
    if live_uids(&cx.app_ctx, cx.ids.work_id.get()).contains(&row.uid) {
        return Some(RecreateRefusal::AlreadyHere);
    }
    if resolve_place(&cx.app_ctx, destination).is_none() {
        return Some(RecreateRefusal::NoDestination);
    }
    None
}

/// The create and the writes, as **one** undo entry.
///
/// A composite rather than a new backend use case, exactly as
/// `goals::distribute_panel::commit_plan` does it: the pieces are an ordinary
/// create plus the same `version_restore::apply` a restore already uses, and the
/// only thing missing was that they landed as several entries. Without the pair,
/// one Undo leaves a row that exists and is empty — the worst halfway state for
/// an operation whose promise is that it can be taken back.
fn commit(
    ctx: &mut EventContext,
    cx: &RecreateContext,
    req: &RecreateRequest,
    place: (u64, usize, i64),
    shown_title: &str,
) {
    let (binder, index, indent) = place;
    // Threaded from `AppIds`, never defaulted: `None` leaks into the
    // never-cleared global stack 0, and the wrong one bleeds undo across Works.
    let stack = cx.ids.stack_id.get();
    let now = chrono::Utc::now();
    let _ = undo_redo_commands::begin_composite(&cx.app_ctx, stack);
    let dto = CreateBinderItemDto {
        // A row brought back from a backup arrives **unmarked**, deliberately.
        //
        // `DeletedRow` records no status, and recording one would not help: a backup's
        // `items.ron` names its rung by *file id*, and every id is re-minted on each
        // `load_work`, so the number in a month-old bundle addresses nothing in the live
        // ladder. Matching by name instead would be a guess — the writer may have renamed,
        // merged or deleted that rung since — and a wrong stage is worse than none, because
        // it is the field they sort and filter on. So the row comes back with its prose and
        // its title, and the writer says where it stands.
        status: None,
        created_at: now,
        updated_at: now,
        // Kept, not minted — see the module docs.
        uid: req.uid,
        title: req.title.clone(),
        sub_title: req.sub_title.clone(),
        role: req.role.clone(),
        sub_role: req.sub_role.clone(),
        activated: true,
        is_exportable: req.is_exportable,
        indent,
        ..Default::default()
    };
    let created = match binder_item_commands::create_binder_item(
        &cx.app_ctx,
        stack,
        &dto,
        binder,
        index as i32,
    ) {
        Ok(c) => c,
        Err(e) => {
            // Cancelled, not ended: a group left open would swallow every later
            // edit of the session into one undo entry.
            undo_redo_commands::cancel_composite(&cx.app_ctx);
            return refuse(
                ctx,
                cx.ids.work_id.get(),
                RecreateRefusal::CreateFailed {
                    reason: e.to_string(),
                },
            );
        }
    };
    // Each text through the same write a restore uses. A failure here is counted
    // and reported but does not abort: the row itself is back, and taking it away
    // again because one of its texts would not write is a worse answer than a row
    // plus a sentence saying how many are missing.
    let mut failed = 0usize;
    let work_uid = cx.editors.work_unique_id();
    for (role, text) in req.writable_prose() {
        if let Err(e) = version_restore::apply(
            &cx.docs,
            created.id,
            &role,
            &text,
            stack,
            work_uid.as_deref(),
        ) {
            eprintln!("recreate: writing {role:?} back failed: {e:?}");
            failed += 1;
        }
    }
    undo_redo_commands::end_composite(&cx.app_ctx);
    // Stamped here, between closing the group and saving — not later, beside
    // the toast. `request_save` flushes the open editors, and a title committed
    // by that flush is itself an undoable command, so a sequence read after it
    // would name the rename rather than the row this toast is about.
    let seq = crate::shared::undo_toast::stamp(&cx.app_ctx);
    // Immediately, not on the autosave debounce: the row and its prose are one
    // large undo entry, and leaving the window with it unsaved means a crash
    // loses the recovery.
    cx.editors.request_save();

    let app_ctx = cx.app_ctx.clone();
    let message = if failed > 0 {
        tr!(versions_recreated_partial_toast(
            item = shown_title.to_string(),
            count = failed as i64
        ))
    } else {
        tr!(versions_recreated_toast(item = shown_title.to_string()))
    };
    ctx.show_toast(
        Toast::success(message)
            .target_work(cx.ids.work_id.get())
            .scoped_id(RECREATE_TOAST_ID, cx.ids.work_id.get())
            .auto_dismiss_after(UNDO_GRACE)
            .action(crate::shared::undo_toast::undo_action(
                app_ctx,
                stack,
                seq,
                tr!(versions_undo()),
                |_c| {},
            )),
    );
}

/// `(binder, insert index, indent)` for a picked destination.
///
/// The picker speaks `DropPosition`, the binder speaks
/// [`skribisto_model::Relation`], and the translation is the one the picker's own
/// docs describe: a binder row means *into this binder*, a container row means
/// *inside it*, and `After` means *after it*.
///
/// `Before` has no translation and refuses. `Relation` cannot express it —
/// `Sibling` is by definition *after the anchor's whole subtree* — so the only
/// honest answers are "refuse" or "add a variant to `Relation`", and refusing is
/// the one that cannot put a row somewhere the writer did not ask for.
///
/// The row's **recorded indent is deliberately not used**. A depth is only
/// meaningful against the tree it was measured in, and that tree is exactly what
/// the row is no longer part of — the chapter it sat under may itself be gone.
/// The destination the writer pointed at decides, the same way it does for a
/// restore out of the trash and for an imported document.
fn resolve_place(ctx: &AppContext, destination: &BinderDestination) -> Option<(u64, usize, i64)> {
    let binder = destination.binder_id;
    if binder == 0 {
        return None;
    }
    let (order, meta) = ordered_meta(ctx, binder);
    let Some(anchor) = destination.anchor_item_id else {
        // A whole binder was chosen: at the end of it, top level.
        return Some((binder, order.len(), 0));
    };
    let pos = order.iter().position(|&x| x == anchor)?;
    let anchor_indent = meta.get(&anchor)?.1;
    let relation = match destination.position {
        DropPosition::Into => Relation::Child,
        DropPosition::After => Relation::Sibling,
        // Exhaustive on purpose. `Relation` has no "before" — `Sibling` is
        // defined as *after the anchor's entire subtree* — while
        // `binder_ordering::resolve_item_target`, which trash-restore and
        // drag-move both follow for this same enum, puts `Before` at a
        // genuinely earlier index. A catch-all arm here would silently place
        // the row on the wrong side of the anchor, and `Before` is
        // `DropPosition`'s own `#[default]`, so a default-constructed
        // destination would land in it. The picker only ever emits
        // `Into`/`After` today; if that changes, this refuses by name rather
        // than guessing.
        DropPosition::Before => return None,
    };
    let (index, indent) =
        placement::insertion_point_for_item(&order, &meta, pos, anchor_indent, relation);
    Some((binder, index, indent))
}

/// A binder's ordered item ids plus `{id -> (role, indent, sub_role)}`.
///
/// The same two reads `OutlineViewModel::ordered_meta` makes, repeated here
/// rather than reached for: that method is private to a Tier-3 view-model this
/// window need not even have built, and two `get_*_relationship` calls are
/// cheaper than widening a view-model's surface for one caller.
fn ordered_meta(ctx: &AppContext, binder: u64) -> (Vec<u64>, ItemMeta) {
    let order = binder_commands::get_binder_relationship(
        ctx,
        &binder,
        &BinderRelationshipField::BinderItems,
    )
    .unwrap_or_default();
    let meta = binder_item_commands::get_binder_item_multi(ctx, &order)
        .unwrap_or_default()
        .into_iter()
        .flatten()
        .map(|it| (it.id, (it.role, it.indent, it.sub_role)))
        .collect();
    (order, meta)
}

/// Every uid the project currently holds, across all of its binders.
///
/// All of them, not just the manuscript: a note or a research row can collide
/// exactly as a scene can, and a check that looked at one binder would let the
/// collision through for the other two a Novel project ships with.
fn live_uids(ctx: &AppContext, work_id: Option<u64>) -> HashSet<uuid::Uuid> {
    let Some(work_id) = work_id else {
        return HashSet::new();
    };
    let mut out = HashSet::new();
    let binders =
        work_commands::get_work_relationship(ctx, &work_id, &WorkRelationshipField::Binders)
            .unwrap_or_default();
    for binder in binders {
        let items = binder_commands::get_binder_relationship(
            ctx,
            &binder,
            &BinderRelationshipField::BinderItems,
        )
        .unwrap_or_default();
        out.extend(
            binder_item_commands::get_binder_item_multi(ctx, &items)
                .unwrap_or_default()
                .into_iter()
                .flatten()
                .map(|it| it.uid),
        );
    }
    out
}

/// Read every recorded blob out of one bundle.
///
/// `None` if **any** of them cannot be read: a row comes back whole or not at
/// all, and a half-read archive quietly producing a scene with no synopsis is
/// the class of loss this whole feature exists against.
fn read_all(
    handle: &ProjectHandle,
    from: &VersionRef,
    blobs: &[(ContentRole, String)],
) -> Option<Vec<(ContentRole, String)>> {
    let backups = BackupVersions {
        directories: handle.destinations.clone(),
        work_unique_id: handle.unique_id.clone(),
        project_path: handle.path.clone(),
    };
    let mut out = Vec::with_capacity(blobs.len());
    for (role, blob) in blobs {
        let text = backups.prose(from, blob).ok()?;
        out.push((role.clone(), text));
    }
    Some(out)
}

/// What to call a row that was never named.
///
/// A structural row usually carries no title of its own — every other surface
/// derives one — and a confirmation reading `Bring back ""?` is not a question.
fn display_title(title: &str) -> String {
    if title.trim().is_empty() {
        tr!(versions_recreate_untitled()).resolve_now()
    } else {
        title.to_string()
    }
}

/// Say why, in the writer's terms. Every refusal is named: a recovery that
/// silently does nothing is indistinguishable from one that silently did the
/// wrong thing.
fn refuse(ctx: &mut EventContext, work_id: Option<u64>, why: RecreateRefusal) {
    let message = match why {
        // The same sentence the restore path uses: the writer's problem is where
        // they are looking, which is one problem, not two.
        RecreateRefusal::InBackupFile => tr!(versions_restore_in_backup_file()),
        RecreateRefusal::AlreadyHere => tr!(versions_recreate_already_here()),
        RecreateRefusal::NoDestination => tr!(versions_recreate_no_destination()),
        RecreateRefusal::Unreadable => tr!(versions_recreate_unreadable()),
        RecreateRefusal::CreateFailed { reason } => tr!(versions_recreate_failed(error = reason)),
    };
    ctx.show_toast(
        Toast::error(message)
            .target_work(work_id)
            .scoped_id(RECREATE_TOAST_ID, work_id)
            .auto_dismiss_after(std::time::Duration::from_secs(8)),
    );
}
