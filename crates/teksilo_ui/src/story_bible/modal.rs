// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The story-bible entry modal: the configuration step the ＋ Create vocabulary opens
//! on a row it has just made.
//!
//! **Nothing is created until Create is pressed.** Every field below lives in
//! local `Signal` state owned by [`EntryPanel`]; Cancel is simply
//! [`EventContext::dismiss_modal`] with no backend call anywhere behind it,
//! which is what makes it free: there is nothing to roll back because nothing
//! was ever asked for. Create (and Create and open) read every field once and
//! hand them to [`super::create::create_entry`] / [`super::create::configure_entry`],
//! the one place either mode actually touches the store.
//!
//! ## One mode, and why there is no location picker
//!
//! `CreateType::StoryBibleEntry` creates its row **immediately**, matching every sibling
//! type exactly (see `OutlineViewModel::add_recommended_returning_id`'s own doc for
//! why), so by the time this widget exists the row already has a place. There is
//! nothing left to pick.
//!
//! It used to have a second mode, for "Add as note", which did carry a
//! [`crate::widgets::destination_picker::DestinationPicker`] because nothing existed
//! yet. That door no longer asks: a tag now says where its notes go, so the whole
//! gesture is one click and there is no modal in it at all. See
//! [`crate::story_bible::capture_flow`].

use std::cell::RefCell;
use std::rc::Rc;

use teksilo::core::BindingLevel;
use teksilo::core::modal::{ModalCloseBehavior, ModalPresentation, ModalRequest};
use teksilo::i18n::LocalizedString;
use teksilo::prelude::*;
use teksilo::text_document::TextDocument;
use teksilo::widgets::rich_text::{EditorHandle, RichTextEditor};
use teksilo::widgets::{
    Button, ButtonVariant, ComboBox, HStack, MaxSize, ModalContainer, Padding, ScrollArea, Spacer,
    TextInput, TextWidget, Toast, VStack,
};

use frontend::AppContext;

use crate::app_ids::AppIds;
use crate::editors::EditorsViewModel;
use crate::note_templates::{NoteTemplatesViewModel, Preset};
use crate::tags::TagsViewModel;
use crate::tags::books::ClearBook;
use crate::tags::cast_add::CastCandidate;
use crate::tags::mention_list::PinReference;
use crate::toast_scope::ToastWorkExt;

use super::create::{self, EntryDraft};
use super::infer_book;

const CARD_W: u32 = 640;
const CARD_H: u32 = 640;

/// Shared by the Create and Create-and-open buttons; `bool` is `open_after`.
type CommitFn = Rc<dyn Fn(bool, &mut EventContext)>;

/// Everything the modal needs that it cannot reach on its own, cloned into the
/// button handlers: a bundle rather than several positional clones, the same
/// shape as `app::recreate_row::RecreateContext` and for the same reason: each
/// is a handle the result closure needs, and a positional list of clones is
/// how the wrong one gets passed.
#[derive(Clone)]
pub struct ModalDeps {
    pub app_ctx: Rc<AppContext>,
    pub ids: AppIds,
    pub tags: TagsViewModel,
    pub templates: NoteTemplatesViewModel,
    pub editors: EditorsViewModel,
}

/// What the caller already knows before the modal opens. Every field here is
/// exactly a starting value for its matching `Signal`: visible, and as
/// changeable as if the writer had typed it themselves, cleared on Cancel like
/// every other pre-fill in this app.
#[derive(Debug, Clone, Default)]
pub struct Prefill {
    pub name: String,
    pub aliases: Vec<String>,
    /// The guessed `books` pre-set. The control that shows it renders whenever
    /// the Work has a Book to file under at all, so a guess is visible and
    /// changeable wherever one can be made: [`infer_book::book_containing`]
    /// names a live `Folder/Book`, which is exactly what
    /// [`crate::docks::inspector::live_books`] offers as a candidate. Only a
    /// Work with no Book at all shows nothing, and there the guess is empty
    /// too.
    pub books: Vec<u64>,
}

/// Which row Create configures.
///
/// A one-variant enum rather than a bare `u64`: it is the argument every function below
/// threads, and naming it is what made the second variant's removal a compile error
/// everywhere it mattered rather than a silent behaviour change.
#[derive(Clone)]
enum Mode {
    /// The row already exists; Create only configures it (see
    /// [`super::create::configure_entry`]).
    Configure { item_id: u64 },
}

/// Open the **configuration** step on a row the ＋ Create vocabulary already
/// made: no location picker, and `books` pre-set by walking backwards from
/// the row's own freshly-landed position (see [`infer_book::book_containing`]).
pub fn present_configure(deps: ModalDeps, item_id: u64, ctx: &mut EventContext) {
    let books = deps
        .ids
        .work_id
        .get()
        .and_then(|work_id| infer_book::book_containing(&deps.app_ctx, work_id, item_id))
        .into_iter()
        .collect();
    let prefill = Prefill {
        books,
        ..Prefill::default()
    };
    open(
        deps,
        Mode::Configure { item_id },
        prefill,
        tr!(story_bible_modal_configure_title()),
        ctx,
    );
}

