// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The **About this import** dialog: the report of a finished Scrivener import (D7-8).
//!
//! What came across, what the writer chose, then the findings in four fixed groups, each
//! a disclosure (the two that need the writer open); each line is a sentence with the
//! items it is about behind a disclosure of its own, in Scrivener's binder order. **Copy
//! report**, **Open now** and **Close** close it.
//!
//! Thin, per the house rules: it binds [`ImportReportViewModel`]'s models and calls its
//! methods. Lists are `Repeater`s over the view-model's models: a report holds a handful of
//! groups and lines, and a line lists at most fifty items.
//!
//! * **Every disclosure is a button controlling a named region** (D7-15): a group's, and
//!   a line's list of items, whose button and region a screen reader hears with the line
//!   they belong to ("3 items: …"), not three "3 items" in a row.
//! * **What a closed disclosure holds is out of reach** ([`Disclosure`]): no Tab stop and
//!   nothing read in it until it opens, though it stays laid out to close with the
//!   animation.
//! * **Nothing sets the column's width but the dialog**: an item opening its row is a link
//!   that wraps like the rest ([`WrappingLink`]), so a long binder path cannot widen the
//!   report past its card and push every line off its right edge.
//! * **Focus opens on Copy report**, never on the header's close cross, which a first
//!   Enter would press.
//! * **A disclosure opens in a window without focus too.** Each body opens in a
//!   `Collapse`, whose height animates. Teksilo before 0.15.1 held every animation of a
//!   window without focus, so a group or a list opened there, by an automation client, an
//!   assistive technology or a click landing before the window took focus, said it was
//!   expanded and stayed at no height.

use teksilo::core::accesskit::{Live, Role};
use teksilo::core::modal::{ModalCloseBehavior, ModalPresentation, ModalRequest};
use teksilo::core::styles::{LinkStyle, LinkStyleConfig, PanelVariant};
use teksilo::data::ListModel;
use teksilo::i18n::{LocalizedString, localized};
use teksilo::prelude::*;
use teksilo::tokens::{BorderRole, CornerRadius};
use teksilo::widgets::styles::recipe_link_style::link_text_role;
use teksilo::widgets::{
    Accordion, Button, ButtonVariant, Collapse, Divider, Expand, FixedSize, HStack, IconButton,
    Link, Padding, Panel, RectWidget, Repeater, ScrollArea, Shrinkable, TextWidget, VStack,
};

use super::report::{Listed, ReportGroup, ReportItem, ReportLine};
use super::report_vm::ImportReportViewModel;

/// The dialog's preferred size (D7-8), and the floor it does not shrink past in a smaller
/// window.
const REPORT_W: f32 = 640.0;
const REPORT_H: f32 = 540.0;
/// Wide enough for the footer's buttons in the longest shipped language (French).
const MIN_REPORT_W: f32 = 440.0;
const MIN_REPORT_H: f32 = 320.0;
/// The gutter left between a shrunk dialog and the window's edge.
const VIEWPORT_MARGIN: f32 = 24.0;

/// The dialog's size in a slot of `slot_w` × `slot_h`: the preferred size when it fits,
/// else the slot less the gutter, never under the floor nor over the slot.
fn report_size(slot_w: f32, slot_h: f32) -> (f32, f32) {
    fn fit(slot: f32, preferred: f32, floor: f32) -> f32 {
        if slot >= preferred {
            preferred
        } else {
            (slot - VIEWPORT_MARGIN).max(floor).min(slot)
        }
    }
    (
        fit(slot_w, REPORT_W, MIN_REPORT_W),
        fit(slot_h, REPORT_H, MIN_REPORT_H),
    )
}

