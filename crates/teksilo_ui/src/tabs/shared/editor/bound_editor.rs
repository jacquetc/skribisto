// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Binding an editor to the settings that decide how it looks and behaves.
//!
//! `TypographyBoundEditor` is the seam between a `RichTextEditor` and the
//! per-`ProseKind` typography, the caret band and the spell checker. It exists
//! because all three are *pushed* into the editor rather than read by it, and
//! something has to notice when the settings behind them change.

use super::*;

/// The framework's non-destructive default typography from a settings bundle's
/// *current* values (font family / line height / first-line indent). Size is
/// applied separately as logical `font_size_scale` (sharp; composes with a11y).
pub(super) fn typo_defaults(typo: &EditorTypography) -> EditorTypographyDefaults {
    EditorTypographyDefaults {
        font_family: Some(typo.font_family.get()),
        line_height: typo.line_height.get(),
        first_line_indent: typo.first_line_indent.get(),
        paragraph_spacing_before: typo.para_spacing_before.get(),
        paragraph_spacing_after: typo.para_spacing_after.get(),
    }
}

/// Push `typo`'s current values onto a live editor: font / line-height / indent
/// as non-destructive defaults, size as logical font-size scale. Idempotent —
/// called on mount and on every settings change.
pub(super) fn push_typography(handle: &EditorHandle, typo: &EditorTypography) {
    handle.set_typography_defaults(typo_defaults(typo));
    handle.set_font_size_scale(typo.size.get());
}

/// Push `band`'s current scope + colour onto a live editor. Idempotent — called on mount and on
/// every change to either, the same shape [`push_typography`] has and for the same reason: the
/// whole bundle goes over whichever single field changed, so the two can never disagree.
pub(super) fn push_caret_band(handle: &EditorHandle, band: &crate::shared::CaretBand) {
    handle.set_caret_highlight(band.resolve());
}

/// Wire a prose editor's `handle` to its document's caret-aware [`SpellSession`]: feed this view's
/// focus + caret (read **live** via `EditorHandle::cursor_position()` — the caret signal lags a
/// frame behind a just-typed character, so the effects only *ping* "something changed") and drive
/// the per-frame recompute via `frame_tick`. Returns the view token (`ctx.self_id()`), which the
/// caller stores and must pass to [`SpellSession::on_blur`] when the widget is torn down (its `Drop`)
/// so a destroyed focused view doesn't pin a stale exemption for a sibling view of the same document
/// (split pane / stream row / the search-preview band). Shared by [`TypographyBoundEditor`] and the
/// Search & Replace preview editor.
pub(crate) fn wire_spell(
    ctx: &mut BuildContext,
    handle: &EditorHandle,
    spell: &Rc<SpellSession>,
) -> WidgetId {
    let token = ctx.self_id();
    // Same dormancy gate as `wire_replacements` / the rich-text frame
    // loop: pre-mounted tab content must not keep ticking spell work
    // (or pinning a caret exemption) while its Switcher page is
    // parked. Rapid tab switches without this gate left every visited
    // tab's session paying O(document) catch-up on every frame wake.
    let active = ctx.activation_signal(token);
    {
        let s = spell.clone();
        let active = active.clone();
        ctx.effect(&handle.cursor_position_signal(), move |_| {
            if active.get() {
                s.on_caret(token);
            }
        });
    }
    {
        let s = spell.clone();
        let h = handle.clone();
        let active = active.clone();
        ctx.effect(&handle.focused_signal(), move |&focused| {
            if focused && active.get() {
                let hh = h.clone();
                s.on_focus(token, Rc::new(move || hh.cursor_position()));
            } else {
                s.on_blur(token);
            }
        });
    }
    {
        let s = spell.clone();
        let active = active.clone();
        ctx.effect(&active, move |&is_active| {
            if !is_active {
                // Tab parked: drop this view as the caret source so a
                // sibling (split pane / later re-open) is not stuck
                // with a stale exemption from a dormant editor.
                s.on_blur(token);
            }
        });
    }
    {
        let s = spell.clone();
        let active = active.clone();
        let tick = ctx.frame_tick();
        ctx.effect(&tick, move |_| {
            if !active.get() {
                return;
            }
            s.tick();
        });
    }
    token
}

