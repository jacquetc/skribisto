// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! `Item/Note`'s **Details** segment: the story-bible fields, at the tab's full
//! width, so a writer can flesh out a character or a place without an Inspector
//! dock open beside the editor at all.
//!
//! Almost the same field set as [`crate::docks::inspector::story_bible`], reused
//! deliberately rather than reinvented: the tag/alias pill fields, the Books
//! chips, the cast list and the point-of-view chips all come straight from
//! [`crate::tags`], and every write still goes through
//! [`crate::singles::SingleBinderItem`]'s existing setters (`set_tags`,
//! `set_aliases`, `set_books`, `set_references`, `set_point_of_view`), the exact
//! commands the dock already issues, with the exact same undo behaviour. A
//! second, home-grown writer for any of those five relationships is exactly how
//! this segment and the dock would quietly start disagreeing about what "cast"
//! or "filed under" means.
//!
//! ## What the dock's plumbing does not need to be built twice here
//!
//! [`crate::docks::inspector::Inspector`] rebuilds itself for **whichever item
//! currently has focus**, which is why its own probe re-targets on every
//! rebuild (`if self.probe.id() != item_id`) and its cast section runs a
//! debounced, frame-ticked [`crate::tags::LiveCastOverlay`] to catch a
//! keystroke without re-exporting the focused document on every one of them. A
//! `ContentTab` never re-targets: [`note_details_pane`] is built once per tab,
//! against the one item that tab already is, for as long as that tab exists. So
//! [`NoteDetailsPane`] points its single [`SingleBinderItem`] probe at
//! [`ContentTab::item_id`] once, at construction, and never again, and reads
//! this item's own live prose straight off [`crate::tabs::ProseField::djot`]
//! (the same `to_djot` a debounced overlay would eventually produce anyway) each
//! time this pane actually rebuilds, rather than polling it every frame. That
//! rebuild is not tied to every keystroke either: nothing here binds to
//! `OpenDoc::edit_gen` at `BindingLevel::Rebuild`, for the same reason
//! `LiveCastOverlay`'s own doc gives ("typing must not rebuild this dock"). A
//! keystroke in the "Note" segment reaches this one the next time the writer
//! actually switches to it, or the next time a tag/scan/mention event rebuilds
//! it for an unrelated reason, which is current enough for a page the writer is
//! not simultaneously looking at.
//!
//! Also dropped: the dock's per-field "echo the write into a local mirror only
//! once it lands" dance. That mirror exists there because the dock's tag/alias/
//! books probes are *separate* `SingleBinderItem`s from the header probe that
//! actually drives its rebuild, so a successful write on one of them would not,
//! by itself, be seen until the next external `BinderItem::Updated` event came
//! back around. Here every write and the one rebuild-triggering read
//! ([`Self::probe`]'s own `dto_signal`) go through the **same** probe, whose
//! setters already refresh that signal synchronously on success. So the next
//! frame's rebuild reads the field straight back out of the DTO, correctly,
//! with no separate mirror to keep in step or fail to echo when a write is
//! silently refused (the item was trashed by another window mid-click, say).
//!
//! And "Apply to children" (the dock's Books section only offers it when the
//! focused row has a subtree) never appears here at all: `Item/Note` is always
//! a leaf, so there is never a subtree to apply anything to.
//!
//! ## What is new: "Appears in the manuscript" does not reuse `MentionList`
//!
//! [`crate::tags::MentionList`] renders [`MentionRow::title`] as the row's own
//! headline, which is correct for the *cast* direction (there, `title` is the
//! target character's name, exactly what a cast row should say) and wrong for
//! *this* direction. [`MentionIndex::backlinks_for`] returns rows whose
//! `target_id` is always this same note, so `title` is always resolved against
//! this note's own name too. Every row would show the same headline, and the
//! one thing this section exists to say (*which* scene or note wrote it) would
//! never appear at all. So this section resolves `owner_id` through
//! [`crate::tags::documents_in_manuscript_order`] instead, which answers the document
//! titles and the reading order in the same walk, and shows *that* title as the row's
//! headline. The Inspector's own backlink list goes through the same function, so the
//! two surfaces cannot come to disagree about either.
//!
//! The evidence sentence is quoted in the row itself, not tucked behind a
//! hover the way the dock's narrow width forces it to be: a full tab has the
//! room to show what was actually written, and "this is what the segment is
//! for" only lands if the sentence is visible without an extra gesture.

use std::collections::{HashMap, HashSet};
use std::rc::Rc;

use teksilo::core::BindingLevel;
use teksilo::core::accesskit::Role;
use teksilo::core::widget::WidgetPlacement;
use teksilo::prelude::*;
use teksilo::widgets::{
    Button, ButtonVariant, ColumnFlow, Expand, GroupHeader, HStack, IconButton, Padding,
    PopoverButton, ScrollArea, Segment, SegmentId, SegmentedControl, Spacer, TextWidget, VStack,
};

use frontend::AppContext;

use crate::app_ids::AppIds;
use crate::intents::AppIntent;
use crate::mentions::{MentionIndex, MentionRow};
use crate::models::OpenDoc;
use crate::singles::SingleBinderItem;
use crate::tags::alias_pill_field::SetAliases;
use crate::tags::books::ClearBook;
use crate::tags::cast_add::{CastAddPopover, CastCandidate};
use crate::tags::mention_list::{CONFIRM_GLYPH, ConfirmPresence, PinReference};
use crate::tags::tag_pill_field::SetTags;
use crate::tags::{
    AliasPillField, TagPillField, TagsViewModel, book_add_button, book_chip_row, book_chips,
};
use crate::tooltip_registry::CONCEPT_TAG;
use crate::widgets::tip::RichTip;

use super::{ContentTab, TitlePart, shared, title_field};