/// Show the report `vm` holds over the window that asked for it.
pub(crate) fn present_import_report(ctx: &mut EventContext, vm: ImportReportViewModel) {
    ctx.present_modal(
        ModalRequest::deferred(move |t| t.add(ImportReportPanel::new(vm.clone())))
            .presentation(ModalPresentation::InTree)
            .close_behavior(ModalCloseBehavior::EscapeOrClickOutside)
            .title(tr!(project_import_report_title()))
            .size(REPORT_W as u32, REPORT_H as u32),
    );
}

pub struct ImportReportPanel {
    vm: ImportReportViewModel,
    root_child: Option<WidgetId>,
    /// **Copy report**, where focus opens ([`Widget::initial_focus_hint`]).
    copy_button: Option<WidgetId>,
    w: Signal<f32>,
    h: Signal<f32>,
}

impl ImportReportPanel {
    pub fn new(vm: ImportReportViewModel) -> Self {
        Self {
            vm,
            root_child: None,
            copy_button: None,
            w: Signal::new(REPORT_W),
            h: Signal::new(REPORT_H),
        }
    }
}

impl std::fmt::Debug for ImportReportPanel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ImportReportPanel").finish()
    }
}

impl Widget for ImportReportPanel {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        let body = report_body(&self.vm);
        let copy_vm = self.vm.clone();
        let copy = ctx.add(
            Button::new(tr!(import_scrivener_copy_report()))
                .variant(ButtonVariant::Plain)
                .on_activate_fn(move |c| copy_vm.copy(c)),
        );
        self.copy_button = Some(copy);
        let footer = footer(&self.vm, copy);
        let root = teksu!(ctx => FixedSize {
            width: self.w.clone()
            height: self.h.clone()
            Panel {
                variant: PanelVariant::Raised
                corner_radius: 10.0
                padding: 0.0
                VStack {
                    spacing: 0.0
                    // `Expand::horizontal` around each bar: a height-only `FixedSize`
                    // reports its child's own width, and the bar would not span the card.
                    Expand::horizontal {
                        FixedSize {
                            height: 44.0
                            Padding::symmetric(8.0, 18.0) {
                                HStack {
                                    spacing: 8.0
                                    Expand::horizontal {
                                        TextWidget::new(tr!(project_import_report_title())) {
                                            style: TextStyleRole::BodyBold
                                            color: TextRole::Primary
                                        }
                                    }
                                    IconButton::clear() {
                                        tooltip: tr!(import_scrivener_report_close())
                                        on_activate_fn: |c| c.dismiss_modal()
                                    }
                                }
                            }
                        }
                    }
                    Expand::horizontal {
                        Divider
                    }
                    Expand::vertical {
                        ScrollArea {
                            Padding::symmetric(16.0, 20.0) {
                                child: body
                            }
                        }
                    }
                    Expand::horizontal {
                        Divider
                    }
                    Expand::horizontal {
                        FixedSize {
                            height: 52.0
                            Padding::symmetric(10.0, 18.0) {
                                child: footer
                            }
                        }
                    }
                }
            }
        });
        self.root_child = Some(root);
        vec![root]
    }

    /// Focus opens on **Copy report**. Left to the modal's own fallback, the first focus
    /// stop in tree order, it would open on the header's close cross, and the first Enter
    /// would dismiss the report the writer just asked for (D7-15).
    fn initial_focus_hint(&self) -> Option<WidgetId> {
        self.copy_button
    }

    /// Announce the dialog by its title: `present_modal` does not wrap a hand-drawn panel
    /// in a container that would.
    fn accessibility(&self, builder: &mut teksilo::core::accessibility::AccessNodeBuilder) {
        builder.set_role(Role::Dialog);
        builder.set_name(tr!(project_import_report_title()).resolve_now());
    }

    /// Always the preferred size, so the slot the overlay hands back is
    /// `min(preferred, window)` on every pass; the dialog then takes what fits of it.
    fn layout_response(&self, proposal: SizeProposal, _ctx: &LayoutContext) -> LayoutResponse {
        if let (Some(slot_w), Some(slot_h)) = (proposal.width, proposal.height) {
            let (w, h) = report_size(slot_w, slot_h);
            self.w.set_if_changed(w);
            self.h.set_if_changed(h);
        }
        Size::new(REPORT_W, REPORT_H).into()
    }

    /// Centre the dialog in its slot, at the size that fits.
    fn place_children(
        &self,
        bounds: Rect,
        _proposal: SizeProposal,
        children: &mut [teksilo::core::widget::WidgetPlacement],
        _ctx: &LayoutContext,
    ) {
        let (w, h) = report_size(bounds.width, bounds.height);
        let origin = Point::new(
            bounds.x + ((bounds.width - w) / 2.0).max(0.0),
            bounds.y + ((bounds.height - h) / 2.0).max(0.0),
        );
        for child in children.iter_mut() {
            child.origin = origin;
            child.size = Size::new(w, h);
        }
    }

    fn children(&self) -> Vec<WidgetId> {
        self.root_child.into_iter().collect()
    }
}

