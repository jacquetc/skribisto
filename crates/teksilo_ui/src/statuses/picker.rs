// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The status picker: one glyph you click to open the ladder.
//!
//! Shaped like the Outline dock's binder switcher — a `PopoverIconButton` whose content is
//! a `MenuList`, `.bare()` because the list draws its own themed surface and the popover's
//! default one under it would be a second frame around the first.
//!
//! # Why the rows carry no checkmark
//!
//! `MenuItem`'s check and radio modes are **mutually exclusive with `.icon()`** — a
//! `debug_assert!` fires when both are set, and *in a release build the icon is silently
//! dropped*. So a picker copied row-for-row from `binder_row` would look right in a debug
//! run and lose every glyph in the shipped binary. That is why the binder switcher itself
//! has no per-row icon.
//!
//! teksilo's own doc names this exact case: `.icon(...)` recolours the glyph with the row's
//! text role, which is *"wrong for one whose colour **is** the content — a tag's swatch, a
//! status light"*. Its escape hatch, `.icon_keeps_color()`, is not the answer here either:
//! it does not dim on a disabled row and must carry its own contrast against a solid accent
//! highlight, which a near-isoluminant status hue cannot. An icon that wants a *role* wants
//! plain `.icon()`, and plain `.icon()` forbids the checkmark.
//!
//! Nothing is lost. The rows are self-identifying by glyph, the trigger already shows the
//! current rung, and the current row is marked by `TextRole::Accent` on its label. One
//! consequence to know: `.icon()` paints the glyph in the *row's* text role, so the ladder
//! reads monochrome inside the popover. That is the design working, not a limitation — if
//! five shapes are not distinguishable in one flat colour, the glyph set is wrong.

use std::rc::Rc;

use teksilo::prelude::*;
use teksilo::widgets::{IconButton, IconButtonSize, MenuItem, MenuList, PopoverIconButton};

use crate::statuses::glyph::{category_icon, status_glyph};
use crate::statuses::{StatusRung, StatusesViewModel};

/// What a surface does when the writer picks a rung. `None` is "no status".
pub type SetStatus = Rc<dyn Fn(Option<u64>)>;

/// The trigger glyph for a row's current rung.
///
/// "No status" renders as a **faint outline ring**, not as nothing: this is a button, and a
/// button with no glyph is a hole the writer cannot aim at. The empty *cell* elsewhere is a
/// different question — a dense table draws nothing there, because 400 marks that all say
/// "nothing" make the eye filter the default case instead of spotting the exception.
fn trigger_icon(current: Option<&StatusRung>) -> teksilo::widgets::IconWidget {
    match current {
        Some(rung) => status_glyph(&rung.category),
        None => category_icon(&common::entities::StatusCategory::Planned).color(TextRole::Disabled),
    }
}

/// The picker, as a `PopoverIconButton`.
///
/// `current` is the rung this row is at (already resolved — `None` covers both "unset" and
/// "the rung was deleted from the ladder", which are the same thing to every reader,
/// because the reference is weak on purpose).
pub fn status_picker(
    vm: &StatusesViewModel,
    current: Option<u64>,
    set: SetStatus,
) -> PopoverIconButton {
    status_picker_marked(vm, current, false, set)
}

/// [`status_picker`], told that the rows *below* this one do not all agree with it.
///
/// Only the Overview passes `true`: it is the one surface that shows a container beside
/// subtree sums, so a derived remark belongs there and nowhere else. The picker composes
/// the tooltip rather than the caller, so the rung's name and the derived remark cannot
/// drift apart across four surfaces.
pub fn status_picker_marked(
    vm: &StatusesViewModel,
    current: Option<u64>,
    subtree_differs: bool,
    set: SetStatus,
) -> PopoverIconButton {
    build(vm, current, subtree_differs, IconButtonSize::Toolbar, set)
}

/// The table variant: the same picker at [`IconButtonSize::Compact`].
///
/// A separate entry point rather than a flag, because the reason is specific and worth
/// naming. The Overview is a *dense* table — its rows are ~22 dp — and a toolbar-sized
/// button in a cell pushes every row to ~36 dp, costing about a third of the rows on
/// screen in a pane that exists to show many at once. Measured, not assumed: the two
/// heights above are from the same fixture with and without this column.
pub fn status_picker_dense(
    vm: &StatusesViewModel,
    current: Option<u64>,
    subtree_differs: bool,
    set: SetStatus,
) -> PopoverIconButton {
    build(vm, current, subtree_differs, IconButtonSize::Compact, set)
}

fn build(
    vm: &StatusesViewModel,
    current: Option<u64>,
    subtree_differs: bool,
    size: IconButtonSize,
    set: SetStatus,
) -> PopoverIconButton {
    let ladder = vm.ladder();
    let current_rung = current.and_then(|id| ladder.iter().find(|r| r.id == id).cloned());

    let mut menu = MenuList::new().max_visible_items(12);

    // "No status" is the zeroth member of the vocabulary, not a null: naming it is what
    // makes "show me everything I have not triaged" a question the writer can ask, and what
    // gives them a way back out of a rung they set by mistake.
    {
        let set = set.clone();
        let mut row = MenuItem::new(tr!(status_none())).on_activate_fn(move |_| set(None));
        if current_rung.is_none() {
            row = row.text_role(TextRole::Accent);
        }
        menu = menu.item(row);
    }
    menu = menu.separator();

    for rung in &ladder {
        let set = set.clone();
        let id = rung.id;
        let is_current = current == Some(id);
        let mut row = MenuItem::new(lit!(rung.name.clone()))
            // Plain `.icon()`, never a check/radio mode beside it — see the module doc.
            .icon(category_icon(&rung.category))
            .on_activate_fn(move |_| set(Some(id)));
        if is_current {
            row = row.text_role(TextRole::Accent);
        }
        if !rung.details.is_empty() {
            row = row.trailing_hint(lit!(rung.details.clone()));
        }
        menu = menu.item(row);
    }

    let tooltip = match (&current_rung, subtree_differs) {
        (Some(r), false) => lit!(r.name.clone()),
        (Some(r), true) => lit!(format!(
            "{} — {}",
            r.name,
            tr!(overview_status_mixed()).resolve_now()
        )),
        (None, false) => tr!(status_none()),
        (None, true) => tr!(overview_status_mixed()),
    };

    PopoverIconButton::new(
        IconButton::new(trigger_icon(current_rung.as_ref()))
            .size(size)
            .tooltip(tooltip),
    )
    .bare()
    .show_disclosure_caret(false)
    .content(menu)
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::entities::StatusCategory;

    fn rung(id: u64, name: &str, category: StatusCategory) -> StatusRung {
        StatusRung {
            id,
            uid: common::uid::fixture_uid(id),
            name: name.into(),
            category,
            details: String::new(),
        }
    }

    /// The picker must survive a rung that no longer resolves — a status can be deleted out
    /// from under an item, and the reference is weak precisely so the row keeps its prose.
    /// Resolving is a plain lookup, so this pins the lookup rather than the widget.
    #[test]
    fn a_status_id_that_no_longer_resolves_reads_as_no_status() {
        let ladder = [
            rung(1, "Draft", StatusCategory::Drafting),
            rung(2, "Final", StatusCategory::Final),
        ];
        let resolve = |current: Option<u64>| {
            current.and_then(|id| ladder.iter().find(|r| r.id == id).cloned())
        };
        assert_eq!(resolve(Some(1)).map(|r| r.name), Some("Draft".to_string()));
        assert_eq!(resolve(Some(99)), None, "a deleted rung reads as unset");
        assert_eq!(resolve(None), None);
    }
}
