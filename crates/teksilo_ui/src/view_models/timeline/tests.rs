// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

use super::*;
use skrib_format::versions::BlobStamp;
use std::path::PathBuf;

fn uid(n: u128) -> uuid::Uuid {
    uuid::Uuid::from_u128(n)
}

fn live(n: u128, title: &str, order: usize, digest: &str) -> LiveRow {
    LiveRow {
        uid: uid(n),
        title: title.to_string(),
        order,
        digest: digest.to_string(),
    }
}

fn version_row(n: u128, title: &str) -> VersionRow {
    VersionRow {
        uid: uid(n),
        title: title.to_string(),
        sub_role: Default::default(),
        indent: 0,
        prose: vec![(
            ContentRole::SceneText,
            format!("binders/01/text/{n}.scene.djot"),
            BlobStamp { bytes: 10 },
        )],
    }
}

/// `compare` needs a filesystem, so its *decision table* is exercised through
/// this pure re-statement of it — the same four rules, over data, including
/// the rank-among-shared-rows definition of "moved".
fn classify(then: &[(usize, VersionRow, String)], now: &[LiveRow]) -> Vec<(String, ChangeKind)> {
    classify_from(then, now, true)
}

/// [`classify`], with `ordered` off for a source that has no order — the log.
fn classify_from(
    then: &[(usize, VersionRow, String)],
    now: &[LiveRow],
    ordered: bool,
) -> Vec<(String, ChangeKind)> {
    let shared: HashSet<uuid::Uuid> = then
        .iter()
        .map(|(_, r, _)| r.uid)
        .filter(|u| now.iter().any(|l| &l.uid == u))
        .collect();
    let rank_then: HashMap<uuid::Uuid, usize> = then
        .iter()
        .map(|(_, r, _)| r.uid)
        .filter(|u| shared.contains(u))
        .enumerate()
        .map(|(i, u)| (u, i))
        .collect();
    let mut ordered_now: Vec<&LiveRow> = now.iter().filter(|l| shared.contains(&l.uid)).collect();
    ordered_now.sort_by_key(|l| l.order);
    let rank_now: HashMap<uuid::Uuid, usize> = ordered_now
        .iter()
        .enumerate()
        .map(|(i, l)| (l.uid, i))
        .collect();
    let moved = |u: &uuid::Uuid| ordered && rank_then.get(u) != rank_now.get(u);

    let mut out = Vec::new();
    for (_, row, digest) in then {
        match now.iter().find(|l| l.uid == row.uid) {
            None if ordered => out.push((row.title.clone(), ChangeKind::Removed)),
            None => {}
            Some(l) if &l.digest != digest => out.push((l.title.clone(), ChangeKind::Changed)),
            Some(l) if moved(&row.uid) => out.push((l.title.clone(), ChangeKind::Moved)),
            Some(_) => {}
        }
    }
    if ordered {
        for l in now {
            if !then.iter().any(|(_, r, _)| r.uid == l.uid) {
                out.push((l.title.clone(), ChangeKind::Added));
            }
        }
    }
    out
}

#[test]
fn a_row_deleted_since_is_reported_as_removed() {
    let then = vec![(0, version_row(1, "The lost scene"), "d1".to_string())];
    let got = classify(&then, &[]);
    assert_eq!(
        got,
        vec![("The lost scene".to_string(), ChangeKind::Removed)]
    );
}

#[test]
fn a_row_written_since_is_reported_as_added() {
    let got = classify(&[], &[live(2, "A new chapter", 0, "d2")]);
    assert_eq!(got, vec![("A new chapter".to_string(), ChangeKind::Added)]);
}

#[test]
fn a_row_whose_text_moved_on_is_reported_as_changed() {
    let then = vec![(0, version_row(1, "Chapter 1"), "old".to_string())];
    let got = classify(&then, &[live(1, "Chapter 1", 0, "new")]);
    assert_eq!(got, vec![("Chapter 1".to_string(), ChangeKind::Changed)]);
}

