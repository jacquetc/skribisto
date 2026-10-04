# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

menu-work = Prosjekt (&W)
menu-new-work = Nytt prosjekt (&N)
menu-open-work = Åpne prosjekt… (&O)
menu-new-window = Nytt vindu (&W)
menu-import-from = Import fra (&I)
menu-create-from = Opprett fra (&C)
menu-import-plume = Plume Creator (.plume)… (&P)
menu-import-document = Dokumenter (Markdown, Word, ODT)... (&D)
menu-import-manuskript = Manuskript (.msk)… (&M)
menu-export = Eksporter (&X)
menu-export-book = Eksporter bok
menu-export-part = Eksporter del
menu-export-chapter = Eksportkapittel
menu-export-scene = Eksporter Scene
menu-export-note = Eksporter notat
menu-export-paratext = Eksporter paratekst
menu-export-folder = Eksporter mappe
menu-export-choose = Velg...
menu-export-none = Åpne et dokument som skal eksporteres
menu-save = Lagre (&S)
menu-save-as-file = Lagre som én fil… (&F)
menu-save-as-folder = Lagre som mappe… (&D)
menu-backup = Sikkerhetskopier nå (&B)
menu-close-work = Lukk prosjekt (&C)
menu-welcome = Velkommen... (&L)
menu-settings = Innstillinger (&E)
menu-quit = Avslutt (&Q)

menu-view = Vis (&V)
menu-outline = Disposisjon (&O)
menu-search = Søk i prosjektet (&S)
menu-timeline = Gå tilbake i tiden (&G)
menu-search-preview = Forhåndsvisning av søk (&P)
menu-fullscreen = Fullstendig (&F)
menu-focus-mode = Distraksjonsfri modus (&D)
menu-format = Format (&O)
menu-scene-break = Sett inn scenepause (&K)
menu-major-scene-break = Sett inn større scenepause (&M)

menu-go = Gå til (&G)
menu-go-next-scene = Neste scene (&S)
menu-go-prev-scene = Forrige scene (&V)
menu-go-next-chapter = Neste kapittel (&C)
menu-go-prev-chapter = Forrige kapittel (&H)
menu-go-next-note = Neste notat (&N)
menu-go-prev-note = Forrige notat (&T)

menu-tools = Verktøy (&T)
menu-help = Hjelp (&H)

ctx-add = Legg til (&A)
ctx-new-item = Nytt element (&N)
ctx-new-folder = Ny mappe (&F)
ctx-rename = Gi nytt navn (&R)
ctx-duplicate = Dupliser (&D)
ctx-import-here = Importer her... (&I)
ctx-indent = Innrykk (&I)
ctx-outdent = Reduser innrykk (&O)
ctx-trash = Flytt til papirkurven (&T)
ctx-open-to-side = Åpent for siden (&S)
ctx-open = Åpne (&O)
ctx-reveal-in-outline = Avsløring i Outline (&L)
ctx-move-up = Flytt opp (&U)
ctx-move-down = Flytt ned (&N)

ctx-tab-close = Lukk (&C)
ctx-tab-close-others = Lukk andre faner (&O)
ctx-tab-close-all = Lukk alle faner (&L)
ctx-tab-open-to-side = Åpent for siden (&S)
ctx-tab-open-in-main = Åpne i hovedruten (&I)
ctx-tab-move-to-side = Flytt til siden (&M)
ctx-tab-move-to-main = Flytt til hovedruten (&M)
ctx-tab-move-to-new-window = Flytt til et nytt vindu (&N)
ctx-tab-move-window-unsaved = Lagre dette prosjektet først: et annet vindu åpnes på en fil på disken
ctx-tab-pin = Pin denne fanen (&P)
ctx-tab-unpin = Fjern denne fanen (&U)
tab-pinned-tooltip = Pinned: - Lukk andre - og - Lukk alle - la det være åpent

create-book = Bok
create-part = Del
create-chapter = Kapittel
create-scene = Scene
create-note = Notat
create-note-folder = Merknadsmappe
create-folder = Mappe
create-paratext = Paratekst
create-paratext-folder = Paratekstmappe
create-book-end = Slutt på bok
create-story-bible-entry = Historiebibel oppføring...
type-book-start = Bokstart
type-text = Tekst

new-item-book = Ny bok
new-item-part = Ny del
new-item-chapter = Nytt kapittel
new-item-note = Nytt notat
new-item-note-folder = Ny notatmappe
new-item-folder = Ny mappe
new-item-paratext = Ny Paratekst
new-item-paratext-folder = Ny Paratekstmappe
new-item-story-bible-entry = Ny historie bibelsk inngang

placement-inside = Inne
placement-after = etter
placement-after-parent = etter foreldre
placement-top-level = toppnivå


ctx-promote = Konverter til (&C)
promote-chapter-folder = Kapittelmappe
promote-flat-chapter = Selvstendig kapittel
promote-lossy-title = Ingen steder å holde teksten
promote-lossy-text = En { $target } har ingen steder å holde disse: { $kinds }. Flytt eller rens teksten først, og deretter konvertere.
content-scene-text = Scenetekst
content-note-text = Notetekst
content-book-title = Boktittel
content-book-subtitle = Bestill undertittel
content-part-title = Deltittel
content-chapter-title = Kapitteltittel
content-epigraph-text = Motto
content-paratext-text = Paratekst
promote-blocked-title = Kapittel ikke tomt
promote-blocked-text = { $count ->
    [one] Dette kapitlet inneholder fortsatt { $count } element. Flytte eller søppel det før konvertering til et flatt kapittel.
   *[other] Dette kapitlet inneholder fortsatt { $count } elementer. Flytt eller søppel dem før du konverterer det til et flatt kapittel.
}

inspector = Egenskaper
inspector-empty = Åpne et element for å inspisere det.
inspector-promote = Konverter til...
statusbar-toggle-outline = Slå bindemidlet
statusbar-toggle-inspector = Slå inspektøren av
statusbar-save-unsaved = Utelatte endringer. Klikk for å lagre
statusbar-save-saved = Alle endringer lagret
statusbar-save-autosave = Automatisering er på. Endringer lagres når du skriver
statusbar-saving = Sparer...
statusbar-word-count = { $count ->
    [one] { $count } ord
   *[other] { $count } ord
}
statusbar-word-count-tooltip = Ord i scenen du redigerer
statusbar-word-char-count = { $words ->
    [one] { $words } ord
   *[other] { $words } ord
} · { $chars ->
    [one] { $chars } tegn
   *[other] { $chars } tegn
}
session-toggle = Skriveøkt: start eller pause en fokusert sprint
session-configure = Angi øktens ordmål og tidsgrense
session-configure-title = Skriveøkt
session-word-goal = Ordmål
session-time-limit = Tidsgrense
session-time-limit-unit = min
session-no-goal = Ingen mål
session-no-limit = Ingen grense
session-reset = Nullstill økt
session-readout = { $words } ord · { $time }
session-readout-timed = { $words } ord · { $time } left
statusbar-focus-exit = Avslutt distraksjonsfri modus
statusbar-focus-synopsis = Sammendrag
statusbar-focus-go-prev = Forrige element (Alt+Up)
statusbar-focus-go-next = Neste element (Alt+Down)

rail-settings = Innstillinger

language = Språk
theme = Tema
english = Engelsk
french = fransk
light = Lys
dark = Mørke
settings-text-width = Tekstbredde
settings-preview-width = Forhåndsvisningsbredde for søk
settings-autosave = Last ned på disk
settings-show-welcome = Vis oppstarteren ved oppstart
settings-show-welcome-tip = Når det er avbrutt, åpner det siste prosjektet i stedet.
settings-check-for-updates = Sjekk etter nye versjoner
settings-check-for-updates-tip = En gang om dagen på det meste, Skribisto spør prosjektets hjemmeside hvilken versjon som er gjeldende. Forespørselen har ingen identifikator, og ikke engang den versjonen du kjører: sammenligningen skjer på denne datamaskinen.

settings-title = Innstillinger
settings-close = Lukk
settings-search = Søkeinnstillinger
settings-reset = Tilbakestill til standardverdier
settings-done = Ferdig
settings-cancel = Avbryt
settings-apply = Bruk
settings-ok = OK
settings-reset-confirm-title = Tilbakestille alle innstillinger til standard?
settings-reset-confirm-body = Gjenoppretter utseendet til dette programmet, redaktør- og skriveinnstillingene. Prosjektets egne innstillinger, tastatursnarveier og lagrede stiler er ikke påvirket. Det kan ikke angres.
settings-empty-title = Ingen innstillinger her enda
settings-empty-hint = Denne delen vil få alternativer i en fremtidig oppdatering.

settings-sec-appearance-behaviour = Utseende Atferd
settings-sec-editor = Redigerer
settings-sec-spelling = Staving
settings-sec-backup = Sikkerhetskopiering
settings-sec-compile = Kompiler eksport
settings-page-appearance = Utseende
settings-field-image-size-policy = Store bilder
settings-image-policy-ask = Spør hver gang
settings-image-policy-keep = Behold originalt
settings-image-policy-downscale = Optimaliser
settings-hint-image-size-policy = Hva du skal gjøre når et innsatt bilde er større enn manuskriptet trenger.
settings-page-notifications = Varsler
settings-page-scene = Scene
settings-page-synopsis = Sammendrag
settings-page-notes = Notater
settings-page-editor-behavior = Redaktøradferd
settings-page-goals = Mål Ord Teller
settings-page-games = Skrive spill
settings-page-corkboard = Oppslagstavle
settings-page-distraction-free = Distraksjonsfri
settings-page-dictionaries = Ordbøker
settings-page-autosave = Automatisere
settings-page-export = Eksporter format
settings-page-paratext = Paratekststrukturer
settings-page-keymap = Nøkkelkart
settings-keymap-filter = Filtrer snarveier

settings-desc-sec-appearance-behaviour = Hvordan selve programmet ser ut, og hva det gjør når det starter.
settings-desc-sec-editor = Skriveoverflaten: Hvordan det ser ut, og hva det gjør som du skriver.
settings-desc-sec-spelling = Stell-checking, og ordbøkene bak den.
settings-desc-sec-backup = Hvordan prosjektet ditt når disken, og hvilke kopier er lagret.
settings-desc-sec-compile = Hva etterlater Skribisto, og i hvilken form.
settings-desc-sec-work = Innstillinger som tilhører dette prosjektet og reiser inne i filen.
settings-desc-sec-extensions = Sider som støttes av utvidelser som er installert her.
settings-desc-group-typography = En side per type tekst: type, størrelse, linjehøyde og avstand.
settings-desc-appearance = Grensesnittspråk, tema, tekststørrelse og oppstarter ved oppstart.
settings-desc-notifications = Hver melding denne sesjonen har vist, og handlingene du kan spille på nytt.
settings-desc-scene = Hvordan scene prosa er satt: typeflate, størrelse, linjehøyde, innrykk og avstand.
settings-desc-synopsis = Hvordan synopsis ruten er satt, uavhengig av manuskriptet.
settings-desc-notes = Hvordan notater er satt, uavhengig av manuskriptet.
settings-desc-corkboard = Kortstørrelse, hva et kort viser, og hvordan styret arrangerer dem.
settings-desc-distraction-free = Typografi, kolonnebredde og kontrollstripe for fullskjermskriving.
settings-desc-distraction-free-themes = Temabiblioteket for fullskjermskriving: de fraktede og dine egne.
settings-desc-editor-behavior = Tekstbredde, synopsis-posisjon, skrivemaskinrulling og markørheving.
settings-desc-punctuation = Den typografiske husstilen nye prosjekter starter fra. Hvert prosjekt følger dette med mindre det setter sin egen.
settings-desc-goals = Ord og karakter mål, og hvordan ord blir talt.
settings-desc-games = Begrenser deg selv mens du utarbeider, som alltid fremover.
settings-desc-spellcheck = Enkeltbryteren som slår stavekontroll på og av overalt.
settings-desc-dictionaries = Installer, fjern og se staveordbøker på denne maskinen.
settings-desc-autosave = Om endringene dine er skrevet til disk på egen hånd.
settings-desc-backup = Når kopier tas, hvor de holdes, og hvor mange. Hvert prosjekt følger dette med mindre det setter sin egen.
settings-desc-export = De stilene hver eksport kompilerer gjennom: de fraktede og dine egne.
settings-desc-paratext = Forsiden og baksiden kan et nytt prosjekt begynne med.
settings-desc-keymap = Hver snarvei og hva den er bundet til.
settings-page-user = Bruker
settings-desc-user = Hvem du er, for kommentarene du skriver.
settings-group-identity = Identifikasjon
settings-field-user-name = Ditt navn
settings-field-user-name-placeholder = Valgfritt
settings-field-user-name-hint = Signerer kommentarer og svar du skriver. Skilt fra et prosjekts forfatternavn, som er bokens byline og reiser i filen.
settings-field-user-initials = Dine initialer
settings-field-user-initials-placeholder = Fra ditt navn
settings-field-user-initials-hint = Hva en ordbehandler viser ved siden av kommentaren i margen. La det stå tomt for å bruke de som vises, tatt fra navnet ditt.
settings-field-user-hint = Begge er valgfrie og gjelder for hvert prosjekt på denne datamaskinen. Endring av dem signerer kommentarene du skriver neste. Kommentarer allerede skrevet holde navnet de ble skrevet under.
settings-desc-author = Navnet på dette prosjektet.
settings-desc-structure = Om dette prosjektets kapitler er mapper eller enkeltelementer.
settings-desc-language = Språket dette prosjektets prosa er kontrollert mot.
settings-desc-work-backup = Dette prosjektets egen sikkerhetskopieringspolitikk, eller den generelle.
settings-desc-personal-dictionary = Ordene dette prosjektet behandler som riktig stavet.
settings-desc-tags = Fargekodede etiketter dette prosjektet markerer sine elementer med.
settings-desc-templates = De notatmalene du kan sette inn mens du skriver i dette prosjektet.
settings-desc-text-replacements = Snarveier som utvider seg når du skriver i dette prosjektet.
settings-desc-work-punctuation = Citatene, dashs og avstand dette prosjektets prosa følger.

settings-group-typography = Typografi
settings-group-writing-column = Skrivekolonne
settings-group-writing-view = Skrivevisning
settings-group-distraction-free-strip = Kontrollstripe
settings-group-theme = Tema
settings-group-language = Språk
settings-group-startup = Starting
settings-group-updates = Oppdateringer
settings-group-autosave = Automatisere
settings-field-typeface = Typesnitt
settings-field-size = Tekststørrelse

editor-size-changed-manuscript = Manuskripttekststørrelse: { $percent }
editor-size-changed-synopsis = Synopsis tekststørrelse: { $percent }
editor-size-changed-notes = Notes tekststørrelse: { $percent }
editor-size-changed-corkboard = Corkboard kort tekststørrelse: { $percent }
editor-size-changed-corkboard-expanded = Utvidet redaktør tekststørrelse: { $percent }
editor-size-changed-distraction-free = Distraksjonsfri tekststørrelse: { $percent }

