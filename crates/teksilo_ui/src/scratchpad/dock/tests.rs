// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

use super::*;

/// `s` without the bidi isolation marks Fluent puts around each placeable.
fn plain(s: &str) -> String {
    s.chars()
        .filter(|c| !matches!(c, '\u{2068}' | '\u{2069}'))
        .collect()
}

/// Every name, value and description the tree gives a screen reader.
fn spoken(tree: &mut teksilo::core::WidgetTree) -> Vec<String> {
    let update = tree.sync_accessibility();
    let mut out = Vec::new();
    for (_, node) in update.nodes.iter() {
        for s in [node.label(), node.value(), node.description()]
            .into_iter()
            .flatten()
        {
            out.push(plain(s));
        }
    }
    out
}

fn format() -> FormatViewModel {
    FormatViewModel::new(Rc::new(|| (None, crate::format::FormatSurface::None)))
}

/// With nothing in front of the writer the panel says how to get notes, and lays out.
#[test]
fn with_no_item_the_panel_says_what_to_do() {
    let ctx = Rc::new(frontend::AppContext::new());
    let vm = ScratchpadViewModel::new(
        ctx.clone(),
        crate::models::OpenDocsStore::new(ctx.clone()),
        crate::app_ids::AppIds::new(),
    );
    crate::test_support::with_real_messages(|| {
        let mut tree = crate::test_support::tree_with_settings(&ctx);
        let id = tree.add(ScratchpadPanel::new(vm, format()));
        tree.layout(SizeProposal::exact(300.0, 600.0));
        assert!(tree.bounds(id).width > 0.0);
        let said = spoken(&mut tree);
        assert!(
            said.iter()
                .any(|s| s == "Open an item to keep your notes beside it."),
            "{said:?}"
        );
    });
}

/// The dock is a door from the rail and from View ▸ Panels, with Ctrl+Shift+A.
#[test]
fn the_dock_is_named_and_has_its_chord() {
    let panel = crate::docks::panels::PANELS
        .iter()
        .find(|p| p.dock == crate::docks::SCRATCHPAD_DOCK_ID)
        .expect("a panel command for the Scratchpad");
    assert_eq!(panel.intent, "scratchpad.show");
    assert_eq!(panel.chord, Key::A);
    crate::test_support::with_real_messages(|| {
        assert_eq!((panel.title)().resolve_now(), "Scratchpad");
        assert_eq!((panel.name)().resolve_now(), "Show the scratchpad");
    });
    let dock = crate::docks::APP_DOCKS
        .iter()
        .find(|d| d.id == crate::docks::SCRATCHPAD_DOCK_ID)
        .expect("in the roster");
    assert_eq!(dock.side, DockSide::Trailing);
    assert!(dock.own_tab, "its own activity on the rail");
}

#[cfg(not(feature = "mocks"))]
mod real {
    use super::*;
    use frontend::commands::{binder_commands, binder_item_commands, work_commands};
    use frontend::common::entities::{BinderItemRole, BinderItemSubRole};
    use frontend::direct_access::{CreateBinderDto, CreateBinderItemDto, CreateWorkDto};
    use teksilo::core::WidgetTree;

    use crate::models::OpenDocsStore;

    struct Mounted {
        ctx: Rc<frontend::AppContext>,
        docs: OpenDocsStore,
        vm: ScratchpadViewModel,
        format: FormatViewModel,
        tree: WidgetTree,
        root: WidgetId,
        items: Vec<u64>,
    }

