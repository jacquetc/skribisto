// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The story-bible roster and its mirror, backlinks — the Inspector's two views of the
//! mention index.
//!
//! **Roster**, on a scene: who and what appears here. **Backlinks**, on a discoverable item:
//! where this character is mentioned. Both are the same rows read from opposite ends, so both
//! render through [`MentionList`].
//!
//! ## Confirmed and suggested are different claims, and look it
//!
//! A *confirmed* row is a persisted `references` entry — a decision the writer made. A
//! *suggested* row is the scan's guess from having found the name in the prose. Suggestions
//! are ghosted and carry a pin; confirmed rows are plain and do not.
//!
//! **Every suggestion shows its evidence.** Hovering gives the sentence the name was found
//! in, with the matched name and how often it occurred. This is the feature's answer to being
//! wrong: the matcher cannot tell a character called Don from the contraction "don't", so
//! rather than assert a roster it shows its working and lets the writer decline. A confident
//! bare list would be worse than useless the first time it was wrong.
//!
//! Nothing here writes except the pin, which goes through `references` and is undoable.

use std::rc::Rc;

use bastyde::core::BindingLevel;
use bastyde::core::accesskit::Role;
use bastyde::core::overlay::TooltipPlacement;
use bastyde::core::widget::WidgetPlacement;
use bastyde::prelude::*;
use bastyde::widgets::{HStack, IconButton, TextWidget, VStack};

use crate::view_models::MentionRow;
use crate::widgets::attach_labelled_composite_tooltip;

/// Persist a new confirmed-reference list for the item the list belongs to.
pub type PinReference = Rc<dyn Fn(u64, &mut EventContext)>;

/// Open a row's target in the side pane.
pub type OpenTarget = Rc<dyn Fn(u64, String, &mut EventContext)>;

pub struct MentionList {
    rows: Vec<MentionRow>,
    /// `None` for the backlinks direction: pinning means "this scene references that
    /// character", which is a statement about the *scene*, and a backlinks list is looking
    /// at the character. Offering a pin there would write the wrong item's references.
    pin: Option<PinReference>,
    open: OpenTarget,
    root_child: Option<WidgetId>,
}

impl MentionList {
    pub fn new(rows: Vec<MentionRow>, pin: Option<PinReference>, open: OpenTarget) -> Self {
        Self {
            rows,
            pin,
            open,
            root_child: None,
        }
    }
}

impl std::fmt::Debug for MentionList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MentionList")
            .field("rows", &self.rows.len())
            .finish()
    }
}

impl Widget for MentionList {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        let mut col = VStack::new().spacing(2.0);

        for row in &self.rows {
            let mut line = HStack::new().spacing(6.0);

            // Ghosted while it is only a guess.
            let colour = if row.is_confirmed {
                TextRole::Primary
            } else {
                TextRole::Secondary
            };
            line = line.child(
                TextWidget::new(lit!(row.title.clone()))
                    .style(TextStyleRole::Small)
                    .color(colour)
                    .max_lines(1),
            );

            // The alias that matched, when it was not the title — "Lizzy" explains a roster
            // entry that reads "Elizabeth Bennet" far better than the count does.
            if !row.is_title_match && !row.matched_name.is_empty() {
                line = line.child(
                    TextWidget::new(lit!(format!("({})", row.matched_name)))
                        .style(TextStyleRole::Tiny)
                        .color(TextRole::Secondary)
                        .max_lines(1),
                );
            }

            line = line.child(bastyde::widgets::Expand::horizontal().child(
                bastyde::widgets::Spacer::new(),
            ));

            if row.hit_count > 1 {
                line = line.child(
                    TextWidget::new(lit!(row.hit_count.to_string()))
                        .style(TextStyleRole::Tiny)
                        .color(TextRole::Secondary),
                );
            }

            // Pin, on suggestions only: a confirmed row is already pinned, and offering the
            // control again would suggest it does something.
            if !row.is_confirmed
                && let Some(pin) = self.pin.clone()
            {
                let target = row.target_id;
                line = line.child(
                    IconButton::add()
                        .embedded()
                        .tooltip(tr!(mentions_pin(name = row.title.clone())))
                        .on_activate_fn(move |c| pin(target, c)),
                );
            }

            let open = self.open.clone();
            let target = row.target_id;
            let title = row.title.clone();
            let id = ctx.add(
                line.access_role(Role::ListItem)
                    .access_label(lit!(row.title.clone()))
                    .focusable(true)
                    .on_tap({
                        let open = open.clone();
                        let title = title.clone();
                        move |_e, c| open(target, title.clone(), c)
                    })
                    // Same reason as the tag picker's rows: `on_tap` never fires from the
                    // keyboard, so a roster entry was Tab-reachable and inert. Enter opens
                    // the mentioned item, which is what clicking it does.
                    .on_key(move |ev, c| {
                        if let WidgetEvent::KeyDown { key, .. } = ev
                            && matches!(key, Key::Enter | Key::Space)
                        {
                            open(target, title.clone(), c);
                            return EventResponse::Handled;
                        }
                        EventResponse::Ignored
                    }),
            );

            // The evidence. Omitted for a confirmed reference the prose never names — there
            // is nothing to show, and an empty tooltip reads as a bug.
            if !row.evidence.is_empty() {
                attach_labelled_composite_tooltip(
                    ctx,
                    id,
                    Box::new(evidence_body(row)),
                    lit!(row.title.clone()),
                    TooltipPlacement::Side,
                );
            }
            col = col.add_child(id);
        }

