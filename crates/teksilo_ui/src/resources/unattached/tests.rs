// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

use super::*;

#[test]
fn only_a_single_file_project_past_the_notice_size_is_told_a_folder_saves_faster() {
    let at = sizes::ZIP_NOTICE_BYTES;
    assert!(suggests_folder(at, Some(WorkShape::Zip)));
    assert!(suggests_folder(at * 3, Some(WorkShape::Zip)));
    assert!(
        !suggests_folder(at - 1, Some(WorkShape::Zip)),
        "under it: nothing to say"
    );
    assert!(
        !suggests_folder(at * 3, Some(WorkShape::Folder)),
        "a folder project already saves only what changed"
    );
    assert!(
        !suggests_folder(at * 3, None),
        "a project never saved has no shape yet"
    );
}

/// The mock project's two loose resources are the page's rows; every other is on an item.
#[cfg(feature = "mocks")]
#[test]
fn the_mock_page_lists_the_two_resources_on_no_item() {
    let app_ctx = Rc::new(frontend::AppContext::new());
    let ids = crate::app_ids::AppIds::new();
    let page = UnattachedResourcesViewModel::new(
        ids.work_id.clone(),
        ResourcesViewModel::detached(app_ctx, ids),
    );
    let titles: Vec<String> = page.rows().into_iter().map(|r| r.title).collect();
    assert_eq!(titles, ["Keeper's cottage", "Lighthouse history"]);
}

#[cfg(not(feature = "mocks"))]
mod real {
    use std::path::PathBuf;

    use frontend::commands::undo_redo_commands;

    use super::*;
    use crate::models::WebLink;
    use crate::resources::ResourcesSession;
    use crate::test_support::RealProject;

    struct Fixture {
        project: RealProject,
        page: UnattachedResourcesViewModel,
        scene: u64,
        second: u64,
    }

    fn fixture() -> Fixture {
        let project = RealProject::empty_novel();
        let (scene, _) = project.scenes()[0];
        let (second, _) = project.add_scene_after(scene, "The quay");
        let session = ResourcesSession::new(project.app_ctx.clone(), project.ids.clone());
        let resources = ResourcesViewModel::new(
            project.app_ctx.clone(),
            project.ids.clone(),
            session,
            Signal::new(None),
            Signal::new(Some(WorkShape::Zip)),
            Signal::new(String::new()),
        );
        let page = UnattachedResourcesViewModel::new(project.ids.work_id.clone(), resources);
        Fixture {
            project,
            page,
            scene,
            second,
        }
    }

    fn links(fx: &Fixture, items: &[u64], titles: &[&str]) -> Vec<u64> {
        let web: Vec<WebLink> = titles
            .iter()
            .map(|t| WebLink {
                url: format!("https://example.org/{}", t.to_lowercase()),
                title: (*t).to_string(),
            })
            .collect();
        fx.page
            .resources()
            .session()
            .model()
            .add(items, &Default::default(), &[] as &[PathBuf], &web)
            .unwrap_or_else(|e| panic!("add: {e}"))
    }

    fn titles(fx: &Fixture) -> Vec<String> {
        fx.page.rows().into_iter().map(|r| r.title).collect()
    }

    fn stack_size(fx: &Fixture) -> usize {
        fx.project.ids.stack_id.get().map_or(0, |s| {
            undo_redo_commands::get_stack_size(&fx.project.app_ctx, s)
        })
    }

    #[test]
    fn the_page_lists_only_the_resources_on_no_item() {
        let fx = fixture();
        links(&fx, &[fx.scene], &["On a scene"]);
        links(&fx, &[], &["Tides", "Harbour"]);
        let loose = links(&fx, &[fx.second], &["Gulls"]);
        fx.page
            .resources()
            .session()
            .model()
            .detach(fx.second, loose[0])
            .unwrap_or_else(|e| panic!("detach: {e}"));
        assert_eq!(titles(&fx), ["Gulls", "Harbour", "Tides"]);
    }

