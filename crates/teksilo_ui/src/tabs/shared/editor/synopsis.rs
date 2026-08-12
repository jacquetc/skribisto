// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The synopsis editors — the second half of every writing row.
//!
//! Every writing row owns a main text *and* a synopsis, which is what the dual
//! pane is for. Three surfaces render one: the box under the prose, the card on
//! the corkboard, and the column beside the prose when the tab is wide enough
//! (that last one's geometry lives in [`super::scroll_sync`], because it is a
//! property of the two panes together rather than of the synopsis alone).

use super::*;

/// How tall a *growing* synopsis editor starts: enough to invite a couple of lines,
/// then it grows with its content like a writing column.
pub const SYNOPSIS_MIN_LINES: u32 = 3;

/// How a synopsis editor sizes itself.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum SynopsisFit {
    /// Capped at six lines and scrolling inside its own box — the *subordinate* pane
    /// above a scene's prose, where it must not push the writing surface off screen.
    Compact,
    /// Intrinsic-height, no inner scroll bar: it grows with the text and the page's
    /// outer `ScrollArea` scrolls it — same flowing behaviour as [`writing_column`].
    /// This is what a synopsis needs wherever it *is* the writing surface: a Full
    /// Synopsis stream row, or a container's own page.
    Growing,
    /// Fills a `Splitter` pane and scrolls **inside** it — the synopsis beside the
    /// prose rather than above it (Side placement).
    ///
    /// Greedy like [`card_synopsis_editor`]: neither `min_lines` nor `max_lines`,
    /// so it consumes the exact height the pane hands it. A `Splitter` places each
    /// pane at a concrete pixel height, which is precisely the "give it an exact
    /// height, never an unbounded one" the greedy recipe requires.
    ///
    /// It also paints **nothing** of its own — no bordered box, and the editor body
    /// is transparent — so the pane's `SurfaceRole::Main` shows through and the
    /// strip reads as dock chrome rather than as a second sheet of paper laid on
    /// the manuscript.
    Side,
}