/// The Details segment's body: `Some(MentionIndex)` is still read from
/// `app_state` inside [`NoteDetailsPane::build`], exactly the way
/// [`crate::tabs::story_bible_place::story_bible_pane`] reads it: this is a
/// plain composition function with no constructor-threaded handle to reach it
/// through, and an `Item/Note` tab is always about the one open project a
/// single window has.
///
/// The tag palette is **not** read that way any more. It used to be, and that
/// was BUG 1: `ctx.app_state::<TagsViewModel>()` resolves to whichever
/// session last registered one, which at first launch (no project open yet)
/// is `startup.rs`'s throwaway `WorkSession` on a fresh, never-seeded
/// `AppIds`, so the "+" popover's `TagsViewModel::create`, which needs a real
/// `work_id`, silently created nothing. [`ContentTab::tags`] is threaded from
/// the same `WorkSession` [`docks::inspector::Inspector`](crate::docks::inspector::Inspector)
/// already receives through its own constructor, so this pane's handle is
/// bound to the tab's actual, open Work regardless of what (if anything) is
/// registered as `app_state`.
///
/// Wired into the tab's `SegmentedControl` / `Switcher` pair by
/// [`crate::tabs::shared::item_note_segmented`], alongside `SEG_NOTE_OWN` and
/// `SEG_NOTE_IN_PROSE`.
pub(crate) fn note_details_pane(tab: &ContentTab) -> Box<dyn Widget> {
    let app_ctx = tab.app_ctx();
    let probe = SingleBinderItem::new(app_ctx.clone());
    probe.set_id(Some(tab.item_id()));
    Box::new(NoteDetailsPane {
        ids: tab.ids().clone(),
        item_id: tab.item_id(),
        open_doc: tab.open_doc.clone(),
        set_tags: tab.set_tags_fn(),
        tags: tab.tags(),
        mention_index: tab.mention_index(),
        appears_in_book: Signal::new(None),
        // Built once, here, and held for as long as this pane is: **not**
        // rebuilt fresh inside `build()`. `title_input` binds to `name.value`
        // directly; a fresh `TitleField` on every rebuild would reseed that
        // signal from the persisted title each time, discarding a rename the
        // writer is still mid-keystroke on the moment anything else (a scan
        // landing, a tag changing) rebuilds this pane for an unrelated reason.
        // Same reasoning as `Inspector` holding its own probe across rebuilds
        // rather than minting one per build. `Rc`-wrapped so `build()` can
        // clone a handle into the commit closure below without fighting the
        // borrow checker over a field also borrowed for `title_input` itself.
        name: Rc::new(title_field(&app_ctx, tab.item_id(), TitlePart::Title)),
        app_ctx,
        probe,
        root: None,
    })
}

struct NoteDetailsPane {
    app_ctx: Rc<AppContext>,
    ids: AppIds,
    item_id: u64,
    /// Which Book's appearances the right column is showing.
    ///
    /// Held on the pane, not minted per build: this pane rebuilds whenever a scan lands or
    /// a tag changes, and a fresh signal each time would throw the writer back to the
    /// first Book mid-read. `None` until the first build picks one.
    appears_in_book: Signal<Option<SegmentId>>,
    mention_index: crate::mentions::MentionIndex,
    /// This item's shared editing state: read for its `tags` mirror (the same
    /// one `set_tags` below writes, and the same one any other open view of
    /// this item already shares) and for `main`'s live Djot text, the cast
    /// section's own "what has been typed so far" input.
    open_doc: Rc<OpenDoc>,
    /// This tab's existing tags writer. See the module doc's "reuse... the
    /// same commands" paragraph.
    set_tags: SetTags,
    /// This tab's tag palette, threaded from [`ContentTab::tags`]. See this
    /// module's own doc for the bug reading it from `app_state` used to cause.
    tags: TagsViewModel,
    /// The name field. See its construction site's own comment for why this
    /// lives here rather than being rebuilt inside `build()`.
    name: Rc<crate::tabs::TitleField>,
    /// Fixed to `item_id` once, at construction, and never re-pointed: see the
    /// module doc's "what the dock's plumbing does not need to be built twice
    /// here" section.
    probe: SingleBinderItem,
    root: Option<WidgetId>,
}

impl std::fmt::Debug for NoteDetailsPane {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NoteDetailsPane").finish()
    }
}

impl Widget for NoteDetailsPane {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        // The one rebuild trigger this pane needs: every write below goes through
        // this same probe, and every one of `SingleBinderItem`'s setters already
        // refreshes its own `dto_signal` synchronously on success; see the module
        // doc. Re-subscribed every build, not once: `BuildContext::subscribe_event`
        // scopes a subscription to the widget's current build and drops it on the
        // next one (the same rule the dock's own probe follows).
        self.probe.dto_signal().bind_to(
            ctx.self_id(),
            ctx.binding_registry(),
            BindingLevel::Rebuild,
        );
        self.probe.wire(ctx);

        // Not yet loaded (or the item was trashed/deleted out from under an
        // already-open tab): degrade to nothing rather than panic on an
        // `Option` that genuinely can be `None` for a moment.
        let Some(d) = self.probe.dto() else {
            self.root = None;
            return Vec::new();
        };
        let stack = self.ids.stack_id.clone();

        // The tag palette is a constructor-threaded handle (`ContentTab::tags`), always
        // present. See this module's own doc for why it is no longer an `app_state`
        // lookup. The mention index still is: a widget test, or a window built before
        // a project has finished loading, may have none, and this pane must show
        // whatever it still can rather than panic. Only the sections *that* one
        // actually feeds are skipped; the rest (the name, Tags, and Books, which reads
        // straight through the backend) still render.
        self.tags.changed_signal().bind_to(
            ctx.self_id(),
            ctx.binding_registry(),
            BindingLevel::Rebuild,
        );
        let mention_index = Some(self.mention_index.clone());
        if let Some(index) = &mention_index {
            index.changed_signal().bind_to(
                ctx.self_id(),
                ctx.binding_registry(),
                BindingLevel::Rebuild,
            );
        }
        // The Book bar under "Appears in the manuscript" writes this signal and nothing
        // else reads it, so without this binding the chip moved and the list under it did
        // not: `SegmentedControl` binds the selection to *itself* only (Relayout /
        // AccessibilityOnly), which repaints the strip but never rebuilds this pane. The
        // rows, their quoted evidence, the per-row confirm buttons and "Confirm every
        // appearance" (scoped to the Book on screen) all come from `backlinks_section`'s
        // own read of this value, so they stayed on the previous Book while the bar said
        // otherwise. Same binding `note_in_prose` already gives its own Book bar.
        self.appears_in_book
            .bind_to(ctx.self_id(), ctx.binding_registry(), BindingLevel::Rebuild);

        let discoverable_ids: HashSet<u64> = self
            .tags
            .rows()
            .into_iter()
            .filter(|t| t.discoverable)
            .map(|t| t.id)
            .collect();
        let is_discoverable = item_is_discoverable(&d.tags, &discoverable_ids);

        // ── Two columns, deliberately ────────────────────────────────────────
        //
        // The whole reason this segment exists rather than sending the writer to the
        // Inspector is room, and room spent on a taller stack of the same fields buys
        // nothing: the dock already stacks them, and it does it in less space. What a
        // tab has that a dock does not is *width*, so the fields take one column and the
        // manuscript takes the other, and the writer can set an alias while looking at
        // the sentence that made them want it.
        //
        // The right column is the point. Everything on the left is a fact the writer is
        // stating; everything on the right is what they have already written about this
        // person, quoted. A form with more room would have been the lazy answer.
        let mut fields = VStack::new().spacing(22.0);

