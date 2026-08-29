// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The pure transaction behind the story-bible creation modal. It has no widgets and no
//! `EventContext`, so it is unit-testable without a live tree. [`crate::story_bible::modal`]
//! is the only caller; everything it gathers in local state funnels through
//! [`create_entry`] or [`configure_entry`] exactly once, when the writer presses
//! Create.
//!
//! **One composite, always**, even on the path that only makes a single backend
//! call: wrapping a lone `create_binder_item` costs nothing and means a future
//! caller adding a second write (a body, later) does not have to remember to
//! reach for `begin_composite` for the first time. A composite left open on
//! failure is *cancelled*, not ended; an ended-but-incomplete group would
//! swallow the writer's very next unrelated edit into this one undo entry.

use std::rc::Rc;

use anyhow::Result;

use frontend::AppContext;
use frontend::commands::{binder_item_commands, content_commands, undo_redo_commands};
use frontend::common::entities::{BinderItemRole, BinderItemSubRole, ContentRole};
use frontend::direct_access::{CreateBinderItemDto, CreateContentDto};

use crate::singles::SingleBinderItem;

/// Everything the writer filled in, gathered in local state and touched by no
/// backend call until [`create_entry`]/[`configure_entry`] runs. Cancel is simply
/// never calling either: there is nothing to roll back because nothing was ever
/// asked for.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct EntryDraft {
    /// The row's title. Never trimmed here; the modal's own Create button gates
    /// on a non-blank name, so this stores exactly what the writer typed.
    pub title: String,
    pub tags: Vec<u64>,
    pub aliases: Vec<String>,
    /// Which Book(s) the entry is filed under. Empty means not yet filed (see
    /// `common::entities::BinderItem::books`'s own doc), never "every Book", and
    /// never written at all when the Work has fewer than two Books, since the
    /// modal's Books control does not exist there to have set it.
    pub books: Vec<u64>,
    /// The Djot body a chosen template contributes, or whatever the small body
    /// editor holds. Empty means no `Content` row is created at all: an entry
    /// with nothing written yet has no more of one than a freshly created Note
    /// does, and a blank `Content` row would be indistinguishable from one the
    /// writer emptied themselves.
    pub body: String,
}

/// The row [`create_entry`] made, for the caller's "Create and open" / reveal step.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CreatedEntry {
    pub item_id: u64,
    pub uid: uuid::Uuid,
}

/// **Fresh creation**, for the "Add as note" door and any future stand-alone "New
/// entry" door, where nothing exists yet. One composite: `create_binder_item`
/// (title, tags, aliases and books all land in the same DTO, since this entity's
/// relationships are ordinary fields on it, not four separate writes) plus, only
/// when `draft.body` is non-empty, one `create_content`.
///
/// `role`/`sub_role` are always `(Item, Note)`, since a story-bible entry has no
/// storage shape of its own (see `skribisto_model::CreateType::StoryBibleEntry`'s
/// own doc), so this never takes them as parameters; a caller cannot accidentally
/// create anything else through this door.
///
/// Returns `None` only when the item itself could not be created: nothing has
/// been written yet at that point, so the composite is safely *cancelled*
/// (never ended empty-handed). Once the item exists, the composite is always
/// *ended*, even if the body write that follows fails: leaving the row
/// half-made with no undo entry to remove it would be a worse failure than a
/// bible entry that came out with no body, the same call
/// [`app::recreate_row::commit`](crate::app::recreate_row) already makes for a
/// version whose row lands but whose text does not, and for the same reason
/// ("the row itself is back... taking it away again... is a worse answer").
pub fn create_entry(
    ctx: &Rc<AppContext>,
    stack: Option<u64>,
    binder: u64,
    index: usize,
    indent: i64,
    draft: &EntryDraft,
) -> Option<CreatedEntry> {
    let _ = undo_redo_commands::begin_composite(ctx, stack);
    let now = chrono::Utc::now();
    let dto = CreateBinderItemDto {
        status: None,
        created_at: now,
        updated_at: now,
        title: draft.title.clone(),
        role: BinderItemRole::Item,
        sub_role: BinderItemSubRole::Note,
        activated: true,
        // Out of the export, like every other note: a story-bible entry is the writer's
        // own workings, not part of the book. See `OutlineViewModel`'s own create path,
        // which applies the same rule to every note shape.
        is_exportable: false,
        indent,
        aliases: draft.aliases.clone(),
        books: draft.books.clone(),
        tags: draft.tags.clone(),
        ..Default::default()
    };
    let created =
        match binder_item_commands::create_binder_item(ctx, stack, &dto, binder, index as i32) {
            Ok(created) => created,
            Err(_) => {
                // Nothing has landed yet, so cancelling outright is safe and correct.
                undo_redo_commands::cancel_composite(ctx);
                return None;
            }
        };

    if !draft.body.is_empty()
        && let Err(e) = write_body(ctx, stack, created.id, &draft.body)
    {
        eprintln!("story_bible: writing the new entry's body failed: {e}");
    }

    undo_redo_commands::end_composite(ctx);
    Some(CreatedEntry {
        item_id: created.id,
        uid: created.uid,
    })
}

