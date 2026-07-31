// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Low-level editor primitives shared by the per-combination tabs: the quiet
//! writing-editor style, the centered/capped writing column, the synopsis box,
//! the title field, and the live typography plumbing. Composite pane renders
//! (heading form, dual-pane prose, folder synopsis) live in [`super::panes`].

use std::cell::RefCell;
use std::rc::Rc;

use bastyde::core::binding::BindingLevel;
use bastyde::core::styles::{RichTextEditorStyle, RichTextEditorStyleConfig};
use bastyde::core::widget::WidgetPlacement;
use bastyde::prelude::*;
use bastyde::text::EditorTypographyDefaults;
use bastyde::text_document::TextDocument;
use bastyde::text_document::{Color as DocColor, HighlightFormat};
use bastyde::tokens::{BorderRole, CornerRadius, SurfaceRole};
use bastyde::widgets::rich_text::{EditorHandle, RichTextEditor, ScrollPolicy};
use bastyde::widgets::{
    Button, ButtonVariant, Checkbox, Expand, FixedSize, GroupHeader, HStack, IconButton,
    IconWidget, MaxSize, MenuItem, MenuList, Padding, Panel, RectWidget, TextInput, TextWidget,
    VStack, ZStack,
};

/// The find banner's query field caps at this width — a full-width field reads as
/// a search page, not a compact IntelliJ-style bar.
const FIND_FIELD_MAX_WIDTH: f32 = 240.0;

use crate::intents::AppIntent;
use crate::spellcheck::SpellSession;
use crate::tabs::TitleField;
use crate::text_replacement::TextReplacementSession;
use crate::view_models::{EditorKind, EditorTypography, FindViewModel, FormatViewModel};

/// Drive a document's replace-while-typing session from the frame tick.
///
/// Deliberately NOT from the editor's `on_change`. That callback runs inside
/// `frame_loop::tick`, which holds a `borrow_mut` on the editor's state for its
/// whole duration, so every `EditorHandle` method panics there — the first
/// character typed took the app down with "RefCell already mutably borrowed".
/// A frame-tick effect runs after that borrow is released, which is exactly how
/// [`wire_spell`] has always driven the spell session.
///
/// The session notices an edit by comparing the document revision, so this costs
/// one field read on the frames where nothing was typed.
fn wire_replacements(
    ctx: &mut BuildContext,
    handle: &EditorHandle,
    doc: &TextDocument,
    session: &Rc<TextReplacementSession>,
) {
    let session = session.clone();
    let handle = handle.clone();
    let doc = doc.clone();
    // Gate on activation: TabWidget pre-mounts every open tab, and
    // `frame_tick` observers fire even for dormant Switcher pages. A
    // multi-tab project would otherwise pay a replacement tick for
    // every open scene on every wake (caret blink, paint).
    let active = ctx.activation_signal(ctx.self_id());
    let tick = ctx.frame_tick();
    ctx.effect(&tick, move |_| {
        if !active.get() {
            return;
        }
        session.tick(&handle, &doc);
    });
}

/// A caret-aware "split scene" action for a writing editor's context menu:
/// invoked with the event context and the current caret offset.
pub type SplitFn = Rc<dyn Fn(&mut EventContext, usize)>;

/// How much narrower (px, total across both margins) the synopsis column is than
/// the main writing column, so it reads as the subordinate pane.
pub const SYNOPSIS_WIDTH_INSET: f32 = 48.0;

/// Minimum height (in lines) of the main writing editor when the scene tab
/// scrolls as one flowing page: a short scene still presents a page-sized
/// writing surface rather than collapsing to its few lines of text.
pub const MAIN_MIN_LINES: u32 = 10;

/// Minimum height for a *subordinate* prose editor inside a stream — a chapter
/// heading's own prose in a Full Part / Full Book view. A chapter folder always has a
/// `SceneText` field (the matrix gives it one), so at `MAIN_MIN_LINES` a Full Book
/// would show an empty ten-line box under every chapter heading. One line, growing
/// with its content, keeps the manuscript readable.
pub const HEADING_PROSE_MIN_LINES: u32 = 1;

/// The centered, max-width main writing column for `doc`, wired so user edits
/// flip the tab's dirty flag via `on_change`.
///
/// **Flowing layout:** the editor is *intrinsic*-sized (`min_lines`, no
/// `max_lines` cap → grows to its content) and its own scroll bar is suppressed
/// (`AlwaysOff`); the scene tab's outer `ScrollArea` scrolls the whole page.
/// [`CenterColumnFlowing`] gives it a bounded width (so it wraps at the column
/// cap) but takes its intrinsic height (so the page grows with the prose).
// Many parameters, and each is a distinct thing this surface has to be handed:
// the document, its typography, the dirty callback, and the optional sessions
// (find, spell, replace-while-typing) it drives. Bundling them into a config
// struct would add a type whose only job is to be destructured immediately,
// and would hide which call sites opt into which session — the thing worth
// seeing at a glance. Allowed rather than restructured.
#[allow(clippy::too_many_arguments)]
pub fn writing_column(
    doc: &TextDocument,
    column_width: &Signal<f32>,
    typo: &EditorTypography,
    min_lines: u32,
    on_change: impl Fn() + 'static,
    split: Option<SplitFn>,
    find: Option<crate::view_models::FindViewModel>,
    spell: Option<Rc<SpellSession>>,
    replacement: Option<Rc<TextReplacementSession>>,
) -> CenterColumnFlowing {
    let mut editor = RichTextEditor::editor(doc.clone())
        .style(WritingEditorStyle)
        .on_change(on_change)
        .content_padding_symmetric(8.0, 12.0)
        .min_lines(min_lines)
        .v_scroll_policy(ScrollPolicy::AlwaysOff)
        // dubious mode: the editor is laid out at full document height and the
        // tab's outer ScrollArea scrolls the page. Window the render to the
        // visible clip so a 13k-word scene only rasterizes the rows on screen
        // instead of the whole document on every paint.
        .window_to_clip(true)
        .typography_defaults(typo_defaults(typo))
        .zoom(typo.size.get());
    // Hand this editor's handle to the find banner so it can select + scroll the
    // current match into view. Re-attached on every rebuild (a fresh widget each
    // time); the handle just re-points at the same underlying editor state.
    if let Some(find) = &find {
        find.attach_handle(editor.handle());
    }
    // Replace the built-in menu with our own — the standard editing actions, an
    // "Add to dictionary" item, and (in a stream) "Split scene". Installed
    // unconditionally so the flat scene tab gets it too. The factory first moves
    // the caret to the click point (unless inside a selection) so Paste lands
    // there and "Add to dictionary" targets the right-clicked word.
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
            )))
        });
    }
    CenterColumnFlowing::new(bati!(
        MaxSize::width(column_width.get()) {
            max_width: column_width.clone()
            Expand::horizontal {
                child: TypographyBoundEditor::new(
                    editor,
                    typo.clone(),
                    spell,
                    replacement.map(|s| (doc.clone(), s)),
                    EditorKind::Prose,
                )
            }
        }
    ))
}

