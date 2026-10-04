# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

menu-work = Projekt (&W)
menu-new-work = Nový projekt (&N)
menu-open-work = Otevřít projekt… (&O)
menu-new-window = Nové okno (&W)
menu-import-from = Dovoz z (&I)
menu-create-from = Vytvořit z (&C)
menu-import-plume = Plume Creator (.plume)… (&P)
menu-import-document = Dokumenty (Markdown, Word, ODT) (&D)
menu-import-manuskript = Manuskript (.msk)… (&M)
menu-export = Exportovat (&X)
menu-export-book = Exportovat knihu
menu-export-part = Část exportu
menu-export-chapter = Kapitola o vývozu
menu-export-scene = Exportní scéna
menu-export-note = Exportovat poznámku
menu-export-paratext = Exportovat paratext
menu-export-folder = Exportovat složku
menu-export-choose = Zvolte možnost
menu-export-none = Otevřít dokument pro vývoz
menu-save = Uložit (&S)
menu-save-as-file = Uložit jako jeden soubor… (&F)
menu-save-as-folder = Uložit jako složku… (&D)
menu-backup = Zálohovat nyní (&B)
menu-close-work = Zavřít projekt (&C)
menu-welcome = Vítejte. (&L)
menu-settings = Nastavení (&E)
menu-quit = Ukončit (&Q)

menu-view = Zobrazení (&V)
menu-outline = Osnova (&O)
menu-search = Vyhledávání v projektu (&S)
menu-timeline = Vrať se v čase. (&G)
menu-search-preview = Náhled vyhledávání (&P)
menu-fullscreen = Fullscreen (&F)
menu-focus-mode = Režim distraktion- free (&D)
menu-format = Formát (&O)
menu-scene-break = Vložit průlom scény (&K)
menu-major-scene-break = Vložit průlom hlavní scény (&M)

menu-go = Přejít (&G)
menu-go-next-scene = Další scéna (&S)
menu-go-prev-scene = Předchozí scéna (&V)
menu-go-next-chapter = Další kapitola (&C)
menu-go-prev-chapter = Předchozí kapitola (&H)
menu-go-next-note = Další poznámka (&N)
menu-go-prev-note = Předchozí poznámka (&T)

menu-tools = Nástroje (&T)
menu-help = Nápověda (&H)

ctx-add = Přidat (&A)
ctx-new-item = Nová položka (&N)
ctx-new-folder = Nová složka (&F)
ctx-rename = Přejmenovat (&R)
ctx-duplicate = Duplikovat (&D)
ctx-import-here = Importovat zde (&I)
ctx-indent = Odhalení (&I)
ctx-outdent = Odhalení (&O)
ctx-trash = Přesunout do koše (&T)
ctx-open-to-side = Otevřít na stranu (&S)
ctx-open = Otevřít (&O)
ctx-reveal-in-outline = Odhalení v terénu (&L)
ctx-move-up = Posunout nahoru (&U)
ctx-move-down = Posunout dolů (&N)

ctx-tab-close = Zavřít (&C)
ctx-tab-close-others = Zavřít ostatní karty (&O)
ctx-tab-close-all = Zavřít všechny karty (&L)
ctx-tab-open-to-side = Otevřít na stranu (&S)
ctx-tab-open-in-main = Otevřít v hlavní tabuli (&I)
ctx-tab-move-to-side = Přesuňte se na stranu. (&M)
ctx-tab-move-to-main = Přesun do hlavního panelu (&M)
ctx-tab-move-to-new-window = Přesunout do nového okna (&N)
ctx-tab-move-window-unsaved = Uložit tento projekt jako první: do souboru na disku se otevře druhé okno
ctx-tab-pin = Připnout tuto kartu (&P)
ctx-tab-unpin = Odepnout tuto kartu (&U)
tab-pinned-tooltip = Pinned: Close others close and close all look it open

create-book = Kniha
create-part = Část
create-chapter = Kapitola
create-scene = Scéna
create-note = Poznámka
create-note-folder = Záznamová složka
create-folder = Složka
create-paratext = Paratext
create-paratext-folder = Složka Paratext
create-book-end = Konec knihy
create-story-bible-entry = Příběh bible vstup
type-book-start = Začátek knihy
type-text = Text

new-item-book = Nová kniha
new-item-part = Nová část
new-item-chapter = Nová kapitola
new-item-note = Nová poznámka
new-item-note-folder = Nová složka poznámek
new-item-folder = Nová složka
new-item-paratext = Nový paratext
new-item-paratext-folder = Nová paratextová složka
new-item-story-bible-entry = Nový příběh biblický vstup

placement-inside = uvnitř
placement-after = po
placement-after-parent = po mateřském podniku
placement-top-level = nejvyšší úroveň


ctx-promote = Převést do (&C)
promote-chapter-folder = Složka kapitoly
promote-flat-chapter = Samostatná kapitola
promote-lossy-title = Není kam si nechat text.
promote-lossy-text = A { $target } nemá kam uchovat tyto: { $kinds }. Přesunout nebo vyčistit, že text první, pak převést.
content-scene-text = Scénář
content-note-text = Poznámka
content-book-title = Název knihy
content-book-subtitle = Knižní titulek
content-part-title = Název části
content-chapter-title = Název kapitoly
content-epigraph-text = Motto
content-paratext-text = Paratext
promote-blocked-title = Kapitola není prázdná
promote-blocked-text = { $count ->
    [one] Tato kapitola stále obsahuje položku { $count }. Přesuňte ji nebo ji vyhoďte, než ji převedete na rovnou kapitolu.
   *[other] Tato kapitola stále obsahuje položky { $count }. Přesuňte je nebo je zlikvidujte, než je převedete na rovnou kapitolu.
}

inspector = Vlastnosti
inspector-empty = Otevřete položku, abyste ji zkontrolovali.
inspector-promote = Převést na crust
statusbar-toggle-outline = Přepnout pořadač
statusbar-toggle-inspector = Přepnout inspektora
statusbar-save-unsaved = Neuložené změny. Kliknutím uložíte
statusbar-save-saved = Všechny uložené změny
statusbar-save-autosave = Autosave je zapnutý. Změny se ukládají, když píšete
statusbar-saving = Záchrana
statusbar-word-count = { $count ->
    [one] { $count } slovo
    [few] { $count } slova
    [many] { $count } slova
   *[other] { $count } slov
}
statusbar-word-count-tooltip = Slova ve scéně, kterou editujete
statusbar-word-char-count = { $words ->
    [one] { $words } slovo
    [few] { $words } slova
    [many] { $words } slova
   *[other] { $words } slov
} · { $chars ->
    [one] { $chars } znak
    [few] { $chars } znaky
    [many] { $chars } znaku
   *[other] { $chars } znaků
}
session-toggle = Zapisovací relace: start nebo pauza cíleného sprintu
session-configure = Nastavit relaci slovo cíl a časový limit
session-configure-title = Zapisovací relace
session-word-goal = Cílový počet slov
session-time-limit = Lhůta
session-time-limit-unit = min
session-no-goal = Žádný cíl.
session-no-limit = Bez omezení
session-reset = Obnovit relaci
session-readout = { $words } slov · { $time }
session-readout-timed = { $words } slov · { $time } vlevo
statusbar-focus-exit = Exit distantion- free mode
statusbar-focus-synopsis = Shrnutí
statusbar-focus-go-prev = Předchozí položka (Alt + Up)
statusbar-focus-go-next = Další položka (Alt + Down)

rail-settings = Nastavení

language = Jazyk
theme = Motiv
english = Anglicky
french = Francouzština
light = Světlo
dark = Tmavé
settings-text-width = Šířka textu
settings-preview-width = Šířka náhledu vyhledávání
settings-autosave = Automatické ukládání na disk
settings-show-welcome = Zobrazit spouštěč při spuštění
settings-show-welcome-tip = Když končí, poslední projekt se místo toho znovu otevře.
settings-check-for-updates = Zkontrolujte nové verze
settings-check-for-updates-tip = Nejčastěji jednou denně se Skribisto zeptá na webovou stránku projektu Appens, která verze je aktuální. Požadavek neobsahuje žádný identifikátor, a dokonce ani verzi, kterou provozujete: srovnání se děje na tomto počítači.

settings-title = Nastavení
settings-close = Zavřít
settings-search = Nastavení vyhledávání
settings-reset = Obnovit do selhání
settings-done = Hotovo
settings-cancel = Zrušit
settings-apply = Použít
settings-ok = Dobře.
settings-reset-confirm-title = Obnovit všechna nastavení do selhání?
settings-reset-confirm-body = Obnovuje tuto aplikaci vzhled, editor a zápis nastavení. Váš projekt je vlastní nastavení, klávesové zkratky a uložené styly nejsou ovlivněny.
settings-empty-title = Zatím žádná nastavení.
settings-empty-hint = Tento oddíl získá možnosti v budoucí aktualizaci.

settings-sec-appearance-behaviour = Vzhled chování
settings-sec-editor = Editor
settings-sec-spelling = Hláskování
settings-sec-backup = Backup Sync
settings-sec-compile = Kompilovat export
settings-page-appearance = Vzhled
settings-field-image-size-policy = Velké obrázky
settings-image-policy-ask = Pokaždé se zeptej.
settings-image-policy-keep = Uchovat originální
settings-image-policy-downscale = Optimalizovat
settings-hint-image-size-policy = Co dělat, když vložený obraz je větší, než rukopis potřebuje.
settings-page-notifications = Oznámení
settings-page-scene = Scéna
settings-page-synopsis = Shrnutí
settings-page-notes = Poznámky
settings-page-editor-behavior = Chování editora
settings-page-goals = Cíle Počet slov
settings-page-games = Psaní hry
settings-page-corkboard = Nástěnka
settings-page-distraction-free = Distraction- free
settings-page-dictionaries = Slovníky
settings-page-autosave = Automaticky uložit
settings-page-export = Exportovat formáty
settings-page-paratext = Paratextové struktury
settings-page-keymap = Klávesa
settings-keymap-filter = Filtrační zkratky

settings-desc-sec-appearance-behaviour = Jak samotná aplikace vypadá a co dělá, když začne.
settings-desc-sec-editor = Psací plocha: jak vypadá a co dělá při psaní.
settings-desc-sec-spelling = Kontrola pravopisu a slovníky, které za tím stojí.
settings-desc-sec-backup = Jak se váš projekt dostane na disk a jaké kopie jsou uchovávány.
settings-desc-sec-compile = Co zbývá Skribisto, a v jakém stavu.
settings-desc-sec-work = Nastavení, která patří k tomuto projektu a cestují uvnitř jeho souboru.
settings-desc-sec-extensions = Stránky přispěly rozšířením nainstalovaných zde.
settings-desc-group-typography = Jedna stránka podle druhu textu: písmo, velikost, výška linky a rozestup.
settings-desc-appearance = Jazyk rozhraní, téma, velikost textu a spouštěč při spuštění.
settings-desc-notifications = Každá zpráva, kterou tato relace ukázala, a akce, které můžete přehrát.
settings-desc-scene = Jak je nastavena scéna próza: písmo, velikost, výška čáry, odznaky a rozestup.
settings-desc-synopsis = Jak je množina synopse nastavena, nezávisle na rukopisu.
settings-desc-notes = Jak jsou nastaveny poznámky, nezávisle na rukopisu.
settings-desc-corkboard = Velikost karty, co karta ukazuje, a jak je rada zařizuje.
settings-desc-distraction-free = Typografie, šířka sloupců a ovládací pásek pro psaní na celé obrazovce.
settings-desc-distraction-free-themes = Tématická knihovna pro celoobrazovkové psaní: ty odeslané a vaše vlastní.
settings-desc-editor-behavior = Šířka textu, pozice synovce, rolování psacích strojů a zvýraznění péče.
settings-desc-punctuation = Typografický dům začíná od nových projektů. Každý projekt následuje, pokud si nenastaví vlastní.
settings-desc-goals = Slovní a povahové cíle, a jak se slova počítají.
settings-desc-games = Omezuje vás to při navrhování, jako třeba vždy dopředu.
settings-desc-spellcheck = Jediný vypínač, který všude zapíná a vypíná.
settings-desc-dictionaries = Nainstalujte, odstraňte a projděte pravopisné slovníky na tomto stroji.
settings-desc-autosave = Zda jsou vaše edity napsány na disk na vlastní pěst.
settings-desc-backup = Při pořizování kopií, kde jsou uchovávány, a kolik. Každý projekt následuje, pokud si nenastaví vlastní.
settings-desc-export = Styly každého exportu sestavuje prostřednictvím: dodávané ty a vaše vlastní.
settings-desc-paratext = Přední a zadní hmota nového projektu může začít.
settings-desc-keymap = Každá zkratka a to, k čemu se váže.
settings-page-user = Uživatel
settings-desc-user = Kdo jste, za komentáře, které píšete.
settings-group-identity = Identita
settings-field-user-name = Vaše jméno
settings-field-user-name-placeholder = Nepovinné
settings-field-user-name-hint = Označuje komentáře a odpovědi, které napíšete. Odlišný od projektu Autoři jméno, který je knihou ithers byline a cestuje uvnitř souboru.
settings-field-user-initials = Vaše iniciály
settings-field-user-initials-placeholder = Z tvého jména
settings-field-user-initials-hint = Co slovo procesor ukazuje vedle vašeho komentáře v marži. Nechte prázdné použít ty zobrazené, převzaté z vašeho jména.
settings-field-user-hint = Oba jsou volitelné a vztahují se na každý projekt na tomto počítači. Změna je značí komentáře, které napíšete příště. Komentáře již napsané udržet jméno, které byly napsány pod.
settings-desc-author = Jméno, které je na tomto projektu.
settings-desc-structure = Zda tento projekt obsahuje kapitoly jsou složky nebo jednotlivé položky.
settings-desc-language = Jazyk tohoto projektu je kontrolován próza.
settings-desc-work-backup = Tento projekt má vlastní záložní politiku, nebo obecnou.
settings-desc-personal-dictionary = Slova, která tento projekt zachází jako správně napsané.
settings-desc-tags = Barvokódované značky tohoto projektu označují jeho položky.
settings-desc-templates = Šablony poznámek, které můžete vložit při zápisu do tohoto projektu.
settings-desc-text-replacements = Zkratky, které se rozšiřují, jak v tomto projektu píšete.
settings-desc-work-punctuation = Následují citáty, kolapsy a mezery v tomto projektu.

settings-group-typography = Typografie
settings-group-writing-column = Sloupec zápisu
settings-group-writing-view = Náhled na zápis
settings-group-distraction-free-strip = Ovládací pás
settings-group-theme = Motiv
settings-group-language = Jazyk
settings-group-startup = Spuštění
settings-group-updates = Aktualizace
settings-group-autosave = Automaticky uložit
settings-field-typeface = Type face
settings-field-size = Velikost textu

editor-size-changed-manuscript = Velikost textu rukopisu: { $percent }
editor-size-changed-synopsis = Velikost textu Synopsis: { $percent }
editor-size-changed-notes = Velikost textu poznámky: { $percent }
editor-size-changed-corkboard = Velikost písma karty: { $percent }
editor-size-changed-corkboard-expanded = Rozšířená velikost textu editoru: { $percent }
editor-size-changed-distraction-free = Distraction- bezplatná velikost textu: { $percent }

menu-text-size-increase = Zvyšte velikost textu (&I)
menu-text-size-decrease = Snížit velikost textu (&C)
menu-text-size-reset = Obnovit velikost textu (&R)
settings-field-line-height = Výška tratě
settings-field-first-line-indent = Odrážka první linie
settings-field-paragraph-spacing-before = Prostor před odstavcem
settings-field-paragraph-spacing-after = Prostor za odstavcem
settings-field-column-width = Šířka sloupce
settings-distraction-free-width-hint = Platí pouze při zapnutém rušicím režimu. Šíře sloupců Scéna, Synopsis a Notes jsou nedotčeny.
settings-distraction-free-title = Uložit název položky
settings-distraction-free-word-count = Počítej si to.
settings-distraction-free-session = Pokračovat ve psaní
settings-distraction-free-go-to = Ponechat tlačítko Přejít na tlačítko
settings-distraction-free-go = Udržujte předchozí a další tlačítka
settings-distraction-free-chrome-hint = Tlačítko Exit vždy zůstane, cokoliv si vyberete zde. Je to vaše cesta ven, pokud Escape je obsazeno.
settings-field-app-theme = Motiv
settings-theme-light = Světlo
settings-theme-dark = Tmavé
settings-field-text-scale = Velikost textu rozhraní
settings-field-language = Jazyk rozhraní
settings-synopsis-placement = Pozice synopsie
settings-synopsis-placement-none = Žádné
settings-synopsis-placement-top = nad
settings-synopsis-placement-side = Vedle
settings-typewriter = Psací stroj rolování
settings-typewriter-tip = Drží řádek, který píšete v pevné výšce, zatímco rukopis sviští pod ním. Kliknutím stále klape, kam kliknete.
settings-typewriter-position = Linie
settings-typewriter-position-top-third = Horní třetí
settings-typewriter-position-middle = Střední
settings-typewriter-position-bottom-quarter = Spodní čtvrtletí
settings-highlight-scope = Zvýraznění kolem košíku
settings-highlight-scope-none = Žádné
settings-highlight-scope-sentence = Rozsudek
settings-highlight-scope-paragraph = Odstavec
settings-highlight-scope-tip-none = Nic není zastíněno, jak píšete.
settings-highlight-scope-tip-sentence = Zapiš větu, kterou píšeš, takže to vyčnívá z těch kolem.
settings-highlight-scope-tip-paragraph = Zapište celý odstavec, který píšete, abyste udrželi průchod, na kterém pracujete.
settings-group-container-views = Náhled kontejneru
settings-remember-view = Zapamatovat si poslední pohled pro každý typ položky
settings-remember-view-tip = Otevřete kontejner na pohledu, který jste naposledy použili pro tento typ
settings-remember-view-tip-more =
    Kniha, část a kapitola každý nabízí několik názorů (jeho vlastní stránka, plná
    rukopis, plný synopse). Zapněte to a každý typ znovu otevře na
    zobrazit jste si naposledy vybrali pro něj, např. přepnout jednu kapitolu na celou kapitolu a
    další kapitola, kterou otevřete, také ukazuje celou kapitolu. Každý typ si pamatuje svůj vlastní názor.
