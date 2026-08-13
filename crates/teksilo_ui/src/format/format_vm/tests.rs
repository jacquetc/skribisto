// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

use super::*;
use teksilo::text_document::TextDocument;
use teksilo::widgets::rich_text::RichTextEditor;

/// A view-model over one live editor, plus the editor itself so the test can
/// assert against the document directly. No `WidgetTree` — the whole point
/// of keeping the resolver injectable.
fn vm_over(text: &str) -> (FormatViewModel, RichTextEditor) {
    let (vm, editor, _doc) = vm_over_doc(text);
    (vm, editor)
}

/// The document as well, for the assertions `EditorHandle` cannot express —
/// list membership has no `is_in_list()` query, so it has to be read off
/// the block itself.
fn vm_over_doc(text: &str) -> (FormatViewModel, RichTextEditor, TextDocument) {
    let doc = TextDocument::new();
    doc.set_markdown(text)
        .expect("parse")
        .wait()
        .expect("import");
    let editor = RichTextEditor::editor(doc.clone());
    let handle = editor.handle();
    // Scene rather than None so the target reads as real prose; the tests
    // that care about grouping drive `set_surface` themselves.
    let vm = FormatViewModel::new(Rc::new(move || {
        (Some(handle.clone()), FormatSurface::Scene)
    }));
    (vm, editor, doc)
}

/// A view-model with nothing focused.
fn vm_detached() -> FormatViewModel {
    FormatViewModel::new(Rc::new(|| (None, FormatSurface::None)))
}

/// Two distinct `WidgetId`s. It is a slotmap key type, so the only way to
/// mint one is from a slotmap — a throwaway one does fine, and this keeps
/// the registry tests free of a `WidgetTree`.
fn two_ids() -> (WidgetId, WidgetId) {
    let mut keys: slotmap::SlotMap<WidgetId, ()> = slotmap::SlotMap::with_key();
    (keys.insert(()), keys.insert(()))
}

/// A standalone editor over `text`, and its handle.
///
/// Everything is selected up front: `is_bold` and friends probe the format
/// at the **selection start**, so without a selection a toggle only changes
/// the typing format and the character there is still unmarked — the round
/// trip would be invisible. Same reason `toggling_bold_mirrors_the_editors_answer`
/// selects before asserting.
fn loose_editor(text: &str) -> (RichTextEditor, EditorHandle) {
    let doc = TextDocument::new();
    doc.set_markdown(text)
        .expect("parse")
        .wait()
        .expect("import");
    let editor = RichTextEditor::editor(doc);
    let handle = editor.handle();
    editor.select_all();
    (editor, handle)
}

/// The registry's whole reason to exist: a corkboard card and a stream row
/// build editors the per-tab resolver structurally cannot name, so without
/// this the formatting surfaces would act on the wrong document — or on
/// nothing at all.
///
/// Asserted by *effect* rather than by comparing handles: `EditorHandle` has
/// no identity API, and "the command reached this document and not that one"
/// is the property that actually matters.
#[test]
fn a_focused_registered_editor_outranks_the_resolvers_answer() {
    let (_tab_editor, tab_handle) = loose_editor("tab prose");
    let (_card_editor, card_handle) = loose_editor("card synopsis");
    let resolved = tab_handle.clone();
    let vm = FormatViewModel::new(Rc::new(move || {
        (Some(resolved.clone()), FormatSurface::Scene)
    }));
    let (card_id, _) = two_ids();
    vm.register(card_id, card_handle.clone(), EditorKind::Synopsis);

    // Not focused yet: the resolver still owns the answer.
    assert_eq!(vm.target().1, FormatSurface::Scene);
    vm.toggle_bold();
    assert!(
        tab_handle.is_bold(),
        "the resolver's editor took the command"
    );
    assert!(!card_handle.is_bold());

    // The writer clicks into the card. Now the card wins, and it is
    // classified as a synopsis however the resolver classified the tab.
    card_handle.focused_signal().set(true);
    assert_eq!(vm.target().1, FormatSurface::Synopsis);
    vm.toggle_italic();
    assert!(card_handle.is_italic(), "the focused card took the command");
    assert!(!tab_handle.is_italic());
}

/// Opening the Format menu blurs the editor. For a tab the resolver stays
/// sticky on its own; a card has no per-tab slot to be sticky in, so the
/// latch has to carry it — otherwise every Format command would land on the
/// tab's prose instead of the card the writer was editing.
#[test]
fn a_tab_never_typed_in_offers_no_caret_to_insert_at() {
    // `has_target` is true here and rightly so — there IS a document to act
    // on, which is what Save as template… and the Format menu need. But
    // nothing has a caret, so a command that inserts *at the caret* has
    // nowhere to put anything.
    let (_editor, handle) = loose_editor("tab prose");
    let resolved = handle.clone();
    let vm = FormatViewModel::new(Rc::new(move || {
        (Some(resolved.clone()), FormatSurface::Scene)
    }));
    vm.refresh();
    assert!(
        vm.has_target().get(),
        "the resolver found the tab's editor, so there is something to act on"
    );
    assert!(
        !vm.has_caret_target().get(),
        "but nobody has typed in it, so there is no caret to insert at"
    );
}