/// The [`Prefill`] for "Add as note": the selected words as the proposed name, and the
/// scene's own containing Book as the `books` pre-set, unambiguous since a scene sits
/// inside exactly one Book.
///
/// Read by [`crate::story_bible::capture_flow`], which no longer opens this modal at
/// all: the tag says where the note goes, so there is nothing left to confirm. What
/// survives here is the part that was never about the dialog, which is working out what
/// the writer actually selected.
///
/// The `books` guess stays a **modal** pre-set, and that door drops it on purpose:
/// filing is a declaration, and a door that shows no chip row has nothing to declare
/// with. See `capture_flow::draft_for`, which says so where the write happens.
///
/// **No alias.** The selection is the entry's *name*, and a name is already a needle:
/// `skribisto_model::mentions::names_of` emits the title first and longest-first
/// resolution drops the duplicate span, so an alias repeating the title matches
/// nothing new. What it does do is persist: the Story bible card counts
/// `aliases.len()` and would report "1 alias" under a title identical to it, on every
/// entry, making "which of my entries still need aliases" unanswerable from the grid.
/// It was defensible while this fed a modal the writer could delete the pill in
/// (the doc called it the first alias *candidate*); the door that replaced that modal
/// asks nothing and commits immediately.
///
/// `None` only when `item_id` no longer resolves: the row was trashed, or the project
/// changed between the right-click and the intent reaching here, in which case the
/// caller files nothing rather than a note about thin air.
pub fn prefill_from_selection(
    app_ctx: &Rc<AppContext>,
    ids: &AppIds,
    item_id: u64,
    selected_text: &str,
) -> Option<Prefill> {
    // Fetched purely to answer "does this row still exist?" — see the `None` case above.
    frontend::commands::binder_item_commands::get_binder_item(app_ctx, &item_id)
        .ok()
        .flatten()?;
    let name = selected_text.trim().to_string();
    let books = ids
        .work_id
        .get()
        .and_then(|work_id| infer_book::book_containing(app_ctx, work_id, item_id))
        .into_iter()
        .collect();
    Some(Prefill {
        name,
        aliases: Vec::new(),
        books,
    })
}

fn open(
    deps: ModalDeps,
    mode: Mode,
    prefill: Prefill,
    title: LocalizedString,
    ctx: &mut EventContext,
) {
    ctx.present_modal(
        ModalRequest::deferred({
            let title = title.clone();
            move |t| {
                t.add(
                    ModalContainer::new(EntryPanel::new(deps, mode, prefill)).title(title.clone()),
                )
            }
        })
        .presentation(ModalPresentation::InTree)
        .title(title)
        .size(CARD_W, CARD_H)
        .close_behavior(ModalCloseBehavior::EscapeOrClickOutside),
    );
}

/// Which built-in preset, or which of the writer's own saved templates, the
/// picker currently holds. `None` (the `ComboBox`'s own `Option`) means no
/// template: the body stays whatever the small editor holds, typically blank.
#[derive(Clone, PartialEq)]
enum TemplateChoice {
    Preset(Preset),
    Saved(u64),
}

fn template_label(templates: &NoteTemplatesViewModel, choice: &TemplateChoice) -> LocalizedString {
    match choice {
        TemplateChoice::Preset(p) => p.label(),
        TemplateChoice::Saved(id) => templates
            .rows()
            .into_iter()
            .find(|r| r.id == *id)
            .map(|r| LocalizedString::literal(r.name))
            .unwrap_or_else(|| LocalizedString::literal(String::new())),
    }
}

fn template_body(templates: &NoteTemplatesViewModel, choice: &TemplateChoice) -> String {
    match choice {
        TemplateChoice::Preset(p) => p
            .rows()
            .into_iter()
            .next()
            .map(|r| r.body)
            .unwrap_or_default(),
        TemplateChoice::Saved(id) => templates.body_of(*id).unwrap_or_default(),
    }
}

/// Read every field once, exactly as it stands the moment Create is pressed.
fn draft_from(
    name: &Signal<String>,
    tags: &Signal<Vec<u64>>,
    aliases: &Signal<Vec<String>>,
    books: &Signal<Vec<u64>>,
    handle: &Rc<RefCell<Option<EditorHandle>>>,
) -> EntryDraft {
    let body = handle
        .borrow()
        .as_ref()
        .map(|h| h.to_djot())
        .unwrap_or_default();
    EntryDraft {
        title: name.get().trim().to_string(),
        tags: tags.get(),
        aliases: aliases.get(),
        books: books.get(),
        body,
    }
}

/// Commit the draft onto the row [`Mode::Configure`] names, then dismiss and, if
/// `open_after`, reveal the item in an editor tab. A blank
/// name refuses silently: the Create buttons are already disabled on it, so
/// this is only reached that way through direct scripting/testing, and doing
/// nothing is the correct answer either way; never a bare-title row.
fn commit(
    deps: &ModalDeps,
    mode: &Mode,
    draft: EntryDraft,
    open_after: bool,
    ctx: &mut EventContext,
) {
    if draft.title.is_empty() {
        return;
    }
    let stack = deps.ids.stack_id.get();
    let Mode::Configure { item_id } = mode;
    let item_id = *item_id;
    let outcome = create::configure_entry(&deps.app_ctx, stack, item_id, &draft).then_some(item_id);
    let Some(item_id) = outcome else {
        ctx.show_toast(
            Toast::error(tr!(story_bible_modal_failed()))
                .scoped_id("story_bible.failed", deps.ids.work_id.get())
                .target_work(deps.ids.work_id.get()),
        );
        return;
    };
    ctx.dismiss_modal();
    ctx.request_frame();
    if open_after {
        deps.editors.activate(item_id, &draft.title, ctx);
    }
}