settings-group-counting = Počítání slov
settings-counting-auto = Automatické (podle jazyka)
settings-counting-whitespace = Rozdělit na mezery
settings-counting-unicode-words = Unicode slova
settings-counting-cjk-hybrid = CJK-aware (na znak)
settings-counting-hint = Automatické počítá čínština a japonština podle charakteru a každý jiný jazyk podle slova. Změnit ji pouze v případě, že počet vypadá špatně pro váš jazyk.
settings-group-goals-display = Zobrazení
settings-show-characters = Zobrazit počet znaků ve stavové liště
settings-autosave-hint = Změny se zapisují automaticky na disk při zápisu.

settings-sec-work = Projekt
settings-page-structure = Struktura
settings-page-author = Autor
settings-field-author-name = Jméno autora
settings-field-author-placeholder = Nepovinné
settings-field-author-hint = Objevuje se na sestavené titulní stránce a ve vyvážených metadatech souborů. Zanechte ji prázdnou, abyste ji vynechali. Jedná se o knižní titulek. Jméno, s nímž jsou vaše komentáře podepsány, je v Nastavení Klienta.
settings-group-chapters = Kapitola
tidy-titles-title = Tiché titulky kapitol
tidy-titles-none = Žádná kapitola nebo část titul je pouze opakování své vlastní číslo.
tidy-titles-lead = { $count ->
        [one] Jedna kapitola nebo část je pojmenována pouze svým vlastním číslem.
       *[other] { $count } kapitol a částí jsou pojmenovány pouze jejich vlastní číslo.
    }
tidy-titles-explain = Čištění těchto titulů zanechává každý pojmenovaný podle čísla, které kniha již zná, stejné číslo exportních tisků. Nic jiného se nezmění, a jeden odpis je všechny vrátí zpět.
menu-document-tidy-titles = Tiché tituly kapitoly (&Y)
settings-group-numbering = Číslo
settings-number-chapters = Kapitola a části čísla
settings-number-chapters-tip = V kapitolách a částech je uvedeno číslo vyhotovené z místa, kde sedí v knize, uvedené vedle jejich názvu zde a vytištěné vývozem.
settings-number-chapters-tip-more = Číslo není nikdy uloženo v názvu, takže zůstane správné, když si objednáte, vložte nebo smažete. Vypněte to a exportovat tiskne tituly sám, ať už styl exportu požaduje cokoliv. Chcete-li nechat jednu kapitolu venku (prolog, interlude), použijte místo toho přepínač Inspektor Côtes Numbering: který jej udržuje v knize, ale zastaví to s číslem.
settings-part-resets-chapter = Restartovat čísla kapitol na každé části
settings-part-resets-chapter-tip = Vypnuto ve výchozím nastavení: kapitoly běží přímo po částech knihy, takže druhá část se otevírá na kapitole Jedenáct.
settings-part-resets-chapter-tip-more = To je obvyklá obchodní praxe, a to, co čtenář očekává. Zapněte ji pro knihu, jejíž části jsou určeny ke čtení jako samostatné objemy, kde každý z nich otevírá na Kapitola1.
settings-chapter-flat = Ploché kapitoly
settings-chapter-flat-hint = Na: kapitola je jedna položka. Napíšete do ní, a to nemá žádné scény. Off: kapitola je složka. Stále do ní zapíšete, ale to může držet scény stejně. Nové kapitoly následovat toto nastavení, existující konvertovat prostřednictvím propagace.

settings-styles-builtin = Stavební- ve stylech
settings-styles-user = Moje styly
settings-styles-builtin-badge = Built- in
settings-styles-duplicate = Duplikovat
settings-styles-edit = Úpravy
settings-styles-delete = Smazat
settings-styles-import = Importovat…
settings-styles-export = Exportovat…
settings-styles-copy-suffix = (kopie)
settings-styles-json-filter = Styl exportu
settings-styles-editor-title = Upravit styl
settings-styles-editor-none = Vyberte vlastní styl pro úpravu, nebo duplikovat built- v jednom.
settings-styles-imported = Importovaný styl
settings-styles-import-failed = Nelze importovat styl
settings-styles-exported = Exportovaný styl
settings-styles-export-failed = Nelze exportovat styl
settings-styles-field-name = Název
settings-styles-field-chapters = Kapitola
settings-styles-field-parts = Části a součásti
settings-styles-field-scene-break = Protržení scény
settings-styles-field-major-scene-break = Přestávka na velkou scénu
settings-styles-field-spacing = Rozhraní tratí
settings-styles-field-justify = Odůvodnit text
settings-styles-field-notes = Zahrnout poznámky
settings-styles-field-synopses = Zahrnout synovce
settings-styles-field-scene-titles = Zahrnout názvy scén
settings-styles-field-epigraphs = Zahrnout epigrenii
settings-styles-field-paratexts = Zahrnout paratexty
settings-styles-field-epigraph-placement = Pozice epigrafu
settings-styles-epigraph-after = Po názvu
settings-styles-epigraph-before = Před titulem
settings-styles-field-footnotes = Zahrnout poznámky pod čarou
settings-styles-field-footnote-numbering = Číslování poznámky pod čarou
settings-styles-footnote-numbering-continuous = Kontinuální
settings-styles-footnote-numbering-per-chapter = Restartovat každou kapitolu
settings-styles-footnote-numbering-per-book = Restartovat každou knihu
settings-styles-field-images = Obrázky
menu-image = Obrázek (&I)
settings-styles-images-beside = Vedle dokumentu
settings-styles-images-embed = Uvnitř dokumentu
settings-styles-images-omit = Vynechte to.
settings-styles-group-round-trip = Odeslání editoru
settings-styles-field-comments = Zahrnout připomínky
settings-styles-field-round-trip-marks = Zahrnout značky kruhových výletů
settings-styles-round-trip-hint = Pouze DOCX a ODT. Značky jsou neviditelné identifikátory, které umožňují vrátit soubor aktualizovat tento projekt místo přistání vedle něj jako druhou kopii.
settings-styles-group-pages = Stránky
settings-styles-field-cover = Otevřít krytem
settings-styles-field-word-count = Počet slov na titulní straně
settings-styles-field-page-books = Nová stránka v každé knize
settings-styles-field-page-parts = Nová stránka na každé části
settings-styles-field-page-chapters = Nová stránka v každé kapitole
settings-styles-field-page-paratexts = Nová stránka v každém paratextu
settings-styles-heading-none = Bez názvu
settings-styles-heading-numbered = Pouze číslo
settings-styles-heading-title = Pouze název
settings-styles-heading-both = Číslo + název
settings-styles-break-blank = Prázdná čára
settings-styles-break-none = Žádné
settings-styles-spacing-single = Jednoduché
settings-styles-spacing-onehalf = 1½
settings-styles-spacing-double = Dvojité

welcome-title = Vítejte vSkribisto.
welcome-version = Verze { $version }
welcome-update-available = Verze { $version } je k dispozici
welcome-update-tip = Otevře stránku ke stažení ve vašem prohlížeči.
welcome-search = Projekty vyhledávání
welcome-open = Otevřít
welcome-new-work = Nový projekt
welcome-create-from = Vytvořit z iqtiqe
welcome-recent-works = Nedávné projekty
welcome-empty-recents = Zatím žádné nové projekty.
welcome-no-matches = Žádný nedávný projekt se neshoduje s vaším hledáním.
welcome-learn-soon = Průvodce a tipy brzy přijdou.
welcome-about-blurb = Skribisto, a Rust + Teksilo přepsat psací aplikace.
welcome-tagline = *Klidné místo pro psaní dlouhých textů.*
welcome-github = GitHub
welcome-discord = Discord
nav-works = Projekty
nav-examples = Příklady
nav-learn = Naučte se
nav-about = O

synopsis = Shrnutí
epigraph = Motto
pane-manuscript = Rukopis
corkboard = Nástěnka
overview = Přehled

corkboard-card-count = { $count ->
    [one] { $count } karta
   *[other] { $count } karet
}
corkboard-child-count = { $count ->
    [one] { $count } položka
   *[other] { $count } položek
}
corkboard-view-nested = Hierarchické
corkboard-view-flat = Ploché
corkboard-layout-hint = Nested ukazuje kontejner přímé děti. Otevřete kartu složky jít dovnitř. Byt ukazuje každou scénu v celém kontejneru najednou.
corkboard-card-size = Velikost karty
corkboard-search-placeholder = Filtrační karty
corkboard-empty-title = Zatím tu nic není.
corkboard-empty-hint = Použijte tlačítko nové výše přidat první.
corkboard-new = Nový
corkboard-show-card-numbers = Číslo karet
corkboard-modal-size = Rozšířená velikost editoru
corkboard-scope-hint = Platí pro všechny otevřené desky, v každém projektu.
corkboard-sort-manuscript = Příkaz k rukopisu
corkboard-sort-title-asc = Hlava A CONTROLZ
corkboard-sort-title-desc = Hlava Z TABULKA
corkboard-card-number = Průkaz { $number }

duplicate = Duplikovat
duplicate-n = { $count ->
    [one] Duplikát { $count } karta
   *[other] Duplikát { $count } karet
}
set-label-n = { $count ->
    [one] Nastavit štítek na kartu { $count }
   *[other] Nastavit štítek na { $count } kartách
}
move-to-trash-n = { $count ->
    [one] Přesunout { $count } kartu do koše
   *[other] Přesunout { $count } karet do koše
}
reveal-in-outline = Odhalení v obrysu

corkboard-move-to = Přesuňte se do centra.
corkboard-move-to-n = { $count ->
    [one] Přesunout { $count } kartu do
   *[other] Přesunout { $count } karet do
}
corkboard-move-picker-title = Přesuňte se do centra.
corkboard-move-picker-empty = Zatím žádné pořadače.
corkboard-move-picker-cancel = Zrušit
corkboard-move-here = Přesuňte se sem.
corkboard-moved-ok = { $count ->
    [one] { $count } karta přesunuta
   *[other] { $count } karet přesunuto
}
corkboard-move-into-self = Kontejner nemůže být přemístěn sám dovnitř. Vyberte si místo mimo něj.
corkboard-move-failed = Ty karty se tam nedá přesunout.

overview-col-title = Název
overview-col-type = Typ
overview-col-label = Popisek
overview-col-tags = Štítky
overview-col-books = Knihy
overview-col-own-words = Slova
overview-col-total-words = Celkem
overview-row-count = { $count ->
    [one] { $count } položka
   *[other] { $count } položek
}
overview-search-placeholder = Filtrační položky
overview-expand-all = Rozšiřte vše
overview-collapse-all = Zhroutit vše
overview-table-label = Obsah
overview-empty-title = Zatím tu nic není.
overview-gone-title = Ten kontejner je pryč.
overview-gone-hint = Byl přesunut do koše. Obnovit, nebo uzavřít tuto kartu.
overview-empty-hint = Použijte tlačítko nové výše přidat první.
corkboard-grid-label = Karty na Corkboard
corkboard-rename-field = Přejmenovat položku
corkboard-expand-synopsis = Rozšiřující synopse
corkboard-synopsis-modal-title = Shrnutí
corkboard-badge-scene = Scéna
corkboard-badge-chapter = Kapitola
corkboard-badge-part = Část
corkboard-badge-book = Kniha
corkboard-badge-note = Poznámka
corkboard-badge-folder = Složka
corkboard-badge-paratext = Paratext
corkboard-badge-text = Text
corkboard-badge-end = Konec
settings-group-corkboard-layout = Rozvržení
settings-group-corkboard-cards = Karty
corkboard-show-word-count = Zobrazit počet slov na kartách
text-heading = Text
no-content = Tato položka nemá editovatelný obsah.
untitled = Bez názvu
placeholder-title = Název…
placeholder-subtitle = Subtitle
placeholder-chapter-title = Název kapitoly
split-editor = Rozdělený editor
close-split-view = Zavřít rozdělený pohled
drop-open-here = Otevři tady.
drop-open-to-side = Otevřeno na stranu

segment-chapter = Kapitola
segment-part = Část
segment-book = Kniha
segment-pace = Pece
segment-notes = Poznámky
segment-story-bible = Encyklopedie příběhu
segment-note-own = Poznámka
segment-note-details = Podrobnosti
segment-note-in-prose = V próze
pace-placeholder = Objeví se tu plánovač Pace.
pace-empty-title = Naplánujte tuto knihu tempo
pace-empty-body = Nastavit slovo cíl a termín, a Skribisto Projekty denní tempo, aby vás tam.
pace-start-planning = Začněte plánovat
pace-section-schedule = Harmonogram
pace-section-progress = Průběh
pace-advancement = Pokrok
pace-goal = Cílový počet slov
pace-deadline = Termín
pace-active = Aktivní pace
pace-day-mon = Po
pace-day-tue = Út
pace-day-wed = St
pace-day-thu = Thu.
pace-day-fri = Fri.
pace-day-sat = Sad
pace-day-sun = Slunce
pace-card-written = napsaná slova
pace-card-of-goal = cíle
pace-card-rate = slova / den psaní
pace-card-days-left = psaní zbývají dny
pace-card-streak = denní pruh
pace-card-ahead = slova před tempem
pace-card-behind = slova za tempem
pace-charts-empty = Grafy pokroku se zde objeví, jakmile je váš počet slov zaznamenán na úspoře.
pace-chart-progression = Slova napsaná vs. cíl
pace-chart-words-per-day = Slova za den
pace-series-actual = Skutečný
pace-series-target = Cíl
pace-series-words-per-day = Slova / den
pace-daily-target-line = Even temp: { $count ->
    [one] { $count } slovo / den
   *[other] { $count } slov / den
}
pace-section-holidays = Dovolená
pace-section-milestones = Milníky
pace-holidays-none = Každý plánovaný den se počítá.
pace-holiday-label = Prázdninové jméno
pace-add-holiday = Přidat
pace-remove = Odebrat
pace-milestones-none = Zatím žádné milníky, nastavte jeden na část nebo kapitolu v inspektorovi.
full-chapter = Celá kapitola
full-part = Plná část
full-book = Úplná kniha
full-synopsis = Kompletní synopsie
rename = Přejmenovat
set-label = Nastavit štítek
insert-scene = Vložit scénu
insert-chapter = Vložit kapitolu
split-scene = Rozdělené scény
move-up = Posunout nahoru
move-down = Posunout dolů
merge-with-previous = Sloučit s předchozím
move-to-trash = Přesunout do koše
rename-chapter = Přejmenovat kapitolu
add-scene = Přidat novou scénu
add-chapter = Přidat novou kapitolu
new-scene-title = Nová scéna
placeholder-scene-name = Název scény
menu-cut = Vyjmout
menu-copy = Kopírovat
menu-paste = Vložit
menu-paste-unformatted = Vložit bez formátování
menu-select-all = Vybrat vše
format-bold = Tučné
format-italic = Italština
format-underline = Podtrženo
format-strikethrough = Prorazit.

binder = Strom dokumentů
no-work = Žádný projekt není načten
no-recent-works = Žádné nedávné projekty
switcher-open-section = V současné době otevřené
switcher-recent-section = Nedávné
switcher-this-window = Toto okno
open-project-title = Otevřený projekt
open-project-question = Jak chcete otevřít CLAS{ $title }?
open-in-new-window = Otevřít v novém okně
open-here = Otevři tady.

binder-all = Všechny pojiva
binder-show-all = Zobrazit všechny pořadače
binder-new = Nový pojivo
binder-default-name = Nový pojivo
binder-item-count = { $count ->
    [one] { $count } položka
   *[other] { $count } položek
}
binder-search-placeholder = Filtrovat obrys
binder-search-scope = Hledat všechny pořadače
binder-trash-confirm-title = Přesunout pořadač do koše?
binder-trash-confirm-text = "{ $name }A všechny jeho věci budou přesunuty do koše.

