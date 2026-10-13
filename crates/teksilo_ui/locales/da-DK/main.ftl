# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

# Skribisto: UI strings (source locale).
# A single `&` marks the keyboard mnemonic of a menu label; `&&` is a literal `&`.

## Menu bar: Work
menu-work = &Projekt
menu-new-work = &Nyt projekt
menu-open-work = Å&bn projekt…
menu-new-window = Nyt vin&due
menu-import-from = &Importér fra
# The Launcher's counterpart to `menu-import-from`, over the same two rows: these
# produce a brand-new project, where the project window's importers land content
# *in* the one already open.
menu-create-from = &Opret fra
menu-import-plume = &Plume Creator (.plume)…
menu-import-document = &Dokumenter (Markdown, Word, ODT)…
menu-import-manuskript = &Manuskript (.msk)…
menu-import-scrivener = &Scrivener (.scriv)…
menu-export = &Eksportér
menu-export-book = Eksportér bog
menu-export-part = Eksportér del
menu-export-chapter = Eksportér kapitel
menu-export-scene = Eksportér scene
menu-export-note = Eksportér note
menu-export-paratext = Eksportér paratekst
menu-export-folder = Eksportér mappe
menu-export-choose = Vælg…
menu-export-none = Åbn et dokument for at eksportere
menu-save = &Gem
menu-save-as-file = Gem som enkelt &fil…
menu-save-as-folder = Gem som &mappe…
menu-backup = &Sikkerhedskopiér nu
menu-close-work = &Luk projekt
menu-welcome = &Velkommen…
menu-settings = Indstillinge&r
menu-quit = &Afslut

## Menu bar: View
menu-view = &Vis
menu-outline = &Disposition
menu-search = &Søg i projekt
menu-timeline = &Gå tilbage i tiden
menu-search-preview = Søgeforhånds&visning
menu-fullscreen = F&uld skærm
menu-focus-mode = Dis&traktionsfri tilstand
menu-format = F&ormat
menu-scene-break = Indsæt s&ceneskift
menu-major-scene-break = Indsæt &stort sceneskift

## Menu bar: Go
menu-go = &Gå
menu-go-next-scene = Næste &scene
menu-go-prev-scene = Forrige s&cene
menu-go-next-chapter = Næste &kapitel
menu-go-prev-chapter = Forrige ka&pitel
menu-go-next-note = Næste &note
menu-go-prev-note = Forrige no&te

menu-tools = Værk&tøjer
menu-help = &Hjælp

## Binder context menu
ctx-add = &Tilføj
ctx-new-item = &Nyt element
ctx-new-folder = Ny &mappe
ctx-rename = &Omdøb
ctx-duplicate = &Duplikér
ctx-import-here = &Importér her…
ctx-indent = &Indryk
ctx-outdent = &Ryk ud
ctx-trash = Flyt til &papirkurven
ctx-open-to-side = Åbn ved &siden af
ctx-open = Å&bn
ctx-reveal-in-outline = Vis i dispositio&nen
ctx-move-up = Flyt &op
ctx-move-down = Flyt &ned

## Editor tab context menu
# Fresh `ctx-tab-*` keys, never a reuse of the `ctx-*` rows above: those double as
# menu-BAR rows (shell/project_menus/document.rs), so retuning one of their
# mnemonics to fit this menu would break the bar's own per-locale uniqueness.
# Three rows are either/or: the split, the move and the pin each build exactly
# one of their pair, so a letter may repeat across a pair but never within one
# built menu, which is a debug_assert! panic in MenuList::build, not a warning.
ctx-tab-close = &Luk
ctx-tab-close-others = Luk &andre
ctx-tab-close-all = Luk all&e
# The split rows read directionally, from the pane the tab is already in, so the
# writer never has to work out which half "the side" means. They duplicate: the
# item ends up open in both panes over one shared document.
ctx-tab-open-to-side = Åbn ved &siden af
ctx-tab-open-in-main = Åbn &i hovedruden
# The move rows read the same way, but the tab leaves the pane it came from.
ctx-tab-move-to-side = &Flyt til siden
ctx-tab-move-to-main = &Flyt til hovedruden
ctx-tab-move-to-new-window = Flyt til et &nyt vindue
# Why that last row is unavailable. A second window is opened onto the project's
# file, so a project that has never been saved has nothing for it to open. A
# tooltip on the disabled row, not a menu label, hence no mnemonic.
ctx-tab-move-window-unsaved = Gem først projektet: et nyt vindue åbner en fil på disken
ctx-tab-pin = Fast&gør denne fane
ctx-tab-unpin = F&rigør denne fane
# Tab tooltip, not a menu row: no mnemonic. It names the two commands a pin
# actually protects against, because the tab itself shows only a glyph and a
# missing close button, neither of which says what the pin is for.
tab-pinned-tooltip = Fastgjort: »Luk andre« og »Luk alle« lader den stå åben

## Create recommendations: logical type labels (SplitButton title + Add ▸ rows)
create-book = Bog
create-part = Del
create-chapter = Kapitel
create-scene = Scene
create-note = Note
create-note-folder = Notemappe
create-folder = Mappe
create-paratext = Paratekst
create-paratext-folder = Paratekstmappe
create-book-end = Bogslut
create-story-bible-entry = Opslag i historiebiblen…
# Item-type names for the Overview's Type column (the rest reuse the create-* nouns).
type-book-start = Bogstart
type-text = Tekst

## Create: the default title a new row is given.
## Data, not chrome. Resolved once at creation and then owned by the writer,
## so switching language never retitles anything already created.
## (A scene reuses `new-scene-title`, shared with split-scene.)
new-item-book = Ny bog
new-item-part = Ny del
new-item-chapter = Nyt kapitel
new-item-note = Ny note
new-item-note-folder = Ny notemappe
new-item-folder = Ny mappe
new-item-paratext = Ny paratekst
new-item-paratext-folder = Ny paratekstmappe
new-item-story-bible-entry = Nyt opslag i historiebiblen

## Create recommendations: trailing placement hint on each row
placement-inside = indeni
placement-after = efter
placement-after-parent = efter overordnet
placement-top-level = øverste niveau

# (Writing-model rich tooltips live in this locale's tooltips.ftl.)

## Promote: convert a binder item to its paired type
ctx-promote = &Konvertér til
promote-chapter-folder = Kapitelmappe
promote-flat-chapter = Fladt kapitel
promote-lossy-title = Ingen plads til teksten
promote-lossy-text = En { $target } har ingen plads til disse: { $kinds }. Flyt eller ryd teksten først, og konvertér derefter.
# Content-role names, for explaining what a conversion cannot carry over.
content-scene-text = Scenetekst
content-note-text = Notetekst
content-book-title = Bogens titel
content-book-subtitle = Bogens undertitel
content-part-title = Delens titel
content-chapter-title = Kapitlets titel
content-epigraph-text = Motto
content-paratext-text = Paratekst
promote-blocked-title = Kapitlet er ikke tomt
promote-blocked-text = { $count ->
    [one] Dette kapitel indeholder stadig { $count } element. Flyt det, eller læg det i papirkurven, før du konverterer kapitlet til et fladt kapitel.
   *[other] Dette kapitel indeholder stadig { $count } elementer. Flyt dem, eller læg dem i papirkurven, før du konverterer kapitlet til et fladt kapitel.
}

## Inspector (trailing dock) + status-bar dock toggles
inspector = Inspektør
inspector-empty = Åbn et element for at se dets egenskaber.
inspector-promote = Konvertér til…
statusbar-toggle-outline = Vis eller skjul ringbindet
statusbar-toggle-inspector = Vis eller skjul inspektøren
# The save indicator (status bar, next to the binder toggle).
statusbar-save-unsaved = Ændringer er ikke gemt. Klik for at gemme
statusbar-save-saved = Alle ændringer er gemt
statusbar-save-autosave = Automatisk gem er slået til. Ændringer gemmes, mens du skriver
statusbar-saving = Gemmer…
# The focused item's live word count (status bar).
statusbar-word-count = { $count ->
    [one] { $count } ord
   *[other] { $count } ord
}
statusbar-word-count-tooltip = Ord i den scene, du redigerer
# Words + characters, shown when "Show character count" is on (Settings ▸ Goals).
statusbar-word-char-count = { $words ->
    [one] { $words } ord
   *[other] { $words } ord
} · { $chars ->
    [one] { $chars } tegn
   *[other] { $chars } tegn
}
# The writing session (status-bar sprint timer + word tracker).
session-toggle = Skrivesession: start eller sæt en fokuseret skrivespurt på pause
session-configure = Angiv sessionens ordmål og tidsgrænse
session-configure-title = Skrivesession
session-word-goal = Ordmål
session-time-limit = Tidsgrænse
# The spin box's non-editable trailing unit. An abbreviation, not a word that
# agrees with the number: a `SpinBox` suffix is static text and cannot carry a
# plural selector. The separating space is prepended in code (Qt's `" min"`
# convention), so give the unit alone. Never shown at 0, which reads
# `session-no-limit` instead.
session-time-limit-unit = min.
session-no-goal = Intet mål
session-no-limit = Ingen grænse
session-reset = Nulstil session
session-readout = { $words ->
    [one] { $words } ord
   *[other] { $words } ord
} · { $time }
session-readout-timed = { $words ->
    [one] { $words } ord
   *[other] { $words } ord
} · { $time } tilbage
# Distraction-free mode's always-visible control strip (Increment 2).
statusbar-focus-exit = Afslut distraktionsfri tilstand
# The strip's Next/Previous buttons (Increment 4) fire the same go.next/go.prev
# actions as the shortcut and the Go menu's generic pair.
statusbar-focus-synopsis = Synopsis
statusbar-focus-go-prev = Forrige element (Alt+Op)
statusbar-focus-go-next = Næste element (Alt+Ned)

## Activity-rail actions (dockless commands in the icon rail)
rail-settings = Indstillinger

## Settings
language = Sprog
theme = Tema
english = Engelsk
french = Fransk
light = Lyst
dark = Mørkt
settings-text-width = Tekstbredde
settings-preview-width = Bredde på søgeforhåndsvisning
settings-autosave = Gem automatisk på disken
settings-show-welcome = Vis startskærmen ved opstart
settings-show-welcome-tip = Når den er slået fra, genåbnes det seneste projekt i stedet.
settings-check-for-updates = Søg efter nye versioner
settings-check-for-updates-tip = Højst én gang om dagen spørger Skribisto projektets websted, hvilken version der er den nyeste. Forespørgslen indeholder ingen identifikator, ikke engang den version, du bruger: sammenligningen foregår på denne computer.

## Settings window: chrome
settings-title = Indstillinger
settings-close = Luk
settings-search = Søg i indstillinger
settings-reset = Nulstil til standard
settings-done = Færdig
settings-cancel = Annullér
settings-apply = Anvend
settings-ok = OK
settings-reset-confirm-title = Nulstil alle indstillinger til standard?
settings-reset-confirm-body = Gendanner programmets indstillinger for udseende, editor og skrivning. Projektets egne indstillinger, tastaturgenveje og gemte typografier berøres ikke. Handlingen kan ikke fortrydes.
settings-empty-title = Ingen indstillinger her endnu
settings-empty-hint = Dette afsnit får indstillinger i en fremtidig opdatering.

## Settings window: categories
settings-sec-appearance-behaviour = Udseende og adfærd
settings-sec-editor = Editor
settings-sec-spelling = Stavning
settings-sec-backup = Sikkerhedskopi og synkronisering
settings-sec-compile = Kompilering og eksport
settings-page-appearance = Udseende
settings-field-image-size-policy = Store billeder
settings-image-policy-ask = Spørg hver gang
settings-image-policy-keep = Behold originalen
settings-image-policy-downscale = Optimér
settings-hint-image-size-policy = Hvad der skal ske, når et indsat billede er større, end manuskriptet har brug for.
settings-page-notifications = Meddelelser
settings-page-scene = Scene
settings-page-synopsis = Synopsis
settings-page-notes = Noter
settings-page-editor-behavior = Editorens adfærd
settings-page-goals = Mål og ordantal
settings-page-games = Skrivelege
settings-page-corkboard = Opslagstavle
settings-page-distraction-free = Distraktionsfri
settings-page-dictionaries = Ordbøger
settings-page-autosave = Automatisk gem
settings-page-export = Eksportformater
settings-page-paratext = Paratekststrukturer
settings-page-keymap = Genvejstaster
# Filter box on the Keymap page (filters the ShortcutSettings list by name / id / category).
settings-keymap-filter = Filtrér genveje

## Settings window: what each page is for
# One line per page, shown under that page's link on its parent's page and,
# for a parent, under its own title. Keep them to one line: they describe what
# the page holds, not how to use it.
settings-desc-sec-appearance-behaviour = Hvordan selve programmet ser ud, og hvad det gør, når det starter.
settings-desc-sec-editor = Skrivefladen: hvordan den ser ud, og hvad den gør, mens du skriver.
settings-desc-sec-spelling = Stavekontrol og ordbøgerne bag den.
settings-desc-sec-backup = Hvordan dit arbejde kommer ned på disken, og hvilke kopier der gemmes.
settings-desc-sec-compile = Hvad der forlader Skribisto, og i hvilken form.
settings-desc-sec-work = Indstillinger, der hører til dette projekt og følger med i dets fil.
settings-desc-sec-extensions = Sider fra de udvidelser, der er installeret her.
settings-desc-group-typography = Én side pr. teksttype: skrifttype, størrelse, linjehøjde og afstand.
settings-desc-appearance = Brugerfladens sprog, tema, tekststørrelse og startskærmen ved opstart.
settings-desc-notifications = Alle meddelelser, der er vist i denne session, og de handlinger, du kan gentage.
settings-desc-scene = Hvordan sceneteksten sættes: skrifttype, størrelse, linjehøjde, indrykning og afstand.
settings-desc-synopsis = Hvordan synopsisruden sættes, uafhængigt af manuskriptet.
settings-desc-notes = Hvordan noter sættes, uafhængigt af manuskriptet.
settings-desc-corkboard = Kortstørrelse, hvad et kort viser, og hvordan tavlen arrangerer kortene.
settings-desc-distraction-free = Typografi, kolonnebredde og kontrolbjælke til skrivning i fuld skærm.
settings-desc-distraction-free-themes = Temabiblioteket til skrivning i fuld skærm: de medfølgende temaer og dine egne.
settings-desc-editor-behavior = Tekstbredde, synopsens placering, skrivemaskinerulning og fremhævning omkring markøren.
settings-desc-punctuation = Den typografiske husstil, som nye projekter starter med. Alle projekter følger den, medmindre de har deres egen.
settings-desc-goals = Mål for ord og tegn, og hvordan ord tælles.
settings-desc-games = Begrænsninger, du selv vælger, mens du skriver udkast, f.eks. Altid fremad.
settings-desc-spellcheck = Den ene kontakt, der slår stavekontrol til og fra overalt.
settings-desc-dictionaries = Installér, fjern og gennemse stavekontrollens ordbøger på denne computer.
settings-desc-autosave = Om dine ændringer skrives til disken af sig selv.
settings-desc-backup = Hvornår der tages kopier, hvor de gemmes, og hvor mange. Alle projekter følger dette, medmindre de har deres egen indstilling.
settings-desc-export = De typografier, alle eksporter kompileres med: de medfølgende og dine egne.
settings-desc-paratext = Det for- og efterstof, et nyt projekt kan starte med.
settings-desc-keymap = Alle genveje, og hvad de er knyttet til.
settings-page-user = Bruger
settings-desc-user = Hvem du er, til de kommentarer du skriver.
settings-group-identity = Identitet
settings-field-user-name = Dit navn
settings-field-user-name-placeholder = Valgfrit
settings-field-user-name-hint = Underskriver de kommentarer og svar, du skriver. Adskiller sig fra projektets forfatternavn, som er bogens byline og følger med i filen.
settings-field-user-initials = Dine initialer
settings-field-user-initials-placeholder = Fra dit navn
settings-field-user-initials-hint = Det, et tekstbehandlingsprogram viser ud for din kommentar i margenen. Lad feltet være tomt for at bruge de viste initialer, som er taget fra dit navn.
settings-field-user-hint = Begge er valgfrie og gælder for alle projekter på denne computer. Ændrer du dem, underskrives de kommentarer, du skriver fremover. Kommentarer, der allerede er skrevet, beholder det navn, de blev skrevet under.
settings-desc-author = Navnet, der står på dette projekt.
settings-desc-structure = Om projektets kapitler er mapper eller enkeltelementer.
settings-desc-language = Det sprog, projektets tekst kontrolleres efter.
settings-desc-work-backup = Projektets egen politik for sikkerhedskopier, eller den generelle.
settings-desc-personal-dictionary = De ord, projektet betragter som korrekt stavet.
settings-desc-tags = De farvekodede mærker, projektet markerer sine elementer med.
settings-desc-templates = De noteskabeloner, du kan indsætte, mens du skriver i dette projekt.
settings-desc-text-replacements = Forkortelser, der udfoldes, mens du skriver i dette projekt.
settings-desc-work-punctuation = De anførselstegn, tankestreger og mellemrum, projektets tekst følger.

## Settings window: fields
settings-group-typography = Typografi
settings-group-writing-column = Skrivekolonne
settings-group-writing-view = Skrivevisning
# The distraction-free control strip's optional items (Exit is never optional).
settings-group-distraction-free-strip = Kontrolbjælke
settings-group-theme = Tema
settings-group-language = Sprog
settings-group-startup = Opstart
settings-group-updates = Opdateringer
settings-group-autosave = Automatisk gem
settings-field-typeface = Skrift
settings-field-size = Tekststørrelse

# The readout shown while Ctrl+Wheel / Ctrl+= / Ctrl+- / Ctrl+0 change an
# editor's text size. One whole message per surface rather than a shared
# "{ $surface }: { $percent }" template: the surface name is not a plain
# substitution in every language, and a translator should get complete
# sentences. $percent arrives already formatted ("110%").
editor-size-changed-manuscript = Manuskriptets tekststørrelse: { $percent }
editor-size-changed-synopsis = Synopsens tekststørrelse: { $percent }
editor-size-changed-notes = Noternes tekststørrelse: { $percent }
editor-size-changed-corkboard = Tekststørrelse på opslagstavlens kort: { $percent }
editor-size-changed-corkboard-expanded = Tekststørrelse i den udvidede editor: { $percent }
editor-size-changed-distraction-free = Tekststørrelse i distraktionsfri tilstand: { $percent }

menu-text-size-increase = &Forstør tekst
menu-text-size-decrease = Form&indsk tekst
menu-text-size-reset = &Nulstil tekststørrelse
settings-field-line-height = Linjehøjde
settings-field-first-line-indent = Indrykning af første linje
settings-field-paragraph-spacing-before = Afstand før afsnit
settings-field-paragraph-spacing-after = Afstand efter afsnit
settings-field-column-width = Kolonnebredde
settings-distraction-free-width-hint = Gælder kun, mens distraktionsfri tilstand er slået til. Kolonnebredderne for scene, synopsis og noter ændres ikke.
settings-distraction-free-title = Behold elementets navn
settings-distraction-free-word-count = Behold ordantallet
settings-distraction-free-session = Behold skrivesessionen
settings-distraction-free-go-to = Behold knappen Gå til…
settings-distraction-free-go = Behold knapperne Forrige og Næste
settings-distraction-free-chrome-hint = Knappen Afslut er der altid, uanset hvad du vælger her. Den er din vej ud, hvis Escape er optaget.
settings-field-app-theme = Tema
# The two entries of the Theme picker (the Fluent light / dark appearances).
settings-theme-light = Lyst
settings-theme-dark = Mørkt
settings-field-text-scale = Brugerfladens tekststørrelse
settings-field-language = Brugerfladens sprog
settings-synopsis-placement = Synopsens placering
settings-synopsis-placement-none = Ingen
settings-synopsis-placement-top = Over
settings-synopsis-placement-side = Ved siden af
settings-typewriter = Skrivemaskinerulning
settings-typewriter-tip = Holder den linje, du skriver på, i en fast højde, mens manuskriptet ruller under den. Et klik placerer stadig markøren, hvor du klikker.
settings-typewriter-position = Linjens placering
settings-typewriter-position-top-third = Øverste tredjedel
settings-typewriter-position-middle = Midten
settings-typewriter-position-bottom-quarter = Nederste fjerdedel
settings-highlight-scope = Fremhævning omkring markøren
settings-highlight-scope-none = Ingen
settings-highlight-scope-sentence = Sætning
settings-highlight-scope-paragraph = Afsnit
settings-highlight-scope-tip-none = Lad siden være ren. Intet skygges, mens du skriver.
settings-highlight-scope-tip-sentence = Skyg den sætning, du skriver, så den skiller sig ud fra sætningerne omkring den.
settings-highlight-scope-tip-paragraph = Skyg hele det afsnit, du skriver, så den passage, du arbejder på, bliver i syne.
settings-group-container-views = Visninger af beholdere
settings-remember-view = Husk den seneste visning for hver elementtype
settings-remember-view-tip = Åbn en beholder i den visning, du sidst brugte for den type
settings-remember-view-tip-more =
    En bog, en del og et kapitel har hver flere visninger (sin egen side, hele
    manuskriptet, hele synopsen). Slår du dette til, genåbner hver type i den
    visning, du sidst valgte for den: skifter du f.eks. et kapitel til Helt kapitel,
    viser det næste kapitel, du åbner, også Helt kapitel. Hver type husker sin egen visning.
# Goals & Word Count pane
settings-group-counting = Optælling af ord
settings-counting-auto = Automatisk (efter sprog)
settings-counting-whitespace = Del ved mellemrum
settings-counting-unicode-words = Unicode-ord
settings-counting-cjk-hybrid = CJK-tilpasset (pr. tegn)
settings-counting-hint = Automatisk tæller kinesisk og japansk i tegn og alle andre sprog i ord. Skift det kun, hvis optællingen ser forkert ud for dit sprog.
settings-group-goals-display = Visning
settings-show-characters = Vis tegnantallet på statuslinjen
settings-autosave-hint = Ændringer skrives automatisk til disken, mens du skriver.

## Settings: Work (the open project)
settings-sec-work = Projekt
settings-page-structure = Struktur
settings-page-author = Forfatter
settings-field-author-name = Forfatternavn
settings-field-author-placeholder = Valgfrit
settings-field-author-hint = Vises på den kompilerede titelside og i de eksporterede filers metadata. Lad feltet være tomt for at udelade det. Dette er bogens byline. Navnet, dine kommentarer underskrives med, findes under Indstillinger ▸ Bruger.
settings-group-chapters = Kapitler
tidy-titles-title = Ryd op i kapiteltitler
tidy-titles-none = Ingen kapitel- eller deltitel gentager blot sit eget nummer.
tidy-titles-lead = { $count ->
        [one] Et kapitel eller en del har intet andet i titlen end sit eget nummer.
       *[other] { $count } kapitler og dele har intet andet i titlen end deres eget nummer.
    }
tidy-titles-explain = Når disse titler ryddes, navngives hver enkelt med det nummer, bogen allerede kender, det samme nummer, som eksporten udskriver. Intet andet ændres, og én fortrydelse sætter dem alle tilbage.
menu-document-tidy-titles = Ryd op i &kapiteltitler…
settings-group-numbering = Nummerering
settings-number-chapters = Nummerér kapitler og dele
settings-number-chapters-tip = Kapitler og dele får et nummer, der udledes af deres plads i bogen. Det vises ved siden af titlen her og udskrives af eksporten.
settings-number-chapters-tip-more = Nummeret gemmes aldrig i titlen, så det forbliver korrekt, når du flytter, indsætter eller sletter. Slår du dette fra, udskriver eksporten kun titlerne, uanset hvad eksporttypografien beder om. Vil du udelade ét kapitel (en prolog, et mellemspil), så brug i stedet kontakten Nummerering i inspektøren: kapitlet bliver i bogen, men får intet nummer.
settings-part-resets-chapter = Start kapitelnumre forfra i hver del
settings-part-resets-chapter-tip = Slået fra som standard: kapitlerne nummereres fortløbende på tværs af bogens dele, så »Anden del« begynder med kapitel elleve.
settings-part-resets-chapter-tip-more = Det er den gængse praksis i forlagsbranchen, og det en læser forventer. Slå det til for en bog, hvis dele skal læses som separate bind, hvor hvert bind begynder med kapitel et.
settings-chapter-flat = Flade kapitler
settings-chapter-flat-hint = Til: et kapitel er et enkelt element. Du skriver i det, og det indeholder ingen scener. Fra: et kapitel er en mappe. Du kan stadig skrive i det, men det kan også indeholde scener. Nye kapitler følger denne indstilling; eksisterende konverteres med Konvertér til.

## Settings: Export styles (Compile & Export ▸ Export Formats)
settings-styles-builtin = Indbyggede typografier
settings-styles-user = Mine typografier
settings-styles-builtin-badge = Indbygget
settings-styles-duplicate = Duplikér
settings-styles-edit = Redigér
settings-styles-delete = Slet
settings-styles-import = Importér…
settings-styles-export = Eksportér…
settings-styles-copy-suffix = (kopi)
settings-styles-json-filter = Eksporttypografi
settings-styles-editor-title = Redigér typografi
settings-styles-editor-none = Vælg en egen typografi for at redigere den, eller duplikér en indbygget.
settings-styles-imported = Typografien er importeret
settings-styles-import-failed = Typografien kunne ikke importeres
settings-styles-exported = Typografien er eksporteret
settings-styles-export-failed = Typografien kunne ikke eksporteres
# Editor field labels
settings-styles-field-name = Navn
settings-styles-field-chapters = Kapiteloverskrifter
settings-styles-field-parts = Deloverskrifter
settings-styles-field-scene-break = Sceneskift
settings-styles-field-major-scene-break = Stort sceneskift
settings-styles-field-spacing = Linjeafstand
settings-styles-field-justify = Lige margener
settings-styles-field-notes = Medtag noter
settings-styles-field-synopses = Medtag synopser
settings-styles-field-scene-titles = Medtag scenetitler
settings-styles-field-epigraphs = Medtag mottoer
settings-styles-field-paratexts = Medtag paratekster
settings-styles-field-epigraph-placement = Mottoets placering
settings-styles-epigraph-after = Efter titlen
settings-styles-epigraph-before = Før titlen
settings-styles-field-footnotes = Medtag fodnoter
settings-styles-field-footnote-numbering = Nummerering af fodnoter
settings-styles-footnote-numbering-continuous = Fortløbende
settings-styles-footnote-numbering-per-chapter = Start forfra i hvert kapitel
settings-styles-footnote-numbering-per-book = Start forfra i hver bog
settings-styles-field-images = Billeder
menu-image = &Billede
settings-styles-images-beside = Ved siden af dokumentet
settings-styles-images-embed = Inde i dokumentet
settings-styles-images-omit = Udelad
settings-styles-group-round-trip = Sende til en redaktør
settings-styles-field-comments = Medtag kommentarer
settings-styles-field-round-trip-marks = Medtag returmarkører
settings-styles-round-trip-hint = Kun DOCX og ODT. Markører er usynlige identifikatorer, som lader en returneret fil opdatere dette projekt i stedet for at havne ved siden af det som en ekstra kopi.
settings-styles-group-pages = Sider
settings-styles-field-cover = Begynd med omslaget
settings-styles-field-word-count = Ordantal på titelsiden
settings-styles-field-page-books = Ny side ved hver bog
settings-styles-field-page-parts = Ny side ved hver del
settings-styles-field-page-chapters = Ny side ved hvert kapitel
settings-styles-field-page-paratexts = Ny side ved hver paratekst
# Heading-scheme options
settings-styles-heading-none = Ingen overskrift
settings-styles-heading-numbered = Kun nummer
settings-styles-heading-title = Kun titel
settings-styles-heading-both = Nummer og titel
# Scene-break options (glyph values stay as typed)
settings-styles-break-blank = Tom linje
settings-styles-break-none = Ingen
# Line-spacing options
settings-styles-spacing-single = Enkelt
settings-styles-spacing-onehalf = 1½
settings-styles-spacing-double = Dobbelt

## Welcome
welcome-title = Velkommen til Skribisto
# $version is stamped from the git tag at build time (see src/version.rs).
welcome-version = Version { $version }
# Shown under the version in the launcher sidebar, only when a newer release exists.
welcome-update-available = Version { $version } er tilgængelig
welcome-update-tip = Åbner downloadsiden i din browser.
welcome-search = Søg i projekter
welcome-open = Åbn
welcome-new-work = Nyt projekt
welcome-create-from = Opret fra…
welcome-recent-works = Seneste projekter
welcome-empty-recents = Ingen seneste projekter endnu.
# Shown in place of the recents list when the search matched none of them,
# distinct from having no recent works at all.
welcome-no-matches = Intet af de seneste projekter matcher din søgning.
welcome-learn-soon = Vejledninger og tips kommer snart.
welcome-about-blurb = Skribisto, skriveprogrammet genskrevet i Rust og Teksilo.
# The *…* is inline markup, not decoration: it italicizes the line (the widget
# renders it in a serif italic). Keep the asterisks when translating.
welcome-tagline = *Et stille sted at skrive lange ting.*
# Tooltip and screen-reader name of the two icon links under the sidebar nav.
# Product names: keep them as they are.
welcome-github = GitHub
welcome-discord = Discord
nav-works = Projekter
nav-examples = Eksempler
nav-learn = Lær
nav-about = Om

## Editor tabs & panes
synopsis = Synopsis
epigraph = Motto
pane-manuscript = Manuskript
corkboard = Opslagstavle
overview = Oversigt

## Corkboard
corkboard-card-count = { $count ->
    [one] { $count } kort
   *[other] { $count } kort
}
corkboard-child-count = { $count ->
    [one] { $count } element
   *[other] { $count } elementer
}
corkboard-view-nested = Indlejret
corkboard-view-flat = Flad
corkboard-layout-hint = Indlejret viser en beholders direkte underelementer. Åbn et mappekort for at gå ind i det. Flad viser alle scener i hele beholderen på én gang.
corkboard-card-size = Kortstørrelse
corkboard-search-placeholder = Filtrér kort…
corkboard-empty-title = Intet her endnu
corkboard-empty-hint = Brug »＋ Ny« ovenfor for at tilføje det første.
corkboard-new = Ny
corkboard-show-card-numbers = Nummerér kortene
corkboard-modal-size = Størrelse på udvidet editor
corkboard-scope-hint = Gælder for alle åbne opslagstavler i alle projekter.
corkboard-sort-manuscript = Manuskriptets rækkefølge
corkboard-sort-title-asc = Titel A–Å
corkboard-sort-title-desc = Titel Å–A
corkboard-card-number = Kort { $number }

## Corkboard batch actions (the card kebab acts on the whole selection)
duplicate = Duplikér
duplicate-n = { $count ->
    [one] Duplikér { $count } kort
   *[other] Duplikér { $count } kort
}
set-label-n = { $count ->
    [one] Angiv etiket på { $count } kort
   *[other] Angiv etiket på { $count } kort
}
move-to-trash-n = { $count ->
    [one] Flyt { $count } kort til papirkurven
   *[other] Flyt { $count } kort til papirkurven
}
reveal-in-outline = Vis i dispositionen

## Corkboard “Move to…” destination picker
corkboard-move-to = Flyt til…
corkboard-move-to-n = { $count ->
    [one] Flyt { $count } kort til…
   *[other] Flyt { $count } kort til…
}
corkboard-move-picker-title = Flyt til…
corkboard-move-picker-empty = Ingen ringbind endnu. Opret et først.
corkboard-move-picker-cancel = Annullér
corkboard-move-here = Flyt hertil
corkboard-moved-ok = { $count ->
    [one] { $count } kort er flyttet
   *[other] { $count } kort er flyttet
}
corkboard-move-into-self = En beholder kan ikke flyttes ind i sig selv. Vælg en destination uden for den.
corkboard-move-failed = Kortene kunne ikke flyttes dertil.

## Overview (the container's contents as a sortable table)
overview-col-title = Titel
overview-col-type = Type
overview-col-label = Etiket
overview-col-tags = Mærker
# Only ever built when the Work has two or more Books: see `overview_columns`'s
# own gate, the same one every Books surface in this edition shares.
overview-col-books = Bøger
overview-col-own-words = Ord
overview-col-total-words = I alt
overview-row-count = { $count ->
    [one] { $count } element
   *[other] { $count } elementer
}
overview-search-placeholder = Filtrér elementer…
overview-expand-all = Udvid alle
overview-collapse-all = Fold alle sammen
overview-table-label = Indhold
overview-empty-title = Intet her endnu
overview-gone-title = Denne beholder findes ikke længere
overview-gone-hint = Den blev flyttet til papirkurven. Gendan den, eller luk denne fane.
overview-empty-hint = Brug »＋ Ny« ovenfor for at tilføje det første.
corkboard-grid-label = Opslagstavlens kort
corkboard-rename-field = Omdøb element
corkboard-expand-synopsis = Udvid synopsis
corkboard-synopsis-modal-title = Synopsis
corkboard-badge-scene = Scene
corkboard-badge-chapter = Kapitel
corkboard-badge-part = Del
corkboard-badge-book = Bog
corkboard-badge-note = Note
corkboard-badge-folder = Mappe
corkboard-badge-paratext = Paratekst
corkboard-badge-text = Tekst
corkboard-badge-end = Slut
settings-group-corkboard-layout = Layout
settings-group-corkboard-cards = Kort
corkboard-show-word-count = Vis ordantal på kortene
text-heading = Tekst
no-content = Dette element har intet redigerbart indhold.
untitled = Uden titel
placeholder-title = Titel…
placeholder-subtitle = Undertitel…
placeholder-chapter-title = Kapitlets titel…
split-editor = Opdel editor
close-split-view = Luk opdelt visning
drop-open-here = Åbn her
drop-open-to-side = Åbn ved siden af

