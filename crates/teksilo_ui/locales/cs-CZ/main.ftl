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
menu-import-scrivener = &Scrivener (.scriv)…
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
statusbar-focus-comments = Komentáře
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
backup-failed-close-error-text = Před zavřením se nepodařilo zapsat žádnou záložní kopii: { $error }. Opravte to a zkuste to znovu, nebo skončete bez zálohování.
backup-no-result = Zálohování skončilo, aniž by oznámilo, zda kopii vytvořilo. Podívejte se do seznamu záloh, zda v něm je nová.

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
import-plume-notes-of = { $name } (poznámky)
import-plume-content-of = { $name } (text)
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
import-plume-no-outline = Tento soubor je komprimovaný archiv, ale ne projekt Plume Creator: nemá osnovu. Nic nebylo zapsáno.
import-plume-empty-outline = Soubor osnovy tohoto projektu Plume Creator je prázdný, takže není co importovat. Nic nebylo zapsáno.
# $tag is the name of the document the part turned out to be.
import-plume-wrong-document = Toto není projekt Plume Creator: jeho část „{ $part }“ je dokument typu „{ $tag }“. Nic nebylo zapsáno.
# $version is the part's own version, $supported the newest this importer reads.
import-plume-too-new = Část „{ $part }“ tohoto projektu zapsala novější verze Plume Creator ({ $version }), než jakou Skribisto umí přečíst (nejvýše { $supported }). Nic nebylo zapsáno.

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
import-manuskript-notes-of = { $title } (poznámky)
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

import-manuskript-not-a-project-file = „{ $file }“ není archiv projektu Manuskript ani malý soubor .msk, který odkazuje na složku projektu. Nic nebylo zapsáno.
import-manuskript-folder-missing = „{ $stub }“ odkazuje na složku projektu „{ $folder }“, která má ležet vedle něj, ale tato složka tam není. Projekt Manuskript ve složkovém režimu tvoří obojí a obojí se musí přenášet společně. Nic nebylo zapsáno.
import-manuskript-not-a-project = Toto nevypadá jako projekt Manuskript: nemá značku Manuskriptu ani osnovu. Nic nebylo zapsáno.
import-manuskript-format-too-new = Tento projekt je ve formátu Manuskript { $version }, novějším, než jaký Skribisto umí přečíst (nejvýše { $supported }). Nic nebylo zapsáno.
import-manuskript-too-many-files = Projekt nebyl importován: obsahuje více než { $limit } souborů, mnohem víc než kterýkoli projekt Manuskript. Nic nebylo zapsáno.
import-manuskript-too-large = Projekt nebyl importován: je větší než { $limit } MB, které Skribisto z projektu Manuskript přečte. Nic nebylo zapsáno.
project-import-close = Zavřít
project-import-location = Cílová složka
# The folder dialog's title.
project-import-location-title = Vyberte cílovou složku
project-import-name = Název souboru
project-import-name-placeholder = Název projektu
project-import-will-create = Vytvoří se
project-import-cancel = Zrušit
project-import-import = Importovat
# Field validation
project-import-location-required = Vyberte cílovou složku
project-import-location-missing = Tato složka neexistuje
project-import-location-not-folder = Tato cesta není složka
project-import-location-readonly = Do této složky nelze zapisovat
project-import-name-required = Zadejte název souboru
project-import-name-exists = Soubor s tímto názvem zde už existuje. Import si přepsání nechá potvrdit
project-import-name-is-folder = Složka s tímto názvem tu už je, možná projekt uložený jako složka. Import ji nemůže nahradit: zvolte jiný název
# Overwrite confirmation
project-import-overwrite-title = Nahradit stávající soubor?
project-import-overwrite-text = „{ $name }“ už existuje. Nahradit jej importovaným projektem?
# Projects are imported one at a time, whichever importer asks.
project-import-busy-title = Právě se importuje jiný projekt
project-import-busy-text = Projekty se importují jeden po druhém. Počkejte, až probíhající import skončí, nebo jej zrušte, a potom tento import spusťte znovu.
# Spoken by a screen reader once per phase, never shown: the toast's title, then
# the phase's name.
project-import-progress-announcement = { $title } { $phase }
# The notice once Cancel is pressed, until the converter has stopped: it may be
# finishing a file it cannot leave half written.
project-import-stopping = Zastavování importu…
project-import-cancelled = Import byl zrušen
project-import-open-now = Otevřít hned
project-import-details = Podrobnosti
# The import's report: what the writer should know about what came across.
project-import-report = { $count ->
    [one] 1 věc, kterou byste o tomto importu měli vědět
    [few] { $count } věci, které byste o tomto importu měli vědět
   *[other] { $count } věcí, které byste o tomto importu měli vědět
}
project-import-report-title = O tomto importu
# Error toast: a short reason in the body, the full technical chain behind Details.
project-import-error-title = Projekt nelze importovat
# Why a project was not imported, whichever importer read it. A $file is a file
# name, a $folder a full path (it is the folder the writer has to act on), a
# $reason what the system or a parser said, a $limit a number.
project-import-failed-cancelled = Import byl zrušen. Nic nebylo zapsáno.
project-import-failed-target-exists = „{ $file }“ už existuje a import jej neměl povoleno nahradit. Nic nebylo zapsáno.
project-import-failed-target-folder = „{ $file }“ je složka, možná projekt uložený jako složka, a import ji nemůže nahradit. Zvolte jiný název. Nic nebylo zapsáno.
project-import-failed-not-written = „{ $file }“ nebylo možné zapsat do složky „{ $folder }“ ({ $reason }). Nic nebylo zapsáno.
project-import-failed-disk-full = Ve složce „{ $folder }“ nezbývá dost místa na zápis „{ $file }“. Uvolněte tam místo a potom import spusťte znovu. Nic nebylo zapsáno.
project-import-failed-source-missing = „{ $file }“ už tam není. Nic nebylo zapsáno.
project-import-failed-unreadable = „{ $file }“ se nepodařilo přečíst ({ $reason }). Nic nebylo zapsáno.
project-import-failed-archive-damaged = „{ $file }“ je komprimovaný soubor, který nebylo možné přečíst, nejspíš proto, že je poškozený ({ $reason }). Nic nebylo zapsáno.
project-import-failed-archive-entries = Projekt nebyl importován: jeho archiv uvádí { $entries ->
    [one] { $entries } soubor
    [few] { $entries } soubory
   *[other] { $entries } souborů
}, víc než { $limit }, které smí projekt obsahovat. Tolik jich má jen soubor sestavený k vyčerpání paměti. Nic nebylo zapsáno.
project-import-failed-archive-member = Projekt nebyl importován: jeho část „{ $part }“ se rozbalí na více než { $limit } MB, což dělá jen soubor sestavený k vyčerpání paměti. Nic nebylo zapsáno.
project-import-failed-archive-size = Projekt nebyl importován: rozbalí se na více než { $limit } MB, což dělá jen soubor sestavený k vyčerpání paměti. Nic nebylo zapsáno.
project-import-failed-nested = Projekt nebyl importován: jeho část „{ $part }“ je vnořená hlouběji než do { $limit } úrovní a její čtení by mohlo Skribisto shodit. Nic nebylo zapsáno.
project-import-failed-entities = Projekt nebyl importován: jeho část „{ $part }“ deklaruje entity XML. Jejich rozvinutí by mohlo zabrat víc paměti, než počítač má, a Skribisto shodit. Nic nebylo zapsáno.
project-import-failed-folders = Projekt nebyl importován: jeden z jeho souborů leží hlouběji než { $limit } složek a jeho čtení by mohlo Skribisto shodit. Nic nebylo zapsáno.
project-import-failed-malformed = Projekt nebyl importován: jeho část „{ $part }“ je poškozená a nelze ji přečíst ({ $reason }). Nic nebylo zapsáno.
# A Scrivener project the importer cannot bring across (a Scrivener 1 for Mac project, a
# damaged binder file). Why is the Scrivener importer's own to say; Details holds its key.
project-import-failed-scrivener-refused = Skribisto tento projekt Scrivener importovat nedokáže. Nic nebylo zapsáno.
# The Scrivener project asks something its review did not answer: a project or a binder
# file appeared in it since.
project-import-failed-scrivener-question = Projekt Scrivener klade otázku, na kterou jeho kontrola neodpověděla. Spusťte jeho import znovu, odpovězte na ni a potom importujte. Nic nebylo zapsáno.
# The Scrivener project raises a question the writer has to accept before it imports.
project-import-failed-scrivener-not-accepted = Aby bylo možné projekt Scrivener importovat, musíte v kroku Projekt zaškrtnout políčko, a to zaškrtnuté nebylo. Spusťte jeho import znovu, políčko zaškrtněte a potom importujte. Nic nebylo zapsáno.
# The Scrivener project changed after the writer reviewed it.
project-import-failed-scrivener-changed = Projekt Scrivener se od vaší kontroly změnil. Spusťte jeho import znovu, zkontrolujte jej a potom importujte. Nic nebylo zapsáno.
# An error nothing above accounts for; $detail is its technical description.
project-import-failed-internal = Import se zastavil na chybě, se kterou Skribisto nepočítá: { $detail }

import-target-open-title = Tento projekt je otevřený
import-target-open-text = „{ $name }“ je otevřen v aplikaci Skribisto. Import přes něj by nahradil projekt, na kterém pracujete, a jeho příští uložení by tento projekt zapsalo zpět přes import. Zvolte jiný název souboru, nebo projekt nejprve zavřete.
import-warnings-announcement = { $result } { $notice }.