        fields = fields.child(name_field(&self.name, stack.clone()));

        fields = fields.child(tags_section(
            self.open_doc.tags.clone(),
            self.set_tags.clone(),
            self.tags.clone(),
        ));

        // Aliases only make sense on an item the mention index will actually
        // scan for: a discoverable tag is what puts it in that set. See
        // `docks::inspector::story_bible`'s own reasoning, reused verbatim.
        if is_discoverable {
            fields = fields.child(aliases_section(
                &self.probe,
                d.aliases.clone(),
                self.item_id,
                stack.get(),
                mention_index.as_ref(),
            ));
        }

        // **One Book is still a Book to file under.** This was gated at two, on the
        // reasoning that a one-Book writer has nothing to declare — which is true of a
        // *filter* and false of a *field*. Filing is not "which Book am I looking at",
        // it is "which Book is this entry part of", and the model refuses to infer it:
        // empty `books` reads as not yet filed, never as every book (see the field's own
        // doc). So a one-Book project could not file anything, every entry stayed unfiled
        // for good, and the moment a second Book arrived the writer had a whole cast to
        // file retrospectively. With no Book at all there is genuinely nothing to offer.
        let book_candidates = crate::docks::inspector::live_books(&self.app_ctx, &self.ids);
        if !book_candidates.is_empty() {
            fields = fields.child(books_section(
                &self.probe,
                book_candidates,
                d.books.clone(),
                self.item_id,
                stack.get(),
            ));
        }

        // Links: always, on any note.
        //
        // Not gated on the project having discoverable tags, and not on this note having
        // one. A link is the writer pointing at another page by hand; whether a scanner
        // could have found either of them is a different question entirely, and the two
        // were tangled here because the control was borrowed from the scene's Cast.
        //
        // Point of view is deliberately not here either. On a note it answers a question
        // nobody asks of a note: a point of view is a fact about a *scene*, set where the
        // writer is looking at that scene. It stays in the Inspector.
        fields = fields.child(links_section(
            &self.app_ctx,
            self.ids.work_id.get().unwrap_or_default(),
            &self.probe,
            d.references.clone(),
            self.item_id,
            stack.get(),
        ));

        // ── The right column: what is already written ────────────────────────
        //
        // Only the mention index can answer this. An item with no discoverable tag is
        // not a target the scan ever reaches, so the empty state is shown only for one
        // that *is* discoverable; an ordinary note gets no column at all rather than an
        // empty promise.
        // Both arms are a `VStack` on purpose: `Box<dyn Widget>` is not itself a
        // `Widget` here, so an optional column has to unify on a concrete type rather
        // than on a trait object.
        let mut manuscript: Option<VStack> = mention_index.as_ref().and_then(|index| {
            let backlinks = index.backlinks_for(self.item_id);
            if !backlinks.is_empty() {
                Some(backlinks_section(
                    &self.app_ctx,
                    self.ids.work_id.get().unwrap_or_default(),
                    backlinks,
                    &self.appears_in_book,
                    self.item_id,
                    self.ids.stack_id.get(),
                ))
            } else if is_discoverable {
                Some(backlinks_empty_hint())
            } else {
                None
            }
        });

        // ── Registered sections: what an extension has to say about *this* entry ──
        //
        // Under the backlinks, in the manuscript column, because a reading is about
        // what the prose already says rather than a field the writer sets — see
        // `shared::note_sections` for the whole of why this door exists and why it is
        // not `container.segments`.
        //
        // A page that had no manuscript column grows one: a section is entitled to say
        // something about an entry the scan found nothing for ("not searched yet", "no
        // hits"), which is a fact about the entry rather than an empty promise.
        if let Some(index) = mention_index.as_ref() {
            let sections = crate::tabs::shared::note_sections::registered_note_sections();
            if !sections.is_empty() {
                let cx = crate::tabs::shared::note_sections::NoteSectionContext {
                    app_ctx: self.app_ctx.clone(),
                    ids: self.ids.clone(),
                    item_id: self.item_id,
                    discoverable: is_discoverable,
                    mention_index: index.clone(),
                };
                let mut col = manuscript.unwrap_or_else(|| VStack::new().spacing(18.0));
                for spec in sections {
                    col = col
                        .child(section_header((spec.label)()))
                        .child(crate::tabs::Boxed::new((spec.view)(&cx)));
                }
                manuscript = Some(col);
            }
        }
        // A reading about an entry, not prose about one: laid out the way the Pace
        // dashboard is, filling the scroll viewport and reflowing to a single column
        // when the window cannot hold two — rather than the fixed two-up spread inside
        // a centred reading column this used to be.
        //
        // That is not only a nicer shape at every width, it is the only one that holds
        // "Appears in the manuscript" inside its own column. Each backlink row sits in
        // the focus ring's `ZStack`, and a `ZStack` reports its content's **unbounded**
        // width on purpose (see its `layout_response`), so a hundred-character chapter
        // title claimed its full natural width and painted across the fields beside it.
        // `ColumnFlow` sizes its columns itself and holds a child to one; a centred
        // `HStack` of `Expand`s could not. Measured, and pinned by
        // `crate::text_overflow`'s own test.
        let mut body = ColumnFlow::new()
            .min_column_width(MIN_DETAILS_COLUMN)
            .max_columns(2)
            .column_spacing(28.0)
            .item_spacing(12.0)
            .child(fields);
        if let Some(right) = manuscript {
            body = body.child(right);
        }

        let id = ctx.add(ScrollArea::new().child(Padding::symmetric(0.0, 24.0).child(body)));
        self.root = Some(id);
        vec![id]
    }

    fn layout_response(&self, proposal: SizeProposal, ctx: &LayoutContext) -> LayoutResponse {
        self.root
            .and_then(|id| ctx.child_size(id, proposal))
            .map(LayoutResponse::from)
            .unwrap_or_else(|| proposal.resolve(0.0, 0.0).into())
    }

    fn children(&self) -> Vec<WidgetId> {
        self.root.into_iter().collect()
    }
}

/// True when any of `tags` is one of `discoverable_ids`. Pulled out of
/// [`NoteDetailsPane::build`] purely so it has a name and a unit test of its
/// own; the gate it decides (aliases, and whether an empty backlinks list still
/// shows its "nothing yet" hint) is worth pinning down on its own.
fn item_is_discoverable(tags: &[u64], discoverable_ids: &HashSet<u64>) -> bool {
    tags.iter().any(|id| discoverable_ids.contains(id))
}

