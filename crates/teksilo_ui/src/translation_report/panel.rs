// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The Report a Translation Error window's content: thin, every decision in
//! [`TranslationReportViewModel`].
//!
//! Two columns over a footer. On the left, the language and the search: the writer
//! finds the text. On the right, the report: what was picked, what is wrong, how it
//! should read, and what will be public. The footer holds the buttons and what the
//! last one did.
//!
//! **What rebuilds, and what must not.** The whole panel rebuilds on a language
//! switch only, as the Help window does: its labels and the language picker read the
//! interface language. Inside it, three blocks rebuild on their own signals (the
//! picked text's card, the public preview, the footer's status) through
//! [`Rebuilding`]. The search field and the two editors sit outside every one of
//! them: a rebuild destroys the widget it rebuilds, and a field destroyed under the
//! writer's fingers loses its caret and, for the search field, everything typed but
//! the last character (the Help filter shipped with exactly that bug).

use teksilo::core::BindingLevel;
use teksilo::core::accesskit::Role;
use teksilo::platform::clipboard::ClipboardHandle;
use teksilo::prelude::*;
use teksilo::widgets::code_editor::TabKey;
use teksilo::widgets::rich_text::RichTextEditor;
use teksilo::widgets::{
    ActivateOn, Button, ButtonVariant, ComboBox, Divider, Expand, HStack, IconLocation, IconWidget,
    ListView, MaxSize, MinSize, Padding, Panel, ScrollArea, SearchField, Spacer, StandardListItem,
    Switcher, TextInput, TextWidget, VStack,
};

use super::report::ProblemKind;
use super::translation_report_vm::{Notice, Phase, PlaceChoice, Scope, TranslationReportViewModel};
use crate::shared::shortcut_group::{ShortcutGroup, ShortcutGroupExt};

/// Ctrl+W (⌘W on macOS) closes the window, as it closes every other one. In the
/// `help.` namespace: the window is a Help-menu door, and its tree is its own.
const CLOSE_SHORTCUT: &str = "help.translation_report_close";

/// The search column's width: wide enough for a menu label and its English source
/// on one line each. Exactly this, neither more nor less: a minimum alone lets the
/// column take the one-line width of its longest prose, which left the report column
/// 63 px wide. A floor and a cap rather than a `FixedSize`, which passes its child
/// only the axes it fixes: a width-only one left the results list no height at all,
/// and the button under it was drawn over the matches.
const SEARCH_WIDTH: f32 = 380.0;

/// The label above a field.
fn field_label(text: LocalizedString) -> TextWidget {
    TextWidget::new(text)
        .style(TextStyleRole::SmallBold)
        .color(TextRole::Secondary)
}

/// Small secondary prose: a hint, a note, a status line.
fn note(text: impl Into<LocalizedString>) -> TextWidget {
    TextWidget::new(text)
        .style(TextStyleRole::Small)
        .color(TextRole::Secondary)
}

/// The window's content.
pub struct TranslationReportPanel {
    vm: TranslationReportViewModel,
    root_child: Option<WidgetId>,
}

impl TranslationReportPanel {
    pub fn new(vm: TranslationReportViewModel) -> Self {
        Self {
            vm,
            root_child: None,
        }
    }
}

impl std::fmt::Debug for TranslationReportPanel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TranslationReportPanel")
            .finish_non_exhaustive()
    }
}

impl Widget for TranslationReportPanel {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        let sid = ctx.self_id();
        let reg = ctx.binding_registry();
        ctx.locale_signal().bind_to(sid, reg, BindingLevel::Rebuild);
        // The report follows the interface language until the writer picks one.
        self.vm.interface_language_changed();

        ctx.register_shortcut_global(
            Shortcut::new(CLOSE_SHORTCUT)
                .group(ShortcutGroup::Help)
                .name(tr!(shortcut_name_translation_report_close()))
                .primary(KeyStroke::ctrl(Key::W))
                .build(),
        );
        ctx.register_action_global(Action::new(CLOSE_SHORTCUT).on_invoke(|_i, c| c.close_window()));

