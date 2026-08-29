// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! What "Add as note" does once the writer has picked a tag.
//!
//! The whole gesture is meant to be one click. The writer selects a name in their
//! prose, right-clicks, and picks a tag; the note exists, tagged, filed, and shaped
//! by whatever that tag starts its notes from. Nothing is asked, because the tag has
//! already answered everything a modal would have asked:
//!
//! | The old modal asked | The tag answers |
//! |---|---|
//! | which tags? | the one that was picked, and [`crate::story_bible::capture`] says why exactly one |
//! | where does it go? | `BinderTag.creates_in` |
//! | what does it start from? | `BinderTag.note_template` |
//! | what is it called? | the selection |
//!
//! **Once**, per tag, there is a question: a tag that has never filed a note does not
//! know where its notes go. That is [`ask_destination`], and the answer is written back
//! onto the tag, so the second capture under that tag is silent again. Untagged capture
//! asks the same question once per project, and keeps the answer in the writer's own
//! settings rather than in the project: an untagged note is not a story-bible decision,
//! so it leaves no trace in a file another writer might open.
//!
//! ## Why the destination lives on the tag, and not here
//!
//! It is the tag's own property, so it survives a reinstall, travels with the project,
//! and is editable in one obvious place (Settings, Work, Tags) rather than being an
//! invisible preference the writer has no way to find. It is also why setting it from
//! the one-time question lands on the undo stack: it is an edit to the project, and a
//! writer who answers the question by mistake must be able to take it back the same way
//! they take back anything else.

use std::rc::Rc;

use teksilo::core::modal::{ModalCloseBehavior, ModalPresentation, ModalRequest};
use teksilo::i18n::LocalizedString;
use teksilo::prelude::*;
use teksilo::widgets::{
    Button, ButtonVariant, HStack, MaxSize, ModalContainer, Padding, Spacer, TextWidget, Toast,
    ToastAction, VStack,
};

use frontend::AppContext;
use frontend::commands::{
    binder_commands, binder_item_commands, undo_redo_commands, work_commands,
};
use frontend::common::direct_access::binder::BinderRelationshipField;
use frontend::common::direct_access::work::WorkRelationshipField;
use frontend::trash_management::DropPosition;

use crate::app_ids::AppIds;
use crate::editors::EditorsViewModel;
use crate::models::NoteCaptureService;
use crate::note_templates::NoteTemplatesViewModel;
use crate::story_bible::create::{self, EntryDraft};
use crate::story_bible::modal;
use crate::tags::TagsViewModel;
use crate::toast_scope::ToastWorkExt;
use crate::widgets::destination_picker::{BinderDestination, DestinationPicker};

/// How long the "added to the story bible" snackbar keeps its Undo.
///
/// Bounded because `undo_redo_commands::undo` pops the top of the shared stack rather
/// than this specific note: after the writer has typed anything else, the top is no
/// longer the capture and the button would take back the wrong thing.
const UNDO_GRACE: std::time::Duration = std::time::Duration::from_secs(10);

/// Everything the capture action reads. Assembled once at registration, from the
/// same session view-models the modal used.
#[derive(Clone)]
pub struct CaptureDeps {
    pub app_ctx: Rc<AppContext>,
    pub ids: AppIds,
    pub tags: TagsViewModel,
    pub templates: NoteTemplatesViewModel,
    pub editors: EditorsViewModel,
    /// The writer's own capture memory: recents, and where untagged notes go. `None`
    /// when the settings directory is unreachable, which costs the ordering of the
    /// menu and the remembered untagged folder, and nothing else.
    pub capture: Option<NoteCaptureService>,
}

/// This project's uid and file path, the pair every `SettingsFile` sibling is keyed and
/// labelled by. `None` on a project that has never been saved, whose uid is not yet
/// usable as a key.
fn project_key(deps: &CaptureDeps) -> Option<(String, String)> {
    let work_id = deps.ids.work_id.get()?;
    let uid = work_commands::get_work(&deps.app_ctx, &work_id)
        .ok()
        .flatten()?
        .unique_id;
    crate::models::uid_is_usable(&uid).then(|| {
        (
            uid,
            crate::current_project_path(&deps.app_ctx, &deps.ids).unwrap_or_default(),
        )
    })
}

