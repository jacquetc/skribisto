// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Split into two tiers, on purpose.
//!
//! The **pure** tests below (`grouped_cards`, `subtree_notes`, `scene_mention_counts`)
//! run in both feature sets: they read the real, in-memory backend
//! (`frontend::commands::*`, which, unlike `TagsViewModel`'s own `mocks` arm, is
//! the same implementation regardless of the crate's `mocks` feature; see
//! `models::binder_stream`'s own module doc for the precedent) or take hand-built
//! data with no backend at all.
//!
//! The **widget-mount** tests near the bottom that read a real discoverable tag
//! back through `TagsViewModel` are gated `#[cfg(not(feature = "mocks"))]`: that
//! view-model's `mocks` arm fabricates its own fixed built-in palette rather than
//! reading what a test created (see `WorkTagsListModel`'s own two `mod imp` arms),
//! so asserting against a tag this test made would fail under `mocks` for a reason
//! that has nothing to do with this feature. The Books-chip and filtering tests do
//! **not** need that gate: `live_books` and `subtree_notes` both go straight through
//! `frontend::commands::*`, real in both builds.

use std::rc::Rc;

use frontend::AppContext;
use frontend::commands::{binder_commands, binder_item_commands, work_commands};
use frontend::common::entities::{BinderItemRole, BinderItemSubRole};
use frontend::direct_access::{CreateBinderDto, CreateBinderItemDto, CreateWorkDto};
use teksilo::core::widget_tree::WidgetTree;

use crate::app_ids::AppIds;
use crate::mentions::{MentionIndex, MentionRow};

use super::*;

// ── Fixtures ─────────────────────────────────────────────────────────────────

/// A Work with two binders: one for the manuscript (Books live there), one for
/// notes (the Story bible place's own container), so a Book's own indent/position
/// never entangles with `subtree_notes`'s per-binder walk.
struct Fixture {
    app_ctx: Rc<AppContext>,
    ids: AppIds,
    manuscript_binder: u64,
    notes_binder: u64,
    notes_folder: u64,
}

fn seed() -> Fixture {
    let app_ctx = Rc::new(AppContext::new());
    let work = work_commands::create_orphan_work(&app_ctx, None, &CreateWorkDto::default())
        .expect("create work");
    let manuscript_binder = binder_commands::create_binder(
        &app_ctx,
        None,
        &CreateBinderDto {
            name: "Manuscript".into(),
            activated: true,
            ..Default::default()
        },
        work.id,
        0,
    )
    .expect("create manuscript binder")
    .id;
    let notes_binder = binder_commands::create_binder(
        &app_ctx,
        None,
        &CreateBinderDto {
            name: "Notes".into(),
            activated: true,
            ..Default::default()
        },
        work.id,
        1,
    )
    .expect("create notes binder")
    .id;
    let notes_folder = binder_item_commands::create_binder_item(
        &app_ctx,
        None,
        &CreateBinderItemDto {
            title: "Story bible".into(),
            role: BinderItemRole::Folder,
            sub_role: BinderItemSubRole::Note,
            activated: true,
            is_exportable: true,
            indent: 0,
            ..Default::default()
        },
        notes_binder,
        0,
    )
    .expect("create notes folder")
    .id;
    let ids = AppIds::new();
    ids.work_id.set(Some(work.id));
    Fixture {
        app_ctx,
        ids,
        manuscript_binder,
        notes_binder,
        notes_folder,
    }
}

#[allow(clippy::too_many_arguments)]
fn create_note(
    f: &Fixture,
    index: i32,
    title: &str,
    tags: Vec<u64>,
    aliases: Vec<String>,
    books: Vec<u64>,
) -> u64 {
    binder_item_commands::create_binder_item(
        &f.app_ctx,
        None,
        &CreateBinderItemDto {
            title: title.into(),
            role: BinderItemRole::Item,
            sub_role: BinderItemSubRole::Note,
            activated: true,
            is_exportable: true,
            indent: 1,
            tags,
            aliases,
            books,
            ..Default::default()
        },
        f.notes_binder,
        index,
    )
    .expect("create note")
    .id
}

