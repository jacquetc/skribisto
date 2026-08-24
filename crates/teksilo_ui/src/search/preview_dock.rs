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

use teksilo::core::binding::BindingLevel;
use teksilo::core::styles::{RichTextEditorStyle, RichTextEditorStyleConfig};
use teksilo::core::widget::WidgetPlacement;
use teksilo::prelude::*;
use teksilo::tokens::HAlignment;
use teksilo::widgets::rich_text::{RichTextEditor, ScrollPolicy};
use teksilo::widgets::{
    Button, ButtonVariant, Center, DockOpenLocation, DockSide, DockWidget, DockWidgetId,
    FocusScope, IconLocation, Padding, ScrollArea, TextWidget, TraversalScopePolicy, VStack,
};

use frontend::common::entities::MatchField;

use crate::format::{EditorKind, FormatViewModel};
use crate::models::OpenDoc;
use crate::search::SearchReplaceViewModel;
use crate::tabs::ProseField;

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
pub fn search_preview_dock(
    vm: SearchReplaceViewModel,
    format: FormatViewModel,
    games: crate::writing_session::WritingGamesViewModel,
    dock_id: DockWidgetId,
) -> DockWidget {
    DockWidget::new(dock_id, tr!(search_preview()), move |_id| {
        FocusScope::new(TraversalScopePolicy::Continue).child(PreviewBody::new(
            vm.clone(),
            format.clone(),
            games.clone(),
        ))
    })
    .icon(crate::icons::activity::search_preview_icon)
    .show_header(false)
    .default_location(DockOpenLocation::side(DockSide::Bottom))
}

