# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

menu-work = P&rojekt
menu-new-work = N&owy projekt
menu-open-work = O&twórz projekt…
menu-new-window = &Nowe okno
menu-import-from = &Importuj z
menu-create-from = &Utwórz z
menu-import-plume = &Plume Creator (.plume)…
menu-import-document = &Dokumenty (Markdown, Word, ODT)…
menu-import-manuskript = &Manuskript (.msk)…
menu-import-scrivener = &Scrivener (.scriv)…
menu-export = &Eksportuj
menu-export-book = Eksportuj książkę
menu-export-part = Eksportuj część
menu-export-chapter = Eksportuj rozdział
menu-export-scene = Eksportuj scenę
menu-export-note = Eksportuj notatkę
menu-export-paratext = Eksportuj paratekst
menu-export-folder = Eksportuj folder
menu-export-choose = Wybierz…
menu-export-none = Otwórz dokument, aby go wyeksportować
menu-save = &Zapisz
menu-save-as-file = Za&pisz jako pojedynczy plik…
menu-save-as-folder = Zapisz &jako folder…
menu-backup = Utw&órz kopię zapasową teraz
menu-close-work = Za&mknij projekt
menu-welcome = Okno &startowe…
menu-settings = &Ustawienia
menu-quit = Z&akończ

menu-view = &Widok
menu-outline = &Konspekt
menu-search = Sz&ukaj w projekcie
menu-timeline = &Cofnij się w czasie
menu-search-preview = P&odgląd wyszukiwania
menu-fullscreen = P&ełny ekran
menu-focus-mode = &Tryb bez rozpraszania
menu-format = &Format
menu-scene-break = Wst&aw separator sceny
menu-major-scene-break = Wstaw &większy separator sceny

menu-go = Pr&zejdź
menu-go-next-scene = &Następna scena
menu-go-prev-scene = P&oprzednia scena
menu-go-next-chapter = Na&stępny rozdział
menu-go-prev-chapter = &Poprzedni rozdział
menu-go-next-note = N&astępna notatka
menu-go-prev-note = Pop&rzednia notatka

menu-tools = &Narzędzia
menu-help = &Pomoc

ctx-add = &Dodaj
ctx-new-item = &Nowy element
ctx-new-folder = Nowy &folder
ctx-rename = Z&mień nazwę
ctx-duplicate = D&uplikuj
ctx-import-here = &Importuj tutaj…
ctx-indent = &Zwiększ wcięcie
ctx-outdent = Zm&niejsz wcięcie
ctx-trash = &Przenieś do kosza
ctx-open-to-side = Otwórz o&bok
ctx-open = &Otwórz
ctx-reveal-in-outline = Pokaż w &konspekcie
ctx-move-up = Przesuń w &górę
ctx-move-down = Przesuń ctx-move-down = Przesuń w d&ółw dół

ctx-tab-close = &Zamknij
ctx-tab-close-others = Z&amknij pozostałe
ctx-tab-close-all = Za&mknij wszystkie
ctx-tab-open-to-side = Otwórz o&bok
ctx-tab-open-in-main = Otwórz w &głównym panelu
ctx-tab-move-to-side = &Przenieś obok
ctx-tab-move-to-main = &Przenieś do głównego panelu
ctx-tab-move-to-new-window = Przenieś do &nowego okna
ctx-tab-move-window-unsaved = Najpierw zapisz ten projekt: drugie okno otwiera plik zapisany na dysku
ctx-tab-pin = Przypnij tę &kartę
ctx-tab-unpin = Odepnij tę &kartę
tab-pinned-tooltip = Przypięta: „Zamknij pozostałe” i „Zamknij wszystkie” jej nie zamykają

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
type-book-start = Początek książki
type-text = Tekst

new-item-book = Nowa książka
new-item-part = Nowa część
new-item-chapter = Nowy rozdział
new-item-note = Nowa notatka
new-item-note-folder = Nowy folder notatek
new-item-folder = Nowy folder
new-item-paratext = Nowy paratekst
new-item-paratext-folder = Nowy folder paratekstów
new-item-story-bible-entry = Nowy wpis biblii opowieści

placement-inside = wewnątrz
placement-after = po
placement-after-parent = po nadrzędnym
placement-top-level = najwyższy poziom


ctx-promote = P&rzekształć w
promote-chapter-folder = Rozdział jako folder
promote-flat-chapter = Rozdział płaski
promote-lossy-title = Nie ma gdzie zachować tekstu
promote-lossy-text = Typ „{ $target }” nie ma gdzie przechować tych treści: { $kinds }. Najpierw przenieś lub wyczyść ten tekst, a potem przekształć element.
content-scene-text = Tekst sceny
content-note-text = Tekst notatki
content-book-title = Tytuł książki
content-book-subtitle = Podtytuł książki
content-part-title = Tytuł części
content-chapter-title = Tytuł rozdziału
content-epigraph-text = Motto
content-paratext-text = Paratekst
promote-blocked-title = Rozdział nie jest pusty
promote-blocked-text = { $count ->
        [one] Ten rozdział zawiera jeszcze { $count } element. Przenieś go gdzie indziej lub do kosza, zanim przekształcisz rozdział w płaski.
        [few] Ten rozdział zawiera jeszcze { $count } elementy. Przenieś je gdzie indziej lub do kosza, zanim przekształcisz rozdział w płaski.
        [many] Ten rozdział zawiera jeszcze { $count } elementów. Przenieś je gdzie indziej lub do kosza, zanim przekształcisz rozdział w płaski.
       *[other] Ten rozdział zawiera jeszcze { $count } elementu. Przenieś je gdzie indziej lub do kosza, zanim przekształcisz rozdział w płaski.
    }

inspector = Inspektor
inspector-empty = Otwórz element, aby wyświetlić jego właściwości.
inspector-promote = Przekształć w…
statusbar-toggle-outline = Pokaż lub ukryj segregator
statusbar-toggle-inspector = Pokaż lub ukryj inspektora
statusbar-save-unsaved = Niezapisane zmiany. Kliknij, aby zapisać
statusbar-save-saved = Wszystkie zmiany zapisane
statusbar-save-autosave = Autozapis jest włączony. Zmiany są zapisywane na bieżąco podczas pisania
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

session-toggle = Sesja pisania: rozpocznij lub wstrzymaj skupiony sprint
session-configure = Ustaw cel liczby słów i limit czasu sesji
session-configure-title = Sesja pisania
session-word-goal = Docelowa liczba słów
session-time-limit = Limit czasu
session-time-limit-unit = min
session-no-goal = Bez celu
session-no-limit = Bez limitu
session-reset = Zresetuj sesję
session-readout = { $words ->
        [one] { $words } słowo
        [few] { $words } słowa
        [many] { $words } słów
       *[other] { $words } słowa
    } · { $time }
session-readout-timed = { $words ->
        [one] { $words } słowo
        [few] { $words } słowa
        [many] { $words } słów
       *[other] { $words } słowa
    } · pozostało { $time }
statusbar-focus-exit = Wyjdź z trybu bez rozpraszania
statusbar-focus-synopsis = Streszczenie
statusbar-focus-comments = Komentarze
statusbar-focus-go-prev = Poprzedni element (Alt+Strzałka w górę)
statusbar-focus-go-next = Następny element (Alt+Strzałka w dół)

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
settings-show-welcome = Pokazuj okno startowe przy uruchomieniu
settings-show-welcome-tip = Gdy ta opcja jest wyłączona, zamiast niego otwiera się ostatni projekt.
settings-check-for-updates = Sprawdzaj dostępność nowych wersji
settings-check-for-updates-tip = Najwyżej raz dziennie Skribisto pyta witrynę projektu, która wersja jest aktualna. Żądanie nie zawiera żadnego identyfikatora, nawet numeru używanej wersji: porównanie odbywa się na tym komputerze.

settings-title = Ustawienia
settings-close = Zamknij
settings-search = Szukaj w ustawieniach
settings-reset = Przywróć domyślne
settings-done = Gotowe
settings-cancel = Anuluj
settings-apply = Zastosuj
settings-ok = OK
settings-reset-confirm-title = Zresetować wszystkie ustawienia do domyślnych?
settings-reset-confirm-body = Przywraca ustawienia wyglądu, edytora i pisania tej aplikacji. Nie dotyczy to ustawień samego projektu, skrótów klawiszowych ani zapisanych stylów. Tej operacji nie można cofnąć.
settings-empty-title = Nie ma tu jeszcze ustawień
settings-empty-hint = W przyszłej aktualizacji pojawią się tu opcje.

settings-sec-appearance-behaviour = Wygląd i zachowanie
settings-sec-editor = Edytor
settings-sec-spelling = Pisownia
settings-sec-backup = Kopie zapasowe i synchronizacja
settings-sec-compile = Kompilacja i eksport
settings-page-appearance = Wygląd
settings-field-image-size-policy = Duże obrazy
settings-image-policy-ask = Pytaj za każdym razem
settings-image-policy-keep = Zachowaj oryginał
settings-image-policy-downscale = Optymalizuj
settings-hint-image-size-policy = Co zrobić, gdy wstawiony obraz jest większy, niż wymaga tego rękopis.
settings-page-notifications = Powiadomienia
settings-page-scene = Scena
settings-page-synopsis = Streszczenie
settings-page-notes = Notatki
settings-page-editor-behavior = Zachowanie edytora
settings-page-goals = Cele i liczba słów
settings-page-games = Gry pisarskie
settings-page-corkboard = Tablica korkowa
settings-page-distraction-free = Bez rozpraszania
settings-page-dictionaries = Słowniki
settings-page-autosave = Autozapis
settings-page-export = Formaty eksportu
settings-page-paratext = Struktury paratekstu
settings-page-keymap = Skróty klawiszowe
settings-keymap-filter = Filtruj skróty

settings-desc-sec-appearance-behaviour = Wygląd samej aplikacji i to, co robi po uruchomieniu.
settings-desc-sec-editor = Obszar pisania: jak wygląda i jak reaguje podczas pisania.
settings-desc-sec-spelling = Sprawdzanie pisowni i używane przez nie słowniki.
settings-desc-sec-backup = Jak Twoja praca trafia na dysk i jakie kopie są przechowywane.
settings-desc-sec-compile = Co opuszcza Skribisto i w jakiej postaci.
settings-desc-sec-work = Ustawienia należące do tego projektu, przechowywane wewnątrz jego pliku.
settings-desc-sec-extensions = Strony dodane przez zainstalowane tu rozszerzenia.
settings-desc-group-typography = Jedna strona na każdy rodzaj tekstu: krój pisma, rozmiar, interlinia i odstępy.
settings-desc-appearance = Język interfejsu, motyw, rozmiar tekstu i okno startowe przy uruchomieniu.
settings-desc-notifications = Wszystkie komunikaty wyświetlone w tej sesji i akcje, które można powtórzyć.
settings-desc-scene = Skład prozy scen: krój pisma, rozmiar, interlinia, wcięcia i odstępy.
settings-desc-synopsis = Skład panelu streszczenia, niezależny od rękopisu.
settings-desc-notes = Skład notatek, niezależny od rękopisu.
settings-desc-corkboard = Rozmiar kart, co pokazuje karta i jak tablica je rozmieszcza.
settings-desc-distraction-free = Typografia, szerokość kolumny i pasek sterowania podczas pisania na pełnym ekranie.
settings-desc-distraction-free-themes = Biblioteka motywów do pisania na pełnym ekranie: wbudowane i własne.
settings-desc-editor-behavior = Szerokość tekstu, położenie streszczenia, przewijanie jak w maszynie do pisania i podświetlenie wokół kursora.
settings-desc-punctuation = Domyślne zasady typograficzne, od których zaczynają nowe projekty. Obowiązują w każdym projekcie, który nie ustawia własnych.
settings-desc-goals = Cele liczby słów i znaków oraz sposób liczenia słów.
settings-desc-games = Ograniczenia, które narzucasz sobie podczas pisania, np. „Zawsze do przodu”.
settings-desc-spellcheck = Jeden przełącznik, który włącza i wyłącza sprawdzanie pisowni wszędzie.
settings-desc-dictionaries = Instaluj, usuwaj i przeglądaj słowniki ortograficzne na tym komputerze.
settings-desc-autosave = Czy Twoje zmiany są zapisywane na dysku automatycznie.
settings-desc-backup = Kiedy tworzone są kopie, gdzie są przechowywane i ile ich jest. Obowiązuje w każdym projekcie, który nie ustawia własnych zasad.
settings-desc-export = Style, według których kompilowany jest każdy eksport: wbudowane i własne.
settings-desc-paratext = Elementy przedtekstowe i zatekstowe, od których może zacząć nowy projekt.
settings-desc-keymap = Wszystkie skróty i przypisane do nich polecenia.
settings-page-user = Użytkownik
settings-desc-user = Kim jesteś: na potrzeby pisanych przez Ciebie komentarzy.
settings-group-identity = Tożsamość
settings-field-user-name = Imię i nazwisko
settings-field-user-name-placeholder = Opcjonalnie
settings-field-user-name-hint = Podpisuje Twoje komentarze i odpowiedzi. To nie to samo co nazwisko autora projektu, które widnieje na książce i jest zapisywane w pliku.
settings-field-user-initials = Twoje inicjały
settings-field-user-initials-placeholder = Z imienia i nazwiska
settings-field-user-initials-hint = To, co edytor tekstu pokazuje obok Twojego komentarza na marginesie. Zostaw puste, aby użyć widocznych inicjałów, wziętych z imienia i nazwiska.
settings-field-user-hint = Oba pola są opcjonalne i dotyczą wszystkich projektów na tym komputerze. Zmiana dotyczy komentarzy, które napiszesz od teraz. Już napisane komentarze zachowują podpis, pod którym powstały.
settings-desc-author = Nazwisko autora widniejące na tym projekcie.
settings-desc-structure = Czy rozdziały tego projektu to foldery czy pojedyncze elementy.
settings-desc-language = Język, według którego sprawdzana jest proza tego projektu.
settings-desc-work-backup = Własne zasady tworzenia kopii zapasowych tego projektu lub zasady ogólne.
settings-desc-personal-dictionary = Słowa, które ten projekt uznaje za poprawnie zapisane.
settings-desc-tags = Kolorowe znaczniki, którymi ten projekt oznacza swoje elementy.
settings-desc-templates = Szablony notatek, które można wstawiać podczas pisania w tym projekcie.
settings-desc-text-replacements = Skróty, które rozwijają się podczas pisania w tym projekcie.
settings-desc-work-punctuation = Cudzysłowy, myślniki i odstępy, których przestrzega proza tego projektu.

settings-group-typography = Typografia
settings-group-writing-column = Kolumna tekstu
settings-group-writing-view = Widok pisania
settings-group-distraction-free-strip = Pasek sterowania
settings-group-theme = Motyw
settings-group-language = Język
settings-group-startup = Uruchamianie
settings-group-updates = Aktualizacje
settings-group-autosave = Autozapis
settings-field-typeface = Krój pisma
settings-field-size = Rozmiar tekstu

editor-size-changed-manuscript = Rozmiar tekstu rękopisu: { $percent }
editor-size-changed-synopsis = Rozmiar tekstu streszczenia: { $percent }
editor-size-changed-notes = Rozmiar tekstu notatek: { $percent }
editor-size-changed-corkboard = Rozmiar tekstu kart tablicy korkowej: { $percent }
editor-size-changed-corkboard-expanded = Rozmiar tekstu rozwiniętego edytora: { $percent }
editor-size-changed-distraction-free = Rozmiar tekstu w trybie bez rozpraszania: { $percent }

menu-text-size-increase = Z&większ rozmiar tekstu
menu-text-size-decrease = Z&mniejsz rozmiar tekstu
menu-text-size-reset = P&rzywróć rozmiar tekstu
settings-field-line-height = Interlinia
settings-field-first-line-indent = Wcięcie pierwszego wiersza
settings-field-paragraph-spacing-before = Odstęp przed akapitem
settings-field-paragraph-spacing-after = Odstęp po akapicie
settings-field-column-width = Szerokość kolumny
settings-distraction-free-width-hint = Działa tylko w trybie bez rozpraszania. Szerokości kolumn scen, streszczenia i notatek pozostają bez zmian.
settings-distraction-free-title = Zachowaj nazwę elementu
settings-distraction-free-word-count = Zachowaj licznik słów
settings-distraction-free-session = Zachowaj sesję pisania
settings-distraction-free-go-to = Zachowaj przycisk „Przejdź do…”
settings-distraction-free-go = Zachowaj przyciski „Poprzedni” i „Następny”
settings-distraction-free-chrome-hint = Przycisk „Wyjdź” zostaje zawsze, niezależnie od wyboru tutaj. To Twoja droga wyjścia, gdy klawisz Escape jest zajęty.
settings-field-app-theme = Motyw
settings-theme-light = Jasny
settings-theme-dark = Ciemny
settings-field-text-scale = Rozmiar tekstu interfejsu
settings-field-language = Język interfejsu
settings-synopsis-placement = Położenie streszczenia
settings-synopsis-placement-none = Brak
settings-synopsis-placement-top = Powyżej
settings-synopsis-placement-side = Obok
settings-typewriter = Przewijanie jak w maszynie do pisania
settings-typewriter-tip = Utrzymuje pisany wiersz na stałej wysokości, a rękopis przewija się pod nim. Kliknięcie nadal umieszcza kursor tam, gdzie klikniesz.
settings-typewriter-position = Położenie wiersza
settings-typewriter-position-top-third = Górna jedna trzecia
settings-typewriter-position-middle = Środek
settings-typewriter-position-bottom-quarter = Dolna ćwiartka
settings-highlight-scope = Podświetlenie wokół kursora
settings-highlight-scope-none = Brak
settings-highlight-scope-sentence = Zdanie
settings-highlight-scope-paragraph = Akapit
settings-highlight-scope-tip-none = Strona pozostaje czysta. Podczas pisania nic nie jest cieniowane.
settings-highlight-scope-tip-sentence = Cieniuje pisane zdanie, aby wyróżniało się spośród sąsiednich.
settings-highlight-scope-tip-paragraph = Cieniuje cały pisany akapit, aby fragment, nad którym pracujesz, pozostawał w polu widzenia.
settings-group-container-views = Widoki kontenerów
settings-remember-view = Zapamiętaj ostatni widok dla każdego typu elementu
settings-remember-view-tip = Otwieraj kontener w widoku ostatnio używanym dla danego typu
settings-remember-view-tip-more =
    Książka, część i rozdział mają po kilka widoków (własną stronę, pełny
    rękopis, pełne streszczenie). Po włączeniu tej opcji każdy typ otwiera się
    w widoku, który ostatnio dla niego wybrano: np. gdy przełączysz jeden rozdział na „Pełny rozdział”,
    następny otwarty rozdział również pokaże „Pełny rozdział”. Każdy typ pamięta własny widok.
settings-group-counting = Liczenie słów
settings-counting-auto = Automatyczne (według języka)
settings-counting-whitespace = Dziel na spacjach
settings-counting-unicode-words = Słowa Unicode
settings-counting-cjk-hybrid = Z obsługą CJK (po znakach)
settings-counting-hint = Tryb automatyczny liczy chiński i japoński po znakach, a wszystkie inne języki po słowach. Zmień go tylko wtedy, gdy wynik wygląda na błędny dla Twojego języka.
settings-group-goals-display = Wyświetlanie
settings-show-characters = Pokaż liczbę znaków w pasku stanu
settings-autosave-hint = Zmiany są automatycznie zapisywane na dysku podczas pisania.

settings-sec-work = Projekt
settings-page-structure = Struktura
settings-page-author = Autor
settings-field-author-name = Imię i nazwisko autora
settings-field-author-placeholder = Opcjonalnie
settings-field-author-hint = Pojawia się na skompilowanej stronie tytułowej i w metadanych eksportowanych plików. Zostaw puste, aby pominąć. To nazwisko autora książki. Podpis Twoich komentarzy ustawiasz w Ustawienia ▸ Użytkownik.
settings-group-chapters = Rozdziały
tidy-titles-title = Uporządkuj tytuły rozdziałów
tidy-titles-none = Żaden tytuł rozdziału ani części nie powtarza jedynie własnego numeru.
tidy-titles-lead = { $count ->
        [one] Jeden rozdział lub część ma tytuł składający się wyłącznie z własnego numeru.
        [few] { $count } rozdziały i części mają tytuły składające się wyłącznie z własnego numeru.
        [many] { $count } rozdziałów i części ma tytuły składające się wyłącznie z własnego numeru.
       *[other] { $count } rozdziału i części ma tytuły składające się wyłącznie z własnego numeru.
    }
tidy-titles-explain = Po wyczyszczeniu tych tytułów każdy element będzie oznaczony numerem, który książka już zna, tym samym, który drukuje eksport. Nic więcej się nie zmienia, a jedno cofnięcie przywraca je wszystkie.
menu-document-tidy-titles = Upo&rządkuj tytuły rozdziałów…
settings-group-numbering = Numeracja
settings-number-chapters = Numeruj rozdziały i części
settings-number-chapters-tip = Rozdziały i części otrzymują numer wynikający z ich miejsca w książce, widoczny tutaj obok tytułu i drukowany przez eksport.
settings-number-chapters-tip-more = Numer nigdy nie jest zapisywany w tytule, więc pozostaje poprawny przy zmianie kolejności, wstawianiu i usuwaniu. Po wyłączeniu tej opcji eksport drukuje same tytuły, niezależnie od ustawień stylu eksportu. Aby pominąć jeden rozdział (prolog, interludium), użyj zamiast tego przełącznika „Numeracja” w inspektorze: rozdział pozostaje w książce, ale nie otrzymuje numeru.
settings-part-resets-chapter = Numeruj rozdziały od nowa w każdej części
settings-part-resets-chapter-tip = Domyślnie wyłączone: rozdziały są numerowane ciągle przez wszystkie części książki, więc „Część druga” zaczyna się od rozdziału jedenastego.
settings-part-resets-chapter-tip-more = Tak zwykle robią wydawnictwa i tego oczekuje czytelnik. Włącz tę opcję dla książki, której części mają być czytane jako osobne tomy, każdy zaczynający się od rozdziału pierwszego.
settings-chapter-flat = Rozdziały płaskie
settings-chapter-flat-hint = Włączone: rozdział jest pojedynczym elementem. Piszesz w nim i nie zawiera on scen. Wyłączone: rozdział jest folderem. Nadal możesz w nim pisać, ale może też zawierać sceny. Nowe rozdziały stosują to ustawienie; istniejące można przekształcić poleceniem „Przekształć w”.

settings-styles-builtin = Style wbudowane
settings-styles-user = Moje style
settings-styles-builtin-badge = Wbudowany
settings-styles-duplicate = Duplikuj
settings-styles-edit = Edytuj
settings-styles-delete = Usuń
settings-styles-import = Importuj…
settings-styles-export = Eksportuj…
settings-styles-copy-suffix = (kopia)
settings-styles-json-filter = Styl eksportu
settings-styles-editor-title = Edytuj styl
settings-styles-editor-none = Wybierz własny styl do edycji albo zduplikuj styl wbudowany.
settings-styles-imported = Zaimportowano styl
settings-styles-import-failed = Nie można zaimportować stylu
settings-styles-exported = Wyeksportowano styl
settings-styles-export-failed = Nie można wyeksportować stylu
settings-styles-field-name = Nazwa
settings-styles-field-chapters = Nagłówki rozdziałów
settings-styles-field-parts = Nagłówki części
settings-styles-field-scene-break = Separator sceny
settings-styles-field-major-scene-break = Większy separator sceny
settings-styles-field-spacing = Odstępy między wierszami
settings-styles-field-justify = Wyjustuj tekst
settings-styles-field-notes = Dołącz notatki
settings-styles-field-synopses = Dołącz streszczenia
settings-styles-field-scene-titles = Dołącz tytuły scen
settings-styles-field-epigraphs = Dołącz motta
settings-styles-field-paratexts = Dołącz parateksty
settings-styles-field-epigraph-placement = Położenie motta
settings-styles-epigraph-after = Po tytule
settings-styles-epigraph-before = Przed tytułem
settings-styles-field-footnotes = Dołącz przypisy
settings-styles-field-footnote-numbering = Numeracja przypisów
settings-styles-footnote-numbering-continuous = Ciągłe
settings-styles-footnote-numbering-per-chapter = Od nowa w każdym rozdziale
settings-styles-footnote-numbering-per-book = Od nowa w każdej książce
settings-styles-field-images = Obrazy
menu-image = &Obraz
settings-styles-images-beside = Obok dokumentu
settings-styles-images-embed = Wewnątrz dokumentu
settings-styles-images-omit = Pomiń
settings-styles-group-round-trip = Wysyłanie do redaktora
settings-styles-field-comments = Dołącz komentarze
settings-styles-field-round-trip-marks = Dołącz znaczniki ponownego importu
settings-styles-round-trip-hint = Tylko DOCX i ODT. Znaczniki to niewidoczne identyfikatory, dzięki którym zwrócony plik aktualizuje ten projekt, zamiast trafić obok niego jako druga kopia.
settings-styles-group-pages = Strony
settings-styles-field-cover = Zacznij od okładki
settings-styles-field-word-count = Liczba słów na stronie tytułowej
settings-styles-field-page-books = Nowa strona przed każdą książką
settings-styles-field-page-parts = Nowa strona przed każdą częścią
settings-styles-field-page-chapters = Nowa strona przed każdym rozdziałem
settings-styles-field-page-paratexts = Nowa strona przed każdym paratekstem
settings-styles-heading-none = Brak nagłówka
settings-styles-heading-numbered = Tylko numer
settings-styles-heading-title = Tylko tytuł
settings-styles-heading-both = Numer + tytuł
settings-styles-break-blank = Pusty wiersz
settings-styles-break-none = Brak
settings-styles-spacing-single = Pojedyncze
settings-styles-spacing-onehalf = 1½
settings-styles-spacing-double = Podwójne

welcome-title = Witamy w Skribisto
welcome-version = Wersja { $version }
welcome-update-available = Wersja { $version } jest dostępna
welcome-update-tip = Otwiera stronę pobierania w przeglądarce.
welcome-search = Szukaj projektów
welcome-open = Otwórz
welcome-new-work = Nowy projekt
welcome-create-from = Utwórz z…
welcome-recent-works = Ostatnie projekty
welcome-empty-recents = Brak ostatnio otwieranych projektów.
welcome-no-matches = Żaden z ostatnich projektów nie pasuje do wyszukiwania.
welcome-learn-soon = Przewodniki i porady pojawią się wkrótce.
welcome-about-blurb = Skribisto: aplikacja do pisania przepisana w języku Rust z użyciem Teksilo.
welcome-tagline = *Ciche miejsce do pisania długich tekstów.*
welcome-github = GitHub
welcome-discord = Discord
nav-works = Projekty
nav-examples = Przykłady
nav-learn = Nauka
nav-about = O programie

synopsis = Streszczenie
epigraph = Motto
pane-manuscript = Rękopis
corkboard = Tablica korkowa
overview = Przegląd

corkboard-card-count = { $count ->
        [one] { $count } karta
        [few] { $count } karty
        [many] { $count } kart
       *[other] { $count } karty
    }
corkboard-child-count = { $count ->
        [one] { $count } element
        [few] { $count } elementy
        [many] { $count } elementów
       *[other] { $count } elementu
    }
corkboard-view-nested = Zagnieżdżony
corkboard-view-flat = Płaski
corkboard-layout-hint = Widok zagnieżdżony pokazuje bezpośrednie elementy podrzędne kontenera. Otwórz kartę folderu, aby wejść do środka. Widok płaski pokazuje naraz wszystkie sceny całego kontenera.
corkboard-card-size = Rozmiar kart
corkboard-search-placeholder = Filtruj karty…
corkboard-empty-title = Jeszcze nic tu nie ma
corkboard-empty-hint = Użyj przycisku „＋ Nowy” powyżej, aby dodać pierwszy element.
corkboard-new = Nowy
corkboard-show-card-numbers = Numeruj karty
corkboard-modal-size = Rozmiar rozwiniętego edytora
corkboard-scope-hint = Dotyczy wszystkich otwartych tablic korkowych we wszystkich projektach.
corkboard-sort-manuscript = Kolejność w rękopisie
corkboard-sort-title-asc = Tytuł A–Z
corkboard-sort-title-desc = Tytuł Z–A
corkboard-card-number = Karta { $number }

duplicate = Duplikuj
duplicate-n = { $count ->
        [one] Duplikuj { $count } kartę
        [few] Duplikuj { $count } karty
        [many] Duplikuj { $count } kart
       *[other] Duplikuj { $count } karty
    }
set-label-n = { $count ->
        [one] Ustaw etykietę na { $count } karcie
        [few] Ustaw etykietę na { $count } kartach
        [many] Ustaw etykietę na { $count } kartach
       *[other] Ustaw etykietę na { $count } karty
    }
move-to-trash-n = { $count ->
        [one] Przenieś { $count } kartę do kosza
        [few] Przenieś { $count } karty do kosza
        [many] Przenieś { $count } kart do kosza
       *[other] Przenieś { $count } karty do kosza
    }
reveal-in-outline = Pokaż w konspekcie

corkboard-move-to = Przenieś do…
corkboard-move-to-n = { $count ->
        [one] Przenieś { $count } kartę do…
        [few] Przenieś { $count } karty do…
        [many] Przenieś { $count } kart do…
       *[other] Przenieś { $count } karty do…
    }
corkboard-move-picker-title = Przenieś do…
corkboard-move-picker-empty = Nie ma jeszcze segregatorów. Najpierw utwórz segregator.
corkboard-move-picker-cancel = Anuluj
corkboard-move-here = Przenieś tutaj
corkboard-moved-ok = { $count ->
        [one] Przeniesiono { $count } kartę
        [few] Przeniesiono { $count } karty
        [many] Przeniesiono { $count } kart
       *[other] Przeniesiono { $count } karty
    }
corkboard-move-into-self = Kontenera nie można przenieść do jego własnego wnętrza. Wybierz miejsce docelowe poza nim.
corkboard-move-failed = Nie udało się przenieść tych kart w to miejsce.

overview-col-title = Tytuł
overview-col-type = Typ
overview-col-label = Etykieta
overview-col-tags = Znaczniki
overview-col-books = Książki
overview-col-own-words = Słowa
overview-col-total-words = Razem
overview-row-count = { $count ->
        [one] { $count } element
        [few] { $count } elementy
        [many] { $count } elementów
       *[other] { $count } elementu
    }
overview-search-placeholder = Filtruj elementy…
overview-expand-all = Rozwiń wszystkie
overview-collapse-all = Zwiń wszystkie
overview-table-label = Zawartość
overview-empty-title = Jeszcze nic tu nie ma
overview-gone-title = Ten kontener już nie istnieje
overview-gone-hint = Został przeniesiony do kosza. Przywróć go albo zamknij tę kartę.
overview-empty-hint = Użyj przycisku „＋ Nowy” powyżej, aby dodać pierwszy element.
corkboard-grid-label = Karty tablicy korkowej
corkboard-rename-field = Zmień nazwę elementu
corkboard-expand-synopsis = Rozwiń streszczenie
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
no-content = Ten element nie ma treści do edycji.
untitled = Bez tytułu
placeholder-title = Tytuł…
placeholder-subtitle = Podtytuł…
placeholder-chapter-title = Tytuł rozdziału…
split-editor = Podziel edytor
close-split-view = Zamknij widok podzielony
drop-open-here = Otwórz tutaj
drop-open-to-side = Otwórz obok

segment-chapter = Rozdział
segment-part = Część
segment-book = Książka
segment-pace = Tempo
segment-notes = Notatki
segment-story-bible = Biblia opowieści
segment-note-own = Notatka
segment-note-details = Szczegóły
segment-note-in-prose = W prozie
pace-placeholder = Tutaj pojawi się planer tempa.
pace-empty-title = Zaplanuj tempo tej książki
pace-empty-body = Ustaw docelową liczbę słów i termin, a Skribisto wyliczy dzienne tempo, które pozwoli Ci go dotrzymać.
pace-start-planning = Rozpocznij planowanie
pace-section-schedule = Harmonogram
pace-section-progress = Postępy
pace-advancement = Zaawansowanie
pace-goal = Docelowa liczba słów
pace-deadline = Termin
pace-active = Plan tempa aktywny
pace-day-mon = pon
pace-day-tue = wt
pace-day-wed = śr
pace-day-thu = czw
pace-day-fri = pt
pace-day-sat = sob
pace-day-sun = ndz
pace-card-written = napisanych słów
pace-card-of-goal = celu
pace-card-rate = słów / dzień pisania
pace-card-days-left = pozostałych dni pisania
pace-card-streak = dni z rzędu
pace-card-ahead = słów przed planem
pace-card-behind = słów za planem
pace-charts-empty = Wykresy postępów pojawią się tutaj, gdy liczba słów zostanie zarejestrowana przy zapisie.
pace-chart-progression = Napisane słowa a cel
pace-chart-words-per-day = Słowa dziennie
pace-series-actual = Rzeczywiste
pace-series-target = Cel
pace-series-words-per-day = Słowa/dzień
pace-daily-target-line = Równe tempo: { $count ->
        [one] { $count } słowo/dzień
        [few] { $count } słowa/dzień
        [many] { $count } słów/dzień
       *[other] { $count } słowa/dzień
    }
pace-section-holidays = Dni wolne
pace-section-milestones = Kamienie milowe
pace-holidays-none = Brak dni wolnych. Liczy się każdy zaplanowany dzień.
pace-holiday-label = Nazwa dnia wolnego
pace-add-holiday = Dodaj
pace-remove = Usuń
pace-milestones-none = Brak kamieni milowych. Ustaw je dla części lub rozdziału w inspektorze.
full-chapter = Pełny rozdział
full-part = Pełna część
full-book = Pełna książka
full-synopsis = Pełne streszczenie
rename = Zmień nazwę
set-label = Ustaw etykietę
insert-scene = Wstaw scenę
insert-chapter = Wstaw rozdział
split-scene = Podziel scenę
move-up = Przesuń w górę
move-down = Przesuń w dół
merge-with-previous = Scal z poprzednią
move-to-trash = Przenieś do kosza
rename-chapter = Zmień nazwę rozdziału
add-scene = Dodaj nową scenę
add-chapter = Dodaj nowy rozdział
new-scene-title = Nowa scena
placeholder-scene-name = Nazwa sceny…
menu-cut = Wytnij
menu-copy = Kopiuj
menu-paste = Wklej
menu-paste-unformatted = Wklej bez formatowania
menu-select-all = Zaznacz wszystko
format-bold = Pogrubienie
format-italic = Kursywa
format-underline = Podkreślenie
format-strikethrough = Przekreślenie

binder = Segregator
no-work = Nie wczytano projektu
no-recent-works = Brak ostatnich projektów
switcher-open-section = Obecnie otwarte
switcher-recent-section = Ostatnie
switcher-this-window = to okno
open-project-title = Otwórz projekt
open-project-question = Jak chcesz otworzyć „{ $title }”?
open-in-new-window = Otwórz w nowym oknie
open-here = Otwórz tutaj

binder-all = Wszystkie segregatory
binder-show-all = Pokaż wszystkie segregatory
binder-new = Nowy segregator…
binder-default-name = Nowy segregator
binder-item-count = { $count ->
        [one] { $count } element
        [few] { $count } elementy
        [many] { $count } elementów
       *[other] { $count } elementu
    }
binder-search-placeholder = Filtruj konspekt…
binder-search-scope = Przeszukaj wszystkie segregatory
binder-trash-confirm-title = Przenieść segregator do kosza?
binder-trash-confirm-text = „{ $name }” i wszystkie jego elementy zostaną przeniesione do kosza.

dialog-rename = Zmień nazwę
dialog-new-binder = Nowy segregator
dialog-set-label = Ustaw etykietę
dialog-new-scene = Nowa scena
close-work-question = Zapisać zmiany przed zamknięciem projektu?
quit-question = Zapisać zmiany przed zakończeniem?
quit-save-work-question = Zapisać zmiany w projekcie „{ $title }” przed zakończeniem?
unsaved-changes = Ten projekt ma niezapisane zmiany.
new-work-unsaved-question = Zapisać zmiany przed utworzeniem nowego projektu?
open-work-unsaved-question = Zapisać zmiany przed otwarciem innego projektu?
switch-backup-discard-title = Odrzucić zmiany w tej kopii zapasowej?
switch-backup-discard-text = Zmian w kopii zapasowej nie można w niej zapisać. Użyj polecenia „Zapisz jako” lub „Przywróć”, aby je zachować, albo odrzuć je i otwórz inny projekt.
switch-save-failed = Nie udało się zapisać projektu, więc nie został zastąpiony: { $error }
switch-save-not-started = Nie udało się zapisać projektu, więc nie został zastąpiony.
close-save-failed = Nie udało się zapisać projektu, więc nie został zamknięty: { $error }
close-save-not-started = Nie udało się zapisać projektu, więc nie został zamknięty.
save-not-started = Nie udało się zapisać projektu.

could-not-open-work = Nie można otworzyć „{ $file }”
could-not-open-work-too-new = „{ $file }” wymaga nowszej wersji Skribisto
could-not-open-work-too-new-detail = Zapisano w formacie Skribisto { $written_by }; do otwarcia potrzebny jest format { $requires } lub nowszy, a ta wersja obsługuje formaty do { $supported }. Zaktualizuj Skribisto, aby otworzyć ten plik.
could-not-open-work-importing = „{ $file }” jest właśnie zapisywany przez import
could-not-open-work-importing-detail = Import nadpisuje ten projekt i jeszcze się nie zakończył. Otwórz go po zakończeniu importu: otwarty teraz pokazałby projekt zastępowany przez import, a jego następny zapis nadpisałby wynik importu tym starym projektem.
could-not-open-example = Nie można otworzyć przykładu: { $error }
could-not-open-link = Nie można otworzyć { $url }: { $error }
could-not-create-work = Nie można utworzyć projektu: { $error }
saving-as-file = Zapisywanie jako { $target }…
saving-as-folder = Zapisywanie jako { $target }/…
saved-as = Zapisano w { $target }
save-error = Nie można zapisać: { $error }
save-flush-failed = Zapis się powiódł, ale tych dokumentów nie udało się do niego przekazać i nadal są niezapisane: { $titles } ({ $error })
backup-error = Nie można utworzyć kopii zapasowej: { $error }
backing-up = Tworzenie kopii zapasowej…
backup-nothing-open = Brak otwartego projektu, z którego można utworzyć kopię zapasową.
backup-already-running = Tworzenie kopii zapasowej już trwa.
backup-complete = Kopia zapasowa ukończona (zapisane: { $ok }, już aktualne: { $skipped })
backup-partial = Kopia zapasowa zakończona (zapisane: { $ok }), { $failed ->
        [one] { $failed } miejsce docelowe zawiodło
        [few] { $failed } miejsca docelowe zawiodły
        [many] { $failed } miejsc docelowych zawiodło
       *[other] { $failed } miejsca docelowego zawiodło
    }
backup-no-destination-title = Brak dostępnego miejsca na kopię zapasową
backup-no-destination-text = Żadne ze skonfigurowanych miejsc docelowych kopii zapasowych nie jest dostępne (na przykład dysk zewnętrzny może być odłączony). Podłącz go i spróbuj ponownie albo zakończ bez tworzenia kopii zapasowej.
backup-failed-close-title = Nie można zapisać kopii zapasowej
backup-failed-close-text = Przed zamknięciem nie udało się zapisać żadnej kopii zapasowej. Zawiodły wszystkie miejsca docelowe (dysk mógł zostać odłączony, zapełniony lub może być zabezpieczony przed zapisem). Usuń problem i spróbuj ponownie albo zakończ bez tworzenia kopii zapasowej.
backup-failed-close-error-text = Przed zamknięciem nie udało się zapisać kopii zapasowej: { $error }. Usuń problem i spróbuj ponownie albo zakończ bez tworzenia kopii zapasowej.
backup-no-result = Tworzenie kopii zapasowej zakończyło się bez informacji, czy kopia powstała. Sprawdź na liście kopii zapasowych, czy pojawiła się nowa.

backup-choice-title = Plik kopii zapasowej
backup-choice-heading = Otwarto kopię zapasową
backup-choice-subtitle = To kopia zapasowa projektu z określonej chwili.
backup-choice-subtitle-dated = Kopia zapasowa utworzona { $date }.
backup-choice-body = Możesz ją otworzyć i swobodnie edytować, ale zmiany można zachować tylko poleceniem „Zapisz jako”. Oryginalny plik projektu pozostaje nietknięty. Możesz też przywrócić projekt dokładnie do stanu z tej kopii zapasowej.
backup-choice-open = Otwórz kopię zapasową
backup-choice-restore = Przywróć projekt do tego stanu…
backup-choice-not-a-backup = Nie, otwórz normalnie
backup-banner-title = Kopia zapasowa: nie można tu zapisać zmian
backup-banner-description = Użyj polecenia „Zapisz jako”, aby zachować zmiany w nowym pliku, lub „Przywróć”, aby zastąpić oryginalny projekt tą kopią zapasową.
backup-banner-restore = Przywróć…
backup-banner-save-as = Zapisz jako…
backup-restore-original-missing = Nie można znaleźć oryginalnego projektu, który miałby zostać przywrócony. Zamiast tego użyj polecenia „Zapisz jako”, aby zachować tę kopię zapasową jako nowy projekt.
backup-restore-close-elsewhere-title = Projekt otwarty w innym oknie
backup-restore-close-elsewhere-text = Projekt, który chcesz przywrócić, jest otwarty w innym oknie. Najpierw zamknij go tam, a potem spróbuj ponownie.
backup-restore-focus-window = Przejdź do tego okna
backup-restore-confirm-title = Przywrócić tę kopię zapasową?
backup-restore-confirm-text = Bieżąca wersja projektu zostanie odłożona jako kopia bezpieczeństwa, zanim zostanie zastąpiona tą wersją.
backup-restore-confirm-ok = Przywróć
backup-restore-error = Nie można przywrócić: { $error }
backup-restored-ok = Projekt przywrócony.
backup-restored-with-safety = Projekt przywrócony. Poprzednia wersja została zapisana w { $path }.
close-backup-discard-title = Odrzucić zmiany w tej kopii zapasowej?
close-backup-discard-text = Zmian w kopii zapasowej nie można w niej zapisać. Użyj polecenia „Zapisz jako”, aby je zachować, albo odrzuć je i zamknij.
quit-backup-discard-title = Odrzucić zmiany i zakończyć?
quit-backup-discard-work-question = Odrzucić zmiany w „{ $title }” i zakończyć?
quit-backup-discard-text = Zmian w kopii zapasowej nie można w niej zapisać. Użyj polecenia „Zapisz jako”, aby je zachować, albo odrzuć je i zakończ.
backup-nudge-text = Dla tego projektu nie skonfigurowano kopii zapasowych.
backup-nudge-action = Skonfiguruj kopie zapasowe…

menu-backups-list = &Kopie zapasowe…
backups-title = Kopie zapasowe
backups-loading = Wyszukiwanie kopii zapasowych…
backups-empty = Nie znaleziono jeszcze kopii zapasowych dla tego projektu.
backups-open = Otwórz
backups-reveal = Pokaż lokalizację
backups-delete = Usuń tę kopię zapasową
backups-delete-confirm-title = Usunąć tę kopię zapasową?
backups-delete-confirm-text = „{ $name }” zostanie trwale usunięta. Tej operacji nie można cofnąć.
backups-delete-error = Nie można usunąć kopii zapasowej: { $error }
backups-refresh = Odśwież
backups-close = Zamknij

settings-page-backup = Domyślne ustawienia kopii
settings-page-work-backup = Kopie zapasowe
settings-backup-general-title = Domyślne ustawienia kopii zapasowych
settings-backup-work-title = Kopie zapasowe tego projektu
settings-backup-inherit = Użyj ogólnych ustawień kopii zapasowych
settings-backup-inheriting = Ten projekt używa ogólnych ustawień kopii zapasowych.
settings-backup-none-hint = Automatyczne kopie zapasowe nie są skonfigurowane (wszystkie wyzwalacze są wyłączone).
settings-backup-last = Ostatnia kopia zapasowa: { $date }
settings-backup-last-never = Nie ma jeszcze kopii zapasowych.
settings-backup-open-list = Otwórz listę kopii zapasowych…
settings-backup-triggers = Kiedy tworzyć kopię zapasową
settings-backup-on-close = Twórz kopię zapasową przy zamykaniu projektu
settings-backup-on-open = Twórz kopię zapasową przy otwieraniu projektu
settings-backup-interval = Twórz kopię zapasową okresowo, co
settings-backup-destinations = Miejsca docelowe kopii zapasowych
settings-backup-default-location = Domyślna lokalizacja
settings-backup-usage-since = { $count ->
        [one] { $count } kopia zapasowa · { $size } · najstarsza { $oldest }
        [few] { $count } kopie zapasowe · { $size } · najstarsza { $oldest }
        [many] { $count } kopii zapasowych · { $size } · najstarsza { $oldest }
       *[other] { $count } kopii zapasowej · { $size } · najstarsza { $oldest }
    }
settings-backup-usage = { $count ->
        [one] { $count } kopia zapasowa · { $size }
        [few] { $count } kopie zapasowe · { $size }
        [many] { $count } kopii zapasowych · { $size }
       *[other] { $count } kopii zapasowej · { $size }
    }
settings-backup-usage-empty = Nic tu jeszcze nie przechowano
settings-backup-usage-measuring = Mierzenie…
settings-backup-reveal-root = Pokaż folder
settings-backup-dest-none = Brak miejsc docelowych. Kopie zapasowe są zapisywane obok projektu.
settings-backup-dest-remove = Usuń
settings-backup-dest-add = Dodaj folder…
settings-backup-dest-refresh = Odśwież
settings-backup-retention = Ile kopii zachować
settings-backup-retention-tiered = Stopniowo
settings-backup-retention-keep-n = Zachowaj ostatnie N
settings-backup-retention-tip = Jak usuwane są stare kopie zapasowe.
settings-backup-retention-tip-more = Tryb stopniowy zachowuje jedną kopię na godzinę przez dobę, jedną na dzień przez tydzień, jedną na tydzień przez miesiąc i jedną na miesiąc dla starszych, więc niedawna historia pozostaje gęsta, a dawna się przerzedza. „Zachowaj ostatnie N” po prostu zachowuje N najnowszych kopii. W obu trybach najnowsze kopie (minimum podane poniżej) są zawsze zachowywane.
settings-backup-gfs-hourly = Co godzinę (ostatnie 24 h)
settings-backup-gfs-daily = Codziennie (ostatni tydzień)
settings-backup-gfs-weekly = Co tydzień (ostatni miesiąc)
settings-backup-gfs-monthly = Co miesiąc
settings-backup-keep-n = Liczba zachowywanych kopii
settings-backup-min-keep = Zawsze zachowuj co najmniej
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
new-work-convert-later = Format można później zmienić.
new-work-location = Lokalizacja
new-work-will-create = Zostanie utworzony
new-work-language = Język domyślny
new-work-language-hint = Dotyczy nowych tekstów i sprawdzania pisowni. Każdy tekst można przełączyć na inny język.
new-work-template = Szablon
new-work-template-none = Brak
new-work-template-empty-novel = Pusta powieść
new-work-template-light-novel = Light novel
new-work-template-novel = Powieść
new-work-template-novel-in-parts = Powieść w częściach
new-work-template-notebook = Notatnik
new-work-cancel = Anuluj
new-work-create = Utwórz projekt
new-work-step-details = Szczegóły
new-work-step-language = Język i struktura
new-work-step-template = Szablon
new-work-back = Wstecz
new-work-next = Dalej
new-work-documents-title = Nowy projekt z dokumentów
new-work-step-import = Importuj
new-work-create-and-import = Utwórz i importuj…
new-work-documents-next-title = Następny krok: Twoje dokumenty
new-work-documents-next-body = Po utworzeniu projektu otworzy się nad nim kreator importu, w którym wybierzesz dokumenty, sprawdzisz strukturę odczytaną z nich przez Skribisto i wskażesz, gdzie ma trafić. Nic nie zostanie zapisane w projekcie, dopóki tego tam nie potwierdzisz.
new-work-documents-no-template = Ten projekt celowo zaczyna się pusty: bez szablonu, więc zaimportowane dokumenty będą w nim jedyną zawartością.
new-work-documents-chapter-scene-hint = Dotyczy rozdziałów tworzonych przez import. Można to później zmienić w ustawieniach projektu.
new-work-single-file-desc = Jedno archiwum .skrib (zip). Przenośne, łatwe do tworzenia kopii zapasowych.
new-work-bundle-desc = Folder zawierający wszystkie teksty i zasoby. Lepiej współpracuje z systemami kontroli wersji.
new-work-template-none-count = pusty segregator
new-work-template-empty-novel-count = 1 rozdział
new-work-template-light-novel-count = 15 rozdziałów
new-work-template-novel-count = 20 rozdziałów
new-work-template-novel-in-parts-count = 3 części, 24 rozdziały
new-work-template-notebook-count = swobodne notatki
new-work-chapter-scene = Rozdziały płaskie
new-work-chapter-scene-tip = Każdy rozdział jest pojedynczym elementem, w którym piszesz bezpośrednio, bez scen pod nim. Pozostaw wyłączone dla klasycznego układu, w którym rozdział jest folderem: w nim również piszesz bezpośrednio, ale może on też zawierać sceny.
new-work-chapter-scene-tip-more = W rozdziale piszesz w obu przypadkach. Jedyna różnica polega na tym, czy może on *zawierać* sceny. Drzewo segregatora w Skribisto służy tylko do porządkowania, więc oba układy kompilują się do tej samej książki, można je swobodnie mieszać, a polecenie „Przekształć w” zmienia typ rozdziału bez utraty choćby słowa.
new-work-name-required = Wprowadź nazwę projektu
new-work-name-invalid = Ta nazwa nie zawiera żadnych użytecznych znaków
new-work-location-required = Wybierz lokalizację
new-work-location-missing = Ten folder nie istnieje
new-work-location-not-folder = Ta ścieżka nie jest folderem
new-work-location-readonly = Brak uprawnień do zapisu w tym folderze
new-work-target-exists = W tym folderze jest już projekt o tej nazwie. Przed jego zastąpieniem pojawi się pytanie
new-work-target-is-folder = W tej lokalizacji jest już folder o tej nazwie. Wybierz inną nazwę
new-work-overwrite-title = Zastąpić istniejący projekt?
new-work-overwrite-text = „{ $name }” już istnieje w tym folderze. Zastąpić go nowym projektem?
new-work-target-open-title = Ten projekt jest otwarty
new-work-target-open-text = „{ $name }” jest otwarty w Skribisto. Utworzenie nowego projektu w jego miejscu zastąpiłoby projekt, nad którym pracujesz, a jego następny zapis nadpisałby nowy projekt. Wybierz inną nazwę lub folder albo najpierw zamknij projekt.
target-importing-title = Import zapisuje ten projekt
target-importing-text = Import nadal zapisuje „{ $name }”. Poczekaj, aż się zakończy, lub wybierz inną nazwę.
backup-restore-importing-text = Import nadal zapisuje „{ $name }”, czyli projekt, który ta kopia zapasowa miałaby zastąpić. Przywróć kopię zapasową po zakończeniu importu.

new-work-manuscript = Rękopis
new-work-notes = Notatki
new-work-research = Materiały
new-work-notebook = Notatnik
new-work-scene = Scena
new-work-note = Notatka
new-work-front-matter = Przedtekst
new-work-back-matter = Zatekst
new-work-paratext = Struktura książki
new-work-paratext-none = Brak struktury
new-work-paratext-hint = Elementy przedtekstowe i zatekstowe, którymi dana tradycja wydawnicza otwiera i zamyka książkę. Każdy z nich można później przenieść, zmienić jego nazwę lub usunąć.
new-work-tags = Znaczniki
new-work-tags-none = Brak znaczników
new-work-tags-hint = Startowa paleta znaczników dla postaci, miejsc i innych. Opcjonalna; każdy znacznik można później przemianować, przebarwić lub usunąć.
new-work-note-templates = Szablony notatek
new-work-note-templates-none = Brak szablonów
new-work-note-templates-hint = Kształt, od którego zaczyna się notatka w biblii opowieści. Opcjonalne; każdy szablon można później edytować lub usunąć.
new-work-characters = Postacie
new-work-places = Miejsca

import-plume-title = Importuj projekt Plume Creator
import-plume-source = Projekt Plume
import-plume-source-hint = Wybierz plik .plume lub .plume_backup (dowolna wersja Plume Creator).
import-plume-trash-warning = ⚠ Elementy z kosza i usunięte nie są przenoszone.
import-plume-source-required = Wybierz plik projektu Plume
import-plume-source-missing = Ten plik nie istnieje
import-plume-source-not-file = Ta ścieżka nie jest plikiem
import-plume-manuscript-binder = Rękopis
import-plume-story-bible-binder = Biblia opowieści
import-plume-notes-of = { $name } (notatki)
import-plume-content-of = { $name } (tekst)
import-plume-progress-title = Importowanie projektu Plume…
import-plume-done = Zaimportowano { $imported ->
        [one] { $imported } element.
        [few] { $imported } elementy.
        [many] { $imported } elementów.
       *[other] { $imported } elementu.
    } { $skipped ->
        [one] { $skipped } element z kosza nie został przeniesiony.
        [few] { $skipped } elementy z kosza nie zostały przeniesione.
        [many] { $skipped } elementów z kosza nie zostało przeniesionych.
       *[other] { $skipped } elementu z kosza nie zostało przeniesionych.
    }
import-plume-nested-too-deep = Projekt nie został zaimportowany: jego część „{ $part }” jest zagnieżdżona na ponad { $limit } poziomów. Żaden projekt Plume Creator nawet się do tego nie zbliża, a jego odczytanie mogłoby spowodować awarię Skribisto. Nic nie zostało zapisane.
import-plume-declares-entities = Projekt nie został zaimportowany: jego część „{ $part }” deklaruje encje XML, czego żaden projekt Plume Creator nie robi. Ich rozwinięcie mogłoby zająć więcej pamięci, niż ma komputer, i spowodować awarię Skribisto. Nic nie zostało zapisane.
import-plume-no-outline = Ten plik jest skompresowanym archiwum, ale nie projektem Plume Creator: nie ma konspektu. Nic nie zostało zapisane.
import-plume-empty-outline = Plik konspektu tego projektu Plume Creator jest pusty, więc nie ma czego importować. Nic nie zostało zapisane.
import-plume-wrong-document = To nie jest projekt Plume Creator: jego część „{ $part }” jest dokumentem typu „{ $tag }”. Nic nie zostało zapisane.
import-plume-too-new = Część „{ $part }” tego projektu zapisała wersja Plume Creator { $version }, nowsza niż ta, którą potrafi odczytać Skribisto (najwyżej { $supported }). Nic nie zostało zapisane.

import-manuskript-title = Importuj projekt Manuskript
import-manuskript-source = Projekt Manuskript
import-manuskript-source-hint = Wybierz plik .msk lub folder projektu (dowolna wersja Manuskript).
import-manuskript-source-file = Wybierz plik…
import-manuskript-source-folder = Wybierz folder…
import-manuskript-source-required = Wybierz projekt Manuskript
import-manuskript-source-missing = Ten plik lub folder nie istnieje
import-manuskript-manuscript-binder = Rękopis
import-manuskript-story-bible-binder = Biblia opowieści
import-manuskript-characters-group = Postacie
import-manuskript-world-group = Świat
import-manuskript-plots-group = Wątki
import-manuskript-project-info-note = Informacje o projekcie
import-manuskript-summary-note = Podsumowanie
import-manuskript-importance-minor = Epizodyczna
import-manuskript-importance-secondary = Drugoplanowa
import-manuskript-importance-main = Pierwszoplanowa
import-manuskript-notes-of = { $title } (notatki)
import-manuskript-progress-title = Importowanie projektu Manuskript…
import-manuskript-done = { $imported ->
        [one] Zaimportowano { $imported } element.
        [few] Zaimportowano { $imported } elementy.
        [many] Zaimportowano { $imported } elementów.
       *[other] Zaimportowano { $imported } elementu.
    } { $revisions ->
        [0] { "" }
        [one] Dołączono { $revisions } wcześniejszą wersję.
        [few] Dołączono { $revisions } wcześniejsze wersje.
        [many] Dołączono { $revisions } wcześniejszych wersji.
       *[other] Dołączono { $revisions } wcześniejszej wersji.
    }
import-manuskript-nested-too-deep = Projekt nie został zaimportowany: jego plik „{ $part }” jest zagnieżdżony na ponad { $limit } poziomów. Żaden projekt Manuskript nawet się do tego nie zbliża, a jego odczytanie mogłoby spowodować awarię Skribisto. Nic nie zostało zapisane.
import-manuskript-folders-too-deep = Projekt nie został zaimportowany: jeden z jego plików leży głębiej niż { $limit } poziomów folderów. Żaden projekt Manuskript nawet się do tego nie zbliża, a jego odczytanie mogłoby spowodować awarię Skribisto. Nic nie zostało zapisane.
import-manuskript-declares-entities = Projekt nie został zaimportowany: jego plik „{ $part }” deklaruje encje XML, czego żaden projekt Manuskript nie robi. Ich rozwinięcie mogłoby zająć więcej pamięci, niż ma komputer, i spowodować awarię Skribisto. Nic nie zostało zapisane.
import-manuskript-not-a-project-file = „{ $file }” nie jest ani archiwum projektu Manuskript, ani małym plikiem .msk, który wskazuje folder projektu. Nic nie zostało zapisane.
import-manuskript-folder-missing = „{ $stub }” wskazuje leżący obok folder projektu o nazwie „{ $folder }”, ale tego folderu tam nie ma. Projekt Manuskript w trybie folderu to jedno i drugie, więc muszą być przenoszone razem. Nic nie zostało zapisane.
import-manuskript-not-a-project = To nie wygląda na projekt Manuskript: nie ma ani oznaczenia projektu Manuskript, ani konspektu. Nic nie zostało zapisane.
import-manuskript-format-too-new = Ten projekt jest w formacie Manuskript { $version }, nowszym niż ten, który potrafi odczytać Skribisto (najwyżej { $supported }). Nic nie zostało zapisane.
import-manuskript-too-many-files = Projekt nie został zaimportowany: liczba jego plików przekracza { $limit }, a żaden projekt Manuskript nie ma ich nawet w przybliżeniu tak wielu. Nic nie zostało zapisane.
import-manuskript-too-large = Projekt nie został zaimportowany: jest większy niż { $limit } MB, czyli więcej, niż Skribisto odczytuje z projektu Manuskript. Nic nie zostało zapisane.

## Project import, shared by every importer: the dialog, its validation and its typed failures
project-import-close = Zamknij
project-import-location = Folder docelowy
project-import-location-title = Wybierz folder docelowy
project-import-name = Nazwa pliku
project-import-name-placeholder = Nazwa projektu
project-import-will-create = Zostanie utworzony
project-import-cancel = Anuluj
project-import-import = Importuj
project-import-location-required = Wybierz folder docelowy
project-import-location-missing = Ten folder nie istnieje
project-import-location-not-folder = Ta ścieżka nie jest folderem
project-import-location-readonly = Brak uprawnień do zapisu w tym folderze
project-import-name-required = Wprowadź nazwę pliku
project-import-name-exists = Plik o tej nazwie już tu istnieje. Import poprosi o potwierdzenie nadpisania
project-import-name-is-folder = Jest tu już folder o tej nazwie, być może projekt zapisany jako folder. Import nie może go zastąpić: wybierz inną nazwę
project-import-overwrite-title = Zastąpić istniejący plik?
project-import-overwrite-text = „{ $name }” już istnieje. Zastąpić go importowanym projektem?
project-import-busy-title = Trwa import innego projektu
project-import-busy-text = Projekty są importowane pojedynczo. Poczekaj na zakończenie trwającego importu lub anuluj go, a następnie uruchom ten ponownie.
project-import-progress-announcement = { $title } { $phase }
project-import-stopping = Zatrzymywanie importu…
project-import-cancelled = Import anulowany
project-import-open-now = Otwórz teraz
project-import-details = Szczegóły
project-import-report = { $count ->
        [one] { $count } uwaga dotycząca tego importu
        [few] { $count } uwagi dotyczące tego importu
        [many] { $count } uwag dotyczących tego importu
       *[other] { $count } uwagi dotyczącej tego importu
    }
project-import-report-title = O tym imporcie
project-import-error-title = Nie można zaimportować projektu
project-import-failed-cancelled = Import został anulowany. Nic nie zostało zapisane.
project-import-failed-target-exists = „{ $file }” już istnieje, a import nie miał zgody na jego zastąpienie. Nic nie zostało zapisane.
project-import-failed-target-folder = „{ $file }” jest folderem, być może projektem zapisanym jako folder, a import nie może go zastąpić. Wybierz inną nazwę. Nic nie zostało zapisane.
project-import-failed-not-written = Nie udało się zapisać „{ $file }” w „{ $folder }” ({ $reason }). Nic nie zostało zapisane.
project-import-failed-disk-full = W „{ $folder }” brakuje miejsca na zapisanie „{ $file }”. Zwolnij tam trochę miejsca, a następnie zaimportuj ponownie. Nic nie zostało zapisane.
project-import-failed-source-missing = Pliku „{ $file }” już tam nie ma. Nic nie zostało zapisane.
project-import-failed-unreadable = Nie udało się odczytać „{ $file }” ({ $reason }). Nic nie zostało zapisane.
project-import-failed-archive-damaged = „{ $file }” to skompresowany plik, którego nie udało się odczytać, prawdopodobnie dlatego, że jest uszkodzony ({ $reason }). Nic nie zostało zapisane.
project-import-failed-archive-entries = Projekt nie został zaimportowany: jego archiwum wymienia { $entries ->
        [one] { $entries } plik
        [few] { $entries } pliki
        [many] { $entries } plików
       *[other] { $entries } pliku
    }, czyli więcej, niż może zawierać projekt ({ $limit }), a tak wygląda tylko plik spreparowany, by wyczerpać pamięć. Nic nie zostało zapisane.
project-import-failed-archive-member = Projekt nie został zaimportowany: jego część „{ $part }” po rozpakowaniu zajmuje ponad { $limit } MB, a tak wygląda tylko plik spreparowany, by wyczerpać pamięć. Nic nie zostało zapisane.
project-import-failed-archive-size = Projekt nie został zaimportowany: po rozpakowaniu zajmuje ponad { $limit } MB, a tak wygląda tylko plik spreparowany, by wyczerpać pamięć. Nic nie zostało zapisane.
project-import-failed-nested = Projekt nie został zaimportowany: jego część „{ $part }” jest zagnieżdżona na ponad { $limit } poziomów, a jej odczytanie mogłoby spowodować awarię Skribisto. Nic nie zostało zapisane.
project-import-failed-entities = Projekt nie został zaimportowany: jego część „{ $part }” deklaruje encje XML. Ich rozwinięcie mogłoby zająć więcej pamięci, niż ma komputer, i spowodować awarię Skribisto. Nic nie zostało zapisane.
project-import-failed-folders = Projekt nie został zaimportowany: jeden z jego plików leży głębiej niż { $limit } poziomów folderów, a jego odczytanie mogłoby spowodować awarię Skribisto. Nic nie zostało zapisane.
project-import-failed-malformed = Projekt nie został zaimportowany: jego część „{ $part }” jest uszkodzona i nie da się jej odczytać ({ $reason }). Nic nie zostało zapisane.
project-import-failed-scrivener-refused = Skribisto nie może zaimportować tego projektu Scrivener. Nic nie zostało zapisane.
project-import-failed-scrivener-question = Projekt Scrivener zadaje pytanie, na które nie odpowiedziano podczas jego sprawdzania. Rozpocznij jego import od nowa i odpowiedz na nie, a potem zaimportuj. Nic nie zostało zapisane.
project-import-failed-scrivener-not-accepted = Aby można było zaimportować projekt Scrivener, musisz zaznaczyć pole w kroku Projekt, a nie zostało ono zaznaczone. Rozpocznij jego import od nowa i zaznacz je, a potem zaimportuj. Nic nie zostało zapisane.
project-import-failed-scrivener-changed = Projekt Scrivener zmienił się po sprawdzeniu. Rozpocznij jego import od nowa i sprawdź go ponownie, a potem zaimportuj. Nic nie zostało zapisane.
project-import-failed-internal = Import zatrzymał się na nieoczekiwanym błędzie: { $detail }

import-target-open-title = Ten projekt jest otwarty
import-target-open-text = „{ $name }” jest otwarty w Skribisto. Import w jego miejsce zastąpiłby projekt, nad którym pracujesz, a jego następny zapis nadpisałby wynik importu. Wybierz inną nazwę pliku albo najpierw zamknij projekt.
import-warnings-announcement = { $result } { $notice }.
quit-import-running-title = Import nadal trwa
quit-import-running-text = Skribisto nadal importuje „{ $name }”. Zakończenie teraz zatrzyma import: nic nie zostanie zapisane, a projekt, z którego importujesz, pozostanie bez zmian.
quit-import-stop = Zatrzymaj import i zakończ

## Import a Scrivener project: the wizard
import-scrivener-title = Importuj projekt Scrivener
import-scrivener-step-project = Projekt
import-scrivener-step-review = Sprawdzenie
import-scrivener-step-save = Zapis
import-scrivener-back = Wstecz
import-scrivener-next = Dalej
import-scrivener-import = Importuj
import-scrivener-cancel = Anuluj
import-scrivener-help = Pomoc

## Project step: choosing the project
import-scrivener-drop-title = Upuść tutaj projekt Scrivener
import-scrivener-drop-hint = Projekt .scriv, znajdujący się w nim plik .scrivx, kopia zapasowa .zip lub plik .scrivtemplate
import-scrivener-source = Projekt Scrivener
import-scrivener-source-placeholder = Ścieżka do projektu
import-scrivener-choose-file = Wybierz plik…
import-scrivener-choose-folder = Wybierz folder…
import-scrivener-source-hint = Wybierz folder .scriv projektu albo otwórz go i wybierz znajdujący się w nim plik .scrivx. Można też wybrać kopię zapasową .zip.
import-scrivener-source-hint-mac = Jeśli projekt wyświetla się jako pojedynczy plik, użyj „Wybierz plik…”. Jeśli wyświetla się jako folder, użyj „Wybierz folder…”. Można też wybrać kopię zapasową .zip.
import-scrivener-dialog-file-title = Wybierz projekt Scrivener
import-scrivener-dialog-folder-title = Wybierz folder projektu Scrivener
import-scrivener-dialog-filter = Projekt Scrivener
import-scrivener-source-missing = Pod tą ścieżką nic nie ma
import-scrivener-source-unreadable-name = Nie można odczytać tej nazwy. Zmień nazwę projektu w menedżerze plików, a następnie wybierz go ponownie.
import-scrivener-empty-hint = Wybierz projekt, aby zobaczyć, co Skribisto w nim znajduje.
import-scrivener-reading = Czytanie projektu…
import-scrivener-stop-reading = Przerwij czytanie
import-scrivener-read-failed = Nie udało się odczytać projektu
import-scrivener-read-failed-body = Podczas czytania coś poszło nie tak. Wybierz „Odczytaj projekt ponownie”, aby spróbować jeszcze raz, lub „Szczegóły”, aby zobaczyć, co poszło nie tak.
import-scrivener-details = Szczegóły
import-scrivener-check-again = Odczytaj projekt ponownie
import-scrivener-refusal-unknown-title = Tego projektu nie można zaimportować
import-scrivener-refusal-unknown-body = Skribisto nie potrafi go przenieść. Przyczynę podają szczegóły.

## Project step: the project card
import-scrivener-made-with-mac = Ostatni zapis: Scrivener { $version } dla macOS
import-scrivener-made-with-windows = Ostatni zapis: Scrivener { $version } dla Windows
import-scrivener-made-with-linux = Ostatni zapis: Scrivener { $version } dla Linuksa
import-scrivener-made-with-ios = Ostatni zapis: Scrivener { $version } na iPhone’a i iPada
import-scrivener-made-with-version = Ostatni zapis: Scrivener { $version }
import-scrivener-made-with-unknown = Projekt Scrivener, format { $format }
import-scrivener-saved-on = Zapisano { $date } na urządzeniu „{ $device }”
import-scrivener-saved-on-no-device = Zapisano { $date }
import-scrivener-card-counts = { $documents ->
        [one] { $documents } dokument
        [few] { $documents } dokumenty
        [many] { $documents } dokumentów
       *[other] { $documents } dokumentu
    }, w tym w rękopisie: { $manuscript }

## Review step
import-scrivener-review-hint = Tak zostanie przeniesiony Twój segregator. Nic nie zostanie zapisane, dopóki nie zaimportujesz, a typ wielu elementów można później zmienić w segregatorze poleceniem „Przekształć w”.
import-scrivener-choices-structure = Poziomy
import-scrivener-top-level = Foldery najwyższego poziomu
import-scrivener-choices-text = Tekst
import-scrivener-choice-containers = Foldery na poziomie { $depth }
import-scrivener-choice-texts = Dokumenty na poziomie { $depth }
import-scrivener-choice-count = { $count ->
        [one] { $count } element
        [few] { $count } elementy
        [many] { $count } elementów
       *[other] { $count } elementu
    }
import-scrivener-choice-count-example = { $count ->
        [1] { $count } element: „{ $example }”
        [few] { $count } elementy, np. „{ $example }”
        [many] { $count } elementów, np. „{ $example }”
       *[other] { $count } elementu, np. „{ $example }”
    }
import-scrivener-rung-part = Części
import-scrivener-rung-chapter = Rozdziały
import-scrivener-rung-plain = Zwykłe foldery
import-scrivener-texts-scenes = Sceny
import-scrivener-texts-chapters = Rozdziały
import-scrivener-choice-books = Książki w rękopisie
import-scrivener-books-one = Jedna książka
import-scrivener-books-several = Jedna książka na folder
import-scrivener-choice-split = { $count ->
        [1] Podziel „{ $title }” według jego nagłówka
        [few] Podziel „{ $title }” według jego { $count } nagłówków
        [many] Podziel „{ $title }” według jego { $count } nagłówków
       *[other] Podziel „{ $title }” według jego { $count } nagłówka
    }
import-scrivener-root-items = { $count ->
        [one] { $count } element
        [few] { $count } elementy
        [many] { $count } elementów
       *[other] { $count } elementu
    }
import-scrivener-root-items-manuscript = { $count ->
        [one] { $count } element. Ten folder przypomina rękopis.
        [few] { $count } elementy. Ten folder przypomina rękopis.
        [many] { $count } elementów. Ten folder przypomina rękopis.
       *[other] { $count } elementu. Ten folder przypomina rękopis.
    }
import-scrivener-dest-notes = Notatki
import-scrivener-dest-story-bible = Biblia opowieści
import-scrivener-dest-manuscript = Kolejna książka
import-scrivener-dest-leave-out = Pomiń
import-scrivener-choice-trash = { $count ->
        [one] Przenieś zawartość kosza ({ $count } element)
        [few] Przenieś zawartość kosza ({ $count } elementy)
        [many] Przenieś zawartość kosza ({ $count } elementów)
       *[other] Przenieś zawartość kosza ({ $count } elementu)
    }
import-scrivener-trash-detail = { $count ->
        [0] Żaden z nich nie zawiera tekstu.
        [one] { $count } z nich zawiera tekst.
        [few] { $count } z nich zawierają tekst.
        [many] { $count } z nich zawiera tekst.
       *[other] { $count } z nich zawiera tekst.
    }
import-scrivener-choice-blank-lines = Puste wiersze stają się separatorami scen
import-scrivener-choice-ornaments = Wiersze z ozdobnikami stają się separatorami scen
import-scrivener-preview-label = Struktura nowego projektu
import-scrivener-col-title = Tytuł
import-scrivener-col-becomes = Staje się
import-scrivener-col-exports = W eksporcie
import-scrivener-in-exports-yes = Tak
import-scrivener-in-exports-no = Nie
import-scrivener-becomes-item = Element
import-scrivener-becomes-trashed = W koszu
import-scrivener-before-title = Zanim zaimportujesz
import-scrivener-review-error-unreadable = Nie udało się przygotować sprawdzenia. Wybierz projekt ponownie.
import-scrivener-review-error-choice = Ten wybór nie pasuje do projektu. Wybierz projekt ponownie.
import-scrivener-review-error-unmappable = Przy tym wyborze segregatora nie da się przenieść bez utraty czegoś. Wybierz inaczej albo zgłoś problem.

## Save step
import-scrivener-save-hint = Import tworzy nowy projekt Skribisto. Twój projekt Scrivener jest tylko odczytywany i nigdy nie jest zmieniany.
import-scrivener-title-field = Tytuł
import-scrivener-author = Autor
import-scrivener-language = Język
import-scrivener-location-inside-project = Wybierz folder poza projektem Scrivener. Scrivener przechowuje w nim tylko własne pliki.
import-scrivener-copy-research-label = Skopiuj pliki materiałów obok nowego projektu
import-scrivener-copy-research = { $count ->
        [one] { $count } plik materiałów, którego Skribisto nie potrafi otworzyć, np. PDF lub nagranie dźwiękowe, trafi do folderu obok projektu. Jeśli to pole nie jest zaznaczone, jego notatka podaje, gdzie leży on w Twoim projekcie Scrivener.
        [few] { $count } pliki materiałów, których Skribisto nie potrafi otworzyć, np. pliki PDF lub nagrania dźwiękowe, trafią do folderu obok projektu. Jeśli to pole nie jest zaznaczone, ich notatki podają, gdzie leżą one w Twoim projekcie Scrivener.
        [many] { $count } plików materiałów, których Skribisto nie potrafi otworzyć, np. pliki PDF lub nagrania dźwiękowe, trafi do folderu obok projektu. Jeśli to pole nie jest zaznaczone, ich notatki podają, gdzie leżą one w Twoim projekcie Scrivener.
       *[other] { $count } pliku materiałów, których Skribisto nie potrafi otworzyć, np. pliki PDF lub nagrania dźwiękowe, trafi do folderu obok projektu. Jeśli to pole nie jest zaznaczone, ich notatki podają, gdzie leżą one w Twoim projekcie Scrivener.
    }
import-scrivener-default-name = Projekt Scrivener

## Progress and result
import-scrivener-progress-title = Importowanie projektu Scrivener…
import-scrivener-done-title = Zaimportowano projekt Scrivener
import-scrivener-done = { $items ->
        [one] Nowy projekt zawiera { $items } element.
        [few] Nowy projekt zawiera { $items } elementy.
        [many] Nowy projekt zawiera { $items } elementów.
       *[other] Nowy projekt zawiera { $items } elementu.
    }
import-scrivener-done-check = { $items ->
        [one] Nowy projekt zawiera { $items } element.
        [few] Nowy projekt zawiera { $items } elementy.
        [many] Nowy projekt zawiera { $items } elementów.
       *[other] Nowy projekt zawiera { $items } elementu.
    } { $findings ->
        [one] { $findings } rzecz wymaga uwagi.
        [few] { $findings } rzeczy wymagają uwagi.
        [many] { $findings } rzeczy wymaga uwagi.
       *[other] { $findings } rzeczy wymaga uwagi.
    }
import-scrivener-review-again = Sprawdź import znowu

## The import's report (the dialog See report opens)
import-scrivener-see-report = Zobacz raport
import-scrivener-report-from = Odczytano z { $path }
import-scrivener-report-to = Zapisano w { $path }
import-scrivener-report-came-across = Co zostało przeniesione
import-scrivener-report-nothing-counted = Niczego nie policzono.
import-scrivener-report-items = { $count ->
        [one] { $count } element
        [few] { $count } elementy
        [many] { $count } elementów
       *[other] { $count } elementu
    }
import-scrivener-report-words = { $count ->
        [one] { $count } słowo
        [few] { $count } słowa
        [many] { $count } słów
       *[other] { $count } słowa
    }
import-scrivener-report-synopses = { $count ->
        [one] { $count } streszczenie
        [few] { $count } streszczenia
        [many] { $count } streszczeń
       *[other] { $count } streszczenia
    }
import-scrivener-report-document-notes = { $count ->
        [one] notatki do { $count } dokumentu
        [few] notatki do { $count } dokumentów
        [many] notatki do { $count } dokumentów
       *[other] notatki do { $count } dokumentu
    }
import-scrivener-report-comments = { $count ->
        [one] { $count } komentarz
        [few] { $count } komentarze
        [many] { $count } komentarzy
       *[other] { $count } komentarza
    }
import-scrivener-report-footnotes = { $count ->
        [one] { $count } przypis
        [few] { $count } przypisy
        [many] { $count } przypisów
       *[other] { $count } przypisu
    }
import-scrivener-report-pictures = { $count ->
        [one] { $count } obraz
        [few] { $count } obrazy
        [many] { $count } obrazów
       *[other] { $count } obrazu
    }
import-scrivener-report-templates = { $count ->
        [one] { $count } szablon notatki
        [few] { $count } szablony notatek
        [many] { $count } szablonów notatek
       *[other] { $count } szablonu notatki
    }
import-scrivener-report-tags = { $count ->
        [one] { $count } znacznik
        [few] { $count } znaczniki
        [many] { $count } znaczników
       *[other] { $count } znacznika
    }
import-scrivener-report-statuses = { $count ->
        [one] { $count } status
        [few] { $count } statusy
        [many] { $count } statusów
       *[other] { $count } statusu
    }
import-scrivener-report-trashed = { $count ->
        [one] { $count } element w koszu
        [few] { $count } elementy w koszu
        [many] { $count } elementów w koszu
       *[other] { $count } elementu w koszu
    }
import-scrivener-report-research-files = { $count ->
        [one] { $count } plik materiałów
        [few] { $count } pliki materiałów
        [many] { $count } plików materiałów
       *[other] { $count } pliku materiałów
    }
import-scrivener-report-choices = Twoje wybory
import-scrivener-report-accepted-mobile = Projekt zaimportowano bez zmian wprowadzonych w programie Scrivener na iPhonie lub iPadzie, których jeszcze w nim nie było.
import-scrivener-report-accepted-no-text = Tę kopię zaimportowano z pustymi dokumentami, ponieważ nie było w niej ani słowa ich tekstu.
import-scrivener-report-choice = { $subject }: { $answer }
import-scrivener-report-yes = tak
import-scrivener-report-no = nie
import-scrivener-report-research-copied = Pliki materiałów, których Skribisto nie potrafi otworzyć, skopiowano do „{ $folder }” obok nowego projektu.
import-scrivener-report-research-not-copied = Pliki materiałów, których Skribisto nie potrafi otworzyć, nie zostały skopiowane. Ich notatki podają, gdzie leżą w Twoim projekcie Scrivener.
import-scrivener-report-research-copied-except = { $count ->
        [one] Pliki materiałów, których Skribisto nie potrafi otworzyć, skopiowano do „{ $folder }” obok nowego projektu, z wyjątkiem { $count } pliku, który zostaje w Twoim projekcie Scrivener. Jego notatka podaje, gdzie leży.
        [few] Pliki materiałów, których Skribisto nie potrafi otworzyć, skopiowano do „{ $folder }” obok nowego projektu, z wyjątkiem { $count } plików, które zostają w Twoim projekcie Scrivener. Ich notatki podają, gdzie leżą.
        [many] Pliki materiałów, których Skribisto nie potrafi otworzyć, skopiowano do „{ $folder }” obok nowego projektu, z wyjątkiem { $count } plików, które zostają w Twoim projekcie Scrivener. Ich notatki podają, gdzie leżą.
       *[other] Pliki materiałów, których Skribisto nie potrafi otworzyć, skopiowano do „{ $folder }” obok nowego projektu, z wyjątkiem { $count } pliku, które zostają w Twoim projekcie Scrivener. Ich notatki podają, gdzie leżą.
    }
import-scrivener-report-research-none-copied = { $count ->
        [1] Zaznaczono kopiowanie plików materiałów, ale jedynego takiego pliku nie udało się skopiować. Zostaje w Twoim projekcie Scrivener, a jego notatka podaje, gdzie leży.
       *[other] Zaznaczono kopiowanie plików materiałów, ale nie udało się skopiować żadnego z { $count } plików. Zostają w Twoim projekcie Scrivener, a ich notatki podają, gdzie leżą.
    }
import-scrivener-report-chose-project = Wybrano import projektu „{ $project }”, jednego z kilku projektów we wskazanym folderze lub archiwum.
import-scrivener-report-chose-manifest = Wybrano plik segregatora „{ $file }”.
import-scrivener-report-chose-enclosing = Wskazano coś wewnątrz projektu „{ $project }”, więc zaimportowano cały projekt.
import-scrivener-report-chose-upgraded = Zaimportowano „{ $current }”, projekt, w którym praca toczyła się dalej, a nie kopię zachowaną przez Scrivener przed aktualizacją.
import-scrivener-report-chose-backup = Zaimportowano „{ $backup }”, starszą kopię zachowaną przez Scrivener przed aktualizacją.
import-scrivener-report-title-placeholder-rejected = Tytuł w ustawieniach projektu był tekstem zastępczym z szablonu, więc zamiast niego zaproponowano nazwę projektu.
import-scrivener-report-author-placeholder-rejected = Autor w ustawieniach projektu był tekstem zastępczym z szablonu, więc nie zaproponowano żadnego autora.
import-scrivener-report-title-from-compile-settings = Zaproponowano tytuł podany w ustawieniach kompilacji projektu: nazwę, jaką projekt miał przy tworzeniu, chyba że ustawiono tam tytuł w programie Scrivener.
import-scrivener-report-title-from-template = Zaproponowano własny tytuł szablonu.
import-scrivener-report-title-from-folder-name = Jako tytuł zaproponowano nazwę projektu.
import-scrivener-report-title-from-project-properties = Zaproponowano tytuł podany we właściwościach projektu.
import-scrivener-report-author-from-project = Zaproponowano autora podanego w ustawieniach projektu. Może tam pozostać autor szablonu, a jego nazwisko pojawiłoby się na stronie tytułowej.
import-scrivener-report-language-not-stated = Projekt nie określał swojego języka, więc zaproponowano język interfejsu Skribisto. Od języka zależy sposób liczenia słów i zapisu cudzysłowów.
import-scrivener-report-language-from-script = Język zaproponowano na podstawie pisma, którym napisano rękopis ({ $tag }).
import-scrivener-report-you-changed-it = { $hint } Zastąpiono tę propozycję.
import-scrivener-report-content-salvaged-text = Tekstu „{ $item }” brakowało w projekcie. Jego słowa pochodzą z indeksu wyszukiwania programu Scrivener, jako zwykły tekst i być może ze starszej wersji: sprawdź je.
import-scrivener-report-content-salvaged-notes = Notatek „{ $item }” brakowało w projekcie. Ich słowa pochodzą z indeksu wyszukiwania programu Scrivener, jako zwykły tekst i być może ze starszej wersji: sprawdź je.
import-scrivener-report-content-salvaged-synopsis = Streszczenia „{ $item }” brakowało w projekcie. Jego słowa pochodzą z indeksu wyszukiwania programu Scrivener, jako zwykły tekst i być może ze starszej wersji: sprawdź je.
import-scrivener-report-document-others = { $others ->
        [one] { $sentence } To samo dzieje się w { $others } innym miejscu, wymienionym poniżej.
        [few] { $sentence } To samo dzieje się w { $others } innych miejscach, wymienionych poniżej.
        [many] { $sentence } To samo dzieje się w { $others } innych miejscach, wymienionych poniżej.
       *[other] { $sentence } To samo dzieje się w { $others } innego miejsca, wymienionego poniżej.
    }
import-scrivener-report-item-notes = { $path } (notatki dokumentu)
import-scrivener-report-item-synopsis = { $path } (streszczenie)
import-scrivener-report-item-project-note = { $path } (notatka projektu)
import-scrivener-report-item-recovered = { $path } (odzyskany tekst)
import-scrivener-report-untitled = Bez tytułu
import-scrivener-report-no-findings = Import nie ma nic więcej do zgłoszenia.
import-scrivener-report-group-unread = Nie udało się odczytać
import-scrivener-report-group-check = Do sprawdzenia
import-scrivener-report-group-changed = Przeniesione inaczej
import-scrivener-report-group-left = Pozostawione
import-scrivener-report-group-count = { $group } ({ $count })
import-scrivener-report-items-toggle = { $count ->
        [one] { $count } element
        [few] { $count } elementy
        [many] { $count } elementów
       *[other] { $count } elementu
    }
import-scrivener-report-items-toggle-named = { $count ->
        [one] { $count } element: { $line }
        [few] { $count } elementy: { $line }
        [many] { $count } elementów: { $line }
       *[other] { $count } elementu: { $line }
    }
import-scrivener-report-names-toggle = { $count ->
        [one] { $count } nazwa
        [few] { $count } nazwy
        [many] { $count } nazw
       *[other] { $count } nazwy
    }
import-scrivener-report-names-toggle-named = { $count ->
        [one] { $count } nazwa: { $line }
        [few] { $count } nazwy: { $line }
        [many] { $count } nazw: { $line }
       *[other] { $count } nazwy: { $line }
    }
import-scrivener-report-more = { $count ->
        [one] I jeszcze { $count }.
        [few] I jeszcze { $count }.
        [many] I jeszcze { $count }.
       *[other] I jeszcze { $count }.
    }
import-scrivener-report-open-item = Otwórz w edytorze
import-scrivener-report-links-hint = Otwórz nowy projekt, a następnie pokaż ten raport ponownie z palety poleceń: każdy wymieniony tu element otworzy się wtedy po kliknięciu.
import-scrivener-copy-report = Kopiuj raport
import-scrivener-report-copied = Skopiowano raport
import-scrivener-report-copy-failed = Nie udało się skopiować raportu.
import-scrivener-report-close = Zamknij
import-scrivener-report-none-title = Brak raportu importu
import-scrivener-report-none = Od uruchomienia Skribisto nie zakończył się żaden import z programu Scrivener. Skribisto przechowuje raporty ostatnich importów do chwili zamknięcia.
import-scrivener-report-gone = { $count ->
        [one] Raport tego importu nie jest już przechowywany. Skribisto przechowuje raport ostatniego importu do chwili zamknięcia.
        [few] Raport tego importu nie jest już przechowywany. Skribisto przechowuje raporty ostatnich { $count } importów do chwili zamknięcia.
        [many] Raport tego importu nie jest już przechowywany. Skribisto przechowuje raporty ostatnich { $count } importów do chwili zamknięcia.
       *[other] Raport tego importu nie jest już przechowywany. Skribisto przechowuje raporty ostatnich { $count } importu do chwili zamknięcia.
    }

## Words the import writes into the new project, in the language Skribisto shows itself in
import-scrivener-manuscript-binder = Rękopis
import-scrivener-notes-binder = Notatki
import-scrivener-notes-of = { $title } (notatki)
import-scrivener-project-notes = Notatki projektu
import-scrivener-recovered = Odzyskane teksty
import-scrivener-untitled-template = Szablon bez tytułu
import-scrivener-research-folder = { $name } (pliki materiałów)
import-scrivener-list-separator = {", "}
import-scrivener-label-details = Z pola „{ $field }” w programie Scrivener
import-scrivener-status-details = Z pola „{ $field }” w programie Scrivener
import-scrivener-unnamed-label = Etykieta { $number }
import-scrivener-unnamed-status = Status { $number }
import-scrivener-label-tag = { $name } (etykieta)
import-scrivener-untitled-tag = Bez nazwy
import-scrivener-keyword-details = Słowo kluczowe „{ $path }” w programie Scrivener
import-scrivener-unnamed-keyword = Słowo kluczowe { $number }
import-scrivener-keyword-tag = { $name } (słowo kluczowe)
import-scrivener-collection-details = Kolekcja w programie Scrivener
import-scrivener-collection-tag = { $name } (kolekcja)
import-scrivener-field-details = Z pola „{ $field }” w programie Scrivener
import-scrivener-field-value = { $field }: { $option }
import-scrivener-field-tag = { $name } (pole)
import-scrivener-deadline-day = Termin
import-scrivener-from-scrivener = Z programu Scrivener
import-scrivener-note-project = Projekt
import-scrivener-note-project-bookmarks = Zakładki projektu
import-scrivener-note-replacements = Zamiany przy kompilacji
import-scrivener-note-auto-complete = Wpisy autouzupełniania
import-scrivener-note-saved-searches = Zapisane wyszukiwania
import-scrivener-note-custom-metadata = Własne metadane
import-scrivener-note-targets = Cele
import-scrivener-note-bookmarks = Zakładki
import-scrivener-abbreviated-title = Skrócony tytuł
import-scrivener-subjects = Tematy
import-scrivener-company = Firma
import-scrivener-session-target = Cel sesji
import-scrivener-session-target-calculated = Cel sesji, obliczony przez Scrivener na podstawie terminu
import-scrivener-writing-days = Dni pisania
import-scrivener-weekday-monday = poniedziałek
import-scrivener-weekday-tuesday = wtorek
import-scrivener-weekday-wednesday = środa
import-scrivener-weekday-thursday = czwartek
import-scrivener-weekday-friday = piątek
import-scrivener-weekday-saturday = sobota
import-scrivener-weekday-sunday = niedziela
import-scrivener-draft-target = Cel rękopisu
import-scrivener-deadline = Termin
import-scrivener-unit-words = słowa
import-scrivener-unit-characters = znaki
import-scrivener-unit-pages = strony
import-scrivener-search-query = Szukana fraza
import-scrivener-search-in = Zakres wyszukiwania
import-scrivener-search-operator = Dopasowanie
import-scrivener-search-in-all = wszystko
import-scrivener-search-in-title = tytuły
import-scrivener-search-in-text = tekst
import-scrivener-search-in-notes = notatki
import-scrivener-search-in-synopsis = streszczenia
import-scrivener-search-in-keywords = słowa kluczowe
import-scrivener-search-in-label = etykiety
import-scrivener-search-in-status = statusy
import-scrivener-search-in-section-type = typy sekcji
import-scrivener-search-in-custom-metadata = własne metadane
import-scrivener-search-any = dowolne słowo
import-scrivener-search-all = wszystkie słowa
import-scrivener-search-exact = dokładna fraza
import-scrivener-label-value = { $label }: { $value }
import-scrivener-replacement-regex = wyrażenie regularne
import-scrivener-replacement-off = wyłączona
import-scrivener-in-own-note = Wymienione w notatce powstałej z tego dokumentu.

## Import a Scrivener project: what the import says about the project, in the writer's words.
## Every key the converter can give has a line here (see `import_scrivener::messages`).

# A refusal on one line: its title, then what to do.
import-scrivener-paragraph = { $title }. { $body }

## When the project cannot be imported
import-scrivener-refusal-missing-title = Pod tą ścieżką nic nie ma
import-scrivener-refusal-missing-body = Nie ma już „{ $file }”. Wybierz projekt ponownie.
import-scrivener-refusal-unreadable-title = Nie można odczytać tego projektu
import-scrivener-refusal-unreadable-body = Nie udało się odczytać „{ $file }” ({ $reason }). Sprawdź, czy możesz go otworzyć, a potem spróbuj ponownie.
import-scrivener-refusal-not-a-scrivener-project-title = Nie ma tu projektu Scrivener
import-scrivener-refusal-not-a-scrivener-project-body = Wybierz folder .scriv projektu, znajdujący się w nim plik .scrivx, kopię zapasową .zip lub plik .scrivtemplate.
import-scrivener-refusal-manifest-without-files-title = Dokumentów nie ma w tej kopii
import-scrivener-refusal-manifest-without-files-body = Jest tu „{ $manifest }”, ale nie ma folderu Files z wymienionymi w nim dokumentami. Usługa synchronizacji mogła jeszcze nie skończyć kopiowania projektu albo skopiowano tylko plik .scrivx. Jeśli wybrany został plik .scrivx, wybierz zamiast niego folder projektu przyciskiem „Wybierz folder…”.
import-scrivener-refusal-search-too-wide-title = Ten folder zawiera za dużo, by go przeszukać
import-scrivener-refusal-search-too-wide-body = Liczba folderów do przejrzenia przekracza { $limit }. Wybierz sam projekt Scrivener.
import-scrivener-refusal-scrivener-one-for-mac-title = Ten projekt utworzono w programie Scrivener 1 dla Maca
import-scrivener-refusal-scrivener-one-for-mac-body = Skribisto odczytuje projekty z programu Scrivener 2 i nowszych wersji. Otwórz go raz w programie Scrivener 2 lub 3, aby go zaktualizować, albo skompiluj go do dokumentu Word i zaimportuj ten dokument.
import-scrivener-refusal-ios-stub-without-mobile-data-title = To tylko plik zastępczy z programu Scrivener na iPhone’a lub iPada
import-scrivener-refusal-ios-stub-without-mobile-data-body = Samego projektu tu nie ma. Zsynchronizuj go z iPhone’a lub iPada albo udostępnij go stamtąd jako plik .zip i zaimportuj ten plik.
import-scrivener-refusal-newer-title = Ten projekt utworzyła nowsza wersja programu Scrivener
import-scrivener-refusal-unknown-format-version-body = Jego format, { $version }, jest nieznany Skribisto. Sprawdź, czy jest dostępna aktualizacja Skribisto. Do tego czasu skompiluj projekt w programie Scrivener do dokumentu Word i zaimportuj ten dokument.
import-scrivener-refusal-newer-format-stamp-body = Jego pliki są oznaczone jako wersja { $stamp }, a Skribisto odczytuje wersje najwyżej do { $newest }. Sprawdź, czy jest dostępna aktualizacja Skribisto. Do tego czasu skompiluj projekt w programie Scrivener do dokumentu Word i zaimportuj ten dokument.
import-scrivener-refusal-version-stamp-unreadable-title = Nie można odczytać wersji projektu
import-scrivener-refusal-version-stamp-unreadable-body = Jego plik Files/version.txt nie zawiera numeru wersji, więc Skribisto nie wie, czy potrafi odczytać ten projekt. Otwórz projekt raz w programie Scrivener, a potem zaimportuj go ponownie.
import-scrivener-refusal-damaged-manifest-title = Nie można odczytać pliku segregatora
import-scrivener-refusal-damaged-manifest-body = „{ $manifest }” jest uszkodzony. { $spare_copies ->
        [0] Otwórz projekt w programie Scrivener, który potrafi go naprawić, a potem zaimportuj go ponownie.
        [1] Scrivener przechowuje w projekcie zapasową kopię tego pliku, której Skribisto nie odczytuje. Otwórz projekt w programie Scrivener, który potrafi go naprawić, a potem zaimportuj go ponownie.
        [few] Scrivener przechowuje w projekcie { $spare_copies } zapasowe kopie tego pliku, których Skribisto nie odczytuje. Otwórz projekt w programie Scrivener, który potrafi go naprawić, a potem zaimportuj go ponownie.
        [many] Scrivener przechowuje w projekcie { $spare_copies } zapasowych kopii tego pliku, których Skribisto nie odczytuje. Otwórz projekt w programie Scrivener, który potrafi go naprawić, a potem zaimportuj go ponownie.
       *[other] Scrivener przechowuje w projekcie { $spare_copies } zapasowej kopii tego pliku, których Skribisto nie odczytuje. Otwórz projekt w programie Scrivener, który potrafi go naprawić, a potem zaimportuj go ponownie.
    }
import-scrivener-refusal-damaged-manifest-new-project-placeholder-body = „{ $manifest }” to pusty plik, który Scrivener zapisuje dla projektu, którego jeszcze nie zapisał. Otwórz projekt w programie Scrivener i pozwól mu go zapisać, a potem zaimportuj go ponownie.
import-scrivener-refusal-manifest-too-large-title = Ten plik segregatora jest za duży
import-scrivener-refusal-manifest-too-large-body = „{ $manifest }” jest większy niż { $limit } MB, czyli więcej, niż odczytuje Skribisto. Żaden prawdziwy projekt nawet się do tego nie zbliża, więc plik może być uszkodzony.
import-scrivener-refusal-no-item-identifiers-title = Segregator nie wymienia żadnego dokumentu, który Skribisto może znaleźć
import-scrivener-refusal-no-item-identifiers-body = Żaden z jego elementów nie ma identyfikatora, więc nie da się znaleźć ich plików. Otwórz projekt raz w programie Scrivener, a potem zaimportuj go ponownie.
import-scrivener-refusal-incomplete-copy-title = W tej kopii brakuje dokumentów
import-scrivener-refusal-incomplete-copy-body = Folder Files jest na miejscu, ale nie ma w nim dokumentów. Usługa synchronizacji mogła jeszcze nie skończyć kopiowania projektu. Poczekaj, aż skończy, lub ponownie skopiuj cały folder .scriv, a potem odczytaj projekt ponownie.
import-scrivener-refusal-archive-title = Nie można otworzyć tego archiwum
import-scrivener-refusal-archive-unreadable-body = Nie udało się odczytać „{ $file }” ({ $reason }). Archiwum może być uszkodzone. Rozpakuj je samodzielnie, a następnie wybierz znajdujący się w nim folder .scriv.
import-scrivener-refusal-archive-unsafe-name-title = Nie można bezpiecznie otworzyć tego archiwum
import-scrivener-refusal-archive-unsafe-name-body = „{ $file }” zawiera plik o nazwie „{ $member }”, który trafiłby poza archiwum, więc Skribisto nie otwiera tego archiwum.
import-scrivener-refusal-archive-unsafe-name-empty-body = „{ $file }” zawiera plik bez nazwy, więc Skribisto nie otwiera tego archiwum.
import-scrivener-refusal-archive-unsafe-name-control-character-body = „{ $file }” zawiera plik, którego nazwa zawiera znak sterujący, więc Skribisto nie otwiera tego archiwum.
import-scrivener-refusal-archive-too-large-title = To archiwum jest za duże, by bezpiecznie je otworzyć
import-scrivener-refusal-archive-too-many-members-body = Liczba zawartych w nim plików przekracza { $limit }. Rozpakuj je samodzielnie, a następnie wybierz znajdujący się w nim folder .scriv.
import-scrivener-refusal-archive-member-too-large-body = „{ $member }” po rozpakowaniu zajmuje ponad { $limit } MB. Rozpakuj archiwum samodzielnie, a następnie wybierz znajdujący się w nim folder .scriv.
import-scrivener-refusal-archive-too-large-body = Po rozpakowaniu zajmuje ponad { $limit } MB. Rozpakuj je samodzielnie, a następnie wybierz znajdujący się w nim folder .scriv.
import-scrivener-refusal-archive-encrypted-title = To archiwum jest chronione hasłem
import-scrivener-refusal-archive-encrypted-body = Rozpakuj „{ $file }” samodzielnie, a następnie wybierz znajdujący się w nim folder .scriv.
import-scrivener-refusal-archive-unsupported-compression-title = To archiwum jest spakowane w sposób, którego Skribisto nie potrafi odczytać
import-scrivener-refusal-archive-unsupported-compression-body = „{ $member }” jest skompresowany metodą { $method }. Rozpakuj archiwum samodzielnie, a następnie wybierz znajdujący się w nim folder .scriv.
import-scrivener-refusal-duplicate-archive-members-title = To archiwum wymienia niektóre pliki dwukrotnie
import-scrivener-refusal-duplicate-archive-members-body = „{ $file }” zawiera więcej niż jeden plik o tej samej nazwie, więc Skribisto nie może ustalić, który z nich należy do projektu. Rozpakuj je samodzielnie, a następnie wybierz znajdujący się w nim folder .scriv.
import-scrivener-refusal-template-undecodable-title = Ten plik szablonu jest niekompletny
import-scrivener-refusal-template-undecodable-too-large-title = Ten plik szablonu jest za duży, by go otworzyć
import-scrivener-refusal-template-undecodable-not-a-template-title = Ten plik nie jest szablonem programu Scrivener
import-scrivener-refusal-template-undecodable-unknown-encoding-title = Ten plik szablonu jest zapisany w postaci, której Skribisto nie zna
import-scrivener-refusal-template-undecodable-body = Nie można zdekodować „{ $file }”: plik może się kończyć, zanim skończy się zawarty w nim projekt. Pobierz go ponownie albo otwórz go w programie Scrivener i utwórz z niego projekt.
import-scrivener-refusal-template-undecodable-too-large-body = „{ $file }” jest większy niż { $limit } MB. Otwórz go w programie Scrivener, utwórz z niego projekt, a następnie zaimportuj ten projekt.
import-scrivener-refusal-template-undecodable-not-a-template-body = „{ $file }” nie zawiera projektu Scrivener. Wybierz plik szablonu zapisany przez Scrivener albo projekt utworzony na jego podstawie.
import-scrivener-refusal-template-undecodable-unknown-encoding-body = „{ $file }” przechowuje swój projekt w sposób, którego Skribisto nie potrafi zdekodować. Otwórz go w programie Scrivener, utwórz z niego projekt, a następnie zaimportuj ten projekt.
import-scrivener-refusal-cloud-placeholders-title = { $count ->
        [one] { $count } plik tego projektu jest nadal w chmurze
        [few] { $count } pliki tego projektu są nadal w chmurze
        [many] { $count } plików tego projektu jest nadal w chmurze
       *[other] { $count } pliku tego projektu jest nadal w chmurze
    }
import-scrivener-refusal-cloud-placeholders-body = { $count ->
        [1] Nie został pobrany na ten komputer: { $examples }. Ustaw w aplikacji do synchronizacji, by projekt był dostępny offline, a potem odczytaj projekt ponownie.
       *[other] Nie zostały pobrane na ten komputer, np. { $examples }. Ustaw w aplikacji do synchronizacji, by projekt był dostępny offline, a potem odczytaj projekt ponownie.
    }
import-scrivener-refusal-unreadable-members-title = { $count ->
        [one] Nie można odczytać { $count } pliku tego projektu
        [few] Nie można odczytać { $count } plików tego projektu
        [many] Nie można odczytać { $count } plików tego projektu
       *[other] Nie można odczytać { $count } pliku tego projektu
    }
import-scrivener-refusal-unreadable-members-body = Na przykład { $examples }. Sprawdź, czy masz uprawnienia do odczytu całego projektu i czy dysk jest sprawny, a potem odczytaj projekt ponownie.
import-scrivener-refusal-needed-member-is-link-title = Ten projekt wskazuje miejsce poza własnym folderem
import-scrivener-refusal-needed-member-is-link-body = „{ $member }” jest dowiązaniem do innego miejsca na tym komputerze, a Skribisto nie podąża za dowiązaniami. Skopiuj projekt tak, by każdy plik znajdował się w jego wnętrzu, a potem spróbuj ponownie.
import-scrivener-refusal-project-changed-during-read-title = Projekt zmienił się podczas czytania
import-scrivener-refusal-project-changed-during-read-body = Scrivener lub usługa synchronizacji nadal do niego zapisuje. Zamknij go w programie Scrivener, poczekaj na zakończenie synchronizacji, a potem odczytaj projekt ponownie.
import-scrivener-refusal-too-many-members-title = Ten projekt zawiera za dużo plików
import-scrivener-refusal-too-many-members-body = Liczba jego plików przekracza { $limit }, a żaden prawdziwy projekt nie ma ich tak wielu. Może to nie być projekt Scrivener albo może być uszkodzony.
import-scrivener-refusal-too-large-title = Ten projekt jest za duży
import-scrivener-refusal-too-large-body = Jego tekst zajmuje ponad { $limit } MB, czyli więcej, niż Skribisto może bezpiecznie zaimportować.
import-scrivener-refusal-structure-unmappable-title = Skribisto nie potrafi odwzorować tego segregatora
import-scrivener-refusal-structure-unmappable-body = Segregatora nie udało się przenieść bez utraty lub błędnego umieszczenia czegoś. To błąd Skribisto, a nie Twojego projektu. Zgłoś go, dołączając to, co pokazują „Szczegóły”.

## When there is a choice to make
import-scrivener-roots-title = { $count ->
        [one] Jest tu { $count } projekt. Który wybrać?
        [few] Są tu { $count } projekty. Który wybrać?
        [many] Jest tu { $count } projektów. Który wybrać?
       *[other] Jest tu { $count } projektu. Który wybrać?
    }
import-scrivener-roots-hint = Ten folder lub archiwum zawiera więcej niż jeden projekt Scrivener. Wybierz ten, który chcesz zaimportować.
import-scrivener-manifests-title = { $count ->
        [one] Ten projekt ma { $count } plik segregatora
        [few] Ten projekt ma { $count } pliki segregatora
        [many] Ten projekt ma { $count } plików segregatora
       *[other] Ten projekt ma { $count } pliku segregatora
    }
import-scrivener-manifests-hint = Scrivener przechowuje segregator, czyli listę wszystkich dokumentów i folderów, w pliku .scrivx. Dodatkowe pliki najprawdopodobniej pozostawiła usługa synchronizacji. Wybierz ten, w którym ostatnio pracowano.
import-scrivener-manifests-hint-preselected = Scrivener przechowuje segregator, czyli listę wszystkich dokumentów i folderów, w pliku .scrivx. Dodatkowe pliki najprawdopodobniej pozostawiła usługa synchronizacji. Zaznaczony jest ten, w którym najprawdopodobniej pracowano ostatnio: sprawdź to, zanim przejdziesz dalej.
import-scrivener-manifest-row = { $file } · zapisano { $date } · { $documents ->
        [one] { $documents } element
        [few] { $documents } elementy
        [many] { $documents } elementów
       *[other] { $documents } elementu
    }
import-scrivener-manifest-row-undated = { $file } · { $documents ->
        [one] { $documents } element
        [few] { $documents } elementy
        [many] { $documents } elementów
       *[other] { $documents } elementu
    }
import-scrivener-manifest-row-unread = { $file } · nie można odczytać
import-scrivener-enclosing-title = To część projektu Scrivener „{ $project }”
import-scrivener-enclosing-body = Wybrano plik lub folder wewnątrz projektu. Skribisto importuje cały projekt.
import-scrivener-enclosing-use = Importuj „{ $project }”
import-scrivener-upgrade-title = To kopia zachowana przez Scrivener przed aktualizacją
import-scrivener-upgrade-body = Gdy Scrivener aktualizuje projekt do nowszej wersji, zachowuje stary jako „{ $backup }”. Projekt, w którym praca toczyła się dalej, to leżący obok „{ $current }”.
import-scrivener-upgrade-body-unclear = Gdy Scrivener aktualizuje projekt do nowszej wersji, zachowuje stary jako „{ $backup }”. Leżącego obok projektu „{ $current }” nie udało się poprawnie odczytać, więc wybierz, który z nich zaimportować.
import-scrivener-upgrade-use-current = Importuj „{ $current }”, projekt, w którym praca toczyła się dalej
import-scrivener-upgrade-keep-backup = Importuj tę starszą kopię, „{ $backup }”

## What must be accepted before going on
import-scrivener-ios-pending-title = Zmian wprowadzonych w programie Scrivener na iPhonie lub iPadzie nie ma jeszcze w tej kopii
import-scrivener-ios-pending-body = Scrivener na komputerze wczyta te zmiany przy następnym otwarciu projektu. Otwórz projekt raz w programie Scrivener na komputerze, a potem zaimportuj go ponownie.
import-scrivener-ios-pending-accept = Importuj bez tych zmian
import-scrivener-no-text-title = { $count ->
        [one] { $count } dokument i ani słowa tekstu w tej kopii
        [few] { $count } dokumenty i ani słowa tekstu w tej kopii
        [many] { $count } dokumentów i ani słowa tekstu w tej kopii
       *[other] { $count } dokumentu i ani słowa tekstu w tej kopii
    }
import-scrivener-no-text-body = Usługa synchronizacji może nadal kopiować projekt albo kopia straciła swoje dokumenty. Jeśli zaimportujesz go teraz, każdy dokument trafi do projektu pusty.
import-scrivener-no-text-accept = Importuj puste dokumenty

## Things to know, nothing to decide
import-scrivener-notice-project-found-below-pick-title = Projekt znaleziono wewnątrz wybranego folderu
import-scrivener-notice-project-found-below-pick-body = Znajduje się w „{ $path }”.
import-scrivener-notice-source-is-template-title = To szablon programu Scrivener
import-scrivener-notice-source-is-template-named = To szablon programu Scrivener „{ $title }”
import-scrivener-notice-source-is-template-body = Jego import tworzy nowy projekt z zawartości szablonu, łącznie z jego instrukcjami.
import-scrivener-notice-source-is-backup-title = To wygląda na kopię zapasową utworzoną przez Scrivener
import-scrivener-notice-source-is-backup-body = Zawiera projekt w stanie z chwili utworzenia kopii. Jeśli praca toczyła się dalej, wybierz sam projekt.
import-scrivener-notice-importing-pre-upgrade-copy-title = Importujesz kopię zachowaną przez Scrivener przed aktualizacją
import-scrivener-notice-importing-pre-upgrade-copy-body = Projekt, w którym praca toczyła się dalej, leży obok i zawiera wszystko, co od tego czasu napisano.
import-scrivener-notice-imported-from-ios-stub-title = Ten projekt utworzono w programie Scrivener na iPhone’a lub iPada
import-scrivener-notice-imported-from-ios-stub-body = Nigdy nie był otwierany na komputerze, więc jest odczytywany z kopii przechowywanej przez iPhone’a lub iPada.
import-scrivener-notice-stale-mobile-binder-ignored-title = Stary segregator z iPhone’a lub iPada nie jest brany pod uwagę
import-scrivener-notice-stale-mobile-binder-ignored-body = Scrivener na komputerze już wczytał jego zmiany.
import-scrivener-notice-identical-manifest-copies-title = { $count ->
        [one] Ten projekt ma { $count } identyczny plik segregatora
        [few] Ten projekt ma { $count } identyczne pliki segregatora
        [many] Ten projekt ma { $count } identycznych plików segregatora
       *[other] Ten projekt ma { $count } identycznego pliku segregatora
    }
import-scrivener-notice-identical-manifest-copies-body = Zawierają to samo, więc każda z nich daje ten sam projekt.
import-scrivener-notice-manifest-copies-ignored-title = Dodatkowe kopie pliku segregatora nie są brane pod uwagę
import-scrivener-notice-manifest-copies-ignored-body = „{ $chosen }” to plik, w którym Scrivener ostatnio pracował. { $count ->
        [one] { $count } kopia pozostawiona przez usługę synchronizacji nie jest brana pod uwagę.
        [few] { $count } kopie pozostawione przez usługę synchronizacji nie są brane pod uwagę.
        [many] { $count } kopii pozostawionych przez usługę synchronizacji nie jest branych pod uwagę.
       *[other] { $count } kopii pozostawionej przez usługę synchronizacji nie jest branej pod uwagę.
    }
import-scrivener-notice-pinned-manifest-looks-like-a-copy-title = Wybrany plik segregatora wygląda na kopię z synchronizacji
import-scrivener-notice-pinned-manifest-looks-like-a-copy-body = „{ $name }” ma nazwę jak kopia pliku „{ $original }”, który leży obok. Jeśli wynik importu wygląda na nieaktualny, wybierz zamiast niego „{ $original }”.
import-scrivener-notice-pinned-manifest-has-siblings-title = { $count ->
        [one] Obok wybranego pliku segregatora leży jeszcze { $count } plik segregatora
        [few] Obok wybranego pliku segregatora leżą jeszcze { $count } pliki segregatora
        [many] Obok wybranego pliku segregatora leży jeszcze { $count } plików segregatora
       *[other] Obok wybranego pliku segregatora leży jeszcze { $count } pliku segregatora
    }
import-scrivener-notice-pinned-manifest-has-siblings-body = Odczytywany jest tylko wybrany plik.
import-scrivener-notice-legacy-v1-files-beside-title = Pliki ze starszej wersji programu Scrivener nie są brane pod uwagę
import-scrivener-notice-legacy-v1-files-beside-body = Zostały pozostawione podczas aktualizacji projektu. Odczytywane są tylko pliki bieżącej wersji.
import-scrivener-notice-mixed-payload-trees-title = W tym projekcie są pliki dwóch wersji programu Scrivener
import-scrivener-notice-mixed-payload-trees-body = Dokumenty są odczytywane z miejsc, które wskazuje plik segregatora.
import-scrivener-notice-version-disagreement-title = Pliki tego projektu nie są zgodne co do jego wersji
import-scrivener-notice-id-scheme-mismatch-body = Plik segregatora podaje wersję { $version }, ale nazywa dokumenty tak jak inna wersja. Są one odczytywane z miejsc, które wskazuje. Jeśli wynik importu wygląda na błędny, otwórz projekt raz w programie Scrivener i zaimportuj go ponownie.
import-scrivener-notice-version-stamp-mismatch-body = Projekt jest odczytywany tak, jak opisuje go plik segregatora. Jeśli wynik importu wygląda na błędny, otwórz projekt raz w programie Scrivener i zaimportuj go ponownie.
import-scrivener-notice-unrecognised-writer-title = Ten projekt zapisał program, którego Skribisto nie rozpoznaje
import-scrivener-notice-unrecognised-writer-named = Program przedstawia się jako „{ $creator }”. Projekt jest odczytywany jako projekt Scrivener.
import-scrivener-notice-unrecognised-writer-body = Program się nie przedstawia. Projekt jest odczytywany jako projekt Scrivener.
import-scrivener-notice-invalid-item-identifier-title = { $count ->
        [one] { $count } element segregatora ma nieprawidłowy identyfikator
        [few] { $count } elementy segregatora mają nieprawidłowe identyfikatory
        [many] { $count } elementów segregatora ma nieprawidłowe identyfikatory
       *[other] { $count } elementu segregatora ma nieprawidłowe identyfikatory
    }
import-scrivener-notice-invalid-item-identifier-body = Taki identyfikator nie jest brany pod uwagę, a element, który go ma, zostaje zachowany.
import-scrivener-notice-items-without-identifier-title = { $count ->
        [one] { $count } element segregatora nie ma identyfikatora
        [few] { $count } elementy segregatora nie mają identyfikatora
        [many] { $count } elementów segregatora nie ma identyfikatora
       *[other] { $count } elementu segregatora nie ma identyfikatora
    }
import-scrivener-notice-items-without-identifier-body = Bez identyfikatora nie da się znaleźć plików elementu, więc trafia on do projektu pusty.
import-scrivener-notice-duplicate-item-key-title = Dwa elementy segregatora mają ten sam identyfikator: „{ $item }”
import-scrivener-notice-duplicate-item-key-body = Pierwszy zachowuje dokumenty. Drugi zachowuje swoje miejsce i trafia do projektu pusty.
import-scrivener-notice-lock-here-title = Scrivener ma ten projekt otwarty na tym komputerze
import-scrivener-notice-lock-here-body = Najpierw zamknij go w programie Scrivener, aby zapisać wszystkie zmiany.
import-scrivener-notice-lock-title = Scrivener miał ten projekt otwarty na urządzeniu „{ $host }”
import-scrivener-notice-lock-elsewhere-title = Scrivener miał ten projekt otwarty na innym komputerze
import-scrivener-notice-lock-body = Jeśli nadal jest tam otwarty, najpierw go zamknij: pliki mogą jeszcze nie zawierać najnowszych zmian. Jeśli Scrivener został zamknięty dawno temu, możesz zaimportować projekt w obecnej postaci.
import-scrivener-notice-archived-while-open-title = To archiwum utworzono, gdy Scrivener miał projekt otwarty
import-scrivener-notice-archived-while-open-body = Może w nim brakować zmian wprowadzonych po tej chwili.
import-scrivener-notice-content-missing-notes = Brakuje notatek „{ $item }”
import-scrivener-notice-content-missing-synopsis = Brakuje streszczenia „{ $item }”
import-scrivener-notice-content-missing-text = Brakuje tekstu „{ $item }”
import-scrivener-notice-content-missing-body = { $place ->
        [notes] Ich pliku nie ma w tej kopii, więc trafiają do projektu puste. Usługa synchronizacji może nadal kopiować projekt.
        [synopsis] Jego pliku nie ma w tej kopii, więc trafia do projektu puste. Usługa synchronizacji może nadal kopiować projekt.
       *[text] Jego pliku nie ma w tej kopii, więc trafia do projektu pusty. Usługa synchronizacji może nadal kopiować projekt.
    }
import-scrivener-notice-content-missing-salvaged = { $place ->
        [notes] Ich pliku nie ma w tej kopii, ale indeks wyszukiwania programu Scrivener nadal przechowywał ich słowa. Zostały zachowane jako zwykły tekst, być może ze starszej wersji.
        [synopsis] Jego pliku nie ma w tej kopii, ale indeks wyszukiwania programu Scrivener nadal przechowywał jego słowa. Zostały zachowane jako zwykły tekst, być może ze starszej wersji.
       *[text] Jego pliku nie ma w tej kopii, ale indeks wyszukiwania programu Scrivener nadal przechowywał jego słowa. Zostały zachowane jako zwykły tekst, być może ze starszej wersji.
    }
import-scrivener-notice-content-truncated-notes = Notatki „{ $item }” są puste na dysku
import-scrivener-notice-content-truncated-synopsis = Streszczenie „{ $item }” jest puste na dysku
import-scrivener-notice-content-truncated-text = Tekst „{ $item }” jest pusty na dysku
import-scrivener-notice-content-truncated-body = { $place ->
        [notes] Według programu Scrivener nie były one puste, ale ich plik nie zawiera tekstu: synchronizacja mogła zatrzymać się w połowie. Trafiają do projektu puste.
        [synopsis] Według programu Scrivener nie było ono puste, ale jego plik nie zawiera tekstu: synchronizacja mogła zatrzymać się w połowie. Trafia do projektu puste.
       *[text] Według programu Scrivener nie był on pusty, ale jego plik nie zawiera tekstu: synchronizacja mogła zatrzymać się w połowie. Trafia do projektu pusty.
    }
import-scrivener-notice-missing-media-title = Brakuje pliku „{ $item }”
import-scrivener-notice-missing-media-body = Zawartego w nim obrazu lub pliku materiałów nie ma w tej kopii.
import-scrivener-notice-sync-conflict-copies-title = { $count ->
        [one] Znaleziono { $count } kopię dokumentu z synchronizacji
        [few] Znaleziono { $count } kopie dokumentów z synchronizacji
        [many] Znaleziono { $count } kopii dokumentów z synchronizacji
       *[other] Znaleziono { $count } kopii dokumentów z synchronizacji
    }
import-scrivener-notice-sync-conflict-copies-body = { $missing ->
        [0] Kopie z synchronizacji zostają przeniesione jako odzyskane teksty, w osobnym folderze, aby można było porównać je z dokumentami.
        [one] Kopie z synchronizacji zostają przeniesione jako odzyskane teksty, w osobnym folderze. { $missing } z nich to kopia dokumentu, którego nie ma już w projekcie.
       *[other] Kopie z synchronizacji zostają przeniesione jako odzyskane teksty, w osobnym folderze. { $missing } z nich to kopie dokumentów, których nie ma już w projekcie.
    }
import-scrivener-notice-recovered-unreferenced-text-title = { $count ->
        [one] Znaleziono { $count } dokument, którego nie wymienia żaden element segregatora
        [few] Znaleziono { $count } dokumenty, których nie wymienia żaden element segregatora
        [many] Znaleziono { $count } dokumentów, których nie wymienia żaden element segregatora
       *[other] Znaleziono { $count } dokumentu, których nie wymienia żaden element segregatora
    }
import-scrivener-notice-recovered-unreferenced-text-body = Takie dokumenty zostają przeniesione jako odzyskane teksty, w osobnym folderze.
import-scrivener-notice-sync-debris-ignored-title = { $count ->
        [one] { $count } dodatkowa kopia pozostawiona przez usługę synchronizacji nie jest brana pod uwagę
        [few] { $count } dodatkowe kopie pozostawione przez usługę synchronizacji nie są brane pod uwagę
        [many] { $count } dodatkowych kopii pozostawionych przez usługę synchronizacji nie jest branych pod uwagę
       *[other] { $count } dodatkowej kopii pozostawionej przez usługę synchronizacji nie jest branej pod uwagę
    }
import-scrivener-notice-sync-debris-ignored-body = Scrivener też nie odczytuje takich kopii. Odczytywane są tylko własne pliki projektu.
import-scrivener-notice-unrecognised-files-title = { $count ->
        [one] { $count } plik, który nie należy do projektu Scrivener, nie jest brany pod uwagę
        [few] { $count } pliki, które nie należą do projektu Scrivener, nie są brane pod uwagę
        [many] { $count } plików, które nie należą do projektu Scrivener, nie jest branych pod uwagę
       *[other] { $count } pliku, które nie należą do projektu Scrivener, nie jest branych pod uwagę
    }
import-scrivener-notice-examples = Na przykład { $examples }.
import-scrivener-notice-symlink-not-followed-title = { $count ->
        [one] Skribisto nie podąża za { $count } dowiązaniem wewnątrz projektu
        [few] Skribisto nie podąża za { $count } dowiązaniami wewnątrz projektu
        [many] Skribisto nie podąża za { $count } dowiązaniami wewnątrz projektu
       *[other] Skribisto nie podąża za { $count } dowiązania wewnątrz projektu
    }
import-scrivener-notice-unreadable-files-skipped-title = { $count ->
        [one] Nie udało się odczytać { $count } pliku, którego import nie potrzebuje
        [few] Nie udało się odczytać { $count } plików, których import nie potrzebuje
        [many] Nie udało się odczytać { $count } plików, których import nie potrzebuje
       *[other] Nie udało się odczytać { $count } pliku, których import nie potrzebuje
    }
import-scrivener-notice-cloud-placeholders-skipped-title = { $count ->
        [one] { $count } plik, który jest nadal w chmurze, zostaje pominięty
        [few] { $count } pliki, które są nadal w chmurze, zostają pominięte
        [many] { $count } plików, które są nadal w chmurze, zostaje pominiętych
       *[other] { $count } pliku, które są nadal w chmurze, zostaje pominiętych
    }
import-scrivener-notice-cloud-placeholders-skipped-body = Nic, czego potrzebuje import, nie zostało w chmurze.
import-scrivener-notice-no-text-found-title = { $count ->
        [1] { $count } dokument i nie ma w nim tekstu
        [few] { $count } dokumenty i w żadnym nie ma tekstu
        [many] { $count } dokumentów i w żadnym nie ma tekstu
       *[other] { $count } dokumentu i w żadnym nie ma tekstu
    }
import-scrivener-notice-no-text-found-body = Ten projekt wygląda na pusty. Zostaje zaimportowany w obecnej postaci.
import-scrivener-notice-metadata-unreadable-title = Nie udało się odczytać „{ $member }”
import-scrivener-notice-metadata-unreadable-body = To, co Scrivener w nim przechowywał, nie zostaje przeniesione.
import-scrivener-notice-dangling-keyword-title = { $count ->
        [one] { $count } odwołanie wskazuje słowo kluczowe, którego projekt nie definiuje
        [few] { $count } odwołania wskazują słowa kluczowe, których projekt nie definiuje
        [many] { $count } odwołań wskazuje słowa kluczowe, których projekt nie definiuje
       *[other] { $count } odwołania wskazuje słowa kluczowe, których projekt nie definiuje
    }
import-scrivener-notice-dangling-label-title = { $count ->
        [one] { $count } odwołanie wskazuje etykietę, której projekt nie definiuje
        [few] { $count } odwołania wskazują etykiety, których projekt nie definiuje
        [many] { $count } odwołań wskazuje etykiety, których projekt nie definiuje
       *[other] { $count } odwołania wskazuje etykiety, których projekt nie definiuje
    }
import-scrivener-notice-dangling-status-title = { $count ->
        [one] { $count } odwołanie wskazuje status, którego projekt nie definiuje
        [few] { $count } odwołania wskazują statusy, których projekt nie definiuje
        [many] { $count } odwołań wskazuje statusy, których projekt nie definiuje
       *[other] { $count } odwołania wskazuje statusy, których projekt nie definiuje
    }
import-scrivener-notice-dangling-section-type-title = { $count ->
        [one] { $count } odwołanie wskazuje typ sekcji, którego projekt nie definiuje
        [few] { $count } odwołania wskazują typy sekcji, których projekt nie definiuje
        [many] { $count } odwołań wskazuje typy sekcji, których projekt nie definiuje
       *[other] { $count } odwołania wskazuje typy sekcji, których projekt nie definiuje
    }
import-scrivener-notice-dangling-custom-field-title = { $count ->
        [one] { $count } odwołanie wskazuje pole własnych metadanych, którego projekt nie definiuje
        [few] { $count } odwołania wskazują pola własnych metadanych, których projekt nie definiuje
        [many] { $count } odwołań wskazuje pola własnych metadanych, których projekt nie definiuje
       *[other] { $count } odwołania wskazuje pola własnych metadanych, których projekt nie definiuje
    }
import-scrivener-notice-dangling-item-title = { $count ->
        [one] { $count } odwołanie wskazuje dokument, którego projekt nie definiuje
        [few] { $count } odwołania wskazują dokumenty, których projekt nie definiuje
        [many] { $count } odwołań wskazuje dokumenty, których projekt nie definiuje
       *[other] { $count } odwołania wskazuje dokumenty, których projekt nie definiuje
    }
import-scrivener-notice-dangling-references-body = Odwołanie do czegoś, czego projekt nie definiuje, zostaje pominięte.
import-scrivener-notice-snapshots-not-imported-line = { $count ->
        [one] { $count } migawka zostaje w Twoim projekcie Scrivener: Skribisto nie przenosi migawek.
        [few] { $count } migawki zostają w Twoim projekcie Scrivener: Skribisto nie przenosi migawek.
        [many] { $count } migawek zostaje w Twoim projekcie Scrivener: Skribisto nie przenosi migawek.
       *[other] { $count } migawki zostaje w Twoim projekcie Scrivener: Skribisto nie przenosi migawek.
    }
import-scrivener-notice-writing-history-not-imported-line = { $days ->
        [one] Prowadzony przez Scrivener zapis słów napisanych każdego dnia ({ $days } dzień) nie zostaje przeniesiony. Skribisto zaczyna własne liczenie od pierwszego zapisu.
        [few] Prowadzony przez Scrivener zapis słów napisanych każdego dnia ({ $days } dni) nie zostaje przeniesiony. Skribisto zaczyna własne liczenie od pierwszego zapisu.
        [many] Prowadzony przez Scrivener zapis słów napisanych każdego dnia ({ $days } dni) nie zostaje przeniesiony. Skribisto zaczyna własne liczenie od pierwszego zapisu.
       *[other] Prowadzony przez Scrivener zapis słów napisanych każdego dnia ({ $days } dnia) nie zostaje przeniesiony. Skribisto zaczyna własne liczenie od pierwszego zapisu.
    }
import-scrivener-notice-title-placeholder-rejected-hint = Tytuł w ustawieniach projektu to symbol zastępczy z szablonu, więc zamiast niego proponowana jest nazwa projektu.
import-scrivener-notice-author-placeholder-rejected-hint = Autor w ustawieniach projektu to tekst zastępczy z szablonu, więc żaden autor nie jest proponowany.
import-scrivener-notice-title-from-project-hint-compile-settings = Ten tytuł pochodzi z ustawień kompilacji projektu. Dopóki nie ustawi się tam tytułu, Scrivener przechowuje nazwę, jaką projekt miał przy tworzeniu.
import-scrivener-notice-title-from-project-hint-template = To własny tytuł szablonu.
import-scrivener-notice-title-from-project-hint-folder-name = Ten tytuł to nazwa projektu.
import-scrivener-notice-title-from-project-hint-project-properties = Ten tytuł pochodzi z właściwości projektu.
import-scrivener-notice-author-from-project-hint = To autor podany w ustawieniach projektu. Może tam pozostać autor szablonu, a jego nazwisko pojawiłoby się na stronie tytułowej.
import-scrivener-notice-language-not-stated-hint = Projekt nie określa swojego języka, więc proponowany jest język interfejsu Skribisto. Od języka zależy sposób liczenia słów i zapisu cudzysłowów.
import-scrivener-notice-language-from-script-hint = Zaproponowano na podstawie pisma, którym napisano rękopis ({ $tag }).

## Before you import: what the import does with the binder, or leaves behind
import-scrivener-structure-script-mode-draft-line = { $script ->
        [1] { $documents ->
            [1] Jedyny dokument rękopisu jest napisany w trybie scenariusza. Skribisto zachowuje jego słowa, ale nie układ scenariusza.
           *[other] { $script } z { $documents } dokumentów rękopisu jest napisany w trybie scenariusza. Skribisto zachowuje jego słowa, ale nie układ scenariusza.
        }
        [few] { $script } z { $documents } dokumentów rękopisu są napisane w trybie scenariusza. Skribisto zachowuje ich słowa, ale nie układ scenariusza.
        [many] { $script } z { $documents } dokumentów rękopisu jest napisanych w trybie scenariusza. Skribisto zachowuje ich słowa, ale nie układ scenariusza.
       *[other] { $script } z { $documents } dokumentów rękopisu jest napisanych w trybie scenariusza. Skribisto zachowuje ich słowa, ale nie układ scenariusza.
    }
import-scrivener-structure-one-document-manuscript-line = { $count ->
        [1] Dokument rękopisu zawiera swoje rozdziały w postaci nagłówków.
        [few] { $count } dokumenty rękopisu zawierają swoje rozdziały w postaci nagłówków.
        [many] { $count } dokumentów rękopisu zawiera swoje rozdziały w postaci nagłówków.
       *[other] { $count } dokumentu rękopisu zawiera swoje rozdziały w postaci nagłówków.
    }
import-scrivener-structure-title-repeats-number-line = { $count ->
        [one] { $count } tytuł rozdziału lub części zawiera też swój numer, np. „Rozdział II” lub „Część pierwsza”, więc w eksporcie numer może pojawić się dwukrotnie.
        [few] { $count } tytuły rozdziałów lub części zawierają też swój numer, np. „Rozdział II” lub „Część pierwsza”, więc w eksporcie numer może pojawić się dwukrotnie.
        [many] { $count } tytułów rozdziałów lub części zawiera też swój numer, np. „Rozdział II” lub „Część pierwsza”, więc w eksporcie numer może pojawić się dwukrotnie.
       *[other] { $count } tytułu rozdziałów lub części zawiera też swój numer, np. „Rozdział II” lub „Część pierwsza”, więc w eksporcie numer może pojawić się dwukrotnie.
    }
import-scrivener-structure-claim-overruled-line = { $count ->
        [one] { $count } element wygląda jak książka, część lub rozdział, ale znajduje się wewnątrz elementu tego samego poziomu, więc trafia na niższy poziom.
        [few] { $count } elementy wyglądają jak książki, części lub rozdziały, ale każdy z nich znajduje się wewnątrz elementu tego samego poziomu, więc wszystkie trafiają na niższy poziom.
        [many] { $count } elementów wygląda jak książki, części lub rozdziały, ale każdy z nich znajduje się wewnątrz elementu tego samego poziomu, więc wszystkie trafiają na niższy poziom.
       *[other] { $count } elementu wygląda jak książki, części lub rozdziały, ale każdy z nich znajduje się wewnątrz elementu tego samego poziomu, więc wszystkie trafiają na niższy poziom.
    }
import-scrivener-structure-adopted-by-chapter-line = { $count ->
        [one] { $count } dokument następujący po rozdziale staje się zakończeniem tego rozdziału.
        [few] { $count } dokumenty następujące po rozdziale stają się zakończeniem tego rozdziału.
        [many] { $count } dokumentów następujących po rozdziale staje się zakończeniem tego rozdziału.
       *[other] { $count } dokumentu następującego po rozdziale staje się zakończeniem tego rozdziału.
    }
import-scrivener-structure-looks-like-manuscript-line = { $count ->
        [one] { $count } folder najwyższego poziomu umieszczony w notatkach przypomina rękopis. Możesz przenieść go do rękopisu jako kolejną książkę.
        [few] { $count } foldery najwyższego poziomu umieszczone w notatkach przypominają rękopis. Możesz przenieść je do rękopisu jako kolejne książki.
        [many] { $count } folderów najwyższego poziomu umieszczonych w notatkach przypomina rękopis. Możesz przenieść je do rękopisu jako kolejne książki.
       *[other] { $count } folderu najwyższego poziomu umieszczonego w notatkach przypomina rękopis. Możesz przenieść je do rękopisu jako kolejne książki.
    }
import-scrivener-structure-generic-sheet-title-line = { $count ->
        [one] { $count } wpis biblii opowieści nadal ma tytuł z szablonu, np. „Bohater”. Skribisto szuka tego tytułu w Twoim tekście i wymienia każde miejsce, w którym się pojawia, w widoku „W prozie”: zmień nazwę wpisu, jeśli to słowo jest częste w Twojej książce.
        [few] { $count } wpisy biblii opowieści nadal mają tytuły z szablonu, np. „Bohater”. Skribisto szuka tych tytułów w Twoim tekście i wymienia każde miejsce, w którym się pojawiają, w widoku „W prozie”: zmień nazwy tych wpisów, jeśli te słowa są częste w Twojej książce.
        [many] { $count } wpisów biblii opowieści nadal ma tytuły z szablonu, np. „Bohater”. Skribisto szuka tych tytułów w Twoim tekście i wymienia każde miejsce, w którym się pojawiają, w widoku „W prozie”: zmień nazwy tych wpisów, jeśli te słowa są częste w Twojej książce.
       *[other] { $count } wpisu biblii opowieści nadal ma tytuły z szablonu, np. „Bohater”. Skribisto szuka tych tytułów w Twoim tekście i wymienia każde miejsce, w którym się pojawiają, w widoku „W prozie”: zmień nazwy tych wpisów, jeśli te słowa są częste w Twojej książce.
    }
import-scrivener-structure-placeholder-title-line = { $count ->
        [one] { $count } tytuł zawierał tylko nazwę swojego poziomu i zostaje pusty: numerują go segregator i eksport.
        [few] { $count } tytuły zawierały tylko nazwę swojego poziomu i zostają puste: numerują je segregator i eksport.
        [many] { $count } tytułów zawierało tylko nazwę swojego poziomu i zostaje pustych: numerują je segregator i eksport.
       *[other] { $count } tytułu zawierało tylko nazwę swojego poziomu i zostaje pustych: numerują je segregator i eksport.
    }
import-scrivener-structure-untitled-item-line = { $count ->
        [one] { $count } element nie miał tytułu w programie Scrivener i pozostaje bez tytułu.
        [few] { $count } elementy nie miały tytułu w programie Scrivener i pozostają bez tytułu.
        [many] { $count } elementów nie miało tytułu w programie Scrivener i pozostaje bez tytułu.
       *[other] { $count } elementu nie miało tytułu w programie Scrivener i pozostaje bez tytułu.
    }
import-scrivener-structure-title-cleaned-line = { $count ->
        [one] { $count } tytuł zawierał podziały wierszy lub tabulatory, które zostają usunięte.
        [few] { $count } tytuły zawierały podziały wierszy lub tabulatory, które zostają usunięte.
        [many] { $count } tytułów zawierało podziały wierszy lub tabulatory, które zostają usunięte.
       *[other] { $count } tytułu zawierało podziały wierszy lub tabulatory, które zostają usunięte.
    }
import-scrivener-structure-own-text-moved-line = { $count ->
        [one] { $count } folder ma własny tekst, który zostaje przeniesiony do pierwszego dokumentu w jego wnętrzu.
        [few] { $count } foldery mają własny tekst, który zostaje przeniesiony do pierwszego dokumentu w ich wnętrzu.
        [many] { $count } folderów ma własny tekst, który zostaje przeniesiony do pierwszego dokumentu w ich wnętrzu.
       *[other] { $count } folderu ma własny tekst, który zostaje przeniesiony do pierwszego dokumentu w ich wnętrzu.
    }
import-scrivener-structure-text-with-children-line = { $count ->
        [one] { $count } dokument zawierający inne dokumenty staje się folderem.
        [few] { $count } dokumenty zawierające inne dokumenty stają się folderami.
        [many] { $count } dokumentów zawierających inne dokumenty staje się folderami.
       *[other] { $count } dokumentu zawierającego inne dokumenty staje się folderami.
    }
import-scrivener-structure-empty-folder-plain-line = { $count ->
        [one] { $count } pusty folder pozostaje zwykłym folderem, więc żaden eksport nie wydrukuje nagłówka, którego Scrivener nigdy nie drukował.
        [few] { $count } puste foldery pozostają zwykłymi folderami, więc żaden eksport nie wydrukuje nagłówka, którego Scrivener nigdy nie drukował.
        [many] { $count } pustych folderów pozostaje zwykłymi folderami, więc żaden eksport nie wydrukuje nagłówka, którego Scrivener nigdy nie drukował.
       *[other] { $count } pustego folderu pozostaje zwykłymi folderami, więc żaden eksport nie wydrukuje nagłówka, którego Scrivener nigdy nie drukował.
    }
import-scrivener-structure-book-inside-book-line = { $count ->
        [one] { $count } książka wewnątrz książki staje się częścią.
        [few] { $count } książki wewnątrz książki stają się częściami.
        [many] { $count } książek wewnątrz książki staje się częściami.
       *[other] { $count } książki wewnątrz książki staje się częściami.
    }
import-scrivener-structure-prologue-not-numbered-line = { $count ->
        [one] { $count } prolog lub epilog zostaje wyłączony z numeracji rozdziałów.
        [few] { $count } prologi lub epilogi zostają wyłączone z numeracji rozdziałów.
        [many] { $count } prologów lub epilogów zostaje wyłączonych z numeracji rozdziałów.
       *[other] { $count } prologu lub epilogu zostaje wyłączonych z numeracji rozdziałów.
    }
import-scrivener-structure-matter-inside-book-line = { $count ->
        [one] { $count } element przedtekstu lub zatekstu spoza rękopisu trafia do książki jako paratekst, wyłączony z eksportu, dopóki go nie uwzględnisz.
        [few] { $count } elementy przedtekstu lub zatekstu spoza rękopisu trafiają do książki jako paratekst, wyłączone z eksportu, dopóki ich nie uwzględnisz.
        [many] { $count } elementów przedtekstu lub zatekstu spoza rękopisu trafia do książki jako paratekst, wyłączonych z eksportu, dopóki ich nie uwzględnisz.
       *[other] { $count } elementu przedtekstu lub zatekstu spoza rękopisu trafia do książki jako paratekst, wyłączonych z eksportu, dopóki ich nie uwzględnisz.
    }
import-scrivener-structure-paratext-not-counted-line = { $documents ->
        [one] { $documents } dokument rękopisu staje się paratekstem (przedtekstem lub zatekstem), więc { $words ->
            [one] { $words } słowo przestaje się liczyć
            [few] { $words } słowa przestają się liczyć
            [many] { $words } słów przestaje się liczyć
           *[other] { $words } słowa przestaje się liczyć
        } do rękopisu.
        [few] { $documents } dokumenty rękopisu stają się paratekstem (przedtekstem lub zatekstem), więc { $words ->
            [one] { $words } słowo przestaje się liczyć
            [few] { $words } słowa przestają się liczyć
            [many] { $words } słów przestaje się liczyć
           *[other] { $words } słowa przestaje się liczyć
        } do rękopisu.
        [many] { $documents } dokumentów rękopisu staje się paratekstem (przedtekstem lub zatekstem), więc { $words ->
            [one] { $words } słowo przestaje się liczyć
            [few] { $words } słowa przestają się liczyć
            [many] { $words } słów przestaje się liczyć
           *[other] { $words } słowa przestaje się liczyć
        } do rękopisu.
       *[other] { $documents } dokumentu rękopisu staje się paratekstem (przedtekstem lub zatekstem), więc { $words ->
            [one] { $words } słowo przestaje się liczyć
            [few] { $words } słowa przestają się liczyć
            [many] { $words } słów przestaje się liczyć
           *[other] { $words } słowa przestaje się liczyć
        } do rękopisu.
    }
import-scrivener-structure-document-notes-line = { $count ->
        [one] Notatki { $count } dokumentu stają się notatką obok niego.
        [few] Notatki { $count } dokumentów stają się notatkami obok nich.
        [many] Notatki { $count } dokumentów stają się notatkami obok nich.
       *[other] Notatki { $count } dokumentu stają się notatkami obok nich.
    }
import-scrivener-structure-draft-content-as-note-line = Własny tekst folderu rękopisu staje się notatką na początku rękopisu.
import-scrivener-structure-story-bible-folder-line = { $count ->
        [one] { $count } folder postaci lub miejsc staje się znacznikiem biblii opowieści.
        [few] { $count } foldery postaci lub miejsc stają się znacznikami biblii opowieści.
        [many] { $count } folderów postaci lub miejsc staje się znacznikami biblii opowieści.
       *[other] { $count } folderu postaci lub miejsc staje się znacznikami biblii opowieści.
    }
import-scrivener-structure-templates-created-line = { $count ->
        [one] { $count } szablon dokumentu staje się szablonem notatki.
        [few] { $count } szablony dokumentów stają się szablonami notatek.
        [many] { $count } szablonów dokumentów staje się szablonami notatek.
       *[other] { $count } szablonu dokumentu staje się szablonami notatek.
    }
import-scrivener-structure-template-nesting-flattened-line = { $count ->
        [one] { $count } folder szablonów wewnątrz innego zostaje spłaszczony: każdy tekst w nim staje się szablonem.
        [few] { $count } foldery szablonów wewnątrz innych zostają spłaszczone: każdy tekst w nich staje się szablonem.
        [many] { $count } folderów szablonów wewnątrz innych zostaje spłaszczonych: każdy tekst w nich staje się szablonem.
       *[other] { $count } folderu szablonów wewnątrz innych zostaje spłaszczonych: każdy tekst w nich staje się szablonem.
    }
import-scrivener-structure-template-kept-as-note-line = { $count ->
        [one] { $count } szablon zawiera coś, czego nie może pomieścić szablon notatki, więc zostaje zachowany jako notatka.
        [few] { $count } szablony zawierają coś, czego nie może pomieścić szablon notatki, więc zostają zachowane jako notatki.
        [many] { $count } szablonów zawiera coś, czego nie może pomieścić szablon notatki, więc zostaje zachowanych jako notatki.
       *[other] { $count } szablonu zawiera coś, czego nie może pomieścić szablon notatki, więc zostaje zachowanych jako notatki.
    }
import-scrivener-structure-recovered-texts-line = { $count ->
        [one] { $count } tekst, którego nie wymieniał żaden element segregatora, zostaje przeniesiony do osobnego folderu.
        [few] { $count } teksty, których nie wymieniał żaden element segregatora, zostają przeniesione do osobnego folderu.
        [many] { $count } tekstów, których nie wymieniał żaden element segregatora, zostaje przeniesionych do osobnego folderu.
       *[other] { $count } tekstu, których nie wymieniał żaden element segregatora, zostaje przeniesionych do osobnego folderu.
    }
import-scrivener-structure-project-notes-line = { $count ->
        [one] { $count } notatka projektu zostaje przeniesiona do osobnego folderu.
        [few] { $count } notatki projektu zostają przeniesione do osobnego folderu.
        [many] { $count } notatek projektu zostaje przeniesionych do osobnego folderu.
       *[other] { $count } notatki projektu zostaje przeniesionych do osobnego folderu.
    }
import-scrivener-structure-trash-brought-across-line = { $items ->
        [1] Zawartość kosza zostaje przeniesiona: { $items } element, który możesz przywrócić z kosza.
        [few] Zawartość kosza zostaje przeniesiona: { $items } elementy, { $writing ->
            [0] wszystkie bez tekstu
           *[other] w tym { $writing } z tekstem
        }, które możesz przywrócić z kosza.
        [many] Zawartość kosza zostaje przeniesiona: { $items } elementów, { $writing ->
            [0] wszystkie bez tekstu
           *[other] w tym { $writing } z tekstem
        }, które możesz przywrócić z kosza.
       *[other] Zawartość kosza zostaje przeniesiona: { $items } elementu, { $writing ->
            [0] wszystkie bez tekstu
           *[other] w tym { $writing } z tekstem
        }, które możesz przywrócić z kosza.
    }
import-scrivener-structure-not-compiled-line = { $count ->
        [one] { $count } element, który Scrivener wyłączył z kompilacji, zostaje też wyłączony z eksportu.
        [few] { $count } elementy, które Scrivener wyłączył z kompilacji, zostają też wyłączone z eksportu.
        [many] { $count } elementów, które Scrivener wyłączył z kompilacji, zostaje też wyłączonych z eksportu.
       *[other] { $count } elementu, które Scrivener wyłączył z kompilacji, zostaje też wyłączonych z eksportu.
    }
import-scrivener-structure-media-in-draft-line = { $count ->
        [one] { $count } obraz lub plik w rękopisie staje się notatką, ponieważ książka nie może go zawierać.
        [few] { $count } obrazy lub pliki w rękopisie stają się notatkami, ponieważ książka nie może ich zawierać.
        [many] { $count } obrazów lub plików w rękopisie staje się notatkami, ponieważ książka nie może ich zawierać.
       *[other] { $count } obrazu lub pliku w rękopisie staje się notatkami, ponieważ książka nie może ich zawierać.
    }
import-scrivener-structure-left-out-line = { $count ->
        [one] { $count } folder najwyższego poziomu zostaje pominięty razem z { $items ->
            [one] { $items } elementem
            [few] { $items } elementami
            [many] { $items } elementami
           *[other] { $items } elementu
        }.
        [few] { $count } foldery najwyższego poziomu zostają pominięte razem z { $items ->
            [one] { $items } elementem
            [few] { $items } elementami
            [many] { $items } elementami
           *[other] { $items } elementu
        }.
        [many] { $count } folderów najwyższego poziomu zostaje pominiętych razem z { $items ->
            [one] { $items } elementem
            [few] { $items } elementami
            [many] { $items } elementami
           *[other] { $items } elementu
        }.
       *[other] { $count } folderu najwyższego poziomu zostaje pominiętych razem z { $items ->
            [one] { $items } elementem
            [few] { $items } elementami
            [many] { $items } elementami
           *[other] { $items } elementu
        }.
    }
import-scrivener-structure-trash-left-out-line = { $items ->
        [1] Zawartość kosza zostaje w projekcie Scrivener: { $items } element.
        [few] Zawartość kosza zostaje w projekcie Scrivener: { $items } elementy, { $writing ->
            [0] wszystkie bez tekstu
           *[other] w tym { $writing } z tekstem
        }.
        [many] Zawartość kosza zostaje w projekcie Scrivener: { $items } elementów, { $writing ->
            [0] wszystkie bez tekstu
           *[other] w tym { $writing } z tekstem
        }.
       *[other] Zawartość kosza zostaje w projekcie Scrivener: { $items } elementu, { $writing ->
            [0] wszystkie bez tekstu
           *[other] w tym { $writing } z tekstem
        }.
    }
import-scrivener-structure-card-image-left-behind-line = { $count ->
        [one] { $count } obraz karty katalogowej na elemencie rękopisu zostaje w projekcie Scrivener: trafiłby do książki.
        [few] { $count } obrazy kart katalogowych na elementach rękopisu zostają w projekcie Scrivener: trafiłyby do książki.
        [many] { $count } obrazów kart katalogowych na elementach rękopisu zostaje w projekcie Scrivener: trafiłyby do książki.
       *[other] { $count } obrazu kart katalogowych na elementach rękopisu zostaje w projekcie Scrivener: trafiłyby do książki.
    }
import-scrivener-structure-default-template-not-linked-line = { $count ->
        [one] { $count } folder miał domyślny szablon dla tworzonych w nim nowych dokumentów. W Skribisto szablon może mieć tylko znacznik biblii opowieści, więc to powiązanie nie zostaje zachowane.
        [few] { $count } foldery miały domyślny szablon dla tworzonych w nich nowych dokumentów. W Skribisto szablon może mieć tylko znacznik biblii opowieści, więc te powiązania nie zostają zachowane.
        [many] { $count } folderów miało domyślny szablon dla tworzonych w nich nowych dokumentów. W Skribisto szablon może mieć tylko znacznik biblii opowieści, więc te powiązania nie zostają zachowane.
       *[other] { $count } folderu miało domyślny szablon dla tworzonych w nich nowych dokumentów. W Skribisto szablon może mieć tylko znacznik biblii opowieści, więc te powiązania nie zostają zachowane.
    }
import-scrivener-structure-page-break-not-kept-line = { $count ->
        [one] { $count } ustawienie „Page Break Before” (podział strony przed) nie ma miejsca w Skribisto i nie zostaje zachowane.
        [few] { $count } ustawienia „Page Break Before” (podział strony przed) nie mają miejsca w Skribisto i nie zostają zachowane.
        [many] { $count } ustawień „Page Break Before” (podział strony przed) nie ma miejsca w Skribisto i nie zostaje zachowanych.
       *[other] { $count } ustawienia „Page Break Before” (podział strony przed) nie ma miejsca w Skribisto i nie zostaje zachowanych.
    }
import-scrivener-structure-compile-as-is-not-kept-line = { $count ->
        [one] { $count } ustawienie „Compile As-Is” (kompiluj bez zmian) nie ma odpowiednika w Skribisto i nie zostaje zachowane.
        [few] { $count } ustawienia „Compile As-Is” (kompiluj bez zmian) nie mają odpowiednika w Skribisto i nie zostają zachowane.
        [many] { $count } ustawień „Compile As-Is” (kompiluj bez zmian) nie ma odpowiednika w Skribisto i nie zostaje zachowanych.
       *[other] { $count } ustawienia „Compile As-Is” (kompiluj bez zmian) nie ma odpowiednika w Skribisto i nie zostaje zachowanych.
    }

## What the import did with the labels, the status list and the dates
import-scrivener-metadata-status-categories-guessed-line = { $count ->
        [one] { $count } status, którego Skribisto nie rozpoznaje, otrzymuje rodzaj według swojego miejsca na liście. Sprawdź go w Ustawienia ▸ Projekt ▸ Statusy.
        [few] { $count } statusy, których Skribisto nie rozpoznaje, otrzymują rodzaj według swojego miejsca na liście. Sprawdź je w Ustawienia ▸ Projekt ▸ Statusy.
        [many] { $count } statusów, których Skribisto nie rozpoznaje, otrzymuje rodzaj według swojego miejsca na liście. Sprawdź je w Ustawienia ▸ Projekt ▸ Statusy.
       *[other] { $count } statusu, których Skribisto nie rozpoznaje, otrzymuje rodzaj według swojego miejsca na liście. Sprawdź je w Ustawienia ▸ Projekt ▸ Statusy.
    }
import-scrivener-metadata-dates-unreadable-line = { $count ->
        [one] Nie udało się odczytać dat { $count } elementu: otrzymuje on datę importu.
        [few] Nie udało się odczytać dat { $count } elementów: otrzymują one datę importu.
        [many] Nie udało się odczytać dat { $count } elementów: otrzymują one datę importu.
       *[other] Nie udało się odczytać dat { $count } elementu: otrzymują one datę importu.
    }
import-scrivener-metadata-labels-became-tags-line = { $tags ->
        [one] Etykiety programu Scrivener stają się { $tags } znacznikiem na { $rows ->
            [one] { $rows } elemencie
            [few] { $rows } elementach
            [many] { $rows } elementach
           *[other] { $rows } elementu
        }.
        [few] Etykiety programu Scrivener stają się { $tags } znacznikami na { $rows ->
            [one] { $rows } elemencie
            [few] { $rows } elementach
            [many] { $rows } elementach
           *[other] { $rows } elementu
        }.
        [many] Etykiety programu Scrivener stają się { $tags } znacznikami na { $rows ->
            [one] { $rows } elemencie
            [few] { $rows } elementach
            [many] { $rows } elementach
           *[other] { $rows } elementu
        }.
       *[other] Etykiety programu Scrivener stają się { $tags } znacznika na { $rows ->
            [one] { $rows } elemencie
            [few] { $rows } elementach
            [many] { $rows } elementach
           *[other] { $rows } elementu
        }.
    }
import-scrivener-metadata-statuses-became-ladder-line = { $rungs ->
        [one] Lista statusów z programu Scrivener zostaje przeniesiona jako lista statusów projektu: { $rungs } status, na { $rows ->
            [one] { $rows } elemencie
            [few] { $rows } elementach
            [many] { $rows } elementach
           *[other] { $rows } elementu
        }.
        [few] Lista statusów z programu Scrivener zostaje przeniesiona jako lista statusów projektu: { $rungs } statusy, na { $rows ->
            [one] { $rows } elemencie
            [few] { $rows } elementach
            [many] { $rows } elementach
           *[other] { $rows } elementu
        }.
        [many] Lista statusów z programu Scrivener zostaje przeniesiona jako lista statusów projektu: { $rungs } statusów, na { $rows ->
            [one] { $rows } elemencie
            [few] { $rows } elementach
            [many] { $rows } elementach
           *[other] { $rows } elementu
        }.
       *[other] Lista statusów z programu Scrivener zostaje przeniesiona jako lista statusów projektu: { $rungs } statusu, na { $rows ->
            [one] { $rows } elemencie
            [few] { $rows } elementach
            [many] { $rows } elementach
           *[other] { $rows } elementu
        }.
    }
import-scrivener-metadata-ladder-seeded-line = { $count ->
        [1] Projekt nie miał listy statusów, więc zaczyna z domyślnym statusem Skribisto.
        [few] Projekt nie miał listy statusów, więc zaczyna z { $count } domyślnymi statusami Skribisto.
        [many] Projekt nie miał listy statusów, więc zaczyna z { $count } domyślnymi statusami Skribisto.
       *[other] Projekt nie miał listy statusów, więc zaczyna z { $count } domyślnego statusu Skribisto.
    }
import-scrivener-metadata-label-renamed-line = W programie Scrivener pole Etykieta nazywało się „{ $title }”. Ta nazwa znajduje się w opisie każdego znacznika.
import-scrivener-metadata-status-renamed-line = W programie Scrivener pole Status nazywało się „{ $title }”. Ta nazwa znajduje się w opisie każdego statusu.
import-scrivener-metadata-none-label-collapsed-line = { $count ->
        [one] Etykieta „{ $name }”, przechowywana obok własnej pustej etykiety programu Scrivener pod tą samą nazwą, jest odczytywana jako brak etykiety na { $count } elemencie.
        [few] Etykieta „{ $name }”, przechowywana obok własnej pustej etykiety programu Scrivener pod tą samą nazwą, jest odczytywana jako brak etykiety na { $count } elementach.
        [many] Etykieta „{ $name }”, przechowywana obok własnej pustej etykiety programu Scrivener pod tą samą nazwą, jest odczytywana jako brak etykiety na { $count } elementach.
       *[other] Etykieta „{ $name }”, przechowywana obok własnej pustej etykiety programu Scrivener pod tą samą nazwą, jest odczytywana jako brak etykiety na { $count } elementu.
    }
import-scrivener-metadata-none-status-collapsed-line = { $count ->
        [one] Status „{ $name }”, przechowywany obok własnego pustego statusu programu Scrivener pod tą samą nazwą, jest odczytywany jako brak statusu na { $count } elemencie.
        [few] Status „{ $name }”, przechowywany obok własnego pustego statusu programu Scrivener pod tą samą nazwą, jest odczytywany jako brak statusu na { $count } elementach.
        [many] Status „{ $name }”, przechowywany obok własnego pustego statusu programu Scrivener pod tą samą nazwą, jest odczytywany jako brak statusu na { $count } elementach.
       *[other] Status „{ $name }”, przechowywany obok własnego pustego statusu programu Scrivener pod tą samą nazwą, jest odczytywany jako brak statusu na { $count } elementu.
    }
import-scrivener-metadata-unnamed-label-line = Etykieta bez nazwy otrzymuje nazwę od swojego numeru, { $id }.
import-scrivener-metadata-unnamed-status-line = Status bez nazwy otrzymuje nazwę od swojego numeru, { $id }.
import-scrivener-metadata-tag-name-disambiguated-line = { $count ->
        [one] { $count } znacznik otrzymuje nową nazwę, ponieważ jego nazwę ma już inny znacznik.
        [few] { $count } znaczniki otrzymują nowe nazwy, ponieważ ich nazwy mają już inne znaczniki.
        [many] { $count } znaczników otrzymuje nowe nazwy, ponieważ ich nazwy mają już inne znaczniki.
       *[other] { $count } znacznika otrzymuje nowe nazwy, ponieważ ich nazwy mają już inne znaczniki.
    }
import-scrivener-metadata-rung-names-disambiguated-line = { $count ->
        [one] { $count } status ma taką samą nazwę jak inny, więc po nazwie otrzymuje numer. Sprawdź go w Ustawienia ▸ Projekt ▸ Statusy.
        [few] { $count } statusy mają taką samą nazwę jak inne, więc każdy otrzymuje numer po nazwie. Sprawdź je w Ustawienia ▸ Projekt ▸ Statusy.
        [many] { $count } statusów ma taką samą nazwę jak inne, więc każdy otrzymuje numer po nazwie. Sprawdź je w Ustawienia ▸ Projekt ▸ Statusy.
       *[other] { $count } statusu ma taką samą nazwę jak inne, więc każdy otrzymuje numer po nazwie. Sprawdź je w Ustawienia ▸ Projekt ▸ Statusy.
    }
import-scrivener-metadata-unused-labels-skipped-line = { $count ->
        [one] { $count } etykieta, której nie używa żaden dokument, nie zostaje zaimportowana.
        [few] { $count } etykiety, których nie używa żaden dokument, nie zostają zaimportowane.
        [many] { $count } etykiet, których nie używa żaden dokument, nie zostaje zaimportowanych.
       *[other] { $count } etykiety, których nie używa żaden dokument, nie zostaje zaimportowanych.
    }
import-scrivener-metadata-labels-without-row-line = { $count ->
        [one] Etykieta { $count } elementu, który nie ma miejsca w nowym projekcie, nie zostaje przeniesiona.
        [few] Etykiety { $count } elementów, które nie mają miejsca w nowym projekcie, nie zostają przeniesione.
        [many] Etykiety { $count } elementów, które nie mają miejsca w nowym projekcie, nie zostają przeniesione.
       *[other] Etykiety { $count } elementu, które nie mają miejsca w nowym projekcie, nie zostają przeniesione.
    }
import-scrivener-metadata-statuses-without-row-line = { $count ->
        [one] Status { $count } elementu, który nie ma miejsca w nowym projekcie, nie zostaje przeniesiony.
        [few] Statusy { $count } elementów, które nie mają miejsca w nowym projekcie, nie zostają przeniesione.
        [many] Statusy { $count } elementów, które nie mają miejsca w nowym projekcie, nie zostają przeniesione.
       *[other] Statusy { $count } elementu, które nie mają miejsca w nowym projekcie, nie zostają przeniesione.
    }

import-scrivener-metadata-unnamed-keyword-line = Słowo kluczowe bez nazwy otrzymuje nazwę od swojego numeru, { $id }.
import-scrivener-metadata-keywords-without-row-line = { $count ->
        [one] Słowa kluczowe { $count } elementu, który nie ma miejsca w nowym projekcie, nie zostają przeniesione.
        [few] Słowa kluczowe { $count } elementów, które nie mają miejsca w nowym projekcie, nie zostają przeniesione.
        [many] Słowa kluczowe { $count } elementów, które nie mają miejsca w nowym projekcie, nie zostają przeniesione.
       *[other] Słowa kluczowe { $count } elementu, które nie mają miejsca w nowym projekcie, nie zostają przeniesione.
    }
import-scrivener-metadata-custom-values-without-row-line = { $count ->
        [one] Własne metadane { $count } elementu, który nie ma miejsca w nowym projekcie, nie zostają przeniesione.
        [few] Własne metadane { $count } elementów, które nie mają miejsca w nowym projekcie, nie zostają przeniesione.
        [many] Własne metadane { $count } elementów, które nie mają miejsca w nowym projekcie, nie zostają przeniesione.
       *[other] Własne metadane { $count } elementu, które nie mają miejsca w nowym projekcie, nie zostają przeniesione.
    }
import-scrivener-metadata-bookmarks-without-row-line = { $count ->
        [one] Zakładki { $count } elementu, który nie ma miejsca w nowym projekcie, nie zostają przeniesione.
        [few] Zakładki { $count } elementów, które nie mają miejsca w nowym projekcie, nie zostają przeniesione.
        [many] Zakładki { $count } elementów, które nie mają miejsca w nowym projekcie, nie zostają przeniesione.
       *[other] Zakładki { $count } elementu, które nie mają miejsca w nowym projekcie, nie zostają przeniesione.
    }
import-scrivener-metadata-targets-without-row-line = { $count ->
        [one] Cel { $count } elementu, który nie ma miejsca w nowym projekcie, nie zostaje przeniesiony.
        [few] Cele { $count } elementów, które nie mają miejsca w nowym projekcie, nie zostają przeniesione.
        [many] Cele { $count } elementów, które nie mają miejsca w nowym projekcie, nie zostają przeniesione.
       *[other] Cele { $count } elementu, które nie mają miejsca w nowym projekcie, nie zostają przeniesione.
    }

## What the import does with the keywords, custom metadata, collections, bookmarks, targets,
## the deadline, the auto-complete list and the icons. Each line is said twice: on the Review
## page before the import, then in its report, so it reads the same at both times. `$note`
## and `$folder` are the titles the import gives a note and its "From Scrivener" folder;
## `$day` is a day as YYYY-MM-DD.
import-scrivener-metadata-pov-hint-line = { $count ->
        [1] Etykieta lub wartość pola { $values } pasuje do wpisu biblii opowieści { $matches }. Jeśli wskazuje punkt widzenia, ustaw go w inspektorze: import niczego nie łączy.
        [few] Etykiety lub wartości pól { $values } pasują do wpisów biblii opowieści { $matches }. Jeśli wskazują punkty widzenia, ustaw je w inspektorze: import niczego nie łączy.
        [many] Etykiety lub wartości pól { $values } pasują do wpisów biblii opowieści { $matches }. Jeśli wskazują punkty widzenia, ustaw je w inspektorze: import niczego nie łączy.
       *[other] Etykiety lub wartości pól { $values } pasują do wpisów biblii opowieści { $matches }. Jeśli wskazują punkty widzenia, ustaw je w inspektorze: import niczego nie łączy.
    }
import-scrivener-metadata-checkbox-default-unknown-line = Pole wyboru „{ $field }” ma wartość domyślną, której Skribisto nie potrafi odczytać, więc nie wiadomo, które dokumenty są domyślnie zaznaczone. Jego znacznik otrzymują tylko dokumenty zaznaczone pojedynczo.
import-scrivener-metadata-goals-in-other-unit-line = { $count ->
        [one] { $count } cel jest ustawiony w drugiej jednostce liczenia. Zostaje zachowany i pojawi się po zmianie jednostki liczenia w Ustawienia ▸ Projekt ▸ Struktura.
        [few] { $count } cele są ustawione w drugiej jednostce liczenia. Zostają zachowane i pojawią się po zmianie jednostki liczenia w Ustawienia ▸ Projekt ▸ Struktura.
        [many] { $count } celów jest ustawionych w drugiej jednostce liczenia. Zostają zachowane i pojawią się po zmianie jednostki liczenia w Ustawienia ▸ Projekt ▸ Struktura.
       *[other] { $count } celu jest ustawionych w drugiej jednostce liczenia. Zostają zachowane i pojawią się po zmianie jednostki liczenia w Ustawienia ▸ Projekt ▸ Struktura.
    }
import-scrivener-metadata-counting-differs-line = Scrivener liczył cel rękopisu na podstawie innych dokumentów niż Skribisto. Cel książki liczy sceny uwzględnione w eksporcie.
import-scrivener-metadata-writing-days-not-read-line = Nie można odczytać dni pisania „{ $raw }”, więc Tempo książki traktuje każdy dzień jako dzień pisania. Zmień je tam.
import-scrivener-metadata-script-mode-rows-line = { $count ->
        [one] { $count } element był w programie Scrivener w trybie scenariusza. Skribisto zachowuje jego słowa jako prozę, bez układu scenariusza.
        [few] { $count } elementy były w programie Scrivener w trybie scenariusza. Skribisto zachowuje ich słowa jako prozę, bez układu scenariusza.
        [many] { $count } elementów było w programie Scrivener w trybie scenariusza. Skribisto zachowuje ich słowa jako prozę, bez układu scenariusza.
       *[other] { $count } elementu było w programie Scrivener w trybie scenariusza. Skribisto zachowuje ich słowa jako prozę, bez układu scenariusza.
    }
import-scrivener-metadata-keywords-became-tags-line = { $tags ->
        [one] { $tags } słowo kluczowe staje się znacznikiem na { $rows ->
            [one] { $rows } elemencie
            [few] { $rows } elementach
            [many] { $rows } elementach
           *[other] { $rows } elementu
        }.
        [few] { $tags } słowa kluczowe stają się znacznikami na { $rows ->
            [one] { $rows } elemencie
            [few] { $rows } elementach
            [many] { $rows } elementach
           *[other] { $rows } elementu
        }.
        [many] { $tags } słów kluczowych staje się znacznikami na { $rows ->
            [one] { $rows } elemencie
            [few] { $rows } elementach
            [many] { $rows } elementach
           *[other] { $rows } elementu
        }.
       *[other] { $tags } słowa kluczowego staje się znacznikami na { $rows ->
            [one] { $rows } elemencie
            [few] { $rows } elementach
            [many] { $rows } elementach
           *[other] { $rows } elementu
        }.
    }
import-scrivener-metadata-keyword-hierarchy-flattened-line = { $count ->
        [one] { $count } słowo kluczowe było w programie Scrivener zagnieżdżone w innym. Znaczniki są płaskie: opis jego znacznika podaje, gdzie się znajdowało.
        [few] { $count } słowa kluczowe były w programie Scrivener zagnieżdżone w innych. Znaczniki są płaskie: opis każdego znacznika podaje, gdzie znajdowało się jego słowo kluczowe.
        [many] { $count } słów kluczowych było w programie Scrivener zagnieżdżonych w innych. Znaczniki są płaskie: opis każdego znacznika podaje, gdzie znajdowało się jego słowo kluczowe.
       *[other] { $count } słowa kluczowego było w programie Scrivener zagnieżdżonych w innych. Znaczniki są płaskie: opis każdego znacznika podaje, gdzie znajdowało się jego słowo kluczowe.
    }
import-scrivener-metadata-fields-became-tags-line = { $fields ->
        [one] { $fields } pole listy lub pole wyboru staje się { $tags ->
            [one] { $tags } znacznikiem
            [few] { $tags } znacznikami
            [many] { $tags } znacznikami
           *[other] { $tags } znacznika
        } na { $rows ->
            [one] { $rows } elemencie
            [few] { $rows } elementach
            [many] { $rows } elementach
           *[other] { $rows } elementu
        }.
        [few] { $fields } pola listy i pola wyboru stają się { $tags ->
            [one] { $tags } znacznikiem
            [few] { $tags } znacznikami
            [many] { $tags } znacznikami
           *[other] { $tags } znacznika
        } na { $rows ->
            [one] { $rows } elemencie
            [few] { $rows } elementach
            [many] { $rows } elementach
           *[other] { $rows } elementu
        }.
        [many] { $fields } pól listy i pól wyboru staje się { $tags ->
            [one] { $tags } znacznikiem
            [few] { $tags } znacznikami
            [many] { $tags } znacznikami
           *[other] { $tags } znacznika
        } na { $rows ->
            [one] { $rows } elemencie
            [few] { $rows } elementach
            [many] { $rows } elementach
           *[other] { $rows } elementu
        }.
       *[other] { $fields } pola listy i pola wyboru staje się { $tags ->
            [one] { $tags } znacznikiem
            [few] { $tags } znacznikami
            [many] { $tags } znacznikami
           *[other] { $tags } znacznika
        } na { $rows ->
            [one] { $rows } elemencie
            [few] { $rows } elementach
            [many] { $rows } elementach
           *[other] { $rows } elementu
        }.
    }
import-scrivener-metadata-collections-became-tags-line = { $tags ->
        [one] { $tags } kolekcja staje się znacznikiem na { $rows ->
            [one] { $rows } elemencie
            [few] { $rows } elementach
            [many] { $rows } elementach
           *[other] { $rows } elementu
        }.
        [few] { $tags } kolekcje stają się znacznikami na { $rows ->
            [one] { $rows } elemencie
            [few] { $rows } elementach
            [many] { $rows } elementach
           *[other] { $rows } elementu
        }.
        [many] { $tags } kolekcji staje się znacznikami na { $rows ->
            [one] { $rows } elemencie
            [few] { $rows } elementach
            [many] { $rows } elementach
           *[other] { $rows } elementu
        }.
       *[other] { $tags } kolekcji staje się znacznikami na { $rows ->
            [one] { $rows } elemencie
            [few] { $rows } elementach
            [many] { $rows } elementach
           *[other] { $rows } elementu
        }.
    }
import-scrivener-metadata-collection-order-lost-line = { $count ->
        [one] Znacznik nie przechowuje kolejności, więc kolejność dokumentów w { $count } kolekcji nie zostaje zachowana.
        [few] Znacznik nie przechowuje kolejności, więc kolejność dokumentów w { $count } kolekcjach nie zostaje zachowana.
        [many] Znacznik nie przechowuje kolejności, więc kolejność dokumentów w { $count } kolekcjach nie zostaje zachowana.
       *[other] Znacznik nie przechowuje kolejności, więc kolejność dokumentów w { $count } kolekcji nie zostaje zachowana.
    }
import-scrivener-metadata-bookmarks-became-links-line = { $count ->
        [one] { $count } zakładka staje się powiązaniem między dwiema notatkami albo między sceną a wpisem biblii opowieści: zobacz sekcje Powiązania i Obsada.
        [few] { $count } zakładki stają się powiązaniami między notatkami albo między scenami a wpisami biblii opowieści: zobacz sekcje Powiązania i Obsada.
        [many] { $count } zakładek staje się powiązaniami między notatkami albo między scenami a wpisami biblii opowieści: zobacz sekcje Powiązania i Obsada.
       *[other] { $count } zakładki staje się powiązaniami między notatkami albo między scenami a wpisami biblii opowieści: zobacz sekcje Powiązania i Obsada.
    }
import-scrivener-metadata-goals-kept-line = { $count ->
        [one] Cel, który Scrivener wyznaczył dla { $count } dokumentu, staje się jego celem w Skribisto.
        [few] Cele, które Scrivener wyznaczył dla { $count } dokumentów, stają się ich celami w Skribisto.
        [many] Cele, które Scrivener wyznaczył dla { $count } dokumentów, stają się ich celami w Skribisto.
       *[other] Cele, które Scrivener wyznaczył dla { $count } dokumentu, stają się ich celami w Skribisto.
    }
import-scrivener-metadata-draft-target-kept-words-line = { $value ->
        [one] Cel rękopisu wynoszący { $value } słowo staje się celem książki.
        [few] Cel rękopisu wynoszący { $value } słowa staje się celem książki.
        [many] Cel rękopisu wynoszący { $value } słów staje się celem książki.
       *[other] Cel rękopisu wynoszący { $value } słowa staje się celem książki.
    }
import-scrivener-metadata-draft-target-kept-characters-line = { $value ->
        [one] Cel rękopisu wynoszący { $value } znak staje się celem książki.
        [few] Cel rękopisu wynoszący { $value } znaki staje się celem książki.
        [many] Cel rękopisu wynoszący { $value } znaków staje się celem książki.
       *[other] Cel rękopisu wynoszący { $value } znaku staje się celem książki.
    }
import-scrivener-metadata-pace-planned-line = Termin, { $day }, ustawia Tempo książki, które od dnia importu rozkłada słowa na każdy dzień pisania wybrany w programie Scrivener.
import-scrivener-metadata-pace-planned-day-off-line = Termin, { $day }, ustawia Tempo książki, które od dnia importu rozkłada słowa na każdy dzień pisania wybrany w programie Scrivener. Tak jak w programie Scrivener sam dzień terminu jest dniem wolnym.
import-scrivener-metadata-dictionary-words-line = { $count ->
        [one] { $count } słowo z listy autouzupełniania trafia do słownika własnego projektu, więc nie jest oznaczane jako błędne.
        [few] { $count } słowa z listy autouzupełniania trafiają do słownika własnego projektu, więc nie są oznaczane jako błędne.
        [many] { $count } słów z listy autouzupełniania trafia do słownika własnego projektu, więc nie są oznaczane jako błędne.
       *[other] { $count } słowa z listy autouzupełniania trafia do słownika własnego projektu, więc nie są oznaczane jako błędne.
    }
import-scrivener-metadata-listed-project-line = { $count ->
        [one] { $count } informacja o projekcie jest wymieniona w notatce „{ $note }” w folderze „{ $folder }”.
        [few] { $count } informacje o projekcie są wymienione w notatce „{ $note }” w folderze „{ $folder }”.
        [many] { $count } informacji o projekcie jest wymienionych w notatce „{ $note }” w folderze „{ $folder }”.
       *[other] { $count } informacji o projekcie jest wymienionych w notatce „{ $note }” w folderze „{ $folder }”.
    }
import-scrivener-metadata-listed-project-bookmarks-line = { $count ->
        [one] { $count } zakładka projektu jest wymieniona w notatce „{ $note }” w folderze „{ $folder }”.
        [few] { $count } zakładki projektu są wymienione w notatce „{ $note }” w folderze „{ $folder }”.
        [many] { $count } zakładek projektu jest wymienionych w notatce „{ $note }” w folderze „{ $folder }”.
       *[other] { $count } zakładki projektu jest wymienionych w notatce „{ $note }” w folderze „{ $folder }”.
    }
import-scrivener-metadata-listed-replacements-line = { $count ->
        [one] { $count } zamiana przy kompilacji jest wymieniona w notatce „{ $note }” w folderze „{ $folder }”. Skribisto jej nie wykonuje.
        [few] { $count } zamiany przy kompilacji są wymienione w notatce „{ $note }” w folderze „{ $folder }”. Skribisto ich nie wykonuje.
        [many] { $count } zamian przy kompilacji jest wymienionych w notatce „{ $note }” w folderze „{ $folder }”. Skribisto ich nie wykonuje.
       *[other] { $count } zamiany przy kompilacji jest wymienionych w notatce „{ $note }” w folderze „{ $folder }”. Skribisto ich nie wykonuje.
    }
import-scrivener-metadata-listed-auto-complete-line = { $count ->
        [one] { $count } wielowyrazowy wpis autouzupełniania jest wymieniony w notatce „{ $note }” w folderze „{ $folder }”.
        [few] { $count } wielowyrazowe wpisy autouzupełniania są wymienione w notatce „{ $note }” w folderze „{ $folder }”.
        [many] { $count } wielowyrazowych wpisów autouzupełniania jest wymienionych w notatce „{ $note }” w folderze „{ $folder }”.
       *[other] { $count } wielowyrazowego wpisu autouzupełniania jest wymienionych w notatce „{ $note }” w folderze „{ $folder }”.
    }
import-scrivener-metadata-listed-saved-searches-line = { $count ->
        [one] { $count } zapisane wyszukiwanie jest wymienione w notatce „{ $note }” w folderze „{ $folder }”.
        [few] { $count } zapisane wyszukiwania są wymienione w notatce „{ $note }” w folderze „{ $folder }”.
        [many] { $count } zapisanych wyszukiwań jest wymienionych w notatce „{ $note }” w folderze „{ $folder }”.
       *[other] { $count } zapisanego wyszukiwania jest wymienionych w notatce „{ $note }” w folderze „{ $folder }”.
    }
import-scrivener-metadata-listed-custom-metadata-line = { $count ->
        [one] { $count } wartość własnych metadanych, której nie może pomieścić znacznik, jest wymieniona w notatce „{ $note }” w folderze „{ $folder }”.
        [few] { $count } wartości własnych metadanych, których nie może pomieścić znacznik, są wymienione w notatce „{ $note }” w folderze „{ $folder }”.
        [many] { $count } wartości własnych metadanych, których nie może pomieścić znacznik, jest wymienionych w notatce „{ $note }” w folderze „{ $folder }”.
       *[other] { $count } wartości własnych metadanych, których nie może pomieścić znacznik, jest wymienionych w notatce „{ $note }” w folderze „{ $folder }”.
    }
import-scrivener-metadata-listed-targets-line = { $count ->
        [one] { $count } cel dokumentu jest wymieniony w notatce „{ $note }” w folderze „{ $folder }”.
        [few] { $count } cele dokumentów są wymienione w notatce „{ $note }” w folderze „{ $folder }”.
        [many] { $count } celów dokumentów jest wymienionych w notatce „{ $note }” w folderze „{ $folder }”.
       *[other] { $count } celu dokumentów jest wymienionych w notatce „{ $note }” w folderze „{ $folder }”.
    }
import-scrivener-metadata-listed-bookmarks-line = { $count ->
        [one] { $count } zakładka, której nie może pokazać żadne powiązanie w Skribisto, jest wymieniona w notatce „{ $note }” w folderze „{ $folder }”.
        [few] { $count } zakładki, których nie może pokazać żadne powiązanie w Skribisto, są wymienione w notatce „{ $note }” w folderze „{ $folder }”.
        [many] { $count } zakładek, których nie może pokazać żadne powiązanie w Skribisto, jest wymienionych w notatce „{ $note }” w folderze „{ $folder }”.
       *[other] { $count } zakładki, których nie może pokazać żadne powiązanie w Skribisto, jest wymienionych w notatce „{ $note }” w folderze „{ $folder }”.
    }
import-scrivener-metadata-targets-on-notes-line = { $count ->
        [one] { $count } cel dokumentu jest ustawiony na notatce, której tekstu nie wlicza się do żadnego celu, więc nie zostaje zachowany jako cel w Skribisto.
        [few] { $count } cele dokumentów są ustawione na notatkach, których tekstu nie wlicza się do żadnego celu, więc nie zostają zachowane jako cele w Skribisto.
        [many] { $count } celów dokumentów jest ustawionych na notatkach, których tekstu nie wlicza się do żadnego celu, więc nie zostaje zachowanych jako cele w Skribisto.
       *[other] { $count } celu dokumentów jest ustawionych na notatkach, których tekstu nie wlicza się do żadnego celu, więc nie zostaje zachowanych jako cele w Skribisto.
    }
import-scrivener-metadata-targets-on-split-documents-line = { $count ->
        [one] { $count } cel dokumentu jest ustawiony na dokumencie dzielonym według nagłówków, którego rozdziałów żaden pojedynczy cel nie liczy razem, więc nie zostaje zachowany jako cel w Skribisto.
        [few] { $count } cele dokumentów są ustawione na dokumentach dzielonych według nagłówków, których rozdziałów żaden pojedynczy cel nie liczy razem, więc nie zostają zachowane jako cele w Skribisto.
        [many] { $count } celów dokumentów jest ustawionych na dokumentach dzielonych według nagłówków, których rozdziałów żaden pojedynczy cel nie liczy razem, więc nie zostaje zachowanych jako cele w Skribisto.
       *[other] { $count } celu dokumentów jest ustawionych na dokumentach dzielonych według nagłówków, których rozdziałów żaden pojedynczy cel nie liczy razem, więc nie zostaje zachowanych jako cele w Skribisto.
    }
import-scrivener-metadata-bookmark-ends-without-row-line = { $count ->
        [one] { $count } zakładka prowadzi do dokumentu, który nie ma miejsca w nowym projekcie, np. do szablonu dokumentu. Jest wymieniona według miejsca, w którym ten dokument znajdował się w programie Scrivener.
        [few] { $count } zakładki prowadzą do dokumentów, które nie mają miejsca w nowym projekcie, np. do szablonów dokumentów. Są wymienione według miejsc, w których te dokumenty znajdowały się w programie Scrivener.
        [many] { $count } zakładek prowadzi do dokumentów, które nie mają miejsca w nowym projekcie, np. do szablonów dokumentów. Są wymienione według miejsc, w których te dokumenty znajdowały się w programie Scrivener.
       *[other] { $count } zakładki prowadzi do dokumentów, które nie mają miejsca w nowym projekcie, np. do szablonów dokumentów. Są wymienione według miejsc, w których te dokumenty znajdowały się w programie Scrivener.
    }
import-scrivener-metadata-unused-keywords-skipped-line = { $count ->
        [one] { $count } słowo kluczowe, którego nie używa żaden dokument, nie zostaje zaimportowane.
        [few] { $count } słowa kluczowe, których nie używa żaden dokument, nie zostają zaimportowane.
        [many] { $count } słów kluczowych, których nie używa żaden dokument, nie zostaje zaimportowanych.
       *[other] { $count } słowa kluczowego, których nie używa żaden dokument, nie zostaje zaimportowanych.
    }
import-scrivener-metadata-list-value-invalid-line = { $count ->
        [one] { $count } wartość pola listy „{ $field }” nie odpowiada żadnej z jego opcji i nie zostaje zaimportowana.
        [few] { $count } wartości pola listy „{ $field }” nie odpowiadają żadnej z jego opcji i nie zostają zaimportowane.
        [many] { $count } wartości pola listy „{ $field }” nie odpowiada żadnej z jego opcji i nie zostaje zaimportowanych.
       *[other] { $count } wartości pola listy „{ $field }” nie odpowiada żadnej z jego opcji i nie zostaje zaimportowanych.
    }
import-scrivener-metadata-undefined-custom-field-line = { $count ->
        [one] { $count } wartość należy do pola własnych metadanych, którego projekt już nie definiuje i którego Scrivener też nie pokazuje. Nie zostaje zaimportowana.
        [few] { $count } wartości należą do pól własnych metadanych, których projekt już nie definiuje i których Scrivener też nie pokazuje. Nie zostają zaimportowane.
        [many] { $count } wartości należy do pól własnych metadanych, których projekt już nie definiuje i których Scrivener też nie pokazuje. Nie zostają zaimportowane.
       *[other] { $count } wartości należy do pól własnych metadanych, których projekt już nie definiuje i których Scrivener też nie pokazuje. Nie zostają zaimportowane.
    }
import-scrivener-metadata-collection-members-missing-line = { $count ->
        [one] { $count } dokument z kolekcji „{ $name }” nie ma miejsca w nowym projekcie albo zniknął z projektu Scrivener.
        [few] { $count } dokumenty z kolekcji „{ $name }” nie mają miejsca w nowym projekcie albo zniknęły z projektu Scrivener.
        [many] { $count } dokumentów z kolekcji „{ $name }” nie ma miejsca w nowym projekcie albo zniknęło z projektu Scrivener.
       *[other] { $count } dokumentu z kolekcji „{ $name }” nie ma miejsca w nowym projekcie albo zniknęło z projektu Scrivener.
    }
import-scrivener-metadata-empty-collections-skipped-line = { $count ->
        [one] { $count } kolekcja, która nie zawiera żadnego dokumentu, nie zostaje zaimportowana.
        [few] { $count } kolekcje, które nie zawierają żadnego dokumentu, nie zostają zaimportowane.
        [many] { $count } kolekcji, które nie zawierają żadnego dokumentu, nie zostaje zaimportowanych.
       *[other] { $count } kolekcji, które nie zawierają żadnego dokumentu, nie zostaje zaimportowanych.
    }
import-scrivener-metadata-bookmarks-unresolved-line = { $count ->
        [one] { $count } zakładka prowadzi do dokumentu, którego projekt Scrivener już nie zawiera, i nie zostaje zaimportowana.
        [few] { $count } zakładki prowadzą do dokumentów, których projekt Scrivener już nie zawiera, i nie zostają zaimportowane.
        [many] { $count } zakładek prowadzi do dokumentów, których projekt Scrivener już nie zawiera, i nie zostaje zaimportowanych.
       *[other] { $count } zakładki prowadzi do dokumentów, których projekt Scrivener już nie zawiera, i nie zostaje zaimportowanych.
    }
import-scrivener-metadata-targets-in-pages-line = { $count ->
        [one] { $count } cel dokumentu liczy strony, których Skribisto nie liczy, więc nie zostaje zachowany jako cel w Skribisto.
        [few] { $count } cele dokumentów liczą strony, których Skribisto nie liczy, więc nie zostają zachowane jako cele w Skribisto.
        [many] { $count } celów dokumentów liczy strony, których Skribisto nie liczy, więc nie zostaje zachowanych jako cele w Skribisto.
       *[other] { $count } celu dokumentów liczy strony, których Skribisto nie liczy, więc nie zostaje zachowanych jako cele w Skribisto.
    }
import-scrivener-metadata-targets-unreadable-line = { $count ->
        [one] { $count } cel nie zostaje zaimportowany: jego wartość nie jest liczbą, jest ujemna albo przekracza to, co może pomieścić cel.
        [few] { $count } cele nie zostają zaimportowane: ich wartości nie są liczbami, są ujemne albo przekraczają to, co może pomieścić cel.
        [many] { $count } celów nie zostaje zaimportowanych: ich wartości nie są liczbami, są ujemne albo przekraczają to, co może pomieścić cel.
       *[other] { $count } celu nie zostaje zaimportowanych: ich wartości nie są liczbami, są ujemne albo przekraczają to, co może pomieścić cel.
    }
import-scrivener-metadata-draft-target-in-pages-line = { $value ->
        [one] Cel rękopisu wynoszący { $value } stronę nie zostaje zachowany jako cel: Skribisto liczy słowa lub znaki. Jest wymieniony w notatce „{ $note }” w folderze „{ $folder }”.
        [few] Cel rękopisu wynoszący { $value } strony nie zostaje zachowany jako cel: Skribisto liczy słowa lub znaki. Jest wymieniony w notatce „{ $note }” w folderze „{ $folder }”.
        [many] Cel rękopisu wynoszący { $value } stron nie zostaje zachowany jako cel: Skribisto liczy słowa lub znaki. Jest wymieniony w notatce „{ $note }” w folderze „{ $folder }”.
       *[other] Cel rękopisu wynoszący { $value } strony nie zostaje zachowany jako cel: Skribisto liczy słowa lub znaki. Jest wymieniony w notatce „{ $note }” w folderze „{ $folder }”.
    }
import-scrivener-metadata-draft-target-no-book-words-line = { $value ->
        [one] Cel rękopisu wynoszący { $value } słowo nie zostaje zachowany jako cel, ponieważ rękopis tworzy kilka książek albo żadnej. Jest wymieniony w notatce „{ $note }” w folderze „{ $folder }”.
        [few] Cel rękopisu wynoszący { $value } słowa nie zostaje zachowany jako cel, ponieważ rękopis tworzy kilka książek albo żadnej. Jest wymieniony w notatce „{ $note }” w folderze „{ $folder }”.
        [many] Cel rękopisu wynoszący { $value } słów nie zostaje zachowany jako cel, ponieważ rękopis tworzy kilka książek albo żadnej. Jest wymieniony w notatce „{ $note }” w folderze „{ $folder }”.
       *[other] Cel rękopisu wynoszący { $value } słowa nie zostaje zachowany jako cel, ponieważ rękopis tworzy kilka książek albo żadnej. Jest wymieniony w notatce „{ $note }” w folderze „{ $folder }”.
    }
import-scrivener-metadata-draft-target-no-book-characters-line = { $value ->
        [one] Cel rękopisu wynoszący { $value } znak nie zostaje zachowany jako cel, ponieważ rękopis tworzy kilka książek albo żadnej. Jest wymieniony w notatce „{ $note }” w folderze „{ $folder }”.
        [few] Cel rękopisu wynoszący { $value } znaki nie zostaje zachowany jako cel, ponieważ rękopis tworzy kilka książek albo żadnej. Jest wymieniony w notatce „{ $note }” w folderze „{ $folder }”.
        [many] Cel rękopisu wynoszący { $value } znaków nie zostaje zachowany jako cel, ponieważ rękopis tworzy kilka książek albo żadnej. Jest wymieniony w notatce „{ $note }” w folderze „{ $folder }”.
       *[other] Cel rękopisu wynoszący { $value } znaku nie zostaje zachowany jako cel, ponieważ rękopis tworzy kilka książek albo żadnej. Jest wymieniony w notatce „{ $note }” w folderze „{ $folder }”.
    }
import-scrivener-metadata-deadline-passed-line = Termin, { $day }, już minął, więc nie ustawia Tempa książki. Jest wymieniony w notatce „{ $note }” w folderze „{ $folder }”.
import-scrivener-metadata-deadline-no-writing-day-line = Od dnia importu do terminu, { $day }, nie pozostał żaden dzień pisania, więc termin nie ustawia Tempa książki. Jest wymieniony w notatce „{ $note }” w folderze „{ $folder }”.
import-scrivener-metadata-deadline-template-line = Termin, { $day }, został zapisany w szablonie, więc nie ustawia Tempa książki. Jest wymieniony w notatce „{ $note }” w folderze „{ $folder }”.
import-scrivener-metadata-deadline-no-word-goal-line = Termin, { $day }, nie ustawia Tempa książki, ponieważ rękopis nie ma celu liczby słów, do którego można by planować. Jest wymieniony w notatce „{ $note }” w folderze „{ $folder }”.
import-scrivener-metadata-deadline-character-goal-line = Termin, { $day }, nie ustawia Tempa książki, ponieważ cel rękopisu liczy znaki, a Tempo liczy słowa. Jest wymieniony w notatce „{ $note }” w folderze „{ $folder }”.
import-scrivener-metadata-deadline-no-book-line = Termin, { $day }, nie ustawia Tempa żadnej książki, ponieważ rękopis tworzy kilka książek albo żadnej. Jest wymieniony w notatce „{ $note }” w folderze „{ $folder }”.
import-scrivener-metadata-deadline-unreadable-line = Nie można odczytać terminu, więc nie ustawia on Tempa książki. Jest wymieniony w dosłownym brzmieniu w notatce „{ $note }” w folderze „{ $folder }”.
import-scrivener-metadata-session-target-words-line = { $value ->
        [one] Cel sesji wynoszący { $value } słowo nie zostaje zaimportowany: w Skribisto cel sesji pisania to jedno ustawienie dla wszystkich projektów. Jest wymieniony w notatce „{ $note }” w folderze „{ $folder }”.
        [few] Cel sesji wynoszący { $value } słowa nie zostaje zaimportowany: w Skribisto cel sesji pisania to jedno ustawienie dla wszystkich projektów. Jest wymieniony w notatce „{ $note }” w folderze „{ $folder }”.
        [many] Cel sesji wynoszący { $value } słów nie zostaje zaimportowany: w Skribisto cel sesji pisania to jedno ustawienie dla wszystkich projektów. Jest wymieniony w notatce „{ $note }” w folderze „{ $folder }”.
       *[other] Cel sesji wynoszący { $value } słowa nie zostaje zaimportowany: w Skribisto cel sesji pisania to jedno ustawienie dla wszystkich projektów. Jest wymieniony w notatce „{ $note }” w folderze „{ $folder }”.
    }
import-scrivener-metadata-session-target-characters-line = { $value ->
        [one] Cel sesji wynoszący { $value } znak nie zostaje zaimportowany: w Skribisto cel sesji pisania to jedno ustawienie dla wszystkich projektów, wyrażone w słowach. Jest wymieniony w notatce „{ $note }” w folderze „{ $folder }”.
        [few] Cel sesji wynoszący { $value } znaki nie zostaje zaimportowany: w Skribisto cel sesji pisania to jedno ustawienie dla wszystkich projektów, wyrażone w słowach. Jest wymieniony w notatce „{ $note }” w folderze „{ $folder }”.
        [many] Cel sesji wynoszący { $value } znaków nie zostaje zaimportowany: w Skribisto cel sesji pisania to jedno ustawienie dla wszystkich projektów, wyrażone w słowach. Jest wymieniony w notatce „{ $note }” w folderze „{ $folder }”.
       *[other] Cel sesji wynoszący { $value } znaku nie zostaje zaimportowany: w Skribisto cel sesji pisania to jedno ustawienie dla wszystkich projektów, wyrażone w słowach. Jest wymieniony w notatce „{ $note }” w folderze „{ $folder }”.
    }
import-scrivener-metadata-session-target-pages-line = { $value ->
        [one] Cel sesji wynoszący { $value } stronę nie zostaje zaimportowany: w Skribisto cel sesji pisania to jedno ustawienie dla wszystkich projektów, wyrażone w słowach. Jest wymieniony w notatce „{ $note }” w folderze „{ $folder }”.
        [few] Cel sesji wynoszący { $value } strony nie zostaje zaimportowany: w Skribisto cel sesji pisania to jedno ustawienie dla wszystkich projektów, wyrażone w słowach. Jest wymieniony w notatce „{ $note }” w folderze „{ $folder }”.
        [many] Cel sesji wynoszący { $value } stron nie zostaje zaimportowany: w Skribisto cel sesji pisania to jedno ustawienie dla wszystkich projektów, wyrażone w słowach. Jest wymieniony w notatce „{ $note }” w folderze „{ $folder }”.
       *[other] Cel sesji wynoszący { $value } strony nie zostaje zaimportowany: w Skribisto cel sesji pisania to jedno ustawienie dla wszystkich projektów, wyrażone w słowach. Jest wymieniony w notatce „{ $note }” w folderze „{ $folder }”.
    }
import-scrivener-metadata-session-target-calculated-words-line = { $value ->
        [one] Cel sesji wynoszący { $value } słowo, który Scrivener obliczył na podstawie terminu i dni pisania, nie zostaje zaimportowany: w Skribisto cel sesji pisania to jedno ustawienie dla wszystkich projektów. Jest wymieniony w notatce „{ $note }” w folderze „{ $folder }”.
        [few] Cel sesji wynoszący { $value } słowa, który Scrivener obliczył na podstawie terminu i dni pisania, nie zostaje zaimportowany: w Skribisto cel sesji pisania to jedno ustawienie dla wszystkich projektów. Jest wymieniony w notatce „{ $note }” w folderze „{ $folder }”.
        [many] Cel sesji wynoszący { $value } słów, który Scrivener obliczył na podstawie terminu i dni pisania, nie zostaje zaimportowany: w Skribisto cel sesji pisania to jedno ustawienie dla wszystkich projektów. Jest wymieniony w notatce „{ $note }” w folderze „{ $folder }”.
       *[other] Cel sesji wynoszący { $value } słowa, który Scrivener obliczył na podstawie terminu i dni pisania, nie zostaje zaimportowany: w Skribisto cel sesji pisania to jedno ustawienie dla wszystkich projektów. Jest wymieniony w notatce „{ $note }” w folderze „{ $folder }”.
    }
import-scrivener-metadata-session-target-calculated-characters-line = { $value ->
        [one] Cel sesji wynoszący { $value } znak, który Scrivener obliczył na podstawie terminu i dni pisania, nie zostaje zaimportowany: w Skribisto cel sesji pisania to jedno ustawienie dla wszystkich projektów, wyrażone w słowach. Jest wymieniony w notatce „{ $note }” w folderze „{ $folder }”.
        [few] Cel sesji wynoszący { $value } znaki, który Scrivener obliczył na podstawie terminu i dni pisania, nie zostaje zaimportowany: w Skribisto cel sesji pisania to jedno ustawienie dla wszystkich projektów, wyrażone w słowach. Jest wymieniony w notatce „{ $note }” w folderze „{ $folder }”.
        [many] Cel sesji wynoszący { $value } znaków, który Scrivener obliczył na podstawie terminu i dni pisania, nie zostaje zaimportowany: w Skribisto cel sesji pisania to jedno ustawienie dla wszystkich projektów, wyrażone w słowach. Jest wymieniony w notatce „{ $note }” w folderze „{ $folder }”.
       *[other] Cel sesji wynoszący { $value } znaku, który Scrivener obliczył na podstawie terminu i dni pisania, nie zostaje zaimportowany: w Skribisto cel sesji pisania to jedno ustawienie dla wszystkich projektów, wyrażone w słowach. Jest wymieniony w notatce „{ $note }” w folderze „{ $folder }”.
    }
import-scrivener-metadata-session-target-calculated-pages-line = { $value ->
        [one] Cel sesji wynoszący { $value } stronę, który Scrivener obliczył na podstawie terminu i dni pisania, nie zostaje zaimportowany: w Skribisto cel sesji pisania to jedno ustawienie dla wszystkich projektów, wyrażone w słowach. Jest wymieniony w notatce „{ $note }” w folderze „{ $folder }”.
        [few] Cel sesji wynoszący { $value } strony, który Scrivener obliczył na podstawie terminu i dni pisania, nie zostaje zaimportowany: w Skribisto cel sesji pisania to jedno ustawienie dla wszystkich projektów, wyrażone w słowach. Jest wymieniony w notatce „{ $note }” w folderze „{ $folder }”.
        [many] Cel sesji wynoszący { $value } stron, który Scrivener obliczył na podstawie terminu i dni pisania, nie zostaje zaimportowany: w Skribisto cel sesji pisania to jedno ustawienie dla wszystkich projektów, wyrażone w słowach. Jest wymieniony w notatce „{ $note }” w folderze „{ $folder }”.
       *[other] Cel sesji wynoszący { $value } strony, który Scrivener obliczył na podstawie terminu i dni pisania, nie zostaje zaimportowany: w Skribisto cel sesji pisania to jedno ustawienie dla wszystkich projektów, wyrażone w słowach. Jest wymieniony w notatce „{ $note }” w folderze „{ $folder }”.
    }
import-scrivener-metadata-icons-dropped-line = { $count ->
        [one] { $count } element miał w segregatorze ikonę, która nie zostaje przeniesiona: Skribisto pokazuje ikonę każdego elementu na podstawie jego typu.
        [few] { $count } elementy miały w segregatorze ikony, które nie zostają przeniesione: Skribisto pokazuje ikonę każdego elementu na podstawie jego typu.
        [many] { $count } elementów miało w segregatorze ikony, które nie zostają przeniesione: Skribisto pokazuje ikonę każdego elementu na podstawie jego typu.
       *[other] { $count } elementu miało w segregatorze ikony, które nie zostają przeniesione: Skribisto pokazuje ikonę każdego elementu na podstawie jego typu.
    }

## What converting the prose did, as the import's report says it. `$count` is how many texts,
## or how many of a thing, as each line says; `$example` quotes the project.
import-scrivener-prose-unreadable-line = { $count ->
        [one] Nie udało się odczytać { $count } tekstu. Trafia do projektu pusty.
        [few] Nie udało się odczytać { $count } tekstów. Trafiają do projektu puste.
        [many] Nie udało się odczytać { $count } tekstów. Trafiają do projektu puste.
       *[other] Nie udało się odczytać { $count } tekstu. Trafiają do projektu puste.
    }
import-scrivener-prose-comments-unreadable-line = { $count ->
        [one] Nie udało się odczytać komentarzy i powiązanych przypisów { $count } tekstu. Słowa, które oznaczały, zostają zachowane.
        [few] Nie udało się odczytać komentarzy i powiązanych przypisów { $count } tekstów. Słowa, które oznaczały, zostają zachowane.
        [many] Nie udało się odczytać komentarzy i powiązanych przypisów { $count } tekstów. Słowa, które oznaczały, zostają zachowane.
       *[other] Nie udało się odczytać komentarzy i powiązanych przypisów { $count } tekstu. Słowa, które oznaczały, zostają zachowane.
    }
import-scrivener-prose-read-as-plain-text-line = { $count ->
        [one] { $count } tekst był zapisany jako zwykły tekst, a nie tekst sformatowany, i tak jest odczytywany.
        [few] { $count } teksty były zapisane jako zwykły tekst, a nie tekst sformatowany, i tak są odczytywane.
        [many] { $count } tekstów było zapisanych jako zwykły tekst, a nie tekst sformatowany, i tak są odczytywane.
       *[other] { $count } tekstu było zapisanych jako zwykły tekst, a nie tekst sformatowany, i tak są odczytywane.
    }
import-scrivener-prose-inline-note-unmatched-line = { $count ->
        [one] { $count } oznaczenie notatki w tekście nie ma pary i zostaje zachowane jako tekst.
        [few] { $count } oznaczenia notatek w tekście nie mają pary i zostają zachowane jako tekst.
        [many] { $count } oznaczeń notatek w tekście nie ma pary i zostaje zachowanych jako tekst.
       *[other] { $count } oznaczenia notatek w tekście nie ma pary i zostaje zachowanych jako tekst.
    }
import-scrivener-prose-style-unresolved-line = { $count ->
        [one] { $count } oznaczenie stylu wskazuje styl, którego projekt nie definiuje. Zostaje usunięte, a tekst zachowuje swoje formatowanie.
        [few] { $count } oznaczenia stylów wskazują style, których projekt nie definiuje. Zostają usunięte, a tekst zachowuje swoje formatowanie.
        [many] { $count } oznaczeń stylów wskazuje style, których projekt nie definiuje. Zostają usunięte, a tekst zachowuje swoje formatowanie.
       *[other] { $count } oznaczenia stylów wskazuje style, których projekt nie definiuje. Zostają usunięte, a tekst zachowuje swoje formatowanie.
    }
import-scrivener-prose-styles-lost-in-export-line = { $count ->
        [one] { $count } akapit może być nagłówkiem, który stracił swój styl, gdy Scrivener 3 wyeksportował projekt do starszego formatu, więc zostaje przeniesiony jako zwykły akapit. Sprawdź go albo zaimportuj projekt, z którego został wyeksportowany.
        [few] { $count } akapity mogą być nagłówkami, które straciły swój styl, gdy Scrivener 3 wyeksportował projekt do starszego formatu, więc zostają przeniesione jako zwykłe akapity. Sprawdź je albo zaimportuj projekt, z którego zostały wyeksportowane.
        [many] { $count } akapitów może być nagłówkami, które straciły swój styl, gdy Scrivener 3 wyeksportował projekt do starszego formatu, więc zostaje przeniesionych jako zwykłe akapity. Sprawdź je albo zaimportuj projekt, z którego zostały wyeksportowane.
       *[other] { $count } akapitu może być nagłówkami, które straciły swój styl, gdy Scrivener 3 wyeksportował projekt do starszego formatu, więc zostaje przeniesionych jako zwykłe akapity. Sprawdź je albo zaimportuj projekt, z którego zostały wyeksportowane.
    }
import-scrivener-prose-comment-anchor-without-entry-line = { $count ->
        [one] { $count } komentarz lub przypis utracił w programie Scrivener swoją treść. Słowa, które oznaczał, zostają zachowane, ale bez niego.
        [few] { $count } komentarze lub przypisy utraciły w programie Scrivener swoją treść. Słowa, które oznaczały, zostają zachowane, ale bez nich.
        [many] { $count } komentarzy lub przypisów utraciło w programie Scrivener swoją treść. Słowa, które oznaczały, zostają zachowane, ale bez nich.
       *[other] { $count } komentarza lub przypisu utraciło w programie Scrivener swoją treść. Słowa, które oznaczały, zostają zachowane, ale bez nich.
    }
import-scrivener-prose-comment-entry-without-anchor-line = { $count ->
        [one] { $count } komentarz nie miał już miejsca w swoim tekście i zostaje przypięty do pierwszego akapitu.
        [few] { $count } komentarze nie miały już miejsca w swoich tekstach i zostają przypięte do pierwszego akapitu.
        [many] { $count } komentarzy nie miało już miejsca w swoich tekstach i zostaje przypiętych do pierwszego akapitu.
       *[other] { $count } komentarza nie miało już miejsca w swoich tekstach i zostaje przypiętych do pierwszego akapitu.
    }
import-scrivener-prose-footnote-entry-without-anchor-line = { $count ->
        [one] { $count } powiązany przypis nie miał już miejsca w swoim tekście, więc zostaje przywołany na końcu tego tekstu.
        [few] { $count } powiązane przypisy nie miały już miejsca w swoich tekstach, więc każdy zostaje przywołany na końcu swojego tekstu.
        [many] { $count } powiązanych przypisów nie miało już miejsca w swoich tekstach, więc każdy zostaje przywołany na końcu swojego tekstu.
       *[other] { $count } powiązanego przypisu nie miało już miejsca w swoich tekstach, więc każdy zostaje przywołany na końcu swojego tekstu.
    }
import-scrivener-prose-range-comment-reading-line = { $count ->
        [one] { $count } komentarz ze starszej wersji programu Scrivener został umieszczony na podstawie innego odczytania jego pozycji. Sprawdź, czy dotyczy właściwych słów.
        [few] { $count } komentarze ze starszej wersji programu Scrivener zostały umieszczone na podstawie innego odczytania ich pozycji. Sprawdź, czy dotyczą właściwych słów.
        [many] { $count } komentarzy ze starszej wersji programu Scrivener zostało umieszczonych na podstawie innego odczytania ich pozycji. Sprawdź, czy dotyczą właściwych słów.
       *[other] { $count } komentarza ze starszej wersji programu Scrivener zostało umieszczonych na podstawie innego odczytania ich pozycji. Sprawdź, czy dotyczą właściwych słów.
    }
import-scrivener-prose-range-comment-reading-markers-counted-line = { $count ->
        [one] { $count } komentarz ze starszej wersji programu Scrivener został umieszczony po policzeniu oznaczeń jego notatek w tekście. Sprawdź, czy dotyczy właściwych słów.
        [few] { $count } komentarze ze starszej wersji programu Scrivener zostały umieszczone po policzeniu oznaczeń ich notatek w tekście. Sprawdź, czy dotyczą właściwych słów.
        [many] { $count } komentarzy ze starszej wersji programu Scrivener zostało umieszczonych po policzeniu oznaczeń ich notatek w tekście. Sprawdź, czy dotyczą właściwych słów.
       *[other] { $count } komentarza ze starszej wersji programu Scrivener zostało umieszczonych po policzeniu oznaczeń ich notatek w tekście. Sprawdź, czy dotyczą właściwych słów.
    }
import-scrivener-prose-range-comment-reading-pictures-not-counted-line = { $count ->
        [one] { $count } komentarz ze starszej wersji programu Scrivener został umieszczony z pominięciem jego obrazów. Sprawdź, czy dotyczy właściwych słów.
        [few] { $count } komentarze ze starszej wersji programu Scrivener zostały umieszczone z pominięciem ich obrazów. Sprawdź, czy dotyczą właściwych słów.
        [many] { $count } komentarzy ze starszej wersji programu Scrivener zostało umieszczonych z pominięciem ich obrazów. Sprawdź, czy dotyczą właściwych słów.
       *[other] { $count } komentarza ze starszej wersji programu Scrivener zostało umieszczonych z pominięciem ich obrazów. Sprawdź, czy dotyczą właściwych słów.
    }
import-scrivener-prose-range-comment-approximate-line = { $count ->
        [one] { $count } komentarz ze starszej wersji programu Scrivener nie pasuje dokładnie do żadnego odczytania swojej pozycji i może być przesunięty o kilka znaków.
        [few] { $count } komentarze ze starszej wersji programu Scrivener nie pasują dokładnie do żadnego odczytania swojej pozycji i mogą być przesunięte o kilka znaków.
        [many] { $count } komentarzy ze starszej wersji programu Scrivener nie pasuje dokładnie do żadnego odczytania swojej pozycji i może być przesuniętych o kilka znaków.
       *[other] { $count } komentarza ze starszej wersji programu Scrivener nie pasuje dokładnie do żadnego odczytania swojej pozycji i może być przesuniętych o kilka znaków.
    }
import-scrivener-prose-document-link-unresolved-line = { $count ->
        [one] { $count } odnośnik wskazuje dokument, którego import nie przeniósł. Jego słowa zostają.
        [few] { $count } odnośniki wskazują dokumenty, których import nie przeniósł. Ich słowa zostają.
        [many] { $count } odnośników wskazuje dokumenty, których import nie przeniósł. Ich słowa zostają.
       *[other] { $count } odnośnika wskazuje dokumenty, których import nie przeniósł. Ich słowa zostają.
    }
import-scrivener-prose-linked-picture-not-read-line = { $count ->
        [1] { $count } obraz powiązany z plikiem na Twoim dysku, „{ $example }”, nie został przeniesiony.
        [few] { $count } obrazy powiązane z plikami na Twoim dysku, np. „{ $example }”, nie zostały przeniesione.
        [many] { $count } obrazów powiązanych z plikami na Twoim dysku, np. „{ $example }”, nie zostało przeniesionych.
       *[other] { $count } obrazu powiązanego z plikami na Twoim dysku, np. „{ $example }”, nie zostało przeniesionych.
    }
import-scrivener-prose-heading-repeats-title-line = { $count ->
        [one] { $count } tekst zaczyna się od nagłówka, który powtarza jego tytuł, więc w eksporcie tytuł może pojawić się dwukrotnie.
        [few] { $count } teksty zaczynają się od nagłówka, który powtarza ich tytuł, więc w eksporcie tytuł może pojawić się dwukrotnie.
        [many] { $count } tekstów zaczyna się od nagłówka, który powtarza ich tytuł, więc w eksporcie tytuł może pojawić się dwukrotnie.
       *[other] { $count } tekstu zaczyna się od nagłówka, który powtarza ich tytuł, więc w eksporcie tytuł może pojawić się dwukrotnie.
    }
import-scrivener-prose-markdown-as-text-line = { $count ->
        [one] { $count } akapit napisany w Markdown zostaje zachowany dosłownie jako tekst.
        [few] { $count } akapity napisane w Markdown zostają zachowane dosłownie jako tekst.
        [many] { $count } akapitów napisanych w Markdown zostaje zachowanych dosłownie jako tekst.
       *[other] { $count } akapitu napisanego w Markdown zostaje zachowanych dosłownie jako tekst.
    }
import-scrivener-prose-citation-keys-as-text-line = { $count ->
        [one] { $count } akapit zawiera klucze cytowań dla programu bibliograficznego, zachowane jako tekst.
        [few] { $count } akapity zawierają klucze cytowań dla programu bibliograficznego, zachowane jako tekst.
        [many] { $count } akapitów zawiera klucze cytowań dla programu bibliograficznego, zachowane jako tekst.
       *[other] { $count } akapitu zawiera klucze cytowań dla programu bibliograficznego, zachowane jako tekst.
    }
import-scrivener-prose-placeholders-in-exported-text-line = { $count ->
        [1] { $count } symbol zastępczy programu Scrivener, { $example }, zostaje zachowany jako zwykły tekst w tym, co eksportujesz. Zastąp go ręcznie.
        [few] { $count } symbole zastępcze programu Scrivener, np. { $example }, zostają zachowane jako zwykły tekst w tym, co eksportujesz. Zastąp je ręcznie.
        [many] { $count } symboli zastępczych programu Scrivener, np. { $example }, zostaje zachowanych jako zwykły tekst w tym, co eksportujesz. Zastąp je ręcznie.
       *[other] { $count } symbolu zastępczego programu Scrivener, np. { $example }, zostaje zachowanych jako zwykły tekst w tym, co eksportujesz. Zastąp je ręcznie.
    }
import-scrivener-prose-placeholders-in-other-text-line = { $count ->
        [1] { $count } symbol zastępczy programu Scrivener, { $example }, zostaje zachowany jako zwykły tekst w tekście, który nie jest eksportowany.
        [few] { $count } symbole zastępcze programu Scrivener, np. { $example }, zostają zachowane jako zwykły tekst w tekstach, które nie są eksportowane.
        [many] { $count } symboli zastępczych programu Scrivener, np. { $example }, zostaje zachowanych jako zwykły tekst w tekstach, które nie są eksportowane.
       *[other] { $count } symbolu zastępczego programu Scrivener, np. { $example }, zostaje zachowanych jako zwykły tekst w tekstach, które nie są eksportowane.
    }
import-scrivener-prose-ornament-lines-kept-line = { $count ->
        [one] { $count } wyśrodkowany wiersz z ozdobnikami może oznaczać separator sceny i zostaje zachowany w obecnej postaci.
        [few] { $count } wyśrodkowane wiersze z ozdobnikami mogą oznaczać separatory scen i zostają zachowane w obecnej postaci.
        [many] { $count } wyśrodkowanych wierszy z ozdobnikami może oznaczać separatory scen i zostaje zachowanych w obecnej postaci.
       *[other] { $count } wyśrodkowanego wiersza z ozdobnikami może oznaczać separatory scen i zostaje zachowanych w obecnej postaci.
    }
import-scrivener-prose-long-document-line = { $count ->
        [one] { $count } tekst jest bardzo długi i może się wolno otwierać.
        [few] { $count } teksty są bardzo długie i mogą się wolno otwierać.
        [many] { $count } tekstów jest bardzo długich i może się wolno otwierać.
       *[other] { $count } tekstu jest bardzo długich i może się wolno otwierać.
    }
import-scrivener-prose-pictures-without-description-line = { $count ->
        [one] { $count } obraz nie ma opisu dla czytelników, którzy go nie widzą. Możesz dodać opis w edytorze.
        [few] { $count } obrazy nie mają opisu dla czytelników, którzy ich nie widzą. Możesz dodać opis w edytorze.
        [many] { $count } obrazów nie ma opisu dla czytelników, którzy ich nie widzą. Możesz dodać opis w edytorze.
       *[other] { $count } obrazu nie ma opisu dla czytelników, którzy ich nie widzą. Możesz dodać opis w edytorze.
    }
import-scrivener-prose-struck-text-compiled-away-line = { $count ->
        [one] Przekreślony tekst w { $count } tekście zostaje zachowany. Kompilacja w programie Scrivener pomijała go w książce, ale eksport ze Skribisto go uwzględnia.
        [few] Przekreślony tekst w { $count } tekstach zostaje zachowany. Kompilacja w programie Scrivener pomijała go w książce, ale eksport ze Skribisto go uwzględnia.
        [many] Przekreślony tekst w { $count } tekstach zostaje zachowany. Kompilacja w programie Scrivener pomijała go w książce, ale eksport ze Skribisto go uwzględnia.
       *[other] Przekreślony tekst w { $count } tekstu zostaje zachowany. Kompilacja w programie Scrivener pomijała go w książce, ale eksport ze Skribisto go uwzględnia.
    }
import-scrivener-prose-compile-deleted-style-printed-line = { $count ->
        [1] Tekst w stylu „{ $example }”, który kompilacja w programie Scrivener usuwała, zostaje zachowany jako proza, a eksport ze Skribisto go uwzględnia.
        [few] Tekst w { $count } stylach, które kompilacja w programie Scrivener usuwała, np. „{ $example }”, zostaje zachowany jako proza, a eksport ze Skribisto go uwzględnia.
        [many] Tekst w { $count } stylach, które kompilacja w programie Scrivener usuwała, np. „{ $example }”, zostaje zachowany jako proza, a eksport ze Skribisto go uwzględnia.
       *[other] Tekst w { $count } stylu, które kompilacja w programie Scrivener usuwała, np. „{ $example }”, zostaje zachowany jako proza, a eksport ze Skribisto go uwzględnia.
    }
import-scrivener-prose-compile-raw-style-printed-line = { $count ->
        [1] Tekst w stylu „{ $example }”, który kompilacja w programie Scrivener przekazywała jako kod dla innego programu, zostaje zachowany jako proza, a eksport ze Skribisto go uwzględnia.
        [few] Tekst w { $count } stylach, które kompilacja w programie Scrivener przekazywała jako kod dla innego programu, np. „{ $example }”, zostaje zachowany jako proza, a eksport ze Skribisto go uwzględnia.
        [many] Tekst w { $count } stylach, które kompilacja w programie Scrivener przekazywała jako kod dla innego programu, np. „{ $example }”, zostaje zachowany jako proza, a eksport ze Skribisto go uwzględnia.
       *[other] Tekst w { $count } stylu, które kompilacja w programie Scrivener przekazywała jako kod dla innego programu, np. „{ $example }”, zostaje zachowany jako proza, a eksport ze Skribisto go uwzględnia.
    }
import-scrivener-prose-left-out-after-document-line = { $count ->
        [one] { $count } fragment uszkodzonego pliku, leżący za jego końcem, zostaje pominięty.
        [few] { $count } fragmenty uszkodzonych plików, leżące za ich końcem, zostają pominięte.
        [many] { $count } fragmentów uszkodzonych plików, leżących za ich końcem, zostaje pominiętych.
       *[other] { $count } fragmentu uszkodzonych plików, leżącego za ich końcem, zostaje pominiętych.
    }
import-scrivener-prose-left-out-header-or-footer-line = { $count ->
        [one] Skribisto pomija { $count } nagłówek lub stopkę strony: nie są częścią tekstu.
        [few] Skribisto pomija { $count } nagłówki i stopki stron: nie są częścią tekstu.
        [many] Skribisto pomija { $count } nagłówków i stopek stron: nie są częścią tekstu.
       *[other] Skribisto pomija { $count } nagłówka lub stopki stron: nie są częścią tekstu.
    }
import-scrivener-prose-left-out-list-label-picture-line = { $count ->
        [one] { $count } obraz użyty jako punktor listy zostaje pominięty.
        [few] { $count } obrazy użyte jako punktory list zostają pominięte.
        [many] { $count } obrazów użytych jako punktory list zostaje pominiętych.
       *[other] { $count } obrazu użytego jako punktory list zostaje pominiętych.
    }
import-scrivener-prose-left-out-line = { $count ->
        [one] { $count } fragment pliku, który nie należy do żadnego tekstu, zostaje pominięty.
        [few] { $count } fragmenty plików, które nie należą do żadnego tekstu, zostają pominięte.
        [many] { $count } fragmentów plików, które nie należą do żadnego tekstu, zostaje pominiętych.
       *[other] { $count } fragmentu plików, które nie należą do żadnego tekstu, zostaje pominiętych.
    }
import-scrivener-prose-undecodable-characters-line = { $count ->
        [one] Nie udało się zdekodować { $count } znaku i w jego miejscu pojawia się znak zastępczy (�).
        [few] Nie udało się zdekodować { $count } znaków i w ich miejscu pojawiają się znaki zastępcze (�).
        [many] Nie udało się zdekodować { $count } znaków i w ich miejscu pojawiają się znaki zastępcze (�).
       *[other] Nie udało się zdekodować { $count } znaku i w ich miejscu pojawiają się znaki zastępcze (�).
    }
import-scrivener-prose-inline-notes-merged-line = { $count ->
        [one] { $count } notatka w tekście, którą Scrivener podzielił w miejscu zmiany formatowania, zostaje z powrotem połączona.
        [few] { $count } notatki w tekście, które Scrivener podzielił w miejscach zmiany formatowania, zostają z powrotem połączone.
        [many] { $count } notatek w tekście, które Scrivener podzielił w miejscach zmiany formatowania, zostaje z powrotem połączonych.
       *[other] { $count } notatki w tekście, które Scrivener podzielił w miejscach zmiany formatowania, zostaje z powrotem połączonych.
    }
import-scrivener-prose-inline-notes-joined-across-paragraphs-line = { $count ->
        [1] { $count } notatka w tekście, pisana akapit po akapicie, zostaje połączona w jedną notatkę.
        [few] { $count } notatki w tekście, pisane akapit po akapicie, zostają połączone w notatki złożone z kilku akapitów.
        [many] { $count } notatek w tekście, pisanych akapit po akapicie, zostaje połączonych w notatki złożone z kilku akapitów.
       *[other] { $count } notatki w tekście, pisanej akapit po akapicie, zostaje połączonych w notatki złożone z kilku akapitów.
    }
import-scrivener-prose-blank-notes-dropped-line = { $count ->
        [one] { $count } pusta notatka w tekście zostaje pominięta.
        [few] { $count } puste notatki w tekście zostają pominięte.
        [many] { $count } pustych notatek w tekście zostaje pominiętych.
       *[other] { $count } pustej notatki w tekście zostaje pominiętych.
    }
import-scrivener-prose-annotations-became-comments-line = { $count ->
        [one] { $count } adnotacja w tekście stała się komentarzem.
        [few] { $count } adnotacje w tekście stały się komentarzami.
        [many] { $count } adnotacji w tekście stało się komentarzami.
       *[other] { $count } adnotacji w tekście stało się komentarzami.
    }
import-scrivener-prose-annotation-only-document-kept-line = { $count ->
        [one] { $count } dokument zawierał tylko adnotację. Jego słowa zostają zachowane jako proza.
        [few] { $count } dokumenty zawierały tylko adnotację. Ich słowa zostają zachowane jako proza.
        [many] { $count } dokumentów zawierało tylko adnotację. Ich słowa zostają zachowane jako proza.
       *[other] { $count } dokumentu zawierało tylko adnotację. Ich słowa zostają zachowane jako proza.
    }
import-scrivener-prose-preserve-formatting-passages-line = { $count ->
        [one] { $count } fragment oznaczony jako „Preserve Formatting” (zachowaj formatowanie) trafia do projektu jako zwykły tekst.
        [few] { $count } fragmenty oznaczone jako „Preserve Formatting” (zachowaj formatowanie) trafiają do projektu jako zwykły tekst.
        [many] { $count } fragmentów oznaczonych jako „Preserve Formatting” (zachowaj formatowanie) trafia do projektu jako zwykły tekst.
       *[other] { $count } fragmentu oznaczonego jako „Preserve Formatting” (zachowaj formatowanie) trafia do projektu jako zwykły tekst.
    }
import-scrivener-prose-nested-note-unwrapped-line = { $count ->
        [one] { $count } notatka wewnątrz komentarza lub przypisu zostaje zachowana jako tekst w miejscu, w którym była.
        [few] { $count } notatki wewnątrz komentarzy lub przypisów zostają zachowane jako tekst w miejscach, w których były.
        [many] { $count } notatek wewnątrz komentarzy lub przypisów zostaje zachowanych jako tekst w miejscach, w których były.
       *[other] { $count } notatki wewnątrz komentarzy lub przypisów zostaje zachowanych jako tekst w miejscach, w których były.
    }
import-scrivener-prose-style-not-carried-line = { $count ->
        [1] Styl „{ $example }” nie ma odpowiednika w Skribisto. Jego tekst zachowuje swoje słowa.
        [few] { $count } style, np. „{ $example }”, nie mają odpowiednika w Skribisto. Ich tekst zachowuje swoje słowa.
        [many] { $count } stylów, np. „{ $example }”, nie ma odpowiednika w Skribisto. Ich tekst zachowuje swoje słowa.
       *[other] { $count } stylu, np. „{ $example }”, nie ma odpowiednika w Skribisto. Ich tekst zachowuje swoje słowa.
    }
import-scrivener-prose-document-links-dropped-line = { $count ->
        [one] { $count } odnośnik do innego dokumentu nie ma miejsca w Skribisto. Jego słowa zostają.
        [few] { $count } odnośniki do innych dokumentów nie mają miejsca w Skribisto. Ich słowa zostają.
        [many] { $count } odnośników do innych dokumentów nie ma miejsca w Skribisto. Ich słowa zostają.
       *[other] { $count } odnośnika do innych dokumentów nie ma miejsca w Skribisto. Ich słowa zostają.
    }
import-scrivener-prose-document-links-kept-line = { $count ->
        [one] { $count } odnośnik do innego dokumentu zostaje zachowany jako odwołanie.
        [few] { $count } odnośniki do innych dokumentów zostają zachowane jako odwołania.
        [many] { $count } odnośników do innych dokumentów zostaje zachowanych jako odwołania.
       *[other] { $count } odnośnika do innych dokumentów zostaje zachowanych jako odwołania.
    }
import-scrivener-prose-other-links-kept-as-text-line = { $count ->
        [one] { $count } odnośnik do innego projektu lub pliku zostaje zachowany jako tekst.
        [few] { $count } odnośniki do innych projektów lub plików zostają zachowane jako tekst.
        [many] { $count } odnośników do innych projektów lub plików zostaje zachowanych jako tekst.
       *[other] { $count } odnośnika do innych projektów lub plików zostaje zachowanych jako tekst.
    }
import-scrivener-prose-soft-breaks-split-line = { $count ->
        [one] { $count } podział wiersza wewnątrz akapitu rozpoczyna teraz osobny akapit.
        [few] { $count } podziały wierszy wewnątrz akapitów rozpoczynają teraz osobne akapity.
        [many] { $count } podziałów wierszy wewnątrz akapitów rozpoczyna teraz osobne akapity.
       *[other] { $count } podziału wierszy wewnątrz akapitów rozpoczyna teraz osobne akapity.
    }
import-scrivener-prose-typed-indents-removed-line = { $count ->
        [one] { $count } wcięcie wpisane na początku akapitu zostaje usunięte.
        [few] { $count } wcięcia wpisane na początku akapitów zostają usunięte.
        [many] { $count } wcięć wpisanych na początku akapitów zostaje usuniętych.
       *[other] { $count } wcięcia wpisanego na początku akapitów zostaje usuniętych.
    }
import-scrivener-prose-left-indents-not-kept-line = { $count ->
        [one] { $count } akapit wcięty od marginesu traci swoje wcięcie.
        [few] { $count } akapity wcięte od marginesu tracą swoje wcięcie.
        [many] { $count } akapitów wciętych od marginesu traci swoje wcięcie.
       *[other] { $count } akapitu wciętego od marginesu traci swoje wcięcie.
    }
import-scrivener-prose-blank-lines-removed-line = { $count ->
        [one] { $count } pusty akapit między dwoma innymi zostaje usunięty.
        [few] { $count } puste akapity między innymi zostają usunięte.
        [many] { $count } pustych akapitów między innymi zostaje usuniętych.
       *[other] { $count } pustego akapitu między innymi zostaje usuniętych.
    }
import-scrivener-prose-verse-stanza-gaps-lost-line = { $count ->
        [one] { $count } pusty wiersz między strofami zostaje usunięty, więc strofy łączą się ze sobą.
        [few] { $count } puste wiersze między strofami zostają usunięte, więc strofy łączą się ze sobą.
        [many] { $count } pustych wierszy między strofami zostaje usuniętych, więc strofy łączą się ze sobą.
       *[other] { $count } pustego wiersza między strofami zostaje usuniętych, więc strofy łączą się ze sobą.
    }
import-scrivener-prose-blank-lines-became-scene-breaks-line = { $count ->
        [one] { $count } pusty wiersz między akapitami stał się separatorem sceny.
        [few] { $count } puste wiersze między akapitami stały się separatorami scen.
        [many] { $count } pustych wierszy między akapitami stało się separatorami scen.
       *[other] { $count } pustego wiersza między akapitami stało się separatorami scen.
    }
import-scrivener-prose-ornaments-became-scene-breaks-line = { $count ->
        [one] { $count } wiersz z ozdobnikami stał się separatorem sceny.
        [few] { $count } wiersze z ozdobnikami stały się separatorami scen.
        [many] { $count } wierszy z ozdobnikami stało się separatorami scen.
       *[other] { $count } wiersza z ozdobnikami stało się separatorami scen.
    }
import-scrivener-prose-table-first-row-header-line = { $count ->
        [one] { $count } tabela zaczyna się teraz od wiersza nagłówka.
        [few] { $count } tabele zaczynają się teraz od wiersza nagłówka.
        [many] { $count } tabel zaczyna się teraz od wiersza nagłówka.
       *[other] { $count } tabeli zaczyna się teraz od wiersza nagłówka.
    }
import-scrivener-prose-table-cells-joined-line = { $count ->
        [one] { $count } komórka tabeli zawierała kilka akapitów, teraz połączonych w jeden.
        [few] { $count } komórki tabel zawierały kilka akapitów, teraz połączonych w jeden.
        [many] { $count } komórek tabel zawierało kilka akapitów, teraz połączonych w jeden.
       *[other] { $count } komórki tabel zawierało kilka akapitów, teraz połączonych w jeden.
    }
import-scrivener-prose-formatting-lost-colour-line = { $count ->
        [one] Kolor tekstu zostaje usunięty w { $count } tekście.
        [few] Kolor tekstu zostaje usunięty w { $count } tekstach.
        [many] Kolor tekstu zostaje usunięty w { $count } tekstach.
       *[other] Kolor tekstu zostaje usunięty w { $count } tekstu.
    }
import-scrivener-prose-formatting-lost-highlight-line = { $count ->
        [one] Zakreślenie zostaje usunięte w { $count } tekście.
        [few] Zakreślenie zostaje usunięte w { $count } tekstach.
        [many] Zakreślenie zostaje usunięte w { $count } tekstach.
       *[other] Zakreślenie zostaje usunięte w { $count } tekstu.
    }
import-scrivener-prose-formatting-lost-small-caps-line = { $count ->
        [one] Kapitaliki są wyświetlane jako zwykłe litery w { $count } tekście.
        [few] Kapitaliki są wyświetlane jako zwykłe litery w { $count } tekstach.
        [many] Kapitaliki są wyświetlane jako zwykłe litery w { $count } tekstach.
       *[other] Kapitaliki są wyświetlane jako zwykłe litery w { $count } tekstu.
    }
import-scrivener-prose-formatting-lost-all-caps-line = { $count ->
        [one] Tekst wyświetlany wersalikami jest pokazywany tak, jak go wpisano, w { $count } tekście.
        [few] Tekst wyświetlany wersalikami jest pokazywany tak, jak go wpisano, w { $count } tekstach.
        [many] Tekst wyświetlany wersalikami jest pokazywany tak, jak go wpisano, w { $count } tekstach.
       *[other] Tekst wyświetlany wersalikami jest pokazywany tak, jak go wpisano, w { $count } tekstu.
    }
import-scrivener-prose-formatting-lost-hidden-line = { $count ->
        [one] Ukryty tekst zostaje pominięty w { $count } tekście.
        [few] Ukryty tekst zostaje pominięty w { $count } tekstach.
        [many] Ukryty tekst zostaje pominięty w { $count } tekstach.
       *[other] Ukryty tekst zostaje pominięty w { $count } tekstu.
    }
import-scrivener-prose-formatting-lost-line = { $count ->
        [one] Formatowanie, dla którego Skribisto nie ma miejsca, zostaje usunięte w { $count } tekście.
        [few] Formatowanie, dla którego Skribisto nie ma miejsca, zostaje usunięte w { $count } tekstach.
        [many] Formatowanie, dla którego Skribisto nie ma miejsca, zostaje usunięte w { $count } tekstach.
       *[other] Formatowanie, dla którego Skribisto nie ma miejsca, zostaje usunięte w { $count } tekstu.
    }
import-scrivener-prose-justified-text-not-kept-line = { $count ->
        [one] { $count } wyjustowany akapit przyjmuje zamiast tego styl eksportu.
        [few] { $count } wyjustowane akapity przyjmują zamiast tego styl eksportu.
        [many] { $count } wyjustowanych akapitów przyjmuje zamiast tego styl eksportu.
       *[other] { $count } wyjustowanego akapitu przyjmuje zamiast tego styl eksportu.
    }
import-scrivener-prose-comment-colours-lost-line = { $count ->
        [one] { $count } komentarz traci swój kolor.
        [few] { $count } komentarze tracą swój kolor.
        [many] { $count } komentarzy traci swój kolor.
       *[other] { $count } komentarza traci swój kolor.
    }
import-scrivener-prose-control-characters-removed-line = { $count ->
        [one] { $count } niewidoczny znak sterujący zostaje usunięty.
        [few] { $count } niewidoczne znaki sterujące zostają usunięte.
        [many] { $count } niewidocznych znaków sterujących zostaje usuniętych.
       *[other] { $count } niewidocznego znaku sterującego zostaje usuniętych.
    }
import-scrivener-prose-note-on-heading-lost-line = { $count ->
        [one] { $count } przypis lub komentarz przy nagłówku rozdziału nie mógł tam pozostać: komentarz trafia do pierwszego akapitu rozdziału, a przypis zostaje pominięty.
        [few] { $count } przypisy lub komentarze przy nagłówkach rozdziałów nie mogły tam pozostać: komentarz trafia do pierwszego akapitu rozdziału, a przypis zostaje pominięty.
        [many] { $count } przypisów lub komentarzy przy nagłówkach rozdziałów nie mogło tam pozostać: komentarz trafia do pierwszego akapitu rozdziału, a przypis zostaje pominięty.
       *[other] { $count } przypisu lub komentarza przy nagłówkach rozdziałów nie mogło tam pozostać: komentarz trafia do pierwszego akapitu rozdziału, a przypis zostaje pominięty.
    }
import-scrivener-prose-media-file-copied-line = { $count ->
        [one] { $count } plik materiałów, którego Skribisto nie potrafi otworzyć, został skopiowany obok nowego projektu. Jego notatka podaje, gdzie się znajduje.
        [few] { $count } pliki materiałów, których Skribisto nie potrafi otworzyć, zostały skopiowane obok nowego projektu. Ich notatki podają, gdzie się znajdują.
        [many] { $count } plików materiałów, których Skribisto nie potrafi otworzyć, zostało skopiowanych obok nowego projektu. Ich notatki podają, gdzie się znajdują.
       *[other] { $count } pliku materiałów, których Skribisto nie potrafi otworzyć, zostało skopiowanych obok nowego projektu. Ich notatki podają, gdzie się znajdują.
    }
import-scrivener-prose-media-file-not-carried-line = { $count ->
        [one] { $count } plik materiałów, którego Skribisto nie potrafi otworzyć, zostaje w Twoim projekcie Scrivener. Jego notatka podaje jego nazwę.
        [few] { $count } pliki materiałów, których Skribisto nie potrafi otworzyć, zostają w Twoim projekcie Scrivener. Ich notatki podają ich nazwy.
        [many] { $count } plików materiałów, których Skribisto nie potrafi otworzyć, zostaje w Twoim projekcie Scrivener. Ich notatki podają ich nazwy.
       *[other] { $count } pliku materiałów, których Skribisto nie potrafi otworzyć, zostaje w Twoim projekcie Scrivener. Ich notatki podają ich nazwy.
    }

## When the import fails
import-scrivener-error-refused = { $refusal } Nic nie zostało zapisane.
import-scrivener-error-question = Projekt zadaje teraz pytanie, na które nie odpowiedziano podczas jego sprawdzania. Wybierz „Sprawdź import znowu”, a potem zaimportuj. Nic nie zostało zapisane.
import-scrivener-error-not-accepted = Aby można było zaimportować projekt, musisz zaznaczyć pole w kroku Projekt, a nie zostało ono zaznaczone. Wybierz „Sprawdź import znowu”, zaznacz je, a potem zaimportuj. Nic nie zostało zapisane.
import-scrivener-error-changed = Projekt zmienił się po sprawdzeniu. Wybierz „Sprawdź import znowu”, a potem zaimportuj. Nic nie zostało zapisane.
import-scrivener-error-choice = Struktura wybrana podczas sprawdzania nie pasuje już do tego projektu. Wybierz „Sprawdź import znowu”, a potem zaimportuj. Nic nie zostało zapisane.

export-title = Eksportuj
export-close = Zamknij
export-scope-label = Co
export-format-label = Format
export-style-label = Styl
export-path-label = Plik
export-browse = Przeglądaj…
export-show-non-exportable = Pokaż elementy wyłączone z eksportu
export-choose-empty = Brak elementów do wyboru.
export-cancel = Anuluj
export-export = Eksportuj
export-save-dialog-title = Eksportuj do pliku
export-section-what = Co eksportować
export-section-style = Styl
export-section-destination = Miejsce docelowe
export-custom-selection = Własny wybór
export-selected-count = Zaznaczone: { $count }
export-chip-chapters-none = Rozdziały: brak
export-chip-chapters-numbered = Rozdziały: numerowane
export-chip-chapters-title = Rozdziały: tylko tytuł
export-chip-chapters-both = Rozdziały: numer + tytuł
export-chip-scene-break-glyph = Separator sceny: { $glyph }
export-chip-scene-break-blank = Separator sceny: pusty wiersz
export-chip-scene-break-none = Separator sceny: brak
export-chip-major-break-glyph = Większy separator: { $glyph }
export-chip-major-break-blank = Większy separator: pusty wiersz
export-chip-major-break-none = Większy separator: brak
export-chip-major-break-same = Oba poziomy separatorów jednakowe
export-chip-spacing-single = Odstępy: pojedyncze
export-chip-spacing-onehalf = Odstępy: 1½
export-chip-spacing-double = Odstępy: podwójne
export-chip-notes-included = Z notatkami
export-chip-notes-excluded = Bez notatek
export-format-docx = Word
export-format-odt = LibreOffice
export-format-html = HTML
export-format-markdown = Markdown
export-format-djot = Djot
export-format-text = Tekst
export-format-latex = LaTeX
export-format-epub = EPUB
export-format-pdf = PDF
export-overwrite-title = Zastąpić istniejący plik?
export-overwrite-text = „{ $name }” już istnieje. Zastąpić go?
export-progress-title = Eksportowanie…
export-cancelled = Eksport anulowany
export-done = Wyeksportowano { $count ->
        [one] { $count } element
        [few] { $count } elementy
        [many] { $count } elementów
       *[other] { $count } elementu
    }
export-open-file = Otwórz
export-show-in-folder = Pokaż w folderze
export-error-title = Nie można wyeksportować
export-error-details = Szczegóły

backup-progress-start = Rozpoczynanie…
backup-progress-retention = Usuwanie starych kopii zapasowych…
backup-progress-done = Gotowe
backup-progress-destination = Miejsce docelowe { $i } z { $n }
backup-details = Szczegóły
backup-issues-title = Problemy z kopią zapasową
backup-failed-title = Nie udało się utworzyć kopii zapasowej
backup-complete-prune-warning = Kopia zapasowa ukończona (zapisane: { $ok }, już aktualne: { $skipped }). Nie udało się usunąć niektórych starych kopii zapasowych

search = Szukaj
search-query-placeholder = Szukaj…
search-replace-placeholder = Zastąp przez…
search-replace-toggle = Pokaż lub ukryj zastępowanie
search-replace-all = Zastąp wszystko
search-opt-case = Uwzględniaj wielkość liter
search-opt-whole-word = Całe słowo
search-opt-diacritics = Uwzględniaj znaki diakrytyczne
search-scope-body = Treść
search-scope-title = Tytuł
search-scope-synopsis = Streszczenie
search-scope-label = Etykieta
search-facet-book = Książki
search-facet-part = Części
search-facet-chapter = Rozdziały
search-facet-scene = Sceny
search-facet-note = Notatki
search-facet-folder = Foldery
search-tip-case = Uwzględniaj wielkość liter: wielkie i małe litery są traktowane jako różne, więc „Elena” i „elena” to osobne trafienia.
search-tip-whole-word = Całe słowo: dopasowuje tylko pełne słowa, więc „kot” nie zostanie znaleziony wewnątrz słowa „kotlet”.
search-tip-diacritics = Uwzględniaj znaki diakrytyczne: litery ze znakami diakrytycznymi są traktowane jako odrębne, więc „zolw” nie pasuje do „żółw”.
search-tip-body = Treść: przeszukuje prozę scen i notatek.
search-tip-title = Tytuł: przeszukuje tytuły elementów segregatora.
search-tip-synopsis = Streszczenie: przeszukuje streszczenie każdego elementu z tekstem.
search-tip-label = Etykieta: przeszukuje krótką adnotację wyświetlaną pod tytułem elementu segregatora.
search-tip-comment = Komentarze: przeszukuje tekst wątków komentarzy i odpowiedzi. Komentarz dotyczy rękopisu, a nie jest jego częścią, więc ma osobny przełącznik, a przy zastępowaniu trafienia w komentarzach pozostają niezaznaczone, dopóki ich nie zaznaczysz.
search-tip-book = Książki: kontener książki i jej znaczniki początku / końca.
search-tip-part = Części: separatory na poziomie części.
search-tip-chapter = Rozdziały: rozdziały, niezależnie od tego, w jaki sposób projekt je przechowuje.
search-tip-scene = Sceny: elementy zawierające Twoją prozę.
search-tip-note = Notatki: swobodne notatki.
search-tip-folder = Foldery: zwykłe foldery porządkowe i separatory.
search-tip-paratext = Tekst, który należy do książki, ale nie do jej opowieści: przedmowa, dedykacja, posłowie. Nigdy nie jest wliczany do rękopisu.
search-tip-preserve-case = Zachowaj wielkość liter: zamiennik przyjmuje wielkość liter znalezionego tekstu, więc „ELENA” staje się „MARTA”, a „Elena” staje się „Marta”.
search-tip-replace = Zastąp: pokazuje pole zamiennika i przycisk „Zastąp wszystko”.
search-error = Wyszukiwanie nie powiodło się: { $message }
search-no-matches = Brak dopasowań
search-count =
    { $matches ->
        [one] { $matches } trafienie
        [few] { $matches } trafienia
        [many] { $matches } trafień
       *[other] { $matches } trafienia
    } w { $items ->
        [one] { $items } dokumencie
        [few] { $items } dokumentach
        [many] { $items } dokumentach
       *[other] { $items } dokumentu
    }
search-count-truncated =
    { $matches ->
        [one] { $matches } trafienie
        [few] { $matches } trafienia
        [many] { $matches } trafień
       *[other] { $matches } trafienia
    } w { $items ->
        [one] { $items } dokumencie
        [few] { $items } dokumentach
        [many] { $items } dokumentach
       *[other] { $items } dokumentu
    } (tylko pierwsze wyniki)
search-occurrences = { $count }
search-field-body = Treść
search-field-title = Tytuł
search-field-synopsis = Streszczenie
search-field-label = Etykieta
search-field-epigraph = Motto
search-field-comment = Komentarz
search-field-comment-reply = Odpowiedź
search-field-footnote = Przypis
search-include-in-replace = Uwzględnij w „Zastąp wszystko”
search-collapse-all = Zwiń wszystkie wyniki
search-replace-here = Zastąp to
search-dismiss = Usuń z wyników
search-undo-dismiss = Przywróć ostatnio usunięty wynik
search-replace-nothing = (nic)
search-replace-confirm-title = Zastąpić wszystkie dopasowania?
search-replace-confirm-text =
    Zastąpić { $occurrences ->
        [one] { $occurrences } wystąpienie
        [few] { $occurrences } wystąpienia
        [many] { $occurrences } wystąpień
       *[other] { $occurrences } wystąpienia
    } „{ $query }” tekstem „{ $replacement }” w { $items ->
        [one] { $items } dokumencie
        [few] { $items } dokumentach
        [many] { $items } dokumentach
       *[other] { $items } dokumentu
    }? Później można to cofnąć z poziomu powiadomienia.
search-replace-done-title = Zastępowanie zakończone
search-replace-done =
    Zastąpiono { $occurrences ->
        [one] { $occurrences } wystąpienie
        [few] { $occurrences } wystąpienia
        [many] { $occurrences } wystąpień
       *[other] { $occurrences } wystąpienia
    } w { $items ->
        [one] { $items } dokumencie
        [few] { $items } dokumentach
        [many] { $items } dokumentach
       *[other] { $items } dokumentu
    }.
search-replace-done-skipped =
    Zastąpiono { $occurrences ->
        [one] { $occurrences } wystąpienie
        [few] { $occurrences } wystąpienia
        [many] { $occurrences } wystąpień
       *[other] { $occurrences } wystąpienia
    } w { $items ->
        [one] { $items } dokumencie
        [few] { $items } dokumentach
        [many] { $items } dokumentach
       *[other] { $items } dokumentu
    }. Pominięto { $skipped ->
        [one] { $skipped } pole
        [few] { $skipped } pola
        [many] { $skipped } pól
       *[other] { $skipped } pola
    } (zmienione od czasu wyszukiwania).
search-replace-undo = Cofnij
search-replace-failed-title = Zastępowanie nie powiodło się
search-replace-skipped-title = Nic nie zastąpiono
search-replace-skipped-body = { $fields ->
        [one] Ten tekst zmienił się od czasu wyszukiwania, więc pozostawiono go bez zmian.
        [few] { $fields } z tych tekstów zmieniły się od czasu wyszukiwania, więc pozostawiono je bez zmian.
        [many] { $fields } z tych tekstów zmieniło się od czasu wyszukiwania, więc pozostawiono je bez zmian.
       *[other] { $fields } z tych tekstów zmieniło się od czasu wyszukiwania, więc pozostawiono je bez zmian.
    } Wyszukaj ponownie, aby zobaczyć, gdzie te słowa są teraz.

search-replace-undo-failed-title = Cofanie nie powiodło się
search-preview = Podgląd
search-preview-empty = Wybierz wynik, aby wyświetlić tu jego podgląd
search-preview-no-prose = To trafienie nie ma tekstu do edycji
search-preview-prompt = Aby zobaczyć tu podgląd, uruchom wyszukiwanie.
search-preview-open-search = Szukaj w projekcie
search-preview-footnote-prompt = To trafienie znajduje się w treści przypisu. Otwórz go w panelu „Przypisy”, aby go zobaczyć i edytować.
search-preview-open-footnotes = Otwórz przypisy

find-placeholder = Znajdź w tym dokumencie…
find-count = { $current } z { $total }
find-no-results = Brak wyników
find-close = Zamknij wyszukiwanie
find-previous = Poprzednie trafienie (Shift+Enter)
find-next = Następne trafienie (Enter)
find-opt-case = Uwzględniaj wielkość liter
find-opt-whole-word = Całe słowo
find-replace-toggle = Pokaż lub ukryj zastępowanie (Ctrl+R)
find-replace-placeholder = Zastąp przez…
find-replace = Zastąp
find-replace-all = Zastąp wszystko
find-preserve-case = Zachowaj wielkość liter

dict-download-title = Pobieranie { $name }…
dict-download-done = Zainstalowano { $name }
dict-download-failed = Nie można pobrać { $name }: { $error }
dict-removed = Usunięto { $name }
dict-accept-first = Zaakceptuj licencję przed pobraniem { $name }
settings-dict-tab-installed = Zainstalowane
settings-dict-tab-get-more = Pobierz więcej
settings-dict-tab-personal = Słowa własne
dict-installed-empty = Nie znaleziono jeszcze słowników na tym komputerze.
dict-get-more-search = Szukaj języków
dict-system-badge = w systemie
dict-unusable-badge = nie do użycia
dict-download-button = Pobierz
dict-downloading = Pobieranie…
dict-installed-label = Zainstalowane
dict-remove = Usuń
dict-view-license = Pokaż licencję
dict-approx-size = ~{ $size }
dict-personal-empty = W tym projekcie nie ma jeszcze słów własnych.
dict-personal-add = Dodaj
dict-personal-placeholder = Dodaj słowo…
dict-missing-toast = { $count ->
        [one] Ten projekt używa { $count } słownika, który nie jest zainstalowany
        [few] Ten projekt używa { $count } słowników, które nie są zainstalowane
        [many] Ten projekt używa { $count } słowników, które nie są zainstalowane
       *[other] Ten projekt używa { $count } słownika, które nie są zainstalowane
    }
dict-missing-action = Pobierz słowniki
dict-license-title = Licencja: { $name }
dict-license-accept = Zaakceptuj i pobierz
dict-license-cancel = Anuluj
dict-license-close = Zamknij
dict-add-button = Dodaj słownik…
dict-add-title = Dodaj słownik
dict-add-close = Zamknij
dict-add-name = Nazwa
dict-add-name-placeholder = np. Mój słownik łaciński
dict-add-code = Kod języka
dict-add-code-placeholder = np. la lub fr-FR-x-custom
dict-add-code-hint = Krótki identyfikator do wyboru. To jego wybierasz jako język dokumentu.
dict-add-aff = Plik afiksów (.aff)
dict-add-dic = Lista słów (.dic)
dict-add-submit = Dodaj
dict-add-cancel = Anuluj
dict-add-name-required = Nadaj słownikowi nazwę.
dict-add-code-required = Wprowadź kod języka.
dict-add-code-invalid = Używaj tylko liter, cyfr oraz znaków - _ .
dict-add-code-reserved = To kod wbudowanego słownika. Wybierz inny.
dict-add-file-required = Wybierz plik.
dict-add-file-missing = Ten plik nie istnieje.
dict-add-code-taken = Słownik dla tego kodu jest już zainstalowany.
dict-add-done = Dodano { $name }
dict-add-unusable = Te pliki nie tworzą użytecznego słownika: { $error }
dict-add-failed = Nie można dodać słownika: { $error }
inspector-export = Eksportuj
inspector-exportable = Uwzględnij w eksporcie
ctx-number = &Numeruj ten rozdział
ctx-unnumber = &Nie numeruj tego rozdziału
inspector-numbering = Numeracja
inspector-numbered = Numerowany
inspector-numbered-tip = Ten rozdział ma swoje miejsce w numeracji książki. Wyłącz tę opcję dla prologu, epilogu lub interludium.
inspector-numbered-tip-more = Rozdział nienumerowany pozostaje w książce dokładnie taki, jaki był: jego nagłówek, proza i liczba słów są nietknięte. Po prostu nie drukuje się przy nim numer i nie zużywa on numeru: rozdział po prologu jest rozdziałem pierwszym, nie drugim. Wyłączenie go z eksportu to inny przełącznik, powyżej, i ten usuwa rozdział z książki całkowicie.
inspector-apply-to-children = Zastosuj do elementów podrzędnych
inspector-milestone = Data kamienia milowego
inspector-milestone-none = Brak daty
inspector-milestone-clear = Wyczyść
inspector-tags = Znaczniki
inspector-aliases = Inne nazwy
inspector-dict-language = Język
inspector-apply-language-to-children = Zastosuj język do elementów podrzędnych
settings-page-language = Język
settings-field-dict-language = Języki
dict-tradeoff-hint = Każdy dodatkowy język akceptuje więcej słów, więc wychwytywanych jest mniej błędów.
lang-inherit-hint = Dziedziczone: ta scena używa języków książki lub projektu.
lang-pill-list = Języki
lang-pill-add = Dodaj język
lang-pill-remove = Usuń { $name }
lang-pill-mute = Wyłącz sprawdzanie pisowni dla języka { $name }
lang-pill-unmute = Włącz sprawdzanie pisowni dla języka { $name }

settings-page-personal-dictionary = Słownik własny
settings-user-dict-desc = Dodane tu słowa są uznawane za poprawne w całej aplikacji i nigdy więcej nie są oznaczane.
settings-user-dict-add-placeholder = Wpisz słowo…
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
settings-user-dict-empty = Brak słów. Dodaj słowa, które sprawdzanie pisowni ma pomijać, lub zaimportuj listę.
settings-user-dict-import = Importuj…
settings-user-dict-export = Eksportuj…
settings-user-dict-import-tip = Dodaje słowa ze zwykłego pliku tekstowego, po jednym słowie w wierszu. Istniejące słowa zostają zachowane.
settings-user-dict-export-tip = Zapisuje całą listę do zwykłego pliku tekstowego, po jednym słowie w wierszu.
settings-user-dict-txt-filter = Pliki tekstowe
settings-user-dict-duplicate = Już w słowniku
settings-user-dict-imported = Zaimportowano { $count ->
        [one] { $count } słowo
        [few] { $count } słowa
        [many] { $count } słów
       *[other] { $count } słowa
    } (już obecne: { $duplicates }).
settings-user-dict-import-failed = Nie można odczytać listy słów: { $error }
settings-user-dict-exported = Zapisano { $count ->
        [one] { $count } słowo.
        [few] { $count } słowa.
        [many] { $count } słów.
       *[other] { $count } słowa.
    }
settings-user-dict-export-failed = Nie można zapisać listy słów: { $error }

settings-page-text-replacements = Zamiany tekstu
settings-text-repl-desc = Zastępuje skrót pełnym tekstem podczas pisania: „pzdr” staje się „pozdrawiam” w chwili wpisania spacji lub znaku interpunkcyjnego.
settings-text-repl-enable = Używaj zamian tekstu w tym projekcie
settings-text-repl-disabled-hint = Włącz tę opcję, aby definiować skróty rozwijane podczas pisania.
settings-text-repl-add = Dodaj regułę
settings-text-repl-trigger-placeholder = Skrót
settings-text-repl-replacement-placeholder = Rozwinięcie
settings-text-repl-added = Dodano „{ $trigger }”
settings-text-repl-duplicate = „{ $trigger }” ma już regułę
settings-text-repl-filter = Filtruj reguły
settings-text-repl-count = { $n ->
        [one] { $n } reguła
        [few] { $n } reguły
        [many] { $n } reguł
       *[other] { $n } reguły
    }
settings-text-repl-row-enabled = Używaj tej reguły
settings-text-repl-delete = Usuń regułę dla { $trigger }
settings-text-repl-deleted = Usunięto regułę dla „{ $trigger }”
settings-text-repl-empty = Brak reguł.
settings-text-repl-csv-filter = Pliki CSV
settings-text-repl-import = Importuj…
settings-text-repl-export = Eksportuj…
settings-text-repl-imported = Zaimportowano: { $added }, pominięto: { $skipped }
settings-text-repl-exported = Wyeksportowano { $n ->
        [one] { $n } regułę
        [few] { $n } reguły
        [many] { $n } reguł
       *[other] { $n } reguły
    }

editor-menu-no-suggestions = Brak sugestii
editor-menu-add-to-dictionary = Dodaj „{ $word }” do słownika
editor-menu-add-words-to-dictionary = Dodaj zaznaczone słowa do słownika
editor-dict-added = Dodano „{ $word }” do słownika.
editor-dict-added-multi = { $count ->
        [one] Dodano { $count } słowo do słownika.
        [few] Dodano { $count } słowa do słownika.
        [many] Dodano { $count } słów do słownika.
       *[other] Dodano { $count } słowa do słownika.
    }
toast-undo = Cofnij

undo-superseded-title = Ten krok nie jest już ostatni
undo-superseded-body = Od tego czasu w projekcie zmieniło się coś innego. Cofnięcie cofnęłoby tamtą zmianę, więc nie zostało wykonane. Nic z tego kroku nie zostało odwrócone. Użyj polecenia Edycja ▸ Cofnij, aby samodzielnie cofać się w historii.
undo-failed = Cofnięcie nie powiodło się: { $error }

menu-edit = &Edycja
menu-edit-undo = C&ofnij
menu-edit-redo = &Ponów
menu-edit-undo-target = &Cofnij: { $target }
menu-edit-redo-target = &Ponów: { $target }
shortcut-name-edit-undo = Cofnij
shortcut-name-edit-redo = Ponów
undo-target-typing = pisanie
undo-target-project = ostatnia zmiana w tym projekcie
prose-history-reset-title = Historia pisania zresetowana
prose-history-reset-body = { $count ->
        [one] Jedna otwarta scena została przywrócona do wcześniejszego stanu, więc jej historia pisania nie ma już zastosowania.
        [few] { $count } otwarte sceny zostały przywrócone do wcześniejszego stanu, więc ich historia pisania nie ma już zastosowania.
        [many] { $count } otwartych scen zostało przywróconych do wcześniejszego stanu, więc ich historia pisania nie ma już zastosowania.
       *[other] { $count } otwartej sceny zostało przywróconych do wcześniejszego stanu, więc ich historia pisania nie ma już zastosowania.
    }
undo-target-trash = przeniesienie do kosza
undo-target-restore = przywrócenie z kosza
undo-target-delete-forever = trwałe usunięcie
undo-target-replace-all = zastąpienie w całym projekcie
undo-target-import = import dokumentu
undo-target-duplicate = duplikowanie
undo-target-move = przeniesienie
undo-target-merge = scalenie dwóch scen
undo-target-split = podział sceny
undo-target-promote = zmiana typu
undo-target-tidy-titles = porządkowanie tytułów rozdziałów
undo-target-import-tags = import znaczników
undo-target-import-templates = import szablonów notatek
undo-target-create = tworzenie
undo-target-remove = usuwanie
undo-target-rename = zmiana nazwy
undo-target-edit = ta zmiana
menu-edit-find = &Znajdź…
menu-edit-find-next = Znaj&dź następny
menu-edit-find-prev = Z&najdź poprzedni
menu-edit-replace = Znajdź menu-edit-replace = Znajd&ź i zastąp…i zastąp…
menu-edit-find-in-project = Zna&jdź w projekcie…
menu-edit-replace-in-project = Za&stąp w projekcie…
undo-frozen = Cofnij (włączone „Zawsze do przodu”)
redo-frozen = Ponów (włączone „Zawsze do przodu”)
menu-edit-cut = Wy&tnij
menu-edit-copy = &Kopiuj
menu-edit-paste = &Wklej
menu-edit-paste-plain = Wk&lej bez formatowania
menu-edit-select-all = Z&aznacz wszystko
shortcut-name-edit-cut = Wytnij
shortcut-name-edit-copy = Kopiuj
shortcut-name-edit-paste = Wklej
shortcut-name-edit-paste-plain = Wklej bez formatowania
shortcut-name-edit-select-all = Zaznacz wszystko

titlebar-spellcheck-on = Sprawdzanie pisowni jest włączone. Kliknij, aby przestać sprawdzać (F7)
titlebar-spellcheck-off = Sprawdzanie pisowni jest wyłączone. Kliknij, aby sprawdzić ponownie (F7)
menu-spellcheck = &Sprawdź pisownię
menu-comments = &Komentarze
settings-page-spellcheck = Sprawdzanie pisowni
settings-group-spellcheck = Sprawdzanie pisowni
settings-spellcheck-enabled = Sprawdzaj pisownię podczas pisania
settings-spellcheck-hint = Podkreśla słowa, których nie zna żaden zainstalowany słownik. Wyłączenie tej opcji zatrzymuje całe sprawdzanie, w każdym projekcie, dopóki jej ponownie nie włączysz. Aby przestać sprawdzać tylko jeden język, odznacz go w polu „Język” projektu lub elementu.

menu-trash = Ko&sz
trash-title = Kosz
trash-empty-state = Kosz jest pusty.
trash-empty-button = Opróżnij kosz…
trash-restore = &Przywróć
trash-restore-to = Przywróć &do…
trash-delete-forever = &Usuń na zawsze
trash-restored-ok = Przywrócono: { $count }.
trash-restore-error = Nie można przywrócić: { $error }
trash-restore-orphaned = Pierwotne miejsce tego elementu już nie istnieje. Wybierz, gdzie go przywrócić.
trash-restore-no-project = Żaden projekt nie jest otwarty, więc nie ma czego przywracać.
trash-delete-no-project = Żaden projekt nie jest otwarty, więc nie ma czego usuwać.
trash-empty-confirm-title = Opróżnić kosz?
trash-empty-confirm-text = { $count ->
        [one] { $count } wpis z kosza zostanie trwale usunięty.
        [few] { $count } wpisy z kosza zostaną trwale usunięte.
        [many] { $count } wpisów z kosza zostanie trwale usuniętych.
       *[other] { $count } wpisu z kosza zostanie trwale usunięte.
    } Opuszczą kosz na dobre. Dopóki nie zrobisz czegoś innego, polecenie Cofnij może je jeszcze przywrócić.
trash-emptied-title = Kosz opróżniony
trash-emptied-body = Cała zawartość kosza została trwale usunięta.
trash-empty-no-project = Żaden projekt nie jest otwarty, więc nie ma kosza do opróżnienia.
trash-delete-forever-confirm-title = Usunąć na zawsze?
trash-delete-forever-confirm-text = { $count ->
        [one] { $count } element zostanie trwale usunięty.
        [few] { $count } elementy zostaną trwale usunięte.
        [many] { $count } elementów zostanie trwale usuniętych.
       *[other] { $count } elementu zostanie trwale usunięte.
    } Opuszczą kosz na dobre. Dopóki nie zrobisz czegoś innego, polecenie Cofnij może je jeszcze przywrócić.
trash-deleted-title = Usunięto na zawsze
trash-deleted-body = Trwale usunięto: { $count }.
trash-undo = Cofnij
trash-restore-picker-title = Przywróć do…
trash-restore-to-confirm-title = Przywrócić tutaj?
trash-restore-to-confirm-text = Przywrócić „{ $item }” do „{ $destination }”?
trash-restore-picker-restore-here = Przywróć tutaj
trash-restore-picker-cancel = Anuluj
trash-restore-picker-empty = Nie ma jeszcze segregatorów. Najpierw utwórz segregator.
trash-banner-title = Ten element jest w koszu
trash-banner-description = Nie pojawi się w konspekcie ani w eksportach, dopóki go nie przywrócisz.
trash-banner-restore = Przywróć…
trash-tab-tooltip = W koszu

settings-styles-sheet-yes = Tak
settings-styles-sheet-no = Nie
settings-styles-sheet-font = Czcionka
settings-styles-sheet-indent = Wcięcie pierwszego wiersza
settings-styles-sheet-para-spacing = Odstępy między akapitami
settings-styles-sheet-page = Rozmiar strony
settings-styles-sheet-margins = Marginesy (góra/prawo/dół/lewo)
settings-styles-sheet-title-page = Strona tytułowa książki
settings-styles-sheet-heading-language = Język nagłówków
settings-styles-sheet-auto = Zgodnie z tekstem
settings-styles-sheet-digits = Cyfry
settings-styles-sheet-direction = Kierunek tekstu
settings-styles-sheet-formats = Formaty
settings-styles-sheet-all-formats = Wszystkie

settings-styles-editor-group = Edytor stylu
settings-styles-editor-missing = Ten styl jest już niedostępny.

settings-styles-page-letter = Letter
settings-styles-digits-western = Zachodnie (0–9)
settings-styles-digits-eastern-arabic = Wschodnioarabskie (٠–٩)
settings-styles-direction-ltr = Od lewej do prawej
settings-styles-direction-rtl = Od prawej do lewej

format-dock-title = Format
format-panel-empty = Umieść kursor w scenie, notatce lub streszczeniu, aby zobaczyć opcje formatowania.
format-group-marks = Tekst
format-group-block = Akapit
format-group-lists = Listy
format-group-tables = Tabela
format-group-breaks = Separatory scen
format-superscript = Indeks górny
format-subscript = Indeks dolny
format-link = Odnośnik…
link-dialog-insert-title = Wstaw odnośnik
link-dialog-edit-title = Edytuj odnośnik
link-dialog-text-label = Tekst
link-dialog-text-placeholder = słowa widoczne dla czytelnika
link-dialog-url-label = Prowadzi do
link-dialog-url-placeholder = example.com
link-dialog-insert = Wstaw
link-dialog-apply = Zastosuj
link-dialog-cancel = Anuluj
link-dialog-remove = Usuń odnośnik
link-scheme-refused = Otwierać można tylko odnośniki do stron WWW i adresy e-mail. Odnośnik { $url } pozostawiono bez zmian.
menu-format-link = &Odnośnik…
format-clear = Wyczyść formatowanie
format-heading = Poziom nagłówka
format-heading-normal = Zwykły tekst
format-heading-1 = Nagłówek 1
format-heading-2 = Nagłówek 2
format-heading-3 = Nagłówek 3
format-heading-4 = Nagłówek 4
format-heading-5 = Nagłówek 5
format-heading-6 = Nagłówek 6
format-align-left = Wyrównaj w lewo
format-align-center = Wyśrodkuj
format-direction-rtl = Akapit od prawej do lewej
format-blockquote = Cytat blokowy
format-list-bullet = Lista punktowana
format-list-numbered = Lista numerowana
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
menu-format-marks-underline = Podkreś&lenie
menu-format-marks-strike = Pr&zekreślenie
menu-format-marks-superscript = Ind&eks górny
menu-format-marks-subscript = In&deks dolny
menu-format-marks-clear = W&yczyść formatowanie
menu-format-heading = &Nagłówek
menu-format-heading-normal = &Zwykły tekst
menu-format-heading-1 = Nagłówek &1
menu-format-heading-2 = Nagłówek &2
menu-format-heading-3 = Nagłówek &3
menu-format-heading-4 = Nagłówek &4
menu-format-heading-5 = Nagłówek &5
menu-format-heading-6 = Nagłówek &6
menu-format-alignment = Wy&równanie
menu-format-align-left = Do &lewej
menu-format-align-center = Do ś&rodka
menu-format-direction = &Kierunek
menu-format-direction-auto = &Automatyczne
menu-format-direction-ltr = &Od lewej do prawej
menu-format-direction-rtl = &Od prawej do lewej
menu-format-blockquote = &Cytat blokowy
menu-format-lists = L&isty
menu-format-list-bullet = &Lista punktowana
menu-format-list-numbered = Lista &numerowana
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

menu-format-typo-marks = Znak &formatujący
menu-format-typo-nbsp = &Spacja nierozdzielająca
menu-format-typo-nbhyphen = Łącznik &nierozdzielający
menu-format-typo-soft-hyphen = Łącznik &opcjonalny
menu-format-typo-nnbsp = &Wąska spacja nierozdzielająca
menu-format-typo-zwsp = Spacja o &zerowej szerokości
menu-format-typo-word-joiner = Łą&cznik wyrazów


menu-about = &O Skribisto…
about-title = O Skribisto
about-version = Wersja { $version }
about-update-available = Wersja { $version } jest dostępna
about-update-see-notes = Zobacz, co się zmieniło
about-update-flathub = Flathub dba o aktualność tej kopii.
about-update-distro = Twoja dystrybucja dba o aktualność tej kopii.
about-tagline = Aplikacja do pisania powieści i innych długich form prozatorskich, napisana w języku Rust z użyciem zestawu narzędzi Teksilo.
about-license = Wydano na warunkach Powszechnej Licencji Publicznej GNU, wersja 3.
about-copyright = © 2026 Cyril Jacquet
about-close = Zamknij

native-menu-about = O { $app }
native-menu-hide = Ukryj { $app }
native-menu-quit = Zakończ { $app }
native-menu-settings = Ustawienia…
native-menu-window = Okno
native-menu-minimize = Minimalizuj
native-menu-zoom = Powiększ

window-title = { $title } - { $app }
window-title-numbered = { $title } - { $app } (Okno { $n })
window-title-empty = { $app }

first-run-window-title = Witamy
first-run-title = Konfiguracja { $app }
first-run-body = { $app } przechowuje ustawienia oddzielnie od Skribisto, więc na początku ich nie ma. Możesz teraz skopiować swoje preferencje, ostatnie projekty, słowniki i układ okien.
first-run-from = Kopiuj z
first-run-to = Kopiuj do
first-run-copy-note = Nic nie zostanie przeniesione ani usunięte. Skribisto zachowuje wszystkie swoje ustawienia i nadal działa dokładnie tak jak wcześniej.
first-run-import = Importuj ustawienia
first-run-start-fresh = Zacznij od zera
first-run-import-failed = Nie udało się zaimportować niektórych ustawień: { $error }

settings-page-punctuation = Domyślna interpunkcja
settings-group-punctuation = Inteligentna interpunkcja
settings-page-work-punctuation = Interpunkcja
settings-punctuation-override = Ustaw dla tego projektu własne zasady interpunkcji
settings-punctuation-override-hint = Wyłączone: projekt stosuje ustawienia aplikacji. Te zasady są zapisywane w pliku .skrib, więc współautor otwierający plik pisze z tą samą typografią.
settings-punctuation-dashes = Zamieniaj -- na półpauzę, a --- na pauzę
settings-punctuation-ellipsis = Zamieniaj ... na wielokropek
settings-punctuation-quotes = Zamieniaj proste cudzysłowy i apostrofy na drukarskie
settings-quote-style = Cudzysłowy
settings-quote-style-locale = Domyślne dla języka
settings-quote-style-curly = “Podwójne”
settings-quote-style-curly-single = ‘Pojedyncze’
settings-quote-style-guillemets = «Szewrony»
settings-quote-style-low-high = „Dolne i górne”
settings-punctuation-spacing = Spacja przed ; : ! ?
settings-punctuation-spacing-hint = Typografia francuska wstawia wąską spację nierozdzielającą przed ; ! ? i pełną przed :. Dotyczy to tylko tekstu napisanego po francusku. Spacje wewnątrz szewronów « » są częścią samego stylu cudzysłowów szewronowych.
settings-punctuation-sample = Twój język daje
settings-punctuation-app-hint = To, co robi każdy projekt, chyba że ma własne zasady w Projekt ▸ Interpunkcja.
settings-punctuation-dialogue = Zaczynaj akapit wpisany jako „- ” od myślnika dialogowego
settings-punctuation-dialogue-hint = Dla języków, które oznaczają wypowiedzi myślnikiem zamiast cudzysłowu: francuskiego, hiszpańskiego, rosyjskiego i innych. Działa tylko na samym początku akapitu.
settings-punctuation-auto-lists = Zaczynaj listę od akapitu wpisanego jako "- ", "* " lub "1. "
settings-punctuation-auto-lists-hint = Akapit staje się listą punktowaną lub numerowaną. Tam, gdzie myślnik dialogowy jest włączony, a język go ma, "- " otwiera zamiast tego kwestię dialogową. Naciśnij Backspace zaraz potem, aby odzyskać to, co wpisano.

statusbar-go-to = Przejdź do…
go-to-placeholder = Przeszukaj segregator
go-to-no-matches = Żaden element nie pasuje do tego wyszukiwania.
menu-go-to = Przejdź &do…

settings-page-distraction-free-themes = Motywy trybu bez rozpraszania
settings-themes-builtin = Motywy wbudowane
settings-themes-builtin-badge = Wbudowany
settings-themes-user = Moje motywy
settings-themes-editor-group = Edytuj motyw
settings-themes-editor-empty = Wybierz motyw w sekcji „Moje motywy”, aby go edytować.
settings-themes-use = Użyj
settings-themes-duplicate = Duplikuj
settings-themes-edit = Edytuj
settings-themes-delete = Usuń
settings-themes-export = Eksportuj…
settings-themes-import = Importuj…
settings-themes-copy-suffix = kopia
settings-themes-json-filter = Motyw (JSON)
settings-themes-imported = Zaimportowano motyw
settings-themes-import-failed = Nie można zaimportować tego motywu
settings-themes-exported = Wyeksportowano motyw
settings-themes-export-failed = Nie można wyeksportować tego motywu
settings-themes-low-contrast = niski kontrast
settings-themes-low-contrast-band = podświetlenie zasłania tekst
settings-themes-field-name = Nazwa
settings-themes-field-paper = Strona
settings-themes-field-ink = Tekst
settings-themes-field-general = Tło
settings-themes-field-widget-text = Tekst paska sterowania
settings-themes-field-caret-band = Podświetlenie wokół kursora

statusbar-focus-settings = Ustawienia trybu bez rozpraszania
statusbar-focus-manage-themes = Zarządzaj motywami…

comments-title = Komentarze
comments-document-title = Ten dokument
comments-empty-project = Brak komentarzy w tym projekcie.
comments-empty-document = Brak komentarzy do tego dokumentu.
comments-filter-all = Wszystkie
comments-filter-open = Otwarte
comments-filter-resolved = Rozwiązane
comments-filter-orphaned = Osierocone
comments-status-open = Otwarty
comments-status-resolved = Rozwiązany
comments-status-orphaned = Utracił swój tekst
comments-orphan-snippet = (skomentowany tekst zniknął)
comments-status-unplaced = Brak pozycji w tekście
comments-unplaced-snippet = (niezakotwiczone w żadnym tekście)
comments-reply-count = { $count ->
    [one] { $count } odpowiedź
    [few] { $count } odpowiedzi
    [many] { $count } odpowiedzi
   *[other] { $count } odpowiedzi
}
comments-menu-resolve = Rozwiąż
comments-menu-reopen = Otwórz ponownie
comments-menu-delete = Usuń
comments-sort-document = W kolejności w dokumencie
comments-sort-newest = Od najnowszych
comments-sort-reader = Pogrupowane według czytelnika
comments-unsigned-author = Bez podpisu
comments-agreement = { $count ->
        [one] { $count } czytelnik
        [few] { $count } czytelnicy
        [many] { $count } czytelników
       *[other] { $count } czytelnika
    }
comments-menu-add = Dodaj komentarz
comments-menu-add-paragraph = Skomentuj ten akapit
overview-col-comments = Komentarze
overview-col-total-comments = Komentarze łącznie
comments-card-placeholder = Napisz komentarz…
comments-card-unknown-author = Nieznany autor
comments-unsigned-toast = Twoje komentarze nie są podpisane: na tym komputerze nie ustawiono Twojego imienia i nazwiska.
comments-unsigned-action = Ustaw imię i nazwisko
comments-card-reply = Odpowiedz
comments-card-reply-placeholder = Odpowiedz…
comments-card-actions = Działania komentarza
comments-reply-actions = Działania odpowiedzi
comments-menu-delete-reply = Usuń odpowiedź
comments-menu-delete-all = Usuń wszystkie komentarze w tym miejscu
comments-deleted-toast = Komentarz usunięty
comments-reply-deleted-toast = Odpowiedź usunięta
comments-deleted-all-toast = { $count ->
        [one] Usunięto { $count } komentarz
        [few] Usunięto { $count } komentarze
        [many] Usunięto { $count } komentarzy
       *[other] Usunięto { $count } komentarza
    }
comments-undo = Cofnij

analysis-segment = Analiza
analysis-scope-book = Analiza tej książki
analysis-run = Przeprowadź analizę
analysis-stale = Zmieniono od czasu tej analizy
analysis-not-run = Jeszcze nie przeanalizowano.
analysis-running = Czytanie rękopisu…
analysis-failed = Nie udało się ukończyć analizy.
analysis-no-scenes = Ta książka nie ma jeszcze scen.

analysis-shape = Kształt

analysis-arrivals = Napływ tekstu
analysis-arrivals-explainer = Jak tekst trafiał do tego projektu, gdy był otwarty. Mówi, którą drogą przyszły znaki, i nic o tym, kto je napisał: pisarz, który tworzy gdzie indziej i wkleja, wkleił, a ten, kto dyktuje, podyktował. Nie ma tu żadnej liczby, do której należy dążyć.
analysis-arrivals-scope = Cały projekt, nie tylko ta książka.
analysis-arrivals-session = Od chwili otwarcia. Zamknięcie projektu rozpoczyna liczenie od nowa.
analysis-arrivals-nothing = W tej sesji nie napłynął jeszcze żaden tekst.
analysis-arrivals-typed = Wpisany
analysis-arrivals-pasted = Wklejony
analysis-arrivals-dictated = Podyktowany
analysis-arrivals-imported = Zaimportowany
analysis-arrivals-programmatic = Wstawiony za Ciebie
analysis-arrivals-count = { $count ->
        [one] { $count } znak
        [few] { $count } znaki
        [many] { $count } znaków
       *[other] { $count } znaku
    }
analysis-arrivals-none = brak

analysis-words-per-scene = Słowa na scenę
analysis-median-words = Mediana długości sceny w tej książce wynosi { $count ->
        [one] { $count } słowo.
        [few] { $count } słowa.
        [many] { $count } słów.
       *[other] { $count } słowa.
    }
analysis-median-line = Mediana: { $count ->
        [one] { $count } słowo
        [few] { $count } słowa
        [many] { $count } słów
       *[other] { $count } słowa
    }
analysis-dialogue = Dialog
analysis-dialogue-unsupported = Dialog nie jest jeszcze mierzony dla tego języka.

analysis-footnote-words = Słowa w przypisach
analysis-footnote-words-count = { $count ->
        [one] W przypisach tej książki jest { $count } słowo, liczone oddzielnie od sumy rękopisu.
        [few] W przypisach tej książki są { $count } słowa, liczone oddzielnie od sumy rękopisu.
        [many] W przypisach tej książki jest { $count } słów, liczonych oddzielnie od sumy rękopisu.
       *[other] W przypisach tej książki jest { $count } słowa, liczone oddzielnie od sumy rękopisu.
    }
analysis-footnote-words-pending = Liczenie przypisów…

analysis-ignore-empty = Pomijaj teksty, w których nic jeszcze nie napisano
analysis-empty-hidden = Ukryto { $count ->
        [one] { $count } pusty tekst
        [few] { $count } puste teksty
        [many] { $count } pustych tekstów
       *[other] { $count } pustego tekstu
    }.
analysis-all-texts-empty = Wszystkie teksty w tej książce są jeszcze puste.

binder-filter-count = Wyświetlono { $shown } z { $total }
binder-filter-clear = Wyczyść
binder-filter-none = Nic nie pasuje do „{ $query }”.

settings-paratext-intro = Elementy przedtekstowe i zatekstowe, od których może zacząć nowy projekt. Każda struktura należy do pewnej tradycji wydawniczej, a tytuły jej stron są zapisane w języku tej tradycji. Po utworzeniu projektu możesz je swobodnie zmieniać.
settings-paratext-structures = Struktury
settings-paratext-broken = Nie można odczytać
settings-paratext-edit = Edytuj
settings-paratext-duplicate = Duplikuj
settings-paratext-delete = Usuń
settings-paratext-new = Nowa struktura
settings-paratext-save = Zapisz
settings-paratext-editor-hint = Nazwa, strony tworzone przed rękopisem i strony tworzone po nim. Gdzie trafią potem, zależy od Ciebie: po utworzeniu projektu są zwykłymi elementami.


image-insert = Wstaw &obraz…
image-no-project = Otwórz projekt przed wstawieniem obrazu.
image-choose-title = Wybierz obraz
image-filter-label = Obrazy
image-large-title = To duży obraz
image-large-text =
    { $name } ma { $megapixels } megapikseli ({ $width }×{ $height }).
    Zachowanie go bez zmian przechowuje w projekcie oryginalny plik: trafia on
    do każdej kopii zapasowej i każdego eksportu. Optymalizacja przechowuje zamiast niego
    mniejszą kopię, do 2560 pikseli na dłuższym boku.
image-large-keep = Zachowaj oryginał
image-large-downscale = Optymalizuj
image-large-remember = Rób tak od teraz i nie pytaj ponownie
image-not-recorded = Obraz został zapisany, ale nie udało się go zarejestrować w projekcie.
image-describe-title = Opisz obraz
image-describe-explain = Co przedstawia obraz, dla czytelnika, który go nie widzi. Opis nie jest częścią rękopisu: nigdy nie jest liczony, przeszukiwany ani eksportowany jako proza.
image-describe-placeholder = latarnia morska na tle szarego nieba
image-resize-title = Zmień rozmiar obrazu
image-resize-explain = Procent obecnie wyświetlanego rozmiaru. 100 pozostawia go bez zmian.
image-resize-invalid = Wprowadź liczbę od 1 do 1000.
image-menu-describe = &Opisz obraz…
image-menu-resize = &Zmień rozmiar obrazu…
image-menu-reset-size = Oryginalny &rozmiar

cover-heading = Okładka
cover-pick = Wybierz…
cover-change = Zmień…
cover-remove = Usuń
cover-none = Brak okładki
cover-missing = Brakuje pliku obrazu w projekcie
cover-choose-title = Wybierz okładkę
cover-set = Okładka ustawiona.
cover-cleared = Okładka została usunięta. Obraz jest nadal w projekcie.
card-label = Etykieta
card-exportable = Eksportowany
card-numbered = Numerowany
card-created = Utworzono
card-modified = Zmodyfikowano
card-yes = tak
card-no = nie
card-type = Typ
card-position = Pozycja
card-children = Elementy podrzędne
card-goal = Cel
card-words = Słowa
card-synopsis = Streszczenie
card-point-of-view = Punkt widzenia
card-aliases = Inne nazwy

export-orphan-footnotes-title = Przypisy bez odwołań
export-comments-dropped =
    { $count ->
        [one] Jednego komentarza nie udało się umieścić w eksportowanym tekście, więc został pominięty.
        [few] { $count } komentarzy nie udało się umieścić w eksportowanym tekście, więc zostały pominięte.
        [many] { $count } komentarzy nie udało się umieścić w eksportowanym tekście, więc zostały pominięte.
       *[other] { $count } komentarza nie udało się umieścić w eksportowanym tekście, więc zostały pominięte.
    }
export-orphan-footnotes =
    { $count ->
        [one] Do jednego przypisu nie ma już odwołania w rękopisie. Jego tekst nie pojawi się w wyeksportowanej książce.
        [few] Do { $count } przypisów nie ma już odwołań w rękopisie. Ich tekst nie pojawi się w wyeksportowanej książce.
        [many] Do { $count } przypisów nie ma już odwołań w rękopisie. Ich tekst nie pojawi się w wyeksportowanej książce.
       *[other] Do { $count } przypisu nie ma już odwołań w rękopisie. Ich tekst nie pojawi się w wyeksportowanej książce.
    }


menu-footnotes = &Przypisy
footnotes-title = Przypisy
footnotes-insert = Wstaw prz&ypis
footnotes-empty = Brak przypisów. Umieść kursor w scenie, a następnie użyj przycisku + powyżej lub skrótu Ctrl+Alt+F.
footnotes-filter-all = Wszystkie
footnotes-filter-document = Ten dokument
footnotes-filter-orphaned = Osierocone
footnotes-orphaned = Nic już nie wskazuje na ten przypis
footnotes-untitled-home = Bez tytułu
footnotes-body-placeholder = treść przypisu
footnotes-insert-tooltip = Wstaw przypis w miejscu kursora (Ctrl+Alt+F)
footnotes-actions = Działania przypisu
footnotes-delete = &Usuń przypis i jego odwołanie
footnotes-no-project = Otwórz projekt przed wstawieniem przypisu.
footnotes-no-caret = Umieść kursor w tekście sceny, aby wstawić tam przypis.
footnotes-not-created = Nie udało się dodać przypisu do projektu.
footnotes-deleted-toast = Przypis usunięty
footnotes-undo-delete = Cofnij

import-document-title = Importuj dokumenty
import-document-close = Zamknij
import-document-step-files = Pliki
import-document-step-review = Sprawdzenie
import-document-step-destination = Miejsce docelowe
import-document-step-reconcile = Scal
import-document-reconcile-hint = Niektóre z tych elementów już masz. Wskaż, co ma się stać z każdym z nich.
import-document-reconcile-all-new = Nic w tym pliku nie pasuje do Twojego projektu: każdy element zostanie dodany jako nowy.
import-document-duplicate-returns = Te pliki to kilka kopii tego samego powracającego rękopisu. Importuj je pojedynczo: dopasowanie zestawia z projektem tylko jedną powracającą kopię, więc import kilku naraz dodałby drugą kopię książki zamiast ją scalić.
import-document-hunk-added = Przyjmij ten nowy akapit
import-document-hunk-removed = Usuń ten akapit
import-document-hunk-changed = Przyjmij tę zmienioną wersję
import-document-col-stray-prose = Jego tekst
import-document-stray-as-paratext = Zachowaj jako paratekst
import-document-stray-discard = Odrzuć tekst
import-document-col-current = W projekcie
import-document-col-incoming = W tym pliku
import-document-col-status = Stan
import-document-col-action = Co zrobić
import-document-status-identical = Bez różnic
import-document-status-editor-edited = Zmieniono w pliku
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
import-document-compare-legend = Twoja wersja w porównaniu z wersją z tego pliku. Tylko do odczytu.
import-document-compare-close = Zamknij
import-document-drop-title = Upuść tutaj dokumenty
import-document-drop-hint = Markdown (.md), Word (.docx), OpenDocument (.odt) i zwykły tekst (.txt)
import-document-browse = Przeglądaj…
import-document-move-up = Przesuń w górę
import-document-move-down = Przesuń w dół
import-document-remove-file = Usuń
import-document-file-count = { $count ->
        [one] { $count } plik
        [few] { $count } pliki
        [many] { $count } plików
       *[other] { $count } pliku
    }
import-document-no-files = Nie wybrano jeszcze żadnych plików.
import-document-col-included = Importuj
import-document-col-title = Tytuł
import-document-col-type = Typ
import-document-col-words = Słowa
import-document-col-breaks = Separatory
import-document-col-comments = Komentarze
import-document-col-footnotes = Przypisy
import-document-col-epigraph = Motto
import-document-col-source = Źródło
import-document-level-rules = Poziomy nagłówków
import-document-level-n = Nagłówek { $level }
import-document-add-top-level = Dodaj najwyższy poziom
import-document-add-top-level-tooltip = Wstaw książkę nad wszystkimi przeanalizowanymi elementami, dla plików rozdziałów bez nagłówka książki
import-document-destination = Miejsce docelowe
import-document-destination-hint = Wybierz segregator lub element: nowe elementy trafiają do wnętrza folderu albo za scenę.
import-document-destination-empty = Nie ma jeszcze segregatorów. Najpierw utwórz segregator.
import-document-plan-empty = Na razie nie ma czego importować.
import-document-pictures = { $count ->
        [1] Razem z tymi dokumentami trafi do projektu jeden obraz.
        [few] Razem z tymi dokumentami trafią do projektu { $count } obrazy.
        [many] Razem z tymi dokumentami trafi do projektu { $count } obrazów.
       *[other] Razem z tymi dokumentami trafi do projektu { $count } obrazu.
    }
import-document-summary = { $rows ->
        [one] { $rows } element
        [few] { $rows } elementy
        [many] { $rows } elementów
       *[other] { $rows } elementu
    } · { $breaks ->
        [one] { $breaks } separator sceny
        [few] { $breaks } separatory scen
        [many] { $breaks } separatorów scen
       *[other] { $breaks } separatora scen
    }
import-document-back = Wstecz
import-document-cancel = Anuluj
import-document-analyse = Dalej
import-document-analysing = Czytanie dokumentów…
import-document-step-analysing = Czytanie
import-document-cancel-analysis = Przerwij czytanie
import-document-analyse-failed = Nie udało się odczytać dokumentów.
import-document-details = Szczegóły
import-document-import = Importuj
import-document-done = { $count ->
        [one] Zaimportowano { $count } element
        [few] Zaimportowano { $count } elementy
        [many] Zaimportowano { $count } elementów
       *[other] Zaimportowano { $count } elementu
    }
import-document-undo = Cofnij
import-diagnostic-file-unreadable = Nie można odczytać „{ $path }”: { $detail }. Pozostałe pliki zostaną zaimportowane.
import-diagnostic-nested-too-deep = Nie odczytano „{ $path }”: jego zawartość jest zagnieżdżona na ponad { $limit } poziomów. Żaden prawdziwy dokument nawet się do tego nie zbliża, a jego odczytanie mogłoby spowodować awarię Skribisto, więc został pominięty. Pozostałe pliki zostaną zaimportowane.
import-diagnostic-archive-too-large = Nie odczytano „{ $path }”: to skompresowany plik spreparowany tak, by po rozpakowaniu był wielokrotnie większy, co mogłoby wyczerpać pamięć i spowodować awarię Skribisto, więc został pominięty. Pozostałe pliki zostaną zaimportowane.
import-diagnostic-lossy-decode = { $count ->
        [one] Nie udało się zdekodować jednego znaku w „{ $path }”. Zapisz plik ponownie w kodowaniu UTF-8, aby go zachować.
        [few] Nie udało się zdekodować { $count } znaków w „{ $path }”. Zapisz plik ponownie w kodowaniu UTF-8, aby je zachować.
        [many] Nie udało się zdekodować { $count } znaków w „{ $path }”. Zapisz plik ponownie w kodowaniu UTF-8, aby je zachować.
       *[other] Nie udało się zdekodować { $count } znaku w „{ $path }”. Zapisz plik ponownie w kodowaniu UTF-8, aby je zachować.
    }
import-diagnostic-decoded-from-bom = „{ $path }” zdekodowano jako { $detail }, a nie UTF-8.
import-diagnostic-empty-file = „{ $path }” jest pusty.
import-diagnostic-no-headings = „{ $path }” nie ma nagłówków, więc trafia do projektu jako jeden element.
import-diagnostic-unsupported-format = Pliki „.{ $detail }” nie są obsługiwane, więc pominięto „{ $path }”.
import-diagnostic-front-matter-not-flat = Metadane w „{ $path }”: „{ $detail }” nie jest prostą wartością i zostało pominięte.
import-diagnostic-footnotes-degraded = { $count ->
        [one] Jeden przypis w „{ $path }” trafia do projektu jako zwykły tekst: przypisy nie są odczytywane z Markdown.
        [few] { $count } przypisy w „{ $path }” trafiają do projektu jako zwykły tekst: przypisy nie są odczytywane z Markdown.
        [many] { $count } przypisów w „{ $path }” trafia do projektu jako zwykły tekst: przypisy nie są odczytywane z Markdown.
       *[other] { $count } przypisu w „{ $path }” trafia do projektu jako zwykły tekst: przypisy nie są odczytywane z Markdown.
    }
import-diagnostic-footnote-not-carried = { $count ->
        [one] Nie udało się przenieść jednego przypisu z „{ $path }”. Pozostałe zostały przeniesione.
        [few] Nie udało się przenieść { $count } przypisów z „{ $path }”. Pozostałe zostały przeniesione.
        [many] Nie udało się przenieść { $count } przypisów z „{ $path }”. Pozostałe zostały przeniesione.
       *[other] Nie udało się przenieść { $count } przypisu z „{ $path }”. Pozostałe zostały przeniesione.
    }
import-diagnostic-raw-html-dropped = { $count ->
        [one] Pominięto jeden blok surowego HTML w „{ $path }”.
        [few] Pominięto { $count } bloki surowego HTML w „{ $path }”.
        [many] Pominięto { $count } bloków surowego HTML w „{ $path }”.
       *[other] Pominięto { $count } bloku surowego HTML w „{ $path }”.
    }
import-diagnostic-nested-break-dropped = { $count ->
        [one] Pominięto jeden separator sceny wewnątrz cytatu lub listy w „{ $path }”. Zachowywany jest tylko separator w osobnym wierszu.
        [few] Pominięto { $count } separatory scen wewnątrz cytatów lub list w „{ $path }”. Zachowywany jest tylko separator w osobnym wierszu.
        [many] Pominięto { $count } separatorów scen wewnątrz cytatów lub list w „{ $path }”. Zachowywany jest tylko separator w osobnym wierszu.
       *[other] Pominięto { $count } separatora scen wewnątrz cytatów lub list w „{ $path }”. Zachowywany jest tylko separator w osobnym wierszu.
    }
import-diagnostic-image-not-ingested-format = W „{ $path }” obraz „{ $detail }” jest w formacie, którego Skribisto nie potrafi wyświetlić, więc został pominięty. Importowane są obrazy PNG, JPEG, WebP, GIF, TIFF i BMP; zapisz go w jednym z tych formatów i wstaw ręcznie.
import-diagnostic-image-not-ingested-remote = W „{ $path }” obraz „{ $detail }” znajduje się w internecie. Podczas importu obrazy nigdy nie są pobierane, więc został pominięty.
import-diagnostic-image-not-ingested-outside = W „{ $path }” obraz „{ $detail }” leży poza dokumentem i jego folderem, więc został pominięty. Importowane są tylko obrazy zapisane w dokumencie lub we własnym folderze dokumentu.
import-diagnostic-image-not-ingested-missing = W „{ $path }” nie udało się znaleźć obrazu „{ $detail }”, więc został pominięty.
import-diagnostic-image-not-ingested-too-large = W „{ $path }” obraz „{ $detail }” jest większy, niż może przyjąć projekt, więc został pominięty.
import-diagnostic-image-not-ingested-unreadable = W „{ $path }” nie udało się odczytać „{ $detail }” jako obrazu, więc został pominięty.
import-diagnostic-image-not-ingested-heading = W „{ $path }” obraz „{ $detail }” znajduje się w nagłówku. Każdy nagłówek staje się tytułem elementu w segregatorze, a tytuł nie może zawierać obrazu, więc obraz został pominięty.
import-diagnostic-image-not-ingested-note = W „{ $path }” obraz „{ $detail }” znajduje się w przypisie dolnym, przypisie końcowym lub komentarzu. Zachowują one słowa, ale nie obrazy, więc obraz został pominięty.
import-diagnostic-image-not-ingested-html = W „{ $path }” obraz „{ $detail }” jest zapisany jako HTML, a import pomija HTML, więc obraz został pominięty razem z nim. Zapisz go jako obraz Markdown, aby go zaimportować.
import-diagnostic-image-not-ingested-document-full = W „{ $path }” pominięto obraz „{ $detail }”: obrazy tego dokumentu łącznie przekraczają to, co może przenieść jeden import. Zaimportuj dokument fragmentami albo wstaw pozostałe obrazy ręcznie.
import-diagnostic-picture-first-frame-only = W „{ $path }” animowany obraz „{ $detail }” trafia do projektu jako swoja pierwsza klatka. Obrazy w projekcie się nie poruszają.
import-diagnostic-duplicate-title = „{ $title }” występuje { $count } razy. Jeśli te pliki były już importowane, zostaną zduplikowane.
import-diagnostic-heading-level-jump = „{ $title }” przeskakuje z poziomu nagłówka { $from } na { $to }; zostaje umieszczony jeden poziom pod elementem nadrzędnym.
import-diagnostic-illegal-combination = „{ $title }” zawiera prozę, ale typ „{ $kind }” nie może jej przechowywać. Import jest wstrzymany, dopóki nie zmienisz typu lub nie wyłączysz tego elementu. W przeciwnym razie nic nie zostanie zaimportowane.
import-diagnostic-tracked-changes-flattened = Plik { $path } był w trakcie redakcji: { $count ->
        [one] przyjęto { $count } śledzoną zmianę
        [few] przyjęto { $count } śledzone zmiany
        [many] przyjęto { $count } śledzonych zmian
       *[other] przyjęto { $count } śledzonej zmiany
    }, a usunięcia zostały wykonane. To jest tekst ostateczny. Aby zamiast tego zachować którekolwiek własne sformułowania, użyj porównania w ostatnim kroku.
import-diagnostic-tracked-changes-flattened-by = Plik { $path } był w trakcie redakcji: { $count ->
        [one] przyjęto { $count } śledzoną zmianę autorstwa { $names }
        [few] przyjęto { $count } śledzone zmiany autorstwa { $names }
        [many] przyjęto { $count } śledzonych zmian autorstwa { $names }
       *[other] przyjęto { $count } śledzonej zmiany autorstwa { $names }
    }, a usunięcia zostały wykonane. To jest tekst ostateczny. Aby zamiast tego zachować którekolwiek własne sformułowania, użyj porównania w ostatnim kroku.
import-diagnostic-text-box-dropped = { $count ->
        [one] { $path } zawiera { $count } pole tekstowe. Jego tekst leży poza głównym biegiem dokumentu, więc nie da się ustalić, gdzie należy w rękopisie. Nie zostało zaimportowane.
        [few] { $path } zawiera { $count } pola tekstowe. Ich tekst leży poza głównym biegiem dokumentu, więc nie da się ustalić, gdzie należy w rękopisie. Nie zostały zaimportowane.
        [many] { $path } zawiera { $count } pól tekstowych. Ich tekst leży poza głównym biegiem dokumentu, więc nie da się ustalić, gdzie należy w rękopisie. Nie zostały zaimportowane.
       *[other] { $path } zawiera { $count } pola tekstowego. Ich tekst leży poza głównym biegiem dokumentu, więc nie da się ustalić, gdzie należy w rękopisie. Nie zostały zaimportowane.
    }
import-diagnostic-embedded-object-dropped = { $count ->
        [one] { $path } zawiera { $count } osadzony obiekt: wykres, równanie lub coś podobnego. Nic w rękopisie nie mogłoby go pomieścić.
        [few] { $path } zawiera { $count } osadzone obiekty: wykresy, równania lub coś podobnego. Nic w rękopisie nie mogłoby ich pomieścić.
        [many] { $path } zawiera { $count } osadzonych obiektów: wykresy, równania lub coś podobnego. Nic w rękopisie nie mogłoby ich pomieścić.
       *[other] { $path } zawiera { $count } osadzonego obiektu: wykresy, równania lub coś podobnego. Nic w rękopisie nie mogłoby ich pomieścić.
    }
import-diagnostic-field-flattened = { $count ->
        [one] { $path } zawiera { $count } pole: numer strony, odsyłacz, datę. Zachowuje ono ostatnio wyświetlany tekst i nie będzie już aktualizowane.
        [few] { $path } zawiera { $count } pola: numery stron, odsyłacze, daty. Każde zachowuje ostatnio wyświetlany tekst i nie będzie już aktualizowane.
        [many] { $path } zawiera { $count } pól: numery stron, odsyłacze, daty. Każde zachowuje ostatnio wyświetlany tekst i nie będzie już aktualizowane.
       *[other] { $path } zawiera { $count } pola: numery stron, odsyłacze, daty. Każde zachowuje ostatnio wyświetlany tekst i nie będzie już aktualizowane.
    }
import-diagnostic-unknown-style-level = { $path } używa stylu „{ $detail }”, który wygląda jak nagłówek, ale nie określa poziomu. Te akapity są importowane jako proza, zamiast zgadywać ich głębokość.
import-diagnostic-comment-unanchored = Komentarza „{ $detail }” w { $path } nie udało się przypiąć do słów, których dotyczył. Pozostaje przy swoim elemencie, skąd można go przenieść.
import-diagnostic-comment-not-carried = Komentarz „{ $detail }” w { $path } nie został zaimportowany, ponieważ nic w pliku nie trafia do projektu jako tekst, przy którym mógłby zostać. Jeśli go potrzebujesz, skopiuj go z pliku.
import-diagnostic-comment-replies-flattened = { $count ->
        [one] { $count } odpowiedź w { $path } wskazuje komentarz, którego nie ma w pliku, więc trafia do projektu jako osobny komentarz.
        [few] { $count } odpowiedzi w { $path } wskazują komentarz, którego nie ma w pliku, więc trafiają do projektu jako osobne komentarze.
        [many] { $count } odpowiedzi w { $path } wskazuje komentarz, którego nie ma w pliku, więc trafiają do projektu jako osobne komentarze.
       *[other] { $count } odpowiedzi w { $path } wskazuje komentarz, którego nie ma w pliku, więc trafiają do projektu jako osobne komentarze.
    }
import-diagnostic-prose-not-verbatim = { $count ->
        [one] Nie udało się przenieść { $count } akapitu z { $path } z formatowaniem dokładnie takim, jak w oryginale, więc trafia do projektu jako zwykły tekst. Porównaj go z oryginałem.
        [few] Nie udało się przenieść { $count } akapitów z { $path } z formatowaniem dokładnie takim, jak w oryginale, więc trafiają do projektu jako zwykły tekst. Porównaj je z oryginałem.
        [many] Nie udało się przenieść { $count } akapitów z { $path } z formatowaniem dokładnie takim, jak w oryginale, więc trafiają do projektu jako zwykły tekst. Porównaj je z oryginałem.
       *[other] Nie udało się przenieść { $count } akapitu z { $path } z formatowaniem dokładnie takim, jak w oryginale, więc trafiają do projektu jako zwykły tekst. Porównaj je z oryginałem.
    }
import-diagnostic-styled-spaces-not-carried = { $count ->
        [one] W { $path } { $count } ciąg pustych spacji był podkreślony lub przekreślony, np. linia do wypełnienia. Skribisto nie zachowuje takich linii na samych spacjach, więc trafia on do projektu jako zwykłe spacje albo wcale, jeśli zaczyna lub kończy akapit.
        [few] W { $path } { $count } ciągi pustych spacji były podkreślone lub przekreślone, np. linie do wypełnienia. Skribisto nie zachowuje takich linii na samych spacjach, więc trafiają one do projektu jako zwykłe spacje albo wcale, jeśli zaczynają lub kończą akapit.
        [many] W { $path } { $count } ciągów pustych spacji było podkreślonych lub przekreślonych, np. linie do wypełnienia. Skribisto nie zachowuje takich linii na samych spacjach, więc trafiają one do projektu jako zwykłe spacje albo wcale, jeśli zaczynają lub kończą akapit.
       *[other] W { $path } { $count } ciągu pustych spacji było podkreślonych lub przekreślonych, np. linie do wypełnienia. Skribisto nie zachowuje takich linii na samych spacjach, więc trafiają one do projektu jako zwykłe spacje albo wcale, jeśli zaczynają lub kończą akapit.
    }
import-diagnostic-list-nesting-flattened = { $count ->
        [one] { $count } element listy w { $path } był zagnieżdżony głębiej niż { $limit } poziomów. Trafia na poziom { $limit }, najgłębszy, jaki zachowuje Skribisto, wraz ze słowami i formatowaniem.
        [few] { $count } elementy listy w { $path } były zagnieżdżone głębiej niż { $limit } poziomów. Trafiają na poziom { $limit }, najgłębszy, jaki zachowuje Skribisto, wraz ze słowami i formatowaniem.
        [many] { $count } elementów listy w { $path } było zagnieżdżonych głębiej niż { $limit } poziomów. Trafiają na poziom { $limit }, najgłębszy, jaki zachowuje Skribisto, wraz ze słowami i formatowaniem.
       *[other] { $count } elementu listy w { $path } było zagnieżdżonych głębiej niż { $limit } poziomów. Trafiają na poziom { $limit }, najgłębszy, jaki zachowuje Skribisto, wraz ze słowami i formatowaniem.
    }
import-diagnostic-table-read-as-paragraphs = { $count ->
        [one] { $count } tabela w { $path } ma wiersz znacznie szerszy od pozostałych, zbyt nieregularny, by ułożyć go w siatkę. Jej komórki trafiają do projektu jako kolejne akapity, wraz ze słowami i formatowaniem.
        [few] { $count } tabele w { $path } mają wiersz znacznie szerszy od pozostałych, zbyt nieregularny, by ułożyć je w siatkę. Ich komórki trafiają do projektu jako kolejne akapity, wraz ze słowami i formatowaniem.
        [many] { $count } tabel w { $path } ma wiersz znacznie szerszy od pozostałych, zbyt nieregularny, by ułożyć je w siatkę. Ich komórki trafiają do projektu jako kolejne akapity, wraz ze słowami i formatowaniem.
       *[other] { $count } tabeli w { $path } ma wiersz znacznie szerszy od pozostałych, zbyt nieregularny, by ułożyć je w siatkę. Ich komórki trafiają do projektu jako kolejne akapity, wraz ze słowami i formatowaniem.
    }
import-diagnostic-spaces-shortened = { $count ->
        [one] { $count } ciąg spacji w { $path } był dłuższy niż { $limit } spacji i zostaje skrócony do { $limit }. Wszystkie słowa wokół niego są zachowane.
        [few] { $count } ciągi spacji w { $path } były dłuższe niż { $limit } spacji i zostają skrócone do { $limit }. Wszystkie słowa wokół nich są zachowane.
        [many] { $count } ciągów spacji w { $path } było dłuższych niż { $limit } spacji i zostają skrócone do { $limit }. Wszystkie słowa wokół nich są zachowane.
       *[other] { $count } ciągu spacji w { $path } było dłuższych niż { $limit } spacji i zostają skrócone do { $limit }. Wszystkie słowa wokół nich są zachowane.
    }
import-diagnostic-epigraph-not-carried = „{ $title }” zaczyna się od motta, ale typ „{ $kind }” nie może go przechowywać. Cytat zostaje zamiast tego zachowany na początku tekstu.
import-diagnostic-epigraph-placement-ambiguous = Motto znajduje się między „{ $title }” a „{ $below }” i może należeć do każdego z nich. Przypisano je do „{ $title }”, bo tam zwykle umieszcza się motto.
import-document-diagnostics = { $errors ->
        [0] { $warnings ->
            [one] { $warnings } uwaga
            [few] { $warnings } uwagi
            [many] { $warnings } uwag
           *[other] { $warnings } uwagi
        }
       *[other] { $errors ->
            [one] Nie można odczytać { $errors } pliku
            [few] Nie można odczytać { $errors } plików
            [many] Nie można odczytać { $errors } plików
           *[other] Nie można odczytać { $errors } pliku
        }
    }
import-document-diagnostics-none = Nic do zgłoszenia.

versions-title = Wersje
versions-scope-synopsis = Streszczenie
versions-scope-prose = Tekst
versions-loading = Przeglądanie kopii zapasowych…
versions-empty = Brak jeszcze wcześniejszej wersji
versions-error = Nie udało się odczytać kopii zapasowych. Nic nie zostało utracone, ale ta lista może być niepełna
versions-did-not-exist = Jeszcze nie istniało w dniu { $date }
versions-deleted-after = Usunięto jakiś czas po { $date }
versions-unreadable = { $count ->
        [one] Nie udało się odczytać { $count } kopii zapasowej
        [few] Nie udało się odczytać { $count } kopii zapasowych
        [many] Nie udało się odczytać { $count } kopii zapasowych
       *[other] Nie udało się odczytać { $count } kopii zapasowej
    }
versions-thinned = { $count ->
        [one] Starsze wersje z czasem się przerzedzają. Własna historia projektu nie przechowuje już { $count } wcześniejszego stanu tego tekstu.
        [few] Starsze wersje z czasem się przerzedzają. Własna historia projektu nie przechowuje już { $count } wcześniejszych stanów tego tekstu.
        [many] Starsze wersje z czasem się przerzedzają. Własna historia projektu nie przechowuje już { $count } wcześniejszych stanów tego tekstu.
       *[other] Starsze wersje z czasem się przerzedzają. Własna historia projektu nie przechowuje już { $count } wcześniejszego stanu tego tekstu.
    }
versions-source-backup = Z kopii zapasowej
versions-source-project = Z własnej historii projektu
versions-list-caption = Jeden wpis na każdą zmianę, nie na każdą kopię zapasową
versions-pick-a-version = Wybierz wersję, aby zobaczyć, co się zmieniło
versions-earliest = Najwcześniejsza zarejestrowana wersja. Nie ma nic starszego do porównania.
versions-no-change = W tej części elementu nic się nie zmieniło
versions-formatting-only = Zmieniło się tu tylko formatowanie. Słowa są te same
versions-show-unchanged = Pokaż niezmienione
versions-hide-unchanged = Ukryj niezmienione
versions-next-change = Następna zmiana
versions-near = w pobliżu „{ $text }”
versions-words-added = { $count ->
        [one] Dodano { $count } słowo
        [few] Dodano { $count } słowa
        [many] Dodano { $count } słów
       *[other] Dodano { $count } słowa
    }
versions-words-removed = { $count ->
        [one] Usunięto { $count } słowo
        [few] Usunięto { $count } słowa
        [many] Usunięto { $count } słów
       *[other] Usunięto { $count } słowa
    }
versions-blocks-moved = { $count ->
        [one] Przeniesiono { $count } akapit
        [few] Przeniesiono { $count } akapity
        [many] Przeniesiono { $count } akapitów
       *[other] Przeniesiono { $count } akapitu
    }
versions-pin = Przypnij kopię zapasową, z której pochodzi ta wersja. Automatyczne czyszczenie nigdy jej nie usunie
versions-unpin = Odepnij kopię zapasową, z której pochodzi ta wersja. Automatyczne czyszczenie znów będzie mogło ją usunąć
versions-pinned-only = Pokaż tylko przypięte wersje
versions-pin-note = Przypinać można tylko wersje z kopii zapasowych. Przypięcie zachowuje plik, a własna historia projektu nie jest plikiem.
versions-pinned-empty = Nic tu jeszcze nie jest przypięte
versions-pinned-empty-log = Nic tu jeszcze nie jest przypięte. Przypinać można tylko wersje z kopii zapasowych. Przypięcie zachowuje plik, a własna historia projektu nie jest plikiem.
versions-range-filter = Pokaż tylko wersje z okresu między dwiema datami
versions-filtered-empty = Żadna wersja nie pasuje do ustawionych filtrów
versions-clear-filters = Wyczyść filtry
versions-last-30-days = Ostatnie 30 dni
versions-restore-button = Przywróć tę wersję
versions-restore-confirm-title = Zastąpić ten tekst wersją z { $date }?
versions-restore-confirm-text = To, co masz teraz, zostanie zastąpione tekstem, który ten element miał w dniu { $date }.
versions-restore-confirm-with-comments = To, co masz teraz, zostanie zastąpione tekstem, który ten element miał w dniu { $date }. { $count ->
        [one] { $count } komentarz jest zakotwiczony w bieżącym tekście i może zostać osierocony.
        [few] { $count } komentarze są zakotwiczone w bieżącym tekście i mogą zostać osierocone.
        [many] { $count } komentarzy jest zakotwiczonych w bieżącym tekście i może zostać osieroconych.
       *[other] { $count } komentarza jest zakotwiczonych w bieżącym tekście i może zostać osieroconych.
    }
versions-restore-confirm-undo-note = Najpierw tworzona jest kopia zapasowa, a Ctrl+Z cofa całą operację jednym krokiem.
versions-restored-toast = Przywrócono wersję z { $date }
versions-undo = Cofnij
versions-restore-row-gone = Tego elementu nie ma już w projekcie
versions-restore-no-home = Ten element zmienił od tamtej pory typ i stary tekst nie ma w nim swojego miejsca
versions-restore-no-safety-copy = Kopia bezpieczeństwa nie została utworzona, więc nic nie zmieniono
versions-restore-failed = Przywracanie nie powiodło się: { $error }
versions-restore-backup-busy = Tworzenie kopii zapasowej już trwa. Spróbuj ponownie za chwilę
versions-restore-in-backup-file = Przeglądasz plik kopii zapasowej; aby przywrócić wersję, otwórz sam projekt
versions-restore-no-project = Żaden projekt nie jest otwarty
versions-recreate-button = Przywróć ten element…
versions-recreate-picker-title = Gdzie go umieścić?
versions-recreate-picker-empty = Ten projekt nie ma segregatora, w którym można by go umieścić
versions-recreate-picker-confirm = Przywróć tutaj
versions-recreate-picker-cancel = Anuluj
versions-recreate-untitled = ten element
versions-recreate-confirm-title = Przywrócić „{ $item }”?
versions-recreate-confirm-text = Zostanie dodany do { $destination } z tekstem, który miał w dniu { $date }.
versions-recreate-confirm-undo-note = Przycisk „Cofnij” w komunikacie, który się pojawi, od razu go usunie.
versions-recreated-toast = „{ $item }” wrócił do projektu
versions-recreated-partial-toast = { $count ->
        [one] „{ $item }” wrócił, ale jednego z jego tekstów nie udało się odczytać
        [few] „{ $item }” wrócił, ale { $count } z jego tekstów nie udało się odczytać
        [many] „{ $item }” wrócił, ale { $count } z jego tekstów nie udało się odczytać
       *[other] „{ $item }” wrócił, ale { $count } z jego tekstów nie udało się odczytać
    }
versions-recreate-already-here = Ten element już jest w projekcie
versions-recreate-no-destination = Wybierz miejsce w segregatorze, w którym go umieścić
versions-recreate-unreadable = Nie udało się odczytać tej kopii zapasowej, więc nic nie dodano
versions-recreate-failed = Nie udało się go przywrócić: { $error }
versions-changed-percent = Zmieniono { $percent }%
versions-hidden-paragraphs = { $count ->
        [one] … { $count } niezmieniony akapit …
        [few] … { $count } niezmienione akapity …
        [many] … { $count } niezmienionych akapitów …
       *[other] … { $count } niezmienionego akapitu …
    }

timeline-title = Cofnij się w czasie
timeline-coverage = { $count ->
        [one] Zarejestrowano { $count } wersję, sięgającą do { $oldest }
        [few] Zarejestrowano { $count } wersje, sięgające do { $oldest }
        [many] Zarejestrowano { $count } wersji, sięgających do { $oldest }
       *[other] Zarejestrowano { $count } wersji, sięgających do { $oldest }
    }
timeline-loading = Przeglądanie historii projektu…
timeline-empty = Nie zarejestrowano jeszcze żadnej wersji tego projektu
timeline-no-changes = Od tamtej pory nic się nie zmieniło
timeline-no-text-changes = Od tamtej pory żaden tekst się nie zmienił
timeline-slider-label = Zarejestrowana wersja
timeline-series-name = Rozmiar projektu
timeline-bars-caption = Każdy słupek to zarejestrowana wersja, o wysokości odpowiadającej ówczesnemu rozmiarowi projektu. Wyróżniony słupek to wersja, którą oglądasz.
timeline-bars-caption-periods = Wersji jest zbyt wiele, by pokazać je pojedynczo, więc każdy słupek to { $period }, o wysokości odpowiadającej rozmiarowi projektu pod jego koniec.
timeline-unit-hour = godzina
timeline-unit-day = dzień
timeline-unit-week = tydzień
timeline-unit-month = miesiąc
timeline-range-filter = Pokaż tylko wersje z okresu między dwiema datami
timeline-range-empty = W tym okresie nie zarejestrowano żadnej wersji
timeline-open-period = Otwórz ten okres
timeline-show-all = Pokaż całą historię
timeline-changed-since = { $count ->
        [one] { $count } element różni się między { $date } a obecnym stanem projektu
        [few] { $count } elementy różnią się między { $date } a obecnym stanem projektu
        [many] { $count } elementów różni się między { $date } a obecnym stanem projektu
       *[other] { $count } elementu różni się między { $date } a obecnym stanem projektu
    }
timeline-kind-added = Napisane od tamtej pory
timeline-kind-removed = Już nie ma w projekcie
timeline-kind-changed = Edytowane od tamtej pory
timeline-kind-moved = Przeniesione od tamtej pory
timeline-not-yet-written = Ten element jeszcze wtedy nie istniał
timeline-no-text-of-its-own = Ten element nie ma własnego tekstu. To nagłówek dla tego, co zawiera
timeline-reader-close = Zamknij
timeline-reader-stamp = Stan z dnia { $date }
timeline-reader-loading = Otwieranie zarejestrowanej wersji…
timeline-reader-unreadable = Nie udało się odczytać tego zapisu. Kopia zapasowa mogła zostać przeniesiona lub usunięta albo znajduje się na niepodłączonym dysku.
timeline-reader-compared = Stan z dnia { $date } zestawiony z obecnym
timeline-reader-diff-legend = Przekreślone zniknęło od tamtej pory; podkreślone zostało dodane.
timeline-reader-view-label = Co pokazać
timeline-reader-view-diff = Zmiany
timeline-reader-view-text = Tekst
timeline-reader-deleted = Tego elementu nie ma już w projekcie. Możesz go tu przeczytać i skopiować.
timeline-prose-only-record = Ten punkt pochodzi z własnej historii projektu, która przechowuje tylko tekst, więc nie można pokazać, co od tamtej pory usunięto lub przeniesiono. Do tego wybierz kopię zapasową.

settings-sec-extensions = Rozszerzenia

settings-group-games-forward = Zawsze do przodu
settings-games-forward-toggle = Graj w „Zawsze do przodu”
settings-games-forward-hint =
    Podczas gry niczego, co napisano, nie można cofnąć: Backspace, Delete,
    wycinanie, przeciąganie i upuszczanie oraz Cofnij są wyłączone w wybranych poniżej obszarach.
    Nadal możesz swobodnie pisać, wklejać, formatować i poruszać się po tekście. Szkic tylko rośnie.
settings-games-session-warning =
    To wybór na jedną sesję: nigdy nie jest zapisywany. Zamknięcie projektu lub zakończenie
    Skribisto zawsze kończy grę, a wszystko, co napisano podczas gry, od tej chwili
    znów można cofnąć.
settings-group-games-scope = Gdzie obowiązuje
settings-games-in-prose = Proza rękopisu
settings-games-in-synopsis = Streszczenia
settings-games-scope-hint =
    Komentarze, przypisy i tytuły nigdy nie są zamrażane: to w nich zapisujesz poprawkę,
    której właśnie zabroniłeś sobie wprowadzić.
settings-games-inert-warning =
    „Zawsze do przodu” jest włączone, ale nic nie obejmuje. Zaznacz powyżej co najmniej jeden obszar,
    w przeciwnym razie podczas pisania nic się nie zmieni.

games-title = Gry pisarskie
games-forward-name = Zawsze do przodu
games-forward-blurb =
    Pisz szkic, niczego nie cofając. Usuwanie, wycinanie i cofanie są wyłączone
    podczas gry. Napisz następne zdanie, zamiast poprawiać poprzednie.
games-forward-playing = Gra trwa: usuwanie jest wyłączone
games-forward-idle = Gra nieaktywna
games-session-note = Kończy się po zamknięciu projektu.
games-scope-prose-and-synopsis = Dotyczy prozy i streszczeń.
games-scope-prose = Dotyczy prozy.
games-scope-synopsis = Dotyczy streszczeń.
games-scope-nothing = Na razie nic nie obejmuje. Wybierz obszar w ustawieniach.
games-settings-link = Ustawienia gier pisarskich…
statusbar-games-forward = Zawsze do przodu
statusbar-games-forward-tooltip =
    „Zawsze do przodu” jest włączone: usuwanie, wycinanie i cofanie są wyłączone podczas
    pisania. Kliknij, aby zakończyć grę.
goal-progress-words = { $g ->
        [one] { $count } z { $goal } słowa
        [few] { $count } z { $goal } słów
        [many] { $count } z { $goal } słów
       *[other] { $count } z { $goal } słowa
    }
goal-progress-characters = { $g ->
        [one] { $count } z { $goal } znaku
        [few] { $count } z { $goal } znaków
        [many] { $count } z { $goal } znaków
       *[other] { $count } z { $goal } znaku
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
overview-excluded-from-export = Wyłączony z eksportu, więc nie wlicza się do żadnej sumy

goal-subtree-total-words = { $count ->
        [one] { $items } cel wewnątrz, łącznie słów: { $words }
        [few] { $items } cele wewnątrz, łącznie słów: { $words }
        [many] { $items } celów wewnątrz, łącznie słów: { $words }
       *[other] { $items } celu wewnątrz, łącznie słów: { $words }
    }
goal-subtree-total-characters = { $count ->
        [one] { $items } cel wewnątrz, łącznie znaków: { $words }
        [few] { $items } cele wewnątrz, łącznie znaków: { $words }
        [many] { $items } celów wewnątrz, łącznie znaków: { $words }
       *[other] { $items } celu wewnątrz, łącznie znaków: { $words }
    }

new-work-goal-unit = Licz w
new-work-goal-unit-hint = Dotyczy wszystkich celów liczby słów lub znaków w tym projekcie. Można to później zmienić.

settings-group-goal-unit = Cele
settings-goal-unit-switch-title = Zmień jednostkę liczenia
settings-goal-unit-switch-text = Cele i kamienie milowe ustawione już w tym projekcie zostały wprowadzone w jednostce „{ $from }”. Przełączenie na „{ $to }” ich nie przelicza, więc każda istniejąca liczba będzie odtąd odczytywana w jednostce „{ $to }”.
settings-goal-unit-switch-informative = Nic nie zostanie utracone. W każdej chwili możesz przełączyć z powrotem, aby przywrócić pierwotne odczytanie, a następnie zaktualizować cele, które chcesz zachować.
settings-goal-unit-switch-confirm = Przełącz

goal-distribute-action = Rozdziel…
distribute-title = Rozdziel cel
distribute-empty = Wewnątrz nie ma niczego, między co można by rozdzielić cel.
distribute-weight = Rozdziel
distribute-weight-length = Według długości
distribute-weight-rows = Według elementów
distribute-weight-even = Równomiernie
distribute-overwrite = Zastąp już ustawione cele
distribute-col-item = Element
distribute-col-current = Teraz
distribute-col-proposed = Po
distribute-total = Łącznie { $total } przy celu { $goal }.
distribute-over-budget = Cele ustawione już wewnątrz przekraczają cel tego kontenera o { $over }. Podnieś jego cel, obniż ich cele albo je zastąp.
distribute-apply = Rozdziel
distribute-cancel = Anuluj

milestone-target-gone = Cel usunięty
milestone-no-target = Brak celu
milestone-add = Dodaj
milestone-add-label = Co ma zostać osiągnięte

pace-summary-title = Na jakim etapie jest książka
pace-summary-remaining = Pozostało: { $words }
pace-summary-open = Otwórz plan
pace-summary-close = Zamknij
pace-summary-dont-show = Nie pokazuj tego przy otwieraniu
pace-summary-menu = Plan pisania…

menu-help-topics = &Tematy pomocy
menu-help-shortcuts = &Skróty klawiszowe…
menu-help-website = Skribisto w &internecie
menu-help-report = &Zgłoś problem…
menu-help-check-updates = Sprawdź &aktualizacje
menu-command-palette = &Paleta poleceń…

learn-help-topics = Tematy pomocy
learn-shortcuts = Skróty klawiszowe
learn-website = Skribisto w internecie

help-window-title = Pomoc
help-filter-topics = Filtruj tematy
help-no-matching-topic = Żaden temat nie pasuje.
help-topic-missing = Ten temat nie jest już dostępny.
help-back = Wstecz
help-not-translated = Ta strona nie została jeszcze przetłumaczona, dlatego jest wyświetlana po angielsku.

help-section-getting-started = Pierwsze kroki
help-section-writing = Pisanie
help-section-reviewing = Recenzowanie
help-section-exchanging = Wprowadzanie i wyprowadzanie tekstu
help-section-keeping = Bezpieczeństwo Twojej pracy
help-section-extensions = Rozszerzenia

help-topic-getting-started = Twój pierwszy projekt
help-topic-writing-model = Jak zbudowana jest książka
help-topic-drafts-and-old-versions = Zachowywanie starszej wersji roboczej
help-topic-goals-and-pace = Cele i tempo
help-topic-comments = Komentarze
help-topic-round-trip = Wysyłanie książki do czytelnika
help-topic-export = Eksportowanie
help-topic-import-documents = Importowanie dokumentów
help-topic-import-projects = Przenoszenie całego projektu
help-topic-import-scrivener = Przejście z programu Scrivener
help-topic-backups-and-versions = Kopie zapasowe i wersje

help-shortcuts-title = Skróty klawiszowe
help-shortcuts-filter = Filtruj skróty
help-shortcuts-no-matches = Żaden skrót nie pasuje.
help-shortcuts-rebind = Zmień skróty…
help-shortcuts-close = Zamknij

command-palette-placeholder = Wpisz polecenie

shortcut-name-binder-duplicate = Duplikuj
shortcut-name-comments-add = Dodaj komentarz
shortcut-name-comments-add-paragraph = Skomentuj akapit
shortcut-name-spellcheck-toggle = Sprawdź pisownię
shortcut-name-editor-tab-close = Zamknij kartę
shortcut-name-editor-tab-pin = Przypnij lub odepnij kartę
shortcut-name-editor-save = Zapisz
shortcut-name-work-export = Eksportuj…
shortcut-name-work-new = Nowy projekt
shortcut-name-work-open = Otwórz projekt
shortcut-name-window-new = Nowe okno
shortcut-name-work-close = Zamknij projekt
shortcut-name-work-import-scrivener = Importuj projekt Scrivener…
shortcut-name-work-import-scrivener-report = Pokaż raport importu z programu Scrivener
shortcut-name-app-settings = Ustawienia
shortcut-name-app-quit = Zakończ
shortcut-name-editor-insert-footnote = Wstaw przypis
shortcut-name-format-scene-break = Wstaw separator sceny
shortcut-name-format-major-scene-break = Wstaw większy separator sceny
shortcut-name-format-link = Wstaw odnośnik
shortcut-name-format-mark-nbsp = Wstaw spację nierozdzielającą
shortcut-name-format-mark-nbhyphen = Wstaw łącznik nierozdzielający
shortcut-name-format-mark-nnbsp = Wstaw wąską spację nierozdzielającą
shortcut-name-format-mark-zwsp = Wstaw spację o zerowej szerokości
shortcut-name-go-next = Następny
shortcut-name-go-prev = Poprzedni
shortcut-name-go-to = Przejdź do
shortcut-name-outline-toggle = Pokaż lub ukryj konspekt
shortcut-name-preview-toggle = Pokaż lub ukryj pasek podglądu
shortcut-name-view-fullscreen = Przełącz pełny ekran
shortcut-name-view-focus-mode = Przełącz tryb bez rozpraszania
shortcut-name-editor-size-increase = Zwiększ rozmiar tekstu
shortcut-name-editor-size-decrease = Zmniejsz rozmiar tekstu
shortcut-name-editor-size-reset = Przywróć rozmiar tekstu
shortcut-name-editor-find = Znajdź
shortcut-name-editor-replace = Zastąp
shortcut-name-editor-find-next = Następne trafienie
shortcut-name-editor-find-prev = Poprzednie trafienie
shortcut-name-search-show = Szukaj w projekcie
shortcut-name-search-replace = Zastąp w projekcie
shortcut-name-outline-open-to-side = Otwórz obok
shortcut-name-help-topics = Tematy pomocy
shortcut-name-help-shortcuts = Skróty klawiszowe
shortcut-name-help-website = Skribisto w internecie
shortcut-name-help-report = Zgłoś problem
shortcut-name-help-check-updates = Sprawdź aktualizacje
update-check-behind = Wersja { $version } jest dostępna.
update-check-current = Używasz najnowszej wersji.
update-check-failed = Nie można sprawdzić nowszej wersji. { $error }
shortcut-name-command-palette = Paleta poleceń
help-section-reference = Pojęcia

help-concept-scene-break = Separator sceny
help-concept-major-scene-break = Większy separator sceny
help-concept-find-in-prose = Wyszukiwanie w prozie
help-concept-story-bible = Biblia opowieści
help-concept-goal-unit = Słowa lub znaki
help-concept-goal-progress = Postęp
help-concept-manuscript-words = Co liczy się jako rękopis
help-concept-exportable = Wyłączone z eksportu
help-concept-distribute = Rozdzielanie celu
help-concept-subtree-total = Suma celów wewnątrz
help-concept-milestone = Kamień milowy
help-concept-pace-plan = Plan tempa

help-concept-tag = Znacznik
help-concept-label = Etykieta
help-concept-point-of-view = Punkt widzenia
help-concept-epigraph = Motto
help-concept-footnote = Przypis
help-concept-chapter-mode = Kształt rozdziału
help-concept-comment = Komentarz
help-concept-backup = Kopia zapasowa
help-concept-version = Wersja
help-concept-trash = Kosz
help-concept-spellcheck = Sprawdzanie pisowni
help-concept-search-replace = Szukaj i zastąp
help-concept-note-template = Szablon notatki
help-concept-story-bible-entry = Wpis biblii opowieści
help-concept-text-replacement = Zamiana tekstu
help-concept-smart-punctuation = Inteligentna interpunkcja
help-concept-export-style = Styl eksportu
help-concept-round-trip-marks = Znaczniki ponownego importu

epigraph-mark-attribution = Wiersz źródła
epigraph-mark-attribution-tip = Oznacz wiersz, w którym jest kursor, jako źródło cytatu, aby drukował się jako podpis
margin-lane-name = Znaczniki marginesu
margin-lane-provider-comments = Komentarze
margin-lane-provider-story-bible = Wpis biblii opowieści
margin-lane-provider-story-bible-hint = W widoku „W prozie” notatki: miejsca, w których wpis jest wymieniony, i sceny opowiadane jego oczami. Zaznacza też te nazwy w samej prozie.
margin-lane-mark-point-of-view = Opowiadane stąd
margin-lane-mark-named-here = Wymieniony tutaj
margin-lane-provider-comments-hint = Miejsca, w których do tekstu dołączono komentarz
margin-lane-provider-search = Trafienia wyszukiwania
margin-lane-provider-search-hint = Każde trafienie ostatniego wyszukiwania, w dowolnym miejscu dokumentu
margin-lane-provider-boundaries = Początki dokumentów
margin-lane-provider-boundaries-hint = Linia na początku każdej sceny w strumieniu, aby było widać, gdzie jesteś
margin-lane-provider-spelling = Pisownia
margin-lane-provider-spelling-hint = Miejsca, w których sprawdzanie pisowni oznaczyło słowo. Domyślnie wyłączone: to opinia maszyny o Twojej prozie
margin-lane-spelling = { $word }, możliwy błąd pisowni
margin-lane-search-hit = { $text }, trafienie { $index } z { $total }
margin-lane-search-current = { $text }, trafienie { $index } z { $total }, bieżące
margin-lane-boundary = Początek: { $title }
margin-lane-boundary-untitled = dokument bez tytułu
settings-page-margin-lane = Znaczniki marginesu
settings-desc-margin-lane = Pasek obok paska przewijania i to, co pokazuje
settings-margin-lane-enabled = Pokaż pas marginesu
settings-margin-lane-enabled-hint = Ten sam przełącznik co Widok ▸ Znaczniki marginesu.
settings-margin-lane-enabled-more = Nigdy nie twierdzi, że coś jest źle: pokazuje, gdzie co jest, a ocenę pozostawia Tobie.
settings-group-margin-lane-marks = Co zaznacza
settings-margin-lane-no-providers = Nic jeszcze nie zaznacza pasa.
settings-group-margin-lane-texture = Faktura dialogów
settings-margin-lane-texture = Pokaż fakturę dialogów
settings-margin-lane-texture-hint = Jeden słupek na akapit: jego długość i jaka jego część to dialog.
settings-margin-lane-texture-more = Niemierzone w językach bez opracowanej konwencji; tam słupek pokazuje tylko długość.
settings-group-margin-lane-surfaces = Gdzie się pojawia
settings-margin-lane-surface-editor = Edytor tekstu
settings-margin-lane-surface-stream = Strumienie
settings-margin-lane-surface-search-preview = Podgląd wyszukiwania
menu-margin-lane = &Znaczniki marginesu

status-preset-drafting = Szkicowanie
status-preset-passes = Kolejne redakcje
status-preset-plume = Plume Creator
status-todo = Do zrobienia
status-draft = Szkic
status-revised = Poprawione
status-final = Ostateczne
status-outline = Konspekt
status-first-edit = 1. redakcja
status-second-edit = 2. redakcja
status-done = Gotowe
status-plume-draft-1 = 1. szkic
status-plume-draft-2 = 2. szkic
status-plume-draft-3 = 3. szkic
status-plume-edit-1 = 1. redakcja
status-plume-edit-2 = 2. redakcja
status-plume-edit-3 = 3. redakcja
status-plume-proofread = Korekta
status-plume-finished = Ukończone
new-work-statuses = Etapy pracy
new-work-statuses-hint = Etapy, przez które przechodzi scena. W każdej chwili możesz zmieniać ich nazwy i kolejność oraz dodawać nowe.
status-none = Brak statusu
inspector-status = Status
overview-col-status = Status
overview-status-mixed = elementy poniżej mają różne statusy
help-concept-status = Status
status-completion-title = Na jakim etapie jest książka
status-completion-headline = Ukończone sceny: { $done } z { $total }
status-completion-empty = Brak scen. To podsumowanie czyta rękopis, więc wypełnia się w miarę pisania.
status-completion-open = Na jakim etapie jest książka…

settings-page-statuses = Statusy
settings-desc-statuses = Zmieniaj nazwy i kolejność, dodawaj i usuwaj etapy, przez które przechodzi scena
settings-statuses-add = Dodaj status
settings-statuses-add-placeholder = Nowa nazwa statusu
settings-statuses-desc =
    Etapy, przez które przechodzi scena, po kolei: od najmniej ukończonego na górze do najbardziej
    ukończonego na dole. Ta kolejność określa, co w całej aplikacji oznacza „mniej ukończone niż”,
    więc ułóż je tak, jak naprawdę przebiega Twoja praca.
settings-statuses-apply-preset = Zastosuj gotowy zestaw…
settings-statuses-preset-applied = { $added ->
        [one] Dodano { $added } status
        [few] Dodano { $added } statusy
        [many] Dodano { $added } statusów
       *[other] Dodano { $added } statusu
    }
settings-statuses-preset-refused = Ten projekt ma już drabinę statusów. Gotowe zestawy wypełniają tylko pustą drabinę. Najpierw usuń szczeble, których nie chcesz.
settings-statuses-duplicate = „{ $name }” jest już na tej drabinie
settings-statuses-added = Dodano „{ $name }”
settings-statuses-move-up = Przesuń w górę (mniej ukończone)
settings-statuses-move-down = Przesuń w dół (bardziej ukończone)
settings-statuses-details-placeholder = Co oznacza ten etap (opcjonalnie)
settings-statuses-delete = Usuń „{ $name }”
settings-statuses-delete-in-use = { $count ->
        [one] Usuń „{ $name }”: { $count } element ma ten status i go straci
        [few] Usuń „{ $name }”: { $count } elementy mają ten status i go stracą
        [many] Usuń „{ $name }”: { $count } elementów ma ten status i go straci
       *[other] Usuń „{ $name }”: { $count } elementu ma ten status i go straci
    }
settings-statuses-deleted = Usunięto „{ $name }”
settings-statuses-deleted-in-use = { $count ->
        [one] Usunięto „{ $name }”. { $count } element nie ma już statusu.
        [few] Usunięto „{ $name }”. { $count } elementy nie mają już statusu.
        [many] Usunięto „{ $name }”. { $count } elementów nie ma już statusu.
       *[other] Usunięto „{ $name }”. { $count } elementu nie ma już statusu.
    }
settings-statuses-empty-title = Brak etapów pracy
settings-statuses-empty-body = Status mówi, na jakim etapie jest scena. Dodaj go powyżej lub zacznij od gotowego zestawu.
status-category-planned = Zaplanowane
status-category-drafting = W trakcie pisania
status-category-needs-work = Do poprawy
status-category-revised = Poprawione
status-category-final = Ostateczne

## Long-operation progress (shared::progress_text). The backend reports phase
## codes it cannot translate; these are their labels. `progress-percent` is the
## toast body on its own, `progress-percent-with-label` with the phase after it.
progress-percent = { $percent }%
progress-percent-with-label = { $percent }% · { $label }
progress-import-opening = Otwieranie projektu…
progress-import-outline = Czytanie konspektu…
progress-import-converting = Przekształcanie rozdziałów i scen…
progress-import-writing = Zapisywanie pliku .skrib…
progress-import-done = Gotowe
progress-import-structure = Ustalanie struktury…
progress-export-start = Rozpoczynanie…
progress-export-writing = Zapisywanie…
progress-project-reading = Czytanie projektu…
# Settings: the open project's section title, "Work: <its title>".
settings-sec-work-titled = Projekt: { $title }

## Project-import notices (shared::import_warnings::notices): one line of the
## Plume / Manuskript warnings list each. $file, $folder, $title, $name, $field
## and $error are data (file names, titles, a parser's own message) and stay as
## they are. $container is "single-file" or "folder"; $kind is "scene", "note",
## "synopsis" or "title".
import-notice-flattened-rich-texts = { $count ->
        [one] Jeden tekst był zagnieżdżony głębiej, niż może pomieścić projekt Skribisto; jego słowa zachowano jako zwykły tekst, bez formatowania.
        [few] { $count } teksty były zagnieżdżone głębiej, niż może pomieścić projekt Skribisto; ich słowa zachowano jako zwykły tekst, bez formatowania.
        [many] { $count } tekstów było zagnieżdżonych głębiej, niż może pomieścić projekt Skribisto; ich słowa zachowano jako zwykły tekst, bez formatowania.
       *[other] { $count } tekstu było zagnieżdżonych głębiej, niż może pomieścić projekt Skribisto; ich słowa zachowano jako zwykły tekst, bez formatowania.
    }
import-notice-free-text-badges = { $count ->
        [one] Jedna plakietka sceny wygląda na swobodny tekst, a nie listę statusów, więc zachowano ją jako etykietę elementu zamiast zamieniać w znacznik.
        [few] { $count } różne plakietki scen wyglądają na swobodny tekst, a nie listę statusów, więc zachowano je jako etykiety elementów zamiast zamieniać w znaczniki.
        [many] { $count } różnych plakietek scen wygląda na swobodny tekst, a nie listę statusów, więc zachowano je jako etykiety elementów zamiast zamieniać w znaczniki.
       *[other] { $count } różnej plakietki scen wygląda na swobodny tekst, a nie listę statusów, więc zachowano je jako etykiety elementów zamiast zamieniać w znaczniki.
    }
import-notice-separator-text-dropped = Separator „{ $name }” zawierał tekst, który nie ma miejsca w projekcie Skribisto; tekstu nie zaimportowano.
import-notice-separator-without-scene = Separator „{ $name }” nie miał w swoim rozdziale poprzedzającej sceny, do której można by go dołączyć, więc nie został zaimportowany.
import-notice-chapter-without-scenes = Rozdział „{ $name }” nie ma scen, więc jego separatory nie zostały zaimportowane.
import-notice-rich-text-unconverted = Części sformatowanego tekstu nie udało się przekształcić ({ $error }); nie została zaimportowana.
import-notice-unresolved-cross-link = Nie udało się podążyć za odnośnikiem do wpisu obsady nr { $number }: wpisu brakuje lub był w koszu.
import-notice-read-copy = Odczytano { $container ->
        [single-file] jednoplikową
       *[folder] folderową
    } kopię tego projektu.
import-notice-read-copy-dated = Odczytano { $container ->
        [single-file] jednoplikową
       *[folder] folderową
    } kopię tego projektu, ostatnio zmienioną { $day }.
import-notice-original-format = Ten projekt jest w pierwotnym formacie Manuskriptu z 2016 roku. Odczytano go w całości; nic w nim nie zostało zmienione.
import-notice-newer-single-file-beside = „{ $file }” jest nowszy niż ten folder ({ $file_day } wobec { $folder_day }). Manuskript pozostawia stary folder, gdy projekt przechodzi w tryb pojedynczego pliku, więc ten folder może być porzuconą kopią; import pliku .msk odczytałby nowszą wersję.
import-notice-pickle-skipped = Plik „{ $file }” pozostawiono nieodczytany. To plik pickle Pythona, a wczytywanie go z pliku projektu było luką bezpieczeństwa w samym Manuskripcie (CVE-2021-35196); zapisane w nim ustawienia to preferencje wyświetlania, a nie tekst.
import-notice-file-unreadable = Nie udało się odczytać „{ $file }” ({ $error }), więc go pominięto; wszystko inne zostało zaimportowane.
import-notice-not-utf8 = „{ $file }” nie jest poprawnym plikiem UTF-8; znaki, których nie dało się odczytać, zostały zastąpione zamiast przerywać import.
import-notice-table-unreadable = Nie udało się odczytać „{ $file }” ({ $error }); jego zawartość nie została zaimportowana.
import-notice-outline-missing = Brakuje pliku „{ $file }”, więc ten projekt nie ma rękopisu do zaimportowania.
import-notice-outline-unreadable = Nie udało się odczytać „{ $file }” ({ $error }); nie zaimportowano rękopisu.
import-notice-named-none-row = „{ $file }” zaczyna się od nazwanego wiersza („{ $name }”) tam, gdzie Manuskript trzyma swój pusty wpis „none”. Pominięto go, aby kolejne wiersze zachowały numery używane w rękopisie.
import-notice-unknown-field = „{ $file }” zawiera pole nieznane temu importerowi: „{ $field }”. Nie zostało zaimportowane.
import-notice-colorless-labels = { $count ->
        [one] „{ $file }” zawiera wiersz bez koloru. Zaimportowano go bez koloru; Manuskript przerywa wczytywanie projektu na takim wierszu.
        [few] „{ $file }” zawiera { $count } wiersze bez koloru. Zaimportowano je bez koloru; Manuskript przerywa wczytywanie projektu na takim wierszu.
        [many] „{ $file }” zawiera { $count } wierszy bez koloru. Zaimportowano je bez koloru; Manuskript przerywa wczytywanie projektu na takim wierszu.
       *[other] „{ $file }” zawiera { $count } wiersza bez koloru. Zaimportowano je bez koloru; Manuskript przerywa wczytywanie projektu na takim wierszu.
    }
import-notice-settings-unreadable = Nie udało się odczytać „{ $file }” ({ $error }); nie zaimportowano języka projektu.
import-notice-revisions-unreadable = Nie udało się odczytać „{ $file }” ({ $error }); nie zaimportowano historii projektu, ale nic innego nie zostało naruszone.
import-notice-foreign-files = { $count ->
        [one] Ten projekt zawiera plik, który nie należy do formatu Manuskript i nie został zaimportowany: { $files }.
        [few] Ten projekt zawiera { $count } pliki, które nie należą do formatu Manuskript i nie zostały zaimportowane: { $files }.
        [many] Ten projekt zawiera { $count } plików, które nie należą do formatu Manuskript i nie zostały zaimportowane: { $files }.
       *[other] Ten projekt zawiera { $count } pliku, które nie należą do formatu Manuskript i nie zostały zaimportowane: { $files }.
    }
import-notice-foreign-files-more = { $count ->
        [one] Ten projekt zawiera { $count } plik
        [few] Ten projekt zawiera { $count } pliki
        [many] Ten projekt zawiera { $count } plików
       *[other] Ten projekt zawiera { $count } pliku
    }, które nie należą do formatu Manuskript i nie zostały zaimportowane: { $files } oraz { $more ->
        [one] jeszcze jeden
        [few] jeszcze { $more }
        [many] jeszcze { $more }
       *[other] jeszcze { $more }
    }.
import-notice-not-a-character-file = „{ $file }” nie jest plikiem postaci Manuskriptu i został pominięty.
import-notice-not-a-text-item = „{ $file }” nie jest elementem tekstowym Manuskriptu i został pominięty w konspekcie.
import-notice-missing-blank-line = „{ $file }” nie ma pustego wiersza po nagłówku. Jego ostatnie pole zostało zachowane, choć Manuskript je gubi.
import-notice-folder-without-metadata = „{ $folder }” nie zawiera pliku { $file }, więc zaimportowano go jako zwykły folder nazwany tak jak katalog. Manuskript pomija taki katalog wraz z całą zawartością.
import-notice-rows-given-ids = { $count ->
        [one] Jeden wiersz konspektu nie miał własnego identyfikatora i otrzymał go. Manuskript pomija taki wiersz albo przerywa wczytywanie projektu, gdy jest to folder.
        [few] { $count } wiersze konspektu nie miały własnego identyfikatora i otrzymały go. Manuskript pomija taki wiersz albo przerywa wczytywanie projektu, gdy jest to folder.
        [many] { $count } wierszy konspektu nie miało własnego identyfikatora i otrzymało go. Manuskript pomija taki wiersz albo przerywa wczytywanie projektu, gdy jest to folder.
       *[other] { $count } wiersza konspektu nie miało własnego identyfikatora i otrzymało go. Manuskript pomija taki wiersz albo przerywa wczytywanie projektu, gdy jest to folder.
    }
import-notice-plots-unreadable = Nie udało się odczytać „{ $file }” ({ $error }); nie zaimportowano wątków.
import-notice-world-unreadable = Nie udało się odczytać „{ $file }” ({ $error }); nie zaimportowano elementów świata.
import-notice-world-not-opml = „{ $file }” nie jest dokumentem OPML; nie zaimportowano elementów świata.
import-notice-text-flattened = Tekst „{ $title }” jest zagnieżdżony głębiej, niż może pomieścić projekt Skribisto; jego słowa zachowano jako zwykły tekst, bez formatowania.
import-notice-text-unconverted = Nie udało się przekształcić tekstu „{ $title }” ({ $error }); zachowano go dokładnie w takiej postaci, w jakiej został zapisany.
import-notice-text-without-file-name = Nie udało się nadać nazwy pliku { $kind ->
        [scene] tekstowi sceny
        [note] tekstowi notatki
        [synopsis] streszczeniu
       *[title] tytułowi
    } elementu „{ $title }”, więc nie został zaimportowany.
import-notice-unknown-label = „{ $title }” ma etykietę { $index }, której nie ma na liście etykiet tego projektu. Zaimportowano go bez etykiety.
import-notice-unknown-status = „{ $title }” ma status { $index }, którego nie ma na liście statusów tego projektu. Zaimportowano go bez statusu.
import-notice-unknown-point-of-view = „{ $title }” jest opowiadany oczami postaci, której nie ma w obsadzie tego projektu. Nie ustawiono punktu widzenia.
import-notice-unresolved-references = Nie udało się rozwiązać odwołań w tekście „{ $title }” ({ $error }); tekst zachowano w postaci po przekształceniu, łącznie ze znacznikami odwołań.
import-notice-custom-icons = { $count ->
        [one] Jeden wiersz miał własną ikonę. Skribisto nadaje każdemu wierszowi ikonę jego typu, więc ikony nie zaimportowano; nic innego w wierszu się nie zmieniło.
        [few] { $count } wiersze miały własne ikony. Skribisto nadaje każdemu wierszowi ikonę jego typu, więc ikon nie zaimportowano; nic innego w tych wierszach się nie zmieniło.
        [many] { $count } wierszy miało własne ikony. Skribisto nadaje każdemu wierszowi ikonę jego typu, więc ikon nie zaimportowano; nic innego w tych wierszach się nie zmieniło.
       *[other] { $count } wiersza miało własne ikony. Skribisto nadaje każdemu wierszowi ikonę jego typu, więc ikon nie zaimportowano; nic innego w tych wierszach się nie zmieniło.
    }
import-notice-character-colors = { $count ->
        [one] Jedna postać miała kolor przy imieniu. Skribisto koloruje znaczniki, a nie wiersze, więc koloru nie zaimportowano; samą postać tak.
        [few] { $count } postacie miały kolor przy imieniu. Skribisto koloruje znaczniki, a nie wiersze, więc kolorów nie zaimportowano; same postacie zaimportowano w całości.
        [many] { $count } postaci miało kolor przy imieniu. Skribisto koloruje znaczniki, a nie wiersze, więc kolorów nie zaimportowano; same postacie zaimportowano w całości.
       *[other] { $count } postaci miało kolor przy imieniu. Skribisto koloruje znaczniki, a nie wiersze, więc kolorów nie zaimportowano; same postacie zaimportowano w całości.
    }
import-notice-versions-kept = { $found ->
        [one] Ten projekt zarejestrował jedną wcześniejszą wersję
        [few] Ten projekt zarejestrował { $found } wcześniejsze wersje
        [many] Ten projekt zarejestrował { $found } wcześniejszych wersji
       *[other] Ten projekt zarejestrował { $found } wcześniejszej wersji
    }; { $kept ->
        [one] zachowano jedną
        [few] zachowano { $kept }
        [many] zachowano { $kept }
       *[other] zachowano { $kept }
    }, przerzedzone tak, jak Skribisto przerzedza własną historię. Znajdziesz je w panelu „Wersje”.
import-notice-versions-unplaced = { $count ->
        [one] Jedna wcześniejsza wersja należy do wiersza, którego nie ma już w projekcie, i nie dało się jej umieścić.
        [few] { $count } wcześniejsze wersje należą do wiersza, którego nie ma już w projekcie, i nie dało się ich umieścić.
        [many] { $count } wcześniejszych wersji należy do wiersza, którego nie ma już w projekcie, i nie dało się ich umieścić.
       *[other] { $count } wcześniejszej wersji należy do wiersza, którego nie ma już w projekcie, i nie dało się ich umieścić.
    }

## Native file dialogs: the window title, and the label of the file-type filter.
dialog-open-work-title = Otwórz projekt Skribisto
dialog-save-as-file-title = Zapisz jako pojedynczy plik .skrib
dialog-save-as-folder-title = Wybierz folder nadrzędny dla projektu
dialog-manuskript-file-title = Wybierz projekt Manuskript
dialog-manuskript-folder-title = Wybierz folder projektu Manuskript
dialog-plume-file-title = Wybierz projekt Plume Creator
dialog-filter-skrib = Projekt Skribisto
dialog-filter-manuskript = Projekt Manuskript
dialog-filter-plume = Projekt Plume Creator
dialog-filter-hunspell-aff = Plik afiksów Hunspell (.aff)
dialog-filter-hunspell-dic = Słownik Hunspell (.dic)
dialog-filter-export = Wyeksportowany plik
# Help ▸ Keyboard shortcuts / Settings ▸ Keymap / command palette.
shortcut-name-view-margin-lane = Pokaż lub ukryj znaczniki marginesu

## The built-in distraction-free themes' names (distraction_free::theme).
df-theme-paper = Papier
df-theme-sepia = Sepia
df-theme-night = Noc
df-theme-high-contrast = Wysoki kontrast
df-theme-midnight = Północ

## Welcome ▸ Examples: the line under a bundled example's title.
welcome-example-byline = autor: { $author }

## Spoken by a screen reader only.
# Welcome window: the name of the section tab bar.
welcome-sections-a11y = Sekcje okna startowego
# A comment's anchored span in the editor: author, plain-text preview, replies.
comments-a11y-summary-author = { $author }: { $text }
comments-a11y-summary-replies = { $text } ({ $count ->
        [one] { $count } odpowiedź
        [few] { $count } odpowiedzi
        [many] { $count } odpowiedzi
       *[other] { $count } odpowiedzi
    })
comments-a11y-summary-author-replies = { $author }: { $text } ({ $count ->
        [one] { $count } odpowiedź
        [few] { $count } odpowiedzi
        [many] { $count } odpowiedzi
       *[other] { $count } odpowiedzi
    })
# Margin marks beside the text: one mark, several merged into one, and the
# lane's own name when nothing more specific was given.
margin-lane-a11y-mark = { $label }, { $percent } procent od początku
margin-lane-a11y-marks = { $label }, { $count ->
        [one] { $count } znacznik
        [few] { $count } znaczniki
        [many] { $count } znaczników
       *[other] { $count } znacznika
    }, { $percent } procent od początku
margin-lane-a11y-fallback = { $count ->
        [one] Znaczniki marginesu, { $count } znacznik
        [few] Znaczniki marginesu, { $count } znaczniki
        [many] Znaczniki marginesu, { $count } znaczników
       *[other] Znaczniki marginesu, { $count } znacznika
    }
# Export ▸ Choose: the row standing for a binder that has no name.
export-choose-unnamed-binder = Segregator bez nazwy
# The built-in export styles named in words (the national manuscript presets
# keep their own language's name).
export-style-neutral = Neutralny
export-style-manuscript-shunn = Standardowy maszynopis (Shunn)
export-style-ebook-clean = Czysty e-book

## Dates. Put the parts in the order your language writes a short date; the
## parts arrive already zero-padded where that matters.
# Comment cards, the outline card's Created/Modified.
date-time-short = { $day }.{ $month }.{ $year } { $hour }:{ $minute }
# Pace chart: a day on the axis, a milestone's date, a holiday.
pace-day-label = { $day }.{ $month }

## File sizes (shared::byte_size). $whole and $tenth are the two sides of the
## decimal mark: write your language's mark between them and its unit symbol.
byte-size-bytes = { $value } B
byte-size-kb = { $value } KB
byte-size-mb = { $whole },{ $tenth } MB
byte-size-gb = { $whole },{ $tenth } GB
# A whole percentage (sliders, the Pace card, the text-size toast).
unit-percent = { $value }%
# A whole number of pixels (slider readouts).
unit-pixels = { $value } px
# Settings ▸ Paratext: a preset saved without a `name` line. `name` is the TOML
# key the writer types and stays as it is.
settings-paratext-error-no-name = Zestaw musi mieć pole `name`.
# Export: started with no project open (a menu raced a close).
export-error-no-project = Brak otwartego projektu do wyeksportowania.

shortcut-group-work = Projekt
shortcut-group-edit = Edycja
shortcut-group-view = Widok
shortcut-group-document = Dokument
shortcut-group-format = Format
shortcut-group-go = Przejdź
shortcut-group-tools = Narzędzia
shortcut-group-help = Pomoc

## Manuskript import: the headings of the notes it writes and the title of a
## project with no name. Written into the imported project once, in the
## interface language.
import-manuskript-section-paragraph-summary = Podsumowanie w jednym akapicie
import-manuskript-section-full-summary = Pełne podsumowanie
import-manuskript-section-motivation = Motywacja
import-manuskript-section-epiphany = Olśnienie
import-manuskript-section-notes = Notatki
import-manuskript-section-description = Opis
import-manuskript-section-passion = Pasja
import-manuskript-section-result = Wynik
import-manuskript-section-summary = Podsumowanie
import-manuskript-section-meta = Uwagi
import-manuskript-section-series = Seria
import-manuskript-section-volume = Tom
import-manuskript-section-genre = Gatunek
import-manuskript-section-license = Licencja
import-manuskript-section-email = E-mail
import-manuskript-section-situation = Sytuacja
import-manuskript-section-sentence = Jedno zdanie
import-manuskript-section-paragraph = Jeden akapit
import-manuskript-section-page = Jedna strona
import-manuskript-section-full = Pełne podsumowanie
import-manuskript-untitled-project = Zaimportowany projekt Manuskript

## Plume import: the story-bible groups an old project implies, and the title
## of a project with no name.
import-plume-group-characters = Postacie
import-plume-group-items = Przedmioty
import-plume-group-places = Miejsca
import-plume-untitled-project = Zaimportowany projekt Plume
