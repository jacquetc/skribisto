// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! `OpenDocsStore` + `OpenDoc` — the app's shared holder of open editing state.
//!
//! An [`OpenDoc`] is one binder item's **live, shareable** editing state: its main
//! text + synopsis documents, its title fields and the dirty flag — everything a
//! `RichTextEditor` binds to. It is a **leaf**: it owns documents and nothing else (see
//! [`OpenDoc::build`] for why the container tabs' stream view-model deliberately does
//! *not* live here). Because a
//! [`TextDocument`](bastyde::text_document::TextDocument) is a cheap `Arc` handle,
//! two editors bound to the same `OpenDoc` share one live document.
//!
//! [`OpenDocsStore`] keeps one `Rc<OpenDoc>` per open item id, reference-counted:
//! [`open`](OpenDocsStore::open) builds-or-reuses (and refs) it, [`release`](OpenDocsStore::release)
//! unrefs and — on the last reference — flushes + evicts. Registered as
//! `app_state`, so the shared documents are reachable **outside** the editor tabs (a
//! preview pane, a manuscript stream's rows, any future consumer), not only from the
//! `TabWidget`s — which is what makes one item resolve to one live document everywhere.
//! This file is written once (no `#[cfg]` seam): the real/mock
//! difference lives below it in `SingleContent` / `SingleBinderItem`, exactly as
//! for the other Layer-A models.

use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::rc::Rc;

use bastyde::prelude::{BuildContext, Signal};
use bastyde::text_document::Color;

use frontend::AppContext;
use frontend::commands::{binder_commands, binder_item_commands, content_commands, work_commands};
use frontend::common::direct_access::binder::BinderRelationshipField;
use frontend::common::direct_access::binder_item::BinderItemRelationshipField;
use frontend::common::direct_access::work::WorkRelationshipField;
use frontend::common::entities::{BinderItem, BinderItemRole, BinderItemSubRole, ContentRole};
use frontend::common::event::{DirectAccessEntity, EntityEvent, Event, Origin};
use frontend::direct_access::ContentDto;

use crate::comments::binding::CommentBinding;
use crate::comments::session::CommentHighlightSession;
use crate::singles::SingleBinderItem;
use crate::spellcheck::{SpellSession, SpellcheckService};
use crate::tabs::{
    ProseField, ProseKind, TitleField, TitlePart, prose_field, prose_kind_for, title_field,
};
use crate::text_replacement::TextReplacementSession;
use crate::text_replacement::typography::SmartPunctuationFlags;
use crate::view_models::TextReplacementRulesViewModel;

/// One open item's live editing state, shared by every view showing that item.
pub struct OpenDoc {
    pub item_id: u64,
    pub role: BinderItemRole,
    pub sub_role: BinderItemSubRole,
    pub kind: Option<ProseKind>,
    pub title: Option<TitleField>,
    pub subtitle: Option<TitleField>,
    pub main: Option<ProseField>,
    pub synopsis: Option<ProseField>,
    /// The epigraph — the quotation set at the head of a book, part or chapter, with its
    /// attribution. Present only on the six headed combinations the matrix allows it on,
    /// so a scene or a note never has one.
    ///
    /// A third prose field rather than a structured pair of rows: the attribution is
    /// free text inside the same blockquote (CMOS asks only for a name and usually a
    /// title, never a citation), and several quotations live in *one* row as several
    /// blockquotes. That last part is forced, not stylistic — every consumer of a
    /// `Content` row finds it by `.find(|c| c.role == role)`, first match wins, so a
    /// second row of the same role would be invisible to all of them forever.
    pub epigraph: Option<ProseField>,
    /// `true` once any editor bound to this doc edited a field since the last
    /// save. Cleared by [`flush`](Self::flush).
    pub dirty: Signal<bool>,
    /// `true` while this item is in the trash (`!BinderItem.activated`). Seeded
    /// on open and flipped live by `EditorsViewModel::items_updated` when the item
    /// is trashed/restored — drives the tab's warning accent + the in-editor
    /// "this item is in the Trash" banner. Shared, so both split panes react.
    pub trashed: Signal<bool>,
    /// The item's tag ids — the editor subtitle's dot row. Lives on the doc rather than in
    /// a per-pane probe because `panes.rs` builds from `&ContentTab` with no context of its
    /// own, and `items_updated` already fetches the DTO this reads.
    pub tags: Signal<Vec<u64>>,
    /// Writer for [`tags`](Self::tags). Tags are a *relationship*, so they cannot ride the
    /// scalar DTO update the title fields use; this probe issues the relationship command.
    /// Held here because the panes compose from `&ContentTab` and have no `AppContext`.
    pub tag_probe: SingleBinderItem,
    /// The store's aggregate "an edit happened" counter — bumped by every edit,
    /// observed by the debounced autosave timer.
    edited: Signal<u64>,
    /// Per-doc edit generation — bumped only when **this** item is edited.
    /// Cast live-overlay (and anything else that must not wake on a side tab)
    /// binds here instead of [`OpenDocsStore::edited_any`].
    pub edit_gen: Signal<u64>,
    /// The caret-aware spell-check range session on the main / synopsis document, if that field
    /// exists. Created once in [`build`](Self::build); `attach_spell` sets its checker (dictionary
    /// install/remove, mute, language change) and the editor feeds it the focused view's caret.
    /// `Rc` so the editor build can hold a clone to drive it.
    spell_main: Option<Rc<SpellSession>>,
    spell_synopsis: Option<Rc<SpellSession>>,
    spell_epigraph: Option<Rc<SpellSession>>,
    /// The comment highlight layer on each prose document, if that field exists.
    ///
    /// It hangs here rather than on a tab or an editor widget for the same reason
    /// the spell sessions do: an `OpenDoc` is shared by every simultaneous view of
    /// one item (its own tab, a split pane, a stream row, a corkboard card), so a
    /// session owned by a widget would paint in whichever view happened to create
    /// it and nowhere else.
    comments_main: Option<Rc<CommentHighlightSession>>,
    comments_synopsis: Option<Rc<CommentHighlightSession>>,
    /// Where this doc's editors fetch an image they meet but do not have —
    /// see [`crate::view_models::images::ImageSource`].
    images: Option<crate::view_models::images::ImageSource>,
    /// The comment feature's view-model, installed by `App` once a project is
    /// open (mirroring `attach_spell`). `None` in the widget tests and in any
    /// build with no comment store behind it, which is what makes the whole
    /// feature degrade to "no comment affordances" rather than to a panic.
    comments_vm: RefCell<Option<crate::view_models::CommentsViewModel>>,
    /// The footnote feature's view-model, on exactly the same footing as
    /// `comments_vm` and installed the same way. `None` degrades the feature to
    /// "no footnote affordances in this document" rather than to a panic.
    footnotes: RefCell<Option<crate::view_models::FootnotesViewModel>>,
    /// The replace-while-typing state machine for each prose document, if the
    /// lexicon view-model was installed on the store. Set by
    /// [`attach_replacements`](Self::attach_replacements) on open, and living as
    /// long as the `OpenDoc` — its pending backspace-revert has to outlive a tab
    /// rebuild, which is exactly why it hangs here and not on the widget.
    replacement_main: RefCell<Option<Rc<TextReplacementSession>>>,
    replacement_synopsis: RefCell<Option<Rc<TextReplacementSession>>>,
    /// The epigraph gets one too, and it is the field that needs it most: an epigraph is
    /// almost entirely quotation marks, dashes and ellipses, which is exactly what the
    /// smart-punctuation engine exists for — and the locale-aware quote pairs matter more
    /// here than anywhere else in the manuscript.
    replacement_epigraph: RefCell<Option<Rc<TextReplacementSession>>>,
    /// How many mounted views are currently *showing* this doc's synopsis.
    ///
    /// Visibility stopped being one global answer once the synopsis could be
    /// hidden per tab (folded away under Side placement) and per window (the
    /// distraction-free toggle). One document can be on screen several times at
    /// once — both panes of a split, plus the distraction-free surface — so the
    /// question "may this spell session sleep?" is genuinely "is *nobody*
    /// showing it?", which is a count, not a flag.
    ///
    /// Maintained through [`acquire_synopsis_viewer`](Self::acquire_synopsis_viewer)
    /// and its guard, mirroring the store's own open/release refcount above.
    synopsis_viewers: Cell<u32>,
}

/// Keeps a doc's synopsis spell session awake for as long as it is held.
///
/// A guard rather than paired calls because the *unbalanced* path is the common
/// one: a tab can be closed, retyped by a Promote, or torn down with its window
/// while its synopsis is on screen, and none of those run a tidy "now hide it"
/// step. Dropping the widget that owns the guard is the one event all of them
/// share.
pub struct SynopsisViewerGuard {
    doc: Rc<OpenDoc>,
}

impl Drop for SynopsisViewerGuard {
    fn drop(&mut self) {
        self.doc.release_synopsis_viewer();
    }
}

