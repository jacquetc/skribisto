# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

wm-book = Książka w Twoim projekcie.
wm-book-more = Książka może być tak prosta lub tak wielowarstwowa, jak chcesz. Opowiadanie może się składać z zaledwie kilku [scen](:wm-scene). Dłuższe dzieło może zagnieżdżać [części](:wm-part) i [rozdziały](:wm-chapter) tak głęboko, jak zechcesz. Kształt zależy od Ciebie.
wm-part = Część grupująca rozdziały.
wm-part-more = Część zbiera [rozdziały](:wm-chapter) pod jednym nagłówkiem. Podobnie jak rozdział trwa do następnej części lub do końca [książki](:wm-book).
wm-chapter = Rozdział książki.
wm-chapter-more = Rozdział trwa od tego miejsca do następnego rozdziału, następnej [części](:wm-part) albo [końca książki](:wm-end-of-book). Ma dwie postaci: rozdział-folder, który zawiera [sceny](:wm-scene), albo rozdział płaski, który ma własną prozę. To, której postaci używają nowe rozdziały, wybierasz w ustawieniach, w sekcji „Struktura”.
wm-scene = Scena z prozą.
wm-scene-more = Scena zawiera Twój tekst. Każdy element z tekstem ma dwie strony: tekst główny i [streszczenie](:wm-synopsis) do planowania. Obie otwierają się razem w edytorze.
wm-note = Notatka.
wm-note-more = Notatka zawiera tekst i [streszczenie](:wm-synopsis), tak jak [scena](:wm-scene), i służy do materiałów, dygresji lub przypomnień. Każdy element może być częścią skompilowanej książki, także notatka umieszczona wewnątrz [rozdziału](:wm-chapter).
wm-note-folder = Notatka zawierająca inne elementy.
wm-note-folder-more = [Notatka](:wm-note), która zawiera też elementy pod sobą, dzięki czemu można grupować powiązane notatki. Ma własne [streszczenie](:wm-synopsis), a elementy wewnątrz mają własny tekst.
wm-folder = Folder porządkowy.
wm-folder-more = Kontener do układania elementów w segregatorze tak, jak Ci odpowiada. Ma [streszczenie](:wm-synopsis), ale nie ma własnej prozy. Tekst zawierają elementy w jego wnętrzu.
wm-end-of-book = Oznacza miejsce, w którym kończy się książka.
wm-end-of-book-more = Wszystkie książki dzielą jedną ciągłą listę, więc o końcu książki nie decyduje zagnieżdżenie. Ten znacznik mówi, że [książka](:wm-book) kończy się w tym miejscu. Wszystko, co jest po nim, należy do następnej książki.
wm-story-bible-entry = Notatka o Twojej opowieści, ze znacznikami i innymi nazwami.
wm-story-bible-entry-more = [Notatka](:wm-note) o postaci, miejscu lub czymkolwiek innym, co warto skatalogować, utworzona w jednym kroku z nazwą, znacznikami, innymi nazwami i szablonem początkowym. Później nic na stronie jej nie wyróżnia: pozostaje zwykłą notatką, której nazwę, znaczniki i typ można zmieniać jak każdej innej.
wm-synopsis = Streszczenie elementu z tekstem.
wm-synopsis-more = Streszczenie dołączone do każdego elementu z tekstem, zwykle krótki akapit, ale może być dowolnie długie. Służy do planowania i poruszania się po tekście, zanim powstanie proza. W edytorze znajduje się obok tekstu głównego.

scene-break-minor = Zwykły separator sceny: zmiana czasu, miejsca lub punktu widzenia w obrębie rozdziału.
scene-break-minor-more =
    Oznacza przerwę *w miejscu, w którym ją wstawisz*, również w środku [sceny](:wm-scene). Podział prozy na dwa elementy jest decyzją organizacyjną, więc sam nigdy nie tworzy przerwy.

    Wpisany do prozy jako trzy gwiazdki oddzielone spacjami. Jak to drukuje decyduje styl eksportu, nie przez to, co wpisujesz: Shunn rękopis ustawia go jako jeden #, papier handlowy jako dinkus, a najbardziej francuski, niemiecki, hiszpański, rosyjski i włoski wydawnictwo jako nagą lukę bez żadnego znaku.