        let search = search_column(&self.vm);
        let report = report_column(&self.vm);
        let footer = footer(&self.vm);
        let root = ctx.add(teksu!(
            VStack {
                spacing: 0.0
                Expand::vertical {
                    HStack {
                        spacing: 0.0
                        MinSize::width(SEARCH_WIDTH) {
                            MaxSize::width(SEARCH_WIDTH) {
                                child: search
                            }
                        }
                        Divider::vertical()
                        Expand::horizontal {
                            child: report
                        }
                    }
                }
                Divider::new()
                child: footer
            }
        ));
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

// ── The search column ───────────────────────────────────────────────────────

fn search_column(vm: &TranslationReportViewModel) -> impl Widget + 'static {
    let language = Signal::new(Some(vm.language()));
    let pick_vm = vm.clone();
    let languages = ComboBox::from_items(
        TranslationReportViewModel::languages(),
        language,
        |tag: &String| lit!(TranslationReportViewModel::language_label(tag)),
    )
    .label(tr!(translation_report_language()))
    .searchable(true)
    .on_select(move |tag, _ctx| pick_vm.choose_language(tag));

    let scope_vm = vm.clone();
    let scope_line = Rebuilding::new(
        {
            let scope = vm.scope();
            move |ctx: &mut BuildContext| {
                scope
                    .clone()
                    .bind_to(ctx.self_id(), ctx.binding_registry(), BindingLevel::Rebuild)
            }
        },
        move |ctx| {
            let Some(scope) = scope_vm.scope().get() else {
                return ctx.add(VStack::new());
            };
            let key = match &scope {
                Scope::HelpPage(k) | Scope::Tooltip(k) => k.clone(),
            };
            let page = crate::help::topic(&key)
                .map(|t| (t.title)().resolve_now())
                .unwrap_or(key);
            let clear_vm = scope_vm.clone();
            // The note above its button, not beside it: wrapping prose in a row
            // keeps its one-line width, and a long page title would push the
            // button out of the column.
            ctx.add(
                VStack::new()
                    .spacing(4.0)
                    .child(note(tr!(translation_report_scope(page = page))))
                    .child(
                        HStack::new()
                            .child(
                                Button::new(tr!(translation_report_scope_clear()))
                                    .variant(ButtonVariant::Ghost)
                                    .on_activate_fn(move |_ctx| clear_vm.open_for(None)),
                            )
                            .child(Spacer::new()),
                    ),
            )
        },
    );

    let list_vm = vm.clone();
    let list = ListView::new(vm.rows(), |_i, row, selected| {
        Box::new(
            StandardListItem::new(lit!(row.title.clone()))
                .subtitle(lit!(row.subtitle.clone()))
                .label_overflow(TextOverflow::Ellipsis(EllipsisMode::Trailing))
                .subtitle_overflow(TextOverflow::Ellipsis(EllipsisMode::Trailing))
                .selected(selected),
        )
    })
    .activate_on(ActivateOn::SingleClick)
    .selection(vm.selection())
    .auto_item_height(48.0)
    .on_activate(move |i, _ctx| list_vm.pick_row(i))
    // Last: this wraps the list, so every ListView call is above it.
    .access_label(tr!(translation_report_results()));

    // Stacked in the order of `RESULTS_PROMPT`, `RESULTS_NONE`, `RESULTS_LIST`.
    let query = vm.query();
    let results = teksu!(
        Switcher::new(vm.results_page()) {
            Padding::uniform(12.0) {
                note(tr!(translation_report_type_to_search()))
            }
            Padding::uniform(12.0) {
                note(tr!(translation_report_no_results()))
            }
            child: list
        }
    );

    let not_found_vm = vm.clone();
    teksu!(
        Padding::uniform(16.0) {
            VStack {
                spacing: 10.0
                child: languages
                child: scope_line
                SearchField::new(query) {
                    label: tr!(translation_report_find())
                    placeholder: tr!(translation_report_find_placeholder())
                }
                Expand::vertical {
                    child: results
                }
                HStack {
                    Button::new(tr!(translation_report_not_found())) {
                        variant: ButtonVariant::Ghost
                        on_activate_fn: move |_ctx| not_found_vm.cannot_find()
                    }
                    Spacer
                }
            }
        }
    )
}

// ── The report column ───────────────────────────────────────────────────────

