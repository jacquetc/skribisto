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
//! ## Why a tooltip-backed topic renders differently
//!
//! A [`HelpBody::Tooltip`] topic is not Djot at all: it is the registered rich
//! tooltip's own body, which is Fluent text with teksilo's three-form inline markup.
//! Rendering it through `TextWidget` rather than converting it to Djot is deliberate.
//! Converting would make the page a *copy* of the tooltip, and a copy is a thing that
//! drifts; reading the same `LocalizedString` means the page and the tooltip are the
//! same words by construction.

use teksilo::core::binding::BindingLevel;
use teksilo::prelude::*;
use teksilo::text::EditorTypographyDefaults;
use teksilo::text_document::TextDocument;
use teksilo::widgets::rich_text::RichTextEditor;
use teksilo::widgets::tooltip::with_tooltip_registry;
use teksilo::widgets::{
    Button, ButtonVariant, Divider, Expand, HStack, MinSize, Padding, ScrollArea, SearchField,
    Spacer, TextWidget, VStack,
};

use super::help_vm::HelpViewModel;
use super::{HelpBody, HelpTopicSpec};

/// Width of the table of contents. Wide enough for the longest topic title in both
/// shipped locales without wrapping.
const NAV_WIDTH: f32 = 240.0;

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
        // The open topic, the filter, and the locale each change what is on screen.
        // The locale one matters more here than almost anywhere else in the app: a
        // topic body is picked *by locale*, so a language switch has to re-resolve the
        // document, not merely re-resolve labels.
        self.vm
            .current_key()
            .bind_to(sid, reg, BindingLevel::Rebuild);
        self.vm.query().bind_to(sid, reg, BindingLevel::Rebuild);
        self.vm.depth().bind_to(sid, reg, BindingLevel::Rebuild);
        ctx.locale_signal().bind_to(sid, reg, BindingLevel::Rebuild);

        let nav_widget = self.build_nav(ctx);
        let nav = ctx.add_boxed(Box::new(nav_widget));
        let content = self.build_content(ctx);

        // `MinSize::width`, not `FixedSize::width`. `FixedSize` proposes `None` on the
        // axis it does not bind, so a width-only one hands the column unbounded height:
        // its `Expand` then has nothing to fill, the topic list collapses to zero, and
        // the filter field floats in the middle of an empty column. That shipped once
        // here and is exactly what `welcome_body` already documents.
        //
        // Plain builders rather than `teksu!`: both halves are built as widget *ids*
        // (the reading pane adds its own document to the tree), and these containers
        // take a child by id only through `child_id`.
        let root = ctx.add(
            HStack::new()
                .spacing(0.0)
                .child(MinSize::width(NAV_WIDTH).child_id(nav))
                .child(Expand::vertical().child(Divider::vertical()))
                .child(Expand::horizontal().child_id(content)),
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
    /// The table of contents: a filter, then one group per non-empty section.
    fn build_nav(&self, ctx: &mut BuildContext) -> impl Widget + 'static {
        let current = self.vm.current_key().get();
        let mut column = VStack::new().spacing(2.0);

        for (section, topics) in self.vm.contents() {
            column = column.child(
                Padding::new(14.0, 8.0, 4.0, 8.0).child(
                    TextWidget::new(section.label())
                        .style(TextStyleRole::SmallBold)
                        .color(TextRole::Secondary),
                ),
            );
            for spec in topics {
                let vm = self.vm.clone();
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
                        .on_activate_fn(move |_ctx| vm.open(key)),
                );
            }
        }

        let empty_note = self.vm.contents().is_empty().then(|| {
            Padding::symmetric(12.0, 12.0).child(
                TextWidget::new(tr!(help_no_matching_topic()))
                    .style(TextStyleRole::Body)
                    .color(TextRole::Secondary),
            )
        });

        let mut body = VStack::new().spacing(0.0).child(
            Padding::symmetric(8.0, 8.0)
                .child(SearchField::new(self.vm.query()).placeholder(tr!(help_filter_topics()))),
        );
        if let Some(note) = empty_note {
            body = body.child(note);
        }
        let _ = ctx;
        body.child(Expand::new().child(ScrollArea::new().child(column)))
    }

    /// The reading pane: a header, then the topic body.
    fn build_content(&self, ctx: &mut BuildContext) -> WidgetId {
        let Some(spec) = self.vm.current_topic() else {
            // Reachable when an extension that registered the open topic is dropped
            // while the window is open. Say so rather than showing the last topic that
            // happened to render.
            return ctx.add(
                Padding::symmetric(24.0, 24.0).child(
                    TextWidget::new(tr!(help_topic_missing()))
                        .style(TextStyleRole::Body)
                        .color(TextRole::Secondary),
                ),
            );
        };

        let vm_back = self.vm.clone();
        let can_go_back = self.vm.depth().get() > 0;
        let header = HStack::new()
            .spacing(8.0)
            .child(
                Button::new(tr!(help_back()))
                    .variant(ButtonVariant::Ghost)
                    .enabled(can_go_back)
                    .on_activate_fn(move |_ctx| vm_back.back()),
            )
            .child(
                TextWidget::new((spec.title)())
                    .style(TextStyleRole::BodyBold)
                    .single_line(),
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

        // The body fills whatever is left under the header. `RichTextEditor` sizes to
        // its content rather than greedily, so without the `Expand` a short topic would
        // leave the reading pane's background showing through beneath it.
        let body = self.build_body(ctx, &spec);
        ctx.add(column.child(Expand::new().child_id(body)))
    }

    /// The prose itself, by body kind.
    fn build_body(&self, ctx: &mut BuildContext, spec: &HelpTopicSpec) -> WidgetId {
        match &spec.body {
            HelpBody::Djot(_) => {
                let source = spec
                    .resolve(&current_locale_tag())
                    .map(|r| r.source)
                    .unwrap_or_default();
                let doc = TextDocument::new();
                // A malformed source is a bug in shipped content, and the drift test
                // parses every one of them, so this cannot be reached by a built-in
                // topic. A contributed topic gets an empty page rather than a panic.
                if let Err(err) = doc.set_djot_sync(source) {
                    eprintln!(
                        "skribisto: help topic '{}' failed to parse: {err}",
                        spec.key
                    );
                }
                let vm = self.vm.clone();
                ctx.add(
                    RichTextEditor::read_only(doc)
                        .content_padding_symmetric(14.0, 22.0)
                        .typography_defaults(reading_typography())
                        .on_link_activated(move |href, ctx| vm.follow_link(href, ctx)),
                )
            }
            HelpBody::Tooltip(tooltip_key) => {
                let key = *tooltip_key;
                let (text, more) = with_tooltip_registry(|reg| {
                    reg.get(key)
                        .map(|c| (Some(c.text.clone()), c.more.clone()))
                        .unwrap_or((None, None))
                })
                .unwrap_or((None, None));

                let vm = self.vm.clone();
                let mut column = VStack::new().spacing(12.0);
                if let Some(text) = text {
                    let vm = vm.clone();
                    column = column.child(
                        TextWidget::new(text)
                            .markup(true)
                            .style(TextStyleRole::Body)
                            .on_link_click(move |href, ctx| vm.follow_link(href, ctx)),
                    );
                }
                if let Some(more) = more {
                    let vm = vm.clone();
                    column = column.child(
                        TextWidget::new(more)
                            .markup(true)
                            .style(TextStyleRole::Body)
                            .on_link_click(move |href, ctx| vm.follow_link(href, ctx)),
                    );
                }
                ctx.add(ScrollArea::new().child(Padding::symmetric(14.0, 22.0).child(column)))
            }
        }
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
    teksilo::i18n::current_locale()
        .map(|sig| sig.get().to_string())
        .unwrap_or_else(|| super::SOURCE_LOCALE.to_string())
}
