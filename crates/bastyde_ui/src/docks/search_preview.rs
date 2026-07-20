// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The search **preview** dock (bottom band): a full-width, **editable** view of
//! the currently-selected result's paragraph.
//!
//! It binds the *same* `Rc<OpenDoc>` an editor tab does — obtained from the
//! shared [`OpenDocsStore`](crate::models::OpenDocsStore) via the
//! [`SearchReplaceViewModel`], refcounted so it is neither evicted while a tab
//! still holds it nor pinned forever — so fixing a typo here **is** editing the
//! manuscript: one document, two views. Edits mark the doc dirty
//! ([`OpenDoc::mark_dirty_fn`](crate::models::OpenDoc)) so autosave persists them,
//! exactly as a tab's edits do.
//!
//! The body rebuilds when the selected result changes (a `BindingLevel::Rebuild`
//! bind on the view-model's selection signal), swapping in the new document — or
//! an empty state when nothing is selected, or when the match was in a field with
//! no editable prose (a title / label).

use bastyde::core::binding::BindingLevel;
use bastyde::core::styles::{RichTextEditorStyle, RichTextEditorStyleConfig};
use bastyde::core::widget::WidgetPlacement;
use bastyde::prelude::*;
use bastyde::tokens::HAlignment;
use bastyde::widgets::rich_text::{RichTextEditor, ScrollPolicy};
use bastyde::widgets::{
    Button, ButtonVariant, Center, DockOpenLocation, DockSide, DockWidget, DockWidgetId,
    FocusScope, IconLocation, Padding, ScrollArea, TextWidget, TraversalScopePolicy, VStack,
};

use frontend::common::entities::MatchField;

use crate::models::OpenDoc;
use crate::tabs::ProseField;
use crate::view_models::{EditorKind, FormatViewModel, SearchReplaceViewModel};

/// An editor that paints **no background of its own**, so it sits flush on the
/// dock's surface (rather than reading as a floating card dropped onto it).
struct SeamlessEditorStyle;

impl RichTextEditorStyle for SeamlessEditorStyle {
    fn make_body(&self, cfg: &RichTextEditorStyleConfig, ctx: &mut BuildContext) -> WidgetId {
        match cfg.content_padding {
            Some((t, r, b, l)) => ctx.add(Padding::new(t, r, b, l).child_id(cfg.viewport)),
            None => cfg.viewport,
        }
    }
}

/// The bottom-side preview dock. `FocusScope` with `Continue` (groups its Tab
/// order without trapping the keyboard), no header (the band's height is spent on
/// prose, not chrome), fronted by a rail glyph so a hidden band can be reopened.
pub fn search_preview_dock(vm: SearchReplaceViewModel, dock_id: DockWidgetId) -> DockWidget {
    DockWidget::new(dock_id, tr!(search_preview()), move |_id| {
        FocusScope::new(TraversalScopePolicy::Continue).child(PreviewBody::new(vm.clone()))
    })
    .icon(crate::icons::activity::search_preview_icon)
    .show_header(false)
    .default_location(DockOpenLocation::side(DockSide::Bottom))
}

/// The dock body: rebuilds on selection change, hosting the selected result's
/// editable document or an empty state.
struct PreviewBody {
    vm: SearchReplaceViewModel,
    child_id: Option<WidgetId>,
    /// The find-highlight layer over the previewed document — so the shown
    /// paragraph highlights the same query the result list matched. Recreated per
    /// previewed doc (dropped, and its highlights with it, when the preview
    /// changes or clears).
    find: std::rc::Rc<std::cell::RefCell<Option<bastyde::widgets::rich_text::FindSession>>>,
    /// The document spell session this preview view currently feeds (its caret) and drives (per
    /// frame), with the view token. The preview is another live view of a shared `OpenDoc`
    /// document, so it participates in the caret-aware exemption like any editor. Tracked here to
    /// release it on a doc-switch rebuild (the effects tear down but never fire `on_blur`) and on
    /// drop — otherwise a stale caret reader would pin a frozen exemption for that document.
    spell_view: Option<(std::rc::Rc<crate::spellcheck::SpellSession>, WidgetId)>,
    /// The formatting registry this preview's editor is announced to, and under
    /// which id. Withdrawn on a doc-switch rebuild and on drop, so the Format
    /// dock and menu can never act through a preview that is no longer shown.
    format_view: Option<(FormatViewModel, WidgetId)>,
}

impl PreviewBody {
    fn new(vm: SearchReplaceViewModel) -> Self {
        Self {
            vm,
            child_id: None,
            find: std::rc::Rc::new(std::cell::RefCell::new(None)),
            spell_view: None,
            format_view: None,
        }
    }

