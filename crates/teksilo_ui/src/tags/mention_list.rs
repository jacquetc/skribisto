// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The cast list and its mirror, backlinks — the Inspector's two views of the
//! mention index.
//!
//! **Cast**, on a scene/note/chapter: who and what the writer has pinned here,
//! plus scan suggestions. **Backlinks** / Appears in, on a discoverable item:
//! where this character is mentioned. Both are the same rows read from opposite
//! ends, so both render through [`MentionList`].
//!
//! ## Confirmed and suggested are different claims, and look it
//!
//! A *confirmed* row is a persisted `references` entry — a decision the writer made. A
//! *suggested* row is the scan's guess from having found the name in the prose. Suggestions
//! are ghosted and carry a pin; confirmed rows are plain and offer unpin.
//!
//! **Every suggestion shows its evidence.** Hovering gives the sentence the name was found
//! in, with the matched name and how often it occurred. This is the feature's answer to being
//! wrong: the matcher cannot tell a character called Don from the contraction "don't", so
//! rather than assert a roster it shows its working and lets the writer decline.
//!
//! ## A third claim: declared, not detected and not necessarily pinned
//!
//! [`MentionRow::is_point_of_view`](crate::mentions::MentionRow::is_point_of_view) is a
//! *different* persisted relationship from `is_confirmed` (`point_of_view`, not
//! `references`), and it must never be read as a stronger or weaker version of a pin.
//! A point-of-view row renders **plain, not ghosted**: whose eyes a scene is narrated
//! through is exactly the kind of thing a scan cannot detect (deep POV may name nobody), so
//! this is the writer's own declaration, not a guess to confirm or decline, and looks like
//! one. It carries a small badge saying so, because the row otherwise looks identical to an
//! ordinary confirmed pin. **It never offers unpin on its own:** unpin removes a
//! `references` entry, and a point of view was never written into `references`, so that
//! control stays keyed off `is_confirmed` alone, exactly as it already was before this flag
//! existed. A row can be point of view *and* confirmed at once (the writer cast the same
//! character formally too); the badge and the confirmed styling simply both apply, and both
//! must read sensibly together rather than fight for the row's one visual state.
//!
//! Nothing here writes except pin/unpin, which go through `references` and are undoable.

use std::rc::Rc;

use teksilo::core::accesskit::Role;
use teksilo::core::overlay::TooltipPlacement;
use teksilo::core::widget::WidgetPlacement;
use teksilo::prelude::*;
use teksilo::widgets::{Badge, HStack, IconButton, TextWidget, VStack};

use crate::mentions::MentionRow;

/// The confirm checkmark's glyph size, matching the leading glyphs the framework's own
/// menu rows draw at.
pub(crate) const CONFIRM_GLYPH: f32 = 12.0;
use crate::widgets::{RING_RADIUS_ROW, attach_labelled_composite_tooltip, with_focus_ring};

/// Persist a new confirmed-reference list for the item the list belongs to (append one id).
pub type PinReference = Rc<dyn Fn(u64, &mut EventContext)>;

/// Remove a confirmed reference from the item the list belongs to.
pub type UnpinReference = Rc<dyn Fn(u64, &mut EventContext)>;

/// Confirm, from a **backlink** row, that the entry this list belongs to really does
/// appear in that row's document — keyed by `owner_id`, the document doing the
/// mentioning.
///
/// Not [`PinReference`] reused, and the difference is the whole reason this exists: a pin
/// is keyed on the row's *target* and writes the list owner's references, which from this
/// direction would record the claim backwards. See [`crate::mentions::confirm_presence`].
pub type ConfirmPresence = Rc<dyn Fn(u64, &mut EventContext)>;

/// Open a row's target in the side pane.
pub type OpenTarget = Rc<dyn Fn(u64, String, &mut EventContext)>;

