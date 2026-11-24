// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Appearance & Behavior ▸ Tabs & views: how many tabs each side of the editor keeps,
//! and which view a container opens on.
//!
//! Under Appearance & Behavior rather than Editor, where both rows used to sit on the
//! Editor Behavior page: the Editor section is the writing surface, how it looks and what
//! it does as you type, and these two are about how the window manages what is open.

use teksilo::widgets::tooltip::TooltipContent;
use teksilo::widgets::{Segment, SegmentedControl};

use crate::shared::TabLimit;

#[allow(unused_imports)]
use super::super::*;

/// Display name for each tab limit.
fn tab_limit_label(limit: TabLimit) -> LocalizedString {
    match limit {
        TabLimit::One => tr!(settings_tab_limit_one()),
        TabLimit::Five => tr!(settings_tab_limit_five()),
        TabLimit::All => tr!(settings_tab_limit_all()),
    }
}

/// What each limit does, as a per-segment tooltip, the same self-documenting shape as
/// the caret band's.
fn tab_limit_tip(limit: TabLimit) -> TooltipContent {
    let (key, text) = match limit {
        TabLimit::One => ("settings.tab_limit.one", tr!(settings_tab_limit_tip_one())),
        TabLimit::Five => (
            "settings.tab_limit.five",
            tr!(settings_tab_limit_tip_five()),
        ),
        TabLimit::All => ("settings.tab_limit.all", tr!(settings_tab_limit_tip_all())),
    };
    TooltipContent::new(key, text)
}

/// Bridge the stored tab limit to the `SegmentedControl`'s `usize`, and return the
/// index signal: two guarded effects, the caret band's shape, whose `!=` guards stop
/// the two signals writing each other back and forth for ever.
fn bridge_tab_limit(ctx: &mut BuildContext, limit: Signal<TabLimit>) -> Signal<usize> {
    let index: Signal<usize> = Signal::new(limit.get().to_index());
    {
        let index = index.clone();
        ctx.effect(&limit, move |l: &TabLimit| {
            let i = l.to_index();
            if index.get() != i {
                index.set(i);
            }
        });
    }
    {
        ctx.effect(&index, move |i| {
            let l = TabLimit::from_index(*i);
            if limit.get() != l {
                limit.set(l);
            }
        });
    }
    index
}

/// Appearance & Behavior ▸ Tabs & views: the tab limit and the container-view memory.
pub(in crate::settings) fn tabs_views_pane(
    ctx: &mut BuildContext,
    crumbs: &Crumbs,
    vm: &SettingsViewModel,
) -> impl Widget {
    let tab_limit_control = TabLimit::all().into_iter().fold(
        SegmentedControl::indexed(bridge_tab_limit(ctx, vm.tab_limit())),
        |control, l| {
            control.segment(Segment::new(tab_limit_label(l)).rich_tooltip_content(tab_limit_tip(l)))
        },
    );

    let form = FormLayout::new()
        .label(tr!(settings_page_tabs_views()))
        .label_gap(16.0)
        .row_spacing(14.0)
        // How many tabs a pane keeps. Applied when a tab opens, never on the change
        // itself, which the hint says, since nothing visible happens on the click.
        .full_width(group(tr!(settings_group_tabs())))
        .line(field_label(tr!(settings_tab_limit())), tab_limit_control)
        .full_width(hint(tr!(settings_hint_tab_limit())))
        .full_width(group(tr!(settings_group_container_views())))
        .line(
            field_label(tr!(settings_remember_view())),
            Toggle::new(vm.remember_view())
                .labelled_externally()
                .rich_tooltip_content(
                    TooltipContent::new(
                        "settings.remember_view",
                        tr!(settings_remember_view_tip()),
                    )
                    .with_more(tr!(settings_remember_view_tip_more())),
                ),
        );

    pane_frame(crumbs.of(Pane::TabsViews), form)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::rc::Rc;
    use teksilo::core::widget_tree::WidgetTree;
    use teksilo::core::{LayoutContext, LayoutResponse, Widget, WidgetId};

    /// Hosts [`bridge_tab_limit`] in a real `BuildContext` and hands the index signal
    /// back, so a test can act as the `SegmentedControl` does.
    struct TabLimitHost {
        limit: Signal<TabLimit>,
        out: Rc<std::cell::RefCell<Option<Signal<usize>>>>,
    }
    impl std::fmt::Debug for TabLimitHost {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("TabLimitHost").finish()
        }
    }
    impl Widget for TabLimitHost {
        fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
            *self.out.borrow_mut() = Some(bridge_tab_limit(ctx, self.limit.clone()));
            Vec::new()
        }
        fn layout_response(&self, proposal: SizeProposal, _ctx: &LayoutContext) -> LayoutResponse {
            proposal.resolve(0.0, 0.0).into()
        }
    }

    /// Picking a segment writes the stored limit, and a limit written elsewhere (Reset)
    /// moves the control.
    #[test]
    fn the_tab_limit_round_trips_through_the_control() {
        let limit = Signal::new(TabLimit::All);
        let out = Rc::new(std::cell::RefCell::new(None));
        let mut tree = WidgetTree::new();
        tree.add(TabLimitHost {
            limit: limit.clone(),
            out: out.clone(),
        });
        tree.layout(SizeProposal::exact(100.0, 100.0));
        let index: Signal<usize> = out.borrow().clone().expect("the bridge was built");
        assert_eq!(index.get(), 2, "All is the third segment");

        index.set(0);
        assert_eq!(limit.get(), TabLimit::One);
        index.set(1);
        assert_eq!(limit.get(), TabLimit::Five);

        limit.set(TabLimit::All); // as Reset to defaults does
        assert_eq!(index.get(), 2, "the control must follow the setting");
        drop(tree);
    }
}
