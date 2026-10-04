# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

wm-book = Egy könyv a projektben.
wm-book-more = A könyv olyan egyszerű, vagy olyan réteges, mint tetszik. Egy rövid történet lehet csak néhány [jelenetek](:wm-scene). Egy hosszabb projekt fészkelheti [alkatrészek](:wm-part) és [fejezetek](:wm-chapter), amennyire csak akarod. Az alakja a tiéd.
wm-part = A fejezetek egy része.
wm-part-more = Egy rész a [fejezetek](:wm-chapter)-et egy címsor alatt foglalja össze. Mint egy fejezet, egészen a [Könyv](:wm-book) következő részéig vagy végéig tart.
wm-chapter = A könyv egy fejezete.
wm-chapter-more = Egy fejezet innentől a következő fejezetig, a következő [Rész](:wm-part), vagy a [a könyv vége](:wm-end-of-book). Két formában jön: egy fejezet mappa, amely rendelkezik [jelenetek](:wm-scene), vagy egy lapos fejezet, amely a saját prózáját. Válassza ki, melyik új fejezetet használja a Beállítások alatt.
wm-scene = A próza jelenete.
wm-scene-more = Minden írásnak két oldala van: a fő szöveg és a [Szinopszis](:wm-synopsis) a tervezéshez. Mindkettő együtt nyílik a szerkesztőben.
wm-note = Egy üzenet.
wm-note-more = A jegyzet tartja szöveg és a [Szinopszis](:wm-synopsis), mint egy [Jelenet](:wm-scene), a kutatás, aszidek, vagy emlékeztetők. Bármely elem lehet része az összeállított könyv, beleértve egy jegyzet ül belsejében [Fejezet](:wm-chapter).
wm-note-folder = Egy megjegyzés, ami más tárgyakat is tartalmaz.
wm-note-folder-more = A [Jegyzet](:wm-note), amely tartalmazza a termékek alatta, így össze lehet csoportosítani a kapcsolódó jegyzetek. Megtartja a saját [Szinopszis](:wm-synopsis), és a tételek belül megőrzi a saját szöveget.
wm-folder = Szervező mappa.
wm-folder-more = A konténer rendezésére tételek a mappában, de megfelel Önnek. Ez hordoz egy [Szinopszis](:wm-synopsis), de nem próza saját. A tételek belsejében tartja az írást.
wm-end-of-book = Jelölések, ahol egy könyv véget ér.
wm-end-of-book-more = Minden könyv osztozik egy folyamatos listán, így a könyv vége nem dönt fészkelés. Ez a jelző azt mondja, hogy a [Könyv](:wm-book) megáll itt. Bármi után tartozik a következő könyv.
wm-story-bible-entry = Egy felcímkézett, elidegenített megjegyzés a történetéről.
wm-story-bible-entry-more = A [Jegyzet](:wm-note) egy karakter, egy hely, vagy bármi más érdemes benyújtani, létrehozott egy név, címkék, álnevek és a sablon minden lépésben. Semmi az oldalon megkülönbözteti azt után, mivel marad egy hétköznapi megjegyzés lehet átnevezni, újragondolni vagy átalakítani, mint bármely más.
wm-synopsis = Egy írás összegzése.
wm-synopsis-more = Egy összefoglaló csatolva minden írási elem, általában egy rövid bekezdés, de ameddig csak tetszik. Használja azt a tervezés és navigáció előtt a próza létezik. Ez mellett áll a fő szöveg a szerkesztő.

scene-break-minor = Egy átlagos jelenet szünet: idő, hely vagy nézőpont eltolódása egy fejezeten belül.
scene-break-minor-more =
    Marks a break *where you put*, including in the middle of a [Jelenet](:wm-scene). Spliting prose into two article is a organization choice, so it never create a break of own.

    A prózába gépelve három, terekkel elválasztott csillag. Az ujjlenyomatokat az exportstílus határozza meg, nem az, hogy mit gépel: egy Shunn kézirat egyetlen #, egy kereskedelmi papír, mint dinkus, és a legtöbb francia, német, spanyol, orosz és olasz kiadvány, mint egy üres rés, semmi jel.
