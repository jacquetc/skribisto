// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The Help window's body: a table of contents on the left, one topic on the right.
//!
//! ## Why the reading pane is a read-only editor
//!
//! A topic's prose is Djot, the same format every scene in a project is stored in, and
//! `RichTextEditor::read_only` is the widget that already renders Djot in this app. So
//! a help page goes through the same parser, the same typesetter and the same
//! selection and copy behaviour as the manuscript, and gains headings, lists and
//! blockquotes without a second renderer existing to disagree with the first.
//!
//! It also means a link in a help page behaves like a link: teksilo follows one on a
//! plain click when the surface is read-only (an editable one still wants Ctrl, since
//! there a click places the caret).
//!
//! ## Why a tooltip-backed topic is a document too
//!
//! A [`HelpBody::Tooltip`] topic is not Djot at all: it is the registered rich
//! tooltip's own body, which is Fluent text with teksilo's three-form inline markup
//! (`**bold**`, `*italic*`, `[label](url)`). It is read from the same `LocalizedString`
//! the tooltip shows, every time the page is built, never stored: the page and the
//! tooltip are the same words by construction.
//!
//! It is shown in the same read-only editor as a Djot page, not as a `TextWidget`. A
//! label takes no keyboard focus, so a glossary entry was the one page a reader moving
//! by keyboard could not get into: Enter on its topic left them on the button, and Tab
//! went past the text. The markup is a subset of Markdown, which `text-document` reads
//! synchronously (`TextCursor::insert_markdown`); a test keeps the tooltips free of the
//! Markdown the subset does not have, so the page reads exactly as the bubble does.

use teksilo::core::binding::BindingLevel;
use teksilo::prelude::*;
use teksilo::text::EditorTypographyDefaults;
use teksilo::text_document::TextDocument;
use teksilo::widgets::rich_text::RichTextEditor;
use teksilo::widgets::tooltip::with_tooltip_registry;
use teksilo::widgets::{
    Button, ButtonVariant, Divider, Expand, HStack, IconButton, IconWidget, MaxSize, MinSize,
    Padding, Panel, ScrollArea, SearchField, Spacer, TextWidget, VStack,
};

use super::help_vm::HelpViewModel;
use super::{HelpBody, HelpTopicSpec};
use crate::shared::shortcut_group::{ShortcutGroup, ShortcutGroupExt};

/// A flat backdrop of `role` behind `child`.
///
/// `corner_radius(0)` and `padding(0)` are the whole point: a bare `Panel` carries the
/// rounded, bordered, inset chrome of a *card*, and two of those side by side read as
/// two floating boxes rather than as the two grounds of one window. This is the same
/// shape `tabs::shared::editor::tab_backdrop` uses for an editor body, for the same
/// reason.
fn ground(role: SurfaceRole, child: WidgetId) -> impl Widget + 'static {
    Panel::new()
        .background(role)
        .corner_radius(0.0)
        .padding(0.0)
        .child(child)
}

/// The shortcut and the action that close the Help window with Ctrl+W (⌘W on macOS).
pub(super) const CLOSE_SHORTCUT: &str = "help.close";

/// Width of the table of contents. Wide enough for the longest topic title in both
/// shipped locales without wrapping.
pub(super) const NAV_WIDTH: f32 = 240.0;

/// The Help window's content.
pub struct HelpPanel {
    vm: HelpViewModel,
    root_child: Option<WidgetId>,
}

impl HelpPanel {
    pub fn new(vm: HelpViewModel) -> Self {
        Self {
            vm,
            root_child: None,
        }
    }
}

impl std::fmt::Debug for HelpPanel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HelpPanel").finish_non_exhaustive()
    }
}

