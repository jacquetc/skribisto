# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

wm-book = Książka w twoim projekcie.
wm-book-more = Książka jest tak prosta lub warstwowa, jak chcesz. Krótka historia może być tylko kilka [sceny](:wm-scene). Dłuższa praca może gniazdo [części](:wm-part) i [rozdziały](:wm-chapter) tak głębokie, jak chcesz. Kształt jest twój.
wm-part = Część grupująca rozdziały.
wm-part-more = Część gromadzi [rozdziały](:wm-chapter) pod jedną pozycją. Podobnie jak rozdział, działa do następnej części lub końca [książka](:wm-book).
wm-chapter = Rozdział książki.
wm-chapter-more = Rozdział biegnie stąd aż do następnego rozdziału, następnego [część](:wm-part), lub [Koniec książki](:wm-end-of-book). Jest on w dwóch kształtach: folder rozdział, który posiada [sceny](:wm-scene), lub płaski rozdział, który nosi własną prozę. Wybierz, który nowy rozdział używać w Ustawieniach, pod Strukturą.
wm-scene = Scena prozy.
wm-scene-more = Scena trzyma twoje pismo. Każdy element zapisu ma dwie strony: główny tekst i [synopsis](:wm-synopsis) do planowania. Oba otwarte razem w edytorze.
wm-note = Notatka.
wm-note-more = Notatka przechowuje tekst i [synopsis](:wm-synopsis), jak [scena](:wm-scene), do badań, asides, lub przypomnienia. Każda pozycja może być częścią zestawionej książki, w tym notatka siedzi wewnątrz [dział](:wm-chapter).
wm-note-folder = Notatka, która zawiera inne przedmioty.
wm-note-folder-more = [Uwaga](:wm-note), który zawiera również elementy pod nim, więc można grupować powiązane notatki razem. Zachowuje swoje własne [synopsis](:wm-synopsis), a elementy wewnątrz zachować swój własny tekst.
wm-folder = Folder organizacyjny.
wm-folder-more = Pojemnik do układania przedmiotów w segregatorze jednak pasuje do Ciebie. Ma [synopsis](:wm-synopsis), ale nie ma prozy własnej.
wm-end-of-book = Znaki tam, gdzie kończy się książka.
wm-end-of-book-more = Każda książka dzieli się jedną ciągłą listą, więc koniec książki nie jest decydowana przez gniazdowanie. Ten marker mówi [książka](:wm-book) zatrzymuje się tutaj. Wszystko po należy do następnej książki.
wm-story-bible-entry = Oznaczona, zniesławiona notatka o twojej historii.
wm-story-bible-entry-more = [Uwaga](:wm-note) o znaku, lokalizacji, lub cokolwiek innego wartego zgłoszenia, utworzone z nazwą, tagami, pseudonimami i szablon startowy wszystko w jednym kroku. Nic na stronie odróżnia go później, ponieważ pozostaje zwykłą nutą można zmienić nazwę, przeetykietować lub konwertować jak każdy inny.
wm-synopsis = Podsumowanie pozycji pisemnej.
wm-synopsis-more = Podsumowanie dołączone do każdej pozycji zapisu, zazwyczaj krótki akapit, ale tak długo, jak chcesz. Użyj go do planowania i nawigacji przed prozą istnieje. Znajduje się obok głównego tekstu w edytorze.

scene-break-minor = Zwykła przerwa w scenie: zmiana czasu, miejsca lub punktu widzenia w obrębie rozdziału.
scene-break-minor-more =
    Oznacza przerwę *w miejscu, w którym ją wstawisz*, również w środku [sceny](:wm-scene). Podział tekstu na dwa elementy służy organizacji i sam w sobie nie tworzy przerwy.

    Wpisany do prozy jako trzy gwiazdki oddzielone spacjami. Jak to drukuje decyduje styl eksportu, nie przez to, co wpisujesz: Shunn rękopis ustawia go jako jeden #, papier handlowy jako dinkus, a najbardziej francuski, niemiecki, hiszpański, rosyjski i włoski wydawnictwo jako nagą lukę bez żadnego znaku.