/// **Configure an already-created item**, for the outline vocabulary's door, where
/// `CreateType::StoryBibleEntry` has already landed a plain, unconfigured row
/// (matching every sibling in the ＋ Create vocabulary, which never defers
/// creation behind a modal) and this applies the writer's name/tags/aliases/books/
/// body to it, as its own composite, one undo step distinct from the row's own
/// creation.
///
/// Never creates or deletes an item. A title left blank in the draft renames the
/// row to blank, the same rule `OutlineViewModel::rename` already applies to an
/// item (as opposed to a binder): an emptied title is a legitimate instruction,
/// not a refusal to type one.
///
/// Returns `false` only when the very first write (the title) fails: nothing has
/// changed yet, so the composite is cancelled outright. Every write that follows
/// is best-effort: once anything has landed, the composite always ends as one
/// undo entry, for the same reason [`create_entry`] never cancels after its item
/// exists.
pub fn configure_entry(
    ctx: &Rc<AppContext>,
    stack: Option<u64>,
    item_id: u64,
    draft: &EntryDraft,
) -> bool {
    let probe = SingleBinderItem::new(ctx.clone());
    probe.set_id(Some(item_id));
    let _ = undo_redo_commands::begin_composite(ctx, stack);
    if let Err(e) = probe.set_title(&draft.title, stack) {
        eprintln!("story_bible: naming the entry failed: {e}");
        undo_redo_commands::cancel_composite(ctx);
        return false;
    }
    if let Err(e) = probe.set_tags(&draft.tags, stack) {
        eprintln!("story_bible: tagging the entry failed: {e}");
    }
    if let Err(e) = probe.set_aliases(&draft.aliases, stack) {
        eprintln!("story_bible: aliasing the entry failed: {e}");
    }
    if let Err(e) = probe.set_books(&draft.books, stack) {
        eprintln!("story_bible: filing the entry failed: {e}");
    }
    if !draft.body.is_empty()
        && let Err(e) = write_body(ctx, stack, item_id, &draft.body)
    {
        eprintln!("story_bible: writing the entry's body failed: {e}");
    }
    undo_redo_commands::end_composite(ctx);
    true
}

/// One `Content` row of `ContentRole::NoteText`, created fresh under `item_id`.
/// Always a create, never an update: both callers above only ever reach this for
/// an item that cannot already carry a `NoteText` row, one just minted, in
/// `create_entry`'s case, or one just minted by the ＋ Create vocabulary and
/// never yet opened in an editor, in `configure_entry`'s.
fn write_body(ctx: &AppContext, stack: Option<u64>, item_id: u64, body: &str) -> Result<()> {
    let now = chrono::Utc::now();
    content_commands::create_content(
        ctx,
        stack,
        &CreateContentDto {
            uid: Default::default(),
            created_at: now,
            updated_at: now,
            activated: true,
            role: ContentRole::NoteText,
            data: body.to_string(),
        },
        item_id,
        0,
    )
    .map(|_| ())
}

#[cfg(test)]
mod tests {
    use super::*;
    use frontend::commands::{binder_commands, binder_item_commands, work_commands};
    use frontend::common::direct_access::binder_item::BinderItemRelationshipField;
    use frontend::direct_access::{CreateBinderDto, CreateWorkDto};