## Manuscript streams (Full Chapter / Part / Book + Full Synopsis)
# The container's own page. "Chapter" is this chapter; "Full Chapter" is this chapter
# and every scene in it.
segment-chapter = Kapitel
segment-part = Del
segment-book = Bog
segment-pace = Tempo
segment-notes = Noter
segment-story-bible = Historiebibel
# The three segments on an `Item/Note` tab: its own prose, its story-bible fields, and
# (only when it carries a discoverable tag) the manuscript prose it has been declared
# present in. See `teksilo_ui::tabs::item_note`.
segment-note-own = Note
segment-note-details = Detaljer
segment-note-in-prose = I teksten
pace-placeholder = Tempoplanlæggeren vises her.
# Pace planner
pace-empty-title = Planlæg bogens tempo
pace-empty-body = Angiv et ordmål og en deadline, så beregner Skribisto det daglige tempo, der skal til for at nå i mål.
pace-start-planning = Begynd planlægningen
pace-section-schedule = Tidsplan
pace-section-progress = Fremskridt
pace-advancement = Fremdrift
pace-goal = Ordmål
pace-deadline = Deadline
pace-active = Tempo aktivt
pace-day-mon = man
pace-day-tue = tir
pace-day-wed = ons
pace-day-thu = tor
pace-day-fri = fre
pace-day-sat = lør
pace-day-sun = søn
pace-card-written = ord skrevet
pace-card-of-goal = af målet
pace-card-rate = ord pr. skrivedag
pace-card-days-left = skrivedage tilbage
pace-card-streak = dage i træk
pace-card-ahead = ord foran tempoet
pace-card-behind = ord bagud i forhold til tempoet
pace-charts-empty = Diagrammerne over fremskridt vises her, når dit ordantal er registreret ved en gemning.
pace-chart-progression = Skrevne ord i forhold til målet
pace-chart-words-per-day = Ord pr. dag
pace-series-actual = Faktisk
pace-series-target = Mål
pace-series-words-per-day = Ord/dag
pace-daily-target-line = Jævnt tempo: { $count ->
    [one] { $count } ord/dag
   *[other] { $count } ord/dag
}
pace-section-holidays = Fridage
pace-section-milestones = Milepæle
pace-holidays-none = Ingen fridage. Alle planlagte dage tæller.
pace-holiday-label = Fridagens navn
pace-add-holiday = Tilføj
pace-remove = Fjern
pace-milestones-none = Ingen milepæle endnu. Sæt en på en del eller et kapitel i inspektøren.
full-chapter = Helt kapitel
full-part = Hel del
full-book = Hel bog
full-synopsis = Hel synopsis
rename = Omdøb
set-label = Angiv etiket
insert-scene = Indsæt scene
insert-chapter = Indsæt kapitel
split-scene = Opdel scene
move-up = Flyt op
move-down = Flyt ned
merge-with-previous = Flet med forrige
move-to-trash = Flyt til papirkurven
rename-chapter = Omdøb kapitel
add-scene = Tilføj ny scene
add-chapter = Tilføj nyt kapitel
new-scene-title = Ny scene
placeholder-scene-name = Scenens navn…
menu-cut = Klip
menu-copy = Kopiér
menu-paste = Sæt ind
menu-paste-unformatted = Sæt ind uden formatering
menu-select-all = Markér alt
# Tooltips on the editor context menu's formatting row. Icon-only buttons, so
# the tooltip is their only accessible name, not decoration.
format-bold = Fed
format-italic = Kursiv
format-underline = Understreget
format-strikethrough = Gennemstreget

## Binder / recents
binder = Ringbind
no-work = Intet projekt indlæst
no-recent-works = Ingen seneste projekter
switcher-open-section = Åbne nu
switcher-recent-section = Seneste
switcher-this-window = dette vindue
open-project-title = Åbn projekt
open-project-question = Hvordan vil du åbne »{ $title }«?
open-in-new-window = Åbn i nyt vindue
open-here = Åbn her

## Binder switcher + search
binder-all = Alle ringbind
binder-show-all = Vis alle ringbind
binder-new = Nyt ringbind…
# Pre-filled in the "New binder" dialog, so the writer can just press Enter.
# Stored literally from then on: a binder name is data, not a label.
binder-default-name = Nyt ringbind
binder-item-count = { $count ->
    [one] { $count } element
   *[other] { $count } elementer
}
binder-search-placeholder = Filtrér dispositionen…
binder-search-scope = Søg i alle ringbind
binder-trash-confirm-title = Flyt ringbindet til papirkurven?
binder-trash-confirm-text = »{ $name }« og alle dets elementer flyttes til papirkurven.

## Dialogs
dialog-rename = Omdøb
dialog-new-binder = Nyt ringbind
dialog-set-label = Angiv etiket
dialog-new-scene = Ny scene
close-work-question = Vil du gemme ændringerne, før projektet lukkes?
quit-question = Vil du gemme ændringerne, før du afslutter?
quit-save-work-question = Vil du gemme ændringerne i { $title }, før du afslutter?
unsaved-changes = Projektet har ændringer, der ikke er gemt.
# Shown by Quit when another open Work (not this window's own) still has
# unsaved edits. The design's "one dialog listing every dirty Work" (see
# app::commands::file's `app.quit` action). Quit is refused until those are
# saved or closed from their own window.
# Replacing the open work in this window (New Work, Open Work, "Open here", the
# import toast's "Open now"), the same guard as closing, since the open work is
# closed either way.
new-work-unsaved-question = Vil du gemme ændringerne, før du opretter et nyt projekt?
open-work-unsaved-question = Vil du gemme ændringerne, før du åbner et andet projekt?
switch-backup-discard-title = Kassér ændringerne i denne sikkerhedskopi?
switch-backup-discard-text = Ændringer i en sikkerhedskopi kan ikke gemmes i den. Brug Gem som eller Gendan for at beholde dem, eller kassér dem og åbn det andet projekt.
switch-save-failed = Projektet kunne ikke gemmes, så det blev ikke erstattet: { $error }
switch-save-not-started = Projektet kunne ikke gemmes, så det blev ikke erstattet.
close-save-failed = Projektet kunne ikke gemmes, så det blev ikke lukket: { $error }
close-save-not-started = Projektet kunne ikke gemmes, så det blev ikke lukket.
save-not-started = Projektet kunne ikke gemmes.

## Toasts
# Toast title. Kept short because a toast title is one line and truncates; the cause goes
# in the body, which is the untranslated error text itself. $file is the file's base name.
could-not-open-work = »{ $file }« kunne ikke åbnes
# A project saved by a newer Skribisto than this one: title and body.
# $written_by is the version that wrote it, $requires the lowest version that can open it
# (the two differ when the newer build happened to use nothing new), $supported the newest
# this build understands.
could-not-open-work-too-new = »{ $file }« kræver en nyere Skribisto
could-not-open-work-too-new-detail = Gemt med Skribisto-format { $written_by }. Det kræver format { $requires } eller nyere at åbne det, og denne version understøtter op til format { $supported }. Opdatér Skribisto for at åbne det.
could-not-open-work-importing = »{ $file }« er ved at blive skrevet af en import
could-not-open-work-importing-detail = En import skriver over dette projekt og er ikke færdig. Åbn det, når importen er færdig: åbnede du det nu, ville det vise det projekt, importen er ved at erstatte, og den næste gemning ville skrive det projekt tilbage over importen.
could-not-open-example = Eksemplet kunne ikke åbnes: { $error }
# The browser (or whatever handles http links) could not be started for one of
# the Welcome sidebar's links. $url is shown so the address can still be copied.
could-not-open-link = { $url } kunne ikke åbnes: { $error }
could-not-create-work = Projektet kunne ikke oprettes: { $error }
saving-as-file = Gemmer som { $target }…
saving-as-folder = Gemmer som { $target }/…
saved-as = Gemt i { $target }
save-error = Der kunne ikke gemmes: { $error }
save-flush-failed = Gemningen gik igennem, men disse dokumenter kunne ikke overdrages til den og er stadig ikke gemt: { $titles } ({ $error })
backup-error = Der kunne ikke tages en sikkerhedskopi: { $error }
backing-up = Tager sikkerhedskopi…
backup-nothing-open = Der er intet åbent projekt at sikkerhedskopiere.
backup-already-running = En sikkerhedskopiering er allerede i gang.
backup-complete = Sikkerhedskopieringen er fuldført ({ $ok } gemt, { $skipped } allerede ajour)
backup-partial = Sikkerhedskopieringen er afsluttet: { $ok } gemt, { $failed ->
    [one] { $failed } destination mislykkedes
   *[other] { $failed } destinationer mislykkedes
}
backup-no-destination-title = Intet sted til sikkerhedskopier er tilgængeligt
backup-no-destination-text = Ingen af de konfigurerede destinationer for sikkerhedskopier kan nås (et eksternt drev er måske frakoblet). Tilslut det, og prøv igen, eller afslut uden at sikkerhedskopiere.
backup-failed-close-title = Sikkerhedskopien kunne ikke gemmes
backup-failed-close-text = Der kunne ikke skrives en sikkerhedskopi før lukning. Alle destinationer mislykkedes (drevet er måske fjernet, fuldt eller skrivebeskyttet). Ret fejlen, og prøv igen, eller afslut uden at sikkerhedskopiere.
backup-failed-close-error-text = Der kunne ikke skrives en sikkerhedskopi før lukning: { $error }. Ret fejlen, og prøv igen, eller afslut uden at sikkerhedskopiere.
backup-no-result = Sikkerhedskopieringen er afsluttet uden at oplyse, om den lavede en kopi. Se på listen over sikkerhedskopier, om der er kommet en ny.

## Open-a-backup (choice modal + permanent banner + restore)
backup-choice-title = Sikkerhedskopifil
backup-choice-heading = Du har åbnet en sikkerhedskopi
backup-choice-subtitle = Dette er en sikkerhedskopi af et projekt fra et bestemt tidspunkt.
backup-choice-subtitle-dated = Sikkerhedskopi taget { $date }.
backup-choice-body = Du kan åbne den og redigere frit, men ændringer kan kun beholdes med Gem som. Den oprindelige projektfil røres ikke. Du kan også gendanne projektet til præcis denne sikkerhedskopi.
backup-choice-open = Åbn sikkerhedskopien
backup-choice-restore = Gendan projektet til dette tidspunkt…
backup-choice-not-a-backup = Nej, åbn den normalt
backup-banner-title = Sikkerhedskopi: ændringer kan ikke gemmes her
backup-banner-description = Brug Gem som for at beholde dine ændringer i en ny fil, eller Gendan for at erstatte det oprindelige projekt med denne sikkerhedskopi.
backup-banner-restore = Gendan…
backup-banner-save-as = Gem som…
backup-restore-original-missing = Det oprindelige projekt, der skulle gendannes over, kan ikke findes. Brug i stedet Gem som for at beholde sikkerhedskopien som et nyt projekt.
backup-restore-close-elsewhere-title = Projektet er åbent i et andet vindue
backup-restore-close-elsewhere-text = Det projekt, du gendanner over, er åbent i et andet vindue. Luk det først der, og prøv så igen.
backup-restore-focus-window = Gå til det vindue
backup-restore-confirm-title = Gendan denne sikkerhedskopi?
backup-restore-confirm-text = Den nuværende version af projektet kopieres til side som en sikkerhedskopi, før den erstattes med denne.
backup-restore-confirm-ok = Gendan
backup-restore-error = Der kunne ikke gendannes: { $error }
backup-restored-ok = Projektet er gendannet.
backup-restored-with-safety = Projektet er gendannet. Din tidligere version blev gemt i { $path }.
close-backup-discard-title = Kassér ændringerne i denne sikkerhedskopi?
close-backup-discard-text = Ændringer i en sikkerhedskopi kan ikke gemmes i den. Brug Gem som for at beholde dem, eller kassér dem og luk.
quit-backup-discard-title = Kassér ændringerne og afslut?
quit-backup-discard-work-question = Kassér ændringerne i { $title } og afslut?
quit-backup-discard-text = Ændringer i en sikkerhedskopi kan ikke gemmes i den. Brug Gem som for at beholde dem, eller kassér dem og afslut.
backup-nudge-text = Der er ikke sat sikkerhedskopiering op for dette projekt.
backup-nudge-action = Opsæt sikkerhedskopiering…

## Backups list panel
menu-backups-list = Sikkerheds&kopier…
backups-title = Sikkerhedskopier
backups-loading = Søger efter sikkerhedskopier…
backups-empty = Der er endnu ikke fundet sikkerhedskopier af dette projekt.
backups-open = Åbn
backups-reveal = Vis i mappe
backups-delete = Slet denne sikkerhedskopi
backups-delete-confirm-title = Slet denne sikkerhedskopi?
backups-delete-confirm-text = »{ $name }« slettes permanent. Det kan ikke fortrydes.
backups-delete-error = Sikkerhedskopien kunne ikke slettes: { $error }
backups-refresh = Opdatér
backups-close = Luk

## Backup settings panes
settings-page-backup = Standard for sikkerhedskopier
settings-page-work-backup = Sikkerhedskopier
settings-backup-general-title = Standardindstillinger for sikkerhedskopier
settings-backup-work-title = Sikkerhedskopier af dette projekt
settings-backup-inherit = Brug de generelle indstillinger for sikkerhedskopier
settings-backup-inheriting = Dette projekt bruger de generelle indstillinger for sikkerhedskopier.
settings-backup-none-hint = Der er ikke sat automatiske sikkerhedskopier op (alle udløsere er slået fra).
settings-backup-last = Seneste sikkerhedskopi: { $date }
settings-backup-last-never = Ingen sikkerhedskopier endnu.
settings-backup-open-list = Åbn listen over sikkerhedskopier…
settings-backup-triggers = Hvornår der skal sikkerhedskopieres
settings-backup-on-close = Sikkerhedskopiér, når projektet lukkes
settings-backup-on-open = Sikkerhedskopiér, når projektet åbnes
settings-backup-interval = Sikkerhedskopiér med jævne mellemrum, hvert
settings-backup-destinations = Destinationer for sikkerhedskopier
settings-backup-default-location = Standardplacering
settings-backup-usage-since = { $count ->
    [one] { $count } sikkerhedskopi · { $size } · ældste { $oldest }
   *[other] { $count } sikkerhedskopier · { $size } · ældste { $oldest }
}
settings-backup-usage = { $count ->
    [one] { $count } sikkerhedskopi · { $size }
   *[other] { $count } sikkerhedskopier · { $size }
}
settings-backup-usage-empty = Intet gemt her endnu
settings-backup-usage-measuring = Måler…
settings-backup-reveal-root = Vis mappe
settings-backup-dest-none = Ingen destinationer. Sikkerhedskopier gemmes ved siden af projektet.
settings-backup-dest-remove = Fjern
settings-backup-dest-add = Tilføj mappe…
settings-backup-dest-refresh = Opdatér
settings-backup-retention = Hvor mange der skal beholdes
settings-backup-retention-tiered = Trinvis
settings-backup-retention-keep-n = Behold de seneste N
settings-backup-retention-tip = Hvordan gamle sikkerhedskopier tyndes ud.
settings-backup-retention-tip-more = Trinvis beholder én sikkerhedskopi pr. time i et døgn, pr. dag i en uge, pr. uge i en måned og derefter pr. måned, så den nyere historik forbliver tæt, og den ældre tyndes ud. »Behold de seneste N« beholder blot de N nyeste kopier. I begge tilstande beholdes de nyeste kopier (minimummet nedenfor) altid.
settings-backup-gfs-hourly = Hver time (seneste 24 t)
settings-backup-gfs-daily = Dagligt (seneste uge)
settings-backup-gfs-weekly = Ugentligt (seneste måned)
settings-backup-gfs-monthly = Månedligt
settings-backup-keep-n = Antal, der skal beholdes
settings-backup-min-keep = Behold altid mindst
settings-backup-dedup = Spring sikkerhedskopien over, når intet er ændret

## New-work dialog
new-work-title = Nyt projekt
new-work-close = Luk
new-work-name = Projektets navn
new-work-name-placeholder = Uden titel
new-work-author = Forfatter
new-work-author-placeholder = Valgfrit
new-work-format = Format
new-work-single-file = Enkelt fil
new-work-bundle = Projektmappe
new-work-convert-later = Du kan skifte mellem formaterne senere.
new-work-location = Placering
new-work-will-create = Opretter
new-work-language = Standardsprog
new-work-language-hint = Bruges til nye tekster og stavekontrol. Hver tekst kan skiftes til et andet sprog.
new-work-template = Skabelon
new-work-template-none = Ingen
new-work-template-empty-novel = Tom roman
new-work-template-light-novel = Kort roman
new-work-template-novel = Roman
new-work-template-novel-in-parts = Roman i dele
new-work-template-notebook = Notesbog
new-work-cancel = Annullér
new-work-create = Opret projekt
# Wizard steps & navigation
new-work-step-details = Detaljer
new-work-step-language = Sprog og struktur
new-work-step-template = Skabelon
new-work-back = Tilbage
new-work-next = Næste
# The same wizard, opened from the Launcher's "From documents…"
new-work-documents-title = Nyt projekt fra dokumenter
new-work-step-import = Import
new-work-create-and-import = Opret og importér…
new-work-documents-next-title = Dine dokumenter kommer bagefter
new-work-documents-next-body = Når projektet er oprettet, åbnes importguiden oven på det. Her vælger du dokumenterne, gennemgår den struktur, Skribisto læser ud af dem, og vælger, hvor den skal placeres. Intet skrives ind i projektet, før du bekræfter det der.
new-work-documents-no-template = Projektet starter med vilje tomt: ingen skabelon, så de importerede dokumenter er det eneste i det.
new-work-documents-chapter-scene-hint = Gælder for de kapitler, importen opretter. Du kan ændre det senere i projektets indstillinger.
# Format tile descriptions
new-work-single-file-desc = Ét .skrib-arkiv (zip). Let at flytte og at sikkerhedskopiere.
new-work-bundle-desc = En mappe med alle tekster og ressourcer. Bedre egnet til versionsstyring.
# Template row trailing counts
new-work-template-none-count = tomt ringbind
new-work-template-empty-novel-count = 1 kapitel
new-work-template-light-novel-count = 15 kapitler
new-work-template-novel-count = 20 kapitler
new-work-template-novel-in-parts-count = 3 dele, 24 kapitler
new-work-template-notebook-count = frie noter
# ChapterScene toggle (novel templates)
new-work-chapter-scene = Flade kapitler
new-work-chapter-scene-tip = Hvert kapitel er et enkelt element, du skriver direkte i, uden scener under sig. Lad dette være slået fra for det klassiske layout, hvor et kapitel er en mappe: du skriver også direkte i den, men den kan desuden indeholde scener.
new-work-chapter-scene-tip-more = Du skriver i et kapitel uanset hvad. Den eneste forskel er, om det kan *indeholde* scener. Skribistos ringbind er kun til organisering, så begge layouts kompileres til den samme bog. Du kan frit blande dem, og Konvertér til skifter et kapitel mellem de to uden at miste et eneste ord.
# Field validation
new-work-name-required = Angiv et navn til projektet
new-work-name-invalid = Navnet indeholder ingen brugbare tegn
new-work-location-required = Vælg en placering
new-work-location-missing = Mappen findes ikke
new-work-location-not-folder = Stien er ikke en mappe
new-work-location-readonly = Der kan ikke skrives i mappen
new-work-target-exists = Der findes allerede et projekt med dette navn i mappen. Du bliver spurgt, før det erstattes
new-work-target-is-folder = Der findes allerede en mappe med dette navn på denne placering. Vælg et andet navn
new-work-overwrite-title = Erstat det eksisterende projekt?
new-work-overwrite-text = »{ $name }« findes allerede i mappen. Vil du erstatte det med det nye projekt?
new-work-target-open-title = Projektet er åbent
new-work-target-open-text = »{ $name }« er åbent i Skribisto. Et nyt projekt på samme sted ville erstatte det projekt, du arbejder på, og din næste gemning ville skrive det projekt tilbage over det nye. Vælg et andet navn eller en anden mappe, eller luk projektet først.
target-importing-title = En import skriver dette projekt
target-importing-text = En import er stadig ved at skrive »{ $name }«. Vent, til den er færdig, eller vælg et andet navn.
backup-restore-importing-text = En import er stadig ved at skrive »{ $name }«, det projekt, denne sikkerhedskopi ville blive gendannet over. Gendan sikkerhedskopien, når importen er færdig.

## New-work template labels (passed to the backend, which can't do i18n)
new-work-manuscript = Manuskript
new-work-notes = Noter
new-work-research = Research
new-work-notebook = Notesbog
new-work-scene = Scene
new-work-note = Note
new-work-front-matter = Forstof
new-work-back-matter = Efterstof
new-work-paratext = Bogens struktur
new-work-paratext-none = Ingen struktur
new-work-paratext-hint = Det for- og efterstof, en tradition åbner og lukker en bog med. Du kan flytte, omdøbe eller slette alt sammen bagefter.
new-work-tags = Mærker
new-work-tags-none = Ingen mærker
new-work-tags-hint = En startpalet til at mærke personer, steder og alt det andet. Valgfri, og alle mærker kan omdøbes, få ny farve eller slettes bagefter.
new-work-note-templates = Noteskabeloner
new-work-note-templates-none = Ingen skabeloner
new-work-note-templates-hint = Den form, en note i historiebiblen starter med. Valgfri, og alle skabeloner kan redigeres eller slettes bagefter.
new-work-characters = Personer
new-work-places = Steder

## Import Plume Creator dialog
import-plume-title = Importér Plume Creator-projekt
import-plume-source = Plume-projekt
import-plume-source-hint = Vælg en .plume- eller .plume_backup-fil (fra enhver version af Plume Creator).
import-plume-trash-warning = ⚠ Elementer i papirkurven og slettede elementer overføres ikke.
# Field validation
import-plume-source-required = Vælg en Plume-projektfil
import-plume-source-missing = Filen findes ikke
import-plume-source-not-file = Stien er ikke en fil
# Binder names passed to the backend (which can't do i18n)
import-plume-manuscript-binder = Manuskript
import-plume-story-bible-binder = Historiebibel
import-plume-notes-of = { $name } (noter)
import-plume-content-of = { $name } (tekst)
# Progress toast (the import runs as a long operation)
import-plume-progress-title = Importerer Plume-projekt…
# Result
import-plume-done = { $imported ->
    [one] { $imported } element er importeret.
   *[other] { $imported } elementer er importeret.
} { $skipped ->
    [one] { $skipped } element i papirkurven blev ikke overført.
   *[other] { $skipped } elementer i papirkurven blev ikke overført.
}
# The project was refused unread because one of its parts nests its XML deeper
# than the app will read. $part is the part's own name (tree, attendance, info);
# $limit is the ceiling, a number.
import-plume-nested-too-deep = Projektet blev ikke importeret: dets del »{ $part }« er indlejret mere end { $limit } niveauer dybt. Intet Plume Creator-projekt kommer i nærheden af det, og at læse det kunne få Skribisto til at gå ned. Intet blev skrevet.
# The project was refused unread because one of its parts declares XML
# entities, which could expand into more memory than the computer has. $part
# is the part's own name (tree, attendance, info).
import-plume-declares-entities = Projektet blev ikke importeret: dets del »{ $part }« erklærer XML-entiteter, hvad intet Plume Creator-projekt gør. At udfolde dem kunne kræve mere hukommelse, end computeren har, og få Skribisto til at gå ned. Intet blev skrevet.
import-plume-no-outline = Filen er et komprimeret arkiv, men ikke et Plume Creator-projekt: den har ingen disposition. Intet blev skrevet.
import-plume-empty-outline = Dispositionsfilen i dette Plume Creator-projekt er tom, så der er intet at importere. Intet blev skrevet.
# $tag is the name of the document the part turned out to be.
import-plume-wrong-document = Dette er ikke et Plume Creator-projekt: dets del »{ $part }« er et »{ $tag }«-dokument. Intet blev skrevet.
# $version is the part's own version, $supported the newest this importer reads.
import-plume-too-new = Delen »{ $part }« i dette projekt er skrevet af en nyere version af Plume Creator (version { $version }), end Skribisto kan læse (op til { $supported }). Intet blev skrevet.

## Import Manuskript project dialog
import-manuskript-title = Importér Manuskript-projekt
import-manuskript-source = Manuskript-projekt
# A Manuskript project is a .msk file plus, in its usual mode, a folder of the
# same name beside it. Either one gets you there, and so does the folder itself.
import-manuskript-source-hint = Vælg .msk-filen eller projektmappen (fra enhver version af Manuskript).
import-manuskript-source-file = Vælg fil…
import-manuskript-source-folder = Vælg mappe…
# Field validation
import-manuskript-source-required = Vælg et Manuskript-projekt
import-manuskript-source-missing = Filen eller mappen findes ikke
# Names passed to the backend (which can't do i18n). Manuskript stores none of
# these: it has no binders, no story-bible groups, and its importance scale is
# three numbers whose names live in its own interface.
import-manuskript-manuscript-binder = Manuskript
import-manuskript-story-bible-binder = Historiebibel
import-manuskript-characters-group = Personer
import-manuskript-world-group = Verden
import-manuskript-plots-group = Plot
import-manuskript-project-info-note = Projektoplysninger
import-manuskript-summary-note = Resumé
import-manuskript-importance-minor = Mindre
import-manuskript-importance-secondary = Sekundær
import-manuskript-importance-main = Hovedperson
import-manuskript-notes-of = { $title } (noter)
# Progress toast (the import is a long operation)
import-manuskript-progress-title = Importerer Manuskript-projekt…
# Result
import-manuskript-done = { $imported ->
    [one] { $imported } element er importeret.
   *[other] { $imported } elementer er importeret.
} { $revisions ->
    [0] { "" }
    [one] { $revisions } tidligere version fulgte med.
   *[other] { $revisions } tidligere versioner fulgte med.
}
# The project was refused unread because one of its files nests its XML deeper
# than the app will read. $part is that file's name inside the project
# (world.opml, plots.xml…); $limit is the ceiling, a number.
import-manuskript-nested-too-deep = Projektet blev ikke importeret: dets fil »{ $part }« er indlejret mere end { $limit } niveauer dybt. Intet Manuskript-projekt kommer i nærheden af det, og at læse det kunne få Skribisto til at gå ned. Intet blev skrevet.
# The project was refused unread because one of its files sits in folders
# nested deeper than the app will read (Manuskript keeps a project's outline
# as folders, one per level). $limit is the ceiling, a number.
import-manuskript-folders-too-deep = Projektet blev ikke importeret: en af dets filer ligger mere end { $limit } mapper dybt. Intet Manuskript-projekt kommer i nærheden af det, og at læse det kunne få Skribisto til at gå ned. Intet blev skrevet.
# The project was refused unread because one of its files declares XML
# entities, which could expand into more memory than the computer has. $part
# is that file's name inside the project (world.opml, plots.xml…).
import-manuskript-declares-entities = Projektet blev ikke importeret: dets fil »{ $part }« erklærer XML-entiteter, hvad intet Manuskript-projekt gør. At udfolde dem kunne kræve mere hukommelse, end computeren har, og få Skribisto til at gå ned. Intet blev skrevet.
import-manuskript-not-a-project-file = »{ $file }« er hverken et Manuskript-projektarkiv eller den lille .msk-fil, der peger på en projektmappe. Intet blev skrevet.
import-manuskript-folder-missing = »{ $stub }« peger på en projektmappe ved navn »{ $folder }« ved siden af sig, og den mappe er der ikke. Et Manuskript-projekt i mappetilstand består af begge dele, og de skal flyttes sammen. Intet blev skrevet.
import-manuskript-not-a-project = Dette ligner ikke et Manuskript-projekt: det har hverken en Manuskript-markør eller en disposition. Intet blev skrevet.
import-manuskript-format-too-new = Projektet er i Manuskript-format { $version }, som er nyere, end Skribisto kan læse (op til { $supported }). Intet blev skrevet.
import-manuskript-too-many-files = Projektet blev ikke importeret: det indeholder mere end { $limit } filer, langt flere end noget Manuskript-projekt. Intet blev skrevet.
import-manuskript-too-large = Projektet blev ikke importeret: det er større end de { $limit } MB, Skribisto læser fra et Manuskript-projekt. Intet blev skrevet.
project-import-close = Luk
project-import-location = Destinationsmappe
# The folder dialog's title.
project-import-location-title = Vælg en destinationsmappe
project-import-name = Filnavn
project-import-name-placeholder = Projektets navn
project-import-will-create = Opretter
project-import-cancel = Annullér
project-import-import = Importér
# Field validation
project-import-location-required = Vælg en destinationsmappe
project-import-location-missing = Mappen findes ikke
project-import-location-not-folder = Stien er ikke en mappe
project-import-location-readonly = Der kan ikke skrives i mappen
project-import-name-required = Angiv et filnavn
project-import-name-exists = Der findes allerede en fil med dette navn her. Du bliver bedt om at bekræfte overskrivningen
project-import-name-is-folder = Der findes allerede en mappe med dette navn her, måske et projekt, der er gemt som mappe. En import kan ikke erstatte den: vælg et andet navn
# Overwrite confirmation
project-import-overwrite-title = Erstat den eksisterende fil?
project-import-overwrite-text = »{ $name }« findes allerede. Vil du erstatte den med det importerede projekt?
# Projects are imported one at a time, whichever importer asks.
project-import-busy-title = Et andet projekt er ved at blive importeret
project-import-busy-text = Projekter importeres ét ad gangen. Vent, til den igangværende import er færdig, eller annullér den, og start så denne igen.
# Spoken by a screen reader once per phase, never shown: the toast's title, then
# the phase's name.
project-import-progress-announcement = { $title } { $phase }
# The notice once Cancel is pressed, until the converter has stopped: it may be
# finishing a file it cannot leave half written.
project-import-stopping = Standser importen…
project-import-cancelled = Importen er annulleret
project-import-open-now = Åbn nu
project-import-details = Detaljer
# The import's report: what the writer should know about what came across.
project-import-report = { $count ->
    [one] 1 ting, du bør vide om denne import
   *[other] { $count } ting, du bør vide om denne import
}
project-import-report-title = Om denne import
# Error toast: a short reason in the body, the full technical chain behind Details.
project-import-error-title = Projektet kunne ikke importeres
# Why a project was not imported, whichever importer read it. A $file is a file
# name, a $folder a full path (it is the folder the writer has to act on), a
# $reason what the system or a parser said, a $limit a number.
project-import-failed-cancelled = Importen blev annulleret. Intet blev skrevet.
project-import-failed-target-exists = »{ $file }« findes allerede, og importen fik ikke lov til at erstatte den. Intet blev skrevet.
project-import-failed-target-folder = »{ $file }« er en mappe, måske et projekt, der er gemt som mappe, og en import kan ikke erstatte den. Vælg et andet navn. Intet blev skrevet.
project-import-failed-not-written = »{ $file }« kunne ikke skrives i »{ $folder }« ({ $reason }). Intet blev skrevet.
project-import-failed-disk-full = Der er ikke plads nok tilbage i »{ $folder }« til at skrive »{ $file }«. Frigør noget plads der, og importér så igen. Intet blev skrevet.
project-import-failed-source-missing = »{ $file }« er der ikke længere. Intet blev skrevet.
project-import-failed-unreadable = Skribisto kunne ikke læse »{ $file }« ({ $reason }). Intet blev skrevet.
project-import-failed-archive-damaged = »{ $file }« er en komprimeret fil, der ikke kunne læses, sandsynligvis fordi den er beskadiget ({ $reason }). Intet blev skrevet.
project-import-failed-archive-entries = Projektet blev ikke importeret: dets arkiv angiver { $entries ->
    [one] { $entries } fil
   *[other] { $entries } filer
}, flere end de { $limit }, et projekt må indeholde. Det gør kun en fil, der er bygget til at opbruge hukommelsen. Intet blev skrevet.
project-import-failed-archive-member = Projektet blev ikke importeret: dets del »{ $part }« fylder mere end { $limit } MB udpakket. Det gør kun en fil, der er bygget til at opbruge hukommelsen. Intet blev skrevet.
project-import-failed-archive-size = Projektet blev ikke importeret: det fylder mere end { $limit } MB udpakket. Det gør kun en fil, der er bygget til at opbruge hukommelsen. Intet blev skrevet.
project-import-failed-nested = Projektet blev ikke importeret: dets del »{ $part }« er indlejret mere end { $limit } niveauer dybt, og at læse den kunne få Skribisto til at gå ned. Intet blev skrevet.
project-import-failed-entities = Projektet blev ikke importeret: dets del »{ $part }« erklærer XML-entiteter. At udfolde dem kunne kræve mere hukommelse, end computeren har, og få Skribisto til at gå ned. Intet blev skrevet.
project-import-failed-folders = Projektet blev ikke importeret: en af dets filer ligger mere end { $limit } mapper dybt, og at læse den kunne få Skribisto til at gå ned. Intet blev skrevet.
project-import-failed-malformed = Projektet blev ikke importeret: dets del »{ $part }« er beskadiget og kan ikke læses ({ $reason }). Intet blev skrevet.
# A Scrivener project the importer cannot bring across (a Scrivener 1 for Mac project, a
# damaged binder file). Why is the Scrivener importer's own to say; Details holds its key.
project-import-failed-scrivener-refused = Skribisto kan ikke importere dette Scrivener-projekt. Intet blev skrevet.
# The Scrivener project asks something its review did not answer: a project or a binder
# file appeared in it since.
project-import-failed-scrivener-question = Scrivener-projektet stiller et spørgsmål, som gennemgangen ikke besvarede. Start importen af det forfra, besvar spørgsmålet, og importér så. Intet blev skrevet.
# The Scrivener project raises a question the writer has to accept before it imports.
project-import-failed-scrivener-not-accepted = Før Scrivener-projektet kan importeres, skal du sætte kryds i et felt på trinnet Projekt, og der blev ikke sat kryds. Start importen af det forfra, sæt krydset, og importér så. Intet blev skrevet.
# The Scrivener project changed after the writer reviewed it.
project-import-failed-scrivener-changed = Scrivener-projektet er blevet ændret, efter du gennemgik det. Start importen af det forfra, gennemgå det igen, og importér så. Intet blev skrevet.
# An error nothing above accounts for; $detail is its technical description.
project-import-failed-internal = Importen stoppede ved en fejl, som Skribisto ikke havde forventet: { $detail }

## Project import, shared by the Plume Creator and Manuskript dialogs
# The chosen destination is a project open in a window, here or in another
# running copy of Skribisto. The import is refused rather than written over it.
import-target-open-title = Projektet er åbent
import-target-open-text = »{ $name }« er åbent i Skribisto. En import over det ville erstatte det projekt, du arbejder på, og din næste gemning ville skrive det projekt tilbage over importen. Vælg et andet filnavn, eller luk projektet først.
# Spoken by a screen reader when an import's warnings notice appears, never shown.
# $result is the import's own result line ("Imported 12 items."), $notice the
# notice's title ("2 things could not be imported exactly"). The notice is read out
# over the result, so it says both.
import-warnings-announcement = { $result } { $notice }.
# Quitting while an import runs. $name is the file the import is writing.
quit-import-running-title = En import kører stadig
quit-import-running-text = Skribisto er stadig ved at importere »{ $name }«. Afslutter du nu, standser importen: intet bliver skrevet, og det projekt, du importerer fra, forbliver, som det var.
quit-import-stop = Stop importen og afslut

## Import a Scrivener project: the wizard
import-scrivener-title = Importér et Scrivener-projekt
import-scrivener-step-project = Projekt
import-scrivener-step-review = Gennemgang
import-scrivener-step-save = Gem
import-scrivener-back = Tilbage
import-scrivener-next = Næste
import-scrivener-import = Importér
import-scrivener-cancel = Annullér
import-scrivener-help = Hjælp