scene-break-major = Egy erősebb megosztottság: egy nagy időkihagyás, vagy egy határozott nézőpont-változás.
scene-break-major-more =
    Ugyanaz az ötlet, mint egy átlagos jelenet szünet, egy lépés felfelé. Használd, amikor egy sima szünet alulértékelné az ugrást.

    A Shunn standard kézirat-formátum egyetlen # ellen # # # -t állít pontosan erre a megkülönböztetésre. Ahol a hagyománynak nincs erősebb jele, mindkét szint ugyanazt nyomtatja.
wm-find-in-prose = Skribisto keresi ezt a tételt a prózában.
wm-find-in-prose-more = Kapcsold be ezt a címkét, hogy a nevet, amiről írsz: karakterek, helyek, objektumok. Minden elem, amely egy ilyen címkét tart a próza a címe és a többi nevet ad neki, így minden [Jelenet](:wm-scene) listák, hogy ki és mi jelenik meg, anélkül, hogy kapcsolna valamit kézzel. Hagyd ki a címkék, hogy leírjon egy elemet, nem egy nevet, mint például egy vázlat státusz vagy emlékeztető, hogy ellenőrizze a folytonosságot. Minden elem, amely egyezik is csatlakozik a [Történetlexikon](:wm-story-bible).
wm-story-bible = Minden karakter, hely és egyéb jelölt bejegyzés, egy rácson belül.
wm-story-bible-more = Megnyitás minden jegyzetből mappák saját tab. Kártyák csoportonként tag, és minden név, hogy hány álnevet egy bejegyzés választ, és hány jelenetek jelent meg, lehívott minden [prózában található](:wm-find-in-prose) tag a projekt. Szűrés könyv csak szűkek, amelyek a kártyák mutatják; ez soha nem dönti el, hogy mely bejegyzések léteznek.

tooltip-go-to = Ugrás minden elemre a mappában (Ctrl + G)
synopsis-collapse-tooltip = A szinopszis oszlop elrejtése

wm-paratext = Egy szöveg, ami nem része a történetnek.
wm-paratext-more = Egy előlap, egy elkötelezettség, egy utószó, egy kolofon: írás, ami a könyvhöz tartozik, de nem a testéhez. Kivitted, ahova tetted, és soha nem számoltad meg a kézirat szó számít. Ahol megy, rajtad múlik: a konvenciók országonként és kiadónként különböznek.
wm-paratext-folder = Egy mappa paratexthez.
wm-paratext-folder-more = Valahol, hogy tartsa előtagok és utószavak, hogy ne zavarják a mappát. Szervezése csak: ez hordozza a [Szinopszis](:wm-synopsis), de nem ad semmit az exportált könyv, még a saját nevét.