/// Wraps a `RichTextEditor`, keeping its per-editor-type typography live for the
/// life of the tab. Initial values are already baked onto `editor` by the caller
/// (`typography_defaults` + `font_size_scale`); this registers one `ctx.effect`
/// per settings field so a preference edit re-pushes the whole bundle through
/// the editor handle to every open tab. Six *separate* effects rather than one
/// combined `zip` signal — `zip`/`zip3` build a *derived* signal, which panics
/// on `.observe()`; the `SettingsStore` signals are mutable, so per-field
/// effects are safe.
pub(super) struct TypographyBoundEditor {
    pub(super) editor: Option<RichTextEditor>,
    pub(super) typo: EditorTypography,
    /// The caret-aware spell session for this editor's document, if spell-check applies. The
    /// editor feeds it this view's focus + caret; `None` disables the wiring (e.g. a read-only or
    /// non-prose surface).
    pub(super) spell: Option<Rc<crate::spellcheck::SpellSession>>,
    /// This document and its replace-while-typing session, when the project has a
    /// lexicon. Paired because the session reads the document it belongs to, and
    /// this wrapper is the only place holding both plus a `BuildContext`.
    pub(super) replacement: Option<(TextDocument, Rc<TextReplacementSession>)>,
    /// This editor's stable identity as the spell session's "view" — set in `build` from
    /// `ctx.self_id()`, read by `Drop` to un-focus the session when the widget is torn down.
    pub(super) token: Option<WidgetId>,
    pub(super) child_id: Option<WidgetId>,
    /// Whether this editor holds manuscript prose or a synopsis — the one thing the
    /// formatting registry cannot work out for itself. See [`EditorKind`].
    pub(super) kind: EditorKind,
    /// The formatting registry this editor announced itself to, and under which id,
    /// so `Drop` can withdraw it. `None` when no `FormatViewModel` was supplied
    /// (the widget tests, which build editors with no app around them).
    pub(super) format: Option<(FormatViewModel, WidgetId)>,
    /// This window's Format VM — used at build to register; not the live registry entry.
    pub(super) format_vm: Option<FormatViewModel>,
    /// Typewriter scrolling for this editor, when it is a full-page writing
    /// surface. `None` for the surfaces that deliberately never pin — the
    /// compact synopsis box and the corkboard cards, both small bounded boxes
    /// where holding a line at a fixed height means nothing.
    pub(super) typewriter: Option<crate::shared::TypewriterSettings>,
    /// The ambient caret band for this editor. `None` on the surfaces built with no app
    /// around them (the widget tests), which draw none.
    pub(super) caret: Option<crate::shared::CaretBand>,
    /// The handle this editor was mounted with, kept **only** so `Drop` can retire the
    /// band — see there for why nothing else may.
    pub(super) banded: Option<EditorHandle>,
    /// The writing games this project is playing, if any. Read live on every
    /// change so switching the game on reaches editors that are already mounted —
    /// which is the whole point: a writer turns it on *while looking at the page*.
    /// `None` on surfaces built with no app around them (the widget tests).
    pub(super) games: Option<crate::writing_session::WritingGamesViewModel>,
    /// This editor's footnote door plus the document it shows, for the *outward*
    /// half of the two-way link: the caret's position is reported so the dock can
    /// highlight the note the writer is standing on. `None` on every surface with
    /// no project behind it, and on every editor that is not a tab's main prose.
    pub(super) footnotes: Option<(crate::footnotes::FootnoteBinding, TextDocument)>,
    /// The `BinderItem` whose text this editor shows, announced to the formatting
    /// registry so anything needing *a named item's* editor can find it — the
    /// margin lane, which converts offsets for every row on screen at once and so
    /// cannot go through focus. `None` on the surfaces built with no project
    /// around them.
    pub(super) item: Option<common::types::EntityId>,
}

