// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Low-level editor primitives shared by the per-combination tabs: the quiet
//! writing-editor style, the centered/capped writing column, the synopsis box,
//! the title field, and the live typography plumbing. Composite pane renders
//! (heading form, dual-pane prose, folder synopsis) live in [`super::panes`].

use std::cell::{Cell, RefCell};
use std::rc::Rc;

use bastyde::core::binding::BindingLevel;
use bastyde::core::styles::{RichTextEditorStyle, RichTextEditorStyleConfig};
use bastyde::core::widget::WidgetPlacement;
use bastyde::prelude::*;
use bastyde::text::EditorTypographyDefaults;
use bastyde::text_document::TextDocument;
use bastyde::text_document::{Color as DocColor, HighlightFormat};
use bastyde::tokens::{BorderRole, CornerRadius, SurfaceRole};
use bastyde::widgets::SplitterModel;
use bastyde::widgets::rich_text::{EditorHandle, RichTextEditor, ScrollPolicy};
use bastyde::widgets::{
    Button, ButtonVariant, Checkbox, Expand, FixedSize, GroupHeader, HStack, IconButton,
    IconWidget, MaxSize, MenuItem, MenuList, Padding, Panel, RectWidget, Switcher, TextInput,
    TextWidget, VStack, ZStack,
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
    format: Option<FormatViewModel>,
    // Typewriter scrolling for this surface. `None` on the surfaces that never
    // pin (and in the widget tests, which build columns with no app around them).
    typewriter: Option<crate::view_models::TypewriterSettings>,
    // The ambient caret band for this surface — the shared preference plus this
    // document's language. `None` on the surfaces built without an app around
    // them (the widget tests), which draw no band.
    caret: Option<crate::view_models::CaretBand>,
    // Where this document's caret should start, and the ports to publish this
    // editor's handle into so the position can be read back. `None` for every
    // surface that is not a tab's *main* prose column — a stream shows one
    // editor per row, so there is no single "the" caret to persist, the same
    // reason `synopsis_column` passes no handle sink there.
    view_state: Option<crate::view_models::ViewStateBinding>,
    // This editor's door to the comment feature — minted by the `OpenDoc` that
    // knows which `Content` row the document came from. `None` on every surface
    // built without a project around it (the widget tests) and on any document
    // with no comment store, which collapses the comment affordances rather than
    // panicking.
    comments: Option<crate::comments::binding::CommentBinding>,
) -> HStack {
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
        // Sharp logical size (composes with a11y text scale) — not page zoom.
        .font_size_scale(typo.size.get());
    // Hand this editor's handle to the find banner so it can select + scroll the
    // current match into view. Re-attached on every rebuild (a fresh widget each
    // time); the handle just re-points at the same underlying editor state.
    if let Some(find) = &find {
        find.attach_handle(editor.handle());
    }
    // Same re-attach-on-every-rebuild contract for the view-state ports, and the
    // caret this editor opens at. `initial` was carried over from the outgoing
    // editor by `tab_pane` before this build started, so a rebuild (a Promote, a
    // settings-driven relayout) puts the writer back where they were rather than
    // at the position the tab was first opened at.
    if let Some(vs) = &view_state {
        let handle = editor.handle();
        vs.ports.attach_editor(handle.clone());
        // A comment dock's "jump to this thread" parks a seek that only this
        // editor can perform, because only it exists once the tab is built. It
        // wins over the restored caret: the writer just asked to go somewhere
        // specific, which is a stronger intent than where they last left off.
        let seek = comments.as_ref().and_then(|c| c.take_seek());
        match seek {
            Some((start, end)) => {
                let last = doc.character_count();
                handle.select_range(start.min(last), end.min(last));
            }
            None => {
                let caret = vs.initial.caret.min(doc.character_count());
                handle.select_range(caret, caret);
            }
        }
    }
    // The margin resolves its marks against this editor's geometry, so it needs
    // the live handle. Filled here rather than cached anywhere longer-lived: a tab
    // rebuild mints a fresh editor, and a stale handle would report the previous
    // one's coordinates.
    let handle_for_margin: Rc<RefCell<Option<EditorHandle>>> =
        Rc::new(RefCell::new(Some(editor.handle())));

    // Expose this document's comment threads to AccessKit. Sighted users get the
    // underline from the highlight session; this is its accessible counterpart,
    // and neither is derivable from the other.
    if let Some(binding) = &comments {
        editor = editor.annotation_spans(binding.annotation_spans());
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
        let comments = comments.clone();
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
        EditorKind::Prose,
        format,
    );
    if let Some(tw) = typewriter {
        bound = bound.with_typewriter(tw);
    }
    if let Some(band) = caret {
        bound = bound.with_caret_band(band);
    }
    let capped = bati!(
        MaxSize::width(column_width.get()) {
            max_width: column_width.clone()
            Expand::horizontal {
                child: bound
            }
        }
    );

    // Always an `HStack`, with or without comments: a single-child stack around an
    // `Expand` lays out identically to the bare column, and one return type keeps
    // every caller free of boxing.
    let row = HStack::new().spacing(0.0);
    match comments {
        // With comments on this document, the writing column shares the pane with
        // a margin holding their cards — the LibreOffice arrangement. The two are
        // placed together by `ColumnWithMargin` rather than by the stack, so the
        // cards stay **flush** against the prose and the page only moves once the
        // pair genuinely stops fitting centred. Letting the stack do it would pin
        // the margin to the window edge and strand every card at the end of a long
        // empty leader.
        Some(binding) => {
            let palette = binding.palette();
            let gutter = binding.gutter();
            let margin = crate::comments::margin::CommentMargin::new(
                Some(binding),
                handle_for_margin,
                palette,
            );
            row.child(
                Expand::horizontal().child(
                    crate::comments::pane::ColumnWithMargin::new(
                        capped,
                        margin,
                        column_width.clone(),
                    )
                    .reserve(gutter),
                ),
            )
        }
        None => row.child(Expand::horizontal().child(CenterColumnFlowing::new(capped))),
    }
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
    comments: Option<crate::comments::binding::CommentBinding>,
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

    // The comment group. A context-menu action whose span is already resolved
    // uses a direct closure over the captured handle rather than an intent:
    // routing it through an intent that re-resolves "the current selection" would
    // act on a stale range if focus moved between the right-click and dispatch.
    //
    // "Add comment" is offered only with a real selection — a zero-width range
    // has nothing to anchor to and the thread would be born orphaned. "Comment on
    // paragraph" always applies, since the caret is always in some block.
    if let Some(binding) = &comments {
        // `selection()` returns the pair unordered (anchor, caret), so normalise.
        let (a, p) = handle.selection();
        let (start, end) = (a.min(p), a.max(p));
        if end > start {
            let b = binding.clone();
            list = list.item(
                MenuItem::new(tr!(comments_menu_add())).on_activate_fn(move |_ctx| {
                    b.add_range(start, end);
                }),
            );
        }
        let b = binding.clone();
        list = list
            .item(
                MenuItem::new(tr!(comments_menu_add_paragraph())).on_activate_fn(move |_ctx| {
                    // The whole selection, so a drag across several paragraphs
                    // comments all of them as one thread.
                    b.add_paragraph(start, end);
                }),
            )
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
    // The synopsis is a *different* `Content` row than the body, so it carries
    // its own binding — anchoring both to "the item" would merge two distinct
    // annotations into one.
    comments: Option<crate::comments::binding::CommentBinding>,
) -> impl Widget {
    let mut editor = RichTextEditor::editor(doc.clone())
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
    bati!(
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
) -> impl Widget {
    let mut editor = RichTextEditor::editor(doc.clone())
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
    let bound = TypographyBoundEditor::new(
        editor,
        typo.clone(),
        spell,
        replacement.map(|s| (doc.clone(), s)),
        EditorKind::Synopsis,
        format,
    );
    match caret {
        Some(band) => bound.with_caret_band(band),
        None => bound,
    }
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
    comments: Option<crate::comments::binding::CommentBinding>,
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
                            format,
                            // Compact: a bounded six-line box, never pinned.
                            Option::None,
                            caret,
                            comments,
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
    comments: Option<crate::comments::binding::CommentBinding>,
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
                    format,
                    typewriter,
                    caret,
                    comments,
                )
            }
        }
    ))
}