/// The name: a one-line input over `BinderItem.title`, committed on blur or
/// Enter. A name is also an identifier (the outline tree and this very tab's
/// own title show it), so it must not wait for anything to debounce. Reuses
/// [`title_field`]/[`shared::title_input`] exactly as the Book/Part/Chapter
/// heading forms do; the only reason `Item/Note` never had one of these before
/// is that its `(role, sub_role)` carries no title `Content` row for
/// [`super::prose_field`] to seed one from. `title_field` never depended on
/// that seeding, it just wraps `BinderItem.title` directly.
///
/// `on_change` is a no-op, deliberately: the dirty flag every other
/// `title_input` call site feeds it is [`OpenDoc::dirty`], and that flag is
/// unconditionally cleared by `OpenDoc::flush` whether or not this field was
/// among the reasons it was set. A field not registered on `OpenDoc` at all
/// (this one is not; `Item/Note`'s matrix entry carries no title content role,
/// so `OpenDoc::build` never seeds `open_doc.title`) has no business setting a
/// flag that `flush` would then clear without ever having flushed it. Nothing
/// is lost by that: `set_title` below writes to the backend immediately, on
/// blur or Enter, exactly like every pill and chip write on this page already
/// does.
fn name_field(field: &Rc<crate::tabs::TitleField>, stack: Signal<Option<u64>>) -> impl Widget {
    let commit_field = field.clone();
    shared::title_input(
        field,
        tr!(note_details_name_placeholder()),
        || {},
        move || {
            let _ = commit_field.flush(stack.get());
        },
    )
}

fn section_header(text: impl Into<teksilo::i18n::LocalizedString>) -> impl Widget {
    GroupHeader::new(text)
        .style(TextStyleRole::SmallBold)
        .color(TextRole::Secondary)
}

fn tags_section(value: Signal<Vec<u64>>, set: SetTags, vm: TagsViewModel) -> impl Widget {
    VStack::new()
        .spacing(6.0)
        .child(RichTip::new(
            CONCEPT_TAG,
            section_header(tr!(note_details_tags())),
        ))
        .child(TagPillField::new(value, set, vm))
}

/// Same write path as `docks::inspector::story_bible`'s own alias section,
/// `SingleBinderItem::set_aliases`, a scalar read-modify-write. Armed with the
/// discoverable table when the mention index is available, so the "+" popover
/// can name any other item already answering to the alias being typed; without
/// it the field still works, it just offers no collision hint (see
/// [`AliasPillField::collision_lookup`]).
fn aliases_section(
    probe: &SingleBinderItem,
    aliases: Vec<String>,
    item_id: u64,
    stack: Option<u64>,
    mention_index: Option<&MentionIndex>,
) -> impl Widget {
    let value = Signal::new(aliases);
    let probe = probe.clone();
    let set_aliases: SetAliases = Rc::new(move |names, _c: &mut EventContext| {
        let _ = probe.set_aliases(&names, stack);
    });
    let mut field = AliasPillField::new(value, set_aliases);
    if let Some(index) = mention_index {
        field = field.collision_lookup(index.discoverable_table(), item_id);
    }
    VStack::new()
        .spacing(6.0)
        .child(section_header(tr!(note_details_aliases())))
        .child(field)
}

/// The Book or Books this note is filed under. No "Apply to children": see the
/// module doc, `Item/Note` is always a leaf.
fn books_section(
    probe: &SingleBinderItem,
    candidates: Vec<CastCandidate>,
    book_ids: Vec<u64>,
    item_id: u64,
    stack: Option<u64>,
) -> impl Widget {
    let probe_pin = probe.clone();
    let set_book: PinReference = Rc::new(move |target, _c| {
        let mut next = probe_pin.dto().map(|x| x.books).unwrap_or_default();
        if !next.contains(&target) {
            next.push(target);
        }
        let _ = probe_pin.set_books(&next, stack);
    });
    let probe_clear = probe.clone();
    let clear_book: ClearBook = Rc::new(move |target: u64, _c: &mut EventContext| {
        let next: Vec<u64> = probe_clear
            .dto()
            .map(|x| x.books)
            .unwrap_or_default()
            .into_iter()
            .filter(|&id| id != target)
            .collect();
        let _ = probe_clear.set_books(&next, stack);
    });

    let mut col = VStack::new()
        .spacing(6.0)
        .child(section_header(tr!(note_details_books())));
    if book_ids.is_empty() {
        col = col.child(
            TextWidget::new(tr!(note_details_books_empty()))
                .style(TextStyleRole::Small)
                .color(TextRole::Secondary),
        );
    } else {
        col = col.child(book_chip_row(
            book_chips(&candidates, &book_ids),
            clear_book,
        ));
    }
    col.child(book_add_button(candidates, book_ids, item_id, set_book))
}

/// **Links** from this note to other notes.
///
/// The same `BinderItem.references` field a *scene* shows as its **Cast**, borrowed for a
/// different job, which is why neither the name nor the shape is shared with it. On a
/// scene, "who is present here" is a fact about the story that a scan can genuinely
/// suggest. On a note it is a pointer the writer makes by hand: Elizabeth's page linking
/// to Longbourn, or to her sister. There is nothing to suggest and nothing to confirm, so
/// this shows only what the writer put here.
///
/// Chips plus a "+", exactly as **Filed under** does directly above it, because it is the
/// same gesture over a different table: a set of items the writer picks by hand and can
/// take off again. A `MentionList` was wrong twice over here, offering scan suggestions
/// for something no scan produces, and a "confirm" for something already deliberate.
///
/// Candidates are the work's own notes, both `Folder/Note` and `Item/Note`, not the
/// discoverable table: a link is not about who the scanner can find, and refusing to link
/// to a plain note would be an arbitrary limit on a manual pointer.
fn links_section(
    app_ctx: &AppContext,
    work_id: u64,
    probe: &SingleBinderItem,
    references: Vec<u64>,
    item_id: u64,
    stack: Option<u64>,
) -> impl Widget {
    let probe_add = probe.clone();
    let add: PinReference = Rc::new(move |target, _c| {
        let mut next = probe_add.dto().map(|x| x.references).unwrap_or_default();
        if !next.contains(&target) {
            next.push(target);
        }
        let _ = probe_add.set_references(&next, stack);
    });
    let probe_clear = probe.clone();
    let clear: ClearBook = Rc::new(move |target: u64, _c: &mut EventContext| {
        let next: Vec<u64> = probe_clear
            .dto()
            .map(|x| x.references)
            .unwrap_or_default()
            .into_iter()
            .filter(|&id| id != target)
            .collect();
        let _ = probe_clear.set_references(&next, stack);
    });

    let candidates = note_candidates(app_ctx, work_id, item_id);

    let mut col = VStack::new()
        .spacing(6.0)
        .child(section_header(tr!(note_details_links())));
    if references.is_empty() {
        col = col.child(
            TextWidget::new(tr!(note_details_links_empty()))
                .style(TextStyleRole::Small)
                .color(TextRole::Secondary),
        );
    } else {
        col = col.child(book_chip_row(book_chips(&candidates, &references), clear));
    }
    col.child(
        PopoverButton::new(
            Button::new(tr!(note_details_links_add())).variant(ButtonVariant::Plain),
        )
        .content(CastAddPopover::new(candidates, references, item_id, add)),
    )
}

