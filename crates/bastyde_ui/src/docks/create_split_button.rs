// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The Outline dock header's context-dependent **Create** control: a
//! `SplitButton` whose title (an add icon + the top recommended type, e.g.
//! "＋ Scene"), dropdown, and tooltips all track the current outline selection.
//!
//! The recommendation logic lives in `skribisto_model` +
//! [`OutlineViewModel`]; this widget only
//! renders it. `SplitButton`'s item list is fixed at `build()` time, so — like
//! [`BinderSwitcherButton`](crate::binder::switcher_button::BinderSwitcherButton) —
//! this widget binds `selection_signal()` at `BindingLevel::Rebuild` and
//! reconstructs itself whenever the selection changes. Picking a row fires
//! `AppIntent::NewItem { .., relation }` (the scriptable command surface).

use bastyde::core::BindingLevel;
use bastyde::prelude::*;
use bastyde::res;
use bastyde::widgets::{ButtonVariant, IconWidget, MenuItem, SplitButton};

use crate::binder::create_labels::{
    recommendation_label, recommendation_placement, recommendation_tooltip_key,
};
use crate::intents::AppIntent;
use crate::view_models::OutlineViewModel;

/// The "＋ `<type>`" split button shown in the outline dock header.
pub struct CreateSplitButton {
    outline: OutlineViewModel,
    root_child: Option<WidgetId>,
}

impl CreateSplitButton {
    pub fn new(outline: OutlineViewModel) -> Self {
        Self {
            outline,
            root_child: None,
        }
    }
}

impl std::fmt::Debug for CreateSplitButton {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CreateSplitButton").finish()
    }
}

/// The fixed "add" glyph for the main region (the "＋"). A project asset in the
/// binder icon style; the dropdown rows carry per-type binder icons instead.
fn add_icon() -> IconWidget {
    IconWidget::from_svg_icon(res!("assets/icons/add.svg")).icon_size(14.0)
}

impl Widget for CreateSplitButton {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        // Title / dropdown / tooltips all depend on the selection → full rebuild
        // when it changes (the SplitButton's rows are fixed at build time).
        self.outline.selection_signal().bind_to(
            ctx.self_id(),
            ctx.binding_registry(),
            BindingLevel::Rebuild,
        );

        let anchor = self.outline.selection().selected_keys().first().copied();
        // A title only for a real *item* anchor (a Binder row / no selection is a
        // top-level context → None, which picks the "at the top level" tooltip).
        let anchor_title = anchor
            .and_then(|k| self.outline.node_item(k))
            .and_then(|(item_id, title)| item_id.map(|_| title));
        let recs = self.outline.recommendations_for_selection();

        // `new_static`: the main region stays pinned to index 0 (the current top
        // recommendation). A dropdown pick must NOT promote/replace it — the title
        // has to keep tracking the *selection*, which a create doesn't change.
        let mut btn = SplitButton::new_static()
            .variant(ButtonVariant::Tinted)
            .icon(add_icon());

        for rec in &recs {
            let label = recommendation_label(rec.create_type);
            let key = recommendation_tooltip_key(rec.create_type);
            // Placement now lives inline on the row's trailing slot (always
            // visible), so the rich tooltip is the pure type explainer.
            let placement =
                recommendation_placement(anchor_title.as_deref(), rec.relation).resolve_now();
            let create_type = rec.create_type;
            let relation = rec.relation;
            btn = btn.item(
                MenuItem::new(label)
                    .icon(crate::binder::icons::create_type_icon(rec.create_type))
                    .shortcut_label(placement)
                    .rich_tooltip(key)
                    .on_activate_fn(move |ctx| {
                        ctx.send_intent(AppIntent::NewItem {
                            create_type,
                            relation,
                            // The outline's Create anchors on the current selection.
                            anchor_item_id: None,
                        });
                    }),
            );
        }

        // Main-region tooltip = the default (top) recommendation's type
        // explainer, so it adapts to the selection alongside the title. (No
        // trailing slot on the main button, so placement stays on the rows.)
        if let Some(top) = recs.first() {
            btn = btn.rich_tooltip(recommendation_tooltip_key(top.create_type));
        }

        let id = ctx.add(btn);
        self.root_child = Some(id);
        vec![id]
    }

    fn layout_response(&self, proposal: SizeProposal, ctx: &LayoutContext) -> LayoutResponse {
        self.root_child
            .and_then(|id| ctx.child_size(id, proposal))
            .unwrap_or_else(|| proposal.resolve(0.0, 0.0))
            .into()
    }
}
