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

/// One folder spelled two ways is not moved onto itself. On a volume that ignores case a
/// rename between two spellings of one folder fails, and the copy that followed copied
/// the folder onto itself and deleted it; Linux renames it onto itself without error, so
/// here the spellings it has (a `.`, a link on the way) stand for that.
#[cfg(unix)]
#[test]
fn a_folder_spelled_two_ways_is_one_place_and_is_not_moved() {
    let dir = tempfile::tempdir().expect("tmp");
    let from = dir.path().join("resources");
    let uid = "6f1c8a6e-0000-4000-8000-000000000001";
    std::fs::create_dir_all(from.join(uid)).expect("mkdir");
    std::fs::write(from.join(uid).join("map.pdf"), b"map").expect("write");
    let alias = dir.path().join("alias");
    std::os::unix::fs::symlink(&from, &alias).expect("link");

    for to in [
        from.join("."),
        alias.clone(),
        dir.path().join("x").join("..").join("resources"),
    ] {
        if let Some(parent) = to.parent() {
            std::fs::create_dir_all(parent).expect("mkdir");
        }
        assert!(same_place(&from, &to), "{}", to.display());
        assert!(move_stored_folders(&from, &to, &[uid.to_string()]).is_empty());
        assert_eq!(
            std::fs::read(from.join(uid).join("map.pdf")).expect("still there"),
            b"map"
        );
    }
    assert!(!same_place(&from, &dir.path().join("elsewhere")));
    assert!(!same_place(&from, &dir.path().join("x")));
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

/// Insert as picture is judged by the file that would be read, and is not offered for
/// one past the size a picture may be: inserting it would refuse it.
#[test]
fn a_picture_is_offered_by_the_file_it_reads_and_within_the_picture_cap() {
    let linked = ResourceRow {
        kind: ResourceKind::LinkedFile,
        file_name: "lighthouse.png".into(),
        path: Some(PathBuf::from("/home/someone/lighthouse.png")),
        ..row(1, "Lighthouse")
    };
    assert!(ResourcesViewModel::offers_picture(&linked));
    assert!(!ResourcesViewModel::offers_picture(&ResourceRow {
        path: Some(PathBuf::from("/home/someone/notes.pdf")),
        ..linked.clone()
    }));
    let cap = document_ingest::pictures::MAX_PICTURE_BYTES;
    assert!(ResourcesViewModel::offers_picture(&ResourceRow {
        byte_size: cap,
        ..linked.clone()
    }));
    assert!(!ResourcesViewModel::offers_picture(&ResourceRow {
        byte_size: cap + 1,
        ..linked
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
    use crate::resources::opened_files::OpenedWhere;
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

    /// Set a file's modification time a little later, so a test does not depend on how
    /// fine the file system's clock is.
    fn touch_later(path: &Path) {
        let later = std::time::SystemTime::now() + Duration::from_secs(5);
        std::fs::File::options()
            .write(true)
            .open(path)
            .and_then(|f| f.set_modified(later))
            .expect("touch");
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

        let copy = fx.session.open_copy(&before, &fx.data).expect("copy");
        std::fs::write(&copy, b"map, annotated in another app").expect("edit");
        touch_later(&copy);

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
        assert_eq!(
            std::fs::read(&copy).expect("read"),
            b"map, first draft",
            "the copy the other application has open no longer shows what was undone"
        );

        fx.session.pickup_changes();
        assert!(
            fx.session.take_notices().is_empty(),
            "the change was reported once"
        );

        // The other application saves again: a new change, taken in like the first.
        std::fs::write(&copy, b"map, annotated a second time").expect("edit");
        touch_later(&copy);
        assert!(fx.session.pickup_changes());
        assert_eq!(fx.session.take_notices().len(), 1);
        assert_eq!(
            std::fs::read(&stored).expect("read"),
            b"map, annotated a second time"
        );
    }

    #[test]
    fn undo_preserves_a_newer_edit_before_and_after_pickup() {
        let fx = fixture();
        let src = fx.data.join("map.pdf");
        std::fs::write(&src, b"first").unwrap();
        store_and_add(&fx.project.app_ctx, &fx.session, fx.scene, &[src]);
        let row = fx.session.model().rows_for(fx.scene)[0].clone();
        let stored = row.path.as_ref().unwrap();
        let copy = fx.session.open_copy(&row, &fx.data).unwrap();
        std::fs::write(&copy, b"second").unwrap();
        assert!(fx.session.pickup_changes());
        let undo = fx.session.take_notices()[0].undo.clone().unwrap();

        std::fs::write(&copy, b"third edit").unwrap();
        assert!(fx.session.undo_change(row.id, &undo).is_err());
        assert_eq!(std::fs::read(stored).unwrap(), b"second");
        assert_eq!(std::fs::read(&copy).unwrap(), b"third edit");
        assert!(fx.session.pickup_changes());
        assert!(fx.session.undo_change(row.id, &undo).is_err());
        assert_eq!(std::fs::read(stored).unwrap(), b"third edit");
    }

    #[test]
    fn save_as_leaves_in_place_edits_with_the_original_project() {
        let fx = fixture();
        let src = fx.data.join("map.pdf");
        std::fs::write(&src, b"first").unwrap();
        store_and_add(&fx.project.app_ctx, &fx.session, fx.scene, &[src]);
        let row = fx.session.model().rows_for(fx.scene)[0].clone();
        let original = row.path.as_ref().unwrap();
        fx.session
            .opened()
            .remember(row.uid, original, OpenedWhere::InPlace, &row.content_hash);
        fx.session.model().set_media_root(fx.data.join("moved"));
        let current = fx.session.model().by_id(row.id).unwrap().path.unwrap();
        opened_files::replace_file(original, &current).unwrap();
        std::fs::write(original, b"edited after Save As").unwrap();
        assert!(!fx.session.pickup_changes());
        assert!(fx.session.opened().files().is_empty());
        assert_eq!(std::fs::read(current).unwrap(), b"first");
        assert_eq!(
            fx.session.model().by_id(row.id).unwrap().content_hash,
            row.content_hash
        );
    }

    /// Undo finds the stored file where it is when asked, not where it was at the
    /// pickup: a Save As in between moves the project's working folder.
    #[test]
    fn undo_puts_the_previous_version_where_the_stored_file_is_now() {
        let fx = fixture();
        let src = fx.data.join("Village map.pdf");
        std::fs::write(&src, b"map, first draft").expect("write");
        store_and_add(&fx.project.app_ctx, &fx.session, fx.scene, &[src]);
        let before = fx.session.model().rows_for(fx.scene)[0].clone();
        let old_dir = fx.session.model().resources_dir();
        let old_stored = before.path.clone().expect("path");
        let copy = fx.session.open_copy(&before, &fx.data).expect("copy");
        std::fs::write(&copy, b"map, annotated").expect("edit");
        touch_later(&copy);
        fx.session.pickup_changes();
        let undo = fx.session.take_notices()[0].undo.clone().expect("undo");

        // The working folder moves, its files with it, as a Save As does.
        fx.session.model().set_media_root(fx.data.join("moved"));
        let new_dir = fx.session.model().resources_dir();
        assert_ne!(new_dir, old_dir);
        let new_stored = fx
            .session
            .model()
            .by_id(before.id)
            .and_then(|r| r.path)
            .expect("path");
        crate::resources::opened_files::replace_file(&old_stored, &new_stored).expect("move");
        std::fs::remove_file(&old_stored).expect("move");

        fx.session.undo_change(before.id, &undo).expect("undone");
        assert_eq!(
            std::fs::read(&new_stored).expect("read"),
            b"map, first draft"
        );
        assert!(!old_stored.exists(), "nothing written where the file was");
    }

    /// A copy the close kept, because the writer saved it after the close's last look,
    /// is taken in at the next open of the same resource rather than overwritten.
    #[test]
    fn a_copy_kept_at_close_is_taken_in_at_the_next_open() {
        let fx = fixture();
        let src = fx.data.join("Village map.pdf");
        std::fs::write(&src, b"map, first draft").expect("write");
        store_and_add(&fx.project.app_ctx, &fx.session, fx.scene, &[src]);
        let row = fx.session.model().rows_for(fx.scene)[0].clone();
        let stored = row.path.clone().expect("path");
        let copy = fx.session.open_copy(&row, &fx.data).expect("copy");
        let key = fx.session.model().resources_dir();
        let key = key.file_name().expect("a key");
        assert!(
            copy.starts_with(fx.data.join("open").join(key)),
            "under the project's own folder: {}",
            copy.display()
        );
        fx.session.opened().close();
        assert!(!copy.exists(), "unchanged, so removed");

        // Saved in the other application after the close took its last look.
        let copy = fx.session.open_copy(&row, &fx.data).expect("copy");
        std::fs::write(&copy, b"map, annotated after the close").expect("edit");
        fx.session.opened().close();
        assert!(copy.exists(), "kept: the project does not hold it");

        let again = fx.session.open_copy(&row, &fx.data).expect("copy");
        assert_eq!(again, copy);
        assert_eq!(
            std::fs::read(&copy).expect("read"),
            b"map, annotated after the close",
            "not overwritten"
        );
        assert_eq!(
            std::fs::read(&stored).expect("read"),
            b"map, annotated after the close",
            "taken in"
        );
        let notices = fx.session.take_notices();
        assert_eq!(notices.len(), 1);
        assert!(notices[0].undo.is_some(), "with its Undo");
        assert!(!fx.session.pickup_changes(), "and settled");
    }

    /// A change that could not be taken in (the stored file could not be replaced) is
    /// not counted as seen: the close keeps its copy, and the next pickup takes it in.
    #[cfg(unix)]
    #[test]
    fn a_change_that_could_not_be_taken_in_is_retried_and_its_copy_kept() {
        use std::os::unix::fs::PermissionsExt;
        let fx = fixture();
        let src = fx.data.join("Village map.pdf");
        std::fs::write(&src, b"map, first draft").expect("write");
        store_and_add(&fx.project.app_ctx, &fx.session, fx.scene, &[src]);
        let row = fx.session.model().rows_for(fx.scene)[0].clone();
        let stored = row.path.clone().expect("path");
        let folder = stored.parent().expect("folder").to_path_buf();
        let copy = fx.session.open_copy(&row, &fx.data).expect("copy");
        std::fs::write(&copy, b"map, annotated").expect("edit");
        touch_later(&copy);

        let set_mode = |mode| {
            std::fs::set_permissions(&folder, std::fs::Permissions::from_mode(mode))
                .expect("chmod");
        };
        // The stored file's folder refuses the temporary file the replace goes through.
        set_mode(0o555);
        let refused = tempfile::tempfile_in(&folder).is_err();
        if !refused {
            // Running as root: nothing is refused, so there is no failure to test.
            set_mode(0o755);
            return;
        }
        assert!(!fx.session.pickup_changes(), "nothing could be taken in");
        assert!(fx.session.take_notices().is_empty());
        set_mode(0o755);

        assert!(fx.session.pickup_changes(), "tried again, and taken in now");
        assert_eq!(std::fs::read(&stored).expect("read"), b"map, annotated");
        assert_eq!(fx.session.take_notices().len(), 1);
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
        touch_later(&stored);

        fx.session.pickup_changes();
        let after = fx.session.model().by_id(before.id).expect("row");
        assert_eq!(after.byte_size, 11);
        let notices = fx.session.take_notices();
        assert_eq!(notices.len(), 1);
        assert!(notices[0].undo.is_none());
    }

    /// A clean project whose copy the writer edited, then closed without coming back
    /// to the window first: the close sees it unsaved, so it is saved rather than
    /// lost, and the copy goes once the project closes.
    #[test]
    fn a_copy_edited_before_a_close_makes_the_project_unsaved() {
        let fx = fixture();
        let src = fx.data.join("Village map.pdf");
        std::fs::write(&src, b"map, first draft").expect("write");
        store_and_add(&fx.project.app_ctx, &fx.session, fx.scene, &[src]);
        let row = fx.session.model().rows_for(fx.scene)[0].clone();
        let stored = row.path.clone().expect("path");
        let save_state = crate::save::SaveStateViewModel::new(
            fx.project.app_ctx.clone(),
            fx.project.ids.clone(),
        );
        {
            let session = fx.session.clone();
            save_state.on_outside_changes(Rc::new(move || session.pickup_changes()));
        }
        save_state.mark_clean();

        let copy = fx.session.open_copy(&row, &fx.data).expect("copy");
        assert!(
            !save_state.is_unsaved_on_leaving(),
            "opening a file changes nothing"
        );
        std::fs::write(&copy, b"map, annotated in another app").expect("edit");
        touch_later(&copy);

        assert!(save_state.is_unsaved_on_leaving());
        assert_eq!(
            std::fs::read(&stored).expect("read"),
            b"map, annotated in another app"
        );
        fx.session.opened().close();
        assert!(!copy.exists(), "the copy goes with the project");
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
            save_state.on_outside_changes(Rc::new(move || {
                ran.set(ran.get() + 1);
                false
            }));
        }
        save_state.request_save();
        assert_eq!(ran.get(), 1);
    }

    // ── Drops ────────────────────────────────────────────────────────────────

    /// A view-model for one window over the fixture's project, its dock following
    /// `focus`.
    fn window_vm(fx: &Fixture, focus: u64) -> ResourcesViewModel {
        ResourcesViewModel::new(
            fx.project.app_ctx.clone(),
            fx.project.ids.clone(),
            fx.session.clone(),
            Signal::new(Some(focus)),
            Signal::new(None),
            Signal::new(String::new()),
        )
    }

    /// An editor over `text`, standing for one text of a window.
    fn editor_over(text: &str) -> EditorHandle {
        let doc = teksilo::text_document::TextDocument::new();
        doc.set_plain_text(text).expect("text");
        teksilo::widgets::rich_text::RichTextEditor::editor(doc).handle()
    }

    /// A file past the large-file size, sparse so that it costs nothing on disk.
    fn large_file(dir: &Path, name: &str) -> PathBuf {
        let path = dir.join(name);
        std::fs::File::create(&path)
            .and_then(|f| f.set_len(sizes::LARGE_FILE_BYTES + 1))
            .expect("a large file");
        path
    }

    /// A tree with a toast host and a text backend, for a test that clicks a toast.
    fn toast_tree(
        fx: &Fixture,
    ) -> (
        teksilo::core::widget_tree::WidgetTree,
        teksilo::widgets::ToastRegistry,
    ) {
        let (tree, toasts) =
            crate::test_support::tree_with_toast_host_and_settings(&fx.project.app_ctx);
        let tree = tree.with_text_backend(Rc::new(RefCell::new(
            teksilo::canvas::MockTextBackend::new(),
        )));
        (tree, toasts)
    }

    /// Click the button labelled `label` in the toasts on screen.
    fn click_in_toast(tree: &mut teksilo::core::widget_tree::WidgetTree, label: &str) {
        let button = tree
            .find_by_label(label)
            .unwrap_or_else(|| panic!("no {label:?} on screen"));
        crate::test_support::click(tree, button);
        tree.layout(SizeProposal::exact(900.0, 600.0));
    }

    /// Files dropped on a text go to that text and its item, whichever editor has
    /// focus and whatever item the dock follows: in a Full Chapter each scene has its
    /// own editor, and the drop moves no focus.
    #[test]
    fn files_dropped_on_a_text_are_filed_on_its_item_and_linked_in_it() {
        let fx = fixture();
        let first = fx.scene;
        let (second, _) = fx.project.add_scene_after(first, "Seven");
        // The dock follows the first scene, and its editor is the one with focus.
        let vm = window_vm(&fx, first);
        let focused = editor_over("Scene three, where the caret is.");
        {
            let focused = focused.clone();
            vm.connect(
                Rc::new(move || Some(focused.clone())),
                Rc::new(PathBuf::new),
                Rc::new(|_| {}),
            );
        }
        // The file lands in the second scene's editor, which put its caret there.
        let dropped_on = editor_over("Scene seven. The file lands here.");
        dropped_on.select_range(13, 13);
        let recording = large_file(&fx.data, "Harbour recording.wav");
        let (mut tree, _toasts) = toast_tree(&fx);
        {
            let (vm, dropped_on) = (vm.clone(), dropped_on.clone());
            crate::test_support::press(&mut tree, move |c| {
                // Linked where it is, as the writer chose once and for all.
                large_file_signal(c).set("link".to_string());
                vm.link_dropped_files(c, &dropped_on, Some(second), vec![recording.clone()]);
            });
        }
        let model = fx.session.model();
        assert_eq!(model.rows_for(second).len(), 1, "filed on the text's item");
        assert!(model.rows_for(first).is_empty(), "not on the dock's item");
        let uid = model.rows_for(second)[0].uid;
        assert!(
            dropped_on.to_djot().contains(&links::resource_href(uid)),
            "linked in the editor it landed on: {}",
            dropped_on.to_djot()
        );
        assert!(
            !focused.to_djot().contains("resource:"),
            "nothing in the editor with focus"
        );
    }

    /// Drop a linked file on `handle`'s text at `at`, attached to the fixture's scene,
    /// as the drop's own pipeline does once the large-file question is answered.
    fn drop_linked(
        fx: &Fixture,
        tree: &mut teksilo::core::widget_tree::WidgetTree,
        handle: &EditorHandle,
        at: usize,
    ) {
        let path = fx.data.join("Tide tables.pdf");
        std::fs::write(&path, b"%PDF-1.7 tides").expect("write");
        let vm = window_vm(fx, fx.scene);
        let (handle, scene) = (handle.clone(), fx.scene);
        crate::test_support::press(tree, move |c| {
            vm.begin_store(
                c,
                Vec::new(),
                vec![path.clone()],
                vec![scene],
                AddTarget::Text {
                    handle: handle.clone(),
                    at,
                },
            );
        });
    }

    /// The drop toast's Undo leaves the resources when it cannot take the words back:
    /// the writer typed before them, so removing the rows would leave a link in the
    /// text pointing at nothing. It says so instead.
    #[test]
    fn the_drop_undo_keeps_the_resources_when_the_text_has_moved() {
        crate::test_support::with_real_messages(|| {
            let fx = fixture();
            let handle = editor_over("Night fell.");
            let (mut tree, _toasts) = toast_tree(&fx);
            drop_linked(&fx, &mut tree, &handle, 5);
            assert_eq!(fx.session.model().all().len(), 1);
            assert!(handle.to_djot().contains("resource:"));
            // The writer goes on typing, before the link.
            handle.select_range(0, 0);
            handle.insert_text("Then ");
            click_in_toast(&mut tree, &tr!(resources_undo()).resolve_now());
            assert_eq!(fx.session.model().all().len(), 1, "the resource stays");
            assert!(
                handle.to_djot().contains("resource:"),
                "and so does its link: {}",
                handle.to_djot()
            );
            tree.layout(SizeProposal::exact(900.0, 600.0));
            assert!(
                tree.find_by_label(&tr!(resources_dropped_undo_kept()).resolve_now())
                    .is_some(),
                "the writer is told why nothing was taken back"
            );
        });
    }

    /// The drop toast's Undo takes back the words and undoes the add itself, leaving
    /// no second step on the project's stack for Ctrl+Z to walk through.
    #[test]
    fn the_drop_undo_takes_back_the_words_and_undoes_the_add() {
        crate::test_support::with_real_messages(|| {
            let fx = fixture();
            let stack = fx.project.ids.stack_id.get().expect("an undo stack");
            let steps = || {
                frontend::commands::undo_redo_commands::get_stack_size(&fx.project.app_ctx, stack)
            };
            let before = steps();
            let handle = editor_over("Night fell.");
            let (mut tree, _toasts) = toast_tree(&fx);
            drop_linked(&fx, &mut tree, &handle, 5);
            assert_eq!(steps(), before + 1, "the add is one step");
            click_in_toast(&mut tree, &tr!(resources_undo()).resolve_now());
            assert!(fx.session.model().all().is_empty(), "the resource is gone");
            assert_eq!(
                handle.to_plain_text(),
                "Night fell.",
                "and so are its words"
            );
            assert_eq!(
                steps(),
                before,
                "the add was undone, not followed by a removal"
            );
        });
    }

    // ── The dock's Unattached view ───────────────────────────────────────────

    /// The dock lists the item's resources or those of no item, as the writer chooses,
    /// and whether its buttons can add follows: with no item open, only Unattached can.
    #[test]
    fn the_dock_lists_the_item_s_resources_or_those_of_no_item() {
        let fx = fixture();
        let model = fx.session.model();
        let on_scene = WebLink {
            url: "https://example.org/harbour".into(),
            title: "Harbour records".into(),
        };
        let loose = WebLink {
            url: "https://example.org/interview".into(),
            title: "Interview with the keeper".into(),
        };
        let none = StoredResourceFilesDto::default();
        model
            .add(&[fx.scene], &none, &[], &[on_scene])
            .expect("added");
        model
            .add(&[], &none, &[], &[loose])
            .expect("added on no item");
        model.reload();

        let vm = window_vm(&fx, fx.scene);
        let titles = |vm: &ResourcesViewModel| -> Vec<String> {
            vm.rows().into_iter().map(|r| r.title).collect()
        };
        assert_eq!(vm.view().get(), DockView::Item, "the item's, by default");
        assert_eq!(titles(&vm), ["Harbour records"]);
        vm.view().set(DockView::Unattached);
        assert_eq!(titles(&vm), ["Interview with the keeper"]);

        let closed = ResourcesViewModel::new(
            fx.project.app_ctx.clone(),
            fx.project.ids.clone(),
            fx.session.clone(),
            Signal::new(None),
            Signal::new(None),
            Signal::new(String::new()),
        );
        assert!(
            !closed.can_add().get(),
            "no item: nowhere to file a resource"
        );
        assert!(closed.rows().is_empty());
        closed.view().set(DockView::Unattached);
        assert!(closed.can_add().get(), "on no item needs no item open");
        assert_eq!(titles(&closed), ["Interview with the keeper"]);
    }

    /// What the writer adds in the Unattached view, a pasted address or a file, goes on
    /// no item; back on This item, it goes on the item again. With no item open, This
    /// item adds nothing.
    #[test]
    fn what_is_added_in_the_unattached_view_goes_on_no_item() {
        let fx = fixture();
        let model = fx.session.model();
        let vm = window_vm(&fx, fx.scene);
        vm.view().set(DockView::Unattached);
        let recording = large_file(&fx.data, "Keeper interview.wav");
        let (mut tree, _toasts) = toast_tree(&fx);
        let added = Rc::new(std::cell::Cell::new(false));
        {
            let (vm, added) = (vm.clone(), added.clone());
            crate::test_support::press(&mut tree, move |c| {
                added.set(vm.add_dropped_text(c, "see https://example.org/tides"));
                // Linked where it is, as the writer chose once and for all.
                large_file_signal(c).set("link".to_string());
                vm.add_paths(c, vec![recording.clone()], AddTarget::Dock);
            });
        }
        assert!(added.get());
        let mut loose: Vec<String> = model
            .unattached()
            .expect("read")
            .into_iter()
            .map(|r| r.display_name().to_string())
            .collect();
        loose.sort();
        assert_eq!(loose, ["Keeper interview.wav", "https://example.org/tides"]);
        assert!(model.rows_for(fx.scene).is_empty(), "nothing on the item");
        assert_eq!(vm.rows().len(), 2, "and the dock lists them");

        vm.view().set(DockView::Item);
        {
            let vm = vm.clone();
            crate::test_support::press(&mut tree, move |c| {
                vm.add_dropped_text(c, "https://example.org/harbour");
            });
        }
        assert_eq!(model.rows_for(fx.scene).len(), 1, "on the item again");

        let closed = ResourcesViewModel::new(
            fx.project.app_ctx.clone(),
            fx.project.ids.clone(),
            fx.session.clone(),
            Signal::new(None),
            Signal::new(None),
            Signal::new(String::new()),
        );
        let refused = Rc::new(std::cell::Cell::new(true));
        {
            let refused = refused.clone();
            crate::test_support::press(&mut tree, move |c| {
                refused.set(!closed.add_dropped_text(c, "https://example.org/lost"));
            });
        }
        assert!(
            refused.get(),
            "no item open and This item chosen: added nowhere"
        );
        assert_eq!(model.all().len(), 3);
    }

    /// A link to a resource of no item put into the text files it on the text's item,
    /// as dragging it there does, so it leaves the Unattached list.
    #[test]
    fn a_loose_resource_linked_from_the_text_is_filed_on_its_item() {
        let fx = fixture();
        let model = fx.session.model();
        let ids = model
            .add(
                &[],
                &StoredResourceFilesDto::default(),
                &[],
                &[WebLink {
                    url: "https://example.org/interview".into(),
                    title: "Interview with the keeper".into(),
                }],
            )
            .expect("added on no item");
        model.reload();
        let row = model.by_id(ids[0]).expect("the row");
        let vm = window_vm(&fx, fx.scene);
        vm.view().set(DockView::Unattached);
        let (mut tree, _toasts) = toast_tree(&fx);
        {
            let vm = vm.clone();
            crate::test_support::press(&mut tree, move |c| vm.file_on_focus(c, &row));
        }
        assert_eq!(
            model.rows_for(fx.scene).len(),
            1,
            "filed on the text's item"
        );
        assert!(vm.rows().is_empty(), "no longer unattached");
    }

    /// A resource dropped on a binder row in one window says so in that window, not in
    /// another window on the same project that handled the item's update first.
    #[test]
    fn a_binder_drop_is_said_in_the_window_it_was_made_in() {
        let fx = fixture();
        let (second, _) = fx.project.add_scene_after(fx.scene, "Two");
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
        // Two windows on one project: one session, a view-model each.
        let one = window_vm(&fx, fx.scene);
        let two = window_vm(&fx, fx.scene);
        assert!(two.attach_dropped_later(second, &ids), "attached");

        let toasts = teksilo::widgets::ToastRegistry::new(Default::default());
        let mut tree = crate::test_support::tree_with_toast_registry(&fx.project.app_ctx, &toasts);
        // Window one's handler runs first, as the older window's does.
        crate::test_support::press(&mut tree, move |c| one.raise_attach_notices(c));
        assert_eq!(toasts.live_count(), 0, "nothing in the other window");
        crate::test_support::press(&mut tree, move |c| two.raise_attach_notices(c));
        assert_eq!(
            toasts.live_count(),
            1,
            "the notice, in the window of the drop"
        );
    }

    /// A binder drop whose attach fails tells the tree, which refuses the drop rather
    /// than taking it as if it had worked, and queues no notice.
    #[test]
    fn a_binder_drop_that_cannot_attach_is_refused() {
        let fx = fixture();
        let vm = window_vm(&fx, fx.scene);
        assert!(
            !vm.attach_dropped_later(fx.scene, &[u64::MAX]),
            "no such resource to attach"
        );
        let toasts = teksilo::widgets::ToastRegistry::new(Default::default());
        let mut tree = crate::test_support::tree_with_toast_registry(&fx.project.app_ctx, &toasts);
        crate::test_support::press(&mut tree, move |c| vm.raise_attach_notices(c));
        assert_eq!(toasts.live_count(), 0);
    }

    /// Removing a stored resource removes the copy it was opened through, rather than
    /// leaving it in the data folder for the sweep, unless that copy holds an edit.
    #[test]
    fn removing_a_resource_removes_its_unedited_copy() {
        let fx = fixture();
        let a = fx.data.join("Village map.pdf");
        let b = fx.data.join("Tide tables.pdf");
        std::fs::write(&a, b"map").expect("write");
        std::fs::write(&b, b"tides").expect("write");
        store_and_add(&fx.project.app_ctx, &fx.session, fx.scene, &[a, b]);
        let rows = sorted_by_title(fx.session.model().rows_for(fx.scene));
        let (tides, map) = (rows[0].clone(), rows[1].clone());
        let map_copy = fx.session.open_copy(&map, &fx.data).expect("copy");
        let tides_copy = fx.session.open_copy(&tides, &fx.data).expect("copy");
        std::fs::write(&tides_copy, b"tides, annotated").expect("edit");

        let vm = window_vm(&fx, fx.scene);
        let mut tree = crate::test_support::tree_with_events(&fx.project.app_ctx);
        let removed = vec![map.clone(), tides.clone()];
        crate::test_support::press(&mut tree, move |c| vm.remove_now(c, &removed));
        assert!(fx.session.model().all().is_empty(), "both removed");
        assert!(
            !map_copy.exists(),
            "the unedited copy goes with its resource"
        );
        assert_eq!(
            std::fs::read(&tides_copy).expect("kept"),
            b"tides, annotated",
            "a copy holding an edit stays"
        );
        assert!(fx.session.opened().files().is_empty());
    }

    /// A folder project from elsewhere can hold, as a stored file's own, a link to any
    /// file on the disk. Its row shows it missing once read, and a row read before the
    /// link was made does not hand it to the desktop either.
    #[cfg(unix)]
    #[test]
    fn a_stored_file_linked_out_of_its_folder_is_never_handed_over() {
        let fx = fixture();
        let src = fx.data.join("Village map.pdf");
        std::fs::write(&src, b"%PDF-1.7 map").expect("write");
        store_and_add(&fx.project.app_ctx, &fx.session, fx.scene, &[src]);
        let before = fx.session.model().rows_for(fx.scene)[0].clone();
        let stored = before.path.clone().expect("path");
        let script = fx.data.join("run.command");
        std::fs::write(&script, b"#!/bin/sh\necho gotcha").expect("write");
        std::fs::remove_file(&stored).expect("remove");
        std::os::unix::fs::symlink(&script, &stored).expect("link");

        // Folder shape: without the check, the path itself would be handed over.
        let vm = ResourcesViewModel::new(
            fx.project.app_ctx.clone(),
            fx.project.ids.clone(),
            fx.session.clone(),
            Signal::new(Some(fx.scene)),
            Signal::new(Some(WorkShape::Folder)),
            Signal::new(String::new()),
        );
        assert!(!before.missing, "read before the link was made");
        assert!(
            vm.file_to_hand_over(&before).is_err(),
            "a link out of its folder is not opened"
        );
        assert!(
            fx.session.opened().files().is_empty(),
            "nor remembered as handed over"
        );

        fx.session.model().reload();
        let after = fx.session.model().by_id(before.id).expect("row");
        assert!(after.missing, "the project does not hold that file");
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

#[cfg(unix)]
#[test]
fn a_link_is_judged_by_the_file_it_points_at() {
    let dir = tempfile::tempdir().expect("tmp");
    let script = dir.path().join("run.command");
    std::fs::write(&script, b"#!/bin/sh").expect("write");
    let link = dir.path().join("map.pdf");
    std::os::unix::fs::symlink(&script, &link).expect("link");
    let disguised = ResourceRow {
        kind: ResourceKind::LinkedFile,
        file_name: "map.pdf".into(),
        location: link.to_string_lossy().into_owned(),
        path: Some(link.clone()),
        ..row(11, "Village map")
    };
    assert_eq!(name_to_judge(&disguised), "run.command");
    assert_eq!(
        open_safety::verdict(&name_to_judge(&disguised)),
        OpenVerdict::Ask,
        "macOS would run what the link points at"
    );
    // A link to nothing has no name to trust: it asks, and is no picture.
    let dangling = dir.path().join("photo.png");
    std::os::unix::fs::symlink(dir.path().join("gone.png"), &dangling).expect("link");
    let dangling = ResourceRow {
        path: Some(dangling),
        ..disguised.clone()
    };
    assert_eq!(
        open_safety::verdict(&name_to_judge(&dangling)),
        OpenVerdict::Ask
    );
    assert!(!file_kind::is_viewable_picture(&name_to_judge(&dangling)));
}
