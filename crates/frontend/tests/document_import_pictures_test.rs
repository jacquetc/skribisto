// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! A picture that came in with an imported document is the project's: a save writes it into
//! the `.skrib`, and the project opened again, anywhere, shows it.
//!
//! End to end through the app's own commands, for both shapes a project takes: a zip, whose
//! pictures live in a working folder under the app's data while it is open and travel inside
//! the file, and a folder, whose `assets/` is where they live. Each is opened again into a
//! fresh store with a media root it has never used, so nothing but the saved project can
//! supply the picture.

use std::path::{Path, PathBuf};
use std::time::Duration;

use frontend::AppContext;
use frontend::commands::{
    asset_commands, binder_commands, import_management_commands, undo_redo_commands, work_commands,
    work_management_commands,
};
use frontend::common::direct_access::work::WorkRelationshipField;
use import_management::{
    AnalyzeDocumentImportDto, ApplyDocumentImportDto, ApplyImportRow, ApplyImportRows,
    DocumentImportRow, DocumentImportRows, DropPosition, ImportPicture, ImportPictures,
};
use work_management::{LoadWorkDto, NewWorkDto, NewWorkTemplate, SaveWorkDto};

/// A one-pixel PNG, the smallest real picture there is.
const PNG: &[u8] = &[
    0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, 0x00, 0x00, 0x00, 0x0D, 0x49, 0x48, 0x44, 0x52,
    0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x08, 0x06, 0x00, 0x00, 0x00, 0x1F, 0x15, 0xC4,
    0x89, 0x00, 0x00, 0x00, 0x0D, 0x49, 0x44, 0x41, 0x54, 0x78, 0xDA, 0x63, 0x64, 0x60, 0xF8, 0x5F,
    0x0F, 0x00, 0x02, 0x87, 0x01, 0x80, 0xEB, 0x47, 0xBA, 0x92, 0x00, 0x00, 0x00, 0x00, 0x49, 0x45,
    0x4E, 0x44, 0xAE, 0x42, 0x60, 0x82,
];

/// A scratch folder of this test's own, removed when it is dropped.
struct Scratch(PathBuf);

impl Scratch {
    fn new(name: &str) -> Self {
        let dir = std::env::temp_dir().join(format!(
            "skrib-import-pictures-{name}-{}",
            std::process::id()
        ));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).expect("scratch folder");
        Scratch(dir)
    }
}

impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

fn wait(ctx: &AppContext, op_id: &str) {
    let completion = ctx
        .long_operation_manager
        .lock()
        .expect("manager")
        .completion_signal();
    assert!(
        completion.wait_for(op_id, Some(Duration::from_secs(60))),
        "the operation finishes"
    );
}

fn the_work(ctx: &AppContext) -> u64 {
    work_commands::get_all_work(ctx)
        .expect("works")
        .pop()
        .expect("one Work")
        .id
}

fn assets_of(ctx: &AppContext, work: u64) -> Vec<frontend::asset::dtos::AssetDto> {
    let ids = work_commands::get_work_relationship(ctx, &work, &WorkRelationshipField::Assets)
        .expect("assets");
    asset_commands::get_asset_multi(ctx, &ids)
        .expect("asset rows")
        .into_iter()
        .flatten()
        .collect()
}