/// The bordered synopsis editor box (caller sizes/centres it). User edits flip the
/// tab's dirty flag via `on_change`. `split` adds the caret-aware "Split scene" action
/// to its context menu — in a Full Synopsis stream the synopsis is the text being cut.
// Many parameters, and each is a distinct thing this surface has to be handed:
// the document, its typography, the dirty callback, and the optional sessions
// (find, spell, replace-while-typing) it drives. Bundling them into a config
// struct would add a type whose only job is to be destructured immediately,
// and would hide which call sites opt into which session — the thing worth
// seeing at a glance. Allowed rather than restructured.
#[allow(clippy::too_many_arguments)]
pub fn synopsis_editor(
    doc: &TextDocument,
    typo: &EditorTypography,
    fit: SynopsisFit,
    on_change: impl Fn() + 'static,
    split: Option<SplitFn>,
    spell: Option<Rc<SpellSession>>,
    replacement: Option<Rc<TextReplacementSession>>,
    // Where to re-attach this editor's handle so tab-level commands can find
    // it. `None` for surfaces with no tab (the corkboard card's own editor).
    handle_sink: Option<Rc<RefCell<Option<EditorHandle>>>>,
    format: Option<FormatViewModel>,
    typewriter: Option<crate::view_models::TypewriterSettings>,
    // The ambient caret band for this surface — the shared preference plus this
    // document's language. `None` on the surfaces built without an app around
    // them (the widget tests), which draw no band.
    caret: Option<crate::view_models::CaretBand>,
    // The writing games this project is playing (currently "Always forward"),
    // which may freeze this surface while one is on. `None` on the surfaces
    // built with no app around them (the widget tests). Which surfaces a game
    // covers is the game's own decision, taken against this editor's kind.
    games: Option<crate::view_models::WritingGamesViewModel>,
    // The synopsis is a *different* `Content` row than the body, so it carries
    // its own binding — anchoring both to "the item" would merge two distinct
    // annotations into one.
    comments: Option<crate::comments::binding::CommentBinding>,
    // Where this editor fetches an image it meets but its document does not
    // have — a picture pasted in from another editor, or brought back by an
    // undo. `None` on the surfaces built without a project around them.
    images: Option<crate::view_models::images::ImageSource>,
    // Whether this surface may be typed into — see `writing_column`.
    read_only: bool,
) -> impl Widget {
    // Stand by to supply an image this document does not have. A picture
    // pasted in from another editor arrives as a reference — pixels live on the
    // document that owns them, and a clipboard fragment is not a document — so
    // without this it lays out at full size and paints nothing.
    let mut editor = if read_only {
        RichTextEditor::read_only(doc.clone())
    } else {
        RichTextEditor::editor(doc.clone())
    };
    if let Some(source) = &images {
        let resolve = source.resolver();
        editor = editor.on_image_missing(resolve);
    }
    let mut editor = editor
        .style(WritingEditorStyle)
        .on_change(on_change)
        .content_padding_symmetric(6.0, 30.0)
        .text_color(TextRole::Secondary)
        .typography_defaults(typo_defaults(typo))
        .font_size_scale(typo.size.get());
    if fit == SynopsisFit::Side {
        // Sit flush on the pane's own Main fill. Dropping the bordered `Panel`
        // below is not enough on its own: `WritingEditorStyle` paints a Content
        // rect behind the viewport too, and that rect covers nearly the whole
        // strip. This is the framework's supported way to say "no surface of your
        // own" — no second style type needed.
        editor = editor.background(SurfaceRole::Transparent);
    }
    // Re-attached on every rebuild, exactly as `writing_column` does for the
    // prose handle: a tab rebuild mints a fresh editor, so a stored handle would
    // address the one the writer *used* to be typing in.
    if let Some(sink) = &handle_sink {
        *sink.borrow_mut() = Some(editor.handle());
    }
    editor = match fit {
        SynopsisFit::Compact => editor
            .min_lines(1)
            .max_lines(6)
            .v_scroll_policy(ScrollPolicy::Auto),
        // No `max_lines` → the editor sizes to its content; its own scroll bar is
        // suppressed so the page scrolls instead. Mirrors `writing_column`, so it
        // windows the render to the visible clip too. (Compact stays self-scrolling
        // and must NOT window — its cull follows its own scroll offset.)
        SynopsisFit::Growing => editor
            .min_lines(SYNOPSIS_MIN_LINES)
            .v_scroll_policy(ScrollPolicy::AlwaysOff)
            .window_to_clip(true),
        // Greedy (no line bounds) so it takes the pane's exact height, and
        // self-scrolling because nothing outside the pane will scroll it. Not
        // `window_to_clip`: that is for an editor laid out at full document
        // height inside someone else's scroll — this one culls from its own
        // scroll offset, exactly as `Compact` does.
        SynopsisFit::Side => editor.v_scroll_policy(ScrollPolicy::Auto),
    };
    {
        let handle = editor.handle();
        let cursor = editor.cursor_position_signal();
        let doc = doc.clone();
        let spell = spell.clone();
        editor = editor.context_menu(move |pt, _ctx| {
            handle.reposition_caret_for_context_menu(pt);
            Some(Box::new(editor_context_menu(
                handle.clone(),
                cursor.clone(),
                split.clone(),
                doc.clone(),
                spell.clone(),
                comments.clone(),
            )))
        });
    }
    let mut bound = TypographyBoundEditor::new(
        editor,
        typo.clone(),
        spell,
        replacement.map(|s| (doc.clone(), s)),
        EditorKind::Synopsis,
        format,
    );
    if let Some(g) = games {
        bound = bound.with_writing_games(g);
    }
    // Only the page-sized synopsis pins. `Compact` is a six-line box with its
    // own scrollbar — holding a line at a fixed height inside it would mean
    // nothing, and would fight the box's own caret-follow.
    if let (SynopsisFit::Growing, Some(tw)) = (fit, typewriter) {
        bound = bound.with_typewriter(tw);
    }
    if let Some(band) = caret {
        bound = bound.with_caret_band(band);
    }
    // Side draws no box: the strip's own Main fill is the background, and a
    // bordered Content card on top of it would re-paper the dock chrome the
    // placement exists to match. A *transparent* Panel (rather than dropping the
    // wrapper) keeps one return type here — and paints nothing, so it also sidesteps
    // the Panel-vs-theme-override hazard the distraction-free surface documents.
    let (fill, border_width, radius) = match fit {
        SynopsisFit::Side => (SurfaceRole::Transparent, 0.0, 0.0),
        SynopsisFit::Compact | SynopsisFit::Growing => (SurfaceRole::Content, 1.0, 6.0),
    };
    teksu!(
        Panel {
            background: fill
            border_color: BorderRole::Default
            border_width: border_width
            corner_radius: radius
            child: bound
        }
    )
}