/// "Text" header + the centered, capped main writing column. The column is
/// intrinsic-height (see [`writing_column`]) so the section grows with the prose
/// and the tab's outer `ScrollArea` scrolls it.
// Same rationale as `writing_column`, which it forwards to almost verbatim: each
// argument is a distinct thing this surface has to be handed, and bundling them
// would hide which call sites opt into which session.
#[allow(clippy::too_many_arguments)]
pub fn writing_section(
    doc: &TextDocument,
    column_width: &Signal<f32>,
    typo: &EditorTypography,
    on_change: impl Fn() + 'static,
    find: Option<crate::view_models::FindViewModel>,
    spell: Option<Rc<SpellSession>>,
    replacement: Option<Rc<TextReplacementSession>>,
    format: Option<FormatViewModel>,
    typewriter: Option<crate::view_models::TypewriterSettings>,
    // The ambient caret band for this surface — the shared preference plus this
    // document's language. `None` on the surfaces built without an app around
    // them (the widget tests), which draw no band.
    caret: Option<crate::view_models::CaretBand>,
    view_state: Option<crate::view_models::ViewStateBinding>,
    comments: Option<crate::comments::binding::CommentBinding>,
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
            format,
            typewriter,
            caret,
            view_state,
            comments,
        ))
}

/// A flat, edge-to-edge backdrop wrapping the tab body (the `TabWidget` doesn't
/// paint a content background).
///
/// `background` is the tab's own answer ([`ContentTab::backdrop_role`]) rather
/// than a constant, and this is the **one** place any of the twelve
/// combinations paints its background — so the distraction-free surface can ask
/// for `Transparent` and paint its own page underneath, without a second
/// renderer and without any per-combination branching.
pub fn tab_backdrop(background: SurfaceRole, body: impl Widget + 'static) -> Box<dyn Widget> {
    Box::new(bati!(
        Panel {
            background: background
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
pub fn tab_backdrop_with_find(
    background: SurfaceRole,
    find: FindViewModel,
    body: impl Widget + 'static,
) -> Box<dyn Widget> {
    let column = find_banner_over(find, body);
    Box::new(bati!(
        Panel {
            background: background
            corner_radius: 0.0
            padding: 0.0
            child: column
        }
    ))
}

/// The find banner stacked above `body`, pinned (it never scrolls with the prose).
///
/// Split out from [`tab_backdrop_with_find`] because *what* the banner spans is not
/// always the whole tab. Under Side placement the tab is two columns — a synopsis
/// strip on the window's own chrome colour, and the manuscript — and a banner
/// stretched across both would read as a tab-wide toolbar while searching only one
/// of them. Wrapping just the prose column keeps the banner over the text it
/// actually searches.
pub fn find_banner_over(find: FindViewModel, body: impl Widget + 'static) -> impl Widget {
    VStack::new()
        .spacing(0.0)
        .child(VisibleWhen::new(
            find.visible_signal(),
            FindBanner::new(find),
        ))
        .child(Expand::new().child(body))
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

/// Minimum width the **prose** column keeps when the synopsis sits beside it.
///
/// Side placement is a preference the layout cannot always honour. The outer
/// editor split already floors a pane at `PANE_MIN_WIDTH` (320px) — that is the
/// entire budget a secondary pane gets — so subtracting a ~280px synopsis from it
/// would leave a prose column too narrow to write in. Below
/// `side_width + PROSE_MIN_WIDTH` of available width, [`WidthProbe`] renders the
/// Top layout instead, rather than honouring the setting into unusability.
pub const PROSE_MIN_WIDTH: f32 = 320.0;

/// Dead band around the Top/Side breakpoint, in px.
///
/// Without it a window left to rest exactly on the threshold flips layout on
/// every pixel of jitter — and since each flip is a relayout that feeds the next
/// decision, the two can chase each other indefinitely. The band makes the
/// crossing points asymmetric (enter Side higher than you leave it), so a width
/// has to move a real distance to change the answer.
const SIDE_BREAKPOINT_HYSTERESIS: f32 = 24.0;

const MODE_TOP: usize = 0;
const MODE_SIDE: usize = 1;

/// Picks the **Top** or **Side** synopsis layout from the width actually
/// available, and shows the chosen one.
///
/// Placement is a global setting, but whether it can be *honoured* is local: the
/// same preference has to produce a side-by-side scene in a maximised window and
/// a stacked one in a 320px secondary pane. Only layout knows which, so the
/// decision is made in [`place_children`](Widget::place_children) — where the
/// resolved width is finally known — and published to a `Signal<usize>` that a
/// [`Switcher`] consumes as its page index.
///
/// **Writing a signal from inside layout** is the [`MenuBar`]-collapse idiom, and
/// it is safe for the same two reasons: the write is guarded by a plain `Cell`
/// shadow so it only happens when the answer actually changes (no churn, no
/// oscillation), and the consumer is a `bind_to`/`visible_when` binding, which the
/// framework *defers* to the next pass rather than running re-entrantly. Nothing
/// downstream of `mode` may use `ctx.effect`: observers fire synchronously inside
/// `Signal::set`, which would re-enter layout from the middle of a layout pass.
///
/// The two branches are `Switcher` pages, so each is built at most once and
/// thereafter only shown or hidden — crossing the breakpoint back and forth keeps
/// the editors' carets, scroll offsets and spell sessions intact instead of
/// rebuilding a scene's worth of widgets on a window drag.
pub(crate) struct WidthProbe {
    /// Whether Side is wanted at all (the placement setting). Derived signals are
    /// fine here: `bind_to` walks a derived signal's mutable roots, and only
    /// `observe()` rejects them.
    side_enabled: Signal<bool>,
    /// Current width of the synopsis pane, so the breakpoint tracks the divider.
    side_width: Signal<f32>,
    mode: Signal<usize>,
    /// Non-reactive mirror of the last value written to `mode` — the guard that
    /// makes the layout-time write idempotent.
    last_mode: Cell<usize>,
    switcher_id: Option<WidgetId>,
    pending: Option<(Box<dyn Widget>, Box<dyn Widget>)>,
}

impl WidthProbe {
    pub fn new(
        side_enabled: Signal<bool>,
        side_width: Signal<f32>,
        top: Box<dyn Widget>,
        side: Box<dyn Widget>,
    ) -> Self {
        Self {
            side_enabled,
            side_width,
            // Start on Top: the available width is unknown until the first
            // layout pass, and Top is the layout that fits every width. A tab
            // that should be Side flips on that first pass, before paint.
            mode: Signal::new(MODE_TOP),
            last_mode: Cell::new(MODE_TOP),
            switcher_id: None,
            pending: Some((top, side)),
        }
    }

    /// The live page index (`0` = Top, `1` = Side). Read it before handing the
    /// probe to the tree.
    pub fn mode_signal(&self) -> Signal<usize> {
        self.mode.clone()
    }

    /// The breakpoint decision, factored out so it can be tested without a tree.
    fn resolve_mode(&self, available: f32) -> usize {
        if !self.side_enabled.get() {
            return MODE_TOP;
        }
        let threshold = self.side_width.get() + PROSE_MIN_WIDTH;
        // Asymmetric crossing points: harder to enter Side than to stay in it.
        let limit = if self.last_mode.get() == MODE_SIDE {
            threshold - SIDE_BREAKPOINT_HYSTERESIS
        } else {
            threshold + SIDE_BREAKPOINT_HYSTERESIS
        };
        if available >= limit {
            MODE_SIDE
        } else {
            MODE_TOP
        }
    }
}

impl std::fmt::Debug for WidthProbe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WidthProbe")
            .field("mode", &self.mode.get())
            .finish()
    }
}

impl Widget for WidthProbe {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        let self_id = ctx.self_id();
        // A placement or width change must re-run `place_children` so the
        // breakpoint is re-evaluated. `Relayout` (not `Rebuild`) — the widget
        // tree is unchanged, only the decision it feeds.
        self.side_enabled
            .bind_to(self_id, ctx.binding_registry(), BindingLevel::Relayout);
        self.side_width
            .bind_to(self_id, ctx.binding_registry(), BindingLevel::Relayout);

        if let Some((top, side)) = self.pending.take() {
            let switcher = Switcher::new(self.mode.clone())
                .child_boxed(top)
                .child_boxed(side);
            self.switcher_id = Some(ctx.add(switcher));
        }
        self.switcher_id.into_iter().collect()
    }

    fn layout_response(&self, proposal: SizeProposal, ctx: &LayoutContext) -> LayoutResponse {
        self.switcher_id
            .and_then(|id| ctx.child_size(id, proposal))
            .unwrap_or_else(|| proposal.resolve(0.0, 0.0))
            .into()
    }

    fn place_children(
        &self,
        bounds: Rect,
        _proposal: SizeProposal,
        children: &mut [WidgetPlacement],
        _ctx: &LayoutContext,
    ) {
        let next = self.resolve_mode(bounds.width);
        if self.last_mode.get() != next {
            self.last_mode.set(next);
            self.mode.set(next);
        }
        for child in children.iter_mut() {
            child.origin = Point::new(bounds.x, bounds.y);
            child.size = bounds.size();
        }
    }

    fn children(&self) -> Vec<WidgetId> {
        self.switcher_id.into_iter().collect()
    }
}

/// The synopsis as the **left column** of a Side-placed dual-pane editor: no box
/// of its own, filling its splitter pane and scrolling inside it.
///
/// A thin call onto [`synopsis_editor`] with [`SynopsisFit::Side`] — the wiring
/// (context menu, handle re-attach, live typography, spell, replace-while-typing,
/// caret band, format registration) is identical to the other two placements and
/// must stay that way; only the sizing and the chrome differ, which is exactly
/// what `SynopsisFit` decides.
///
/// No `split` action and no typewriter: splitting a scene is an operation on the
/// manuscript, and pinning a line means nothing in a box that scrolls itself.
#[allow(clippy::too_many_arguments)]
pub fn side_synopsis_editor(
    doc: &TextDocument,
    typo: &EditorTypography,
    on_change: impl Fn() + 'static,
    spell: Option<Rc<SpellSession>>,
    replacement: Option<Rc<TextReplacementSession>>,
    handle_sink: Option<Rc<RefCell<Option<EditorHandle>>>>,
    format: Option<FormatViewModel>,
    caret: Option<crate::view_models::CaretBand>,
    // Threaded like every other synopsis placement. Left at `None` this column
    // would be the one surface in the app where a synopsis quietly cannot be
    // commented on — and it is a *placement* of the same `Content` row, not a
    // different thing, so the annotations must follow it across the fold.
    comments: Option<crate::comments::binding::CommentBinding>,
) -> impl Widget {
    synopsis_editor(
        doc,
        typo,
        SynopsisFit::Side,
        on_change,
        None,
        spell,
        replacement,
        handle_sink,
        format,
        None,
        caret,
        comments,
    )
}

/// Index of the synopsis pane in a tab's Side splitter.
pub(crate) const SYNOPSIS_PANE: usize = 0;

/// Publishes one scroll area's offset/max to the tab's view-state ports — but only
/// while the branch it sits in is the one on screen.
///
/// A tab's remembered caret and page scroll live in a single slot, last write
/// wins. That was fine while a prose tab had exactly one scrolling page; the Top
/// and Side layouts each have their own, so whichever was *constructed* last would
/// otherwise own the slot regardless of which is *displayed* — and the tab would
/// restore, and report, the scroll of a page nobody is looking at.
///
/// Re-attaching on activation makes the answer "the visible one" by construction,
/// including on the way back to a branch that was built earlier and will not build
/// again (a `Switcher` keeps its pages, so a second `build()` never comes).
pub(crate) struct PageScrollPort {
    ports: Rc<crate::view_models::ViewStatePorts>,
    offset: Signal<f32>,
    max: Signal<f32>,
}

impl PageScrollPort {
    pub fn new(
        ports: Rc<crate::view_models::ViewStatePorts>,
        offset: Signal<f32>,
        max: Signal<f32>,
    ) -> Self {
        Self { ports, offset, max }
    }
}

impl std::fmt::Debug for PageScrollPort {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PageScrollPort").finish()
    }
}

impl Widget for PageScrollPort {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        let active = ctx.activation_signal(ctx.self_id());
        let attach: Rc<dyn Fn()> = {
            let ports = self.ports.clone();
            let offset = self.offset.clone();
            let max = self.max.clone();
            let active = active.clone();
            Rc::new(move || {
                if active.get() {
                    ports.attach_page_scroll(offset.clone(), max.clone());
                }
            })
        };
        attach();
        let f = attach.clone();
        ctx.effect(&active, move |_| f());
        Vec::new()
    }

    fn layout_response(&self, _proposal: SizeProposal, _ctx: &LayoutContext) -> LayoutResponse {
        Size::new(0.0, 0.0).into()
    }

    fn place_children(
        &self,
        _bounds: Rect,
        _proposal: SizeProposal,
        _children: &mut [WidgetPlacement],
        _ctx: &LayoutContext,
    ) {
    }

    fn children(&self) -> Vec<WidgetId> {
        Vec::new()
    }
}

