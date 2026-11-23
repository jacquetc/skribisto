# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

story-bible-modal-configure-title = Skonfiguruj nowy wpis
story-bible-modal-name-label = Nazwa
story-bible-modal-name-placeholder = Postać, miejsce, cokolwiek, co warto skatalogować…
story-bible-modal-no-binders = Ten projekt nie ma jeszcze segregatora, w którym można by go umieścić
story-bible-modal-template-label = Rozpocznij od szablonu (opcjonalnie)
story-bible-modal-template-placeholder = Brak szablonu
story-bible-modal-body-label = Treść
story-bible-modal-cancel = Anuluj
story-bible-modal-create = Utwórz
story-bible-modal-create-and-open = Utwórz i otwórz
story-bible-modal-failed = Nie udało się utworzyć wpisu

ctx-add-as-note = Dodaj jako &notatkę…
ctx-add-as-note-all-tags = Wszystkie znaczniki…
ctx-add-as-note-untagged = Bez znacznika

story-bible-grid-label = Biblia opowieści
story-bible-grid-untagged = Jeszcze bez znacznika
story-bible-grid-empty-title = Nic tu jeszcze nie skatalogowano
story-bible-grid-empty-hint = Nadaj notatce znacznik wyszukiwany w prozie albo dodaj ją z zaznaczenia, a pojawi się tutaj.
story-bible-grid-alias-count = { $count ->
        [0] Brak innych nazw
        [one] { $count } inna nazwa
        [few] { $count } inne nazwy
        [many] { $count } innych nazw
       *[other] { $count } innej nazwy
    }
story-bible-grid-mention-count = { $count ->
        [0] Jeszcze nigdzie nie występuje
        [one] Występuje w { $count } scenie w całym projekcie
        [few] Występuje w { $count } scenach w całym projekcie
        [many] Występuje w { $count } scenach w całym projekcie
       *[other] Występuje w { $count } sceny w całym projekcie
    }
story-bible-books-filter-all = Wszystkie książki
story-bible-grid-not-searched = Jeszcze nie wyszukiwano: potrzebny znacznik biblii opowieści

note-in-prose-pov = Punkt widzenia
note-in-prose-cast = Obsada
note-in-prose-pov-and-cast = Punkt widzenia · Obsada
note-in-prose-no-books = Ten projekt nie ma jeszcze książki
# The page lists only scenes where the writer placed the entry in the cast or the point of
# view, never the ones the scan merely found its name in, so an empty page says so: a bare
# "nothing here" read as a search that had failed.
note-in-prose-empty-book = Żadna scena w tej książce nie ma jeszcze tego wpisu w obsadzie ani w punkcie widzenia.
note-in-prose-empty-viewpoint = Żadna scena w tej książce nie jest jeszcze opowiedziana z punktu widzenia tego wpisu.
note-in-prose-scope = Widok „W prozie” pokazuje tylko sceny potwierdzone przez Ciebie. Scena, która wymienia ten wpis, pozostaje sugestią, dopóki nie dodasz wpisu do jej obsady w widoku „Szczegóły” lub w inspektorze.
# A switch beside the Book bar: keep only the scenes this entry narrates.
note-in-prose-viewpoint-only = Tylko sceny z jego punktu widzenia
note-in-prose-viewpoint-only-tooltip = Pokaż tylko sceny opowiedziane z punktu widzenia tego wpisu i ukryj te, w których tylko się pojawia

story-bible-capture-toast = Dodano „{ $name }” do biblii opowieści
story-bible-capture-open = Otwórz
story-bible-capture-undo = Cofnij
story-bible-capture-where-title = Gdzie mają trafiać te notatki?
story-bible-capture-where-prompt = Wybierz folder, w którym mają być umieszczane nowe notatki.
story-bible-capture-where-confirm = Umieść tutaj
story-bible-capture-where-needs-folder = Wybierz folder, do którego mają trafiać te notatki.
story-bible-capture-where-tag = Pytanie pada tylko raz. Notatki ze znacznikiem „{ $tag }” będą od teraz trafiać tutaj; możesz to później zmienić w Ustawienia, Projekt, Znaczniki.
story-bible-capture-where-untagged = Pytanie pada tylko raz. Notatki bez znacznika będą od teraz trafiać tutaj; możesz to później zmienić w Ustawienia, Projekt, Znaczniki.

# What a screen reader hears for a binder row that is a story bible entry: its icon
# says so on screen, and this says it aloud. The second form keeps the row's label.
binder-row-story-bible-entry = Wpis biblii opowieści
binder-row-story-bible-entry-with-label = Wpis biblii opowieści, { $label }
