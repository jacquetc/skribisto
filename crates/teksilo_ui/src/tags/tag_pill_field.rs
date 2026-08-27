// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! `TagPillField` — the item's tags as a wrapping flow of coloured chips ending in a "+"
//! popover. The Inspector's tag section, and (from Stage 4) the chip popover's body.
//!
//! Shape borrowed from `LanguagePillField`, minus the checkmark slot: each chip is a
//! [`Pill`] painted in the tag's own colour with a derived text colour,
//! removing on the hover `×` and showing name + description in a composite tooltip.
//!
//! The "+" popover both **assigns** an existing palette tag and **creates** one, because the
//! moment a writer wants a tag is the moment they notice it is missing — sending them to
//! Settings to make it and back here to apply it is the click-fest this feature exists to
//! avoid. Creation offers colour + the discoverable toggle inline: it is the switch the
//! whole story-bible half turns on, and burying it in a settings pane a writer has no
//! reason to visit would defeat it.
//!
//! It never knows *whose* tags it edits — the caller supplies the current ids and a writer.

use std::rc::Rc;

use teksilo::core::BindingLevel;
use teksilo::core::accesskit::Role;
use teksilo::core::widget::WidgetPlacement;
use teksilo::prelude::*;
use teksilo::tokens::Color;
use teksilo::widgets::{
    Button, ButtonVariant, ColorEdit, Divider, HStack, IconButton, IconWidget, MaxSize, Padding,
    Panel, PopoverIconButton, ScrollArea, TextInput, TextWidget, Toast, Toggle, VStack, Wrap,
};

use crate::app_ids::HasWorkId;
use crate::toast_scope::ToastWorkExt;

use crate::models::{TagRow, name_key};
use crate::tags::TagsViewModel;
use crate::tags::contrast;
use crate::tags::tag_tooltip::tag_tooltip_body;
use crate::widgets::{Pill, PillTooltip};

/// Persist a new tag-id list for the item. Takes an `EventContext` so it can run a command.
pub type SetTags = Rc<dyn Fn(Vec<u64>, &mut EventContext)>;

/// Colour given to a tag created from the "+" popover when the writer leaves the default.
/// Deliberately the same mid-slate the CSV importer falls back to: legible in both themes,
/// and visibly "unset" so Settings is still the place for a careful palette.
const QUICK_CREATE_COLOR: &str = "#607d8b";

fn default_create_color() -> Color {
    contrast::parse(QUICK_CREATE_COLOR)
}

pub struct TagPillField {
    /// The item's current tag ids (a local mirror the caller keeps in sync).
    value: Signal<Vec<u64>>,
    set: SetTags,
    vm: TagsViewModel,
    root_child: Option<WidgetId>,
}