/// The whole gesture: file the selection as a note under `tag_id`.
///
/// `tag_id` is `None` for the menu's **Untagged** row, which is always offered and is
/// not a failure state. A writer capturing a stray thought has not decided they are
/// building a story bible, and a menu with no way through without picking a tag would
/// have got that moment wrong.
pub fn capture(
    deps: &CaptureDeps,
    item_id: u64,
    selected_text: &str,
    tag_id: Option<u64>,
    ctx: &mut EventContext,
) {
    let Some(prefill) =
        modal::prefill_from_selection(&deps.app_ctx, &deps.ids, item_id, selected_text)
    else {
        return;
    };
    if prefill.name.is_empty() {
        return;
    }
    match remembered_folder(deps, tag_id) {
        Some(folder) => commit(deps, &prefill, tag_id, folder, ctx),
        // Never filed under this tag before: ask, once.
        None => ask_destination(deps.clone(), prefill, tag_id, ctx),
    }
}

/// The folder this tag (or, untagged, this project) files notes into, if it has one.
fn remembered_folder(deps: &CaptureDeps, tag_id: Option<u64>) -> Option<u64> {
    match tag_id {
        Some(tag_id) => deps
            .tags
            .rows()
            .into_iter()
            .find(|r| r.id == tag_id)
            .and_then(|r| r.creates_in),
        None => {
            let uid = deps
                .capture
                .as_ref()?
                .untagged_folder(&project_key(deps)?.0)?;
            // Kept as a uid rather than an id: this one lives in the writer's own
            // settings file, which outlives the process that minted the ids in it.
            item_id_of_uid(&deps.app_ctx, deps.ids.work_id.get()?, uid)
        }
    }
}

/// `(binder, index, indent)` for "at the end of, and inside, this folder row".
///
/// Goes through [`BinderDestination`] rather than computing a place directly, so a
/// capture lands exactly where the picker would have put it had the writer chosen the
/// same row by hand. `None` when the folder is gone since it was remembered, which the
/// caller reads as "ask again" rather than guessing at another folder.
///
/// **Trashed counts as gone.** Trashing does not remove the row: `trash_binder_items_uc`
/// leaves it in the binder's order and only flips `activated` to false over the subtree,
/// and nothing sweeps the tag's `creates_in` because no relationship was removed. So
/// membership in the binder order is not the test it looks like: without the
/// `activated` check below, a capture lands inside the trash, reports success, is
/// invisible in the outline, and is destroyed by the next Empty trash. It is also what
/// the Settings pane already believes, since its own folder list is built from
/// `binder_stream::ordered_flat_items`, which drops trashed rows: with the tag's folder
/// trashed that pane shows the tag as unset, and this must agree with it.
fn place_inside(ctx: &AppContext, work_id: u64, folder: u64) -> Option<(u64, usize, i64)> {
    if !binder_item_commands::get_binder_item(ctx, &folder)
        .ok()
        .flatten()
        .is_some_and(|it| it.activated)
    {
        return None;
    }
    let binder = binder_of_item(ctx, work_id, folder)?;
    BinderDestination {
        binder_id: binder,
        anchor_item_id: Some(folder),
        position: DropPosition::Into,
        title: String::new(),
    }
    .resolve(ctx)
}

/// Which of the Work's binders holds this item.
///
/// All of them are searched, not just the manuscript: a project ships with three, and
/// a story bible most naturally lives in the research one. Scanned rather than read off
/// the item, because the ownership only exists as the binder's own ordered list.
fn binder_of_item(ctx: &AppContext, work_id: u64, item_id: u64) -> Option<u64> {
    for binder in
        work_commands::get_work_relationship(ctx, &work_id, &WorkRelationshipField::Binders)
            .unwrap_or_default()
    {
        let order = binder_commands::get_binder_relationship(
            ctx,
            &binder,
            &BinderRelationshipField::BinderItems,
        )
        .unwrap_or_default();
        if order.contains(&item_id) {
            return Some(binder);
        }
    }
    None
}