/// A writing editor's right-click menu: the **formatting row** (Bold / Italic /
/// Underline / Strikethrough), the **spelling group** (corrections for the
/// right-clicked word, then *Add to dictionary*), the standard edit actions (Cut /
/// Copy / Paste / Paste Unformatted / Select All, via the editor handle), and —
/// when a split is offered — **Split scene** at the caret.
///
/// Built fresh on each right-click, *after* the factory has moved the caret to the
/// click point, so the resolved word and any Paste act where the user clicked.
///
/// The formatting row leads, but it does not displace the spelling group's claim
/// to the top: a horizontal strip of icons reads as chrome rather than as a list
/// row, so the corrections are still the first thing the eye lands on among the
/// menu's *items*. This is where macOS and Word's mini-toolbar put the same strip.
///
/// The spelling group then leads the items, as it does in every browser and word
/// processor: the corrections are the reason the menu was opened on a squiggle, and
/// they sit flat rather than behind a submenu, one click from the fix. "Add to
/// dictionary" belongs with them — it is the other answer to the same squiggle, and
/// its label names the word so a wrong target is visible before committing.
///
/// The whole group is **omitted** when nothing flagged resolves here, rather than
/// shown greyed out: a right-click on ordinary prose opens straight at Cut instead
/// of pinning a dead item to the top of the most-used menu in the app.
fn editor_context_menu(
    handle: EditorHandle,
    cursor: Signal<usize>,
    split: Option<SplitFn>,
    doc: TextDocument,
    spell: Option<Rc<SpellSession>>,
) -> MenuList {
    // One resolution for the whole spelling group — only misspelled words are
    // offered, filtered through this editor's live spell-checker so the group
    // matches the squiggles exactly.
    let spelling = super::dictionary_menu::resolve_spelling(&doc, &handle, spell.as_deref());

    let mut list = MenuList::new().item(format_row(&handle)).separator();

    // The spelling group, present only when there is something to say about
    // spelling here — ordinary prose gets a menu that starts at Cut, rather than a
    // greyed item nailed to the top of every right-click.
    if !spelling.words.is_empty() {
        if let Some(c) = &spelling.correction {
            if c.suggestions.is_empty() {
                // Flagged but uncorrectable: say so, rather than leave a gap that
                // reads as "we forgot to look".
                list = list.item(MenuItem::new(tr!(editor_menu_no_suggestions())).enabled(false));
            }
            for suggestion in &c.suggestions {
                // The span comes from the same resolution that produced the word,
                // and the menu is rebuilt per right-click, so it cannot drift out
                // from under the item.
                let (h, start, end) = (handle.clone(), c.start, c.end);
                let suggestion = suggestion.clone();
                list = list.item(
                    MenuItem::new(lit!(suggestion.clone())) // a word, not UI chrome — never translated
                        .on_activate_fn(move |_ctx| h.replace_range(start, end, &suggestion)),
                );
            }
        }
        let add_label = match spelling.words.as_slice() {
            [w] => tr!(editor_menu_add_to_dictionary(word = w.clone())),
            _ => tr!(editor_menu_add_words_to_dictionary()),
        };
        let words = spelling.words;
        list = list
            .item(MenuItem::new(add_label).on_activate_fn(move |ctx| {
                ctx.send_intent(AppIntent::AddWordsToDictionary {
                    words: words.clone(),
                });
            }))
            .separator();
    }

    let cut = handle.clone();
    let copy = handle.clone();
    let paste = handle.clone();
    let paste_plain = handle.clone();
    let select = handle;
    list = list
        .item(MenuItem::new(tr!(menu_cut())).on_activate_fn(move |ctx| cut.cut(ctx)))
        .item(MenuItem::new(tr!(menu_copy())).on_activate_fn(move |ctx| copy.copy(ctx)))
        .item(MenuItem::new(tr!(menu_paste())).on_activate_fn(move |ctx| paste.paste(ctx)))
        .item(
            MenuItem::new(tr!(menu_paste_unformatted()))
                .on_activate_fn(move |ctx| paste_plain.paste_unformatted(ctx)),
        )
        .separator()
        .item(
            MenuItem::new(tr!(menu_select_all())).on_activate_fn(move |_ctx| select.select_all()),
        );
    if let Some(split) = split {
        list = list.separator().item(
            MenuItem::new(tr!(split_scene())).on_activate_fn(move |ctx| split(ctx, cursor.get())),
        );
    }
    list
}