dialog-rename = Přejmenovat
dialog-new-binder = Nový pojivo
dialog-set-label = Nastavit štítek
dialog-new-scene = Nová scéna
close-work-question = Uložit změny před zavřením projektu?
quit-question = Uložit změny před ukončením?
quit-save-work-question = Uložit změny na { $title } před skončením?
unsaved-changes = Tento projekt obsahuje neuložené změny.
new-work-unsaved-question = Uložit změny před vytvořením nového projektu?
open-work-unsaved-question = Uložit změny před otevřením jiného projektu?
switch-backup-discard-title = Zbavit se změn v záloze?
switch-backup-discard-text = Změny zálohování lze uložit na něj. Použijte Uložit jako nebo obnovit je, nebo je vyhodit a otevřít druhý projekt.
switch-save-failed = Projekt mohl být zachráněn, takže byl nahrazen: { $error }
switch-save-not-started = Projekt mohl být zachráněn, takže byl nahrazen.
close-save-failed = Projekt mohl být zachráněn, takže byl uzavřen: { $error }
close-save-not-started = Projekt mohl být zachráněn, takže byl uzavřen.
save-not-started = Projekt by mohl být zachráněn.

could-not-open-work = Nelze otevřít „{ $file }“
could-not-open-work-too-new = „{ $file }“ vyžaduje novější verzi Skribisto
could-not-open-work-too-new-detail = Soubor byl uložen ve formátu Skribisto { $written_by }. K otevření je nutný formát { $requires } nebo novější, ale tato verze podporuje pouze formáty do { $supported }. Aktualizujte Skribisto.
could-not-open-work-importing = Do „{ $file }“ právě zapisuje import
could-not-open-work-importing-detail = Import právě přepisuje tento projekt a ještě neskončil. Otevřete projekt až po dokončení importu. Při otevření nyní by se zobrazil původní projekt a jeho další uložení by přepsalo importovaná data.
could-not-open-example = Nelze otevřít příklad: { $error }
could-not-open-link = Nelze otevřít { $url }: { $error }
could-not-create-work = Nelze vytvořit projekt: { $error }
saving-as-file = Uložení jako { $target }
saving-as-folder = Uložení jako { $target } / RRRR
saved-as = Uloženo na { $target }
save-error = Nelze uložit: { $error }
save-flush-failed = Uložení proběhlo, ale tyto dokumenty nemohly být předány a jsou stále neuložené: { $titles } ({ $error })
backup-error = Nelze zálohovat: { $error }
backing-up = Balení
backup-nothing-open = Žádný projekt není otevřený k zálohování.
backup-already-running = Záloha už probíhá.
backup-complete = Záloha kompletní ({ $ok } uloženo, { $skipped } již aktuální)
backup-partial = Záloha dokončena: { $ok } uloženo, { $failed ->
    [one] { $failed } cíl selhalo
   *[other] { $failed } destinací selhalo
}
backup-no-destination-title = Není k dispozici žádná záložní poloha
backup-no-destination-text = Žádný z nakonfigurovaných zálohovacích cílů není dosažen (např. externí pohon může být odpojen). Zapojte jej a znovu zkuste, nebo vystupte bez zálohování.
backup-failed-close-title = Záloha nemohla být uložena
backup-failed-close-text = Před zavřením nemohla být zapsána žádná záložní kopie. Každá destinace selhala (disk mohl být odstraněn, nebo může být plný nebo write- protected). Opravte ji a retry, nebo opusťte bez zálohování.

backup-choice-title = Zálohový soubor
backup-choice-heading = Otevřel jsi záložní kopii.
backup-choice-subtitle = Tohle je záloha projektu v čase.
backup-choice-subtitle-dated = Zálohování zabralo{ $date }.
backup-choice-body = Můžete jej otevřít a editovat volně, ale změny lze udržovat pouze s Uložit jako. Původní soubor projektu se nedotkne. Nebo tento projekt obnovit přesně na tuto zálohu.
backup-choice-open = Otevřete zálohu.
backup-choice-restore = Obnovit tento projekt do tohoto bodu
backup-choice-not-a-backup = Ne, normálně to otevři.
backup-banner-title = Zálohová kopie: změny mohou být uloženy zde
backup-banner-description = Použít Uložit jako udržet své edity v novém souboru, nebo Obnovit nahradit původní projekt s touto zálohou.
backup-banner-restore = Obnovit…
backup-banner-save-as = Uložit jako
backup-restore-original-missing = Nelze najít původní projekt obnovit. Použijte Uložit jako ponechat tuto zálohu jako nový projekt místo.
backup-restore-close-elsewhere-title = Projekt otevřen v jiném okně
backup-restore-close-elsewhere-text = Projekt, který obnovujete, je otevřen v jiném okně. Prosím, zavřete to tam první, pak to zkuste znovu.
backup-restore-focus-window = Zaostři to okno.
backup-restore-confirm-title = Obnovit tu zálohu?
backup-restore-confirm-text = Současná verze projektu bude před tím, než bude nahrazena tímto, zkopírována jako bezpečnostní záloha.
backup-restore-confirm-ok = Obnovit
backup-restore-error = Nelze obnovit: { $error }
backup-restored-ok = Projekt obnoven.
backup-restored-with-safety = Projekt obnoven. Vaše předchozí verze byla uložena na{ $path }.
close-backup-discard-title = Zbavit se změn v záloze?
close-backup-discard-text = Změny zálohování lze uložit na něj. Použijte Uložit jako udržet je, nebo vyhodit a zavřít.
quit-backup-discard-title = Zbavit se změn a skončit?
quit-backup-discard-work-question = Zbavit se změn na { $title } a skončit?
quit-backup-discard-text = Změny zálohování lze uložit na něj. Použijte Uložit jako je, aby je, nebo vyhodit a skončit.
backup-nudge-text = Pro tento projekt nejsou nastaveny žádné zálohy.
backup-nudge-action = Nastavit zálohy

menu-backups-list = Zálohy… (&K)
backups-title = Zálohy
backups-loading = Skenování záloh
backups-empty = Pro tento projekt zatím nebyly nalezeny žádné zálohy.
backups-open = Otevřít
backups-reveal = Odhalení
backups-delete = Smazat tuto zálohu
backups-delete-confirm-title = Smazat tu zálohu?
backups-delete-confirm-text = „{ $name }“ bude trvale smazáno. Tuto akci nelze vrátit zpět.
backups-delete-error = Nelze smazat zálohu: { $error }
backups-refresh = Obnovit
backups-close = Zavřít

settings-page-backup = Zálohování závad
settings-page-work-backup = Zálohy
settings-backup-general-title = Výchozí nastavení zálohování
settings-backup-work-title = Zálohy pro tento projekt
settings-backup-inherit = Použít obecné nastavení zálohování
settings-backup-inheriting = Tento projekt používá obecné nastavení záloh.
settings-backup-none-hint = Žádné automatické zálohování není nastaveno (každý spouštěč je vypnutý).
settings-backup-last = Poslední záloha: { $date }
settings-backup-last-never = Zatím žádné zálohy.
settings-backup-open-list = Otevřít seznam záloh
settings-backup-triggers = Kdy zálohovat
settings-backup-on-close = Zálohovat při uzavření projektu
settings-backup-on-open = Zálohovat při otevření projektu
settings-backup-interval = Zálohovat pravidelně, každý
settings-backup-destinations = Zálohové destinace
settings-backup-default-location = Výchozí umístění
settings-backup-usage-since = { $count ->
    [one] { $count } Záloha · { $size } · nejstarší { $oldest }
   *[other] { $count } zálohy · { $size } · nejstarší { $oldest }
}
settings-backup-usage = { $count ->
    [one] { $count } záloha · { $size }
   *[other] { $count } zálohy · { $size }
}
settings-backup-usage-empty = Zatím tu nic nezůstalo.
settings-backup-usage-measuring = Měření
settings-backup-reveal-root = Zobrazit složku
settings-backup-dest-none = Žádné cíle. Zálohy jsou uloženy vedle projektu.
settings-backup-dest-remove = Odebrat
settings-backup-dest-add = Přidat složku
settings-backup-dest-refresh = Obnovit
settings-backup-retention = Kolik si jich nechat?
settings-backup-retention-tiered = Pneumatiky
settings-backup-retention-keep-n = Ponechat poslední N
settings-backup-retention-tip = Jak staré zálohy jsou prostříhané.
settings-backup-retention-tip-more = Tiered udržuje jednu zálohu za hodinu na den, za den na týden, za týden na měsíc, a za měsíc po něm, takže poslední historie zůstává hustá a stará historie se tenčí ven. The keep last N 'rest keeps the N last copy. V obou režimech jsou vždy nejnovější kopie (minimum níže).
settings-backup-gfs-hourly = Hodiny (posledních 24 h)
settings-backup-gfs-daily = Denně (minulý týden)
settings-backup-gfs-weekly = Týdenní (poslední měsíc)
settings-backup-gfs-monthly = Měsíční
settings-backup-keep-n = Číslo, které je třeba uchovávat
settings-backup-min-keep = Vždy mějte alespoň
settings-backup-dedup = Přeskočit zálohu, když se nic nezměnilo.

new-work-title = Nový projekt
new-work-close = Zavřít
new-work-name = Název projektu
new-work-name-placeholder = Bez názvu
new-work-author = Autor
new-work-author-placeholder = Nepovinné
new-work-format = Formát
new-work-single-file = Jednotný soubor
new-work-bundle = BundleCity in New York USA
new-work-convert-later = Můžete konvertovat mezi formáty později.
new-work-location = Umístění
new-work-will-create = Vytvořit
new-work-language = Výchozí jazyk
new-work-language-hint = Aplikováno na nové texty spellcheck. Každý text lze přepnout do jiného jazyka.
new-work-template = Šablona
new-work-template-none = Žádné
new-work-template-empty-novel = Prázdný román
new-work-template-light-novel = Světlý román
new-work-template-novel = Román
new-work-template-novel-in-parts = Novel v částech
new-work-template-notebook = Zápisník
new-work-cancel = Zrušit
new-work-create = Vytvořit projekt
new-work-step-details = Podrobnosti
new-work-step-language = Jazyková struktura
new-work-step-template = Šablona
new-work-back = Zpět
new-work-next = Další
new-work-documents-title = Nový projekt z dokumentů
new-work-step-import = Importovat
new-work-create-and-import = Vytvořit import
new-work-documents-next-title = Vaše dokumenty jsou na řadě.
new-work-documents-next-body = Vytvoření projektu otevírá import průvodce nad ním, kde si vyberete dokumenty, přezkoumat strukturu Skribisto čte z nich, a vybrat, kde to přistane. Nic není zapsáno do projektu, dokud si to tam potvrdit.
new-work-documents-no-template = Tento projekt začíná naschvál prázdný: žádná šablona, takže dovážené dokumenty jsou jedinou věcí v něm.
new-work-documents-chapter-scene-hint = Platí pro kapitoly, které import vytváří. Můžete jej později změnit v nastavení projektu.
new-work-single-file-desc = Jeden .skrib archiv (zip). Přenosný, snadno zálohovat.
new-work-bundle-desc = Složka s každým textovým aktivem, přátelštější ke kontrole verzí.
new-work-template-none-count = prázdný pojivo
new-work-template-empty-novel-count = 1 kapitola
new-work-template-light-novel-count = 15 kapitol
new-work-template-novel-count = 20 kapitol
new-work-template-novel-in-parts-count = 3 části, 24 kapitol
new-work-template-notebook-count = noty ve volném tvaru
new-work-chapter-scene = Ploché kapitoly
new-work-chapter-scene-tip = Každá kapitola je jedna položka, do které napíšete přímo, bez scén pod ním. Zanechte to pro klasické rozložení, kde kapitola je složka: zapíšete přímo do ní také, ale to může držet scény stejně.
new-work-chapter-scene-tip-more = Zapíšete do kapitoly tak jako tak. Jediný rozdíl je v tom, zda to může *obsahovat* scény. Skribisto Vazivo strom je organizační pouze, takže obě rozložení compile do stejné knihy, můžete je volně míchat, a Promote konvertuje kapitolu mezi oběma, aniž by ztratil slovo.
new-work-name-required = Zadejte název projektu
new-work-name-invalid = Tento název nemá žádné použitelné znaky
new-work-location-required = Vyberte si místo
new-work-location-missing = Tato složka neexistuje
new-work-location-not-folder = Tato cesta není složka
new-work-location-readonly = Tato složka není zapisovatelná
new-work-target-exists = Projekt s tímto názvem je již v této složce. Vytvoření projektu se bude ptát před jeho nahrazením
new-work-target-is-folder = Složka s tímto názvem je již v této lokalitě. Vyberte si jiný název
new-work-overwrite-title = Nahradit existující projekt?
new-work-overwrite-text = V této složce již existuje { $name }. Nahradit jej novým projektem?
new-work-target-open-title = Tento projekt je otevřený.
new-work-target-open-text = „{ $name }“ je otevřen ve Skribisto. Vytvoření nového projektu na stejném místě by nahradilo projekt, na kterém pracujete, a jeho další uložení by přepsalo nový projekt. Zvolte jiný název nebo složku, případně nejprve projekt zavřete.
target-importing-title = Import je psaní tohoto projektu
target-importing-text = Import stále zapisuje do „{ $name }“. Počkejte na dokončení nebo zvolte jiný název.
import-plume-busy-title = Další Plume Creator projekt je dovážen
import-plume-busy-text = Plume Creator projekty jsou importovány jeden po druhém. Počkejte, až bude dokončen import, nebo jej zrušte, pak začněte znovu.
import-manuskript-busy-title = Další Manuskript projekt je dovážen
import-manuskript-busy-text = Manuskript projekty jsou importovány jeden po druhém. Počkejte, až bude dokončen import, nebo jej zrušte, pak začněte znovu.
backup-restore-importing-text = Import je stále psaní CLAS{ $name }, projekt tato záloha by být obnoven. Obnovit zálohu, jakmile import skončil.

new-work-manuscript = Rukopis
new-work-notes = Poznámky
new-work-research = Výzkum
new-work-notebook = Zápisník
new-work-scene = Scéna
new-work-note = Poznámka
new-work-front-matter = Přední část
new-work-back-matter = Záda
new-work-paratext = Struktura knihy
new-work-paratext-none = Žádná struktura
new-work-paratext-hint = Na přední a zadní straně záleží, že tradice otevře a zavře knihu s. Můžete přesunout, přejmenovat nebo smazat některou z nich později.
new-work-tags = Štítky
new-work-tags-none = Žádné štítky
new-work-tags-hint = Počáteční paleta pro označení znaků, míst a ostatních. Volitelné, a každý štítek lze přejmenovat, reklored nebo smazat později.
new-work-note-templates = Šablona poznámek
new-work-note-templates-none = Bez šablon
new-work-note-templates-hint = Tvar příběh-bible poznámka začíná v. Volitelné, a každá šablona může být editována nebo smazána později.
new-work-characters = Postavy
new-work-places = Místa

import-plume-title = Import Plume Creator projektu
import-plume-close = Zavřít
import-plume-source = Projekt Plume
import-plume-source-hint = Vyberte soubor .plume nebo .plume_backup (libovolná verze Plume Creator).
import-plume-location = Adresář určení
import-plume-name = Název souboru
import-plume-name-placeholder = Název projektu
import-plume-will-create = Vytvořit
import-plume-trash-warning = Klikaté / vymazané položky se nepřemisťují.
import-plume-cancel = Zrušit
import-plume-import = Importovat
import-plume-source-required = Vyberte soubor s projektem Plume
import-plume-source-missing = Tento soubor neexistuje
import-plume-source-not-file = Tato cesta není soubor
import-plume-location-required = Vyberte cílovou složku
import-plume-location-missing = Tato složka neexistuje
import-plume-location-not-folder = Tato cesta není složka
import-plume-location-readonly = Tato složka není zapisovatelná
import-plume-name-required = Zadejte název souboru
import-plume-name-exists = Soubor s tímto názvem již zde existuje. Import potvrdí přepsání
import-plume-overwrite-title = Nahradit existující soubor?
import-plume-overwrite-text = Již existuje { $name }. Nahradit ho importovaným projektem?
import-plume-manuscript-binder = Rukopis
import-plume-story-bible-binder = Encyklopedie příběhu
import-plume-progress-title = Importování projektu Plume
import-plume-cancel-import = Zrušit
import-plume-cancelled = Import zrušen
import-plume-done = Dovezené { $imported ->
    [one] Položka { $imported }.
   *[other] { $imported } položek.
 } { $skipped ->
    [one] { $skipped } zdemolované zboží nebylo přemístěno.
   *[other] { $skipped } zdemolované předměty nebyly migrovány.
}
import-plume-open-now = Otevři teď.
import-plume-warnings = { $count ->
    [one] 1 věc nemohla být dovážena přesně
   *[other] { $count } věci nemohly být dováženy přesně
}
import-plume-details = Podrobnosti
import-plume-warnings-title = Oznámení o dovozu
import-plume-error-title = Nelze importovat projekt
import-plume-nested-too-deep = Projekt nebyl importován: jeho projekt{ $part }Samotná část je více než vnořená než { $limit } Stupeň hloubky. Plume Creator Projekt se k tomu přiblíží a čtení by mohlo spadnout. SkribistoNic nebylo napsáno.
import-plume-declares-entities = Projekt nebyl importován: jeho projekt{ $part }Uvádí XML jednotky, které ne Plume Creator Rozšíření by mohlo vzít více paměti, než počítač má a zřítit SkribistoNic nebylo napsáno.
import-plume-error-details = Podrobnosti

