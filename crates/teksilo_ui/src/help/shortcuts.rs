// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Help ▸ Keyboard shortcuts — a read-only sheet of every chord the app has.
//!
//! **It writes no new prose and it cannot go stale.** Every row is read live from the
//! tree's `ShortcutRegistry`: the name is the one the command registered (already
//! translated, because it came from `tr!` at registration), the chord is the *effective*
//! one with the reader's own rebinds merged in, and the grouping is the registry's own
//! deterministic `(category, id)` order. A shortcut added, removed or rebound anywhere
//! in the app appears here correctly without anyone editing this file.
//!
//! That is the whole reason this is a separate surface from the printed-word help: it
//! is generated from truth rather than authored and hoped to stay true.
//!
//! ## Why not just open Settings ▸ Keymap
//!
//! Settings ▸ Keymap renders the same data through teksilo's `ShortcutSettings`, but it
//! is a *rebinding* surface: every row carries Rebind / Unbind / Reset and a conflict
//! prompt. Someone who pressed F1 wants to read, and handing them a page where every
//! row is a live control invites an accidental rebind. The Rebind button at the bottom
//! of this sheet takes them there when changing a chord is what they actually wanted.

use teksilo::core::modal::{ModalCloseBehavior, ModalPresentation, ModalRequest};
use teksilo::core::styles::PanelVariant;
use teksilo::prelude::*;
use teksilo::widgets::keystroke_format::format_keystroke;
use teksilo::widgets::{
    Button, ButtonVariant, Divider, Expand, HStack, Padding, Panel, ScrollArea, SearchField,
    Spacer, TextWidget, VStack,
};

const CARD_W: u32 = 560;
const CARD_H: u32 = 620;

/// Present the shortcuts sheet. Fired by the `help.shortcuts` global action.
pub fn present_shortcuts(ctx: &mut EventContext) {
    ctx.present_modal(
        ModalRequest::deferred(|t| t.add(ShortcutSheet::new()))
            .presentation(ModalPresentation::InTree)
            .title(tr!(help_shortcuts_title()))
            .size(CARD_W, CARD_H)
            .close_behavior(ModalCloseBehavior::EscapeOrClickOutside),
    );
}

struct ShortcutSheet {
    filter: Signal<String>,
    root_child: Option<WidgetId>,
}

impl ShortcutSheet {
    fn new() -> Self {
        Self {
            filter: Signal::new(String::new()),
            root_child: None,
        }
    }
}

impl std::fmt::Debug for ShortcutSheet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ShortcutSheet").finish_non_exhaustive()
    }
}

/// One row's data, lifted out of the registry so the borrow ends before any widget is
/// built.
struct Row {
    name: String,
    category: Option<&'static str>,
    chord: Option<String>,
}

impl Widget for ShortcutSheet {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        let sid = ctx.self_id();
        let reg = ctx.binding_registry();
        self.filter
            .bind_to(sid, reg, teksilo::core::binding::BindingLevel::Rebuild);
        // A rebind made in Settings while this sheet is open must reach it.
        ctx.shortcut_version()
            .bind_to(sid, reg, teksilo::core::binding::BindingLevel::Rebuild);

        let needle = self.filter.get().trim().to_lowercase();
        let rows: Vec<Row> = ctx
            .shortcut_registry()
            .iter_effective()
            .map(|eff| Row {
                name: eff.shortcut.name.get(),
                category: eff.shortcut.category,
                chord: eff.primary.map(format_keystroke),
            })
            // A command with no chord belongs in the command palette, not on a page
            // titled "Keyboard shortcuts": listing it here with a blank column would
            // read as a shortcut nobody could discover rather than as a command that
            // has none.
            .filter(|row| row.chord.is_some())
            .filter(|row| {
                needle.is_empty()
                    || row.name.to_lowercase().contains(&needle)
                    || row
                        .category
                        .is_some_and(|c| c.to_lowercase().contains(&needle))
                    || row
                        .chord
                        .as_ref()
                        .is_some_and(|c| c.to_lowercase().contains(&needle))
            })
            .collect();

        let mut list = VStack::new().spacing(0.0);
        let mut last_category: Option<Option<&'static str>> = None;
        for row in &rows {
            if last_category != Some(row.category) {
                let label = row.category.unwrap_or("");
                list = list.child(
                    Padding::new(14.0, 6.0, 4.0, 6.0).child(
                        TextWidget::new(lit!(label.to_string()))
                            .style(TextStyleRole::SmallBold)
                            .color(TextRole::Secondary),
                    ),
                );
                last_category = Some(row.category);
            }
            list = list.child(
                Padding::symmetric(4.0, 6.0).child(
                    HStack::new()
                        .spacing(12.0)
                        .child(TextWidget::new(lit!(row.name.clone())).style(TextStyleRole::Body))
                        .child(Spacer::new())
                        .child(
                            TextWidget::new(lit!(row.chord.clone().unwrap_or_default()))
                                .style(TextStyleRole::Mono)
                                .color(TextRole::Secondary)
                                .single_line(),
                        ),
                ),
            );
        }

        let empty = rows.is_empty().then(|| {
            Padding::symmetric(14.0, 14.0).child(
                TextWidget::new(tr!(help_shortcuts_no_matches()))
                    .style(TextStyleRole::Body)
                    .color(TextRole::Secondary),
            )
        });

        let mut body = VStack::new()
            .spacing(0.0)
            .child(Padding::symmetric(8.0, 12.0).child(
                SearchField::new(self.filter.clone()).placeholder(tr!(help_shortcuts_filter())),
            ));
        if let Some(empty) = empty {
            body = body.child(empty);
        }
        body = body.child(Expand::new().child(ScrollArea::new().child(list)));

        let footer = HStack::new()
            .spacing(8.0)
            .child(
                Button::new(tr!(help_shortcuts_rebind()))
                    .variant(ButtonVariant::Plain)
                    .on_activate_fn(|ctx| {
                        // Dismiss first: Settings is itself a modal, and stacking one
                        // on the other leaves the reader two Escapes from their work.
                        ctx.dismiss_modal();
                        ctx.send_intent(teksilo::core::intent::Intent::new("app.settings.keymap"));
                    }),
            )
            .child(Spacer::new())
            .child(
                Button::new(tr!(help_shortcuts_close()))
                    .variant(ButtonVariant::Filled)
                    .on_activate_fn(|ctx| ctx.dismiss_modal()),
            );

        let root = ctx.add(
            Panel::new()
                .variant(PanelVariant::Raised)
                .corner_radius(10.0)
                .padding(0.0)
                .child(
                    VStack::new()
                        .spacing(0.0)
                        .child(Expand::new().child(body))
                        .child(Divider::new())
                        .child(Padding::symmetric(10.0, 12.0).child(footer)),
                ),
        );
        self.root_child = Some(root);
        vec![root]
    }

    fn layout_response(
        &self,
        proposal: SizeProposal,
        ctx: &LayoutContext,
    ) -> teksilo::core::widget::LayoutResponse {
        self.root_child
            .and_then(|id| ctx.child_size(id, proposal))
            .map(Into::into)
            .unwrap_or_else(|| proposal.resolve(0.0, 0.0).into())
    }

    fn children(&self) -> Vec<WidgetId> {
        self.root_child.into_iter().collect()
    }
}
