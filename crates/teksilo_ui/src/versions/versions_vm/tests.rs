// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

use super::*;

#[test]
fn synopsis_leads_because_it_can_be_read_at_a_glance() {
    assert_eq!(VersionScope::default(), VersionScope::Synopsis);
    assert_eq!(VersionScope::Synopsis.roles(), &[ContentRole::SynopsisText]);
}

#[test]
fn the_body_scope_covers_every_kind_of_rows_text() {
    // The dock must not need to know which kind of row it is showing.
    let roles = VersionScope::Prose.roles();
    assert!(roles.contains(&ContentRole::SceneText));
    assert!(roles.contains(&ContentRole::NoteText));
    assert!(
        roles.contains(&ContentRole::ParatextText),
        "a preface's body is its own role, and without it the dock never looks",
    );
    assert_eq!(
        roles.last(),
        Some(&ContentRole::EpigraphText),
        "a Part's epigraph is its only text, and comes after every body role",
    );
}

#[test]
fn nothing_focused_clears_the_timeline_without_touching_the_disk() {
    let vm = VersionsViewModel::new();
    vm.set_project(ProjectHandle {
        path: "/definitely/not/a/project.skrib".into(),
        ..Default::default()
    });
    vm.load_for(None);
    assert!(vm.view().get().is_empty());
    assert!(vm.error().get().is_empty(), "no target is not an error");
    assert!(!vm.loading().get());
}

#[test]
fn an_unsaved_project_reports_no_past_rather_than_an_error() {
    // A project that has never been written has nowhere to have kept history,
    // which is a normal state and must not read as a failure.
    let vm = VersionsViewModel::new();
    vm.set_project(ProjectHandle::default());
    vm.load_for(Some(uuid::Uuid::from_u128(1)));
    assert!(vm.view().get().is_empty());
    assert!(vm.error().get().is_empty());
    assert!(!vm.loading().get());
}

#[test]
fn a_missing_project_file_yields_an_empty_timeline_synchronously() {
    // With no executor the scan runs inline, so a headless caller gets a real
    // answer instead of a permanent "loading" — the same degradation
    // `BackupsListViewModel` relies on.
    let vm = VersionsViewModel::new();
    vm.set_project(ProjectHandle {
        path: "/nonexistent/Novel.skrib".into(),
        unique_id: "u".into(),
        destinations: vec!["/nonexistent".into()],
        revision: 0,
    });
    vm.load_for(Some(uuid::Uuid::from_u128(1)));
    assert!(!vm.loading().get(), "the load must have completed inline");
    assert!(vm.view().get().is_empty());
}

/// The dock calls `load_for` from `build`, and a load writes signals `build`
/// binds. Without the guard the first scan schedules the second, forever.
#[test]
fn repeating_a_load_for_the_same_row_does_not_scan_again() {
    let vm = VersionsViewModel::new();
    vm.set_project(ProjectHandle {
        path: "/nonexistent/Novel.skrib".into(),
        unique_id: "u".into(),
        destinations: vec!["/nonexistent".into()],
        revision: 0,
    });
    let uid = Some(uuid::Uuid::from_u128(1));
    vm.load_for(uid);
    let first = vm.view().get();
    // A second call must not even reach `loading`, which is the signal that
    // would retrigger the build that called it.
    vm.load_for(uid);
    assert!(!vm.loading().get());
    assert!(vm.view().get() == first);
}

/// **The bug this guards.** A backup taken mid-session writes a new version
/// of every row, and nothing about *where* the project lives changes — so a
/// dock that had already loaded kept showing its stale answer. It only ever
/// came right by accident, when a tab switch changed the other half of the
/// key.
#[test]
fn a_newly_recorded_version_makes_the_dock_look_again() {
    let vm = VersionsViewModel::new();
    let base = ProjectHandle {
        path: "/nonexistent/Novel.skrib".into(),
        unique_id: "u".into(),
        destinations: vec!["/nonexistent".into()],
        revision: 0,
    };
    vm.set_project(base.clone());
    let uid = Some(uuid::Uuid::from_u128(1));
    vm.load_for(uid);
    assert!(
        vm.loaded.borrow().is_some(),
        "precondition: the first load ran",
    );

    // Same row, same scope, same project — and the guard must still let this
    // one through, because the past itself moved.
    vm.set_project(ProjectHandle {
        revision: 1,
        ..base.clone()
    });
    let before = vm.loaded.borrow().clone();
    vm.load_for(uid);
    assert!(
        before != *vm.loaded.borrow(),
        "a recorded version has to invalidate the load key, not just the path",
    );
}

