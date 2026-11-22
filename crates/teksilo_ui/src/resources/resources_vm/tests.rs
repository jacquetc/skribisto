// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

use super::*;
use crate::models::ResourceKind;

fn row(id: u64, title: &str) -> ResourceRow {
    ResourceRow {
        id,
        uid: uuid::Uuid::from_u128(u128::from(id)),
        title: title.to_string(),
        ..Default::default()
    }
}

#[test]
fn the_large_file_setting_reads_its_three_answers() {
    assert_eq!(large_file_choice("ask"), None);
    assert_eq!(large_file_choice(""), None);
    assert_eq!(large_file_choice("store"), Some(LargeFileChoice::Store));
    assert_eq!(large_file_choice("link"), Some(LargeFileChoice::Link));
    assert_eq!(
        crate::RESOURCES_LARGE_FILE_DEFAULT,
        "ask",
        "a large file asks until the writer says otherwise"
    );
}

#[test]
fn linking_the_large_files_stores_the_small_ones() {
    let paths = vec![
        PathBuf::from("/a/small.txt"),
        PathBuf::from("/a/recording.wav"),
        PathBuf::from("/a/notes.md"),
    ];
    let large = vec![(PathBuf::from("/a/recording.wav"), 50 << 20)];
    let (store, link) = split_large(paths.clone(), &large, LargeFileChoice::Link);
    assert_eq!(
        store,
        vec![PathBuf::from("/a/small.txt"), PathBuf::from("/a/notes.md")]
    );
    assert_eq!(link, vec![PathBuf::from("/a/recording.wav")]);
    let (store, link) = split_large(paths.clone(), &large, LargeFileChoice::Store);
    assert_eq!(store, paths);
    assert!(link.is_empty());
}

#[test]
fn rows_sort_by_title_as_a_reader_looks_them_up() {
    let rows = vec![
        row(1, "village map"),
        row(2, "Écluse"),
        row(3, "Anchor"),
        row(4, "eel traps"),
        row(5, "anchor"),
    ];
    let titles: Vec<String> = sorted_by_title(rows).into_iter().map(|r| r.title).collect();
    assert_eq!(
        titles,
        ["Anchor", "anchor", "Écluse", "eel traps", "village map"],
        "case and accents aside, ties by id"
    );
}

#[test]
fn only_a_plain_web_address_is_kept() {
    assert!(is_web_address("https://example.org/harbour"));
    assert!(is_web_address("HTTP://example.org"));
    for refused in [
        "ftp://example.org",
        "file:///etc/passwd",
        "https://user:pass@example.org/",
        "https://",
        "example.org",
        "mailto:a@b.org",
        "resource:12345678-9abc-def0-1234-56789abcdef0",
    ] {
        assert!(!is_web_address(refused), "{refused}");
    }
}

#[test]
fn a_dropped_text_yields_its_web_addresses_once_each() {
    let text = "# a uri-list comment\nhttps://example.org/a\r\n<https://example.org/b>\n\
                see https://example.org/a again, and file:///tmp/x";
    assert_eq!(
        web_addresses_in(text),
        ["https://example.org/a", "https://example.org/b"]
    );
    assert!(web_addresses_in("no address here").is_empty());
}

#[test]
fn a_project_says_no_to_the_folder_notice_for_itself_only() {
    assert!(!declined_for("", "uid-a"));
    let once = with_declined("", "uid-a");
    assert_eq!(once, "uid-a");
    assert!(declined_for(&once, "uid-a"));
    assert!(
        !declined_for(&once, "uid-b"),
        "another project is still asked"
    );
    let twice = with_declined(&once, "uid-b");
    assert_eq!(twice, "uid-a uid-b");
    assert_eq!(with_declined(&twice, "uid-a"), twice, "no duplicate");
    assert_eq!(
        with_declined(&twice, "  "),
        twice,
        "an unsaved project has no identity to remember"
    );
    assert!(!declined_for("uid-a", ""));
}

/// Locate brings along the other missing files of the same old folder found under the
/// same name in the new one, and only those.
#[test]
fn locating_one_file_relinks_its_missing_neighbours() {
    let new_dir = tempfile::tempdir().expect("tmp");
    std::fs::write(new_dir.path().join("Tide tables.pdf"), b"x").expect("write");
    std::fs::write(new_dir.path().join("Harbour plan.pdf"), b"x").expect("write");
    std::fs::write(new_dir.path().join("Present.pdf"), b"x").expect("write");
    let linked = |id: u64, location: &str, missing: bool| ResourceRow {
        id,
        kind: ResourceKind::LinkedFile,
        location: location.to_string(),
        missing,
        ..row(id, "")
    };
    let located = linked(1, "/old/Research/Tide tables.pdf", true);
    let rows = vec![
        located.clone(),
        // Same old folder, missing, there in the new one: relinked.
        linked(2, "/old/Research/Harbour plan.pdf", true),
        // Same old folder, missing, not in the new one: left.
        linked(3, "/old/Research/Letters.pdf", true),
        // Another folder: left.
        linked(4, "/elsewhere/Present.pdf", true),
        // Not missing: left, even though a file of that name is there.
        linked(5, "/old/Research/Present.pdf", false),
    ];
    let new_path = new_dir.path().join("Tide tables.pdf");
    assert_eq!(
        siblings_to_relink(&rows, &located, &new_path),
        vec![(2, new_dir.path().join("Harbour plan.pdf"))]
    );
}