impl TagPillField {
    pub fn new(value: Signal<Vec<u64>>, set: SetTags, vm: TagsViewModel) -> Self {
        Self {
            value,
            set,
            vm,
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
        // **Do not** rebuild on the picker's filter text — that lives inside TagPicker and
        // would recreate the TextInput, parking the caret at 0 / select-all every keystroke.
        self.value
            .bind_to(ctx.self_id(), ctx.binding_registry(), BindingLevel::Rebuild);
        self.vm.changed_signal().bind_to(
            ctx.self_id(),
            ctx.binding_registry(),
            BindingLevel::Rebuild,
        );

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

        let id = ctx.add(
            flow.access_role(Role::List)
                .access_label(tr!(tags_pill_list())),
        );
        self.root_child = Some(id);
        vec![id]
    }

    fn layout_response(&self, proposal: SizeProposal, ctx: &LayoutContext) -> LayoutResponse {
        // `Wrap` needs a concrete width to compute multi-line height. Measure
        // against the offered width (or a dock-ish fallback when the parent
        // left width open), then **claim that full width** so VStack places us
        // at the column width rather than the content-hug of the longest line
        // — otherwise place-time reflow can disagree with measure-time height
        // and the chips look stacked/clipped instead of flowing.
        let wrap_w = proposal.width.or(Some(280.0));
        let measured = self
            .root_child
            .and_then(|id| {
                ctx.child_size(
                    id,
                    SizeProposal {
                        width: wrap_w,
                        height: None,
                    },
                )
            })
            .unwrap_or(Size::ZERO);
        Size::new(proposal.width.unwrap_or(measured.width), measured.height).into()
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
/// **Filter typing must not rebuild this shell.** The query signal is held here and bound
/// only by the list/create children — if this widget rebuilt on every keystroke, the
/// `TextInput` would be recreated with caret at 0 / select-all, and typing would reverse.
///
/// [`TagDotsRow`]: crate::tags::TagDotsRow
pub(crate) struct TagPicker {
    query: Signal<String>,
    new_discoverable: Signal<bool>,
    new_color: Signal<Color>,
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
            new_color: Signal::new(default_create_color()),
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
        // No `query` Rebuild bind on this widget — see type docs.
        // Value/palette changes are handled by the list and create children so the
        // filter field survives assignment toggles that only need the list refreshed…
        // except TagPillField still rebuilds the whole popover when value changes, which
        // is fine (caret only matters while typing, not mid-toggle).

        let mut col = VStack::new().spacing(6.0);
        col = col.child(
            TextInput::new(self.query.clone())
                .placeholder(tr!(tags_pill_filter_placeholder()))
                .min_width(220.0),
        );

        col = col.child(TagPickerList {
            query: self.query.clone(),
            value: self.value.clone(),
            set: self.set.clone(),
            vm: self.vm.clone(),
            root_child: None,
        });

        col = col.child(TagPickerCreate {
            query: self.query.clone(),
            new_discoverable: self.new_discoverable.clone(),
            new_color: self.new_color.clone(),
            value: self.value.clone(),
            set: self.set.clone(),
            vm: self.vm.clone(),
            root_child: None,
        });

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

/// Filterable tick list — rebuilds on query / value / palette without touching the
/// filter field above it.
struct TagPickerList {
    query: Signal<String>,
    value: Signal<Vec<u64>>,
    set: SetTags,
    vm: TagsViewModel,
    root_child: Option<WidgetId>,
}

impl std::fmt::Debug for TagPickerList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TagPickerList").finish()
    }
}

impl Widget for TagPickerList {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        self.query
            .bind_to(ctx.self_id(), ctx.binding_registry(), BindingLevel::Rebuild);
        self.vm.changed_signal().bind_to(
            ctx.self_id(),
            ctx.binding_registry(),
            BindingLevel::Rebuild,
        );
        self.value
            .bind_to(ctx.self_id(), ctx.binding_registry(), BindingLevel::Rebuild);

        let q = self.query.get();
        let key = name_key(&q);
        let palette = self.vm.rows();
        let assigned = self.value.get();

        let matching: Vec<&TagRow> = palette
            .iter()
            .filter(|t| key.is_empty() || name_key(&t.name).contains(&key))
            .collect();

        let mut col = VStack::new().spacing(2.0);
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
                // Toggling does NOT clear the filter — a tick is a state you may want to flip
                // back immediately, and clearing the filter would lose the row just clicked.
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
        // `Role::ListBox`, not `Role::List`: the rows below declare
        // `Role::ListBoxOption` and carry a selected state, and an option
        // outside a listbox is an orphan role that AT reads as a bare group.
        col = col.child(
            MaxSize::height(220.0).child(
                ScrollArea::new().child(
                    list.access_role(Role::ListBox)
                        .access_label(tr!(tags_pick_list())),
                ),
            ),
        );

        let id = ctx.add(col);
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
}

/// Create-new-tag affordance: colour picker, story-bible toggle, Plain create button.
/// Rebuilds when the filter text / palette changes so the create row appears only for a
/// new name. Colour and discoverable signals are owned by the parent and survive rebuilds.
struct TagPickerCreate {
    query: Signal<String>,
    new_discoverable: Signal<bool>,
    new_color: Signal<Color>,
    value: Signal<Vec<u64>>,
    set: SetTags,
    vm: TagsViewModel,
    root_child: Option<WidgetId>,
}

impl std::fmt::Debug for TagPickerCreate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TagPickerCreate").finish()
    }
}