/// Where the import read and wrote, what came across, what the writer chose, then the
/// groups.
fn report_body(vm: &ImportReportViewModel) -> impl Widget + use<> {
    let report = vm.report();
    let from = vm
        .read_from_line()
        .map(|line| small(line, TextRole::Secondary));
    let choices_block = vm.has_choices().then(|| {
        teksu!(VStack {
            spacing: 4.0
            TextWidget::new(tr!(import_scrivener_report_choices())) {
                style: TextStyleRole::SmallBold
                color: TextRole::Primary
            }
            Repeater::new(vm.choices(), |choice: &LocalizedString| {
                Box::new(small(choice.clone(), TextRole::Secondary)) as Box<dyn Widget>
            }) {
                spacing: 2.0
            }
        })
    });
    let no_findings = (!report.has_findings()).then(|| {
        small(
            tr!(import_scrivener_report_no_findings()),
            TextRole::Secondary,
        )
    });
    let links_hint = vm.says_where_items_open().then(|| {
        small(
            tr!(import_scrivener_report_links_hint()),
            TextRole::Secondary,
        )
    });
    let group_vm = vm.clone();
    teksu!(VStack {
        spacing: 14.0
        VStack {
            spacing: 2.0
            child_opt: from
            child: small(vm.written_to_line(), TextRole::Secondary)
        }
        VStack {
            spacing: 4.0
            TextWidget::new(tr!(import_scrivener_report_came_across())) {
                style: TextStyleRole::SmallBold
                color: TextRole::Primary
            }
            child: small(report.came_across_line(), TextRole::Secondary)
        }
        child_opt: choices_block
        child_opt: no_findings
        child_opt: links_hint
        Repeater::new(vm.groups(), move |group: &ReportGroup| group_view(&group_vm, group)) {
            spacing: 10.0
        }
    })
}

/// A group: a disclosure titled with its name and how many lines it holds, its lines
/// under it.
fn group_view(vm: &ImportReportViewModel, group: &ReportGroup) -> Box<dyn Widget> {
    let (line_vm, lines) = (vm.clone(), group.lines.clone());
    let title = group.title();
    Box::new(Disclosure::new(
        title.clone(),
        title,
        Signal::new(group.expanded_by_default()),
        None,
        Box::new(move |reachable| {
            let line_vm = line_vm.clone();
            let lines = ListModel::from_vec(lines.clone());
            Box::new(teksu!(Padding::new(4.0, 0.0, 4.0, 20.0) {
                Repeater::new(lines, move |line: &ReportLine| line_view(&line_vm, line, &reachable)) {
                    spacing: 8.0
                }
            }))
        }),
    ))
}

