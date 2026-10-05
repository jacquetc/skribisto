# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

wm-book = Kniha ve vašem projektu.
wm-book-more = Kniha může být tak jednoduchá, nebo tak vrstevnatá, jak chcete. Povídka může tvořit jen pár [scén](:wm-scene). Delší dílo může vnořovat [části](:wm-part) a [kapitoly](:wm-chapter) do libovolné hloubky. Podoba je na vás.
wm-part = Část, která seskupuje kapitoly.
wm-part-more = Část shromažďuje [kapitoly](:wm-chapter) pod jedním nadpisem. Stejně jako kapitola trvá až do další části nebo do konce [knihy](:wm-book).
wm-chapter = Kapitola knihy.
wm-chapter-more = Kapitola trvá odsud až do další kapitoly, další [části](:wm-part) nebo [konce knihy](:wm-end-of-book). Má dvě podoby: složka kapitoly, která obsahuje [scény](:wm-scene), nebo plochá kapitola, která nese vlastní prózu. Kterou podobu nové kapitoly použijí, zvolíte v Nastavení v části Struktura.
wm-scene = Scéna s prózou.
wm-scene-more = Scéna obsahuje vaše psaní. Každá psací položka má dvě strany: hlavní text a [synopsi](:wm-synopsis) pro plánování. Obě se v editoru otevírají společně.
wm-note = Poznámka.
wm-note-more = Poznámka nese text a [synopsi](:wm-synopsis) stejně jako [scéna](:wm-scene) a slouží pro rešerše, odbočky či připomínky. Součástí sestavené knihy může být kterákoli položka, včetně poznámky uvnitř [kapitoly](:wm-chapter).
wm-note-folder = Poznámka, která obsahuje další položky.
wm-note-folder-more = [Poznámka](:wm-note), pod kterou jsou navíc další položky, takže můžete seskupovat související poznámky. Má vlastní [synopsi](:wm-synopsis) a položky uvnitř si ponechávají vlastní text.
wm-folder = Organizační složka.
wm-folder-more = Kontejner, ve kterém si položky v pořadači uspořádáte, jak vám vyhovuje. Nese [synopsi](:wm-synopsis), ale žádnou vlastní prózu. Psaní obsahují položky uvnitř.
wm-end-of-book = Označuje, kde kniha končí.
wm-end-of-book-more = Všechny knihy sdílejí jeden souvislý seznam, takže konec knihy se neurčuje vnořením. Tato značka říká, že [kniha](:wm-book) končí zde. Vše za ní patří do další knihy.
wm-story-bible-entry = Poznámka o vašem příběhu se štítky a alternativními jmény.
wm-story-bible-entry-more = [Poznámka](:wm-note) o postavě, místě nebo čemkoli jiném, co stojí za zaznamenání, vytvořená v jediném kroku i se jménem, štítky, alternativními jmény a výchozí šablonou. Na stránce ji pak nic neodlišuje: zůstává obyčejnou poznámkou, kterou můžete přejmenovat, přeštítkovat nebo převést jako kteroukoli jinou.
wm-synopsis = Shrnutí psací položky.
wm-synopsis-more = Shrnutí připojené ke každé psací položce, obvykle krátký odstavec, ale může být libovolně dlouhé. Slouží k plánování a orientaci ještě předtím, než próza existuje. V editoru stojí vedle hlavního textu.

scene-break-minor = Běžný předěl scény: posun v čase, místě nebo úhlu pohledu v rámci kapitoly.
scene-break-minor-more =
    Označuje předěl *tam, kam jej umístíte*, i uprostřed [scény](:wm-scene). Rozdělení prózy do dvou položek je organizační volba, takže samo o sobě předěl nikdy nevytvoří.

    Do prózy se píše jako tři hvězdičky oddělené mezerami. Jak se vytiskne, určuje exportní styl, ne to, co napíšete: rukopis podle Shunna jej sází jako jediný znak #, komerční paperback jako dinkus a většina francouzských, německých, španělských, ruských a italských nakladatelů jako prostou mezeru bez jakékoli značky.