/// A Scene, outside the notes binder entirely: the shape a real manuscript row
/// has, and what `scene_mention_counts` must count.
fn create_scene(f: &Fixture, index: i32, title: &str) -> u64 {
    binder_item_commands::create_binder_item(
        &f.app_ctx,
        None,
        &CreateBinderItemDto {
            title: title.into(),
            role: BinderItemRole::Item,
            sub_role: BinderItemSubRole::Scene,
            activated: true,
            is_exportable: true,
            indent: 0,
            ..Default::default()
        },
        f.manuscript_binder,
        index,
    )
    .expect("create scene")
    .id
}

fn create_book(f: &Fixture, index: i32, title: &str) -> u64 {
    binder_item_commands::create_binder_item(
        &f.app_ctx,
        None,
        &CreateBinderItemDto {
            title: title.into(),
            role: BinderItemRole::Folder,
            sub_role: BinderItemSubRole::Book,
            activated: true,
            is_exportable: true,
            indent: 0,
            ..Default::default()
        },
        f.manuscript_binder,
        index,
    )
    .expect("create book")
    .id
}

fn tag(id: u64, name: &str, discoverable: bool) -> TagRow {
    TagRow {
        id,
        uid: uuid::Uuid::nil(),
        name: name.to_string(),
        color: "#2e7d32".to_string(),
        details: String::new(),
        discoverable,
        creates_in: None,
        note_template: None,
    }
}

// ── `subtree_notes` ──────────────────────────────────────────────────────────

/// The read scopes to the container's own subtree, picks up its Note-facet
/// children's fields, and leaves a sibling Book folder (in a different binder
/// entirely) untouched.
#[test]
fn subtree_notes_reads_the_containers_own_note_children_only() {
    let f = seed();
    // `f.notes_folder` itself already occupies index 0 in `notes_binder` (`seed`),
    // so every note created here starts at 1.
    let elizabeth = create_note(
        &f,
        1,
        "Elizabeth Bennet",
        vec![7],
        vec!["Lizzy".into(), "Liz".into()],
        vec![42],
    );
    let _locke_manor = create_note(&f, 2, "Locke Manor", vec![], vec![], vec![]);
    let _book = create_book(&f, 0, "Book One");

    let entries = subtree_notes(&f.app_ctx, f.ids.work_id.get().unwrap(), f.notes_folder);
    assert_eq!(entries.len(), 2, "both notes, and only the notes");

    let e = entries
        .iter()
        .find(|e| e.item_id == elizabeth)
        .expect("Elizabeth must be in the subtree");
    assert_eq!(e.title, "Elizabeth Bennet");
    assert_eq!(e.tags, vec![7]);
    assert_eq!(e.alias_count, 2, "two aliases");
    assert_eq!(e.book_ids, vec![42]);
}

/// A row outside the subtree (here, a second notes folder at the same depth as
/// the one under test, in the same binder) must not leak in.
#[test]
fn subtree_notes_does_not_cross_into_a_sibling_folders_own_children() {
    let f = seed();
    // `f.notes_folder` occupies index 0; every row below starts at 1.
    let _inside = create_note(&f, 1, "Inside", vec![], vec![], vec![]);
    let sibling_folder = binder_item_commands::create_binder_item(
        &f.app_ctx,
        None,
        &CreateBinderItemDto {
            title: "Another folder".into(),
            role: BinderItemRole::Folder,
            sub_role: BinderItemSubRole::Note,
            activated: true,
            is_exportable: true,
            indent: 0,
            ..Default::default()
        },
        f.notes_binder,
        2,
    )
    .expect("create sibling folder")
    .id;
    let _outside = create_note(&f, 3, "Outside", vec![], vec![], vec![]);
    // `create_note` always uses indent 1 and `f.notes_binder`, so "Outside" here
    // is deliberately placed *after* the sibling folder at indent 1 too: its
    // presence would prove the walk over-ran the first folder's own subtree.
    let _ = sibling_folder;

    let entries = subtree_notes(&f.app_ctx, f.ids.work_id.get().unwrap(), f.notes_folder);
    let titles: Vec<&str> = entries.iter().map(|e| e.title.as_str()).collect();
    assert_eq!(
        titles,
        vec!["Inside"],
        "the walk must stop at the first row back at the container's own indent"
    );
}