struct EntryPanel {
    deps: ModalDeps,
    mode: Mode,
    name: Signal<String>,
    tags: Signal<Vec<u64>>,
    aliases: Signal<Vec<String>>,
    books: Signal<Vec<u64>>,
    template: Signal<Option<TemplateChoice>>,
    doc: TextDocument,
    /// Filled in during `build()`: a `RichTextEditor`'s handle exists the
    /// moment it is constructed, independent of mounting (the same contract
    /// `tabs::shared::editor::writing_column` already relies on), but the
    /// buttons below need to read (Create) and write (a template pick replaces
    /// the preview) it well after construction, so it is parked here.
    handle: Rc<RefCell<Option<EditorHandle>>>,
    root_child: Option<WidgetId>,
    /// Captured during `build()`, read back only by this module's own tests:
    /// there is no "find a widget by its label" query in this codebase, so a
    /// test that needs to click a specific button (Cancel vs Create vs Create
    /// and open) has to know its id, the same way `root_child` already does
    /// for the panel's own layout.
    #[cfg(test)]
    cancel_id: Option<WidgetId>,
    #[cfg(test)]
    create_id: Option<WidgetId>,
    #[cfg(test)]
    create_and_open_id: Option<WidgetId>,
    /// Whether `build()` took the Books-section branch on its last run: the
    /// direct answer to "with one Book there is no books control anywhere in
    /// the modal", read back the same way the button ids are.
    #[cfg(test)]
    books_section_rendered: bool,
    /// How many times `build()` has run, read back the same way. The direct
    /// answer to "did the panel redraw when the filing changed": the chip row
    /// is drawn from a snapshot of `books`, so a change nothing dirties leaves
    /// the writer looking at the value Create is no longer going to write.
    #[cfg(test)]
    builds: usize,
}

impl EntryPanel {
    fn new(deps: ModalDeps, mode: Mode, prefill: Prefill) -> Self {
        Self {
            deps,
            mode,
            name: Signal::new(prefill.name),
            tags: Signal::new(Vec::new()),
            aliases: Signal::new(prefill.aliases),
            books: Signal::new(prefill.books),
            template: Signal::new(None),
            doc: TextDocument::new(),
            handle: Rc::new(RefCell::new(None)),
            root_child: None,
            #[cfg(test)]
            cancel_id: None,
            #[cfg(test)]
            create_id: None,
            #[cfg(test)]
            create_and_open_id: None,
            #[cfg(test)]
            books_section_rendered: false,
            #[cfg(test)]
            builds: 0,
        }
    }

    /// Every live `Folder/Book` in the Work: the same candidate table, gated
    /// the same way (below two, empty), the Inspector's own Books section
    /// reads. See that function's own doc for why this crosses module
    /// boundaries rather than being a second, independently-drifting walk.
    fn book_candidates(&self) -> Vec<CastCandidate> {
        crate::docks::inspector::live_books(&self.deps.app_ctx, &self.deps.ids)
    }
}

impl std::fmt::Debug for EntryPanel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EntryPanel").finish()
    }
}

impl Widget for EntryPanel {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        // The Books section below reads `books` with a plain `get()`, which registers
        // no dependency at all: without this the chip row keeps showing the guess
        // after the writer has cleared it, while Create writes what the signal now
        // holds. Every other field on this panel is drawn by a child widget that
        // binds its own signal (`AliasPillField::build` does exactly this); the chip
        // row is a plain function over a `Vec`, so the binding has to live here.
        // `Rebuild`, because a cleared chip is a child gone, not a repaint.
        self.books
            .bind_to(ctx.self_id(), ctx.binding_registry(), BindingLevel::Rebuild);
        #[cfg(test)]
        {
            self.builds += 1;
        }

        let mut col = VStack::new().spacing(14.0);

        // ── Name ──
        col = col
            .child(
                TextWidget::new(tr!(story_bible_modal_name_label()))
                    .style(TextStyleRole::Tiny)
                    .color(TextRole::Secondary),
            )
            .child(
                TextInput::new(self.name.clone())
                    .placeholder(tr!(story_bible_modal_name_placeholder())),
            );

        // ── Tags ──
        col = col
            .child(
                TextWidget::new(tr!(inspector_tags()))
                    .style(TextStyleRole::Tiny)
                    .color(TextRole::Secondary),
            )
            .child(crate::tags::TagPillField::new(
                self.tags.clone(),
                Rc::new({
                    let mirror = self.tags.clone();
                    move |ids: Vec<u64>, _c: &mut EventContext| mirror.set(ids)
                }),
                self.deps.tags.clone(),
            ));