/// Import `document` into the open project's first binder, with the wizard's own folders,
/// on an undo stack of its own. The picture's hash, and the stack.
fn import(ctx: &AppContext, document: &Path, staging: &Path, media_root: &Path) -> (String, u64) {
    let work = the_work(ctx);
    let binder = work_commands::get_work_relationship(ctx, &work, &WorkRelationshipField::Binders)
        .expect("binders")[0];
    // The wizard's folder, made by the wizard: the analysis only writes into it.
    std::fs::create_dir_all(staging).expect("the picture folder");
    let op = import_management_commands::analyze_document_import(
        ctx,
        &AnalyzeDocumentImportDto {
            work_id: work,
            binder_id: binder,
            source_paths: vec![document.to_string_lossy().into_owned()],
            anchor_item_id: 0,
            drop_position: DropPosition::Into,
            picture_dir: staging.to_string_lossy().into_owned(),
        },
    )
    .expect("analyse");
    wait(ctx, &op);
    let plan = import_management_commands::get_analyze_document_import_result(ctx, &op)
        .expect("result")
        .expect("a plan");
    let ImportPictures::Found(pictures) = &plan.pictures else {
        panic!("the plan lists its pictures");
    };
    let hash = match pictures.as_slice() {
        [ImportPicture::Found { content_hash, .. }] => content_hash.clone(),
        other => panic!("one picture: {other:?}"),
    };
    let DocumentImportRows::Found(rows) = plan.rows else {
        panic!("rows");
    };
    let rows = rows
        .into_iter()
        .filter_map(|row| match row {
            DocumentImportRow::Found {
                indent,
                kind,
                title,
                djot,
                ..
            } => Some(ApplyImportRow::Create {
                indent,
                kind,
                title,
                djot,
                epigraph: String::new(),
                comments: Vec::new(),
                footnotes: Vec::new(),
                source_uid_tag: String::new(),
                source_file_name: "chapter.md".into(),
                source_file_digest: String::new(),
            }),
            DocumentImportRow::Empty => None,
        })
        .collect();
    let stack = undo_redo_commands::create_new_stack(ctx);
    import_management_commands::apply_document_import(
        ctx,
        Some(stack),
        &ApplyDocumentImportDto {
            work_id: work,
            binder_id: binder,
            anchor_item_id: 0,
            drop_position: DropPosition::Into,
            row: ApplyImportRow::Empty,
            rows: ApplyImportRows::Create(rows),
            pictures: plan.pictures.clone(),
            picture_dir: staging.to_string_lossy().into_owned(),
            media_root: media_root.to_string_lossy().into_owned(),
        },
    )
    .expect("apply");
    (hash, stack)
}

fn save(ctx: &AppContext, project: &Path, media_root: &Path) {
    let op = work_management_commands::save_work(
        ctx,
        &SaveWorkDto {
            work_id: the_work(ctx),
            file_name: project.to_string_lossy().into_owned(),
            overwrite: true,
            media_root: media_root.to_string_lossy().into_owned(),
        },
    )
    .expect("save");
    wait(ctx, &op);
    work_management_commands::get_save_work_result(ctx, &op)
        .expect("save result")
        .expect("a finished save");
}

/// A new project at `project`, zip or folder, saved once: the save that makes its file, and
/// with a folder project the folder its pictures live in from then on.
fn new_project(project: &Path, media_root: &Path, is_folder: bool) -> AppContext {
    let ctx = AppContext::new();
    work_management_commands::new_work(
        &ctx,
        &NewWorkDto {
            goal_unit: Default::default(),
            file_name: project.to_string_lossy().into_owned(),
            title: String::new(),
            is_folder,
            template_kind: NewWorkTemplate::EmptyNovel,
            labels: vec![],
            language: vec!["en-US".to_string()],
            author_name: String::new(),
            chapter_scene_mode: false,
            paratext_front: Vec::new(),
            paratext_back: Vec::new(),
        },
    )
    .expect("new work");
    save(&ctx, project, media_root);
    ctx
}

/// Save As `target`, then record the new path and shape as the app does once it lands
/// (`SaveAsViewModel::on_long_op_completed`): the project is `target` from then on.
fn save_as(ctx: &AppContext, target: &Path, as_folder: bool, media_root: &Path) {
    let op = work_management_commands::save_as(
        ctx,
        &work_management::SaveAsDto {
            work_id: the_work(ctx),
            file_name: target.to_string_lossy().into_owned(),
            as_folder,
            media_root: media_root.to_string_lossy().into_owned(),
        },
    )
    .expect("save as");
    wait(ctx, &op);
    let output = work_management_commands::get_save_as_result(ctx, &op)
        .expect("save as result")
        .expect("a finished Save As")
        .output_path;
    let info = frontend::commands::work_info_commands::get_all_work_info(ctx)
        .expect("work infos")
        .into_iter()
        .find(|info| info.work == Some(the_work(ctx)))
        .expect("the project's WorkInfo");
    frontend::commands::work_info_commands::update_work_info(
        ctx,
        &frontend::direct_access::UpdateWorkInfoDto {
            id: info.id,
            created_at: info.created_at,
            updated_at: chrono::Utc::now(),
            file_name: Some(output),
            shape: if as_folder {
                frontend::common::entities::WorkShape::Folder
            } else {
                frontend::common::entities::WorkShape::Zip
            },
        },
    )
    .expect("the new path recorded");
}