#[test]
fn a_caret_target_survives_the_menu_taking_focus() {
    // The regression a live-focus gate reintroduces (commit 2d4890b0): the
    // row would grey out at the instant the writer reached for it.
    let (_editor, handle) = loose_editor("tab prose");
    let resolved = handle.clone();
    let vm = FormatViewModel::new(Rc::new(move || {
        (Some(resolved.clone()), FormatSurface::None)
    }));
    let (id, _) = two_ids();
    vm.register(id, handle.clone(), EditorKind::Prose);

    handle.focused_signal().set(true);
    vm.refresh();
    assert!(vm.has_caret_target().get(), "the writer is in the editor");

    // The menu bar takes focus away.
    handle.focused_signal().set(false);
    vm.refresh();
    assert!(
        vm.has_caret_target().get(),
        "opening the menu must not grey the row the writer is reaching for"
    );
}

#[test]
fn a_card_stays_the_target_after_the_menu_takes_focus() {
    let (_tab_editor, tab_handle) = loose_editor("tab prose");
    let (_card_editor, card_handle) = loose_editor("card synopsis");
    let resolved = tab_handle.clone();
    // The live resolver reports `None` for the surface once focus has left
    // the editors — exactly what `App` wires.
    let vm = FormatViewModel::new(Rc::new(move || {
        (Some(resolved.clone()), FormatSurface::None)
    }));
    let (card_id, _) = two_ids();
    vm.register(card_id, card_handle.clone(), EditorKind::Synopsis);

    card_handle.focused_signal().set(true);
    // The per-frame refresh `App` drives off the frame tick. This is what
    // writes the latch, and a focus change always pumps a frame (the caret
    // has to appear), so in the app it has always run by this point.
    vm.refresh();
    // The menu bar takes focus away.
    card_handle.focused_signal().set(false);

    vm.toggle_bold();
    assert!(
        card_handle.is_bold(),
        "the command must still reach the card the writer was in"
    );
    assert!(!tab_handle.is_bold());
    assert_eq!(
        vm.target().1,
        FormatSurface::None,
        "the surface stays live so the dock empties — only the target is sticky"
    );
}

/// A popover the **dock itself** opens takes keyboard focus, so the live
/// resolver reports `None` — the same answer it gives for a click into the
/// binder, and the reason pressing the heading picker used to blank the
/// whole dock. The surface has to survive that, but only that: a real blur
/// must still empty the dock, or the "hide, don't grey" decision would be
/// undone by the first popover anyone added.
#[test]
fn a_popover_the_dock_owns_does_not_blank_it() {
    let (_editor, handle) = loose_editor("scene prose");
    let focused = handle.focused_signal();
    // `App`'s resolver, in miniature: sticky target, live surface.
    let (resolved, live) = (handle.clone(), focused.clone());
    let vm = FormatViewModel::new(Rc::new(move || {
        let surface = if live.get() {
            FormatSurface::Scene
        } else {
            FormatSurface::None
        };
        (Some(resolved.clone()), surface)
    }));

    focused.set(true);
    vm.refresh();
    assert_eq!(vm.surface_signal().get(), FormatSurface::Scene);
    assert!(vm.groups().block.get());

    // The picker opens and takes the focus with it.
    vm.set_dock_overlay_open(true);
    focused.set(false);
    vm.refresh();
    assert_eq!(
        vm.surface_signal().get(),
        FormatSurface::Scene,
        "the dock's own popover borrowing focus is not a blur"
    );
    assert!(
        vm.groups().block.get(),
        "the group holding the open picker must not hide — hiding it \
             dormants the subtree and takes the list down with it"
    );
    assert!(!vm.groups().empty.get());

    // Dismissed: the overlay manager hands focus back to the editor.
    vm.set_dock_overlay_open(false);
    focused.set(true);
    vm.refresh();
    assert_eq!(vm.surface_signal().get(), FormatSurface::Scene);

    // A genuine blur — clicking into the binder — still empties the dock.
    focused.set(false);
    vm.refresh();
    assert_eq!(vm.surface_signal().get(), FormatSurface::None);
    assert!(vm.groups().empty.get());
}

/// The latch holds the *last* surface, it does not pin `Scene`: with the
/// popover somehow still open while focus lands in another editor, the real
/// answer wins. Only the empty answer is overridden.
#[test]
fn the_latch_yields_to_a_real_surface() {
    let (_editor, handle) = loose_editor("synopsis prose");
    let surface = Signal::new(FormatSurface::Scene);
    let (resolved, reported) = (handle.clone(), surface.clone());
    let vm = FormatViewModel::new(Rc::new(move || (Some(resolved.clone()), reported.get())));

    vm.refresh();
    assert!(vm.groups().scene_breaks.get());

    vm.set_dock_overlay_open(true);
    surface.set(FormatSurface::Synopsis);
    vm.refresh();
    assert_eq!(vm.surface_signal().get(), FormatSurface::Synopsis);
    assert!(
        !vm.groups().scene_breaks.get(),
        "a live surface is a real move and must win over the latch"
    );
}