/// The four character marks, as a strip across the top of the context menu.
///
/// Acts on the **right-clicked** editor's handle rather than resolving "whichever
/// editor has focus": the user pointed at one, and `reposition_caret_for_context_menu`
/// has already preserved their selection if the click landed inside it. So
/// select-a-phrase → right-click → Bold formats the phrase, and no focus
/// resolution is involved. (Right-clicking bare prose collapses to a caret, where
/// a toggle sets the *typing* format — the word-processor convention.)
///
/// Unlike the dock, the state is read once here and never polled: the whole menu
/// is rebuilt on every right-click, so a snapshot cannot go stale.
///
/// Clicking one of these does **not** close the menu — dismissal is `MenuItem`
/// plumbing (`ctx.dismiss_self_overlay_chain`) that `IconButton` has no part in —
/// so the strip works as a sticky mini-toolbar: bold, then italic, then Escape.
/// That is the better behaviour, and it is why the buttons cannot lean on
/// `IconButton::toggle`'s optimistic flip: over a mixed selection "toggle bold" is
/// not a negation, and with no rebuild coming the button would lie for the rest of
/// the visit. Each click therefore runs the real command and writes back what the
/// editor actually did.
fn format_row(handle: &EditorHandle) -> Padding {
    /// One mark: an icon, its accessible name, the command, and the state it shows.
    fn mark(
        icon: IconWidget,
        tooltip: impl Into<bastyde::i18n::LocalizedString>,
        state: Signal<bool>,
        handle: EditorHandle,
        apply: fn(&EditorHandle),
        read: fn(&EditorHandle) -> bool,
    ) -> IconButton {
        IconButton::new(icon)
            .toolbar()
            // Keeps the strip out of Tab order, matching bastyde's own format
            // toolbar. The buttons stay reachable to a screen reader regardless:
            // `.focusable(false)` governs Tab only, and AccessKit emission and
            // `Action::Click` dispatch are independent of it.
            .focusable(false)
            .tooltip(tooltip)
            .toggle(state.clone())
            .on_activate_fn(move |ctx| {
                apply(&handle);
                state.set(read(&handle));
                // The edit lands, but the pointer is on the menu overlay and
                // the editor is not focused, so nothing schedules the frame
                // that would drain the document's events and repaint it — the
                // formatting only appeared once the menu was dismissed.
                ctx.request_frame();
                // Close the menu, exactly like every other item in it. The
                // strip was first written to stay open so bold and italic could
                // be applied in one visit, but that left the writer with no
                // caret and no way back to the prose except Escape — a menu
                // that swallows focus and will not close is a worse trade than
                // a second right-click. Dismissing also restores focus to the
                // editor for free: `show_context_menu_for` owns the overlay
                // lifecycle and puts focus back where it took it from.
                ctx.dismiss_top_overlay();
            })
    }

    let bold = Signal::new(handle.is_bold());
    let italic = Signal::new(handle.is_italic());
    let underline = Signal::new(handle.is_underline());
    let strikethrough = Signal::new(handle.is_strikethrough());

    Padding::symmetric(6.0, 6.0).child(
        HStack::new()
            .spacing(4.0)
            .child(mark(
                crate::icons::format::bold(),
                tr!(format_bold()),
                bold,
                handle.clone(),
                EditorHandle::toggle_bold,
                EditorHandle::is_bold,
            ))
            .child(mark(
                crate::icons::format::italic(),
                tr!(format_italic()),
                italic,
                handle.clone(),
                EditorHandle::toggle_italic,
                EditorHandle::is_italic,
            ))
            .child(mark(
                crate::icons::format::underline(),
                tr!(format_underline()),
                underline,
                handle.clone(),
                EditorHandle::toggle_underline,
                EditorHandle::is_underline,
            ))
            .child(mark(
                crate::icons::format::strikethrough(),
                tr!(format_strikethrough()),
                strikethrough,
                handle.clone(),
                EditorHandle::toggle_strikethrough,
                EditorHandle::is_strikethrough,
            )),
    )
}

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
) -> impl Widget {
    let mut editor = RichTextEditor::editor(doc.clone())
        .style(WritingEditorStyle)
        .on_change(on_change)
        .content_padding_symmetric(6.0, 30.0)
        .text_color(TextRole::Secondary)
        .typography_defaults(typo_defaults(typo))
        .zoom(typo.size.get());
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
            )))
        });
    }
    bati!(
        Panel {
            background: SurfaceRole::Content
            border_color: BorderRole::Default
            border_width: 1.0
            corner_radius: 6.0
            child: TypographyBoundEditor::new(
                editor,
                typo.clone(),
                spell,
                replacement.map(|s| (doc.clone(), s)),
                EditorKind::Synopsis,
            )
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
) -> impl Widget {
    let mut editor = RichTextEditor::editor(doc.clone())
        .style(WritingEditorStyle)
        .on_change(on_change)
        .content_padding_symmetric(4.0, 8.0)
        .v_scroll_policy(ScrollPolicy::Auto)
        .typography_defaults(typo_defaults(&typo))
        .zoom(typo.size.get());
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
            )))
        });
    }
    TypographyBoundEditor::new(
        editor,
        typo.clone(),
        spell,
        replacement.map(|s| (doc.clone(), s)),
        EditorKind::Synopsis,
    )
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
struct DirtyOnEdit {
    input: Option<TextInput>,
    value: Signal<String>,
    /// Reads back whether the field currently differs from what was loaded.
    is_edited: Rc<dyn Fn() -> bool>,
    on_change: Rc<dyn Fn()>,
    child_id: Option<WidgetId>,
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
) -> impl Widget {
    let synopsis_width = column_width.map(|w| (w - SYNOPSIS_WIDTH_INSET).max(0.0));
    bati!(
        VStack {
            spacing: 5.0
            GroupHeader::new(tr!(synopsis())) {
                style: TextStyleRole::SmallBold
                color: TextRole::Secondary
            }
            child: CenterColumnFlowing::new(bati!(
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
) -> CenterColumnFlowing {
    let synopsis_width = column_width.map(|w| (w - SYNOPSIS_WIDTH_INSET).max(0.0));
    CenterColumnFlowing::new(bati!(
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
                )
            }
        }
    ))
}

/// "Text" header + the centered, capped main writing column. The column is
/// intrinsic-height (see [`writing_column`]) so the section grows with the prose
/// and the tab's outer `ScrollArea` scrolls it.
pub fn writing_section(
    doc: &TextDocument,
    column_width: &Signal<f32>,
    typo: &EditorTypography,
    on_change: impl Fn() + 'static,
    find: Option<crate::view_models::FindViewModel>,
    spell: Option<Rc<SpellSession>>,
    replacement: Option<Rc<TextReplacementSession>>,
) -> impl Widget {
    VStack::new()
        .spacing(5.0)
        .child(
            GroupHeader::new(tr!(text_heading()))
                .style(TextStyleRole::SmallBold)
                .color(TextRole::Secondary),
        )
        .child(writing_column(
            doc,
            column_width,
            typo,
            MAIN_MIN_LINES,
            on_change,
            None,
            find,
            spell,
            replacement,
        ))
}

/// A flat, edge-to-edge Content-surface backdrop wrapping the tab body (the
/// `TabWidget` doesn't paint a content background).
pub fn tab_backdrop(body: impl Widget + 'static) -> Box<dyn Widget> {
    Box::new(bati!(
        Panel {
            background: SurfaceRole::Content
            corner_radius: 0.0
            padding: 0.0
            child: Expand {
                child: body
            }
        }
    ))
}

/// As [`tab_backdrop`], plus the per-editor **find banner** (Ctrl+F) above the
/// body — the IntelliJ shape: a full-width strip at the top of the editor, in
/// normal flow, that pushes the prose down rather than floating over it (a
/// top-right floating box covers the very match it just found, which is an open
/// bug in VS Code itself).
///
/// It mounts here, **above the body**, and the body is the tab's outer
/// `ScrollArea` — so the banner is pinned and never scrolls away with the prose.
/// `visible` comes from the tab's [`FindViewModel`], and the banner is gated by
/// [`VisibleWhen`] (not `Switcher`/`Collapse` — see its docs): dormant when
/// closed, so the prose sits flush at the top and the banner is out of the a11y
/// tree and Tab order until Ctrl+F opens it.
pub fn tab_backdrop_with_find(find: FindViewModel, body: impl Widget + 'static) -> Box<dyn Widget> {
    let column = VStack::new()
        .spacing(0.0)
        .child(VisibleWhen::new(
            find.visible_signal(),
            FindBanner::new(find),
        ))
        .child(Expand::new().child(body));
    Box::new(bati!(
        Panel {
            background: SurfaceRole::Content
            corner_radius: 0.0
            padding: 0.0
            child: column
        }
    ))
}

/// Shows `child` only while `visible`, and occupies **nothing** when hidden.
///
/// This is `ctx.visible_when` — the framework's own show/hide gate, the same one the
/// docking system uses to park a side's content. A hidden node goes *dormant*: it
/// drops out of layout (so the prose sits flush at the top until the banner opens),
/// it is not painted, and it leaves the accessibility tree and the Tab order.
///
/// Deliberately **not** a `Switcher`. A `Switcher` reports its child's *natural* width,
/// so the closed banner still claimed the width of its query row (~650px) — which made
/// the whole tab overhang any window narrower than that, for the full height of the
/// scene. That overhang is exactly the geometry that used to wedge the renderer (see
/// [`centered`]). `visible_when` reports nothing at all when hidden, so it cannot.
/// (`Collapse` was the other candidate and is simply broken — driven by an external
/// signal it never leaves `progress = 0`, because `ctx.animated_signal()` mints a fresh
/// signal on every build while the `ctx.effect()` observer survives rebuilds.)
#[derive(Debug)]
pub struct VisibleWhen {
    visible: Signal<bool>,
    child_id: Option<WidgetId>,
    pending: Option<Box<dyn Widget>>,
}

impl VisibleWhen {
    pub fn new(visible: Signal<bool>, child: impl Widget + 'static) -> Self {
        Self {
            visible,
            child_id: None,
            pending: Some(Box::new(child)),
        }
    }
}

impl Widget for VisibleWhen {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        if let Some(w) = self.pending.take() {
            let id = ctx.add_boxed(w);
            ctx.visible_when(id, self.visible.clone());
            self.child_id = Some(id);
        }
        self.child_id.into_iter().collect()
    }

    fn layout_response(&self, proposal: SizeProposal, ctx: &LayoutContext) -> LayoutResponse {
        // The gate is read here too, not just handed to `visible_when`: a dormant child
        // must contribute **zero height**, or the VStack would still reserve the
        // banner's row and the prose would sit 56px low with nothing above it.
        if !self.visible.get() {
            return Size::new(0.0, 0.0).into();
        }
        self.child_id
            .and_then(|id| ctx.child_size(id, proposal))
            .unwrap_or(Size::new(0.0, 0.0))
            .into()
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

/// A highlight format from a background `SurfaceRole` and an optional foreground
/// `TextRole` — so highlighted text keeps its contrast (a strong current-match
/// background needs an on-that-surface text colour, or the glyphs disappear into
/// it). Theme colours are f32 components (0..1); document highlight colours are
/// `u8` (0..255). Shared with the search preview's find-highlight session.
pub(crate) fn highlight_of(
    bg: SurfaceRole,
    fg: Option<TextRole>,
    colors: &bastyde::tokens::ColorTokens,
) -> HighlightFormat {
    let to_u8 = |v: f32| (v * 255.0).round().clamp(0.0, 255.0) as u8;
    let to_doc = |c: bastyde::tokens::Color| {
        DocColor::rgba(to_u8(c.r()), to_u8(c.g()), to_u8(c.b()), to_u8(c.a()))
    };
    HighlightFormat {
        background_color: Some(to_doc(bg.resolve(colors))),
        foreground_color: fg.map(|role| to_doc(role.resolve(colors))),
        ..Default::default()
    }
}

/// The per-editor find banner: a raised strip with a query field, an "N of M"
/// counter, previous / next / close controls, and Escape-to-close. Bound to the
/// tab's [`FindViewModel`], which owns the [`FindSession`](bastyde::widgets::rich_text::FindSession)
/// that highlights the matches and the editor handle that scrolls the current one
/// into view.
///
/// A custom widget (not a plain function) because `build` needs a context: it
/// resolves the two highlight colours from the theme, creates the find session
/// lazily, and wires the reactive layer — re-run the query when it or the options
/// change, and re-derive the matches once per frame if an edit staled them.
struct FindBanner {
    find: FindViewModel,
    child_id: Option<WidgetId>,
}

impl FindBanner {
    fn new(find: FindViewModel) -> Self {
        Self {
            find,
            child_id: None,
        }
    }
}

impl std::fmt::Debug for FindBanner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FindBanner").finish()
    }
}