/// A line: its sentence, and the items it is about behind a disclosure of their own,
/// inside the group's, whose `group_reachable` says whether the group is open.
fn line_view(
    vm: &ImportReportViewModel,
    line: &ReportLine,
    group_reachable: &Signal<bool>,
) -> Box<dyn Widget> {
    let text = TextWidget::new(line.text.clone())
        .style(TextStyleRole::Small)
        .color(TextRole::Primary);
    if line.items.is_empty() {
        return Box::new(text);
    }
    let (item_vm, shown, more) = (vm.clone(), line.shown_items().to_vec(), line.more());
    let count = line.items.len() as i64;
    let sentence = line.text.clone();
    // A screen reader meets the button on its own when tabbing: it names its line.
    let listed = line.listed;
    let named = localized(move || {
        let line = sentence.resolve_now();
        match listed {
            Listed::Items => tr!(import_scrivener_report_items_toggle_named(
                count = count,
                line = line
            )),
            Listed::Names => tr!(import_scrivener_report_names_toggle_named(
                count = count,
                line = line
            )),
        }
        .resolve_now()
    });
    let title = match listed {
        Listed::Items => tr!(import_scrivener_report_items_toggle(count = count)),
        Listed::Names => tr!(import_scrivener_report_names_toggle(count = count)),
    };
    let items = Disclosure::new(
        title,
        named,
        Signal::new(false),
        Some(group_reachable.clone()),
        Box::new(move |_| {
            let item_vm = item_vm.clone();
            let items = ListModel::from_vec(shown.clone());
            let more_line = (more > 0).then(|| {
                small(
                    tr!(import_scrivener_report_more(count = more as i64)),
                    TextRole::Secondary,
                )
            });
            // More room between two items than between the lines of one (6 against
            // none), so a path that wraps still reads as one item.
            Box::new(teksu!(Padding::new(2.0, 0.0, 2.0, 20.0) {
                VStack {
                    spacing: 6.0
                    Repeater::new(items, move |item: &ReportItem| item_view(&item_vm, item)) {
                        spacing: 6.0
                    }
                    child_opt: more_line
                }
            }))
        }),
    )
    .secondary();
    Box::new(teksu!(VStack {
        spacing: 2.0
        child: text
        child: items
    }))
}

/// What a disclosure holds, built when it is: given the disclosure's `reachable`, for a
/// disclosure inside it.
type DisclosureContent = Box<dyn Fn(Signal<bool>) -> Box<dyn Widget>>;

/// An accordion's header over a region of its own, which opens to its height as the
/// accordion's own body would.
///
/// Not the accordion's own body, for two reasons. That body is a region named after the
/// header's visible title, and a line's title reads "3 items" like every other line's: a
/// group of three lines would give a screen reader three regions of that name. Here the
/// region carries `name`, which says which line it belongs to, as the header's spoken
/// name does. And a closed body stays laid out, clipped to no height, so its buttons and
/// links would still be Tab stops and still be read. Here, while the disclosure or one
/// around it is closed, the region is no Tab stop and is hidden from assistive
/// technologies; it still closes with the animation, since it is never taken out of the
/// layout.
struct Disclosure {
    /// The header's visible title.
    title: LocalizedString,
    /// The header's spoken name, and the region's.
    name: LocalizedString,
    /// A line's list of items: small, secondary type under its sentence.
    secondary: bool,
    open: Signal<bool>,
    /// Whether the disclosure around this one is open and in reach, if there is one.
    within: Option<Signal<bool>>,
    /// Whether what this disclosure holds is in reach: it is open, and so is every
    /// disclosure around it.
    reachable: Signal<bool>,
    /// `!reachable`, which the region's `access_hidden` binds.
    hidden: Signal<bool>,
    content: DisclosureContent,
    root: Option<WidgetId>,
}

impl Disclosure {
    fn new(
        title: LocalizedString,
        name: LocalizedString,
        open: Signal<bool>,
        within: Option<Signal<bool>>,
        content: DisclosureContent,
    ) -> Self {
        let reachable = open.get() && within.as_ref().is_none_or(Signal::get);
        Self {
            title,
            name,
            secondary: false,
            open,
            within,
            reachable: Signal::new(reachable),
            hidden: Signal::new(!reachable),
            content,
            root: None,
        }
    }

