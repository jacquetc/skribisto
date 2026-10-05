# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

tags-preset-basic = Podstawowy
tags-preset-scifi = Science fiction
tags-preset-fantasy = Fantasy
tags-preset-mystery = Kryminał
tags-preset-historical = Historyczny


tags-preset-needs-research = wymaga dokumentacji
tags-preset-continuity-check = sprawdzić ciągłość
tags-preset-plot-point = punkt zwrotny

tags-preset-character = postać
tags-preset-place = miejsce
tags-preset-item = przedmiot

tags-preset-vessel = statek
tags-preset-planet = planeta
tags-preset-organization = organizacja
tags-preset-creature = stworzenie
tags-preset-faction = frakcja
tags-preset-artifact = artefakt
tags-preset-realm = kraina
tags-preset-magic-system = system magii
tags-preset-suspect = podejrzany
tags-preset-victim = ofiara
tags-preset-clue = wskazówka
tags-preset-red-herring = fałszywy trop
tags-preset-historical-figure = postać historyczna
tags-preset-source = źródło
tags-preset-period-detail = realia epoki

tags-pill-list = Znaczniki
tags-pill-add = Dodaj znacznik
tags-none-yet = Brak znaczników
tags-pick-list = Znaczniki do wyboru
tags-pill-remove = Usuń { $name }
tags-pill-filter-placeholder = Filtruj lub nazwij nowy znacznik
tags-pill-no-match = Żaden znacznik nie pasuje
tags-pill-create = Utwórz „{ $name }”
tags-pill-create-failed = Nie można utworzyć „{ $name }”
tags-pill-new-discoverable = Znacznik wyszukiwany w prozie
tags-pill-new-discoverable-hint = Elementy z tym znacznikiem są wyszukiwane w Twojej prozie, aby uzupełnić obsadę.

tags-alias-list = Inne nazwy
tags-alias-add = Dodaj inną nazwę
tags-alias-remove = Usuń { $name }
tags-alias-placeholder = Inna nazwa, potem Enter
tags-alias-hint = Nazwy, pod którymi ten element pojawia się w Twojej prozie, oprócz tytułu.
tags-alias-collision = { $name } już odpowiada na tę nazwę.

settings-page-tags = Znaczniki
settings-tags-desc = Znaczniki oznaczają elementy w segregatorze. Znacznik wyszukiwany w prozie dodatkowo każe Skribisto szukać nazw tego elementu w Twojej prozie.
settings-tags-add = Dodaj znacznik
settings-tags-add-placeholder = Nazwij nowy znacznik
settings-tags-added = Dodano „{ $name }”
settings-tags-duplicate = „{ $name }” już istnieje
settings-tags-filter = Filtruj znaczniki
settings-tags-count = { $n ->
        [one] { $n } znacznik
        [few] { $n } znaczniki
        [many] { $n } znaczników
       *[other] { $n } znacznika
    }
settings-tags-details-placeholder = Co oznacza ten znacznik
settings-tags-discoverable = Wyszukuj w prozie
settings-tags-creates-in = Nowe notatki trafiają do
settings-tags-creates-in-unset = Zapytaj mnie za pierwszym razem
settings-tags-creates-in-untitled = Folder bez nazwy
settings-tags-creates-in-trashed = { $name } (w koszu)
settings-tags-template = Szablon początkowy
settings-tags-template-unset = Pusta notatka
settings-tags-delete = Usuń { $name }
settings-tags-deleted = Usunięto „{ $name }” ze wszystkich elementów
settings-tags-empty = Brak znaczników.
settings-tags-apply-preset = Zastosuj gotowy zestaw…
settings-tags-preset-applied = Dodano: { $added }, pominięto już obecne: { $skipped }
settings-tags-csv-filter = Pliki CSV
settings-tags-import = Importuj…
settings-tags-export = Eksportuj…
settings-tags-imported = Zaimportowano: { $added }, pominięto: { $skipped }
settings-tags-exported = Wyeksportowano { $n ->
        [one] { $n } znacznik
        [few] { $n } znaczniki
        [many] { $n } znaczników
       *[other] { $n } znacznika
    }

tags-chip-more = { $n ->
        [one] jeszcze { $n } znacznik
        [few] jeszcze { $n } znaczniki
        [many] jeszcze { $n } znaczników
       *[other] jeszcze { $n } znacznika
    }

cast-section = Obsada
cast-add = Dodaj do obsady…
cast-add-filter-placeholder = Filtruj biblię opowieści…
cast-add-empty = Brak wpisów biblii opowieści do dodania
cast-pin = Dodaj { $name } do obsady
cast-unpin = Usuń { $name } z obsady
cast-empty = Nikogo jeszcze nie przypięto. Dodaj kogoś lub zachowaj sugestię
cast-unresolved = Już nie ma w biblii opowieści

mentions-confirm = Dodaj do obsady { $name }
mentions-hit-count = { $n ->
    [one] raz
    [few] { $n } razy
    [many] { $n } razy
   *[other] { $n } razy
}
mentions-point-of-view-badge = Punkt widzenia
mentions-point-of-view-badge-tooltip = Ustawiony tutaj ręcznie jako punkt widzenia, a nie wykryty w prozie.

mentions-roster = Obsada
mentions-pin = Dodaj { $name } do obsady

pov-section = Punkt widzenia
pov-empty = Nie ustawiono punktu widzenia
pov-add = Ustaw punkt widzenia…
pov-multiple = Ta scena ma dwa punkty widzenia.
pov-remove = Usuń { $name } jako punkt widzenia
pov-unresolved = Przypięto tu punkt widzenia, ale ten wpis już nie istnieje lub utracił znacznik biblii opowieści.

books-section = Przypisane do
books-empty = Jeszcze nie przypisano do żadnej książki
books-add = Przypisz do książki…
books-remove = Usuń z { $name }
books-apply-to-children = Zastosuj przypisanie do elementów podrzędnych

note-details-name-placeholder = Nazwa…
note-details-tags = Znaczniki
note-details-aliases = Inne nazwy
note-details-books = Przypisane do
note-details-books-empty = Jeszcze nie przypisano do żadnej książki
note-details-links = Powiązania
note-details-links-empty = Brak powiązań. Dodaj powiązanie z innym wpisem w biblii opowieści.
note-details-links-add = Powiąż z notatką…
note-details-pov = Punkt widzenia
note-details-pov-empty = Nie ustawiono punktu widzenia
note-details-pov-unresolved = Przypięto tu punkt widzenia, ale ten wpis już nie istnieje lub utracił znacznik biblii opowieści.
note-details-pov-multiple = Ta notatka ma dwa punkty widzenia.
note-details-backlinks = Występuje w rękopisie
note-details-backlinks-empty = Na razie nic. Gdy ta nazwa pojawi się w Twojej prozie, zobaczysz to tutaj.
note-details-backlinks-outside = Poza książkami
note-details-backlinks-confirm-all = Potwierdź wszystkie wystąpienia
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
note-in-prose-mention-next = Następna wzmianka