#[test]
fn changing_the_scope_does_start_a_new_scan() {
    let vm = VersionsViewModel::new();
    vm.set_project(ProjectHandle {
        path: "/nonexistent/Novel.skrib".into(),
        unique_id: "u".into(),
        destinations: vec!["/nonexistent".into()],
        revision: 0,
    });
    let uid = Some(uuid::Uuid::from_u128(1));
    vm.load_for(uid);
    vm.set_scope(VersionScope::Prose);
    vm.load_for(uid);
    // Reaching the end without the guard swallowing it is the assertion; the
    // scan itself finds nothing, which the previous test already covers.
    assert!(vm.error().get().is_empty());
}

// ── diffing ─────────────────────────────────────────────────────────────

/// A hand-built view, so the diff logic is testable without a filesystem.
///
/// Each change's `from.path` is its own index, so a stub pin store can answer
/// "is this one pinned" without touching a disk.
fn view_with(texts: &[&str], absent_at: Option<chrono::DateTime<chrono::Utc>>) -> TimelineView {
    let now = chrono::Utc::now();
    let changes: Vec<Change> = (0..texts.len())
        .map(|i| Change {
            at: now - chrono::Duration::minutes(i as i64),
            source: SourceKind::Backup,
            from: skrib_format::versions::VersionRef {
                path: std::path::PathBuf::from(i.to_string()),
                taken_at: now,
                source: SourceKind::Backup,
            },
            blob_path: String::new(),
            hash: format!("h{i}"),
            bytes: 0,
            title: String::new(),
        })
        .collect();
    TimelineView {
        timeline: Timeline {
            absent_at,
            changes,
            ..Default::default()
        },
        texts: texts.iter().map(|t| t.to_string()).collect(),
        magnitudes: Vec::new(),
        role: Some(ContentRole::SceneText),
    }
}

#[test]
fn a_version_is_compared_with_the_one_before_it_not_with_the_current_text() {
    // Newest first, so index 1 is what index 0 replaced.
    let view = view_with(&["the lamp guttered", "the lamp went out"], None);
    assert_eq!(view.predecessor(0), Some("the lamp went out"));
}

/// The trap this guards: the oldest recorded state is not necessarily the
/// moment the row was written.
#[test]
fn the_earliest_state_is_not_reported_as_newly_written_unless_it_was() {
    let unknown = view_with(&["the lamp went out"], None);
    assert_eq!(
        unknown.predecessor(0),
        None,
        "with nothing older on record, a diff would be a guess",
    );

    let known = view_with(&["the lamp went out"], Some(chrono::Utc::now()));
    assert_eq!(
        known.predecessor(0),
        Some(""),
        "a version in which the row was absent proves it was created here",
    );
}

#[test]
fn selecting_a_version_produces_a_diff_against_its_predecessor() {
    let vm = VersionsViewModel::new();
    *vm.loaded.borrow_mut() = Some(LoadKey {
        uid: Some(uuid::Uuid::from_u128(1)),
        scope: 0,
        project: ProjectHandle::default(),
    });
    vm.view
        .set(view_with(&["the lamp guttered", "the lamp went out"], None));
    vm.selection.select(0);
    vm.sync_diff();

    let diff = vm.diff().get().expect("a version with a predecessor diffs");
    assert!(diff.summary.words_added > 0);
    assert!(diff.summary.words_removed > 0);
    assert!(!vm.selection_is_earliest());
}

#[test]
fn the_earliest_version_offers_no_diff_and_says_which_state_it_is_in() {
    let vm = VersionsViewModel::new();
    *vm.loaded.borrow_mut() = Some(LoadKey {
        uid: Some(uuid::Uuid::from_u128(1)),
        scope: 0,
        project: ProjectHandle::default(),
    });
    vm.view.set(view_with(&["the lamp went out"], None));
    vm.selection.select(0);
    vm.sync_diff();

    assert!(vm.diff().get().is_none());
    assert!(
        vm.selection_is_earliest(),
        "the pane must be able to tell 'nothing older' from 'nothing changed'",
    );
}

#[test]
fn repeating_the_diff_for_the_same_selection_recomputes_nothing() {
    let vm = VersionsViewModel::new();
    *vm.loaded.borrow_mut() = Some(LoadKey {
        uid: Some(uuid::Uuid::from_u128(1)),
        scope: 0,
        project: ProjectHandle::default(),
    });
    vm.view
        .set(view_with(&["the lamp guttered", "the lamp went out"], None));
    vm.selection.select(0);
    vm.sync_diff();
    let first = vm.diff().get();
    vm.sync_diff();
    assert!(vm.diff().get() == first);
}

