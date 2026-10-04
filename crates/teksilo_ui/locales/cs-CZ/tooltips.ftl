# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

wm-book = Kniha ve vašem projektu.
wm-book-more = Kniha je tak jednoduchá nebo vrstvená, jak se vám líbí. Krátký příběh může být jen pár [scény](:wm-scene). Delší projekt může hnízdit [Části a součásti](:wm-part) a [kapitoly](:wm-chapter) tak hluboko, jak chcete. Tvar je váš.
wm-part = Částečné seskupování kapitol.
wm-part-more = Část shromáždí[kapitoly](:wm-chapter) pod jedním záhlavím. Jako kapitola, to běží až do další části nebo na konci[Kniha](:wm-book).
wm-chapter = Kapitola knihy.
wm-chapter-more = Kapitola běží odtud až do další kapitoly, další [Část](:wm-part), nebo [konec knihy](:wm-end-of-book). Přichází ve dvou tvarech: složka kapitoly, která drží [scény](:wm-scene), nebo plochá kapitola, která nese svůj vlastní próza. Vyberte si, který z nových kapitol používá v Nastavení, pod strukturou.
wm-scene = Scéna prózy.
wm-scene-more = Scéna drží vaše psaní. Každá položka má dvě strany: hlavní text a [Shrnutí](:wm-synopsis) plánovat s. Oba otevřeny společně v editoru.
wm-note = Dopis.
wm-note-more = Poznámka vede text a[Shrnutí](:wm-synopsis), jako je[Scéna](:wm-scene), pro výzkum, asides, nebo upomínky. Jakákoli položka může být součástí sestavené knihy, včetně poznámky sedí uvnitř[Kapitola](:wm-chapter).
wm-note-folder = Poznámka, která obsahuje jiné položky.
wm-note-folder-more = A [Poznámka](:wm-note), který také obsahuje položky pod ním, takže můžete seskupit související poznámky dohromady. To si nechává své vlastní [Shrnutí](:wm-synopsis), a položky uvnitř ponechat svůj vlastní text.
wm-folder = Organizační složka.
wm-folder-more = Kontejner pro aranžování položek v pořadači vám však vyhovuje. Nosí [Shrnutí](:wm-synopsis), ale žádnou prózu sama o sobě. Položky uvnitř drží psaní.
wm-end-of-book = Značky, kde kniha končí.
wm-end-of-book-more = Každá kniha sdílí jeden nepřetržitý seznam, takže o konci knihy není rozhodnuto hnízda. Tato značka říká, že [Kniha](:wm-book) zastaví zde. Cokoliv po ní patří do další knihy.
wm-story-bible-entry = Tagged, aliated note o vašem příběhu.
wm-story-bible-entry-more = A [Poznámka](:wm-note) o charakteru, umístění, nebo cokoliv jiného, co stojí za podání, vytvořené s názvem, tagy, aliasy a výchozí šablony vše v jednom kroku. Nic na stránce rozlišuje to poté, protože to zůstává obyčejný vzkaz můžete přejmenovat, přejmenovat nebo převést jako každý jiný.
wm-synopsis = Shrnutí textu.
wm-synopsis-more = Souhrn připojený k jakékoli psaní položky, obvykle krátký odstavec, ale tak dlouho, jak se vám líbí. Použijte ho k plánování a navigace před próza existuje. Sedí vedle hlavního textu v editoru.

scene-break-minor = Obyčejná scéna přestávka: posun času, místa nebo pohledu v rámci kapitoly.
scene-break-minor-more =
    Označuje přestávku *, kam ji dáte *, včetně uprostřed [Scéna](:wm-scene). Rozdělení prózy do dvou položek je organizační volba, takže nikdy nevytvoří přestávku na vlastní pěst.

    Psaní do prózy jako tři hvězdičky oddělené mezerami. Jak to tiskne je rozhodnuto exportním stylem, ne podle toho, co zadáte: Shunn rukopis nastaví to jako singl #, obchodní paperback jako dinkus, a většina francouzské, německé, španělské, ruské a italské vydavatelství jako holý mezera bez značky vůbec.