import-manuskript-title = Import Manuskript projektu
import-manuskript-close = Zavřít
import-manuskript-source = Manuskript projekt
import-manuskript-source-hint = Vyberte soubor .msk nebo složku projektu (libovolná verze Manuskript).
import-manuskript-source-file = Zvolte soubor
import-manuskript-source-folder = Vybrat složku
import-manuskript-location = Adresář určení
import-manuskript-name = Název souboru
import-manuskript-name-placeholder = Název projektu
import-manuskript-will-create = Vytvořit
import-manuskript-cancel = Zrušit
import-manuskript-import = Importovat
import-manuskript-source-required = Vyberte si Manuskript projekt
import-manuskript-source-missing = Tento soubor nebo složka neexistuje
import-manuskript-location-required = Vyberte cílovou složku
import-manuskript-location-missing = Tato složka neexistuje
import-manuskript-location-not-folder = Tato cesta není složka
import-manuskript-location-readonly = Tato složka není zapisovatelná
import-manuskript-name-required = Zadejte název souboru
import-manuskript-name-exists = Soubor s tímto názvem již zde existuje. Import potvrdí přepsání
import-manuskript-overwrite-title = Nahradit existující soubor?
import-manuskript-overwrite-text = Již existuje { $name }. Nahradit ho importovaným projektem?
import-manuskript-manuscript-binder = Rukopis
import-manuskript-story-bible-binder = Encyklopedie příběhu
import-manuskript-characters-group = Postavy
import-manuskript-world-group = Svět
import-manuskript-plots-group = Šňůry
import-manuskript-project-info-note = Informace o projektu
import-manuskript-summary-note = Shrnutí
import-manuskript-importance-minor = Menší
import-manuskript-importance-secondary = Sekundární
import-manuskript-importance-main = Hlavní
import-manuskript-progress-title = Dovoz Manuskript projektu
import-manuskript-cancel-import = Zrušit
import-manuskript-cancelled = Import zrušen
import-manuskript-done = { $imported ->
    [one] { $imported } dovezená položka.
   *[other] { $imported } dovezených položek.
 } { $revisions ->
    [0] { "" }
    [one] { $revisions } starší verze přišla s ním.
   *[other] { $revisions } starší verze přišel s nimi.
}
import-manuskript-open-now = Otevři teď.
import-manuskript-warnings = { $count ->
    [one] 1 věc vědět o tomto importu
   *[other] { $count } věcí vědět o tomto dovozu
}
import-manuskript-details = Podrobnosti
import-manuskript-warnings-title = O tomto importu
import-manuskript-error-title = Nelze importovat projekt
import-manuskript-nested-too-deep = Projekt nebyl importován: jeho soubor{ $part }Ibrahim je vnořený více než { $limit } Stupeň hloubky. Manuskript Projekt se k tomu přiblíží a čtení by mohlo spadnout. SkribistoNic nebylo napsáno.
import-manuskript-folders-too-deep = Projekt nebyl importován: jeden z jeho souborů sedí více než { $limit } složky hluboké. Ne Manuskript projekt přijde blízko, a čtení by mohlo havaroval Skribisto. Nic nebylo napsáno.
import-manuskript-declares-entities = Projekt nebyl importován: jeho soubor CARL { $part } prohlašuje XML entity, což žádný Manuskript projekt nedělá. Rozšíření by mohlo trvat více paměti, než počítač má a havárii Skribisto. Nic nebylo napsáno.
import-manuskript-error-details = Podrobnosti

import-target-open-title = Tento projekt je otevřený.
import-target-open-text = "{ $name }Inhibitor je otevřen v Skribisto. Importování přes něj by nahradilo projekt, na kterém pracujete, a vaše další uložení by napsalo tento projekt zpět přes import. Vyberte jiný název souboru, nebo zavřete projekt jako první.
import-warnings-announcement = { $result } { $notice }.

export-title = Exportovat
export-close = Zavřít
export-scope-label = Co?
export-format-label = Formát
export-style-label = Styl
export-path-label = Soubor
export-browse = Procházet
export-show-non-exportable = Zobrazit neexportovatelné
export-choose-empty = Není z čeho vybírat.
export-cancel = Zrušit
export-export = Exportovat
export-save-dialog-title = Exportovat do souboru
export-section-what = Co vyvážet
export-section-style = Styl
export-section-destination = Země určení
export-custom-selection = Vlastní výběr
export-selected-count = Vybrané { $count }
export-chip-chapters-none = Kapitola: žádné
export-chip-chapters-numbered = Kapitola: číslováno
export-chip-chapters-title = Kapitoly: pouze název
export-chip-chapters-both = Kapitola: číslo + název
export-chip-scene-break-glyph = Scénář: { $glyph }
export-chip-scene-break-blank = Protržení scény: prázdný řádek
export-chip-scene-break-none = Scénář: žádný
export-chip-major-break-glyph = Hlavní zlom { $glyph }
export-chip-major-break-blank = Hlavní zlom: prázdný řádek
export-chip-major-break-none = Hlavní zlom: žádný
export-chip-major-break-same = Obě úrovně zlomu jsou stejné.
export-chip-spacing-single = Mezery: jednoduché
export-chip-spacing-onehalf = Mezera: 1 ½
export-chip-spacing-double = Prostor: double
export-chip-notes-included = Zahrnuté poznámky
export-chip-notes-excluded = Vyloučené poznámky
export-format-docx = Slovo
export-format-odt = LibreOffice
export-format-html = HTML
export-format-markdown = Markdown
export-format-djot = Djot
export-format-text = Text
export-format-latex = LaTeX
export-format-epub = EPUB
export-format-pdf = PDF
export-overwrite-title = Nahradit existující soubor?
export-overwrite-text = Už existuje{ $name }.
export-progress-title = Vývoz
export-cancelled = Export zrušen
export-done = Vývoz { $count ->
    [one] { $count } položka
   *[other] { $count } položek
}
export-open-file = Otevřít
export-show-in-folder = Zobrazit ve složce
export-error-title = Nelze exportovat
export-error-details = Podrobnosti

backup-progress-start = Spouštím
backup-progress-retention = Uklízení starých záloh
backup-progress-done = Hotovo
backup-progress-destination = Místo určení { $i } z { $n }
backup-details = Podrobnosti
backup-issues-title = Záložní problémy
backup-failed-title = Zálohování selhalo
backup-complete-prune-warning = Zálohování dokončeno ({ $ok } uloženo, { $skipped } již aktuální).

search = Hledat
search-query-placeholder = Hledat…
search-replace-placeholder = Nahradit pomocí
search-replace-toggle = Zapnout výměnu
search-replace-all = Nahradit vše
search-opt-case = Shoda
search-opt-whole-word = Celé slovo
search-opt-diacritics = Zápas akcentů
search-scope-body = Tělo
search-scope-title = Název
search-scope-synopsis = Shrnutí
search-scope-label = Popisek
search-facet-book = Knihy
search-facet-part = Části a součásti
search-facet-chapter = Kapitola
search-facet-scene = Scény
search-facet-note = Poznámky
search-facet-folder = Složky
search-tip-case = Zápas případ: zachází s horní a dolní pouzdro jako odlišné, takže Elena Carella a Carehelena jsou oddělené zápasy.
search-tip-whole-word = Celé slovo: zápas pouze kompletní slova, takže Cate Cate není nalezen uvnitř Category Category.
search-tip-diacritics = Zápas akcenty: zachází s přibývajícími písmeny jako odlišné, takže café café neodpovídá café café café café.
search-tip-body = Tělo: prohledej prózu scén a poznámek.
search-tip-title = Název: vyhledejte názvy pořadových položek.
search-tip-synopsis = Synopsis: vyhledejte každou položku zápisu shrnutí.
search-tip-label = Etiketa: vyhledejte krátkou poznámku uvedenou pod titulem pořadače.
search-tip-comment = Poznámky: vyhledejte text komentářů závity a jejich odpovědi. Komentář je o rukopisu spíše než jeho část, takže má svůj vlastní vypínač, a nahradit listy komentář odpovídá unticked, dokud je zaškrtnete.
search-tip-book = Knihy: knižní kontejner a jeho značky start / end.
search-tip-part = Části a součásti: části a součásti děličů.
search-tip-chapter = Kapitola: kapitoly, podle toho, jak je projekt ukládá.
search-tip-scene = Scény: položky, které drží váš próza.
search-tip-note = Poznámky: poznámky ve volném tvaru.
search-tip-folder = Složky: jednoduché uspořádání složek a oddělovačů.
search-tip-paratext = Text, který patří do knihy, ale ne do její příběh: předmluva, oddanost, další slovo. Nikdy nepočítal v rukopisu.
search-tip-preserve-case = Rezervní pouzdro: náhradník vezme ten případ, který našel, takže se z něj stane Marta a z Eleny se stane Marta.
search-tip-replace = Nahradit: zobrazit náhradní pole a nahradit vše.
search-error = Vyhledávání selhalo: { $message }
search-no-matches = Žádné zápalky.
search-count =
    { $matches ->
        [one] { $matches } zápas
       *[other] { $matches } zápasů
    } in { $items ->
        [one] { $items } dokument
       *[other] { $items } dokumentů
    }
search-count-truncated =
    { $matches ->
        [one] { $matches } zápas
       *[other] { $matches } zápasů
    } in { $items ->
        [one] { $items } dokument
       *[other] { $items } dokumentů
    } (pouze první výsledky)
search-occurrences = { $count }
search-field-body = Tělo
search-field-title = Název
search-field-synopsis = Shrnutí
search-field-label = Popisek
search-field-epigraph = Motto
search-field-comment = Komentář
search-field-comment-reply = Odpověď
search-field-footnote = Poznámka pod čarou
search-include-in-replace = Zahrnout do nahradit vše
search-collapse-all = Kolaps všechny výsledky
search-replace-here = Nahradit
search-dismiss = Rozchod z výsledků
search-undo-dismiss = Přineste zpět poslední zamítnutý výsledek.
search-replace-nothing = (nic)
search-replace-confirm-title = Nahradit všechny zápalky?
search-replace-confirm-text =
    Nahradit { $occurrences ->
        [one] { $occurrences } výskyt
       *[other] { $occurrences } události
    } of "{ $query }" with "{ $replacement }" across { $items ->
        [one] { $items } dokument
       *[other] { $items } dokumentů
    } Můžete to později zrušit z oznámení.
search-replace-done-title = Nahradit kompletní
search-replace-done =
    { $occurrences ->
        [one] { $occurrences } výskyt
       *[other] { $occurrences } události
    } replaced in { $items ->
        [one] { $items } dokument
       *[other] { $items } dokumentů
    } .
search-replace-done-skipped =
    { $occurrences ->
        [one] { $occurrences } výskyt
       *[other] { $occurrences } události
    } replaced in { $items ->
        [one] { $items } dokument
       *[other] { $items } dokumentů
    }. { $skipped ->
        [one] { $skipped } pole
       *[other] { $skipped } polí
    } přeskočen (změněn od hledání).
search-replace-undo = Zpět
search-replace-failed-title = Nahrazení selhalo
search-replace-skipped-title = Nic nahrazeno
search-replace-skipped-body = Počet textů změněných od vyhledávání: { $fields }. Tyto texty byly ponechány beze změny. Vyhledejte znovu, abyste zjistili aktuální umístění slov.
search-replace-undo-failed-title = Zrušit selhalo
search-preview = Náhled
search-preview-empty = Zvolte výsledek pro náhled zde
search-preview-no-prose = Tento zápas nemá editovatelný text
search-preview-prompt = Chcete-li vidět náhled zde, spusťte vyhledávání.
search-preview-open-search = Vyhledávání v projektu
search-preview-footnote-prompt = Tento zápas je v poznámce pod čarou vlastní text. Otevřete jej v doku Footnotes vidět a upravit.
search-preview-open-footnotes = Otevřené poznámky pod čarou

find-placeholder = Najít v tomto dokumentu
find-count = { $current } z { $total }
find-no-results = Žádné výsledky
find-close = Zavřít vyhledávání
find-previous = Předchozí zápas (Shift + Enter)
find-next = Další zápas (Enter)
find-opt-case = Shoda
find-opt-whole-word = Celé slovo
find-replace-toggle = Nahradit přepínač (Ctrl + R)
find-replace-placeholder = Nahradit pomocí
find-replace = Nahradit
find-replace-all = Nahradit vše
find-preserve-case = Rezervní pouzdro

dict-download-title = Stahování { $name }
dict-download-done = Nainstalováno { $name }
dict-download-failed = Mohl byste stáhnout { $name }: { $error }
dict-removed = Odstraněno { $name }
dict-accept-first = Přijetí licence před stažením { $name }
settings-dict-tab-installed = Instalováno
settings-dict-tab-get-more = Získat více
settings-dict-tab-personal = Osobní slova
dict-installed-empty = Na tomto počítači se zatím nenašly žádné slovníky.
dict-get-more-search = Jazyky pro vyhledávání
dict-system-badge = na vašem systému
dict-unusable-badge = nepoužitelné
dict-download-button = Stáhnout
dict-downloading = Stahuje se
dict-installed-label = Instalováno
dict-remove = Odebrat
dict-view-license = View licence
dict-approx-size = ~{ $size }
dict-personal-empty = V tomto projektu zatím žádná osobní slova.
dict-personal-add = Přidat
dict-personal-placeholder = Přidat slovo
dict-missing-toast = { $count ->
    [one] Tento projekt používá { $count } slovník, který jste nainstalovali
   *[other] Tento projekt využívá { $count } slovníků, které jste nainstalovali
}
dict-missing-action = Získat slovníky
dict-license-title = { $name } licence
dict-license-accept = Přijmout stahování
dict-license-cancel = Zrušit
dict-license-close = Zavřít
dict-add-button = Přidat slovník
dict-add-title = Přidat slovník
dict-add-close = Zavřít
dict-add-name = Název
dict-add-name-placeholder = např. Můj latinský slovník
dict-add-code = Kód jazyka
dict-add-code-placeholder = např. la nebo fr-FR-x-custom
dict-add-code-hint = Krátký štítek vašeho výběru. To je to, co si vyberete jako dokument jazyk.
dict-add-aff = Affix soubor (.aff)
dict-add-dic = Seznam slov (.dic)
dict-add-submit = Přidat
dict-add-cancel = Zrušit
dict-add-name-required = Dej slovníku jméno.
dict-add-code-required = Zadejte kód jazyka.
dict-add-code-invalid = Používejte pouze písmena, číslice a - _.
dict-add-code-reserved = To je budova-ve slovníku kód. Vyberte jiný.
dict-add-file-required = Vyberte si soubor.
dict-add-file-missing = Ten soubor neexistuje.
dict-add-code-taken = Slovník pro tento kód je již nainstalován.
dict-add-done = Přidáno { $name }
dict-add-unusable = Tyto soubory nejsou použitelným slovníkem: { $error }
dict-add-failed = Mohl bych přidat slovník: { $error }
inspector-export = Exportovat
inspector-exportable = Zahrnout do vývozu
ctx-number = Číslo této kapitoly (&N)
ctx-unnumber = Nečíslovat tuto kapitolu (&N)
inspector-numbering = Číslo
inspector-numbered = Seřazeno
inspector-numbered-tip = Tato kapitola zaujímá své místo v knihách číslování. Vypněte ji na prolog, epilog nebo interlude.
inspector-numbered-tip-more = Nečíslovaná kapitola zůstává v knize přesně tak, jak to bylo: její nadpis, jeho próza a jeho počet slov jsou nedotčené. To prostě tiskne žádné číslo, a nepoužívá jednu nahoru: kapitola po prolog je kapitola jedna, ne kapitola druhá. Vynechání ji z vývozu místo je jiný přepínač, výše, a že jeden odstraní kapitolu z knihy úplně.
inspector-apply-to-children = Aplikovat na děti
inspector-milestone = Datum milníku
inspector-milestone-none = Žádné datum
inspector-milestone-clear = Vymazat
inspector-tags = Štítky
inspector-aliases = Také známý jako
inspector-dict-language = Jazyk
inspector-apply-language-to-children = Aplikujte jazyk na děti
settings-page-language = Jazyk
settings-field-dict-language = Jazyky
dict-tradeoff-hint = Každý další jazyk přijímá více slov, takže méně chyb jsou zachyceny.
lang-inherit-hint = Zděděná: tato scéna používá knižní nebo projektové jazyky.
lang-pill-list = Jazyky
lang-pill-add = Přidat jazyk
lang-pill-remove = Odstranit { $name }
lang-pill-mute = Vypnout hláskování pro { $name }
lang-pill-unmute = Zapněte kontrolu pravopisu na { $name }