menu-text-size-increase = Øk tekststørrelsen (&I)
menu-text-size-decrease = Reduser tekststørrelsen (&C)
menu-text-size-reset = Tilbakestill tekststørrelse (&R)
settings-field-line-height = Linjehøyde
settings-field-first-line-indent = Første linjes innrykk
settings-field-paragraph-spacing-before = Rom før avsnitt
settings-field-paragraph-spacing-after = Rom etter avsnitt
settings-field-column-width = Kolonnebredde
settings-distraction-free-width-hint = Gjelder bare mens distraksjonsfri modus er på. Scene, Synopsis og Notes kolonne bredder er uberørt.
settings-distraction-free-title = Behold gjenstandens navn
settings-distraction-free-word-count = Behold ordet count
settings-distraction-free-session = Behold skriveøkten
settings-distraction-free-go-to = Behold Gå til... knappen
settings-distraction-free-go = Behold de forrige og neste knappene
settings-distraction-free-chrome-hint = Utløpsknappen forblir alltid, uansett hva du velger her. Det er din vei ut hvis Escape er opptatt.
settings-field-app-theme = Tema
settings-theme-light = Lys
settings-theme-dark = Mørke
settings-field-text-scale = Tekststørrelse på grensesnitt
settings-field-language = Grensesnittspråk
settings-synopsis-placement = Synopsis posisjon
settings-synopsis-placement-none = Ingen
settings-synopsis-placement-top = Over
settings-synopsis-placement-side = Ved siden av
settings-typewriter = Skrivemaskin rulling
settings-typewriter-tip = Holder linjen du skriver i en fast høyde mens manuskriptet ruller under den. Klikker du fortsatt legger markøren der du klikker.
settings-typewriter-position = Linjeposisjon
settings-typewriter-position-top-third = Topp tredje
settings-typewriter-position-middle = Midten
settings-typewriter-position-bottom-quarter = Nederste kvartal
settings-highlight-scope = Høydepunkter rundt markøren
settings-highlight-scope-none = Ingen
settings-highlight-scope-sentence = Dommen
settings-highlight-scope-paragraph = Avsnitt
settings-highlight-scope-tip-none = Ikke noe er skyggelagt når du skriver.
settings-highlight-scope-tip-sentence = Skyv setningen du skriver, så det skiller seg ut fra dem rundt den.
settings-highlight-scope-tip-paragraph = Skygg hele avsnittet du skriver, for å holde passasjen du jobber med i betraktning.
settings-group-container-views = Beholdervisninger
settings-remember-view = Husk den siste visningen for hver elementtype
settings-remember-view-tip = Åpne en beholder på visningen du sist brukte for den typen
settings-remember-view-tip-more =
    En bok, del og kapittel tilbyr hver enkelt visning (dens egen side, den fulle
    manuskript, det fulle synopsis. Slå på dette og hver type åpnes på nytt
    se at du sist valgte det, f.eks. bytte ett kapittel til fullt kapittel og
    Neste kapittel du åpner viser Full kapittel også. Hver type husker sin egen mening.
settings-group-counting = Ordtelling
settings-counting-auto = Automatisk (etter språk)
settings-counting-whitespace = Del på mellomrom
settings-counting-unicode-words = Unicode ord
settings-counting-cjk-hybrid = CJK-programvare (per tegn)
settings-counting-hint = Automatisk teller kinesisk og japansk etter tegn og alle andre språk etter ord. Endre det bare hvis tellingen ser feil ut for språket ditt.
settings-group-goals-display = Vis
settings-show-characters = Vis tegntellingen i statuslinjen
settings-autosave-hint = Endringene skrives automatisk til disken mens du skriver.

settings-sec-work = Prosjekt
settings-page-structure = Struktur
settings-page-author = Forfatter
settings-field-author-name = Forfatternavn
settings-field-author-placeholder = Valgfritt
settings-field-author-hint = Viser seg på den kompilerte tittelsiden og i eksporterte filmetadata. La det være tomt for å utelate det. Dette er bokens byline. Navnet dine kommentarer er signert med er i Innstillinger · Bruker.
settings-group-chapters = Kapittel
tidy-titles-title = Tidy-kapittel titler
tidy-titles-none = Ingen kapittel eller deltittel gjentar bare sitt eget nummer.
tidy-titles-lead = { $count ->
        [one] Et kapittel eller en del har ikke noe annet enn sitt eget nummer.
       *[other] { $count } kapitler og deler har tittelen uten noe annet enn sitt eget nummer.
    }
tidy-titles-explain = Å fjerne disse titlene etterlater seg hver som heter etter nummeret boken allerede vet, samme nummer som eksporten skriver ut. Ingenting annet endrer seg, og en angre setter dem alle tilbake.
menu-document-tidy-titles = Tidy Chapter titler... (&Y)
settings-group-numbering = Nummerering
settings-number-chapters = Antall kapitler og deler
settings-number-chapters-tip = Kapittel og deler bærer et tall utarbeidt fra hvor de sitter i boken, vist ved siden av tittelen her og trykt av eksporten.
settings-number-chapters-tip-more = Nummeret lagres aldri i tittelen, så det forblir riktig når du omordner, setter inn eller sletter. Slå av dette og eksporten skriver ut titler alene, uansett hva eksportstilen ber om. For å forlate ett kapittel ut (en prolog, en interlude), bruk Inspectors nummereringsbryter i stedet: som holder det i boken, men stopper det å ta et tall.
settings-part-resets-chapter = Start kapitlsnumrene på nytt i hver del
settings-part-resets-chapter-tip = Av som standard: kapitler går rett på tvers av deler av en bok, så - Del 2 - åpner på kapittel 11.
settings-part-resets-chapter-tip-more = Det er den vanlige handelspraksisen, og hva en leser forventer. Slå på den for en bok hvis deler er ment å lese som separate volumer, hvor hver og en åpner på kapittel 1.
settings-chapter-flat = Flat kapitler
settings-chapter-flat-hint = På: Et kapittel er et enkelt element. Du skriver inn det, og det har ingen scener. Av: Et kapittel er en mappe. Du skriver fortsatt i det, men det kan også holde scener. Nye kapitler følger denne innstillingen; eksisterende de konverterer via Fremme.

settings-styles-builtin = Innebygde stiler
settings-styles-user = Mine stiler
settings-styles-builtin-badge = Innebygget
settings-styles-duplicate = Dupliser
settings-styles-edit = Rediger
settings-styles-delete = Slett
settings-styles-import = Importer…
settings-styles-export = Eksporter…
settings-styles-copy-suffix = (kopi)
settings-styles-json-filter = Eksporter stil
settings-styles-editor-title = Rediger stil
settings-styles-editor-none = Velg en egendefinert stil for å redigere den, eller kopiere en innebygd.
settings-styles-imported = Stil importert
settings-styles-import-failed = Kunne ikke importere stil
settings-styles-exported = Style eksportert
settings-styles-export-failed = Kunne ikke eksportere stil
settings-styles-field-name = Navn
settings-styles-field-chapters = Kapitteloverskrifter
settings-styles-field-parts = Del overskrifter
settings-styles-field-scene-break = Scenepause
settings-styles-field-major-scene-break = Stor scenepause
settings-styles-field-spacing = Linjeavstand
settings-styles-field-justify = Justify tekst
settings-styles-field-notes = Inkluder notater
settings-styles-field-synopses = Inkludere synopser
settings-styles-field-scene-titles = Inkluder scenetitler
settings-styles-field-epigraphs = Inkluder epigrafer
settings-styles-field-paratexts = Inkluder paratekster
settings-styles-field-epigraph-placement = Epigrafposisjon
settings-styles-epigraph-after = Etter tittelen
settings-styles-epigraph-before = Før tittelen
settings-styles-field-footnotes = Inkluder fotnoter
settings-styles-field-footnote-numbering = STORE nummerering
settings-styles-footnote-numbering-continuous = Kontinuerlig
settings-styles-footnote-numbering-per-chapter = Start hvert kapittel på nytt
settings-styles-footnote-numbering-per-book = Start om hver bok
settings-styles-field-images = Bilder
menu-image = Bilde (&I)
settings-styles-images-beside = Ved siden av dokumentet
settings-styles-images-embed = Inne i dokumentet
settings-styles-images-omit = La oss være
settings-styles-group-round-trip = Sender til en redaktør
settings-styles-field-comments = Inkluder kommentarer
settings-styles-field-round-trip-marks = Inkludere runde markører
settings-styles-round-trip-hint = DOCX og ODT er kun usynlige identifikatorer som lar en fil oppdatere dette prosjektet i stedet for å lande ved siden av det som en andre kopi.
settings-styles-group-pages = Sider
settings-styles-field-cover = Åpne med dekselet
settings-styles-field-word-count = Ord teller på tittelsiden
settings-styles-field-page-books = Ny side i hver bok
settings-styles-field-page-parts = Ny side i hver del
settings-styles-field-page-chapters = Ny side i hvert kapittel
settings-styles-field-page-paratexts = Ny side i hver paratekst
settings-styles-heading-none = Ingen overskrift
settings-styles-heading-numbered = Kun nummer
settings-styles-heading-title = Kun tittel
settings-styles-heading-both = Nummer + tittel
settings-styles-break-blank = Tom linje
settings-styles-break-none = Ingen
settings-styles-spacing-single = Enkel
settings-styles-spacing-onehalf = 1½
settings-styles-spacing-double = Dobbel

welcome-title = Velkommen til Skribisto
welcome-version = Versjon { $version }
welcome-update-available = Versjon { $version } er tilgjengelig
welcome-update-tip = Åpner nedlastingssiden i nettleseren din.
welcome-search = Søkeprosjekter
welcome-open = Åpne
welcome-new-work = Nytt prosjekt
welcome-create-from = Opprett fra...
welcome-recent-works = Nylige prosjekter
welcome-empty-recents = Ingen nylige prosjekter ennå.
welcome-no-matches = Ingen nylige prosjekt matcher ditt søk.
welcome-learn-soon = Guider og tips kommer snart.
welcome-about-blurb = Skribisto, en Rust + Teksilo omskriving av skriveappen.
welcome-tagline = En stille sted å skrive lange ting.*
welcome-github = GitHub
welcome-discord = Discord
nav-works = Prosjekter
nav-examples = Eksempler
nav-learn = Lære
nav-about = Om

synopsis = Sammendrag
epigraph = Motto
pane-manuscript = Manuskript
corkboard = Oppslagstavle
overview = Oversikt

corkboard-card-count = { $count ->
    [one] { $count } kort
   *[other] { $count } kort
}
corkboard-child-count = { $count ->
    [one] { $count } element
   *[other] { $count } varer
}
corkboard-view-nested = Hierarkisk
corkboard-view-flat = Flat visning
corkboard-layout-hint = Plassert viser en beholders direkte barn. Åpne et mappekort for å gå inn i det. Flat viser alle scenene i hele beholderen på en gang.
corkboard-card-size = Kortstørrelse
corkboard-search-placeholder = Filtrer kort...
corkboard-empty-title = Ingenting her ennå
corkboard-empty-hint = Bruk + Ny - ovenfor for å legge til den første.
corkboard-new = Ny
corkboard-show-card-numbers = Tal på kortene
corkboard-modal-size = Utvidet redaktørstørrelse
corkboard-scope-hint = Gjelder for hvert åpent korkbrett i hvert prosjekt.
corkboard-sort-manuscript = Manuskriptbestilling
corkboard-sort-title-asc = Title A-Z
corkboard-sort-title-desc = Tittel Z-A
corkboard-card-number = Kort { $number }

duplicate = Dupliser
duplicate-n = { $count ->
    [one] Duplikat { $count } kort
   *[other] Duplikat { $count } kort
}
set-label-n = { $count ->
    [one] Sett etikett på { $count } kort
   *[other] Sett etikett på { $count } kort
}
move-to-trash-n = { $count ->
    [one] Flytt { $count } kort til søppel
   *[other] Flytt { $count } kort til søppel
}
reveal-in-outline = Avslør i omriss

corkboard-move-to = Flytt til...
corkboard-move-to-n = { $count ->
    [one] Flytt { $count } kort til...
   *[other] Flytt { $count } kort til...
}
corkboard-move-picker-title = Flytt til...
corkboard-move-picker-empty = Ingen bindemidler ennå. Opprett en først.
corkboard-move-picker-cancel = Avbryt
corkboard-move-here = Flytt hit
corkboard-moved-ok = { $count ->
    [one] { $count } kort flyttet
   *[other] { $count } kort flyttet
}
corkboard-move-into-self = En beholder kan ikke flyttes inne i seg selv. Velg en destinasjon utenfor den.
corkboard-move-failed = Disse kortene kunne ikke flyttes dit.

overview-col-title = Tittel
overview-col-type = Type
overview-col-label = Kort beskrivelse
overview-col-tags = Etiketter
overview-col-books = Bøker
overview-col-own-words = Ord
overview-col-total-words = Totalt
overview-row-count = { $count ->
    [one] { $count } element
   *[other] { $count } varer
}
overview-search-placeholder = Filtrer elementer...
overview-expand-all = Utvid alle
overview-collapse-all = Slå sammen alle
overview-table-label = Innhold
overview-empty-title = Ingenting her ennå
overview-gone-title = Denne beholderen er borte
overview-gone-hint = Den ble flyttet til papirkurven. Gjenopprett den, eller lukk denne fanen.
overview-empty-hint = Bruk + Ny - ovenfor for å legge til den første.
corkboard-grid-label = Corkboard-kort
corkboard-rename-field = Endre navn på element
corkboard-expand-synopsis = Utvid synopsis
corkboard-synopsis-modal-title = Sammendrag
corkboard-badge-scene = Scene
corkboard-badge-chapter = Kapittel
corkboard-badge-part = Del
corkboard-badge-book = Bok
corkboard-badge-note = Notat
corkboard-badge-folder = Mappe
corkboard-badge-paratext = Paratekst
corkboard-badge-text = Tekst
corkboard-badge-end = Slutt
settings-group-corkboard-layout = Utforming
settings-group-corkboard-cards = Kort
corkboard-show-word-count = Vis ord teller på kort
text-heading = Tekst
no-content = Dette elementet har ikke noe redigerbart innhold.
untitled = Uten navn
placeholder-title = Tittel…
placeholder-subtitle = Undertekst...
placeholder-chapter-title = Kapitteltittel...
split-editor = Del editor
close-split-view = Lukk delt visning
drop-open-here = Åpne her
drop-open-to-side = Åpent mot siden

segment-chapter = Kapittel
segment-part = Del
segment-book = Bok
segment-pace = Skrivetempo
segment-notes = Notater
segment-story-bible = Historiebibel
segment-note-own = Notat
segment-note-details = Detaljer
segment-note-in-prose = I prosa
pace-placeholder = Pace-planleggeren vises her.
pace-empty-title = Planlegg denne bokens tempo
pace-empty-body = Sett et ordmål og en deadline, og Skribisto Prosjekter ut det daglige tempoet for å komme deg dit.
pace-start-planning = Start planlegging
pace-section-schedule = Planlegging
pace-section-progress = Fremdrift
pace-advancement = Fremdrift
pace-goal = Ordmål
pace-deadline = Frist
pace-active = Pace aktiv
pace-day-mon = Man
pace-day-tue = Tir
pace-day-wed = Ons
pace-day-thu = Tor
pace-day-fri = Fre
pace-day-sat = Satt
pace-day-sun = Søn
pace-card-written = Ord skrevet
pace-card-of-goal = av målet
pace-card-rate = ord / skrivedag
pace-card-days-left = skrive dager left
pace-card-streak = dagstrimmel
pace-card-ahead = Ord i forkant av tempo
pace-card-behind = Ord bak tempoet
pace-charts-empty = Progress diagrammer vises her når ordet ditt teller er registrert på en lagring.
pace-chart-progression = Ord skrevet vs mål
pace-chart-words-per-day = Ord per dag
pace-series-actual = Faktisk
pace-series-target = Mål
pace-series-words-per-day = Ord/dag
pace-daily-target-line = Selv tempo: { $count ->
    [one] { $count } ord/dag
   *[other] { $count } ord/dag
}
pace-section-holidays = Fridager
pace-section-milestones = Milepæler
pace-holidays-none = Ingen helligdager. Hver planlagt dag teller.
pace-holiday-label = Ferienavn
pace-add-holiday = Legg til
pace-remove = Fjern
pace-milestones-none = Ingen milepæler ennå. Sett en på en del eller kapittel i inspektøren.
full-chapter = Fullstendig kapittel
full-part = Full del
full-book = Full bok
full-synopsis = Fullt sammendrag
rename = Gi nytt navn
set-label = Sett etikett
insert-scene = Sett inn scene
insert-chapter = Sett inn kapittel
split-scene = Del scene
move-up = Flytt opp
move-down = Flytt ned
merge-with-previous = Føy sammen med tidligere
move-to-trash = Flytt til papirkurven
rename-chapter = Endre navn på kapittel
add-scene = Legg til ny scene
add-chapter = Legg til nytt kapittel
new-scene-title = Ny scene
placeholder-scene-name = Scenenavn...
menu-cut = Klipp ut
menu-copy = Kopier
menu-paste = Lim inn
menu-paste-unformatted = Lim inn uten formatering
menu-select-all = Velg alle
format-bold = Fet
format-italic = Italisk
format-underline = Understreking
format-strikethrough = Gjennomstreking

binder = Dokumenttre
no-work = Ingen prosjekt lastet
no-recent-works = Ingen nye prosjekter
switcher-open-section = For tiden åpen
switcher-recent-section = Nylig
switcher-this-window = dette vinduet
open-project-title = Åpent prosjekt
open-project-question = Hvordan ønsker du å åpne{ $title }"?
open-in-new-window = Åpne i nytt vindu
open-here = Åpne her

binder-all = Alle Binders
binder-show-all = Vis alle bindemidler
binder-new = Nyt bindemiddel...
binder-default-name = Nyt bindemiddel
binder-item-count = { $count ->
    [one] { $count } element
   *[other] { $count } varer
}
binder-search-placeholder = Filtrer konturen...
binder-search-scope = Søk i alle bindemidler
binder-trash-confirm-title = Flytte bindemiddel til søppel?
binder-trash-confirm-text = "{ $name }- og alle dets elementer vil bli flyttet til søppelet.

dialog-rename = Gi nytt navn
dialog-new-binder = Nyt bindemiddel
dialog-set-label = Sett etikett
dialog-new-scene = Ny scene
close-work-question = Lagre endringer før prosjektet lukkes?
quit-question = Lagre endringer før du avslutter?
quit-save-work-question = Lagre endringer i { $title } før du slutter?
unsaved-changes = Prosjektet har ulagrede endringer.
new-work-unsaved-question = Lagre endringer før du oppretter et nytt prosjekt?
open-work-unsaved-question = Lagre endringer før du åpner et annet prosjekt?
switch-backup-discard-title = Forkast endringer i denne sikkerhetskopien?
switch-backup-discard-text = Endringer i en sikkerhetskopi kan ikke lagres på den. Bruk Lagre som eller gjenopprett for å holde dem, eller kaste dem og åpne det andre prosjektet.
switch-save-failed = Prosjektet kunne ikke reddes, så det ble ikke erstattet: { $error }
switch-save-not-started = Prosjektet kunne ikke reddes, så det ble ikke erstattet.
close-save-failed = Prosjektet kunne ikke lagres, så det var ikke stengt: { $error }
close-save-not-started = Prosjektet kunne ikke reddes, så det var ikke stengt.
save-not-started = Prosjektet kan ikke reddes.

could-not-open-work = Kunne ikke åpne{ $file }"
could-not-open-work-too-new = "{ $file } Trenger en nyere Skribisto
could-not-open-work-too-new-detail = Reddet av Skribisto format { $written_by }Å åpne den trenger format { $requires }  eller nyere, og denne bygningen støtter opp til format { $supported } Oppdater Skribisto  å åpne den.
could-not-open-work-importing = "{ $file }- blir skrevet av en import
could-not-open-work-importing-detail = En import skriver over dette prosjektet og er ikke ferdig. Åpne det når import er gjort: åpnet nå, det vil vise prosjektet import erstatter, og den neste lagre vil skrive det prosjektet tilbake over import.
could-not-open-example = Kunne ikke åpne eksempel: { $error }
could-not-open-link = Kunne ikke åpne { $url }: { $error }
could-not-create-work = Kunne ikke opprette prosjekt: { $error }
saving-as-file = Sparer som { $target }...
saving-as-folder = Sparer som { $target }/...
saved-as = Sparet til { $target }
save-error = Kunne ikke lagre: { $error }
save-flush-failed = Redningen gikk gjennom, men disse dokumentene kunne ikke overføres til den og er fortsatt uløst: { $titles } ({ $error })
backup-error = Kunne ikke sikkerhetskopiere: { $error }
backing-up = Sikkerhetskopiering...
backup-nothing-open = Ingen prosjekt er åpent for å sikkerhetskopiere.
backup-already-running = En sikkerhetskopi er allerede i gang.
backup-complete = Backup komplett ({ $ok } lagret, { $skipped } allerede gjeldende)
backup-partial = Sikkerhetskopi ferdig: { $ok } lagret, { $failed ->
    [one] { $failed } destinasjon mislyktes
   *[other] { $failed } destinasjoner mislyktes
}
backup-no-destination-title = Ingen sikkerhetskopiering er tilgjengelig
backup-no-destination-text = Ingen av de konfigurerte sikkerhetskopieringsdestinasjonene kan nås (for eksempel kan en ekstern stasjon bli avkoblet). Koble den inn og prøv på nytt, eller avslutte uten sikkerhetskopiering.
backup-failed-close-title = Sikkerhetskopien kunne ikke lagres
backup-failed-close-text = Ingen sikkerhetskopi kan skrives før stenging. Hver destinasjon mislyktes (stasjonen kan ha blitt fjernet, eller den kan være full eller skrivebeskyttet). Rett den og prøv på nytt, eller avslutt uten å sikkerhetskopiere.

backup-choice-title = Sikkerhetskopifil
backup-choice-heading = Du har åpnet en sikkerhetskopi
backup-choice-subtitle = Dette er en punkt-i-tid backup av et prosjekt.
backup-choice-subtitle-dated = Sikkerhetskopi tatt { $date }.
backup-choice-body = Du kan åpne den og redigere fritt, men endringer kan bare holdes med Lagre As. Den opprinnelige prosjektfilen er ikke rørt. Eller gjenopprett dette prosjektet for å nøyaktig denne sikkerhetskopien.
backup-choice-open = Åpne sikkerhetskopien
backup-choice-restore = Gjenopprett dette prosjektet til dette punktet...
backup-choice-not-a-backup = Åpne den normalt
backup-banner-title = Sikkerhetskopi: endringer kan ikke lagres her
backup-banner-description = Bruk Lagre Som for å holde dine redigeringer i en ny fil, eller gjenopprett for å erstatte det opprinnelige prosjektet med denne sikkerhetskopien.
backup-banner-restore = Gjenopprett…
backup-banner-save-as = Lagre som...
backup-restore-original-missing = Kan ikke finne det opprinnelige prosjektet å gjenopprette over. Bruk Lagre som for å holde denne sikkerhetskopien som et nytt prosjekt i stedet.
backup-restore-close-elsewhere-title = Prosjektet åpnes i et annet vindu
backup-restore-close-elsewhere-text = Prosjektet du gjenoppretter er åpent i et annet vindu. Vennligst lukk det der først, så prøv igjen.
backup-restore-focus-window = Fokuser vinduet
backup-restore-confirm-title = Gjenopprette denne sikkerhetskopien?
backup-restore-confirm-text = Den aktuelle versjonen av prosjektet vil bli kopiert til side som en sikkerhetskopi før den erstattes med denne.
backup-restore-confirm-ok = Gjenopprett
backup-restore-error = Kunne ikke gjenopprette: { $error }
backup-restored-ok = Prosjektet er restaurert.
backup-restored-with-safety = Prosjektet gjenopprettet. Din forrige versjon ble lagret til { $path }.
close-backup-discard-title = Forkast endringer i denne sikkerhetskopien?
close-backup-discard-text = Endringer i en sikkerhetskopi kan ikke lagres på den. Bruk Lagre som for å holde dem, eller kassere og stenge.
quit-backup-discard-title = Forkast endringer og avslutt?
quit-backup-discard-work-question = Kaste endringer til { $title } og slutte?
quit-backup-discard-text = Endringer i en sikkerhetskopi kan ikke lagres på den. Bruk Lagre som for å beholde dem, eller kassere og avslutte.
backup-nudge-text = Ingen backups er satt opp for dette prosjektet.
backup-nudge-action = Sett opp sikkerhetskopier...

menu-backups-list = Sikkerhetskopier… (&K)
backups-title = Sikkerhetskopier
backups-loading = Søker etter sikkerhetskopier...
backups-empty = Ingen sikkerhetskopier funnet for dette prosjektet ennå.
backups-open = Åpne
backups-reveal = Avsløring
backups-delete = Slett denne sikkerhetskopien
backups-delete-confirm-title = Slette denne sikkerhetskopien?
backups-delete-confirm-text = «{ $name }» slettes permanent. Dette kan ikke angres.
backups-delete-error = Kunne ikke slette sikkerhetskopien: { $error }
backups-refresh = Oppfrisk
backups-close = Lukk

settings-page-backup = Sikkerhetskopieringsstandarder
settings-page-work-backup = Sikkerhetskopier
settings-backup-general-title = Standard sikkerhetskopiinnstillinger
settings-backup-work-title = sikkerhetskopier for dette prosjektet
settings-backup-inherit = Bruk de generelle sikkerhetskopiinnstillingene
settings-backup-inheriting = Dette prosjektet bruker de generelle sikkerhetskopiinnstillingene.
settings-backup-none-hint = Ingen automatiske sikkerhetskopier er satt opp (hver utløser er av).
settings-backup-last = Siste sikkerhetskopi: { $date }
settings-backup-last-never = Ingen sikkerhetskopier ennå.
settings-backup-open-list = Åpne backup-listen...
settings-backup-triggers = Når å sikkerhetskopiere
settings-backup-on-close = Sikkerhetskopier når prosjektet avsluttes
settings-backup-on-open = Tilbake når du åpner prosjektet
settings-backup-interval = Sikkerhetskopiere regelmessig, hver
settings-backup-destinations = Sikkerhetskopi destinasjoner
settings-backup-default-location = Standard plassering
settings-backup-usage-since = { $count ->
    [one] { $count } backup · { $size } · eldste { $oldest }
   *[other] { $count } backups · { $size } · eldste { $oldest }
}
settings-backup-usage = { $count ->
    [one] { $count } backup · { $size }
   *[other] { $count } sikkerhetskopier · { $size }
}
settings-backup-usage-empty = Ingenting holdt her ennå
settings-backup-usage-measuring = Måling...
settings-backup-reveal-root = Vis mappe
settings-backup-dest-none = Ingen destinasjoner. Backups lagres ved siden av prosjektet.
settings-backup-dest-remove = Fjern
settings-backup-dest-add = Legg til mappe...
settings-backup-dest-refresh = Oppfrisk
settings-backup-retention = Hvor mange å holde
settings-backup-retention-tiered = Trinnvis
settings-backup-retention-keep-n = Hold siste N
settings-backup-retention-tip = Hvor gamle backups er skuret.
settings-backup-retention-tip-more = Tiered holder én sikkerhetskopi i timen i en dag, per dag i en uke, i uken i en måned, og per måned utover, så den siste historien forblir tett og gammel historie tynner ut. - Hold siste N - bare holder N siste kopier. I begge modusene de nyeste kopiene (minimum nedenfor) blir alltid lagret.
settings-backup-gfs-hourly = Hourly (siste 24 t)
settings-backup-gfs-daily = Daglig (siste uke)
settings-backup-gfs-weekly = Ukelig (siste måned)
settings-backup-gfs-monthly = Månedlig
settings-backup-keep-n = Antall å holde
settings-backup-min-keep = Hold alltid i det minste
settings-backup-dedup = Hopp over en sikkerhetskopi når ingenting har endret seg

new-work-title = Nytt prosjekt
new-work-close = Lukk
new-work-name = Prosjektnavn
new-work-name-placeholder = Uten navn
new-work-author = Forfatter
new-work-author-placeholder = Valgfritt
new-work-format = Format
new-work-single-file = En enkelt fil
new-work-bundle = Samlefil
new-work-convert-later = Du kan konvertere mellom formater senere.
new-work-location = Beliggenhet
new-work-will-create = Vil opprette
new-work-language = Standardspråk
new-work-language-hint = Brukes på nye tekster stavekontroll. Hver tekst kan endres til et annet språk.
new-work-template = Mal
new-work-template-none = Ingen
new-work-template-empty-novel = Tomt novell
new-work-template-light-novel = Light Novel
new-work-template-novel = Roman
new-work-template-novel-in-parts = Novel i deler
new-work-template-notebook = Notatbok
new-work-cancel = Avbryt
new-work-create = Opprett prosjekt
new-work-step-details = Detaljer
new-work-step-language = Språkstruktur
new-work-step-template = Mal
new-work-back = Tilbake
new-work-next = Neste
new-work-documents-title = Nytt prosjekt fra dokumenter
new-work-step-import = Importer
new-work-create-and-import = Opprett import...
new-work-documents-next-title = Dine dokumenter kommer neste
new-work-documents-next-body = Opprettelse av prosjektet åpner importguiden over det, der du velger dokumentene, se gjennom strukturen Skribisto leser fra dem, og velg hvor den lander. Ingenting er skrevet i prosjektet før du bekrefter det der.
new-work-documents-no-template = Dette prosjektet begynner tom på hensikt: ingen mal, så de importerte dokumentene er det eneste i det.
new-work-documents-chapter-scene-hint = Gjelder kapitlene import skaper. Du kan endre det senere i prosjektets innstillinger.
new-work-single-file-desc = Et .skrib arkiv (zip). Bærbar, lett å sikkerhetskopiere.
new-work-bundle-desc = En mappe med alle tekstressurser. Venner til versjonskontroll.
new-work-template-none-count = tom bindemiddel
new-work-template-empty-novel-count = 1 kapittel
new-work-template-light-novel-count = 15 kapitler
new-work-template-novel-count = 20 kapitler
new-work-template-novel-in-parts-count = 3 deler, 24 kapitler
new-work-template-notebook-count = free-form notater
new-work-chapter-scene = Flat kapitler
new-work-chapter-scene-tip = Hvert kapittel er et enkelt element du skriver rett inn i, uten scener under det. La dette være av for den klassiske utformingen, der et kapittel er en mappe: du skriver rett inn i det også, men det kan også holde scener.
new-work-chapter-scene-tip-more = Du skriver inn i et kapittel uansett. Den eneste forskjellen er om det kan *innehalde* scener. Skribisto bindetre er bare organisatorisk, så begge layouter kompilerer til samme bok, du kan blande dem fritt, og fremme konverterer et kapittel mellom de to uten å miste et ord.
new-work-name-required = Skriv inn navnet på prosjektet
new-work-name-invalid = Dette navnet har ingen brukbare tegn
new-work-location-required = Velg et sted
new-work-location-missing = Denne mappen eksisterer ikke
new-work-location-not-folder = Denne banen er ikke en mappe
new-work-location-readonly = Denne mappen er ikke skrivbar
new-work-target-exists = Et prosjekt med dette navnet er allerede i denne mappen. Oppretting av prosjektet vil spørre før det erstattes
new-work-target-is-folder = En mappe med dette navnet er allerede på denne plasseringen. Velg et annet navn
new-work-overwrite-title = Erstatning av eksisterende prosjekt?
new-work-overwrite-text = "{ $name }- finnes det allerede i denne mappen. Bytt ut det med det nye prosjektet?
new-work-target-open-title = Dette prosjektet er åpent
new-work-target-open-text = "{ $name }Åpent i Skribisto. å opprette et nytt prosjekt på sin plass vil erstatte prosjektet du jobber på, og din neste lagre ville skrive det prosjektet tilbake over den nye. Velg et annet navn eller mappe, eller lukk prosjektet først.
target-importing-title = En import skriver dette prosjektet
target-importing-text = En import skriver fortsatt -{ $name }Vent til den er ferdig, eller velg et annet navn.
import-plume-busy-title = Et nytt Plume Creator-prosjekt blir importert
import-plume-busy-text = Plume Creator prosjekter importeres en om gangen. Vent til import underveis er ferdig, eller avbryt den, og start deretter denne igjen.
import-manuskript-busy-title = Et nytt Manuskript-prosjekt blir importert
import-manuskript-busy-text = Manuskript prosjekter importeres en om gangen. Vent til import underveis er ferdig, eller avbryt den, og start deretter denne igjen.
backup-restore-importing-text = En import skriver fortsatt -{ $name }-, prosjektet denne sikkerhetskopien vil bli gjenopprettet. Gjenopprette sikkerhetskopien når importen er ferdig.

new-work-manuscript = Manuskript
new-work-notes = Notater
new-work-research = Forskning
new-work-notebook = Notatbok
new-work-scene = Scene
new-work-note = Notat
new-work-front-matter = Front materie
new-work-back-matter = Tilbake Matter
new-work-paratext = Bokstruktur
new-work-paratext-none = Ingen struktur
new-work-paratext-hint = Fronten og tilbake spiller en tradisjon åpner og stenger en bok med. Du kan flytte, omdøbe eller slette noe av den etterpå.
new-work-tags = Etiketter
new-work-tags-none = Ingen etiketter
new-work-tags-hint = En startpalett for tagging tegn, steder og resten. Valgfritt, og hver tag kan endre navn, omfarget eller slettet etterpå.
new-work-note-templates = Notemaler
new-work-note-templates-none = Ingen maler
new-work-note-templates-hint = Formen en historiebibelnotat starter i. Valgfritt, og hver mal kan redigeres eller slettes etterpå.
new-work-characters = Personer
new-work-places = Steder

import-plume-title = Import Plume Creator prosjekt
import-plume-close = Lukk
import-plume-source = Plume-prosjekt
import-plume-source-hint = Velg en .plume eller .plume_backup fil (noen Plume Creator versjon).
import-plume-location = Målmappe
import-plume-name = Filnavn
import-plume-name-placeholder = Prosjektnavn
import-plume-will-create = Vil opprette
import-plume-trash-warning = · Trashed / slettede gjenstander er ikke migrert.
import-plume-cancel = Avbryt
import-plume-import = Importer
import-plume-source-required = Velg en Plume prosjektfil
import-plume-source-missing = Denne filen eksisterer ikke
import-plume-source-not-file = Denne banen er ikke en fil
import-plume-location-required = Velg en målmappe
import-plume-location-missing = Denne mappen eksisterer ikke
import-plume-location-not-folder = Denne banen er ikke en mappe
import-plume-location-readonly = Denne mappen er ikke skrivbar
import-plume-name-required = Skriv inn et filnavn
import-plume-name-exists = En fil med dette navnet eksisterer allerede her. Import bekrefter overskriving
import-plume-overwrite-title = Erstatt eksisterende fil?
import-plume-overwrite-text = "{ $name }- allerede eksisterer. Erstatte det med det importerte prosjektet?
import-plume-manuscript-binder = Manuskript
import-plume-story-bible-binder = Historiebibel
import-plume-progress-title = Importerer Plume-prosjekt...
import-plume-cancel-import = Avbryt
import-plume-cancelled = Importering avlyst
import-plume-done = Importert { $imported ->
    [one] { $imported } element.
   *[other] { $imported } varer.
 } { $skipped ->
    [one] { $skipped } var ikke migrert.
   *[other] { $skipped } var ikke migrert.
}
import-plume-open-now = Åpne nå
import-plume-warnings = { $count ->
    [one] En ting kan ikke importeres nøyaktig
   *[other] { $count } ting kan ikke importeres nøyaktig
}
import-plume-details = Detaljer
import-plume-warnings-title = Importer advarsler
import-plume-error-title = Kunne ikke importere prosjektet
import-plume-nested-too-deep = Prosjektet ble ikke importert: dets{ $part }En del er hekket mer enn { $limit }  nivå dypt. Nei Plume Creator  Prosjektet kommer i nærheten og leser det kan krasje Skribisto Ingenting ble skrevet.
import-plume-declares-entities = Prosjektet ble ikke importert: dets{ $part }Del erklærer XML-enheter, som ikke Plume Creator  prosjektet gjør. Å utvide dem kan ta mer minne enn datamaskinen har og krasjer Skribisto Ingenting ble skrevet.
import-plume-error-details = Detaljer

import-manuskript-title = Import Manuskript prosjekt
import-manuskript-close = Lukk
import-manuskript-source = Manuskript prosjekt
import-manuskript-source-hint = Velg .msk-filen eller prosjektmappen (noen Manuskript-versjon).
import-manuskript-source-file = Velg fil ...
import-manuskript-source-folder = Velg mappe...
import-manuskript-location = Målmappe
import-manuskript-name = Filnavn
import-manuskript-name-placeholder = Prosjektnavn
import-manuskript-will-create = Vil opprette
import-manuskript-cancel = Avbryt
import-manuskript-import = Importer
import-manuskript-source-required = Velg et Manuskript-prosjekt
import-manuskript-source-missing = Denne filen eller mappen eksisterer ikke
import-manuskript-location-required = Velg en målmappe
import-manuskript-location-missing = Denne mappen eksisterer ikke
import-manuskript-location-not-folder = Denne banen er ikke en mappe
import-manuskript-location-readonly = Denne mappen er ikke skrivbar
import-manuskript-name-required = Skriv inn et filnavn
import-manuskript-name-exists = En fil med dette navnet eksisterer allerede her. Import bekrefter overskriving
import-manuskript-overwrite-title = Erstatt eksisterende fil?
import-manuskript-overwrite-text = "{ $name }- allerede eksisterer. Erstatte det med det importerte prosjektet?
import-manuskript-manuscript-binder = Manuskript
import-manuskript-story-bible-binder = Historiebibel
import-manuskript-characters-group = Personer
import-manuskript-world-group = verden
import-manuskript-plots-group = Handlingslinjer
import-manuskript-project-info-note = Prosjektinformasjon
import-manuskript-summary-note = Sammendrag
import-manuskript-importance-minor = Mindre viktig
import-manuskript-importance-secondary = Sekundær
import-manuskript-importance-main = Hoved
import-manuskript-progress-title = Importerer Manuskript prosjekt...
import-manuskript-cancel-import = Avbryt
import-manuskript-cancelled = Importering avlyst
import-manuskript-done = { $imported ->
    [one] { $imported } element importert.
   *[other] { $imported } varer importert.
 } { $revisions ->
    [0] { "" }
    [one] { $revisions } tidligere versjon kom med den.
   *[other] { $revisions } tidligere versjoner kom med dem.
}
import-manuskript-open-now = Åpne nå
import-manuskript-warnings = { $count ->
    [one] 1 ting å vite om denne importen
   *[other] { $count } ting å vite om denne importen
}
import-manuskript-details = Detaljer
import-manuskript-warnings-title = Om denne importen
import-manuskript-error-title = Kunne ikke importere prosjektet
import-manuskript-nested-too-deep = Prosjektet ble ikke importert: dens fil{ $part }- er hekket mer enn { $limit }  nivå dypt. Nei Manuskript  Prosjektet kommer i nærheten og leser det kan krasje Skribisto Ingenting ble skrevet.
import-manuskript-folders-too-deep = Prosjektet ble ikke importert: en av filene sitter mer enn { $limit } mapper dypt. Ingen Manuskript prosjekt kommer nær det, og lese det kan krasje Skribisto. Ingenting ble skrevet.
import-manuskript-declares-entities = Prosjektet ble ikke importert: dens fil{ $part }- erklærer XML-enheter, som ikke Manuskript  prosjektet gjør. Å utvide dem kan ta mer minne enn datamaskinen har og krasjer Skribisto Ingenting ble skrevet.
import-manuskript-error-details = Detaljer

import-target-open-title = Dette prosjektet er åpent
import-target-open-text = "{ $name }Åpent i Skribisto. Importere over det ville erstatte prosjektet du jobber på, og din neste lagre ville skrive det prosjektet tilbake over import. Velg et annet filnamn, eller steng prosjektet først.
import-warnings-announcement = { $result } { $notice }.

export-title = Eksporter
export-close = Lukk
export-scope-label = Hva
export-format-label = Format
export-style-label = Stil
export-path-label = Fil
export-browse = Bla gjennom...
export-show-non-exportable = Vis ikke-eksporterbare
export-choose-empty = Ingen prosjekt å velge mellom.
export-cancel = Avbryt
export-export = Eksporter
export-save-dialog-title = Eksporter til fil
export-section-what = Hva å eksportere
export-section-style = Stil
export-section-destination = Destinasjon
export-custom-selection = Egendefinert utvalg
export-selected-count = { $count } valgt
export-chip-chapters-none = Kapittel: Ingen
export-chip-chapters-numbered = Kapittler: nummerert
export-chip-chapters-title = Kapittel: Bare tittel
export-chip-chapters-both = Kapittel: nummer + tittel
export-chip-scene-break-glyph = Scenepause: { $glyph }
export-chip-scene-break-blank = Scenepause: tom linje
export-chip-scene-break-none = Scenepause: none
export-chip-major-break-glyph = Stor pause { $glyph }
export-chip-major-break-blank = Stor pause: tom linje
export-chip-major-break-none = Stor pause: ingen
export-chip-major-break-same = Begge pausenivåene er like
export-chip-spacing-single = Avstand: enkelt
export-chip-spacing-onehalf = Avstand: 11⁄2
export-chip-spacing-double = Avstand: dobbel
export-chip-notes-included = Noter inkludert
export-chip-notes-excluded = Noter utelukket
export-format-docx = Ord
export-format-odt = LibreOffice
export-format-html = HTML
export-format-markdown = Markdown
export-format-djot = Djot
export-format-text = Tekst
export-format-latex = LaTeX
export-format-epub = EPUB
export-format-pdf = PDF
export-overwrite-title = Erstatt eksisterende fil?
export-overwrite-text = "{ $name }- Er det allerede til?
export-progress-title = Eksporterer...
export-cancelled = Eksportering avlyst
export-done = Eksportert { $count ->
    [one] { $count } element
   *[other] { $count } varer
}
export-open-file = Åpne
export-show-in-folder = Vis i mappe
export-error-title = Kunne ikke eksportere
export-error-details = Detaljer

backup-progress-start = Begynner...
backup-progress-retention = Rengjøring av gamle backups...
backup-progress-done = Ferdig
backup-progress-destination = Destinasjon { $i } av { $n }
backup-details = Detaljer
backup-issues-title = Sikkerhetskopieringsproblemer
backup-failed-title = Sikkerhetskopiering mislyktes
backup-complete-prune-warning = Sikkerhetskopier komplett ({ $ok } lagret, { $skipped } allerede gjeldende). Noen gamle sikkerhetskopier kunne ikke fjernes

search = Søk
search-query-placeholder = Søk…
search-replace-placeholder = Bytt ut med...
search-replace-toggle = Bytt ut
search-replace-all = Bytt ut alle
search-opt-case = Skill mellom store og små bokstaver
search-opt-whole-word = Hele ordet
search-opt-diacritics = Match aksenter
search-scope-body = Kroppen
search-scope-title = Tittel
search-scope-synopsis = Sammendrag
search-scope-label = Kort beskrivelse
search-facet-book = Bøker
search-facet-part = Deler
search-facet-chapter = Kapittel
search-facet-scene = Scenes
search-facet-note = Notater
search-facet-folder = Mapper
search-tip-case = Match case: behandle store bokstaver og små bokstavar som forskjellig, så - Elena - og -elena - er separate kamper.
search-tip-whole-word = Hele ordet: matcher bare komplette ord, så - katt - er ikke funnet inne i kategorien -
search-tip-diacritics = Match aksenter: behandle aksenterte bokstaver som distinkte, så - kafé - ikke matcher - café -
search-tip-body = Kropp: Søk i prosa av scener og notater.
search-tip-title = Title: Søk i titlene på bindeelementer.
search-tip-synopsis = Synopsis: Søk i hver skriveartikkels sammendrag.
search-tip-label = Etikett: søk i kortnotatet som vises under tittelen på et bindeelement.
search-tip-comment = Kommentarer: Søk i teksten på kommentartråder og deres svar. En kommentar handler om manuskriptet i stedet for en del av det, så den har sin egen bryter, og en erstatte lauters comment matchs uncicked inntil du krysser dem.
search-tip-book = Bøker: bokbeholderen og dens begynnelse / slutten markører.
search-tip-part = Deler: delnivådeler.
search-tip-chapter = Kapittler: kapitler, på hvilken måte prosjektet lagrer dem.
search-tip-scene = Scenes: elementene som holder prosaen din.
search-tip-note = Noter: friformede notater.
search-tip-folder = Mapper: Enkel organisering av mapper og separatorer.
search-tip-paratext = En tekst som hører til boken, men ikke til dens historie: et forord, et engasjement, et etterord. Aldri talt i manuskriptet.
search-tip-preserve-case = Bevar tilfelle: En erstatning tar saken den fant, så -ELENA - blir -MARTA - og -Elena - blir - Marta -
search-tip-replace = Bytt ut: Vis erstatningsfeltet og Erstatt alle.
search-error = Søk mislyktes: { $message }
search-no-matches = Ingen kamper
search-count =
    { $matches ->
        [one] { $matches } match
       *[other] { $matches } kamper
    } in { $items ->
        [one] { $items } dokument
       *[other] { $items } dokumenter
    }
search-count-truncated =
    { $matches ->
        [one] { $matches } match
       *[other] { $matches } kamper
    } in { $items ->
        [one] { $items } dokument
       *[other] { $items } dokumenter
    } (bare de første resultatene)
search-occurrences = { $count }
search-field-body = Kroppen
search-field-title = Tittel
search-field-synopsis = Sammendrag
search-field-label = Kort beskrivelse
search-field-epigraph = Motto
search-field-comment = Kommentar
search-field-comment-reply = Svar
search-field-footnote = Fotnote
search-include-in-replace = Ta med i Erstat alle
search-collapse-all = Slå sammen alle resultater
search-replace-here = Erstatt dette
search-dismiss = Utelukkelse fra resultatene
search-undo-dismiss = Tilbake til det siste avviste resultatet
search-replace-nothing = (ingenting)
search-replace-confirm-title = Bytt ut alle kamper?
search-replace-confirm-text =
    Erstatt { $occurrences ->
        [one] { $occurrences } forekomst
       *[other] { $occurrences } hendelser
    } of "{ $query }" with "{ $replacement }" across { $items ->
        [one] { $items } dokument
       *[other] { $items } dokumenter
    } ? Du kan angre dette fra meldingen etterpå.
search-replace-done-title = Bytt ut komplett
search-replace-done =
    { $occurrences ->
        [one] { $occurrences } forekomst
       *[other] { $occurrences } hendelser
    } replaced in { $items ->
        [one] { $items } dokument
       *[other] { $items } dokumenter
    } .
search-replace-done-skipped =
    { $occurrences ->
        [one] { $occurrences } forekomst
       *[other] { $occurrences } hendelser
    } replaced in { $items ->
        [one] { $items } dokument
       *[other] { $items } dokumenter
    }. { $skipped ->
        [one] { $skipped } felt
       *[other] { $skipped } felt
    } Hoppet over (endre siden søket).
search-replace-undo = Angre
search-replace-failed-title = Utskifting mislyktes
search-replace-skipped-title = Ingenting erstattet
search-replace-skipped-body = Antall tekster som er endret siden søket: { $fields }. Disse tekstene ble ikke endret. Søk på nytt for å se hvor ordene er nå.
search-replace-undo-failed-title = Angre mislyktes
search-preview = Forhåndsvisning
search-preview-empty = Velg et resultat for å forhåndsvise det her
search-preview-no-prose = Denne kampen har ingen redigerbar tekst
search-preview-prompt = Hvis du vil se en forhåndsvisning her, kjører du et søk.
search-preview-open-search = Søk i prosjektet
search-preview-footnote-prompt = Denne kampen er i en fotnotes egen tekst. Åpne den i dokken for å se og redigere den.
search-preview-open-footnotes = Åpen KJØP

find-placeholder = Finn i dette dokumentet...
find-count = { $current } av { $total }
find-no-results = Ingen resultater
find-close = Lukk søket
find-previous = Forrige kamp (Shift+Enter)
find-next = Neste kamp (Enter)
find-opt-case = Skill mellom store og små bokstaver
find-opt-whole-word = Hele ordet
find-replace-toggle = Bytt ut (Ctrl+R)
find-replace-placeholder = Bytt ut med...
find-replace = Erstatt
find-replace-all = Bytt ut alle
find-preserve-case = Ta vare på saken

dict-download-title = Laster ned { $name }...
dict-download-done = Installert { $name }
dict-download-failed = Kunne ikke laste ned { $name }: { $error }
dict-removed = Fjernet { $name }
dict-accept-first = Godta lisens før nedlasting { $name }
settings-dict-tab-installed = Installert
settings-dict-tab-get-more = Få mer
settings-dict-tab-personal = Personlige ord
dict-installed-empty = Ingen ordbøker funnet på denne datamaskinen ennå.
dict-get-more-search = Søk i språk
dict-system-badge = på systemet ditt
dict-unusable-badge = Ubrukelig
dict-download-button = Last ned
dict-downloading = Laster ned...
dict-installed-label = Installert
dict-remove = Fjern
dict-view-license = Vis lisens
dict-approx-size = ~{ $size }
dict-personal-empty = Ingen personlige ord i dette prosjektet ennå.
dict-personal-add = Legg til
dict-personal-placeholder = Legg til et ord...
dict-missing-toast = { $count ->
    [one] Dette prosjektet bruker { $count } ordbok du ikke har installert
   *[other] Dette prosjektet bruker { $count } ordbøker du ikke har installert
}
dict-missing-action = Få ordbøker
dict-license-title = { $name } lisens
dict-license-accept = Accepter nedlasting
dict-license-cancel = Avbryt
dict-license-close = Lukk
dict-add-button = Legg til ordbok...
dict-add-title = Legg til en ordbok
dict-add-close = Lukk
dict-add-name = Navn
dict-add-name-placeholder = For eksempel Min latinske ordbok
dict-add-code = Språkkode
dict-add-code-placeholder = f.eks. la eller fr-FR-x-custom
dict-add-code-hint = En kort tag av ditt valg. Dette er hva du velger som dokumentspråk.
dict-add-aff = Affix-fil (.aff)
dict-add-dic = Ordliste (.dic)
dict-add-submit = Legg til
dict-add-cancel = Avbryt
dict-add-name-required = Gi ordboka et navn.
dict-add-code-required = Skriv inn en språkkode.
dict-add-code-invalid = Bruk kun bokstaver, siffer og - _ .
dict-add-code-reserved = Det er en innebygd ordbok kode. Velg en annen.
dict-add-file-required = Velg en fil.
dict-add-file-missing = Denne filen eksisterer ikke.
dict-add-code-taken = En ordbok for den koden er allerede installert.
dict-add-done = Lagt til { $name }
dict-add-unusable = Disse filene er ikke en brukbar ordbok: { $error }
dict-add-failed = Kunne ikke legge til ordboka: { $error }
inspector-export = Eksporter
inspector-exportable = Inkluderes i eksport
ctx-number = Nummerer dette kapitlet (&N)
ctx-unnumber = Ikke nummerer dette kapitlet (&N)
inspector-numbering = Nummerering
inspector-numbered = Nummerert
inspector-numbered-tip = Dette kapittelet finner sted i bokens nummerering. Slå det av for en prolog, en epilog eller en interlude.
inspector-numbered-tip-more = Et unummerert kapittel forblir i boken nøyaktig som den var: dens overskrift, dens prosa og dets ordtall er urørt. Den skriver bare ikke noe tall, og bruker ikke det ene opp: kapittelet etter en prolog er kapittel ett, ikke kapittel to. Å forlate det ut av eksporten i stedet er en annen bryter, ovenfor, og at man fjerner kapitlet helt fra boken.
inspector-apply-to-children = Søk på barn
inspector-milestone = Miletone dato
inspector-milestone-none = Ingen dato
inspector-milestone-clear = Tøm
inspector-tags = Etiketter
inspector-aliases = Også kjent som
inspector-dict-language = Språk
inspector-apply-language-to-children = Bruk språket til barn
settings-page-language = Språk
settings-field-dict-language = Språk
dict-tradeoff-hint = Hvert ekstra språk aksepterer flere ord, så færre feil blir fanget.
lang-inherit-hint = Arvelig: Denne scenen bruker bokens eller prosjektets språk.
lang-pill-list = Språk
lang-pill-add = Legg til et språk
lang-pill-remove = Fjerne { $name }
lang-pill-mute = Slå stavekontroll av for { $name }
lang-pill-unmute = Slå på stavekontroll for { $name }

settings-page-personal-dictionary = Personlig ordbok
settings-user-dict-desc = Ord du legger til her behandles som riktig stavet overalt i appen, og aldri flagget igjen.
settings-user-dict-add-placeholder = Legg til et ord...
settings-user-dict-add = Legg til ord
settings-user-dict-search = Filtrer ord
settings-user-dict-count = { $count ->
    [one] { $count } ord
   *[other] { $count } ord
}
settings-user-dict-rename = Gi nytt navn
settings-user-dict-remove = Fjern
settings-user-dict-empty = Ingen ord ennå. Legg til ord stavekontrollen bør ignorere, eller importere en liste.
settings-user-dict-import = Importer…
settings-user-dict-export = Eksporter…
settings-user-dict-import-tip = Legg til ord fra en vanlig tekstfil, ett ord per linje. Eksisterende ord holdes.
settings-user-dict-export-tip = Lagre hele listen til en vanlig tekstfil, ett ord per linje.
settings-user-dict-txt-filter = Tekstfiler
settings-user-dict-duplicate = Allerede i ordboka
settings-user-dict-imported = Importert { $count ->
    [one] { $count } ord
   *[other] { $count } ord
} ({ $duplicates } allerede tilstede).
settings-user-dict-import-failed = Kunne ikke lese ordlisten: { $error }
settings-user-dict-exported = Lagret { $count ->
    [one] { $count } ord.
   *[other] { $count } ord.
}
settings-user-dict-export-failed = Kunne ikke lagre ordlisten: { $error }

settings-page-text-replacements = Tekstutskiftinger
settings-text-repl-desc = Bytt ut shorthand med hele teksten som du skriver: - btw - blir - forresten - det øyeblikket du skriver en plass eller tegnsetting.
settings-text-repl-enable = Bruk teksterstatninger i dette prosjektet
settings-text-repl-disabled-hint = Slå på dette for å definere korthånd som utvides når du skriver.
settings-text-repl-add = Legg til regel
settings-text-repl-trigger-placeholder = Forkortelse
settings-text-repl-replacement-placeholder = Hva det blir til
settings-text-repl-added = Lagt til{ $trigger }"
settings-text-repl-duplicate = "{ $trigger }- har allerede en regel
settings-text-repl-filter = Filtrerregler
settings-text-repl-count = { $n ->
    [one] 1 regel
   *[other] { $n } regler
}
settings-text-repl-row-enabled = Bruk denne regelen
settings-text-repl-delete = Slett regelen for { $trigger }
settings-text-repl-deleted = Slettet regelen for -{ $trigger }"
settings-text-repl-empty = Ingen regler ennå.
settings-text-repl-csv-filter = CSV filer
settings-text-repl-import = Importer…
settings-text-repl-export = Eksporter…
settings-text-repl-imported = Importert { $added }, hoppet { $skipped }
settings-text-repl-exported = Eksportert { $n ->
    [one] 1 regel
   *[other] { $n } regler
}

editor-menu-no-suggestions = Ingen forslag
editor-menu-add-to-dictionary = Legg til{ $word }til ordbok
editor-menu-add-words-to-dictionary = Legg til valgte ord i ordliste
editor-dict-added = Lagt til{ $word }- til din ordbok.
editor-dict-added-multi = { $count ->
    [one] Lagt { $count } ord til din ordbok.
   *[other] Lagt { $count } ord til din ordbok.
}
toast-undo = Angre

undo-superseded-title = Dette trinnet er ikke lenger det siste
undo-superseded-body = Noe annet har endret seg i dette prosjektet siden. Angre vil ta det tilbake i stedet, så det har ikke gjort det. Ingenting dette trinnet har blitt reversert. Bruk Rediger ? Angre å gå tilbake gjennom historien selv.
undo-failed = Angre mislyktes: { $error }

menu-edit = Rediger (&E)
menu-edit-undo = Angre (&U)
menu-edit-redo = Gjør om (&R)
menu-edit-undo-target = Utendørs { $target } (&U)
menu-edit-redo-target = Avrunkning { $target } (&R)
shortcut-name-edit-undo = Angre
shortcut-name-edit-redo = Gjør om
undo-target-typing = å skrive
undo-target-project = siste endring til dette prosjektet
prose-history-reset-title = Tilbakestilling av skrivehistorikk
prose-history-reset-body = { $count ->
    [one] En åpen scene ble restaurert fra en tidligere tilstand, så dens skrivehistorie gjelder ikke lenger.
   *[other] { $count } åpne scener ble restaurert fra tidligere tilstand, så deres skrivehistorie gjelder ikke lenger.
}
undo-target-trash = Flytte til søppel
undo-target-restore = gjenoppretting fra søppel
undo-target-delete-forever = Sletting for godt
undo-target-replace-all = erstatter hele prosjektet
undo-target-import = import av dokumentet
undo-target-duplicate = Duplicating
undo-target-move = bevegelig
undo-target-merge = Slå sammen to scener
undo-target-split = Dele en scene
undo-target-promote = Endre type
undo-target-tidy-titles = Å rydde opp i kapitlet titler
undo-target-import-tags = importer etiketter
undo-target-import-templates = Importere notatmaler
undo-target-create = Oppretter
undo-target-remove = Sletter
undo-target-rename = Endre navn
undo-target-edit = som redigerer
menu-edit-find = Finn… (&F)
menu-edit-find-next = Finn neste (&N)
menu-edit-find-prev = Finn tidligere (&V)
menu-edit-replace = Finne og erstatte... (&L)
menu-edit-find-in-project = Finn i prosjektet... (&J)
menu-edit-replace-in-project = Bytt ut i prosjektet... (&I)
undo-frozen = Angre - Alltid fremover - er på)
redo-frozen = Regree - alltid fremover - er på)
menu-edit-cut = Klipp ut (&T)
menu-edit-copy = Kopier (&C)
menu-edit-paste = Lim inn (&P)
menu-edit-paste-plain = Lim inn uten formatering (&W)
menu-edit-select-all = Velg alle (&A)
shortcut-name-edit-cut = Klipp ut
shortcut-name-edit-copy = Kopier
shortcut-name-edit-paste = Lim inn
shortcut-name-edit-paste-plain = Lim inn uten formatering
shortcut-name-edit-select-all = Velg alle