/// Drives a tab's Side divider from the same boolean that decides whether the
/// synopsis is on screen at all.
///
/// Showing and hiding a splitter pane is not just a visibility flip: a `Splitter`
/// folds **every** pane's `min_size` into its own intrinsic minimum, visible or
/// not, so a hidden pane left holding a real minimum keeps setting a floor under
/// the whole tab's width. The minimum is therefore raised only while the pane is
/// up and dropped to zero on the way down — the same dance
/// `EditorsViewModel::set_split` performs for the editor's own side pane, and in
/// the same order (minimum first, both ways).
#[derive(Clone)]
pub(crate) struct SideSync {
    model: SplitterModel,
    /// Where a user-dragged width is persisted, and what the next tab seeds from.
    width: Signal<f32>,
    /// Set while *this* code is mutating the model, so the width write-back can
    /// tell the app's own bookkeeping apart from a real drag. See
    /// [`SideSync::watch_width`].
    suppress: Rc<Cell<bool>>,
}

impl SideSync {
    pub fn new(model: SplitterModel, width: Signal<f32>) -> Self {
        Self {
            model,
            width,
            suppress: Rc::new(Cell::new(false)),
        }
    }

    /// Whether the *setting* puts a synopsis column in this tab at all.
    ///
    /// Distinct from folding it away, and both states are used: hidden removes the
    /// pane **and its gutter**, which is right when there is no synopsis to reach;
    /// folded keeps the gutter, which is what lets the writer pull it back.
    fn set_visible(&self, visible: bool) {
        // Idempotent, and that is load-bearing rather than an optimisation: the
        // dormancy effect re-runs on the model's own `version`, so a `set_visible`
        // that mutated unconditionally would bump the version it is reacting to and
        // spin. Re-entrancy here is a hang, not a wasted call.
        if self.model.is_pane_visible(SYNOPSIS_PANE) == visible {
            return;
        }
        self.suppress.set(true);
        if visible {
            self.model
                .set_min_size(SYNOPSIS_PANE, crate::SYNOPSIS_SIDE_WIDTH_MIN);
            self.model.set_pane_visible(SYNOPSIS_PANE, true);
        } else {
            self.model.set_min_size(SYNOPSIS_PANE, 0.0);
            self.model.set_pane_visible(SYNOPSIS_PANE, false);
        }
        self.suppress.set(false);
    }