        // ── Aliases ──
        col = col
            .child(
                TextWidget::new(tr!(inspector_aliases()))
                    .style(TextStyleRole::Tiny)
                    .color(TextRole::Secondary),
            )
            .child(crate::tags::AliasPillField::new(
                self.aliases.clone(),
                Rc::new({
                    let mirror = self.aliases.clone();
                    move |names: Vec<String>, _c: &mut EventContext| mirror.set(names)
                }),
            ));

        // ── Books: whenever the Work has one to file under, the same gate every
        // other Books surface shares. Filing is a field, not a filter — see
        // `docks::inspector::live_books`'s own doc for why one Book is still a Book.
        let candidates = self.book_candidates();
        #[cfg(test)]
        {
            self.books_section_rendered = !candidates.is_empty();
        }
        if !candidates.is_empty() {
            col = col.child(
                TextWidget::new(tr!(books_section()))
                    .style(TextStyleRole::Tiny)
                    .color(TextRole::Secondary),
            );
            let book_ids = self.books.get();
            if book_ids.is_empty() {
                col = col.child(
                    TextWidget::new(tr!(books_empty()))
                        .style(TextStyleRole::Tiny)
                        .color(TextRole::Secondary),
                );
            } else {
                let chips = crate::tags::book_chips(&candidates, &book_ids);
                let clear: ClearBook = {
                    let books = self.books.clone();
                    Rc::new(move |target: u64, _c: &mut EventContext| {
                        let next: Vec<u64> =
                            books.get().into_iter().filter(|&id| id != target).collect();
                        books.set(next);
                    })
                };
                col = col.child(crate::tags::book_chip_row(chips, clear));
            }
            let set: PinReference = {
                let books = self.books.clone();
                Rc::new(move |target: u64, _c: &mut EventContext| {
                    let mut next = books.get();
                    if !next.contains(&target) {
                        next.push(target);
                    }
                    books.set(next);
                })
            };
            col = col.child(crate::tags::book_add_button(candidates, book_ids, 0, set));
        }

        // ── Template: the presets carry the flavour a generic "story bible
        // entry" flow would otherwise lack (Character sheet, Location,
        // Artifact…), alongside whatever templates the writer has saved of
        // their own. Picking one replaces the body preview below wholesale
        // (select-all + insert), the same primitive `editor.insert_template`
        // uses for an already-open note; there is no "insert at creation
        // time" API to call instead, only this one, applied to a document
        // that happens not to be saved anywhere yet.
        col = col.child(
            TextWidget::new(tr!(story_bible_modal_template_label()))
                .style(TextStyleRole::Tiny)
                .color(TextRole::Secondary),
        );
        {
            let choices: Vec<TemplateChoice> = Preset::ALL
                .iter()
                .map(|p| TemplateChoice::Preset(*p))
                .chain(
                    self.deps
                        .templates
                        .menu_rows()
                        .into_iter()
                        .map(|r| TemplateChoice::Saved(r.id)),
                )
                .collect();
            let templates_for_label = self.deps.templates.clone();
            let templates_for_select = self.deps.templates.clone();
            let handle = self.handle.clone();
            let combo = ComboBox::from_items(choices, self.template.clone(), move |choice| {
                template_label(&templates_for_label, choice)
            })
            .placeholder(tr!(story_bible_modal_template_placeholder()))
            .on_select(move |choice: &TemplateChoice, _c: &mut EventContext| {
                let body = template_body(&templates_for_select, choice);
                if let Some(h) = handle.borrow().as_ref() {
                    h.select_all();
                    h.insert_djot(&body);
                }
            });
            col = col.child(combo);
        }

        // ── Body ──
        col = col.child(
            TextWidget::new(tr!(story_bible_modal_body_label()))
                .style(TextStyleRole::Tiny)
                .color(TextRole::Secondary),
        );
        let editor = RichTextEditor::editor(self.doc.clone()).min_lines(6);
        *self.handle.borrow_mut() = Some(editor.handle());
        col = col.child(MaxSize::height(180.0).child(ScrollArea::new().child(editor)));

        // ── Buttons ──
        let name_ok = self.name.map(|s| !s.trim().is_empty());
        let deps = self.deps.clone();
        let mode = self.mode.clone();
        let name = self.name.clone();
        let tags = self.tags.clone();
        let aliases = self.aliases.clone();
        let books = self.books.clone();
        let handle = self.handle.clone();
        let do_commit: CommitFn = Rc::new(move |open_after, c| {
            let draft = draft_from(&name, &tags, &aliases, &books, &handle);
            commit(&deps, &mode, draft, open_after, c);
        });

