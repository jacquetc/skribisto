# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

menu-work = Projekt (&W)
menu-new-work = Nyt projekt (&N)
menu-open-work = Åbn projekt… (&O)
menu-new-window = Nyt vindue (&W)
menu-import-from = Import fra (&I)
menu-create-from = Opret fra (&C)
menu-import-plume = Plume Creator (.plume)… (&P)
menu-import-document = Dokumenter (Markdown, Word, ODT) (&D)
menu-import-manuskript = Manuskript (.msk)… (&M)
menu-export = Eksportér (&X)
menu-export-book = Eksportér bog
menu-export-part = Eksportér del
menu-export-chapter = Eksportkapitel
menu-export-scene = Eksportér scene
menu-export-note = Eksportbemærkning
menu-export-paratext = Eksportér paratekst
menu-export-folder = Eksportér mappe
menu-export-choose = Vælg plugins
menu-export-none = Åbn et dokument til eksport
menu-save = Gem (&S)
menu-save-as-file = Gem som én fil… (&F)
menu-save-as-folder = Gem som mappe… (&D)
menu-backup = Sikkerhedskopiér nu (&B)
menu-close-work = Luk projekt (&C)
menu-welcome = Velkommen. (&L)
menu-settings = Indstillinger (&E)
menu-quit = Afslut (&Q)

menu-view = Vis (&V)
menu-outline = Disposition (&O)
menu-search = Søg i projekt (&S)
menu-timeline = Gå tilbage i tiden (&G)
menu-search-preview = Søgeforhåndsvisning (&P)
menu-fullscreen = Fuldskærm (&F)
menu-focus-mode = Distraktion- fri tilstand (&D)
menu-format = Format (&O)
menu-scene-break = Indsæt scenebrud (&K)
menu-major-scene-break = Indsæt større scenebrud (&M)

menu-go = Gå til (&G)
menu-go-next-scene = Næste scene (&S)
menu-go-prev-scene = Forrige scene (&V)
menu-go-next-chapter = Næste kapitel (&C)
menu-go-prev-chapter = Forrige kapitel (&H)
menu-go-next-note = Næste note (&N)
menu-go-prev-note = Forrige note (&T)

menu-tools = Værktøjer (&T)
menu-help = Hjælp (&H)

ctx-add = Tilføj (&A)
ctx-new-item = Nyt element (&N)
ctx-new-folder = Ny mappe (&F)
ctx-rename = Omdøb (&R)
ctx-duplicate = Dupliker (&D)
ctx-import-here = Importér her (&I)
ctx-indent = Indsats (&I)
ctx-outdent = Udestående (&O)
ctx-trash = Flyt til papirkurv (&T)
ctx-open-to-side = Åbn til siden (&S)
ctx-open = Åbn (&O)
ctx-reveal-in-outline = Afsked i omrids (&L)
ctx-move-up = Flyt op (&U)
ctx-move-down = Flyt ned (&N)

ctx-tab-close = Luk (&C)
ctx-tab-close-others = Luk andre faner (&O)
ctx-tab-close-all = Luk alle faner (&L)
ctx-tab-open-to-side = Åbn til siden (&S)
ctx-tab-open-in-main = Åbn i hovedruden (&I)
ctx-tab-move-to-side = Flyt til siden (&M)
ctx-tab-move-to-main = Flyt til hovedruden (&M)
ctx-tab-move-to-new-window = Flyt ind i et nyt vindue (&N)
ctx-tab-move-window-unsaved = Gem dette projekt først: et andet vindue åbner på en fil på disk
ctx-tab-pin = Pin dette faneblad (&P)
ctx-tab-unpin = Fjern denne fane (&U)
tab-pinned-tooltip = Pyntet: Luk andre og luk alt og lad det stå åbent

create-book = Bog
create-part = Del
create-chapter = Kapitel
create-scene = Scene
create-note = Note
create-note-folder = Notér mappe
create-folder = Mappe
create-paratext = Paratekst
create-paratext-folder = Paratext- mappe
create-book-end = Slutningen af bogen
create-story-bible-entry = Story bible entry
type-book-start = Bogstart
type-text = Tekst

new-item-book = Ny bog
new-item-part = Ny del
new-item-chapter = Nyt kapitel
new-item-note = Ny note
new-item-note-folder = Ny notemappe
new-item-folder = Ny mappe
new-item-paratext = Ny paratext
new-item-paratext-folder = Ny Paratext mappe
new-item-story-bible-entry = Ny historie Bibelindgang

placement-inside = Indenfor
placement-after = efter
placement-after-parent = efter forælder
placement-top-level = øverste niveau


ctx-promote = Konvertér til (&C)
promote-chapter-folder = Kapitelmappe
promote-flat-chapter = Selvstændigt kapitel
promote-lossy-title = Ingen steder at opbevare teksten
promote-lossy-text = En { $target } har ingen steder at holde disse: { $kinds }. Flyt eller klar denne tekst først, derefter konvertere.
content-scene-text = Scene tekst
content-note-text = Bemærk tekst
content-book-title = Bogtitel
content-book-subtitle = Bogundertekst
content-part-title = Deltitel
content-chapter-title = Kapitel titel
content-epigraph-text = Motto
content-paratext-text = Paratekst
promote-blocked-title = Kapitel ikke tom
promote-blocked-text = { $count ->
    [one] Dette kapitel indeholder stadig { $count } element. Flyt eller skru det før konvertere det til et fladt kapitel.
   *[other] Dette kapitel indeholder stadig { $count } elementer. Flyt eller skru dem før konvertere det til et fladt kapitel.
}

inspector = Egenskaber
inspector-empty = Åbn en genstand for at inspicere den.
inspector-promote = Konvertér til +
statusbar-toggle-outline = Slå bindemidlet til / fra
statusbar-toggle-inspector = Slå inspektøren til og fra
statusbar-save-unsaved = Ikke gemte ændringer. Klik for at gemme
statusbar-save-saved = Alle ændringer gemt
statusbar-save-autosave = Autogem er tændt. Ændringer gemmes som du skriver
statusbar-saving = Redningsudstyr
statusbar-word-count = { $count ->
    [one] { $count } ord
   *[other] { $count } ord
}
statusbar-word-count-tooltip = Ord i scenen du laver redigering
statusbar-word-char-count = { $words ->
    [one] { $words } ord
   *[other] { $words } ord
} · { $chars ->
    [one] { $chars } tegn
   *[other] { $chars } tegn
}
session-toggle = Brænder session: start eller pause en fokuseret sprint
session-configure = Indstil sessionsens ordmål og tidsgrænse
session-configure-title = Brænder session
session-word-goal = Ordmål
session-time-limit = Tidsfrist
session-time-limit-unit = min.
session-no-goal = Intet mål
session-no-limit = Ingen grænse
session-reset = Nulstil session
session-readout = { $words } ord { $time }
session-readout-timed = { $words } ord · { $time } venstre
statusbar-focus-exit = Afslut distraktion- free mode
statusbar-focus-synopsis = Resumé
statusbar-focus-go-prev = Forrige punkt (Alt + Op)
statusbar-focus-go-next = Næste punkt på dagsordenen (Alt + Ned)

rail-settings = Indstillinger

language = Sprog
theme = Tema
english = Engelsk
french = Fransk
light = Lys
dark = Mørk
settings-text-width = Tekstbredde
settings-preview-width = Søg i forhåndsvisningsbredde
settings-autosave = Autogem til disk
settings-show-welcome = Vis starteren ved opstart
settings-show-welcome-tip = Når det er slukket, genåbner det sidste projekt i stedet.
settings-check-for-updates = Tjek for nye versioner
settings-check-for-updates-tip = En gang om dagen på højst, Skribisto spørger projektet hjemmeside, hvilken version er aktuel. Anmodningen bærer ingen identifikator, og ikke engang den version, du kører: sammenligningen sker på denne computer.

settings-title = Indstillinger
settings-close = Luk
settings-search = Søgeindstillinger
settings-reset = Nulstil til standard
settings-done = Færdig
settings-cancel = Annuller
settings-apply = Anvend
settings-ok = OK
settings-reset-confirm-title = Nulstil alle indstillinger til standard?
settings-reset-confirm-body = Gendanner denne applikation, udseende, editor og skrivning indstillinger. Dit projekt er egne indstillinger, tastatur genveje og gemte stilarter er ikke påvirket. Det kan ikke fortrydes.
settings-empty-title = Ingen indstillinger her endnu
settings-empty-hint = Dette afsnit vil få muligheder i en fremtidig opdatering.

settings-sec-appearance-behaviour = Udseende Opførsel
settings-sec-editor = Editor
settings-sec-spelling = Stavning
settings-sec-backup = Sikkerhedskopiér synkronisering
settings-sec-compile = Compile Export
settings-page-appearance = Udseende
settings-field-image-size-policy = Store billeder
settings-image-policy-ask = Spørg hver gang
settings-image-policy-keep = Behold originalen
settings-image-policy-downscale = Optimér
settings-hint-image-size-policy = Hvad skal man gøre, når et indsat billede er større end manuskriptet behøver.
settings-page-notifications = Meddelelser
settings-page-scene = Scene
settings-page-synopsis = Resumé
settings-page-notes = Noter
settings-page-editor-behavior = Editorens opførsel
settings-page-goals = Mål Word Count
settings-page-games = Skrivning spil
settings-page-corkboard = Opslagstavle
settings-page-distraction-free = Distraktion- fri
settings-page-dictionaries = Ordbøger
settings-page-autosave = Autogem
settings-page-export = Eksportér formater
settings-page-paratext = Paratekststrukturer
settings-page-keymap = Nøglekort
settings-keymap-filter = Filtergenveje

settings-desc-sec-appearance-behaviour = Hvordan selve programmet ser ud, og hvad det gør, når det starter.
settings-desc-sec-editor = Skrivende overflade: hvordan det ser ud, og hvad det gør, som du skriver.
settings-desc-sec-spelling = Stavekontrol, og ordbøgerne bag det.
settings-desc-sec-backup = Hvordan dit projekt når disken, og hvilke kopier der opbevares.
settings-desc-sec-compile = Hvad efterlader Skribisto, og i hvilken form.
settings-desc-sec-work = Indstillinger, der tilhører dette projekt og rejser i sin fil.
settings-desc-sec-extensions = Sider bidraget med udvidelser installeret her.
settings-desc-group-typography = En side pr. tekstform: skrivefelt, størrelse, linjehøjde og afstand.
settings-desc-appearance = Grænseflade sprog, tema, tekststørrelse, og opsenderen ved opstart.
settings-desc-notifications = Hver besked denne session har vist, og de handlinger, du kan spille igen.
settings-desc-scene = Hvordan scene prosa er indstillet: typeface, størrelse, linje højde, led og afstand.
settings-desc-synopsis = Hvordan synopsis ruden er indstillet, uafhængigt af manuskriptet.
settings-desc-notes = Hvordan noter er sat, uafhængigt af manuskriptet.
settings-desc-corkboard = Kortstørrelse, hvad et kort viser, og hvordan bestyrelsen arrangerer dem.
settings-desc-distraction-free = Typografi, søjlebredde og kontrolbånd til fuldskærmsskrivning.
settings-desc-distraction-free-themes = Temabiblioteket for fuldskærmsskrivning: de sendte og dine egne.
settings-desc-editor-behavior = Tekstbredde, synopsis position, skrivemaskine scrolling og pleje højdepunkt.
settings-desc-punctuation = Det typografiske hus stil nye projekter starter fra. Hvert projekt følger dette, medmindre det sætter sin egen.
settings-desc-goals = Ord og tegn mål, og hvordan ord tælles.
settings-desc-games = Restriktioner du sætter dig selv under udarbejdelsen, såsom Altid fremad.
settings-desc-spellcheck = Den eneste kontakt, der tænder stavekontrol overalt.
settings-desc-dictionaries = Installere, fjerne og gennemse stavefejl på denne maskine.
settings-desc-autosave = Om dine edits er skrevet til disk på egen hånd.
settings-desc-backup = Når kopier tages, hvor de opbevares, og hvor mange. Hvert projekt følger dette, medmindre det sætter sin egen.
settings-desc-export = De stilarter hver eksport kompilerer gennem: de afsendt dem og din egen.
settings-desc-paratext = Foran og bagside stof et nyt projekt kan starte med.
settings-desc-keymap = Alle genveje, og hvad det er bundet til.
settings-page-user = Bruger
settings-desc-user = Hvem du er, for de kommentarer du skriver.
settings-group-identity = Identitet
settings-field-user-name = Dit navn
settings-field-user-name-placeholder = Valgfrit
settings-field-user-name-hint = Tegn de kommentarer og svar, du skriver. Distinkt fra et projekt Buds forfatternavn, som er bogen Buds byline og rejser inde i filen.
settings-field-user-initials = Dine initialer
settings-field-user-initials-placeholder = Fra dit navn
settings-field-user-initials-hint = Hvad en tekstprocessor viser ved siden af din kommentar i marginen. Lad det være tomt at bruge dem, der vises, taget fra dit navn.
settings-field-user-hint = Begge er valgfrie og gælder for hvert projekt på denne computer. Ændring af dem tegn de kommentarer, du skriver næste. Kommentarer allerede skrevet holde det navn, de blev skrevet under.
settings-desc-author = Navnet, der står på dette projekt.
settings-desc-structure = Hvorvidt dette projekt er kapitler er mapper eller enkelte elementer.
settings-desc-language = Sproget dette projekt er prosa er kontrolleret imod.
settings-desc-work-backup = Dette projekt har sin egen backup politik, eller den generelle.
settings-desc-personal-dictionary = De ord dette projekt behandler som korrekt stavet.
settings-desc-tags = De farvekodede etiketter dette projekt markerer sine elementer med.
settings-desc-templates = Den note skabeloner, du kan indsætte, mens du skriver i dette projekt.
settings-desc-text-replacements = Genveje, der udvider som du skriver i dette projekt.
settings-desc-work-punctuation = De citater, piskeslag og afstand dette projekt er prosa følger.

settings-group-typography = Typografi
settings-group-writing-column = Skrivning kolonne
settings-group-writing-view = Brænder visning
settings-group-distraction-free-strip = Kontrolbånd
settings-group-theme = Tema
settings-group-language = Sprog
settings-group-startup = Opstart
settings-group-updates = Opdateringer
settings-group-autosave = Autogem
settings-field-typeface = Skrifttype
settings-field-size = Tekststørrelse

editor-size-changed-manuscript = Manuskript: { $percent }
editor-size-changed-synopsis = Synopsis tekststørrelse: { $percent }
editor-size-changed-notes = Noter tekststørrelse: { $percent }
editor-size-changed-corkboard = Korkbræt kort tekststørrelse: { $percent }
editor-size-changed-corkboard-expanded = Udvidet editor tekststørrelse: { $percent }
editor-size-changed-distraction-free = Distraktion- fri tekststørrelse: { $percent }

menu-text-size-increase = Forøg tekststørrelse (&I)
menu-text-size-decrease = Formindsk tekststørrelse (&C)
menu-text-size-reset = Nulstil tekststørrelse (&R)
settings-field-line-height = Linjehøjde
settings-field-first-line-indent = First- line led
settings-field-paragraph-spacing-before = Plads før punkt
settings-field-paragraph-spacing-after = Plads efter punkt
settings-field-column-width = Kolonnebredde
settings-distraction-free-width-hint = Gælder kun, mens distraction- free tilstand er tændt. Din Scene, Synopsis og Notes kolonne bredder er uberørt.
settings-distraction-free-title = Behold varenavnet
settings-distraction-free-word-count = Hold ordet tæller
settings-distraction-free-session = Behold skrivesession
settings-distraction-free-go-to = Behold Gå til + - knappen
settings-distraction-free-go = Hold de forrige og næste knapper
settings-distraction-free-chrome-hint = Den Exit knappen altid bliver, uanset hvad du vælger her. Det er din vej ud, hvis Escape er optaget.
settings-field-app-theme = Tema
settings-theme-light = Lys
settings-theme-dark = Mørk
settings-field-text-scale = Grænsefladens tekststørrelse
settings-field-language = Grænsefladesprog
settings-synopsis-placement = Synopsisposition
settings-synopsis-placement-none = Ingen
settings-synopsis-placement-top = Ovenfor
settings-synopsis-placement-side = Ved siden af
settings-typewriter = Skrivemaskine scrolling
settings-typewriter-tip = Holder den linje, du skriver på en fast højde, mens manuskriptet ruller under det. Ved at klikke stadig sætter pleje, hvor du klikker.
settings-typewriter-position = Linjeposition
settings-typewriter-position-top-third = Top tredje
settings-typewriter-position-middle = Mellem
settings-typewriter-position-bottom-quarter = Nederste kvartal
settings-highlight-scope = Fremhæv omegnen af omsorgen
settings-highlight-scope-none = Ingen
settings-highlight-scope-sentence = Dommen
settings-highlight-scope-paragraph = Afsnit
settings-highlight-scope-tip-none = Intet er skygget, som du skriver.
settings-highlight-scope-tip-sentence = Skyg den sætning, du skriver, så det skiller sig ud fra dem omkring det.
settings-highlight-scope-tip-paragraph = Shade hele det afsnit, du skriver, for at holde den passage, du arbejder på i betragtning.
settings-group-container-views = Containervisning
settings-remember-view = Husk den sidste visning for hver type element
settings-remember-view-tip = Åbn en beholder på den visning, du sidst brugte til den type
settings-remember-view-tip-more =
    En bog, del og kapitel hver tilbyder flere visninger (sin egen side, den fulde
    manuskript, den fulde synopsis). Tænd dette og hver type genåbner på
    se du sidst valgte for det, fx skifte et kapitel til Full Chapter og
    næste kapitel du åbner viser Full Chapter også. Hver type husker sin egen opfattelse.
