# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

menu-work = &Projekt
menu-new-work = &Nový projekt
menu-open-work = &Otevřít projekt…
menu-new-window = No&vé okno
menu-import-from = &Importovat z
menu-create-from = &Vytvořit z
menu-import-plume = &Plume Creator (.plume)…
menu-import-document = &Dokumenty (Markdown, Word, ODT)…
menu-import-manuskript = &Manuskript (.msk)…
menu-export = &Exportovat
menu-export-book = Exportovat knihu
menu-export-part = Exportovat část
menu-export-chapter = Exportovat kapitolu
menu-export-scene = Exportovat scénu
menu-export-note = Exportovat poznámku
menu-export-paratext = Exportovat paratext
menu-export-folder = Exportovat složku
menu-export-choose = Vybrat…
menu-export-none = Otevřete dokument, který chcete exportovat
menu-save = &Uložit
menu-save-as-file = Uložit jako jeden &soubor…
menu-save-as-folder = Uložit jako slož&ku…
menu-backup = Zálo&hovat nyní
menu-close-work = &Zavřít projekt
menu-welcome = Úvo&dní obrazovka…
menu-settings = N&astavení
menu-quit = Ukonči&t

menu-view = &Zobrazení
menu-outline = &Osnova
menu-search = &Hledat v projektu
menu-timeline = Návr&at v čase
menu-search-preview = &Náhled hledání
menu-fullscreen = &Celá obrazovka
menu-focus-mode = Režim &bez rozptylování
menu-format = &Formát
menu-scene-break = Vložit předěl &scény
menu-major-scene-break = Vložit hlavní předěl scén&y

menu-go = Pře&jít
menu-go-next-scene = Další &scéna
menu-go-prev-scene = Předchozí s&céna
menu-go-next-chapter = Další &kapitola
menu-go-prev-chapter = Předchozí k&apitola
menu-go-next-note = Další &poznámka
menu-go-prev-note = Předchozí p&oznámka

menu-tools = &Nástroje
menu-help = Nápo&věda

ctx-add = &Přidat
ctx-new-item = &Nová položka
ctx-new-folder = Nová &složka
ctx-rename = Přej&menovat
ctx-duplicate = &Duplikovat
ctx-import-here = &Importovat sem…
ctx-indent = &Zvětšit odsazení
ctx-outdent = Zmenšit &odsazení
ctx-trash = Přesunout do &koše
ctx-open-to-side = Otevřít &vedle
ctx-open = &Otevřít
ctx-reveal-in-outline = Ukázat v o&snově
ctx-move-up = Posunout &nahoru
ctx-move-down = Posunout &dolů

ctx-tab-close = &Zavřít
ctx-tab-close-others = Zavřít &ostatní
ctx-tab-close-all = Zavřít &vše
ctx-tab-open-to-side = Otevřít v&edle
ctx-tab-open-in-main = Otevřít v &hlavním panelu
ctx-tab-move-to-side = &Přesunout vedle
ctx-tab-move-to-main = &Přesunout do hlavního panelu
ctx-tab-move-to-new-window = Přesunout do &nového okna
ctx-tab-move-window-unsaved = Nejprve tento projekt uložte: druhé okno se otevírá nad souborem na disku
ctx-tab-pin = Př&ipnout tuto kartu
ctx-tab-unpin = O&depnout tuto kartu
tab-pinned-tooltip = Připnuto: „Zavřít ostatní“ a „Zavřít vše“ ji nechají otevřenou

create-book = Kniha
create-part = Část
create-chapter = Kapitola
create-scene = Scéna
create-note = Poznámka
create-note-folder = Složka poznámek
create-folder = Složka
create-paratext = Paratext
create-paratext-folder = Složka paratextů
create-book-end = Konec knihy
create-story-bible-entry = Záznam bible příběhu…
type-book-start = Začátek knihy
type-text = Text

new-item-book = Nová kniha
new-item-part = Nová část
new-item-chapter = Nová kapitola
new-item-note = Nová poznámka
new-item-note-folder = Nová složka poznámek
new-item-folder = Nová složka
new-item-paratext = Nový paratext
new-item-paratext-folder = Nová složka paratextů
new-item-story-bible-entry = Nový záznam bible příběhu

placement-inside = dovnitř
placement-after = za
placement-after-parent = za nadřazenou položku
placement-top-level = nejvyšší úroveň


ctx-promote = Převést &na
promote-chapter-folder = Složka kapitoly
promote-flat-chapter = Plochá kapitola
promote-lossy-title = Text nemá kam přejít
promote-lossy-text = Položka typu „{ $target }“ nemá kam uložit tento obsah: { $kinds }. Nejprve tento text přesuňte nebo vymažte, potom převeďte.
content-scene-text = Text scény
content-note-text = Text poznámky
content-book-title = Název knihy
content-book-subtitle = Podtitul knihy
content-part-title = Název části
content-chapter-title = Název kapitoly
content-epigraph-text = Motto
content-paratext-text = Paratext
promote-blocked-title = Kapitola není prázdná
promote-blocked-text = { $count ->
    [one] Tato kapitola ještě obsahuje { $count } položku. Před převodem na plochou kapitolu ji přesuňte nebo vyhoďte do koše.
    [few] Tato kapitola ještě obsahuje { $count } položky. Před převodem na plochou kapitolu je přesuňte nebo vyhoďte do koše.
   *[other] Tato kapitola ještě obsahuje { $count } položek. Před převodem na plochou kapitolu je přesuňte nebo vyhoďte do koše.
}

inspector = Inspektor
inspector-empty = Otevřete položku, kterou chcete prohlížet.
inspector-promote = Převést na…
statusbar-toggle-outline = Zobrazit nebo skrýt pořadač
statusbar-toggle-inspector = Zobrazit nebo skrýt inspektor
statusbar-save-unsaved = Neuložené změny. Kliknutím uložíte
statusbar-save-saved = Všechny změny jsou uloženy
statusbar-save-autosave = Automatické ukládání je zapnuté. Změny se ukládají průběžně při psaní
statusbar-saving = Ukládání…
statusbar-word-count = { $count ->
    [one] { $count } slovo
    [few] { $count } slova
   *[other] { $count } slov
}
statusbar-word-count-tooltip = Slova ve scéně, kterou upravujete
statusbar-word-char-count = { $words ->
    [one] { $words } slovo
    [few] { $words } slova
   *[other] { $words } slov
} · { $chars ->
    [one] { $chars } znak
    [few] { $chars } znaky
   *[other] { $chars } znaků
}
session-toggle = Psací sezení: spustit nebo pozastavit soustředěný sprint
session-configure = Nastavit cílový počet slov a časový limit sezení
session-configure-title = Psací sezení
session-word-goal = Cílový počet slov
session-time-limit = Časový limit
session-time-limit-unit = min
session-no-goal = Bez cíle
session-no-limit = Bez limitu
session-reset = Vynulovat sezení
session-readout = { $words ->
    [one] { $words } slovo
    [few] { $words } slova
   *[other] { $words } slov
} · { $time }
session-readout-timed = { $words ->
    [one] { $words } slovo
    [few] { $words } slova
   *[other] { $words } slov
} · zbývá { $time }
statusbar-focus-exit = Ukončit režim bez rozptylování
statusbar-focus-synopsis = Synopse
statusbar-focus-go-prev = Předchozí položka (Alt+šipka nahoru)
statusbar-focus-go-next = Další položka (Alt+šipka dolů)

rail-settings = Nastavení

language = Jazyk
theme = Motiv
english = Angličtina
french = Francouzština
light = Světlý
dark = Tmavý
settings-text-width = Šířka textu
settings-preview-width = Šířka náhledu hledání
settings-autosave = Automaticky ukládat na disk
settings-show-welcome = Při spuštění zobrazit úvodní obrazovku
settings-show-welcome-tip = Je-li vypnuto, znovu se otevře poslední projekt.
settings-check-for-updates = Kontrolovat nové verze
settings-check-for-updates-tip = Nejvýše jednou denně se Skribisto zeptá webu projektu, která verze je aktuální. Požadavek nenese žádný identifikátor, dokonce ani verzi, kterou používáte: porovnání proběhne na tomto počítači.

settings-title = Nastavení
settings-close = Zavřít
settings-search = Hledat v nastavení
settings-reset = Obnovit výchozí
settings-done = Hotovo
settings-cancel = Zrušit
settings-apply = Použít
settings-ok = OK
settings-reset-confirm-title = Obnovit všechna nastavení na výchozí hodnoty?
settings-reset-confirm-body = Obnoví vzhled aplikace, nastavení editoru a psaní. Vlastní nastavení projektu, klávesové zkratky a uložené styly zůstanou beze změny. Tuto akci nelze vrátit zpět.
settings-empty-title = Zatím tu nejsou žádná nastavení
settings-empty-hint = Tato sekce získá volby v některé z příštích aktualizací.

settings-sec-appearance-behaviour = Vzhled a chování
settings-sec-editor = Editor
settings-sec-spelling = Pravopis
settings-sec-backup = Zálohování a synchronizace
settings-sec-compile = Sestavení a export
settings-page-appearance = Vzhled
settings-field-image-size-policy = Velké obrázky
settings-image-policy-ask = Pokaždé se zeptat
settings-image-policy-keep = Ponechat původní
settings-image-policy-downscale = Optimalizovat
settings-hint-image-size-policy = Co dělat, když je vložený obrázek větší, než rukopis potřebuje.
settings-page-notifications = Oznámení
settings-page-scene = Scéna
settings-page-synopsis = Synopse
settings-page-notes = Poznámky
settings-page-editor-behavior = Chování editoru
settings-page-goals = Cíle a počet slov
settings-page-games = Psací hry
settings-page-corkboard = Nástěnka
settings-page-distraction-free = Bez rozptylování
settings-page-dictionaries = Slovníky
settings-page-autosave = Automatické ukládání
settings-page-export = Exportní formáty
settings-page-paratext = Struktury paratextů
settings-page-keymap = Klávesové zkratky
settings-keymap-filter = Filtrovat zkratky

settings-desc-sec-appearance-behaviour = Jak vypadá samotná aplikace a co dělá při spuštění.
settings-desc-sec-editor = Psací plocha: jak vypadá a co dělá při psaní.
settings-desc-sec-spelling = Kontrola pravopisu a slovníky, o které se opírá.
settings-desc-sec-backup = Jak se vaše práce dostává na disk a jaké kopie se uchovávají.
settings-desc-sec-compile = Co z aplikace Skribisto odchází a v jaké podobě.
settings-desc-sec-work = Nastavení, která patří k tomuto projektu a jsou uložena v jeho souboru.
settings-desc-sec-extensions = Stránky přidané zde nainstalovanými rozšířeními.
settings-desc-group-typography = Jedna stránka pro každý druh textu: písmo, velikost, výška řádku a mezery.
settings-desc-appearance = Jazyk rozhraní, motiv, velikost textu a úvodní obrazovka při spuštění.
settings-desc-notifications = Všechny zprávy, které se během tohoto sezení zobrazily, a akce, které můžete zopakovat.
settings-desc-scene = Sazba prózy scén: písmo, velikost, výška řádku, odsazení a mezery.
settings-desc-synopsis = Sazba panelu synopse, nezávisle na rukopisu.
settings-desc-notes = Sazba poznámek, nezávisle na rukopisu.
settings-desc-corkboard = Velikost karet, co karta ukazuje a jak je nástěnka uspořádá.
settings-desc-distraction-free = Typografie, šířka sloupce a ovládací lišta pro psaní na celou obrazovku.
settings-desc-distraction-free-themes = Knihovna motivů pro psaní na celou obrazovku: dodané motivy i vaše vlastní.
settings-desc-editor-behavior = Šířka textu, umístění synopse, posouvání jako u psacího stroje a zvýraznění u kurzoru.
settings-desc-punctuation = Výchozí typografický styl, se kterým začínají nové projekty. Každý projekt se jím řídí, pokud si nenastaví vlastní.
settings-desc-goals = Cíle počtu slov a znaků a způsob počítání slov.
settings-desc-games = Omezení, která si sami ukládáte při psaní konceptu, například Vždy vpřed.
settings-desc-spellcheck = Jediný přepínač, který kontrolu pravopisu všude zapíná a vypíná.
settings-desc-dictionaries = Instalace, odebrání a procházení pravopisných slovníků v tomto počítači.
settings-desc-autosave = Zda se vaše úpravy zapisují na disk samy.
settings-desc-backup = Kdy se pořizují kopie, kde se uchovávají a kolik jich je. Každý projekt se tím řídí, pokud si nenastaví vlastní.
settings-desc-export = Styly, kterými prochází každý export: dodané i vaše vlastní.
settings-desc-paratext = Úvodní a závěrečné části knihy, se kterými může nový projekt začít.
settings-desc-keymap = Všechny klávesové zkratky a k čemu jsou přiřazeny.
settings-page-user = Uživatel
settings-desc-user = Kdo jste, pro komentáře, které píšete.
settings-group-identity = Identita
settings-field-user-name = Vaše jméno
settings-field-user-name-placeholder = Nepovinné
settings-field-user-name-hint = Podepisuje komentáře a odpovědi, které píšete. Liší se od jména autora projektu, které je autorským údajem knihy a je uloženo v jejím souboru.
settings-field-user-initials = Vaše iniciály
settings-field-user-initials-placeholder = Z vašeho jména
settings-field-user-initials-hint = Co textový procesor zobrazí vedle vašeho komentáře na okraji. Ponecháte-li pole prázdné, použijí se zobrazené iniciály odvozené z vašeho jména.
settings-field-user-hint = Obojí je nepovinné a platí pro všechny projekty v tomto počítači. Změna se projeví u komentářů, které napíšete příště. Již napsané komentáře si ponechají jméno, pod kterým vznikly.
settings-desc-author = Jméno, které se uvádí u tohoto projektu.
settings-desc-structure = Zda jsou kapitoly tohoto projektu složky, nebo samostatné položky.
settings-desc-language = Jazyk, podle kterého se kontroluje próza tohoto projektu.
settings-desc-work-backup = Vlastní pravidla zálohování tohoto projektu, nebo obecná pravidla.
settings-desc-personal-dictionary = Slova, která tento projekt považuje za správně napsaná.
settings-desc-tags = Barevné štítky, kterými tento projekt označuje své položky.
settings-desc-templates = Šablony poznámek, které můžete vkládat při psaní v tomto projektu.
settings-desc-text-replacements = Zkratky, které se v tomto projektu rozvinou při psaní.
settings-desc-work-punctuation = Uvozovky, pomlčky a mezery, kterými se řídí próza tohoto projektu.

settings-group-typography = Typografie
settings-group-writing-column = Sloupec textu
settings-group-writing-view = Zobrazení psaní
settings-group-distraction-free-strip = Ovládací lišta
settings-group-theme = Motiv
settings-group-language = Jazyk
settings-group-startup = Spuštění
settings-group-updates = Aktualizace
settings-group-autosave = Automatické ukládání
settings-field-typeface = Písmo
settings-field-size = Velikost textu

editor-size-changed-manuscript = Velikost textu rukopisu: { $percent }
editor-size-changed-synopsis = Velikost textu synopse: { $percent }
editor-size-changed-notes = Velikost textu poznámek: { $percent }
editor-size-changed-corkboard = Velikost textu karet nástěnky: { $percent }
editor-size-changed-corkboard-expanded = Velikost textu rozbaleného editoru: { $percent }
editor-size-changed-distraction-free = Velikost textu v režimu bez rozptylování: { $percent }

menu-text-size-increase = Zvě&tšit text
menu-text-size-decrease = Z&menšit text
menu-text-size-reset = Obnovit &velikost textu
settings-field-line-height = Výška řádku
settings-field-first-line-indent = Odsazení prvního řádku
settings-field-paragraph-spacing-before = Mezera před odstavcem
settings-field-paragraph-spacing-after = Mezera za odstavcem
settings-field-column-width = Šířka sloupce
settings-distraction-free-width-hint = Platí jen v režimu bez rozptylování. Šířky sloupců scény, synopse a poznámek zůstanou beze změny.
settings-distraction-free-title = Ponechat název položky
settings-distraction-free-word-count = Ponechat počet slov
settings-distraction-free-session = Ponechat psací sezení
settings-distraction-free-go-to = Ponechat tlačítko Přejít na…
settings-distraction-free-go = Ponechat tlačítka Předchozí a Další
settings-distraction-free-chrome-hint = Tlačítko Ukončit zůstává vždy, ať zde zvolíte cokoli. Je to vaše cesta ven, pokud je klávesa Esc obsazená.
settings-field-app-theme = Motiv
settings-theme-light = Světlý
settings-theme-dark = Tmavý
settings-field-text-scale = Velikost textu rozhraní
settings-field-language = Jazyk rozhraní
settings-synopsis-placement = Umístění synopse
settings-synopsis-placement-none = Žádné
settings-synopsis-placement-top = Nad textem
settings-synopsis-placement-side = Vedle textu
settings-typewriter = Posouvání jako u psacího stroje
settings-typewriter-tip = Drží řádek, na kterém píšete, ve stálé výšce, zatímco se rukopis posouvá pod ním. Kliknutím stále umístíte kurzor tam, kam kliknete.
settings-typewriter-position = Poloha řádku
settings-typewriter-position-top-third = Horní třetina
settings-typewriter-position-middle = Uprostřed
settings-typewriter-position-bottom-quarter = Dolní čtvrtina
settings-highlight-scope = Zvýraznění kolem kurzoru
settings-highlight-scope-none = Žádné
settings-highlight-scope-sentence = Věta
settings-highlight-scope-paragraph = Odstavec
settings-highlight-scope-tip-none = Stránka zůstane čistá. Při psaní se nic nestínuje.
settings-highlight-scope-tip-sentence = Podbarví větu, kterou píšete, aby vynikla mezi okolními.
settings-highlight-scope-tip-paragraph = Podbarví celý odstavec, který píšete, abyste měli pasáž, na které pracujete, na očích.
settings-group-container-views = Zobrazení kontejnerů
settings-remember-view = Pamatovat si poslední zobrazení pro každý typ položky
settings-remember-view-tip = Otevře kontejner v zobrazení, které jste pro daný typ použili naposledy
settings-remember-view-tip-more =
    Kniha, část i kapitola nabízejí několik zobrazení (vlastní stránku, celý
    rukopis, celou synopsi). Když tuto volbu zapnete, otevře se každý typ ve
    zobrazení, které jste pro něj zvolili naposledy: přepnete-li jednu kapitolu na
    Celou kapitolu, další otevřená kapitola ukáže také Celou kapitolu. Každý typ si pamatuje své zobrazení.
settings-group-counting = Počítání slov
settings-counting-auto = Automaticky (podle jazyka)
settings-counting-whitespace = Dělit podle mezer
settings-counting-unicode-words = Slova podle Unicode
settings-counting-cjk-hybrid = S ohledem na CJK (po znacích)
settings-counting-hint = Automatický režim počítá čínštinu a japonštinu po znacích a všechny ostatní jazyky po slovech. Měňte jej jen tehdy, když se počet pro váš jazyk zdá chybný.
settings-group-goals-display = Zobrazování
settings-show-characters = Zobrazovat počet znaků ve stavovém řádku
settings-autosave-hint = Změny se při psaní automaticky zapisují na disk.

settings-sec-work = Projekt
settings-page-structure = Struktura
settings-page-author = Autor
settings-field-author-name = Jméno autora
settings-field-author-placeholder = Nepovinné
settings-field-author-hint = Objeví se na titulní stránce sestavené knihy a v metadatech exportovaných souborů. Necháte-li pole prázdné, vynechá se. Je to autorský údaj knihy. Jméno, kterým podepisujete komentáře, najdete v Nastavení ▸ Uživatel.
settings-group-chapters = Kapitoly
tidy-titles-title = Uklidit názvy kapitol
tidy-titles-none = Žádný název kapitoly ani části pouze neopakuje její vlastní číslo.
tidy-titles-lead = { $count ->
        [one] Jedna kapitola nebo část má za název jen své vlastní číslo.
        [few] { $count } kapitoly a části mají za název jen své vlastní číslo.
       *[other] { $count } kapitol a částí má za název jen své vlastní číslo.
    }
