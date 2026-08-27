// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The tag filter chip row above the table: the filter half of a promise the
//! Tags column's own doc comment already made (`columns::tags_column`: "finding
//! tagged rows is a filter question, not a sort one"). The column itself stays
//! exactly as it was: display-only, unsortable. This is the surface that answers
//! the filter question instead.
//!
//! **OR, not AND.** Checking two chips shows every row carrying *either* tag:
//! see [`OverviewFilters::tag_filter`]'s own doc for why that reading, not "every
//! checked tag on the same row", is the one a chip row visually promises.
//!
//! The palette is **threaded from the tab**, not read from `ctx.app_state`. It was read
//! that way at first, on [`crate::tags::tag_chip::TagDotsRow`]'s precedent, and that is
//! the hole [`crate::tabs::ContentTab::tags`] was added to close: `app_state` resolves to
//! whichever window's session registered one last, which on the ordinary launcher-first
//! startup path is `startup.rs`'s throwaway `WorkSession` on a fresh, never-seeded
//! `AppIds` - and `app_state` cannot be re-registered afterwards. Its `WorkTagsListModel`
//! is scoped to a `work_id` of `None`, so `rows()` answers empty and this row would never
//! appear at all, however many tags the project has, for the whole session.

#[allow(unused_imports)]
use super::*;

use teksilo::widgets::{Button, Wrap};

use crate::tags::TagsViewModel;

/// The row costs a project with no palette at all nothing: with an empty palette the
/// widget mounts no child and resolves to zero height, **its own padding included** - the
/// same "no chrome for a question nobody can ask yet" discipline the Books column and the
/// Story bible place's own Books chip row both follow. That is why the padding is inside
/// this widget rather than wrapped around it by the pane.
pub(super) fn tag_filter_row(vm: &OverviewViewModel, tags: TagsViewModel) -> Box<dyn Widget> {
    Box::new(TagFilterChips {
        selected: vm.tag_filter_signal(),
        tags,
        root: None,
    })
}

struct TagFilterChips {
    selected: Signal<Vec<u64>>,
    /// This tab's palette, threaded from [`crate::tabs::ContentTab::tags`]. See this
    /// module's own doc for the bug an `app_state` lookup caused here.
    tags: TagsViewModel,
    root: Option<WidgetId>,
}

impl std::fmt::Debug for TagFilterChips {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TagFilterChips").finish()
    }
}

impl Widget for TagFilterChips {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        let tags_vm = self.tags.clone();
        tags_vm.changed_signal().bind_to(
            ctx.self_id(),
            ctx.binding_registry(),
            BindingLevel::Rebuild,
        );
        self.selected
            .bind_to(ctx.self_id(), ctx.binding_registry(), BindingLevel::Rebuild);

        let palette = tags_vm.rows();
        if palette.is_empty() {
            self.root = None;
            return Vec::new();
        }
        let current = self.selected.get();

        let mut row = Wrap::new().spacing(6.0).line_spacing(6.0);
        for t in palette {
            let id = t.id;
            let on = current.contains(&id);
            let selected = self.selected.clone();
            row = row.child(
                Button::new(lit!(t.name.clone()))
                    .variant(if on {
                        ButtonVariant::Tinted
                    } else {
                        ButtonVariant::Plain
                    })
                    .on_activate_fn(move |_c| {
                        let mut next = selected.get();
                        if on {
                            next.retain(|x| *x != id);
                        } else {
                            next.push(id);
                        }
                        selected.set(next);
                    }),
            );
        }
        // The padding belongs **inside** the row, not around it: `Padding` reports
        // `child.height + v_inset`, so an outer one reserved a 16 px blank band above the
        // table on every project with no palette at all, for a control that is not there.
        // Wrapped here instead, where the "no palette, no row" return above is already
        // the whole answer.
        let id = ctx.add(Padding::symmetric(14.0, 8.0).child(row));
        self.root = Some(id);
        vec![id]
    }

    fn layout_response(&self, proposal: SizeProposal, ctx: &LayoutContext) -> LayoutResponse {
        self.root
            .and_then(|id| ctx.child_size(id, proposal))
            .map(LayoutResponse::from)
            .unwrap_or_else(|| proposal.resolve(0.0, 0.0).into())
    }

    fn children(&self) -> Vec<WidgetId> {
        self.root.into_iter().collect()
    }
}

/// Real backend only: the palette this row reads comes back through
/// `WorkTagsListModel`, whose `mocks` arm fabricates a fixed built-in palette rather than
/// answering with the tags a test created (see its own two `mod imp` arms). Gated for the
/// same reason `story_bible_place`'s own palette tests are, and the gate costs no
/// coverage: nothing here is feature-dependent except where the rows come from.
#[cfg(all(test, not(feature = "mocks")))]
mod tests {
    use super::*;

    use std::rc::Rc;