    fn seed() -> (Rc<AppContext>, u64, u64) {
        let ctx = Rc::new(AppContext::new());
        let work = work_commands::create_orphan_work(&ctx, None, &CreateWorkDto::default())
            .expect("create work");
        let binder = binder_commands::create_binder(
            &ctx,
            None,
            &CreateBinderDto {
                name: "Manuscript".into(),
                activated: true,
                ..Default::default()
            },
            work.id,
            -1,
        )
        .expect("create binder");
        (ctx, work.id, binder.id)
    }

    fn draft(title: &str) -> EntryDraft {
        EntryDraft {
            title: title.to_string(),
            tags: Vec::new(),
            aliases: vec!["Lizzy".to_string()],
            books: Vec::new(),
            body: "A note about Elizabeth.".to_string(),
        }
    }

    /// **The core creation-pipeline guarantee.** One composite, one undo entry,
    /// and undoing it removes the item *and* the content row it seeded: never a
    /// row with its body silently left behind because a caller forgot the
    /// content write belonged in the same group as the create.
    #[test]
    fn create_entry_is_one_undo_entry_and_undo_removes_the_whole_entry() {
        let (ctx, _work_id, binder_id) = seed();
        let stack = frontend::commands::undo_redo_commands::create_new_stack(&ctx);

        let created = create_entry(
            &ctx,
            Some(stack),
            binder_id,
            0,
            0,
            &draft("Elizabeth Bennet"),
        )
        .expect("create_entry must succeed against a freshly seeded binder");

        assert_eq!(
            frontend::commands::undo_redo_commands::get_stack_size(&ctx, stack),
            1,
            "the item create and the content create must land as ONE undo entry, not two"
        );

        // Both halves exist before undo: the item, and its seeded body.
        let before = binder_item_commands::get_binder_item(&ctx, &created.item_id)
            .expect("read")
            .expect("the item must exist before undo");
        assert_eq!(before.title, "Elizabeth Bennet");
        assert_eq!(before.aliases, vec!["Lizzy".to_string()]);
        let content_ids = binder_item_commands::get_binder_item_relationship(
            &ctx,
            &created.item_id,
            &BinderItemRelationshipField::Contents,
        )
        .unwrap_or_default();
        assert_eq!(
            content_ids.len(),
            1,
            "the seeded body must exist before undo"
        );

        frontend::commands::undo_redo_commands::undo(&ctx, Some(stack)).expect("undo");

        assert!(
            binder_item_commands::get_binder_item(&ctx, &created.item_id)
                .ok()
                .flatten()
                .is_none(),
            "one undo must remove the whole entry, item included"
        );
    }

    /// Nothing is called until `create_entry` runs: Cancel is simply never
    /// reaching this function. Proven here as the flip side of the test above:
    /// a draft filled in every field and then discarded (never handed to
    /// `create_entry`) leaves the binder exactly as seeded.
    #[test]
    fn a_draft_that_is_never_committed_creates_nothing() {
        let (ctx, _work_id, binder_id) = seed();
        let _unused = draft("Elizabeth Bennet"); // filled in, never committed
        let items = binder_commands::get_binder_relationship(
            &ctx,
            &binder_id,
            &frontend::common::direct_access::binder::BinderRelationshipField::BinderItems,
        )
        .unwrap_or_default();
        assert!(
            items.is_empty(),
            "gathering a draft locally must never touch the store"
        );
    }

    /// A template's body lands as the note's content, verbatim.
    #[test]
    fn a_templates_body_lands_as_the_notes_content() {
        let (ctx, _work_id, binder_id) = seed();
        let stack = frontend::commands::undo_redo_commands::create_new_stack(&ctx);
        let mut d = draft("Locke Manor");
        d.body = "# Locke Manor\n\nA crumbling estate on the coast.".to_string();

        let created = create_entry(&ctx, Some(stack), binder_id, 0, 0, &d).expect("create_entry");
        let content_ids = binder_item_commands::get_binder_item_relationship(
            &ctx,
            &created.item_id,
            &BinderItemRelationshipField::Contents,
        )
        .unwrap_or_default();
        let content = frontend::commands::content_commands::get_content_multi(&ctx, &content_ids)
            .unwrap_or_default()
            .into_iter()
            .flatten()
            .find(|c| c.role == ContentRole::NoteText)
            .expect("the body must have landed as NoteText content");
        assert_eq!(content.data, d.body);
    }