// ── `grouped_cards` ──────────────────────────────────────────────────────────

fn entry(item_id: u64, title: &str, tags: Vec<u64>, alias_count: usize) -> BibleEntry {
    BibleEntry {
        item_id,
        title: title.to_string(),
        tags,
        alias_count,
        book_ids: Vec::new(),
    }
}

/// The core grouping guarantee: an entry appears under every discoverable tag it
/// carries, in the palette's own order, and an entry with no discoverable tag
/// falls back to one "not yet tagged" group, ordered last.
#[test]
fn grouped_cards_groups_by_discoverable_tag_and_falls_back_to_untagged() {
    let discoverable = vec![tag(1, "character", true), tag(2, "location", true)];
    let entries = vec![
        entry(10, "Elizabeth Bennet", vec![1], 1),
        entry(11, "Locke Manor", vec![2], 0),
        entry(12, "A loose idea", vec![], 0),
    ];
    let cards = grouped_cards(entries, &HashMap::new(), &discoverable);
    let groups: Vec<(&str, &str)> = cards
        .iter()
        .map(|c| (c.group.as_str(), c.entry.title.as_str()))
        .collect();
    assert_eq!(
        groups,
        vec![
            ("character", "Elizabeth Bennet"),
            ("location", "Locke Manor"),
            ("Not yet tagged", "A loose idea"),
        ],
        "palette order, catch-all last"
    );
}

/// An entry carrying two discoverable tags browses under both, the same
/// multi-membership a photo grouped into two albums would show under both.
#[test]
fn an_entry_with_two_discoverable_tags_appears_under_both_groups() {
    let discoverable = vec![tag(1, "character", true), tag(2, "poi", true)];
    let entries = vec![entry(10, "The Innkeeper", vec![1, 2], 0)];
    let cards = grouped_cards(entries, &HashMap::new(), &discoverable);
    let groups: Vec<&str> = cards.iter().map(|c| c.group.as_str()).collect();
    assert_eq!(groups, vec!["character", "poi"]);
    assert!(cards.iter().all(|c| c.entry.title == "The Innkeeper"));
}

/// A non-discoverable tag never grounds a group of its own: it plays no part in
/// this grid at all, matching every other discoverable-only surface in this
/// edition.
#[test]
fn a_non_discoverable_tag_does_not_form_a_group() {
    let discoverable = vec![tag(1, "character", true)];
    // Tag 9 is not in the discoverable table at all (e.g. "status/draft").
    let entries = vec![entry(10, "Draft note", vec![9], 0)];
    let cards = grouped_cards(entries, &HashMap::new(), &discoverable);
    assert_eq!(cards.len(), 1);
    assert_eq!(cards[0].group, "Not yet tagged");
}

/// A card carries its own alias count and its own scene-scoped mention count,
/// read straight off the fields `grouped_cards` was handed: this is the data a
/// [`super::BibleCard`] renders verbatim.
#[test]
fn a_card_carries_its_alias_count_and_mention_count() {
    let discoverable = vec![tag(1, "character", true)];
    let entries = vec![entry(10, "Elizabeth Bennet", vec![1], 3)];
    let mut mentions = HashMap::new();
    mentions.insert(10, 5usize);
    let cards = grouped_cards(entries, &mentions, &discoverable);
    assert_eq!(cards.len(), 1);
    assert_eq!(cards[0].entry.alias_count, 3);
    assert_eq!(cards[0].mentions, 5);
}

