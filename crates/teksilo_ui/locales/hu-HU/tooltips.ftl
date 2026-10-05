# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

wm-book = Egy könyv a projektben.
wm-book-more = Egy könyv lehet olyan egyszerű vagy olyan tagolt, amilyennek szeretné. Egy novella állhat néhány [jelenetből](:wm-scene). Egy hosszabb mű [részeket](:wm-part) és [fejezeteket](:wm-chapter) ágyazhat egymásba, amilyen mélyen csak akarja. A forma az Öné.
wm-part = Fejezeteket csoportosító rész.
wm-part-more = Egy rész egyetlen cím alatt gyűjti össze a [fejezeteket](:wm-chapter). A fejezethez hasonlóan a következő részig vagy a [könyv](:wm-book) végéig tart.
wm-chapter = A könyv egy fejezete.
wm-chapter-more = Egy fejezet innen a következő fejezetig, a következő [részig](:wm-part) vagy a [könyv végéig](:wm-end-of-book) tart. Kétféle formája van: [jeleneteket](:wm-scene) tartalmazó fejezetmappa, vagy saját prózát hordozó egyszerű fejezet. Hogy az új fejezetek melyiket használják, azt a Beállítások Szerkezet oldalán választhatja ki.
wm-scene = Egy prózai jelenet.
wm-scene-more = A jelenet tartalmazza az írását. Minden írási elemnek két oldala van: a főszöveg és a tervezéshez használt [szinopszis](:wm-synopsis). A szerkesztőben mindkettő együtt nyílik meg.
wm-note = Egy jegyzet.
wm-note-more = A jegyzet a [jelenethez](:wm-scene) hasonlóan szöveget és [szinopszist](:wm-synopsis) tartalmaz, kutatáshoz, kitérőkhöz vagy emlékeztetőkhöz. Bármely elem része lehet az összeállított könyvnek, egy [fejezeten](:wm-chapter) belül álló jegyzet is.
wm-note-folder = Más elemeket tartalmazó jegyzet.
wm-note-folder-more = Olyan [jegyzet](:wm-note), amely alatta további elemeket is tartalmaz, így összetartozó jegyzeteket csoportosíthat. Saját [szinopszisa](:wm-synopsis) van, a benne lévő elemek pedig megtartják a saját szövegüket.
wm-folder = Rendszerező mappa.
wm-folder-more = Tároló, amellyel tetszése szerint rendezheti az elemeket az iratrendezőben. Van [szinopszisa](:wm-synopsis), de saját prózája nincs. Az írást a benne lévő elemek tartalmazzák.
wm-end-of-book = Egy könyv végét jelöli.
wm-end-of-book-more = Minden könyv egyetlen folyamatos listán osztozik, így a könyv végét nem a beágyazás határozza meg. Ez a jelölő mondja meg, hogy a [könyv](:wm-book) itt ér véget. Ami utána jön, a következő könyvhöz tartozik.
wm-story-bible-entry = Címkézett, álnevekkel ellátott jegyzet a történetéről.
wm-story-bible-entry-more = [Jegyzet](:wm-note) egy szereplőről, helyszínről vagy bármi másról, amit érdemes nyilvántartani; egyetlen lépésben jön létre névvel, címkékkel, álnevekkel és kezdősablonnal. Utána semmi sem különbözteti meg az oldalon: közönséges jegyzet marad, amelyet ugyanúgy átnevezhet, átcímkézhet vagy átalakíthat, mint bármely mást.
wm-synopsis = Egy írási elem összefoglalója.
wm-synopsis-more = Bármely írási elemhez kapcsolódó összefoglaló, általában egy rövid bekezdés, de tetszőleges hosszúságú lehet. Arra való, hogy már a próza megszületése előtt tervezhessen és tájékozódhasson vele. A szerkesztőben a főszöveg mellett áll.
scene-break-minor = Közönséges jelenettörés: idő-, hely- vagy nézőpontváltás egy fejezeten belül.
scene-break-minor-more =
    Törést jelöl *ott, ahová teszi*, akár egy [jelenet](:wm-scene) közepén is. A próza két elemre bontása rendszerezési döntés, így önmagában sosem hoz létre törést.

    A prózába három, szóközzel elválasztott csillagként gépeli be. Hogy nyomtatásban hogyan jelenik meg, azt az exportstílus dönti el, nem az, amit begépel: a Shunn-féle kézirat egyetlen #-ként, a kereskedelmi kiadás díszjelként szedi, a legtöbb francia, német, spanyol, orosz és olasz kiadó pedig jel nélküli üres közként.