scene-break-major = Silnější dělení: velký časový skok nebo rozhodná změna úhlu pohledu.
scene-break-major-more =
    Stejná myšlenka jako běžný předěl scény, jen o stupeň výš. Použijte jej, když by obyčejný předěl skok zlehčoval.

    Píše se jako # # # a exportní styl jej vytiskne jinak než běžnou úroveň. Standardní formát rukopisu podle Shunna sází právě kvůli tomuto rozlišení jediný # proti # # #. Kde tradice silnější značku nemá, tisknou se obě úrovně stejně.
wm-find-in-prose = Skribisto tuto položku hledá ve vaší próze.
wm-find-in-prose-more = Zapněte u štítků, které pojmenovávají to, o čem píšete: postavy, místa, předměty. Každá položka s takovým štítkem se hledá ve vaší próze podle svého názvu a podle dalších jmen, která jí dáte, takže každá [scéna](:wm-scene) uvádí, kdo a co se v ní objevuje, aniž byste cokoli propojovali ručně. U štítků, které položku popisují, místo aby ji pojmenovávaly, například stav konceptu nebo připomínka zkontrolovat návaznost, nechte volbu vypnutou. Každá nalezená položka se také stane součástí vaší [bible příběhu](:wm-story-bible).
wm-story-bible = Všechny postavy, místa a další označené záznamy shromážděné v jedné mřížce.
wm-story-bible-more = Otevírá se z vlastní karty kterékoli složky poznámek. Karty se seskupují podle štítků a každá uvádí, na kolik alternativních jmen záznam slyší a v kolika scénách se objevil, podle všech štítků [hledání v próze](:wm-find-in-prose) v projektu. Filtr podle knihy jen zužuje, které karty se zobrazí; nikdy nerozhoduje o tom, které záznamy existují.

tooltip-go-to = Přejít na libovolnou položku v pořadači (Ctrl+G)
synopsis-collapse-tooltip = Skrýt sloupec synopse

wm-paratext = Text, který není součástí příběhu.
wm-paratext-more = Předmluva, věnování, doslov, tiráž: texty, které patří ke knize, ale ne k jejímu tělu. Exportují se tam, kam je umístíte, a nikdy se nezapočítávají do počtu slov rukopisu. Kam je dáte, je na vás: zvyklosti se liší podle země a nakladatele.
wm-paratext-folder = Složka pro paratexty.
wm-paratext-folder-more = Místo pro předmluvy a doslovy, aby nezaplňovaly pořadač. Slouží jen k organizaci: nese [synopsi](:wm-synopsis), ale do exportované knihy nepřidá nic, ani svůj vlastní název.