/// **The bug this guards.** With "pinned only" on, the list is a projection:
/// the cursor's position in it is not the position in the timeline. Read
/// straight through, selecting the first *visible* row diffed — and would
/// have restored — a completely different version's text than the one on
/// screen.
#[test]
fn a_filtered_list_still_selects_the_version_the_writer_can_see() {
    let vm = VersionsViewModel::new();
    *vm.loaded.borrow_mut() = Some(LoadKey {
        uid: Some(uuid::Uuid::from_u128(1)),
        scope: 0,
        project: ProjectHandle::default(),
    });
    // Newest first; only the third is pinned.
    vm.view
        .set(view_with(&["newest", "middle", "oldest"], None));
    let pinned = std::rc::Rc::new(std::cell::RefCell::new(vec![false, false, true]));
    {
        let by_path = pinned.clone();
        vm.set_pins(Pins {
            is_pinned: Rc::new(move |p: &std::path::Path| {
                let i: usize = p.to_string_lossy().parse().unwrap_or(0);
                by_path.borrow().get(i).copied().unwrap_or(false)
            }),
            set: Rc::new(|_, _| {}),
        });
    }

    assert_eq!(vm.visible_indices(), vec![0, 1, 2], "unfiltered: all three");
    vm.toggle_pinned_only();
    assert_eq!(
        vm.visible_indices(),
        vec![2],
        "filtered: only the pinned one is on screen",
    );

    // The writer selects the one row they can see — list position 0.
    vm.selection.select(0);
    assert_eq!(
        vm.selected_index(),
        Some(2),
        "position 0 of a filtered list is the *third* version, not the first",
    );
    let req = vm
        .restore_request(7)
        .expect("the selected version can be restored");
    assert_eq!(
        req.past, "oldest",
        "restore must write back the version the writer actually looked at",
    );
}

/// The same trap as "pinned only", one filter later: a date range is another
/// projection, and everything acting on the selection has to come back
/// through it or Restore writes a version the writer never looked at.
#[test]
fn a_date_filtered_list_still_restores_the_version_on_screen() {
    use teksilo::widgets::DateRange;
    let vm = VersionsViewModel::new();
    *vm.loaded.borrow_mut() = Some(LoadKey {
        uid: Some(uuid::Uuid::from_u128(1)),
        scope: 0,
        project: ProjectHandle::default(),
    });
    // `view_with` dates them one minute apart, newest first.
    vm.view
        .set(view_with(&["newest", "middle", "oldest"], None));
    assert_eq!(vm.visible_count(), 3);

    // A range covering only the oldest — two minutes back, one day wide.
    let oldest = vm.view.get().timeline.changes[2].at;
    let day = crate::date_convert::to_jiff_date(oldest).unwrap();
    vm.range().set(Some(DateRange::new(day, day)));

    // All three fall on the same day here, so narrow to a day that holds none
    // and check the dock is told, rather than shown an empty list.
    let elsewhere = crate::date_convert::to_jiff_date(oldest - chrono::Duration::days(30)).unwrap();
    vm.range().set(Some(DateRange::new(elsewhere, elsewhere)));
    assert_eq!(vm.visible_count(), 0, "the filter excludes every version");
    assert!(
        vm.is_filtered(),
        "and the dock can say why the list is empty"
    );
    assert_eq!(
        vm.selected_index(),
        None,
        "nothing on screen means nothing selected, so nothing to restore",
    );

    vm.clear_filters();
    assert_eq!(vm.visible_count(), 3);
    assert!(!vm.is_filtered());
}

/// The preset sets a real range, and — like every other filter here — drops
/// a cursor that pointed into the unfiltered list.
#[test]
fn the_recent_preset_sets_a_range_and_clears_the_cursor() {
    let vm = VersionsViewModel::new();
    *vm.loaded.borrow_mut() = Some(LoadKey {
        uid: Some(uuid::Uuid::from_u128(1)),
        scope: 0,
        project: ProjectHandle::default(),
    });
    vm.view
        .set(view_with(&["newest", "middle", "oldest"], None));
    vm.selection.select(1);
    assert!(!vm.is_filtered());

    vm.set_last_days(30);
    let range = vm.range().get().expect("the preset sets a range");
    assert!(vm.is_filtered());
    assert!(
        vm.selection().selected_indices().is_empty(),
        "the list is about to renumber under a positional cursor",
    );

    // `view_with` dates its changes minutes ago, so all three are inside the
    // last thirty days — the preset narrows without hiding recent work.
    assert_eq!(vm.visible_count(), 3);
    let today = crate::date_convert::today_utc().expect("a sane clock");
    assert_eq!(range.end, today, "the window ends today");
    assert!(range.start < today, "and reaches back before it");
}