impl OpenDoc {
    /// Build an item's editing state from its already-fetched `Content` rows,
    /// loading each allowed role into the right field. `edited` is the store's
    /// shared edit counter.
    ///
    /// An `OpenDoc` is a **leaf**: it owns documents, nothing else. The container
    /// tabs' `StreamViewModel` deliberately does *not* live here — it holds the
    /// `OpenDocsStore` (to open its rows), and the store owns this `OpenDoc`, so
    /// hanging it here would close an `Rc` cycle. Worse, `OpenDocsStore::clear()`
    /// drops its `OpenDoc`s *while* holding the map's `RefCell` borrow, so a `Drop`
    /// that released row refs would re-enter `borrow_mut()` and panic. It lives on
    /// `ContentTab` instead, which nothing in the store points back at.
    pub fn build(
        ctx: &Rc<AppContext>,
        item_id: u64,
        role: &BinderItemRole,
        sub_role: &BinderItemSubRole,
        contents: &[ContentDto],
        edited: Signal<u64>,
        media_dir: &std::path::Path,
    ) -> Self {
        let mut doc = OpenDoc {
            item_id,
            role: role.clone(),
            sub_role: sub_role.clone(),
            kind: prose_kind_for(role, sub_role),
            title: None,
            subtitle: None,
            main: None,
            synopsis: None,
            epigraph: None,
            dirty: Signal::new(false),
            trashed: Signal::new(false),
            tags: Signal::new(Vec::new()),
            tag_probe: {
                let p = SingleBinderItem::new(ctx.clone());
                p.set_id(Some(item_id));
                p
            },
            edited,
            edit_gen: Signal::new(0),
            spell_main: None,
            spell_synopsis: None,
            spell_epigraph: None,
            comments_main: None,
            comments_synopsis: None,
            images: None,
            comments_vm: RefCell::new(None),
            footnotes: RefCell::new(None),
            replacement_main: RefCell::new(None),
            replacement_synopsis: RefCell::new(None),
            replacement_epigraph: RefCell::new(None),
            synopsis_viewers: Cell::new(0),
        };
        for cr in skribisto_model::allowed_content(role, sub_role) {
            let existing = contents.iter().find(|c| &c.role == cr);
            match cr {
                ContentRole::SynopsisText => {
                    doc.synopsis = Some(prose_field(ctx, item_id, cr.clone(), existing))
                }
                ContentRole::SceneText | ContentRole::NoteText => {
                    doc.main = Some(prose_field(ctx, item_id, cr.clone(), existing))
                }
                ContentRole::EpigraphText => {
                    doc.epigraph = Some(prose_field(ctx, item_id, cr.clone(), existing))
                }
                // A paratext's prose is the page's whole content, so it takes the main
                // slot — the same surface a scene writes into, with the same sessions.
                ContentRole::ParatextText => {
                    doc.main = Some(prose_field(ctx, item_id, cr.clone(), existing))
                }
                // The two *names*. They are edited as the item's title/subtitle (what the
                // outline tree and the tab show) and mirrored into these content rows on
                // save — see `TitleField`.
                ContentRole::BookSubtitle => {
                    doc.subtitle = Some(title_field(ctx, item_id, TitlePart::SubTitle))
                }
                ContentRole::BookTitle | ContentRole::ChapterTitle | ContentRole::PartTitle => {
                    doc.title = Some(title_field(ctx, item_id, TitlePart::Title))
                }
            }
        }
        // Give each present prose document its caret-aware spell-check range session. Empty until
        // `attach_spell` sets a checker; it lives as long as the `OpenDoc` (its `Drop` retires the
        // highlight layer).
        doc.spell_main = doc.main.as_ref().map(|f| SpellSession::new(&f.doc));
        doc.spell_synopsis = doc.synopsis.as_ref().map(|f| SpellSession::new(&f.doc));
        doc.spell_epigraph = doc.epigraph.as_ref().map(|f| SpellSession::new(&f.doc));
        // Empty until the comments view-model seeds it with a re-anchor pass; the
        // layer itself lives as long as the `OpenDoc` (its `Drop` retires it).
        // No comment layer on the epigraph, deliberately. A comment anchors to a quote of
        // the *manuscript* — the two docks are scoped to the streams — and an epigraph is
        // quoted matter that is not the author's own text to annotate in place.
        doc.comments_main = doc
            .main
            .as_ref()
            .map(|f| CommentHighlightSession::new(&f.doc));
        doc.comments_synopsis = doc
            .synopsis
            .as_ref()
            .map(|f| CommentHighlightSession::new(&f.doc));
        // Resolve every image the prose names, before the first paint. After a
        // reload the anchors exist but the document's resource table is empty,
        // so without this a reopened project shows correctly-sized blanks.
        for field in [
            doc.main.as_ref(),
            doc.synopsis.as_ref(),
            doc.epigraph.as_ref(),
        ]
        .into_iter()
        .flatten()
        {
            let djot = field.doc.to_djot().unwrap_or_default();
            crate::view_models::images::register_referenced(&field.doc, &djot, media_dir);
        }
        // …and stand by for the ones that arrive later. An image pasted in from
        // another editor brings its reference and not its pixels, so the editor
        // asks for them the first time it tries to paint one it does not know.
        doc.images = Some(crate::view_models::images::ImageSource::new(
            media_dir.to_path_buf(),
        ));

        doc
    }