/// An entry with no recorded mentions gets `0`, not a missing map entry:
/// [`super::scene_mention_counts`] never omits an id it was asked about.
#[test]
fn an_entry_with_no_mentions_reads_as_zero_not_missing() {
    let discoverable: Vec<TagRow> = Vec::new();
    let entries = vec![entry(10, "Nobody has met them yet", vec![], 0)];
    let cards = grouped_cards(entries, &HashMap::new(), &discoverable);
    assert_eq!(cards[0].mentions, 0);
}

/// [`super::BibleCard`] builds and lays out without panicking over the data
/// `grouped_cards` produces: the render half of "a card shows its alias count
/// and mention badge".
#[test]
fn the_card_widget_builds_over_its_own_grouped_data() {
    let card = GroupedCard {
        entry: entry(10, "Elizabeth Bennet", vec![1], 2),
        mentions: 4,
        group: "character".to_string(),
    };
    let mut tree = WidgetTree::new();
    let id = tree.add_boxed(Box::new(BibleCard { card, root: None }));
    tree.layout(SizeProposal::exact(260.0, 96.0));
    assert!(
        tree.bounds(id).width > 0.0 && tree.bounds(id).height > 0.0,
        "the card must actually lay out to a real size"
    );
}

// ── `scene_mention_counts` ───────────────────────────────────────────────────

fn mention_row(owner_id: u64, target_id: u64) -> MentionRow {
    MentionRow {
        owner_id,
        target_id,
        title: String::new(),
        matched_name: String::new(),
        is_title_match: false,
        hit_count: 1,
        is_confirmed: false,
        is_point_of_view: false,
        evidence: String::new(),
    }
}

/// A row with no text hit at all: a scene bound only through `point_of_view` or
/// `references`, exactly as `scan_mentions_uc` would fold one in.
fn declared_mention_row(owner_id: u64, target_id: u64, point_of_view: bool) -> MentionRow {
    MentionRow {
        hit_count: 0,
        is_point_of_view: point_of_view,
        ..mention_row(owner_id, target_id)
    }
}

/// Scene-owned hits count; a Note-owned hit on the same entry does not: the
/// badge's own "scenes" wording stays honest.
#[test]
fn scene_mention_counts_excludes_note_owned_hits() {
    let f = seed();
    let scene = create_scene(&f, 0, "A dawn scene");
    let other_note = create_note(&f, 1, "A worldbuilding note", vec![], vec![], vec![]);
    let elizabeth = entry(999, "Elizabeth Bennet", vec![], 0);

    let index = MentionIndex::seeded_for_tests(
        Vec::new(),
        vec![
            mention_row(scene, elizabeth.item_id),
            mention_row(other_note, elizabeth.item_id),
        ],
    );

    let counts = scene_mention_counts(&f.app_ctx, &index, std::slice::from_ref(&elizabeth));
    assert_eq!(
        counts.get(&elizabeth.item_id),
        Some(&1),
        "one scene-owned hit counted, the note-owned one excluded"
    );
}

/// **The badge's meaning, restated in a test.** A scene bound only through
/// `point_of_view`, no text hit, no pin, still counts toward the badge, the
/// same way an `is_confirmed`-only row always has: see the module doc's own
/// paragraph on why this is not a meaning change for "appears in N scenes".
#[test]
fn scene_mention_counts_includes_a_point_of_view_only_scene() {
    let f = seed();
    let scene = create_scene(&f, 0, "A scene telling nothing but her thoughts");
    let elizabeth = entry(999, "Elizabeth Bennet", vec![], 0);

    let index = MentionIndex::seeded_for_tests(
        Vec::new(),
        vec![declared_mention_row(scene, elizabeth.item_id, true)],
    );

    let counts = scene_mention_counts(&f.app_ctx, &index, std::slice::from_ref(&elizabeth));
    assert_eq!(
        counts.get(&elizabeth.item_id),
        Some(&1),
        "a declared point of view is a real presence in the scene, with or without a \
         text hit to show for it"
    );
}