/// Every other note in the work, as link candidates, in manuscript order.
///
/// Both note shapes: a `Folder/Note` is as linkable as an `Item/Note` and a writer who
/// organised their bible into folders would otherwise find half of it unreachable. This
/// note itself is excluded, because a link from a page to itself says nothing.
fn note_candidates(app_ctx: &AppContext, work_id: u64, item_id: u64) -> Vec<CastCandidate> {
    crate::models::binder_stream::ordered_flat_items(app_ctx, work_id)
        .into_iter()
        .filter(|(_, it)| {
            it.sub_role == frontend::common::entities::BinderItemSubRole::Note && it.id != item_id
        })
        .map(|(_, it)| CastCandidate {
            id: it.id,
            title: it.title,
        })
        .collect()
}

/// One row of "Appears in the manuscript": a [`MentionRow`] paired with the
/// title of the document it was found in. See the module doc for why this is
/// not [`MentionRow::title`] itself.
#[derive(Clone, Debug, PartialEq)]
struct BacklinkRow {
    owner_id: u64,
    document_title: String,
    matched_names: Vec<String>,
    is_title_match: bool,
    hit_count: i64,
    is_confirmed: bool,
    is_point_of_view: bool,
    evidence: String,
}

/// Pair `rows` with the title of the document each was found in, falling back
/// to `untitled` for a document whose title is blank or that no longer
/// resolves at all (trashed or deleted between the scan and this read). Same
/// "named, not left blank" reasoning [`MentionList`]'s own doc gives for an
/// unresolved pin.
///
/// Pure: the backend read that builds `titles` lives in
/// [`backlinks_section`], not here, so this is testable with no `AppContext`
/// at all.
fn backlink_rows(
    rows: Vec<MentionRow>,
    titles: &HashMap<u64, String>,
    untitled: &str,
) -> Vec<BacklinkRow> {
    rows.into_iter()
        .map(|r| {
            let document_title = titles
                .get(&r.owner_id)
                .map(|t| t.trim())
                .filter(|t| !t.is_empty())
                .map(str::to_string)
                .unwrap_or_else(|| untitled.to_string());
            BacklinkRow {
                owner_id: r.owner_id,
                document_title,
                matched_names: r.matched_names,
                is_title_match: r.is_title_match,
                hit_count: r.hit_count,
                is_confirmed: r.is_confirmed,
                is_point_of_view: r.is_point_of_view,
                evidence: r.evidence,
            }
        })
        .collect()
}

/// The segment id for "in no Book at all".
///
/// A real id, not an absence: this group is a place a writer can stand, and it has to be
/// distinguishable from every Book's own id. `u64::MAX` because a Book's id is a store id
/// and will never reach it.
const OUTSIDE_BOOKS: SegmentId = SegmentId::from_u64(u64::MAX);

/// A Book's own label, numbered as the binder numbers it.
fn book_label(book: &crate::models::BookChoice) -> String {
    let (text, badge) =
        crate::models::label_and_badge(&book.title, book.fallback_label.as_deref(), book.number);
    match badge {
        Some(n) => format!("{n}. {text}"),
        None => text,
    }
}

/// Group `rows` by the Book the document each was found in belongs to, in manuscript
/// order, with whatever is in no Book at all last, under [`OUTSIDE_BOOKS`]. A group with
/// nothing in it is left out entirely, so the caller's "two or more groups, or no bar"
/// rule reads off the length.
///
/// **A Book's own row counts as being inside that Book.** [`crate::models::BookIndex`]
/// maps the rows a Book *contains* and never the Book row itself (`book_index` pushes the
/// `BookChoice` and moves straight on), so a mention found in a Book's own synopsis - and
/// `scan_mentions_uc::is_prose` counts a synopsis - would otherwise be grouped under "Not
/// in any book" as a row headed with that very Book's title, which reads as the page
/// contradicting itself about the writer's own book.
///
/// Pure: the backend read that builds the index lives in [`backlinks_section`], so this
/// is testable with no `AppContext` at all.
fn book_groups(
    rows: &[MentionRow],
    index: &crate::models::BookIndex,
) -> Vec<(SegmentId, Vec<MentionRow>)> {
    let mut groups: Vec<(SegmentId, Vec<MentionRow>)> = Vec::new();
    for book in &index.books {
        let mine: Vec<MentionRow> = rows
            .iter()
            .filter(|r| {
                r.owner_id == book.item_id || index.of_item.get(&r.owner_id) == Some(&book.item_id)
            })
            .cloned()
            .collect();
        if !mine.is_empty() {
            groups.push((SegmentId::from_u64(book.item_id), mine));
        }
    }
    let loose: Vec<MentionRow> = rows
        .iter()
        .filter(|r| {
            !index.of_item.contains_key(&r.owner_id)
                && !index.books.iter().any(|b| b.item_id == r.owner_id)
        })
        .cloned()
        .collect();
    if !loose.is_empty() {
        groups.push((OUTSIDE_BOOKS, loose));
    }
    groups
}

