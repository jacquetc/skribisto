# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

menu-work = Projekt (&W)
menu-new-work = Új projekt (&N)
menu-open-work = Projekt megnyitása… (&O)
menu-new-window = Új ablak (&W)
menu-import-from = Import (&I)
menu-create-from = Létrehozás (&C)
menu-import-plume = Plume Creator (.plume)… (&P)
menu-import-document = Dokumentumok (Markdown, Word, ODT) (&D)
menu-import-manuskript = Manuskript (.msk)… (&M)
menu-export = Exportálás (&X)
menu-export-book = Kiviteli könyv
menu-export-part = Export rész
menu-export-chapter = Exportfejezet
menu-export-scene = Exportjelenet
menu-export-note = Export megjegyzés
menu-export-paratext = Paratext exportálása
menu-export-folder = A mappa exportálása
menu-export-choose = Válasszon!
menu-export-none = Dokumentum megnyitása exportáláshoz
menu-save = Mentés (&S)
menu-save-as-file = Mentés egyetlen fájlként… (&F)
menu-save-as-folder = Mentés mappaként… (&D)
menu-backup = Biztonsági másolat most (&B)
menu-close-work = Projekt bezárása (&C)
menu-welcome = Üdvözöljük! (&L)
menu-settings = Beállítások (&E)
menu-quit = Kilépés (&Q)

menu-view = Nézet (&V)
menu-outline = Vázlat (&O)
menu-search = Keresés a projektben (&S)
menu-timeline = Menj vissza az időben. (&G)
menu-search-preview = Keresési előnézet (&P)
menu-fullscreen = Teljes képernyő (&F)
menu-focus-mode = Disztrakciómentes mód (&D)
menu-format = Formázás (&O)
menu-scene-break = Helyszíni szünet beszúrása (&K)
menu-major-scene-break = Helyezze be a nagyobb jelenet szünet (&M)

menu-go = Ugrás (&G)
menu-go-next-scene = Következő jelenet (&S)
menu-go-prev-scene = Előző jelenet (&V)
menu-go-next-chapter = Következő fejezet (&C)
menu-go-prev-chapter = Előző fejezet (&H)
menu-go-next-note = Következő jegyzet (&N)
menu-go-prev-note = Előző jegyzet (&T)

menu-tools = Eszközök (&T)
menu-help = Súgó (&H)

ctx-add = Hozzáadás (&A)
ctx-new-item = Új tétel (&N)
ctx-new-folder = Új mappa (&F)
ctx-rename = Átnevezés (&R)
ctx-duplicate = Másolat készítése (&D)
ctx-import-here = Import here (&I)
ctx-indent = Behúzás növelése (&I)
ctx-outdent = Külső (&O)
ctx-trash = Áthelyezés a kukába (&T)
ctx-open-to-side = Megnyitás az oldalra (&S)
ctx-open = Megnyitás (&O)
ctx-reveal-in-outline = Felfedés a fővonalon (&L)
ctx-move-up = Mozgatás felfelé (&U)
ctx-move-down = Mozgatás lefelé (&N)

ctx-tab-close = Bezárás (&C)
ctx-tab-close-others = Többi lap bezárása (&O)
ctx-tab-close-all = Összes lap bezárása (&L)
ctx-tab-open-to-side = Megnyitás az oldalra (&S)
ctx-tab-open-in-main = Megnyitás a főtáblán (&I)
ctx-tab-move-to-side = Oldalra (&M)
ctx-tab-move-to-main = Áthelyezés a főtáblára (&M)
ctx-tab-move-to-new-window = Áthelyezés új ablakba (&N)
ctx-tab-move-window-unsaved = Elõször mentse el ezt a projektet: egy második ablak nyílik meg egy lemezre
ctx-tab-pin = Pin ebben a lapon (&P)
ctx-tab-unpin = A lap kibontása (&U)
tab-pinned-tooltip = Benyújtva: Zárd be a többit, zárd be az összeset és hagyd nyitva.

create-book = Könyv
create-part = Rész
create-chapter = Fejezet
create-scene = Jelenet
create-note = Jegyzet
create-note-folder = Megjegyzés mappa
create-folder = Mappa
create-paratext = Paratextus
create-paratext-folder = Paratext mappa
create-book-end = Könyv vége
create-story-bible-entry = Történet bibliai bejegyzés
type-book-start = Könyvindítás
type-text = Szöveg

new-item-book = Új könyv
new-item-part = Új rész
new-item-chapter = Új fejezet
new-item-note = Új jegyzet
new-item-note-folder = Új Note mappa
new-item-folder = Új mappa
new-item-paratext = Új paratext
new-item-paratext-folder = Új Paratext mappa
new-item-story-bible-entry = Új történet Bibliai bejegyzés

placement-inside = belül
placement-after = után
placement-after-parent = az anyavállalat után
placement-top-level = felső szint


ctx-promote = Átalakítás (&C)
promote-chapter-folder = Fejezetmappa
promote-flat-chapter = Önálló fejezet
promote-lossy-title = A szöveget nem lehet megtartani.
promote-lossy-text = A { $target }-nek nincs hol tartania ezeket: { $kinds }. Mozgassa vagy törölje a szöveget először, majd konvertálni.
content-scene-text = Szöveg
content-note-text = Megjegyzés szövege
content-book-title = Könyvcím
content-book-subtitle = A könyv felirata
content-part-title = Részcím
content-chapter-title = A fejezet címe
content-epigraph-text = Mottó
content-paratext-text = Paratextus
promote-blocked-title = Nem üres fejezet
promote-blocked-text = { $count ->
    [one] Ez a fejezet még mindig rendelkezik { $count } elem. Mozgassa vagy szemét előtt konvertálni, hogy egy lapos fejezet.
   *[other] Ez a fejezet még mindig rendelkezik { $count } elemek. Mozgassa vagy roncsolja őket, mielőtt konvertálni, hogy egy lapos fejezet.
}

inspector = Tulajdonságok
inspector-empty = Nyisson ki egy tárgyat, hogy megvizsgálhassa.
inspector-promote = Átalakítás
statusbar-toggle-outline = A mappa bekapcsolása
statusbar-toggle-inspector = A felügyelő ki- be kapcsolása
statusbar-save-unsaved = Nem elmentett változtatások. Kattintson a mentéshez
statusbar-save-saved = Minden módosítás elmentve
statusbar-save-autosave = Az automatikus mentés be van kapcsolva. A módosítások elmentve vannak az írás közben
statusbar-saving = Mentés
statusbar-word-count = { $count ->
    [one] { $count } szó
   *[other] { $count } szó
}
statusbar-word-count-tooltip = Words in the scene you 're educed
statusbar-word-char-count = { $words ->
    [one] { $words } szó
   *[other] { $words } szó
} · { $chars ->
    [one] { $chars } karakter
   *[other] { $chars } karakter
}
session-toggle = Írási munkamenet: fókuszált sprint indítása vagy leállítása
session-configure = A munkamenet céljának és időkorlátjának beállítása
session-configure-title = Írási ülés
session-word-goal = Szócél
session-time-limit = Határidő
session-time-limit-unit = perc
session-no-goal = Nincs cél
session-no-limit = Nincs limit
session-reset = A munkamenet újraindítása
session-readout = { $words } szavak · { $time }
session-readout-timed = { $words } Szavak { $time } balra
statusbar-focus-exit = Kilépési elterelés-mentes mód
statusbar-focus-synopsis = Szinopszis
statusbar-focus-go-prev = Előző tétel (Alt + Fel)
statusbar-focus-go-next = Következő napirendi pont (Alt + lefelé)

rail-settings = Beállítások

language = Nyelv
theme = Téma
english = Angol
french = Francia
light = Fény
dark = Sötét
settings-text-width = Szövegszélesség
settings-preview-width = Keresés előnézeti szélessége
settings-autosave = Automatikus mentés a lemezre
settings-show-welcome = A kilövő megjelenítése indításkor
settings-show-welcome-tip = Ha nincs, az utolsó projekt újra megnyílik helyette.
settings-check-for-updates = Új verziók keresése
settings-check-for-updates-tip = Egy nap legfeljebb, Skribisto megkérdezi a projekt website, amely verzió aktuális. A kérés nem tartalmaz azonosítót, és nem is az a verzió fut: az összehasonlítás történik ezen a számítógépen.

settings-title = Beállítások
settings-close = Bezárás
settings-search = Keresési beállítások
settings-reset = Alapértelmezés
settings-done = Kész
settings-cancel = Mégse
settings-apply = Alkalmazás
settings-ok = Oké.
settings-reset-confirm-title = Minden beállítás alapértelmezésbe állítása?
settings-reset-confirm-body = Helyreállítja ezt az alkalmazást a megjelenés, szerkesztő és írás beállítások. Az Ön projektje saját beállítások, billentyűzet rövidítések és mentett stílusok nem befolyásolják.
settings-empty-title = Még nincs beállítás
settings-empty-hint = Ez a szakasz egy későbbi frissítésben nyeri meg a lehetőségeket.

settings-sec-appearance-behaviour = Megjelenés viselkedés
settings-sec-editor = Szerkesztő
settings-sec-spelling = Betűzés
settings-sec-backup = Erősítés szinkronizálás
settings-sec-compile = Compile Export
settings-page-appearance = Megjelenés
settings-field-image-size-policy = Nagy képek
settings-image-policy-ask = Kérdezz minden alkalommal.
settings-image-policy-keep = Eredeti megtartása
settings-image-policy-downscale = Optimalizálás
settings-hint-image-size-policy = Mit kell tenni, ha egy beépített kép nagyobb, mint a kézirat szüksége.
settings-page-notifications = Értesítések
settings-page-scene = Jelenet
settings-page-synopsis = Szinopszis
settings-page-notes = Jegyzetek
settings-page-editor-behavior = Behavior szerkesztő
settings-page-goals = Célok Word gróf
settings-page-games = Írás játékok
settings-page-corkboard = Parafatábla
settings-page-distraction-free = Disztrakció-mentes
settings-page-dictionaries = Szótár
settings-page-autosave = Automatikus mentés
settings-page-export = Exportformátumok
settings-page-paratext = Paratext struktúrák
settings-page-keymap = Kulcstérkép
settings-keymap-filter = Szűrőnyeg

settings-desc-sec-appearance-behaviour = Hogyan néz ki az alkalmazás, és mit csinál, amikor kezdődik.
settings-desc-sec-editor = Az írás felülete: hogyan néz ki, és mit csinál, ahogy gépel.
settings-desc-sec-spelling = Spell- ellenőrzés, és a szótárak mögötte.
settings-desc-sec-backup = Hogyan jut el a Projekt a lemezhez, és milyen másolatokat tárolnak.
settings-desc-sec-compile = Mi marad Skribisto, és milyen formában.
settings-desc-sec-work = Beállítások, amelyek a projekthez tartoznak, és a fájlban utaznak.
settings-desc-sec-extensions = Oldalak hozzájárultak a telepített kiterjesztések.
settings-desc-group-typography = Szövegenként egy oldal: betűtípus, méret, vonalmagasság és távolság.
settings-desc-appearance = Interfész nyelv, téma, szövegméret, és a kilövő indításkor.
settings-desc-notifications = Minden üzenet, amit ez a munkamenet mutatott, és a cselekvések, amiket vissza tudsz adni.
settings-desc-scene = Hogyan van beállítva a jelenet próza: betűtípus, méret, vonal magassága, francia és a távolság.
settings-desc-synopsis = Hogyan állítják be a szinopszist, a kézirattól függetlenül.
settings-desc-notes = Hogyan rögzítik a jegyzeteket, függetlenül a kézirattól.
settings-desc-corkboard = Kártyaméret, amit egy kártya mutat, és hogyan szervezi meg őket a bizottság.
settings-desc-distraction-free = Typography, oszlop szélessége és vezérlő szalag teljes képernyős írás.
settings-desc-distraction-free-themes = A téma könyvtár teljes képernyős írás: a szállított és a saját.
settings-desc-editor-behavior = Szövegszélesség, szinopszis pozíció, írógép görgetés és a gondozási kiemelés.
settings-desc-punctuation = A tipográfiai ház stílusú új projektek indulnak. Minden projekt követi ezt, hacsak nem állítja be a saját.
settings-desc-goals = A szavak és karakterek célpontjai, és hogy hogyan számolják a szavakat.
settings-desc-games = Kényszerítések, amiket a szerkesztés közben tett, mint például a "Mindig előre".
settings-desc-spellcheck = Az egyetlen kapcsoló, ami ki- és bekapcsolja mindenhol.
settings-desc-dictionaries = Telepítse, távolítsa el és böngészze a helyesírási szótárak ezen a gépen.
settings-desc-autosave = Hogy a szerkentyűid saját lemezre vannak-e írva.
settings-desc-backup = Amikor másolatokat vesznek, hol tartják őket, és hány. Minden projekt követi ezt, hacsak nem állítja meg a saját.
settings-desc-export = A stílus minden export összeáll: a szállított és a saját.
settings-desc-paratext = Az eleje és a hátulja egy új projekttel kezdődhet.
settings-desc-keymap = Minden rövidebb utat, és azt is, hogy mi köze hozzá.
settings-page-user = Felhasználó
settings-desc-user = Hogy ki vagy, a megjegyzéseidért, amiket írsz.
settings-group-identity = Azonosító
settings-field-user-name = Az Ön neve
settings-field-user-name-placeholder = Választható
settings-field-user-name-hint = Jelölje meg a hozzászólásokat és a válaszokat, amiket megír. Távolság egy projekttől, a szerző nevétől, ami a könyv byline-ja, és az aktában utazik.
settings-field-user-initials = A monogramod.
settings-field-user-initials-placeholder = A nevedtől
settings-field-user-initials-hint = Mit mutat egy szó processzor mellett a megjegyzést a margón. Hagyja üresen, hogy használja a látható, vette a nevét.
settings-field-user-hint = Mindkét választható, és alkalmazni minden projekt ezen a számítógépen. Megváltoztatása őket aláírja a hozzászólásokat, hogy írja le a következő. Hozzászólások már megírt tartsa meg a nevet, hogy írták alá.
settings-desc-author = A név, ami ebben a projektben szerepel.
settings-desc-structure = A projekt fejezetei mappák vagy egyes elemek.
settings-desc-language = Az a nyelv, amivel ez a projekt szemben áll.
settings-desc-work-backup = Ez a projekt saját biztonsági politika, vagy a tábornok.
settings-desc-personal-dictionary = A szavak, amiket ez a projekt helyesen ír.
settings-desc-tags = A színkódos címkék ezzel a projekttel jelölik a termékeit.
settings-desc-templates = A megjegyzés sablonokat lehet beszúrni írás közben ebben a projektben.
settings-desc-text-replacements = Rövidítések, amelyek bővülnek, ahogy gépel ebben a projektben.
settings-desc-work-punctuation = A projekt idézetei, részletei és részletei következnek.

settings-group-typography = Tipográfia
settings-group-writing-column = Írási oszlop
settings-group-writing-view = Írási nézet
settings-group-distraction-free-strip = Vezérlőszalag
settings-group-theme = Téma
settings-group-language = Nyelv
settings-group-startup = Indítás
settings-group-updates = Frissítések
settings-group-autosave = Automatikus mentés
settings-field-typeface = Betűtípus
settings-field-size = Szövegméret

editor-size-changed-manuscript = Kézirat szöveg mérete: { $percent }
editor-size-changed-synopsis = Szöveg mérete: { $percent }
editor-size-changed-notes = Megjegyzések szöveges mérete: { $percent }
editor-size-changed-corkboard = Fedélzeti kártya mérete: { $percent }
editor-size-changed-corkboard-expanded = Bővített szövegméret: { $percent }
editor-size-changed-distraction-free = Terjesztésmentes szövegméret: { $percent }

menu-text-size-increase = Szövegméret növelése (&I)
menu-text-size-decrease = Szövegméret csökkentése (&C)
menu-text-size-reset = Szövegméret újraindítása (&R)
settings-field-line-height = Vonalmagasság
settings-field-first-line-indent = Első vonalbeli francia bekezdés
settings-field-paragraph-spacing-before = Tér a bekezdés előtt
settings-field-paragraph-spacing-after = A bekezdés utáni szóköz
settings-field-column-width = Oszlop szélessége
settings-distraction-free-width-hint = Csak akkor alkalmazható, ha a disztrakciómentes üzemmód be van kapcsolva. A jelenet, a szinopszis és a jegyzetek oszlopszélessége érintetlen.
settings-distraction-free-title = A tétel megtartása
settings-distraction-free-word-count = Tartsd számon a szót.
settings-distraction-free-session = Az írói ülés megtartása
settings-distraction-free-go-to = Tartsa meg a Go to Gombbal
settings-distraction-free-go = Tartsa meg az Előző és Következő gombokat
settings-distraction-free-chrome-hint = A Kilépés gomb mindig marad, bármit is választasz itt. Ez a kivezető út, ha Escape foglalt.
settings-field-app-theme = Téma
settings-theme-light = Fény
settings-theme-dark = Sötét
settings-field-text-scale = Interfész szövegmérete
settings-field-language = Interfészek nyelve
settings-synopsis-placement = Synopsis pozíció
settings-synopsis-placement-none = Nincs
settings-synopsis-placement-top = Felül
settings-synopsis-placement-side = Mellette
settings-typewriter = Gépírógörgetés
settings-typewriter-tip = Tartja a sort írsz egy fix magasságban, míg a kézirat tekercs alá. Clicking még mindig helyezi a gondozó, ahol kattintasz.
settings-typewriter-position = Vonal helyzete
settings-typewriter-position-top-third = A harmadik
settings-typewriter-position-middle = Középső
settings-typewriter-position-bottom-quarter = Alsó negyedév
settings-highlight-scope = Kiemelés a gondozó körül
settings-highlight-scope-none = Nincs
settings-highlight-scope-sentence = Ítélet
settings-highlight-scope-paragraph = Bekezdés
settings-highlight-scope-tip-none = Semmi sincs árnyékolva, ahogy írsz.
settings-highlight-scope-tip-sentence = Shade a mondat, amit írsz, így kitűnik a körülötte lévők közül.
settings-highlight-scope-tip-paragraph = Shade az egész bekezdés, amit írsz, hogy megtartsa az átjárót, amin dolgozol.
settings-group-container-views = Konténerpanelek
settings-remember-view = Ne feledje, hogy az utolsó nézet minden egyes tételtípus
settings-remember-view-tip = Konténer kinyitása az ehhez a típushoz utoljára használt nézetre
settings-remember-view-tip-more =
    A könyv, rész és fejezet minden kínál több nézetet (saját oldal, a teljes
    kézirat, a teljes szinopszis). Kapcsolja be ezt, és minden típus újra megnyitja a
    megtekintés utoljára választott rá, pl. kapcsoljon egy fejezetet a Full Chapter és a
    Következő fejezet megnyitása mutatja Full Chapter is. Minden típus emlékszik a saját nézetét.
