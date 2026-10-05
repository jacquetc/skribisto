# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

## The story-bible entry modal: the configuration step the ＋ Create vocabulary's
## "Story bible entry…" opens on a row it has just made. See
## `teksilo_ui::story_bible::modal`.
##
## "Add as note" no longer reaches it: that door files the note straight away, under
## the one tag the writer picks. Its own strings are under "the capture flow" below.

story-bible-modal-configure-title = Configurați noua intrare
story-bible-modal-name-label = Nume
story-bible-modal-name-placeholder = Un personaj, un loc, orice merită consemnat…
story-bible-modal-no-binders = Acest proiect nu are încă niciun clasor în care să fie pusă
story-bible-modal-template-label = Pornește de la un șablon (opțional)
story-bible-modal-template-placeholder = Fără șablon
story-bible-modal-body-label = Conținut
story-bible-modal-cancel = Renunță
story-bible-modal-create = Creează
story-bible-modal-create-and-open = Creează și deschide
story-bible-modal-failed = Intrarea nu a putut fi creată

## "Add as note": the editor context-menu row. A submenu of this project's tags, in
## three tiers, with Untagged always last and always present. Picking one files the
## selection as a note immediately, with no dialog: the tag says where it goes and what
## it starts from.
ctx-add-as-note = Adaugă ca &notă…
ctx-add-as-note-all-tags = Toate etichetele…
ctx-add-as-note-untagged = Fără etichetă

## The Story bible place (C2): a card grid on every notes folder, grouped by
## discoverable tag. See `teksilo_ui::tabs::story_bible_place`.
story-bible-grid-label = Biblia poveștii
story-bible-grid-untagged = Încă fără etichetă
story-bible-grid-empty-title = Încă nu este nimic consemnat aici
story-bible-grid-empty-hint = Etichetați o notă pentru a o căuta în proză sau adăugați una dintr-o selecție, iar ea va apărea aici.
story-bible-grid-alias-count = { $count ->
       [0] Încă niciun nume alternativ
       [one] { $count } nume alternativ
       [few] { $count } nume alternative
      *[other] { $count } de nume alternative
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
       [0] Încă nu apare nicăieri
       [one] Apare în { $count } scenă din tot proiectul
       [few] Apare în { $count } scene din tot proiectul
      *[other] Apare în { $count } de scene din tot proiectul
   }
story-bible-books-filter-all = Toate cărțile
# An entry with no discoverable tag was never in the scan's alias table, so nothing
# was ever looked for. Reporting "No appearances yet" for it asserts an absence that
# was never measured: a character written into every chapter would read as appearing
# in none. See `GroupedCard::discoverable`.
story-bible-grid-not-searched = Încă necăutată: îi trebuie o etichetă de biblie a poveștii

## The "In prose" segment on an `Item/Note` tab (C3): a writable stream of the
## manuscript prose this note has been declared present in, one Book at a time.
## See `teksilo_ui::tabs::note_in_prose`.
note-in-prose-pov = Punct de vedere
note-in-prose-cast = Distribuție
note-in-prose-pov-and-cast = Punct de vedere · Distribuție
note-in-prose-no-books = Acest proiect nu are încă nicio carte
note-in-prose-empty-book = Încă nu este declarată în această carte

# The capture flow: one click from a selection to a filed note.
story-bible-capture-toast = „{ $name }” a fost adăugat în biblia poveștii
story-bible-capture-open = Deschide
story-bible-capture-undo = Anulează
story-bible-capture-where-title = Unde ajung aceste note?
story-bible-capture-where-prompt = Alegeți dosarul în care să fie puse notele noi.
story-bible-capture-where-confirm = Pune aici
story-bible-capture-where-needs-folder = Alegeți un dosar în care să intre aceste note.
story-bible-capture-where-tag = Întrebare unică. De acum înainte, notele etichetate „{ $tag }” vor ajunge aici; puteți schimba asta oricând din Setări, Proiect, Etichete.
story-bible-capture-where-untagged = Întrebare unică. De acum înainte, notele fără etichetă vor ajunge aici; puteți schimba asta oricând din Setări, Proiect, Etichete.