/// The pictures of `project` opened again into a fresh store, with a media root it has never
/// used: each asset's hash, and whether its bytes came with it.
fn reopened_pictures(project: &Path, root: &Path) -> Vec<(String, bool)> {
    let fresh_root = root.join("reopened").join("media");
    let ctx = AppContext::new();
    work_management_commands::load_work(
        &ctx,
        &LoadWorkDto {
            media_root: fresh_root.to_string_lossy().into_owned(),
            file_name: project.to_string_lossy().into_owned(),
        },
    )
    .expect("the project opens again");
    let work = work_commands::get_work(&ctx, &the_work(&ctx))
        .expect("work")
        .expect("the work");
    let media_dir =
        skrib_format::media::media_dir(project, &work.unique_id, &fresh_root, &work.unique_id);
    assets_of(&ctx, work.id)
        .into_iter()
        .map(|asset| {
            let present = media_dir
                .join(format!("{}.png", asset.content_hash))
                .exists();
            (asset.content_hash, present)
        })
        .collect()
}

/// A folder project's first save makes the folder its pictures live in from then on, and a
/// save reads them there. The import used to be handed the directory the project had when it
/// was created, under the app's data, so a picture imported after that first save was
/// written where no save looks, and the next one dropped its row: the project opened again
/// with prose naming a picture it did not hold.
#[test]
fn a_picture_imported_after_a_new_projects_first_save_stays_with_it() {
    for is_folder in [true, false] {
        let scratch = Scratch::new(&format!("after-first-save-{is_folder}"));
        let root = &scratch.0;
        let project = root.join("Novel.skrib");
        let media_root = root.join("data").join("media");
        std::fs::create_dir_all(root.join("sources")).expect("sources");
        std::fs::write(root.join("sources").join("map.png"), PNG).expect("png");
        let document = root.join("sources").join("chapter.md");
        std::fs::write(&document, "# The Harbour\n\n![The map](map.png)\n").expect("document");

        let ctx = new_project(&project, &media_root, is_folder);
        let (hash, _) = import(&ctx, &document, &root.join("staging"), &media_root);
        save(&ctx, &project, &media_root);
        drop(ctx);

        assert_eq!(
            reopened_pictures(&project, root),
            vec![(hash, true)],
            "is_folder={is_folder}"
        );
    }
}

/// Undo holds the pictures an import created, for a redo after a save pruned them. Redo
/// wrote them back where the import had first put them; a Save As in between moves the
/// project, and its next save reads another directory, so the redone import's pictures were
/// dropped from the copy, their prose still naming them.
#[test]
fn a_redo_after_a_save_as_brings_the_pictures_where_the_project_now_is() {
    for (from_folder, to_folder) in [(true, true), (false, true), (true, false)] {
        let scratch = Scratch::new(&format!("redo-save-as-{from_folder}-{to_folder}"));
        let root = &scratch.0;
        let project = root.join("Novel.skrib");
        let media_root = root.join("data").join("media");
        std::fs::create_dir_all(root.join("sources")).expect("sources");
        std::fs::write(root.join("sources").join("map.png"), PNG).expect("png");
        let document = root.join("sources").join("chapter.md");
        std::fs::write(&document, "# The Harbour\n\n![The map](map.png)\n").expect("document");

        let ctx = new_project(&project, &media_root, from_folder);
        let (hash, stack) = import(&ctx, &document, &root.join("staging"), &media_root);
        undo_redo_commands::undo(&ctx, Some(stack)).expect("undo");
        let copy = root.join("Copy.skrib");
        save_as(&ctx, &copy, to_folder, &media_root);
        undo_redo_commands::redo(&ctx, Some(stack)).expect("redo");
        save(&ctx, &copy, &media_root);
        drop(ctx);

        assert_eq!(
            reopened_pictures(&copy, root),
            vec![(hash, true)],
            "from_folder={from_folder} to_folder={to_folder}"
        );
    }
}