/// Which end of each mention names its row.
///
/// The same `MentionRow` is read from two directions. A **roster** on a scene asks "who
/// appears here", so each row is named by its *target*: the story-bible entry. A
/// **backlink** list on an entry asks "where does she appear", so each row is named by its
/// *owner*: the scene or chapter.
///
/// Naming both by the target is how a note's own "Appears in" came to render its own
/// title once per row, over and over, saying nothing at all. The owner titles come from
/// the caller because resolving them is a batched store read and this widget has no
/// `AppContext`.
pub enum MentionNaming {
    /// Name each row by the entry it mentions.
    Target,
    /// Name each row by the document doing the mentioning, titles keyed by `owner_id`.
    Owner(std::collections::HashMap<u64, String>),
}

/// Backlink rows in **manuscript order**, and the title of the document each one names.
///
/// Two things the backlink direction needs and the roster does not, resolved in one walk
/// of the work's flat item stream because that walk answers both at once.
///
/// **Order.** [`crate::mentions::MentionIndex`] sorts a row list by how good a match it
/// is: confirmed first, then title matches, then hit count. That is right for a roster,
/// which is a set of candidates, and wrong for a backlink list, which is a *reading*. "She
/// appears in chapter two, then not again until chapter nine" is a fact about the shape of
/// the book, and a list ordered by match quality throws it away.
///
/// **Names.** A row carries its target's title, never its owner's, so a backlink list has
/// to look the documents up. Batched here rather than per row.
///
/// Rows whose owner is not in the stream keep their relative order at the end rather than
/// being dropped: the scan already excludes trashed and deactivated owners, so anything
/// left is a row this walk simply could not place, and silently losing it would be worse
/// than showing it last.
pub fn documents_in_manuscript_order(
    ctx: &frontend::AppContext,
    work_id: u64,
    mut rows: Vec<MentionRow>,
) -> (Vec<MentionRow>, MentionNaming) {
    let flat = crate::models::binder_stream::ordered_flat_items(ctx, work_id);
    let mut position: std::collections::HashMap<u64, usize> =
        std::collections::HashMap::with_capacity(flat.len());
    let mut titles: std::collections::HashMap<u64, String> =
        std::collections::HashMap::with_capacity(flat.len());
    for (i, (_, it)) in flat.iter().enumerate() {
        position.insert(it.id, i);
        titles.insert(it.id, it.title.clone());
    }
    rows.sort_by_key(|r| position.get(&r.owner_id).copied().unwrap_or(usize::MAX));
    (rows, MentionNaming::Owner(titles))
}

pub struct MentionList {
    rows: Vec<MentionRow>,
    naming: MentionNaming,
    /// `None` for the backlinks direction: pinning means "this scene references that
    /// character", which is a statement about the *scene*, and a backlinks list is looking
    /// at the character. Offering a pin there would write the wrong item's references.
    pin: Option<PinReference>,
    /// Same gate as pin: only the cast direction (owner item) can unpin.
    unpin: Option<UnpinReference>,
    /// The backlink direction's own control — see [`ConfirmPresence`]. `None` on a
    /// roster, where the pin above is already the right shape.
    confirm: Option<ConfirmPresence>,
    open: OpenTarget,
    root_child: Option<WidgetId>,
}

impl MentionList {
    pub fn new(
        rows: Vec<MentionRow>,
        naming: MentionNaming,
        pin: Option<PinReference>,
        unpin: Option<UnpinReference>,
        open: OpenTarget,
    ) -> Self {
        Self {
            rows,
            naming,
            pin,
            unpin,
            confirm: None,
            open,
            root_child: None,
        }
    }

