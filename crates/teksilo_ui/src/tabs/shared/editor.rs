// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Low-level editor primitives shared by the per-combination tabs: the quiet
//! writing-editor style, the centered/capped writing column, the synopsis box,
//! the title field, and the live typography plumbing. Composite pane renders
//! (heading form, dual-pane prose, folder synopsis) live in [`super::panes`].

use std::cell::{Cell, RefCell};
use std::rc::Rc;

use teksilo::core::binding::BindingLevel;
use teksilo::core::styles::{RichTextEditorStyle, RichTextEditorStyleConfig};
use teksilo::core::widget::WidgetPlacement;
use teksilo::core::widget_builder::HandlerSet;
use teksilo::prelude::*;
use teksilo::text::EditorTypographyDefaults;
use teksilo::text_document::TextDocument;
use teksilo::text_document::{Color as DocColor, HighlightFormat};
use teksilo::tokens::{BorderRole, CornerRadius, SurfaceRole};
use teksilo::widgets::SplitterModel;
use teksilo::widgets::rich_text::{EditCommandKind, EditorHandle, RichTextEditor, ScrollPolicy};
use teksilo::widgets::{
    Button, ButtonVariant, Checkbox, Expand, FixedSize, GroupHeader, HStack, IconButton,
    IconWidget, MaxSize, MenuItem, MenuList, Padding, Panel, RectWidget, Switcher, TextInput,
    TextWidget, VStack, ZStack,
};

/// The find banner's query field caps at this width — a full-width field reads as
/// a search page, not a compact IntelliJ-style bar.
mod bound_editor;
mod find_banner;
mod scroll_sync;
mod synopsis;

pub(crate) use bound_editor::wire_spell;
pub(crate) use find_banner::highlight_of;
pub use scroll_sync::{PROSE_MIN_WIDTH, side_synopsis_editor};
pub(crate) use scroll_sync::{
    PageScrollPort, SYNOPSIS_PANE, SideSync, SynopsisPaneEffects, WidthProbe,
};
pub use synopsis::{
    SYNOPSIS_MIN_LINES, SynopsisFit, card_synopsis_editor, synopsis_column, synopsis_editor,
    synopsis_section, title_input,
};

use bound_editor::*;
use find_banner::*;

const FIND_FIELD_MAX_WIDTH: f32 = 240.0;

