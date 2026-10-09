# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

menu-work = &Projekt
menu-new-work = Ú&j projekt
menu-open-work = Projekt me&gnyitása…
menu-new-window = Új &ablak
menu-import-from = &Importálás forrásból
menu-create-from = &Létrehozás forrásból
menu-import-plume = &Plume Creator (.plume)…
menu-import-document = &Dokumentumok (Markdown, Word, ODT)…
menu-import-manuskript = &Manuskript (.msk)…
menu-export = &Exportálás
menu-export-book = Könyv exportálása
menu-export-part = Rész exportálása
menu-export-chapter = Fejezet exportálása
menu-export-scene = Jelenet exportálása
menu-export-note = Jegyzet exportálása
menu-export-paratext = Paratextus exportálása
menu-export-folder = Mappa exportálása
menu-export-choose = Kiválasztás…
menu-export-none = Nyisson meg egy dokumentumot az exportáláshoz
menu-save = &Mentés
menu-save-as-file = Mentés egyetlen &fájlként…
menu-save-as-folder = Mentés ma&ppaként…
menu-backup = &Biztonsági másolat készítése most
menu-close-work = Projekt be&zárása
menu-welcome = Ü&dvözlőképernyő…
menu-settings = Beállítá&sok
menu-quit = &Kilépés
menu-view = &Nézet
menu-outline = &Vázlat
menu-search = &Keresés a projektben
menu-timeline = Vissza az &időben
menu-search-preview = Keresési &előnézet
menu-fullscreen = &Teljes képernyő
menu-focus-mode = &Zavarmentes mód
menu-format = &Formátum
menu-scene-break = &Jelenettörés beszúrása
menu-major-scene-break = &Nagy jelenettörés beszúrása
menu-go = &Ugrás
menu-go-next-scene = Következő &jelenet
menu-go-prev-scene = &Előző jelenet
menu-go-next-chapter = Következő &fejezet
menu-go-prev-chapter = Előző feje&zet
menu-go-next-note = Következő jegyze&t
menu-go-prev-note = Előző je&gyzet
menu-tools = &Eszközök
menu-help = &Súgó
ctx-add = &Hozzáadás
ctx-new-item = Új &elem
ctx-new-folder = Új &mappa
ctx-rename = Át&nevezés
ctx-duplicate = &Kettőzés
ctx-import-here = &Importálás ide…
ctx-indent = &Behúzás növelése
ctx-outdent = Behúzás &csökkentése
ctx-trash = Á&thelyezés a kukába
ctx-open-to-side = Megnyitás &oldalt
ctx-open = &Megnyitás
ctx-reveal-in-outline = Megjelenítés a &vázlatban
ctx-move-up = Mozgatás &fel
ctx-move-down = Mozgatás &le
ctx-tab-close = &Bezárás
ctx-tab-close-others = &Többi bezárása
ctx-tab-close-all = Ö&sszes bezárása
ctx-tab-open-to-side = Megnyitás &oldalt
ctx-tab-open-in-main = Megnyitás a &fő panelen
ctx-tab-move-to-side = Át&helyezés oldalra
ctx-tab-move-to-main = Át&helyezés a fő panelre
ctx-tab-move-to-new-window = Áthelyezés ú&j ablakba
ctx-tab-move-window-unsaved = Előbb mentse a projektet: a második ablak egy lemezen lévő fájlt nyit meg
ctx-tab-pin = Lap &rögzítése
ctx-tab-unpin = Lap rögzítésének f&eloldása
tab-pinned-tooltip = Rögzítve: a „Többi bezárása” és az „Összes bezárása” nyitva hagyja
create-book = Könyv
create-part = Rész
create-chapter = Fejezet
create-scene = Jelenet
create-note = Jegyzet
create-note-folder = Jegyzetmappa
create-folder = Mappa
create-paratext = Paratextus
create-paratext-folder = Paratextusmappa
create-book-end = Könyv vége
create-story-bible-entry = Történetbiblia-bejegyzés…
type-book-start = Könyv eleje
type-text = Szöveg
new-item-book = Új könyv
new-item-part = Új rész
new-item-chapter = Új fejezet
new-item-note = Új jegyzet
new-item-note-folder = Új jegyzetmappa
new-item-folder = Új mappa
new-item-paratext = Új paratextus
new-item-paratext-folder = Új paratextusmappa
new-item-story-bible-entry = Új történetbiblia-bejegyzés
placement-inside = belül
placement-after = utána
placement-after-parent = a szülő után
placement-top-level = legfelső szint
ctx-promote = Át&alakítás
promote-chapter-folder = Fejezetmappa
promote-flat-chapter = Egyszerű fejezet
promote-lossy-title = Nincs hová menteni a szöveget
promote-lossy-text = A(z) { $target } típus nem tudja tárolni ezeket: { $kinds }. Előbb helyezze át vagy törölje ezt a szöveget, majd alakítsa át.
content-scene-text = Jelenetszöveg
content-note-text = Jegyzetszöveg
content-book-title = Könyvcím
content-book-subtitle = Könyv alcíme
content-part-title = Részcím
content-chapter-title = Fejezetcím
content-epigraph-text = Mottó
content-paratext-text = Paratextus
promote-blocked-title = A fejezet nem üres
promote-blocked-text = { $count ->
    [one] Ez a fejezet még { $count } elemet tartalmaz. Helyezze át vagy dobja a kukába, mielőtt egyszerű fejezetté alakítja.
   *[other] Ez a fejezet még { $count } elemet tartalmaz. Helyezze át vagy dobja őket a kukába, mielőtt egyszerű fejezetté alakítja.
}
inspector = Tulajdonságok
inspector-empty = Nyisson meg egy elemet a tulajdonságai megtekintéséhez.
inspector-promote = Átalakítás…
statusbar-toggle-outline = Iratrendező be- és kikapcsolása
statusbar-toggle-inspector = Tulajdonságok panel be- és kikapcsolása
statusbar-save-unsaved = Nem mentett módosítások. Kattintson a mentéshez
statusbar-save-saved = Minden módosítás mentve
statusbar-save-autosave = Az automatikus mentés be van kapcsolva. A módosítások írás közben mentődnek
statusbar-saving = Mentés…
statusbar-word-count = { $count ->
    [one] { $count } szó
   *[other] { $count } szó
}
statusbar-word-count-tooltip = A szerkesztett jelenet szavai
statusbar-word-char-count = { $words ->
    [one] { $words } szó
   *[other] { $words } szó
} · { $chars ->
    [one] { $chars } karakter
   *[other] { $chars } karakter
}
session-toggle = Írási munkamenet: összpontosított sprint indítása vagy szüneteltetése
session-configure = A munkamenet szócéljának és időkorlátjának beállítása
session-configure-title = Írási munkamenet
session-word-goal = Szócél
session-time-limit = Időkorlát
session-time-limit-unit = perc
session-no-goal = Nincs cél
session-no-limit = Nincs korlát
session-reset = Munkamenet nullázása
session-readout = { $words ->
    [one] { $words } szó
   *[other] { $words } szó
} · { $time }
session-readout-timed = { $words ->
    [one] { $words } szó
   *[other] { $words } szó
} · { $time } van hátra
statusbar-focus-exit = Kilépés a zavarmentes módból
statusbar-focus-synopsis = Szinopszis
statusbar-focus-go-prev = Előző elem (Alt+Fel)
statusbar-focus-go-next = Következő elem (Alt+Le)
rail-settings = Beállítások
language = Nyelv
theme = Téma
english = Angol
french = Francia
light = Világos
dark = Sötét
settings-text-width = Szövegszélesség
settings-preview-width = Keresési előnézet szélessége
settings-autosave = Automatikus mentés lemezre
settings-show-welcome = Az indítóablak megjelenítése induláskor
settings-show-welcome-tip = Kikapcsolva helyette az utoljára használt projekt nyílik meg.
settings-check-for-updates = Új verziók keresése
settings-check-for-updates-tip = A Skribisto legfeljebb naponta egyszer megkérdezi a projekt weboldalától, melyik az aktuális verzió. A kérés nem tartalmaz azonosítót, még a futó verzió számát sem: az összehasonlítás ezen a számítógépen történik.
settings-title = Beállítások
settings-close = Bezárás
settings-search = Keresés a beállításokban
settings-reset = Alapértelmezések visszaállítása
settings-done = Kész
settings-cancel = Mégse
settings-apply = Alkalmaz
settings-ok = OK
settings-reset-confirm-title = Visszaállítja az összes beállítást az alapértelmezésre?
settings-reset-confirm-body = Visszaállítja az alkalmazás megjelenési, szerkesztő- és írásbeállításait. A projekt saját beállításait, a gyorsbillentyűket és a mentett stílusokat ez nem érinti. A művelet nem vonható vissza.
settings-empty-title = Itt még nincsenek beállítások
settings-empty-hint = Ez a szakasz egy későbbi frissítésben kap beállításokat.
settings-sec-appearance-behaviour = Megjelenés és működés
settings-sec-editor = Szerkesztő
settings-sec-spelling = Helyesírás
settings-sec-backup = Biztonsági másolat és szinkronizálás
settings-sec-compile = Összeállítás és exportálás
settings-page-appearance = Megjelenés
settings-field-image-size-policy = Nagy képek
settings-image-policy-ask = Rákérdezés minden alkalommal
settings-image-policy-keep = Eredeti megtartása
settings-image-policy-downscale = Optimalizálás
settings-hint-image-size-policy = Mi történjen, ha egy beszúrt kép nagyobb, mint amekkora a kézirathoz szükséges.
settings-page-notifications = Értesítések
settings-page-scene = Jelenet
settings-page-synopsis = Szinopszis
settings-page-notes = Jegyzetek
settings-page-editor-behavior = Szerkesztő működése
settings-page-goals = Célok és szószám
settings-page-games = Írójátékok
settings-page-corkboard = Parafatábla
settings-page-distraction-free = Zavarmentes mód
settings-page-dictionaries = Szótárak
settings-page-autosave = Automatikus mentés
settings-page-export = Exportformátumok
settings-page-paratext = Paratextus-szerkezetek
settings-page-keymap = Billentyűkiosztás
settings-keymap-filter = Gyorsbillentyűk szűrése
settings-desc-sec-appearance-behaviour = Hogyan néz ki maga az alkalmazás, és mit tesz induláskor.
settings-desc-sec-editor = Az íráshoz használt felület: hogyan néz ki, és mit tesz gépelés közben.
settings-desc-sec-spelling = Helyesírás-ellenőrzés és a mögötte álló szótárak.
settings-desc-sec-backup = Hogyan kerül a munkája a lemezre, és milyen másolatok készülnek.
settings-desc-sec-compile = Mi hagyja el a Skribistót, és milyen formában.
settings-desc-sec-work = A projekthez tartozó beállítások, amelyek a projekt fájljában tárolódnak.
settings-desc-sec-extensions = Az itt telepített bővítmények által hozzáadott oldalak.
settings-desc-group-typography = Szövegfajtánként egy oldal: betűtípus, méret, sormagasság és térközök.
settings-desc-appearance = A felület nyelve, a téma, a szövegméret és az indítóablak induláskor.
settings-desc-notifications = A munkamenet során megjelent összes üzenet, és az újra végrehajtható műveletek.
settings-desc-scene = A jelenetek prózájának szedése: betűtípus, méret, sormagasság, behúzások és térközök.
settings-desc-synopsis = A szinopszispanel szedése, a kézirattól függetlenül.
settings-desc-notes = A jegyzetek szedése, a kézirattól függetlenül.
settings-desc-corkboard = A kártyák mérete, tartalma és elrendezése a táblán.
settings-desc-distraction-free = Tipográfia, hasábszélesség és vezérlősáv a teljes képernyős íráshoz.
settings-desc-distraction-free-themes = A teljes képernyős írás tématára: a beépített és a saját témák.
settings-desc-editor-behavior = Szövegszélesség, a szinopszis helye, írógépszerű görgetés és a kurzor körüli kiemelés.
settings-desc-punctuation = A tipográfiai alapstílus, amellyel az új projektek indulnak. Minden projekt ezt követi, hacsak nem állít be sajátot.
settings-desc-goals = Szó- és karaktercélok, és a szavak számlálásának módja.
settings-desc-games = Az írás közben önmagának szabott korlátok, például a Mindig előre.
settings-desc-spellcheck = Az egyetlen kapcsoló, amely mindenhol be- és kikapcsolja a helyesírás-ellenőrzést.
settings-desc-dictionaries = A számítógépen lévő helyesírási szótárak telepítése, eltávolítása és böngészése.
settings-desc-autosave = A módosítások maguktól kerüljenek-e a lemezre.
settings-desc-backup = Mikor készülnek másolatok, hol tárolódnak, és hány marad meg. Minden projekt ezt követi, hacsak nem állít be sajátot.
settings-desc-export = Stílusok, amelyeken minden exportálás átmegy: a beépítettek és a sajátjai.
settings-desc-paratext = Az elő- és utórészek, amelyekkel egy új projekt indulhat.
settings-desc-keymap = Minden gyorsbillentyű, és hogy mihez van rendelve.
settings-page-user = Felhasználó
settings-desc-user = Kicsoda Ön: ez a név szerepel a megjegyzésein.
settings-group-identity = Azonosság
settings-field-user-name = Az Ön neve
settings-field-user-name-placeholder = Nem kötelező
settings-field-user-name-hint = Ezzel írja alá a megjegyzéseit és a válaszait. Nem azonos a projekt szerzőnevével, amely a könyvben szerzőként szerepel és a fájlban tárolódik.
settings-field-user-initials = Az Ön monogramja
settings-field-user-initials-placeholder = A nevéből
settings-field-user-initials-hint = Ezt mutatja a szövegszerkesztő a megjegyzése mellett a margón. Hagyja üresen, ha a nevéből képzett, itt látható monogramot szeretné használni.
settings-field-user-hint = Mindkettő nem kötelező, és a számítógép minden projektjére vonatkozik. Módosításuk az ezután írt megjegyzések aláírását változtatja meg. A már megírt megjegyzések megtartják a nevet, amellyel készültek.
settings-desc-author = A projekten szereplő név.
settings-desc-structure = A projekt fejezetei mappák vagy önálló elemek.
settings-desc-language = A nyelv, amely szerint a projekt prózáját ellenőrzi a program.
settings-desc-work-backup = A projekt saját biztonsági másolat-szabályai, vagy az általános szabályok.
settings-desc-personal-dictionary = A szavak, amelyeket ez a projekt helyesen írottnak tekint.
settings-desc-tags = A színkódos címkék, amelyekkel a projekt elemeit jelöli.
settings-desc-templates = A jegyzetsablonok, amelyeket a projektben írás közben beszúrhat.
settings-desc-text-replacements = Rövidítések, amelyek gépelés közben kibővülnek ebben a projektben.
settings-desc-work-punctuation = Az idézőjelek, gondolatjelek és térközök, amelyeket a projekt prózája követ.
settings-group-typography = Tipográfia
settings-group-writing-column = Írási hasáb
settings-group-writing-view = Írási nézet
settings-group-distraction-free-strip = Vezérlősáv
settings-group-theme = Téma
settings-group-language = Nyelv
settings-group-startup = Indítás
settings-group-updates = Frissítések
settings-group-autosave = Automatikus mentés
settings-field-typeface = Betűcsalád
settings-field-size = Szövegméret
editor-size-changed-manuscript = A kézirat szövegmérete: { $percent }
editor-size-changed-synopsis = A szinopszis szövegmérete: { $percent }
editor-size-changed-notes = A jegyzetek szövegmérete: { $percent }
editor-size-changed-corkboard = A parafatábla kártyáinak szövegmérete: { $percent }
editor-size-changed-corkboard-expanded = A kinagyított szerkesztő szövegmérete: { $percent }
editor-size-changed-distraction-free = A zavarmentes mód szövegmérete: { $percent }
menu-text-size-increase = Szövegméret &növelése
menu-text-size-decrease = Szövegméret &csökkentése
menu-text-size-reset = Szövegméret &alaphelyzetbe
settings-field-line-height = Sormagasság
settings-field-first-line-indent = Első sor behúzása
settings-field-paragraph-spacing-before = Térköz a bekezdés előtt
settings-field-paragraph-spacing-after = Térköz a bekezdés után
settings-field-column-width = Hasábszélesség
settings-distraction-free-width-hint = Csak zavarmentes módban érvényes. A Jelenet, a Szinopszis és a Jegyzetek hasábszélessége nem változik.
settings-distraction-free-title = Az elem nevének megtartása
settings-distraction-free-word-count = A szószám megtartása
settings-distraction-free-session = Az írási munkamenet megtartása
settings-distraction-free-go-to = Az Ugrás… gomb megtartása
settings-distraction-free-go = Az Előző és a Következő gomb megtartása
settings-distraction-free-chrome-hint = A Kilépés gomb mindig megmarad, bármit választ is itt. Ezzel léphet ki, ha az Escape billentyű foglalt.
settings-field-app-theme = Téma
settings-theme-light = Világos
settings-theme-dark = Sötét
settings-field-text-scale = A felület szövegmérete
settings-field-language = A felület nyelve
settings-synopsis-placement = A szinopszis helye
settings-synopsis-placement-none = Nincs
settings-synopsis-placement-top = Fölötte
settings-synopsis-placement-side = Mellette
settings-typewriter = Írógépszerű görgetés
settings-typewriter-tip = Az éppen írt sort rögzített magasságban tartja, miközben alatta a kézirat gördül. Kattintással továbbra is oda kerül a kurzor, ahová kattint.
settings-typewriter-position = A sor helyzete
settings-typewriter-position-top-third = Felső harmad
settings-typewriter-position-middle = Közép
settings-typewriter-position-bottom-quarter = Alsó negyed
settings-highlight-scope = Kiemelés a kurzor körül
settings-highlight-scope-none = Nincs
settings-highlight-scope-sentence = Mondat
settings-highlight-scope-paragraph = Bekezdés
settings-highlight-scope-tip-none = Az oldal egyszerű marad. Írás közben semmi sincs árnyékolva.
settings-highlight-scope-tip-sentence = Az éppen írt mondat árnyékolása, hogy kiemelkedjen a környezetéből.
settings-highlight-scope-tip-paragraph = Az éppen írt teljes bekezdés árnyékolása, hogy a szövegrész, amelyen dolgozik, szem előtt maradjon.
settings-group-container-views = Tárolónézetek
settings-remember-view = Az utolsó nézet megjegyzése elemtípusonként
settings-remember-view-tip = A tárolók azon a nézeten nyílnak meg, amelyet az adott típusnál utoljára használt
settings-remember-view-tip-more =
    A Könyv, a Rész és a Fejezet több nézetet kínál (a saját oldalát, a teljes
    kéziratot, a teljes szinopszist). Ha bekapcsolja, minden típus azon a nézeten
    nyílik meg újra, amelyet legutóbb választott hozzá: ha például egy fejezetet a teljes fejezet
    nézetre vált, a következő megnyitott fejezet is a teljes fejezetet mutatja. Minden típus a saját nézetét jegyzi meg.
settings-group-counting = Szavak számlálása
settings-counting-auto = Automatikus (nyelv szerint)
settings-counting-whitespace = Felosztás szóközöknél
settings-counting-unicode-words = Unicode-szavak
settings-counting-cjk-hybrid = CJK-érzékeny (karakterenként)
settings-counting-hint = Az automatikus mód a kínait és a japánt karakterenként, minden más nyelvet szavanként számolja. Csak akkor módosítsa, ha a nyelvén hibásnak tűnik a szám.
settings-group-goals-display = Megjelenítés
settings-show-characters = A karakterszám megjelenítése az állapotsoron
settings-autosave-hint = A módosítások írás közben automatikusan a lemezre kerülnek.
settings-sec-work = Projekt
settings-page-structure = Szerkezet
settings-page-author = Szerző
settings-field-author-name = Szerző neve
settings-field-author-placeholder = Nem kötelező
settings-field-author-hint = Megjelenik az összeállított címoldalon és az exportált fájlok metaadataiban. Hagyja üresen, ha nem szeretné feltüntetni. Ez a könyv szerzőjeként szerepel. A név, amellyel a megjegyzéseit aláírja, a Beállítások ▸ Felhasználó oldalon található.
settings-group-chapters = Fejezetek
tidy-titles-title = Fejezetcímek rendezése
tidy-titles-none = Egyetlen fejezet vagy rész címe sem ismétli pusztán a saját számát.
tidy-titles-lead = { $count ->
        [one] Egy fejezet vagy rész címe nem más, mint a saját száma.
       *[other] { $count } fejezet és rész címe nem más, mint a saját száma.
    }
