// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! A **parent's own page** — what a section, or the nested Typography group,
//! shows when it is selected: its title, the line that says what it is for, and a
//! link to each thing under it.
//!
//! Before this, a parent selected nothing at all. The tree highlighted the row,
//! the chevron expanded it, and the right-hand pane went on showing whichever leaf
//! had been open last — so the window contradicted its own navigation, and the
//! only way to learn what was inside a collapsed section was to open it.
//!
//! ## Why links and not a table of contents widget
//!
//! Each entry is a [`Link`] with the page's one-line description under it, because
//! that is what the entry *is*: the same jump the tree row performs, in prose the
//! tree has no room for. A nested group appears as **one** link to its own page
//! rather than flattening its six pages into its parent's list — the group exists
//! precisely because those six crowd the section, and inlining them here would
//! undo that.
//!
//! An extension's page has a label and no description ([`Pane::description`]
//! returns `None`), so it renders as a bare link. That is honest — the app has no
//! line to write on its behalf — and it is the only case where the gloss is
//! missing.
//!
//! Built with chained builders rather than `teksu!`: the entry list is a loop over
//! however many children the tree spec gives, which is the same reason the
//! `FormLayout` panes beside it are chained.

use teksilo::widgets::Link;

#[allow(unused_imports)]
use super::super::*;

/// Gap between two entries — wide enough that a two-line description doesn't read
/// as belonging to the link below it.
const ENTRY_SPACING: f32 = 16.0;
/// Gap between a link and its own description.
const LINK_DESCRIPTION_GAP: f32 = 3.0;

/// One entry: the child's name as a link, and what is on it.
fn entry(child: Pane, nav: &Navigator) -> impl Widget {
    let nav = nav.clone();
    let link = Link::new(child.label()).on_activate_fn(move |_ctx| nav.go(child));
    let column = VStack::new().spacing(LINK_DESCRIPTION_GAP).child(link);
    match child.description() {
        Some(description) => column.child(hint(description)),
        None => column,
    }
}

