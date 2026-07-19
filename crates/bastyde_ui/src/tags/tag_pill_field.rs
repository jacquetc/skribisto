// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! `TagPillField` — the item's tags as a wrapping flow of coloured chips ending in a "+"
//! popover. The Inspector's tag section, and (from Stage 4) the chip popover's body.
//!
//! Shape borrowed from `LanguagePillField`, minus the checkmark slot: each chip is a
//! [`Pill`](crate::widgets::Pill) painted in the tag's own colour with a derived text colour,
//! removing on the hover `×` and showing name + description in a composite tooltip.
//!
//! The "+" popover both **assigns** an existing palette tag and **creates** one, because the
//! moment a writer wants a tag is the moment they notice it is missing — sending them to
//! Settings to make it and back here to apply it is the click-fest this feature exists to
//! avoid. Creation offers the discoverable toggle inline: it is the switch the whole
//! story-bible half turns on, and burying it in a settings pane a writer has no reason to
//! visit would defeat it.
//!
//! It never knows *whose* tags it edits — the caller supplies the current ids and a writer.

use std::rc::Rc;

use bastyde::core::BindingLevel;
use bastyde::core::accesskit::Role;
use bastyde::core::widget::WidgetPlacement;
use bastyde::prelude::*;
use bastyde::widgets::{
    Divider, HStack, IconButton, MaxSize, Padding, PopoverIconButton, ScrollArea, TextInput,
    TextWidget, Toggle, VStack, Wrap,
};

use crate::models::{TagRow, name_key};
use crate::tags::contrast;
use crate::tags::tag_tooltip::tag_tooltip_body;
use crate::view_models::TagsViewModel;
use crate::widgets::{Pill, PillTooltip};

/// Persist a new tag-id list for the item. Takes an `EventContext` so it can run a command.
pub type SetTags = Rc<dyn Fn(Vec<u64>, &mut EventContext)>;

/// Colour given to a tag created from the "+" popover, where there is no colour picker.
/// Deliberately the same mid-slate the CSV importer falls back to: legible in both themes,
/// and visibly "unset" so the writer knows Settings is where to choose one.
const QUICK_CREATE_COLOR: &str = "#607d8b";

pub struct TagPillField {
    /// The item's current tag ids (a local mirror the caller keeps in sync).
    value: Signal<Vec<u64>>,
    set: SetTags,
    vm: TagsViewModel,
    /// Filter/new-name text in the "+" popover.
    query: Signal<String>,
    /// Whether a tag created from the popover is story-bible material.
    new_discoverable: Signal<bool>,
    root_child: Option<WidgetId>,
}

impl TagPillField {
    pub fn new(value: Signal<Vec<u64>>, set: SetTags, vm: TagsViewModel) -> Self {
        Self {
            value,
            set,
            vm,
            query: Signal::new(String::new()),
            new_discoverable: Signal::new(false),
            root_child: None,
        }
    }
}

impl std::fmt::Debug for TagPillField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TagPillField").finish()
    }
}

impl Widget for TagPillField {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        // Rebuild when the item's tags change (an assign/remove) or the palette does (a
        // rename, a recolour, a preset applied from Settings while this is on screen).
        self.value
            .bind_to(ctx.self_id(), ctx.binding_registry(), BindingLevel::Rebuild);
        self.vm.changed_signal().bind_to(
            ctx.self_id(),
            ctx.binding_registry(),
            BindingLevel::Rebuild,
        );
        self.query
            .bind_to(ctx.self_id(), ctx.binding_registry(), BindingLevel::Rebuild);

        let palette = self.vm.rows();
        let assigned_ids = self.value.get();

        let mut flow = Wrap::new().spacing(6.0).line_spacing(6.0);

        // Chips in palette order (alphabetical) rather than assignment order, so the same
        // set of tags always looks the same wherever it is shown.
        for tag in palette.iter().filter(|t| assigned_ids.contains(&t.id)) {
            let fill = contrast::parse(&tag.color);
            let on_remove: Rc<dyn Fn(&mut EventContext)> = {
                let set = self.set.clone();
                let value = self.value.clone();
                let id = tag.id;
                Rc::new(move |c| {
                    let next: Vec<u64> = value.get().into_iter().filter(|t| *t != id).collect();
                    value.set(next.clone());
                    set(next, c);
                })
            };
            flow = flow.child(
                Pill::new(tag.name.clone(), lit!(tag.name.clone()))
                    .background(fill)
                    .outline(contrast::outline_on(fill))
                    .text_color(contrast::text_on(fill))
                    .tooltip(PillTooltip::Composite {
                        body: Box::new(tag_tooltip_body(tag)),
                        access_label: lit!(tag.name.clone()),
                    })
                    .on_remove(tr!(tags_pill_remove(name = tag.name.clone())), move |c| {
                        on_remove(c)
                    }),
            );
        }

        let picker = TagPicker {
            query: self.query.clone(),
            new_discoverable: self.new_discoverable.clone(),
            palette,
            assigned: assigned_ids,
            value: self.value.clone(),
            set: self.set.clone(),
            vm: self.vm.clone(),
            root_child: None,
        };
        // `.bare()`: the picker draws its own panel, so skip the popover's second chrome.
        flow = flow.child(
            PopoverIconButton::new(IconButton::add().tooltip(tr!(tags_pill_add())))
                .bare()
                .content(picker),
        );

        let id = ctx.add(flow.access_role(Role::List).access_label(tr!(tags_pill_list())));
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

/// The "+" popover body: filter, unassigned matches, and a create row.
struct TagPicker {
    query: Signal<String>,
    new_discoverable: Signal<bool>,
    palette: Vec<TagRow>,
    assigned: Vec<u64>,
    value: Signal<Vec<u64>>,
    set: SetTags,
    vm: TagsViewModel,
    root_child: Option<WidgetId>,
}

impl std::fmt::Debug for TagPicker {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TagPicker").finish()
    }
}

