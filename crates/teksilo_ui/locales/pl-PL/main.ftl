# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

menu-work = P&rojekt
menu-new-work = N&owy projekt
menu-open-work = O&twórz projekt…
menu-new-window = &Nowe okno
menu-import-from = &Importuj z
menu-create-from = &Utwórz z
menu-import-plume = &Plume Creator (.plume)…
menu-import-document = &Dokumenty (Markdown, Word, ODT) …
menu-import-manuskript = &Manuskript (.msk)…
menu-export = &Eksportuj
menu-export-book = Eksportuj książkę
menu-export-part = Eksportuj część
menu-export-chapter = Eksportuj rozdział
menu-export-scene = Eksportuj scenę
menu-export-note = Eksportuj notatkę
menu-export-paratext = Eksportuj paratekst
menu-export-folder = Eksportuj folder
menu-export-choose = Wybierz…
menu-export-none = Otwórz dokument do wywozu
menu-save = &Zapisz
menu-save-as-file = Za&pisz jako pojedynczy plik…
menu-save-as-folder = Zapisz &jako folder…
menu-backup = Utw&órz kopię zapasową teraz
menu-close-work = Za&mknij projekt
menu-welcome = &Witaj…
menu-settings = &Ustawienia
menu-quit = Z&akończ

menu-view = &Widok
menu-outline = &Struktura
menu-search = Sz&ukaj w projekcie
menu-timeline = Wr&óć do przeszłości
menu-search-preview = P&odgląd wyszukiwania
menu-fullscreen = P&ełny ekran
menu-focus-mode = &Tryb bez rozpraszania
menu-format = &Format
menu-scene-break = Wst&aw przerwę w scenie
menu-major-scene-break = Wstaw powa&żną przerwę na scenie

menu-go = Pr&zejdź
menu-go-next-scene = &Następna scena
menu-go-prev-scene = P&oprzednia scena
menu-go-next-chapter = Na&stępny rozdział
menu-go-prev-chapter = &Poprzedni rozdział
menu-go-next-note = N&astępna uwaga
menu-go-prev-note = Pop&rzednia uwaga

menu-tools = &Narzędzia
menu-help = &Pomoc

ctx-add = &Dodaj
ctx-new-item = &Nowy element
ctx-new-folder = &Nowy folder
ctx-rename = Z&mień nazwę
ctx-duplicate = D&uplikuj
ctx-import-here = &Importuj tutaj…
ctx-indent = &Zwiększ wcięcie
ctx-outdent = Zm&niejsz wcięcie
ctx-trash = &Przenieś do kosza
ctx-open-to-side = &Otwarte na bok
ctx-open = &Otwórz
ctx-reveal-in-outline = &Ujawnienie w zarysie
ctx-move-up = &Przesuń w górę
ctx-move-down = &Przesuń w dół

ctx-tab-close = &Zamknij
ctx-tab-close-others = Z&amknij pozostałe
ctx-tab-close-all = Za&mknij wszystkie
ctx-tab-open-to-side = &Otwarte na bok
ctx-tab-open-in-main = &Otwórz w głównej tafli
ctx-tab-move-to-side = P&rzesuń się na bok.
ctx-tab-move-to-main = P&rzesuń się do głównej szyby
ctx-tab-move-to-new-window = &Przenieś do nowego okna
ctx-tab-move-window-unsaved = Zapisz ten projekt najpierw: drugie okno otwiera się na plik na dysku
ctx-tab-pin = Prz&ypisuj tę zakładkę
ctx-tab-unpin = O&twórz tę zakładkę
tab-pinned-tooltip = Zapinane: "Zamknij innych" i "Zamknij wszystkich" zostaw otwarte

create-book = Książka
create-part = Część
create-chapter = Rozdział
create-scene = Scena
create-note = Notatka
create-note-folder = Folder notatek
create-folder = Folder
create-paratext = Paratekst
create-paratext-folder = Folder paratekstów
create-book-end = Koniec książki
create-story-bible-entry = Wpis w biblii opowieści…
type-book-start = Uruchom książkę
type-text = Tekst

new-item-book = Nowa książka
new-item-part = Nowa część
new-item-chapter = Nowy rozdział
new-item-note = Nowa notatka
new-item-note-folder = Nowy folder notatek
new-item-folder = Nowy folder
new-item-paratext = Nowy paratext
new-item-paratext-folder = Nowy folder paratext
new-item-story-bible-entry = Nowy wpis biblijny

placement-inside = wewnątrz
placement-after = po
placement-after-parent = po macierzystym
placement-top-level = górny poziom


ctx-promote = P&rzekształć w
promote-chapter-folder = Rozdział jako folder
promote-flat-chapter = Rozdział bez scen
promote-lossy-title = Nie ma gdzie zatrzymać tekstu.
promote-lossy-text = { $target } nie ma gdzie zachować tych: { $kinds }. Przesuń lub wyczyść ten tekst najpierw, a następnie przekonwertować.
content-scene-text = Tekst sceny
content-note-text = Tekst notatki
content-book-title = Tytuł książki
content-book-subtitle = Podtytuł książki
content-part-title = Tytuł części
content-chapter-title = Tytuł rozdziału
content-epigraph-text = Motto
content-paratext-text = Paratekst
promote-blocked-title = Rozdział nie pusty
promote-blocked-text = { $count ->
    [one] Ten rozdział nadal posiada { $count } element. Przenieś lub wyrzucić go przed przekształceniem go do płaskiego rozdziału.
    [few] Ten rozdział nadal posiada { $count } elementy. Przenieś lub wyrzucić je przed przekształceniem go do płaskiego rozdziału.
    [many] Ten rozdział nadal zawiera { $count } pozycji. Przenieś lub wyrzuć je przed przekształceniem na płaski rozdział.
   *[other] Ten rozdział nadal posiada { $count } pozycji. Przenieś lub wyrzucić je przed przekształceniem go do płaskiego rozdziału.
}

inspector = Właściwości
inspector-empty = Otwórz przedmiot, żeby go sprawdzić.
inspector-promote = Przekształć w…
statusbar-toggle-outline = Pokaż lub ukryj segregator
statusbar-toggle-inspector = Pokaż lub ukryj właściwości
statusbar-save-unsaved = Niezapisane zmiany. Kliknij, aby zapisać
statusbar-save-saved = Wszystkie zmiany zapisane
statusbar-save-autosave = Autosave jest włączony. Zmiany są zapisywane podczas zapisywania
statusbar-saving = Zapisywanie…
statusbar-word-count = { $count ->
    [one] { $count } słowo
    [few] { $count } słowa
    [many] { $count } słów
   *[other] { $count } słowa
}

statusbar-word-count-tooltip = Słowa w scenie, którą edytujesz
statusbar-word-char-count = { $words ->
    [one] { $words } słowo
    [few] { $words } słowa
    [many] { $words } słów
   *[other] { $words } słowa
} · { $chars ->
    [one] { $chars } znak
    [few] { $chars } znaki
    [many] { $chars } znaków
   *[other] { $chars } znaku
}

session-toggle = Sesja napisania: uruchom lub przerw skoncentrowany sprint
session-configure = Ustaw bramkę słowa sesji i limit czasowy
session-configure-title = Nagrywanie sesji
session-word-goal = Docelowa liczba słów
session-time-limit = Termin
session-time-limit-unit = min.
session-no-goal = Brak celu
session-no-limit = Brak ograniczeń
session-reset = Zresetuj sesję
session-readout = Słowa: { $words } · { $time }
session-readout-timed = { $words } słowa · { $time } lewo
statusbar-focus-exit = Tryb "exit distraction- free"
statusbar-focus-synopsis = Streszczenie
statusbar-focus-go-prev = Poprzednia pozycja (Alt + Początek rozdziału)
statusbar-focus-go-next = Następny punkt (Alt + w dół)

rail-settings = Ustawienia

language = Język
theme = Motyw
english = Angielski
french = Francuski
light = Jasny
dark = Ciemny
settings-text-width = Szerokość tekstu
settings-preview-width = Szerokość podglądu wyszukiwania
settings-autosave = Automatyczne zapisywanie na dysk
settings-show-welcome = Pokaż wyrzutnię przy starcie
settings-show-welcome-tip = Po przerwie, ostatni projekt otwiera się ponownie.
settings-check-for-updates = Sprawdź nowe wersje
settings-check-for-updates-tip = Raz dziennie, co najwyżej, Skribisto pyta strony internetowej projektu, która wersja jest aktualna. Żądanie nie ma identyfikatora, a nawet wersji uruchomiony: porównanie odbywa się na tym komputerze.

settings-title = Ustawienia
settings-close = Zamknij
settings-search = Ustawienia wyszukiwania
settings-reset = Przywróć domyślne
settings-done = Gotowe
settings-cancel = Anuluj
settings-apply = Zastosuj
settings-ok = OK
settings-reset-confirm-title = Zresetować wszystkie ustawienia do domyślnych?
settings-reset-confirm-body = Przywraca wygląd, edytor i ustawienia zapisu tej aplikacji. Nie ma to wpływu na własne ustawienia, skróty klawiszowe i zachowane style. Nie można tego cofnąć.
settings-empty-title = Nie ma tu jeszcze ustawień
settings-empty-hint = Ta sekcja uzyska opcje w przyszłej aktualizacji.

settings-sec-appearance-behaviour = Zachowanie wyglądu
settings-sec-editor = Edytor
settings-sec-spelling = Pisownia
settings-sec-backup = Synchronizacja kopii zapasowej
settings-sec-compile = Kompilacja i eksport
settings-page-appearance = Wygląd
settings-field-image-size-policy = Duże obrazy
settings-image-policy-ask = Pytaj za każdym razem
settings-image-policy-keep = Zachowaj oryginał
settings-image-policy-downscale = Optymalizuj
settings-hint-image-size-policy = Co robić, gdy wstawiony obraz jest większy niż manuskrypt potrzebuje.
settings-page-notifications = Powiadomienia
settings-page-scene = Scena
settings-page-synopsis = Streszczenie
settings-page-notes = Uwagi
settings-page-editor-behavior = Zachowanie edytora
settings-page-goals = Licznik celów Word Count
settings-page-games = Pisanie gier
settings-page-corkboard = Tablica korkowa
settings-page-distraction-free = Bezproblemowo
settings-page-dictionaries = Słowniki
settings-page-autosave = Zapis automatyczny
settings-page-export = Formaty eksportu
settings-page-paratext = Struktury paratextu
settings-page-keymap = Skróty klawiszowe
settings-keymap-filter = Skróty filtrów

settings-desc-sec-appearance-behaviour = Jak wygląda aplikacja i co robi, kiedy się zaczyna.
settings-desc-sec-editor = Powierzchnia zapisu: jak wygląda i co robi jak piszesz.
settings-desc-sec-spelling = Sprawdzanie pisowni i słowników.
settings-desc-sec-backup = Jak twoja praca dociera do dysku i jakie kopie są przechowywane.
settings-desc-sec-compile = Co pozostawia Skribisto i w jakim stanie.
settings-desc-sec-work = Ustawienia, które należą do tego projektu i podróżują wewnątrz jego pliku.
settings-desc-sec-extensions = Strony włożone przez rozszerzenia zainstalowane tutaj.
settings-desc-group-typography = Jedna strona na rodzaj tekstu: krój, rozmiar, wysokość linii i odstępy.
settings-desc-appearance = Język interfejsu, motyw, rozmiar tekstu i okno początkowe.
settings-desc-notifications = Każda wiadomość, którą wyświetliła ta sesja, oraz działania, które możesz powtórzyć.
settings-desc-scene = Jak ustawiona jest proza sceny: twarz, rozmiar, wysokość linii, tiret i odstępy.
settings-desc-synopsis = Jak ustawiona jest tafla synopsis, niezależnie od rękopisu.
settings-desc-notes = Jak są ustawiane notatki, niezależnie od rękopisu.
settings-desc-corkboard = Rozmiar karty, co pokazuje karta, i jak zarząd organizuje je.
settings-desc-distraction-free = Typografia, szerokość kolumn i pasek sterujący do zapisu pełnoekranowego.
settings-desc-distraction-free-themes = Biblioteka tematyczna do zapisu pełnoekranowego: wysłane i własne.
settings-desc-editor-behavior = Szerokość tekstu, położenie synopsis, przewijanie maszyny do pisania i podświetlenie opiekuna.
settings-desc-punctuation = Styl typograficzny domu rozpoczynają się od nowych projektów. Każdy projekt podąża za tym, chyba że ustawi swój własny.
settings-desc-goals = Słowo i charakter celów, i jak słowa są liczone.
settings-desc-games = Ograniczenia, które stawiasz sobie podczas pisania, takie jak zawsze do przodu.
settings-desc-spellcheck = Pojedynczy przełącznik, który wszędzie włącza i wyłącza sprawdzanie pisowni.
settings-desc-dictionaries = Instaluj, usuwaj i przeglądaj słowniki pisowni na tej maszynie.
settings-desc-autosave = Czy Twoje edycje są zapisywane na dysk samodzielnie.
settings-desc-backup = Kiedy są wykonywane kopie, gdzie są przechowywane i jak wiele. Każdy projekt postępuje zgodnie z tym, chyba że ustawia własne.
settings-desc-export = Style każdego eksportu kompiluje poprzez: wysyłane i własne.
settings-desc-paratext = Z przodu i z tyłu materia nowy projekt może rozpocząć się od.
settings-desc-keymap = Każdy skrót i to, do czego jest związany.
settings-page-user = Użytkownik
settings-desc-user = Kim jesteś, za komentarze, które piszesz.
settings-group-identity = Tożsamość
settings-field-user-name = Jak się nazywasz?
settings-field-user-name-placeholder = Opcjonalnie
settings-field-user-name-hint = Znaki komentarzy i odpowiedzi, które piszesz. Oddzielny od nazwy autora projektu, który jest wierszem książki i podróżuje wewnątrz pliku.
settings-field-user-initials = Twoje inicjały
settings-field-user-initials-placeholder = Z twojego imienia.
settings-field-user-initials-hint = Co procesor słowa pokazuje obok komentarza na marginesie. Zostaw puste używać tych pokazanych, pobranych z nazwy.
settings-field-user-hint = Oba są opcjonalne i odnoszą się do każdego projektu na tym komputerze. Zmienia je oznacza komentarze, które piszesz dalej. Komentarze już napisane zachować nazwę, pod którą zostały napisane.
settings-desc-author = Nazwa tego projektu.
settings-desc-structure = Czy rozdziały tego projektu to foldery czy pojedyncze elementy.
settings-desc-language = Język prozy tego projektu jest sprawdzany.
settings-desc-work-backup = Polityka tworzenia kopii zapasowych tego projektu, albo polityka generalna.
settings-desc-personal-dictionary = Słowa, które ten projekt traktuje jako poprawnie napisane.
settings-desc-tags = Kolorowe etykiety, którymi ten projekt znakuje swoje pozycje.
settings-desc-templates = Szablony notatki można umieścić podczas pisania w tym projekcie.
settings-desc-text-replacements = Skróty, które rozszerzają się podczas pisania w tym projekcie.
settings-desc-work-punctuation = Cytaty, kreski i odstępy prozy tego projektu następują.

settings-group-typography = Typografia
settings-group-writing-column = Kolumna zapisu
settings-group-writing-view = Widok zapisu
settings-group-distraction-free-strip = Taśma kontrolna
settings-group-theme = Motyw
settings-group-language = Język
settings-group-startup = Uruchamianie
settings-group-updates = Aktualizacje
settings-group-autosave = Zapis automatyczny
settings-field-typeface = Krój pisma
settings-field-size = Rozmiar tekstu

editor-size-changed-manuscript = Rozmiar tekstu manuskryptu: { $percent }
editor-size-changed-synopsis = Rozmiar tekstu synopsis: { $percent }
editor-size-changed-notes = Rozmiar tekstu w uwagach: { $percent }
editor-size-changed-corkboard = Rozmiar karty korbowej: { $percent }
editor-size-changed-corkboard-expanded = Rozszerzony rozmiar tekstu edytora: { $percent }
editor-size-changed-distraction-free = Rozróżnienie - wolny od tekstu rozmiar: { $percent }

menu-text-size-increase = Z&większ rozmiar tekstu
menu-text-size-decrease = Z&mniejsz rozmiar tekstu
menu-text-size-reset = P&rzywróć rozmiar tekstu
settings-field-line-height = Wysokość linii
settings-field-first-line-indent = Wcięcie na pierwszej linii
settings-field-paragraph-spacing-before = Przestrzeń przed akapitem
settings-field-paragraph-spacing-after = Przestrzeń po punkcie
settings-field-column-width = Szerokość kolumny
settings-distraction-free-width-hint = Stosuje się tylko wtedy, gdy włączony jest tryb wolny od rozpraszania. Szerokość kolumn Scena, Synopsis i Notes jest nietknięta.
settings-distraction-free-title = Zachowaj nazwę elementu
settings-distraction-free-word-count = Zachowaj słowo liczenie
settings-distraction-free-session = Zachowanie sesji pisarskiej
settings-distraction-free-go-to = Zatrzymaj przycisk Go do …
settings-distraction-free-go = Zachowaj poprzednie i następne przyciski
settings-distraction-free-chrome-hint = Przycisk Exit zawsze pozostaje, cokolwiek wybierzesz tutaj. To jest twoja droga wyjścia, jeśli Escape jest zajęty.
settings-field-app-theme = Motyw
settings-theme-light = Jasny
settings-theme-dark = Ciemny
settings-field-text-scale = Rozmiar tekstu interfejsu
settings-field-language = Język interfejsu
settings-synopsis-placement = Pozycja synopsis
settings-synopsis-placement-none = Brak
settings-synopsis-placement-top = Powyżej
settings-synopsis-placement-side = Obok
settings-typewriter = Przewijanie maszynisty do pisania
settings-typewriter-tip = Utrzymuje linię, którą piszesz na stałej wysokości, podczas gdy manuskrypt przewija się pod nią. Kliknięcie nadal umieszcza opiekuna tam, gdzie klikniesz.
settings-typewriter-position = Pozycja linii
settings-typewriter-position-top-third = Pierwsza trzecia
settings-typewriter-position-middle = Środkowy
settings-typewriter-position-bottom-quarter = Kwadrat dolny
settings-highlight-scope = Zaznacz wokół opiekuna
settings-highlight-scope-none = Brak
settings-highlight-scope-sentence = Wyrok
settings-highlight-scope-paragraph = Paragraf
settings-highlight-scope-tip-none = Nic nie jest zacienione, jak piszesz.
settings-highlight-scope-tip-sentence = Shade zdanie piszesz, więc wyróżnia się od tych wokół niego.
settings-highlight-scope-tip-paragraph = Shade cały ustęp piszesz, aby zachować przejście pracujesz w perspektywie.
settings-group-container-views = Widok kontenera
settings-remember-view = Zapamiętaj ostatni widok dla każdego typu elementu
settings-remember-view-tip = Otwórz pojemnik na widok ostatnio używany dla tego typu
settings-remember-view-tip-more =
    Książka, Część i Rozdział każdy oferuje kilka widoki (jego własna strona, pełna
    manuskrypt, pełna synopsis). Włącz to i każdy typ ponownie otwiera się na
    widok ostatnio wybrany dla niego, np. przełącz jeden rozdział do pełnego rozdziału i
    Następny rozdział, który otwierasz, pokazuje także cały rozdział. Każdy typ pamięta swój własny widok.