scene-break-major = Mocniejszy podział: duży przeskok w czasie lub zdecydowana zmiana punktu widzenia.
scene-break-major-more =
    Ta sama idea co zwykły separator sceny, o stopień wyżej. Użyj go, gdy zwykły separator nie oddałby skali przeskoku.

    Wpisany jako # # #, i drukowany inaczej od zwykłego poziomu według stylu eksportu. Standardowy format rękopisu Shunn ustawia jeden # przeciwko # # dla dokładnie tego rozróżnienia. Tam, gdzie tradycja nie ma mocniejszego znaku, oba poziomy drukować to samo.
wm-find-in-prose = Skribisto szuka tego elementu w Twojej prozie.
wm-find-in-prose-more = Włącz tę opcję dla znaczników, które nazywają to, o czym piszesz: postacie, miejsca, przedmioty. Każdy element z takim znacznikiem jest wyszukiwany w prozie po tytule i innych nadanych mu nazwach, więc każda [scena](:wm-scene) pokazuje, kto i co się w niej pojawia, bez ręcznego łączenia czegokolwiek. Pozostaw ją wyłączoną dla znaczników, które opisują element, a nie go nazywają, np. etap pracy czy przypomnienie o sprawdzeniu ciągłości. Każdy dopasowany element trafia też do Twojej [biblii opowieści](:wm-story-bible).
wm-story-bible = Wszystkie postacie, miejsca i inne oznaczone wpisy zebrane w jednej siatce.
wm-story-bible-more = Otwierana z karty dowolnego folderu notatek. Karty są pogrupowane według znaczników, a każda podaje, na ile innych nazw odpowiada wpis i w ilu scenach się pojawił, na podstawie wszystkich znaczników [wyszukiwanych w prozie](:wm-find-in-prose) w projekcie. Filtrowanie według książki zawęża tylko widoczne karty; nigdy nie decyduje o tym, które wpisy istnieją.

tooltip-go-to = Przejdź do dowolnego elementu w segregatorze (Ctrl+G)
synopsis-collapse-tooltip = Ukryj kolumnę streszczenia

wm-paratext = Tekst, który nie jest częścią opowieści.
wm-paratext-more = Przedmowa, dedykacja, posłowie, kolofon: teksty należące do książki, ale nie do jej głównej części. Eksportowane tam, gdzie je umieścisz, i nigdy nie wliczane do liczby słów rękopisu. Gdzie trafią, zależy od Ciebie: zwyczaje różnią się w zależności od kraju i wydawcy.
wm-paratext-folder = Folder na parateksty.
wm-paratext-folder-more = Miejsce na przedmowy i posłowia, aby nie zaśmiecały segregatora. Służy tylko do porządkowania: ma [streszczenie](:wm-synopsis), ale nie dodaje niczego do wyeksportowanej książki, nawet własnej nazwy.