tidy-titles-explain = Ha törli ezeket a címeket, mindegyiket az a szám nevezi meg, amelyet a könyv már ismer, ugyanaz, amelyet az exportálás is kiír. Semmi más nem változik, és egyetlen visszavonás mindet visszaállítja.
menu-document-tidy-titles = Fejezetcímek &rendezése…
settings-group-numbering = Számozás
settings-number-chapters = Fejezetek és részek számozása
settings-number-chapters-tip = A fejezetek és részek a könyvben elfoglalt helyükből kiszámított számot kapnak, amely itt a címük mellett látszik, és az exportálás is kiírja.
settings-number-chapters-tip-more = A szám sosem kerül a címbe, így átrendezés, beszúrás vagy törlés után is helyes marad. Ha kikapcsolja, az exportálás csak a címeket írja ki, bármit kér is az exportstílus. Ha egyetlen fejezetet szeretne kihagyni (prológust, közjátékot), használja inkább a Tulajdonságok panel Számozás kapcsolóját: így a fejezet a könyvben marad, de nem kap számot.
settings-part-resets-chapter = Fejezetszámozás újrakezdése minden résznél
settings-part-resets-chapter-tip = Alapértelmezés szerint ki van kapcsolva: a fejezetek a könyv részein át folyamatosan számozódnak, így a „Második rész” a tizenegyedik fejezettel kezdődik.
settings-part-resets-chapter-tip-more = Ez a kiadói gyakorlat, és az olvasó is ezt várja. Kapcsolja be olyan könyvnél, amelynek részei külön kötetként olvasandók, és mindegyik az első fejezettel kezdődik.
settings-chapter-flat = Egyszerű fejezetek
settings-chapter-flat-hint = Bekapcsolva: a fejezet egyetlen elem. Közvetlenül bele ír, és nem tartalmaz jeleneteket. Kikapcsolva: a fejezet mappa. Továbbra is írhat bele, de jeleneteket is tartalmazhat. Az új fejezetek ezt a beállítást követik; a meglévők az Átalakítás paranccsal alakíthatók át.
settings-styles-builtin = Beépített stílusok
settings-styles-user = Saját stílusok
settings-styles-builtin-badge = Beépített
settings-styles-duplicate = Kettőzés
settings-styles-edit = Szerkesztés
settings-styles-delete = Törlés
settings-styles-import = Importálás…
settings-styles-export = Exportálás…
settings-styles-copy-suffix = (másolat)
settings-styles-json-filter = Exportstílus
settings-styles-editor-title = Stílus szerkesztése
settings-styles-editor-none = Válasszon ki egy saját stílust a szerkesztéséhez, vagy kettőzzön meg egy beépítettet.
settings-styles-imported = A stílus importálva
settings-styles-import-failed = Nem sikerült importálni a stílust
settings-styles-exported = A stílus exportálva
settings-styles-export-failed = Nem sikerült exportálni a stílust
settings-styles-field-name = Név
settings-styles-field-chapters = Fejezetcímek
settings-styles-field-parts = Részcímek
settings-styles-field-scene-break = Jelenettörés
settings-styles-field-major-scene-break = Nagy jelenettörés
settings-styles-field-spacing = Sorköz
settings-styles-field-justify = Sorkizárt szöveg
settings-styles-field-notes = Jegyzetek belefoglalása
settings-styles-field-synopses = Szinopszisok belefoglalása
settings-styles-field-scene-titles = Jelenetcímek belefoglalása
settings-styles-field-epigraphs = Mottók belefoglalása
settings-styles-field-paratexts = Paratextusok belefoglalása
settings-styles-field-epigraph-placement = A mottó helye
settings-styles-epigraph-after = A cím után
settings-styles-epigraph-before = A cím előtt
settings-styles-field-footnotes = Lábjegyzetek belefoglalása
settings-styles-field-footnote-numbering = Lábjegyzetek számozása
settings-styles-footnote-numbering-continuous = Folyamatos
settings-styles-footnote-numbering-per-chapter = Újrakezdés fejezetenként
settings-styles-footnote-numbering-per-book = Újrakezdés könyvenként
settings-styles-field-images = Képek
menu-image = &Kép
settings-styles-images-beside = A dokumentum mellett
settings-styles-images-embed = A dokumentumba ágyazva
settings-styles-images-omit = Kihagyás
settings-styles-group-round-trip = Küldés szerkesztőnek
settings-styles-field-comments = Megjegyzések belefoglalása
settings-styles-field-round-trip-marks = Visszaküldési jelölők belefoglalása
settings-styles-round-trip-hint = Csak DOCX és ODT esetén. A jelölők láthatatlan azonosítók, amelyek révén a visszaérkező fájl ezt a projektet frissíti ahelyett, hogy második példányként mellé kerülne.
settings-styles-group-pages = Oldalak
settings-styles-field-cover = Kezdés a borítóval
settings-styles-field-word-count = Szószám a címoldalon
settings-styles-field-page-books = Új oldal minden könyvnél
settings-styles-field-page-parts = Új oldal minden résznél
settings-styles-field-page-chapters = Új oldal minden fejezetnél
settings-styles-field-page-paratexts = Új oldal minden paratextusnál
settings-styles-heading-none = Nincs címsor
settings-styles-heading-numbered = Csak szám
settings-styles-heading-title = Csak cím
settings-styles-heading-both = Szám + cím
settings-styles-break-blank = Üres sor
settings-styles-break-none = Nincs
settings-styles-spacing-single = Szimpla
settings-styles-spacing-onehalf = 1½
settings-styles-spacing-double = Dupla
welcome-title = Üdvözli a Skribisto
welcome-version = { $version } verzió
welcome-update-available = Elérhető a(z) { $version } verzió
welcome-update-tip = Megnyitja a letöltési oldalt a böngészőben.
welcome-search = Keresés a projektek között
welcome-open = Megnyitás
welcome-new-work = Új projekt
welcome-create-from = Létrehozás forrásból…
welcome-recent-works = Legutóbbi projektek
welcome-empty-recents = Még nincsenek legutóbbi projektek.
welcome-no-matches = Egyetlen legutóbbi projekt sem felel meg a keresésnek.
welcome-learn-soon = Hamarosan útmutatók és tippek érkeznek.
welcome-about-blurb = Skribisto, az írói alkalmazás Rust + Teksilo alapú újraírása.
welcome-tagline = *Csendes hely a hosszú szövegek írásához.*
welcome-github = GitHub
welcome-discord = Discord
nav-works = Projektek
nav-examples = Példák
nav-learn = Tanulás
nav-about = Névjegy
synopsis = Szinopszis
epigraph = Mottó
pane-manuscript = Kézirat
corkboard = Parafatábla
overview = Áttekintés
corkboard-card-count = { $count ->
    [one] { $count } kártya
   *[other] { $count } kártya
}
corkboard-child-count = { $count ->
    [one] { $count } elem
   *[other] { $count } elem
}
corkboard-view-nested = Egymásba ágyazott
corkboard-view-flat = Egyszintű
corkboard-layout-hint = Az egymásba ágyazott nézet a tároló közvetlen gyermekelemeit mutatja; egy mappakártyát megnyitva beléphet. Az egyszintű nézet a teljes tároló összes jelenetét egyszerre mutatja.
corkboard-card-size = Kártyaméret
corkboard-search-placeholder = Kártyák szűrése…
corkboard-empty-title = Itt még nincs semmi
corkboard-empty-hint = Az első elemet a fenti „＋ Új” gombbal adhatja hozzá.
corkboard-new = Új
corkboard-show-card-numbers = Kártyák számozása
corkboard-modal-size = A kinagyított szerkesztő mérete
corkboard-scope-hint = Minden megnyitott parafatáblára érvényes, minden projektben.
corkboard-sort-manuscript = Kéziratbeli sorrend
corkboard-sort-title-asc = Cím szerint A–Z
corkboard-sort-title-desc = Cím szerint Z–A
corkboard-card-number = { $number }. kártya
duplicate = Kettőzés
duplicate-n = { $count ->
    [one] { $count } kártya kettőzése
   *[other] { $count } kártya kettőzése
}
set-label-n = { $count ->
    [one] Felirat beállítása { $count } kártyán
   *[other] Felirat beállítása { $count } kártyán
}
move-to-trash-n = { $count ->
    [one] { $count } kártya áthelyezése a kukába
   *[other] { $count } kártya áthelyezése a kukába
}
reveal-in-outline = Megjelenítés a vázlatban
corkboard-move-to = Áthelyezés ide…
corkboard-move-to-n = { $count ->
    [one] { $count } kártya áthelyezése ide…
   *[other] { $count } kártya áthelyezése ide…
}
corkboard-move-picker-title = Áthelyezés ide…
corkboard-move-picker-empty = Még nincs iratrendező. Előbb hozzon létre egyet.
corkboard-move-picker-cancel = Mégse
corkboard-move-here = Áthelyezés ide
corkboard-moved-ok = { $count ->
    [one] { $count } kártya áthelyezve
   *[other] { $count } kártya áthelyezve
}
corkboard-move-into-self = Egy tároló nem helyezhető át önmagába. Válasszon rajta kívül eső célt.
corkboard-move-failed = Ezeket a kártyákat nem sikerült oda áthelyezni.
overview-col-title = Cím
overview-col-type = Típus
overview-col-label = Felirat
overview-col-tags = Címkék
overview-col-books = Könyvek
overview-col-own-words = Szavak
overview-col-total-words = Összesen
overview-row-count = { $count ->
    [one] { $count } elem
   *[other] { $count } elem
}
overview-search-placeholder = Elemek szűrése…
overview-expand-all = Összes kibontása
overview-collapse-all = Összes összecsukása
overview-table-label = Tartalom
overview-empty-title = Itt még nincs semmi
overview-gone-title = Ez a tároló már nem létezik
overview-gone-hint = A kukába került. Állítsa vissza, vagy zárja be ezt a lapot.
overview-empty-hint = Az első elemet a fenti „＋ Új” gombbal adhatja hozzá.
corkboard-grid-label = A parafatábla kártyái
corkboard-rename-field = Elem átnevezése
corkboard-expand-synopsis = Szinopszis kinagyítása
corkboard-synopsis-modal-title = Szinopszis
corkboard-badge-scene = Jelenet
corkboard-badge-chapter = Fejezet
corkboard-badge-part = Rész
corkboard-badge-book = Könyv
corkboard-badge-note = Jegyzet
corkboard-badge-folder = Mappa
corkboard-badge-paratext = Paratextus
corkboard-badge-text = Szöveg
corkboard-badge-end = Vége
settings-group-corkboard-layout = Elrendezés
settings-group-corkboard-cards = Kártyák
corkboard-show-word-count = Szószám megjelenítése a kártyákon
text-heading = Szöveg
no-content = Ennek az elemnek nincs szerkeszthető tartalma.
untitled = Névtelen
placeholder-title = Cím…
placeholder-subtitle = Alcím…
placeholder-chapter-title = Fejezetcím…
split-editor = Szerkesztő felosztása
close-split-view = Osztott nézet bezárása
drop-open-here = Megnyitás itt
drop-open-to-side = Megnyitás oldalt
segment-chapter = Fejezet
segment-part = Rész
segment-book = Könyv
segment-pace = Tempó
segment-notes = Jegyzetek
segment-story-bible = Történetbiblia
segment-note-own = Jegyzet
segment-note-details = Részletek
segment-note-in-prose = A prózában
pace-placeholder = Itt jelenik meg a tempótervező.
pace-empty-title = Tervezze meg a könyv tempóját
pace-empty-body = Adjon meg egy szócélt és egy határidőt, és a Skribisto kiszámítja a napi tempót, amellyel elérheti.
pace-start-planning = Tervezés indítása
pace-section-schedule = Ütemezés
pace-section-progress = Haladás
pace-advancement = Előrehaladás
pace-goal = Szócél
pace-deadline = Határidő
pace-active = Tempó aktív
pace-day-mon = H
pace-day-tue = K
pace-day-wed = Sze
pace-day-thu = Cs
pace-day-fri = P
pace-day-sat = Szo
pace-day-sun = V
pace-card-written = megírt szó
pace-card-of-goal = a célból
pace-card-rate = szó / írási nap
pace-card-days-left = hátralévő írási nap
pace-card-streak = napos sorozat
pace-card-ahead = szóval a terv előtt
pace-card-behind = szóval a terv mögött
pace-charts-empty = A haladási diagramok itt jelennek meg, amint egy mentés rögzíti a szószámot.
pace-chart-progression = Megírt szavak a célhoz képest
pace-chart-words-per-day = Szavak naponta
pace-series-actual = Tényleges
pace-series-target = Célérték
pace-series-words-per-day = Szó/nap
pace-daily-target-line = Egyenletes tempó: { $count ->
    [one] { $count } szó/nap
   *[other] { $count } szó/nap
}
pace-section-holidays = Szünnapok
pace-section-milestones = Mérföldkövek
pace-holidays-none = Nincsenek szünnapok. Minden ütemezett nap számít.
pace-holiday-label = A szünnap neve
pace-add-holiday = Hozzáadás
pace-remove = Eltávolítás
pace-milestones-none = Még nincsenek mérföldkövek. Egy részhez vagy fejezethez a Tulajdonságok panelen állíthat be egyet.
full-chapter = Teljes fejezet
full-part = Teljes rész
full-book = Teljes könyv
full-synopsis = Teljes szinopszis
rename = Átnevezés
set-label = Felirat beállítása
insert-scene = Jelenet beszúrása
insert-chapter = Fejezet beszúrása
split-scene = Jelenet kettéosztása
move-up = Mozgatás fel
move-down = Mozgatás le
merge-with-previous = Egyesítés az előzővel
move-to-trash = Áthelyezés a kukába
rename-chapter = Fejezet átnevezése
add-scene = Új jelenet hozzáadása
add-chapter = Új fejezet hozzáadása
new-scene-title = Új jelenet
placeholder-scene-name = Jelenet neve…
menu-cut = Kivágás
menu-copy = Másolás
menu-paste = Beillesztés
menu-paste-unformatted = Beillesztés formázatlanul
menu-select-all = Összes kijelölése
format-bold = Félkövér
format-italic = Dőlt
format-underline = Aláhúzott
format-strikethrough = Áthúzott
binder = Iratrendező
no-work = Nincs betöltött projekt
no-recent-works = Nincsenek legutóbbi projektek
switcher-open-section = Jelenleg megnyitva
switcher-recent-section = Legutóbbiak
switcher-this-window = ez az ablak
open-project-title = Projekt megnyitása
open-project-question = Hogyan szeretné megnyitni ezt: „{ $title }”?
open-in-new-window = Megnyitás új ablakban
open-here = Megnyitás itt
binder-all = Minden iratrendező
binder-show-all = Minden iratrendező megjelenítése
binder-new = Új iratrendező…
binder-default-name = Új iratrendező
binder-item-count = { $count ->
    [one] { $count } elem
   *[other] { $count } elem
}
binder-search-placeholder = A vázlat szűrése…
binder-search-scope = Keresés minden iratrendezőben
binder-trash-confirm-title = Áthelyezi az iratrendezőt a kukába?
binder-trash-confirm-text = A(z) „{ $name }” és minden eleme a kukába kerül.
dialog-rename = Átnevezés
dialog-new-binder = Új iratrendező
dialog-set-label = Felirat beállítása
dialog-new-scene = Új jelenet
close-work-question = Menti a módosításokat a projekt bezárása előtt?
quit-question = Menti a módosításokat kilépés előtt?
quit-save-work-question = Menti a(z) „{ $title }” módosításait kilépés előtt?
unsaved-changes = A projektben nem mentett módosítások vannak.
new-work-unsaved-question = Menti a módosításokat új projekt létrehozása előtt?
open-work-unsaved-question = Menti a módosításokat egy másik projekt megnyitása előtt?
switch-backup-discard-title = Elveti a biztonsági másolaton végzett módosításokat?
switch-backup-discard-text = A biztonsági másolaton végzett módosítások nem menthetők bele. A megtartásukhoz használja a Mentés másként vagy a Visszaállítás parancsot, vagy vesse el őket, és nyissa meg a másik projektet.
switch-save-failed = A projektet nem sikerült menteni, ezért nem lett lecserélve: { $error }
switch-save-not-started = A projektet nem sikerült menteni, ezért nem lett lecserélve.
close-save-failed = A projektet nem sikerült menteni, ezért nem lett bezárva: { $error }
close-save-not-started = A projektet nem sikerült menteni, ezért nem lett bezárva.
save-not-started = A projektet nem sikerült menteni.
could-not-open-work = Nem sikerült megnyitni: „{ $file }”
could-not-open-work-too-new = A(z) „{ $file }” megnyitásához újabb Skribisto kell
could-not-open-work-too-new-detail = A fájlt a Skribisto { $written_by }. formátumban mentette; a megnyitásához legalább { $requires }. formátum szükséges, ez a változat pedig legfeljebb a(z) { $supported }. formátumot támogatja. A megnyitáshoz frissítse a Skribistót.
could-not-open-work-importing = A(z) „{ $file }” fájlt éppen egy importálás írja
could-not-open-work-importing-detail = Egy importálás felülírja ezt a projektet, és még nem fejeződött be. Akkor nyissa meg, ha az importálás véget ért: ha most nyitja meg, azt a projektet mutatná, amelyet az importálás lecserél, és a következő mentés azt írná vissza az importált tartalom fölé.
could-not-open-example = Nem sikerült megnyitni a példát: { $error }
could-not-open-link = Nem sikerült megnyitni: { $url }: { $error }
could-not-create-work = Nem sikerült létrehozni a projektet: { $error }
saving-as-file = Mentés ide: { $target }…
saving-as-folder = Mentés ide: { $target }/…
saved-as = Mentve ide: { $target }
save-error = Nem sikerült menteni: { $error }
save-flush-failed = A mentés megtörtént, de ezeket a dokumentumokat nem sikerült átadni neki, így még nincsenek mentve: { $titles } ({ $error })
backup-error = Nem sikerült biztonsági másolatot készíteni: { $error }
backing-up = Biztonsági másolat készítése…
backup-nothing-open = Nincs megnyitott projekt, amelyről biztonsági másolat készülhetne.
backup-already-running = Már folyamatban van egy biztonsági másolat készítése.
backup-complete = A biztonsági másolat elkészült ({ $ok } mentve, { $skipped } már naprakész)
backup-partial = A biztonsági másolat befejeződött: { $ok } mentve, { $failed ->
    [one] { $failed } cél sikertelen
   *[other] { $failed } cél sikertelen
}
backup-no-destination-title = Nem érhető el a biztonsági másolatok helye
backup-no-destination-text = A beállított biztonsági másolati célok egyike sem érhető el (például egy külső meghajtó nincs csatlakoztatva). Csatlakoztassa, és próbálja újra, vagy lépjen ki biztonsági másolat nélkül.
backup-failed-close-title = Nem sikerült menteni a biztonsági másolatot
backup-failed-close-text = Bezárás előtt nem sikerült biztonsági másolatot írni. Minden cél sikertelen volt (lehet, hogy a meghajtót eltávolították, megtelt vagy írásvédett). Hárítsa el a hibát, és próbálja újra, vagy lépjen ki biztonsági másolat nélkül.
backup-choice-title = Biztonsági másolat fájlja
backup-choice-heading = Egy biztonsági másolatot nyitott meg
backup-choice-subtitle = Ez egy projekt adott időpontban készült biztonsági másolata.
backup-choice-subtitle-dated = A biztonsági másolat időpontja: { $date }.
backup-choice-body = Megnyithatja és szabadon szerkesztheti, de a módosítások csak a Mentés másként paranccsal őrizhetők meg. Az eredeti projektfájl érintetlen marad. Vagy visszaállíthatja a projektet pontosan erre a biztonsági másolatra.
backup-choice-open = A biztonsági másolat megnyitása
backup-choice-restore = A projekt visszaállítása erre az állapotra…
backup-choice-not-a-backup = Nem, megnyitás a szokásos módon
backup-banner-title = Biztonsági másolat: a módosítások itt nem menthetők
backup-banner-description = A Mentés másként paranccsal új fájlban őrizheti meg a módosításait, a Visszaállítással pedig az eredeti projektet cserélheti le erre a biztonsági másolatra.
backup-banner-restore = Visszaállítás…
backup-banner-save-as = Mentés másként…
backup-restore-original-missing = Nem található az eredeti projekt, amelyet vissza kellene állítani. Ehelyett a Mentés másként paranccsal őrizze meg ezt a biztonsági másolatot új projektként.
backup-restore-close-elsewhere-title = A projekt egy másik ablakban meg van nyitva
backup-restore-close-elsewhere-text = A visszaállítandó projekt egy másik ablakban meg van nyitva. Előbb zárja be ott, majd próbálja újra.
backup-restore-focus-window = Ugrás arra az ablakra
backup-restore-confirm-title = Visszaállítja ezt a biztonsági másolatot?
backup-restore-confirm-text = A projekt jelenlegi változatáról előbb biztonsági tartalékmásolat készül, csak utána cseréli le ez a változat.
backup-restore-confirm-ok = Visszaállítás
backup-restore-error = Nem sikerült a visszaállítás: { $error }
backup-restored-ok = A projekt visszaállítva.
backup-restored-with-safety = A projekt visszaállítva. Az előző változat ide került: { $path }.
close-backup-discard-title = Elveti a biztonsági másolaton végzett módosításokat?
close-backup-discard-text = A biztonsági másolaton végzett módosítások nem menthetők bele. A megtartásukhoz használja a Mentés másként parancsot, vagy vesse el őket, és zárja be.
quit-backup-discard-title = Elveti a módosításokat, és kilép?
quit-backup-discard-work-question = Elveti a(z) „{ $title }” módosításait, és kilép?
quit-backup-discard-text = A biztonsági másolaton végzett módosítások nem menthetők bele. A megtartásukhoz használja a Mentés másként parancsot, vagy vesse el őket, és lépjen ki.
backup-nudge-text = Ehhez a projekthez nincs beállítva biztonsági másolat.
backup-nudge-action = Biztonsági másolatok beállítása…
menu-backups-list = Biztonsági másolatok &listája…
backups-title = Biztonsági másolatok
backups-loading = Biztonsági másolatok keresése…
backups-empty = Ehhez a projekthez még nem található biztonsági másolat.
backups-open = Megnyitás
backups-reveal = Megjelenítés a mappában
backups-delete = A biztonsági másolat törlése
backups-delete-confirm-title = Törli ezt a biztonsági másolatot?
backups-delete-confirm-text = A(z) „{ $name }” véglegesen törlődik. A művelet nem vonható vissza.
backups-delete-error = Nem sikerült törölni a biztonsági másolatot: { $error }
backups-refresh = Frissítés
backups-close = Bezárás
settings-page-backup = Biztonsági másolat alapértelmezései
settings-page-work-backup = Biztonsági másolatok
settings-backup-general-title = A biztonsági másolat alapértelmezett beállításai
settings-backup-work-title = A projekt biztonsági másolatai
settings-backup-inherit = Az általános biztonsági másolat-beállítások használata
settings-backup-inheriting = Ez a projekt az általános biztonsági másolat-beállításokat használja.
settings-backup-none-hint = Nincs beállítva automatikus biztonsági másolat (minden kiváltó esemény ki van kapcsolva).
settings-backup-last = Utolsó biztonsági másolat: { $date }
settings-backup-last-never = Még nincs biztonsági másolat.
settings-backup-open-list = A biztonsági másolatok listájának megnyitása…
settings-backup-triggers = Mikor készüljön biztonsági másolat
settings-backup-on-close = Biztonsági másolat a projekt bezárásakor
settings-backup-on-open = Biztonsági másolat a projekt megnyitásakor
settings-backup-interval = Rendszeres biztonsági másolat, ennyi időnként:
settings-backup-destinations = Biztonsági másolatok céljai
settings-backup-default-location = Alapértelmezett hely
settings-backup-usage-since = { $count ->
    [one] { $count } biztonsági másolat · { $size } · a legrégebbi: { $oldest }
   *[other] { $count } biztonsági másolat · { $size } · a legrégebbi: { $oldest }
}
settings-backup-usage = { $count ->
    [one] { $count } biztonsági másolat · { $size }
   *[other] { $count } biztonsági másolat · { $size }
}
settings-backup-usage-empty = Itt még nincs semmi tárolva
settings-backup-usage-measuring = Mérés…
settings-backup-reveal-root = Mappa megjelenítése
settings-backup-dest-none = Nincsenek célok. A biztonsági másolatok a projekt mellé kerülnek.
settings-backup-dest-remove = Eltávolítás
settings-backup-dest-add = Mappa hozzáadása…
settings-backup-dest-refresh = Frissítés
settings-backup-retention = Hány maradjon meg
settings-backup-retention-tiered = Lépcsőzetes
settings-backup-retention-keep-n = Az utolsó N megtartása
settings-backup-retention-tip = Hogyan ritkulnak a régi biztonsági másolatok.
settings-backup-retention-tip-more = A lépcsőzetes mód egy napig óránként, egy hétig naponta, egy hónapig hetente, azon túl havonta egy biztonsági másolatot tart meg, így a közelmúlt sűrűn, a régebbi időszak ritkábban marad meg. Az „Utolsó N megtartása” egyszerűen az N legfrissebb másolatot őrzi meg. Mindkét módban a legújabb másolatok (az alábbi minimum) mindig megmaradnak.
settings-backup-gfs-hourly = Óránként (utolsó 24 óra)
settings-backup-gfs-daily = Naponta (utolsó hét)
settings-backup-gfs-weekly = Hetente (utolsó hónap)
settings-backup-gfs-monthly = Havonta
settings-backup-keep-n = Megtartandó darabszám
settings-backup-min-keep = Legalább ennyi mindig megmarad
settings-backup-dedup = Kihagyás, ha semmi sem változott
new-work-title = Új projekt
new-work-close = Bezárás
new-work-name = A projekt neve
new-work-name-placeholder = Névtelen
new-work-author = Szerző
new-work-author-placeholder = Nem kötelező
new-work-format = Formátum
new-work-single-file = Egyetlen fájl
new-work-bundle = Mappa
new-work-convert-later = A formátumok között később is válthat.
new-work-location = Hely
new-work-will-create = Létrejön
new-work-language = Alapértelmezett nyelv
new-work-language-hint = Az új szövegekre és a helyesírás-ellenőrzésre vonatkozik. Minden szöveg átállítható más nyelvre.
new-work-template = Sablon
new-work-template-none = Nincs
new-work-template-empty-novel = Üres regény
new-work-template-light-novel = Kisregény
new-work-template-novel = Regény
new-work-template-novel-in-parts = Részekre osztott regény
new-work-template-notebook = Jegyzetfüzet
new-work-cancel = Mégse
new-work-create = Projekt létrehozása
new-work-step-details = Adatok
new-work-step-language = Nyelv és szerkezet
new-work-step-template = Sablon
new-work-back = Vissza
new-work-next = Tovább
new-work-documents-title = Új projekt dokumentumokból
new-work-step-import = Importálás
new-work-create-and-import = Létrehozás és importálás…
new-work-documents-next-title = Ezután jönnek a dokumentumai
new-work-documents-next-body = A projekt létrehozása után megnyílik fölötte az importálási varázsló, ahol kiválasztja a dokumentumokat, átnézi a szerkezetet, amelyet a Skribisto kiolvasott belőlük, és megadja, hová kerüljenek. Amíg ott jóvá nem hagyja, semmi sem kerül a projektbe.
new-work-documents-no-template = Ez a projekt szándékosan üresen indul, sablon nélkül, így csak az importált dokumentumok lesznek benne.
new-work-documents-chapter-scene-hint = Az importálás által létrehozott fejezetekre vonatkozik. Később a projekt beállításaiban módosíthatja.
new-work-single-file-desc = Egyetlen .skrib archívum (zip). Hordozható, könnyen menthető.
new-work-bundle-desc = Egy mappa, amely minden szöveget és erőforrást tartalmaz. Verziókezelőkhöz jobban illik.
new-work-template-none-count = üres iratrendező
new-work-template-empty-novel-count = 1 fejezet
new-work-template-light-novel-count = 15 fejezet
new-work-template-novel-count = 20 fejezet
new-work-template-novel-in-parts-count = 3 rész, 24 fejezet
new-work-template-notebook-count = kötetlen jegyzetek
new-work-chapter-scene = Egyszerű fejezetek
new-work-chapter-scene-tip = Minden fejezet egyetlen elem, amelybe közvetlenül ír, és nincsenek alatta jelenetek. A klasszikus elrendezéshez hagyja kikapcsolva: ekkor a fejezet mappa, amelybe szintén közvetlenül írhat, de jeleneteket is tartalmazhat.
new-work-chapter-scene-tip-more = Fejezetbe mindkét esetben írhat. Az egyetlen különbség az, hogy *tartalmazhat-e* jeleneteket. A Skribisto iratrendezőjének fája csak a rendszerezést szolgálja, így mindkét elrendezésből ugyanaz a könyv áll össze, szabadon keverheti őket, és az Átalakítás egyetlen szó elvesztése nélkül váltja át a fejezetet egyikből a másikba.
new-work-name-required = Adjon nevet a projektnek
new-work-name-invalid = Ebben a névben nincs használható karakter
new-work-location-required = Válasszon helyet
new-work-location-missing = Ez a mappa nem létezik
new-work-location-not-folder = Ez az útvonal nem mappa
new-work-location-readonly = Ez a mappa nem írható
new-work-target-exists = Ebben a mappában már van ilyen nevű projekt. Létrehozáskor a program rákérdez, mielőtt lecserélné
new-work-target-is-folder = Ezen a helyen már van ilyen nevű mappa. Válasszon másik nevet
new-work-overwrite-title = Lecseréli a meglévő projektet?
new-work-overwrite-text = A(z) „{ $name }” már létezik ebben a mappában. Lecseréli az új projektre?
new-work-target-open-title = Ez a projekt meg van nyitva
new-work-target-open-text = A(z) „{ $name }” meg van nyitva a Skribistóban. Ha a helyére új projektet hoz létre, az lecserélné azt a projektet, amelyen dolgozik, és a következő mentés azt írná vissza az új fölé. Válasszon másik nevet vagy mappát, vagy előbb zárja be a projektet.
target-importing-title = Egy importálás éppen írja ezt a projektet
target-importing-text = Egy importálás még írja a(z) „{ $name }” projektet. Várja meg, amíg befejeződik, vagy válasszon másik nevet.
backup-restore-importing-text = Egy importálás még írja a(z) „{ $name }” projektet, amelyre ez a biztonsági másolat visszaállítódna. Az importálás befejeződése után állítsa vissza a biztonsági másolatot.
new-work-manuscript = Kézirat
new-work-notes = Jegyzetek
new-work-research = Kutatás
new-work-notebook = Jegyzetfüzet
new-work-scene = Jelenet
new-work-note = Jegyzet
new-work-front-matter = Előzéklapok
new-work-back-matter = Függelékek
new-work-paratext = Könyvszerkezet
new-work-paratext-none = Nincs szerkezet
new-work-paratext-hint = Az elő- és utórészek, amelyekkel egy hagyomány a könyvet kezdi és zárja. Később bármelyiket áthelyezheti, átnevezheti vagy törölheti.
new-work-tags = Címkék
new-work-tags-none = Nincsenek címkék
new-work-tags-hint = Kezdő paletta szereplők, helyszínek és egyebek címkézéséhez. Nem kötelező, és minden címke később átnevezhető, átszínezhető vagy törölhető.
new-work-note-templates = Jegyzetsablonok
new-work-note-templates-none = Nincsenek sablonok
new-work-note-templates-hint = A forma, amellyel egy történetbiblia-jegyzet indul. Nem kötelező, és minden sablon később szerkeszthető vagy törölhető.
new-work-characters = Szereplők
new-work-places = Helyszínek
import-plume-title = Plume Creator-projekt importálása
import-plume-source = Plume-projekt
import-plume-source-hint = Válasszon egy .plume vagy .plume_backup fájlt (bármely Plume Creator-verzióból).
import-plume-trash-warning = ⚠ A kukába dobott vagy törölt elemek nem kerülnek át.
import-plume-source-required = Válasszon egy Plume-projektfájlt
import-plume-source-missing = Ez a fájl nem létezik
import-plume-source-not-file = Ez az útvonal nem fájl
import-plume-manuscript-binder = Kézirat
import-plume-story-bible-binder = Történetbiblia
import-plume-progress-title = Plume-projekt importálása…
import-plume-done = Importálva: { $imported ->
    [one] { $imported } elem.
   *[other] { $imported } elem.
} { $skipped ->
    [one] { $skipped } kukába dobott elem nem került át.
   *[other] { $skipped } kukába dobott elem nem került át.
}
import-plume-nested-too-deep = A projekt nem lett importálva: a(z) „{ $part }” része több mint { $limit } szint mélységig van egymásba ágyazva. Egyetlen Plume Creator-projekt sem közelíti meg ezt, és a beolvasása összeomlaszthatná a Skribistót. Semmi sem lett kiírva.
import-plume-declares-entities = A projekt nem lett importálva: a(z) „{ $part }” része XML-entitásokat deklarál, amit egyetlen Plume Creator-projekt sem tesz. A kibontásuk több memóriát igényelhet, mint amennyi a számítógépben van, és összeomlaszthatná a Skribistót. Semmi sem lett kiírva.
import-manuskript-title = Manuskript-projekt importálása
import-manuskript-source = Manuskript-projekt
import-manuskript-source-hint = Válassza ki a .msk fájlt vagy a projekt mappáját (bármely Manuskript-verzióból).
import-manuskript-source-file = Fájl kiválasztása…
import-manuskript-source-folder = Mappa kiválasztása…
import-manuskript-source-required = Válasszon egy Manuskript-projektet
import-manuskript-source-missing = Ez a fájl vagy mappa nem létezik
import-manuskript-manuscript-binder = Kézirat
import-manuskript-story-bible-binder = Történetbiblia
import-manuskript-characters-group = Szereplők
import-manuskript-world-group = Világ
import-manuskript-plots-group = Cselekményszálak
import-manuskript-project-info-note = Projektadatok
import-manuskript-summary-note = Összefoglaló
import-manuskript-importance-minor = Mellékes
import-manuskript-importance-secondary = Másodlagos
import-manuskript-importance-main = Fő
import-manuskript-progress-title = Manuskript-projekt importálása folyamatban…
import-manuskript-done = { $imported ->
    [one] { $imported } elem importálva.
   *[other] { $imported } elem importálva.
} { $revisions ->
    [0] { "" }
    [one] { $revisions } korábbi változat is átkerült.
   *[other] { $revisions } korábbi változat is átkerült.
}
import-manuskript-nested-too-deep = A projekt nem lett importálva: a(z) „{ $part }” fájl több mint { $limit } szint mélységig van egymásba ágyazva. Egyetlen Manuskript-projekt sem közelíti meg ezt, és a beolvasása összeomlaszthatná a Skribistót. Semmi sem lett kiírva.
import-manuskript-folders-too-deep = A projekt nem lett importálva: az egyik fájlja több mint { $limit } mappa mélyen van. Egyetlen Manuskript-projekt sem közelíti meg ezt, és a beolvasása összeomlaszthatná a Skribistót. Semmi sem lett kiírva.
import-manuskript-declares-entities = A projekt nem lett importálva: a(z) „{ $part }” fájl XML-entitásokat deklarál, amit egyetlen Manuskript-projekt sem tesz. A kibontásuk több memóriát igényelhet, mint amennyi a számítógépben van, és összeomlaszthatná a Skribistót. Semmi sem lett kiírva.
import-target-open-title = Ez a projekt meg van nyitva
import-target-open-text = A(z) „{ $name }” meg van nyitva a Skribistóban. Ha fölé importál, az lecserélné azt a projektet, amelyen dolgozik, és a következő mentés azt írná vissza az importált tartalom fölé. Válasszon másik fájlnevet, vagy előbb zárja be a projektet.
import-warnings-announcement = { $result } { $notice }.
export-title = Exportálás
export-close = Bezárás
export-scope-label = Mit
export-format-label = Formátum
export-style-label = Stílus
export-path-label = Fájl
export-browse = Tallózás…
export-show-non-exportable = Nem exportálhatók megjelenítése
export-choose-empty = Nincs projekt, amelyből választani lehetne.
export-cancel = Mégse
export-export = Exportálás
export-save-dialog-title = Exportálás fájlba
export-section-what = Mit exportál
export-section-style = Stílus
export-section-destination = Célhely
export-custom-selection = Egyéni kijelölés
export-selected-count = { $count } kijelölve
export-chip-chapters-none = Fejezetek: nincs cím
export-chip-chapters-numbered = Fejezetek: számozva
export-chip-chapters-title = Fejezetek: csak cím
export-chip-chapters-both = Fejezetek: szám + cím
export-chip-scene-break-glyph = Jelenettörés: { $glyph }
export-chip-scene-break-blank = Jelenettörés: üres sor
export-chip-scene-break-none = Jelenettörés: nincs
export-chip-major-break-glyph = Nagy törés: { $glyph }
export-chip-major-break-blank = Nagy törés: üres sor
export-chip-major-break-none = Nagy törés: nincs
export-chip-major-break-same = A két törésszint egyforma
export-chip-spacing-single = Sorköz: szimpla
export-chip-spacing-onehalf = Sorköz: 1½
export-chip-spacing-double = Sorköz: dupla
export-chip-notes-included = Jegyzetekkel
export-chip-notes-excluded = Jegyzetek nélkül
export-format-docx = Word
export-format-odt = LibreOffice
export-format-html = HTML
export-format-markdown = Markdown
export-format-djot = Djot
export-format-text = Szöveg
export-format-latex = LaTeX
export-format-epub = EPUB
export-format-pdf = PDF
export-overwrite-title = Lecseréli a meglévő fájlt?
export-overwrite-text = A(z) „{ $name }” már létezik. Lecseréli?
export-progress-title = Exportálás…
export-cancelled = Az exportálás megszakítva
export-done = Exportálva: { $count ->
    [one] { $count } elem
   *[other] { $count } elem
}
export-open-file = Megnyitás
export-show-in-folder = Megjelenítés a mappában
export-error-title = Nem sikerült exportálni
export-error-details = Részletek
backup-progress-start = Indítás…
backup-progress-retention = A régi biztonsági másolatok törlése…
backup-progress-done = Kész
backup-progress-destination = { $i }. cél / { $n }
backup-details = Részletek
backup-issues-title = Biztonsági másolati problémák
backup-failed-title = A biztonsági másolat készítése sikertelen
backup-complete-prune-warning = A biztonsági másolat elkészült ({ $ok } mentve, { $skipped } már naprakész). Néhány régi biztonsági másolatot nem sikerült eltávolítani
search = Keresés
search-query-placeholder = Keresés…
search-replace-placeholder = Csere erre…
search-replace-toggle = Csere be- és kikapcsolása
search-replace-all = Összes cseréje
search-opt-case = Kis- és nagybetűk megkülönböztetése
search-opt-whole-word = Teljes szó
search-opt-diacritics = Ékezetek figyelembevétele
search-scope-body = Törzsszöveg
search-scope-title = Cím
search-scope-synopsis = Szinopszis
search-scope-label = Felirat
search-facet-book = Könyvek
search-facet-part = Részek
search-facet-chapter = Fejezetek
search-facet-scene = Jelenetek
search-facet-note = Jegyzetek
search-facet-folder = Mappák
search-tip-case = Kis- és nagybetűk megkülönböztetése: a nagy- és kisbetűket eltérőnek tekinti, így az „Elena” és az „elena” külön találat.
search-tip-whole-word = Teljes szó: csak egész szavakat talál meg, így a „kat” nem található meg a „katalógus” szóban.
search-tip-diacritics = Ékezetek figyelembevétele: az ékezetes betűket külön betűnek tekinti, így a „kave” nem egyezik a „kávé” szóval.
search-tip-body = Törzsszöveg: keresés a jelenetek és jegyzetek prózájában.
search-tip-title = Cím: keresés az iratrendező elemeinek címeiben.
search-tip-synopsis = Szinopszis: keresés minden írási elem összefoglalójában.
search-tip-label = Felirat: keresés az iratrendező elemeinek címe alatt látható rövid szövegben.
search-tip-comment = Megjegyzések: keresés a megjegyzésszálak és a rájuk adott válaszok szövegében. A megjegyzés a kéziratról szól, nem része annak, ezért külön kapcsolója van, és a csere nem jelöli be a megjegyzésekben talált egyezéseket, amíg Ön be nem jelöli őket.
search-tip-book = Könyvek: a könyvtároló és a kezdetét és végét jelölő elemek.
search-tip-part = Részek: a részszintű elválasztók.
search-tip-chapter = Fejezetek: a fejezetek, bármilyen formában tárolja is őket a projekt.
search-tip-scene = Jelenetek: a prózát tartalmazó elemek.
search-tip-note = Jegyzetek: kötetlen formájú jegyzetek.
search-tip-folder = Mappák: egyszerű rendszerező mappák és elválasztók.
search-tip-paratext = A könyvhöz tartozó, de nem a történet részét képező szöveg: előszó, ajánlás, utószó. A kézirat szószámába sosem számít bele.
search-tip-preserve-case = Kis- és nagybetűk megtartása: a csere átveszi a talált szöveg írásmódját, így az „ELENA” helyett „MARTA”, az „Elena” helyett „Marta” lesz.
search-tip-replace = Csere: a csereszöveg mezőjének és az Összes cseréje gombnak a megjelenítése.
search-error = A keresés sikertelen: { $message }
search-no-matches = Nincs találat
search-count =
    { $matches ->
        [one] { $matches } találat
       *[other] { $matches } találat
    }, { $items ->
        [one] { $items } dokumentumban
       *[other] { $items } dokumentumban
    }