settings-group-counting = Liczenie słów
settings-counting-auto = Automatyczne (według języka)
settings-counting-whitespace = Podział na przestrzenie
settings-counting-unicode-words = Słowa Unicode
settings-counting-cjk-hybrid = CJK- aware (na znak)
settings-counting-hint = Automatyczne liczenie chińskich i japońskich według znaków i każdego innego języka przez słowo. Zmień go tylko wtedy, gdy liczba wygląda źle dla Twojego języka.
settings-group-goals-display = Wyświetl
settings-show-characters = Pokaż liczbę znaków w pasku stanu
settings-autosave-hint = Zmiany są zapisywane na dysk automatycznie podczas pisania.

settings-sec-work = Projekt
settings-page-structure = Struktura
settings-page-author = Autor
settings-field-author-name = Nazwa autora
settings-field-author-placeholder = Opcjonalnie
settings-field-author-hint = Pojawia się na skompilowanej stronie tytułu i w eksportowanych metadanych plików. Zostaw puste, aby ją pominąć. Jest to bajka książki. Nazwa twoich komentarzy jest podpisana w Ustawieniach ▸ Użytkownik.
settings-group-chapters = Rozdziały
tidy-titles-title = Tytuły rozdziałów Tidy
tidy-titles-none = Żaden rozdział ani tytuł nie jest tylko powtarzaniem własnego numeru.
tidy-titles-lead = { $count ->
    [one] Jeden rozdział lub część jest zatytułowana tylko jego własny numer.
    [few] { $count } rozdziały i części są nazwane tylko ich własnym numerem.
    [many] { $count } rozdziałów i części są zatytułowane tylko ich własny numer.
   *[other] { $count } rozdziałów i części są zatytułowane tylko z ich własnym numerem.
    }
tidy-titles-explain = Wyczyszczenie tych tytułów pozostawia każdy nazwany przez liczbę, którą już zna książka, tą samą liczbę, jaką drukuje eksporter. Nic innego się nie zmienia, a jedno cofnięcie umieszcza je z powrotem.
menu-document-tidy-titles = &Titles…
settings-group-numbering = Numeracja
settings-number-chapters = Liczba rozdziałów i części
settings-number-chapters-tip = Rozdziały i części noszą numer opracowany z miejsca, z którego siedzą w książce, pokazany obok ich tytułu tutaj, i wydrukowany przez eksport.
settings-number-chapters-tip-more = Numer nigdy nie jest zapisany w tytule, więc pozostaje w prawo, gdy ponownie zamówić, wstawić lub usunąć. Wyłącz to i eksportu drukuje tytuły samodzielnie, bez względu na styl eksportu prosi. Aby pozostawić jeden rozdział (prolog, interlude), użyj zamiast tego przełącznika inspektora Numeracja: że trzyma go w książce, ale zatrzymuje go biorąc liczbę.
settings-part-resets-chapter = Przywróć numery rozdziałów w każdej części
settings-part-resets-chapter-tip = Domyślnie wyłączone: rozdziały przebiegają prosto przez części książki, więc "Część druga" otwiera się na rozdziale 11.
settings-part-resets-chapter-tip-more = Jest to zwyczajowa praktyka handlowa i czego oczekuje czytelnik. Włącz ją dla książki, której części mają być odczytywane jako oddzielne tomy, gdzie każda z nich otwiera się na rozdziale pierwszym.
settings-chapter-flat = Rozdziały bez scen
settings-chapter-flat-hint = Na: rozdział jest pojedynczym elementem. Piszesz do niego i nie zawiera żadnych scen. Off: rozdział jest folderem. Wciąż piszesz do niego, ale może również pomieścić sceny. Nowe rozdziały podążają za tym ustawieniem; istniejące konwertują się przez Probote.

settings-styles-builtin = Built- in styles
settings-styles-user = Moje style
settings-styles-builtin-badge = Budowa - w
settings-styles-duplicate = Duplikuj
settings-styles-edit = Edycja
settings-styles-delete = Usuń
settings-styles-import = Znaczenie …
settings-styles-export = Eksport …
settings-styles-copy-suffix = (kopia)
settings-styles-json-filter = Styl eksportu
settings-styles-editor-title = Edytuj styl
settings-styles-editor-none = Wybierz własny styl do edycji albo zduplikuj styl wbudowany.
settings-styles-imported = Style importowane
settings-styles-import-failed = Nie można zaimportować stylu
settings-styles-exported = Eksportowany styl
settings-styles-export-failed = Nie można eksportować stylu
settings-styles-field-name = Nazwa
settings-styles-field-chapters = Pozycja działu
settings-styles-field-parts = Nagłówki części
settings-styles-field-scene-break = Przerwa w scenie
settings-styles-field-major-scene-break = Poważna przerwa w scenie
settings-styles-field-spacing = Odstęp między liniami
settings-styles-field-justify = Uzasadnienie
settings-styles-field-notes = Dołącz notatki
settings-styles-field-synopses = Dołącz synopse
settings-styles-field-scene-titles = Dołącz tytuły sceny
settings-styles-field-epigraphs = Dołącz epigrenę
settings-styles-field-paratexts = Uwzględnij parateksty
settings-styles-field-epigraph-placement = Położenie odwzorowania
settings-styles-epigraph-after = Po tytule
settings-styles-epigraph-before = Przed tytułem
settings-styles-field-footnotes = Dołącz przypisy
settings-styles-field-footnote-numbering = Numeracja przypisu
settings-styles-footnote-numbering-continuous = Ciągłe
settings-styles-footnote-numbering-per-chapter = Wznowienie każdego rozdziału
settings-styles-footnote-numbering-per-book = Wznowienie każdej książki
settings-styles-field-images = Obrazy
menu-image = &Obraz
settings-styles-images-beside = Poza dokumentem
settings-styles-images-embed = Wnętrze dokumentu
settings-styles-images-omit = Wypad.
settings-styles-group-round-trip = Wysyłanie do redaktora
settings-styles-field-comments = Dołącz komentarze
settings-styles-field-round-trip-marks = Dołącz znaczniki ponownego importu
settings-styles-round-trip-hint = Tylko DOCX i ODT. Niewidoczne znaczniki pozwalają aktualizować projekt przy ponownym imporcie poprawionego pliku zamiast tworzyć kolejną kopię.
settings-styles-group-pages = Strony
settings-styles-field-cover = Otwórz z pokrywą
settings-styles-field-word-count = Licz słowa na stronie tytułu
settings-styles-field-page-books = Nowa strona w każdej książce
settings-styles-field-page-parts = Nowa strona w każdej części
settings-styles-field-page-chapters = Nowa strona w każdym rozdziale
settings-styles-field-page-paratexts = Nowa strona przy każdym paratexcie
settings-styles-heading-none = Brak nagłówka
settings-styles-heading-numbered = Tylko liczba
settings-styles-heading-title = Tylko tytuł
settings-styles-heading-both = Numer + tytuł
settings-styles-break-blank = Puste linie
settings-styles-break-none = Brak
settings-styles-spacing-single = Pojedyncze
settings-styles-spacing-onehalf = 1½
settings-styles-spacing-double = Podwójne

welcome-title = Witamy w Skribisto
welcome-version = Wersja { $version }
welcome-update-available = Wersja { $version } jest dostępna
welcome-update-tip = Otwiera stronę pobierania w przeglądarce.
welcome-search = Wyszukiwarka
welcome-open = Otwórz
welcome-new-work = Nowy projekt
welcome-create-from = Utwórz z…
welcome-recent-works = Ostatnie projekty
welcome-empty-recents = Nie ma jeszcze nowych prac.
welcome-no-matches = Ostatnie prace nie pasują do twoich poszukiwań.
welcome-learn-soon = Przewodniki i napiwki wkrótce się pojawią.
welcome-about-blurb = Skribisto, a Rust + Teksilo przeredagować aplikację pisarską.
welcome-tagline = * Ciche miejsce na pisanie długich rzeczy. *
welcome-github = GitHub
welcome-discord = Discord
nav-works = Projekty
nav-examples = Przykłady
nav-learn = Ucz się
nav-about = O

synopsis = Streszczenie
epigraph = Motto
pane-manuscript = Manuskrypt
corkboard = Tablica korkowa
overview = Przegląd

corkboard-card-count = { $count ->
    [one] { $count } karta
    [few] { $count } karty
    [many] { $count } kart
   *[other] { $count } kart
}
corkboard-child-count = { $count ->
    [one] { $count } pozycja
    [few] { $count } pozycje
    [many] { $count } pozycji
   *[other] { $count } pozycji
}
corkboard-view-nested = Zagnieżdżone
corkboard-view-flat = Płaski
corkboard-layout-hint = Nested pokazuje bezpośrednie dzieci kontenera. Otwórz kartę folderu, aby wejść do środka. Mieszkanie pokazuje każdą scenę w całym pojemniku na raz.
corkboard-card-size = Wielkość karty
corkboard-search-placeholder = Karty filtracyjne …
corkboard-empty-title = Jeszcze nic tu nie ma
corkboard-empty-hint = Użyj "＋ Nowy" powyżej, aby dodać pierwszy.
corkboard-new = Nowy
corkboard-show-card-numbers = Liczba kart
corkboard-modal-size = Rozszerzony rozmiar edytora
corkboard-scope-hint = Dotyczy każdego otwartego korkociągu, każdego projektu.
corkboard-sort-manuscript = Porządek manuskryptu
corkboard-sort-title-asc = Tytuł A- Z
corkboard-sort-title-desc = Tytuł ZA
corkboard-card-number = Karta { $number }

duplicate = Duplikuj
duplicate-n = { $count ->
    [one] Duplikat { $count } karta
    [few] Duplikat { $count } kart
    [many] Duplikat { $count } kart
   *[other] Duplikaty kart { $count }
}
set-label-n = { $count ->
    [one] Ustaw etykietę na { $count } karcie
    [few] Ustaw etykietę na { $count } karty
    [many] Ustaw etykietę na { $count } kart
   *[other] Ustaw etykietę na kartach { $count }
}
move-to-trash-n = { $count ->
    [one] Przenieś { $count } kartę do kosza
    [few] Przenieś { $count } karty do kosza
    [many] Przenieś { $count } kart do kosza
   *[other] Przenieś { $count } kart do kosza
}
reveal-in-outline = Ujawnienie w zarysie

corkboard-move-to = Przenieś do …
corkboard-move-to-n = { $count ->
    [one] Przenieś { $count } kartę do …
    [few] Przenieś { $count } karty do …
    [many] Przenieś { $count } kart do …
   *[other] Przenieś { $count } kart do …
}
corkboard-move-picker-title = Przenieś do …
corkboard-move-picker-empty = Nie ma jeszcze segregatorów.
corkboard-move-picker-cancel = Anuluj
corkboard-move-here = Przesuń tutaj
corkboard-moved-ok = { $count ->
    [one] { $count } karta przeniesiona
    [few] { $count } karty przeniesione
    [many] { $count } kart przeniesionych
   *[other] { $count } karty przeniesione
}
corkboard-move-into-self = Kontener nie może być przesunięty wewnątrz siebie. Wybierz cel poza nim.
corkboard-move-failed = Te karty nie mogły być tam przeniesione.

overview-col-title = Tytuł
overview-col-type = Rodzaj
overview-col-label = Adnotacja
overview-col-tags = Etykiety
overview-col-books = Książki
overview-col-own-words = Słowa
overview-col-total-words = Razem
overview-row-count = { $count ->
    [one] { $count } pozycja
    [few] { $count } pozycje
    [many] { $count } pozycji
   *[other] { $count } pozycji
}
overview-search-placeholder = Filtr items…
overview-expand-all = Rozwiń wszystkie
overview-collapse-all = Zwiń wszystko
overview-table-label = Spis treści
overview-empty-title = Jeszcze nic tu nie ma
overview-gone-title = Ten kontener zniknął.
overview-gone-hint = Został przeniesiony do kosza. Przywróć go, lub zamknij tę kartę.
overview-empty-hint = Użyj "＋ Nowy" powyżej, aby dodać pierwszy.
corkboard-grid-label = Karty korkotwórcze
corkboard-rename-field = Zmień nazwę elementu
corkboard-expand-synopsis = Rozwiń synopsę
corkboard-synopsis-modal-title = Streszczenie
corkboard-badge-scene = Scena
corkboard-badge-chapter = Rozdział
corkboard-badge-part = Część
corkboard-badge-book = Książka
corkboard-badge-note = Notatka
corkboard-badge-folder = Folder
corkboard-badge-paratext = Paratekst
corkboard-badge-text = Tekst
corkboard-badge-end = Koniec
settings-group-corkboard-layout = Układ
settings-group-corkboard-cards = Karty
corkboard-show-word-count = Pokaż liczbę słów na kartach
text-heading = Tekst
no-content = Ta pozycja nie ma treści edytowalnej.
untitled = Bez tytułu
placeholder-title = Tytuł…
placeholder-subtitle = Podtytuł…
placeholder-chapter-title = Dział tytle…
split-editor = Edytor podziału
close-split-view = Zamknij widok podzielony
drop-open-here = Otwórz tutaj.
drop-open-to-side = Otwórz na bok

segment-chapter = Rozdział
segment-part = Część
segment-book = Książka
segment-pace = Tempo
segment-notes = Uwagi
segment-story-bible = Biblia opowieści
segment-note-own = Notatka
segment-note-details = Szczegóły
segment-note-in-prose = W prozie
pace-placeholder = Pojawia się tu organizator Pace 'ów.
pace-empty-title = Zaplanuj tempo tej książki
pace-empty-body = Ustaw słowo gola i termin, a Skribisto pracuje na bieżąco, aby dostać się tam.
pace-start-planning = Rozpocząć planowanie
pace-section-schedule = Schemat
pace-section-progress = Postępy
pace-advancement = Postępy
pace-goal = Docelowa liczba słów
pace-deadline = Termin
pace-active = Plan pisania aktywny
pace-day-mon = pon
pace-day-tue = wt
pace-day-wed = śr
pace-day-thu = czw
pace-day-fri = pt
pace-day-sat = sob
pace-day-sun = niedz
pace-card-written = słowa napisane
pace-card-of-goal = celu
pace-card-rate = słowa / dzień pisania
pace-card-days-left = Pozostały dni pisania
pace-card-streak = passa dzienna
pace-card-ahead = słowa wyprzedzające tempo
pace-card-behind = słowa za tempem
pace-charts-empty = Wykresy postępu pojawiają się tutaj, gdy liczba słów jest zapisywana na zapisach.
pace-chart-progression = Słowa napisane w stosunku do celu
pace-chart-words-per-day = Słowa na dzień
pace-series-actual = Rzeczywiste
pace-series-target = Cel
pace-series-words-per-day = Słowa / dzień
pace-daily-target-line = Nawet tempo: { $count ->
    [one] { $count } słowo / dzień
    [few] { $count } słowa / dzień
    [many] { $count } słów / dzień
   *[other] { $count } słów / dzień
}
pace-section-holidays = Wakacje
pace-section-milestones = Kamienie milowe
pace-holidays-none = Każdy zaplanowany dzień się liczy.
pace-holiday-label = Nazwa wakacyjna
pace-add-holiday = Dodaj
pace-remove = Usuń
pace-milestones-none = Umieść jeden na części lub rozdziale w inspektorze.
full-chapter = Pełny rozdział
full-part = Pełna część
full-book = Pełna książka
full-synopsis = Pełna synopsja
rename = Zmień nazwę
set-label = Ustaw etykietę
insert-scene = Wstaw scenę
insert-chapter = Wstaw rozdział
split-scene = Scena podziału
move-up = Przesuń się.
move-down = Przesuń w dół
merge-with-previous = Łączenie z poprzednimi
move-to-trash = Przenieś do kosza
rename-chapter = Zmień nazwę rozdziału
add-scene = Dodaj nową scenę
add-chapter = Dodaj nowy rozdział
new-scene-title = Nowa scena
placeholder-scene-name = Nazwa sceny …
menu-cut = Cięcie
menu-copy = Kopiuj
menu-paste = Wklej
menu-paste-unformatted = Wklej bez formatowania
menu-select-all = Zaznacz wszystko
format-bold = Pogrubienie
format-italic = Kursywa
format-underline = Podkreślenie
format-strikethrough = Przekreślenie

binder = Segregator
no-work = Brak pracy załadowanej
no-recent-works = Brak najnowszych prac
switcher-open-section = Obecnie otwarte
switcher-recent-section = Najnowsze
switcher-this-window = to okno
open-project-title = Otwarty projekt
open-project-question = Jak chcesz otworzyć "{ $title }"?
open-in-new-window = Otwórz w nowym oknie
open-here = Otwórz tutaj.

binder-all = Wszystkie segregatory
binder-show-all = Pokaż wszystkie segregatory
binder-new = Nowy binder…
binder-default-name = Nowy segregator
binder-item-count = { $count ->
    [one] { $count } pozycja
    [few] { $count } pozycje
    [many] { $count } pozycji
   *[other] { $count } pozycji
}
binder-search-placeholder = Filtruj outline…
binder-search-scope = Przeszukaj wszystkie segregatory
binder-trash-confirm-title = Przenieść segregator do śmieci?
binder-trash-confirm-text = "{ $name }" i wszystkie jego przedmioty zostaną przeniesione do kosza.

dialog-rename = Zmień nazwę
dialog-new-binder = Nowy segregator
dialog-set-label = Ustaw etykietę
dialog-new-scene = Nowa scena
close-work-question = Zapisać zmiany przed zamknięciem pracy?
quit-question = Zapisać zmiany przed odejściem?
quit-save-work-question = Zapisać zmiany do { $title } przed odejściem?
unsaved-changes = Ta praca ma niezapisane zmiany.
new-work-unsaved-question = Zapisać zmiany przed utworzeniem nowej pracy?
open-work-unsaved-question = Zapisać zmiany przed otwarciem innej pracy?
switch-backup-discard-title = Wyrzucić zmiany w tym kopii zapasowej?
switch-backup-discard-text = Zmiany kopii zapasowej nie mogą być zapisywane. Użyj Zapisz jako lub Przywróć, aby je zachować, lub wyrzucić je i otworzyć inne prace.
switch-save-failed = Praca nie mogła zostać uratowana, więc nie została zastąpiona: { $error }
switch-save-not-started = Praca nie mogła zostać uratowana, więc nie została zastąpiona.
close-save-failed = Prace nie można było uratować, więc nie było zamknięte: { $error }
close-save-not-started = Praca nie mogła zostać uratowana, więc nie była zamknięta.
save-not-started = Praca nie mogła zostać uratowana.