titlebar-spellcheck-on = stavekontroll er på. Klikk for å slutte å sjekke (F7)
titlebar-spellcheck-off = Spellekontroll er slått av. Klikk for å sjekke igjen (F7)
menu-spellcheck = Kontroller staving (&C)
menu-comments = Kommentarer (&M)
settings-page-spellcheck = Spellekontroll
settings-group-spellcheck = Spellekontroll
settings-spellcheck-enabled = Sjekk staving som jeg skriver
settings-spellcheck-hint = Understreker ord ingen installert ordbok vet. Ved å slå av dette stopper alle kontroller, i hvert prosjekt, til du slår det på igjen. For å slutte å sjekke ett språk bare, renser dets sjekkmerke på Språkfeltet i prosjektet eller et element.

menu-trash = Papirkurv (&T)
trash-title = Papirkurv
trash-empty-state = Papirkurven er tom.
trash-empty-button = Tøm papirkurv...
trash-restore = Gjenopprett (&R)
trash-restore-to = Gjenopprett til... (&T)
trash-delete-forever = Slett for alltid (&D)
trash-restored-ok = { $count } restaurert.
trash-restore-error = Kunne ikke gjenopprette: { $error }
trash-restore-orphaned = Dette elementets opprinnelige sted er borte. Velg hvor å gjenopprette det.
trash-restore-no-project = Ikke noe prosjekt er åpent, så det er ingenting å gjenopprette.
trash-delete-no-project = Ingen prosjekter er åpne, så det er ingenting å slette.
trash-empty-confirm-title = -Tøm søppelet?
trash-empty-confirm-text = { $count ->
    [one] { $count } slettes permanent.
   *[other] { $count } slettede oppføringer vil bli slettet permanent.
} De forlater søppelet for godt, og kan likevel bringe dem tilbake til du gjør noe annet.
trash-emptied-title = Papirkurv tømt
trash-emptied-body = Alt i søppelet ble slettet permanent.
trash-empty-no-project = Ikke noe prosjekt er åpent, så det er ingen søppel å tømme.
trash-delete-forever-confirm-title = Slette for alltid?
trash-delete-forever-confirm-text = { $count ->
    [one] { $count } element vil bli permanent slettet.
   *[other] { $count } varer vil bli permanent slettet.
} De forlater søppelet for godt, og kan likevel bringe dem tilbake til du gjør noe annet.
trash-deleted-title = Slettet for alltid
trash-deleted-body = { $count } permanent slettet.
trash-undo = Angre
trash-restore-picker-title = Gjenopprett til...
trash-restore-to-confirm-title = Restaurere her?
trash-restore-to-confirm-text = Restore{ $item } inn i -{ $destination }"?
trash-restore-picker-restore-here = Gjenopprett her
trash-restore-picker-cancel = Avbryt
trash-restore-picker-empty = Ingen bindemidler ennå. Opprett en først.
trash-banner-title = Denne varen er i papirkurven
trash-banner-description = Det vil ikke vises i kontur eller eksport før du gjenoppretter det.
trash-banner-restore = Gjenopprett…
trash-tab-tooltip = I papirkurv