    /// The `on_change` hook for this doc's editors: mark it dirty and bump the
    /// store's aggregate edit counter (drives autosave) plus this doc's
    /// [`edit_gen`](Self::edit_gen) (per-item observers such as cast live-overlay).
    pub fn mark_dirty_fn(&self) -> impl Fn() + 'static {
        let dirty = self.dirty.clone();
        let edited = self.edited.clone();
        let edit_gen = self.edit_gen.clone();
        move || {
            dirty.set(true);
            edited.set(edited.get().wrapping_add(1));
            edit_gen.set(edit_gen.get().wrapping_add(1));
        }
    }

    /// Persist every changed field back to its `Content` row (creating the row on
    /// first save) via each field's `SingleContent`. Idempotent (clean fields are
    /// a no-op). Routes through the undo `stack`.
    ///
    /// Each prose field records the `content_revision` it flushed at as it goes
    /// (see [`ProseField::flush`]) — [`Self::is_stale`] compares against it for
    /// an exact per-document staleness check, sharper than the `dirty` flag this
    /// method clears unconditionally below.
    pub fn flush(&self, stack: Option<u64>) -> anyhow::Result<()> {
        if let Some(f) = &self.title {
            f.flush(stack)?;
        }
        if let Some(f) = &self.subtitle {
            f.flush(stack)?;
        }
        if let Some(f) = &self.main {
            f.flush(stack)?;
        }
        if let Some(f) = &self.synopsis {
            f.flush(stack)?;
        }
        if let Some(f) = &self.epigraph {
            f.flush(stack)?;
        }
        self.dirty.set(false);
        Ok(())
    }

    /// Exact "has any prose field changed since it was last flushed" check,
    /// across whichever of `main`/`synopsis` this doc has — see
    /// [`ProseField::is_stale`]'s doc for why this is more precise than the
    /// aggregate [`dirty`](Self::dirty) flag above (which `title`/`subtitle`
    /// still rely on; their own edit detection — diffing the live signal
    /// against the value loaded at open, see `TitleField::edited_probe` — has
    /// no such imprecision to begin with, since it holds no intermediate flag
    /// to go stale).
    ///
    /// No caller yet — folded in as a primitive per the migration design doc
    /// (§3), not wired into autosave/save-indicator behaviour this phase (that
    /// would be a visible behaviour change, out of scope here). See
    /// `ProseField`'s own test for the property this proves.
    #[allow(dead_code)]
    pub fn is_stale(&self) -> bool {
        self.main.as_ref().is_some_and(ProseField::is_stale)
            || self.synopsis.as_ref().is_some_and(ProseField::is_stale)
            || self.epigraph.as_ref().is_some_and(ProseField::is_stale)
    }

    /// Discard the live edits and re-read every present field from its persisted
    /// `Content` row — for a doc another use case rewrote out from under us (a merge
    /// absorbing a neighbour, a split cutting the source in two). Both writing roles
    /// are reloaded, not just the prose: a merge concatenates the synopses too.
    ///
    /// The caller flushes first, so nothing unsaved is lost; it must also pump a
    /// frame afterwards, since `set_djot` only queues a document event.
    pub fn reload(&self) {
        if let Some(f) = &self.title {
            f.reload();
        }
        if let Some(f) = &self.subtitle {
            f.reload();
        }
        if let Some(f) = &self.main {
            f.reload();
        }
        if let Some(f) = &self.synopsis {
            f.reload();
        }
        if let Some(f) = &self.epigraph {
            f.reload();
        }
        self.dirty.set(false);
    }

    /// Point this doc's spell sessions at the effective language list `tags`, in the squiggle
    /// `color`. Builds one [`SpellChecker`](crate::spellcheck) and hands it to each present prose
    /// session, which recomputes immediately; `None` (nothing installed/active) clears the
    /// squiggles — the degrade path. Only the two prose fields carry a session; the title/subtitle
    /// are plain `Signal<String>`.
    pub fn attach_spell(
        &self,
        spell: &SpellcheckService,
        tags: &[String],
        color: Color,
        work_id: Option<u64>,
    ) {
        let checker = spell.build_checker(tags, work_id);
        if let Some(s) = &self.spell_main {
            s.set_checker(checker.clone(), color);
        }
        if let Some(s) = &self.spell_synopsis {
            s.set_checker(checker.clone(), color);
        }
        if let Some(s) = &self.spell_epigraph {
            s.set_checker(checker, color);
        }
    }

    /// The caret-aware spell session on the main prose document, if any — the editor build feeds it
    /// the focused view's caret. `None` for a doc with no main prose (a pure container).
    pub fn spell_main(&self) -> Option<Rc<SpellSession>> {
        self.spell_main.clone()
    }

    /// The caret-aware spell session on the synopsis document, if any.
    pub fn spell_synopsis(&self) -> Option<Rc<SpellSession>> {
        self.spell_synopsis.clone()
    }

    /// The comment highlight layer on the main prose document, if any.
    pub fn comments_main(&self) -> Option<Rc<CommentHighlightSession>> {
        self.comments_main.clone()
    }

    /// The comment highlight layer on the synopsis document, if any.
    /// Where this doc's editors fetch an image they meet but do not have.
    pub fn images(&self) -> Option<crate::view_models::images::ImageSource> {
        self.images.clone()
    }

    pub fn comments_synopsis(&self) -> Option<Rc<CommentHighlightSession>> {
        self.comments_synopsis.clone()
    }

    /// Install the comment view-model and seed both documents' highlight layers.
    ///
    /// Mirrors [`attach_spell`](Self::attach_spell): called once per open, so an
    /// item reopened after an external edit re-anchors from its stored quotes
    /// rather than trusting offsets that may have rotted.
    pub fn attach_comments(&self, vm: crate::view_models::CommentsViewModel) {
        *self.comments_vm.borrow_mut() = Some(vm);
        if let Some(b) = self.comment_binding_main() {
            b.push_live();
        }
        if let Some(b) = self.comment_binding_synopsis() {
            b.push_live();
        }
    }

    /// This item's main prose editor's door to the comment feature, if the
    /// document, its highlight layer, its `Content` row and the view-model are all
    /// present. `None` collapses every comment affordance in that editor, which is
    /// the correct degrade for a container with no prose or a test with no app.
    pub fn comment_binding_main(&self) -> Option<CommentBinding> {
        Some(CommentBinding::new(
            self.comments_vm.borrow().clone()?,
            self.main.as_ref()?.doc.clone(),
            self.comments_main.clone()?,
            self.main_content_id()?,
        ))
    }

    /// The synopsis editor's binding — a *different* `Content` row than the body's,
    /// so the two never merge.
    pub fn comment_binding_synopsis(&self) -> Option<CommentBinding> {
        Some(CommentBinding::new(
            self.comments_vm.borrow().clone()?,
            self.synopsis.as_ref()?.doc.clone(),
            self.comments_synopsis.clone()?,
            self.synopsis_content_id()?,
        ))
    }

    /// This doc's main-prose footnote door, if the store has a view-model wired.
    ///
    /// Minted here, beside `comment_binding_main`, for the same reason: only the
    /// `OpenDoc` knows which `Content` row each of its documents came from, and an
    /// editor handed the bare view-model could act on the wrong one.
    pub fn footnote_binding_main(&self) -> Option<crate::view_models::FootnoteBinding> {
        Some(
            self.footnotes
                .borrow()
                .clone()?
                .binding(self.main_content_id()?),
        )
    }

    /// The same for the synopsis document.
    pub fn footnote_binding_synopsis(&self) -> Option<crate::view_models::FootnoteBinding> {
        Some(
            self.footnotes
                .borrow()
                .clone()?
                .binding(self.synopsis_content_id()?),
        )
    }

    /// Install the footnotes view-model on this doc (on open, and on the
    /// back-fill when `App` wires one after documents are already open).
    pub fn attach_footnotes(&self, vm: crate::view_models::FootnotesViewModel) {
        *self.footnotes.borrow_mut() = Some(vm);
    }

    /// Tell each of this doc's prose documents what a footnote marker prints.
    ///
    /// Every field, not just the main prose: a reference can sit in a synopsis or
    /// an epigraph as readily as in a scene, and a document left without the map
    /// falls back to drawing the raw label.
    pub fn set_footnote_markers(&self, markers: &std::collections::HashMap<String, String>) {
        for field in [&self.main, &self.synopsis, &self.epigraph]
            .into_iter()
            .flatten()
        {
            field.doc.set_footnote_markers(markers.clone());
        }
    }

    /// The `Content` row id behind the main prose document, if any — what a
    /// comment created in that editor anchors to.
    pub fn main_content_id(&self) -> Option<u64> {
        self.main.as_ref().and_then(|f| f.content_id())
    }

    /// The `Content` row id behind the synopsis document, if any.
    ///
    /// Separate from [`main_content_id`](Self::main_content_id) because a
    /// `BinderItem` owns several `Content` rows — up to four, on a chapter carrying a
    /// title, an epigraph, its prose and a synopsis — and a comment on the synopsis is a
    /// comment on a *different row* than one on the body; anchoring both to "the item"
    /// would silently merge them.
    pub fn synopsis_content_id(&self) -> Option<u64> {
        self.synopsis.as_ref().and_then(|f| f.content_id())
    }

    /// Register a mounted view as *showing* this doc's synopsis, waking its spell
    /// session if it was the first. Release by dropping the returned guard.
    ///
    /// Only the 0→1 edge does any work: a synopsis shown in two panes at once is
    /// already awake, and waking it twice would buy a second catch-up rebuild of
    /// the same text.
    pub fn acquire_synopsis_viewer(self: &Rc<Self>) -> SynopsisViewerGuard {
        let next = self.synopsis_viewers.get() + 1;
        self.synopsis_viewers.set(next);
        if next == 1
            && let Some(s) = &self.spell_synopsis
        {
            s.set_active(true);
        }
        SynopsisViewerGuard { doc: self.clone() }
    }

    /// The 1→0 edge: the last view showing this synopsis has gone, so the session
    /// stops paying for re-attaches it cannot display. Private — reached only by
    /// dropping a [`SynopsisViewerGuard`], so the count cannot be driven negative
    /// or left unbalanced by an early return.
    fn release_synopsis_viewer(&self) {
        let next = self.synopsis_viewers.get().saturating_sub(1);
        self.synopsis_viewers.set(next);
        if next == 0
            && let Some(s) = &self.spell_synopsis
        {
            s.set_active(false);
        }
    }

    /// How many mounted views currently show this doc's synopsis.
    #[cfg(test)]
    pub fn synopsis_viewers(&self) -> u32 {
        self.synopsis_viewers.get()
    }

    /// Give each present prose document its replace-while-typing session.
    ///
    /// Idempotent: a doc already carrying sessions keeps them, so re-opening an
    /// already-open item cannot discard a pending backspace-revert mid-keystroke.
    pub fn attach_replacements(&self, vm: &TextReplacementRulesViewModel) {
        if self.main.is_some() && self.replacement_main.borrow().is_none() {
            *self.replacement_main.borrow_mut() = Some(TextReplacementSession::new(vm.clone()));
        }
        if self.synopsis.is_some() && self.replacement_synopsis.borrow().is_none() {
            *self.replacement_synopsis.borrow_mut() = Some(TextReplacementSession::new(vm.clone()));
        }
        if self.epigraph.is_some() && self.replacement_epigraph.borrow().is_none() {
            *self.replacement_epigraph.borrow_mut() = Some(TextReplacementSession::new(vm.clone()));
        }
    }

    /// Tell both replace-while-typing sessions what language this document is
    /// written in. Idempotent; a real change makes the next keystroke recompile.
    ///
    /// Separate from [`attach_replacements`](Self::attach_replacements), which
    /// only *creates* the sessions and is deliberately a no-op once they exist:
    /// the language of an item can change long after it was opened, and that has
    /// to reach the engine — a Turkish trigger folds differently from an English
    /// one, so a stale locale means the rule silently stops matching.
    pub fn set_replacement_locale(&self, languages: &[String]) {
        let tag = skribisto_model::language::primary(languages);
        for session in [
            &self.replacement_main,
            &self.replacement_synopsis,
            &self.replacement_epigraph,
        ] {
            if let Some(s) = session.borrow().as_ref() {
                s.set_locale(tag);
            }
        }
    }

    /// Tell both sessions which punctuation rules this project wants.
    ///
    /// Pushed for the same reason the locale is: the row can change while the
    /// document is open (the settings pane is right there), and a stale flag set
    /// means the writer flips a switch and nothing happens until they reopen the
    /// scene.
    ///
    /// `None` means "not resolved yet" and substitutes nothing — see
    /// [`TextReplacementSession::set_punctuation`].
    pub fn set_punctuation(&self, flags: Option<SmartPunctuationFlags>) {
        for session in [
            &self.replacement_main,
            &self.replacement_synopsis,
            &self.replacement_epigraph,
        ] {
            if let Some(s) = session.borrow().as_ref() {
                s.set_punctuation(flags.clone());
            }
        }
    }

    /// The replace-while-typing session on the main prose document, if any.
    pub fn replacement_main(&self) -> Option<Rc<TextReplacementSession>> {
        self.replacement_main.borrow().clone()
    }

    /// The replace-while-typing session on the synopsis document, if any.
    pub fn replacement_synopsis(&self) -> Option<Rc<TextReplacementSession>> {
        self.replacement_synopsis.borrow().clone()
    }

    /// The replace-while-typing session on the epigraph document, if any.
    pub fn replacement_epigraph(&self) -> Option<Rc<TextReplacementSession>> {
        self.replacement_epigraph.borrow().clone()
    }

    /// The caret-aware spell session on the epigraph document, if any.
    pub fn spell_epigraph(&self) -> Option<Rc<SpellSession>> {
        self.spell_epigraph.clone()
    }
}