settings-group-counting = Szövegszámlálás
settings-counting-auto = Automatikus (nyelv szerint)
settings-counting-whitespace = Tereken feloszlás
settings-counting-unicode-words = Unicode szavak
settings-counting-cjk-hybrid = CJK- aware (karakterenként)
settings-counting-hint = Automatikus számít kínai és japán karakter és minden más nyelv a szó. Változtassa meg csak akkor, ha a gróf néz rossz a nyelvet.
settings-group-goals-display = Megjelenítés
settings-show-characters = A karakterszám megjelenítése az állapotsorban
settings-autosave-hint = A változások automatikusan a lemezre vannak írva, ahogy írod.

settings-sec-work = Projekt
settings-page-structure = Szerkezet
settings-page-author = Szerző
settings-field-author-name = Szerző neve
settings-field-author-placeholder = Választható
settings-field-author-hint = Megjelenik az összeállított címoldalon és az exportált fájl metaadatok. Hagyja üresen, hogy hagyja ki. Ez az a könyv, a byline.
settings-group-chapters = Fejezet
tidy-titles-title = Tidy fejezetcímek
tidy-titles-none = Egyetlen fejezet vagy részben cím sem ismétli meg a saját számát.
tidy-titles-lead = { $count ->
        [one] Egy fejezet vagy egy rész címe nem más, mint a saját száma.
       *[other] { $count } fejezetek és részek címe nem más, mint a saját száma.
    }
tidy-titles-explain = A címek törlésével minden egyes nevet a könyv által már ismert szám, ugyanaz a szám, mint az export nyomatok. Semmi más nem változik, és az egyik visszavonja azokat vissza.
menu-document-tidy-titles = Tidy fejezetcímek (&Y)
settings-group-numbering = Számozás
settings-number-chapters = Szám fejezetek és alkatrészek
settings-number-chapters-tip = Az árucikkeket és alkatrészeket a könyvben szereplő helyekről készítik, a címük mellett bemutatják, és az export nyomtatja.
settings-number-chapters-tip-more = A szám soha nem tárolja a cím, így marad jobbra, amikor rendel, beszúrja vagy törli. Kapcsolja ki ezt, és az export nyomatok címek egyedül, függetlenül attól, hogy az export stílus kéri. Ahhoz, hogy egy fejezetet ki (egy prologue, egy interlude), használja a felügyelő szám kapcsoló helyett: ami megtartja a könyvben, de megakadályozza, hogy vegyen egy számot.
settings-part-resets-chapter = A fejezetszámok újraindítása minden résznél
settings-part-resets-chapter-tip = Alapértelmezés szerint: a fejezetek egyenesen egy könyv részein futnak, így a második rész a 11-es fejezeten nyílik.
settings-part-resets-chapter-tip-more = Ez a szokásos kereskedelmi gyakorlat, és amit az olvasó vár. Kapcsolja be egy könyv, amelynek részei arra szolgálnak, hogy olvassa el, mint külön köteteket, ahol mindegyik nyit az első fejezetben.
settings-chapter-flat = Síkhengerelt, legalább 600 mm szélességű termék vasból vagy ötvözetlen acélból, melegen hengerelve, plattírozás, lemezelés vagy bevonás nélkül
settings-chapter-flat-hint = On: egy fejezet egy tétel. Írsz bele, és nem tárol jeleneteket. Off: egy fejezet egy mappa. Még mindig ír bele, de azt is tartja jelenetek is. Új fejezetek követik ezt a beállítást; a meglévők átalakulnak Promote.

settings-styles-builtin = Beépített stílusok
settings-styles-user = A stílusom.
settings-styles-builtin-badge = Beépített
settings-styles-duplicate = Másolat készítése
settings-styles-edit = Szerkesztés
settings-styles-delete = Törlés
settings-styles-import = Importálás…
settings-styles-export = Exportálás…
settings-styles-copy-suffix = (másolat)
settings-styles-json-filter = Exportstílus
settings-styles-editor-title = A stílus szerkesztése
settings-styles-editor-none = Válasszon egy egyéni stílust a szerkesztéséhez, vagy másolja meg az építményt az egyikben.
settings-styles-imported = Behozott stílus
settings-styles-import-failed = Nem sikerült importálni a stílust
settings-styles-exported = Kivitt stílus
settings-styles-export-failed = Nem sikerült exportálni
settings-styles-field-name = Név
settings-styles-field-chapters = Árucsoportok
settings-styles-field-parts = Alcímek
settings-styles-field-scene-break = Jelenettörés
settings-styles-field-major-scene-break = Nagy jelenet kitörése
settings-styles-field-spacing = Vonaltávolság
settings-styles-field-justify = A szöveg indoklása
settings-styles-field-notes = Megjegyzés
settings-styles-field-synopses = Szinkronokat is tartalmaz
settings-styles-field-scene-titles = Helyszíni címek beillesztése
settings-styles-field-epigraphs = Magában foglalja az epiglicerint
settings-styles-field-paratexts = Tartalmazza a paratexteket
settings-styles-field-epigraph-placement = Epigraph pozíció
settings-styles-epigraph-after = A cím után
settings-styles-epigraph-before = A cím előtt
settings-styles-field-footnotes = A lábjegyzeteket is tartalmazza
settings-styles-field-footnote-numbering = A lábjegyzetszámozás
settings-styles-footnote-numbering-continuous = Folyamatos
settings-styles-footnote-numbering-per-chapter = Minden fejezet újraindítása
settings-styles-footnote-numbering-per-book = Minden könyv újraindítása
settings-styles-field-images = Képek
menu-image = Kép (&I)
settings-styles-images-beside = A dokumentum mellett
settings-styles-images-embed = A dokumentumban
settings-styles-images-omit = Hagyj ki!
settings-styles-group-round-trip = Küldés szerkesztőnek
settings-styles-field-comments = Megjegyzések beillesztése
settings-styles-field-round-trip-marks = Tartalmazza a round-trip markerek
settings-styles-round-trip-hint = Csak DOCX és ODT. Jelölők láthatatlan azonosítók, amelyek lehetővé teszik a visszatérő fájl frissíti ezt a projektet, ahelyett, hogy leszáll mellé, mint egy második másolat.
settings-styles-group-pages = Oldalak
settings-styles-field-cover = A borító kinyitása
settings-styles-field-word-count = Szavak száma a címoldalon
settings-styles-field-page-books = Új oldal minden könyvben
settings-styles-field-page-parts = Új oldal minden résznél
settings-styles-field-page-chapters = Új oldal minden fejezetben
settings-styles-field-page-paratexts = Új oldal minden paratextnél
settings-styles-heading-none = Nincs vámtarifaszám
settings-styles-heading-numbered = Csak szám
settings-styles-heading-title = Kizárólag cím
settings-styles-heading-both = Szám + cím
settings-styles-break-blank = Üres vonal
settings-styles-break-none = Nincs
settings-styles-spacing-single = Egyedi
settings-styles-spacing-onehalf = 1½
settings-styles-spacing-double = Dupla

welcome-title = Üdvözöljük a Skribisto-nél.
welcome-version = Változat { $version }
welcome-update-available = A { $version } verzió elérhető
welcome-update-tip = Megnyitja a letöltési oldalt a böngészőben.
welcome-search = Keresési projektek
welcome-open = Megnyitás
welcome-new-work = Új projekt
welcome-create-from = Létrehozás
welcome-recent-works = A legutóbbi projektek
welcome-empty-recents = Még nincs új projekt.
welcome-no-matches = A legutóbbi projekt nem egyezik a keresésével.
welcome-learn-soon = Mindjárt jönnek a tippek és az útmutatók.
welcome-about-blurb = Skribisto, a Rust + Teksilo átírása az írási app.
welcome-tagline = *Egy csendes hely, ahol hosszú dolgokat írhatsz.*
welcome-github = GitHub
welcome-discord = Discord
nav-works = Projektek
nav-examples = Példák
nav-learn = Tanulj!
nav-about = Körülbelül

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
    [one] { $count } tétel
   *[other] { $count } tétel
}
corkboard-view-nested = Nedvesség
corkboard-view-flat = Lapos
corkboard-layout-hint = Nested mutatja a konténer közvetlen gyermekek. Nyisd ki a mappa kártyát, hogy menjen be. Lapos mutatja minden jelenet az egész tartály egyszerre.
corkboard-card-size = Kártyaméret
corkboard-search-placeholder = Szűrőkártyák
corkboard-empty-title = Még nincs itt semmi.
corkboard-empty-hint = Az első hozzáadásához használja a fenti new yardot.
corkboard-new = Új
corkboard-show-card-numbers = A kártyák száma
corkboard-modal-size = Kiterjesztett szerkesztőméret
corkboard-scope-hint = Minden nyitott rekesztáblára vonatkozik, minden projektben.
corkboard-sort-manuscript = Kézírási sorrend
corkboard-sort-title-asc = A. cím - Z. cím
corkboard-sort-title-desc = Z. cím -
corkboard-card-number = Kártya { $number }

duplicate = Másolat készítése
duplicate-n = { $count ->
    [one] Duplicate { $count } kártya
   *[other] Duplicate { $count } kártyák
}
set-label-n = { $count ->
    [one] Címke beállítása a { $count } kártyán
   *[other] Címke beállítása a { $count } kártyán
}
move-to-trash-n = { $count ->
    [one] Mozgassa a { $count } kártyát a szemétbe
   *[other] Mozgassa a { $count } kártyát a szemétbe
}
reveal-in-outline = Felfedés vázlatban

corkboard-move-to = Áthelyezés
corkboard-move-to-n = { $count ->
    [one] Mozgassa a { $count } kártyát
   *[other] Mozgassa a { $count } kártyákat
}
corkboard-move-picker-title = Áthelyezés
corkboard-move-picker-empty = Még nincs irattartó, először készíts egyet.
corkboard-move-picker-cancel = Mégse
corkboard-move-here = Mozgás!
corkboard-moved-ok = { $count ->
    [one] { $count } kártya áthelyezve
   *[other] { $count } mozgatott kártya
}
corkboard-move-into-self = A konténert nem lehet belül mozgatni. Válasszon egy célállomást azon kívül.
corkboard-move-failed = Azokat a kártyákat nem lehetett odavinni.

overview-col-title = Cím
overview-col-type = Típus
overview-col-label = Rövid leírás
overview-col-tags = Címkék
overview-col-books = Könyvek
overview-col-own-words = Szavak
overview-col-total-words = Összesen
overview-row-count = { $count ->
    [one] { $count } tétel
   *[other] { $count } tétel
}
overview-search-placeholder = Szűrőelemek
overview-expand-all = Az összes kiterjesztés
overview-collapse-all = Ütközés
overview-table-label = Tartalom
overview-empty-title = Még nincs itt semmi.
overview-gone-title = Ez a konténer eltűnt.
overview-gone-hint = Áthelyezték a szemétbe, vagy visszaállítják, vagy bezárják a számlát.
overview-empty-hint = Az első hozzáadásához használja a fenti new yardot.
corkboard-grid-label = Kártya
corkboard-rename-field = A tétel átnevezése
corkboard-expand-synopsis = Szinopszis kibontása
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
corkboard-show-word-count = A szavak számítása a kártyákon
text-heading = Szöveg
no-content = Ez a tétel nincs szerkeszthető tartalom.
untitled = Név nélkül
placeholder-title = Cím…
placeholder-subtitle = Alcím
placeholder-chapter-title = A fejezet címe
split-editor = Split szerkesztő
close-split-view = Osztott nézet bezárása
drop-open-here = Nyisd ki itt!
drop-open-to-side = Oldalra

segment-chapter = Fejezet
segment-part = Rész
segment-book = Könyv
segment-pace = Írási ütem
segment-notes = Jegyzetek
segment-story-bible = Történetlexikon
segment-note-own = Jegyzet
segment-note-details = Részletek
segment-note-in-prose = A prózában
pace-placeholder = A Pace tervező itt jelenik meg.
pace-empty-title = Tervezze meg ezt a könyvet a tempó
pace-empty-body = Állítsa be a szó cél és a határidő, és Skribisto Projektek ki a napi tempót, hogy oda.
pace-start-planning = Tervezés megkezdése
pace-section-schedule = Menetrend
pace-section-progress = Haladás
pace-advancement = Előrelépés
pace-goal = Szócél
pace-deadline = Határidő
pace-active = Pace aktív
pace-day-mon = H
pace-day-tue = Kakukk
pace-day-wed = Sze
pace-day-thu = Cs
pace-day-fri = Fri.
pace-day-sat = Szaturáció
pace-day-sun = Nap
pace-card-written = szavak
pace-card-of-goal = A cél
pace-card-rate = szavak / írás napja
pace-card-days-left = Írás napja maradt
pace-card-streak = nap csík
pace-card-ahead = szavak a tempó előtt
pace-card-behind = szavak a tempó mögött
pace-charts-empty = Haladási diagramok jelennek meg itt, ha a szavak száma rögzítik a mentés.
pace-chart-progression = Szavak megírva a cél ellen
pace-chart-words-per-day = Szavak naponta
pace-series-actual = Tényleges
pace-series-target = Cél
pace-series-words-per-day = Szavak / nap
pace-daily-target-line = Egyenletes tempó: { $count ->
    [one] { $count } szó / nap
   *[other] { $count } szó / nap
}
pace-section-holidays = Ünnepek
pace-section-milestones = Mérföldkövek
pace-holidays-none = Nincs ünnep, minden nap számít.
pace-holiday-label = Nyaralónév
pace-add-holiday = Hozzáadás
pace-remove = Eltávolítás
pace-milestones-none = Még nincs mérföldkő, állítson fel egy részt vagy fejezetet a felügyelőben.
full-chapter = Teljes fejezet
full-part = Teljes rész
full-book = Teljes könyv
full-synopsis = Teljes szinopszis
rename = Átnevezés
set-label = Címke beállítása
insert-scene = Helyszín beszúrása
insert-chapter = Fejezet beszúrása
split-scene = Jelenet felosztása
move-up = Mozgatás felfelé
move-down = Mozgatás lefelé
merge-with-previous = Egyesülés a korábbi
move-to-trash = Áthelyezés a kukába
rename-chapter = Címsor átnevezése
add-scene = Új jelenet hozzáadása
add-chapter = Új fejezet hozzáadása
new-scene-title = Új jelenet
placeholder-scene-name = Helyszín neve
menu-cut = Kivágás
menu-copy = Másolás
menu-paste = Beillesztés
menu-paste-unformatted = Beillesztés formázás nélkül
menu-select-all = Összes kijelölése
format-bold = Merész
format-italic = Olasz
format-underline = Aláhúzott
format-strikethrough = Áthúzott

binder = Dokumentumfa
no-work = Nincs betöltve projekt
no-recent-works = Nincs új projekt
switcher-open-section = Jelenleg nyitva
switcher-recent-section = Nemrég
switcher-this-window = Ez az ablak
open-project-title = Nyílt projekt
open-project-question = Hogy akarod megnyitni a { $title }-et?
open-in-new-window = Új ablakban nyílik meg
open-here = Nyisd ki itt!

binder-all = Összes kötőanyag
binder-show-all = Az összes mappa megjelenítése
binder-new = Új mappa
binder-default-name = Új mappa
binder-item-count = { $count ->
    [one] { $count } tétel
   *[other] { $count } tétel
}
binder-search-placeholder = Szűrjük át a vázlatot
binder-search-scope = Keresés az összes kötőanyag
binder-trash-confirm-title = A szemétbe rakjam a mappát?
binder-trash-confirm-text = A { $name } és minden holmija a szemétbe kerül.

dialog-rename = Átnevezés
dialog-new-binder = Új mappa
dialog-set-label = Címke beállítása
dialog-new-scene = Új jelenet
close-work-question = Menti a módosításokat a projekt bezárása előtt?
quit-question = Menti a módosításokat kilépés előtt?
quit-save-work-question = A { $title }-es változtatások elmentése a kilépés előtt?
unsaved-changes = A projekt nem mentett módosításokat tartalmaz.
new-work-unsaved-question = Változások mentése egy új projekt létrehozása előtt?
open-work-unsaved-question = A módosítások elmentése egy másik projekt megnyitása előtt?
switch-backup-discard-title = Megsemmisíteni a biztonsági mentést?
switch-backup-discard-text = A biztonsági mentés módosítása elmenthető. A mentés vagy visszaállítás használata megtartja őket, vagy eldobja őket, és megnyitja a másik projektet.
switch-save-failed = A projektet meg lehetett menteni, így felváltották: { $error }
switch-save-not-started = A projektet meg lehetett menteni, ezért kicserélték.
close-save-failed = A projektet meg lehetett menteni, így zárva volt: { $error }
close-save-not-started = A projektet meg lehetett menteni, így bezárták.
save-not-started = A projektet meg lehet menteni.

