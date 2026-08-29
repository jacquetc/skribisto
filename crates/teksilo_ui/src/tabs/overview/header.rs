// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The Overview's header strip: row count, search, expand/collapse all, ＋ New.

#[allow(unused_imports)]
use super::*;

use teksilo::res;
use teksilo::widgets::{IconWidget, SplitButton};

pub(super) fn overview_header(
    vm: &OverviewViewModel,
    tags: crate::tags::TagsViewModel,
    statuses: crate::statuses::StatusesViewModel,
) -> impl Widget {
    teksu!(
        Panel {
            background: SurfaceRole::Raised
            Padding::symmetric(14.0, 8.0) {
                // The filter chips live *under* the search field, inside the same banner:
                // they answer the same question it does — which rows am I looking at —
                // and below the pane they read as content, a band of buttons sitting on
                // top of the table they are not part of.
                //
                // `spacing: 0.0` on purpose. Each chip row carries its own vertical
                // padding and mounts nothing at all when its vocabulary is empty, so a
                // project with no tags pays no height for the row *and* no gap for it.
                // A `VStack` spacing would have reserved the gap either way.
                VStack {
                    spacing: 0.0
                    HStack {
                        spacing: 10.0
                        Expand::horizontal {
                            SearchField::new(vm.search_query_signal()) {
                                placeholder: tr!(overview_search_placeholder())
                            }
                        }
                        // Parenthesised: a bare `Name { .. }` at body position is a
                        // `teksu!` element, not a Rust struct literal.
                        child: (OverviewCount {
                            vm: vm.clone(),
                            root: None,
                        })
                        child: expand_collapse_buttons(vm)
                        child: (OverviewCreateButton {
                            vm: vm.clone(),
                            root: None,
                        })
                    }
                    child: crate::tabs::Boxed::new(super::tag_filter::tag_filter_row(vm, tags))
                    // Beside the tag row, not merged with it: they are different
                    // questions on different axes, and a single row of mixed chips would
                    // read as one set.
                    child: crate::tabs::Boxed::new(super::status_filter::status_filter_row(vm, statuses))
                }
            }
        }
    )
}

/// Expand-all / collapse-all. A book's outline is the one place where "show me
/// everything" and "show me only the shape" are both routine, and clicking twist by
/// twist through forty chapters is neither.
fn expand_collapse_buttons(vm: &OverviewViewModel) -> impl Widget {
    let expand = vm.clone();
    let collapse = vm.clone();
    teksu!(
        HStack {
            spacing: 2.0
            // `tooltip` is also the button's accessible name (an `IconButton` has no
            // separate label — see the framework's own `a11y_builtin_*` helpers), so
            // these are named for a screen reader by the same call.
            IconButton::new(IconWidget::from_svg_icon(res!(
                "assets/icons/expand-all.svg"
            ))) {
                tooltip: tr!(overview_expand_all())
                on_activate_fn: move |_| expand.expand_all()
            }
            IconButton::new(IconWidget::from_svg_icon(res!(
                "assets/icons/collapse-all.svg"
            ))) {
                tooltip: tr!(overview_collapse_all())
                on_activate_fn: move |_| collapse.collapse_all()
            }
        }
    )
}

/// The "N rows" count — follows the *visible* set, so it reflects an active search.
pub(super) struct OverviewCount {
    pub(super) vm: OverviewViewModel,
    pub(super) root: Option<WidgetId>,
}
impl std::fmt::Debug for OverviewCount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("OverviewCount").finish()
    }
}
impl Widget for OverviewCount {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        self.vm.count_signal().bind_to(
            ctx.self_id(),
            ctx.binding_registry(),
            BindingLevel::Rebuild,
        );
        let n = self.vm.count_signal().get();
        let id = ctx.add(
            TextWidget::new(tr!(overview_row_count(count = n as i64))).color(TextRole::Secondary),
        );
        self.root = Some(id);
        vec![id]
    }
    fn layout_response(&self, p: SizeProposal, ctx: &LayoutContext) -> LayoutResponse {
        self.root
            .and_then(|id| ctx.child_size(id, p))
            .unwrap_or_else(|| p.resolve(0.0, 0.0))
            .into()
    }
    fn children(&self) -> Vec<WidgetId> {
        self.root.into_iter().collect()
    }
}