scene-break-major = Silniejszy podział: duża zmiana czasu lub decydująca zmiana punktu widzenia.
scene-break-major-more =
    Ten sam pomysł, co zwykła przerwa w scenie, jeden krok w górę.

    Wpisany jako # # #, i drukowany inaczej od zwykłego poziomu według stylu eksportu. Standardowy format rękopisu Shunn ustawia jeden # przeciwko # # dla dokładnie tego rozróżnienia. Tam, gdzie tradycja nie ma mocniejszego znaku, oba poziomy drukować to samo.
wm-find-in-prose = Skribisto szuka tego elementu w twojej prozie.
wm-find-in-prose-more = Włącz to dla tagów, które nazywają rzeczy, o których piszesz: znaki, miejsca, obiekty. Każdy element noszący taki znacznik jest dopasowany do twojej prozy przez jego tytuł i przez inne nazwy, które mu dajesz, więc każdy [scena](:wm-scene) list, który i co pojawia się w nim bez łączenia czegokolwiek ręcznie. Zostaw go dla tagów, które opisują element, a nie nazwę jednego, takich jak status projektu lub przypomnienie, aby sprawdzić ciągłość. Każdy element, który pasuje również łączy się z Twoim [biblia historyczna](:wm-story-bible).
wm-story-bible = Każdy znak, miejsce i inne oznaczone wpis, zebrane w jednej siatce.
wm-story-bible-more = Otwórz z każdej karty folderu notatek. Grupa kart po tagu, a każda nazwa jak wiele aliasów odpowiedzi na wpis i jak wiele scen, w którym się znalazł, wyciągnięta z każdego znacznika [znaleźć w prozie](:wm-find-in-prose) w projekcie. Filtrowanie tylko książki zawęża, które karty pokazują; nigdy nie decyduje, które wpisy istnieją.

tooltip-go-to = Przejdź do dowolnego elementu w segregatorze (Ctrl + G)
synopsis-collapse-tooltip = Ukryj kolumnę synopsis

wm-paratext = Tekst, który nie jest częścią historii.
wm-paratext-more = Przedstawienie, dedykacja, po słowie, siarka: pisanie, które należy do książki, ale nie do jej ciała. Wykazane gdziekolwiek umieścić go, i nigdy nie liczone w rękopis słowo liczyć. Gdzie idzie zależy od Ciebie: konwencje różnią się według kraju i wydawcy.
wm-paratext-folder = Folder dla paratextów.
wm-paratext-folder-more = W miejscu, gdzie można zachować preface i słowa wtórne, aby nie zaciskać segregatora. Organizowanie tylko: ma [synopsis](:wm-synopsis), ale nie dodaje nic do książki eksportowanej, nawet jej własnej nazwy.

