# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

tags-preset-basic = Podstawowy
tags-preset-scifi = Science fiction
tags-preset-fantasy = Fantazja
tags-preset-mystery = Tajemnica
tags-preset-historical = Historyczne


tags-preset-needs-research = potrzebuje badań
tags-preset-continuity-check = kontrola ciągłości
tags-preset-plot-point = punkt powierzchni

tags-preset-character = znak
tags-preset-place = miejsce
tags-preset-item = pozycja

tags-preset-vessel = statek
tags-preset-planet = planeta
tags-preset-organization = organizacja
tags-preset-creature = stworzenie
tags-preset-faction = frakcja
tags-preset-artifact = artefakt
tags-preset-realm = kraina
tags-preset-magic-system = magiczny system
tags-preset-suspect = podejrzany
tags-preset-victim = ofiara
tags-preset-clue = wskazówka
tags-preset-red-herring = śledź czerwony
tags-preset-historical-figure = wartość historyczna
tags-preset-source = źródło
tags-preset-period-detail = okres szczegółowości

tags-pill-list = Etykiety
tags-pill-add = Dodaj znacznik
tags-none-yet = Brak znaczników
tags-pick-list = Dostępne etykiety
tags-pill-remove = Usuń { $name }
tags-pill-filter-placeholder = Filtruj lub podaj nowy znacznik
tags-pill-no-match = Brak dopasowania znacznika
tags-pill-create = Utwórz "{ $name }"
tags-pill-create-failed = Nie można stworzyć "{ $name }"
tags-pill-new-discoverable = Znacznik Find- in- prose
tags-pill-new-discoverable-hint = Elementy z tym znacznikiem są dopasowane do twojej prozy, aby wypełnić listę.

tags-alias-list = Inne nazwy
tags-alias-add = Dodaj inną nazwę
tags-alias-remove = Usuń { $name }
tags-alias-placeholder = Kolejna nazwa, a następnie Enter
tags-alias-hint = Nazwisko to pojawia się w twojej prozie, obok jej tytułu.
tags-alias-collision = { $name } już odpowiada na to nazwisko.

settings-page-tags = Etykiety
settings-tags-desc = Znaczniki etykietować elementy w segregatorze. Znacznik find- in- prose mówi również Skribisto, aby szukać nazwy tego elementu w prozie.
settings-tags-add = Dodaj znacznik
settings-tags-add-placeholder = Nazwa nowego znacznika
settings-tags-added = Dodano "{ $name }"
settings-tags-duplicate = "{ $name }" już istnieje
settings-tags-filter = Znaczniki filtrów
settings-tags-count = { $n ->
    [one] { $n } tag
    [few] { $n } tagi
    [many] { $n } tagów
   *[other] { $n } tagów
}
settings-tags-details-placeholder = Co oznacza ten znacznik
settings-tags-discoverable = Szukaj w tekście
settings-tags-creates-in = Nowe notatki idą do
settings-tags-creates-in-unset = Zapytaj mnie za pierwszym razem.
settings-tags-creates-in-untitled = Nienazwany folder
settings-tags-creates-in-trashed = { $name } (w śmieciach)
settings-tags-template = Szablon początkowy
settings-tags-template-unset = Puste notatki
settings-tags-delete = Skreślić { $name }
settings-tags-deleted = Usunięto "{ $name }" i usunięto go z każdej pozycji
settings-tags-empty = Nie ma jeszcze znaczników.
settings-tags-apply-preset = Zastosuj preset…
settings-tags-preset-applied = Dodano { $added }, pominięto { $skipped } już obecne
settings-tags-csv-filter = Pliki CSV
settings-tags-import = Znaczenie …
settings-tags-export = Eksport …
settings-tags-imported = Przywożone { $added }, pominięte { $skipped }
settings-tags-exported = Wywóz { $n ->
    [one] { $n } tag
    [few] { $n } tagi
    [many] { $n } tagów
   *[other] { $n } tagów
}

tags-chip-more = { $n ->
    [one] { $n } więcej znacznika
    [few] { $n } więcej tagów
    [many] { $n } więcej tagów
   *[other] { $n } więcej tagów
}

cast-section = Obecne postacie
cast-add = Dodaj do cast…
cast-add-filter-placeholder = Filtr historia bible…
cast-add-empty = Brak wpisów z biblią historyczną do dodania
cast-pin = Dodaj { $name } do odlewu
cast-unpin = Usuń { $name } z odlewu
cast-empty = Nikt jeszcze nie przygwoździł. Dodaj lub zachować sugestie
cast-unresolved = Nie ma już biblii.

mentions-confirm = Dodaj do obsady { $name }
mentions-hit-count = { $n ->
    [one] raz
    [few] { $n } razy
    [many] { $n } razy
   *[other] { $n } razy
}
mentions-point-of-view-badge = Punkt widzenia
mentions-point-of-view-badge-tooltip = Oznaczony jako punkt widzenia tutaj, ustawiony ręcznie, nie wykryty w prozie.

mentions-roster = Obecne postacie
mentions-pin = Dodaj { $name } do odlewu

pov-section = Punkt widzenia
pov-empty = Brak zbioru punktów widzenia
pov-add = Punkt docelowy view…
pov-multiple = Ta scena ma dwa punkty widzenia.
pov-remove = Usuń { $name } jako punkt widzenia
pov-unresolved = Punkt widzenia został przypięty tutaj, ale ten wpis już nie istnieje lub stracił swój znacznik biblijny.

books-section = Przypisane do
books-empty = Jeszcze nie wypełniony na podstawie książki
books-add = Plik pod book…
books-remove = Usuń z { $name }
books-apply-to-children = Złóż wniosek do dzieci

note-details-name-placeholder = Nazwa…
note-details-tags = Etykiety
note-details-aliases = Inne nazwy
note-details-books = Przypisane do
note-details-books-empty = Jeszcze nie wypełniony pod księgą
note-details-links = Linki
note-details-links-empty = Dodaj jeden do innego wpisu w Biblii historii.
note-details-links-add = Link do note…
note-details-pov = Punkt widzenia
note-details-pov-empty = Brak zbioru punktów widzenia
note-details-pov-unresolved = Punkt widzenia został przypięty tutaj, ale ten wpis już nie istnieje lub stracił swój znacznik biblijny.
note-details-pov-multiple = Ta uwaga ma dwa punkty widzenia.
note-details-backlinks = Pojawia się w manuskrypcie
note-details-backlinks-empty = Kiedy to nazwisko pojawi się w twojej prozie, pojawi się tutaj.
note-details-backlinks-outside = Poza książkami.
note-details-backlinks-confirm-all = Potwierdź każdy wygląd
note-details-backlinks-confirm-all-tooltip = Dodaj ten wpis do obsady każdego dokumentu pokazanego tutaj, który jeszcze go nie potwierdził
note-details-untitled-document = Bez tytułu

note-in-prose-mentions = { $n ->
    [one] { $n } wzmianka
    [few] { $n } wzmianki
    [many] { $n } wzmianek
   *[other] { $n } wzmianki
}
note-in-prose-mention-at = { $current } z { $total }
note-in-prose-mention-previous = Poprzednia wzmianka
note-in-prose-mention-next = Następne wspomnienie
