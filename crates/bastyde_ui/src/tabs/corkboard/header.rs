// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The corkboard's header strip: breadcrumb, count, nested/flat toggle, ＋ Create.

#[allow(unused_imports)]
use super::*;

// ── Header ────────────────────────────────────────────────────────────────────

pub(super) fn corkboard_header(vm: &CorkboardViewModel) -> impl Widget {
    let top = HStack::new()
        .spacing(10.0)
        .child(Expand::horizontal().child(CorkboardBreadcrumb {
            vm: vm.clone(),
            root: None,
        }))
        .child(CorkboardCount {
            vm: vm.clone(),
            root: None,
        })
        .child(NestedFlatToggle {
            vm: vm.clone(),
            index: Signal::new(0),
            root: None,
        })
        .child(CorkboardCreateButton {
            vm: vm.clone(),
            root: None,
        });

    let bottom = HStack::new()
        .spacing(10.0)
        .child(
            Expand::horizontal().child(
                SearchField::new(vm.search_query_signal())
                    .placeholder(tr!(corkboard_search_placeholder())),
            ),
        )
        .child(TextWidget::new(tr!(corkboard_card_size())).color(TextRole::Secondary))
        .child(
            FixedSize::new().width(170.0).child(
                Slider::new(
                    vm.card_size(),
                    crate::CORKBOARD_CARD_SIZE_MIN,
                    crate::CORKBOARD_CARD_SIZE_MAX,
                )
                .step(crate::CORKBOARD_CARD_SIZE_STEP)
                .label(tr!(corkboard_card_size())),
            ),
        );

    Panel::new().background(SurfaceRole::Raised).child(
        Padding::symmetric(14.0, 8.0).child(VStack::new().spacing(8.0).child(top).child(bottom)),
    )
}

/// The breadcrumb trail — rebuilds when navigation (drill in/out) changes it.
pub(super) struct CorkboardBreadcrumb {
    pub(super) vm: CorkboardViewModel,
    pub(super) root: Option<WidgetId>,
}
impl std::fmt::Debug for CorkboardBreadcrumb {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CorkboardBreadcrumb").finish()
    }
}
impl Widget for CorkboardBreadcrumb {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        self.vm.trail_signal().bind_to(
            ctx.self_id(),
            ctx.binding_registry(),
            BindingLevel::Rebuild,
        );
        // The current crumb's title tracks the container probe, so also rebuild
        // when it resolves/changes.
        self.vm.container_title().bind_to(
            ctx.self_id(),
            ctx.binding_registry(),
            BindingLevel::Rebuild,
        );
        let trail = self.vm.trail_signal().get();
        let current_title = self.vm.container_title().get();
        let last = trail.len().saturating_sub(1);
        let mut bc = Breadcrumb::new().label(tr!(corkboard_grid_label()));
        for (i, (_id, title)) in trail.iter().enumerate() {
            if i == last {
                // The current container — its title comes from the live probe.
                let t = if current_title.is_empty() {
                    title.clone()
                } else {
                    current_title.clone()
                };
                bc = bc.item(BreadcrumbItem::current(lit!(t)));
            } else {
                let vm = self.vm.clone();
                let title = title.clone();
                bc = bc.item(
                    BreadcrumbItem::new(lit!(title)).on_activate_fn(move |_ctx| vm.go_to_crumb(i)),
                );
            }
        }
        let id = ctx.add(bc);
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

/// The "N cards" count — rebuilds when the card set changes.
pub(super) struct CorkboardCount {
    pub(super) vm: CorkboardViewModel,
    pub(super) root: Option<WidgetId>,
}
impl std::fmt::Debug for CorkboardCount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CorkboardCount").finish()
    }
}
impl Widget for CorkboardCount {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        self.vm.count_signal().bind_to(
            ctx.self_id(),
            ctx.binding_registry(),
            BindingLevel::Rebuild,
        );
        let n = self.vm.count_signal().get();
        let id = ctx.add(
            TextWidget::new(tr!(corkboard_card_count(count = n as i64))).color(TextRole::Secondary),
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

/// Nested ⇄ Flat mode toggle — a two-segment control bridged to the VM's `nested`
/// bool (0 = Nested, 1 = Flat) via two guarded effects, the shape `goals_pane` uses.
pub(super) struct NestedFlatToggle {
    pub(super) vm: CorkboardViewModel,
    pub(super) index: Signal<usize>,
    pub(super) root: Option<WidgetId>,
}
impl std::fmt::Debug for NestedFlatToggle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NestedFlatToggle").finish()
    }
}
impl Widget for NestedFlatToggle {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        let nested = self.vm.nested();
        self.index.set(if nested.get() { 0 } else { 1 });
        {
            let index = self.index.clone();
            ctx.effect(&nested, move |n| {
                let i = if *n { 0 } else { 1 };
                if index.get() != i {
                    index.set(i);
                }
            });
        }
        {
            let nested = nested.clone();
            ctx.effect(&self.index, move |i| {
                let n = *i == 0;
                if nested.get() != n {
                    nested.set(n);
                }
            });
        }
        let ctrl = SegmentedControl::new(self.index.clone())
            .segment(Segment::new(tr!(corkboard_view_nested())))
            .segment(Segment::new(tr!(corkboard_view_flat())));
        let id = ctx.add(ctrl);
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

/// The "＋ New" split button: offers the container's recommended child types,
/// anchored on the *current* (drilled-into) container. Mirrors the outline dock's
/// `CreateSplitButton`; rebuilds when the container changes.
pub(super) struct CorkboardCreateButton {
    pub(super) vm: CorkboardViewModel,
    pub(super) root: Option<WidgetId>,
}
impl std::fmt::Debug for CorkboardCreateButton {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CorkboardCreateButton").finish()
    }
}
impl Widget for CorkboardCreateButton {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        // Recommendations depend on the container's (role, sub_role) — rebuild when
        // the container (hence its title) changes.
        self.vm.container_title().bind_to(
            ctx.self_id(),
            ctx.binding_registry(),
            BindingLevel::Rebuild,
        );
        let recs = self.vm.create_recommendations();
        let anchor_title = self.vm.container_title().get();

        let mut btn = SplitButton::new_static()
            .variant(ButtonVariant::Tinted)
            .icon(add_icon());
        for rec in &recs {
            let placement =
                recommendation_placement(Some(anchor_title.as_str()), rec.relation);
            let create_type = rec.create_type;
            let relation = rec.relation;
            let vm = self.vm.clone();
            btn = btn.item(
                MenuItem::new(recommendation_label(rec.create_type))
                    .icon(crate::binder::icons::create_type_icon(rec.create_type))
                    .trailing_hint(placement)
                    .rich_tooltip(recommendation_tooltip_key(rec.create_type))
                    .on_activate_fn(move |ctx| {
                        vm.fire_create(
                            ctx,
                            skribisto_model::Recommendation {
                                create_type,
                                relation,
                            },
                        )
                    }),
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
