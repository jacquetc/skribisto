// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Where this installation keeps project media.
//!
//! Every backend call that reads or writes a project's image bytes takes a
//! *media root* — the app-level directory, not the project's own folder inside
//! it. The split matters at load time: which directory a project uses depends on
//! its `unique_id`, and that id is still inside the file being opened, so only
//! the use case can resolve it (see `skrib_format::media`).
//!
//! Not a view-model — a path helper with no state.

use std::path::PathBuf;

/// The app's media root, `<data_dir>/media`.
///
/// `data_dir` rather than `cache_dir`: bytes land here the moment a writer
/// inserts an image, before any save, so this is where an unsaved picture lives
/// until the project is written. A cache directory is something the OS may
/// reclaim, and reclaiming it would destroy work the writer has not yet saved.
///
/// Returns an empty path when the platform offers no data directory. The
/// backend treats that as "no media", so a project on such a platform still
/// opens — without its images — rather than failing to open at all.
pub fn media_root() -> PathBuf {
    crate::identity::app_paths()
        .map(|paths| paths.data_dir().join("media"))
        .unwrap_or_default()
}

/// The media root as the `String` the DTOs carry.
pub fn media_root_string() -> String {
    media_root().to_string_lossy().into_owned()
}

/// The media directory of the open Work `work_id` where the project is now, under
/// `media_root`: resolved from the path its `WorkInfo` holds at this moment and its
/// `unique_id`, the two a save resolves it from (`skrib_format::media::media_dir`).
///
/// Asked each time rather than resolved once when the Work opens, because the answer moves
/// with the project. A new folder project lives in the uid-keyed directory until its first
/// save makes the folder, and in the folder's own `assets/` from then on; a Save As moves it
/// too. A picture written into the directory resolved at opening then sat where no save
/// reads, and the next save dropped it.
pub fn project_media_dir(
    app_ctx: &frontend::AppContext,
    work_id: u64,
    media_root: &std::path::Path,
) -> PathBuf {
    let uid = frontend::commands::work_commands::get_work(app_ctx, &work_id)
        .ok()
        .flatten()
        .map(|work| work.unique_id)
        .unwrap_or_default();
    let project = frontend::commands::work_info_commands::get_all_work_info(app_ctx)
        .ok()
        .into_iter()
        .flatten()
        .find(|info| info.work == Some(work_id))
        .and_then(|info| info.file_name)
        .unwrap_or_default();
    skrib_format::media::media_dir(std::path::Path::new(&project), &uid, media_root, &uid)
}

#[cfg(all(test, not(feature = "mocks")))]
mod tests {
    use super::*;
    use frontend::commands::{handling_app_lifecycle_commands, work_management_commands};
    use frontend::work_management::{NewWorkDto, NewWorkTemplate, SaveWorkDto};

    /// A new folder project keeps its pictures under the app's data until its first save
    /// makes its folder, and in the folder's `assets/` from then on, where every later save
    /// reads them. The directory resolved when the project was created stayed the answer
    /// for the whole session, so a picture added after the first save was dropped by the
    /// next one.
    #[test]
    fn the_media_directory_follows_a_new_folder_project_to_its_folder() {
        let dir = tempfile::tempdir().expect("tempdir");
        let project = dir.path().join("Novel.skrib");
        let root = dir.path().join("data");
        let app_ctx = frontend::AppContext::new();
        handling_app_lifecycle_commands::initialize_app(&app_ctx).expect("app");
        work_management_commands::new_work(
            &app_ctx,
            &NewWorkDto {
                goal_unit: Default::default(),
                file_name: project.to_string_lossy().into_owned(),
                title: String::new(),
                is_folder: true,
                template_kind: NewWorkTemplate::EmptyNovel,
                labels: vec![],
                language: vec!["en".to_string()],
                author_name: String::new(),
                chapter_scene_mode: false,
                paratext_front: Vec::new(),
                paratext_back: Vec::new(),
            },
        )
        .expect("new work");
        let work = frontend::commands::work_commands::get_all_work(&app_ctx)
            .expect("works")
            .pop()
            .expect("the work");

        let before = project_media_dir(&app_ctx, work.id, &root);
        assert_eq!(
            before,
            skrib_format::media::media_dir(
                std::path::Path::new(""),
                &work.unique_id,
                &root,
                &work.unique_id
            ),
            "unsaved, it waits under the app's data"
        );

        let op = work_management_commands::save_work(
            &app_ctx,
            &SaveWorkDto {
                work_id: work.id,
                file_name: project.to_string_lossy().into_owned(),
                overwrite: true,
                media_root: root.to_string_lossy().into_owned(),
            },
        )
        .expect("save");
        let completion = app_ctx
            .long_operation_manager
            .lock()
            .expect("manager")
            .completion_signal();
        assert!(completion.wait_for(&op, Some(std::time::Duration::from_secs(60))));

        let after = project_media_dir(&app_ctx, work.id, &root);
        assert_eq!(
            Some(after),
            skrib_format::media::folder_project_root(&project)
                .map(|folder| folder.join(skrib_format::media::ASSETS_DIR)),
            "saved, its pictures live in its own folder"
        );
    }
}
