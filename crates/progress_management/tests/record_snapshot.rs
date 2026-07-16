//! End-to-end: `record_progress_snapshot` upserts the daily row on the open WorkInfo.

use std::sync::Arc;

use chrono::{DateTime, Utc};
use common::database::db_context::DbContext;
use common::event::EventHub;
use direct_access::ProgressSnapshotDto;
use direct_access::progress_snapshot::progress_snapshot_controller;
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

fn rec(d: DateTime<Utc>, words: i64) -> RecordProgressSnapshotDto {
    RecordProgressSnapshotDto {
        day: d,
        total_word_count: words,
        total_char_count: words * 5,
        book_item_ids: vec![],
        book_word_counts: vec![],
    }
}

#[test]
fn record_progress_snapshot_upserts_by_day() {
    let dir = tempfile::tempdir().unwrap();
    let db = DbContext::new().unwrap();
    let hub = Arc::new(EventHub::new());

    // A fresh project creates the WorkInfo the snapshots hang off.
    work_management_controller::new_work(
        &db,
        &hub,
        &NewWorkDto {
            file_name: dir.path().join("Novel.skrib").to_str().unwrap().to_string(),
            is_folder: false,
            template_kind: NewWorkTemplate::None,
            labels: vec![],
            language: "en-US".into(),
            chapter_scene_mode: false,
        },
    )
    .expect("new_work");

    record_progress_snapshot(&db, &hub, &rec(day("2020-05-01T10:00:00+00:00"), 100)).unwrap();
    assert_eq!(snapshots(&db).len(), 1, "first day recorded");

    // Same day, later time, different total → replaced in place (idempotent by day).
    record_progress_snapshot(&db, &hub, &rec(day("2020-05-01T22:00:00+00:00"), 250)).unwrap();
    let s = snapshots(&db);
    assert_eq!(s.len(), 1, "the same day must not pile up rows");
    assert_eq!(s[0].total_word_count, 250, "the row was updated to the new total");
    assert_eq!(s[0].day, day("2020-05-01T00:00:00+00:00"), "keyed at midnight UTC");

    // A new day → a second row.
    record_progress_snapshot(&db, &hub, &rec(day("2020-05-02T09:00:00+00:00"), 400)).unwrap();
    assert_eq!(snapshots(&db).len(), 2, "a new day adds a row");
}