settings-styles-sheet-yes = Ja
settings-styles-sheet-no = Nei
settings-styles-sheet-font = Skrift
settings-styles-sheet-indent = Første linjes innrykk
settings-styles-sheet-para-spacing = Avstand til avsnitt
settings-styles-sheet-page = Sidestørrelse
settings-styles-sheet-margins = Marginer (T/R/B/L)
settings-styles-sheet-title-page = Bestill tittelsiden
settings-styles-sheet-heading-language = Hovedspråk
settings-styles-sheet-auto = Følger teksten
settings-styles-sheet-digits = Siffer
settings-styles-sheet-direction = Tekstretning
settings-styles-sheet-formats = Formaterer
settings-styles-sheet-all-formats = Alle

settings-styles-editor-group = Stilredigering
settings-styles-editor-missing = Denne stilen er ikke lenger tilgjengelig.

settings-styles-page-letter = Brev
settings-styles-digits-western = Vestlig (0-9)
settings-styles-digits-eastern-arabic = Øst-Arabisk (-)
settings-styles-direction-ltr = Venstre til høyre
settings-styles-direction-rtl = Høyre til venstre

format-dock-title = Format
format-panel-empty = Plasser markøren i en scene, notat eller synopsis for å se formateringsalternativer.
format-group-marks = Tekst
format-group-block = Avsnitt
format-group-lists = Lister
format-group-tables = Tabell
format-group-breaks = Scenepauser
format-superscript = Hevet
format-subscript = Nedsettelse
format-link = Link...
link-dialog-insert-title = Sett inn lenke
link-dialog-edit-title = Rediger lenke
link-dialog-text-label = Tekst
link-dialog-text-placeholder = Ord som leseren ser
link-dialog-url-label = Links til
link-dialog-url-placeholder = example.com
link-dialog-insert = Sett inn
link-dialog-apply = Bruk
link-dialog-cancel = Avbryt
link-dialog-remove = Fjern lenke
link-scheme-refused = Kun web- og e-postkoblinger kan åpnes. { $url } ble igjen alene.
menu-format-link = Link... (&N)
format-clear = Tøm formatering
format-heading = Overskriftsnivå
format-heading-normal = Normal tekst
format-heading-1 = Overskrift 1
format-heading-2 = Overskrift 2
format-heading-3 = Overskrift 3
format-heading-4 = Overskrift 4
format-heading-5 = Overskrift 5
format-heading-6 = Overskrift 6
format-align-left = Juster venstre
format-align-center = Senter
format-direction-rtl = Høyre til venstre avsnitt
format-blockquote = Blokksitat
format-list-bullet = Punktliste
format-list-numbered = Nummerert liste
format-indent = Innrykk
format-outdent = Reduser innrykk
format-table-insert = Sett inn tabell
format-table-row-above = Sett inn rad over
format-table-row-below = Sett inn rad under
format-table-col-before = Sett inn kolonne før
format-table-col-after = Sett inn kolonne etter
format-table-row-delete = Slett rad
format-table-col-delete = Slett kolonne
format-table-remove = Fjern tabell

