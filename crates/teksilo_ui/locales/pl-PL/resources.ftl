# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

# Zasoby: pliki i odnośniki internetowe dołączone do elementów segregatora.

## Panel

resources-title = Zasoby
shortcut-name-resources-show = Pokaż zasoby
resources-add-files = Dodaj pliki
resources-add-files-title = Dodaj pliki do tego elementu
resources-add-link = Dodaj odnośnik internetowy
resources-drop-hint = Upuść tu pliki lub adresy internetowe, aby je dołączyć
resources-no-item = Otwórz element, aby zobaczyć jego zasoby.
resources-empty = Do tego elementu nic jeszcze nie dołączono. Dodaj pliki lub odnośniki internetowe przyciskami powyżej albo upuść je tutaj.
resources-actions = Działania
resources-not-found = Nie znaleziono
resources-row-linked = Plik połączony
resources-row-linked-size = Plik połączony, { $size }
resources-row-name = { $title }, { $kind }, { $detail }
resources-row-name-missing = { $title }, { $kind }, { $detail }, nie znaleziono

## Rodzaje plików, tak jak odczytuje je czytnik ekranu

resources-kind-picture = obraz
resources-kind-pdf = dokument PDF
resources-kind-audio = nagranie dźwiękowe
resources-kind-video = wideo
resources-kind-document = dokument
resources-kind-ebook = e-book
resources-kind-archive = archiwum
resources-kind-file = plik
resources-kind-web-link = odnośnik internetowy

## Menu wiersza

resources-open = Otwórz
resources-show-in-folder = Pokaż w folderze
resources-locate = Odszukaj…
resources-insert-into-text = Wstaw do tekstu
resources-insert-picture = Wstaw obraz do tekstu
resources-insert-link = Wstaw odnośnik do tekstu
resources-edit = Zmień nazwę i opisz…
resources-detach = Odłącz od tego elementu
resources-remove = Usuń z projektu…

## Dwa okna dialogowe

resources-link-dialog-title = Dodaj odnośnik internetowy
resources-link-dialog-address = Adres
resources-link-dialog-address-placeholder = https://
resources-link-dialog-name = Tytuł
resources-link-dialog-name-placeholder = Nazwa strony, jeśli zostawisz to pole puste
resources-link-dialog-add = Dodaj
resources-edit-dialog-title = Zmień nazwę i opisz
resources-edit-dialog-name = Tytuł
resources-edit-dialog-description = Opis
resources-edit-dialog-description-placeholder = Co to jest i dlaczego to zachowujesz
resources-edit-dialog-save = Zapisz
resources-dialog-cancel = Anuluj

## Dodawanie

resources-large-title = Duży plik
resources-large-text = „{ $name }” ma { $size }. Przechowywany w projekcie, podróżuje z każdą jego kopią i każdą kopią zapasową. Połączony, zostaje tam, gdzie jest, a projekt pamięta tylko, gdzie go znaleźć.
resources-large-text-many = „{ $name }” ma { $size }, a { $others ->
    [one] jeszcze jeden plik też jest duży
    [few] jeszcze { $others } inne pliki też są duże
    [many] jeszcze { $others } innych plików też jest dużych
   *[other] jeszcze { $others } innego pliku też jest dużych
}. Przechowywane w projekcie, podróżują z każdą jego kopią i każdą kopią zapasową. Połączone, zostają tam, gdzie są, a projekt pamięta tylko, gdzie je znaleźć.
resources-large-store = Przechowaj go w projekcie
resources-large-link = Zamiast tego połącz
resources-large-remember = Nie pytaj ponownie
resources-storing-title = Dodawanie plików do projektu
resources-store-cancel = Anuluj
resources-store-cancelled = Nie dodano żadnych plików.
resources-store-busy = Pliki są jeszcze dodawane. Spróbuj ponownie, gdy dodawanie się zakończy.
resources-store-failed = Nie udało się dodać plików: { $error }
resources-no-project = Otwórz projekt, zanim dołączysz pliki.
resources-moved-away = Projekt został przeniesiony w trakcie kopiowania pliku.
resources-some-failed = { $count ->
    [one] Nie udało się dodać jednego pliku: { $names }
    [few] Nie udało się dodać { $count } plików: { $names }
    [many] Nie udało się dodać { $count } plików: { $names }
   *[other] Nie udało się dodać { $count } pliku: { $names }
}
resources-add-failed = Nie udało się dodać zasobów: { $error }
resources-added = { $count ->
    [one] Dodano jeden zasób
    [few] Dodano { $count } zasoby
    [many] Dodano { $count } zasobów
   *[other] Dodano { $count } zasobu
}
resources-dropped-added = { $count ->
    [one] Dodano jako zasób tego elementu i połączono w tekście
    [few] Dodano { $count } pliki jako zasoby tego elementu i połączono je w tekście
    [many] Dodano { $count } plików jako zasoby tego elementu i połączono je w tekście
   *[other] Dodano { $count } pliku jako zasoby tego elementu i połączono je w tekście
}
resources-undo = Cofnij
resources-link-refused = „{ $url }” nie jest adresem internetowym, który Skribisto może zachować. Użyj adresu zaczynającego się od http:// lub https://, bez nazwy użytkownika i hasła.