could-not-open-work = Nem sikerült megnyitni a { $file }
could-not-open-work-too-new = A { $file }-nek új Skribisto kell.
could-not-open-work-too-new-detail = Megmentette Skribisto formátum { $written_by }; megnyitására szüksége van formátum { $requires } vagy újabb, és ez az épület támogatja akár formátum { $supported }. Frissítse Skribisto megnyitni.
could-not-open-work-importing = A { $file }-et importálással írják.
could-not-open-work-importing-detail = Egy import íródik erre a projektre, és még nem fejeződött be. Nyissa meg, ha az import elkészült: most nyílik meg, megmutatja, hogy a projekt a behozatal helyébe lép, és a következő mentés visszaírná a projektet az importra.
could-not-open-example = Nem sikerült megnyitni a példát: { $error }
could-not-open-link = Nem sikerült megnyitni a { $url }: { $error }
could-not-create-work = Nem sikerült létrehozni a projektet: { $error }
saving-as-file = Mentés { $target }-ként
saving-as-folder = Megtakarítás { $target }-ként
saved-as = Mentett { $target }
save-error = Nem sikerült megmenteni: { $error }
save-flush-failed = A mentés ment keresztül, de ezeket a dokumentumokat nem lehetett átadni neki, és még mindig mentetlen: { $titles } ({ $error })
backup-error = Nem sikerült hátramaradni: { $error }
backing-up = Zsákmány feltöltése
backup-nothing-open = Nincs olyan projekt, ami megerősítené.
backup-already-running = Az erősítés már folyamatban van.
backup-complete = Erősítés kész ({ $ok } mentett, { $skipped } már aktuális)
backup-partial = Erősítés befejezve: { $ok } mentve, { $failed ->
    [one] { $failed } rendeltetési hely sikertelen
   *[other] A { $failed } rendeltetési hely nem sikerült
}
backup-no-destination-title = Nincs mentés helye
backup-no-destination-text = A konfigurált biztonsági célpontok egyike sem érhető el (például egy külső meghajtót lehet kikapcsolni). Dugd be és próbáld újra, vagy lépj ki anélkül, hogy felerősítenéd.
backup-failed-close-title = A mentés nem sikerült.
backup-failed-close-text = Zárás előtt nem sikerült másolatot írni. Minden cél nem sikerült (a meghajtó lehet, hogy el lett távolítva, vagy lehet, hogy teljes, vagy írásvédett). Javítsa meg és próbálja újra, vagy lépjen ki anélkül, hogy hátraszólna.

backup-choice-title = A mentési fájl
backup-choice-heading = Kinyitottál egy másolatot.
backup-choice-subtitle = Ez egy pont-in-time mentés egy projekthez.
backup-choice-subtitle-dated = Erősítés { $date }.
backup-choice-body = Megnyithatja és szabadon szerkesztheti, de a változtatásokat csak a Save As-szel lehet tartani. Az eredeti projektfájl nincs érintve. Vagy visszaállíthatja ezt a projektet pontosan erre a mentésre.
backup-choice-open = Nyissák ki az erősítést!
backup-choice-restore = A projekt visszaállítása erre a pontra
backup-choice-not-a-backup = Nem, normálisan nyisd ki.
backup-banner-title = Erősítés másolat: a módosítások itt menthetők meg
backup-banner-description = A mentés és a szerkesztések egy új fájlban tarthatók, vagy az eredeti projekt visszaállítása ezzel a mentéssel.
backup-banner-restore = Visszaállítás…
backup-banner-save-as = Mentés
backup-restore-original-missing = Nem lehet megtalálni az eredeti projektet, hogy helyreállítsa. Használja Mentés As tartani ezt a biztonsági mentést, mint egy új projekt helyett.
backup-restore-close-elsewhere-title = A projekt megnyitása egy másik ablakban
backup-restore-close-elsewhere-text = A projekt, amit felújít, egy másik ablakban nyílik. Kérem, előbb zárja be, majd próbálja újra.
backup-restore-focus-window = Fókuszálj arra az ablakra!
backup-restore-confirm-title = Visszaállítanád az erősítést?
backup-restore-confirm-text = A projekt jelenlegi verzióját biztonsági tartalékként félre fogják másolni, mielőtt ezzel helyettesítenék.
backup-restore-confirm-ok = Visszaállítás
backup-restore-error = Nem sikerült visszaállítani: { $error }
backup-restored-ok = A projekt helyreállt.
backup-restored-with-safety = Az előző verzióját fenntartották { $path }-re.
close-backup-discard-title = Megsemmisíteni a biztonsági mentést?
close-backup-discard-text = A biztonsági mentés módosítása elmenthető. A Mentés mint használata megtartása, vagy eldobása és bezárása.
quit-backup-discard-title = Eldobjuk a változtatásokat és kilépünk?
quit-backup-discard-work-question = Eldobni a { $title }-et és feladni?
quit-backup-discard-text = Változások egy tartalék lehet menteni, hogy. Használja Mentés As tartani őket, vagy dobja el, és hagyja abba.
backup-nudge-text = Nincs biztonsági mentés ehhez a projekthez.
backup-nudge-action = Biztonsági mentések beállítása

menu-backups-list = Biztonsági másolatok… (&K)
backups-title = Biztonsági másolatok
backups-loading = Biztonsági mentések keresése
backups-empty = Még nincs mentés a projekthez.
backups-open = Megnyitás
backups-reveal = Felfedés
backups-delete = A mentés törlése
backups-delete-confirm-title = Töröljem az erősítést?
backups-delete-confirm-text = „{ $name }” végleg törlődik. Ez nem vonható vissza.
backups-delete-error = Nem sikerült törölni a mentést: { $error }
backups-refresh = Frissítés
backups-close = Bezárás

settings-page-backup = A mentés alapértelmezett értékei
settings-page-work-backup = Biztonsági másolatok
settings-backup-general-title = Alapértelmezett mentési beállítások
settings-backup-work-title = A projekt biztonsági mentése
settings-backup-inherit = Az általános mentési beállítások használata
settings-backup-inheriting = Ez a projekt az általános mentési beállításokat használja.
settings-backup-none-hint = Nincs automatikus mentés (minden kioldó ki van kapcsolva).
settings-backup-last = Utolsó mentés: { $date }
settings-backup-last-never = Még nincs mentés.
settings-backup-open-list = A mentések listájának megnyitása
settings-backup-triggers = Mikor hátrább?
settings-backup-on-close = Vissza a projekt lezárásakor
settings-backup-on-open = Vissza a projekt megnyitásakor
settings-backup-interval = Időnként, minden nap
settings-backup-destinations = Tartalék rendeltetési helyek
settings-backup-default-location = Alapértelmezett hely
settings-backup-usage-since = { $count ->
    [one] { $count } tartalék · { $size } · legidősebb { $oldest }
   *[other] { $count } mentés · { $size } · legrégebbi { $oldest }
}
settings-backup-usage = { $count ->
    [one] { $count } tartalék · { $size }
   *[other] { $count } mentés · { $size }
}
settings-backup-usage-empty = Még semmi sem tartott itt.
settings-backup-usage-measuring = Gránátalma
settings-backup-reveal-root = A mappa megjelenítése
settings-backup-dest-none = Nincs úti cél. Biztonsági mentés a projekt mellett.
settings-backup-dest-remove = Eltávolítás
settings-backup-dest-add = A mappa hozzáadása
settings-backup-dest-refresh = Frissítés
settings-backup-retention = Hány embert kell megtartani?
settings-backup-retention-tiered = Többrétegű
settings-backup-retention-keep-n = Az utolsó N
settings-backup-retention-tip = Hogy a régi mentések metszik.
settings-backup-retention-tip-more = Rendszeresen tart egy biztonsági mentés egy nap, naponta egy hétig, hetente egy hónapig, és egy hónap múlva, így a friss történelem marad sűrű és régi történelem vékonyabb ki.
settings-backup-gfs-hourly = Óra (utolsó 24 óra)
settings-backup-gfs-daily = Naponta (múlt héten)
settings-backup-gfs-weekly = Hetente (múlt hónapban)
settings-backup-gfs-monthly = Havi
settings-backup-keep-n = Megtartandó szám
settings-backup-min-keep = Mindig tartsd meg legalább.
settings-backup-dedup = Kihagyunk egy erősítést, ha semmi sem változott.

new-work-title = Új projekt
new-work-close = Bezárás
new-work-name = A projekt neve
new-work-name-placeholder = Név nélkül
new-work-author = Szerző
new-work-author-placeholder = Választható
new-work-format = Formázás
new-work-single-file = Egyazon fájl
new-work-bundle = Dugó
new-work-convert-later = A formátumokat később is át lehet alakítani.
new-work-location = Hely
new-work-will-create = Létrehoz
new-work-language = Alapértelmezett nyelv
new-work-language-hint = Alkalmazott új szövegek helyesírás. Minden szöveg váltható egy másik nyelv.
new-work-template = Sablon
new-work-template-none = Nincs
new-work-template-empty-novel = Üres lap
new-work-template-light-novel = Fényújítás
new-work-template-novel = Újság
new-work-template-novel-in-parts = Új részek
new-work-template-notebook = Jegyzetfüzet
new-work-cancel = Mégse
new-work-create = Projekt létrehozása
new-work-step-details = Részletek
new-work-step-language = Nyelvi struktúra
new-work-step-template = Sablon
new-work-back = Vissza
new-work-next = Következő
new-work-documents-title = Új projekt a dokumentumokból
new-work-step-import = Importálás
new-work-create-and-import = Importál létrehozása
new-work-documents-next-title = A dokumentumok jönnek a következő
new-work-documents-next-body = A projekt létrehozása nyitja meg az import varázsló rajta, ahol kiválasztja a dokumentumokat, vizsgálja felül a szerkezet Skribisto olvassa el őket, és válassza ki, hol landol. Semmi nem kerül a projektbe, amíg meg nem erősíti, hogy ott.
new-work-documents-no-template = Ez a projekt szándékosan üresen indul: sablon nélkül, így az importált dokumentumok az egyetlenek benne.
new-work-documents-chapter-scene-hint = Az import által létrehozott fejezetekre vonatkozik. A projekt beállításaiban később megváltoztathatod.
new-work-single-file-desc = Egy .skrib archívum (zip). Hordozható, könnyen menthető.
new-work-bundle-desc = Egy mappa, ami minden szöveges eszközt tartalmaz.
new-work-template-none-count = üres mappa
new-work-template-empty-novel-count = 1 fejezet
new-work-template-light-novel-count = 15 fejezet
new-work-template-novel-count = 20 fejezet
new-work-template-novel-in-parts-count = 3 rész, 24 fejezet
new-work-template-notebook-count = free- form notes
new-work-chapter-scene = Síkhengerelt, legalább 600 mm szélességű termék vasból vagy ötvözetlen acélból, melegen hengerelve, plattírozás, lemezelés vagy bevonás nélkül
new-work-chapter-scene-tip = Minden fejezet egy olyan tétel, amibe egyenesen beleírunk, jelenetek nélkül. Ezt hagyjuk ki a klasszikus elrendezéshez, ahol egy fejezet egy mappa: ebbe is egyenesen beleírunk, de a jeleneteket is megtarthatjuk.
new-work-chapter-scene-tip-more = Az egyetlen különbség az, hogy lehet-e *tartalmaz* jeleneteket. Skribisto kötőfa csak szervezeti, így mindkét rétegek összeáll, hogy ugyanaz a könyv, akkor keverjük őket szabadon, és Promote átalakítja a fejezet között két szó nélkül.
new-work-name-required = Adja meg a projekt nevét
new-work-name-invalid = Ennek a névnek nincs használható karaktere
new-work-location-required = Válassza ki a helyet
new-work-location-missing = Ez a mappa nem létezik
new-work-location-not-folder = Ez az útvonal nem egy mappa
new-work-location-readonly = Ez a mappa nem írható
new-work-target-exists = A projekt ezzel a névvel már ebben a mappában. Létrehozása a projekt kérni fogja, mielőtt helyettesíti
new-work-target-is-folder = A mappa ezzel a névvel már ezen a helyen van. Válasszon egy másik nevet
new-work-overwrite-title = A meglévő projekt helyettesítése?
new-work-overwrite-text = A { $name } már létezik ebben a mappában.
new-work-target-open-title = Ez a projekt nyitott.
new-work-target-open-text = A { $name } a Skribisto-ben nyílik meg. Egy új projekt létrehozása a helyén helyettesítené az Ön által kidolgozott projektet, és a következő mentés visszaírná a projektet az új projektre. Válasszon egy másik nevet vagy mappát, vagy zárja le először a projektet.
target-importing-title = Egy import írja ezt a projektet
target-importing-text = Az import még mindig a { $name }-et írja. Várja meg, amíg befejeződik, vagy válasszon más nevet.
import-plume-busy-title = Egy másik Plume Creator projektet is importálnak.
import-plume-busy-text = Plume Creator projekt kerül importálásra egyszerre. Várja meg, amíg a behozatal befejeződik, vagy törölje, majd indítsa el ezt újra.
import-manuskript-busy-title = Egy másik Manuskript projektet is importálnak.
import-manuskript-busy-text = Manuskript projekt kerül importálásra egyszerre. Várja meg, amíg a behozatal befejeződik, vagy törölje, majd indítsa el ezt újra.
backup-restore-importing-text = Egy import még mindig írja a { $name }, a projekt a biztonsági mentés lenne visszaállítva. Helyreállítani a biztonsági mentés, miután az import befejeződött.

new-work-manuscript = Kézirat
new-work-notes = Jegyzetek
new-work-research = Kutatás
new-work-notebook = Jegyzetfüzet
new-work-scene = Jelenet
new-work-note = Jegyzet
new-work-front-matter = Első anyag
new-work-back-matter = Hátanyag
new-work-paratext = Könyvszerkezet
new-work-paratext-none = Nincs felépítés
new-work-paratext-hint = Az eleje és a hátulja számít egy hagyomány megnyitja és bezárja a könyvet. Mozgathatja, átnevezheti vagy törölheti azt követően.
new-work-tags = Címkék
new-work-tags-none = Nincs címke
new-work-tags-hint = A kezdő paletta a jelölő karakterek, helyek és a többi. Opcionális, és minden tag lehet átnevezni, recolored vagy törölni utána.
new-work-note-templates = Megjegyzés sablonok
new-work-note-templates-none = Nincs sablon
new-work-note-templates-hint = A forma egy történet-bibliai jegyzet kezdődik. Opcionális, és minden sablon lehet szerkeszteni vagy törölni utána.
new-work-characters = Szereplők
new-work-places = Helyszínek

import-plume-title = Plume Creator projekt importálása
import-plume-close = Bezárás
import-plume-source = Plume projekt
import-plume-source-hint = Válasszon egy .plume vagy .plume_backup fájlt (bármilyen Plume Creator verzió).
import-plume-location = Rendeltetési mappa
import-plume-name = Fájl neve
import-plume-name-placeholder = A projekt neve
import-plume-will-create = Létrehoz
import-plume-trash-warning = A törölt / törölt tételek nem vándorolnak.
import-plume-cancel = Mégse
import-plume-import = Importálás
import-plume-source-required = Válassza ki a Plume projektfájlt
import-plume-source-missing = Ez a fájl nem létezik
import-plume-source-not-file = Ez az útvonal nem egy fájl.
import-plume-location-required = Válassza ki a célmappát
import-plume-location-missing = Ez a mappa nem létezik
import-plume-location-not-folder = Ez az útvonal nem egy mappa
import-plume-location-readonly = Ez a mappa nem írható
import-plume-name-required = Adja meg a fájl nevét
import-plume-name-exists = A fájl ezzel a névvel már létezik itt. Import megerősíti felülírás
import-plume-overwrite-title = A meglévő fájl cseréje?
import-plume-overwrite-text = A { $name } már létezik.
import-plume-manuscript-binder = Kézirat
import-plume-story-bible-binder = Történetlexikon
import-plume-progress-title = Plume projekt importálása
import-plume-cancel-import = Mégse
import-plume-cancelled = Importálás törölve
import-plume-done = Importált { $imported ->
    [one] { $imported } tétel.
   *[other] { $imported } darab.
 } { $skipped ->
    [one] { $skipped } roncsolt tárgy nem vándorolt.
   *[other] { $skipped } roncsolt tárgyak nem vándoroltak.
}
import-plume-open-now = Nyisd ki most!
import-plume-warnings = { $count ->
    [one] 1 dolog nem importálható pontosan
   *[other] { $count } nem lehetett pontosan importálni
}
import-plume-details = Részletek
import-plume-warnings-title = Figyelmeztetések importálása
import-plume-error-title = Nem sikerült importálni a projektet
import-plume-nested-too-deep = A projekt nem került importálásra: a { $part }-es alkatrészt több mint { $limit }-es mélységig fészkelik. A Plume Creator-as projekt a közelébe kerül, és a Skribisto-es karambolozhat.
import-plume-declares-entities = A projekt nem importálásra került: a { $part } rész XML entitásokat jelentett be, amit a Plume Creator projekt nem. A kibővítés több memóriát vehet igénybe, mint a számítógép, és összeomlik Skribisto. Semmi nem íródott.
import-plume-error-details = Részletek