/// The corkboard card's editable synopsis: **borderless** (like the scene main
/// editor) and **bounded** — it fills the caller's box and scrolls internally
/// (`Auto`) rather than growing, so a long synopsis never overflows the fixed-height
/// card or the expand modal, and the caret stays in view while typing. Same
/// right-click menu (incl. **Split scene**) as the Full-* editors.
///
/// **Greedy on purpose:** it sets neither `min_lines` nor `max_lines`, so it
/// *consumes the proposal* — the bounded height its parent hands it — instead of
/// growing to the text's intrinsic height. That is what makes `ScrollPolicy::Auto`
/// engage: an intrinsic (`min_lines`) editor reports the whole document's height and
/// so never has an overflowing viewport to scroll or to keep the caret inside. A
/// bounded box + greedy sizing + `Auto` is the standard "fill this and scroll"
/// editor (it is the plain `RichTextEditor::editor` default).
///
/// **Give it its height with a `FixedSize`, never an `Expand`.** A greedy editor
/// only bounds when its parent *proposes an exact height* in its layout pass;
/// `Expand` measures its child with an unspecified height (a ~100 px fallback), so
/// the editor never learns the box and overflows — vertically centered, scrollbar
/// pinned. Both call sites (the card and the modal) wrap this in a `FixedSize`.
// Many parameters, and each is a distinct thing this surface has to be handed:
// the document, its typography, the dirty callback, and the optional sessions
// (find, spell, replace-while-typing) it drives. Bundling them into a config
// struct would add a type whose only job is to be destructured immediately,
// and would hide which call sites opt into which session — the thing worth
// seeing at a glance. Allowed rather than restructured.
#[allow(clippy::too_many_arguments)]
pub fn card_synopsis_editor(
    doc: TextDocument,
    typo: EditorTypography,
    on_change: impl Fn() + 'static,
    split: Option<SplitFn>,
    spell: Option<Rc<SpellSession>>,
    replacement: Option<Rc<TextReplacementSession>>,
    format: Option<FormatViewModel>,
    // The ambient caret band for this surface — the shared preference plus this
    // document's language. `None` on the surfaces built without an app around
    // them (the widget tests), which draw no band.
    caret: Option<crate::view_models::CaretBand>,
    // The writing games this project is playing (currently "Always forward"),
    // which may freeze this surface while one is on. `None` on the surfaces
    // built with no app around them (the widget tests). Which surfaces a game
    // covers is the game's own decision, taken against this editor's kind.
    games: Option<crate::view_models::WritingGamesViewModel>,
    // Where this editor fetches an image it meets but its document does not
    // have — a picture pasted in from another editor, or brought back by an
    // undo. `None` on the surfaces built without a project around them.
    images: Option<crate::view_models::images::ImageSource>,
) -> (impl Widget, EditorHandle) {
    // Stand by to supply an image this document does not have. A picture
    // pasted in from another editor arrives as a reference — pixels live on the
    // document that owns them, and a clipboard fragment is not a document — so
    // without this it lays out at full size and paints nothing.
    let mut editor = RichTextEditor::editor(doc.clone());
    if let Some(source) = &images {
        let resolve = source.resolver();
        editor = editor.on_image_missing(resolve);
    }
    let mut editor = editor
        .style(WritingEditorStyle)
        .on_change(on_change)
        .content_padding_symmetric(4.0, 8.0)
        .v_scroll_policy(ScrollPolicy::Auto)
        .typography_defaults(typo_defaults(&typo))
        .font_size_scale(typo.size.get());
    {
        let handle = editor.handle();
        let cursor = editor.cursor_position_signal();
        let doc = doc.clone();
        let spell = spell.clone();
        editor = editor.context_menu(move |pt, _ctx| {
            handle.reposition_caret_for_context_menu(pt);
            Some(Box::new(editor_context_menu(
                handle.clone(),
                cursor.clone(),
                split.clone(),
                doc.clone(),
                spell.clone(),
                // A corkboard card is a preview surface, not a writing surface —
                // it offers no comment affordances.
                None,
            )))
        });
    }
    // Taken before the editor moves into its wrappers, so a caller can put the
    // caret in it after mount — a descendant walk from outside cannot reach it
    // (see `SynopsisModal::build`).
    let handle = editor.handle();
    let mut bound = TypographyBoundEditor::new(
        editor,
        typo.clone(),
        spell,
        replacement.map(|s| (doc.clone(), s)),
        EditorKind::Synopsis,
        format,
    );
    if let Some(g) = games {
        bound = bound.with_writing_games(g);
    }
    let widget = match caret {
        Some(band) => bound.with_caret_band(band),
        None => bound,
    };
    (widget, handle)
}