    /// Two scenes and an end-of-book marker in a real store, and the panel mounted over
    /// a view-model of its own.
    fn mounted() -> Mounted {
        let ctx = Rc::new(frontend::AppContext::new());
        let work = work_commands::create_orphan_work(&ctx, None, &CreateWorkDto::default())
            .expect("a work")
            .id;
        let binder = binder_commands::create_binder(
            &ctx,
            None,
            &CreateBinderDto {
                name: "Manuscript".into(),
                activated: true,
                ..Default::default()
            },
            work,
            0,
        )
        .expect("a binder");
        let items = [
            ("Low tide", BinderItemSubRole::Scene),
            ("The keeper", BinderItemSubRole::Scene),
            ("", BinderItemSubRole::BookEnd),
        ]
        .into_iter()
        .map(|(title, sub_role)| {
            binder_item_commands::create_binder_item(
                &ctx,
                None,
                &CreateBinderItemDto {
                    title: title.into(),
                    role: BinderItemRole::Item,
                    sub_role,
                    activated: true,
                    is_exportable: true,
                    ..Default::default()
                },
                binder.id,
                -1,
            )
            .expect("an item")
            .id
        })
        .collect();
        let docs = OpenDocsStore::new(ctx.clone());
        let vm = ScratchpadViewModel::new(ctx.clone(), docs.clone(), crate::app_ids::AppIds::new());
        let format = format();
        let mut tree = crate::test_support::tree_with_settings(&ctx);
        let root = tree.add(ScratchpadPanel::new(vm.clone(), format.clone()));
        tree.layout(SizeProposal::exact(300.0, 600.0));
        Mounted {
            ctx,
            docs,
            vm,
            format,
            tree,
            root,
            items,
        }
    }

    impl Mounted {
        fn settle(&mut self) {
            let mut noop = teksilo::core::window::NoopWindowOps;
            self.tree.layout(SizeProposal::exact(300.0, 600.0));
            self.tree.run_mount_actions(&mut noop);
            self.tree.layout(SizeProposal::exact(300.0, 600.0));
            let _ = self.tree.render();
        }

        /// The scratchpad editor's handle, reached the way the Format dock reaches it:
        /// by putting the caret in it.
        fn editor(&mut self) -> EditorHandle {
            let stop = *self
                .tree
                .tab_stops_within(self.root)
                .first()
                .expect("the editor is a Tab stop");
            self.tree.focus(stop);
            self.format
                .handle_for_commands()
                .expect("the focused scratchpad is the format target")
        }
    }

    /// An item's scratchpad is an editor named "Scratchpad", described by the item it
    /// belongs to, under a line naming that item.
    #[test]
    fn an_item_shows_its_scratchpad_under_its_name() {
        crate::test_support::with_real_messages(|| {
            let mut m = mounted();
            m.vm.view_changed(Some(m.items[0]), false, None);
            m.settle();
            let said = spoken(&mut m.tree);
            assert!(
                said.iter().any(|s| s == "Scratchpad · Low tide"),
                "{said:?}"
            );
            assert!(said.iter().any(|s| s == "Scratchpad"), "{said:?}");
            assert!(
                said.iter()
                    .any(|s| s == "Your private notes on Low tide, never exported or counted"),
                "{said:?}"
            );
        });
    }

    /// A marker says it has no scratchpad, and shows no editor.
    #[test]
    fn a_marker_says_it_has_no_scratchpad() {
        crate::test_support::with_real_messages(|| {
            let mut m = mounted();
            m.vm.view_changed(Some(m.items[2]), false, None);
            m.settle();
            let said = spoken(&mut m.tree);
            assert!(
                said.iter()
                    .any(|s| s == "This marker holds no text, so it has no scratchpad."),
                "{said:?}"
            );
            assert!(m.tree.tab_stops_within(m.root).is_empty(), "no editor");
        });
    }

    /// Typing in the mounted editor marks the item's document changed and bumps the
    /// store's edit counter, which the autosave and the unsaved mark observe; a save's
    /// flush then writes the scratchpad's row.
    #[test]
    fn typing_in_the_dock_marks_the_project_changed() {
        let mut m = mounted();
        m.vm.view_changed(Some(m.items[1]), false, None);
        m.settle();
        let edited = m.docs.edited_any().get();
        let handle = m.editor();
        handle.insert_text("Who rang the bell?");
        m.settle();

        let doc = m.vm.doc().expect("open");
        assert!(doc.dirty.get());
        assert_ne!(m.docs.edited_any().get(), edited);
        assert!(m.docs.flush_all(None).is_empty());
        assert_eq!(
            doc.scratchpad()
                .and_then(|f| f.doc.to_plain_text().ok())
                .unwrap_or_default()
                .trim(),
            "Who rang the bell?"
        );
        assert!(doc.scratchpad_content_id().is_some(), "the row was written");
        let _ = &m.ctx;
    }

