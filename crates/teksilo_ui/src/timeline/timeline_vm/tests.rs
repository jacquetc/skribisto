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
        is_exportable: true,
        uid: uid(n),
        title: title.to_string(),
        sub_title: String::new(),
        role: Default::default(),
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
    assert!(
        main_blob(&row).is_some_and(|(role, b)| {
            role == ContentRole::SceneText && b.ends_with(".scene.djot")
        })
    );

    row.prose = vec![(
        ContentRole::SynopsisText,
        "binders/01/text/1.synopsis.djot".to_string(),
        BlobStamp { bytes: 4 },
    )];
    assert!(
        main_blob(&row).is_some_and(|(role, b)| {
            role == ContentRole::SynopsisText && b.ends_with(".synopsis.djot")
        }),
        "a row with only a synopsis opens on it rather than on nothing, and says which text that is",
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
        main_blob(&row).is_some_and(|(role, b)| {
            role == ContentRole::EpigraphText && b.ends_with(".epigraph.djot")
        }),
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
            is_exportable: true,
            uid: uid(1),
            title: "x".into(),
            sub_title: String::new(),
            role: Default::default(),
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

// ── one tick per moment ────────────────────────────────────────────────────

fn moment_at(secs: i64, bytes: u64, source: SourceKind) -> Moment {
    let at = DateTime::from_timestamp(secs, 0).unwrap();
    Moment {
        at,
        source,
        from: VersionRef {
            path: PathBuf::from(match source {
                SourceKind::Backup => "/backups/Novel.skrib",
                SourceKind::Log => "/Novel.skrib",
            }),
            taken_at: at,
            source,
        },
        bytes,
    }
}

/// **The record that can say more has to be the one that survives.** A "Back up
/// now" straight after a save records the same manuscript twice, at the same
/// instant, at the same size. Log entries are gathered first, so collapsing the
/// pair by instant alone kept the *log* — and against a log moment the band can
/// only ever report "edited", so that bar silently lost the ability to say
/// anything was deleted or moved, with a full bundle sitting right behind it.
#[test]
fn a_backup_outranks_a_log_entry_describing_the_same_instant() {
    let mut moments = vec![
        moment_at(1_700_000_000, 5_000, SourceKind::Log),
        moment_at(1_700_000_000, 5_000, SourceKind::Backup),
    ];
    one_tick_per_moment(&mut moments);

    assert_eq!(moments.len(), 1, "one instant, one tick");
    assert_eq!(
        moments[0].source,
        SourceKind::Backup,
        "the bundle answers all four kinds; the log answers one",
    );
    assert_eq!(
        moments[0].from.source,
        SourceKind::Backup,
        "and the reference kept has to be the bundle's, or nothing can be read from it",
    );
}

/// `dedup_by` only ever collapses *adjacent* pairs, so the sort has to make
/// equals adjacent. Three records of one instant sized 100, 200, 100 left two
/// identical-looking bars standing when the sort key was the instant alone.
#[test]
fn records_of_one_instant_collapse_however_their_sizes_interleave() {
    let mut moments = vec![
        moment_at(1_700_000_000, 100, SourceKind::Log),
        moment_at(1_700_000_000, 200, SourceKind::Log),
        moment_at(1_700_000_000, 100, SourceKind::Backup),
    ];
    one_tick_per_moment(&mut moments);

    assert_eq!(
        moments.len(),
        2,
        "two distinct sizes, two ticks: {moments:?}"
    );
    assert_eq!(
        moments[0].source,
        SourceKind::Backup,
        "and the pair that collapsed still kept the richer record",
    );
}

/// The band reads left to right, so whatever else the collapse does it has to
/// leave the moments in order.
#[test]
fn the_ticks_come_out_oldest_first() {
    let mut moments = vec![
        moment_at(1_700_000_300, 300, SourceKind::Backup),
        moment_at(1_700_000_100, 100, SourceKind::Log),
        moment_at(1_700_000_200, 200, SourceKind::Backup),
    ];
    one_tick_per_moment(&mut moments);
    let order: Vec<u64> = moments.iter().map(|m| m.bytes).collect();
    assert_eq!(order, vec![100, 200, 300]);
}

/// **What the band has to say out loud.** Only a backup is a whole bundle. The
/// in-project history log records prose and nothing else, so against a log moment
/// the change list can report "edited" and nothing more — and a writer who is not
/// told reads that silence as *nothing was deleted or moved this morning*, which
/// is a claim about the record, not about their book.
#[test]
fn the_band_can_tell_a_prose_only_record_from_a_whole_bundle() {
    let vm = TimelineViewModel::new();
    vm.seed_moments_for_test(vec![
        moment_at(1_700_000_000, 1_000, SourceKind::Backup),
        moment_at(1_700_000_100, 1_100, SourceKind::Log),
    ]);

    // The band opens on the newest moment, which here is the save.
    assert_eq!(vm.selected().map(|m| m.source), Some(SourceKind::Log));
    assert!(
        vm.selected_is_prose_only(),
        "a save records text alone and the list beside it has to say so",
    );

    vm.position().set(0.0);
    assert_eq!(vm.selected().map(|m| m.source), Some(SourceKind::Backup));
    assert!(
        !vm.selected_is_prose_only(),
        "a bundle answers all four kinds, so the caveat would be a lie",
    );
}

/// With nothing recorded there is no moment to qualify, and the caveat must not
/// appear over an empty band.
#[test]
fn an_empty_history_makes_no_claim_about_its_records() {
    assert!(!TimelineViewModel::new().selected_is_prose_only());
}

// ── the way back for a row that is gone ─────────────────────────────────────

fn a_backup_moment() -> Moment {
    moment_at(0, 0, SourceKind::Backup)
}

/// **A row is put back as itself, or not at all.** The reader opens on one text;
/// a writer bringing back a cut chapter means the chapter — its body, its
/// synopsis, its epigraph — and dropping the rest on the way would be a second
/// silent loss on top of the one being recovered.
#[test]
fn what_a_removed_row_carries_back_is_the_whole_row() {
    let mut row = version_row(1, "The lost chapter");
    row.role = common::entities::BinderItemRole::Folder;
    row.sub_role = common::entities::BinderItemSubRole::ChapterScene;
    row.indent = 2;
    row.prose.push((
        ContentRole::SynopsisText,
        "binders/01/text/1.synopsis.djot".to_string(),
        BlobStamp { bytes: 4 },
    ));

    let gone = gone_row(&a_backup_moment(), &row);
    assert_eq!(gone.role, common::entities::BinderItemRole::Folder);
    assert_eq!(
        gone.sub_role,
        common::entities::BinderItemSubRole::ChapterScene,
    );
    assert_eq!(
        gone.title, "The lost chapter",
        "the name it had then — there is no live row to take one from",
    );
    assert_eq!(gone.indent, 2);
    assert_eq!(gone.prose.len(), 2, "body and synopsis, not just the body");
    assert!(
        gone.prose
            .iter()
            .any(|(r, _)| r == &ContentRole::SynopsisText)
    );
}

/// **An export exclusion is the writer's decision, and survives the round trip.**
/// Leaving a draft or a note out of the book is a choice made about that row; a
/// recreate that silently re-included it would undo the choice with nothing on
/// screen to say so. Unlike `indent` — a fact about a tree that has since moved
/// on, and deliberately dropped — this one is restored exactly.
#[test]
fn a_removed_row_carries_back_its_export_exclusion() {
    let mut row = version_row(2, "A note kept out of the book");
    row.is_exportable = false;

    let gone = gone_row(&a_backup_moment(), &row);
    assert!(
        !gone.is_exportable,
        "the exclusion the writer set must come back with the row, not default to included",
    );

    // And the ordinary case still reads as included.
    let included = gone_row(&a_backup_moment(), &version_row(3, "An ordinary scene"));
    assert!(included.is_exportable);
}

/// **A Book's subtitle is a field, not prose.** Nothing that walks `prose` would
/// notice it missing, so a Book put back without it comes back silently untitled
/// underneath — and there is no version left to read it out of.
#[test]
fn a_books_second_name_comes_back_with_it() {
    let mut row = version_row(1, "The Novel");
    row.role = common::entities::BinderItemRole::Folder;
    row.sub_role = common::entities::BinderItemSubRole::Book;
    row.sub_title = "A Story of the Forge".into();
    let gone = gone_row(&a_backup_moment(), &row);
    assert_eq!(gone.sub_title, "A Story of the Forge");
}

/// **`role` cannot be derived from `sub_role`.** `Part`, `ChapterScene` and
/// `Paratext` are each valid under both `Item` and `Folder`, so a row put back
/// without it could return as a flat marker where it was the container holding
/// the rest of the chapter.
#[test]
fn the_container_axis_survives_the_round_trip() {
    for role in [
        common::entities::BinderItemRole::Item,
        common::entities::BinderItemRole::Folder,
    ] {
        let mut row = version_row(1, "A part");
        row.role = role.clone();
        row.sub_role = common::entities::BinderItemSubRole::Part;
        assert_eq!(gone_row(&a_backup_moment(), &row).role, role);
    }
}

/// An empty blob path is not a path — the same reason `main_blob` filters one
/// out. Carrying it would make the recreate read a blob that is not there and
/// refuse the whole row.
#[test]
fn a_recorded_role_with_no_blob_behind_it_is_not_carried() {
    let mut row = version_row(1, "A scene");
    row.prose.push((
        ContentRole::SynopsisText,
        String::new(),
        BlobStamp { bytes: 0 },
    ));
    let gone = gone_row(&a_backup_moment(), &row);
    assert_eq!(gone.prose.len(), 1);
    assert!(gone.prose.iter().all(|(_, blob)| !blob.is_empty()));
}