impl Widget for TagPicker {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        self.query
            .bind_to(ctx.self_id(), ctx.binding_registry(), BindingLevel::Rebuild);
        self.vm.changed_signal().bind_to(
            ctx.self_id(),
            ctx.binding_registry(),
            BindingLevel::Rebuild,
        );

        let q = self.query.get();
        let key = name_key(&q);

        let mut col = VStack::new().spacing(6.0);
        col = col.child(
            TextInput::new(self.query.clone())
                .placeholder(tr!(tags_pill_filter_placeholder()))
                .min_width(220.0),
        );

        let unassigned: Vec<&TagRow> = self
            .palette
            .iter()
            .filter(|t| !self.assigned.contains(&t.id))
            .filter(|t| key.is_empty() || name_key(&t.name).contains(&key))
            .collect();

        if unassigned.is_empty() && !key.is_empty() {
            col = col.child(
                Padding::symmetric(4.0, 6.0)
                    .child(TextWidget::new(tr!(tags_pill_no_match())).color(TextRole::Secondary)),
            );
        }

        let mut list = VStack::new().spacing(2.0);
        for tag in unassigned {
            let fill = contrast::parse(&tag.color);
            let set = self.set.clone();
            let value = self.value.clone();
            let id = tag.id;
            let query = self.query.clone();
            list = list.child(
                HStack::new()
                    .spacing(6.0)
                    .child(swatch(fill))
                    .child(TextWidget::new(lit!(tag.name.clone())))
                    .access_role(Role::ListItem)
                    .access_label(lit!(tag.name.clone()))
                    .focusable(true)
                    .on_tap(move |_e, c| {
                        let mut next = value.get();
                        if !next.contains(&id) {
                            next.push(id);
                        }
                        value.set(next.clone());
                        set(next, c);
                        // Clear the filter so the popover is ready for the next pick rather
                        // than still showing a one-item list.
                        query.set(String::new());
                    }),
            );
        }
        // Capped and scrollable: a project with forty tags must not grow a popover taller
        // than the window. `MaxSize` does the capping — `ScrollArea` has no height setter of
        // its own and would otherwise take whatever the overlay proposes.
        col = col.child(MaxSize::height(220.0).child(ScrollArea::new().child(list)));

        // Create, only when the typed name is new. Comparing case-insensitively against the
        // WHOLE palette, not just the unassigned ones: offering "Create «character»" when a
        // `character` tag already exists on this item would silently make a second one.
        let exact_exists = !key.is_empty()
            && self
                .palette
                .iter()
                .any(|t| name_key(&t.name) == key);
        if !key.is_empty() && !exact_exists {
            let vm = self.vm.clone();
            let set = self.set.clone();
            let value = self.value.clone();
            let query = self.query.clone();
            let discoverable = self.new_discoverable.clone();
            let name = q.trim().to_string();
            col = col.child(Divider::new());
            // Same registry key as the settings pane's switch, so the explanation cannot
            // drift between the two places this flag is offered.
            col = col.child(
                Toggle::new(self.new_discoverable.clone())
                    .label(tr!(tags_pill_new_discoverable()))
                    .rich_tooltip(crate::tooltip_registry::WM_STORY_BIBLE),
            );
            // The inline hint stays despite the tooltip: this is a creation form in a
            // transient popover, where a hover-only explanation is easy to never find. The
            // hint says what the switch does; the tooltip's disclosure teaches why.
            col = col.child(
                TextWidget::new(tr!(tags_pill_new_discoverable_hint()))
                    .color(TextRole::Secondary)
                    .style(TextStyleRole::Tiny),
            );
            col = col.child(
                HStack::new()
                    .spacing(6.0)
                    .child(swatch(contrast::parse(QUICK_CREATE_COLOR)))
                    .child(TextWidget::new(tr!(tags_pill_create(name = name.clone()))))
                    .access_role(Role::Button)
                    .access_label(tr!(tags_pill_create(name = name.clone())))
                    .focusable(true)
                    .on_tap(move |_e, c| {
                        if let Some(id) = vm.create(
                            &name,
                            QUICK_CREATE_COLOR,
                            "",
                            discoverable.get(),
                        ) {
                            let mut next = value.get();
                            next.push(id);
                            value.set(next.clone());
                            set(next, c);
                        }
                        query.set(String::new());
                        discoverable.set(false);
                    }),
            );
        }

        let id = ctx.add(
            bastyde::widgets::Panel::new()
                .child(Padding::uniform(8.0).child(col))
                .access_role(Role::Dialog)
                .access_label(tr!(tags_pill_add())),
        );
        self.root_child = Some(id);
        vec![id]
    }

    fn layout_response(&self, proposal: SizeProposal, ctx: &LayoutContext) -> LayoutResponse {
        // Popover content: size to the panel, never to the (unbounded) overlay proposal.
        self.root_child
            .and_then(|id| ctx.child_size(id, proposal))
            .map(LayoutResponse::from)
            .unwrap_or_else(|| proposal.resolve(0.0, 0.0).into())
    }

    fn children(&self) -> Vec<WidgetId> {
        self.root_child.into_iter().collect()
    }
}

/// A small round colour dot, used in the picker rows.
fn swatch(color: bastyde::tokens::Color) -> impl Widget {
    bastyde::widgets::MinSize::new(10.0, 10.0).child(
        bastyde::widgets::RectWidget::new()
            .background(color)
            .corner_radius(bastyde::tokens::CornerRadius::uniform(9999.0))
            .border_color(BorderRole::Default)
            .border_width(1.0),
    )
}