goal-target = Jak długo ten kawałek ma być.
goal-target-more = Ustaw cel na wszystko: [scena](:wm-scene), [dział](:wm-chapter), [część](:wm-part), cały [książka](:wm-book). Każdy z nich jest swoją własną liczbą i stoi na swoim własnym, więc ustawienie celu na rozdziale nigdy nie zmienia książki, a cel książki nigdy nie jest sumą rozdziałów wewnątrz niego. Co dodają jest pisanie: pojemnik [postęp](:goal-progress) jest wszystko napisane pod nim. Zostaw cel na zero i po prostu nie jest ustawiony. Liczba ta jest liczona w [słowa lub znaki](:goal-unit), w zależności od tego, który projekt wykorzystuje.
goal-unit = Czy ten projekt liczy się słowami czy znakami.
goal-unit-more = Szeroki wybór projektu, ponieważ jednostka długości należy do rękopisu i jego rynku, a nie do jednej sekcji: Niemiecki i francuski środek wydawniczy w znakach, japoński w 400- znaków arkuszy, chiński przez tysiąc znaków, większość angielskich publikacji językowych w słowach. Obie liczby są trzymane obok siebie i nigdy nie jest przeliczane na inne, więc przełączanie jednostki punktów co [docelowy](:goal-target) na drugiej figurze i przełączanie z powrotem przywraca pierwszy. Co to nie robi jest przekonwertować cokolwiek, więc cele już wprowadzone będzie trzeba uaktualnić ręcznie.
goal-progress = Ile jest napisane, przeciwko celowi.
goal-progress-explained = Kawałek z własną prozą mierzy się w stosunku do własnej długości; kontener przeciw [Wszystko, co pod nim napisane](:goal-manuscript-words). Pasek ogrzewa się od czerwonego przez bursztyn do zielonego, gdy pojawia się pismo, i zmienia się ponownie, gdy utwór przebiega dobrze, co było planowane, co warto zauważyć, gdy rozdział ma długość do utrzymania.
goal-manuscript-words = Co liczy się jako rękopis.
goal-manuscript-words-more = Tylko proza, która faktycznie byłaby eksportowana: [sceny](:wm-scene) i [rozdziały](:wm-chapter) niosąc własny tekst. [Uwaga](:wm-note) lub [parateks](:wm-paratext) nie jest częścią rękopisu i nigdy nie jest policzone w kierunku jednego. Wszystko w śmieciach spada, i tak robi coś [pozostający poza wywozem](:goal-exportable).
goal-exportable = Pozycja ta pozostaje poza wywozem.
goal-exportable-more = Jego własna długość nadal pokazuje, ponieważ pismo jest nadal tam, ale liczy się nie ogółem, dokładnie tak, jak to pojawi się w żadnej książce eksportowanej. Pozostawienie kontenera na zewnątrz nie pozostawia elementów wewnątrz niego na zewnątrz: każdy element odpowiada za siebie, co jest tym, co przycisk "Zastosuj do dzieci" obok przełącznika jest do.
goal-distribute = Podziel się tym celem przez to, co jest w środku.
goal-distribute-more = Dzieli [książki](:wm-book) lub [Część](:wm-part) cel na swoich bezpośrednich dzieci, więc kawałki sumują się do niego dokładnie. Domyślnie wypełnia tylko te bez celu jeszcze i pozostawia swoje własne numery same. To, co pisze są zwykłe [cele](:goal-target), wolne do edycji później i swobodnie dryfować od siebie: jest to jedno działanie, a nie stałe porozumienie.
goal-subtree-total = To, do czego służą cele wewnątrz.
goal-subtree-total-more = Fakt, nie cel. Ten projekt nigdy nie traktuje celu kontenera jako sumy tych wewnątrz niego, ponieważ tak figura folderu kończy się poruszając się na własną rękę, gdy scena wewnątrz niego jest podana liczba. Ta linia po prostu mówi, do czego dochodzi twój własny numer, więc można go porównać z [docelowy](:goal-target) pojemnika samodzielnie, lub [podziel się jednym z nich](:goal-distribute).
goal-milestone = Randka, żeby coś osiągnąć.
goal-milestone-more = Dwa rodzaje. Jeden mówi, że [książka](:wm-book) powinien stać na danej długości o dacie, sposób na jego własnej krzywej. Drugi mówi, że określony [dział](:wm-chapter) lub [część](:wm-part) powinien osiągnąć [jego własny cel](:goal-target) wtedy, i czyta, że liczba żyć zamiast trzymać kopię, więc edycja celu uaktualnia kamień milowy zbyt. Oba pojawiają się na [plan tempa](:pace-plan) książki.
pace-plan = Książka pisze grafik.
pace-plan-more = [docelowy](:goal-target) i termin, z dni piszesz na, zamienił się w ile potrzebuje dzień. Czyta własny cel książki, więc ustawienie go tutaj i ustawienie go w inspektorze są takie same liczby w dwóch miejscach. Waypoints po drodze są [kamienie milowe](:goal-milestone).