goal-target = Jak długi ma być ten fragment.
goal-target-more = Cel można ustawić dla wszystkiego: [sceny](:wm-scene), [rozdziału](:wm-chapter), [części](:wm-part), całej [książki](:wm-book). Każdy cel to osobna liczba, niezależna od innych, więc ustawienie celu rozdziału nigdy nie zmienia celu jego książki, a cel książki nigdy nie jest sumą rozdziałów w niej zawartych. Sumuje się natomiast tekst: [postęp](:goal-progress) kontenera to wszystko, co napisano w jego wnętrzu. Cel równy zero po prostu nie jest ustawiony. Liczba jest liczona w [słowach lub znakach](:goal-unit), zależnie od ustawienia projektu.
goal-unit = Czy ten projekt liczy w słowach, czy w znakach.
goal-unit-more = Wybór dla całego projektu, ponieważ jednostka długości należy do rękopisu i jego rynku, a nie do jednej sekcji: wydawnictwa niemieckie i francuskie liczą znaki, japońskie arkusze po 400 znaków, chińskie tysiące znaków, a większość anglojęzycznych słowa. Obie liczby są przechowywane obok siebie i żadna nie jest przeliczana na drugą, więc zmiana jednostki kieruje każdy [cel](:goal-target) na drugą wartość, a powrót przywraca pierwszą. Niczego natomiast nie przelicza, więc już wprowadzone cele trzeba zaktualizować ręcznie.
goal-progress = Ile napisano w stosunku do celu.
goal-progress-explained = Fragment z własną prozą jest mierzony własną długością; kontener wszystkim, [co napisano w jego wnętrzu](:goal-manuscript-words). Pasek zmienia kolor od czerwonego przez bursztynowy do zielonego w miarę przybywania tekstu i zmienia się ponownie, gdy fragment wyraźnie przekroczy zaplanowaną długość, co warto zauważyć, gdy rozdział ma określoną długość.
goal-manuscript-words = Co liczy się jako rękopis.
goal-manuscript-words-more = Tylko proza, która rzeczywiście zostałaby wyeksportowana: [sceny](:wm-scene) i [rozdziały](:wm-chapter) z własnym tekstem. [Notatka](:wm-note) ani [paratekst](:wm-paratext) nie są częścią rękopisu i nigdy nie są do niego wliczane. Wypada też wszystko, co jest w koszu, oraz wszystko, co zostało [wyłączone z eksportu](:goal-exportable).
goal-exportable = Ten element jest wyłączony z eksportu.
goal-exportable-more = Jego długość jest nadal wyświetlana, bo tekst wciąż istnieje, ale nie wlicza się do żadnej sumy, tak samo jak nie pojawi się w żadnej wyeksportowanej książce. Wyłączenie kontenera nie wyłącza elementów w jego wnętrzu: każdy element odpowiada sam za siebie i do tego służy przycisk „Zastosuj do elementów podrzędnych” obok przełącznika.
goal-distribute = Rozdziel ten cel między zawartość.
goal-distribute-more = Dzieli cel [książki](:wm-book) lub [części](:wm-part) między jej bezpośrednie elementy podrzędne, tak aby fragmenty sumowały się dokładnie do niego. Domyślnie wypełnia tylko elementy bez celu i nie zmienia Twoich własnych liczb. Zapisuje zwykłe [cele](:goal-target), które można potem dowolnie edytować i które mogą się rozejść: to jednorazowa akcja, a nie stałe powiązanie.
goal-subtree-total = Suma celów wewnątrz.
goal-subtree-total-more = Fakt, a nie cel. Ten projekt nigdy nie traktuje celu kontenera jako sumy celów w jego wnętrzu, bo wtedy wartość folderu zmieniałaby się sama, gdy scena w jego wnętrzu dostanie liczbę. Ten wiersz mówi tylko, ile dają Twoje własne liczby, abyś mógł samodzielnie porównać je z [celem](:goal-target) kontenera albo [rozdzielić go](:goal-distribute).
goal-milestone = Data, do której coś ma zostać osiągnięte.
goal-milestone-more = Dwa rodzaje. Jeden mówi, że [książka](:wm-book) ma osiągnąć określoną długość do danej daty, jako punkt pośredni na jej własnej krzywej. Drugi mówi, że konkretny [rozdział](:wm-chapter) lub [część](:wm-part) ma do tego czasu osiągnąć [swój własny cel](:goal-target), i odczytuje tę liczbę na bieżąco zamiast przechowywać jej kopię, więc edycja celu aktualizuje też kamień milowy. Oba pojawiają się w [planie tempa](:pace-plan) książki.
pace-plan = Harmonogram pisania książki.
pace-plan-more = [Cel](:goal-target) i termin, wraz z dniami, w które piszesz, przeliczone na to, ile trzeba napisać dziennie. Plan odczytuje cel samej książki, więc ustawienie go tutaj i w inspektorze to ta sama liczba w dwóch miejscach. Punkty pośrednie po drodze to [kamienie milowe](:goal-milestone).