could-not-open-work = Nie można otworzyć "{ $file }"
could-not-open-work-too-new = "{ $file }" potrzebuje nowszego Skribisto
could-not-open-work-too-new-detail = Oszczędzony przez Skribisto format { $written_by }; otwarcie go potrzebuje formatu { $requires } lub nowszego, a ten build obsługuje do formatu { $supported }. Aktualizacja Skribisto, aby go otworzyć.
could-not-open-work-importing = "{ $file }" jest zapisywany przez import
could-not-open-work-importing-detail = Import zapisuje ten projekt i nie został ukończony. Otwiera go po zakończeniu importu: otwiera się teraz, pokazuje, że import zastępuje, a jego następny zapis zapisuje ten projekt z powrotem nad importem.
could-not-open-example = Nie można otworzyć przykładu: { $error }
could-not-open-link = Nie można otworzyć { $url }: { $error }
could-not-create-work = Nie można utworzyć pracy: { $error }
saving-as-file = Zapisywanie jako { $target }…
saving-as-folder = Oszczędność jako { $target } / …
saved-as = Zapisane na { $target }
save-error = Nie można zapisać: { $error }
save-flush-failed = Zapis przeszedł, ale te dokumenty nie mogły być przekazane do niego i są nadal niezapisane: { $titles } ({ $error })
backup-error = Nie można utworzyć kopii zapasowej: { $error }
backing-up = Backing up …
backup-nothing-open = Żaden projekt nie jest otwarty na wsparcie.
backup-already-running = Wsparcie jest już w toku.
backup-complete = Backup kompletny ({ $ok } zapisane, { $skipped } już aktualne)
backup-partial = Kopia zapasowa zakończona: { $ok } zapisany, { $failed ->
    [one] { $failed } cel nie powiódł się
    [few] { $failed } miejsca przeznaczenia nie powiodły się
    [many] { $failed } miejsc przeznaczenia nie powiodło się
   *[other] { $failed } miejsc docelowych nie powiodło się
}
backup-no-destination-title = Brak lokalizacji kopii zapasowej
backup-no-destination-text = Nie można osiągnąć żadnego z skonfigurowanych celów tworzenia kopii zapasowych (na przykład można odłączyć zewnętrzny napęd). Podłącz go i spróbuj ponownie, lub wyjść bez tworzenia kopii zapasowych.
backup-failed-close-title = Nie można zapisać kopii zapasowej
backup-failed-close-text = Przed zamknięciem nie można było napisać kopii zapasowej. Każde miejsce docelowe nie powiodło się (dysk może zostać usunięty, albo może być w pełni zabezpieczony, lub może być zabezpieczony przed zapisem). Napraw go i spróbuj ponownie, lub wyjdź bez tworzenia kopii zapasowych.

backup-choice-title = Plik kopii zapasowej
backup-choice-heading = Otworzyłeś kopię zapasową.
backup-choice-subtitle = To jest kopia zapasowa projektu.
backup-choice-subtitle-dated = Wsparcie zajęte { $date }.
backup-choice-body = Możesz go otworzyć i swobodnie edytować, ale zmiany mogą być przechowywane tylko za pomocą Zapisz jako. Oryginalny plik projektu nie jest dotknięty. Albo przywróć ten projekt dokładnie do tego kopii zapasowej.
backup-choice-open = Otwórz kopię zapasową
backup-choice-restore = Przywrócenie projektu do tego punktu …
backup-choice-not-a-backup = Nie, otwórz normalnie.
backup-banner-title = Kopia kopii zapasowej: nie można tutaj zapisać zmian
backup-banner-description = Użyj Zapisz jako, aby zachować swoje edycje w nowym pliku, lub Przywróć, aby zastąpić oryginalny projekt tym backup.
backup-banner-restore = Przywróć…
backup-banner-save-as = Zapisz As…
backup-restore-original-missing = Nie można znaleźć oryginalnego projektu, aby przywrócić. Użyj Zapisz jako, aby zachować kopię zapasową jako nowy projekt zamiast.
backup-restore-close-elsewhere-title = Projekt otwarty w innym oknie
backup-restore-close-elsewhere-text = Projekt, który odrestaurujesz jest otwarty w innym oknie. Proszę zamknąć go tam najpierw, a następnie ponownie spróbować.
backup-restore-focus-window = Skup to okno.
backup-restore-confirm-title = Przywrócić to wsparcie?
backup-restore-confirm-text = Obecna wersja projektu zostanie skopiowana na bok jako kopia zapasowa, zanim zostanie zastąpiona tą wersją.
backup-restore-confirm-ok = Przywróć
backup-restore-error = Nie można przywrócić: { $error }
backup-restored-ok = Projekt przywrócony.
backup-restored-with-safety = Twoja poprzednia wersja została zachowana na { $path }.
close-backup-discard-title = Wyrzucić zmiany w tym kopii zapasowej?
close-backup-discard-text = Zmiany kopii zapasowej nie mogą być zapisywane do niego. Użyj Zapisz jako, aby je zachować, lub wyrzucić i zamknąć.
quit-backup-discard-title = Wyrzucić zmiany i zrezygnować?
quit-backup-discard-work-question = Wyrzucić zmiany na { $title } i zrezygnować?
quit-backup-discard-text = Zmiany kopii zapasowej nie mogą być zapisywane do niego. Użyj Zapisz jako, aby je zachować, lub wyrzucić i zrezygnować.
backup-nudge-text = Nie ma kopii zapasowych dla tego projektu.
backup-nudge-action = Ustaw backups…

menu-backups-list = &Backups…
backups-title = Kopie zapasowe
backups-loading = Wyszukiwanie kopii zapasowych…
backups-empty = Nie znaleziono jeszcze kopii zapasowych dla tego projektu.
backups-open = Otwórz
backups-reveal = Pokaż lokalizację
backups-delete = Usuń tę kopię zapasową
backups-delete-confirm-title = Skasować kopię zapasową?
backups-delete-confirm-text = "{ $name }" zostanie na stałe usunięte.
backups-delete-error = Nie można usunąć kopii zapasowej: { $error }
backups-refresh = Odśwież
backups-close = Zamknij

settings-page-backup = Domyślne kopie zapasowe
settings-page-work-backup = Kopie zapasowe
settings-backup-general-title = Domyślne ustawienia kopii zapasowej
settings-backup-work-title = Kopie zapasowe dla tego projektu
settings-backup-inherit = Użyj ogólnych ustawień kopii zapasowej
settings-backup-inheriting = Ten projekt wykorzystuje ogólne ustawienia kopii zapasowej.
settings-backup-none-hint = Nie są ustawione automatyczne kopie zapasowe (każdy spust jest wyłączony).
settings-backup-last = Ostatnia kopia zapasowa: { $date }
settings-backup-last-never = Nie ma jeszcze kopii zapasowych.
settings-backup-open-list = Otwórz kopie zapasowe list…
settings-backup-triggers = Kiedy się wycofać?
settings-backup-on-close = Backup przy zamykaniu projektu
settings-backup-on-open = Backup przy otwieraniu projektu
settings-backup-interval = Kopia zapasowa okresowo, co
settings-backup-destinations = Cele kopii zapasowej
settings-backup-default-location = Domyślna lokalizacja
settings-backup-usage-since = { $count ->
    [one] { $count } kopia zapasowa · { $size } · najstarszy { $oldest }
    [few] { $count } kopie zapasowe · { $size } · najstarszy { $oldest }
    [many] { $count } kopii zapasowych · { $size } · najstarszy { $oldest }
   *[other] { $count } kopie zapasowe · { $size } · najstarsze { $oldest }
}
settings-backup-usage = { $count ->
    [one] { $count } kopia zapasowa · { $size }
    [few] { $count } kopie zapasowe · { $size }
    [many] { $count } kopii zapasowych · { $size }
   *[other] { $count } kopii zapasowych · { $size }
}
settings-backup-usage-empty = Nic tu jeszcze nie zostało.
settings-backup-usage-measuring = Pomiar …
settings-backup-reveal-root = Pokaż folder
settings-backup-dest-none = Kopie zapasowe są zapisywane obok projektu.
settings-backup-dest-remove = Usuń
settings-backup-dest-add = Dodaj folder…
settings-backup-dest-refresh = Odśwież
settings-backup-retention = Jak wiele zatrzymać
settings-backup-retention-tiered = Warstwowy
settings-backup-retention-keep-n = Zachowaj ostatnie N
settings-backup-retention-tip = Jak stare kopie są przycinane.
settings-backup-retention-tip-more = Tiered utrzymuje jedną kopię zapasową na godzinę na dzień, na dzień na tydzień, na tydzień na miesiąc i na miesiąc poza, więc najnowsza historia pozostaje gęsta i stara historia nie. "Keep last N" po prostu zachowuje N najbardziej aktualne kopie. W obu trybach najnowsze kopie (minimum poniżej) są zawsze przechowywane.
settings-backup-gfs-hourly = Godzina (ostatnie 24 godziny)
settings-backup-gfs-daily = Codziennie (w zeszłym tygodniu)
settings-backup-gfs-weekly = Tygodniowy (ostatni miesiąc)
settings-backup-gfs-monthly = Miesięcznie
settings-backup-keep-n = Numer, który należy zachować
settings-backup-min-keep = Zawsze co najmniej
settings-backup-dedup = Pomiń kopię zapasową, gdy nic się nie zmieniło

new-work-title = Nowy projekt
new-work-close = Zamknij
new-work-name = Nazwa projektu
new-work-name-placeholder = Bez tytułu
new-work-author = Autor
new-work-author-placeholder = Opcjonalnie
new-work-format = Format
new-work-single-file = Pojedynczy plik
new-work-bundle = Folder projektu
new-work-convert-later = Możesz przekonwertować między formatami później.
new-work-location = Miejsce
new-work-will-create = Utworzy
new-work-language = Język domyślny
new-work-language-hint = Zastosowanie do nowych tekstów. Każdy tekst może być przestawiony na inny język.
new-work-template = Szablon
new-work-template-none = Brak
new-work-template-empty-novel = Pusta powieść
new-work-template-light-novel = Light Novel
new-work-template-novel = Powieść
new-work-template-novel-in-parts = Powieść w częściach
new-work-template-notebook = Notatnik
new-work-cancel = Anuluj
new-work-create = Utwórz projekt
new-work-step-details = Szczegóły
new-work-step-language = Struktura językowa
new-work-step-template = Szablon
new-work-back = Wstecz
new-work-next = Dalej
new-work-documents-title = Nowy projekt z dokumentów
new-work-step-import = Importuj
new-work-create-and-import = Utwórz import…
new-work-documents-next-title = Twoje dokumenty będą następne.
new-work-documents-next-body = Tworzenie projektu otwiera kreatora importu nad nim, gdzie wybrać dokumenty, przegląd struktury Skribisto czyta z nich i wybrać, gdzie wyląduje. Nic nie jest zapisane w projekcie, dopóki nie potwierdzisz go tam.
new-work-documents-no-template = Ten projekt zaczyna się pusty specjalnie: nie ma szablonu, więc importowane dokumenty są jedyną rzeczą w nim.
new-work-documents-chapter-scene-hint = Dotyczy rozdziałów, które tworzy import. Można je później zmienić w ustawieniach projektu.
new-work-single-file-desc = Jedno archiwum .skrib (zip). Przenośne, łatwe do tworzenia kopii zapasowych.
new-work-bundle-desc = Folder zawierający każdy element tekstowy. Przyjaźń do kontroli wersji.
new-work-template-none-count = pusty segregator
new-work-template-empty-novel-count = 1 rozdział
new-work-template-light-novel-count = 15 rozdziałów
new-work-template-novel-count = 20 rozdziałów
new-work-template-novel-in-parts-count = 3 części, 24 rozdziały
new-work-template-notebook-count = Notatki w formie bezpłatnej
new-work-chapter-scene = Rozdziały bez scen
new-work-chapter-scene-tip = Każdy rozdział jest pojedynczym elementem, do którego piszesz bezpośrednio, bez scen pod nim. Zostaw to dla klasycznego układu, w którym rozdział jest folderem: zapisujesz do niego także bezpośrednio, ale może również pomieścić sceny.
new-work-chapter-scene-tip-more = Piszesz do rozdziału tak czy inaczej. Jedyną różnicą jest to, czy może on * zawierać * sceny. Drzewo segregatora Skribisto jest tylko organizacyjne, więc oba układy kompilować do tej samej książki, można je mieszać swobodnie, i Promote konwertuje rozdział między tymi dwoma bez utraty słowa.
new-work-name-required = Wprowadź nazwę pracy
new-work-name-invalid = Ta nazwa nie ma żadnych przydatnych znaków
new-work-location-required = Wybierz lokalizację
new-work-location-missing = Ten folder nie istnieje
new-work-location-not-folder = Ta ścieżka nie jest katalogiem
new-work-location-readonly = Ten folder nie jest zapisywalny
new-work-target-exists = Projekt o tej nazwie jest już w tym folderze. Tworzenie pracy zapyta przed jego zastąpieniem
new-work-target-is-folder = Folder o tej nazwie jest już w tej lokalizacji. Wybierz inną nazwę
new-work-overwrite-title = Zastąpić istniejący projekt?
new-work-overwrite-text = "{ $name }" już istnieje w tym folderze. Zastąpić go nowym projektem?
new-work-target-open-title = Ten projekt jest otwarty
new-work-target-open-text = "{ $name }" jest otwarty w Skribisto. Tworzenie nowego projektu w jego miejsce zastąpi projekt, nad którym pracujesz, a następny zapis będzie pisać ten projekt z powrotem nad nowym. Wybierz inną nazwę lub folder, lub zamknąć projekt najpierw.
target-importing-title = Import pisze ten projekt
target-importing-text = Import jest nadal pisanie "{ $name }". Poczekaj, aż skończy, lub wybierz inną nazwę.
import-plume-busy-title = Import kolejnego projektu Plume Creator
import-plume-busy-text = Plume Creator projektów są importowane jeden na raz. Poczekajcie, aż import w toku zakończy się, lub anulować go, a następnie uruchomić ten ponownie.
import-manuskript-busy-title = Import kolejnego projektu Manuskript
import-manuskript-busy-text = Manuskript projektów są importowane jeden na raz. Poczekajcie, aż import w toku zakończy się, lub anulować go, a następnie uruchomić ten ponownie.
backup-restore-importing-text = Import nadal pisze "{ $name }", projekt ten backup zostanie przywrócony. Przywróć kopię zapasową po zakończeniu importu.

new-work-manuscript = Manuskrypt
new-work-notes = Uwagi
new-work-research = Materiały badawcze
new-work-notebook = Notatnik
new-work-scene = Scena
new-work-note = Notatka
new-work-front-matter = Materia czołowa
new-work-back-matter = Materia wsteczna
new-work-paratext = Struktura książki
new-work-paratext-none = Brak struktury
new-work-paratext-hint = Z przodu i z tyłu materia tradycja otwiera i zamyka książkę z. Możesz przenieść, zmienić nazwę lub usunąć którąkolwiek z nich później.
new-work-tags = Etykiety
new-work-tags-none = Brak znaczników
new-work-tags-hint = Paleta startowa do znakowania znaków, miejsc i reszty. Opcjonalnie, a każdy znacznik można zmienić nazwę, przefarbować lub usunąć później.
new-work-note-templates = Szablony notatek
new-work-note-templates-none = Brak szablonów
new-work-note-templates-hint = Kształt notatki biblijnej zaczyna się w. Opcjonalnie, a każdy szablon może być później edytowany lub usunięty.
new-work-characters = Znaki
new-work-places = Miejsca

import-plume-title = Import Plume Creator projektu
import-plume-close = Zamknij
import-plume-source = Projekt Plume
import-plume-source-hint = Wybierz plik .plume lub .plume_backup (jakakolwiek wersja Plume Creator).
import-plume-location = Folder docelowy
import-plume-name = Nazwa pliku
import-plume-name-placeholder = Nazwa projektu
import-plume-will-create = Utworzy
import-plume-trash-warning = Zmiażdżone / usunięte elementy nie są migrowane.
import-plume-cancel = Anuluj
import-plume-import = Importuj
import-plume-source-required = Wybierz plik projektu Plume
import-plume-source-missing = Ten plik nie istnieje
import-plume-source-not-file = Ta ścieżka nie jest plikiem
import-plume-location-required = Wybierz folder docelowy
import-plume-location-missing = Ten folder nie istnieje
import-plume-location-not-folder = Ta ścieżka nie jest katalogiem
import-plume-location-readonly = Ten folder nie jest zapisywalny
import-plume-name-required = Wprowadź nazwę pliku
import-plume-name-exists = Plik o tej nazwie istnieje już tutaj. Import potwierdzi nadpisanie
import-plume-overwrite-title = Zastąpić istniejący plik?
import-plume-overwrite-text = "{ $name }" już istnieje. Zastąpić go importowanym projektem?
import-plume-manuscript-binder = Manuskrypt
import-plume-story-bible-binder = Biblia Opowieści
import-plume-progress-title = Importowanie projektu Plume …
import-plume-cancel-import = Anuluj
import-plume-cancelled = Importuj anulowane
import-plume-done = Przywożone { $imported ->
    [one] { $imported } pozycja.
    [few] { $imported } pozycje.
    [many] { $imported } pozycji.
   *[other] { $imported } pozycji.
} { $skipped ->
    [one] { $skipped } zdemolowany przedmiot nie został wyemigrowany.
    [few] { $skipped } zniszczone przedmioty nie zostały wyemigrowane.
    [many] { $skipped } zniszczonych przedmiotów nie zostało wyemigrowanych.
   *[other] { $skipped } zniszczonych przedmiotów nie zostało wyemigrowanych.
}
import-plume-open-now = Otwórz teraz.
import-plume-warnings = { $count ->
    [one] { $count } rzecz nie może być importowane dokładnie
    [few] { $count } rzeczy nie mogły być importowane dokładnie
    [many] { $count } rzeczy nie mogły być importowane dokładnie
   *[other] { $count } rzeczy nie mogły być importowane dokładnie
}
import-plume-details = Szczegóły
import-plume-warnings-title = Ostrzeżenia dotyczące przywozu
import-plume-error-title = Nie można zaimportować projektu
import-plume-nested-too-deep = Projekt nie został przywieziony: jego część "{ $part }" jest zagnieżdżona ponad { $limit } poziomów głębokości. Nie Plume Creator projektu zbliża się do tego, i czytanie może rozbił Skribisto. Nic nie zostało napisane.
import-plume-declares-entities = Projekt nie był importowany: jego część "{ $part }" deklaruje podmioty XML, które nie Plume Creator projektu robi. Rozszerzenie ich może zabrać więcej pamięci niż komputer ma i awarii Skribisto. Nic nie zostało napisane.
import-plume-error-details = Szczegóły

import-manuskript-title = Import Manuskript projektu
import-manuskript-close = Zamknij
import-manuskript-source = Projekt Manuskript
import-manuskript-source-hint = Wybierz plik .msk lub folder projektu (jakakolwiek wersja Manuskript).
import-manuskript-source-file = Wybierz file…
import-manuskript-source-folder = Wybierz folder…
import-manuskript-location = Folder docelowy
import-manuskript-name = Nazwa pliku
import-manuskript-name-placeholder = Nazwa projektu
import-manuskript-will-create = Utworzy
import-manuskript-cancel = Anuluj
import-manuskript-import = Importuj
import-manuskript-source-required = Wybierz projekt Manuskript
import-manuskript-source-missing = Ten plik lub folder nie istnieje
import-manuskript-location-required = Wybierz folder docelowy
import-manuskript-location-missing = Ten folder nie istnieje
import-manuskript-location-not-folder = Ta ścieżka nie jest katalogiem
import-manuskript-location-readonly = Ten folder nie jest zapisywalny
import-manuskript-name-required = Wprowadź nazwę pliku
import-manuskript-name-exists = Plik o tej nazwie istnieje już tutaj. Import potwierdzi nadpisanie
import-manuskript-overwrite-title = Zastąpić istniejący plik?
import-manuskript-overwrite-text = "{ $name }" już istnieje. Zastąpić go importowanym projektem?
import-manuskript-manuscript-binder = Manuskrypt
import-manuskript-story-bible-binder = Biblia opowieści
import-manuskript-characters-group = Znaki
import-manuskript-world-group = Świat
import-manuskript-plots-group = Plotki
import-manuskript-project-info-note = Informacje o projekcie
import-manuskript-summary-note = Podsumowanie
import-manuskript-importance-minor = Mniejsze
import-manuskript-importance-secondary = Wtórne
import-manuskript-importance-main = Główne
import-manuskript-progress-title = Przywóz Manuskript projektów …
import-manuskript-cancel-import = Anuluj
import-manuskript-cancelled = Importuj anulowane
import-manuskript-done = { $imported ->
    [one] { $imported } produkt importowany.
    [few] { $imported } produkty importowane.
    [many] { $imported } sztuk importowanych.
   *[other] { $imported } pozycji importowanych.
} { $revisions ->
    [0] { "" }
    [one] { $revisions } wcześniejsza wersja była z nim.
    [few] { $revisions } wcześniejsze wersje były razem z nimi.
    [many] { $revisions } wcześniejszych wersji było razem z nimi.
   *[other] { $revisions } wcześniejsze wersje były z nimi.
}
import-manuskript-open-now = Otwórz teraz.
import-manuskript-warnings = { $count ->
    [one] { $count } rzecz wiedzieć o tym imporcie
    [few] { $count } rzeczy wiedzieć o tym imporcie
    [many] { $count } rzeczy wiedzieć o tym imporcie
   *[other] { $count } rzeczy wiedzieć o tym imporcie
}
import-manuskript-details = Szczegóły
import-manuskript-warnings-title = O tym imporcie
import-manuskript-error-title = Nie można zaimportować projektu
import-manuskript-nested-too-deep = Projekt nie został zaimportowany: jego plik "{ $part }" jest zagnieżdżony ponad { $limit } poziomów głębokości. Nie Manuskript projekt zbliża się do tego, i czytanie może się rozbijać Skribisto. Nic nie zostało napisane.
import-manuskript-folders-too-deep = Projekt nie został zaimportowany: jeden z jego plików siedzi ponad { $limit } folderów głębokich. Nie Manuskript projekt zbliża się do tego, i czytanie może się rozbijać Skribisto. Nic nie zostało napisane.
import-manuskript-declares-entities = Projekt nie został zaimportowany: jego plik "{ $part }" deklaruje podmioty XML, które nie Manuskript projekt robi. Rozszerzenie ich może zabrać więcej pamięci niż komputer ma i awarii Skribisto. Nic nie zostało napisane.
import-manuskript-error-details = Szczegóły