    /// Attach (or replace) a find-highlight session over the previewed document
    /// `doc`, seeded with the current search query, and keep it live: re-run on a
    /// query/option change and re-derive after an edit. The session is
    /// paint-only, so the previewed prose shows every match the result list found.
    fn install_find_highlight(
        &self,
        ctx: &mut BuildContext,
        doc: &bastyde::text_document::TextDocument,
    ) {
        use bastyde::widgets::rich_text::FindSession;

        let colors = &ctx.theme().colors;
        let current = crate::tabs::shared::editor::highlight_of(
            SurfaceRole::Accent,
            Some(TextRole::OnAccent),
            colors,
        );
        let other =
            crate::tabs::shared::editor::highlight_of(SurfaceRole::AccentSubtle, None, colors);
        let mut session = FindSession::new(doc, current, other);
        session.set_query(&self.vm.query_signal().get(), &self.vm.find_options());
        *self.find.borrow_mut() = Some(session);

        // Re-run when the query or the matching options change; the preview does
        // not rebuild on those (only on a selection change), so the stored session
        // is updated in place.
        let update = {
            let find = self.find.clone();
            let vm = self.vm.clone();
            move || {
                if let Some(s) = find.borrow_mut().as_mut() {
                    s.set_query(&vm.query_signal().get(), &vm.find_options());
                }
            }
        };
        macro_rules! on_change {
            ($sig:expr) => {{
                let update = update.clone();
                ctx.effect(&$sig, move |_| update());
            }};
        }
        on_change!(self.vm.query_signal());
        on_change!(self.vm.case_sensitive_signal());
        on_change!(self.vm.whole_word_signal());
        on_change!(self.vm.diacritic_sensitive_signal());

        // Re-derive the matches if an edit (here or in an open tab of the same
        // document) moved the offsets, keeping the highlight boxes aligned.
        let find = self.find.clone();
        let tick = ctx.frame_tick();
        ctx.effect(&tick, move |_| {
            if let Some(s) = find.borrow_mut().as_mut() {
                s.refresh_if_stale();
            }
        });
    }
}

impl Drop for PreviewBody {
    fn drop(&mut self) {
        // Release the document's spell session so a destroyed preview doesn't pin a stale caret
        // exemption for a scene tab still showing the same document.
        if let Some((spell, token)) = self.spell_view.take() {
            spell.on_blur(token);
        }
        if let Some((format, token)) = self.format_view.take() {
            format.unregister(token);
        }
    }
}

impl std::fmt::Debug for PreviewBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PreviewBody").finish()
    }
}

impl Widget for PreviewBody {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        // Rebuild whenever the selected result changes — that is exactly when the
        // previewed document (and its matched field) changes.
        self.vm.selected_result_signal().bind_to(
            ctx.self_id(),
            ctx.binding_registry(),
            BindingLevel::Rebuild,
        );

