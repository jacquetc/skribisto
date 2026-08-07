// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The title-bar **Export** split-button: its primary region names *what the focused item
//! is* — Export Scene / Note / Chapter / Folder / Book — and its dropdown adds the enclosing
//! structural containers.
//!
//! Like [`CreateSplitButton`](crate::docks::create_split_button::CreateSplitButton), the
//! `SplitButton`'s item list is fixed at `build()` time, so this widget binds
//! [`ExportViewModel::applicable_signal`] at [`BindingLevel::Rebuild`] and reconstructs
//! itself whenever the focused item (and thus the applicable scopes) changes. Picking a row —
//! or clicking the primary region, which fires the first row — sends
//! `AppIntent::ExportScoped { scope }` (the scriptable command surface); the `export.scope`
//! action opens the Export panel pre-scoped. Nothing exports on a single click.
//!
//! With no exportable item focused the applicable list is empty and the control degrades to a
//! disabled "Export" button.

use teksilo::core::BindingLevel;
use teksilo::prelude::*;
use teksilo::widgets::{Button, ButtonVariant, MenuItem, SplitButton};

use export_management::ExportScopeKind;

use crate::intents::AppIntent;
use crate::view_models::{ExportViewModel, scope_label};

pub struct ExportSplitButton {
    vm: ExportViewModel,
    root_child: Option<WidgetId>,
}

impl ExportSplitButton {
    pub fn new(vm: ExportViewModel) -> Self {
        Self {
            vm,
            root_child: None,
        }
    }
}

impl std::fmt::Debug for ExportSplitButton {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ExportSplitButton").finish()
    }
}

impl Widget for ExportSplitButton {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        // The label / dropdown depend on the focused item's applicable scopes → full rebuild
        // when they change (the SplitButton's rows are fixed at build time).
        self.vm.applicable_signal().bind_to(
            ctx.self_id(),
            ctx.binding_registry(),
            BindingLevel::Rebuild,
        );

        let scopes = self.vm.applicable_signal().get();
        let id = if scopes.is_empty() {
            // No project open — a disabled affordance, not a dead-end.
            ctx.add(
                Button::new(tr!(export_title()))
                    .variant(ButtonVariant::Tinted)
                    .enabled(false),
            )
        } else if scopes.as_slice() == [ExportScopeKind::Custom] {
            // A project is open but nothing exportable is focused: no quick scope applies, so
            // the primary is a plain "Export" that opens the Choose… picker (reads as an
            // export action, not a bare "Choose…").
            ctx.add(
                Button::new(tr!(export_title()))
                    .variant(ButtonVariant::Tinted)
                    .on_activate_fn(|ctx| {
                        ctx.send_intent(AppIntent::ExportScoped {
                            scope: ExportScopeKind::Custom,
                        });
                    }),
            )
        } else {
            // `new_static`: the primary region stays pinned to index 0 (the focused item's own
            // facet). A dropdown pick must NOT promote/replace it — the label tracks the
            // *selection*, which opening the panel does not change.
            let mut btn = SplitButton::new_static().variant(ButtonVariant::Tinted);
            for scope in &scopes {
                let s = scope.clone();
                btn = btn.item(
                    MenuItem::new(scope_label(scope)).on_activate_fn(move |ctx| {
                        ctx.send_intent(AppIntent::ExportScoped { scope: s.clone() });
                    }),
                );
            }
            ctx.add(btn)
        };
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tabs::shared::editor::VisibleWhen;
    use teksilo::core::widget_tree::WidgetTree;

    /// A `SplitButton` pre-builds its dropdown as a child parked with
    /// `ctx.set_dormant` and shows it through an overlay on demand. Parking an
    /// **ancestor** dormant and waking it again must not wake that dropdown.
    ///
    /// It did: the distraction-free surface parks the whole project shell,
    /// including the title bar, so leaving the mode left the Export button's
    /// menu-item labels rendered inline underneath it — text with no popup
    /// behind it, because the overlay presentation never ran.
    ///
    /// Raw `SplitButton`, not `ExportSplitButton`: the behaviour under test
    /// belongs to the framework, and naming it here is what keeps the fix from
    /// being mistaken for something about export.
    #[test]
    fn a_closed_dropdown_survives_an_ancestors_dormancy_cycle() {
        let gate = Signal::new(true);
        let mut tree = WidgetTree::new();
        tree.add(VisibleWhen::new(
            gate.clone(),
            SplitButton::new_static()
                .item(MenuItem::new(lit!("Export Book")))
                .item(MenuItem::new(lit!("Export Chapter"))),
        ));

        // The a11y walk skips dormant nodes, so it answers exactly the question
        // that matters: is the closed dropdown on screen?
        let menu_is_showing = |tree: &mut WidgetTree| {
            tree.layout(SizeProposal::exact(400.0, 200.0));
            tree.sync_accessibility()
                .nodes
                .iter()
                .filter_map(|(_, n)| n.label().map(|s| s.to_string()))
                .any(|l| l == "Export Chapter")
        };

        assert!(
            !menu_is_showing(&mut tree),
            "the dropdown is showing before anything was even hidden"
        );

        gate.set(false);
        assert!(!menu_is_showing(&mut tree), "hidden: nothing should show");

        gate.set(true);
        assert!(
            !menu_is_showing(&mut tree),
            "waking the ancestor also woke the closed dropdown — its items are \
             now rendered inline under the button"
        );
    }
}