import-manuskript-title = Manuskript projekt importálása
import-manuskript-close = Bezárás
import-manuskript-source = Manuskript projekt
import-manuskript-source-hint = Válassza ki a .msk fájlt vagy a projekt mappáját (bármilyen Manuskript verzió).
import-manuskript-source-file = Válassza ki a fájlt
import-manuskript-source-folder = Válassza ki a mappát
import-manuskript-location = Rendeltetési mappa
import-manuskript-name = Fájl neve
import-manuskript-name-placeholder = A projekt neve
import-manuskript-will-create = Létrehoz
import-manuskript-cancel = Mégse
import-manuskript-import = Importálás
import-manuskript-source-required = Válasszon ki egy Manuskript projektet
import-manuskript-source-missing = Ez a fájl vagy könyvtár nem létezik
import-manuskript-location-required = Válassza ki a célmappát
import-manuskript-location-missing = Ez a mappa nem létezik
import-manuskript-location-not-folder = Ez az útvonal nem egy mappa
import-manuskript-location-readonly = Ez a mappa nem írható
import-manuskript-name-required = Adja meg a fájl nevét
import-manuskript-name-exists = A fájl ezzel a névvel már létezik itt. Import megerősíti felülírás
import-manuskript-overwrite-title = A meglévő fájl cseréje?
import-manuskript-overwrite-text = A { $name } már létezik.
import-manuskript-manuscript-binder = Kézirat
import-manuskript-story-bible-binder = Történetlexikon
import-manuskript-characters-group = Szereplők
import-manuskript-world-group = Világ
import-manuskript-plots-group = Területek
import-manuskript-project-info-note = Projektinformációk
import-manuskript-summary-note = Összefoglaló
import-manuskript-importance-minor = Kisebb
import-manuskript-importance-secondary = Másodlagos
import-manuskript-importance-main = Fő
import-manuskript-progress-title = Manuskript projekt importálása
import-manuskript-cancel-import = Mégse
import-manuskript-cancelled = Importálás törölve
import-manuskript-done = { $imported ->
    [one] { $imported } termék importálása.
   *[other] { $imported } importált termék.
 } { $revisions ->
    [0] { "" }
    [one] { $revisions } korábbi verzió jött vele.
   *[other] { $revisions } korábbi verziók jöttek velük.
}
import-manuskript-open-now = Nyisd ki most!
import-manuskript-warnings = { $count ->
    [one] Egy dolog tudni erről import
   *[other] { $count } dolog tudni erről a behozatalról
}
import-manuskript-details = Részletek
import-manuskript-warnings-title = Erről a behozatalról
import-manuskript-error-title = Nem sikerült importálni a projektet
import-manuskript-nested-too-deep = A projekt nem került importálásra: a { $part }-es fájl több mint { $limit }-es mélységbe került. A Manuskript-as projekt közel van ehhez, és a Skribisto-es karambolozhat. Semmi nem volt megírva.
import-manuskript-folders-too-deep = A projekt nem került importálásra: az egyik fájl több mint { $limit } mappa mély. No Manuskript projekt jön a közelébe, és olvasó lehet összeomlik Skribisto. Semmit sem írtak.
import-manuskript-declares-entities = A projekt nem volt importálva: a fájl { $part } provide XML entities, amely nem Manuskript projekt. Kiterjesztése több memóriát, mint a számítógép, és összeomlik Skribisto. Semmi nem íródott.
import-manuskript-error-details = Részletek

import-target-open-title = Ez a projekt nyitott.
import-target-open-text = A { $name } a Skribisto-ben nyílik meg. Az importálás a projekt helyébe lépne, és a következő mentés visszaírná a projektet az importra. Válasszon egy másik fájlnevet, vagy zárja le először a projektet.
import-warnings-announcement = { $result } { $notice }.

export-title = Exportálás
export-close = Bezárás
export-scope-label = Mi?
export-format-label = Formázás
export-style-label = Stílus
export-path-label = Fájl
export-browse = Böngészés
export-show-non-exportable = Nem exportálható
export-choose-empty = Nincs olyan projekt, amiből választhatnék.
export-cancel = Mégse
export-export = Exportálás
export-save-dialog-title = Exportálás fájlba
export-section-what = Mit exportáljunk?
export-section-style = Stílus
export-section-destination = Rendeltetési hely
export-custom-selection = Egyéni kiválasztás
export-selected-count = Kiválasztott { $count }
export-chip-chapters-none = Fejezet: nincs
export-chip-chapters-numbered = fejezetek: számozott
export-chip-chapters-title = Címsorok: csak cím
export-chip-chapters-both = Alcímek: szám + cím
export-chip-scene-break-glyph = Jelenettörés: { $glyph }
export-chip-scene-break-blank = Jelenettörés: üres vonal
export-chip-scene-break-none = Jelenettörés: nincs
export-chip-major-break-glyph = Nagy törés { $glyph }
export-chip-major-break-blank = Nagy törés: üres vonal
export-chip-major-break-none = Nagy törés: nincs
export-chip-major-break-same = Mindkét törésszint egyforma
export-chip-spacing-single = Ütemezés: egyetlen
export-chip-spacing-onehalf = Üveg: 1 ½
export-chip-spacing-double = Helymeghatározás: dupla
export-chip-notes-included = Megjegyzések
export-chip-notes-excluded = Megjegyzések kizárva
export-format-docx = Szó
export-format-odt = LibreOffice
export-format-html = HTML
export-format-markdown = Markdown
export-format-djot = Djot
export-format-text = Szöveg
export-format-latex = LaTeX
export-format-epub = EPUB
export-format-pdf = PDF
export-overwrite-title = A meglévő fájl cseréje?
export-overwrite-text = A { $name } már létezik.
export-progress-title = Exportálás
export-cancelled = Exportálás törölve
export-done = Kivitel { $count ->
    [one] { $count } tétel
   *[other] { $count } tétel
}
export-open-file = Megnyitás
export-show-in-folder = A mappában jelenjen meg
export-error-title = Nem sikerült exportálni
export-error-details = Részletek

backup-progress-start = Kezdés
backup-progress-retention = A régi mentések tisztítása.
backup-progress-done = Kész
backup-progress-destination = Rendeltetési hely: { $i }, { $n }
backup-details = Részletek
backup-issues-title = Erősítéssel kapcsolatos kérdések
backup-failed-title = A mentés sikertelen
backup-complete-prune-warning = A mentés befejeződött ({ $ok } mentett, { $skipped } már aktuális). Néhány régi mentés nem távolítható el

search = Keresés
search-query-placeholder = Keresés…
search-replace-placeholder = Cserélhető
search-replace-toggle = A pótlék bekapcsolása
search-replace-all = Az összes cseréje
search-opt-case = Gyufásdoboz
search-opt-whole-word = Egész szó
search-opt-diacritics = Match akcentusok
search-scope-body = Test
search-scope-title = Cím
search-scope-synopsis = Szinopszis
search-scope-label = Rövid leírás
search-facet-book = Könyvek
search-facet-part = Alkatrész
search-facet-chapter = Fejezet
search-facet-scene = Méretek
search-facet-note = Jegyzetek
search-facet-folder = Könyvtárak
search-tip-case = Match case: kezelni felső és alsó case, mint a különböző, így a "Elena" és a "Therapelena" külön gyufa.
search-tip-whole-word = Egész szó: csak a teljes szavak egyeznek, így a macska nem található meg a kategórián belül.
search-tip-diacritics = Match acquents: kezeli a hangolt betűket, mint megkülönböztethető, így nem felel meg a hawaii.
search-tip-body = Body: kutassák át a jelenetek és jegyzetek prózáját.
search-tip-title = Cím: kutassa át a címeket a mappa elemek.
search-tip-synopsis = Synopsis: kutasson át minden egyes írási elemet, az összefoglalót.
search-tip-label = Címke: kutassa át a kötőelem címszó alatt látható rövid jegyzetet.
search-tip-comment = Megjegyzések: kutassa át a szöveget, a komment szálak és azok válaszait. Egy megjegyzés a kézirattal kapcsolatos, nem pedig annak egy részét, így van a saját kapcsolója, és a csere levelek komment egyezik jelöletlen, amíg meg nem jelöli őket.
search-tip-book = Könyvek: a könyv tartály és annak eleje / vége markerek.
search-tip-part = Alkatrészek: részszintű osztók.
search-tip-chapter = Fejezet: fejezetek, bármilyen módon is tárolja őket a projekt.
search-tip-scene = Jelenetek: azok a tárgyak, amik a prózádat tartják.
search-tip-note = Megjegyzés: szabad formájú megjegyzések.
search-tip-folder = Könyvtárak: egyszerű mappák és elválasztók.
search-tip-paratext = Egy szöveg, ami a könyvhöz tartozik, de nem a történethez: egy előlap, egy odaadás, egy utószó.
search-tip-preserve-case = Tartalékos eset: a csere átveszi az ügyet, amit talált, így az ELENA-ból MARTA lesz, és az Elena-ból is az lesz.
search-tip-replace = Csere: mutassa meg a csere mező és cserélje az összes.
search-error = Keresés sikertelen: { $message }
search-no-matches = Nincs egyezés.
search-count =
    { $matches ->
        [one] { $matches } mérkőzés
       *[other] { $matches } mérkőzés
    } in { $items ->
        [one] { $items } dokumentum
       *[other] { $items } dokumentum
    }
search-count-truncated =
    { $matches ->
        [one] { $matches } mérkőzés
       *[other] { $matches } mérkőzés
    } in { $items ->
        [one] { $items } dokumentum
       *[other] { $items } dokumentum
    } (csak az első eredmények)
search-occurrences = { $count }
search-field-body = Test
search-field-title = Cím
search-field-synopsis = Szinopszis
search-field-label = Rövid leírás
search-field-epigraph = Mottó
search-field-comment = Megjegyzés
search-field-comment-reply = Válasz
search-field-footnote = Lábjegyzet
search-include-in-replace = Tartalmazza a Csere minden
search-collapse-all = Az összes eredmény összecsukása
search-replace-here = Ezt cseréld ki.
search-dismiss = Az eredményekből való felmentés
search-undo-dismiss = Hozd vissza az utolsó elutasított eredményt.
search-replace-nothing = (semmi)
search-replace-confirm-title = Minden gyufát kicserélni?
search-replace-confirm-text =
    Csere { $occurrences ->
        [one] { $occurrences } előfordulás
       *[other] { $occurrences } Előfordulások
    } of "{ $query }" with "{ $replacement }" across { $items ->
        [one] { $items } dokumentum
       *[other] { $items } dokumentum
    } Ezt visszacsinálhatod a bejelentésből.
search-replace-done-title = A csere befejeződött
search-replace-done =
    { $occurrences ->
        [one] { $occurrences } előfordulás
       *[other] { $occurrences } Előfordulások
    } replaced in { $items ->
        [one] { $items } dokumentum
       *[other] { $items } dokumentum
    } .
search-replace-done-skipped =
    { $occurrences ->
        [one] { $occurrences } előfordulás
       *[other] { $occurrences } Előfordulások
    } replaced in { $items ->
        [one] { $items } dokumentum
       *[other] { $items } dokumentum
    }. { $skipped ->
        [one] { $skipped } mező
       *[other] { $skipped } mező
    } kimaradt (megváltozott a keresés óta).
search-replace-undo = Visszavonás
search-replace-failed-title = A csere nem sikerült
search-replace-skipped-title = Semmi sem változott.
search-replace-skipped-body = A keresés óta megváltozott szövegek száma: { $fields }. Ezeket a szövegeket nem módosítottuk. Keressen újra a szavak jelenlegi helyének megtekintéséhez.
search-replace-undo-failed-title = Nem sikerült visszavonni
search-preview = Előnézet
search-preview-empty = Válassza ki az eredményt megtekintéséhez itt
search-preview-no-prose = Ez a mérkőzés nincs szerkeszthető szöveg
search-preview-prompt = Hogy megnézze az előzetest, futtasson le egy keresést.
search-preview-open-search = Keresés a projektben
search-preview-footnote-prompt = Ez a mérkőzés egy lábjegyzetben található, saját szövegben. Nyissa ki a Footnotes dokkban, hogy lássa és szerkesztse.
search-preview-open-footnotes = Nyitott lábjegyzetek

find-placeholder = Keresse meg ebben a dokumentumban
find-count = { $current }, { $total }
find-no-results = Nincs eredmény
find-close = Közel van
find-previous = Előző mérkőzés (Shift + Enter)
find-next = Következő mérkőzés (Enter)
find-opt-case = Gyufásdoboz
find-opt-whole-word = Egész szó
find-replace-toggle = Tuggle csere (Ctrl + R)
find-replace-placeholder = Cserélhető
find-replace = Csere
find-replace-all = Cserélje ki az összeset
find-preserve-case = Tartózkodási eset

dict-download-title = Letöltés { $name }
dict-download-done = Telepített { $name }
dict-download-failed = Letölthető { $name }: { $error }
dict-removed = Eltávolított { $name }
dict-accept-first = A { $name } letöltése előtt fogadja el a licencet
settings-dict-tab-installed = Telepítve
settings-dict-tab-get-more = Szerezz többet
settings-dict-tab-personal = Személyes szavak
dict-installed-empty = Még nem találtunk szótárat ezen a számítógépen.
dict-get-more-search = Keresési nyelvek
dict-system-badge = a rendszeréről
dict-unusable-badge = használhatatlan
dict-download-button = Letöltés
dict-downloading = Letöltés
dict-installed-label = Telepítve
dict-remove = Eltávolítás
dict-view-license = Jogosítvány megtekintése
dict-approx-size = ~{ $size }
dict-personal-empty = Ebben a projektben még nincsenek személyes szavak.
dict-personal-add = Hozzáadás
dict-personal-placeholder = Egy szó hozzáadása
dict-missing-toast = { $count ->
    [one] A projekt { $count } szótárt használ, amit nem telepítettél.
   *[other] Ez a projekt { $count } szótárakat használ, amiket nem telepítettél.
}
dict-missing-action = Szótárak
dict-license-title = { $name } engedély
dict-license-accept = Letöltés elfogadása
dict-license-cancel = Mégse
dict-license-close = Bezárás
dict-add-button = Szótár hozzáadása
dict-add-title = Szótár hozzáadása
dict-add-close = Bezárás
dict-add-name = Név
dict-add-name-placeholder = pl. a latin szótáram
dict-add-code = Nyelvi kód
dict-add-code-placeholder = pl. la vagy fr- FR- x- egyedi
dict-add-code-hint = Egy rövid címke, amit választhat. Ez az, amit választ, mint egy dokumentum nyelv.
dict-add-aff = Affix fájl (.aff)
dict-add-dic = Szöveglista (.dic)
dict-add-submit = Hozzáadás
dict-add-cancel = Mégse
dict-add-name-required = Mondj egy nevet a szótárnak.
dict-add-code-required = Adja meg a nyelvkódot.
dict-add-code-invalid = Csak betűket, számokat és... _.
dict-add-code-reserved = Ez egy építmény szótár kód. Válasszon egy másik.
dict-add-file-required = Válasszon egy fájlt.
dict-add-file-missing = Az a fájl nem létezik.
dict-add-code-taken = Ennek a kódnak a szótára már be van telepítve.
dict-add-done = Hozzáadott { $name }
dict-add-unusable = Ezek a fájlok nem egy használható szótár: { $error }
dict-add-failed = A szótár hozzáadása: { $error }
inspector-export = Exportálás
inspector-exportable = A kivitelben való részvétel
ctx-number = Szám ebben a fejezetben (&N)
ctx-unnumber = Ne számozd ezt a fejezetet (&N)
inspector-numbering = Számozás
inspector-numbered = Számozott
inspector-numbered-tip = Ez a fejezet veszi át a helyét a könyv download. Kapcsolja ki egy prologue, egy epilogue vagy egy interlude.
inspector-numbered-tip-more = Egy számozatlan fejezet pontosan úgy marad a könyvben, ahogy volt: a címe, a prózája és a szószáma érintetlen. Egyszerűen nem jelent meg számot, és nem használ fel egyet: a prológust követő fejezet az első fejezet, nem a második fejezet. Az exportból való kihagyása ehelyett egy másik kapcsoló, és az egyik teljesen eltávolítja a fejezetet a könyvből.
inspector-apply-to-children = Alkalmazás gyermekekre
inspector-milestone = Mérföldkő dátuma
inspector-milestone-none = Nincs dátum
inspector-milestone-clear = Törlés
inspector-tags = Címkék
inspector-aliases = Más néven:
inspector-dict-language = Nyelv
inspector-apply-language-to-children = Alkalmazza a nyelvet a gyermekek
settings-page-language = Nyelv
settings-field-dict-language = Nyelvek
dict-tradeoff-hint = Minden kiegészítő nyelv több szót fogad el, így kevesebb hibát kapunk.
lang-inherit-hint = Inherited: ez a jelenet használja a könyv, illetve projekt a nyelvek.
lang-pill-list = Nyelvek
lang-pill-add = Nyelv hozzáadása
lang-pill-remove = A { $name } eltávolítása
lang-pill-mute = A { $name }-es átkapcsolása
lang-pill-unmute = Turn spell- check on for { $name }