fn backlinks_section(
    app_ctx: &AppContext,
    work_id: u64,
    rows: Vec<MentionRow>,
    selected: &Signal<Option<SegmentId>>,
    // The entry this reading belongs to, and the undo stack its confirmations go on.
    entry: u64,
    stack: Option<u64>,
) -> VStack {
    // **Story order**, not match quality. This is a reading, not a set of candidates:
    // "she appears in chapter two, then not again until chapter nine" is a fact about the
    // shape of the book, and the index's own ordering, which puts confirmed pins and title
    // matches first, throws it away. One walk of the flat stream answers both the order
    // and the document titles, which is the same walk the Inspector's own backlink list
    // goes through so the two cannot disagree about either.
    let (rows, naming) = crate::tags::documents_in_manuscript_order(app_ctx, work_id, rows);
    let titles: HashMap<u64, String> = match naming {
        crate::tags::MentionNaming::Owner(titles) => titles,
        crate::tags::MentionNaming::Target => HashMap::new(),
    };

    // ── Which Book am I reading? ─────────────────────────────────────────────
    //
    // A bar only when there is genuinely a choice: two or more groups. One Book with
    // everything inside it, or no Book at all, is a question with a single answer, and the
    // app already refuses to draw a control for one of those (see `live_books`).
    //
    // **Rows in no Book get a group of their own**, and only when some exist. Front
    // matter, a stray note at the top, anything past a `BookEnd`: without this they would
    // be filtered out by every Book segment and become unreachable the moment a project
    // has two Books. That is the case a "one segment per Book" bar quietly loses.
    let index = crate::models::book_index(app_ctx, work_id);
    let groups: Vec<(SegmentId, LocalizedString, Vec<MentionRow>)> = book_groups(&rows, &index)
        .into_iter()
        .map(|(id, mine)| {
            let label = index
                .books
                .iter()
                .find(|b| SegmentId::from_u64(b.item_id) == id)
                .map(|b| lit!(book_label(b)))
                .unwrap_or_else(|| tr!(note_details_backlinks_outside()));
            (id, label, mine)
        })
        .collect();

    let single = groups.len() < 2;
    let shown: Vec<MentionRow> = if single {
        rows
    } else {
        let chosen = selected
            .get()
            .filter(|id| groups.iter().any(|(g, _, _)| g == id))
            .unwrap_or(groups[0].0);
        if selected.get() != Some(chosen) {
            selected.set(Some(chosen));
        }
        groups
            .iter()
            .find(|(g, _, _)| *g == chosen)
            .map(|(_, _, r)| r.clone())
            .unwrap_or_default()
    };
    let rows = shown;
    let untitled = tr!(note_details_untitled_document()).resolve_now();
    let backlinks = backlink_rows(rows, &titles, &untitled);

    // **Scoped to the Book on screen**, not to the whole reading. The bar above is what
    // makes that scope visible, and a button that silently reached past it into Books the
    // writer is not looking at would be the one control on this page whose reach is wider
    // than the list under it.
    // A declared point of view is already in the document's cast — `set_point_of_view`
    // writes both as one composite — so it is neither unconfirmed nor a row this button
    // has anything to say about.
    let unconfirmed: Vec<u64> = backlinks
        .iter()
        .filter(|r| !r.is_confirmed && !r.is_point_of_view)
        .map(|r| r.owner_id)
        .collect();

    let confirm_one: ConfirmPresence = {
        let app_ctx = app_ctx.clone();
        Rc::new(move |owner, _c| {
            let _ = crate::mentions::confirm_presence(&app_ctx, &[owner], entry, stack);
        })
    };

    let mut col = VStack::new()
        .spacing(6.0)
        .child(section_header(tr!(note_details_backlinks())));
    if !single {
        let mut bar = SegmentedControl::new(selected.clone());
        for (id, label, _) in &groups {
            bar = bar.segment(Segment::new(label.clone()).id(*id));
        }
        col = col.child(bar);
    }
    col = col.child(BacklinksList {
        rows: backlinks,
        confirm: Some(confirm_one),
        root: None,
    });
    // Offered only while there is something to confirm: with every document on this page
    // already agreed to, the button is a control that can do nothing, and hiding it is
    // how the section says "you have worked through this one".
    if !unconfirmed.is_empty() {
        let app_ctx = app_ctx.clone();
        col = col.child(
            Button::new(tr!(note_details_backlinks_confirm_all()))
                .variant(ButtonVariant::Tinted)
                .tooltip(tr!(note_details_backlinks_confirm_all_tooltip()))
                .on_activate_fn(move |_c| {
                    let _ = crate::mentions::confirm_presence(&app_ctx, &unconfirmed, entry, stack);
                }),
        );
    }
    col
}

fn backlinks_empty_hint() -> VStack {
    VStack::new()
        .spacing(6.0)
        .child(section_header(tr!(note_details_backlinks())))
        .child(
            TextWidget::new(tr!(note_details_backlinks_empty()))
                .style(TextStyleRole::Small)
                .color(TextRole::Secondary),
        )
}

/// The list itself: one row per document that mentions this note, each naming
/// the document, quoting the sentence the first hit sits in, and, when it
/// applies, how often the name was found and which alias matched.
///
/// A row that is a confirmed cast pin or a declared point of view on the
/// *mentioning* scene reads plain, not ghosted, the same "the writer's own
/// declaration, not the scanner's guess" rule [`MentionList`]'s own doc states:
/// `MentionRow::is_confirmed`/`is_point_of_view` mean exactly the same thing
/// here as they do in the cast direction, just read from the other end: "that
/// scene has *this note* pinned", not "this note has pinned *it*".
/// The narrowest a Details column may be before the pane reflows to one.
///
/// Wide enough to hold a field's label beside its value, and a backlink's title beside
/// its confirm control; below that the two-up spread stops being readable and one
/// column is the better answer. The same judgement `pace`'s dashboard makes.
const MIN_DETAILS_COLUMN: f32 = 320.0;

struct BacklinksList {
    rows: Vec<BacklinkRow>,
    /// Confirm, on a row the scan only guessed at, that this entry really does appear in
    /// that document — see [`crate::mentions::confirm_presence`]. Confirm only: an
    /// agreed row offers nothing, because taking a mention back is a statement about the
    /// document's cast and belongs where the cast is edited.
    confirm: Option<ConfirmPresence>,
    root: Option<WidgetId>,
}

impl std::fmt::Debug for BacklinksList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BacklinksList")
            .field("rows", &self.rows.len())
            .finish()
    }
}

impl Widget for BacklinksList {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        let mut col = VStack::new().spacing(12.0);