/// A Save As that moved the project while its files were copied: the uid folders the
/// copy made follow the project to its new resources directory.
#[test]
fn stored_folders_move_to_where_the_project_now_keeps_them() {
    let dir = tempfile::tempdir().expect("tmp");
    let from = dir.path().join("before");
    let to = dir.path().join("after");
    let uid = "6f1c8a6e-0000-4000-8000-000000000001";
    std::fs::create_dir_all(from.join(uid)).expect("mkdir");
    std::fs::write(from.join(uid).join("map.pdf"), b"map").expect("write");
    let failed = move_stored_folders(&from, &to, &[uid.to_string(), "missing-uid".to_string()]);
    assert_eq!(failed, vec!["missing-uid".to_string()]);
    assert_eq!(
        std::fs::read(to.join(uid).join("map.pdf")).expect("moved"),
        b"map"
    );
    assert!(!from.join(uid).exists());
}

#[test]
fn a_picture_asks_and_nothing_else_does() {
    let picture = ResourceRow {
        file_name: "Lighthouse.jpg".into(),
        ..row(1, "Lighthouse")
    };
    assert!(ResourcesViewModel::offers_picture(&picture));
    assert!(!ResourcesViewModel::offers_picture(&ResourceRow {
        missing: true,
        ..picture.clone()
    }));
    assert!(!ResourcesViewModel::offers_picture(&ResourceRow {
        file_name: "Map.pdf".into(),
        ..row(2, "Map")
    }));
    assert!(!ResourcesViewModel::offers_picture(&ResourceRow {
        kind: ResourceKind::WebLink,
        location: "https://example.org/a.jpg".into(),
        ..row(3, "Page")
    }));
}

#[test]
fn a_resource_links_by_uid_and_a_web_link_by_its_address() {
    let file = row(7, "Village map");
    assert_eq!(
        link_of(&file),
        ("Village map".to_string(), links::resource_href(file.uid))
    );
    let web = ResourceRow {
        kind: ResourceKind::WebLink,
        location: "https://example.org/tides".into(),
        ..row(8, "")
    };
    assert_eq!(
        link_of(&web),
        (
            "https://example.org/tides".to_string(),
            "https://example.org/tides".to_string()
        ),
        "with no title, the words are the address"
    );
}

// ── Against the real backend ─────────────────────────────────────────────────

#[cfg(not(feature = "mocks"))]
mod real {
    use super::*;
    use crate::resources::opened_files::{OpenedWhere, write_open_copy};
    use crate::test_support::RealProject;

    /// Store `files` into the project's resources directory, as the dock does, and
    /// attach them to `item`. Returns the new rows' ids.
    fn store_and_add(
        app_ctx: &AppContext,
        session: &ResourcesSession,
        item: u64,
        files: &[PathBuf],
    ) -> Vec<u64> {
        let model = session.model();
        let dest = model.resources_dir();
        let StoreStart::Running(op) = model.start_store(&dest, files).expect("the copy starts")
        else {
            panic!("the real model copies through a long operation");
        };
        // Take the completion signal and release the manager before waiting on it.
        let completion = app_ctx
            .long_operation_manager
            .lock()
            .expect("manager")
            .completion_signal();
        assert!(completion.wait_for(&op, Some(Duration::from_secs(60))));
        let stored = model.store_result(&op).expect("a result");
        assert!(
            stored.failed_paths.is_empty(),
            "{:?}",
            stored.failure_reasons
        );
        model
            .add(&[item], &stored, &[], &[])
            .expect("the rows are made")
    }

    struct Fixture {
        project: RealProject,
        session: ResourcesSession,
        scene: u64,
        _data: tempfile::TempDir,
        data: PathBuf,
    }

    fn fixture() -> Fixture {
        let project = RealProject::empty_novel();
        let session = ResourcesSession::new(project.app_ctx.clone(), project.ids.clone());
        let data = tempfile::tempdir().expect("a data folder");
        session.model().set_media_root(data.path().join("media"));
        let scene = project.scenes()[0].0;
        Fixture {
            data: data.path().to_path_buf(),
            _data: data,
            project,
            session,
            scene,
        }
    }

