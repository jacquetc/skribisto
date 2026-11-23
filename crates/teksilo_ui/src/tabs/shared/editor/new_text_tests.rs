// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

use super::*;
use teksilo::widgets::rich_text::RichTextEditor;

/// An editor over `text` with the writer in it, and its justifier with the switch at
/// `on`. The editor comes back too, so it outlives the handle's state.
fn writing_in(text: &str, on: bool) -> (RichTextEditor, EditorHandle, NewTextJustifier) {
    let doc = TextDocument::new();
    if !text.is_empty() {
        doc.set_plain_text(text).expect("text");
    }
    let editor = RichTextEditor::editor(doc.clone());
    let handle = editor.handle();
    handle.focused_signal().set(true);
    let justifier = NewTextJustifier::new(doc, Signal::new(on));
    (editor, handle, justifier)
}

/// Type `text` at the caret and let a frame go by.
fn type_in(handle: &EditorHandle, justifier: &NewTextJustifier, text: &str) {
    handle.insert_text(text);
    justifier.tick(handle);
}

#[test]
fn the_first_letter_of_an_empty_text_justifies_its_paragraph() {
    let (_editor, handle, justifier) = writing_in("", true);
    type_in(&handle, &justifier, "I");
    assert_eq!(handle.get_alignment(), Alignment::Justify);
    assert_eq!(handle.to_plain_text(), "I", "the letter is kept");
}

#[test]
fn enter_carries_the_justification_on() {
    let (_editor, handle, justifier) = writing_in("", true);
    type_in(&handle, &justifier, "It began.");
    handle.insert_block();
    justifier.tick(&handle);
    type_in(&handle, &justifier, "It went on.");
    assert_eq!(handle.get_alignment(), Alignment::Justify);
    assert!(handle.to_djot().matches("alignment=justify").count() == 2);
}

#[test]
fn with_the_switch_off_a_new_text_stays_as_typed() {
    let (_editor, handle, justifier) = writing_in("", false);
    type_in(&handle, &justifier, "I");
    assert_eq!(handle.get_alignment(), Alignment::Left);
}

#[test]
fn the_switch_is_read_when_the_text_begins_not_when_the_editor_opens() {
    let doc = TextDocument::new();
    let editor = RichTextEditor::editor(doc.clone());
    let handle = editor.handle();
    handle.focused_signal().set(true);
    let wanted = Signal::new(false);
    let justifier = NewTextJustifier::new(doc, wanted.clone());
    wanted.set(true);
    type_in(&handle, &justifier, "I");
    assert_eq!(handle.get_alignment(), Alignment::Justify);
}

#[test]
fn a_text_that_already_has_words_is_left_alone() {
    let (_editor, handle, justifier) = writing_in("Already here.", true);
    type_in(&handle, &justifier, " More.");
    assert_eq!(handle.get_alignment(), Alignment::Left);
}

#[test]
fn opening_an_empty_text_writes_nothing_into_it() {
    let (_editor, handle, justifier) = writing_in("", true);
    justifier.tick(&handle);
    justifier.tick(&handle);
    assert_eq!(handle.get_alignment(), Alignment::Left);
    assert!(
        !handle.to_djot().contains("alignment"),
        "{}",
        handle.to_djot()
    );
}

#[test]
fn a_view_the_writer_is_not_in_leaves_it() {
    // The other half of a split, or a text filled under the editor: the same
    // change, but not typed there.
    let (_editor, handle, justifier) = writing_in("", true);
    handle.focused_signal().set(false);
    type_in(&handle, &justifier, "I");
    assert_eq!(handle.get_alignment(), Alignment::Left);
}

#[test]
fn undo_takes_the_justification_off_and_keeps_the_letter() {
    let (_editor, handle, justifier) = writing_in("", true);
    type_in(&handle, &justifier, "I");
    handle.undo();
    justifier.tick(&handle);
    assert_eq!(handle.get_alignment(), Alignment::Left);
    assert_eq!(handle.to_plain_text(), "I");

    // Writing on does not put it back: the text was begun already.
    type_in(&handle, &justifier, "t was.");
    assert_eq!(handle.get_alignment(), Alignment::Left);
}

#[test]
fn undo_bringing_a_deleted_text_back_does_not_justify_it() {
    let (_editor, handle, justifier) = writing_in("Kept as it was.", true);
    handle.select_all();
    handle.delete_selection();
    justifier.tick(&handle);
    assert!(handle.is_empty());

    handle.undo();
    justifier.tick(&handle);
    assert_eq!(handle.to_plain_text(), "Kept as it was.");
    assert_eq!(handle.get_alignment(), Alignment::Left);
    assert!(
        justifier.doc.can_redo(),
        "nothing was written over what is left to redo"
    );
}

#[test]
fn starting_again_in_an_emptied_text_justifies_it_again() {
    let (_editor, handle, justifier) = writing_in("A first try.", true);
    handle.select_all();
    handle.delete_selection();
    justifier.tick(&handle);
    type_in(&handle, &justifier, "A");
    assert_eq!(handle.get_alignment(), Alignment::Justify);
}

#[test]
fn a_paragraph_already_aligned_keeps_its_alignment() {
    let (_editor, handle, justifier) = writing_in("", true);
    handle.set_alignment(Alignment::Center);
    justifier.tick(&handle);
    type_in(&handle, &justifier, "Chapter One");
    assert_eq!(handle.get_alignment(), Alignment::Center);
}

#[test]
fn a_heading_is_left_alone() {
    let (_editor, handle, justifier) = writing_in("", true);
    handle.set_heading_level(2);
    justifier.tick(&handle);
    type_in(&handle, &justifier, "Part One");
    assert_eq!(handle.get_alignment(), Alignment::Left);
}

#[test]
fn a_paste_of_several_paragraphs_keeps_its_own_layout() {
    let (_editor, handle, justifier) = writing_in("", true);
    handle.insert_djot("One.\n\nTwo.");
    justifier.tick(&handle);
    assert!(
        !handle.to_djot().contains("alignment"),
        "{}",
        handle.to_djot()
    );
}