    /// An attach writes each item's whole list, built on what the item carries: read as
    /// a result, so a read that failed could never stand for "carries nothing" and the
    /// write take the item's other resources off.
    #[test]
    fn attaching_keeps_what_the_item_carries_in_its_order() {
        let fx = fixture();
        let first = links(&fx, &[fx.scene], &["Tides", "Harbour"]);
        let loose = links(&fx, &[], &["Gulls"]);
        let model = fx.page.resources().session().model();
        model
            .attach(&[fx.scene, fx.second], &loose)
            .unwrap_or_else(|e| panic!("attach: {e}"));
        let carried = model
            .attachments(&[fx.scene, fx.second])
            .unwrap_or_else(|e| panic!("attachments: {e}"));
        assert_eq!(
            carried.get(&fx.scene),
            Some(&vec![first[0], first[1], loose[0]])
        );
        assert_eq!(carried.get(&fx.second), Some(&loose));
        assert_eq!(
            model
                .unattached()
                .unwrap_or_else(|e| panic!("unattached: {e}")),
            Vec::<ResourceRow>::new()
        );
    }

    #[test]
    fn attaching_several_to_several_items_is_one_undo_step_and_empties_the_page() {
        let fx = fixture();
        links(&fx, &[], &["Tides", "Harbour"]);
        let rows = fx.page.rows();
        let before = stack_size(&fx);
        let mut tree = crate::test_support::tree_with_events(&fx.project.app_ctx);
        {
            let (page, rows, items) = (fx.page.clone(), rows.clone(), vec![fx.scene, fx.second]);
            crate::test_support::press(&mut tree, move |c| {
                assert!(page.attach(c, &items, &rows));
            });
        }
        assert!(titles(&fx).is_empty(), "nothing loose any more");
        let model = fx.page.resources().session().model();
        assert_eq!(model.rows_for(fx.scene).len(), 2);
        assert_eq!(model.rows_for(fx.second).len(), 2);
        assert_eq!(stack_size(&fx), before + 1, "two items, one step");
        undo_redo_commands::undo(&fx.project.app_ctx, fx.project.ids.stack_id.get())
            .unwrap_or_else(|e| panic!("undo: {e}"));
        assert_eq!(titles(&fx), ["Harbour", "Tides"], "one undo puts both back");
    }

    #[test]
    fn deleting_several_asks_once_and_is_one_undo_step() {
        let fx = fixture();
        links(&fx, &[], &["Tides", "Harbour", "Gulls"]);
        let rows: Vec<ResourceRow> = fx.page.rows().into_iter().take(2).collect();
        let before = stack_size(&fx);
        let mut tree = crate::test_support::tree_with_events(&fx.project.app_ctx);
        {
            let (page, rows) = (fx.page.clone(), rows.clone());
            crate::test_support::press(&mut tree, move |c| page.delete(c, &rows));
        }
        let title = tr!(resources_remove_many_title(count = 2)).resolve_now();
        let confirm = tr!(resources_remove_confirm()).resolve_now();
        crate::test_support::answer_dialog(&mut tree, &title, &confirm);
        assert_eq!(titles(&fx), ["Tides"]);
        assert_eq!(stack_size(&fx), before + 1);
    }

    #[test]
    fn the_size_line_counts_every_stored_file_attached_or_not() {
        let fx = fixture();
        let dir = tempfile::tempdir().unwrap_or_else(|e| panic!("tmp: {e}"));
        let model = fx.page.resources().session().model();
        model.set_media_root(dir.path().join("media"));
        let stored = frontend::resource_management::StoredResourceFilesDto {
            uids: vec![
                uuid::Uuid::new_v4().hyphenated().to_string(),
                uuid::Uuid::new_v4().hyphenated().to_string(),
            ],
            file_names: vec!["a.pdf".into(), "b.pdf".into()],
            mime_types: vec![String::new(), String::new()],
            byte_sizes: vec![sizes::ZIP_NOTICE_BYTES - 10, 10],
            content_hashes: vec!["1".repeat(64), "2".repeat(64)],
            widths: vec![0, 0],
            heights: vec![0, 0],
            ..Default::default()
        };
        let ids = model
            .add(&[fx.scene], &stored, &[], &[])
            .unwrap_or_else(|e| panic!("add: {e}"));
        assert_eq!(fx.page.stored_bytes(), sizes::ZIP_NOTICE_BYTES);
        assert!(
            fx.page.suggests_folder(),
            "a single-file project at the notice size"
        );
        model
            .detach(fx.scene, ids[0])
            .unwrap_or_else(|e| panic!("detach: {e}"));
        assert_eq!(
            fx.page.stored_bytes(),
            sizes::ZIP_NOTICE_BYTES,
            "a loose stored file still travels with the project"
        );
        links(&fx, &[], &["Tides"]);
        assert_eq!(
            fx.page.stored_bytes(),
            sizes::ZIP_NOTICE_BYTES,
            "a link stores nothing"
        );
    }
}