settings-page-personal-dictionary = Személyes szótár
settings-user-dict-desc = A szavakat, amiket itt hozzáteszel, úgy kezelik, mint a helyesen betűzött mindenhol az app, és soha többé nem jelölt.
settings-user-dict-add-placeholder = Egy szó hozzáadása
settings-user-dict-add = Szöveg hozzáadása
settings-user-dict-search = Szűrőszavak
settings-user-dict-count = { $count ->
    [one] { $count } szó
   *[other] { $count } szó
}
settings-user-dict-rename = Átnevezés
settings-user-dict-remove = Eltávolítás
settings-user-dict-empty = Még nincs szó. Add hozzá a szavakat a helyesírási kell figyelmen kívül hagyni, vagy importálni egy listát.
settings-user-dict-import = Importálás…
settings-user-dict-export = Exportálás…
settings-user-dict-import-tip = Adja hozzá a szavakat egy egyszerű szöveges fájl, egy szó soronként. Meglévő szavak megtartják.
settings-user-dict-export-tip = Mentse az egész listát egy egyszerű szöveges fájl, egy szó soronként.
settings-user-dict-txt-filter = Szövegfájlok
settings-user-dict-duplicate = Már a szótárban
settings-user-dict-imported = Importált { $count ->
    [one] { $count } szó
   *[other] { $count } szó
} ({ $duplicates } már jelen van).
settings-user-dict-import-failed = Nem sikerült elolvasni a szólistát: { $error }
settings-user-dict-exported = Mentett { $count ->
    [one] { $count } szó.
   *[other] { $count } szó.
}
settings-user-dict-export-failed = Nem sikerült elmenteni a szólistát: { $error }

settings-page-text-replacements = Szövegcsere
settings-text-repl-desc = Cserélje ki a whithand a teljes szöveget, ahogy gépel: d.m.m.m.m.m.
settings-text-repl-enable = Szöveg helyettesítések használata ebben a projektben
settings-text-repl-disabled-hint = Kapcsolja be ezt, hogy meghatározza a gyorsírás, hogy bővül, ahogy írsz.
settings-text-repl-add = A szabály hozzáadása
settings-text-repl-trigger-placeholder = Short
settings-text-repl-replacement-placeholder = Mivé válik
settings-text-repl-added = Hozzáadva { $trigger }
settings-text-repl-duplicate = A { $trigger } már rendelkezik egy szabállyal.
settings-text-repl-filter = Szűrési szabályok
settings-text-repl-count = { $n ->
    [one] 1 szabály
   *[other] { $n } szabály
}
settings-text-repl-row-enabled = E szabály alkalmazása
settings-text-repl-delete = A { $trigger } szabályának törlése
settings-text-repl-deleted = A { $trigger } szabály törlése
settings-text-repl-empty = Még nincsenek szabályok.
settings-text-repl-csv-filter = CSV fájl
settings-text-repl-import = Importálás…
settings-text-repl-export = Exportálás…
settings-text-repl-imported = Importált { $added }, kimaradt { $skipped }
settings-text-repl-exported = Kivitel { $n ->
    [one] 1 szabály
   *[other] { $n } szabály
}

editor-menu-no-suggestions = Nincs javaslat.
editor-menu-add-to-dictionary = Hozzáadás { $word } A szótárhoz
editor-menu-add-words-to-dictionary = A kijelölt szavak hozzáadása a szótárhoz
editor-dict-added = Hozzáadva { $word }-et a szótáradhoz.
editor-dict-added-multi = { $count ->
    [one] Hozzáadva { $count } szót a szótárhoz.
   *[other] Hozzáadva { $count } szavakat a szótárhoz.
}
toast-undo = Visszavonás

undo-superseded-title = Ez a lépés már nem az utolsó.
undo-superseded-body = Valami más is megváltozott ebben a projektben azóta. A visszavonás inkább visszavinné azt, így ez nem történt meg. Ez a lépés nem fordult meg. Használja Edit Undo-t, hogy lépjen vissza a történelem.
undo-failed = Nem sikerült visszavonni: { $error }

menu-edit = Szerkesztés (&E)
menu-edit-undo = Visszavonás (&U)
menu-edit-redo = Újra (&R)
menu-edit-undo-target = Indo { $target } (&U)
menu-edit-redo-target = Redo { $target } (&R)
shortcut-name-edit-undo = Visszavonás
shortcut-name-edit-redo = Újra
undo-target-typing = gépelés
undo-target-project = a projekt utolsó módosítása
prose-history-reset-title = A gépelési előzmények visszaállítása
prose-history-reset-body = { $count ->
    [one] Egy nyílt jelenet egy korábbi állapotból került helyre, így a gépelési előzmények már nem érvényesek.
   *[other] A { $count } nyílt jeleneteket egy korábbi államból állították vissza, így a gépelési előzményeik már nem érvényesek.
}
undo-target-trash = a szemétbe való költözés,
undo-target-restore = a szemétből történő visszaállítás
undo-target-delete-forever = törlés örökre
undo-target-replace-all = a projekt egészének felváltása
undo-target-import = a behozatali okmány
undo-target-duplicate = sokszorosítás
undo-target-move = mozgás
undo-target-merge = két jelenet összevonása
undo-target-split = Egy jelenet feldarabolása
undo-target-promote = a típus megváltoztatása
undo-target-tidy-titles = A címek tisztítása
undo-target-import-tags = importáló címkék
undo-target-import-templates = az importáló jegyzetek sablonjai
undo-target-create = Létrehozása
undo-target-remove = Törlés
undo-target-rename = átnevező
undo-target-edit = a szerkesztés
menu-edit-find = Keresés… (&F)
menu-edit-find-next = Keresse meg a következő (&N)
menu-edit-find-prev = Előző keresése (&V)
menu-edit-replace = Keresés és helyettesítés (&L)
menu-edit-find-in-project = Keresés a projektben (&J)
menu-edit-replace-in-project = A projekt helyettesítése (&I)
undo-frozen = Visszavonás (mindig előre)
redo-frozen = Redo (mindig előre)
menu-edit-cut = Kivágás (&T)
menu-edit-copy = Másolás (&C)
menu-edit-paste = Beillesztés (&P)
menu-edit-paste-plain = Beillesztés formázás nélkül (&W)
menu-edit-select-all = Összes kijelölése (&A)
shortcut-name-edit-cut = Kivágás
shortcut-name-edit-copy = Másolás
shortcut-name-edit-paste = Beillesztés
shortcut-name-edit-paste-plain = Beillesztés formázás nélkül
shortcut-name-edit-select-all = Összes kijelölése

titlebar-spellcheck-on = A helyesírás- ellenőrzés be van kapcsolva. Kattintson ide, ha abba szeretné hagyni az ellenőrzést (F7)
titlebar-spellcheck-off = A helyesírás- ellenőrzés kikapcsolt. Kattintson ide, ha újra ellenőrizni szeretné (F7)
menu-spellcheck = A helyesírás ellenőrzése (&C)
menu-comments = Megjegyzések (&M)
settings-page-spellcheck = Betűellenőrzés
settings-group-spellcheck = Betűellenőrzés
settings-spellcheck-enabled = Csekkold a helyesírást, ahogy írok
settings-spellcheck-hint = Alsó szavak nincs telepített szótár tudja. Ha ezt kikapcsolja leállítja minden ellenőrzés, minden projekt, amíg be nem kapcsolja vissza. Ahhoz, hogy ne ellenőrizze csak egy nyelv, törölje az ellenőrző jel a nyelvi területen a projekt vagy egy elem.

menu-trash = Kuka (&T)
trash-title = Kuka
trash-empty-state = A szemét üres.
trash-empty-button = Üres szemétdomb
trash-restore = Visszaállítás (&R)
trash-restore-to = Helyreállítás (&T)
trash-delete-forever = Törlés mindörökké (&D)
trash-restored-ok = { $count } restaurálva.
trash-restore-error = Nem sikerült visszaállítani: { $error }
trash-restore-orphaned = Ez a tétel az eredeti hely eltűnt. Válassza ki, hol kell visszaállítani.
trash-restore-no-project = Nincs nyitott projekt, így nincs mit helyreállítani.
trash-delete-no-project = Nincs nyitott projekt, így nincs mit törölni.
trash-empty-confirm-title = Ürítsd ki a szemetet?
trash-empty-confirm-text = { $count ->
    [one] A roncsolt { $count } bejegyzést véglegesen el kell hagyni.
   *[other] { $count } törölt bejegyzéseket véglegesen törölni kell.
} A szemetet örökre otthagyják, és a visszavonulás visszahozza őket, amíg nem csinálsz valami mást.
trash-emptied-title = Ürített szemét
trash-emptied-body = Minden a szemétben véglegesen törlődött.
trash-empty-no-project = Nincs nyitott projekt, így nincs ürítendő szemét.
trash-delete-forever-confirm-title = Örökre kitörölni?
trash-delete-forever-confirm-text = { $count ->
    [one] A { $count } tételt véglegesen törölni kell.
   *[other] A { $count } elemet véglegesen törölni kell.
} A szemetet örökre otthagyják, és a visszavonulás visszahozza őket, amíg nem csinálsz valami mást.
trash-deleted-title = Örökre törölve
trash-deleted-body = { $count } véglegesen törölve.
trash-undo = Visszavonás
trash-restore-picker-title = Helyreállítás
trash-restore-to-confirm-title = Itt?
trash-restore-to-confirm-text = Helyreállítani a { $item }-et a { $destination }-be?
trash-restore-picker-restore-here = Visszaállítás itt
trash-restore-picker-cancel = Mégse
trash-restore-picker-empty = Még nincs irattartó, először készíts egyet.
trash-banner-title = Ez a tétel a szemétben van.
trash-banner-description = Megjelenik a vázlatban vagy az exportban, amíg vissza nem állítod.
trash-banner-restore = Visszaállítás…
trash-tab-tooltip = A kukában

settings-styles-sheet-yes = Igen
settings-styles-sheet-no = Nem
settings-styles-sheet-font = Betűtípus
settings-styles-sheet-indent = Első vonalbeli francia bekezdés
settings-styles-sheet-para-spacing = A bekezdés sorrendje
settings-styles-sheet-page = Oldalméret
settings-styles-sheet-margins = Jelölések (T / R / B / L)
settings-styles-sheet-title-page = Könyvcímoldal
settings-styles-sheet-heading-language = Címnyelv
settings-styles-sheet-auto = A szöveg nyomon követése
settings-styles-sheet-digits = Számok
settings-styles-sheet-direction = Szövegirány
settings-styles-sheet-formats = Formátumok
settings-styles-sheet-all-formats = Valamennyi

settings-styles-editor-group = Stílusszerkesztő
settings-styles-editor-missing = Ez a stílus már nem elérhető.

settings-styles-page-letter = Levél
settings-styles-digits-western = Nyugati (0) 9
settings-styles-digits-eastern-arabic = Keleti arab (NRG)
settings-styles-direction-ltr = Balról jobbra
settings-styles-direction-rtl = Jobbra balra

format-dock-title = Formázás
format-panel-empty = Helyezze a kurzor egy jelenet, megjegyzés, vagy szinopszis látni formázás lehetőségeket.
format-group-marks = Szöveg
format-group-block = Bekezdés
format-group-lists = Listák
format-group-tables = Táblázat
format-group-breaks = Jelenettörések
format-superscript = Szuperszkript
format-subscript = Alforgatókönyv
format-link = Link
link-dialog-insert-title = Kapcsolat beszúrása
link-dialog-edit-title = A link szerkesztése
link-dialog-text-label = Szöveg
link-dialog-text-placeholder = A szavak, amiket az olvasó lát
link-dialog-url-label = Linkek
link-dialog-url-placeholder = example.com
link-dialog-insert = Beillesztés
link-dialog-apply = Alkalmazás
link-dialog-cancel = Mégse
link-dialog-remove = Kapcsolat eltávolítása
link-scheme-refused = Csak webes és e-mail linkeket lehet megnyitni. { $url } maradt egyedül.
menu-format-link = Link (&N)
format-clear = A formázás törlése
format-heading = Irányszint
format-heading-normal = Normál szöveg
format-heading-1 = 1. fejezet
format-heading-2 = 2. fejezet
format-heading-3 = 3. fejezet
format-heading-4 = 4. fejezet
format-heading-5 = 5. fejezet
format-heading-6 = 6. fejezet
format-align-left = Balra igazítás
format-align-center = Középpont
format-direction-rtl = Jobbra-balra bekezdés
format-blockquote = BlokkolName
format-list-bullet = Tömörített lista
format-list-numbered = Számozott lista
format-indent = Behúzás növelése
format-outdent = Külső
format-table-insert = A táblázat beszúrása
format-table-row-above = A fenti sor beszúrása
format-table-row-below = Az alábbi sor beszúrása
format-table-col-before = Az oszlop beszúrása
format-table-col-after = Az oszlop beszúrása után
format-table-row-delete = A sor törlése
format-table-col-delete = Az oszlop törlése
format-table-remove = A táblázat eltávolítása

menu-format-marks-bold = Merész (&D)
menu-format-marks-italic = Olasz (&I)
menu-format-marks-underline = Aláhúzott (&U)
menu-format-marks-strike = Áthúzott (&S)
menu-format-marks-superscript = Szuperszkript (&P)
menu-format-marks-subscript = Alforgatókönyv (&B)
menu-format-marks-clear = Átalakítás törlése (&C)
menu-format-heading = Címsor (&H)
menu-format-heading-normal = Normál szöveg (&N)
menu-format-heading-1 = 1. fejezet (&1)
menu-format-heading-2 = 2. fejezet (&2)
menu-format-heading-3 = 3. fejezet (&3)
menu-format-heading-4 = 4. fejezet (&4)
menu-format-heading-5 = 5. fejezet (&5)
menu-format-heading-6 = 6. fejezet (&6)
menu-format-alignment = Kiigazítás (&A)
menu-format-align-left = Balra igazítás (&L)
menu-format-align-center = Középpont (&C)
menu-format-direction = Iránya (&E)
menu-format-direction-auto = Automatikus (&A)
menu-format-direction-ltr = Balról jobbra (&L)
menu-format-direction-rtl = Jobbra balra (&R)
menu-format-blockquote = BlokkolName (&Q)
menu-format-lists = Listák (&L)
menu-format-list-bullet = Tömörített lista (&B)
menu-format-list-numbered = Számozott lista (&N)
menu-format-indent = Behúzás növelése (&I)
menu-format-outdent = Külső (&O)
menu-format-table = Táblázat (&T)
menu-format-table-insert = A táblázat beszúrása (&I)
menu-format-table-2x2 = 2 x 2 (&2)
menu-format-table-3x3 = 3 x 3 (&3)
menu-format-table-4x4 = 4 x 4 (&4)
menu-format-table-row-above = A fenti sor beszúrása (&V)
menu-format-table-row-below = Az alábbi sor beszúrása (&W)
menu-format-table-col-before = Oszlop beszúrása (&L)
menu-format-table-col-after = Az oszlop beszúrása után (&T)
menu-format-table-row-delete = A sor törlése (&D)
menu-format-table-col-delete = Az oszlop törlése (&C)
menu-format-table-remove = A táblázat eltávolítása (&R)

menu-format-typo-marks = A jelölés formázása (&F)
menu-format-typo-nbsp = Nem törhető tér (&N)
menu-format-typo-nbhyphen = Nem törjük meg a hyphent. (&H)
menu-format-typo-soft-hyphen = Puha hyphen (&S)
menu-format-typo-nnbsp = Szűk No- break tér (&R)
menu-format-typo-zwsp = Zeroszélesség Terület (&Z)
menu-format-typo-word-joiner = Word Joiner (&W)


menu-about = Körülbelül Skribisto (&A)
about-title = Körülbelül Skribisto
about-version = Változat { $version }
about-update-available = A { $version } verzió elérhető
about-update-see-notes = Lássuk, mi változott.
about-update-flathub = Flathub naprakészen tartja ezt a másolatot.
about-update-distro = A terjesztésed naprakészen tartja ezt a másolatot.
about-tagline = Újságíró app hosszú formátumú fikcióhoz, a Teksilo eszköztárral együtt íródott rozsda formájában.
about-license = A GNU General Public License 3. verziója szerint adták ki.
about-copyright = © 2026 Cyril Jacquet
about-close = Bezárás

native-menu-about = Körülbelül { $app }
native-menu-hide = A { $app } elrejtése
native-menu-quit = Kilépés { $app }
native-menu-settings = Beállítások…
native-menu-window = Ablak
native-menu-minimize = Minimalizálás
native-menu-zoom = Nagyítás

window-title = { $title } - { $app }
window-title-numbered = { $title } - { $app } (Ablak { $n })
window-title-empty = { $app }

first-run-window-title = Üdvözlöm.
first-run-title = Állítsák fel a { $app }
first-run-body = { $app } a beállításokat elkülönítve tartja a Skribisto-től, így üresen indul. Az Ön preferenciái, a legutóbbi projektek, a szótárak és az ablak elrendezése most már másolható.
first-run-from = Másolás innen:
first-run-to = Másolás
first-run-copy-note = Semmi sem mozdítható vagy távolítható el. Skribisto megtartja az összes saját beállításokat, és folytatja a munkát pontosan úgy, mint korábban.
first-run-import = Beállítások importálása
first-run-start-fresh = Újrakezdés
first-run-import-failed = Egyes beállítások nem importálhatók: { $error }