struct Entry {
    doc: Rc<OpenDoc>,
    refs: usize,
}

/// Every open item's resolved language tags, valid for one structural
/// fingerprint — recomputed whenever that fingerprint moves.
type LangCache = (LangFingerprint, HashMap<u64, Vec<String>>);

struct Inner {
    open: RefCell<HashMap<u64, Entry>>,
    app_ctx: Rc<AppContext>,
    /// Reactive read handle re-pointed at an item to fetch its `(role, sub_role)`.
    item_probe: SingleBinderItem,
    /// Aggregate "an edit happened" counter shared by every open doc.
    edited: Signal<u64>,
    /// The spell-check engine, set once by `App`. `None` until then (headless tests, or before
    /// the first project loads) — every attach is then a no-op, so opening still works.
    spell: RefCell<Option<SpellcheckService>>,
    /// The custom replacement lexicon, set once by `App` on the same footing as
    /// `spell`. `None` until then, and then every attach is a no-op — a headless
    /// test opens documents that simply never expand anything.
    text_replacements: RefCell<Option<TextReplacementRulesViewModel>>,
    /// The comments view-model, installed once per window and handed to every
    /// document as it opens (mirroring `text_replacements`).
    comments: RefCell<Option<crate::view_models::CommentsViewModel>>,
    /// The footnotes view-model, installed once per window and handed to every
    /// document as it opens (mirroring `comments`).
    footnotes: RefCell<Option<crate::view_models::FootnotesViewModel>>,
    /// What each footnote label's marker prints, project-wide.
    ///
    /// Held here rather than resolved per document because the number is a fact
    /// about the **manuscript**: a scene citing a note first introduced two
    /// chapters earlier must draw that note's number, which nothing inside the
    /// scene's own document knows. Remembered so a document opened later is seeded
    /// on the way in — without that a tab opened mid-session would print raw
    /// labels (`fn7`) into the writer's prose while the tabs opened at load time
    /// showed proper numbers.
    footnote_markers: RefCell<std::collections::HashMap<String, String>>,
    /// The squiggle colour, resolved from a theme role by `App` (updated on theme change).
    squiggle: Cell<Color>,
    /// The open project, for resolving each item's effective language (its own tag, else
    /// the Work's). Set by `App` on `LoadWork`/`NewWork`.
    work_id: Cell<Option<u64>>,
    work_lang: RefCell<Vec<String>>,
    /// Where this project's image bytes live. Tier 2, beside `work_id`: two
    /// Works open at once have different media directories, and a document
    /// loaded against the wrong one shows blanks.
    media_dir: RefCell<std::path::PathBuf>,
    /// The open project's punctuation rules, pushed down to every session on
    /// change and to each newly-opened document. `None` until resolved.
    punctuation: RefCell<Option<SmartPunctuationFlags>>,
    /// The memoised [`language_map`](OpenDocsStore::with_language_map), with the binder
    /// [fingerprint](LangFingerprint) it was built from.
    ///
    /// The uncached call fetched and cloned every `BinderItem` in the project to build
    /// the whole map at once. `open()` attaches spell-check to each
    /// freshly-built doc, so a container stream opening one document per row paid that
    /// whole-project walk once per row — O(rows × items). Cached, the walk happens once
    /// per structural change instead.
    lang_cache: RefCell<Option<LangCache>>,
}

/// What the cached language map is keyed on: the open work, its default language, and
/// every binder item id.
///
/// Creates and removals change the id sequence, so both drop the cache. Order is folded in
/// too, which is now *conservative* rather than load-bearing: an item's language no longer
/// depends on its neighbours (`skribisto_model::language::tags_in_binder` resolves an item's
/// own tag, else the Work's), so a pure reorder cannot change any answer. It stays in the
/// hash because a reorder is rare and cheap to over-invalidate, and because dropping it
/// would buy a subtle dependency on the resolver never regaining a positional rule.
///
/// Deliberately *not* covered: an item's own `dict_language` changing in place, which leaves
/// the id sequence identical. Detecting that here would mean re-reading every item — exactly
/// the cost this exists to avoid — so it is caught from the other side instead, by
/// [`OpenDocsStore::wire`]'s subscription to the `BinderItem::Updated` event. Shape is
/// fingerprinted; in-place fields are pushed. Between them the cache has no blind spot —
/// including "Apply to children", which writes `dict_language` on every descendant through
/// that same event.
type LangFingerprint = u64;

/// Fingerprint the inputs the language map is derived from that are cheap to read:
/// the work, its default language, and the ordered item ids of every binder.
fn fingerprint_of(
    work_id: Option<u64>,
    work_lang: &[String],
    shape: &[Vec<u64>],
) -> LangFingerprint {
    let mut hasher = DefaultHasher::new();
    work_id.hash(&mut hasher);
    work_lang.hash(&mut hasher);
    shape.hash(&mut hasher);
    hasher.finish()
}

/// The app-wide store of open documents (cheap `Rc` handle, shared by clone).
#[derive(Clone)]
pub struct OpenDocsStore {
    inner: Rc<Inner>,
}

impl OpenDocsStore {
    /// Point this store at the open project's media directory.
    ///
    /// Set when a Work opens, before any document is built: an image's bytes are
    /// resolved as its document loads, so a document created before this is
    /// known shows every picture as a correctly-sized blank.
    ///
    /// Tier 2, beside `work_id` — two Works open at once have different media
    /// directories, and resolving against the wrong one finds nothing.
    pub fn set_media_dir(&self, dir: std::path::PathBuf) {
        *self.inner.media_dir.borrow_mut() = dir;
    }

    /// The open project's media directory.
    pub fn media_dir(&self) -> std::path::PathBuf {
        self.inner.media_dir.borrow().clone()
    }

    pub fn new(app_ctx: Rc<AppContext>) -> Self {
        Self {
            inner: Rc::new(Inner {
                open: RefCell::new(HashMap::new()),
                item_probe: SingleBinderItem::new(app_ctx.clone()),
                app_ctx,
                edited: Signal::new(0),
                spell: RefCell::new(None),
                text_replacements: RefCell::new(None),
                comments: RefCell::new(None),
                footnotes: RefCell::new(None),
                footnote_markers: RefCell::new(std::collections::HashMap::new()),
                // A sensible default until `App` resolves the theme's error role.
                squiggle: Cell::new(Color::rgb(202, 66, 60)),
                work_id: Cell::new(None),
                work_lang: RefCell::new(Vec::new()),
                media_dir: RefCell::new(std::path::PathBuf::new()),
                punctuation: RefCell::new(None),
                lang_cache: RefCell::new(None),
            }),
        }
    }

    /// Install the spell-check engine (once, from `App`). Until this is set, spell attaches are
    /// no-ops and the app behaves exactly as before spell-check existed.
    pub fn set_spellcheck(&self, spell: SpellcheckService) {
        *self.inner.spell.borrow_mut() = Some(spell);
    }

    /// Install the comments view-model and seed every already-open document.
    ///
    /// Seeding the open ones matters: the store is populated before `App` finishes
    /// wiring, so a document opened by workspace restore would otherwise show no
    /// comment highlights until it was closed and reopened.
    pub fn set_comments(&self, vm: crate::view_models::CommentsViewModel) {
        *self.inner.comments.borrow_mut() = Some(vm.clone());
        let docs: Vec<Rc<OpenDoc>> = self
            .inner
            .open
            .borrow()
            .values()
            .map(|e| e.doc.clone())
            .collect();
        for doc in docs {
            doc.attach_comments(vm.clone());
        }
    }

    /// Install the footnotes view-model and seed every already-open document.
    ///
    /// The back-fill matters for the same reason `set_comments`' does: a project
    /// restoring its remembered tabs has documents open before `App` finishes
    /// wiring, and one built before this would carry no footnote door at all —
    /// no insertion, no navigation — until it was closed and reopened.
    pub fn set_footnotes(&self, vm: crate::view_models::FootnotesViewModel) {
        *self.inner.footnotes.borrow_mut() = Some(vm.clone());
        let docs: Vec<Rc<OpenDoc>> = self
            .inner
            .open
            .borrow()
            .values()
            .map(|e| e.doc.clone())
            .collect();
        for doc in docs {
            doc.attach_footnotes(vm.clone());
        }
    }

    /// The installed footnotes view-model, if `App` has wired one.
    ///
    /// The project-wide handle, reachable **without an open document** — which is
    /// what the insert command needs: it has to tell "no project" apart from "no
    /// caret", and resolving through some document's binding collapses the two.
    pub fn footnotes(&self) -> Option<crate::view_models::FootnotesViewModel> {
        self.inner.footnotes.borrow().clone()
    }

    /// Tell every open document what each footnote label's marker prints, and
    /// remember it for the documents opened next.
    ///
    /// Presentation only and never serialised: the number lives nowhere on disk
    /// because it is derived from where the reference sits, and a stored number
    /// would be wrong the first time a chapter moved.
    pub fn set_footnote_markers(&self, markers: std::collections::HashMap<String, String>) {
        *self.inner.footnote_markers.borrow_mut() = markers.clone();
        let docs: Vec<Rc<OpenDoc>> = self
            .inner
            .open
            .borrow()
            .values()
            .map(|e| e.doc.clone())
            .collect();
        for doc in docs {
            doc.set_footnote_markers(&markers);
        }
    }