# Quitting while an import runs. $name is the file the import is writing.
quit-import-running-title = Import ještě probíhá
quit-import-running-text = Skribisto ještě importuje „{ $name }“. Ukončením aplikace teď import zastavíte: nic se nezapíše a projekt, ze kterého importujete, zůstane, jak byl.
quit-import-stop = Zastavit import a ukončit
## Import a Scrivener project: the wizard
import-scrivener-title = Importovat projekt Scrivener
import-scrivener-step-project = Projekt
import-scrivener-step-review = Kontrola
import-scrivener-step-save = Uložení
import-scrivener-back = Zpět
import-scrivener-next = Další
import-scrivener-import = Importovat
import-scrivener-cancel = Zrušit
import-scrivener-help = Nápověda
## Project step: choosing the project
import-scrivener-drop-title = Přetáhněte projekt Scrivener sem
import-scrivener-drop-hint = Projekt .scriv, soubor .scrivx v něm, záložní .zip nebo .scrivtemplate
import-scrivener-source = Projekt Scrivener
import-scrivener-source-placeholder = Cesta k projektu
import-scrivener-choose-file = Vybrat soubor…
import-scrivener-choose-folder = Vybrat složku…
# Windows and Linux show a .scriv project as an ordinary folder.
import-scrivener-source-hint = Vyberte složku projektu .scriv, nebo ji otevřete a vyberte soubor .scrivx uvnitř. Poslouží i záložní .zip.
# macOS shows a .scriv project as a single file when Scrivener is installed, as a folder when it is not.
import-scrivener-source-hint-mac = Pokud se projekt zobrazuje jako jeden soubor, použijte „Vybrat soubor…“. Pokud se zobrazuje jako složka, použijte „Vybrat složku…“. Poslouží i záložní .zip.
# File-dialog titles and filter label, resolved when the dialog opens.
import-scrivener-dialog-file-title = Vyberte projekt Scrivener
import-scrivener-dialog-folder-title = Vyberte složku projektu Scrivener
import-scrivener-dialog-filter = Projekt Scrivener
import-scrivener-source-missing = Na této cestě nic není
# A path whose name is not Unicode: the field shows it with its unreadable letters replaced.
import-scrivener-source-unreadable-name = Tento název nelze přečíst. Přejmenujte projekt ve správci souborů a potom jej vyberte znovu.
import-scrivener-empty-hint = Vyberte projekt a uvidíte, co v něm Skribisto najde.
import-scrivener-reading = Čtení projektu…
import-scrivener-stop-reading = Přestat číst
import-scrivener-read-failed = Projekt nebylo možné přečíst
import-scrivener-read-failed-body = Při čtení se něco pokazilo. Zvolte „Načíst projekt znovu“, chcete-li to zkusit ještě jednou, nebo „Podrobnosti“, chcete-li zjistit, co se pokazilo.
import-scrivener-details = Podrobnosti
import-scrivener-check-again = Načíst projekt znovu
# A refusal this version does not know how to word.
import-scrivener-refusal-unknown-title = Tento projekt nelze importovat
import-scrivener-refusal-unknown-body = Skribisto neví, jak jej převést. Důvod uvádějí podrobnosti.
## Project step: the project card
import-scrivener-made-with-mac = Naposledy uloženo aplikací Scrivener { $version } pro macOS
import-scrivener-made-with-windows = Naposledy uloženo aplikací Scrivener { $version } pro Windows
import-scrivener-made-with-linux = Naposledy uloženo aplikací Scrivener { $version } pro Linux
import-scrivener-made-with-ios = Naposledy uloženo aplikací Scrivener { $version } pro iPhone a iPad
import-scrivener-made-with-version = Naposledy uloženo aplikací Scrivener { $version }
import-scrivener-made-with-unknown = Projekt Scrivener, formát { $format }
import-scrivener-saved-on = Uloženo { $date } na zařízení „{ $device }“
import-scrivener-saved-on-no-device = Uloženo { $date }
import-scrivener-card-counts = { $documents ->
    [one] 1 dokument
    [few] { $documents } dokumenty
   *[other] { $documents } dokumentů
}, z toho { $manuscript } v rukopisu
## Review step
import-scrivener-review-hint = Takto se váš pořadač převede. Dokud neimportujete, nic se nezapíše, a u mnoha položek můžete typ později změnit v pořadači příkazem Převést na.
import-scrivener-choices-structure = Úrovně
import-scrivener-top-level = Složky nejvyšší úrovně
import-scrivener-choices-text = Text
import-scrivener-choice-containers = Složky na úrovni { $depth }
import-scrivener-choice-texts = Dokumenty na úrovni { $depth }
import-scrivener-choice-count = { $count ->
    [one] 1 položka
    [few] { $count } položky
   *[other] { $count } položek
}
import-scrivener-choice-count-example = { $count ->
    [1] 1 položka: „{ $example }“
    [few] { $count } položky, například „{ $example }“
   *[other] { $count } položek, například „{ $example }“
}
import-scrivener-rung-part = Části
import-scrivener-rung-chapter = Kapitoly
import-scrivener-rung-plain = Obyčejné složky
import-scrivener-texts-scenes = Scény
import-scrivener-texts-chapters = Kapitoly
import-scrivener-choice-books = Knihy v rukopisu
import-scrivener-books-one = Jedna kniha
import-scrivener-books-several = Jedna kniha na každou složku
import-scrivener-choice-split = { $count ->
    [1] Rozdělit „{ $title }“ podle jeho nadpisu
    [few] Rozdělit „{ $title }“ podle jeho { $count } nadpisů
   *[other] Rozdělit „{ $title }“ podle jeho { $count } nadpisů
}
import-scrivener-root-items = { $count ->
    [one] 1 položka
    [few] { $count } položky
   *[other] { $count } položek
}
import-scrivener-root-items-manuscript = { $count ->
    [one] 1 položka. Tato složka vypadá jako rukopis.
    [few] { $count } položky. Tato složka vypadá jako rukopis.
   *[other] { $count } položek. Tato složka vypadá jako rukopis.
}
import-scrivener-dest-notes = Poznámky
import-scrivener-dest-story-bible = Bible příběhu
import-scrivener-dest-manuscript = Další kniha
import-scrivener-dest-leave-out = Vynechat
import-scrivener-choice-trash = { $count ->
    [one] Přenést i koš (1 položka)
    [few] Přenést i koš ({ $count } položky)
   *[other] Přenést i koš ({ $count } položek)
}
import-scrivener-trash-detail = { $count ->
    [0] Žádná z nich neobsahuje text.
    [one] 1 z nich obsahuje text.
    [few] { $count } z nich obsahují text.
   *[other] { $count } z nich obsahuje text.
}
import-scrivener-choice-blank-lines = Prázdné řádky se stanou předěly scén
import-scrivener-choice-ornaments = Řádky z ozdobných znaků se stanou předěly scén
import-scrivener-preview-label = Struktura nového projektu
import-scrivener-col-title = Název
import-scrivener-col-becomes = Stane se
import-scrivener-col-exports = V exportech
import-scrivener-in-exports-yes = Ano
import-scrivener-in-exports-no = Ne
# What a row the binder has no creation command for becomes.
import-scrivener-becomes-item = Položka
import-scrivener-becomes-trashed = V koši
import-scrivener-before-title = Před importem
import-scrivener-review-error-unreadable = Kontrolu nebylo možné připravit. Vyberte projekt znovu.
import-scrivener-review-error-choice = Tato volba k projektu nesedí. Vyberte projekt znovu.
import-scrivener-review-error-unmappable = S touto volbou nelze pořadač převést, aniž by se něco ztratilo. Zvolte jinak, nebo problém nahlaste.
## Save step
import-scrivener-save-hint = Import vytvoří nový projekt Skribisto. Váš projekt Scrivener se jen čte, nikdy se nemění.
import-scrivener-title-field = Název
import-scrivener-author = Autor
import-scrivener-language = Jazyk
import-scrivener-location-inside-project = Vyberte složku mimo projekt Scrivener. Scrivener v ní uchovává jen své vlastní soubory.
import-scrivener-copy-research-label = Zkopírovat soubory z rešerší vedle nového projektu
import-scrivener-copy-research = { $count ->
    [one] 1 soubor z rešerší, který Skribisto neumí otevřít, například PDF nebo zvuk, se uloží do složky vedle projektu. Pokud volbu nezaškrtnete, jeho poznámka uvede, kde ve vašem projektu Scrivener leží.
    [few] { $count } soubory z rešerší, které Skribisto neumí otevřít, například PDF nebo zvuky, se uloží do složky vedle projektu. Pokud volbu nezaškrtnete, jejich poznámky uvedou, kde ve vašem projektu Scrivener leží.
   *[other] { $count } souborů z rešerší, které Skribisto neumí otevřít, například PDF nebo zvuky, se uloží do složky vedle projektu. Pokud volbu nezaškrtnete, jejich poznámky uvedou, kde ve vašem projektu Scrivener leží.
}
# The file name proposed when the project's name holds nothing a file name can.
import-scrivener-default-name = Projekt Scrivener
## Progress and result
import-scrivener-progress-title = Importování projektu Scrivener…
import-scrivener-done-title = Projekt Scrivener byl importován
# How many rows the new project holds: the documents and folders that came across, and
# the rows the import made for them (a document's notes, the project notes).
import-scrivener-done = { $items ->
    [one] Nový projekt obsahuje 1 položku.
    [few] Nový projekt obsahuje { $items } položky.
   *[other] Nový projekt obsahuje { $items } položek.
}
# The same, with how many kinds of thing could not be read or need checking.
import-scrivener-done-check = { $items ->
    [one] Nový projekt obsahuje 1 položku.
    [few] Nový projekt obsahuje { $items } položky.
   *[other] Nový projekt obsahuje { $items } položek.
} { $findings ->
    [one] 1 věc vyžaduje vaši pozornost.
    [few] { $findings } věci vyžadují vaši pozornost.
   *[other] { $findings } věcí vyžaduje vaši pozornost.
}
import-scrivener-review-again = Znovu projít import
## The import's report (the dialog See report opens)
import-scrivener-see-report = Zobrazit zprávu
import-scrivener-report-from = Přečteno z { $path }
import-scrivener-report-to = Zapsáno do { $path }
import-scrivener-report-came-across = Co se přeneslo
import-scrivener-report-nothing-counted = Nic nebylo započítáno.
import-scrivener-report-items = { $count ->
    [one] 1 položka
    [few] { $count } položky
   *[other] { $count } položek
}
import-scrivener-report-words = { $count ->
    [one] 1 slovo
    [few] { $count } slova
   *[other] { $count } slov
}
import-scrivener-report-synopses = { $count ->
    [one] 1 synopse
    [few] { $count } synopse
   *[other] { $count } synopsí
}
import-scrivener-report-document-notes = { $count ->
    [one] poznámky 1 dokumentu
    [few] poznámky { $count } dokumentů
   *[other] poznámky { $count } dokumentů
}
import-scrivener-report-comments = { $count ->
    [one] 1 komentář
    [few] { $count } komentáře
   *[other] { $count } komentářů
}
import-scrivener-report-footnotes = { $count ->
    [one] 1 poznámka pod čarou
    [few] { $count } poznámky pod čarou
   *[other] { $count } poznámek pod čarou
}
import-scrivener-report-pictures = { $count ->
    [one] 1 obrázek
    [few] { $count } obrázky
   *[other] { $count } obrázků
}
import-scrivener-report-templates = { $count ->
    [one] 1 šablona poznámky
    [few] { $count } šablony poznámek
   *[other] { $count } šablon poznámek
}
import-scrivener-report-tags = { $count ->
    [one] 1 štítek
    [few] { $count } štítky
   *[other] { $count } štítků
}
import-scrivener-report-statuses = { $count ->
    [one] 1 stav
    [few] { $count } stavy
   *[other] { $count } stavů
}
import-scrivener-report-trashed = { $count ->
    [one] 1 položka v koši
    [few] { $count } položky v koši
   *[other] { $count } položek v koši
}
import-scrivener-report-research-files = { $count ->
    [one] 1 soubor z rešerší
    [few] { $count } soubory z rešerší
   *[other] { $count } souborů z rešerší
}
import-scrivener-report-choices = Co jste zvolili
import-scrivener-report-accepted-mobile = Projekt jste importovali bez změn provedených v aplikaci Scrivener pro iPhone nebo iPad, které v něm ještě nebyly.
import-scrivener-report-accepted-no-text = Tuto kopii jste importovali s prázdnými dokumenty, protože žádný jejich text v ní nebyl.
import-scrivener-report-choice = { $subject }: { $answer }
import-scrivener-report-yes = ano
import-scrivener-report-no = ne
import-scrivener-report-research-copied = Soubory z rešerší, které Skribisto neumí otevřít, byly zkopírovány do složky „{ $folder }“ vedle nového projektu.
import-scrivener-report-research-not-copied = Soubory z rešerší, které Skribisto neumí otevřít, nebyly zkopírovány. Jejich poznámky uvádějí, kde ve vašem projektu Scrivener leží.
import-scrivener-report-research-copied-except = { $count ->
    [one] Soubory z rešerší, které Skribisto neumí otevřít, byly zkopírovány do složky „{ $folder }“ vedle nového projektu, kromě 1 souboru, který zůstává ve vašem projektu Scrivener. Jeho poznámka uvádí, kde leží.
    [few] Soubory z rešerší, které Skribisto neumí otevřít, byly zkopírovány do složky „{ $folder }“ vedle nového projektu, kromě { $count } souborů, které zůstávají ve vašem projektu Scrivener. Jejich poznámky uvádějí, kde leží.
   *[other] Soubory z rešerší, které Skribisto neumí otevřít, byly zkopírovány do složky „{ $folder }“ vedle nového projektu, kromě { $count } souborů, které zůstávají ve vašem projektu Scrivener. Jejich poznámky uvádějí, kde leží.
}
import-scrivener-report-research-none-copied = { $count ->
    [1] Požádali jste o zkopírování souborů z rešerší, ale jediný takový soubor se zkopírovat nepodařilo. Zůstává ve vašem projektu Scrivener a jeho poznámka uvádí, kde leží.
   *[other] Požádali jste o zkopírování souborů z rešerší, ale nepodařilo se zkopírovat žádný z { $count } souborů. Zůstávají ve vašem projektu Scrivener a jejich poznámky uvádějí, kde leží.
}
import-scrivener-report-chose-project = Zvolili jste import projektu „{ $project }“, jednoho z několika projektů, které obsahuje vámi vybraná složka nebo archiv.
import-scrivener-report-chose-manifest = Zvolili jste soubor pořadače „{ $file }“.
import-scrivener-report-chose-enclosing = Vybrali jste něco uvnitř projektu „{ $project }“, proto byl importován celý projekt.
import-scrivener-report-chose-upgraded = Importovali jste „{ $current }“, projekt, ve kterém jste dál psali, nikoli kopii, kterou Scrivener uchoval před aktualizací.
import-scrivener-report-chose-backup = Importovali jste „{ $backup }“, starší kopii, kterou Scrivener uchoval před aktualizací.
import-scrivener-report-title-placeholder-rejected = Název v nastavení projektu byl zástupným textem šablony, proto bylo místo něj navrženo jméno projektu.
import-scrivener-report-author-placeholder-rejected = Jméno autora v nastavení projektu bylo zástupným textem šablony, proto nebylo navrženo žádné.
import-scrivener-report-title-from-compile-settings = Navržen byl název, který uvádí nastavení kompilace projektu: jméno, které projekt dostal při vytvoření, pokud tam v aplikaci Scrivener nebyl nastaven jiný název.
import-scrivener-report-title-from-template = Navržen byl vlastní název šablony.
import-scrivener-report-title-from-folder-name = Jako název bylo navrženo jméno projektu.
import-scrivener-report-title-from-project-properties = Navržen byl název, který uvádějí vlastnosti projektu.
import-scrivener-report-author-from-project = Navrženo bylo jméno autora, které uvádí nastavení projektu. Může tam zůstat autor šablony a ten by se vytiskl na titulní stránku.
import-scrivener-report-language-not-stated = Projekt neuváděl svůj jazyk, proto byl navržen jazyk, ve kterém se zobrazuje Skribisto. Jazyk určuje, jak se počítají slova a jak se píší uvozovky.
import-scrivener-report-language-from-script = Jazyk byl navržen podle písma, kterým je rukopis napsán ({ $tag }).
# A proposal of the Save page the writer replaced with their own.
import-scrivener-report-you-changed-it = { $hint } Tento návrh jste nahradili.
import-scrivener-report-content-salvaged-text = Text položky „{ $item }“ v projektu chyběl. Jeho slova pocházejí z vyhledávacího indexu aplikace Scrivener, jako prostý text a možná ze starší verze: zkontrolujte je.
import-scrivener-report-content-salvaged-notes = Poznámky položky „{ $item }“ v projektu chyběly. Jejich slova pocházejí z vyhledávacího indexu aplikace Scrivener, jako prostý text a možná ze starší verze: zkontrolujte je.
import-scrivener-report-content-salvaged-synopsis = Synopse položky „{ $item }“ v projektu chyběla. Její slova pocházejí z vyhledávacího indexu aplikace Scrivener, jako prostý text a možná ze starší verze: zkontrolujte je.
# A finding of the document importer found in several places: its sentence about the
# first, then how many more the list under it names.
import-scrivener-report-document-others = { $others ->
    [one] { $sentence } Totéž platí ještě pro 1 další místo, uvedené níže.
    [few] { $sentence } Totéž platí ještě pro { $others } další místa, uvedená níže.
   *[other] { $sentence } Totéž platí ještě pro { $others } dalších míst, uvedených níže.
}
# An item of the report, by its path in Scrivener's binder, with which of its writings
# the line is about when it is not the text.
import-scrivener-report-item-notes = { $path } (poznámky dokumentu)
import-scrivener-report-item-synopsis = { $path } (synopse)
import-scrivener-report-item-project-note = { $path } (poznámka k projektu)
import-scrivener-report-item-recovered = { $path } (obnovený text)
# An item with no name in Scrivener.
import-scrivener-report-untitled = Bez názvu
import-scrivener-report-no-findings = Import nemá nic dalšího k hlášení.
import-scrivener-report-group-unread = Nelze přečíst
import-scrivener-report-group-check = Zkontrolujte
import-scrivener-report-group-changed = Převedeno jinak
import-scrivener-report-group-left = Zůstalo stranou
import-scrivener-report-group-count = { $group } ({ $count })
import-scrivener-report-items-toggle = { $count ->
    [one] 1 položka
    [few] { $count } položky
   *[other] { $count } položek
}
# The same, as a screen reader names it: with the line whose items it lists.
import-scrivener-report-items-toggle-named = { $count ->
    [one] 1 položka: { $line }
    [few] { $count } položky: { $line }
   *[other] { $count } položek: { $line }
}
import-scrivener-report-names-toggle = { $count ->
    [one] 1 název
    [few] { $count } názvy
   *[other] { $count } názvů
}
# The same, as a screen reader hears it, naming the line it belongs to.
import-scrivener-report-names-toggle-named = { $count ->
    [one] 1 název: { $line }
    [few] { $count } názvy: { $line }
   *[other] { $count } názvů: { $line }
}
import-scrivener-report-more = { $count ->
    [one] A ještě 1 další.
    [few] A ještě { $count } další.
   *[other] A ještě { $count } dalších.
}
import-scrivener-report-open-item = Otevřít v editoru
import-scrivener-report-links-hint = Otevřete nový projekt a potom tuto zprávu zobrazte znovu z palety příkazů: každá zde uvedená položka se pak otevře kliknutím.
import-scrivener-copy-report = Kopírovat zprávu
import-scrivener-report-copied = Zpráva byla zkopírována
import-scrivener-report-copy-failed = Zprávu nebylo možné zkopírovat.
import-scrivener-report-close = Zavřít
import-scrivener-report-none-title = Žádná zpráva o importu
import-scrivener-report-none = Od spuštění aplikace Skribisto neskončil žádný import z aplikace Scrivener. Skribisto uchovává zprávy o posledních importech, dokud se nezavře.
import-scrivener-report-gone = { $count ->
    [one] Zpráva o tomto importu už není uchována. Skribisto uchovává zprávu o posledním importu, dokud se nezavře.
   *[other] Zpráva o tomto importu už není uchována. Skribisto uchovává zprávy o posledních { $count } importech, dokud se nezavře.
}
## Words the import writes into the new project, in the language Skribisto shows itself in
import-scrivener-manuscript-binder = Rukopis
import-scrivener-notes-binder = Poznámky
import-scrivener-notes-of = { $title } (poznámky)
import-scrivener-project-notes = Poznámky k projektu
import-scrivener-recovered = Obnovené texty
import-scrivener-untitled-template = Šablona bez názvu
# The folder beside the new project its research files are copied into.
import-scrivener-research-folder = { $name } (soubory z rešerší)
# Between two authors' names.
import-scrivener-list-separator = {", "}
import-scrivener-label-details = Z pole „{ $field }“ v aplikaci Scrivener
import-scrivener-status-details = Z pole „{ $field }“ v aplikaci Scrivener
import-scrivener-unnamed-label = Popisek { $number }
import-scrivener-unnamed-status = Stav { $number }
import-scrivener-label-tag = { $name } (popisek)
import-scrivener-untitled-tag = Bez názvu
# The tags, the pace plan and the "From Scrivener" folder the import makes from what a
# Scrivener project holds beyond its labels and status.
import-scrivener-keyword-details = Klíčové slovo „{ $path }“ v aplikaci Scrivener
import-scrivener-unnamed-keyword = Klíčové slovo { $number }
import-scrivener-keyword-tag = { $name } (klíčové slovo)
import-scrivener-collection-details = Kolekce v aplikaci Scrivener
import-scrivener-collection-tag = { $name } (kolekce)
import-scrivener-field-details = Z pole „{ $field }“ v aplikaci Scrivener
# A tag made from one option of a list field.
import-scrivener-field-value = { $field }: { $option }
import-scrivener-field-tag = { $name } (pole)
# The day off a pace plan takes on its deadline.
import-scrivener-deadline-day = Termín
import-scrivener-from-scrivener = Z aplikace Scrivener
import-scrivener-note-project = Projekt
import-scrivener-note-project-bookmarks = Záložky projektu
import-scrivener-note-replacements = Náhrady při kompilaci
import-scrivener-note-auto-complete = Položky automatického doplňování
import-scrivener-note-saved-searches = Uložená hledání
import-scrivener-note-custom-metadata = Vlastní metadata
import-scrivener-note-targets = Cíle
import-scrivener-note-bookmarks = Záložky
import-scrivener-abbreviated-title = Zkrácený název
import-scrivener-subjects = Témata
import-scrivener-company = Společnost
import-scrivener-session-target = Cíl sezení
import-scrivener-session-target-calculated = Cíl sezení, vypočtený aplikací Scrivener z termínu
import-scrivener-writing-days = Psací dny
import-scrivener-weekday-monday = pondělí
import-scrivener-weekday-tuesday = úterý
import-scrivener-weekday-wednesday = středa
import-scrivener-weekday-thursday = čtvrtek
import-scrivener-weekday-friday = pátek
import-scrivener-weekday-saturday = sobota
import-scrivener-weekday-sunday = neděle
import-scrivener-draft-target = Cíl rukopisu
import-scrivener-deadline = Termín
# A unit after a count, in parentheses: "1000 (words)".
import-scrivener-unit-words = slova
import-scrivener-unit-characters = znaky
import-scrivener-unit-pages = strany
import-scrivener-search-query = Hledaný text
import-scrivener-search-in = Oblast hledání
import-scrivener-search-operator = Způsob shody
# What a saved search looks in, by the kind Scrivener names.
import-scrivener-search-in-all = vše
import-scrivener-search-in-title = názvy
import-scrivener-search-in-text = text
import-scrivener-search-in-notes = poznámky
import-scrivener-search-in-synopsis = synopse
import-scrivener-search-in-keywords = klíčová slova
import-scrivener-search-in-label = popisky
import-scrivener-search-in-status = stavy
import-scrivener-search-in-section-type = typy oddílů
import-scrivener-search-in-custom-metadata = vlastní metadata
# How a saved search's words combine.
import-scrivener-search-any = kterékoli slovo
import-scrivener-search-all = všechna slova
import-scrivener-search-exact = přesná fráze
# One line of a listing: a bookmark's place and its note, or a link's words and its address.
import-scrivener-label-value = { $label }: { $value }
import-scrivener-replacement-regex = regulární výraz
import-scrivener-replacement-off = vypnuto
# Said of a note whose own text lists its custom metadata or bookmarks.
import-scrivener-in-own-note = Uvedeno v poznámce, která vznikla z tohoto dokumentu.
# A refusal on one line: its title, then what to do.
import-scrivener-paragraph = { $title }. { $body }
## When the project cannot be imported
import-scrivener-refusal-missing-title = Na této cestě nic není
import-scrivener-refusal-missing-body = „{ $file }“ už tam není. Vyberte projekt znovu.
import-scrivener-refusal-unreadable-title = Tento projekt nelze přečíst
import-scrivener-refusal-unreadable-body = „{ $file }“ se nepodařilo přečíst ({ $reason }). Ověřte, že jej můžete otevřít, a zkuste to znovu.
import-scrivener-refusal-not-a-scrivener-project-title = Žádný projekt Scrivener tu není
import-scrivener-refusal-not-a-scrivener-project-body = Vyberte složku projektu .scriv, soubor .scrivx v ní, záložní .zip nebo .scrivtemplate.
import-scrivener-refusal-manifest-without-files-title = Dokumenty v této kopii nejsou
import-scrivener-refusal-manifest-without-files-body = „{ $manifest }“ tu je, ale chybí složka Files s dokumenty, které uvádí. Synchronizační služba možná ještě nedokončila kopírování projektu, nebo se zkopíroval jen soubor .scrivx. Pokud jste vybrali soubor .scrivx, vyberte místo něj složku projektu pomocí „Vybrat složku…“.
import-scrivener-refusal-search-too-wide-title = V této složce je toho na prohledání příliš mnoho
import-scrivener-refusal-search-too-wide-body = Obsahuje více než { $limit } složek k prohledání. Vyberte přímo projekt Scrivener.
import-scrivener-refusal-scrivener-one-for-mac-title = Tento projekt vytvořila aplikace Scrivener 1 pro Mac
import-scrivener-refusal-scrivener-one-for-mac-body = Skribisto čte projekty z aplikace Scrivener 2 a novějších. Otevřete jej jednou v aplikaci Scrivener 2 nebo 3, aby se aktualizoval, nebo jej zkompilujte do dokumentu Word a importujte ten.
import-scrivener-refusal-ios-stub-without-mobile-data-title = Toto je jen zástupný soubor z aplikace Scrivener pro iPhone nebo iPad
import-scrivener-refusal-ios-stub-without-mobile-data-body = Samotný projekt tu není. Synchronizujte jej ze svého iPhonu nebo iPadu, nebo jej odtamtud sdílejte jako .zip a importujte ten.
import-scrivener-refusal-newer-title = Tento projekt vytvořila novější verze aplikace Scrivener
import-scrivener-refusal-unknown-format-version-body = Jeho formát, { $version }, Skribisto nezná. Zkontrolujte, zda není k dispozici aktualizace aplikace Skribisto. Do té doby jej v aplikaci Scrivener zkompilujte do dokumentu Word a importujte ten.
import-scrivener-refusal-newer-format-stamp-body = Jeho soubory jsou označeny jako verze { $stamp } a Skribisto čte nejvýše verzi { $newest }. Zkontrolujte, zda není k dispozici aktualizace aplikace Skribisto. Do té doby jej v aplikaci Scrivener zkompilujte do dokumentu Word a importujte ten.
import-scrivener-refusal-version-stamp-unreadable-title = Verzi projektu nelze přečíst
import-scrivener-refusal-version-stamp-unreadable-body = Jeho soubor Files/version.txt neobsahuje číslo verze, takže Skribisto nemůže určit, zda tento projekt umí číst. Otevřete projekt jednou v aplikaci Scrivener a potom jej importujte znovu.
import-scrivener-refusal-damaged-manifest-title = Soubor pořadače nelze přečíst
import-scrivener-refusal-damaged-manifest-body = „{ $manifest }“ je poškozený. { $spare_copies ->
    [0] Otevřete projekt v aplikaci Scrivener, která jej umí opravit, a potom jej importujte znovu.
    [1] Scrivener si uvnitř projektu uchovává jeho záložní kopii, kterou Skribisto nečte. Otevřete projekt v aplikaci Scrivener, která jej umí opravit, a potom jej importujte znovu.
    [few] Scrivener si uvnitř projektu uchovává { $spare_copies } jeho záložní kopie, které Skribisto nečte. Otevřete projekt v aplikaci Scrivener, která jej umí opravit, a potom jej importujte znovu.
   *[other] Scrivener si uvnitř projektu uchovává { $spare_copies } jeho záložních kopií, které Skribisto nečte. Otevřete projekt v aplikaci Scrivener, která jej umí opravit, a potom jej importujte znovu.
}
# The empty file Scrivener writes for a new project before its first save: not damage.
import-scrivener-refusal-damaged-manifest-new-project-placeholder-body = „{ $manifest }“ je prázdný soubor, který Scrivener zapisuje pro projekt, jenž ještě neuložil. Otevřete projekt v aplikaci Scrivener, nechte jej uložit a potom jej importujte znovu.
import-scrivener-refusal-manifest-too-large-title = Tento soubor pořadače je příliš velký
import-scrivener-refusal-manifest-too-large-body = „{ $manifest }“ je větší než { $limit } MB, které Skribisto přečte. Žádný skutečný projekt se tomu ani neblíží, takže soubor může být poškozený.
import-scrivener-refusal-no-item-identifiers-title = Pořadač neuvádí žádný dokument, který by šlo najít
import-scrivener-refusal-no-item-identifiers-body = Žádná z jeho položek nemá identifikátor, takže jejich soubory nelze najít. Otevřete projekt jednou v aplikaci Scrivener a potom jej importujte znovu.
import-scrivener-refusal-incomplete-copy-title = V této kopii chybějí dokumenty
import-scrivener-refusal-incomplete-copy-body = Složka Files tu je, ale dokumenty v ní ne. Synchronizační služba možná ještě nedokončila kopírování projektu. Počkejte, až skončí, nebo znovu zkopírujte celou složku .scriv, a potom projekt načtěte znovu.
import-scrivener-refusal-archive-title = Tento archiv nelze otevřít
import-scrivener-refusal-archive-unreadable-body = „{ $file }“ se nepodařilo přečíst ({ $reason }). Možná je poškozený. Rozbalte jej sami a potom vyberte složku .scriv, kterou obsahuje.
import-scrivener-refusal-archive-unsafe-name-title = Tento archiv nelze bezpečně otevřít
# A name with `..` in it, or an absolute one.
import-scrivener-refusal-archive-unsafe-name-body = „{ $file }“ obsahuje soubor s názvem „{ $member }“, který by se ocitl mimo archiv, proto jej Skribisto neotevře.
import-scrivener-refusal-archive-unsafe-name-empty-body = „{ $file }“ obsahuje soubor bez názvu, proto jej Skribisto neotevře.
import-scrivener-refusal-archive-unsafe-name-control-character-body = „{ $file }“ obsahuje soubor, v jehož názvu je řídicí znak, proto jej Skribisto neotevře.
import-scrivener-refusal-archive-too-large-title = Tento archiv je příliš velký, než aby šel bezpečně otevřít
import-scrivener-refusal-archive-too-many-members-body = Obsahuje více než { $limit } souborů. Rozbalte jej sami a potom vyberte složku .scriv, kterou obsahuje.
import-scrivener-refusal-archive-member-too-large-body = „{ $member }“ se rozbalí na více než { $limit } MB. Rozbalte archiv sami a potom vyberte složku .scriv, kterou obsahuje.
import-scrivener-refusal-archive-too-large-body = Rozbalí se na více než { $limit } MB. Rozbalte jej sami a potom vyberte složku .scriv, kterou obsahuje.
import-scrivener-refusal-archive-encrypted-title = Tento archiv je chráněn heslem
import-scrivener-refusal-archive-encrypted-body = Rozbalte „{ $file }“ sami a potom vyberte složku .scriv, kterou obsahuje.
import-scrivener-refusal-archive-unsupported-compression-title = Tento archiv je zabalen způsobem, který Skribisto neumí přečíst
import-scrivener-refusal-archive-unsupported-compression-body = „{ $member }“ je komprimován metodou { $method }. Rozbalte archiv sami a potom vyberte složku .scriv, kterou obsahuje.
import-scrivener-refusal-duplicate-archive-members-title = Tento archiv uvádí některé soubory dvakrát
import-scrivener-refusal-duplicate-archive-members-body = „{ $file }“ obsahuje více souborů se stejným názvem, takže Skribisto nemůže určit, který z nich patří k projektu. Rozbalte jej sami a potom vyberte složku .scriv, kterou obsahuje.
# What a file cut short looks like; a file too large, one that is not a template and one
# encoded another way have their own words.
import-scrivener-refusal-template-undecodable-title = Tento soubor šablony je neúplný
import-scrivener-refusal-template-undecodable-too-large-title = Tento soubor šablony je příliš velký na to, aby šel otevřít
import-scrivener-refusal-template-undecodable-not-a-template-title = Tento soubor není šablona aplikace Scrivener
import-scrivener-refusal-template-undecodable-unknown-encoding-title = Tento soubor šablony je uložen v podobě, kterou Skribisto nezná
import-scrivener-refusal-template-undecodable-body = „{ $file }“ nelze dekódovat: možná končí dřív než projekt, který obsahuje. Stáhněte jej znovu, nebo jej otevřete v aplikaci Scrivener a vytvořte z něj projekt.
import-scrivener-refusal-template-undecodable-too-large-body = „{ $file }“ je větší než { $limit } MB. Otevřete jej v aplikaci Scrivener, vytvořte z něj projekt a potom importujte ten projekt.
import-scrivener-refusal-template-undecodable-not-a-template-body = „{ $file }“ neobsahuje projekt Scrivener. Vyberte soubor šablony, který uložil Scrivener, nebo projekt z něj vytvořený.
import-scrivener-refusal-template-undecodable-unknown-encoding-body = „{ $file }“ ukládá svůj projekt způsobem, který Skribisto neumí dekódovat. Otevřete jej v aplikaci Scrivener, vytvořte z něj projekt a potom importujte ten projekt.
import-scrivener-refusal-cloud-placeholders-title = { $count ->
    [one] 1 soubor tohoto projektu je stále v cloudu
    [few] { $count } soubory tohoto projektu jsou stále v cloudu
   *[other] { $count } souborů tohoto projektu je stále v cloudu
}
import-scrivener-refusal-cloud-placeholders-body = { $count ->
    [1] Nebyl stažen do tohoto počítače: { $examples }. Ve své synchronizační aplikaci zpřístupněte projekt offline a potom projekt načtěte znovu.
   *[other] Nebyly staženy do tohoto počítače, například { $examples }. Ve své synchronizační aplikaci zpřístupněte projekt offline a potom projekt načtěte znovu.
}
import-scrivener-refusal-unreadable-members-title = { $count ->
    [one] 1 soubor tohoto projektu nelze přečíst
    [few] { $count } soubory tohoto projektu nelze přečíst
   *[other] { $count } souborů tohoto projektu nelze přečíst
}
import-scrivener-refusal-unreadable-members-body = Například { $examples }. Ověřte, že máte oprávnění číst celý projekt a že je disk v pořádku, a potom projekt načtěte znovu.
import-scrivener-refusal-needed-member-is-link-title = Tento projekt odkazuje mimo svou vlastní složku
import-scrivener-refusal-needed-member-is-link-body = „{ $member }“ je odkaz na jiné místo v tomto počítači a Skribisto odkazy nesleduje. Zkopírujte projekt tak, aby byly všechny soubory uvnitř, a zkuste to znovu.
import-scrivener-refusal-project-changed-during-read-title = Projekt se během čtení změnil
import-scrivener-refusal-project-changed-during-read-body = Scrivener nebo synchronizační služba do něj stále zapisuje. Zavřete jej v aplikaci Scrivener, počkejte na dokončení synchronizace a potom projekt načtěte znovu.
import-scrivener-refusal-too-many-members-title = Tento projekt obsahuje příliš mnoho souborů
import-scrivener-refusal-too-many-members-body = Má více než { $limit } souborů, víc než kterýkoli skutečný projekt. Možná to není projekt Scrivener, nebo je poškozený.
import-scrivener-refusal-too-large-title = Tento projekt je příliš velký
import-scrivener-refusal-too-large-body = Jeho text má více než { $limit } MB, víc, než Skribisto dokáže bezpečně importovat.
import-scrivener-refusal-structure-unmappable-title = Skribisto tento pořadač neumí převést
import-scrivener-refusal-structure-unmappable-body = Pořadač nebylo možné převést, aniž by se něco ztratilo nebo ocitlo na špatném místě. Jde o chybu v aplikaci Skribisto, ne ve vašem projektu. Nahlaste ji prosím spolu s tím, co ukazují „Podrobnosti“.
## When there is a choice to make
import-scrivener-roots-title = { $count ->
    [one] Je tu { $count } projekt. Který z nich?
    [few] Jsou tu { $count } projekty. Který z nich?
   *[other] Je tu { $count } projektů. Který z nich?
}
import-scrivener-roots-hint = Tato složka nebo archiv obsahuje více projektů Scrivener. Vyberte ten, který chcete importovat.
import-scrivener-manifests-title = { $count ->
    [one] Tento projekt má { $count } soubor pořadače
    [few] Tento projekt má { $count } soubory pořadače
   *[other] Tento projekt má { $count } souborů pořadače
}
import-scrivener-manifests-hint = Scrivener uchovává pořadač, tedy seznam všech dokumentů a složek, v souboru .scrivx. Ty navíc nejspíš zanechala synchronizační služba. Vyberte ten, ve kterém jste naposledy pracovali.
import-scrivener-manifests-hint-preselected = Scrivener uchovává pořadač, tedy seznam všech dokumentů a složek, v souboru .scrivx. Ty navíc nejspíš zanechala synchronizační služba. Vybraný je ten, ve kterém jste nejspíš pracovali naposledy: než budete pokračovat, ověřte to.
import-scrivener-manifest-row = { $file } · uloženo { $date } · { $documents ->
    [one] 1 položka
    [few] { $documents } položky
   *[other] { $documents } položek
}
import-scrivener-manifest-row-undated = { $file } · { $documents ->
    [one] 1 položka
    [few] { $documents } položky
   *[other] { $documents } položek
}
import-scrivener-manifest-row-unread = { $file } · nelze přečíst
import-scrivener-enclosing-title = Toto je součást projektu Scrivener „{ $project }“
import-scrivener-enclosing-body = Vybrali jste soubor nebo složku uvnitř projektu. Skribisto importuje celý projekt.
import-scrivener-enclosing-use = Importovat „{ $project }“
import-scrivener-upgrade-title = Toto je kopie, kterou Scrivener uchoval před aktualizací
import-scrivener-upgrade-body = Když Scrivener aktualizuje projekt na novější verzi, uchová ten starý jako „{ $backup }“. Projekt, ve kterém jste dál psali, je „{ $current }“ vedle něj.
import-scrivener-upgrade-body-unclear = Když Scrivener aktualizuje projekt na novější verzi, uchová ten starý jako „{ $backup }“. Projekt vedle něj, „{ $current }“, nebylo možné bez potíží přečíst, proto vyberte, který z nich importovat.
import-scrivener-upgrade-use-current = Importovat „{ $current }“, projekt, ve kterém jste dál psali
import-scrivener-upgrade-keep-backup = Importovat tuto starší kopii, „{ $backup }“
## What must be accepted before going on
import-scrivener-ios-pending-title = Změny provedené v aplikaci Scrivener pro iPhone nebo iPad v této kopii ještě nejsou
import-scrivener-ios-pending-body = Scrivener v počítači tyto změny převezme, až projekt příště otevře. Otevřete projekt jednou v aplikaci Scrivener ve svém počítači a potom jej importujte znovu.
import-scrivener-ios-pending-accept = Importovat jej bez těchto změn
import-scrivener-no-text-title = { $count ->
    [one] 1 dokument, a v této kopii ani slovo textu
    [few] { $count } dokumenty, a v této kopii ani slovo textu
   *[other] { $count } dokumentů, a v této kopii ani slovo textu
}
import-scrivener-no-text-body = Synchronizační služba možná projekt ještě kopíruje, nebo kopie přišla o své dokumenty. Pokud ji importujete teď, všechny dokumenty budou prázdné.
import-scrivener-no-text-accept = Importovat dokumenty prázdné
## Things to know, nothing to decide
import-scrivener-notice-project-found-below-pick-title = Projekt byl nalezen uvnitř složky, kterou jste vybrali
import-scrivener-notice-project-found-below-pick-body = Je v „{ $path }“.
import-scrivener-notice-source-is-template-title = Toto je šablona aplikace Scrivener
import-scrivener-notice-source-is-template-named = Toto je šablona aplikace Scrivener „{ $title }“
import-scrivener-notice-source-is-template-body = Import z ní vytvoří nový projekt z toho, co šablona obsahuje, včetně jejích pokynů.
import-scrivener-notice-source-is-backup-title = Vypadá to jako záloha, kterou vytvořil Scrivener
import-scrivener-notice-source-is-backup-body = Obsahuje projekt tak, jak byl v okamžiku zálohování. Pokud jste potom dál psali, vyberte samotný projekt.
import-scrivener-notice-importing-pre-upgrade-copy-title = Importujete kopii, kterou Scrivener uchoval před aktualizací
import-scrivener-notice-importing-pre-upgrade-copy-body = Projekt, ve kterém jste dál psali, leží vedle ní a obsahuje vše, co jste od té doby napsali.
import-scrivener-notice-imported-from-ios-stub-title = Tento projekt vznikl v aplikaci Scrivener pro iPhone nebo iPad
import-scrivener-notice-imported-from-ios-stub-body = Nikdy nebyl otevřen v počítači, proto se čte z kopie, kterou uchovává iPhone nebo iPad.
import-scrivener-notice-stale-mobile-binder-ignored-title = Starý pořadač z iPhonu nebo iPadu zůstává stranou
import-scrivener-notice-stale-mobile-binder-ignored-body = Scrivener v počítači už jeho změny převzal.
import-scrivener-notice-identical-manifest-copies-title = { $count ->
    [one] Tento projekt má { $count } shodný soubor pořadače
    [few] Tento projekt má { $count } shodné soubory pořadače
   *[other] Tento projekt má { $count } shodných souborů pořadače
}
import-scrivener-notice-identical-manifest-copies-body = Obsahují totéž, takže kterákoli z nich dá stejný projekt.
import-scrivener-notice-manifest-copies-ignored-title = Nadbytečné kopie souboru pořadače zůstávají stranou
import-scrivener-notice-manifest-copies-ignored-body = „{ $chosen }“ je ten, ve kterém Scrivener pracoval naposledy. { $count ->
    [one] 1 kopie, kterou zanechala synchronizační služba, zůstává stranou.
    [few] { $count } kopie, které zanechala synchronizační služba, zůstávají stranou.
   *[other] { $count } kopií, které zanechala synchronizační služba, zůstává stranou.
}
import-scrivener-notice-pinned-manifest-looks-like-a-copy-title = Vybraný soubor pořadače vypadá jako kopie ze synchronizace
import-scrivener-notice-pinned-manifest-looks-like-a-copy-body = „{ $name }“ je pojmenován jako kopie souboru „{ $original }“, který leží vedle něj. Pokud bude import vypadat zastarale, vyberte místo něj „{ $original }“.
import-scrivener-notice-pinned-manifest-has-siblings-title = { $count ->
    [one] Vedle vybraného souboru pořadače leží 1 další
    [few] Vedle vybraného souboru pořadače leží { $count } další
   *[other] Vedle vybraného souboru pořadače leží { $count } dalších
}
import-scrivener-notice-pinned-manifest-has-siblings-body = Čte se jen ten, který jste vybrali.
import-scrivener-notice-legacy-v1-files-beside-title = Soubory ze starší verze aplikace Scrivener zůstávají stranou
import-scrivener-notice-legacy-v1-files-beside-body = Zůstaly tu po aktualizaci projektu. Čtou se jen soubory aktuální verze.
import-scrivener-notice-mixed-payload-trees-title = V tomto projektu jsou soubory dvou verzí aplikace Scrivener
import-scrivener-notice-mixed-payload-trees-body = Dokumenty se čtou z míst, která uvádí soubor pořadače.
import-scrivener-notice-version-disagreement-title = Soubory tohoto projektu se neshodují na jeho verzi
import-scrivener-notice-id-scheme-mismatch-body = Soubor pořadače uvádí verzi { $version }, ale své dokumenty pojmenovává jako jiná verze. Čtou se z míst, na která odkazuje. Pokud bude import vypadat špatně, otevřete projekt jednou v aplikaci Scrivener a importujte jej znovu.
import-scrivener-notice-version-stamp-mismatch-body = Čte se tak, jak jej popisuje jeho soubor pořadače. Pokud bude import vypadat špatně, otevřete projekt jednou v aplikaci Scrivener a importujte jej znovu.
import-scrivener-notice-unrecognised-writer-title = Tento projekt uložil program, který Skribisto nezná
import-scrivener-notice-unrecognised-writer-named = Sám se označuje jako „{ $creator }“. Čte se jako projekt Scrivener.
import-scrivener-notice-unrecognised-writer-body = Své jméno neuvádí. Čte se jako projekt Scrivener.
import-scrivener-notice-invalid-item-identifier-title = { $count ->
    [one] 1 položka pořadače má neplatný identifikátor
    [few] { $count } položky pořadače mají neplatný identifikátor
   *[other] { $count } položek pořadače má neplatný identifikátor
}
import-scrivener-notice-invalid-item-identifier-body = Takový identifikátor zůstane stranou a položka, která jej nese, se zachová.
import-scrivener-notice-items-without-identifier-title = { $count ->
    [one] 1 položka pořadače nemá identifikátor
    [few] { $count } položky pořadače nemají identifikátor
   *[other] { $count } položek pořadače nemá identifikátor
}
import-scrivener-notice-items-without-identifier-body = Bez identifikátoru nelze soubory položky najít, proto se přenese prázdná.
import-scrivener-notice-duplicate-item-key-title = Dvě položky pořadače sdílejí jeden identifikátor: „{ $item }“
import-scrivener-notice-duplicate-item-key-body = První si ponechá dokumenty. Druhá si ponechá své místo a bude prázdná.
import-scrivener-notice-lock-here-title = Scrivener má tento projekt otevřený v tomto počítači
import-scrivener-notice-lock-here-body = Nejprve jej v aplikaci Scrivener zavřete, aby se uložily všechny změny.
import-scrivener-notice-lock-title = Scrivener měl tento projekt otevřený v počítači „{ $host }“
import-scrivener-notice-lock-elsewhere-title = Scrivener měl tento projekt otevřený v jiném počítači
import-scrivener-notice-lock-body = Pokud je tam stále otevřený, nejprve jej zavřete: soubory možná ještě neobsahují nejnovější změny. Pokud byl Scrivener zavřen už dávno, můžete projekt importovat tak, jak je.
import-scrivener-notice-archived-while-open-title = Tento archiv vznikl, když měl Scrivener projekt otevřený
import-scrivener-notice-archived-while-open-body = Změny provedené po tomto okamžiku v něm možná chybějí.
import-scrivener-notice-content-missing-notes = Chybějí poznámky položky „{ $item }“
import-scrivener-notice-content-missing-synopsis = Chybí synopse položky „{ $item }“
import-scrivener-notice-content-missing-text = Chybí text položky „{ $item }“
import-scrivener-notice-content-missing-body = { $place ->
    [notes] Jejich soubor v této kopii není, proto se přenesou prázdné. Synchronizační služba možná projekt ještě kopíruje.
    [synopsis] Její soubor v této kopii není, proto se přenese prázdná. Synchronizační služba možná projekt ještě kopíruje.
   *[text] Jeho soubor v této kopii není, proto se přenese prázdný. Synchronizační služba možná projekt ještě kopíruje.
}
import-scrivener-notice-content-missing-salvaged = { $place ->
    [notes] Jejich soubor v této kopii není, ale vyhledávací index aplikace Scrivener ještě uchovával jejich slova. Zachovají se jako prostý text, možná ze starší verze.
    [synopsis] Její soubor v této kopii není, ale vyhledávací index aplikace Scrivener ještě uchovával její slova. Zachovají se jako prostý text, možná ze starší verze.
   *[text] Jeho soubor v této kopii není, ale vyhledávací index aplikace Scrivener ještě uchovával jeho slova. Zachovají se jako prostý text, možná ze starší verze.
}
import-scrivener-notice-content-truncated-notes = Poznámky položky „{ $item }“ jsou na disku prázdné
import-scrivener-notice-content-truncated-synopsis = Synopse položky „{ $item }“ je na disku prázdná
import-scrivener-notice-content-truncated-text = Text položky „{ $item }“ je na disku prázdný
import-scrivener-notice-content-truncated-body = { $place ->
    [notes] Podle aplikace Scrivener nebyly prázdné, ale jejich soubor neobsahuje žádný text: synchronizace se možná zastavila v polovině. Přenesou se prázdné.
    [synopsis] Podle aplikace Scrivener nebyla prázdná, ale její soubor neobsahuje žádný text: synchronizace se možná zastavila v polovině. Přenese se prázdná.
   *[text] Podle aplikace Scrivener nebyl prázdný, ale jeho soubor neobsahuje žádný text: synchronizace se možná zastavila v polovině. Přenese se prázdný.
}
import-scrivener-notice-missing-media-title = Chybí soubor položky „{ $item }“
import-scrivener-notice-missing-media-body = Obrázek nebo soubor z rešerší, který položka obsahuje, v této kopii není.
import-scrivener-notice-sync-conflict-copies-title = { $count ->
    [one] Byla nalezena 1 synchronizační kopie dokumentu
    [few] Byly nalezeny { $count } synchronizační kopie dokumentů
   *[other] Bylo nalezeno { $count } synchronizačních kopií dokumentů
}
import-scrivener-notice-sync-conflict-copies-body = { $missing ->
    [0] Synchronizační kopie se přenesou jako obnovené texty do vlastní složky, abyste je mohli porovnat s dokumenty.
    [one] Synchronizační kopie se přenesou jako obnovené texty do vlastní složky. { $missing } z nich je kopií dokumentu, který už v projektu není.
    [few] Synchronizační kopie se přenesou jako obnovené texty do vlastní složky. { $missing } z nich jsou kopiemi dokumentů, které už v projektu nejsou.
   *[other] Synchronizační kopie se přenesou jako obnovené texty do vlastní složky. { $missing } z nich je kopiemi dokumentů, které už v projektu nejsou.
}
import-scrivener-notice-recovered-unreferenced-text-title = { $count ->
    [one] Byl nalezen 1 dokument, který neuvádí žádná položka pořadače
    [few] Byly nalezeny { $count } dokumenty, které neuvádí žádná položka pořadače
   *[other] Bylo nalezeno { $count } dokumentů, které neuvádí žádná položka pořadače
}
import-scrivener-notice-recovered-unreferenced-text-body = Takové dokumenty se přenesou jako obnovené texty do vlastní složky.
import-scrivener-notice-sync-debris-ignored-title = { $count ->
    [one] 1 nadbytečná kopie, kterou zanechala synchronizační služba, zůstává stranou
    [few] { $count } nadbytečné kopie, které zanechala synchronizační služba, zůstávají stranou
   *[other] { $count } nadbytečných kopií, které zanechala synchronizační služba, zůstává stranou
}
import-scrivener-notice-sync-debris-ignored-body = Scrivener takové kopie také nečte. Čtou se jen vlastní soubory projektu.
import-scrivener-notice-unrecognised-files-title = { $count ->
    [one] 1 soubor, který není součástí projektu Scrivener, zůstává stranou
    [few] { $count } soubory, které nejsou součástí projektu Scrivener, zůstávají stranou
   *[other] { $count } souborů, které nejsou součástí projektu Scrivener, zůstává stranou
}
import-scrivener-notice-examples = Například { $examples }.
import-scrivener-notice-symlink-not-followed-title = { $count ->
    [one] 1 odkaz uvnitř projektu se nesleduje
    [few] { $count } odkazy uvnitř projektu se nesledují
   *[other] { $count } odkazů uvnitř projektu se nesleduje
}
import-scrivener-notice-unreadable-files-skipped-title = { $count ->
    [one] 1 soubor, který import nepotřebuje, nebylo možné přečíst
    [few] { $count } soubory, které import nepotřebuje, nebylo možné přečíst
   *[other] { $count } souborů, které import nepotřebuje, nebylo možné přečíst
}
import-scrivener-notice-cloud-placeholders-skipped-title = { $count ->
    [one] 1 soubor, který je stále v cloudu, se přeskakuje
    [few] { $count } soubory, které jsou stále v cloudu, se přeskakují
   *[other] { $count } souborů, které jsou stále v cloudu, se přeskakuje
}
import-scrivener-notice-cloud-placeholders-skipped-body = Nic, co import potřebuje, nezůstalo v cloudu.
import-scrivener-notice-no-text-found-title = { $count ->
    [1] 1 dokument, a ten neobsahuje žádný text
    [few] { $count } dokumenty, a žádný z nich neobsahuje text
   *[other] { $count } dokumentů, a žádný z nich neobsahuje text
}
import-scrivener-notice-no-text-found-body = Tento projekt vypadá prázdný. Importuje se tak, jak je.
import-scrivener-notice-metadata-unreadable-title = „{ $member }“ se nepodařilo přečíst
import-scrivener-notice-metadata-unreadable-body = To, co v něm Scrivener uchovával, se nepřenese.
import-scrivener-notice-dangling-keyword-title = { $count ->
    [one] 1 odkaz vede na klíčové slovo, které projekt nedefinuje
    [few] { $count } odkazy vedou na klíčová slova, která projekt nedefinuje
   *[other] { $count } odkazů vede na klíčová slova, která projekt nedefinuje
}
import-scrivener-notice-dangling-label-title = { $count ->
    [one] 1 odkaz vede na popisek, který projekt nedefinuje
    [few] { $count } odkazy vedou na popisky, které projekt nedefinuje
   *[other] { $count } odkazů vede na popisky, které projekt nedefinuje
}
import-scrivener-notice-dangling-status-title = { $count ->
    [one] 1 odkaz vede na stav, který projekt nedefinuje
    [few] { $count } odkazy vedou na stavy, které projekt nedefinuje
   *[other] { $count } odkazů vede na stavy, které projekt nedefinuje
}
import-scrivener-notice-dangling-section-type-title = { $count ->
    [one] 1 odkaz vede na typ oddílu, který projekt nedefinuje
    [few] { $count } odkazy vedou na typy oddílů, které projekt nedefinuje
   *[other] { $count } odkazů vede na typy oddílů, které projekt nedefinuje
}
import-scrivener-notice-dangling-custom-field-title = { $count ->
    [one] 1 odkaz vede na vlastní pole metadat, které projekt nedefinuje
    [few] { $count } odkazy vedou na vlastní pole metadat, která projekt nedefinuje
   *[other] { $count } odkazů vede na vlastní pole metadat, která projekt nedefinuje
}
import-scrivener-notice-dangling-item-title = { $count ->
    [one] 1 odkaz vede na dokument, který projekt nedefinuje
    [few] { $count } odkazy vedou na dokumenty, které projekt nedefinuje
   *[other] { $count } odkazů vede na dokumenty, které projekt nedefinuje
}
import-scrivener-notice-dangling-references-body = Odkaz na něco, co projekt nedefinuje, se vynechá.
import-scrivener-notice-snapshots-not-imported-line = { $count ->
    [one] 1 snímek zůstává ve vašem projektu Scrivener: Skribisto snímky nepřenáší.
    [few] { $count } snímky zůstávají ve vašem projektu Scrivener: Skribisto snímky nepřenáší.
   *[other] { $count } snímků zůstává ve vašem projektu Scrivener: Skribisto snímky nepřenáší.
}
import-scrivener-notice-writing-history-not-imported-line = { $days ->
    [one] Záznam aplikace Scrivener o slovech napsaných v jednotlivých dnech (1 den) se nepřenáší. Skribisto začne vlastní počítání od prvního uložení.
    [few] Záznam aplikace Scrivener o slovech napsaných v jednotlivých dnech ({ $days } dny) se nepřenáší. Skribisto začne vlastní počítání od prvního uložení.
   *[other] Záznam aplikace Scrivener o slovech napsaných v jednotlivých dnech ({ $days } dní) se nepřenáší. Skribisto začne vlastní počítání od prvního uložení.
}
import-scrivener-notice-title-placeholder-rejected-hint = Název v nastavení projektu je zástupnou značkou šablony, proto se místo něj navrhuje jméno projektu.
import-scrivener-notice-author-placeholder-rejected-hint = Jméno autora v nastavení projektu je zástupným textem šablony, proto se žádné nenavrhuje.
import-scrivener-notice-title-from-project-hint-compile-settings = Tento název uvádí nastavení kompilace projektu. Dokud tam není nastaven název, Scrivener v něm uchovává jméno, které projekt dostal při vytvoření.
import-scrivener-notice-title-from-project-hint-template = Toto je vlastní název šablony.
import-scrivener-notice-title-from-project-hint-folder-name = Tento název je jméno projektu.
import-scrivener-notice-title-from-project-hint-project-properties = Tento název uvádějí vlastnosti projektu.
import-scrivener-notice-author-from-project-hint = Toto jméno uvádí nastavení projektu. Může tam zůstat autor šablony a ten by se vytiskl na titulní stránku.
import-scrivener-notice-language-not-stated-hint = Projekt neuvádí svůj jazyk, proto se navrhuje jazyk, ve kterém se zobrazuje Skribisto. Jazyk určuje, jak se počítají slova a jak se píší uvozovky.
import-scrivener-notice-language-from-script-hint = Navrženo podle písma, kterým je rukopis napsán ({ $tag }).
## Before you import: what the import does with the binder, or leaves behind
import-scrivener-structure-script-mode-draft-line = { $script ->
    [1] { $documents ->
        [1] Jediný dokument rukopisu je napsán v režimu scénáře. Skribisto zachová jeho slova, ne jeho scenáristickou úpravu.
       *[other] { $script } z { $documents } dokumentů rukopisu je napsán v režimu scénáře. Skribisto zachová jeho slova, ne jeho scenáristickou úpravu.
    }
    [few] { $script } z { $documents } dokumentů rukopisu jsou napsány v režimu scénáře. Skribisto zachová jejich slova, ne jejich scenáristickou úpravu.
   *[other] { $script } z { $documents } dokumentů rukopisu je napsáno v režimu scénáře. Skribisto zachová jejich slova, ne jejich scenáristickou úpravu.
}
import-scrivener-structure-one-document-manuscript-line = { $count ->
    [1] Jeden dokument rukopisu obsahuje své kapitoly jako nadpisy.
    [few] { $count } dokumenty rukopisu obsahují své kapitoly jako nadpisy.
   *[other] { $count } dokumentů rukopisu obsahuje své kapitoly jako nadpisy.
}
import-scrivener-structure-title-repeats-number-line = { $count ->
    [one] 1 název kapitoly nebo části obsahuje i své číslo, například „Kapitola II“ nebo „První část“, takže export může číslo vytisknout dvakrát.
    [few] { $count } názvy kapitol nebo částí obsahují i své číslo, například „Kapitola II“ nebo „První část“, takže export může číslo vytisknout dvakrát.
   *[other] { $count } názvů kapitol nebo částí obsahuje i své číslo, například „Kapitola II“ nebo „První část“, takže export může číslo vytisknout dvakrát.
}
import-scrivener-structure-claim-overruled-line = { $count ->
    [one] 1 položka vypadá jako kniha, část nebo kapitola, ale leží uvnitř položky stejné úrovně, proto se přenese na nižší úroveň.
    [few] { $count } položky vypadají jako knihy, části nebo kapitoly, ale každá leží uvnitř položky stejné úrovně, proto se všechny přenesou na nižší úroveň.
   *[other] { $count } položek vypadá jako knihy, části nebo kapitoly, ale každá leží uvnitř položky stejné úrovně, proto se všechny přenesou na nižší úroveň.
}
import-scrivener-structure-adopted-by-chapter-line = { $count ->
    [one] 1 dokument, který následuje za kapitolou, se stane koncem této kapitoly.
    [few] { $count } dokumenty, které následují za kapitolou, se stanou jejím koncem.
   *[other] { $count } dokumentů, které následují za kapitolou, se stane jejím koncem.
}
import-scrivener-structure-looks-like-manuscript-line = { $count ->
    [one] 1 složka nejvyšší úrovně ponechaná v poznámkách vypadá jako rukopis. Můžete ji poslat do rukopisu jako další knihu.
    [few] { $count } složky nejvyšší úrovně ponechané v poznámkách vypadají jako rukopis. Můžete je poslat do rukopisu jako další knihy.
   *[other] { $count } složek nejvyšší úrovně ponechaných v poznámkách vypadá jako rukopis. Můžete je poslat do rukopisu jako další knihy.
}
import-scrivener-structure-generic-sheet-title-line = { $count ->
    [one] 1 záznam bible příběhu má stále název převzatý ze šablony, například „Hrdina“. Skribisto tento název hledá ve vašem textu a každé místo, kde se objeví, uvádí v zobrazení V próze: pokud je to ve vaší knize běžné slovo, záznam přejmenujte.
    [few] { $count } záznamy bible příběhu mají stále názvy převzaté ze šablony, například „Hrdina“. Skribisto tyto názvy hledá ve vašem textu a každé místo, kde se objeví, uvádí v zobrazení V próze: pokud jsou to ve vaší knize běžná slova, záznamy přejmenujte.
   *[other] { $count } záznamů bible příběhu má stále názvy převzaté ze šablony, například „Hrdina“. Skribisto tyto názvy hledá ve vašem textu a každé místo, kde se objeví, uvádí v zobrazení V próze: pokud jsou to ve vaší knize běžná slova, záznamy přejmenujte.
}
import-scrivener-structure-placeholder-title-line = { $count ->
    [one] 1 název jen pojmenovával svou úroveň a zůstane prázdný: očíslují jej pořadač a exporty.
    [few] { $count } názvy jen pojmenovávaly svou úroveň a zůstanou prázdné: očíslují je pořadač a exporty.
   *[other] { $count } názvů jen pojmenovávalo svou úroveň a zůstane prázdných: očíslují je pořadač a exporty.
}
import-scrivener-structure-untitled-item-line = { $count ->
    [one] 1 položka neměla v aplikaci Scrivener název a zůstane bez názvu.
    [few] { $count } položky neměly v aplikaci Scrivener název a zůstanou bez názvu.
   *[other] { $count } položek nemělo v aplikaci Scrivener název a zůstane bez názvu.
}
import-scrivener-structure-title-cleaned-line = { $count ->
    [one] 1 název obsahoval zalomení řádků nebo tabulátory, které se odstraní.
    [few] { $count } názvy obsahovaly zalomení řádků nebo tabulátory, které se odstraní.
   *[other] { $count } názvů obsahovalo zalomení řádků nebo tabulátory, které se odstraní.
}
import-scrivener-structure-own-text-moved-line = { $count ->
    [one] 1 složka má vlastní text, který se přesune do prvního dokumentu uvnitř ní.
    [few] { $count } složky mají vlastní text, který se přesune do prvního dokumentu uvnitř nich.
   *[other] { $count } složek má vlastní text, který se přesune do prvního dokumentu uvnitř nich.
}
import-scrivener-structure-text-with-children-line = { $count ->
    [one] 1 dokument, který obsahuje další dokumenty, se stane složkou.
    [few] { $count } dokumenty, které obsahují další dokumenty, se stanou složkami.
   *[other] { $count } dokumentů, které obsahují další dokumenty, se stane složkami.
}
import-scrivener-structure-empty-folder-plain-line = { $count ->
    [one] 1 prázdná složka zůstane obyčejnou složkou, aby žádný export nevytiskl nadpis, který Scrivener nikdy netiskl.
    [few] { $count } prázdné složky zůstanou obyčejnými složkami, aby žádný export nevytiskl nadpis, který Scrivener nikdy netiskl.
   *[other] { $count } prázdných složek zůstane obyčejnými složkami, aby žádný export nevytiskl nadpis, který Scrivener nikdy netiskl.
}
import-scrivener-structure-book-inside-book-line = { $count ->
    [one] 1 kniha uvnitř knihy se stane částí.
    [few] { $count } knihy uvnitř knihy se stanou částmi.
   *[other] { $count } knih uvnitř knihy se stane částmi.
}
import-scrivener-structure-prologue-not-numbered-line = { $count ->
    [one] 1 prolog nebo epilog se vynechá z číslování kapitol.
    [few] { $count } prology nebo epilogy se vynechají z číslování kapitol.
   *[other] { $count } prologů nebo epilogů se vynechá z číslování kapitol.
}
import-scrivener-structure-matter-inside-book-line = { $count ->
    [one] 1 položka úvodní nebo závěrečné části, která byla mimo rukopis, přejde do knihy jako paratext a do exportů se nedostane, dokud ji nezahrnete.
    [few] { $count } položky úvodní nebo závěrečné části, které byly mimo rukopis, přejdou do knihy jako paratext a do exportů se nedostanou, dokud je nezahrnete.
   *[other] { $count } položek úvodní nebo závěrečné části, které byly mimo rukopis, přejde do knihy jako paratext a do exportů se nedostane, dokud je nezahrnete.
}
import-scrivener-structure-paratext-not-counted-line = { $documents ->
    [one] 1 dokument rukopisu se stane paratextem (úvodní nebo závěrečnou částí), takže { $words ->
        [one] { $words } slovo se už nepočítá
        [few] { $words } slova se už nepočítají
       *[other] { $words } slov se už nepočítá
    } do rukopisu.
    [few] { $documents } dokumenty rukopisu se stanou paratextem (úvodní nebo závěrečnou částí), takže { $words ->
        [one] { $words } slovo se už nepočítá
        [few] { $words } slova se už nepočítají
       *[other] { $words } slov se už nepočítá
    } do rukopisu.
   *[other] { $documents } dokumentů rukopisu se stane paratextem (úvodní nebo závěrečnou částí), takže { $words ->
        [one] { $words } slovo se už nepočítá
        [few] { $words } slova se už nepočítají
       *[other] { $words } slov se už nepočítá
    } do rukopisu.
}
import-scrivener-structure-document-notes-line = { $count ->
    [one] Poznámky 1 dokumentu se stanou poznámkou vedle něj.
    [few] Poznámky { $count } dokumentů se stanou poznámkami vedle nich.
   *[other] Poznámky { $count } dokumentů se stanou poznámkami vedle nich.
}
import-scrivener-structure-draft-content-as-note-line = Vlastní text složky rukopisu se stane poznámkou na začátku rukopisu.
import-scrivener-structure-story-bible-folder-line = { $count ->
    [one] 1 složka postav nebo míst se stane štítkem bible příběhu.
    [few] { $count } složky postav nebo míst se stanou štítky bible příběhu.
   *[other] { $count } složek postav nebo míst se stane štítky bible příběhu.
}
import-scrivener-structure-templates-created-line = { $count ->
    [one] 1 šablona dokumentu se stane šablonou poznámky.
    [few] { $count } šablony dokumentů se stanou šablonami poznámek.
   *[other] { $count } šablon dokumentů se stane šablonami poznámek.
}
import-scrivener-structure-template-nesting-flattened-line = { $count ->
    [one] 1 složka šablon vnořená do jiné se zploští: každý text v ní se stane šablonou.
    [few] { $count } složky šablon vnořené do jiných se zploští: každý text v nich se stane šablonou.
   *[other] { $count } složek šablon vnořených do jiných se zploští: každý text v nich se stane šablonou.
}
import-scrivener-structure-template-kept-as-note-line = { $count ->
    [one] 1 šablona obsahuje to, co šablona poznámky pojmout nemůže, a zachová se jako poznámka.
    [few] { $count } šablony obsahují to, co šablona poznámky pojmout nemůže, a zachovají se jako poznámky.
   *[other] { $count } šablon obsahuje to, co šablona poznámky pojmout nemůže, a zachová se jako poznámky.
}
import-scrivener-structure-recovered-texts-line = { $count ->
    [one] 1 text, který neuváděla žádná položka pořadače, se přenese do vlastní složky.
    [few] { $count } texty, které neuváděla žádná položka pořadače, se přenesou do vlastní složky.
   *[other] { $count } textů, které neuváděla žádná položka pořadače, se přenese do vlastní složky.
}
import-scrivener-structure-project-notes-line = { $count ->
    [one] 1 poznámka k projektu se přenese do vlastní složky.
    [few] { $count } poznámky k projektu se přenesou do vlastní složky.
   *[other] { $count } poznámek k projektu se přenese do vlastní složky.
}
import-scrivener-structure-trash-brought-across-line = { $items ->
    [1] Koš se přenese: 1 položka, kterou můžete z koše obnovit.
    [few] Koš se přenese: { $items } položky, { $writing ->
        [0] všechny bez textu
       *[other] z toho { $writing } s textem
    }, které můžete z koše obnovit.
   *[other] Koš se přenese: { $items } položek, { $writing ->
        [0] všechny bez textu
       *[other] z toho { $writing } s textem
    }, které můžete z koše obnovit.
}
import-scrivener-structure-not-compiled-line = { $count ->
    [one] 1 položka, kterou Scrivener vynechával z kompilace, bude vynechána i z exportů.
    [few] { $count } položky, které Scrivener vynechával z kompilace, budou vynechány i z exportů.
   *[other] { $count } položek, které Scrivener vynechával z kompilace, bude vynecháno i z exportů.
}
import-scrivener-structure-media-in-draft-line = { $count ->
    [one] 1 obrázek nebo soubor v rukopisu se stane poznámkou, protože kniha jej obsahovat nemůže.
    [few] { $count } obrázky nebo soubory v rukopisu se stanou poznámkami, protože kniha je obsahovat nemůže.
   *[other] { $count } obrázků nebo souborů v rukopisu se stane poznámkami, protože kniha je obsahovat nemůže.
}
import-scrivener-structure-left-out-line = { $count ->
    [one] 1 složka nejvyšší úrovně se vynechá, s { $items ->
        [one] 1 položkou
       *[other] { $items } položkami
    }.
    [few] { $count } složky nejvyšší úrovně se vynechají, s { $items ->
        [one] 1 položkou
       *[other] { $items } položkami
    }.
   *[other] { $count } složek nejvyšší úrovně se vynechá, s { $items ->
        [one] 1 položkou
       *[other] { $items } položkami
    }.
}
import-scrivener-structure-trash-left-out-line = { $items ->
    [1] Koš zůstane stranou: 1 položka.
    [few] Koš zůstane stranou: { $items } položky, { $writing ->
        [0] všechny bez textu
       *[other] z toho { $writing } s textem
    }.
   *[other] Koš zůstane stranou: { $items } položek, { $writing ->
        [0] všechny bez textu
       *[other] z toho { $writing } s textem
    }.
}
import-scrivener-structure-card-image-left-behind-line = { $count ->
    [one] 1 obrázek kartičky u položky rukopisu zůstane stranou: vytiskl by se v knize.
    [few] { $count } obrázky kartiček u položek rukopisu zůstanou stranou: vytiskly by se v knize.
   *[other] { $count } obrázků kartiček u položek rukopisu zůstane stranou: vytiskly by se v knize.
}
import-scrivener-structure-default-template-not-linked-line = { $count ->
    [one] 1 složka měla výchozí šablonu pro nové dokumenty vytvořené v ní. V aplikaci Skribisto může šablonu nést jen štítek bible příběhu, proto se toto propojení nezachová.
    [few] { $count } složky měly výchozí šablonu pro nové dokumenty vytvořené v nich. V aplikaci Skribisto může šablonu nést jen štítek bible příběhu, proto se tato propojení nezachovají.
   *[other] { $count } složek mělo výchozí šablonu pro nové dokumenty vytvořené v nich. V aplikaci Skribisto může šablonu nést jen štítek bible příběhu, proto se tato propojení nezachovají.
}
import-scrivener-structure-page-break-not-kept-line = { $count ->
    [one] 1 nastavení „Page Break Before“ (zalomení stránky před) nemá v aplikaci Skribisto své místo a nezachová se.
    [few] { $count } nastavení „Page Break Before“ (zalomení stránky před) nemají v aplikaci Skribisto své místo a nezachovají se.
   *[other] { $count } nastavení „Page Break Before“ (zalomení stránky před) nemá v aplikaci Skribisto své místo a nezachová se.
}
import-scrivener-structure-compile-as-is-not-kept-line = { $count ->
    [one] 1 nastavení „Compile As-Is“ (kompilovat tak, jak je) nemá v aplikaci Skribisto obdobu a nezachová se.
    [few] { $count } nastavení „Compile As-Is“ (kompilovat tak, jak je) nemají v aplikaci Skribisto obdobu a nezachovají se.
   *[other] { $count } nastavení „Compile As-Is“ (kompilovat tak, jak je) nemá v aplikaci Skribisto obdobu a nezachová se.
}
## What the import did with the labels, the status list and the dates
import-scrivener-metadata-status-categories-guessed-line = { $count ->
    [one] 1 stav, který Skribisto nezná, dostane druh podle svého místa v seznamu. Zkontrolujte jej v Nastavení ▸ Projekt ▸ Stavy.
    [few] { $count } stavy, které Skribisto nezná, dostanou druh podle svého místa v seznamu. Zkontrolujte je v Nastavení ▸ Projekt ▸ Stavy.
   *[other] { $count } stavů, které Skribisto nezná, dostane druh podle svého místa v seznamu. Zkontrolujte je v Nastavení ▸ Projekt ▸ Stavy.
}
import-scrivener-metadata-dates-unreadable-line = { $count ->
    [one] Data 1 položky nebylo možné přečíst: nese datum importu.
    [few] Data { $count } položek nebylo možné přečíst: nesou datum importu.
   *[other] Data { $count } položek nebylo možné přečíst: nesou datum importu.
}
import-scrivener-metadata-labels-became-tags-line = { $tags ->
    [one] Z popisků aplikace Scrivener se stane 1 štítek, u { $rows ->
        [one] 1 položky
       *[other] { $rows } položek
    }.
    [few] Z popisků aplikace Scrivener se stanou { $tags } štítky, u { $rows ->
        [one] 1 položky
       *[other] { $rows } položek
    }.
   *[other] Z popisků aplikace Scrivener se stane { $tags } štítků, u { $rows ->
        [one] 1 položky
       *[other] { $rows } položek
    }.
}
import-scrivener-metadata-statuses-became-ladder-line = { $rungs ->
    [one] Seznam stavů z aplikace Scrivener se přenese jako seznam stavů projektu: { $rungs } stav, u { $rows ->
        [one] 1 položky
       *[other] { $rows } položek
    }.
    [few] Seznam stavů z aplikace Scrivener se přenese jako seznam stavů projektu: { $rungs } stavy, u { $rows ->
        [one] 1 položky
       *[other] { $rows } položek
    }.
   *[other] Seznam stavů z aplikace Scrivener se přenese jako seznam stavů projektu: { $rungs } stavů, u { $rows ->
        [one] 1 položky
       *[other] { $rows } položek
    }.
}
import-scrivener-metadata-ladder-seeded-line = { $count ->
    [1] Projekt neměl seznam stavů, proto začíná s výchozím stavem aplikace Skribisto.
    [few] Projekt neměl seznam stavů, proto začíná s { $count } výchozími stavy aplikace Skribisto.
   *[other] Projekt neměl seznam stavů, proto začíná s { $count } výchozími stavy aplikace Skribisto.
}
import-scrivener-metadata-label-renamed-line = Pole Label (popisek) se v aplikaci Scrivener jmenovalo „{ $title }“. Tento název je uveden v podrobnostech každého štítku.
import-scrivener-metadata-status-renamed-line = Pole Status (stav) se v aplikaci Scrivener jmenovalo „{ $title }“. Tento název je uveden v podrobnostech každého stavu.
import-scrivener-metadata-none-label-collapsed-line = { $count ->
    [one] Popisek „{ $name }“, uchovaný pod stejným názvem vedle prázdného popisku aplikace Scrivener, se u 1 položky čte jako žádný popisek.
   *[other] Popisek „{ $name }“, uchovaný pod stejným názvem vedle prázdného popisku aplikace Scrivener, se u { $count } položek čte jako žádný popisek.
}
import-scrivener-metadata-none-status-collapsed-line = { $count ->
    [one] Stav „{ $name }“, uchovaný pod stejným názvem vedle prázdného stavu aplikace Scrivener, se u 1 položky čte jako žádný stav.
   *[other] Stav „{ $name }“, uchovaný pod stejným názvem vedle prázdného stavu aplikace Scrivener, se u { $count } položek čte jako žádný stav.
}
import-scrivener-metadata-unnamed-label-line = Popisek bez názvu dostane název podle svého čísla, { $id }.
import-scrivener-metadata-unnamed-status-line = Stav bez názvu dostane název podle svého čísla, { $id }.
import-scrivener-metadata-tag-name-disambiguated-line = { $count ->
    [one] 1 štítek dostane nový název, protože jeho původní název už nese jiný štítek.
    [few] { $count } štítky dostanou nový název, protože jejich původní názvy už nesou jiné štítky.
   *[other] { $count } štítků dostane nový název, protože jejich původní názvy už nesou jiné štítky.
}
import-scrivener-metadata-rung-names-disambiguated-line = { $count ->
    [one] 1 stav má stejný název jako jiný, proto dostane za název číslo. Zkontrolujte jej v Nastavení ▸ Projekt ▸ Stavy.
    [few] { $count } stavy mají stejný název jako jiné, proto každý dostane za název číslo. Zkontrolujte je v Nastavení ▸ Projekt ▸ Stavy.
   *[other] { $count } stavů má stejný název jako jiné, proto každý dostane za název číslo. Zkontrolujte je v Nastavení ▸ Projekt ▸ Stavy.
}
import-scrivener-metadata-unused-labels-skipped-line = { $count ->
    [one] 1 popisek, který není přiřazen žádnému dokumentu, se neimportuje.
    [few] { $count } popisky, které nejsou přiřazeny žádnému dokumentu, se neimportují.
   *[other] { $count } popisků, které nejsou přiřazeny žádnému dokumentu, se neimportuje.
}
import-scrivener-metadata-labels-without-row-line = { $count ->
    [one] Popisek 1 položky, která v novém projektu nemá místo, se nepřenese.
    [few] Popisky { $count } položek, které v novém projektu nemají místo, se nepřenesou.
   *[other] Popisky { $count } položek, které v novém projektu nemají místo, se nepřenesou.
}
import-scrivener-metadata-statuses-without-row-line = { $count ->
    [one] Stav 1 položky, která v novém projektu nemá místo, se nepřenese.
    [few] Stavy { $count } položek, které v novém projektu nemají místo, se nepřenesou.
   *[other] Stavy { $count } položek, které v novém projektu nemají místo, se nepřenesou.
}
import-scrivener-metadata-unnamed-keyword-line = Klíčové slovo bez názvu dostane název podle svého čísla, { $id }.
import-scrivener-metadata-keywords-without-row-line = { $count ->
    [one] Klíčová slova 1 položky, která v novém projektu nemá místo, se nepřenesou.
   *[other] Klíčová slova { $count } položek, které v novém projektu nemají místo, se nepřenesou.
}
import-scrivener-metadata-custom-values-without-row-line = { $count ->
    [one] Vlastní metadata 1 položky, která v novém projektu nemá místo, se nepřenesou.
   *[other] Vlastní metadata { $count } položek, které v novém projektu nemají místo, se nepřenesou.
}
import-scrivener-metadata-bookmarks-without-row-line = { $count ->
    [one] Záložky 1 položky, která v novém projektu nemá místo, se nepřenesou.
   *[other] Záložky { $count } položek, které v novém projektu nemají místo, se nepřenesou.
}
import-scrivener-metadata-targets-without-row-line = { $count ->
    [one] Cíl 1 položky, která v novém projektu nemá místo, se nepřenese.
   *[other] Cíle { $count } položek, které v novém projektu nemají místo, se nepřenesou.
}
## What the import does with the keywords, custom metadata, collections, bookmarks, targets,
## the deadline, the auto-complete list and the icons. Each line is said twice: on the Review
## page before the import, then in its report, so it reads the same at both times. `$note`
## and `$folder` are the titles the import gives a note and its "From Scrivener" folder;
## `$day` is a day as YYYY-MM-DD.
import-scrivener-metadata-pov-hint-line = { $count ->
    [1] Popisek nebo hodnota pole { $values } odpovídá záznamu bible příběhu { $matches }. Pokud označuje úhel pohledu, nastavte jej v Inspektoru: import nic nepropojuje.
   *[other] Popisky nebo hodnoty polí { $values } odpovídají záznamům bible příběhu { $matches }. Pokud označují úhly pohledu, nastavte je v Inspektoru: import nic nepropojuje.
}
import-scrivener-metadata-checkbox-default-unknown-line = Pole se zaškrtávacím políčkem „{ $field }“ má výchozí hodnotu, kterou Skribisto neumí přečíst, takže nelze určit, které dokumenty jsou ve výchozím stavu zaškrtnuté. Jeho štítek nesou jen dokumenty zaškrtnuté jednotlivě.
import-scrivener-metadata-goals-in-other-unit-line = { $count ->
    [one] 1 cíl je nastaven v jiné jednotce počítání. Zachová se a zobrazí se, jakmile jednotku počítání přepnete v Nastavení ▸ Projekt ▸ Struktura.
    [few] { $count } cíle jsou nastaveny v jiné jednotce počítání. Zachovají se a zobrazí se, jakmile jednotku počítání přepnete v Nastavení ▸ Projekt ▸ Struktura.
   *[other] { $count } cílů je nastaveno v jiné jednotce počítání. Zachovají se a zobrazí se, jakmile jednotku počítání přepnete v Nastavení ▸ Projekt ▸ Struktura.
}
import-scrivener-metadata-counting-differs-line = Scrivener počítal cíl rukopisu z jiných dokumentů než Skribisto. Cíl knihy počítá scény zahrnuté do exportů.
import-scrivener-metadata-writing-days-not-read-line = Psací dny „{ $raw }“ nelze přečíst, proto Tempo knihy počítá každý den jako psací. Změňte je tam.
import-scrivener-metadata-script-mode-rows-line = { $count ->
    [one] 1 položka byla v aplikaci Scrivener v režimu scénáře. Skribisto zachová její slova jako prózu, bez scenáristické úpravy.
    [few] { $count } položky byly v aplikaci Scrivener v režimu scénáře. Skribisto zachová jejich slova jako prózu, bez scenáristické úpravy.
   *[other] { $count } položek bylo v aplikaci Scrivener v režimu scénáře. Skribisto zachová jejich slova jako prózu, bez scenáristické úpravy.
}
import-scrivener-metadata-keywords-became-tags-line = { $tags ->
    [one] 1 klíčové slovo se stane štítkem, u { $rows ->
        [one] 1 položky
       *[other] { $rows } položek
    }.
    [few] { $tags } klíčová slova se stanou štítky, u { $rows ->
        [one] 1 položky
       *[other] { $rows } položek
    }.
   *[other] { $tags } klíčových slov se stane štítky, u { $rows ->
        [one] 1 položky
       *[other] { $rows } položek
    }.
}
import-scrivener-metadata-keyword-hierarchy-flattened-line = { $count ->
    [one] 1 klíčové slovo bylo v aplikaci Scrivener vnořeno do jiného. Štítky jsou ploché: podrobnosti jeho štítku uvádějí, kde leželo.
    [few] { $count } klíčová slova byla v aplikaci Scrivener vnořena do jiných. Štítky jsou ploché: podrobnosti každého štítku uvádějí, kde leželo jeho klíčové slovo.
   *[other] { $count } klíčových slov bylo v aplikaci Scrivener vnořeno do jiných. Štítky jsou ploché: podrobnosti každého štítku uvádějí, kde leželo jeho klíčové slovo.
}
import-scrivener-metadata-fields-became-tags-line = { $fields ->
    [one] 1 pole se seznamem nebo zaškrtávacím políčkem se převede na { $tags ->
        [one] 1 štítek
        [few] { $tags } štítky
       *[other] { $tags } štítků
    }, u { $rows ->
        [one] 1 položky
       *[other] { $rows } položek
    }.
    [few] { $fields } pole se seznamem nebo zaškrtávacím políčkem se převedou na { $tags ->
        [one] 1 štítek
        [few] { $tags } štítky
       *[other] { $tags } štítků
    }, u { $rows ->
        [one] 1 položky
       *[other] { $rows } položek
    }.
   *[other] { $fields } polí se seznamem nebo zaškrtávacím políčkem se převede na { $tags ->
        [one] 1 štítek
        [few] { $tags } štítky
       *[other] { $tags } štítků
    }, u { $rows ->
        [one] 1 položky
       *[other] { $rows } položek
    }.
}
import-scrivener-metadata-collections-became-tags-line = { $tags ->
    [one] 1 kolekce se stane štítkem, u { $rows ->
        [one] 1 položky
       *[other] { $rows } položek
    }.
    [few] { $tags } kolekce se stanou štítky, u { $rows ->
        [one] 1 položky
       *[other] { $rows } položek
    }.
   *[other] { $tags } kolekcí se stane štítky, u { $rows ->
        [one] 1 položky
       *[other] { $rows } položek
    }.
}
import-scrivener-metadata-collection-order-lost-line = { $count ->
    [one] Štítek nemá žádné pořadí, proto se pořadí dokumentů 1 kolekce nezachová.
   *[other] Štítek nemá žádné pořadí, proto se pořadí dokumentů { $count } kolekcí nezachová.
}
import-scrivener-metadata-bookmarks-became-links-line = { $count ->
    [one] 1 záložka se stane odkazem mezi dvěma poznámkami nebo mezi scénou a záznamem bible příběhu: viz Odkazy a Postavy.
    [few] { $count } záložky se stanou odkazy mezi poznámkami nebo mezi scénami a záznamy bible příběhu: viz Odkazy a Postavy.
   *[other] { $count } záložek se stane odkazy mezi poznámkami nebo mezi scénami a záznamy bible příběhu: viz Odkazy a Postavy.
}
import-scrivener-metadata-goals-kept-line = { $count ->
    [one] Cíl, který Scrivener nastavil 1 dokumentu, se v aplikaci Skribisto stane jeho cílem.
    [few] Cíle, které Scrivener nastavil { $count } dokumentům, se v aplikaci Skribisto stanou jejich cíli.
   *[other] Cíle, které Scrivener nastavil { $count } dokumentům, se v aplikaci Skribisto stanou jejich cíli.
}
import-scrivener-metadata-draft-target-kept-words-line = { $value ->
    [one] Cíl rukopisu (1 slovo) se stane cílem knihy.
    [few] Cíl rukopisu ({ $value } slova) se stane cílem knihy.
   *[other] Cíl rukopisu ({ $value } slov) se stane cílem knihy.
}
import-scrivener-metadata-draft-target-kept-characters-line = { $value ->
    [one] Cíl rukopisu (1 znak) se stane cílem knihy.
    [few] Cíl rukopisu ({ $value } znaky) se stane cílem knihy.
   *[other] Cíl rukopisu ({ $value } znaků) se stane cílem knihy.
}
import-scrivener-metadata-pace-planned-line = Termín { $day } nastaví Tempo knihy, které ode dne importu rozvrhne slova na každý psací den, který jste zvolili v aplikaci Scrivener.
import-scrivener-metadata-pace-planned-day-off-line = Termín { $day } nastaví Tempo knihy, které ode dne importu rozvrhne slova na každý psací den, který jste zvolili v aplikaci Scrivener. Stejně jako v aplikaci Scrivener je samotný den termínu volným dnem.
import-scrivener-metadata-dictionary-words-line = { $count ->
    [one] 1 slovo pro automatické doplňování přejde do osobního slovníku projektu, takže nebude označeno jako chybné.
    [few] { $count } slova pro automatické doplňování přejdou do osobního slovníku projektu, takže nebudou označena jako chybná.
   *[other] { $count } slov pro automatické doplňování přejde do osobního slovníku projektu, takže nebudou označena jako chybná.
}
import-scrivener-metadata-listed-project-line = { $count ->
    [one] 1 údaj o projektu je uveden v poznámce „{ $note }“ ve složce „{ $folder }“.
    [few] { $count } údaje o projektu jsou uvedeny v poznámce „{ $note }“ ve složce „{ $folder }“.
   *[other] { $count } údajů o projektu je uvedeno v poznámce „{ $note }“ ve složce „{ $folder }“.
}
import-scrivener-metadata-listed-project-bookmarks-line = { $count ->
    [one] 1 záložka projektu je uvedena v poznámce „{ $note }“ ve složce „{ $folder }“.
    [few] { $count } záložky projektu jsou uvedeny v poznámce „{ $note }“ ve složce „{ $folder }“.
   *[other] { $count } záložek projektu je uvedeno v poznámce „{ $note }“ ve složce „{ $folder }“.
}
import-scrivener-metadata-listed-replacements-line = { $count ->
    [one] 1 náhrada při kompilaci je uvedena v poznámce „{ $note }“ ve složce „{ $folder }“. Skribisto ji neprovádí.
    [few] { $count } náhrady při kompilaci jsou uvedeny v poznámce „{ $note }“ ve složce „{ $folder }“. Skribisto je neprovádí.
   *[other] { $count } náhrad při kompilaci je uvedeno v poznámce „{ $note }“ ve složce „{ $folder }“. Skribisto je neprovádí.
}
import-scrivener-metadata-listed-auto-complete-line = { $count ->
    [one] 1 víceslovná položka automatického doplňování je uvedena v poznámce „{ $note }“ ve složce „{ $folder }“.
    [few] { $count } víceslovné položky automatického doplňování jsou uvedeny v poznámce „{ $note }“ ve složce „{ $folder }“.
   *[other] { $count } víceslovných položek automatického doplňování je uvedeno v poznámce „{ $note }“ ve složce „{ $folder }“.
}
import-scrivener-metadata-listed-saved-searches-line = { $count ->
    [one] 1 uložené hledání je uvedeno v poznámce „{ $note }“ ve složce „{ $folder }“.
    [few] { $count } uložená hledání jsou uvedena v poznámce „{ $note }“ ve složce „{ $folder }“.
   *[other] { $count } uložených hledání je uvedeno v poznámce „{ $note }“ ve složce „{ $folder }“.
}
import-scrivener-metadata-listed-custom-metadata-line = { $count ->
    [one] 1 hodnota vlastních metadat, kterou štítek nedokáže pojmout, je uvedena v poznámce „{ $note }“ ve složce „{ $folder }“.
    [few] { $count } hodnoty vlastních metadat, které štítek nedokáže pojmout, jsou uvedeny v poznámce „{ $note }“ ve složce „{ $folder }“.
   *[other] { $count } hodnot vlastních metadat, které štítek nedokáže pojmout, je uvedeno v poznámce „{ $note }“ ve složce „{ $folder }“.
}
import-scrivener-metadata-listed-targets-line = { $count ->
    [one] 1 cíl dokumentu je uveden v poznámce „{ $note }“ ve složce „{ $folder }“.
    [few] { $count } cíle dokumentů jsou uvedeny v poznámce „{ $note }“ ve složce „{ $folder }“.
   *[other] { $count } cílů dokumentů je uvedeno v poznámce „{ $note }“ ve složce „{ $folder }“.
}
import-scrivener-metadata-listed-bookmarks-line = { $count ->
    [one] 1 záložka, kterou žádný odkaz v aplikaci Skribisto nezobrazí, je uvedena v poznámce „{ $note }“ ve složce „{ $folder }“.
    [few] { $count } záložky, které žádný odkaz v aplikaci Skribisto nezobrazí, jsou uvedeny v poznámce „{ $note }“ ve složce „{ $folder }“.
   *[other] { $count } záložek, které žádný odkaz v aplikaci Skribisto nezobrazí, je uvedeno v poznámce „{ $note }“ ve složce „{ $folder }“.
}
import-scrivener-metadata-targets-on-notes-line = { $count ->
    [one] 1 cíl dokumentu patří k poznámce, jejíž text žádný cíl nepočítá, takže se nepřevezme.
    [few] { $count } cíle dokumentů patří k poznámkám, jejichž text žádný cíl nepočítá, takže se nepřevezmou.
   *[other] { $count } cílů dokumentů patří k poznámkám, jejichž text žádný cíl nepočítá, takže se nepřevezme.
}
import-scrivener-metadata-targets-on-split-documents-line = { $count ->
    [one] 1 cíl dokumentu patří k dokumentu rozdělenému podle nadpisů, jehož kapitoly dohromady nepočítá žádný jednotlivý cíl, takže se nepřevezme.
    [few] { $count } cíle dokumentů patří k dokumentům rozděleným podle nadpisů, jejichž kapitoly dohromady nepočítá žádný jednotlivý cíl, takže se nepřevezmou.
   *[other] { $count } cílů dokumentů patří k dokumentům rozděleným podle nadpisů, jejichž kapitoly dohromady nepočítá žádný jednotlivý cíl, takže se nepřevezme.
}
import-scrivener-metadata-bookmark-ends-without-row-line = { $count ->
    [one] 1 záložka vede na dokument, který v novém projektu nemá místo, například na šablonu dokumentu. Je uvedena podle toho, kde dokument v aplikaci Scrivener ležel.
    [few] { $count } záložky vedou na dokumenty, které v novém projektu nemají místo, například na šablony dokumentů. Jsou uvedeny podle toho, kde dokumenty v aplikaci Scrivener ležely.
   *[other] { $count } záložek vede na dokumenty, které v novém projektu nemají místo, například na šablony dokumentů. Jsou uvedeny podle toho, kde dokumenty v aplikaci Scrivener ležely.
}
import-scrivener-metadata-unused-keywords-skipped-line = { $count ->
    [one] 1 klíčové slovo, které není přiřazeno žádnému dokumentu, se neimportuje.
    [few] { $count } klíčová slova, která nejsou přiřazena žádnému dokumentu, se neimportují.
   *[other] { $count } klíčových slov, která nejsou přiřazena žádnému dokumentu, se neimportuje.
}
import-scrivener-metadata-list-value-invalid-line = { $count ->
    [one] 1 hodnota pole se seznamem „{ $field }“ neodpovídá žádné z jeho možností a neimportuje se.
    [few] { $count } hodnoty pole se seznamem „{ $field }“ neodpovídají žádné z jeho možností a neimportují se.
   *[other] { $count } hodnot pole se seznamem „{ $field }“ neodpovídá žádné z jeho možností a neimportuje se.
}
import-scrivener-metadata-undefined-custom-field-line = { $count ->
    [one] 1 hodnota patří k vlastnímu poli, které projekt už nedefinuje a které nezobrazuje ani Scrivener. Neimportuje se.
    [few] { $count } hodnoty patří k vlastním polím, která projekt už nedefinuje a která nezobrazuje ani Scrivener. Neimportují se.
   *[other] { $count } hodnot patří k vlastním polím, která projekt už nedefinuje a která nezobrazuje ani Scrivener. Neimportují se.
}
import-scrivener-metadata-collection-members-missing-line = { $count ->
    [one] 1 dokument kolekce „{ $name }“ nemá v novém projektu místo, nebo už v projektu Scrivener není.
    [few] { $count } dokumenty kolekce „{ $name }“ nemají v novém projektu místo, nebo už v projektu Scrivener nejsou.
   *[other] { $count } dokumentů kolekce „{ $name }“ nemá v novém projektu místo, nebo už v projektu Scrivener nejsou.
}
import-scrivener-metadata-empty-collections-skipped-line = { $count ->
    [one] 1 kolekce, která neobsahuje žádný dokument, se neimportuje.
    [few] { $count } kolekce, které neobsahují žádný dokument, se neimportují.
   *[other] { $count } kolekcí, které neobsahují žádný dokument, se neimportuje.
}
import-scrivener-metadata-bookmarks-unresolved-line = { $count ->
    [one] 1 záložka vede na dokument, který projekt Scrivener už neobsahuje, a neimportuje se.
    [few] { $count } záložky vedou na dokumenty, které projekt Scrivener už neobsahuje, a neimportují se.
   *[other] { $count } záložek vede na dokumenty, které projekt Scrivener už neobsahuje, a neimportuje se.
}
import-scrivener-metadata-targets-in-pages-line = { $count ->
    [one] 1 cíl dokumentu počítá stránky, které Skribisto nepočítá, takže se nepřevezme.
    [few] { $count } cíle dokumentů počítají stránky, které Skribisto nepočítá, takže se nepřevezmou.
   *[other] { $count } cílů dokumentů počítá stránky, které Skribisto nepočítá, takže se nepřevezme.
}
import-scrivener-metadata-targets-unreadable-line = { $count ->
    [one] 1 cíl se neimportuje: jeho hodnota není číslo, je záporná, nebo je větší, než cíl dokáže pojmout.
    [few] { $count } cíle se neimportují: jejich hodnoty nejsou čísla, jsou záporné, nebo jsou větší, než cíl dokáže pojmout.
   *[other] { $count } cílů se neimportuje: jejich hodnoty nejsou čísla, jsou záporné, nebo jsou větší, než cíl dokáže pojmout.
}
import-scrivener-metadata-draft-target-in-pages-line = { $value ->
    [one] Cíl rukopisu (1 strana) se nepřevezme: Skribisto počítá slova nebo znaky. Je uveden v poznámce „{ $note }“ ve složce „{ $folder }“.
    [few] Cíl rukopisu ({ $value } strany) se nepřevezme: Skribisto počítá slova nebo znaky. Je uveden v poznámce „{ $note }“ ve složce „{ $folder }“.
   *[other] Cíl rukopisu ({ $value } stran) se nepřevezme: Skribisto počítá slova nebo znaky. Je uveden v poznámce „{ $note }“ ve složce „{ $folder }“.
}
import-scrivener-metadata-draft-target-no-book-words-line = { $value ->
    [one] Cíl rukopisu (1 slovo) se nepřevezme, protože rukopis tvoří několik knih, nebo žádnou. Je uveden v poznámce „{ $note }“ ve složce „{ $folder }“.
    [few] Cíl rukopisu ({ $value } slova) se nepřevezme, protože rukopis tvoří několik knih, nebo žádnou. Je uveden v poznámce „{ $note }“ ve složce „{ $folder }“.
   *[other] Cíl rukopisu ({ $value } slov) se nepřevezme, protože rukopis tvoří několik knih, nebo žádnou. Je uveden v poznámce „{ $note }“ ve složce „{ $folder }“.
}
import-scrivener-metadata-draft-target-no-book-characters-line = { $value ->
    [one] Cíl rukopisu (1 znak) se nepřevezme, protože rukopis tvoří několik knih, nebo žádnou. Je uveden v poznámce „{ $note }“ ve složce „{ $folder }“.
    [few] Cíl rukopisu ({ $value } znaky) se nepřevezme, protože rukopis tvoří několik knih, nebo žádnou. Je uveden v poznámce „{ $note }“ ve složce „{ $folder }“.
   *[other] Cíl rukopisu ({ $value } znaků) se nepřevezme, protože rukopis tvoří několik knih, nebo žádnou. Je uveden v poznámce „{ $note }“ ve složce „{ $folder }“.
}
import-scrivener-metadata-deadline-passed-line = Termín { $day } už uplynul, takže Tempo knihy nenastaví. Je uveden v poznámce „{ $note }“ ve složce „{ $folder }“.
import-scrivener-metadata-deadline-no-writing-day-line = Ode dne importu do termínu { $day } nezbývá žádný psací den, takže termín Tempo knihy nenastaví. Je uveden v poznámce „{ $note }“ ve složce „{ $folder }“.
import-scrivener-metadata-deadline-template-line = Termín { $day } byl uložen v šabloně, takže Tempo knihy nenastaví. Je uveden v poznámce „{ $note }“ ve složce „{ $folder }“.
import-scrivener-metadata-deadline-no-word-goal-line = Termín { $day } Tempo knihy nenastaví, protože rukopis nemá cílový počet slov, ke kterému by se dalo plánovat. Je uveden v poznámce „{ $note }“ ve složce „{ $folder }“.
import-scrivener-metadata-deadline-character-goal-line = Termín { $day } Tempo knihy nenastaví, protože cíl rukopisu počítá znaky, zatímco Tempo počítá slova. Je uveden v poznámce „{ $note }“ ve složce „{ $folder }“.
import-scrivener-metadata-deadline-no-book-line = Termín { $day } nenastaví Tempo žádné knihy, protože rukopis tvoří několik knih, nebo žádnou. Je uveden v poznámce „{ $note }“ ve složce „{ $folder }“.
import-scrivener-metadata-deadline-unreadable-line = Termín nelze přečíst, takže Tempo knihy nenastaví. Je uveden tak, jak byl zapsán, v poznámce „{ $note }“ ve složce „{ $folder }“.
import-scrivener-metadata-session-target-words-line = { $value ->
    [one] Cíl sezení (1 slovo) se neimportuje: v aplikaci Skribisto je cíl psacího sezení jedno nastavení pro všechny projekty. Je uveden v poznámce „{ $note }“ ve složce „{ $folder }“.
    [few] Cíl sezení ({ $value } slova) se neimportuje: v aplikaci Skribisto je cíl psacího sezení jedno nastavení pro všechny projekty. Je uveden v poznámce „{ $note }“ ve složce „{ $folder }“.
   *[other] Cíl sezení ({ $value } slov) se neimportuje: v aplikaci Skribisto je cíl psacího sezení jedno nastavení pro všechny projekty. Je uveden v poznámce „{ $note }“ ve složce „{ $folder }“.
}
import-scrivener-metadata-session-target-characters-line = { $value ->
    [one] Cíl sezení (1 znak) se neimportuje: v aplikaci Skribisto je cíl psacího sezení jedno nastavení pro všechny projekty, ve slovech. Je uveden v poznámce „{ $note }“ ve složce „{ $folder }“.
    [few] Cíl sezení ({ $value } znaky) se neimportuje: v aplikaci Skribisto je cíl psacího sezení jedno nastavení pro všechny projekty, ve slovech. Je uveden v poznámce „{ $note }“ ve složce „{ $folder }“.
   *[other] Cíl sezení ({ $value } znaků) se neimportuje: v aplikaci Skribisto je cíl psacího sezení jedno nastavení pro všechny projekty, ve slovech. Je uveden v poznámce „{ $note }“ ve složce „{ $folder }“.
}
import-scrivener-metadata-session-target-pages-line = { $value ->
    [one] Cíl sezení (1 strana) se neimportuje: v aplikaci Skribisto je cíl psacího sezení jedno nastavení pro všechny projekty, ve slovech. Je uveden v poznámce „{ $note }“ ve složce „{ $folder }“.
    [few] Cíl sezení ({ $value } strany) se neimportuje: v aplikaci Skribisto je cíl psacího sezení jedno nastavení pro všechny projekty, ve slovech. Je uveden v poznámce „{ $note }“ ve složce „{ $folder }“.
   *[other] Cíl sezení ({ $value } stran) se neimportuje: v aplikaci Skribisto je cíl psacího sezení jedno nastavení pro všechny projekty, ve slovech. Je uveden v poznámce „{ $note }“ ve složce „{ $folder }“.
}
import-scrivener-metadata-session-target-calculated-words-line = { $value ->
    [one] Cíl sezení (1 slovo), který Scrivener vypočítal z termínu a psacích dnů, se neimportuje: v aplikaci Skribisto je cíl psacího sezení jedno nastavení pro všechny projekty. Je uveden v poznámce „{ $note }“ ve složce „{ $folder }“.
    [few] Cíl sezení ({ $value } slova), který Scrivener vypočítal z termínu a psacích dnů, se neimportuje: v aplikaci Skribisto je cíl psacího sezení jedno nastavení pro všechny projekty. Je uveden v poznámce „{ $note }“ ve složce „{ $folder }“.
   *[other] Cíl sezení ({ $value } slov), který Scrivener vypočítal z termínu a psacích dnů, se neimportuje: v aplikaci Skribisto je cíl psacího sezení jedno nastavení pro všechny projekty. Je uveden v poznámce „{ $note }“ ve složce „{ $folder }“.
}
import-scrivener-metadata-session-target-calculated-characters-line = { $value ->
    [one] Cíl sezení (1 znak), který Scrivener vypočítal z termínu a psacích dnů, se neimportuje: v aplikaci Skribisto je cíl psacího sezení jedno nastavení pro všechny projekty, ve slovech. Je uveden v poznámce „{ $note }“ ve složce „{ $folder }“.
    [few] Cíl sezení ({ $value } znaky), který Scrivener vypočítal z termínu a psacích dnů, se neimportuje: v aplikaci Skribisto je cíl psacího sezení jedno nastavení pro všechny projekty, ve slovech. Je uveden v poznámce „{ $note }“ ve složce „{ $folder }“.
   *[other] Cíl sezení ({ $value } znaků), který Scrivener vypočítal z termínu a psacích dnů, se neimportuje: v aplikaci Skribisto je cíl psacího sezení jedno nastavení pro všechny projekty, ve slovech. Je uveden v poznámce „{ $note }“ ve složce „{ $folder }“.
}
import-scrivener-metadata-session-target-calculated-pages-line = { $value ->
    [one] Cíl sezení (1 strana), který Scrivener vypočítal z termínu a psacích dnů, se neimportuje: v aplikaci Skribisto je cíl psacího sezení jedno nastavení pro všechny projekty, ve slovech. Je uveden v poznámce „{ $note }“ ve složce „{ $folder }“.
    [few] Cíl sezení ({ $value } strany), který Scrivener vypočítal z termínu a psacích dnů, se neimportuje: v aplikaci Skribisto je cíl psacího sezení jedno nastavení pro všechny projekty, ve slovech. Je uveden v poznámce „{ $note }“ ve složce „{ $folder }“.
   *[other] Cíl sezení ({ $value } stran), který Scrivener vypočítal z termínu a psacích dnů, se neimportuje: v aplikaci Skribisto je cíl psacího sezení jedno nastavení pro všechny projekty, ve slovech. Je uveden v poznámce „{ $note }“ ve složce „{ $folder }“.
}
import-scrivener-metadata-icons-dropped-line = { $count ->
    [one] 1 položka měla v pořadači ikonu, která se nepřenese: Skribisto zobrazuje ikonu každé položky podle jejího typu.
    [few] { $count } položky měly v pořadači ikony, které se nepřenesou: Skribisto zobrazuje ikonu každé položky podle jejího typu.
   *[other] { $count } položek mělo v pořadači ikony, které se nepřenesou: Skribisto zobrazuje ikonu každé položky podle jejího typu.
}
## What converting the prose did, as the import's report says it. `$count` is how many texts,
## or how many of a thing, as each line says; `$example` quotes the project.
import-scrivener-prose-unreadable-line = { $count ->
    [one] 1 text nebylo možné přečíst. Bude prázdný.
    [few] { $count } texty nebylo možné přečíst. Budou prázdné.
   *[other] { $count } textů nebylo možné přečíst. Budou prázdné.
}
import-scrivener-prose-comments-unreadable-line = { $count ->
    [one] Komentáře a propojené poznámky pod čarou 1 textu nebylo možné přečíst. Slova, která označovaly, se zachovají.
   *[other] Komentáře a propojené poznámky pod čarou { $count } textů nebylo možné přečíst. Slova, která označovaly, se zachovají.
}
import-scrivener-prose-read-as-plain-text-line = { $count ->
    [one] 1 text byl uložen jako prostý text, nikoli jako formátovaný, a tak se i čte.
    [few] { $count } texty byly uloženy jako prostý text, nikoli jako formátovaný, a tak se i čtou.
   *[other] { $count } textů bylo uloženo jako prostý text, nikoli jako formátovaný, a tak se i čte.
}
import-scrivener-prose-inline-note-unmatched-line = { $count ->
    [one] 1 značka vložené poznámky nemá svůj protějšek a zachová se jako text.
    [few] { $count } značky vložených poznámek nemají svůj protějšek a zachovají se jako text.
   *[other] { $count } značek vložených poznámek nemá svůj protějšek a zachová se jako text.
}
import-scrivener-prose-style-unresolved-line = { $count ->
    [one] 1 značka stylu odkazuje na styl, který projekt nedefinuje. Odstraní se a text si ponechá své formátování.
    [few] { $count } značky stylu odkazují na styl, který projekt nedefinuje. Odstraní se a text si ponechá své formátování.
   *[other] { $count } značek stylu odkazuje na styl, který projekt nedefinuje. Odstraní se a text si ponechá své formátování.
}
import-scrivener-prose-styles-lost-in-export-line = { $count ->
    [one] 1 odstavec může být nadpis, který ztratil svůj styl, když Scrivener 3 exportoval projekt do svého staršího formátu, a přenese se jako běžný odstavec. Zkontrolujte jej, nebo importujte projekt, ze kterého byl export pořízen.
    [few] { $count } odstavce mohou být nadpisy, které ztratily svůj styl, když Scrivener 3 exportoval projekt do svého staršího formátu, a přenesou se jako běžné odstavce. Zkontrolujte je, nebo importujte projekt, ze kterého byl export pořízen.
   *[other] { $count } odstavců může být nadpisy, které ztratily svůj styl, když Scrivener 3 exportoval projekt do svého staršího formátu, a přenese se jako běžné odstavce. Zkontrolujte je, nebo importujte projekt, ze kterého byl export pořízen.
}
import-scrivener-prose-comment-anchor-without-entry-line = { $count ->
    [one] U 1 komentáře nebo poznámky pod čarou se v aplikaci Scrivener ztratil obsah. Označená slova se zachovají, ale bez něj.
   *[other] U { $count } komentářů nebo poznámek pod čarou se v aplikaci Scrivener ztratil obsah. Označená slova se zachovají, ale bez nich.
}
import-scrivener-prose-comment-entry-without-anchor-line = { $count ->
    [one] 1 komentář už ve svém textu neměl místo a připojí se k prvnímu odstavci.
    [few] { $count } komentáře už ve svém textu neměly místo a připojí se k prvnímu odstavci.
   *[other] { $count } komentářů už ve svém textu nemělo místo a připojí se k prvnímu odstavci.
}
import-scrivener-prose-footnote-entry-without-anchor-line = { $count ->
    [one] 1 propojená poznámka pod čarou už ve svém textu neměla místo, proto se uvede na konci tohoto textu.
    [few] { $count } propojené poznámky pod čarou už ve svých textech neměly místo, proto se každá uvede na konci svého textu.
   *[other] { $count } propojených poznámek pod čarou už ve svých textech nemělo místo, proto se každá uvede na konci svého textu.
}
import-scrivener-prose-range-comment-reading-line = { $count ->
    [one] 1 komentář ze starší verze aplikace Scrivener byl umístěn podle jiného výkladu své pozice. Ověřte, že je u správných slov.
    [few] { $count } komentáře ze starší verze aplikace Scrivener byly umístěny podle jiného výkladu své pozice. Ověřte, že jsou u správných slov.
   *[other] { $count } komentářů ze starší verze aplikace Scrivener bylo umístěno podle jiného výkladu své pozice. Ověřte, že jsou u správných slov.
}
import-scrivener-prose-range-comment-reading-markers-counted-line = { $count ->
    [one] 1 komentář ze starší verze aplikace Scrivener byl umístěn podle počtu značek svých vložených poznámek. Ověřte, že je u správných slov.
    [few] { $count } komentáře ze starší verze aplikace Scrivener byly umístěny podle počtu značek svých vložených poznámek. Ověřte, že jsou u správných slov.
   *[other] { $count } komentářů ze starší verze aplikace Scrivener bylo umístěno podle počtu značek svých vložených poznámek. Ověřte, že jsou u správných slov.
}
import-scrivener-prose-range-comment-reading-pictures-not-counted-line = { $count ->
    [one] 1 komentář ze starší verze aplikace Scrivener byl umístěn bez započítání svých obrázků. Ověřte, že je u správných slov.
    [few] { $count } komentáře ze starší verze aplikace Scrivener byly umístěny bez započítání svých obrázků. Ověřte, že jsou u správných slov.
   *[other] { $count } komentářů ze starší verze aplikace Scrivener bylo umístěno bez započítání svých obrázků. Ověřte, že jsou u správných slov.
}
import-scrivener-prose-range-comment-approximate-line = { $count ->
    [one] 1 komentář ze starší verze aplikace Scrivener neodpovídá přesně žádnému výkladu své pozice a může být o několik znaků posunutý.
    [few] { $count } komentáře ze starší verze aplikace Scrivener neodpovídají přesně žádnému výkladu své pozice a mohou být o několik znaků posunuté.
   *[other] { $count } komentářů ze starší verze aplikace Scrivener neodpovídá přesně žádnému výkladu své pozice a může být o několik znaků posunuto.
}
import-scrivener-prose-document-link-unresolved-line = { $count ->
    [one] 1 odkaz vede na dokument, který import nepřenesl. Jeho slova zůstávají.
    [few] { $count } odkazy vedou na dokumenty, které import nepřenesl. Jejich slova zůstávají.
   *[other] { $count } odkazů vede na dokumenty, které import nepřenesl. Jejich slova zůstávají.
}
import-scrivener-prose-linked-picture-not-read-line = { $count ->
    [1] 1 obrázek propojený se souborem na vašem disku, „{ $example }“, nebyl přenesen.
    [few] { $count } obrázky propojené se soubory na vašem disku, například „{ $example }“, nebyly přeneseny.
   *[other] { $count } obrázků propojených se soubory na vašem disku, například „{ $example }“, nebylo přeneseno.
}
import-scrivener-prose-heading-repeats-title-line = { $count ->
    [one] 1 text začíná nadpisem, který opakuje jeho název, takže export může název vytisknout dvakrát.
    [few] { $count } texty začínají nadpisem, který opakuje jejich název, takže export může název vytisknout dvakrát.
   *[other] { $count } textů začíná nadpisem, který opakuje jejich název, takže export může název vytisknout dvakrát.
}
import-scrivener-prose-markdown-as-text-line = { $count ->
    [one] 1 odstavec napsaný v Markdownu se zachová beze změny jako text.
    [few] { $count } odstavce napsané v Markdownu se zachovají beze změny jako text.
   *[other] { $count } odstavců napsaných v Markdownu se zachová beze změny jako text.
}
import-scrivener-prose-citation-keys-as-text-line = { $count ->
    [one] 1 odstavec obsahuje citační klíče pro bibliografický program, zachované jako text.
    [few] { $count } odstavce obsahují citační klíče pro bibliografický program, zachované jako text.
   *[other] { $count } odstavců obsahuje citační klíče pro bibliografický program, zachované jako text.
}
import-scrivener-prose-placeholders-in-exported-text-line = { $count ->
    [1] 1 zástupná značka aplikace Scrivener, { $example }, zůstane v tom, co exportujete, jako prostý text. Nahraďte ji ručně.
    [few] { $count } zástupné značky aplikace Scrivener, například { $example }, zůstanou v tom, co exportujete, jako prostý text. Nahraďte je ručně.
   *[other] { $count } zástupných značek aplikace Scrivener, například { $example }, zůstane v tom, co exportujete, jako prostý text. Nahraďte je ručně.
}
import-scrivener-prose-placeholders-in-other-text-line = { $count ->
    [1] 1 zástupná značka aplikace Scrivener, { $example }, zůstane jako prostý text v textu, který se neexportuje.
    [few] { $count } zástupné značky aplikace Scrivener, například { $example }, zůstanou jako prostý text v textech, které se neexportují.
   *[other] { $count } zástupných značek aplikace Scrivener, například { $example }, zůstane jako prostý text v textech, které se neexportují.
}
import-scrivener-prose-ornament-lines-kept-line = { $count ->
    [one] 1 řádek z ozdobných znaků zarovnaný na střed může označovat předěl scény a zachová se tak, jak je napsán.
    [few] { $count } řádky z ozdobných znaků zarovnané na střed mohou označovat předěly scén a zachovají se tak, jak jsou napsány.
   *[other] { $count } řádků z ozdobných znaků zarovnaných na střed může označovat předěly scén a zachová se tak, jak jsou napsány.
}
import-scrivener-prose-long-document-line = { $count ->
    [one] 1 text je velmi dlouhý a může se otevírat pomalu.
    [few] { $count } texty jsou velmi dlouhé a mohou se otevírat pomalu.
   *[other] { $count } textů je velmi dlouhých a může se otevírat pomalu.
}
import-scrivener-prose-pictures-without-description-line = { $count ->
    [one] 1 obrázek nemá popis pro čtenáře, kteří jej nevidí. Popis můžete přidat v editoru.
    [few] { $count } obrázky nemají popis pro čtenáře, kteří je nevidí. Popis můžete přidat v editoru.
   *[other] { $count } obrázků nemá popis pro čtenáře, kteří je nevidí. Popis můžete přidat v editoru.
}
import-scrivener-prose-struck-text-compiled-away-line = { $count ->
    [one] 1 text obsahuje přeškrtnutý text, který se zachová. Kompilace aplikace Scrivener jej z knihy vynechávala, ale export z aplikace Skribisto jej vytiskne.
    [few] { $count } texty obsahují přeškrtnutý text, který se zachová. Kompilace aplikace Scrivener jej z knihy vynechávala, ale export z aplikace Skribisto jej vytiskne.
   *[other] { $count } textů obsahuje přeškrtnutý text, který se zachová. Kompilace aplikace Scrivener jej z knihy vynechávala, ale export z aplikace Skribisto jej vytiskne.
}
import-scrivener-prose-compile-deleted-style-printed-line = { $count ->
    [1] Text ve stylu „{ $example }“, který kompilace aplikace Scrivener mazala, se zachová jako próza a export z aplikace Skribisto jej vytiskne.
   *[other] Text { $count } stylů, které kompilace aplikace Scrivener mazala, například „{ $example }“, se zachová jako próza a export z aplikace Skribisto jej vytiskne.
}
import-scrivener-prose-compile-raw-style-printed-line = { $count ->
    [1] Text ve stylu „{ $example }“, který kompilace aplikace Scrivener předávala jako kód pro jiný program, se zachová jako próza a export z aplikace Skribisto jej vytiskne.
   *[other] Text { $count } stylů, které kompilace aplikace Scrivener předávala jako kód pro jiný program, například „{ $example }“, se zachová jako próza a export z aplikace Skribisto jej vytiskne.
}
import-scrivener-prose-left-out-after-document-line = { $count ->
    [one] 1 část poškozeného souboru za jeho koncem se vynechá.
    [few] { $count } části poškozených souborů za jejich koncem se vynechají.
   *[other] { $count } částí poškozených souborů za jejich koncem se vynechá.
}
import-scrivener-prose-left-out-header-or-footer-line = { $count ->
    [one] 1 záhlaví nebo zápatí se vynechá: není součástí textu.
    [few] { $count } záhlaví a zápatí se vynechají: nejsou součástí textu.
   *[other] { $count } záhlaví a zápatí se vynechá: nejsou součástí textu.
}
import-scrivener-prose-left-out-list-label-picture-line = { $count ->
    [one] 1 obrázek použitý jako odrážka seznamu se vynechá.
    [few] { $count } obrázky použité jako odrážky seznamu se vynechají.
   *[other] { $count } obrázků použitých jako odrážky seznamu se vynechá.
}
import-scrivener-prose-left-out-line = { $count ->
    [one] 1 část souboru, kterou neobsahuje žádný text, se vynechá.
    [few] { $count } části souborů, které neobsahuje žádný text, se vynechají.
   *[other] { $count } částí souborů, které neobsahuje žádný text, se vynechá.
}
import-scrivener-prose-undecodable-characters-line = { $count ->
    [one] 1 znak nebylo možné dekódovat a na jeho místě se zobrazuje náhradní znak (�).
    [few] { $count } znaky nebylo možné dekódovat a na jejich místě se zobrazují náhradní znaky (�).
   *[other] { $count } znaků nebylo možné dekódovat a na jejich místě se zobrazují náhradní znaky (�).
}
import-scrivener-prose-inline-notes-merged-line = { $count ->
    [one] 1 vložená poznámka, kterou Scrivener rozdělil v místě, kde se měnilo formátování, se znovu spojí.
    [few] { $count } vložené poznámky, které Scrivener rozdělil v místě, kde se měnilo formátování, se znovu spojí.
   *[other] { $count } vložených poznámek, které Scrivener rozdělil v místě, kde se měnilo formátování, se znovu spojí.
}
import-scrivener-prose-inline-notes-joined-across-paragraphs-line = { $count ->
    [1] 1 vložená poznámka psaná po jednotlivých odstavcích se spojí do jediné poznámky.
    [few] { $count } vložené poznámky psané po jednotlivých odstavcích se spojí do poznámek o více odstavcích.
   *[other] { $count } vložených poznámek psaných po jednotlivých odstavcích se spojí do poznámek o více odstavcích.
}
import-scrivener-prose-blank-notes-dropped-line = { $count ->
    [one] 1 prázdná vložená poznámka se vynechá.
    [few] { $count } prázdné vložené poznámky se vynechají.
   *[other] { $count } prázdných vložených poznámek se vynechá.
}
import-scrivener-prose-annotations-became-comments-line = { $count ->
    [one] 1 vložená anotace se stala komentářem.
    [few] { $count } vložené anotace se staly komentáři.
   *[other] { $count } vložených anotací se stalo komentáři.
}
import-scrivener-prose-annotation-only-document-kept-line = { $count ->
    [one] 1 dokument neobsahoval nic než anotaci. Jeho slova se zachovají jako próza.
    [few] { $count } dokumenty neobsahovaly nic než anotaci. Jejich slova se zachovají jako próza.
   *[other] { $count } dokumentů neobsahovalo nic než anotaci. Jejich slova se zachovají jako próza.
}
import-scrivener-prose-preserve-formatting-passages-line = { $count ->
    [one] 1 pasáž označená jako „Preserve Formatting“ (zachovat formátování) přejde jako běžný text.
    [few] { $count } pasáže označené jako „Preserve Formatting“ (zachovat formátování) přejdou jako běžný text.
   *[other] { $count } pasáží označených jako „Preserve Formatting“ (zachovat formátování) přejde jako běžný text.
}
import-scrivener-prose-nested-note-unwrapped-line = { $count ->
    [one] 1 poznámka uvnitř komentáře nebo poznámky pod čarou se zachová jako text na svém místě.
    [few] { $count } poznámky uvnitř komentářů nebo poznámek pod čarou se zachovají jako text na svém místě.
   *[other] { $count } poznámek uvnitř komentářů nebo poznámek pod čarou se zachová jako text na svém místě.
}
import-scrivener-prose-style-not-carried-line = { $count ->
    [1] Styl „{ $example }“ nemá v aplikaci Skribisto obdobu. Jeho text si ponechá svá slova.
    [few] { $count } styly, například „{ $example }“, nemají v aplikaci Skribisto obdobu. Jejich text si ponechá svá slova.
   *[other] { $count } stylů, například „{ $example }“, nemá v aplikaci Skribisto obdobu. Jejich text si ponechá svá slova.
}
import-scrivener-prose-document-links-dropped-line = { $count ->
    [one] 1 odkaz na jiný dokument nemá v aplikaci Skribisto své místo. Jeho slova zůstávají.
    [few] { $count } odkazy na jiné dokumenty nemají v aplikaci Skribisto své místo. Jejich slova zůstávají.
   *[other] { $count } odkazů na jiné dokumenty nemá v aplikaci Skribisto své místo. Jejich slova zůstávají.
}
import-scrivener-prose-document-links-kept-line = { $count ->
    [one] 1 odkaz na jiný dokument se zachová jako odkaz mezi položkami.
    [few] { $count } odkazy na jiné dokumenty se zachovají jako odkazy mezi položkami.
   *[other] { $count } odkazů na jiné dokumenty se zachová jako odkazy mezi položkami.
}
import-scrivener-prose-other-links-kept-as-text-line = { $count ->
    [one] 1 odkaz na jiný projekt nebo na soubor se zachová jako text.
    [few] { $count } odkazy na jiné projekty nebo na soubory se zachovají jako text.
   *[other] { $count } odkazů na jiné projekty nebo na soubory se zachová jako text.
}
import-scrivener-prose-soft-breaks-split-line = { $count ->
    [one] 1 zalomení řádku uvnitř odstavce teď začíná samostatný odstavec.
    [few] { $count } zalomení řádků uvnitř odstavců teď začínají samostatné odstavce.
   *[other] { $count } zalomení řádků uvnitř odstavců teď začíná samostatné odstavce.
}
import-scrivener-prose-typed-indents-removed-line = { $count ->
    [one] 1 odsazení napsané na začátku odstavce se odstraní.
    [few] { $count } odsazení napsaná na začátku odstavců se odstraní.
   *[other] { $count } odsazení napsaných na začátku odstavců se odstraní.
}
import-scrivener-prose-left-indents-not-kept-line = { $count ->
    [one] 1 odstavec odsazený od okraje přijde o své odsazení.
    [few] { $count } odstavce odsazené od okraje přijdou o své odsazení.
   *[other] { $count } odstavců odsazených od okraje přijde o své odsazení.
}
import-scrivener-prose-blank-lines-removed-line = { $count ->
    [one] 1 prázdný odstavec mezi dvěma jinými se odstraní.
    [few] { $count } prázdné odstavce mezi jinými se odstraní.
   *[other] { $count } prázdných odstavců mezi jinými se odstraní.
}
import-scrivener-prose-verse-stanza-gaps-lost-line = { $count ->
    [one] 1 prázdný řádek mezi slokami básně se odstraní, takže sloky na sebe navazují.
    [few] { $count } prázdné řádky mezi slokami básně se odstraní, takže sloky na sebe navazují.
   *[other] { $count } prázdných řádků mezi slokami básně se odstraní, takže sloky na sebe navazují.
}
import-scrivener-prose-blank-lines-became-scene-breaks-line = { $count ->
    [one] 1 prázdný řádek mezi odstavci se stal předělem scény.
    [few] { $count } prázdné řádky mezi odstavci se staly předěly scén.
   *[other] { $count } prázdných řádků mezi odstavci se stalo předěly scén.
}
import-scrivener-prose-ornaments-became-scene-breaks-line = { $count ->
    [one] 1 řádek z ozdobných znaků se stal předělem scény.
    [few] { $count } řádky z ozdobných znaků se staly předěly scén.
   *[other] { $count } řádků z ozdobných znaků se stalo předěly scén.
}
import-scrivener-prose-table-first-row-header-line = { $count ->
    [one] 1 tabulka teď začíná řádkem záhlaví.
    [few] { $count } tabulky teď začínají řádkem záhlaví.
   *[other] { $count } tabulek teď začíná řádkem záhlaví.
}
import-scrivener-prose-table-cells-joined-line = { $count ->
    [one] 1 buňka tabulky obsahovala více odstavců, které jsou teď spojeny do jednoho.
    [few] { $count } buňky tabulek obsahovaly více odstavců, které jsou teď spojeny do jednoho.
   *[other] { $count } buněk tabulek obsahovalo více odstavců, které jsou teď spojeny do jednoho.
}
import-scrivener-prose-formatting-lost-colour-line = { $count ->
    [one] U 1 textu se odstraní barva písma.
   *[other] U { $count } textů se odstraní barva písma.
}
import-scrivener-prose-formatting-lost-highlight-line = { $count ->
    [one] U 1 textu se odstraní zvýraznění.
   *[other] U { $count } textů se odstraní zvýraznění.
}
import-scrivener-prose-formatting-lost-small-caps-line = { $count ->
    [one] U 1 textu se kapitálky zobrazí jako běžná písmena.
   *[other] U { $count } textů se kapitálky zobrazí jako běžná písmena.
}
import-scrivener-prose-formatting-lost-all-caps-line = { $count ->
    [one] U 1 textu se text zobrazený velkými písmeny zobrazí tak, jak byl napsán.
   *[other] U { $count } textů se text zobrazený velkými písmeny zobrazí tak, jak byl napsán.
}
import-scrivener-prose-formatting-lost-hidden-line = { $count ->
    [one] U 1 textu se vynechá skrytý text.
   *[other] U { $count } textů se vynechá skrytý text.
}
import-scrivener-prose-formatting-lost-line = { $count ->
    [one] U 1 textu se odstraní formátování, pro které Skribisto nemá místo.
   *[other] U { $count } textů se odstraní formátování, pro které Skribisto nemá místo.
}
import-scrivener-prose-justified-text-not-kept-line = { $count ->
    [one] 1 odstavec zarovnaný do bloku se místo toho řídí stylem exportu.
    [few] { $count } odstavce zarovnané do bloku se místo toho řídí stylem exportu.
   *[other] { $count } odstavců zarovnaných do bloku se místo toho řídí stylem exportu.
}
import-scrivener-prose-comment-colours-lost-line = { $count ->
    [one] 1 komentář přijde o svou barvu.
    [few] { $count } komentáře přijdou o svou barvu.
   *[other] { $count } komentářů přijde o svou barvu.
}
import-scrivener-prose-control-characters-removed-line = { $count ->
    [one] 1 neviditelný řídicí znak se odstraní.
    [few] { $count } neviditelné řídicí znaky se odstraní.
   *[other] { $count } neviditelných řídicích znaků se odstraní.
}
import-scrivener-prose-note-on-heading-lost-line = { $count ->
    [one] 1 poznámku pod čarou nebo komentář u nadpisu kapitoly nebylo možné ponechat na místě: komentář se přesune do prvního odstavce kapitoly, poznámka pod čarou se vynechá.
    [few] { $count } poznámky pod čarou nebo komentáře u nadpisů kapitol nebylo možné ponechat na místě: komentář se přesune do prvního odstavce kapitoly, poznámka pod čarou se vynechá.
   *[other] { $count } poznámek pod čarou nebo komentářů u nadpisů kapitol nebylo možné ponechat na místě: komentář se přesune do prvního odstavce kapitoly, poznámka pod čarou se vynechá.
}
import-scrivener-prose-media-file-copied-line = { $count ->
    [one] 1 soubor z rešerší, který Skribisto neumí otevřít, byl zkopírován vedle nového projektu. Jeho poznámka uvádí kam.
    [few] { $count } soubory z rešerší, které Skribisto neumí otevřít, byly zkopírovány vedle nového projektu. Jejich poznámky uvádějí kam.
   *[other] { $count } souborů z rešerší, které Skribisto neumí otevřít, bylo zkopírováno vedle nového projektu. Jejich poznámky uvádějí kam.
}
import-scrivener-prose-media-file-not-carried-line = { $count ->
    [one] 1 soubor z rešerší, který Skribisto neumí otevřít, zůstává ve vašem projektu Scrivener. Jeho poznámka uvádí jeho název.
    [few] { $count } soubory z rešerší, které Skribisto neumí otevřít, zůstávají ve vašem projektu Scrivener. Jejich poznámky uvádějí jejich názvy.
   *[other] { $count } souborů z rešerší, které Skribisto neumí otevřít, zůstává ve vašem projektu Scrivener. Jejich poznámky uvádějí jejich názvy.
}
## When the import fails
import-scrivener-error-refused = { $refusal } Nic nebylo zapsáno.
import-scrivener-error-question = Projekt teď klade otázku, na kterou jeho kontrola neodpověděla. Zvolte „Znovu projít import“ a potom importujte. Nic nebylo zapsáno.
import-scrivener-error-not-accepted = Aby bylo možné projekt importovat, musíte v kroku Projekt zaškrtnout políčko, a to zaškrtnuté nebylo. Zvolte „Znovu projít import“, políčko zaškrtněte a potom importujte. Nic nebylo zapsáno.
import-scrivener-error-changed = Projekt se od vaší kontroly změnil. Zvolte „Znovu projít import“ a potom importujte. Nic nebylo zapsáno.
import-scrivener-error-choice = Struktura zvolená při kontrole už k tomuto projektu nesedí. Zvolte „Znovu projít import“ a potom importujte. Nic nebylo zapsáno.

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
settings-punctuation-auto-lists = Začít seznam odstavcem napsaným jako "- ", "* " nebo "1. "
settings-punctuation-auto-lists-hint = Z odstavce se stane odrážkový nebo číslovaný seznam. Kde je zapnutá dialogová pomlčka a jazyk ji má, otevře "- " místo toho repliku. Klávesou Backspace hned poté vrátíte, co jste napsali.

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