    #[test]
    fn a_stored_file_becomes_a_row_with_its_file_in_the_resources_directory() {
        let fx = fixture();
        let src = fx.data.join("Village map.pdf");
        std::fs::write(&src, b"%PDF-1.7 map").expect("write");
        let ids = store_and_add(&fx.project.app_ctx, &fx.session, fx.scene, &[src]);
        assert_eq!(ids.len(), 1);
        let rows = fx.session.model().rows_for(fx.scene);
        assert_eq!(rows.len(), 1);
        let row = &rows[0];
        assert_eq!(row.title, "Village map");
        assert!(row.is_stored());
        assert!(!row.missing, "the file is where the row says");
        let path = row.path.clone().expect("a stored file has a path");
        assert!(path.starts_with(fx.session.model().resources_dir()));
        assert_eq!(std::fs::read(path).expect("read"), b"%PDF-1.7 map");
        assert_eq!(
            fx.session.model().by_uid(row.uid).map(|r| r.id),
            Some(row.id)
        );
    }

    #[test]
    fn a_missing_file_is_shown_missing_and_relinking_finds_it() {
        let fx = fixture();
        let gone = fx.data.join("old").join("Tide tables.pdf");
        let ids = fx
            .session
            .model()
            .add(
                &[fx.scene],
                &StoredResourceFilesDto::default(),
                std::slice::from_ref(&gone),
                &[],
            )
            .expect("linked");
        let row = fx.session.model().by_id(ids[0]).expect("the row");
        assert!(row.is_linked() && row.missing);
        let found = fx.data.join("Tide tables.pdf");
        std::fs::write(&found, b"tides").expect("write");
        fx.session.model().relink(row.id, &found).expect("relinked");
        let row = fx.session.model().by_id(row.id).expect("the row");
        assert!(!row.missing);
        assert_eq!(row.location, found.to_string_lossy());
    }

    #[test]
    fn detaching_leaves_the_resource_in_the_project_and_removing_takes_it_out() {
        let fx = fixture();
        let ids = fx
            .session
            .model()
            .add(
                &[fx.scene],
                &StoredResourceFilesDto::default(),
                &[],
                &[WebLink {
                    url: "https://example.org/harbour".into(),
                    title: "Harbour records".into(),
                }],
            )
            .expect("added");
        let model = fx.session.model();
        assert_eq!(model.rows_for(fx.scene).len(), 1);
        model.detach(fx.scene, ids[0]).expect("detached");
        assert!(model.rows_for(fx.scene).is_empty());
        assert_eq!(model.all().len(), 1, "still the project's");
        model.remove(&ids).expect("removed");
        assert!(model.all().is_empty());
    }

    /// A resource dropped on another item's text or row is attached to it, once: a
    /// second drop of the same resource adds nothing and queues no second notice, and
    /// the notice names the item and carries the undo step it landed on.
    #[test]
    fn a_dropped_resource_is_attached_to_the_item_once_with_a_notice() {
        let fx = fixture();
        // The scene, and any other item of the binder (the empty novel's chapter).
        let first = fx.scene;
        let second = crate::models::binder_stream::ordered_binder_items(
            &fx.project.app_ctx,
            fx.project.work_id,
        )
        .into_iter()
        .map(|row| row.id)
        .find(|id| *id != first)
        .expect("a second item");
        let ids = fx
            .session
            .model()
            .add(
                &[first],
                &StoredResourceFilesDto::default(),
                &[],
                &[WebLink {
                    url: "https://example.org/harbour".into(),
                    title: "Harbour records".into(),
                }],
            )
            .expect("added");
        let vm = ResourcesViewModel::detached(fx.project.app_ctx.clone(), fx.project.ids.clone());
        let notice = vm
            .attach_quietly(second, &ids)
            .expect("attached")
            .expect("a notice");
        assert_eq!(notice.count, 1);
        assert_eq!(notice.title, "Harbour records");
        assert!(notice.seq.is_some(), "an undo step to offer");
        let on_second: Vec<u64> = fx
            .session
            .model()
            .rows_for(second)
            .iter()
            .map(|r| r.id)
            .collect();
        assert_eq!(on_second, ids);
        // Again: it is already there.
        assert_eq!(vm.attach_quietly(second, &ids).expect("no error"), None);
        // The first item keeps it.
        assert_eq!(fx.session.model().rows_for(first).len(), 1);
    }