concept-tag = Kolorowy znacznik, który można nadać dowolnemu elementowi i używać w całym projekcie.
concept-tag-more = Nadaj elementowi znacznik, aby go oznaczyć: etap, miejsce, śledzony wątek. Ten sam znacznik może mieć dowolnie wiele elementów, a jeden element może mieć kilka znaczników. Włącz [wyszukiwanie w prozie](:wm-find-in-prose) dla znacznika, który nazywa coś w Twojej książce, postać lub miejsce, a Skribisto zacznie szukać tego w Twojej prozie.
concept-label = Krótka adnotacja pisana pod tytułem elementu, dla Ciebie.
concept-label-more = To nie [znacznik](:concept-tag): etykieta należy tylko do jednego elementu, to zwykły tekst bez koloru i katalogu, np. „1. punkt zwrotny” lub „wymyślić imię”. Ustawisz ją z menu kontekstowego elementu lub edytując bezpośrednio w kolumnie „Etykieta” przeglądu; wyświetla się jako mały podtytuł pod tytułem, zarówno w konspekcie, jak i w strumieniu.
concept-point-of-view = Oczami kogo opowiadana jest scena: jedna lub więcej postaci z obsady Twojej opowieści.
concept-point-of-view-more = Ustawiany w inspektorze, obok obsady. Wybranie kogoś, kogo jeszcze nie ma w obsadzie, dodaje tę postać również tam. Więcej niż jeden punkt widzenia jest dozwolony, przydaje się przy wspólnej scenie lub zmianie perspektywy w środku książki; Skribisto tego nie blokuje. Kandydaci pochodzą z elementów, które znacznik [wyszukiwany w prozie](:wm-find-in-prose) umieszcza w Twojej biblii opowieści.
concept-epigraph = Cytat umieszczony na początku części lub rozdziału.
concept-epigraph-more = Osobne pole, niezależne od prozy [rozdziału](:wm-chapter) lub [części](:wm-part); [scena](:wm-scene) ani [notatka](:wm-note) go nie mają. Wpisz jedno motto jako zwykły cytat; wpisz kilka, a każde stanie się osobnym cytatem blokowym, drukowanym jako osobne motto. Jego słowa nigdy nie są liczone jako słowa rękopisu.
concept-footnote = Odwołanie zaznaczone w prozie, drukowane jako numerowany przypis.
concept-footnote-more = Odwołanie to pojedynczy znak wewnątrz Twojego tekstu, więc przesuwa się i usuwa dokładnie tak jak słowa wokół niego. Jego numer nigdy nie jest przechowywany: Skribisto wylicza go na nowo z położenia wszystkich odwołań w całym rękopisie, więc wstawienie jednego przenumerowuje wszystkie kolejne przypisy. Słowa samego przypisu są liczone oddzielnie od rękopisu, nigdy do niego nie wliczane.
concept-chapter-mode = Czy nowy rozdział jest folderem scen, czy pojedynczym płaskim elementem.
concept-chapter-mode-more = Ustawiane w ustawieniach, w sekcji „Struktura”. W obu przypadkach piszesz bezpośrednio w [rozdziale](:wm-chapter); jedyna różnica polega na tym, czy może on też zawierać sceny pod sobą. Obie postaci kompilują się do tej samej książki i można je swobodnie mieszać w jednym projekcie. Polecenie „Przekształć w” zmienia postać istniejącego rozdziału bez utraty choćby słowa.
concept-comment = Komentarz zakotwiczony w konkretnym fragmencie prozy, wyświetlany na marginesie.
concept-comment-more = Zapamiętuje słowa, na które wskazuje, i ich otoczenie, a nie stałą pozycję, więc przetrwa zwykłą edycję i pełne ponowne wczytanie. Odpowiadaj pod nim, aby prowadzić wątek. Jeśli dokładnego cytatu nie da się już znaleźć, komentarz to sygnalizuje, zamiast po cichu przesunąć się do niewłaściwego zdania.
concept-backup = Kopia całego projektu z określonej chwili, przechowywana obok niego.
concept-backup-more = Tworzona według wybranego harmonogramu: przy otwieraniu, przy zamykaniu, co określony czas lub w dowolnym połączeniu tych opcji, a stare kopie są usuwane według wieku lub liczby. Otwarta kopia wczytuje się we własnym oknie: można ją przeglądać, a nawet edytować, ale cokolwiek zachowa tylko „Zapisz jako” lub „Przywróć”. To coś innego niż [wersja](:concept-version), która śledzi przeszłość jednego elementu.
concept-version = Przeszłość jednego elementu: każda zmiana jego tekstu lub streszczenia.
concept-version-more = Tworzona na podstawie historii zapisów projektu i jego kopii zapasowych łącznie, jeden wpis na zmianę, a nie na plik. Wybierz wpis, aby zobaczyć, co się zmieniło, i przywrócić do niego tylko tekst tego elementu; reszta projektu pozostaje nietknięta, a najpierw tworzona jest kopia bezpieczeństwa. To coś innego niż [kopia zapasowa](:concept-backup), która jest kopią wszystkiego.
concept-trash = Element w koszu jest ukryty, a nie przeniesiony: zostaje dokładnie tam, gdzie był.
concept-trash-more = Przeniesienie do kosza zmienia tylko jedną flagę: element zachowuje swoje miejsce w segregatorze, oznaczony jako nieaktywny, a nie usunięty. Po przywróceniu pojawia się dokładnie w tym samym miejscu, bez ponownego porządkowania. „Opróżnij kosz” usuwa elementy z kosza na dobre, a każda z tych operacji to jeden krok, który można cofnąć, jak każdą inną zmianę w projekcie.
concept-spellcheck = Podkreśla słowa, których nie rozpoznaje żaden zainstalowany słownik, według języka projektu.
concept-spellcheck-more = Każdy projekt ma własny język lub języki robocze; element może je zmienić dla sceny napisanej w innym języku. Słowo, którego Skribisto nie zna, dodasz z menu kontekstowego edytora do słownika własnego, używanego we wszystkich projektach. Wyłączenie sprawdzania pisowni zatrzymuje je wszędzie, dopóki nie włączysz go ponownie.
concept-search-replace = Znajduje słowo lub frazę w całym projekcie, nie tylko w otwartym dokumencie.
concept-search-replace-more = Przeszukuje każdą scenę, notatkę, tytuł, streszczenie i etykietę, a także komentarze i przypisy. Wielkość liter, dopasowanie całych słów i znaki diakrytyczne to trzy osobne przełączniki. „Zastąp wszystko” zmienia każde przejrzane trafienie za jednym razem i jednym krokiem cofania, więc Ctrl+Z cofa całą partię, a nie tylko ostatnią zmianę.
concept-note-template = Fragment tekstu wielokrotnego użytku, który wstawiasz do tego, nad czym pracujesz.
concept-note-template-more = Pusta karta postaci, profil miejsca, rozpiska scen: napisz raz i wstawiaj, gdzie potrzebujesz, do dowolnego elementu, nie tylko notatki. Zacznij od wbudowanego szablonu, zaimportuj plik .md lub .djot albo napisz coś i wybierz Dokument ▸ Zapisz jako szablon. Szablony są przechowywane w projekcie, więc współautor, który go otworzy, ma ten sam zestaw.
concept-text-replacement = Twoje własne skróty, rozwijane automatycznie podczas pisania.
concept-text-replacement-more = Ustaw skrót i jego rozwinięcie, np. „pzdr” na „pozdrawiam”, a rozwinie się w chwili wpisania po nim spacji lub znaku interpunkcyjnego. Wielkość liter idzie za tym, co wpisano: napisz skrót wielką literą, a rozwinięcie też zacznie się wielką. Backspace tuż po rozwinięciu cofa tylko tę jedną zamianę. Włączane dla każdego projektu osobno, niezależne od [inteligentnej interpunkcji](:concept-smart-punctuation).
concept-smart-punctuation = Typografia zgodna z językiem, stosowana automatycznie podczas pisania: cudzysłowy, myślniki, wielokropki.
concept-smart-punctuation-more = Zamienia proste cudzysłowy na drukarskie, -- na półpauzę, --- na pauzę, ... na wielokropek, a tam, gdzie to włączysz, zaczyna akapit wpisany jako „- ” od myślnika dialogowego. Ustawiana dla każdego projektu i zapisywana w pliku .skrib, więc współautor pisze według tych samych zasad. To coś innego niż [Twoje własne reguły](:concept-text-replacement), które piszesz sam.
concept-export-style = Nazwany zestaw ustawień eksportu: nagłówki, odstępy, co zostaje dołączone.
concept-export-style-more = Jeden styl, używany w każdym eksporcie, niezależnie od wybranego formatu. Style wbudowane są tylko do odczytu; zduplikuj jeden, aby mieć punkt wyjścia, który możesz zmieniać. Wśród ustawień jest dołączanie [znaczników ponownego importu](:concept-round-trip-marks) dla pliku, który ma trafić do redaktora i wrócić.
concept-round-trip-marks = Niewidoczne zakładki, dzięki którym zredagowany plik DOCX lub ODT po powrocie zostaje rozpoznany.
concept-round-trip-marks-more = Zapisywane tylko w dwóch formatach, które redaktor może odesłać: DOCX i ODT; żaden inny eksport ich nie zawiera. To zakładka, a nie własny atrybut: LibreOffice usuwa własne atrybuty przy zapisie, ale zakładkę pozostawia. Włączane w [stylu eksportu](:concept-export-style), w sekcji „Wysyłanie do redaktora”.
concept-status = Na jakim etapie Twojej pracy jest element: jeden etap naraz, zawsze do przodu.
concept-status-more = To nie [znacznik](:concept-tag): element ma dokładnie jeden status, etapy są uporządkowane, a lista należy do projektu, więc w każdej chwili możesz zmieniać nazwy, kolejność i dodawać nowe. Gwiazdka przy rozdziale lub książce oznacza, że nie wszystkie części poniżej mają ten sam status. Ustawisz go w inspektorze, na karcie, w elemencie strumienia lub tutaj.