## Project step: choosing the project
import-scrivener-drop-title = Slip et Scrivener-projekt her
import-scrivener-drop-hint = Et .scriv-projekt, .scrivx-filen i det, en .zip-sikkerhedskopi eller en .scrivtemplate
import-scrivener-source = Scrivener-projekt
import-scrivener-source-placeholder = Sti til projektet
import-scrivener-choose-file = Vælg fil…
import-scrivener-choose-folder = Vælg mappe…
# Windows and Linux show a .scriv project as an ordinary folder.
import-scrivener-source-hint = Vælg projektets .scriv-mappe, eller åbn den, og vælg .scrivx-filen i den. En .zip-sikkerhedskopi virker også.
# macOS shows a .scriv project as a single file when Scrivener is installed, as a folder when it is not.
import-scrivener-source-hint-mac = Vises projektet som én fil, så brug »Vælg fil…«. Vises det som en mappe, så brug »Vælg mappe…«. En .zip-sikkerhedskopi virker også.
# File-dialog titles and filter label, resolved when the dialog opens.
import-scrivener-dialog-file-title = Vælg et Scrivener-projekt
import-scrivener-dialog-folder-title = Vælg en Scrivener-projektmappe
import-scrivener-dialog-filter = Scrivener-projekt
import-scrivener-source-missing = Der er intet på denne sti
# A path whose name is not Unicode: the field shows it with its unreadable letters replaced.
import-scrivener-source-unreadable-name = Navnet kan ikke læses. Omdøb projektet i din filhåndtering, og vælg det så igen.
import-scrivener-empty-hint = Vælg et projekt for at se, hvad Skribisto finder i det.
import-scrivener-reading = Læser projektet…
import-scrivener-stop-reading = Stop læsningen
import-scrivener-read-failed = Projektet kunne ikke læses
import-scrivener-read-failed-body = Noget gik galt under læsningen. Vælg »Læs projektet igen« for at prøve en gang til, eller »Detaljer« for at se, hvad der gik galt.
import-scrivener-details = Detaljer
import-scrivener-check-again = Læs projektet igen
# A refusal this version does not know how to word.
import-scrivener-refusal-unknown-title = Dette projekt kan ikke importeres
import-scrivener-refusal-unknown-body = Skribisto ved ikke, hvordan det skal overføres. Detaljerne nævner årsagen.

## Project step: the project card
import-scrivener-made-with-mac = Senest gemt af Scrivener { $version } til macOS
import-scrivener-made-with-windows = Senest gemt af Scrivener { $version } til Windows
import-scrivener-made-with-linux = Senest gemt af Scrivener { $version } til Linux
import-scrivener-made-with-ios = Senest gemt af Scrivener { $version } til iPhone og iPad
import-scrivener-made-with-version = Senest gemt af Scrivener { $version }
import-scrivener-made-with-unknown = Scrivener-projekt, format { $format }
import-scrivener-saved-on = Gemt { $date } på »{ $device }«
import-scrivener-saved-on-no-device = Gemt { $date }
import-scrivener-card-counts = { $documents ->
    [one] 1 dokument
   *[other] { $documents } dokumenter
}, { $manuscript } i manuskriptet

## Review step
import-scrivener-review-hint = Sådan overføres dit ringbind. Intet skrives, før du importerer, og mange elementer kan bagefter skifte type i ringbindet med Konvertér til.
import-scrivener-choices-structure = Niveauer
import-scrivener-top-level = Mapper på øverste niveau
import-scrivener-choices-text = Tekst
import-scrivener-choice-containers = Mapper på niveau { $depth }
import-scrivener-choice-texts = Dokumenter på niveau { $depth }
import-scrivener-choice-count = { $count ->
    [one] 1 element
   *[other] { $count } elementer
}
import-scrivener-choice-count-example = { $count ->
    [1] 1 element: »{ $example }«
   *[other] { $count } elementer, f.eks. »{ $example }«
}
import-scrivener-rung-part = Dele
import-scrivener-rung-chapter = Kapitler
import-scrivener-rung-plain = Almindelige mapper
import-scrivener-texts-scenes = Scener
import-scrivener-texts-chapters = Kapitler
import-scrivener-choice-books = Bøger i manuskriptet
import-scrivener-books-one = Én bog
import-scrivener-books-several = Én bog pr. mappe
import-scrivener-choice-split = { $count ->
    [1] Opdel »{ $title }« ved overskriften
   *[other] Opdel »{ $title }« ved de { $count } overskrifter
}
import-scrivener-root-items = { $count ->
    [one] 1 element
   *[other] { $count } elementer
}
import-scrivener-root-items-manuscript = { $count ->
    [one] 1 element. Denne mappe ligner et manuskript.
   *[other] { $count } elementer. Denne mappe ligner et manuskript.
}
import-scrivener-dest-notes = Noter
import-scrivener-dest-story-bible = Historiebibel
import-scrivener-dest-manuscript = Endnu en bog
import-scrivener-dest-leave-out = Udelad
import-scrivener-choice-trash = { $count ->
    [one] Tag papirkurven med (1 element)
   *[other] Tag papirkurven med ({ $count } elementer)
}
import-scrivener-trash-detail = { $count ->
    [0] Ingen af dem indeholder tekst.
    [one] 1 af dem indeholder tekst.
   *[other] { $count } af dem indeholder tekst.
}
import-scrivener-choice-blank-lines = Tomme linjer bliver til sceneskift
import-scrivener-choice-ornaments = Linjer med ornamenter bliver til sceneskift
import-scrivener-preview-label = Det nye projekts struktur
import-scrivener-col-title = Titel
import-scrivener-col-becomes = Bliver til
import-scrivener-col-exports = I eksporter
import-scrivener-in-exports-yes = Ja
import-scrivener-in-exports-no = Nej
# What a row the binder has no creation command for becomes.
import-scrivener-becomes-item = Element
import-scrivener-becomes-trashed = I papirkurven
import-scrivener-before-title = Før du importerer
import-scrivener-review-error-unreadable = Gennemgangen kunne ikke forberedes. Vælg projektet igen.
import-scrivener-review-error-choice = Dette valg passer ikke til projektet. Vælg projektet igen.
import-scrivener-review-error-unmappable = Med dette valg kunne ringbindet ikke overføres, uden at noget gik tabt. Vælg anderledes, eller rapportér problemet.

## Save step
import-scrivener-save-hint = Importen laver et nyt Skribisto-projekt. Dit Scrivener-projekt bliver kun læst, aldrig ændret.
import-scrivener-title-field = Titel
import-scrivener-author = Forfatter
import-scrivener-language = Sprog
import-scrivener-location-inside-project = Vælg en mappe uden for Scrivener-projektet. Derinde må der kun ligge Scriveners egne filer.
import-scrivener-copy-research-label = Kopiér researchfiler ved siden af det nye projekt
import-scrivener-copy-research = { $count ->
    [one] 1 researchfil, som Skribisto ikke kan åbne, f.eks. en PDF eller en lydfil, lægges i en mappe ved siden af projektet. Fjerner du fluebenet, fortæller dens note, hvor den ligger i dit Scrivener-projekt.
   *[other] { $count } researchfiler, som Skribisto ikke kan åbne, f.eks. PDF'er eller lydfiler, lægges i en mappe ved siden af projektet. Fjerner du fluebenet, fortæller deres noter, hvor de ligger i dit Scrivener-projekt.
}
# The file name proposed when the project's name holds nothing a file name can.
import-scrivener-default-name = Scrivener-projekt

## Progress and result
import-scrivener-progress-title = Importerer Scrivener-projekt…
import-scrivener-done-title = Scrivener-projektet er importeret
# How many rows the new project holds: the documents and folders that came across, and
# the rows the import made for them (a document's notes, the project notes).
import-scrivener-done = { $items ->
    [one] Det nye projekt indeholder 1 element.
   *[other] Det nye projekt indeholder { $items } elementer.
}
# The same, with how many kinds of thing could not be read or need checking.
import-scrivener-done-check = { $items ->
    [one] Det nye projekt indeholder 1 element.
   *[other] Det nye projekt indeholder { $items } elementer.
} { $findings ->
    [one] 1 ting kræver din opmærksomhed.
   *[other] { $findings } ting kræver din opmærksomhed.
}
import-scrivener-review-again = Gennemgå importen igen
## The import's report (the dialog See report opens)
import-scrivener-see-report = Se rapport
import-scrivener-report-from = Læst fra { $path }
import-scrivener-report-to = Skrevet til { $path }
import-scrivener-report-came-across = Hvad der blev overført
import-scrivener-report-nothing-counted = Intet blev talt.
import-scrivener-report-items = { $count ->
    [one] 1 element
   *[other] { $count } elementer
}
import-scrivener-report-words = { $count ->
    [one] 1 ord
   *[other] { $count } ord
}
import-scrivener-report-synopses = { $count ->
    [one] 1 synopsis
   *[other] { $count } synopser
}
import-scrivener-report-document-notes = { $count ->
    [one] 1 dokuments noter
   *[other] { $count } dokumenters noter
}
import-scrivener-report-comments = { $count ->
    [one] 1 kommentar
   *[other] { $count } kommentarer
}
import-scrivener-report-footnotes = { $count ->
    [one] 1 fodnote
   *[other] { $count } fodnoter
}
import-scrivener-report-pictures = { $count ->
    [one] 1 billede
   *[other] { $count } billeder
}
import-scrivener-report-templates = { $count ->
    [one] 1 noteskabelon
   *[other] { $count } noteskabeloner
}
import-scrivener-report-tags = { $count ->
    [one] 1 mærke
   *[other] { $count } mærker
}
import-scrivener-report-statuses = { $count ->
    [one] 1 status
   *[other] { $count } statusser
}
import-scrivener-report-trashed = { $count ->
    [one] 1 element i papirkurven
   *[other] { $count } elementer i papirkurven
}
import-scrivener-report-research-files = { $count ->
    [one] 1 researchfil
   *[other] { $count } researchfiler
}
import-scrivener-report-choices = Hvad du valgte
import-scrivener-report-accepted-mobile = Du importerede projektet uden de ændringer fra Scrivener til iPhone eller iPad, som endnu ikke var kommet med i det.
import-scrivener-report-accepted-no-text = Du importerede denne kopi med tomme dokumenter, da ingen af deres tekst var i den.
import-scrivener-report-choice = { $subject }: { $answer }
import-scrivener-report-yes = ja
import-scrivener-report-no = nej
import-scrivener-report-research-copied = Researchfiler, som Skribisto ikke kan åbne, blev kopieret til »{ $folder }« ved siden af det nye projekt.
import-scrivener-report-research-not-copied = Researchfiler, som Skribisto ikke kan åbne, blev ikke kopieret. Deres noter fortæller, hvor de ligger i dit Scrivener-projekt.
import-scrivener-report-research-copied-except = { $count ->
    [one] Researchfiler, som Skribisto ikke kan åbne, blev kopieret til »{ $folder }« ved siden af det nye projekt, undtagen 1, der bliver i dit Scrivener-projekt. Dens note fortæller, hvor den ligger.
   *[other] Researchfiler, som Skribisto ikke kan åbne, blev kopieret til »{ $folder }« ved siden af det nye projekt, undtagen { $count }, der bliver i dit Scrivener-projekt. Deres noter fortæller, hvor de ligger.
}
import-scrivener-report-research-none-copied = { $count ->
    [1] Du bad om at få researchfiler kopieret, men den eneste fil kunne ikke kopieres. Den bliver i dit Scrivener-projekt, og dens note fortæller, hvor den ligger.
   *[other] Du bad om at få researchfiler kopieret, men ingen af de { $count } kunne kopieres. De bliver i dit Scrivener-projekt, og deres noter fortæller, hvor de ligger.
}
import-scrivener-report-chose-project = Du valgte at importere »{ $project }«, et af flere projekter i den mappe eller det arkiv, du havde valgt.
import-scrivener-report-chose-manifest = Du valgte ringbindsfilen »{ $file }«.
import-scrivener-report-chose-enclosing = Du valgte noget inde i »{ $project }«, så hele projektet blev importeret.
import-scrivener-report-chose-upgraded = Du importerede »{ $current }«, det projekt, du fortsatte med at skrive i, og ikke den kopi, Scrivener gemte før en opdatering.
import-scrivener-report-chose-backup = Du importerede »{ $backup }«, den ældre kopi, Scrivener gemte før en opdatering.
import-scrivener-report-title-placeholder-rejected = Titlen i projektets indstillinger var en pladsholder fra en skabelon, så Skribisto foreslog i stedet projektets navn.
import-scrivener-report-author-placeholder-rejected = Forfatternavnet i projektets indstillinger var en pladsholder fra en skabelon, så Skribisto foreslog intet navn.
import-scrivener-report-title-from-compile-settings = Skribisto foreslog den titel, projektets kompileringsindstillinger angiver: det navn, projektet havde, da det blev oprettet, medmindre der er angivet en titel der i Scrivener.
import-scrivener-report-title-from-template = Skribisto foreslog skabelonens egen titel.
import-scrivener-report-title-from-folder-name = Skribisto foreslog projektets navn som titel.
import-scrivener-report-title-from-project-properties = Skribisto foreslog den titel, projektets egenskaber angiver.
import-scrivener-report-author-from-project = Skribisto foreslog det forfatternavn, projektets indstillinger angiver. En skabelons forfatter kan hænge ved der, og den ville blive trykt på en titelside.
import-scrivener-report-language-not-stated = Projektet angav ikke sit sprog, så Skribisto foreslog det sprog, programmet selv vises på. Det afgør, hvordan ord tælles, og hvordan citationstegn skrives.
import-scrivener-report-language-from-script = Skribisto foreslog sproget ud fra den skrift, manuskriptet er skrevet med ({ $tag }).
# A proposal of the Save page the writer replaced with their own.
import-scrivener-report-you-changed-it = { $hint } Du erstattede dette forslag.
import-scrivener-report-content-salvaged-text = Teksten til »{ $item }« manglede i projektet. Ordene kommer fra Scriveners søgeindeks, som ren tekst og måske fra en ældre version: kontrollér dem.
import-scrivener-report-content-salvaged-notes = Noterne til »{ $item }« manglede i projektet. Ordene kommer fra Scriveners søgeindeks, som ren tekst og måske fra en ældre version: kontrollér dem.
import-scrivener-report-content-salvaged-synopsis = Synopsen til »{ $item }« manglede i projektet. Ordene kommer fra Scriveners søgeindeks, som ren tekst og måske fra en ældre version: kontrollér dem.
# A finding of the document importer found in several places: its sentence about the
# first, then how many more the list under it names.
import-scrivener-report-document-others = { $others ->
    [one] { $sentence } Det samme sker 1 sted mere, som er angivet nedenfor.
   *[other] { $sentence } Det samme sker { $others } steder mere, som er angivet nedenfor.
}
# An item of the report, by its path in Scrivener's binder, with which of its writings
# the line is about when it is not the text.
import-scrivener-report-item-notes = { $path } (dokumentnoter)
import-scrivener-report-item-synopsis = { $path } (synopsis)
import-scrivener-report-item-project-note = { $path } (projektnote)
import-scrivener-report-item-recovered = { $path } (gendannet tekst)
# An item with no name in Scrivener.
import-scrivener-report-untitled = Uden titel
import-scrivener-report-no-findings = Importen har ikke mere at rapportere.
import-scrivener-report-group-unread = Kunne ikke læses
import-scrivener-report-group-check = Kontrollér disse
import-scrivener-report-group-changed = Overført på en anden måde
import-scrivener-report-group-left = Efterladt
import-scrivener-report-group-count = { $group } ({ $count })
import-scrivener-report-items-toggle = { $count ->
    [one] 1 element
   *[other] { $count } elementer
}
# The same, as a screen reader names it: with the line whose items it lists.
import-scrivener-report-items-toggle-named = { $count ->
    [one] 1 element: { $line }
   *[other] { $count } elementer: { $line }
}
import-scrivener-report-names-toggle = { $count ->
    [one] 1 navn
   *[other] { $count } navne
}
# The same, as a screen reader hears it, naming the line it belongs to.
import-scrivener-report-names-toggle-named = { $count ->
    [one] 1 navn: { $line }
   *[other] { $count } navne: { $line }
}
import-scrivener-report-more = { $count ->
    [one] Og 1 mere.
   *[other] Og { $count } mere.
}
import-scrivener-report-open-item = Åbn i editoren
import-scrivener-report-links-hint = Åbn det nye projekt, og vis så denne rapport igen fra kommandopaletten: så åbnes hvert element, der er nævnt her, med et klik.
import-scrivener-copy-report = Kopiér rapport
import-scrivener-report-copied = Rapporten er kopieret
import-scrivener-report-copy-failed = Rapporten kunne ikke kopieres.
import-scrivener-report-close = Luk
import-scrivener-report-none-title = Ingen importrapport
import-scrivener-report-none = Ingen Scrivener-import er afsluttet, siden Skribisto startede. Skribisto gemmer rapporterne fra de seneste importer, indtil programmet lukkes.
import-scrivener-report-gone = { $count ->
    [one] Rapporten fra denne import gemmes ikke længere. Skribisto gemmer rapporten fra den seneste import, indtil programmet lukkes.
   *[other] Rapporten fra denne import gemmes ikke længere. Skribisto gemmer rapporterne fra de sidste { $count } importer, indtil programmet lukkes.
}

## Words the import writes into the new project, in the language Skribisto shows itself in
import-scrivener-manuscript-binder = Manuskript
import-scrivener-notes-binder = Noter
import-scrivener-notes-of = { $title } (noter)
import-scrivener-project-notes = Projektnoter
import-scrivener-recovered = Gendannede tekster
import-scrivener-untitled-template = Skabelon uden titel
# The folder beside the new project its research files are copied into.
import-scrivener-research-folder = Researchfiler til { $name }
# Between two authors' names.
import-scrivener-list-separator = {", "}
import-scrivener-label-details = Fra feltet »{ $field }« i Scrivener
import-scrivener-status-details = Fra feltet »{ $field }« i Scrivener
import-scrivener-unnamed-label = Etiket { $number }
import-scrivener-unnamed-status = Status { $number }
import-scrivener-label-tag = { $name } (etiket)
import-scrivener-untitled-tag = Uden titel
# The tags, the pace plan and the "From Scrivener" folder the import makes from what a
# Scrivener project holds beyond its labels and status.
import-scrivener-keyword-details = Nøgleordet »{ $path }« i Scrivener
import-scrivener-unnamed-keyword = Nøgleord { $number }
import-scrivener-keyword-tag = { $name } (nøgleord)
import-scrivener-collection-details = En samling i Scrivener
import-scrivener-collection-tag = { $name } (samling)
import-scrivener-field-details = Fra feltet »{ $field }« i Scrivener
# A tag made from one option of a list field.
import-scrivener-field-value = { $field }: { $option }
import-scrivener-field-tag = { $name } (felt)
# The day off a pace plan takes on its deadline.
import-scrivener-deadline-day = Deadline
import-scrivener-from-scrivener = Fra Scrivener
import-scrivener-note-project = Projekt
import-scrivener-note-project-bookmarks = Projektbogmærker
import-scrivener-note-replacements = Kompileringserstatninger
import-scrivener-note-auto-complete = Poster til autofuldførelse
import-scrivener-note-saved-searches = Gemte søgninger
import-scrivener-note-custom-metadata = Brugerdefinerede metadata
import-scrivener-note-targets = Mål
import-scrivener-note-bookmarks = Bogmærker
import-scrivener-abbreviated-title = Forkortet titel
import-scrivener-subjects = Emner
import-scrivener-company = Firma
import-scrivener-session-target = Sessionsmål
import-scrivener-session-target-calculated = Sessionsmål, beregnet af Scrivener ud fra deadlinen
import-scrivener-writing-days = Skrivedage
import-scrivener-weekday-monday = mandag
import-scrivener-weekday-tuesday = tirsdag
import-scrivener-weekday-wednesday = onsdag
import-scrivener-weekday-thursday = torsdag
import-scrivener-weekday-friday = fredag
import-scrivener-weekday-saturday = lørdag
import-scrivener-weekday-sunday = søndag
import-scrivener-draft-target = Udkastmål
import-scrivener-deadline = Deadline
# A unit after a count, in parentheses: "1000 (words)".
import-scrivener-unit-words = ord
import-scrivener-unit-characters = tegn
import-scrivener-unit-pages = sider
import-scrivener-search-query = Søgte efter
import-scrivener-search-in = Søgte i
import-scrivener-search-operator = Matchning
# What a saved search looks in, by the kind Scrivener names.
import-scrivener-search-in-all = alt
import-scrivener-search-in-title = titler
import-scrivener-search-in-text = tekst
import-scrivener-search-in-notes = noter
import-scrivener-search-in-synopsis = synopser
import-scrivener-search-in-keywords = nøgleord
import-scrivener-search-in-label = etiketter
import-scrivener-search-in-status = statusser
import-scrivener-search-in-section-type = sektionstyper
import-scrivener-search-in-custom-metadata = brugerdefinerede metadata
# How a saved search's words combine.
import-scrivener-search-any = et af ordene
import-scrivener-search-all = alle ordene
import-scrivener-search-exact = den nøjagtige frase
# One line of a listing: a bookmark's place and its note, or a link's words and its address.
import-scrivener-label-value = { $label }: { $value }
import-scrivener-replacement-regex = regulært udtryk
import-scrivener-replacement-off = slået fra
# Said of a note whose own text lists its custom metadata or bookmarks.
import-scrivener-in-own-note = Angivet i den note, dette dokument blev til.


## Import a Scrivener project: what the import says about the project, in the writer's words.
## Every key the converter can give has a line here (see `import_scrivener::messages`).

# A refusal on one line: its title, then what to do.
import-scrivener-paragraph = { $title }. { $body }

## When the project cannot be imported
import-scrivener-refusal-missing-title = Der er intet på denne sti
import-scrivener-refusal-missing-body = »{ $file }« er der ikke længere. Vælg projektet igen.
import-scrivener-refusal-unreadable-title = Dette projekt kan ikke læses
import-scrivener-refusal-unreadable-body = Skribisto kunne ikke læse »{ $file }« ({ $reason }). Kontrollér, at du kan åbne den, og prøv så igen.
import-scrivener-refusal-not-a-scrivener-project-title = Intet Scrivener-projekt her
import-scrivener-refusal-not-a-scrivener-project-body = Vælg projektets .scriv-mappe, .scrivx-filen i den, en .zip-sikkerhedskopi eller en .scrivtemplate.
import-scrivener-refusal-manifest-without-files-title = Dokumenterne er ikke i denne kopi
import-scrivener-refusal-manifest-without-files-body = »{ $manifest }« er her, men ikke mappen Files med de dokumenter, den angiver. En synkroniseringstjeneste er måske ikke færdig med at kopiere projektet, eller kun .scrivx-filen blev kopieret. Valgte du .scrivx-filen, så vælg i stedet projektets mappe med »Vælg mappe…«.
import-scrivener-refusal-search-too-wide-title = Mappen indeholder for meget til at blive gennemsøgt
import-scrivener-refusal-search-too-wide-body = Den har mere end { $limit } mapper at gennemse. Vælg selve Scrivener-projektet.
import-scrivener-refusal-scrivener-one-for-mac-title = Dette projekt er lavet med Scrivener 1 til Mac
import-scrivener-refusal-scrivener-one-for-mac-body = Skribisto læser projekter fra Scrivener 2 og nyere. Åbn det én gang i Scrivener 2 eller 3 for at opdatere det, eller kompilér det til et Word-dokument, og importér det.
import-scrivener-refusal-ios-stub-without-mobile-data-title = Dette er kun en pladsholder fra Scrivener til iPhone eller iPad
import-scrivener-refusal-ios-stub-without-mobile-data-body = Selve projektet er her ikke. Synkronisér det fra din iPhone eller iPad, eller del det derfra som en .zip, og importér den.
import-scrivener-refusal-newer-title = Dette projekt er lavet med en nyere version af Scrivener
import-scrivener-refusal-unknown-format-version-body = Formatet, { $version }, er ikke et, Skribisto kender. Se efter en opdatering af Skribisto. Indtil da kan du kompilere projektet til et Word-dokument i Scrivener og importere det.
import-scrivener-refusal-newer-format-stamp-body = Filerne er mærket som version { $stamp }, og Skribisto læser op til version { $newest }. Se efter en opdatering af Skribisto. Indtil da kan du kompilere projektet til et Word-dokument i Scrivener og importere det.
import-scrivener-refusal-version-stamp-unreadable-title = Projektets version kan ikke læses
import-scrivener-refusal-version-stamp-unreadable-body = Filen Files/version.txt indeholder intet versionsnummer, så Skribisto kan ikke afgøre, om programmet kan læse projektet. Åbn projektet én gang i Scrivener, og importér det så igen.
import-scrivener-refusal-damaged-manifest-title = Ringbindsfilen kan ikke læses
import-scrivener-refusal-damaged-manifest-body = »{ $manifest }« er beskadiget. { $spare_copies ->
    [0] Åbn projektet i Scrivener, som kan reparere filen, og importér det så igen.
    [1] Scrivener gemmer en reservekopi af den inde i projektet, som Skribisto ikke læser. Åbn projektet i Scrivener, som kan reparere filen, og importér det så igen.
   *[other] Scrivener gemmer { $spare_copies } reservekopier af den inde i projektet, som Skribisto ikke læser. Åbn projektet i Scrivener, som kan reparere filen, og importér det så igen.
}
# The empty file Scrivener writes for a new project before its first save: not damage.
import-scrivener-refusal-damaged-manifest-new-project-placeholder-body = »{ $manifest }« er den tomme fil, Scrivener skriver for et projekt, programmet endnu ikke har gemt. Åbn projektet i Scrivener, lad Scrivener gemme det, og importér det så igen.
import-scrivener-refusal-manifest-too-large-title = Denne ringbindsfil er for stor
import-scrivener-refusal-manifest-too-large-body = »{ $manifest }« er større end de { $limit } MB, Skribisto læser. Intet rigtigt projekt kommer i nærheden af det, så filen er måske beskadiget.
import-scrivener-refusal-no-item-identifiers-title = Ringbindet angiver intet dokument, Skribisto kan finde
import-scrivener-refusal-no-item-identifiers-body = Ingen af dets elementer har en identifikator, så deres filer kan ikke findes. Åbn projektet én gang i Scrivener, og importér det så igen.
import-scrivener-refusal-incomplete-copy-title = Dokumenterne mangler i denne kopi
import-scrivener-refusal-incomplete-copy-body = Mappen Files er her, men dokumenterne i den er ikke. En synkroniseringstjeneste er måske ikke færdig med at kopiere projektet. Vent, til den er færdig, eller kopiér hele .scriv-mappen på ny, og læs så projektet igen.
import-scrivener-refusal-archive-title = Arkivet kan ikke åbnes
import-scrivener-refusal-archive-unreadable-body = »{ $file }« kunne ikke læses ({ $reason }). Det er måske beskadiget. Pak det selv ud, og vælg så .scriv-mappen i det.
import-scrivener-refusal-archive-unsafe-name-title = Arkivet kan ikke åbnes sikkert
# A name with `..` in it, or an absolute one.
import-scrivener-refusal-archive-unsafe-name-body = »{ $file }« indeholder en fil ved navn »{ $member }«, som ville havne uden for arkivet, så Skribisto åbner det ikke.
import-scrivener-refusal-archive-unsafe-name-empty-body = »{ $file }« indeholder en fil uden navn, så Skribisto åbner det ikke.
import-scrivener-refusal-archive-unsafe-name-control-character-body = »{ $file }« indeholder en fil, hvis navn indeholder et kontroltegn, så Skribisto åbner det ikke.
import-scrivener-refusal-archive-too-large-title = Arkivet er for stort til at blive åbnet sikkert
import-scrivener-refusal-archive-too-many-members-body = Det indeholder mere end { $limit } filer. Pak det selv ud, og vælg så .scriv-mappen i det.
import-scrivener-refusal-archive-member-too-large-body = »{ $member }« fylder mere end { $limit } MB udpakket. Pak arkivet selv ud, og vælg så .scriv-mappen i det.
import-scrivener-refusal-archive-too-large-body = Det fylder mere end { $limit } MB udpakket. Pak det selv ud, og vælg så .scriv-mappen i det.
import-scrivener-refusal-archive-encrypted-title = Arkivet er beskyttet med en adgangskode
import-scrivener-refusal-archive-encrypted-body = Pak »{ $file }« selv ud, og vælg så .scriv-mappen i det.
import-scrivener-refusal-archive-unsupported-compression-title = Arkivet er pakket på en måde, Skribisto ikke kan læse
import-scrivener-refusal-archive-unsupported-compression-body = »{ $member }« er komprimeret med { $method }. Pak arkivet selv ud, og vælg så .scriv-mappen i det.
import-scrivener-refusal-duplicate-archive-members-title = Arkivet angiver nogle filer to gange
import-scrivener-refusal-duplicate-archive-members-body = »{ $file }« indeholder mere end én fil under samme navn, så Skribisto kan ikke afgøre, hvilken der hører til projektet. Pak det selv ud, og vælg så .scriv-mappen i det.
# What a file cut short looks like; a file too large, one that is not a template and one
# encoded another way have their own words.
import-scrivener-refusal-template-undecodable-title = Skabelonfilen er ufuldstændig
import-scrivener-refusal-template-undecodable-too-large-title = Skabelonfilen er for stor til at blive åbnet
import-scrivener-refusal-template-undecodable-not-a-template-title = Filen er ikke en Scrivener-skabelon
import-scrivener-refusal-template-undecodable-unknown-encoding-title = Skabelonfilen er gemt i en form, Skribisto ikke kender
import-scrivener-refusal-template-undecodable-body = »{ $file }« kan ikke afkodes: den slutter måske, før projektet i den gør. Hent den igen, eller åbn den i Scrivener, og lav et projekt ud fra den.
import-scrivener-refusal-template-undecodable-too-large-body = »{ $file }« er større end { $limit } MB. Åbn den i Scrivener, lav et projekt ud fra den, og importér så det projekt.
import-scrivener-refusal-template-undecodable-not-a-template-body = »{ $file }« indeholder intet Scrivener-projekt. Vælg den skabelonfil, Scrivener gemte, eller et projekt, der er lavet ud fra den.
import-scrivener-refusal-template-undecodable-unknown-encoding-body = »{ $file }« gemmer sit projekt på en måde, Skribisto ikke kan afkode. Åbn den i Scrivener, lav et projekt ud fra den, og importér så det projekt.
import-scrivener-refusal-cloud-placeholders-title = { $count ->
    [one] 1 fil i dette projekt ligger stadig i skyen
   *[other] { $count } filer i dette projekt ligger stadig i skyen
}
import-scrivener-refusal-cloud-placeholders-body = { $count ->
    [1] Den er ikke hentet ned til denne computer: { $examples }. Gør projektet tilgængeligt offline i din synkroniseringsapp, og læs så projektet igen.
   *[other] De er ikke hentet ned til denne computer, f.eks. { $examples }. Gør projektet tilgængeligt offline i din synkroniseringsapp, og læs så projektet igen.
}
import-scrivener-refusal-unreadable-members-title = { $count ->
    [one] 1 fil i dette projekt kan ikke læses
   *[other] { $count } filer i dette projekt kan ikke læses
}
import-scrivener-refusal-unreadable-members-body = For eksempel { $examples }. Sørg for, at du har adgang til at læse hele projektet, og at disken er i orden, og læs så projektet igen.
import-scrivener-refusal-needed-member-is-link-title = Projektet peger uden for sin egen mappe
import-scrivener-refusal-needed-member-is-link-body = »{ $member }« er et link til et andet sted på denne computer, og Skribisto følger ikke links. Kopiér projektet, så alle filer ligger inde i det, og prøv så igen.
import-scrivener-refusal-project-changed-during-read-title = Projektet blev ændret, mens det blev læst
import-scrivener-refusal-project-changed-during-read-body = Scrivener eller en synkroniseringstjeneste skriver stadig til det. Luk det i Scrivener, vent, til synkroniseringen er færdig, og læs så projektet igen.
import-scrivener-refusal-too-many-members-title = Projektet indeholder for mange filer
import-scrivener-refusal-too-many-members-body = Det har mere end { $limit } filer, flere end noget rigtigt projekt. Det er måske ikke et Scrivener-projekt, eller det er måske beskadiget.
import-scrivener-refusal-too-large-title = Projektet er for stort
import-scrivener-refusal-too-large-body = Dets tekst fylder mere end { $limit } MB, mere end Skribisto kan importere sikkert.
import-scrivener-refusal-structure-unmappable-title = Skribisto kan ikke omsætte dette ringbind
import-scrivener-refusal-structure-unmappable-body = Ringbindet kunne ikke overføres, uden at noget gik tabt eller havnede forkert. Det er en fejl i Skribisto, ikke i dit projekt. Rapportér den gerne sammen med det, der står under »Detaljer«.

## When there is a choice to make
import-scrivener-roots-title = { $count ->
    [one] Her ligger { $count } projekt. Hvilket af dem?
   *[other] Her ligger { $count } projekter. Hvilket af dem?
}
import-scrivener-roots-hint = Denne mappe eller dette arkiv indeholder mere end ét Scrivener-projekt. Vælg det, der skal importeres.
import-scrivener-manifests-title = { $count ->
    [one] Projektet har { $count } ringbindsfil
   *[other] Projektet har { $count } ringbindsfiler
}
import-scrivener-manifests-hint = Scrivener gemmer ringbindet, listen over alle dokumenter og mapper, i en .scrivx-fil. De ekstra filer er sandsynligvis efterladt af en synkroniseringstjeneste. Vælg den, du sidst arbejdede i.
import-scrivener-manifests-hint-preselected = Scrivener gemmer ringbindet, listen over alle dokumenter og mapper, i en .scrivx-fil. De ekstra filer er sandsynligvis efterladt af en synkroniseringstjeneste. Den valgte er den, du sandsynligvis sidst arbejdede i: kontrollér den, før du går videre.
import-scrivener-manifest-row = { $file } · gemt { $date } · { $documents ->
    [one] 1 element
   *[other] { $documents } elementer
}
import-scrivener-manifest-row-undated = { $file } · { $documents ->
    [one] 1 element
   *[other] { $documents } elementer
}
import-scrivener-manifest-row-unread = { $file } · kan ikke læses
import-scrivener-enclosing-title = Dette er en del af Scrivener-projektet »{ $project }«
import-scrivener-enclosing-body = Du valgte en fil eller mappe inde i et projekt. Skribisto importerer hele projektet.
import-scrivener-enclosing-use = Importér »{ $project }«
import-scrivener-upgrade-title = Dette er den kopi, Scrivener gemte før en opdatering
import-scrivener-upgrade-body = Når Scrivener opdaterer et projekt til en nyere version, gemmer programmet det gamle som »{ $backup }«. Det projekt, du fortsatte med at skrive i, er »{ $current }« ved siden af.
import-scrivener-upgrade-body-unclear = Når Scrivener opdaterer et projekt til en nyere version, gemmer programmet det gamle som »{ $backup }«. Projektet ved siden af, »{ $current }«, kunne ikke læses fejlfrit, så vælg, hvilket der skal importeres.
import-scrivener-upgrade-use-current = Importér »{ $current }«, det projekt, du fortsatte med at skrive i
import-scrivener-upgrade-keep-backup = Importér denne ældre kopi, »{ $backup }«

## What must be accepted before going on
import-scrivener-ios-pending-title = Ændringer fra Scrivener til iPhone eller iPad er endnu ikke i denne kopi
import-scrivener-ios-pending-body = Scrivener på en computer henter de ændringer ind, næste gang programmet åbner projektet. Åbn projektet én gang i Scrivener på din computer, og importér det så igen.
import-scrivener-ios-pending-accept = Importér det uden de ændringer
import-scrivener-no-text-title = { $count ->
    [one] 1 dokument og ikke ét ord tekst i denne kopi
   *[other] { $count } dokumenter og ikke ét ord tekst i denne kopi
}
import-scrivener-no-text-body = En synkroniseringstjeneste kopierer måske stadig projektet, eller kopien har mistet sine dokumenter. Importeres den nu, kommer alle dokumenter tomme ind.
import-scrivener-no-text-accept = Importér dokumenterne tomme