/// A reorder is not a rewrite, and calling it one would send a writer looking
/// for an edit that never happened.
#[test]
fn a_row_that_only_changed_place_is_reported_as_moved() {
    let then = vec![
        (0, version_row(1, "Chapter 1"), "a".to_string()),
        (1, version_row(2, "Chapter 2"), "b".to_string()),
    ];
    let got = classify(
        &then,
        &[live(2, "Chapter 2", 0, "b"), live(1, "Chapter 1", 1, "a")],
    );
    assert_eq!(got.len(), 2, "both ends of a swap moved: {got:?}");
    assert!(got.iter().all(|(_, k)| *k == ChangeKind::Moved));
}

/// **Rank among the rows both sides share**, not absolute position. One scene
/// written since would otherwise shift every row after it and report the
/// whole book as rearranged.
#[test]
fn a_row_written_since_does_not_report_everything_after_it_as_moved() {
    let then = vec![
        (0, version_row(1, "Chapter 1"), "a".to_string()),
        (1, version_row(2, "Chapter 2"), "b".to_string()),
    ];
    let got = classify(
        &then,
        &[
            live(9, "A new opening", 0, "n"),
            live(1, "Chapter 1", 1, "a"),
            live(2, "Chapter 2", 2, "b"),
        ],
    );
    assert_eq!(
        got,
        vec![("A new opening".to_string(), ChangeKind::Added)],
        "an insertion is one addition, not an addition plus a rearranged book",
    );
}

/// **The bug this caught live.** The history log records *prose*, so its rows
/// come out of a map keyed by uid — an order nothing ever had — and it holds
/// no entry at all for a row without text. Asked the structural questions, it
/// reported the whole manuscript as rearranged and every folder as written
/// since, against a moment minutes old.
#[test]
fn a_source_that_recorded_only_prose_makes_no_structural_claims() {
    let then = vec![
        (0, version_row(1, "Chapter 1"), "a".to_string()),
        (1, version_row(2, "Chapter 2"), "b".to_string()),
    ];
    // Reordered, one row gone from the record, one folder the log never knew.
    let now = [
        live(2, "Chapter 2", 0, "b"),
        live(1, "Chapter 1", 1, "a"),
        live(7, "Front matter", 2, ""),
    ];
    let structural = classify_from(&then, &now, true);
    assert!(
        structural.len() >= 3,
        "precondition: a bundle sees the moves and the addition: {structural:?}",
    );
    assert!(
        classify_from(&then, &now, false).is_empty(),
        "a prose-only record cannot say what existed or where it sat",
    );
}

/// …and it still says the one thing it does know.
#[test]
fn a_prose_only_source_still_reports_a_changed_text() {
    let then = vec![(0, version_row(1, "Chapter 1"), "old".to_string())];
    let got = classify_from(&then, &[live(1, "Chapter 1", 0, "new")], false);
    assert_eq!(got, vec![("Chapter 1".to_string(), ChangeKind::Changed)]);
}

/// The failure this exists to avoid: a project-wide list that repeats every
/// untouched scene is the per-row timeline's "forty identical backups"
/// problem, one level up.
#[test]
fn an_untouched_row_is_not_listed_at_all() {
    let then = vec![(0, version_row(1, "Chapter 1"), "same".to_string())];
    assert!(classify(&then, &[live(1, "Chapter 1", 0, "same")]).is_empty());
}

/// A row is named by what it is called *now* when it still exists, because
/// that is the name in the binder the writer is looking at.
#[test]
fn a_renamed_row_is_listed_under_the_name_it_has_today() {
    let then = vec![(0, version_row(1, "Working title"), "old".to_string())];
    let got = classify(&then, &[live(1, "The Garden Gate", 0, "new")]);
    assert_eq!(got[0].0, "The Garden Gate");
}

