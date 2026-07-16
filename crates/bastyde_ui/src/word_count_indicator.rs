// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! `WordCountIndicator` — the status bar's live word count of the focused item.
//!
//! A quiet, secondary-coloured count sitting after the save glyph, showing the words in
//! the scene the writer is editing. It appears only when a project is open and something
//! prose-bearing is focused (a container tab / an unopened item shows nothing and takes no
//! width). The count is one scene, cheap to recompute, so it tracks typing live — no
//! spinner-style hysteresis. Thin per the house rules: the decision is the pure
//! [`crate::view_models::count_display`] table and the number comes from [`StatsModel`].

use bastyde::core::BindingLevel;
use bastyde::prelude::*;
use bastyde::widgets::TextWidget;

use crate::models::StatsModel;
use crate::view_models::{CountDisplay, count_display};

pub struct WordCountIndicator {
    stats: StatsModel,
    /// A project is open at all (same test the save indicator uses).
    has_work: Signal<bool>,
    /// Show the character count beside the words (`goals.show_characters`).
    show_characters: Signal<bool>,
    root_child: Option<WidgetId>,
}

impl WordCountIndicator {
    pub fn new(stats: StatsModel, has_work: Signal<bool>, show_characters: Signal<bool>) -> Self {
        Self {
            stats,
            has_work,
            show_characters,
            root_child: None,
        }
    }

    fn render(&self, ctx: &mut BuildContext, display: CountDisplay) -> Option<WidgetId> {
        let label = match display {
            CountDisplay::Hidden => return None,
            CountDisplay::Words(n) => tr!(statusbar_word_count(count = n as i64)),
            CountDisplay::WordsChars { words, chars } => {
                tr!(statusbar_word_char_count(words = words as i64, chars = chars as i64))
            }
        };
        Some(ctx.add(TextWidget::new(label).color(TextRole::Secondary).single_line()))
    }
}

impl std::fmt::Debug for WordCountIndicator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WordCountIndicator").finish()
    }
}

impl Widget for WordCountIndicator {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        let sid = ctx.self_id();
        let reg = ctx.binding_registry();
        // Focus change → recount immediately; any edit → recount (cheap, one scene);
        // a counting-method or show-characters change also re-derives (the status bar
        // sits behind the Settings modal, so nothing else would rebuild it).
        self.has_work.bind_to(sid, reg, BindingLevel::Rebuild);
        self.stats.active_item().bind_to(sid, reg, BindingLevel::Rebuild);
        self.stats.edited_signal().bind_to(sid, reg, BindingLevel::Rebuild);
        self.stats.method_signal().bind_to(sid, reg, BindingLevel::Rebuild);
        self.show_characters.bind_to(sid, reg, BindingLevel::Rebuild);

        let focused = self.stats.focused_counts().map(|c| (c.words, c.chars_with_spaces));
        let display = count_display(self.has_work.get(), focused, self.show_characters.get());
        self.root_child = self.render(ctx, display);
        self.root_child.into_iter().collect()
    }

    /// Zero-sized when hidden (no project / a container tab), so it takes no width of
    /// its own there — the surrounding `HStack` spacing still leaves a small gap.
    fn layout_response(&self, proposal: SizeProposal, ctx: &LayoutContext) -> LayoutResponse {
        self.root_child
            .and_then(|id| ctx.child_size(id, proposal))
            .map(LayoutResponse::from)
            .unwrap_or_else(|| proposal.resolve(0.0, 0.0).into())
    }
}