/// A filter that hides the selected row must not leave the cursor pointing at
/// a version the writer can no longer see.
#[test]
fn narrowing_the_range_does_not_leave_a_hidden_version_selected() {
    use teksilo::widgets::DateRange;
    let vm = VersionsViewModel::new();
    *vm.loaded.borrow_mut() = Some(LoadKey {
        uid: Some(uuid::Uuid::from_u128(1)),
        scope: 0,
        project: ProjectHandle::default(),
    });
    vm.view
        .set(view_with(&["newest", "middle", "oldest"], None));
    vm.selection.select(2);
    assert_eq!(vm.selected_index(), Some(2));

    let far = crate::date_convert::to_jiff_date(
        vm.view.get().timeline.changes[0].at - chrono::Duration::days(365),
    )
    .unwrap();
    vm.range().set(Some(DateRange::new(far, far)));
    assert_eq!(
        vm.selected_index(),
        None,
        "position 2 of an empty projection is not a version",
    );
    assert!(
        vm.restore_request(7).is_none(),
        "and there is nothing to put back"
    );
}

#[test]
fn clearing_the_selection_clears_the_diff() {
    let vm = VersionsViewModel::new();
    *vm.loaded.borrow_mut() = Some(LoadKey {
        uid: Some(uuid::Uuid::from_u128(1)),
        scope: 0,
        project: ProjectHandle::default(),
    });
    vm.view
        .set(view_with(&["the lamp guttered", "the lamp went out"], None));
    vm.selection.select(0);
    vm.sync_diff();
    assert!(vm.diff().get().is_some());

    vm.selection.clear();
    vm.sync_diff();
    assert!(vm.diff().get().is_none());
}

/// Loading a different row must not leave the previous row's cursor behind:
/// index 3 of one timeline is not index 3 of another.
#[test]
fn loading_another_row_drops_the_previous_selection_and_diff() {
    let vm = VersionsViewModel::new();
    vm.set_project(ProjectHandle {
        path: "/nonexistent/Novel.skrib".into(),
        unique_id: "u".into(),
        destinations: vec!["/nonexistent".into()],
        revision: 0,
    });
    vm.load_for(Some(uuid::Uuid::from_u128(1)));
    vm.selection.select(0);
    vm.load_for(Some(uuid::Uuid::from_u128(2)));
    assert!(vm.selection().selected_indices().is_empty());
    assert!(vm.diff().get().is_none());
}

#[test]
fn magnitudes_line_up_with_the_changes_they_describe() {
    let view = measure_texts(&["a b c d e f g h", "a b c d e f g X", "a b c d e f g h"]);
    assert_eq!(view.magnitudes.len(), 3);
    assert!(
        view.magnitudes[0].is_some_and(|m| m > 0.0),
        "one word changed against the state below it",
    );
    assert_eq!(
        view.magnitudes[2], None,
        "the oldest state has nothing behind it to measure against",
    );
}

/// `measure` without touching a filesystem: the reading half is exercised by
/// the timeline tests, the comparing half is what matters here.
fn measure_texts(texts: &[&str]) -> TimelineView {
    let mut view = TimelineView {
        timeline: Timeline::default(),
        texts: texts.iter().map(|t| t.to_string()).collect(),
        magnitudes: Vec::new(),
        role: Some(ContentRole::SceneText),
    };
    view.magnitudes = (0..view.texts.len())
        .map(|i| {
            view.predecessor(i)
                .map(|older| diff_djot(older, &view.texts[i]).magnitude())
        })
        .collect();
    view
}

// ── the pin note: what the list can and cannot promise ──────────────────────

/// Same shape as [`view_with`], but every change comes from the project's own
/// history log — the half of the list that can carry no pin at all.
fn log_view(texts: &[&str]) -> TimelineView {
    let mut view = view_with(texts, None);
    for c in view.timeline.changes.iter_mut() {
        c.source = SourceKind::Log;
        c.from.source = SourceKind::Log;
    }
    view
}