/// The registry entry lives exactly as long as the widget. A stream row
/// scrolled out of existence must not stay formattable.
#[test]
fn unregistering_drops_the_entry_and_the_latch() {
    let (_tab_editor, tab_handle) = loose_editor("tab prose");
    let (_row_editor, row_handle) = loose_editor("row prose");
    let resolved = tab_handle.clone();
    let vm = FormatViewModel::new(Rc::new(move || {
        (Some(resolved.clone()), FormatSurface::Scene)
    }));
    let (row_id, _) = two_ids();
    vm.register(row_id, row_handle.clone(), EditorKind::Prose);
    row_handle.focused_signal().set(true);
    row_handle.focused_signal().set(false);

    // The row is torn down while still holding the latch.
    vm.unregister(row_id);
    assert!(vm.registry.borrow().is_empty());
    vm.toggle_bold();
    assert!(
        tab_handle.is_bold(),
        "with the row gone the resolver's tab editor is the answer again"
    );
    assert!(!row_handle.is_bold());
}

/// A rebuild mints a fresh `EditorState` for the same widget slot, so the
/// registry must re-point rather than accumulate — and the latch with it,
/// or a command would address the widget's previous state.
#[test]
fn rebuilding_repoints_the_entry_instead_of_duplicating_it() {
    let (_first, first_handle) = loose_editor("before rebuild");
    let (_second, second_handle) = loose_editor("after rebuild");
    let vm = vm_detached();
    let (id, _) = two_ids();

    vm.register(id, first_handle.clone(), EditorKind::Prose);
    first_handle.focused_signal().set(true);
    vm.refresh(); // latches, as the frame tick would
    first_handle.focused_signal().set(false);

    // Same widget id, fresh handle — a theme or locale switch is enough.
    vm.register(id, second_handle.clone(), EditorKind::Prose);
    assert_eq!(vm.registry.borrow().len(), 1, "one slot, one entry");
    vm.toggle_bold();
    assert!(
        second_handle.is_bold(),
        "the latch must follow the rebuild, not keep the dead state"
    );
    assert!(!first_handle.is_bold());
}

/// A prose editor cannot tell a scene from a note; the tab can. The
/// registry must not flatten a note into a scene and start offering scene
/// breaks the exporter would ignore.
#[test]
fn a_registered_prose_editor_keeps_the_tabs_note_classification() {
    let (_editor, handle) = loose_editor("a note");
    let resolved = handle.clone();
    let vm = FormatViewModel::new(Rc::new(move || {
        (Some(resolved.clone()), FormatSurface::Note)
    }));
    let (id, _) = two_ids();
    vm.register(id, handle.clone(), EditorKind::Prose);
    handle.focused_signal().set(true);
    assert_eq!(vm.target().1, FormatSurface::Note);
}

#[test]
fn commands_are_inert_when_no_editor_is_focused() {
    // Every command must survive being invoked from a menu that outlived
    // its editor — the menu bar blurs the editor to open, and a tab can
    // close underneath an open overlay.
    let vm = vm_detached();
    vm.toggle_bold();
    vm.toggle_italic();
    vm.toggle_underline();
    vm.toggle_strikethrough();
    vm.clear_formatting();
    vm.set_heading(3);
    vm.set_alignment(ALIGN_CENTER);
    vm.toggle_blockquote();
    vm.insert_bullet_list();
    vm.insert_numbered_list();
    vm.indent();
    vm.outdent();
    vm.insert_table(3, 3);
    vm.insert_row_above();
    vm.remove_table();
    vm.undo();
    vm.redo();
    vm.refresh();

    assert!(!vm.bold().get(), "no editor means no state to mirror");
    assert_eq!(vm.heading().get(), 0);
    assert_eq!(vm.alignment().get(), ALIGN_LEFT);
    assert!(vm.surface_signal().get().is_empty());
}

#[test]
fn toggling_bold_mirrors_the_editors_answer() {
    let (vm, editor) = vm_over("Hello world");
    // `is_bold` probes the format at the selection start, so a selection is
    // what makes the round trip observable — without one, a toggle changes
    // the typing format and the character at the caret is still unbolded.
    editor.select_all();
    vm.sync_now();
    assert!(!vm.bold().get());

    vm.toggle_bold();
    assert!(
        vm.bold().get(),
        "the mirror follows the editor, not a guess"
    );
    assert!(editor.handle().is_bold());

    vm.toggle_bold();
    assert!(!vm.bold().get());
    assert!(!editor.handle().is_bold());
}

#[test]
fn heading_and_alignment_round_trip_through_the_radio_indices() {
    let (vm, editor) = vm_over("Hello");
    assert_eq!(vm.heading().get(), 0);

    vm.set_heading(2);
    assert_eq!(vm.heading().get(), 2);
    assert_eq!(editor.handle().get_heading_level(), 2);

    vm.set_alignment(ALIGN_CENTER);
    assert_eq!(vm.alignment().get(), ALIGN_CENTER);
    assert_eq!(editor.handle().get_alignment(), Alignment::Center);

    vm.set_alignment(ALIGN_LEFT);
    assert_eq!(vm.alignment().get(), ALIGN_LEFT);
}