        for row in &self.rows {
            // Plain when the mentioning scene has declared a real relationship to
            // this note; ghosted when it is only the scan's guess. See this
            // struct's own doc.
            let colour = if row.is_confirmed || row.is_point_of_view {
                TextRole::Primary
            } else {
                TextRole::Secondary
            };

            let mut head = HStack::new().spacing(6.0).child(
                TextWidget::new(lit!(row.document_title.clone()))
                    .style(TextStyleRole::SmallBold)
                    .color(colour)
                    .single_line(),
            );
            // The alias that matched, when it was not the title: "Lizzy" explains
            // a row that otherwise just repeats "Elizabeth Bennet".
            // Shown unless the only thing that matched was the title, where it would
            // merely repeat the row's own headline.
            if row.matched_names.len() > 1 || (row.matched_names.len() == 1 && !row.is_title_match)
            {
                head = head.child(
                    TextWidget::new(lit!(format!("({})", row.matched_names.join(", "))))
                        .style(TextStyleRole::Tiny)
                        .color(TextRole::Secondary)
                        .single_line(),
                );
            }
            if row.hit_count > 1 {
                head = head.child(
                    TextWidget::new(lit!(row.hit_count.to_string()))
                        .style(TextStyleRole::Tiny)
                        .color(TextRole::Secondary),
                );
            }
            if !row.is_confirmed
                && !row.is_point_of_view
                && let Some(confirm) = self.confirm.clone()
            {
                let owner = row.owner_id;
                head = head.child(Expand::horizontal().child(Spacer::new())).child(
                    IconButton::new(teksilo::widgets::primitives::IconWidget::checkmark(
                        CONFIRM_GLYPH,
                    ))
                    .embedded()
                    .tooltip(tr!(mentions_confirm(name = row.document_title.clone())))
                    .on_activate_fn(move |c| confirm(owner, c)),
                );
            }

            let mut line = VStack::new().spacing(2.0).child(head);
            // Omitted for a confirmed reference or a declared point of view the
            // prose never names: there is nothing to quote, and an empty quote
            // reads as a bug rather than as "nothing found".
            if !row.evidence.is_empty() {
                line = line.child(
                    TextWidget::new(lit!(format!("\u{201c}{}\u{201d}", row.evidence)))
                        .style(TextStyleRole::Small)
                        .color(TextRole::Secondary),
                );
            }

            let owner_id = row.owner_id;
            let title = row.document_title.clone();
            // A focus stop with no visible ring is WCAG 2.4.7 — see
            // `crate::widgets::focus_ring` for why the framework paints none.
            let focused = ctx.signal(false);
            let ringed = crate::widgets::with_focus_ring(
                ctx,
                crate::widgets::RING_RADIUS_ROW,
                line,
                &focused,
            );
            let id = ctx.add(
                ringed
                    .access_role(Role::ListItem)
                    .access_label(lit!(row.document_title.clone()))
                    .focusable(true)
                    .on_focus({
                        let focused = focused.clone();
                        move |gained, _c| focused.set(gained)
                    })
                    .on_tap({
                        let title = title.clone();
                        move |_e, c: &mut EventContext| {
                            c.send_intent(AppIntent::OpenItemToSide {
                                item_id: owner_id,
                                title: title.clone(),
                            })
                        }
                    })
                    // `on_tap` never fires from the keyboard: Enter/Space opens the
                    // mentioning document, the same accessible affordance
                    // `MentionList`'s own rows offer.
                    .on_key(move |ev, c| {
                        if let WidgetEvent::KeyDown { key, .. } = ev
                            && matches!(key, Key::Enter | Key::Space)
                        {
                            c.send_intent(AppIntent::OpenItemToSide {
                                item_id: owner_id,
                                title: title.clone(),
                            });
                            return EventResponse::Handled;
                        }
                        EventResponse::Ignored
                    }),
            );
            // `Expand::horizontal` is doing real work here, not cosmetics. Each row sits
            // in the focus ring's `ZStack`, and a `ZStack` reports its content's
            // **unbounded** width by deliberate design — its `layout_response` explains
            // that taking the width from the bounded pass would truncate a shrinkable
            // label to a `MinSize`'s minimum during intrinsic measurement. A plain
            // column then places the row at the width it asked for, so a
            // hundred-character chapter title painted straight across the pane beside
            // this one. `Expand` holds the row to the width it is actually given, and
            // the elided titles inside take the difference.
            //
            // Measured, in a 200px box: `VStack(ZStack(row))` lays out at 968,
            // `VStack(Expand(ZStack(row)))` at 200. `crate::text_overflow`'s own test
            // pins both, and pins that `Shrinkable` and a bare `ColumnFlow` column do
            // not close it — a stack only distributes a deficit along its main axis, and
            // a flow's clamp does not reach through an intermediate stack.
            col = col.child(Expand::horizontal().child_id(id));
        }