concept-tag = Barwna etykieta, którą można umieścić na dowolnym elemencie, nadająca się do wielokrotnego użytku w całym projekcie.
concept-tag-more = Tag element, aby go oznaczyć: status, lokalizacja, wątek, który śledzisz. Ten sam znacznik może usiąść na dowolnej liczbie elementów, a jeden element może nosić kilka. Włącz [znaleźć w prozie](:wm-find-in-prose) dla znacznika, który nazywa coś w książce, znak lub miejsce, i Skribisto zaczyna go szukać w prozie.
concept-label = Krótka notatka, którą piszesz pod tytułem danego przedmiotu, na własne oczy.
concept-label-more = Nie [tag](:concept-tag): etykieta należy tylko do jednego elementu, zwykły tekst bez koloru lub katalogu za nim, coś jak "1 punkt wykresu" lub "potrzebuje nazwy". Ustaw ją z menu kontekstowego elementu, lub edytuj ją inline w kolumnie Overview 's Label; pokazuje jako mały podtytuł pod tytułem, w obrysie i w strumieniu.
concept-point-of-view = Czyje oczy scena jest narratorem: jeden lub więcej obsady twojej historii.
concept-point-of-view-more = Ustawienie w Inspektorze, obok gipsu. Wybór kogoś jeszcze nie w gipsie dodaje je tam zbyt. Więcej niż jeden punkt widzenia jest prawnym wyborem, przydatne dla wspólnej sceny lub zmiana oczu mid- book, nie coś Skribisto bloki. Kandydaci pochodzą z pozycji [znaleźć w prozie](:wm-find-in-prose) tag umieszcza w Biblii historii.
concept-epigraph = Cytat ustawiony na czele części lub rozdziału.
concept-epigraph-more = Jego własne pole, oddzielone od [dział](:wm-chapter) lub [część](:wm-part) własnej prozy; [scena](:wm-scene) lub [Uwaga](:wm-note) nie nosi żadnego. Napisz jeden jako zwykły cytat; napisz kilka i każdy staje się swoją własną blockquote, wydrukowany jako własny epigraph. Jego słowa nigdy nie są liczone jako rękopis słowa.
concept-footnote = Cytat oznaczony w prozie, drukujący jako nuta ponumerowana.
concept-footnote-more = Odniesienie jest pojedynczym znakiem wewnątrz twoich słów, więc porusza się i usuwa dokładnie tak, jak słowa wokół niego. Jego numer nigdy nie jest przechowywany: Skribisto pracuje go świeże, z którego każde odniesienie siedzi w całym rękojeści, tak wstawiając jeden renumbers każdy notatka po nim. Przypis własne słowa są liczone oprócz rękopisu, nigdy złożone w nim.
concept-chapter-mode = Czy nowy rozdział jest folderem scen czy pojedynczym płaskim elementem.
concept-chapter-mode-more = Ustawienie w Ustawieniach pod Strukturą. Piszesz bezpośrednio do [dział](:wm-chapter) tak czy inaczej; jedyną różnicą jest to, czy może ona również zawierać sceny pod nim. Oba kodowania kompilować do tej samej książki, a można je swobodnie mieszać w jednym projekcie. Promowanie konwertuje istniejący rozdział między tymi dwoma bez utraty słowa.
concept-comment = Nota zakotwiczona do dokładnego rozciągania prozy, pokazana na marginesie.
concept-comment-more = Pamięta słowa, na które wskazuje i co ich otacza, a nie stałe położenie, więc przetrwa zwykłą edycję i pełne przeładowanie. Odpowiadaj pod spodem, aby utrzymać wątek w ruchu. Jeśli dokładny cytat nie może być już znaleziony, komentarz mówi tak, a nie po cichu dryfując do złego zdania.
concept-backup = Kopia czasu-punktu całego projektu, trzymana obok niego.
concept-backup-more = Wybrane w harmonogramie: na otwartym, zamkniętym, w odstępie czasu, lub dowolnej kombinacji, ze starymi kopiami przycinanymi według wieku lub liczenia. Otwarcie jednego ładuje go we własnym oknie: można patrzeć, a nawet edytować, ale tylko Zapisz jako lub Przywracanie zachowuje cokolwiek. Różne od [wersja](:concept-version), który śledzi własną przeszłość jednego elementu.
concept-version = Przeszłość jednego elementu: każda zmiana tekstu lub synopsis.
concept-version-more = Zbudowany z własnej historii zapisu projektu i jego kopie zapasowe razem, jeden wpis na zmianę, a nie jeden na plik. Wybierz wpis, aby zobaczyć, co się zmieniło i przywrócić tylko tekst tego elementu do niego; reszta projektu jest nietknięty, a kopia bezpieczeństwa jest najpierw. Inny od [kopia zapasowa](:concept-backup), który jest kopią wszystkiego.
concept-trash = Zdemolowany przedmiot jest ukryty, nie przeniesiony: pozostaje dokładnie tam, gdzie był.
concept-trash-more = Zmiażdżenie tylko przewraca jedną flagę: element zachowuje swoje miejsce w segregatorze, oznaczony jako nieaktywny, a nie usunięty. Przywraca go i pojawia się ponownie w dokładnie tym miejscu, bez ponownego zgłoszenia. Pusty Kosz usuwa zniszczone przedmioty na dobre, a każdy z nich jest jednym niewykonalnym krokiem, jak każda inna edycja projektu.
concept-spellcheck = Podkreśla słowa, których nie rozpoznaje zainstalowany słownik, według języka projektu.
concept-spellcheck-more = Każdy projekt ma swój własny język roboczy lub języki; element może je obejść dla sceny napisanej w innym języku. Dodaj słowo Skribisto nie wie z menu kontekstowego edytora, do osobistego słownika, który przenosi się do każdego projektu. Przekręcanie spell- check zatrzymuje go wszędzie, aż go z powrotem włączyć.
concept-search-replace = Znajduje słowo lub frazę wszędzie w projekcie, nie tylko otwarty dokument.
concept-search-replace-more = Wygląda na każdą scenę, notatkę, tytuł, synopsis i etykietę, a przez komentarze i przypisy zbyt. Case, w całości dopasowanie słowa i dopasowanie akcentu są trzy oddzielne przełączniki. Zastąp wszystkie przepisywania każdego przeglądu meczu w jednym przełęczy i jeden krok cofnięcia, więc Ctrl + Z pobiera z powrotem całą partię, nigdy tylko ostatnia zmiana.
concept-note-template = Wielofunkcyjny kawałek pisania wstawić do wszystkiego, nad czym pracujesz.
concept-note-template-more = Pusty arkusz znaków, profil lokalizacji, bitowy arkusz: napisz go raz i upuść go tam, gdzie go potrzebujesz, w dowolnym elemencie, nie tylko notatkę. Zacznij od built- in preset, zaimportuj plik .md lub .djot, lub napisz coś i wybierz dokument ▸ Zapisz jako szablon. Przechowywany w projekcie, więc współautor, który otwiera go dostaje ten sam zestaw.
concept-text-replacement = Twój własny skrót, rozszerzony automatycznie jak piszesz.
concept-text-replacement-more = Ustaw wyzwalacz i jego zamiennik, "btw" na "przy okazji", i rozszerza moment wpisywania przestrzeni lub interpunkcji po nim. Przypadek następuje to, co wpisałeś: kapitalizacja wyzwalacza i ekspansja kapitalizuje zbyt. A Backspace zaraz po rozszerzeniu odtwarza tylko to jedno zastąpienie. Włączony na projekt, różni się od [inteligentna interpunkcja](:concept-smart-punctuation).
concept-smart-punctuation = Typografia Locale- eaware stosowana automatycznie, jak piszesz: cudzysłów, kreski, elipsy.
concept-smart-punctuation-more = Curls proste cytaty w typograficznych, zamienia -- w en dash i --- w em dash, zamienia... w elipsę, a gdzie go włączyć, otwiera paragraf wpisany jako "-" z deski dialogowej. Ustawienie na projekt, i podróżuje wewnątrz .skrib więc współautor pisze z tymi samymi zasadami. Oddzielny od [Twoje własne zasady](:concept-text-replacement), który piszesz sam.
concept-export-style = Nazwa pakietu wyborów eksportowych: nagłówki, odstępy, co jest zawarte.
concept-export-style-more = Jeden styl, ponownie używany w każdym eksporcie dowolny format wybrać. Built- w stylach są read- tylko; duplikat jeden, aby uzyskać punkt wyjścia można rzeczywiście zmienić. Wśród jego wyborów jest to, czy włączyć [markery rejsu okrągłego](:concept-round-trip-marks), dla pliku związanego dla edytora i z powrotem.
concept-round-trip-marks = Niewidzialne zakładki, które pozwalają edytowane DOCX lub ODT wrócić do domu uznane.
concept-round-trip-marks-more = Zapisany tylko w dwóch formatach edytor może wysłać z powrotem, DOCX i ODT; każdy inny eksport nie nosi żadnego. Zakładka, a nie niestandardowy atrybut: LibreOffice paski niestandardowe atrybuty na zapisać, ale pozostawia zakładkę nienaruszone. Turned on per [styl eksportu](:concept-export-style), pod "Wysyłanie do edytora".
concept-status = Gdzie element stoi we własnym strumieniu pracy: jeden etap na raz, posuwając się do przodu.
concept-status-more = Nie [tag](:concept-tag): element ma dokładnie jeden status, etapy są zamawiane, a projekt jest właścicielem listy, więc można zmienić nazwę, ponownie zamówić lub dodać do niego, kiedy chcesz. Gwiazdka obok rozdziału lub książki oznacza części poniżej nie wszystkie zgadzają się z nim. Ustaw go od Inspektora, karty, elementu strumienia, lub tutaj.
