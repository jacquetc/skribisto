# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

## The story-bible entry modal: the configuration step the ＋ Create vocabulary's
## "Story bible entry…" opens on a row it has just made. See
## `teksilo_ui::story_bible::modal`.
##
## "Add as note" no longer reaches it: that door files the note straight away, under
## the one tag the writer picks. Its own strings are under "the capture flow" below.

story-bible-modal-configure-title = Indstil det nye opslag
story-bible-modal-name-label = Navn
story-bible-modal-name-placeholder = En person, et sted, alt, der er værd at registrere…
story-bible-modal-no-binders = Projektet har endnu intet ringbind at placere det i
story-bible-modal-template-label = Start fra en skabelon (valgfrit)
story-bible-modal-template-placeholder = Ingen skabelon
story-bible-modal-body-label = Brødtekst
story-bible-modal-cancel = Annullér
story-bible-modal-create = Opret
story-bible-modal-create-and-open = Opret og åbn
story-bible-modal-failed = Opslaget kunne ikke oprettes

## "Add as note": the editor context-menu row. A submenu of this project's tags, in
## three tiers, with Untagged always last and always present. Picking one files the
## selection as a note immediately, with no dialog: the tag says where it goes and what
## it starts from.
ctx-add-as-note = Tilføj som &note…
ctx-add-as-note-all-tags = Alle mærker…
ctx-add-as-note-untagged = Uden mærke

## The Story bible place (C2): a card grid on every notes folder, grouped by
## discoverable tag. See `teksilo_ui::tabs::story_bible_place`.
story-bible-grid-label = Historiebibel
story-bible-grid-untagged = Endnu uden mærke
story-bible-grid-empty-title = Intet registreret her endnu
story-bible-grid-empty-hint = Giv en note et mærke til søgning i teksten, eller tilføj en fra en markering, så vises den her.
story-bible-grid-alias-count = { $count ->
    [0] Ingen alternative navne endnu
    [one] { $count } alternativt navn
   *[other] { $count } alternative navne
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
    [0] Ingen forekomster endnu
    [one] Optræder i { $count } scene i hele projektet
   *[other] Optræder i { $count } scener i hele projektet
}
story-bible-books-filter-all = Alle bøger
# An entry with no discoverable tag was never in the scan's alias table, so nothing
# was ever looked for. Reporting "No appearances yet" for it asserts an absence that
# was never measured: a character written into every chapter would read as appearing
# in none. See `GroupedCard::discoverable`.
story-bible-grid-not-searched = Ikke søgt endnu: kræver et historiebibel-mærke

## The "In prose" segment on an `Item/Note` tab (C3): a writable stream of the
## manuscript prose this note has been declared present in, one Book at a time.
## See `teksilo_ui::tabs::note_in_prose`.
note-in-prose-pov = Synsvinkel
note-in-prose-cast = Persongalleri
note-in-prose-pov-and-cast = Synsvinkel · Persongalleri
note-in-prose-no-books = Projektet har endnu ingen bog
note-in-prose-empty-book = Endnu ikke angivet i denne bog

# The capture flow: one click from a selection to a filed note.
story-bible-capture-toast = »{ $name }« er føjet til historiebiblen
story-bible-capture-open = Åbn
story-bible-capture-undo = Fortryd
story-bible-capture-where-title = Hvor skal disse noter placeres?
story-bible-capture-where-prompt = Vælg den mappe, nye noter skal placeres i.
story-bible-capture-where-confirm = Placér her
story-bible-capture-where-needs-folder = Vælg en mappe, noterne skal placeres i.
story-bible-capture-where-tag = Du bliver kun spurgt én gang. Noter med mærket »{ $tag }« placeres her fra nu af, og du kan ændre det senere under Indstillinger ▸ Projekt ▸ Mærker.
story-bible-capture-where-untagged = Du bliver kun spurgt én gang. Noter uden mærke placeres her fra nu af, og du kan ændre det senere under Indstillinger ▸ Projekt ▸ Mærker.