## Powiadomienie dla projektu w jednym pliku

resources-folder-title = Ten projekt robi się duży
resources-folder-text = Przechowywane w nim pliki zajmują teraz { $size }. Projekt trzymany jako jeden plik jest przy każdym zapisie zapisywany w całości, razem z plikami. Trzymany jako folder zapisuje tylko to, co się zmieniło. Możesz teraz zapisać go jako folder i od tej pory pracować w folderze.
resources-folder-switch = Zapisz jako folder…
resources-folder-not-now = Nie teraz
resources-folder-never = Nie pytaj ponownie o ten projekt

## Otwieranie

resources-not-found-toast = „{ $name }” nie jest tam, gdzie projekt się go spodziewa.
resources-unsafe-title = Otworzyć „{ $name }”?
resources-unsafe-text = Skribisto nie wie, czy ten rodzaj pliku jest bezpieczny. Otwarcie go mogłoby uruchomić program na twoim komputerze, a projekt może pochodzić od kogoś innego. Najpierw pokaż go w jego folderze, aby zobaczyć, co to jest.
resources-open-anyway = Otwórz mimo to
resources-open-failed = Nie udało się otworzyć pliku: { $error }
resources-no-data-folder = ten system nie daje Skribisto żadnego folderu danych, w którym można otworzyć kopię
resources-link-gone = Ten odnośnik prowadzi do zasobu, którego projekt już nie ma.
resources-locate-title = Gdzie jest teraz „{ $name }”?
resources-relinked-more = { $count ->
    [one] W tym samym folderze znaleziono jeszcze jeden brakujący plik i połączono go ponownie.
    [few] W tym samym folderze znaleziono jeszcze { $count } brakujące pliki i połączono je ponownie.
    [many] W tym samym folderze znaleziono jeszcze { $count } brakujących plików i połączono je ponownie.
   *[other] W tym samym folderze znaleziono jeszcze { $count } brakującego pliku i połączono je ponownie.
}
resources-changed-elsewhere = „{ $name }” zmieniono w innej aplikacji. Projekt zachowuje nową wersję.

## Edycja

resources-edit-failed = Nie udało się zmienić zasobu: { $error }
resources-remove-title = Usunąć „{ $title }” z projektu?
resources-remove-text = Zostanie odłączony od każdego elementu, do którego jest dołączony. Plik przechowywany w projekcie znika razem z nim; plik połączony i strona internetowa zostają tam, gdzie są. Cofnij przywraca go.
resources-remove-confirm = Usuń
resources-remove-failed = Nie udało się usunąć zasobu: { $error }
resources-removed-toast = „{ $title }” usunięto z projektu.
resources-removed-many-toast = { $count ->
    [few] Usunięto { $count } zasoby z projektu.
   *[other] Usunięto { $count } zasobów z projektu.
}
resources-detached-toast = „{ $title }” odłączono.
resources-attached-one-toast = „{ $title }” dodano do zasobów „{ $item }”.
resources-attached-many-toast = { $count ->
    [few] Dodano { $count } zasoby do „{ $item }”.
   *[other] Dodano { $count } zasobów do „{ $item }”.
}

## Do tekstu

resources-no-text = Najpierw umieść kursor w tekście, a potem wstaw tam zasób.
resources-drop-insert-picture = Wstaw obraz
resources-drop-insert-link = Wstaw odnośnik do zasobu

## Postęp

progress-resources-storing = Kopiowanie plików
progress-resources-stored = Pliki skopiowane

## Strona Zasoby folderu