/// Batched across several entries in one backend read: proven by asserting the
/// per-entry answers stay correct, not by counting calls (nothing here exposes a
/// call counter, and the doc comment on the function states the batching
/// rationale).
#[test]
fn scene_mention_counts_is_correct_across_several_entries_at_once() {
    let f = seed();
    let scene_a = create_scene(&f, 0, "Scene A");
    let scene_b = create_scene(&f, 1, "Scene B");
    let elizabeth = entry(101, "Elizabeth", vec![], 0);
    let darcy = entry(102, "Darcy", vec![], 0);

    let index = MentionIndex::seeded_for_tests(
        Vec::new(),
        vec![
            mention_row(scene_a, elizabeth.item_id),
            mention_row(scene_b, elizabeth.item_id),
            mention_row(scene_a, darcy.item_id),
        ],
    );

    let counts = scene_mention_counts(&f.app_ctx, &index, &[elizabeth.clone(), darcy.clone()]);
    assert_eq!(counts.get(&elizabeth.item_id), Some(&2));
    assert_eq!(counts.get(&darcy.item_id), Some(&1));
}

// ── Widget mount: the Books filter chip row and filtering ──────────────────

/// A `TagsViewModel` that resolves (so the pane does not bail out on a missing
/// `app_state`), whatever it happens to hold: these tests never assert on tag
/// grouping, only on the Books chip row and the filter, both of which read
/// straight through `frontend::commands::*` and never through this view-model.
fn any_tags_vm(f: &Fixture) -> TagsViewModel {
    TagsViewModel::new(
        crate::models::WorkTagsListModel::new(f.app_ctx.clone(), f.ids.clone()),
        f.ids.clone(),
    )
}

fn blank_typography() -> crate::settings::EditorTypography {
    crate::settings::EditorTypography {
        font_family: Signal::new(String::new()),
        size: Signal::new(1.0),
        line_height: Signal::new(1.5),
        first_line_indent: Signal::new(0.0),
        para_spacing_before: Signal::new(0.0),
        para_spacing_after: Signal::new(0.0),
        size_range: crate::settings::TypographySizeRange::default(),
    }
}

fn typography() -> crate::settings::EditorTypographySet {
    crate::settings::EditorTypographySet {
        scene: blank_typography(),
        synopsis: blank_typography(),
        notes: blank_typography(),
        corkboard: blank_typography(),
        distraction_free: blank_typography(),
    }
}

fn notes_tab(f: &Fixture) -> ContentTab {
    crate::tabs::tab_for(
        &f.app_ctx,
        f.notes_folder,
        &BinderItemRole::Folder,
        &BinderItemSubRole::Note,
        &[],
        Signal::new(700.0),
        Signal::new(true),
        typography(),
        crate::settings::EditorViewMemory::detached(false),
        &f.ids,
    )
}

fn mount(f: &Fixture, tab: &ContentTab) -> (WidgetTree, WidgetId) {
    use std::any::{Any, TypeId};

    let mut state: HashMap<TypeId, Box<dyn Any>> = HashMap::new();
    state.insert(TypeId::of::<TagsViewModel>(), Box::new(any_tags_vm(f)));
    state.insert(
        TypeId::of::<MentionIndex>(),
        Box::new(MentionIndex::seeded_for_tests(Vec::new(), Vec::new())),
    );
    let mut tree = crate::test_support::tree_with_app_state(&f.app_ctx, state);
    let id = tree.add_boxed(story_bible_pane(tab));
    tree.layout(SizeProposal::exact(800.0, 600.0));
    (tree, id)
}

fn pane_of(tree: &WidgetTree, id: WidgetId) -> &StoryBiblePane {
    tree.widget_as_any(id)
        .and_then(|a| a.downcast_ref::<StoryBiblePane>())
        .expect("StoryBiblePane must be introspectable via as_any")
}