#[test]
fn an_out_of_range_heading_is_refused_rather_than_clamped() {
    let (vm, editor) = vm_over("Hello");
    vm.set_heading(2);
    vm.set_heading(9);
    assert_eq!(
        editor.handle().get_heading_level(),
        2,
        "H9 is a caller bug; silently giving it H6 would hide it"
    );
}

#[test]
fn an_alignment_we_do_not_offer_lights_no_button() {
    let (vm, editor) = vm_over("Hello");
    // Djot round-trips block alignment, so an imported document can arrive
    // justified even though Skribisto offers no way to ask for it.
    editor.handle().set_alignment(Alignment::Justify);
    vm.sync_now();
    assert_eq!(vm.alignment().get(), ALIGN_OTHER);

    // And it is not a state the user can request.
    vm.set_alignment(ALIGN_OTHER);
    assert_eq!(editor.handle().get_alignment(), Alignment::Justify);

    // Choosing a real alignment replaces it.
    vm.set_alignment(ALIGN_LEFT);
    assert_eq!(vm.alignment().get(), ALIGN_LEFT);
}

#[test]
fn clear_formatting_flattens_marks_and_block_structure() {
    let (vm, editor) = vm_over("Hello world");
    let handle = editor.handle();
    editor.select_all();
    handle.set_bold(true);
    handle.set_italic(true);
    handle.set_heading_level(2);
    handle.set_alignment(Alignment::Center);
    vm.sync_now();
    assert!(vm.bold().get() && vm.italic().get());

    vm.clear_formatting();

    assert!(!handle.is_bold(), "bold cleared");
    assert!(!handle.is_italic(), "italic cleared");
    assert_eq!(handle.get_heading_level(), 0, "heading flattened");
    assert_eq!(handle.get_alignment(), Alignment::Left, "alignment reset");
    assert!(!vm.bold().get(), "and the mirrors followed");
    assert_eq!(vm.heading().get(), 0);
}

#[test]
fn superscript_and_subscript_are_mutually_exclusive_in_the_mirrors() {
    let (vm, editor) = vm_over("H2O");
    editor.handle().select_range(1, 2);
    vm.sync_now();

    vm.toggle_subscript();
    assert!(vm.subscript().get() && !vm.superscript().get());

    vm.toggle_superscript();
    assert!(
        vm.superscript().get() && !vm.subscript().get(),
        "one property, two buttons — both lit would be a lie about the document"
    );

    vm.toggle_superscript();
    assert!(!vm.superscript().get() && !vm.subscript().get());
}

#[test]
fn clear_formatting_is_a_single_undo_entry() {
    // A writer clearing a bold, centred heading means one action. Without
    // the edit block this took five Ctrl+Z presses, and the first one left
    // the paragraph half-cleared.
    let (vm, editor) = vm_over("Hello world");
    let handle = editor.handle();
    editor.select_all();
    handle.set_bold(true);
    handle.set_italic(true);
    handle.set_heading_level(2);
    handle.set_alignment(Alignment::Center);
    vm.sync_now();

    vm.clear_formatting();
    assert!(!handle.is_bold() && handle.get_heading_level() == 0);

    handle.undo();
    vm.sync_now();
    assert!(handle.is_bold(), "one undo restores the marks");
    assert!(handle.is_italic(), "...all of them");
    assert_eq!(handle.get_heading_level(), 2, "...and the block format too");
    assert_eq!(handle.get_alignment(), Alignment::Center);
}

#[test]
fn clear_formatting_takes_the_block_out_of_a_list() {
    // `outdent` bottoms out at depth 0 by design, so before
    // `remove_from_list` existed a cleared paragraph stayed a list item.
    let (vm, editor, doc) = vm_over_doc("item");
    let handle = editor.handle();
    handle.insert_list(false);
    vm.sync_now();
    assert!(
        doc.block_at_position(0).expect("block").list().is_some(),
        "precondition: the block is a list item"
    );

    vm.clear_formatting();
    assert!(
        doc.block_at_position(0).expect("block").list().is_none(),
        "clearing formatting must leave a plain paragraph"
    );
}

#[test]
fn remove_from_list_is_reachable_on_its_own() {
    // The dock offers it as its own control, not only via clear-formatting.
    let (vm, editor, doc) = vm_over_doc("item");
    editor.handle().insert_list(true);
    assert!(doc.block_at_position(0).expect("block").list().is_some());

    vm.remove_from_list();
    assert!(doc.block_at_position(0).expect("block").list().is_none());

    // Outside a list it is a no-op, not an error.
    vm.remove_from_list();
    assert!(doc.block_at_position(0).expect("block").list().is_none());
}

#[test]
fn clear_formatting_flattens_superscript() {
    let (vm, editor) = vm_over("E=mc2");
    let handle = editor.handle();
    handle.select_range(4, 5);
    vm.toggle_superscript();
    assert!(vm.superscript().get());

    vm.clear_formatting();
    assert!(
        !handle.is_superscript(),
        "superscript is a character mark and goes with the rest"
    );
    assert!(!vm.superscript().get());
}