goal-target = Jak dlouhý má tento text být.
goal-target-more = Cíl můžete nastavit čemukoli: [scéně](:wm-scene), [kapitole](:wm-chapter), [části](:wm-part) i celé [knize](:wm-book). Každý cíl je samostatné číslo, takže cíl kapitoly nikdy nemění cíl její knihy a cíl knihy nikdy není součtem kapitol v ní. Sčítá se jen napsaný text: [průběh](:goal-progress) kontejneru zahrnuje vše, co je pod ním napsáno. Cíl ponechaný na nule prostě není nastaven. Počítá se ve [slovech, nebo znacích](:goal-unit), podle toho, co tento projekt používá.
goal-unit = Zda tento projekt počítá ve slovech, nebo ve znacích.
goal-unit-more = Volba pro celý projekt, protože jednotka délky patří k rukopisu a jeho trhu, ne k jedné jeho části: německá a francouzská nakladatelství měří ve znacích, japonská v listech po 400 znacích, čínská v tisících znaků a většina anglicky publikujících ve slovech. Obě čísla se vedou souběžně a nikdy se jedno na druhé nepřevádí, takže přepnutí jednotky nasměruje každý [cíl](:goal-target) na druhý údaj a přepnutí zpět obnoví ten první. Nic však nepřevádí, takže už zadané cíle bude potřeba upravit ručně.
goal-progress = Kolik je napsáno vzhledem k cíli.
goal-progress-explained = Text s vlastní prózou se měří podle své vlastní délky, kontejner podle [všeho, co je pod ním napsáno](:goal-manuscript-words). Ukazatel se s přibývajícím textem barví od červené přes jantarovou po zelenou a znovu se změní, jakmile text plánovaný rozsah výrazně překročí, což se hodí vědět, když má kapitola dodržet určitou délku.
goal-manuscript-words = Co se počítá do rukopisu.
goal-manuscript-words-more = Jen próza, která by se skutečně exportovala: [scény](:wm-scene) a [kapitoly](:wm-chapter) s vlastním textem. [Poznámka](:wm-note) ani [paratext](:wm-paratext) nejsou součástí rukopisu a nikdy se do něj nepočítají. Vše v koši vypadává, stejně jako vše [vynechané z exportu](:goal-exportable).
goal-exportable = Tato položka je vynechána z exportu.
goal-exportable-more = Její vlastní délka se dál zobrazuje, protože text tu stále je, ale nezapočítává se do žádného součtu, přesně tak, jak se neobjeví v žádné exportované knize. Vynecháním kontejneru se nevynechají položky v něm: každá položka rozhoduje sama za sebe, a k tomu slouží tlačítko „Použít na podřízené položky“ vedle přepínače.
goal-distribute = Rozdělit tento cíl mezi to, co je uvnitř.
goal-distribute-more = Rozdělí cíl [knihy](:wm-book) nebo [části](:wm-part) mezi její přímé potomky tak, aby jejich součet přesně odpovídal. Ve výchozím stavu vyplní jen ty, které cíl ještě nemají, a vaše vlastní čísla nechá být. Zapíše obyčejné [cíle](:goal-target), které můžete později libovolně upravit a které se mohou rozejít: jde o jednorázovou akci, ne o trvalé uspořádání.
goal-subtree-total = Kolik dávají dohromady cíle uvnitř.
goal-subtree-total-more = Údaj, ne cíl. Tento projekt nikdy nebere cíl kontejneru jako součet cílů v něm, protože právě tak se údaj složky začne sám měnit, jakmile scéna uvnitř dostane číslo. Tento řádek vám jen řekne, kolik dávají dohromady vaše vlastní čísla, abyste je mohli sami porovnat s [cílem](:goal-target) kontejneru, nebo jej [rozdělit](:goal-distribute).
goal-milestone = Datum, do kterého se má něčeho dosáhnout.
goal-milestone-more = Dva druhy. První říká, že [kniha](:wm-book) má do určitého data dosáhnout dané délky, což je bod na její vlastní křivce. Druhý říká, že určitá [kapitola](:wm-chapter) nebo [část](:wm-part) má do té doby dosáhnout [svého vlastního cíle](:goal-target), a čte toto číslo průběžně, místo aby si uchovávala jeho kopii, takže úprava cíle aktualizuje i milník. Oba se zobrazují v [plánu tempa](:pace-plan) knihy.
pace-plan = Harmonogram psaní knihy.
pace-plan-more = [Cíl](:goal-target) a termín spolu se dny, kdy píšete, převedené na to, kolik je třeba napsat za den. Čte vlastní cíl knihy, takže nastavení zde a nastavení v inspektoru je totéž číslo na dvou místech. Body na cestě jsou [milníky](:goal-milestone).

