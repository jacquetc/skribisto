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

## Do tekstu

resources-no-text = Najpierw umieść kursor w tekście, a potem wstaw tam zasób.
resources-drop-insert-picture = Wstaw obraz
resources-drop-insert-link = Wstaw odnośnik do zasobu

## Postęp

progress-resources-storing = Kopiowanie plików
progress-resources-stored = Pliki skopiowane