scene-break-major = Silnější rozdělení: velký časový skok, nebo rozhodující změna pohledu.
scene-break-major-more =
    Stejný nápad jako normální scéna break, jeden krok nahoru. Použijte ji, když obyčejný break by podhodnotil skok.

    Napsaná jako # # #, a tištěná jinak než běžná úroveň exportním stylem. Shunn standardní rukopis formát nastavuje singl # proti # # # pro přesně tento rozdíl. Kde tradice nemá silnější značku, obě úrovně tisknou stejně.
wm-find-in-prose = Skribisto hledá tuto položku ve vaší próze.
wm-find-in-prose-more = Zapněte to pro značky, které pojmenují věci, o kterých píšete: znaky, místa, objekty. Každá položka nesoucí takový štítek je porovnána s vaším prózou podle jeho názvu a podle ostatních jmen, které dáte, takže každý[Scéna](:wm-scene) seznam, který a co se v něm objeví, aniž byste cokoliv spojili ručně. Zanechte to na štítky, které popisují položku spíše než jméno jednoho, jako je návrh stavu nebo připomínka kontrolovat kontinuitu. Každá položka, která se shoduje, se připojí také k vašemu[Encyklopedie příběhu](:wm-story-bible).
wm-story-bible = Každý znak, místo a další označený vstup, shromážděný v jedné síti.
wm-story-bible-more = Otevřeno z libovolné složky poznámky vlastní kartu. Karty skupiny po tagu, a každé jméno, kolik aliasů záznam odpovědi a kolik scén se objevil v, čerpané z každého [najít v próza](:wm-find-in-prose) tag v projektu. Filtrování podle knihy pouze zužuje, které karty ukazují, nikdy se rozhodne, které položky existují.

tooltip-go-to = Přeskočit na libovolnou položku v pořadači (Ctrl + G)
synopsis-collapse-tooltip = Skrýt sloupek synopse

wm-paratext = Text, který není součástí příběhu.
wm-paratext-more = Předmluva, věnování, afterword, colophon: psaní, které patří do knihy, ale ne do jejího těla. Exported kamkoli jste ji dal, a nikdy započítán v rukopisu slova počítat. Kde to jde je na vás: úmluvy se liší podle země a vydavatele.
wm-paratext-folder = Složka pro paratexty.
wm-paratext-folder-more = Někde, kde se drží předmluvy a následná slova, takže se neruší pořadač. Organizuje pouze: nese [Shrnutí](:wm-synopsis), ale nepřidává nic do exportované knihy, ani své vlastní jméno.