/// The live id for a durable uid, across every binder of the Work.
///
/// **Live**, in the same sense [`place_inside`] means it: a trashed row keeps its uid
/// and its place in the binder order, so matching on the uid alone would hand the
/// untagged path a destination inside the trash instead of asking the writer again.
fn item_id_of_uid(ctx: &AppContext, work_id: u64, uid: uuid::Uuid) -> Option<u64> {
    for binder in
        work_commands::get_work_relationship(ctx, &work_id, &WorkRelationshipField::Binders)
            .unwrap_or_default()
    {
        let order = binder_commands::get_binder_relationship(
            ctx,
            &binder,
            &BinderRelationshipField::BinderItems,
        )
        .unwrap_or_default();
        if let Some(found) = binder_item_commands::get_binder_item_multi(ctx, &order)
            .unwrap_or_default()
            .into_iter()
            .flatten()
            .find(|it| it.uid == uid && it.activated)
        {
            return Some(found.id);
        }
    }
    None
}

/// Create the note, remember the tag, and say so.
fn commit(
    deps: &CaptureDeps,
    prefill: &modal::Prefill,
    tag_id: Option<u64>,
    folder: u64,
    ctx: &mut EventContext,
) {
    let work_id = deps.ids.work_id.get();
    let draft = draft_for(prefill, tag_id, template_body(deps, tag_id));
    let Some(landed) =
        work_id.and_then(|w| file_note(&deps.app_ctx, deps.ids.stack_id.get(), w, &draft, folder))
    else {
        // Either the remembered folder is gone, or the write failed. The first is the
        // ordinary case (a writer reorganised their binder) and deserves the question
        // again rather than a note dropped somewhere they never chose; the second is
        // not, and says so.
        if work_id
            .and_then(|w| place_inside(&deps.app_ctx, w, folder))
            .is_none()
        {
            ask_destination(deps.clone(), prefill.clone(), tag_id, ctx);
        } else {
            ctx.show_toast(
                Toast::error(tr!(story_bible_modal_failed()))
                    .scoped_id("story_bible.failed", work_id)
                    .target_work(work_id),
            );
        }
        return;
    };
    remember_tag(deps, tag_id);
    let stack = deps.ids.stack_id.get();
    let app_ctx = deps.app_ctx.clone();
    let editors = deps.editors.clone();
    let title = prefill.name.clone();
    let item_id = landed.item_id;
    ctx.show_toast(
        Toast::success(tr!(story_bible_capture_toast(name = prefill.name.clone())))
            .target_work(work_id)
            .scoped_id("story_bible.captured", work_id)
            // `undo` pops whatever is on top of the shared stack, which is this capture
            // only until the writer does something else. The affordance therefore lives
            // exactly as long as it means what it says, the same grace
            // `app::recreate_row`'s own snackbar keeps for the identical reason.
            .auto_dismiss_after(UNDO_GRACE)
            .action(ToastAction::primary(
                tr!(story_bible_capture_open()),
                move |c| {
                    editors.activate(item_id, &title, c);
                },
            ))
            .action(ToastAction::new(
                tr!(story_bible_capture_undo()),
                move |_c| {
                    let _ = undo_redo_commands::undo(&app_ctx, stack);
                },
            )),
    );
    ctx.request_frame();
}

/// The draft a capture files: the selection as the title, the one chosen tag, and
/// whatever that tag's template starts its notes from.
///
/// Pure, and separate from the write below, because the two can be wrong independently:
/// this is where "exactly one tag" is actually enforced, and the write is where the row
/// lands in the right place.
fn draft_for(prefill: &modal::Prefill, tag_id: Option<u64>, body: String) -> EntryDraft {
    EntryDraft {
        title: prefill.name.clone(),
        // Exactly one, or none. See [`crate::story_bible::capture`] for why the menu
        // asks for a single tag rather than a set: one tag is what settles the
        // destination and the template, and two would settle them twice.
        tags: tag_id.into_iter().collect(),
        aliases: prefill.aliases.clone(),
        // **Not the Book the selection sat in**, however confidently that can be
        // worked out. `BinderItem.books` is the writer's declaration of what they have
        // filed, never an observation of where a row physically sits (the field's own
        // doc in `qleany.yaml` says so, and `tags::books` repeats it); the positional
        // answer already exists as `infer_book::book_containing` and is nobody's
        // filing. The modal door may pre-set it because it *shows* the chip row and
        // the writer presses Create with it on screen. This door shows nothing at all,
        // by design, so a guess written here is a filing the writer never made, never
        // saw, and would only discover by wondering why their entry vanished from a
        // Book's own grid. Empty is the honest state: "not yet filed", which the
        // Inspector's Books section and the entry's own Details page are both there to
        // change.
        books: Vec::new(),
        body,
    }
}

