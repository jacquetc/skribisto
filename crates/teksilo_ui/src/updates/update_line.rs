// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! `UpdateLine` — the whole visible surface of the update check.
//!
//! One line, and only when there is something to say. It is mounted wherever the
//! application already prints its own version (the Launcher sidebar and the
//! About box), because that is where a reader who is thinking about versions is
//! already looking, and nowhere else.
//!
//! ## Why it is a widget rather than a row each caller builds
//!
//! The fact arrives asynchronously and can arrive while both surfaces are open,
//! so the row has to appear and disappear on its own. Binding
//! [`crate::updates::Available`] at [`BindingLevel::Rebuild`] and reconstructing
//! is the same shape `crate::statusbar::notification_bell` uses for the same
//! reason, and it keeps the reactivity in one place instead of two.
//!
//! Building **nothing** when there is nothing to say is the important half. The
//! row is absent, not empty and not hidden: a launcher sidebar that reserves
//! space for a message it is not showing has spent the pixel anyway.
//!
//! ## Why a button and not a label with a link after it
//!
//! The whole line is the target. A reader who has just been told a newer version
//! exists wants the page, and a separate "download" affordance beside the
//! sentence is one more thing to aim at for no added meaning. It carries
//! `Role::Link` so a screen reader announces it as what it is, and the tooltip
//! says where it goes, because a link that does not say it opens a browser is a
//! small ambush.

use teksilo::canvas::EdgeInsets;
use teksilo::core::accesskit::Role;
use teksilo::core::binding::BindingLevel;
use teksilo::prelude::*;
use teksilo::widgets::styles::RecipeButtonStyle;
use teksilo::widgets::{Button, ButtonVariant, TextWidget};

use crate::updates::UpdateViewModel;

/// A one-line "a newer version is available" link, or nothing at all.
pub struct UpdateLine {
    vm: UpdateViewModel,
    root_child: Option<WidgetId>,
}

impl UpdateLine {
    pub fn new(vm: UpdateViewModel) -> Self {
        Self {
            vm,
            root_child: None,
        }
    }
}

impl std::fmt::Debug for UpdateLine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("UpdateLine").finish()
    }
}

impl Widget for UpdateLine {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        let available = self.vm.available();
        available.bind_to(ctx.self_id(), ctx.binding_registry(), BindingLevel::Rebuild);

        self.root_child = None;

        // Nothing to say: build nothing. Not a hidden row, not a zero-height
        // spacer — the surfaces above and below close up as though the feature
        // did not exist, which for a reader who is up to date it does not.
        let Some(available) = available.get() else {
            return Vec::new();
        };

        // A feed that named a version but published no link. The sentence is
        // still true and still useful, so it is still shown; it simply is not a
        // link, because there is nowhere honest to send the reader.
        let Some(target) = available
            .download_url
            .clone()
            .or_else(|| available.notes_url.clone())
        else {
            let id = ctx.add(
                TextWidget::new(tr!(welcome_update_available(version = available.version)))
                    .style(TextStyleRole::Small)
                    .color(TextRole::Secondary),
            );
            self.root_child = Some(id);
            return vec![id];
        };

        let button = Button::new(tr!(welcome_update_available(
            version = available.version.clone()
        )))
        .variant(ButtonVariant::Ghost)
        .style(flush_link_style())
        // Same weight as the version line it sits under, so the pair reads as one
        // block of version information rather than as a control that has been
        // dropped into the brand mark.
        .text_style(TextStyleRole::Small)
        .text_role(TextRole::Secondary)
        .tooltip(tr!(welcome_update_tip()))
        .on_activate_fn(move |c| {
            crate::shared::external_link::open_external_link(&target, c);
        })
        // Last: this wraps the Button, so every Button-specific call is above it.
        .access_role(Role::Link);

        let id = ctx.add(button);
        self.root_child = Some(id);
        vec![id]
    }

    fn layout_response(&self, proposal: SizeProposal, ctx: &LayoutContext) -> LayoutResponse {
        self.root_child
            .and_then(|id| ctx.child_size(id, proposal))
            .map(LayoutResponse::from)
            // No child means nothing to say, and nothing to say must cost no
            // height: the sidebar's `VStack` would otherwise keep its spacing
            // around an invisible row.
            .unwrap_or_else(|| LayoutResponse::from(Size::new(0.0, 0.0)))
    }
}