    /// Offer, on every suggested row, to confirm that this entry really appears in that
    /// document. **Confirm only**: a row that is already confirmed shows no control at
    /// all, because taking a mention back is a statement about the *scene's* cast and
    /// belongs where the cast is edited.
    pub fn confirm(mut self, confirm: ConfirmPresence) -> Self {
        self.confirm = Some(confirm);
        self
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

            // Ghosted while it is only a guess. A confirmed pin and a declared point of view
            // are both the writer's own decision rather than the scan's, so either keeps the
            // row plain.
            let colour = if row.is_confirmed || row.is_point_of_view {
                TextRole::Primary
            } else {
                TextRole::Secondary
            };
            // **A pin whose target is gone is named, not left blank.** `cast_for` resolves a
            // row's title through the discoverable table and falls back to an empty string
            // when the target is not in it, which happens for a pin whose entry has since
            // been trashed or had its story-bible tag removed. Rendered raw that was a
            // nameless row carrying nothing but a delete button, which reads as a bug rather
            // than as a stale pin.
            //
            // Named here in the view rather than invented in the index, which is right to
            // report "no title" for something it genuinely cannot resolve. And named rather
            // than dropped, unlike `pov_chips`, because this row owns the only affordance
            // that can remove the pin: hiding it would strand the writer with a reference
            // they can see the effects of and cannot reach.
            let name = match &self.naming {
                MentionNaming::Target => row.title.clone(),
                MentionNaming::Owner(titles) => {
                    titles.get(&row.owner_id).cloned().unwrap_or_default()
                }
            };
            let unresolved = name.trim().is_empty();
            let missing = match &self.naming {
                MentionNaming::Target => tr!(cast_unresolved()),
                // A document with no title of its own is ordinary, not stale: it is what
                // the binder itself calls an unnamed row, not a pin pointing at nothing.
                MentionNaming::Owner(_) => tr!(note_details_untitled_document()),
            };
            let label = if unresolved {
                TextWidget::new(missing)
                    .style(TextStyleRole::Small)
                    .color(TextRole::Secondary)
                    .max_lines(1)
            } else {
                TextWidget::new(lit!(name.clone()))
                    .style(TextStyleRole::Small)
                    .color(colour)
                    .max_lines(1)
            };
            line = line.child(label);

            // The alias that matched, when it was not the title — "Lizzy" explains a roster
            // entry that reads "Elizabeth Bennet" far better than the count does.
            // Every name that matched here, not just the first: "(Elizabeth, Lizzy)"
            // says which names this scene actually reaches for, which one of them could
            // not. Omitted when the only match was the title, where it would just repeat
            // the row's own headline.
            // Shown unless the only thing that matched was the title, where it would
            // merely repeat the row's own headline.
            if row.matched_names.len() > 1 || (row.matched_names.len() == 1 && !row.is_title_match)
            {
                line = line.child(
                    TextWidget::new(lit!(format!("({})", row.matched_label())))
                        .style(TextStyleRole::Tiny)
                        .color(TextRole::Secondary)
                        .max_lines(1),
                );
            }

            // A small, deliberate badge, never a substitute for the pin/unpin state above,
            // which stays keyed off `is_confirmed` alone. See the module doc's "third claim"
            // section for why this cannot be folded into `is_confirmed` instead.
            if row.is_point_of_view {
                line = line.child(
                    Badge::new(tr!(mentions_point_of_view_badge()))
                        .tooltip(tr!(mentions_point_of_view_badge_tooltip())),
                );
            }

            line = line.child(
                teksilo::widgets::Expand::horizontal().child(teksilo::widgets::Spacer::new()),
            );

            if row.hit_count > 1 {
                line = line.child(
                    TextWidget::new(lit!(row.hit_count.to_string()))
                        .style(TextStyleRole::Tiny)
                        .color(TextRole::Secondary),
                );
            }

            if row.is_confirmed {
                if let Some(unpin) = self.unpin.clone() {
                    let target = row.target_id;
                    line = line.child(
                        IconButton::clear()
                            .embedded()
                            .tooltip(tr!(cast_unpin(name = row.title.clone())))
                            .on_activate_fn(move |c| unpin(target, c)),
                    );
                }
            } else if let Some(pin) = self.pin.clone() {
                let target = row.target_id;
                line = line.child(
                    IconButton::add()
                        .embedded()
                        .tooltip(tr!(cast_pin(name = row.title.clone())))
                        .on_activate_fn(move |c| pin(target, c)),
                );
            } else if !row.is_point_of_view
                && let Some(confirm) = self.confirm.clone()
            {
                // A checkmark, not the roster's plus: from this end the writer is
                // agreeing with something the scan already found, not adding someone to
                // a scene they were absent from.
                //
                // Never on a declared point of view. `set_point_of_view` already writes
                // the character into the scene's cast as one composite, so such a row is
                // confirmed too and never reaches here — and a row that renders *plain*,
                // as a declaration does, must not also carry a control that says the
                // writer has yet to agree with it.
                let owner = row.owner_id;
                line = line.child(
                    IconButton::new(teksilo::widgets::primitives::IconWidget::checkmark(
                        CONFIRM_GLYPH,
                    ))
                    .embedded()
                    .tooltip(tr!(mentions_confirm(name = name.clone())))
                    .on_activate_fn(move |c| confirm(owner, c)),
                );
            }

            let open = self.open.clone();
            // Open what the row *names*. A roster row names the entry, so it opens the
            // entry; a backlink row names the document, so it opens the document. Opening
            // the target from a backlink list would reopen the note the writer is already
            // looking at, once per row.
            let target = match &self.naming {
                MentionNaming::Target => row.target_id,
                MentionNaming::Owner(_) => row.owner_id,
            };
            let row_name = if unresolved {
                String::new()
            } else {
                name.clone()
            };
            let title = row_name.clone();
            // The row is a focus stop, so it must *look* like one when the
            // keyboard lands on it. Teksilo paints no ring for a hand-built
            // node — see `crate::widgets::focus_ring` — so these rows were
            // Tab-reachable and visually silent.
            let focused = ctx.signal(false);
            let ringed = with_focus_ring(ctx, RING_RADIUS_ROW, line, &focused);
            let id = ctx.add(
                ringed
                    .access_role(Role::ListItem)
                    .access_label(lit!(row_name.clone()))
                    .focusable(true)
                    .on_focus({
                        let focused = focused.clone();
                        move |gained, _c| focused.set(gained)
                    })
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
                    lit!(row_name.clone()),
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
        TextWidget::new(lit!(row.matched_label()))
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
    use teksilo::core::widget_tree::WidgetTree;

    fn row(target: u64, title: &str, confirmed: bool, evidence: &str) -> MentionRow {
        pov_row(target, title, confirmed, false, evidence)
    }

    /// Same shape as [`row`], with the point-of-view flag also settable, kept as a second
    /// function rather than a fifth positional bool on `row` itself, whose call sites (all
    /// predating this flag) stay untouched.
    fn pov_row(
        target: u64,
        title: &str,
        confirmed: bool,
        point_of_view: bool,
        evidence: &str,
    ) -> MentionRow {
        MentionRow {
            owner_id: 1,
            target_id: target,
            title: title.to_string(),
            matched_names: vec![title.to_string()],
            is_title_match: true,
            hit_count: 1,
            is_confirmed: confirmed,
            is_point_of_view: point_of_view,
            evidence: evidence.to_string(),
        }
    }

    fn list(rows: Vec<MentionRow>, with_pin: bool, with_unpin: bool) -> MentionList {
        MentionList::new(
            rows,
            MentionNaming::Target,
            with_pin.then(|| Rc::new(|_id: u64, _c: &mut EventContext| {}) as PinReference),
            with_unpin.then(|| Rc::new(|_id: u64, _c: &mut EventContext| {}) as UnpinReference),
            Rc::new(|_id: u64, _t: String, _c: &mut EventContext| {}),
        )
    }

    #[test]
    fn one_line_per_row() {
        let mut tree = WidgetTree::new().with_theme(teksilo::presets::intui::light());
        let id = tree.add_boxed(Box::new(list(
            vec![
                row(2, "Grace", false, "Grace smiled."),
                row(3, "Will", true, ""),
            ],
            true,
            true,
        )));
        tree.layout(SizeProposal::exact(300.0, 200.0));
        let col = tree.children(id)[0];
        assert_eq!(tree.children(col).len(), 2);
    }

    /// How many buttons the list emits. Pin and unpin are the only ones, so this
    /// counts controls without depending on how deeply the row happens to nest.
    fn button_count(rows: Vec<MentionRow>, with_pin: bool, with_unpin: bool) -> usize {
        let mut tree = WidgetTree::new().with_theme(teksilo::presets::intui::light());
        tree.add_boxed(Box::new(list(rows, with_pin, with_unpin)));
        tree.layout(SizeProposal::exact(300.0, 200.0));
        let _ = tree.render();
        tree.sync_accessibility()
            .nodes
            .iter()
            .filter(|(_, n)| n.role() == Role::Button)
            .count()
    }

    /// A confirmed row offers unpin; a suggestion offers pin — never both on one row.
    #[test]
    fn suggestions_pin_and_confirmed_unpin() {
        let rows = vec![row(2, "Suggested", false, "x"), row(3, "Pinned", true, "x")];
        assert_eq!(button_count(rows, true, true), 2, "one pin + one unpin");
    }

    /// A backlinks list has no pin/unpin at all: those write the *owner* item's
    /// references, and a backlinks list is looking at the character.
    #[test]
    fn a_list_without_callbacks_renders_no_controls() {
        let rows = vec![row(2, "Grace", false, "x"), row(3, "Will", true, "x")];
        assert_eq!(button_count(rows.clone(), true, true), 2);
        assert_eq!(button_count(rows, false, false), 0);
    }

    #[test]
    fn empty_list_builds() {
        let mut tree = WidgetTree::new().with_theme(teksilo::presets::intui::light());
        let id = tree.add_boxed(Box::new(list(vec![], true, true)));
        tree.layout(SizeProposal::exact(300.0, 200.0));
        assert!(tree.children(id).len() <= 1);
    }

    /// The badge announces as `Role::Label` (see `Badge`'s own doc); nothing else in a row
    /// does, so counting labels is a reliable proxy for "how many point-of-view badges did
    /// this list draw" the same way `button_count` is a reliable proxy for pin/unpin.
    fn label_count(rows: Vec<MentionRow>, with_pin: bool, with_unpin: bool) -> usize {
        let mut tree = WidgetTree::new().with_theme(teksilo::presets::intui::light());
        tree.add_boxed(Box::new(list(rows, with_pin, with_unpin)));
        tree.layout(SizeProposal::exact(300.0, 200.0));
        let _ = tree.render();
        tree.sync_accessibility()
            .nodes
            .iter()
            .filter(|(_, n)| n.role() == Role::Label)
            .count()
    }

    /// **The defect this row type was fixed against.** A point of view with no reference
    /// pin has nothing in `references` to remove, so it must offer pin (to confirm it as a
    /// cast member too), never unpin: a dead unpin button that silently does nothing on
    /// click is worse than the missing row it would sit next to.
    #[test]
    fn a_point_of_view_only_row_offers_pin_never_unpin() {
        let rows = vec![pov_row(2, "Grace", false, true, "")];
        assert_eq!(
            button_count(rows, true, true),
            1,
            "pin only: is_confirmed is false, so there is nothing to unpin"
        );
    }

    /// Once the same target is also pinned, the row switches to unpin, same as any other
    /// confirmed row: the point-of-view flag never overrides `is_confirmed`'s own control.
    #[test]
    fn a_row_that_is_both_confirmed_and_point_of_view_offers_unpin_not_pin() {
        let rows = vec![pov_row(2, "Grace", true, true, "")];
        assert_eq!(
            button_count(rows, true, true),
            1,
            "unpin only: is_confirmed is true regardless of the point-of-view flag"
        );
    }

    /// The badge renders only for a point-of-view row, and exactly once, never doubled up
    /// when the same row is also confirmed. Compared against a same-shaped row's own
    /// baseline label count (the title text is itself an AT label, see `TextWidget`'s
    /// default role, so the useful assertion is "one more label than an otherwise
    /// identical row", not an absolute count that would break the moment the row grows
    /// another label of its own for an unrelated reason).
    #[test]
    fn the_point_of_view_badge_renders_only_for_a_point_of_view_row() {
        let baseline = label_count(vec![row(2, "Grace", false, "")], true, true);

        let pov_only = vec![pov_row(2, "Grace", false, true, "")];
        assert_eq!(
            label_count(pov_only, true, true),
            baseline + 1,
            "a point-of-view row adds exactly one label: its badge"
        );

        let both = vec![pov_row(2, "Grace", true, true, "")];
        assert_eq!(
            label_count(both, true, true),
            baseline + 1,
            "a row that is both confirmed and point of view still shows exactly one badge, \
             not two"
        );
    }
}
