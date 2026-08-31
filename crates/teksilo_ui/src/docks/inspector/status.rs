// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Where this row is on the project's workflow ladder.
//!
//! The Inspector is the one surface with room for the glyph *and* the word, so it shows
//! both: the dense surfaces (the Overview cell, the stream row) show the glyph alone with
//! the name on hover, and the corkboard card shows both because a card has the width.
//!
//! Shown only where a status means something — [`skribisto_model::status_capable`], which
//! is "does this combination carry any content at all". That excludes exactly the two
//! content-free markers, `Item/BookEnd` and `Item/Text`.

use frontend::direct_access::BinderItemDto;
use teksilo::prelude::*;
use teksilo::widgets::{HStack, TextWidget, VStack};

use super::Inspector;

pub(super) fn section(
    mut col: VStack,
    panel: &Inspector,
    _ctx: &mut BuildContext,
    d: &BinderItemDto,
) -> VStack {
    if !skribisto_model::status_capable(&d.role, &d.sub_role) {
        return col;
    }

    let vm = panel.statuses.clone();
    let current = vm.rung(d.status.unwrap_or(0));
    // `d.status` may name a rung that no longer exists — the reference is weak so that
    // deleting one from the ladder leaves the rows that wore it intact. `rung()` returning
    // `None` is that case and the never-set case at once, which is right: to every reader
    // they are the same thing.
    let name = match &current {
        Some(r) => lit!(r.name.clone()),
        None => tr!(status_none()),
    };

    let set: crate::statuses::SetStatus = {
        let vm = vm.clone();
        let id = d.id;
        std::rc::Rc::new(move |status| vm.set_item_status(id, status))
    };

    col = col.child(
        TextWidget::new(tr!(inspector_status()))
            .style(TextStyleRole::Tiny)
            .color(TextRole::Secondary),
    );
    col = col.child(
        HStack::new()
            .spacing(6.0)
            .child(crate::statuses::status_picker(&vm, d.status, set))
            .child(
                TextWidget::new(name)
                    .color(match &current {
                        Some(_) => TextRole::Primary,
                        // An unset status is *stated*, quietly, rather than left blank:
                        // this is the pane you come to to find out where a row stands, and
                        // an empty space there does not answer the question.
                        None => TextRole::Secondary,
                    })
                    .single_line(),
            ),
    );
    col
}
