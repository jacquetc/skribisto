// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

use super::*;
use crate::round_trip::digest;

fn existing(tag: &str, title: &str, prose: &str) -> ExistingRow {
    ExistingRow {
        uid_tag: tag.into(),
        title: title.into(),
        create_type: CreateType::Chapter,
        digest: digest(prose),
    }
}

/// An incoming row from a file this project exported: it knows what it was and what it
/// digested to at the time.
fn returning(tag: &str, title: &str, baseline: &str, now: &str) -> IncomingRow {
    IncomingRow {
        source_uid_tag: Some(tag.into()),
        source_digest: Some(digest(baseline)),
        title: title.into(),
        create_type: CreateType::Chapter,
        digest: digest(now),
    }
}

/// An incoming row from anywhere else — no mark, no baseline.
fn foreign(title: &str, prose: &str) -> IncomingRow {
    IncomingRow {
        source_uid_tag: None,
        source_digest: None,
        title: title.into(),
        create_type: CreateType::Chapter,
        digest: digest(prose),
    }
}

// ── the status table ────────────────────────────────────────────────────────────────

#[test]
fn nobody_touched_it() {
    let e = [existing("t1", "One", "Same prose.")];
    let i = [returning("t1", "One", "Same prose.", "Same prose.")];
    assert_eq!(align(&e, &i)[0].status, RowStatus::Identical);
}

#[test]
fn the_editor_edited_it() {
    let e = [existing("t1", "One", "Same prose.")];
    let i = [returning("t1", "One", "Same prose.", "Edited prose.")];
    let row = &align(&e, &i)[0];
    assert_eq!(row.status, RowStatus::EditorEdited);
    assert_eq!(row.default_action(), RowAction::TakeImport);
}

#[test]
fn you_edited_it() {
    let e = [existing("t1", "One", "My newer prose.")];
    let i = [returning("t1", "One", "Same prose.", "Same prose.")];
    let row = &align(&e, &i)[0];
    assert_eq!(row.status, RowStatus::YouEdited);
    assert_eq!(
        row.default_action(),
        RowAction::KeepCurrent,
        "taking the import would discard the writer's own work"
    );
}

#[test]
fn both_edited_it_differently_is_a_conflict() {
    let e = [existing("t1", "One", "My newer prose.")];
    let i = [returning("t1", "One", "Same prose.", "Their newer prose.")];
    let row = &align(&e, &i)[0];
    assert_eq!(row.status, RowStatus::Conflict);
    assert!(row.status.needs_attention());
    assert_eq!(row.default_action(), RowAction::KeepCurrent);
}

/// Both sides moved and landed on the same text. Asking the writer to adjudicate between
/// two identical passages would be a conflict in name only.
#[test]
fn both_edited_it_the_same_way_is_not_a_conflict() {
    let e = [existing("t1", "One", "The agreed wording.")];
    let i = [returning(
        "t1",
        "One",
        "Old wording.",
        "The agreed wording.",
    )];
    assert_eq!(align(&e, &i)[0].status, RowStatus::Identical);
}

#[test]
fn without_a_baseline_the_verdict_is_only_same_or_different() {
    let e = [existing("t1", "One", "Some prose.")];
    assert_eq!(
        align(&e, &[foreign("One", "Some prose.")])[0].status,
        RowStatus::Identical
    );
    let row = &align(&e, &[foreign("One", "Other prose.")])[0];
    assert_eq!(row.status, RowStatus::Different);
    assert_eq!(
        row.default_action(),
        RowAction::KeepCurrent,
        "with no way to know who changed it, changing nothing is the only safe default"
    );
}

// ── pairing ─────────────────────────────────────────────────────────────────────────

/// A mark outranks a title, and must: an editor is free to retitle a chapter, and the
/// mark is the only evidence of what it actually is.
#[test]
fn a_mark_pairs_a_row_the_editor_retitled() {
    let e = [existing("t1", "Chapter One", "Prose.")];
    let i = [returning("t1", "A Better Title", "Prose.", "Prose.")];
    let rows = align(&e, &i);
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].current, Some(0));
    assert_eq!(rows[0].status, RowStatus::Identical);
}

#[test]
fn without_a_mark_a_matching_type_and_title_pairs() {
    let e = [existing("t1", "Chapter One", "Prose.")];
    let i = [foreign("chapter one", "Prose.")];
    assert_eq!(align(&e, &i)[0].current, Some(0));
}

#[test]
fn a_different_type_does_not_pair_however_alike_the_title() {
    let e = [existing("t1", "Prologue", "Prose.")];
    let mut inc = foreign("Prologue", "Prose.");
    inc.create_type = CreateType::Scene;
    let rows = align(&e, &[inc]);
    assert_eq!(rows.len(), 2, "one new and one missing: {rows:#?}");
    assert!(rows.iter().any(|r| r.status == RowStatus::New));
    assert!(rows.iter().any(|r| r.status == RowStatus::Missing));
}