goal-target = Mennyi ideig tart ez a darab?
goal-target-more = A cél a következő: a [Jelenet](:wm-scene), a [Fejezet](:wm-chapter), a [Rész](:wm-part), az egész [Könyv](:wm-book). Mindegyik a saját száma, és áll a saját, így a cél egy fejezetre soha nem változtatja meg a könyvet, és a könyv cél soha nem egy összeg a fejezetek benne. Mit kell hozzáadni az írás: a konténer [Haladás](:goal-progress) van minden alatta. Hagyja a cél nulla, és egyszerűen meg van írva. A szám számít [szavak vagy karakterek](:goal-unit), függetlenül attól, hogy ez a projekt használja.
goal-unit = Akár szavakban, akár karakterekben számít ez a projekt.
goal-unit-more = A projekt széles választéka, mert egy hosszú egység inkább a kézirathoz és annak piacához tartozik, semmint egy részhez: német és francia kiadói intézkedés jelekben, japán 400 karakteres lapok, kínai az ezer karakter, a legtöbb angol nyelvű kiadvány. Mindkét számot egymás mellett tartják, és egyik sem alakítható át a másikba, így az egység pontjait minden [Cél](:goal-target)-re váltják, és az elsőt visszaállítják. Amit nem tesz, az átalakít bármit, így a már megadott célokat kézzel kell frissíteni.
goal-progress = Mennyi van megírva, a célpont ellen.
goal-progress-explained = Egy saját prózával rendelkező darabot a saját hosszához mértek; egy tartályt a [Minden alatta van.](:goal-manuscript-words)-hez. A rúd a vörös és borostyánsárga között a zöldre melegszik, amint az írás beérkezik, és újra megváltozik, amint a darab elfut a tervezett, ami érdemes észrevenni, ha egy fejezetnek van egy hossza, hogy tartsa.
goal-manuscript-words = Ami a kéziratnak számít.
goal-manuscript-words-more = Csak próza, hogy valóban exportálni: [jelenetek](:wm-scene) és [fejezetek](:wm-chapter) saját szövegével. A [Jegyzet](:wm-note) vagy [Paratextus](:wm-paratext) nem része a kézirat, és soha nem számít egy. Bármi a szemét kiesik, és így csinál minden [kimaradt az exportból](:goal-exportable).
goal-exportable = Ez a tétel kimaradt az exportból.
goal-exportable-more = A saját hossza még mindig mutatja, mert az írás még mindig ott van, de számít, hogy nem teljes, pontosan, mint ez fog megjelenni nem exportált könyv. Kihagyva egy konténer nem hagyja ki a darabokat benne: minden egyes elem válaszol magának, ami az, amit a duplair a gyermekek mellett a kapcsoló.
goal-distribute = Oszd meg a célpontot a bent lévők között.
goal-distribute-more = Split egy [könyv](:wm-book) vagy [rész](:wm-part) cél a közvetlen gyermekek, így a darabok összeadódnak, hogy pontosan. Alapértelmezés szerint csak azokat tölti ki, amelyeknek még nincs céljuk, és hagyja a saját számok egyedül. Amit ír, a szokásos [célok](:goal-target), szabadon szerkeszthető, és szabadon sodródik egymástól: ez egy intézkedés, nem pedig állandó megállapodás.
goal-subtree-total = Amihez a belső célpontok hozzáadnak.
goal-subtree-total-more = Tény, nem egy cél. Ez a projekt soha nem kezeli a konténer cél, mint az összeg a benne lévők, mert ez az, hogy egy mappa figura végül mozog a saját, amikor egy jelenet belsejében adnak egy számot. Ez a sor egyszerűen megmondja, hogy mi a saját számok jönnek, így lehet összehasonlítani a konténer [Cél](:goal-target) magad, vagy [Osszon egyet.](:goal-distribute).
goal-milestone = Egy dátum, hogy elérjünk valamit.
goal-milestone-more = Két fajta. Az egyik azt mondja, hogy a [Könyv](:wm-book) kell állni egy adott hosszúságban egy dátum, egy menetpont a saját görbén. A másik azt mondja, hogy egy bizonyos [Fejezet](:wm-chapter) vagy [Rész](:wm-part) kellett volna elérni [saját célja](:goal-target) addigra, és olvassa el ezt a számot élőben, ahelyett, hogy egy másolatot, így szerkeszteni a cél frissíti a mérföldkövet is. Mindkettő megjelenik a könyv [ütemterv](:pace-plan).
pace-plan = A könyv írási ütemterve.
pace-plan-more = A [Cél](:goal-target) és a határidő, a napok, amire írni, fordult, hogy mennyi egy nap szüksége van. Elolvassa a könyv a saját cél, így a beállítás itt és a beállítás a felügyelő két helyen ugyanaz a szám. Waypoints mentén az út [mérföldkövek](:goal-milestone).