settings-group-counting = Ordoptælling
settings-counting-auto = Automatisk (efter sprog)
settings-counting-whitespace = Split på mellemrum
settings-counting-unicode-words = Unicode- ord
settings-counting-cjk-hybrid = CJK- aware (per karakter)
settings-counting-hint = Automatisk tæller kinesisk og japansk efter karakter og alle andre sprog efter ord. Ændre det kun, hvis tællingen ser forkert for dit sprog.
settings-group-goals-display = Vis
settings-show-characters = Vis tegntællingen i statuslinjen
settings-autosave-hint = Ændringer er skrevet til disk automatisk, som du skriver.

settings-sec-work = Projekt
settings-page-structure = Struktur
settings-page-author = Forfatter
settings-field-author-name = Forfatternavn
settings-field-author-placeholder = Valgfrit
settings-field-author-hint = Udseende på den kompilerede titelside og i eksporterede fil metadata. Lad det være blankt for at udelade det. Dette er bogen Buds byline. Navnet dine kommentarer er signeret med er i Indstillingsbruger.
settings-group-chapters = Kapitel
tidy-titles-title = Tideoverskrifter
tidy-titles-none = Intet kapitel eller en del af titlen gentager blot sit eget nummer.
tidy-titles-lead = { $count ->
        [one] Et kapitel eller en del har kun sit eget nummer.
       *[other] { $count } kapitler og dele har kun deres eget nummer.
    }
tidy-titles-explain = Rydning disse titler efterlader hver enkelt navngivet efter det nummer, bogen allerede kender, det samme nummer eksport udskrifter. Intet andet ændrer, og en fortryde sætter dem alle tilbage.
menu-document-tidy-titles = Tideoverskrifter (&Y)
settings-group-numbering = Nummerering
settings-number-chapters = Antal kapitler og dele
settings-number-chapters-tip = Kapitler og dele bærer et nummer, der er beregnet fra, hvor de sidder i bogen, vist ved siden af deres titel her, og trykt af eksporten.
settings-number-chapters-tip-more = Nummeret er aldrig gemt i titlen, så det forbliver rigtigt, når du ombestille, indsætte eller slette. Slå dette fra og eksport udskriver titler alene, uanset eksport stil beder om. For at efterlade et kapitel ud (en prolog, en interlude), bruge Inspektør Numbering switch i stedet: der holder det i bogen, men stopper det at tage et nummer.
settings-part-resets-chapter = Genstart kapitel numre på hver del
settings-part-resets-chapter-tip = Off som standard: kapitler køre lige på tværs af de dele af en bog, så Buddy Part To Buddy åbner på kapitel Elleve.
settings-part-resets-chapter-tip-more = Det er den sædvanlige handelspraksis, og hvad en læser forventer. Tænd den for en bog, hvis dele skal læses som separate bind, hvor hver enkelt åbner på kapitel 1.
settings-chapter-flat = Flade kapitler
settings-chapter-flat-hint = På: et kapitel er et enkelt element. Du skriver ind i det, og det holder ingen scener. Off: et kapitel er en mappe. Du stadig skrive ind i det, men det kan holde scener så godt. Nye kapitler følger denne indstilling; eksisterende dem konvertere via Promote.

settings-styles-builtin = Bygge-i stilarter
settings-styles-user = Mine stilarter
settings-styles-builtin-badge = Bygning-in
settings-styles-duplicate = Dupliker
settings-styles-edit = Rediger
settings-styles-delete = Slet
settings-styles-import = Importér…
settings-styles-export = Eksportér…
settings-styles-copy-suffix = (kopi)
settings-styles-json-filter = Eksportér stil
settings-styles-editor-title = Redigér stil
settings-styles-editor-none = Vælg en brugerdefineret stil til at redigere det, eller duplikér en build- in one.
settings-styles-imported = Importeret stil
settings-styles-import-failed = Kunne ikke importere stil
settings-styles-exported = Udførsel af stil
settings-styles-export-failed = Kunne ikke eksportere stil
settings-styles-field-name = Navn
settings-styles-field-chapters = Kapitelsager
settings-styles-field-parts = Deloverskrifter
settings-styles-field-scene-break = Sceneskift
settings-styles-field-major-scene-break = Større scenebrud
settings-styles-field-spacing = Linjeafstand
settings-styles-field-justify = Begrund tekst
settings-styles-field-notes = Inkludér noter
settings-styles-field-synopses = Inkludér synopser
settings-styles-field-scene-titles = Inkludér scene- titler
settings-styles-field-epigraphs = Inkludér epigrafer
settings-styles-field-paratexts = Inkludér paratekter
settings-styles-field-epigraph-placement = Epigraph- position
settings-styles-epigraph-after = Efter titlen
settings-styles-epigraph-before = Før titlen
settings-styles-field-footnotes = Medtag fodnoter
settings-styles-field-footnote-numbering = Nummerering af fodnoter
settings-styles-footnote-numbering-continuous = Løbende
settings-styles-footnote-numbering-per-chapter = Genstart hvert kapitel
settings-styles-footnote-numbering-per-book = Genstart hver bog
settings-styles-field-images = Billeder
menu-image = Billede (&I)
settings-styles-images-beside = Ved siden af dokumentet
settings-styles-images-embed = Inden i dokumentet
settings-styles-images-omit = Bland dig udenom
settings-styles-group-round-trip = Sender til en editor
settings-styles-field-comments = Inkludér kommentarer
settings-styles-field-round-trip-marks = Inkludér markeringer for rundkørsel
settings-styles-round-trip-hint = DOCX og ODT kun. Markører er usynlige identifikatorer, der lader en returnerende fil opdatere dette projekt i stedet for at lande ved siden af det som en anden kopi.
settings-styles-group-pages = Sider
settings-styles-field-cover = Åbn med omslaget
settings-styles-field-word-count = Ordet tæller på titelsiden
settings-styles-field-page-books = Ny side i hver bog
settings-styles-field-page-parts = Ny side i hver del
settings-styles-field-page-chapters = Ny side i hvert kapitel
settings-styles-field-page-paratexts = Ny side på hver paratext
settings-styles-heading-none = Ingen overskrift
settings-styles-heading-numbered = Antal
settings-styles-heading-title = Kun titel
settings-styles-heading-both = Antal + titel
settings-styles-break-blank = Blank linje
settings-styles-break-none = Ingen
settings-styles-spacing-single = Enkelt
settings-styles-spacing-onehalf = 1½
settings-styles-spacing-double = Dobbelt

welcome-title = Velkommen til Skribisto
welcome-version = Version { $version }
welcome-update-available = Version { $version } er tilgængelig
welcome-update-tip = Åbner downloadsiden i din browser.
welcome-search = Søgeprojekter
welcome-open = Åbn
welcome-new-work = Nyt projekt
welcome-create-from = Opret fra plugins
welcome-recent-works = Nylige projekter
welcome-empty-recents = Ingen nye projekter endnu.
welcome-no-matches = Intet nyligt projekt matcher din søgning.
welcome-learn-soon = Guider og tips kommer snart.
welcome-about-blurb = Skribisto, en rust + Teksilo omskrivning af skrive app.
welcome-tagline = *Et roligt sted at skrive lange ting.*
welcome-github = GitHub
welcome-discord = Discord
nav-works = Projekter
nav-examples = Eksempler
nav-learn = Lær
nav-about = Om

synopsis = Resumé
epigraph = Motto
pane-manuscript = Manuskript
corkboard = Opslagstavle
overview = Oversigt

corkboard-card-count = { $count ->
    [one] { $count } kort
   *[other] { $count } kort
}
corkboard-child-count = { $count ->
    [one] { $count } konto
   *[other] { $count } poster
}
corkboard-view-nested = Nøgne
corkboard-view-flat = Flad
corkboard-layout-hint = Nested viser en container Buds direkte børn. Åbn et mappekort til at gå ind i det. Flat viser hver scene i hele beholderen på én gang.
corkboard-card-size = Kortstørrelse
corkboard-search-placeholder = Filterkort
corkboard-empty-title = Intet her endnu.
corkboard-empty-hint = Brug en ny over for at tilføje den første.
corkboard-new = Ny
corkboard-show-card-numbers = Nummerér kortene
corkboard-modal-size = Udvidet editorstørrelse
corkboard-scope-hint = Gælder for alle åbne bundkort, i hvert projekt.
corkboard-sort-manuscript = Manuskriptordre
corkboard-sort-title-asc = Afsnit A
corkboard-sort-title-desc = Afsnit Z I A
corkboard-card-number = Kort { $number }

duplicate = Dupliker
duplicate-n = { $count ->
    [one] Duplikat { $count } kort
   *[other] Duplikat { $count } kort
}
set-label-n = { $count ->
    [one] Sæt etiket på { $count } kort
   *[other] Sæt etiket på { $count } kort
}
move-to-trash-n = { $count ->
    [one] Flyt { $count } kort til papirkurven
   *[other] Flyt { $count } kort til papirkurven
}
reveal-in-outline = Afkald i omrids

corkboard-move-to = Flyt til @ title: group
corkboard-move-to-n = { $count ->
    [one] Flyt { $count } kort til
   *[other] Flyt { $count } kort til
}
corkboard-move-picker-title = Flyt til @ title: group
corkboard-move-picker-empty = Ingen bindemidler endnu. Opret en først.
corkboard-move-picker-cancel = Annuller
corkboard-move-here = Flyt her
corkboard-moved-ok = { $count ->
    [one] { $count } kort flyttet
   *[other] { $count } kort flyttet
}
corkboard-move-into-self = En container kan ikke flyttes ind i sig selv. Vælg en destination uden for den.
corkboard-move-failed = De kort kunne ikke flyttes der.

overview-col-title = Titel
overview-col-type = Type
overview-col-label = Kort beskrivelse
overview-col-tags = Mærker
overview-col-books = Bøger
overview-col-own-words = Ord
overview-col-total-words = I alt
overview-row-count = { $count ->
    [one] { $count } konto
   *[other] { $count } poster
}
overview-search-placeholder = Filterelementer
overview-expand-all = Udvid alle
overview-collapse-all = Kollaps alle
overview-table-label = Indhold
overview-empty-title = Intet her endnu.
overview-gone-title = Beholderen er væk.
overview-gone-hint = Den blev flyttet til papirkurven. Gendanne det, eller lukke denne fane.
overview-empty-hint = Brug en ny over for at tilføje den første.
corkboard-grid-label = Korkkort
corkboard-rename-field = Omdøb element
corkboard-expand-synopsis = Udvid synopsis
corkboard-synopsis-modal-title = Resumé
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
corkboard-show-word-count = Vis ord tæller på kort
text-heading = Tekst
no-content = Dette punkt har intet redigerbart indhold.
untitled = Uden titel
placeholder-title = Titel…
placeholder-subtitle = Subtitle
placeholder-chapter-title = Kapitel titel
split-editor = Opdel editor
close-split-view = Luk splitvisning
drop-open-here = Åbn her
drop-open-to-side = Åbn til siden

segment-chapter = Kapitel
segment-part = Del
segment-book = Bog
segment-pace = Skrivetempo
segment-notes = Noter
segment-story-bible = Historiebibel
segment-note-own = Note
segment-note-details = Detaljer
segment-note-in-prose = I prose
pace-placeholder = Pace planneren vises her.
pace-empty-title = Planlæg denne bog Buds tempo
pace-empty-body = Sæt et ord mål og en deadline, og Skribisto Projekter ud af den daglige tempo for at få dig der.
pace-start-planning = Start planlægning
pace-section-schedule = Skema
pace-section-progress = Fremskridt
pace-advancement = Fremskyndelse
pace-goal = Ordmål
pace-deadline = Deadline
pace-active = Pace aktiv
pace-day-mon = Man
pace-day-tue = Tir
pace-day-wed = Ons
pace-day-thu = Tor
pace-day-fri = Fre
pace-day-sat = Lø
pace-day-sun = Sol
pace-card-written = ord skrevet
pace-card-of-goal = af målet
pace-card-rate = ord / skrift dag
pace-card-days-left = skriftligt dage tilbage
pace-card-streak = dagstribe
pace-card-ahead = ord foran tempoet
pace-card-behind = ord bag tempoet
pace-charts-empty = Fremskridt diagrammer vises her, når dit ord tæller er registreret på en spare.
pace-chart-progression = Ord skrevet vs. mål
pace-chart-words-per-day = Ord pr. dag
pace-series-actual = Faktisk
pace-series-target = Mål
pace-series-words-per-day = Krydderier i form af frø
pace-daily-target-line = Lige tempo: { $count ->
    [one] { $count } ord / dag
   *[other] { $count } ord / dag
}
pace-section-holidays = Ferie
pace-section-milestones = Milepæle
pace-holidays-none = Ingen ferier, hver dag tæller.
pace-holiday-label = Ferienavn
pace-add-holiday = Tilføj
pace-remove = Fjern
pace-milestones-none = Ingen milepæle endnu. Sæt en på en del eller kapitel i inspektøren.
full-chapter = Fuldstændigt kapitel
full-part = Fuldstændig del
full-book = Fuld bog
full-synopsis = Fuld synopsis
rename = Omdøb
set-label = Sæt etiket
insert-scene = Indsæt scene
insert-chapter = Indsæt kapitel
split-scene = Opdel scene
move-up = Flyt op
move-down = Flyt ned
merge-with-previous = Sammenfletning med tidligere
move-to-trash = Flyt til papirkurv
rename-chapter = Omdøb kapitel
add-scene = Tilføj ny scene
add-chapter = Tilføj nyt kapitel
new-scene-title = Ny scene
placeholder-scene-name = Scenenenavn
menu-cut = Klip
menu-copy = Kopiér
menu-paste = Indsæt
menu-paste-unformatted = Indsæt uden formatering
menu-select-all = Markér alt
format-bold = Fed
format-italic = Kursiv
format-underline = Underlinje
format-strikethrough = Gennemstreget

binder = Dokumenttræ
no-work = Intet projekt indlæst
no-recent-works = Ingen nyere projekter
switcher-open-section = Aktuelt åbent
switcher-recent-section = Seneste
switcher-this-window = dette vindue
open-project-title = Åbne projekter
open-project-question = Hvordan vil du åbne { $title }?
open-in-new-window = Åbn i nyt vindue
open-here = Åbn her

binder-all = Alle bindere
binder-show-all = Vis alle bindemidler
binder-new = Nyt bindemiddel
binder-default-name = Nyt bindemiddel
binder-item-count = { $count ->
    [one] { $count } konto
   *[other] { $count } poster
}
binder-search-placeholder = Filtrér omridset
binder-search-scope = Søg i alle bindemidler
binder-trash-confirm-title = Flytte bindemiddel til affald?
binder-trash-confirm-text = { $name } og alle dens elementer vil blive flyttet til papirkurven.

