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
import-plume-busy-title = Trwa import innego projektu Plume Creator
import-plume-busy-text = Projekty Plume Creator są importowane pojedynczo. Poczekaj na zakończenie trwającego importu lub anuluj go, a następnie uruchom ten ponownie.
import-manuskript-busy-title = Trwa import innego projektu Manuskript
import-manuskript-busy-text = Projekty Manuskript są importowane pojedynczo. Poczekaj na zakończenie trwającego importu lub anuluj go, a następnie uruchom ten ponownie.
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
import-plume-close = Zamknij
import-plume-source = Projekt Plume
import-plume-source-hint = Wybierz plik .plume lub .plume_backup (dowolna wersja Plume Creator).
import-plume-location = Folder docelowy
import-plume-name = Nazwa pliku
import-plume-name-placeholder = Nazwa projektu
import-plume-will-create = Zostanie utworzony
import-plume-trash-warning = ⚠ Elementy z kosza i usunięte nie są przenoszone.
import-plume-cancel = Anuluj
import-plume-import = Importuj
import-plume-source-required = Wybierz plik projektu Plume
import-plume-source-missing = Ten plik nie istnieje
import-plume-source-not-file = Ta ścieżka nie jest plikiem
import-plume-location-required = Wybierz folder docelowy
import-plume-location-missing = Ten folder nie istnieje
import-plume-location-not-folder = Ta ścieżka nie jest folderem
import-plume-location-readonly = Brak uprawnień do zapisu w tym folderze
import-plume-name-required = Wprowadź nazwę pliku
import-plume-name-exists = Plik o tej nazwie już tu istnieje. Import poprosi o potwierdzenie nadpisania
import-plume-overwrite-title = Zastąpić istniejący plik?
import-plume-overwrite-text = „{ $name }” już istnieje. Zastąpić go importowanym projektem?
import-plume-manuscript-binder = Rękopis
import-plume-story-bible-binder = Biblia opowieści
import-plume-progress-title = Importowanie projektu Plume…
import-plume-cancel-import = Anuluj
import-plume-cancelled = Import anulowany
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
import-plume-open-now = Otwórz teraz
import-plume-warnings = { $count ->
        [one] { $count } rzecz nie została zaimportowana wiernie
        [few] { $count } rzeczy nie zostały zaimportowane wiernie
        [many] { $count } rzeczy nie zostało zaimportowanych wiernie
       *[other] { $count } rzeczy nie zostało zaimportowanych wiernie
    }
import-plume-details = Szczegóły
import-plume-warnings-title = Ostrzeżenia importu
import-plume-error-title = Nie można zaimportować projektu
import-plume-nested-too-deep = Projekt nie został zaimportowany: jego część „{ $part }” jest zagnieżdżona na ponad { $limit } poziomów. Żaden projekt Plume Creator nawet się do tego nie zbliża, a jego odczytanie mogłoby spowodować awarię Skribisto. Nic nie zostało zapisane.
import-plume-declares-entities = Projekt nie został zaimportowany: jego część „{ $part }” deklaruje encje XML, czego żaden projekt Plume Creator nie robi. Ich rozwinięcie mogłoby zająć więcej pamięci, niż ma komputer, i spowodować awarię Skribisto. Nic nie zostało zapisane.
import-plume-error-details = Szczegóły

import-manuskript-title = Importuj projekt Manuskript
import-manuskript-close = Zamknij
import-manuskript-source = Projekt Manuskript
import-manuskript-source-hint = Wybierz plik .msk lub folder projektu (dowolna wersja Manuskript).
import-manuskript-source-file = Wybierz plik…
import-manuskript-source-folder = Wybierz folder…
import-manuskript-location = Folder docelowy
import-manuskript-name = Nazwa pliku
import-manuskript-name-placeholder = Nazwa projektu
import-manuskript-will-create = Zostanie utworzony
import-manuskript-cancel = Anuluj
import-manuskript-import = Importuj
import-manuskript-source-required = Wybierz projekt Manuskript
import-manuskript-source-missing = Ten plik lub folder nie istnieje
import-manuskript-location-required = Wybierz folder docelowy
import-manuskript-location-missing = Ten folder nie istnieje
import-manuskript-location-not-folder = Ta ścieżka nie jest folderem
import-manuskript-location-readonly = Brak uprawnień do zapisu w tym folderze
import-manuskript-name-required = Wprowadź nazwę pliku
import-manuskript-name-exists = Plik o tej nazwie już tu istnieje. Import poprosi o potwierdzenie nadpisania
import-manuskript-overwrite-title = Zastąpić istniejący plik?
import-manuskript-overwrite-text = „{ $name }” już istnieje. Zastąpić go importowanym projektem?
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
import-manuskript-progress-title = Importowanie projektu Manuskript…
import-manuskript-cancel-import = Anuluj
import-manuskript-cancelled = Import anulowany
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
import-manuskript-open-now = Otwórz teraz
import-manuskript-warnings = { $count ->
        [one] { $count } uwaga dotycząca tego importu
        [few] { $count } uwagi dotyczące tego importu
        [many] { $count } uwag dotyczących tego importu
       *[other] { $count } uwagi dotyczącej tego importu
    }
import-manuskript-details = Szczegóły
import-manuskript-warnings-title = O tym imporcie
import-manuskript-error-title = Nie można zaimportować projektu
import-manuskript-nested-too-deep = Projekt nie został zaimportowany: jego plik „{ $part }” jest zagnieżdżony na ponad { $limit } poziomów. Żaden projekt Manuskript nawet się do tego nie zbliża, a jego odczytanie mogłoby spowodować awarię Skribisto. Nic nie zostało zapisane.
import-manuskript-folders-too-deep = Projekt nie został zaimportowany: jeden z jego plików leży głębiej niż { $limit } poziomów folderów. Żaden projekt Manuskript nawet się do tego nie zbliża, a jego odczytanie mogłoby spowodować awarię Skribisto. Nic nie zostało zapisane.
import-manuskript-declares-entities = Projekt nie został zaimportowany: jego plik „{ $part }” deklaruje encje XML, czego żaden projekt Manuskript nie robi. Ich rozwinięcie mogłoby zająć więcej pamięci, niż ma komputer, i spowodować awarię Skribisto. Nic nie zostało zapisane.
import-manuskript-error-details = Szczegóły

import-target-open-title = Ten projekt jest otwarty
import-target-open-text = „{ $name }” jest otwarty w Skribisto. Import w jego miejsce zastąpiłby projekt, nad którym pracujesz, a jego następny zapis nadpisałby wynik importu. Wybierz inną nazwę pliku albo najpierw zamknij projekt.
import-warnings-announcement = { $result } { $notice }.

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

cover-choose = &Wybierz okładkę książki…
cover-clear = Usuń okła&dkę
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
import-diagnostic-image-not-ingested = „{ $path }” odwołuje się do obrazu „{ $detail }”. Odwołanie trafia do projektu jako tekst; sam obraz nie jest kopiowany.
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