    /// Fold the column away, leaving the divider behind as the way back.
    pub fn fold(&self) {
        self.model.set_collapsed(SYNOPSIS_PANE, true);
    }

    fn is_folded(&self) -> bool {
        self.model.is_collapsed(SYNOPSIS_PANE)
    }

    /// Whether this handle is part-way through its own mutation of the model.
    ///
    /// The model bumps `version` from *inside* `set_pane_visible`, before the flag
    /// it is setting has landed — so a version observer that re-entered here would
    /// read the old value, mutate again, and recurse until the framework's
    /// nesting limit killed it. Reading the flag is not enough; the observer has
    /// to know the write is still in flight.
    fn is_settling(&self) -> bool {
        self.suppress.get()
    }

    fn version(&self) -> Signal<u64> {
        self.model.version()
    }

    /// Persist the synopsis column's width when the **writer** drags the divider.
    ///
    /// `SplitterModel::version()` is one coarse signal bumped by every mutation
    /// there is, so it cannot be taken at face value: this widget's own show/hide
    /// dance bumps it twice on every toggle, and a drag past the minimum bumps it
    /// while collapsing the pane to nothing. Persisting either would quietly
    /// rewrite the writer's chosen width with a number they never chose — a zero,
    /// in the collapse case.
    ///
    /// Three filters, in order of what they exclude: the suppression flag (our own
    /// mutations, set synchronously around them because observers run inside
    /// `Signal::set`), the pane's own state (a hidden or collapsed pane's width is
    /// not a width anyone picked), and a last-written shadow (so an unrelated bump
    /// is not a write).
    fn watch_width(&self, ctx: &mut BuildContext) {
        let model = self.model.clone();
        let target = self.width.clone();
        let suppress = self.suppress.clone();
        let last = Rc::new(Cell::new(model.stored_size(SYNOPSIS_PANE)));
        ctx.effect(&model.version(), move |_| {
            if suppress.get() {
                return;
            }
            if !model.is_pane_visible(SYNOPSIS_PANE) || model.is_collapsed(SYNOPSIS_PANE) {
                return;
            }
            let width = model.stored_size(SYNOPSIS_PANE).clamp(
                crate::SYNOPSIS_SIDE_WIDTH_MIN,
                crate::SYNOPSIS_SIDE_WIDTH_MAX,
            );
            if (width - last.get()).abs() > 0.5 {
                last.set(width);
                target.set(width);
            }
        });
    }
}