        // Release the spell session this preview fed on the previous build. A doc-switch rebuild
        // tears the caret effects down but never fires `on_blur`, so do it here (and on drop);
        // the current document is re-wired below.
        if let Some((old, tok)) = self.format_view.take() {
            old.unregister(tok);
        }
        if let Some((old, tok)) = self.spell_view.take() {
            old.on_blur(tok);
        }
        let doc = self.vm.preview_signal().get();
        let field = self.vm.preview_field_signal().get();
        let child: Box<dyn Widget> = match doc.as_ref().and_then(|d| editable_field(d, field)) {
            Some((open_doc, prose, spell, kind)) => {
                self.install_find_highlight(ctx, &prose.doc);
                // Cap the editor's width like a scene column, so a wide paragraph
                // stays readable (Settings ▸ preview width). Flowing (intrinsic
                // height, inner scroll off) inside an outer `ScrollArea` — the same
                // shape the scene tabs use — so the band scrolls a long paragraph.
                let width =
                    crate::view_models::SettingsViewModel::new(ctx.settings()).preview_width();
                let editor = RichTextEditor::editor(prose.doc.clone())
                    .style(SeamlessEditorStyle)
                    .on_change(open_doc.mark_dirty_fn())
                    .content_padding_symmetric(8.0, 8.0)
                    .v_scroll_policy(ScrollPolicy::AlwaysOff);
                // The preview is another live view of a shared document. Feed its caret and drive
                // the doc's spell session's per-frame recompute — otherwise an edit here would
                // never re-tick the squiggles (stale), and the caret word wouldn't be exempt,
                // whenever no scene tab of the same document is open to tick it.
                if let Some(spell) = &spell {
                    let handle = editor.handle();
                    let token = crate::tabs::shared::editor::wire_spell(ctx, &handle, spell);
                    self.spell_view = Some((spell.clone(), token));
                }
                // The preview band is a real editing surface — it writes through to
                // the shared document — so the formatting surfaces must reach it too.
                // Registered here rather than through `TypographyBoundEditor` (which
                // this editor deliberately does not use: it carries the seamless
                // style and the preview's own width, not a tab's typography), on the
                // same release-on-rebuild-and-drop discipline as `spell_view` above.
                if let Some(format) = ctx.app_state::<FormatViewModel>().cloned() {
                    let self_id = ctx.self_id();
                    format.register(self_id, editor.handle(), kind);
                    self.format_view = Some((format, self_id));
                }
                Box::new(
                    ScrollArea::new().child(
                        Padding::symmetric(12.0, 8.0)
                            .child(crate::tabs::shared::editor::centered(editor, &width)),
                    ),
                )
            }
            None => {
                // Nothing previewed — drop any highlight layer.
                *self.find.borrow_mut() = None;
                Box::new(empty_state(&self.vm, doc.is_some()))
            }
        };
        self.child_id = Some(ctx.add_boxed(child));
        self.child_id.into_iter().collect()
    }

    fn layout_response(&self, proposal: SizeProposal, ctx: &LayoutContext) -> LayoutResponse {
        match self.child_id.and_then(|id| ctx.child_size(id, proposal)) {
            Some(size) => size.into(),
            None => proposal.resolve(0.0, 0.0).into(),
        }
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

    fn children(&self) -> Vec<WidgetId> {
        self.child_id.into_iter().collect()
    }
}

/// The editable prose field to preview for a match, and the doc that owns it (for
/// the dirty hook). A synopsis match shows the synopsis; everything else (body,
/// and title/label matches, which have no rich field of their own) shows the main
/// prose — falling back to whichever field the item actually has. `None` when the
/// item has no editable prose at all (a folder / heading).
fn editable_field(
    open_doc: &std::rc::Rc<OpenDoc>,
    field: Option<MatchField>,
) -> Option<(
    std::rc::Rc<OpenDoc>,
    &ProseField,
    Option<std::rc::Rc<crate::spellcheck::SpellSession>>,
    EditorKind,
)> {
    // Pair the resolved prose field with ITS spell session (main vs synopsis), honouring the same
    // fallback, so the preview drives the right document's squiggles. The kind rides along for the
    // formatting registry — the fallbacks mean the *matched* field is not always the field shown,
    // so it has to be read off the branch actually taken.
    let (prose, spell, kind) = match field {
        Some(MatchField::Synopsis) => match open_doc.synopsis.as_ref() {
            Some(p) => (p, open_doc.spell_synopsis(), EditorKind::Synopsis),
            None => (
                open_doc.main.as_ref()?,
                open_doc.spell_main(),
                EditorKind::Prose,
            ),
        },
        _ => match open_doc.main.as_ref() {
            Some(p) => (p, open_doc.spell_main(), EditorKind::Prose),
            None => (
                open_doc.synopsis.as_ref()?,
                open_doc.spell_synopsis(),
                EditorKind::Synopsis,
            ),
        },
    };
    Some((open_doc.clone(), prose, spell, kind))
}

/// The empty state. With a result selected but no editable prose, a plain note.
/// With nothing selected, an invitation to search — text plus a button that
/// reveals the leading search dock, so the band is never a dead end.
fn empty_state(vm: &SearchReplaceViewModel, has_doc: bool) -> impl Widget {
    if has_doc {
        return Center::new().child(
            Padding::symmetric(24.0, 16.0).child(
                TextWidget::new(tr!(search_preview_no_prose()))
                    .style(TextStyleRole::Small)
                    .color(TextRole::Secondary),
            ),
        );
    }
    let vm = vm.clone();
    Center::new().child(
        Padding::symmetric(24.0, 16.0).child(
            VStack::new()
                .spacing(12.0)
                .alignment(HAlignment::Center)
                .child(
                    TextWidget::new(tr!(search_preview_prompt()))
                        .style(TextStyleRole::Small)
                        .color(TextRole::Secondary),
                )
                .child(
                    Button::new(tr!(search_preview_open_search()))
                        .variant(ButtonVariant::Tinted)
                        .icon(crate::icons::activity::search_icon(), IconLocation::Leading)
                        .on_activate_fn(move |_| vm.reveal_search()),
                ),
        ),
    )
}