concept-tag = Egy színkódos címke, amit bármilyen tételre feltehetsz, az egész projektben újra felhasználható.
concept-tag-more = Tag egy elem, hogy jelezze: egy állapot, egy hely, egy szál, amit követ. Ugyanaz a tag tud ülni bármilyen számú elem, és egy elem lehet vinni több. Kapcsolja be [prózában található](:wm-find-in-prose) egy tag, amely megnevezi valamit a könyvben, egy karakter vagy egy hely, és Skribisto elkezd keresni azt a prózában.
concept-label = Egy rövid megjegyzés, amit a saját szemedre írsz egy cikk címére.
concept-label-more = Nem [Címke](:concept-tag): egy címke csak egy tételhez tartozik, egyszerű szöveg színezés vagy katalógus nélkül mögötte, valami olyan, mint az első telek pont, vagy a "nem" nevet kell használni. Állítsuk be a "pont" menüből, vagy vágjuk bele az "Overview" címsor rovatába; a cím alatt, a vázlatban és a patakban is kis felirattal látható.
concept-point-of-view = Akinek a szeme egy jelenetről szól: egy vagy több történeted van.
concept-point-of-view-more = A Felügyelő, a gipsz mellett. Válasszon ki valakit, aki még nem szerepel a gipszben, ott is hozzáteszi őket. Több mint egy nézőpont egy jogi választás, hasznos egy közös jelenet vagy a szemcsere középkönyv, nem valami Skribisto blokkok. A jelöltek jönnek a tételek egy [prózában található](:wm-find-in-prose) tag teszi a történet bibliát.
concept-epigraph = Egy adott rész vagy árucsoport fejéhez rögzített idézet.
concept-epigraph-more = Saját területe, a [Fejezet](:wm-chapter) vagy a [Rész](:wm-part) saját prózájától elkülönítve; a [Jelenet](:wm-scene) vagy a [Jegyzet](:wm-note) nem tartalmaz semmit. Írjon egyet, mint egy rendes idézetet; írjon néhányat és mindkettőnek saját blokáriuma lesz, saját feliratként nyomtatva. Szavait soha nem számolja kézírásnak.
concept-footnote = Egy idézet a prózában, számozott jegyként.
concept-footnote-more = A hivatkozás egyetlen jel belül a szavakat, így mozog és törli pontosan, mint a szavak körül teszi. A szám soha nem tárolja: Skribisto Projekt ki frissen, ahonnan minden hivatkozás áll az egész kézirat, így behelyezve egy átszámolókat minden megjegyzés után. Egy lábjegyzet a saját szavakat számolja el a kézirat, soha nem hajtotta bele.
concept-chapter-mode = Hogy egy új fejezet egy jelenetek mappája vagy egy lapos elem.
concept-chapter-mode-more = Beállítja a Beállítások, Structure. Ön írja közvetlenül egy [Fejezet](:wm-chapter) így is; az egyetlen különbség, hogy ez is tartalmaz jeleneteket alatta. Mindkét kódolások áll, hogy ugyanaz a könyv, és akkor keverjük össze szabadon egy projekt. Promote átalakítja a meglévő fejezet között két anélkül, hogy elveszítené egy szót.
concept-comment = Egy megjegyzés, ami pontosan egy próza-szakaszhoz van rögzítve, a peremen látható.
concept-comment-more = Emlékezik a szavakat mutat, és mi körülveszi őket, nem egy fix pozíció, így túléli a rendes szerkesztés és a teljes újratöltés. Válasz alatt tartani egy szál megy. Ha a pontos idézet már nem található, a megjegyzés azt mondja, hogy ahelyett, hogy csendben sodródik a rossz mondat.
concept-backup = Az egész projekt pontos és pontos másolata, mellette tartva.
concept-backup-more = Az Ön által választott ütemterv: nyitott, közel, egy intervallum, vagy bármilyen keverék, régi példányok metszett kor vagy szám szerint. Megnyitás egy terhelés, hogy a saját ablakban: meg lehet nézni, és akár szerkeszteni, de csak Save As vagy Restare megtart mindent. Különbözik a [változat](:concept-version), amely nyomon követi az egyik elem a saját múltját.
concept-version = Egy tétel a saját múltja: minden változás a szöveg vagy a szinopszis.
concept-version-more = Épült a projekt saját mentési történelem és a biztonsági mentések együtt, egy bejegyzés egy változtatás helyett egy fájl. Válasszon ki egy bejegyzést, hogy mi változott, és helyreállítja csak az elem a szöveg, hogy a projekt többi része érintetlen, és a biztonsági másolatot először. Eltérve a [Biztonsági másolat](:concept-backup), amely egy másolat minden.
concept-trash = A roncsolt tárgy el van rejtve, nem mozog: pontosan ott marad, ahol volt.
concept-trash-more = Trashing csak egy zászló: a termék tartja helyét a mappában, jelölve inaktív helyett eltávolítani. Helyezze vissza, és újra megjelenik, hogy pontosan ott, nem újra-regisztráció csinálni. Üres Trash eltávolítja a roncsokat a jó, és ezek mindegyike egy hajthatatlan lépés, mint bármely más projekt szerkesztése.
concept-spellcheck = Alsó szavak nincs telepített szótár felismeri, projektnyelvenként.
concept-spellcheck-more = Minden projektnek megvan a saját munkanyelve vagy nyelvei; egy termék felülbírálhatja őket egy másik nyelven írt jelenetért. A Skribisto szó hozzáadása nem ismert a szerkesztő kontextusából egy személyes szótárba, amely átmegy minden projektre. A helyesírás kikapcsolása mindenhol megáll, amíg vissza nem kapcsolod.
concept-search-replace = A projektben mindenhol talál egy szót vagy kifejezést, nem csak a nyílt dokumentumot.
concept-search-replace-more = Átnéz minden jelenetet, jegyzetet, címet, szinopszist és címkét, és megjegyzéseket és lábjegyzeteket is. Case, teljes szó egyezés és akcentus egyezés három külön kapcsolót. Helyettesíti minden átírt mérkőzés egy passz és egy visszavonási lépés, így Ctrl + Z veszi vissza az egész tételt, nem csak az utolsó változás.
concept-note-template = Egy újrahasználható írás, amit beleteszel bármibe, amin dolgozol.
concept-note-template-more = Egy üres karakter lap, egy helyszín profil, egy beat lap: írja egyszer, és dobja be, ahol szüksége van rá, minden elem, nem csak egy megjegyzés. Kezdje egy built-in preset, import .md vagy .djot fájlt, vagy írjon valamit, és válassza Document Save sablon. Elrejtette a projektben, így egy társszerző, aki megnyitja azt kap ugyanazt a készletet.
concept-text-replacement = A saját rövidítése, automatikusan bővült, ahogy gépel.
concept-text-replacement-more = Beállítja a ravaszt, és annak helyettesítése, akkor btw) az út mentén, és kiterjeszti a pillanatot, amikor gépel egy helyet vagy poctuation után. Az eset azt követi, amit gépelt: tőkésítse a ravaszt, és a bővítés kapitalizes is. A Backspace után a bővítés csak azt az egy helyettesítést undoes. Becserélt projektenként, elhatárolva [intelligens pontozás](:concept-smart-punctuation).
concept-smart-punctuation = Localeaware tipográfia automatikusan alkalmazott, mint a típus: idézetek, dash, ellipszis.
concept-smart-punctuation-more = A Curls egyenes idézőjelben tipográfiává alakul, aztán -- egy i-dash-é, egy ellipszissé alakul, és ahol bekapcsolod, egy olyan bekezdést nyit meg, amelyet párbeszédkötővel gépeltek be. Projektenként beállít, és a .skrib-en belül utazik, így egy társszerző azonos szabályokkal ír. Távolság a [a saját szabályaid](:concept-text-replacement)-től, amit te magad írsz.
concept-export-style = A megnevezett csomag export választás: címek, osztás, mi lesz benne.
concept-export-style-more = Egy stílus, újra használt minden export bármilyen formátumot választotok. Az épület-in stílusok read- csak; másolja meg az egyik, hogy a kiindulópont lehet ténylegesen változtatni. Között a választás, hogy tartalmazza [körutazásjelzők](:concept-round-trip-marks), egy fájl kötött egy szerkesztő és vissza.
concept-round-trip-marks = Láthatatlan könyvjelzők, amelyek lehetővé teszik egy szerkesztett DOCX vagy ODT jön haza elismert.
concept-round-trip-marks-more = Írta csak a két formátumba, amit egy szerkesztő küldhet vissza, DOCX és ODT; minden más export nem szállít. Könyvjelző, nem egyedi attribútum: LibreOffice csík egyedi attribútumok mentésére, de hagyja egy könyvjelző érintetlen. Bekapcsolva per [exportálási stílus](:concept-export-style), alatta dedikált.
concept-status = Ahol egy elem a saját munkafolyamatában áll: egyszerre egy szakaszban, haladva előre.
concept-status-more = Nem [Címke](:concept-tag): egy tétel pontosan egy státusszal rendelkezik, a szakaszok megrendelésre kerülnek, és a projekt tulajdonosa a lista, így lehet átnevezni, újra, vagy hozzá bármikor. Egy csillag mellett egy fejezet vagy könyv azt jelenti, hogy az alatta lévő részek nem minden egyetért vele. Állítsa be a felügyelő, egy kártya, egy stream elem, vagy itt.