settings-page-punctuation = Szúrási hiba
settings-group-punctuation = Intelligens pontozás
settings-page-work-punctuation = Pontozás
settings-punctuation-override = Adja meg a projekt saját pontozási szabályait
settings-punctuation-override-hint = Off: a projekt követi a jelentkezési preferenciát. Ezek a szabályok a .skrib-en belül mozognak, így egy társszerző, aki a fájlt nyitja, azonos tipográfiával ír.
settings-punctuation-dashes = Turn -- into an dash, --- into a em dash
settings-punctuation-ellipsis = Átváltozunk ellipszissé.
settings-punctuation-quotes = Curl idézőjelek és aposztrófák
settings-quote-style = Tőzsdei jegyzés jelei
settings-quote-style-locale = Nyelvi alapértelmezés
settings-quote-style-curly = - Kettős?
settings-quote-style-curly-single = *Single* *Single*
settings-quote-style-guillemets = [Guillemes]
settings-quote-style-low-high = - Low- high
settings-punctuation-spacing = Tér előtt;:!?
settings-punctuation-spacing-hint = Francia tipográfia beállít egy keskeny no-break tér előtt;! és egy teljes előtt:. Ez csak a szöveg franciául írt. A hely belül guillemes meg jön a guillemet idézetek magukat.
settings-punctuation-sample = A nyelved ad
settings-punctuation-app-hint = Amit minden projekt tesz, kivéve, ha saját szabályokat fogad el a Projekt Punctuation-ban.
settings-punctuation-dialogue = Megnyitunk egy bekezdést, amelyet úgy írunk le, mint egy dialógus-kötőjel
settings-punctuation-dialogue-hint = A nyelvek, amelyek a beszédet egy kötőjel helyett idézőjel: francia, spanyol, orosz és mások. Ez csak tűz a legelején egy bekezdés.

statusbar-go-to = Menj el!
go-to-placeholder = Keresse meg a mappát
go-to-no-matches = Nincs olyan tárgy, ami egyezne a kereséssel.
menu-go-to = Menj el! (&G)

settings-page-distraction-free-themes = Disztrakció-mentes témák
settings-themes-builtin = Beépített témák
settings-themes-builtin-badge = Beépített
settings-themes-user = A témáim
settings-themes-editor-group = Téma szerkesztése
settings-themes-editor-empty = Válassz egy témát az én témáim szerint, hogy szerkeszd.
settings-themes-use = Felhasználás
settings-themes-duplicate = Másolat készítése
settings-themes-edit = Szerkesztés
settings-themes-delete = Törlés
settings-themes-export = Exportálás…
settings-themes-import = Importálás…
settings-themes-copy-suffix = Másolás
settings-themes-json-filter = Téma (JSON)
settings-themes-imported = Importált téma
settings-themes-import-failed = Nem sikerült importálni ezt a témát
settings-themes-exported = Exportált téma
settings-themes-export-failed = Nem sikerült exportálni ezt a témát
settings-themes-low-contrast = alacsony kontraszt
settings-themes-low-contrast-band = a kiemelés elrejti a szöveget
settings-themes-field-name = Név
settings-themes-field-paper = Oldal
settings-themes-field-ink = Szöveg
settings-themes-field-general = Háttér
settings-themes-field-widget-text = A vezérlőcsík szövege
settings-themes-field-caret-band = Kiemelés a gondozó körül

statusbar-focus-settings = Terjesztésmentes beállítások
statusbar-focus-manage-themes = Kezelési témák

comments-title = Megjegyzések
comments-document-title = Ez a dokumentum
comments-empty-project = Ebben a projektben még nincs hozzászólás.
comments-empty-document = Nincs hozzászólás ehhez a dokumentumhoz.
comments-filter-all = Valamennyi
comments-filter-open = Megnyitás
comments-filter-resolved = Megoldva
comments-filter-orphaned = Árva
comments-status-open = Megnyitás
comments-status-resolved = Megoldva
comments-status-orphaned = Elvesztette a szövegét
comments-orphan-snippet = (a fűzött szöveg eltűnt)
comments-status-unplaced = Nincs szöveges pozíció
comments-unplaced-snippet = (semmilyen szöveghez nem rögzítve)
comments-reply-count = { $count ->
    [one] { $count } válasz
   *[other] { $count } válasz
}
comments-menu-resolve = Megoldás
comments-menu-reopen = Újraindítás
comments-menu-delete = Törlés
comments-sort-document = Dokumentumsorrend
comments-sort-newest = Legközelebbre.
comments-sort-reader = Leolvasó által csoportosítva
comments-unsigned-author = Aláírás nélkül
comments-agreement = { $count ->
    [one] { $count } olvasó
   *[other] { $count } leolvasó
}
comments-menu-add = Hozzászólás
comments-menu-add-paragraph = Megjegyzés erre a bekezdésre
overview-col-comments = Megjegyzések
overview-col-total-comments = Összes észrevétel
comments-card-placeholder = Írj egy megjegyzést!
comments-card-unknown-author = Ismeretlen szerző
comments-unsigned-toast = Az Ön észrevételeit nem írták alá: nincs beállítva név az Ön számára ezen a számítógépen.
comments-unsigned-action = A neved beállítása
comments-card-reply = Válasz
comments-card-reply-placeholder = Válasz
comments-card-actions = Észrevételek
comments-reply-actions = Válasz
comments-menu-delete-reply = A válasz törlése
comments-menu-delete-all = Az összes hozzászólás törlése ide
comments-deleted-toast = Megjegyzés törölve
comments-reply-deleted-toast = A válasz törölve
comments-deleted-all-toast = { $count ->
    [one] { $count } megjegyzés törölve
   *[other] { $count } hozzászólás törölve
}
comments-undo = Visszavonás

analysis-segment = Elemzés
analysis-scope-book = A könyv elemzése
analysis-run = Elemzés futtatása
analysis-stale = Megváltozott, mióta ez futott.
analysis-not-run = Még nem elemezték.
analysis-running = A kézirat olvasása
analysis-failed = Az elemzés nem fejeződött be.
analysis-no-scenes = Ebben a könyvben még nincsenek jelenetek.

analysis-shape = Alakzat

analysis-arrivals = Érkezések
analysis-arrivals-explainer = Azt írja, hogy a karakterek melyik útvonalán jöttek le, és egyáltalán nem arról van szó, hogy ki írta őket: egy író, aki máshova rajzol, és a paszta megtapadt, és aki diktál, azt diktálta. Nincs olyan szám, amire célozhatna.
analysis-arrivals-scope = Ez az egész projekt, nem csak ez a könyv.
analysis-arrivals-session = Amióta kinyitottad, a projekt lezárása újra megkezdi a számolást.
analysis-arrivals-nothing = Ezen az ülésen még nem érkezett szöveg.
analysis-arrivals-typed = Gépelt
analysis-arrivals-pasted = Pasztált
analysis-arrivals-dictated = Diktálva
analysis-arrivals-imported = Importált
analysis-arrivals-programmatic = Beírattalak.
analysis-arrivals-count = { $count ->
    [one] { $count } karakter
   *[other] { $count } karakter
}
analysis-arrivals-none = Nincs

analysis-words-per-scene = Szavak jelenetenként
analysis-median-words = Ez a könyv a Medián jelenet fut { $count ->
    [one] { $count } szó.
   *[other] { $count } szó.
}
analysis-median-line = Medián: { $count ->
    [one] { $count } szó
   *[other] { $count } szó
}
analysis-dialogue = Párbeszéd
analysis-dialogue-unsupported = A párbeszéd még nem mérhető ezen a nyelven.

analysis-footnote-words = Lábjegyzetek
analysis-footnote-words-count = { $count ->
    [one] { $count } szó van ebben a könyvben.
   *[other] { $count } szó van ebben a könyvben, a lábjegyzeteket, a kézirat teljes.
}
analysis-footnote-words-pending = A lábjegyzeteket számoljuk.

analysis-ignore-empty = Még nem írt szövegek figyelmen kívül hagyása
analysis-empty-hidden = { $count } üres { $count ->
        [one] Szöveg
       *[other] szövegek
    } Elrejtve.
analysis-all-texts-empty = Ebben a könyvben minden szöveg üres.

binder-filter-count = { $shown } of { $total } mutatva
binder-filter-clear = Törlés
binder-filter-none = Semmi sem passzol a { $query }-hez.

settings-paratext-intro = A front és a back számít egy új projekt lehet kezdeni. Minden struktúra tartozik egy kiadói hagyomány, és az oldal címei vannak írva, hogy a hagyomány downs saját nyelven. Nevezze meg őket szabadon, ha egy projekt jön létre.
settings-paratext-structures = Struktúrák
settings-paratext-broken = Nem sikerült leolvasni
settings-paratext-edit = Szerkesztés
settings-paratext-duplicate = Másolat készítése
settings-paratext-delete = Törlés
settings-paratext-new = Új struktúra
settings-paratext-save = Mentés
settings-paratext-editor-hint = Egy név, a kézirat előtt létrehozandó oldalak, és a utána létrehozandó oldalak. Ahol végül létrejönnek, azok a tieid: a projekt létrejötte után szokásos tárgyak.


image-insert = A kép beszúrása (&M)
image-no-project = Projekt megnyitása a kép behelyezése előtt.
image-choose-title = Válasszon ki egy képet
image-filter-label = Képek
image-large-title = Ez egy nagy kép.
image-large-text =
    { $name } az { $megapixels } megapixel ({ $width } × { $height }).
    Megtartani, ahogy az eredeti fájlt tárolja a projektben: utazik
    Minden biztonsági mentés és minden export. Optimalizálása tárol egy kisebb másolat
    Ehelyett 2560 pixel a leghosszabb oldalon.
image-large-keep = Eredeti megtartása
image-large-downscale = Optimalizálás
image-large-remember = Tedd meg ezt mostantól, ne kérdezd újra.
image-not-recorded = A kép mentett, de nem lehetett rögzíteni a projektben.
image-describe-title = Írja le a képet
image-describe-explain = Amit a kép mutat, egy olvasó, aki nem látja. Ez nem része a kézirat: soha nem számít, keres vagy exportált próza.
image-describe-placeholder = Egy világítótorony a szürke ég ellen
image-resize-title = A kép átméretezése
image-resize-explain = Egy százalékát a méret jelenik meg most. 100 elhagyja azt, ahogy van.
image-resize-invalid = Adjon meg egy számot 1 és 1000 között.
image-menu-describe = Írja le a képet (&D)
image-menu-resize = A kép átméretezése (&R)
image-menu-reset-size = Eredeti méret (&Z)

cover-choose = Könyvborító (&V)
cover-clear = A borító eltávolítása (&R)
cover-choose-title = Válasszon egy borítót
cover-set = A borító be van állítva.
cover-cleared = A borítót eltávolítottuk, a kép még mindig benne van a projektben.
card-label = Rövid leírás
card-exportable = Kivitel
card-numbered = Számozott
card-created = Létrehozva
card-modified = Módosítva
card-yes = Igen
card-no = Nem
card-type = Típus
card-position = Pozíció
card-children = Gyermekek
card-goal = Cél
card-words = Szavak
card-synopsis = Szinopszis
card-point-of-view = Nézőpont
card-aliases = Más néven:

export-orphan-footnotes-title = Hivatkozás nélküli lábjegyzetek
export-comments-dropped =
    { $count ->
        [one] Egy észrevétel nem került be az exportált szövegbe, és azt kihagyták.
       *[other] A { $count } észrevétel nem kerülhetett bele az exportált szövegbe, és azt kihagyták.
    }
export-orphan-footnotes =
    { $count ->
        [one] Egy lábjegyzet már nem szerepel a kéziratban. A szöveg nem jelenik meg az exportált könyvben.
       *[other] { $count } lábjegyzeteket már nem hivatkoznak sehol a kéziratban. A szövegük nem jelenik meg az exportált könyvben.
    }


menu-footnotes = Lábjegyzetek (&N)
footnotes-title = Lábjegyzetek
footnotes-insert = A lábjegyzet beillesztése (&F)
footnotes-empty = Nincs lábjegyzet még. Tegye a kurzor egy jelenetet, majd használja + fenti, vagy Ctrl + Alt + F.
footnotes-filter-all = Valamennyi
footnotes-filter-document = Ez a dokumentum
footnotes-filter-orphaned = Árva
footnotes-orphaned = Semmi sem utal erre a levélre többé.
footnotes-untitled-home = Név nélkül
footnotes-body-placeholder = maga a jegyzet
footnotes-insert-tooltip = lábjegyzet beillesztése a kurzorba (Ctrl + Alt + F)
footnotes-actions = A lábjegyzetben szereplő intézkedések
footnotes-delete = Megjegyzés és hivatkozási száma (&D)
footnotes-no-project = A projekt megnyitása a lábjegyzet beillesztése előtt.
footnotes-no-caret = Rakja a kurzort egy szcéna szövegbe, hogy egy lábjegyzetet helyezzen oda.
footnotes-not-created = A lábjegyzetet nem lehetett hozzáadni a projekthez.
footnotes-deleted-toast = Megjegyzés törölve
footnotes-undo-delete = Visszavonás

