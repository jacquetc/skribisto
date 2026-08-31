// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! `StructureNumber` — the chapter's (or part's) ordinal, shown beside its title.
//!
//! The number the exporter prints, made visible where the writer works. A chapter is
//! *called* "Welcome"; it *is* the third chapter; those are two facts and the writer needs
//! both. Until this existed, the app showed only the first, so writers typed the second
//! into the title — and the exporter, generating its own, printed it twice.
//!
//! # Why a sibling widget and not a prefixed string
//!
//! Three reasons, and each of them is a bug avoided rather than a preference:
//!
//! * **The title stays the title.** `BinderItem.title` has two homes kept in lockstep by
//!   `SingleBinderItem::write_name`, it is what the mention index matches against prose,
//!   what search filters on, and what an inline rename seeds its buffer from. A display
//!   string that ever reached one of those write paths would put a numeral into stored
//!   data — the exact failure this feature exists to undo.
//! * **Bidi.** A bare numeral has no strong directional character, so a `"3. "` spliced
//!   onto a Hebrew or Arabic title resolves LTR and lands on the wrong side of it. A
//!   separate widget carries its own direction and cannot mis-order the title's.
//! * **It is already the house pattern.** `tabs/corkboard/card.rs`'s `CardNumber` does
//!   exactly this for board position, down to rendering *nothing* (not an empty label)
//!   when off, so a hidden number costs no layout and the titles do not shift.
//!
//! `CardNumber` and this are deliberately **not** merged: that one is "which card is this
//! on the board" (1-based within whatever the board currently shows, every sub_role
//! counted), this one is "which chapter is this in the book". Card #7 can be Chapter 3.

use teksilo::core::widget::Widget;
use teksilo::prelude::*;
use teksilo::tokens::TextRole;
use teksilo::widgets::{TextWidget, VStack};

/// The ordinal badge. `number` is fixed for one build: each row source recomputes the
/// whole manuscript's numbering on reload and bakes the result onto its row, and a
/// structural edit re-sources the view — so a stale badge cannot outlive the row it sits
/// on. There is no separate numbering model to observe.
///
/// `None` renders nothing at all, which is the common case: scenes, notes, folders, a
/// prologue the writer excluded, and every row of a manuscript with numbering switched
/// off. An empty label would still claim its slot's spacing and shift every title in the
/// tree by the width of a gap.
pub struct StructureNumber {
    pub number: Option<usize>,
    root: Option<WidgetId>,
}

impl StructureNumber {
    pub fn new(number: Option<usize>) -> Self {
        Self { number, root: None }
    }
}

impl std::fmt::Debug for StructureNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StructureNumber")
            .field("number", &self.number)
            .finish()
    }
}

impl Widget for StructureNumber {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        let id = match self.number {
            // Dimmed and small, like `CardNumber`: the number is orientation, not content,
            // and must never out-shout the title it sits beside. Semantic roles only, so
            // it stays legible in both themes.
            Some(n) => ctx.add(
                TextWidget::new(lit!(format!("{n}.")))
                    .style(TextStyleRole::Small)
                    .color(TextRole::Secondary),
            ),
            None => ctx.add(VStack::new()),
        };
        self.root = Some(id);
        vec![id]
    }

    fn layout_response(&self, p: SizeProposal, ctx: &LayoutContext) -> LayoutResponse {
        self.root
            .and_then(|id| ctx.child_size(id, p))
            .unwrap_or_else(|| p.resolve(0.0, 0.0))
            .into()
    }

    fn children(&self) -> Vec<WidgetId> {
        self.root.into_iter().collect()
    }
}