#[test]
fn clear_formatting_terminates_on_a_blockquote() {
    // `decrease_blockquote_depth` is driven blind (depth is not queryable),
    // so the loop must be bounded — this test would hang, not fail, on a
    // regression that let it spin.
    let (vm, editor) = vm_over("Hello");
    editor.handle().toggle_blockquote();
    vm.sync_now();

    vm.clear_formatting();
    assert!(!editor.handle().is_in_blockquote());
    assert!(!vm.blockquote().get());
}

#[test]
fn refresh_re_reads_only_when_the_editor_reports_a_change() {
    let (vm, editor) = vm_over("Hello world");
    let handle = editor.handle();
    editor.select_all();
    vm.refresh();
    assert!(!vm.bold().get());

    // Toggle behind the view-model's back, the way typing Ctrl+B in the
    // editor does. `format_version` is bumped by the editor's *batched*
    // event drain, which the frame loop runs — so with no frame loop it
    // stays put, and `refresh` correctly declines to re-read the document.
    // That is the whole point of the gate: it runs every frame, and an
    // unguarded version would wake every bound widget continuously.
    handle.toggle_bold();
    vm.refresh();
    assert!(
        !vm.bold().get(),
        "no version bump yet, so nothing to re-read"
    );

    // Stand in for the drain the frame loop would have done. In the app the
    // dock calls `refresh` from a frame-tick effect, which fires *after*
    // the editor's own tick closure has drained and released its borrow.
    let version = handle.format_version();
    version.set(version.get() + 1);
    vm.refresh();
    assert!(vm.bold().get(), "a version bump makes refresh re-read");
}

#[test]
fn a_command_updates_the_mirrors_without_waiting_for_a_frame() {
    // Commands routed through the view-model re-sync unconditionally, so a
    // button lights up on click rather than a frame later — and, on a mixed
    // selection, shows the editor's answer rather than `IconButton`'s
    // optimistic flip.
    let (vm, editor) = vm_over("Hello world");
    editor.select_all();
    vm.refresh();

    vm.toggle_bold();
    assert!(
        vm.bold().get(),
        "no format_version bump happened, yet the mirror is current"
    );
    assert!(editor.handle().is_bold());
}

#[test]
fn a_degenerate_table_is_refused() {
    let (vm, editor) = vm_over("Hello");
    vm.insert_table(0, 3);
    vm.insert_table(3, 0);
    assert!(
        !editor.handle().is_in_table(),
        "a zero-sized table is not a table"
    );

    vm.insert_table(2, 2);
    assert!(editor.handle().is_in_table());
    assert!(vm.in_table().get(), "and the table group unlocks");
}

/// A synopsis is a real formatting target, not a second-class one: the
/// commands must reach it, and the surface must say what it is so the dock
/// drops the one group a synopsis has no use for.
///
/// This is the gap that shipped first time round — `App` could only ever
/// resolve a tab's *main prose* handle, so the caret sitting in a synopsis
/// produced `None` and the dock showed its empty state over perfectly
/// formattable text.
#[test]
fn a_synopsis_is_a_formattable_target_of_its_own_kind() {
    let doc = TextDocument::new();
    doc.set_markdown("a synopsis line")
        .expect("parse")
        .wait()
        .expect("import");
    let editor = RichTextEditor::editor(doc);
    editor.select_all();
    let handle = editor.handle();
    let vm = FormatViewModel::new(Rc::new(move || {
        (Some(handle.clone()), FormatSurface::Synopsis)
    }));

    vm.refresh();
    assert_eq!(vm.surface_signal().get(), FormatSurface::Synopsis);

    let g = vm.groups();
    assert!(
        g.marks.get() && g.lists.get() && g.history.get(),
        "a synopsis is prose: marks, lists and history all apply"
    );
    assert!(
        g.block.get() && g.tables.get(),
        "and so do headings, alignment and tables — an outline and a beat \
             sheet are exactly what a synopsis is for"
    );
    assert!(
        !g.scene_breaks.get(),
        "only the scene breaks go, and only because the exporter never \
             scans a synopsis for their markers"
    );
    assert!(!g.empty.get(), "and it is emphatically not the empty state");

    // The commands reach it like any other editor.
    vm.toggle_bold();
    assert!(vm.bold().get(), "bold must apply to a synopsis");
    assert!(editor.handle().is_bold());
}

/// A note's synopsis classifies as `Synopsis`, never `Note` — the structural reason the
/// insert command cannot reach a synopsis box.
#[test]
fn a_notes_synopsis_classifies_as_synopsis_not_note() {
    let vm = FormatViewModel::new(Rc::new(|| (None, FormatSurface::None)));
    assert_eq!(
        vm.classify(EditorKind::Synopsis, FormatSurface::Note),
        FormatSurface::Synopsis,
        "the editor kind wins over the tab's own note-ness"
    );
    assert_eq!(
        vm.classify(EditorKind::Prose, FormatSurface::Note),
        FormatSurface::Note
    );
}