        let id = ctx.add(col.access_role(Role::List));
        self.root = Some(id);
        vec![id]
    }

    fn layout_response(&self, proposal: SizeProposal, ctx: &LayoutContext) -> LayoutResponse {
        self.root
            .and_then(|id| ctx.child_size(id, proposal))
            .map(LayoutResponse::from)
            .unwrap_or_else(|| proposal.resolve(0.0, 0.0).into())
    }

    fn children(&self) -> Vec<WidgetId> {
        self.root.into_iter().collect()
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

#[cfg(test)]
mod tests {
    use super::*;

    fn row(owner: u64, target: u64, title: &str, hits: i64, evidence: &str) -> MentionRow {
        MentionRow {
            owner_id: owner,
            target_id: target,
            title: title.to_string(),
            matched_names: vec![title.to_string()],
            is_title_match: true,
            hit_count: hits,
            is_confirmed: false,
            is_point_of_view: false,
            evidence: evidence.to_string(),
        }
    }

    /// Walk the built tree, naming every widget type it holds.
    fn types(tree: &teksilo::core::widget_tree::WidgetTree, root: WidgetId) -> Vec<String> {
        let mut out = vec![tree.widget_type_name(root).unwrap_or_default().to_string()];
        for child in tree.children(root) {
            out.extend(types(tree, child));
        }
        out
    }

    fn section(rows: Vec<MentionRow>) -> Vec<String> {
        let ctx = Rc::new(frontend::AppContext::new());
        let selected: Signal<Option<SegmentId>> = Signal::new(None);
        let mut tree = teksilo::core::widget_tree::WidgetTree::new();
        let root = tree.add(backlinks_section(&ctx, 0, rows, &selected, 42, None));
        tree.layout(teksilo::prelude::SizeProposal::exact(500.0, 600.0));
        types(&tree, root)
    }

    /// **Both confirm controls are offered only while there is something to confirm** —
    /// the per-row checkmark, and the section's "Confirm every appearance".
    ///
    /// A reading the writer has already worked through would otherwise keep a button that
    /// can do nothing, and pressing it would be indistinguishable from pressing it on a
    /// reading full of suggestions, since it writes nothing either way. Their absence is
    /// how the section says the work is done.
    ///
    /// The two are counted apart — `IconButton` also ends in "Button" — because they
    /// answer different questions: one row, or the whole page.
    #[test]
    fn the_confirm_controls_appear_only_while_something_is_unconfirmed() {
        let counts = |rows: Vec<MentionRow>| {
            let built = section(rows);
            // Full paths, so `ends_with` separates the two: an `IconButton` also ends
            // in "Button", and the section's own button is the wider claim of the pair.
            let per_row = built.iter().filter(|t| t.ends_with("::IconButton")).count();
            let all = built.iter().filter(|t| t.ends_with("::Button")).count();
            (per_row, all)
        };

        let suggested = row(7, 42, "Elena", 2, "Elena crossed.");
        assert_eq!(
            counts(vec![suggested.clone()]),
            (1, 1),
            "a suggested appearance is confirmable on its own row and with the page"
        );

        let mut agreed = suggested.clone();
        agreed.is_confirmed = true;
        assert_eq!(
            counts(vec![agreed]),
            (0, 0),
            "nothing left to confirm, so neither control"
        );

        // A declared point of view is already in the document's cast — `set_point_of_view`
        // writes both as one composite — so it is not a suggestion waiting on agreement,
        // and it renders plain rather than ghosted, which a control would contradict.
        let mut pov = suggested;
        pov.is_point_of_view = true;
        assert_eq!(
            counts(vec![pov]),
            (0, 0),
            "a declared point of view is not an unconfirmed suggestion"
        );
    }

    #[test]
    fn an_item_with_no_discoverable_tags_at_all_is_not_discoverable() {
        let discoverable: HashSet<u64> = [10, 11].into_iter().collect();
        assert!(!item_is_discoverable(&[1, 2], &discoverable));
    }

    #[test]
    fn an_item_carrying_a_discoverable_tag_is_discoverable() {
        let discoverable: HashSet<u64> = [10, 11].into_iter().collect();
        assert!(item_is_discoverable(&[2, 11], &discoverable));
    }

    #[test]
    fn an_item_with_no_tags_at_all_is_not_discoverable() {
        let discoverable: HashSet<u64> = [10].into_iter().collect();
        assert!(!item_is_discoverable(&[], &discoverable));
    }

    /// The whole reason `backlink_rows` exists: the document title, not
    /// `MentionRow::title` (which would be this same note's own name on every
    /// row; see the module doc).
    #[test]
    fn a_backlink_row_names_the_owner_document_not_the_target() {
        let rows = vec![row(2, 99, "This Note's Own Name", 1, "Grace smiled.")];
        let mut titles = HashMap::new();
        titles.insert(2, "Chapter Three".to_string());
        let out = backlink_rows(rows, &titles, "Untitled");
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].document_title, "Chapter Three");
        assert_eq!(out[0].owner_id, 2);
        assert_eq!(out[0].evidence, "Grace smiled.");
    }

    /// An owner with a blank title falls back to the placeholder, not an empty
    /// headline.
    #[test]
    fn a_blank_owner_title_falls_back_to_the_placeholder() {
        let rows = vec![row(2, 99, "x", 1, "")];
        let mut titles = HashMap::new();
        titles.insert(2, "   ".to_string());
        let out = backlink_rows(rows, &titles, "Untitled");
        assert_eq!(out[0].document_title, "Untitled");
    }

    /// An owner that no longer resolves at all (trashed or deleted between the
    /// scan and this read) gets the same placeholder as a blank title, rather
    /// than a row with nothing at all to click on.
    #[test]
    fn an_unresolved_owner_falls_back_to_the_placeholder() {
        let rows = vec![row(2, 99, "x", 1, "")];
        let out = backlink_rows(rows, &HashMap::new(), "Untitled");
        assert_eq!(out[0].document_title, "Untitled");
    }

    /// Every field but the title is carried straight through, unchanged.
    #[test]
    fn every_other_field_is_carried_through_unchanged() {
        let mut r = row(2, 99, "x", 5, "Evidence here.");
        r.is_confirmed = true;
        r.is_point_of_view = true;
        r.is_title_match = false;
        r.matched_names = vec!["Lizzy".to_string()];
        let mut titles = HashMap::new();
        titles.insert(2, "Scene One".to_string());
        let out = backlink_rows(vec![r], &titles, "Untitled");
        assert_eq!(out[0].hit_count, 5);
        assert!(out[0].is_confirmed);
        assert!(out[0].is_point_of_view);
        assert!(!out[0].is_title_match);
        assert_eq!(out[0].matched_names, vec!["Lizzy".to_string()]);
    }

    /// A `BookChoice` as `book_index` would answer it: an id, a title, no number.
    fn book(item_id: u64, title: &str) -> crate::models::BookChoice {
        crate::models::BookChoice {
            item_id,
            uid: uuid::Uuid::nil(),
            title: title.to_string(),
            number: None,
            fallback_label: None,
        }
    }

    /// **A Book's own synopsis is inside that Book.** `scan_mentions_uc` counts a
    /// synopsis as prose, so a name written into Book One's own synopsis comes back as a
    /// row owned by the Book row itself - and `BookIndex::of_item` never maps a Book to
    /// itself, so grouping on that map alone filed it under "Not in any book", as a row
    /// headed "Book One". The page contradicted itself about the writer's own book.
    #[test]
    fn a_mention_in_a_books_own_synopsis_is_grouped_under_that_book() {
        let index = crate::models::BookIndex {
            books: vec![book(10, "Book One"), book(20, "Book Two")],
            of_item: [(11u64, 10u64), (21u64, 20u64)].into_iter().collect(),
        };
        let rows = vec![
            row(10, 99, "Elizabeth", 1, "Elizabeth returns to Longbourn."),
            row(11, 99, "Elizabeth", 1, "She walked out."),
        ];

        let groups = book_groups(&rows, &index);
        assert_eq!(
            groups.len(),
            1,
            "one Book has rows, so one group: {groups:?}"
        );
        assert_eq!(groups[0].0, SegmentId::from_u64(10), "Book One's own group");
        assert_eq!(groups[0].1.len(), 2, "the synopsis row and the scene row");
        assert!(
            groups.iter().all(|(id, _)| *id != OUTSIDE_BOOKS),
            "nothing here is outside the books"
        );
    }

    /// A row genuinely in no Book (front matter, anything past a `BookEnd`) still gets
    /// its own group, last: without it those rows would be unreachable behind every
    /// Book's segment.
    #[test]
    fn a_row_in_no_book_at_all_keeps_its_own_group() {
        let index = crate::models::BookIndex {
            books: vec![book(10, "Book One")],
            of_item: [(11u64, 10u64)].into_iter().collect(),
        };
        let rows = vec![
            row(11, 99, "Elizabeth", 1, "She walked out."),
            row(99, 99, "Elizabeth", 1, "A dedication."),
        ];

        let groups = book_groups(&rows, &index);
        assert_eq!(groups.len(), 2);
        assert_eq!(groups[0].0, SegmentId::from_u64(10));
        assert_eq!(groups[1].0, OUTSIDE_BOOKS, "the loose group comes last");
        assert_eq!(groups[1].1.len(), 1);
    }

    /// Two different owners resolve to two different rows, in the order given
    /// (`MentionIndex::backlinks_for` already sorts; this must not reshuffle).
    #[test]
    fn two_owners_resolve_to_two_distinct_documents() {
        let rows = vec![row(2, 99, "x", 3, ""), row(3, 99, "x", 1, "")];
        let mut titles = HashMap::new();
        titles.insert(2, "Chapter One".to_string());
        titles.insert(3, "Chapter Two".to_string());
        let out = backlink_rows(rows, &titles, "Untitled");
        assert_eq!(out[0].document_title, "Chapter One");
        assert_eq!(out[1].document_title, "Chapter Two");
    }
}