impl Widget for TagPickerCreate {
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
        let palette = self.vm.rows();
        // Create only when the typed name is new. Comparing case-insensitively against the
        // WHOLE palette, not just the unassigned ones: offering "Create «character»" when a
        // `character` tag already exists on this item would silently make a second one.
        let exact_exists = !key.is_empty() && palette.iter().any(|t| name_key(&t.name) == key);
        if key.is_empty() || exact_exists {
            // Empty placeholder so the parent still has a stable child slot.
            let id = ctx.add(VStack::new());
            self.root_child = Some(id);
            return vec![id];
        }

        let name = q.trim().to_string();
        let vm = self.vm.clone();
        let set = self.set.clone();
        let value = self.value.clone();
        let query = self.query.clone();
        let discoverable = self.new_discoverable.clone();
        let color = self.new_color.clone();
        let new_name = name.clone();
        let create: Rc<dyn Fn(&mut EventContext)> = Rc::new(move |c: &mut EventContext| {
            // Defensive: the create row is only shown when the name is new, but a
            // palette refresh mid-popover could land a collision. Refuse + toast
            // rather than minting a second tag with the same name.
            if let Some(existing) = vm.duplicate_name(&new_name, None) {
                c.show_toast(
                    Toast::warning(tr!(settings_tags_duplicate(name = existing)))
                        .scoped_id("tags.duplicate", vm.work_id())
                        .target_work(vm.work_id()),
                );
                return;
            }
            let hex = color.get().to_hex_lower(false);
            match vm.create(&new_name, &hex, "", discoverable.get()) {
                Some(id) => {
                    let mut next = value.get();
                    next.push(id);
                    value.set(next.clone());
                    set(next, c);
                    query.set(String::new());
                    discoverable.set(false);
                    color.set(default_create_color());
                }
                None => {
                    // `TagsViewModel::create` returns `None` only when there is no open
                    // Work to create against (see its own doc), a case the constructor-
                    // threaded handle every caller now gets should not reach in practice.
                    // But a silent no-op here used to look identical to a successful
                    // create: the popover cleared as if the tag existed. Toast, and leave
                    // the typed name/colour/toggle in place rather than resetting a form
                    // that did not actually submit, the same "refuse and say why" shape
                    // the duplicate-name guard above already uses.
                    c.show_toast(
                        Toast::warning(tr!(tags_pill_create_failed(name = new_name.clone())))
                            .scoped_id("tags.create-failed", vm.work_id())
                            .target_work(vm.work_id()),
                    );
                }
            }
        });

        let mut col = VStack::new().spacing(6.0);
        col = col.child(Divider::new());
        // Colour + create label on one row so the picker is obvious, not a hidden default.
        col = col.child(
            HStack::new()
                .spacing(8.0)
                .child(
                    ColorEdit::new(self.new_color.clone())
                        .swatches(teksilo::widgets::color_picker::DEFAULT_SWATCHES.to_vec()),
                )
                .child(
                    TextWidget::new(tr!(tags_pill_create(name = name.clone())))
                        .style(TextStyleRole::Small)
                        .color(TextRole::Secondary),
                ),
        );
        // Same registry key as the settings pane's switch, so the explanation cannot
        // drift between the two places this flag is offered.
        col = col.child(
            Toggle::new(self.new_discoverable.clone())
                .label(tr!(tags_pill_new_discoverable()))
                .rich_tooltip(crate::tooltip_registry::WM_FIND_IN_PROSE),
        );
        // The inline hint stays despite the tooltip: this is a creation form in a
        // transient popover, where a hover-only explanation is easy to never find.
        col = col.child(
            TextWidget::new(tr!(tags_pill_new_discoverable_hint()))
                .color(TextRole::Secondary)
                .style(TextStyleRole::Tiny),
        );
        col = col.child(
            Button::new(tr!(tags_pill_create(name = name.clone())))
                .variant(ButtonVariant::Plain)
                .on_activate_fn({
                    let create = create.clone();
                    move |c| create(c)
                }),
        );

        let id = ctx.add(col);
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