/// A parent's page.
///
/// `title` is passed in rather than taken from `parent` because the open
/// project's section reads "Work: `<title>`", and the project's title is not the
/// enum's to know — [`Crumbs::titled`] takes it for the same reason.
///
/// The trail above it comes from `crumbs`, which derives it from the same spec
/// the rail was built from. It used to be handed in as a single label and the
/// `Pane` behind it recovered by matching the *resolved text* of every section
/// the navigator knew — which is a lookup by rendered string, wrong in any
/// locale where two sections happen to print alike, and silently inert (a link
/// that does nothing) whenever it missed.
pub(in crate::settings) fn overview_pane(
    parent: Pane,
    title: LocalizedString,
    crumbs: &Crumbs,
    children: Vec<Pane>,
    nav: Navigator,
) -> impl Widget {
    // No title line here: the breadcrumb band 34 px above already prints exactly
    // this string, and printing it twice made every parent page open with its own
    // name stuttered back at the reader.
    let mut header = VStack::new().spacing(6.0);
    if let Some(description) = parent.description() {
        header = header.child(hint(description));
    }

    let mut entries = VStack::new().spacing(ENTRY_SPACING);
    for child in children {
        entries = entries.child(entry(child, &nav));
    }

    pane_frame(
        crumbs.titled(parent, title),
        VStack::new().spacing(22.0).child(header).child(entries),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::settings::nav::GroupKind;
    use std::rc::Rc;
    use teksilo::core::widget_tree::WidgetTree;
    use teksilo::core::{LayoutContext, LayoutResponse, WidgetId};

    /// Builds the real category tree so a test can hold the same `Navigator` the
    /// window does — node ids are opaque, so there is no way to fabricate one.
    struct NavHost {
        out: Rc<std::cell::RefCell<Option<Navigator>>>,
    }
    impl std::fmt::Debug for NavHost {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("NavHost").finish()
        }
    }
    impl Widget for NavHost {
        fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
            let selected_pane = Signal::new(Pane::SceneTypography);
            let spec = Rc::new(crate::settings::tree_spec(false, &[]));
            let (tree, reveal, selection, nodes) =
                crate::settings::tree::build_tree(ctx, &selected_pane, &spec, String::new());
            *self.out.borrow_mut() = Some(Navigator {
                selection,
                nodes: Rc::new(nodes),
                selected_pane,
                reveal,
                spec,
            });
            vec![ctx.add(tree)]
        }
        fn layout_response(&self, proposal: SizeProposal, _ctx: &LayoutContext) -> LayoutResponse {
            proposal.resolve(0.0, 0.0).into()
        }
    }

    fn navigator() -> (WidgetTree, Navigator) {
        let out = Rc::new(std::cell::RefCell::new(None));
        let mut tree = WidgetTree::new();
        tree.add(NavHost { out: out.clone() });
        tree.layout(SizeProposal::exact(262.0, 500.0));
        let nav = out.borrow().clone().expect("the tree was built");
        (tree, nav)
    }

    /// A [`Crumbs`] over the same spec the rail is built from, with the real
    /// navigator behind it — what the window hands every pane.
    fn crumbs(nav: &Navigator) -> Crumbs {
        Crumbs::new(
            Rc::new(crate::settings::tree_spec(false, &[])),
            "",
            Some(nav.clone()),
        )
    }

    /// A leaf two levels down names **both** its ancestors, and each of them is
    /// a live link.
    ///
    /// The trail used to be two levels while the tree is three, so every
    /// typography page read "Editor › Scene" and never named the group it is
    /// actually in — and the ancestor it did print carried no action at all.
    #[test]
    fn a_typography_pages_trail_names_its_group_and_its_section() {
        let (_tree, nav) = navigator();
        let crumbs = crumbs(&nav);
        let trail = crumbs.trail(Pane::SceneTypography);
        assert_eq!(
            trail.iter().map(|(p, _)| *p).collect::<Vec<_>>(),
            vec![
                Pane::Section(Sec::Editor),
                Pane::Group(GroupKind::Typography)
            ],
            "Scene sits under Editor ▸ Typography, so its crumb must name both"
        );
        // Trail plus the page itself: "Editor › Typography › Scene".
        assert_eq!(trail.len() + 1, 3);
        assert!(
            trail.iter().all(|(p, _)| crumbs.jump(*p).is_some()),
            "an ancestor crumb with no action is a Role::Link that cannot be \
             activated — announced to a screen reader, inert to a click"
        );
    }

    /// Activating the *middle* crumb lands on the group's own landing page —
    /// both halves of it: the pane switches and the tree's highlight follows.
    #[test]
    fn activating_the_middle_crumb_opens_the_groups_landing_page() {
        let (mut tree, nav) = navigator();
        let crumbs = crumbs(&nav);
        let group = Pane::Group(GroupKind::Typography);
        let go = crumbs
            .trail(Pane::SceneTypography)
            .into_iter()
            .map(|(p, _)| p)
            .find(|p| *p == group)
            .and_then(|p| crumbs.jump(p))
            .expect("the Typography crumb is wired");

        tree.run_with_event_context(&mut teksilo::core::NoopWindowOps, |ctx| go(ctx));

        assert_eq!(
            nav.selected_pane.get(),
            group,
            "the pane must switch to the group's own page"
        );
        let node = nav.nodes.get(&group).copied().expect("the group has a row");
        assert_eq!(
            nav.selection.selected_keys(),
            vec![node],
            "the tree's highlight must follow the crumb, or the window and its \
             navigation disagree about where the reader is"
        );
    }

    /// The parent page must print its own name **once**.
    ///
    /// The breadcrumb band 34 px above already carries it; a title line under the
    /// band repeated it, on all seven parent pages. The header column now holds
    /// the description alone — at most one child, never a title plus a gloss.
    #[test]
    fn a_parents_page_does_not_print_its_title_under_the_breadcrumb() {
        let (_nav_tree, nav) = navigator();
        let mut tree = WidgetTree::new();
        let crumbs = crumbs(&nav);
        let frame = tree.add(overview_pane(
            Pane::Section(Sec::Editor),
            Sec::Editor.label(),
            &crumbs,
            vec![Pane::EditorBehavior, Pane::Goals],
            nav,
        ));
        tree.layout(SizeProposal::exact(657.0, 519.0));

        // pane_frame → band · rule · scroll → ScrollArea → Padding → the body
        // VStack, whose first child is the header column.
        let scroll = tree.children(tree.children(frame)[2])[0];
        let body = tree.children(tree.children(scroll)[0])[0];
        let header = tree.children(body)[0];
        assert!(
            tree.children(header).len() <= 1,
            "the header column must hold the description alone, got {} children",
            tree.children(header).len()
        );
    }
}