tidy-titles-explain = Po vymazání těchto názvů se každá kapitola bude jmenovat číslem, které kniha už zná, tedy stejným číslem, jaké vytiskne export. Nic dalšího se nezmění a jediné vrácení zpět je všechny obnoví.
menu-document-tidy-titles = &Uklidit názvy kapitol…
settings-group-numbering = Číslování
settings-number-chapters = Číslovat kapitoly a části
settings-number-chapters-tip = Kapitoly a části nesou číslo odvozené z jejich místa v knize, zobrazené zde vedle názvu a vytištěné při exportu.
settings-number-chapters-tip-more = Číslo se nikdy neukládá do názvu, takže zůstane správné, i když položky přeskupíte, vložíte nebo smažete. Když tuto volbu vypnete, export vytiskne jen názvy, ať exportní styl požaduje cokoli. Chcete-li vynechat jednu kapitolu (prolog, mezihru), použijte raději přepínač Číslování v inspektoru: kapitola zůstane v knize, jen nedostane číslo.
settings-part-resets-chapter = Číslovat kapitoly v každé části znovu od jedné
settings-part-resets-chapter-tip = Ve výchozím stavu vypnuto: kapitoly se číslují průběžně napříč částmi knihy, takže „Druhá část“ začíná kapitolou jedenáct.
settings-part-resets-chapter-tip-more = Tak je to v nakladatelské praxi obvyklé a tak to čtenář očekává. Zapněte tuto volbu u knihy, jejíž části se mají číst jako samostatné svazky, z nichž každý začíná kapitolou jedna.
settings-chapter-flat = Ploché kapitoly
settings-chapter-flat-hint = Zapnuto: kapitola je jediná položka. Píšete přímo do ní a neobsahuje žádné scény. Vypnuto: kapitola je složka. Stále do ní můžete psát, ale může obsahovat i scény. Nové kapitoly se řídí tímto nastavením, stávající převedete příkazem Převést na.

settings-styles-builtin = Vestavěné styly
settings-styles-user = Moje styly
settings-styles-builtin-badge = Vestavěný
settings-styles-duplicate = Duplikovat
settings-styles-edit = Upravit
settings-styles-delete = Smazat
settings-styles-import = Importovat…
settings-styles-export = Exportovat…
settings-styles-copy-suffix = (kopie)
settings-styles-json-filter = Exportní styl
settings-styles-editor-title = Upravit styl
settings-styles-editor-none = Vyberte vlastní styl, který chcete upravit, nebo duplikujte některý vestavěný.
settings-styles-imported = Styl byl importován
settings-styles-import-failed = Styl nelze importovat
settings-styles-exported = Styl byl exportován
settings-styles-export-failed = Styl nelze exportovat
settings-styles-field-name = Název
settings-styles-field-chapters = Nadpisy kapitol
settings-styles-field-parts = Nadpisy částí
settings-styles-field-scene-break = Předěl scény
settings-styles-field-major-scene-break = Hlavní předěl scény
settings-styles-field-spacing = Řádkování
settings-styles-field-justify = Zarovnat text do bloku
settings-styles-field-notes = Zahrnout poznámky
settings-styles-field-synopses = Zahrnout synopse
settings-styles-field-scene-titles = Zahrnout názvy scén
settings-styles-field-epigraphs = Zahrnout motta
settings-styles-field-paratexts = Zahrnout paratexty
settings-styles-field-epigraph-placement = Umístění motta
settings-styles-epigraph-after = Za názvem
settings-styles-epigraph-before = Před názvem
settings-styles-field-footnotes = Zahrnout poznámky pod čarou
settings-styles-field-footnote-numbering = Číslování poznámek pod čarou
settings-styles-footnote-numbering-continuous = Průběžné
settings-styles-footnote-numbering-per-chapter = Znovu v každé kapitole
settings-styles-footnote-numbering-per-book = Znovu v každé knize
settings-styles-field-images = Obrázky
menu-image = O&brázek
settings-styles-images-beside = Vedle dokumentu
settings-styles-images-embed = Uvnitř dokumentu
settings-styles-images-omit = Vynechat
settings-styles-group-round-trip = Odeslání redaktorovi
settings-styles-field-comments = Zahrnout komentáře
settings-styles-field-round-trip-marks = Zahrnout značky pro zpětný import
settings-styles-round-trip-hint = Jen DOCX a ODT. Značky jsou neviditelné identifikátory, díky kterým vrácený soubor aktualizuje tento projekt, místo aby se vedle něj objevil jako druhá kopie.
settings-styles-group-pages = Stránky
settings-styles-field-cover = Začínat obálkou
settings-styles-field-word-count = Počet slov na titulní stránce
settings-styles-field-page-books = Nová stránka u každé knihy
settings-styles-field-page-parts = Nová stránka u každé části
settings-styles-field-page-chapters = Nová stránka u každé kapitoly
settings-styles-field-page-paratexts = Nová stránka u každého paratextu
settings-styles-heading-none = Bez nadpisu
settings-styles-heading-numbered = Jen číslo
settings-styles-heading-title = Jen název
settings-styles-heading-both = Číslo + název
settings-styles-break-blank = Prázdný řádek
settings-styles-break-none = Žádný
settings-styles-spacing-single = Jednoduché
settings-styles-spacing-onehalf = 1½
settings-styles-spacing-double = Dvojité

welcome-title = Vítejte v aplikaci Skribisto
welcome-version = Verze { $version }
welcome-update-available = Je k dispozici verze { $version }
welcome-update-tip = Otevře stránku ke stažení ve vašem prohlížeči.
welcome-search = Hledat projekty
welcome-open = Otevřít
welcome-new-work = Nový projekt
welcome-create-from = Vytvořit z…
welcome-recent-works = Nedávné projekty
welcome-empty-recents = Zatím žádné nedávné projekty.
welcome-no-matches = Hledání neodpovídá žádný nedávný projekt.
welcome-learn-soon = Návody a tipy brzy přibudou.
welcome-about-blurb = Skribisto, aplikace pro psaní přepsaná v Rustu a Teksilu.
welcome-tagline = *Klidné místo pro psaní dlouhých textů.*
welcome-github = GitHub
welcome-discord = Discord
nav-works = Projekty
nav-examples = Ukázky
nav-learn = Výuka
nav-about = O aplikaci

synopsis = Synopse
epigraph = Motto
pane-manuscript = Rukopis
corkboard = Nástěnka
overview = Přehled

corkboard-card-count = { $count ->
    [one] { $count } karta
    [few] { $count } karty
   *[other] { $count } karet
}
corkboard-child-count = { $count ->
    [one] { $count } položka
    [few] { $count } položky
   *[other] { $count } položek
}
corkboard-view-nested = Vnořené
corkboard-view-flat = Ploché
corkboard-layout-hint = Vnořené zobrazení ukazuje přímé potomky kontejneru. Kartu složky otevřete, abyste do ní vstoupili. Ploché zobrazení ukazuje najednou všechny scény celého kontejneru.
corkboard-card-size = Velikost karet
corkboard-search-placeholder = Filtrovat karty…
corkboard-empty-title = Zatím tu nic není
corkboard-empty-hint = První položku přidáte tlačítkem „＋ Nový“ nahoře.
corkboard-new = Nový
corkboard-show-card-numbers = Číslovat karty
corkboard-modal-size = Velikost rozbaleného editoru
corkboard-scope-hint = Platí pro všechny otevřené nástěnky ve všech projektech.
corkboard-sort-manuscript = Pořadí v rukopisu
corkboard-sort-title-asc = Název A–Z
corkboard-sort-title-desc = Název Z–A
corkboard-card-number = Karta { $number }

duplicate = Duplikovat
duplicate-n = { $count ->
    [one] Duplikovat { $count } kartu
    [few] Duplikovat { $count } karty
   *[other] Duplikovat { $count } karet
}
set-label-n = { $count ->
    [one] Nastavit popisek { $count } kartě
    [few] Nastavit popisek { $count } kartám
   *[other] Nastavit popisek { $count } kartám
}
move-to-trash-n = { $count ->
    [one] Přesunout { $count } kartu do koše
    [few] Přesunout { $count } karty do koše
   *[other] Přesunout { $count } karet do koše
}
reveal-in-outline = Ukázat v osnově

corkboard-move-to = Přesunout do…
corkboard-move-to-n = { $count ->
    [one] Přesunout { $count } kartu do…
    [few] Přesunout { $count } karty do…
   *[other] Přesunout { $count } karet do…
}
corkboard-move-picker-title = Přesunout do…
corkboard-move-picker-empty = Zatím žádné pořadače. Nejprve nějaký vytvořte.
corkboard-move-picker-cancel = Zrušit
corkboard-move-here = Přesunout sem
corkboard-moved-ok = { $count ->
    [one] { $count } karta přesunuta
    [few] { $count } karty přesunuty
   *[other] { $count } karet přesunuto
}
corkboard-move-into-self = Kontejner nelze přesunout do sebe sama. Zvolte cíl mimo něj.
corkboard-move-failed = Tyto karty tam nelze přesunout.

overview-col-title = Název
overview-col-type = Typ
overview-col-label = Popisek
overview-col-tags = Štítky
overview-col-books = Knihy
overview-col-own-words = Slova
overview-col-total-words = Celkem
overview-row-count = { $count ->
    [one] { $count } položka
    [few] { $count } položky
   *[other] { $count } položek
}
overview-search-placeholder = Filtrovat položky…
overview-expand-all = Rozbalit vše
overview-collapse-all = Sbalit vše
overview-table-label = Obsah
overview-empty-title = Zatím tu nic není
overview-gone-title = Tento kontejner už neexistuje
overview-gone-hint = Byl přesunut do koše. Obnovte jej, nebo tuto kartu zavřete.
overview-empty-hint = První položku přidáte tlačítkem „＋ Nový“ nahoře.
corkboard-grid-label = Karty nástěnky
corkboard-rename-field = Přejmenovat položku
corkboard-expand-synopsis = Rozbalit synopsi
corkboard-synopsis-modal-title = Synopse
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
corkboard-show-word-count = Zobrazovat počet slov na kartách
text-heading = Text
no-content = Tato položka nemá žádný upravitelný obsah.
untitled = Bez názvu
placeholder-title = Název…
placeholder-subtitle = Podtitul…
placeholder-chapter-title = Název kapitoly…
split-editor = Rozdělit editor
close-split-view = Zavřít rozdělené zobrazení
drop-open-here = Otevřít zde
drop-open-to-side = Otevřít vedle

segment-chapter = Kapitola
segment-part = Část
segment-book = Kniha
segment-pace = Tempo
segment-notes = Poznámky
segment-story-bible = Bible příběhu
segment-note-own = Poznámka
segment-note-details = Podrobnosti
segment-note-in-prose = V próze
pace-placeholder = Zde se zobrazí plánovač tempa.
pace-empty-title = Naplánujte tempo této knihy
pace-empty-body = Nastavte cílový počet slov a termín a Skribisto spočítá denní tempo, které vás k cíli dovede.
pace-start-planning = Začít plánovat
pace-section-schedule = Harmonogram
pace-section-progress = Průběh
pace-advancement = Postup
pace-goal = Cílový počet slov
pace-deadline = Termín
pace-active = Tempo aktivní
pace-day-mon = Po
pace-day-tue = Út
pace-day-wed = St
pace-day-thu = Čt
pace-day-fri = Pá
pace-day-sat = So
pace-day-sun = Ne
pace-card-written = napsaných slov
pace-card-of-goal = z cíle
pace-card-rate = slov / psací den
pace-card-days-left = zbývajících psacích dnů
pace-card-streak = dnů v řadě
pace-card-ahead = slov náskok před tempem
pace-card-behind = slov skluz za tempem
pace-charts-empty = Grafy průběhu se zde objeví, jakmile se při uložení zaznamená počet slov.
pace-chart-progression = Napsaná slova vs. cíl
pace-chart-words-per-day = Slova za den
pace-series-actual = Skutečnost
pace-series-target = Cíl
pace-series-words-per-day = Slova/den
pace-daily-target-line = Rovnoměrné tempo: { $count ->
    [one] { $count } slovo/den
    [few] { $count } slova/den
   *[other] { $count } slov/den
}
pace-section-holidays = Volné dny
pace-section-milestones = Milníky
pace-holidays-none = Žádné volné dny. Počítá se každý naplánovaný den.
pace-holiday-label = Název volna
pace-add-holiday = Přidat
pace-remove = Odebrat
pace-milestones-none = Zatím žádné milníky. Nastavte milník u části nebo kapitoly v inspektoru.
full-chapter = Celá kapitola
full-part = Celá část
full-book = Celá kniha
full-synopsis = Celá synopse
rename = Přejmenovat
set-label = Nastavit popisek
insert-scene = Vložit scénu
insert-chapter = Vložit kapitolu
split-scene = Rozdělit scénu
move-up = Posunout nahoru
move-down = Posunout dolů
merge-with-previous = Sloučit s předchozí
move-to-trash = Přesunout do koše
rename-chapter = Přejmenovat kapitolu
add-scene = Přidat novou scénu
add-chapter = Přidat novou kapitolu
new-scene-title = Nová scéna
placeholder-scene-name = Název scény…
menu-cut = Vyjmout
menu-copy = Kopírovat
menu-paste = Vložit
menu-paste-unformatted = Vložit bez formátování
menu-select-all = Vybrat vše
format-bold = Tučné
format-italic = Kurzíva
format-underline = Podtržení
format-strikethrough = Přeškrtnutí

binder = Pořadač
no-work = Není načten žádný projekt
no-recent-works = Žádné nedávné projekty
switcher-open-section = Právě otevřené
switcher-recent-section = Nedávné
switcher-this-window = toto okno
open-project-title = Otevřít projekt
open-project-question = Jak chcete otevřít „{ $title }“?
open-in-new-window = Otevřít v novém okně
open-here = Otevřít zde

binder-all = Všechny pořadače
binder-show-all = Zobrazit všechny pořadače
binder-new = Nový pořadač…
binder-default-name = Nový pořadač
binder-item-count = { $count ->
    [one] { $count } položka
    [few] { $count } položky
   *[other] { $count } položek
}
binder-search-placeholder = Filtrovat osnovu…
binder-search-scope = Hledat ve všech pořadačích
binder-trash-confirm-title = Přesunout pořadač do koše?
binder-trash-confirm-text = „{ $name }“ a všechny jeho položky budou přesunuty do koše.

dialog-rename = Přejmenovat
dialog-new-binder = Nový pořadač
dialog-set-label = Nastavit popisek
dialog-new-scene = Nová scéna
close-work-question = Uložit změny před zavřením projektu?
quit-question = Uložit změny před ukončením?
quit-save-work-question = Uložit před ukončením změny v projektu { $title }?
unsaved-changes = Tento projekt obsahuje neuložené změny.
new-work-unsaved-question = Uložit změny před vytvořením nového projektu?
open-work-unsaved-question = Uložit změny před otevřením jiného projektu?
switch-backup-discard-title = Zahodit změny v této záloze?
switch-backup-discard-text = Změny v záloze do ní nelze uložit. Chcete-li je zachovat, použijte Uložit jako nebo Obnovit, případně je zahoďte a otevřete druhý projekt.
switch-save-failed = Projekt nelze uložit, proto nebyl nahrazen: { $error }
switch-save-not-started = Projekt nelze uložit, proto nebyl nahrazen.
close-save-failed = Projekt nelze uložit, proto nebyl zavřen: { $error }
close-save-not-started = Projekt nelze uložit, proto nebyl zavřen.
save-not-started = Projekt nelze uložit.

could-not-open-work = Nelze otevřít „{ $file }“
could-not-open-work-too-new = „{ $file }“ vyžaduje novější Skribisto
could-not-open-work-too-new-detail = Uloženo ve formátu Skribisto { $written_by }; k otevření je potřeba formát { $requires } nebo novější a tato verze podporuje formáty nejvýše do { $supported }. Chcete-li soubor otevřít, aktualizujte Skribisto.
could-not-open-work-importing = Do „{ $file }“ právě zapisuje import
could-not-open-work-importing-detail = Import právě přepisuje tento projekt a ještě neskončil. Otevřete jej, až import doběhne: kdybyste jej otevřeli teď, uvidíte projekt, který import nahrazuje, a jeho příští uložení by tento projekt zapsalo zpět přes import.
could-not-open-example = Ukázku nelze otevřít: { $error }
could-not-open-link = Nelze otevřít { $url }: { $error }
could-not-create-work = Projekt nelze vytvořit: { $error }
saving-as-file = Ukládání jako { $target }…
saving-as-folder = Ukládání jako { $target }/…
saved-as = Uloženo do { $target }
save-error = Nelze uložit: { $error }
save-flush-failed = Uložení proběhlo, ale tyto dokumenty mu nebylo možné předat a zůstávají neuložené: { $titles } ({ $error })
backup-error = Nelze zálohovat: { $error }
backing-up = Zálohování…
backup-nothing-open = Není otevřen žádný projekt, který by bylo možné zálohovat.
backup-already-running = Zálohování už probíhá.
backup-complete = Zálohování dokončeno (uloženo: { $ok }, již aktuální: { $skipped })
backup-partial = Zálohování dokončeno, uloženo: { $ok }, { $failed ->
    [one] { $failed } cíl selhal
    [few] { $failed } cíle selhaly
   *[other] { $failed } cílů selhalo
}
backup-no-destination-title = Není dostupné žádné umístění záloh
backup-no-destination-text = Žádné z nastavených umístění záloh není dostupné (například může být odpojen externí disk). Připojte jej a zkuste to znovu, nebo skončete bez zálohování.
backup-failed-close-title = Zálohu nelze uložit
backup-failed-close-text = Před zavřením se nepodařilo zapsat žádnou záložní kopii. Selhala všechna umístění (disk mohl být odpojen, nebo je plný či chráněný proti zápisu). Opravte to a zkuste to znovu, nebo skončete bez zálohování.

backup-choice-title = Soubor zálohy
backup-choice-heading = Otevřeli jste záložní kopii
backup-choice-subtitle = Toto je záloha projektu k určitému okamžiku.
backup-choice-subtitle-dated = Záloha pořízena { $date }.
backup-choice-body = Můžete ji otevřít a volně upravovat, ale změny lze zachovat jen příkazem Uložit jako. Původního souboru projektu se nic nedotkne. Nebo můžete tento projekt obnovit přesně do stavu této zálohy.
backup-choice-open = Otevřít zálohu
backup-choice-restore = Obnovit tento projekt do tohoto stavu…
backup-choice-not-a-backup = Ne, otevřít normálně
backup-banner-title = Záložní kopie: změny sem nelze uložit
backup-banner-description = Úpravy zachováte v novém souboru příkazem Uložit jako, nebo příkazem Obnovit nahradíte původní projekt touto zálohou.
backup-banner-restore = Obnovit…
backup-banner-save-as = Uložit jako…
backup-restore-original-missing = Původní projekt, přes který se má obnovovat, nelze najít. Použijte raději Uložit jako a ponechte si tuto zálohu jako nový projekt.
backup-restore-close-elsewhere-title = Projekt je otevřen v jiném okně
backup-restore-close-elsewhere-text = Projekt, který obnovujete, je otevřen v jiném okně. Nejprve jej tam zavřete a pak to zkuste znovu.
backup-restore-focus-window = Přepnout do tohoto okna
backup-restore-confirm-title = Obnovit tuto zálohu?
backup-restore-confirm-text = Aktuální verze projektu se před nahrazením touto zálohou odloží stranou jako bezpečnostní záloha.
backup-restore-confirm-ok = Obnovit
backup-restore-error = Nelze obnovit: { $error }
backup-restored-ok = Projekt byl obnoven.
backup-restored-with-safety = Projekt byl obnoven. Vaše předchozí verze byla uložena do { $path }.
close-backup-discard-title = Zahodit změny v této záloze?
close-backup-discard-text = Změny v záloze do ní nelze uložit. Chcete-li je zachovat, použijte Uložit jako, nebo je zahoďte a zavřete.
quit-backup-discard-title = Zahodit změny a ukončit?
quit-backup-discard-work-question = Zahodit změny v projektu { $title } a ukončit?
quit-backup-discard-text = Změny v záloze do ní nelze uložit. Chcete-li je zachovat, použijte Uložit jako, nebo je zahoďte a ukončete aplikaci.
backup-nudge-text = Pro tento projekt nejsou nastaveny žádné zálohy.
backup-nudge-action = Nastavit zálohy…