dialog-rename = Omdøb
dialog-new-binder = Nyt bindemiddel
dialog-set-label = Sæt etiket
dialog-new-scene = Ny scene
close-work-question = Gem ændringer inden projektet lukkes?
quit-question = Gem ændringer inden afslutning?
quit-save-work-question = Gem ændringer til { $title } før du stopper?
unsaved-changes = Dette projekt har ændringer, der ikke er gemt.
new-work-unsaved-question = Gem ændringer før du opretter et nyt projekt?
open-work-unsaved-question = Gem ændringer før du åbner et andet projekt?
switch-backup-discard-title = Kassér ændringer til denne sikkerhedskopi?
switch-backup-discard-text = Ændringer til en sikkerhedskopi kan ikke gemmes til det. Brug Gem som eller gendanne for at holde dem, eller kassere dem og åbne det andet projekt.
switch-save-failed = Projektet kunne ikke reddes, så det blev erstattet: { $error }
switch-save-not-started = Projektet kunne ikke reddes, så det blev ikke erstattet.
close-save-failed = Projektet kunne ikke reddes, så det blev lukket: { $error }
close-save-not-started = Projektet kunne ikke reddes, så det var lukket.
save-not-started = Projektet kunne ikke reddes.

could-not-open-work = Kunne ikke åbne { $file }
could-not-open-work-too-new = • { $file } • har brug for en nyere Skribisto
could-not-open-work-too-new-detail = Reddet af Skribisto format { $written_by }; åbning det kræver format { $requires } eller nyere, og denne bygning understøtter op til format { $supported }. Opdatering Skribisto for at åbne det.
could-not-open-work-importing = Der er ved at blive skrevet { $file }.
could-not-open-work-importing-detail = En import skriver over dette projekt og er ikke færdig. Åbn det, når importen er færdig: åbnes nu, vil det vise projektet, at importen er ved at erstatte, og dens næste besparelse vil skrive det projekt tilbage over importen.
could-not-open-example = Kunne ikke åbne eksempel: { $error }
could-not-open-link = Kunne ikke åbne { $url }: { $error }
could-not-create-work = Kunne ikke oprette projekt: { $error }
saving-as-file = Lagring som { $target }
saving-as-folder = Lagring som { $target } / Name
saved-as = Reddet til { $target }
save-error = Kunne ikke gemme: { $error }
save-flush-failed = Redningen gik igennem, men disse dokumenter kunne ikke gives til det og er stadig ikke gemt: { $titles } ({ $error })
backup-error = Kunne ikke sikkerhedskopiere: { $error }
backing-up = Bakker op
backup-nothing-open = Intet projekt er åbent for backup.
backup-already-running = En backup er allerede i gang.
backup-complete = Backup komplet ({ $ok } gemt, { $skipped } allerede nuværende)
backup-partial = Backup færdig: { $ok } gemt, { $failed ->
    [one] { $failed } destination mislykkedes
   *[other] { $failed } destinationer mislykkedes
}
backup-no-destination-title = Ingen sikkerhedskopiering tilgængelig
backup-no-destination-text = Ingen af de konfigurerede backupdestinationer kan nås (for eksempel kan et eksternt drev frakobles). Sæt det ind og prøv igen, eller afslut uden sikkerhedskopiering.
backup-failed-close-title = Sikkerhedskopien kunne ikke gemmes
backup-failed-close-text = Ingen sikkerhedskopi kunne skrives før lukning. Hver destination mislykkedes (drevet kan være blevet fjernet, eller det kan være fuld eller skrivebeskyttet). Fix det og prøve igen, eller afslutte uden sikkerhedskopiering.

backup-choice-title = Backup fil
backup-choice-heading = Du har åbnet en sikkerhedskopi
backup-choice-subtitle = Dette er en point-in-time backup af et projekt.
backup-choice-subtitle-dated = Backup taget { $date }.
backup-choice-body = Du kan åbne den og redigere frit, men ændringer kan kun holdes med Gem som. Den oprindelige projektfil er ikke rørt. Eller gendanne dette projekt til præcis denne sikkerhedskopi.
backup-choice-open = Åbn sikkerhedskopi
backup-choice-restore = Genopret dette projekt til dette punkt
backup-choice-not-a-backup = Nej, åbn den normalt
backup-banner-title = Backup kopi: ændringer kan ikke gemmes her
backup-banner-description = Brug Gem som at holde dine redigeringer i en ny fil, eller gendanne for at erstatte det oprindelige projekt med denne sikkerhedskopi.
backup-banner-restore = Gendan…
backup-banner-save-as = Gem så hurtigt som muligt
backup-restore-original-missing = Kan ikke finde det oprindelige projekt til at gendanne over. Brug Gem som til at holde denne backup som et nyt projekt i stedet.
backup-restore-close-elsewhere-title = Projekt åbent i et andet vindue
backup-restore-close-elsewhere-text = Det projekt, du skal genoprette over er åbent i et andet vindue. Vær venlig at lukke det der først, så prøv igen.
backup-restore-focus-window = Fokusér på det vindue
backup-restore-confirm-title = Gendanne denne backup?
backup-restore-confirm-text = Den nuværende version af projektet vil blive kopieret til side som en sikkerhedskopi, før den erstattes med denne.
backup-restore-confirm-ok = Gendan
backup-restore-error = Kunne ikke genoprette: { $error }
backup-restored-ok = Projektet er genoprettet.
backup-restored-with-safety = Projekt restaureret. Din tidligere version blev gemt til { $path }.
close-backup-discard-title = Kassér ændringer til denne sikkerhedskopi?
close-backup-discard-text = Ændringer til en sikkerhedskopi kan ikke gemmes til det. Brug Gem som for at holde dem, eller kassér og luk.
quit-backup-discard-title = Kassere ændringer og stoppe?
quit-backup-discard-work-question = Kassere ændringer til { $title } og stoppe?
quit-backup-discard-text = Ændringer til en sikkerhedskopi kan ikke gemmes til det. Brug Gem som for at holde dem, eller kassere og afslutte.
backup-nudge-text = Der oprettes ingen sikkerhedskopier til dette projekt.
backup-nudge-action = Opsætning af sikkerhedskopier

menu-backups-list = Sikkerhedskopier… (&K)
backups-title = Sikkerhedskopier
backups-loading = Scanner efter sikkerhedskopier
backups-empty = Ingen sikkerhedskopier fundet til dette projekt endnu.
backups-open = Åbn
backups-reveal = Aflivning
backups-delete = Slet denne sikkerhedskopi
backups-delete-confirm-title = Slet denne backup?
backups-delete-confirm-text = "{ $name }" slettes permanent. Det kan ikke fortrydes.
backups-delete-error = Kunne ikke slette sikkerhedskopi: { $error }
backups-refresh = Genopfrisk
backups-close = Luk

settings-page-backup = Sikkerhedskopieringsstandard
settings-page-work-backup = Sikkerhedskopier
settings-backup-general-title = Standardindstillinger for sikkerhedskopi
settings-backup-work-title = Backup til dette projekt
settings-backup-inherit = Brug de generelle sikkerhedskopier
settings-backup-inheriting = Dette projekt bruger de generelle backup indstillinger.
settings-backup-none-hint = Ingen automatiske sikkerhedskopier er sat op (hver udløser er slukket).
settings-backup-last = Sidste sikkerhedskopi: { $date }
settings-backup-last-never = Ingen sikkerhedskopier endnu.
settings-backup-open-list = Åbn sikkerhedskopier listName
settings-backup-triggers = Hvornår du skal bakke op
settings-backup-on-close = Bak op, når projektet lukkes
settings-backup-on-open = Bak op, når projektet åbnes
settings-backup-interval = Tilbage med jævne mellemrum, hver
settings-backup-destinations = Backup destinationer
settings-backup-default-location = Standardplacering
settings-backup-usage-since = { $count ->
    [one] { $count } { $size } · ældste { $oldest }
   *[other] { $count } Backup · { $size } · ældste { $oldest }
}
settings-backup-usage = { $count ->
    [one] { $count } backup { $size }
   *[other] { $count } backup { $size }
}
settings-backup-usage-empty = Intet holdt her endnu
settings-backup-usage-measuring = Måleenheder
settings-backup-reveal-root = Vis mappe
settings-backup-dest-none = Ingen destinationer. Backup gemmes ved siden af projektet.
settings-backup-dest-remove = Fjern
settings-backup-dest-add = Tilføj mappe- pluginName
settings-backup-dest-refresh = Genopfrisk
settings-backup-retention = Hvor mange at beholde
settings-backup-retention-tiered = Træet
settings-backup-retention-keep-n = Behold sidste N
settings-backup-retention-tip = Hvor gamle backups er beskåret.
settings-backup-retention-tip-more = Træt holder en backup per time i en dag, per dag i en uge, per uge i en måned, og per måned efter, så nyere historie forbliver tæt og gamle historie thins ud. Hold sidste N simpelthen holder N nyeste kopier. I begge tilstande de nyeste kopier (minimum nedenfor) altid holdes.
settings-backup-gfs-hourly = Timeløn (sidste 24 timer)
settings-backup-gfs-daily = Daglig (sidste uge)
settings-backup-gfs-weekly = Ugentlig (sidste måned)
settings-backup-gfs-monthly = Månedlig
settings-backup-keep-n = Nummer at beholde
settings-backup-min-keep = Behold i det mindste altid
settings-backup-dedup = Spring en sikkerhedskopi over når intet har ændret sig

new-work-title = Nyt projekt
new-work-close = Luk
new-work-name = Projektnavn
new-work-name-placeholder = Uden titel
new-work-author = Forfatter
new-work-author-placeholder = Valgfrit
new-work-format = Format
new-work-single-file = Enkeltfil
new-work-bundle = Samlet fil
new-work-convert-later = Du kan konvertere mellem formater senere.
new-work-location = Sted
new-work-will-create = Vil skabe
new-work-language = Standardsprog
new-work-language-hint = Anvendes til nye tekster stavekontrol. Hver tekst kan skiftes til et andet sprog.
new-work-template = Skabelon
new-work-template-none = Ingen
new-work-template-empty-novel = Tomme novel
new-work-template-light-novel = Lys novel
new-work-template-novel = Roman
new-work-template-novel-in-parts = Novel i dele
new-work-template-notebook = Notesbog
new-work-cancel = Annuller
new-work-create = Opret projekt
new-work-step-details = Detaljer
new-work-step-language = Sprogstruktur
new-work-step-template = Skabelon
new-work-back = Tilbage
new-work-next = Næste
new-work-documents-title = Nyt projekt fra dokumenter
new-work-step-import = Importér
new-work-create-and-import = Opret importfil @ action: inmenu
new-work-documents-next-title = Dine dokumenter kommer næste
new-work-documents-next-body = Oprettelse af projektet åbner import guiden over det, hvor du vælger de dokumenter, gennemgå strukturen Skribisto læser fra dem, og vælge, hvor det lander. Intet er skrevet ind i projektet, indtil du bekræfter det der.
new-work-documents-no-template = Dette projekt starter tomt med vilje: ingen skabelon, så de importerede dokumenter er det eneste i det.
new-work-documents-chapter-scene-hint = Gælder for de kapitler, som importen skaber. Du kan ændre det senere i projektets indstillinger.
new-work-single-file-desc = Et .skrib arkiv (zip). Bærbart, nemt at sikkerhedskopiere.
new-work-bundle-desc = En mappe med hver tekst aktiv. Venligere til version kontrol.
new-work-template-none-count = tomt bindemiddel
new-work-template-empty-novel-count = 1 kapitel
new-work-template-light-novel-count = 15 kapitler
new-work-template-novel-count = 20 kapitler
new-work-template-novel-in-parts-count = 3 dele, 24 kapitler
new-work-template-notebook-count = free-form-noter
new-work-chapter-scene = Flade kapitler
new-work-chapter-scene-tip = Hvert kapitel er et enkelt emne, du skriver direkte ind i, uden scener under det. Lad dette ligge til det klassiske layout, hvor et kapitel er en mappe: du skriver lige ind i det også, men det kan holde scener så godt.
new-work-chapter-scene-tip-more = Du skriver ind i et kapitel begge veje. Den eneste forskel er, om det kan *indeholde* scener. Skribisto Buds bindertræ er organisatorisk kun, så begge layouts kompilere til den samme bog, kan du blande dem frit, og Promote konverterer et kapitel mellem de to uden at miste et ord.
new-work-name-required = Indtast et navn for projektet
new-work-name-invalid = Dette navn har ingen brugbare tegn
new-work-location-required = Vælg et sted
new-work-location-missing = Denne mappe eksisterer ikke
new-work-location-not-folder = Denne sti er ikke en mappe
new-work-location-readonly = Denne mappe er ikke skrivbar
new-work-target-exists = Et projekt med dette navn er allerede i denne mappe. Oprettelse af projektet vil spørge, før du erstatter det
new-work-target-is-folder = En mappe med dette navn er allerede i dette sted. Vælg et andet navn
new-work-overwrite-title = Erstat det eksisterende projekt?
new-work-overwrite-text = Der findes allerede { $name } i denne mappe. Erstat den med det nye projekt?
new-work-target-open-title = Dette projekt er åbent
new-work-target-open-text = "{ $name }Der er åbent i Skribisto. Oprettelse af et nyt projekt i stedet ville erstatte det projekt, du arbejder på, og din næste besparelse ville skrive dette projekt tilbage over den nye. Vælg et andet navn eller mappe, eller luk projektet først.
target-importing-title = En import skriver dette projekt
target-importing-text = En import er stadig skriftligt { $name }. Vent, indtil det er færdig, eller vælg et andet navn.
import-plume-busy-title = Et andet Plume Creator-projekt er ved at blive importeret
import-plume-busy-text = Plume Creator projekter er importeret en ad gangen. Vent, indtil den igangværende import er færdig, eller annullere det, så starte denne igen.
import-manuskript-busy-title = Et andet Manuskript-projekt er ved at blive importeret
import-manuskript-busy-text = Manuskript projekter er importeret en ad gangen. Vent, indtil den igangværende import er færdig, eller annullere det, så starte denne igen.
backup-restore-importing-text = En import skriver stadig { $name }, projektet denne sikkerhedskopi ville blive restaureret over. Genoprette sikkerhedskopi, når importen er færdig.

new-work-manuscript = Manuskript
new-work-notes = Noter
new-work-research = Forskning
new-work-notebook = Notesbog
new-work-scene = Scene
new-work-note = Note
new-work-front-matter = Forstof
new-work-back-matter = Bagstof
new-work-paratext = Bogstruktur
new-work-paratext-none = Ingen struktur
new-work-paratext-hint = Forsiden og bagsiden stof en tradition åbner og lukker en bog med. Du kan flytte, omdøbe eller slette noget af det bagefter.
new-work-tags = Mærker
new-work-tags-none = Ingen mærker
new-work-tags-hint = En startpalet til at mærke tegn, steder og resten. Valgfri, og hvert mærke kan omdøbes, genkendes eller slettes bagefter.
new-work-note-templates = Note skabeloner
new-work-note-templates-none = Ingen skabeloner
new-work-note-templates-hint = Formen en historie- bible note starter i. Valgfri, og hver skabelon kan redigeres eller slettes bagefter.
new-work-characters = Personer
new-work-places = Steder

import-plume-title = Import Plume Creator projekt
import-plume-close = Luk
import-plume-source = Plume-projektet
import-plume-source-hint = Vælg en .plume eller .plume_backup fil (enhver Plume Creator version).
import-plume-location = Destinationsmappe
import-plume-name = Filnavn
import-plume-name-placeholder = Projektnavn
import-plume-will-create = Vil skabe
import-plume-trash-warning = Trassed / slettede elementer migreres ikke.
import-plume-cancel = Annuller
import-plume-import = Importér
import-plume-source-required = Vælg en Plume projektfil
import-plume-source-missing = Denne fil eksisterer ikke
import-plume-source-not-file = Denne sti er ikke en fil
import-plume-location-required = Vælg en destinationsmappe
import-plume-location-missing = Denne mappe eksisterer ikke
import-plume-location-not-folder = Denne sti er ikke en mappe
import-plume-location-readonly = Denne mappe er ikke skrivbar
import-plume-name-required = Indtast et filnavn
import-plume-name-exists = En fil med dette navn findes allerede her. Import bekræfter overskrivning
import-plume-overwrite-title = Erstat eksisterende fil?
import-plume-overwrite-text = Der findes allerede { $name }. Erstat det med det importerede projekt?
import-plume-manuscript-binder = Manuskript
import-plume-story-bible-binder = Historiebibel
import-plume-progress-title = Import af Plume-projektet
import-plume-cancel-import = Annuller
import-plume-cancelled = Importér annulleret
import-plume-done = Importeret { $imported ->
    [one] { $imported } konto.
   *[other] { $imported } poster.
 } { $skipped ->
    [one] { $skipped } skrottede genstande blev ikke migreret.
   *[other] { $skipped } trash items was not migrated.
}
import-plume-open-now = Åbn nu
import-plume-warnings = { $count ->
    [one] 1 ting kunne ikke importeres nøjagtigt
   *[other] { $count } ting kunne ikke importeres præcist
}
import-plume-details = Detaljer
import-plume-warnings-title = Importadvarsler
import-plume-error-title = Kunne ikke importere projektet
import-plume-nested-too-deep = Projektet blev ikke importeret: dets Mezzogiorno { $part } Mezzogiorno er indlejret mere end { $limit } niveauer dybt. Nr. Plume Creator projekt kommer tæt på, og læse det kunne gå ned Skribisto. Intet blev skrevet.
import-plume-declares-entities = Projektet blev ikke importeret: dets Mex{ $part } Mexpart erklærer XML enheder, som ingen Plume Creator projekt gør. At udvide dem kunne tage mere hukommelse, end computeren har, og crash Skribisto. Intet blev skrevet.
import-plume-error-details = Detaljer