    /// An empty body creates no `Content` row at all: never a blank one that
    /// would be indistinguishable from a scene the writer emptied themselves.
    #[test]
    fn an_empty_body_creates_no_content_row() {
        let (ctx, _work_id, binder_id) = seed();
        let stack = frontend::commands::undo_redo_commands::create_new_stack(&ctx);
        let mut d = draft("Empty");
        d.body = String::new();
        let created = create_entry(&ctx, Some(stack), binder_id, 0, 0, &d).expect("create_entry");
        let content_ids = binder_item_commands::get_binder_item_relationship(
            &ctx,
            &created.item_id,
            &BinderItemRelationshipField::Contents,
        )
        .unwrap_or_default();
        assert!(content_ids.is_empty());
    }

    /// `configure_entry` is its own composite, distinct from the item's own
    /// creation: the outline vocabulary's door creates first (its own undo
    /// entry, matching every sibling type), then this applies the writer's
    /// configuration as a second, separate entry.
    #[test]
    fn configure_entry_names_tags_aliases_books_and_body_in_one_more_undo_entry() {
        let (ctx, _work_id, binder_id) = seed();
        let stack = frontend::commands::undo_redo_commands::create_new_stack(&ctx);
        let created = binder_item_commands::create_binder_item(
            &ctx,
            Some(stack),
            &frontend::direct_access::CreateBinderItemDto {
                status: None,
                title: "New story bible entry".into(),
                role: BinderItemRole::Item,
                sub_role: BinderItemSubRole::Note,
                activated: true,
                is_exportable: true,
                ..Default::default()
            },
            binder_id,
            0,
        )
        .expect("create the row the outline vocabulary would have made");
        assert_eq!(
            frontend::commands::undo_redo_commands::get_stack_size(&ctx, stack),
            1,
            "the plain create is its own undo entry"
        );

        let mut d = draft("Elizabeth Bennet");
        d.body = "Eldest of the Bennet sisters.".to_string();
        assert!(configure_entry(&ctx, Some(stack), created.id, &d));

        assert_eq!(
            frontend::commands::undo_redo_commands::get_stack_size(&ctx, stack),
            2,
            "configuring is a SECOND, separate undo entry from the row's own creation"
        );

        // The backend round trip below only holds against the real backend.
        // `configure_entry` writes through its own, internal `SingleBinderItem`
        // probe, and under `mocks` that probe's setters mutate only the
        // probe's own local `dto` signal, never the store this test's `ctx`
        // reads back from (see `SingleBinderItem`'s mock `set_title`/`set_aliases`,
        // and `docks::inspector::tests::setting_a_book_filing_writes_it_and_it_reads_back`,
        // which reads back off the very probe that wrote, for the same reason).
        // There is no way for this test to reach `configure_entry`'s own probe
        // from out here, so under `mocks` the undo-count assertions above are
        // this test's whole guarantee.
        if cfg!(not(feature = "mocks")) {
            let after = binder_item_commands::get_binder_item(&ctx, &created.id)
                .expect("read")
                .expect("item");
            assert_eq!(after.title, "Elizabeth Bennet");
            assert_eq!(after.aliases, vec!["Lizzy".to_string()]);
        }
    }