/// …and a row that is gone keeps the only name anyone ever gave it.
#[test]
fn a_deleted_row_keeps_the_name_it_had_when_it_existed() {
    let then = vec![(0, version_row(1, "Cut opening"), "d".to_string())];
    let got = classify(&then, &[]);
    assert_eq!(got[0].0, "Cut opening");
}

// ── the view-model's own state ──────────────────────────────────────────

#[test]
fn an_unsaved_project_scans_to_nothing_rather_than_to_an_error() {
    let vm = TimelineViewModel::new();
    vm.set_project(ProjectHandle::default());
    vm.scan();
    assert!(vm.moments().get().is_empty());
    assert!(vm.error().get().is_empty());
    assert!(!vm.loading().get());
}

#[test]
fn a_missing_project_scans_synchronously_with_no_executor() {
    let vm = TimelineViewModel::new();
    vm.set_project(ProjectHandle {
        path: "/nonexistent/Novel.skrib".into(),
        unique_id: "u".into(),
        destinations: vec!["/nonexistent".into()],
        revision: 0,
    });
    vm.scan();
    assert!(!vm.loading().get(), "the scan must have completed inline");
    assert!(vm.moments().get().is_empty());
}

/// **The bug this guards, and the worse half of it.** The Versions dock at
/// least came right on the next tab switch, because its key carries the
/// focused row. This band's only key is the project, so a backup taken
/// mid-session was invisible to it for the rest of the session — the writer
/// pressed "Back up now" and the band went on saying the project had no
/// recorded past at all.
#[test]
fn a_newly_recorded_version_makes_the_band_scan_again() {
    let vm = TimelineViewModel::new();
    let base = ProjectHandle {
        path: "/nonexistent/Novel.skrib".into(),
        unique_id: "u".into(),
        destinations: vec!["/nonexistent".into()],
        revision: 0,
    };
    vm.set_project(base.clone());
    vm.scan();
    let first = vm.scanned.borrow().clone();
    assert!(first.is_some(), "precondition: the first scan ran");

    vm.set_project(ProjectHandle {
        revision: 1,
        ..base
    });
    vm.scan();
    assert!(
        first.as_ref() != vm.scanned.borrow().as_ref(),
        "a recorded version has to invalidate the scan key",
    );
}

/// The dock calls `scan` from `build`, and a scan writes signals `build`
/// binds. Without the guard the first would schedule the second, forever.
#[test]
fn rescanning_the_same_project_does_nothing() {
    let vm = TimelineViewModel::new();
    vm.set_project(ProjectHandle {
        path: "/nonexistent/Novel.skrib".into(),
        unique_id: "u".into(),
        destinations: vec!["/nonexistent".into()],
        revision: 0,
    });
    vm.scan();
    vm.scan();
    assert!(!vm.loading().get());
}

#[test]
fn the_slider_lands_on_the_newest_moment_and_clamps_to_what_exists() {
    let vm = TimelineViewModel::new();
    assert_eq!(vm.index(), 0, "an empty timeline has no index to be out of");
    vm.finish(
        (0..3)
            .map(|i| Moment {
                at: Utc::now() + chrono::Duration::minutes(i),
                source: SourceKind::Backup,
                from: VersionRef {
                    path: PathBuf::from("/x"),
                    taken_at: Utc::now(),
                    source: SourceKind::Backup,
                },
                bytes: 100,
            })
            .collect(),
        String::new(),
    );
    assert_eq!(
        vm.index(),
        2,
        "opening on the newest is opening on the least"
    );

    // A stale thumb from a longer timeline must not index past the end.
    vm.position().set(99.0);
    assert_eq!(vm.index(), 2);
}

