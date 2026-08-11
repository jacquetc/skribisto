// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

use super::*;
use tempfile::tempdir;

fn svc(dir: &std::path::Path) -> WorkspaceLayoutService {
    WorkspaceLayoutService::open_at(dir.join("workspace.toml"), Duration::ZERO).unwrap()
}

fn u(n: u128) -> Uuid {
    Uuid::from_u128(n)
}

fn sample(uid: &str) -> PerProjectLayout {
    PerProjectLayout {
        work_uid: uid.to_string(),
        // Distinct paths per uid — a real project is one file (dedup-by-path).
        last_path: format!("/x/{uid}.skrib"),
        primary: PaneLayout {
            tabs: vec![u(3), u(7), u(1)],
            selected: Some(u(7)),
            view_states: Vec::new(),
        },
        secondary: PaneLayout {
            tabs: vec![u(9)],
            selected: Some(u(9)),
            view_states: Vec::new(),
        },
        focus_secondary: false,
        editor_splitter: None,
        docks: Some(DockLayoutState::default()),
        known_docks: vec![1, 2, 3],
    }
}

/// A v1 file's ordinal tab lists are **dropped**, not translated — an ordinal only
/// means something against a loaded project's item stream, which a settings migration
/// does not have. Everything else in the row survives, so the user loses exactly one
/// launch's remembered tabs rather than reopening the wrong items forever.
#[test]
fn the_v1_migration_drops_ordinal_tabs_and_keeps_the_rest() {
    let d = tempdir().unwrap();
    let path = d.path().join("workspace.toml");
    std::fs::write(
        &path,
        r#"version = 1
[[projects]]
work_uid = "uid-A"
last_path = "/x/a.skrib"
focus_secondary = true
[projects.primary]
tabs = [3, 7, 1]
selected = 7
[projects.secondary]
tabs = [9]
selected = 9
"#,
    )
    .unwrap();

    let s = WorkspaceLayoutService::open_at(path, Duration::ZERO).unwrap();
    let got = s.get("uid-A").expect("the row survived the migration");
    assert!(
        got.primary.tabs.is_empty() && got.primary.selected.is_none(),
        "v1 ordinals cannot be translated, so they are dropped"
    );
    assert!(got.secondary.tabs.is_empty());
    assert_eq!(got.last_path, "/x/a.skrib", "the rest of the row is kept");
    assert!(got.focus_secondary, "including the focused pane");
}

/// **v3 → v4 loses nothing.** Unlike v1→v2 and v2→v3, which both had to drop
/// data they could not translate, `view_states` is a brand-new field: an
/// existing file loads with every tab, selection, splitter and dock intact
/// and simply no remembered caret positions yet. The version bump exists so
/// an *older* build meeting a v4 file is refused rather than silently
/// rewriting it.
#[test]
fn the_v4_migration_is_additive_and_keeps_every_tab() {
    let d = tempdir().unwrap();
    let path = d.path().join("workspace.toml");
    std::fs::write(
        &path,
        r#"version = 3
[[projects]]
work_uid = "uid-A"
last_path = "/x/a.skrib"
focus_secondary = true
[projects.primary]
tabs = ["00000000-0000-0000-0000-000000000003"]
selected = "00000000-0000-0000-0000-000000000003"
[projects.secondary]
tabs = []
"#,
    )
    .unwrap();

    let s = WorkspaceLayoutService::open_at(path, Duration::ZERO).unwrap();
    let got = s.get("uid-A").expect("the row survived the migration");
    assert_eq!(got.primary.tabs.len(), 1, "a v3 file keeps its tabs");
    assert!(got.primary.selected.is_some());
    assert_eq!(got.last_path, "/x/a.skrib");
    assert!(got.focus_secondary);
    assert!(
        got.primary.view_states.is_empty(),
        "nothing remembered yet, but the field must exist rather than fail the load"
    );
}