impl Widget for HelpPanel {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        let sid = ctx.self_id();
        let reg = ctx.binding_registry();
        // **The locale is the only thing this shell rebuilds on, and the filter text is
        // deliberately not on the list.** The `SearchField` is built inside this
        // subtree, so a rebuild destroys it and builds a fresh one whose caret parks at
        // 0 with the whole value selected — and the next keystroke then replaces
        // everything typed so far. Binding the query here is exactly why the field
        // could never hold more than the last character struck. The two children below
        // bind what each of them actually reads instead, which leaves the field the
        // reader is typing into untouched. `TagPicker`/`TagPickerList` carry the same
        // split for the same bug.
        //
        // The locale earns the shell-wide rebuild the other two do not. A topic body is
        // picked *by locale*, so a language switch has to re-resolve the document, not
        // merely re-resolve labels — and it has to re-resolve the filter's own
        // placeholder too, which no child of it can do on its behalf.
        ctx.locale_signal().bind_to(sid, reg, BindingLevel::Rebuild);

        // Ctrl+W (⌘W on macOS) closes the Help window, as it closes every other one.
        // Registered here because this panel is that window's whole body (see
        // `window::help_window_config`) and its tree is its own: the project window's
        // Ctrl+W lives in another registry and never reaches this one.
        ctx.register_shortcut_global(
            Shortcut::new(CLOSE_SHORTCUT)
                .group(ShortcutGroup::Help)
                .name(tr!(shortcut_name_help_close()))
                .primary(KeyStroke::ctrl(Key::W))
                .build(),
        );
        ctx.register_action_global(Action::new(CLOSE_SHORTCUT).on_invoke(|_i, c| c.close_window()));

        // The page first: its id is where every way of opening a topic puts the
        // reader (a topic in the list, a link, Back). It stays the same id across the
        // page's own rebuilds, which is what lets a focus request made before one
        // land in the page that replaces it.
        let page = ctx.add(HelpPageBody::new(self.vm.clone()));
        let header = ctx.add(HelpPageHeader::new(self.vm.clone(), page));
        let nav_widget = self.build_nav(page);
        let nav = ctx.add_boxed(Box::new(nav_widget));
        // The body fills whatever is left under the header. `RichTextEditor` sizes to
        // its content rather than greedily, so without the `Expand` a short topic would
        // leave the reading pane's background showing through beneath it.
        let content = ctx.add(
            VStack::new()
                .spacing(0.0)
                .child(header)
                .child(Expand::new().child(page)),
        );

        // `MinSize::width` capped by `MaxSize::width`, not `FixedSize::width`.
        // `FixedSize` proposes `None` on the axis it does not bind, so a width-only one
        // hands the column unbounded height: its `Expand` then has nothing to fill, the
        // topic list collapses to zero, and the filter field floats in the middle of an
        // empty column. That shipped once here and is exactly what `welcome_body`
        // already documents. The cap is there because a `MinSize` is only a floor: it
        // forwards an unbounded width unbounded (teksilo 0.16), so on its own the
        // column took the filter field's natural width, or a long topic title's.
        //
        // Plain builders here, not because of `child_id`: `child_id: id` is an
        // ordinary `teksu!` property and would parse fine. This HStack is a plain
        // builder because it was never converted, not because of any DSL wall.
        // The two halves take the main window's own grounds, measured rather than
        // guessed: a dock panel is transparent over `SurfaceRole::Main`, and an editor
        // tab's body sits on `SurfaceRole::Content` (`ContentTab::backdrop_role`). This
        // window read as one flat sheet because the reading pane inherited Main too, so
        // the prose had no page under it.
        let root = ctx.add(
            HStack::new()
                .spacing(0.0)
                .child(
                    MaxSize::width(NAV_WIDTH)
                        .child(MinSize::width(NAV_WIDTH).child(ground(SurfaceRole::Main, nav))),
                )
                .child(Expand::vertical().child(Divider::vertical()))
                .child(Expand::horizontal().child(ground(SurfaceRole::Content, content))),
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

impl HelpPanel {
    /// The table of contents: the filter field, then the filtered list under it.
    ///
    /// Takes no [`BuildContext`] on purpose. Everything it assembles is a plain widget,
    /// and the one part that has to react to typing is [`HelpTopicList`], which owns its
    /// own `build` — which is the whole point of the split.
    fn build_nav(&self, page: WidgetId) -> impl Widget + 'static {
        VStack::new()
            .spacing(0.0)
            .child(
                Padding::symmetric(8.0, 8.0).child(
                    // The placeholder is gone once the reader types, and is never a
                    // name: without the label the field has none.
                    SearchField::new(self.vm.query())
                        .label(tr!(help_filter_topics()))
                        .placeholder(tr!(help_filter_topics())),
                ),
            )
            .child(Expand::new().child(HelpTopicList::new(self.vm.clone(), page)))
    }
}

/// The filtered table of contents: one group per section that still has a topic in it,
/// or a note saying nothing matched.
///
/// It is a widget of its own for one reason: **the filter query has to be bound below
/// the `SearchField`, never above it.** Bound above — which is how this shipped — every
/// keystroke rebuilt the field along with the list, and a fresh `TextInput` opens with
/// its caret at 0 and its value selected, so the following character overwrote the
/// whole query. The field could hold exactly one letter, which read as it erasing every
/// stroke. Bound here, the list narrows on each keystroke and the field is never touched.
///
/// It binds no locale: [`HelpPanel`] does, and a rebuild there rebuilds this with it.
struct HelpTopicList {
    vm: HelpViewModel,
    /// The page a topic opens into, and so where its button sends the reader.
    page: WidgetId,
    root_child: Option<WidgetId>,
}

impl HelpTopicList {
    fn new(vm: HelpViewModel, page: WidgetId) -> Self {
        Self {
            vm,
            page,
            root_child: None,
        }
    }
}

impl std::fmt::Debug for HelpTopicList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HelpTopicList").finish_non_exhaustive()
    }
}