/// A one-line name input bound to `field.value`, wired so an edit marks the tab dirty.
///
/// `TextInput` has no `on_change` hook and its text is a plain `Signal`, so the edit is
/// detected by watching that signal and **diffing against the loaded value** — an effect
/// that also fires on registration must not mark a freshly-opened tab dirty.
///
/// Without this, renaming a chapter in its own editor never set the dirty flag: no
/// autosave, no unsaved-changes prompt, and the edit was lost unless something else
/// happened to flush the tab.
pub fn title_input(
    field: &TitleField,
    placeholder: impl Into<LocalizedString>,
    on_change: impl Fn() + 'static,
    on_commit: impl Fn() + 'static,
) -> impl Widget {
    // `on_commit` fires on blur and on Enter — a name is also an identifier (the tree,
    // the tab and the Inspector all show it), so it must not wait for the autosave
    // debounce the way prose can.
    let commit = Rc::new(on_commit);
    let (blur, submit) = (commit.clone(), commit);
    let input = TextInput::new(field.value.clone())
        .placeholder(placeholder)
        .on_blur_fn(move |_ctx| blur())
        .on_submit_fn(move |_ctx| submit());
    DirtyOnEdit {
        input: Some(input),
        value: field.value.clone(),
        is_edited: field.edited_probe(),
        on_change: Rc::new(on_change),
        child_id: None,
    }
}

/// Wraps a title `TextInput` and reports genuine edits (see [`title_input`]).
pub(super) struct DirtyOnEdit {
    pub(super) input: Option<TextInput>,
    pub(super) value: Signal<String>,
    /// Reads back whether the field currently differs from what was loaded.
    pub(super) is_edited: Rc<dyn Fn() -> bool>,
    pub(super) on_change: Rc<dyn Fn()>,
    pub(super) child_id: Option<WidgetId>,
}

impl std::fmt::Debug for DirtyOnEdit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DirtyOnEdit").finish_non_exhaustive()
    }
}