import-target-open-title = Ten projekt jest otwarty
import-target-open-text = "{ $name }" jest otwarty w Skribisto. Importowanie go zastąpiłoby projekt, nad którym pracujesz, a następny zapis zapisałby ten projekt z powrotem nad importem. Wybierz inną nazwę pliku, lub zamknij projekt najpierw.
import-warnings-announcement = { $result } { $notice }.

export-title = Eksportuj
export-close = Zamknij
export-scope-label = Co?
export-format-label = Format
export-style-label = Styl
export-path-label = Plik
export-browse = Przeglądaj…
export-show-non-exportable = Pokaż nie-exportable
export-choose-empty = Nie ma żadnego projektu do wyboru.
export-cancel = Anuluj
export-export = Eksportuj
export-save-dialog-title = Eksportuj do pliku
export-section-what = Co eksportować
export-section-style = Styl
export-section-destination = Miejsce przeznaczenia
export-custom-selection = Własny wybór
export-selected-count = Wybrane { $count }
export-chip-chapters-none = Rozdziały: brak
export-chip-chapters-numbered = Rozdziały: ponumerowane
export-chip-chapters-title = Rozdziały: tylko tytuł
export-chip-chapters-both = Rozdziały: liczba + tytuł
export-chip-scene-break-glyph = Przerwa w scenie: { $glyph }
export-chip-scene-break-blank = Przerwa w scenie: pusta linia
export-chip-scene-break-none = Przerwa w scenie: brak
export-chip-major-break-glyph = Poważna przerwa { $glyph }
export-chip-major-break-blank = Poważna przerwa: pusta linia
export-chip-major-break-none = Poważna przerwa: brak
export-chip-major-break-same = Oba poziomy się łamią.
export-chip-spacing-single = Interlinia: pojedyncza
export-chip-spacing-onehalf = Odstęp: 1 ½
export-chip-spacing-double = Odstęp: podwójny
export-chip-notes-included = Uwagi
export-chip-notes-excluded = Uwagi z wyłączeniem
export-format-docx = Słowo
export-format-odt = LibreOffice
export-format-html = HTML
export-format-markdown = Markdown
export-format-djot = Djot
export-format-text = Tekst
export-format-latex = LaTeX
export-format-epub = EPUB
export-format-pdf = PDF
export-overwrite-title = Zastąpić istniejący plik?
export-overwrite-text = "{ $name }" już istnieje.
export-progress-title = Eksportowanie …
export-cancelled = Eksportuj anulowane
export-done = Wywóz { $count ->
    [one] { $count } pozycja
    [few] { $count } pozycje
    [many] { $count } pozycji
   *[other] { $count } pozycji
}
export-open-file = Otwórz
export-show-in-folder = Pokaż w folderze
export-error-title = Nie można eksportować
export-error-details = Szczegóły

backup-progress-start = Uruchamianie…
backup-progress-retention = Sprzątanie starych kopii zapasowych …
backup-progress-done = Gotowe
backup-progress-destination = Miejsce przeznaczenia { $i } z { $n }
backup-details = Szczegóły
backup-issues-title = Zagadnienia dotyczące kopii zapasowej
backup-failed-title = Backup nie powiódł się
backup-complete-prune-warning = Backup kompletny ({ $ok } zapisane, { $skipped } już aktualne). Niektóre stare kopie zapasowe nie mogły być usunięte

search = Szukaj
search-query-placeholder = Szukaj…
search-replace-placeholder = Zastąp …
search-replace-toggle = Włączenie / wyłączenie wymiany
search-replace-all = Zastąp wszystko
search-opt-case = Przypadek dopasowania
search-opt-whole-word = Całe słowo
search-opt-diacritics = Akcent dopasowania
search-scope-body = Treść
search-scope-title = Tytuł
search-scope-synopsis = Streszczenie
search-scope-label = Adnotacja
search-facet-book = Książki
search-facet-part = Części
search-facet-chapter = Rozdziały
search-facet-scene = Sceny
search-facet-note = Uwagi
search-facet-folder = Foldery
search-tip-case = Przypadek meczu: traktować duże i małe skrzynie jako różne, więc "Elena" i "elena" są oddzielne zapałki.
search-tip-whole-word = Całe słowo: dopasować tylko kompletne słowa, więc "kot" nie znajduje się wewnątrz "kategorii".
search-tip-diacritics = Akcent dopasowania: traktować akcentowane litery jako odrębne, więc "kawiarnia" nie pasuje do "kawiarnia".
search-tip-body = Ciało: przeszukać prozę scen i notatek.
search-tip-title = Tytuł: przeszukać tytuły pozycji segregatorów.
search-tip-synopsis = Synopsis: wyszukaj streszczenie każdego elementu zapisu.
search-tip-label = Etykieta: wyszukaj krótką nutę wyświetlaną pod tytułem elementu segregatora.
search-tip-comment = Komentarze: przeszukać tekst wątków komentarza i ich odpowiedzi. Komentarz jest o rękopis, a nie jego część, więc ma swój własny przełącznik, a zastępstwo pozostawia komentarze dopasowania nieticked aż do ich tick.
search-tip-book = Książki: kontener książki i jej znaczniki początkowe / końcowe.
search-tip-part = Części: rozdzielacze częściowe.
search-tip-chapter = Rozdziały: rozdziały, w zależności od tego, w jaki sposób projekt je przechowuje.
search-tip-scene = Sceny: przedmioty, które trzymają prozę.
search-tip-note = Uwagi: Uwagi w formie bezpłatnej.
search-tip-folder = Foldery: proste organizowanie folderów i separatorów.
search-tip-paratext = Tekst, który należy do książki, ale nie do jej historii: przedmowa, dedykacja, słowo wtórne. Nigdy nie liczył się w rękopisach.
search-tip-preserve-case = Przypadek zachowawczy: zastępstwo bierze przypadek, który znalazł, więc "ELENA" staje się "MARTA", a "Elena" staje się "Marta".
search-tip-replace = Zastąp: pokaż pole zastępcze i zastąp wszystko.
search-error = Wyszukiwanie nie powiodło się: { $message }
search-no-matches = Brak dopasowań
search-count =
    { $matches ->
    [one] { $matches } mecz
    [few] { $matches } mecze
    [many] { $matches } meczów
   *[other] { $matches } meczów
    } w { $items ->
    [one] { $items } dokument
    [few] { $items } dokumenty
    [many] { $items } dokumentów
   *[other] { $items } dokumentów
    }
search-count-truncated =
    { $matches ->
    [one] { $matches } mecz
    [few] { $matches } mecze
    [many] { $matches } meczów
   *[other] { $matches } meczów
    } w { $items ->
    [one] { $items } dokument
    [few] { $items } dokumenty
    [many] { $items } dokumentów
   *[other] { $items } dokumentów
    } (tylko pierwsze wyniki)
search-occurrences = { $count }
search-field-body = Treść
search-field-title = Tytuł
search-field-synopsis = Streszczenie
search-field-label = Adnotacja
search-field-epigraph = Motto
search-field-comment = Komentarz
search-field-comment-reply = Odpowiedz
search-field-footnote = Przypis
search-include-in-replace = Zastąp wszystkie
search-collapse-all = Zwiń wszystkie wyniki
search-replace-here = Zastąp to
search-dismiss = Odstąpienie od wyników
search-undo-dismiss = Przywróć ostatni odrzucony wynik.
search-replace-nothing = (nic)
search-replace-confirm-title = Zastąpić wszystkie dopasowania?
search-replace-confirm-text =
    Zastąp { $occurrences ->
    [one] { $occurrences } wystąpienie
    [few] { $occurrences } Zdarzenia
    [many] { $occurrences } Zdarzenia
   *[other] { $occurrences } Zdarzenia
    } z "{ $query }" z "{ $replacement }" { $items ->
    [one] { $items } dokument
    [few] { $items } dokumenty
    [many] { $items } dokumentów
   *[other] { $items } dokumentów
    }?
search-replace-done-title = Zastąp całkowicie
search-replace-done =
    { $occurrences ->
    [one] { $occurrences } wystąpienie
    [few] { $occurrences } Zdarzenia
    [many] { $occurrences } Zdarzenia
   *[other] { $occurrences } Zdarzenia
    } zastąpiony { $items ->
    [one] { $items } dokument
    [few] { $items } dokumenty
    [many] { $items } dokumentów
   *[other] { $items } dokumentów
    }.
search-replace-done-skipped =
    { $occurrences ->
    [one] { $occurrences } wystąpienie
    [few] { $occurrences } Zdarzenia
    [many] { $occurrences } Zdarzenia
   *[other] { $occurrences } Zdarzenia
    } zastąpiony { $items ->
    [one] { $items } dokument
    [few] { $items } dokumenty
    [many] { $items } dokumentów
   *[other] { $items } dokumentów
    }. { $skipped ->
    [one] { $skipped } pole
    [few] { $skipped } pola
    [many] { $skipped } pól
   *[other] { $skipped } pól
    } pominięto (zmienił się od czasu wyszukiwania).
search-replace-undo = Cofnij
search-replace-failed-title = Zastąp nieudany
search-replace-skipped-title = Nic nie zastąpiło
search-replace-skipped-body = Teksty zmieniły się od czasu wyszukiwania. Liczba: { $fields }. Pozostawiono je bez zmian. Wyszukaj ponownie, aby zobaczyć aktualne wyniki.

search-replace-undo-failed-title = Cofnij nieudany
search-preview = Podgląd
search-preview-empty = Wybierz wynik do podglądu tutaj
search-preview-no-prose = Ten mecz nie ma edytowalnego tekstu
search-preview-prompt = Aby zobaczyć podgląd tutaj, uruchom poszukiwania.
search-preview-open-search = Szukaj w projekcie
search-preview-footnote-prompt = Ten mecz znajduje się w tekście przypisu. Otwórz go w doku Przypisy, aby zobaczyć i edytować go.
search-preview-open-footnotes = Otwarte przypisy

find-placeholder = Znajdź w tym dokumencie …
find-count = { $current } z { $total }
find-no-results = Brak wyników
find-close = Znajdź
find-previous = Poprzedni mecz (Shift + Enter)
find-next = Następny mecz (Enter)
find-opt-case = Przypadek dopasowania
find-opt-whole-word = Całe słowo
find-replace-toggle = Pokaż lub ukryj zastępowanie (Ctrl+R)
find-replace-placeholder = Zastąp …
find-replace = Zastąp
find-replace-all = Zastąp wszystko
find-preserve-case = Przypadek zachowania

dict-download-title = Pobieranie { $name }…
dict-download-done = Zainstalowane { $name }
dict-download-failed = Nie można pobrać { $name }: { $error }
dict-removed = Usunięty { $name }
dict-accept-first = Przyjęcie licencji przed pobraniem { $name }
settings-dict-tab-installed = Zainstalowane
settings-dict-tab-get-more = Pobierz więcej
settings-dict-tab-personal = Słowa osobiste
dict-installed-empty = Nie znaleziono jeszcze słowników na tym komputerze.
dict-get-more-search = Szukaj języków
dict-system-badge = w twoim systemie
dict-unusable-badge = nieprzydatny
dict-download-button = Pobieranie
dict-downloading = Pobieranie …
dict-installed-label = Zainstalowane
dict-remove = Usuń
dict-view-license = Widok licencji
dict-approx-size = ~{ $size }
dict-personal-empty = Nie ma jeszcze osobistych słów w tym projekcie.
dict-personal-add = Dodaj
dict-personal-placeholder = Dodaj słowo …
dict-missing-toast = { $count ->
    [one] Ten projekt używa { $count } słownika, którego nie zainstalowałeś
    [few] Ten projekt wykorzystuje { $count } słowniki, których nie zainstalowałeś
    [many] Ten projekt wykorzystuje { $count } słowników, których nie zainstalowałeś
   *[other] Ten projekt wykorzystuje słowniki { $count }, których nie zainstalowałeś
}
dict-missing-action = Pobierz słowniki
dict-license-title = { $name } licencja
dict-license-accept = Zaakceptuj pobranie
dict-license-cancel = Anuluj
dict-license-close = Zamknij
dict-add-button = Dodaj dictionary…
dict-add-title = Dodaj słownik
dict-add-close = Zamknij
dict-add-name = Nazwa
dict-add-name-placeholder = np. Mój słownik łaciński
dict-add-code = Kod języka
dict-add-code-placeholder = np. la lub fr-FR-x-custom
dict-add-code-hint = Krótki znacznik z wyboru. To jest to, co można wybrać jako język dokumentu.
dict-add-aff = Plik afiksów (.aff)
dict-add-dic = Lista słów (.dic)
dict-add-submit = Dodaj
dict-add-cancel = Anuluj
dict-add-name-required = Podaj słownikowi imię.
dict-add-code-required = Wprowadź kod języka.
dict-add-code-invalid = Używaj tylko liter, cyfr i - _.
dict-add-code-reserved = To jest wbudowany kod słownika. Wybierz inny.
dict-add-file-required = Wybierz plik.
dict-add-file-missing = Ten plik nie istnieje.
dict-add-code-taken = Słownik dla tego kodu jest już zainstalowany.
dict-add-done = Dodano { $name }
dict-add-unusable = Pliki te nie są słownikiem użytecznym: { $error }
dict-add-failed = Nie można dodać słownika: { $error }
inspector-export = Eksportuj
inspector-exportable = Uwzględnij w eksporcie
ctx-number = &Numer tego rozdziału
ctx-unnumber = &Nie numeruj tego rozdziału
inspector-numbering = Numeracja
inspector-numbered = Numeracja
inspector-numbered-tip = Ten rozdział zajmuje miejsce w numeracji książki. Wyłączcie go na prologu, epilogu lub interludium.
inspector-numbered-tip-more = Rozdział nienumerowany pozostaje w książce dokładnie tak, jak był: jego pozycja, jego proza i liczba słów są nietknięte. Po prostu nie drukuje żadnej liczby i nie używa się jej w górę: rozdział po prologu jest rozdziałem pierwszym, a nie rozdziałem drugim. Pozostawienie go poza eksportem jest innym przełącznikiem, powyżej, a ten usuwa cały rozdział z książki.
inspector-apply-to-children = Stosować u dzieci
inspector-milestone = Data milestonu
inspector-milestone-none = Brak daty
inspector-milestone-clear = Czysto.
inspector-tags = Etykiety
inspector-aliases = Inne nazwy
inspector-dict-language = Język
inspector-apply-language-to-children = Zastosuj język do dzieci
settings-page-language = Język
settings-field-dict-language = Języki
dict-tradeoff-hint = Każdy dodatkowy język akceptuje więcej słów, więc mniej błędów zostaje złapanych.
lang-inherit-hint = Dziedziczone: scena ta wykorzystuje języki książki lub projektu.
lang-pill-list = Języki
lang-pill-add = Dodaj język
lang-pill-remove = Usuń { $name }
lang-pill-mute = Wyłącz sprawdzanie pisowni na { $name }
lang-pill-unmute = Włącz sprawdzanie pisowni na { $name }

settings-page-personal-dictionary = Słownik osobisty
settings-user-dict-desc = Słowa dodawane tutaj są traktowane jak poprawnie napisane wszędzie w aplikacji, i nigdy więcej nie oznaczone.
settings-user-dict-add-placeholder = Dodaj słowo …
settings-user-dict-add = Dodaj słowo
settings-user-dict-search = Filtruj słowa
settings-user-dict-count = { $count ->
    [one] { $count } słowo
    [few] { $count } słowa
    [many] { $count } słów
   *[other] { $count } słowa
}

settings-user-dict-rename = Zmień nazwę
settings-user-dict-remove = Usuń
settings-user-dict-empty = Nie ma jeszcze słów. Dodaj słowa, które pisarz powinien zignorować lub zaimportować listę.
settings-user-dict-import = Znaczenie …
settings-user-dict-export = Eksport …
settings-user-dict-import-tip = Dodaj słowa z zwykłego pliku tekstowego, jedno słowo na linię. Istniejące słowa są przechowywane.
settings-user-dict-export-tip = Zapisz całą listę do zwykłego pliku tekstowego, jedno słowo na linię.
settings-user-dict-txt-filter = Pliki tekstowe
settings-user-dict-duplicate = Już w słowniku
settings-user-dict-imported = Przywożone { $count ->
    [one] { $count } słowo
    [few] { $count } słowa
    [many] { $count } słów
   *[other] { $count } słów
} ({ $duplicates } już obecne).
settings-user-dict-import-failed = Nie można odczytać listy słów: { $error }
settings-user-dict-exported = Uratowana { $count ->
    [one] { $count } słowo.
    [few] { $count } słowa.
    [many] { $count } słów.
   *[other] { $count } słów.
}
settings-user-dict-export-failed = Nie można zapisać listy słów: { $error }

settings-page-text-replacements = Wymiana tekstu
settings-text-repl-desc = Zastąp skrót pełnym tekstem: "btw" staje się "przy okazji" w momencie wpisywania miejsca lub interpunkcji.
settings-text-repl-enable = Użyj wymiany tekstu w tym projekcie
settings-text-repl-disabled-hint = Włącz to, aby zdefiniować skrót, który rozszerza się podczas pisania.
settings-text-repl-add = Dodaj regułę
settings-text-repl-trigger-placeholder = Skrót
settings-text-repl-replacement-placeholder = Czym się staje?
settings-text-repl-added = Dodano "{ $trigger }"
settings-text-repl-duplicate = "{ $trigger }" ma już zasadę
settings-text-repl-filter = Zasady filtrowania
settings-text-repl-count = { $n ->
    [one] { $n } zasada
    [few] { $n } zasady
    [many] { $n } zasad
   *[other] Zasady { $n }
}
settings-text-repl-row-enabled = Użyj tej zasady
settings-text-repl-delete = Skreślić zasadę dla { $trigger }
settings-text-repl-deleted = Skreślił zasadę dla "{ $trigger }"
settings-text-repl-empty = Nie ma jeszcze zasad.
settings-text-repl-csv-filter = Pliki CSV
settings-text-repl-import = Znaczenie …
settings-text-repl-export = Eksport …
settings-text-repl-imported = Przywożone { $added }, pominięte { $skipped }
settings-text-repl-exported = Wywóz { $n ->
    [one] { $n } zasada
    [few] { $n } zasady
    [many] { $n } zasad
   *[other] Zasady { $n }
}