menu-backups-list = Zá&lohy…
backups-title = Zálohy
backups-loading = Vyhledávání záloh…
backups-empty = Pro tento projekt zatím nebyly nalezeny žádné zálohy.
backups-open = Otevřít
backups-reveal = Ukázat
backups-delete = Smazat tuto zálohu
backups-delete-confirm-title = Smazat tuto zálohu?
backups-delete-confirm-text = „{ $name }“ bude trvale smazána. Tuto akci nelze vrátit zpět.
backups-delete-error = Zálohu nelze smazat: { $error }
backups-refresh = Aktualizovat
backups-close = Zavřít

settings-page-backup = Výchozí zálohování
settings-page-work-backup = Zálohy
settings-backup-general-title = Výchozí nastavení zálohování
settings-backup-work-title = Zálohy tohoto projektu
settings-backup-inherit = Použít obecné nastavení zálohování
settings-backup-inheriting = Tento projekt používá obecné nastavení zálohování.
settings-backup-none-hint = Nejsou nastaveny žádné automatické zálohy (všechny spouštěče jsou vypnuté).
settings-backup-last = Poslední záloha: { $date }
settings-backup-last-never = Zatím žádné zálohy.
settings-backup-open-list = Otevřít seznam záloh…
settings-backup-triggers = Kdy zálohovat
settings-backup-on-close = Zálohovat při zavření projektu
settings-backup-on-open = Zálohovat při otevření projektu
settings-backup-interval = Zálohovat pravidelně, vždy po
settings-backup-destinations = Umístění záloh
settings-backup-default-location = Výchozí umístění
settings-backup-usage-since = { $count ->
    [one] { $count } záloha · { $size } · nejstarší { $oldest }
    [few] { $count } zálohy · { $size } · nejstarší { $oldest }
   *[other] { $count } záloh · { $size } · nejstarší { $oldest }
}
settings-backup-usage = { $count ->
    [one] { $count } záloha · { $size }
    [few] { $count } zálohy · { $size }
   *[other] { $count } záloh · { $size }
}
settings-backup-usage-empty = Zatím zde nic není uloženo
settings-backup-usage-measuring = Měření…
settings-backup-reveal-root = Zobrazit složku
settings-backup-dest-none = Žádná umístění. Zálohy se ukládají vedle projektu.
settings-backup-dest-remove = Odebrat
settings-backup-dest-add = Přidat složku…
settings-backup-dest-refresh = Aktualizovat
settings-backup-retention = Kolik záloh uchovávat
settings-backup-retention-tiered = Odstupňovaně
settings-backup-retention-keep-n = Posledních N
settings-backup-retention-tip = Jak se staré zálohy probírají.
settings-backup-retention-tip-more = Odstupňovaný režim uchovává jednu zálohu za hodinu po dobu jednoho dne, jednu za den po dobu týdne, jednu za týden po dobu měsíce a dále jednu za měsíc, takže nedávná historie zůstává hustá a starší řídne. Režim „Posledních N“ jednoduše uchovává N nejnovějších kopií. V obou režimech se nejnovější kopie (minimum níže) uchovávají vždy.
settings-backup-gfs-hourly = Po hodinách (posledních 24 h)
settings-backup-gfs-daily = Po dnech (poslední týden)
settings-backup-gfs-weekly = Po týdnech (poslední měsíc)
settings-backup-gfs-monthly = Po měsících
settings-backup-keep-n = Počet uchovávaných
settings-backup-min-keep = Vždy uchovat nejméně
settings-backup-dedup = Přeskočit zálohu, pokud se nic nezměnilo

new-work-title = Nový projekt
new-work-close = Zavřít
new-work-name = Název projektu
new-work-name-placeholder = Bez názvu
new-work-author = Autor
new-work-author-placeholder = Nepovinné
new-work-format = Formát
new-work-single-file = Jeden soubor
new-work-bundle = Složka
new-work-convert-later = Mezi formáty můžete později převádět.
new-work-location = Umístění
new-work-will-create = Vytvoří se
new-work-language = Výchozí jazyk
new-work-language-hint = Použije se pro nové texty a kontrolu pravopisu. Každý text lze přepnout do jiného jazyka.
new-work-template = Šablona
new-work-template-none = Žádná
new-work-template-empty-novel = Prázdný román
new-work-template-light-novel = Light novel
new-work-template-novel = Román
new-work-template-novel-in-parts = Román v částech
new-work-template-notebook = Zápisník
new-work-cancel = Zrušit
new-work-create = Vytvořit projekt
new-work-step-details = Podrobnosti
new-work-step-language = Jazyk a struktura
new-work-step-template = Šablona
new-work-back = Zpět
new-work-next = Další
new-work-documents-title = Nový projekt z dokumentů
new-work-step-import = Import
new-work-create-and-import = Vytvořit a importovat…
new-work-documents-next-title = Vaše dokumenty přijdou na řadu potom
new-work-documents-next-body = Po vytvoření projektu se nad ním otevře průvodce importem, ve kterém vyberete dokumenty, zkontrolujete strukturu, kterou v nich Skribisto rozpoznal, a zvolíte, kam se umístí. Do projektu se nic nezapíše, dokud to tam nepotvrdíte.
new-work-documents-no-template = Tento projekt záměrně začíná prázdný, bez šablony, takže jediným obsahem budou importované dokumenty.
new-work-documents-chapter-scene-hint = Platí pro kapitoly, které import vytvoří. Později to můžete změnit v nastavení projektu.
new-work-single-file-desc = Jeden archiv .skrib (zip). Přenosný a snadno zálohovatelný.
new-work-bundle-desc = Složka se všemi texty a přílohami. Vhodnější pro správu verzí.
new-work-template-none-count = prázdný pořadač
new-work-template-empty-novel-count = 1 kapitola
new-work-template-light-novel-count = 15 kapitol
new-work-template-novel-count = 20 kapitol
new-work-template-novel-in-parts-count = 3 části, 24 kapitol
new-work-template-notebook-count = volné poznámky
new-work-chapter-scene = Ploché kapitoly
new-work-chapter-scene-tip = Každá kapitola je jediná položka, do které píšete přímo, bez scén pod ní. Pro klasické uspořádání, kde je kapitola složkou, nechte volbu vypnutou: i do takové kapitoly píšete přímo, ale může navíc obsahovat scény.
new-work-chapter-scene-tip-more = Do kapitoly píšete v obou případech. Rozdíl je jen v tom, zda může *obsahovat* scény. Strom pořadače v aplikaci Skribisto slouží jen k organizaci, takže obě uspořádání se sestaví do stejné knihy, můžete je volně kombinovat a příkaz Převést na změní kapitolu z jednoho typu na druhý bez ztráty jediného slova.
new-work-name-required = Zadejte název projektu
new-work-name-invalid = Tento název neobsahuje žádné použitelné znaky
new-work-location-required = Zvolte umístění
new-work-location-missing = Tato složka neexistuje
new-work-location-not-folder = Tato cesta není složka
new-work-location-readonly = Do této složky nelze zapisovat
new-work-target-exists = V této složce už je projekt s tímto názvem. Před jeho nahrazením se vytvoření projektu zeptá
new-work-target-is-folder = V tomto umístění už je složka s tímto názvem. Zvolte jiný název
new-work-overwrite-title = Nahradit stávající projekt?
new-work-overwrite-text = „{ $name }“ už v této složce existuje. Nahradit jej novým projektem?
new-work-target-open-title = Tento projekt je otevřený
new-work-target-open-text = „{ $name }“ je otevřen v aplikaci Skribisto. Vytvořením nového projektu na jeho místě byste nahradili projekt, na kterém pracujete, a jeho příští uložení by tento projekt zapsalo zpět přes nový. Zvolte jiný název nebo složku, nebo projekt nejprve zavřete.
target-importing-title = Do tohoto projektu zapisuje import
target-importing-text = Do „{ $name }“ ještě zapisuje import. Počkejte, až skončí, nebo zvolte jiný název.
backup-restore-importing-text = Do „{ $name }“, projektu, přes který by se tato záloha obnovila, ještě zapisuje import. Zálohu obnovte, až import skončí.

new-work-manuscript = Rukopis
new-work-notes = Poznámky
new-work-research = Rešerše
new-work-notebook = Zápisník
new-work-scene = Scéna
new-work-note = Poznámka
new-work-front-matter = Úvodní část
new-work-back-matter = Závěrečná část
new-work-paratext = Struktura knihy
new-work-paratext-none = Bez struktury
new-work-paratext-hint = Úvodní a závěrečné části, kterými určitá tradice knihu otevírá a uzavírá. Kteroukoli z nich můžete později přesunout, přejmenovat nebo smazat.
new-work-tags = Štítky
new-work-tags-none = Bez štítků
new-work-tags-hint = Výchozí paleta štítků pro postavy, místa a další. Je nepovinná a každý štítek lze později přejmenovat, přebarvit nebo smazat.
new-work-note-templates = Šablony poznámek
new-work-note-templates-none = Bez šablon
new-work-note-templates-hint = Podoba, se kterou začíná poznámka bible příběhu. Je nepovinná a každou šablonu lze později upravit nebo smazat.
new-work-characters = Postavy
new-work-places = Místa

import-plume-title = Importovat projekt Plume Creator
import-plume-source = Projekt Plume
import-plume-source-hint = Vyberte soubor .plume nebo .plume_backup (z libovolné verze Plume Creator).
import-plume-trash-warning = ⚠ Položky v koši a smazané položky se nepřevádějí.
import-plume-source-required = Vyberte soubor projektu Plume
import-plume-source-missing = Tento soubor neexistuje
import-plume-source-not-file = Tato cesta není soubor
import-plume-manuscript-binder = Rukopis
import-plume-story-bible-binder = Bible příběhu
import-plume-progress-title = Importování projektu Plume…
import-plume-done = { $imported ->
    [one] Importována { $imported } položka.
    [few] Importovány { $imported } položky.
   *[other] Importováno { $imported } položek.
} { $skipped ->
    [one] { $skipped } položka z koše nebyla převedena.
    [few] { $skipped } položky z koše nebyly převedeny.
   *[other] { $skipped } položek z koše nebylo převedeno.
}
import-plume-nested-too-deep = Projekt nebyl importován: jeho část „{ $part }“ je vnořená hlouběji než do { $limit } úrovní. Žádný projekt Plume Creator se tomu ani neblíží a jeho čtení by mohlo Skribisto shodit. Nic nebylo zapsáno.
import-plume-declares-entities = Projekt nebyl importován: jeho část „{ $part }“ deklaruje entity XML, což žádný projekt Plume Creator nedělá. Jejich rozvinutí by mohlo zabrat víc paměti, než počítač má, a Skribisto shodit. Nic nebylo zapsáno.

import-manuskript-title = Importovat projekt Manuskript
import-manuskript-source = Projekt Manuskript
import-manuskript-source-hint = Vyberte soubor .msk nebo složku projektu (z libovolné verze Manuskriptu).
import-manuskript-source-file = Vybrat soubor…
import-manuskript-source-folder = Vybrat složku…
import-manuskript-source-required = Vyberte projekt Manuskript
import-manuskript-source-missing = Tento soubor nebo složka neexistuje
import-manuskript-manuscript-binder = Rukopis
import-manuskript-story-bible-binder = Bible příběhu
import-manuskript-characters-group = Postavy
import-manuskript-world-group = Svět
import-manuskript-plots-group = Dějové linie
import-manuskript-project-info-note = Informace o projektu
import-manuskript-summary-note = Shrnutí
import-manuskript-importance-minor = Vedlejší
import-manuskript-importance-secondary = Druhořadá
import-manuskript-importance-main = Hlavní
import-manuskript-progress-title = Importování projektu Manuskript…
import-manuskript-done = { $imported ->
    [one] Importována { $imported } položka.
    [few] Importovány { $imported } položky.
   *[other] Importováno { $imported } položek.
} { $revisions ->
    [0] { "" }
    [one] Přenesla se i { $revisions } starší verze.
    [few] Přenesly se i { $revisions } starší verze.
   *[other] Přeneslo se i { $revisions } starších verzí.
}
import-manuskript-nested-too-deep = Projekt nebyl importován: jeho soubor „{ $part }“ je vnořený hlouběji než do { $limit } úrovní. Žádný projekt Manuskript se tomu ani neblíží a jeho čtení by mohlo Skribisto shodit. Nic nebylo zapsáno.
import-manuskript-folders-too-deep = Projekt nebyl importován: jeden z jeho souborů leží hlouběji než { $limit } složek. Žádný projekt Manuskript se tomu ani neblíží a jeho čtení by mohlo Skribisto shodit. Nic nebylo zapsáno.
import-manuskript-declares-entities = Projekt nebyl importován: jeho soubor „{ $part }“ deklaruje entity XML, což žádný projekt Manuskript nedělá. Jejich rozvinutí by mohlo zabrat víc paměti, než počítač má, a Skribisto shodit. Nic nebylo zapsáno.

import-target-open-title = Tento projekt je otevřený
import-target-open-text = „{ $name }“ je otevřen v aplikaci Skribisto. Import přes něj by nahradil projekt, na kterém pracujete, a jeho příští uložení by tento projekt zapsalo zpět přes import. Zvolte jiný název souboru, nebo projekt nejprve zavřete.
import-warnings-announcement = { $result } { $notice }.

export-title = Export
export-close = Zavřít
export-scope-label = Co
export-format-label = Formát
export-style-label = Styl
export-path-label = Soubor
export-browse = Procházet…
export-show-non-exportable = Zobrazit i neexportované
export-choose-empty = Není z čeho vybírat.
export-cancel = Zrušit
export-export = Exportovat
export-save-dialog-title = Exportovat do souboru
export-section-what = Co exportovat
export-section-style = Styl
export-section-destination = Umístění
export-custom-selection = Vlastní výběr
export-selected-count = Vybráno: { $count }
export-chip-chapters-none = Kapitoly: bez nadpisu
export-chip-chapters-numbered = Kapitoly: číslované
export-chip-chapters-title = Kapitoly: jen název
export-chip-chapters-both = Kapitoly: číslo + název
export-chip-scene-break-glyph = Předěl scény: { $glyph }
export-chip-scene-break-blank = Předěl scény: prázdný řádek
export-chip-scene-break-none = Předěl scény: žádný
export-chip-major-break-glyph = Hlavní předěl: { $glyph }
export-chip-major-break-blank = Hlavní předěl: prázdný řádek
export-chip-major-break-none = Hlavní předěl: žádný
export-chip-major-break-same = Obě úrovně předělů stejné
export-chip-spacing-single = Řádkování: jednoduché
export-chip-spacing-onehalf = Řádkování: 1½
export-chip-spacing-double = Řádkování: dvojité
export-chip-notes-included = Včetně poznámek
export-chip-notes-excluded = Bez poznámek
export-format-docx = Word
export-format-odt = LibreOffice
export-format-html = HTML
export-format-markdown = Markdown
export-format-djot = Djot
export-format-text = Text
export-format-latex = LaTeX
export-format-epub = EPUB
export-format-pdf = PDF
export-overwrite-title = Nahradit stávající soubor?
export-overwrite-text = „{ $name }“ už existuje. Nahradit jej?
export-progress-title = Exportování…
export-cancelled = Export byl zrušen
export-done = { $count ->
    [one] Exportována { $count } položka
    [few] Exportovány { $count } položky
   *[other] Exportováno { $count } položek
}
export-open-file = Otevřít
export-show-in-folder = Zobrazit ve složce
export-error-title = Nelze exportovat
export-error-details = Podrobnosti

backup-progress-start = Spouštění…
backup-progress-retention = Probírání starých záloh…
backup-progress-done = Hotovo
backup-progress-destination = Umístění { $i } z { $n }
backup-details = Podrobnosti
backup-issues-title = Potíže se zálohováním
backup-failed-title = Zálohování selhalo
backup-complete-prune-warning = Zálohování dokončeno (uloženo: { $ok }, již aktuální: { $skipped }). Některé staré zálohy nebylo možné odstranit

search = Hledat
search-query-placeholder = Hledat…
search-replace-placeholder = Nahradit za…
search-replace-toggle = Zobrazit nebo skrýt nahrazování
search-replace-all = Nahradit vše
search-opt-case = Rozlišovat velikost písmen
search-opt-whole-word = Celá slova
search-opt-diacritics = Rozlišovat diakritiku
search-scope-body = Text
search-scope-title = Název
search-scope-synopsis = Synopse
search-scope-label = Popisek
search-facet-book = Knihy
search-facet-part = Části
search-facet-chapter = Kapitoly
search-facet-scene = Scény
search-facet-note = Poznámky
search-facet-folder = Složky
search-tip-case = Rozlišovat velikost písmen: velká a malá písmena se berou jako různá, takže „Elena“ a „elena“ jsou samostatné shody.
search-tip-whole-word = Celá slova: hledají se jen celá slova, takže „kos“ se nenajde uvnitř slova „kosmos“.
search-tip-diacritics = Rozlišovat diakritiku: písmena s diakritikou se berou jako odlišná, takže „cafe“ neodpovídá „café“.
search-tip-body = Text: hledá v próze scén a poznámek.
search-tip-title = Název: hledá v názvech položek pořadače.
search-tip-synopsis = Synopse: hledá ve shrnutí každé psací položky.
search-tip-label = Popisek: hledá v krátké poznámce zobrazené pod názvem položky pořadače.
search-tip-comment = Komentáře: hledá v textu vláken komentářů a odpovědí na ně. Komentář se týká rukopisu, ale není jeho součástí, proto má vlastní přepínač a nahrazování nechává shody v komentářích nezaškrtnuté, dokud je nezaškrtnete.
search-tip-book = Knihy: kontejner knihy a jeho značky začátku a konce.
search-tip-part = Části: oddělovače na úrovni částí.
search-tip-chapter = Kapitoly: kapitoly, ať je projekt ukládá jakkoli.
search-tip-scene = Scény: položky, které obsahují vaši prózu.
search-tip-note = Poznámky: volné poznámky.
search-tip-folder = Složky: obyčejné organizační složky a oddělovače.
search-tip-paratext = Text, který patří ke knize, ale ne k jejímu příběhu: předmluva, věnování, doslov. Do rukopisu se nikdy nezapočítává.
search-tip-preserve-case = Zachovat velikost písmen: náhrada převezme velikost písmen nalezeného textu, takže z „ELENA“ se stane „MARTA“ a z „Elena“ „Marta“.
search-tip-replace = Nahradit: zobrazí pole pro náhradu a tlačítko Nahradit vše.
search-error = Hledání selhalo: { $message }
search-no-matches = Žádné shody
search-count =
    { $matches ->
        [one] { $matches } shoda
        [few] { $matches } shody
       *[other] { $matches } shod
    } v { $items ->
        [one] { $items } dokumentu
        [few] { $items } dokumentech
       *[other] { $items } dokumentech
    }