## Things to know, nothing to decide
import-scrivener-notice-project-found-below-pick-title = Projektet blev fundet inde i den mappe, du valgte
import-scrivener-notice-project-found-below-pick-body = Det ligger i »{ $path }«.
import-scrivener-notice-source-is-template-title = Dette er en Scrivener-skabelon
import-scrivener-notice-source-is-template-named = Dette er Scrivener-skabelonen »{ $title }«
import-scrivener-notice-source-is-template-body = Importerer du den, laves et nyt projekt ud fra det, skabelonen indeholder, inklusive dens vejledning.
import-scrivener-notice-source-is-backup-title = Dette ligner en sikkerhedskopi, Scrivener har lavet
import-scrivener-notice-source-is-backup-body = Den indeholder projektet, som det var, da sikkerhedskopien blev lavet. Skrev du videre efter det, så vælg selve projektet.
import-scrivener-notice-importing-pre-upgrade-copy-title = Du importerer den kopi, Scrivener gemte før en opdatering
import-scrivener-notice-importing-pre-upgrade-copy-body = Det projekt, du fortsatte med at skrive i, ligger ved siden af og indeholder alt, hvad der er skrevet siden.
import-scrivener-notice-imported-from-ios-stub-title = Dette projekt er lavet i Scrivener til iPhone eller iPad
import-scrivener-notice-imported-from-ios-stub-body = Det er aldrig blevet åbnet på en computer, så det læses fra den kopi, din iPhone eller iPad gemmer.
import-scrivener-notice-stale-mobile-binder-ignored-title = Et gammelt ringbind fra en iPhone eller iPad lægges til side
import-scrivener-notice-stale-mobile-binder-ignored-body = Scrivener på en computer har allerede hentet dets ændringer ind.
import-scrivener-notice-identical-manifest-copies-title = { $count ->
    [one] Projektet har { $count } identisk ringbindsfil
   *[other] Projektet har { $count } identiske ringbindsfiler
}
import-scrivener-notice-identical-manifest-copies-body = De indeholder det samme, så hvilken som helst af dem giver det samme projekt.
import-scrivener-notice-manifest-copies-ignored-title = Ekstra kopier af ringbindsfilen lægges til side
import-scrivener-notice-manifest-copies-ignored-body = »{ $chosen }« er den, Scrivener sidst arbejdede i. { $count ->
    [one] 1 kopi efterladt af en synkroniseringstjeneste lægges til side.
   *[other] { $count } kopier efterladt af en synkroniseringstjeneste lægges til side.
}
import-scrivener-notice-pinned-manifest-looks-like-a-copy-title = Den ringbindsfil, du valgte, ligner en synkroniseringskopi
import-scrivener-notice-pinned-manifest-looks-like-a-copy-body = »{ $name }« er navngivet som en kopi af »{ $original }«, der ligger ved siden af. Ser importen gammel ud, så vælg i stedet »{ $original }«.
import-scrivener-notice-pinned-manifest-has-siblings-title = { $count ->
    [one] Der ligger 1 anden ringbindsfil ved siden af den, du valgte
   *[other] Der ligger { $count } andre ringbindsfiler ved siden af den, du valgte
}
import-scrivener-notice-pinned-manifest-has-siblings-body = Kun den, du valgte, læses.
import-scrivener-notice-legacy-v1-files-beside-title = Filer fra en ældre version af Scrivener lægges til side
import-scrivener-notice-legacy-v1-files-beside-body = De blev efterladt, da projektet blev opdateret. Kun filerne fra den nuværende version læses.
import-scrivener-notice-mixed-payload-trees-title = Projektet indeholder filer fra to versioner af Scrivener
import-scrivener-notice-mixed-payload-trees-body = Dokumenterne læses der, hvor ringbindsfilen siger, at de ligger.
import-scrivener-notice-version-disagreement-title = Projektets filer er ikke enige om dets version
import-scrivener-notice-id-scheme-mismatch-body = Ringbindsfilen siger version { $version }, men navngiver sine dokumenter, som en anden version gør. De læses der, hvor den peger. Ser importen forkert ud, så åbn projektet én gang i Scrivener, og importér det igen.
import-scrivener-notice-version-stamp-mismatch-body = Projektet læses, som ringbindsfilen beskriver det. Ser importen forkert ud, så åbn projektet én gang i Scrivener, og importér det igen.
import-scrivener-notice-unrecognised-writer-title = Projektet blev gemt af et program, Skribisto ikke genkender
import-scrivener-notice-unrecognised-writer-named = Programmet kalder sig »{ $creator }«. Projektet læses som et Scrivener-projekt.
import-scrivener-notice-unrecognised-writer-body = Programmet angiver ikke sit navn. Projektet læses som et Scrivener-projekt.
import-scrivener-notice-invalid-item-identifier-title = { $count ->
    [one] 1 element i ringbindet har en identifikator, der ikke er gyldig
   *[other] { $count } elementer i ringbindet har en identifikator, der ikke er gyldig
}
import-scrivener-notice-invalid-item-identifier-body = En sådan identifikator lægges til side, og elementet, der bærer den, bevares.
import-scrivener-notice-items-without-identifier-title = { $count ->
    [one] 1 element i ringbindet har ingen identifikator
   *[other] { $count } elementer i ringbindet har ingen identifikator
}
import-scrivener-notice-items-without-identifier-body = Uden en identifikator kan et elements filer ikke findes, så elementet kommer tomt ind.
import-scrivener-notice-duplicate-item-key-title = To elementer i ringbindet deler én identifikator: »{ $item }«
import-scrivener-notice-duplicate-item-key-body = Det første beholder dokumenterne. Det andet beholder sin plads og kommer tomt ind.
import-scrivener-notice-lock-here-title = Scrivener har dette projekt åbent på denne computer
import-scrivener-notice-lock-here-body = Luk det først i Scrivener, så alle ændringer bliver gemt.
import-scrivener-notice-lock-title = Scrivener havde dette projekt åbent på »{ $host }«
import-scrivener-notice-lock-elsewhere-title = Scrivener havde dette projekt åbent på en anden computer
import-scrivener-notice-lock-body = Er det stadig åbent der, så luk det først: filerne indeholder måske endnu ikke de seneste ændringer. Blev Scrivener lukket for længe siden, kan du importere projektet, som det er.
import-scrivener-notice-archived-while-open-title = Arkivet blev lavet, mens Scrivener havde projektet åbent
import-scrivener-notice-archived-while-open-body = Ændringer, der blev lavet efter det tidspunkt, mangler måske i det.
import-scrivener-notice-content-missing-notes = Noterne til »{ $item }« mangler
import-scrivener-notice-content-missing-synopsis = Synopsen til »{ $item }« mangler
import-scrivener-notice-content-missing-text = Teksten til »{ $item }« mangler
import-scrivener-notice-content-missing-body = { $place ->
    [notes] Filen til noterne er ikke i denne kopi, så de kommer tomme ind. En synkroniseringstjeneste kopierer måske stadig projektet.
    [synopsis] Filen til synopsen er ikke i denne kopi, så den kommer tom ind. En synkroniseringstjeneste kopierer måske stadig projektet.
   *[text] Filen til teksten er ikke i denne kopi, så den kommer tom ind. En synkroniseringstjeneste kopierer måske stadig projektet.
}
import-scrivener-notice-content-missing-salvaged = { $place ->
    [notes] Filen til noterne er ikke i denne kopi, men Scriveners søgeindeks indeholdt stadig deres ord. Ordene bevares som ren tekst, måske fra en ældre version.
    [synopsis] Filen til synopsen er ikke i denne kopi, men Scriveners søgeindeks indeholdt stadig dens ord. Ordene bevares som ren tekst, måske fra en ældre version.
   *[text] Filen til teksten er ikke i denne kopi, men Scriveners søgeindeks indeholdt stadig dens ord. Ordene bevares som ren tekst, måske fra en ældre version.
}
import-scrivener-notice-content-truncated-notes = Noterne til »{ $item }« er tomme på disken
import-scrivener-notice-content-truncated-synopsis = Synopsen til »{ $item }« er tom på disken
import-scrivener-notice-content-truncated-text = Teksten til »{ $item }« er tom på disken
import-scrivener-notice-content-truncated-body = { $place ->
    [notes] Scrivener registrerede indhold i noterne, men filen til dem indeholder intet: en synkronisering er måske stoppet halvvejs. Noterne kommer tomme ind.
    [synopsis] Scrivener registrerede indhold i synopsen, men filen til den indeholder intet: en synkronisering er måske stoppet halvvejs. Synopsen kommer tom ind.
   *[text] Scrivener registrerede indhold i teksten, men filen til den indeholder intet: en synkronisering er måske stoppet halvvejs. Teksten kommer tom ind.
}
import-scrivener-notice-missing-media-title = Filen til »{ $item }« mangler
import-scrivener-notice-missing-media-body = Det billede eller den researchfil, elementet indeholder, er ikke i denne kopi.
import-scrivener-notice-sync-conflict-copies-title = { $count ->
    [one] Der blev fundet 1 synkroniseringskopi af et dokument
   *[other] Der blev fundet { $count } synkroniseringskopier af dokumenter
}
import-scrivener-notice-sync-conflict-copies-body = { $missing ->
    [0] Synkroniseringskopier overføres som gendannede tekster i deres egen mappe, så du kan sammenligne dem med dokumenterne.
    [one] Synkroniseringskopier overføres som gendannede tekster i deres egen mappe. { $missing } af dem er en kopi af et dokument, der ikke længere er i projektet.
   *[other] Synkroniseringskopier overføres som gendannede tekster i deres egen mappe. { $missing } af dem er kopier af dokumenter, der ikke længere er i projektet.
}
import-scrivener-notice-recovered-unreferenced-text-title = { $count ->
    [one] Der blev fundet 1 dokument, som intet element i ringbindet angiver
   *[other] Der blev fundet { $count } dokumenter, som intet element i ringbindet angiver
}
import-scrivener-notice-recovered-unreferenced-text-body = Dokumenter som disse overføres som gendannede tekster i deres egen mappe.
import-scrivener-notice-sync-debris-ignored-title = { $count ->
    [one] 1 ekstra kopi efterladt af en synkroniseringstjeneste lægges til side
   *[other] { $count } ekstra kopier efterladt af en synkroniseringstjeneste lægges til side
}
import-scrivener-notice-sync-debris-ignored-body = Scrivener læser heller ikke sådanne kopier. Kun projektets egne filer læses.
import-scrivener-notice-unrecognised-files-title = { $count ->
    [one] 1 fil, der ikke hører til et Scrivener-projekt, lægges til side
   *[other] { $count } filer, der ikke hører til et Scrivener-projekt, lægges til side
}
import-scrivener-notice-examples = For eksempel { $examples }.
import-scrivener-notice-symlink-not-followed-title = { $count ->
    [one] 1 link inde i projektet følges ikke
   *[other] { $count } links inde i projektet følges ikke
}
import-scrivener-notice-unreadable-files-skipped-title = { $count ->
    [one] 1 fil, som importen ikke har brug for, kunne ikke læses
   *[other] { $count } filer, som importen ikke har brug for, kunne ikke læses
}
import-scrivener-notice-cloud-placeholders-skipped-title = { $count ->
    [one] 1 fil, der stadig ligger i skyen, springes over
   *[other] { $count } filer, der stadig ligger i skyen, springes over
}
import-scrivener-notice-cloud-placeholders-skipped-body = Intet af det, importen har brug for, ligger stadig i skyen.
import-scrivener-notice-no-text-found-title = { $count ->
    [1] 1 dokument, og det indeholder ingen tekst
   *[other] { $count } dokumenter, og ingen af dem indeholder tekst
}
import-scrivener-notice-no-text-found-body = Projektet ser tomt ud. Det importeres, som det er.
import-scrivener-notice-metadata-unreadable-title = »{ $member }« kunne ikke læses
import-scrivener-notice-metadata-unreadable-body = Det, Scrivener gemte i filen, overføres ikke.
import-scrivener-notice-dangling-keyword-title = { $count ->
    [one] 1 henvisning peger på et nøgleord, projektet ikke definerer
   *[other] { $count } henvisninger peger på nøgleord, projektet ikke definerer
}
import-scrivener-notice-dangling-label-title = { $count ->
    [one] 1 henvisning peger på en etiket, projektet ikke definerer
   *[other] { $count } henvisninger peger på etiketter, projektet ikke definerer
}
import-scrivener-notice-dangling-status-title = { $count ->
    [one] 1 henvisning peger på en status, projektet ikke definerer
   *[other] { $count } henvisninger peger på statusser, projektet ikke definerer
}
import-scrivener-notice-dangling-section-type-title = { $count ->
    [one] 1 henvisning peger på en sektionstype, projektet ikke definerer
   *[other] { $count } henvisninger peger på sektionstyper, projektet ikke definerer
}
import-scrivener-notice-dangling-custom-field-title = { $count ->
    [one] 1 henvisning peger på et brugerdefineret metadatafelt, projektet ikke definerer
   *[other] { $count } henvisninger peger på brugerdefinerede metadatafelter, projektet ikke definerer
}
import-scrivener-notice-dangling-item-title = { $count ->
    [one] 1 henvisning peger på et dokument, projektet ikke definerer
   *[other] { $count } henvisninger peger på dokumenter, projektet ikke definerer
}
import-scrivener-notice-dangling-references-body = En henvisning til noget, som projektet ikke definerer, udelades.
import-scrivener-notice-snapshots-not-imported-line = { $count ->
    [one] 1 øjebliksbillede bliver i dit Scrivener-projekt: Skribisto overfører ikke øjebliksbilleder.
   *[other] { $count } øjebliksbilleder bliver i dit Scrivener-projekt: Skribisto overfører ikke øjebliksbilleder.
}
import-scrivener-notice-writing-history-not-imported-line = { $days ->
    [one] Scriveners oversigt over, hvor mange ord der er skrevet hver dag (1 dag), overføres ikke. Skribisto begynder sin egen optælling fra den første gemning.
   *[other] Scriveners oversigt over, hvor mange ord der er skrevet hver dag ({ $days } dage), overføres ikke. Skribisto begynder sin egen optælling fra den første gemning.
}
import-scrivener-notice-title-placeholder-rejected-hint = Titlen i projektets indstillinger er en pladsholder fra en skabelon, så projektets navn foreslås i stedet.
import-scrivener-notice-author-placeholder-rejected-hint = Forfatternavnet i projektets indstillinger er en pladsholder fra en skabelon, så der foreslås intet navn.
import-scrivener-notice-title-from-project-hint-compile-settings = Denne titel er den, projektets kompileringsindstillinger angiver. Indtil der angives en titel der, beholder Scrivener det navn, projektet havde, da det blev oprettet.
import-scrivener-notice-title-from-project-hint-template = Denne titel er skabelonens egen.
import-scrivener-notice-title-from-project-hint-folder-name = Denne titel er projektets navn.
import-scrivener-notice-title-from-project-hint-project-properties = Denne titel er den, projektets egenskaber angiver.
import-scrivener-notice-author-from-project-hint = Dette navn er det, projektets indstillinger angiver. En skabelons forfatter kan hænge ved der, og den ville blive trykt på en titelside.
import-scrivener-notice-language-not-stated-hint = Projektet angiver ikke sit sprog, så det sprog, Skribisto selv vises på, foreslås. Det afgør, hvordan ord tælles, og hvordan citationstegn skrives.
import-scrivener-notice-language-from-script-hint = Foreslået ud fra den skrift, manuskriptet er skrevet med ({ $tag }).

## Before you import: what the import does with the binder, or leaves behind
import-scrivener-structure-script-mode-draft-line = { $script ->
    [1] { $documents ->
        [1] Manuskriptets eneste dokument er skrevet i drejebogstilstand. Skribisto beholder ordene, ikke drejebogsopsætningen.
       *[other] 1 af de { $documents } manuskriptdokumenter er skrevet i drejebogstilstand. Skribisto beholder ordene, ikke drejebogsopsætningen.
    }
   *[other] { $script } af de { $documents } manuskriptdokumenter er skrevet i drejebogstilstand. Skribisto beholder ordene, ikke drejebogsopsætningen.
}
import-scrivener-structure-one-document-manuscript-line = { $count ->
    [1] Ét manuskriptdokument indeholder sine kapitler som overskrifter.
   *[other] { $count } manuskriptdokumenter indeholder deres kapitler som overskrifter.
}
import-scrivener-structure-title-repeats-number-line = { $count ->
    [one] 1 kapitel- eller deltitel indeholder også sit nummer, f.eks. »Kapitel II« eller »Første del«, så en eksport udskriver måske nummeret to gange.
   *[other] { $count } kapitel- eller deltitler indeholder også deres nummer, f.eks. »Kapitel II« eller »Første del«, så en eksport udskriver måske nummeret to gange.
}
import-scrivener-structure-claim-overruled-line = { $count ->
    [one] 1 element ligner en bog, en del eller et kapitel, men ligger inde i et element, der allerede er på det niveau, så det overføres på et lavere niveau.
   *[other] { $count } elementer ligner bøger, dele eller kapitler, men hvert af dem ligger inde i et element, der allerede er på det niveau, så de overføres på et lavere niveau.
}
import-scrivener-structure-adopted-by-chapter-line = { $count ->
    [one] 1 dokument, der følger efter et kapitel, bliver til slutningen af det kapitel.
   *[other] { $count } dokumenter, der følger efter et kapitel, bliver til slutningen af det kapitel.
}
import-scrivener-structure-looks-like-manuscript-line = { $count ->
    [one] 1 mappe på øverste niveau, der placeres i noterne, ligner manuskript. Du kan sende den til manuskriptet som endnu en bog.
   *[other] { $count } mapper på øverste niveau, der placeres i noterne, ligner manuskript. Du kan sende dem til manuskriptet som flere bøger.
}
import-scrivener-structure-generic-sheet-title-line = { $count ->
    [one] 1 opslag i historiebiblen har stadig en skabelons titel, f.eks. »Helt«. Skribisto leder efter den titel i din tekst og viser hvert sted, hvor den forekommer, under »I teksten«: omdøb opslaget, hvis ordet er almindeligt i din bog.
   *[other] { $count } opslag i historiebiblen har stadig en skabelons titel, f.eks. »Helt«. Skribisto leder efter de titler i din tekst og viser hvert sted, hvor de forekommer, under »I teksten«: omdøb opslagene, hvis ordene er almindelige i din bog.
}
import-scrivener-structure-placeholder-title-line = { $count ->
    [one] 1 titel navngav kun sit niveau og efterlades tom: ringbindet og eksporterne nummererer den.
   *[other] { $count } titler navngav kun deres niveau og efterlades tomme: ringbindet og eksporterne nummererer dem.
}
import-scrivener-structure-untitled-item-line = { $count ->
    [one] 1 element havde ingen titel i Scrivener og forbliver uden titel.
   *[other] { $count } elementer havde ingen titel i Scrivener og forbliver uden titel.
}
import-scrivener-structure-title-cleaned-line = { $count ->
    [one] 1 titel indeholdt linjeskift eller tabulatorer, som fjernes.
   *[other] { $count } titler indeholdt linjeskift eller tabulatorer, som fjernes.
}
import-scrivener-structure-own-text-moved-line = { $count ->
    [one] 1 mappe har sin egen tekst, som flyttes til et første dokument inde i den.
   *[other] { $count } mapper har deres egen tekst, som flyttes til et første dokument inde i dem.
}
import-scrivener-structure-text-with-children-line = { $count ->
    [one] 1 dokument med dokumenter inde i sig bliver til en mappe.
   *[other] { $count } dokumenter med dokumenter inde i sig bliver til mapper.
}
import-scrivener-structure-empty-folder-plain-line = { $count ->
    [one] 1 tom mappe forbliver en almindelig mappe, så ingen eksport udskriver en overskrift, som Scrivener aldrig udskrev.
   *[other] { $count } tomme mapper forbliver almindelige mapper, så ingen eksport udskriver en overskrift, som Scrivener aldrig udskrev.
}
import-scrivener-structure-book-inside-book-line = { $count ->
    [one] 1 bog inde i en bog bliver til en del.
   *[other] { $count } bøger inde i en bog bliver til dele.
}
import-scrivener-structure-prologue-not-numbered-line = { $count ->
    [one] 1 prolog eller epilog holdes uden for kapitelnummereringen.
   *[other] { $count } prologer eller epiloger holdes uden for kapitelnummereringen.
}
import-scrivener-structure-matter-inside-book-line = { $count ->
    [one] 1 element med for- eller efterstof uden for manuskriptet placeres inde i bogen som paratekst og udelades af eksporter, indtil du medtager det.
   *[other] { $count } elementer med for- eller efterstof uden for manuskriptet placeres inde i bogen som paratekst og udelades af eksporter, indtil du medtager dem.
}
import-scrivener-structure-paratext-not-counted-line = { $documents ->
    [one] 1 manuskriptdokument bliver til paratekst (for- eller efterstof), så { $words ->
        [one] { $words } ord tæller
       *[other] { $words } ord tæller
    } ikke længere som manuskript.
   *[other] { $documents } manuskriptdokumenter bliver til paratekst (for- eller efterstof), så { $words ->
        [one] { $words } ord tæller
       *[other] { $words } ord tæller
    } ikke længere som manuskript.
}
import-scrivener-structure-document-notes-line = { $count ->
    [one] Noterne til 1 dokument bliver til en note ved siden af det.
   *[other] Noterne til { $count } dokumenter bliver til noter ved siden af dem.
}
import-scrivener-structure-draft-content-as-note-line = Manuskriptmappens egen tekst bliver til en note øverst i manuskriptet.
import-scrivener-structure-story-bible-folder-line = { $count ->
    [one] 1 person- eller stedmappe bliver til et historiebibel-mærke.
   *[other] { $count } person- eller stedmapper bliver til historiebibel-mærker.
}
import-scrivener-structure-templates-created-line = { $count ->
    [one] 1 dokumentskabelon bliver til en noteskabelon.
   *[other] { $count } dokumentskabeloner bliver til noteskabeloner.
}
import-scrivener-structure-template-nesting-flattened-line = { $count ->
    [one] 1 skabelonmappe inde i en anden gøres flad: hver tekst i den bliver til en skabelon.
   *[other] { $count } skabelonmapper inde i andre gøres flade: hver tekst i dem bliver til en skabelon.
}
import-scrivener-structure-template-kept-as-note-line = { $count ->
    [one] 1 skabelon indeholder noget, en noteskabelon ikke kan rumme, og bevares som en note.
   *[other] { $count } skabeloner indeholder noget, en noteskabelon ikke kan rumme, og bevares som noter.
}
import-scrivener-structure-recovered-texts-line = { $count ->
    [one] 1 tekst, som intet element i ringbindet angav, overføres i sin egen mappe.
   *[other] { $count } tekster, som intet element i ringbindet angav, overføres i deres egen mappe.
}
import-scrivener-structure-project-notes-line = { $count ->
    [one] 1 projektnote overføres i sin egen mappe.
   *[other] { $count } projektnoter overføres i deres egen mappe.
}
import-scrivener-structure-trash-brought-across-line = { $items ->
    [1] Papirkurven overføres: 1 element, som du kan gendanne fra papirkurven.
   *[other] Papirkurven overføres: { $items } elementer, { $writing ->
        [0] heraf ingen med tekst
       *[other] heraf { $writing } med tekst
    }, som du kan gendanne fra papirkurven.
}
import-scrivener-structure-not-compiled-line = { $count ->
    [one] 1 element, som Scrivener udelod af sin kompilering, udelades også af eksporter.
   *[other] { $count } elementer, som Scrivener udelod af sin kompilering, udelades også af eksporter.
}
import-scrivener-structure-media-in-draft-line = { $count ->
    [one] 1 billede eller en fil i manuskriptet bliver til en note, da en bog ikke kan rumme den slags.
   *[other] { $count } billeder eller filer i manuskriptet bliver til noter, da en bog ikke kan rumme dem.
}
import-scrivener-structure-left-out-line = { $count ->
    [one] 1 mappe på øverste niveau udelades sammen med { $items ->
        [one] 1 element
       *[other] { $items } elementer
    }.
   *[other] { $count } mapper på øverste niveau udelades sammen med { $items ->
        [one] 1 element
       *[other] { $items } elementer
    }.
}
import-scrivener-structure-trash-left-out-line = { $items ->
    [1] Papirkurven efterlades: 1 element.
   *[other] Papirkurven efterlades: { $items } elementer, { $writing ->
        [0] heraf ingen med tekst
       *[other] heraf { $writing } med tekst
    }.
}
import-scrivener-structure-card-image-left-behind-line = { $count ->
    [one] 1 billede på et kartotekskort til et manuskriptelement efterlades: det ville blive trykt i bogen.
   *[other] { $count } billeder på kartotekskort til manuskriptelementer efterlades: de ville blive trykt i bogen.
}
import-scrivener-structure-default-template-not-linked-line = { $count ->
    [one] 1 mappe havde en standardskabelon til de nye dokumenter i den. I Skribisto kan kun et historiebibel-mærke have en skabelon, så den forbindelse bevares ikke.
   *[other] { $count } mapper havde en standardskabelon til de nye dokumenter i dem. I Skribisto kan kun et historiebibel-mærke have en skabelon, så de forbindelser bevares ikke.
}
import-scrivener-structure-page-break-not-kept-line = { $count ->
    [one] 1 indstilling »Page Break Before« (sideskift før) har ingen plads i Skribisto og bevares ikke.
   *[other] { $count } indstillinger »Page Break Before« (sideskift før) har ingen plads i Skribisto og bevares ikke.
}
import-scrivener-structure-compile-as-is-not-kept-line = { $count ->
    [one] 1 indstilling »Compile As-Is« (kompilér uændret) har intet modstykke i Skribisto og bevares ikke.
   *[other] { $count } indstillinger »Compile As-Is« (kompilér uændret) har intet modstykke i Skribisto og bevares ikke.
}

## What the import did with the labels, the status list and the dates
import-scrivener-metadata-status-categories-guessed-line = { $count ->
    [one] 1 status, som Skribisto ikke genkender, får en kategori ud fra sin plads på listen. Kontrollér den under Indstillinger ▸ Projekt ▸ Statusser.
   *[other] { $count } statusser, som Skribisto ikke genkender, får en kategori ud fra deres plads på listen. Kontrollér dem under Indstillinger ▸ Projekt ▸ Statusser.
}
import-scrivener-metadata-dates-unreadable-line = { $count ->
    [one] Datoerne for 1 element kunne ikke læses: det får importens dato.
   *[other] Datoerne for { $count } elementer kunne ikke læses: de får importens dato.
}
import-scrivener-metadata-labels-became-tags-line = { $tags ->
    [one] Etiketterne bliver til 1 mærke, anvendt på { $rows ->
        [one] { $rows } element
       *[other] { $rows } elementer
    }.
   *[other] Etiketterne bliver til { $tags } mærker, anvendt på { $rows ->
        [one] { $rows } element
       *[other] { $rows } elementer
    }.
}
import-scrivener-metadata-statuses-became-ladder-line = { $rungs ->
    [one] Statuslisten overføres som projektets liste over statusser: { $rungs } status, anvendt på { $rows ->
        [one] { $rows } element
       *[other] { $rows } elementer
    }.
   *[other] Statuslisten overføres som projektets liste over statusser: { $rungs } statusser, anvendt på { $rows ->
        [one] { $rows } element
       *[other] { $rows } elementer
    }.
}
import-scrivener-metadata-ladder-seeded-line = { $count ->
    [1] Projektet havde ingen statusliste, så det starter med Skribistos standardstatus.
   *[other] Projektet havde ingen statusliste, så det starter med Skribistos { $count } standardstatusser.
}
import-scrivener-metadata-label-renamed-line = Feltet Etiket hed »{ $title }« i Scrivener. Det navn står i hvert mærkes detaljer.
import-scrivener-metadata-status-renamed-line = Feltet Status hed »{ $title }« i Scrivener. Det navn står i detaljerne for hver status.
import-scrivener-metadata-none-label-collapsed-line = { $count ->
    [one] Etiketten »{ $name }«, som findes under samme navn ved siden af Scriveners egen tomme etiket, læses som ingen etiket på 1 element.
   *[other] Etiketten »{ $name }«, som findes under samme navn ved siden af Scriveners egen tomme etiket, læses som ingen etiket på { $count } elementer.
}
import-scrivener-metadata-none-status-collapsed-line = { $count ->
    [one] Statussen »{ $name }«, som findes under samme navn ved siden af Scriveners egen tomme status, læses som ingen status på 1 element.
   *[other] Statussen »{ $name }«, som findes under samme navn ved siden af Scriveners egen tomme status, læses som ingen status på { $count } elementer.
}
import-scrivener-metadata-unnamed-label-line = En etiket uden navn opkaldes efter sit nummer, { $id }.
import-scrivener-metadata-unnamed-status-line = En status uden navn opkaldes efter sit nummer, { $id }.
import-scrivener-metadata-tag-name-disambiguated-line = { $count ->
    [one] 1 mærke får et nyt navn, fordi et andet mærke allerede har dets navn.
   *[other] { $count } mærker får et nyt navn, fordi andre mærker allerede har deres navne.
}
import-scrivener-metadata-rung-names-disambiguated-line = { $count ->
    [one] 1 status har samme navn som en anden, så den får et nummer efter sit navn. Kontrollér den under Indstillinger ▸ Projekt ▸ Statusser.
   *[other] { $count } statusser har samme navn som en anden, så hver af dem får et nummer efter sit navn. Kontrollér dem under Indstillinger ▸ Projekt ▸ Statusser.
}
import-scrivener-metadata-unused-labels-skipped-line = { $count ->
    [one] 1 etiket, som intet dokument bærer, importeres ikke.
   *[other] { $count } etiketter, som intet dokument bærer, importeres ikke.
}
import-scrivener-metadata-labels-without-row-line = { $count ->
    [one] Etiketten på 1 element uden plads i det nye projekt overføres ikke.
   *[other] Etiketterne på { $count } elementer uden plads i det nye projekt overføres ikke.
}
import-scrivener-metadata-statuses-without-row-line = { $count ->
    [one] Statussen på 1 element uden plads i det nye projekt overføres ikke.
   *[other] Statusserne på { $count } elementer uden plads i det nye projekt overføres ikke.
}

import-scrivener-metadata-unnamed-keyword-line = Et nøgleord uden navn opkaldes efter sit nummer, { $id }.
import-scrivener-metadata-keywords-without-row-line = { $count ->
    [one] Nøgleordene på 1 element uden plads i det nye projekt overføres ikke.
   *[other] Nøgleordene på { $count } elementer uden plads i det nye projekt overføres ikke.
}
import-scrivener-metadata-custom-values-without-row-line = { $count ->
    [one] De brugerdefinerede metadata på 1 element uden plads i det nye projekt overføres ikke.
   *[other] De brugerdefinerede metadata på { $count } elementer uden plads i det nye projekt overføres ikke.
}
import-scrivener-metadata-bookmarks-without-row-line = { $count ->
    [one] Bogmærkerne på 1 element uden plads i det nye projekt overføres ikke.
   *[other] Bogmærkerne på { $count } elementer uden plads i det nye projekt overføres ikke.
}
import-scrivener-metadata-targets-without-row-line = { $count ->
    [one] Målet for 1 element uden plads i det nye projekt overføres ikke.
   *[other] Målene for { $count } elementer uden plads i det nye projekt overføres ikke.
}