impl TypographyBoundEditor {
    pub(super) fn new(
        editor: RichTextEditor,
        typo: EditorTypography,
        spell: Option<Rc<crate::spellcheck::SpellSession>>,
        replacement: Option<(TextDocument, Rc<TextReplacementSession>)>,
        kind: EditorKind,
        format_vm: Option<FormatViewModel>,
    ) -> Self {
        Self {
            editor: Some(editor),
            typo,
            spell,
            replacement,
            token: None,
            child_id: None,
            kind,
            format: None,
            format_vm,
            typewriter: None,
            caret: None,
            banded: None,
            games: None,
            footnotes: None,
            item: None,
        }
    }

    /// Name the `BinderItem` this editor is showing. Opt-in, because the surfaces
    /// that are not showing one item's text (the search preview band, the widget
    /// tests) have no id to give.
    pub(super) fn with_item(mut self, item: common::types::EntityId) -> Self {
        self.item = Some(item);
        self
    }

    /// Let this editor be frozen by a writing game (currently "Always forward").
    ///
    /// Opt-in for the same reason the others are: a surface built with no app
    /// behind it has no game to read. Which surfaces a game *covers* is the
    /// game's own decision, taken against this editor's [`EditorKind`] — so a
    /// caller only says "this editor is part of the app", never "this editor is
    /// prose" a second time.
    pub(super) fn with_writing_games(
        mut self,
        games: crate::writing_session::WritingGamesViewModel,
    ) -> Self {
        self.games = Some(games);
        self
    }

    /// Pin this editor's caret line per the shared typewriter setting. Opt-in,
    /// because only the full-page writing surfaces want it.
    pub(super) fn with_typewriter(mut self, typewriter: crate::shared::TypewriterSettings) -> Self {
        self.typewriter = Some(typewriter);
        self
    }

    /// Shade the sentence or paragraph the caret is in, per the shared setting. Opt-in
    /// only because a surface built with no app behind it has no setting to read.
    pub(super) fn with_caret_band(mut self, caret: crate::shared::CaretBand) -> Self {
        self.caret = Some(caret);
        self
    }

    /// Report this editor's caret to the footnotes dock. Opt-in, because only a
    /// tab's main prose has a single caret worth reporting — a stream shows one
    /// editor per row, and a corkboard one per card.
    pub(super) fn with_footnotes(
        mut self,
        binding: crate::footnotes::FootnoteBinding,
        doc: TextDocument,
    ) -> Self {
        self.footnotes = Some((binding, doc));
        self
    }
}

impl Drop for TypographyBoundEditor {
    fn drop(&mut self) {
        // If this view held the spell session's caret focus, release it — otherwise a sibling
        // view of the same document (a split pane / stream row) would keep a stale caret reader
        // and pin a frozen exemption.
        if let (Some(spell), Some(token)) = (&self.spell, self.token) {
            spell.on_blur(token);
        }
        // Withdraw from the formatting registry in the same breath. Tying the
        // entry to this widget's lifetime is what keeps the registry honest: a
        // handle can never outlive the editor it addresses, so the dock and the
        // menu cannot format a stream row that has scrolled out of existence.
        if let Some((format, id)) = &self.format {
            format.unregister(*id);
        }
        // And retire the caret band, for the same reason as the spell session
        // above: the band is a range session on the **shared** document, so one
        // left behind is still painted by every other view of that document.
        //
        // Not theoretical, and not covered by the session's own `Drop`: an
        // editor state can outlive the widget that mounted it (the handle is an
        // `Rc`, and this widget is replaced — not rebuilt — on every tab
        // rebuild), and a stale state's band effects are gone, so nothing else
        // can ever reach it. Leaving distraction-free mode was where it showed:
        // the writer came back to a docked editor shaded in the
        // distraction-free theme's colour. It was invisible before that theme
        // had a band of its own, because the leftover was the same shade as the
        // band the pane draws for itself.
        if let Some(handle) = self.banded.take() {
            handle.set_caret_highlight(None);
        }
    }
}

impl std::fmt::Debug for TypographyBoundEditor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TypographyBoundEditor")
            .finish_non_exhaustive()
    }
}