    /// Every item id with a document currently open.
    ///
    /// For the footnotes model's live-edit gate, which asks each open document
    /// which notes it references before deciding whether the whole manuscript
    /// needs renumbering.
    pub fn open_item_ids(&self) -> Vec<u64> {
        self.inner.open.borrow().keys().copied().collect()
    }

    /// The installed comments view-model, if `App` has wired one.
    ///
    /// The project-wide handle, reachable **without an open document**. Callers that
    /// need to watch the comment store for a page that currently has no commentable
    /// surface — an empty container's stream, before any row exists — must come
    /// through here: resolving the view-model via some row's binding answers `None`
    /// on exactly the pages that need it most, and a caller that gives up at that
    /// point never subscribes at all.
    pub fn comments(&self) -> Option<crate::view_models::CommentsViewModel> {
        self.inner.comments.borrow().clone()
    }

    /// Install the custom replacement lexicon (once, from `App`), and give it to
    /// every document already open.
    ///
    /// The back-fill matters: `App::build` installs this after the store exists,
    /// and a project restoring its remembered tabs can have opened documents by
    /// then. Without it those tabs would silently never expand anything until
    /// they were closed and reopened.
    pub fn set_text_replacements(&self, vm: TextReplacementRulesViewModel) {
        *self.inner.text_replacements.borrow_mut() = Some(vm.clone());
        let docs: Vec<Rc<OpenDoc>> = self
            .inner
            .open
            .borrow()
            .values()
            .map(|e| e.doc.clone())
            .collect();
        for doc in docs {
            doc.attach_replacements(&vm);
        }
    }

    /// Subscribe the cached language map to the edits its
    /// [fingerprint](LangFingerprint) cannot see.
    ///
    /// The fingerprint covers the binder's *shape*, so a create, a removal or a move
    /// invalidates the map on its own. An item's own `dict_language` or `sub_role`
    /// changing in place leaves that shape byte-for-byte identical, and re-reading every
    /// item to notice is the very cost the cache exists to avoid — so the entity event
    /// pushes it instead. `BinderItem::Updated` covers both: the Inspector's language
    /// pill writes `dict_language` through it, and a Promote rewrites `sub_role` through
    /// it.
    ///
    /// Call from `build`, on **every** build. `BuildContext::subscribe_event` scopes a
    /// subscription to the widget's current build and drops it on the next, so a
    /// "subscribe once" guard would make the store go deaf the first time `App` rebuilt;
    /// re-subscribing cannot duplicate, because the old callback is already gone.
    ///
    /// The closure may hold the store by value — unlike the stream view-model's, this
    /// subscription is owned by the widget tree, not by the thing it captures, so there
    /// is no `Rc` cycle to close.
    pub fn wire(&self, ctx: &mut BuildContext) {
        let me = self.clone();
        ctx.subscribe_event(
            Origin::DirectAccess(DirectAccessEntity::BinderItem(EntityEvent::Updated)),
            move |_event: &Event| me.invalidate_language_cache(),
        );
    }

    /// Set the squiggle colour (a theme error role, resolved by `App`). Re-attaches only when
    /// the colour actually changed (a theme switch) — `App` calls this on every rebuild, so an
    /// unconditional re-attach would be needless churn.
    pub fn set_squiggle_color(&self, color: Color) {
        if self.inner.squiggle.get() != color {
            self.inner.squiggle.set(color);
            self.attach_all();
        }
    }

    /// Show or hide anchored comments across every open document — Tools ▸ Comments.
    ///
    /// Writes the view-model's own flag first (the margin binds it and rebuilds), then
    /// walks the open documents' highlight layers so the marks in the prose go with it.
    /// One door for both halves: two callers setting them separately is how a margin
    /// with no cards ends up beside prose that is still washed ochre.
    ///
    /// The layers are deactivated, never dropped — they keep folding edits into their
    /// anchors while hidden, so showing again does not resurrect stale offsets.
    pub fn set_comments_visible(&self, visible: bool) {
        if let Some(vm) = self.inner.comments.borrow().clone() {
            vm.set_visible(visible);
        }
        for entry in self.inner.open.borrow().values() {
            if let Some(s) = entry.doc.comments_main() {
                s.set_active(visible);
            }
            if let Some(s) = entry.doc.comments_synopsis() {
                s.set_active(visible);
            }
        }
    }

    // There is no `set_synopsis_visible` here any more. It used to mirror one
    // global "is the synopsis pane shown" setting into every open doc's spell
    // session; that stopped being the right question once the synopsis could
    // also be folded away per tab (Side placement) and toggled per window
    // (distraction-free). "May this session sleep?" is now answered by counting
    // the views that actually show it — see `OpenDoc::acquire_synopsis_viewer`,
    // held by the mounted pane itself.
    /// Point the store at the open project's default language, for the effective-language
    /// resolution. Called on `LoadWork`/`NewWork`; does not itself re-attach (the caller pairs
    /// it with [`attach_all`](Self::attach_all) once personal words are also loaded).
    pub fn set_project_language(&self, work_id: Option<u64>, work_lang: Vec<String>) {
        self.inner.work_id.set(work_id);
        *self.inner.work_lang.borrow_mut() = work_lang;
        // A different project (or default language) resolves every item differently.
        self.invalidate_language_cache();
    }

    /// This store's own open Work, if any — the `work_id` `set_project_language` last set.
    /// Exposed so a caller holding only an `OpenDocsStore` (e.g. [`LanguagePillField`](crate::spellcheck::language_pill_field::LanguagePillField))
    /// can scope its own `SpellcheckService` calls (`is_muted`/`set_muted`/`build_checker`) to
    /// *this* window's Work rather than guessing or reaching for a process-wide fallback.
    pub fn work_id(&self) -> Option<u64> {
        self.inner.work_id.get()
    }

    /// Set the open project's punctuation rules and push them to every open
    /// document at once.
    ///
    /// Called from an effect over the project's `SmartPunctuation` row, so a
    /// switch flipped in Settings reaches the scene the writer is looking at
    /// without reopening it. Idempotent — each session compares before
    /// recompiling.
    pub fn set_punctuation(&self, flags: Option<SmartPunctuationFlags>) {
        *self.inner.punctuation.borrow_mut() = flags.clone();
        let docs: Vec<Rc<OpenDoc>> = self
            .inner
            .open
            .borrow()
            .values()
            .map(|e| e.doc.clone())
            .collect();
        // Collected first, then pushed outside the borrow: a session recompile
        // must not run while the map is borrowed.
        for doc in docs {
            doc.set_punctuation(flags.clone());
        }
    }

    /// Re-attach the spell-checker to **every** open document — the single path for install,
    /// remove, mute, language-change, focus-regain, and theme change. Recomputes each item's
    /// effective language through the same resolver search uses.
    pub fn attach_all(&self) {
        let color = self.inner.squiggle.get();
        // Belt and braces. [`wire`](Self::wire) already drops the map when an item's
        // language changes, but every language edit also funnels through here, and
        // `attach_all` is rare enough that re-resolving costs nothing. It also keeps a
        // store that was never wired (a headless test) correct.
        self.invalidate_language_cache();
        let work_lang = self.inner.work_lang.borrow().clone();
        let docs: Vec<Rc<OpenDoc>> = self
            .inner
            .open
            .borrow()
            .values()
            .map(|e| e.doc.clone())
            .collect();
        // Resolve every doc's tags under one borrow of the map, then attach outside it.
        let attachments: Vec<(Rc<OpenDoc>, Vec<String>)> = self.with_language_map(|map| {
            docs.into_iter()
                .map(|doc| {
                    let tags = map
                        .get(&doc.item_id)
                        .cloned()
                        .unwrap_or_else(|| work_lang.clone());
                    (doc, tags)
                })
                .collect()
        });
        let work_id = self.inner.work_id.get();
        // Language first, and NOT behind the spell check below: replace-while-typing
        // folds case through the document's language, and a writer with no
        // dictionary installed still gets their lexicon. Spell-check is the
        // feature that needs an engine; this one only needs the tag.
        let punctuation = self.inner.punctuation.borrow().clone();
        for (doc, tags) in &attachments {
            doc.set_replacement_locale(tags);
            // Pushed here too, not only from `set_punctuation`: a document
            // opened after the settings resolved would otherwise never hear
            // about them and would silently substitute nothing.
            doc.set_punctuation(punctuation.clone());
        }
        let Some(spell) = self.inner.spell.borrow().clone() else {
            return;
        };
        for (doc, tags) in attachments {
            doc.attach_spell(&spell, &tags, color, work_id);
        }
    }