search-count-truncated =
    { $matches ->
        [one] { $matches } shoda
        [few] { $matches } shody
       *[other] { $matches } shod
    } v { $items ->
        [one] { $items } dokumentu
        [few] { $items } dokumentech
       *[other] { $items } dokumentech
    } (jen první výsledky)
search-occurrences = { $count }
search-field-body = Text
search-field-title = Název
search-field-synopsis = Synopse
search-field-label = Popisek
search-field-epigraph = Motto
search-field-comment = Komentář
search-field-comment-reply = Odpověď
search-field-footnote = Poznámka pod čarou
search-include-in-replace = Zahrnout do Nahradit vše
search-collapse-all = Sbalit všechny výsledky
search-replace-here = Nahradit zde
search-dismiss = Odebrat z výsledků
search-undo-dismiss = Vrátit poslední odebraný výsledek
search-replace-nothing = (nic)
search-replace-confirm-title = Nahradit všechny shody?
search-replace-confirm-text =
    Nahradit { $occurrences ->
        [one] { $occurrences } výskyt
        [few] { $occurrences } výskyty
       *[other] { $occurrences } výskytů
    } „{ $query }“ za „{ $replacement }“ v { $items ->
        [one] { $items } dokumentu
        [few] { $items } dokumentech
       *[other] { $items } dokumentech
    }? Potom to můžete vrátit zpět z oznámení.
search-replace-done-title = Nahrazování dokončeno
search-replace-done =
    { $occurrences ->
        [one] Nahrazen { $occurrences } výskyt
        [few] Nahrazeny { $occurrences } výskyty
       *[other] Nahrazeno { $occurrences } výskytů
    } v { $items ->
        [one] { $items } dokumentu
        [few] { $items } dokumentech
       *[other] { $items } dokumentech
    }.
search-replace-done-skipped =
    { $occurrences ->
        [one] Nahrazen { $occurrences } výskyt
        [few] Nahrazeny { $occurrences } výskyty
       *[other] Nahrazeno { $occurrences } výskytů
    } v { $items ->
        [one] { $items } dokumentu
        [few] { $items } dokumentech
       *[other] { $items } dokumentech
    }. { $skipped ->
        [one] { $skipped } pole přeskočeno
        [few] { $skipped } pole přeskočena
       *[other] { $skipped } polí přeskočeno
    } (od hledání se změnila).
search-replace-undo = Vrátit zpět
search-replace-failed-title = Nahrazení selhalo
search-replace-skipped-title = Nic nebylo nahrazeno
search-replace-skipped-body =
    { $fields ->
        [one] Tento text se od hledání změnil, a proto zůstal beze změny.
        [few] { $fields } z těchto textů se od hledání změnily, a proto zůstaly beze změny.
       *[other] { $fields } z těchto textů se od hledání změnilo, a proto zůstaly beze změny.
    } Hledejte znovu, abyste viděli, kde slova jsou teď.
search-replace-undo-failed-title = Vrácení zpět selhalo
search-preview = Náhled
search-preview-empty = Vyberte výsledek a zobrazí se zde jeho náhled
search-preview-no-prose = Tato shoda nemá upravitelný text
search-preview-prompt = Chcete-li zde vidět náhled, spusťte hledání.
search-preview-open-search = Hledat v projektu
search-preview-footnote-prompt = Tato shoda je ve vlastním textu poznámky pod čarou. Chcete-li ji vidět a upravit, otevřete ji v panelu Poznámky pod čarou.
search-preview-open-footnotes = Otevřít poznámky pod čarou

find-placeholder = Hledat v tomto dokumentu…
find-count = { $current } z { $total }
find-no-results = Žádné výsledky
find-close = Zavřít hledání
find-previous = Předchozí shoda (Shift+Enter)
find-next = Další shoda (Enter)
find-opt-case = Rozlišovat velikost písmen
find-opt-whole-word = Celá slova
find-replace-toggle = Zobrazit nebo skrýt nahrazování (Ctrl+R)
find-replace-placeholder = Nahradit za…
find-replace = Nahradit
find-replace-all = Nahradit vše
find-preserve-case = Zachovat velikost písmen

dict-download-title = Stahování: { $name }…
dict-download-done = Nainstalováno: { $name }
dict-download-failed = Nelze stáhnout { $name }: { $error }
dict-removed = Odebráno: { $name }
dict-accept-first = Před stažením slovníku { $name } přijměte licenci
settings-dict-tab-installed = Nainstalované
settings-dict-tab-get-more = Získat další
settings-dict-tab-personal = Osobní slova
dict-installed-empty = V tomto počítači zatím nebyly nalezeny žádné slovníky.
dict-get-more-search = Hledat jazyky
dict-system-badge = v systému
dict-unusable-badge = nepoužitelný
dict-download-button = Stáhnout
dict-downloading = Stahování…
dict-installed-label = Nainstalováno
dict-remove = Odebrat
dict-view-license = Zobrazit licenci
dict-approx-size = ~{ $size }
dict-personal-empty = V tomto projektu zatím nejsou žádná osobní slova.
dict-personal-add = Přidat
dict-personal-placeholder = Přidat slovo…
dict-missing-toast = { $count ->
    [one] Tento projekt používá { $count } slovník, který nemáte nainstalovaný
    [few] Tento projekt používá { $count } slovníky, které nemáte nainstalované
   *[other] Tento projekt používá { $count } slovníků, které nemáte nainstalované
}
dict-missing-action = Získat slovníky
dict-license-title = Licence slovníku { $name }
dict-license-accept = Přijmout a stáhnout
dict-license-cancel = Zrušit
dict-license-close = Zavřít
dict-add-button = Přidat slovník…
dict-add-title = Přidání slovníku
dict-add-close = Zavřít
dict-add-name = Název
dict-add-name-placeholder = např. Můj latinský slovník
dict-add-code = Kód jazyka
dict-add-code-placeholder = např. la nebo fr-FR-x-custom
dict-add-code-hint = Krátké označení podle vašeho výběru. Právě to pak zvolíte jako jazyk dokumentu.
dict-add-aff = Soubor afixů (.aff)
dict-add-dic = Seznam slov (.dic)
dict-add-submit = Přidat
dict-add-cancel = Zrušit
dict-add-name-required = Zadejte název slovníku.
dict-add-code-required = Zadejte kód jazyka.
dict-add-code-invalid = Používejte jen písmena, číslice a znaky - _ .
dict-add-code-reserved = Toto je kód vestavěného slovníku. Zvolte jiný.
dict-add-file-required = Vyberte soubor.
dict-add-file-missing = Tento soubor neexistuje.
dict-add-code-taken = Slovník s tímto kódem už je nainstalován.
dict-add-done = Přidáno: { $name }
dict-add-unusable = Tyto soubory netvoří použitelný slovník: { $error }
dict-add-failed = Slovník nelze přidat: { $error }
inspector-export = Export
inspector-exportable = Zahrnout do exportu
ctx-number = Čí&slovat tuto kapitolu
ctx-unnumber = Neči&slovat tuto kapitolu
inspector-numbering = Číslování
inspector-numbered = Číslovaná
inspector-numbered-tip = Tato kapitola má své místo v číslování knihy. U prologu, epilogu nebo mezihry ji vypněte.
inspector-numbered-tip-more = Nečíslovaná kapitola zůstává v knize přesně taková, jaká byla: její nadpis, próza i počet slov zůstávají beze změny. Jen se u ní nevytiskne číslo a žádné ani nespotřebuje: kapitola po prologu je kapitola jedna, ne dvě. Vynechání z exportu je jiný přepínač, výše, a ten kapitolu z knihy odstraní úplně.
inspector-apply-to-children = Použít na podřízené položky
inspector-milestone = Datum milníku
inspector-milestone-none = Bez data
inspector-milestone-clear = Vymazat
inspector-tags = Štítky
inspector-aliases = Také známo jako
inspector-dict-language = Jazyk
inspector-apply-language-to-children = Použít jazyk na podřízené položky
settings-page-language = Jazyk
settings-field-dict-language = Jazyky
dict-tradeoff-hint = Každý další jazyk přijímá víc slov, takže se zachytí méně chyb.
lang-inherit-hint = Zděděno: tato scéna používá jazyky knihy nebo projektu.
lang-pill-list = Jazyky
lang-pill-add = Přidat jazyk
lang-pill-remove = Odebrat { $name }
lang-pill-mute = Vypnout kontrolu pravopisu pro { $name }
lang-pill-unmute = Zapnout kontrolu pravopisu pro { $name }

settings-page-personal-dictionary = Osobní slovník
settings-user-dict-desc = Slova, která sem přidáte, se v celé aplikaci považují za správně napsaná a už nikdy nebudou označena.
settings-user-dict-add-placeholder = Přidat slovo…
settings-user-dict-add = Přidat slovo
settings-user-dict-search = Filtrovat slova
settings-user-dict-count = { $count ->
    [one] { $count } slovo
    [few] { $count } slova
   *[other] { $count } slov
}
settings-user-dict-rename = Přejmenovat
settings-user-dict-remove = Odebrat
settings-user-dict-empty = Zatím žádná slova. Přidejte slova, která má kontrola pravopisu přehlížet, nebo importujte seznam.
settings-user-dict-import = Importovat…
settings-user-dict-export = Exportovat…
settings-user-dict-import-tip = Přidá slova z prostého textového souboru, jedno slovo na řádek. Stávající slova zůstanou zachována.
settings-user-dict-export-tip = Uloží celý seznam do prostého textového souboru, jedno slovo na řádek.
settings-user-dict-txt-filter = Textové soubory
settings-user-dict-duplicate = Už je ve slovníku
settings-user-dict-imported = { $count ->
    [one] Importováno { $count } slovo
    [few] Importována { $count } slova
   *[other] Importováno { $count } slov
} (již existujících: { $duplicates }).
settings-user-dict-import-failed = Seznam slov nelze přečíst: { $error }
settings-user-dict-exported = { $count ->
    [one] Uloženo { $count } slovo.
    [few] Uložena { $count } slova.
   *[other] Uloženo { $count } slov.
}
settings-user-dict-export-failed = Seznam slov nelze uložit: { $error }

settings-page-text-replacements = Nahrazování textu
settings-text-repl-desc = Při psaní nahrazuje zkratky plným textem: „mj“ se změní na „mimo jiné“ ve chvíli, kdy napíšete mezeru nebo interpunkci.
settings-text-repl-enable = Používat nahrazování textu v tomto projektu
settings-text-repl-disabled-hint = Zapněte tuto volbu, chcete-li definovat zkratky, které se při psaní rozvinou.
settings-text-repl-add = Přidat pravidlo
settings-text-repl-trigger-placeholder = Zkratka
settings-text-repl-replacement-placeholder = Čím se nahradí
settings-text-repl-added = Přidáno „{ $trigger }“
settings-text-repl-duplicate = „{ $trigger }“ už pravidlo má
settings-text-repl-filter = Filtrovat pravidla
settings-text-repl-count = { $n ->
    [one] 1 pravidlo
    [few] { $n } pravidla
   *[other] { $n } pravidel
}
settings-text-repl-row-enabled = Používat toto pravidlo
settings-text-repl-delete = Smazat pravidlo pro { $trigger }
settings-text-repl-deleted = Pravidlo pro „{ $trigger }“ bylo smazáno
settings-text-repl-empty = Zatím žádná pravidla.
settings-text-repl-csv-filter = Soubory CSV
settings-text-repl-import = Importovat…
settings-text-repl-export = Exportovat…
settings-text-repl-imported = Importováno: { $added }, přeskočeno: { $skipped }
settings-text-repl-exported = { $n ->
    [one] Exportováno 1 pravidlo
    [few] Exportována { $n } pravidla
   *[other] Exportováno { $n } pravidel
}

editor-menu-no-suggestions = Žádné návrhy
editor-menu-add-to-dictionary = Přidat „{ $word }“ do slovníku
editor-menu-add-words-to-dictionary = Přidat vybraná slova do slovníku
editor-dict-added = Slovo „{ $word }“ bylo přidáno do vašeho slovníku.
editor-dict-added-multi = { $count ->
    [one] Do vašeho slovníku bylo přidáno { $count } slovo.
    [few] Do vašeho slovníku byla přidána { $count } slova.
   *[other] Do vašeho slovníku bylo přidáno { $count } slov.
}
toast-undo = Vrátit zpět

undo-superseded-title = Tento krok už není poslední
undo-superseded-body = Od té doby se v projektu změnilo něco jiného. Vrácení zpět by místo toho vzalo zpět tuto změnu, a proto neproběhlo. Nic z toho, co tento krok udělal, nebylo vráceno. Chcete-li historií procházet sami, použijte Úpravy ▸ Zpět.
undo-failed = Vrácení zpět selhalo: { $error }

menu-edit = Úpr&avy
menu-edit-undo = &Zpět
menu-edit-redo = Znov&u
menu-edit-undo-target = &Zpět: { $target }
menu-edit-redo-target = Znov&u: { $target }
shortcut-name-edit-undo = Zpět
shortcut-name-edit-redo = Znovu
undo-target-typing = psaní
undo-target-project = poslední změna tohoto projektu
prose-history-reset-title = Historie psaní byla vynulována
prose-history-reset-body = { $count ->
    [one] Jedna otevřená scéna byla obnovena z dřívějšího stavu, takže její historie psaní už neplatí.
    [few] { $count } otevřené scény byly obnoveny z dřívějšího stavu, takže jejich historie psaní už neplatí.
   *[other] { $count } otevřených scén bylo obnoveno z dřívějšího stavu, takže jejich historie psaní už neplatí.
}
undo-target-trash = přesun do koše
undo-target-restore = obnovení z koše
undo-target-delete-forever = trvalé smazání
undo-target-replace-all = nahrazení v celém projektu
undo-target-import = import dokumentu
undo-target-duplicate = duplikace
undo-target-move = přesun
undo-target-merge = sloučení dvou scén
undo-target-split = rozdělení scény
undo-target-promote = změna typu
undo-target-tidy-titles = úklid názvů kapitol
undo-target-import-tags = import štítků
undo-target-import-templates = import šablon poznámek
undo-target-create = vytvoření
undo-target-remove = smazání
undo-target-rename = přejmenování
undo-target-edit = poslední úprava
menu-edit-find = &Najít…
menu-edit-find-next = Najít &další
menu-edit-find-prev = Najít &předchozí
menu-edit-replace = Najít a nah&radit…
menu-edit-find-in-project = Najít v pro&jektu…
menu-edit-replace-in-project = Nahradit v projek&tu…
undo-frozen = Zpět (je zapnuto „Vždy vpřed“)
redo-frozen = Znovu (je zapnuto „Vždy vpřed“)
menu-edit-cut = &Vyjmout
menu-edit-copy = &Kopírovat
menu-edit-paste = V&ložit
menu-edit-paste-plain = Vložit &bez formátování
menu-edit-select-all = Vybr&at vše
shortcut-name-edit-cut = Vyjmout
shortcut-name-edit-copy = Kopírovat
shortcut-name-edit-paste = Vložit
shortcut-name-edit-paste-plain = Vložit bez formátování
shortcut-name-edit-select-all = Vybrat vše

titlebar-spellcheck-on = Kontrola pravopisu je zapnutá. Kliknutím ji vypnete (F7)
titlebar-spellcheck-off = Kontrola pravopisu je vypnutá. Kliknutím ji znovu zapnete (F7)
menu-spellcheck = &Kontrolovat pravopis
menu-comments = Ko&mentáře
settings-page-spellcheck = Kontrola pravopisu
settings-group-spellcheck = Kontrola pravopisu
settings-spellcheck-enabled = Kontrolovat pravopis při psaní
settings-spellcheck-hint = Podtrhává slova, která nezná žádný nainstalovaný slovník. Vypnutím zastavíte veškerou kontrolu ve všech projektech, dokud ji znovu nezapnete. Chcete-li přestat kontrolovat jen jeden jazyk, zrušte jeho zaškrtnutí v poli Jazyk u projektu nebo položky.

menu-trash = &Koš
trash-title = Koš
trash-empty-state = Koš je prázdný.
trash-empty-button = Vyprázdnit koš…
trash-restore = &Obnovit
trash-restore-to = Obnovit &do…
trash-delete-forever = &Smazat natrvalo
trash-restored-ok = Obnoveno: { $count }.
trash-restore-error = Nelze obnovit: { $error }
trash-restore-orphaned = Původní místo této položky už neexistuje. Zvolte, kam ji obnovit.
trash-restore-no-project = Není otevřen žádný projekt, takže není co obnovit.
trash-delete-no-project = Není otevřen žádný projekt, takže není co smazat.
trash-empty-confirm-title = Vyprázdnit koš?
trash-empty-confirm-text = { $count ->
    [one] { $count } položka z koše bude trvale smazána.
    [few] { $count } položky z koše budou trvale smazány.
   *[other] { $count } položek z koše bude trvale smazáno.
} Koš tím natrvalo opustí. Dokud neuděláte nic jiného, můžete je ještě vrátit příkazem Zpět.
trash-emptied-title = Koš byl vyprázdněn
trash-emptied-body = Vše v koši bylo trvale smazáno.
trash-empty-no-project = Není otevřen žádný projekt, takže není co vyprázdnit.
trash-delete-forever-confirm-title = Smazat natrvalo?
trash-delete-forever-confirm-text = { $count ->
    [one] { $count } položka bude trvale smazána.
    [few] { $count } položky budou trvale smazány.
   *[other] { $count } položek bude trvale smazáno.
} Koš tím natrvalo opustí. Dokud neuděláte nic jiného, můžete je ještě vrátit příkazem Zpět.
trash-deleted-title = Smazáno natrvalo
trash-deleted-body = Trvale smazáno: { $count }.
trash-undo = Vrátit zpět
trash-restore-picker-title = Obnovit do…
trash-restore-to-confirm-title = Obnovit sem?
trash-restore-to-confirm-text = Obnovit „{ $item }“ do „{ $destination }“?
trash-restore-picker-restore-here = Obnovit sem
trash-restore-picker-cancel = Zrušit
trash-restore-picker-empty = Zatím žádné pořadače. Nejprve nějaký vytvořte.
trash-banner-title = Tato položka je v koši
trash-banner-description = Dokud ji neobnovíte, nezobrazí se v osnově ani v exportech.
trash-banner-restore = Obnovit…
trash-tab-tooltip = V koši

settings-styles-sheet-yes = Ano
settings-styles-sheet-no = Ne
settings-styles-sheet-font = Písmo
settings-styles-sheet-indent = Odsazení prvního řádku
settings-styles-sheet-para-spacing = Mezery mezi odstavci
settings-styles-sheet-page = Velikost stránky
settings-styles-sheet-margins = Okraje (H/P/D/L)
settings-styles-sheet-title-page = Titulní stránka knihy
settings-styles-sheet-heading-language = Jazyk nadpisů
settings-styles-sheet-auto = Podle textu
settings-styles-sheet-digits = Číslice
settings-styles-sheet-direction = Směr textu
settings-styles-sheet-formats = Formáty
settings-styles-sheet-all-formats = Všechny

settings-styles-editor-group = Editor stylu
settings-styles-editor-missing = Tento styl už není k dispozici.

settings-styles-page-letter = Letter
settings-styles-digits-western = Západní (0–9)
settings-styles-digits-eastern-arabic = Východoarabské (٠–٩)
settings-styles-direction-ltr = Zleva doprava
settings-styles-direction-rtl = Zprava doleva