impl Widget for TypographyBoundEditor {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        let editor = self
            .editor
            .take()
            .expect("TypographyBoundEditor built once");
        let handle = editor.handle();
        let id = ctx.add(editor);
        self.child_id = Some(id);
        // Ctrl+Wheel resizes the type this editor is dressed in. One handler
        // here reaches every writing surface in the app, because every one of
        // them is wrapped in this widget — and each already carries the right
        // bundle, so there is no dispatch to get wrong.
        //
        // `on_pointer_event`, NOT `on_scroll`: the framework fires the former in
        // the *preview* pass on each strict ancestor of the pointer target, and
        // the latter only in the bubble pass starting at the target itself. The
        // wheel's target is the `RichTextEditor` below us, whose own
        // `handle_scroll` discards modifiers and scrolls unconditionally — as
        // does the page `ScrollArea` above us. In bubble we would arrive after
        // one of them had already moved the page. Same reason, same shape, as
        // teksilo's own tab-bar wheel remap.
        {
            let typo = self.typo.clone();
            let mut wheel = crate::shared::editor_size::WheelAccumulator::default();
            ctx.apply_self_handlers(HandlerSet::new().on_pointer_event(
                move |event: &WidgetEvent, ctx: &mut EventContext| -> EventResponse {
                    let WidgetEvent::Scroll { delta, modifiers } = event else {
                        return EventResponse::Ignored;
                    };
                    // Super as well as Ctrl, so ⌘-wheel is the gesture on macOS
                    // without a second code path.
                    if !(modifiers.ctrl() || modifiers.super_key()) {
                        // Ignored, so an ordinary wheel falls through to the
                        // bubble pass and scrolls the page exactly as before.
                        return EventResponse::Ignored;
                    }
                    if let Some(notches) = wheel.feed(*delta) {
                        crate::shared::editor_size::step_and_announce(ctx, &typo, notches);
                    }
                    // Handled unconditionally while the modifier is down —
                    // including at both clamp limits, and on a pixel dribble too
                    // small to complete a notch. Returning `Ignored` in those
                    // cases would scroll the page precisely when the writer has
                    // reached the largest size and is still turning the wheel.
                    EventResponse::Handled
                },
            ));
        }
        // Any one field changing re-pushes the whole bundle (font/line/indent +
        // zoom) to the live editor. Separate effects — a combined `zip` signal is
        // derived and would panic on observe.
        {
            let (h, t) = (handle.clone(), self.typo.clone());
            ctx.effect(&self.typo.font_family, move |_| push_typography(&h, &t));
        }
        {
            let (h, t) = (handle.clone(), self.typo.clone());
            ctx.effect(&self.typo.size, move |_| push_typography(&h, &t));
        }
        {
            let (h, t) = (handle.clone(), self.typo.clone());
            ctx.effect(&self.typo.line_height, move |_| push_typography(&h, &t));
        }
        {
            let (h, t) = (handle.clone(), self.typo.clone());
            ctx.effect(&self.typo.first_line_indent, move |_| {
                push_typography(&h, &t)
            });
        }
        {
            let (h, t) = (handle.clone(), self.typo.clone());
            ctx.effect(&self.typo.para_spacing_before, move |_| {
                push_typography(&h, &t)
            });
        }
        {
            let (h, t) = (handle.clone(), self.typo.clone());
            ctx.effect(&self.typo.para_spacing_after, move |_| {
                push_typography(&h, &t)
            });
        }
        // Typewriter scrolling, on the same footing as typography and for the
        // same reason: two separate effects, because a combined `zip` signal is
        // derived and would panic on observe. Pushed once up front so an editor
        // built while the setting is already on pins from its first keystroke,
        // not only after the next settings change.
        if let Some(tw) = self.typewriter.clone() {
            handle.set_typewriter(tw.editor_anchor());
            {
                let (h, t) = (handle.clone(), tw.clone());
                ctx.effect(&tw.enabled, move |_| h.set_typewriter(t.editor_anchor()));
            }
            {
                let (h, t) = (handle.clone(), tw.clone());
                ctx.effect(&tw.preset, move |_| h.set_typewriter(t.editor_anchor()));
            }
        }
        // The ambient caret band, on the same footing as typography and typewriter, and for the
        // same reason: two separate effects, because a combined `zip` signal is derived and
        // would panic on observe. Pushed once up front so an editor built while the setting is
        // already on bands from its first frame, not only after the next settings change.
        if let Some(band) = self.caret.clone() {
            push_caret_band(&handle, &band);
            self.banded = Some(handle.clone());
            {
                let (h, b) = (handle.clone(), band.clone());
                ctx.effect(&band.settings.scope, move |_| push_caret_band(&h, &b));
            }
            {
                let (h, b) = (handle.clone(), band.clone());
                // The colour signal is driven by `app.rs`'s theme effect, so this is what makes
                // an open band follow a light/dark switch.
                ctx.effect(&band.settings.color, move |_| push_caret_band(&h, &b));
            }
        }
        // Writing games. Pushed once up front so an editor built while a game is
        // already being played is frozen from its first keystroke, then re-pushed
        // whenever the activation *or* either "which surfaces" option changes —
        // three separate effects, because a combined `zip` is a derived signal and
        // `observe` panics on one (the same reason typography and the typewriter
        // register per-field effects above).
        //
        // The whole rule lives in the view-model's `filter_for`: this only asks
        // "what filter should an editor of my kind be running under now?" and
        // hands the answer to teksilo, which owns the far harder half — which of
        // its commands take text away, including a same-editor drag-move and a
        // context-menu Cut that no application-level key handler would catch.
        if let Some(games) = self.games.clone() {
            let kind = self.kind;
            let push = {
                let (h, g) = (handle.clone(), games.clone());
                move || h.set_command_filter(g.filter_for(kind))
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
        // Caret-aware spell-check: feed this view's focus + caret and drive the per-frame recompute.
        if let Some(spell) = self.spell.clone() {
            self.token = Some(wire_spell(ctx, &handle, &spell));
        }
        // Replace-while-typing, on the same footing and for the same reason: the
        // work needs the handle, so it cannot run from `on_change`.
        if let Some((doc, replacement)) = self.replacement.clone() {
            wire_replacements(ctx, &handle, &doc, &replacement);
        }
        // The outward half of the footnote link: tell the dock which note the
        // caret is standing on. An effect on the caret signal rather than a click
        // callback, so arrowing onto a marker lights up its row exactly as
        // clicking it does — and so this needs nothing from the framework that
        // moving the caret does not already publish.
        if let Some((binding, doc)) = self.footnotes.clone() {
            let h = handle.clone();
            ctx.effect(&handle.cursor_position_signal(), move |&pos| {
                // Only the editor the writer is actually in may speak: a split
                // pane's other half publishes its own idle caret otherwise, and
                // the dock's highlight would flicker between the two.
                if h.focused_signal().get() {
                    binding.caret_moved(&doc, pos);
                }
            });
        }
        // Announce this editor to the formatting surfaces. Done here rather than
        // at the ~six call sites because *every* writing editor in the app is
        // wrapped in this widget — the scene tab's prose, a Full Chapter/Part/
        // Book row, a Full Synopsis row, a corkboard card — so one hook reaches
        // all of them, and the ones the per-tab resolver cannot see get found by
        // focus instead. Re-registering on rebuild re-points the entry at the
        // fresh handle, which is exactly the staleness rule this app follows for
        // handles everywhere else.
        if let Some(format) = self.format_vm.clone() {
            let self_id = ctx.self_id();
            format.register(self_id, handle.clone(), self.kind);
            // …and the bundle it is dressed in, so Ctrl+= / Ctrl+− / Ctrl+0
            // resize whichever editor holds focus. The wheel gesture above needs
            // no such announcement — it is already standing on the editor it
            // means; only the keyboard has to be told where the writer is.
            format.set_registered_typography(self_id, self.typo.clone());
            // This editor's footnote door, for the command that anchors a note
            // to the row being typed into — the `OpenDoc` minted it per field,
            // so a stream row's own row wins over its container's.
            if let Some((binding, _)) = &self.footnotes {
                format.set_registered_footnotes(self_id, binding.clone());
            }
            // Which item's text this is, for the readers that are not focus-shaped.
            if let Some(item) = self.item {
                format.set_registered_item(self_id, item);
            }
            self.format = Some((format, self_id));
        }
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