menu-format-marks-bold = Fet (&D)
menu-format-marks-italic = Italisk (&I)
menu-format-marks-underline = Understreking (&U)
menu-format-marks-strike = Gjennomstreking (&S)
menu-format-marks-superscript = Hevet (&P)
menu-format-marks-subscript = Nedsettelse (&B)
menu-format-marks-clear = Fjern formatering (&C)
menu-format-heading = Overskrift (&H)
menu-format-heading-normal = Normal tekst (&N)
menu-format-heading-1 = Overskrift 1 (&1)
menu-format-heading-2 = Overskrift 2 (&2)
menu-format-heading-3 = Overskrift 3 (&3)
menu-format-heading-4 = Overskrift 4 (&4)
menu-format-heading-5 = Overskrift 5 (&5)
menu-format-heading-6 = Overskrift 6 (&6)
menu-format-alignment = Justering (&A)
menu-format-align-left = Juster venstre (&L)
menu-format-align-center = Senter (&C)
menu-format-direction = Retning (&E)
menu-format-direction-auto = Automatisk (&A)
menu-format-direction-ltr = Venstre til høyre (&L)
menu-format-direction-rtl = Høyre til venstre (&R)
menu-format-blockquote = Blokksitat (&Q)
menu-format-lists = Lister (&L)
menu-format-list-bullet = Punktliste (&B)
menu-format-list-numbered = Nummerert liste (&N)
menu-format-indent = Innrykk (&I)
menu-format-outdent = Reduser innrykk (&O)
menu-format-table = Tabell (&T)
menu-format-table-insert = Sett inn tabell (&I)
menu-format-table-2x2 = 2 x 2 (&2)
menu-format-table-3x3 = 3 x 3 (&3)
menu-format-table-4x4 = 4 x 4 (&4)
menu-format-table-row-above = Sett inn rad ovenfor (&V)
menu-format-table-row-below = Sett inn rad nedenfor (&W)
menu-format-table-col-before = Sett inn kolonne før (&L)
menu-format-table-col-after = Sett inn kolonne etter (&T)
menu-format-table-row-delete = Slett rad (&D)
menu-format-table-col-delete = Slett kolonne (&C)
menu-format-table-remove = Fjern tabell (&R)

menu-format-typo-marks = Formatering av merke (&F)
menu-format-typo-nbsp = No-break plass (&N)
menu-format-typo-nbhyphen = No-break Hyphen (&H)
menu-format-typo-soft-hyphen = Mykt hyphen (&S)
menu-format-typo-nnbsp = Narrow No-break Space (&R)
menu-format-typo-zwsp = Nullbredde-rom (&Z)
menu-format-typo-word-joiner = Word Joiner (&W)


menu-about = Om Skribisto... (&A)
about-title = Om Skribisto
about-version = Versjon { $version }
about-update-available = Versjon { $version } er tilgjengelig
about-update-see-notes = Se hva som har endret seg
about-update-flathub = Flathub holder denne kopien oppdatert.
about-update-distro = Din distribusjon holder denne kopien oppdatert.
about-tagline = En roman-skrive app for langformet fiksjon, skrevet i Rust med Teksilo toolkit.
about-license = Utgitt under GNU General Public License, versjon 3.
about-copyright = © 2026 Cyril Jacquet
about-close = Lukk

native-menu-about = Om { $app }
native-menu-hide = Skjul { $app }
native-menu-quit = Avslutt { $app }
native-menu-settings = Innstillinger…
native-menu-window = Vindu
native-menu-minimize = Minimer
native-menu-zoom = Zoom

window-title = { $title } - { $app }
window-title-numbered = { $title } - { $app } (vindue { $n })
window-title-empty = { $app }

first-run-window-title = Velkommen
first-run-title = Sett opp { $app }
first-run-body = { $app } holder innstillingene separat fra Skribisto, så det starter tom. Preferanser, nylige prosjekter, ordbøker og vindu layout kan kopieres på tvers av nå.
first-run-from = Kopi fra
first-run-to = Kopier til
first-run-copy-note = Ingenting er flyttet eller fjernet. Skribisto holder hver av sine egne innstillinger, og fortsetter å fungere nøyaktig som før.
first-run-import = Importer innstillinger
first-run-start-fresh = Begynn fersk
first-run-import-failed = Noen innstillinger kunne ikke importeres: { $error }

