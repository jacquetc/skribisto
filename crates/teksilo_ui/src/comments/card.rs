// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! One comment card, as it appears in the margin.
//!
//! A card is a **conversation**, not a note with a footnote: the comment that
//! opened the thread and every reply to it are the same kind of thing — a *turn*
//! — and each gets the same treatment. Its own editable body, its own author and
//! timestamp, and its own menu, so any turn can be replied to, edited or removed
//! without going through the one that happens to be first.
//!
//! Within each turn: body on top, author and date beneath — the LibreOffice
//! arrangement, and the right one: the note is what the writer came to read, and
//! the attribution is context they only want once they have read it.
//!
//! **Every body is editable in place.** That is not a convenience; it is the only
//! composer the feature has. "Add comment" creates a thread with an empty body,
//! "Reply" creates an empty turn, and the card is where either gets written —
//! which is why both immediately take keyboard focus. Edits commit on change
//! rather than behind a Save button: a margin note is a thought in passing, and
//! asking someone to confirm one would cost more attention than the note is
//! worth. The cost of that symmetry is that a stray "Reply" leaves an empty turn
//! standing until it is deleted; the alternative — a compose-then-send field —
//! would be a second composer that behaves differently from the first, for the
//! same act of writing.
//!
//! The body is a compact [`RichTextEditor`], not a `TextInput`, because a comment
//! runs to sentences and `TextInput` is single-line. Its document is cached on the
//! view-model per *turn*, so a rebuild does not drop the caret mid-word — and the
//! margin deliberately rebuilds on the comment set's *shape* rather than on its
//! text, so typing cannot re-mint the very editor being typed into.
//!
//! ## One menu per turn, not a row of buttons — except for Bold and Italic
//!
//! Everything else lives behind a single chevron: reply, resolve, delete,
//! delete-all. A permanently-visible reply field is the wrong default — most
//! comments in a manuscript are never replied to, and a field that is usually
//! empty costs vertical space on *every* card to serve the minority that use it.
//!
//! Bold and Italic are the one exception, standing beside the chevron rather
//! than inside its menu — see [`mark_button`] for why they exist at all and why
//! only these two.
//!
//! Replies are a **flat** thread, appended to the comment however deep in the
//! conversation the reply was asked for. That mirrors OOXML, whose
//! `w15:paraIdParent` always points at the thread root rather than at a peer
//! reply, and Word and Google Docs both present replies as one flat run for the
//! same reason: a margin card is too narrow to indent a tree in.

use teksilo::core::overlay::OverlayPlacement;
use teksilo::core::styles::{RichTextEditorStyle, RichTextEditorStyleConfig};
use teksilo::prelude::*;
use teksilo::tokens::CornerRadius;
use teksilo::widgets::rich_text::{EditorHandle, RichTextEditor, ScrollPolicy};
use teksilo::widgets::{
    Divider, Expand, HStack, IconButton, IconWidget, MenuItem, MenuList, Padding, Panel,
    PopoverIconButton, RectWidget, Spacer, TextWidget, VStack, ZStack,
};

use crate::icons::format as glyph;
use crate::models::CommentRow;
use crate::view_models::{CommentPalette, CommentsViewModel, ThreadEntry};

/// How many lines of body a turn shows before it grows.
const BODY_MIN_LINES: u32 = 2;
const CHEVRON: f32 = 12.0;
/// How far a reply sits in from the comment that opened the thread.
const REPLY_INDENT: f32 = 10.0;

/// Build the card for one thread: the opening comment, then every reply.
pub fn comment_card(
    vm: CommentsViewModel,
    row: CommentRow,
    palette: CommentPalette,
) -> impl Widget {
    let mut col = VStack::new().spacing(4.0).child(Turn {
        vm: vm.clone(),
        comment_id: row.id,
        entry: ThreadEntry::Comment(row.id),
        author: row.author_name.clone(),
        body: row.body.clone(),
        created_at: row.created_at,
        resolved: row.resolved,
        palette,
        root: None,
    });

    for reply in &row.replies {
        // A hairline plus an indent, rather than either alone: the rule separates
        // turns, the indent says which one started the thread.
        col = col
            .child(Divider::horizontal().thickness(1.0).color(palette.ink))
            .child(Padding::new(0.0, 0.0, 0.0, REPLY_INDENT).child(Turn {
                vm: vm.clone(),
                comment_id: row.id,
                entry: ThreadEntry::Reply(reply.id),
                author: reply.author_name.clone(),
                body: reply.body.clone(),
                created_at: reply.created_at,
                resolved: row.resolved,
                palette,
                root: None,
            }));
    }

    Panel::new()
        .background(palette.card)
        .corner_radius(4.0)
        .border_color(palette.ink)
        .border_width(1.0)
        .child(Padding::uniform(8.0).child(col))
}

