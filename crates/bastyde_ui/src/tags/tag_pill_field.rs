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
    Divider, HStack, IconButton, IconWidget, MaxSize, Padding, Panel, PopoverIconButton,
    ScrollArea, TextInput, TextWidget, Toggle, VStack, Wrap,
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

        // `.bare()` + an explicit panel: `PopoverIconButton`'s own chrome is skipped and this
        // call site supplies it, because the picker itself is now bare so that
        // `TagDotsRow` can drop it straight into a `Popover`, which brings its own surface.
        let picker = Panel::new()
            .child(Padding::uniform(8.0).child(TagPicker::new(
                self.value.clone(),
                self.set.clone(),
                self.vm.clone(),
            )))
            .access_role(Role::Dialog)
            .access_label(tr!(tags_pill_add()));
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

/// The tag popover body: filter, one tickable row per palette tag, and a create row.
///
/// Bare on purpose — it draws no panel of its own. Both call sites wrap it: the "+" button
/// with an explicit `Panel` (its `PopoverIconButton` is `.bare()`), and [`TagDotsRow`] by
/// handing it to a `Popover`, which supplies a themed surface. Self-chroming would nest a
/// second panel inside that surface.
///
/// Rows **tick** rather than only add. The pill field can unassign with each chip's `×`, but
/// a dot row has no such affordance — from a corkboard card the popover is the only way to
/// take a tag off, so it has to be able to. One list that toggles serves both, and needs no
/// mode flag.
///
/// [`TagDotsRow`]: crate::tags::TagDotsRow
pub(crate) struct TagPicker {
    query: Signal<String>,
    new_discoverable: Signal<bool>,
    value: Signal<Vec<u64>>,
    set: SetTags,
    vm: TagsViewModel,
    root_child: Option<WidgetId>,
}

impl TagPicker {
    pub(crate) fn new(value: Signal<Vec<u64>>, set: SetTags, vm: TagsViewModel) -> Self {
        Self {
            query: Signal::new(String::new()),
            new_discoverable: Signal::new(false),
            value,
            set,
            vm,
            root_child: None,
        }
    }
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
        // Also the item's own tags: a tick has to flip the moment it is clicked, and the
        // popover stays open across several toggles.
        self.value
            .bind_to(ctx.self_id(), ctx.binding_registry(), BindingLevel::Rebuild);

        let q = self.query.get();
        let key = name_key(&q);
        let palette = self.vm.rows();
        let assigned = self.value.get();

        let mut col = VStack::new().spacing(6.0);
        col = col.child(
            TextInput::new(self.query.clone())
                .placeholder(tr!(tags_pill_filter_placeholder()))
                .min_width(220.0),
        );

        // Every palette tag, ticked or not — not just the unassigned ones, so the same list
        // both adds and removes.
        let matching: Vec<&TagRow> = palette
            .iter()
            .filter(|t| key.is_empty() || name_key(&t.name).contains(&key))
            .collect();

        if matching.is_empty() && !key.is_empty() {
            col = col.child(
                Padding::symmetric(4.0, 6.0)
                    .child(TextWidget::new(tr!(tags_pill_no_match())).color(TextRole::Secondary)),
            );
        }

        let mut list = VStack::new().spacing(2.0);
        for tag in matching {
            let set = self.set.clone();
            let value = self.value.clone();
            let id = tag.id;
            let on = assigned.contains(&id);
            list = list.child(TagPickRow {
                tag: (*tag).clone(),
                checked: on,
                // Toggling does NOT clear the filter. The old add-only list cleared it so the
                // popover was ready for the next pick, but a tick is a state you may want to
                // flip back immediately, and losing the row you just clicked makes that
                // impossible.
                on_toggle: Rc::new(move |c: &mut EventContext| {
                    let mut next = value.get();
                    if on {
                        next.retain(|t| *t != id);
                    } else if !next.contains(&id) {
                        next.push(id);
                    }
                    value.set(next.clone());
                    set(next, c);
                }),
                root_child: None,
            });
        }
        // Capped and scrollable: a project with forty tags must not grow a popover taller
        // than the window. `MaxSize` does the capping — `ScrollArea` has no height setter of
        // its own and would otherwise take whatever the overlay proposes.
        col = col.child(MaxSize::height(220.0).child(ScrollArea::new().child(list)));