    /// Leaving an item and coming back puts the caret and the selection back where the
    /// writer left them, though the editor was rebuilt in between.
    #[test]
    fn the_caret_comes_back_with_the_item() {
        let mut m = mounted();
        let (a, b) = (m.items[0], m.items[1]);
        m.vm.view_changed(Some(a), false, None);
        m.settle();
        let handle = m.editor();
        handle.insert_text("Check the tide tables.");
        handle.select_range(6, 9);
        m.settle();

        m.vm.view_changed(Some(b), false, None);
        m.settle();
        assert_eq!(m.vm.remembered_caret(a), Some((6, 9)));
        let other = m.editor();
        other.insert_text("Bell.");
        other.select_range(2, 2);
        m.settle();

        m.vm.view_changed(Some(a), false, None);
        m.settle();
        assert_eq!(m.editor().selection(), (6, 9), "A's selection is back");
        m.vm.view_changed(Some(b), false, None);
        m.settle();
        assert_eq!(m.editor().selection(), (2, 2), "and B's caret");
    }

    /// A trashed item's scratchpad is read-only, and the panel says why. Restoring the
    /// item makes it writable again.
    #[test]
    fn a_trashed_item_s_scratchpad_is_read_only() {
        crate::test_support::with_real_messages(|| {
            let mut m = mounted();
            m.vm.view_changed(Some(m.items[0]), false, None);
            m.settle();
            assert!(crate::shared::editor_writes::accepts_writes(&m.editor()));

            let doc = m.vm.doc().expect("open");
            doc.trashed.set(true);
            m.settle();
            assert!(!crate::shared::editor_writes::accepts_writes(&m.editor()));
            let said = spoken(&mut m.tree);
            assert!(
                said.iter()
                    .any(|s| s == "This item is in the Trash, so its notes are read-only."),
                "{said:?}"
            );

            doc.trashed.set(false);
            m.settle();
            assert!(crate::shared::editor_writes::accepts_writes(&m.editor()));
        });
    }

    /// The scratchpad's spell checking is awake while the panel shows it, and goes back
    /// to sleep when the panel moves on or goes away.
    #[test]
    fn the_viewer_count_follows_what_the_panel_shows() {
        let mut m = mounted();
        let (a, b) = (m.items[0], m.items[1]);
        m.vm.view_changed(Some(a), false, None);
        m.settle();
        let doc_a = m.vm.doc().expect("A");
        assert_eq!(doc_a.scratchpad_viewers(), 1);

        m.vm.view_changed(Some(b), false, None);
        m.settle();
        let doc_b = m.vm.doc().expect("B");
        assert_eq!(doc_a.scratchpad_viewers(), 0);
        assert_eq!(doc_b.scratchpad_viewers(), 1);

        // The window goes, and the panel with it.
        drop(m);
        assert_eq!(doc_b.scratchpad_viewers(), 0);
    }

    /// A rename reaches the header and what a screen reader is told about the editor,
    /// and the caret stays where it was.
    #[test]
    fn a_rename_reaches_the_header_and_the_description() {
        crate::test_support::with_real_messages(|| {
            let mut m = mounted();
            m.vm.view_changed(Some(m.items[1]), false, None);
            m.settle();
            let handle = m.editor();
            handle.insert_text("Who rang the bell?");
            handle.select_range(4, 8);
            m.settle();

            let mut dto = binder_item_commands::get_binder_item(&m.ctx, &m.items[1])
                .expect("read")
                .expect("the scene");
            dto.title = "The bell".into();
            binder_item_commands::update_binder_item(&m.ctx, None, &dto.into()).expect("rename");
            m.vm.refresh_title();
            m.settle();

            let said = spoken(&mut m.tree);
            assert!(
                said.iter().any(|s| s == "Scratchpad · The bell"),
                "{said:?}"
            );
            assert!(
                said.iter()
                    .any(|s| s == "Your private notes on The bell, never exported or counted"),
                "{said:?}"
            );
            assert_eq!(m.editor().selection(), (4, 8));
        });
    }
}