settings-page-personal-dictionary = Osobní slovník
settings-user-dict-desc = Slova, která zde přidáte, se považují za správně napsaná všude v aplikaci a už nikdy neoznačená.
settings-user-dict-add-placeholder = Přidat slovo
settings-user-dict-add = Přidat slovo
settings-user-dict-search = Filtrační slova
settings-user-dict-count = { $count ->
    [one] { $count } slovo
    [few] { $count } slova
    [many] { $count } slova
   *[other] { $count } slov
}
settings-user-dict-rename = Přejmenovat
settings-user-dict-remove = Odebrat
settings-user-dict-empty = Zatím žádná slova. Přidat slova spell- checker by měl ignorovat, nebo importovat seznam.
settings-user-dict-import = Importovat…
settings-user-dict-export = Exportovat…
settings-user-dict-import-tip = Přidat slova z prostého textového souboru, jedno slovo na řádek. Stávající slova jsou držena.
settings-user-dict-export-tip = Uložit celý seznam do prostého textového souboru, jedno slovo na řádek.
settings-user-dict-txt-filter = Textové soubory
settings-user-dict-duplicate = Už ve slovníku.
settings-user-dict-imported = Dovezené { $count ->
    [one] { $count } slovo
   *[other] { $count } slov
} ({ $duplicates } již přítomen).
settings-user-dict-import-failed = Nelze přečíst seznam slov: { $error }
settings-user-dict-exported = Uložené { $count ->
    [one] { $count } slovo.
   *[other] { $count } slov.
}
settings-user-dict-export-failed = Nelze uložit seznam slov: { $error }

settings-page-text-replacements = Náhrady textů
settings-text-repl-desc = Nahradit zkratku s plným textem, jak jste na typu: Ibrabtw se stane tím, jak jste zadejte mezeru nebo interpunkci.
settings-text-repl-enable = Použít textové náhrady v tomto projektu
settings-text-repl-disabled-hint = Zapněte to a definujte zkratku, která se zvětšuje, když píšete.
settings-text-repl-add = Přidat pravidlo
settings-text-repl-trigger-placeholder = Zkratka
settings-text-repl-replacement-placeholder = Co se z toho stane
settings-text-repl-added = Přidáno { $trigger }
settings-text-repl-duplicate = CLAS{ $trigger } již má pravidlo
settings-text-repl-filter = Filtrační pravidla
settings-text-repl-count = { $n ->
    [one] 1 pravidlo
   *[other] { $n } pravidla
}
settings-text-repl-row-enabled = Použijte toto pravidlo.
settings-text-repl-delete = Smazat pravidlo pro { $trigger }
settings-text-repl-deleted = Zrušeno pravidlo pro CLAS{ $trigger }
settings-text-repl-empty = Zatím žádná pravidla.
settings-text-repl-csv-filter = CSV souborů
settings-text-repl-import = Importovat…
settings-text-repl-export = Exportovat…
settings-text-repl-imported = Dovezené { $added }, přeskočené { $skipped }
settings-text-repl-exported = Vývoz { $n ->
    [one] 1 pravidlo
   *[other] { $n } pravidla
}

editor-menu-no-suggestions = Žádné návrhy.
editor-menu-add-to-dictionary = Přidat do slovníku PDF { $word }
editor-menu-add-words-to-dictionary = Přidat vybraná slova do slovníku
editor-dict-added = Přidáno { $word } do slovníku.
editor-dict-added-multi = { $count ->
    [one] Přidáno { $count } slovo do slovníku.
   *[other] Přidáno { $count } slov do slovníku.
}
toast-undo = Zpět

undo-superseded-title = Ten krok už není poslední.
undo-superseded-body = Něco jiného se změnilo v tomto projektu od té doby. Undo by vzít, že zpět místo, tak to má se. Nic tento krok udělal byl obrácen. Použijte Edith Undo, aby krok zpět do historie sám.
undo-failed = Zpět selhala: { $error }

menu-edit = Úpravy (&E)
menu-edit-undo = Zpět (&U)
menu-edit-redo = Znovu (&R)
menu-edit-undo-target = Undo { $target } (&U)
menu-edit-redo-target = Redo { $target } (&R)
shortcut-name-edit-undo = Zpět
shortcut-name-edit-redo = Znovu
undo-target-typing = psaní
undo-target-project = poslední změna tohoto projektu
prose-history-reset-title = Resetovat historii písma
prose-history-reset-body = { $count ->
    [one] Jedna otevřená scéna byla obnovena z dřívějšího stavu, takže její psaní historie již neplatí.
   *[other] { $count } otevřené scény byly obnoveny z dřívějšího stavu, takže jejich psaní historie již neplatí.
}
undo-target-trash = Stěhování do koše
undo-target-restore = obnova z koše
undo-target-delete-forever = vymazání nadobro
undo-target-replace-all = nahrazení celého projektu
undo-target-import = dovoz dokumentu
undo-target-duplicate = duplikace
undo-target-move = pohyb
undo-target-merge = sloučení dvou scén
undo-target-split = Rozdělení scény
undo-target-promote = změna typu
undo-target-tidy-titles = odklízení titulků kapitoly
undo-target-import-tags = dovážet etikety
undo-target-import-templates = Šablony dovážejících bankovek
undo-target-create = tvorba
undo-target-remove = smazání
undo-target-rename = přejmenování
undo-target-edit = které editovat
menu-edit-find = Najít… (&F)
menu-edit-find-next = Najít další (&N)
menu-edit-find-prev = Najít předchozí (&V)
menu-edit-replace = Najít a nahradit (&L)
menu-edit-find-in-project = Najít v projektu (&J)
menu-edit-replace-in-project = Nahradit projekt (&I)
undo-frozen = Zpět (vždy dopředu)
redo-frozen = Redo (vždy dopředu)
menu-edit-cut = Vyjmout (&T)
menu-edit-copy = Kopírovat (&C)
menu-edit-paste = Vložit (&P)
menu-edit-paste-plain = Vložit bez formátování (&W)
menu-edit-select-all = Vybrat vše (&A)
shortcut-name-edit-cut = Vyjmout
shortcut-name-edit-copy = Kopírovat
shortcut-name-edit-paste = Vložit
shortcut-name-edit-paste-plain = Vložit bez formátování
shortcut-name-edit-select-all = Vybrat vše

titlebar-spellcheck-on = Spell- checking je zapnutý. Klikněte pro zastavení kontroly (F7)
titlebar-spellcheck-off = Spell- checking je vypnuto. Klikněte pro kontrolu znovu (F7)
menu-spellcheck = Kontrola pravopisu (&C)
menu-comments = Komentáře (&M)
settings-page-spellcheck = Kontrola pravopisu
settings-group-spellcheck = Kontrola pravopisu
settings-spellcheck-enabled = Zkontrolovat pravopis při psaní
settings-spellcheck-hint = Podtrhuje slova, která žádný nainstalovaný slovník neví. Vypnutí tohoto vypnutí zastaví všechny kontroly, v každém projektu, dokud jej nezapnete. Chcete-li přestat kontrolovat pouze jeden jazyk, očistěte jeho kontrolní značku na jazykovém poli projektu nebo prvku.

menu-trash = Koš (&T)
trash-title = Koš
trash-empty-state = Koš je prázdný.
trash-empty-button = Prázdný odpad
trash-restore = Obnovit (&R)
trash-restore-to = Obnovit do provozu (&T)
trash-delete-forever = Smazat navždy (&D)
trash-restored-ok = { $count } obnoveno.
trash-restore-error = Nelze obnovit: { $error }
trash-restore-orphaned = Tato položka je původní místo je pryč. Vyberte si, kde ji obnovit.
trash-restore-no-project = Žádný projekt není otevřený, takže není co obnovit.
trash-delete-no-project = Žádný projekt není otevřený, takže není co smazat.
trash-empty-confirm-title = Vyprázdnit odpadky?
trash-empty-confirm-text = { $count ->
    [one] { $count } zničený vstup bude trvale vymazán.
   *[other] { $count } zdemolované položky budou trvale vymazány.
} Odcházejí z odpadků nadobro a Undo je může vrátit, dokud neuděláš něco jiného.
trash-emptied-title = Vyprázdněný odpad
trash-emptied-body = Všechno v koši bylo trvale vymazáno.
trash-empty-no-project = Žádný projekt není otevřený, takže tu není žádný odpad k vyprázdnění.
trash-delete-forever-confirm-title = Vymazat navždy?
trash-delete-forever-confirm-text = { $count ->
    [one] Položka { $count } bude trvale vymazána.
   *[other] { $count } položky budou trvale vymazány.
} Odcházejí z odpadků nadobro a Undo je může vrátit, dokud neuděláš něco jiného.
trash-deleted-title = Vymazáno navždy
trash-deleted-body = { $count } trvale vymazán.
trash-undo = Zpět
trash-restore-picker-title = Obnovit do provozu
trash-restore-to-confirm-title = Obnovit tady?
trash-restore-to-confirm-text = Obnovit { $item } do { $destination }?
trash-restore-picker-restore-here = Obnovit zde
trash-restore-picker-cancel = Zrušit
trash-restore-picker-empty = Zatím žádné pořadače.
trash-banner-title = Tato položka je v koši
trash-banner-description = Bude se objevovat v obrysu nebo exportu, dokud ho neobnovíte.
trash-banner-restore = Obnovit…
trash-tab-tooltip = V koši

settings-styles-sheet-yes = Ano
settings-styles-sheet-no = Ne
settings-styles-sheet-font = Písmo
settings-styles-sheet-indent = Odrážka první linie
settings-styles-sheet-para-spacing = Odstupňování odstavců
settings-styles-sheet-page = Velikost stránky
settings-styles-sheet-margins = Mramor (T / R / B / L)
settings-styles-sheet-title-page = Rezervujte titulní stranu
settings-styles-sheet-heading-language = Jazyk názvu
settings-styles-sheet-auto = Sleduje text
settings-styles-sheet-digits = Čísla
settings-styles-sheet-direction = Směr textu
settings-styles-sheet-formats = Formáty
settings-styles-sheet-all-formats = Všechny

settings-styles-editor-group = Editor stylu
settings-styles-editor-missing = Ten styl už není k dispozici.

settings-styles-page-letter = Dopis
settings-styles-digits-western = západní oblast (0 / 9)
settings-styles-digits-eastern-arabic = Východoarabská (ISCED)
settings-styles-direction-ltr = Zleva doprava.
settings-styles-direction-rtl = Doprava doleva

format-dock-title = Formát
format-panel-empty = Umístěte kurzor do scény, poznámky, nebo synopse vidět formátování možností.
format-group-marks = Text
format-group-block = Odstavec
format-group-lists = Seznamy
format-group-tables = Tabulka
format-group-breaks = Přetržení scény
format-superscript = Horní index
format-subscript = Dolní index
format-link = Odkaz
link-dialog-insert-title = Vložit odkaz
link-dialog-edit-title = Upravit odkaz
link-dialog-text-label = Text
link-dialog-text-placeholder = slova, která čtenář vidí
link-dialog-url-label = Odkazy na
link-dialog-url-placeholder = example.com
link-dialog-insert = Vložit
link-dialog-apply = Použít
link-dialog-cancel = Zrušit
link-dialog-remove = Odstranit odkaz
link-scheme-refused = Pouze webové a e-mailové odkazy mohou být otevřeny. { $url } bylo ponecháno na pokoji.
menu-format-link = Odkaz (&N)
format-clear = Smazat formátování
format-heading = Hladina
format-heading-normal = Normální text
format-heading-1 = Hlava 1
format-heading-2 = Hlava 2
format-heading-3 = Položka 3
format-heading-4 = Hlava 4
format-heading-5 = KAPITOLA 5
format-heading-6 = Položka 6
format-align-left = Zarovnat doleva
format-align-center = Střed
format-direction-rtl = Pravý až levý odstavec
format-blockquote = Bloková citace
format-list-bullet = Vybroušený seznam
format-list-numbered = Seřazený seznam
format-indent = Odhalení
format-outdent = Odhalení
format-table-insert = Vložit tabulku
format-table-row-above = Vložit řádek výše
format-table-row-below = Vložit řádek níže
format-table-col-before = Vložit sloupec před
format-table-col-after = Vložit sloupec za
format-table-row-delete = Smazat řádek
format-table-col-delete = Smazat sloupec
format-table-remove = Odstranit tabulku

menu-format-marks-bold = Tučné (&D)
menu-format-marks-italic = Italština (&I)
menu-format-marks-underline = Podtrženo (&U)
menu-format-marks-strike = Prorazit. (&S)
menu-format-marks-superscript = Horní index (&P)
menu-format-marks-subscript = Dolní index (&B)
menu-format-marks-clear = Smazat formátování (&C)
menu-format-heading = Název (&H)
menu-format-heading-normal = Normální text (&N)
menu-format-heading-1 = Hlava 1 (&1)
menu-format-heading-2 = Hlava 2 (&2)
menu-format-heading-3 = Položka 3 (&3)
menu-format-heading-4 = Hlava 4 (&4)
menu-format-heading-5 = KAPITOLA 5 (&5)
menu-format-heading-6 = Položka 6 (&6)
menu-format-alignment = Zarovnání (&A)
menu-format-align-left = Zarovnat doleva (&L)
menu-format-align-center = Střed (&C)
menu-format-direction = Směr (&E)
menu-format-direction-auto = Automatické (&A)
menu-format-direction-ltr = Vlevo vpravo (&L)
menu-format-direction-rtl = Doprava doleva (&R)
menu-format-blockquote = Bloková citace (&Q)
menu-format-lists = Seznamy (&L)
menu-format-list-bullet = Vybroušený seznam (&B)
menu-format-list-numbered = Nastavený seznam (&N)
menu-format-indent = Odhalení (&I)
menu-format-outdent = Odhalení (&O)
menu-format-table = Tabulka (&T)
menu-format-table-insert = Vložit tabulku (&I)
menu-format-table-2x2 = 2 x 2 (&2)
menu-format-table-3x3 = 3 x 3 (&3)
menu-format-table-4x4 = 4 x 4 (&4)
menu-format-table-row-above = Vložit řádek nad (&V)
menu-format-table-row-below = Vložit řádek níže (&W)
menu-format-table-col-before = Vložit sloupec před (&L)
menu-format-table-col-after = Vložit sloupec za (&T)
menu-format-table-row-delete = Smazat řádek (&D)
menu-format-table-col-delete = Smazat sloupec (&C)
menu-format-table-remove = Odstranit tabulku (&R)

menu-format-typo-marks = Formátování značky (&F)
menu-format-typo-nbsp = Ne-break prostor (&N)
menu-format-typo-nbhyphen = Ne-break Hyphen (&H)
menu-format-typo-soft-hyphen = Soft Hyphen (&S)
menu-format-typo-nnbsp = Úzký no- break prostor (&R)
menu-format-typo-zwsp = Zero- wide Space (&Z)
menu-format-typo-word-joiner = Word Joiner (&W)


menu-about = Asi Skribisto (&A)
about-title = Okolo Skribisto
about-version = Verze { $version }
about-update-available = Verze { $version } je k dispozici
about-update-see-notes = Zjisti, co se změnilo.
about-update-flathub = Flathub udržuje tuto kopii aktualizovanou.
about-update-distro = Vaše distribuce udržuje tuto kopii aktualizovanou.
about-tagline = Novel- writing app for long-form fiction, napsaný v rustu pomocí Teksilo toolkit.
about-license = Vydáno pod GNU General Public License, verze3.
about-copyright = © 2026 Cyril Jacquet
about-close = Zavřít

native-menu-about = Okolo { $app }
native-menu-hide = Skrýt { $app }
native-menu-quit = Ukončení { $app }
native-menu-settings = Nastavení…
native-menu-window = Okno
native-menu-minimize = Minimalizovat
native-menu-zoom = Zvětšit

window-title = { $title } - { $app }
window-title-numbered = { $title } - { $app } (Okno { $n })
window-title-empty = { $app }

first-run-window-title = Vítejte.
first-run-title = Založení { $app }
first-run-body = { $app } udržuje nastavení odděleně od Skribisto, takže začíná prázdné. Vaše preference, nedávné projekty, slovníky a rozložení oken lze zkopírovat přes nyní.
first-run-from = Kopírovat od
first-run-to = Kopírovat
first-run-copy-note = Nic není přesunuto nebo odstraněno. Skribisto udržuje každý z jeho vlastní nastavení, a pokračuje v práci přesně jako předtím.
first-run-import = Nastavení importu
first-run-start-fresh = Začít znovu
first-run-import-failed = Některé nastavení nebylo možné importovat: { $error }

settings-page-punctuation = Chyby při přepínání
settings-group-punctuation = Inteligentní interpunkce
settings-page-work-punctuation = Punktuace
settings-punctuation-override = Dejte tomuto projektu vlastní pravidla pro interpunkci
settings-punctuation-override-hint = Off: projekt se řídí preferencí aplikace. Tato pravidla cestují uvnitř .skrib, takže spoluautor otevírající soubor píše se stejnou typografií.
settings-punctuation-dashes = Měnit -- na krátkou pomlčku a --- na dlouhou pomlčku
settings-punctuation-ellipsis = Proměň... v elipsii.
settings-punctuation-quotes = Uvozovky a apoštrofy
settings-quote-style = Značky kotace
settings-quote-style-locale = Výchozí jazyk
settings-quote-style-curly = Dvojnásobek
settings-quote-style-curly-single = Small Singleweather forecast
settings-quote-style-guillemets = Guillemets
settings-quote-style-low-high = Alow- high (vysoká hladina)
settings-punctuation-spacing = Prostor před;:!?
settings-punctuation-spacing-hint = Francouzská typografie stanoví úzký no- break prostor před;!? a plný před:. Platí pouze pro text napsaný ve francouzštině. Prostor uvnitř guillemets comes přichází s guillemet cituje sami.
settings-punctuation-sample = Váš jazyk dává
settings-punctuation-app-hint = Co každý projekt dělá, pokud si nevezme vlastní pravidla v Projektu România Punctuation.
settings-punctuation-dialogue = Otevřít odstavec zapsaný jako "Dialog dash"
settings-punctuation-dialogue-hint = Pro jazyky, které označují řeč pomlčkou spíše než uvozovkami: francouzština, španělština, ruština a další. Střílí pouze na samém začátku odstavce.