/// Land the draft inside `folder`, with no `EventContext` anywhere in it.
///
/// Split out of [`commit`] so the one thing that actually changes the project can be
/// tested against a real backend, which is the coverage the modal's own `Mode::Create`
/// test used to carry for this door.
///
/// `None` covers both "the remembered folder is gone" and "the write failed"; the caller
/// tells them apart, because one of them is an ordinary Tuesday and the other is not.
fn file_note(
    ctx: &Rc<AppContext>,
    stack: Option<u64>,
    work_id: u64,
    draft: &EntryDraft,
    folder: u64,
) -> Option<create::CreatedEntry> {
    let (binder, index, indent) = place_inside(ctx, work_id, folder)?;
    create::create_entry(ctx, stack, binder, index, indent, draft)
}

/// The Djot this tag's notes start from, or nothing.
fn template_body(deps: &CaptureDeps, tag_id: Option<u64>) -> String {
    tag_id
        .and_then(|id| deps.tags.rows().into_iter().find(|r| r.id == id))
        .and_then(|r| r.note_template)
        .and_then(|t| deps.templates.body_of(t))
        .unwrap_or_default()
}

/// Move this tag to the front of the project's recents, so the menu offers it next time
/// without the writer going looking. Untagged is not a tag and is never ranked: it has
/// its own permanent row at the bottom of the menu.
fn remember_tag(deps: &CaptureDeps, tag_id: Option<u64>) {
    let (Some(svc), Some(tag_id)) = (deps.capture.as_ref(), tag_id) else {
        return;
    };
    let Some((uid, path)) = project_key(deps) else {
        return;
    };
    let tag_uid = deps
        .tags
        .rows()
        .into_iter()
        .find(|r| r.id == tag_id)
        .map(|r| r.uid);
    if let Some(tag_uid) = tag_uid
        && let Err(e) = svc.note_tag_used(&uid, &path, tag_uid)
    {
        eprintln!("story_bible: remembering the capture tag failed: {e}");
    }
}

/// The folder a pick names, or `None` when the pick is not a filing answer at all.
///
/// [`DestinationPicker::selected`] answers with a **position** as well as a row, and both
/// matter. Its own contract: a container row means *inside* it, anything else means
/// *after* it. Only the first is an answer here, because `creates_in` names a folder that
/// notes go into.
///
/// Reading the row and dropping the position takes a click on a scene as "file inside
/// this scene", writes that onto the tag, and never asks again, so every later capture
/// under that tag nests another note inside a scene of the manuscript, exportable. A
/// whole binder fails the same test from the other side: it answers `Into` but names no
/// row, and `creates_in` points at an item.
fn folder_answered_by(picked: Option<&BinderDestination>) -> Option<u64> {
    let picked = picked?;
    matches!(picked.position, DropPosition::Into)
        .then_some(picked.anchor_item_id)
        .flatten()
}

/// The one-time question: where do this tag's notes go?
///
/// Asked once and never again, because the answer is written back onto the tag. The
/// panel says so out loud, and says where to change it: a question a writer answers by
/// reflex is one they must be able to find and correct later.
fn ask_destination(
    deps: CaptureDeps,
    prefill: modal::Prefill,
    tag_id: Option<u64>,
    ctx: &mut EventContext,
) {
    let title = tr!(story_bible_capture_where_title());
    ctx.present_modal(
        ModalRequest::deferred({
            let title = title.clone();
            move |t| {
                t.add(
                    ModalContainer::new(WherePanel::new(deps.clone(), prefill.clone(), tag_id))
                        .title(title.clone()),
                )
            }
        })
        .presentation(ModalPresentation::InTree)
        .title(title)
        .size(460, 520)
        .close_behavior(ModalCloseBehavior::EscapeOrClickOutside),
    );
}

struct WherePanel {
    deps: CaptureDeps,
    prefill: modal::Prefill,
    tag_id: Option<u64>,
    picker: DestinationPicker,
    root_child: Option<WidgetId>,
}