format-dock-title = Formát
format-panel-empty = Umístěte kurzor do scény, poznámky nebo synopse a zobrazí se volby formátování.
format-group-marks = Text
format-group-block = Odstavec
format-group-lists = Seznamy
format-group-tables = Tabulka
format-group-breaks = Předěly scén
format-superscript = Horní index
format-subscript = Dolní index
format-link = Odkaz…
link-dialog-insert-title = Vložit odkaz
link-dialog-edit-title = Upravit odkaz
link-dialog-text-label = Text
link-dialog-text-placeholder = slova, která čtenář uvidí
link-dialog-url-label = Odkazuje na
link-dialog-url-placeholder = example.com
link-dialog-insert = Vložit
link-dialog-apply = Použít
link-dialog-cancel = Zrušit
link-dialog-remove = Odebrat odkaz
link-scheme-refused = Otevřít lze jen webové a e-mailové odkazy. Odkaz { $url } zůstal beze změny.
menu-format-link = &Odkaz…
format-clear = Vymazat formátování
format-heading = Úroveň nadpisu
format-heading-normal = Normální text
format-heading-1 = Nadpis 1
format-heading-2 = Nadpis 2
format-heading-3 = Nadpis 3
format-heading-4 = Nadpis 4
format-heading-5 = Nadpis 5
format-heading-6 = Nadpis 6
format-align-left = Zarovnat doleva
format-align-center = Na střed
format-direction-rtl = Odstavec zprava doleva
format-blockquote = Bloková citace
format-list-bullet = Odrážkový seznam
format-list-numbered = Číslovaný seznam
format-indent = Zvětšit odsazení
format-outdent = Zmenšit odsazení
format-table-insert = Vložit tabulku
format-table-row-above = Vložit řádek nad
format-table-row-below = Vložit řádek pod
format-table-col-before = Vložit sloupec před
format-table-col-after = Vložit sloupec za
format-table-row-delete = Smazat řádek
format-table-col-delete = Smazat sloupec
format-table-remove = Odebrat tabulku

menu-format-marks-bold = &Tučné
menu-format-marks-italic = &Kurzíva
menu-format-marks-underline = &Podtržení
menu-format-marks-strike = Př&eškrtnutí
menu-format-marks-superscript = &Horní index
menu-format-marks-subscript = &Dolní index
menu-format-marks-clear = &Vymazat formátování
menu-format-heading = &Nadpis
menu-format-heading-normal = &Normální text
menu-format-heading-1 = Nadpis &1
menu-format-heading-2 = Nadpis &2
menu-format-heading-3 = Nadpis &3
menu-format-heading-4 = Nadpis &4
menu-format-heading-5 = Nadpis &5
menu-format-heading-6 = Nadpis &6
menu-format-alignment = &Zarovnání
menu-format-align-left = Zarovnat do&leva
menu-format-align-center = Na &střed
menu-format-direction = Směr te&xtu
menu-format-direction-auto = &Automaticky
menu-format-direction-ltr = Z&leva doprava
menu-format-direction-rtl = Z&prava doleva
menu-format-blockquote = &Bloková citace
menu-format-lists = Sezna&my
menu-format-list-bullet = &Odrážkový seznam
menu-format-list-numbered = Čí&slovaný seznam
menu-format-indent = &Zvětšit odsazení
menu-format-outdent = Z&menšit odsazení
menu-format-table = T&abulka
menu-format-table-insert = &Vložit tabulku
menu-format-table-2x2 = &2 × 2
menu-format-table-3x3 = &3 × 3
menu-format-table-4x4 = &4 × 4
menu-format-table-row-above = Vložit řádek &nad
menu-format-table-row-below = Vložit řádek &pod
menu-format-table-col-before = Vložit sloupec př&ed
menu-format-table-col-after = Vložit sloupec &za
menu-format-table-row-delete = Smazat řá&dek
menu-format-table-col-delete = Smazat &sloupec
menu-format-table-remove = &Odebrat tabulku

menu-format-typo-marks = &Formátovací značka
menu-format-typo-nbsp = &Nezlomitelná mezera
menu-format-typo-nbhyphen = Nezlomitelný &spojovník
menu-format-typo-soft-hyphen = &Měkký spojovník
menu-format-typo-nnbsp = Ú&zká nezlomitelná mezera
menu-format-typo-zwsp = Mezera nu&lové šířky
menu-format-typo-word-joiner = Spo&jovač slov


menu-about = &O aplikaci Skribisto…
about-title = O aplikaci Skribisto
about-version = Verze { $version }
about-update-available = Je k dispozici verze { $version }
about-update-see-notes = Co je nového
about-update-flathub = Tuto kopii udržuje aktuální Flathub.
about-update-distro = Tuto kopii udržuje aktuální vaše distribuce.
about-tagline = Aplikace pro psaní románů a dalších dlouhých prozaických textů, napsaná v Rustu s knihovnou Teksilo.
about-license = Vydáno pod licencí GNU General Public License, verze 3.
about-copyright = © 2026 Cyril Jacquet
about-close = Zavřít

native-menu-about = O aplikaci { $app }
native-menu-hide = Skrýt { $app }
native-menu-quit = Ukončit { $app }
native-menu-settings = Nastavení…
native-menu-window = Okno
native-menu-minimize = Minimalizovat
native-menu-zoom = Zvětšit

window-title = { $title } - { $app }
window-title-numbered = { $title } - { $app } (okno { $n })
window-title-empty = { $app }

first-run-window-title = Vítejte
first-run-title = Nastavení aplikace { $app }
first-run-body = { $app } uchovává svá nastavení odděleně od aplikace Skribisto, takže začíná s prázdným nastavením. Vaše předvolby, nedávné projekty, slovníky a rozvržení oken teď můžete zkopírovat.
first-run-from = Kopírovat z
first-run-to = Kopírovat do
first-run-copy-note = Nic se nepřesouvá ani neodstraňuje. Skribisto si ponechá všechna svá nastavení a bude fungovat přesně jako dosud.
first-run-import = Importovat nastavení
first-run-start-fresh = Začít s čistým nastavením
first-run-import-failed = Některá nastavení nebylo možné importovat: { $error }

settings-page-punctuation = Výchozí interpunkce
settings-group-punctuation = Chytrá interpunkce
settings-page-work-punctuation = Interpunkce
settings-punctuation-override = Použít pro tento projekt vlastní pravidla interpunkce
settings-punctuation-override-hint = Vypnuto: projekt se řídí předvolbou aplikace. Tato pravidla jsou uložena v souboru .skrib, takže spoluautor, který soubor otevře, píše se stejnou typografií.
settings-punctuation-dashes = Měnit -- na krátkou pomlčku a --- na dlouhou pomlčku
settings-punctuation-ellipsis = Měnit ... na výpustku (…)
settings-punctuation-quotes = Měnit rovné uvozovky a apostrofy na typografické
settings-quote-style = Uvozovky
settings-quote-style-locale = Podle jazyka
settings-quote-style-curly = “Dvojité”
settings-quote-style-curly-single = ‘Jednoduché’
settings-quote-style-guillemets = «Lomené»
settings-quote-style-low-high = „Dolní a horní“
settings-punctuation-spacing = Mezera před ; : ! ?
settings-punctuation-spacing-hint = Francouzská typografie vkládá úzkou nezlomitelnou mezeru před ; ! ? a plnou před :. Platí to jen pro text psaný francouzsky. Mezera uvnitř uvozovek « » přichází spolu s lomenými uvozovkami samotnými.
settings-punctuation-sample = Váš jazyk dává
settings-punctuation-app-hint = Co dělá každý projekt, pokud si v Projekt ▸ Interpunkce nenastaví vlastní pravidla.
settings-punctuation-dialogue = Odstavec začínající „- “ otevřít dialogovou pomlčkou
settings-punctuation-dialogue-hint = Pro jazyky, které přímou řeč označují pomlčkou místo uvozovek: francouzštinu, španělštinu, ruštinu a další. Uplatní se jen na úplném začátku odstavce.

statusbar-go-to = Přejít na…
go-to-placeholder = Hledat v pořadači
go-to-no-matches = Hledání neodpovídá žádná položka.
menu-go-to = &Přejít na…

settings-page-distraction-free-themes = Motivy bez rozptylování
settings-themes-builtin = Vestavěné motivy
settings-themes-builtin-badge = Vestavěný
settings-themes-user = Moje motivy
settings-themes-editor-group = Upravit motiv
settings-themes-editor-empty = Chcete-li motiv upravit, vyberte jej v části „Moje motivy“.
settings-themes-use = Použít
settings-themes-duplicate = Duplikovat
settings-themes-edit = Upravit
settings-themes-delete = Smazat
settings-themes-export = Exportovat…
settings-themes-import = Importovat…
settings-themes-copy-suffix = kopie
settings-themes-json-filter = Motiv (JSON)
settings-themes-imported = Motiv byl importován
settings-themes-import-failed = Tento motiv nelze importovat
settings-themes-exported = Motiv byl exportován
settings-themes-export-failed = Tento motiv nelze exportovat
settings-themes-low-contrast = nízký kontrast
settings-themes-low-contrast-band = zvýraznění skrývá text
settings-themes-field-name = Název
settings-themes-field-paper = Stránka
settings-themes-field-ink = Text
settings-themes-field-general = Pozadí
settings-themes-field-widget-text = Text ovládací lišty
settings-themes-field-caret-band = Zvýraznění kolem kurzoru

statusbar-focus-settings = Nastavení režimu bez rozptylování
statusbar-focus-manage-themes = Spravovat motivy…

comments-title = Komentáře
comments-document-title = Tento dokument
comments-empty-project = V tomto projektu zatím nejsou žádné komentáře.
comments-empty-document = K tomuto dokumentu nejsou žádné komentáře.
comments-filter-all = Vše
comments-filter-open = Otevřené
comments-filter-resolved = Vyřešené
comments-filter-orphaned = Osiřelé
comments-status-open = Otevřený
comments-status-resolved = Vyřešený
comments-status-orphaned = Ztratil svůj text
comments-orphan-snippet = (komentovaný text už neexistuje)
comments-status-unplaced = Bez pozice v textu
comments-unplaced-snippet = (není ukotven k žádnému textu)
comments-reply-count = { $count ->
    [one] { $count } odpověď
    [few] { $count } odpovědi
   *[other] { $count } odpovědí
}
comments-menu-resolve = Vyřešit
comments-menu-reopen = Znovu otevřít
comments-menu-delete = Smazat
comments-sort-document = V pořadí dokumentu
comments-sort-newest = Nejnovější první
comments-sort-reader = Seskupit podle čtenáře
comments-unsigned-author = Nepodepsáno
comments-agreement = { $count ->
    [one] { $count } čtenář
    [few] { $count } čtenáři
   *[other] { $count } čtenářů
}
comments-menu-add = Přidat komentář
comments-menu-add-paragraph = Komentovat tento odstavec
overview-col-comments = Komentáře
overview-col-total-comments = Komentáře celkem
comments-card-placeholder = Napište komentář…
comments-card-unknown-author = Neznámý autor
comments-unsigned-toast = Vaše komentáře nejsou podepsané: v tomto počítači pro vás není nastaveno žádné jméno.
comments-unsigned-action = Nastavit jméno
comments-card-reply = Odpovědět
comments-card-reply-placeholder = Odpovědět…
comments-card-actions = Akce komentáře
comments-reply-actions = Akce odpovědi
comments-menu-delete-reply = Smazat odpověď
comments-menu-delete-all = Smazat zde všechny komentáře
comments-deleted-toast = Komentář byl smazán
comments-reply-deleted-toast = Odpověď byla smazána
comments-deleted-all-toast = { $count ->
    [one] { $count } komentář smazán
    [few] { $count } komentáře smazány
   *[other] { $count } komentářů smazáno
}
comments-undo = Vrátit zpět

analysis-segment = Analýza
analysis-scope-book = Analyzuje se tato kniha
analysis-run = Spustit analýzu
analysis-stale = Od posledního spuštění změněno
analysis-not-run = Zatím neanalyzováno.
analysis-running = Čtení rukopisu…
analysis-failed = Analýzu nebylo možné dokončit.
analysis-no-scenes = V této knize zatím nejsou žádné scény.

analysis-shape = Tvar

analysis-arrivals = Přírůstky
analysis-arrivals-explainer = Jak se text dostával do tohoto projektu, dokud byl otevřený. Říká, kterou cestou znaky přišly, a vůbec nic o tom, kdo je napsal: kdo píše koncept jinde a vkládá jej, ten vložil, a kdo diktuje, ten nadiktoval. Není tu žádné číslo, o které by se mělo usilovat.
analysis-arrivals-scope = Celý tento projekt, ne jen tato kniha.
analysis-arrivals-session = Od chvíle, kdy jste jej otevřeli. Zavřením projektu začne počítání znovu.
analysis-arrivals-nothing = V tomto sezení zatím nepřibyl žádný text.
analysis-arrivals-typed = Napsáno
analysis-arrivals-pasted = Vloženo
analysis-arrivals-dictated = Nadiktováno
analysis-arrivals-imported = Importováno
analysis-arrivals-programmatic = Vloženo aplikací
analysis-arrivals-count = { $count ->
    [one] { $count } znak
    [few] { $count } znaky
   *[other] { $count } znaků
}
analysis-arrivals-none = nic

analysis-words-per-scene = Slova na scénu
analysis-median-words = Medián délky scény v této knize je { $count ->
    [one] { $count } slovo.
    [few] { $count } slova.
   *[other] { $count } slov.
}
analysis-median-line = Medián: { $count ->
    [one] { $count } slovo
    [few] { $count } slova
   *[other] { $count } slov
}
analysis-dialogue = Dialogy
analysis-dialogue-unsupported = Pro tento jazyk se dialogy zatím neměří.

analysis-footnote-words = Slova v poznámkách pod čarou
analysis-footnote-words-count = { $count ->
    [one] V poznámkách pod čarou této knihy je { $count } slovo, počítané odděleně od celkového rozsahu rukopisu.
    [few] V poznámkách pod čarou této knihy jsou { $count } slova, počítaná odděleně od celkového rozsahu rukopisu.
   *[other] V poznámkách pod čarou této knihy je { $count } slov, počítaných odděleně od celkového rozsahu rukopisu.
}
analysis-footnote-words-pending = Počítání poznámek pod čarou…

analysis-ignore-empty = Přehlížet texty, ve kterých zatím nic není
analysis-empty-hidden = { $count ->
        [one] Skryt { $count } prázdný text.
        [few] Skryty { $count } prázdné texty.
       *[other] Skryto { $count } prázdných textů.
    }
analysis-all-texts-empty = Všechny texty v této knize jsou zatím prázdné.

binder-filter-count = Zobrazeno { $shown } z { $total }
binder-filter-clear = Vymazat
binder-filter-none = Hledání „{ $query }“ nic neodpovídá.

settings-paratext-intro = Úvodní a závěrečné části knihy, se kterými může nový projekt začít. Každá struktura patří k určité nakladatelské tradici a názvy jejích stránek jsou napsány v jazyce této tradice. Po vytvoření projektu je můžete libovolně přejmenovat.
settings-paratext-structures = Struktury
settings-paratext-broken = Nelze přečíst
settings-paratext-edit = Upravit
settings-paratext-duplicate = Duplikovat
settings-paratext-delete = Smazat
settings-paratext-new = Nová struktura
settings-paratext-save = Uložit
settings-paratext-editor-hint = Název, stránky vytvářené před rukopisem a stránky vytvářené za ním. Kam se nakonec dostanou, je na vás: jakmile projekt existuje, jsou to běžné položky.


image-insert = Vložit o&brázek…
image-no-project = Před vložením obrázku otevřete projekt.
image-choose-title = Vyberte obrázek
image-filter-label = Obrázky
image-large-title = Toto je velký obrázek
image-large-text =
    { $name } má { $megapixels } Mpx ({ $width }×{ $height }).
    Když jej ponecháte, jak je, uloží se do projektu váš původní soubor:
    putuje s každou zálohou a každým exportem. Optimalizace místo toho uloží
    menší kopii, nejvýše 2560 pixelů na delší straně.
image-large-keep = Ponechat původní
image-large-downscale = Optimalizovat
image-large-remember = Dělat to tak od teď a už se neptat
image-not-recorded = Obrázek byl uložen, ale nepodařilo se jej zaznamenat do projektu.
image-describe-title = Popis obrázku
image-describe-explain = Co obrázek ukazuje, pro čtenáře, který jej nevidí. Není součástí rukopisu: nikdy se nepočítá, neprohledává ani neexportuje jako próza.
image-describe-placeholder = maják proti šedé obloze
image-resize-title = Změnit velikost obrázku
image-resize-explain = Procento velikosti, ve které se zobrazuje nyní. Hodnota 100 jej ponechá beze změny.
image-resize-invalid = Zadejte číslo od 1 do 1000.
image-menu-describe = &Popsat obrázek…
image-menu-resize = &Změnit velikost obrázku…
image-menu-reset-size = Pů&vodní velikost

cover-choose = O&bálka knihy…
cover-clear = Odeb&rat obálku
cover-choose-title = Vyberte obálku
cover-set = Obálka je nastavena.
cover-cleared = Obálka byla odebrána. Obrázek v projektu zůstává.
card-label = Popisek
card-exportable = Exportováno
card-numbered = Číslováno
card-created = Vytvořeno
card-modified = Změněno
card-yes = ano
card-no = ne
card-type = Typ
card-position = Pozice
card-children = Podřízené položky
card-goal = Cíl
card-words = Slova
card-synopsis = Synopse
card-point-of-view = Úhel pohledu
card-aliases = Také známo jako

export-orphan-footnotes-title = Poznámky pod čarou bez odkazu
export-comments-dropped =
    { $count ->
        [one] Jeden komentář nebylo možné umístit do exportovaného textu, a byl proto vynechán.
        [few] { $count } komentáře nebylo možné umístit do exportovaného textu, a byly proto vynechány.
       *[other] { $count } komentářů nebylo možné umístit do exportovaného textu, a byly proto vynechány.
    }
export-orphan-footnotes =
    { $count ->
        [one] Na jednu poznámku pod čarou už v rukopisu nic neodkazuje. Její text se v exportované knize neobjeví.
        [few] Na { $count } poznámky pod čarou už v rukopisu nic neodkazuje. Jejich text se v exportované knize neobjeví.
       *[other] Na { $count } poznámek pod čarou už v rukopisu nic neodkazuje. Jejich text se v exportované knize neobjeví.
    }


menu-footnotes = Poznámky pod ča&rou
footnotes-title = Poznámky pod čarou
footnotes-insert = Vložit poznámku pod ča&rou
footnotes-empty = Zatím žádné poznámky pod čarou. Umístěte kurzor do scény a použijte + nahoře, nebo Ctrl+Alt+F.
footnotes-filter-all = Vše
footnotes-filter-document = Tento dokument
footnotes-filter-orphaned = Osiřelé
footnotes-orphaned = Na tuto poznámku už nic neodkazuje
footnotes-untitled-home = Bez názvu
footnotes-body-placeholder = text poznámky
footnotes-insert-tooltip = Vložit poznámku pod čarou u kurzoru (Ctrl+Alt+F)
footnotes-actions = Akce poznámky pod čarou
footnotes-delete = &Smazat poznámku a odkaz na ni
footnotes-no-project = Před vložením poznámky pod čarou otevřete projekt.
footnotes-no-caret = Chcete-li vložit poznámku pod čarou, umístěte kurzor do textu scény.
footnotes-not-created = Poznámku pod čarou se nepodařilo přidat do projektu.
footnotes-deleted-toast = Poznámka byla smazána
footnotes-undo-delete = Vrátit zpět