fn report_column(vm: &TranslationReportViewModel) -> impl Widget + 'static {
    let card_vm = vm.clone();
    let card = Rebuilding::new(
        {
            let version = vm.card_version();
            move |ctx: &mut BuildContext| {
                version.clone().bind_to(
                    ctx.self_id(),
                    ctx.binding_registry(),
                    BindingLevel::Rebuild,
                )
            }
        },
        move |ctx| build_card(&card_vm, ctx),
    );

    let kind_vm = vm.clone();
    let kind = ComboBox::from_items(ProblemKind::ALL, vm.kind(), |k: &ProblemKind| {
        kind_label(*k)
    })
    .label(tr!(translation_report_kind()))
    .placeholder(tr!(translation_report_kind_placeholder()))
    .on_select(move |k, _ctx| kind_vm.choose_kind(*k));

    let explain_vm = vm.clone();
    let explanation = RichTextEditor::editor(vm.explanation())
        .label(tr!(translation_report_explain()))
        .tab_key(TabKey::MoveFocus)
        .min_lines(2)
        .max_lines(6)
        .on_change(move || explain_vm.edited());

    let suggest_vm = vm.clone();
    let suggestion = RichTextEditor::editor(vm.suggestion())
        .label(tr!(translation_report_suggestion()))
        .tab_key(TabKey::MoveFocus)
        .min_lines(3)
        .max_lines(10)
        .on_change(move || suggest_vm.edited());

    let hints_vm = vm.clone();
    let hints = Rebuilding::new(
        {
            let version = vm.card_version();
            move |ctx: &mut BuildContext| {
                version.clone().bind_to(
                    ctx.self_id(),
                    ctx.binding_registry(),
                    BindingLevel::Rebuild,
                )
            }
        },
        move |ctx| {
            let column = hints_vm
                .suggestion_hints()
                .into_iter()
                .fold(VStack::new().spacing(4.0), |c, h| c.child(note(lit!(h))));
            ctx.add(column)
        },
    );

    let preview = public_preview(vm);

    teksu!(
        ScrollArea::new() {
            Padding::uniform(16.0) {
                VStack {
                    spacing: 12.0
                    child: card
                    field_label(tr!(translation_report_kind()))
                    // Inside an `Expand`: a combo sizes to its label and never
                    // shrinks on its own, so a long kind ("Der Text klingt
                    // unnatürlich…") made the whole column wider than the window.
                    // Given a bounded slot, its one-line label ends in an ellipsis
                    // instead, and a screen reader still hears the whole value.
                    Expand::horizontal {
                        child: kind
                    }
                    field_label(tr!(translation_report_explain()))
                    Expand::horizontal {
                        Panel::new() {
                            padding: 0.0
                            child: explanation
                        }
                    }
                    field_label(tr!(translation_report_suggestion()))
                    Expand::horizontal {
                        Panel::new() {
                            padding: 0.0
                            child: suggestion
                        }
                    }
                    child: hints
                    child: preview
                }
            }
        }
    )
}

/// What the picked text is, where it appears, and what a fix to it changes; or the
/// "where did you see it" field for a text the search did not find.
fn build_card(vm: &TranslationReportViewModel, ctx: &mut BuildContext) -> WidgetId {
    if vm.is_not_found() {
        return ctx.add(teksu!(
            VStack {
                spacing: 6.0
                field_label(tr!(translation_report_seen_at()))
                TextInput::new(vm.seen_at()) {
                    label: tr!(translation_report_seen_at())
                    placeholder: tr!(translation_report_seen_at_placeholder())
                }
                note(tr!(translation_report_system_dialogs()))
            }
        ));
    }
    let Some(card) = vm.card() else {
        return ctx.add(VStack::new());
    };

    let mut column = VStack::new()
        .spacing(6.0)
        .child(field_label(tr!(translation_report_selected())))
        .child(
            TextWidget::new(lit!(card.text.clone()))
                .style(TextStyleRole::BodyBold)
                .access_role(Role::Heading)
                .access_customize(|node| node.set_level(2)),
        );
    let mut detail = card.location.clone();
    if let Some(english) = &card.english {
        detail.push_str(" · ");
        detail.push_str(&tr!(translation_report_english(sources = english.clone())).resolve_now());
    }
    column = column
        .child(note(lit!(detail)))
        .child(note(tr!(translation_report_message_id(
            id = card.id.clone(),
            path = card.path.clone()
        ))));
    if let Some(text) = &card.note {
        column = column.child(note(lit!(text.clone())));
    }
    if !card.places.is_empty() {
        let labels = card.places.clone();
        let choices: Vec<PlaceChoice> = labels.iter().map(|(c, _)| *c).collect();
        let place_vm = vm.clone();
        column = column
            .child(field_label(tr!(translation_report_place())))
            .child(
                Expand::horizontal().child(
                    ComboBox::from_items(choices, vm.place(), move |c: &PlaceChoice| {
                        lit!(place_label(&labels, c))
                    })
                    .label(tr!(translation_report_place()))
                    .on_select(move |c, _ctx| place_vm.choose_place(*c)),
                ),
            );
    }
    ctx.add(column)
}