/// Several rows sharing a title pair in order rather than all claiming the first.
#[test]
fn repeated_titles_pair_in_stream_order() {
    let e = [
        existing("t1", "Untitled", "First."),
        existing("t2", "Untitled", "Second."),
    ];
    let i = [
        foreign("Untitled", "First."),
        foreign("Untitled", "Second."),
    ];
    let rows = align(&e, &i);
    assert_eq!(rows[0].current, Some(0));
    assert_eq!(rows[1].current, Some(1));
    assert!(rows.iter().all(|r| r.status == RowStatus::Identical));
}

/// An exact match must not lose its partner to a title guess made earlier in the list.
#[test]
fn a_mark_claims_its_row_before_any_title_guess_can() {
    let e = [
        existing("t1", "Untitled", "First."),
        existing("t2", "Untitled", "Second."),
    ];
    // The first incoming row has no mark and would otherwise claim `t1` by title; the
    // second names `t1` outright.
    let i = [
        foreign("Untitled", "First."),
        returning("t1", "Untitled", "First.", "First."),
    ];
    let rows = align(&e, &i);
    let marked = rows
        .iter()
        .find(|r| r.incoming == Some(1))
        .expect("the marked row is present");
    assert_eq!(marked.current, Some(0), "the mark wins its own row");
}

// ── alignment ───────────────────────────────────────────────────────────────────────

/// The case the whole two-column design exists for.
#[test]
fn a_chapter_inserted_between_two_existing_ones_lands_between_them() {
    let e = [
        existing("t1", "One", "First."),
        existing("t3", "Three", "Third."),
    ];
    let i = [
        returning("t1", "One", "First.", "First."),
        foreign("Two", "Brand new."),
        returning("t3", "Three", "Third.", "Third."),
    ];
    let rows = align(&e, &i);
    assert_eq!(rows.len(), 3);
    assert_eq!(rows[0].current, Some(0));
    assert_eq!(
        (rows[1].current, rows[1].status),
        (None, RowStatus::New),
        "the inserted chapter has no current side, and sits between its neighbours"
    );
    assert_eq!(rows[2].current, Some(1));
}

#[test]
fn a_chapter_the_file_no_longer_has_is_shown_not_deleted() {
    let e = [
        existing("t1", "One", "First."),
        existing("t2", "Two", "Second."),
    ];
    let i = [returning("t1", "One", "First.", "First.")];
    let rows = align(&e, &i);
    assert_eq!(rows.len(), 2);
    assert_eq!(rows[1].status, RowStatus::Missing);
    assert_eq!(rows[1].incoming, None);
    assert_eq!(
        rows[1].actions,
        vec![RowAction::KeepCurrent],
        "nothing here may delete the writer's chapter"
    );
}

/// A moved chapter stays one row rather than becoming a deletion and an addition —
/// otherwise its identity, comments and history are torn apart by a reordering.
#[test]
fn a_moved_chapter_is_reported_as_moved_not_as_delete_plus_insert() {
    let e = [
        existing("t1", "One", "First."),
        existing("t2", "Two", "Second."),
        existing("t3", "Three", "Third."),
    ];
    let i = [
        returning("t3", "Three", "Third.", "Third."),
        returning("t1", "One", "First.", "First."),
        returning("t2", "Two", "Second.", "Second."),
    ];
    let rows = align(&e, &i);
    assert_eq!(rows.len(), 3, "three rows in, three rows out: {rows:#?}");
    assert!(
        rows.iter()
            .all(|r| r.current.is_some() && r.incoming.is_some()),
        "every row is paired: {rows:#?}"
    );
    assert_eq!(
        rows.iter().filter(|r| r.moved).count(),
        1,
        "exactly the one that jumped: {rows:#?}"
    );
}

/// The mirror of the test above, which is the rotation that was broken.
///
/// Moving the **first** chapter to the end puts a paired-but-moved row behind every anchor,
/// so the anchor walk's forward-fill sweeps over it before its own pairing is reached. It
/// used to announce that row as deleted and then emit it a second time, correctly, further
/// down: the writer read "Chapter One — missing" directly above "Chapter One — moved".
#[test]
fn moving_the_first_chapter_to_the_end_does_not_report_it_as_deleted() {
    let e = [
        existing("t1", "One", "First."),
        existing("t2", "Two", "Second."),
        existing("t3", "Three", "Third."),
    ];
    let i = [
        returning("t2", "Two", "Second.", "Second."),
        returning("t3", "Three", "Third.", "Third."),
        returning("t1", "One", "First.", "First."),
    ];
    let rows = align(&e, &i);
    assert_eq!(rows.len(), 3, "three rows in, three rows out: {rows:#?}");
    assert!(
        !rows.iter().any(|r| r.status == RowStatus::Missing),
        "the file still carries every chapter: {rows:#?}"
    );
    assert_eq!(
        rows.iter().filter(|r| r.moved).count(),
        1,
        "exactly the one that jumped: {rows:#?}"
    );
}