search-count-truncated =
    { $matches ->
        [one] { $matches } találat
       *[other] { $matches } találat
    }, { $items ->
        [one] { $items } dokumentumban
       *[other] { $items } dokumentumban
    } (csak az első találatok)
search-occurrences = { $count }
search-field-body = Törzsszöveg
search-field-title = Cím
search-field-synopsis = Szinopszis
search-field-label = Felirat
search-field-epigraph = Mottó
search-field-comment = Megjegyzés
search-field-comment-reply = Válasz
search-field-footnote = Lábjegyzet
search-include-in-replace = Belefoglalás az Összes cseréjébe
search-collapse-all = Minden találat összecsukása
search-replace-here = Ennek cseréje
search-dismiss = Eltávolítás a találatok közül
search-undo-dismiss = Az utoljára eltávolított találat visszahozása
search-replace-nothing = (semmi)
search-replace-confirm-title = Lecseréli az összes találatot?
search-replace-confirm-text =
    Lecseréli a(z) „{ $query }” { $occurrences ->
        [one] { $occurrences } előfordulását
       *[other] { $occurrences } előfordulását
    } erre: „{ $replacement }”, { $items ->
        [one] { $items } dokumentumban
       *[other] { $items } dokumentumban
    }? Utána az értesítésből visszavonhatja.