statusbar-go-to = Jdi do Atlanticu.
go-to-placeholder = Hledat pořadač
go-to-no-matches = Žádná položka se neshoduje s tím hledáním.
menu-go-to = Jdi do Atlanticu. (&G)

settings-page-distraction-free-themes = Distraction- zdarma témata
settings-themes-builtin = Built- v tématech
settings-themes-builtin-badge = Built- in
settings-themes-user = Moje témata
settings-themes-editor-group = Upravit téma
settings-themes-editor-empty = Vyberte si téma pod Témata a upravte ho.
settings-themes-use = Použití
settings-themes-duplicate = Duplikovat
settings-themes-edit = Úpravy
settings-themes-delete = Smazat
settings-themes-export = Exportovat…
settings-themes-import = Importovat…
settings-themes-copy-suffix = Kopírovat
settings-themes-json-filter = Téma (JSON)
settings-themes-imported = Importované téma
settings-themes-import-failed = Nelze importovat toto téma
settings-themes-exported = Vyvezené téma
settings-themes-export-failed = Nelze exportovat toto téma
settings-themes-low-contrast = nízký kontrast
settings-themes-low-contrast-band = zvýraznění skryje text
settings-themes-field-name = Název
settings-themes-field-paper = Strana
settings-themes-field-ink = Text
settings-themes-field-general = Pozadí
settings-themes-field-widget-text = Text ovládacího pásu
settings-themes-field-caret-band = Zvýraznění kolem košíku

statusbar-focus-settings = Rozptýlení - bezplatná nastavení
statusbar-focus-manage-themes = Spravovat témata

comments-title = Komentáře
comments-document-title = Tento dokument
comments-empty-project = V tomto projektu zatím nebyly žádné připomínky.
comments-empty-document = Žádné připomínky k tomuto dokumentu.
comments-filter-all = Všechny
comments-filter-open = Otevřít
comments-filter-resolved = Vyřešeno
comments-filter-orphaned = Osiřelé
comments-status-open = Otevřít
comments-status-resolved = Vyřešeno
comments-status-orphaned = Ztratil svůj text
comments-orphan-snippet = (Komentovaný text je pryč)
comments-status-unplaced = Žádná textová pozice
comments-unplaced-snippet = (není ukotveno na žádný text)
comments-reply-count = { $count ->
    [one] { $count } odpověď
   *[other] { $count } replies
}
comments-menu-resolve = Řešení
comments-menu-reopen = Znovu otevřít
comments-menu-delete = Smazat
comments-sort-document = V pořadí dokumentů
comments-sort-newest = Nejnovější
comments-sort-reader = Seskupit podle čtenáře
comments-unsigned-author = Nesignováno
comments-agreement = { $count ->
    [one] { $count } čtečka
   *[other] { $count } čtenářů
}
comments-menu-add = Přidat komentář
comments-menu-add-paragraph = Komentář k tomuto odstavci
overview-col-comments = Komentáře
overview-col-total-comments = Celkové připomínky
comments-card-placeholder = Napište komentář
comments-card-unknown-author = Neznámý autor
comments-unsigned-toast = Vaše komentáře nejsou podepsané: na tomto počítači pro vás není nastaveno žádné jméno.
comments-unsigned-action = Nastavit své jméno
comments-card-reply = Odpověď
comments-card-reply-placeholder = Odpověď
comments-card-actions = Akce v oblasti komentování
comments-reply-actions = Odpovědné akce
comments-menu-delete-reply = Smazat odpověď
comments-menu-delete-all = Smazat všechny komentáře zde
comments-deleted-toast = Komentář smazán
comments-reply-deleted-toast = Zrušená odpověď
comments-deleted-all-toast = { $count ->
    [one] { $count } komentář smazán
   *[other] { $count } připomínky smazány
}
comments-undo = Zpět

analysis-segment = Analýza
analysis-scope-book = Analyzuji tuto knihu.
analysis-run = Spustit analýzu
analysis-stale = Změnil se od té doby, co běžel.
analysis-not-run = Ještě jsem to nezanalyzoval.
analysis-running = Čtení rukopisu
analysis-failed = Analýza nemohla skončit.
analysis-no-scenes = V téhle knize ještě žádné scény nejsou.

analysis-shape = Tvar

analysis-arrivals = Příjezdy
analysis-arrivals-explainer = Jak text dosáhl tohoto projektu v době, kdy byl otevřen. Píše se tu, která trasa postavy přišla dolů, a vůbec nic o tom, kdo je napsal: spisovatel, který drafts jinde a pasty se usadil, a ten, kdo diktuje, diktoval.
analysis-arrivals-scope = Celý tenhle projekt, ne jen tahle kniha.
analysis-arrivals-session = Od té doby, co jsi ho otevřel, se znovu začíná počítat.
analysis-arrivals-nothing = Na tomto zasedání ještě nedorazil žádný text.
analysis-arrivals-typed = Psaní
analysis-arrivals-pasted = Pečené
analysis-arrivals-dictated = Diktované
analysis-arrivals-imported = Dovezené
analysis-arrivals-programmatic = Vloženo pro tebe
analysis-arrivals-count = { $count ->
    [one] { $count } znak
    [few] { $count } znaky
    [many] { $count } znaku
   *[other] { $count } znaků
}
analysis-arrivals-none = Žádné

analysis-words-per-scene = Slova na scénu
analysis-median-words = Tato kniha je medián scény běží { $count ->
    [one] { $count } slovo.
   *[other] { $count } slov.
}
analysis-median-line = Medián: { $count ->
    [one] { $count } slovo
   *[other] { $count } slov
}
analysis-dialogue = Dialog
analysis-dialogue-unsupported = Dialog se pro tento jazyk ještě neměří.

analysis-footnote-words = Poznámka pod čarou
analysis-footnote-words-count = { $count ->
    [one] { $count } slovo je v této knize poznámky pod čarou, držené mimo rukopis celkem.
   *[other] { $count } slova jsou v této knize jsou poznámky pod čarou, uchovávány mimo rukopis celkem.
}
analysis-footnote-words-pending = Počítání poznámek pod čarou

analysis-ignore-empty = Ignorovat texty bez ničeho napsaného.
analysis-empty-hidden = { $count } prázdné { $count ->
        [one] Text
       *[other] texty
    } skryté.
analysis-all-texts-empty = Každý text v této knize je stále prázdný.

binder-filter-count = { $shown } z { $total } zobrazeno
binder-filter-clear = Vymazat
binder-filter-none = Nic neodpovídá{ $query }.

settings-paratext-intro = Na přední i zadní straně může začít nový projekt. Každá struktura patří k publikační tradici a její tituly jsou napsány v této tradici vlastní jazyk. Přejmenovat je volně, jakmile je projekt vytvořen.
settings-paratext-structures = Struktury
settings-paratext-broken = Nelze číst
settings-paratext-edit = Úpravy
settings-paratext-duplicate = Duplikovat
settings-paratext-delete = Smazat
settings-paratext-new = Nová struktura
settings-paratext-save = Uložit
settings-paratext-editor-hint = Jméno, stránky, které vytvoříte před rukopisem, a stránky, které vytvoříte po něm. Tam, kde skončí, je vaše: jsou běžné položky, jakmile projekt existuje.


image-insert = Vložit obrázek (&M)
image-no-project = Před vložením obrázku otevřete projekt.
image-choose-title = Vyberte si obrázek
image-filter-label = Obrázky
image-large-title = Tohle je velký obrázek.
image-large-text =
    { $name } je { $megapixels } megapixelů ({ $width } × { $height }).
    Udržení původního souboru v projektu: cestuje
    s každou zálohou a každý export. Optimalizace ukládá menší kopii
    místo toho až 2560 pixelů na své nejdelší straně.
image-large-keep = Uchovat originální
image-large-downscale = Optimalizovat
image-large-remember = Od teď se neptej znovu.
image-not-recorded = Obrázek byl uložen, ale nemohl být zaznamenán v projektu.
image-describe-title = Popište obrázek
image-describe-explain = Co obrázek ukazuje, pro čtenáře, který to nevidí. To není součástí rukopisu: to se nikdy nepočítá, hledá nebo exportuje jako próza.
image-describe-placeholder = maják proti šedé obloze
image-resize-title = Změnit velikost obrázku
image-resize-explain = Procento velikosti je nyní zobrazeno. 100 nechává tak, jak je.
image-resize-invalid = Zadejte číslo mezi1 a1000.
image-menu-describe = Popište obrázek (&D)
image-menu-resize = Změnit velikost obrázku (&R)
image-menu-reset-size = Původní velikost (&Z)

cover-choose = Knižní obálka (&V)
cover-clear = Odstranit kryt (&R)
cover-choose-title = Vyberte obálku
cover-set = Krytí je nastaveno.
cover-cleared = Kryt byl odstraněn, obraz je stále v projektu.
card-label = Popisek
card-exportable = Vývoz
card-numbered = Seřazeno
card-created = Vytvořeno
card-modified = Upraveno
card-yes = Ano
card-no = Ne
card-type = Typ
card-position = Poloha
card-children = Děti
card-goal = Cíl
card-words = Slova
card-synopsis = Shrnutí
card-point-of-view = Úhel pohledu
card-aliases = Také známý jako

export-orphan-footnotes-title = Poznámka pod čarou bez odkazu
export-comments-dropped =
    { $count ->
        [one] Jeden komentář nemohl být uveden do vyváženého textu a byl vynechán.
       *[other] Ve vyváženém textu nebylo možné uvést { $count } připomínek a byly vynechány.
    }
export-orphan-footnotes =
    { $count ->
        [one] Jedna poznámka pod čarou se již nikde v rukopisu nezmiňuje. Jeho text se ve vyvážené knize neobjeví.
       *[other] { $count } poznámky pod čarou se již nikde v rukopisu nezmiňují. Jejich text se ve vyvážené knize neobjeví.
    }


menu-footnotes = Poznámky (&N)
footnotes-title = Poznámky
footnotes-insert = Vložit poznámku pod čarou (&F)
footnotes-empty = Zatím žádné poznámky pod čarou. Vložte kurzor do scény, pak použijte + výše, nebo Ctrl + Alt + F.
footnotes-filter-all = Všechny
footnotes-filter-document = Tento dokument
footnotes-filter-orphaned = Osiřelé
footnotes-orphaned = Nic už neukazuje na tento vzkaz.
footnotes-untitled-home = Bez názvu
footnotes-body-placeholder = samotná poznámka
footnotes-insert-tooltip = Vložte poznámku pod čarou na kurzor (Ctrl + Alt + F)
footnotes-actions = Žaloby týkající se poznámky pod čarou
footnotes-delete = Smazat poznámku a její odkaz (&D)
footnotes-no-project = Otevřít projekt před vložením poznámky pod čarou.
footnotes-no-caret = Vložte kurzor do scény a vložte tam poznámku pod čarou.
footnotes-not-created = Poznámka pod čarou nemohla být do projektu doplněna.
footnotes-deleted-toast = Poznámka zrušena
footnotes-undo-delete = Zpět