import-document-title = Importokmányok
import-document-close = Bezárás
import-document-step-files = Fájlok
import-document-step-review = Felülvizsgálat
import-document-step-destination = Rendeltetési hely
import-document-step-reconcile = Egyesülés
import-document-reconcile-hint = Néhány ilyen elemek azok, amelyek már van. Mondd, hogy mi kell történni az egyes.
import-document-reconcile-all-new = Ebben a fájlban semmi sem felel meg a projektnek: minden elemet új példányként adunk hozzá.
import-document-duplicate-returns = Ezek a fájlok több, mint egy másolata ugyanannak a kéziratnak, ami visszatér. Importálja őket egyszerre: csak egy visszaadó példányt sorol össze a projektjével, így több importáló együtt hozzáadna egy második példányt a könyvből, ahelyett, hogy egyesítené azt.
import-document-hunk-added = Vegyük ezt az új bekezdést.
import-document-hunk-removed = Dobd el ezt a bekezdést
import-document-hunk-changed = Fogd ezt az átírást.
import-document-col-stray-prose = A dokumentum szövege
import-document-stray-as-paratext = Paratextként kell tartani.
import-document-stray-discard = Dobd el a szöveget
import-document-col-current = Az Ön projektjében
import-document-col-incoming = Ebben a fájlban
import-document-col-status = Állapot
import-document-col-action = Mit tegyünk?
import-document-status-identical = Ugyanaz.
import-document-status-editor-edited = Megszerkesztették.
import-document-status-you-edited = Megszerkesztetted.
import-document-status-conflict = Mindkettő szerkesztette.
import-document-status-different = Differek
import-document-status-new = Új
import-document-status-missing = Nem ebben az aktában.
import-document-status-moved = Áthelyezve
import-document-action-comments-only = Csak észrevételek
import-document-action-take-import = Vegyük ezt a verziót
import-document-action-keep-current = Tartsd meg az enyémet.
import-document-action-create-new = Hozzáadás új
import-document-action-ignore = Kihagyás
import-document-compare = Összehasonlítás
import-document-compare-legend = A te verziód ehhez a fájlhoz képest.
import-document-compare-close = Bezárás
import-document-drop-title = Dokumentumok eldobása itt
import-document-drop-hint = Markdown (.md), Word (.docx), OpenDocument (.odt) és egyszerű szöveg (.txt)
import-document-browse = Böngészés
import-document-move-up = Mozgatás felfelé
import-document-move-down = Mozgatás lefelé
import-document-remove-file = Eltávolítás
import-document-file-count = { $count ->
    [one] 1 fájl
   *[other] { $count } fájl
}
import-document-no-files = Még nincs kiválasztva fájl.
import-document-col-included = Importálás
import-document-col-title = Cím
import-document-col-type = Típus
import-document-col-words = Szavak
import-document-col-breaks = Törések
import-document-col-comments = Megjegyzések
import-document-col-footnotes = Lábjegyzetek
import-document-col-epigraph = Mottó
import-document-col-source = Forrás
import-document-level-rules = Irányok
import-document-level-n = { $level } vtsz.
import-document-add-top-level = A felső szint hozzáadása
import-document-add-top-level-tooltip = Helyezzen egy könyvet minden elemzett elem fölé, olyan fejezetfájlokhoz, amelyeknek nincs könyvcímük
import-document-destination = Rendeltetési hely
import-document-destination-hint = Válasszon egy mappát vagy elemet: új elemek landolnak egy mappában, vagy egy jelenet után.
import-document-destination-empty = Még nincs irattartó, először készíts egyet.
import-document-plan-empty = Még nincs mit behozni.
import-document-summary = { $rows ->
    [one] 1 tétel
   *[other] { $rows } tétel
} · { $breaks ->
    [one] 1 jelenet szünet
   *[other] { $breaks } jelenet szünetek
}
import-document-back = Vissza
import-document-cancel = Mégse
import-document-analyse = Következő
import-document-analysing = A dokumentumok olvasása
import-document-step-analysing = Olvasás
import-document-cancel-analysis = Hagyd abba az olvasást.
import-document-analyse-failed = A dokumentumokat nem lehetett elolvasni.
import-document-details = Részletek
import-document-import = Importálás
import-document-done = { $count ->
    [one] 1 Behozott tétel
   *[other] { $count } importált termék
}
import-document-undo = Visszavonás
import-diagnostic-file-unreadable = A { $path } nem olvasható: { $detail }.
import-diagnostic-nested-too-deep = A { $path } nem olvasott: a tartalma több mint { $limit }-es mélységig fészkelve van. Ehhez nincs valódi dokumentum, ami a közelébe jutna, és a Skribisto-at is leolvasná, tehát kimaradt. A többi fájl még mindig importálódik.
import-diagnostic-archive-too-large = A { $path } nem olvasható: ez egy olyan tömörített fájl, ami sokkal nagyobb, mint amekkora, ami kiszívja a memóriát és összeomlik a Skribisto-t, így kimarad. A többi fájl még mindig importálódik.
import-diagnostic-lossy-decode = { $count ->
    [one] Az egyik karakter nem dekódolt a { $path }-ben. A fájl elmentése UTF-8-ként.
   *[other] { $count } karakterek a "{ $path }" nem dekódolt. Re- mentse a fájlt, mint UTF-8 tartani őket.
}
import-diagnostic-decoded-from-bom = A { $path }-et { $detail }-nek, nem pedig UTF-8-nak minősítették.
import-diagnostic-empty-file = A { $path } üres.
import-diagnostic-no-headings = A { $path }-nek nincsenek vámtarifaszámai, így egy tételként érkezik.
import-diagnostic-unsupported-format = Semmi nem utal a { $detail } fájlokra, így a { $path }-et kihagyták.
import-diagnostic-front-matter-not-flat = Elöljáróanyag{ $path }": "{ $detail }Ez nem egy egyszerű érték, és kihagyták.
import-diagnostic-footnotes-degraded = { $count ->
    [one] A { $path } számú lábjegyzet egyszerű szövegként érkezik: a lábjegyzeteket nem a Markdown-ből olvassák.
   *[other] { $count } lábjegyzetben a { $path }-ben egyszerű szövegként érkezik: a lábjegyzeteket nem a Markdown-ból olvassák.
}
import-diagnostic-footnote-not-carried = { $count ->
    [one] Az egyik lábjegyzetet a { $path }-ben nem lehetett áthozni.
   *[other] { $count } lábjegyzeteket a { $path }-ben nem lehetett áthozni.
}
import-diagnostic-raw-html-dropped = { $count ->
    [one] Egy darab nyers HTML-es blokkot ejtettek a { $path }-es blokkban.
   *[other] { $count } darab nyers HTML darab, { $path } darabból álló blokkot ejtettek.
}
import-diagnostic-nested-break-dropped = { $count ->
    [one] Egy jelenet a { $path }-es idézet vagy lista belsejébe esett, és a saját vonalán csak egy szünet maradt.
   *[other] A { $count }-es jelenet a { $path }-es idézőjelben vagy listákon belül törik szét, és csak a saját vonalán tartanak szünetet.
}
import-diagnostic-image-not-ingested = "{ $path }- A kép a lényeg.{ $detail }A hivatkozás szövegként érkezik; a kép nem másolódik be.
import-diagnostic-duplicate-title = A { $title } { $count }-szer jelenik meg. Ha korábban már importálta ezeket a fájlokat, ez lemásolja őket.
import-diagnostic-heading-level-jump = A { $title } vámtarifaszám alá tartozó, { $from }-{ $to } vámtarifaszám alá tartozó ugrások az anyavállalat alá kerülnek.
import-diagnostic-illegal-combination = A { $title } a prózát szállítja, de a { $kind } nem tarthatja meg. A behozatal addig tart, amíg meg nem változtatja a típusát, vagy ki nem hagyja.
import-diagnostic-tracked-changes-flattened = A { $path }-et felülvizsgálták: { $count ->
    [one] { $count } nyomon követett változást fogadtak el
   *[other] { $count } nyomon követett változtatásokat fogadtak el
} Ez a végleges szöveg. Ha a saját szövegezését szeretné megőrizni, használja a Hasonlítást az utolsó lépésben.
import-diagnostic-tracked-changes-flattened-by = A { $path }-et felülvizsgálták: { $count ->
    [one] { $count } pályaváltoztatást fogadtak el { $names }-ig
   *[other] { $count } nyomkövető változtatásokat fogadtak el { $names }-ig
} Ez a végleges szöveg. Ha a saját szövegezését szeretné megőrizni, használja a Hasonlítást az utolsó lépésben.
import-diagnostic-text-box-dropped = { $count ->
    [one] { $path } rendelkezik { $count } szöveges doboz. Szöveg kívül esik a dokumentumon is, így ahol egy kézirat tartozik, nem lehet válaszolni. Nem importálják.
   *[other] { $path } rendelkezik { $count } szöveges dobozokkal. A szövegük kívül esik a dokumentumon, így ahol egy kéziratban található, nem lehet megválaszolni.
}
import-diagnostic-embedded-object-dropped = { $count ->
    [one] { $path } holds { $count } beágyazott objektum: egy diagram, egy egyenlet vagy hasonló. Nincs semmi egy kéziratban, hogy képes tartani.
   *[other] { $path } rendelkezik { $count } beágyazott objektumok: egy diagram, egy egyenlet vagy hasonló. Nincs semmi egy kéziratban, hogy képes tartani őket.
}
import-diagnostic-field-flattened = { $count ->
    [one] { $path } holds { $count } field: egy oldalszám, egy kereszt-hivatkozás, egy dátum. Megtartja a szöveget, amit utoljára mutatott, és nem frissíti újra.
   *[other] { $path } holds { $count } mezők: egy oldalszám, egy kereszthivatkozás, egy dátum. Mindegyik megtartja a szöveget, amit utoljára mutatott, és nem frissíti újra.
}
import-diagnostic-unknown-style-level = { $path } a { $detail }-es stílust használja, amely úgy néz ki, mint egy vámtarifaszám, de nem nevez meg semmilyen szintet. Ezeket a bekezdéseket prózaként importálják, ahelyett, hogy egy mélységben találnák ki.
import-diagnostic-comment-unanchored = A hozzászólást a { $detail } A { $path } nem lehetett csatolni a szavakat, hogy volt róla. Tartalmazza a tételt, ahol lehet mozgatni.
import-diagnostic-comment-not-carried = A hozzászólás a { $detail } A { $path } nem importálják, mert semmi a fájlban érkezik, mint szöveg lehet tartani. Másolja le az aktában, ha szüksége van rá.
import-diagnostic-comment-replies-flattened = { $count ->
    [one] { $count } válasz { $path } megnevezett egy megjegyzést, amely nincs az aktában, így érkezik, mint egy megjegyzést saját.
   *[other] { $count } válaszok { $path } megnevezett egy megjegyzést, amely nincs az aktában, így érkeznek, mint a saját észrevételeit.
}
import-diagnostic-prose-not-verbatim = { $count ->
    [one] { $count } bekezdés { $path } nem lehetett áthozni a formázás pontosan úgy, ahogy meg van írva, így érkezik, mint egyszerű szöveg. Hasonlítsa össze az eredeti.
   *[other] { $count } bekezdés { $path } nem lehetett áthozni a formázás pontosan az írásban, így megérkeznek, mint egyszerű szöveg. Hasonlítsa össze őket az eredeti.
}
import-diagnostic-styled-spaces-not-carried = { $count ->
    [one] A { $path }, { $count } sor üres hely volt aláhúzva, vagy átcsapott, egy sor maradt, hogy töltse ki, például. Skribisto nem tartja vonal a szóközöket egyedül, így érkezik, mint sima terek, vagy egyáltalán nem, ahol kezdődik vagy végződik egy bekezdést.
   *[other] A { $path }, { $count } szakaszok üres hely volt aláhúzva, vagy átcsapott, vonalak maradt, hogy töltse ki például. Skribisto nem tartja vonal a szóközöket egyedül, így megérkeznek, mint egyszerű terek, vagy egyáltalán nem, ahol elkezdik vagy befejezik egy bekezdés.
}
import-diagnostic-list-nesting-flattened = { $count ->
    [one] { $count } listatétel { $path } fészkelték több mint { $limit } szint mély. Megérkezik szinten { $limit }, a legmélyebb Skribisto tartja, a szavak és formázás.
   *[other] { $count } listás tételek { $path } fészkelték több mint { $limit } szint mély. Érkeznek szinten { $limit }, a legmélyebb Skribisto tartja, a szavak és formázás.
}
import-diagnostic-table-read-as-paragraphs = { $count ->
    [one] { $count } táblázat { $path } van egy sor sokkal szélesebb, mint a többi, túl féloldalas ahhoz, hogy ki, mint egy rács. A cellák érkeznek, mint bekezdések, egyik a másik után, a szavak és formázás.
   *[other] { $count } asztalok { $path } van egy sor sokkal szélesebb, mint a többi, túl féloldalas ahhoz, hogy ki, mint egy rács. A cellák érkeznek, mint bekezdések, egymás után, a szavak és formázás.
}
import-diagnostic-spaces-shortened = { $count ->
    [one] { $count } a { $path } területen hosszabb volt, mint a { $limit }, és { $limit }-re rövidítve érkezett.
   *[other] A { $count } pályaszámú terület a { $path }-ben hosszabb volt, mint a { $limit }, és { $limit }-re rövidítve érkezett.
}
import-diagnostic-epigraph-not-carried = A { $title }-et egy epigraph vezeti, de egy { $kind }-es nem bír el egyet. Az idézetet a szöveg tetején tartják.
import-diagnostic-epigraph-placement-ambiguous = Egy epigraph ül között{ $title }Az e rendeletben előírt intézkedések összhangban vannak a Növények, Állatok, Élelmiszerek és Takarmányok Állandó Bizottságának véleményével,{ $below }És akár el is mehetett volna.{ $title }Ez az a hely, ahová egy epigraph szokott menni.
import-document-diagnostics = { $errors ->
    [0] { $warnings ->
            [one] 1 dolog, amit tudni kell
           *[other] { $warnings } dolog tudni
        }
   *[other] { $errors ->
            [one] 1 fájl nem olvasható
           *[other] A { $errors } fájlt nem sikerült elolvasni
        }
}
import-document-diagnostics-none = Nincs mit jelenteni.

versions-title = Verziók
versions-scope-synopsis = Szinopszis
versions-scope-prose = Szöveg
versions-loading = Átnézem a tartalékaidat.
versions-empty = Ennek még nincs korábbi változata.
versions-error = Semmi sem veszett el, de ez a lista lehet, hogy nem teljes.
versions-did-not-exist = A { $date }-en még nem léteztek
versions-deleted-after = Néhány évvel a { $date } után törölték
versions-unreadable = { $count ->
    [one] { $count } tartalék lehet olvasni
   *[other] { $count } backups could be read
}
versions-thinned = { $count ->
    [one] A projekt saját történelme már a { $count } korábbi állapotában csökkent.
   *[other] A projekt saját története már a { $count } korábbi állapotában csökkent.
}
versions-source-backup = Erősítés.
versions-source-project = A projekt saját története
versions-list-caption = Egy bejegyzés egy változtatás, nem egy tartalék
versions-pick-a-version = Válassza ki a változatot, hogy mi változott
versions-earliest = A legkorábbi verziót nem lehet összehasonlítani.
versions-no-change = Semmi sem változott ezen a részen.
versions-formatting-only = Csak a formázás változott itt. A szavak ugyanaz
versions-show-unchanged = Változatlan
versions-hide-unchanged = Változatlanul elrejtve
versions-next-change = Következő változás
versions-near = { $text } közelében
versions-words-added = { $count ->
    [one] { $count } szó hozzáadva
   *[other] { $count } szavak hozzáadása
}
versions-words-removed = { $count ->
    [one] { $count } szó eltávolítva
   *[other] { $count } szó eltávolítva
}
versions-blocks-moved = { $count ->
    [one] { $count } bekezdés áthelyezve
   *[other] { $count } bekezdés áthelyezve
}
versions-pin = Pin a biztonsági mentés ez a verzió jött. Automatikus tisztítás soha nem törli
versions-unpin = A mentés kikapcsolása ez a verzió jött. Automatikus tisztítás törölheti újra
versions-pinned-only = Csak a beállított verziók megjelenítése
versions-pin-note = A pin megtart egy fájlt, és a projekt saját története az egyes.
versions-pinned-empty = Itt még semmi sincs beszorulva.
versions-pinned-empty-log = Itt még semmi sincs bemetszve, csak az erősítés verzióit lehet beszögezni, a pin megtart egy fájlt, és a projekt saját története az egyes.
versions-range-filter = Csak két dátum közötti verziók megjelenítése
versions-filtered-empty = Nincs olyan verzió, ami megfelelne a szűrőknek.
versions-clear-filters = A szűrők törlése
versions-last-30-days = Az elmúlt 30 nap
versions-restore-button = @ info: whatsthis
versions-restore-confirm-title = A szöveg helyettesítése a { $date }-es verzióval?
versions-restore-confirm-text = Ami most van, azt a szöveg váltja fel, ami a { $date }-en volt.
versions-restore-confirm-with-comments = Ami most van, azt a szöveg váltja fel, ami a { $date }-en volt. { $count ->
    [one] A jelenlegi szövegben a { $count } észrevétel rögzítve van, és árva maradhat.
   *[other] A jelenlegi szövegben a { $count } észrevétel szerepel, és árva maradhat.
}
versions-restore-confirm-undo-note = A biztonsági mentés az első, és Ctrl + Z visszavonja ezt egy lépésben.
versions-restored-toast = Helyreállítva a { $date }-es változatot
versions-undo = Visszavonás
versions-restore-row-gone = Ez a tétel már nincs ebben a projektben.
versions-restore-no-home = Ez a tétel megváltozott típus azóta, és a régi szöveg nem megy sehova, hogy
versions-restore-no-safety-copy = A biztonsági mentés nem sikerült, így semmi sem változott.
versions-restore-failed = A helyreállítás sikertelen: { $error }
versions-restore-backup-busy = A tartalék már fut. Próbálja újra egy pillanat alatt.
versions-restore-in-backup-file = Megnézel egy biztonsági fájlt, és megnyitod a projektet, hogy helyreállítsd.
versions-restore-no-project = Egy projekt sincs megnyitva
versions-recreate-button = Hozd ezt vissza!
versions-recreate-picker-title = Hová kellene mennie?
versions-recreate-picker-empty = Ebben a projektben nincs olyan mappa, amibe bele lehetne rakni.
versions-recreate-picker-confirm = Hozd vissza ide!
versions-recreate-picker-cancel = Mégse
versions-recreate-untitled = Ez a tétel
versions-recreate-confirm-title = Visszahozni a { $item }-et?
versions-recreate-confirm-text = A { $destination }-et a { $date }-re vonatkozó szövegével egészítik ki.
versions-recreate-confirm-undo-note = Visszavonni az üzenetet, amit követünk, egyenesen kiveszi.
versions-recreated-toast = A { $item } újra a projektben van.
versions-recreated-partial-toast = { $count ->
    [one] A { $item } visszatért, de az egyik szöveget el lehetett olvasni.
   *[other] A { $item } visszatért, de a { $count } szöveget el lehetett olvasni.
}
versions-recreate-already-here = Ez a tétel már benne van a projektben.
versions-recreate-no-destination = Válassz valamit a mappában, hogy betedd.
versions-recreate-unreadable = A biztonsági mentést nem lehetett elolvasni, így nem adtak hozzá semmit.
versions-recreate-failed = Nem lehet visszahozni: { $error }
versions-changed-percent = Ennek { $percent }% -a megváltozott.
versions-hidden-paragraphs = { $count ->
    [one] - { $count } változatlan bekezdés.
   *[other] … { $count } változatlan bekezdések
}

timeline-title = Menj vissza az időben.
timeline-coverage = { $count ->
    [one] { $count } verzió rögzített, megy vissza { $oldest }
   *[other] { $count } változat rögzítve, visszatérve { $oldest }-re
}
timeline-loading = Átnézem a projektjét, a múltját.
timeline-empty = A projekt egyetlen változata sem került rögzítésre.
timeline-no-changes = Azóta semmi sem változott.
timeline-no-text-changes = Azóta egyetlen szöveg sem változott.
timeline-slider-label = Rögzített változat
timeline-series-name = A projekt mérete
timeline-bars-caption = Minden bár egy rögzített verzió, olyan magas, mint a projekt volt akkor. A kiemelten az, amit akkor néz.
timeline-bars-caption-periods = Túl sok a verzió, hogy egyenként mutassuk, így minden bár egy { $period }, olyan magas, mint a projekt végére.
timeline-unit-hour = óra
timeline-unit-day = nap
timeline-unit-week = hét
timeline-unit-month = hónap
timeline-range-filter = Csak két dátum közötti verziók megjelenítése
timeline-range-empty = Ezekben az időpontokban egyetlen változatot sem jegyeztek fel
timeline-open-period = Ezt az időszakot meg kell nyitni
timeline-show-all = Az egész történelem megjelenítése
timeline-changed-since = { $count ->
    [one] { $count } tétel különbözik a { $date } és a projekt most
   *[other] { $count } tételek különböznek { $date } és a projekt most
}
timeline-kind-added = Írva azóta
timeline-kind-removed = A projekt megszűnése
timeline-kind-changed = Szerkesztés azóta
timeline-kind-moved = Azóta költözött ide.
timeline-not-yet-written = Ez akkor még nem létezett.
timeline-no-text-of-its-own = Ennek nincs saját szövege.
timeline-reader-close = Bezárás
timeline-reader-stamp = Mint a { $date }-en.
timeline-reader-loading = A rögzített verzió megnyitása
timeline-reader-unreadable = Ezt a felvételt lehet, hogy elmozdították, törölték, vagy egy meghajtón van, ami kapcsolódott.
timeline-reader-compared = Ahogy a { $date }-en is volt, ellenezte, amit most mond.
timeline-reader-diff-legend = Azóta megszakadt, aláhúzták.
timeline-reader-view-label = Mit mutasson
timeline-reader-view-diff = Változások
timeline-reader-view-text = Szöveg
timeline-reader-deleted = Ez már nem a te projekted, elolvashatod és kimásolhatod innen.
timeline-prose-only-record = Ez a pont a projekt saját története, amely megtartja a szöveget, és semmi más, így mit töröltek vagy költözött, mivel lehet mutatni. Válasszon egy biztonsági mentést, hogy.

settings-sec-extensions = Kiterjedések