/// The sentence most writers will get most of the value from, since most of
/// them open these surfaces two or three times a year.
#[test]
fn coverage_says_how_much_is_kept_and_how_far_back_it_reaches() {
    let vm = TimelineViewModel::new();
    assert_eq!(
        vm.coverage(),
        None,
        "with nothing recorded the caller has to say *that*, not '0 since never'",
    );

    let oldest = Utc::now() - chrono::Duration::days(40);
    vm.finish(
        (0..3)
            .map(|i| Moment {
                at: oldest + chrono::Duration::days(i),
                source: SourceKind::Backup,
                from: VersionRef {
                    path: PathBuf::from("/x"),
                    taken_at: oldest,
                    source: SourceKind::Backup,
                },
                bytes: 10,
            })
            .collect(),
        String::new(),
    );
    let (count, back_to) = vm.coverage().expect("three moments are coverage");
    assert_eq!(count, 3);
    assert_eq!(
        back_to, oldest,
        "the reach is the *oldest* moment, which is the reassuring end of the list",
    );
}

#[test]
fn the_body_is_what_opening_a_row_shows_and_a_synopsis_is_the_fallback() {
    let mut row = version_row(1, "Chapter 1");
    assert!(main_blob(&row).is_some_and(|b| b.ends_with(".scene.djot")));

    row.prose = vec![(
        ContentRole::SynopsisText,
        "binders/01/text/1.synopsis.djot".to_string(),
        BlobStamp { bytes: 4 },
    )];
    assert!(
        main_blob(&row).is_some_and(|b| b.ends_with(".synopsis.djot")),
        "a row with only a synopsis opens on it rather than on nothing",
    );
}

/// **The bug this caught.** A Part or a chapter can carry an epigraph and
/// nothing else. The preference list left that role out, so the row resolved
/// to no blob — and, because the caller took an empty string for a path, it
/// opened a titled, dated, completely empty reader.
#[test]
fn a_row_whose_only_prose_is_an_epigraph_still_opens_on_it() {
    let mut row = version_row(1, "Part One");
    row.prose = vec![(
        ContentRole::EpigraphText,
        "binders/01/text/1.epigraph.djot".to_string(),
        BlobStamp { bytes: 40 },
    )];
    assert!(
        main_blob(&row).is_some_and(|b| b.ends_with(".epigraph.djot")),
        "an epigraph is prose, and it is the only prose this row has",
    );
}

/// …and a row that recorded nothing says so, rather than handing on a path
/// that is not one.
#[test]
fn a_row_with_no_recorded_prose_offers_nothing_to_read() {
    let mut row = version_row(1, "Front matter");
    row.prose = Vec::new();
    assert_eq!(main_blob(&row), None);

    let moment = Moment {
        at: Utc::now(),
        source: SourceKind::Backup,
        from: VersionRef {
            path: PathBuf::from("/x.skrib"),
            taken_at: Utc::now(),
            source: SourceKind::Backup,
        },
        bytes: 0,
    };
    assert_eq!(
        recorded_at(&moment, &row),
        None,
        "a real bundle paired with an empty blob path is not a readable source",
    );
}

/// The guard that keeps the epigraph bug from happening again under a
/// different name: every role that is prose has to be a role this can open.
#[test]
fn every_prose_role_is_something_this_can_open() {
    let all = [
        ContentRole::SceneText,
        ContentRole::NoteText,
        ContentRole::SynopsisText,
        ContentRole::EpigraphText,
        ContentRole::ParatextText,
        ContentRole::BookTitle,
        ContentRole::BookSubtitle,
        ContentRole::PartTitle,
        ContentRole::ChapterTitle,
    ];
    for role in all {
        // The same definition of "is this prose" the digest uses, so the two
        // sides cannot drift into disagreeing about what a row holds.
        if skrib_format::slug::prose_kind(&role).is_none() {
            continue;
        }
        let row = VersionRow {
            uid: uid(1),
            title: "x".into(),
            sub_role: Default::default(),
            indent: 0,
            prose: vec![(
                role.clone(),
                "binders/01/text/1.djot".to_string(),
                BlobStamp { bytes: 1 },
            )],
        };
        assert!(
            main_blob(&row).is_some(),
            "{role:?} is prose but a row carrying only it opens on nothing",
        );
    }
}