/// **The confusion the note exists for.** `pin_state` leaves a log row with no
/// control at all, which is the right call — a disabled one asks "why?" without
/// answering — but the absence then teaches nothing, while the tooltip on the
/// rows that *do* have one promises automatic cleanup will never delete the
/// version.
#[test]
fn a_list_holding_a_version_that_cannot_be_pinned_says_so() {
    let vm = VersionsViewModel::new();
    vm.view.set(log_view(&["newest", "older"]));
    assert!(
        vm.shows_unpinnable(),
        "a log-sourced row on screen is exactly when the note applies",
    );
}

#[test]
fn a_list_of_backups_alone_needs_no_note() {
    let vm = VersionsViewModel::new();
    vm.view.set(view_with(&["newest", "older"], None));
    assert!(!vm.shows_unpinnable());
}

/// Keyed on the **source**, never on `pin_state`: that is also `None` before the
/// shell has installed the pin store, and the note would then appear over a list
/// of backups whose pins are merely late.
#[test]
fn the_note_does_not_appear_just_because_the_pin_store_is_not_installed_yet() {
    let vm = VersionsViewModel::new();
    vm.view.set(view_with(&["newest"], None));
    assert!(vm.pin_state(&vm.view.get().timeline.changes[0]).is_none());
    assert!(!vm.shows_unpinnable());
}

/// **"Pinned only" over a past the log holds can never match.** Saying only "no
/// version matches the filters you've set" sends the writer looking for pins
/// they could not have made.
#[test]
fn an_empty_pinned_list_says_why_it_can_be_empty() {
    let vm = VersionsViewModel::new();
    vm.view.set(log_view(&["newest", "older"]));
    vm.set_pins(Pins {
        is_pinned: Rc::new(|_: &std::path::Path| false),
        set: Rc::new(|_, _| {}),
    });
    assert!(!vm.pinned_filter_found_nothing(), "no filter, no sentence");
    vm.toggle_pinned_only();
    assert_eq!(vm.visible_count(), 0);
    assert!(vm.pinned_filter_found_nothing());
}

/// **And only where the reason applies.** On a list of backups nobody has pinned
/// yet, "only versions from a backup can be pinned" is true, irrelevant, and
/// reads as an explanation for an emptiness it did not cause.
#[test]
fn an_all_backup_list_is_empty_without_being_told_why() {
    let vm = VersionsViewModel::new();
    vm.view.set(view_with(&["newest", "older"], None));
    vm.set_pins(Pins {
        is_pinned: Rc::new(|_: &std::path::Path| false),
        set: Rc::new(|_, _| {}),
    });
    vm.toggle_pinned_only();
    assert!(vm.pinned_filter_found_nothing(), "nothing is pinned");
    assert!(
        !vm.has_unpinnable_versions(),
        "…but every version here could have been, so the reason does not apply",
    );
}

/// `has_unpinnable_versions` asks about the **whole** timeline, not the rows on
/// screen — under "pinned only" there are none, so `shows_unpinnable` would
/// answer no for every empty list and the reason would never be attached.
#[test]
fn the_reason_survives_the_filter_that_hid_the_rows_it_is_about() {
    let vm = VersionsViewModel::new();
    vm.view.set(log_view(&["newest", "older"]));
    vm.set_pins(Pins {
        is_pinned: Rc::new(|_: &std::path::Path| false),
        set: Rc::new(|_, _| {}),
    });
    vm.toggle_pinned_only();
    assert_eq!(vm.visible_count(), 0);
    assert!(
        !vm.shows_unpinnable(),
        "nothing is on screen to be unpinnable"
    );
    assert!(
        vm.has_unpinnable_versions(),
        "but the past this list is of still holds one",
    );
}

/// …and stays quiet when the filter is simply narrowing a list that does have
/// pins in it, where the generic sentence is the right one.
#[test]
fn a_pinned_list_with_something_in_it_is_not_the_empty_case() {
    let vm = VersionsViewModel::new();
    vm.view.set(view_with(&["newest", "older"], None));
    vm.set_pins(Pins {
        is_pinned: Rc::new(|p: &std::path::Path| p.to_string_lossy() == "1"),
        set: Rc::new(|_, _| {}),
    });
    vm.toggle_pinned_only();
    assert_eq!(vm.visible_count(), 1);
    assert!(!vm.pinned_filter_found_nothing());
}

// ── the past of every kind of row ───────────────────────────────────────────

