// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! End-to-end: `record_progress_snapshot` upserts the daily row on the open WorkInfo.

use std::sync::Arc;

use chrono::{DateTime, Utc};
use common::database::db_context::DbContext;
use common::event::EventHub;
use direct_access::ProgressSnapshotDto;
use direct_access::progress_snapshot::progress_snapshot_controller;
use direct_access::work::work_controller;
use progress_management::RecordProgressSnapshotDto;
use progress_management::progress_management_controller::record_progress_snapshot;
use work_management::work_management_controller;
use work_management::{NewWorkDto, NewWorkTemplate};

fn day(s: &str) -> DateTime<Utc> {
    DateTime::parse_from_rfc3339(s).unwrap().with_timezone(&Utc)
}

fn snapshots(db: &DbContext) -> Vec<ProgressSnapshotDto> {
    progress_snapshot_controller::get_all(db).unwrap()
}

fn rec(work_id: u64, d: DateTime<Utc>, words: i64) -> RecordProgressSnapshotDto {
    RecordProgressSnapshotDto {
        work_id,
        day: d,
        total_word_count: words,
        total_char_count: words * 5,
        book_item_ids: vec![],
        book_word_counts: vec![],
    }
}

/// A fresh project, which creates the WorkInfo the snapshots hang off.
fn new_project(dir: &std::path::Path, db: &DbContext, hub: &Arc<EventHub>) -> u64 {
    work_management_controller::new_work(
        db,
        hub,
        &NewWorkDto {
            goal_unit: Default::default(),
            file_name: dir.join("Novel.skrib").to_str().unwrap().to_string(),
            title: String::new(),
            is_folder: false,
            template_kind: NewWorkTemplate::None,
            labels: vec![],
            language: vec!["en-US".to_string()],
            author_name: String::new(),
            chapter_scene_mode: false,
            paratext_front: Vec::new(),
            paratext_back: Vec::new(),
        },
    )
    .expect("new_work");
    work_controller::get_all(db).unwrap().pop().unwrap().id
}

#[test]
fn record_progress_snapshot_upserts_by_day() {
    let dir = tempfile::tempdir().unwrap();
    let db = DbContext::new().unwrap();
    let hub = Arc::new(EventHub::new());
    let work_id = new_project(dir.path(), &db, &hub);

    record_progress_snapshot(
        &db,
        &hub,
        &rec(work_id, day("2020-05-01T10:00:00+00:00"), 100),
    )
    .unwrap();
    assert_eq!(snapshots(&db).len(), 1, "first day recorded");

    // Same day, later time, different total → replaced in place (idempotent by day).
    record_progress_snapshot(
        &db,
        &hub,
        &rec(work_id, day("2020-05-01T22:00:00+00:00"), 250),
    )
    .unwrap();
    let s = snapshots(&db);
    assert_eq!(s.len(), 1, "the same day must not pile up rows");
    assert_eq!(
        s[0].total_word_count, 250,
        "the row was updated to the new total"
    );
    assert_eq!(
        s[0].day,
        day("2020-05-01T00:00:00+00:00"),
        "keyed at midnight UTC"
    );

    // A new day → a second row.
    record_progress_snapshot(
        &db,
        &hub,
        &rec(work_id, day("2020-05-02T09:00:00+00:00"), 400),
    )
    .unwrap();
    assert_eq!(snapshots(&db).len(), 2, "a new day adds a row");
}

/// A row filed under a day after today holds a total counted before today's.
///
/// Today is the writer's, so a row can sit under a later day without being from
/// the future: a Los Angeles evening was filed under the next day while
/// snapshots still followed UTC's calendar, and a writer who flies west goes
/// back to a day they already had. Left in place, that row stood as the newest
/// day of the history with an older total, and the Pace planner read it as the
/// current count until the writer's calendar caught up.
#[test]
fn a_row_filed_under_a_later_day_gives_way_to_todays_total() {
    let dir = tempfile::tempdir().unwrap();
    let db = DbContext::new().unwrap();
    let hub = Arc::new(EventHub::new());
    let work_id = new_project(dir.path(), &db, &hub);

    // The day before, and a wrong clock's row far ahead: neither was counted on
    // another zone's calendar, so both stay.
    record_progress_snapshot(
        &db,
        &hub,
        &rec(work_id, day("2026-09-26T12:00:00+00:00"), 3000),
    )
    .unwrap();
    record_progress_snapshot(
        &db,
        &hub,
        &rec(work_id, day("2026-10-15T12:00:00+00:00"), 100),
    )
    .unwrap();
    // 18:00 on the 27th in Los Angeles, filed under UTC's day: the 28th.
    record_progress_snapshot(
        &db,
        &hub,
        &rec(work_id, day("2026-09-28T01:00:00+00:00"), 4000),
    )
    .unwrap();

    // 19:00 the same evening, filed under the writer's day: the 27th.
    record_progress_snapshot(
        &db,
        &hub,
        &rec(work_id, day("2026-09-27T00:00:00+00:00"), 4500),
    )
    .unwrap();

    let mut rows: Vec<(DateTime<Utc>, i64)> = snapshots(&db)
        .into_iter()
        .map(|s| (s.day, s.total_word_count))
        .collect();
    rows.sort();
    assert_eq!(
        rows,
        vec![
            (day("2026-09-26T00:00:00+00:00"), 3000),
            (day("2026-09-27T00:00:00+00:00"), 4500),
            (day("2026-10-15T00:00:00+00:00"), 100),
        ],
        "the evening's older total, filed under the 28th, is superseded by the one \
         counted after it; the days either side are kept",
    );
    let work_info = direct_access::work_info::work_info_controller::get_all(&db)
        .unwrap()
        .pop()
        .unwrap();
    assert_eq!(
        work_info.progress_snapshots.len(),
        3,
        "the WorkInfo lists exactly the rows that are left",
    );
}