scene-break-major = Erősebb tagolás: nagy időugrás vagy döntő nézőpontváltás.
scene-break-major-more =
    Ugyanaz, mint a közönséges jelenettörés, egy fokkal erősebben. Akkor használja, ha egy egyszerű törés alulértékelné az ugrást.

    Begépelve # # #, és az exportstílus a közönséges szinttől eltérően nyomtatja. A Shunn-féle szabványos kéziratformátum pontosan ennek a megkülönböztetésnek a kedvéért szed egyetlen #-et a # # # helyett. Ahol egy hagyománynak nincs erősebb jele, a két szint egyformán jelenik meg.
wm-find-in-prose = A Skribisto keresi ezt az elemet a prózájában.
wm-find-in-prose-more = Kapcsolja be azoknál a címkéknél, amelyek olyasmit neveznek meg, amiről ír: szereplőket, helyszíneket, tárgyakat. Az ilyen címkét viselő elemeket a program a címük és az Ön által adott többi nevük alapján megkeresi a prózában, így minden [jelenet](:wm-scene) felsorolja, ki és mi szerepel benne, anélkül hogy bármit kézzel kellene összekapcsolnia. Hagyja kikapcsolva azoknál a címkéknél, amelyek leírnak egy elemet, nem megneveznek valamit, például egy piszkozat állapotát vagy egy emlékeztetőt a folytonosság ellenőrzésére. Minden egyező elem a [történetbibliába](:wm-story-bible) is bekerül.
wm-story-bible = Minden szereplő, helyszín és más címkézett bejegyzés egyetlen rácsban.
wm-story-bible-more = Bármely jegyzetmappa saját lapjáról megnyitható. A kártyák címkék szerint csoportosulnak, és mindegyik megmutatja, hány álnévre hallgat a bejegyzés, és hány jelenetben bukkant fel, a projekt összes [prózában keresett](:wm-find-in-prose) címkéje alapján. A könyv szerinti szűrés csak azt szűkíti, mely kártyák látszanak; azt sosem dönti el, mely bejegyzések léteznek.
tooltip-go-to = Ugrás az iratrendező bármely elemére (Ctrl+G)
synopsis-collapse-tooltip = A szinopszishasáb elrejtése
wm-paratext = Szöveg, amely nem része a történetnek.
wm-paratext-more = Előszó, ajánlás, utószó, kolofon: olyan írás, amely a könyvhöz tartozik, de nem a törzséhez. Ott exportálódik, ahová teszi, és sosem számít bele a kézirat szószámába. Hogy hová kerül, az Önön múlik: a szokások országonként és kiadónként eltérnek.
wm-paratext-folder = Mappa paratextusokhoz.
wm-paratext-folder-more = Hely az előszavak és utószavak tárolására, hogy ne zsúfolják az iratrendezőt. Csak rendszerezésre szolgál: van [szinopszisa](:wm-synopsis), de semmit sem tesz hozzá az exportált könyvhöz, még a saját nevét sem.
goal-target = Milyen hosszúra tervezi ezt a darabot.
goal-target-more = Bármire beállíthat célértéket: egy [jelenetre](:wm-scene), egy [fejezetre](:wm-chapter), egy [részre](:wm-part), az egész [könyvre](:wm-book). Mindegyik önálló szám, így egy fejezet célértéke sosem változtatja meg a könyvét, és a könyv célértéke sosem a benne lévő fejezetek összege. Ami összeadódik, az az írás: egy tároló [haladása](:goal-progress) mindaz, ami alatta meg van írva. A nulla célérték egyszerűen nincs beállítva. A szám [szavakban vagy karakterekben](:goal-unit) értendő, aszerint, hogy a projekt melyiket használja.
goal-unit = A projekt szavakban vagy karakterekben számol.
goal-unit-more = Projektszintű döntés, mert a hosszmértékegység a kézirathoz és annak piacához tartozik, nem egy-egy szakaszhoz: a német és a francia kiadók leütésben mérnek, a japánok 400 karakteres lapokban, a kínaiak ezer karakterben, a legtöbb angol nyelvű kiadó szavakban. Mindkét számot párhuzamosan tárolja a program, és egyiket sem alakítja át a másikba, így az egység váltásakor minden [célérték](:goal-target) a másik számra vonatkozik, visszaváltáskor pedig visszaáll az első. Átváltás nem történik, ezért a már megadott célértékeket kézzel kell frissíteni.
goal-progress = Mennyi van megírva a célértékhez képest.
goal-progress-explained = A saját prózával rendelkező darab a saját hosszához mérődik, a tároló pedig [mindahhoz, ami alatta meg van írva](:goal-manuscript-words). A sáv az írás előrehaladtával pirosból borostyánon át zöldre vált, és ismét megváltozik, ha a darab jócskán túlnő a terven, amire akkor érdemes figyelni, ha egy fejezetnek adott hosszt kell tartania.
goal-manuscript-words = Mi számít a kézirathoz.
goal-manuscript-words-more = Csak az a próza, amely ténylegesen exportálódna: a saját szöveget hordozó [jelenetek](:wm-scene) és [fejezetek](:wm-chapter). A [jegyzet](:wm-note) és a [paratextus](:wm-paratext) nem része a kéziratnak, és sosem számít bele. Ami a kukában van, kiesik, és ugyanígy minden, ami [kimarad az exportálásból](:goal-exportable).
goal-exportable = Ez az elem kimarad az exportálásból.
goal-exportable-more = A saját hossza továbbra is látszik, hiszen az írás megvan, de egyetlen összesítésbe sem számít bele, ahogy egyetlen exportált könyvben sem jelenik meg. Egy tároló kihagyása nem hagyja ki a benne lévő darabokat: minden elem önmagáért felel, erre való a kapcsoló melletti „Alkalmazás a gyermekelemekre” gomb.
goal-distribute = A célérték elosztása a benne lévők között.
goal-distribute-more = Egy [könyv](:wm-book) vagy [rész](:wm-part) célértékét a közvetlen gyermekelemei között osztja szét úgy, hogy a darabok pontosan kiadják. Alapértelmezés szerint csak a még célérték nélküli elemeket tölti ki, a saját számait érintetlenül hagyja. Közönséges [célértékeket](:goal-target) ír, amelyek utána szabadon szerkeszthetők, és szabadon el is térhetnek egymástól: ez egyszeri művelet, nem állandó kapcsolat.
goal-subtree-total = A benne lévő célértékek összege.
goal-subtree-total-more = Tény, nem célérték. A projekt sosem tekinti egy tároló célértékét a benne lévők összegének, mert akkor egy mappa száma magától elmozdulna, valahányszor egy benne lévő jelenet számot kap. Ez a sor csak megmutatja, mennyit tesznek ki a saját számai, hogy összevethesse a tároló [célértékével](:goal-target), vagy [eloszthasson egyet](:goal-distribute).
goal-milestone = Egy dátum, amelyre valamit el kell érni.
goal-milestone-more = Kétféle van. Az egyik azt mondja, hogy a [könyvnek](:wm-book) egy adott dátumra adott hosszúságúnak kell lennie: egy állomás a saját görbéjén. A másik azt, hogy egy adott [fejezetnek](:wm-chapter) vagy [résznek](:wm-part) addigra el kell érnie [a saját célértékét](:goal-target), és ezt a számot élőben olvassa, nem másolatot tart róla, így a célérték módosítása a mérföldkövet is frissíti. Mindkettő megjelenik a könyv [tempótervében](:pace-plan).
pace-plan = A könyv írási ütemterve.
pace-plan-more = Egy [célérték](:goal-target) és egy határidő az írásra szánt napokkal együtt, átszámítva arra, mennyit kell naponta írni. A könyv saját célértékét olvassa, így az itt és a Tulajdonságok panelen beállított érték ugyanaz a szám két helyen. Az út közbeni állomások a [mérföldkövek](:goal-milestone).
concept-tag = Színkódos címke, amelyet bármely elemre feltehet, és az egész projektben újra felhasználhat.
concept-tag-more = Címkével megjelölhet egy elemet: egy állapotot, egy helyszínt, egy követett cselekményszálat. Ugyanaz a címke tetszőleges számú elemen szerepelhet, és egy elem több címkét is viselhet. Ha egy címke valamit megnevez a könyvében, például egy szereplőt vagy helyszínt, kapcsolja be rajta a [prózában keresést](:wm-find-in-prose), és a Skribisto elkezdi keresni a prózájában.
concept-label = Rövid szöveg, amelyet egy elem címe alá ír, a saját használatára.
concept-label-more = Nem [címke](:concept-tag): a felirat egyetlen elemhez tartozik, egyszerű szöveg szín és katalógus nélkül, például „1. fordulópont” vagy „nevet kell neki adni”. Az elem helyi menüjéből állíthatja be, vagy helyben szerkesztheti az Áttekintés Felirat oszlopában; kis alcímként jelenik meg a cím alatt, a vázlatban és a szövegfolyamban egyaránt.
concept-point-of-view = Kinek a szemével mesélik el a jelenetet: egy vagy több szereplője.
concept-point-of-view-more = A Tulajdonságok panelen állíthatja be, a szereplők mellett. Ha olyasvalakit választ, aki még nincs a szereplők között, oda is bekerül. Egynél több nézőpont is megengedett, közös jelenetnél vagy a könyv közepén bekövetkező nézőpontváltásnál hasznos, a Skribisto nem tiltja. A jelöltek azokból az elemekből kerülnek ki, amelyeket egy [prózában keresett](:wm-find-in-prose) címke a történetbibliába helyez.
concept-epigraph = Egy rész vagy fejezet élére helyezett idézet.
concept-epigraph-more = Külön mező, elválasztva a [fejezet](:wm-chapter) vagy a [rész](:wm-part) saját prózájától; [jelenetnek](:wm-scene) és [jegyzetnek](:wm-note) nincs ilyen. Írjon egyet közönséges idézetként; ha többet ír, mindegyik külön blokkidézet lesz, és külön mottóként jelenik meg. A szavai sosem számítanak a kézirat szavai közé.
concept-footnote = A prózában jelölt hivatkozás, amely számozott jegyzetként jelenik meg.
concept-footnote-more = A hivatkozás egyetlen jel a szavai között, így pontosan úgy mozog és törlődik, mint a körülötte lévő szavak. A száma sosem tárolódik: a Skribisto mindig frissen számítja ki abból, hol áll az egyes hivatkozás a teljes kéziratban, így egy új beszúrása után minden későbbi jegyzet átszámozódik. A lábjegyzet saját szavai a kézirattól külön számítanak, sosem olvadnak bele.
concept-chapter-mode = Egy új fejezet jelenetekből álló mappa vagy egyetlen egyszerű elem legyen.
concept-chapter-mode-more = A Beállítások Szerkezet oldalán állítható be. A [fejezetbe](:wm-chapter) mindkét esetben közvetlenül írhat; az egyetlen különbség, hogy alatta jeleneteket is tartalmazhat-e. Mindkét forma ugyanazt a könyvet adja, és egy projekten belül szabadon keverheti őket. Az Átalakítás egy szó elvesztése nélkül váltja át a meglévő fejezetet egyikből a másikba.
concept-comment = Pontos prózaszakaszhoz kötött megjegyzés, amely a margón jelenik meg.
concept-comment-more = A szavakra emlékszik, amelyekre mutat, és a környezetükre, nem rögzített pozícióra, így túléli a szokásos szerkesztést és a teljes újratöltést. Alá válaszolva folytathatja a beszélgetést. Ha a pontos idézet már nem található, a megjegyzés jelzi ezt, ahelyett hogy csendben rossz mondathoz vándorolna.
concept-backup = Az egész projekt adott időpontban készült másolata, a projekt mellett tárolva.
concept-backup-more = Az Ön által választott ütemezés szerint készül: megnyitáskor, bezáráskor, időközönként, vagy ezek bármely kombinációjában; a régi másolatok kor vagy darabszám szerint ritkulnak. Megnyitva saját ablakban töltődik be: megnézheti, sőt szerkesztheti is, de csak a Mentés másként vagy a Visszaállítás őriz meg bármit. Nem azonos a [változattal](:concept-version), amely egyetlen elem saját múltját követi.
concept-version = Egyetlen elem saját múltja: a szövegének vagy szinopszisának minden változása.
concept-version-more = A projekt saját mentési előzményeiből és a biztonsági másolataiból együtt áll össze, változásonként egy bejegyzéssel, nem fájlonként. Válasszon egy bejegyzést, hogy lássa, mi változott, és csak az adott elem szövegét állítsa vissza rá; a projekt többi része érintetlen marad, és előbb biztonsági másolat készül. Nem azonos a [biztonsági másolattal](:concept-backup), amely mindenről másolat.
concept-trash = A kukába dobott elem elrejtődik, nem kerül át máshová: pontosan ott marad, ahol volt.
concept-trash-more = A kukába dobás csak egy jelzőt állít át: az elem megtartja a helyét az iratrendezőben, inaktívként megjelölve, nem eltávolítva. Ha visszaállítja, pontosan ugyanott jelenik meg újra, nem kell újra elhelyezni. A Kuka ürítése véglegesen eltávolítja a kukába dobott elemeket, és mindegyik művelet egyetlen visszavonható lépés, mint a projekt bármely más módosítása.
concept-spellcheck = Aláhúzza azokat a szavakat, amelyeket a projekt nyelvein egyetlen telepített szótár sem ismer.
concept-spellcheck-more = Minden projektnek saját munkanyelve vagy munkanyelvei vannak; egy elem felülbírálhatja őket egy más nyelven írt jelenetnél. A Skribisto által nem ismert szót a szerkesztő helyi menüjéből veheti fel a saját szótárba, amely minden projektre érvényes. A helyesírás-ellenőrzés kikapcsolása mindenhol leállítja, amíg vissza nem kapcsolja.
concept-search-replace = Egy szót vagy kifejezést a projekt egészében megtalál, nem csak a megnyitott dokumentumban.
concept-search-replace-more = Átnéz minden jelenetet, jegyzetet, címet, szinopszist és feliratot, valamint a megjegyzéseket és lábjegyzeteket is. A kis- és nagybetűk, a teljes szó és az ékezetek figyelembevétele három külön kapcsoló. Az Összes cseréje minden átnézett találatot egyetlen menetben és egyetlen visszavonási lépésben ír át, így a Ctrl+Z az egész köteget visszavonja, nem csak az utolsó változást.
concept-note-template = Újrahasználható szövegrész, amelyet bármibe beszúrhat, amin éppen dolgozik.
concept-note-template-more = Üres szereplőlap, helyszínprofil, jelenetvázlat: írja meg egyszer, és illessze be, ahol szüksége van rá, bármely elembe, nem csak jegyzetbe. Induljon egy beépített mintából, importáljon egy .md vagy .djot fájlt, vagy írjon valamit, és válassza a Dokumentum ▸ Mentés sablonként parancsot. A projektben tárolódik, így a projektet megnyitó szerzőtárs ugyanazt a készletet kapja.
concept-text-replacement = Saját rövidítései, amelyek gépelés közben automatikusan kibővülnek.
concept-text-replacement-more = Adjon meg egy rövidítést és a helyettesítőjét, például „pl” helyett „például”, és a rövidítés abban a pillanatban kibővül, amint szóközt vagy írásjelet ír utána. A kis- és nagybetűk követik a gépelést: ha a rövidítést nagybetűvel kezdi, a kibővített szöveg is nagybetűvel kezdődik. A kibővítés után azonnal lenyomott Backspace csak azt az egy cserét vonja vissza. Projektenként kapcsolható be, és nem azonos az [intelligens írásjelekkel](:concept-smart-punctuation).
concept-smart-punctuation = A nyelvhez igazodó tipográfia, amely gépelés közben automatikusan érvényesül: idézőjelek, gondolatjelek, három pont.
concept-smart-punctuation-more = Az egyenes idézőjeleket tipográfiaiakra cseréli, a -- jelből nagykötőjelet, a --- jelből hosszú gondolatjelet, a ...-ból három pont karaktert készít, és ahol bekapcsolja, a „- ” jellel kezdett bekezdést párbeszédjellel nyitja. Projektenként állítható be, és a .skrib fájlban utazik, így a szerzőtárs ugyanazokkal a szabályokkal ír. Nem azonos a [saját szabályaival](:concept-text-replacement), amelyeket Ön ír meg.
concept-export-style = Exportálási beállítások elnevezett csomagja: címsorok, térközök, mi kerüljön bele.
concept-export-style-more = Egyetlen stílus, amelyet minden exportálás használ, bármilyen formátumot választ is. A beépített stílusok csak olvashatók; kettőzzön meg egyet, hogy legyen egy módosítható kiindulópontja. Az egyik beállítása az, hogy belefoglalja-e a [visszaküldési jelölőket](:concept-round-trip-marks) egy szerkesztőhöz küldött és onnan visszaérkező fájlba.
concept-round-trip-marks = Láthatatlan könyvjelzők, amelyek révén egy szerkesztett DOCX vagy ODT felismerten tér haza.
concept-round-trip-marks-more = Csak abba a két formátumba kerülnek, amelyet egy szerkesztő vissza tud küldeni, a DOCX-be és az ODT-be; minden más exportálás nélkülük készül. Könyvjelző, nem egyéni attribútum: a LibreOffice mentéskor eltávolítja az egyéni attribútumokat, de a könyvjelzőt érintetlenül hagyja. [Exportstílusonként](:concept-export-style) kapcsolható be, a „Küldés szerkesztőnek” csoportban.
concept-status = Hol tart egy elem a saját munkafolyamatában: egyszerre egy szakasz, előrehaladva.
concept-status-more = Nem [címke](:concept-tag): egy elemnek pontosan egy állapota van, a szakaszok sorrendbe rendezettek, és a listát a projekt birtokolja, így bármikor átnevezheti, átrendezheti vagy bővítheti. A fejezet vagy könyv melletti csillag azt jelenti, hogy az alatta lévő részek nem mind egyeznek vele. A Tulajdonságok panelen, egy kártyán, egy szövegfolyam-elemen vagy itt állíthatja be.