impl Widget for DirtyOnEdit {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        let input = self.input.take().expect("DirtyOnEdit built once");
        let id = ctx.add(input);
        self.child_id = Some(id);
        let (edited, on_change) = (self.is_edited.clone(), self.on_change.clone());
        ctx.effect(&self.value, move |_| {
            if edited() {
                on_change();
            }
        });
        vec![id]
    }

    fn layout_response(&self, proposal: SizeProposal, ctx: &LayoutContext) -> LayoutResponse {
        self.child_id
            .and_then(|id| ctx.child_size(id, proposal))
            .map(LayoutResponse::from)
            .unwrap_or_else(|| proposal.resolve(0.0, 0.0).into())
    }

    fn place_children(
        &self,
        bounds: Rect,
        _proposal: SizeProposal,
        children: &mut [WidgetPlacement],
        _ctx: &LayoutContext,
    ) {
        for child in children.iter_mut() {
            child.origin = Point::new(bounds.x, bounds.y);
            child.size = bounds.size();
        }
    }

    fn children(&self) -> Vec<WidgetId> {
        self.child_id.into_iter().collect()
    }
}

/// "Synopsis" header + the **compact** synopsis box (a touch narrower than the main
/// column so it reads as subordinate) — the dual-pane editor's upper half, where it
/// sits above the prose and must not push it off screen.
///
/// Centred through [`CenterColumnFlowing`], **not** an `HStack` + `Spacer` — for the
/// same reason spelled out on [`synopsis_column`], which this had quietly drifted away
/// from. An alignment widget measures its child with an *unbounded* proposal, so the
/// `MaxSize` reported its full cap and this box stayed ~656px wide in a 300px window,
/// overhanging the tab to the right for the whole height of the scene. That overhang is
/// what the renderer then tried to stripe, and it froze the app (see [`centered`]).
//
// One parameter per independently-optional editor service (find, spell, replacement,
// format, typewriter, caret band, comments). Bundling them into a struct would only
// move the same list somewhere else while hiding which surfaces opt out of what —
// the sibling builders in this module carry the same allow for the same reason.
#[allow(clippy::too_many_arguments)]
pub fn synopsis_section(
    doc: &TextDocument,
    column_width: &Signal<f32>,
    typo: &EditorTypography,
    on_change: impl Fn() + 'static,
    spell: Option<Rc<SpellSession>>,
    replacement: Option<Rc<TextReplacementSession>>,
    // Where the built editor re-attaches its handle, so tab-level commands
    // (the format dock) can act on the synopsis the caret is actually in.
    handle_sink: Option<Rc<RefCell<Option<EditorHandle>>>>,
    format: Option<FormatViewModel>,
    // The ambient caret band for this surface — the shared preference plus this
    // document's language. `None` on the surfaces built without an app around
    // them (the widget tests), which draw no band.
    caret: Option<crate::view_models::CaretBand>,
    // The writing games this project is playing (currently "Always forward"),
    // which may freeze this surface while one is on. `None` on the surfaces
    // built with no app around them (the widget tests). Which surfaces a game
    // covers is the game's own decision, taken against this editor's kind.
    games: Option<crate::view_models::WritingGamesViewModel>,
    comments: Option<crate::comments::binding::CommentBinding>,
    // Where this editor fetches an image it meets but its document does not
    // have — a picture pasted in from another editor, or brought back by an
    // undo. `None` on the surfaces built without a project around them.
    images: Option<crate::view_models::images::ImageSource>,
    // Whether this surface may be typed into — see `writing_column`.
    read_only: bool,
) -> impl Widget {
    let synopsis_width = column_width.map(|w| (w - SYNOPSIS_WIDTH_INSET).max(0.0));
    teksu!(
        VStack {
            spacing: 5.0
            GroupHeader::new(tr!(synopsis())) {
                style: TextStyleRole::SmallBold
                color: TextRole::Secondary
            }
            child: CenterColumnFlowing::new(teksu!(
                MaxSize::width(synopsis_width.get()) {
                    max_width: synopsis_width.clone()
                    Expand::horizontal {
                        child: synopsis_editor(
                            doc,
                            typo,
                            SynopsisFit::Compact,
                            on_change,
                            Option::None,
                            spell,
                            replacement,
                            handle_sink,
                            format,
                            // Compact: a bounded six-line box, never pinned.
                            Option::None,
                            caret,
                            games,
                            comments,
                            images.clone(),
                            read_only,
                        )
                    }
                }
            ))
        }
    )
}