settings-page-punctuation = Teiknsettingsstandarder
settings-group-punctuation = Smart tegnsetting
settings-page-work-punctuation = Teiknsetting
settings-punctuation-override = Gi dette prosjektet sine egne regler for tegnsetting
settings-punctuation-override-hint = Av: prosjektet følger applikasjonens preferanse. Disse reglene reiser inne i .skrib, så en medforfatter som åpner filen skriver med samme typografi.
settings-punctuation-dashes = Gjør -- til en bindestrek, ---- til en em dash
settings-punctuation-ellipsis = Gjør ... til en ellipse
settings-punctuation-quotes = Curl sitatmerker og apostrofer
settings-quote-style = Kvotemerker
settings-quote-style-locale = Språkstandard
settings-quote-style-curly = Dobbel
settings-quote-style-curly-single = «Enkelt»
settings-quote-style-guillemets = «Guillemets»
settings-quote-style-low-high = Lavhøye
settings-punctuation-spacing = Rom før ; !?
settings-punctuation-spacing-hint = Fransk typografi setter et smalt no-break plass før ; ?? og en full før :. Det gjelder bare tekst skrevet på fransk. Plasset inne guillemets « » kommer med guillemet sitater selv.
settings-punctuation-sample = Språket ditt gir
settings-punctuation-app-hint = Hva hvert prosjekt gjør med mindre det tar egne regler i prosjektet .
settings-punctuation-dialogue = Åpne et avsnitt som skrives som --- med en dialog dash
settings-punctuation-dialogue-hint = For språk som markerer tale med en dash i stedet for sitatmerker: fransk, spansk, russisk og andre. Det bare brann i begynnelsen av et avsnitt.

statusbar-go-to = Gå til...
go-to-placeholder = Søk i bindemiddelet
go-to-no-matches = Ingen ting passer til det søket.
menu-go-to = Gå til... (&G)

settings-page-distraction-free-themes = Distraksjonsfrie temaer
settings-themes-builtin = Innebygde temaer
settings-themes-builtin-badge = Innebygget
settings-themes-user = Temaene mine
settings-themes-editor-group = Rediger tema
settings-themes-editor-empty = Velg et tema under - Mine temaer - å redigere det.
settings-themes-use = Bruk
settings-themes-duplicate = Dupliser
settings-themes-edit = Rediger
settings-themes-delete = Slett
settings-themes-export = Eksporter…
settings-themes-import = Importer…
settings-themes-copy-suffix = Kopier
settings-themes-json-filter = Tema (JSON)
settings-themes-imported = Tema importert
settings-themes-import-failed = Kunne ikke importere det temaet
settings-themes-exported = Tema eksportert
settings-themes-export-failed = Kunne ikke eksportere dette temaet
settings-themes-low-contrast = lav kontrast
settings-themes-low-contrast-band = markerer skjuler teksten
settings-themes-field-name = Navn
settings-themes-field-paper = Nettsiden
settings-themes-field-ink = Tekst
settings-themes-field-general = Bakgrunn
settings-themes-field-widget-text = Kontrollstripetekst
settings-themes-field-caret-band = Høydepunkter rundt markøren

statusbar-focus-settings = Distraksjonsfrie innstillinger
statusbar-focus-manage-themes = Administrer temaer...

comments-title = Kommentarer
comments-document-title = Dette dokumentet
comments-empty-project = Ingen kommentarer i dette prosjektet ennå.
comments-empty-document = Ingen kommentarer til dette dokumentet.
comments-filter-all = Alle
comments-filter-open = Åpne
comments-filter-resolved = Løst
comments-filter-orphaned = Uten teksttilknytning
comments-status-open = Åpne
comments-status-resolved = Løst
comments-status-orphaned = Mistet teksten
comments-orphan-snippet = (Den kommenterte teksten er borte)
comments-status-unplaced = Ingen tekstposisjon
comments-unplaced-snippet = (ikke forankret i noen tekst)
comments-reply-count = { $count ->
    [one] { $count } svar
   *[other] { $count } svar
}
comments-menu-resolve = Løsning
comments-menu-reopen = Åpne igjen
comments-menu-delete = Slett
comments-sort-document = I dokumentrekkefølge
comments-sort-newest = Nyeste først
comments-sort-reader = Gruppert av leseren
comments-unsigned-author = Usignert
comments-agreement = { $count ->
    [one] { $count } leser
   *[other] { $count } lesere
}
comments-menu-add = Legg til kommentar
comments-menu-add-paragraph = Kommentar til dette avsnittet
overview-col-comments = Kommentarer
overview-col-total-comments = Totalt sett kommentarer
comments-card-placeholder = Skriv en kommentar...
comments-card-unknown-author = Ukjend forfatter
comments-unsigned-toast = Dine kommentarer er ubesvart: Ingen navn er satt for deg på denne datamaskinen.
comments-unsigned-action = Oppgi ditt navn
comments-card-reply = Svar
comments-card-reply-placeholder = Svar...
comments-card-actions = Kommentarhandlinger
comments-reply-actions = Svarhandlinger
comments-menu-delete-reply = Slett svar
comments-menu-delete-all = Slett alle kommentarer her
comments-deleted-toast = Kommentar slettet
comments-reply-deleted-toast = Svar slettet
comments-deleted-all-toast = { $count ->
    [one] { $count } kommentar slettet
   *[other] { $count } kommentarer slettet
}
comments-undo = Angre

analysis-segment = Analyse
analysis-scope-book = Analysere denne boken
analysis-run = Kjør analyse
analysis-stale = Endret siden dette løp
analysis-not-run = Ikke analysert ennå.
analysis-running = Leser manuskriptet...
analysis-failed = Analysen kan ikke fullføres.
analysis-no-scenes = Ingen scener i denne boken ennå.

analysis-shape = Form

analysis-arrivals = Ankomster
analysis-arrivals-explainer = Hvordan teksten nådde dette prosjektet mens det har vært åpent. Det står hvilken rute tegnene kom ned, og ingenting i det hele tatt om hvem som skrev dem: en forfatter som utkaster andre steder og pastaer har limt seg, og en som dikterer har diktert. Det er ikke noe nummer å sikte på her.
analysis-arrivals-scope = Hele prosjektet, ikke bare denne boken.
analysis-arrivals-session = Siden du åpnet den. Å stenge prosjektet starter tellingen igjen.
analysis-arrivals-nothing = Ingen tekst er kommet i denne sesjonen ennå.
analysis-arrivals-typed = Typet
analysis-arrivals-pasted = Limet inn
analysis-arrivals-dictated = Diktat
analysis-arrivals-imported = Importert
analysis-arrivals-programmatic = Sett inn for deg
analysis-arrivals-count = { $count ->
    [one] { $count } tegn
   *[other] { $count } tegn
}
analysis-arrivals-none = Ingen

analysis-words-per-scene = Ord per scene
analysis-median-words = Bokens medianscene løper { $count ->
    [one] { $count } ord.
   *[other] { $count } ord.
}
analysis-median-line = Median: { $count ->
    [one] { $count } ord
   *[other] { $count } ord
}
analysis-dialogue = Dialog
analysis-dialogue-unsupported = Dialogen er ikke målt for dette språket ennå.

analysis-footnote-words = Ord som ikke er angitt
analysis-footnote-words-count = { $count ->
    [one] { $count } ord står i denne bokens fotnoter, som holdes bortsett fra manuskriptet totalt.
   *[other] { $count } ord er i denne bokens fotnoter, holdt fra manuskriptet totalt.
}
analysis-footnote-words-pending = Teller fotnotene...

analysis-ignore-empty = Oversett tekster med ingenting skrevet ennå
analysis-empty-hidden = { $count } tom { $count ->
        [one] Tekst
       *[other] tekster
    } skjult.
analysis-all-texts-empty = Hver tekst i denne boken er fortsatt tom.

binder-filter-count = { $shown } av { $total } vist
binder-filter-clear = Tøm
binder-filter-none = Ingen treff -{ $query }".

settings-paratext-intro = Forsiden og baksiden kan starte med et nytt prosjekt. Hver struktur tilhører en utgivelsestradisjon, og dens sidetitler er skrevet på den tradisjonens eget språk. Gi dem nytt navn fritt når et prosjekt er opprettet.
settings-paratext-structures = Strukturer
settings-paratext-broken = Kunne ikke leses
settings-paratext-edit = Rediger
settings-paratext-duplicate = Dupliser
settings-paratext-delete = Slett
settings-paratext-new = Ny struktur
settings-paratext-save = Lagre
settings-paratext-editor-hint = Et navn, sidene å opprette før manuskriptet, og sidene å opprette etter det. Der de ender opp er din: de er vanlige elementer når prosjektet eksisterer.


image-insert = Sett inn bildet... (&M)
image-no-project = Åpne et prosjekt før du setter inn et bilde.
image-choose-title = Velg et bilde
image-filter-label = Bilder
image-large-title = Dette er et stort bilde
image-large-text =
    { $name } er { $megapixels } megapiksler ({ $width }×{ $height }).
    Holde det som det er lagrer din opprinnelige fil i prosjektet: det reiser
    med hver sikkerhetskopi og hver eksport. Optimerer lagrer en mindre kopi
    I stedet, opp til 2560 piksler på sin lengste side.
image-large-keep = Behold originalt
image-large-downscale = Optimaliser
image-large-remember = Gjør dette fra nå av, ikke spør igjen
image-not-recorded = Bildet ble lagret, men kunne ikke registreres i prosjektet.
image-describe-title = Beskriv bildet
image-describe-explain = Hva bildet viser, for en leser som ikke kan se det. Det er ikke en del av manuskriptet: det regnes aldri, søkes eller eksporteres som prosa.
image-describe-placeholder = et fyr mot en grå himmel
image-resize-title = Endre størrelse på bildet
image-resize-explain = En prosentandel av størrelsen den er vist nå. 100 forlater den som den er.
image-resize-invalid = Skriv inn et tall mellom 1 og 1000.
image-menu-describe = Beskriv bildet... (&D)
image-menu-resize = Endre størrelse på bildet... (&R)
image-menu-reset-size = Opprinnelig størrelse (&Z)

cover-choose = Oppdaterer listen... (&V)
cover-clear = Fjern dekselet (&R)
cover-choose-title = Velg et cover
cover-set = Dekselet er satt.
cover-cleared = Dekselet er fjernet. Bildet er fortsatt i prosjektet.
card-label = Kort beskrivelse
card-exportable = Eksportert
card-numbered = Nummerert
card-created = Laget
card-modified = Endret
card-yes = Ja
card-no = Nei
card-type = Type
card-position = Stilling
card-children = Barn
card-goal = Mål
card-words = Ord
card-synopsis = Sammendrag
card-point-of-view = Synsvinkel
card-aliases = Også kjent som

export-orphan-footnotes-title = KORT uten referanse
export-comments-dropped =
    { $count ->
        [one] En kommentar kunne ikke plasseres i den eksporterede teksten og ble utelatt.
       *[other] { $count } kommentarer kunne ikke plasseres i den eksporterte teksten og ble utelatt.
    }
export-orphan-footnotes =
    { $count ->
        [one] En fotnote er ikke lenger referert til noe sted i manuskriptet. Teksten vises ikke i den eksporterte boken.
       *[other] { $count } fotnoter er ikke lenger referert til noe sted i manuskriptet. Teksten vil ikke vises i den eksporterte boken.
    }


menu-footnotes = STORLEK (&N)
footnotes-title = STORLEK
footnotes-insert = Sett inn fotnote (&F)
footnotes-empty = Ingen fotnoter ennå. Legg markøren i en scene, deretter bruk + ovenfor, eller Ctrl+Alt+F.
footnotes-filter-all = Alle
footnotes-filter-document = Dette dokumentet
footnotes-filter-orphaned = Uten teksttilknytning
footnotes-orphaned = Ikke noe poeng på denne noten lenger
footnotes-untitled-home = Uten navn
footnotes-body-placeholder = Noten selv
footnotes-insert-tooltip = Sett inn en fotnote ved markøren (Ctrl+Alt+F)
footnotes-actions = Utelukkende tiltak
footnotes-delete = Slett notat og dens referanse (&D)
footnotes-no-project = Åpne et prosjekt før du setter inn en fotnote.
footnotes-no-caret = Legg markøren i en scenes tekst for å sette inn en fotnote der.
footnotes-not-created = Fotnoten kan ikke legges til prosjektet.
footnotes-deleted-toast = Note slettet
footnotes-undo-delete = Angre