segment-folder = Folder
resources-folder-count = { $count ->
    [one] Jeden zasób
    [few] { $count } zasoby
    [many] { $count } zasobów
   *[other] { $count } zasobu
}
resources-view-label = Pokaż zasoby jako
resources-view-list = Lista
resources-view-grid = Siatka
resources-grid-label = Zasoby
resources-folder-drop-hint = Upuść tu pliki lub adresy internetowe, aby dołączyć je do tego folderu
resources-folder-empty = Do tego folderu ani do niczego, co zawiera, nic jeszcze nie dołączono. Dodaj pliki lub odnośniki internetowe przyciskami powyżej albo upuść je tutaj: zostaną dołączone do folderu.
resources-attached-to = W
resources-attached-separator = ,
resources-attached-more = { $count ->
    [one] i jeszcze jednym
    [few] i jeszcze { $count }
    [many] i jeszcze { $count }
   *[other] i jeszcze { $count }
}
resources-row-name-attached = { $name }, w { $items }
resources-open-item = Otwórz „{ $title }”
resources-detach-from = Odłącz od „{ $title }”

## Zasoby-obrazy w Galerii

gallery-resource-badge = Zasób
gallery-tile-resource = Obraz będący zasobem elementu { $title }: { $description }

## Strona Zasoby w ustawieniach projektu

settings-page-resources = Zasoby
settings-desc-resources = Pliki i odnośniki internetowe, których nie ma już żaden element: dołącz je ponownie, usuń je lub wyeksportuj kopie
settings-resources-description = Zasób nie jest dołączony do niczego, gdy odłączysz go od jego ostatniego elementu albo gdy element, do którego był dołączony, zostanie usunięty na dobre. Zostaje w projekcie, a przechowywany plik wędruje z każdą kopią i każdą kopią zapasową projektu, dopóki nie dołączysz go ponownie albo go nie usuniesz.
settings-resources-group-storage = Miejsce
settings-resources-group-unattached = Niedołączone do niczego
settings-resources-stored = Pliki przechowywane w tym projekcie: { $size }
settings-resources-folder-advice = Projekt trzymany w jednym pliku jest zapisywany od nowa w całości przy każdym zapisie, razem z przechowywanymi plikami. Trzymany jako folder zapisuje tylko to, co się zmieniło, a to jest szybsze.
settings-resources-count = { $count ->
    [one] Jeden zasób
    [few] { $count } zasoby
    [many] { $count } zasobów
   *[other] { $count } zasobu
}
settings-resources-empty = Każdy zasób tego projektu jest dołączony do jakiegoś elementu.
resources-attach-to-items = Dołącz do elementów…
resources-export-copies = Eksportuj kopie…
resources-export-copy = Eksportuj kopię…
resources-remove-many-title = Usunąć zasoby ({ $count }) z projektu?
resources-attach-title = Dołącz do elementów
resources-attach-hint = Wybierz elementy, do których dołączyć. Ctrl+kliknięcie lub Shift+kliknięcie wybiera kilka.
resources-attach-empty = Ten projekt nie ma jeszcze elementów.
resources-attach-confirm = Dołącz
resources-attached = { $count ->
    [one] Dołączono jeden zasób
    [few] Dołączono { $count } zasoby
    [many] Dołączono { $count } zasobów
   *[other] Dołączono { $count } zasobu
} do { $items ->
    [one] jednego elementu
    [few] { $items } elementów
    [many] { $items } elementów
   *[other] { $items } elementu
}
resources-export-title = Eksportuj kopie do folderu
resources-exported = { $count ->
    [one] Wyeksportowano jeden zasób
    [few] Wyeksportowano { $count } zasoby
    [many] Wyeksportowano { $count } zasobów
   *[other] Wyeksportowano { $count } zasobu
}
resources-export-missing = { $count ->
    [one] Jednego pliku nie ma tam, gdzie oczekuje go projekt, więc pominięto go: { $names }
    [few] { $count } plików nie ma tam, gdzie oczekuje ich projekt, więc pominięto je: { $names }
    [many] { $count } plików nie ma tam, gdzie oczekuje ich projekt, więc pominięto je: { $names }
   *[other] { $count } pliku nie ma tam, gdzie oczekuje go projekt, więc pominięto go: { $names }
}
resources-export-failed = { $count ->
    [one] Nie udało się skopiować jednego pliku: { $names }
    [few] Nie udało się skopiować { $count } plików: { $names }
    [many] Nie udało się skopiować { $count } plików: { $names }
   *[other] Nie udało się skopiować { $count } pliku: { $names }
}

## Ustawienie dla dużych plików (Zachowanie edytora)

settings-resources-large-file = Pliki powyżej 10 MB
settings-resources-large-ask = Pytaj za każdym razem
settings-resources-large-store = Zawsze przechowuj w projekcie
settings-resources-large-link = Zawsze łącz z plikiem
settings-hint-resources-large-file = Co dzieje się z plikiem powyżej 10 MB, który dołączasz jako zasób. Przechowywany w projekcie wędruje z każdą kopią i każdą kopią zapasową projektu. Połączony zostaje tam, gdzie jest, a projekt pamięta tylko, gdzie go znaleźć.
