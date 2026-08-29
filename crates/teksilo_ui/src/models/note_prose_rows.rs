// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Which manuscript rows a story-bible note has been **declared** present in, and the
//! order its "In prose" segment shows them in.
//!
//! **Declaration only.** A row belongs here because a writer put this note in its point
//! of view or its cast, never because the mention scanner noticed the note's name (or an
//! alias) somewhere in the row's prose. [`skribisto_model::mentions`] and
//! [`crate::mentions::MentionIndex`] are not consulted anywhere in this module, on
//! purpose: this reading exists precisely so a writer can trust that every row it shows
//! is one they put the note in themselves, with nothing ghosted in from a scan that could
//! be wrong about a name it half-recognised. `docks::inspector::story_bible`'s own Cast
//! section already blends confirmed pins with scan suggestions for the *editing* surface;
//! this is the *reading* surface, and it only shows what was confirmed.
//!
//! **Membership, exactly:** a row belongs when it is activated, when
//! [`skribisto_model::counts_prose`] says it carries countable manuscript prose (never a
//! hand-rolled `Scene | ChapterScene` list, so a chapter folder's own subordinate prose
//! is included on the same terms as a flat scene), and when the note's id appears in the
//! row's `point_of_view` or its `references` (the "Cast" a writer pins in the Inspector,
//! see `docks::inspector::story_bible`'s own section of that name). A row can carry both
//! declarations at once, which is legal and common (a POV character is, by definition,
//! also present); see [`Declaration`].
//!
//! **Book scoping is a single forward pass**, not [`crate::story_bible::infer_book::book_containing`]'s
//! repeated backward walk: that function answers "which Book is *this one* item inside"
//! and is written to be called once per item, which would cost this module one backward
//! scan per candidate row. The manuscript is a flat, ordered stream in which a `Folder/Book`
//! row marks where one Book's rows end and the next one's begin (the same "state machine
//! over the flat item list" the constraint matrix is built on; see `skribisto_model`'s own
//! module doc), so [`declared_rows_in_book`] walks the stream once, flipping a single flag
//! at every row that opens or closes a Book, and keeps whatever it meets while that flag
//! names the Book asked for. No indent check: exactly as `book_containing` establishes,
//! position in the flat stream is what carries meaning here, not nesting depth.
//!
//! **Two rules that walk has to obey, and both have been got wrong before.** It asks
//! [`skribisto_model::SubRoleExt`] rather than comparing `sub_role` to `Book`, so the flat
//! `Item/BookBegin` a legacy import produces opens a Book and an `Item/BookEnd` closes
//! one. And it resets at every **binder edge**: [`ordered_flat_items`] concatenates the
//! Work's binders end to end, and a manuscript that stops without an explicit `BookEnd`
//! (which is optional, and only the shipped templates happen to write) would otherwise go
//! on being its own last Book through the Notes and Research binders behind it. That is
//! the bug `skribisto_model::compile` carries `ItemMeta.binder_id` for.

use frontend::AppContext;
use frontend::common::entities::{BinderItemRole, BinderItemSubRole};
use frontend::direct_access::BinderItemDto;
use skribisto_model::numbering::Numbered;

use crate::models::binder_stream::ordered_flat_items;
use crate::models::numbering::{
    fallback_label_for, numbers_for_items, ordered_item_dtos, work_language_tags,
};

/// Why a row belongs to this note's "In prose" reading: which declaration put it there.
///
/// A row can be both at once (`Both`): a deep-POV scene the writer has also pinned to
/// the cast is not a contradiction, it is the ordinary case a POV character's own pin
/// discipline produces (see `SingleBinderItem::set_point_of_view`, which groups a POV
/// write with a cast write for exactly this reason). The UI has to say so rather than
/// silently picking one: showing only "Point of view" on a row that is *also* confirmed
/// cast would read as though the writer forgot to pin the cast, when they did not.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Declaration {
    PointOfView,
    Cast,
    Both,
}

impl Declaration {
    /// Which declaration (if any) put `note_id` on `item`'s row, reading only the two
    /// relationship fields a writer's own pin populates.
    fn of(item: &BinderItemDto, note_id: u64) -> Option<Self> {
        let pov = item.point_of_view.contains(&note_id);
        let cast = item.references.contains(&note_id);
        match (pov, cast) {
            (true, true) => Some(Self::Both),
            (true, false) => Some(Self::PointOfView),
            (false, true) => Some(Self::Cast),
            (false, false) => None,
        }
    }
}