    /// Attach the spell-checker to one freshly-built doc (on open / rebuild).
    fn attach_one(&self, doc: &Rc<OpenDoc>) {
        // Comments first and unconditionally: unlike spell-check it needs no
        // engine and no installed dictionary, so a freshly-opened document must
        // re-anchor its threads even in a project with nothing else configured.
        if let Some(vm) = self.inner.comments.borrow().clone() {
            doc.attach_comments(vm);
        }
        // Footnotes on the same footing, and before the markers below: the door
        // is what an editor built from this doc reaches for when the writer asks
        // to insert one.
        if let Some(vm) = self.inner.footnotes.borrow().clone() {
            doc.attach_footnotes(vm);
        }
        // Before anything can paint: a marker map that does not yet know this
        // document's labels renders each one as its raw label, and the writer sees
        // `fn7` in their prose until something else forces a relayout.
        doc.set_footnote_markers(&self.inner.footnote_markers.borrow());
        // The replacement lexicon first, and outside the spell early-return: the
        // two features are independent, and a project with no dictionary
        // installed must still expand its own shorthand.
        if let Some(vm) = self.inner.text_replacements.borrow().clone() {
            doc.attach_replacements(&vm);
            doc.set_replacement_locale(&self.language_for(doc.item_id));
            // The project's punctuation rules too, and for the same reason the
            // locale is set here: this is the path a doc opened *after* the
            // project loaded takes, and it does not go through `attach_all`.
            // Without this a scene opened mid-session substituted nothing at
            // all, while the scenes already open when the project loaded did —
            // the kind of split behaviour that reads as the feature being flaky.
            doc.set_punctuation(self.inner.punctuation.borrow().clone());
        }
        let Some(spell) = self.inner.spell.borrow().clone() else {
            return;
        };
        // A freshly opened doc has no viewer yet, so its synopsis session starts
        // asleep and never eagerly tokenises a (possibly huge) synopsis nobody is
        // looking at. The first mounted view that shows it wakes it — see
        // `OpenDoc::acquire_synopsis_viewer`.
        if let Some(s) = doc.spell_synopsis() {
            s.set_active(doc.synopsis_viewers.get() > 0);
        }
        let tags = self.language_for(doc.item_id);
        doc.attach_spell(
            &spell,
            &tags,
            self.inner.squiggle.get(),
            self.inner.work_id.get(),
        );
    }

    /// Every distinct language tag the open project actually uses — the union across every
    /// item's effective list plus the Work's default. Drives the post-open missing-dictionary
    /// scan (Step 10).
    pub fn project_languages(&self) -> std::collections::BTreeSet<String> {
        let mut out: std::collections::BTreeSet<String> = std::collections::BTreeSet::new();
        self.with_language_map(|map| {
            for list in map.values() {
                out.extend(skribisto_model::language::all(list).map(str::to_string));
            }
        });
        out.extend(
            skribisto_model::language::all(&self.inner.work_lang.borrow()).map(str::to_string),
        );
        out
    }

    /// The effective language list of one item (its own tag, else the Work's), for the
    /// Inspector's placeholder. Falls back to the Work's default.
    pub fn effective_language(&self, item_id: u64) -> Vec<String> {
        self.language_for(item_id)
    }

    /// One item's effective language list, read through the cached map.
    /// [`language_for`](Self::language_for), for tests.
    #[cfg(test)]
    pub(crate) fn language_for_test(&self, item_id: u64) -> Vec<String> {
        self.language_for(item_id)
    }

    fn language_for(&self, item_id: u64) -> Vec<String> {
        self.with_language_map(|map| map.get(&item_id).cloned())
            .unwrap_or_else(|| self.inner.work_lang.borrow().clone())
    }

    /// Drop the cached language map, forcing the next read to re-resolve from the items.
    ///
    /// For the edits the fingerprint cannot see — an item's own `dict_language` or
    /// `sub_role` changing in place. See [`LangFingerprint`].
    fn invalidate_language_cache(&self) {
        *self.inner.lang_cache.borrow_mut() = None;
    }

    /// Run `f` against the project's language map, rebuilding it first if the binder's
    /// shape changed since it was cached.
    ///
    /// Passes the map by reference rather than returning it: a clone would be one
    /// `String` allocation per item on every call, which is most of what the cache is
    /// here to avoid.
    fn with_language_map<R>(&self, f: impl FnOnce(&HashMap<u64, Vec<String>>) -> R) -> R {
        let shape = self.binder_shape();
        let fingerprint = fingerprint_of(
            self.inner.work_id.get(),
            &self.inner.work_lang.borrow(),
            &shape,
        );
        let fresh = matches!(&*self.inner.lang_cache.borrow(), Some((fp, _)) if *fp == fingerprint);
        if !fresh {
            // Build before taking the cache's mutable borrow — the build reads the
            // backend, never the cache.
            let map = self.build_language_map(&shape);
            *self.inner.lang_cache.borrow_mut() = Some((fingerprint, map));
        }
        let cache = self.inner.lang_cache.borrow();
        match &*cache {
            Some((_, map)) => f(map),
            // Unreachable: just filled above. Total rather than `expect`.
            None => f(&HashMap::new()),
        }
    }

    /// The open project's binder item ids, per binder, in document order.
    ///
    /// The cheap half of language resolution: ids only — no `BinderItem` is fetched or
    /// cloned — which is what makes it affordable to check on every read.
    fn binder_shape(&self) -> Vec<Vec<u64>> {
        let Some(work_id) = self.inner.work_id.get() else {
            return Vec::new();
        };
        let ctx = &*self.inner.app_ctx;
        work_commands::get_work_relationship(ctx, &work_id, &WorkRelationshipField::Binders)
            .unwrap_or_default()
            .into_iter()
            .map(|binder_id| {
                binder_commands::get_binder_relationship(
                    ctx,
                    &binder_id,
                    &BinderRelationshipField::BinderItems,
                )
                .unwrap_or_default()
            })
            .collect()
    }

    /// The effective language tag list of every item in the open project, resolved through the
    /// **shared** `skribisto_model::language` rule (an item's own tag, else the Work's) so
    /// spell-check and search never disagree about what language a scene is in.
    ///
    /// The expensive half: one `BinderItem` fetched and cloned per item. Called only when
    /// [`binder_shape`](Self::binder_shape) says the project changed.
    fn build_language_map(&self, shape: &[Vec<u64>]) -> HashMap<u64, Vec<String>> {
        let mut map = HashMap::new();
        if self.inner.work_id.get().is_none() {
            return map;
        }
        let work_lang = self.inner.work_lang.borrow().clone();
        let ctx = &*self.inner.app_ctx;
        for item_ids in shape {
            // Only the fields the resolver reads. Order is irrelevant to the resolver now
            // (each item answers for itself), but relationship order is what we get anyway.
            let items: Vec<BinderItem> = binder_item_commands::get_binder_item_multi(ctx, item_ids)
                .unwrap_or_default()
                .into_iter()
                .flatten()
                .map(|dto| BinderItem {
                    id: dto.id,
                    sub_role: dto.sub_role.clone(),
                    dict_language: dto.dict_language.clone(),
                    ..Default::default()
                })
                .collect();
            skribisto_model::language::tags_in_binder(&work_lang, &items, &mut map);
        }
        map
    }

    /// The aggregate edit signal — bind the debounced autosave to it.
    pub fn edited_any(&self) -> Signal<u64> {
        self.inner.edited.clone()
    }

    /// Look up an **already-open** doc without changing its reference count — for a
    /// read-only consumer (the status-bar word count) that must not perturb the
    /// open/release lifecycle the editor panes own. `None` if the item isn't open.
    pub fn peek(&self, item_id: u64) -> Option<Rc<OpenDoc>> {
        self.inner
            .open
            .borrow()
            .get(&item_id)
            .map(|e| e.doc.clone())
    }

    /// Open item `item_id`, building its [`OpenDoc`] the first time and reusing it
    /// (bumping the refcount) thereafter. `None` if the item can't be read.
    pub fn open(&self, item_id: u64) -> Option<Rc<OpenDoc>> {
        if let Some(entry) = self.inner.open.borrow_mut().get_mut(&item_id) {
            entry.refs += 1;
            return Some(entry.doc.clone());
        }
        // Resolve `(role, sub_role)` via the reactive single (Layer A), then load
        // the allowed content rows.
        self.inner.item_probe.set_id(Some(item_id));
        let item = self.inner.item_probe.dto()?;
        let contents = self.load_contents(item_id, &item.role, &item.sub_role);
        let doc = Rc::new(OpenDoc::build(
            &self.inner.app_ctx,
            item_id,
            &item.role,
            &item.sub_role,
            &contents,
            self.inner.edited.clone(),
            &self.inner.media_dir.borrow(),
        ));
        // Seed the trash state (a trashed item can be opened from the trash dock).
        doc.trashed.set(!item.activated);
        doc.tags.set(item.tags.clone());
        self.inner.open.borrow_mut().insert(
            item_id,
            Entry {
                doc: doc.clone(),
                refs: 1,
            },
        );
        self.attach_one(&doc);
        Some(doc)
    }