search-replace-done-title = A csere kész
search-replace-done =
    { $occurrences ->
        [one] { $occurrences } előfordulás lecserélve
       *[other] { $occurrences } előfordulás lecserélve
    } { $items ->
        [one] { $items } dokumentumban
       *[other] { $items } dokumentumban
    }.
search-replace-done-skipped =
    { $occurrences ->
        [one] { $occurrences } előfordulás lecserélve
       *[other] { $occurrences } előfordulás lecserélve
    } { $items ->
        [one] { $items } dokumentumban
       *[other] { $items } dokumentumban
    }. { $skipped ->
        [one] { $skipped } mező
       *[other] { $skipped } mező
    } kimaradt (a keresés óta megváltozott).
search-replace-undo = Visszavonás
search-replace-failed-title = A csere sikertelen
search-replace-skipped-title = Semmi sem lett lecserélve
search-replace-skipped-body =
    { $fields ->
        [one] Ez a szöveg
       *[other] Ezek közül { $fields } szöveg
    } megváltozott a keresés óta, ezért { $fields ->
        [one] érintetlen maradt
       *[other] érintetlen maradt
    }. Keressen újra, hogy lássa, hol vannak most a szavak.
search-replace-undo-failed-title = A visszavonás sikertelen
search-preview = Előnézet
search-preview-empty = Válasszon ki egy találatot, hogy itt lássa az előnézetét
search-preview-no-prose = Ennek a találatnak nincs szerkeszthető szövege
search-preview-prompt = Az előnézethez indítson egy keresést.
search-preview-open-search = Keresés a projektben
search-preview-footnote-prompt = Ez a találat egy lábjegyzet saját szövegében van. A megtekintéséhez és szerkesztéséhez nyissa meg a Lábjegyzetek panelen.
search-preview-open-footnotes = Lábjegyzetek megnyitása
find-placeholder = Keresés ebben a dokumentumban…
find-count = { $current } / { $total }
find-no-results = Nincs találat
find-close = Keresés bezárása
find-previous = Előző találat (Shift+Enter)
find-next = Következő találat (Enter)
find-opt-case = Kis- és nagybetűk megkülönböztetése
find-opt-whole-word = Teljes szó
find-replace-toggle = Csere be- és kikapcsolása (Ctrl+R)
find-replace-placeholder = Csere erre…
find-replace = Csere
find-replace-all = Összes cseréje
find-preserve-case = Kis- és nagybetűk megtartása
dict-download-title = { $name } letöltése…
dict-download-done = Telepítve: { $name }
dict-download-failed = Nem sikerült letölteni: { $name }: { $error }
dict-removed = Eltávolítva: { $name }
dict-accept-first = A(z) { $name } letöltése előtt fogadja el a licencet
settings-dict-tab-installed = Telepítve
settings-dict-tab-get-more = Továbbiak beszerzése
settings-dict-tab-personal = Saját szavak
dict-installed-empty = Ezen a számítógépen még nem található szótár.
dict-get-more-search = Nyelvek keresése
dict-system-badge = a rendszerben
dict-unusable-badge = használhatatlan
dict-download-button = Letöltés
dict-downloading = Letöltés folyamatban…
dict-installed-label = Telepítve
dict-remove = Eltávolítás
dict-view-license = Licenc megtekintése
dict-approx-size = ~{ $size }
dict-personal-empty = Ebben a projektben még nincsenek saját szavak.
dict-personal-add = Hozzáadás
dict-personal-placeholder = Szó hozzáadása…
dict-missing-toast = { $count ->
    [one] Ez a projekt { $count } olyan szótárt használ, amely nincs telepítve
   *[other] Ez a projekt { $count } olyan szótárt használ, amely nincs telepítve
}
dict-missing-action = Szótárak beszerzése
dict-license-title = A(z) { $name } licence
dict-license-accept = Elfogadás és letöltés
dict-license-cancel = Mégse
dict-license-close = Bezárás
dict-add-button = Szótár hozzáadása…
dict-add-title = Új szótár hozzáadása
dict-add-close = Bezárás
dict-add-name = Név
dict-add-name-placeholder = pl. Saját latin szótár
dict-add-code = Nyelvkód
dict-add-code-placeholder = pl. la vagy fr-FR-x-custom
dict-add-code-hint = Egy szabadon választott rövid azonosító. Ezt választja majd ki egy dokumentum nyelveként.
dict-add-aff = Toldalékfájl (.aff)
dict-add-dic = Szólista (.dic)
dict-add-submit = Hozzáadás
dict-add-cancel = Mégse
dict-add-name-required = Adjon nevet a szótárnak.
dict-add-code-required = Adjon meg egy nyelvkódot.
dict-add-code-invalid = Csak betűket, számjegyeket és a - _ . karaktereket használjon.
dict-add-code-reserved = Ez egy beépített szótár kódja. Válasszon másikat.
dict-add-file-required = Válasszon egy fájlt.
dict-add-file-missing = Ez a fájl nem létezik.
dict-add-code-taken = Ehhez a kódhoz már telepítve van egy szótár.
dict-add-done = Hozzáadva: { $name }
dict-add-unusable = Ezek a fájlok nem alkotnak használható szótárt: { $error }
dict-add-failed = Nem sikerült hozzáadni a szótárt: { $error }
inspector-export = Exportálás
inspector-exportable = Belefoglalás az exportálásokba
ctx-number = Fejezet &számozása
ctx-unnumber = Fejezet &számozásának kikapcsolása
inspector-numbering = Számozás
inspector-numbered = Számozott
inspector-numbered-tip = Ez a fejezet részt vesz a könyv számozásában. Prológusnál, epilógusnál vagy közjátéknál kapcsolja ki.
inspector-numbered-tip-more = A számozatlan fejezet pontosan úgy marad a könyvben, ahogy volt: a címe, a prózája és a szószáma érintetlen. Egyszerűen nem kap számot, és nem is foglal el egyet: a prológus utáni fejezet az első, nem a második. Az exportálásból való kihagyás egy másik, fenti kapcsoló, amely a fejezetet teljesen eltávolítja a könyvből.
inspector-apply-to-children = Alkalmazás a gyermekelemekre
inspector-milestone = Mérföldkő dátuma
inspector-milestone-none = Nincs dátum
inspector-milestone-clear = Törlés
inspector-tags = Címkék
inspector-aliases = Más néven
inspector-dict-language = Nyelv
inspector-apply-language-to-children = A nyelv alkalmazása a gyermekelemekre
settings-page-language = Nyelv
settings-field-dict-language = Nyelvek
dict-tradeoff-hint = Minden további nyelv több szót fogad el, így kevesebb hiba akad fenn.
lang-inherit-hint = Örökölt: ez a jelenet a könyv vagy a projekt nyelveit használja.
lang-pill-list = Nyelvek
lang-pill-add = Nyelv hozzáadása
lang-pill-remove = { $name } eltávolítása
lang-pill-mute = Helyesírás-ellenőrzés kikapcsolása ehhez: { $name }
lang-pill-unmute = Helyesírás-ellenőrzés bekapcsolása ehhez: { $name }
settings-page-personal-dictionary = Saját szótár
settings-user-dict-desc = Az itt hozzáadott szavakat az alkalmazás mindenhol helyesen írottnak tekinti, és soha többé nem jelöli meg.
settings-user-dict-add-placeholder = Szó hozzáadása…
settings-user-dict-add = Szó hozzáadása
settings-user-dict-search = Szavak szűrése
settings-user-dict-count = { $count ->
    [one] { $count } szó
   *[other] { $count } szó
}
settings-user-dict-rename = Átnevezés
settings-user-dict-remove = Eltávolítás
settings-user-dict-empty = Még nincsenek szavak. Adja hozzá azokat, amelyeket a helyesírás-ellenőrzőnek figyelmen kívül kell hagynia, vagy importáljon egy listát.
settings-user-dict-import = Importálás…
settings-user-dict-export = Exportálás…
settings-user-dict-import-tip = Szavak hozzáadása egyszerű szövegfájlból, soronként egy szóval. A meglévő szavak megmaradnak.
settings-user-dict-export-tip = A teljes lista mentése egyszerű szövegfájlba, soronként egy szóval.
settings-user-dict-txt-filter = Szövegfájlok
settings-user-dict-duplicate = Már szerepel a szótárban
settings-user-dict-imported = Importálva: { $count ->
    [one] { $count } szó
   *[other] { $count } szó
} ({ $duplicates } már szerepelt).
settings-user-dict-import-failed = Nem sikerült beolvasni a szólistát: { $error }
settings-user-dict-exported = Mentve: { $count ->
    [one] { $count } szó.
   *[other] { $count } szó.
}
settings-user-dict-export-failed = Nem sikerült menteni a szólistát: { $error }
settings-page-text-replacements = Szövegcserék
settings-text-repl-desc = A rövidítést gépelés közben a teljes szövegre cseréli: a „pl” abban a pillanatban „például” lesz, amint szóközt vagy írásjelet ír.
settings-text-repl-enable = Szövegcserék használata ebben a projektben
settings-text-repl-disabled-hint = Kapcsolja be, hogy írás közben kibővülő rövidítéseket adhasson meg.
settings-text-repl-add = Szabály hozzáadása
settings-text-repl-trigger-placeholder = Rövidítés
settings-text-repl-replacement-placeholder = Amivé válik
settings-text-repl-added = Hozzáadva: „{ $trigger }”
settings-text-repl-duplicate = A(z) „{ $trigger }” rövidítéshez már van szabály
settings-text-repl-filter = Szabályok szűrése
settings-text-repl-count = { $n ->
    [one] 1 szabály
   *[other] { $n } szabály
}
settings-text-repl-row-enabled = A szabály használata
settings-text-repl-delete = A(z) { $trigger } szabályának törlése
settings-text-repl-deleted = A(z) „{ $trigger }” szabálya törölve
settings-text-repl-empty = Még nincsenek szabályok.
settings-text-repl-csv-filter = CSV-fájlok
settings-text-repl-import = Importálás…
settings-text-repl-export = Exportálás…
settings-text-repl-imported = Importálva: { $added }, kihagyva: { $skipped }
settings-text-repl-exported = Exportálva: { $n ->
    [one] 1 szabály
   *[other] { $n } szabály
}
editor-menu-no-suggestions = Nincs javaslat
editor-menu-add-to-dictionary = „{ $word }” hozzáadása a szótárhoz
editor-menu-add-words-to-dictionary = A kijelölt szavak hozzáadása a szótárhoz
editor-dict-added = „{ $word }” hozzáadva a szótárhoz.
editor-dict-added-multi = { $count ->
    [one] { $count } szó hozzáadva a szótárhoz.
   *[other] { $count } szó hozzáadva a szótárhoz.
}
toast-undo = Visszavonás
undo-superseded-title = Ez a lépés már nem az utolsó
undo-superseded-body = Azóta valami más is megváltozott a projektben. A visszavonás azt vonná vissza, ezért nem történt meg. Ennek a lépésnek egyetlen hatása sem lett visszafordítva. A Szerkesztés ▸ Visszavonás paranccsal saját maga lépkedhet vissza az előzményekben.
undo-failed = A visszavonás sikertelen: { $error }
menu-edit = S&zerkesztés
menu-edit-undo = &Visszavonás
menu-edit-redo = &Mégis
menu-edit-undo-target = &Visszavonás: { $target }
menu-edit-redo-target = &Mégis: { $target }
shortcut-name-edit-undo = Visszavonás
shortcut-name-edit-redo = Mégis
undo-target-typing = gépelés
undo-target-project = a projekt utolsó módosítása
prose-history-reset-title = A gépelési előzmények törölve
prose-history-reset-body = { $count ->
    [one] Egy megnyitott jelenet egy korábbi állapotra lett visszaállítva, így a gépelési előzményei már nem érvényesek.
   *[other] { $count } megnyitott jelenet egy korábbi állapotra lett visszaállítva, így a gépelési előzményeik már nem érvényesek.
}
undo-target-trash = áthelyezés a kukába
undo-target-restore = visszaállítás a kukából
undo-target-delete-forever = végleges törlés
undo-target-replace-all = csere a teljes projektben
undo-target-import = a dokumentum importálása
undo-target-duplicate = kettőzés
undo-target-move = áthelyezés
undo-target-merge = két jelenet egyesítése
undo-target-split = jelenet kettéosztása
undo-target-promote = típusváltás
undo-target-tidy-titles = a fejezetcímek rendezése
undo-target-import-tags = címkék importálása
undo-target-import-templates = jegyzetsablonok importálása
undo-target-create = létrehozás
undo-target-remove = törlés
undo-target-rename = átnevezés
undo-target-edit = ez a módosítás
menu-edit-find = &Keresés…
menu-edit-find-next = Következő &találat
menu-edit-find-prev = &Előző találat
menu-edit-replace = Keresés és &csere…
menu-edit-find-in-project = Keresés a &projektben…
menu-edit-replace-in-project = Csere a pr&ojektben…
undo-frozen = Visszavonás (a „Mindig előre” be van kapcsolva)
redo-frozen = Mégis (a „Mindig előre” be van kapcsolva)
menu-edit-cut = Kivá&gás
menu-edit-copy = Má&solás
menu-edit-paste = &Beillesztés
menu-edit-paste-plain = Beillesztés &formázás nélkül
menu-edit-select-all = Összes ki&jelölése
shortcut-name-edit-cut = Kivágás
shortcut-name-edit-copy = Másolás
shortcut-name-edit-paste = Beillesztés
shortcut-name-edit-paste-plain = Beillesztés formázás nélkül
shortcut-name-edit-select-all = Összes kijelölése
titlebar-spellcheck-on = A helyesírás-ellenőrzés be van kapcsolva. Kattintson a kikapcsoláshoz (F7)
titlebar-spellcheck-off = A helyesírás-ellenőrzés ki van kapcsolva. Kattintson a bekapcsoláshoz (F7)
menu-spellcheck = &Helyesírás-ellenőrzés
menu-comments = &Megjegyzések
settings-page-spellcheck = Helyesírás-ellenőrzés
settings-group-spellcheck = Helyesírás-ellenőrzés
settings-spellcheck-enabled = Helyesírás-ellenőrzés írás közben
settings-spellcheck-hint = Aláhúzza azokat a szavakat, amelyeket egyik telepített szótár sem ismer. Kikapcsolásával minden ellenőrzés leáll, minden projektben, amíg vissza nem kapcsolja. Ha csak egy nyelv ellenőrzését szeretné leállítani, a projekt vagy egy elem Nyelv mezőjében vegye ki a pipát a nyelv mellől.
menu-trash = K&uka
trash-title = Kuka
trash-empty-state = A kuka üres.
trash-empty-button = Kuka ürítése…
trash-restore = &Visszaállítás
trash-restore-to = Visszaállítás &ide…
trash-delete-forever = Végleges &törlés
trash-restored-ok = { $count } visszaállítva.
trash-restore-error = Nem sikerült a visszaállítás: { $error }
trash-restore-orphaned = Az elem eredeti helye már nem létezik. Válassza ki, hová kerüljön vissza.
trash-restore-no-project = Nincs megnyitott projekt, így nincs mit visszaállítani.
trash-delete-no-project = Nincs megnyitott projekt, így nincs mit törölni.
trash-empty-confirm-title = Kiüríti a kukát?
trash-empty-confirm-text = { $count ->
    [one] { $count } kukába dobott bejegyzés véglegesen törlődik.
   *[other] { $count } kukába dobott bejegyzés véglegesen törlődik.
} Végleg kikerülnek a kukából. A Visszavonás még visszahozhatja őket, amíg mást nem tesz.
trash-emptied-title = A kuka kiürítve
trash-emptied-body = A kukában lévő minden elem véglegesen törlődött.
trash-empty-no-project = Nincs megnyitott projekt, így nincs kiüríthető kuka.
trash-delete-forever-confirm-title = Végleges törlés?
trash-delete-forever-confirm-text = { $count ->
    [one] { $count } elem véglegesen törlődik.
   *[other] { $count } elem véglegesen törlődik.
} Végleg kikerülnek a kukából. A Visszavonás még visszahozhatja őket, amíg mást nem tesz.
trash-deleted-title = Véglegesen törölve
trash-deleted-body = { $count } véglegesen törölve.
trash-undo = Visszavonás
trash-restore-picker-title = Visszaállítás ide…
trash-restore-to-confirm-title = Visszaállítja ide?
trash-restore-to-confirm-text = Visszaállítja a(z) „{ $item }” elemet ide: „{ $destination }”?
trash-restore-picker-restore-here = Visszaállítás ide
trash-restore-picker-cancel = Mégse
trash-restore-picker-empty = Még nincs iratrendező. Előbb hozzon létre egyet.
trash-banner-title = Ez az elem a kukában van
trash-banner-description = Amíg vissza nem állítja, nem jelenik meg a vázlatban és az exportálásokban.
trash-banner-restore = Visszaállítás…
trash-tab-tooltip = A kukában
settings-styles-sheet-yes = Igen
settings-styles-sheet-no = Nem
settings-styles-sheet-font = Betűkészlet
settings-styles-sheet-indent = Első sor behúzása
settings-styles-sheet-para-spacing = Bekezdések közötti térköz
settings-styles-sheet-page = Oldalméret
settings-styles-sheet-margins = Margók (F/J/A/B)
settings-styles-sheet-title-page = A könyv címoldala
settings-styles-sheet-heading-language = A címsorok nyelve
settings-styles-sheet-auto = A szöveget követi
settings-styles-sheet-digits = Számjegyek
settings-styles-sheet-direction = Szövegirány
settings-styles-sheet-formats = Formátumok
settings-styles-sheet-all-formats = Mind
settings-styles-editor-group = Stílusszerkesztő
settings-styles-editor-missing = Ez a stílus már nem érhető el.
settings-styles-page-letter = Letter
settings-styles-digits-western = Nyugati (0–9)
settings-styles-digits-eastern-arabic = Keleti arab (٠–٩)
settings-styles-direction-ltr = Balról jobbra
settings-styles-direction-rtl = Jobbról balra
format-dock-title = Formátum
format-panel-empty = A formázási lehetőségekhez helyezze a kurzort egy jelenetbe, jegyzetbe vagy szinopszisba.
format-group-marks = Szöveg
format-group-block = Bekezdés
format-group-lists = Listák
format-group-tables = Táblázat
format-group-breaks = Jelenettörések
format-superscript = Felső index
format-subscript = Alsó index
format-link = Hivatkozás…
link-dialog-insert-title = Hivatkozás beszúrása
link-dialog-edit-title = Hivatkozás szerkesztése
link-dialog-text-label = Szöveg
link-dialog-text-placeholder = az olvasó által látott szavak
link-dialog-url-label = Webcím
link-dialog-url-placeholder = example.com
link-dialog-insert = Beszúrás
link-dialog-apply = Alkalmaz
link-dialog-cancel = Mégse
link-dialog-remove = Hivatkozás eltávolítása
link-scheme-refused = Csak webes és e-mail-hivatkozások nyithatók meg. A(z) { $url } nem lett megnyitva.
menu-format-link = Hi&vatkozás…
format-clear = Formázás törlése
format-heading = Címsorszint
format-heading-normal = Normál szöveg
format-heading-1 = Címsor 1
format-heading-2 = Címsor 2
format-heading-3 = Címsor 3
format-heading-4 = Címsor 4
format-heading-5 = Címsor 5
format-heading-6 = Címsor 6
format-align-left = Balra igazítás
format-align-center = Középre igazítás
format-direction-rtl = Jobbról balra író bekezdés
format-blockquote = Blokkidézet
format-list-bullet = Felsorolás
format-list-numbered = Számozott lista
format-indent = Behúzás növelése
format-outdent = Behúzás csökkentése
format-table-insert = Táblázat beszúrása
format-table-row-above = Sor beszúrása fölé
format-table-row-below = Sor beszúrása alá
format-table-col-before = Oszlop beszúrása elé
format-table-col-after = Oszlop beszúrása mögé
format-table-row-delete = Sor törlése
format-table-col-delete = Oszlop törlése
format-table-remove = Táblázat eltávolítása
menu-format-marks-bold = &Félkövér
menu-format-marks-italic = &Dőlt
menu-format-marks-underline = &Aláhúzott
menu-format-marks-strike = Át&húzott
menu-format-marks-superscript = Felső &index
menu-format-marks-subscript = Alsó inde&x
menu-format-marks-clear = Formázás tö&rlése
menu-format-heading = &Címsor
menu-format-heading-normal = &Normál szöveg
menu-format-heading-1 = Címsor &1
menu-format-heading-2 = Címsor &2
menu-format-heading-3 = Címsor &3
menu-format-heading-4 = Címsor &4
menu-format-heading-5 = Címsor &5
menu-format-heading-6 = Címsor &6
menu-format-alignment = Iga&zítás
menu-format-align-left = &Balra
menu-format-align-center = &Középre
menu-format-direction = &Szövegirány
menu-format-direction-auto = &Automatikus
menu-format-direction-ltr = &Balról jobbra
menu-format-direction-rtl = &Jobbról balra
menu-format-blockquote = &Blokkidézet
menu-format-lists = &Listák
menu-format-list-bullet = &Felsorolás
menu-format-list-numbered = &Számozott lista
menu-format-indent = Behúzás &növelése
menu-format-outdent = Behúzás &csökkentése
menu-format-table = &Táblázat
menu-format-table-insert = Táblázat &beszúrása
menu-format-table-2x2 = &2 × 2
menu-format-table-3x3 = &3 × 3
menu-format-table-4x4 = &4 × 4
menu-format-table-row-above = Sor beszúrása &fölé
menu-format-table-row-below = Sor beszúrása &alá
menu-format-table-col-before = Oszlop beszúrása &elé
menu-format-table-col-after = Oszlop beszúrása &mögé
menu-format-table-row-delete = &Sor törlése
menu-format-table-col-delete = &Oszlop törlése
menu-format-table-remove = Táblázat el&távolítása
menu-format-typo-marks = Formázási j&el
menu-format-typo-nbsp = Nem törhető &szóköz
menu-format-typo-nbhyphen = Nem törhető &kötőjel
menu-format-typo-soft-hyphen = &Feltételes kötőjel
menu-format-typo-nnbsp = Keske&ny nem törhető szóköz
menu-format-typo-zwsp = &Zérus szélességű szóköz
menu-format-typo-word-joiner = Szóösszekö&tő
menu-about = A Skribisto &névjegye…
about-title = A Skribisto névjegye
about-version = { $version } verzió
about-update-available = Elérhető a(z) { $version } verzió
about-update-see-notes = A változások megtekintése
about-update-flathub = Ezt a példányt a Flathub tartja naprakészen.
about-update-distro = Ezt a példányt a disztribúciója tartja naprakészen.
about-tagline = Regényíró alkalmazás hosszú prózai művekhez, Rust nyelven, a Teksilo eszközkészlettel írva.
about-license = A GNU General Public License 3. verziója alatt kiadva.
about-copyright = © 2026 Cyril Jacquet
about-close = Bezárás
native-menu-about = { $app } névjegye
native-menu-hide = { $app } elrejtése
native-menu-quit = Kilépés: { $app }
native-menu-settings = Beállítások…
native-menu-window = Ablak
native-menu-minimize = Kis méret
native-menu-zoom = Nagyítás
window-title = { $title } - { $app }
window-title-numbered = { $title } - { $app } ({ $n }. ablak)
window-title-empty = { $app }
first-run-window-title = Üdvözöljük
first-run-title = { $app } beállítása
first-run-body = A(z) { $app } a Skribistótól külön tárolja a beállításait, ezért üresen indul. A beállításai, a legutóbbi projektek, a szótárak és az ablakelrendezés most átmásolhatók.
first-run-from = Másolás innen
first-run-to = Másolás ide
first-run-copy-note = Semmi sem kerül áthelyezésre vagy törlésre. A Skribisto minden saját beállítását megtartja, és pontosan úgy működik tovább, mint eddig.
first-run-import = Beállítások importálása
first-run-start-fresh = Kezdés tiszta lappal
first-run-import-failed = Néhány beállítást nem sikerült importálni: { $error }
settings-page-punctuation = Írásjelek alapértelmezései
settings-group-punctuation = Intelligens írásjelek
settings-page-work-punctuation = Írásjelek
settings-punctuation-override = Saját írásjelszabályok ennek a projektnek
settings-punctuation-override-hint = Kikapcsolva: a projekt az alkalmazás beállítását követi. Ezek a szabályok a .skrib fájlban tárolódnak, így a fájlt megnyitó szerzőtárs ugyanazzal a tipográfiával ír.
settings-punctuation-dashes = A -- nagykötőjellé, a --- hosszú gondolatjellé alakítása
settings-punctuation-ellipsis = A ... három pont karakterré alakítása
settings-punctuation-quotes = Tipográfiai idézőjelek és aposztrófok
settings-quote-style = Idézőjelek
settings-quote-style-locale = A nyelv alapértelmezése
settings-quote-style-curly = “Dupla”
settings-quote-style-curly-single = ‘Szimpla’
settings-quote-style-guillemets = «Francia»
settings-quote-style-low-high = „Alsó-felső“
settings-punctuation-spacing = Szóköz a ; : ! ? előtt
settings-punctuation-spacing-hint = A francia tipográfia keskeny nem törhető szóközt tesz a ; ! ? elé, és teljeset a : elé. Csak francia nyelvű szövegre vonatkozik. A « » idézőjelek belső szóköze magukkal a francia idézőjelekkel jár.
settings-punctuation-sample = Az Ön nyelvén
settings-punctuation-app-hint = Minden projekt ezt követi, hacsak a Projekt ▸ Írásjelek oldalon nem állít be saját szabályokat.
settings-punctuation-dialogue = A „- ” jellel kezdett bekezdés párbeszédjellel induljon
settings-punctuation-dialogue-hint = Olyan nyelvekhez, amelyek az idézőjelek helyett gondolatjellel jelölik a beszédet: a magyar, a francia, a spanyol, az orosz és mások. Csak a bekezdés legelején lép működésbe.
statusbar-go-to = Ugrás…
go-to-placeholder = Keresés az iratrendezőben
go-to-no-matches = Egyetlen elem sem felel meg a keresésnek.
menu-go-to = &Ugrás…
settings-page-distraction-free-themes = A zavarmentes mód témái
settings-themes-builtin = Beépített témák
settings-themes-builtin-badge = Beépített
settings-themes-user = Saját témák
settings-themes-editor-group = Téma szerkesztése
settings-themes-editor-empty = A szerkesztéshez válasszon egy témát a „Saját témák” közül.
settings-themes-use = Használat
settings-themes-duplicate = Kettőzés
settings-themes-edit = Szerkesztés
settings-themes-delete = Törlés
settings-themes-export = Exportálás…
settings-themes-import = Importálás…
settings-themes-copy-suffix = másolat
settings-themes-json-filter = Téma (JSON)
settings-themes-imported = A téma importálva
settings-themes-import-failed = Nem sikerült importálni a témát
settings-themes-exported = A téma exportálva
settings-themes-export-failed = Nem sikerült exportálni a témát
settings-themes-low-contrast = alacsony kontraszt
settings-themes-low-contrast-band = a kiemelés eltakarja a szöveget
settings-themes-field-name = Név
settings-themes-field-paper = Lap
settings-themes-field-ink = Betű
settings-themes-field-general = Háttér
settings-themes-field-widget-text = A vezérlősáv szövege
settings-themes-field-caret-band = Kiemelés a kurzor körül
statusbar-focus-settings = Zavarmentes mód beállításai
statusbar-focus-manage-themes = Témák kezelése…
comments-title = Megjegyzések
comments-document-title = Ez a dokumentum
comments-empty-project = Ebben a projektben még nincsenek megjegyzések.
comments-empty-document = Ehhez a dokumentumhoz nincsenek megjegyzések.
comments-filter-all = Mind
comments-filter-open = Nyitott
comments-filter-resolved = Lezárt
comments-filter-orphaned = Gazdátlan
comments-status-open = Nyitott
comments-status-resolved = Lezárt
comments-status-orphaned = Elveszett a szövege
comments-orphan-snippet = (a megjegyzéssel ellátott szöveg már nem létezik)
comments-status-unplaced = Nincs szöveghelye
comments-unplaced-snippet = (nincs szöveghez kötve)
comments-reply-count = { $count ->
    [one] { $count } válasz
   *[other] { $count } válasz
}
comments-menu-resolve = Lezárás
comments-menu-reopen = Újranyitás
comments-menu-delete = Törlés
comments-sort-document = Dokumentumbeli sorrendben
comments-sort-newest = Legújabbak elöl
comments-sort-reader = Olvasók szerint csoportosítva
comments-unsigned-author = Aláíratlan
comments-agreement = { $count ->
    [one] { $count } olvasó
   *[other] { $count } olvasó
}
comments-menu-add = Megjegyzés hozzáadása
comments-menu-add-paragraph = Megjegyzés ehhez a bekezdéshez
overview-col-comments = Megjegyzések
overview-col-total-comments = Összes megjegyzés
comments-card-placeholder = Írjon megjegyzést…
comments-card-unknown-author = Ismeretlen szerző
comments-unsigned-toast = A megjegyzései aláíratlanok: ezen a számítógépen nincs megadva az Ön neve.
comments-unsigned-action = Név megadása
comments-card-reply = Válasz
comments-card-reply-placeholder = Válasz…
comments-card-actions = Megjegyzésműveletek
comments-reply-actions = Válaszműveletek
comments-menu-delete-reply = Válasz törlése
comments-menu-delete-all = Az itteni összes megjegyzés törlése
comments-deleted-toast = Megjegyzés törölve
comments-reply-deleted-toast = Válasz törölve
comments-deleted-all-toast = { $count ->
    [one] { $count } megjegyzés törölve
   *[other] { $count } megjegyzés törölve
}
comments-undo = Visszavonás
analysis-segment = Elemzés
analysis-scope-book = A könyv elemzése
analysis-run = Elemzés futtatása
analysis-stale = A futtatás óta megváltozott
analysis-not-run = Még nincs elemezve.
analysis-running = A kézirat olvasása…
analysis-failed = Az elemzést nem sikerült befejezni.
analysis-no-scenes = Ebben a könyvben még nincsenek jelenetek.
analysis-shape = Alak
analysis-arrivals = Beérkezés
analysis-arrivals-explainer = Hogyan került szöveg a projektbe, amíg meg volt nyitva. Azt mutatja meg, milyen úton érkeztek a karakterek, és semmit sem arról, ki írta őket: aki máshol fogalmaz és beilleszt, az beillesztett, aki diktál, az diktált. Itt nincs elérendő szám.
analysis-arrivals-scope = Az egész projekt, nem csak ez a könyv.
analysis-arrivals-session = A megnyitás óta. A projekt bezárása újraindítja a számlálást.
analysis-arrivals-nothing = Ebben a munkamenetben még nem érkezett szöveg.
analysis-arrivals-typed = Gépelt
analysis-arrivals-pasted = Beillesztett
analysis-arrivals-dictated = Diktált
analysis-arrivals-imported = Importált
analysis-arrivals-programmatic = A program szúrta be
analysis-arrivals-count = { $count ->
    [one] { $count } karakter
   *[other] { $count } karakter
}
analysis-arrivals-none = nincs
analysis-words-per-scene = Szavak jelenetenként
analysis-median-words = A könyv medián jelenete { $count ->
    [one] { $count } szó hosszú.
   *[other] { $count } szó hosszú.
}
analysis-median-line = Medián: { $count ->
    [one] { $count } szó
   *[other] { $count } szó
}
analysis-dialogue = Párbeszéd
analysis-dialogue-unsupported = Ezen a nyelven a párbeszéd aránya még nem mérhető.
analysis-footnote-words = Lábjegyzetszavak
analysis-footnote-words-count = { $count ->
    [one] { $count } szó van a könyv lábjegyzeteiben, a kézirat összesítésétől külön számolva.
   *[other] { $count } szó van a könyv lábjegyzeteiben, a kézirat összesítésétől külön számolva.
}
analysis-footnote-words-pending = A lábjegyzetek számolása…
analysis-ignore-empty = A még üres szövegek figyelmen kívül hagyása
analysis-empty-hidden = { $count } üres { $count ->
        [one] szöveg
       *[other] szöveg
    } elrejtve.