cover-heading = Obálka
cover-pick = Vybrat…
cover-change = Změnit…
cover-remove = Odebrat
cover-none = Zatím žádná obálka
cover-missing = Soubor s obrázkem v projektu chybí
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
# How many pictures the items being imported bring into the project, under the
# review tree. Shown only when there is at least one.
import-document-pictures = { $count ->
    [1] S těmito dokumenty přichází jeden obrázek.
    [few] S těmito dokumenty přicházejí { $count } obrázky.
   *[other] S těmito dokumenty přichází { $count } obrázků.
}
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

# A picture the document shows that does not come with it, one sentence per reason.
# $detail is the picture as the document names it: a file name, a path inside the
# document, or a web address. Nothing is left in the text where it was.
import-diagnostic-image-not-ingested-format = V „{ $path }“ je obrázek „{ $detail }“ ve formátu, který Skribisto neumí zobrazit, proto byl vynechán. Importují se obrázky ve formátech PNG, JPEG, WebP, GIF, TIFF a BMP; uložte jej v jednom z nich a vložte jej ručně.
import-diagnostic-image-not-ingested-remote = V „{ $path }“ je obrázek „{ $detail }“ umístěný na webu. Import nikdy nic nestahuje, proto byl vynechán.
import-diagnostic-image-not-ingested-outside = V „{ $path }“ leží obrázek „{ $detail }“ mimo dokument i jeho složku, proto byl vynechán. Importují se jen obrázky uložené v dokumentu nebo ve vlastní složce dokumentu.
import-diagnostic-image-not-ingested-missing = V „{ $path }“ se obrázek „{ $detail }“ nepodařilo najít, proto byl vynechán.
import-diagnostic-image-not-ingested-too-large = V „{ $path }“ je obrázek „{ $detail }“ větší, než projekt přijme, proto byl vynechán.
import-diagnostic-image-not-ingested-unreadable = V „{ $path }“ se obrázek „{ $detail }“ nepodařilo přečíst jako obrázek, proto byl vynechán.
import-diagnostic-image-not-ingested-heading = V „{ $path }“ je obrázek „{ $detail }“ v nadpisu. Každý nadpis se stane názvem položky v pořadači a název obrázek obsahovat nemůže, proto byl obrázek vynechán.
import-diagnostic-image-not-ingested-note = V „{ $path }“ je obrázek „{ $detail }“ v poznámce pod čarou, vysvětlivce nebo komentáři. Ty si ponechají svá slova, ale žádný obrázek, proto byl vynechán.
import-diagnostic-image-not-ingested-html = V „{ $path }“ je obrázek „{ $detail }“ zapsaný jako HTML a import HTML vynechává, proto byl vynechán i obrázek. Chcete-li jej importovat, zapište jej jako obrázek v Markdownu.
import-diagnostic-image-not-ingested-document-full = V „{ $path }“ byl obrázek „{ $detail }“ vynechán: obrázky tohoto dokumentu dohromady přesahují to, co jeden import přenese. Importujte dokument po částech, nebo zbývající obrázky vložte ručně.
import-diagnostic-picture-first-frame-only = V „{ $path }“ přichází animovaný obrázek „{ $detail }“ jako svůj první snímek. Obrázky v projektu se nehýbou.

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
help-topic-import-scrivener = Přechod z aplikace Scrivener
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
shortcut-name-work-import-scrivener = Importovat projekt Scrivener…
shortcut-name-work-import-scrivener-report = Zobrazit zprávu o importu z aplikace Scrivener
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
dialog-plume-file-title = Vyberte projekt Plume Creator
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
