// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The **Scratchpad** dock: the private notes of the item in front of the writer.
//!
//! Trailing rail, after Resources, in its own activity: the two panels there about what
//! sits *beside* the text rather than in it. Which item it shows, and the document it
//! holds open for it, are [`ScratchpadViewModel`]'s; this file only draws what that
//! decided.
//!
//! The panel is a line naming the item ("Scratchpad · Scene 3"), a second line when the
//! item is in the Trash, and the editor filling the rest. Without an item, or for a marker
//! that carries no text, it says why there is nothing to type into.
//!
//! The editor is rebuilt for every item (a `RichTextEditor` is built over one document and
//! cannot be handed another), so the caret the writer left in each scratchpad is read off
//! the editor before it goes and put back when that item comes round again.

use std::cell::RefCell;
use std::rc::Rc;

use teksilo::core::BindingLevel;
use teksilo::prelude::*;
use teksilo::widgets::rich_text::EditorHandle;
use teksilo::widgets::{
    Divider, DockOpenLocation, DockSide, DockWidget, DockWidgetId, Expand, FocusScope, Padding,
    TextWidget, TraversalScopePolicy, VStack,
};

use crate::format::FormatViewModel;
use crate::models::ScratchpadViewerGuard;
use crate::scratchpad::{ScratchpadState, ScratchpadViewModel};

/// Package the Scratchpad panel as a trailing `DockWidget`.
///
/// `format` is the window's formatting view-model: the editor registers with it, so the
/// Format dock and menu act on the scratchpad while the caret is in it.
pub fn scratchpad_dock(
    vm: ScratchpadViewModel,
    format: FormatViewModel,
    dock_id: DockWidgetId,
) -> DockWidget {
    DockWidget::new(dock_id, tr!(scratchpad_title()), move |_id| {
        FocusScope::new(TraversalScopePolicy::Continue)
            .child(ScratchpadPanel::new(vm.clone(), format.clone()))
    })
    .icon(crate::icons::activity::scratchpad_icon)
    .show_header(true)
    .default_location(DockOpenLocation::side(DockSide::Trailing))
}

/// The panel: rebuilt when the item changes, when its document is replaced (a Promote),
/// when it is renamed, and when it goes to or comes back from the Trash.
pub struct ScratchpadPanel {
    vm: ScratchpadViewModel,
    format: FormatViewModel,
    /// Where the editor puts its handle. Held between builds so the caret can be read
    /// off the editor a rebuild is about to replace. Never captured by a callback.
    sink: Rc<RefCell<Option<EditorHandle>>>,
    /// The item the editor in [`Self::sink`] shows.
    shown_item: Option<u64>,
    /// Keeps the shown scratchpad's spell checking awake while the panel is on screen.
    guard: Rc<RefCell<Option<ScratchpadViewerGuard>>>,
    /// `true` while the panel is on screen, `false` while its rail parks it.
    active: Option<Signal<bool>>,
    child: Option<WidgetId>,
}

impl ScratchpadPanel {
    pub fn new(vm: ScratchpadViewModel, format: FormatViewModel) -> Self {
        Self {
            vm,
            format,
            sink: Rc::new(RefCell::new(None)),
            shown_item: None,
            guard: Rc::new(RefCell::new(None)),
            active: None,
            child: None,
        }
    }

    /// Hand the caret of the editor about to go to the view-model, under its item.
    fn keep_caret(&mut self) {
        let handle = self.sink.borrow_mut().take();
        if let (Some(item), Some(handle)) = (self.shown_item.take(), handle) {
            self.vm.remember_caret(item, handle.selection());
        }
    }