concept-tag = Barevný štítek, který můžete dát libovolné položce a používat v celém projektu.
concept-tag-more = Štítkem položku označíte: stav, místo, dějovou linku, kterou sledujete. Stejný štítek může mít libovolný počet položek a jedna položka jich může mít několik. Zapněte [hledání v próze](:wm-find-in-prose) u štítku, který pojmenovává něco ve vaší knize, postavu nebo místo, a Skribisto to začne hledat ve vaší próze.
concept-label = Krátká poznámka pod názvem položky, jen pro vaše oči.
concept-label-more = Není to [štítek](:concept-tag): popisek patří jen jedné položce, je to prostý text bez barvy a bez katalogu, například „1. zápletka“ nebo „potřebuje jméno“. Nastavíte jej z místní nabídky položky nebo jej upravíte přímo ve sloupci Popisek v přehledu; zobrazuje se jako malý podtitul pod názvem, v osnově i v souvislém zobrazení.
concept-point-of-view = Číma očima je scéna vyprávěna: jedna či více postav vašeho příběhu.
concept-point-of-view-more = Nastavuje se v inspektoru vedle seznamu postav. Zvolíte-li někoho, kdo mezi postavami ještě není, přidá se tam také. Více úhlů pohledu je přípustná volba, užitečná pro sdílenou scénu nebo změnu vypravěče uprostřed knihy, a Skribisto ji neblokuje. Kandidáti pocházejí z položek, které štítek [hledání v próze](:wm-find-in-prose) zařadí do vaší bible příběhu.
concept-epigraph = Citát v čele části nebo kapitoly.
concept-epigraph-more = Samostatné pole, oddělené od vlastní prózy [kapitoly](:wm-chapter) nebo [části](:wm-part); [scéna](:wm-scene) ani [poznámka](:wm-note) žádné nemají. Napište jedno jako obyčejný citát; napíšete-li jich několik, každé se stane samostatnou blokovou citací a vytiskne se jako samostatné motto. Jeho slova se nikdy nezapočítávají do rukopisu.
concept-footnote = Odkaz vyznačený v próze, tištěný jako číslovaná poznámka.
concept-footnote-more = Odkaz je jediná značka mezi vašimi slovy, takže se přesouvá a maže přesně jako slova kolem ní. Jeho číslo se nikdy neukládá: Skribisto jej pokaždé znovu odvodí z místa, kde každý odkaz v celém rukopisu stojí, takže vložením jednoho se přečíslují všechny poznámky za ním. Vlastní slova poznámky pod čarou se počítají odděleně od rukopisu a nikdy se do něj nezahrnují.
concept-chapter-mode = Zda je nová kapitola složkou scén, nebo jedinou plochou položkou.
concept-chapter-mode-more = Nastavuje se v Nastavení v části Struktura. Do [kapitoly](:wm-chapter) píšete přímo v obou případech; jediný rozdíl je, zda pod sebou může mít i scény. Obě podoby se sestaví do stejné knihy a v jednom projektu je můžete libovolně kombinovat. Příkaz Převést na změní stávající kapitolu z jedné podoby na druhou bez ztráty jediného slova.
concept-comment = Poznámka ukotvená k přesnému úseku prózy, zobrazená na okraji.
concept-comment-more = Pamatuje si slova, na která ukazuje, a jejich okolí, ne pevnou pozici, takže přežije běžné úpravy i úplné znovunačtení. Odpovědí pod ním můžete vést vlákno. Pokud přesný citát už nelze najít, komentář to řekne, místo aby se potichu posunul k nesprávné větě.
concept-backup = Kopie celého projektu k určitému okamžiku, uložená vedle něj.
concept-backup-more = Pořizuje se podle plánu, který si zvolíte: při otevření, při zavření, v intervalu nebo libovolnou kombinací, přičemž staré kopie se probírají podle stáří nebo počtu. Otevřená záloha se načte ve vlastním okně: můžete si ji prohlížet a dokonce upravovat, ale cokoli zachová jen Uložit jako nebo Obnovit. Liší se od [verze](:concept-version), která sleduje minulost jedné položky.
concept-version = Minulost jedné položky: každá změna jejího textu nebo synopse.
concept-version-more = Sestavuje se z vlastní historie ukládání projektu a z jeho záloh dohromady, jeden záznam na změnu, ne na soubor. Vyberte záznam, uvidíte, co se změnilo, a můžete na něj obnovit jen text této položky; zbytek projektu zůstane nedotčen a nejprve se pořídí bezpečnostní kopie. Liší se od [zálohy](:concept-backup), která je kopií všeho.
concept-trash = Položka v koši je skrytá, ne přesunutá: zůstává přesně tam, kde byla.
concept-trash-more = Přesunutí do koše jen přepne jeden příznak: položka si ponechá své místo v pořadači a je označena jako neaktivní, ne odstraněna. Když ji obnovíte, objeví se znovu přesně na tom místě a nemusíte ji nikam zařazovat. Vyprázdnit koš odstraní položky v koši natrvalo a každý z těchto kroků lze vrátit zpět jako kteroukoli jinou úpravu projektu.
concept-spellcheck = Podtrhává slova, která nezná žádný nainstalovaný slovník, podle jazyka projektu.
concept-spellcheck-more = Každý projekt má svůj pracovní jazyk či jazyky; položka je může přepsat u scény psané v jiném jazyce. Slovo, které Skribisto nezná, přidáte z místní nabídky editoru do osobního slovníku, který platí pro všechny projekty. Vypnutím kontroly pravopisu ji zastavíte všude, dokud ji znovu nezapnete.
concept-search-replace = Najde slovo nebo frázi kdekoli v projektu, nejen v otevřeném dokumentu.
concept-search-replace-more = Prochází všechny scény, poznámky, názvy, synopse a popisky a také komentáře a poznámky pod čarou. Rozlišování velikosti písmen, hledání celých slov a rozlišování diakritiky jsou tři samostatné přepínače. Nahradit vše přepíše všechny zkontrolované shody najednou jako jeden krok vrácení, takže Ctrl+Z vrátí celou dávku, nikdy jen poslední změnu.
concept-note-template = Opakovaně použitelný kus textu, který vložíte do čehokoli, na čem pracujete.
concept-note-template-more = Prázdný list postavy, profil místa, osnova scén: napíšete jednou a vložíte, kdekoli je potřebujete, do jakékoli položky, nejen do poznámky. Začněte vestavěnou předvolbou, importujte soubor .md nebo .djot, nebo něco napište a zvolte Dokument ▸ Uložit jako šablonu. Ukládá se v projektu, takže spoluautor, který jej otevře, dostane stejnou sadu.
concept-text-replacement = Vaše vlastní zkratky, rozvinuté automaticky při psaní.
concept-text-replacement-more = Nastavte zkratku a její náhradu, například „mj“ za „mimo jiné“, a ta se rozvine ve chvíli, kdy za ní napíšete mezeru nebo interpunkci. Velikost písmen se řídí tím, co jste napsali: napíšete-li zkratku s velkým písmenem, i náhrada začne velkým písmenem. Backspace hned po rozvinutí vrátí jen tuto jednu náhradu. Zapíná se pro každý projekt zvlášť a liší se od [chytré interpunkce](:concept-smart-punctuation).
concept-smart-punctuation = Typografie podle jazyka, uplatňovaná automaticky při psaní: uvozovky, pomlčky, výpustky.
concept-smart-punctuation-more = Mění rovné uvozovky na typografické, -- na krátkou pomlčku a --- na dlouhou pomlčku, ... na výpustku a tam, kde to zapnete, otevře odstavec začínající „- “ dialogovou pomlčkou. Nastavuje se pro každý projekt a je uložena v souboru .skrib, takže spoluautor píše podle stejných pravidel. Liší se od [vašich vlastních pravidel](:concept-text-replacement), která si píšete sami.
concept-export-style = Pojmenovaná sada voleb exportu: nadpisy, mezery, co se zahrne.
concept-export-style-more = Jeden styl, použitelný pro každý export bez ohledu na zvolený formát. Vestavěné styly jsou jen pro čtení; duplikujte některý z nich a získáte výchozí bod, který můžete skutečně měnit. Mezi jeho volbami je i to, zda zahrnout [značky pro zpětný import](:concept-round-trip-marks), pro soubor, který putuje k redaktorovi a zpět.
concept-round-trip-marks = Neviditelné záložky, díky kterým se upravený DOCX nebo ODT vrátí domů rozpoznaný.
concept-round-trip-marks-more = Zapisují se jen do dvou formátů, které vám redaktor může poslat zpět, DOCX a ODT; žádný jiný export je nenese. Záložka, ne vlastní atribut: LibreOffice vlastní atributy při ukládání odstraňuje, ale záložku nechá netknutou. Zapínají se v každém [exportním stylu](:concept-export-style) v části „Odeslání redaktorovi“.
concept-status = Kde položka stojí ve vašem vlastním pracovním postupu: vždy jedna fáze, směrem vpřed.
concept-status-more = Není to [štítek](:concept-tag): položka má právě jeden stav, fáze jsou seřazené a seznam patří projektu, takže jej můžete kdykoli přejmenovat, přeuspořádat nebo doplnit. Hvězdička u kapitoly nebo knihy znamená, že části pod ní s ní všechny nesouhlasí. Nastavíte jej v inspektoru, na kartě, u položky v souvislém zobrazení nebo zde.