    use frontend::AppContext;
    use frontend::commands::{binder_commands, binder_tag_commands, work_commands};
    use frontend::common::entities::{BinderItemRole, BinderItemSubRole};
    use frontend::direct_access::{CreateBinderDto, CreateBinderTagDto, CreateWorkDto};
    use teksilo::core::widget_tree::WidgetTree;

    use crate::app_ids::AppIds;

    /// A Work with one container to build an Overview against, and `tags` tags in its
    /// palette.
    fn seed(tags: &[&str]) -> (Rc<AppContext>, AppIds, u64) {
        let app_ctx = Rc::new(AppContext::new());
        let work = work_commands::create_orphan_work(&app_ctx, None, &CreateWorkDto::default())
            .expect("create work");
        let binder = binder_commands::create_binder(
            &app_ctx,
            None,
            &CreateBinderDto {
                name: "Manuscript".into(),
                activated: true,
                ..Default::default()
            },
            work.id,
            0,
        )
        .expect("create binder");
        let container = frontend::commands::binder_item_commands::create_binder_item(
            &app_ctx,
            None,
            &frontend::direct_access::CreateBinderItemDto {
                title: "Book One".into(),
                role: BinderItemRole::Folder,
                sub_role: BinderItemSubRole::Book,
                activated: true,
                is_exportable: true,
                ..Default::default()
            },
            binder.id,
            0,
        )
        .expect("create the container");
        let ids = AppIds::new();
        ids.work_id.set(Some(work.id));
        for name in tags {
            let now = chrono::Utc::now();
            binder_tag_commands::create_binder_tag(
                &app_ctx,
                None,
                &CreateBinderTagDto {
                    uid: Default::default(),
                    created_at: now,
                    updated_at: now,
                    name: (*name).to_string(),
                    color: "#2e7d32".to_string(),
                    details: String::new(),
                    discoverable: false,
                    creates_in: None,
                    note_template: None,
                },
                work.id,
                -1,
            )
            .expect("create tag");
        }
        (app_ctx, ids, container.id)
    }

    fn view_model(app_ctx: &Rc<AppContext>, ids: &AppIds, container: u64) -> OverviewViewModel {
        crate::overview::OverviewViewModel::new(
            app_ctx.clone(),
            ids.clone(),
            container,
            &BinderItemRole::Folder,
            &BinderItemSubRole::Book,
            Signal::new(Default::default()),
            crate::settings::TreeExpansionViewModel::new(
                app_ctx.clone(),
                ids.clone(),
                crate::models::TreeExpansionService::in_memory_default(),
            ),
            Signal::new(Default::default()),
        )
        .expect("a Book is overview-capable")
    }

    fn chips(tree: &WidgetTree, id: WidgetId) -> usize {
        let mine = usize::from(
            tree.widget_type_name(id)
                .is_some_and(|n| n.ends_with("::Button")),
        );
        tree.children(id)
            .into_iter()
            .map(|c| chips(tree, c))
            .sum::<usize>()
            + mine
    }

    /// **The palette comes from the tab, not from `app_state`.** Mounted in a tree with
    /// no `app_state` registered at all - which is exactly what the launcher-first
    /// startup path leaves this row looking at, since the throwaway bootstrap session
    /// registers a `TagsViewModel` on a never-seeded `AppIds` and `app_state` cannot be
    /// re-registered afterwards. The row used to come back empty there for the whole
    /// session, however many tags the project had.
    #[test]
    fn the_chip_row_reads_the_tabs_own_palette_rather_than_app_state() {
        let (app_ctx, ids, container) = seed(&["Characters", "Places"]);
        let vm = view_model(&app_ctx, &ids, container);
        let tags = crate::tags::TagsViewModel::detached(app_ctx.clone(), ids.clone());

        let mut tree = crate::test_support::tree_with_events(&app_ctx);
        let root = tree.add_boxed(tag_filter_row(&vm, tags));
        tree.layout(SizeProposal::with_width(600.0));

        assert_eq!(chips(&tree, root), 2, "one chip per tag in the project");
    }

    /// **A project with no palette costs the pane no height.** The padding lives inside
    /// this widget for exactly this: wrapped around it by the pane, it reported its own
    /// insets over a child that is not there, and every untagged project got a permanent
    /// blank band between the Overview header and the table.
    #[test]
    fn a_project_with_no_palette_takes_no_room_at_all() {
        let (app_ctx, ids, container) = seed(&[]);
        let vm = view_model(&app_ctx, &ids, container);
        let tags = crate::tags::TagsViewModel::detached(app_ctx.clone(), ids.clone());

        let mut tree = crate::test_support::tree_with_events(&app_ctx);
        let root = tree.add_boxed(tag_filter_row(&vm, tags));
        tree.layout(SizeProposal::with_width(600.0));

        assert_eq!(
            tree.bounds(root).height,
            0.0,
            "no palette, no row, and no padding around the row either"
        );
    }
}