import-document-title = Importere dokumenter
import-document-close = Lukk
import-document-step-files = Filer
import-document-step-review = Anmeldelse
import-document-step-destination = Destinasjon
import-document-step-reconcile = Slå sammen
import-document-reconcile-hint = Noen av disse elementene er de du allerede har. Si hva som skal skje med hver.
import-document-reconcile-all-new = Ingenting i denne filen samsvarer med prosjektet ditt: hvert element vil bli lagt til som nytt.
import-document-duplicate-returns = Disse filene er mer enn én kopi av det samme manuskriptet kommer tilbake. Importer dem én om gangen: å matche bare linjer opp én returnerende kopi mot prosjektet ditt, så import av flere sammen ville legge til en annen kopi av boken i stedet for å slå den sammen.
import-document-hunk-added = Ta dette nye avsnittet
import-document-hunk-removed = Drop dette avsnittet
import-document-hunk-changed = Ta denne omskrivingen
import-document-col-stray-prose = Dens tekst
import-document-stray-as-paratext = Behold som paratekst
import-document-stray-discard = Send teksten
import-document-col-current = I prosjektet ditt
import-document-col-incoming = I denne filen
import-document-col-status = Status
import-document-col-action = Hva å gjøre
import-document-status-identical = Lik
import-document-status-editor-edited = De redigerte det
import-document-status-you-edited = Du redigerte den
import-document-status-conflict = Begge har redigert det
import-document-status-different = Differenter
import-document-status-new = Ny
import-document-status-missing = Ikke i denne filen
import-document-status-moved = Flyttet
import-document-action-comments-only = Kommentarer kun
import-document-action-take-import = Ta denne versjonen
import-document-action-keep-current = Hold min
import-document-action-create-new = Legg til som ny
import-document-action-ignore = Hopp
import-document-compare = Sammenlign
import-document-compare-legend = Din versjon sammenlignet med denne filen. Skrivbar.
import-document-compare-close = Lukk
import-document-drop-title = Drop dokumenter her
import-document-drop-hint = Markdown (.md), Ord (.docx), OpenDocument (.odt) og vanlig tekst (.txt)
import-document-browse = Bla gjennom...
import-document-move-up = Flytt opp
import-document-move-down = Flytt ned
import-document-remove-file = Fjern
import-document-file-count = { $count ->
    [one] 1 fil
   *[other] { $count } filer
}
import-document-no-files = Ingen filer valgt ennå.
import-document-col-included = Importer
import-document-col-title = Tittel
import-document-col-type = Type
import-document-col-words = Ord
import-document-col-breaks = Skiller
import-document-col-comments = Kommentarer
import-document-col-footnotes = STORLEK
import-document-col-epigraph = Motto
import-document-col-source = Kilde
import-document-level-rules = Utskriftsnivå
import-document-level-n = Leder { $level }
import-document-add-top-level = Legg til toppnivå
import-document-add-top-level-tooltip = Sett inn en bok over alle analyserte elementer, for kapittelfiler som ikke har bokoverskrift
import-document-destination = Destinasjon
import-document-destination-hint = Velg et bindeelement eller element: nye elementer lander i en mappe, eller etter en scene.
import-document-destination-empty = Ingen bindemidler ennå. Opprett en først.
import-document-plan-empty = Ingenting å importere enda.
import-document-summary = { $rows ->
    [one] 1 element
   *[other] { $rows } varer
} · { $breaks ->
    [one] 1 scenepause
   *[other] { $breaks } scene pauser
}
import-document-back = Tilbake
import-document-cancel = Avbryt
import-document-analyse = Neste
import-document-analysing = Lese dokumenter...
import-document-step-analysing = Lesing
import-document-cancel-analysis = Slutt å lese
import-document-analyse-failed = Dokumentene kunne ikke leses.
import-document-details = Detaljer
import-document-import = Importer
import-document-done = { $count ->
    [one] 1 element importert
   *[other] { $count } varer importert
}
import-document-undo = Angre
import-diagnostic-file-unreadable = "{ $path }Kunne ikke leses: { $detail }De andre filene importerer fortsatt.
import-diagnostic-nested-too-deep = "{ $path }- ble ikke lest: innholdet er hekket mer enn { $limit }  nivå dypt. Ingen ekte dokument kommer nær det, og å lese det kan krasje Skribisto, så det er utelatt. De andre filene fortsatt importere.
import-diagnostic-archive-too-large = "{ $path }- ble ikke lest: det er en komprimert fil bygget for å pakke ut langt større enn det er, noe som kan eksosminne og krasje Skribisto, så det er utelatt. De andre filene fortsatt importere.
import-diagnostic-lossy-decode = { $count ->
    [one] En karakter i -{ $path }- ikke dekode. Lagre filen på nytt som UTF-8 for å beholde det.
   *[other] { $count }  tegn i -{ $path }- ikke dekode. Lagre filen på nytt som UTF-8  å beholde dem.
}
import-diagnostic-decoded-from-bom = "{ $path } ble dekodet som { $detail }, ikke UTF-8.
import-diagnostic-empty-file = "{ $path }- Det er tomt.
import-diagnostic-no-headings = "{ $path }- har ingen overskrifter, så det kommer som ett element.
import-diagnostic-unsupported-format = Ingenting leser -.{ $detail }Filer, så-{ $path }- Jeg ble hoppet over.
import-diagnostic-front-matter-not-flat = Forsiden i{ $path }": "{ $detail }- er ikke en enkel verdi og ble hoppet over.
import-diagnostic-footnotes-degraded = { $count ->
    [one] En fotnote i -{ $path }- ankommer som vanlig tekst: fotnoter leses ikke fra Markdown.
   *[other] { $count } Fotnoter i{ $path }ankommer som vanlig tekst: fotnoter leses ikke fra Markdown.
}
import-diagnostic-footnote-not-carried = { $count ->
    [one] En fotnote i -{ $path }- De andre var det ikke.
   *[other] { $count } Fotnoter i{ $path }- De andre var det ikke.
}
import-diagnostic-raw-html-dropped = { $count ->
    [one] En blokk av rå HTML i-{ $path }- Han ble droppet.
   *[other] { $count }  blokker av rå HTML i-{ $path }- Han ble droppet.
}
import-diagnostic-nested-break-dropped = { $count ->
    [one] Et scenebrudd inne i et sitat eller en liste i{ $path } Bare en pause på egen linje holdes.
   *[other] { $count } scene bryter inn i sitater eller lister i-{ $path }- ble droppet. Bare en pause på egen linje holdes.
}
import-diagnostic-image-not-ingested = "{ $path }Referer til bildet{ $detail } referansen kommer som tekst; bildet i seg selv er ikke kopiert i.
import-diagnostic-duplicate-title = "{ $title }- vises { $count }  ganger. Hvis du har importert disse filene før, vil dette duplisere dem.
import-diagnostic-heading-level-jump = "{ $title }- hopp fra overskriftsnivå { $from } til { $to }Den er plassert på ett nivå under sin forelder.
import-diagnostic-illegal-combination = "{ $title } bære prosa, men en { $kind }  kan ikke holde noen. Import holdes til du endrer sin type eller la den være ute. Ingenting importeres i det hele tatt ellers.
import-diagnostic-tracked-changes-flattened = { $path } var midtrevision: { $count ->
    [one] { $count } ble akseptert
   *[other] { $count } sporbare endringer ble akseptert
} , og slettinger falt. Det er den endelige teksten. For å holde noen av din egen formulering i stedet, bruk Sammenlign i det siste trinnet.
import-diagnostic-tracked-changes-flattened-by = { $path } var midtrevision: { $count ->
    [one] { $count } spores endring av { $names } ble akseptert
   *[other] { $count } sporte endringer av { $names } ble akseptert
} , og slettinger falt. Det er den endelige teksten. For å holde noen av din egen formulering i stedet, bruk Sammenlign i det siste trinnet.
import-diagnostic-text-box-dropped = { $count ->
    [one] { $path } inneholder { $count } tekstboks. Dens tekst sitter utenfor dokumentets strømmen, så der det hører hjemme i et manuskript kan ikke besvares. Den er ikke importert.
   *[other] { $path } inneholder { $count } tekstbokser. Deres tekst sitter utenfor dokumentets flyt, så der det hører hjemme i et manuskript kan ikke besvares. De er ikke importert.
}
import-diagnostic-embedded-object-dropped = { $count ->
    [one] { $path } har { $count } innbygd objekt: et diagram, en likning eller lignende. Det er ingenting i et manuskript som kan holde det.
   *[other] { $path } har { $count } innebygde objekter: et diagram, en likning eller lignende. Det er ingenting i et manuskript som kan holde dem.
}
import-diagnostic-field-flattened = { $count ->
    [one] { $path } inneholder { $count } felt: et sidenummer, en kryssreferanse, en dato. Den holder teksten den sist viste og vil ikke oppdatere igjen.
   *[other] { $path } har { $count } felt: et sidenummer, en kryssreferanse, en dato. Hver holder teksten den sist viste og vil ikke oppdatere igjen.
}
import-diagnostic-unknown-style-level = { $path }  Bruker stilen -{ $detail }, som ser ut som en overskrift men navn ikke nivå. Disse avsnittene importeres som prosa i stedet for gjettet på en dybde.
import-diagnostic-comment-unanchored = Kommentaren -{ $detail }- i { $path }  kunne ikke være knyttet til ordene det handlet om. Det holdes på det elementet, hvor du kan flytte det.
import-diagnostic-comment-not-carried = Kommentaren -{ $detail }- i { $path } er ikke importert, fordi ingenting i filen ankommer som tekst det kan holdes på. Kopier det fra filen hvis du trenger det.
import-diagnostic-comment-replies-flattened = { $count ->
    [one] { $count } svarer i { $path } ga en kommentar som ikke er i filen, så det kommer som en egen kommentar.
   *[other] { $count } svar i { $path } ga en kommentar som ikke er i filen, så de kommer som egne kommentarer.
}
import-diagnostic-prose-not-verbatim = { $count ->
    [one] { $count } avsnitt i { $path } kunne ikke bringes over med sin formatering nøyaktig som skrevet, så det kommer som vanlig tekst. Sammenlign det med den opprinnelige.
   *[other] { $count } avsnitt i { $path } kunne ikke bringes over med formateringen nøyaktig som skrevet, så de kommer som vanlig tekst. Sammenlign dem med den opprinnelige.
}
import-diagnostic-styled-spaces-not-carried = { $count ->
    [one] I { $path }, { $count }-strekningen av tomrom ble understreket eller slått gjennom, en linje som var igjen å fylle ut for eksempel. Skribisto holder ingen linje på rom alene, så den ankommer som vanlig rom, eller ikke i det hele tatt hvor den starter eller slutter et avsnitt.
   *[other] I { $path }, { $count }-strekninger med tomrom ble understreket eller slått gjennom, linjer som var igjen å fylle ut for eksempel. Skribisto holder ingen linje på rom alene, slik at de ankommer som rene rom, eller ikke i det hele tatt hvor de starter eller avslutter et avsnitt.
}
import-diagnostic-list-nesting-flattened = { $count ->
    [one] { $count } listeelement i { $path } ble hekket mer enn { $limit } nivåer dypt. Den kommer til nivå { $limit }, den dypeste Skribisto holder, med sine ord og formatering.
   *[other] { $count } liste elementer i { $path } ble hekket mer enn { $limit } nivåer dypt. De ankommer til nivå { $limit }, den dypeste Skribisto holder, med sine ord og formatering.
}
import-diagnostic-table-read-as-paragraphs = { $count ->
    [one] { $count }-tabellen i { $path } har en rad langt bredere enn resten, for skrå til å legge ut som et rutenett. Dens celler ankommer som avsnitt, den ene etter den andre, med sine ord og formatering.
   *[other] { $count } tabeller i { $path } har en rad som er langt bredere enn resten, som er for skråstilt til å legge ut som et rutenett. Deres celler kommer som avsnitt, den ene etter den andre, med deres ord og formatering.
}
import-diagnostic-spaces-shortened = { $count ->
    [one] { $count } kjøring av rom i { $path } var lengre enn { $limit } rom, og ankommer forkortet til { $limit }. Ordene rundt den er alle der.
   *[other] { $count } løper av rom i { $path } var lengre enn { $limit } rom, og ankommer forkortet til { $limit }. Ordene rundt dem er alle der.
}
import-diagnostic-epigraph-not-carried = "{ $title } Ledes av en epigraf, men en { $kind }  kan ikke holde en. sitatet holdes øverst i teksten i stedet.
import-diagnostic-epigraph-placement-ambiguous = En epigraf sitter mellom...{ $title }- og -{ $below }- og kan hodet også. Det ble gitt til-{ $title } Det er der en epigraf vanligvis går.
import-document-diagnostics = { $errors ->
    [0] { $warnings ->
            [one] 1 ting å vite
           *[other] { $warnings } Ting å vite
        }
   *[other] { $errors ->
            [one] 1 fil kunne ikke leses
           *[other] { $errors }-filer kunne ikke leses
        }
}
import-document-diagnostics-none = Ingenting å rapportere.

versions-title = Versjoner
versions-scope-synopsis = Sammendrag
versions-scope-prose = Tekst
versions-loading = Se gjennom dine backups...
versions-empty = Ingen tidligere versjon av dette ennå
versions-error = Kunne ikke lese sikkerhetskopiene. Ingenting har gått tapt, men denne listen kan være ufullstendig
versions-did-not-exist = Det eksisterte ikke ennå på { $date }
versions-deleted-after = Slettet litt tid etter { $date }
versions-unreadable = { $count ->
    [one] { $count } sikkerhetskopi kunne ikke leses
   *[other] { $count } sikkerhetskopier kunne ikke leses
}
versions-thinned = { $count ->
    [one] Eldre versjoner tynner ut etter alder. Prosjektets egen historie har allerede falt { $count } tidligere tilstand i denne teksten.
   *[other] Eldre versjoner som er tynne ut som de alder. Prosjektets egen historie har allerede falt { $count } tidligere tilstander i denne teksten.
}
versions-source-backup = Fra en backup
versions-source-project = Fra prosjektets egen historie
versions-list-caption = En oppføring per endring, ikke per sikkerhetskopi
versions-pick-a-version = Velg en versjon for å se hva som endret seg
versions-earliest = Den tidligste versjonen på rekorden. Det er ingenting eldre å sammenligne den med.
versions-no-change = Ingenting endret seg i denne delen av elementet
versions-formatting-only = Bare formateringen endret her. Ordene er de samme
versions-show-unchanged = Vis uendret
versions-hide-unchanged = Skjul uendret
versions-next-change = Neste endring
versions-near = i nærheten av{ $text }"
versions-words-added = { $count ->
    [one] { $count } ord lagt til
   *[other] { $count } ord lagt til
}
versions-words-removed = { $count ->
    [one] { $count } ord fjernet
   *[other] { $count } ord fjernet
}
versions-blocks-moved = { $count ->
    [one] { $count } avsnitt flyttet
   *[other] { $count } avsnitt flyttet
}
versions-pin = Pin sikkerhetskopien denne versjonen kom fra. Automatisk opprydding vil aldri slette den
versions-unpin = Fjern sikkerhetskopien denne versjonen kom fra. Automatisk opprydding kan slette den igjen
versions-pinned-only = Vis kun festede versjoner
versions-pin-note = Bare versjoner fra en sikkerhetskopi kan festes. En pinne beholder en fil, og prosjektets egen historie er ikke én.
versions-pinned-empty = Ingenting her er festet ennå
versions-pinned-empty-log = Ingenting her er festet ennå. Bare versjoner fra en sikkerhetskopi kan festes. En pinne holder en fil, og prosjektets egen historie er ikke én.
versions-range-filter = Vis kun versjoner mellom to datoer
versions-filtered-empty = Ingen versjon matcher filtrene du har satt
versions-clear-filters = Tøm filtrene
versions-last-30-days = Siste 30 dager
versions-restore-button = Gjenopprett denne versjonen
versions-restore-confirm-title = Bytt ut denne teksten med versjonen fra { $date }?
versions-restore-confirm-text = Hva du har nå vil bli erstattet med teksten dette elementet hadde den { $date }.
versions-restore-confirm-with-comments = Hva du har nå vil bli erstattet med teksten dette elementet hadde den { $date }. { $count ->
    [one] { $count } kommentar er forankret i den gjeldende teksten og kan etterlates foreldreløse.
   *[other] { $count } kommentarer er forankret i den aktuelle teksten og kan etterlates foreldreløse.
}
versions-restore-confirm-undo-note = En sikkerhetskopi gjøres først, og Ctrl+Z angrer dette i ett trinn.
versions-restored-toast = Gjenopprettet versjonen fra { $date }
versions-undo = Angre
versions-restore-row-gone = Denne varen er ikke lenger i dette prosjektet
versions-restore-no-home = Dette elementet har endret type siden da, og den gamle teksten har ingen steder å gå inn i det
versions-restore-no-safety-copy = Sikkerhetskopien din gikk ikke, så ingenting ble endret
versions-restore-failed = Gjenopprettingen mislyktes: { $error }
versions-restore-backup-busy = En sikkerhetskopi kjører allerede. Prøv igjen om et øyeblikk
versions-restore-in-backup-file = Du ser på en sikkerhetskopifil; åpne prosjektet selv for å gjenopprette det
versions-restore-no-project = Ingen prosjekter er åpne
versions-recreate-button = Ta det tilbake...
versions-recreate-picker-title = Hvor skal den gå?
versions-recreate-picker-empty = Dette prosjektet har ingen bindemiddel å sette det i
versions-recreate-picker-confirm = Ta den tilbake hit
versions-recreate-picker-cancel = Avbryt
versions-recreate-untitled = dette elementet
versions-recreate-confirm-title = Ta med deg{ $item }- Tilbake?
versions-recreate-confirm-text = Den vil bli lagt til { $destination }, med teksten den hadde på { $date }.
versions-recreate-confirm-undo-note = På meldingen som følger, tar den rett ut.
versions-recreated-toast = "{ $item }er tilbake i prosjektet ditt
versions-recreated-partial-toast = { $count ->
    [one] "{ $item }- er tilbake, men en av tekstene kan ikke leses
   *[other] "{ $item }- er tilbake, men { $count }  Tekstene kunne ikke leses
}
versions-recreate-already-here = Varen er allerede i prosjektet ditt
versions-recreate-no-destination = Velg et sted i bindemidlet for å sette det
versions-recreate-unreadable = Denne sikkerhetskopien kunne ikke leses, så ingenting ble lagt til
versions-recreate-failed = Kunne ikke bringe den tilbake: { $error }
versions-changed-percent = { $percent }% av dette endret
versions-hidden-paragraphs = { $count ->
    [one] ... { $count } uendret avsnitt ...
   *[other] ... { $count } uendrete avsnitt ...
}

timeline-title = Gå tilbake i tiden
timeline-coverage = { $count ->
    [one] { $count } versjon registrert, går tilbake til { $oldest }
   *[other] { $count } versjoner registrert, går tilbake til { $oldest }
}
timeline-loading = Se gjennom prosjektets fortid...
timeline-empty = Ingen versjon av dette prosjektet er registrert ennå
timeline-no-changes = Ingenting har endret seg siden da
timeline-no-text-changes = Ingen tekst har endret seg siden da
timeline-slider-label = Innspilt versjon
timeline-series-name = Prosjektstørrelse
timeline-bars-caption = Hver bar er en registrert versjon, så høy som prosjektet var da. Den uthevede er det du ser på.
timeline-bars-caption-periods = For mange versjoner til å vise en etter en, så hver bar er en { $period }, så høy som prosjektet var ved slutten av det.
timeline-unit-hour = time
timeline-unit-day = dag
timeline-unit-week = Uke
timeline-unit-month = måned
timeline-range-filter = Vis kun versjoner mellom to datoer
timeline-range-empty = Ingen versjon er registrert på disse datoene
timeline-open-period = Åpne denne perioden
timeline-show-all = Vis hele historien
timeline-changed-since = { $count ->
    [one] { $count } element varierer mellom { $date } og prosjektet ditt nå
   *[other] { $count } elementer varierer mellom { $date } og prosjektet ditt nå
}
timeline-kind-added = Skrevet siden
timeline-kind-removed = Ikke lenger i prosjektet
timeline-kind-changed = Redigert siden
timeline-kind-moved = Flyttet siden
timeline-not-yet-written = Dette eksisterte ikke ennå på det tidspunktet.
timeline-no-text-of-its-own = Dette har ingen egen tekst. Det er en overskrift for hva som er inne i den.
timeline-reader-close = Lukk
timeline-reader-stamp = Som det var den { $date }
timeline-reader-loading = Åpne den registrerte versjonen...
timeline-reader-unreadable = Denne opptaket kunne ikke leses. Sikkerhetskopien kan ha blitt flyttet, slettet eller være på en enhet som ikke er tilkoblet.
timeline-reader-compared = Som det var på { $date }, sett mot det den sier nå
timeline-reader-diff-legend = Struck gjennom har gått siden; understreket har blitt lagt til.
timeline-reader-view-label = Hva å vise
timeline-reader-view-diff = Endringer
timeline-reader-view-text = Tekst
timeline-reader-deleted = Dette er ikke lenger i prosjektet. Du kan lese det og kopiere det her ute.
timeline-prose-only-record = Dette punktet kommer fra prosjektets egen historie, som holder tekst og ingenting annet, så det som ble slettet eller flyttet siden det ikke kan vises. Velg en sikkerhetskopi for det.

