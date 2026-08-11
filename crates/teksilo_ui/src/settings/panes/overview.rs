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
/// enum's to know. `parent_crumb` is the section a nested group sits in — `None`
/// for a section, which is already top level.
pub(in crate::settings) fn overview_pane(
    parent: Pane,
    title: LocalizedString,
    parent_crumb: Option<LocalizedString>,
    children: Vec<Pane>,
    nav: Navigator,
) -> impl Widget {
    let mut header = VStack::new()
        .spacing(6.0)
        .child(TextWidget::new(title.clone()).style(TextStyleRole::BodyBold));
    if let Some(description) = parent.description() {
        header = header.child(hint(description));
    }

    let mut entries = VStack::new().spacing(ENTRY_SPACING);
    for child in children {
        entries = entries.child(entry(child, &nav));
    }

    pane_frame(
        crumb(parent_crumb, title),
        VStack::new().spacing(22.0).child(header).child(entries),
    )
}