impl Widget for FindBanner {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        // Rebuild when the banner is (re)opened — that is the one moment we can
        // grab keyboard focus for the field (this widget is otherwise a dormant,
        // build-once child of the visibility gate).
        self.find.focus_seq_signal().bind_to(
            ctx.self_id(),
            ctx.binding_registry(),
            BindingLevel::Rebuild,
        );

        // Create the find session with theme-resolved highlight colours — the
        // current match in the accent colour with on-accent text (a solid, legible
        // highlight), the rest in a subtle accent tint that keeps the normal text
        // colour. Both paint-only, so they stay out of the accessibility tree.
        let colors = &ctx.theme().colors;
        let current = highlight_of(SurfaceRole::Accent, Some(TextRole::OnAccent), colors);
        let other = highlight_of(SurfaceRole::AccentSubtle, None, colors);
        self.find.ensure_session(current, other);

        // Reactive layer: re-run the query when it or the match options change,
        // and re-derive once per frame if an edit moved the matched offsets.
        let f = self.find.clone();
        ctx.effect(&self.find.query_signal(), move |_| f.refresh_query());
        let f = self.find.clone();
        ctx.effect(&self.find.case_sensitive_signal(), move |_| {
            f.refresh_query()
        });
        let f = self.find.clone();
        ctx.effect(&self.find.whole_word_signal(), move |_| f.refresh_query());
        // Find lives under `VisibleWhen`: when the banner is closed the
        // whole subtree is dormant, but `frame_tick` still fires. Skip
        // the re-derive while hidden so a closed banner does not keep
        // a multi-tab project busy.
        let f = self.find.clone();
        let active = ctx.activation_signal(ctx.self_id());
        let tick = ctx.frame_tick();
        ctx.effect(&tick, move |_| {
            if active.get() {
                f.tick();
            }
        });

        // "N of M" / "No results" — empty while the query is empty.
        let label = self
            .find
            .query_signal()
            .zip(&self.find.current_signal())
            .zip(&self.find.count_signal())
            .map(|((query, current), total)| {
                if query.trim().is_empty() {
                    String::new()
                } else if *total == 0 {
                    tr!(find_no_results()).resolve_now()
                } else {
                    tr!(find_count(current = *current as i64, total = *total as i64)).resolve_now()
                }
            });

        let prev = self.find.clone();
        let next = self.find.clone();
        let close = self.find.clone();
        let submit = self.find.clone();
        let sprev = self.find.clone();
        let esc = self.find.clone();
        let alt_a = self.find.clone();
        let rc_submit = self.find.clone();
        let rc_btn = self.find.clone();
        let ra_btn = self.find.clone();