/// The invariant, over **every** reordering rather than the two anyone thought to write.
///
/// Both hand-written move tests passed while a third rotation was broken, which is the
/// argument for exhausting the space: four rows is 24 permutations and costs nothing.
#[test]
fn no_reordering_makes_a_row_appear_twice_or_vanish() {
    let e = [
        existing("t1", "One", "First."),
        existing("t2", "Two", "Second."),
        existing("t3", "Three", "Third."),
        existing("t4", "Four", "Fourth."),
    ];
    let tags = ["t1", "t2", "t3", "t4"];
    let titles = ["One", "Two", "Three", "Four"];
    let proses = ["First.", "Second.", "Third.", "Fourth."];

    for order in permutations(&[0, 1, 2, 3]) {
        let i: Vec<IncomingRow> = order
            .iter()
            .map(|&n| returning(tags[n], titles[n], proses[n], proses[n]))
            .collect();
        let rows = align(&e, &i);

        for j in 0..e.len() {
            assert_eq!(
                rows.iter().filter(|r| r.current == Some(j)).count(),
                1,
                "existing row {j} appears once for order {order:?}: {rows:#?}"
            );
        }
        for k in 0..i.len() {
            assert_eq!(
                rows.iter().filter(|r| r.incoming == Some(k)).count(),
                1,
                "incoming row {k} appears once for order {order:?}: {rows:#?}"
            );
        }
        assert!(
            !rows.iter().any(|r| r.status == RowStatus::Missing),
            "a pure reordering deletes nothing, order {order:?}: {rows:#?}"
        );
    }
}

fn permutations(items: &[usize]) -> Vec<Vec<usize>> {
    if items.len() <= 1 {
        return vec![items.to_vec()];
    }
    let mut out = Vec::new();
    for (at, &head) in items.iter().enumerate() {
        let mut rest = items.to_vec();
        rest.remove(at);
        for mut tail in permutations(&rest) {
            tail.insert(0, head);
            out.push(tail);
        }
    }
    out
}

#[test]
fn a_first_import_into_an_empty_project_is_all_new() {
    let i = [foreign("One", "First."), foreign("Two", "Second.")];
    let rows = align(&[], &i);
    assert_eq!(rows.len(), 2);
    assert!(rows.iter().all(|r| r.status == RowStatus::New));
    assert!(
        rows.iter()
            .all(|r| r.default_action() == RowAction::CreateNew)
    );
}

#[test]
fn nothing_incoming_leaves_every_existing_row_untouched() {
    let e = [existing("t1", "One", "First.")];
    let rows = align(&e, &[]);
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].status, RowStatus::Missing);
}

/// Every row of both inputs appears exactly once, whatever the shape — the property a
/// merge view rests on, and the one an alignment bug breaks silently.
#[test]
fn every_row_of_both_sides_appears_exactly_once() {
    let e = [
        existing("t1", "One", "First."),
        existing("t2", "Two", "Second."),
        existing("t3", "Three", "Third."),
    ];
    let i = [
        returning("t2", "Two", "Second.", "Edited."),
        foreign("New", "Fresh."),
        returning("t1", "One", "First.", "First."),
    ];
    let rows = align(&e, &i);

    for j in 0..e.len() {
        assert_eq!(
            rows.iter().filter(|r| r.current == Some(j)).count(),
            1,
            "existing row {j} appears once: {rows:#?}"
        );
    }
    for k in 0..i.len() {
        assert_eq!(
            rows.iter().filter(|r| r.incoming == Some(k)).count(),
            1,
            "incoming row {k} appears once: {rows:#?}"
        );
    }
}

#[test]
fn the_longest_increasing_subsequence_is_the_one_it_claims_to_be() {
    assert_eq!(longest_increasing(&[]), Vec::<usize>::new());
    assert_eq!(longest_increasing(&[5]), vec![0]);
    assert_eq!(longest_increasing(&[0, 1, 2]), vec![0, 1, 2]);
    // 2 is the odd one out.
    assert_eq!(longest_increasing(&[0, 1, 5, 2, 3]), vec![0, 1, 3, 4]);
    assert_eq!(longest_increasing(&[3, 2, 1]).len(), 1);
}