import-document-title = Importovat dokumenty
import-document-close = Zavřít
import-document-step-files = Soubory
import-document-step-review = Kontrola
import-document-step-destination = Umístění
import-document-step-reconcile = Sloučení
import-document-reconcile-hint = Některé z těchto položek už máte. Určete, co se má s každou z nich stát.
import-document-reconcile-all-new = Nic v tomto souboru neodpovídá vašemu projektu: všechny položky se přidají jako nové.
import-document-duplicate-returns = Tyto soubory představují více vracejících se kopií téhož rukopisu. Importujte je jednu po druhé: párování porovnává s vaším projektem vždy jen jednu vrácenou kopii, takže import několika najednou by místo sloučení přidal druhou kopii knihy.
import-document-hunk-added = Převzít tento nový odstavec
import-document-hunk-removed = Vypustit tento odstavec
import-document-hunk-changed = Převzít tuto úpravu
import-document-col-stray-prose = Její text
import-document-stray-as-paratext = Ponechat jako paratext
import-document-stray-discard = Zahodit text
import-document-col-current = Ve vašem projektu
import-document-col-incoming = V tomto souboru
import-document-col-status = Stav
import-document-col-action = Co udělat
import-document-status-identical = Stejné
import-document-status-editor-edited = Upravil redaktor
import-document-status-you-edited = Upravili jste vy
import-document-status-conflict = Upravili oba
import-document-status-different = Liší se
import-document-status-new = Nové
import-document-status-missing = Není v tomto souboru
import-document-status-moved = Přesunuto
import-document-action-comments-only = Jen komentáře
import-document-action-take-import = Převzít tuto verzi
import-document-action-keep-current = Ponechat moji
import-document-action-create-new = Přidat jako novou
import-document-action-ignore = Přeskočit
import-document-compare = Porovnat
import-document-compare-legend = Vaše verze ve srovnání s verzí z tohoto souboru. Jen pro čtení.
import-document-compare-close = Zavřít
import-document-drop-title = Přetáhněte dokumenty sem
import-document-drop-hint = Markdown (.md), Word (.docx), OpenDocument (.odt) a prostý text (.txt)
import-document-browse = Procházet…
import-document-move-up = Posunout nahoru
import-document-move-down = Posunout dolů
import-document-remove-file = Odebrat
import-document-file-count = { $count ->
    [one] 1 soubor
    [few] { $count } soubory
   *[other] { $count } souborů
}
import-document-no-files = Zatím nejsou vybrány žádné soubory.
import-document-col-included = Importovat
import-document-col-title = Název
import-document-col-type = Typ
import-document-col-words = Slova
import-document-col-breaks = Předěly
import-document-col-comments = Komentáře
import-document-col-footnotes = Poznámky pod čarou
import-document-col-epigraph = Motto
import-document-col-source = Zdroj
import-document-level-rules = Úrovně nadpisů
import-document-level-n = Nadpis { $level }
import-document-add-top-level = Přidat nejvyšší úroveň
import-document-add-top-level-tooltip = Vloží knihu nad všechny analyzované položky, pro soubory kapitol, které nemají nadpis knihy
import-document-destination = Umístění
import-document-destination-hint = Vyberte pořadač nebo položku: nové položky se umístí dovnitř složky, nebo za scénu.
import-document-destination-empty = Zatím žádné pořadače. Nejprve nějaký vytvořte.
import-document-plan-empty = Zatím není co importovat.
import-document-summary = { $rows ->
    [one] 1 položka
    [few] { $rows } položky
   *[other] { $rows } položek
} · { $breaks ->
    [one] 1 předěl scény
    [few] { $breaks } předěly scén
   *[other] { $breaks } předělů scén
}
import-document-back = Zpět
import-document-cancel = Zrušit
import-document-analyse = Další
import-document-analysing = Čtení dokumentů…
import-document-step-analysing = Čtení
import-document-cancel-analysis = Přestat číst
import-document-analyse-failed = Dokumenty nebylo možné přečíst.
import-document-details = Podrobnosti
import-document-import = Importovat
import-document-done = { $count ->
    [one] Importována 1 položka
    [few] Importovány { $count } položky
   *[other] Importováno { $count } položek
}
import-document-undo = Vrátit zpět
import-diagnostic-file-unreadable = „{ $path }“ nelze přečíst: { $detail }. Ostatní soubory se importují.
import-diagnostic-nested-too-deep = „{ $path }“ nebyl přečten: jeho obsah je vnořen hlouběji než do { $limit } úrovní. Žádný skutečný dokument se tomu ani neblíží a jeho čtení by mohlo Skribisto shodit, proto je vynechán. Ostatní soubory se importují.
import-diagnostic-archive-too-large = „{ $path }“ nebyl přečten: jde o komprimovaný soubor sestavený tak, aby se rozbalil do mnohonásobně větší velikosti, což by mohlo vyčerpat paměť a Skribisto shodit, proto je vynechán. Ostatní soubory se importují.
import-diagnostic-lossy-decode = { $count ->
    [one] Jeden znak v „{ $path }“ se nepodařilo dekódovat. Chcete-li jej zachovat, uložte soubor znovu v kódování UTF-8.
    [few] { $count } znaky v „{ $path }“ se nepodařilo dekódovat. Chcete-li je zachovat, uložte soubor znovu v kódování UTF-8.
   *[other] { $count } znaků v „{ $path }“ se nepodařilo dekódovat. Chcete-li je zachovat, uložte soubor znovu v kódování UTF-8.
}
import-diagnostic-decoded-from-bom = „{ $path }“ byl dekódován jako { $detail }, ne jako UTF-8.
import-diagnostic-empty-file = „{ $path }“ je prázdný.
import-diagnostic-no-headings = „{ $path }“ nemá žádné nadpisy, proto se importuje jako jedna položka.
import-diagnostic-unsupported-format = Soubory „.{ $detail }“ nic nepřečte, proto byl „{ $path }“ přeskočen.
import-diagnostic-front-matter-not-flat = Metadata v „{ $path }“: „{ $detail }“ není jednoduchá hodnota a byla přeskočena.
import-diagnostic-footnotes-degraded = { $count ->
    [one] Jedna poznámka pod čarou v „{ $path }“ se importuje jako prostý text: z Markdownu se poznámky pod čarou nenačítají.
    [few] { $count } poznámky pod čarou v „{ $path }“ se importují jako prostý text: z Markdownu se poznámky pod čarou nenačítají.
   *[other] { $count } poznámek pod čarou v „{ $path }“ se importuje jako prostý text: z Markdownu se poznámky pod čarou nenačítají.
}
import-diagnostic-footnote-not-carried = { $count ->
    [one] Jednu poznámku pod čarou v „{ $path }“ nebylo možné převést. Ostatní ano.
    [few] { $count } poznámky pod čarou v „{ $path }“ nebylo možné převést. Ostatní ano.
   *[other] { $count } poznámek pod čarou v „{ $path }“ nebylo možné převést. Ostatní ano.
}
import-diagnostic-raw-html-dropped = { $count ->
    [one] Jeden blok surového HTML v „{ $path }“ byl vynechán.
    [few] { $count } bloky surového HTML v „{ $path }“ byly vynechány.
   *[other] { $count } bloků surového HTML v „{ $path }“ bylo vynecháno.
}
import-diagnostic-nested-break-dropped = { $count ->
    [one] Jeden předěl scény uvnitř citace nebo seznamu v „{ $path }“ byl vynechán. Zachová se jen předěl na samostatném řádku.
    [few] { $count } předěly scén uvnitř citací nebo seznamů v „{ $path }“ byly vynechány. Zachová se jen předěl na samostatném řádku.
   *[other] { $count } předělů scén uvnitř citací nebo seznamů v „{ $path }“ bylo vynecháno. Zachová se jen předěl na samostatném řádku.
}
import-diagnostic-duplicate-title = „{ $title }“ se vyskytuje { $count }×. Pokud jste tyto soubory už dříve importovali, vzniknou duplikáty.
import-diagnostic-heading-level-jump = „{ $title }“ přeskakuje z úrovně nadpisu { $from } na { $to }; umístí se o jednu úroveň pod nadřazenou položku.
import-diagnostic-illegal-combination = „{ $title }“ obsahuje prózu, ale položka typu „{ $kind }“ žádnou obsahovat nemůže. Import je pozastaven, dokud nezměníte její typ nebo ji nevynecháte. Jinak se neimportuje vůbec nic.
import-diagnostic-tracked-changes-flattened = Soubor { $path } byl uprostřed revize: { $count ->
    [one] { $count } sledovaná změna byla přijata
    [few] { $count } sledované změny byly přijaty
   *[other] { $count } sledovaných změn bylo přijato
} a smazání byla provedena. Toto je konečný text. Chcete-li místo toho zachovat některé ze svých formulací, použijte v posledním kroku Porovnat.
import-diagnostic-tracked-changes-flattened-by = Soubor { $path } byl uprostřed revize: { $count ->
    [one] { $count } sledovaná změna od { $names } byla přijata
    [few] { $count } sledované změny od { $names } byly přijaty
   *[other] { $count } sledovaných změn od { $names } bylo přijato
} a smazání byla provedena. Toto je konečný text. Chcete-li místo toho zachovat některé ze svých formulací, použijte v posledním kroku Porovnat.
import-diagnostic-text-box-dropped = { $count ->
    [one] { $path } obsahuje { $count } textové pole. Jeho text stojí mimo tok dokumentu, takže nelze určit, kam v rukopisu patří. Neimportuje se.
    [few] { $path } obsahuje { $count } textová pole. Jejich text stojí mimo tok dokumentu, takže nelze určit, kam v rukopisu patří. Neimportují se.
   *[other] { $path } obsahuje { $count } textových polí. Jejich text stojí mimo tok dokumentu, takže nelze určit, kam v rukopisu patří. Neimportují se.
}
import-diagnostic-embedded-object-dropped = { $count ->
    [one] { $path } obsahuje { $count } vložený objekt: graf, rovnici nebo něco podobného. V rukopisu není nic, co by jej mohlo pojmout.
    [few] { $path } obsahuje { $count } vložené objekty: graf, rovnici nebo něco podobného. V rukopisu není nic, co by je mohlo pojmout.
   *[other] { $path } obsahuje { $count } vložených objektů: graf, rovnici nebo něco podobného. V rukopisu není nic, co by je mohlo pojmout.
}
import-diagnostic-field-flattened = { $count ->
    [one] { $path } obsahuje { $count } pole: číslo stránky, křížový odkaz, datum. Ponechá si text, který naposledy zobrazovalo, a už se nebude aktualizovat.
    [few] { $path } obsahuje { $count } pole: číslo stránky, křížový odkaz, datum. Každé si ponechá text, který naposledy zobrazovalo, a už se nebude aktualizovat.
   *[other] { $path } obsahuje { $count } polí: číslo stránky, křížový odkaz, datum. Každé si ponechá text, který naposledy zobrazovalo, a už se nebude aktualizovat.
}
import-diagnostic-unknown-style-level = { $path } používá styl „{ $detail }“, který vypadá jako nadpis, ale neuvádí úroveň. Tyto odstavce se importují jako próza, místo aby se jejich úroveň odhadovala.
import-diagnostic-comment-unanchored = Komentář „{ $detail }“ v { $path } nebylo možné připojit ke slovům, kterých se týkal. Zůstane u své položky, kde jej můžete přesunout.
import-diagnostic-comment-not-carried = Komentář „{ $detail }“ v { $path } se neimportuje, protože nic ze souboru nepřichází jako text, u kterého by mohl zůstat. Pokud jej potřebujete, zkopírujte si jej ze souboru.
import-diagnostic-comment-replies-flattened = { $count ->
    [one] { $count } odpověď v { $path } se vztahuje ke komentáři, který v souboru není, proto se importuje jako samostatný komentář.
    [few] { $count } odpovědi v { $path } se vztahují ke komentáři, který v souboru není, proto se importují jako samostatné komentáře.
   *[other] { $count } odpovědí v { $path } se vztahuje ke komentáři, který v souboru není, proto se importují jako samostatné komentáře.
}
import-diagnostic-prose-not-verbatim = { $count ->
    [one] { $count } odstavec v { $path } nebylo možné převést s formátováním přesně tak, jak byl napsán, proto se importuje jako prostý text. Porovnejte jej s originálem.
    [few] { $count } odstavce v { $path } nebylo možné převést s formátováním přesně tak, jak byly napsány, proto se importují jako prostý text. Porovnejte je s originálem.
   *[other] { $count } odstavců v { $path } nebylo možné převést s formátováním přesně tak, jak byly napsány, proto se importují jako prostý text. Porovnejte je s originálem.
}
import-diagnostic-styled-spaces-not-carried = { $count ->
    [one] V { $path } byl { $count } úsek prázdného místa podtržený nebo přeškrtnutý, například linka k vyplnění. Skribisto neuchovává čáru na samotných mezerách, proto se importuje jako obyčejné mezery, nebo vůbec, pokud odstavec začíná nebo končí.
    [few] V { $path } byly { $count } úseky prázdného místa podtržené nebo přeškrtnuté, například linky k vyplnění. Skribisto neuchovává čáru na samotných mezerách, proto se importují jako obyčejné mezery, nebo vůbec, pokud odstavec začínají nebo končí.
   *[other] V { $path } bylo { $count } úseků prázdného místa podtržených nebo přeškrtnutých, například linky k vyplnění. Skribisto neuchovává čáru na samotných mezerách, proto se importují jako obyčejné mezery, nebo vůbec, pokud odstavec začínají nebo končí.
}
import-diagnostic-list-nesting-flattened = { $count ->
    [one] { $count } položka seznamu v { $path } byla vnořena hlouběji než do { $limit } úrovní. Importuje se na úroveň { $limit }, nejhlubší, kterou Skribisto uchovává, i se svými slovy a formátováním.
    [few] { $count } položky seznamu v { $path } byly vnořeny hlouběji než do { $limit } úrovní. Importují se na úroveň { $limit }, nejhlubší, kterou Skribisto uchovává, i se svými slovy a formátováním.
   *[other] { $count } položek seznamu v { $path } bylo vnořeno hlouběji než do { $limit } úrovní. Importují se na úroveň { $limit }, nejhlubší, kterou Skribisto uchovává, i se svými slovy a formátováním.
}
import-diagnostic-table-read-as-paragraphs = { $count ->
    [one] { $count } tabulka v { $path } má řádek mnohem širší než ostatní, příliš nevyvážený na to, aby se dal uspořádat do mřížky. Její buňky se importují jako odstavce jeden za druhým, i se svými slovy a formátováním.
    [few] { $count } tabulky v { $path } mají řádek mnohem širší než ostatní, příliš nevyvážený na to, aby se dal uspořádat do mřížky. Jejich buňky se importují jako odstavce jeden za druhým, i se svými slovy a formátováním.
   *[other] { $count } tabulek v { $path } má řádek mnohem širší než ostatní, příliš nevyvážený na to, aby se dal uspořádat do mřížky. Jejich buňky se importují jako odstavce jeden za druhým, i se svými slovy a formátováním.
}
import-diagnostic-spaces-shortened = { $count ->
    [one] { $count } souvislá řada mezer v { $path } byla delší než { $limit } mezer a importuje se zkrácená na { $limit }. Slova kolem ní jsou zachována.
    [few] { $count } souvislé řady mezer v { $path } byly delší než { $limit } mezer a importují se zkrácené na { $limit }. Slova kolem nich jsou zachována.
   *[other] { $count } souvislých řad mezer v { $path } bylo delších než { $limit } mezer a importují se zkrácené na { $limit }. Slova kolem nich jsou zachována.
}
import-diagnostic-epigraph-not-carried = „{ $title }“ začíná mottem, ale položka typu „{ $kind }“ žádné obsahovat nemůže. Citát se místo toho ponechá na začátku jejího textu.
import-diagnostic-epigraph-placement-ambiguous = Mezi „{ $title }“ a „{ $below }“ stojí motto, které může patřit k oběma. Bylo přiřazeno k „{ $title }“, kam motto obvykle patří.
import-document-diagnostics = { $errors ->
    [0] { $warnings ->
            [one] 1 věc, kterou byste měli vědět
            [few] { $warnings } věci, které byste měli vědět
           *[other] { $warnings } věcí, které byste měli vědět
        }
   *[other] { $errors ->
            [one] 1 soubor nelze přečíst
            [few] { $errors } soubory nelze přečíst
           *[other] { $errors } souborů nelze přečíst
        }
}
import-document-diagnostics-none = Není co hlásit.

versions-title = Verze
versions-scope-synopsis = Synopse
versions-scope-prose = Text
versions-loading = Procházení vašich záloh…
versions-empty = Zatím žádná dřívější verze
versions-error = Vaše zálohy nebylo možné přečíst. Nic se neztratilo, ale tento seznam může být neúplný
versions-did-not-exist = K { $date } ještě neexistovalo
versions-deleted-after = Smazáno někdy po { $date }
versions-unreadable = { $count ->
    [one] { $count } zálohu nebylo možné přečíst
    [few] { $count } zálohy nebylo možné přečíst
   *[other] { $count } záloh nebylo možné přečíst
}
versions-thinned = { $count ->
    [one] Starší verze s věkem řídnou. Vlastní historie projektu už vypustila { $count } dřívější stav tohoto textu.
    [few] Starší verze s věkem řídnou. Vlastní historie projektu už vypustila { $count } dřívější stavy tohoto textu.
   *[other] Starší verze s věkem řídnou. Vlastní historie projektu už vypustila { $count } dřívějších stavů tohoto textu.
}
versions-source-backup = Ze zálohy
versions-source-project = Z vlastní historie projektu
versions-list-caption = Jeden záznam na změnu, ne na zálohu
versions-pick-a-version = Vyberte verzi a uvidíte, co se změnilo
versions-earliest = Nejstarší zaznamenaná verze. Není nic staršího, s čím by se dala porovnat.
versions-no-change = V této části položky se nic nezměnilo
versions-formatting-only = Změnilo se zde jen formátování. Slova jsou stejná
versions-show-unchanged = Zobrazit nezměněné
versions-hide-unchanged = Skrýt nezměněné
versions-next-change = Další změna
versions-near = poblíž „{ $text }“
versions-words-added = { $count ->
    [one] { $count } slovo přidáno
    [few] { $count } slova přidána
   *[other] { $count } slov přidáno
}
versions-words-removed = { $count ->
    [one] { $count } slovo odebráno
    [few] { $count } slova odebrána
   *[other] { $count } slov odebráno
}
versions-blocks-moved = { $count ->
    [one] { $count } odstavec přesunut
    [few] { $count } odstavce přesunuty
   *[other] { $count } odstavců přesunuto
}
versions-pin = Připnout zálohu, ze které tato verze pochází. Automatický úklid ji nikdy nesmaže
versions-unpin = Odepnout zálohu, ze které tato verze pochází. Automatický úklid ji pak může smazat
versions-pinned-only = Zobrazit jen připnuté verze
versions-pin-note = Připnout lze jen verze ze zálohy. Připnutí uchovává soubor a vlastní historie projektu souborem není.
versions-pinned-empty = Zatím zde není nic připnuto
versions-pinned-empty-log = Zatím zde není nic připnuto. Připnout lze jen verze ze zálohy. Připnutí uchovává soubor a vlastní historie projektu souborem není.
versions-range-filter = Zobrazit jen verze mezi dvěma daty
versions-filtered-empty = Nastaveným filtrům neodpovídá žádná verze
versions-clear-filters = Zrušit filtry
versions-last-30-days = Posledních 30 dní
versions-restore-button = Obnovit tuto verzi
versions-restore-confirm-title = Nahradit tento text verzí z { $date }?
versions-restore-confirm-text = To, co máte teď, bude nahrazeno textem, který tato položka měla k { $date }.
versions-restore-confirm-with-comments = To, co máte teď, bude nahrazeno textem, který tato položka měla k { $date }. { $count ->
    [one] { $count } komentář je ukotven v současném textu a může zůstat osiřelý.
    [few] { $count } komentáře jsou ukotveny v současném textu a mohou zůstat osiřelé.
   *[other] { $count } komentářů je ukotveno v současném textu a mohou zůstat osiřelé.
}
versions-restore-confirm-undo-note = Nejprve se pořídí záloha a Ctrl+Z tuto akci vrátí jedním krokem.
versions-restored-toast = Obnovena verze z { $date }
versions-undo = Vrátit zpět
versions-restore-row-gone = Tato položka už v projektu není
versions-restore-no-home = Tato položka od té doby změnila typ a starý text v ní nemá kam přijít
versions-restore-no-safety-copy = Bezpečnostní záloha neproběhla, proto se nic nezměnilo
versions-restore-failed = Obnovení selhalo: { $error }
versions-restore-backup-busy = Zálohování už probíhá. Zkuste to za chvíli znovu
versions-restore-in-backup-file = Prohlížíte si soubor zálohy; chcete-li do projektu obnovovat, otevřete samotný projekt
versions-restore-no-project = Není otevřen žádný projekt
versions-recreate-button = Vrátit zpět do projektu…
versions-recreate-picker-title = Kam ji umístit?
versions-recreate-picker-empty = Tento projekt nemá žádný pořadač, kam ji umístit
versions-recreate-picker-confirm = Vrátit ji sem
versions-recreate-picker-cancel = Zrušit
versions-recreate-untitled = tato položka
versions-recreate-confirm-title = Vrátit „{ $item }“ zpět?
versions-recreate-confirm-text = Přidá se do { $destination } s textem, který měla k { $date }.
versions-recreate-confirm-undo-note = Tlačítko Vrátit zpět v následující zprávě ji zase rovnou odebere.
versions-recreated-toast = „{ $item }“ je zpět ve vašem projektu
versions-recreated-partial-toast = { $count ->
    [one] „{ $item }“ je zpět, ale jeden z jejích textů nelze přečíst
    [few] „{ $item }“ je zpět, ale { $count } z jejích textů nelze přečíst
   *[other] „{ $item }“ je zpět, ale { $count } z jejích textů nelze přečíst
}
versions-recreate-already-here = Tato položka už ve vašem projektu je
versions-recreate-no-destination = Vyberte místo v pořadači, kam ji umístit
versions-recreate-unreadable = Tuto zálohu nebylo možné přečíst, proto se nic nepřidalo
versions-recreate-failed = Nepodařilo se ji vrátit: { $error }
versions-changed-percent = Změněno { $percent } %
versions-hidden-paragraphs = { $count ->
    [one] … { $count } nezměněný odstavec …
    [few] … { $count } nezměněné odstavce …
   *[other] … { $count } nezměněných odstavců …
}