/// The centered, capped, **growing** synopsis column — no "Synopsis" caption (in a
/// stream it would repeat on every row, and the segment already says it).
///
/// Built exactly like [`writing_column`], and for the same reason: the editor is
/// intrinsic-sized and its own scroll bar suppressed, so it grows with the text while
/// the page's outer `ScrollArea` does the scrolling. It must go through
/// [`CenterColumnFlowing`], *not* an `HStack` + `Spacer` — an alignment widget measures
/// its child with an **unbounded** proposal, so the `MaxSize` would report its full cap
/// and the editor would never wrap or shrink to fit.
// Same shape as the builders above — see the note on `writing_column`.
#[allow(clippy::too_many_arguments)]
pub fn synopsis_column(
    doc: &TextDocument,
    column_width: &Signal<f32>,
    typo: &EditorTypography,
    on_change: impl Fn() + 'static,
    split: Option<SplitFn>,
    spell: Option<Rc<SpellSession>>,
    replacement: Option<Rc<TextReplacementSession>>,
    // Where the built editor re-attaches its handle, so tab-level commands
    // (the format dock) can act on the synopsis the caret is actually in.
    handle_sink: Option<Rc<RefCell<Option<EditorHandle>>>>,
    format: Option<FormatViewModel>,
    typewriter: Option<crate::view_models::TypewriterSettings>,
    // The ambient caret band for this surface — the shared preference plus this
    // document's language. `None` on the surfaces built without an app around
    // them (the widget tests), which draw no band.
    caret: Option<crate::view_models::CaretBand>,
    // The writing games this project is playing (currently "Always forward"),
    // which may freeze this surface while one is on. `None` on the surfaces
    // built with no app around them (the widget tests). Which surfaces a game
    // covers is the game's own decision, taken against this editor's kind.
    games: Option<crate::view_models::WritingGamesViewModel>,
    comments: Option<crate::comments::binding::CommentBinding>,
    // Where this editor fetches an image it meets but its document does not
    // have — a picture pasted in from another editor, or brought back by an
    // undo. `None` on the surfaces built without a project around them.
    images: Option<crate::view_models::images::ImageSource>,
    // Whether this surface may be typed into.
    //
    // A **construction-time** choice, not a runtime flag: `RichTextEditor` fixes
    // its read-only policy when it is built, and nothing can flip it afterwards.
    // That is why a tab is rebuilt when a trashed item is *restored* rather
    // than merely re-bound (see `EditorsViewModel::items_updated`). Trashing
    // goes the other way: the tab is closed, and this surface is then reached
    // only by opening a still-trashed item from the Trash dock.
    //
    // Read-only rather than disabled, deliberately. A trashed item's text is
    // still the writer's text: they must be able to select it, read it and copy
    // it out — the one thing a disabled surface takes away and the one thing
    // someone looking at a scene they just deleted actually wants.
    //
    // **What it covers.** The policy applies to the *input* paths: the IME
    // descriptor is left unset and drops are refused, so nothing a writer types
    // or drags reaches the text. It is **not** a policy on `EditorHandle`, whose
    // `insert_text`/`insert_djot` write straight to the cursor — that is the API
    // paste, Insert footnote and a version restore all go through, and gating it
    // here would make restoring into a trashed item impossible. A command that
    // must respect the trash has to check it itself; see the
    // `the_gate_stops_typing_and_not_the_programmatic_api` test.
    read_only: bool,
) -> CenterColumnFlowing {
    let synopsis_width = column_width.map(|w| (w - SYNOPSIS_WIDTH_INSET).max(0.0));
    CenterColumnFlowing::new(teksu!(
        MaxSize::width(synopsis_width.get()) {
            max_width: synopsis_width.clone()
            Expand::horizontal {
                child: synopsis_editor(
                    doc,
                    typo,
                    SynopsisFit::Growing,
                    on_change,
                    split,
                    spell,
                    replacement,
                    handle_sink,
                    format,
                    typewriter,
                    caret,
                    games,
                    comments,
                    images.clone(),
                    read_only,
                )
            }
        }
    ))
}