goal-target = Jak dlouho to má trvat?
goal-target-more = Nastavit cíl na cokoliv: [Scéna](:wm-scene), a [Kapitola](:wm-chapter), a [Část](:wm-part), the whole [Kniha](:wm-book). Každý z nich je své vlastní číslo a stojí na vlastní pěst, takže stanovení cíle na kapitolu nikdy nemění své knihy, a kniha je cíl není nikdy součet kapitol uvnitř něj. Co to sečte je psaní: kontejneru [Průběh](:goal-progress) je vše napsané pod ním. Zanechat cíl na nule a to prostě není nastaveno. Číslo se počítá v [slova nebo znaky](:goal-unit), podle toho, co tento projekt použije.
goal-unit = Ať už se tento projekt počítá slovy nebo znaky.
goal-unit-more = Projekt- široký výběr, protože délka jednotka patří k rukopisu a jeho trh spíše než do jedné části: Německé a francouzské vydavatelské opatření ve znacích, Japonci ve 400- znak listy, Číňan o tisíc znaků, většina anglický jazyk publikování slovy. Obě čísla jsou uchovávány vedle sebe a ani je někdy převeden na druhý, takže přepínání jednotkových bodů každé [Cíl](:goal-target) na druhé postavě a přepínání zpět obnoví první. To, co nedělá, je konvertovat cokoliv, takže cíle již zadané budou muset aktualizovat ručně.
goal-progress = Kolik je napsáno, proti cíli.
goal-progress-explained = Kousek s prózou vlastní se měří proti své vlastní délce; kontejner proti [všechno napsané pod ním.](:goal-manuscript-words). Bar ohřívá od červené přes oranžové do zelené, jak se píše, a mění se znovu, jakmile kus běžel dobře za to, co bylo plánováno, což stojí za povšimnutí, když kapitola má délku držet.
goal-manuscript-words = Co se počítá jako rukopis.
goal-manuscript-words-more = Pouze próza, která by byla skutečně vyvážena:[scény](:wm-scene) a[kapitoly](:wm-chapter) nese svůj vlastní text.[Poznámka](:wm-note) nebo[Paratext](:wm-paratext) není součástí rukopisu a nikdy se nepočítá do jednoho. Cokoliv v koši vypadne, a tak dělá něco[vynecháno z vývozu](:goal-exportable).
goal-exportable = Tato položka je vynechána z vývozu.
goal-exportable-more = Jeho vlastní délka stále ukazuje, protože psaní je stále tam, ale to se počítá k žádné celkové, přesně tak, jak se objeví v žádné exportované knize. Vynechání kontejneru nenechává kusy uvnitř něj ven: každá položka odpovídá sám za sebe, což je to, co se vztahuje na děti tlačítko vedle vypínače je pro.
goal-distribute = Sdílet tento cíl přes to, co je uvnitř.
goal-distribute-more = Rozdělit [knihy](:wm-book) nebo [part (part)](:wm-part) cíl na své nejbližší děti, takže kusy přidat do něj přesně. Ve výchozím nastavení se vyplní pouze ty, které nemají žádný cíl ještě a zanechá vaše vlastní čísla sama. Co to píše jsou běžné [cíle](:goal-target), zdarma editovat poté a volně unášet od sebe: to je jedna akce, ne standing uspořádání.
goal-subtree-total = K čemu jsou ty cíle uvnitř?
goal-subtree-total-more = Faktem není cíl. Tento projekt se nikdy nezachází s terčem kontejneru jako se součtem těch uvnitř, protože tak se postava složky nakonec pohybuje sama, když scéna uvnitř je uveden číslo. Tento řádek vám jednoduše řekne, k čemu vaše vlastní čísla přicházejí, takže jej můžete porovnat s kontejnerem[Cíl](:goal-target) sám, nebo[Sdílet jeden ven](:goal-distribute).
goal-milestone = Datum k dosažení něčeho.
goal-milestone-more = Dva druhy. Jeden říká, že[Kniha](:wm-book) by měl stát v dané délce o datu, bod na jeho vlastní křivce. Druhý říká, že konkrétní[Kapitola](:wm-chapter) nebo[Část](:wm-part) by měl dosáhnout[svůj vlastní cíl](:goal-target) do té doby, a čte, že číslo žít spíše než udržet kopii, takže editace cíle aktualizace milník příliš. Oba se objeví v knize GLOS[plán tempa](:pace-plan).
pace-plan = Knižní rozvrh psaní.
pace-plan-more = A [Cíl](:goal-target) a termín, s dny, které napíšete na, se změnil v to, kolik den potřebuje. Čte knihu Côls vlastní cíl, takže nastavení ho zde a nastavení v inspektorovi jsou stejné číslo na dvou místech. Waypoints podél cesty jsou[milníky](:goal-milestone).