timeline-title = Návrat v čase
timeline-coverage = { $count ->
    [one] { $count } zaznamenaná verze, sahající až do { $oldest }
    [few] { $count } zaznamenané verze, sahající až do { $oldest }
   *[other] { $count } zaznamenaných verzí, sahajících až do { $oldest }
}
timeline-loading = Procházení minulosti vašeho projektu…
timeline-empty = Zatím nebyla zaznamenána žádná verze tohoto projektu
timeline-no-changes = Od té doby se nic nezměnilo
timeline-no-text-changes = Od té doby se nezměnil žádný text
timeline-slider-label = Zaznamenaná verze
timeline-series-name = Velikost projektu
timeline-bars-caption = Každý sloupec je jedna zaznamenaná verze, vysoký jako tehdejší projekt. Zvýrazněný je ten, na který se díváte.
timeline-bars-caption-periods = Verzí je příliš mnoho na to, aby se zobrazily jednotlivě, takže každý sloupec představuje { $period } a je vysoký jako projekt na konci tohoto období.
timeline-unit-hour = hodinu
timeline-unit-day = den
timeline-unit-week = týden
timeline-unit-month = měsíc
timeline-range-filter = Zobrazit jen verze mezi dvěma daty
timeline-range-empty = V tomto období nebyla zaznamenána žádná verze
timeline-open-period = Otevřít toto období
timeline-show-all = Zobrazit celou historii
timeline-changed-since = { $count ->
    [one] { $count } položka se liší mezi stavem k { $date } a vaším současným projektem
    [few] { $count } položky se liší mezi stavem k { $date } a vaším současným projektem
   *[other] { $count } položek se liší mezi stavem k { $date } a vaším současným projektem
}
timeline-kind-added = Napsáno od té doby
timeline-kind-removed = Už není v projektu
timeline-kind-changed = Od té doby upraveno
timeline-kind-moved = Od té doby přesunuto
timeline-not-yet-written = V té době to ještě neexistovalo
timeline-no-text-of-its-own = Nemá vlastní text. Je to nadpis pro to, co obsahuje
timeline-reader-close = Zavřít
timeline-reader-stamp = Stav k { $date }
timeline-reader-loading = Otevírání zaznamenané verze…
timeline-reader-unreadable = Tento záznam nelze přečíst. Záloha mohla být přesunuta, smazána, nebo je na disku, který není připojen.
timeline-reader-compared = Stav k { $date } ve srovnání se současným textem
timeline-reader-diff-legend = Přeškrtnuté od té doby zmizelo, podtržené přibylo.
timeline-reader-view-label = Co zobrazit
timeline-reader-view-diff = Změny
timeline-reader-view-text = Text
timeline-reader-deleted = Toto už ve vašem projektu není. Zde si to můžete přečíst a zkopírovat.
timeline-prose-only-record = Tento bod pochází z vlastní historie projektu, která uchovává jen text, takže nelze ukázat, co bylo od té doby smazáno nebo přesunuto. K tomu vyberte zálohu.

settings-sec-extensions = Rozšíření

settings-group-games-forward = Vždy vpřed
settings-games-forward-toggle = Hrát „Vždy vpřed“
settings-games-forward-hint =
    Během hry nelze nic z napsaného vzít zpět: Backspace, Delete, vyjmutí,
    přetahování a vrácení zpět jsou v plochách zvolených níže vypnuté.
    Psát, vkládat, formátovat a volně se pohybovat můžete dál. Koncept jen roste.
settings-games-session-warning =
    Tato volba platí jen pro jedno sezení: nikdy se neukládá. Zavřením projektu
    nebo ukončením aplikace Skribisto hra vždy končí a vše, co jste během ní napsali,
    lze od té chvíle opět vracet zpět.
settings-group-games-scope = Kde platí
settings-games-in-prose = Próza rukopisu
settings-games-in-synopsis = Synopse
settings-games-scope-hint =
    Komentáře, poznámky pod čarou a názvy se nikdy nezmrazují: právě do nich si
    poznamenáte opravu, kterou jste si právě zakázali provést.
settings-games-inert-warning =
    „Vždy vpřed“ je zapnuté, ale na nic se nevztahuje. Zaškrtněte nahoře alespoň
    jednu plochu, jinak se při psaní nic nezmění.

games-title = Psací hry
games-forward-name = Vždy vpřed
games-forward-blurb =
    Pište koncept, aniž byste cokoli brali zpět. Mazání, vyjímání a vracení zpět jsou
    během hry vypnuté. Napište další větu, místo abyste opravovali tu poslední.
games-forward-playing = Hraje se: mazání je vypnuté
games-forward-idle = Nehraje se
games-session-note = Skončí, když zavřete projekt.
games-scope-prose-and-synopsis = Platí pro vaši prózu a synopse.
games-scope-prose = Platí pro vaši prózu.
games-scope-synopsis = Platí pro vaše synopse.
games-scope-nothing = Zatím se nevztahuje na nic. Vyberte plochu v Nastavení.
games-settings-link = Nastavení psacích her…
statusbar-games-forward = Vždy vpřed
statusbar-games-forward-tooltip =
    „Vždy vpřed“ je zapnuté: mazání, vyjímání a vracení zpět jsou při psaní
    konceptu vypnuté. Kliknutím hru ukončíte.
goal-progress-words = { $g ->
    [one] { $count } z { $goal } slova
    [few] { $count } z { $goal } slov
   *[other] { $count } z { $goal } slov
}
goal-progress-characters = { $g ->
    [one] { $count } z { $goal } znaku
    [few] { $count } z { $goal } znaků
   *[other] { $count } z { $goal } znaků
}
goal-count-words = { $n ->
    [one] { $count } slovo
    [few] { $count } slova
   *[other] { $count } slov
}
goal-count-characters = { $n ->
    [one] { $count } znak
    [few] { $count } znaky
   *[other] { $count } znaků
}
goal-unit-words = Slova
goal-unit-characters = Znaky

inspector-goal = Cíl
inspector-goal-none = Bez cíle

overview-col-goal = Cíl
overview-excluded-from-export = Vynecháno z exportu, takže se nezapočítává do žádného součtu

goal-subtree-total-words = { $count ->
    [one] { $items } cíl uvnitř, celkem slov: { $words }
    [few] { $items } cíle uvnitř, celkem slov: { $words }
   *[other] { $items } cílů uvnitř, celkem slov: { $words }
}
goal-subtree-total-characters = { $count ->
    [one] { $items } cíl uvnitř, celkem znaků: { $words }
    [few] { $items } cíle uvnitř, celkem znaků: { $words }
   *[other] { $items } cílů uvnitř, celkem znaků: { $words }
}

new-work-goal-unit = Počítat v
new-work-goal-unit-hint = Používá se pro všechny cíle počtu slov či znaků v tomto projektu. Později to lze změnit.

settings-group-goal-unit = Cíle
settings-goal-unit-switch-title = Změnit jednotku počítání
settings-goal-unit-switch-text = Cíle a milníky, které už jsou pro tento projekt nastavené, byly zadány v jednotkách „{ $from }“. Přepnutí na „{ $to }“ je nepřevede, takže každé stávající číslo se odteď bude číst v jednotkách „{ $to }“.
settings-goal-unit-switch-informative = Nic se neztratí. Kdykoli se můžete přepnout zpět a obnovit původní význam, a pak upravit cíle, které chcete zachovat.
settings-goal-unit-switch-confirm = Přepnout

goal-distribute-action = Rozdělit…
distribute-title = Rozdělit cíl
distribute-empty = Uvnitř není nic, mezi co by se dal cíl rozdělit.
distribute-weight = Způsob rozdělení
distribute-weight-length = Podle délky
distribute-weight-rows = Podle počtu částí
distribute-weight-even = Rovnoměrně
distribute-overwrite = Nahradit už nastavené cíle
distribute-col-item = Položka
distribute-col-current = Nyní
distribute-col-proposed = Potom
distribute-total = Součet je { $total } při cíli { $goal }.
distribute-over-budget = Cíle už nastavené uvnitř přesahují vlastní cíl tohoto kontejneru o { $over }. Zvyšte jeho cíl, snižte jejich, nebo je nahraďte.
distribute-apply = Rozdělit
distribute-cancel = Zrušit

milestone-target-gone = Cíl byl smazán
milestone-no-target = Bez cíle
milestone-add = Přidat
milestone-add-label = Čeho má být dosaženo

pace-summary-title = Kde kniha stojí
pace-summary-remaining = zbývá { $words }
pace-summary-open = Otevřít plán
pace-summary-close = Zavřít
pace-summary-dont-show = Nezobrazovat při otevírání
pace-summary-menu = Plán psaní…

menu-help-topics = &Témata nápovědy
menu-help-shortcuts = &Klávesové zkratky…
menu-help-website = Skribisto na &webu
menu-help-report = &Nahlásit problém…
menu-help-check-updates = Zkontrolovat &aktualizace
menu-command-palette = &Paleta příkazů…

learn-help-topics = Témata nápovědy
learn-shortcuts = Klávesové zkratky
learn-website = Skribisto na webu

help-window-title = Nápověda
help-filter-topics = Filtrovat témata
help-no-matching-topic = Žádné téma neodpovídá.
help-topic-missing = Toto téma už není k dispozici.
help-back = Zpět
help-not-translated = Tato stránka zatím nebyla přeložena, proto se zobrazuje anglicky.

help-section-getting-started = Začínáme
help-section-writing = Psaní
help-section-reviewing = Revize
help-section-exchanging = Import a export textu
help-section-keeping = Bezpečí vaší práce
help-section-extensions = Rozšíření

help-topic-getting-started = Váš první projekt
help-topic-writing-model = Jak je kniha poskládaná
help-topic-drafts-and-old-versions = Uchování staršího konceptu
help-topic-goals-and-pace = Cíle a tempo
help-topic-comments = Komentáře
help-topic-round-trip = Odeslání knihy čtenáři
help-topic-export = Export
help-topic-import-documents = Import dokumentů
help-topic-import-projects = Převzetí celého projektu
help-topic-backups-and-versions = Zálohy a verze

help-shortcuts-title = Klávesové zkratky
help-shortcuts-filter = Filtrovat zkratky
help-shortcuts-no-matches = Žádná zkratka neodpovídá.
help-shortcuts-rebind = Změnit zkratky…
help-shortcuts-close = Zavřít

command-palette-placeholder = Zadejte příkaz

shortcut-name-binder-duplicate = Duplikovat
shortcut-name-comments-add = Přidat komentář
shortcut-name-comments-add-paragraph = Komentovat odstavec
shortcut-name-spellcheck-toggle = Kontrolovat pravopis
shortcut-name-editor-tab-close = Zavřít kartu
shortcut-name-editor-tab-pin = Připnout nebo odepnout kartu
shortcut-name-editor-save = Uložit
shortcut-name-work-export = Exportovat…
shortcut-name-work-new = Nový projekt
shortcut-name-work-open = Otevřít projekt
shortcut-name-window-new = Nové okno
shortcut-name-work-close = Zavřít projekt
shortcut-name-app-settings = Nastavení
shortcut-name-app-quit = Ukončit
shortcut-name-editor-insert-footnote = Vložit poznámku pod čarou
shortcut-name-format-scene-break = Vložit předěl scény
shortcut-name-format-major-scene-break = Vložit hlavní předěl scény
shortcut-name-format-link = Vložit odkaz
shortcut-name-format-mark-nbsp = Vložit nezlomitelnou mezeru
shortcut-name-format-mark-nbhyphen = Vložit nezlomitelný spojovník
shortcut-name-format-mark-nnbsp = Vložit úzkou nezlomitelnou mezeru
shortcut-name-format-mark-zwsp = Vložit mezeru nulové šířky
shortcut-name-go-next = Další
shortcut-name-go-prev = Předchozí
shortcut-name-go-to = Přejít na
shortcut-name-outline-toggle = Zobrazit nebo skrýt osnovu
shortcut-name-preview-toggle = Zobrazit nebo skrýt pás náhledu
shortcut-name-view-fullscreen = Celá obrazovka
shortcut-name-view-focus-mode = Režim bez rozptylování
shortcut-name-editor-size-increase = Zvětšit text
shortcut-name-editor-size-decrease = Zmenšit text
shortcut-name-editor-size-reset = Obnovit velikost textu
shortcut-name-editor-find = Najít
shortcut-name-editor-replace = Nahradit
shortcut-name-editor-find-next = Další shoda
shortcut-name-editor-find-prev = Předchozí shoda
shortcut-name-search-show = Hledat v projektu
shortcut-name-search-replace = Nahradit v projektu
shortcut-name-outline-open-to-side = Otevřít vedle
shortcut-name-help-topics = Témata nápovědy
shortcut-name-help-shortcuts = Klávesové zkratky
shortcut-name-help-website = Skribisto na webu
shortcut-name-help-report = Nahlásit problém
shortcut-name-help-check-updates = Zkontrolovat aktualizace
update-check-behind = Je k dispozici verze { $version }.
update-check-current = Používáte nejnovější verzi.
update-check-failed = Nepodařilo se zjistit, zda existuje novější verze. { $error }
shortcut-name-command-palette = Paleta příkazů
help-section-reference = Přehled pojmů

help-concept-scene-break = Předěl scény
help-concept-major-scene-break = Hlavní předěl scény
help-concept-find-in-prose = Hledání v próze
help-concept-story-bible = Bible příběhu
help-concept-goal-unit = Slova, nebo znaky
help-concept-goal-progress = Průběh
help-concept-manuscript-words = Co se počítá do rukopisu
help-concept-exportable = Vynecháno z exportu
help-concept-distribute = Rozdělení cíle
help-concept-subtree-total = Součet cílů uvnitř
help-concept-milestone = Milník
help-concept-pace-plan = Plán tempa

help-concept-tag = Štítek
help-concept-label = Popisek
help-concept-point-of-view = Úhel pohledu
help-concept-epigraph = Motto
help-concept-footnote = Poznámka pod čarou
help-concept-chapter-mode = Podoba kapitol
help-concept-comment = Komentář
help-concept-backup = Záloha
help-concept-version = Verze
help-concept-trash = Koš
help-concept-spellcheck = Kontrola pravopisu
help-concept-search-replace = Hledat a nahradit
help-concept-note-template = Šablona poznámky
help-concept-story-bible-entry = Záznam bible příběhu
help-concept-text-replacement = Nahrazování textu
help-concept-smart-punctuation = Chytrá interpunkce
help-concept-export-style = Exportní styl
help-concept-round-trip-marks = Značky pro zpětný import

epigraph-mark-attribution = Řádek zdroje
epigraph-mark-attribution-tip = Označí řádek s kurzorem jako zdroj citátu, aby se vytiskl jako jeho uvedení autora
margin-lane-name = Značky na okraji
margin-lane-provider-comments = Komentáře
margin-lane-provider-story-bible = Záznam bible příběhu
margin-lane-provider-story-bible-hint = Při čtení poznámky v zobrazení V próze ukazuje, kde je daný záznam zmíněn, a scény vyprávěné jeho očima. Tato jména vyznačuje i přímo v próze.
margin-lane-mark-point-of-view = Vyprávěno odsud
margin-lane-mark-named-here = Zmíněno zde
margin-lane-provider-comments-hint = Kde je k textu připojena poznámka
margin-lane-provider-search = Výsledky hledání
margin-lane-provider-search-hint = Všechny výskyty toho, co jste naposledy hledali, kdekoli v dokumentu
margin-lane-provider-boundaries = Kde začíná každý dokument
margin-lane-provider-boundaries-hint = Linka na začátku každé scény v souvislém zobrazení, abyste viděli, kde jste
margin-lane-provider-spelling = Pravopis
margin-lane-provider-spelling-hint = Kde kontrola pravopisu označila slovo. Vypnuto, dokud o to nepožádáte: vyznačuje názor stroje na vaši prózu
margin-lane-spelling = { $word }, možná překlep
margin-lane-search-hit = { $text }, shoda { $index } z { $total }
margin-lane-search-current = { $text }, shoda { $index } z { $total }, aktuální
margin-lane-boundary = Začátek: { $title }
margin-lane-boundary-untitled = dokument bez názvu
settings-page-margin-lane = Značky na okraji
settings-desc-margin-lane = Pruh vedle posuvníku a co zobrazuje
settings-margin-lane-enabled = Zobrazit okrajový pruh
settings-margin-lane-enabled-hint = Stejný přepínač jako Zobrazení ▸ Značky na okraji.
settings-margin-lane-enabled-more = Nikdy netvrdí, že je něco špatně: ukazuje vám, kde co je, a co si o tom myslet, nechává na vás.
settings-group-margin-lane-marks = Co vyznačuje
settings-margin-lane-no-providers = Pruh zatím nic nevyznačuje.
settings-group-margin-lane-texture = Struktura dialogů
settings-margin-lane-texture = Zobrazit strukturu dialogů
settings-margin-lane-texture-hint = Jeden proužek na odstavec: jak je dlouhý a jaká jeho část je přímá řeč.
settings-margin-lane-texture-more = U jazyků bez ověřené konvence se neměří; proužek tam ukazuje jen délku.
settings-group-margin-lane-surfaces = Kde se zobrazuje
settings-margin-lane-surface-editor = Textový editor
settings-margin-lane-surface-stream = Souvislé zobrazení
settings-margin-lane-surface-search-preview = Náhled hledání
menu-margin-lane = &Značky na okraji