analysis-all-texts-empty = A könyvben még minden szöveg üres.
binder-filter-count = { $shown } / { $total } látható
binder-filter-clear = Szűrő törlése
binder-filter-none = Semmi sem felel meg ennek: „{ $query }”.
settings-paratext-intro = Az elő- és utórészek, amelyekkel egy új projekt indulhat. Minden szerkezet egy kiadói hagyományhoz tartozik, és az oldalcímei e hagyomány saját nyelvén vannak megírva. A projekt létrehozása után szabadon átnevezheti őket.
settings-paratext-structures = Szerkezetek
settings-paratext-broken = Nem sikerült beolvasni
settings-paratext-edit = Szerkesztés
settings-paratext-duplicate = Kettőzés
settings-paratext-delete = Törlés
settings-paratext-new = Új szerkezet
settings-paratext-save = Mentés
settings-paratext-editor-hint = Egy név, a kézirat előtt és a kézirat után létrehozandó oldalak. Hogy hová kerülnek, az az Ön dolga: a projekt létrejötte után ezek közönséges elemek.
image-insert = Ké&p beszúrása…
image-no-project = Kép beszúrása előtt nyisson meg egy projektet.
image-choose-title = Kép kiválasztása
image-filter-label = Képek
image-large-title = Ez egy nagy kép
image-large-text =
    A(z) { $name } { $megapixels } megapixeles ({ $width }×{ $height }).
    Ha változatlanul megtartja, az eredeti fájl kerül a projektbe: minden
    biztonsági másolattal és exportálással együtt utazik. Az optimalizálás
    ehelyett egy kisebb másolatot tárol, amelynek hosszabbik oldala legfeljebb 2560 képpont.
image-large-keep = Eredeti megtartása
image-large-downscale = Optimalizálás
image-large-remember = Mostantól mindig így, ne kérdezzen újra
image-not-recorded = A kép mentve lett, de nem sikerült rögzíteni a projektben.
image-describe-title = A kép leírása
image-describe-explain = Mit ábrázol a kép, egy olyan olvasó számára, aki nem látja. Nem része a kéziratnak: sosem számít bele a szószámba, nem kereshető, és nem exportálódik prózaként.
image-describe-placeholder = világítótorony szürke ég előtt
image-resize-title = A kép átméretezése
image-resize-explain = A jelenlegi megjelenítési méret százalékában. A 100 változatlanul hagyja.
image-resize-invalid = Adjon meg egy számot 1 és 1000 között.
image-menu-describe = A kép &leírása…
image-menu-resize = A kép &átméretezése…
image-menu-reset-size = &Eredeti méret
cover-choose = Könyvbo&rító…
cover-clear = Borító el&távolítása
cover-choose-title = Borító kiválasztása
cover-set = A borító beállítva.
cover-cleared = A borító eltávolítva. A kép továbbra is a projektben van.
card-label = Felirat
card-exportable = Exportálva
card-numbered = Számozott
card-created = Létrehozva
card-modified = Módosítva
card-yes = igen
card-no = nem
card-type = Típus
card-position = Helyzet
card-children = Gyermekelemek
card-goal = Cél
card-words = Szavak
card-synopsis = Szinopszis
card-point-of-view = Nézőpont
card-aliases = Más néven
export-orphan-footnotes-title = Hivatkozás nélküli lábjegyzetek
export-comments-dropped =
    { $count ->
        [one] Egy megjegyzést nem sikerült elhelyezni az exportált szövegben, ezért kimaradt.
       *[other] { $count } megjegyzést nem sikerült elhelyezni az exportált szövegben, ezért kimaradtak.
    }
export-orphan-footnotes =
    { $count ->
        [one] Egy lábjegyzetre a kézirat már sehol sem hivatkozik. A szövege nem jelenik meg az exportált könyvben.
       *[other] { $count } lábjegyzetre a kézirat már sehol sem hivatkozik. A szövegük nem jelenik meg az exportált könyvben.
    }