import-manuskript-title = Import Manuskript projekt
import-manuskript-close = Luk
import-manuskript-source = Manuskript projekt
import-manuskript-source-hint = Vælg .msk filen eller projektmappen (en hvilken som helst Manuskript version).
import-manuskript-source-file = Vælg filName
import-manuskript-source-folder = Vælg mappe- pluginName
import-manuskript-location = Destinationsmappe
import-manuskript-name = Filnavn
import-manuskript-name-placeholder = Projektnavn
import-manuskript-will-create = Vil skabe
import-manuskript-cancel = Annuller
import-manuskript-import = Importér
import-manuskript-source-required = Vælg et Manuskript projekt
import-manuskript-source-missing = Denne fil eller mappe eksisterer ikke
import-manuskript-location-required = Vælg en destinationsmappe
import-manuskript-location-missing = Denne mappe eksisterer ikke
import-manuskript-location-not-folder = Denne sti er ikke en mappe
import-manuskript-location-readonly = Denne mappe er ikke skrivbar
import-manuskript-name-required = Indtast et filnavn
import-manuskript-name-exists = En fil med dette navn findes allerede her. Import bekræfter overskrivning
import-manuskript-overwrite-title = Erstat eksisterende fil?
import-manuskript-overwrite-text = Der findes allerede { $name }. Erstat det med det importerede projekt?
import-manuskript-manuscript-binder = Manuskript
import-manuskript-story-bible-binder = Historiebibel
import-manuskript-characters-group = Personer
import-manuskript-world-group = Verden
import-manuskript-plots-group = Pletter
import-manuskript-project-info-note = Projektoplysninger
import-manuskript-summary-note = Oversigt
import-manuskript-importance-minor = Mindre
import-manuskript-importance-secondary = Sekundær
import-manuskript-importance-main = Vigtigste @ action: inmenu
import-manuskript-progress-title = Import Manuskript projekt
import-manuskript-cancel-import = Annuller
import-manuskript-cancelled = Importér annulleret
import-manuskript-done = { $imported ->
    [one] { $imported } vare importeret.
   *[other] { $imported } produkter importeret.
 } { $revisions ->
    [0] { "" }
    [one] { $revisions } tidligere version kom med det.
   *[other] { $revisions } tidligere versioner kom med dem.
}
import-manuskript-open-now = Åbn nu
import-manuskript-warnings = { $count ->
    [one] 1 ting at vide om denne import
   *[other] { $count } ting at vide om denne import
}
import-manuskript-details = Detaljer
import-manuskript-warnings-title = Om denne import
import-manuskript-error-title = Kunne ikke importere projektet
import-manuskript-nested-too-deep = Projektet blev ikke importeret: dets fil{ $part }Der er indlejret over { $limit } niveauer dybt. Manuskript projektet kommer tæt på det, og læse det kunne gå ned SkribistoDer er ikke skrevet noget.
import-manuskript-folders-too-deep = Projektet blev ikke importeret: en af sine filer sidder mere end { $limit } mapper dybe. Nr. Manuskript projekt kommer tæt på, og læsning det kunne gå ned Skribisto. Intet blev skrevet.
import-manuskript-declares-entities = Projektet blev ikke importeret: dets fil er { $part } Budskaber XML enheder, som ingen Manuskript projekt gør. At udvide dem kunne tage mere hukommelse, end computeren har, og crash Skribisto. Intet blev skrevet.
import-manuskript-error-details = Detaljer

import-target-open-title = Dette projekt er åbent
import-target-open-text = "{ $name }Der er åbent i Skribisto. Importerer over det ville erstatte det projekt, du arbejder på, og din næste gemme ville skrive dette projekt tilbage over import. Vælg et andet filnavn, eller lukke projektet først.
import-warnings-announcement = { $result } { $notice }.

export-title = Eksportér
export-close = Luk
export-scope-label = Hvad?
export-format-label = Format
export-style-label = Stil
export-path-label = Fil
export-browse = BrowseName
export-show-non-exportable = Vis ikke - ekspresérbar
export-choose-empty = Intet projekt at vælge imellem.
export-cancel = Annuller
export-export = Eksportér
export-save-dialog-title = Eksportér til fil
export-section-what = Eksportér
export-section-style = Stil
export-section-destination = Bestemmelsessted
export-custom-selection = Brugervalgt markering
export-selected-count = { $count } valgt
export-chip-chapters-none = Kapitel: ingen
export-chip-chapters-numbered = Kapitel: nummereret
export-chip-chapters-title = Kapitel: Kun titel
export-chip-chapters-both = Kapitel: nummer + titel
export-chip-scene-break-glyph = Sceneskift: { $glyph }
export-chip-scene-break-blank = Scene break: blank linje
export-chip-scene-break-none = Scene pause: ingen
export-chip-major-break-glyph = Stort skift: { $glyph }
export-chip-major-break-blank = Større brud: blank linje
export-chip-major-break-none = Større brud: ingen
export-chip-major-break-same = Begge breaktier er ens
export-chip-spacing-single = Rum: enkelt
export-chip-spacing-onehalf = Rum: 1 ½
export-chip-spacing-double = Mellemrum: dobbelt
export-chip-notes-included = Noter inkluderet
export-chip-notes-excluded = Bemærkninger, der ikke er medtaget
export-format-docx = Ord
export-format-odt = LibreOffice
export-format-html = HTML
export-format-markdown = Markdown
export-format-djot = Djot
export-format-text = Tekst
export-format-latex = LaTeX
export-format-epub = EPUB
export-format-pdf = PDF
export-overwrite-title = Erstat eksisterende fil?
export-overwrite-text = - Der findes allerede { $name }.
export-progress-title = Eksporterende
export-cancelled = Eksportér annulleret
export-done = Eksporteret { $count ->
    [one] { $count } konto
   *[other] { $count } poster
}
export-open-file = Åbn
export-show-in-folder = Vis i mappe
export-error-title = Kunne ikke eksportere
export-error-details = Detaljer

backup-progress-start = Startdato
backup-progress-retention = Oprydning af gamle sikkerhedskopier
backup-progress-done = Færdig
backup-progress-destination = Bestemmelsessted { $i } af { $n }
backup-details = Detaljer
backup-issues-title = Sikkerhedskopiering
backup-failed-title = Sikkerhedskopi mislykkedes
backup-complete-prune-warning = Backup komplet ({ $ok } gemt, { $skipped } allerede nuværende). Nogle gamle sikkerhedskopier kunne ikke fjernes

search = Søg
search-query-placeholder = Søg…
search-replace-placeholder = Erstat med number
search-replace-toggle = Skift til / fra
search-replace-all = Erstat alle
search-opt-case = Match- tilfælde
search-opt-whole-word = Hele ord
search-opt-diacritics = Matchaccenter
search-scope-body = Krop
search-scope-title = Titel
search-scope-synopsis = Resumé
search-scope-label = Kort beskrivelse
search-facet-book = Bøger
search-facet-part = Dele
search-facet-chapter = Kapitel
search-facet-scene = Scener
search-facet-note = Noter
search-facet-folder = Mapper
search-tip-case = Match sag: behandle uppercase og lavercase som forskellige, så bland Elena og bland er separate kampe.
search-tip-whole-word = Hele ord: matche kun komplette ord, så cat cat er ikke fundet inde kategori cate.
search-tip-diacritics = Match accenter: behandle accented bogstaver som særskilte, så Buddcafe er ikke matche Buddcafé.
search-tip-body = Krop: søge prosa af scener og noter.
search-tip-title = Titel: søge efter titlerne på bindemiddel elementer.
search-tip-synopsis = Synopsis: søge i hvert skrive element Buds resumé.
search-tip-label = Etiket: søg i den korte note, der vises under et bindemiddel element has titel.
search-tip-comment = Kommentarer: søg teksten af kommentar tråde og deres svar. En kommentar er om manuskriptet snarere end en del af det, så det har sin egen kontakt, og en erstatning efterlader kommentar matcher unticked indtil du sætter kryds dem.
search-tip-book = Bøger: bogbeholderen og dens start- / slutmarkører.
search-tip-part = Dele: dele til delniveau.
search-tip-chapter = Kapitel: kapitler, uanset hvordan projektet gemmer dem.
search-tip-scene = Scener: de elementer, der holder din prosa.
search-tip-note = Noter: free-form noter.
search-tip-folder = Mapper: plain organisere mapper og separatorer.
search-tip-paratext = En tekst, der tilhører bogen, men ikke dens historie: et forord, en dedikation, et efterord. Aldrig talt i manuskriptet.
search-tip-preserve-case = Reserve sag: en udskiftning tager den sag, det fandt, så skal ELENA bliver MARTA og Elena bliver Marta.
search-tip-replace = Erstat: Vis erstatningsfeltet og Erstat alle.
search-error = Søgning mislykkedes: { $message }
search-no-matches = Ingen kampe
search-count =
    { $matches ->
        [one] { $matches } match
       *[other] { $matches } kampe
    } in { $items ->
        [one] { $items } dokument
       *[other] { $items } dokumenter
    }
search-count-truncated =
    { $matches ->
        [one] { $matches } match
       *[other] { $matches } kampe
    } in { $items ->
        [one] { $items } dokument
       *[other] { $items } dokumenter
    } (kun de første resultater)
search-occurrences = { $count }
search-field-body = Krop
search-field-title = Titel
search-field-synopsis = Resumé
search-field-label = Kort beskrivelse
search-field-epigraph = Motto
search-field-comment = Kommentar
search-field-comment-reply = Svar
search-field-footnote = Fodnote
search-include-in-replace = Inkludér i Erstat alle
search-collapse-all = Kollaps alle resultater
search-replace-here = Erstat dette
search-dismiss = Frafald fra resultaterne
search-undo-dismiss = Bring det sidste afviste resultat tilbage
search-replace-nothing = (ingenting)
search-replace-confirm-title = Erstatte alle tændstikker?
search-replace-confirm-text =
    Erstat { $occurrences ->
        [one] { $occurrences } forekomst
       *[other] { $occurrences } forekomster
    } of "{ $query }" with "{ $replacement }" across { $items ->
        [one] { $items } dokument
       *[other] { $items } dokumenter
    } ? Du kan fortryde dette fra anmeldelsen bagefter.
search-replace-done-title = Erstat komplet
search-replace-done =
    { $occurrences ->
        [one] { $occurrences } forekomst
       *[other] { $occurrences } forekomster
    } replaced in { $items ->
        [one] { $items } dokument
       *[other] { $items } dokumenter
    } .
search-replace-done-skipped =
    { $occurrences ->
        [one] { $occurrences } forekomst
       *[other] { $occurrences } forekomster
    } replaced in { $items ->
        [one] { $items } dokument
       *[other] { $items } dokumenter
    }. { $skipped ->
        [one] { $skipped } felt
       *[other] { $skipped } felter
    } sprunget (ændret siden søgningen).
search-replace-undo = Fortryd
search-replace-failed-title = Erstat mislykkedes
search-replace-skipped-title = Intet erstattet
search-replace-skipped-body = Antal tekster, der er ændret siden søgningen: { $fields }. Disse tekster blev ikke ændret. Søg igen for at se, hvor ordene er nu.
search-replace-undo-failed-title = Fortryd mislykkedes
search-preview = Forhåndsvisning
search-preview-empty = Vælg et resultat for at få vist det her
search-preview-no-prose = Dette match har ingen redigerbar tekst
search-preview-prompt = For at se en forhåndsvisning her, køre en søgning.
search-preview-open-search = Søg i projekt
search-preview-footnote-prompt = Denne kamp er i en fodnote af egen tekst. Åbn den i Footnotes dock for at se og redigere den.
search-preview-open-footnotes = Åbne fodnoter

find-placeholder = Find i dette dokument
find-count = { $current } af { $total }
find-no-results = Ingen resultater
find-close = Tæt find
find-previous = Forrige kamp (Shift + Enter)
find-next = Næste match (Enter)
find-opt-case = Match- tilfælde
find-opt-whole-word = Hele ord
find-replace-toggle = Skift til / fra (Ctrl + R)
find-replace-placeholder = Erstat med number
find-replace = Erstat
find-replace-all = Erstat alle
find-preserve-case = Reservekravstilfælde

dict-download-title = Download{ $name }
dict-download-done = Installeret { $name }
dict-download-failed = Kunne ikke downloade { $name }: { $error }
dict-removed = Fjernet { $name }
dict-accept-first = Acceptér licensen, før du henter { $name }
settings-dict-tab-installed = Installeret
settings-dict-tab-get-more = Få mere
settings-dict-tab-personal = Personlige ord
dict-installed-empty = Ingen ordbøger fundet på denne computer endnu.
dict-get-more-search = Søgesprog
dict-system-badge = på dit system
dict-unusable-badge = ubrugelig
dict-download-button = Download
dict-downloading = Downloader
dict-installed-label = Installeret
dict-remove = Fjern
dict-view-license = Vis licens
dict-approx-size = ~{ $size }
dict-personal-empty = Ingen personlige ord i dette projekt endnu.
dict-personal-add = Tilføj
dict-personal-placeholder = Tilføj et ord
dict-missing-toast = { $count ->
    [one] Dette projekt bruger { $count } ordbog du don 't har installeret
   *[other] Dette projekt bruger { $count } ordbøger du ikke har installeret
}
dict-missing-action = Hent ordbøger
dict-license-title = { $name } licens
dict-license-accept = Acceptér download
dict-license-cancel = Annuller
dict-license-close = Luk
dict-add-button = Tilføj ordforråd
dict-add-title = Tilføj en ordbog
dict-add-close = Luk
dict-add-name = Navn
dict-add-name-placeholder = f.eks. min latinske ordbog
dict-add-code = Sprogkode
dict-add-code-placeholder = F.eks. la eller fr- FR- x- custom
dict-add-code-hint = Det er, hvad du vælger som dokumentsprog.
dict-add-aff = Påfiks fil (.aff)
dict-add-dic = Tekstliste (.dic)
dict-add-submit = Tilføj
dict-add-cancel = Annuller
dict-add-name-required = Giv ordbogen et navn.
dict-add-code-required = Indtast en sprogkode.
dict-add-code-invalid = Brug kun bogstaver, tal og - _.
dict-add-code-reserved = Det er en building- i ordbog kode. Vælg en anden.
dict-add-file-required = Vælg en fil.
dict-add-file-missing = Denne fil findes ikke.
dict-add-code-taken = En ordbog for denne kode er allerede installeret.
dict-add-done = Tilføjet { $name }
dict-add-unusable = Disse filer er ikke en brugbar ordbog: { $error }
dict-add-failed = Kunne ikke tilføje ordbogen: { $error }
inspector-export = Eksportér
inspector-exportable = Inklusive i eksporten
ctx-number = Nummerér dette kapitel (&N)
ctx-unnumber = Nummerér ikke dette kapitel (&N)
inspector-numbering = Nummerering
inspector-numbered = Nummereret
inspector-numbered-tip = Dette kapitel indtager sin plads i bogens nummerering. Slå det fra for en prolog, en prolog eller en interlude.
inspector-numbered-tip-more = Et umummereret kapitel forbliver i bogen præcis som det var: dets overskrift, dens prosa og dens ordtal er uberørt. Det udskriver simpelthen intet nummer, og bruger ikke et op: kapitlet efter en prolog er kapitel et, ikke kapitel to. At forlade det ud af eksporten i stedet er en anden kontakt, ovenfor, og at man fjerner kapitlet helt fra bogen.
inspector-apply-to-children = Anvend på børn
inspector-milestone = Milepæl-dato
inspector-milestone-none = Ingen dato
inspector-milestone-clear = Ryd
inspector-tags = Mærker
inspector-aliases = Også kendt som
inspector-dict-language = Sprog
inspector-apply-language-to-children = Anvend sprog på børn
settings-page-language = Sprog
settings-field-dict-language = Sprog
dict-tradeoff-hint = Hvert ekstra sprog accepterer flere ord, så færre fejl er fanget.
lang-inherit-hint = Inherited: Denne scene bruger bogen eller projekt-sprog.
lang-pill-list = Sprog
lang-pill-add = Tilføj et sprog
lang-pill-remove = Fjern { $name }
lang-pill-mute = Drej stavekontrol for { $name }
lang-pill-unmute = Tænd for stavekontrol for { $name }