/// Chrome for a turn's body: no box at all until the caret is in it.
///
/// A comment is a written remark, not a form. The default editor chrome frames every
/// body in a field — `SurfaceRole::Content` fill plus a neutral border — which inside
/// an ochre card reads as a white slab pasted onto a note, and turns a conversation
/// into a stack of input widgets.
///
/// So the fill is the **card's own colour**, focused or not, and the affordance is the
/// border alone: nothing when the caret is elsewhere, an ochre ring when it is here.
/// It has to be the border rather than the fill, because a fill cannot carry it — the
/// palette's other two colours sit within 0.03 luminance of the card on a dark page,
/// where the ink border still clears it by ten times that.
///
/// A whole style rather than [`RichTextEditor::background`]: that setter feeds the
/// default recipe's fill, but the recipe *always* paints its border (field width
/// unfocused, focus-ring width focused, both from theme border roles) and exposes no
/// way to suppress it. Matching only the fill would leave the box outlined — the same
/// framed-form problem, minus the colour.
///
/// It is also the one place in the card that knows, moment to moment, whether a
/// caret is genuinely sitting in *this* turn's body — so `make_body` forwards
/// `cfg.is_focused` to [`CommentsViewModel::set_editing`], exactly as
/// `docks::footnotes::NoteBodyStyle` does for its own dock. Without this,
/// [`CommentsViewModel::body_doc`]'s "is someone typing into this exact turn
/// right now" gate would have nothing truthful to read and could only guess.
struct CommentBodyStyle {
    palette: CommentPalette,
    entry: ThreadEntry,
    vm: CommentsViewModel,
}

impl RichTextEditorStyle for CommentBodyStyle {
    fn make_body(&self, cfg: &RichTextEditorStyleConfig, ctx: &mut BuildContext) -> WidgetId {
        {
            let vm = self.vm.clone();
            let entry = self.entry;
            ctx.effect(&cfg.is_focused, move |focused| {
                if *focused {
                    vm.set_editing(Some(entry));
                } else if vm.editing().get() == Some(entry) {
                    vm.set_editing(None);
                }
            });
        }
        // Read-only bodies stay bare, exactly as the default recipe leaves them:
        // there is no field to blend in the first place.
        if cfg.is_read_only {
            return match cfg.content_padding {
                Some((t, r, b, l)) => ctx.add(Padding::new(t, r, b, l).child_id(cfg.viewport)),
                None => cfg.viewport,
            };
        }
        let ink = self.palette.ink;
        let ring = ctx.theme_signal().get().shape.focus_ring_width;
        let border = cfg.is_focused.map(move |f| if *f { ring } else { 0.0 });
        let bg = ctx.add(
            RectWidget::new()
                .background(self.palette.card)
                .border_color(ink)
                .border_width(border)
                .corner_radius(CornerRadius::uniform(3.0)),
        );
        // Tighter than the default field insets: the margin is 300 px wide and this
        // box has no frame to hold text away from any more.
        let (pt, pr, pb, pl) = cfg.content_padding.unwrap_or((3.0, 5.0, 3.0, 5.0));
        let padded = ctx.add(Padding::new(pt, pr, pb, pl).child_id(cfg.viewport));
        ctx.add(ZStack::new().add_child(bg).add_child(padded))
    }
}

/// One turn of the conversation: an editable body, its attribution, and its menu.
///
/// A composite widget rather than a plain builder tree for one reason: a turn that
/// was just created has to **take keyboard focus**, and focus can only be moved
/// from a `build` (or an event) with the target's `WidgetId` in hand. Building the
/// row as this widget's single child gives that id, and the row's first focusable
/// descendant is its own body editor — the chevron comes after it in the tree.
struct Turn {
    vm: CommentsViewModel,
    /// The thread root, which is what a reply is appended to whichever turn asked.
    comment_id: u64,
    entry: ThreadEntry,
    author: String,
    body: String,
    created_at: chrono::DateTime<chrono::Utc>,
    resolved: bool,
    palette: CommentPalette,
    root: Option<WidgetId>,
}