/// The "＋ New" split button: the container's recommended child types, anchored on the
/// container itself (not on any row, and not on the outline's selection). Mirrors the
/// corkboard header's button; rebuilds when the container resolves or changes.
pub(super) struct OverviewCreateButton {
    pub(super) vm: OverviewViewModel,
    pub(super) root: Option<WidgetId>,
}
impl std::fmt::Debug for OverviewCreateButton {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("OverviewCreateButton").finish()
    }
}
impl Widget for OverviewCreateButton {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        // The offers depend on the container's `(role, sub_role)`; its title resolving is
        // the signal that the probe has loaded it.
        self.vm.container_title().bind_to(
            ctx.self_id(),
            ctx.binding_registry(),
            BindingLevel::Rebuild,
        );
        let recs = self.vm.create_recommendations(None);
        let anchor_title = self.vm.container_title().get();

        let mut btn = SplitButton::new_static()
            .variant(ButtonVariant::Tinted)
            .icon(IconWidget::from_svg_icon(res!("assets/icons/add.svg")).icon_size(14.0));
        for rec in &recs {
            let placement = recommendation_placement(Some(anchor_title.as_str()), rec.relation);
            let rec_owned = *rec;
            let vm = self.vm.clone();
            btn = btn.item(
                MenuItem::new(recommendation_label(rec.create_type))
                    .icon(crate::binder::icons::create_type_icon(rec.create_type))
                    .trailing_hint(placement)
                    .rich_tooltip(recommendation_tooltip_key(rec.create_type))
                    .on_activate_fn(move |ctx| vm.fire_create(ctx, rec_owned, None)),
            );
        }
        if let Some(top) = recs.first() {
            btn = btn.rich_tooltip(recommendation_tooltip_key(top.create_type));
        }
        let id = ctx.add(btn);
        self.root = Some(id);
        vec![id]
    }
    fn layout_response(&self, p: SizeProposal, ctx: &LayoutContext) -> LayoutResponse {
        self.root
            .and_then(|id| ctx.child_size(id, p))
            .unwrap_or_else(|| p.resolve(0.0, 0.0))
            .into()
    }
    fn children(&self) -> Vec<WidgetId> {
        self.root.into_iter().collect()
    }
}

/// The empty state.
///
/// Two of them, because "nothing in here" and "this is gone" are different facts. A
/// container that was trashed out from under an open tab must NOT offer "＋ New": the
/// create would be anchored on an item `binder_ops::locate` can no longer find, so the
/// button would silently do nothing. It says what happened instead.
pub(super) struct OverviewEmpty {
    pub(super) vm: OverviewViewModel,
    pub(super) root: Option<WidgetId>,
}
impl std::fmt::Debug for OverviewEmpty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("OverviewEmpty").finish()
    }
}
impl Widget for OverviewEmpty {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        self.vm.container_present().bind_to(
            ctx.self_id(),
            ctx.binding_registry(),
            BindingLevel::Rebuild,
        );
        let col = if self.vm.container_present().get() {
            VStack::new()
                .spacing(10.0)
                .child(
                    TextWidget::new(tr!(overview_empty_title()))
                        .style(TextStyleRole::BodyBold)
                        .color(TextRole::Secondary),
                )
                .child(TextWidget::new(tr!(overview_empty_hint())).color(TextRole::Secondary))
                .child(OverviewCreateButton {
                    vm: self.vm.clone(),
                    root: None,
                })
        } else {
            // No create button — see the type docs.
            VStack::new()
                .spacing(10.0)
                .child(
                    TextWidget::new(tr!(overview_gone_title()))
                        .style(TextStyleRole::BodyBold)
                        .color(TextRole::Secondary),
                )
                .child(TextWidget::new(tr!(overview_gone_hint())).color(TextRole::Secondary))
        };
        let id = ctx.add(Center::new().child(col));
        self.root = Some(id);
        vec![id]
    }
    fn layout_response(&self, p: SizeProposal, ctx: &LayoutContext) -> LayoutResponse {
        self.root
            .and_then(|id| ctx.child_size(id, p))
            .unwrap_or_else(|| p.resolve(0.0, 0.0))
            .into()
    }
    fn children(&self) -> Vec<WidgetId> {
        self.root.into_iter().collect()
    }
}