/// "What will be public": the cost and the steps always, the exact title and body
/// on request.
fn public_preview(vm: &TranslationReportViewModel) -> impl Widget + 'static {
    let open = Signal::new(false);
    let toggle_open = open.clone();
    let body_vm = vm.clone();
    let body = Rebuilding::new(
        {
            let vm = vm.clone();
            let open = open.clone();
            move |ctx: &mut BuildContext| {
                let (sid, reg) = (ctx.self_id(), ctx.binding_registry());
                open.clone().bind_to(sid, reg, BindingLevel::Rebuild);
                vm.can_continue().bind_to(sid, reg, BindingLevel::Rebuild);
                vm.card_version().bind_to(sid, reg, BindingLevel::Rebuild);
                vm.kind().bind_to(sid, reg, BindingLevel::Rebuild);
            }
        },
        move |ctx| {
            let report = open.get().then(|| body_vm.report()).flatten();
            let Some(report) = report else {
                return ctx.add(VStack::new());
            };
            ctx.add(
                Panel::new().child(
                    VStack::new()
                        .spacing(6.0)
                        .child(note(tr!(translation_report_public_title(
                            title = report.title()
                        ))))
                        .child(
                            TextWidget::new(lit!(report.clipboard_text()))
                                .style(TextStyleRole::Small),
                        ),
                ),
            )
        },
    );
    // A disclosure: the chevron says which way it is, and so does the button's
    // expanded state for a screen reader.
    let toggle = Rebuilding::new(
        {
            let open = toggle_open.clone();
            move |ctx: &mut BuildContext| {
                open.clone()
                    .bind_to(ctx.self_id(), ctx.binding_registry(), BindingLevel::Rebuild)
            }
        },
        move |ctx| {
            let open = toggle_open.clone();
            let is_open = open.get();
            let chevron = if is_open {
                IconWidget::chevron_down(12.0)
            } else {
                IconWidget::chevron_right(12.0)
            };
            ctx.add(
                HStack::new()
                    .child(
                        Button::new(tr!(translation_report_public()))
                            .variant(ButtonVariant::Ghost)
                            .icon(chevron, IconLocation::Leading)
                            .on_activate_fn(move |_ctx| open.set(!open.get()))
                            .access_customize(move |node| node.set_expanded(is_open)),
                    )
                    .child(Spacer::new()),
            )
        },
    );
    teksu!(
        VStack {
            spacing: 6.0
            child: toggle
            note(tr!(translation_report_public_cost()))
            note(tr!(translation_report_public_steps()))
            child: body
        }
    )
}

/// The label of one of a card's places.
fn place_label(labels: &[(PlaceChoice, String)], choice: &PlaceChoice) -> String {
    labels
        .iter()
        .find(|(k, _)| k == choice)
        .map(|(_, l)| l.clone())
        .unwrap_or_default()
}

fn kind_label(kind: ProblemKind) -> LocalizedString {
    match kind {
        ProblemKind::WrongMeaning => tr!(translation_report_kind_wrong_meaning()),
        ProblemKind::Spelling => tr!(translation_report_kind_spelling()),
        ProblemKind::NotTranslated => tr!(translation_report_kind_not_translated()),
        ProblemKind::Unnatural => tr!(translation_report_kind_unnatural()),
        ProblemKind::Inconsistent => tr!(translation_report_kind_inconsistent()),
        ProblemKind::PluralForm => tr!(translation_report_kind_plural_form()),
        ProblemKind::CutOff => tr!(translation_report_kind_cut_off()),
        ProblemKind::Other => tr!(translation_report_kind_other()),
    }
}

// ── The footer ──────────────────────────────────────────────────────────────