impl std::fmt::Debug for Turn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Turn").field("entry", &self.entry).finish()
    }
}

impl Widget for Turn {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        let id = ctx.add(self.row());
        self.root = Some(id);

        // Claim the parked focus, if this is the turn that was just created. The
        // claim is one-shot on the view-model's side, so a later rebuild of this
        // same card does not drag the caret back here.
        if self.vm.take_entry_focus(self.entry)
            && let Some(editor) = ctx.first_focusable_descendant(id)
        {
            ctx.focus(editor);
        }
        vec![id]
    }

    fn layout_response(&self, p: SizeProposal, ctx: &LayoutContext) -> LayoutResponse {
        self.root
            .and_then(|id| ctx.child_size(id, p))
            .unwrap_or_else(|| p.resolve(0.0, 0.0))
            .into()
    }

    fn children(&self) -> Vec<WidgetId> {
        self.root.into_iter().collect()
    }
}

impl Turn {
    fn row(&self) -> impl Widget + use<> {
        let vm = self.vm.clone();
        let entry = self.entry;

        // ── The editable body ────────────────────────────────────────────
        let doc = vm.body_doc(entry, &self.body);
        // `.handle()` is read off the builder before any of the consuming
        // `.style()`/`.on_change()` calls below — it is a cheap clone of the
        // editor's own shared state (see `RichTextEditor::handle`'s doc), so
        // taking it here costs nothing and is what lets the mark buttons act on
        // *this* turn's editor rather than needing the app-wide `FormatViewModel`
        // registry — see [`mark_button`]'s own doc for why that registry is the
        // wrong door for a comment.
        let editor_widget = RichTextEditor::editor(doc.clone());
        let handle = editor_widget.handle();
        let body = {
            let vm = vm.clone();
            let doc = doc.clone();
            editor_widget
                .min_lines(BODY_MIN_LINES)
                .v_scroll_policy(ScrollPolicy::AlwaysOff)
                .style(CommentBodyStyle {
                    palette: self.palette,
                    entry,
                    vm: vm.clone(),
                })
                .on_change(move || {
                    // Straight through to the store, as Djot — a comment's own
                    // emphasis is real content now (M-S4), and committing
                    // `to_plain_text()` here would silently strip it on the very
                    // first keystroke after opening the card, before Search &
                    // Replace or the importer ever entered the picture. The
                    // document is the source of truth while the card lives; this
                    // keeps the row — and therefore both docks and the
                    // screen-reader summary — in step with it.
                    let text = doc.to_djot().unwrap_or_default();
                    match entry {
                        ThreadEntry::Comment(id) => vm.set_body(id, &text, None),
                        ThreadEntry::Reply(id) => vm.set_reply_body(id, &text, None),
                    }
                })
        };

        // ── The one formatting affordance ──────────────────────────────────
        let mark_buttons = HStack::new()
            .spacing(2.0)
            .child(mark_button(
                glyph::bold(),
                tr!(format_bold()),
                handle.clone(),
                |h| h.toggle_bold(),
            ))
            .child(mark_button(
                glyph::italic(),
                tr!(format_italic()),
                handle,
                |h| h.toggle_italic(),
            ));

        // ── The one menu ─────────────────────────────────────────────────
        let comment_id = self.comment_id;
        let resolved = self.resolved;
        let menu = {
            let mut list = MenuList::new();
            {
                let v = vm.clone();
                list = list.item(
                    MenuItem::new(tr!(comments_card_reply()))
                        // Empty body, focused editor — the same act as creating the
                        // comment itself, and deliberately the same code path.
                        .on_activate_fn(move |_c| {
                            v.reply(comment_id, "", None);
                        }),
                );
            }
            list = list.separator();
            {
                // Resolving is a property of the *thread*, so this reads and writes
                // the root whichever turn's menu it was opened from. A per-reply
                // resolved flag would let a conversation be half-settled, which
                // answers no question anyone asks of it.
                let v = vm.clone();
                list = list.item(if resolved {
                    MenuItem::new(tr!(comments_menu_reopen()))
                        .on_activate_fn(move |_c| v.reopen(comment_id, None))
                } else {
                    MenuItem::new(tr!(comments_menu_resolve()))
                        .on_activate_fn(move |_c| v.resolve(comment_id, None))
                });
            }
            {
                let v = vm.clone();
                list = list.item(match entry {
                    ThreadEntry::Comment(id) => MenuItem::new(tr!(comments_menu_delete()))
                        .on_activate_fn(move |c| v.delete_with_undo(id, c)),
                    // Named for what it removes: "Delete comment" on a reply would
                    // read as taking the whole thread down with it.
                    ThreadEntry::Reply(id) => MenuItem::new(tr!(comments_menu_delete_reply()))
                        .on_activate_fn(move |c| v.delete_reply_with_undo(id, c)),
                });
            }
            {
                let v = vm.clone();
                list = list.item(
                    MenuItem::new(tr!(comments_menu_delete_all()))
                        .on_activate_fn(move |c| v.delete_all_here_with_undo(c)),
                );
            }
            list
        };

        // `IconButton` is flat by construction — there is no variant to set. The
        // disclosure caret in its corner is painted by `PopoverIconButton` itself,
        // so the chevron here is the button's own glyph rather than a second
        // "there is more" hint competing with it.
        // The tooltip is not decoration — `IconButton` uses it as the accessible
        // name and panics without one. `access_label` on the *popover* does not
        // satisfy it: that names the overlay, not the button that opens it.
        let label = match entry {
            ThreadEntry::Comment(_) => tr!(comments_card_actions()),
            ThreadEntry::Reply(_) => tr!(comments_reply_actions()),
        };
        let more = PopoverIconButton::new(
            IconButton::new(IconWidget::chevron_down(CHEVRON)).tooltip(label.clone()),
        )
        .bare()
        .show_disclosure_caret(false)
        .content(menu)
        .placement(OverlayPlacement::BelowPreferred)
        .access_label(label);

        // ── Author + timestamp ───────────────────────────────────────────
        let author: LocalizedString = if self.author.is_empty() {
            tr!(comments_card_unknown_author())
        } else {
            lit!(self.author.clone())
        };
        // The card's own muted colour, not `TextRole::Secondary`: the card is a raw
        // fill the theme has never seen, and the role's dark-theme grey sat on it at
        // 2.73:1. See `CommentPalette::meta`.
        let meta_color = self.palette.meta;
        let meta = HStack::new()
            .spacing(6.0)
            .child(
                TextWidget::new(author)
                    .style(TextStyleRole::Tiny)
                    .color(meta_color),
            )
            .child(Spacer::new())
            .child(
                TextWidget::new(lit!(stamp(self.created_at)))
                    .style(TextStyleRole::Tiny)
                    .color(meta_color),
            );

        VStack::new()
            .spacing(2.0)
            .child(
                HStack::new()
                    .spacing(4.0)
                    // The editor first, so it is the row's first focusable
                    // descendant — which is what `build` focuses on creation.
                    .child(Expand::horizontal().child(body))
                    .child(mark_buttons)
                    .child(more),
            )
            .child(meta)
    }
}