    /// **The undo round-trip on `configure_entry`'s own composite**, proven by
    /// actually calling `undo()` on it and reading back, not just by the stack
    /// growing by one entry. The entry-count assertion above would pass just as
    /// happily on a partial rollback (the title reverting but the seeded
    /// `Content` row left orphaned, say), since it never calls `undo()` at all.
    /// This is [`create_entry_is_one_undo_entry_and_undo_removes_the_whole_entry`]'s
    /// counterpart for the *second* transaction `EntryDraft` can go through.
    #[test]
    fn configure_entry_undo_reverts_every_field_and_removes_the_seeded_content_row() {
        let (ctx, _work_id, binder_id) = seed();
        let stack = frontend::commands::undo_redo_commands::create_new_stack(&ctx);

        // A Book to file under, so `books` is exercised the same as `tags` and
        // `aliases` rather than left at its always-empty default.
        let book_id = binder_item_commands::create_binder_item(
            &ctx,
            None,
            &frontend::direct_access::CreateBinderItemDto {
                status: None,
                title: "Book One".into(),
                role: BinderItemRole::Folder,
                sub_role: BinderItemSubRole::Book,
                activated: true,
                ..Default::default()
            },
            binder_id,
            -1,
        )
        .expect("create a Book to file under")
        .id;

        // The plain, unconfigured row the outline vocabulary would have made:
        // this is `configure_entry`'s pre-state, and what undo must return to.
        let created = binder_item_commands::create_binder_item(
            &ctx,
            Some(stack),
            &frontend::direct_access::CreateBinderItemDto {
                status: None,
                title: "New story bible entry".into(),
                role: BinderItemRole::Item,
                sub_role: BinderItemSubRole::Note,
                activated: true,
                is_exportable: true,
                ..Default::default()
            },
            binder_id,
            0,
        )
        .expect("create the row the outline vocabulary would have made");

        let mut d = draft("Elizabeth Bennet");
        d.books = vec![book_id];
        d.body = "Eldest of the Bennet sisters.".to_string();
        assert!(configure_entry(&ctx, Some(stack), created.id, &d));

        assert_eq!(
            frontend::commands::undo_redo_commands::get_stack_size(&ctx, stack),
            2,
            "configuring is its own, second undo entry"
        );

        // Same limitation as the test above: `configure_entry` writes through its
        // own, internal `SingleBinderItem` probe, whose `mocks` arm mutates only
        // the probe's own local signal, never the store this test's `ctx` reads
        // back from. The undo round trip through `ctx` is provable only against
        // the real backend; under `mocks` the entry-count assertion above is this
        // test's whole guarantee, same as its neighbor.
        if cfg!(not(feature = "mocks")) {
            let configured = binder_item_commands::get_binder_item(&ctx, &created.id)
                .expect("read")
                .expect("item");
            assert_eq!(configured.title, "Elizabeth Bennet");
            assert_eq!(configured.aliases, vec!["Lizzy".to_string()]);
            assert_eq!(configured.books, vec![book_id]);
            let content_ids_before = binder_item_commands::get_binder_item_relationship(
                &ctx,
                &created.id,
                &BinderItemRelationshipField::Contents,
            )
            .unwrap_or_default();
            assert_eq!(
                content_ids_before.len(),
                1,
                "configure_entry's body write must have seeded one Content row"
            );

            // Undo ONLY the configure step: the second undo entry, not the row's
            // own creation underneath it.
            frontend::commands::undo_redo_commands::undo(&ctx, Some(stack)).expect("undo");

            let reverted = binder_item_commands::get_binder_item(&ctx, &created.id)
                .expect("read")
                .expect("the row itself must still exist: only its configuration is undone");
            assert_eq!(
                reverted.title, "New story bible entry",
                "the title must revert to what the outline vocabulary seeded, not stay Elizabeth Bennet"
            );
            assert!(
                reverted.aliases.is_empty(),
                "the alias configure_entry wrote must be gone"
            );
            assert!(
                reverted.books.is_empty(),
                "the book filing configure_entry wrote must be gone"
            );
            let content_ids_after = binder_item_commands::get_binder_item_relationship(
                &ctx,
                &created.id,
                &BinderItemRelationshipField::Contents,
            )
            .unwrap_or_default();
            assert!(
                content_ids_after.is_empty(),
                "the Content row write_body seeded must be gone too, not left orphaned"
            );

            // And the row's own creation, the FIRST undo entry, must still stand:
            // undoing configure_entry's composite must not reach past it.
            assert_eq!(
                frontend::commands::undo_redo_commands::get_stack_size(&ctx, stack),
                1,
                "one more undo entry remains: the row's own creation"
            );
        }
    }
}