impl Widget for HelpTopicList {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        let sid = ctx.self_id();
        let reg = ctx.binding_registry();
        self.vm.query().bind_to(sid, reg, BindingLevel::Rebuild);
        // Which topic is open is the only state a row carries (the `Tinted` variant
        // below), so the highlight moving is this list's business rather than the whole
        // window's — one more thing the reading pane no longer drags the filter through.
        self.vm
            .current_key()
            .bind_to(sid, reg, BindingLevel::Rebuild);

        let current = self.vm.current_key().get();
        let contents = self.vm.contents();
        let nothing_matched = contents.is_empty();

        let mut column = VStack::new().spacing(2.0);
        for (section, topics) in contents {
            column = column.child(
                Padding::new(14.0, 8.0, 4.0, 8.0).child(
                    TextWidget::new(section.label())
                        .style(TextStyleRole::SmallBold)
                        .color(TextRole::Secondary),
                ),
            );
            for spec in topics {
                let vm = self.vm.clone();
                let page = self.page;
                let key = spec.key;
                let is_current = key == current;
                column = column.child(
                    Button::new((spec.title)())
                        // `Tinted` for the open topic is the only state this list
                        // carries: a plain list of links with nothing marked leaves the
                        // reader unable to answer "which page am I on".
                        .variant(if is_current {
                            ButtonVariant::Tinted
                        } else {
                            ButtonVariant::Ghost
                        })
                        // Into the page, not back onto this button: a reader who
                        // moves by keyboard opens a topic to read it, and a focus
                        // that stays here announces nothing new.
                        .on_activate_fn(move |ctx| {
                            vm.open(key);
                            ctx.request_focus_into(page);
                        }),
                );
            }
        }

        let mut body = VStack::new().spacing(0.0);
        if nothing_matched {
            body = body.child(
                Padding::symmetric(12.0, 12.0).child(
                    TextWidget::new(tr!(help_no_matching_topic()))
                        .style(TextStyleRole::Body)
                        .color(TextRole::Secondary),
                ),
            );
        }

        let id = ctx.add(body.child(Expand::new().child(ScrollArea::new().child(column))));
        self.root_child = Some(id);
        vec![id]
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

/// The reading pane's header: Back, the topic's title, and the note a page served in
/// another language carries.
///
/// Bound to the open topic and the depth of the back trail, and to nothing else. Kept
/// out of [`HelpPanel`] so that shell does not rebuild on the open topic, which is
/// what keeps a click in the table of contents from recreating (and re-selecting the
/// contents of) the filter field the reader may be part way through typing into.
struct HelpPageHeader {
    vm: HelpViewModel,
    /// Where Back puts the reader: the page it brings back.
    page: WidgetId,
    root_child: Option<WidgetId>,
}

impl HelpPageHeader {
    fn new(vm: HelpViewModel, page: WidgetId) -> Self {
        Self {
            vm,
            page,
            root_child: None,
        }
    }
}

impl std::fmt::Debug for HelpPageHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HelpPageHeader").finish_non_exhaustive()
    }
}