menu-footnotes = &Lábjegyzetek
footnotes-title = Lábjegyzetek
footnotes-insert = &Lábjegyzet beszúrása
footnotes-empty = Még nincsenek lábjegyzetek. Helyezze a kurzort egy jelenetbe, majd használja a fenti + gombot vagy a Ctrl+Alt+F billentyűparancsot.
footnotes-filter-all = Mind
footnotes-filter-document = Ez a dokumentum
footnotes-filter-orphaned = Gazdátlan
footnotes-orphaned = Már semmi sem hivatkozik erre a lábjegyzetre
footnotes-untitled-home = Névtelen
footnotes-body-placeholder = maga a lábjegyzet
footnotes-insert-tooltip = Lábjegyzet beszúrása a kurzor helyére (Ctrl+Alt+F)
footnotes-actions = Lábjegyzetműveletek
footnotes-delete = A lábjegyzet és hivatkozásának &törlése
footnotes-no-project = Lábjegyzet beszúrása előtt nyisson meg egy projektet.
footnotes-no-caret = Lábjegyzet beszúrásához helyezze a kurzort egy jelenet szövegébe.
footnotes-not-created = Nem sikerült hozzáadni a lábjegyzetet a projekthez.
footnotes-deleted-toast = Lábjegyzet törölve
footnotes-undo-delete = Visszavonás
import-document-title = Dokumentumok importálása
import-document-close = Bezárás
import-document-step-files = Fájlok
import-document-step-review = Ellenőrzés
import-document-step-destination = Célhely
import-document-step-reconcile = Egyesítés
import-document-reconcile-hint = Ezek közül néhány elem már megvan Önnél. Adja meg, mi történjen mindegyikkel.
import-document-reconcile-all-new = Ebben a fájlban semmi sem egyezik a projekttel: minden elem újként kerül be.
import-document-duplicate-returns = Ezek a fájlok ugyanannak a kéziratnak több visszaérkezett példányát tartalmazzák. Egyenként importálja őket: az egyeztetés csak egy visszaérkezett példányt vet össze a projekttel, így ha egyszerre importálná őket, a könyv második példánya kerülne be ahelyett, hogy egyesülne.
import-document-hunk-added = Ennek az új bekezdésnek az átvétele
import-document-hunk-removed = Ennek a bekezdésnek az elvetése
import-document-hunk-changed = Ennek az átírásnak az átvétele
import-document-col-stray-prose = A szövege
import-document-stray-as-paratext = Megtartás paratextusként
import-document-stray-discard = A szöveg elvetése
import-document-col-current = Az Ön projektjében
import-document-col-incoming = Ebben a fájlban
import-document-col-status = Állapot
import-document-col-action = Teendő
import-document-status-identical = Azonos
import-document-status-editor-edited = Ők szerkesztették
import-document-status-you-edited = Ön szerkesztette
import-document-status-conflict = Mindkét fél szerkesztette
import-document-status-different = Eltér
import-document-status-new = Új
import-document-status-missing = Nincs ebben a fájlban
import-document-status-moved = Áthelyezve
import-document-action-comments-only = Csak a megjegyzések
import-document-action-take-import = Ennek a változatnak az átvétele
import-document-action-keep-current = A saját megtartása
import-document-action-create-new = Hozzáadás újként
import-document-action-ignore = Kihagyás
import-document-compare = Összehasonlítás
import-document-compare-legend = Az Ön változata összevetve a fájlban lévővel. Csak olvasható.
import-document-compare-close = Bezárás
import-document-drop-title = Húzza ide a dokumentumokat
import-document-drop-hint = Markdown (.md), Word (.docx), OpenDocument (.odt) és egyszerű szöveg (.txt)
import-document-browse = Tallózás…
import-document-move-up = Mozgatás fel
import-document-move-down = Mozgatás le
import-document-remove-file = Eltávolítás
import-document-file-count = { $count ->
    [one] 1 fájl
   *[other] { $count } fájl
}
import-document-no-files = Még nincs kiválasztott fájl.
import-document-col-included = Importálás
import-document-col-title = Cím
import-document-col-type = Típus
import-document-col-words = Szavak
import-document-col-breaks = Törések
import-document-col-comments = Megjegyzések
import-document-col-footnotes = Lábjegyzetek
import-document-col-epigraph = Mottó
import-document-col-source = Forrás
import-document-level-rules = Címsorszintek
import-document-level-n = Címsor { $level }
import-document-add-top-level = Legfelső szint hozzáadása
import-document-add-top-level-tooltip = Egy Könyv beszúrása minden elemzett elem fölé, könyvcímsor nélküli fejezetfájlokhoz
import-document-destination = Célhely
import-document-destination-hint = Válasszon iratrendezőt vagy elemet: az új elemek mappába, illetve jelenet után kerülnek.
import-document-destination-empty = Még nincs iratrendező. Előbb hozzon létre egyet.
import-document-plan-empty = Még nincs mit importálni.
import-document-summary = { $rows ->
    [one] 1 elem
   *[other] { $rows } elem
} · { $breaks ->
    [one] 1 jelenettörés
   *[other] { $breaks } jelenettörés
}
import-document-back = Vissza
import-document-cancel = Mégse
import-document-analyse = Tovább
import-document-analysing = A dokumentumok olvasása…
import-document-step-analysing = Olvasás
import-document-cancel-analysis = Olvasás leállítása
import-document-analyse-failed = A dokumentumokat nem sikerült beolvasni.
import-document-details = Részletek
import-document-import = Importálás
import-document-done = { $count ->
    [one] 1 elem importálva
   *[other] { $count } elem importálva
}
import-document-undo = Visszavonás
import-diagnostic-file-unreadable = A(z) „{ $path }” nem olvasható: { $detail }. A többi fájl importálása folytatódik.
import-diagnostic-nested-too-deep = A(z) „{ $path }” nem lett beolvasva: a tartalma több mint { $limit } szint mélységig van egymásba ágyazva. Valódi dokumentum nem közelíti meg ezt, és a beolvasása összeomlaszthatná a Skribistót, ezért kimarad. A többi fájl importálása folytatódik.
import-diagnostic-archive-too-large = A(z) „{ $path }” nem lett beolvasva: olyan tömörített fájl, amely kibontva a méreténél sokkal nagyobbra nő, ami kimerítheti a memóriát és összeomlaszthatná a Skribistót, ezért kimarad. A többi fájl importálása folytatódik.
import-diagnostic-lossy-decode = { $count ->
    [one] A(z) „{ $path }” egy karakterét nem sikerült dekódolni. A megőrzéséhez mentse újra a fájlt UTF-8 kódolással.
   *[other] A(z) „{ $path }” { $count } karakterét nem sikerült dekódolni. A megőrzésükhöz mentse újra a fájlt UTF-8 kódolással.
}
import-diagnostic-decoded-from-bom = A(z) „{ $path }” { $detail } kódolásként lett dekódolva, nem UTF-8-ként.
import-diagnostic-empty-file = A(z) „{ $path }” üres.
import-diagnostic-no-headings = A(z) „{ $path }” nem tartalmaz címsorokat, ezért egyetlen elemként érkezik.
import-diagnostic-unsupported-format = A(z) „.{ $detail }” fájlokat semmi sem olvassa, ezért a(z) „{ $path }” kimaradt.
import-diagnostic-front-matter-not-flat = Előtét a(z) „{ $path }” fájlban: a(z) „{ $detail }” nem egyszerű érték, ezért kimaradt.
import-diagnostic-footnotes-degraded = { $count ->
    [one] A(z) „{ $path }” egy lábjegyzete egyszerű szövegként érkezik: Markdownból a lábjegyzetek nem olvashatók be.
   *[other] A(z) „{ $path }” { $count } lábjegyzete egyszerű szövegként érkezik: Markdownból a lábjegyzetek nem olvashatók be.
}
import-diagnostic-footnote-not-carried = { $count ->
    [one] A(z) „{ $path }” egy lábjegyzetét nem sikerült átvenni. A többit igen.
   *[other] A(z) „{ $path }” { $count } lábjegyzetét nem sikerült átvenni. A többit igen.
}
import-diagnostic-raw-html-dropped = { $count ->
    [one] A(z) „{ $path }” egy nyers HTML-blokkja kimaradt.
   *[other] A(z) „{ $path }” { $count } nyers HTML-blokkja kimaradt.
}
import-diagnostic-nested-break-dropped = { $count ->
    [one] A(z) „{ $path }” egy idézeten vagy listán belüli jelenettörése kimaradt. Csak a külön sorban álló törés marad meg.
   *[other] A(z) „{ $path }” { $count } idézeten vagy listán belüli jelenettörése kimaradt. Csak a külön sorban álló törés marad meg.
}
import-diagnostic-image-not-ingested = A(z) „{ $path }” a(z) „{ $detail }” képre hivatkozik. A hivatkozás szövegként érkezik; magát a képet nem másolja át a program.
import-diagnostic-duplicate-title = A(z) „{ $title }” { $count } alkalommal szerepel. Ha ezeket a fájlokat korábban már importálta, most megkettőződnek.
import-diagnostic-heading-level-jump = A(z) „{ $title }” a(z) { $from }. címsorszintről a(z) { $to }. szintre ugrik; a szülője alá egy szinttel lejjebb kerül.
import-diagnostic-illegal-combination = A(z) „{ $title }” prózát tartalmaz, de egy { $kind } típusú elem nem tárolhat prózát. Az importálás addig várakozik, amíg meg nem változtatja a típusát, vagy ki nem hagyja. Addig semmi sem kerül importálásra.
import-diagnostic-tracked-changes-flattened = A(z) { $path } átdolgozás alatt állt: { $count ->
    [one] { $count } követett módosítás el lett fogadva
   *[other] { $count } követett módosítás el lett fogadva
}, a törlések pedig kimaradtak. Ez a végleges szöveg. Ha inkább a saját megfogalmazását tartaná meg, használja az utolsó lépés Összehasonlítás funkcióját.
import-diagnostic-tracked-changes-flattened-by = A(z) { $path } átdolgozás alatt állt: { $count ->
    [one] { $names } { $count } követett módosítása el lett fogadva
   *[other] { $names } { $count } követett módosítása el lett fogadva
}, a törlések pedig kimaradtak. Ez a végleges szöveg. Ha inkább a saját megfogalmazását tartaná meg, használja az utolsó lépés Összehasonlítás funkcióját.
import-diagnostic-text-box-dropped = { $count ->
    [one] A(z) { $path } { $count } szövegdobozt tartalmaz. A szövege kívül esik a dokumentum folyamán, így nem dönthető el, hová tartozna a kéziratban. Nem kerül importálásra.
   *[other] A(z) { $path } { $count } szövegdobozt tartalmaz. A szövegük kívül esik a dokumentum folyamán, így nem dönthető el, hová tartoznának a kéziratban. Nem kerülnek importálásra.
}
import-diagnostic-embedded-object-dropped = { $count ->
    [one] A(z) { $path } { $count } beágyazott objektumot tartalmaz: diagramot, egyenletet vagy hasonlót. A kéziratban nincs semmi, ami befogadhatná.
   *[other] A(z) { $path } { $count } beágyazott objektumot tartalmaz: diagramot, egyenletet vagy hasonlót. A kéziratban nincs semmi, ami befogadhatná őket.
}
import-diagnostic-field-flattened = { $count ->
    [one] A(z) { $path } { $count } mezőt tartalmaz: oldalszámot, kereszthivatkozást, dátumot. Megőrzi az utoljára mutatott szöveget, és többé nem frissül.
   *[other] A(z) { $path } { $count } mezőt tartalmaz: oldalszámot, kereszthivatkozást, dátumot. Mindegyik megőrzi az utoljára mutatott szöveget, és többé nem frissül.
}
import-diagnostic-unknown-style-level = A(z) { $path } a(z) „{ $detail }” stílust használja, amely címsornak látszik, de nem ad meg szintet. Ezek a bekezdések prózaként kerülnek be, nem pedig találgatott mélységben.
import-diagnostic-comment-unanchored = A(z) { $path } „{ $detail }” megjegyzését nem sikerült ahhoz a szöveghez kötni, amelyre vonatkozott. Az elemén marad, ahonnan áthelyezheti.
import-diagnostic-comment-not-carried = A(z) { $path } „{ $detail }” megjegyzése nem kerül importálásra, mert a fájlból semmi sem érkezik szövegként, amelyen megmaradhatna. Ha szüksége van rá, másolja ki a fájlból.
import-diagnostic-comment-replies-flattened = { $count ->
    [one] A(z) { $path } { $count } válasza olyan megjegyzésre hivatkozik, amely nincs a fájlban, ezért önálló megjegyzésként érkezik.
   *[other] A(z) { $path } { $count } válasza olyan megjegyzésre hivatkozik, amely nincs a fájlban, ezért önálló megjegyzésekként érkeznek.
}
import-diagnostic-prose-not-verbatim = { $count ->
    [one] A(z) { $path } { $count } bekezdését nem sikerült pontosan az eredeti formázással átvenni, ezért egyszerű szövegként érkezik. Vesse össze az eredetivel.
   *[other] A(z) { $path } { $count } bekezdését nem sikerült pontosan az eredeti formázással átvenni, ezért egyszerű szövegként érkeznek. Vesse össze őket az eredetivel.
}
import-diagnostic-styled-spaces-not-carried = { $count ->
    [one] A(z) { $path } fájlban { $count } üres szakasz alá volt húzva vagy át volt húzva, például kitöltendő vonalként. A Skribisto egyik vonalat sem tartja meg csupán szóközökön, ezért egyszerű szóközökként érkezik, bekezdés elején vagy végén pedig egyáltalán nem.
   *[other] A(z) { $path } fájlban { $count } üres szakasz alá volt húzva vagy át volt húzva, például kitöltendő vonalakként. A Skribisto egyik vonalat sem tartja meg csupán szóközökön, ezért egyszerű szóközökként érkeznek, bekezdés elején vagy végén pedig egyáltalán nem.
}
import-diagnostic-list-nesting-flattened = { $count ->
    [one] A(z) { $path } { $count } listaeleme több mint { $limit } szint mélyen volt beágyazva. A(z) { $limit }. szintre, a Skribisto által megőrzött legmélyebbre kerül, a szavaival és a formázásával együtt.
   *[other] A(z) { $path } { $count } listaeleme több mint { $limit } szint mélyen volt beágyazva. A(z) { $limit }. szintre, a Skribisto által megőrzött legmélyebbre kerülnek, a szavaikkal és a formázásukkal együtt.
}
import-diagnostic-table-read-as-paragraphs = { $count ->
    [one] A(z) { $path } { $count } táblázatának egyik sora sokkal szélesebb a többinél, túl aránytalan ahhoz, hogy rácsként lehessen elrendezni. A cellái bekezdésekként érkeznek, egymás után, a szavaikkal és a formázásukkal együtt.
   *[other] A(z) { $path } { $count } táblázatának egy-egy sora sokkal szélesebb a többinél, túl aránytalan ahhoz, hogy rácsként lehessen elrendezni. A celláik bekezdésekként érkeznek, egymás után, a szavaikkal és a formázásukkal együtt.
}
import-diagnostic-spaces-shortened = { $count ->
    [one] A(z) { $path } { $count } szóközsorozata hosszabb volt { $limit } szóköznél, ezért { $limit } szóközre rövidítve érkezik. A körülötte lévő szavak mind megvannak.
   *[other] A(z) { $path } { $count } szóközsorozata hosszabb volt { $limit } szóköznél, ezért { $limit } szóközre rövidítve érkeznek. A körülöttük lévő szavak mind megvannak.
}
import-diagnostic-epigraph-not-carried = A(z) „{ $title }” mottóval kezdődik, de egy { $kind } típusú elem nem tárolhat mottót. Az idézet ehelyett a szöveg elején marad.
import-diagnostic-epigraph-placement-ambiguous = Egy mottó áll a(z) „{ $title }” és a(z) „{ $below }” között, és bármelyiket bevezethetné. A(z) „{ $title }” kapta meg, mert a mottó általában oda tartozik.
import-document-diagnostics = { $errors ->
    [0] { $warnings ->
            [one] 1 tudnivaló
           *[other] { $warnings } tudnivaló
        }
   *[other] { $errors ->
            [one] 1 fájlt nem sikerült beolvasni
           *[other] { $errors } fájlt nem sikerült beolvasni
        }
}
import-document-diagnostics-none = Nincs mit jelenteni.
versions-title = Változatok
versions-scope-synopsis = Szinopszis
versions-scope-prose = Szöveg
versions-loading = A biztonsági másolatok átnézése…
versions-empty = Ennek még nincs korábbi változata
versions-error = Nem sikerült beolvasni a biztonsági másolatokat. Semmi sem veszett el, de ez a lista hiányos lehet
versions-did-not-exist = { $date } idején még nem létezett
versions-deleted-after = Valamikor { $date } után törölve
versions-unreadable = { $count ->
    [one] { $count } biztonsági másolatot nem sikerült beolvasni
   *[other] { $count } biztonsági másolatot nem sikerült beolvasni
}
versions-thinned = { $count ->
    [one] A régebbi változatok idővel megritkulnak. A projekt saját előzményei már elvetették ennek a szövegnek { $count } korábbi állapotát.
   *[other] A régebbi változatok idővel megritkulnak. A projekt saját előzményei már elvetették ennek a szövegnek { $count } korábbi állapotát.
}
versions-source-backup = Biztonsági másolatból
versions-source-project = A projekt saját előzményeiből
versions-list-caption = Változásonként egy bejegyzés, nem biztonsági másolatonként
versions-pick-a-version = Válasszon egy változatot, hogy lássa, mi változott
versions-earliest = A legkorábbi rögzített változat. Nincs nála régebbi, amivel össze lehetne vetni.
versions-no-change = Az elem ezen részében semmi sem változott
versions-formatting-only = Itt csak a formázás változott. A szavak ugyanazok
versions-show-unchanged = Változatlanok megjelenítése
versions-hide-unchanged = Változatlanok elrejtése
versions-next-change = Következő változás
versions-near = a(z) „{ $text }” közelében
versions-words-added = { $count ->
    [one] { $count } szó hozzáadva
   *[other] { $count } szó hozzáadva
}
versions-words-removed = { $count ->
    [one] { $count } szó eltávolítva
   *[other] { $count } szó eltávolítva
}
versions-blocks-moved = { $count ->
    [one] { $count } bekezdés áthelyezve
   *[other] { $count } bekezdés áthelyezve
}
versions-pin = A változat forrását jelentő biztonsági másolat rögzítése. Az automatikus takarítás soha nem törli
versions-unpin = A változat forrását jelentő biztonsági másolat rögzítésének feloldása. Az automatikus takarítás ismét törölheti
versions-pinned-only = Csak a rögzített változatok megjelenítése
versions-pin-note = Csak biztonsági másolatból származó változat rögzíthető. A rögzítés egy fájlt őriz meg, a projekt saját előzményei pedig nem fájl.
versions-pinned-empty = Itt még semmi sincs rögzítve
versions-pinned-empty-log = Itt még semmi sincs rögzítve. Csak biztonsági másolatból származó változat rögzíthető. A rögzítés egy fájlt őriz meg, a projekt saját előzményei pedig nem fájl.
versions-range-filter = Csak két dátum közötti változatok megjelenítése
versions-filtered-empty = Egyetlen változat sem felel meg a beállított szűrőknek
versions-clear-filters = Szűrők törlése
versions-last-30-days = Az utolsó 30 nap
versions-restore-button = A változat visszaállítása
versions-restore-confirm-title = Lecseréli ezt a szöveget a(z) { $date } időpontban rögzített változatra?
versions-restore-confirm-text = A jelenlegi szöveget az a szöveg váltja fel, amely ennél az elemnél { $date } időpontban volt.
versions-restore-confirm-with-comments = A jelenlegi szöveget az a szöveg váltja fel, amely ennél az elemnél { $date } időpontban volt. { $count ->
    [one] { $count } megjegyzés a jelenlegi szöveghez van kötve, és gazdátlanná válhat.
   *[other] { $count } megjegyzés a jelenlegi szöveghez van kötve, és gazdátlanná válhat.
}
versions-restore-confirm-undo-note = Előbb biztonsági másolat készül, és a Ctrl+Z egy lépésben visszavonja.
versions-restored-toast = A(z) { $date } időpontban rögzített változat visszaállítva
versions-undo = Visszavonás
versions-restore-row-gone = Ez az elem már nincs a projektben
versions-restore-no-home = Az elem típusa azóta megváltozott, és a régi szövegnek nincs helye benne
versions-restore-no-safety-copy = A biztonsági mentés nem futott le, így semmi sem változott
versions-restore-failed = A visszaállítás sikertelen: { $error }
versions-restore-backup-busy = Már fut egy biztonsági másolat készítése. Próbálja újra egy kicsit később
versions-restore-in-backup-file = Egy biztonsági másolat fájlját nézi; a visszaállításhoz magát a projektet nyissa meg
versions-restore-no-project = Nincs megnyitott projekt
versions-recreate-button = Visszahozás…
versions-recreate-picker-title = Hová kerüljön?
versions-recreate-picker-empty = Ebben a projektben nincs iratrendező, ahová kerülhetne
versions-recreate-picker-confirm = Visszahozás ide
versions-recreate-picker-cancel = Mégse
versions-recreate-untitled = ez az elem
versions-recreate-confirm-title = Visszahozza ezt: „{ $item }”?
versions-recreate-confirm-text = Ide kerül: { $destination }, azzal a szöveggel, amely { $date } időpontban volt benne.
versions-recreate-confirm-undo-note = Az ezután megjelenő üzenet Visszavonás gombja azonnal ki is veszi.
versions-recreated-toast = A(z) „{ $item }” ismét a projektben van
versions-recreated-partial-toast = { $count ->
    [one] A(z) „{ $item }” visszakerült, de az egyik szövegét nem sikerült beolvasni
   *[other] A(z) „{ $item }” visszakerült, de { $count } szövegét nem sikerült beolvasni
}
versions-recreate-already-here = Ez az elem már a projektben van
versions-recreate-no-destination = Válasszon egy helyet az iratrendezőben
versions-recreate-unreadable = Ezt a biztonsági másolatot nem sikerült beolvasni, így semmi sem került hozzáadásra
versions-recreate-failed = Nem sikerült visszahozni: { $error }
versions-changed-percent = { $percent }%-a megváltozott
versions-hidden-paragraphs = { $count ->
    [one] … { $count } változatlan bekezdés …
   *[other] … { $count } változatlan bekezdés …
}
timeline-title = Vissza az időben
timeline-coverage = { $count ->
    [one] { $count } rögzített változat, { $oldest } időpontig visszamenőleg
   *[other] { $count } rögzített változat, { $oldest } időpontig visszamenőleg
}
timeline-loading = A projekt múltjának átnézése…
timeline-empty = Ennek a projektnek még egyetlen változata sincs rögzítve
timeline-no-changes = Azóta semmi sem változott
timeline-no-text-changes = Azóta egyetlen szöveg sem változott
timeline-slider-label = Rögzített változat
timeline-series-name = A projekt mérete
timeline-bars-caption = Minden oszlop egy rögzített változat, olyan magas, amekkora a projekt akkor volt. A kiemelt az, amelyet éppen néz.
timeline-bars-caption-periods = Túl sok változat van ahhoz, hogy egyenként jelenjenek meg, ezért minden oszlop egy-egy időszak ({ $period }), olyan magas, amekkora a projekt a végére lett.
timeline-unit-hour = óra
timeline-unit-day = nap
timeline-unit-week = hét
timeline-unit-month = hónap
timeline-range-filter = Csak két dátum közötti változatok megjelenítése
timeline-range-empty = Ezek között a dátumok között nem rögzült változat
timeline-open-period = Az időszak megnyitása
timeline-show-all = A teljes előzmény megjelenítése
timeline-changed-since = { $count ->
    [one] { $count } elem különbözik { $date } és a projekt mostani állapota között
   *[other] { $count } elem különbözik { $date } és a projekt mostani állapota között
}
timeline-kind-added = Azóta írva
timeline-kind-removed = Már nincs a projektben
timeline-kind-changed = Azóta szerkesztve
timeline-kind-moved = Azóta áthelyezve
timeline-not-yet-written = Ez akkor még nem létezett
timeline-no-text-of-its-own = Ennek nincs saját szövege. A benne lévők címsora
timeline-reader-close = Bezárás
timeline-reader-stamp = Ahogy { $date } időpontban volt
timeline-reader-loading = A rögzített változat megnyitása…
timeline-reader-unreadable = Ezt a rögzítést nem sikerült beolvasni. Lehet, hogy a biztonsági másolatot áthelyezték, törölték, vagy nem csatlakoztatott meghajtón van.
timeline-reader-compared = Ahogy { $date } időpontban volt, összevetve a mostanival
timeline-reader-diff-legend = Az áthúzott azóta eltűnt; az aláhúzott azóta került be.
timeline-reader-view-label = Mit mutasson
timeline-reader-view-diff = Változások
timeline-reader-view-text = Szöveg
timeline-reader-deleted = Ez már nincs a projektben. Itt elolvashatja és kimásolhatja.
timeline-prose-only-record = Ez a pont a projekt saját előzményeiből származik, amely csak a szöveget őrzi meg, semmi mást, így nem mutatható meg, mi lett azóta törölve vagy áthelyezve. Ehhez válasszon egy biztonsági másolatot.
settings-sec-extensions = Bővítmények
settings-group-games-forward = Mindig előre
settings-games-forward-toggle = „Mindig előre” játék
settings-games-forward-hint =
    Játék közben semmi sem vonható vissza abból, amit leírt: a Backspace, a Delete,
    a kivágás, a húzás és ejtés, valamint a visszavonás le van tiltva az alább választott felületeken.
    Továbbra is szabadon gépelhet, beilleszthet, formázhat és mozoghat. A piszkozat csak nő.