#[test]
fn surface_decides_which_groups_appear() {
    use FormatSurface::*;

    // Scene prose is the only place a scene break means anything — the same
    // predicate the compiler uses to decide what it scans.
    assert!(Scene.shows_scene_breaks());
    assert!(!Note.shows_scene_breaks());
    assert!(!Synopsis.shows_scene_breaks());

    // A synopsis is real prose, and gets every group a note does. Headings,
    // alignment and tables were once withheld here on the grounds that a
    // synopsis "is not chapter-structured"; an outline with levels and a
    // beat sheet in a table are both ordinary planning, and the compiler
    // pushes a synopsis through the same `push_prose` as a scene, so there
    // was nothing behind the restriction but the guess.
    assert!(Synopsis.shows_marks());
    assert!(Synopsis.shows_lists());
    assert!(Synopsis.shows_block());
    assert!(Synopsis.shows_tables());

    // Scene breaks are the whole of the difference between a synopsis and a
    // scene. If a second predicate ever narrows again, it needs a reason as
    // concrete as this one's.
    for surface in [Scene, Note, Synopsis] {
        assert!(surface.shows_block());
        assert!(surface.shows_tables());
    }

    // History and marks are the high-frequency groups: they never vanish
    // while there is anywhere to type, so moving between a scene and its
    // synopsis does not make the dock flicker.
    for surface in [Scene, Note, Synopsis] {
        assert!(surface.shows_history());
        assert!(surface.shows_marks());
        assert!(surface.shows_lists());
        assert!(!surface.is_empty());
    }

    // And nothing focused shows nothing at all.
    assert!(None.is_empty());
    assert!(!None.shows_history());
    assert!(!None.shows_marks());
    assert!(!None.shows_lists());
}

#[test]
fn the_surface_signal_only_fires_on_a_real_change() {
    let vm = vm_detached();
    assert_eq!(vm.surface_signal().get(), FormatSurface::None);
    vm.set_surface(FormatSurface::Scene);
    assert_eq!(vm.surface_signal().get(), FormatSurface::Scene);
    vm.set_surface(FormatSurface::Scene);
    assert_eq!(vm.surface_signal().get(), FormatSurface::Scene);
}

#[test]
fn losing_focus_clears_the_mirrors() {
    // A stale "bold" lingering over the empty state would be a lie about a
    // document the user is no longer in.
    let flip: Rc<Cell<bool>> = Rc::new(Cell::new(true));
    let doc = TextDocument::new();
    doc.set_markdown("Hello")
        .expect("parse")
        .wait()
        .expect("import");
    let editor = RichTextEditor::editor(doc);
    editor.select_all();
    editor.handle().set_bold(true);

    let handle = editor.handle();
    let gate = flip.clone();
    let vm = FormatViewModel::new(Rc::new(move || {
        if gate.get() {
            (Some(handle.clone()), FormatSurface::Scene)
        } else {
            (None, FormatSurface::None)
        }
    }));

    vm.refresh();
    assert!(vm.bold().get());

    flip.set(false);
    vm.refresh();
    assert!(!vm.bold().get(), "mirrors clear when the editor goes away");
}

#[test]
fn a_fresh_paragraph_reads_as_automatic_direction() {
    let (vm, _editor) = vm_over("Hello");
    vm.refresh();
    assert_eq!(vm.direction().get(), DIR_AUTO);
    assert!(!vm.dir_rtl().get());
}

#[test]
fn pinning_a_direction_is_reported_back() {
    let (vm, _editor) = vm_over("Hello");
    vm.refresh();

    vm.set_direction(DIR_RTL);
    vm.refresh();
    assert_eq!(vm.direction().get(), DIR_RTL);
    assert!(vm.dir_rtl().get(), "the dock toggle should light up");

    vm.set_direction(DIR_LTR);
    vm.refresh();
    assert_eq!(vm.direction().get(), DIR_LTR);
    assert!(
        !vm.dir_rtl().get(),
        "an explicit left-to-right is not right-to-left"
    );
}

#[test]
fn automatic_is_a_state_the_writer_can_get_back_to() {
    // The reason `clear_direction` had to exist: pinning
    // left-to-right is *not* the same as never having chosen, and
    // only an unset direction lets Arabic auto-detect as RTL.
    let (vm, _editor) = vm_over("Hello");
    vm.refresh();

    vm.set_direction(DIR_RTL);
    vm.refresh();
    assert_eq!(vm.direction().get(), DIR_RTL);

    vm.set_direction(DIR_AUTO);
    vm.refresh();
    assert_eq!(
        vm.direction().get(),
        DIR_AUTO,
        "choosing Automatic must unset the direction, not pin LTR"
    );
    assert!(!vm.dir_rtl().get());
}

#[test]
fn the_dock_toggle_flips_between_rtl_and_automatic() {
    let (vm, _editor) = vm_over("Hello");
    vm.refresh();

    vm.toggle_direction();
    vm.refresh();
    assert_eq!(vm.direction().get(), DIR_RTL);

    // Off returns to automatic rather than pinning left-to-right —
    // the writer is undoing a choice, not making the opposite one.
    vm.toggle_direction();
    vm.refresh();
    assert_eq!(vm.direction().get(), DIR_AUTO);
}