/// Write a one-row project and save it twice, the way `save_work` does: carry
/// the log forward, record the state, write. `bodies(n)` is the row's content
/// at the `n`th save.
fn saved_twice(
    dir: &std::path::Path,
    item: common::entities::BinderItem,
    bodies: impl Fn(usize) -> Vec<common::entities::Content>,
) -> String {
    use common::entities::{Binder, Work};
    use skrib_format::{BinderWithItems, ItemWithContents, ShapeTag, SkribShape};

    let path = dir.join("Novel.skrib").to_string_lossy().into_owned();
    let first_save = chrono::DateTime::parse_from_rfc3339("2026-09-01T09:00:00Z")
        .expect("a valid instant")
        .with_timezone(&chrono::Utc);
    let project = |n: usize| {
        let work = Work {
            id: 1,
            unique_id: "one-row-project".into(),
            binders: vec![100],
            ..Default::default()
        };
        let binders = vec![BinderWithItems {
            binder: Binder {
                id: 100,
                uid: uuid::Uuid::from_u128(0x7003),
                name: "Manuscript".into(),
                activated: true,
                ..Default::default()
            },
            items: vec![ItemWithContents {
                item: item.clone(),
                contents: bodies(n),
            }],
        }];
        skrib_format::from_entities(
            &work,
            &[],
            &[],
            &[],
            &[],
            &[],
            &[],
            Default::default(),
            None,
            &[],
            &[],
            &[],
            &[],
            &[],
            &binders,
            ShapeTag::Zip,
        )
    };

    let mut bundle = project(0);
    skrib_format::history::record(&mut bundle, first_save);
    skrib_format::write_bundle(&path, SkribShape::ZipFile, &bundle).expect("first save");

    let mut bundle = project(1);
    bundle.history = skrib_format::history::load(&path);
    skrib_format::history::record(&mut bundle, first_save + chrono::Duration::hours(1));
    skrib_format::write_bundle(&path, SkribShape::ZipFile, &bundle).expect("second save");
    path
}

/// A row of the given kind.
fn row(
    uid: uuid::Uuid,
    title: &str,
    role: common::entities::BinderItemRole,
    sub_role: common::entities::BinderItemSubRole,
) -> common::entities::BinderItem {
    common::entities::BinderItem {
        id: 300,
        uid,
        title: title.into(),
        role,
        sub_role,
        activated: true,
        is_exportable: true,
        ..Default::default()
    }
}

/// One content row holding `text`.
fn body(id: u64, role: ContentRole, text: &str) -> common::entities::Content {
    common::entities::Content {
        id,
        uid: uuid::Uuid::from_u128(0x7100 + u128::from(id)),
        activated: true,
        role,
        data: text.to_string(),
        ..Default::default()
    }
}

/// The prose scope's timeline for the one row of the project at `path`.
fn prose_versions_of(path: String, uid: uuid::Uuid) -> (VersionsViewModel, TimelineView) {
    let vm = VersionsViewModel::new();
    vm.set_project(ProjectHandle {
        path,
        unique_id: "one-row-project".into(),
        destinations: Vec::new(),
        revision: 0,
    });
    vm.set_scope(VersionScope::Prose);
    vm.load_for(Some(uid));
    assert!(!vm.loading().get(), "with no executor the scan runs inline");
    assert!(vm.error().get().is_empty(), "{}", vm.error().get());
    let view = vm.view().get();
    (vm, view)
}

/// **P-09.** A preface's body is `ParatextText`, which the prose scope did not
/// name, so the dock looked for a scene's or a note's past on a row that has
/// neither and reported a paratext with two saved wordings as having no history
/// at all.
#[test]
fn a_paratext_row_with_two_recorded_wordings_shows_two_versions() {
    use common::entities::{BinderItemRole, BinderItemSubRole};
    let dir = tempfile::tempdir().expect("tmp");
    let preface = uuid::Uuid::from_u128(0x7001);
    let wordings = [
        "For my mother.",
        "For my mother, who read everything first.",
    ];
    let path = saved_twice(
        dir.path(),
        row(
            preface,
            "Preface",
            BinderItemRole::Item,
            BinderItemSubRole::Paratext,
        ),
        |n| vec![body(1, ContentRole::ParatextText, wordings[n])],
    );

    let (vm, view) = prose_versions_of(path, preface);
    assert_eq!(view.role, Some(ContentRole::ParatextText));
    assert_eq!(
        vm.visible_count(),
        2,
        "two saved wordings of the preface are two versions",
    );
    assert_eq!(
        view.texts,
        vec![wordings[1].to_string(), wordings[0].to_string()],
        "newest first, each with the wording it recorded",
    );
}