settings-games-session-warning =
    Ez munkamenetenkénti választás: sosem kerül mentésre. A projekt bezárása vagy a
    Skribisto kilépése mindig véget vet a játéknak, és a játék közben írt szöveg
    abban a pillanatban ismét visszavonhatóvá válik.
settings-group-games-scope = Hol érvényes
settings-games-in-prose = Kéziratpróza
settings-games-in-synopsis = Szinopszisok
settings-games-scope-hint =
    A megjegyzések, a lábjegyzetek és a címek sosem fagynak be: ott jegyezheti fel azt a javítást,
    amelynek elvégzését épp most tiltotta meg magának.
settings-games-inert-warning =
    A „Mindig előre” be van kapcsolva, de semmire sem vonatkozik. Jelöljön be fent legalább egy felületet,
    különben írás közben semmin sem változtat.
games-title = Írójátékok
games-forward-name = Mindig előre
games-forward-blurb =
    Fogalmazzon anélkül, hogy bármit visszavonna. A törlés, a kivágás és a visszavonás le van tiltva
    játék közben. Írja meg a következő mondatot ahelyett, hogy az előzőt javítgatná.
games-forward-playing = Játék folyamatban: a törlés le van tiltva
games-forward-idle = Nincs játék
games-session-note = A projekt bezárásakor véget ér.
games-scope-prose-and-synopsis = A prózájára és a szinopszisaira vonatkozik.
games-scope-prose = A prózájára vonatkozik.
games-scope-synopsis = A szinopszisaira vonatkozik.
games-scope-nothing = Még semmire sem vonatkozik. Válasszon felületet a Beállításokban.
games-settings-link = Az írójátékok beállításai…
statusbar-games-forward = Mindig előre
statusbar-games-forward-tooltip =
    A „Mindig előre” be van kapcsolva: fogalmazás közben a törlés, a kivágás és a visszavonás
    le van tiltva. Kattintson a játék befejezéséhez.
goal-progress-words = { $g ->
    [one] { $count } / { $goal } szó
   *[other] { $count } / { $goal } szó
}
goal-progress-characters = { $g ->
    [one] { $count } / { $goal } karakter
   *[other] { $count } / { $goal } karakter
}
goal-count-words = { $n ->
    [one] { $count } szó
   *[other] { $count } szó
}
goal-count-characters = { $n ->
    [one] { $count } karakter
   *[other] { $count } karakter
}
goal-unit-words = Szavak
goal-unit-characters = Karakterek
inspector-goal = Célérték
inspector-goal-none = Nincs célérték
overview-col-goal = Célérték
overview-excluded-from-export = Kimarad az exportálásból, így egyetlen összesítésbe sem számít bele
goal-subtree-total-words = { $count ->
    [one] A benne lévő { $items } célérték összesen { $words } szó
   *[other] A benne lévő { $items } célérték összesen { $words } szó
}
goal-subtree-total-characters = { $count ->
    [one] A benne lévő { $items } célérték összesen { $words } karakter
   *[other] A benne lévő { $items } célérték összesen { $words } karakter
}
new-work-goal-unit = Számolás
new-work-goal-unit-hint = A projekt minden szó- vagy karaktercéljára vonatkozik. Később módosítható.
settings-group-goal-unit = Célértékek
settings-goal-unit-switch-title = Számlálási egység váltása
settings-goal-unit-switch-text = A projekthez már beállított célértékek és mérföldkövek { $from } egységben lettek megadva. A(z) { $to } egységre váltás nem alakítja át őket, így minden meglévő szám ezentúl { $to } egységként értelmeződik.
settings-goal-unit-switch-informative = Semmi sem vész el. Bármikor visszaválthat az eredeti értelmezéshez, majd frissítheti a megtartani kívánt célértékeket.
settings-goal-unit-switch-confirm = Váltás
goal-distribute-action = Elosztás…
distribute-title = A célérték elosztása
distribute-empty = Nincs benne semmi, amire a célértéket el lehetne osztani.
distribute-weight = Elosztás módja
distribute-weight-length = Hossz szerint
distribute-weight-rows = Darabszám szerint
distribute-weight-even = Egyenlően
distribute-overwrite = A már beállított célértékek felülírása
distribute-col-item = Elem
distribute-col-current = Most
distribute-col-proposed = Utána
distribute-total = Ezek összesen { $total }, a(z) { $goal } célértékkel szemben.
distribute-over-budget = A benne már beállított célértékek { $over } értékkel többet tesznek ki, mint a tároló saját célértéke. Növelje a tároló célértékét, csökkentse a bennük lévőket, vagy írja felül őket.
distribute-apply = Elosztás
distribute-cancel = Mégse
milestone-target-gone = A cél törölve
milestone-no-target = Nincs cél
milestone-add = Hozzáadás
milestone-add-label = Mit kell elérni
pace-summary-title = Hol tart a könyv
pace-summary-remaining = Még { $words } van hátra
pace-summary-open = A terv megnyitása
pace-summary-close = Bezárás
pace-summary-dont-show = Ne jelenjen meg megnyitáskor
pace-summary-menu = Írási terv…
menu-help-topics = &Súgótémák
menu-help-shortcuts = &Gyorsbillentyűk…
menu-help-website = Skribisto a &weben
menu-help-report = &Hiba bejelentése…
menu-help-check-updates = &Frissítések keresése
menu-command-palette = &Parancspaletta…
learn-help-topics = Súgótémák
learn-shortcuts = Gyorsbillentyűk
learn-website = Skribisto a weben
help-window-title = Súgó
help-filter-topics = Témák szűrése
help-no-matching-topic = Nincs egyező téma.
help-topic-missing = Ez a téma már nem érhető el.
help-back = Vissza
help-not-translated = Ez az oldal még nincs lefordítva, ezért angolul jelenik meg.
help-section-getting-started = Első lépések
help-section-writing = Írás
help-section-reviewing = Átnézés
help-section-exchanging = Szöveg be és ki
help-section-keeping = A munka biztonsága
help-section-extensions = Bővítmények
help-topic-getting-started = Az első projekt
help-topic-writing-model = Hogyan épül fel egy könyv
help-topic-drafts-and-old-versions = Egy régebbi piszkozat megőrzése
help-topic-goals-and-pace = Célértékek és tempó
help-topic-comments = Megjegyzések
help-topic-round-trip = A könyv elküldése egy olvasónak
help-topic-export = Exportálás
help-topic-import-documents = Dokumentumok importálása
help-topic-import-projects = Teljes projekt átvétele
help-topic-backups-and-versions = Biztonsági másolatok és változatok
help-shortcuts-title = Gyorsbillentyűk
help-shortcuts-filter = Gyorsbillentyűk szűrése
help-shortcuts-no-matches = Nincs egyező gyorsbillentyű.
help-shortcuts-rebind = Gyorsbillentyűk módosítása…
help-shortcuts-close = Bezárás
command-palette-placeholder = Írjon be egy parancsot
shortcut-name-binder-duplicate = Kettőzés
shortcut-name-comments-add = Megjegyzés hozzáadása
shortcut-name-comments-add-paragraph = Megjegyzés a bekezdéshez
shortcut-name-spellcheck-toggle = Helyesírás-ellenőrzés
shortcut-name-editor-tab-close = Lap bezárása
shortcut-name-editor-tab-pin = Lap rögzítése vagy feloldása
shortcut-name-editor-save = Mentés
shortcut-name-work-export = Exportálás…
shortcut-name-work-new = Új projekt
shortcut-name-work-open = Projekt megnyitása
shortcut-name-window-new = Új ablak
shortcut-name-work-close = Projekt bezárása
shortcut-name-app-settings = Beállítások
shortcut-name-app-quit = Kilépés
shortcut-name-editor-insert-footnote = Lábjegyzet beszúrása
shortcut-name-format-scene-break = Jelenettörés beszúrása
shortcut-name-format-major-scene-break = Nagy jelenettörés beszúrása
shortcut-name-format-link = Hivatkozás beszúrása
shortcut-name-format-mark-nbsp = Nem törhető szóköz beszúrása
shortcut-name-format-mark-nbhyphen = Nem törhető kötőjel beszúrása
shortcut-name-format-mark-nnbsp = Keskeny nem törhető szóköz beszúrása
shortcut-name-format-mark-zwsp = Zérus szélességű szóköz beszúrása
shortcut-name-go-next = Következő
shortcut-name-go-prev = Előző
shortcut-name-go-to = Ugrás
shortcut-name-outline-toggle = Vázlat be- és kikapcsolása
shortcut-name-preview-toggle = Előnézeti sáv be- és kikapcsolása
shortcut-name-view-fullscreen = Teljes képernyő be- és kikapcsolása
shortcut-name-view-focus-mode = Zavarmentes mód be- és kikapcsolása
shortcut-name-editor-size-increase = Szövegméret növelése
shortcut-name-editor-size-decrease = Szövegméret csökkentése
shortcut-name-editor-size-reset = Szövegméret alaphelyzetbe
shortcut-name-editor-find = Keresés
shortcut-name-editor-replace = Csere
shortcut-name-editor-find-next = Következő találat
shortcut-name-editor-find-prev = Előző találat
shortcut-name-search-show = Keresés a projektben
shortcut-name-search-replace = Csere a projektben
shortcut-name-outline-open-to-side = Megnyitás oldalt
shortcut-name-help-topics = Súgótémák
shortcut-name-help-shortcuts = Gyorsbillentyűk
shortcut-name-help-website = Skribisto a weben
shortcut-name-help-report = Hiba bejelentése
shortcut-name-help-check-updates = Frissítések keresése
update-check-behind = Elérhető a(z) { $version } verzió.
update-check-current = A legújabb verziót használja.
update-check-failed = Nem sikerült újabb verziót keresni. { $error }
shortcut-name-command-palette = Parancspaletta
help-section-reference = Fogalmak
help-concept-scene-break = Jelenettörés
help-concept-major-scene-break = Nagy jelenettörés
help-concept-find-in-prose = Keresés a prózában
help-concept-story-bible = Történetbiblia
help-concept-goal-unit = Szavak vagy karakterek
help-concept-goal-progress = Haladás
help-concept-manuscript-words = Mi számít a kézirathoz
help-concept-exportable = Kihagyás az exportálásból
help-concept-distribute = Célérték elosztása
help-concept-subtree-total = A benne lévő célértékek összege
help-concept-milestone = Mérföldkő
help-concept-pace-plan = Tempóterv
help-concept-tag = Címke
help-concept-label = Felirat
help-concept-point-of-view = Nézőpont
help-concept-epigraph = Mottó
help-concept-footnote = Lábjegyzet
help-concept-chapter-mode = Fejezetforma
help-concept-comment = Megjegyzés
help-concept-backup = Biztonsági másolat
help-concept-version = Változat
help-concept-trash = Kuka
help-concept-spellcheck = Helyesírás-ellenőrzés
help-concept-search-replace = Keresés és csere
help-concept-note-template = Jegyzetsablon
help-concept-story-bible-entry = Történetbiblia-bejegyzés
help-concept-text-replacement = Szövegcsere
help-concept-smart-punctuation = Intelligens írásjelek
help-concept-export-style = Exportstílus
help-concept-round-trip-marks = Visszaküldési jelölők
epigraph-mark-attribution = Forrássor
epigraph-mark-attribution-tip = A kurzort tartalmazó sor megjelölése az idézet forrásaként, hogy hivatkozásként jelenjen meg nyomtatásban
margin-lane-name = Margójelek
margin-lane-provider-comments = Megjegyzések
margin-lane-provider-story-bible = Történetbiblia-bejegyzés
margin-lane-provider-story-bible-hint = Egy jegyzet A prózában nézetében: ahol a bejegyzést megnevezik, és azok a jelenetek, amelyeket az ő szemével mesélnek el. A neveket magában a prózában is megjelöli.
margin-lane-mark-point-of-view = Innen elmesélve
margin-lane-mark-named-here = Itt megnevezve
margin-lane-provider-comments-hint = Ahol megjegyzés kapcsolódik a szöveghez
margin-lane-provider-search = Keresési találatok
margin-lane-provider-search-hint = A legutóbb keresett kifejezés minden találata, bárhol is legyen a dokumentumban
margin-lane-provider-boundaries = Ahol az egyes dokumentumok kezdődnek
margin-lane-provider-boundaries-hint = Egy vonal minden jelenet elején a szövegfolyamban, hogy lássa, hol tart
margin-lane-provider-spelling = Helyesírás
margin-lane-provider-spelling-hint = Ahol a helyesírás-ellenőrző megjelölt egy szót. Csak kérésre: egy gép véleményét jelöli a prózájáról
margin-lane-spelling = { $word }, lehetséges helyesírási hiba
margin-lane-search-hit = { $text }, { $index }. találat / { $total }
margin-lane-search-current = { $text }, { $index }. találat / { $total }, ez az aktuális
margin-lane-boundary = { $title } kezdete
margin-lane-boundary-untitled = egy névtelen dokumentum
settings-page-margin-lane = Margójelek
settings-desc-margin-lane = A görgetősáv melletti sáv, és amit mutat
settings-margin-lane-enabled = A margósáv megjelenítése
settings-margin-lane-enabled-hint = Ugyanaz a kapcsoló, mint a Nézet ▸ Margójelek.
settings-margin-lane-enabled-more = Sosem állítja, hogy valami hibás: megmutatja, hol vannak a dolgok, és Önre bízza, mit kezd ezzel.
settings-group-margin-lane-marks = Mit jelöl
settings-margin-lane-no-providers = Még semmi sem jelöl a sávban.
settings-group-margin-lane-texture = A párbeszéd szövete
settings-margin-lane-texture = A párbeszéd szövetének megjelenítése
settings-margin-lane-texture-hint = Bekezdésenként egy sáv: milyen hosszú, és mekkora része elhangzó beszéd.
settings-margin-lane-texture-more = Kidolgozott konvenció nélküli nyelveken nem mérhető; ott a sáv csak a hosszt mutatja.
settings-group-margin-lane-surfaces = Hol jelenik meg
settings-margin-lane-surface-editor = Szövegszerkesztő
settings-margin-lane-surface-stream = Szövegfolyamok
settings-margin-lane-surface-search-preview = Keresési előnézet
menu-margin-lane = &Margójelek
status-preset-drafting = Fogalmazás
status-preset-passes = Átdolgozási körök
status-preset-plume = Plume Creator
status-todo = Elvégzendő
status-draft = Piszkozat
status-revised = Átdolgozva
status-final = Végleges
status-outline = Vázlat
status-first-edit = 1. szerkesztés
status-second-edit = 2. szerkesztés
status-done = Kész
status-plume-draft-1 = 1. piszkozat
status-plume-draft-2 = 2. piszkozat
status-plume-draft-3 = 3. piszkozat
status-plume-edit-1 = 1. szerkesztés
status-plume-edit-2 = 2. szerkesztés
status-plume-edit-3 = 3. szerkesztés
status-plume-proofread = Korrektúrázva
status-plume-finished = Befejezve
new-work-statuses = Munkafolyamat
new-work-statuses-hint = A szakaszok, amelyeken egy jelenet végighalad. Bármikor átnevezheti, átrendezheti vagy bővítheti őket.
status-none = Nincs állapot
inspector-status = Állapot
overview-col-status = Állapot
overview-status-mixed = az alatta lévő részek eltérnek
help-concept-status = Állapot
status-completion-title = Hol tart a könyv
status-completion-headline = { $done } / { $total } jelenet kész
status-completion-empty = Még nincsenek jelenetek. Ez a kéziratot olvassa, így írás közben telik meg.
status-completion-open = Hol tart a könyv…
settings-page-statuses = Állapotok
settings-desc-statuses = A szakaszok átnevezése, átrendezése, hozzáadása és eltávolítása, amelyeken egy jelenet végighalad
settings-statuses-add = Állapot hozzáadása
settings-statuses-add-placeholder = Az új állapot neve
settings-statuses-desc =
    A szakaszok, amelyeken egy jelenet sorban végighalad, fent a legkevésbé, lent a leginkább
    kész állapottal. Ez a sorrend határozza meg az alkalmazás minden más pontján, mit jelent a „kevésbé kész”,
    ezért úgy rendezze őket, ahogy a munkafolyamata valóban zajlik.