#[test]
fn clearing_formatting_also_unsets_the_direction() {
    let (vm, _editor) = vm_over("Hello");
    vm.refresh();
    vm.set_direction(DIR_RTL);
    vm.refresh();

    vm.clear_formatting();
    assert_eq!(
        vm.direction().get(),
        DIR_AUTO,
        "clear formatting must unset the direction, not leave the \
             paragraph pinned right-to-left"
    );
}

/// A footnote binding over a `Content` row with a known id.
fn binding_over(content_id: u64) -> crate::footnotes::FootnoteBinding {
    let ctx = Rc::new(frontend::AppContext::new());
    let docs = crate::models::OpenDocsStore::new(ctx.clone());
    let model = crate::models::FootnotesListModel::new(
        ctx.clone(),
        crate::app_ids::AppIds::new(),
        docs.clone(),
    );
    let vm = crate::footnotes::FootnotesViewModel::new(model, docs, Signal::new(None));
    vm.binding(crate::singles::SingleContent::from_id(ctx, content_id))
}

/// A footnote goes into the editor the **caret** is in, and annotates *that*
/// editor's `Content` row.
///
/// The regression this pins is the one that shipped: the command resolved its
/// handle from the focused *tab* instead, which in a stream is the container's
/// prose rather than the row being typed in — and which reports a caret of 0
/// for an editor nobody is in. Every marker landed at the top of the document,
/// wherever the writer had actually put the caret.
#[test]
fn a_footnote_targets_the_focused_editors_own_content_row() {
    let vm = vm_detached();
    let (synopsis_id, prose_id) = two_ids();
    let synopsis = RichTextEditor::editor(TextDocument::new());
    let prose = RichTextEditor::editor(TextDocument::new());
    let (sh, ph) = (synopsis.handle(), prose.handle());
    vm.register(synopsis_id, sh.clone(), EditorKind::Synopsis);
    vm.set_registered_footnotes(synopsis_id, binding_over(11));
    vm.register(prose_id, ph.clone(), EditorKind::Prose);
    vm.set_registered_footnotes(prose_id, binding_over(22));

    assert!(
        vm.footnote_target().is_none(),
        "nothing focused and nothing latched: refusing is the honest answer, \
             and inserting into an arbitrary editor at its stale caret is not"
    );

    // The prose editor wins, and hands back *its own* door — the identity
    // that matters is which registration answered, since the row behind it
    // may not even exist yet (see `FootnoteBinding::ensure_content_id`).
    ph.focused_signal().set(true);
    let (handle, _) = vm.footnote_target().expect("the focused prose editor");
    assert!(
        handle.focused_signal().get(),
        "resolved an editor that is not the one holding the caret"
    );

    // Reaching the menu takes focus away; the latch carries the command through.
    ph.focused_signal().set(false);
    assert!(
        vm.footnote_target().is_some(),
        "the latch must survive the menu overlay taking focus"
    );

    // A synopsis is planning text: a note attached there prints into a
    // synopsis export and nowhere in the book.
    sh.focused_signal().set(true);
    assert!(
        vm.footnote_target().is_none(),
        "a synopsis is not where a footnote goes"
    );
}

/// An editor with no footnote door at all is not a target: the surfaces that
/// have one are the prose surfaces, and guessing a row for the others is how
/// a note would end up on a different scene than its marker.
#[test]
fn an_editor_with_no_footnote_binding_is_not_a_target() {
    let vm = vm_detached();
    let (id, _) = two_ids();
    let editor = RichTextEditor::editor(TextDocument::new());
    let handle = editor.handle();
    vm.register(id, handle.clone(), EditorKind::Prose);
    handle.focused_signal().set(true);
    assert!(vm.footnote_target().is_none());
}

// ── Links ───────────────────────────────────────────────────────
//
// The Link command is the only formatting command that carries data, so it is
// the only one with real branching: link the selection, link at a bare caret,
// edit an existing link's destination, rename its text, remove it. Each branch
// is asserted through the document's own Djot, which is what a save writes.

/// A view-model over one editor with the caret parked at `position` and
/// nothing selected — the shape "the caret is inside a link" needs.
fn vm_at(text: &str, position: usize) -> (FormatViewModel, RichTextEditor, EditorHandle) {
    let doc = TextDocument::new();
    doc.set_djot(text).expect("parse").wait().expect("import");
    let editor = RichTextEditor::editor(doc);
    let handle = editor.handle();
    handle.select_range(position, position);
    let h = handle.clone();
    let vm = FormatViewModel::new(Rc::new(move || (Some(h.clone()), FormatSurface::Scene)));
    (vm, editor, handle)
}

#[test]
fn linking_a_selection_wraps_exactly_it() {
    let (vm, _editor, handle) = vm_at("Read the manual today", 0);
    handle.select_range(9, 15);

    vm.apply_link("manual", "https://example.com");

    assert_eq!(
        handle.to_djot().trim(),
        "Read the [manual](https://example.com) today"
    );
}

