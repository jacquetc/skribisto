// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Point of view — whose eyes a scene is told through.
//!
//! Sits beside the cast in the Inspector and deliberately reuses its picker: the candidates
//! are the same story-bible entries, and picking one that is not yet in the cast adds them
//! (see `SingleBinderItem::set_point_of_view`, which groups the two writes so one Ctrl+Z
//! undoes both).
//!
//! What is **not** reused is [`crate::tags::MentionList`] for the display. Its rows carry
//! scan evidence — a hit count, the matched name, the sentence the name was found in — and
//! none of that means anything for a point of view: a scene told in deep POV may never name
//! its own viewpoint character at all, which is exactly why the mention scanner unions
//! hand-pinned references in rather than trusting text hits. A chip row states the fact
//! without implying it was detected.

use std::rc::Rc;

use skribisto_model::mentions::DiscoverableEntity;
use teksilo::prelude::*;
use teksilo::widgets::{Button, ButtonVariant, HStack, IconButton, PopoverButton, TextWidget};

use super::cast_add::{CastAddPopover, CastCandidate};
use super::mention_list::PinReference;

/// Remove `target` as a point of view on the focused item.
pub type ClearPointOfView = Rc<dyn Fn(u64, &mut EventContext)>;

/// One viewpoint character, as shown.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PovChip {
    pub id: u64,
    pub title: String,
}

/// Resolve the ids stored on an item against the story-bible table.
///
/// An id with no matching entry is dropped rather than rendered as a blank chip: it means
/// the target was trashed or lost its discoverable tag, and the same unresolved-target
/// filtering the mention scan applies to `references` applies here for the same reason.
pub fn pov_chips(table: &[DiscoverableEntity], ids: &[u64]) -> Vec<PovChip> {
    ids.iter()
        .filter_map(|id| {
            table.iter().find(|e| e.id == *id).map(|e| PovChip {
                id: e.id,
                title: e.title.clone(),
            })
        })
        .collect()
}

/// True when a point of view was pinned but does not fully resolve against `table`:
/// the target was trashed, or lost its discoverable tag, after being pinned.
///
/// Distinct from `ids.is_empty()`, which means no point of view was ever set: this
/// means one *was*, and rotted out from under the pin. [`pov_chips`] silently drops
/// an id like that rather than rendering a blank chip, which is right for the chip
/// row itself, but leaves nothing telling the writer the pin is still there and
/// pointing at nothing, the same blank space a never-set point of view would show.
/// The Inspector section renders this in plain words instead of staying silent.
pub fn pov_has_unresolved(table: &[DiscoverableEntity], ids: &[u64]) -> bool {
    !ids.is_empty() && pov_chips(table, ids).len() < ids.len()
}

/// The viewpoint characters of one item, each removable.
pub fn pov_chip_row(chips: Vec<PovChip>, clear: ClearPointOfView) -> impl Widget {
    let mut row = HStack::new().spacing(4.0);
    for chip in chips {
        let clear = clear.clone();
        let id = chip.id;
        row = row
            .child(
                TextWidget::new(lit!(chip.title.clone()))
                    .style(TextStyleRole::Tiny)
                    // See the note in `tags::books::book_chip_row`.
                    .single_line(),
            )
            .child(
                IconButton::clear()
                    .embedded()
                    .tooltip(tr!(pov_remove(name = chip.title.clone())))
                    .on_activate_fn(move |c| clear(id, c)),
            );
    }
    row
}

/// Trigger button that opens the point-of-view picker.
///
/// `already` is the current point of view, not the cast: the popover hides what is already
/// chosen, and a character can be in the cast without holding the camera.
pub fn pov_add_button(
    candidates: Vec<CastCandidate>,
    already: Vec<u64>,
    owner_id: u64,
    set: PinReference,
) -> impl Widget {
    PopoverButton::new(Button::new(tr!(pov_add())).variant(ButtonVariant::Plain))
        .content(CastAddPopover::new(candidates, already, owner_id, set))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn entity(id: u64, title: &str) -> DiscoverableEntity {
        DiscoverableEntity {
            id,
            title: title.to_string(),
            aliases: vec![],
        }
    }

    #[test]
    fn chips_resolve_ids_against_the_story_bible() {
        let table = vec![entity(1, "Devon"), entity(2, "Hap")];
        let chips = pov_chips(&table, &[2]);
        assert_eq!(chips.len(), 1);
        assert_eq!(chips[0].title, "Hap");
    }

    /// A trashed or un-tagged target must vanish rather than render as a blank chip — the
    /// same unresolved-target rule the mention scan applies to `references`.
    #[test]
    fn an_unresolvable_id_is_dropped_not_rendered_blank() {
        let table = vec![entity(1, "Devon")];
        assert!(pov_chips(&table, &[99]).is_empty());
        assert_eq!(pov_chips(&table, &[1, 99]).len(), 1);
    }

    /// Two viewpoints is a legal, deliberate state — the schema allows it so head-hopping
    /// can be seen rather than blocked, and the row must render both.
    #[test]
    fn two_viewpoints_both_render() {
        let table = vec![entity(1, "Devon"), entity(2, "Hap")];
        assert_eq!(pov_chips(&table, &[1, 2]).len(), 2);
    }

    #[test]
    fn no_point_of_view_is_an_empty_row_not_an_error() {
        assert!(pov_chips(&[entity(1, "Devon")], &[]).is_empty());
    }

    /// Order follows the stored ids, so the row is stable across rebuilds rather than
    /// reshuffling on every repaint.
    #[test]
    fn chip_order_follows_the_stored_ids() {
        let table = vec![entity(1, "Devon"), entity(2, "Hap")];
        let titles: Vec<String> = pov_chips(&table, &[2, 1])
            .into_iter()
            .map(|c| c.title)
            .collect();
        assert_eq!(titles, ["Hap", "Devon"]);
    }

    /// No point of view was ever set: not the "rotted pin" state at all.
    #[test]
    fn no_pins_is_not_an_unresolved_pin() {
        let table = vec![entity(1, "Devon")];
        assert!(!pov_has_unresolved(&table, &[]));
    }

    /// Every pin resolves cleanly: nothing to explain to the writer.
    #[test]
    fn fully_resolved_pins_are_not_reported_as_unresolved() {
        let table = vec![entity(1, "Devon"), entity(2, "Hap")];
        assert!(!pov_has_unresolved(&table, &[1, 2]));
    }

    /// The one pin that was set points at an id the table no longer has, the way a
    /// trashed or untagged former viewpoint character would: the pill row must not
    /// look identical to "no point of view was ever set".
    #[test]
    fn a_pin_pointing_at_a_gone_target_is_reported_as_unresolved() {
        let table = vec![entity(1, "Devon")];
        assert!(pov_has_unresolved(&table, &[99]));
    }

    /// One of two pins rotted out; the other still resolves and still renders. The
    /// state must still be flagged, since a writer glancing at one chip has no way
    /// to know a second one used to be there.
    #[test]
    fn a_partially_resolved_pin_set_is_still_reported_as_unresolved() {
        let table = vec![entity(1, "Devon")];
        assert!(pov_has_unresolved(&table, &[1, 99]));
    }
}