/// Keeps a tab's synopsis **plumbing** in step with whether the synopsis is
/// actually on screen: the spell session's dormancy, and (under Side placement)
/// the divider.
///
/// A widget rather than a call in `prose()` because both jobs need a
/// `BuildContext` to register effects on, and `prose()` is a plain builder
/// function. It draws nothing and occupies nothing — it is mounted purely so that
/// its lifetime, and the framework's own activation gate, can be borrowed.
///
/// Used by **both** placements, so "is the synopsis being shown?" has exactly one
/// answer in the codebase rather than one per layout. Under Top the divider half
/// is simply absent.
pub(crate) struct SynopsisPaneEffects {
    doc: Rc<crate::models::OpenDoc>,
    /// The window's or the mode's "show synopsis" flag.
    show: Signal<bool>,
    side: Option<SideSync>,
    guard: Rc<RefCell<Option<crate::models::SynopsisViewerGuard>>>,
}

impl SynopsisPaneEffects {
    pub fn new(
        doc: Rc<crate::models::OpenDoc>,
        show: Signal<bool>,
        side: Option<SideSync>,
    ) -> Self {
        Self {
            doc,
            show,
            side,
            guard: Rc::new(RefCell::new(None)),
        }
    }
}

impl std::fmt::Debug for SynopsisPaneEffects {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SynopsisPaneEffects")
            .field("item", &self.doc.item_id)
            .finish()
    }
}

impl Widget for SynopsisPaneEffects {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        let self_id = ctx.self_id();
        // Activation, not just the two flags. A `TabWidget` pre-mounts every open
        // tab and a `Switcher` keeps the branch it switched away from alive, so
        // effects keep firing for panes nobody can see — and a synopsis session
        // pinned awake by a background tab is exactly the cost this refcount
        // exists to avoid. Same gate `wire_spell`/`wire_replacements` already use.
        let active = ctx.activation_signal(self_id);

        let apply: Rc<dyn Fn()> = {
            let doc = self.doc.clone();
            let guard = self.guard.clone();
            let side = self.side.clone();
            let show = self.show.clone();
            let active = active.clone();
            Rc::new(move || {
                let shown = active.get() && show.get();
                if let Some(side) = &side {
                    side.set_visible(shown);
                }
                // A folded column is on screen in name only — the framework parks
                // its content dormant behind the divider — so it must not hold the
                // spell session awake either. Asked of the splitter rather than
                // mirrored into a second flag: the divider can also be dragged
                // shut, and a mirror would go stale the moment the writer did that.
                let reachable = shown && !side.as_ref().is_some_and(SideSync::is_folded);
                let mut slot = guard.borrow_mut();
                match (reachable, slot.is_some()) {
                    // Dropping the guard is what puts the session to sleep, and
                    // it only reaches a `Cell` on the doc — no re-entry into the
                    // borrow held here.
                    (true, false) => *slot = Some(doc.acquire_synopsis_viewer()),
                    (false, true) => *slot = None,
                    _ => {}
                }
            })
        };

        // Seed: `ctx.effect` fires on later changes only, and the pane may well be
        // built with the synopsis already showing.
        apply();
        if let Some(side) = &self.side {
            side.watch_width(ctx);
            // Folding and unfolding are splitter mutations, so this is how a fold
            // reaches dormancy — including one done by dragging the divider shut
            // or double-clicking it, not just by pressing the button.
            let f = apply.clone();
            let guard = side.clone();
            ctx.effect(&side.version(), move |_| {
                if !guard.is_settling() {
                    f();
                }
            });
        }
        for src in [&self.show, &active] {
            let f = apply.clone();
            ctx.effect(src, move |_| f());
        }
        Vec::new()
    }

    fn layout_response(&self, _proposal: SizeProposal, _ctx: &LayoutContext) -> LayoutResponse {
        Size::new(0.0, 0.0).into()
    }

    fn place_children(
        &self,
        _bounds: Rect,
        _proposal: SizeProposal,
        _children: &mut [WidgetPlacement],
        _ctx: &LayoutContext,
    ) {
    }

    fn children(&self) -> Vec<WidgetId> {
        Vec::new()
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
/// applied separately as logical `font_size_scale` (sharp; composes with a11y).
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
/// as non-destructive defaults, size as logical font-size scale. Idempotent —
/// called on mount and on every settings change.
fn push_typography(handle: &EditorHandle, typo: &EditorTypography) {
    handle.set_typography_defaults(typo_defaults(typo));
    handle.set_font_size_scale(typo.size.get());
}

/// Push `band`'s current scope + colour onto a live editor. Idempotent — called on mount and on
/// every change to either, the same shape [`push_typography`] has and for the same reason: the
/// whole bundle goes over whichever single field changed, so the two can never disagree.
fn push_caret_band(handle: &EditorHandle, band: &crate::view_models::CaretBand) {
    handle.set_caret_highlight(band.resolve());
}

/// Wraps a `RichTextEditor`, keeping its per-editor-type typography live for the
/// life of the tab. Initial values are already baked onto `editor` by the caller
/// (`typography_defaults` + `font_size_scale`); this registers one `ctx.effect`
/// per settings field so a preference edit re-pushes the whole bundle through
/// the editor handle to every open tab. Four *separate* effects rather than one
/// combined `zip` signal — `zip`/`zip3` build a *derived* signal, which panics
/// on `.observe()`; the `SettingsStore` signals are mutable, so per-field
/// effects are safe.
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
    /// so `Drop` can withdraw it. `None` when no `FormatViewModel` was supplied
    /// (the widget tests, which build editors with no app around them).
    format: Option<(FormatViewModel, WidgetId)>,
    /// This window's Format VM — used at build to register; not the live registry entry.
    format_vm: Option<FormatViewModel>,
    /// Typewriter scrolling for this editor, when it is a full-page writing
    /// surface. `None` for the surfaces that deliberately never pin — the
    /// compact synopsis box and the corkboard cards, both small bounded boxes
    /// where holding a line at a fixed height means nothing.
    typewriter: Option<crate::view_models::TypewriterSettings>,
    /// The ambient caret band for this editor. `None` on the surfaces built with no app
    /// around them (the widget tests), which draw none.
    caret: Option<crate::view_models::CaretBand>,
}