/// The banner's own shape: the filter chips live *inside* it, under the search field.
///
/// Real backend only, for the reason `tag_filter`'s tests record: the mock
/// `WorkTagsListModel` fabricates a fixed palette instead of answering with the tags a
/// test created.
#[cfg(all(test, not(feature = "mocks")))]
mod tests {
    use super::*;

    use std::rc::Rc;

    use frontend::AppContext;
    use frontend::commands::{binder_commands, binder_status_commands, binder_tag_commands};
    use frontend::commands::{binder_item_commands, work_commands};
    use frontend::common::entities::{BinderItemRole, BinderItemSubRole, StatusCategory};
    use frontend::direct_access::{
        CreateBinderDto, CreateBinderItemDto, CreateBinderStatusDto, CreateBinderTagDto,
        CreateWorkDto,
    };
    use teksilo::core::widget_tree::WidgetTree;

    use crate::app_ids::AppIds;

    /// A Work with one Book, one tag and one status rung — enough for both chip rows to
    /// have something to show.
    fn seed() -> (Rc<AppContext>, AppIds, u64) {
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
        let container = binder_item_commands::create_binder_item(
            &app_ctx,
            None,
            &CreateBinderItemDto {
                status: None,
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
        let now = chrono::Utc::now();
        binder_tag_commands::create_binder_tag(
            &app_ctx,
            None,
            &CreateBinderTagDto {
                uid: Default::default(),
                created_at: now,
                updated_at: now,
                name: "Characters".into(),
                color: "#2e7d32".into(),
                details: String::new(),
                discoverable: false,
                creates_in: None,
                note_template: None,
            },
            work.id,
            -1,
        )
        .expect("create tag");
        binder_status_commands::create_binder_status(
            &app_ctx,
            None,
            &CreateBinderStatusDto {
                uid: Default::default(),
                created_at: now,
                updated_at: now,
                name: "Draft".into(),
                category: StatusCategory::Drafting,
                details: String::new(),
            },
            work.id,
            -1,
        )
        .expect("create rung");
        let ids = AppIds::new();
        ids.work_id.set(Some(work.id));
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

    /// Every node whose type name contains `needle`, as `(id, top edge)`.
    fn find(tree: &WidgetTree, id: WidgetId, needle: &str, out: &mut Vec<(WidgetId, f32)>) {
        if tree
            .widget_type_name(id)
            .is_some_and(|n| n.contains(needle))
        {
            out.push((id, tree.bounds(id).y));
        }
        for c in tree.children(id) {
            find(tree, c, needle, out);
        }
    }

    /// **The chips are in the banner, under the search field — not a band above the
    /// table.**
    ///
    /// They answer the same question the search field does — which rows am I looking at —
    /// and below the pane they read as content, a strip of buttons sitting on top of a
    /// table they are not part of. Asserted by geometry rather than by structure, because
    /// what is being promised here is where a reader's eye finds them.
    #[test]
    fn the_filter_chips_sit_inside_the_banner_below_the_search_field() {
        let (app_ctx, ids, container) = seed();
        let vm = view_model(&app_ctx, &ids, container);
        let tags = crate::tags::TagsViewModel::detached(app_ctx.clone(), ids.clone());
        let statuses = crate::statuses::StatusesViewModel::new(app_ctx.clone(), ids.clone());

        let mut tree = crate::test_support::tree_with_events(&app_ctx);
        let root = tree.add(overview_header(&vm, tags, statuses));
        tree.layout(SizeProposal::with_width(900.0));

        let mut search = Vec::new();
        find(&tree, root, "SearchField", &mut search);
        let mut chips = Vec::new();
        find(&tree, root, "button::Button", &mut chips);

        assert!(!search.is_empty(), "the banner lost its search field");
        assert!(
            chips.len() >= 2,
            "expected a tag chip and a status chip, got {}",
            chips.len()
        );

        let search_top = search[0].1;
        let banner = tree.bounds(root);
        for (id, top) in &chips {
            assert!(
                *top > search_top,
                "a filter chip sits at or above the search field ({top} vs {search_top})"
            );
            let b = tree.bounds(*id);
            assert!(
                b.y >= banner.y && b.y + b.height <= banner.y + banner.height,
                "a filter chip escaped the banner: {b:?} against {banner:?}"
            );
        }
    }
}