    /// Rebuild `item_id`'s document **in place**, keeping its reference count.
    ///
    /// An `OpenDoc`'s fields are decided once, at construction, from the constraint
    /// matrix for the item's `(role, sub_role)`. A Promote rewrites that type, so every
    /// cached doc for it is now wrong — it may hold a prose field the new type has no
    /// room for, or be missing one it now has. Releasing is not enough: while any other
    /// holder still references the entry, the stale doc stays in the map and the next
    /// `open()` hands it straight back.
    ///
    /// Returns the fresh doc, or `None` if the item isn't open (or can't be read).
    pub fn rebuild(&self, item_id: u64, stack: Option<u64>) -> Option<Rc<OpenDoc>> {
        let old = self
            .inner
            .open
            .borrow()
            .get(&item_id)
            .map(|e| e.doc.clone())?;
        // A failed flush here is worse silently ignored than logged: the fresh doc
        // built below re-reads from the persisted `Content` rows, so whatever
        // didn't make it out of `old` is gone the moment this function returns —
        // there is no later retry. We still proceed with the rebuild regardless:
        // `old` is typed for the *previous* (role, sub_role), and a Promote already
        // committed the new one at the backend, so keeping the stale-typed doc
        // around is the worse of the two outcomes documented above.
        if let Err(e) = old.flush(stack) {
            eprintln!("open docs: rebuild flush failed for item {item_id}: {e}");
        }

        self.inner.item_probe.set_id(Some(item_id));
        let item = self.inner.item_probe.dto()?;
        let contents = self.load_contents(item_id, &item.role, &item.sub_role);
        let fresh = Rc::new(OpenDoc::build(
            &self.inner.app_ctx,
            item_id,
            &item.role,
            &item.sub_role,
            &contents,
            self.inner.edited.clone(),
            &self.inner.media_dir.borrow(),
        ));
        if let Some(entry) = self.inner.open.borrow_mut().get_mut(&item_id) {
            entry.doc = fresh.clone();
        }
        self.attach_one(&fresh);
        Some(fresh)
    }

    /// Release one reference to `item_id`. On the **last** reference, flush the
    /// doc (persisting any unsaved edits) and evict it.
    pub fn release(&self, item_id: u64, stack: Option<u64>) {
        let evicted = {
            let mut map = self.inner.open.borrow_mut();
            let Some(entry) = map.get_mut(&item_id) else {
                return;
            };
            entry.refs = entry.refs.saturating_sub(1);
            if entry.refs == 0 {
                map.remove(&item_id).map(|e| e.doc)
            } else {
                None
            }
        };
        if let Some(doc) = evicted {
            // The entry is already gone from `open` by this point (`map.remove`
            // above), so a failed flush here is a genuine, unrecoverable loss of
            // whatever was still unsaved — there is no live entry left to retry
            // against. Logging at least turns a silent loss into a diagnosable
            // one; see `flush_all`'s own note for the same trade-off at shutdown.
            if let Err(e) = doc.flush(stack) {
                eprintln!("open docs: release flush failed for item {item_id}: {e}");
            }
        }
    }

    /// Flush every open doc once (changed fields only).
    pub fn flush_all(&self, stack: Option<u64>) {
        let docs: Vec<Rc<OpenDoc>> = self
            .inner
            .open
            .borrow()
            .values()
            .map(|e| e.doc.clone())
            .collect();
        for doc in docs {
            // Keep flushing the rest even if one document fails — one bad write
            // must not stop every other open document from being saved too.
            if let Err(e) = doc.flush(stack) {
                eprintln!("open docs: flush_all failed for item {}: {e}", doc.item_id);
            }
        }
    }

    /// Drop **all** open docs without flushing — for a project switch, where the
    /// outgoing work is already saved (or discarded) by the close/load flow.
    pub fn clear(&self) {
        self.inner.open.borrow_mut().clear();
        // The incoming project re-resolves from scratch; don't hold the old map's
        // strings until then.
        self.invalidate_language_cache();
    }

    /// Reload only the docs among `item_ids` that are **currently open**, discarding
    /// their live edits and re-reading each present field from disk.
    ///
    /// For a use case that rewrote a `Content` row out from under a live editor — a
    /// project-wide replace being the case here: `replace_in_project` edits the
    /// persisted prose directly, so any open tab of a touched item now shows the
    /// old text until it is re-read. Unlike [`open`](Self::open)+`reload`+`release`,
    /// this never *builds* a doc that wasn't already open (that would parse the
    /// whole item only to evict it a line later), and unlike [`rebuild`](Self::rebuild)
    /// it keeps the same `Rc<OpenDoc>` (the `(role, sub_role)` didn't change, only
    /// the text) so live tab views stay bound. The caller must pump a frame after
    /// (`set_djot` only queues a document event).
    pub fn reload_open(&self, item_ids: &[u64]) {
        let map = self.inner.open.borrow();
        for id in item_ids {
            if let Some(entry) = map.get(id) {
                entry.doc.reload();
            }
        }
    }

    /// Read an item's content rows, keeping only the roles the constraint matrix
    /// allows for its `(role, sub_role)`. `Content.data` is Djot.
    fn load_contents(
        &self,
        item_id: u64,
        role: &BinderItemRole,
        sub_role: &BinderItemSubRole,
    ) -> Vec<ContentDto> {
        let ctx = &*self.inner.app_ctx;
        let allowed = skribisto_model::allowed_content(role, sub_role);
        let content_ids = binder_item_commands::get_binder_item_relationship(
            ctx,
            &item_id,
            &BinderItemRelationshipField::Contents,
        )
        .unwrap_or_default();
        content_commands::get_content_multi(ctx, &content_ids)
            .unwrap_or_default()
            .into_iter()
            .flatten()
            .filter(|c| allowed.contains(&c.role))
            .collect()
    }
}

#[cfg(test)]
impl OpenDocsStore {
    /// Seed one entry directly (bypassing the backend probe) at one reference, so
    /// the refcount/eviction lifecycle is testable without a loaded project.
    ///
    /// `pub(crate)`, not private: `view_models::editors`'s own tests use this to
    /// model two windows sharing one `OpenDocsStore` (Tier 2), each opening
    /// different items, for `EditorsViewModel::release_own_open_docs`'s
    /// "releases only this window's own items" contract.
    pub(crate) fn insert_for_test(&self, doc: Rc<OpenDoc>) {
        let id = doc.item_id;
        self.inner
            .open
            .borrow_mut()
            .insert(id, Entry { doc, refs: 1 });
    }

    /// The current reference count for `item_id`, or `None` if not open.
    pub(crate) fn refs_for_test(&self, item_id: u64) -> Option<usize> {
        self.inner.open.borrow().get(&item_id).map(|e| e.refs)
    }
}

#[cfg(test)]
mod tests {
    /// Space-separated in the tests, a list in storage — one parser, shared.
    use skribisto_model::language::parse_legacy_list as tags;

    use super::*;

    /// The language cache is only as good as what its fingerprint distinguishes. It is
    /// the *shape* half of the contract on [`LangFingerprint`]: every structural change
    /// that can move an item under a different Book — and so change the language it
    /// inherits — must change the fingerprint.
    #[test]
    fn fingerprint_distinguishes_every_structural_change() {
        let base = fingerprint_of(Some(1), &tags("en-US"), &[vec![10, 11, 12]]);

        assert_eq!(
            base,
            fingerprint_of(Some(1), &tags("en-US"), &[vec![10, 11, 12]]),
            "same shape must reuse the cache"
        );
        assert_ne!(
            base,
            fingerprint_of(Some(2), &tags("en-US"), &[vec![10, 11, 12]]),
            "a different work resolves every item differently"
        );
        assert_ne!(
            base,
            fingerprint_of(Some(1), &tags("fr-FR"), &[vec![10, 11, 12]]),
            "the work's default language is the fallback for every item"
        );
        assert_ne!(
            base,
            fingerprint_of(Some(1), &tags("en-US"), &[vec![10, 11, 12, 13]]),
            "a created item"
        );
        assert_ne!(
            base,
            fingerprint_of(Some(1), &tags("en-US"), &[vec![10, 12]]),
            "a removed item"
        );
        // The load-bearing one: a move keeps the same ids, so only *order* betrays it —
        // and order is exactly what decides which Book an item inherits from.
        assert_ne!(
            base,
            fingerprint_of(Some(1), &tags("en-US"), &[vec![12, 11, 10]]),
            "a reorder can change which Book an item sits under"
        );
        assert_ne!(
            base,
            fingerprint_of(Some(1), &tags("en-US"), &[vec![10, 11], vec![12]]),
            "the same ids split across two binders is a different shape"
        );
        assert_ne!(
            base,
            fingerprint_of(None, &tags("en-US"), &[vec![10, 11, 12]]),
            "no open project"
        );
    }

    /// `open()` reuses an already-open doc (bumping refs), `release()` decrements
    /// and keeps it while other references remain, and evicts only at zero — the
    /// lifecycle both split panes rely on for the same item.
    #[test]
    fn refcount_reuses_releases_and_evicts() {
        let ctx = Rc::new(AppContext::new());
        let store = OpenDocsStore::new(ctx.clone());
        // Seed one open doc (as `open()` would for the first consumer).
        let doc = Rc::new(OpenDoc::build(
            &ctx,
            1,
            &BinderItemRole::Item,
            &BinderItemSubRole::Scene,
            &[],
            store.edited_any(),
            std::path::Path::new(""),
        ));
        store.insert_for_test(doc);
        assert_eq!(store.refs_for_test(1), Some(1));

        // A second consumer (e.g. the other pane) reuses the entry — refs 1 -> 2.
        let reused = store.open(1).expect("an already-open doc is reused");
        assert_eq!(store.refs_for_test(1), Some(2));
        assert_eq!(reused.item_id, 1);

        // Releasing once keeps the entry (still referenced by the first consumer).
        store.release(1, None);
        assert_eq!(store.refs_for_test(1), Some(1));

        // Releasing the last reference evicts it.
        store.release(1, None);
        assert_eq!(store.refs_for_test(1), None, "evicted at zero references");

        // Releasing an unknown / already-evicted id is a no-op.
        store.release(1, None);
        assert_eq!(store.refs_for_test(1), None);
    }