/// **v5 → v6 is additive and keeps every tab and caret.** Same shape as v4:
/// `TabViewState::corkboard` is a brand-new field with a serde default, so a v5
/// document loads whole and merely gains an empty board state. The step exists
/// so an *older* build meeting a v6 file is refused rather than silently
/// rewriting it and dropping every board's remembered navigation.
#[test]
fn the_v6_migration_is_additive_and_keeps_every_tab_and_caret() {
    let d = tempdir().unwrap();
    let path = d.path().join("workspace.toml");
    std::fs::write(
        &path,
        r#"version = 5
[[projects]]
work_uid = "uid-A"
last_path = "/x/a.skrib"
known_docks = [13631489]
[projects.primary]
tabs = ["00000000-0000-0000-0000-000000000003"]
selected = "00000000-0000-0000-0000-000000000003"
[[projects.primary.view_states]]
uid = "00000000-0000-0000-0000-000000000003"
caret = 412
scroll = 96.5
[projects.secondary]
tabs = []
"#,
    )
    .unwrap();

    let s = WorkspaceLayoutService::open_at(path, Duration::ZERO).unwrap();
    let got = s.get("uid-A").expect("the row survived the migration");
    assert_eq!(got.primary.tabs.len(), 1, "a v5 file keeps its tabs");
    assert_eq!(got.last_path, "/x/a.skrib");
    assert_eq!(got.known_docks, vec![13631489], "and its dock roster");
    let vs = got.primary.view_states.first().expect("the caret survived");
    assert_eq!(vs.caret, 412);
    assert_eq!(vs.scroll, 96.5);
    assert!(
        vs.corkboard.is_empty(),
        "no board navigation remembered yet, but the field must exist rather \
             than fail the load"
    );
}

/// A drilled board's trail and filter survive a real write and re-read — the
/// half of the round trip the in-memory view-model test cannot cover.
#[test]
fn corkboard_board_state_round_trips_through_disk() {
    let d = tempdir().unwrap();
    let mut rec = sample("uid-A");
    rec.primary.view_states = vec![TabViewState {
        uid: u(3),
        caret: 0,
        scroll: 0.0,
        corkboard: CorkboardTabState {
            trail: vec![u(1), u(2), u(3)],
            query: "ferry".into(),
        },
    }];
    {
        let s = svc(d.path());
        s.set(rec).unwrap();
    }
    let s = svc(d.path());
    let got = s.get("uid-A").expect("row");
    let board = &got.primary.view_states[0].corkboard;
    assert_eq!(board.trail, vec![u(1), u(2), u(3)]);
    assert_eq!(board.query, "ferry");
}

/// **v4 → v5 stamps the roster and destroys nothing.** This is the migration that
/// let the comments docks reach existing projects *without* repeating v2 → v3's
/// blanket "drop every saved arrangement": the row keeps its docks, tabs, splitter
/// and focused pane, and merely gains the list of docks its author knew.
#[test]
fn the_v5_migration_stamps_the_v4_roster_and_keeps_the_saved_docks() {
    let d = tempdir().unwrap();
    let path = d.path().join("workspace.toml");
    // Build a genuine v4 document: a real serialized `DockLayoutState` (a
    // hand-written partial one is silently dropped by `lenient_docks`, which
    // would make this test pass for the wrong reason), stamped back to version 4
    // with the v5-only key removed.
    let mut f = WorkspaceLayoutFile {
        version: 4,
        projects: vec![PerProjectLayout {
            focus_secondary: true,
            ..sample("uid-A")
        }],
    };
    f.projects[0].known_docks.clear();
    let text = toml::to_string(&f)
        .unwrap()
        .replace("known_docks = []\n", "");
    assert!(
        !text.contains("known_docks"),
        "the fixture must be a real v4 file — no v5 key"
    );
    std::fs::write(&path, text).unwrap();

    let s = WorkspaceLayoutService::open_at(path, Duration::ZERO).unwrap();
    let got = s.get("uid-A").expect("the row survived the migration");
    assert_eq!(
        got.known_docks,
        vec![
            0xD0C_0001, 0xD0C_0002, 0xD0C_0003, 0xD0C_0004, 0xD0C_0005, 0xD0C_0006
        ],
        "stamped with the six docks a v4-era build knew — comments (7, 8) deliberately absent"
    );
    assert!(
        got.docks.is_some(),
        "unlike v2 -> v3, the saved arrangement is KEPT"
    );
    assert_eq!(
        got.primary.tabs,
        vec![u(3), u(7), u(1)],
        "and so are the tabs, in order"
    );
    assert_eq!(got.secondary.tabs, vec![u(9)]);
    assert!(got.focus_secondary, "and the focused pane");
}