use crate::format::{EditorKind, FormatViewModel};
use crate::intents::AppIntent;
use crate::search::FindViewModel;
use crate::settings::EditorTypography;
use crate::spellcheck::SpellSession;
use crate::tabs::TitleField;
use crate::text_replacement::TextReplacementSession;

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
    find: Option<FindViewModel>,
    spell: Option<Rc<SpellSession>>,
    replacement: Option<Rc<TextReplacementSession>>,
    format: Option<FormatViewModel>,
    // Typewriter scrolling for this surface. `None` on the surfaces that never
    // pin (and in the widget tests, which build columns with no app around them).
    typewriter: Option<crate::shared::TypewriterSettings>,
    // The ambient caret band for this surface — the shared preference plus this
    // document's language. `None` on the surfaces built without an app around
    // them (the widget tests), which draw no band.
    caret: Option<crate::shared::CaretBand>,
    // The writing games this project is playing (currently "Always forward"),
    // which may freeze this surface while one is on. `None` on the surfaces
    // built with no app around them (the widget tests). Which surfaces a game
    // covers is the game's own decision, taken against this editor's kind.
    games: Option<crate::writing_session::WritingGamesViewModel>,
    // Where this document's caret should start, and the ports to publish this
    // editor's handle into so the position can be read back. `None` for every
    // surface that is not a tab's *main* prose column — a stream shows one
    // editor per row, so there is no single "the" caret to persist, the same
    // reason `synopsis_column` passes no handle sink there.
    view_state: Option<crate::shared::ViewStateBinding>,
    // This editor's door to the comment feature — minted by the `OpenDoc` that
    // knows which `Content` row the document came from. `None` on every surface
    // built without a project around it (the widget tests) and on any document
    // with no comment store, which collapses the comment affordances rather than
    // panicking.
    comments: Option<crate::comments::binding::CommentBinding>,
    // This editor's door to the footnote feature, minted by the same `OpenDoc`
    // and for the same reason as `comments` above. It carries both directions:
    // the dock's parked "reveal this note" request on the way in, and this
    // editor's caret on the way out, so a marker the writer clicks lights up its
    // row in the dock. `None` on every surface built without a project around it.
    footnotes: Option<crate::footnotes::FootnoteBinding>,
    // Where this editor fetches an image it meets but its document does not
    // have — a picture pasted in from another editor, or brought back by an
    // undo. `None` on the surfaces built without a project around them.
    images: Option<crate::shared::images::ImageSource>,
    // Which project this surface's typing belongs to — `Work.unique_id`, from
    // [`ContentTab::work_unique_id`].
    //
    // `None` on every surface built without a project around it, on an unsaved
    // one, and in the widget tests — the same shape as `comments` / `footnotes`
    // / `images` above, and for the same reason.
    //
    // The **project**, not the item and not the window: a project can have
    // several editors open on it, and a count that meant anything per window
    // would mean nothing at all. The tally itself is `common::arrival::shared()`,
    // reached rather than threaded — see that module for why the end that
    // reports and the end that reads cannot hand it to each other.
    arrival_project: Option<String>,
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
    // The `BinderItem` whose text this is **and the surface showing it**, announced
    // to the formatting registry so anything needing *a named item's* editor can
    // find it without going through focus — the margin lane, which converts offsets
    // for every row of a stream at once. `None` on the surfaces built with no
    // project around them, the same shape as `comments` / `footnotes` / `images`
    // above.
    //
    // The surface, not just the item: several live editors can show the same item
    // at once (a dual-pane tab builds two prose columns for its own scene; a scene
    // open in a tab is also a row of the Full Chapter beside it), and a lane asking
    // by item alone was answered by whichever had registered first. See
    // `crate::margin_lane::LaneAnchor`.
    anchor: Option<crate::margin_lane::LaneAnchor>,
    // Whether this editor should guess its height from its text before anything has
    // laid it out.
    //
    // `true` for a **stream row**, and only there. A row below the fold never lays
    // out, so it claims the `min_lines` floor whatever it holds, and the page's
    // height is the sum of those claims — the scroll extent starts wrong by an order
    // of magnitude and settles a row at a time as the reader arrives. A tab's editor
    // is on screen and lays out on its first frame, so the guess would buy it nothing
    // and every consumer of its first-frame size would pay for it.
    estimate_height: bool,
    // This project's tags, for the "Add as note" submenu: the writer picks one tag, and
    // that one choice settles whether the entry is findable in prose, where it is filed,
    // and what shape it starts in. `None` on every surface built with no project around
    // it, the same shape as `comments` / `footnotes` / `images` above, and the submenu
    // simply does not render there. See [`CapturePalette`].
    tags: Option<crate::tags::TagsViewModel>,
) -> HStack {
    // Stand by to supply an image this document does not have. A picture
    // pasted in from another editor arrives as a reference — pixels live on the
    // document that owns them, and a clipboard fragment is not a document — so
    // without this it lays out at full size and paints nothing.
    let mut editor = if read_only {
        RichTextEditor::read_only(doc.clone())
    } else {
        RichTextEditor::editor(doc.clone())
    }
    .estimate_height_before_layout(estimate_height);
    if let Some(source) = &images {
        let resolve = source.resolver();
        editor = editor.on_image_missing(resolve);
    }
    // Cloned before the tally consumes it: the capture submenu, built later, keys this
    // project's recents on the same uid.
    let capture = tags.clone().map(|tags| CapturePalette {
        tags,
        work_uid: arrival_project.clone(),
    });
    if let Some(project) = arrival_project {
        // The toolkit says which channel; what a *manuscript* makes of that is
        // the application's to decide, so the mapping is here rather than in
        // teksilo. `Ime` maps to `Typed` because a writer composing in Japanese
        // is typing — the characters that land are not the keys pressed, but any
        // other answer would be wrong about them specifically.
        let tally = common::arrival::shared();
        editor = editor.on_text_inserted(move |source, chars| {
            use common::arrival::Arrival;
            use teksilo::widgets::rich_text::EditSource;
            let arrival = match source {
                EditSource::Keyboard | EditSource::Ime => Arrival::Typed,
                EditSource::Clipboard => Arrival::Pasted,
                EditSource::Accessibility => Arrival::Dictated,
                EditSource::Programmatic => Arrival::Programmatic,
            };
            tally.record(&project, arrival, chars as u64);
        });
    }
    let mut editor = editor
        .style(WritingEditorStyle)
        .on_change(on_change)
        // Ctrl(⌘)+click follows a hyperlink; a plain click just puts the caret
        // in it, because this is an editor and the writer is usually trying to
        // edit the words. The scheme check lives in the opener: a `.skrib` can
        // come from anywhere, and a link in one is a string someone else chose.
        .on_link_activated(|href, ctx| {
            crate::shared::external_link::open_external_link(href, ctx);
        })
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
        *vs.page_editor.borrow_mut() = Some(handle.clone());
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
            // Only reached (and so only consumed — see below) when no comment seek
            // is pending: a comment seek always wins, and a footnote seek left
            // untouched here survives to be resolved on this editor's next build,
            // rather than being silently thrown away by a match arm that never
            // looked at it.
            None => {
                // A footnotes dock row parks a **label**, not a range: where its
                // marker sits is whatever this document says right now, which is
                // the only offset that cannot have gone stale between the click
                // and this build. Resolved to a one-character selection, so the
                // writer sees exactly which reference the row meant.
                let note_seek = footnotes
                    .as_ref()
                    .and_then(|f| f.take_seek())
                    .and_then(|label| crate::footnotes::FootnoteBinding::position_of(doc, &label));
                match note_seek {
                    Some(pos) => {
                        let last = doc.character_count();
                        handle.select_range(pos.min(last), (pos + 1).min(last));
                    }
                    None => {
                        // Not clamped here. `TextCursor::set_position` clamps to the
                        // document's own maximum cursor position, which counts the
                        // separator between every pair of blocks; `character_count`
                        // does not count them. Clamping against the smaller of the
                        // two silently walks the caret back one character per
                        // paragraph, so restoring the end of a 90 block chapter put
                        // the writer 90 characters short of where they left off, and
                        // the further into a document they were the worse it got.
                        handle.select_range(vs.initial.caret, vs.initial.caret);
                    }
                }
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
        // The row, off the anchor: "Add as note" needs the item this text belongs to,
        // and the anchor is where that is named now that a surface token travels with
        // it. A column built with no project around it has no anchor and no capture
        // either, which is the same `None` the submenu already handles.
        let item = anchor.map(|a| a.item);
        let cursor = editor.cursor_position_signal();
        let doc = doc.clone();
        let spell = spell.clone();
        let comments = comments.clone();
        editor = editor.context_menu(move |pt, ctx| {
            handle.reposition_caret_for_context_menu(pt);
            // Which tags this writer reaches for, read off the service the app already
            // holds open rather than reopening its file here. `MenuItem::submenu` runs
            // its factory during `build`, not at hover, so anything the submenu does
            // happens on *every* right-click that offers the row: a fresh
            // `SettingsFile::load` would take an advisory lock and parse TOML on the UI
            // thread before the writer had so much as looked at "Add as note".
            let recents = capture_recents(ctx, capture.as_ref());
            Some(Box::new(editor_context_menu(
                handle.clone(),
                cursor.clone(),
                split.clone(),
                doc.clone(),
                spell.clone(),
                comments.clone(),
                item,
                capture.clone(),
                recents,
            )))
        });
    }
    // A click on an image records which one, so the Document menu's describe /
    // resize commands have something to act on. The editor deliberately does not
    // move the caret onto an image (the same rule links follow), so this is the
    // only thing that can say *which* picture the writer means — and a document
    // may hold the same one three times.
    if let Some(fvm) = &format {
        let fvm = fvm.clone();
        let handle = editor.handle();
        editor = editor.on_image_activated(move |activation, _ctx| {
            fvm.set_active_image(Some((activation.offset, activation.name.clone())));
            // …and select it. An image is one character, so this selects
            // exactly it — which is why the activation carries the offset. The
            // editor deliberately does not move the caret itself (the rule
            // links follow), so a host that wants the picture selected has to
            // say so; before this the only way to select one was to drag from
            // the end of the block above to the start of the one below.
            handle.select_range(activation.offset, activation.offset + 1);
        });
    }
    // Files dropped on the prose. The editor has already put the caret where
    // they landed; turning a path into a picture needs the media directory, an
    // `Asset` row and an undo stack, so the paths are parked on the view-model
    // and the command that owns that pipeline picks them up.
    if let Some(fvm) = &format {
        let fvm = fvm.clone();
        editor = editor.on_files_dropped(move |paths, ctx| {
            fvm.dropped_files().set(paths.to_vec());
            ctx.send_intent(Intent::new("editor.insert_dropped_images"));
        });
    }
    // Dragging a corner grip reports a size; writing it into the prose is this
    // side's job, because only the app knows the display size lives in the
    // reference's Djot attributes. Same rewrite as the Resize command, so a
    // dragged resize and a typed one land identically on the undo stack.
    {
        let handle = editor.handle();
        editor = editor.on_image_resized(move |resize, _ctx| {
            let Some(image) = crate::shared::images::image_at(
                &handle.to_plain_text(),
                resize.offset,
                &handle.to_djot(),
            ) else {
                return;
            };
            handle.select_range(resize.offset, resize.offset + 1);
            handle.insert_djot(&image.djot(&image.alt, Some((resize.width, resize.height))));
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
    if let Some(g) = games {
        bound = bound.with_writing_games(g);
    }
    if let Some(tw) = typewriter {
        bound = bound.with_typewriter(tw);
    }
    if let Some(anchor) = anchor {
        bound = bound.with_anchor(anchor);
    }
    if let Some(band) = caret {
        bound = bound.with_caret_band(band);
    }
    if let Some(f) = footnotes.clone() {
        bound = bound.with_footnotes(f, doc.clone());
    }
    let capped = teksu!(
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

/// The image the selection covers, when it covers exactly one and nothing else.
///
/// An inline image is a single character, so a selection of length 1 that
/// resolves to an image *is* an image selection. A wider selection that happens
/// to contain a picture is not: the writer selected prose, and prose actions are
/// what they want.
fn selected_image(handle: &EditorHandle) -> Option<crate::shared::images::ImageRef> {
    let (a, b) = handle.selection();
    let (start, end) = (a.min(b), a.max(b));
    if end != start + 1 {
        return None;
    }
    crate::shared::images::image_at(&handle.to_plain_text(), start, &handle.to_djot())
}

/// The right-click menu over a selected image: move it, then act on it.
///
/// Cut/Copy/Paste lead because they are what a picture is most often
/// right-clicked for, and because they are the actions whose meaning does not
/// change over an image — cutting one takes the picture, exactly as cutting a
/// word takes the word.
///
/// The three below are the same commands the Image menu offers, reached by
/// intent so a keyboard user and a right-clicking user run identical code. No
/// Insert image here: this menu only exists because an image is already
/// selected.
fn image_context_menu(handle: EditorHandle) -> MenuList {
    let cut = handle.clone();
    let copy = handle.clone();
    let paste = handle;
    // Cutting a picture out is still taking something away — same gate as the
    // prose menu's Cut, for the same reason.
    let mut menu = MenuList::new();
    if cut.command_filter().accepts(EditCommandKind::Cut) {
        menu = menu.item(MenuItem::new(tr!(menu_cut())).on_activate_fn(move |ctx| cut.cut(ctx)));
    }
    menu.item(MenuItem::new(tr!(menu_copy())).on_activate_fn(move |ctx| copy.copy(ctx)))
        .item(MenuItem::new(tr!(menu_paste())).on_activate_fn(move |ctx| paste.paste(ctx)))
        .separator()
        .item(
            MenuItem::new(tr!(image_menu_describe()))
                .on_activate_fn(move |ctx| ctx.send_intent(Intent::new("image.describe"))),
        )
        .item(
            MenuItem::new(tr!(image_menu_resize()))
                .on_activate_fn(move |ctx| ctx.send_intent(Intent::new("image.resize"))),
        )
        .item(
            MenuItem::new(tr!(image_menu_reset_size()))
                .on_activate_fn(move |ctx| ctx.send_intent(Intent::new("image.reset_size"))),
        )
}

/// A writing editor's right-click menu: the **formatting row** (Bold / Italic /
/// Underline / Strikethrough) as a chrome strip above the list, the **spelling
/// group** (corrections for the right-clicked word, then *Add to dictionary*)
/// leading the actual items, the standard edit actions (Cut / Copy / Paste / Paste
/// Unformatted / Select All), and — when a split is offered — **Split scene** at
/// the caret.
///
/// Built fresh on each right-click, *after* the factory has moved the caret to the
/// click point, so the resolved word and any Paste act where the user clicked.
///
/// The spelling group leads because the corrections are the reason the menu was
/// opened on a squiggle, one click from the fix rather than behind a submenu. It is
/// **omitted** entirely when nothing flagged resolves here, rather than shown
/// greyed out — a right-click on ordinary prose opens straight at Cut.
/// What the "Add as note" submenu needs to offer this project's tags.
///
/// Two things travel together because neither is any use alone: the palette says which
/// tags exist, and the project uid says whose recents to rank them by. Bundled rather
/// than passed as a pair so a surface that wants capture threads one parameter, and so
/// "this surface has no project around it" stays a single `None` instead of two that
/// could disagree.
///
/// Threaded rather than read off `app_state`: a palette is Tier-2, so an `app_state`
/// lookup answers with whichever Work's session registered first, which on a second open
/// project is the wrong book's tags.
#[derive(Clone)]
pub struct CapturePalette {
    /// This project's tags.
    pub tags: crate::tags::TagsViewModel,
    /// `Work.unique_id`, the key [`crate::models::NoteCaptureService`] files recents
    /// under. `None` on a project not yet saved anywhere, which simply has no recents
    /// yet and gets plain palette order.
    pub work_uid: Option<String>,
}

/// Which tags this project's writer reached for most recently, for the capture submenu's
/// second tier.
///
/// Read from the [`crate::models::NoteCaptureService`] the app registered at startup,
/// which is already in memory. Emphatically **not** a fresh `NoteCaptureService::open`:
/// `MenuItem::submenu` builds its content eagerly during `build` rather than at hover
/// (see its own `build`, which calls the factory and then dormants the result), so
/// everything the submenu does happens on every right-click that offers the row, and a
/// file open there would put an advisory lock and a TOML parse on the UI thread for a
/// writer who only wanted Copy.
///
/// Empty for a project not yet saved (no usable uid to key by) and when the service is
/// unreachable, which costs the menu its ordering and nothing else.
pub(super) fn capture_recents(
    ctx: &EventContext,
    palette: Option<&CapturePalette>,
) -> Vec<uuid::Uuid> {
    let Some(uid) = palette.and_then(|p| p.work_uid.as_deref()) else {
        return Vec::new();
    };
    ctx.app_state::<crate::models::NoteCaptureService>()
        .map(|svc| svc.recent_tags(uid))
        .unwrap_or_default()
}

/// The "Add as note" submenu: which tag this capture is filed under.
///
/// Three tiers and an escape hatch, in this order: the discoverable tags, because those
/// are the story-bible ones and the common case; the few other tags this writer actually
/// reaches for; one **All tags** submenu holding everything, discoverable first, for
/// whatever the two shortlists left out; and **Untagged**, always last and always
/// present.
///
/// Untagged is not a fallback for a broken state. A writer capturing a stray thought has
/// not decided they are building a story bible, and a menu that offers no way through
/// without picking one has got the moment wrong. It is also what makes a brand-new
/// project work with no special case: with no tags at all this renders Untagged alone.
///
/// Every row fires the same intent with a different tag, so the whole gesture is one
/// click and nothing is asked twice. The ordering rules are
/// [`crate::story_bible::capture::build_menu`]'s, and tested there.
fn capture_submenu(
    item_id: u64,
    selected_text: &str,
    palette: Option<&CapturePalette>,
    recents: &[uuid::Uuid],
) -> MenuList {
    use crate::story_bible::capture::build_menu;

    let menu = palette
        .map(|palette| {
            build_menu(
                &palette.tags.rows(),
                recents,
                crate::models::MAX_RECENT_TAGS,
            )
        })
        .unwrap_or_default();
    render_capture_menu(item_id, selected_text, menu)
}

/// The tag's own colour, as the disc that goes in a capture row's icon column.
///
/// The writer picked these colours and reads their palette by them everywhere else in
/// the app — the binder's dots, the corkboard's, the editor's subtitle row. A menu that
/// dropped them would be the one place a tag is named and not recognised, and this menu
/// is exactly where a writer is choosing between four tags at speed.
///
/// `icon_keeps_color` at the call sites is what makes it a swatch rather than a glyph:
/// without it the row tints every icon to its own foreground, which is right for an icon
/// that repeats the label and destroys one whose colour *is* the label.
///
/// **No hairline**, unlike [`crate::tags::TagDotsRow`]'s dot, and the difference is what
/// the dot is doing. There it is alone and carries the whole identity, so a near-white
/// tag has to be outlined or it is an invisible mark; here the tag's name is right beside
/// it and the disc is recognition, not identification. An `IconWidget` fills one path in
/// one colour anyway, so a ring is not on offer without a second widget in a slot that
/// takes an icon.
fn tag_swatch(color: &str) -> IconWidget {
    // Inset by half a pixel so the disc does not graze the column's edge at the
    // sizes a menu row actually draws at.
    const SWATCH: f32 = 10.0;
    let centre = teksilo::canvas::Point::new(SWATCH / 2.0, SWATCH / 2.0);
    let path = teksilo::canvas::Path::circle(centre, SWATCH / 2.0 - 0.5);
    IconWidget::from_path(path, SWATCH).color(crate::tags::contrast::parse(color))
}

/// [`capture_submenu`]'s rendering half, with the tiers already resolved.
///
/// Split out so a test can hand in a tier shape directly: resolving the tiers needs a
/// `TagsViewModel`, and therefore a whole project, while the rule this half encodes,
/// which dividers render, is decided entirely by which tiers came back empty.
fn render_capture_menu(
    item_id: u64,
    selected_text: &str,
    menu: crate::story_bible::capture::CaptureMenu,
) -> MenuList {
    use crate::story_bible::capture::CaptureEntry;

    let fire = |tag_id: Option<u64>| {
        let text = selected_text.to_string();
        move |ctx: &mut EventContext| {
            ctx.send_intent(AppIntent::AddAsNote {
                item_id,
                selected_text: text.clone(),
                tag_id,
            });
        }
    };

    let mut list = MenuList::new();
    for entry in menu.entries() {
        list = match entry {
            CaptureEntry::Separator => list.separator(),
            CaptureEntry::Tag(t) => list.item(
                MenuItem::new(lit!(t.name.clone()))
                    .icon(tag_swatch(&t.color))
                    .icon_keeps_color()
                    .on_activate_fn(fire(Some(t.id))),
            ),
            CaptureEntry::Untagged => {
                list.item(MenuItem::new(tr!(ctx_add_as_note_untagged())).on_activate_fn(fire(None)))
            }
            CaptureEntry::AllTags => {
                // The overflow tier repeats the whole palette, discoverable first, so a
                // tag the two shortlists left out is still one click further in rather
                // than unreachable.
                let all = menu.all.clone();
                let text = selected_text.to_string();
                list.item(MenuItem::submenu(
                    tr!(ctx_add_as_note_all_tags()),
                    move || {
                        let mut inner = MenuList::new();
                        for t in &all {
                            let (text, tag_id) = (text.clone(), t.id);
                            inner = inner.item(
                                MenuItem::new(lit!(t.name.clone()))
                                    .icon(tag_swatch(&t.color))
                                    .icon_keeps_color()
                                    .on_activate_fn(move |ctx: &mut EventContext| {
                                        ctx.send_intent(AppIntent::AddAsNote {
                                            item_id,
                                            selected_text: text.clone(),
                                            tag_id: Some(tag_id),
                                        });
                                    }),
                            );
                        }
                        Box::new(inner) as Box<dyn Widget>
                    },
                ))
            }
        };
    }
    list
}

// Eight, and every one of them is a distinct door this menu opens: the handle and
// cursor it acts through, the split callback, the document, spelling, comments, the
// item, and the tag palette. Bundling them would only move the list somewhere else.
#[allow(clippy::too_many_arguments)]
fn editor_context_menu(
    handle: EditorHandle,
    cursor: Signal<usize>,
    split: Option<SplitFn>,
    doc: TextDocument,
    spell: Option<Rc<SpellSession>>,
    comments: Option<crate::comments::binding::CommentBinding>,
    // The row this editor's text belongs to. `None` on the surfaces built with
    // no project around them (the widget tests), the same shape every other
    // project-only capability here already takes. Feeds "Add as note": the
    // selection cannot span two rows of a manuscript stream (each row is its
    // own document), so the row the caret is in is the only scope the feature
    // needs, and the only one it can name.
    item_id: Option<u64>,
    // This project's tags, for the "Add as note" submenu. `None` on a surface built
    // with no project around it; the submenu then comes down to **Untagged** alone,
    // which is a working capture and not a broken one. Deliberately *not* a gate on
    // whether the row appears: a surface that forgets to thread this would silently
    // lose the feature, and the whole point of the third tier is that there is always
    // a way through.
    capture: Option<CapturePalette>,
    // This project's recently used capture tags, most recent first. Resolved by the
    // caller, which has a context to read the app's own service through; see
    // [`capture_recents`].
    recents: Vec<uuid::Uuid>,
) -> MenuList {
    // One resolution for the whole spelling group — only misspelled words are
    // offered, filtered through this editor's live spell-checker so the group
    // matches the squiggles exactly.
    // Right-clicking a selected picture opens a different menu entirely. The
    // formatting strip, the spelling group and Split scene are all about prose;
    // over an image every one of them is either inert or nonsense, and a menu
    // whose top half does nothing is worse than a shorter one.
    //
    // Decided from the *selection*, not from the click point: clicking an image
    // selects exactly it, so "the selection is one image" is precisely the state
    // this menu is for — and it is the same test wherever the pointer landed.
    if selected_image(&handle).is_some() {
        return image_context_menu(handle);
    }

    let spelling = super::dictionary_menu::resolve_spelling(&doc, &handle, spell.as_deref());

    let mut list = MenuList::new()
        .item(format_row(&handle, &CharacterMark::ALL))
        .separator();

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

    // "Add as note" files the current selection as a new story-bible entry.
    // Resolved *now*, from the selection this right-click actually has (the menu
    // is rebuilt fresh on every open, so this can never go stale the way a
    // cached selection would): empty, or no row to scope it to, and the row
    // simply is not offered, the same "omitted, not greyed out" rule the
    // spelling group above already follows.
    let selected = handle.selected_text();
    if let Some(item_id) = item_id
        && !selected.trim().is_empty()
    {
        let selected_text = selected;
        list = list
            .item(MenuItem::submenu(tr!(ctx_add_as_note()), move || {
                Box::new(capture_submenu(
                    item_id,
                    &selected_text,
                    capture.as_ref(),
                    &recents,
                )) as Box<dyn Widget>
            }))
            .separator();
    }

    let cut = handle.clone();
    let copy = handle.clone();
    let paste = handle.clone();
    let paste_plain = handle.clone();
    let select = handle;
    // Cut takes prose away, so it answers to the same policy Ctrl+X does. The
    // menu is rebuilt on every right-click, so reading the filter here is live —
    // and `EditorHandle::cut` deliberately bypasses the keyboard layer (it is the
    // API paste and version-restore go through), which is exactly why this call
    // site has to ask the question itself rather than assume the editor will.
    if cut.command_filter().accepts(EditCommandKind::Cut) {
        list = list.item(MenuItem::new(tr!(menu_cut())).on_activate_fn(move |ctx| cut.cut(ctx)));
    }
    list = list
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

/// One character mark a [`format_row`] strip can offer.
///
/// A vocabulary rather than a fixed set so each surface states which marks it
/// warrants: the writing editors take [`CharacterMark::ALL`], a comment card's
/// body takes Bold and Italic only (see `comments::card::comment_context_menu`
/// for why those two and no more).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CharacterMark {
    Bold,
    Italic,
    Underline,
    Strikethrough,
}

impl CharacterMark {
    /// Every mark, in the strip's canonical order.
    pub const ALL: [CharacterMark; 4] = [
        CharacterMark::Bold,
        CharacterMark::Italic,
        CharacterMark::Underline,
        CharacterMark::Strikethrough,
    ];

    fn icon(self) -> IconWidget {
        match self {
            CharacterMark::Bold => crate::icons::format::bold(),
            CharacterMark::Italic => crate::icons::format::italic(),
            CharacterMark::Underline => crate::icons::format::underline(),
            CharacterMark::Strikethrough => crate::icons::format::strikethrough(),
        }
    }

    fn tooltip(self) -> teksilo::i18n::LocalizedString {
        match self {
            CharacterMark::Bold => tr!(format_bold()),
            CharacterMark::Italic => tr!(format_italic()),
            CharacterMark::Underline => tr!(format_underline()),
            CharacterMark::Strikethrough => tr!(format_strikethrough()),
        }
    }

    fn apply(self) -> fn(&EditorHandle) {
        match self {
            CharacterMark::Bold => EditorHandle::toggle_bold,
            CharacterMark::Italic => EditorHandle::toggle_italic,
            CharacterMark::Underline => EditorHandle::toggle_underline,
            CharacterMark::Strikethrough => EditorHandle::toggle_strikethrough,
        }
    }

    fn read(self) -> fn(&EditorHandle) -> bool {
        match self {
            CharacterMark::Bold => EditorHandle::is_bold,
            CharacterMark::Italic => EditorHandle::is_italic,
            CharacterMark::Underline => EditorHandle::is_underline,
            CharacterMark::Strikethrough => EditorHandle::is_strikethrough,
        }
    }
}

/// The requested character marks, as a strip across the top of a context menu.
///
/// Acts on the **right-clicked** editor's handle rather than resolving "whichever
/// editor has focus": the user pointed at one, and `reposition_caret_for_context_menu`
/// has already preserved their selection if the click landed inside it. So
/// select-a-phrase → right-click → Bold formats the phrase, and no focus
/// resolution is involved. (Right-clicking bare prose collapses to a caret, where
/// a toggle sets the *typing* format — the word-processor convention.)
///
/// Unlike the dock, the state is read once here and never polled: the whole menu
/// is rebuilt on every right-click, so a snapshot cannot go stale. That is also
/// why this is `pub` for the comment card's menu to mount: a card has no frame
/// tick to poll `format_version()` on, so a menu rebuilt per right-click is the
/// one place a comment can show a lit Bold without the borrow trap
/// `FormatViewModel`'s module doc warns about.
///
/// Clicking one of these does **not** close the menu — dismissal is `MenuItem`
/// plumbing (`ctx.dismiss_self_overlay_chain`) that `IconButton` has no part in —
/// so the strip works as a sticky mini-toolbar: bold, then italic, then Escape.
/// That is the better behaviour, and it is why the buttons cannot lean on
/// `IconButton::toggle`'s optimistic flip: over a mixed selection "toggle bold" is
/// not a negation, and with no rebuild coming the button would lie for the rest of
/// the visit. Each click therefore runs the real command and writes back what the
/// editor actually did.
pub fn format_row(handle: &EditorHandle, marks: &[CharacterMark]) -> Padding {
    /// One mark: an icon, its accessible name, the command, and the state it shows.
    fn mark(
        icon: IconWidget,
        tooltip: impl Into<teksilo::i18n::LocalizedString>,
        state: Signal<bool>,
        handle: EditorHandle,
        apply: fn(&EditorHandle),
        read: fn(&EditorHandle) -> bool,
    ) -> IconButton {
        IconButton::new(icon)
            .toolbar()
            // Keeps the strip out of Tab order, matching teksilo's own format
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

    let mut row = HStack::new().spacing(4.0);
    for m in marks {
        let read = m.read();
        row = row.child(mark(
            m.icon(),
            m.tooltip(),
            Signal::new(read(handle)),
            handle.clone(),
            m.apply(),
            read,
        ));
    }
    Padding::symmetric(6.0, 6.0).child(row)
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
    find: Option<FindViewModel>,
    spell: Option<Rc<SpellSession>>,
    replacement: Option<Rc<TextReplacementSession>>,
    format: Option<FormatViewModel>,
    typewriter: Option<crate::shared::TypewriterSettings>,
    // The ambient caret band for this surface — the shared preference plus this
    // document's language. `None` on the surfaces built without an app around
    // them (the widget tests), which draw no band.
    caret: Option<crate::shared::CaretBand>,
    // The writing games this project is playing (currently "Always forward"),
    // which may freeze this surface while one is on. `None` on the surfaces
    // built with no app around them (the widget tests). Which surfaces a game
    // covers is the game's own decision, taken against this editor's kind.
    games: Option<crate::writing_session::WritingGamesViewModel>,
    view_state: Option<crate::shared::ViewStateBinding>,
    comments: Option<crate::comments::binding::CommentBinding>,
    // Forwarded straight to [`writing_column`] — see its own note.
    footnotes: Option<crate::footnotes::FootnoteBinding>,
    // Where this editor fetches an image it meets but its document does not
    // have — a picture pasted in from another editor, or brought back by an
    // undo. `None` on the surfaces built without a project around them.
    images: Option<crate::shared::images::ImageSource>,
    // Forwarded straight to [`writing_column`] — see its own note.
    arrival_project: Option<String>,
    // Whether this surface may be typed into — see `writing_column`.
    read_only: bool,
    // Forwarded straight to [`writing_column`] — see its own note.
    anchor: Option<crate::margin_lane::LaneAnchor>,
    // Forwarded straight to [`writing_column`] — see its own note.
    estimate_height: bool,
    tags: Option<crate::tags::TagsViewModel>,
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
            games,
            view_state,
            comments,
            footnotes,
            images,
            arrival_project,
            read_only,
            anchor,
            estimate_height,
            tags,
        ))
}

/// A flat, edge-to-edge backdrop wrapping the tab body (the `TabWidget` doesn't
/// paint a content background).
///
/// `background` is the tab's own answer
/// ([`ContentTab::backdrop_role`](crate::tabs::ContentTab::backdrop_role)) rather
/// than a constant, and this is the **one** place any of the twelve
/// combinations paints its background — so the distraction-free surface can ask
/// for `Transparent` and paint its own page underneath, without a second
/// renderer and without any per-combination branching.
pub fn tab_backdrop(background: SurfaceRole, body: impl Widget + 'static) -> Box<dyn Widget> {
    Box::new(teksu!(
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
    Box::new(teksu!(
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

/// A fixed vertical gap.
pub fn vspace(height: f32) -> impl Widget {
    teksu!(FixedSize { height: height })
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
    CenterColumnFlowing::new(teksu!(
        MaxSize::width(column_width.get()) {
            max_width: column_width.clone()
            child: child
        }
    ))
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

/// The writing surface driven by the **real frame loop** — the integration nothing
/// else tests.
///
/// The other replace-while-typing tests (`text_replacement::session::live_editor_tests`)
/// build a bare `RichTextEditor` and call `session.tick(...)` directly — proving the state
/// machine, not how it is invoked. Replace-while-typing is wired from the editor's
/// `on_change`, which runs inside `frame_loop::tick` while the editor's own state is
/// mutably borrowed; a bare `tick()` call never exercises that borrow.
///
/// `WidgetTree::request_frame()` arms the tick and `layout()` advances it
/// (`layout_impl` calls `advance_frame_tick` on every pass), so the effect registered by
/// [`wire_replacements`] fires with the same `BuildContext` and borrow order as the
/// running app. `advance_frame_tick` stays `pub(crate)` — the two public calls above
/// already drive it exactly as the winit loop does each frame.
#[cfg(all(test, feature = "mocks"))]
mod frame_loop_tests;

#[cfg(test)]
mod tests;

/// Typewriter scrolling reaches the editor the app actually builds.
///
/// The unit tests for the preset→fraction vocabulary live with
/// [`crate::shared::TypewriterAnchor`]; these pin the *wiring* — that the
/// shared setting arrives at a real `RichTextEditor` built through
/// `writing_column`, and keeps arriving when the setting changes under it.
#[cfg(all(test, feature = "mocks"))]
mod typewriter_tests;

/// The caret band, from the settings signal all the way into the document.
///
/// The layers below this have their own tests — the segmenter in `text-document`, the session
/// and the frame loop in `teksilo`. What only this level can prove is that the *chain* is
/// connected: `SettingsViewModel` → `CaretHighlightSettings` → `ContentTab` →
/// `writing_column` → `TypographyBoundEditor` → `EditorHandle` → the document's paint spans.
#[cfg(all(test, feature = "mocks"))]
mod caret_band_tests;

/// Ctrl+Wheel, from the gesture to the persisted size preference.
///
/// Gated with the other whole-column fixtures above, and for the same reason:
/// these build a real `writing_column` rather than a widget in isolation.
#[cfg(all(test, feature = "mocks"))]
mod text_size_tests;

#[cfg(test)]
mod width_probe_tests;