## What the import does with the keywords, custom metadata, collections, bookmarks, targets,
## the deadline, the auto-complete list and the icons. Each line is said twice: on the Review
## page before the import, then in its report, so it reads the same at both times. `$note`
## and `$folder` are the titles the import gives a note and its "From Scrivener" folder;
## `$day` is a day as YYYY-MM-DD.
import-scrivener-metadata-pov-hint-line = { $count ->
    [1] Etiketten eller feltværdien { $values } matcher opslaget { $matches } i historiebiblen. Angiver den synsvinklen, så sæt den i Inspektøren: importen linker intet.
   *[other] Etiketterne eller feltværdierne { $values } matcher opslagene { $matches } i historiebiblen. Angiver de synsvinkler, så sæt dem i Inspektøren: importen linker intet.
}
import-scrivener-metadata-checkbox-default-unknown-line = Afkrydsningsfeltet »{ $field }« har en standardværdi, Skribisto ikke kan læse, så programmet kan ikke afgøre, hvilke dokumenter der er markeret som standard. Kun de dokumenter, der er markeret enkeltvis, får feltets mærke.
import-scrivener-metadata-goals-in-other-unit-line = { $count ->
    [one] 1 mål er sat i den anden tælleenhed. Det bevares og vises, når du skifter tælleenhed under Indstillinger ▸ Projekt ▸ Struktur.
   *[other] { $count } mål er sat i den anden tælleenhed. De bevares og vises, når du skifter tælleenhed under Indstillinger ▸ Projekt ▸ Struktur.
}
import-scrivener-metadata-counting-differs-line = Scrivener talte udkastmålet ud fra andre dokumenter, end Skribisto gør. Bogens mål tæller de scener, der medtages i eksporter.
import-scrivener-metadata-writing-days-not-read-line = Skrivedagene »{ $raw }« kan ikke læses, så bogens Tempo tæller hver dag som en skrivedag. Ret dem der.
import-scrivener-metadata-script-mode-rows-line = { $count ->
    [one] 1 element var i drejebogstilstand i Scrivener. Skribisto beholder ordene som almindelig tekst uden drejebogsopsætningen.
   *[other] { $count } elementer var i drejebogstilstand i Scrivener. Skribisto beholder ordene som almindelig tekst uden drejebogsopsætningen.
}
import-scrivener-metadata-keywords-became-tags-line = { $tags ->
    [one] 1 nøgleord bliver til et mærke, anvendt på { $rows ->
        [one] { $rows } element
       *[other] { $rows } elementer
    }.
   *[other] { $tags } nøgleord bliver til mærker, anvendt på { $rows ->
        [one] { $rows } element
       *[other] { $rows } elementer
    }.
}
import-scrivener-metadata-keyword-hierarchy-flattened-line = { $count ->
    [one] 1 nøgleord lå inde i et andet i Scrivener. Mærker er flade: mærkets detaljer fortæller, hvor det lå.
   *[other] { $count } nøgleord lå inde i andre i Scrivener. Mærker er flade: hvert mærkes detaljer fortæller, hvor dets nøgleord lå.
}
import-scrivener-metadata-fields-became-tags-line = { $fields ->
    [one] 1 liste- eller afkrydsningsfelt bliver til { $tags ->
        [one] { $tags } mærke
       *[other] { $tags } mærker
    }, anvendt på { $rows ->
        [one] { $rows } element
       *[other] { $rows } elementer
    }.
   *[other] { $fields } liste- og afkrydsningsfelter bliver til { $tags ->
        [one] { $tags } mærke
       *[other] { $tags } mærker
    }, anvendt på { $rows ->
        [one] { $rows } element
       *[other] { $rows } elementer
    }.
}
import-scrivener-metadata-collections-became-tags-line = { $tags ->
    [one] 1 samling bliver til et mærke, anvendt på { $rows ->
        [one] { $rows } element
       *[other] { $rows } elementer
    }.
   *[other] { $tags } samlinger bliver til mærker, anvendt på { $rows ->
        [one] { $rows } element
       *[other] { $rows } elementer
    }.
}
import-scrivener-metadata-collection-order-lost-line = { $count ->
    [one] Et mærke har ingen rækkefølge, så rækkefølgen af dokumenterne i 1 samling bevares ikke.
   *[other] Et mærke har ingen rækkefølge, så rækkefølgen af dokumenterne i { $count } samlinger bevares ikke.
}
import-scrivener-metadata-bookmarks-became-links-line = { $count ->
    [one] 1 bogmærke bliver til et link mellem to noter eller mellem en scene og et opslag i historiebiblen: se Links og Persongalleri.
   *[other] { $count } bogmærker bliver til links mellem noter eller mellem scener og opslag i historiebiblen: se Links og Persongalleri.
}
import-scrivener-metadata-goals-kept-line = { $count ->
    [one] Det mål, Scrivener havde sat for 1 dokument, bliver dets mål i Skribisto.
   *[other] De mål, Scrivener havde sat for { $count } dokumenter, bliver deres mål i Skribisto.
}
import-scrivener-metadata-draft-target-kept-words-line = { $value ->
    [one] Udkastmålet på 1 ord bliver til bogens mål.
   *[other] Udkastmålet på { $value } ord bliver til bogens mål.
}
import-scrivener-metadata-draft-target-kept-characters-line = { $value ->
    [one] Udkastmålet på 1 tegn bliver til bogens mål.
   *[other] Udkastmålet på { $value } tegn bliver til bogens mål.
}
import-scrivener-metadata-pace-planned-line = Deadlinen, { $day }, opretter bogens Tempo, som fra importdagen planlægger ordene for hver af de skrivedage, du valgte i Scrivener.
import-scrivener-metadata-pace-planned-day-off-line = Deadlinen, { $day }, opretter bogens Tempo, som fra importdagen planlægger ordene for hver af de skrivedage, du valgte i Scrivener. Som i Scrivener er selve deadlinedagen en fridag.
import-scrivener-metadata-dictionary-words-line = { $count ->
    [one] 1 ord fra autofuldførelseslisten kommer i projektets personlige ordbog, så det ikke markeres som stavefejl.
   *[other] { $count } ord fra autofuldførelseslisten kommer i projektets personlige ordbog, så de ikke markeres som stavefejl.
}
import-scrivener-metadata-listed-project-line = { $count ->
    [one] 1 oplysning om projektet er angivet i noten »{ $note }« i mappen »{ $folder }«.
   *[other] { $count } oplysninger om projektet er angivet i noten »{ $note }« i mappen »{ $folder }«.
}
import-scrivener-metadata-listed-project-bookmarks-line = { $count ->
    [one] 1 projektbogmærke er angivet i noten »{ $note }« i mappen »{ $folder }«.
   *[other] { $count } projektbogmærker er angivet i noten »{ $note }« i mappen »{ $folder }«.
}
import-scrivener-metadata-listed-replacements-line = { $count ->
    [one] 1 kompileringserstatning er angivet i noten »{ $note }« i mappen »{ $folder }«. Skribisto foretager den ikke.
   *[other] { $count } kompileringserstatninger er angivet i noten »{ $note }« i mappen »{ $folder }«. Skribisto foretager dem ikke.
}
import-scrivener-metadata-listed-auto-complete-line = { $count ->
    [one] 1 post til autofuldførelse på flere ord er angivet i noten »{ $note }« i mappen »{ $folder }«.
   *[other] { $count } poster til autofuldførelse på flere ord er angivet i noten »{ $note }« i mappen »{ $folder }«.
}
import-scrivener-metadata-listed-saved-searches-line = { $count ->
    [one] 1 gemt søgning er angivet i noten »{ $note }« i mappen »{ $folder }«.
   *[other] { $count } gemte søgninger er angivet i noten »{ $note }« i mappen »{ $folder }«.
}
import-scrivener-metadata-listed-custom-metadata-line = { $count ->
    [one] 1 brugerdefineret metadataværdi, som et mærke ikke kan rumme, er angivet i noten »{ $note }« i mappen »{ $folder }«.
   *[other] { $count } brugerdefinerede metadataværdier, som et mærke ikke kan rumme, er angivet i noten »{ $note }« i mappen »{ $folder }«.
}
import-scrivener-metadata-listed-targets-line = { $count ->
    [one] 1 dokumentmål er angivet i noten »{ $note }« i mappen »{ $folder }«.
   *[other] { $count } dokumentmål er angivet i noten »{ $note }« i mappen »{ $folder }«.
}
import-scrivener-metadata-listed-bookmarks-line = { $count ->
    [one] 1 bogmærke, som intet link i Skribisto viser, er angivet i noten »{ $note }« i mappen »{ $folder }«.
   *[other] { $count } bogmærker, som intet link i Skribisto viser, er angivet i noten »{ $note }« i mappen »{ $folder }«.
}
import-scrivener-metadata-targets-on-notes-line = { $count ->
    [one] 1 dokumentmål sidder på en note, hvis tekst intet mål tæller, så det bliver ikke til et mål.
   *[other] { $count } dokumentmål sidder på noter, hvis tekst intet mål tæller, så de bliver ikke til mål.
}
import-scrivener-metadata-targets-on-split-documents-line = { $count ->
    [one] 1 dokumentmål sidder på et dokument, der er opdelt ved sine overskrifter, og hvis kapitler intet enkelt mål tæller sammen, så det bliver ikke til et mål.
   *[other] { $count } dokumentmål sidder på dokumenter, der er opdelt ved deres overskrifter, og hvis kapitler intet enkelt mål tæller sammen, så de bliver ikke til mål.
}
import-scrivener-metadata-bookmark-ends-without-row-line = { $count ->
    [one] 1 bogmærke fører til et dokument uden plads i det nye projekt, f.eks. en dokumentskabelon. Det er angivet ud fra, hvor dokumentet lå i Scrivener.
   *[other] { $count } bogmærker fører til dokumenter uden plads i det nye projekt, f.eks. dokumentskabeloner. De er angivet ud fra, hvor dokumenterne lå i Scrivener.
}
import-scrivener-metadata-unused-keywords-skipped-line = { $count ->
    [one] 1 nøgleord, som intet dokument bærer, importeres ikke.
   *[other] { $count } nøgleord, som intet dokument bærer, importeres ikke.
}
import-scrivener-metadata-list-value-invalid-line = { $count ->
    [one] 1 værdi i listefeltet »{ $field }« svarer ikke til nogen af feltets valgmuligheder og importeres ikke.
   *[other] { $count } værdier i listefeltet »{ $field }« svarer ikke til nogen af feltets valgmuligheder og importeres ikke.
}
import-scrivener-metadata-undefined-custom-field-line = { $count ->
    [one] 1 værdi hører til et brugerdefineret felt, som projektet ikke længere definerer, og som Scrivener heller ikke viser. Den importeres ikke.
   *[other] { $count } værdier hører til brugerdefinerede felter, som projektet ikke længere definerer, og som Scrivener heller ikke viser. De importeres ikke.
}
import-scrivener-metadata-collection-members-missing-line = { $count ->
    [one] 1 dokument i samlingen »{ $name }« har ingen plads i det nye projekt eller er forsvundet fra Scrivener-projektet.
   *[other] { $count } dokumenter i samlingen »{ $name }« har ingen plads i det nye projekt eller er forsvundet fra Scrivener-projektet.
}
import-scrivener-metadata-empty-collections-skipped-line = { $count ->
    [one] 1 samling, der ikke indeholder noget dokument, importeres ikke.
   *[other] { $count } samlinger, der ikke indeholder noget dokument, importeres ikke.
}
import-scrivener-metadata-bookmarks-unresolved-line = { $count ->
    [one] 1 bogmærke fører til et dokument, som Scrivener-projektet ikke længere indeholder, og importeres ikke.
   *[other] { $count } bogmærker fører til dokumenter, som Scrivener-projektet ikke længere indeholder, og importeres ikke.
}
import-scrivener-metadata-targets-in-pages-line = { $count ->
    [one] 1 dokumentmål tæller sider, som Skribisto ikke tæller, så det bliver ikke til et mål.
   *[other] { $count } dokumentmål tæller sider, som Skribisto ikke tæller, så de bliver ikke til mål.
}
import-scrivener-metadata-targets-unreadable-line = { $count ->
    [one] 1 mål importeres ikke: dets antal er ikke et tal, er under nul eller er større, end et mål kan rumme.
   *[other] { $count } mål importeres ikke: deres antal er ikke tal, er under nul eller er større, end et mål kan rumme.
}
import-scrivener-metadata-draft-target-in-pages-line = { $value ->
    [one] Udkastmålet på 1 side bliver ikke til et mål: Skribisto tæller ord eller tegn. Det er angivet i noten »{ $note }« i mappen »{ $folder }«.
   *[other] Udkastmålet på { $value } sider bliver ikke til et mål: Skribisto tæller ord eller tegn. Det er angivet i noten »{ $note }« i mappen »{ $folder }«.
}
import-scrivener-metadata-draft-target-no-book-words-line = { $value ->
    [one] Udkastmålet på 1 ord bliver ikke til et mål, fordi manuskriptet udgør flere bøger eller ingen. Det er angivet i noten »{ $note }« i mappen »{ $folder }«.
   *[other] Udkastmålet på { $value } ord bliver ikke til et mål, fordi manuskriptet udgør flere bøger eller ingen. Det er angivet i noten »{ $note }« i mappen »{ $folder }«.
}
import-scrivener-metadata-draft-target-no-book-characters-line = { $value ->
    [one] Udkastmålet på 1 tegn bliver ikke til et mål, fordi manuskriptet udgør flere bøger eller ingen. Det er angivet i noten »{ $note }« i mappen »{ $folder }«.
   *[other] Udkastmålet på { $value } tegn bliver ikke til et mål, fordi manuskriptet udgør flere bøger eller ingen. Det er angivet i noten »{ $note }« i mappen »{ $folder }«.
}
import-scrivener-metadata-deadline-passed-line = Deadlinen, { $day }, er overskredet, så den opretter ikke bogens Tempo. Den er angivet i noten »{ $note }« i mappen »{ $folder }«.
import-scrivener-metadata-deadline-no-writing-day-line = Der er ingen skrivedag tilbage fra importdagen til deadlinen, { $day }, så den opretter ikke bogens Tempo. Den er angivet i noten »{ $note }« i mappen »{ $folder }«.
import-scrivener-metadata-deadline-template-line = Deadlinen, { $day }, blev gemt i skabelonen, så den opretter ikke bogens Tempo. Den er angivet i noten »{ $note }« i mappen »{ $folder }«.
import-scrivener-metadata-deadline-no-word-goal-line = Deadlinen, { $day }, opretter ikke bogens Tempo, fordi udkastet ikke har noget ordmål at planlægge efter. Den er angivet i noten »{ $note }« i mappen »{ $folder }«.
import-scrivener-metadata-deadline-character-goal-line = Deadlinen, { $day }, opretter ikke bogens Tempo, fordi udkastmålet tæller tegn, og Tempo tæller ord. Den er angivet i noten »{ $note }« i mappen »{ $folder }«.
import-scrivener-metadata-deadline-no-book-line = Deadlinen, { $day }, opretter ikke Tempo for nogen bog, fordi manuskriptet udgør flere bøger eller ingen. Den er angivet i noten »{ $note }« i mappen »{ $folder }«.
import-scrivener-metadata-deadline-unreadable-line = Deadlinen kan ikke læses, så den opretter ikke bogens Tempo. Den er angivet, som den er skrevet, i noten »{ $note }« i mappen »{ $folder }«.
import-scrivener-metadata-session-target-words-line = { $value ->
    [one] Sessionsmålet på 1 ord importeres ikke: i Skribisto er skrivesessionens mål én indstilling for alle projekter. Det er angivet i noten »{ $note }« i mappen »{ $folder }«.
   *[other] Sessionsmålet på { $value } ord importeres ikke: i Skribisto er skrivesessionens mål én indstilling for alle projekter. Det er angivet i noten »{ $note }« i mappen »{ $folder }«.
}
import-scrivener-metadata-session-target-characters-line = { $value ->
    [one] Sessionsmålet på 1 tegn importeres ikke: i Skribisto er skrivesessionens mål én indstilling for alle projekter, målt i ord. Det er angivet i noten »{ $note }« i mappen »{ $folder }«.
   *[other] Sessionsmålet på { $value } tegn importeres ikke: i Skribisto er skrivesessionens mål én indstilling for alle projekter, målt i ord. Det er angivet i noten »{ $note }« i mappen »{ $folder }«.
}
import-scrivener-metadata-session-target-pages-line = { $value ->
    [one] Sessionsmålet på 1 side importeres ikke: i Skribisto er skrivesessionens mål én indstilling for alle projekter, målt i ord. Det er angivet i noten »{ $note }« i mappen »{ $folder }«.
   *[other] Sessionsmålet på { $value } sider importeres ikke: i Skribisto er skrivesessionens mål én indstilling for alle projekter, målt i ord. Det er angivet i noten »{ $note }« i mappen »{ $folder }«.
}
import-scrivener-metadata-session-target-calculated-words-line = { $value ->
    [one] Sessionsmålet på 1 ord, som Scrivener beregnede ud fra deadlinen og skrivedagene, importeres ikke: i Skribisto er skrivesessionens mål én indstilling for alle projekter. Det er angivet i noten »{ $note }« i mappen »{ $folder }«.
   *[other] Sessionsmålet på { $value } ord, som Scrivener beregnede ud fra deadlinen og skrivedagene, importeres ikke: i Skribisto er skrivesessionens mål én indstilling for alle projekter. Det er angivet i noten »{ $note }« i mappen »{ $folder }«.
}
import-scrivener-metadata-session-target-calculated-characters-line = { $value ->
    [one] Sessionsmålet på 1 tegn, som Scrivener beregnede ud fra deadlinen og skrivedagene, importeres ikke: i Skribisto er skrivesessionens mål én indstilling for alle projekter, målt i ord. Det er angivet i noten »{ $note }« i mappen »{ $folder }«.
   *[other] Sessionsmålet på { $value } tegn, som Scrivener beregnede ud fra deadlinen og skrivedagene, importeres ikke: i Skribisto er skrivesessionens mål én indstilling for alle projekter, målt i ord. Det er angivet i noten »{ $note }« i mappen »{ $folder }«.
}
import-scrivener-metadata-session-target-calculated-pages-line = { $value ->
    [one] Sessionsmålet på 1 side, som Scrivener beregnede ud fra deadlinen og skrivedagene, importeres ikke: i Skribisto er skrivesessionens mål én indstilling for alle projekter, målt i ord. Det er angivet i noten »{ $note }« i mappen »{ $folder }«.
   *[other] Sessionsmålet på { $value } sider, som Scrivener beregnede ud fra deadlinen og skrivedagene, importeres ikke: i Skribisto er skrivesessionens mål én indstilling for alle projekter, målt i ord. Det er angivet i noten »{ $note }« i mappen »{ $folder }«.
}
import-scrivener-metadata-icons-dropped-line = { $count ->
    [one] 1 element havde et ikon i ringbindet, som ikke overføres: Skribisto viser hvert elements ikon ud fra dets type.
   *[other] { $count } elementer havde ikoner i ringbindet, som ikke overføres: Skribisto viser hvert elements ikon ud fra dets type.
}

## What converting the prose did, as the import's report says it. `$count` is how many texts,
## or how many of a thing, as each line says; `$example` quotes the project.
import-scrivener-prose-unreadable-line = { $count ->
    [one] 1 tekst kunne ikke læses. Den kommer tom ind.
   *[other] { $count } tekster kunne ikke læses. De kommer tomme ind.
}
import-scrivener-prose-comments-unreadable-line = { $count ->
    [one] Kommentarerne og de linkede fodnoter i 1 tekst kunne ikke læses. De ord, de markerede, bevares.
   *[other] Kommentarerne og de linkede fodnoter i { $count } tekster kunne ikke læses. De ord, de markerede, bevares.
}
import-scrivener-prose-read-as-plain-text-line = { $count ->
    [one] 1 tekst var gemt som ren tekst i stedet for formateret tekst og læses som sådan.
   *[other] { $count } tekster var gemt som ren tekst i stedet for formateret tekst og læses som sådan.
}
import-scrivener-prose-inline-note-unmatched-line = { $count ->
    [one] 1 markør for en indlejret note danner ikke par med en anden og bevares som tekst.
   *[other] { $count } markører for indlejrede noter danner ikke par med en anden og bevares som tekst.
}
import-scrivener-prose-style-unresolved-line = { $count ->
    [one] 1 typografimarkør nævner en typografi, projektet ikke definerer. Den fjernes, og teksten beholder sin formatering.
   *[other] { $count } typografimarkører nævner typografier, projektet ikke definerer. De fjernes, og teksten beholder sin formatering.
}
import-scrivener-prose-styles-lost-in-export-line = { $count ->
    [one] 1 afsnit er måske en overskrift, der mistede sin typografi, da Scrivener 3 eksporterede projektet til sit ældre format, og det overføres som et almindeligt afsnit. Kontrollér det, eller importér det projekt, det blev eksporteret fra.
   *[other] { $count } afsnit er måske overskrifter, der mistede deres typografi, da Scrivener 3 eksporterede projektet til sit ældre format, og de overføres som almindelige afsnit. Kontrollér dem, eller importér det projekt, de blev eksporteret fra.
}
import-scrivener-prose-comment-anchor-without-entry-line = { $count ->
    [one] 1 kommentar eller fodnote havde mistet sit indhold i Scrivener. De ord, den markerede, bevares uden den.
   *[other] { $count } kommentarer eller fodnoter havde mistet deres indhold i Scrivener. De ord, de markerede, bevares uden dem.
}
import-scrivener-prose-comment-entry-without-anchor-line = { $count ->
    [one] 1 kommentar havde ikke længere nogen plads i sin tekst og knyttes til det første afsnit.
   *[other] { $count } kommentarer havde ikke længere nogen plads i deres tekst og knyttes til det første afsnit.
}
import-scrivener-prose-footnote-entry-without-anchor-line = { $count ->
    [one] 1 linket fodnote havde ikke længere nogen plads i sin tekst, så den får sin henvisning i slutningen af den tekst.
   *[other] { $count } linkede fodnoter havde ikke længere nogen plads i deres tekster, så hver af dem får sin henvisning i slutningen af sin egen tekst.
}
import-scrivener-prose-range-comment-reading-line = { $count ->
    [one] 1 kommentar fra en ældre version af Scrivener blev placeret ud fra en anden tolkning af dens position. Kontrollér, at den sidder på de rigtige ord.
   *[other] { $count } kommentarer fra en ældre version af Scrivener blev placeret ud fra en anden tolkning af deres position. Kontrollér, at de sidder på de rigtige ord.
}
import-scrivener-prose-range-comment-reading-markers-counted-line = { $count ->
    [one] 1 kommentar fra en ældre version af Scrivener blev placeret ved at tælle markørerne for dens indlejrede noter med. Kontrollér, at den sidder på de rigtige ord.
   *[other] { $count } kommentarer fra en ældre version af Scrivener blev placeret ved at tælle markørerne for deres indlejrede noter med. Kontrollér, at de sidder på de rigtige ord.
}
import-scrivener-prose-range-comment-reading-pictures-not-counted-line = { $count ->
    [one] 1 kommentar fra en ældre version af Scrivener blev placeret ved at se bort fra dens billeder. Kontrollér, at den sidder på de rigtige ord.
   *[other] { $count } kommentarer fra en ældre version af Scrivener blev placeret ved at se bort fra deres billeder. Kontrollér, at de sidder på de rigtige ord.
}
import-scrivener-prose-range-comment-approximate-line = { $count ->
    [one] 1 kommentar fra en ældre version af Scrivener passer ikke præcist med nogen tolkning af dens position og sidder måske et par tegn forkert.
   *[other] { $count } kommentarer fra en ældre version af Scrivener passer ikke præcist med nogen tolkning af deres position og sidder måske et par tegn forkert.
}
import-scrivener-prose-document-link-unresolved-line = { $count ->
    [one] 1 link peger på et dokument, importen ikke overførte. Linkets tekst bliver stående.
   *[other] { $count } links peger på dokumenter, importen ikke overførte. Linkenes tekst bliver stående.
}
import-scrivener-prose-linked-picture-not-read-line = { $count ->
    [1] 1 billede, der var linket til en fil på din disk, »{ $example }«, blev ikke overført.
   *[other] { $count } billeder, der var linket til filer på din disk, f.eks. »{ $example }«, blev ikke overført.
}
import-scrivener-prose-heading-repeats-title-line = { $count ->
    [one] 1 tekst begynder med en overskrift, der gentager dens titel, så en eksport udskriver måske titlen to gange.
   *[other] { $count } tekster begynder med en overskrift, der gentager deres titel, så en eksport udskriver måske titlen to gange.
}
import-scrivener-prose-markdown-as-text-line = { $count ->
    [one] 1 afsnit skrevet i Markdown bevares som den tekst, det er.
   *[other] { $count } afsnit skrevet i Markdown bevares som den tekst, de er.
}
import-scrivener-prose-citation-keys-as-text-line = { $count ->
    [one] 1 afsnit indeholder citatnøgler til et litteraturhåndteringsprogram og bevares som tekst.
   *[other] { $count } afsnit indeholder citatnøgler til et litteraturhåndteringsprogram og bevares som tekst.
}
import-scrivener-prose-placeholders-in-exported-text-line = { $count ->
    [1] 1 Scrivener-pladsholder, { $example }, bevares som ren tekst i det, du eksporterer. Erstat den manuelt.
   *[other] { $count } Scrivener-pladsholdere, f.eks. { $example }, bevares som ren tekst i det, du eksporterer. Erstat dem manuelt.
}
import-scrivener-prose-placeholders-in-other-text-line = { $count ->
    [1] 1 Scrivener-pladsholder, { $example }, bevares som ren tekst i en tekst, der ikke eksporteres.
   *[other] { $count } Scrivener-pladsholdere, f.eks. { $example }, bevares som ren tekst i tekster, der ikke eksporteres.
}
import-scrivener-prose-ornament-lines-kept-line = { $count ->
    [one] 1 centreret linje med ornamenter markerer måske et sceneskift og bevares, som den er skrevet.
   *[other] { $count } centrerede linjer med ornamenter markerer måske sceneskift og bevares, som de er skrevet.
}
import-scrivener-prose-long-document-line = { $count ->
    [one] 1 tekst er meget lang og kan være langsom at åbne.
   *[other] { $count } tekster er meget lange og kan være langsomme at åbne.
}
import-scrivener-prose-pictures-without-description-line = { $count ->
    [one] 1 billede har ingen beskrivelse til læsere, der ikke kan se det. Du kan tilføje en i editoren.
   *[other] { $count } billeder har ingen beskrivelse til læsere, der ikke kan se dem. Du kan tilføje en i editoren.
}
import-scrivener-prose-struck-text-compiled-away-line = { $count ->
    [one] Gennemstreget tekst i 1 tekst bevares. Scriveners kompilering udelod den af bogen, men en eksport fra Skribisto udskriver den.
   *[other] Gennemstreget tekst i { $count } tekster bevares. Scriveners kompilering udelod den af bogen, men en eksport fra Skribisto udskriver den.
}
import-scrivener-prose-compile-deleted-style-printed-line = { $count ->
    [1] Tekst i typografien »{ $example }«, som Scriveners kompilering slettede, bevares som almindelig tekst, og en eksport fra Skribisto udskriver den.
   *[other] Tekst i { $count } typografier, som Scriveners kompilering slettede, f.eks. »{ $example }«, bevares som almindelig tekst, og en eksport fra Skribisto udskriver den.
}
import-scrivener-prose-compile-raw-style-printed-line = { $count ->
    [1] Tekst i typografien »{ $example }«, som Scriveners kompilering sendte videre som kode til et andet program, bevares som almindelig tekst, og en eksport fra Skribisto udskriver den.
   *[other] Tekst i { $count } typografier, som Scriveners kompilering sendte videre som kode til et andet program, f.eks. »{ $example }«, bevares som almindelig tekst, og en eksport fra Skribisto udskriver den.
}
import-scrivener-prose-left-out-after-document-line = { $count ->
    [one] 1 stykke efter slutningen af en beskadiget fil udelades.
   *[other] { $count } stykker efter slutningen af beskadigede filer udelades.
}
import-scrivener-prose-left-out-header-or-footer-line = { $count ->
    [one] 1 sidehoved eller sidefod udelades: det er ikke en del af teksten.
   *[other] { $count } sidehoveder og sidefødder udelades: de er ikke en del af teksten.
}
import-scrivener-prose-left-out-list-label-picture-line = { $count ->
    [one] 1 billede, der bruges som punkttegn i en liste, udelades.
   *[other] { $count } billeder, der bruges som punkttegn i lister, udelades.
}
import-scrivener-prose-left-out-line = { $count ->
    [one] 1 stykke af en fil, som ikke hører til nogen tekst, udelades.
   *[other] { $count } stykker af filer, som ikke hører til nogen tekst, udelades.
}
import-scrivener-prose-undecodable-characters-line = { $count ->
    [one] 1 tegn kunne ikke afkodes og vises som et erstatningstegn (�), hvor det stod.
   *[other] { $count } tegn kunne ikke afkodes og vises som erstatningstegn (�), hvor de stod.
}
import-scrivener-prose-inline-notes-merged-line = { $count ->
    [one] 1 indlejret note, som Scrivener havde delt, hvor dens formatering skiftede, sættes sammen igen.
   *[other] { $count } indlejrede noter, som Scrivener havde delt, hvor deres formatering skiftede, sættes sammen igen.
}
import-scrivener-prose-inline-notes-joined-across-paragraphs-line = { $count ->
    [1] 1 indlejret note, der var skrevet ét afsnit ad gangen, samles til én note.
   *[other] { $count } indlejrede noter, der var skrevet ét afsnit ad gangen, samles til noter på flere afsnit.
}
import-scrivener-prose-blank-notes-dropped-line = { $count ->
    [one] 1 tom indlejret note udelades.
   *[other] { $count } tomme indlejrede noter udelades.
}
import-scrivener-prose-annotations-became-comments-line = { $count ->
    [one] 1 indlejret annotation blev til en kommentar.
   *[other] { $count } indlejrede annotationer blev til kommentarer.
}
import-scrivener-prose-annotation-only-document-kept-line = { $count ->
    [one] 1 dokument indeholdt kun en annotation. Ordene bevares som almindelig tekst.
   *[other] { $count } dokumenter indeholdt kun en annotation. Ordene bevares som almindelig tekst.
}
import-scrivener-prose-preserve-formatting-passages-line = { $count ->
    [one] 1 passage markeret med »Preserve Formatting« (bevar formatering) kommer ind som almindelig tekst.
   *[other] { $count } passager markeret med »Preserve Formatting« (bevar formatering) kommer ind som almindelig tekst.
}
import-scrivener-prose-nested-note-unwrapped-line = { $count ->
    [one] 1 note inde i en kommentar eller en fodnote bevares som tekst, hvor den stod.
   *[other] { $count } noter inde i kommentarer eller fodnoter bevares som tekst, hvor de stod.
}
import-scrivener-prose-style-not-carried-line = { $count ->
    [1] Typografien »{ $example }« har intet modstykke i Skribisto. Dens tekst beholder sine ord.
   *[other] { $count } typografier, f.eks. »{ $example }«, har intet modstykke i Skribisto. Deres tekst beholder sine ord.
}
import-scrivener-prose-document-links-dropped-line = { $count ->
    [one] 1 link til et andet dokument har ingen plads i Skribisto. Linkets tekst bliver stående.
   *[other] { $count } links til andre dokumenter har ingen plads i Skribisto. Linkenes tekst bliver stående.
}
import-scrivener-prose-document-links-kept-line = { $count ->
    [one] 1 link til et andet dokument bevares som en henvisning.
   *[other] { $count } links til andre dokumenter bevares som henvisninger.
}
import-scrivener-prose-other-links-kept-as-text-line = { $count ->
    [one] 1 link til et andet projekt eller til en fil bevares som tekst.
   *[other] { $count } links til andre projekter eller til filer bevares som tekst.
}
import-scrivener-prose-soft-breaks-split-line = { $count ->
    [one] 1 linjeskift inde i et afsnit starter nu sit eget afsnit.
   *[other] { $count } linjeskift inde i afsnit starter nu deres egne afsnit.
}
import-scrivener-prose-typed-indents-removed-line = { $count ->
    [one] 1 indrykning, der var tastet i starten af et afsnit, fjernes.
   *[other] { $count } indrykninger, der var tastet i starten af afsnit, fjernes.
}
import-scrivener-prose-left-indents-not-kept-line = { $count ->
    [one] 1 afsnit, der var indrykket fra margenen, mister sin indrykning.
   *[other] { $count } afsnit, der var indrykket fra margenen, mister deres indrykning.
}
import-scrivener-prose-blank-lines-removed-line = { $count ->
    [one] 1 tomt afsnit mellem to andre fjernes.
   *[other] { $count } tomme afsnit mellem andre fjernes.
}
import-scrivener-prose-verse-stanza-gaps-lost-line = { $count ->
    [one] 1 tom linje mellem et digts strofer fjernes, så stroferne løber sammen.
   *[other] { $count } tomme linjer mellem et digts strofer fjernes, så stroferne løber sammen.
}
import-scrivener-prose-blank-lines-became-scene-breaks-line = { $count ->
    [one] 1 tom linje mellem afsnit blev til et sceneskift.
   *[other] { $count } tomme linjer mellem afsnit blev til sceneskift.
}
import-scrivener-prose-ornaments-became-scene-breaks-line = { $count ->
    [one] 1 linje med ornamenter blev til et sceneskift.
   *[other] { $count } linjer med ornamenter blev til sceneskift.
}
import-scrivener-prose-table-first-row-header-line = { $count ->
    [one] 1 tabel begynder nu med en overskriftsrække.
   *[other] { $count } tabeller begynder nu med en overskriftsrække.
}
import-scrivener-prose-table-cells-joined-line = { $count ->
    [one] 1 tabelcelle indeholdt flere afsnit, som nu er samlet til ét.
   *[other] { $count } tabelceller indeholdt flere afsnit, som nu er samlet til ét.
}
import-scrivener-prose-formatting-lost-colour-line = { $count ->
    [one] Tekstfarve fjernes i 1 tekst.
   *[other] Tekstfarve fjernes i { $count } tekster.
}
import-scrivener-prose-formatting-lost-highlight-line = { $count ->
    [one] Fremhævning fjernes i 1 tekst.
   *[other] Fremhævning fjernes i { $count } tekster.
}
import-scrivener-prose-formatting-lost-small-caps-line = { $count ->
    [one] Kapitæler vises som almindelige bogstaver i 1 tekst.
   *[other] Kapitæler vises som almindelige bogstaver i { $count } tekster.
}
import-scrivener-prose-formatting-lost-all-caps-line = { $count ->
    [one] Tekst, der blev vist med store bogstaver, vises, som den blev skrevet, i 1 tekst.
   *[other] Tekst, der blev vist med store bogstaver, vises, som den blev skrevet, i { $count } tekster.
}
import-scrivener-prose-formatting-lost-hidden-line = { $count ->
    [one] Skjult tekst udelades i 1 tekst.
   *[other] Skjult tekst udelades i { $count } tekster.
}
import-scrivener-prose-formatting-lost-line = { $count ->
    [one] Formatering, som Skribisto ikke har plads til, fjernes i 1 tekst.
   *[other] Formatering, som Skribisto ikke har plads til, fjernes i { $count } tekster.
}
import-scrivener-prose-justified-text-not-kept-line = { $count ->
    [one] 1 blokjusteret afsnit følger i stedet eksporttypografien.
   *[other] { $count } blokjusterede afsnit følger i stedet eksporttypografien.
}
import-scrivener-prose-comment-colours-lost-line = { $count ->
    [one] 1 kommentar mister sin farve.
   *[other] { $count } kommentarer mister deres farve.
}
import-scrivener-prose-control-characters-removed-line = { $count ->
    [one] 1 usynligt kontroltegn fjernes.
   *[other] { $count } usynlige kontroltegn fjernes.
}
import-scrivener-prose-note-on-heading-lost-line = { $count ->
    [one] 1 fodnote eller kommentar på en kapiteloverskrift kunne ikke blive der: en kommentar flyttes til kapitlets første afsnit, en fodnote udelades.
   *[other] { $count } fodnoter eller kommentarer på kapiteloverskrifter kunne ikke blive der: en kommentar flyttes til kapitlets første afsnit, en fodnote udelades.
}
import-scrivener-prose-media-file-copied-line = { $count ->
    [one] 1 researchfil, som Skribisto ikke kan åbne, blev kopieret ved siden af det nye projekt. Dens note fortæller hvor.
   *[other] { $count } researchfiler, som Skribisto ikke kan åbne, blev kopieret ved siden af det nye projekt. Deres noter fortæller hvor.
}
import-scrivener-prose-media-file-not-carried-line = { $count ->
    [one] 1 researchfil, som Skribisto ikke kan åbne, bliver i dit Scrivener-projekt. Dens note nævner den.
   *[other] { $count } researchfiler, som Skribisto ikke kan åbne, bliver i dit Scrivener-projekt. Deres noter nævner dem.
}

## When the import fails
import-scrivener-error-refused = { $refusal } Intet blev skrevet.
import-scrivener-error-question = Projektet stiller nu et spørgsmål, som gennemgangen ikke besvarede. Vælg »Gennemgå importen igen«, og importér så. Intet blev skrevet.
import-scrivener-error-not-accepted = Før projektet kan importeres, skal du sætte kryds i et felt på trinnet Projekt, og der blev ikke sat kryds. Vælg »Gennemgå importen igen«, sæt krydset, og importér så. Intet blev skrevet.
import-scrivener-error-changed = Projektet er blevet ændret, efter du gennemgik det. Vælg »Gennemgå importen igen«, og importér så. Intet blev skrevet.
import-scrivener-error-choice = Den struktur, der blev valgt i gennemgangen, passer ikke længere til projektet. Vælg »Gennemgå importen igen«, og importér så. Intet blev skrevet.

## Export dialog
export-title = Eksport
export-close = Luk
export-scope-label = Hvad
export-format-label = Format
export-style-label = Eksporttypografi
export-path-label = Fil
export-browse = Gennemse…
export-show-non-exportable = Vis elementer, der ikke eksporteres
export-choose-empty = Intet projekt at vælge fra.
export-cancel = Annullér
export-export = Eksportér
export-save-dialog-title = Eksportér til fil
# Left-pane section headers (normal case) + scope segmented control
export-section-what = Hvad der skal eksporteres
export-section-style = Eksporttypografi
export-section-destination = Destination
export-custom-selection = Brugerdefineret udvalg
export-selected-count = { $count } valgt
# Style-preset summary chips
export-chip-chapters-none = Kapitler: ingen
export-chip-chapters-numbered = Kapitler: nummererede
export-chip-chapters-title = Kapitler: kun titel
export-chip-chapters-both = Kapitler: nummer og titel
export-chip-scene-break-glyph = Sceneskift: { $glyph }
export-chip-scene-break-blank = Sceneskift: tom linje
export-chip-scene-break-none = Sceneskift: intet
export-chip-major-break-glyph = Stort skift: { $glyph }
export-chip-major-break-blank = Stort skift: tom linje
export-chip-major-break-none = Stort skift: intet
export-chip-major-break-same = Begge slags skift ens
export-chip-spacing-single = Linjeafstand: enkelt
export-chip-spacing-onehalf = Linjeafstand: 1½
export-chip-spacing-double = Linjeafstand: dobbelt
export-chip-notes-included = Noter medtaget
export-chip-notes-excluded = Noter udeladt
# Output formats
export-format-docx = Word
export-format-odt = LibreOffice
export-format-html = HTML
export-format-markdown = Markdown
export-format-djot = Djot
export-format-text = Tekst
export-format-latex = LaTeX
export-format-epub = EPUB
export-format-pdf = PDF
# Overwrite confirmation
export-overwrite-title = Erstat den eksisterende fil?
export-overwrite-text = »{ $name }« findes allerede. Vil du erstatte den?
# Progress toast (the export runs as a long operation)
export-progress-title = Eksporterer…
export-cancelled = Eksporten er annulleret
export-done = { $count ->
    [one] { $count } element er eksporteret
   *[other] { $count } elementer er eksporteret
}
export-open-file = Åbn
export-show-in-folder = Vis i mappe
# Error toast: a short reason in the body, the full technical chain behind Details
export-error-title = Der kunne ikke eksporteres
export-error-details = Detaljer