/// A row with **no** `docks` blob is not stamped. It restores from the pristine
/// defaults, which already carry the whole current roster, so claiming it knew
/// only the v4 six would be false — and would then suppress a reconcile for a row
/// that never needed one.
#[test]
fn the_v5_migration_does_not_stamp_a_row_that_saved_no_docks() {
    let d = tempdir().unwrap();
    let path = d.path().join("workspace.toml");
    std::fs::write(
        &path,
        "version = 4\n\n[[projects]]\nwork_uid = \"uid-A\"\n[projects.primary]\n\
             tabs = [\"00000000-0000-0000-0000-000000000001\"]\n",
    )
    .unwrap();
    let s = WorkspaceLayoutService::open_at(path, Duration::ZERO).unwrap();
    let got = s.get("uid-A").unwrap();
    assert!(got.docks.is_none());
    assert!(
        got.known_docks.is_empty(),
        "no snapshot to describe, so no claim about what its author knew"
    );
}

/// A caret + scroll pair survives a real write and re-read.
#[test]
fn view_states_round_trip_through_disk() {
    let d = tempdir().unwrap();
    let mut rec = sample("uid-A");
    rec.primary.view_states = vec![TabViewState {
        uid: u(3),
        caret: 412,
        scroll: 96.5,
        corkboard: CorkboardTabState {
            trail: vec![u(9), u(10)],
            query: "keep".into(),
        },
    }];
    {
        let s = svc(d.path());
        s.set(rec).unwrap();
    }
    let s = svc(d.path());
    let got = s.get("uid-A").expect("row");
    assert_eq!(
        got.primary.view_states,
        vec![TabViewState {
            uid: u(3),
            caret: 412,
            scroll: 96.5,
            corkboard: CorkboardTabState {
                trail: vec![u(9), u(10)],
                query: "keep".into(),
            },
        }]
    );
}

#[test]
fn round_trips_through_toml() {
    let d = tempdir().unwrap();
    {
        let s = svc(d.path());
        s.set(sample("uid-A")).unwrap();
        s.flush_now().unwrap();
    }
    // Reopen from disk — the layout survived the TOML round-trip.
    let s = svc(d.path());
    let got = s.get("uid-A").expect("entry present");
    assert_eq!(got.primary.tabs, vec![u(3), u(7), u(1)]);
    assert_eq!(got.primary.selected, Some(u(7)));
    assert_eq!(got.secondary.tabs, vec![u(9)]);
    assert!(got.docks.is_some());
}

#[test]
fn dedup_by_path_keeps_one_row_per_file() {
    // A legacy uid-less .skrib mints a fresh uid every open, so the same path
    // arrives under a new uid each time — keep one row (the latest), not one
    // orphan per open.
    let d = tempdir().unwrap();
    let s = svc(d.path());
    let mut open1 = sample("uid-open-1");
    open1.last_path = "/x/legacy.skrib".to_string();
    let mut open2 = sample("uid-open-2");
    open2.last_path = "/x/legacy.skrib".to_string();
    s.set(open1).unwrap();
    s.set(open2).unwrap();
    assert_eq!(s.file.borrow().projects.len(), 1, "one row per file path");
    assert!(
        s.get("uid-open-1").is_none(),
        "the earlier open's orphan row is gone"
    );
    assert!(s.get("uid-open-2").is_some());
}

#[test]
fn no_op_write_is_skipped() {
    // Re-setting an identical desk must not grow or reorder the file.
    let d = tempdir().unwrap();
    let s = svc(d.path());
    s.set(sample("uid-A")).unwrap();
    s.set(sample("uid-A")).unwrap();
    s.set(sample("uid-A")).unwrap();
    assert_eq!(s.file.borrow().projects.len(), 1);
}

#[test]
fn row_count_is_capped() {
    let d = tempdir().unwrap();
    let s = svc(d.path());
    for i in 0..(MAX_PROJECTS + 20) {
        s.set(sample(&format!("uid-{i}"))).unwrap();
    }
    let f = s.file.borrow();
    assert_eq!(f.projects.len(), MAX_PROJECTS, "bounded at the cap");
    // Oldest evicted, newest kept.
    assert!(f.projects.iter().all(|p| p.work_uid != "uid-0"));
    assert!(
        f.projects
            .iter()
            .any(|p| p.work_uid == format!("uid-{}", MAX_PROJECTS + 19))
    );
}

#[test]
fn upsert_replaces_and_isolates_per_uid() {
    let d = tempdir().unwrap();
    let s = svc(d.path());
    s.set(sample("uid-A")).unwrap();
    s.set(sample("uid-B")).unwrap();

    // Overwrite A with a different desk.
    let mut a2 = sample("uid-A");
    a2.primary.tabs = vec![u(42)];
    a2.primary.selected = Some(u(42));
    s.set(a2).unwrap();

    assert_eq!(s.get("uid-A").unwrap().primary.tabs, vec![u(42)]);
    assert_eq!(
        s.get("uid-B").unwrap().primary.tabs,
        vec![u(3), u(7), u(1)],
        "B untouched"
    );
    assert_eq!(s.file.borrow().projects.len(), 2, "no duplicate row for A");
}