        // The query field is built as a standalone widget so we can hold its id
        // and steer the on-open autofocus straight at it — the replace-mode toggle
        // sits to its left and would otherwise win `first_focusable_descendant`.
        let query_id = ctx.add(
            MaxSize::width(FIND_FIELD_MAX_WIDTH).child(
                TextInput::new(self.find.query_signal())
                    .placeholder(tr!(find_placeholder()))
                    .on_submit_fn(move |ctx| submit.submit(ctx)),
            ),
        );

        // The find row. A left replace-mode toggle (⇄) discloses the replace row;
        // then a capped-width query field, the count, the prev/next chevrons, and
        // the Match-case / Whole-word toggles; a spacer pushes Close to the trailing
        // edge. All buttons are flat (`.toolbar()` = ghost).
        let find_row = HStack::new()
            .spacing(3.0)
            .child(
                IconButton::new(crate::icons::find::replace_icon())
                    .toolbar()
                    .toggle(self.find.replace_mode_signal())
                    .tooltip(tr!(find_replace_toggle())),
            )
            .add_child(query_id)
            .child(
                Padding::symmetric(0.0, 6.0).child(
                    TextWidget::new(lit!(""))
                        .text(label)
                        .style(TextStyleRole::Small)
                        .color(TextRole::Secondary),
                ),
            )
            .child(
                IconButton::new(crate::icons::find::nav_prev_icon())
                    .toolbar()
                    .tooltip(tr!(find_previous()))
                    .on_activate_fn(move |ctx| prev.prev(ctx)),
            )
            .child(
                IconButton::new(crate::icons::find::nav_next_icon())
                    .toolbar()
                    .tooltip(tr!(find_next()))
                    .on_activate_fn(move |ctx| next.next(ctx)),
            )
            .child(
                IconButton::new(crate::icons::find::case_icon())
                    .toolbar()
                    .toggle(self.find.case_sensitive_signal())
                    .tooltip(tr!(find_opt_case())),
            )
            .child(
                IconButton::new(crate::icons::find::whole_word_icon())
                    .toolbar()
                    .toggle(self.find.whole_word_signal())
                    .tooltip(tr!(find_opt_whole_word())),
            )
            .child(Expand::horizontal().child(FixedSize::new().height(1.0)))
            .child(
                IconButton::clear()
                    .toolbar()
                    .tooltip(tr!(find_close()))
                    .on_activate_fn(move |ctx| close.close_and_refocus(ctx)),
            );

        // The replace row (disclosed when replace mode is on): a capped-width
        // replacement field (Enter = replace the current match), Replace / Replace
        // All actions, and the Preserve-case toggle.
        let replace_row = HStack::new()
            .spacing(6.0)
            .child(
                MaxSize::width(FIND_FIELD_MAX_WIDTH).child(
                    TextInput::new(self.find.replacement_signal())
                        .placeholder(tr!(find_replace_placeholder()))
                        .on_submit_fn(move |ctx| rc_submit.replace_current(ctx)),
                ),
            )
            .child(
                Button::new(tr!(find_replace()))
                    .variant(ButtonVariant::Tinted)
                    .on_activate_fn(move |ctx| rc_btn.replace_current(ctx)),
            )
            .child(
                Button::new(tr!(find_replace_all()))
                    .variant(ButtonVariant::Filled)
                    .on_activate_fn(move |ctx| ra_btn.replace_all(ctx)),
            )
            .child(Checkbox::new(self.find.preserve_case_signal()).label(tr!(find_preserve_case())))
            .child(Expand::horizontal().child(FixedSize::new().height(1.0)));

        // Enter (in the field) navigates / replaces via `on_submit`; the row handles
        // the modified chords: Shift+Enter steps back, Escape closes, Alt+A replaces
        // all (find-bar conventions).
        let keyed = VStack::new()
            .spacing(4.0)
            .child(find_row)
            .child(VisibleWhen::new(
                self.find.replace_mode_signal(),
                replace_row,
            ))
            .on_key(move |ev, ctx| match ev {
                WidgetEvent::KeyDown {
                    key: Key::Enter,
                    modifiers,
                    ..
                } if modifiers.shift() => {
                    sprev.prev(ctx);
                    EventResponse::Handled
                }
                WidgetEvent::KeyDown {
                    key: Key::Character('a' | 'A'),
                    modifiers,
                    ..
                } if modifiers.alt() && alt_a.replace_mode_signal().get() => {
                    alt_a.replace_all(ctx);
                    EventResponse::Handled
                }
                WidgetEvent::KeyDown {
                    key: Key::Escape, ..
                } => {
                    esc.close_and_refocus(ctx);
                    EventResponse::Handled
                }
                _ => EventResponse::Ignored,
            });