        // A focus stop with no visible ring: see `crate::widgets::focus_ring`.
        let focused = ctx.signal(false);
        let body = HStack::new()
            .spacing(6.0)
            .add_child(check_id)
            .child(swatch(contrast::parse(&self.tag.color)))
            .child(TextWidget::new(lit!(self.tag.name.clone())));
        let ringed =
            crate::widgets::with_focus_ring(ctx, crate::widgets::RING_RADIUS_ROW, body, &focused);
        let id = ctx.add(
            ringed
                // Role, name and selected go on the SAME node that is focusable -- the pattern
                // `MentionList`'s rows already use. Splitting them across the outer node and this
                // one leaves keyboard focus landing on an unnamed GenericContainer.
                .access_role(Role::ListBoxOption)
                .access_label(lit!(self.tag.name.clone()))
                .access_customize({
                    let checked = self.checked;
                    move |b| b.set_selected(checked)
                })
                .focusable(true)
                .on_focus({
                    let focused = focused.clone();
                    move |gained, _c| focused.set(gained)
                })
                .on_tap({
                    let on_toggle = on_toggle.clone();
                    move |_e, c| on_toggle(c)
                })
                // `on_tap` is pointer-only, so without this the row is Tab-reachable and
                // completely inert -- which is still WCAG 2.1.1: reaching a control you
                // cannot operate is not keyboard access. Enter and Space both tick, the
                // two keys a listbox option is expected to answer to.
                .on_key(move |ev, c| {
                    if let WidgetEvent::KeyDown { key, .. } = ev
                        && matches!(key, Key::Enter | Key::Space)
                    {
                        on_toggle(c);
                        return EventResponse::Handled;
                    }
                    EventResponse::Ignored
                }),
        );
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
}

/// A small round colour dot, used in the picker rows.
fn swatch(color: teksilo::tokens::Color) -> impl Widget {
    teksilo::widgets::MinSize::new(10.0, 10.0).child(
        teksilo::widgets::RectWidget::new()
            .background(color)
            .corner_radius(teksilo::tokens::CornerRadius::uniform(9999.0))
            // Derived, not a border token: see `contrast::outline_on`. A picker row for a
            // near-white tag is exactly where an invisible dot is most confusing, since the
            // row is what you use to tell tags apart.
            .border_color(contrast::outline_on(color))
            .border_width(1.0),
    )
}

#[cfg(test)]
mod flow_tests {
    use super::*;
    use frontend::AppContext;
    use std::rc::Rc;
    use teksilo::core::widget_tree::WidgetTree;

    /// TagPillField must reflow chips under a narrow proposal the same way
    /// LanguagePillField / AliasPillField do — a composing-widget bug that
    /// measures the Wrap under unbounded width would stack one chip per line
    /// or clip the flow.
    #[test]
    fn tag_pill_field_wraps_under_inspector_width() {
        let ctx = Rc::new(AppContext::new());
        let ids = crate::app_ids::AppIds::new();
        let list = crate::models::WorkTagsListModel::new(ctx, ids.clone());
        let vm = TagsViewModel::new(list, ids);
        // Assign every mock (or empty) palette tag; when mocks ship 6 tags we
        // need wrapping. With a real empty store, seed by creating if possible.
        let mut ids_vec: Vec<u64> = vm.rows().into_iter().map(|r| r.id).collect();
        if ids_vec.is_empty() {
            // Real backend with no open Work: skip meaningfully.
            for (i, name) in ["A", "B", "C", "D", "E", "F"].iter().enumerate() {
                if let Some(id) = vm.create(name, "#2980b9", "", false) {
                    ids_vec.push(id);
                } else {
                    // Can't create without a Work — still assert the field lays out.
                    let _ = i;
                }
            }
        }
        if ids_vec.len() < 3 {
            // Headless without Work can't seed tags; layout still must not panic.
            let field = TagPillField::new(Signal::new(ids_vec), Rc::new(|_, _| {}), vm);
            let mut tree = WidgetTree::new().with_theme(teksilo::presets::intui::light());
            let id = tree.add_boxed(Box::new(field));
            tree.layout(SizeProposal::exact(220.0, 400.0));
            assert!(tree.bounds(id).height > 0.0);
            return;
        }
        let field = TagPillField::new(Signal::new(ids_vec), Rc::new(|_, _| {}), vm);
        let mut tree = WidgetTree::new().with_theme(teksilo::presets::intui::light());
        let id = tree.add_boxed(Box::new(field));
        tree.layout(SizeProposal::exact(220.0, 400.0));
        let b = tree.bounds(id);
        assert!(
            b.height > 40.0,
            "several tag pills in a 220 dp inspector should wrap; height={}",
            b.height
        );
    }
}