#[test]
fn linking_keeps_formatting_already_on_the_words() {
    // The reason the link is applied as a format rather than by reinserting
    // the text: retyping the words would drop the italic they carry.
    let (vm, _editor, handle) = vm_at("Read the _manual_ today", 0);
    handle.select_range(9, 15);

    vm.apply_link("manual", "https://example.com");

    assert_eq!(
        handle.to_djot().trim(),
        "Read the _[manual](https://example.com)_ today"
    );
}

#[test]
fn linking_at_a_bare_caret_inserts_the_name() {
    let (vm, _editor, handle) = vm_at("Read  today", 5);

    vm.apply_link("the manual", "https://example.com");

    assert_eq!(
        handle.to_djot().trim(),
        "Read [the manual](https://example.com) today"
    );
}

#[test]
fn editing_a_links_destination_leaves_its_words_alone() {
    let (vm, _editor, handle) = vm_at("Read [the manual](https://old.example) today", 10);

    // The caret is inside the link and the name is unchanged, so this is a
    // pure format merge — no text is rewritten.
    vm.apply_link("the manual", "https://new.example");

    assert_eq!(
        handle.to_djot().trim(),
        "Read [the manual](https://new.example) today"
    );
}

#[test]
fn renaming_a_link_replaces_only_its_own_words() {
    let (vm, _editor, handle) = vm_at("Read [the manual](https://example.com) today", 10);

    vm.apply_link("the handbook", "https://example.com");

    assert_eq!(
        handle.to_djot().trim(),
        "Read [the handbook](https://example.com) today"
    );
}

#[test]
fn editing_a_link_split_by_a_mark_rewrites_the_whole_link() {
    // The extent has to coalesce across the runs the italic split the link
    // into, or a rename would rewrite one fragment and leave the rest linked.
    let (vm, _editor, handle) = vm_at("Read [the _long_ manual](https://example.com) now", 12);

    vm.apply_link("the handbook", "https://example.com");

    assert_eq!(
        handle.to_djot().trim(),
        "Read [the handbook](https://example.com) now"
    );
}

#[test]
fn removing_a_link_keeps_its_words() {
    let (vm, _editor, handle) = vm_at("Read [the manual](https://example.com) today", 10);

    vm.remove_link();

    assert_eq!(handle.to_djot().trim(), "Read the manual today");
}

#[test]
fn the_link_mirror_reports_whether_the_caret_is_on_one() {
    let (vm, _editor, handle) = vm_at("Read [the manual](https://example.com) today", 10);
    vm.refresh();
    assert!(vm.link().get(), "the caret is inside the link");

    handle.select_range(1, 1);
    vm.refresh();
    assert!(!vm.link().get(), "the caret is out in plain prose");
}

#[test]
fn the_link_request_pre_fills_from_the_link_under_the_caret() {
    let (vm, _editor, _handle) = vm_at("Read [the manual](https://example.com) today", 10);

    let req = vm.link_request().expect("an editor is focused");
    assert_eq!(req.name, "the manual");
    assert_eq!(req.href, "https://example.com");
    assert!(
        req.editing,
        "an existing link is an edit, and offers removal"
    );
}

#[test]
fn the_link_request_pre_fills_from_the_selection_when_there_is_no_link() {
    let (vm, _editor, handle) = vm_at("Read the manual today", 0);
    handle.select_range(9, 15);

    let req = vm.link_request().expect("an editor is focused");
    assert_eq!(req.name, "manual", "the selected words become the name");
    assert!(req.href.is_empty());
    assert!(!req.editing, "a fresh link offers no removal");
}

#[test]
fn clearing_formatting_takes_the_link_off_too() {
    // "Plain prose" means plain: a link left behind by Clear formatting would
    // be a surprise, and the writer has no other way to reach it from there.
    let (vm, _editor, handle) = vm_at("Read [the manual](https://example.com) today", 10);

    vm.clear_formatting();

    assert_eq!(handle.to_djot().trim(), "Read the manual today");
}

#[test]
fn the_link_mirror_is_the_signal_the_dock_and_menu_bind() {
    // The Link command travels by intent, so it cannot use `toggle_button`'s
    // constructor — which is how its button first shipped with no state at all,
    // silently inert while every other mark lit up. This pins the property
    // those two surfaces depend on: the signal a *caller* holds is the one that
    // moves, so a button or a menu row bound to it reflects the document.
    let (vm, _editor, handle) = vm_at("Read [the manual](https://example.com) today", 1);

    // Held the way `intent_toggle_button(.., vm.link(), ..)` and the menu's
    // `.checked(f.link())` hold it: cloned once, up front, never re-read.
    let bound = vm.link();
    vm.refresh();
    assert!(!bound.get(), "the caret starts out in plain prose");

    handle.select_range(10, 10);
    vm.refresh();
    assert!(
        bound.get(),
        "moving the caret into a link must reach a signal cloned before the move"
    );

    handle.select_range(1, 1);
    vm.refresh();
    assert!(!bound.get(), "and leaving it must clear the same signal");
}