/// One row of a note's "In prose" stream: which manuscript item it is, how it is
/// captioned, and which declaration put it here.
///
/// `title`/`number`/`fallback_label` mirror [`crate::models::StreamRow`]'s own three
/// fields, on purpose: a row's caption here must read exactly as it does in Full Book, and
/// the same [`crate::models::label_and_badge`] rule renders it (see
/// `tabs::shared::stream::row_header`, whose shape this reading's own row copies rather
/// than calling into). Deliberately **not** a live title probe: this whole row list is
/// re-derived on every relevant backend event (see `tabs::note_in_prose`), so a plain
/// snapshot string is never stale for longer than one coalesced reload.
#[derive(Debug, Clone, PartialEq)]
pub struct NoteProseRow {
    pub item_id: u64,
    pub role: BinderItemRole,
    pub sub_role: BinderItemSubRole,
    pub title: String,
    pub number: Option<usize>,
    pub fallback_label: Option<String>,
    pub declaration: Declaration,
}

/// One Book a Work carries, for the segmented control above the stream.
#[derive(Debug, Clone, PartialEq)]
pub struct BookChoice {
    pub item_id: u64,
    /// The durable identity [`crate::models::NoteBookChoiceService`] persists: a store id
    /// is re-minted on every `load_work` and cannot be written down.
    pub uid: uuid::Uuid,
    pub title: String,
    pub number: Option<usize>,
    pub fallback_label: Option<String>,
}

/// Which Book each row of the work sits in, alongside the Books themselves.
///
/// One walk of the flat stream, because a caller needing both would otherwise make two and
/// could get its answers from two different moments.
///
/// **A row need not be in any Book.** Front matter, a stray note at the top of the binder,
/// anything past a `BookEnd`, every row of another binder that opens no Book of its own
/// (a Notes or Research binder, in particular), and every row of a project with no Book at
/// all: those are simply absent from `of_item`, which is a real answer and not a failure.
/// A surface that groups by Book has to say something about them rather than drop them.
pub struct BookIndex {
    /// Every Book, in manuscript order.
    pub books: Vec<BookChoice>,
    /// Row id to the `item_id` of the Book it is in. Absent means "in no Book".
    pub of_item: std::collections::HashMap<u64, u64>,
}

/// Build a [`BookIndex`] for the work.
///
/// A Book runs from its own marker to the next one, to its explicit end marker, or to the
/// edge of the binder it lives in, through [`skribisto_model::SubRoleExt`] rather than a
/// literal `sub_role == Book`, so the flat `Item/BookBegin` a legacy import produces opens
/// a book here exactly as a `Folder/Book` does. Indent plays no part: the compiler folds
/// the ordered list and never consults it.
pub fn book_index(ctx: &AppContext, work_id: u64) -> BookIndex {
    use skribisto_model::SubRoleExt;

    let flat = ordered_flat_items(ctx, work_id);
    let whole = ordered_item_dtos(ctx, work_id);
    let numbers = numbers_for_items(ctx, work_id, &whole);
    let langs = work_language_tags(ctx, work_id);

    let mut books = Vec::new();
    let mut of_item = std::collections::HashMap::new();
    let mut current: Option<u64> = None;
    let mut binder: Option<u64> = None;
    for (binder_id, it) in &flat {
        // A Book stops at its own binder's edge, exactly as `skribisto_model::compile`
        // stops one: `flat` is binder-major and concatenated, so without this reset the
        // manuscript's last Book would swallow every note and research row behind it and
        // every surface grouping by Book would file them under a Book they are not in.
        if binder != Some(*binder_id) {
            binder = Some(*binder_id);
            current = None;
        }
        if it.sub_role.closes_book() {
            current = None;
            continue;
        }
        if it.sub_role.opens_book() {
            current = Some(it.id);
            let numbered = numbers.get(&it.id);
            books.push(BookChoice {
                item_id: it.id,
                uid: it.uid,
                number: numbered.map(Numbered::number),
                fallback_label: fallback_label_for(it, numbered, &langs),
                title: it.title.clone(),
            });
            continue;
        }
        if let Some(book) = current {
            of_item.insert(it.id, book);
        }
    }
    BookIndex { books, of_item }
}