editor-menu-no-suggestions = Brak sugestii
editor-menu-add-to-dictionary = Dodaj "{ $word }" do słownika
editor-menu-add-words-to-dictionary = Dodaj zaznaczone słowa do słownika
editor-dict-added = Dodano "{ $word }" do słownika.
editor-dict-added-multi = { $count ->
    [one] Dodano { $count } słowo do twojego słownika.
    [few] Dodano { $count } słowa do słownika.
    [many] Dodano { $count } słów do słownika.
   *[other] Dodano { $count } słów do słownika.
}
toast-undo = Cofnij

undo-superseded-title = Ten krok nie jest już ostatnim.
undo-superseded-body = Coś innego zmieniło się w tym projekcie od. Cofnij to w zamian, więc nie. Nic tego kroku nie zostało odwrócone. Użyj Edycja ▸ Cofnij się, aby przejść przez historię sam.
undo-failed = Cofnięcie nie powiodło się: { $error }

menu-edit = &Edycja
menu-edit-undo = C&ofnij
menu-edit-redo = &Ponów
menu-edit-undo-target = &Cofnij: { $target }
menu-edit-redo-target = &Ponów: { $target }
shortcut-name-edit-undo = Cofnij
shortcut-name-edit-redo = Ponów
undo-target-typing = maszynopisanie
undo-target-project = ostatnia zmiana w tym projekcie
prose-history-reset-title = Wpisywanie historii reset
prose-history-reset-body = { $count ->
    [one] Jedna otwarta scena została przywrócona z wcześniejszego stanu, więc jej historia pisania nie ma już zastosowania.
    [few] { $count } otwarte sceny zostały przywrócone z wcześniejszego stanu, więc ich historia pisania nie ma już zastosowania.
    [many] { $count } otwartych scen zostało przywróconych z wcześniejszego stanu, więc ich historia pisania nie ma już zastosowania.
   *[other] { $count } otwarte sceny zostały przywrócone z wcześniejszego stanu, więc ich historia pisania nie ma już zastosowania.
}
undo-target-trash = Przenosimy się do śmieci.
undo-target-restore = Przywracanie ze śmietnika
undo-target-delete-forever = usuwanie na dobre
undo-target-replace-all = zastępowanie całego projektu
undo-target-import = import dokumentu
undo-target-duplicate = duplikating
undo-target-move = przenoszenie
undo-target-merge = połączenie dwóch scen
undo-target-split = rozdzielenie sceny
undo-target-promote = zmiana typu
undo-target-tidy-titles = czyszczenie tytułów rozdziału
undo-target-import-tags = Etykiety importujące
undo-target-import-templates = Wzory formularzy notatek importowych
undo-target-create = tworzenie
undo-target-remove = Usuwanie
undo-target-rename = zmianę nazwy
undo-target-edit = który edytuje
menu-edit-find = &Znajdź…
menu-edit-find-next = Znaj&dź następny
menu-edit-find-prev = Z&najdź poprzedni
menu-edit-replace = Znajd&ź i zastąp …
menu-edit-find-in-project = Zna&jdź w projekcie …
menu-edit-replace-in-project = Za&stąp w projekcie …
undo-frozen = Cofnij ("Zawsze naprzód" jest włączone)
redo-frozen = Redo ("Always forward" jest włączony)
menu-edit-cut = &Cięcie
menu-edit-copy = &Kopiuj
menu-edit-paste = &Wklej
menu-edit-paste-plain = Wk&lej bez formatowania
menu-edit-select-all = Z&aznacz wszystkie
shortcut-name-edit-cut = Cięcie
shortcut-name-edit-copy = Kopiuj
shortcut-name-edit-paste = Wklej
shortcut-name-edit-paste-plain = Wklej bez formatowania
shortcut-name-edit-select-all = Zaznacz wszystkie

titlebar-spellcheck-on = Włącza się sprawdzanie pisowni. Kliknij, aby przerwać sprawdzanie (F7)
titlebar-spellcheck-off = Sprawdzanie pisowni jest wyłączone. Kliknij, aby sprawdzić ponownie (F7)
menu-spellcheck = &Sprawdź pisownię
menu-comments = &Komentarze
settings-page-spellcheck = Sprawdzanie pisowni
settings-group-spellcheck = Sprawdzanie pisowni
settings-spellcheck-enabled = Zaznacz literowanie jak piszę
settings-spellcheck-hint = Podkreśla słowa, których nie zna zainstalowany słownik. Wyłączenie tego zatrzymuje wszystkie sprawdzanie, w każdym projekcie, dopóki nie włączysz go z powrotem. Aby zaprzestać sprawdzania tylko jednego języka, wyczyść jego znak kontrolny na polu językowym pracy lub elementu.

menu-trash = &Kosz
trash-title = Kosz
trash-empty-state = Śmieci są puste.
trash-empty-button = Opróżnij kosz…
trash-restore = &Przywróć
trash-restore-to = &Przywrócić do …
trash-delete-forever = &Usuń na zawsze
trash-restored-ok = { $count } przywrócony.
trash-restore-error = Nie można przywrócić: { $error }
trash-restore-orphaned = Oryginalne miejsce tego elementu zniknęło. Wybierz gdzie go przywrócić.
trash-restore-no-project = Żaden projekt nie jest otwarty, więc nie ma nic do przywrócenia.
trash-delete-no-project = Żaden projekt nie jest otwarty, więc nie ma nic do usunięcia.
trash-empty-confirm-title = Opróżniać śmieci?
trash-empty-confirm-text = { $count ->
    [one] { $count } zdemolowany wpis zostanie trwale usunięty.
    [few] { $count } zdemolowane wpisy zostaną trwale usunięte.
    [many] { $count } zdemolowanych wpisów zostanie trwale usuniętych.
   *[other] { $count } zdemolowane wpisy zostaną trwale usunięte.
} Zostawiają śmieci na dobre.
trash-emptied-title = Śmietnik opróżniony
trash-emptied-body = Wszystko w śmieciach zostało na stałe usunięte.
trash-empty-no-project = Żaden projekt nie jest otwarty, więc nie ma śmieci do opróżnienia.
trash-delete-forever-confirm-title = Skasować na zawsze?
trash-delete-forever-confirm-text = { $count ->
    [one] { $count } pozycja zostanie na stałe usunięta.
    [few] { $count } pozycje zostaną na stałe usunięte.
    [many] { $count } pozycji zostanie na stałe usuniętych.
   *[other] { $count } pozycji zostaną trwale usunięte.
} Zostawiają śmieci na dobre.
trash-deleted-title = Usunięty na zawsze
trash-deleted-body = { $count } na stałe usunięte.
trash-undo = Cofnij
trash-restore-picker-title = Przywrócić do …
trash-restore-to-confirm-title = Przywrócić tutaj?
trash-restore-to-confirm-text = Przywrócić "{ $item }" do "{ $destination }"?
trash-restore-picker-restore-here = Przywróć tutaj
trash-restore-picker-cancel = Anuluj
trash-restore-picker-empty = Nie ma jeszcze segregatorów.
trash-banner-title = Ta pozycja znajduje się w koszu
trash-banner-description = Nie pojawi się w obrysie ani w eksporcie, dopóki go nie przywrócisz.
trash-banner-restore = Przywróć…
trash-tab-tooltip = W śmieciach

settings-styles-sheet-yes = Tak.
settings-styles-sheet-no = Nie.
settings-styles-sheet-font = Czcionka
settings-styles-sheet-indent = Wcięcie na pierwszej linii
settings-styles-sheet-para-spacing = Odstęp między punktami
settings-styles-sheet-page = Rozmiar strony
settings-styles-sheet-margins = Marginesy (góra/prawo/dół/lewo)
settings-styles-sheet-title-page = Strona tytułowa książki
settings-styles-sheet-heading-language = Język kierunkowy
settings-styles-sheet-auto = Zgodnie z tekstem
settings-styles-sheet-digits = Numery
settings-styles-sheet-direction = Kierunek tekstu
settings-styles-sheet-formats = Formaty
settings-styles-sheet-all-formats = Wszystkie

settings-styles-editor-group = Edytor stylu
settings-styles-editor-missing = Ten styl jest już niedostępny.

settings-styles-page-letter = List
settings-styles-digits-western = Zachód (0- 9)
settings-styles-digits-eastern-arabic = Wschodni Arabski (Xion- Xion-)
settings-styles-direction-ltr = Od lewej do prawej
settings-styles-direction-rtl = Od prawej do lewej

format-dock-title = Format
format-panel-empty = Umieść kursor w scenie, notatki lub synopsis, aby zobaczyć opcje formatowania.
format-group-marks = Tekst
format-group-block = Paragraf
format-group-lists = Listy
format-group-tables = Tabela
format-group-breaks = Przerwa w scenie
format-superscript = Indeks górny
format-subscript = Indeks dolny
format-link = Łącze…
link-dialog-insert-title = Wstaw link
link-dialog-edit-title = Edytuj odnośnik
link-dialog-text-label = Tekst
link-dialog-text-placeholder = słowa, które czytelnik widzi
link-dialog-url-label = Linki do
link-dialog-url-placeholder = example.com
link-dialog-insert = Wstaw
link-dialog-apply = Zastosuj
link-dialog-cancel = Anuluj
link-dialog-remove = Usuń odnośnik
link-scheme-refused = Tylko linki internetowe i e-mail mogą być otwarte. { $url } pozostawiono w spokoju.
menu-format-link = &Łącze…
format-clear = Wyczyść formatowanie
format-heading = Poziom nagłówka
format-heading-normal = Normalny tekst
format-heading-1 = Nagłówek 1
format-heading-2 = Nagłówek 2
format-heading-3 = Nagłówek 3
format-heading-4 = Nagłówek 4
format-heading-5 = Nagłówek 5
format-heading-6 = Nagłówek 6
format-align-left = Wyrównaj w lewo
format-align-center = Wyśrodkuj
format-direction-rtl = Paragraf od prawej do lewej
format-blockquote = Cytat blokowy
format-list-bullet = Lista punktowana
format-list-numbered = Lista numeryczna
format-indent = Zwiększ wcięcie
format-outdent = Zmniejsz wcięcie
format-table-insert = Wstaw tabelę
format-table-row-above = Wstaw wiersz powyżej
format-table-row-below = Wstaw wiersz poniżej
format-table-col-before = Wstaw kolumnę przed
format-table-col-after = Wstaw kolumnę po
format-table-row-delete = Usuń wiersz
format-table-col-delete = Usuń kolumnę
format-table-remove = Usuń tabelę

menu-format-marks-bold = &Pogrubienie
menu-format-marks-italic = K&ursywa
menu-format-marks-underline = P&odkreślenie
menu-format-marks-strike = P&rzekreślenie
menu-format-marks-superscript = Ind&eks górny
menu-format-marks-subscript = In&deks dolny
menu-format-marks-clear = W&yczyść formatowanie
menu-format-heading = &Nagłówek
menu-format-heading-normal = N&ormalny tekst
menu-format-heading-1 = &Nagłówek 1
menu-format-heading-2 = N&agłówek 2
menu-format-heading-3 = Na&główek 3
menu-format-heading-4 = Nag&łówek 4
menu-format-heading-5 = Nagł&ówek 5
menu-format-heading-6 = Nagłó&wek 6
menu-format-alignment = Do&stosowanie
menu-format-align-left = W&yrównaj w lewo
menu-format-align-center = &Wyśrodkuj
menu-format-direction = &Kierunek
menu-format-direction-auto = &Automatyczne
menu-format-direction-ltr = &Od lewej do prawej
menu-format-direction-rtl = &Od prawej do lewej
menu-format-blockquote = &Cytat blokowy
menu-format-lists = L&isty
menu-format-list-bullet = &Lista punktowana
menu-format-list-numbered = L&ista numeryczna
menu-format-indent = &Zwiększ wcięcie
menu-format-outdent = Z&mniejsz wcięcie
menu-format-table = &Tabela
menu-format-table-insert = &Wstaw tabelę
menu-format-table-2x2 = &2 x 2
menu-format-table-3x3 = &3 x 3
menu-format-table-4x4 = &4 x 4
menu-format-table-row-above = Wst&aw wiersz powyżej
menu-format-table-row-below = Wstaw w&iersz poniżej
menu-format-table-col-before = Wstaw &kolumnę przed
menu-format-table-col-after = Ws&taw kolumnę po
menu-format-table-row-delete = &Usuń wiersz
menu-format-table-col-delete = U&suń kolumnę
menu-format-table-remove = Usu&ń tabelę

menu-format-typo-marks = &Formatowanie znaku
menu-format-typo-nbsp = &No- break Space
menu-format-typo-nbhyphen = &Nieprzerwana Hyphen
menu-format-typo-soft-hyphen = &Łącznik opcjonalny
menu-format-typo-nnbsp = &Narrow No- break Space
menu-format-typo-zwsp = &Szerokość zero- przestrzeń
menu-format-typo-word-joiner = &Łącznik wyrazów


menu-about = &O Skribisto…
about-title = O Skribisto
about-version = Wersja { $version }
about-update-available = Wersja { $version } jest dostępna
about-update-see-notes = Zobacz, co się zmieniło.
about-update-flathub = Flathub uaktualnia tę kopię.
about-update-distro = Twoja dystrybucja uaktualnia tę kopię.
about-tagline = Novel- writing app for long-form fiction, written in Rust with the Teksilo toolkit.
about-license = Wydany na podstawie Powszechnej Licencji Publicznej GNU, wersja 3.
about-copyright = © 2026 Cyril Jacquet
about-close = Zamknij

native-menu-about = O { $app }
native-menu-hide = Ukryj { $app }
native-menu-quit = Zakończ { $app }
native-menu-settings = Ustawienie…
native-menu-window = Okno
native-menu-minimize = Minimalizuj
native-menu-zoom = Powiększ

window-title = { $title } - { $app }
window-title-numbered = { $title } - { $app } (Okno { $n })
window-title-empty = { $app }

first-run-window-title = Powitanie
first-run-title = Ustawcie { $app }
first-run-body = { $app } utrzymuje ustawienia oddzielnie od Skribisto, więc zaczyna się pusty. Twoje preferencje, niedawne projekty, słowniki i układ okien mogą być kopiowane teraz.
first-run-from = Kopiuj
first-run-to = Kopiuj
first-run-copy-note = Nic nie jest przesuwane lub usuwane. Skribisto utrzymuje każdy z jego własnych ustawień, i kontynuuje pracę dokładnie jak wcześniej.
first-run-import = Importuj ustawienia
first-run-start-fresh = Zacznij od nowa.
first-run-import-failed = Niektóre ustawienia nie mogły być importowane: { $error }

settings-page-punctuation = Domyślne interpunkcje
settings-group-punctuation = Inteligentna interpunkcja
settings-page-work-punctuation = Interpunkcja
settings-punctuation-override = Dać temu projektowi własne zasady interpunkcji
settings-punctuation-override-hint = Wyłączony: projekt jest zgodny z preferencją aplikacji. Zasady te poruszają się wewnątrz .skrib, więc współautor otwierający plik pisze tą samą typografią.
settings-punctuation-dashes = Zamień -- w en dash, --- w em dash
settings-punctuation-ellipsis = Zamień... w elipsę.
settings-punctuation-quotes = Znaki cytowania i apostrofy
settings-quote-style = Znaki notowania
settings-quote-style-locale = Domyślny język
settings-quote-style-curly = "Podwójne"
settings-quote-style-curly-single = "Single"
settings-quote-style-guillemets = "Guillemets"
settings-quote-style-low-high = "Niski poziom"
settings-punctuation-spacing = Przestrzeń przed;:!?
settings-punctuation-spacing-hint = Francuska typografia ustawia wąską przestrzeń bez przerwy przed;!? i pełną przed:. Dotyczy tylko tekstu napisanego w języku francuskim. Przestrzeń wewnątrz gillemetali "" pochodzi z samych cytaty gillemet.
settings-punctuation-sample = Twój język daje
settings-punctuation-app-hint = Co każdy projekt robi, chyba że przyjmuje własne zasady w Projekcie ▸ Punctuation.
settings-punctuation-dialogue = Otwórz akapit wpisany jako "-" ze znacznikiem dialogowym
settings-punctuation-dialogue-hint = W przypadku języków oznaczających mowę za pomocą myślnika zamiast znaków notowania: francuski, hiszpański, rosyjski i inne.

statusbar-go-to = Idź do …
go-to-placeholder = Przeszukaj segregator
go-to-no-matches = Żaden element nie pasuje do tego wyszukiwania.
menu-go-to = &Idź do …

settings-page-distraction-free-themes = Tematy wolne od dystrakcji
settings-themes-builtin = Built- in themes
settings-themes-builtin-badge = Budowa - w
settings-themes-user = Moje motywy
settings-themes-editor-group = Edytuj motyw
settings-themes-editor-empty = Wybierz motyw pod "Moje motywy", aby go edytować.
settings-themes-use = Stosowanie
settings-themes-duplicate = Duplikuj
settings-themes-edit = Edycja
settings-themes-delete = Usuń
settings-themes-export = Eksport …
settings-themes-import = Znaczenie …
settings-themes-copy-suffix = kopia
settings-themes-json-filter = Temat (JSON)
settings-themes-imported = Zaimportowany motyw
settings-themes-import-failed = Nie można zaimportować tego motywu
settings-themes-exported = Wywóz motywu
settings-themes-export-failed = Nie można wyeksportować tego tematu
settings-themes-low-contrast = niski kontrast
settings-themes-low-contrast-band = podświetlenie ukrywa tekst
settings-themes-field-name = Nazwa
settings-themes-field-paper = Strona
settings-themes-field-ink = Tekst
settings-themes-field-general = Kontekst
settings-themes-field-widget-text = Tekst paska kontrolnego
settings-themes-field-caret-band = Zaznacz wokół opiekuna

statusbar-focus-settings = Ustawienia wolne od dystrakcji
statusbar-focus-manage-themes = Zarządzaj nimi …