import-document-title = Dovozní doklady
import-document-close = Zavřít
import-document-step-files = Soubory
import-document-step-review = Přezkum
import-document-step-destination = Země určení
import-document-step-reconcile = Sloučení
import-document-reconcile-hint = Některé z těchto položek jsou ty, které již máte. Řekni, co by se mělo stát s každým.
import-document-reconcile-all-new = Nic v tomto souboru se neshoduje s vaším projektem: každá položka bude přidána jako nová.
import-document-duplicate-returns = Tyto soubory jsou více než jedna kopie stejného rukopisu vrací. Importovat je jeden po druhém: odpovídající pouze řádky až jedna vrátí kopii proti vašemu projektu, takže import několik dohromady by přidat druhou kopii knihy namísto sloučení.
import-document-hunk-added = Vezměte si tento nový odstavec.
import-document-hunk-removed = Zahoď ten odstavec.
import-document-hunk-changed = Vezmi si to přepsat.
import-document-col-stray-prose = Jeho text
import-document-stray-as-paratext = Ponechat jako paratext
import-document-stray-discard = Zahoď ten text.
import-document-col-current = Ve vašem projektu
import-document-col-incoming = V tomto souboru
import-document-col-status = Stav
import-document-col-action = Co dělat
import-document-status-identical = Stejné.
import-document-status-editor-edited = Vystřihli to.
import-document-status-you-edited = Upravil jsi to.
import-document-status-conflict = Oba ho upravili.
import-document-status-different = Obtíže
import-document-status-new = Nový
import-document-status-missing = Ne v tomto souboru.
import-document-status-moved = Přesunuto
import-document-action-comments-only = Pouze připomínky
import-document-action-take-import = Vezměte si tuto verzi.
import-document-action-keep-current = Nech si můj.
import-document-action-create-new = Přidat jako nový
import-document-action-ignore = Přeskočit
import-document-compare = Porovnat
import-document-compare-legend = Vaše verze ve srovnání s tímto souborem. Čtěte pouze.
import-document-compare-close = Zavřít
import-document-drop-title = Přiložte dokumenty sem.
import-document-drop-hint = Markdown (.md), Word (.docx), OpenDocument (.odt) a prostý text (.txt)
import-document-browse = Procházet
import-document-move-up = Posunout nahoru
import-document-move-down = Posunout dolů
import-document-remove-file = Odebrat
import-document-file-count = { $count ->
    [one] 1 soubor
   *[other] { $count } souborů
}
import-document-no-files = Zatím nebyly vybrány žádné soubory.
import-document-col-included = Importovat
import-document-col-title = Název
import-document-col-type = Typ
import-document-col-words = Slova
import-document-col-breaks = Přestávky
import-document-col-comments = Komentáře
import-document-col-footnotes = Poznámky
import-document-col-epigraph = Motto
import-document-col-source = Zdroj
import-document-level-rules = Hlavičky
import-document-level-n = Do čísla { $level }
import-document-add-top-level = Přidat nejvyšší úroveň
import-document-add-top-level-tooltip = Vložit knihu nad každou analyzovanou položku, pro soubory kapitoly, které nemají název knihy
import-document-destination = Země určení
import-document-destination-hint = Vyberte pořadač nebo položku: nové položky přistávají uvnitř složky nebo po scéně.
import-document-destination-empty = Zatím žádné pořadače.
import-document-plan-empty = Ještě není co dovážet.
import-document-summary = { $rows ->
    [one] 1 položka
   *[other] { $rows } položek
} · { $breaks ->
    [one] 1 scéna přestávka
   *[other] { $breaks } scény přestávky
}
import-document-back = Zpět
import-document-cancel = Zrušit
import-document-analyse = Další
import-document-analysing = Čtení dokumentů
import-document-step-analysing = Čtení
import-document-cancel-analysis = Přestaň číst.
import-document-analyse-failed = Dokumenty se nedaly přečíst.
import-document-details = Podrobnosti
import-document-import = Importovat
import-document-done = { $count ->
    [one] 1 dovezená položka
   *[other] { $count } dovezených položek
}
import-document-undo = Zpět
import-diagnostic-file-unreadable = Nelze číst { $path }: { $detail }. Ostatní soubory stále importují.
import-diagnostic-nested-too-deep = GEO{ $path } nebyla přečtena: jeho obsah je vnořen více než { $limit } úrovní hlubokých. Žádný skutečný dokument se nepřiblíží, a čtení by mohlo havárii Skribisto, takže je vynechán. Ostatní soubory stále importovat.
import-diagnostic-archive-too-large = Nečetl se { $path }: je to komprimovaný soubor postavený k odbalení mnohem většího, než je, který by mohl vyčerpat paměť a havárii Skribisto, takže je vynechán. Ostatní soubory stále importovat.
import-diagnostic-lossy-decode = { $count ->
    [one] Jeden znak v tis. { $path } Nedekódoval. Re- uložit soubor jako UTF-8, aby se to.
   *[other] { $count } znaků v tis. { $path } Nedekódoval. Re- uložit soubor jako UTF-8, aby se jim.
}
import-diagnostic-decoded-from-bom = Dohromady{ $path } byl dekódován jako{ $detail }, neUTF-8.
import-diagnostic-empty-file = CLAS{ $path } je prázdný.
import-diagnostic-no-headings = "{ $path }Nemá žádné tituly, takže přichází jako jeden předmět.
import-diagnostic-unsupported-format = Nic nečte .{ $detail } soubory, takže jste přeskočili{ $path }.
import-diagnostic-front-matter-not-flat = Metadata v „{ $path }“: „{ $detail }“ není jednoduchá hodnota a byla přeskočena.
import-diagnostic-footnotes-degraded = { $count ->
    [one] Jedna poznámka pod čarou v poznámce pod čarou č.{ $path } se uvádí jako prostý text: poznámky pod čarou nejsou čteny z položkyMarkdown.
   *[other] { $count } poznámky pod čarou v poznámce pod čarou{ $path } se doručují jako prostý text: poznámky pod čarou nejsou čteny zMarkdown.
}
import-diagnostic-footnote-not-carried = { $count ->
    [one] Jedna poznámka v poznámce pod čarou č. { $path } nemohla být předána. Ostatní byly.
   *[other] { $count } poznámky pod čarou v DON { $path } could not be bring over. Ostatní byly.
}
import-diagnostic-raw-html-dropped = { $count ->
    [one] Jeden blok HTML v „{ $path }“ byl vynechán.
    [few] { $count } bloky HTML v „{ $path }“ byly vynechány.
   *[other] { $count } bloků HTML v „{ $path }“ bylo vynecháno.
}
import-diagnostic-nested-break-dropped = { $count ->
    [one] Jeden předěl scény uvnitř citace nebo seznamu v „{ $path }“ byl vynechán. Zachovají se pouze předěly na samostatném řádku.
    [few] { $count } předěly scén uvnitř citací nebo seznamů v „{ $path }“ byly vynechány. Zachovají se pouze předěly na samostatném řádku.
   *[other] { $count } předělů scén uvnitř citací nebo seznamů v „{ $path }“ bylo vynecháno. Zachovají se pouze předěly na samostatném řádku.
}
import-diagnostic-image-not-ingested = „{ $path }“ odkazuje na obrázek „{ $detail }“. Odkaz se importuje jako text; samotný obrázek se nekopíruje.
import-diagnostic-duplicate-title = „{ $title }“ se vyskytuje { $count }krát. Pokud jste tyto soubory již importovali, vzniknou duplikáty.
import-diagnostic-heading-level-jump = Nadpis „{ $title }“ přeskakuje z úrovně { $from } na { $to }; bude umístěn pod nadřazenou úroveň.
import-diagnostic-illegal-combination = GEO{ $title } přenáší prózu, ale { $kind } nemůže držet žádný. Import se koná, dokud nezměníte jeho typ nebo jej nevynecháte. Nic není dováženo vůbec jinak.
import-diagnostic-tracked-changes-flattened = { $path } byl mid- revision: { $count ->
    [one] { $count } byla přijata změna stopy
   *[other] Bylo přijato { $count } sledovaných změn
} , a smazání klesl. To je konečný text. Chcete-li zachovat některý z vašich vlastních znění místo, použijte Porovnat v posledním kroku.
import-diagnostic-tracked-changes-flattened-by = { $path } byl mid- revision: { $count ->
    [one] { $count } byla přijata sledovaná změna o { $names }
   *[other] { $count } byly přijaty sledované změny o { $names }
} , a smazání klesl. To je konečný text. Chcete-li zachovat některý z vašich vlastních znění místo, použijte Porovnat v posledním kroku.
import-diagnostic-text-box-dropped = { $count ->
    [one] { $path } má { $count } textový rámeček. Jeho text leží mimo průtok dokumentů, takže kam patří do rukopisu nelze odpovědět. Není importován.
   *[other] { $path } má { $count } textových rámečků. Jejich text leží mimo průtok dokumentů, takže kam patří do rukopisu nelze odpovědět. Nejsou dováženy.
}
import-diagnostic-embedded-object-dropped = { $count ->
    [one] { $path } drží { $count } vložený objekt: graf, rovnice nebo podobné. Neexistuje nic v rukopisu, který by mohl držet.
   *[other] { $path } drží { $count } vložených objektů: graf, rovnice nebo podobné. Neexistuje nic v rukopisu, který by je mohl držet.
}
import-diagnostic-field-flattened = { $count ->
    [one] { $path } drží { $count } pole: číslo stránky, křížový odkaz, datum. Udržuje text, který naposledy ukazoval a nebude aktualizovat znovu.
   *[other] { $path } má { $count } polí: číslo stránky, křížový odkaz, datum. Každý uchovává text, který byl naposledy zobrazen a nebude aktualizovat znovu.
}
import-diagnostic-unknown-style-level = Soubor { $path } používá styl „{ $detail }“, který vypadá jako nadpis, ale neuvádí úroveň. Tyto odstavce se importují jako běžný text bez odhadování úrovně.
import-diagnostic-comment-unanchored = Komentář „{ $detail }“ v { $path } nelze připojit k původnímu textu. Zůstane u položky, kde jej můžete přesunout.
import-diagnostic-comment-not-carried = Komentář „{ $detail }“ v { $path } se neimportuje, protože soubor neobsahuje importovaný text, ke kterému by jej bylo možné připojit. Pokud jej potřebujete, zkopírujte ho ze souboru.
import-diagnostic-comment-replies-flattened = { $count ->
    [one] { $count } odpověď v { $path } označil komentář, který není ve spisu, a tak přichází jako vlastní komentář.
   *[other] { $count } odpovědi v { $path } jmenoval komentář, který není ve spisu, a tak přicházejí jako vlastní připomínky.
}
import-diagnostic-prose-not-verbatim = { $count ->
    [one] { $count } odstavec v { $path } nemohl být předán s jeho formátování přesně tak, jak je psáno, takže dorazí jako prostý text. Porovnejte jej s originálem.
   *[other] { $count } odstavce v { $path } nemohl být předán s jejich formátování přesně tak, jak je psáno, takže dorazí jako prostý text. Porovnejte je s originálem.
}
import-diagnostic-styled-spaces-not-carried = { $count ->
    [one] V { $path }, { $count } úsek prázdného prostoru byl podtržen nebo prorazil, linka vlevo vyplnit například. Skribisto nemá řádek na mezery sám, tak to dorazí jako prosté prostory, nebo vůbec, kde to začíná nebo končí odstavec.
   *[other] V { $path }, { $count } úseky prázdného prostoru byly podtrženy nebo prorazil, linky vlevo vyplnit například. Skribisto nemá řádek na mezery sám, takže přicházejí jako prosté prostory, nebo vůbec, kde začínají nebo končí odstavec.
}
import-diagnostic-list-nesting-flattened = { $count ->
    [one] { $count } seznam položky v { $path } byl vnořen více než { $limit } úrovní hluboké. Dorazí na úrovni { $limit }, nejhlubší Skribisto udržuje, s jeho slova a formátování.
   *[other] { $count } seznam položek v { $path } byly usazeny více než { $limit } úrovní hluboké. Dorazí na úrovni { $limit }, nejhlubší Skribisto udržuje, s jejich slovy a formátování.
}
import-diagnostic-table-read-as-paragraphs = { $count ->
    [one] { $count } tabulka v { $path } má řadu mnohem širší než ostatní, příliš zaokrouhlené, aby se položila jako mřížka. Jeho buňky přicházejí jako odstavce, jeden za druhým, s jejich slova a formátování.
   *[other] { $count } tabulky v { $path } mají řadu mnohem širší než ostatní, příliš zaokrouhlené, aby se položily jako mřížka. Jejich buňky přicházejí jako odstavce, jeden za druhým, s jejich slova a formátování.
}
import-diagnostic-spaces-shortened = { $count ->
    [one] { $count } běh prostor v { $path } byl delší než { $limit } prostory, a dorazí zkrácen na { $limit }. Slova kolem toho jsou všechny tam.
   *[other] { $count } šňůry prostor v { $path } byly delší než { $limit } prostory, a dorazí zkrácen na { $limit }. Slova kolem nich jsou všechny tam.
}
import-diagnostic-epigraph-not-carried = V čele s epigrafem je { $title }, ale { $kind } ho nemůže držet. Citace je vedena v horní části svého textu.
import-diagnostic-epigraph-placement-ambiguous = Epigraf se nachází mezi { $title } a DOPLŇUJE { $below } a mohl hlavu buď. To bylo dáno k DOPLŇKOVÉ { $title }, což je místo, kam epigraf obvykle jde.
import-document-diagnostics = { $errors ->
    [0] { $warnings ->
            [one] Jedna věc vědět
           *[other] { $warnings } věcí vědět
        }
   *[other] { $errors ->
            [one] Nelze přečíst 1 soubor
           *[other] { $errors } souborů nelze přečíst
        }
}
import-document-diagnostics-none = Není co hlásit.

versions-title = Verze
versions-scope-synopsis = Shrnutí
versions-scope-prose = Text
versions-loading = Prohlížet si vaše zálohy
versions-empty = Žádná dřívější verze tohoto ještě
versions-error = Nemohl jsem si přečíst vaše zálohy. Nic se neztratilo, ale tento seznam může být neúplný
versions-did-not-exist = Existovalo ještě { $date }
versions-deleted-after = Vymazáno někdy po { $date }
versions-unreadable = { $count ->
    [one] { $count } záloha mohla být přečtena
   *[other] { $count } záloha mohla být přečtena
}
versions-thinned = { $count ->
    [one] Starší verze jsou tenčí, jak stárnou. Project 's vlastní historie již klesla { $count } dřívější stav tohoto textu.
   *[other] Starší verze jsou tenčí, jak stárnou. Project 's vlastní historie již klesla { $count } dřívější stavy tohoto textu.
}
versions-source-backup = Ze zálohy.
versions-source-project = Z projektu je vlastní historie
versions-list-caption = Jeden vstup za změnu, ne za zálohu
versions-pick-a-version = Vyberte si verzi, abyste viděli, co se změnilo
versions-earliest = Není nic staršího, s čím by se dalo porovnat.
versions-no-change = V této části položky se nic nezměnilo.
versions-formatting-only = Pouze formátování se zde změnilo. Slova jsou stejná
versions-show-unchanged = Zobrazit beze změny
versions-hide-unchanged = Skrýt beze změny
versions-next-change = Další změna
versions-near = v blízkosti města { $text }
versions-words-added = { $count ->
    [one] { $count } přidáno slovo
   *[other] { $count } přidána slova
}
versions-words-removed = { $count ->
    [one] { $count } odstraněno slovo
   *[other] { $count } odstraněna slova
}
versions-blocks-moved = { $count ->
    [one] { $count } odst.
   *[other] { $count } posunuté odstavce
}
versions-pin = Připnout zálohu, ze které tato verze pochází. Automatické čištění ji nikdy nesmaže.
versions-unpin = Odpojit zálohu tato verze pochází z. Automatické čištění může smazat znovu
versions-pinned-only = Zobrazit pouze připnuté verze
versions-pin-note = Pouze verze ze zálohy mohou být připnuty. Pin uchovává soubor, a projekt je vlastní historie je jeden.
versions-pinned-empty = Nic tu ještě není přišpendlené.
versions-pinned-empty-log = Nic tu není přišpendlené. Pouze verze ze zálohy mohou být připnuty. Pin uchovává soubor, a projekt je vlastní historie je jedna.
versions-range-filter = Zobrazovat pouze verze mezi dvěma daty
versions-filtered-empty = Žádná verze neodpovídá filtrům, které jste nastavili
versions-clear-filters = Vyčistit filtry
versions-last-30-days = Posledních 30 dní
versions-restore-button = Obnovit tuto verzi
versions-restore-confirm-title = Nahradit tento text verzí z { $date }?
versions-restore-confirm-text = To, co máte nyní, bude nahrazeno textem, který měl tento bod na adrese{ $date }.
versions-restore-confirm-with-comments = To, co máte nyní, bude nahrazeno textem, který měl tento bod na adrese{ $date }. { $count ->
    [one] { $count } komentář je ukotven v aktuálním textu a může být ponechán osiřelý.
   *[other] { $count } komentáře jsou zakotveny v aktuálním textu a mohou být ponechány osiřelé.
}
versions-restore-confirm-undo-note = Nejprve se provede záloha a Ctrl + Z to zruší v jednom kroku.
versions-restored-toast = Obnovená verze z { $date }
versions-undo = Zpět
versions-restore-row-gone = Tato položka již není součástí tohoto projektu.
versions-restore-no-home = Tato položka změnila typ od té doby, a starý text nemá kam jít
versions-restore-no-safety-copy = Vaše bezpečnostní záloha neutekla, takže se nic nezměnilo.
versions-restore-failed = Obnova selhala: { $error }
versions-restore-backup-busy = Záloha už běží. Zkuste to za chvíli znovu.
versions-restore-in-backup-file = Díváte se do záložního souboru, otevřete projekt sám, abyste se do něj vrátil.
versions-restore-no-project = Žádný projekt není otevřen
versions-recreate-button = Přines to zpátky.
versions-recreate-picker-title = Kam by to mělo jít?
versions-recreate-picker-empty = Tento projekt nemá žádný pořadač, který by ho tam dal.
versions-recreate-picker-confirm = Přines to sem.
versions-recreate-picker-cancel = Zrušit
versions-recreate-untitled = Tato položka
versions-recreate-confirm-title = Přivést zpět { $item }?
versions-recreate-confirm-text = Bude doplněna o{ $destination }, s textem, který měl na{ $date }.
versions-recreate-confirm-undo-note = Zpět, na zprávu, která následuje, bere to zpět ven.
versions-recreated-toast = „{ $item }“ je znovu ve vašem projektu
versions-recreated-partial-toast = { $count ->
    [one] „{ $item }“ je zpět, ale jeden z jeho textů nelze přečíst
    [few] „{ $item }“ je zpět, ale { $count } jeho texty nelze přečíst
   *[other] „{ $item }“ je zpět, ale { $count } jeho textů nelze přečíst
}
versions-recreate-already-here = Ta položka už je ve vašem projektu.
versions-recreate-no-destination = Vyberte si místo v pořadači, aby se to
versions-recreate-unreadable = Ta záloha nemohla být přečtena, takže nic nebylo přidáno.
versions-recreate-failed = Nemohl jsem to vrátit: { $error }
versions-changed-percent = { $percent }% z toho se změnilo
versions-hidden-paragraphs = { $count ->
    [one] { $count } nezměněný odstavec
    [few] { $count } nezměněné odstavce
   *[other] { $count } nezměněných odstavců
}

timeline-title = Vrať se v čase.
timeline-coverage = { $count ->
    [one] { $count } nahraná verze, která se vrací k { $oldest }
   *[other] { $count } nahraných verzí, které se vracejí k { $oldest }
}
timeline-loading = Prohlížet si váš projekt je minulost
timeline-empty = Zatím nebyla zaznamenána žádná verze tohoto projektu.
timeline-no-changes = Od té doby se nic nezměnilo.
timeline-no-text-changes = Od té doby se žádný text nezměnil.
timeline-slider-label = Zaznamenaná verze
timeline-series-name = Velikost projektu
timeline-bars-caption = Každý bar je nahraná verze, stejně vysoká jako projekt byl tehdy. Zvýrazněný je to, na co se díváte.
timeline-bars-caption-periods = Příliš mnoho verzí ukázat jeden po druhém, takže každý bar je { $period }, tak vysoký, jak projekt byl na konci.
timeline-unit-hour = hodina
timeline-unit-day = den
timeline-unit-week = týden
timeline-unit-month = měsíc
timeline-range-filter = Zobrazovat pouze verze mezi dvěma daty
timeline-range-empty = V těchto datech nebyla zaznamenána žádná verze
timeline-open-period = Otevřít toto období
timeline-show-all = Zobrazit celou historii
timeline-changed-since = { $count ->
    [one] { $count } položka se liší mezi { $date } a váš projekt nyní
   *[other] { $count } položek se liší mezi { $date } a váš projekt nyní
}
timeline-kind-added = Napsal od
timeline-kind-removed = Již není v projektu
timeline-kind-changed = Od té doby upraveno
timeline-kind-moved = Přesunuto od
timeline-not-yet-written = To ještě neexistovalo.
timeline-no-text-of-its-own = Nemá to žádný vlastní text, ale míří k tomu, co je uvnitř.
timeline-reader-close = Zavřít
timeline-reader-stamp = Jak to bylo na { $date }
timeline-reader-loading = Otevírání nahrané verze
timeline-reader-unreadable = Tato nahrávka mohla být přečtena. Zálohování mohlo být přesunuto, smazáno, nebo být na disku, který je připojen.
timeline-reader-compared = Jak to bylo na { $date }, nastavit proti tomu, co říká teď
timeline-reader-diff-legend = Prorazil se od té doby, podtržen byl přidán.
timeline-reader-view-label = Co ukázat
timeline-reader-view-diff = Změny
timeline-reader-view-text = Text
timeline-reader-deleted = Už to není ve vašem projektu. Můžete si to přečíst a zkopírovat zde.
timeline-prose-only-record = Tento bod pochází z projektu je vlastní historie, která udržuje text a nic jiného, takže to, co bylo smazáno nebo přesunuto, protože to může být zobrazeno. Vyberte si zálohu pro to.

settings-sec-extensions = Rozšíření

settings-group-games-forward = Vždy dopředu.
settings-games-forward-toggle = Přehrát vždy dopředu
settings-games-forward-hint =
    Když hrajete, nic, co jste napsali, nelze vzít zpět: Backspace, Smazat,
    Střih, drag- and- drop a Undo jsou zakázány v površích, které si zvolíte níže.
    Stále můžete psát, vložit, formátovat a pohybovat se volně. Návrh roste pouze.