/// Every Book in `work_id`, activated, in manuscript order: the segmented control's
/// candidates. Empty when the Work has no Book at all, which is what tells the pane to
/// draw its honest empty state instead of a bar with nothing on it.
///
/// [`book_index`]'s own list, rather than a second filter over the same stream: the two
/// once differed by a literal `sub_role == Book` here against `opens_book()` there, which
/// made a legacy-imported project answer "this project has no Book yet" on the segmented
/// control while the very same note's Details page grouped its backlinks under those Books.
/// One walk cannot disagree with itself.
pub fn books_in_work(ctx: &AppContext, work_id: u64) -> Vec<BookChoice> {
    book_index(ctx, work_id).books
}

/// The rows `note_id` has been declared present in, scoped to `book_id` (a live store id
/// from [`books_in_work`]), in manuscript order.
///
/// See the module doc for the membership rule, for why this walks the flat stream once
/// rather than resolving each candidate row's Book separately, and for the two rules that
/// walk obeys: [`skribisto_model::SubRoleExt`] rather than a literal `sub_role == Book`,
/// and a reset at every binder edge.
pub fn declared_rows_in_book(
    ctx: &AppContext,
    work_id: u64,
    note_id: u64,
    book_id: u64,
) -> Vec<NoteProseRow> {
    use skribisto_model::SubRoleExt;

    let flat = ordered_flat_items(ctx, work_id);
    let whole = ordered_item_dtos(ctx, work_id);
    let numbers = numbers_for_items(ctx, work_id, &whole);
    let langs = work_language_tags(ctx, work_id);

    let mut in_book = false;
    let mut binder: Option<u64> = None;
    let mut out = Vec::new();
    for (binder_id, it) in &flat {
        // The binder edge closes a Book here for the same reason it does in
        // [`book_index`]: otherwise an old draft kept as scenes in a second binder is
        // offered as rows of whichever Book the manuscript happened to end inside.
        if binder != Some(*binder_id) {
            binder = Some(*binder_id);
            in_book = false;
        }
        if it.sub_role.closes_book() {
            in_book = false;
        } else if it.sub_role.opens_book() {
            in_book = it.id == book_id;
        }
        if !in_book {
            continue;
        }
        if !skribisto_model::counts_prose(&it.role, &it.sub_role) {
            continue;
        }
        let Some(declaration) = Declaration::of(it, note_id) else {
            continue;
        };
        let numbered = numbers.get(&it.id);
        out.push(NoteProseRow {
            item_id: it.id,
            role: it.role.clone(),
            sub_role: it.sub_role.clone(),
            number: numbered.map(Numbered::number),
            fallback_label: fallback_label_for(it, numbered, &langs),
            title: it.title.clone(),
            declaration,
        });
    }
    out
}