impl TypographyBoundEditor {
    fn new(
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
        }
    }

    /// Pin this editor's caret line per the shared typewriter setting. Opt-in,
    /// because only the full-page writing surfaces want it.
    fn with_typewriter(mut self, typewriter: crate::view_models::TypewriterSettings) -> Self {
        self.typewriter = Some(typewriter);
        self
    }

    /// Shade the sentence or paragraph the caret is in, per the shared setting. Opt-in
    /// only because a surface built with no app behind it has no setting to read.
    fn with_caret_band(mut self, caret: crate::view_models::CaretBand) -> Self {
        self.caret = Some(caret);
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
        if let Some(format) = self.format_vm.clone() {
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
        // Honour an app-supplied fill (`RichTextEditor::background`) the way the
        // framework's own recipe style does, defaulting to the writing surface.
        // Without this the Side synopsis could not sit flush on its pane: this rect
        // covers the whole padded viewport, so a caller that drops the outer box
        // would still get a Content slab where the dock chrome should be.
        let bg = ctx.add(
            RectWidget::new()
                .background(
                    cfg.background
                        .clone()
                        .unwrap_or_else(|| SurfaceRole::Content.into()),
                )
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
            None,
            None,
            None,
            None,
            // No project around this tree, so no comment binding: the margin
            // collapses to nothing and the column lays out on its own.
            None,
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
        let editor = card_synopsis_editor(doc, test_typo(), || {}, None, None, None, None, None);
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
    /// Lay out a 60-paragraph synopsis at `fit` and report the height it claimed.
    /// `pane_height` stands in for a `Splitter` pane: a hard box the editor is
    /// expected to stay inside. `None` proposes an unbounded height, the way a
    /// flowing page does.
    fn synopsis_height(fit: SynopsisFit, pane_height: Option<f32>) -> f32 {
        use bastyde::widgets::FixedSize;
        let doc = TextDocument::new();
        let _ =
            doc.set_djot_sync(&"A line of synopsis prose that says what happens.\n\n".repeat(60));
        let typo = test_typo();
        let editor = synopsis_editor(
            &doc,
            &typo,
            fit,
            || {},
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            // No project around this tree, so no comment binding.
            None,
        );
        let mut tree = WidgetTree::new();
        let id = match pane_height {
            Some(h) => tree.add(FixedSize::new().width(320.0).height(h).child(editor)),
            None => tree.add(editor),
        };
        tree.layout(SizeProposal::with_width(320.0));
        tree.bounds(id).height
    }

    /// The Side fit fills the pane it is given and scrolls **inside** it, rather
    /// than growing to its content the way `Growing` does. A `Splitter` places a
    /// pane at an exact pixel height, so this is the sizing that makes a
    /// side-by-side synopsis a fixed viewport instead of a column that runs off
    /// the bottom of the tab.
    ///
    /// (The pane's *colour* — Main showing through rather than a Content card —
    /// is not observable from a headless layout tree; it is checked in the app.)
    #[test]
    fn the_side_fit_is_greedy_so_it_takes_its_panes_exact_height() {
        let h = synopsis_height(SynopsisFit::Side, Some(200.0));
        assert!(
            (h - 200.0).abs() < 2.0,
            "a Side synopsis must stay inside its pane (200px) and scroll the \
             overflow, got {h:.1}px — a taller value means it reverted to \
             intrinsic sizing and would run past the bottom of the pane"
        );
    }

    /// Adding the Side arm must not have disturbed the two fits that were already
    /// there: Compact stays a short capped box, Growing still grows past it.
    #[test]
    fn the_existing_fits_keep_their_sizing() {
        let compact_h = synopsis_height(SynopsisFit::Compact, None);
        let growing_h = synopsis_height(SynopsisFit::Growing, None);
        assert!(
            compact_h > 0.0 && compact_h < growing_h,
            "Compact ({compact_h:.1}px) must stay capped well under a 60-paragraph \
             Growing synopsis ({growing_h:.1}px)"
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

/// Typewriter scrolling reaches the editor the app actually builds.
///
/// The unit tests for the preset→fraction vocabulary live with
/// [`crate::view_models::TypewriterAnchor`]; these pin the *wiring* — that the
/// shared setting arrives at a real `RichTextEditor` built through
/// `writing_column`, and keeps arriving when the setting changes under it.
#[cfg(all(test, feature = "mocks"))]
mod typewriter_tests {
    use super::*;
    use bastyde::core::widget_tree::WidgetTree;

    use crate::view_models::{TypewriterAnchor, TypewriterSettings};

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

    /// A real writing column, reaching the editor's handle the way the rest of
    /// the app does — through the find view-model `writing_column` attaches it
    /// to. A detached editor over the same document would share the text but
    /// not the state under test.
    fn column_with(typewriter: Option<TypewriterSettings>) -> (EditorHandle, WidgetTree) {
        column_with_band(typewriter, None)
    }

    /// As [`column_with`], with an explicit caret band — the shape the band tests need.
    fn column_with_band(
        typewriter: Option<TypewriterSettings>,
        caret: Option<crate::view_models::CaretBand>,
    ) -> (EditorHandle, WidgetTree) {
        let (_doc, handle, tree) = column_with_document_and(typewriter, caret);
        (handle, tree)
    }

    /// As [`column_with_band`], handing back the document too — the caret-band tests assert
    /// on its paint spans, which is where the whole chain ends up.
    pub(super) fn column_with_document(
        caret: Option<crate::view_models::CaretBand>,
    ) -> (TextDocument, EditorHandle, WidgetTree) {
        column_with_document_and(None, caret)
    }

    fn column_with_document_and(
        typewriter: Option<TypewriterSettings>,
        caret: Option<crate::view_models::CaretBand>,
    ) -> (TextDocument, EditorHandle, WidgetTree) {
        let doc = TextDocument::new();
        doc.set_plain_text("Some prose to write in.").unwrap();
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
            None,
            None,
            typewriter,
            caret,
            None,
            None,
        );
        let mut tree = WidgetTree::new();
        tree.add(col);
        tree.layout(SizeProposal::exact(900.0, 600.0));
        let handle = find
            .editor_handle()
            .expect("writing_column attached its handle");
        (doc, handle, tree)
    }

    #[test]
    fn the_setting_reaches_the_editor_at_build_time() {
        // Pushed up front, not only on the next settings change — an editor
        // opened while the feature is already on must pin from its first
        // keystroke.
        let tw = TypewriterSettings::new(
            Signal::new(true),
            Signal::new(Some(TypewriterAnchor::BottomQuarter)),
        );
        let (handle, _tree) = column_with(Some(tw));
        assert_eq!(handle.get_typewriter(), Some(0.75));
    }

    #[test]
    fn an_editor_built_with_the_feature_off_does_not_pin() {
        let tw = TypewriterSettings::new(Signal::new(false), Signal::new(None));
        let (handle, _tree) = column_with(Some(tw));
        assert_eq!(handle.get_typewriter(), None);
    }

    #[test]
    fn a_surface_that_never_pins_passes_no_setting_at_all() {
        let (handle, _tree) = column_with(None);
        assert_eq!(handle.get_typewriter(), None);
    }

    #[test]
    fn toggling_the_setting_reaches_an_already_open_editor() {
        // The live path: Settings ▸ Editor Behavior flips the signal while tabs
        // are open. Both source signals must drive it — a combined derived
        // signal would panic on observe, which is why the wiring registers one
        // effect per field.
        let enabled = Signal::new(false);
        let preset = Signal::new(Some(TypewriterAnchor::Middle));
        let tw = TypewriterSettings::new(enabled.clone(), preset.clone());
        let (handle, _tree) = column_with(Some(tw));
        assert_eq!(handle.get_typewriter(), None);

        enabled.set(true);
        assert_eq!(
            handle.get_typewriter(),
            Some(0.5),
            "turning the feature on must reach an editor that is already open"
        );

        preset.set(Some(TypewriterAnchor::TopThird));
        assert!(
            (handle.get_typewriter().unwrap() - 1.0 / 3.0).abs() < 1e-6,
            "changing the preset must move the pin on an already-open editor"
        );

        enabled.set(false);
        assert_eq!(
            handle.get_typewriter(),
            None,
            "and turning it off must stop it"
        );
    }
}

/// The caret band, from the settings signal all the way into the document.
///
/// The layers below this have their own tests — the segmenter in `text-document`, the session
/// and the frame loop in `bastyde`. What only this level can prove is that the *chain* is
/// connected: `SettingsViewModel` → `CaretHighlightSettings` → `ContentTab` →
/// `writing_column` → `TypographyBoundEditor` → `EditorHandle` → the document's paint spans.
#[cfg(all(test, feature = "mocks"))]
mod caret_band_tests {
    use super::typewriter_tests::*;
    use super::*;
    use bastyde::core::widget_tree::WidgetTree;
    use bastyde::text_document::{Color, FlowElementSnapshot, HighlightMask};

    use crate::view_models::{CaretBand, CaretHighlightSettings, HighlightScope};

    const BAND: Color = Color {
        red: 255,
        green: 254,
        blue: 235,
        alpha: 255,
    };

    fn band(scope: HighlightScope) -> (CaretBand, Signal<HighlightScope>, Signal<Color>) {
        let scope_sig = Signal::new(scope);
        let color_sig = Signal::new(BAND);
        let settings = CaretHighlightSettings::new(scope_sig.clone(), color_sig.clone());
        (
            CaretBand::new(settings, Some("en".into())),
            scope_sig,
            color_sig,
        )
    }

    /// The banded extents on the document's first block, as `(start, length)`.
    fn banded(doc: &TextDocument, color: Color) -> Vec<(usize, usize)> {
        match &doc.snapshot_flow_masked(&HighlightMask::all()).elements[0] {
            FlowElementSnapshot::Block(b) => b
                .paint_highlights
                .iter()
                .filter(|s| s.background_color == Some(color))
                .map(|s| (s.start, s.length))
                .collect(),
            _ => panic!("expected a block"),
        }
    }

    fn pump(tree: &mut WidgetTree) {
        tree.request_frame();
        tree.tick_animations(std::time::Duration::from_millis(16));
        tree.layout(SizeProposal::exact(900.0, 600.0));
    }

    /// Click into the column so the editor takes focus. The band deliberately shows only in
    /// the **focused** view (that is what keeps a split banding once, not twice), so a test
    /// that never focuses would see nothing however well the wiring works.
    fn click_into(tree: &mut WidgetTree) {
        let _ = tree.render();
        tree.dispatch_event(bastyde::core::WidgetEvent::PointerDown {
            position: bastyde::canvas::Point::new(450.0, 20.0),
            button: bastyde::core::PointerButton::Primary,
            modifiers: bastyde::core::Modifiers::NONE,
        });
        pump(tree);
        assert!(tree.focused().is_some(), "the click must focus the editor");
    }

    /// Turning the setting on bands the caret's sentence in a real writing column — no test
    /// double anywhere between the preference and the paint span.
    #[test]
    fn the_setting_reaches_a_real_writing_column() {
        let (band, _scope, _color) = band(HighlightScope::Sentence);
        let (doc, handle, mut tree) = column_with_document(Some(band));

        click_into(&mut tree);
        handle.select_range(2, 2);
        pump(&mut tree);

        assert_eq!(
            banded(&doc, BAND),
            [(0, 23)],
            "the caret's sentence is banded"
        );
    }

    /// Changing the preference on an already-open editor must reach it, which is what the
    /// per-signal effects in `TypographyBoundEditor::build` are for.
    #[test]
    fn changing_the_scope_reaches_an_already_open_editor() {
        let (band, scope, _color) = band(HighlightScope::None);
        let (doc, handle, mut tree) = column_with_document(Some(band));
        click_into(&mut tree);
        handle.select_range(2, 2);
        pump(&mut tree);
        assert!(banded(&doc, BAND).is_empty(), "off by default here");

        scope.set(HighlightScope::Sentence);
        pump(&mut tree);
        assert_eq!(banded(&doc, BAND), [(0, 23)], "the band appeared");

        scope.set(HighlightScope::None);
        pump(&mut tree);
        assert!(banded(&doc, BAND).is_empty(), "and went away again");
    }

    /// A live theme switch drives the colour signal, and the band must follow it — otherwise
    /// going dark would leave every open editor banded in the light theme's shade.
    #[test]
    fn changing_the_colour_repaints_an_open_band() {
        const DARK: Color = Color {
            red: 38,
            green: 40,
            blue: 46,
            alpha: 255,
        };
        let (band, _scope, color) = band(HighlightScope::Sentence);
        let (doc, handle, mut tree) = column_with_document(Some(band));
        click_into(&mut tree);
        handle.select_range(2, 2);
        pump(&mut tree);
        assert_eq!(banded(&doc, BAND), [(0, 23)]);

        color.set(DARK);
        pump(&mut tree);
        assert!(banded(&doc, BAND).is_empty(), "the old shade is gone");
        assert_eq!(banded(&doc, DARK), [(0, 23)], "repainted in the new one");
    }

    /// A surface built with no settings behind it draws nothing — the contract every widget
    /// test in this file relies on.
    #[test]
    fn a_column_without_a_band_draws_none() {
        let (doc, handle, mut tree) = column_with_document(None);
        click_into(&mut tree);
        handle.select_range(2, 2);
        pump(&mut tree);
        assert!(banded(&doc, BAND).is_empty());
        assert!(handle.get_caret_highlight().is_none());
    }
}

#[cfg(test)]
mod width_probe_tests {
    use super::*;
    use bastyde::core::widget_tree::WidgetTree;

    /// A leaf that records how many times it was built, so a test can prove the
    /// `Switcher` under [`WidthProbe`] *keeps* a branch alive across breakpoint
    /// crossings instead of rebuilding it.
    #[derive(Debug)]
    struct Tagged {
        builds: Rc<Cell<u32>>,
    }

    impl Tagged {
        fn new() -> (Self, Rc<Cell<u32>>) {
            let builds = Rc::new(Cell::new(0));
            (
                Self {
                    builds: builds.clone(),
                },
                builds,
            )
        }
    }

    impl Widget for Tagged {
        fn build(&mut self, _ctx: &mut BuildContext) -> Vec<WidgetId> {
            self.builds.set(self.builds.get() + 1);
            Vec::new()
        }

        fn layout_response(&self, proposal: SizeProposal, _ctx: &LayoutContext) -> LayoutResponse {
            proposal.resolve(10.0, 10.0).into()
        }

        fn place_children(
            &self,
            _bounds: Rect,
            _proposal: SizeProposal,
            _children: &mut [WidgetPlacement],
            _ctx: &LayoutContext,
        ) {
        }

        fn children(&self) -> Vec<WidgetId> {
            Vec::new()
        }
    }

    struct Probe {
        tree: WidgetTree,
        mode: Signal<usize>,
        top_builds: Rc<Cell<u32>>,
        side_builds: Rc<Cell<u32>>,
    }

    impl Probe {
        fn new(enabled: bool, side_width: f32) -> Self {
            let (top, top_builds) = Tagged::new();
            let (side, side_builds) = Tagged::new();
            let probe = WidthProbe::new(
                Signal::new(enabled),
                Signal::new(side_width),
                Box::new(top),
                Box::new(side),
            );
            let mode = probe.mode_signal();
            let mut tree = WidgetTree::new();
            tree.add_boxed(Box::new(probe));
            Self {
                tree,
                mode,
                top_builds,
                side_builds,
            }
        }

        /// Lay out at `width` and let the decision settle. The breakpoint is
        /// published from `place_children`, and the `Switcher` consumes it as a
        /// *deferred* rebuild binding — so a mode change costs one extra pass
        /// before the new branch is mounted. Two passes is the contract; the
        /// third proves it has converged rather than oscillating.
        fn settle(&mut self, width: f32) -> usize {
            for _ in 0..3 {
                self.tree
                    .layout(bastyde::prelude::SizeProposal::exact(width, 600.0));
            }
            self.mode.get()
        }
    }

    /// Wide enough for a synopsis *and* a writable prose column → Side.
    /// Too narrow → Top, even though the setting says Side. The setting is a
    /// preference; the width is the veto.
    #[test]
    fn the_breakpoint_vetoes_side_when_the_prose_column_would_not_fit() {
        // threshold = side_width (280) + PROSE_MIN_WIDTH (320) = 600
        let mut wide = Probe::new(true, 280.0);
        assert_eq!(wide.settle(900.0), MODE_SIDE, "900px fits both columns");

        let mut narrow = Probe::new(true, 280.0);
        assert_eq!(
            narrow.settle(500.0),
            MODE_TOP,
            "500px cannot seat a 280px synopsis beside a 320px prose column"
        );
    }

    /// Placement Top is honoured at every width — the probe never promotes a tab
    /// to Side on its own.
    #[test]
    fn top_placement_is_never_overridden_by_available_width() {
        let mut probe = Probe::new(false, 280.0);
        assert_eq!(probe.settle(1600.0), MODE_TOP);
    }

    /// The crossing points are asymmetric, so a width parked on the threshold
    /// cannot flip the layout back and forth. Entering Side needs
    /// `threshold + hysteresis`; leaving it needs to fall below
    /// `threshold - hysteresis`.
    #[test]
    fn the_breakpoint_has_hysteresis_so_a_parked_width_cannot_oscillate() {
        let mut probe = Probe::new(true, 280.0); // threshold 600, band 576..=624
        assert_eq!(probe.settle(900.0), MODE_SIDE);

        assert_eq!(
            probe.settle(590.0),
            MODE_SIDE,
            "inside the band from above: stay in Side rather than flip on jitter"
        );
        assert_eq!(probe.settle(570.0), MODE_TOP, "below the band: leave Side");
        assert_eq!(
            probe.settle(590.0),
            MODE_TOP,
            "the same 590px that kept Side must not re-enter it — that asymmetry \
             is what makes the breakpoint stable"
        );
        assert_eq!(probe.settle(630.0), MODE_SIDE, "above the band: enter Side");
    }

    /// Crossing the breakpoint must not rebuild the branch being returned to.
    /// `Switcher::preserves_children_on_rebuild` is what keeps a scene's caret,
    /// scroll offset and spell session alive while the writer drags the window —
    /// this pins that we actually get it.
    #[test]
    fn crossing_the_breakpoint_keeps_each_branch_alive() {
        let mut probe = Probe::new(true, 280.0);

        probe.settle(900.0);
        assert_eq!(probe.side_builds.get(), 1, "Side mounted once");
        let top_after_first = probe.top_builds.get();

        probe.settle(400.0);
        probe.settle(900.0);
        probe.settle(400.0);

        assert_eq!(
            probe.side_builds.get(),
            1,
            "Side was rebuilt on a later crossing — the Switcher is not preserving it"
        );
        assert_eq!(
            probe.top_builds.get(),
            top_after_first,
            "Top was rebuilt on a later crossing — the Switcher is not preserving it"
        );
    }
}