/// The dock body: rebuilds on selection change, hosting the selected result's
/// editable document or an empty state.
struct PreviewBody {
    vm: SearchReplaceViewModel,
    format: FormatViewModel,
    /// The writing games this project is playing. The preview is a **live view of
    /// the same document** a scene tab shows, so a game that has frozen the
    /// manuscript has to freeze it here too — otherwise the band is a way to
    /// delete prose that Backspace refuses to delete two panes away.
    games: crate::writing_session::WritingGamesViewModel,
    child_id: Option<WidgetId>,
    /// The find-highlight layer over the previewed document — so the shown
    /// paragraph highlights the same query the result list matched. Recreated per
    /// previewed doc (dropped, and its highlights with it, when the preview
    /// changes or clears).
    find: std::rc::Rc<std::cell::RefCell<Option<teksilo::widgets::rich_text::FindSession>>>,
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
    fn new(
        vm: SearchReplaceViewModel,
        format: FormatViewModel,
        games: crate::writing_session::WritingGamesViewModel,
    ) -> Self {
        Self {
            vm,
            format,
            games,
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
        doc: &teksilo::text_document::TextDocument,
    ) {
        use teksilo::widgets::rich_text::FindSession;

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
                // The same moment tells the margin lane what this search is looking
                // for, on every surface rather than only this band. This is the
                // project half of the arbiter the find banner writes the other half
                // of, and it belongs here because here is where the search is being
                // *used*, which is what "last one used wins" has to mean.
                vm.publish_to_lane();
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

        // And once now, for the query that was already typed when this band was
        // built: the effects above only fire on a *change*, and selecting a result
        // is not one.
        self.vm.publish_to_lane();

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

/// The preview band, with the margin lane beside it.
///
/// The same shape [`panes::laned`](crate::tabs::shared::panes) uses, and for the
/// same reasons: the lane is the scroll area's *sibling* because it maps the extent
/// the area scrolls, `Expand` on the prose side so the strip cannot narrow the
/// measure, and the content reports where it landed so the lane hears about a
/// reflow at all.
///
/// Not shared with `panes::laned` despite the shape, because that one takes a
/// `ContentTab` and this band has none: it is a preview of a document, not a tab
/// open on one.
#[allow(clippy::too_many_arguments)]
fn laned_band(
    vm: &SearchReplaceViewModel,
    format: &FormatViewModel,
    item: u64,
    // The band's own token, shared with the editor above — the previewed scene is
    // very often also open in a tab, and both register for the same item. See
    // `crate::margin_lane::LaneScope`.
    scope: crate::margin_lane::LaneScope,
    doc: teksilo::text_document::TextDocument,
    kind: crate::format::EditorKind,
    content: impl Widget + 'static,
) -> impl Widget {
    use teksilo::widgets::{Expand, HStack};

    use crate::margin_lane::{LaneInputs, LaneRow, LaneRows, LaneSurface};

    let area = ScrollArea::new();
    let extents = crate::margin_lane::RowExtents::new();
    let app_ctx = vm.app_ctx();
    let row = LaneRow {
        item,
        markers: crate::margin_lane::texture::markers_for_item(
            &app_ctx,
            vm.ids().work_id.get(),
            item,
        ),
        doc,
        // No spell session either, and for the reason the comment anchors give
        // below: this band mounts no highlight layer, so there is nothing keeping
        // a set of live offsets ticking. A preview is for finding the hit, not
        // for proofreading.
        spell: None,
        // **No comment anchors here, and not because the document has none.** The
        // live offsets belong to an editor's own highlight session, and this band
        // mounts no comment layer to keep one ticking; the stored offsets are only
        // rewritten when some editor's comment margin rebuilds, so reading those
        // instead would put marks a paragraph out on the one surface least able to
        // notice. A preview shows where the hits are; the tab is where the notes are.
        comments: None,
    };
    let lane = crate::margin_lane::lane_for(
        &area,
        LaneInputs {
            app_ctx,
            ids: vm.ids().clone(),
            surface: LaneSurface::SearchPreview,
            kind,
            scope,
            format: format.clone(),
            rows: LaneRows::Placed {
                extents: extents.clone(),
                row: std::rc::Rc::new(move |_| Some(row.clone())),
            },
        },
    );
    let band =
        crate::margin_lane::RowExtent::new(item, extents, area.scroll_y_signal().clone(), content);
    HStack::new()
        .child(Expand::new().child(area.child(band)))
        .child(lane)
}

impl Drop for PreviewBody {
    fn drop(&mut self) {
        // Take this search's hits off every lane in the window. Only its own: the
        // writer may have used Ctrl+F since, and clearing unconditionally would wipe
        // marks nobody asked to lose.
        crate::margin_lane::clear_active_query_from(crate::margin_lane::LaneQuerySource::Project);
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
        // Cloned into the closure below (`MatchField` is not `Copy`) so `field`
        // itself survives to be read again in the `None` arm's footnote check.
        let child: Box<dyn Widget> = match doc
            .as_ref()
            .and_then(|d| editable_field(d, field.clone()))
        {
            Some((open_doc, prose, spell, kind)) => {
                self.install_find_highlight(ctx, &prose.doc);
                // This band's own token. The scene being previewed is very often the
                // one open in a tab as well, and both register an editor for the same
                // item — so the band's lane has to be able to say which of the two is
                // *its*. See `crate::margin_lane::LaneScope`.
                let scope = crate::margin_lane::LaneScope::fresh();
                // Cap the editor's width like a scene column, so a wide paragraph
                // stays readable (Settings ▸ preview width). Flowing (intrinsic
                // height, inner scroll off) inside an outer `ScrollArea` — the same
                // shape the scene tabs use — so the band scrolls a long paragraph.
                //
                // `min_lines` is what *makes* it flowing, and it is load-bearing:
                // without it the editor is greedy, and `centered`'s
                // `CenterColumnFlowing` measures its child width-only — so a
                // greedy editor falls through to `RichTextEditor`'s
                // `proposal.height.unwrap_or(100.0)` fallback instead of
                // growing with the prose.
                let width = crate::settings::SettingsViewModel::new(ctx.settings()).preview_width();
                let editor = RichTextEditor::editor(prose.doc.clone())
                    // Registered with the format view-model below, so the Link command
                    // reaches this band — and a link it makes here must be followable.
                    .on_link_activated(|href, ctx| {
                        crate::shared::external_link::open_external_link(href, ctx);
                    })
                    .style(SeamlessEditorStyle)
                    .on_change(open_doc.mark_dirty_fn())
                    .content_padding_symmetric(8.0, 8.0)
                    .min_lines(1)
                    .v_scroll_policy(ScrollPolicy::AlwaysOff)
                    // The preview holds the whole document, not just the matched
                    // paragraph, and it is laid out at full document height — so
                    // window the render to the visible clip, exactly as the scene
                    // tabs do, or previewing a match in a 13k-word scene would
                    // rasterize every row of it on each paint.
                    .window_to_clip(true);
                // This band edits the manuscript, so it answers to the same
                // writing game every other view of that prose does. Pushed at
                // build and re-pushed from three effects, exactly as
                // `TypographyBoundEditor` does — the preview deliberately
                // bypasses that wrapper for typography, so it has to carry
                // this itself. `EditorKind::Prose`: what the band shows is a
                // scene's text, whichever `Content` row the match came from.
                {
                    let handle = editor.handle();
                    let games = self.games.clone();
                    let push = {
                        let (h, g) = (handle.clone(), games.clone());
                        move || h.set_command_filter(g.filter_for(crate::format::EditorKind::Prose))
                    };
                    push();
                    {
                        let push = push.clone();
                        ctx.effect(&games.always_forward(), move |_| push());
                    }
                    {
                        let push = push.clone();
                        ctx.effect(&games.forward_in_prose(), move |_| push());
                    }
                    ctx.effect(&games.forward_in_synopsis(), move |_| push());
                }
                // The preview is another live view of a shared document. Feed its caret and drive
                // the doc's spell session's per-frame recompute — otherwise an edit here would
                // never re-tick the squiggles (stale), and the caret word wouldn't be exempt,
                // whenever no scene tab of the same document is open to tick it.
                if let Some(spell) = &spell {
                    let handle = editor.handle();
                    let token = crate::tabs::shared::editor::wire_spell(ctx, &handle, spell);
                    self.spell_view = Some((spell.clone(), token));
                }
                // …and the ambient caret band, for the same reason: this is a real writing
                // surface, so the setting must reach it like every other one. Wired by hand
                // here because this editor deliberately bypasses `TypographyBoundEditor`, which
                // is where every other surface picks the band up.
                {
                    let handle = editor.handle();
                    let band = crate::shared::CaretBand::new(
                        crate::shared::CaretHighlightSettings::from_context(ctx),
                        self.vm.preview_locale(open_doc.item_id),
                    );
                    handle.set_caret_highlight(band.resolve());
                    {
                        let (h, b) = (handle.clone(), band.clone());
                        ctx.effect(&band.settings.scope, move |_| {
                            h.set_caret_highlight(b.resolve())
                        });
                    }
                    {
                        let (h, b) = (handle.clone(), band.clone());
                        ctx.effect(&band.settings.color, move |_| {
                            h.set_caret_highlight(b.resolve())
                        });
                    }
                }
                // The preview band is a real editing surface — it writes through to
                // the shared document — so the formatting surfaces must reach it too.
                // Registered here rather than through `TypographyBoundEditor` (which
                // this editor deliberately does not use: it carries the seamless
                // style and the preview's own width, not a tab's typography), on the
                // same release-on-rebuild-and-drop discipline as `spell_view` above.
                {
                    let format = self.format.clone();
                    let self_id = ctx.self_id();
                    format.register(self_id, editor.handle(), kind);
                    // …and which item it is showing, which is the one thing a margin
                    // lane needs and focus cannot answer. `TypographyBoundEditor`
                    // does the same for every other writing surface; this band
                    // bypasses it, so it says so itself.
                    format.set_registered_anchor(
                        self_id,
                        crate::margin_lane::LaneAnchor::new(open_doc.item_id, scope),
                    );
                    self.format_view = Some((format, self_id));
                }
                Box::new(laned_band(
                    &self.vm,
                    &self.format,
                    open_doc.item_id,
                    scope,
                    prose.doc.clone(),
                    kind,
                    Padding::symmetric(12.0, 8.0)
                        .child(crate::tabs::shared::editor::centered(editor, &width)),
                ))
            }
            None => {
                // Nothing previewed — drop any highlight layer.
                *self.find.borrow_mut() = None;
                if field == Some(MatchField::Footnote) {
                    // `SearchReplaceViewModel::select_result` deliberately never
                    // opens a document for a footnote hit (its match is the note's
                    // own body, never a field of any scene) — so `doc` is always
                    // `None` here too, and this branch, not the generic empty
                    // states below, is what actually fires for a footnote match.
                    Box::new(footnote_empty_state(&self.vm))
                } else {
                    Box::new(empty_state(&self.vm, doc.is_some()))
                }
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

/// The document, prose field, spell session and editor kind a preview row
/// resolved to. They travel together because the *matched* field is not always
/// the field shown (see the fallbacks below), so re-deriving any one of them
/// downstream would risk disagreeing with the others.
type PreviewTarget<'a> = (
    std::rc::Rc<OpenDoc>,
    &'a ProseField,
    Option<std::rc::Rc<crate::spellcheck::SpellSession>>,
    EditorKind,
);

/// The editable prose field to preview for a match, and the doc that owns it (for
/// the dirty hook). A synopsis match shows the synopsis; everything else (body,
/// and title/label matches, which have no rich field of their own) shows the main
/// prose — falling back to whichever field the item actually has. `None` when the
/// item has no editable prose at all (a folder / heading).
fn editable_field(
    open_doc: &std::rc::Rc<OpenDoc>,
    field: Option<MatchField>,
) -> Option<PreviewTarget<'_>> {
    // Pair the resolved prose field with ITS spell session (main vs synopsis), honouring the same
    // fallback, so the preview drives the right document's squiggles. The kind rides along for the
    // formatting registry — the fallbacks mean the *matched* field is not always the field shown,
    // so it has to be read off the branch actually taken.
    let (prose, spell, kind) = match field {
        // A comment hit has no prose field to preview. Falling through to the
        // scene's body — which is what the `_` arm below would do — would show the
        // writer a page that does not contain their query at all, with the match
        // highlighted nowhere: the worst of both, since it looks like the preview
        // works and simply found nothing. The thread itself lives in the margin and
        // in the two comment docks.
        // A footnote's body is prose, but it is not any of this document's
        // editors: falling through would highlight the scene, which does not
        // contain the match. Same trap, same answer — the note is shown in the
        // footnotes dock instead.
        Some(MatchField::Comment) | Some(MatchField::CommentReply) | Some(MatchField::Footnote) => {
            return None;
        }
        Some(MatchField::Synopsis) => match open_doc.synopsis.as_ref() {
            Some(p) => (p, open_doc.spell_synopsis(), EditorKind::Synopsis),
            None => (
                open_doc.main.as_ref()?,
                open_doc.spell_main(),
                EditorKind::Prose,
            ),
        },
        // An epigraph match names its own field, so it resolves exactly rather than
        // through the fallback chain — which matters because a chapter can hold scene
        // prose *and* an epigraph, and preferring `main` would show the writer a
        // document their search never matched. Falls back only if the field is gone
        // (the item was converted out from under a stale result).
        Some(MatchField::Epigraph) => match open_doc.epigraph.as_ref() {
            Some(p) => (p, open_doc.spell_epigraph(), EditorKind::Prose),
            None => (
                open_doc.main.as_ref()?,
                open_doc.spell_main(),
                EditorKind::Prose,
            ),
        },
        // Body, and the title/label matches, which have no rich field of their own.
        _ => match open_doc.main.as_ref() {
            Some(p) => (p, open_doc.spell_main(), EditorKind::Prose),
            None => match open_doc.epigraph.as_ref() {
                Some(p) => (p, open_doc.spell_epigraph(), EditorKind::Prose),
                None => (
                    open_doc.synopsis.as_ref()?,
                    open_doc.spell_synopsis(),
                    EditorKind::Synopsis,
                ),
            },
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

/// The footnote-match empty state: a hit whose text lives in a `Footnote`'s own
/// body, not in any open document — `editable_field` returns `None` for it by
/// design (see its own comment), and `select_result` never opens a document for it
/// either (see that method's comment), so this is what fires instead of the
/// generic [`empty_state`] whenever the selected result is a footnote hit.
///
/// Explains where the match actually is and offers a working way there, rather
/// than the dead end this used to be: selecting the result already reveals the
/// Footnotes dock as a side effect (`SearchReplaceViewModel::select_result`), so
/// this button mostly matters when the writer closed that dock again afterward —
/// same button, same door, not a second one.
fn footnote_empty_state(vm: &SearchReplaceViewModel) -> impl Widget {
    let vm = vm.clone();
    Center::new().child(
        Padding::symmetric(24.0, 16.0).child(
            VStack::new()
                .spacing(12.0)
                .alignment(HAlignment::Center)
                .child(
                    TextWidget::new(tr!(search_preview_footnote_prompt()))
                        .style(TextStyleRole::Small)
                        .color(TextRole::Secondary),
                )
                .child(
                    Button::new(tr!(search_preview_open_footnotes()))
                        .variant(ButtonVariant::Tinted)
                        .icon(
                            crate::icons::activity::footnotes_icon(),
                            IconLocation::Leading,
                        )
                        .on_activate_fn(move |_| vm.reveal_footnotes()),
                ),
        ),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use frontend::AppContext;
    use frontend::common::entities::{BinderItemRole, BinderItemSubRole};
    use std::rc::Rc;
    use teksilo::core::widget_tree::WidgetTree;
    use teksilo::prelude::SizeProposal;

    use crate::app_ids::AppIds;
    use crate::models::{OpenDocsStore, SearchResultsModel, SearchSettingsService};
    use teksilo::widgets::DockingModel;

    /// A view-model whose preview holds `paragraphs` paragraphs of scene prose.
    fn vm_previewing(paragraphs: usize) -> (Rc<AppContext>, SearchReplaceViewModel) {
        let app_ctx = Rc::new(AppContext::new());
        let ids = AppIds::new();
        let vm = SearchReplaceViewModel::new(
            app_ctx.clone(),
            ids,
            SearchResultsModel::new(app_ctx.clone(), Signal::new(None)),
            SearchSettingsService::in_memory_default(),
            OpenDocsStore::new(app_ctx.clone()),
            DockingModel::new(),
            DockWidgetId::fresh(),
            DockWidgetId::fresh(),
        );
        let doc = Rc::new(OpenDoc::build(
            &app_ctx,
            1,
            &BinderItemRole::Item,
            &BinderItemSubRole::Scene,
            &[],
            Signal::new(0),
            std::path::Path::new(""),
        ));
        let _ = doc
            .main
            .as_ref()
            .expect("a scene has main prose")
            .doc
            .set_djot_sync(&"The rain had not stopped since Tuesday.\n\n".repeat(paragraphs));
        vm.preview_signal().set(Some(doc));
        vm.preview_field_signal().set(Some(MatchField::Body));
        (app_ctx, vm)
    }

    fn find(tree: &WidgetTree, id: WidgetId, needle: &str) -> Option<WidgetId> {
        if tree
            .widget_type_name(id)
            .is_some_and(|t| t.contains(needle))
        {
            return Some(id);
        }
        tree.children(id)
            .into_iter()
            .find_map(|c| find(tree, c, needle))
    }

    /// The height of the editor's text body inside a preview dock laid out at
    /// `dock_height`.
    fn editor_body_height(paragraphs: usize, dock_height: f32) -> f32 {
        let (ctx, vm) = vm_previewing(paragraphs);
        let mut tree = crate::test_support::tree_with_settings(&ctx);
        let root = tree.add(PreviewBody::new(
            vm,
            FormatViewModel::detached(),
            crate::writing_session::WritingGamesViewModel::detached(),
        ));
        tree.layout(SizeProposal::exact(900.0, dock_height));
        let body = find(&tree, root, "RichTextEditorBody")
            .expect("the preview mounts a rich text editor over the previewed document");
        tree.bounds(body).height
    }

    /// One line of the embedded font, measured the way an intrinsic editor reports
    /// it — the floor `min_lines(1)` puts under the preview.
    fn one_line() -> f32 {
        let mut tree = WidgetTree::new();
        let root = tree
            .add(RichTextEditor::editor(teksilo::text_document::TextDocument::new()).min_lines(1));
        tree.layout(SizeProposal::with_width(900.0));
        let body = find(&tree, root, "RichTextEditorBody").expect("a rich text editor has a body");
        tree.bounds(body).height
    }

    /// The preview editor must size **intrinsically** (`min_lines`), never
    /// greedily — [`centered`](crate::tabs::shared::editor::centered)'s
    /// `CenterColumnFlowing` measures its child **width-only**, so a greedy
    /// editor falls straight through to `RichTextEditor`'s
    /// `proposal.height.unwrap_or(100.0)` fallback and never grows with the
    /// prose.
    ///
    /// **What this can and cannot see.** The editor shapes its document in
    /// `paint` (`engine.layout_full`), not `layout_response`, so headless
    /// layout always reports the `min_lines` floor rather than the real
    /// content height. What is provable here is the *mode*: an intrinsic
    /// editor reports one line, a greedy one reports the 100 px fallback.
    #[test]
    fn the_preview_editor_is_intrinsic_not_the_greedy_100px_fallback() {
        let h = editor_body_height(40, 400.0);
        let line = one_line();
        assert!(
            line > 0.0,
            "the embedded font must report a line height, got {line:.1}px"
        );
        assert!(
            (h - line).abs() < 1.0,
            "the preview editor must measure one line ({line:.1}px) under headless \
             layout — the `min_lines(1)` floor of an intrinsic editor. Got {h:.1}px; \
             ~100px means it is greedy again and `CenterColumnFlowing`'s width-only \
             proposal pinned it to the fallback."
        );
    }

    /// The editor must not take its height from the *dock*: it is intrinsic, and the
    /// dock's height only decides how much of it the outer `ScrollArea` shows. If the
    /// editor tracked the dock instead, a short band would clip the scene rather than
    /// scroll it.
    #[test]
    fn the_preview_editor_does_not_follow_the_dock_height() {
        let tall = editor_body_height(40, 800.0);
        let short = editor_body_height(40, 120.0);
        assert!(
            (tall - short).abs() < 1.0,
            "the previewed scene measures {tall:.1}px in an 800px dock but {short:.1}px \
             in a 120px one — the editor must size to its content, and the ScrollArea \
             scroll it"
        );
    }

    /// The band's scrolling shell, on the other hand, *must* fill the dock — that is
    /// what turns an over-tall editor into something the writer can scroll rather
    /// than a clipped stub.
    ///
    /// Everything but the margin lane's own column, which sits beside the band and
    /// takes its declared width off the prose side. That is the one deduction, and
    /// it is asserted rather than tolerated: a strip that took any more than its
    /// width would be narrowing the previewed prose.
    #[test]
    fn the_band_fills_the_dock_so_a_tall_scene_can_scroll() {
        let (ctx, vm) = vm_previewing(40);
        let mut tree = crate::test_support::tree_with_settings(&ctx);
        let root = tree.add(PreviewBody::new(
            vm,
            FormatViewModel::detached(),
            crate::writing_session::WritingGamesViewModel::detached(),
        ));
        tree.layout(SizeProposal::exact(900.0, 400.0));
        let scroll = find(&tree, root, "ScrollArea").expect("the preview band scrolls");
        let bounds = tree.bounds(scroll);
        let lane = find(&tree, root, "MarginLane").expect("the band carries a lane");
        let lane_width = tree.bounds(lane).width;
        assert!(
            (lane_width - crate::widgets::DEFAULT_LANE_WIDTH).abs() < 0.01,
            "the lane took {lane_width:.1} px, not its declared width"
        );
        assert!(
            (bounds.height - 400.0).abs() < 1.0
                && (bounds.width - (900.0 - lane_width)).abs() < 1.0,
            "the ScrollArea must fill the dock less the lane, got {:.1}x{:.1}",
            bounds.width,
            bounds.height
        );
    }

    /// **The lane over the band maps the previewed document**, and the query it
    /// marks is the project search's own — published from here, which is where that
    /// search is being used.
    #[test]
    fn the_bands_lane_marks_what_the_project_search_found() {
        let (ctx, vm) = vm_previewing(40);
        let _providers = crate::margin_lane::install_builtin_providers();
        // A word the fixture's prose actually repeats, once per paragraph.
        vm.query_signal().set("rain".into());

        let mut tree = crate::test_support::tree_with_settings(&ctx);
        let root = tree.add(PreviewBody::new(
            vm.clone(),
            FormatViewModel::detached(),
            crate::writing_session::WritingGamesViewModel::detached(),
        ));
        // Two frames: `render` runs the editor's own text layout, and until it has
        // there is no geometry to convert an offset against.
        tree.layout(SizeProposal::exact(900.0, 400.0));
        let _ = tree.render();
        tree.layout(SizeProposal::exact(900.0, 400.0));
        let _ = tree.render();

        let published = crate::margin_lane::active_query()
            .get()
            .expect("the band publishes what the project search is looking for");
        assert_eq!(published.text, "rain");
        assert_eq!(
            published.source,
            crate::margin_lane::LaneQuerySource::Project
        );
        assert!(
            published.current_in(1).is_none(),
            "a project search stands in no one document, so it marks no current hit"
        );

        let lane_id = find(&tree, root, "MarginLane").expect("the band carries a lane");
        let marks = tree
            .widget_as_any(lane_id)
            .and_then(|a| a.downcast_ref::<crate::widgets::MarginLane>())
            .expect("MarginLane opts into as_any")
            .resolve_marks(tree.bounds(lane_id));
        assert!(
            !marks.is_empty(),
            "the previewed prose contains the query and the lane found none of it"
        );

        crate::margin_lane::set_active_query(None);
    }

    /// A footnote match — no document (`SearchReplaceViewModel::select_result`
    /// never opens one for this match kind, anchored or orphaned) — builds and lays
    /// out non-degenerately, and mounts no editor: a footnote's own body is never
    /// rendered as an editable field of the previewed document here (see
    /// `editable_field`'s own `MatchField::Footnote` arm). Regression for the dead
    /// end this dock used to be for a footnote hit.
    #[test]
    fn a_footnote_match_builds_its_own_empty_state_with_no_editor() {
        let app_ctx = Rc::new(AppContext::new());
        let ids = AppIds::new();
        let vm = SearchReplaceViewModel::new(
            app_ctx.clone(),
            ids,
            SearchResultsModel::new(app_ctx.clone(), Signal::new(None)),
            SearchSettingsService::in_memory_default(),
            OpenDocsStore::new(app_ctx.clone()),
            DockingModel::new(),
            DockWidgetId::fresh(),
            DockWidgetId::fresh(),
        );
        vm.preview_field_signal().set(Some(MatchField::Footnote));
        // `preview_signal` stays `None` — exactly the state `select_result` leaves
        // it in for a footnote match (see that method's own comment).

        let mut tree = crate::test_support::tree_with_settings(&app_ctx);
        let root = tree.add(PreviewBody::new(
            vm,
            FormatViewModel::detached(),
            crate::writing_session::WritingGamesViewModel::detached(),
        ));
        tree.layout(SizeProposal::exact(900.0, 400.0));

        let b = tree.bounds(root);
        assert!(
            b.width > 0.0 && b.height > 0.0,
            "laid out to nothing ({b:?})"
        );
        assert!(
            find(&tree, root, "RichTextEditorBody").is_none(),
            "a footnote's own body is not editable here — the dock, not the \
             preview band, is where it is shown and edited"
        );
    }

    /// The preview band edits the **same document** a scene tab shows, so a
    /// writing game that has frozen the manuscript has to freeze it here too.
    /// Without this the band is a way to delete prose that Backspace refuses to
    /// delete two panes away — which is how the bug shipped in review.
    #[test]
    fn the_preview_band_is_frozen_by_a_writing_game() {
        use teksilo::widgets::rich_text::CommandFilter;

        let (ctx, vm) = vm_previewing(2);
        let games = crate::writing_session::WritingGamesViewModel::detached();
        games.set_always_forward(true);

        let mut tree = crate::test_support::tree_with_settings(&ctx);
        let body = PreviewBody::new(vm, FormatViewModel::detached(), games.clone());
        let root = tree.add(body);
        tree.layout(SizeProposal::exact(900.0, 400.0));

        assert!(
            find(&tree, root, "RichTextEditorBody").is_some(),
            "precondition: the band mounts an editor over the previewed document"
        );
        // The filter is pushed onto the editor at build; the game covers prose by
        // default, so the band must be forward-only rather than freely editable.
        assert_eq!(
            games.filter_for(crate::format::EditorKind::Prose),
            CommandFilter::ForwardOnly,
            "the game covers manuscript prose, which is what this band shows"
        );
    }
}