/// **Below two Books, no chip row at all; at two, it appears.** The same
/// "gated at the source, not per surface" discipline every other Books control
/// in this edition follows.
#[test]
fn the_book_filter_chip_row_is_absent_with_one_book_and_present_with_two() {
    let one_book = seed();
    let _only = create_book(&one_book, 0, "Book One");
    let tab = notes_tab(&one_book);
    let (tree, id) = mount(&one_book, &tab);
    assert!(
        !pane_of(&tree, id).book_filter_rendered,
        "one Book: no chip row"
    );

    let two_books = seed();
    let _a = create_book(&two_books, 0, "Book One");
    let _b = create_book(&two_books, 1, "Book Two");
    let tab = notes_tab(&two_books);
    let (tree, id) = mount(&two_books, &tab);
    assert!(
        pane_of(&tree, id).book_filter_rendered,
        "two Books: the chip row must appear"
    );
}

/// Filtering by a Book hides an entry filed under a different Book; the
/// unfiltered default ("All books") hides nothing, including an entry that was
/// never filed at all.
#[test]
fn filtering_by_book_hides_entries_filed_elsewhere_and_the_unfiltered_default_hides_nothing() {
    let f = seed();
    let book_one = create_book(&f, 0, "Book One");
    let book_two = create_book(&f, 1, "Book Two");
    let _filed_under_one = create_note(
        &f,
        1,
        "Filed under Book One",
        vec![],
        vec![],
        vec![book_one],
    );
    let _filed_under_two = create_note(
        &f,
        2,
        "Filed under Book Two",
        vec![],
        vec![],
        vec![book_two],
    );
    let _unfiled = create_note(&f, 3, "Not yet filed", vec![], vec![], vec![]);
    let tab = notes_tab(&f);

    // Unfiltered default: every entry shows, filed or not.
    let (tree, id) = mount(&f, &tab);
    assert_eq!(
        pane_of(&tree, id).cards_rendered.len(),
        3,
        "the unfiltered default must hide nothing"
    );

    // Filter to Book One: the Book Two entry and the unfiled entry both drop out.
    // An unfiled entry is "not yet filed", never "every Book".
    tab.story_bible_book_filter.set(Some(book_one));
    let (tree, id) = mount(&f, &tab);
    let titles: Vec<String> = pane_of(&tree, id)
        .cards_rendered
        .iter()
        .map(|c| c.entry.title.clone())
        .collect();
    assert_eq!(titles, vec!["Filed under Book One".to_string()]);
}

#[cfg(not(feature = "mocks"))]
mod real_backend_only {
    use super::*;
    use frontend::commands::binder_tag_commands;
    use frontend::direct_access::CreateBinderTagDto;

    fn create_tag(f: &Fixture, name: &str, discoverable: bool) -> u64 {
        let now = chrono::Utc::now();
        binder_tag_commands::create_binder_tag(
            &f.app_ctx,
            None,
            &CreateBinderTagDto {
                uid: Default::default(),
                created_at: now,
                updated_at: now,
                name: name.to_string(),
                color: "#2e7d32".to_string(),
                details: String::new(),
                discoverable,
                creates_in: None,
                note_template: None,
            },
            f.ids.work_id.get().expect("a Work must be open"),
            -1,
        )
        .expect("create tag")
        .id
    }

    /// **The one true end-to-end proof: the pane actually renders on a real
    /// `Folder/Note` tab and groups a real entry under a real discoverable
    /// tag's own name.** Gated off `mocks` because `TagsViewModel`'s own mock
    /// arm answers with a fixed built-in palette rather than reading this
    /// tag back: see this module's own doc comment.
    #[test]
    fn the_grid_renders_on_a_notes_folder_tab_and_groups_by_the_real_discoverable_tag() {
        let f = seed();
        let character_tag = create_tag(&f, "character", true);
        let _elizabeth = create_note(
            &f,
            1,
            "Elizabeth Bennet",
            vec![character_tag],
            vec!["Lizzy".into()],
            vec![],
        );
        let tab = notes_tab(&f);
        let (tree, id) = mount(&f, &tab);
        let pane = pane_of(&tree, id);
        assert_eq!(pane.cards_rendered.len(), 1);
        assert_eq!(pane.cards_rendered[0].group, "character");
        assert_eq!(pane.cards_rendered[0].entry.title, "Elizabeth Bennet");
        assert_eq!(pane.cards_rendered[0].entry.alias_count, 1);
    }
}