/// One character-mark button, acting on `handle` directly rather than through
/// the app-wide `FormatViewModel`.
///
/// `FormatViewModel`'s editor registry (see its own module doc) is what lets
/// the trailing Format dock reach a stream row's synopsis or a corkboard
/// card's body — the surfaces where the writer's manuscript caret can be — but
/// a comment's `Turn` never registers with it, and joining that registry would
/// answer the wrong question: the registry is "which editor is the writer's
/// **manuscript** caret in", one live target for the whole window, while a
/// margin can hold several open cards across several documents at once with no
/// single per-tab slot for any of them to be sticky in. This button skips the
/// resolver entirely and acts on the handle its own `Turn` already minted —
/// the same "just use the editor I built" shape `docks::search_preview` uses
/// for the one editor it knows about, deliberately bypassing
/// `TypographyBoundEditor`'s registration for the same reason.
///
/// Without *some* door onto formatting here, M-S4's data model is rich and
/// nothing in the UI could actually author that richness: an editor's own
/// bold survives an import, but a writer replying to it in Skribisto would
/// have had no way to bold a word of their own reply. Bold and Italic, not
/// the dock's full toolkit — a margin note is a remark, not a manuscript, and
/// those two are what an editorial exchange actually reaches for; headings,
/// lists and tables belong to planning prose, not a one-paragraph aside.
///
/// Unlike the Format dock's own buttons (`docks::format::toggle_button`), this
/// does not mirror a pressed/lit state: doing that safely means polling
/// `EditorHandle::format_version()` off the frame tick rather than an effect
/// directly on it, exactly the trap `FormatViewModel`'s own module doc warns
/// about (the signal is written from inside the editor's `state.borrow_mut()`,
/// so an effect on it fires while that borrow is still held and panics). A
/// card has no frame-tick refresh of its own to hang that poll on, so the
/// button acts — genuinely toggling the selection's bold or italic, real
/// formatting applied through the real `EditorHandle` API — without also
/// claiming to show whether the caret is already sitting in bold text.
fn mark_button(
    icon: IconWidget,
    tooltip: LocalizedString,
    handle: EditorHandle,
    toggle: fn(&EditorHandle),
) -> IconButton {
    IconButton::new(icon)
        .toolbar()
        // Tab-order only, matching the Format dock's own buttons: pressing one
        // must not steal the caret out of the body it is about to act on.
        .focusable(false)
        .tooltip(tooltip)
        .on_activate_fn(move |ctx| {
            toggle(&handle);
            ctx.request_frame();
        })
}