status-preset-drafting = Psaní konceptu
status-preset-passes = Revizní kola
status-preset-plume = Plume Creator
status-todo = K napsání
status-draft = Koncept
status-revised = Revidováno
status-final = Finální
status-outline = Osnova
status-first-edit = 1. úprava
status-second-edit = 2. úprava
status-done = Hotovo
status-plume-draft-1 = 1. koncept
status-plume-draft-2 = 2. koncept
status-plume-draft-3 = 3. koncept
status-plume-edit-1 = 1. úprava
status-plume-edit-2 = 2. úprava
status-plume-edit-3 = 3. úprava
status-plume-proofread = Po korektuře
status-plume-finished = Dokončeno
new-work-statuses = Pracovní postup
new-work-statuses-hint = Fáze, kterými scéna prochází. Kdykoli je můžete přejmenovat, přeuspořádat nebo doplnit.
status-none = Bez stavu
inspector-status = Stav
overview-col-status = Stav
overview-status-mixed = části níže se liší
help-concept-status = Stav
status-completion-title = Kde kniha stojí
status-completion-headline = Dokončené scény: { $done } z { $total }
status-completion-empty = Zatím žádné scény. Tento přehled čte rukopis, takže se bude plnit, jak budete psát.
status-completion-open = Kde kniha stojí…

settings-page-statuses = Stavy
settings-desc-statuses = Přejmenování, přeuspořádání, přidání a odebrání fází, kterými scéna prochází
settings-statuses-add = Přidat stav
settings-statuses-add-placeholder = Název nového stavu
settings-statuses-desc =
    Fáze, kterými scéna prochází, seřazené od nejméně hotové nahoře po nejvíce hotovou
    dole. Toto pořadí určuje, co všude jinde v aplikaci znamená „méně hotové než“,
    proto je seřaďte tak, jak váš postup skutečně probíhá.
settings-statuses-apply-preset = Použít předvolbu…
settings-statuses-preset-applied = { $added ->
    [one] Přidán 1 stav
    [few] Přidány { $added } stavy
   *[other] Přidáno { $added } stavů
}
settings-statuses-preset-refused = Tento projekt už škálu stavů má. Předvolby vyplní jen prázdnou. Nejprve smažte stupně, které nechcete.
settings-statuses-duplicate = „{ $name }“ už na této škále je
settings-statuses-added = Přidáno „{ $name }“
settings-statuses-move-up = Posunout nahoru (méně hotové)
settings-statuses-move-down = Posunout dolů (více hotové)
settings-statuses-details-placeholder = Co tato fáze znamená (nepovinné)
settings-statuses-delete = Smazat „{ $name }“
settings-statuses-delete-in-use = { $count ->
    [one] Smazat „{ $name }“: má jej 1 položka a přijde o svůj stav
    [few] Smazat „{ $name }“: mají jej { $count } položky a přijdou o svůj stav
   *[other] Smazat „{ $name }“: má jej { $count } položek a přijdou o svůj stav
}
settings-statuses-deleted = Smazáno „{ $name }“
settings-statuses-deleted-in-use = { $count ->
    [one] Smazáno „{ $name }“. 1 položka už nemá stav.
    [few] Smazáno „{ $name }“. { $count } položky už nemají stav.
   *[other] Smazáno „{ $name }“. { $count } položek už nemá stav.
}
settings-statuses-empty-title = Zatím žádný pracovní postup
settings-statuses-empty-body = Stav říká, jak daleko scéna je. Přidejte jej nahoře, nebo začněte z předvolby.
status-category-planned = Plánováno
status-category-drafting = Psaní konceptu
status-category-needs-work = Vyžaduje práci
status-category-revised = Revidováno
status-category-final = Finální

## Long-operation progress (shared::progress_text). The backend reports phase
## codes it cannot translate; these are their labels. `progress-percent` is the
## toast body on its own, `progress-percent-with-label` with the phase after it.
progress-percent = { $percent } %
progress-percent-with-label = { $percent } % · { $label }
progress-import-opening = Otevírání projektu…
progress-import-outline = Čtení osnovy…
progress-import-converting = Převod kapitol a scén…
progress-import-writing = Zápis souboru .skrib…
progress-import-done = Hotovo
progress-import-structure = Zjišťování struktury…
progress-export-start = Spouštění…
progress-export-writing = Zápis…
progress-project-reading = Čtení projektu…
# Settings: the open project's section title, "Work: <its title>".
settings-sec-work-titled = Projekt: { $title }

## Project-import notices (shared::import_warnings::notices): one line of the
## Plume / Manuskript warnings list each. $file, $folder, $title, $name, $field
## and $error are data (file names, titles, a parser's own message) and stay as
## they are. $container is "single-file" or "folder"; $kind is "scene", "note",
## "synopsis" or "title".
import-notice-flattened-rich-texts = { $count ->
    [one] Jeden text byl vnořen hlouběji, než projekt Skribisto dokáže pojmout; jeho slova byla zachována jako prostý text, bez formátování.
    [few] { $count } texty byly vnořeny hlouběji, než projekt Skribisto dokáže pojmout; jejich slova byla zachována jako prostý text, bez formátování.
   *[other] { $count } textů bylo vnořeno hlouběji, než projekt Skribisto dokáže pojmout; jejich slova byla zachována jako prostý text, bez formátování.
}
import-notice-free-text-badges = { $count ->
    [one] Jeden odznak scény vypadá spíš jako volný text než jako seznam stavů, a proto byl zachován jako popisek položky, místo aby se z něj stal štítek.
    [few] { $count } různé odznaky scén vypadají spíš jako volný text než jako seznam stavů, a proto byly zachovány jako popisky položek, místo aby se z nich staly štítky.
   *[other] { $count } různých odznaků scén vypadá spíš jako volný text než jako seznam stavů, a proto bylo zachováno jako popisky položek, místo aby se z nich staly štítky.
}
import-notice-separator-text-dropped = Oddělovač „{ $name }“ obsahoval text, pro který v projektu Skribisto není místo; text nebyl importován.
import-notice-separator-without-scene = Oddělovač „{ $name }“ neměl ve své kapitole před sebou žádnou scénu, ke které by se dal připojit, a proto nebyl importován.
import-notice-chapter-without-scenes = Kapitola „{ $name }“ nemá žádné scény, a proto její oddělovače nebyly importovány.
import-notice-rich-text-unconverted = Část formátovaného textu se nepodařilo převést ({ $error }); nebyla importována.
import-notice-unresolved-cross-link = Odkaz na postavu č. { $number } nelze následovat: záznam chybí nebo byl v koši.
import-notice-read-copy = Načtena kopie tohoto projektu { $container ->
    [single-file] v podobě jednoho souboru
   *[folder] v podobě složky
}.
import-notice-read-copy-dated = Načtena kopie tohoto projektu { $container ->
    [single-file] v podobě jednoho souboru
   *[folder] v podobě složky
}, naposledy změněná { $day }.
import-notice-original-format = Tento projekt je v původním formátu Manuskriptu z roku 2016. Byl načten celý; nic v něm nebylo změněno.
import-notice-newer-single-file-beside = „{ $file }“ je novější než tato složka ({ $file_day } oproti { $folder_day }). Když projekt přejde do jednosouborového režimu, Manuskript ponechá starou složku na místě, takže tato složka může být opuštěná kopie; import souboru .msk by místo ní načetl novější verzi.
import-notice-pickle-skipped = „{ $file }“ nebyl načten. Jde o soubor pickle jazyka Python a načtení takového souboru z projektu bylo bezpečnostní chybou přímo v Manuskriptu (CVE-2021-35196); nastavení, která obsahuje, se týkají zobrazení, ne psaní.
import-notice-file-unreadable = „{ $file }“ se nepodařilo načíst ({ $error }) a byl přeskočen; vše ostatní bylo importováno.
import-notice-not-utf8 = „{ $file }“ není platný text v UTF-8; znaky, které nešlo přečíst, byly nahrazeny a import se nezastavil.
import-notice-table-unreadable = „{ $file }“ se nepodařilo načíst ({ $error }); jeho obsah nebyl importován.
import-notice-outline-missing = „{ $file }“ chybí, takže tento projekt nemá žádný rukopis k importu.
import-notice-outline-unreadable = „{ $file }“ se nepodařilo načíst ({ $error }); nebyl importován žádný rukopis.
import-notice-named-none-row = „{ $file }“ začíná pojmenovaným řádkem („{ $name }“) tam, kde Manuskript uchovává svůj prázdný záznam „none“. Tento řádek byl vynechán, aby řádky za ním zachovaly čísla, která používá rukopis.
import-notice-unknown-field = „{ $file }“ obsahuje pole, které tento importér nezná: „{ $field }“. Nebylo importováno.
import-notice-colorless-labels = { $count ->
    [one] „{ $file }“ obsahuje řádek bez barvy. Byl importován bez ní; Manuskript na takovém řádku přestane projekt načítat.
    [few] „{ $file }“ obsahuje { $count } řádky bez barvy. Byly importovány bez ní; Manuskript na takovém řádku přestane projekt načítat.
   *[other] „{ $file }“ obsahuje { $count } řádků bez barvy. Byly importovány bez ní; Manuskript na takovém řádku přestane projekt načítat.
}
import-notice-settings-unreadable = „{ $file }“ se nepodařilo načíst ({ $error }); jazyk projektu nebyl importován.
import-notice-revisions-unreadable = „{ $file }“ se nepodařilo načíst ({ $error }); historie projektu nebyla importována, nic jiného tím ale nebylo dotčeno.
import-notice-foreign-files = { $count ->
    [one] Tento projekt obsahuje soubor, který není součástí formátu Manuskript a nebyl importován: { $files }.
    [few] Tento projekt obsahuje { $count } soubory, které nejsou součástí formátu Manuskript a nebyly importovány: { $files }.
   *[other] Tento projekt obsahuje { $count } souborů, které nejsou součástí formátu Manuskript a nebyly importovány: { $files }.
}
import-notice-foreign-files-more = Soubory tohoto projektu, které nejsou součástí formátu Manuskript a nebyly importovány ({ $count }): { $files } a { $more ->
    [one] jeden další
    [few] { $more } další
   *[other] { $more } dalších
}.
import-notice-not-a-character-file = „{ $file }“ není soubor postavy Manuskriptu, a proto byl vynechán.
import-notice-not-a-text-item = „{ $file }“ není textová položka Manuskriptu, a proto byl z osnovy vynechán.
import-notice-missing-blank-line = „{ $file }“ nemá za hlavičkou prázdný řádek. Jeho poslední pole bylo zachováno, ačkoli Manuskript o ně přijde.
import-notice-folder-without-metadata = Složka „{ $folder }“ neobsahuje { $file }, a proto byla importována jako obyčejná složka pojmenovaná podle adresáře. Manuskript takový adresář i vše v něm přeskočí.
import-notice-rows-given-ids = { $count ->
    [one] Jeden řádek osnovy neměl vlastní ID a jedno dostal. Manuskript takový řádek zahodí, a je-li to složka, přestane projekt načítat.
    [few] { $count } řádky osnovy neměly vlastní ID a dostaly ho. Manuskript takový řádek zahodí, a je-li to složka, přestane projekt načítat.
   *[other] { $count } řádků osnovy nemělo vlastní ID a dostalo ho. Manuskript takový řádek zahodí, a je-li to složka, přestane projekt načítat.
}
import-notice-plots-unreadable = „{ $file }“ se nepodařilo načíst ({ $error }); dějové linie nebyly importovány.
import-notice-world-unreadable = „{ $file }“ se nepodařilo načíst ({ $error }); záznamy o světě nebyly importovány.
import-notice-world-not-opml = „{ $file }“ není dokument OPML; záznamy o světě nebyly importovány.
import-notice-text-flattened = Text položky „{ $title }“ je vnořen hlouběji, než projekt Skribisto dokáže pojmout; jeho slova byla zachována jako prostý text, bez formátování.
import-notice-text-unconverted = Text položky „{ $title }“ se nepodařilo převést ({ $error }); byl zachován přesně tak, jak byl napsán.
import-notice-text-without-file-name = Pro { $kind ->
    [scene] text scény
    [note] text poznámky
    [synopsis] synopsi
   *[title] název
} položky „{ $title }“ nešlo vytvořit název souboru; tato část nebyla importována.
import-notice-unknown-label = „{ $title }“ má popisek { $index }, který v seznamu popisků tohoto projektu není. Položka byla importována bez popisku.
import-notice-unknown-status = „{ $title }“ má stav { $index }, který v seznamu stavů tohoto projektu není. Položka byla importována bez stavu.
import-notice-unknown-point-of-view = „{ $title }“ je vyprávěno očima postavy, která mezi postavami tohoto projektu není. Úhel pohledu nebyl nastaven.
import-notice-unresolved-references = Odkazy v textu položky „{ $title }“ se nepodařilo vyřešit ({ $error }); text byl zachován tak, jak byl převeden, včetně značek odkazů.
import-notice-custom-icons = { $count ->
    [one] Jeden řádek měl vlastní ikonu. Skribisto dává každému řádku ikonu jeho typu, a proto ikona nebyla importována; nic jiného se na řádku nezměnilo.
    [few] { $count } řádky měly vlastní ikonu. Skribisto dává každému řádku ikonu jeho typu, a proto ikony nebyly importovány; nic jiného se na těchto řádcích nezměnilo.
   *[other] { $count } řádků mělo vlastní ikonu. Skribisto dává každému řádku ikonu jeho typu, a proto ikony nebyly importovány; nic jiného se na těchto řádcích nezměnilo.
}
import-notice-character-colors = { $count ->
    [one] Jedna postava měla u jména barvu. Skribisto barví štítky, ne řádky, a proto barva nebyla importována; samotná postava ano.
    [few] { $count } postavy měly u jména barvu. Skribisto barví štítky, ne řádky, a proto barvy nebyly importovány; samotné postavy ano, všechny.
   *[other] { $count } postav mělo u jména barvu. Skribisto barví štítky, ne řádky, a proto barvy nebyly importovány; samotné postavy ano, všechny.
}
import-notice-versions-kept = { $found ->
    [one] Tento projekt zaznamenal jednu dřívější verzi
    [few] Tento projekt zaznamenal { $found } dřívější verze
   *[other] Tento projekt zaznamenal { $found } dřívějších verzí
}; { $kept ->
    [one] jedna byla zachována
    [few] { $kept } byly zachovány
   *[other] { $kept } bylo zachováno
}, prořídnuté stejně, jako Skribisto prořeďuje svou vlastní historii. Najdete je na panelu Verze.
import-notice-versions-unplaced = { $count ->
    [one] Jedna dřívější verze patří k řádku, který už v projektu není, a nešlo ji umístit.
    [few] { $count } dřívější verze patří k řádku, který už v projektu není, a nešlo je umístit.
   *[other] { $count } dřívějších verzí patří k řádku, který už v projektu není, a nešlo je umístit.
}

## Native file dialogs: the window title, and the label of the file-type filter.
dialog-open-work-title = Otevřít projekt Skribisto
dialog-save-as-file-title = Uložit jako jeden soubor .skrib
dialog-save-as-folder-title = Vyberte nadřazenou složku pro projekt
dialog-manuskript-file-title = Vyberte projekt Manuskript
dialog-manuskript-folder-title = Vyberte složku projektu Manuskript
dialog-filter-skrib = Projekt Skribisto
dialog-filter-manuskript = Projekt Manuskript
dialog-filter-plume = Projekt Plume Creator
dialog-filter-hunspell-aff = Soubor afixů Hunspell (.aff)
dialog-filter-hunspell-dic = Slovník Hunspell (.dic)
dialog-filter-export = Exportovaný soubor
# Help ▸ Keyboard shortcuts / Settings ▸ Keymap / command palette.
shortcut-name-view-margin-lane = Zobrazit nebo skrýt značky na okraji

## The built-in distraction-free themes' names (distraction_free::theme).
df-theme-paper = Papír
df-theme-sepia = Sépie
df-theme-night = Noc
df-theme-high-contrast = Vysoký kontrast
df-theme-midnight = Půlnoc

## Welcome ▸ Examples: the line under a bundled example's title.
welcome-example-byline = Autor: { $author }

## Spoken by a screen reader only.
# Welcome window: the name of the section tab bar.
welcome-sections-a11y = Části úvodního okna
# A comment's anchored span in the editor: author, plain-text preview, replies.
comments-a11y-summary-author = { $author }: { $text }
comments-a11y-summary-replies = { $text } ({ $count ->
    [one] 1 odpověď
    [few] { $count } odpovědi
   *[other] { $count } odpovědí
})
comments-a11y-summary-author-replies = { $author }: { $text } ({ $count ->
    [one] 1 odpověď
    [few] { $count } odpovědi
   *[other] { $count } odpovědí
})
# Margin marks beside the text: one mark, several merged into one, and the
# lane's own name when nothing more specific was given.
margin-lane-a11y-mark = { $label }, { $percent } % od začátku
margin-lane-a11y-marks = { $label }, značky: { $count }, { $percent } % od začátku
margin-lane-a11y-fallback = { $count ->
    [one] Značky na okraji, 1 značka
    [few] Značky na okraji, { $count } značky
   *[other] Značky na okraji, { $count } značek
}
# Export ▸ Choose: the row standing for a binder that has no name.
export-choose-unnamed-binder = Nepojmenovaný pořadač
# The built-in export styles named in words (the national manuscript presets
# keep their own language's name).
export-style-neutral = Neutrální
export-style-manuscript-shunn = Standardní rukopis (Shunn)
export-style-ebook-clean = Čistá e-kniha

## Dates. Put the parts in the order your language writes a short date; the
## parts arrive already zero-padded where that matters.
# Comment cards, the outline card's Created/Modified.
date-time-short = { $day }.{ $month }.{ $year } { $hour }:{ $minute }
# Pace chart: a day on the axis, a milestone's date, a holiday.
pace-day-label = { $day }.{ $month }.

## File sizes (shared::byte_size). $whole and $tenth are the two sides of the
## decimal mark: write your language's mark between them and its unit symbol.
byte-size-bytes = { $value } B
byte-size-kb = { $value } kB
byte-size-mb = { $whole },{ $tenth } MB
byte-size-gb = { $whole },{ $tenth } GB
# A whole percentage (sliders, the Pace card, the text-size toast).
unit-percent = { $value } %
# A whole number of pixels (slider readouts).
unit-pixels = { $value } px
# Settings ▸ Paratext: a preset saved without a `name` line. `name` is the TOML
# key the writer types and stays as it is.
settings-paratext-error-no-name = Struktura musí mít řádek `name`.
# Export: started with no project open (a menu raced a close).
export-error-no-project = Není otevřený žádný projekt, který by šlo exportovat.

shortcut-group-work = Projekt
shortcut-group-edit = Úpravy
shortcut-group-view = Zobrazení
shortcut-group-document = Dokument
shortcut-group-format = Formát
shortcut-group-go = Přejít
shortcut-group-tools = Nástroje
shortcut-group-help = Nápověda

## Manuskript import: the headings of the notes it writes and the title of a
## project with no name. Written into the imported project once, in the
## interface language.
import-manuskript-section-paragraph-summary = Shrnutí v jednom odstavci
import-manuskript-section-full-summary = Úplné shrnutí
import-manuskript-section-motivation = Motivace
import-manuskript-section-epiphany = Prozření
import-manuskript-section-notes = Poznámky
import-manuskript-section-description = Popis
import-manuskript-section-passion = Vášeň
import-manuskript-section-result = Výsledek
import-manuskript-section-summary = Shrnutí
import-manuskript-section-meta = Metadata
import-manuskript-section-series = Série
import-manuskript-section-volume = Svazek
import-manuskript-section-genre = Žánr
import-manuskript-section-license = Licence
import-manuskript-section-email = E-mail
import-manuskript-section-situation = Situace
import-manuskript-section-sentence = Jedna věta
import-manuskript-section-paragraph = Jeden odstavec
import-manuskript-section-page = Jedna strana
import-manuskript-section-full = Úplné shrnutí
import-manuskript-untitled-project = Importovaný projekt Manuskript

## Plume import: the story-bible groups an old project implies, and the title
## of a project with no name.
import-plume-group-characters = Postavy
import-plume-group-items = Předměty
import-plume-group-places = Místa
import-plume-untitled-project = Importovaný projekt Plume