comments-title = Komentarze
comments-document-title = Niniejszy dokument
comments-empty-project = Brak komentarzy w tym projekcie.
comments-empty-document = Brak komentarzy do tego dokumentu.
comments-filter-all = Wszystkie
comments-filter-open = Otwórz
comments-filter-resolved = Rozwiązane
comments-filter-orphaned = Bez powiązanego tekstu
comments-status-open = Otwórz
comments-status-resolved = Rozwiązane
comments-status-orphaned = Utracono powiązany tekst
comments-orphan-snippet = (skomentowany tekst zniknął)
comments-status-unplaced = Brak pozycji tekstowej
comments-unplaced-snippet = (niezakotwiczone w żadnym tekście)
comments-reply-count = { $count ->
    [one] { $count } odpowiedź
    [few] { $count } odpowiedzi
    [many] { $count } odpowiedzi
   *[other] { $count } odpowiedzi
}
comments-menu-resolve = Rozwiąż
comments-menu-reopen = Ponowne otwarcie
comments-menu-delete = Usuń
comments-sort-document = W kolejności dokumentów
comments-sort-newest = Od najnowszych
comments-sort-reader = Grupowany przez czytelnika
comments-unsigned-author = Niepodpisane
comments-agreement = { $count ->
    [one] { $count } czytnik
    [few] { $count } czytniki
    [many] { $count } czytników
   *[other] { $count } czytelników
}
comments-menu-add = Dodaj komentarz
comments-menu-add-paragraph = Komentarz do niniejszego ustępu
overview-col-comments = Komentarze
overview-col-total-comments = Uwagi ogółem
comments-card-placeholder = Napisz komentarz …
comments-card-unknown-author = Nieznany autor
comments-unsigned-toast = Twoje komentarze są niepodpisane: na tym komputerze nie ma dla ciebie żadnej nazwy.
comments-unsigned-action = Ustaw swoje imię
comments-card-reply = Odpowiedz
comments-card-reply-placeholder = Repli…
comments-card-actions = Działania komentatorskie
comments-reply-actions = Odpowiedzi na skargi
comments-menu-delete-reply = Usuń odpowiedź
comments-menu-delete-all = Usuń wszystkie komentarze tutaj
comments-deleted-toast = Komentarz usunięty
comments-reply-deleted-toast = Odpowiedź usunięta
comments-deleted-all-toast = { $count ->
    [one] { $count } komentarz usunięty
    [few] { $count } uwagi skreślone
    [many] { $count } komentarzy skreślonych
   *[other] Skreślone uwagi { $count }
}
comments-undo = Cofnij

analysis-segment = Analiza
analysis-scope-book = Analizując tę książkę
analysis-run = Przeprowadź analizę
analysis-stale = Zmienił się, odkąd to się zaczęło.
analysis-not-run = Jeszcze nie zanalizowaliśmy.
analysis-running = Czytanie rękopisów …
analysis-failed = Analiza nie mogła się zakończyć.
analysis-no-scenes = Jeszcze nie ma scen w tej książce.

analysis-shape = Kształt

analysis-arrivals = Przyloty
analysis-arrivals-explainer = W jaki sposób tekst dotarł do tego projektu, kiedy był otwarty. Pisze, którą trasę zeszli postacie i w ogóle nic o tym, kto je napisał: pisarz, który rysuje gdzie indziej i pasty się klei, a ten, kto dyktuje, dyktował. Nie ma tu numeru, do którego można dążyć.
analysis-arrivals-scope = Ten cały projekt, nie tylko tę książkę.
analysis-arrivals-session = Zamknięcie projektu zaczyna się od nowa.
analysis-arrivals-nothing = Na tej sesji nie ma jeszcze tekstu.
analysis-arrivals-typed = Wpisywane
analysis-arrivals-pasted = Wklejone
analysis-arrivals-dictated = Dyktat
analysis-arrivals-imported = Przywożone
analysis-arrivals-programmatic = Wstawione dla ciebie
analysis-arrivals-count = { $count ->
    [one] { $count } znak
    [few] { $count } znaki
    [many] { $count } znaków
   *[other] { $count } znaków
}
analysis-arrivals-none = brak

analysis-words-per-scene = Słowa na scenę
analysis-median-words = Ta książka mediana sceny działa { $count ->
    [one] { $count } słowo.
    [few] { $count } słowa.
    [many] { $count } słów.
   *[other] { $count } słów.
}
analysis-median-line = Mediana: { $count ->
    [one] { $count } słowo
    [few] { $count } słowa
    [many] { $count } słów
   *[other] { $count } słów
}
analysis-dialogue = Dialog
analysis-dialogue-unsupported = Dialog nie jest jeszcze mierzony dla tego języka.

analysis-footnote-words = Słowa przypisywane
analysis-footnote-words-count = { $count ->
    [one] { $count } słowo jest w przypisach tej książki, trzymane z dala od rękopisu.
    [few] { $count } słowa są w przypisach tej książki, trzymane z dala od rękopisu.
    [many] { $count } słów jest w przypisach tej książki, trzymanych z dala od rękopisu.
   *[other] { $count } słów są w przypisach tej książki, przechowywane z dala od rękopisu całości.
}
analysis-footnote-words-pending = Liczenie punktów …

analysis-ignore-empty = Ignoruj teksty z niczym jeszcze napisanym
analysis-empty-hidden = { $count } pusty { $count ->
    [one] tekst
    [few] teksty
    [many] teksty
   *[other] teksty
    } ukryte.
analysis-all-texts-empty = Każdy tekst w tej książce jest nadal pusty.

binder-filter-count = Wykazano { $shown } z { $total }
binder-filter-clear = Czysto.
binder-filter-none = Nic nie pasuje do { $query }.

settings-paratext-intro = Materia frontowa i tylna nowy projekt może rozpocząć się od. Każda struktura należy do tradycji wydawniczej, a jej tytuły stron są napisane w języku tej tradycji. Zmień je swobodnie po utworzeniu projektu.
settings-paratext-structures = Konstrukcje
settings-paratext-broken = Nie można odczytać
settings-paratext-edit = Edycja
settings-paratext-duplicate = Duplikuj
settings-paratext-delete = Usuń
settings-paratext-new = Nowa struktura
settings-paratext-save = Zapisz
settings-paratext-editor-hint = Nazwa, strony do tworzenia przed rękopisem i strony do tworzenia po nim. Tam, gdzie kończy się jest twój: są to zwykłe elementy, gdy projekt istnieje.


image-insert = W&staw obrazek …
image-no-project = Otwórz projekt przed włożeniem obrazu.
image-choose-title = Wybierz obrazek
image-filter-label = Obrazy
image-large-title = To duży obraz.
image-large-text =
    { $name } to { $megapixels } megapikseli ({ $width } × { $height }).
    Zachowanie go tak, jak jest przechowuje oryginalny plik w projekcie: podróżuje
    z każdej kopii zapasowej i każdego eksportu. Optymalizacja sklepów mniejszy egzemplarz
    zamiast tego, do 2560 pikseli po jego najdłuższej stronie.
image-large-keep = Zachowaj oryginał
image-large-downscale = Optymalizuj
image-large-remember = Zrób to od teraz, nie pytaj ponownie.
image-not-recorded = Obraz został zapisany, ale nie mógł być zapisany w projekcie.
image-describe-title = Opisz obraz
image-describe-explain = Co pokazuje obraz, dla czytelnika, który go nie widzi. Nie jest częścią manuskryptu: nigdy nie jest liczony, przeszukiwany lub eksportowany jako proza.
image-describe-placeholder = latarnia morska przeciw szaremu niebu
image-resize-title = Zmień rozmiar obrazu
image-resize-explain = Procent rozmiaru jest pokazany na teraz. 100 pozostawia to tak jak jest.
image-resize-invalid = Wprowadź liczbę od 1 do 1000.
image-menu-describe = &Opisz obrazek …
image-menu-resize = &Zmień rozmiar obrazka …
image-menu-reset-size = &Oryginalny rozmiar

cover-choose = &Książka cover…
cover-clear = U&suń pokrywę
cover-choose-title = Wybierz okładkę
cover-set = Okładka ustawiona.
cover-cleared = Okładka została usunięta. Obraz jest nadal w projekcie.
card-label = Adnotacja
card-exportable = Wywóz
card-numbered = Numeracja
card-created = Stworzony
card-modified = Zmodyfikowane
card-yes = tak
card-no = nie
card-type = Rodzaj
card-position = Pozycja
card-children = Dzieci
card-goal = Cel
card-words = Słowa
card-synopsis = Streszczenie
card-point-of-view = Punkt widzenia
card-aliases = Inne nazwy

export-orphan-footnotes-title = Przypisy bez odniesienia
export-comments-dropped =
    { $count ->
    [one] Jeden komentarz nie mógł być umieszczony w eksportowanym tekście i został pominięty.
    [few] { $count } komentarze nie mogły zostać umieszczone w tekście eksportowanym i zostały pominięte.
    [many] { $count } uwag nie można było umieścić w tekście eksportowanym i zostały pominięte.
   *[other] Uwagi { $count } nie mogły zostać umieszczone w tekście eksportowanym i zostały pominięte.
    }
export-orphan-footnotes =
    { $count ->
    [one] W manuskrypcie nie ma już wzmianki o jednym przypisie. Jego tekst nie pojawi się w książce eksportowej.
    [few] { $count } przypisy nie są już nigdzie wymienione w manuskrypcie. Ich tekst nie pojawi się w eksportowanej książce.
    [many] { $count } przypisach nie jest już nigdzie w rękopisach. Ich tekst nie pojawi się w książce eksportowej.
   *[other] { $count } przypisy nie są już nigdzie wymienione w manuskrypcie. Ich tekst nie pojawi się w eksportowanej książce.
    }


menu-footnotes = &Przypisy
footnotes-title = Przypisy
footnotes-insert = &Dodać przypis
footnotes-empty = Brak przypisu. Umieść kursor w scenie, a następnie użyć + powyżej, lub Ctrl + Alt + F.
footnotes-filter-all = Wszystkie
footnotes-filter-document = Niniejszy dokument
footnotes-filter-orphaned = Bez powiązanego tekstu
footnotes-orphaned = Nic więcej nie wskazuje na tę uwagę.
footnotes-untitled-home = Bez tytułu
footnotes-body-placeholder = sama nota
footnotes-insert-tooltip = Dodać przypis do kursora (Ctrl + Alt + F)
footnotes-actions = Działania na podstawie przypisu
footnotes-delete = &Skreślić notatkę i jej odniesienie
footnotes-no-project = Otwarcie projektu przed umieszczeniem przypisu.
footnotes-no-caret = Umieść kursor w tekście sceny, aby umieścić tam przypis.
footnotes-not-created = Przypis ten nie mógł zostać dodany do projektu.
footnotes-deleted-toast = Uwaga skreślona
footnotes-undo-delete = Cofnij

