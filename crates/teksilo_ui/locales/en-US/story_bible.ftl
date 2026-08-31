# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

## The story-bible entry modal: the configuration step the ＋ Create vocabulary's
## "Story bible entry…" opens on a row it has just made. See
## `teksilo_ui::story_bible::modal`.
##
## "Add as note" no longer reaches it: that door files the note straight away, under
## the one tag the writer picks. Its own strings are under "the capture flow" below.

story-bible-modal-configure-title = Configure the new entry
story-bible-modal-name-label = Name
story-bible-modal-name-placeholder = A character, a place, anything worth filing…
story-bible-modal-no-binders = This project has no binders to file it into yet
story-bible-modal-template-label = Start from a template (optional)
story-bible-modal-template-placeholder = No template
story-bible-modal-body-label = Body
story-bible-modal-cancel = Cancel
story-bible-modal-create = Create
story-bible-modal-create-and-open = Create and open
story-bible-modal-failed = Could not create the entry

## "Add as note": the editor context-menu row. A submenu of this project's tags, in
## three tiers, with Untagged always last and always present. Picking one files the
## selection as a note immediately, with no dialog: the tag says where it goes and what
## it starts from.
ctx-add-as-note = Add as &note…
ctx-add-as-note-all-tags = All tags…
ctx-add-as-note-untagged = Untagged

## The Story bible place (C2): a card grid on every notes folder, grouped by
## discoverable tag. See `teksilo_ui::tabs::story_bible_place`.
story-bible-grid-label = Story bible
story-bible-grid-untagged = Not yet tagged
story-bible-grid-empty-title = Nothing filed here yet
story-bible-grid-empty-hint = Tag a note to find it in prose, or add one from a selection, and it appears here.
story-bible-grid-alias-count = { $count ->
    [0] No aliases yet
    [one] { $count } alias
   *[other] { $count } aliases
}
# Work-wide, never a per-book figure: a scan hit in any Book of this project
# counts, regardless of which Book the writer currently has open. Scene-owned
# only: a worldbuilding note naming this entry does not count as "a scene".
# Counts a declared presence, not only a text hit: a scene the writer pinned
# by hand, or declared as its point of view, counts even when the entry's
# name is never actually written there. Worded "appears", not "mentioned":
# the wording must not claim the name is in the text when only a
# relationship was declared.
story-bible-grid-mention-count = { $count ->
    [0] No appearances yet
    [one] Appears in { $count } scene across the whole project
   *[other] Appears in { $count } scenes across the whole project
}
story-bible-books-filter-all = All books
# An entry with no discoverable tag was never in the scan's alias table, so nothing
# was ever looked for. Reporting "No appearances yet" for it asserts an absence that
# was never measured: a character written into every chapter would read as appearing
# in none. See `GroupedCard::discoverable`.
story-bible-grid-not-searched = Not searched yet: needs a story-bible tag

## The "In prose" segment on an `Item/Note` tab (C3): a writable stream of the
## manuscript prose this note has been declared present in, one Book at a time.
## See `teksilo_ui::tabs::note_in_prose`.
note-in-prose-pov = Point of view
note-in-prose-cast = Cast
note-in-prose-pov-and-cast = Point of view · Cast
note-in-prose-no-books = This project has no Book yet
note-in-prose-empty-book = Not declared in this Book yet

# The capture flow: one click from a selection to a filed note.
story-bible-capture-toast = "{ $name }" added to the story bible
story-bible-capture-open = Open
story-bible-capture-undo = Undo
story-bible-capture-where-title = Where do these notes go?
story-bible-capture-where-prompt = Choose the folder new notes should be filed in.
story-bible-capture-where-confirm = File here
story-bible-capture-where-needs-folder = Choose a folder for these notes to go inside.
story-bible-capture-where-tag = Asked once. Notes tagged "{ $tag }" will go here from now on, and you can change it later in Settings, Work, Tags.
story-bible-capture-where-untagged = Asked once. Notes with no tag will go here from now on, and you can change it later in Settings, Work, Tags.