    /// `flush_all` must visit **every** open doc, not stop at the first awkward
    /// one — the property the discarded `let _ = doc.flush(stack)` made
    /// unobservable, since a failure there produced no error, no log and no
    /// visible difference.
    ///
    /// It deliberately does **not** try to force a flush failure. The only seam
    /// a unit test can reach is `SingleContent::save`, which short-circuits on
    /// its own `dirty` flag before it ever calls `update_content`, so seeding a
    /// `Content` row with an id that was never created does not actually fail —
    /// asserting against such a doc would be asserting on a stand-in that
    /// silently succeeds. The real "a failed flush never marks a document
    /// clean" guarantee is structural and lives in
    /// [`ProseField::flush`](crate::tabs::ProseField): `set_modified(false)`
    /// sits *after* `content.save(stack)?`, so an `Err` returns before it.
    #[test]
    fn flush_all_visits_every_open_doc() {
        let ctx = Rc::new(AppContext::new());
        let store = OpenDocsStore::new(ctx.clone());

        // Seed the doc with an "existing" `Content` row whose id was never
        // actually created in the store. `flush` then routes its edit through
        // `update_content` on a missing id, which Qleany's generated
        // `UndoableUpdateUseCase` rejects outright ("Entity with id … does not
        // exist") — a small, deterministic stand-in for the real-world case this
        // whole audit is about: the row this tab was editing is gone by the time
        // the write reaches the store (trashed and purged, in the real app).
        let stale_row = ContentDto {
            id: 999_999,
            role: ContentRole::SceneText,
            ..Default::default()
        };
        let doomed = Rc::new(OpenDoc::build(
            &ctx,
            2, // distinct from the healthy doc's item id below
            &BinderItemRole::Item,
            &BinderItemSubRole::Scene,
            &[stale_row],
            store.edited_any(),
            std::path::Path::new(""),
        ));
        let doomed_main = doomed
            .main
            .as_ref()
            .expect("a Scene owns a main text document");
        doomed_main
            .doc
            .set_djot("doomed edit")
            .and_then(|op| op.wait())
            .expect("staging the edit itself must succeed");
        store.insert_for_test(doomed.clone());

        // A second, healthy doc for a real item — flush_all must still reach it.
        let healthy = Rc::new(OpenDoc::build(
            &ctx,
            1,
            &BinderItemRole::Item,
            &BinderItemSubRole::Scene,
            &[],
            store.edited_any(),
            std::path::Path::new(""),
        ));
        healthy.dirty.set(true); // pretend an editor touched it
        store.insert_for_test(healthy.clone());

        // Must not panic, and must not bail out at the first doc.
        store.flush_all(None);

        assert!(
            !doomed_main.doc.is_modified(),
            "the first doc must have been flushed, not skipped"
        );
        assert!(
            !healthy.dirty.get(),
            "flush_all must still reach the doc after the first one"
        );
    }

    /// Changing the project's default language must reach an item that has no
    /// language of its own.
    ///
    /// The store caches the Work language, and the cache fingerprint is computed
    /// *from* that cached value — so a stale cache looks fresh to itself and no
    /// invalidation can rescue it. Before `App` grew an effect over the Work's
    /// `dict_language`, only project load wrote this, and a language edit in
    /// Settings re-resolved every document against the previous language.
    /// Spell-check inherited the same staleness; punctuation only made it
    /// visible.
    #[test]
    fn changing_the_project_language_reaches_an_inheriting_item() {
        let ctx = Rc::new(AppContext::new());
        let store = OpenDocsStore::new(ctx);
        store.set_project_language(Some(1), vec!["en-US".to_string()]);
        assert_eq!(
            store.language_for_test(42),
            vec!["en-US".to_string()],
            "an item with no tag of its own inherits the project's"
        );

        store.set_project_language(Some(1), vec!["es-ES".to_string()]);
        assert_eq!(
            store.language_for_test(42),
            vec!["es-ES".to_string()],
            "…and follows it when the writer changes it"
        );
    }

    /// A language pushed at a document reaches its replace-while-typing session.
    ///
    /// This is the last link in "I set the project to Spanish and `¿` did not
    /// fire": the session-level tests prove a session *with* an `es` locale
    /// fires Spanish, and `language_for` resolution is proven above, but nothing
    /// pinned that `set_replacement_locale` — the call `attach_all` makes after a
    /// language change — actually reaches the session. It does.
    #[cfg(feature = "mocks")]
    #[test]
    fn a_pushed_language_reaches_the_replacement_session() {
        use crate::app_ids::AppIds;
        use crate::models::TextReplacementRuleListModel;
        use crate::singles::SingleWork;
        use crate::view_models::TextReplacementRulesViewModel;

        let ctx = Rc::new(AppContext::new());
        let doc = Rc::new(OpenDoc::build(
            &ctx,
            1,
            &BinderItemRole::Item,
            &BinderItemSubRole::Scene,
            &[],
            Signal::new(0),
            std::path::Path::new(""),
        ));

        let work = SingleWork::new(ctx.clone());
        let ids = AppIds::new();
        let vm = TextReplacementRulesViewModel::new(
            TextReplacementRuleListModel::new(ctx, ids.clone()),
            work,
            ids,
        );
        doc.attach_replacements(&vm);
        doc.set_replacement_locale(&["es-ES".to_string()]);

        let session = doc.replacement_main().expect("a scene has a main session");
        assert_eq!(
            session.locale_for_test(),
            "es-ES",
            "the locale the store pushes must land on the session the editor drives"
        );
    }

    /// The store's document is a live, shareable resource **independent of any
    /// `TabWidget`**: two handles to one `OpenDoc`'s main text are the *same* live
    /// document (an edit in one is seen by the other). This is the exact mechanism
    /// the split view's two panes rely on to show one item side-by-side.
    #[test]
    fn open_doc_shares_one_live_document() {
        let ctx = Rc::new(AppContext::new());
        let doc = OpenDoc::build(
            &ctx,
            1,
            &BinderItemRole::Item,
            &BinderItemSubRole::Scene,
            &[],
            Signal::new(0),
            std::path::Path::new(""),
        );
        let main = doc
            .main
            .as_ref()
            .expect("a Scene owns a main text document");
        // No `TabWidget` anywhere: two bare handles to the shared document.
        let a = main.doc.clone();
        let b = main.doc.clone();
        let _ = a.set_djot("shared edit").and_then(|op| op.wait());
        assert!(
            b.to_djot().unwrap().contains("shared edit"),
            "an edit through one handle must be visible through the other"
        );
    }

    /// Tools ▸ Comments reaches **both** halves of the hide from one call.
    ///
    /// The failure this guards is a half-applied toggle: the margin's cards gone
    /// while the prose is still washed ochre, or the reverse. Two callers setting
    /// the view-model flag and the documents' layers separately is exactly how that
    /// happens, which is why `set_comments_visible` is the single door.
    ///
    /// Mocks-only because it needs a document with real content behind it, which is
    /// what the fabricated binder supplies without a project on disk.
    #[cfg(feature = "mocks")]
    #[test]
    fn hiding_comments_reaches_the_view_model_and_every_open_document() {
        use crate::app_ids::AppIds;
        use crate::models::CommentsListModel;
        use crate::view_models::CommentsViewModel;

        let ctx = Rc::new(AppContext::new());
        let store = OpenDocsStore::new(ctx.clone());
        let vm = CommentsViewModel::new(
            CommentsListModel::new(ctx.clone(), AppIds::new()),
            ctx,
            Signal::new(None),
        );
        store.set_comments(vm.clone());
        let doc = store.open(201).expect("the mock binder's Scene 1");
        let main = doc.comments_main().expect("a prose highlight layer");

        store.set_comments_visible(false);
        assert!(!vm.is_visible(), "the margin's own flag was left showing");
        assert!(!main.is_active(), "the prose layer was left painting");

        store.set_comments_visible(true);
        assert!(vm.is_visible());
        assert!(
            main.is_active(),
            "showing again must wake the layer back up"
        );
    }

    /// `mark_dirty_fn` flips the doc's dirty flag and bumps the store's aggregate
    /// edit counter (what the autosave timer observes) and this doc's edit_gen.
    #[test]
    fn mark_dirty_sets_dirty_and_bumps_edited() {
        let ctx = Rc::new(AppContext::new());
        let edited = Signal::new(0u64);
        let doc = OpenDoc::build(
            &ctx,
            1,
            &BinderItemRole::Item,
            &BinderItemSubRole::Scene,
            &[],
            edited.clone(),
            std::path::Path::new(""),
        );
        assert!(!doc.dirty.get());
        let before = edited.get();
        let gen_before = doc.edit_gen.get();
        doc.mark_dirty_fn()();
        assert!(doc.dirty.get());
        assert_eq!(edited.get(), before + 1);
        assert_eq!(doc.edit_gen.get(), gen_before + 1);
    }
}