/// A Part's only text is its epigraph, and the prose scope did not name that
/// role either: a Part with two recorded epigraphs showed no history at all.
#[test]
fn a_parts_epigraph_with_two_recorded_wordings_shows_two_versions() {
    use common::entities::{BinderItemRole, BinderItemSubRole};
    let dir = tempfile::tempdir().expect("tmp");
    let part = uuid::Uuid::from_u128(0x7004);
    let wordings = ["All happy families.", "Happy families are all alike."];
    let path = saved_twice(
        dir.path(),
        row(
            part,
            "Part One",
            BinderItemRole::Folder,
            BinderItemSubRole::Part,
        ),
        |n| vec![body(1, ContentRole::EpigraphText, wordings[n])],
    );

    let (vm, view) = prose_versions_of(path, part);
    assert_eq!(view.role, Some(ContentRole::EpigraphText));
    assert_eq!(vm.visible_count(), 2);
    assert_eq!(
        view.texts,
        vec![wordings[1].to_string(), wordings[0].to_string()],
    );
}

/// A chapter carries an epigraph *and* prose. "Text" is its prose: the
/// epigraph is offered only to a row that has nothing else.
#[test]
fn a_chapters_text_is_its_prose_even_with_an_epigraph() {
    use common::entities::{BinderItemRole, BinderItemSubRole};
    let dir = tempfile::tempdir().expect("tmp");
    let chapter = uuid::Uuid::from_u128(0x7005);
    let path = saved_twice(
        dir.path(),
        row(
            chapter,
            "Chapter One",
            BinderItemRole::Folder,
            BinderItemSubRole::ChapterScene,
        ),
        |n| {
            vec![
                body(
                    1,
                    ContentRole::EpigraphText,
                    ["Epigraph.", "An epigraph."][n],
                ),
                body(
                    2,
                    ContentRole::SceneText,
                    ["It began.", "It began at night."][n],
                ),
            ]
        },
    );

    let (_, view) = prose_versions_of(path, chapter);
    assert_eq!(view.role, Some(ContentRole::SceneText));
    assert_eq!(
        view.texts,
        vec!["It began at night.".to_string(), "It began.".to_string()],
    );
}

// ── dates, on the writer's calendar ─────────────────────────────────────────

fn at(rfc3339: &str) -> chrono::DateTime<chrono::Utc> {
    chrono::DateTime::parse_from_rfc3339(rfc3339)
        .expect("a valid test instant")
        .with_timezone(&chrono::Utc)
}

/// A view whose changes were recorded at exactly `stamps`, newest first.
fn view_at(stamps: &[&str]) -> TimelineView {
    let mut view = view_with(&vec!["text"; stamps.len()], None);
    for (change, stamp) in view.timeline.changes.iter_mut().zip(stamps) {
        change.at = at(stamp);
    }
    view
}

/// **The defect.** A row saved at 21:00 on the 4th in New York is stamped the
/// 5th in UTC. The list labels it the 4th now, and the date filter has to agree
/// with the label, or a writer who picks the 4th watches that row disappear.
#[test]
fn the_date_filter_keeps_the_rows_its_labels_put_on_that_day() {
    use crate::shared::stamps::{Zone, override_writer_zone};
    use teksilo::widgets::DateRange;
    let _new_york = override_writer_zone(Zone::new_york());
    let vm = VersionsViewModel::new();
    vm.view.set(view_at(&[
        "2026-03-06T02:00:00Z", // 21:00 on the 5th in New York
        "2026-03-05T02:00:00Z", // 21:00 on the 4th
        "2026-03-04T15:00:00Z", // 10:00 on the 4th
    ]));

    let day = jiff::civil::date(2026, 3, 4);
    vm.range().set(Some(DateRange::new(day, day)));
    assert_eq!(
        vm.visible_indices(),
        vec![1, 2],
        "both of the writer's saves on the 4th, and not the one on the 5th",
    );
}

/// "The last thirty days" ends on the writer's today, not UTC's.
#[test]
fn the_recent_preset_ends_on_the_writers_today() {
    use crate::shared::stamps::{Zone, override_writer_zone};
    let _ahead = override_writer_zone(Zone::a_day_ahead());
    let vm = VersionsViewModel::new();
    vm.set_last_days(30);
    let range = vm.range().get().expect("the preset sets a range");
    assert_eq!(Some(range.end), Zone::a_day_ahead().today());
    assert_ne!(
        Some(range.end),
        crate::date_convert::today_utc(),
        "precondition: this zone's today is never UTC's",
    );
}