import-document-title = Dokumenty importowe
import-document-close = Zamknij
import-document-step-files = Pliki
import-document-step-review = Przegląd
import-document-step-destination = Miejsce przeznaczenia
import-document-step-reconcile = Scal
import-document-reconcile-hint = Niektóre z tych przedmiotów są już masz. Powiedz, co powinno się stać z każdym.
import-document-reconcile-all-new = Nic w tym pliku nie pasuje do Twojego projektu: każdy element zostanie dodany jako nowy.
import-document-duplicate-returns = Pliki te są więcej niż jedną kopią tego samego manuskryptu wracającego. Importuj je pojedynczo: dopasowując tylko jedną kopię zwrotną do twojego projektu, więc import kilku razem dodałby drugą kopię książki zamiast jej łączenia.
import-document-hunk-added = Weź ten nowy ustęp
import-document-hunk-removed = Wycofaj ten ustęp
import-document-hunk-changed = Weź to przepisanie
import-document-col-stray-prose = Jego tekst
import-document-stray-as-paratext = Przechowywać jako paratext
import-document-stray-discard = Rzuć tekst
import-document-col-current = W projekcie
import-document-col-incoming = W tym pliku
import-document-col-status = Stan
import-document-col-action = Co robić
import-document-status-identical = Bez zmian
import-document-status-editor-edited = Zmieniono u czytelnika
import-document-status-you-edited = Zmieniono u Ciebie
import-document-status-conflict = Zmieniono po obu stronach
import-document-status-different = Różni się
import-document-status-new = Nowy
import-document-status-missing = Brak w tym pliku
import-document-status-moved = Przeniesiono
import-document-action-comments-only = Tylko komentarze
import-document-action-take-import = Użyj tej wersji
import-document-action-keep-current = Zachowaj moją
import-document-action-create-new = Dodaj jako nowy
import-document-action-ignore = Pomiń
import-document-compare = Porównaj
import-document-compare-legend = Twoja wersja w porównaniu z tym plikiem. Read - tylko.
import-document-compare-close = Zamknij
import-document-drop-title = Rzuć tu dokumenty.
import-document-drop-hint = Markdown (.md), Word (.docx), OpenDocument (.odt) i zwykły tekst (.txt)
import-document-browse = Przeglądaj…
import-document-move-up = Przesuń się.
import-document-move-down = Przesuń w dół
import-document-remove-file = Usuń
import-document-file-count = { $count ->
    [one] { $count } plik
    [few] { $count } pliki
    [many] { $count } plików
   *[other] Pliki { $count }
}
import-document-no-files = Nie wybrano jeszcze żadnych plików.
import-document-col-included = Importuj
import-document-col-title = Tytuł
import-document-col-type = Rodzaj
import-document-col-words = Słowa
import-document-col-breaks = Podziały
import-document-col-comments = Komentarze
import-document-col-footnotes = Przypisy
import-document-col-epigraph = Motto
import-document-col-source = Źródło
import-document-level-rules = Poziomy pozycji
import-document-level-n = Pozycja { $level }
import-document-add-top-level = Dodaj górny poziom
import-document-add-top-level-tooltip = Umieść książkę nad każdym analizowanym elementem, dla plików rozdziałów, które nie mają nagłówka książki
import-document-destination = Miejsce przeznaczenia
import-document-destination-hint = Wybierz segregator lub element: nowe elementy lądują wewnątrz folderu lub po scenie.
import-document-destination-empty = Nie ma jeszcze segregatorów.
import-document-plan-empty = Nie ma jeszcze czego importować.
import-document-summary = { $rows ->
    [one] { $rows } pozycja
    [few] { $rows } pozycje
    [many] { $rows } pozycji
   *[other] { $rows } pozycji
} · { $breaks ->
    [one] { $breaks } scena przerwy
    [few] { $breaks } przerwy sceniczne
    [many] { $breaks } przerw scenicznych
   *[other] { $breaks } przerw scenicznych
}
import-document-back = Wstecz
import-document-cancel = Anuluj
import-document-analyse = Dalej
import-document-analysing = Czytanie dokumentów …
import-document-step-analysing = Czytanie
import-document-cancel-analysis = Przestań czytać.
import-document-analyse-failed = Dokumenty nie mogły zostać przeczytane.
import-document-details = Szczegóły
import-document-import = Importuj
import-document-done = { $count ->
    [one] { $count } pozycja przywieziona
    [few] { $count } Przywożone produkty
    [many] { $count } sztuk przywożonych
   *[other] { $count } pozycji przywożonych
}
import-document-undo = Cofnij
import-diagnostic-file-unreadable = Nie można odczytać "{ $path }": { $detail }. Pozostałe pliki nadal importowane.
import-diagnostic-nested-too-deep = "{ $path }" nie został przeczytany: jego zawartość jest zagnieżdżona więcej niż { $limit } poziomów głębokości. Żaden prawdziwy dokument nie zbliża się do tego, i czytanie może rozbił Skribisto, więc jest pomijany. Inne pliki nadal importowane.
import-diagnostic-archive-too-large = "{ $path }" nie został przeczytany: jest to skompresowany plik zbudowany do rozpakowania znacznie większy niż jest, który może spakować pamięć i awarii Skribisto, więc jest pominięty. Inne pliki nadal importowane.
import-diagnostic-lossy-decode = { $count ->
    [one] Jeden znak w "{ $path }" nie dekoduje. Ponownie zapisać plik jako UTF-8, aby go zatrzymać.
    [few] { $count } znaki w "{ $path }" nie dekodowali. Ponownie zapisać plik jako UTF-8, aby je zatrzymać.
    [many] { $count } znaków w "{ $path }" nie dekoduje. Ponownie zapisać plik jako UTF-8, aby je zachować.
   *[other] { $count } znaków w "{ $path }" nie dekoduje. Ponownie zapisać plik jako UTF- 8, aby je zachować.
}
import-diagnostic-decoded-from-bom = "{ $path }" został zdekodowany jako { $detail }, a nie UTF- 8.
import-diagnostic-empty-file = "{ $path }" jest puste.
import-diagnostic-no-headings = "{ $path }" nie ma nagłówków, więc przybywa jako jedna pozycja.
import-diagnostic-unsupported-format = Nic nie czyta. "{ $detail }" pliki, więc "{ $path }" został pominięty.
import-diagnostic-front-matter-not-flat = Materia frontowa w "{ $path }": "{ $detail }" nie jest prostą wartością i została pominięta.
import-diagnostic-footnotes-degraded = { $count ->
    [one] Jeden przypis w "{ $path }" przybywa jako zwykły tekst: przypisy nie są odczytywane z Markdown.
    [few] { $count } przypisy w "{ $path }" przychodzą jako zwykły tekst: przypisy nie są odczytywane z Markdown.
    [many] { $count } przypisach w "{ $path }" przybywa jako zwykły tekst: przypisy nie są odczytywane z Markdown.
   *[other] { $count } przypisy w "{ $path }" przybywają jako zwykły tekst: przypisy nie są odczytywane z Markdown.
}
import-diagnostic-footnote-not-carried = { $count ->
    [one] Jeden przypis w "{ $path }" nie mógł zostać przeniesiony.
    [few] { $count } przypisy w "{ $path }" nie mogły zostać przeniesione. Pozostałe były.
    [many] { $count } przypisach w "{ $path }" nie mogło zostać przywiezionych.
   *[other] { $count } przypisy w "{ $path }" nie mogły zostać przeniesione. Pozostałe były.
}
import-diagnostic-raw-html-dropped = { $count ->
    [one] Jeden blok surowca HTML w "{ $path }" został zrzucony.
    [few] Zrzucono { $count } bloki surowego HTML w "{ $path }".
    [many] Zrzucono { $count } bloków surowych HTML w "{ $path }".
   *[other] Zrzucono { $count } bloków surowych HTML w "{ $path }".
}
import-diagnostic-nested-break-dropped = { $count ->
    [one] Jedna przerwa w scenie wewnątrz cytatu lub listy w "{ $path }" została odrzucona. Tylko przerwa na własnej linii jest zachowana.
    [few] { $count } sceny przerw wewnątrz cytatów lub list w "{ $path }" zostały wycofane. Tylko przerwa na własnej linii jest zachowana.
    [many] { $count } przerw scenicznych wewnątrz cytatów lub list w "{ $path }" zostały wycofane.
   *[other] Zrzucono { $count } przerw scenicznych wewnątrz cytatów lub list w "{ $path }". Tylko przerwa na własnej linii jest zachowana.
}
import-diagnostic-image-not-ingested = "{ $path }" odnosi się do obrazu "{ $detail }". Odniesienie pojawia się jako tekst; sam obraz nie jest kopiowany.
import-diagnostic-duplicate-title = "{ $title }" pojawia się { $count } razy. Jeśli wcześniej zaimportowałeś te pliki, to je powieli.
import-diagnostic-heading-level-jump = "{ $title }" skacze z poziomu kursu { $from } do { $to }; jest umieszczony na jednym poziomie pod jego rodzicem.
import-diagnostic-illegal-combination = "{ $title }" nosi prozę, ale { $kind } nie może trzymać żadnych. Import odbywa się do czasu zmiany typu lub pozostawić go. Nic nie jest importowane w inny sposób.
import-diagnostic-tracked-changes-flattened = { $path } było w połowie okresu rewizyjnego: { $count ->
    [one] { $count } śledzona zmiana została zaakceptowana
    [few] { $count } śledzone zmiany zostały zaakceptowane
    [many] Zaakceptowano { $count } zmian śledzonych
   *[other] Zaakceptowano zmiany śledzenia { $count }
}, a skreślenia spadła. To jest ostateczny tekst. Aby zachować swoje własne brzmienie zamiast, użyj Porównaj w ostatnim kroku.
import-diagnostic-tracked-changes-flattened-by = { $path } było w połowie okresu rewizyjnego: { $count ->
    [one] { $count } śledzona zmiana przez { $names } została przyjęta
    [few] { $count } śledzone zmiany przez { $names } zostały zaakceptowane
    [many] { $count } prześledzonych zmian przez { $names } zostały przyjęte
   *[other] { $count } zmiany śledzone przez { $names } zostały zaakceptowane
}, a skreślenia spadła. To jest ostateczny tekst. Aby zachować swoje własne brzmienie zamiast, użyj Porównaj w ostatnim kroku.
import-diagnostic-text-box-dropped = { $count ->
    [one] { $path } posiada { $count } pole tekstowe. Jego tekst znajduje się poza przepływem dokumentu, więc gdzie należy do manuskryptu nie można odpowiedzieć. Nie jest importowany.
    [few] { $path } posiada { $count } pola tekstowe. Ich tekst znajduje się poza przepływem dokumentu, więc gdzie należy do rękopisu nie można odpowiedzieć. Nie są one importowane.
    [many] { $path } posiada { $count } pól tekstowych. Ich tekst znajduje się poza przepływem dokumentu, więc gdzie należy do manuskryptu nie można odpowiedzieć. Nie są one importowane.
   *[other] { $path } posiada { $count } pól tekstowych. Ich tekst znajduje się poza przepływem dokumentu, więc gdzie należy do manuskryptu nie można odpowiedzieć. Nie są one importowane.
}
import-diagnostic-embedded-object-dropped = { $count ->
    [one] { $path } posiada { $count } wbudowany obiekt: wykres, równanie lub podobne. W rękojeści nie ma nic, co mogłoby go utrzymać.
    [few] { $path } posiada { $count } wbudowane obiekty: wykres, równanie lub podobne. W rękojeści nie ma nic, co mogłoby je utrzymać.
    [many] { $path } posiada { $count } wbudowanych obiektów: wykres, równanie lub podobne. W rękojeści nie ma nic, co mogłoby je utrzymać.
   *[other] { $path } posiada { $count } wbudowanych obiektów: wykres, równanie lub podobne. Nie ma nic w rękopis, który mógłby je utrzymać.
}
import-diagnostic-field-flattened = { $count ->
    [one] { $path } posiada { $count } pole: numer strony, odniesienie krzyżowe, datę. Zachowuje tekst, który był ostatnio wyświetlany i nie będzie ponownie aktualizowany.
    [few] { $path } posiada { $count } pola: numer strony, odniesienie krzyżowe, datę. Każdy zachowuje tekst, który ostatnio pokazywał i nie będzie ponownie aktualizowany.
    [many] { $path } posiada { $count } pól: numer strony, odnośnik krzyżowy, datę. Każdy zachowuje tekst, który ostatnio pokazywał i nie będzie ponownie aktualizowany.
   *[other] { $path } posiada { $count } pól: numer strony, odniesienie krzyżowe, datę. Każdy zachowuje tekst, który był ostatnio wyświetlany i nie będzie ponownie aktualizowany.
}
import-diagnostic-unknown-style-level = { $path } używa stylu "{ $detail }", który wygląda jak nagłówek, ale nie nazywa poziomu. Te paragrafy są importowane jako proza zamiast odgadnąć na głębokości.
import-diagnostic-comment-unanchored = Komentarz "{ $detail }" w { $path } nie mógł być dołączony do słów o nim. Jest trzymany na jego pozycji, gdzie można go przenieść.
import-diagnostic-comment-not-carried = Komentarz "{ $detail }" w { $path } nie jest importowany, ponieważ nic w pliku nie przybywa jako tekst, który może być przechowywany. Kopiuj go z pliku, jeśli potrzebujesz.
import-diagnostic-comment-replies-flattened = { $count ->
    [one] { $count } odpowiedź w { $path } nazwał komentarz, który nie jest w pliku, więc przybywa jako komentarz własny.
    [few] { $count } odpowiedzi w { $path } nazwał komentarz, który nie jest w pliku, więc przybywają jako własne komentarze.
    [many] { $count } odpowiedzi w { $path } nazwał komentarz, który nie jest w pliku, więc przybywają jako własne komentarze.
   *[other] { $count } odpowiedzi w { $path } nazwał komentarz, który nie jest w pliku, więc przybywają jako własne komentarze.
}
import-diagnostic-prose-not-verbatim = { $count ->
    [one] { $count } akapit w { $path } nie mógł być przeniesiony z jego formatowania dokładnie tak, jak napisane, więc przybywa jako zwykły tekst. Porównaj go z oryginałem.
    [few] { $count } ustępy w { $path } nie mogły być przeniesione z ich formatowania dokładnie tak, jak napisane, więc przybywają jako zwykły tekst. Porównaj je z oryginałem.
    [many] { $count } akapitów w { $path } nie można było przynieść z ich formatowania dokładnie tak, jak napisane, więc przybywają jako zwykły tekst. Porównaj je z oryginałem.
   *[other] { $count } akapitów w { $path } nie można było przynieść z ich formatowania dokładnie tak, jak napisane, więc przybywają jako zwykły tekst. Porównaj je z oryginałem.
}
import-diagnostic-styled-spaces-not-carried = { $count ->
    [one] W { $path }, { $count } odcinek pustej przestrzeni został podkreślony lub przebity, linia pozostała do wypełnienia, na przykład. Skribisto nie utrzymuje żadnej linii na spacjach samodzielnie, więc dociera jako zwykłe przestrzenie, lub wcale, gdzie zaczyna lub kończy akapit.
    [few] W { $path }, { $count } odcinki pustej przestrzeni zostały podkreślone lub wybite, linie pozostawione do wypełnienia na przykład. Skribisto nie utrzymuje żadnej linii na spacjach samodzielnie, więc docierają jako proste przestrzenie, lub wcale, gdzie zaczynają lub kończą akapit.
    [many] W { $path }, { $count } odcinków pustej przestrzeni zostały podkreślone lub trafione przez, linie pozostawione do wypełnienia na przykład. Skribisto nie utrzymuje żadnej linii na samych przestrzeniach, więc przybywają jako proste przestrzenie, lub wcale, gdzie zaczynają lub kończą akapit.
   *[other] W { $path }, { $count } odcinków pustej przestrzeni zostały podkreślone lub trafione przez, linie pozostawione do wypełnienia, na przykład. Skribisto nie utrzymuje żadnej linii na spacjach samodzielnie, więc przybywają jako zwykłe przestrzenie, lub wcale, gdzie zaczynają lub kończą akapit.
}
import-diagnostic-list-nesting-flattened = { $count ->
    [one] { $count } pozycja listy w { $path } została zagnieżdżona ponad { $limit } poziomów głębokości. Przybywa na poziom { $limit }, najgłębszy Skribisto trzyma, z jego słów i formatowania.
    [few] { $count } pozycje listy w { $path } zostały zagnieżdżone ponad { $limit } poziomów głębokości. Przybywają na poziom { $limit }, najgłębszy Skribisto trzyma, z ich słów i formatowania.
    [many] { $count } pozycji listy w { $path } zostały zagnieżdżone ponad { $limit } poziomów głębokości. Przybywają na poziom { $limit }, najgłębszy Skribisto trzyma, z ich słów i formatowania.
   *[other] { $count } pozycji listy w { $path } zostały zagnieżdżone ponad { $limit } poziomów głębokości. Przybywają na poziom { $limit }, najgłębsze Skribisto trzyma, z ich słów i formatowania.
}
import-diagnostic-table-read-as-paragraphs = { $count ->
    [one] { $count } tabela w { $path } ma rząd znacznie szerszy niż reszta, zbyt skręcony, aby położyć się jako siatka. Jego komórki przybywają jako akapity, jeden za drugim, z ich słów i formatowania.
    [few] { $count } tabele w { $path } mają rząd znacznie szerszy niż pozostałe, zbyt skręcone, aby położyć się jako siatka. Ich komórki przybywają jako akapity, jeden po drugim, z ich słów i formatowania.
    [many] { $count } tabel w { $path } mają rząd znacznie szerszy niż reszta, zbyt skręcony, aby położyć się jako siatka. Ich komórki przybywają jako akapity, jeden za drugim, z ich słów i formatowania.
   *[other] { $count } tabele w { $path } mają rząd znacznie szerszy niż pozostałe, zbyt skręcony, aby położyć się jako siatka. Ich komórki przybywają jako akapity, jeden po drugim, z ich słów i formatowania.
}
import-diagnostic-spaces-shortened = { $count ->
    [one] { $count } bieg spacji w { $path } był dłuższy niż { $limit } spacji, i przybywa skrócony do { $limit }. Słowa wokół niego są tam.
    [few] { $count } przedziały w { $path } były dłuższe niż { $limit }, a przybywają skrócone do { $limit }. Wszystkie słowa wokół nich są tam.
    [many] { $count } przebiegów przestrzeni w { $path } były dłuższe niż { $limit } przestrzenie, i przybywają skrócone do { $limit }. Słowa wokół nich są tam.
   *[other] { $count } przebiegów przestrzeni w { $path } były dłuższe niż { $limit } przestrzenie, i przybywają skrócone do { $limit }. Słowa wokół nich są tam.
}
import-diagnostic-epigraph-not-carried = "{ $title }" jest na czele epigrafu, ale { $kind } nie może go utrzymać. Cytat jest przechowywany w górnej części tekstu.
import-diagnostic-epigraph-placement-ambiguous = Epigraf znajduje się między "{ $title }" i "{ $below }" i może kierować. Zostało ono podane do "{ $title }", co jest miejscem, gdzie epigraph zwykle idzie.
import-document-diagnostics = { $errors ->
    [0] { $warnings ->
    [one] { $errors } rzecz wiedzieć
    [few] { $warnings } rzeczy wiedzieć
    [many] { $warnings } rzeczy wiedzieć
   *[other] { $warnings } rzeczy wiedzieć
        }
   *[other] { $errors ->
    [one] Nie można odczytać { $errors } pliku
    [few] Nie można odczytać { $errors } plików
    [many] Nie można odczytać { $errors } plików
   *[other] Nie można odczytać plików { $errors }
        }
}
import-document-diagnostics-none = Nic do zgłoszenia.

versions-title = Wersje
versions-scope-synopsis = Streszczenie
versions-scope-prose = Tekst
versions-loading = Patrząc przez swoje backups…
versions-empty = Brak wcześniejszej wersji tego jeszcze
versions-error = Nic nie zginęło, ale ta lista może być niekompletna.
versions-did-not-exist = Nie było jeszcze { $date }
versions-deleted-after = Usunięto po { $date }
versions-unreadable = { $count ->
    [one] { $count } kopii zapasowej nie można odczytać
    [few] { $count } kopie zapasowe nie mogły być odczytane
    [many] { $count } kopii zapasowych nie można odczytać
   *[other] { $count } kopii zapasowych nie można odczytać
}
versions-thinned = { $count ->
    [one] Starsze wersje są cienkie w miarę starzenia się. Historia projektu spadła już o { $count } wcześniejszy stan tego tekstu.
    [few] Starsze wersje są cienkie w miarę starzenia się. Historia projektu spadła już o { $count } wcześniejsze stany tego tekstu.
    [many] Starsze wersje cieńszą się wraz z wiekiem. Historia projektu spadła już { $count } wcześniejszych stanów tego tekstu.
   *[other] Starsze wersje są cienkie w miarę starzenia się. Historia projektu spadła już { $count } wcześniejszych stanów tego tekstu.
}
versions-source-backup = Z kopii zapasowej
versions-source-project = Z własnej historii projektu
versions-list-caption = Jeden wpis na zmianę, a nie na kopię zapasową
versions-pick-a-version = Wybierz wersję, aby zobaczyć, co się zmieniło
versions-earliest = Najwcześniejsza wersja, z którą można ją porównać.
versions-no-change = Nic się nie zmieniło w tej części pozycji.
versions-formatting-only = Tylko formatowanie zmieniło się tutaj.
versions-show-unchanged = Pokaż bez zmian
versions-hide-unchanged = Ukryj bez zmian
versions-next-change = Następna zmiana
versions-near = w pobliżu "{ $text }"
versions-words-added = { $count ->
    [one] { $count } słowo dodane
    [few] { $count } dodane słowa
    [many] Dodano { $count } słów
   *[other] Dodano { $count } słów
}
versions-words-removed = { $count ->
    [one] { $count } słowo usunięte
    [few] { $count } słowa usunięte
    [many] { $count } słów usuniętych
   *[other] { $count } usuniętych słów
}
versions-blocks-moved = { $count ->
    [one] { $count } akapit przeniesiony
    [few] { $count } akapity przeniesione
    [many] { $count } ustępów przeniesionych
   *[other] { $count } akapity przeniesiony
}
versions-pin = Pin kopii zapasowej ta wersja pochodzi z. Automatyczne czyszczenie nigdy go nie usunie
versions-unpin = Odpisuj kopię zapasową, z której pochodzi ta wersja. Automatyczne czyszczenie może ponownie ją usunąć
versions-pinned-only = Pokaż tylko przypięte wersje
versions-pin-note = Tylko wersje z kopii zapasowej mogą być przypięte. Zawleczka trzyma plik, a własna historia projektu nie jest jedną z nich.
versions-pinned-empty = Nic tu jeszcze nie jest przygniecione.
versions-pinned-empty-log = Tylko wersje kopii zapasowej mogą być przypięte.
versions-range-filter = Pokaż tylko wersje pomiędzy dwoma datami
versions-filtered-empty = Żadna wersja nie pasuje do filtrów, które ustawiłeś
versions-clear-filters = Wyczyść filtry
versions-last-30-days = Ostatnie 30 dni
versions-restore-button = Przywróć tę wersję
versions-restore-confirm-title = Zastąpić ten tekst wersją z { $date }?
versions-restore-confirm-text = To, co teraz masz, zostanie zastąpione tekstem tego punktu na { $date }.
versions-restore-confirm-with-comments = To, co teraz masz, zostanie zastąpione tekstem tego punktu na { $date }. { $count ->
    [one] { $count } komentarz jest zakotwiczony w bieżącym tekście i może być pozostawiony osierocony.
    [few] { $count } komentarze są zakotwiczone w obecnym tekście i mogą zostać osierocone.
    [many] { $count } komentarzy jest zakotwiczonych w obecnym tekście i może zostać osieroconych.
   *[other] Uwagi { $count } są zakotwiczone w obecnym tekście i mogą zostać osierocone.
}
versions-restore-confirm-undo-note = Najpierw robi się kopię zapasową, a Ctrl + Z cofa to w jednym kroku.
versions-restored-toast = Przywrócono wersję z { $date }
versions-undo = Cofnij
versions-restore-row-gone = Pozycja ta nie znajduje się już w tym projekcie.
versions-restore-no-home = Ten element zmienił typ od tego czasu, a stary tekst nie ma gdzie się w nim znaleźć
versions-restore-no-safety-copy = Twoje wsparcie bezpieczeństwa nie działa, więc nic się nie zmieniło.
versions-restore-failed = Przywracanie nie powiodło się: { $error }
versions-restore-backup-busy = Backup jest już uruchomiony. Spróbuj ponownie za chwilę
versions-restore-in-backup-file = Patrzysz na plik kopii zapasowej; otwórz sam projekt, aby przywrócić do niego
versions-restore-no-project = Żaden projekt nie jest otwarty
versions-recreate-button = Przynieś to z powrotem …
versions-recreate-picker-title = Gdzie to powinno pójść?
versions-recreate-picker-empty = Ten projekt nie ma żadnego segregatora.
versions-recreate-picker-confirm = Przywieź go tutaj.
versions-recreate-picker-cancel = Anuluj
versions-recreate-untitled = ta pozycja
versions-recreate-confirm-title = Sprowadzić "{ $item }" z powrotem?
versions-recreate-confirm-text = Zostanie on dodany do { $destination }, z tekstem na { $date }.
versions-recreate-confirm-undo-note = Cofnij, na wiadomość, która następuje, wyciąga go z powrotem.
versions-recreated-toast = "{ $item }" wraca do twojego projektu
versions-recreated-partial-toast = { $count ->
    [one] "{ $item }" wrócił, ale jeden z jego tekstów nie mógł być odczytany
    [few] "{ $item }" wrócił, ale { $count } z jego tekstów nie można odczytać
    [many] "{ $item }" powróciło, ale { $count } z jego tekstów nie mogło być odczytanych
   *[other] "{ $item }" jest powrotem, ale { $count } z jego tekstów nie można odczytać
}
versions-recreate-already-here = Ten przedmiot jest już w twoim projekcie.
versions-recreate-no-destination = Wybierz gdzieś w segregatorze.
versions-recreate-unreadable = Nie można było odczytać kopii zapasowej, więc nic nie dodano.
versions-recreate-failed = Nie mogłem go zwrócić: { $error }
versions-changed-percent = { $percent }% z tej zmiany
versions-hidden-paragraphs = { $count ->
    [one] … { $count } bez zmian pkt …
    [few] … { $count } niezmienione paragrafy …
    [many] … { $count } niezmienione paragrafy …
   *[other] … { $count } niezmienione punkty …
}

timeline-title = Wróć do przeszłości
timeline-coverage = { $count ->
    [one] { $count } wersja nagrana, wracając do { $oldest }
    [few] { $count } wersje nagrane, wracając do { $oldest }
    [many] { $count } wersji nagranych, wracając do { $oldest }
   *[other] Zapisane wersje { $count }, wracając do { $oldest }
}
timeline-loading = Przeglądając twój projekt, przejrzałeś swój projekt, jego historię …
timeline-empty = Żadna wersja tego projektu nie została jeszcze zarejestrowana.
timeline-no-changes = Od tego czasu nic się nie zmieniło.
timeline-no-text-changes = Od tego czasu żaden tekst się nie zmienił.
timeline-slider-label = Wersja rejestrowana
timeline-series-name = Rozmiar projektu
timeline-bars-caption = Każdy bar jest nagraną wersją, tak wysoką jak wtedy projekt.
timeline-bars-caption-periods = Zbyt wiele wersji, aby pokazać jeden po drugim, więc każdy bar jest { $period }, tak wysoki jak projekt był pod jego koniec.
timeline-unit-hour = godzina
timeline-unit-day = dzień
timeline-unit-week = tydzień
timeline-unit-month = miesiąc
timeline-range-filter = Pokaż tylko wersje pomiędzy dwoma datami
timeline-range-empty = W tych terminach nie zarejestrowano żadnej wersji
timeline-open-period = Otwórz ten okres
timeline-show-all = Pokaż całą historię
timeline-changed-since = { $count ->
    [one] { $count } pozycja różni się od { $date } i twój projekt teraz
    [few] { $count } produkty różnią się od { $date } i twój projekt teraz
    [many] { $count } pozycji różnią się między { $date } a Twoim projektem teraz
   *[other] { $count } pozycji różnią się między { $date } a Twoim projektem teraz
}
timeline-kind-added = Napisane od
timeline-kind-removed = Nie będzie już w projekcie
timeline-kind-changed = Edytowany od
timeline-kind-moved = Przemieszczone od
timeline-not-yet-written = To jeszcze nie istniało.
timeline-no-text-of-its-own = To nie ma własnego tekstu, to kurs na to, co jest w środku.
timeline-reader-close = Zamknij
timeline-reader-stamp = Jak było na { $date }
timeline-reader-loading = Otwarcie zarejestrowanej wersji …
timeline-reader-unreadable = To nagranie nie mogło być odczytywane. Backup mógł zostać przeniesiony, usunięty, albo być na dysku, który nie jest podłączony.
timeline-reader-compared = Jak było na { $date }, nastawione przeciwko temu, co mówi teraz
timeline-reader-diff-legend = Od tego czasu przeszli przez to; został dodany podkreślenie.
timeline-reader-view-label = Co pokazać
timeline-reader-view-diff = Zmiany
timeline-reader-view-text = Tekst
timeline-reader-deleted = To już nie jest w twoim projekcie, możesz przeczytać i skopiować tutaj.
timeline-prose-only-record = Ten punkt pochodzi z własnej historii projektu, który trzyma tekst i nic innego, więc co zostało usunięte lub przeniesione, ponieważ nie można go pokazać. Wybierz kopię zapasową do tego.