        let id = ctx.add(col.access_role(Role::List));
        self.root_child = Some(id);
        vec![id]
    }

    fn layout_response(&self, proposal: SizeProposal, ctx: &LayoutContext) -> LayoutResponse {
        self.root_child
            .and_then(|id| ctx.child_size(id, proposal))
            .map(LayoutResponse::from)
            .unwrap_or_else(|| proposal.resolve(0.0, 0.0).into())
    }

    fn children(&self) -> Vec<WidgetId> {
        self.root_child.into_iter().collect()
    }

    fn place_children(
        &self,
        bounds: Rect,
        _proposal: SizeProposal,
        children: &mut [WidgetPlacement],
        _ctx: &LayoutContext,
    ) {
        for child in children.iter_mut() {
            child.origin = bounds.origin();
            child.size = bounds.size();
        }
    }
}

/// What a hover shows: which name matched, how often, and the sentence it sat in.
///
/// The sentence is the point. A count alone cannot distinguish a character named Don from
/// four hundred contractions; the sentence can, at a glance.
fn evidence_body(row: &MentionRow) -> impl Widget {
    let mut col = VStack::new().spacing(2.0).child(
        TextWidget::new(lit!(row.matched_name.clone()))
            .style(TextStyleRole::Small)
            .color(TextRole::TooltipText),
    );
    if row.hit_count > 1 {
        col = col.child(
            TextWidget::new(tr!(mentions_hit_count(n = row.hit_count)))
                .style(TextStyleRole::Tiny)
                .color(TextRole::TooltipText),
        );
    }
    col.child(
        TextWidget::new(lit!(row.evidence.clone()))
            .style(TextStyleRole::Tiny)
            .color(TextRole::TooltipText),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use bastyde::core::widget_tree::WidgetTree;

    fn row(target: u64, title: &str, confirmed: bool, evidence: &str) -> MentionRow {
        MentionRow {
            owner_id: 1,
            target_id: target,
            title: title.to_string(),
            matched_name: title.to_string(),
            is_title_match: true,
            hit_count: 1,
            is_confirmed: confirmed,
            evidence: evidence.to_string(),
        }
    }

    fn list(rows: Vec<MentionRow>, with_pin: bool) -> MentionList {
        MentionList::new(
            rows,
            with_pin.then(|| Rc::new(|_id: u64, _c: &mut EventContext| {}) as PinReference),
            Rc::new(|_id: u64, _t: String, _c: &mut EventContext| {}),
        )
    }

    #[test]
    fn one_line_per_row() {
        let mut tree = WidgetTree::new().with_theme(bastyde::presets::intui::light());
        let id = tree.add_boxed(Box::new(list(
            vec![row(2, "Grace", false, "Grace smiled."), row(3, "Will", true, "")],
            true,
        )));
        tree.layout(SizeProposal::exact(300.0, 200.0));
        let col = tree.children(id)[0];
        assert_eq!(tree.children(col).len(), 2);
    }

    /// How many buttons the list emits. The pin is the only one, so this counts pins
    /// without depending on how deeply the row happens to nest.
    fn pin_count(rows: Vec<MentionRow>, with_pin: bool) -> usize {
        let mut tree = WidgetTree::new().with_theme(bastyde::presets::intui::light());
        tree.add_boxed(Box::new(list(rows, with_pin)));
        tree.layout(SizeProposal::exact(300.0, 200.0));
        let _ = tree.render();
        tree.sync_accessibility()
            .nodes
            .iter()
            .filter(|(_, n)| n.role() == Role::Button)
            .count()
    }

    /// A confirmed row is already pinned; offering the control again would imply it does
    /// something.
    #[test]
    fn only_suggestions_offer_a_pin() {
        let rows = vec![row(2, "Suggested", false, "x"), row(3, "Pinned", true, "x")];
        assert_eq!(pin_count(rows, true), 1, "one pin, on the suggestion only");
    }

    /// A backlinks list has no pin at all: pinning states that *this scene* references a
    /// character, and a backlinks list is looking at the character, so the control would
    /// write the wrong item's references.
    #[test]
    fn a_list_without_a_pin_callback_renders_no_pin() {
        let rows = vec![row(2, "Grace", false, "x"), row(3, "Will", false, "x")];
        assert_eq!(pin_count(rows.clone(), true), 2, "both are suggestions");
        assert_eq!(pin_count(rows, false), 0, "no pin without a callback");
    }
}