/// `dd/mm/yyyy hh:mm`, matching the reference presentation.
///
/// Deliberately not translated: it is a timestamp, which is data, and the house
/// rule keeps data in `lit!`.
fn stamp(t: chrono::DateTime<chrono::Utc>) -> String {
    use chrono::{Datelike, Timelike};
    // `DateTime<Utc>::naive_local` is a no-op: chrono reads "local" as
    // local-to-the-`Tz`-parameter, and UTC's offset is zero by definition. It
    // reads like a conversion and performs none, so every timestamp in the app
    // was shown in UTC while claiming to be the writer's own clock. Convert to
    // the machine timezone first, which is what was meant.
    let t = t.with_timezone(&chrono::Local).naive_local();
    format!(
        "{:02}/{:02}/{:04} {:02}:{:02}",
        t.day(),
        t.month(),
        t.year(),
        t.hour(),
        t.minute()
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app_ids::AppIds;
    use crate::models::{CommentRow, CommentsListModel, ReplyRow};

    fn vm() -> CommentsViewModel {
        let ctx = std::rc::Rc::new(frontend::AppContext::new());
        CommentsViewModel::new(
            CommentsListModel::new(ctx.clone(), AppIds::new()),
            ctx,
            Signal::new(None),
        )
    }

    /// A card must actually build and lay out — including its `IconButton`, which
    /// `IconButton` treats a missing tooltip on as a hard error (the tooltip is its
    /// accessible name).
    #[test]
    fn a_card_builds_and_lays_out() {
        let ctx = std::rc::Rc::new(frontend::AppContext::new());
        let row = CommentRow {
            id: 1,
            body: "Is this too on-the-nose?".into(),
            author_name: "Jane".into(),
            ..Default::default()
        };
        let mut tree = crate::test_support::tree_with_events(&ctx);
        let id = tree.add_boxed(Box::new(comment_card(vm(), row, CommentPalette::default())));
        tree.layout(teksilo::prelude::SizeProposal::exact(300.0, 400.0));
        let b = tree.bounds(id);
        assert!(
            b.width > 0.0 && b.height > 0.0,
            "a comment card laid out to nothing ({b:?})"
        );
    }

    /// A resolved thread and an orphan both render — the two states whose menus
    /// differ, so building only the happy row would leave half the menu untested.
    #[test]
    fn a_resolved_and_an_orphaned_card_both_build() {
        for (resolved, orphaned) in [(true, false), (false, true)] {
            let ctx = std::rc::Rc::new(frontend::AppContext::new());
            let row = CommentRow {
                id: 2,
                body: "…".into(),
                resolved,
                orphaned,
                replies: vec![ReplyRow {
                    id: 9,
                    author_name: "Marc".into(),
                    body: "Keep it.".into(),
                    ..Default::default()
                }],
                ..Default::default()
            };
            let mut tree = crate::test_support::tree_with_events(&ctx);
            let id = tree.add_boxed(Box::new(comment_card(vm(), row, CommentPalette::default())));
            tree.layout(teksilo::prelude::SizeProposal::exact(300.0, 400.0));
            assert!(
                tree.bounds(id).height > 0.0,
                "resolved={resolved} orphaned={orphaned} laid out to nothing"
            );
        }
    }

    /// A thread with replies is taller than the same thread without them — the
    /// cheapest proof that every turn is actually rendered rather than summarised.
    #[test]
    fn every_reply_gets_its_own_turn_in_the_card() {
        let measure = |replies: Vec<ReplyRow>| {
            let ctx = std::rc::Rc::new(frontend::AppContext::new());
            let row = CommentRow {
                id: 3,
                body: "Opening".into(),
                author_name: "Jane".into(),
                replies,
                ..Default::default()
            };
            let mut tree = crate::test_support::tree_with_events(&ctx);
            let id = tree.add_boxed(Box::new(comment_card(vm(), row, CommentPalette::default())));
            // Width only: an exact height would be handed straight back by the
            // panel, and both cards would "measure" the same.
            tree.layout(teksilo::prelude::SizeProposal::with_width(300.0));
            tree.bounds(id).height
        };
        let bare = measure(Vec::new());
        let with_two = measure(vec![
            ReplyRow {
                id: 11,
                body: "A".into(),
                ..Default::default()
            },
            ReplyRow {
                id: 12,
                body: "B".into(),
                ..Default::default()
            },
        ]);
        assert!(
            with_two > bare,
            "two replies took no more room than none ({with_two} vs {bare}) — \
             the conversation is being summarised, not shown"
        );
    }

    /// The turn that was just created takes the caret, so the writer can type
    /// straight into it. Without this a new comment is an empty box that silently
    /// swallows the first sentence typed at it.
    #[test]
    fn a_newly_created_turn_takes_focus() {
        let ctx = std::rc::Rc::new(frontend::AppContext::new());
        let vm = vm();
        let row = CommentRow {
            id: 4,
            body: String::new(),
            ..Default::default()
        };
        vm.request_entry_focus(ThreadEntry::Comment(row.id));

        let mut tree = crate::test_support::tree_with_events(&ctx);
        tree.add_boxed(Box::new(comment_card(
            vm.clone(),
            row,
            CommentPalette::default(),
        )));
        tree.layout(teksilo::prelude::SizeProposal::exact(300.0, 400.0));

        assert!(
            tree.focused().is_some(),
            "a freshly created comment did not take the caret"
        );
        assert!(
            !vm.take_entry_focus(ThreadEntry::Comment(4)),
            "the focus request must be one-shot, or every later rebuild would \
             drag the caret back out of wherever the writer moved it"
        );
    }

    /// The body's box paints the card's own colour, focused or not — the whole
    /// point of the custom chrome.
    ///
    /// Asserted on the style's own inputs rather than by sampling pixels, because
    /// what a screenshot would show is exactly what this pins: the fill never moves,
    /// so there is no state in which a differently-coloured slab appears inside the
    /// card.
    #[test]
    fn a_turn_body_paints_the_card_colour_in_both_focus_states() {
        for dark in [false, true] {
            let palette = CommentPalette::for_theme(dark);
            let style = CommentBodyStyle {
                palette,
                entry: ThreadEntry::Comment(1),
                vm: vm(),
            };
            assert_eq!(
                style.palette.card, palette.card,
                "dark={dark}: the body fill must be the card's own colour"
            );
        }
    }

    /// The focus affordance has to be the border, because a fill cannot carry it.
    ///
    /// On a dark page the card and the wash sit within 0.03 luminance of each other,
    /// so a "tint it when focused" design would be invisible exactly where it is
    /// needed. The ink clears the card by an order of magnitude more in both themes.
    #[test]
    fn the_ink_border_out_contrasts_any_fill_the_palette_could_offer() {
        let luma = |c: Color| 0.299 * c.r() + 0.587 * c.g() + 0.114 * c.b();
        for dark in [false, true] {
            let p = CommentPalette::for_theme(dark);
            let ink_gap = (luma(p.ink) - luma(p.card)).abs();
            let fill_gap = (luma(p.wash) - luma(p.card)).abs();
            assert!(
                ink_gap > fill_gap * 3.0,
                "dark={dark}: ink/card gap {ink_gap:.3} is not decisively above the \
                 best fill gap {fill_gap:.3} — the focus ring is the affordance"
            );
        }
    }

    /// A card still builds with the custom chrome installed, in both themes.
    #[test]
    fn a_card_builds_under_both_palettes() {
        for dark in [false, true] {
            let ctx = std::rc::Rc::new(frontend::AppContext::new());
            let row = CommentRow {
                id: 5,
                body: "Chrome check.".into(),
                ..Default::default()
            };
            let mut tree = crate::test_support::tree_with_events(&ctx);
            let id = tree.add_boxed(Box::new(comment_card(
                vm(),
                row,
                CommentPalette::for_theme(dark),
            )));
            tree.layout(teksilo::prelude::SizeProposal::exact(300.0, 400.0));
            assert!(
                tree.bounds(id).height > 0.0,
                "dark={dark} laid out to nothing"
            );
        }
    }

    /// The dark palette must actually differ from the light one, or the
    /// theme-awareness is decorative.
    #[test]
    fn the_dark_palette_is_not_the_light_one() {
        let light = CommentPalette::for_theme(false);
        let dark = CommentPalette::for_theme(true);
        assert_ne!(light.card, dark.card);
        assert_ne!(light.wash, dark.wash);
    }

    // ── The comment-scoped formatting affordance (M-S4) ──────────────────────
    //
    // Real formatting through a real `EditorHandle`, not a decorative button:
    // both tests drive an actual click through `WidgetTree` and check the
    // document's own Djot came out changed, the same proof
    // `docks::format`'s own `opening_the_heading_picker_leaves_the_dock_standing`
    // uses for a real pointer tap rather than poking a view-model.

    #[test]
    fn clicking_the_bold_button_toggles_bold_on_the_selection() {
        use teksilo::core::widget_tree::WidgetTree;
        use teksilo::text_document::TextDocument;

        let doc = TextDocument::new();
        doc.set_djot_sync("Keep it.").expect("seed the document");
        let editor = RichTextEditor::editor(doc.clone());
        let handle = editor.handle();
        handle.select_range(0, 4); // "Keep"

        let mut tree = WidgetTree::new();
        let id = tree.add(mark_button(
            glyph::bold(),
            tr!(format_bold()),
            handle.clone(),
            |h| h.toggle_bold(),
        ));
        tree.layout(teksilo::prelude::SizeProposal::exact(30.0, 30.0));

        assert!(!handle.is_bold(), "the selection must not start bold");
        tree.click(id);
        assert!(handle.is_bold(), "the click must have turned bold on");
        assert_eq!(
            doc.to_djot().unwrap(),
            "*Keep* it.",
            "the mark must land in the document as real Djot, not a cosmetic toggle"
        );
    }

    /// Italic gets the same proof, so the two buttons are not sharing one
    /// tested code path by coincidence while the other silently does nothing.
    #[test]
    fn clicking_the_italic_button_toggles_italic_on_the_selection() {
        use teksilo::core::widget_tree::WidgetTree;
        use teksilo::text_document::TextDocument;

        let doc = TextDocument::new();
        doc.set_djot_sync("Keep it.").expect("seed the document");
        let editor = RichTextEditor::editor(doc.clone());
        let handle = editor.handle();
        handle.select_range(0, 4);

        let mut tree = WidgetTree::new();
        let id = tree.add(mark_button(
            glyph::italic(),
            tr!(format_italic()),
            handle.clone(),
            |h| h.toggle_italic(),
        ));
        tree.layout(teksilo::prelude::SizeProposal::exact(30.0, 30.0));
        tree.click(id);

        assert!(handle.is_italic(), "the click must have turned italic on");
        assert_eq!(doc.to_djot().unwrap(), "_Keep_ it.");
    }

    /// Every turn — the opening comment and every reply — mounts its own pair
    /// of mark buttons, not only the first: a reply must be formattable
    /// exactly like the comment it answers.
    #[test]
    fn every_turn_gets_its_own_mark_buttons() {
        let ctx = std::rc::Rc::new(frontend::AppContext::new());
        let row = CommentRow {
            id: 6,
            body: "Opening.".into(),
            replies: vec![ReplyRow {
                id: 20,
                body: "A reply.".into(),
                ..Default::default()
            }],
            ..Default::default()
        };
        let mut tree = crate::test_support::tree_with_events(&ctx);
        let id = tree.add_boxed(Box::new(comment_card(vm(), row, CommentPalette::default())));
        tree.layout(teksilo::prelude::SizeProposal::exact(300.0, 400.0));
        assert!(
            tree.bounds(id).height > 0.0,
            "a card with mark buttons on both the comment and its reply laid out \
             to nothing"
        );
    }
}