concept-tag = Barvokódovaný štítek, který můžete dát na libovolnou položku, opakovaně použitelnou po celém projektu.
concept-tag-more = Značka položky označit: stav, umístění, vlákno, které sledujete. Stejná značka může sedět na libovolném počtu položek, a jedna položka může nést několik. Zapněte [najít v próza](:wm-find-in-prose) pro značku, která pojmenuje něco ve vaší knize, charakter nebo místo, a Skribisto začíná hledat to ve vaší próze.
concept-label = Krátká poznámka, kterou napíšete pod titulem položky, na vlastní oči.
concept-label-more = Ne [Štítek](:concept-tag): štítek patří do jedné položky sám, prostý text bez barvy nebo katalog za ním, něco jako appropriate 1. místo pozemku nebo appropriate potřebuje název. Nastavte ho z kontextového menu položky approins, nebo jej upravit inline ve sloupci Přehled approach s Label; ukazuje jako malý titulek pod názvem, v obrysu a v proudu podobně.
concept-point-of-view = Jejichž oči jsou proneseny scénou: jeden nebo více vašich příběhů.
concept-point-of-view-more = Nastavit v inspektorovi, vedle obsazení. Volba někoho, kdo ještě není v obsazení přidává je tam také. Více než jeden pohled je právní volba, užitečné pro sdílenou scénu nebo změna očí midbook, není něco Skribisto bloky. Kandidáti pocházejí z položek [najít v próza](:wm-find-in-prose) tag dává do vašeho příběhu bible.
concept-epigraph = Citace v čele části nebo kapitoly.
concept-epigraph-more = Jeho vlastní pole, oddělené od [Kapitola](:wm-chapter) nebo [Část](:wm-part) Dublin vlastní próza; a [Scéna](:wm-scene) nebo [Poznámka](:wm-note) nosit žádné. Napište jeden jako obyčejný citát; napsat několik a každý se stane vlastní blockquote, vytištěné jako vlastní epigraf. Jeho slova se nikdy nezapočítávají jako rukopis slova.
concept-footnote = Citace označená v próze, tištěná jako očíslovaná bankovka.
concept-footnote-more = Odkaz je jedinou značkou uvnitř vašich slov, takže se pohybuje a maže přesně jako slova kolem něj. Jeho číslo není nikdy uloženo: Skribisto Projects it out fresh from where every reference seeds in the whole rukopis, takže vložení jeden renumbers každý note after it. Poznámka pod čarou se vlastní slova se počítají kromě rukopisu, nikdy složené do něj.
concept-chapter-mode = Ať už nová kapitola je složka scén nebo jeden plochý předmět.
concept-chapter-mode-more = Nastavte v Nastavení, pod strukturou. Zapíšete přímo do [Kapitola](:wm-chapter) tak jako tak; jediným rozdílem je, zda může obsahovat scény pod ním. Oba zápisy se skládají do stejné knihy, a můžete je volně míchat v jednom projektu. Podporovat konvertuje stávající kapitolu mezi oběma, aniž by ztratil slovo.
concept-comment = Poznámka ukotvená na přesný úsek prózy, znázorněná na okraji.
concept-comment-more = Pamatuje si slova, na která ukazuje a co je obklopuje, ne pevnou pozici, takže přežívá běžné úpravy a plné zatížení. Odpovědět pod, aby vlákno běží. Pokud přesný citát již nelze nalézt, komentář říká, že spíše než tiše unášet do špatné věty.
concept-backup = Point- in-time kopie celého projektu, držel vedle něj.
concept-backup-more = Pořízeno na rozvrhu si vyberete: na otevřené, na close, na interval, nebo jakýkoli mix, se starými kopie prostříhané podle věku nebo podle počtu. Otevření jeden načte to ve svém vlastním okně: můžete se podívat a dokonce upravit, ale pouze Uložit jako nebo Obnovit udržuje cokoliv. Liší se od [verze](:concept-version), který sleduje jednu položku IR vlastní minulost.
concept-version = Jedna položka vlastní minulost: každá změna svého textu nebo synovce.
concept-version-more = Postaven z projektu je vlastní historie uložení a jeho záloh společně, jeden záznam za změnu, spíše než jeden na soubor. Vyberte položku vidět, co se změnilo a obnovit pouze tento prvek je text k němu; zbytek projektu je nedotčen, a bezpečnostní kopie je přijata jako první. Liší se od [Záloha](:concept-backup), což je kopie všeho.
concept-trash = Zdemolovaný předmět je skrytý, nepohnutý: zůstává přesně tam, kde byl.
concept-trash-more = Trishing pouze jednu vlajku: položka udržuje své místo v pořadači, označené neaktivní spíše než odstranit. Obnovit a znovu se objeví v tomto přesném místě, bez re- podání dělat. Prázdný Trash odstraňuje rozházené položky pro dobro, a každý z nich je jeden nepřijatelný krok, jako každý jiný projekt editovat.
concept-spellcheck = Podtrhuje slova, která nerozeznává žádný nainstalovaný slovník, podle jazyka projektu.
concept-spellcheck-more = Každý projekt má svůj vlastní pracovní jazyk nebo jazyky; položka může přepsat je pro scénu napsanou v jiném jazyce. Přidat slovo Skribisto neví z kontextového menu editor Côls, do osobního slovníku, který přenáší do každého projektu. Zapínání spell- check off zastaví všude, dokud ho znovu nezapnete.
concept-search-replace = Najde slovo nebo frázi všude v projektu, nejen otevřený dokument.
concept-search-replace-more = Prohlíží každou scénu, notu, titul, synopse a štítek, a také komentáře a poznámky pod čarou. Případ, celoslovní odpovídající a přízvuk odpovídající jsou tři samostatné přepínače. Nahraďte všechny přepisuje každý revidovaný zápas v jednom pass a jeden krok zpět, takže Ctrl + Z bere zpět celou dávku, nikdy jen poslední změnu.
concept-note-template = Znovu použitelnou část psaní, kterou vložíte do čehokoliv, na čem pracujete.
concept-note-template-more = Prázdný znak list, profil umístění, beat list: napsat jednou a kapka to tam, kde to potřebujete, v jakékoli položce, nejen poznámka. Začněte od built- in preset, importovat .md nebo .djot soubor, nebo napsat něco a vyberte Dokument space jako šablona. Uloženo v projektu, takže spoluautor, který otevře to dostane stejný soubor.
concept-text-replacement = Vaše vlastní zkratka, rozšířena automaticky, jak jste psát.
concept-text-replacement-more = Nastavte spouštěč a jeho nahrazení, což je mimochodem, a to rozšiřuje okamžik, kdy zadáte prostor nebo interpunkci po ní. Případ následuje to, co jste napsali: kapitalizovat spouštěč a expanze kapitalizuje také. ABackspace hned po expanzi odčiní pouze to, že jedna substituce. Switched on per project, odlišné od[inteligentní interpunkce](:concept-smart-punctuation).
concept-smart-punctuation = Lokale- vědomi typografie aplikované automaticky, jak jste typ: uvozovky, otlaky, elipsy.
concept-smart-punctuation-more = Křivky rovné citace do typografické ty, změní - do en pomlčka a --- do em pomlčka, změní... do elipsy, a, kde ji zapnete, otevře odstavec zapsaný jako Dialog pomlčka. Nastavit na projekt, a cestuje uvnitř .skrib, takže co- autor píše se stejnými pravidly. Významný od [Tvoje vlastní pravidla.](:concept-text-replacement), které píšete sami.
concept-export-style = Pojmenovaný svazek exportních možností: záhlaví, rozestup, co je zahrnuto.
concept-export-style-more = Jeden styl, znovu použit přes každý export jakýkoliv formát si vyberete. Built- in styly jsou read- only; duplicate jeden získat výchozí bod můžete skutečně změnit. Mezi jeho volby je, zda zahrnout [markery kruhového letu](:concept-round-trip-marks), pro soubor vázaný na editor a zpět.
concept-round-trip-marks = Neviditelné záložky, které dovolily editované DOCX nebo ODT vrátit se domů uznány.
concept-round-trip-marks-more = Napsaný pouze do dvou formátů editor může poslat zpět, DOCX a ODT; každý jiný export nenese žádný. Záložka, není vlastní atribut: LibreOffice pruhy vlastní atributy na uložení, ale ponechává záložku neporušenou. Zapnuto pro [vývozní styl](:concept-export-style), pod posíláním editoru.
concept-status = V případě, že předmět stojí ve vašem vlastním pracovním toku: jeden stupeň po druhém, pohybující se vpřed.
concept-status-more = Ne [Štítek](:concept-tag): položka má přesně jeden stav, fáze jsou objednány, a projekt vlastní seznam, takže si můžete přejmenovat, přeobjednat nebo přidat k němu kdykoliv chcete. Asterisk vedle kapitoly nebo knihy znamená, že části pod to ne všichni souhlasí s ním. Nastavte to od inspektora, karty, stream položky, nebo zde.
