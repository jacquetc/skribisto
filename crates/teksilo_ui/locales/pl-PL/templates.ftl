# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

settings-page-templates = Szablony
settings-templates-description = Fragmenty tekstu wielokrotnego użytku, które możesz wstawić do wszystkiego, co piszesz: pusta karta postaci, profil miejsca, rozpiska scen. Są przechowywane w tym projekcie, więc każdy, kto go otworzy, ma ten sam zestaw.
settings-templates-filter = Filtruj szablony
settings-templates-count = { $n ->
        [0] Brak szablonów
        [one] { $n } szablon
        [few] { $n } szablony
        [many] { $n } szablonów
       *[other] { $n } szablonu
    }
settings-templates-empty = Ten projekt nie ma jeszcze szablonów.
settings-templates-empty-hint = Dodaj szablon z gotowych wzorów, zaimportuj plik .md lub .djot albo napisz coś i wybierz Dokument ▸ Zapisz jako szablon.
settings-templates-presets = Dodaj gotowy wzór
settings-templates-import = Importuj…
settings-templates-export = Eksportuj…
settings-templates-star = Pokazuj jako pierwszy w menu wstawiania
settings-templates-unstar = Nie pokazuj jako pierwszego
settings-templates-move-up = Przesuń w górę
settings-templates-move-down = Przesuń w dół
settings-templates-delete = Usuń szablon
settings-templates-name = Nazwa
settings-templates-name-placeholder = Nazwa szablonu
settings-templates-body-placeholder = Tekst wstawiany przez ten szablon
settings-templates-duplicate-name = Inny szablon ma już nazwę „{ $name }”
settings-templates-words = { $n ->
        [one] { $n } słowo
        [few] { $n } słowa
        [many] { $n } słów
       *[other] { $n } słowa
    }

templates-imported = { $added ->
        [one] Zaimportowano { $added } szablon
        [few] Zaimportowano { $added } szablony
        [many] Zaimportowano { $added } szablonów
       *[other] Zaimportowano { $added } szablonu
    }
templates-imported-renamed = Zmieniono nazwy, aby uniknąć konfliktu: { $names }
templates-imported-skipped = Nie można odczytać: { $names }
templates-import-filter = Szablony (.md, .djot)
templates-exported = { $n ->
        [one] Wyeksportowano { $n } szablon
        [few] Wyeksportowano { $n } szablony
        [many] Wyeksportowano { $n } szablonów
       *[other] Wyeksportowano { $n } szablonu
    }
templates-preset-applied = Dodano „{ $name }”

templates-delete-title = Usunąć ten szablon?
templates-delete-body = „{ $name }” zostanie usunięty z tego projektu. Nie dotyczy to notatek już z niego utworzonych.
templates-delete-confirm = Usuń

menu-document = &Dokument
menu-insert-template = Wstaw &szablon
menu-insert-template-none = Brak szablonów w tym projekcie
menu-save-as-template = Zapisz jako sza&blon…

save-as-template-title = Zapisz jako szablon
save-as-template-explain = Edytowany tekst stanie się szablonem, który możesz wstawić w dowolnym innym miejscu.
save-as-template-name = Nazwa
save-as-template-placeholder = Karta postaci
save-as-template-duplicate = Szablon o nazwie „{ $name }” już istnieje
save-as-template-empty-editor = Ten edytor jest pusty. Nie ma czego zapisać.
save-as-template-confirm = Zapisz szablon
save-as-template-saved = Zapisano „{ $name }” jako szablon
template-inserted = Wstawiono „{ $name }”

note-template-preset-character-sheet = Karta postaci
note-template-preset-location = Miejsce
note-template-preset-artifact = Przedmiot
note-template-preset-beat-sheet = Rozpiska scen
note-template-preset-faction = Frakcja
note-template-preset-research-note = Notatka z materiałami

note-template-set-essentials = Podstawowe
note-template-set-everything = Wszystkie szablony

note-template-section-identity = Tożsamość
note-template-section-appearance = Wygląd
note-template-section-voice = Głos
note-template-section-psychology = Życie wewnętrzne
note-template-section-history = Historia
note-template-section-arc = Łuk fabularny
note-template-section-first-impression = Pierwsze wrażenie
note-template-section-in-the-story = W opowieści
note-template-section-the-thing-itself = Sam przedmiot
note-template-section-the-scene = Scena
note-template-section-the-shape = Kształt
note-template-section-what-it-is = Czym jest
note-template-section-source = Źródło
note-template-section-what-it-says = Co mówi

note-template-field-full-name = Imię i nazwisko
note-template-field-known-as = Inne nazwy, zapisywane w polu „Inne nazwy” inspektora
note-template-field-age = Wiek
note-template-field-role-in-story = Rola w opowieści
note-template-field-build-and-features = Budowa ciała i cechy charakterystyczne
note-template-field-habitual-bearing = Zwykła postawa
note-template-field-speech-patterns = Sposób mówienia i słowne nawyki
note-template-field-what-they-never-say = Czego nigdy nie mówi
note-template-field-want = Czego chce
note-template-field-need = Czego potrzebuje, nie wiedząc o tym
note-template-field-fear = Czego się boi
note-template-field-flaw = Wada, która go drogo kosztuje
note-template-field-formative-event = Wydarzenie formujące
note-template-field-relationships = Relacje
note-template-field-starts-as = Zaczyna się jako
note-template-field-ends-as = Kończy jako
note-template-field-what-you-notice-first = Co zauważa się najpierw
note-template-field-sound-and-smell = Dźwięk i zapach
note-template-field-light-and-weather = Światło i pogoda
note-template-field-what-happened-here = Co się tu wydarzyło
note-template-field-who-lives-or-works-here = Kto tu mieszka lub pracuje
note-template-field-scenes-set-here = Sceny rozgrywające się tutaj
note-template-field-why-it-matters = Dlaczego to ważne
note-template-field-appearance = Wygląd
note-template-field-age-and-origin = Wiek i pochodzenie
note-template-field-what-it-does = Do czego służy
note-template-field-who-holds-it = Kto go posiada
note-template-field-who-wants-it = Kto go pragnie
note-template-field-pov = Punkt widzenia
note-template-field-time-and-place = Czas i miejsce
note-template-field-goal = Cel
note-template-field-conflict = Konflikt
note-template-field-turn = Punkt zwrotny
note-template-field-exit-emotion = Emocja na wyjściu
note-template-field-purpose = Przeznaczenie
note-template-field-who-leads-it = Kto nią kieruje
note-template-field-resources = Zasoby
note-template-field-allies-and-enemies = Sojusznicy i wrogowie
note-template-field-what-it-wants-now = Czego teraz chce
note-template-field-where-from = Skąd pochodzi
note-template-field-page-or-link = Strona lub odnośnik
note-template-field-key-facts = Kluczowe fakty
note-template-field-how-it-is-used = Jak jest wykorzystany w opowieści