impl Widget for HelpPageHeader {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        let sid = ctx.self_id();
        let reg = ctx.binding_registry();
        self.vm
            .current_key()
            .bind_to(sid, reg, BindingLevel::Rebuild);
        self.vm.depth().bind_to(sid, reg, BindingLevel::Rebuild);

        let id = self.build_content(ctx);
        self.root_child = Some(id);
        vec![id]
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

impl HelpPageHeader {
    fn build_content(&self, ctx: &mut BuildContext) -> WidgetId {
        // A topic that names nothing has no header: the page says so on its own.
        let Some(spec) = self.vm.current_topic() else {
            return ctx.add(VStack::new());
        };

        let vm_back = self.vm.clone();
        let page = self.page;
        let can_go_back = self.vm.depth().get() > 0;
        let header = HStack::new()
            .spacing(8.0)
            .child(
                // `tooltip` is not decoration here: `IconButton` uses it as the
                // accessible name, and a debug assert fires without one. So the same
                // string that labelled the old text button still names this control to
                // a screen reader.
                IconButton::new(IconWidget::chevron_left(16.0))
                    .tooltip(tr!(help_back()))
                    .enabled(can_go_back)
                    .on_activate_fn(move |ctx| {
                        vm_back.back();
                        ctx.request_focus_into(page);
                    }),
            )
            .child(
                // The page's level-one heading, so a screen reader landing in the
                // page hears which one it is, and the page's own `##` headings sit
                // under it (no page opens with a level-one heading of its own).
                TextWidget::new((spec.title)())
                    .style(TextStyleRole::BodyBold)
                    .single_line()
                    .access_role(teksilo::core::accesskit::Role::Heading)
                    .access_customize(|node| node.set_level(1)),
            )
            .child(Spacer::new());

        let mut column = VStack::new()
            .spacing(0.0)
            .child(Padding::symmetric(10.0, 14.0).child(header))
            .child(Divider::new());

        // A page served in a language the reader did not ask for says so. Silence here
        // would be the app claiming a translation it does not have.
        if let Some(resolved) = spec.resolve(&current_locale_tag())
            && resolved.is_fallback
        {
            column = column.child(
                Padding::symmetric(8.0, 14.0).child(
                    TextWidget::new(tr!(help_not_translated()))
                        .style(TextStyleRole::Small)
                        .color(TextRole::Secondary),
                ),
            );
        }
        ctx.add(column)
    }
}

/// The open topic's text.
///
/// Bound to the open topic only, so typing in the filter does not re-parse and
/// re-typeset the page being read on every keystroke. Its own id is the one the
/// topic list, Back and the page's links send the reader into: a rebuild keeps it,
/// and teksilo puts a focus that was inside it back at the same place among the
/// fresh children, which is the new page's text.
struct HelpPageBody {
    vm: HelpViewModel,
    root_child: Option<WidgetId>,
}

impl HelpPageBody {
    fn new(vm: HelpViewModel) -> Self {
        Self {
            vm,
            root_child: None,
        }
    }
}

impl std::fmt::Debug for HelpPageBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HelpPageBody").finish_non_exhaustive()
    }
}

impl Widget for HelpPageBody {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        let sid = ctx.self_id();
        let reg = ctx.binding_registry();
        self.vm
            .current_key()
            .bind_to(sid, reg, BindingLevel::Rebuild);
        self.vm
            .read_signal()
            .bind_to(sid, reg, BindingLevel::Rebuild);

        // Opened to be read (see `HelpViewModel::open_to_read`): enter the page once it
        // is mounted under its window, which also wins over the window's own first
        // focus on its filter field.
        if self.vm.take_read_request() {
            ctx.run_after_mount(move |c| c.request_focus_into(sid));
        }

