# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

## The story-bible entry creation modal (C1), reached from "Add as note" on a
## text selection, and as the configuration step the ＋ Create vocabulary's
## "Story bible entry…" opens on a row it just made. See
## `teksilo_ui::story_bible::modal`.

story-bible-modal-create-title = New story bible entry
story-bible-modal-configure-title = Configure the new entry
story-bible-modal-name-label = Name
story-bible-modal-name-placeholder = A character, a place, anything worth filing…
story-bible-modal-location-label = Where
story-bible-modal-no-binders = This project has no binders to file it into yet
story-bible-modal-template-label = Start from a template (optional)
story-bible-modal-template-placeholder = No template
story-bible-modal-body-label = Body
story-bible-modal-cancel = Cancel
story-bible-modal-create = Create
story-bible-modal-create-and-open = Create and open
story-bible-modal-choose-location = Choose where to file this entry first
story-bible-modal-failed = Could not create the entry

## "Add as note": the editor context-menu row that opens the modal above,
## pre-filled from the current selection.
ctx-add-as-note = Add as &note…

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

## The "In prose" segment on an `Item/Note` tab (C3): a writable stream of the
## manuscript prose this note has been declared present in, one Book at a time.
## See `teksilo_ui::tabs::note_in_prose`.
note-in-prose-pov = Point of view
note-in-prose-cast = Cast
note-in-prose-pov-and-cast = Point of view · Cast
note-in-prose-no-books = This project has no Book yet
note-in-prose-empty-book = Not declared in this Book yet
