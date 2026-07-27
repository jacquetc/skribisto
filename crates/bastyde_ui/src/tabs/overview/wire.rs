// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The pane's wiring child and the per-row context menu.

#[allow(unused_imports)]
use super::*;

use uuid::Uuid;

/// Zero-size child that wires the view-model (model subscriptions, the container probe,
/// the reorder commit, the header count) on build. `wire` is idempotent per build.
pub(super) struct WireOverview {
    pub(super) vm: OverviewViewModel,
}
impl std::fmt::Debug for WireOverview {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WireOverview").finish()
    }
}
impl Widget for WireOverview {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        self.vm.wire(ctx);
        Vec::new()
    }
    fn layout_response(&self, _p: SizeProposal, _c: &LayoutContext) -> LayoutResponse {
        Size::new(0.0, 0.0).into()
    }
}

/// The per-row context menu: open / open to the side · reveal in outline · Add ▸ ·
/// Convert to ▸ · rename · duplicate · move up/down · trash.
///
/// Every batch action ("duplicate", "trash") resolves its targets through
/// [`OverviewViewModel::batch_for`], i.e. against **this table's** selection — a
/// right-click inside the selection acts on all of it, one outside acts on that row
/// alone. The mutations themselves are the same backend calls the outline and the
/// corkboard make (shared in `binder_ops`), so there is one implementation per verb; what
/// is local is only *which rows* it applies to.
pub(super) fn overview_context_menu(
    vm: OverviewViewModel,
    uid: Uuid,
    row: OverviewRow,
) -> MenuList {
    let batch = vm.batch_for(uid);

    let open_title = row.title.clone();
    let open_id = row.item_id;
    let side = vm.clone();
    let reveal = vm.clone();
    let add_vm = vm.clone();
    let rename = vm.clone();
    let dup = vm.clone();
    let dup_batch = batch.clone();
    let up = vm.clone();
    let down = vm.clone();
    let trash = vm.clone();
    let trash_batch = batch;

    let mut menu = MenuList::new()
        .item(MenuItem::new(tr!(ctx_open())).on_activate_fn(move |ctx| {
            ctx.send_intent(crate::intents::AppIntent::OpenItem {
                item_id: open_id,
                title: open_title.clone(),
            })
        }))
        .item(
            MenuItem::new(tr!(ctx_open_to_side()))
                .on_activate_fn(move |ctx| side.open_to_side(ctx, uid)),
        )
        .separator()
        .item(
            MenuItem::new(tr!(ctx_reveal_in_outline()))
                .on_activate_fn(move |ctx| reveal.reveal_in_outline(ctx, uid)),
        )
        .separator()
        // Context-dependent "Add ▸": the types the writing model recommends *inside*
        // this row, anchored on it — never on the outline's selection.
        .item(MenuItem::submenu(tr!(ctx_add()), move || {
            Box::new(add_recommendations_menu(add_vm.clone(), uid)) as Box<dyn Widget>
        }));

    // "Convert to ▸" — every type this row may become, behind the shared guards.
    let targets = vm.promote_targets_of(uid);
    if !targets.is_empty() {
        let promote_vm = vm.clone();
        menu = menu
            .separator()
            .item(MenuItem::submenu(tr!(ctx_promote()), move || {
                Box::new(promote_menu(promote_vm.clone(), uid)) as Box<dyn Widget>
            }));
    }

    menu = menu.separator().item(
        MenuItem::new(tr!(ctx_rename()))
            .on_activate_fn(move |_| rename.begin_edit(uid, crate::models::COL_TITLE)),
    );
    menu = menu.item(
        MenuItem::new(tr!(ctx_duplicate())).on_activate_fn(move |_| dup.duplicate(&dup_batch)),
    );

    // Move up / down are omitted entirely while the table is projecting: under a sort or
    // a search the row's "neighbour" is a projected one, so the move would write an order
    // the writer never chose. Hiding beats greying — there is nothing to re-enable here
    // except by clearing the box, which the writer can already see.
    if vm.can_reorder() {
        menu = menu
            .separator()
            .item(MenuItem::new(tr!(ctx_move_up())).on_activate_fn(move |_| up.move_up(uid)))
            .item(MenuItem::new(tr!(ctx_move_down())).on_activate_fn(move |_| down.move_down(uid)));
    }

    menu.separator()
        .item(MenuItem::new(tr!(ctx_trash())).on_activate_fn(move |_| trash.trash(&trash_batch)))
}

/// The "Add ▸" submenu: the recommended child types for this row, each with the shared
/// rich tooltip and the "where it lands" hint — identical to the outline's and the
/// corkboard's, because it is the same vocabulary from the same model.
fn add_recommendations_menu(vm: OverviewViewModel, uid: Uuid) -> MenuList {
    let recs = vm.create_recommendations(Some(uid));
    let anchor_title = vm.row_of(&uid).map(|r| r.title).unwrap_or_default();
    let mut menu = MenuList::new();
    for rec in &recs {
        let vm = vm.clone();
        let rec_owned = *rec;
        let placement = recommendation_placement(Some(anchor_title.as_str()), rec.relation);
        menu = menu.item(
            MenuItem::new(recommendation_label(rec.create_type))
                .icon(crate::binder::icons::create_type_icon(rec.create_type))
                .trailing_hint(placement)
                .rich_tooltip(recommendation_tooltip_key(rec.create_type))
                .on_activate_fn(move |ctx| vm.fire_create(ctx, rec_owned, Some(uid))),
        );
    }
    menu
}

/// The "Convert to ▸" submenu, behind [`OverviewViewModel::promote_with_guard`] — the
/// same two refusals (a container becoming a leaf must be empty; the target must have
/// somewhere to keep the text) the outline applies, from the same shared implementation.
fn promote_menu(vm: OverviewViewModel, uid: Uuid) -> MenuList {
    let mut menu = MenuList::new();
    for target in vm.promote_targets_of(uid) {
        let (_, sub_role) = target.combo();
        let vm = vm.clone();
        menu = menu.item(
            MenuItem::new(crate::binder::create_labels::promote_target_label(target))
                .icon(crate::binder::icons::sub_role_icon(&sub_role))
                .rich_tooltip(crate::binder::create_labels::promote_target_tooltip_key(
                    target,
                ))
                .on_activate_fn(move |ctx| vm.promote_with_guard(ctx, uid, target)),
        );
    }
    menu
}