    /// Set the header in small, secondary type.
    fn secondary(mut self) -> Self {
        self.secondary = true;
        self
    }
}

impl std::fmt::Debug for Disclosure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Disclosure").finish()
    }
}

impl Widget for Disclosure {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        // Keep `reachable` and `hidden` in step with this disclosure and the one around it.
        let sync = {
            let (open, within) = (self.open.clone(), self.within.clone());
            let (reachable, hidden) = (self.reachable.clone(), self.hidden.clone());
            move || {
                let now = open.get() && within.as_ref().is_none_or(Signal::get);
                reachable.set_if_changed(now);
                hidden.set_if_changed(!now);
            }
        };
        {
            let sync = sync.clone();
            ctx.effect(&self.open, move |_: &bool| sync());
        }
        if let Some(within) = &self.within {
            ctx.effect(within, move |_: &bool| sync());
        }

        let content = (self.content)(self.reachable.clone());
        let region = teksu!(ctx => VStack {
            access_role: Role::Region
            access_label: self.name.clone()
            access_hidden: self.hidden.clone()
            child: content
        });
        ctx.set_tab_stop(region, self.reachable.clone());
        let (style, color) = if self.secondary {
            (TextStyleRole::Small, TextRole::Secondary)
        } else {
            (TextStyleRole::SmallBold, TextRole::Primary)
        };
        let root = teksu!(ctx => VStack {
            spacing: 2.0
            Accordion::new(self.title.clone(), self.open.clone()) {
                title_style: style
                title_color: color
                access_label: self.name.clone()
                access_controls: region
            }
            Collapse::new(self.open.clone()) {
                child: region
            }
        });
        self.root = Some(root);
        vec![root]
    }

    fn layout_response(&self, proposal: SizeProposal, ctx: &LayoutContext) -> LayoutResponse {
        self.root
            .and_then(|id| ctx.child_size(id, proposal))
            .map(LayoutResponse::from)
            .unwrap_or_else(|| proposal.resolve(0.0, 0.0).into())
    }

    fn place_children(
        &self,
        bounds: Rect,
        _proposal: SizeProposal,
        children: &mut [teksilo::core::widget::WidgetPlacement],
        _ctx: &LayoutContext,
    ) {
        for child in children.iter_mut() {
            child.origin = Point::new(bounds.x, bounds.y);
            child.size = Size::new(bounds.width, bounds.height);
        }
    }

    fn children(&self) -> Vec<WidgetId> {
        self.root.into_iter().collect()
    }
}

/// An item: a link to its row when this window shows the new project, else its path.
fn item_view(vm: &ImportReportViewModel, item: &ReportItem) -> Box<dyn Widget> {
    match item.uid.filter(|uid| vm.can_open(*uid)) {
        Some(uid) => {
            let open_vm = vm.clone();
            Box::new(
                Link::new(item.label())
                    .style(WrappingLink)
                    .tooltip(tr!(import_scrivener_report_open_item()))
                    .on_activate_fn(move |c| open_vm.open_item(c, uid)),
            )
        }
        None => Box::new(small(item.label(), TextRole::Secondary)),
    }
}