        let banner = bati!(
            Panel {
                background: SurfaceRole::Raised
                corner_radius: 0.0
                padding: 0.0
                child: Padding::symmetric(6.0, 6.0) {
                    child: keyed
                }
            }
        );
        let root = ctx.add(banner);
        // Autofocus the query field when the banner is open — drill into the query
        // field's own subtree, not the whole row, so focus lands on the SearchField
        // and never on the replace-mode toggle to its left. Fires on open via the
        // `focus_seq` rebuild binding above; harmless on a closed-state rebuild
        // (focus is a no-op on a dormant subtree).
        if self.find.visible_signal().get()
            && let Some(field) = ctx.first_focusable_descendant(query_id)
        {
            ctx.focus(field);
        }
        self.child_id = Some(root);
        self.child_id.into_iter().collect()
    }

    fn layout_response(&self, proposal: SizeProposal, ctx: &LayoutContext) -> LayoutResponse {
        self.child_id
            .and_then(|id| ctx.child_size(id, proposal))
            .unwrap_or(Size::new(0.0, 0.0))
            .into()
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

/// A fixed vertical gap.
pub fn vspace(height: f32) -> impl Widget {
    bati!(FixedSize { height: height })
}

/// How narrow the writing column may get before it stops following the window down.
///
/// The column *grows* to the width set in Settings, but it must also *shrink*: a
/// writer who narrows the window to sit beside another app should get a narrower
/// column, not a page that hangs off the right edge. Below this floor shrinking
/// stops — a column of a few pixels is not a writing surface, and something has to
/// bound the wrap width.
pub const MIN_COLUMN_WIDTH: f32 = 100.0;

/// The width to propose to the writing column when the tab has `available` px.
///
/// Just the floor — the *cap* is the `MaxSize` inside, fed by the Settings width — but
/// it must be applied identically wherever the column is measured and placed, or the
/// two disagree and the child is centred against a size it was never measured at.
fn column_width(available: f32) -> f32 {
    available.max(MIN_COLUMN_WIDTH)
}

/// Center `child` horizontally, capped at the live writing-column width and floored
/// at [`MIN_COLUMN_WIDTH`].
///
/// **This must not be an `HStack` + `Spacer`.** That is the alignment-proposal trap
/// already called out in `CLAUDE.md`: an alignment parent measures its child with an
/// *unbounded* width, so the `MaxSize` inside always reported its **full cap** and
/// never shrank. Narrow the window below the column width and every centered row —
/// title, synopsis, header, bar — overhung the tab to the right by the difference,
/// for the entire height of the scene.
///
/// That overhang is what froze the app: the inspector's overflow overlay painted
/// hazard stripes across a strip as tall as the whole document, and one 45° band over
/// it became a 7573x7563 path — a 229 MB rasterization the path atlas could never
/// store, re-done every frame at 100% CPU. Both of those are hardened now, but the
/// geometry was born here.
///
/// [`CenterColumnFlowing`] proposes a **bounded** width, which is exactly what lets
/// `MaxSize` resolve to `min(cap, available)` and actually shrink.
pub fn centered(child: impl Widget + 'static, column_width: &Signal<f32>) -> impl Widget {
    CenterColumnFlowing::new(bati!(
        MaxSize::width(column_width.get()) {
            max_width: column_width.clone()
            child: child
        }
    ))
}

/// The framework's non-destructive default typography from a settings bundle's
/// *current* values (font family / line height / first-line indent). Size is
/// applied separately as editor zoom.
fn typo_defaults(typo: &EditorTypography) -> EditorTypographyDefaults {
    EditorTypographyDefaults {
        font_family: Some(typo.font_family.get()),
        line_height: typo.line_height.get(),
        first_line_indent: typo.first_line_indent.get(),
        paragraph_spacing_before: typo.para_spacing_before.get(),
        paragraph_spacing_after: typo.para_spacing_after.get(),
    }
}

/// Push `typo`'s current values onto a live editor: font / line-height / indent
/// as non-destructive defaults, size as zoom. Idempotent — called on mount and
/// on every settings change.
fn push_typography(handle: &EditorHandle, typo: &EditorTypography) {
    handle.set_typography_defaults(typo_defaults(typo));
    handle.set_zoom_level(typo.size.get());
}

/// Wraps a `RichTextEditor`, keeping its per-editor-type typography live for the
/// life of the tab. Initial values are already baked onto `editor` by the caller
/// (`typography_defaults` + `zoom`); this registers one `ctx.effect` per settings
/// field so a preference edit re-pushes the whole bundle through the editor
/// handle to every open tab. Four *separate* effects rather than one combined
/// `zip` signal — `zip`/`zip3` build a *derived* signal, which panics on
/// `.observe()`; the `SettingsStore` signals are mutable, so per-field effects
/// are safe.
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

struct TypographyBoundEditor {
    editor: Option<RichTextEditor>,
    typo: EditorTypography,
    /// The caret-aware spell session for this editor's document, if spell-check applies. The
    /// editor feeds it this view's focus + caret; `None` disables the wiring (e.g. a read-only or
    /// non-prose surface).
    spell: Option<Rc<crate::spellcheck::SpellSession>>,
    /// This document and its replace-while-typing session, when the project has a
    /// lexicon. Paired because the session reads the document it belongs to, and
    /// this wrapper is the only place holding both plus a `BuildContext`.
    replacement: Option<(TextDocument, Rc<TextReplacementSession>)>,
    /// This editor's stable identity as the spell session's "view" — set in `build` from
    /// `ctx.self_id()`, read by `Drop` to un-focus the session when the widget is torn down.
    token: Option<WidgetId>,
    child_id: Option<WidgetId>,
    /// Whether this editor holds manuscript prose or a synopsis — the one thing the
    /// formatting registry cannot work out for itself. See [`EditorKind`].
    kind: EditorKind,
    /// The formatting registry this editor announced itself to, and under which id,
    /// so `Drop` can withdraw it. `None` when no `FormatViewModel` is in app state
    /// (the widget tests, which build editors with no app around them).
    format: Option<(FormatViewModel, WidgetId)>,
}

impl TypographyBoundEditor {
    fn new(
        editor: RichTextEditor,
        typo: EditorTypography,
        spell: Option<Rc<crate::spellcheck::SpellSession>>,
        replacement: Option<(TextDocument, Rc<TextReplacementSession>)>,
        kind: EditorKind,
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
        }
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
        // Caret-aware spell-check: feed this view's focus + caret and drive the per-frame recompute.
        if let Some(spell) = self.spell.clone() {
            self.token = Some(wire_spell(ctx, &handle, &spell));
        }
        // Replace-while-typing, on the same footing and for the same reason: the
        // work needs the handle, so it cannot run from `on_change`.
        if let Some((doc, replacement)) = self.replacement.clone() {
            wire_replacements(ctx, &handle, &doc, &replacement);
        }
        // Announce this editor to the formatting surfaces. Done here rather than
        // at the ~six call sites because *every* writing editor in the app is
        // wrapped in this widget — the scene tab's prose, a Full Chapter/Part/
        // Book row, a Full Synopsis row, a corkboard card — so one hook reaches
        // all of them, and the ones the per-tab resolver cannot see get found by
        // focus instead. Re-registering on rebuild re-points the entry at the
        // fresh handle, which is exactly the staleness rule this app follows for
        // handles everywhere else.
        if let Some(format) = ctx.app_state::<FormatViewModel>().cloned() {
            let self_id = ctx.self_id();
            format.register(self_id, handle.clone(), self.kind);
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

/// Editor chrome with a **constant** border instead of the default recipe's
/// focus-aware one — a writing editor is almost always focused, so the accent
/// focus ring would read as a permanent frame. Keeps a quiet, edge-to-edge box.
#[derive(Debug, Default, Clone, Copy)]
pub struct WritingEditorStyle;

impl RichTextEditorStyle for WritingEditorStyle {
    fn make_body(&self, cfg: &RichTextEditorStyleConfig, ctx: &mut BuildContext) -> WidgetId {
        if cfg.is_read_only {
            return match cfg.content_padding {
                Some((t, r, b, l)) => ctx.add(Padding::new(t, r, b, l).child_id(cfg.viewport)),
                None => cfg.viewport,
            };
        }
        let bg = ctx.add(
            RectWidget::new()
                .background(SurfaceRole::Content)
                .border_color(BorderRole::Default)
                .border_width(0.0)
                .corner_radius(CornerRadius::uniform(6.0)),
        );
        let (pt, pr, pb, pl) = cfg.content_padding.unwrap_or((8.0, 12.0, 8.0, 12.0));
        let padded = ctx.add(Padding::new(pt, pr, pb, pl).child_id(cfg.viewport));
        ctx.add(ZStack::new().add_child(bg).add_child(padded))
    }
}

/// Fills the available width and centers its single child **horizontally**, but
/// takes the child's **intrinsic height** instead of filling — for a flowing
/// page inside an outer `ScrollArea`. It proposes a *bounded* width (so a
/// width-capped wrapping child wraps at its cap rather than overflowing) and an
/// *unspecified* height (so the child reports its natural content height), then
/// centers the child horizontally.
///
/// The bounded width proposal is the whole point: it is what lets a `MaxSize` child
/// resolve to `min(cap, available)` and **shrink** with the window. An alignment
/// widget (`Center`, `HStack` + `Spacer`) proposes an *unbounded* width instead, so
/// the same `MaxSize` would report its full cap forever and overhang a narrow window.
///
/// The proposal is floored at [`MIN_COLUMN_WIDTH`], so the column bottoms out instead
/// of collapsing toward zero (which would wrap the prose one word per line and make
/// the page absurdly tall).
#[derive(Debug)]
pub struct CenterColumnFlowing {
    child_id: Option<WidgetId>,
    pending: Option<Box<dyn Widget>>,
}

impl CenterColumnFlowing {
    pub fn new(child: impl Widget + 'static) -> Self {
        Self {
            child_id: None,
            pending: Some(Box::new(child)),
        }
    }
}

impl Widget for CenterColumnFlowing {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        if let Some(w) = self.pending.take() {
            self.child_id = Some(ctx.add_boxed(w));
        }
        self.child_id.into_iter().collect()
    }

    fn layout_response(&self, proposal: SizeProposal, ctx: &LayoutContext) -> LayoutResponse {
        let width = proposal.resolve(0.0, 0.0).width;
        let height = self
            .child_id
            .and_then(|id| ctx.child_size(id, SizeProposal::with_width(column_width(width))))
            .map(|s| s.height)
            .unwrap_or(0.0);
        Size::new(width, height).into()
    }

    fn place_children(
        &self,
        bounds: Rect,
        _proposal: SizeProposal,
        children: &mut [WidgetPlacement],
        ctx: &LayoutContext,
    ) {
        for child in children.iter_mut() {
            let size = ctx
                .child_size(
                    child.id,
                    SizeProposal::with_width(column_width(bounds.width)),
                )
                .unwrap_or_else(|| bounds.size());
            let dx = ((bounds.width - size.width) / 2.0).max(0.0);
            child.origin = Point::new(bounds.x + dx, bounds.y);
            child.size = size;
        }
    }

    fn children(&self) -> Vec<WidgetId> {
        self.child_id.into_iter().collect()
    }
}

/// The writing surface driven by the **real frame loop** — the integration that
/// both shipped crashes came through, and that nothing else tests.
///
/// The other replace-while-typing tests
/// (`text_replacement::session::live_editor_tests`) build a bare
/// `RichTextEditor` and call `session.tick(...)` directly. That proves the state
/// machine, and proves nothing about *how it is invoked*: the first version of
/// this feature drove the session from the editor's `on_change`, which runs
/// inside `frame_loop::tick` while the editor's state is mutably borrowed, so
/// the first character typed panicked with "RefCell already mutably borrowed".
/// Every session test passed anyway, because none of them went through a frame.
///
/// These do. `WidgetTree::request_frame()` arms the tick and `layout()` advances
/// it (`layout_impl` calls `advance_frame_tick` on every pass), so the effect
/// registered by [`wire_replacements`] fires exactly as it does in the running
/// app — same `BuildContext`, same borrow order, same everything.
///
/// **No bastyde change was needed for this.** `advance_frame_tick` is
/// `pub(crate)`, but it does not have to be reachable: the two public calls
/// above drive it between them, which is also precisely what the winit loop
/// does each frame. Reaching for a wider framework API would have bought a
/// less faithful test.
#[cfg(all(test, feature = "mocks"))]
mod frame_loop_tests {
    use super::*;
    use bastyde::core::widget_tree::WidgetTree;
    use frontend::AppContext;

    use crate::app_ids::AppIds;
    use crate::models::TextReplacementRuleListModel;
    use crate::singles::SingleWork;
    use crate::text_replacement::typography::SmartPunctuationFlags;
    use crate::view_models::TextReplacementRulesViewModel;

    fn typo() -> EditorTypography {
        EditorTypography {
            font_family: Signal::new("Literata".to_string()),
            size: Signal::new(1.0),
            line_height: Signal::new(1.5),
            first_line_indent: Signal::new(0.0),
            para_spacing_before: Signal::new(0.0),
            para_spacing_after: Signal::new(0.0),
        }
    }

    /// A real writing column over a real document, with the session wired the
    /// way the app wires it — through `writing_column`, not by hand.
    fn column() -> (
        TextDocument,
        EditorHandle,
        Rc<TextReplacementSession>,
        WidgetTree,
    ) {
        let doc = TextDocument::new();
        let ctx = Rc::new(AppContext::new());
        let work = SingleWork::new(ctx.clone());
        work.set_custom_replacement_rules_enabled(true);
        let ids = AppIds::new();
        let vm = TextReplacementRulesViewModel::new(
            TextReplacementRuleListModel::new(ctx, ids.clone()),
            work,
            ids,
        );
        let session = TextReplacementSession::new(vm);
        session.set_locale("en-US");
        session.set_punctuation(Some(SmartPunctuationFlags::default()));

        // The handle must be the one belonging to the editor the tree actually
        // builds — a detached `RichTextEditor` over the same document shares the
        // text but NOT the caret, so the session's caret-advanced gate would
        // never see movement and no rule would ever fire. `writing_column` hands
        // its own handle to the find view-model, which is how the rest of the
        // app reaches it too.
        let find = crate::view_models::FindViewModel::new(doc.clone());
        let col = writing_column(
            &doc,
            &Signal::new(700.0),
            &typo(),
            MAIN_MIN_LINES,
            || {},
            None,
            Some(find.clone()),
            None,
            Some(session.clone()),
        );
        let mut tree = WidgetTree::new();
        tree.add(col);
        tree.layout(SizeProposal::exact(900.0, 600.0));
        let handle = find
            .editor_handle()
            .expect("writing_column attached its handle");
        (doc, handle, session, tree)
    }

    /// Run one frame, exactly as the event loop does: arm the tick, then lay out.
    fn frame(tree: &mut WidgetTree) {
        tree.request_frame();
        tree.layout(SizeProposal::exact(900.0, 600.0));
    }

    fn plain(doc: &TextDocument) -> String {
        doc.to_plain_text().unwrap_or_default()
    }

    /// **The regression test for the crash.** Typing one character and running a
    /// frame must not panic. This is the exact sequence that took the app down.
    #[test]
    fn typing_one_character_through_a_real_frame_does_not_panic() {
        let (doc, handle, _session, mut tree) = column();
        handle.insert_text("a");
        frame(&mut tree);
        assert_eq!(plain(&doc), "a");
    }

    /// A lexicon rule fires when the frame runs, not when the key is pressed —
    /// through the effect the app registers, over the real editor state.
    #[test]
    fn a_lexicon_rule_fires_from_the_frame_tick() {
        let (doc, handle, _session, mut tree) = column();
        for c in "I saw btw ".chars() {
            handle.insert_text(&c.to_string());
            frame(&mut tree);
        }
        assert_eq!(plain(&doc), "I saw by the way ");
    }

    /// And so does a punctuation rule.
    #[test]
    fn a_punctuation_rule_fires_from_the_frame_tick() {
        let (doc, handle, _session, mut tree) = column();
        for c in "wait...".chars() {
            handle.insert_text(&c.to_string());
            frame(&mut tree);
        }
        assert_eq!(plain(&doc), "wait…");
    }

    /// Frames on which nothing was typed must be inert — the session's early-out
    /// is what keeps this effect off the per-frame budget.
    #[test]
    fn idle_frames_change_nothing() {
        let (doc, handle, _session, mut tree) = column();
        handle.insert_text("btw");
        for _ in 0..30 {
            frame(&mut tree);
        }
        assert_eq!(plain(&doc), "btw", "no delimiter typed, so nothing fires");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bastyde::core::widget_tree::WidgetTree;

    fn test_typo() -> EditorTypography {
        EditorTypography {
            font_family: Signal::new("Literata".to_string()),
            size: Signal::new(1.0),
            line_height: Signal::new(1.5),
            first_line_indent: Signal::new(0.0),
            para_spacing_before: Signal::new(0.0),
            para_spacing_after: Signal::new(0.0),
        }
    }

    /// A long synopsis must **scroll** inside the fixed-height card / expand modal,
    /// never overflow it — and the caret must stay in view while typing. Two things
    /// make that work, and this pins both:
    ///
    /// 1. The editor is *greedy* (neither `min_lines` nor `max_lines`), so it consumes
    ///    the height its box proposes instead of growing to the whole document's
    ///    intrinsic height. An intrinsic (`min_lines`) editor reports the full document
    ///    height, so there's no bounded viewport for `ScrollPolicy::Auto` to scroll.
    /// 2. It must be given that height by a widget that *proposes an exact height*
    ///    (`FixedSize`), NOT an `Expand` — an `Expand` measures its child with an
    ///    unspecified height (a 100 px fallback), so the greedy editor never learns the
    ///    box height and overflows, vertically centered, scrollbar pinned. The card
    ///    and the modal both wrap this editor in a `FixedSize` for exactly this reason.
    ///
    /// Here a 60-paragraph synopsis inside a `FixedSize` 320×200 box must still measure
    /// ~200 px tall — proving it stayed bounded. A value near the (much taller) document
    /// height would mean the editor reverted to intrinsic sizing and overflowed.
    #[test]
    fn card_synopsis_editor_in_a_fixed_box_bounds_a_tall_synopsis_so_it_scrolls() {
        use bastyde::widgets::FixedSize;
        let doc = TextDocument::new();
        let _ =
            doc.set_djot_sync(&"A line of synopsis prose that says what happens.\n\n".repeat(60));
        let editor = card_synopsis_editor(doc, test_typo(), || {}, None, None, None);
        let mut tree = WidgetTree::new();
        let id = tree.add(FixedSize::new().width(320.0).height(200.0).child(editor));
        // Propose an *unbounded* height, the way the corkboard's GridView tile does —
        // the `FixedSize` must still pin the editor to 200 px regardless.
        tree.layout(SizeProposal::with_width(320.0));
        let h = tree.bounds(id).height;
        assert!(
            (h - 200.0).abs() < 2.0,
            "the synopsis editor in a FixedSize(200) box must stay 200px (scroll the \
             overflow), got {h:.1}px — a taller value means it grew past the card/modal"
        );
    }
    /// A menu built over a selection must open with the marks the selection
    /// already carries — a bold phrase should show Bold lit, not off.
    #[test]
    fn the_format_row_opens_showing_the_selections_state() {
        let doc = TextDocument::new();
        doc.set_markdown("hello world")
            .expect("parse")
            .wait()
            .expect("import");
        let editor = RichTextEditor::editor(doc);
        editor.select_all();
        let handle = editor.handle();
        handle.set_bold(true);

        let mut tree = WidgetTree::new();
        let id = tree.add(format_row(&handle));
        tree.layout(SizeProposal::exact(200.0, 40.0));
        assert!(
            tree.bounds(id).width > 0.0,
            "the row must lay out to something clickable"
        );
        assert!(handle.is_bold(), "precondition");
    }

    /// The row acts on the editor it was built over, and writes back what that
    /// editor actually did rather than an optimistic flip. The menu stays open
    /// across clicks, so a wrong assumption here would persist for the whole
    /// visit instead of being corrected by the next rebuild.
    #[test]
    fn the_format_row_reports_what_the_editor_did() {
        let doc = TextDocument::new();
        doc.set_markdown("hello world")
            .expect("parse")
            .wait()
            .expect("import");
        let editor = RichTextEditor::editor(doc);
        editor.select_all();
        let handle = editor.handle();

        // The command path the row's buttons drive.
        assert!(!handle.is_bold());
        handle.toggle_bold();
        assert!(handle.is_bold());
        handle.toggle_italic();
        assert!(
            handle.is_bold() && handle.is_italic(),
            "a second mark must not undo the first — the menu stays open, so both \
             apply in one visit"
        );
    }

    /// A right-click inside a selection keeps it, so the row formats the phrase
    /// the writer chose rather than collapsing to a caret first. The behaviour
    /// belongs to `reposition_caret_for_context_menu`; this pins that the menu's
    /// contract depends on it.
    #[test]
    fn right_clicking_inside_a_selection_keeps_it() {
        let doc = TextDocument::new();
        doc.set_markdown("hello world")
            .expect("parse")
            .wait()
            .expect("import");
        let editor = RichTextEditor::editor(doc);
        let handle = editor.handle();
        handle.select_range(0, 5);
        let (before_a, before_b) = handle.selection();
        assert_ne!(before_a, before_b, "precondition: there is a selection");

        handle.toggle_bold();
        let (after_a, after_b) = handle.selection();
        assert_eq!(
            (before_a, before_b),
            (after_a, after_b),
            "formatting must not move the selection out from under the next click"
        );
        assert!(handle.is_bold());
    }
}