fn footer(vm: &TranslationReportViewModel) -> impl Widget + 'static {
    let status_vm = vm.clone();
    let status = Rebuilding::new(
        {
            let vm = vm.clone();
            move |ctx: &mut BuildContext| {
                let (sid, reg) = (ctx.self_id(), ctx.binding_registry());
                vm.phase().bind_to(sid, reg, BindingLevel::Rebuild);
                vm.notice().bind_to(sid, reg, BindingLevel::Rebuild);
                vm.card_version().bind_to(sid, reg, BindingLevel::Rebuild);
            }
        },
        move |ctx| build_status(&status_vm, ctx),
    );

    let copy_vm = vm.clone();
    let continue_vm = vm.clone();
    let can_continue = vm.can_continue();
    teksu!(
        Padding::symmetric(12.0, 16.0) {
            HStack {
                spacing: 8.0
                Expand::horizontal {
                    child: status
                }
                Button::new(tr!(translation_report_copy_text())) {
                    enabled: can_continue.clone()
                    on_activate_fn: move |ctx| {
                        let clipboard = ctx.app_state::<ClipboardHandle>().cloned();
                        copy_vm.copy_text(clipboard.as_ref());
                    }
                }
                Button::new(tr!(translation_report_close())) {
                    on_activate_fn: |ctx| ctx.close_window()
                }
                Button::new(tr!(translation_report_continue())) {
                    variant: ButtonVariant::Filled
                    enabled: can_continue
                    on_activate_fn: move |ctx| continue_vm.continue_on_github(ctx)
                }
            }
        }
    )
}

/// What the last action did, with **Copy link** and **Report another** once the
/// report has gone to the browser.
fn build_status(vm: &TranslationReportViewModel, ctx: &mut BuildContext) -> WidgetId {
    let mut column = VStack::new().spacing(4.0);
    let phase = vm.phase().get();
    match &phase {
        Phase::Editing => {
            let watched = crate::startup::watched_locales();
            let language = vm.language();
            if watched.contains(&language) {
                column = column.child(note(tr!(translation_report_dev_catalogue(
                    locale = language
                ))));
            }
        }
        Phase::Continued { copied } => {
            column = column.child(note(tr!(translation_report_continued())));
            if *copied {
                column = column.child(note(tr!(translation_report_continued_copied())));
            }
            column = column.child(note(tr!(translation_report_continued_help())));
        }
        Phase::OpenFailed(error) => {
            column = column.child(note(tr!(translation_report_open_failed(
                error = error.clone()
            ))));
        }
    }
    if let Some(notice) = vm.notice().get() {
        column = column.child(note(match notice {
            Notice::TextCopied => tr!(translation_report_text_copied()),
            Notice::LinkCopied => tr!(translation_report_link_copied()),
            Notice::CopyFailed => tr!(translation_report_copy_failed()),
        }));
    }
    if phase != Phase::Editing {
        let link_vm = vm.clone();
        let again_vm = vm.clone();
        column = column.child(
            HStack::new()
                .spacing(8.0)
                .child(
                    Button::new(tr!(translation_report_copy_link())).on_activate_fn(move |ctx| {
                        let clipboard = ctx.app_state::<ClipboardHandle>().cloned();
                        link_vm.copy_link(clipboard.as_ref());
                    }),
                )
                .child(
                    Button::new(tr!(translation_report_another()))
                        .variant(ButtonVariant::Ghost)
                        .on_activate_fn(move |_ctx| again_vm.report_another()),
                )
                .child(Spacer::new()),
        );
    }
    // Announced when it changes, so a screen-reader user hears what Continue did.
    ctx.add(column.access_role(Role::Status))
}

// ── Helpers ─────────────────────────────────────────────────────────────────

/// A block rebuilt from scratch whenever one of the signals `bind` binds changes.
///
/// For the parts of the window that are a function of the view-model's state and
/// hold nothing the writer is typing into.
struct Rebuilding {
    bind: Box<dyn Fn(&mut BuildContext)>,
    make: Box<dyn Fn(&mut BuildContext) -> WidgetId>,
    root_child: Option<WidgetId>,
}

impl Rebuilding {
    fn new(
        bind: impl Fn(&mut BuildContext) + 'static,
        make: impl Fn(&mut BuildContext) -> WidgetId + 'static,
    ) -> Self {
        Self {
            bind: Box::new(bind),
            make: Box::new(make),
            root_child: None,
        }
    }
}

impl std::fmt::Debug for Rebuilding {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Rebuilding").finish_non_exhaustive()
    }
}

impl Widget for Rebuilding {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        (self.bind)(ctx);
        let id = (self.make)(ctx);
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

#[cfg(test)]
mod tests;