/// Resolve a persisted Book choice against the Work's **live** Book list, never trusting
/// the stored uid blindly: a Book the writer deleted (or that belongs to a `.skrib` opened
/// for the first time on this machine) must not silently point the reading at nothing.
///
/// Falls back to the first Book in manuscript order when the stored uid is `None` or does
/// not resolve: a fresh project, or a stale choice, both read as "start from the top"
/// rather than as "show nothing", which is what the honest-empty-state branch is for
/// instead (an *empty* Book list, not merely an unresolved choice).
pub fn resolve_book_choice(books: &[BookChoice], stored: Option<uuid::Uuid>) -> Option<u64> {
    stored
        .and_then(|uid| books.iter().find(|b| b.uid == uid))
        .or_else(|| books.first())
        .map(|b| b.item_id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use frontend::commands::{binder_commands, binder_item_commands, work_commands};
    use frontend::direct_access::{CreateBinderDto, CreateBinderItemDto, CreateWorkDto};
    use std::rc::Rc;

    // ---- `Declaration::of`, pure, no backend ----

    fn dto_with(references: Vec<u64>, point_of_view: Vec<u64>) -> BinderItemDto {
        BinderItemDto {
            references,
            point_of_view,
            ..Default::default()
        }
    }

    #[test]
    fn a_row_naming_nobody_declares_nothing() {
        assert_eq!(Declaration::of(&dto_with(vec![], vec![]), 7), None);
    }

    #[test]
    fn a_row_in_the_cast_only_declares_cast() {
        assert_eq!(
            Declaration::of(&dto_with(vec![7], vec![]), 7),
            Some(Declaration::Cast)
        );
    }

    #[test]
    fn a_row_told_from_the_notes_view_only_declares_point_of_view() {
        assert_eq!(
            Declaration::of(&dto_with(vec![], vec![7]), 7),
            Some(Declaration::PointOfView)
        );
    }

    #[test]
    fn a_row_both_telling_from_and_naming_the_note_declares_both() {
        assert_eq!(
            Declaration::of(&dto_with(vec![7], vec![7]), 7),
            Some(Declaration::Both)
        );
    }

    #[test]
    fn a_declaration_for_a_different_note_is_not_this_notes_declaration() {
        assert_eq!(Declaration::of(&dto_with(vec![9], vec![9]), 7), None);
    }

    // ---- `resolve_book_choice`, pure, no backend ----

    fn book(item_id: u64, uid: u128) -> BookChoice {
        BookChoice {
            item_id,
            uid: uuid::Uuid::from_u128(uid),
            title: String::new(),
            number: None,
            fallback_label: None,
        }
    }

    #[test]
    fn a_resolving_uid_wins_even_when_it_is_not_first() {
        let books = vec![book(1, 10), book(2, 20), book(3, 30)];
        assert_eq!(
            resolve_book_choice(&books, Some(uuid::Uuid::from_u128(20))),
            Some(2)
        );
    }

    #[test]
    fn a_stale_uid_falls_back_to_the_first_book() {
        let books = vec![book(1, 10), book(2, 20)];
        assert_eq!(
            resolve_book_choice(&books, Some(uuid::Uuid::from_u128(999))),
            Some(1)
        );
    }

    #[test]
    fn no_stored_choice_falls_back_to_the_first_book() {
        let books = vec![book(1, 10), book(2, 20)];
        assert_eq!(resolve_book_choice(&books, None), Some(1));
    }

    #[test]
    fn an_empty_book_list_resolves_to_nothing() {
        assert_eq!(resolve_book_choice(&[], None), None);
    }

    // ---- `books_in_work` / `declared_rows_in_book`, a real backend ----

    /// Two Books, each with a chapter and two scenes, plus a Part heading and a Note
    /// filed before any Book. Shaped to exercise every membership axis at once: an
    /// inactive row, a non-prose row that is nonetheless declared, a row declared in
    /// the *other* Book, and rows declared by point of view, by cast, and by both.
    struct Fixture {
        ctx: Rc<AppContext>,
        work_id: u64,
        note_id: u64,
        book_one: u64,
        book_two: u64,
        // Book one's rows, in the order they were created (and so, in manuscript order).
        b1_chapter: u64,
        b1_scene_pov: u64,
        b1_scene_cast: u64,
        b1_scene_both: u64,
        b1_scene_undeclared: u64,
        b1_scene_inactive: u64,
        b1_part_declared_but_not_prose: u64,
        // Book two's own declared scene, kept out of Book one's reading.
        b2_scene_declared: u64,
        // A declared scene in the **notes** binder, behind the whole manuscript: an old
        // draft a writer kept. It is in no Book, and the manuscript's last Book must not
        // claim it just because the manuscript ends without a `BookEnd`.
        notes_scene_declared: u64,
    }

    fn seed() -> Fixture {
        let ctx = Rc::new(AppContext::new());
        let work = work_commands::create_orphan_work(&ctx, None, &CreateWorkDto::default())
            .expect("create work");
        // Explicit binder indices (never `-1`) and a **per-binder** item-index counter
        // below: `binder_item_commands::create_binder_item`'s index is relative to the
        // binder it names, and `story_bible_place::tests`'s own two-binder fixture is
        // the proof this crate already relies on that a shared counter across binders
        // would quietly violate (its manuscript and notes binders each start their own
        // items at 0).
        let manuscript = binder_commands::create_binder(
            &ctx,
            None,
            &CreateBinderDto {
                name: "Manuscript".into(),
                activated: true,
                ..Default::default()
            },
            work.id,
            0,
        )
        .expect("create binder")
        .id;
        let notes = binder_commands::create_binder(
            &ctx,
            None,
            &CreateBinderDto {
                name: "Notes".into(),
                activated: true,
                ..Default::default()
            },
            work.id,
            1,
        )
        .expect("create binder")
        .id;

        let mut next_manuscript_index = 0i32;
        let mut next_notes_index = 0i32;
        let create = |binder_id: u64,
                      index: &mut i32,
                      role: BinderItemRole,
                      sub_role: BinderItemSubRole,
                      indent: i64,
                      activated: bool| {
            let created = binder_item_commands::create_binder_item(
                &ctx,
                None,
                &CreateBinderItemDto {
                    status: None,
                    title: "row".into(),
                    role,
                    sub_role,
                    activated,
                    is_exportable: true,
                    indent,
                    ..Default::default()
                },
                binder_id,
                *index,
            )
            .expect("create item");
            *index += 1;
            created.id
        };

        let note_id = create(
            notes,
            &mut next_notes_index,
            BinderItemRole::Item,
            BinderItemSubRole::Note,
            0,
            true,
        );

        let book_one = create(
            manuscript,
            &mut next_manuscript_index,
            BinderItemRole::Folder,
            BinderItemSubRole::Book,
            0,
            true,
        );
        let b1_part_declared_but_not_prose = create(
            manuscript,
            &mut next_manuscript_index,
            BinderItemRole::Folder,
            BinderItemSubRole::Part,
            1,
            true,
        );
        let b1_chapter = create(
            manuscript,
            &mut next_manuscript_index,
            BinderItemRole::Folder,
            BinderItemSubRole::ChapterScene,
            2,
            true,
        );
        let b1_scene_pov = create(
            manuscript,
            &mut next_manuscript_index,
            BinderItemRole::Item,
            BinderItemSubRole::Scene,
            3,
            true,
        );
        let b1_scene_cast = create(
            manuscript,
            &mut next_manuscript_index,
            BinderItemRole::Item,
            BinderItemSubRole::Scene,
            3,
            true,
        );
        let b1_scene_both = create(
            manuscript,
            &mut next_manuscript_index,
            BinderItemRole::Item,
            BinderItemSubRole::Scene,
            3,
            true,
        );
        let b1_scene_undeclared = create(
            manuscript,
            &mut next_manuscript_index,
            BinderItemRole::Item,
            BinderItemSubRole::Scene,
            3,
            true,
        );
        let b1_scene_inactive = create(
            manuscript,
            &mut next_manuscript_index,
            BinderItemRole::Item,
            BinderItemSubRole::Scene,
            3,
            false,
        );

        let book_two = create(
            manuscript,
            &mut next_manuscript_index,
            BinderItemRole::Folder,
            BinderItemSubRole::Book,
            0,
            true,
        );
        let b2_scene_declared = create(
            manuscript,
            &mut next_manuscript_index,
            BinderItemRole::Item,
            BinderItemSubRole::Scene,
            1,
            true,
        );
        // The manuscript deliberately ends with no `BookEnd`, which is the ordinary case:
        // the marker is optional and only the shipped templates write one.
        let notes_scene_declared = create(
            notes,
            &mut next_notes_index,
            BinderItemRole::Item,
            BinderItemSubRole::Scene,
            0,
            true,
        );

        use frontend::common::direct_access::binder_item::BinderItemRelationshipField;
        use frontend::direct_access::BinderItemRelationshipDto;
        let declare = |item_id: u64, field: BinderItemRelationshipField, ids: Vec<u64>| {
            binder_item_commands::set_binder_item_relationship(
                &ctx,
                None,
                &BinderItemRelationshipDto {
                    id: item_id,
                    field,
                    right_ids: ids,
                },
            )
            .expect("declare relationship");
        };
        declare(
            b1_scene_pov,
            BinderItemRelationshipField::PointOfView,
            vec![note_id],
        );
        declare(
            b1_scene_cast,
            BinderItemRelationshipField::References,
            vec![note_id],
        );
        declare(
            b1_scene_both,
            BinderItemRelationshipField::PointOfView,
            vec![note_id],
        );
        declare(
            b1_scene_both,
            BinderItemRelationshipField::References,
            vec![note_id],
        );
        // A Part is not `counts_prose` (no `SceneText`), so declaring the note on it
        // must never surface it, however the writer got it declared.
        declare(
            b1_part_declared_but_not_prose,
            BinderItemRelationshipField::References,
            vec![note_id],
        );
        // An inactive row must be excluded even though it is declared.
        declare(
            b1_scene_inactive,
            BinderItemRelationshipField::References,
            vec![note_id],
        );
        // Declared, but in the *other* Book.
        declare(
            b2_scene_declared,
            BinderItemRelationshipField::References,
            vec![note_id],
        );
        // Declared, and in no Book at all: another binder entirely.
        declare(
            notes_scene_declared,
            BinderItemRelationshipField::References,
            vec![note_id],
        );

        Fixture {
            ctx,
            work_id: work.id,
            note_id,
            book_one,
            book_two,
            b1_chapter,
            b1_scene_pov,
            b1_scene_cast,
            b1_scene_both,
            b1_scene_undeclared,
            b1_scene_inactive,
            b1_part_declared_but_not_prose,
            b2_scene_declared,
            notes_scene_declared,
        }
    }

    #[test]
    fn books_in_work_lists_both_books_in_manuscript_order() {
        let f = seed();
        let books = books_in_work(&f.ctx, f.work_id);
        let ids: Vec<u64> = books.iter().map(|b| b.item_id).collect();
        assert_eq!(ids, vec![f.book_one, f.book_two]);
    }

    #[test]
    fn declared_rows_land_in_manuscript_order_with_the_right_declaration() {
        let f = seed();
        let rows = declared_rows_in_book(&f.ctx, f.work_id, f.note_id, f.book_one);
        let got: Vec<(u64, Declaration)> =
            rows.iter().map(|r| (r.item_id, r.declaration)).collect();
        assert_eq!(
            got,
            vec![
                (f.b1_scene_pov, Declaration::PointOfView),
                (f.b1_scene_cast, Declaration::Cast),
                (f.b1_scene_both, Declaration::Both),
            ],
            "undeclared, inactive, non-prose and other-Book rows must all be absent"
        );
    }

    #[test]
    fn an_undeclared_prose_row_never_appears() {
        let f = seed();
        let rows = declared_rows_in_book(&f.ctx, f.work_id, f.note_id, f.book_one);
        assert!(!rows.iter().any(|r| r.item_id == f.b1_scene_undeclared));
    }

    #[test]
    fn an_inactive_declared_row_is_excluded() {
        let f = seed();
        let rows = declared_rows_in_book(&f.ctx, f.work_id, f.note_id, f.book_one);
        assert!(!rows.iter().any(|r| r.item_id == f.b1_scene_inactive));
    }

    /// `counts_prose`, never a hand-rolled sub_role list: a declared Part is exactly the
    /// shape of row that check exists to keep out.
    #[test]
    fn a_declared_but_non_prose_row_is_excluded() {
        let f = seed();
        let rows = declared_rows_in_book(&f.ctx, f.work_id, f.note_id, f.book_one);
        assert!(
            !rows
                .iter()
                .any(|r| r.item_id == f.b1_part_declared_but_not_prose)
        );
    }

    #[test]
    fn a_chapter_folders_own_prose_can_be_declared_too() {
        let f = seed();
        use frontend::common::direct_access::binder_item::BinderItemRelationshipField;
        use frontend::direct_access::BinderItemRelationshipDto;
        binder_item_commands::set_binder_item_relationship(
            &f.ctx,
            None,
            &BinderItemRelationshipDto {
                id: f.b1_chapter,
                field: BinderItemRelationshipField::References,
                right_ids: vec![f.note_id],
            },
        )
        .expect("declare on the chapter's own prose");
        let rows = declared_rows_in_book(&f.ctx, f.work_id, f.note_id, f.book_one);
        assert!(
            rows.iter().any(|r| r.item_id == f.b1_chapter),
            "a Folder/ChapterScene carries its own SceneText and must count as prose"
        );
    }

    #[test]
    fn a_row_declared_in_a_different_book_never_leaks_across_the_scope() {
        let f = seed();
        let in_book_one = declared_rows_in_book(&f.ctx, f.work_id, f.note_id, f.book_one);
        assert!(!in_book_one.iter().any(|r| r.item_id == f.b2_scene_declared));

        let in_book_two = declared_rows_in_book(&f.ctx, f.work_id, f.note_id, f.book_two);
        assert_eq!(
            in_book_two.iter().map(|r| r.item_id).collect::<Vec<_>>(),
            vec![f.b2_scene_declared]
        );
    }

    #[test]
    fn a_note_declared_nowhere_reads_as_an_empty_stream() {
        let f = seed();
        let other_note_id = f.b1_scene_undeclared; // any id this note never appears on
        assert!(declared_rows_in_book(&f.ctx, f.work_id, other_note_id, f.book_one).is_empty());
    }

    // ---- `book_index`: which Book a row is in, and the rows in none ----

    /// A work with one binder and nothing in it, plus a way to append rows.
    ///
    /// Its own seeder rather than [`seed`]'s: that one builds a fixed two-Book shape these
    /// tests would have to work around, and what they are about is the *stream*, one row
    /// at a time.
    fn bare_work() -> (
        Rc<AppContext>,
        u64,
        impl FnMut(BinderItemRole, BinderItemSubRole, i64) -> u64,
    ) {
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
            0,
        )
        .expect("create binder")
        .id;
        let c = ctx.clone();
        let mut index = 0i32;
        let add = move |role: BinderItemRole, sub_role: BinderItemSubRole, indent: i64| {
            let id = binder_item_commands::create_binder_item(
                &c,
                None,
                &CreateBinderItemDto {
                    status: None,
                    title: "row".into(),
                    role,
                    sub_role,
                    activated: true,
                    is_exportable: true,
                    indent,
                    ..Default::default()
                },
                binder,
                index,
            )
            .expect("create item")
            .id;
            index += 1;
            id
        };
        (ctx, work.id, add)
    }

    /// **A row need not be in any Book, and that is a real answer.**
    ///
    /// Front matter before the first Book, and anything past a `BookEnd`, belong to no
    /// Book at all. A surface grouping by Book has to be able to say so, or those rows
    /// become unreachable the moment a project has two Books and every segment filters
    /// them out.
    #[test]
    fn rows_outside_every_book_are_absent_rather_than_guessed() {
        let (ctx, work, mut add) = bare_work();
        let front = add(BinderItemRole::Item, BinderItemSubRole::Scene, 0);
        let book = add(BinderItemRole::Folder, BinderItemSubRole::Book, 0);
        let inside = add(BinderItemRole::Item, BinderItemSubRole::Scene, 1);
        let end = add(BinderItemRole::Item, BinderItemSubRole::BookEnd, 0);
        let after = add(BinderItemRole::Item, BinderItemSubRole::Scene, 0);

        let index = book_index(&ctx, work);
        assert_eq!(
            index.books.iter().map(|b| b.item_id).collect::<Vec<_>>(),
            vec![book],
            "one Book, found through opens_book"
        );
        assert_eq!(index.of_item.get(&inside), Some(&book));
        assert_eq!(index.of_item.get(&front), None, "before any Book");
        assert_eq!(index.of_item.get(&after), None, "past the end marker");
        assert_eq!(
            index.of_item.get(&end),
            None,
            "the marker itself is in no Book"
        );
    }

    /// A project with **no Book at all** places nothing, and says so by having no Books
    /// rather than by inventing one. This is the shape the note pane must not draw a
    /// one-segment bar for.
    #[test]
    fn a_project_with_no_book_places_nothing() {
        let (ctx, work, mut add) = bare_work();
        let a = add(BinderItemRole::Item, BinderItemSubRole::Scene, 0);
        let b = add(BinderItemRole::Item, BinderItemSubRole::Scene, 0);

        let index = book_index(&ctx, work);
        assert!(index.books.is_empty());
        assert_eq!(index.of_item.get(&a), None);
        assert_eq!(index.of_item.get(&b), None);
    }

    /// The flat `Item/BookBegin` encoding opens a Book exactly as `Folder/Book` does, so a
    /// legacy-imported project groups like any other.
    #[test]
    fn a_flat_book_marker_opens_a_book_here_too() {
        let (ctx, work, mut add) = bare_work();
        let begin = add(BinderItemRole::Item, BinderItemSubRole::BookBegin, 0);
        let scene = add(BinderItemRole::Item, BinderItemSubRole::Scene, 0);

        let index = book_index(&ctx, work);
        assert_eq!(
            index.books.iter().map(|b| b.item_id).collect::<Vec<_>>(),
            vec![begin]
        );
        assert_eq!(
            index.of_item.get(&scene),
            Some(&begin),
            "at the marker's own indent, which plays no part"
        );
    }

    /// Pin the note on a row's cast, the way the Inspector's own Cast section does.
    fn declare_cast(ctx: &AppContext, item_id: u64, note_id: u64) {
        use frontend::common::direct_access::binder_item::BinderItemRelationshipField;
        use frontend::direct_access::BinderItemRelationshipDto;
        binder_item_commands::set_binder_item_relationship(
            ctx,
            None,
            &BinderItemRelationshipDto {
                id: item_id,
                field: BinderItemRelationshipField::References,
                right_ids: vec![note_id],
            },
        )
        .expect("declare cast");
    }

    // ---- the binder edge ----

    /// **A Book stops at its own binder's edge.**
    ///
    /// [`ordered_flat_items`] hands back every binder concatenated, so a manuscript that
    /// ends without an explicit `BookEnd` (the ordinary case: the marker is optional) used
    /// to go on being its own last Book straight through the Notes binder behind it. Every
    /// notes row then landed in `of_item` under that Book, and the note's Details page told
    /// the writer an entry mentioned only in a notebook page appears in Book two.
    #[test]
    fn a_row_in_another_binder_is_in_no_book() {
        let f = seed();
        let index = book_index(&f.ctx, f.work_id);
        assert_eq!(
            index.of_item.get(&f.b2_scene_declared),
            Some(&f.book_two),
            "the manuscript's own rows are unaffected"
        );
        assert_eq!(
            index.of_item.get(&f.note_id),
            None,
            "a note in the Notes binder is in no Book"
        );
        assert_eq!(
            index.of_item.get(&f.notes_scene_declared),
            None,
            "and neither is a scene kept in that binder"
        );
    }

    /// The same edge, on the reading itself: an old draft kept as scenes in a second
    /// binder must not be offered as rows of whichever Book the manuscript ended inside.
    #[test]
    fn a_declared_row_in_another_binder_is_no_books_row() {
        let f = seed();
        for book in [f.book_one, f.book_two] {
            let rows = declared_rows_in_book(&f.ctx, f.work_id, f.note_id, book);
            assert!(
                !rows.iter().any(|r| r.item_id == f.notes_scene_declared),
                "a row of the Notes binder belongs to no Book"
            );
        }
    }

    // ---- `SubRoleExt`, never a literal `sub_role == Book` ----

    /// A legacy import marks its Books with flat `Item/BookBegin` rows. `books_in_work`
    /// once compared `sub_role` to `Book` literally and answered "no Books", so the whole
    /// In prose reading drew "This project has no Book yet" for every entry, permanently,
    /// while the same note's Details page grouped its backlinks under those very Books.
    #[test]
    fn a_flat_book_marker_is_a_book_on_the_segmented_control_too() {
        let (ctx, work, mut add) = bare_work();
        let note = add(BinderItemRole::Item, BinderItemSubRole::Note, 0);
        let begin = add(BinderItemRole::Item, BinderItemSubRole::BookBegin, 0);
        let scene = add(BinderItemRole::Item, BinderItemSubRole::Scene, 0);
        declare_cast(&ctx, scene, note);

        assert_eq!(
            books_in_work(&ctx, work)
                .iter()
                .map(|b| b.item_id)
                .collect::<Vec<_>>(),
            vec![begin],
            "a flat marker is a Book to choose between"
        );
        assert_eq!(
            declared_rows_in_book(&ctx, work, note, begin)
                .iter()
                .map(|r| r.item_id)
                .collect::<Vec<_>>(),
            vec![scene],
            "and its rows are that Book's rows"
        );
    }

    /// **`Item/BookEnd` closes the Book for the reading too.** The writer picked
    /// Create -> End of book; the row after it is outside every Book, and listing it under
    /// the Book above would put it in a reading the export disagrees with.
    #[test]
    fn an_end_marker_closes_the_reading_too() {
        let (ctx, work, mut add) = bare_work();
        let note = add(BinderItemRole::Item, BinderItemSubRole::Note, 0);
        let book = add(BinderItemRole::Folder, BinderItemSubRole::Book, 0);
        let inside = add(BinderItemRole::Item, BinderItemSubRole::Scene, 1);
        let _end = add(BinderItemRole::Item, BinderItemSubRole::BookEnd, 0);
        let after = add(BinderItemRole::Item, BinderItemSubRole::Scene, 0);
        declare_cast(&ctx, inside, note);
        declare_cast(&ctx, after, note);

        assert_eq!(
            declared_rows_in_book(&ctx, work, note, book)
                .iter()
                .map(|r| r.item_id)
                .collect::<Vec<_>>(),
            vec![inside],
            "the row past the end marker is in no Book, exactly as `book_index` says"
        );
        let index = book_index(&ctx, work);
        assert_eq!(index.of_item.get(&after), None, "and the two agree");
    }
}