        let actions = {
            let create_and_open = do_commit.clone();
            let create_only = do_commit.clone();
            let cancel_id = ctx.add(
                Button::new(tr!(story_bible_modal_cancel()))
                    .variant(ButtonVariant::Plain)
                    .on_activate_fn(|c| c.dismiss_modal()),
            );
            let create_and_open_id = ctx.add(
                Button::new(tr!(story_bible_modal_create_and_open()))
                    .variant(ButtonVariant::Plain)
                    .enabled(name_ok.clone())
                    .on_activate_fn(move |c| create_and_open(true, c)),
            );
            let create_id = ctx.add(
                Button::new(tr!(story_bible_modal_create()))
                    .variant(ButtonVariant::Filled)
                    .enabled(name_ok)
                    .on_activate_fn(move |c| create_only(false, c)),
            );
            #[cfg(test)]
            {
                self.cancel_id = Some(cancel_id);
                self.create_and_open_id = Some(create_and_open_id);
                self.create_id = Some(create_id);
            }
            HStack::new()
                .spacing(8.0)
                .add_child(cancel_id)
                .child(Spacer::new())
                .add_child(create_and_open_id)
                .add_child(create_id)
        };

        let root = ctx.add(
            Padding::uniform(4.0).child(
                VStack::new()
                    .spacing(16.0)
                    .child(ScrollArea::new().child(col))
                    .child(actions),
            ),
        );
        self.root_child = Some(root);
        vec![root]
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

