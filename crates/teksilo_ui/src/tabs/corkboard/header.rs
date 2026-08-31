// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The corkboard's header strip: breadcrumb, count, nested/flat toggle, ＋ Create.

#[allow(unused_imports)]
use super::*;

// ── Header ────────────────────────────────────────────────────────────────────

pub(super) fn corkboard_header(vm: &CorkboardViewModel) -> impl Widget {
    // The four header widgets are plain Rust struct literals, so each one is
    // parenthesised: a bare `Name { .. }` at body position is a `teksu!` *element*,
    // which would lower to `Name::new().vm(..)` instead of the struct.
    let top = teksu!(
        HStack {
            spacing: 10.0
            Expand::horizontal {
                child: (CorkboardBreadcrumb {
                    vm: vm.clone(),
                    root: None,
                })
            }
            child: (CorkboardCount {
                vm: vm.clone(),
                root: None,
            })
            child: (NestedFlatToggle {
                vm: vm.clone(),
                index: Signal::new(0),
                root: None,
            })
            child: (CorkboardCreateButton {
                vm: vm.clone(),
                root: None,
            })
        }
    );

    let bottom = teksu!(
        HStack {
            spacing: 10.0
            Expand::horizontal {
                SearchField::new(vm.search_query_signal()) {
                    placeholder: tr!(corkboard_search_placeholder())
                }
            }
            child: (SortControl {
                vm: vm.clone(),
                root: None,
            })
            TextWidget::new(tr!(corkboard_card_size())) {
                color: TextRole::Secondary
            }
            FixedSize::new() {
                width: 170.0
                Slider::new(vm.card_size(), crate::CORKBOARD_CARD_SIZE_MIN, crate::CORKBOARD_CARD_SIZE_MAX) {
                    step: crate::CORKBOARD_CARD_SIZE_STEP
                    label: tr!(corkboard_card_size())
                    // The slider writes an app-global setting, but sits inside one
                    // tab's header — say so, or dragging it silently resizes every
                    // other open board too.
                    tooltip: tr!(corkboard_scope_hint())
                }
            }
        }
    );

    teksu!(
        Panel {
            background: SurfaceRole::Raised
            Padding::symmetric(14.0, 8.0) {
                VStack {
                    spacing: 8.0
                    child: top
                    child: bottom
                }
            }
        }
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
        // The scope hint sits on each `Segment` — `tooltip` is a per-segment
        // affordance, not a property of the control as a whole.
        let ctrl = SegmentedControl::indexed(self.index.clone())
            .segment(
                Segment::new(tr!(corkboard_view_nested())).tooltip(tr!(corkboard_scope_hint())),
            )
            .segment(Segment::new(tr!(corkboard_view_flat())).tooltip(tr!(corkboard_scope_hint())));
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

/// The board's order: manuscript order, or a title sort.
///
/// Binds the view-model's own `sort` signal **directly** — `ComboBox` takes its
/// `selected` handle as a constructor argument, so there is no widget-owned signal
/// to bridge (the `TreeTableView` dance in `tabs/overview/table.rs` exists only
/// because that widget allocates its own). `None` *is* manuscript order, which is
/// what `CorkboardViewModel::wire` already reads as `clear_sort()` — so the
/// placeholder names that state rather than a fourth sentinel item.
pub(super) struct SortControl {
    pub(super) vm: CorkboardViewModel,
    pub(super) root: Option<WidgetId>,
}
impl std::fmt::Debug for SortControl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SortControl").finish()
    }
}
impl Widget for SortControl {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        let title = || crate::corkboard::SORT_TITLE.to_string();
        // Each button shows the order currently in force and, on activate, moves to
        // the next one: manuscript → A–Z → Z–A → manuscript. `None` *is* manuscript
        // order (what `wire`'s effect reads as `clear_sort`), so there is no fourth
        // sentinel state.
        type Order = Option<(String, SortDirection)>;
        let button =
            |icon: IconWidget, tip: LocalizedString, next: Order, vm: CorkboardViewModel| {
                IconButton::new(icon)
                    .toolbar()
                    .tooltip(tip)
                    // `Fn`, not `FnOnce` — clone the target order per invocation
                    // rather than moving the captured one out.
                    .on_activate_fn(move |_ctx| vm.sort_signal().set(next.clone()))
            };
        let switcher = Switcher::new(self.vm.sort_signal().map(|s| match s {
            None => 0usize,
            Some((_, SortDirection::Ascending)) => 1,
            Some((_, SortDirection::Descending)) => 2,
        }))
        .child(button(
            crate::icons::corkboard::sort_manuscript_icon(),
            tr!(corkboard_sort_manuscript()),
            Some((title(), SortDirection::Ascending)),
            self.vm.clone(),
        ))
        .child(button(
            crate::icons::corkboard::sort_title_asc_icon(),
            tr!(corkboard_sort_title_asc()),
            Some((title(), SortDirection::Descending)),
            self.vm.clone(),
        ))
        .child(button(
            crate::icons::corkboard::sort_title_desc_icon(),
            tr!(corkboard_sort_title_desc()),
            None,
            self.vm.clone(),
        ));
        let id = ctx.add(switcher);
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
            let placement = recommendation_placement(Some(anchor_title.as_str()), rec.relation);
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