settings-games-session-warning =
    Toto je per- session volba: není nikdy uložen. Ukončení projektu, nebo ukončení projektu
    Skribisto, vždy končí hru, a vše, co jste napsal při hraní se stává
    Neuskutečnitelný znovu ve chvíli, kdy se to stane.
settings-group-games-scope = Použije-li se
settings-games-in-prose = Rukopis próza
settings-games-in-synopsis = Synopse
settings-games-scope-hint =
    Komentáře, poznámky pod čarou a názvy se nikdy nezmrazí: jsou tam, kde si všimnete fixu
    Právě jste si zakázali vyrábět.
settings-games-inert-warning =
    Vždy dopředu je zapnutý, ale vztahuje se na nic. Tisk alespoň jeden povrch nad,
    nebo to nic nezmění, když píšeš.

games-title = Psaní hry
games-forward-name = Vždy dopředu.
games-forward-blurb =
    Návrh, aniž by něco zpět. Smazání, řezání a zrušení jsou zakázány
    Napiš další větu, místo toho, abys napravil tu poslední.
games-forward-playing = Přehrávání: smazání je zakázáno
games-forward-idle = Nehrajte.
games-session-note = Končí, když uzavřeš projekt.
games-scope-prose-and-synopsis = Platí pro vaši prózu a synovce.
games-scope-prose = Platí pro vaši prózu.
games-scope-synopsis = Platí pro tvé synovce.
games-scope-nothing = Platí pro nic zatím. Vyberte plochu v Nastavení.
games-settings-link = Psaní nastavení hry
statusbar-games-forward = Vždy dopředu.
statusbar-games-forward-tooltip =
    Vždy vpřed je zapnuto: smazání, řezání a zrušení jsou zakázány, zatímco vy
    draft. Klikněte pro zastavení přehrávání.
goal-progress-words = { $g ->
   *[other] Slova: { $count } / { $goal }
}
goal-progress-characters = { $g ->
   *[other] Znaky: { $count } / { $goal }
}
goal-count-words = { $n ->
    [one] { $count } slovo
    [few] { $count } slova
    [many] { $count } slova
   *[other] { $count } slov
}
goal-count-characters = { $n ->
    [one] { $count } znak
    [few] { $count } znaky
    [many] { $count } znaku
   *[other] { $count } znaků
}
goal-unit-words = Slova
goal-unit-characters = Znaky

inspector-goal = Cíl
inspector-goal-none = Žádný cíl.

overview-col-goal = Cíl
overview-excluded-from-export = Vynecháno z exportu, takže se to nepočítá do součtu.

goal-subtree-total-words = { $count ->
    [one] { $items } cíl uvnitř přidává až { $words } slov
   *[other] { $items } cíle uvnitř přidat až { $words } slov
}
goal-subtree-total-characters = { $count ->
    [one] { $items } cíl uvnitř přidává až { $words } znaků
   *[other] { $items } cíle uvnitř přidat až { $words } znaků
}

new-work-goal-unit = Počítejte v
new-work-goal-unit-hint = Používá se pro každé slovo nebo znak cíl v tomto projektu. Změnitelné později.

settings-group-goal-unit = Cíle
settings-goal-unit-switch-title = Spínací počítací jednotka
settings-goal-unit-switch-text = Cíle a milníky již stanovené pro tento projekt byly zadány v{ $from }. Přechod na{ $to } nepřevádí je, takže každé existující číslo bude nyní čteno jako{ $to }.
settings-goal-unit-switch-informative = Nic není ztraceno. Přepněte zpět kdykoliv obnovit původní čtení, pak aktualizovat cíle, které chcete udržet.
settings-goal-unit-switch-confirm = Přepínač

goal-distribute-action = Distribuce
distribute-title = Rozdělit cíl
distribute-empty = Není v tom nic, s čím bychom se mohli podělit o cíl.
distribute-weight = Sdílet
distribute-weight-length = Podle délky
distribute-weight-rows = Po částech
distribute-weight-even = Večer.
distribute-overwrite = Nahradit již stanovené cíle
distribute-col-item = Položka
distribute-col-current = Teď.
distribute-col-proposed = Po
distribute-total = Ty přičítají k{ $total }, proti cíli{ $goal }.
distribute-over-budget = Cíle již stanovené uvnitř přijít na { $over } více než tento kontejner vlastní. Zvýšit svůj cíl, snížit jejich, nebo nahradit.
distribute-apply = Distribuce
distribute-cancel = Zrušit

milestone-target-gone = Cíl vymazán
milestone-no-target = Žádný cíl.
milestone-add = Přidat
milestone-add-label = Čeho je třeba dosáhnout

pace-summary-title = Kde stojí kniha
pace-summary-remaining = { $words } zbývá.
pace-summary-open = Otevři plán.
pace-summary-close = Zavřít
pace-summary-dont-show = Neukazujte to při otevírání
pace-summary-menu = Psací plán

menu-help-topics = Nápověda Topics (&T)
menu-help-shortcuts = Klávesové zkratky (&K)
menu-help-website = Skribisto na webu (&W)
menu-help-report = Nahlásit problém (&R)
menu-help-check-updates = Zkontrolujte aktualizace (&U)
menu-command-palette = Příkazová paleta (&C)

learn-help-topics = Nápověda témata
learn-shortcuts = Klávesové zkratky
learn-website = Skribisto na webu

help-window-title = Nápověda
help-filter-topics = Filtrační témata
help-no-matching-topic = Žádné téma se neshoduje.
help-topic-missing = Toto téma již není k dispozici.
help-back = Zpět
help-not-translated = Tento článek zatím není dostupný v češtině. Zobrazuje se anglicky.

help-section-getting-started = Začínáme
help-section-writing = Psaní
help-section-reviewing = Revize
help-section-exchanging = Dostávám slova dovnitř a ven.
help-section-keeping = Udržet váš projekt v bezpečí
help-section-extensions = Rozšíření

help-topic-getting-started = Váš první projekt
help-topic-writing-model = How a book is table
help-topic-drafts-and-old-versions = Vedení staršího návrhu
help-topic-goals-and-pace = Cíle a tempo
help-topic-comments = Komentáře
help-topic-round-trip = Posílání své knihy čtenáři
help-topic-export = Vývoz
help-topic-import-documents = Dovážející doklady
help-topic-import-projects = Přinášíme celý projekt
help-topic-backups-and-versions = Zálohy a verze

help-shortcuts-title = Klávesové zkratky
help-shortcuts-filter = Filtrační zkratky
help-shortcuts-no-matches = Žádné zkratky se neshodují.
help-shortcuts-rebind = Změnit zkratky
help-shortcuts-close = Zavřít

command-palette-placeholder = Napište příkaz

shortcut-name-binder-duplicate = Duplikovat
shortcut-name-comments-add = Přidat komentář
shortcut-name-comments-add-paragraph = Komentář k odstavci
shortcut-name-spellcheck-toggle = Kontrola pravopisu
shortcut-name-editor-tab-close = Zavřít kartu
shortcut-name-editor-tab-pin = Pin nebo Unpin Tab
shortcut-name-editor-save = Uložit
shortcut-name-work-export = Exportovat…
shortcut-name-work-new = Nový projekt
shortcut-name-work-open = Otevřený projekt
shortcut-name-window-new = Nové okno
shortcut-name-work-close = Zavřít projekt
shortcut-name-app-settings = Nastavení
shortcut-name-app-quit = Ukončit
shortcut-name-editor-insert-footnote = Vložit poznámku pod čarou
shortcut-name-format-scene-break = Vložit průlom scény
shortcut-name-format-major-scene-break = Vložit hlavní scéna break
shortcut-name-format-link = Vložit odkaz
shortcut-name-format-mark-nbsp = Vložit no- break prostor
shortcut-name-format-mark-nbhyphen = Vložit no- break Hyphen
shortcut-name-format-mark-nnbsp = Vložit úzký no- break prostor
shortcut-name-format-mark-zwsp = Vložit Zero- width prostor
shortcut-name-go-next = Další
shortcut-name-go-prev = Předchozí
shortcut-name-go-to = Přejít na
shortcut-name-outline-toggle = Přepnout obrys
shortcut-name-preview-toggle = Přepnout náhled pásma
shortcut-name-view-fullscreen = Přepnout Fullscreen
shortcut-name-view-focus-mode = Přepnout režim distraktion- free
shortcut-name-editor-size-increase = Zvýšit velikost textu
shortcut-name-editor-size-decrease = Snížit velikost textu
shortcut-name-editor-size-reset = Obnovit velikost textu
shortcut-name-editor-find = Najít
shortcut-name-editor-replace = Nahradit
shortcut-name-editor-find-next = Další zápas
shortcut-name-editor-find-prev = Předchozí shoda
shortcut-name-search-show = Vyhledávání v projektu
shortcut-name-search-replace = Nahradit projekt
shortcut-name-outline-open-to-side = Otevřít na stranu
shortcut-name-help-topics = Nápověda Topics
shortcut-name-help-shortcuts = Klávesové zkratky
shortcut-name-help-website = Skribisto na webu
shortcut-name-help-report = Nahlásit problém
shortcut-name-help-check-updates = Zkontrolujte aktualizace
update-check-behind = Verze { $version } je k dispozici.
update-check-current = Spouštíš nejnovější verzi.
update-check-failed = Nelze zkontrolovat novější verzi. { $error }
shortcut-name-command-palette = Příkazová paleta
help-section-reference = Přehled pojmů

help-concept-scene-break = Protržení scény
help-concept-major-scene-break = Přestávka na velkou scénu
help-concept-find-in-prose = Najít v próza
help-concept-story-bible = Encyklopedie příběhu
help-concept-goal-unit = Slova nebo znaky
help-concept-goal-progress = Průběh
help-concept-manuscript-words = Co se počítá jako rukopis
help-concept-exportable = Vynecháno z vývozu
help-concept-distribute = Sdílení cíle ven
help-concept-subtree-total = Co cíle uvnitř přidat k
help-concept-milestone = Milník
help-concept-pace-plan = Paceův plán

help-concept-tag = Štítek
help-concept-label = Popisek
help-concept-point-of-view = Úhel pohledu
help-concept-epigraph = Motto
help-concept-footnote = Poznámka pod čarou
help-concept-chapter-mode = Tvar kapitoly
help-concept-comment = Komentář
help-concept-backup = Záloha
help-concept-version = Verze
help-concept-trash = Koš
help-concept-spellcheck = Kontrola pravopisu
help-concept-search-replace = Hledat a nahradit
help-concept-note-template = Poznámka:
help-concept-story-bible-entry = Zápis z bible příběhu
help-concept-text-replacement = Náhrada textu
help-concept-smart-punctuation = Inteligentní interpunkce
help-concept-export-style = Styl exportu
help-concept-round-trip-marks = Značky kruhových výjezdů

epigraph-mark-attribution = Zdroj
epigraph-mark-attribution-tip = Označte řádek, ve kterém je kareta jako zdroj citace, takže tiskne jako přiřazení
margin-lane-name = Značky marží
margin-lane-provider-comments = Komentáře
margin-lane-provider-story-bible = Zápis z bible příběhu
margin-lane-provider-story-bible-hint = Na vědomí se nachází v próza čtení, kde je tento záznam pojmenován, a scény vyprávěné jeho očima. Také vyznačuje tato jména v próza sám.
margin-lane-mark-point-of-view = Odsud jsem to řekl.
margin-lane-mark-named-here = Pojmenován zde
margin-lane-provider-comments-hint = Je-li ke textu připojena poznámka
margin-lane-provider-search = Hledání hitů
margin-lane-provider-search-hint = Každý zásah po tom, co jste naposledy hledali, ať už je to kdekoliv v dokumentu.
margin-lane-provider-boundaries = Tam, kde každý dokument začíná
margin-lane-provider-boundaries-hint = Pravidlo na vrcholu každé scény v proudu, takže můžete vidět, kde jste
margin-lane-provider-spelling = Hláskování
margin-lane-provider-spelling-hint = Tam, kde kontrola pravopisu označil slovo. Vypnuto, pokud si o to nepožádáte: to označuje stroj názor na vaše próza
margin-lane-spelling = { $word }, možný omyl
margin-lane-search-hit = { $text }, utkání { $index } z { $total }
margin-lane-search-current = { $text }, zápas { $index } z { $total }, ten, na kterém jste
margin-lane-boundary = Začátek { $title }
margin-lane-boundary-untitled = Nepojmenovaný dokument
settings-page-margin-lane = Značky marží
settings-desc-margin-lane = Strip vedle svitku, a to, co ukazuje
settings-margin-lane-enabled = Zobrazit pruh okraje
settings-margin-lane-enabled-hint = Stejný přepínač, jako značky na Margin.
settings-margin-lane-enabled-more = Nikdy neříká, že je něco špatně: ukazuje vám to, kde věci jsou, a nechává vám to, co si o tom myslet.
settings-group-margin-lane-marks = Co to značí
settings-margin-lane-no-providers = Zatím nic neznačí pruh.
settings-group-margin-lane-texture = Struktura dialogu
settings-margin-lane-texture = Zobrazit texturu dialogu
settings-margin-lane-texture-hint = Jeden sloupec za odstavec: jak dlouho to je a kolik z toho se mluví.
settings-margin-lane-texture-more = Neměřeno pro jazyky bez kadeřnické konvence, kde bar ukazuje délku sám.
settings-group-margin-lane-surfaces = Kde se to objeví
settings-margin-lane-surface-editor = Editor textu
settings-margin-lane-surface-stream = Řeky
settings-margin-lane-surface-search-preview = Náhled vyhledávání
menu-margin-lane = Značky marží (&M)

status-preset-drafting = Psaní konceptu
status-preset-passes = Revizní průkazy
status-preset-plume = Plume Creator
status-todo = K vypracování
status-draft = Koncept
status-revised = Revidováno
status-final = Finální
status-outline = Osnova
status-first-edit = 1. editace
status-second-edit = 2. editace
status-done = Hotovo
status-plume-draft-1 = První návrh
status-plume-draft-2 = 2. návrh
status-plume-draft-3 = Třetí návrh
status-plume-edit-1 = 1. editace
status-plume-edit-2 = 2. editace
status-plume-edit-3 = 3. editace
status-plume-proofread = Po korektuře
status-plume-finished = Dokončeno
new-work-statuses = Pracovní postup
new-work-statuses-hint = Fáze můžete přesunout scénu přes. Můžete přejmenovat, přeobjednat nebo přidat k nim kdykoliv.
status-none = Bez statusu
inspector-status = Stav
overview-col-status = Stav
overview-status-mixed = níže uvedené části nesouhlasí
help-concept-status = Stav
status-completion-title = Kde stojí kniha
status-completion-headline = { $done } z { $total } dokončených scén
status-completion-empty = Zatím žádné scény, tohle čte rukopis, takže se vyplní, jak píšete.
status-completion-open = Kde stojí kniha

settings-page-statuses = Statuty
settings-desc-statuses = Přejmenovat, přeobjednat, přidat a odstranit fáze budete pohybovat scénu přes
settings-statuses-add = Přidat stav
settings-statuses-add-placeholder = Název nového stavu
settings-statuses-desc =
    Fáze scény se pohybuje přes, v pořadí, od nejméně dokončené na vrcholu až po většinu
    Tento řád je to, co je méně dokončený, než znamená všude jinde.
    v aplikaci, takže je dejte tak, jak váš proces skutečně běží.
settings-statuses-apply-preset = Použít přednastavené nastavení
settings-statuses-preset-applied = { $added ->
    [one] 1 přidaný status
   *[other] Přidáno { $added } statů
}
settings-statuses-preset-refused = Tento projekt již má žebřík. Předvolby vyplní pouze prázdný. Smazat příčky, které nechcete první.
settings-statuses-duplicate = Na tomto žebříku je již{ $name }.
settings-statuses-added = Přidáno { $name }
settings-statuses-move-up = Posunout nahoru (méně dokončené)
settings-statuses-move-down = Posunout dolů (více dokončeno)
settings-statuses-details-placeholder = Co tato etapa znamená (nepovinné)
settings-statuses-delete = Vymazat { $name }
settings-statuses-delete-in-use = { $count ->
    [one] Smazat CLAS{ $name }: 1 položka je na něm a ztratí svůj status
   *[other] Smazat DOPLŇKOVÉ { $name }: { $count } položky jsou na něm a ztratí svůj status
}
settings-statuses-deleted = Zrušeno { $name }
settings-statuses-deleted-in-use = { $count ->
    [one] Vymazáno { $name }. 1 položka již nemá status.
   *[other] Vymazáno { $name }. { $count } položky již nemají status.
}
settings-statuses-empty-title = Zatím žádný pracovní postup.
settings-statuses-empty-body = Stav říká, jak daleko je scéna. Přidat jeden výše, nebo začít od přednastaveného.
status-category-planned = Plánované
status-category-drafting = Psaní konceptu
status-category-needs-work = Vyžaduje úpravy
status-category-revised = Revidováno
status-category-final = Finální