## Backup scheduler (progress toast + failure/prune-warning details, backup review, T1-2/T1-7/T2-3/T2-8/T2-9)
backup-progress-start = Starter…
backup-progress-retention = Rydder op i gamle sikkerhedskopier…
backup-progress-done = Færdig
backup-progress-destination = Destination { $i } af { $n }
backup-details = Detaljer
backup-issues-title = Problemer med sikkerhedskopiering
backup-failed-title = Sikkerhedskopieringen mislykkedes
backup-complete-prune-warning = Sikkerhedskopieringen er fuldført ({ $ok } gemt, { $skipped } allerede ajour). Nogle gamle sikkerhedskopier kunne ikke fjernes

# Search & Replace
search = Søg
search-query-placeholder = Søg…
search-replace-placeholder = Erstat med…
search-replace-toggle = Vis eller skjul Erstat
search-replace-all = Erstat alle
search-opt-case = Forskel på store og små bogstaver
search-opt-whole-word = Hele ord
search-opt-diacritics = Forskel på accenter
search-scope-body = Brødtekst
search-scope-title = Titel
search-scope-synopsis = Synopsis
search-scope-label = Etiket
search-facet-book = Bøger
search-facet-part = Dele
search-facet-chapter = Kapitler
search-facet-scene = Scener
search-facet-note = Noter
search-facet-folder = Mapper
# Rich tooltips for the option toggles
search-tip-case = Forskel på store og små bogstaver: store og små bogstaver behandles forskelligt, så »Elena« og »elena« er to forskellige forekomster.
search-tip-whole-word = Hele ord: kun hele ord findes, så »kat« findes ikke inde i »katalog«.
search-tip-diacritics = Forskel på accenter: bogstaver med accent behandles som forskellige, så »cafe« matcher ikke »café«.
search-tip-body = Brødtekst: søg i teksten i scener og noter.
search-tip-title = Titel: søg i titlerne på ringbindets elementer.
search-tip-synopsis = Synopsis: søg i hvert skriveelements resumé.
search-tip-label = Etiket: søg i den korte note, der vises under titlen på et element i ringbindet.
search-tip-comment = Kommentarer: søg i teksten i kommentartråde og deres svar. En kommentar handler om manuskriptet og er ikke en del af det, så den har sin egen kontakt, og en erstatning lader forekomster i kommentarer være umarkerede, indtil du markerer dem.
search-tip-book = Bøger: bogbeholderen og dens start- og slutmarkører.
search-tip-part = Dele: skillelinjer på delniveau.
search-tip-chapter = Kapitler: kapitler, uanset hvordan projektet gemmer dem.
search-tip-scene = Scener: de elementer, der indeholder din tekst.
search-tip-note = Noter: frie noter.
search-tip-folder = Mapper: almindelige organiseringsmapper og skillelinjer.
search-tip-paratext = En tekst, der hører til bogen, men ikke til historien: et forord, en dedikation, et efterord. Tælles aldrig med i manuskriptet.
search-tip-preserve-case = Bevar store/små bogstaver: en erstatning overtager skrivemåden for det fundne, så »ELENA« bliver til »MARTA«, og »Elena« bliver til »Marta«.
search-tip-replace = Erstat: vis erstatningsfeltet og Erstat alle.
search-error = Søgningen mislykkedes: { $message }
search-no-matches = Ingen forekomster
search-count =
    { $matches ->
        [one] { $matches } forekomst
       *[other] { $matches } forekomster
    } i { $items ->
        [one] { $items } dokument
       *[other] { $items } dokumenter
    }
search-count-truncated =
    { $matches ->
        [one] { $matches } forekomst
       *[other] { $matches } forekomster
    } i { $items ->
        [one] { $items } dokument
       *[other] { $items } dokumenter
    } (kun de første resultater)
search-occurrences = { $count }
search-field-body = Brødtekst
search-field-title = Titel
search-field-synopsis = Synopsis
search-field-label = Etiket
search-field-epigraph = Motto
search-field-comment = Kommentar
search-field-comment-reply = Svar
search-field-footnote = Fodnote
search-include-in-replace = Medtag i Erstat alle
search-collapse-all = Fold alle resultater sammen
search-replace-here = Erstat denne
search-dismiss = Fjern fra resultaterne
search-undo-dismiss = Hent det senest fjernede resultat tilbage
search-replace-nothing = (ingenting)
search-replace-confirm-title = Erstat alle forekomster?
search-replace-confirm-text =
    Vil du erstatte { $occurrences ->
        [one] { $occurrences } forekomst
       *[other] { $occurrences } forekomster
    } af »{ $query }« med »{ $replacement }« i { $items ->
        [one] { $items } dokument
       *[other] { $items } dokumenter
    }? Du kan fortryde det fra meddelelsen bagefter.
search-replace-done-title = Erstatningen er fuldført
search-replace-done =
    { $occurrences ->
        [one] { $occurrences } forekomst
       *[other] { $occurrences } forekomster
    } erstattet i { $items ->
        [one] { $items } dokument
       *[other] { $items } dokumenter
    }.
search-replace-done-skipped =
    { $occurrences ->
        [one] { $occurrences } forekomst
       *[other] { $occurrences } forekomster
    } erstattet i { $items ->
        [one] { $items } dokument
       *[other] { $items } dokumenter
    }. { $skipped ->
        [one] { $skipped } felt
       *[other] { $skipped } felter
    } sprunget over (ændret siden søgningen).
search-replace-undo = Fortryd
search-replace-failed-title = Erstatningen mislykkedes
search-replace-skipped-title = Intet blev erstattet
search-replace-skipped-body =
    { $fields ->
        [one] Denne tekst
       *[other] { $fields } af disse tekster
    } er ændret siden søgningen, så { $fields ->
        [one] den blev
       *[other] de blev
    } ikke rørt. Søg igen for at se, hvor ordene står nu.
search-replace-undo-failed-title = Fortrydelsen mislykkedes
search-preview = Forhåndsvisning
search-preview-empty = Vælg et resultat for at se det her
search-preview-no-prose = Denne forekomst har ingen redigerbar tekst
search-preview-prompt = Kør en søgning for at se en forhåndsvisning her.
search-preview-open-search = Søg i projekt
search-preview-footnote-prompt = Denne forekomst står i selve teksten i en fodnote. Åbn den i panelet Fodnoter for at se og redigere den.
search-preview-open-footnotes = Åbn Fodnoter

# Per-editor find banner (Ctrl+F)
find-placeholder = Find i dette dokument…
find-count = { $current } af { $total }
find-no-results = Ingen resultater
find-close = Luk Find
find-previous = Forrige forekomst (Skift+Enter)
find-next = Næste forekomst (Enter)
find-opt-case = Forskel på store og små bogstaver
find-opt-whole-word = Hele ord
find-replace-toggle = Vis eller skjul Erstat (Ctrl+R)
find-replace-placeholder = Erstat med…
find-replace = Erstat
find-replace-all = Erstat alle
find-preserve-case = Bevar store/små bogstaver

## Dictionaries & spell-checking
# Download toasts (app-local download, not a backend long operation)
dict-download-title = Henter { $name }…
dict-download-done = { $name } er installeret
dict-download-failed = { $name } kunne ikke hentes: { $error }
dict-removed = { $name } er fjernet
dict-accept-first = Acceptér licensen, før du henter { $name }
# Settings ▸ Dictionaries pane
settings-dict-tab-installed = Installeret
settings-dict-tab-get-more = Hent flere
settings-dict-tab-personal = Personlige ord
dict-installed-empty = Der er endnu ikke fundet ordbøger på denne computer.
dict-get-more-search = Søg efter sprog
dict-system-badge = på dit system
dict-unusable-badge = ubrugelig
dict-download-button = Hent
dict-downloading = Henter…
dict-installed-label = Installeret
dict-remove = Fjern
dict-view-license = Vis licens
dict-approx-size = ~{ $size }
dict-personal-empty = Ingen personlige ord i dette projekt endnu.
dict-personal-add = Tilføj
dict-personal-placeholder = Tilføj et ord…
# Missing-dictionary prompt after opening a project
dict-missing-toast = { $count ->
    [one] Projektet bruger { $count } ordbog, som du ikke har installeret
   *[other] Projektet bruger { $count } ordbøger, som du ikke har installeret
}
dict-missing-action = Hent ordbøger
# License modal
dict-license-title = Licens for { $name }
dict-license-accept = Acceptér og hent
dict-license-cancel = Annullér
dict-license-close = Luk
# Add a dictionary (sideload local .aff/.dic files)
dict-add-button = Tilføj ordbog…
dict-add-title = Tilføj en ordbog
dict-add-close = Luk
dict-add-name = Navn
dict-add-name-placeholder = f.eks. Min latinske ordbog
dict-add-code = Sprogkode
dict-add-code-placeholder = f.eks. la eller fr-FR-x-custom
dict-add-code-hint = En kort kode efter eget valg. Det er den, du vælger som et dokuments sprog.
dict-add-aff = Affiksfil (.aff)
dict-add-dic = Ordliste (.dic)
dict-add-submit = Tilføj
dict-add-cancel = Annullér
dict-add-name-required = Giv ordbogen et navn.
dict-add-code-required = Angiv en sprogkode.
dict-add-code-invalid = Brug kun bogstaver, tal og - _ .
dict-add-code-reserved = Det er koden for en indbygget ordbog. Vælg en anden.
dict-add-file-required = Vælg en fil.
dict-add-file-missing = Filen findes ikke.
dict-add-code-taken = Der er allerede installeret en ordbog med den kode.
dict-add-done = { $name } er tilføjet
dict-add-unusable = Filerne er ikke en brugbar ordbog: { $error }
dict-add-failed = Ordbogen kunne ikke tilføjes: { $error }
# The Inspector's per-item export controls (M3)
inspector-export = Eksport
inspector-exportable = Medtag i eksporter
ctx-number = &Nummerér dette kapitel
ctx-unnumber = Nummerér &ikke dette kapitel
inspector-numbering = Nummerering
inspector-numbered = Nummereret
inspector-numbered-tip = Kapitlet indgår i bogens nummerering. Slå det fra for en prolog, en epilog eller et mellemspil.
inspector-numbered-tip-more = Et unummereret kapitel bliver i bogen præcis, som det var: overskrift, tekst og ordantal røres ikke. Det udskriver blot intet nummer og bruger ikke et op: kapitlet efter en prolog er kapitel et, ikke kapitel to. At udelade det fra eksporten er en anden kontakt ovenfor, og den fjerner kapitlet helt fra bogen.
inspector-apply-to-children = Anvend på underelementer
# The Inspector's per-Part/Chapter milestone date (M5), shown on the Book's Pace.
inspector-milestone = Dato for milepæl
inspector-milestone-none = Ingen dato
inspector-milestone-clear = Ryd
# The language pill field (Inspector + Settings)
inspector-tags = Mærker
inspector-aliases = Også kendt som
inspector-dict-language = Sprog
inspector-apply-language-to-children = Anvend sproget på underelementer
settings-page-language = Sprog
settings-field-dict-language = Sprog
dict-tradeoff-hint = Hvert ekstra sprog accepterer flere ord, så færre fejl bliver fanget.
lang-inherit-hint = Arvet: scenen bruger bogens eller projektets sprog.
lang-pill-list = Sprog
lang-pill-add = Tilføj et sprog
lang-pill-remove = Fjern { $name }
lang-pill-mute = Slå stavekontrol fra for { $name }
lang-pill-unmute = Slå stavekontrol til for { $name }

## Settings: Work ▸ Personal dictionary (per-project word list)
settings-page-personal-dictionary = Personlig ordbog
settings-user-dict-desc = Ord, du tilføjer her, betragtes som korrekt stavet overalt i programmet og markeres aldrig igen.
settings-user-dict-add-placeholder = Tilføj et ord…
settings-user-dict-add = Tilføj ord
settings-user-dict-search = Filtrér ord
settings-user-dict-count = { $count ->
    [one] { $count } ord
   *[other] { $count } ord
}
settings-user-dict-rename = Omdøb
settings-user-dict-remove = Fjern
settings-user-dict-empty = Ingen ord endnu. Tilføj ord, som stavekontrollen skal ignorere, eller importér en liste.
settings-user-dict-import = Importér…
settings-user-dict-export = Eksportér…
settings-user-dict-import-tip = Tilføj ord fra en ren tekstfil med ét ord pr. linje. Eksisterende ord bevares.
settings-user-dict-export-tip = Gem hele listen i en ren tekstfil med ét ord pr. linje.
settings-user-dict-txt-filter = Tekstfiler
settings-user-dict-duplicate = Findes allerede i ordbogen
settings-user-dict-imported = { $count ->
    [one] { $count } ord er importeret
   *[other] { $count } ord er importeret
} ({ $duplicates } fandtes allerede).
settings-user-dict-import-failed = Ordlisten kunne ikke læses: { $error }
settings-user-dict-exported = { $count ->
    [one] { $count } ord er gemt.
   *[other] { $count } ord er gemt.
}
settings-user-dict-export-failed = Ordlisten kunne ikke gemmes: { $error }

## Settings: Work ▸ Text replacements (per-project custom lexicon)
settings-page-text-replacements = Teksterstatninger
settings-text-repl-desc = Erstat forkortelser med den fulde tekst, mens du skriver: »bl.a.« bliver til »blandt andet«, så snart du skriver et mellemrum eller et tegn.
settings-text-repl-enable = Brug teksterstatninger i dette projekt
settings-text-repl-disabled-hint = Slå dette til for at definere forkortelser, der udfoldes, mens du skriver.
settings-text-repl-add = Tilføj regel
settings-text-repl-trigger-placeholder = Forkortelse
settings-text-repl-replacement-placeholder = Hvad den bliver til
settings-text-repl-added = »{ $trigger }« er tilføjet
settings-text-repl-duplicate = »{ $trigger }« har allerede en regel
settings-text-repl-filter = Filtrér regler
settings-text-repl-count = { $n ->
    [one] 1 regel
   *[other] { $n } regler
}
settings-text-repl-row-enabled = Brug denne regel
settings-text-repl-delete = Slet reglen for { $trigger }
settings-text-repl-deleted = Reglen for »{ $trigger }« er slettet
settings-text-repl-empty = Ingen regler endnu.
settings-text-repl-csv-filter = CSV-filer
settings-text-repl-import = Importér…
settings-text-repl-export = Eksportér…
settings-text-repl-imported = { $added } importeret, { $skipped } sprunget over
settings-text-repl-exported = { $n ->
    [one] 1 regel er eksporteret
   *[other] { $n } regler er eksporteret
}

## Editor: spelling (context menu + toast)
# Shown in place of the corrections when a flagged word has none to offer.
editor-menu-no-suggestions = Ingen forslag
editor-menu-add-to-dictionary = Føj »{ $word }« til ordbogen
editor-menu-add-words-to-dictionary = Føj de markerede ord til ordbogen
editor-dict-added = »{ $word }« er føjet til din ordbog.
editor-dict-added-multi = { $count ->
    [one] { $count } ord er føjet til din ordbog.
   *[other] { $count } ord er føjet til din ordbog.
}
toast-undo = Fortryd

## Taking one operation back
##
## The Undo on a notification is offered for one specific thing. If anything
## else has happened since, it says so rather than taking back whatever now
## happens to be last.
undo-superseded-title = Det trin er ikke længere det seneste
undo-superseded-body = Noget andet er ændret i projektet siden. Fortryd ville tage det tilbage i stedet, så det er ikke sket. Intet af det, trinnet gjorde, er rullet tilbage. Brug Redigér ▸ Fortryd for selv at gå tilbage gennem historikken.
undo-failed = Fortrydelsen mislykkedes: { $error }

## The Edit menu
##
## The Undo row names what it will take back, because on a stack where prose and
## structure both appear a bare "Undo" leaves the writer guessing whether the
## next press retypes a word or resurrects a chapter.
menu-edit = &Redigér
menu-edit-undo = &Fortryd
menu-edit-redo = &Gentag
menu-edit-undo-target = &Fortryd { $target }
menu-edit-redo-target = &Gentag { $target }
shortcut-name-edit-undo = Fortryd
shortcut-name-edit-redo = Gentag
undo-target-typing = indtastning
undo-target-project = seneste ændring af projektet
prose-history-reset-title = Skrivehistorikken er nulstillet
prose-history-reset-body = { $count ->
    [one] En åben scene blev gendannet fra en tidligere tilstand, så dens skrivehistorik gælder ikke længere.
   *[other] { $count } åbne scener blev gendannet fra en tidligere tilstand, så deres skrivehistorik gælder ikke længere.
}
undo-target-trash = flytning til papirkurven
undo-target-restore = gendannelse fra papirkurven
undo-target-delete-forever = endelig sletning
undo-target-replace-all = erstatning i hele projektet
undo-target-import = dokumentimporten
undo-target-duplicate = duplikering
undo-target-move = flytning
undo-target-merge = sammenfletning af to scener
undo-target-split = opdeling af en scene
undo-target-promote = ændring af typen
undo-target-tidy-titles = oprydning i kapiteltitlerne
undo-target-import-tags = import af mærker
undo-target-import-templates = import af noteskabeloner
undo-target-create = oprettelse
undo-target-remove = sletning
undo-target-rename = omdøbning
undo-target-edit = den redigering
menu-edit-find = Fin&d…
menu-edit-find-next = Find &næste
menu-edit-find-prev = Find fo&rrige
menu-edit-replace = Find og &erstat…
menu-edit-find-in-project = Søg i pro&jekt…
menu-edit-replace-in-project = Erstat i projek&t…
undo-frozen = Fortryd (»Altid fremad« er slået til)
redo-frozen = Gentag (»Altid fremad« er slået til)
menu-edit-cut = &Klip
menu-edit-copy = K&opiér
menu-edit-paste = Sæt &ind
menu-edit-paste-plain = Sæt ind &uden formatering
menu-edit-select-all = Markér &alt
shortcut-name-edit-cut = Klip
shortcut-name-edit-copy = Kopiér
shortcut-name-edit-paste = Sæt ind
shortcut-name-edit-paste-plain = Sæt ind uden formatering
shortcut-name-edit-select-all = Markér alt

## Spell-check: the master switch (title bar / View menu / F7 / Settings ▸ Spelling)
titlebar-spellcheck-on = Stavekontrol er slået til. Klik for at stoppe kontrollen (F7)
titlebar-spellcheck-off = Stavekontrol er slået fra. Klik for at kontrollere igen (F7)
menu-spellcheck = &Kontrollér stavning
menu-comments = Ko&mmentarer
settings-page-spellcheck = Stavekontrol
settings-group-spellcheck = Stavekontrol
settings-spellcheck-enabled = Kontrollér stavningen, mens jeg skriver
settings-spellcheck-hint = Understreger ord, som ingen installeret ordbog kender. Slår du dette fra, stopper al kontrol i alle projekter, indtil du slår det til igen. Vil du kun stoppe kontrollen af ét sprog, så fjern dets flueben i feltet Sprog for projektet eller for et element.

## Trash panel
menu-trash = &Papirkurv
trash-title = Papirkurv
trash-empty-state = Papirkurven er tom.
trash-empty-button = Tøm papirkurven…
trash-restore = &Gendan
trash-restore-to = Gendan &til…
trash-delete-forever = &Slet permanent
trash-restored-ok = { $count } gendannet.
trash-restore-error = Der kunne ikke gendannes: { $error }
trash-restore-orphaned = Elementets oprindelige plads findes ikke længere. Vælg, hvor det skal gendannes.
trash-restore-no-project = Intet projekt er åbent, så der er intet at gendanne.
trash-delete-no-project = Intet projekt er åbent, så der er intet at slette.
trash-empty-confirm-title = Tøm papirkurven?
trash-empty-confirm-text = { $count ->
    [one] { $count } element i papirkurven slettes permanent.
   *[other] { $count } elementer i papirkurven slettes permanent.
} De forlader papirkurven for altid. Fortryd kan stadig hente dem tilbage, indtil du gør noget andet.
trash-emptied-title = Papirkurven er tømt
trash-emptied-body = Alt i papirkurven blev slettet permanent.
trash-empty-no-project = Intet projekt er åbent, så der er ingen papirkurv at tømme.
trash-delete-forever-confirm-title = Slet permanent?
trash-delete-forever-confirm-text = { $count ->
    [one] { $count } element slettes permanent.
   *[other] { $count } elementer slettes permanent.
} De forlader papirkurven for altid. Fortryd kan stadig hente dem tilbage, indtil du gør noget andet.
trash-deleted-title = Slettet permanent
trash-deleted-body = { $count } slettet permanent.
trash-undo = Fortryd
trash-restore-picker-title = Gendan til…
trash-restore-to-confirm-title = Gendan her?
trash-restore-to-confirm-text = Vil du gendanne »{ $item }« i »{ $destination }«?
trash-restore-picker-restore-here = Gendan her
trash-restore-picker-cancel = Annullér
trash-restore-picker-empty = Ingen ringbind endnu. Opret et først.
trash-banner-title = Dette element ligger i papirkurven
trash-banner-description = Det vises ikke i dispositionen eller i eksporter, før du gendanner det.
trash-banner-restore = Gendan…
trash-tab-tooltip = I papirkurven

# Composite tooltip: the full parameter sheet shown on an export-style row.
settings-styles-sheet-yes = Ja
settings-styles-sheet-no = Nej
settings-styles-sheet-font = Skrifttype
settings-styles-sheet-indent = Indrykning af første linje
settings-styles-sheet-para-spacing = Afstand mellem afsnit
settings-styles-sheet-page = Sidestørrelse
settings-styles-sheet-margins = Margener (T/H/B/V)
settings-styles-sheet-title-page = Bogens titelside
settings-styles-sheet-heading-language = Overskrifternes sprog
settings-styles-sheet-auto = Følger teksten
settings-styles-sheet-digits = Talsystem
settings-styles-sheet-direction = Tekstretning
settings-styles-sheet-formats = Formater
settings-styles-sheet-all-formats = Alle

settings-styles-editor-group = Typografieditor
settings-styles-editor-missing = Den typografi er ikke længere tilgængelig.

settings-styles-page-letter = Letter
settings-styles-digits-western = Vestlige (0–9)
settings-styles-digits-eastern-arabic = Østarabiske (٠–٩)
settings-styles-direction-ltr = Venstre mod højre
settings-styles-direction-rtl = Højre mod venstre

## Format dock
format-dock-title = Format
format-panel-empty = Placér markøren i en scene, en note eller en synopsis for at se formateringsmulighederne.
# Group headers.
format-group-marks = Tekst
format-group-block = Afsnit
format-group-lists = Lister
format-group-tables = Tabel
format-group-breaks = Sceneskift
# Button tooltips. Icon-only buttons, so each tooltip is that button's only
# accessible name, not decoration.
format-superscript = Hævet skrift
format-subscript = Sænket skrift
format-link = Link…
# The Link command: a hyperlink in the prose. One command, three doors (the
# Format dock, the Format menu and Ctrl+K), so these strings are shared.
link-dialog-insert-title = Indsæt link
link-dialog-edit-title = Redigér link
link-dialog-text-label = Tekst
link-dialog-text-placeholder = de ord, læseren ser
link-dialog-url-label = Linker til
link-dialog-url-placeholder = eksempel.dk
link-dialog-insert = Indsæt
link-dialog-apply = Anvend
link-dialog-cancel = Annullér
link-dialog-remove = Fjern link
# Refused because a document could otherwise make a click launch a program.
link-scheme-refused = Kun web- og e-maillinks kan åbnes. { $url } blev ikke rørt.
menu-format-link = L&ink…
format-clear = Ryd formatering
format-heading = Overskriftsniveau
format-heading-normal = Normal tekst
format-heading-1 = Overskrift 1
format-heading-2 = Overskrift 2
format-heading-3 = Overskrift 3
format-heading-4 = Overskrift 4
format-heading-5 = Overskrift 5
format-heading-6 = Overskrift 6
format-align-left = Venstrejustér
format-align-center = Centrér
format-direction-rtl = Afsnit fra højre mod venstre
format-blockquote = Blokcitat
format-list-bullet = Punktopstilling
format-list-numbered = Nummereret liste
format-indent = Forøg indrykning
format-outdent = Formindsk indrykning
format-table-insert = Indsæt tabel
format-table-row-above = Indsæt række over
format-table-row-below = Indsæt række under
format-table-col-before = Indsæt kolonne før
format-table-col-after = Indsæt kolonne efter
format-table-row-delete = Slet række
format-table-col-delete = Slet kolonne
format-table-remove = Fjern tabel

## Menu: Format
menu-format-marks-bold = &Fed
menu-format-marks-italic = &Kursiv
menu-format-marks-underline = &Understreget
menu-format-marks-strike = &Gennemstreget
menu-format-marks-superscript = &Hævet skrift
menu-format-marks-subscript = Sænk&et skrift
menu-format-marks-clear = &Ryd formatering
menu-format-heading = &Overskrift
menu-format-heading-normal = &Normal tekst
menu-format-heading-1 = Overskrift &1
menu-format-heading-2 = Overskrift &2
menu-format-heading-3 = Overskrift &3
menu-format-heading-4 = Overskrift &4
menu-format-heading-5 = Overskrift &5
menu-format-heading-6 = Overskrift &6
menu-format-alignment = &Justering
menu-format-align-left = &Venstrejustér
menu-format-align-center = &Centrér
menu-format-direction = Ret&ning
menu-format-direction-auto = &Automatisk
menu-format-direction-ltr = &Venstre mod højre
menu-format-direction-rtl = &Højre mod venstre
menu-format-blockquote = &Blokcitat
menu-format-lists = &Lister
menu-format-list-bullet = &Punktopstilling
menu-format-list-numbered = &Nummereret liste
menu-format-indent = &Forøg indrykning
menu-format-outdent = F&ormindsk indrykning
menu-format-table = &Tabel
menu-format-table-insert = &Indsæt tabel
menu-format-table-2x2 = &2 × 2
menu-format-table-3x3 = &3 × 3
menu-format-table-4x4 = &4 × 4
menu-format-table-row-above = Indsæt række &over
menu-format-table-row-below = Indsæt række &under
menu-format-table-col-before = Indsæt &kolonne før
menu-format-table-col-after = Indsæt kolonne &efter
menu-format-table-row-delete = &Slet række
menu-format-table-col-delete = Slet ko&lonne
menu-format-table-remove = &Fjern tabel

## Formatting marks: the invisible characters a writer places by hand.
## Mnemonics are unique within the submenu (N, H, S, r, Z, W).
menu-format-typo-marks = For&materingstegn
menu-format-typo-nbsp = &Hårdt mellemrum
menu-format-typo-nbhyphen = H&ård bindestreg
menu-format-typo-soft-hyphen = &Blød bindestreg
menu-format-typo-nnbsp = &Smalt hårdt mellemrum
menu-format-typo-zwsp = &Nulbredde-mellemrum
menu-format-typo-word-joiner = &Ordsammenbinder

## About panel

menu-about = &Om Skribisto…
about-title = Om Skribisto
about-version = Version { $version }
about-update-available = Version { $version } er tilgængelig
about-update-see-notes = Se, hvad der er ændret
about-update-flathub = Flathub holder denne kopi opdateret.
about-update-distro = Din distribution holder denne kopi opdateret.
about-tagline = Et program til at skrive romaner og andre lange fiktionstekster, skrevet i Rust med Teksilo-værktøjssættet.
about-license = Udgivet under GNU General Public License, version 3.
about-copyright = © 2026 Cyril Jacquet
about-close = Luk

## Native (macOS) menu bar
# Labels for the platform-standard App and Window menus mirrored into the macOS
# global menu bar. No `&` mnemonics here: macOS has none, and the native bridge
# resolves these without stripping one, so an ampersand would print literally.
# The application name is data (the running edition's own name), so it arrives
# as an argument: same rule as the window titles below.
native-menu-about = Om { $app }
native-menu-hide = Skjul { $app }
native-menu-quit = Afslut { $app }
native-menu-settings = Indstillinger…
native-menu-window = Vindue
native-menu-minimize = Minimér
native-menu-zoom = Zoom

# Window titles. The application name is data (the running edition's own name),
# so it arrives as an argument rather than being written into the value here.
window-title = { $title } - { $app }
window-title-numbered = { $title } - { $app } (vindue { $n })
window-title-empty = { $app }

# First-run settings import (an edition with its own config directory, finding
# the community installation's settings beside it).
first-run-window-title = Velkommen
first-run-title = Opsæt { $app }
first-run-body = { $app } gemmer sine indstillinger adskilt fra Skribisto og starter derfor tom. Dine præferencer, seneste projekter, ordbøger og vindueslayout kan kopieres over nu.
first-run-from = Kopiér fra
first-run-to = Kopiér til
first-run-copy-note = Intet flyttes eller fjernes. Skribisto beholder alle sine egne indstillinger og fungerer præcis som før.
first-run-import = Importér indstillinger
first-run-start-fresh = Start forfra
first-run-import-failed = Nogle indstillinger kunne ikke importeres: { $error }

# ── Work ▸ Punctuation: the project's typographic house style ──────────────
settings-page-punctuation = Standard for tegnsætning
settings-group-punctuation = Smart tegnsætning
settings-page-work-punctuation = Tegnsætning
settings-punctuation-override = Giv dette projekt sine egne regler for tegnsætning
settings-punctuation-override-hint = Fra: projektet følger programmets indstilling. Reglerne følger med i .skrib-filen, så en medforfatter, der åbner filen, skriver med samme typografi.
settings-punctuation-dashes = Lav -- om til en kort tankestreg og --- om til en lang tankestreg
settings-punctuation-ellipsis = Lav ... om til en ellipse (…)
settings-punctuation-quotes = Brug typografiske anførselstegn og apostroffer
settings-quote-style = Anførselstegn
settings-quote-style-locale = Sprogets standard
settings-quote-style-curly = “Dobbelte”
settings-quote-style-curly-single = ‘Enkelte’
settings-quote-style-guillemets = «Gåseøjne»
settings-quote-style-low-high = „Lav-høj“
settings-punctuation-spacing = Mellemrum før ; : ! ?
settings-punctuation-spacing-hint = Fransk typografi sætter et smalt hårdt mellemrum før ; ! ? og et fuldt før :. Det gælder kun tekst skrevet på fransk. Mellemrummet inden for gåseøjnene « » følger med selve gåseøjne-anførselstegnene.
settings-punctuation-sample = Dit sprog giver
settings-punctuation-app-hint = Det, alle projekter gør, medmindre de har deres egne regler under Projekt ▸ Tegnsætning.
settings-punctuation-dialogue = Begynd et afsnit, der indledes med »- «, med en replikstreg
settings-punctuation-dialogue-hint = Til sprog, der markerer replikker med en tankestreg i stedet for anførselstegn: fransk, spansk, russisk og andre. Det sker kun helt i begyndelsen af et afsnit.

## Go to (jump to any item)
statusbar-go-to = Gå til…
go-to-placeholder = Søg i ringbindet
go-to-no-matches = Intet element matcher søgningen.
menu-go-to = &Gå til…

# ── Distraction-free themes (Settings ▸ Editor ▸ Distraction-free themes) ──
settings-page-distraction-free-themes = Distraktionsfri temaer
settings-themes-builtin = Indbyggede temaer
settings-themes-builtin-badge = Indbygget
settings-themes-user = Mine temaer
settings-themes-editor-group = Redigér tema
settings-themes-editor-empty = Vælg et tema under »Mine temaer« for at redigere det.
settings-themes-use = Brug
settings-themes-duplicate = Duplikér
settings-themes-edit = Redigér
settings-themes-delete = Slet
settings-themes-export = Eksportér…
settings-themes-import = Importér…
settings-themes-copy-suffix = kopi
settings-themes-json-filter = Tema (JSON)
settings-themes-imported = Temaet er importeret
settings-themes-import-failed = Temaet kunne ikke importeres
settings-themes-exported = Temaet er eksporteret
settings-themes-export-failed = Temaet kunne ikke eksporteres
# Shown on a theme whose text and page are below the WCAG AA contrast floor.
settings-themes-low-contrast = lav kontrast
# Shown when the page and the text are fine but the caret band is what hides
# the prose: a different fault, and one the row's own swatches do not show.
settings-themes-low-contrast-band = fremhævningen skjuler teksten
settings-themes-field-name = Navn
settings-themes-field-paper = Side
settings-themes-field-ink = Tekst
settings-themes-field-general = Baggrund
settings-themes-field-widget-text = Tekst i kontrolbjælken
# The shading drawn around the caret: the sentence or paragraph being written,
# per Editor ▸ "Highlight around the caret". This field is the color it uses in
# distraction-free mode.
settings-themes-field-caret-band = Fremhævning omkring markøren

# The distraction-free strip's quick-settings gear, and the way through to the
# full theme library from inside it.
statusbar-focus-settings = Indstillinger for distraktionsfri tilstand
statusbar-focus-manage-themes = Administrér temaer…

## Comments: the two docks, the thread cards, and their actions.
comments-title = Kommentarer
comments-document-title = Dette dokument
comments-empty-project = Ingen kommentarer i dette projekt endnu.
comments-empty-document = Ingen kommentarer til dette dokument.
comments-filter-all = Alle
comments-filter-open = Åbne
comments-filter-resolved = Løste
# Always shown, including at zero: an orphan nobody looks for is an orphan nobody fixes.
comments-filter-orphaned = Forældreløse
comments-status-open = Åben
comments-status-resolved = Løst
comments-status-orphaned = Teksten er væk
comments-orphan-snippet = (den kommenterede tekst er væk)
# A comment that resolved successfully yet has no live range to point at: an
# import from a format with no text-position concept for it (a heading, a blank
# paragraph, a table). Distinct from "Lost its text": nothing went missing, it
# never had a position to begin with.
comments-status-unplaced = Ingen tekstposition
comments-unplaced-snippet = (ikke forankret i nogen tekst)
comments-reply-count = { $count ->
    [one] { $count } svar
   *[other] { $count } svar
}
comments-menu-resolve = Markér som løst
comments-menu-reopen = Genåbn
comments-menu-delete = Slet
comments-sort-document = I dokumentets rækkefølge
comments-sort-newest = Nyeste først
comments-sort-reader = Grupperet efter læser
comments-unsigned-author = Uden underskrift
# Shown on a thread that shares its passage with another reader's. Never at one:
# a lone remark needs no number, and "1 reader" beside every card would drown the
# two that say 3.
comments-agreement = { $count ->
    [one] { $count } læser
   *[other] { $count } læsere
}
comments-menu-add = Tilføj kommentar
comments-menu-add-paragraph = Kommentér dette afsnit
overview-col-comments = Kommentarer
overview-col-total-comments = Kommentarer i alt
comments-card-placeholder = Skriv en kommentar…
comments-card-unknown-author = Ukendt forfatter
comments-unsigned-toast = Dine kommentarer er uden underskrift: der er ikke angivet et navn for dig på denne computer.
comments-unsigned-action = Angiv dit navn
comments-card-reply = Svar
comments-card-reply-placeholder = Svar…
comments-card-actions = Handlinger for kommentar
comments-reply-actions = Handlinger for svar
comments-menu-delete-reply = Slet svar
comments-menu-delete-all = Slet alle kommentarer her
comments-deleted-toast = Kommentaren er slettet
comments-reply-deleted-toast = Svaret er slettet
comments-deleted-all-toast = { $count ->
    [one] { $count } kommentar er slettet
   *[other] { $count } kommentarer er slettet
}
comments-undo = Fortryd