    // Test-only introspection: the button ids `build()` captures are only
    // ever read back by this module's own tests, via `WidgetTree::widget_as_any`.
    // There is no other query in this toolkit for "which widget is that
    // specific button". See the `cancel_id`/`create_id`/`create_and_open_id`
    // fields' own doc.
    #[cfg(test)]
    fn as_any(&self) -> Option<&dyn std::any::Any> {
        Some(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use frontend::commands::{binder_commands, binder_item_commands, work_commands};
    use frontend::common::direct_access::binder_item::BinderItemRelationshipField;
    use frontend::common::entities::{BinderItemRole, BinderItemSubRole, GoalUnit};
    use frontend::direct_access::{CreateBinderDto, CreateBinderItemDto, CreateWorkDto};
    use teksilo::core::widget_tree::WidgetTree;

    struct Fixture {
        app_ctx: Rc<AppContext>,
        ids: AppIds,
        binder_id: u64,
    }

    /// A Work with one Binder, no Books yet: the "fewer than two Books" shape
    /// every gated surface in this feature has to answer identically to a
    /// project that never heard of the field.
    fn seed() -> Fixture {
        let app_ctx = Rc::new(AppContext::new());
        let work = work_commands::create_orphan_work(&app_ctx, None, &CreateWorkDto::default())
            .expect("create work");
        let binder = binder_commands::create_binder(
            &app_ctx,
            None,
            &CreateBinderDto {
                name: "Manuscript".into(),
                activated: true,
                ..Default::default()
            },
            work.id,
            -1,
        )
        .expect("create binder");
        let ids = AppIds::new();
        ids.work_id.set(Some(work.id));
        Fixture {
            app_ctx,
            ids,
            binder_id: binder.id,
        }
    }

    fn create_book(f: &Fixture, index: i32, title: &str) -> u64 {
        binder_item_commands::create_binder_item(
            &f.app_ctx,
            None,
            &CreateBinderItemDto {
                title: title.into(),
                role: BinderItemRole::Folder,
                sub_role: BinderItemSubRole::Book,
                activated: true,
                is_exportable: true,
                ..Default::default()
            },
            f.binder_id,
            index,
        )
        .expect("create book")
        .id
    }

    fn create_note(f: &Fixture, index: i32, title: &str) -> u64 {
        binder_item_commands::create_binder_item(
            &f.app_ctx,
            None,
            &CreateBinderItemDto {
                title: title.into(),
                role: BinderItemRole::Item,
                sub_role: BinderItemSubRole::Note,
                activated: true,
                is_exportable: true,
                ..Default::default()
            },
            f.binder_id,
            index,
        )
        .expect("create note")
        .id
    }

    fn tags_vm(f: &Fixture) -> TagsViewModel {
        TagsViewModel::new(
            crate::models::WorkTagsListModel::new(f.app_ctx.clone(), f.ids.clone()),
            f.ids.clone(),
        )
    }

    fn templates_vm(f: &Fixture) -> NoteTemplatesViewModel {
        NoteTemplatesViewModel::new(
            crate::models::WorkNoteTemplatesListModel::new(f.app_ctx.clone(), f.ids.clone()),
            f.ids.clone(),
        )
    }

    /// A single blank typography bundle, reused across every field:
    /// `EditorTypographySet` has no `detached()`/`default()` shortcut, so this
    /// mirrors `editors_vm::tests::test_typography`'s own construction rather
    /// than reaching for one that does not exist.
    fn blank_typography() -> crate::settings::EditorTypography {
        crate::settings::EditorTypography {
            font_family: Signal::new(String::new()),
            size: Signal::new(1.0),
            line_height: Signal::new(1.5),
            first_line_indent: Signal::new(0.0),
            para_spacing_before: Signal::new(0.0),
            para_spacing_after: Signal::new(0.0),
            size_range: crate::settings::TypographySizeRange::default(),
        }
    }

    /// A minimal, real `EditorsViewModel`: the same construction
    /// `editors::editors_vm::tests::editors_with` uses, duplicated rather than
    /// exposed, since that helper is private to its own module and this is the
    /// one other place in the crate that needs a genuine "did Create and open
    /// actually open a tab" answer rather than a stub.
    fn editors_vm(f: &Fixture) -> crate::editors::EditorsViewModel {
        let save_state = crate::save::SaveStateViewModel::new(f.app_ctx.clone(), f.ids.clone());
        let docs = crate::models::OpenDocsStore::new(f.app_ctx.clone());
        let tree_expansion = crate::settings::TreeExpansionViewModel::new(
            f.app_ctx.clone(),
            f.ids.clone(),
            crate::models::TreeExpansionService::in_memory_default(),
        );
        let typo = crate::settings::EditorTypographySet {
            scene: blank_typography(),
            synopsis: blank_typography(),
            notes: blank_typography(),
            corkboard: blank_typography(),
            distraction_free: blank_typography(),
        };
        crate::editors::EditorsViewModel::new(
            f.app_ctx.clone(),
            Signal::new(700.0),
            Signal::new(true),
            Signal::new(crate::shared::SynopsisPlacement::default()),
            Signal::new(crate::SYNOPSIS_SIDE_WIDTH_DEFAULT),
            typo,
            crate::shared::TypewriterSettings::off(),
            crate::shared::CaretHighlightSettings::off(),
            crate::settings::EditorViewMemory::detached(false),
            crate::settings::CorkboardDefaults::detached(),
            f.ids.clone(),
            docs,
            Signal::new(false),
            save_state,
            Signal::new(false),
            tree_expansion,
            Signal::new(false),
            Signal::new(620.0),
            crate::go::GoAvailability::new(),
            crate::format::FormatViewModel::detached(),
            crate::writing_session::WritingGamesViewModel::detached(),
            Signal::new(GoalUnit::default()),
            tags_vm(f),
            crate::mentions::MentionIndex::new(f.app_ctx.clone(), f.ids.clone()),
        )
    }

    fn deps(f: &Fixture) -> ModalDeps {
        ModalDeps {
            app_ctx: f.app_ctx.clone(),
            ids: f.ids.clone(),
            tags: tags_vm(f),
            templates: templates_vm(f),
            editors: editors_vm(f),
        }
    }

    fn item_content_count(f: &Fixture, item_id: u64) -> usize {
        binder_item_commands::get_binder_item_relationship(
            &f.app_ctx,
            &item_id,
            &BinderItemRelationshipField::Contents,
        )
        .unwrap_or_default()
        .len()
    }

    /// **The selected words arrive as the proposed name**, C1b's own
    /// guarantee, tested at the pure resolution function rather than through a
    /// live right-click, since nothing about turning a selection into a
    /// pre-fill needs a widget tree.
    ///
    /// And as the name **only**: an alias repeating the title matches nothing
    /// the title does not already match, and the door this feeds commits
    /// without showing it, so every captured entry would report "1 alias" on
    /// the grid for an alias its writer never wrote.
    #[test]
    fn the_selection_arrives_as_the_proposed_name_and_nothing_else() {
        let f = seed();
        let item_id = create_note(&f, 0, "New note");
        let prefill = prefill_from_selection(&f.app_ctx, &f.ids, item_id, "  Elizabeth Bennet  ")
            .expect("a live item must resolve a prefill");
        assert_eq!(prefill.name, "Elizabeth Bennet");
        assert!(
            prefill.aliases.is_empty(),
            "the entry's own name is not one of its aliases"
        );
    }

    /// **Inference pre-sets `books` from the scene's Book, and the writer can
    /// change it before Create.** The pre-set is only ever a starting `Signal`
    /// value, proven here by overriding it before the draft is read, exactly
    /// as a writer clicking a different chip would.
    #[test]
    fn books_are_preset_from_the_containing_book_and_stay_editable() {
        let f = seed();
        let book_one = create_book(&f, 0, "Book One");
        let _book_two = create_book(&f, 1, "Book Two");
        let scene = binder_item_commands::create_binder_item(
            &f.app_ctx,
            None,
            &CreateBinderItemDto {
                title: "A scene".into(),
                role: BinderItemRole::Item,
                sub_role: BinderItemSubRole::Scene,
                activated: true,
                is_exportable: true,
                indent: 1,
                ..Default::default()
            },
            f.binder_id,
            1,
        )
        .expect("create scene")
        .id;

        let prefill =
            prefill_from_selection(&f.app_ctx, &f.ids, scene, "Locke Manor").expect("prefill");
        assert_eq!(
            prefill.books,
            vec![book_one],
            "the scene's own containing Book is the guessed pre-set"
        );

        // The writer changes it before Create: a plain `Signal` write, the
        // same act clicking a different chip in the modal performs.
        let books = Signal::new(prefill.books);
        books.set(vec![]);
        let name = Signal::new(String::new());
        let tags = Signal::new(Vec::new());
        let aliases = Signal::new(Vec::new());
        let handle = Rc::new(RefCell::new(None));
        let draft = draft_from(&name, &tags, &aliases, &books, &handle);
        assert!(
            draft.books.is_empty(),
            "the writer's own change must win over the guess"
        );
    }

    /// Build a mounted panel and read back its captured button ids via the
    /// `Widget::as_any` introspection hook: the arena owns the widget the
    /// moment it is added, so this is the only way a test reaches the ids
    /// `build()` captured, the same mechanism `TableView`/`TreeTableView`
    /// tests already lean on elsewhere in this toolkit.
    fn mount(app_ctx: &Rc<AppContext>, panel: EntryPanel) -> (WidgetTree, WidgetId, ButtonIds) {
        let mut tree = crate::test_support::tree_with_events(app_ctx);
        let id = tree.add_boxed(Box::new(panel));
        // Unbounded height: an *exact* proposal lets the inner `ScrollArea` fill
        // and report back the proposed height regardless of content, which is
        // exactly the size `the_books_control_appears_with_the_first_book_not_the_second` needs to
        // tell apart. This panel is only ever laid out inside a modal's own
        // fixed card in the real app, but every button click test here only
        // needs the tree built and clickable, not a particular pixel size.
        tree.layout(teksilo::prelude::SizeProposal {
            width: Some(CARD_W as f32),
            height: None,
        });
        let ids = tree
            .widget_as_any(id)
            .and_then(|a| a.downcast_ref::<EntryPanel>())
            .map(|p| ButtonIds {
                cancel: p.cancel_id.expect("built"),
                create: p.create_id.expect("built"),
                create_and_open: p.create_and_open_id.expect("built"),
            })
            .expect("EntryPanel must be introspectable via as_any");
        (tree, id, ids)
    }

    struct ButtonIds {
        cancel: WidgetId,
        create: WidgetId,
        create_and_open: WidgetId,
    }

    /// **Create and open opens the new item.**
    #[test]
    fn create_and_open_opens_the_new_item() {
        let f = seed();
        let item_id = create_note(&f, 0, "New story bible entry");
        let deps = deps(&f);
        let panel = EntryPanel::new(
            deps.clone(),
            Mode::Configure { item_id },
            Prefill {
                name: "Elizabeth Bennet".into(),
                ..Prefill::default()
            },
        );
        let (mut tree, _id, ids) = mount(&f.app_ctx, panel);

        crate::test_support::click(&mut tree, ids.create_and_open);

        assert!(
            deps.editors
                .tab_item_ids(crate::editors::Side::Primary)
                .contains(&item_id),
            "Create and open must reveal the new item in an editor tab"
        );
        // Only holds against the real backend: see
        // `create::tests::configure_entry_names_tags_aliases_books_and_body_in_one_more_undo_entry`'s
        // own note on why `configure_entry`'s title write does not reach this
        // test's `ctx` under `mocks`.
        if cfg!(not(feature = "mocks")) {
            let after = binder_item_commands::get_binder_item(&f.app_ctx, &item_id)
                .expect("read")
                .expect("item");
            assert_eq!(after.title, "Elizabeth Bennet");
        }
    }

    /// Plain Create configures the row exactly like Create and open, but
    /// never opens it: the two buttons must actually differ, not just carry
    /// different labels.
    #[test]
    fn create_alone_configures_but_does_not_open() {
        let f = seed();
        let item_id = create_note(&f, 0, "New story bible entry");
        let deps = deps(&f);
        let panel = EntryPanel::new(
            deps.clone(),
            Mode::Configure { item_id },
            Prefill {
                name: "Elizabeth Bennet".into(),
                ..Prefill::default()
            },
        );
        let (mut tree, _id, ids) = mount(&f.app_ctx, panel);

        crate::test_support::click(&mut tree, ids.create);

        // Only holds against the real backend: see
        // `create::tests::configure_entry_names_tags_aliases_books_and_body_in_one_more_undo_entry`'s
        // own note on why `configure_entry`'s title write does not reach this
        // test's `ctx` under `mocks`.
        if cfg!(not(feature = "mocks")) {
            let after = binder_item_commands::get_binder_item(&f.app_ctx, &item_id)
                .expect("read")
                .expect("item");
            assert_eq!(
                after.title, "Elizabeth Bennet",
                "Create must still configure"
            );
        }
        assert!(
            !deps
                .editors
                .tab_item_ids(crate::editors::Side::Primary)
                .contains(&item_id),
            "plain Create must not open an editor tab"
        );
    }

    /// **Cancel after filling every field creates nothing at all.** Every
    /// local `Signal` is pushed to a non-default value (tags, aliases,
    /// books, a body) and Cancel is clicked; the row this modal was
    /// configuring must come back out exactly as it went in.
    #[test]
    fn cancel_after_filling_every_field_leaves_the_item_untouched() {
        let f = seed();
        let book_one = create_book(&f, 0, "Book One");
        let _book_two = create_book(&f, 1, "Book Two");
        let item_id = create_note(&f, 2, "New story bible entry");
        let deps = deps(&f);
        let mut panel = EntryPanel::new(
            deps.clone(),
            Mode::Configure { item_id },
            Prefill {
                name: "Elizabeth Bennet".into(),
                aliases: vec!["Lizzy".into()],
                books: vec![book_one],
            },
        );
        // A tag pick too: Cancel must discard this along with everything
        // else, never write it on the way out. The id need not resolve to a
        // real tag: the point is that `configure_entry` is never reached at
        // all, so nothing ever reads it back.
        panel.tags = Signal::new(vec![42]);
        let (mut tree, _id, ids) = mount(&f.app_ctx, panel);

        crate::test_support::click(&mut tree, ids.cancel);

        let after = binder_item_commands::get_binder_item(&f.app_ctx, &item_id)
            .expect("read")
            .expect("item");
        assert_eq!(
            after.title, "New story bible entry",
            "Cancel must not rename the item"
        );
        assert!(after.aliases.is_empty(), "Cancel must not write aliases");
        assert!(after.books.is_empty(), "Cancel must not write books");
        assert_eq!(
            item_content_count(&f, item_id),
            0,
            "Cancel must not create a content row"
        );
    }

    /// **With one Book there is no books control anywhere in the modal**,
    /// and with zero Books, and with none at all in a project too small to
    /// have run C0's own migration path. `build()`'s own gate is read back
    /// via the same `as_any` introspection the button ids use.
    #[test]
    fn the_books_control_appears_with_the_first_book_not_the_second() {
        let no_books = seed();
        let panel_none = EntryPanel::new(
            deps(&no_books),
            Mode::Configure { item_id: 0 },
            Prefill::default(),
        );
        let (_t, id_none, _) = mount(&no_books.app_ctx, panel_none);
        assert!(
            !books_section_rendered(&_t, id_none),
            "zero Books: no control"
        );

        let one_book = seed();
        let _only_book = create_book(&one_book, 0, "Book One");
        let panel_one = EntryPanel::new(
            deps(&one_book),
            Mode::Configure { item_id: 0 },
            Prefill::default(),
        );
        let (tree_one, id_one, _) = mount(&one_book.app_ctx, panel_one);
        assert!(
            books_section_rendered(&tree_one, id_one),
            "one Book is still a Book to file under: filing is a field, not a filter, \
             and gated at two it could never be set on a one-Book project at all"
        );

        let two_books = seed();
        let _a = create_book(&two_books, 0, "Book One");
        let _b = create_book(&two_books, 1, "Book Two");
        let panel_two = EntryPanel::new(
            deps(&two_books),
            Mode::Configure { item_id: 0 },
            Prefill::default(),
        );
        let (tree_two, id_two, _) = mount(&two_books.app_ctx, panel_two);
        assert!(
            books_section_rendered(&tree_two, id_two),
            "two Books: the control must appear"
        );
    }

    fn books_section_rendered(tree: &WidgetTree, id: WidgetId) -> bool {
        tree.widget_as_any(id)
            .and_then(|a| a.downcast_ref::<EntryPanel>())
            .expect("EntryPanel must be introspectable via as_any")
            .books_section_rendered
    }

    fn panel(tree: &WidgetTree, id: WidgetId) -> &EntryPanel {
        tree.widget_as_any(id)
            .and_then(|a| a.downcast_ref::<EntryPanel>())
            .expect("EntryPanel must be introspectable via as_any")
    }

    /// **Changing the filing redraws the row that shows it.**
    ///
    /// The chip row and the add button are plain functions over a `Vec<u64>`,
    /// so nothing under them holds the signal and nothing under them can bind
    /// it: the panel itself has to. Read with a bare `get()` and left unbound,
    /// a cleared chip stayed on screen while Create wrote the emptied value,
    /// and the writer's only evidence was the entry filed under a Book they
    /// had just removed. Every other field here is drawn by a child widget
    /// that binds its own signal, which is why this is the one that broke.
    #[test]
    fn changing_the_filing_rebuilds_the_panel() {
        let f = seed();
        let book_one = create_book(&f, 0, "Book One");
        let _book_two = create_book(&f, 1, "Book Two");
        let panel_widget = EntryPanel::new(
            deps(&f),
            Mode::Configure { item_id: 0 },
            Prefill {
                books: vec![book_one],
                ..Prefill::default()
            },
        );
        let (mut tree, id, _) = mount(&f.app_ctx, panel_widget);
        assert!(
            books_section_rendered(&tree, id),
            "two Books: the chip row under test must be on screen"
        );
        let before = panel(&tree, id).builds;
        let books = panel(&tree, id).books.clone();

        // Exactly what the chip's own clear handler does.
        books.set(Vec::new());
        // `layout` is what walks a pending signal change into the arena; laying
        // out a settled tree again costs nothing, so the loop only makes the
        // assertion independent of how many passes one change needs.
        for _ in 0..3 {
            tree.layout(teksilo::prelude::SizeProposal {
                width: Some(CARD_W as f32),
                height: None,
            });
        }

        assert!(
            panel(&tree, id).builds > before,
            "clearing the chip must dirty the panel that draws it, or the writer \
             keeps looking at a filing Create is no longer going to write"
        );
    }

    /// A template's body resolves to the preset's own text: the other half
    /// of "a template's body lands as the note's content" (the write side is
    /// covered by `create::tests::a_templates_body_lands_as_the_notes_content`).
    #[test]
    fn a_preset_choice_resolves_to_its_own_body() {
        let f = seed();
        let templates = templates_vm(&f);
        let choice = TemplateChoice::Preset(Preset::CharacterSheet);
        let body = template_body(&templates, &choice);
        assert_eq!(
            body,
            Preset::CharacterSheet.rows()[0].body,
            "a preset choice must resolve to exactly that preset's own body"
        );
        assert!(!body.trim().is_empty());
    }
}