settings-sec-extensions = Forlengelser

settings-group-games-forward = Alltid fremover
settings-games-forward-toggle = Spill - alltid fremover -
settings-games-forward-hint =
    Mens du spiller, kan ingenting du har skrevet tas tilbake: Backspace, Slett,
    Klippe ut, dra og slippe og angre er alle deaktivert i overflatene du velger nedenfor.
    Du kan fortsatt skrive, lime inn, formatere og bevege seg fritt. Utkastet vokser bare.
settings-games-session-warning =
    Dette er et per-session valg: det er aldri lagret. Lukking av prosjektet, eller slutte
    Skribisto, avslutter alltid spillet, og alt du skrev mens du spiller blir
    Kan angres igjen det øyeblikket det gjør.
settings-group-games-scope = Når det gjelder
settings-games-in-prose = Manuscript prosa
settings-games-in-synopsis = Sammendrag
settings-games-scope-hint =
    Kommentarer, fotnoter og titler er aldri frosset: de er der du legger merke til løsningen
    Du har nettopp forbudt deg selv å lage.
settings-games-inert-warning =
    - Alltid fremover - er på, men gjelder ingenting. Tick minst én overflate over,
    eller endrer ingenting som du skriver.

games-title = Skrive spill
games-forward-name = Alltid fremover
games-forward-blurb =
    Utkast uten å ta noe tilbake. Sletting, skjæring og angre er deaktivert
    mens du spiller. Skriv neste setning i stedet for å fikse den siste.
games-forward-playing = Spilling: Sletting er deaktivert
games-forward-idle = Ikke å spille
games-session-note = Slutter når du stenger prosjektet.
games-scope-prose-and-synopsis = Gjelder dine prosa og synops.
games-scope-prose = Gjelder for prosa.
games-scope-synopsis = Gjelder synopsene dine.
games-scope-nothing = Gjelder ingenting ennå. Velg en overflate i Innstillinger.
games-settings-link = Skriver spillinnstillinger...
statusbar-games-forward = Alltid fremover
statusbar-games-forward-tooltip =
    - Alltid fremover - er på: slette, kutte og angre er deaktivert mens du
    Klikk for å slutte å spille.
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
inspector-goal-none = Ingen mål

overview-col-goal = Mål
overview-excluded-from-export = Ute av eksporten, så det teller mot ingen total

goal-subtree-total-words = { $count ->
    [one] { $items } målet inne legger opp til { $words } ord
   *[other] { $items } mål inne legge til { $words } ord
}
goal-subtree-total-characters = { $count ->
    [one] { $items } mål inne legger til { $words } tegn
   *[other] { $items } mål inne legger til { $words } tegn
}

new-work-goal-unit = Inntelling
new-work-goal-unit-hint = Brukes for hvert ord eller tegnmål i dette prosjektet. Kan endres senere.

settings-group-goal-unit = Mål
settings-goal-unit-switch-title = Bytt telleenhet
settings-goal-unit-switch-text = Målene og milepælene som allerede er satt for dette prosjektet ble angitt i { $from }. Bytt til { $to } konverterer dem ikke, så hvert eksisterende tall vil nå leses som { $to }.
settings-goal-unit-switch-informative = Ingenting går tapt. Bytt tilbake når som helst for å gjenopprette den originale lesningen, og oppdater deretter målene du vil beholde.
settings-goal-unit-switch-confirm = Bytt

goal-distribute-action = Distribuer...
distribute-title = Fordel målet
distribute-empty = Det er ingenting i dette å dele målet ut til.
distribute-weight = Del ut
distribute-weight-length = Etter lengd
distribute-weight-rows = Av biter
distribute-weight-even = Jevnt
distribute-overwrite = Bytt ut mål som allerede er satt
distribute-col-item = Vare
distribute-col-current = Nå
distribute-col-proposed = Etter
distribute-total = Disse tilsettes opp til { $total }, mot et mål på { $goal }.
distribute-over-budget = Målene som allerede er satt inne, kommer til { $over } mer enn denne beholderens egen. Løft målet, senk deres eller erstatte dem.
distribute-apply = Distribuer
distribute-cancel = Avbryt

milestone-target-gone = Mål slettet
milestone-no-target = Ingen mål
milestone-add = Legg til
milestone-add-label = Hva som skal nås

pace-summary-title = Hvor boken står
pace-summary-remaining = { $words } å gå
pace-summary-open = Åpne planen
pace-summary-close = Lukk
pace-summary-dont-show = Ikke vis dette når du åpner
pace-summary-menu = Skriveplan...

menu-help-topics = Hjelpemner (&T)
menu-help-shortcuts = Tastatursnarveier... (&K)
menu-help-website = Skribisto på nettet (&W)
menu-help-report = Rapporter et problem... (&R)
menu-help-check-updates = Sjekk etter oppdateringer (&U)
menu-command-palette = Kommandopalett... (&C)

learn-help-topics = Hjelpemner
learn-shortcuts = Tastatursnarveier
learn-website = Skribisto på nettet

help-window-title = Hjelp
help-filter-topics = Filtrer emner
help-no-matching-topic = Ingen emnekamper.
help-topic-missing = Dette emnet er ikke lenger tilgjengelig.
help-back = Tilbake
help-not-translated = Denne artikkelen finnes ikke på norsk ennå. Den vises på engelsk.

help-section-getting-started = Komme i gang
help-section-writing = Skriving
help-section-reviewing = Gjennomgang
help-section-exchanging = Få ord inn og ut
help-section-keeping = Hold prosjektet trygt
help-section-extensions = Forlengelser

help-topic-getting-started = Ditt første prosjekt
help-topic-writing-model = Hvordan en bok settes sammen
help-topic-drafts-and-old-versions = Å holde et eldre utkast
help-topic-goals-and-pace = Mål og tempo
help-topic-comments = Kommentarer
help-topic-round-trip = Sende boken din til en leser
help-topic-export = Eksporterer
help-topic-import-documents = Importere dokumenter
help-topic-import-projects = Å bringe et helt prosjekt på tvers
help-topic-backups-and-versions = Sikkerhetskopier og versjoner

help-shortcuts-title = Tastatursnarveier
help-shortcuts-filter = Filtrer snarveier
help-shortcuts-no-matches = Ingen snarveiskamper.
help-shortcuts-rebind = Endre snarveier...
help-shortcuts-close = Lukk

command-palette-placeholder = Skriv inn en kommando

shortcut-name-binder-duplicate = Dupliser
shortcut-name-comments-add = Legg til kommentar
shortcut-name-comments-add-paragraph = Kommentar til avsnitt
shortcut-name-spellcheck-toggle = Kontroller staving
shortcut-name-editor-tab-close = Lukk fane
shortcut-name-editor-tab-pin = Pin eller Unpin-fanen
shortcut-name-editor-save = Lagre
shortcut-name-work-export = Eksporter…
shortcut-name-work-new = Nytt prosjekt
shortcut-name-work-open = Åpent prosjekt
shortcut-name-window-new = Nytt vindu
shortcut-name-work-close = Lukk prosjekt
shortcut-name-app-settings = Innstillinger
shortcut-name-app-quit = Avslutt
shortcut-name-editor-insert-footnote = Sett inn nytt element
shortcut-name-format-scene-break = Sett inn Scene Break
shortcut-name-format-major-scene-break = Sett inn større scenebrudd
shortcut-name-format-link = Sett inn lenke
shortcut-name-format-mark-nbsp = Sett inn ledig plass
shortcut-name-format-mark-nbhyphen = Innsett no-break Hyphen
shortcut-name-format-mark-nnbsp = Sett inn smalt no-break plass
shortcut-name-format-mark-zwsp = Sett inn nullbredde-rom
shortcut-name-go-next = Neste
shortcut-name-go-prev = Forrige
shortcut-name-go-to = Gå til
shortcut-name-outline-toggle = Slå av/på omlinje
shortcut-name-preview-toggle = Slå av og på forhåndsvisningsbånd
shortcut-name-view-fullscreen = Slå av/på fullskjerm
shortcut-name-view-focus-mode = Slå av/på distraksjonsfri modus
shortcut-name-editor-size-increase = Øk tekststørrelsen
shortcut-name-editor-size-decrease = Reduser tekststørrelse
shortcut-name-editor-size-reset = Tilbakestill tekststørrelsen
shortcut-name-editor-find = Finn
shortcut-name-editor-replace = Erstatt
shortcut-name-editor-find-next = Neste kamp
shortcut-name-editor-find-prev = Forrige kamp
shortcut-name-search-show = Søk i prosjektet
shortcut-name-search-replace = Bytt ut i prosjektet
shortcut-name-outline-open-to-side = Åpent for siden
shortcut-name-help-topics = Hjelpemner
shortcut-name-help-shortcuts = Tastatursnarveier
shortcut-name-help-website = Skribisto på nettet
shortcut-name-help-report = Rapporter et problem
shortcut-name-help-check-updates = Sjekk etter oppdateringer
update-check-behind = Versjon { $version } er tilgjengelig.
update-check-current = Du kjører den nyeste versjonen.
update-check-failed = Kunne ikke sjekke for en nyere versjon. { $error }
shortcut-name-command-palette = Kommandopalett
help-section-reference = Hva er det?

help-concept-scene-break = Scenepause
help-concept-major-scene-break = Stor scenepause
help-concept-find-in-prose = Finn i prosa
help-concept-story-bible = Historiebibel
help-concept-goal-unit = Ord eller tegn
help-concept-goal-progress = Fremdrift
help-concept-manuscript-words = Hva teller som manuskript
help-concept-exportable = Ute av eksporten
help-concept-distribute = Dele et mål ut
help-concept-subtree-total = Hva målene i innsiden legger til
help-concept-milestone = Milepæl
help-concept-pace-plan = Plan for skrivetempo

help-concept-tag = Etikett
help-concept-label = Kort beskrivelse
help-concept-point-of-view = Synsvinkel
help-concept-epigraph = Motto
help-concept-footnote = Fotnote
help-concept-chapter-mode = Kapittelform
help-concept-comment = Kommentar
help-concept-backup = Sikkerhetskopi
help-concept-version = Utgave
help-concept-trash = Papirkurv
help-concept-spellcheck = stavekontroll
help-concept-search-replace = Søk og erstatte
help-concept-note-template = Merkemal
help-concept-story-bible-entry = Story Bible Entry
help-concept-text-replacement = Tekstutskifting
help-concept-smart-punctuation = Smart tegnsetting
help-concept-export-style = Eksporter stil
help-concept-round-trip-marks = Rundt-trip markører

epigraph-mark-attribution = Kildelinje
epigraph-mark-attribution-tip = Marker linjen markøren er i som sitatets kilde, så det skriver ut som en tilskrivelse
margin-lane-name = Marginmerker
margin-lane-provider-comments = Kommentarer
margin-lane-provider-story-bible = Story Bible Entry
margin-lane-provider-story-bible-hint = På et notat i prosalesing, hvor oppføringen er navngitt, og scenene fortalte gjennom øynene. Også markerer disse navnene i prosa selv.
margin-lane-mark-point-of-view = Fortalt herfra
margin-lane-mark-named-here = Navngitt her
margin-lane-provider-comments-hint = Når en note er knyttet til teksten
margin-lane-provider-search = Søk treff
margin-lane-provider-search-hint = Hver hit for det du siste søkte etter, uansett hvor det er i dokumentet
margin-lane-provider-boundaries = Hvor hvert dokument starter
margin-lane-provider-boundaries-hint = En regel på toppen av hver scene i en strøm, slik at du kan se hvor du er
margin-lane-provider-spelling = Staving
margin-lane-provider-spelling-hint = Hvor stavekontrollen har flagget et ord. Av med mindre du ber om det: det markerer en maskins mening om prosa
margin-lane-spelling = { $word }, mulig feilstaving
margin-lane-search-hit = { $text }, match { $index } av { $total }
margin-lane-search-current = { $text }, match { $index } av { $total }, den du er på
margin-lane-boundary = Begynnelsen på { $title }
margin-lane-boundary-untitled = et unavngitt dokument
settings-page-margin-lane = Marginmerker
settings-desc-margin-lane = Stripen ved siden av rullelinjen, og hva den viser
settings-margin-lane-enabled = Vis marginbanen
settings-margin-lane-enabled-hint = Den samme bryteren som Vis  - Marginmerker.
settings-margin-lane-enabled-more = Det sier aldri noe er galt: det viser deg hvor ting er, og etterlater hva du skal gjøre av det.
settings-group-margin-lane-marks = Hva det markerer
settings-margin-lane-no-providers = Ingenting markerer banen ennå.
settings-group-margin-lane-texture = Dialogtekstur
settings-margin-lane-texture = Vis dialogteksturen
settings-margin-lane-texture-hint = En bar per avsnitt: hvor lang tid det er, og hvor mye av det blir talt.
settings-margin-lane-texture-more = Ikke målt for språk uten kurert konvensjon, der baren viser lengde alene.
settings-group-margin-lane-surfaces = Hvor det vises
settings-margin-lane-surface-editor = Tekstredigering
settings-margin-lane-surface-stream = Tekststrømmer
settings-margin-lane-surface-search-preview = Forhåndsvisning av søk
menu-margin-lane = Marginmerker (&M)

status-preset-drafting = Utkast
status-preset-passes = Revisjon passerer
status-preset-plume = Plume Creator
status-todo = å gjøre
status-draft = Utkast
status-revised = Revidert
status-final = Endelig
status-outline = Disposisjon
status-first-edit = 1. redigering
status-second-edit = 2. redigering
status-done = Ferdig
status-plume-draft-1 = Første utkast
status-plume-draft-2 = 2. utkast
status-plume-draft-3 = 3. utkast
status-plume-edit-1 = 1. redigering
status-plume-edit-2 = 2. redigering
status-plume-edit-3 = 3rd redigering
status-plume-proofread = Korrekturlest
status-plume-finished = Ferdig
new-work-statuses = Arbeidsflyt
new-work-statuses-hint = De stadiene du flytter en scene gjennom. Du kan gi nytt navn, ombestille eller legge til i disse når som helst.
status-none = Ingen status
inspector-status = Status
overview-col-status = Status
overview-status-mixed = deler under er uenige
help-concept-status = Status
status-completion-title = Hvor boken står
status-completion-headline = { $done } av { $total } scener ferdig
status-completion-empty = Ingen scener ennå. Dette leser manuskriptet, så det fyller i som du skriver.
status-completion-open = Hvor boken står...

settings-page-statuses = Statuser
settings-desc-statuses = Endre navn på, omorganisere, legge til og fjerne stadier du flytter en scene gjennom
settings-statuses-add = Legg til status
settings-statuses-add-placeholder = Nytt statusnavn
settings-statuses-desc =
    Stadiene en scene beveger seg gjennom, i rekkefølge, fra minst ferdig på toppen til mest
    fullført nederst. Den rekkefølgen er det - mindre ferdig enn - betyr overalt andre steder.
    i appen, så legg dem som prosessen din faktisk kjører.
settings-statuses-apply-preset = Bruk en forhåndsinnstilling...
settings-statuses-preset-applied = { $added ->
    [one] 1 status la til
   *[other] { $added } statuser lagt til
}
settings-statuses-preset-refused = Dette prosjektet har allerede en stige. Forhåndsinnstillingene fyller bare ut en tom. Slett kjørsene du ikke vil ha først.
settings-statuses-duplicate = "{ $name }- er allerede på denne stigen
settings-statuses-added = Lagt til{ $name }"
settings-statuses-move-up = Flytt opp (mindre ferdig)
settings-statuses-move-down = Flytt ned (mer ferdig)
settings-statuses-details-placeholder = Hva dette stadiet betyr (valgfritt)
settings-statuses-delete = Slette{ $name }"
settings-statuses-delete-in-use = { $count ->
    [one] Slette{ $name }1 element er på det og vil miste sin status
   *[other] Slette{ $name }": { $count } Varene er på det og vil miste sin status
}
settings-statuses-deleted = Slettet{ $name }"
settings-statuses-deleted-in-use = { $count ->
    [one] Slettet{ $name }1 element har ikke lenger en status.
   *[other] Slettet{ $name }". { $count } Varene har ikke lenger status.
}
settings-statuses-empty-title = Ingen arbeidsflyt ennå
settings-statuses-empty-body = En status sier hvor langt langs en scene er. Legg til en over, eller start fra en forhåndsinnstilling.
status-category-planned = Planlagt
status-category-drafting = Utkast
status-category-needs-work = Trenger arbeid
status-category-revised = Revidert
status-category-final = Endelig