settings-page-personal-dictionary = Personlig ordbog
settings-user-dict-desc = Ord du tilføjer her behandles som korrekt stavet overalt i appen, og aldrig markeret igen.
settings-user-dict-add-placeholder = Tilføj et ord
settings-user-dict-add = Tilføj ord
settings-user-dict-search = Filterord
settings-user-dict-count = { $count ->
    [one] { $count } ord
   *[other] { $count } ord
}
settings-user-dict-rename = Omdøb
settings-user-dict-remove = Fjern
settings-user-dict-empty = Ingen ord endnu. Tilføj ord stavekontrollen bør ignorere, eller importere en liste.
settings-user-dict-import = Importér…
settings-user-dict-export = Eksportér…
settings-user-dict-import-tip = Tilføj ord fra en almindelig tekstfil, et ord pr. linje. Eksisterende ord gemmes.
settings-user-dict-export-tip = Gem hele listen til en almindelig tekstfil, et ord pr. linje.
settings-user-dict-txt-filter = Tekstfiler
settings-user-dict-duplicate = Allerede i ordbogen
settings-user-dict-imported = Importeret { $count ->
    [one] { $count } ord
   *[other] { $count } ord
} ({ $duplicates } allerede til stede).
settings-user-dict-import-failed = Kunne ikke læse ordlisten: { $error }
settings-user-dict-exported = Gemt { $count ->
    [one] { $count } ord.
   *[other] { $count } ord.
}
settings-user-dict-export-failed = Kunne ikke gemme ordlisten: { $error }

settings-page-text-replacements = Teksterstatninger
settings-text-repl-desc = Erstat shorthand med den fulde tekst som du skriver: Memory bliver ved den måde, du skriver en plads eller punktuation.
settings-text-repl-enable = Brug teksterstatninger i dette projekt
settings-text-repl-disabled-hint = Tænd dette for at definere kortslutning, der udvider sig som du skriver.
settings-text-repl-add = Tilføj regel
settings-text-repl-trigger-placeholder = Stenografi
settings-text-repl-replacement-placeholder = Hvad det bliver til
settings-text-repl-added = Tilføjet: { $trigger }
settings-text-repl-duplicate = Der findes allerede en regel for { $trigger }.
settings-text-repl-filter = Filterregler
settings-text-repl-count = { $n ->
    [one] 1 regel
   *[other] { $n } regler
}
settings-text-repl-row-enabled = Brug denne regel
settings-text-repl-delete = Slet reglen for { $trigger }
settings-text-repl-deleted = Udgår reglen for "{ $trigger }"
settings-text-repl-empty = Ingen regler endnu.
settings-text-repl-csv-filter = CSV filer
settings-text-repl-import = Importér…
settings-text-repl-export = Eksportér…
settings-text-repl-imported = Importeret { $added }, sprunget { $skipped }
settings-text-repl-exported = Eksporteret { $n ->
    [one] 1 regel
   *[other] { $n } regler
}

editor-menu-no-suggestions = Ingen forslag
editor-menu-add-to-dictionary = Tilføj br. { $word } br.
editor-menu-add-words-to-dictionary = Tilføj markerede ord til ordbog
editor-dict-added = Tilføjet { $word } til din ordbog.
editor-dict-added-multi = { $count ->
    [one] Tilføjet { $count } ord til din ordbog.
   *[other] Tilføjet { $count } ord til din ordbog.
}
toast-undo = Fortryd

undo-superseded-title = Dette skridt er ikke længere det sidste.
undo-superseded-body = Noget andet har ændret sig i dette projekt siden. Fortryd ville tage det tilbage i stedet, så det har været. Intet dette skridt er blevet vendt. Brug Rediger Fortryd til at træde tilbage gennem historien selv.
undo-failed = Fortryd mislykkedes: { $error }