#[test]
fn an_imported_picture_is_saved_with_the_project_and_opens_with_it() {
    for is_folder in [false, true] {
        let scratch = Scratch::new(if is_folder { "folder" } else { "zip" });
        let root = &scratch.0;
        let project = root.join("Novel.skrib");
        let media_root = root.join("data").join("media");
        std::fs::create_dir_all(root.join("sources").join("images")).expect("sources");
        std::fs::write(root.join("sources").join("images").join("map.png"), PNG).expect("png");
        let document = root.join("sources").join("chapter.md");
        std::fs::write(&document, "# The Harbour\n\n![The map](images/map.png)\n")
            .expect("document");

        let ctx = AppContext::new();
        work_management_commands::new_work(
            &ctx,
            &NewWorkDto {
                goal_unit: Default::default(),
                file_name: project.to_string_lossy().into_owned(),
                title: String::new(),
                is_folder,
                template_kind: NewWorkTemplate::EmptyNovel,
                labels: vec![],
                language: vec!["en-US".to_string()],
                author_name: String::new(),
                chapter_scene_mode: false,
                paratext_front: Vec::new(),
                paratext_back: Vec::new(),
            },
        )
        .expect("new work");
        // The first save makes the file, and with it the folder a folder project keeps its
        // pictures in: the media directory is resolved from what is on disk.
        save(&ctx, &project, &media_root);
        let work = work_commands::get_work(&ctx, &the_work(&ctx))
            .expect("work")
            .expect("the work");
        let media_dir =
            skrib_format::media::media_dir(&project, &work.unique_id, &media_root, &work.unique_id);

        let (hash, _) = import(&ctx, &document, &root.join("staging"), &media_root);
        let stored_name = format!("{hash}.png");
        assert_eq!(assets_of(&ctx, work.id).len(), 1);
        assert_eq!(
            std::fs::read(media_dir.join(&stored_name)).expect("in the media directory"),
            PNG
        );
        save(&ctx, &project, &media_root);
        drop(ctx);

        // Everything this session touched outside the project, gone: only the file can
        // supply the picture now.
        let _ = std::fs::remove_dir_all(root.join("data"));
        let _ = std::fs::remove_dir_all(root.join("staging"));
        let _ = std::fs::remove_dir_all(root.join("sources"));

        let fresh_root = root.join("elsewhere").join("media");
        let ctx = AppContext::new();
        work_management_commands::load_work(
            &ctx,
            &LoadWorkDto {
                media_root: fresh_root.to_string_lossy().into_owned(),
                file_name: project.to_string_lossy().into_owned(),
            },
        )
        .expect("the project opens again");
        let reopened = work_commands::get_work(&ctx, &the_work(&ctx))
            .expect("work")
            .expect("the work");
        let assets = assets_of(&ctx, reopened.id);
        assert_eq!(assets.len(), 1, "is_folder={is_folder}");
        assert_eq!(assets[0].content_hash, hash);
        assert_eq!(assets[0].mime_type, "image/png");
        assert_eq!((assets[0].width, assets[0].height), (1, 1));
        let reopened_media = skrib_format::media::media_dir(
            &project,
            &reopened.unique_id,
            &fresh_root,
            &reopened.unique_id,
        );
        assert_eq!(
            std::fs::read(reopened_media.join(&stored_name)).expect("the picture's bytes"),
            PNG,
            "is_folder={is_folder}: the bytes travelled with the project"
        );
        let prose: Vec<String> = frontend::commands::content_commands::get_all_content(&ctx)
            .expect("contents")
            .into_iter()
            .map(|c| c.data)
            .collect();
        assert!(
            prose
                .iter()
                .any(|p| p.contains(&format!("![The map](assets/{stored_name})"))),
            "{prose:?}"
        );
        let binders = binder_commands::get_all_binder(&ctx).expect("binders");
        assert!(!binders.is_empty());
    }
}