# ── Analysis (Book container segment) ────────────────────────────────────────
# House rule for every string here: describe, never judge. No "too many", no
# "weak", no "should". Every comparison is against the book's own median or its
# own distribution, never a genre norm, and never a target value.
analysis-segment = Analyse
analysis-scope-book = Analyserer denne bog
analysis-run = Kør analyse
analysis-stale = Ændret siden analysen
analysis-not-run = Ikke analyseret endnu.
analysis-running = Læser manuskriptet…
analysis-failed = Analysen kunne ikke gøres færdig.
analysis-no-scenes = Ingen scener i denne bog endnu.

analysis-shape = Form

# ── How the text arrived ──────────────────────────────────────────────────────
# A fact about input, never about authorship. Nothing here weighs one route
# against another, and there is no total to reach and nothing to score.
analysis-arrivals = Tilgang
analysis-arrivals-explainer = Hvordan tekst er kommet ind i projektet, mens det har været åbent. Det fortæller, ad hvilken vej tegnene kom, og intet som helst om, hvem der skrev dem: en forfatter, der skriver udkast andetsteds og sætter ind, har sat ind, og en, der dikterer, har dikteret. Der er intet tal at sigte efter her.
# The two scope caveats, said plainly rather than left to be assumed. Everything
# else on this bar is about one book and the whole of its text; this is about the
# whole project and only since it was opened.
analysis-arrivals-scope = Hele projektet, ikke kun denne bog.
analysis-arrivals-session = Siden du åbnede det. Når projektet lukkes, starter optællingen forfra.
analysis-arrivals-nothing = Der er endnu ikke kommet tekst ind i denne session.
analysis-arrivals-typed = Skrevet
analysis-arrivals-pasted = Sat ind
analysis-arrivals-dictated = Dikteret
analysis-arrivals-imported = Importeret
analysis-arrivals-programmatic = Indsat for dig
analysis-arrivals-count = { $count ->
    [one] { $count } tegn
   *[other] { $count } tegn
}
# Said where a route contributed nothing, so a reader is not left wondering
# whether it was measured at all.
analysis-arrivals-none = ingen

analysis-words-per-scene = Ord pr. scene
analysis-median-words = Bogens mediane scene er på { $count ->
    [one] { $count } ord.
   *[other] { $count } ord.
}
analysis-median-line = Median: { $count ->
    [one] { $count } ord
   *[other] { $count } ord
}
analysis-dialogue = Dialog
# Said instead of showing 0%, which would read as "there is no dialogue here".
analysis-dialogue-unsupported = Dialog måles endnu ikke for dette sprog.

analysis-footnote-words = Ord i fodnoter
# Kept apart from the manuscript total on purpose. See the module doc on
# AnalysisViewModel::run for why a footnote is authored prose but must not be folded
# into how far along the story reads as being.
analysis-footnote-words-count = { $count ->
    [one] { $count } ord står i bogens fodnoter og holdes uden for manuskriptets samlede antal.
   *[other] { $count } ord står i bogens fodnoter og holdes uden for manuskriptets samlede antal.
}
# The figure comes from its own operation and can still be catching up even once the
# rest of this report is ready, said plainly rather than shown as a misleading 0.
analysis-footnote-words-pending = Tæller fodnoterne…

# Shape's charts draw one bar per text, so an outlined-but-unwritten book is mostly gaps.
analysis-ignore-empty = Ignorér tekster, hvor der endnu ikke er skrevet noget
analysis-empty-hidden =
    { $count } { $count ->
        [one] tom tekst
       *[other] tomme tekster
    } skjult.
analysis-all-texts-empty = Alle tekster i denne bog er stadig tomme.

# ── Binder filter feedback ───────────────────────────────────────────────────
# The binder's search field lives in a popover, so once it is dismissed nothing
# on screen says a filter is still narrowing the tree. These strings are what
# say it. Without them a filtered-to-nothing binder is indistinguishable from
# an empty project.
binder-filter-count = { $shown } af { $total } vist
binder-filter-clear = Ryd
binder-filter-none = Intet matcher »{ $query }«.

## Settings: paratext structures
settings-paratext-intro = Det for- og efterstof, et nyt projekt kan starte med. Hver struktur hører til en udgivertradition, og dens sidetitler er skrevet på traditionens eget sprog. Omdøb dem frit, når projektet er oprettet.
settings-paratext-structures = Strukturer
settings-paratext-broken = Kunne ikke læses
settings-paratext-edit = Redigér
settings-paratext-duplicate = Duplikér
settings-paratext-delete = Slet
settings-paratext-new = Ny struktur
settings-paratext-save = Gem
settings-paratext-editor-hint = Et navn, de sider, der skal oprettes før manuskriptet, og de sider, der skal oprettes efter det. Hvor de havner, bestemmer du: de er almindelige elementer, når projektet først findes.

## Images

image-insert = Indsæt &billede…
image-no-project = Åbn et projekt, før du indsætter et billede.
image-choose-title = Vælg et billede
image-filter-label = Billeder
image-large-title = Dette er et stort billede
image-large-text =
    { $name } er på { $megapixels } megapixel ({ $width }×{ $height }).
    Beholder du det, som det er, gemmes din originale fil i projektet: den følger
    med i alle sikkerhedskopier og alle eksporter. Optimér gemmer i stedet en mindre
    kopi, højst 2560 pixel på den længste side.
image-large-keep = Behold originalen
image-large-downscale = Optimér
image-large-remember = Gør dette fremover, og spørg ikke igen
image-not-recorded = Billedet blev gemt, men kunne ikke registreres i projektet.
image-describe-title = Beskriv billedet
image-describe-explain = Hvad billedet viser, til en læser, der ikke kan se det. Det er ikke en del af manuskriptet: det tælles, gennemsøges og eksporteres aldrig som tekst.
image-describe-placeholder = et fyrtårn mod en grå himmel
image-resize-title = Skift billedets størrelse
image-resize-explain = En procentdel af den størrelse, det vises i nu. 100 lader det være, som det er.
image-resize-invalid = Angiv et tal mellem 1 og 1000.
image-menu-describe = &Beskriv billedet…
image-menu-resize = &Skift billedets størrelse…
image-menu-reset-size = &Oprindelig størrelse

# The book's cover, chosen from the book, not typed into a scene.
cover-heading = Omslag
cover-pick = Vælg…
cover-change = Skift…
cover-remove = Fjern
cover-none = Intet omslag endnu
cover-missing = Billedfilen mangler i projektet
cover-choose-title = Vælg et omslag
cover-set = Omslaget er valgt.
cover-cleared = Omslaget er fjernet. Billedet findes stadig i projektet.
# ── Outline row card ────────────────────────────────────────────────────────
# The hover card on an outline row: what a row is, without opening it.
card-label = Etiket
card-exportable = Eksporteres
card-numbered = Nummereret
card-created = Oprettet
card-modified = Ændret
card-yes = ja
card-no = nej
card-type = Type
card-position = Position
card-children = Underelementer
card-goal = Mål
card-words = Ord
card-synopsis = Synopsis
card-point-of-view = Synsvinkel
card-aliases = Også kendt som

export-orphan-footnotes-title = Fodnoter uden henvisning
export-comments-dropped =
    { $count ->
        [one] En kommentar kunne ikke placeres i den eksporterede tekst og blev udeladt.
       *[other] { $count } kommentarer kunne ikke placeres i den eksporterede tekst og blev udeladt.
    }
export-orphan-footnotes =
    { $count ->
        [one] En fodnote henvises der ikke længere til noget sted i manuskriptet. Dens tekst kommer ikke med i den eksporterede bog.
       *[other] { $count } fodnoter henvises der ikke længere til noget sted i manuskriptet. Deres tekst kommer ikke med i den eksporterede bog.
    }

## Footnotes

menu-footnotes = F&odnoter
footnotes-title = Fodnoter
footnotes-insert = Indsæt &fodnote
footnotes-empty = Ingen fodnoter endnu. Placér markøren i en scene, og brug så + ovenfor eller Ctrl+Alt+F.
footnotes-filter-all = Alle
footnotes-filter-document = Dette dokument
footnotes-filter-orphaned = Forældreløse
footnotes-orphaned = Intet henviser længere til denne note
footnotes-untitled-home = Uden titel
footnotes-body-placeholder = selve noten
footnotes-insert-tooltip = Indsæt en fodnote ved markøren (Ctrl+Alt+F)
footnotes-actions = Handlinger for fodnote
footnotes-delete = &Slet noten og henvisningen til den
footnotes-no-project = Åbn et projekt, før du indsætter en fodnote.
footnotes-no-caret = Placér markøren i en scenes tekst for at indsætte en fodnote der.
footnotes-not-created = Fodnoten kunne ikke føjes til projektet.
footnotes-deleted-toast = Noten er slettet
footnotes-undo-delete = Fortryd

# ── Import documents (Markdown / Word / ODT / plain text) ─────────────────────
import-document-title = Importér dokumenter
import-document-close = Luk
import-document-step-files = Filer
import-document-step-review = Gennemgang
import-document-step-destination = Destination
import-document-step-reconcile = Flet
import-document-reconcile-hint = Nogle af disse elementer har du allerede. Angiv, hvad der skal ske med hvert enkelt.
import-document-reconcile-all-new = Intet i denne fil matcher dit projekt: alle elementer tilføjes som nye.
import-document-duplicate-returns = Disse filer er flere kopier af det samme manuskript, der kommer tilbage. Importér dem én ad gangen: sammenligningen matcher kun én returneret kopi med dit projekt, så importerer du flere på én gang, tilføjes en ekstra kopi af bogen i stedet for at blive flettet ind.
import-document-hunk-added = Tag dette nye afsnit
import-document-hunk-removed = Fjern dette afsnit
import-document-hunk-changed = Tag denne omskrivning
import-document-col-stray-prose = Dets tekst
import-document-stray-as-paratext = Behold som paratekst
import-document-stray-discard = Fjern teksten
import-document-col-current = I dit projekt
import-document-col-incoming = I denne fil
import-document-col-status = Status
import-document-col-action = Hvad der skal ske
import-document-status-identical = Ens
import-document-status-editor-edited = De har redigeret det
import-document-status-you-edited = Du har redigeret det
import-document-status-conflict = Begge har redigeret det
import-document-status-different = Forskellig
import-document-status-new = Ny
import-document-status-missing = Ikke i denne fil
import-document-status-moved = Flyttet
import-document-action-comments-only = Kun kommentarer
import-document-action-take-import = Tag denne version
import-document-action-keep-current = Behold min
import-document-action-create-new = Tilføj som ny
import-document-action-ignore = Spring over
import-document-compare = Sammenlign
import-document-compare-legend = Din version sammenlignet med filens. Skrivebeskyttet.
import-document-compare-close = Luk
import-document-drop-title = Slip dokumenter her
import-document-drop-hint = Markdown (.md), Word (.docx), OpenDocument (.odt) og ren tekst (.txt)
import-document-browse = Gennemse…
import-document-move-up = Flyt op
import-document-move-down = Flyt ned
import-document-remove-file = Fjern
import-document-file-count = { $count ->
    [one] 1 fil
   *[other] { $count } filer
}
import-document-no-files = Ingen filer valgt endnu.
import-document-col-included = Importér
import-document-col-title = Titel
import-document-col-type = Type
import-document-col-words = Ord
import-document-col-breaks = Sceneskift
import-document-col-comments = Kommentarer
import-document-col-footnotes = Fodnoter
import-document-col-epigraph = Motto
import-document-col-source = Kilde
import-document-level-rules = Overskriftsniveauer
import-document-level-n = Overskrift { $level }
import-document-add-top-level = Tilføj øverste niveau
import-document-add-top-level-tooltip = Indsæt en bog over alle analyserede elementer, til kapitelfiler uden en bogoverskrift
import-document-destination = Destination
import-document-destination-hint = Vælg et ringbind eller et element: nye elementer havner inde i en mappe eller efter en scene.
import-document-destination-empty = Ingen ringbind endnu. Opret et først.
import-document-plan-empty = Intet at importere endnu.
# How many pictures the items being imported bring into the project, under the
# review tree. Shown only when there is at least one.
import-document-pictures = { $count ->
    [1] Ét billede følger med disse dokumenter.
   *[other] { $count } billeder følger med disse dokumenter.
}
import-document-summary = { $rows ->
    [one] 1 element
   *[other] { $rows } elementer
} · { $breaks ->
    [one] 1 sceneskift
   *[other] { $breaks } sceneskift
}
import-document-back = Tilbage
import-document-cancel = Annullér
import-document-analyse = Næste
import-document-analysing = Læser dokumenter…
import-document-step-analysing = Læser
import-document-cancel-analysis = Stop læsningen
import-document-analyse-failed = Dokumenterne kunne ikke læses.
import-document-details = Detaljer
import-document-import = Importér
import-document-done = { $count ->
    [one] 1 element er importeret
   *[other] { $count } elementer er importeret
}
import-document-undo = Fortryd
# ── Import diagnostics ────────────────────────────────────────────────────────
# One per `document_ingest::ImportDiagnostic::key()`. The variant's data arrives
# as arguments; the sentence is assembled here, per locale. A row-scoped
# diagnostic gets its { $title } and { $kind } from the row it names, never from
# the wire.
import-diagnostic-file-unreadable = »{ $path }« kunne ikke læses: { $detail }. De øvrige filer importeres stadig.
# A file refused unread because its XML nests deeper than the app will read.
# $limit is the ceiling, a number.
import-diagnostic-nested-too-deep = »{ $path }« blev ikke læst: indholdet er indlejret mere end { $limit } niveauer dybt. Intet rigtigt dokument kommer i nærheden af det, og at læse det kunne få Skribisto til at gå ned, så det udelades. De øvrige filer importeres stadig.
import-diagnostic-archive-too-large = »{ $path }« blev ikke læst: det er en komprimeret fil, der er bygget til at fylde langt mere udpakket, end den gør, hvilket kunne opbruge hukommelsen og få Skribisto til at gå ned, så den udelades. De øvrige filer importeres stadig.
import-diagnostic-lossy-decode = { $count ->
    [one] Ét tegn i »{ $path }« kunne ikke afkodes. Gem filen igen som UTF-8 for at bevare det.
   *[other] { $count } tegn i »{ $path }« kunne ikke afkodes. Gem filen igen som UTF-8 for at bevare dem.
}
import-diagnostic-decoded-from-bom = »{ $path }« blev afkodet som { $detail }, ikke UTF-8.
import-diagnostic-empty-file = »{ $path }« er tom.
import-diagnostic-no-headings = »{ $path }« har ingen overskrifter, så den kommer ind som ét element.
import-diagnostic-unsupported-format = Intet kan læse ».{ $detail }«-filer, så »{ $path }« blev sprunget over.
import-diagnostic-front-matter-not-flat = Front matter i »{ $path }«: »{ $detail }« er ikke en simpel værdi og blev sprunget over.
import-diagnostic-footnotes-degraded = { $count ->
    [one] Én fodnote i »{ $path }« kommer ind som almindelig tekst: fodnoter læses ikke fra Markdown.
   *[other] { $count } fodnoter i »{ $path }« kommer ind som almindelig tekst: fodnoter læses ikke fra Markdown.
}
# The sibling of the line above, and deliberately a different sentence. That one
# says the reader cannot carry footnotes at all (Markdown); this one says it
# carried the rest and names what it could not: a note with no text behind it, or
# one whose reference sits where no prose block covers it, a chapter title being
# the ordinary case.
import-diagnostic-footnote-not-carried = { $count ->
    [one] Én fodnote i »{ $path }« kunne ikke tages med. De andre blev.
   *[other] { $count } fodnoter i »{ $path }« kunne ikke tages med. De andre blev.
}
import-diagnostic-raw-html-dropped = { $count ->
    [one] Én blok rå HTML i »{ $path }« blev udeladt.
   *[other] { $count } blokke rå HTML i »{ $path }« blev udeladt.
}
import-diagnostic-nested-break-dropped = { $count ->
    [one] Ét sceneskift inde i et citat eller en liste i »{ $path }« blev udeladt. Kun et sceneskift på sin egen linje bevares.
   *[other] { $count } sceneskift inde i citater eller lister i »{ $path }« blev udeladt. Kun et sceneskift på sin egen linje bevares.
}
# A picture the document shows that does not come with it, one sentence per reason.
# $detail is the picture as the document names it: a file name, a path inside the
# document, or a web address. Nothing is left in the text where it was.
import-diagnostic-image-not-ingested-format = I »{ $path }« er billedet »{ $detail }« i et format, Skribisto ikke kan vise, så det blev udeladt. Billeder i PNG, JPEG, WebP, GIF, TIFF eller BMP kommer med; gem det i et af de formater, og indsæt det manuelt.
import-diagnostic-image-not-ingested-remote = I »{ $path }« ligger billedet »{ $detail }« på nettet. Billeder hentes aldrig ned under en import, så det blev udeladt.
import-diagnostic-image-not-ingested-outside = I »{ $path }« ligger billedet »{ $detail }« uden for dokumentet og dets mappe, så det blev udeladt. Kun billeder, der er gemt i dokumentet eller i dokumentets egen mappe, kommer med.
import-diagnostic-image-not-ingested-missing = I »{ $path }« kunne billedet »{ $detail }« ikke findes, så det blev udeladt.
import-diagnostic-image-not-ingested-too-large = I »{ $path }« er billedet »{ $detail }« større, end et projekt kan rumme, så det blev udeladt.
import-diagnostic-image-not-ingested-unreadable = I »{ $path }« kunne billedet »{ $detail }« ikke læses som et billede, så det blev udeladt.
import-diagnostic-image-not-ingested-heading = I »{ $path }« står billedet »{ $detail }« i en overskrift. Hver overskrift bliver til titlen på et element i ringbindet, og en titel kan ikke rumme billeder, så billedet blev udeladt.
import-diagnostic-image-not-ingested-note = I »{ $path }« står billedet »{ $detail }« i en fodnote, en slutnote eller en kommentar. De beholder deres ord, men ingen billeder, så det blev udeladt.
import-diagnostic-image-not-ingested-html = I »{ $path }« er billedet »{ $detail }« skrevet som HTML, og en import udelader HTML, så billedet blev udeladt sammen med den. Skriv det som et Markdown-billede for at få det med.
import-diagnostic-image-not-ingested-document-full = I »{ $path }« blev billedet »{ $detail }« udeladt: dokumentets billeder overstiger tilsammen, hvad én import tager med. Importér dokumentet i dele, eller indsæt resten manuelt.
import-diagnostic-picture-first-frame-only = I »{ $path }« kommer det animerede billede »{ $detail }« kun med som sit første enkeltbillede. Billeder i et projekt bevæger sig ikke.
import-diagnostic-duplicate-title = »{ $title }« forekommer { $count } gange. Har du importeret disse filer før, vil de blive duplikeret.
import-diagnostic-heading-level-jump = »{ $title }« springer fra overskriftsniveau { $from } til { $to }; det placeres ét niveau under sit overordnede element.
import-diagnostic-illegal-combination = »{ $title }« indeholder tekst, men en { $kind } kan ikke rumme tekst. Importen holdes tilbage, indtil du ændrer typen eller udelader elementet. Ellers importeres der slet intet.
# Two sentences, because the file may or may not say who. `$names` is a
# ready-formatted list the app never splits; empty when the source records no
# author, which is what an anonymised .docx looks like.
import-diagnostic-tracked-changes-flattened = { $path } var midt i en revision: { $count ->
    [one] { $count } registreret ændring blev accepteret
   *[other] { $count } registrerede ændringer blev accepteret
}, og sletninger blev fjernet. Det er den endelige tekst. Vil du i stedet beholde noget af din egen ordlyd, så brug Sammenlign i det sidste trin.
import-diagnostic-tracked-changes-flattened-by = { $path } var midt i en revision: { $count ->
    [one] { $count } registreret ændring af { $names } blev accepteret
   *[other] { $count } registrerede ændringer af { $names } blev accepteret
}, og sletninger blev fjernet. Det er den endelige tekst. Vil du i stedet beholde noget af din egen ordlyd, så brug Sammenlign i det sidste trin.
import-diagnostic-text-box-dropped = { $count ->
    [one] { $path } indeholder { $count } tekstfelt. Dets tekst står uden for dokumentets forløb, så det kan ikke afgøres, hvor den hører hjemme i et manuskript. Det importeres ikke.
   *[other] { $path } indeholder { $count } tekstfelter. Deres tekst står uden for dokumentets forløb, så det kan ikke afgøres, hvor den hører hjemme i et manuskript. De importeres ikke.
}
import-diagnostic-embedded-object-dropped = { $count ->
    [one] { $path } indeholder { $count } indlejret objekt: et diagram, en ligning eller lignende. Der er intet i et manuskript, der kan rumme det.
   *[other] { $path } indeholder { $count } indlejrede objekter: et diagram, en ligning eller lignende. Der er intet i et manuskript, der kan rumme dem.
}
import-diagnostic-field-flattened = { $count ->
    [one] { $path } indeholder { $count } felt: et sidetal, en krydshenvisning, en dato. Det beholder den tekst, det sidst viste, og opdateres ikke igen.
   *[other] { $path } indeholder { $count } felter: et sidetal, en krydshenvisning, en dato. Hvert felt beholder den tekst, det sidst viste, og opdateres ikke igen.
}
import-diagnostic-unknown-style-level = { $path } bruger typografien »{ $detail }«, der ligner en overskrift, men ikke angiver noget niveau. Disse afsnit importeres som tekst i stedet for at blive gættet ind på et niveau.
import-diagnostic-comment-unanchored = Kommentaren »{ $detail }« i { $path } kunne ikke knyttes til de ord, den handlede om. Den bevares på sit element, hvor du kan flytte den.
# The sibling of the line above, for the one case where there is no item to keep the
# comment on: the file brought no text at all, or only headings.
import-diagnostic-comment-not-carried = Kommentaren »{ $detail }« i { $path } importeres ikke, fordi intet i filen kommer ind som tekst, den kunne knyttes til. Kopiér den fra filen, hvis du har brug for den.
import-diagnostic-comment-replies-flattened = { $count ->
    [one] { $count } svar i { $path } henviste til en kommentar, der ikke findes i filen, så det kommer ind som en selvstændig kommentar.
   *[other] { $count } svar i { $path } henviste til en kommentar, der ikke findes i filen, så de kommer ind som selvstændige kommentarer.
}
import-diagnostic-prose-not-verbatim = { $count ->
    [one] { $count } afsnit i { $path } kunne ikke overføres med formateringen præcis som skrevet, så det kommer ind som almindelig tekst. Sammenlign det med originalen.
   *[other] { $count } afsnit i { $path } kunne ikke overføres med formateringen præcis som skrevet, så de kommer ind som almindelig tekst. Sammenlign dem med originalen.
}
import-diagnostic-styled-spaces-not-carried = { $count ->
    [one] I { $path } var { $count } stykke tomt mellemrum understreget eller gennemstreget, f.eks. en linje til at udfylde. Skribisto bevarer ingen af de to streger på rene mellemrum, så det kommer ind som almindelige mellemrum, eller slet ikke, hvor det begynder eller afslutter et afsnit.
   *[other] I { $path } var { $count } stykker tomt mellemrum understreget eller gennemstreget, f.eks. linjer til at udfylde. Skribisto bevarer ingen af de to streger på rene mellemrum, så de kommer ind som almindelige mellemrum, eller slet ikke, hvor de begynder eller afslutter et afsnit.
}
# List items nested deeper than the importer keeps, placed at the deepest level it keeps.
# $count is how many items; $limit is that deepest level, a number.
import-diagnostic-list-nesting-flattened = { $count ->
    [one] { $count } listepunkt i { $path } var indlejret mere end { $limit } niveauer dybt. Det kommer ind på niveau { $limit }, det dybeste Skribisto bevarer, med ord og formatering.
   *[other] { $count } listepunkter i { $path } var indlejret mere end { $limit } niveauer dybt. De kommer ind på niveau { $limit }, det dybeste Skribisto bevarer, med ord og formatering.
}
# Tables too lopsided to complete into a grid, one row far wider than many short ones,
# which arrive as the paragraphs of their cells. $count is how many tables.
import-diagnostic-table-read-as-paragraphs = { $count ->
    [one] { $count } tabel i { $path } har en række, der er langt bredere end resten, for skæv til at blive sat op som et gitter. Dens celler kommer ind som afsnit, det ene efter det andet, med ord og formatering.
   *[other] { $count } tabeller i { $path } har en række, der er langt bredere end resten, for skæv til at blive sat op som et gitter. Deres celler kommer ind som afsnit, det ene efter det andet, med ord og formatering.
}
# Runs of spaces longer than the importer keeps, cut to that length. $count is how many
# runs; $limit is the most spaces a run keeps, a number.
import-diagnostic-spaces-shortened = { $count ->
    [one] { $count } række mellemrum i { $path } var længere end { $limit } mellemrum og kommer ind forkortet til { $limit }. Ordene omkring den er der alle sammen.
   *[other] { $count } rækker mellemrum i { $path } var længere end { $limit } mellemrum og kommer ind forkortet til { $limit }. Ordene omkring dem er der alle sammen.
}
import-diagnostic-epigraph-not-carried = »{ $title }« indledes af et motto, men en { $kind } kan ikke rumme et. Citatet bevares i stedet øverst i teksten.
import-diagnostic-epigraph-placement-ambiguous = Et motto står mellem »{ $title }« og »{ $below }« og kunne indlede begge. Det blev givet til »{ $title }«, hvor et motto normalt hører til.
import-document-diagnostics = { $errors ->
    [0] { $warnings ->
            [one] 1 ting, du bør vide
           *[other] { $warnings } ting, du bør vide
        }
   *[other] { $errors ->
            [one] 1 fil kunne ikke læses
           *[other] { $errors } filer kunne ikke læses
        }
}
import-document-diagnostics-none = Intet at rapportere.

# ── Versions dock ──
versions-title = Versioner
versions-scope-synopsis = Synopsis
versions-scope-prose = Tekst
versions-loading = Gennemgår dine sikkerhedskopier…
versions-empty = Ingen tidligere version af dette endnu
versions-error = Dine sikkerhedskopier kunne ikke læses. Intet er gået tabt, men listen er måske ufuldstændig
versions-did-not-exist = Fandtes endnu ikke { $date }
versions-deleted-after = Slettet et stykke tid efter { $date }
versions-unreadable = { $count ->
    [one] { $count } sikkerhedskopi kunne ikke læses
   *[other] { $count } sikkerhedskopier kunne ikke læses
}
# Counts what the project's own history dropped, which is why it names it: a
# backup may still hold one of those states, and then it is in the list above.
versions-thinned = { $count ->
    [one] Ældre versioner tyndes ud med alderen. Projektets egen historik har allerede smidt { $count } tidligere tilstand af denne tekst ud.
   *[other] Ældre versioner tyndes ud med alderen. Projektets egen historik har allerede smidt { $count } tidligere tilstande af denne tekst ud.
}
versions-source-backup = Fra en sikkerhedskopi
versions-source-project = Fra projektets egen historik
versions-list-caption = Én post pr. ændring, ikke pr. sikkerhedskopi
versions-pick-a-version = Vælg en version for at se, hvad der er ændret
versions-earliest = Den tidligste registrerede version. Der er intet ældre at sammenligne den med.
versions-no-change = Intet er ændret i denne del af elementet
versions-formatting-only = Kun formateringen er ændret her. Ordene er de samme
versions-show-unchanged = Vis uændrede
versions-hide-unchanged = Skjul uændrede
versions-next-change = Næste ændring
versions-near = nær »{ $text }«
versions-words-added = { $count ->
    [one] { $count } ord tilføjet
   *[other] { $count } ord tilføjet
}
versions-words-removed = { $count ->
    [one] { $count } ord fjernet
   *[other] { $count } ord fjernet
}
versions-blocks-moved = { $count ->
    [one] { $count } afsnit flyttet
   *[other] { $count } afsnit flyttet
}
# Names the backup, not the version: a pin is held against a file path, so it
# keeps the whole snapshot this row's text was read out of. See versions-pin-note.
versions-pin = Fastgør den sikkerhedskopi, denne version kommer fra. Automatisk oprydning sletter den aldrig
versions-unpin = Frigør den sikkerhedskopi, denne version kommer fra. Automatisk oprydning kan slette den igen
versions-pinned-only = Vis kun fastgjorte versioner
versions-pin-note = Kun versioner fra en sikkerhedskopi kan fastgøres. En fastgørelse bevarer en fil, og projektets egen historik er ikke en fil.
versions-pinned-empty = Intet her er fastgjort endnu
# The same sentence with the reason, for a list that actually holds a version no
# pin can reach. On an all-backup list the reason is true and irrelevant, and
# reads as an explanation for an emptiness it did not cause.
versions-pinned-empty-log = Intet her er fastgjort endnu. Kun versioner fra en sikkerhedskopi kan fastgøres. En fastgørelse bevarer en fil, og projektets egen historik er ikke en fil.
versions-range-filter = Vis kun versioner mellem to datoer
versions-filtered-empty = Ingen version matcher de filtre, du har valgt
versions-clear-filters = Ryd filtrene
# Also used by the Timeline band's filter row, like versions-error above it.
versions-last-30-days = Seneste 30 dage
versions-restore-button = Gendan denne version
versions-restore-confirm-title = Erstat denne tekst med versionen fra { $date }?
versions-restore-confirm-text = Det, du har nu, erstattes af den tekst, elementet havde { $date }.
versions-restore-confirm-with-comments = Det, du har nu, erstattes af den tekst, elementet havde { $date }. { $count ->
    [one] { $count } kommentar er forankret i den nuværende tekst og kan blive forældreløs.
   *[other] { $count } kommentarer er forankret i den nuværende tekst og kan blive forældreløse.
}
versions-restore-confirm-undo-note = Der tages først en sikkerhedskopi, og Ctrl+Z fortryder det hele i ét trin.
versions-restored-toast = Versionen fra { $date } er gendannet
versions-undo = Fortryd
versions-restore-row-gone = Elementet findes ikke længere i dette projekt
versions-restore-no-home = Elementet har skiftet type siden da, og den gamle tekst har ingen plads i det
versions-restore-no-safety-copy = Sikkerhedskopien blev ikke taget, så intet er ændret
versions-restore-failed = Gendannelsen mislykkedes: { $error }
versions-restore-backup-busy = En sikkerhedskopiering er allerede i gang. Prøv igen om lidt
versions-restore-in-backup-file = Du ser på en sikkerhedskopifil; åbn selve projektet for at gendanne i det
versions-restore-no-project = Intet projekt er åbent
# ── Bringing a deleted row back ──
versions-recreate-button = Hent det tilbage…
versions-recreate-picker-title = Hvor skal det placeres?
versions-recreate-picker-empty = Projektet har intet ringbind at placere det i
versions-recreate-picker-confirm = Hent det tilbage hertil
versions-recreate-picker-cancel = Annullér
versions-recreate-untitled = dette element
versions-recreate-confirm-title = Hent »{ $item }« tilbage?
versions-recreate-confirm-text = Det føjes til { $destination } med den tekst, det havde { $date }.
# NOT "Ctrl+Z": that undoes the focused editor's *document*, and this creates a
# binder row. The way back is the Undo on the toast, as it is for trash and
# comments. See versions-recreated-toast.
versions-recreate-confirm-undo-note = Fortryd i den meddelelse, der følger, fjerner det straks igen.
versions-recreated-toast = »{ $item }« er tilbage i dit projekt
versions-recreated-partial-toast = { $count ->
    [one] »{ $item }« er tilbage, men en af dets tekster kunne ikke læses
   *[other] »{ $item }« er tilbage, men { $count } af dets tekster kunne ikke læses
}
versions-recreate-already-here = Elementet findes allerede i dit projekt
versions-recreate-no-destination = Vælg et sted i ringbindet at placere det
versions-recreate-unreadable = Sikkerhedskopien kunne ikke læses, så intet blev tilføjet
versions-recreate-failed = Det kunne ikke hentes tilbage: { $error }
versions-changed-percent = { $percent } % af dette er ændret
versions-hidden-paragraphs = { $count ->
    [one] … { $count } uændret afsnit …
   *[other] … { $count } uændrede afsnit …
}

# ── Timeline band ──
timeline-title = Gå tilbage i tiden
timeline-coverage = { $count ->
    [one] { $count } version registreret, tilbage til { $oldest }
   *[other] { $count } versioner registreret, tilbage til { $oldest }
}
timeline-loading = Gennemgår dit projekts fortid…
timeline-empty = Der er endnu ikke registreret nogen version af dette projekt
timeline-no-changes = Intet er ændret siden da
timeline-no-text-changes = Ingen tekst er ændret siden da
timeline-slider-label = Registreret version
timeline-series-name = Projektets størrelse
timeline-bars-caption = Hver søjle er en registreret version, lige så høj som projektet var dengang. Den fremhævede er den, du ser på.
timeline-bars-caption-periods = For mange versioner til at vise dem én for én, så hver søjle er en { $period }, lige så høj som projektet var ved dens slutning.
timeline-unit-hour = time
timeline-unit-day = dag
timeline-unit-week = uge
timeline-unit-month = måned
timeline-range-filter = Vis kun versioner mellem to datoer
timeline-range-empty = Der blev ikke registreret nogen version i det tidsrum
timeline-open-period = Åbn denne periode
timeline-show-all = Vis hele historikken
timeline-changed-since = { $count ->
    [one] { $count } element er forskelligt mellem { $date } og dit projekt nu
   *[other] { $count } elementer er forskellige mellem { $date } og dit projekt nu
}
timeline-kind-added = Skrevet siden
timeline-kind-removed = Ikke længere i projektet
timeline-kind-changed = Redigeret siden
timeline-kind-moved = Flyttet siden
timeline-not-yet-written = Dette fandtes endnu ikke på det tidspunkt
timeline-no-text-of-its-own = Dette har ingen tekst selv. Det er en overskrift for det, der ligger inden i det
timeline-reader-close = Luk
timeline-reader-stamp = Som det var { $date }
timeline-reader-loading = Åbner den registrerede version…
timeline-reader-unreadable = Denne registrering kunne ikke læses. Sikkerhedskopien er måske flyttet, slettet eller ligger på et drev, der ikke er tilsluttet.
timeline-reader-compared = Som det var { $date }, sat over for det, der står nu
timeline-reader-diff-legend = Gennemstreget er forsvundet siden; understreget er tilføjet.
timeline-reader-view-label = Hvad der skal vises
timeline-reader-view-diff = Ændringer
timeline-reader-view-text = Tekst
timeline-reader-deleted = Dette findes ikke længere i dit projekt. Du kan læse det og kopiere det ud herfra.
timeline-prose-only-record = Dette punkt kommer fra projektets egen historik, som kun gemmer tekst og intet andet, så det, der er slettet eller flyttet siden, kan ikke vises. Vælg en sikkerhedskopi til det.