settings-sec-extensions = Rozszerzenia

settings-group-games-forward = Zawsze do przodu
settings-games-forward-toggle = Zagraj "Zawsze do przodu"
settings-games-forward-hint =
    Podczas gry, nic, co napisałeś nie może być odebrane: Backspace, Usuń,
    Cut, drag- and- drop i Undo są wszystkie wyłączone na powierzchniach, które wybrałeś poniżej.
    Można nadal pisać, wklejać, formatować i poruszać się swobodnie.
settings-games-session-warning =
    Jest to wybór persession: nigdy nie jest zapisany. Zamknięcie projektu, lub odejście
    Skribisto, zawsze kończy grę, i wszystko, co napisałeś podczas gry staje się
    Nie da się tego zrobić, gdy tylko to się stanie.
settings-group-games-scope = W przypadku gdy dotyczy
settings-games-in-prose = Proza Manuscript
settings-games-in-synopsis = Synopsy
settings-games-scope-hint =
    Komentarze, przypisy i tytuły nigdy nie są zamrożone: są tam, gdzie można zauważyć fix
    Zabroniłeś sobie robić.
settings-games-inert-warning =
    "Zawsze do przodu" jest włączone, ale nie odnosi się do niczego.
    Albo nic się nie zmienia, jak piszesz.

games-title = Pisanie gier
games-forward-name = Zawsze do przodu
games-forward-blurb =
    Draft bez odbierania niczego z powrotem. Usuwanie, cięcie i wyłączanie są wyłączone
    Napisz następne zdanie zamiast naprawić ostatnie.
games-forward-playing = Odtwarzanie: usuwanie jest wyłączone
games-forward-idle = Nie grasz.
games-session-note = Kończy się, gdy zamkniesz projekt.
games-scope-prose-and-synopsis = Odnosi się do twojej prozy i synopsji.
games-scope-prose = Odnosi się do twojej prozy.
games-scope-synopsis = Odnosi się do twoich notatek.
games-scope-nothing = Na razie nie ma zastosowania. Wybierz powierzchnię w Ustawieniach.
games-settings-link = Pisanie settings…
statusbar-games-forward = Zawsze do przodu
statusbar-games-forward-tooltip =
    "Zawsze do przodu" jest włączone: usuwanie, cięcie i wyłączanie są wyłączone podczas
    Draft. Kliknij, aby przestać grać.
goal-progress-words = { $g ->
    [one] { $count } z { $goal } słowa
    [few] { $count } z { $goal } słów
    [many] { $count } z { $goal } słów
   *[other] { $count } z { $goal } słów
}
goal-progress-characters = { $g ->
    [one] { $count } o znaku { $goal }
    [few] { $count } z { $goal } znaków
    [many] { $count } z { $goal } znaków
   *[other] { $count } z { $goal } znaków
}
goal-count-words = { $n ->
    [one] { $count } słowo
    [few] { $count } słowa
    [many] { $count } słów
   *[other] { $count } słowa
}

goal-count-characters = { $n ->
    [one] { $count } znak
    [few] { $count } znaki
    [many] { $count } znaków
   *[other] { $count } znaku
}

goal-unit-words = Słowa
goal-unit-characters = Znaki

inspector-goal = Cel
inspector-goal-none = Brak celu

overview-col-goal = Cel
overview-excluded-from-export = Porzucony z eksportu, więc nie liczy się w całości

goal-subtree-total-words = { $count ->
    [one] { $items } cel wewnątrz dodaje się do { $words } słów
    [few] { $items } cele wewnątrz dodać do { $words } słów
    [many] { $items } cele wewnątrz dodać do { $words } słów
   *[other] { $items } cele wewnątrz dodać do { $words } słów
}
goal-subtree-total-characters = { $count ->
    [one] { $items } cel wewnątrz dodaje do { $words } znaków
    [few] { $items } cele wewnątrz dodać do { $words } znaków
    [many] { $items } cele wewnątrz dodać do { $words } znaków
   *[other] { $items } cele wewnątrz dodać do { $words } znaków
}

new-work-goal-unit = Licz
new-work-goal-unit-hint = Używany dla każdego słowa lub celu postaci w tym projekcie. Zmienne później.

settings-group-goal-unit = Cele
settings-goal-unit-switch-title = Jednostka liczenia przełączeń
settings-goal-unit-switch-text = Cele i kamienie milowe już ustalone dla tego projektu zostały wprowadzone w { $from }. Zmiana na { $to } nie przekształca ich, więc każda istniejąca liczba będzie teraz odczytywana jako { $to }.
settings-goal-unit-switch-informative = Nic nie jest stracone. Przełącz się z powrotem w dowolnym momencie, aby przywrócić oryginalne czytanie, a następnie uaktualnić cele, które chcesz zachować.
settings-goal-unit-switch-confirm = Przełącz

goal-distribute-action = Dystrybucja …
distribute-title = Rozdzielenie celu
distribute-empty = Nie ma w tym nic, do czego można by się podzielić celem.
distribute-weight = Podziel się
distribute-weight-length = Według długości
distribute-weight-rows = Kawałki
distribute-weight-even = Równomiernie
distribute-overwrite = Zastąp wyznaczone cele
distribute-col-item = Pozycja
distribute-col-current = Teraz.
distribute-col-proposed = Po
distribute-total = Są to { $total } w stosunku do celu { $goal }.
distribute-over-budget = Cele już postawione w środku pochodzą z { $over } więcej niż ten kontener. Podnieść swój cel, obniżyć ich, lub zastąpić je.
distribute-apply = Dystrybucja
distribute-cancel = Anuluj

milestone-target-gone = Cel usunięty
milestone-no-target = Brak celu
milestone-add = Dodaj
milestone-add-label = Co powinno zostać osiągnięte

pace-summary-title = Gdzie stoi książka
pace-summary-remaining = { $words } na wynos
pace-summary-open = Otwórz plan.
pace-summary-close = Zamknij
pace-summary-dont-show = Nie pokazuj tego podczas otwierania
pace-summary-menu = Pisanie plan…

menu-help-topics = &Pomoc Topics
menu-help-shortcuts = S&króty klawiszowe …
menu-help-website = &Skribisto w sieci
menu-help-report = Sp&rawozdanie a Problem…
menu-help-check-updates = Spr&awdź aktualizacje
menu-command-palette = &Dowództwo Palette…

learn-help-topics = Tematy pomocy
learn-shortcuts = Skróty klawiszowe
learn-website = Skribisto w Internecie

help-window-title = Pomoc
help-filter-topics = Tematy filtrów
help-no-matching-topic = Brak dopasowania tematu.
help-topic-missing = Temat ten jest już niedostępny.
help-back = Wstecz
help-not-translated = Ta strona nie została jeszcze przetłumaczona, dlatego jest wyświetlana po angielsku.

help-section-getting-started = Zaczynając
help-section-writing = Pisanie
help-section-reviewing = Przegląd
help-section-exchanging = Wyciąganie słów w i na zewnątrz
help-section-keeping = Zapewnianie bezpieczeństwa pracy
help-section-extensions = Rozszerzenia

help-topic-getting-started = Twój pierwszy projekt
help-topic-writing-model = Jak książka składa się razem
help-topic-drafts-and-old-versions = Utrzymanie starszego projektu
help-topic-goals-and-pace = Cele i tempo
help-topic-comments = Komentarze
help-topic-round-trip = Wysyłanie książki do czytelnika
help-topic-export = Eksportowanie
help-topic-import-documents = Importowanie dokumentów
help-topic-import-projects = Przesunięcie całego projektu
help-topic-backups-and-versions = Kopie zapasowe i wersje

help-shortcuts-title = Skróty klawiszowe
help-shortcuts-filter = Skróty filtrów
help-shortcuts-no-matches = Nie pasuje skrót.
help-shortcuts-rebind = Zmień skróty …
help-shortcuts-close = Zamknij

command-palette-placeholder = Wpisz polecenie

shortcut-name-binder-duplicate = Duplikuj
shortcut-name-comments-add = Dodaj komentarz
shortcut-name-comments-add-paragraph = Komentarz do paragrafu
shortcut-name-spellcheck-toggle = Sprawdź pisownię
shortcut-name-editor-tab-close = Zamknij kartę
shortcut-name-editor-tab-pin = Przypnij lub odepnij kartę
shortcut-name-editor-save = Zapisz
shortcut-name-work-export = Eksport …
shortcut-name-work-new = Nowy projekt
shortcut-name-work-open = Otwórz projekt
shortcut-name-window-new = Nowe okno
shortcut-name-work-close = Zamknij projekt
shortcut-name-app-settings = Ustawienia
shortcut-name-app-quit = Zakończ
shortcut-name-editor-insert-footnote = Wstaw przypis
shortcut-name-format-scene-break = Wstaw przerwę w scenie
shortcut-name-format-major-scene-break = Wstaw poważną przerwę w scenie
shortcut-name-format-link = Wstaw odnośnik
shortcut-name-format-mark-nbsp = Wstaw przestrzeń bez przerw
shortcut-name-format-mark-nbhyphen = Wstaw nieprzerwany Hyphen
shortcut-name-format-mark-nnbsp = Wstaw wąską przestrzeń
shortcut-name-format-mark-zwsp = Wstaw przestrzeń o szerokości zero-
shortcut-name-go-next = Dalej
shortcut-name-go-prev = Poprzedni
shortcut-name-go-to = Idź do
shortcut-name-outline-toggle = Pokaż lub ukryj strukturę
shortcut-name-preview-toggle = Pokaż lub ukryj podgląd
shortcut-name-view-fullscreen = Przełącz pełny ekran
shortcut-name-view-focus-mode = Włączenie / wyłączenie trybu wolnego od distrakcji
shortcut-name-editor-size-increase = Zwiększ rozmiar tekstu
shortcut-name-editor-size-decrease = Zmniejsz rozmiar tekstu
shortcut-name-editor-size-reset = Zmień rozmiar tekstu
shortcut-name-editor-find = Znajdź
shortcut-name-editor-replace = Zastąp
shortcut-name-editor-find-next = Następny mecz
shortcut-name-editor-find-prev = Poprzednie dopasowanie
shortcut-name-search-show = Szukaj w projekcie
shortcut-name-search-replace = Zastąp projekt
shortcut-name-outline-open-to-side = Otwarte na bok
shortcut-name-help-topics = Pomoc Topics
shortcut-name-help-shortcuts = Skróty klawiszowe
shortcut-name-help-website = Skribisto w sieci
shortcut-name-help-report = Zgłoś problem
shortcut-name-help-check-updates = Sprawdź aktualizacje
update-check-behind = Wersja { $version } jest dostępna.
update-check-current = Uruchomiłeś najnowszą wersję.
update-check-failed = Nie można sprawdzić nowszej wersji. { $error }
shortcut-name-command-palette = Paleta poleceń
help-section-reference = Co to jest?

help-concept-scene-break = Przerwa w scenie
help-concept-major-scene-break = Poważna przerwa w scenie
help-concept-find-in-prose = Szukaj w tekście
help-concept-story-bible = Biblia opowieści
help-concept-goal-unit = Słowa lub znaki
help-concept-goal-progress = Postępy
help-concept-manuscript-words = Co liczy się jako rękopis
help-concept-exportable = Pozostawione poza wywozem
help-concept-distribute = Udostępnianie celu
help-concept-subtree-total = Co cele wewnątrz dodać do
help-concept-milestone = Kamień milowy
help-concept-pace-plan = Plan pisania

help-concept-tag = Etykieta
help-concept-label = Adnotacja
help-concept-point-of-view = Punkt widzenia
help-concept-epigraph = Motto
help-concept-footnote = Przypis
help-concept-chapter-mode = Kształt działu
help-concept-comment = Komentarz
help-concept-backup = Kopia zapasowa
help-concept-version = Wersja
help-concept-trash = Kosz
help-concept-spellcheck = Sprawdzanie pisowni
help-concept-search-replace = Szukaj i zastąp
help-concept-note-template = Szablon notatki
help-concept-story-bible-entry = Wpis biblijny historii
help-concept-text-replacement = Wymiana tekstu
help-concept-smart-punctuation = Inteligentna interpunkcja
help-concept-export-style = Styl eksportu
help-concept-round-trip-marks = Znaczniki przejazdu okrągłego

epigraph-mark-attribution = Linia źródłowa
epigraph-mark-attribution-tip = Zaznacz linię, w której znajduje się opiekun jako źródło cytatu, więc drukuje jako przypisanie
margin-lane-name = Znaki marży
margin-lane-provider-comments = Komentarze
margin-lane-provider-story-bible = Wpis biblijny historii
margin-lane-provider-story-bible-hint = Na notatce jest w prozie czytanie, gdzie ten wpis jest nazwany, a sceny opowiadane przez jego oczy.
margin-lane-mark-point-of-view = Powiedziane stąd.
margin-lane-mark-named-here = Nazwany tutaj.
margin-lane-provider-comments-hint = Jeżeli do tekstu dołączona jest notatka
margin-lane-provider-search = Wyszukiwarka
margin-lane-provider-search-hint = Każde trafienie tego, czego ostatnio szukałeś, gdziekolwiek jest w dokumencie.
margin-lane-provider-boundaries = W przypadku gdy każdy dokument się rozpoczyna
margin-lane-provider-boundaries-hint = Zasada na szczycie każdej sceny w strumieniu, żebyś mógł zobaczyć, gdzie jesteś.
margin-lane-provider-spelling = Pisownia
margin-lane-provider-spelling-hint = Gdzie sprawdzacz zaklęć oznaczył słowo. Wyłączony, chyba że o nie poprosisz: oznacza maszynie opinię o swojej prozie
margin-lane-spelling = { $word }, możliwe błędy w pisowni
margin-lane-search-hit = { $text }, mecz { $index } z { $total }
margin-lane-search-current = { $text }, mecz { $index } z { $total }, na którym jesteś
margin-lane-boundary = Początek { $title }
margin-lane-boundary-untitled = dokument bez tytułu
settings-page-margin-lane = Znaki marży
settings-desc-margin-lane = Pasek obok paska przewijania, i to co pokazuje
settings-margin-lane-enabled = Pokaż linię brzegową
settings-margin-lane-enabled-hint = Ten sam przełącznik co widok ▸ Margines.
settings-margin-lane-enabled-more = Nigdy nie mówi, że coś jest nie tak: pokazuje wam, gdzie są rzeczy i pozostawia wam, co z tym zrobić.
settings-group-margin-lane-marks = Co to oznacza?
settings-margin-lane-no-providers = Nic jeszcze nie wskazuje drogi.
settings-group-margin-lane-texture = Tektura dialogu
settings-margin-lane-texture = Pokaż teksturę dialogu
settings-margin-lane-texture-hint = Jeden bar na akapit: jak długo jest i jak dużo jest mówiony.
settings-margin-lane-texture-more = Nie mierzone dla języków bez kuratorskiej konwencji, gdzie bar pokazuje tylko długość.
settings-group-margin-lane-surfaces = Gdzie się pojawia
settings-margin-lane-surface-editor = Edytor tekstu
settings-margin-lane-surface-stream = Strumyki
settings-margin-lane-surface-search-preview = Podgląd wyszukiwania
menu-margin-lane = &Znaki marży

status-preset-drafting = Sporządzanie
status-preset-passes = Pozwolenia na przegląd
status-preset-plume = Plume Creator
status-todo = Do zrobienia
status-draft = Projekt
status-revised = Zmienione
status-final = Ostateczny
status-outline = Struktura
status-first-edit = Pierwsza edycja
status-second-edit = 2. Edycja
status-done = Gotowe
status-plume-draft-1 = Pierwszy projekt
status-plume-draft-2 = Drugi projekt
status-plume-draft-3 = Trzeci projekt
status-plume-edit-1 = Pierwsza edycja
status-plume-edit-2 = 2. Edycja
status-plume-edit-3 = Trzecia redakcja
status-plume-proofread = Korekta
status-plume-finished = Skończone
new-work-statuses = Przepływ pracy
new-work-statuses-hint = Etapy, przez które przesuwasz scenę. Możesz zmienić nazwę, zmienić kolejność lub dodać do nich w każdej chwili.
status-none = Brak statusu
inspector-status = Stan
overview-col-status = Stan
overview-status-mixed = Poniższe części nie zgadzają się
help-concept-status = Stan
status-completion-title = Gdzie stoi książka
status-completion-headline = { $done } z { $total } zakończonych scen
status-completion-empty = To czyta rękopis, więc wypełnia się, kiedy piszesz.
status-completion-open = Gdzie książka stoi …

settings-page-statuses = Statusy
settings-desc-statuses = Zmień nazwę, zamów, dodaj i usuń etapy przesuwania sceny
settings-statuses-add = Dodaj status
settings-statuses-add-placeholder = Nowa nazwa statusu
settings-statuses-desc =
    Etapy sceny przechodzi przez, w kolejności, z co najmniej wykończone na górze do większości
    Ta kolejność jest tym, co "mniej wykończone niż" oznacza wszędzie indziej
    w aplikacji, więc umieść je tak jak w rzeczywistości działa twój proces.
settings-statuses-apply-preset = Zastosuj preset…
settings-statuses-preset-applied = { $added ->
    [one] Dodano { $added } status
    [few] { $added } statusy dodane
    [many] Dodano { $added } statusów
   *[other] Dodano { $added } statusów
}
settings-statuses-preset-refused = Ten projekt ma już drabinę. Presety wypełniają tylko pusty. Usuń szczeble, których nie chcesz najpierw.
settings-statuses-duplicate = "{ $name }" jest już na tej drabinie
settings-statuses-added = Dodano "{ $name }"
settings-statuses-move-up = Przesuń do góry (w mniejszym stopniu)
settings-statuses-move-down = Przesuń w dół (więcej zakończonych)
settings-statuses-details-placeholder = Co oznacza ten etap (nieobowiązkowo)
settings-statuses-delete = Usuń "{ $name }"
settings-statuses-delete-in-use = { $count ->
    [one] Usuń "{ $name }": { $count } pozycja jest na nim i straci swój status
    [few] Usuń "{ $name }": { $count } pozycje są na nim i straci swój status
    [many] Usuń "{ $name }": { $count } pozycji na nim i straci swój status
   *[other] Usuń "{ $name }": { $count } pozycji są na nim i straci swój status
}
settings-statuses-deleted = Usunięty "{ $name }"
settings-statuses-deleted-in-use = { $count ->
    [one] Usunięto "{ $name }". { $count } pozycja nie ma już statusu.
    [few] Usunięto "{ $name }". { $count } pozycje nie mają już statusu.
    [many] Usunięto "{ $name }". { $count } pozycji nie ma już statusu.
   *[other] Usunięty "{ $name }". { $count } pozycji nie ma już statusu.
}
settings-statuses-empty-title = Brak jeszcze przepływu pracy
settings-statuses-empty-body = Stan mówi, jak daleko jest scena. Dodaj jeden powyżej, lub rozpocząć od góry.
status-category-planned = Planowane
status-category-drafting = Sporządzanie
status-category-needs-work = Wymaga pracy
status-category-revised = Zmienione
status-category-final = Ostateczny