settings-group-games-forward = Mindig előre
settings-games-forward-toggle = Játszd mindig előre
settings-games-forward-hint =
    Amíg játszik, semmit nem írt vissza lehet venni: Backspace, Törlés,
    Vágás, drag- and-drop és undo minden le van tiltva az alábbi felületeken.
    Még mindig lehet gépelni, paszta, formátum és mozog szabadon. A vázlat csak növekszik.
settings-games-session-warning =
    Ez egy persession választás: ez soha nem mentett. Zárja be a projektet, vagy hagyja abba
    Skribisto, mindig véget ér a játék, és minden, amit írt közben a játék
    Megismételhetetlen, abban a pillanatban, amikor megtörténik.
settings-group-games-scope = Amennyiben alkalmazandó
settings-games-in-prose = Kézirat próza
settings-games-in-synopsis = Szinópok
settings-games-scope-hint =
    Hozzászólások, lábjegyzeteket és címeket soha nem fagyasztanak be: ezek azok, ahol a fix
    Megtiltottad magadnak, hogy készíts.
settings-games-inert-warning =
    Mindig előre van, de semmire sem vonatkozik.
    Vagy nem változtat semmin, ahogy írsz.

games-title = Írás játékok
games-forward-name = Mindig előre
games-forward-blurb =
    Draft anélkül, hogy bármit vissza. Törlés, vágás és megsemmisítése tiltva
    Írd meg a következő mondatot, ahelyett, hogy az utolsót javítanád.
games-forward-playing = Lejátszás: törlés letiltva
games-forward-idle = Nem játszik.
games-session-note = Akkor ér véget, amikor lezárod a projektet.
games-scope-prose-and-synopsis = A prózádra és a szövegeidre vonatkozik.
games-scope-prose = A prózádra is vonatkozik.
games-scope-synopsis = - A szinopszisodra vonatkozik.
games-scope-nothing = Még semmit sem jelent. Válasszon felületet a Beállítások között.
games-settings-link = A játékbeállítások írása
statusbar-games-forward = Mindig előre
statusbar-games-forward-tooltip =
    - Mindig előre van: a törlés, a vágás és a megsemmisítés nem működik, amíg Ön
    Draft. Kattintson, hogy hagyja abba a játékot.
goal-progress-words = { $g ->
   *[other] Szavak: { $count } / { $goal }
}
goal-progress-characters = { $g ->
   *[other] Karakterek: { $count } / { $goal }
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

inspector-goal = Cél
inspector-goal-none = Nincs célpont.

overview-col-goal = Cél
overview-excluded-from-export = Kimaradt az export, így számít, hogy nem teljes

goal-subtree-total-words = { $count ->
    [one] { $items } cél belül összesen { $words } szó
   *[other] { $items } cél belül összeadva { $words } szó
}
goal-subtree-total-characters = { $count ->
    [one] { $items } cél belül összesen { $words } karakter
   *[other] { $items } belső célok max. { $words } karakter
}

new-work-goal-unit = Számolj be.
new-work-goal-unit-hint = Használt minden szó vagy karakter cél ebben a projektben. Változó később.

settings-group-goal-unit = Célok
settings-goal-unit-switch-title = Számolóegység kapcsolása
settings-goal-unit-switch-text = Az erre a projektre már meghatározott célokat és mérföldköveket a { $from }-ben rögzítették. A { $to }-re történő váltás nem konvertálja azokat, így minden létező szám { $to }-nak fog tűnni.
settings-goal-unit-switch-informative = Semmi sem veszett el. Válts vissza bármikor, hogy visszaállítsa az eredeti leolvasást, majd frissítse a célokat szeretné tartani.
settings-goal-unit-switch-confirm = Váltás

goal-distribute-action = Differenciált
distribute-title = A cél felosztása
distribute-empty = Nincs benne semmi, ami megosztaná a célpontot.
distribute-weight = Megosztás
distribute-weight-length = Hosszúság
distribute-weight-rows = Darabonként
distribute-weight-even = Egyenletesen
distribute-overwrite = A már meghatározott célok pótlása
distribute-col-item = Tétel
distribute-col-current = Most
distribute-col-proposed = Után
distribute-total = Ezek összeadódnak a { $total }-gyel, szemben a { $goal }-es célponttal.
distribute-over-budget = A már beépített célpontok { $over }-be kerülnek, ami nagyobb, mint ez a konténer. Emelje a célpontot, alacsonyabban, vagy cserélje le őket.
distribute-apply = Megkülönböztetés
distribute-cancel = Mégse

milestone-target-gone = Cél törölve
milestone-no-target = Nincs célpont.
milestone-add = Hozzáadás
milestone-add-label = Mit kell elérni?

pace-summary-title = Where the book stand
pace-summary-remaining = { $words } elvitelre
pace-summary-open = A terv megnyitása
pace-summary-close = Bezárás
pace-summary-dont-show = Felnyitás közben ne jelenjen meg
pace-summary-menu = Írási terv

menu-help-topics = Súgó témakörök (&T)
menu-help-shortcuts = Billentyűzet Rövidítések (&K)
menu-help-website = Skribisto az interneten (&W)
menu-help-report = Jelentést kérek! (&R)
menu-help-check-updates = A frissítések ellenőrzése (&U)
menu-command-palette = Command Palette (&C)

learn-help-topics = A témakörök segítése
learn-shortcuts = Billentyűzetnyaláb
learn-website = Skribisto az interneten

help-window-title = Súgó
help-filter-topics = Szűrési témák
help-no-matching-topic = Nincs téma egyezés.
help-topic-missing = Ez a téma már nem elérhető.
help-back = Vissza
help-not-translated = Ez a cikk még nem érhető el magyarul. Angolul jelenik meg.

help-section-getting-started = Kezdődik.
help-section-writing = Írás
help-section-reviewing = Átnézés
help-section-exchanging = A szavak be- és kijuttatása
help-section-keeping = A projekt biztonsága
help-section-extensions = Kiterjedések

help-topic-getting-started = Az első projekt
help-topic-writing-model = Hogyan rakjuk össze a könyvet?
help-topic-drafts-and-old-versions = Egy régebbi tervezet megtartása
help-topic-goals-and-pace = Célok és tempó
help-topic-comments = Megjegyzések
help-topic-round-trip = Elküldöm a könyvet egy olvasónak.
help-topic-export = Exportálás
help-topic-import-documents = Importáló okmányok
help-topic-import-projects = Egy teljes projekt átvitele
help-topic-backups-and-versions = Biztonsági mentések és verziók

help-shortcuts-title = Billentyűzetnyaláb
help-shortcuts-filter = Szűrőnyeg
help-shortcuts-no-matches = Nincs rövidebb út.
help-shortcuts-rebind = A rövidítések módosítása
help-shortcuts-close = Bezárás

command-palette-placeholder = A parancs típusa

shortcut-name-binder-duplicate = Másolat készítése
shortcut-name-comments-add = Megjegyzés hozzáadása
shortcut-name-comments-add-paragraph = Megjegyzés a bekezdéshez
shortcut-name-spellcheck-toggle = A helyesírás ellenőrzése
shortcut-name-editor-tab-close = A lap bezárása
shortcut-name-editor-tab-pin = Pin vagy Unpin Tab
shortcut-name-editor-save = Mentés
shortcut-name-work-export = Exportálás…
shortcut-name-work-new = Új projekt
shortcut-name-work-open = Projekt megnyitása
shortcut-name-window-new = Új ablak
shortcut-name-work-close = Projekt bezárása
shortcut-name-app-settings = Beállítások
shortcut-name-app-quit = Kilépés
shortcut-name-editor-insert-footnote = A lábjegyzet beszúrása
shortcut-name-format-scene-break = Helyszín beszúrása
shortcut-name-format-major-scene-break = Helyezze be a nagyobb jelenet szünet
shortcut-name-format-link = Kapcsolat beszúrása
shortcut-name-format-mark-nbsp = Helyezze be a No- break szóközt
shortcut-name-format-mark-nbhyphen = A nem-törés hyphen beszúrása
shortcut-name-format-mark-nnbsp = Szűkség beszúrása N- break szóköz
shortcut-name-format-mark-zwsp = Zeroszélesség beszúrása
shortcut-name-go-next = Következő
shortcut-name-go-prev = Előző
shortcut-name-go-to = Menj!
shortcut-name-outline-toggle = A kimenet ki- be kapcsolása
shortcut-name-preview-toggle = Preview sáv ki- be kapcsolása
shortcut-name-view-fullscreen = Teljes képernyő ki- be kapcsolása
shortcut-name-view-focus-mode = Disztrakciómentes mód ki- be kapcsolása
shortcut-name-editor-size-increase = Szövegméret növelése
shortcut-name-editor-size-decrease = Szövegméret csökkentése
shortcut-name-editor-size-reset = Szövegméret újraindítása
shortcut-name-editor-find = Keresés
shortcut-name-editor-replace = Csere
shortcut-name-editor-find-next = Következő mérkőzés
shortcut-name-editor-find-prev = Előző mérkőzés
shortcut-name-search-show = Keresés a projektben
shortcut-name-search-replace = Pótlás a projektben
shortcut-name-outline-open-to-side = Megnyitás az oldalra
shortcut-name-help-topics = Súgó témakörök
shortcut-name-help-shortcuts = Billentyűzet billentyűparancsok
shortcut-name-help-website = Skribisto az interneten
shortcut-name-help-report = Probléma bejelentése
shortcut-name-help-check-updates = A frissítések ellenőrzése
update-check-behind = A { $version } verzió elérhető.
update-check-current = Te futtatod a legújabb verziót.
update-check-failed = Nem sikerült ellenőrizni az újabb verziót. { $error }
shortcut-name-command-palette = Parancspaletta
help-section-reference = Mik a dolgok?

help-concept-scene-break = Jelenettörés
help-concept-major-scene-break = Nagy jelenet kitörése
help-concept-find-in-prose = Keresés a prózában
help-concept-story-bible = Történetlexikon
help-concept-goal-unit = Szavak vagy karakterek
help-concept-goal-progress = Haladás
help-concept-manuscript-words = Mi számít kéziratnak?
help-concept-exportable = Kihagyva az exportból
help-concept-distribute = Megosztjuk a célpontot.
help-concept-subtree-total = A belső célpontok mit érnek el
help-concept-milestone = Mérföldkő
help-concept-pace-plan = Pace-terv

help-concept-tag = Címke
help-concept-label = Rövid leírás
help-concept-point-of-view = Nézőpont
help-concept-epigraph = Mottó
help-concept-footnote = Lábjegyzet
help-concept-chapter-mode = Árucsoport
help-concept-comment = Megjegyzés
help-concept-backup = Biztonsági másolat
help-concept-version = Verzió
help-concept-trash = Kuka
help-concept-spellcheck = A helyesírás ellenőrzése
help-concept-search-replace = Keresés és helyettesítés
help-concept-note-template = Megjegyzés sablon
help-concept-story-bible-entry = Történet bibliai bejegyzés
help-concept-text-replacement = Szövegcsere
help-concept-smart-punctuation = Intelligens pontozás
help-concept-export-style = Exportstílus
help-concept-round-trip-marks = Kerekes útjelzők

epigraph-mark-attribution = Forrásvonal
epigraph-mark-attribution-tip = Jelölje meg azt a vonalat, ahol a gondozó van, mint az idézet forrása, így a nyomtatás, mint a hozzárendelés
margin-lane-name = Határértékek
margin-lane-provider-comments = Megjegyzések
margin-lane-provider-story-bible = Történet bibliai bejegyzés
margin-lane-provider-story-bible-hint = Egy megjegyzés: A próza olvasása, ahol a bejegyzés nevét, és a jelenetek mondta a szemén keresztül. Szintén megjelöli a neveket magát a prózában.
margin-lane-mark-point-of-view = Innen már mondtam.
margin-lane-mark-named-here = Itt nevezték el.
margin-lane-provider-comments-hint = Ha a szöveghez egy megjegyzést csatolnak
margin-lane-provider-search = Keresés
margin-lane-provider-search-hint = Minden találatot, amit legutóbb keresett, bárhol is van a dokumentumban.
margin-lane-provider-boundaries = Amikor minden dokumentum elindul
margin-lane-provider-boundaries-hint = Egy szabály minden jelenet tetején egy patakban, hogy láthasd, hol vagy
margin-lane-provider-spelling = Betűzés
margin-lane-provider-spelling-hint = Ahol a varázs ellenőrzője megjelölte a szót, kivéve, ha azt kéri: ez jelzi a gép véleményét a próza
margin-lane-spelling = { $word }, lehetséges téves helyesírás
margin-lane-search-hit = { $text }, mérkőzés { $index } of { $total }
margin-lane-search-current = { $text }, mérkőzés { $index } a { $total }, az egyik Ön
margin-lane-boundary = A { $title } kezdete
margin-lane-boundary-untitled = egy meg nem nevezett dokumentum
settings-page-margin-lane = Határértékek
settings-desc-margin-lane = A szalag a tekercs bár mellett, és amit mutat
settings-margin-lane-enabled = A peremsáv megjelenítése
settings-margin-lane-enabled-hint = Ugyanaz a kapcsoló, mint a View Margin jelek.
settings-margin-lane-enabled-more = Soha nem mondja, hogy valami baj van: megmutatja, hogy hol vannak a dolgok, és hagyja, hogy mit jelent ez neked.
settings-group-margin-lane-marks = Mit jelöl?
settings-margin-lane-no-providers = Még semmi sem jelzi a sávot.
settings-group-margin-lane-texture = Párbeszéd szerkezete
settings-margin-lane-texture = A párbeszéd szerkezetének megjelenítése
settings-margin-lane-texture-hint = Egy bar / bekezdés: mennyi idő, és mennyit beszélnek belőle.
settings-margin-lane-texture-more = Nem mérik olyan nyelveken, ahol nincs kurált konvenció, ahol a bár csak hosszúságot mutat.
settings-group-margin-lane-surfaces = Ahol úgy tűnik.
settings-margin-lane-surface-editor = Szövegszerkesztő
settings-margin-lane-surface-stream = Szövegfolyamok
settings-margin-lane-surface-search-preview = Keresési előnézet
menu-margin-lane = Határértékek (&M)

status-preset-drafting = Vázlatírás
status-preset-passes = Felülvizsgálati engedélyek
status-preset-plume = Plume Creator
status-todo = Elvégzendő
status-draft = Piszkozat
status-revised = Átdolgozott
status-final = Végleges
status-outline = Vázlat
status-first-edit = 1. szerkesztő
status-second-edit = 2. szerkesztés
status-done = Kész
status-plume-draft-1 = Első tervezet
status-plume-draft-2 = Második tervezet
status-plume-draft-3 = 3. tervezet
status-plume-edit-1 = 1. szerkesztő
status-plume-edit-2 = 2. szerkesztés
status-plume-edit-3 = 3. szerkesztés
status-plume-proofread = Korrektúrázva
status-plume-finished = Befejezett
new-work-statuses = Munkafolyamat
new-work-statuses-hint = A szakaszok mozgatja a jelenetet. Lehet átnevezni, átsorolni, vagy hozzá ezeket bármikor.
status-none = Nincs státusz
inspector-status = Állapot
overview-col-status = Állapot
overview-status-mixed = az alábbi részek nem értenek egyet
help-concept-status = Állapot
status-completion-title = Where the book stand
status-completion-headline = { $done } of { $total } jelenetek kész
status-completion-empty = Még nincsenek jelenetek, ez olvassa a kéziratot, így kitölti, ahogy írod.
status-completion-open = Hol áll a könyv?

settings-page-statuses = Állapot
settings-desc-statuses = Átnevez, rendel, hozzáad, és távolítsa el a szakaszok mozgatja a jelenetet
settings-statuses-add = Állapot hozzáadása
settings-statuses-add-placeholder = Új státus neve
settings-statuses-desc =
    A szakaszok a jelenet mozog keresztül, sorrendben, a legkevésbé befejezett a tetején a legtöbb
    Ez a rendelés nem jelenti azt, hogy minden más.
    az applikációban, szóval tedd őket úgy, ahogy a folyamat fut.
settings-statuses-apply-preset = Előzetes beállítás alkalmazása
settings-statuses-preset-applied = { $added ->
    [one] 1 státusz hozzáadva
   *[other] Hozzáadott status { $added }
}
settings-statuses-preset-refused = Ennek a projektnek már van létrája, csak egy üres lapját tölti ki. Törölje azokat a rudakat, amelyeket nem akar először.
settings-statuses-duplicate = A { $name } már ezen a létrán van.
settings-statuses-added = Hozzáadva { $name }
settings-statuses-move-up = Felfelé (kevesebb kész)
settings-statuses-move-down = Lejjebb (több kész)
settings-statuses-details-placeholder = Mit jelent ez a szakasz (nem kötelező)
settings-statuses-delete = Törlés { $name }
settings-statuses-delete-in-use = { $count ->
    [one] Törlés { $name }
   *[other] Törlés{ $name }": { $count } elemek vannak rajta, és elveszíti a státuszát
}
settings-statuses-deleted = Eltávolított { $name }
settings-statuses-deleted-in-use = { $count ->
    [one] Eltávolított { $name }. 1 tétel már nincs státusz.
   *[other] A { $name }, { $count } tételnek már nincs státusa.
}
settings-statuses-empty-title = Még nincs munkafolyamat.
settings-statuses-empty-body = A státusz azt mutatja, milyen messze van a jelenet. Add hozzá egy fenti, vagy kezdődik egy előre beállított.
status-category-planned = Tervezett
status-category-drafting = Vázlatírás
status-category-needs-work = Átdolgozást igényel
status-category-revised = Átdolgozott
status-category-final = Végleges