/// The footer: **Copy report** (`copy`) and what it did, then **Open now** and **Close**.
/// What it did gives up its width first, then Copy report its label, so Close always
/// stays on the card. Open now is left out where this window already shows the new
/// project: there it could only close the report.
fn footer(vm: &ImportReportViewModel, copy: WidgetId) -> impl Widget + use<> {
    let open_vm = vm.clone();
    let open_now = (!vm.links_items()).then(|| {
        Button::new(tr!(project_import_open_now()))
            .variant(ButtonVariant::Plain)
            .on_activate_fn(move |c| open_vm.open_now(c))
    });
    teksu!(HStack {
        spacing: 8.0
        Shrinkable::new() {
            shrink: 0.25
            min_width: 96.0
            child: copy
        }
        // Empty until the report is copied, then said once.
        Expand::horizontal {
            TextWidget::new(lit!("")) {
                text: vm.copy_status()
                style: TextStyleRole::Small
                color: TextRole::Secondary
                access_live: Live::Polite
                single_line
            }
        }
        child_opt: open_now
        Button::new(tr!(import_scrivener_report_close())) {
            variant: ButtonVariant::Filled
            on_activate_fn: |c| c.dismiss_modal()
        }
    })
}

/// An item opening its row: a link whose words wrap, in the report's small type, so a
/// long binder path stays inside the column instead of setting its width. The link's own
/// style draws one line at body size, in a `ZStack` that measures it at no width at all,
/// and so asks for the whole path on one line.
#[derive(Debug, Clone, Copy)]
struct WrappingLink;

impl LinkStyle for WrappingLink {
    fn make_body(&self, cfg: &LinkStyleConfig, ctx: &mut BuildContext) -> WidgetId {
        let role: Signal<TextRole> = cfg
            .is_hovered
            .zip3(&cfg.is_pressed, &cfg.is_focused)
            .zip(&cfg.is_visited)
            .zip(&cfg.is_disabled)
            .map(|(((h, p, f), v), d)| link_text_role(*h, *p, *f, *v, *d));
        ctx.add(RingedText {
            text: cfg.text.clone(),
            role,
            focused: cfg.is_focused.clone(),
            parts: None,
        })
    }
}

/// A link's words and the ring drawn round them while it holds the keyboard's focus,
/// sized by the words alone at the width it is offered, so they wrap.
struct RingedText {
    text: Prop<String>,
    role: Signal<TextRole>,
    focused: Signal<bool>,
    /// The ring, then the words.
    parts: Option<(WidgetId, WidgetId)>,
}

impl std::fmt::Debug for RingedText {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RingedText").finish()
    }
}

impl Widget for RingedText {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        let ring_width = ctx.theme().shape.focus_ring_width;
        let ring = ctx.add(
            RectWidget::new()
                .border_color(self.focused.map(|f| {
                    if *f {
                        BorderRole::Focused
                    } else {
                        BorderRole::Transparent
                    }
                }))
                .border_width(self.focused.map(move |f| if *f { ring_width } else { 0.0 }))
                .corner_radius(CornerRadius::uniform(4.0)),
        );
        let words = ctx.add(
            TextWidget::new(lit!(""))
                .text(self.text.clone())
                .style(TextStyleRole::Small)
                .color(self.role.clone())
                .a11y_hidden(),
        );
        self.parts = Some((ring, words));
        vec![ring, words]
    }

    fn layout_response(&self, proposal: SizeProposal, ctx: &LayoutContext) -> LayoutResponse {
        self.parts
            .and_then(|(_, words)| ctx.child_size(words, proposal))
            .map(LayoutResponse::from)
            .unwrap_or_else(|| proposal.resolve(0.0, 0.0).into())
    }

    fn place_children(
        &self,
        bounds: Rect,
        _proposal: SizeProposal,
        children: &mut [teksilo::core::widget::WidgetPlacement],
        _ctx: &LayoutContext,
    ) {
        for child in children.iter_mut() {
            child.origin = Point::new(bounds.x, bounds.y);
            child.size = Size::new(bounds.width, bounds.height);
        }
    }

    fn children(&self) -> Vec<WidgetId> {
        self.parts
            .map(|(ring, words)| vec![ring, words])
            .unwrap_or_default()
    }
}

/// A line of small text that wraps.
fn small(text: LocalizedString, color: TextRole) -> TextWidget {
    TextWidget::new(text)
        .style(TextStyleRole::Small)
        .color(color)
}

#[cfg(test)]
mod tests;