// ── what one load reads ─────────────────────────────────────────────────────

/// Backups that hold one row with no body text at all, as a Book's is, and
/// count how often they are listed and read.
struct BookInEveryBackup {
    uid: uuid::Uuid,
    versions: Vec<skrib_format::versions::VersionRef>,
    lists: std::cell::Cell<usize>,
    indexes: std::cell::Cell<usize>,
}

impl BookInEveryBackup {
    fn new(uid: uuid::Uuid, count: i64) -> Self {
        let first = at("2026-03-01T10:00:00Z");
        Self {
            uid,
            versions: (0..count)
                .map(|n| skrib_format::versions::VersionRef {
                    path: format!("/backups/Novel-{n}.skrib").into(),
                    taken_at: first + chrono::Duration::days(n),
                    source: SourceKind::Backup,
                })
                .collect(),
            lists: std::cell::Cell::new(0),
            indexes: std::cell::Cell::new(0),
        }
    }
}

impl VersionSource for BookInEveryBackup {
    fn list(&self) -> anyhow::Result<Vec<skrib_format::versions::VersionRef>> {
        self.lists.set(self.lists.get() + 1);
        Ok(self.versions.iter().rev().cloned().collect())
    }

    fn index(
        &self,
        v: &skrib_format::versions::VersionRef,
    ) -> anyhow::Result<skrib_format::versions::VersionIndex> {
        use common::entities::{BinderItemRole, BinderItemSubRole};
        self.indexes.set(self.indexes.get() + 1);
        Ok(skrib_format::versions::VersionIndex {
            taken_at: v.taken_at,
            rows: vec![skrib_format::versions::VersionRow {
                uid: self.uid,
                title: "The book".into(),
                sub_title: String::new(),
                role: BinderItemRole::Folder,
                sub_role: BinderItemSubRole::Book,
                indent: 0,
                is_exportable: true,
                prose: vec![(
                    ContentRole::SynopsisText,
                    "binders/01-manuscript/text/1-the-book.synopsis.djot".into(),
                    skrib_format::versions::BlobStamp { bytes: 12 },
                )],
            }],
        })
    }

    fn prose(
        &self,
        _v: &skrib_format::versions::VersionRef,
        _blob_path: &str,
    ) -> anyhow::Result<String> {
        Ok("A book about a lighthouse.".into())
    }

    fn comments(
        &self,
        _v: &skrib_format::versions::VersionRef,
        _blob_path: &str,
    ) -> anyhow::Result<Vec<skrib_format::CommentFile>> {
        Ok(Vec::new())
    }
}

/// A log with nothing in it.
struct NoLog;

impl VersionSource for NoLog {
    fn list(&self) -> anyhow::Result<Vec<skrib_format::versions::VersionRef>> {
        Ok(Vec::new())
    }

    fn index(
        &self,
        v: &skrib_format::versions::VersionRef,
    ) -> anyhow::Result<skrib_format::versions::VersionIndex> {
        Ok(skrib_format::versions::VersionIndex {
            taken_at: v.taken_at,
            rows: Vec::new(),
        })
    }

    fn prose(
        &self,
        _v: &skrib_format::versions::VersionRef,
        _blob_path: &str,
    ) -> anyhow::Result<String> {
        anyhow::bail!("the log holds nothing")
    }

    fn comments(
        &self,
        _v: &skrib_format::versions::VersionRef,
        _blob_path: &str,
    ) -> anyhow::Result<Vec<skrib_format::CommentFile>> {
        Ok(Vec::new())
    }
}

/// The body scope tries four roles in turn, and a Book has none of them. Each
/// try used to list the destinations and read every backup again, so the dock
/// read every backup four times over before it could say the Book has no text
/// on record.
#[test]
fn the_body_scope_reads_each_backup_once_for_a_row_with_no_body() {
    let uid = uuid::Uuid::from_u128(0xB00C);
    let backups = BookInEveryBackup::new(uid, 3);
    let roles = VersionScope::Prose.roles();
    assert_eq!(roles.len(), 4, "precondition: the scope tries four roles");

    let view = collect_from(&NoLog, &backups, uid, roles).expect("a readable past");

    assert!(view.timeline.changes.is_empty(), "a Book has no body text");
    assert!(view.timeline.unreadable.is_empty(), "every backup was read");
    assert_eq!(backups.lists.get(), 1, "the destinations are listed once");
    assert_eq!(
        backups.indexes.get(),
        3,
        "each of the three backups is read once, not once per role",
    );
}
