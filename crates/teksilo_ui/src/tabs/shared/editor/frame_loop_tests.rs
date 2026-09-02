// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

use super::*;
use frontend::AppContext;
use teksilo::core::widget_tree::WidgetTree;
use teksilo::widgets::rich_text::CommandFilter;

use crate::app_ids::AppIds;
use crate::models::TextReplacementRuleListModel;
use crate::settings::TextReplacementRulesViewModel;
use crate::singles::SingleWork;
use crate::text_replacement::typography::SmartPunctuationFlags;

fn typo() -> EditorTypography {
    EditorTypography {
        font_family: Signal::new("Literata".to_string()),
        size: Signal::new(1.0),
        line_height: Signal::new(1.5),
        first_line_indent: Signal::new(0.0),
        para_spacing_before: Signal::new(0.0),
        para_spacing_after: Signal::new(0.0),
        size_range: crate::settings::TypographySizeRange::default(),
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
    column_with(false)
}

/// [`column`], with the read-only gate a trashed item's tab sets.
fn column_with(
    read_only: bool,
) -> (
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
    let find = crate::search::FindViewModel::new(doc.clone());
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
        // No app around this tree, so no writing game either.
        None,
        None,
        // No project around this tree, so no comment binding: the margin
        // collapses to nothing and the column lays out on its own.
        None,
        // …nor a footnote binding, an image source, or a project to tally
        // typing against, for the same reason.
        None,
        None,
        None,
        read_only,
        // No project around this tree, so no item to name.
        None,
        false,
        // No project behind a frame-loop probe, so no palette and no capture menu.
        None,
        // Nothing private on this probe's editor: the default `all()`.
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

// ── a trashed item's text is read-only ──────────────────────────────────
//
// The banner over a trashed tab is a *statement*. It was stacked above the
// content as a sibling and gated nothing: the prose beneath it was built by
// the same editable render path as any other tab, so a writer could keep
// typing into a scene the app was simultaneously telling them was deleted —
// and those edits were saved.

/// The gated column stands up and lays out — the regression guard for the
/// branch itself, which builds a different widget subtree (no background
/// rect, no `ZStack`; see `WritingEditorStyle::make_body`).
#[test]
fn a_trashed_items_column_builds_and_lays_out() {
    let (doc, _handle, _session, mut tree) = column_with(true);
    frame(&mut tree);
    assert_eq!(
        plain(&doc),
        "",
        "a fresh document, laid out without panicking"
    );
}

/// **What the gate does and does not stop.**
///
/// `read_only` is a policy on the *input* paths — it leaves the IME
/// descriptor unset and refuses drops — so a writer cannot type into a
/// trashed item, which is the bug this fixes. It is deliberately **not** a
/// policy on `EditorHandle`, whose `insert_text` writes straight to the
/// cursor; that is the API paste, Insert footnote and a version restore all
/// go through, and gating it here would break restoring *into* the trash.
///
/// Pinned so the distinction is a decision on record rather than a surprise
/// the next person meets while debugging.
#[test]
fn the_gate_stops_typing_and_not_the_programmatic_api() {
    let (doc, handle, _session, mut tree) = column_with(true);
    handle.insert_text("a");
    frame(&mut tree);
    assert_eq!(
        plain(&doc),
        "a",
        "the handle API is intentionally outside the read-only policy — a \
             command that must respect the trash has to check it itself",
    );
}

// ── writing games freeze the surfaces they cover ────────────────────────
//
// The app half of "Always forward". Teksilo owns *what* a frozen editor
// refuses (see its `forward_only_*` tests); these pin that Skribisto puts the
// right editors under it, that switching the game reaches editors that are
// **already mounted**, and that the per-surface options are honoured.

/// A prose column carrying a game, so the wiring under test is the real one.
fn prose_column_playing(
    games: &crate::writing_session::WritingGamesViewModel,
) -> (EditorHandle, WidgetTree) {
    let doc = TextDocument::new();
    let find = crate::search::FindViewModel::new(doc.clone());
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
        None,
        None,
        Some(games.clone()),
        None,
        None,
        None,
        None,
        // No project around this tree to tally typing against.
        None,
        false,
        // No project around this tree, so no item to name.
        None,
        false,
        // No project behind a frame-loop probe, so no palette and no capture menu.
        None,
        // Nothing private on this probe's editor: the default `all()`.
        None,
    );
    let mut tree = WidgetTree::new();
    tree.add(col);
    tree.layout(SizeProposal::exact(900.0, 600.0));
    let handle = find
        .editor_handle()
        .expect("writing_column attached its handle");
    (handle, tree)
}

#[test]
fn a_prose_editor_is_frozen_while_always_forward_is_played() {
    let games = crate::writing_session::WritingGamesViewModel::detached();
    games.set_always_forward(true);
    let (handle, _tree) = prose_column_playing(&games);
    assert_eq!(
        handle.command_filter(),
        CommandFilter::ForwardOnly,
        "an editor built while the game is on must be frozen from its first keystroke"
    );
}

#[test]
fn a_prose_editor_is_untouched_while_no_game_is_played() {
    let games = crate::writing_session::WritingGamesViewModel::detached();
    let (handle, _tree) = prose_column_playing(&games);
    assert_eq!(handle.command_filter(), CommandFilter::All);
}

/// The case the feature exists for: a writer switches the game on *while
/// looking at the page*. The mounted editor must follow without a rebuild.
#[test]
fn switching_the_game_reaches_an_already_mounted_editor() {
    let games = crate::writing_session::WritingGamesViewModel::detached();
    let (handle, mut tree) = prose_column_playing(&games);
    assert_eq!(handle.command_filter(), CommandFilter::All);

    games.set_always_forward(true);
    frame(&mut tree);
    assert_eq!(
        handle.command_filter(),
        CommandFilter::ForwardOnly,
        "the effect must push the new filter onto the live editor"
    );

    games.set_always_forward(false);
    frame(&mut tree);
    assert_eq!(
        handle.command_filter(),
        CommandFilter::All,
        "and give it back the moment the game ends"
    );
}

/// Turning the prose option off releases the manuscript even mid-game — the
/// option is not merely read once when the game starts.
#[test]
fn the_prose_option_is_honoured_live() {
    let games = crate::writing_session::WritingGamesViewModel::detached();
    games.set_always_forward(true);
    let (handle, mut tree) = prose_column_playing(&games);
    assert_eq!(handle.command_filter(), CommandFilter::ForwardOnly);

    games.forward_in_prose().set(false);
    frame(&mut tree);
    assert_eq!(
        handle.command_filter(),
        CommandFilter::All,
        "prose follows the prose option, not merely the activation"
    );
}

/// The programmatic API stays open while a game is on — the same boundary the
/// trash gate draws, and the reason the replacement engines (smart
/// punctuation, the custom lexicon) keep working mid-game. They rewrite *for*
/// the writer through `EditorHandle`, which no command filter gates.
#[test]
fn a_game_stops_typing_and_not_the_programmatic_api() {
    let games = crate::writing_session::WritingGamesViewModel::detached();
    games.set_always_forward(true);
    let doc = TextDocument::new();
    let find = crate::search::FindViewModel::new(doc.clone());
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
        None,
        None,
        Some(games.clone()),
        None,
        None,
        None,
        None,
        // No project around this tree to tally typing against.
        None,
        false,
        // No project around this tree, so no item to name.
        None,
        false,
        // No project behind a frame-loop probe, so no palette and no capture menu.
        None,
        // Nothing private on this probe's editor: the default `all()`.
        None,
    );
    let mut tree = WidgetTree::new();
    tree.add(col);
    tree.layout(SizeProposal::exact(900.0, 600.0));
    let handle = find.editor_handle().expect("handle attached");

    handle.insert_text("écrit");
    frame(&mut tree);
    assert_eq!(
        plain(&doc),
        "écrit",
        "an engine writing through the handle is not what the game forbids"
    );
    assert_eq!(handle.command_filter(), CommandFilter::ForwardOnly);
}

/// The ungated surface is the same in that respect, so the test above is
/// describing the policy and not a broken harness.
#[test]
fn the_same_column_without_the_gate_still_accepts_typing() {
    let (doc, handle, _session, mut tree) = column_with(false);
    handle.insert_text("a");
    frame(&mut tree);
    assert_eq!(plain(&doc), "a");
}

/// Read-only, not disabled: a deleted scene's words are still the writer's,
/// and selecting them is how they get copied back out.
#[test]
fn a_trashed_item_can_still_be_read_and_selected() {
    let (doc, handle, _session, mut tree) = column_with(true);
    doc.set_djot_sync("The scene that was thrown away.")
        .unwrap();
    frame(&mut tree);
    handle.select_range(4, 9);
    assert_eq!(
        handle.selection(),
        (4, 9),
        "text that cannot be selected cannot be recovered by hand",
    );
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