    /// The editor over `field`, its header and the Trash line.
    fn ready(
        &mut self,
        ctx: &mut BuildContext,
        doc: &Rc<crate::models::OpenDoc>,
        field: &Rc<crate::tabs::ProseField>,
    ) -> WidgetId {
        let item = doc.item_id;
        let read_only = doc.trashed.get();
        let typo = crate::settings::SettingsViewModel::new(ctx.settings())
            .editor_typography()
            .scratchpad;
        let band = crate::shared::CaretBand::new(
            crate::shared::CaretHighlightSettings::from_context(ctx),
            crate::shared::CaretDimSettings::off(),
            self.vm.locale(item),
        );
        let title = self.vm.title().get();
        let sink = Rc::new(RefCell::new(None));
        let editor = crate::tabs::shared::scratchpad_editor(
            &field.doc,
            &typo,
            doc.mark_scratchpad_dirty_fn(),
            doc.spell_scratchpad(),
            doc.replacement_scratchpad(),
            Some(sink.clone()),
            Some(self.format.clone()),
            Some(band),
            doc.images(),
            read_only,
            tr!(content_scratchpad_text()),
            tr!(scratchpad_description(title = title.clone())),
            Some(doc.content_language()),
        );
        // Back where the writer left it. Before the editor is mounted is fine: the handle
        // is the editor's own state, minted when it was constructed.
        let handle = sink.borrow().clone();
        if let Some(handle) = &handle
            && let Some((anchor, position)) = self.vm.remembered_caret(item)
        {
            handle.select_range(anchor, position);
        }
        // A search hit opened from the Search panel, delivered the way a tab's editor
        // takes one: selected if its words are still there, focused and scrolled into
        // view once the editor is mounted and painted, the caret in the editor either way.
        let pending: crate::search::editor_result::PendingHit =
            Signal::new(self.vm.take_hit_for(item));
        let editor = crate::search::editor_result::Destination::prose(
            editor,
            pending,
            frontend::common::entities::MatchField::Scratchpad,
            &field.doc,
            handle,
        );
        self.sink = sink;
        self.shown_item = Some(item);

        // A localized string, so a change of language reaches it as it reaches the Trash
        // line; a rename rebuilds the panel (`build` binds the title), so the name in it
        // is never stale either. A string resolved once froze the sentence in the
        // language it was first read in.
        let header = tr!(scratchpad_header(title = title));
        teksu!(ctx => VStack {
            spacing: 0.0
            Padding::symmetric(8.0, 12.0) {
                TextWidget::new(header) {
                    style: TextStyleRole::Small
                    color: TextRole::Secondary
                }
            }
            if read_only {
                Padding::symmetric(0.0, 12.0) {
                    TextWidget::new(tr!(scratchpad_trashed())) {
                        style: TextStyleRole::Small
                        color: TextRole::Secondary
                    }
                }
            }
            Divider
            Expand::vertical() {
                child: editor
            }
        })
    }
}

/// A sentence in place of the editor.
fn note(text: LocalizedString) -> impl Widget {
    teksu!(Padding::symmetric(16.0, 24.0) {
        TextWidget::new(text) {
            style: TextStyleRole::Small
            color: TextRole::Secondary
        }
    })
}

impl std::fmt::Debug for ScratchpadPanel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ScratchpadPanel")
            .field("shown_item", &self.shown_item)
            .finish_non_exhaustive()
    }
}

impl Widget for ScratchpadPanel {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        let sid = ctx.self_id();
        let reg = ctx.binding_registry();
        self.vm.target().bind_to(sid, reg, BindingLevel::Rebuild);
        self.vm
            .generation()
            .bind_to(sid, reg, BindingLevel::Rebuild);
        // The name is in the editor's description, which a screen reader reads from the
        // built editor: a rename rebuilds it, the caret kept as for any other rebuild.
        self.vm.title().bind_to(sid, reg, BindingLevel::Rebuild);
        // A search hit to select, perhaps in the item already shown.
        self.vm
            .hit_requests()
            .bind_to(sid, reg, BindingLevel::Rebuild);

        self.keep_caret();
        // The previous item's scratchpad is no longer on screen here.
        *self.guard.borrow_mut() = None;
        let active = match &self.active {
            Some(active) => active.clone(),
            None => {
                let active = ctx.activation_signal(sid);
                self.active = Some(active.clone());
                active
            }
        };

        let id = match self.vm.state() {
            ScratchpadState::NoItem => ctx.add(note(tr!(scratchpad_no_item()))),
            ScratchpadState::NoScratchpad => ctx.add(note(tr!(scratchpad_none()))),
            ScratchpadState::Ready { doc, field } => {
                doc.trashed
                    .bind_to(sid, ctx.binding_registry(), BindingLevel::Rebuild);
                self.vm.note_shown(&doc);
                let id = self.ready(ctx, &doc, &field);
                // Awake while on screen, asleep while the rail parks the panel.
                let guard = self.guard.clone();
                let show = move |on: bool| {
                    *guard.borrow_mut() = on.then(|| doc.acquire_scratchpad_viewer());
                };
                show(active.get());
                ctx.effect(&active, move |on| show(*on));
                id
            }
        };
        self.child = Some(id);
        vec![id]
    }

    fn layout_response(&self, proposal: SizeProposal, ctx: &LayoutContext) -> LayoutResponse {
        self.child
            .and_then(|id| ctx.child_size(id, proposal))
            .map(LayoutResponse::from)
            .unwrap_or_else(|| proposal.resolve(0.0, 0.0).into())
    }
}

impl Drop for ScratchpadPanel {
    fn drop(&mut self) {
        self.keep_caret();
    }
}

#[cfg(test)]
mod tests;