impl WherePanel {
    fn new(deps: CaptureDeps, prefill: modal::Prefill, tag_id: Option<u64>) -> Self {
        let picker = DestinationPicker::new(deps.app_ctx.clone(), deps.ids.work_id.clone());
        Self {
            deps,
            prefill,
            tag_id,
            picker,
            root_child: None,
        }
    }

    /// The sentence under the tree: what this answer will be remembered as, and where
    /// to change it. Named for the tag when there is one, so the writer can tell "notes
    /// tagged Character" from "notes with no tag at all" without guessing.
    fn explanation(&self) -> LocalizedString {
        match self
            .tag_id
            .and_then(|id| self.deps.tags.rows().into_iter().find(|r| r.id == id))
        {
            Some(row) => tr!(story_bible_capture_where_tag(tag = row.name)),
            None => tr!(story_bible_capture_where_untagged()),
        }
    }
}

impl std::fmt::Debug for WherePanel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WherePanel").finish()
    }
}

impl Widget for WherePanel {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        let confirm = {
            let deps = self.deps.clone();
            let prefill = self.prefill.clone();
            let tag_id = self.tag_id;
            let picker = self.picker.clone();
            move |c: &mut EventContext| {
                let Some(folder) = folder_answered_by(picker.selected().as_ref()) else {
                    c.show_toast(
                        Toast::warning(tr!(story_bible_capture_where_needs_folder()))
                            .scoped_id("story_bible.capture_no_folder", deps.ids.work_id.get())
                            .target_work(deps.ids.work_id.get()),
                    );
                    return;
                };
                // One undo entry for the whole gesture. Answering the question and
                // filing the note are two writes, and a writer who presses Undo straight
                // afterwards means "no, not there" about both of them: taking back the
                // note while leaving the tag pointing at the folder they just rejected
                // would file every later capture there, silently, and never ask again.
                // Composites nest (`UndoRedoManager::begin_composite` counts a level),
                // so `create_entry`'s own composite simply joins this one.
                let stack = deps.ids.stack_id.get();
                let _ = undo_redo_commands::begin_composite(&deps.app_ctx, stack);
                if !remember_folder(&deps, tag_id, folder) {
                    undo_redo_commands::cancel_composite(&deps.app_ctx);
                    c.show_toast(
                        Toast::error(tr!(story_bible_modal_failed()))
                            .scoped_id("story_bible.failed", deps.ids.work_id.get())
                            .target_work(deps.ids.work_id.get()),
                    );
                    return;
                }
                c.dismiss_modal();
                commit(&deps, &prefill, tag_id, folder, c);
                undo_redo_commands::end_composite(&deps.app_ctx);
            }
        };
        let col = VStack::new()
            .spacing(12.0)
            .child(TextWidget::new(tr!(story_bible_capture_where_prompt())))
            .child(
                MaxSize::height(260.0).child(self.picker.view(tr!(story_bible_modal_no_binders()))),
            )
            .child(
                TextWidget::new(self.explanation())
                    .style(TextStyleRole::Tiny)
                    .color(TextRole::Secondary),
            )
            .child(
                HStack::new()
                    .spacing(8.0)
                    .child(Spacer::new())
                    .child(
                        Button::new(tr!(story_bible_modal_cancel()))
                            .variant(ButtonVariant::Plain)
                            .on_activate_fn(|c| c.dismiss_modal()),
                    )
                    .child(
                        Button::new(tr!(story_bible_capture_where_confirm()))
                            .variant(ButtonVariant::Filled)
                            .on_activate_fn(confirm),
                    ),
            );
        let root = ctx.add(Padding::uniform(4.0).child(col));
        self.root_child = Some(root);
        vec![root]
    }

    fn layout_response(&self, proposal: SizeProposal, ctx: &LayoutContext) -> LayoutResponse {
        self.root_child
            .and_then(|id| ctx.child_size(id, proposal))
            .map(LayoutResponse::from)
            .unwrap_or_else(|| proposal.resolve(0.0, 0.0).into())
    }

    fn children(&self) -> Vec<WidgetId> {
        self.root_child.into_iter().collect()
    }
}