menu-edit = Rediger (&E)
menu-edit-undo = Fortryd (&U)
menu-edit-redo = Annuller fortryd (&R)
menu-edit-undo-target = Fortryd { $target } (&U)
menu-edit-redo-target = Redo { $target } (&R)
shortcut-name-edit-undo = Fortryd
shortcut-name-edit-redo = Annuller fortryd
undo-target-typing = maskinskrivning
undo-target-project = den seneste ændring af dette projekt
prose-history-reset-title = Skrivning historik nulstilles
prose-history-reset-body = { $count ->
    [one] En åben scene blev genoprettet fra en tidligere tilstand, så dens skrive historie ikke længere gælder.
   *[other] { $count } åbne scener blev restaureret fra en tidligere tilstand, så deres skrive historie ikke længere gælder.
}
undo-target-trash = Flytning til papirkurven
undo-target-restore = gendannelse fra papirkurven
undo-target-delete-forever = sletning til gode
undo-target-replace-all = om udskiftning på tværs af projektet
undo-target-import = dokumentets import
undo-target-duplicate = duplikering
undo-target-move = Flytning
undo-target-merge = sammenfletning af to scener
undo-target-split = Opdeling af en scene
undo-target-promote = om ændring af typen
undo-target-tidy-titles = rydning af kapiteltitler
undo-target-import-tags = Importmærker
undo-target-import-templates = Importér note- skabeloner
undo-target-create = oprettelse
undo-target-remove = sletning
undo-target-rename = omdøbning
undo-target-edit = der redigerer
menu-edit-find = Find… (&F)
menu-edit-find-next = Find næste (&N)
menu-edit-find-prev = Find tidligere (&V)
menu-edit-replace = Find og erstat (&L)
menu-edit-find-in-project = Find i projektdetaljer (&J)
menu-edit-replace-in-project = Erstat i projektafdelingen (&I)
undo-frozen = Fortryd (# Altid fremad # er tændt)
redo-frozen = Redo (# Altid fremadrettet # er tændt)
menu-edit-cut = Klip (&T)
menu-edit-copy = Kopiér (&C)
menu-edit-paste = Indsæt (&P)
menu-edit-paste-plain = Indsæt uden formatering (&W)
menu-edit-select-all = Markér alt (&A)
shortcut-name-edit-cut = Klip
shortcut-name-edit-copy = Kopiér
shortcut-name-edit-paste = Indsæt
shortcut-name-edit-paste-plain = Indsæt uden formatering
shortcut-name-edit-select-all = Markér alt

titlebar-spellcheck-on = Stavekontrol er tændt. Klik for at stoppe kontrollen (F7)
titlebar-spellcheck-off = Stavekontrol er slukket. Klik for at tjekke igen (F7)
menu-spellcheck = Kontrolstavning (&C)
menu-comments = Kommentarer (&M)
settings-page-spellcheck = Stavekontrol
settings-group-spellcheck = Stavekontrol
settings-spellcheck-enabled = Tjek stavning som jeg skriver
settings-spellcheck-hint = Understøtter ord ingen installeret ordbog ved. At slå dette fra stopper al kontrol, i hvert projekt, indtil du tænder det igen. For at stoppe med kun at kontrollere et sprog, skal du fjerne dets kontrolmærke på projektets sprogfelt eller på et element.

menu-trash = Papirkurv (&T)
trash-title = Papirkurv
trash-empty-state = Affald er tomt.
trash-empty-button = Tomt affald
trash-restore = Gendan (&R)
trash-restore-to = Genopret til @ action: button (&T)
trash-delete-forever = Slet for evigt (&D)
trash-restored-ok = { $count } genoprettet.
trash-restore-error = Kunne ikke genoprette: { $error }
trash-restore-orphaned = Dette element er oprindelige sted er væk. Vælg, hvor at gendanne det.
trash-restore-no-project = Intet projekt er åbent, så der er intet at genoprette.
trash-delete-no-project = Intet projekt er åbent, så der er intet at slette.
trash-empty-confirm-title = Tøm affaldet?
trash-empty-confirm-text = { $count ->
    [one] { $count } trash entry vil blive permanent slettet.
   *[other] { $count } trash indgange vil blive slettet permanent.
} De efterlader affaldet for evigt. Undo kan stadig bringe dem tilbage indtil du gør noget andet.
trash-emptied-title = Trash tømt
trash-emptied-body = Alt i skraldespanden blev slettet permanent.
trash-empty-no-project = Intet projekt er åbent, så der er intet affald at tømme.
trash-delete-forever-confirm-title = Slet for evigt?
trash-delete-forever-confirm-text = { $count ->
    [one] { $count } post vil blive permanent slettet.
   *[other] { $count } emner vil blive permanent slettet.
} De efterlader affaldet for evigt. Undo kan stadig bringe dem tilbage indtil du gør noget andet.
trash-deleted-title = Slettet for evigt
trash-deleted-body = { $count } udgår permanent.
trash-undo = Fortryd
trash-restore-picker-title = Genopret til @ action: button
trash-restore-to-confirm-title = Gendan her?
trash-restore-to-confirm-text = Gendan { $item } til { $destination }?
trash-restore-picker-restore-here = Gendan her
trash-restore-picker-cancel = Annuller
trash-restore-picker-empty = Ingen bindemidler endnu. Opret en først.
trash-banner-title = Dette punkt er i papirkurven
trash-banner-description = Det vandt ikke vises i omridset eller eksport, indtil du gendanne det.
trash-banner-restore = Gendan…
trash-tab-tooltip = I papirkurven

settings-styles-sheet-yes = Ja
settings-styles-sheet-no = Nej
settings-styles-sheet-font = Skrifttype
settings-styles-sheet-indent = First- line led
settings-styles-sheet-para-spacing = Afstandsafstand
settings-styles-sheet-page = Sidestørrelse
settings-styles-sheet-margins = Marginer (T / R / B / L)
settings-styles-sheet-title-page = Book titelside
settings-styles-sheet-heading-language = Kurssprog
settings-styles-sheet-auto = Følger teksten
settings-styles-sheet-digits = Numre
settings-styles-sheet-direction = Tekstretning
settings-styles-sheet-formats = Formater
settings-styles-sheet-all-formats = Alle

settings-styles-editor-group = Typografiredigering
settings-styles-editor-missing = Den stil er ikke længere tilgængelig.

settings-styles-page-letter = Brev
settings-styles-digits-western = Regionskommando Vest (0 + 9)
settings-styles-digits-eastern-arabic = Regionskommando Øst-Arabisk
settings-styles-direction-ltr = Venstre mod højre
settings-styles-direction-rtl = Højre mod venstre

format-dock-title = Format
format-panel-empty = Placer din markør i en scene, note, eller synopsis for at se formatering muligheder.
format-group-marks = Tekst
format-group-block = Afsnit
format-group-lists = Lister
format-group-tables = Tabel
format-group-breaks = Scene pauser
format-superscript = Overskrift
format-subscript = Sænket skrift
format-link = Link
link-dialog-insert-title = Indsæt link
link-dialog-edit-title = Redigér link
link-dialog-text-label = Tekst
link-dialog-text-placeholder = de ord læseren ser
link-dialog-url-label = Links til
link-dialog-url-placeholder = example.com
link-dialog-insert = Indsæt
link-dialog-apply = Anvend
link-dialog-cancel = Annuller
link-dialog-remove = Fjern link
link-scheme-refused = Kun web-og e-mail-links kan åbnes. { $url } blev efterladt alene.
menu-format-link = Link (&N)
format-clear = Ryd formatering
format-heading = Udgiftsniveau
format-heading-normal = Normal tekst
format-heading-1 = Udgiftsområde 1
format-heading-2 = Udgiftsområde 2
format-heading-3 = Udgiftsområde 3
format-heading-4 = Udgiftsområde 4
format-heading-5 = Udgiftsområde 5
format-heading-6 = Udgiftsområde 6
format-align-left = Justér til venstre
format-align-center = Centralt
format-direction-rtl = Højre-til-venstre afsnit
format-blockquote = Blockcitat
format-list-bullet = Bulletsliste
format-list-numbered = Nummereret liste
format-indent = Indsats
format-outdent = Udestående
format-table-insert = Indsæt tabel
format-table-row-above = Indsæt række over
format-table-row-below = Indsæt række nedenfor
format-table-col-before = Indsæt kolonne før
format-table-col-after = Indsæt kolonne efter
format-table-row-delete = Slet række
format-table-col-delete = Slet søjle
format-table-remove = Fjern tabel

menu-format-marks-bold = Fed (&D)
menu-format-marks-italic = Kursiv (&I)
menu-format-marks-underline = Underlinje (&U)
menu-format-marks-strike = Gennemstreget (&S)
menu-format-marks-superscript = Overskrift (&P)
menu-format-marks-subscript = Sænket skrift (&B)
menu-format-marks-clear = Ryd formatering (&C)
menu-format-heading = Udgiftsområde (&H)
menu-format-heading-normal = Normal tekst (&N)
menu-format-heading-1 = Udgiftsområde 1 (&1)
menu-format-heading-2 = Udgiftsområde 2 (&2)
menu-format-heading-3 = Udgiftsområde 3 (&3)
menu-format-heading-4 = Udgiftsområde 4 (&4)
menu-format-heading-5 = Udgiftsområde 5 (&5)
menu-format-heading-6 = Udgiftsområde 6 (&6)
menu-format-alignment = Justering (&A)
menu-format-align-left = Justér til venstre (&L)
menu-format-align-center = Centralt (&C)
menu-format-direction = Retning (&E)
menu-format-direction-auto = Automatisk (&A)
menu-format-direction-ltr = Venstre mod højre (&L)
menu-format-direction-rtl = Højre mod venstre (&R)
menu-format-blockquote = Blockcitat (&Q)
menu-format-lists = Lister (&L)
menu-format-list-bullet = Skjulte liste (&B)
menu-format-list-numbered = Nummereret liste (&N)
menu-format-indent = Indsats (&I)
menu-format-outdent = Udestående (&O)
menu-format-table = Tabel (&T)
menu-format-table-insert = Indsæt tabel (&I)
menu-format-table-2x2 = 2 x 2 (&2)
menu-format-table-3x3 = 3 x 3 (&3)
menu-format-table-4x4 = 4 x 4 (&4)
menu-format-table-row-above = Indsæt række over (&V)
menu-format-table-row-below = Indsæt række nedenfor (&W)
menu-format-table-col-before = Indsæt søjle før (&L)
menu-format-table-col-after = Indsæt søjle efter (&T)
menu-format-table-row-delete = Slet række (&D)
menu-format-table-col-delete = Slet søjle (&C)
menu-format-table-remove = Fjern tabel (&R)

menu-format-typo-marks = Formatering af markering (&F)
menu-format-typo-nbsp = No- break Space (&N)
menu-format-typo-nbhyphen = Ingen-break Hyphen (&H)
menu-format-typo-soft-hyphen = Soft Hyphen (&S)
menu-format-typo-nnbsp = Small No- break Space (&R)
menu-format-typo-zwsp = Zerobredde mellemrum (&Z)
menu-format-typo-word-joiner = Word Joiner (&W)


menu-about = ca. Skribisto (&A)
about-title = Omkring Skribisto
about-version = Version { $version }
about-update-available = Version { $version } er tilgængelig
about-update-see-notes = Se hvad der ændrede sig
about-update-flathub = Flatthub holder kopien opdateret.
about-update-distro = Din distribution holder denne kopi opdateret.
about-tagline = En bog-skrive app til lang-form fiktion, skrevet i Rust med Teksilo toolkit.
about-license = Løsladt under GNU General Public License, version 3.
about-copyright = © 2026 Cyril Jacquet
about-close = Luk

native-menu-about = Omkring { $app }
native-menu-hide = Skjul { $app }
native-menu-quit = Afslut { $app }
native-menu-settings = Indstillinger…
native-menu-window = Vindue
native-menu-minimize = Minimér
native-menu-zoom = Zoom

window-title = { $title } - { $app }
window-title-numbered = { $title } - { $app } (vindue { $n })
window-title-empty = { $app }

first-run-window-title = Velkommen.
first-run-title = Oprettet { $app }
first-run-body = { $app } holder sine indstillinger adskilt fra Skribisto, så det starter tom. Dine præferencer, nylige projekter, ordbøger og vindue layout kan kopieres på tværs nu.
first-run-from = Kopiér fra
first-run-to = Kopiér til
first-run-copy-note = Intet er flyttet eller fjernet. Skribisto holder hver eneste af sine egne indstillinger, og fortsætter med at arbejde præcis som før.
first-run-import = Importér indstillinger
first-run-start-fresh = Start frisk
first-run-import-failed = Nogle indstillinger kunne ikke importeres: { $error }

settings-page-punctuation = Punktueringsfejl
settings-group-punctuation = Smart punktuering
settings-page-work-punctuation = Punktuering
settings-punctuation-override = Giv dette projekt sine egne tegnregler
settings-punctuation-override-hint = Off: projektet følger ansøgningen præference. Disse regler rejser inden for .skrib, så en co-forfatter åbner filen skriver med den samme typografi.
settings-punctuation-dashes = Drej -- til en en dash, --- til en em dash
settings-punctuation-ellipsis = Gør... til en ellipse
settings-punctuation-quotes = Krølliteringer og apostroffer
settings-quote-style = Noteringsmærker
settings-quote-style-locale = Sprogstandard
settings-quote-style-curly = Det er det samme som det her.
settings-quote-style-curly-single = English English
settings-quote-style-guillemets = - Guillemets.
settings-quote-style-low-high = Low- high
settings-punctuation-spacing = Plads før;:!?
settings-punctuation-spacing-hint = Fransk typografi sætter en smal no- break plads før;!? og en fuld en før:. Det gælder kun for tekst skrevet på fransk. Rummet inde guillemet Buddy kommer med guillemet citater selv.
settings-punctuation-sample = Dit sprog giver
settings-punctuation-app-hint = Hvad hvert projekt gør, medmindre det tager sine egne regler i Projekt Medical Punctuation.
settings-punctuation-dialogue = Åbn et afsnit, der er maskinskrevet som en dialog-dash
settings-punctuation-dialogue-hint = For sprog, der markerer tale med en streg snarere end citationstegn: fransk, spansk, russisk og andre. Det kun brande i begyndelsen af et afsnit.

statusbar-go-to = Gå til den.
go-to-placeholder = Søg i bindemidlet
go-to-no-matches = Ingen ting matcher den søgning.
menu-go-to = Gå til den. (&G)

settings-page-distraction-free-themes = Distraktionsfrie temaer
settings-themes-builtin = Bygge-i temaer
settings-themes-builtin-badge = Bygning-in
settings-themes-user = Mine temaer
settings-themes-editor-group = Redigér tema
settings-themes-editor-empty = Vælg et tema under min temaer for at redigere det.
settings-themes-use = Anvendelse
settings-themes-duplicate = Dupliker
settings-themes-edit = Rediger
settings-themes-delete = Slet
settings-themes-export = Eksportér…
settings-themes-import = Importér…
settings-themes-copy-suffix = Kopiér
settings-themes-json-filter = Tema (JSON)
settings-themes-imported = Tema importeret
settings-themes-import-failed = Kunne ikke importere dette tema
settings-themes-exported = Eksporten af tema
settings-themes-export-failed = Kunne ikke eksportere dette tema
settings-themes-low-contrast = lav kontrast
settings-themes-low-contrast-band = fremhæve skjuler teksten
settings-themes-field-name = Navn
settings-themes-field-paper = Side
settings-themes-field-ink = Tekst
settings-themes-field-general = Baggrund
settings-themes-field-widget-text = Styrestriptekst
settings-themes-field-caret-band = Fremhæv omegnen af omsorgen

statusbar-focus-settings = Distraktion- free indstillinger
statusbar-focus-manage-themes = Håndtér temaer

comments-title = Kommentarer
comments-document-title = Dette dokument
comments-empty-project = Ingen kommentarer i dette projekt endnu.
comments-empty-document = Ingen kommentarer til dette dokument.
comments-filter-all = Alle
comments-filter-open = Åbn
comments-filter-resolved = Opløst
comments-filter-orphaned = Alfoner
comments-status-open = Åbn
comments-status-resolved = Opløst
comments-status-orphaned = Mistede sin tekst
comments-orphan-snippet = (Den kommenterede tekst er væk)
comments-status-unplaced = Ingen tekstposition
comments-unplaced-snippet = (ikke forankret i nogen tekst)
comments-reply-count = { $count ->
    [one] { $count } svar
   *[other] { $count } replies
}
comments-menu-resolve = Markér som løst
comments-menu-reopen = Genopen
comments-menu-delete = Slet
comments-sort-document = I dokumentrækkefølge
comments-sort-newest = Nyeste først
comments-sort-reader = Grupperet af læsere
comments-unsigned-author = Ikke signeret
comments-agreement = { $count ->
    [one] { $count } læser
   *[other] { $count } læsere
}
comments-menu-add = Tilføj kommentar
comments-menu-add-paragraph = Kommentar til dette punkt
overview-col-comments = Kommentarer
overview-col-total-comments = Bemærkninger i alt
comments-card-placeholder = Skriv en kommentar
comments-card-unknown-author = Ukendt forfatter
comments-unsigned-toast = Dine kommentarer er ikke underskrevet: Intet navn er sat til dig på denne computer.
comments-unsigned-action = Sæt dit navn
comments-card-reply = Svar
comments-card-reply-placeholder = Svar
comments-card-actions = Kommentaraktioner
comments-reply-actions = Svar
comments-menu-delete-reply = Slet svar
comments-menu-delete-all = Slet alle kommentarer her
comments-deleted-toast = Kommentar slettet
comments-reply-deleted-toast = Svar slettet
comments-deleted-all-toast = { $count ->
    [one] { $count } kommentar slettet
   *[other] { $count } kommentarer slettet
}
comments-undo = Fortryd

analysis-segment = Analyse
analysis-scope-book = Analyserer denne bog
analysis-run = Løbeanalyse
analysis-stale = Ændret siden dette løb
analysis-not-run = Ikke analyseret endnu.
analysis-running = Læsning af manuskriptet
analysis-failed = Analysen kunne ikke afsluttes.
analysis-no-scenes = Ingen scener i denne bog endnu.

analysis-shape = Form

analysis-arrivals = Ankomst
analysis-arrivals-explainer = Hvordan tekst nåede dette projekt, mens det har været åbent. Der står, hvilken rute figurerne kom ned, og overhovedet intet om, hvem der skrev dem: En forfatter, der skriver andre steder og pastaer, har indsat, og en, der dikterer har dikteret. Der er ingen tal at sigte efter her.
analysis-arrivals-scope = Hele projektet, ikke kun denne bog.
analysis-arrivals-session = Siden du åbnede den, begynder optællingen igen.
analysis-arrivals-nothing = Der er endnu ikke kommet nogen tekst på dette møde.
analysis-arrivals-typed = Skrivet
analysis-arrivals-pasted = Fastsat
analysis-arrivals-dictated = Dikteret
analysis-arrivals-imported = Importeret
analysis-arrivals-programmatic = Indsat for dig
analysis-arrivals-count = { $count ->
    [one] { $count } tegn
   *[other] { $count } tegn
}
analysis-arrivals-none = Ingen

analysis-words-per-scene = Ord pr. scene
analysis-median-words = Denne bog er median scene kører { $count ->
    [one] { $count } ord.
   *[other] { $count } ord.
}
analysis-median-line = Median: { $count ->
    [one] { $count } ord
   *[other] { $count } ord
}
analysis-dialogue = Dialog
analysis-dialogue-unsupported = Dialogen måles endnu ikke for dette sprog.

analysis-footnote-words = Fodnoteord
analysis-footnote-words-count = { $count ->
    [one] { $count } ord er i denne bog Buds fodnoter, holdes adskilt fra manuskriptet i alt.
   *[other] { $count } ord er i denne bog Buds fodnoter, holdes adskilt fra manuskriptet i alt.
}
analysis-footnote-words-pending = Tæller fodnoterne

analysis-ignore-empty = Ignorér tekster uden noget skrevet endnu
analysis-empty-hidden = { $count } tom { $count ->
        [one] Tekst
       *[other] tekster
    } Skjult.
analysis-all-texts-empty = Hver eneste tekst i denne bog er stadig tom.

binder-filter-count = { $shown } af { $total } vist
binder-filter-clear = Ryd
binder-filter-none = Intet matcher { $query }.

settings-paratext-intro = Forsiden og bagsiden stof et nyt projekt kan starte med. Hver struktur tilhører en udgivelse tradition, og dens sidetitler er skrevet på denne tradition er eget sprog. Omdøb dem frit, når et projekt er oprettet.
settings-paratext-structures = Strukturer
settings-paratext-broken = Kunne ikke læses
settings-paratext-edit = Rediger
settings-paratext-duplicate = Dupliker
settings-paratext-delete = Slet
settings-paratext-new = Ny struktur
settings-paratext-save = Gem
settings-paratext-editor-hint = Et navn, de sider, der skal laves før manuskriptet, og de sider, der skal laves efter det. Hvor de ender, er dit: de er almindelige genstande, når projektet eksisterer.


image-insert = Indsæt billederName (&M)
image-no-project = Åbn et projekt, før du indsætter et billede.
image-choose-title = Vælg et billede
image-filter-label = Billeder
image-large-title = Dette er et stort billede
image-large-text =
    { $name } er { $megapixels } megapixels ({ $width } × { $height }).
    Holde det som det er gemmer din oprindelige fil i projektet: det rejser
    med hver backup og hver eksport. Optimering gemmer en mindre kopi
    i stedet, op til 2560 pixels på sin længste side.
image-large-keep = Behold originalen
image-large-downscale = Optimér
image-large-remember = Gør det fra nu af, spørg ikke igen
image-not-recorded = Billedet blev gemt, men kunne ikke registreres i projektet.
image-describe-title = Beskriv billedet
image-describe-explain = Hvad billedet viser, for en læser, der ikke kan se det. Det er ikke en del af manuskriptet: det er aldrig tælles, gennemsøges eller eksporteres som prosa.
image-describe-placeholder = et fyrtårn mod en grå himmel
image-resize-title = Ændr størrelse på billedet
image-resize-explain = En procentdel af den størrelse det er vist på nu. 100 efterlader det som det er.
image-resize-invalid = Indtast et tal mellem 1 og 1000.
image-menu-describe = Beskriv billedet (&D)
image-menu-resize = Ændr størrelse på billedet (&R)
image-menu-reset-size = Oprindelig størrelse (&Z)

cover-choose = Bogomslag (&V)
cover-clear = Fjern omslaget (&R)
cover-choose-title = Vælg et omslag
cover-set = Omslaget er klar.
cover-cleared = Omslaget er fjernet. Billedet er stadig i projektet.
card-label = Kort beskrivelse
card-exportable = Eksporteret
card-numbered = Nummereret
card-created = Oprettet
card-modified = Ændret
card-yes = Ja
card-no = Nej
card-type = Type
card-position = Stilling
card-children = Børn
card-goal = Mål
card-words = Ord
card-synopsis = Resumé
card-point-of-view = Synsvinkel
card-aliases = Også kendt som

export-orphan-footnotes-title = Fodnoter uden reference
export-comments-dropped =
    { $count ->
        [one] En kommentar kunne ikke placeres i den eksporterede tekst og blev udeladt.
       *[other] { $count } bemærkninger kunne ikke medtages i den eksporterede tekst og blev udeladt.
    }
export-orphan-footnotes =
    { $count ->
        [one] En fodnote er ikke længere refereret nogen steder i manuskriptet. Dens tekst vil ikke blive vist i den eksporterede bog.
       *[other] { $count } fodnoter er ikke længere refereret nogen steder i manuskriptet. Deres tekst vil ikke blive vist i den eksporterede bog.
    }


menu-footnotes = Fodnoter (&N)
footnotes-title = Fodnoter
footnotes-insert = Indsæt fodnote (&F)
footnotes-empty = Ingen fodnoter endnu. Sæt markøren i en scene, derefter bruge + ovenfor, eller Ctrl + Alt + F.
footnotes-filter-all = Alle
footnotes-filter-document = Dette dokument
footnotes-filter-orphaned = Alfoner
footnotes-orphaned = Intet peger på denne note mere
footnotes-untitled-home = Uden titel
footnotes-body-placeholder = selve noten
footnotes-insert-tooltip = Indsæt en fodnote på markøren (Ctrl + Alt + F)
footnotes-actions = Fodnoteaktioner
footnotes-delete = Slet note og reference hertil (&D)
footnotes-no-project = Åbn et projekt, før der indsættes en fodnote.
footnotes-no-caret = Sæt markøren i en scene-tekst for at indsætte en fodnote der.
footnotes-not-created = Fodnoten kunne ikke tilføjes projektet.
footnotes-deleted-toast = Bemærkning slettet
footnotes-undo-delete = Fortryd

import-document-title = Importdokumenter
import-document-close = Luk
import-document-step-files = Filer
import-document-step-review = Revision
import-document-step-destination = Bestemmelsessted
import-document-step-reconcile = Sammenflet
import-document-reconcile-hint = Nogle af disse elementer er dem, du allerede har. Sig, hvad der skal ske med hver.
import-document-reconcile-all-new = Intet i denne fil matcher dit projekt: hvert element vil blive tilføjet som nyt.
import-document-duplicate-returns = Disse filer er mere end en kopi af det samme manuskript kommer tilbage. Importér dem en ad gangen: matchende kun linjer op en returnerende kopi mod dit projekt, så import flere sammen ville tilføje en anden kopi af bogen i stedet for at fusionere det.
import-document-hunk-added = Tag dette nye punkt
import-document-hunk-removed = Smid dette punkt
import-document-hunk-changed = Tag denne omskrivning
import-document-col-stray-prose = Dens tekst
import-document-stray-as-paratext = Hold som paratext
import-document-stray-discard = Smid teksten
import-document-col-current = I dit projekt
import-document-col-incoming = I denne fil
import-document-col-status = Status
import-document-col-action = Hvad skal man gøre?
import-document-status-identical = Samme
import-document-status-editor-edited = De redigerede det
import-document-status-you-edited = Du redigerede det
import-document-status-conflict = Begge redigerede det
import-document-status-different = Differe
import-document-status-new = Ny
import-document-status-missing = Ikke i denne fil
import-document-status-moved = Flyttet
import-document-action-comments-only = Kun bemærkninger
import-document-action-take-import = Tag denne version
import-document-action-keep-current = Behold mine
import-document-action-create-new = Tilføj som ny
import-document-action-ignore = Spring over
import-document-compare = Sammenlign
import-document-compare-legend = Din version sammenlignet med denne fil. Read- kun.
import-document-compare-close = Luk
import-document-drop-title = Drop dokumenter her
import-document-drop-hint = Markdown (.md), Word (.docx), OpenDocument (.odt) og almindelig tekst (.txt)
import-document-browse = BrowseName
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
import-document-col-breaks = Afbrydelser
import-document-col-comments = Kommentarer
import-document-col-footnotes = Fodnoter
import-document-col-epigraph = Motto
import-document-col-source = Kilde
import-document-level-rules = Udgiftsniveauer
import-document-level-n = Pos. { $level }
import-document-add-top-level = Tilføj øverste niveau
import-document-add-top-level-tooltip = Indsæt en bog over alle analyserede emner, for kapitelfiler som ikke har nogen bogoverskrift
import-document-destination = Bestemmelsessted
import-document-destination-hint = Vælg et bindemiddel eller element: nye elementer lander inde i en mappe, eller efter en scene.
import-document-destination-empty = Ingen bindemidler endnu. Opret en først.
import-document-plan-empty = Intet at importere endnu.
import-document-summary = { $rows ->
    [one] 1 post
   *[other] { $rows } poster
} · { $breaks ->
    [one] 1 scene break
   *[other] { $breaks } scenepauser
}
import-document-back = Tilbage
import-document-cancel = Annuller
import-document-analyse = Næste
import-document-analysing = Læsning af dokumenter
import-document-step-analysing = Læsning
import-document-cancel-analysis = Stop læsning
import-document-analyse-failed = Dokumenterne kunne ikke læses.
import-document-details = Detaljer
import-document-import = Importér
import-document-done = { $count ->
    [one] 1 vare importeret
   *[other] { $count } importerede varer
}
import-document-undo = Fortryd
import-diagnostic-file-unreadable = Der kunne ikke læses { $path }: { $detail }. De øvrige filer importerer stadig.
import-diagnostic-nested-too-deep = Der blev ikke læst { $path }: dens indhold er indlejret mere end { $limit } niveauer dybt. Intet rigtigt dokument kommer i nærheden af det, og læsning det kunne gå ned Skribisto, så det er udeladt. De andre filer stadig importere.
import-diagnostic-archive-too-large = "{ $path }Der blev ikke læst: det er en komprimeret fil bygget til at pakke langt større end det er, som kunne udstøde hukommelse og nedbrud SkribistoDe andre filer importerer stadig.
import-diagnostic-lossy-decode = { $count ->
    [one] Et tegn i denne { $path } blev ikke afkodet. Re- gem filen som UTF-8 for at beholde den.
   *[other] { $count } tegn i MB { $path } ikke dekodere. Re- gemme filen som UTF-8 at holde dem.
}
import-diagnostic-decoded-from-bom = { $path } blev afkodet som { $detail }, ikke UTF-8.
import-diagnostic-empty-file = - { $path } er tomt.
import-diagnostic-no-headings = Der er ingen overskrifter på { $path }, så det kommer som en ting.
import-diagnostic-unsupported-format = Intet læser .{ $detail }-filer, så { $path }-filer blev sprunget over.
import-diagnostic-front-matter-not-flat = Frontmateriale i Mezzogiorno { $path }: Mezzogiorno { $detail } Mezzogiorno er ikke en simpel værdi og blev sprunget over.
import-diagnostic-footnotes-degraded = { $count ->
    [one] En fodnote i parentes{ $path }Ankomst som almindelig tekst: fodnoter er ikke læst fra Markdown.
   *[other] { $count } fodnoter i MB { $path } Ankomst som almindelig tekst: fodnoter læses ikke fra Markdown.
}
import-diagnostic-footnote-not-carried = { $count ->
    [one] En fodnote i "{ $path }" kunne ikke tages med. De andre var.
   *[other] { $count } fodnoter i parentes{ $path }- De andre kunne ikke overtages.
}
import-diagnostic-raw-html-dropped = { $count ->
    [one] En blok af rå HTML i MB { $path } blev droppet.
   *[other] { $count } blokke af rå HTML i MB { $path } blev tabt.
}
import-diagnostic-nested-break-dropped = { $count ->
    [one] En scene bryde inde i et tilbud eller liste i Budding{ $path } blev droppet. Kun en pause på sin egen linje er holdt.
   *[other] { $count } scene pauser inde citater eller lister i MB { $path } blev droppet. Kun en pause på sin egen linje er holdt.
}
import-diagnostic-image-not-ingested = Note { $path }; henviser til billedet; { $detail }; henvisningen kommer som tekst; selve billedet er ikke kopieret.
import-diagnostic-duplicate-title = "{ $title }Det ser ud til, at { $count } gange. Hvis du har importeret disse filer før, dette vil duplikere dem.
import-diagnostic-heading-level-jump = { $title }-jumps fra kursiveau { $from } til { $to }; det placeres et niveau under sin forælder.
import-diagnostic-illegal-combination = Budd{ $title } Buddhares prose, men en { $kind } kan ikke holde nogen. Import holdes, indtil du ændrer sin type eller udelade det. Intet er importeret på nogen anden måde.
import-diagnostic-tracked-changes-flattened = { $path } var midtrevision: { $count ->
    [one] { $count } Sporændring blev accepteret
   *[other] { $count } sporede ændringer blev accepteret
} , og sletninger faldt. Det er den endelige tekst. For at holde nogen af dine egne formuleringer i stedet, skal du bruge Sammenlign i det sidste trin.
import-diagnostic-tracked-changes-flattened-by = { $path } var midtrevision: { $count ->
    [one] { $count } Sporet ændring af { $names } blev accepteret
   *[other] { $count } sporede ændringer af { $names } blev accepteret
} , og sletninger faldt. Det er den endelige tekst. For at holde nogen af dine egne formuleringer i stedet, skal du bruge Sammenlign i det sidste trin.
import-diagnostic-text-box-dropped = { $count ->
    [one] { $path } holder { $count } tekstboks. Dens tekst sidder uden for dokumentets flow, så hvor det hører til i et manuskript kan ikke besvares. Det er ikke importeret.
   *[other] { $path } indeholder { $count } tekstbokse. Deres tekst sidder uden for dokumentets flow, så hvor det hører til i et manuskript, kan ikke besvares. De er ikke importeret.
}
import-diagnostic-embedded-object-dropped = { $count ->
    [one] { $path } holder { $count } indlejret objekt: et diagram, en ligning eller lignende. Der er intet i et manuskript, der kunne holde det.
   *[other] { $path } indeholder { $count } indlejrede objekter: et diagram, en ligning eller lignende. Der er intet i et manuskript, der kunne holde dem.
}
import-diagnostic-field-flattened = { $count ->
    [one] { $path } holder { $count } felt: et sidenummer, en krydsreference, en dato. Det holder teksten det var sidst viser og vil ikke opdatere igen.
   *[other] { $path } indeholder { $count } felter: et sidenummer, en krydsreference, en dato. Hver holder teksten det sidst viste og vil ikke opdatere igen.
}
import-diagnostic-unknown-style-level = { $path } bruger stil af { $detail }, som ligner en overskrift, men navne ikke niveau. Disse afsnit er importeret som prosa snarere end gættet på en dybde.
import-diagnostic-comment-unanchored = Kommentaren var { $detail } i { $path } kunne ikke være knyttet til de ord, det var om. Det er holdt på sin genstand, hvor du kan flytte det.
import-diagnostic-comment-not-carried = Kommentaren Note { $detail } i { $path } er ikke importeret, fordi intet i filen ankommer som tekst det kunne holdes på. Kopiér det fra filen, hvis du har brug for det.
import-diagnostic-comment-replies-flattened = { $count ->
    [one] { $count } svarede i { $path } navngivet en kommentar, der ikke er i filen, så det ankommer som en kommentar af sin egen.
   *[other] { $count } svar i { $path } navngivet en kommentar, der ikke er i filen, så de ankommer som kommentarer af deres egne.
}
import-diagnostic-prose-not-verbatim = { $count ->
    [one] { $count } afsnit i { $path } kunne ikke bringes over med sin formatering præcis som skrevet, så det ankommer som almindelig tekst. Sammenlign det med originalen.
   *[other] { $count } stykker i { $path } kunne ikke bringes over med deres formatering præcis som skrevet, så de ankommer som almindelig tekst. Sammenlign dem med originalen.
}
import-diagnostic-styled-spaces-not-carried = { $count ->
    [one] I { $path }, { $count } strækning af blank plads blev understreget eller slået igennem, en linje tilbage til at udfylde for eksempel. Skribisto holder hverken linje på rum alene, så det ankommer som almindeligt rum, eller slet ikke, hvor det starter eller slutter et afsnit.
   *[other] I { $path }, { $count } strækninger af blank plads blev understreget eller slået igennem, linjer tilbage til at udfylde for eksempel. Skribisto holder hverken linje på rum alene, så de ankommer som almindeligt rum, eller slet ikke, hvor de starter eller slutter et afsnit.
}
import-diagnostic-list-nesting-flattened = { $count ->
    [one] { $count } listepunkt i { $path } blev indlejret mere end { $limit } niveauer dybt. Det ankommer til niveau { $limit }, den dybeste Skribisto holder, med sine ord og formatering.
   *[other] { $count } liste poster i { $path } blev indlejret mere end { $limit } niveauer dybt. De ankommer til niveau { $limit }, den dybeste Skribisto holder, med deres ord og formatering.
}
import-diagnostic-table-read-as-paragraphs = { $count ->
    [one] { $count } tabel i { $path } har en række langt bredere end resten, alt for skævt til at lægge ud som et gitter. Dens celler ankommer som afsnit, den ene efter den anden, med deres ord og formatering.
   *[other] { $count } tabeller i { $path } har en række langt bredere end resten, alt for skævt til at lægge ud som et gitter. Deres celler ankommer som afsnit, den ene efter den anden, med deres ord og formatering.
}
import-diagnostic-spaces-shortened = { $count ->
    [one] { $count } løb af pladser i { $path } var længere end { $limit } rum, og ankommer forkortet til { $limit }. Ordene omkring det er alle der.
   *[other] { $count } løb af rum i { $path } var længere end { $limit } rum, og ankommer forkortet til { $limit }. Ordene omkring dem er alle der.
}
import-diagnostic-epigraph-not-carried = Mead{ $title } Meading by a epigraph, but a { $kind } cannot hold one. Citatet holdes øverst i sin tekst i stedet.
import-diagnostic-epigraph-placement-ambiguous = En epigraph sidder mellem { $title } og { $below } og kunne hovedet enten. Det blev givet til { $title }, som er, hvor en epigraph normalt går.
import-document-diagnostics = { $errors ->
    [0] { $warnings ->
            [one] 1 ting at vide
           *[other] { $warnings } ting at vide
        }
   *[other] { $errors ->
            [one] 1 fil kunne ikke læses
           *[other] { $errors } filer kunne ikke læses
        }
}
import-document-diagnostics-none = Intet at rapportere.

versions-title = Versioner
versions-scope-synopsis = Resumé
versions-scope-prose = Tekst
versions-loading = Kigger gennem dine sikkerhedskopier
versions-empty = Ingen tidligere version af dette endnu
versions-error = Intet er gået tabt, men listen er måske ufuldstændig.
versions-did-not-exist = Findes endnu ikke på { $date }
versions-deleted-after = Udgået et stykke tid efter { $date }
versions-unreadable = { $count ->
    [one] { $count } backup kunne ikke læses
   *[other] { $count } sikkerhedskopier kunne ikke læses
}
versions-thinned = { $count ->
    [one] Ældre versioner tynde ud som de ældes. Projektets egen historie er allerede faldet { $count } tidligere tilstand af denne tekst.
   *[other] Ældre versioner tynde ud som de ældes. Projektets egen historie er allerede faldet { $count } tidligere tilstande af denne tekst.
}
versions-source-backup = Fra en backup
versions-source-project = Fra projektets egen historie
versions-list-caption = En indgang pr. ændring, ikke pr. sikkerhedskopi
versions-pick-a-version = Vælg en version for at se hvad der ændrede sig
versions-earliest = Der er intet ældre at sammenligne med.
versions-no-change = Intet ændret i denne del af elementet
versions-formatting-only = Kun formateringen ændret her. Ordene er de samme
versions-show-unchanged = Vis uændret
versions-hide-unchanged = Skjul uændret
versions-next-change = Næste ændring
versions-near = i nærheden af { $text }
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
versions-pin = Pin backup denne version kom fra. Automatisk oprydning vil aldrig slette det
versions-unpin = Fjern sikkerhedskopi denne version kom fra. Automatisk oprydning kan slette det igen
versions-pinned-only = Vis kun fastgjorte versioner
versions-pin-note = Kun versioner fra en sikkerhedskopi kan fastgøres. En pin holder en fil, og projektets egen historie er ikke en.
versions-pinned-empty = Intet her er fastlåst endnu
versions-pinned-empty-log = Kun versioner fra en backup kan fastgøres. En nål holder en fil, og projektets egen historie er ikke en.
versions-range-filter = Vis kun versioner mellem to datoer
versions-filtered-empty = Ingen version matcher de filtre du sætter
versions-clear-filters = Ryd filtrene
versions-last-30-days = De sidste 30 dage
versions-restore-button = Gendan denne version
versions-restore-confirm-title = Erstat denne tekst med versionen fra { $date }?
versions-restore-confirm-text = Det, De har nu, vil blive erstattet af den tekst, som dette punkt havde på { $date }.
versions-restore-confirm-with-comments = Det, De har nu, vil blive erstattet af den tekst, som dette punkt havde på { $date }. { $count ->
    [one] { $count } kommentarer er forankret i den aktuelle tekst og kan efterlades forældreløse.
   *[other] { $count } kommentarer er forankret i den nuværende tekst og kan efterlades forældreløse.
}
versions-restore-confirm-undo-note = En backup er lavet først, og Ctrl + Z omgør dette i et trin.
versions-restored-toast = Gendannede versionen fra { $date }
versions-undo = Fortryd
versions-restore-row-gone = Denne post er ikke længere i dette projekt
versions-restore-no-home = Dette punkt har ændret type siden da, og den gamle tekst har ingen steder at gå i det
versions-restore-no-safety-copy = Din sikkerhedskopi kørte ikke, så intet blev ændret.
versions-restore-failed = Genoprettelsen mislykkedes: { $error }
versions-restore-backup-busy = En backup kører allerede. Prøv igen om et øjeblik
versions-restore-in-backup-file = Du skal kigge på en backup-fil; åbne projektet selv for at gendanne ind i det
versions-restore-no-project = Intet projekt er åbent
versions-recreate-button = Kom tilbage med den.
versions-recreate-picker-title = Hvor skal den hen?
versions-recreate-picker-empty = Dette projekt har ingen bindemiddel til at sætte det i
versions-recreate-picker-confirm = Kom tilbage med det.
versions-recreate-picker-cancel = Annuller
versions-recreate-untitled = Denne post
versions-recreate-confirm-title = Får du { $item } tilbage?
versions-recreate-confirm-text = Det vil blive tilføjet til { $destination }, med den tekst, det havde på { $date }.
versions-recreate-confirm-undo-note = Fortryd, på den besked, der følger, tager det lige tilbage ud.
versions-recreated-toast = Der er { $item } tilbage i dit projekt
versions-recreated-partial-toast = { $count ->
    [one] { $item } er tilbage, men en af dens tekster kunne ikke læses
   *[other] { $item } er tilbage, men { $count } af dens tekster kunne ikke læses
}
versions-recreate-already-here = Det er allerede i dit projekt.
versions-recreate-no-destination = Vælg et sted i ringbind for at sætte det
versions-recreate-unreadable = At backup kunne ikke læses, så intet blev tilføjet
versions-recreate-failed = Kunne ikke bringe det tilbage: { $error }
versions-changed-percent = { $percent }% af dette ændrede sig
versions-hidden-paragraphs = { $count ->
    [one] Note: { $count } uændret afsnit
   *[other] - { $count } uændrede afsnit
}

timeline-title = Gå tilbage i tiden
timeline-coverage = { $count ->
    [one] { $count } version indspillet, går tilbage til { $oldest }
   *[other] { $count } indspillede versioner, der går tilbage til { $oldest }
}
timeline-loading = Kigger igennem dit projekt er forbi
timeline-empty = Der er endnu ikke registreret nogen version af dette projekt
timeline-no-changes = Intet har ændret sig siden da.
timeline-no-text-changes = Ingen tekst har ændret sig siden da
timeline-slider-label = Optaget version
timeline-series-name = Projektstørrelse
timeline-bars-caption = Hver bar er en indspillet version, så høj som projektet var dengang. Den fremhævede en er, hvad du skal kigge på.
timeline-bars-caption-periods = For mange versioner til at vise én efter én, så hver bar er en { $period }, så høj som projektet var ved udgangen af det.
timeline-unit-hour = time
timeline-unit-day = dag
timeline-unit-week = uge
timeline-unit-month = måned
timeline-range-filter = Vis kun versioner mellem to datoer
timeline-range-empty = Ingen version blev registreret på disse datoer
timeline-open-period = Åbn denne periode
timeline-show-all = Vis hele historien
timeline-changed-since = { $count ->
    [one] { $count } post varierer mellem { $date } og dit projekt nu
   *[other] { $count } poster varierer mellem { $date } og dit projekt nu
}
timeline-kind-added = Skrevet siden
timeline-kind-removed = Ikke længere i projektet
timeline-kind-changed = Edited siden
timeline-kind-moved = Flyttet siden
timeline-not-yet-written = Dette eksisterede endnu ikke på det tidspunkt
timeline-no-text-of-its-own = Dette har ingen egen tekst. Det er en kurs for, hvad der er inde i det
timeline-reader-close = Luk
timeline-reader-stamp = Som det var på { $date }
timeline-reader-loading = Åbning af den registrerede version
timeline-reader-unreadable = Denne optagelse kunne ikke læses. Sikkerhedskopien kan være blevet flyttet, slettet, eller være på et drev, der isn 't tilsluttet.
timeline-reader-compared = Som det var på { $date }, sat imod, hvad det siger nu
timeline-reader-diff-legend = Der er gået et stykke tid siden, og det er blevet understreget.
timeline-reader-view-label = Hvad skal du vise
timeline-reader-view-diff = Ændringer
timeline-reader-view-text = Tekst
timeline-reader-deleted = Dette er ikke længere i dit projekt. Du kan læse det og kopiere det her.
timeline-prose-only-record = Dette punkt kommer fra projektets egen historie, som holder tekst og intet andet, så hvad der blev slettet eller flyttet, da det kan ikke vises. Vælg en backup for det.

settings-sec-extensions = Udvidelser

settings-group-games-forward = Altid fremad
settings-games-forward-toggle = Afspil altid fremad
settings-games-forward-hint =
    Mens du spiller, intet du har skrevet kan tages tilbage: Backspace, Slet,
    Klip, drag- and - drop og Fortryd er alle deaktiveret i de overflader, du vælger nedenfor.
    Du kan stadig skrive, indsætte, formatere og bevæge dig frit rundt. Udkastet vokser kun.
settings-games-session-warning =
    Dette er et per-session valg: det er aldrig gemt. Lukning af projektet, eller stoppe
    Skribisto, altid slutter spillet, og alt hvad du skrev, mens du spiller bliver
    Umuligt igen i det øjeblik, det gør.
settings-group-games-scope = Hvis den finder anvendelse
settings-games-in-prose = Manuskript prose
settings-games-in-synopsis = Synopser
settings-games-scope-hint =
    Kommentarer, fodnoter og titler er aldrig frosne: de er, hvor du bemærker fix
    Du har lige forbudt dig selv at lave.
settings-games-inert-warning =
    Altid fremad er på, men gælder for ingenting. Tik mindst en overflade over,
    eller det ændrer intet som du skriver.

games-title = Skrivning spil
games-forward-name = Altid fremad
games-forward-blurb =
    Draft uden at tage noget tilbage. Sletning, skæring og udløsning er deaktiveret
    Skriv den næste sætning i stedet for at lave den sidste.
games-forward-playing = Afspiller: sletning er deaktiveret
games-forward-idle = Ikke at spille
games-session-note = Det slutter, når du lukker projektet.
games-scope-prose-and-synopsis = Gælder for din prosa og synopser.
games-scope-prose = Det gælder for din prosa.
games-scope-synopsis = Det gælder dine synopser.
games-scope-nothing = Finder ikke anvendelse endnu. Vælg en overflade i Indstillinger.
games-settings-link = Skrivning af spilindstillinger
statusbar-games-forward = Altid fremad
statusbar-games-forward-tooltip =
    Altid fremad er på: sletning, skæring og udløsning er deaktiveret, mens du
    draft. Klik for at stoppe med at spille.
goal-progress-words = { $g ->
   *[other] Ord: { $count } / { $goal }
}
goal-progress-characters = { $g ->
   *[other] Tegn: { $count } / { $goal }
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

inspector-goal = Mål
inspector-goal-none = Intet mål

overview-col-goal = Mål
overview-excluded-from-export = Venstre ud af eksporten, så det tæller til ingen total

goal-subtree-total-words = { $count ->
    [one] { $items } mål inde tilføjer op til { $words } ord
   *[other] { $items } mål inde tilføje op til { $words } ord
}
goal-subtree-total-characters = { $count ->
    [one] { $items } mål inde tilføjer op til { $words } tegn
   *[other] { $items } mål inde tilføje op til { $words } tegn
}

new-work-goal-unit = Tæl i
new-work-goal-unit-hint = Bruges til hvert ord eller karakter mål i dette projekt. Changable senere.

settings-group-goal-unit = Mål
settings-goal-unit-switch-title = Skift tællingsenhed
settings-goal-unit-switch-text = De mål og milepæle, der allerede er fastsat for dette projekt, blev indtastet i { $from }. Skift til { $to } ikke konvertere dem, så alle eksisterende antal vil nu blive læst som { $to }.
settings-goal-unit-switch-informative = Intet går tabt. Skift tilbage til enhver tid for at genoprette den oprindelige læsning, og opdater derefter de mål, du ønsker at holde.
settings-goal-unit-switch-confirm = Skift

goal-distribute-action = FordelingName
distribute-title = Fordel målet
distribute-empty = Der er intet i det her at dele målet ud til.
distribute-weight = Del ud
distribute-weight-length = Efter længde
distribute-weight-rows = I stykker
distribute-weight-even = Aftenweather condition
distribute-overwrite = Erstat mål, der allerede er sat
distribute-col-item = Konto
distribute-col-current = Nu
distribute-col-proposed = Efter
distribute-total = Disse beløb udgør { $total } mod et mål på { $goal }.
distribute-over-budget = De mål, der allerede er sat inde, kommer til { $over } mere end denne container. Hæv sit mål, lavere deres, eller erstatte dem.
distribute-apply = Fordel
distribute-cancel = Annuller

milestone-target-gone = Målet slettet
milestone-no-target = Intet mål
milestone-add = Tilføj
milestone-add-label = Hvad skal man opnå?

pace-summary-title = Hvor bogen står
pace-summary-remaining = { $words } tilbage
pace-summary-open = Åbn planen
pace-summary-close = Luk
pace-summary-dont-show = Vis ikke dette når du åbner
pace-summary-menu = Skrivning af plan

menu-help-topics = Hjælp emner (&T)
menu-help-shortcuts = Tastaturgenveje (&K)
menu-help-website = Skribisto på nettet (&W)
menu-help-report = Rapportér et problem (&R)
menu-help-check-updates = Tjek for opdateringer (&U)
menu-command-palette = Kommando PaletteComment (&C)

learn-help-topics = Hjælp emner
learn-shortcuts = Tastaturgenveje
learn-website = Skribisto på nettet

help-window-title = Hjælp
help-filter-topics = Filtrér emner
help-no-matching-topic = Intet emne matcher.
help-topic-missing = Dette emne er ikke længere tilgængeligt.
help-back = Tilbage
help-not-translated = Denne artikel findes endnu ikke på dansk. Den vises på engelsk.

help-section-getting-started = Kom i gang
help-section-writing = Skrivning
help-section-reviewing = Gennemgang
help-section-exchanging = Få ord ind og ud
help-section-keeping = Holde dit projekt sikkert
help-section-extensions = Udvidelser

help-topic-getting-started = Dit første projekt
help-topic-writing-model = Hvordan en bog er sat sammen
help-topic-drafts-and-old-versions = Holde et ældre udkast
help-topic-goals-and-pace = Mål og tempo
help-topic-comments = Kommentarer
help-topic-round-trip = Send din bog til en læser
help-topic-export = Eksportering
help-topic-import-documents = Importdokumenter
help-topic-import-projects = At bringe et helt projekt på tværs
help-topic-backups-and-versions = Backup og versioner

help-shortcuts-title = Tastaturgenveje
help-shortcuts-filter = Filtergenveje
help-shortcuts-no-matches = Ingen genvej passer.
help-shortcuts-rebind = Skift genveje
help-shortcuts-close = Luk

command-palette-placeholder = Skriv en kommando

shortcut-name-binder-duplicate = Dupliker
shortcut-name-comments-add = Tilføj kommentar
shortcut-name-comments-add-paragraph = Kommentar til afsnit
shortcut-name-spellcheck-toggle = Tjek stavning
shortcut-name-editor-tab-close = Luk faneblad
shortcut-name-editor-tab-pin = Pin- eller unpin- faneblad
shortcut-name-editor-save = Gem
shortcut-name-work-export = Eksportér…
shortcut-name-work-new = Nyt projekt
shortcut-name-work-open = Åbn projekt
shortcut-name-window-new = Nyt vindue
shortcut-name-work-close = Luk projekt
shortcut-name-app-settings = Indstillinger
shortcut-name-app-quit = Afslut
shortcut-name-editor-insert-footnote = Indsæt fodnote
shortcut-name-format-scene-break = Indsæt Scene Break
shortcut-name-format-major-scene-break = Indsæt større scene break
shortcut-name-format-link = Indsæt link
shortcut-name-format-mark-nbsp = Indsæt nulpunktsmellemrum
shortcut-name-format-mark-nbhyphen = Indsæt not- break Hyphen
shortcut-name-format-mark-nnbsp = Indsæt smallere nulpunktsmellemrum
shortcut-name-format-mark-zwsp = Indsæt Zerobredde mellemrum
shortcut-name-go-next = Næste
shortcut-name-go-prev = Forrige
shortcut-name-go-to = Gå til
shortcut-name-outline-toggle = Slå omrids til / fra
shortcut-name-preview-toggle = Slå forhåndsvisningsbånd til og fra
shortcut-name-view-fullscreen = Slå fuldskærm til / fra
shortcut-name-view-focus-mode = Slå distraktion- free tilstand til / fra
shortcut-name-editor-size-increase = Forøg tekststørrelse
shortcut-name-editor-size-decrease = Formindsk tekststørrelse
shortcut-name-editor-size-reset = Nulstil tekststørrelse
shortcut-name-editor-find = Find
shortcut-name-editor-replace = Erstat
shortcut-name-editor-find-next = Næste match
shortcut-name-editor-find-prev = Forrige Match
shortcut-name-search-show = Søg i projekt
shortcut-name-search-replace = Erstat i projekt
shortcut-name-outline-open-to-side = Åbn til siden
shortcut-name-help-topics = Hjælp emner
shortcut-name-help-shortcuts = Tastaturgenveje
shortcut-name-help-website = Skribisto på nettet
shortcut-name-help-report = Rapportér et problem
shortcut-name-help-check-updates = Tjek for opdateringer
update-check-behind = Version { $version } er tilgængelig.
update-check-current = Du kører den nyeste version.
update-check-failed = Kunne ikke tjekke for en nyere version. { $error }
shortcut-name-command-palette = Kommandopalette
help-section-reference = Hvad er det?

help-concept-scene-break = Sceneskift
help-concept-major-scene-break = Større scenebrud
help-concept-find-in-prose = Find i prose
help-concept-story-bible = Historiebibel
help-concept-goal-unit = Ord eller tegn
help-concept-goal-progress = Fremskridt
help-concept-manuscript-words = Hvad der tæller som manuskriptet
help-concept-exportable = Forladt fra eksport
help-concept-distribute = Deling et mål ud
help-concept-subtree-total = Hvad målene inde i tilføjer op til
help-concept-milestone = Milepæl
help-concept-pace-plan = Passeplan

help-concept-tag = Mærke
help-concept-label = Kort beskrivelse
help-concept-point-of-view = Synsvinkel
help-concept-epigraph = Motto
help-concept-footnote = Fodnote
help-concept-chapter-mode = Kapitel form
help-concept-comment = Kommentar
help-concept-backup = Sikkerhedskopi
help-concept-version = Version
help-concept-trash = Papirkurv
help-concept-spellcheck = Stavekontrol
help-concept-search-replace = Søg og erstat
help-concept-note-template = Note skabelon
help-concept-story-bible-entry = Indgang i historiebibel
help-concept-text-replacement = Teksterstatning
help-concept-smart-punctuation = Smart punktuering
help-concept-export-style = Eksportér stil
help-concept-round-trip-marks = Round- trip markeringer

epigraph-mark-attribution = Kildelinje
epigraph-mark-attribution-tip = Markér den linje, som omsorgen er i som citatet kilde, så det udskriver som en tildeling
margin-lane-name = Marginmærker
margin-lane-provider-comments = Kommentarer
margin-lane-provider-story-bible = Indgang i historiebibel
margin-lane-provider-story-bible-hint = På en note Buds I prosa læsning, hvor denne indgang er navngivet, og scenerne fortalte gennem sine øjne. Også markerer disse navne i prosa selv.
margin-lane-mark-point-of-view = Fortalt herfra
margin-lane-mark-named-here = Opkaldt her
margin-lane-provider-comments-hint = Når teksten er vedlagt en note
margin-lane-provider-search = Søg hits
margin-lane-provider-search-hint = Hvert hit for hvad du sidst søgte efter, uanset hvor det er i dokumentet
margin-lane-provider-boundaries = Hvor hvert dokument starter
margin-lane-provider-boundaries-hint = En regel på toppen af hver scene i en stream, så du kan se, hvor du er
margin-lane-provider-spelling = Stavning
margin-lane-provider-spelling-hint = Hvor stavekontrollen har markeret et ord. Off, medmindre du beder om det: det markerer en maskine Buds mening om din prosa
margin-lane-spelling = { $word }, muligt stavefejl
margin-lane-search-hit = { $text }, match { $index } af { $total }
margin-lane-search-current = { $text }, match { $index } af { $total }, den du er på
margin-lane-boundary = Start på { $title }
margin-lane-boundary-untitled = et dokument uden titel
settings-page-margin-lane = Marginmærker
settings-desc-margin-lane = Striben ved siden af rullebjælken, og hvad den viser
settings-margin-lane-enabled = Vis marginer
settings-margin-lane-enabled-hint = Samme knap som Vis Margin mærker.
settings-margin-lane-enabled-more = Det siger aldrig noget er galt: det viser dig, hvor tingene er, og overlader hvad du skal gøre af det til dig.
settings-group-margin-lane-marks = Hvad det markerer
settings-margin-lane-no-providers = Intet markerer vejen endnu.
settings-group-margin-lane-texture = Dialog tekstur
settings-margin-lane-texture = Vis dialogtekstur
settings-margin-lane-texture-hint = En bar pr. afsnit: hvor lang tid det er, og hvor meget af det tales.
settings-margin-lane-texture-more = Ikke målt for sprog uden kurateret konvention, hvor baren viser længde alene.
settings-group-margin-lane-surfaces = Hvis det viser sig
settings-margin-lane-surface-editor = Teksteditor
settings-margin-lane-surface-stream = StreamsCity in Italy
settings-margin-lane-surface-search-preview = Søgeforhåndsvisning
menu-margin-lane = Marginmærker (&M)

status-preset-drafting = Udkast
status-preset-passes = Revisionskort
status-preset-plume = Plume Creator
status-todo = Til at gøre
status-draft = Udkast
status-revised = Revideret
status-final = Endelig
status-outline = Disposition
status-first-edit = Første redigering
status-second-edit = 2. redigér
status-done = Færdig
status-plume-draft-1 = Første udkast
status-plume-draft-2 = 2. udkast
status-plume-draft-3 = Tredje udkast
status-plume-edit-1 = Første redigering
status-plume-edit-2 = 2. redigér
status-plume-edit-3 = 3. redigér
status-plume-proofread = Korrekturlæst
status-plume-finished = Færdig
new-work-statuses = Arbejdsgang
new-work-statuses-hint = De stadier, du flytter en scene igennem. Du kan omdøbe, ombestille eller tilføje til disse til enhver tid.
status-none = Ingen status
inspector-status = Status
overview-col-status = Status
overview-status-mixed = Følgende dele er uenige:
help-concept-status = Status
status-completion-title = Hvor bogen står
status-completion-headline = { $done } af { $total } scener færdige
status-completion-empty = Ingen scener endnu. Dette læser manuskriptet, så det udfylder som du skriver.
status-completion-open = Hvor bogen står

settings-page-statuses = Statuser
settings-desc-statuses = Omdøbe, ombestille, tilføje og fjerne de stadier, du flytter en scene gennem
settings-statuses-add = Tilføj status
settings-statuses-add-placeholder = Navn på ny status
settings-statuses-desc =
    De stadier en scene bevæger sig igennem, i rækkefølge, fra mindst færdig øverst til de fleste
    færdig i bunden. Denne ordre er, hvad uden færdig end det betyder overalt andre steder
    i app 'en, så sætte dem den måde, din proces rent faktisk kører.
settings-statuses-apply-preset = Påfør et forudindstillet
settings-statuses-preset-applied = { $added ->
    [one] 1 tilføjet status
   *[other] { $added } tilsatte statuer
}
settings-statuses-preset-refused = Dette projekt har allerede en stige. Presets kun udfylde en tom en. Slet de trin, du don 't ønsker først.
settings-statuses-duplicate = Der er allerede { $name } på denne stige.
settings-statuses-added = Tilføjet: { $name }
settings-statuses-move-up = Flyt op (mindre færdig)
settings-statuses-move-down = Flyt ned (mere færdig)
settings-statuses-details-placeholder = Hvad denne fase betyder (valgfrit)
settings-statuses-delete = Slet mv. { $name } mdr.
settings-statuses-delete-in-use = { $count ->
    [one] Slet Mex{ $name } Mex: 1 element er på det og vil miste sin status
   *[other] Slet Budd{ $name } Buddha: { $count } elementer er på det og vil miste deres status
}
settings-statuses-deleted = Udgået fra { $name }
settings-statuses-deleted-in-use = { $count ->
    [one] Slettet{ $name }En genstand har ikke længere status.
   *[other] Slettet i { $name }. { $count } poster har ikke længere status.
}
settings-statuses-empty-title = Ingen workflow endnu
settings-statuses-empty-body = En status siger, hvor langt langs en scene er. Tilføj en ovenfor, eller start fra en forudindstillet.
status-category-planned = Planlagt
status-category-drafting = Udkast
status-category-needs-work = Kræver arbejde
status-category-revised = Revideret
status-category-final = Endelig