settings-statuses-apply-preset = Előre beállított sor alkalmazása…
settings-statuses-preset-applied = { $added ->
    [one] 1 állapot hozzáadva
   *[other] { $added } állapot hozzáadva
}
settings-statuses-preset-refused = Ennek a projektnek már van állapotsora. Az előre beállított sorok csak üres sort töltenek fel. Előbb törölje a nem kívánt fokokat.
settings-statuses-duplicate = A(z) „{ $name }” már szerepel ebben a sorban
settings-statuses-added = Hozzáadva: „{ $name }”
settings-statuses-move-up = Mozgatás fel (kevésbé kész)
settings-statuses-move-down = Mozgatás le (inkább kész)
settings-statuses-details-placeholder = Mit jelent ez a szakasz (nem kötelező)
settings-statuses-delete = „{ $name }” törlése
settings-statuses-delete-in-use = { $count ->
    [one] „{ $name }” törlése: 1 elem viseli, és elveszíti az állapotát
   *[other] „{ $name }” törlése: { $count } elem viseli, és elveszítik az állapotukat
}
settings-statuses-deleted = Törölve: „{ $name }”
settings-statuses-deleted-in-use = { $count ->
    [one] Törölve: „{ $name }”. 1 elemnek már nincs állapota.
   *[other] Törölve: „{ $name }”. { $count } elemnek már nincs állapota.
}
settings-statuses-empty-title = Még nincs munkafolyamat
settings-statuses-empty-body = Az állapot azt mutatja, hol tart egy jelenet. Adjon hozzá egyet fent, vagy induljon egy előre beállított sorból.
status-category-planned = Tervezett
status-category-drafting = Fogalmazás alatt
status-category-needs-work = Átdolgozandó
status-category-revised = Átdolgozva
status-category-final = Végleges

## Long-operation progress (shared::progress_text). The backend reports phase
## codes it cannot translate; these are their labels. `progress-percent` is the
## toast body on its own, `progress-percent-with-label` with the phase after it.
progress-percent = { $percent }%
progress-percent-with-label = { $percent }% · { $label }
progress-import-opening = A projekt megnyitása…
progress-import-outline = A vázlat beolvasása…
progress-import-converting = A fejezetek és jelenetek átalakítása…
progress-import-writing = A .skrib fájl írása…
progress-import-done = Kész
progress-import-structure = A szerkezet feltérképezése…
progress-export-start = Indítás…
progress-export-writing = Írás…
progress-project-reading = A projekt beolvasása…
# Settings: the open project's section title, "Work: <its title>".
settings-sec-work-titled = Projekt: { $title }

## Project-import notices (shared::import_warnings::notices): one line of the
## Plume / Manuskript warnings list each. $file, $folder, $title, $name, $field
## and $error are data (file names, titles, a parser's own message) and stay as
## they are. $container is "single-file" or "folder"; $kind is "scene", "note",
## "synopsis" or "title".
import-notice-flattened-rich-texts = { $count ->
    [one] Egy szöveg mélyebben volt egymásba ágyazva, mint amit egy Skribisto-projekt kezelni tud; a szavai formázás nélküli, egyszerű szövegként maradtak meg.
   *[other] { $count } szöveg mélyebben volt egymásba ágyazva, mint amit egy Skribisto-projekt kezelni tud; a szavaik formázás nélküli, egyszerű szövegként maradtak meg.
}
import-notice-free-text-badges = { $count ->
    [one] Egy jelenetjelvény inkább szabad szövegnek tűnik, mint állapotok listájának, ezért az elem felirataként maradt meg, és nem lett belőle címke.
   *[other] { $count } különböző jelenetjelvény inkább szabad szövegnek tűnik, mint állapotok listájának, ezért az elemek feliratai lettek, nem pedig címkék.
}
import-notice-separator-text-dropped = A(z) „{ $name }” elválasztó szöveget tartalmazott, amelynek nincs helye egy Skribisto-projektben; a szöveg nem lett importálva.
import-notice-separator-without-scene = A(z) „{ $name }” elválasztó előtt nem volt jelenet a fejezetében, amelyhez kapcsolódhatott volna, ezért nem lett importálva.
import-notice-chapter-without-scenes = A(z) „{ $name }” fejezetben nincsenek jelenetek, ezért az elválasztói nem lettek importálva.
import-notice-rich-text-unconverted = Egy formázott szövegrészt nem sikerült átalakítani ({ $error }); nem lett importálva.
import-notice-unresolved-cross-link = A szereplők { $number }. bejegyzésére mutató hivatkozást nem lehetett követni: a bejegyzés hiányzik, vagy a kukában volt.
import-notice-read-copy = Beolvasva: a projekt { $container ->
    [single-file] egyfájlos
   *[folder] mappás
} példánya.
import-notice-read-copy-dated = Beolvasva: a projekt { $container ->
    [single-file] egyfájlos
   *[folder] mappás
} példánya (utoljára módosítva: { $day }).
import-notice-original-format = Ez a projekt a Manuskript eredeti, 2016-os formátumában van. Teljes egészében be lett olvasva; semmi sem változott benne.
import-notice-newer-single-file-beside = A(z) „{ $file }” újabb, mint ez a mappa ({ $file_day }, szemben ezzel: { $folder_day }). A Manuskript a helyén hagyja a régi mappát, amikor egy projekt egyfájlos módra vált, így ez a mappa elhagyott példány lehet; ha inkább a .msk fájlt importálja, az újabbat olvassa be.
import-notice-pickle-skipped = A(z) „{ $file }” beolvasatlan maradt. Ez egy Python-pickle fájl, és az ilyen fájl betöltése egy projektfájlból biztonsági rés volt magában a Manuskriptben (CVE-2021-35196); a benne tárolt beállítások megjelenítési beállítások, nem írás.
import-notice-file-unreadable = Nem sikerült beolvasni ezt: „{ $file }” ({ $error }), ezért kimaradt; minden más importálva lett.
import-notice-not-utf8 = A(z) „{ $file }” nem érvényes UTF-8; a beolvashatatlan karakterek le lettek cserélve, ahelyett hogy az importálás leállt volna.
import-notice-table-unreadable = Nem sikerült beolvasni ezt: „{ $file }” ({ $error }); a tartalma nem lett importálva.
import-notice-outline-missing = A(z) „{ $file }” hiányzik, így ebben a projektben nincs importálható kézirat.
import-notice-outline-unreadable = Nem sikerült beolvasni ezt: „{ $file }” ({ $error }); nem lett kézirat importálva.
import-notice-named-none-row = A(z) „{ $file }” egy névvel ellátott sorral (»{ $name }«) kezdődik ott, ahol a Manuskript az üres „none” bejegyzését tartja. Ez a sor kimaradt, így az utána következő sorok megtartják a kézirat által használt számukat.
import-notice-unknown-field = A(z) „{ $file }” olyan mezőt tartalmaz, amelyet ez az importáló nem ismer: „{ $field }”. Nem lett importálva.
import-notice-colorless-labels = { $count ->
    [one] A(z) „{ $file }” egyik sorában nincs szín. Szín nélkül lett importálva; a Manuskript egy ilyen sornál leállítja a projekt betöltését.
   *[other] A(z) „{ $file }” { $count } sorában nincs szín. Szín nélkül lettek importálva; a Manuskript egy ilyen sornál leállítja a projekt betöltését.
}
import-notice-settings-unreadable = Nem sikerült beolvasni ezt: „{ $file }” ({ $error }); a projekt nyelve nem lett importálva.
import-notice-revisions-unreadable = Nem sikerült beolvasni ezt: „{ $file }” ({ $error }); a projekt előzményei nem lettek importálva, de ez semmi mást nem érintett.
import-notice-foreign-files = { $count ->
    [one] Ebben a projektben van egy fájl, amely nem része a Manuskript formátumának, és nem lett importálva: { $files }.
   *[other] Ebben a projektben { $count } olyan fájl van, amely nem része a Manuskript formátumának, és nem lett importálva: { $files }.
}
import-notice-foreign-files-more = Ebben a projektben { $count } olyan fájl van, amely nem része a Manuskript formátumának, és nem lett importálva: { $files }, valamint { $more ->
    [one] még egy
   *[other] még { $more }
}.
import-notice-not-a-character-file = A(z) „{ $file }” nem Manuskript-szereplőfájl, ezért kimaradt.
import-notice-not-a-text-item = A(z) „{ $file }” nem Manuskript-szövegelem, ezért kimaradt a vázlatból.
import-notice-missing-blank-line = A(z) „{ $file }” fejléce után nincs üres sor. Az utolsó mezője megmaradt, bár a Manuskript elveszíti.
import-notice-folder-without-metadata = A(z) „{ $folder }” mappában nincs { $file }, ezért egyszerű mappaként lett importálva, a könyvtár nevével. A Manuskript az ilyen könyvtárat és mindent, ami benne van, kihagy.
import-notice-rows-given-ids = { $count ->
    [one] A vázlat egyik sorának nem volt saját azonosítója, ezért kapott egyet. A Manuskript az ilyen sort elveti, vagy ha mappáról van szó, leállítja a projekt betöltését.
   *[other] A vázlat { $count } sorának nem volt saját azonosítója, ezért kaptak egyet. A Manuskript az ilyen sort elveti, vagy ha mappáról van szó, leállítja a projekt betöltését.
}
import-notice-plots-unreadable = Nem sikerült beolvasni ezt: „{ $file }” ({ $error }); a cselekményszálak nem lettek importálva.
import-notice-world-unreadable = Nem sikerült beolvasni ezt: „{ $file }” ({ $error }); a világ bejegyzései nem lettek importálva.
import-notice-world-not-opml = A(z) „{ $file }” nem OPML-dokumentum; a világ bejegyzései nem lettek importálva.
import-notice-text-flattened = A(z) „{ $title }” szövege mélyebben van egymásba ágyazva, mint amit egy Skribisto-projekt kezelni tud; a szavai formázás nélküli, egyszerű szövegként maradtak meg.
import-notice-text-unconverted = A(z) „{ $title }” szövegét nem sikerült átalakítani ({ $error }); pontosan úgy maradt meg, ahogy meg volt írva.
import-notice-text-without-file-name = A(z) „{ $title }” { $kind ->
    [scene] jelenetszövegének
    [note] jegyzetszövegének
    [synopsis] szinopszisának
   *[title] címének
} nem lehetett fájlnevet adni, ezért nem lett importálva.
import-notice-unknown-label = A(z) „{ $title }” felirata { $index }, ami nem szerepel a projekt feliratlistájában. Felirat nélkül lett importálva.
import-notice-unknown-status = A(z) „{ $title }” állapota { $index }, ami nem szerepel a projekt állapotlistájában. Állapot nélkül lett importálva.
import-notice-unknown-point-of-view = A(z) „{ $title }” egy olyan szereplő szemszögéből szól, aki nem tartozik a projekt szereplői közé. A nézőpontja nem lett beállítva.
import-notice-unresolved-references = A(z) „{ $title }” szövegében lévő hivatkozásokat nem sikerült feloldani ({ $error }); a szöveg úgy maradt meg, ahogy átalakult, a hivatkozásjelölőkkel együtt.
import-notice-custom-icons = { $count ->
    [one] Egy sornak egyéni ikonja volt. A Skribisto minden sornak a típusa szerinti ikont adja, ezért az ikon nem lett importálva; a sorban semmi más nem változott.
   *[other] { $count } sornak egyéni ikonja volt. A Skribisto minden sornak a típusa szerinti ikont adja, ezért az ikonok nem lettek importálva; ezekben a sorokban semmi más nem változott.
}
import-notice-character-colors = { $count ->
    [one] Egy szereplő neve mellett szín állt. A Skribisto a címkéket színezi, nem a sorokat, ezért a szín nem lett importálva; maga a szereplő igen.
   *[other] { $count } szereplő neve mellett szín állt. A Skribisto a címkéket színezi, nem a sorokat, ezért a színek nem lettek importálva; maguk a szereplők mind igen.
}
import-notice-versions-kept = { $found ->
    [one] Ez a projekt egy korábbi változatot rögzített
   *[other] Ez a projekt { $found } korábbi változatot rögzített
}; { $kept ->
    [one] egy megmaradt
   *[other] { $kept } megmaradt
}, ugyanúgy megritkítva, ahogy a Skribisto a saját előzményeit ritkítja. A Változatok panelen találhatók.
import-notice-versions-unplaced = { $count ->
    [one] Egy korábbi változat olyan sorhoz tartozik, amely már nincs a projektben, ezért nem lehetett elhelyezni.
   *[other] { $count } korábbi változat olyan sorhoz tartozik, amely már nincs a projektben, ezért nem lehetett elhelyezni őket.
}

## Native file dialogs: the window title, and the label of the file-type filter.
dialog-open-work-title = Skribisto-projekt megnyitása
dialog-save-as-file-title = Mentés egyetlen .skrib fájlként
dialog-save-as-folder-title = Válassza ki a projekt szülőmappáját
dialog-manuskript-file-title = Válasszon egy Manuskript-projektet
dialog-manuskript-folder-title = Válasszon egy Manuskript-projektmappát
dialog-filter-skrib = Skribisto-projekt
dialog-filter-manuskript = Manuskript-projekt
dialog-filter-plume = Plume Creator-projekt
dialog-filter-hunspell-aff = Hunspell toldalékfájl (.aff)
dialog-filter-hunspell-dic = Hunspell szótár (.dic)
dialog-filter-export = Exportált fájl
# Help ▸ Keyboard shortcuts / Settings ▸ Keymap / command palette.
shortcut-name-view-margin-lane = Margójelek be- és kikapcsolása

## The built-in distraction-free themes' names (distraction_free::theme).
df-theme-paper = Papír
df-theme-sepia = Szépia
df-theme-night = Éjszaka
df-theme-high-contrast = Nagy kontraszt
df-theme-midnight = Éjfél

## Welcome ▸ Examples: the line under a bundled example's title.
welcome-example-byline = Szerző: { $author }

## Spoken by a screen reader only.
# Welcome window: the name of the section tab bar.
welcome-sections-a11y = Üdvözlőképernyő részei
# A comment's anchored span in the editor: author, plain-text preview, replies.
comments-a11y-summary-author = { $author }: { $text }
comments-a11y-summary-replies = { $text } ({ $count ->
    [one] 1 válasz
   *[other] { $count } válasz
})
comments-a11y-summary-author-replies = { $author }: { $text } ({ $count ->
    [one] 1 válasz
   *[other] { $count } válasz
})
# Margin marks beside the text: one mark, several merged into one, and the
# lane's own name when nothing more specific was given.
margin-lane-a11y-mark = { $label }, a szöveg { $percent } százalékánál
margin-lane-a11y-marks = { $label }, { $count } jel, a szöveg { $percent } százalékánál
margin-lane-a11y-fallback = { $count ->
    [one] Margójelek, 1 jel
   *[other] Margójelek, { $count } jel
}
# Export ▸ Choose: the row standing for a binder that has no name.
export-choose-unnamed-binder = Névtelen iratrendező
# The built-in export styles named in words (the national manuscript presets
# keep their own language's name).
export-style-neutral = Semleges
export-style-manuscript-shunn = Szabványos kézirat (Shunn)
export-style-ebook-clean = Letisztult e-könyv

## Dates. Put the parts in the order your language writes a short date; the
## parts arrive already zero-padded where that matters.
# Comment cards, the outline card's Created/Modified.
date-time-short = { $year }. { $month }. { $day }. { $hour }:{ $minute }
# Pace chart: a day on the axis, a milestone's date, a holiday.
pace-day-label = { $month }. { $day }.

## File sizes (shared::byte_size). $whole and $tenth are the two sides of the
## decimal mark: write your language's mark between them and its unit symbol.
byte-size-bytes = { $value } B
byte-size-kb = { $value } kB
byte-size-mb = { $whole },{ $tenth } MB
byte-size-gb = { $whole },{ $tenth } GB
# A whole percentage (sliders, the Pace card, the text-size toast).
unit-percent = { $value }%
# A whole number of pixels (slider readouts).
unit-pixels = { $value } px
# Settings ▸ Paratext: a preset saved without a `name` line. `name` is the TOML
# key the writer types and stays as it is.
settings-paratext-error-no-name = A szerkezethez kell egy `name` sor.
# Export: started with no project open (a menu raced a close).
export-error-no-project = Nincs megnyitott projekt, amelyet exportálni lehetne.

shortcut-group-work = Projekt
shortcut-group-edit = Szerkesztés
shortcut-group-view = Nézet
shortcut-group-document = Dokumentum
shortcut-group-format = Formátum
shortcut-group-go = Ugrás
shortcut-group-tools = Eszközök
shortcut-group-help = Súgó

## Manuskript import: the headings of the notes it writes (a character's
## sheet, a world entry, a plot and its steps, the project details, the
## snowflake summary ladder) and the title of a project with no name.
import-manuskript-section-paragraph-summary = Egybekezdéses összefoglaló
import-manuskript-section-full-summary = Teljes összefoglaló
import-manuskript-section-motivation = Motiváció
import-manuskript-section-epiphany = Ráébredés
import-manuskript-section-notes = Jegyzetek
import-manuskript-section-description = Leírás
import-manuskript-section-passion = Szenvedély
import-manuskript-section-result = Eredmény
import-manuskript-section-summary = Összefoglaló
import-manuskript-section-meta = Megjegyzések
import-manuskript-section-series = Sorozat
import-manuskript-section-volume = Kötet
import-manuskript-section-genre = Műfaj
import-manuskript-section-license = Licenc
import-manuskript-section-email = E-mail
# The snowflake ladder's five tellings, shortest first.
import-manuskript-section-situation = Alaphelyzet
import-manuskript-section-sentence = Egy mondatban
import-manuskript-section-paragraph = Egy bekezdésben
import-manuskript-section-page = Egy oldalon
import-manuskript-section-full = Teljes összefoglaló
import-manuskript-untitled-project = Importált Manuskript-projekt

## Plume import: the story-bible groups an old project implies, and the title
## of a project with no name.
import-plume-group-characters = Szereplők
import-plume-group-items = Tárgyak
import-plume-group-places = Helyszínek
import-plume-untitled-project = Importált Plume-projekt