/// Write the answer down: onto the tag when there is one, into the writer's own
/// settings when there is not.
///
/// The tag write goes on the undo stack, because it is an edit to the project and a
/// writer who answered by reflex must be able to take it back. The untagged answer does
/// not: it is a preference in a settings file, which no undo stack has ever covered.
#[must_use]
fn remember_folder(deps: &CaptureDeps, tag_id: Option<u64>, folder: u64) -> bool {
    match tag_id {
        Some(tag_id) => {
            deps.tags.set_creates_in(tag_id, Some(folder));
            true
        }
        None => {
            let (Some(svc), Some((uid, path))) = (deps.capture.as_ref(), project_key(deps)) else {
                // No settings file to remember it in. The note is still created; the
                // question simply comes back next time.
                return true;
            };
            let Some(folder_uid) = binder_item_commands::get_binder_item(&deps.app_ctx, &folder)
                .ok()
                .flatten()
                .map(|it| it.uid)
            else {
                return false;
            };
            if let Err(e) = svc.set_untagged_folder(&uid, &path, folder_uid) {
                eprintln!("story_bible: remembering the untagged folder failed: {e}");
            }
            true
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::entities::{BinderItemRole, BinderItemSubRole};
    use frontend::direct_access::{CreateBinderDto, CreateBinderItemDto, CreateWorkDto};

    /// A Work with two binders, because "which binder holds this item" is only a real
    /// question when there is more than one, and a story bible most naturally lives in
    /// the second.
    fn seed() -> (Rc<AppContext>, u64, u64, u64) {
        let ctx = Rc::new(AppContext::new());
        let work = work_commands::create_orphan_work(&ctx, None, &CreateWorkDto::default())
            .expect("create work");
        let binder = |name: &str| {
            binder_commands::create_binder(
                &ctx,
                None,
                &CreateBinderDto {
                    name: name.into(),
                    activated: true,
                    ..Default::default()
                },
                work.id,
                -1,
            )
            .expect("create binder")
            .id
        };
        let manuscript = binder("Manuscript");
        let research = binder("Research");
        (ctx, work.id, manuscript, research)
    }

    fn add(
        ctx: &Rc<AppContext>,
        binder: u64,
        title: &str,
        role: BinderItemRole,
        indent: i64,
    ) -> u64 {
        let now = chrono::Utc::now();
        binder_item_commands::create_binder_item(
            ctx,
            None,
            &CreateBinderItemDto {
                status: None,
                created_at: now,
                updated_at: now,
                title: title.into(),
                role,
                sub_role: BinderItemSubRole::Note,
                activated: true,
                indent,
                ..Default::default()
            },
            binder,
            -1,
        )
        .expect("create item")
        .id
    }

    /// Trash `item` the way the binder does: the row keeps its uid and its place in
    /// the binder's order, and only `activated` flips. That is exactly what makes
    /// "is it still in the order" the wrong question to ask about a destination.
    fn trash(ctx: &Rc<AppContext>, item: u64) {
        let mut it = binder_item_commands::get_binder_item(ctx, &item)
            .expect("read")
            .expect("exists");
        it.activated = false;
        binder_item_commands::update_binder_item(ctx, None, &it.into()).expect("trash the row");
    }

    #[test]
    fn the_owning_binder_is_found_whichever_one_it_is() {
        let (ctx, work, manuscript, research) = seed();
        let scene = add(&ctx, manuscript, "The ferry", BinderItemRole::Item, 0);
        let people = add(&ctx, research, "People", BinderItemRole::Folder, 0);
        assert_eq!(binder_of_item(&ctx, work, scene), Some(manuscript));
        // The one that matters: a story bible in the *second* binder is still found.
        // A search that stopped at the manuscript would file every captured note into
        // whichever folder happened to share its id there, or nowhere at all.
        assert_eq!(binder_of_item(&ctx, work, people), Some(research));
        assert_eq!(binder_of_item(&ctx, work, 999_999), None);
    }

    #[test]
    fn a_remembered_uid_still_resolves_after_the_ids_have_moved_on() {
        let (ctx, work, _manuscript, research) = seed();
        let people = add(&ctx, research, "People", BinderItemRole::Folder, 0);
        let uid = binder_item_commands::get_binder_item(&ctx, &people)
            .expect("read")
            .expect("exists")
            .uid;
        assert_eq!(item_id_of_uid(&ctx, work, uid), Some(people));
        // A uid from another project, or one whose row has been deleted: refuse rather
        // than resolve to a neighbour. The caller asks the question again instead.
        assert_eq!(item_id_of_uid(&ctx, work, uuid::Uuid::nil()), None);
    }

    #[test]
    fn a_capture_lands_inside_the_folder_and_after_what_is_already_in_it() {
        let (ctx, work, _manuscript, research) = seed();
        let people = add(&ctx, research, "People", BinderItemRole::Folder, 0);
        let (binder, index, indent) =
            place_inside(&ctx, work, people).expect("an empty folder still resolves");
        assert_eq!(binder, research);
        assert_eq!(index, 1, "immediately after the folder row itself");
        assert_eq!(indent, 1, "inside it, not beside it");

        // With a note already filed there, the next one goes after it, at the same
        // depth. A capture that landed at the top would reverse the writer's list every
        // time they used it.
        let _first = add(&ctx, research, "Elizabeth", BinderItemRole::Item, 1);
        let (_, index, indent) = place_inside(&ctx, work, people).expect("resolves");
        assert_eq!((index, indent), (2, 1));
    }

    #[test]
    fn a_folder_that_has_been_deleted_since_refuses_rather_than_guessing() {
        let (ctx, work, _manuscript, _research) = seed();
        assert_eq!(place_inside(&ctx, work, 999_999), None);
    }

    /// **A trashed folder is not a destination either.**
    ///
    /// Trashing is not deletion: the row stays in the binder's order and nothing
    /// sweeps the tag's `creates_in`, so the only thing that tells a live folder from a
    /// trashed one is `activated`. Without that check the capture reports success and
    /// puts the note inside the trash, where the outline does not show it and the next
    /// Empty trash destroys it. The Settings pane already reads this folder as gone,
    /// because its list drops trashed rows, and the two must not disagree.
    #[test]
    fn a_trashed_folder_refuses_rather_than_filing_into_the_trash() {
        let (ctx, work, _manuscript, research) = seed();
        let people = add(&ctx, research, "People", BinderItemRole::Folder, 0);
        assert!(
            place_inside(&ctx, work, people).is_some(),
            "live to begin with, or the assertion below proves nothing"
        );

        trash(&ctx, people);

        let order = binder_commands::get_binder_relationship(
            &ctx,
            &research,
            &BinderRelationshipField::BinderItems,
        )
        .unwrap_or_default();
        assert!(
            order.contains(&people),
            "trashing keeps the row in the binder: that is the whole trap"
        );
        assert_eq!(
            place_inside(&ctx, work, people),
            None,
            "a trashed folder must be refused, so the caller asks again"
        );

        let draft = draft_for(
            &modal::Prefill {
                name: "Elizabeth Bennet".into(),
                ..modal::Prefill::default()
            },
            None,
            String::new(),
        );
        assert!(
            file_note(&ctx, None, work, &draft, people).is_none(),
            "and nothing is filed inside the trash"
        );
    }

    /// The untagged path resolves its remembered folder by uid, and a trashed row keeps
    /// its uid: the same refusal has to happen there, or the answer kept in the
    /// writer's settings outlives the folder it names.
    #[test]
    fn a_trashed_folder_does_not_resolve_from_its_remembered_uid() {
        let (ctx, work, _manuscript, research) = seed();
        let people = add(&ctx, research, "People", BinderItemRole::Folder, 0);
        let uid = binder_item_commands::get_binder_item(&ctx, &people)
            .expect("read")
            .expect("exists")
            .uid;
        assert_eq!(item_id_of_uid(&ctx, work, uid), Some(people));

        trash(&ctx, people);

        assert_eq!(
            item_id_of_uid(&ctx, work, uid),
            None,
            "the uid still matches a row; the row is simply no longer a place to file"
        );
    }

    /// **Only a container is a filing answer.**
    ///
    /// The one-time question shows the whole live binder, not a folders-only tree, so a
    /// writer can click a scene as easily as a folder. `creates_in` names a folder notes
    /// go *into*, and the pick is remembered forever, so the wrong kind of answer has to
    /// be refused rather than coerced.
    #[test]
    fn only_a_container_answers_where_notes_go() {
        let pick = |anchor: Option<u64>, position| BinderDestination {
            binder_id: 1,
            anchor_item_id: anchor,
            position,
            title: String::new(),
        };

        assert_eq!(
            folder_answered_by(Some(&pick(Some(7), DropPosition::Into))),
            Some(7),
            "a folder means inside it, which is the answer"
        );
        assert_eq!(
            folder_answered_by(Some(&pick(Some(7), DropPosition::After))),
            None,
            "a scene means beside it, which is not a folder to file into"
        );
        assert_eq!(
            folder_answered_by(Some(&pick(None, DropPosition::Into))),
            None,
            "a whole binder names no row, and creates_in points at an item"
        );
        assert_eq!(
            folder_answered_by(Some(&pick(Some(7), DropPosition::Before))),
            None
        );
        assert_eq!(folder_answered_by(None), None, "nothing picked yet");
    }

    /// **Exactly one tag, or none.** The menu asks for a single tag on purpose, and
    /// this is where that becomes true of the row: two tags would mean two answers to
    /// "where does this go" and "what does it start from".
    #[test]
    fn a_draft_carries_the_one_chosen_tag_and_nothing_else() {
        let prefill = modal::Prefill {
            name: "Elizabeth Bennet".into(),
            aliases: vec!["Lizzy".into()],
            books: vec![7],
        };
        let tagged = draft_for(&prefill, Some(3), "## Appearance\n".into());
        assert_eq!(tagged.tags, vec![3]);
        assert_eq!(tagged.title, "Elizabeth Bennet");
        assert_eq!(tagged.aliases, vec!["Lizzy".to_string()]);
        assert!(
            tagged.books.is_empty(),
            "filing is a declaration: this door asks nothing and shows nothing, so it \
             must not file the entry under the Book the selection happened to sit in"
        );
        assert_eq!(tagged.body, "## Appearance\n", "the tag's template");

        // Untagged is a real capture, not a degraded one: the same row, no tags on it.
        let untagged = draft_for(&prefill, None, String::new());
        assert!(untagged.tags.is_empty());
        assert_eq!(untagged.title, "Elizabeth Bennet");
    }

    /// **A capture lands exactly one row, inside the folder that was chosen.**
    ///
    /// The end-to-end coverage of the door: what `draft_for` decided actually reaches
    /// the project, in the right place. This is the coverage the story-bible modal's own
    /// `Mode::Create` test used to carry before that path was retired.
    #[test]
    fn a_capture_lands_exactly_one_row_inside_the_chosen_folder() {
        let (ctx, work, _manuscript, research) = seed();
        let people = add(&ctx, research, "People", BinderItemRole::Folder, 0);
        let draft = draft_for(
            &modal::Prefill {
                name: "Elizabeth Bennet".into(),
                aliases: vec!["Lizzy".into()],
                ..modal::Prefill::default()
            },
            Some(3),
            String::new(),
        );
        let landed = file_note(&ctx, None, work, &draft, people).expect("the note is filed");

        let order = binder_commands::get_binder_relationship(
            &ctx,
            &research,
            &BinderRelationshipField::BinderItems,
        )
        .unwrap_or_default();
        assert_eq!(order.len(), 2, "the folder, and the one note inside it");
        let created = binder_item_commands::get_binder_item(&ctx, &landed.item_id)
            .expect("read")
            .expect("exists");
        assert_eq!(created.title, "Elizabeth Bennet");
        assert_eq!(created.aliases, vec!["Lizzy".to_string()]);
        assert_eq!(created.indent, 1, "inside the folder, not beside it");
    }

    /// A folder that vanished between being remembered and being used files nothing.
    /// The caller reads this as "ask again", which is the only answer that cannot put a
    /// note somewhere the writer never chose.
    #[test]
    fn a_vanished_folder_files_nothing() {
        let (ctx, work, _manuscript, _research) = seed();
        let draft = draft_for(
            &modal::Prefill {
                name: "Elizabeth Bennet".into(),
                ..modal::Prefill::default()
            },
            None,
            String::new(),
        );
        assert!(file_note(&ctx, None, work, &draft, 999_999).is_none());
    }
}