/// A Ghost button with no padding of its own, so its text starts on the same
/// vertical line as the plain label above it.
///
/// The recipe is edited rather than written from scratch, exactly as the
/// launcher's own `flat_icon_button_style` does it: the fill, border and
/// focus-ring states, and any theme that reshapes them, still apply. Only the
/// inset that would push the text out of alignment is removed.
fn flush_link_style() -> RecipeButtonStyle {
    let mut style = RecipeButtonStyle::intui();
    if let Some(ghost) = style.recipes.get_mut(&ButtonVariant::Ghost) {
        // `EdgeInsets::symmetric` takes (horizontal, vertical). Zero horizontally
        // so the text is flush; a little vertically so the hit target is still
        // comfortably taller than the glyphs.
        ghost.padding = EdgeInsets::symmetric(0.0, 3.0);
        ghost.min_size = Size::new(0.0, 0.0);
    }
    style
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::updates_file::UpdatesService;
    use crate::updates::UpdateViewModel;
    use std::collections::BTreeMap;
    use teksilo::core::widget_tree::WidgetTree;
    use teksilo::presets::intui;

    fn links() -> BTreeMap<String, String> {
        BTreeMap::from([(
            "en".to_string(),
            "https://www.skribisto.eu/download/".to_string(),
        )])
    }

    fn vm_with(latest: &str, links: BTreeMap<String, String>) -> UpdateViewModel {
        let store = UpdatesService::in_memory_default();
        if !latest.is_empty() {
            store.record(latest, "2026-12-01", links.clone(), links);
        }
        UpdateViewModel::new(store)
    }

    #[test]
    fn an_up_to_date_reader_is_shown_nothing_at_all() {
        let vm = vm_with("", BTreeMap::new());
        let mut tree = WidgetTree::new().with_theme(intui::light());
        tree.add(UpdateLine::new(vm));
        tree.layout(SizeProposal::exact(240.0, 60.0));
        assert!(
            tree.find_by_label("Version 999.0.0 is available").is_none(),
            "nothing to say means no row"
        );
    }

    #[test]
    fn a_newer_version_is_named_in_one_line() {
        let vm = vm_with("999.0.0", links());
        let mut tree = WidgetTree::new().with_theme(intui::light());
        tree.add(UpdateLine::new(vm));
        tree.layout(SizeProposal::exact(240.0, 60.0));
        assert!(
            tree.find_by_label("Version 999.0.0 is available").is_some(),
            "the version has to be in the label, not just in a tooltip"
        );
    }

    /// The row has to appear on its own: the check completes long after both
    /// surfaces were built, and a reader looking at the Launcher when it lands
    /// must see it without reopening anything.
    #[test]
    fn the_row_appears_when_a_check_lands_while_the_surface_is_open() {
        let store = UpdatesService::in_memory_default();
        let vm = UpdateViewModel::new(store.clone());
        let mut tree = WidgetTree::new().with_theme(intui::light());
        tree.add(UpdateLine::new(vm.clone()));
        tree.layout(SizeProposal::exact(240.0, 60.0));
        assert!(tree.find_by_label("Version 999.0.0 is available").is_none());

        store.record("999.0.0", "2026-12-01", links(), links());
        vm.reload_from_store();
        tree.layout(SizeProposal::exact(240.0, 60.0));
        assert!(
            tree.find_by_label("Version 999.0.0 is available").is_some(),
            "the Rebuild-level binding must reconstruct the row"
        );
    }

    /// And disappear again, which is what happens the moment the reader updates
    /// and relaunches into a build the stored version is no longer newer than.
    #[test]
    fn the_row_goes_away_once_there_is_nothing_to_say() {
        let store = UpdatesService::in_memory_default();
        store.record("999.0.0", "2026-12-01", links(), links());
        let vm = UpdateViewModel::new(store.clone());
        let mut tree = WidgetTree::new().with_theme(intui::light());
        tree.add(UpdateLine::new(vm.clone()));
        tree.layout(SizeProposal::exact(240.0, 60.0));
        assert!(tree.find_by_label("Version 999.0.0 is available").is_some());

        vm.forget_if_disabled(false);
        tree.layout(SizeProposal::exact(240.0, 60.0));
        assert!(
            tree.find_by_label("Version 999.0.0 is available").is_none(),
            "turning the check off has to clear the surface, not just the file"
        );
    }

    #[test]
    fn a_feed_with_no_link_still_states_the_fact() {
        // Still true, still useful, just not clickable.
        let vm = vm_with("999.0.0", BTreeMap::new());
        let mut tree = WidgetTree::new().with_theme(intui::light());
        tree.add(UpdateLine::new(vm));
        tree.layout(SizeProposal::exact(240.0, 60.0));
        assert!(tree.find_by_label("Version 999.0.0 is available").is_some());
    }
}