# The Settings tree's section for pages an extension contributed.
settings-sec-extensions = Udvidelser

# Writing games pane: self-imposed drafting constraints
settings-group-games-forward = Altid fremad
settings-games-forward-toggle = Spil »Altid fremad«
settings-games-forward-hint =
    Mens du spiller, kan intet af det, du har skrevet, tages tilbage: Backspace, Delete,
    Klip, træk og slip og Fortryd er slået fra på de flader, du vælger nedenfor.
    Du kan stadig skrive, sætte ind, formatere og bevæge dig frit rundt. Udkastet vokser kun.
settings-games-session-warning =
    Valget gælder kun denne session og gemmes aldrig. Når du lukker projektet eller afslutter
    Skribisto, slutter spillet altid, og alt, hvad du skrev, mens du spillede, kan
    fortrydes igen fra det øjeblik.
settings-group-games-scope = Hvor det gælder
settings-games-in-prose = Manuskriptets tekst
settings-games-in-synopsis = Synopser
settings-games-scope-hint =
    Kommentarer, fodnoter og titler fryses aldrig: det er dér, du noterer den rettelse,
    du lige har forbudt dig selv at lave.
settings-games-inert-warning =
    »Altid fremad« er slået til, men gælder ingen steder. Markér mindst én flade ovenfor,
    ellers ændrer det intet, mens du skriver.

# Writing games dock (leading rail) + the status-bar warning while a game is on
games-title = Skrivelege
games-forward-name = Altid fremad
games-forward-blurb =
    Skriv udkast uden at tage noget tilbage. Sletning, klip og fortryd er slået fra,
    mens du spiller. Skriv den næste sætning i stedet for at rette den forrige.
games-forward-playing = Spiller: sletning er slået fra
games-forward-idle = Spiller ikke
games-session-note = Slutter, når du lukker projektet.
games-scope-prose-and-synopsis = Gælder din tekst og dine synopser.
games-scope-prose = Gælder din tekst.
games-scope-synopsis = Gælder dine synopser.
games-scope-nothing = Gælder endnu ingen steder. Vælg en flade i Indstillinger.
games-settings-link = Indstillinger for skrivelege…
statusbar-games-forward = Altid fremad
statusbar-games-forward-tooltip =
    »Altid fremad« er slået til: sletning, klip og fortryd er slået fra, mens du
    skriver udkast. Klik for at stoppe spillet.
# ── Word / character targets ──────────────────────────────────────────────────
# The shared vocabulary every surface that shows a target uses: the Inspector, the
# Overview column, the status bar, a container page and the Distribute preview. Plurals
# select on the bare number (`$g` / `$n`) while the visible figure is the grouped string,
# because a count printed with thin spaces is no longer a number Fluent can select on.
goal-progress-words = { $g ->
    [one] { $count } af { $goal } ord
   *[other] { $count } af { $goal } ord
}
goal-progress-characters = { $g ->
    [one] { $count } af { $goal } tegn
   *[other] { $count } af { $goal } tegn
}
goal-count-words = { $n ->
    [one] { $count } ord
   *[other] { $count } ord
}
goal-count-characters = { $n ->
    [one] { $count } tegn
   *[other] { $count } tegn
}
goal-unit-words = Ord
goal-unit-characters = Tegn

# The Inspector's per-item target.
inspector-goal = Mål
inspector-goal-none = Intet mål

# The Overview's target column, and the note on a row the export leaves out.
overview-col-goal = Mål
overview-excluded-from-export = Udeladt af eksporten, så det tæller ikke med i nogen sum

# A container's own page: what the targets set inside it add up to. Deliberately not a
# target itself, and worded so it cannot be read as one.
goal-subtree-total-words = { $count ->
    [one] { $items } mål herinde giver tilsammen { $words } ord
   *[other] { $items } mål herinde giver tilsammen { $words } ord
}
goal-subtree-total-characters = { $count ->
    [one] { $items } mål herinde giver tilsammen { $words } tegn
   *[other] { $items } mål herinde giver tilsammen { $words } tegn
}

# The New Work panel's counting-unit picker, beside the language it is seeded from.
new-work-goal-unit = Tæl i
new-work-goal-unit-hint = Bruges til alle mål for ord eller tegn i dette projekt. Kan ændres senere.

# Settings ▸ Work ▸ Structure: the project's counting unit, and the warning shown before
# switching it. No conversion happens, in either direction.
settings-group-goal-unit = Mål
settings-goal-unit-switch-title = Skift tælleenhed
settings-goal-unit-switch-text = De mål og milepæle, der allerede er sat for projektet, blev angivet i { $from }. Skifter du til { $to }, konverteres de ikke, så alle eksisterende tal vil nu blive læst som { $to }.
settings-goal-unit-switch-informative = Intet går tabt. Skift tilbage når som helst for at få den oprindelige tolkning igen, og opdatér derefter de mål, du vil beholde.
settings-goal-unit-switch-confirm = Skift

# Distribute: sharing a container's target across the pieces inside it.
goal-distribute-action = Fordel…
distribute-title = Fordel målet
distribute-empty = Der er intet herinde at fordele målet på.
distribute-weight = Fordel efter
distribute-weight-length = Efter længde
distribute-weight-rows = Efter antal dele
distribute-weight-even = Ligeligt
distribute-overwrite = Erstat mål, der allerede er sat
distribute-col-item = Element
distribute-col-current = Nu
distribute-col-proposed = Efter
distribute-total = Tilsammen giver de { $total } mod et mål på { $goal }.
distribute-over-budget = De mål, der allerede er sat herinde, giver { $over } mere end beholderens eget mål. Hæv dets mål, sænk deres, eller erstat dem.
distribute-apply = Fordel
distribute-cancel = Annullér

# Milestones on the Book's pace: a dated waypoint, of one of two kinds.
milestone-target-gone = Målet er slettet
milestone-no-target = Intet mål
milestone-add = Tilføj
milestone-add-label = Hvad der skal være nået

# The writing-plan summary shown once when a project with an active plan opens.
pace-summary-title = Hvor bogen står
pace-summary-remaining = { $words } tilbage
pace-summary-open = Åbn planen
pace-summary-close = Luk
pace-summary-dont-show = Vis ikke dette ved åbning
# Work menu row that opens the same card on demand, greyed out when the project has
# no active writing plan.
pace-summary-menu = Skriveplan…

# ── Help ─────────────────────────────────────────────────────────────────────
# The Help window (crates/teksilo_ui/src/help/), its table of contents and the
# keyboard-shortcuts sheet. Topic *bodies* are not here: they are Djot documents
# under crates/teksilo_ui/help/<locale>/, one file per topic per locale.
menu-help-topics = Hjælpe&emner
menu-help-shortcuts = &Tastaturgenveje…
menu-help-website = Skribisto på &nettet
menu-help-report = &Rapportér et problem…
menu-help-check-updates = Søg efter op&dateringer
menu-command-palette = &Kommandopalet…

# The Learn pane's rows in the Launcher. Separate keys from the Help menu's rows
# on purpose: a menu label carries an `&` mnemonic, and a Button renders that
# ampersand literally ("Help &Topics"), which is what shipped for one build.
learn-help-topics = Hjælpeemner
learn-shortcuts = Tastaturgenveje
learn-website = Skribisto på nettet

help-window-title = Hjælp
help-filter-topics = Filtrér emner
help-no-matching-topic = Intet emne matcher.
help-topic-missing = Dette emne er ikke længere tilgængeligt.
help-back = Tilbage
help-not-translated = Denne side er endnu ikke oversat, så den vises på engelsk.

help-section-getting-started = Kom i gang
help-section-writing = Skrivning
help-section-reviewing = Gennemsyn
help-section-exchanging = Få ord ind og ud
help-section-keeping = Pas på dit arbejde
help-section-extensions = Udvidelser

help-topic-getting-started = Dit første projekt
help-topic-writing-model = Hvordan en bog er bygget op
help-topic-drafts-and-old-versions = Gem et ældre udkast
help-topic-goals-and-pace = Mål og tempo
help-topic-comments = Kommentarer
help-topic-round-trip = Send din bog til en læser
help-topic-export = Eksport
help-topic-import-documents = Import af dokumenter
help-topic-import-projects = Overfør et helt projekt
help-topic-import-scrivener = Skift fra Scrivener
help-topic-backups-and-versions = Sikkerhedskopier og versioner

help-shortcuts-title = Tastaturgenveje
help-shortcuts-filter = Filtrér genveje
help-shortcuts-no-matches = Ingen genvej matcher.
help-shortcuts-rebind = Skift genveje…
help-shortcuts-close = Luk

command-palette-placeholder = Skriv en kommando

# ── Shortcut names ───────────────────────────────────────────────────────────
# The user-visible name of every rebindable `Shortcut` (`Shortcut::name`,
# teksilo-core), reactive to a locale switch via `LocalizedString`/
# `Prop<String>`. Read by Settings ▸ Keymap, the Help ▸ Keyboard shortcuts
# sheet and the command palette. Kept in lockstep with the `Shortcut::new(...)`
# calls in `app/commands/*.rs`, `binder/dock.rs` and `welcome/panel.rs`. Where
# the same command already carries a menu label above, that exact wording is
# reused here so the menu and the shortcut list agree.
shortcut-name-binder-duplicate = Duplikér
shortcut-name-comments-add = Tilføj kommentar
shortcut-name-comments-add-paragraph = Kommentér afsnit
shortcut-name-spellcheck-toggle = Kontrollér stavning
shortcut-name-editor-tab-close = Luk fane
shortcut-name-editor-tab-pin = Fastgør eller frigør fane
shortcut-name-editor-save = Gem
shortcut-name-work-export = Eksportér…
shortcut-name-work-new = Nyt projekt
shortcut-name-work-open = Åbn projekt
shortcut-name-window-new = Nyt vindue
shortcut-name-work-close = Luk projekt
shortcut-name-work-import-scrivener = Importér Scrivener-projekt…
shortcut-name-work-import-scrivener-report = Vis rapporten fra Scrivener-importen
shortcut-name-app-settings = Indstillinger
shortcut-name-app-quit = Afslut
shortcut-name-editor-insert-footnote = Indsæt fodnote
shortcut-name-format-scene-break = Indsæt sceneskift
shortcut-name-format-major-scene-break = Indsæt stort sceneskift
shortcut-name-format-link = Indsæt link
shortcut-name-format-mark-nbsp = Indsæt hårdt mellemrum
shortcut-name-format-mark-nbhyphen = Indsæt hård bindestreg
shortcut-name-format-mark-nnbsp = Indsæt smalt hårdt mellemrum
shortcut-name-format-mark-zwsp = Indsæt nulbredde-mellemrum
shortcut-name-go-next = Næste
shortcut-name-go-prev = Forrige
shortcut-name-go-to = Gå til
shortcut-name-outline-toggle = Vis eller skjul disposition
shortcut-name-preview-toggle = Vis eller skjul forhåndsvisningsbåndet
shortcut-name-view-fullscreen = Fuld skærm til/fra
shortcut-name-view-focus-mode = Distraktionsfri tilstand til/fra
shortcut-name-editor-size-increase = Forstør tekst
shortcut-name-editor-size-decrease = Formindsk tekst
shortcut-name-editor-size-reset = Nulstil tekststørrelse
shortcut-name-editor-find = Find
shortcut-name-editor-replace = Erstat
shortcut-name-editor-find-next = Næste forekomst
shortcut-name-editor-find-prev = Forrige forekomst
shortcut-name-search-show = Søg i projekt
shortcut-name-search-replace = Erstat i projekt
shortcut-name-outline-open-to-side = Åbn ved siden af
shortcut-name-help-topics = Hjælpeemner
shortcut-name-help-shortcuts = Tastaturgenveje
shortcut-name-help-website = Skribisto på nettet
shortcut-name-help-report = Rapportér et problem
shortcut-name-help-check-updates = Søg efter opdateringer
# Raised only by the Help menu command, which the reader asked for.
update-check-behind = Version { $version } er tilgængelig.
update-check-current = Du bruger den nyeste version.
update-check-failed = Der kunne ikke søges efter en nyere version. { $error }
shortcut-name-command-palette = Kommandopalet
help-section-reference = Begreber

# Titles for the glossary entries whose concept is not something you can create, so
# there is no "＋ Create" label to reuse. See help.rs::concept_topics.
help-concept-scene-break = Sceneskift
help-concept-major-scene-break = Stort sceneskift
help-concept-find-in-prose = Søg i teksten
help-concept-story-bible = Historiebibel
help-concept-goal-unit = Ord eller tegn
help-concept-goal-progress = Fremskridt
help-concept-manuscript-words = Hvad der tæller som manuskriptet
help-concept-exportable = Udeladt af eksporten
help-concept-distribute = Fordeling af et mål
help-concept-subtree-total = Hvad målene herinde giver tilsammen
help-concept-milestone = Milepæl
help-concept-pace-plan = Tempoplan

# Glossary titles for the feature-concept web. See help.rs::concept_topics.
help-concept-tag = Mærke
help-concept-label = Etiket
help-concept-point-of-view = Synsvinkel
help-concept-epigraph = Motto
help-concept-footnote = Fodnote
help-concept-chapter-mode = Kapitlets form
help-concept-comment = Kommentar
help-concept-backup = Sikkerhedskopi
help-concept-version = Version
help-concept-trash = Papirkurv
help-concept-spellcheck = Stavekontrol
help-concept-search-replace = Søg og erstat
help-concept-note-template = Noteskabelon
help-concept-story-bible-entry = Opslag i historiebiblen
help-concept-text-replacement = Teksterstatning
help-concept-smart-punctuation = Smart tegnsætning
help-concept-export-style = Eksporttypografi
help-concept-round-trip-marks = Returmarkører

# The epigraph's attribution control. See tabs/shared/panes.rs::attribution_control.
epigraph-mark-attribution = Kildelinje
epigraph-mark-attribution-tip = Markér linjen med markøren som citatets kilde, så den udskrives som en kildeangivelse
## Margin lane: the strip beside the scrollbar that maps a document
margin-lane-name = Margenmærker
margin-lane-provider-comments = Kommentarer
margin-lane-provider-story-bible = Opslag i historiebiblen
margin-lane-provider-story-bible-hint = I en notes visning I teksten: hvor opslaget nævnes, og de scener, der fortælles gennem dets øjne. Markerer også navnene i selve teksten.
margin-lane-mark-point-of-view = Fortalt herfra
margin-lane-mark-named-here = Nævnt her
margin-lane-provider-comments-hint = Hvor en note er knyttet til teksten
margin-lane-provider-search = Søgeresultater
margin-lane-provider-search-hint = Alle resultater for det, du sidst søgte efter, uanset hvor i dokumentet de er
margin-lane-provider-boundaries = Hvor hvert dokument begynder
margin-lane-provider-boundaries-hint = En streg øverst i hver scene i et forløb, så du kan se, hvor du er
margin-lane-provider-spelling = Stavning
margin-lane-provider-spelling-hint = Hvor stavekontrollen har markeret et ord. Slået fra, medmindre du beder om det: det viser en maskines mening om din tekst
margin-lane-spelling = { $word }, mulig stavefejl
margin-lane-search-hit = { $text }, forekomst { $index } af { $total }
margin-lane-search-current = { $text }, forekomst { $index } af { $total }, den du står ved
margin-lane-boundary = Begyndelsen af { $title }
margin-lane-boundary-untitled = et dokument uden titel
settings-page-margin-lane = Margenmærker
settings-desc-margin-lane = Feltet ved siden af rullepanelet, og hvad det viser
settings-margin-lane-enabled = Vis margenfeltet
settings-margin-lane-enabled-hint = Samme kontakt som Vis ▸ Margenmærker.
settings-margin-lane-enabled-more = Det siger aldrig, at noget er forkert: det viser dig, hvor tingene er, og overlader det til dig, hvad du får ud af det.
settings-group-margin-lane-marks = Hvad det markerer
settings-margin-lane-no-providers = Intet markerer feltet endnu.
settings-group-margin-lane-texture = Dialogtekstur
settings-margin-lane-texture = Vis dialogteksturen
settings-margin-lane-texture-hint = Én bjælke pr. afsnit: hvor langt det er, og hvor meget af det der er replikker.
settings-margin-lane-texture-more = Måles ikke for sprog uden en fastlagt konvention; der viser bjælken kun længden.
settings-group-margin-lane-surfaces = Hvor det vises
settings-margin-lane-surface-editor = Teksteditor
settings-margin-lane-surface-stream = Forløb
settings-margin-lane-surface-search-preview = Søgeforhåndsvisning
menu-margin-lane = &Margenmærker

## Statuses: the project's workflow ladder.
## A rung's name is project DATA: these are resolved once, when a ladder is seeded, and
## stored literally from then on. They are not re-translated afterwards, and the writer is
## free to rename any of them.
status-preset-drafting = Skrivefaser
status-preset-passes = Revisionsrunder
status-preset-plume = Plume Creator
status-todo = Mangler
status-draft = Udkast
status-revised = Revideret
status-final = Endelig
status-outline = Disposition
status-first-edit = 1. redigering
status-second-edit = 2. redigering
status-done = Færdig
status-plume-draft-1 = 1. udkast
status-plume-draft-2 = 2. udkast
status-plume-draft-3 = 3. udkast
status-plume-edit-1 = 1. redigering
status-plume-edit-2 = 2. redigering
status-plume-edit-3 = 3. redigering
status-plume-proofread = Korrekturlæst
status-plume-finished = Afsluttet
new-work-statuses = Arbejdsgang
new-work-statuses-hint = De faser, du fører en scene igennem. Du kan til enhver tid omdøbe dem, ændre rækkefølgen eller tilføje flere.
status-none = Ingen status
inspector-status = Status
overview-col-status = Status
overview-status-mixed = delene nedenunder er ikke enige
help-concept-status = Status
status-completion-title = Hvor bogen står
status-completion-headline = { $done } af { $total } scener er færdige
status-completion-empty = Ingen scener endnu. Dette læser manuskriptet, så det fyldes ud, mens du skriver.
status-completion-open = Hvor bogen står…

## Settings ▸ Work ▸ Statuses: the ladder editor.
## The writer owns a rung's NAME and the ladder's ORDER; the app owns its category, and
## the category owns the glyph and the colour. Only the category names below are
## translated: a rung's name is the writer's own text and stays as they typed it.
settings-page-statuses = Statusser
settings-desc-statuses = Omdøb, omordn, tilføj og fjern de faser, du fører en scene igennem
settings-statuses-add = Tilføj status
settings-statuses-add-placeholder = Navn på ny status
settings-statuses-desc =
    De faser, en scene gennemløber, i rækkefølge fra mindst færdig øverst til mest
    færdig nederst. Denne rækkefølge er det, »mindre færdig end« betyder overalt ellers
    i programmet, så sæt dem op, som din proces faktisk forløber.
settings-statuses-apply-preset = Anvend en forudindstilling…
settings-statuses-preset-applied = { $added ->
    [one] 1 status er tilføjet
   *[other] { $added } statusser er tilføjet
}
settings-statuses-preset-refused = Projektet har allerede en statusstige. Forudindstillinger udfylder kun en tom stige. Slet først de trin, du ikke vil have.
settings-statuses-duplicate = »{ $name }« findes allerede på denne stige
settings-statuses-added = »{ $name }« er tilføjet
settings-statuses-move-up = Flyt op (mindre færdig)
settings-statuses-move-down = Flyt ned (mere færdig)
settings-statuses-details-placeholder = Hvad denne fase betyder (valgfrit)
settings-statuses-delete = Slet »{ $name }«
settings-statuses-delete-in-use = { $count ->
    [one] Slet »{ $name }«: 1 element har denne status og mister den
   *[other] Slet »{ $name }«: { $count } elementer har denne status og mister den
}
settings-statuses-deleted = »{ $name }« er slettet
settings-statuses-deleted-in-use = { $count ->
    [one] »{ $name }« er slettet. 1 element har ikke længere en status.
   *[other] »{ $name }« er slettet. { $count } elementer har ikke længere en status.
}
settings-statuses-empty-title = Ingen arbejdsgang endnu
settings-statuses-empty-body = En status fortæller, hvor langt en scene er nået. Tilføj en ovenfor, eller start fra en forudindstilling.
status-category-planned = Planlagt
status-category-drafting = Skrives
status-category-needs-work = Kræver arbejde
status-category-revised = Revideret
status-category-final = Endelig

## Long-operation progress (shared::progress_text). The backend reports phase
## codes it cannot translate; these are their labels. `progress-percent` is the
## toast body on its own, `progress-percent-with-label` with the phase after it.
progress-percent = { $percent } %
progress-percent-with-label = { $percent } % · { $label }
progress-import-opening = Åbner projektet…
progress-import-outline = Læser dispositionen…
progress-import-converting = Konverterer kapitler og scener…
progress-import-writing = Skriver .skrib-filen…
progress-import-done = Færdig
progress-import-structure = Finder frem til strukturen…
progress-export-start = Starter…
progress-export-writing = Skriver…
progress-project-reading = Læser projektet…
# Settings: the open project's section title, "Work: <its title>".
settings-sec-work-titled = Projekt: { $title }

## Project-import notices (shared::import_warnings::notices): one line of the
## Plume / Manuskript warnings list each. $file, $folder, $title, $name, $field
## and $error are data (file names, titles, a parser's own message) and stay as
## they are. $container is "single-file" or "folder"; $kind is "scene", "note",
## "synopsis" or "title".
import-notice-flattened-rich-texts = { $count ->
    [one] Én tekst var indlejret dybere, end et Skribisto-projekt kan rumme; ordene blev bevaret som ren tekst uden formatering.
   *[other] { $count } tekster var indlejret dybere, end et Skribisto-projekt kan rumme; ordene blev bevaret som ren tekst uden formatering.
}
import-notice-free-text-badges = { $count ->
    [one] Ét sceneskilt ligner fri tekst snarere end en liste over statusser, så det blev bevaret som elementets etiket i stedet for at blive et mærke.
   *[other] { $count } forskellige sceneskilte ligner fri tekst snarere end en liste over statusser, så de blev bevaret som elementernes etiketter i stedet for at blive mærker.
}
import-notice-separator-text-dropped = Skillelinjen »{ $name }« indeholdt tekst, som der ikke er plads til i et Skribisto-projekt; teksten blev ikke importeret.
import-notice-separator-without-scene = Skillelinjen »{ $name }« havde ingen scene foran sig i sit kapitel at blive knyttet til, så den blev ikke importeret.
import-notice-chapter-without-scenes = Kapitlet »{ $name }« har ingen scener, så dets skillelinjer blev ikke importeret.
import-notice-rich-text-unconverted = Noget formateret tekst kunne ikke konverteres ({ $error }); den blev ikke importeret.
import-notice-unresolved-cross-link = Et link til post nr. { $number } i persongalleriet kunne ikke følges: posten mangler eller lå i papirkurven.
import-notice-read-copy = Projektets kopi { $container ->
    [single-file] som enkelt fil
   *[folder] som mappe
} blev læst.
import-notice-read-copy-dated = Projektets kopi { $container ->
    [single-file] som enkelt fil
   *[folder] som mappe
} (senest ændret { $day }) blev læst.
import-notice-original-format = Dette projekt er i Manuskripts oprindelige format fra 2016. Det blev læst i sin helhed; intet i det blev ændret.
import-notice-newer-single-file-beside = »{ $file }« er nyere end denne mappe ({ $file_day } mod { $folder_day }). Manuskript lader den gamle mappe ligge, når et projekt skifter til enkeltfilstilstand, så denne mappe kan være en forladt kopi; importerer du .msk-filen i stedet, læses den nyere.
import-notice-pickle-skipped = »{ $file }« blev ikke læst. Det er en Python-pickle, og at indlæse en sådan fra en projektfil var et sikkerhedshul i selve Manuskript (CVE-2021-35196); de indstillinger, den indeholder, er visningspræferencer, ikke skrivning.
import-notice-file-unreadable = »{ $file }« kunne ikke læses ({ $error }) og blev sprunget over; alt andet blev importeret.
import-notice-not-utf8 = »{ $file }« er ikke gyldig UTF-8; de tegn, der ikke kunne læses, blev erstattet i stedet for at standse importen.
import-notice-table-unreadable = »{ $file }« kunne ikke læses ({ $error }); indholdet blev ikke importeret.
import-notice-outline-missing = »{ $file }« mangler, så dette projekt har intet manuskript at importere.
import-notice-outline-unreadable = »{ $file }« kunne ikke læses ({ $error }); intet manuskript blev importeret.
import-notice-named-none-row = »{ $file }« begynder med en navngiven række (»{ $name }«) der, hvor Manuskript har sin tomme »none«-post. Den blev udeladt, så rækkerne efter den beholder de numre, manuskriptet bruger.
import-notice-unknown-field = »{ $file }« har et felt, som denne import ikke kender: »{ $field }«. Det blev ikke importeret.
import-notice-colorless-labels = { $count ->
    [one] »{ $file }« har en linje uden farve. Den blev importeret uden; Manuskript holder op med at indlæse projektet ved sådan en linje.
   *[other] »{ $file }« har { $count } linjer uden farve. De blev importeret uden; Manuskript holder op med at indlæse projektet ved sådan en linje.
}
import-notice-settings-unreadable = »{ $file }« kunne ikke læses ({ $error }); projektets sprog blev ikke importeret.
import-notice-revisions-unreadable = »{ $file }« kunne ikke læses ({ $error }); projektets historik blev ikke importeret, men intet andet blev berørt.
import-notice-foreign-files = { $count ->
    [one] Dette projekt indeholder en fil, der ikke hører til Manuskript-formatet og ikke blev importeret: { $files }.
   *[other] Dette projekt indeholder { $count } filer, der ikke hører til Manuskript-formatet og ikke blev importeret: { $files }.
}
import-notice-foreign-files-more = Dette projekt indeholder { $count } filer, der ikke hører til Manuskript-formatet og ikke blev importeret: { $files } og { $more ->
    [one] én mere
   *[other] { $more } mere
}.
import-notice-not-a-character-file = »{ $file }« er ikke en Manuskript-personfil og blev udeladt.
import-notice-not-a-text-item = »{ $file }« er ikke et Manuskript-tekstelement og blev udeladt af dispositionen.
import-notice-missing-blank-line = »{ $file }« har ingen tom linje efter sit hoved. Det sidste felt blev bevaret, selvom Manuskript mister det.
import-notice-folder-without-metadata = »{ $folder }« har ingen { $file }, så den blev importeret som en almindelig mappe opkaldt efter biblioteket. Manuskript springer sådan et bibliotek og alt i det over.
import-notice-rows-given-ids = { $count ->
    [one] Én række i dispositionen havde intet eget ID og fik et. Manuskript dropper sådan en række eller holder op med at indlæse projektet, når den er en mappe.
   *[other] { $count } rækker i dispositionen havde intet eget ID og fik et. Manuskript dropper sådan en række eller holder op med at indlæse projektet, når den er en mappe.
}
import-notice-plots-unreadable = »{ $file }« kunne ikke læses ({ $error }); plottene blev ikke importeret.
import-notice-world-unreadable = »{ $file }« kunne ikke læses ({ $error }); verdensposterne blev ikke importeret.
import-notice-world-not-opml = »{ $file }« er ikke et OPML-dokument; verdensposterne blev ikke importeret.
import-notice-text-flattened = Teksten i »{ $title }« er indlejret dybere, end et Skribisto-projekt kan rumme; ordene blev bevaret som ren tekst uden formatering.
import-notice-text-unconverted = Teksten i »{ $title }« kunne ikke konverteres ({ $error }); den blev bevaret præcis, som den var skrevet.
import-notice-text-without-file-name = { $kind ->
    [scene] Scenetekst
    [note] Notetekst
    [synopsis] Synopsis
   *[title] Titel
} i »{ $title }« kunne ikke få et filnavn, så den blev ikke importeret.
import-notice-unknown-label = »{ $title }« har etiket { $index }, som ikke findes i projektets etiketliste. Elementet blev importeret uden etiket.
import-notice-unknown-status = »{ $title }« har status { $index }, som ikke findes i projektets statusliste. Elementet blev importeret uden status.
import-notice-unknown-point-of-view = »{ $title }« fortælles gennem en person, der ikke er i projektets persongalleri. Synsvinklen blev ikke angivet.
import-notice-unresolved-references = Henvisningerne i teksten til »{ $title }« kunne ikke opløses ({ $error }); teksten blev bevaret, som den blev konverteret, inklusive henvisningsmarkører.
import-notice-custom-icons = { $count ->
    [one] Én række havde et brugerdefineret ikon. Skribisto giver hver række ikonet for dens type, så ikonet blev ikke importeret; intet andet ved rækken blev ændret.
   *[other] { $count } rækker havde et brugerdefineret ikon. Skribisto giver hver række ikonet for dens type, så ikonerne blev ikke importeret; intet andet ved rækkerne blev ændret.
}
import-notice-character-colors = { $count ->
    [one] Én person havde en farve ved sit navn. Skribisto farver mærker frem for rækker, så farven blev ikke importeret; selve personen blev.
   *[other] { $count } personer havde en farve ved deres navn. Skribisto farver mærker frem for rækker, så farverne blev ikke importeret; selve personerne blev alle importeret.
}
import-notice-versions-kept = { $found ->
    [one] Dette projekt havde registreret én tidligere version
   *[other] Dette projekt havde registreret { $found } tidligere versioner
}; { $kept ->
    [one] én blev bevaret
   *[other] { $kept } blev bevaret
}, udtyndet på samme måde, som Skribisto udtynder sin egen historik. De findes i panelet Versioner.
import-notice-versions-unplaced = { $count ->
    [one] Én tidligere version hører til en række, der ikke længere er i projektet, og kunne ikke placeres.
   *[other] { $count } tidligere versioner hører til en række, der ikke længere er i projektet, og kunne ikke placeres.
}

## Native file dialogs: the window title, and the label of the file-type filter.
dialog-open-work-title = Åbn Skribisto-projekt
dialog-save-as-file-title = Gem som en enkelt .skrib-fil
dialog-save-as-folder-title = Vælg en overordnet mappe til projektet
dialog-manuskript-file-title = Vælg et Manuskript-projekt
dialog-manuskript-folder-title = Vælg en Manuskript-projektmappe
dialog-plume-file-title = Vælg et Plume Creator-projekt
dialog-filter-skrib = Skribisto-projekt
dialog-filter-manuskript = Manuskript-projekt
dialog-filter-plume = Plume Creator-projekt
dialog-filter-hunspell-aff = Hunspell-affiksfil (.aff)
dialog-filter-hunspell-dic = Hunspell-ordbog (.dic)
dialog-filter-export = Eksporteret fil
# Help ▸ Keyboard shortcuts / Settings ▸ Keymap / command palette.
shortcut-name-view-margin-lane = Vis eller skjul margenmærker

## The built-in distraction-free themes' names (distraction_free::theme).
df-theme-paper = Papir
df-theme-sepia = Sepia
df-theme-night = Nat
df-theme-high-contrast = Høj kontrast
df-theme-midnight = Midnat

## Welcome ▸ Examples: the line under a bundled example's title.
welcome-example-byline = af { $author }

## Spoken by a screen reader only.
# Welcome window: the name of the section tab bar.
welcome-sections-a11y = Velkomstafsnit
# A comment's anchored span in the editor: author, plain-text preview, replies.
comments-a11y-summary-author = { $author }: { $text }
comments-a11y-summary-replies = { $text } ({ $count ->
    [one] 1 svar
   *[other] { $count } svar
})
comments-a11y-summary-author-replies = { $author }: { $text } ({ $count ->
    [one] 1 svar
   *[other] { $count } svar
})
# Margin marks beside the text: one mark, several merged into one, and the
# lane's own name when nothing more specific was given.
margin-lane-a11y-mark = { $label }, { $percent } procent inde i teksten
margin-lane-a11y-marks = { $label }, { $count } mærker, { $percent } procent inde i teksten
margin-lane-a11y-fallback = { $count ->
    [one] Margenmærker, 1 mærke
   *[other] Margenmærker, { $count } mærker
}
# Export ▸ Choose: the row standing for a binder that has no name.
export-choose-unnamed-binder = Unavngivet ringbind
# The built-in export styles named in words (the national manuscript presets
# keep their own language's name).
export-style-neutral = Neutral
export-style-manuscript-shunn = Standardmanuskript (Shunn)
export-style-ebook-clean = Enkel e-bog

## Dates. Put the parts in the order your language writes a short date; the
## parts arrive already zero-padded where that matters.
# Comment cards, the outline card's Created/Modified.
date-time-short = { $day }.{ $month }.{ $year } { $hour }.{ $minute }
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
settings-paratext-error-no-name = En struktur skal have en `name`-linje.
# Export: started with no project open (a menu raced a close).
export-error-no-project = Der er intet åbent projekt at eksportere.

shortcut-group-work = Projekt
shortcut-group-edit = Redigér
shortcut-group-view = Vis
shortcut-group-document = Dokument
shortcut-group-format = Format
shortcut-group-go = Gå
shortcut-group-tools = Værktøjer
shortcut-group-help = Hjælp

## Manuskript import: the headings of the notes it writes (a character's
## sheet, a world entry, a plot and its steps, the project details, the
## snowflake summary ladder) and the title of a project with no name. Written
## into the imported project once, in the interface language; the writer can
## rename them like any other text. Goal and Conflict reuse the note templates'
## own words.
import-manuskript-section-paragraph-summary = Resumé i ét afsnit
import-manuskript-section-full-summary = Fuldt resumé
import-manuskript-section-motivation = Motivation
import-manuskript-section-epiphany = Erkendelse
import-manuskript-section-notes = Noter
import-manuskript-section-description = Beskrivelse
import-manuskript-section-passion = Passion
import-manuskript-section-result = Resultat
import-manuskript-section-summary = Resumé
import-manuskript-section-meta = Metadata
import-manuskript-section-series = Serie
import-manuskript-section-volume = Bind
import-manuskript-section-genre = Genre
import-manuskript-section-license = Licens
import-manuskript-section-email = E-mail
# The snowflake ladder's five tellings, shortest first: one situation, one
# sentence, one paragraph, one page, the full summary.
import-manuskript-section-situation = Situation
import-manuskript-section-sentence = Én sætning
import-manuskript-section-paragraph = Ét afsnit
import-manuskript-section-page = Én side
import-manuskript-section-full = Fuldt resumé
import-manuskript-untitled-project = Importeret Manuskript-projekt

## Plume import: the story-bible groups an old (0.3 and earlier) project
## implies without naming them, and the title of a project with no name.
## Written into the imported project once, in the interface language.
import-plume-group-characters = Personer
import-plume-group-items = Genstande
import-plume-group-places = Steder
import-plume-untitled-project = Importeret Plume-projekt
