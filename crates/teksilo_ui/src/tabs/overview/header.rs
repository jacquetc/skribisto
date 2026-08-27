// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The Overview's header strip: row count, search, expand/collapse all, ＋ New.

#[allow(unused_imports)]
use super::*;

use teksilo::res;
use teksilo::widgets::{IconWidget, SplitButton};

pub(super) fn overview_header(vm: &OverviewViewModel) -> impl Widget {
    teksu!(
        Panel {
            background: SurfaceRole::Raised
            Padding::symmetric(14.0, 8.0) {
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