        // Create, only when the typed name is new. Comparing case-insensitively against the
        // WHOLE palette, not just the unassigned ones: offering "Create «character»" when a
        // `character` tag already exists on this item would silently make a second one.
        let exact_exists = !key.is_empty() && palette.iter().any(|t| name_key(&t.name) == key);
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

        // Bare: no panel, no `Role::Dialog`. Whoever mounts this supplies the surface — see
        // the type docs.
        let id = ctx.add(col);
        self.root_child = Some(id);
        vec![id]
    }

    fn layout_response(&self, proposal: SizeProposal, ctx: &LayoutContext) -> LayoutResponse {
        // Popover content: size to the content, never to the (unbounded) overlay proposal.
        self.root_child
            .and_then(|id| ctx.child_size(id, proposal))
            .map(LayoutResponse::from)
            .unwrap_or_else(|| proposal.resolve(0.0, 0.0).into())
    }

    fn children(&self) -> Vec<WidgetId> {
        self.root_child.into_iter().collect()
    }
}

/// One tickable palette row: check, swatch, name.
///
/// Its own `Widget` rather than a decorated `HStack` because the tick has to reach the
/// accessibility tree as *state*, not as a glyph: `accessibility` reports
/// `Role::ListBoxOption` + `set_selected`, the shape `SearchField`'s suggestion rows already
/// use. Encoding it in the label instead ("✓ character") would read as a name, not a
/// toggle, and would not update when the state flips.
struct TagPickRow {
    tag: TagRow,
    checked: bool,
    on_toggle: Rc<dyn Fn(&mut EventContext)>,
    root_child: Option<WidgetId>,
}

impl std::fmt::Debug for TagPickRow {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TagPickRow")
            .field("tag", &self.tag.name)
            .field("checked", &self.checked)
            .finish()
    }
}

impl Widget for TagPickRow {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        let on_toggle = self.on_toggle.clone();
        // The check is always laid out and merely faded, so ticking a row never shifts the
        // names beside it — the same trick the language pill's checkmark uses.
        let check = IconWidget::checkmark(11.0).color(TextRole::Accent);
        let check_id = ctx.add(check);
        ctx.set_opacity(check_id, if self.checked { 1.0 } else { 0.0 });

        let id = ctx.add(
            HStack::new()
                .spacing(6.0)
                .add_child(check_id)
                .child(swatch(contrast::parse(&self.tag.color)))
                .child(TextWidget::new(lit!(self.tag.name.clone())))
                .focusable(true)
                .on_tap(move |_e, c| on_toggle(c)),
        );
        self.root_child = Some(id);
        vec![id]
    }

    fn accessibility(&self, builder: &mut bastyde::core::accessibility::AccessNodeBuilder) {
        builder.set_role(Role::ListBoxOption);
        builder.set_name(self.tag.name.clone());
        builder.set_selected(self.checked);
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
}

/// A small round colour dot, used in the picker rows.
fn swatch(color: bastyde::tokens::Color) -> impl Widget {
    bastyde::widgets::MinSize::new(10.0, 10.0).child(
        bastyde::widgets::RectWidget::new()
            .background(color)
            .corner_radius(bastyde::tokens::CornerRadius::uniform(9999.0))
            // Derived, not a border token: see `contrast::outline_on`. A picker row for a
            // near-white tag is exactly where an invisible dot is most confusing, since the
            // row is what you use to tell tags apart.
            .border_color(contrast::outline_on(color))
            .border_width(1.0),
    )
}