        let id = match self.vm.current_topic() {
            Some(spec) => self.build_body(ctx, &spec),
            // Reachable when an extension that registered the open topic is dropped
            // while the window is open. Say so rather than showing the last topic that
            // happened to render.
            None => ctx.add(
                Padding::symmetric(24.0, 24.0).child(
                    TextWidget::new(tr!(help_topic_missing()))
                        .style(TextStyleRole::Body)
                        .color(TextRole::Secondary),
                ),
            ),
        };
        self.root_child = Some(id);
        vec![id]
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

impl HelpPageBody {
    /// The prose itself, by body kind: either way a read-only document named after its
    /// topic.
    fn build_body(&self, ctx: &mut BuildContext, spec: &HelpTopicSpec) -> WidgetId {
        let doc = TextDocument::new();
        match &spec.body {
            HelpBody::Djot(_) => {
                let source = spec
                    .resolve(&current_locale_tag())
                    .map(|r| r.source)
                    .unwrap_or_default();
                // A malformed source is a bug in shipped content, and the drift test
                // parses every one of them, so this cannot be reached by a built-in
                // topic. A contributed topic gets an empty page rather than a panic.
                // Chords as this platform writes them (`⌘S` on macOS): see `chords`.
                let source = super::chords::platform_chords(source);
                if let Err(err) = doc.set_djot_sync(&source) {
                    eprintln!(
                        "skribisto: help topic '{}' failed to parse: {err}",
                        spec.key
                    );
                }
            }
            HelpBody::Tooltip(tooltip_key) => {
                // The tooltip's text, then its "more", as two paragraphs.
                let key = *tooltip_key;
                let source = with_tooltip_registry(|reg| {
                    reg.get(key).map(|c| {
                        let mut source = c.text.resolve_now();
                        if let Some(more) = &c.more {
                            source.push_str("\n\n");
                            source.push_str(&more.resolve_now());
                        }
                        source
                    })
                })
                .flatten()
                .unwrap_or_default();
                if let Err(err) = doc.cursor().insert_markdown(&source) {
                    eprintln!("skribisto: help topic '{}' failed to load: {err}", spec.key);
                }
            }
        }
        // A link to another topic opens it, and the reader follows into it.
        let page = ctx.self_id();
        let vm = self.vm.clone();
        ctx.add(
            RichTextEditor::read_only(doc)
                .content_padding_symmetric(14.0, 22.0)
                .typography_defaults(reading_typography())
                .on_link_activated(move |href, ctx| {
                    if vm.follow_link(href, ctx) {
                        ctx.request_focus_into(page);
                    }
                })
                // The document is named after its topic, so landing in it says which
                // page it is. It used to be an unnamed document.
                .access_label((spec.title)()),
        )
    }
}

/// How a help page is set: leaded lines, air between paragraphs, and an indented
/// first line.
///
/// These are *defaults*, filled only where a block carries no explicit format of its
/// own, and the typesetter does not apply the indent or the paragraph space to headings
/// or list items. So a numbered set of steps stays tight while the prose around it
/// breathes, without the source having to say so.
///
/// Setting both an indent and a paragraph space is a deliberate choice rather than an
/// oversight: book typography picks one or the other, but a help page is scanned as
/// often as it is read, and the space is what lets a reader find their place again after
/// looking away at the app. The indent is kept small enough not to fight it.
fn reading_typography() -> EditorTypographyDefaults {
    EditorTypographyDefaults {
        font_family: None,
        line_height: 1.35,
        first_line_indent: 16.0,
        paragraph_spacing_before: 0.0,
        // Space *after* rather than before, so a paragraph following a heading sits
        // close to it, which is what makes the heading read as belonging to what
        // follows rather than floating between two sections.
        paragraph_spacing_after: 9.0,
    }
}

/// The active locale as a plain tag (`"fr-FR"`), or the source locale before the i18n
/// manager exists (which is the case in a headless test).
pub(crate) fn current_locale_tag() -> String {
    crate::shared::ui_locale::current_tag().unwrap_or_else(|| super::SOURCE_LOCALE.to_string())
}
