# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

settings-page-templates = Szablony
settings-templates-description = Wielofunkcyjne fragmenty zapisu można wrzucić do wszystkiego, co piszesz: puste arkusze znaków, profil lokalizacji, arkusz bitów. Są one przechowywane w tym projekcie, więc każdy, kto je otwiera, dostaje ten sam zestaw.
settings-templates-filter = Szablony filtrów
settings-templates-count = { $n ->
    [0] Brak szablonów
    [one] { $n } szablon
    [few] { $n } szablony
    [many] { $n } szablonów
   *[other] Wzory formularzy { $n }
}
settings-templates-empty = Ten projekt nie ma jeszcze szablonów.
settings-templates-empty-hint = Dodaj jeden z góry, zaimportuj plik .md lub .djot lub napisz coś i wybierz dokument ▸ Zapisz jako szablon.
settings-templates-presets = Dodaj zakładkę
settings-templates-import = Znaczenie …
settings-templates-export = Eksport …
settings-templates-star = Pokaż najpierw w menu wstawiania
settings-templates-unstar = Nie pokazuj się pierwszy
settings-templates-move-up = Przesuń się.
settings-templates-move-down = Przesuń w dół
settings-templates-delete = Usuń szablon
settings-templates-name = Nazwa
settings-templates-name-placeholder = Nazwa szablonu
settings-templates-body-placeholder = Tekst ten szablon wprowadza
settings-templates-duplicate-name = Inny szablon jest już nazywany "{ $name }"
settings-templates-words = { $n ->
    [one] { $n } słowo
    [few] { $n } słowa
    [many] { $n } słów
   *[other] { $n } słów
}

templates-imported = { $added ->
    [one] { $added } Scenariusz importowany
    [few] { $added } zaimportowane szablony
    [many] { $added } zaimportowanych szablonów
   *[other] { $added } Zaimportowane szablony
}
templates-imported-renamed = Zmiana nazwy, aby uniknąć starcia: { $names }
templates-imported-skipped = Nie można odczytać: { $names }
templates-import-filter = Szablony (.md, .djot)
templates-exported = { $n ->
    [one] Wywóz { $n } szablonu
    [few] Eksportowane { $n } szablony
    [many] Eksportowane { $n } szablonów
   *[other] Wywóz { $n }
}
templates-preset-applied = Dodano "{ $name }"

templates-delete-title = Skasować ten szablon?
templates-delete-body = "{ $name }" zostanie usunięty z tego projektu. Uwagi, które zostały już z niego utworzone, nie są naruszone.
templates-delete-confirm = Usuń

menu-document = &Dokument
menu-insert-template = &Wstaw szablon
menu-insert-template-none = Brak szablonów w tym projekcie
menu-save-as-template = Z&apisz jako template…

save-as-template-title = Zapisz jako szablon
save-as-template-explain = Tekst, który edytujesz, staje się szablonem, który możesz wrzucić gdziekolwiek indziej.
save-as-template-name = Nazwa
save-as-template-placeholder = Karta postaci
save-as-template-duplicate = Szablon o nazwie "{ $name }" już istnieje
save-as-template-empty-editor = Ten edytor jest pusty.
save-as-template-confirm = Zapisz szablon
save-as-template-saved = Zapisane "{ $name }" jako szablon
template-inserted = Dodano "{ $name }"

note-template-preset-character-sheet = Karta postaci
note-template-preset-location = Miejsce
note-template-preset-artifact = Przedmiot
note-template-preset-beat-sheet = Plan wydarzeń
note-template-preset-faction = Frakcja
note-template-preset-research-note = Notatka badawcza

note-template-set-essentials = Podstawowe
note-template-set-everything = Każdy szablon

note-template-section-identity = Tożsamość
note-template-section-appearance = Wygląd
note-template-section-voice = Głos
note-template-section-psychology = Wewnętrzne życie
note-template-section-history = Historia
note-template-section-arc = Łuk fabularny
note-template-section-first-impression = Pierwsze wrażenie
note-template-section-in-the-story = W tej historii
note-template-section-the-thing-itself = Sam obiekt
note-template-section-the-scene = Scena
note-template-section-the-shape = Kształt
note-template-section-what-it-is = Co to jest?
note-template-section-source = Źródło
note-template-section-what-it-says = Co tam jest napisane?

note-template-field-full-name = Pełna nazwa
note-template-field-known-as = Znany również jako, śledzony na polu Alias inspektora
note-template-field-age = Wiek
note-template-field-role-in-story = Rola w historii
note-template-field-build-and-features = Zbuduj i wyróżniaj cechy
note-template-field-habitual-bearing = Nakładanie habitualne
note-template-field-speech-patterns = Wzory mowy i słowne
note-template-field-what-they-never-say = Co nigdy nie mówią
note-template-field-want = Czego chcą?
note-template-field-need = Czego potrzebują, ale o tym nie wiedzą.
note-template-field-fear = Czego się boją?
note-template-field-flaw = Wada, która ich kosztuje.
note-template-field-formative-event = Zdarzenie Fortyfikujące
note-template-field-relationships = Związki
note-template-field-starts-as = Zaczyna się jako
note-template-field-ends-as = Kończy jako
note-template-field-what-you-notice-first = To, co zauważysz najpierw
note-template-field-sound-and-smell = Dźwięk i zapach
note-template-field-light-and-weather = Światło i pogoda
note-template-field-what-happened-here = Co tu się stało?
note-template-field-who-lives-or-works-here = Kto tu mieszka lub pracuje?
note-template-field-scenes-set-here = Sceny ustawione tutaj
note-template-field-why-it-matters = Dlaczego to ma znaczenie?
note-template-field-appearance = Wygląd
note-template-field-age-and-origin = Wiek i pochodzenie
note-template-field-what-it-does = Co to robi?
note-template-field-who-holds-it = Kto ją trzyma?
note-template-field-who-wants-it = Kto tego chce?
note-template-field-pov = Punkt widzenia
note-template-field-time-and-place = Czas i miejsce
note-template-field-goal = Cel
note-template-field-conflict = Konflikt
note-template-field-turn = Punkt zwrotny
note-template-field-exit-emotion = Emocje w drodze powrotnej
note-template-field-purpose = Cel
note-template-field-who-leads-it = Kto to prowadzi?
note-template-field-resources = Zasoby
note-template-field-allies-and-enemies = Alianci i wrogowie
note-template-field-what-it-wants-now = Czego teraz chce?
note-template-field-where-from = Skąd pochodzi.
note-template-field-page-or-link = Strona lub link
note-template-field-key-facts = Kluczowe fakty
note-template-field-how-it-is-used = Jak to jest używane w historii