#[test]
fn an_empty_uid_never_persists() {
    // A brand-new unsaved project has no unique_id; keying "" would collide
    // across unrelated projects, so a set on it is silently dropped.
    let d = tempdir().unwrap();
    let s = svc(d.path());
    s.set(sample("")).unwrap();
    assert!(s.get("").is_none(), "empty uid must not persist");
    assert!(s.file.borrow().projects.is_empty());
}

#[test]
fn forget_removes_only_the_named_entry() {
    let d = tempdir().unwrap();
    let s = svc(d.path());
    s.set(sample("uid-A")).unwrap();
    s.set(sample("uid-B")).unwrap();
    s.forget("uid-A").unwrap();
    assert!(s.get("uid-A").is_none());
    assert!(s.get("uid-B").is_some());
}

#[test]
fn two_shared_services_over_one_file_do_not_clobber_each_others_projects() {
    // Two processes (one per project) sharing one workspace.toml, each writing
    // a different project's layout. The locked read-modify-write keeps the
    // second write's stale snapshot from dropping the first.
    let d = tempdir().unwrap();
    let path = d.path().join("workspace.toml");
    let a = WorkspaceLayoutService::open_at(path.clone(), Duration::ZERO).unwrap();
    let b = WorkspaceLayoutService::open_at(path.clone(), Duration::ZERO).unwrap();
    a.set(sample("uid-A")).unwrap();
    b.set(sample("uid-B")).unwrap();

    let c = WorkspaceLayoutService::open_at(path, Duration::ZERO).unwrap();
    assert!(c.get("uid-A").is_some());
    assert!(c.get("uid-B").is_some());
}

#[test]
fn an_unreadable_docks_blob_drops_to_none_without_failing_the_file() {
    // A project row whose embedded `docks` is a garbage/future-incompatible
    // table must load with `docks: None` and its tabs intact — and must not
    // take down a sibling project's row (blast-radius containment).
    let d = tempdir().unwrap();
    let path = d.path().join("workspace.toml");
    std::fs::write(
        &path,
        r#"
version = 2

[[projects]]
work_uid = "uid-good"
[projects.primary]
tabs = ["00000000-0000-0000-0000-000000000001", "00000000-0000-0000-0000-000000000004"]
selected = "00000000-0000-0000-0000-000000000004"

[[projects]]
work_uid = "uid-bad-docks"
[projects.primary]
tabs = ["00000000-0000-0000-0000-000000000002"]
[projects.docks]
this_is_not = "a valid DockLayoutState"
leading = 12345
"#,
    )
    .unwrap();
    let s = WorkspaceLayoutService::open_at(path, Duration::ZERO).unwrap();
    // The good row is untouched.
    let good = s.get("uid-good").expect("good row present");
    assert_eq!(good.primary.tabs, vec![u(1), u(4)]);
    assert!(good.docks.is_none());
    // The bad-docks row still loads; only its docks dropped to None.
    let bad = s.get("uid-bad-docks").expect("bad-docks row still loaded");
    assert_eq!(
        bad.primary.tabs,
        vec![u(2)],
        "tabs survive an unreadable docks blob"
    );
    assert!(
        bad.docks.is_none(),
        "unreadable docks blob -> None, not a load failure"
    );
}

#[test]
fn missing_optional_fields_default_cleanly() {
    // A minimal legacy-shaped row (only work_uid + a couple of tabs) must load,
    // the rest defaulting — additive schema evolution safety.
    let d = tempdir().unwrap();
    let path = d.path().join("workspace.toml");
    std::fs::write(
        &path,
        "version = 2\n\n[[projects]]\nwork_uid = \"uid-A\"\n[projects.primary]\n\
             tabs = [\"00000000-0000-0000-0000-000000000001\"]\n",
    )
    .unwrap();
    let s = WorkspaceLayoutService::open_at(path, Duration::ZERO).unwrap();
    let got = s.get("uid-A").unwrap();
    assert_eq!(got.primary.tabs, vec![u(1)]);
    assert_eq!(got.primary.selected, None);
    assert!(!got.focus_secondary);
    assert!(got.docks.is_none());
    assert!(got.secondary.is_empty());
}