    /// Option A for a single-file project: the copy another application edited
    /// replaces the stored file, the row follows, and Undo puts both back.
    #[test]
    fn an_edited_copy_replaces_the_stored_file_and_undo_puts_it_back() {
        let fx = fixture();
        let src = fx.data.join("Village map.pdf");
        std::fs::write(&src, b"map, first draft").expect("write");
        store_and_add(&fx.project.app_ctx, &fx.session, fx.scene, &[src]);
        let before = fx.session.model().rows_for(fx.scene)[0].clone();
        let stored = before.path.clone().expect("path");

        let copy = write_open_copy(&stored, &fx.data, before.uid, &before.file_name).expect("copy");
        fx.session
            .opened()
            .remember(before.uid, &copy, OpenedWhere::Copy, &before.content_hash);
        std::fs::write(&copy, b"map, annotated in another app").expect("edit");
        let later = std::time::SystemTime::now() + Duration::from_secs(5);
        std::fs::File::options()
            .write(true)
            .open(&copy)
            .and_then(|f| f.set_modified(later))
            .expect("touch");

        fx.session.pickup_changes();
        assert_eq!(
            std::fs::read(&stored).expect("read"),
            b"map, annotated in another app"
        );
        let after = fx.session.model().by_id(before.id).expect("row");
        assert_ne!(after.content_hash, before.content_hash);
        assert_eq!(after.byte_size, 29);
        let notices = fx.session.take_notices();
        assert_eq!(notices.len(), 1);
        assert_eq!(notices[0].file_name, "Village map.pdf");
        let undo = notices[0]
            .undo
            .clone()
            .expect("a copy's change can be undone");

        fx.session.undo_change(before.id, &undo).expect("undone");
        assert_eq!(std::fs::read(&stored).expect("read"), b"map, first draft");
        let undone = fx.session.model().by_id(before.id).expect("row");
        assert_eq!(undone.content_hash, before.content_hash);
        assert_eq!(undone.byte_size, before.byte_size);

        fx.session.pickup_changes();
        assert!(
            fx.session.take_notices().is_empty(),
            "the change was reported once"
        );
    }

    /// A folder project's file edited in place is the project's already: only the row
    /// moves, and there is nothing to undo.
    #[test]
    fn a_file_edited_in_place_updates_its_row_with_no_undo() {
        let fx = fixture();
        let src = fx.data.join("Notes.txt");
        std::fs::write(&src, b"one").expect("write");
        store_and_add(&fx.project.app_ctx, &fx.session, fx.scene, &[src]);
        let before = fx.session.model().rows_for(fx.scene)[0].clone();
        let stored = before.path.clone().expect("path");
        fx.session.opened().remember(
            before.uid,
            &stored,
            OpenedWhere::InPlace,
            &before.content_hash,
        );
        std::fs::write(&stored, b"one and two").expect("edit");
        let later = std::time::SystemTime::now() + Duration::from_secs(5);
        std::fs::File::options()
            .write(true)
            .open(&stored)
            .and_then(|f| f.set_modified(later))
            .expect("touch");

        fx.session.pickup_changes();
        let after = fx.session.model().by_id(before.id).expect("row");
        assert_eq!(after.byte_size, 11);
        let notices = fx.session.take_notices();
        assert_eq!(notices.len(), 1);
        assert!(notices[0].undo.is_none());
    }

    /// The edit picked up reaches the save that follows: the hook runs before the
    /// save is asked for.
    #[test]
    fn the_pickup_runs_before_each_save_is_asked_for() {
        let project = RealProject::empty_novel();
        let save_state =
            crate::save::SaveStateViewModel::new(project.app_ctx.clone(), project.ids.clone());
        let ran = Rc::new(std::cell::Cell::new(0));
        {
            let ran = ran.clone();
            save_state.on_before_save(Rc::new(move || ran.set(ran.get() + 1)));
        }
        save_state.request_save();
        assert_eq!(ran.get(), 1);
    }
}

#[test]
fn a_linked_file_is_judged_by_the_file_it_opens_not_by_its_name() {
    // A shared project can name a link "map.pdf" and point it at a launcher.
    let disguised = ResourceRow {
        kind: ResourceKind::LinkedFile,
        file_name: "map.pdf".into(),
        location: "/home/someone/.local/share/applications/evil.desktop".into(),
        path: Some(PathBuf::from(
            "/home/someone/.local/share/applications/evil.desktop",
        )),
        ..row(9, "Village map")
    };
    assert_eq!(name_to_judge(&disguised), "evil.desktop");
    assert_eq!(
        open_safety::verdict(&name_to_judge(&disguised)),
        OpenVerdict::Ask
    );
    // With no path at all, the row's own name is what there is.
    let bare = ResourceRow {
        file_name: "notes.txt".into(),
        ..row(10, "Notes")
    };
    assert_eq!(name_to_judge(&bare), "notes.txt");
}
