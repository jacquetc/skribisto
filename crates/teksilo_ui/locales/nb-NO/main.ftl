# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

menu-work = &Prosjekt
menu-new-work = &Nytt prosjekt
menu-open-work = Å&pne prosjekt…
menu-new-window = Nytt vind&u
menu-import-from = &Importer fra
menu-create-from = &Opprett fra
menu-import-plume = &Plume Creator (.plume)…
menu-import-document = &Dokumenter (Markdown, Word, ODT)…
menu-import-manuskript = &Manuskript (.msk)…
menu-import-scrivener = &Scrivener (.scriv)…
menu-export = &Eksporter
menu-export-book = Eksporter bok
menu-export-part = Eksporter del
menu-export-chapter = Eksporter kapittel
menu-export-scene = Eksporter scene
menu-export-note = Eksporter notat
menu-export-paratext = Eksporter paratekst
menu-export-folder = Eksporter mappe
menu-export-choose = Velg…
menu-export-none = Åpne et dokument som skal eksporteres
menu-save = &Lagre
menu-save-as-file = Lagre som én &fil…
menu-save-as-folder = Lagre som &mappe…
menu-backup = &Sikkerhetskopier nå
menu-close-work = Lukk p&rosjekt
menu-welcome = &Velkommen…
menu-settings = Inns&tillinger
menu-quit = &Avslutt

menu-view = &Vis
menu-outline = &Disposisjon
menu-search = &Søk i prosjektet
menu-timeline = &Gå tilbake i tid
menu-search-preview = Søke&forhåndsvisning
menu-fullscreen = F&ullskjerm
menu-focus-mode = Distraksjonsfri m&odus
menu-format = F&ormat
menu-scene-break = Sett inn s&ceneskift
menu-major-scene-break = Sett inn større scenesk&ift

menu-go = &Gå
menu-go-next-scene = Neste &scene
menu-go-prev-scene = Forrige s&cene
menu-go-next-chapter = Neste &kapittel
menu-go-prev-chapter = Forrige ka&pittel
menu-go-next-note = Neste &notat
menu-go-prev-note = Forrige no&tat

menu-tools = Ver&ktøy
menu-help = &Hjelp

ctx-add = &Legg til
ctx-new-item = &Nytt element
ctx-new-folder = Ny &mappe
ctx-rename = &Gi nytt navn
ctx-duplicate = &Dupliser
ctx-import-here = &Importer hit…
ctx-indent = &Rykk inn
ctx-outdent = Rykk &ut
ctx-trash = Flytt til &papirkurven
ctx-open-to-side = Åpne ved &siden av
ctx-open = &Åpne
ctx-reveal-in-outline = Vis i &disposisjonen
ctx-move-up = Flytt &opp
ctx-move-down = Flytt &ned

ctx-tab-close = &Lukk
ctx-tab-close-others = Lukk &andre
ctx-tab-close-all = Lukk all&e
ctx-tab-open-to-side = Åpne ved &siden av
ctx-tab-open-in-main = Åpne i &hovedruten
ctx-tab-move-to-side = &Flytt til siden
ctx-tab-move-to-main = &Flytt til hovedruten
ctx-tab-move-to-new-window = Flytt til et &nytt vindu
ctx-tab-move-window-unsaved = Lagre dette prosjektet først: et nytt vindu åpner en fil på disken
ctx-tab-pin = Fes&t denne fanen
ctx-tab-unpin = Løsne &denne fanen
tab-pinned-tooltip = Festet: «Lukk andre» og «Lukk alle» lar den stå åpen

create-book = Bok
create-part = Del
create-chapter = Kapittel
create-scene = Scene
create-note = Notat
create-note-folder = Notatmappe
create-folder = Mappe
create-paratext = Paratekst
create-paratext-folder = Paratekstmappe
create-book-end = Bokslutt
create-story-bible-entry = Oppføring i historiebibelen…
type-book-start = Bokstart
type-text = Tekst

new-item-book = Ny bok
new-item-part = Ny del
new-item-chapter = Nytt kapittel
new-item-note = Nytt notat
new-item-note-folder = Ny notatmappe
new-item-folder = Ny mappe
new-item-paratext = Ny paratekst
new-item-paratext-folder = Ny paratekstmappe
new-item-story-bible-entry = Ny oppføring i historiebibelen

placement-inside = inni
placement-after = etter
placement-after-parent = etter overordnet
placement-top-level = toppnivå


ctx-promote = &Konverter til
promote-chapter-folder = Kapittelmappe
promote-flat-chapter = Flatt kapittel
promote-lossy-title = Ingen plass til teksten
promote-lossy-text = En { $target } har ingen plass til dette: { $kinds }. Flytt eller tøm den teksten først, og konverter deretter.
content-scene-text = Scenetekst
content-note-text = Notattekst
content-book-title = Boktittel
content-book-subtitle = Bokundertittel
content-part-title = Deltittel
content-chapter-title = Kapitteltittel
content-epigraph-text = Epigraf
content-paratext-text = Paratekst
promote-blocked-title = Kapittelet er ikke tomt
promote-blocked-text = { $count ->
    [one] Dette kapittelet inneholder fortsatt { $count } element. Flytt det eller legg det i papirkurven før du konverterer kapittelet til et flatt kapittel.
   *[other] Dette kapittelet inneholder fortsatt { $count } elementer. Flytt dem eller legg dem i papirkurven før du konverterer kapittelet til et flatt kapittel.
}

inspector = Inspektør
inspector-empty = Åpne et element for å se egenskapene.
inspector-promote = Konverter til…
statusbar-toggle-outline = Vis/skjul permen
statusbar-toggle-inspector = Vis/skjul inspektøren
statusbar-save-unsaved = Ulagrede endringer. Klikk for å lagre
statusbar-save-saved = Alle endringer lagret
statusbar-save-autosave = Autolagring er på. Endringene lagres mens du skriver
statusbar-saving = Lagrer…
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
session-readout = { $words ->
    [one] { $words } ord
   *[other] { $words } ord
} · { $time }
session-readout-timed = { $words ->
    [one] { $words } ord
   *[other] { $words } ord
} · { $time } igjen
statusbar-focus-exit = Avslutt distraksjonsfri modus
statusbar-focus-synopsis = Synopsis
statusbar-focus-go-prev = Forrige element (Alt+Opp)
statusbar-focus-go-next = Neste element (Alt+Ned)

rail-settings = Innstillinger

language = Språk
theme = Tema
english = Engelsk
french = Fransk
light = Lys
dark = Mørk
settings-text-width = Tekstbredde
settings-preview-width = Bredde på søkeforhåndsvisning
settings-autosave = Lagre automatisk til disk
settings-show-welcome = Vis startvinduet ved oppstart
settings-show-welcome-tip = Når dette er av, åpnes det siste prosjektet i stedet.
settings-check-for-updates = Se etter nye versjoner
settings-check-for-updates-tip = Høyst én gang om dagen spør Skribisto prosjektets nettsted hvilken versjon som er gjeldende. Forespørselen inneholder ingen identifikator, og ikke engang versjonen du kjører: sammenligningen skjer på denne datamaskinen.

settings-title = Innstillinger
settings-close = Lukk
settings-search = Søk i innstillinger
settings-reset = Tilbakestill til standardverdier
settings-done = Ferdig
settings-cancel = Avbryt
settings-apply = Bruk
settings-ok = OK
settings-reset-confirm-title = Vil du tilbakestille alle innstillinger til standard?
settings-reset-confirm-body = Gjenoppretter programmets innstillinger for utseende, redigering og skriving. Prosjektets egne innstillinger, hurtigtaster og lagrede stiler påvirkes ikke. Dette kan ikke angres.
settings-empty-title = Ingen innstillinger her ennå
settings-empty-hint = Denne delen får flere valg i en fremtidig oppdatering.

settings-sec-appearance-behaviour = Utseende og oppførsel
settings-sec-editor = Redigering
settings-sec-spelling = Rettskriving
settings-sec-backup = Sikkerhetskopi og synkronisering
settings-sec-compile = Kompilering og eksport
settings-page-appearance = Utseende
settings-field-image-size-policy = Store bilder
settings-image-policy-ask = Spør hver gang
settings-image-policy-keep = Behold originalen
settings-image-policy-downscale = Optimaliser
settings-hint-image-size-policy = Hva som skal skje når et innsatt bilde er større enn manuset trenger.
settings-page-notifications = Varsler
settings-page-scene = Scene
settings-page-synopsis = Synopsis
settings-page-notes = Notater
settings-page-editor-behavior = Redigeringsoppførsel
settings-page-goals = Mål og ordtelling
settings-page-games = Skrivelek
settings-page-corkboard = Oppslagstavle
settings-page-distraction-free = Distraksjonsfri
settings-page-dictionaries = Ordbøker
settings-page-autosave = Autolagring
settings-page-export = Eksportformater
settings-page-paratext = Paratekststrukturer
settings-page-keymap = Hurtigtaster
settings-keymap-filter = Filtrer snarveier

settings-desc-sec-appearance-behaviour = Hvordan selve programmet ser ut, og hva det gjør når det starter.
settings-desc-sec-editor = Skriveflaten: hvordan den ser ut, og hva den gjør mens du skriver.
settings-desc-sec-spelling = Stavekontroll og ordbøkene bak den.
settings-desc-sec-backup = Hvordan arbeidet ditt havner på disken, og hvilke kopier som tas vare på.
settings-desc-sec-compile = Hva som forlater Skribisto, og i hvilken form.
settings-desc-sec-work = Innstillinger som hører til dette prosjektet og følger med i filen.
settings-desc-sec-extensions = Sider fra utvidelsene som er installert her.
settings-desc-group-typography = Én side per teksttype: skrift, størrelse, linjehøyde og avstand.
settings-desc-appearance = Grensesnittspråk, tema, tekststørrelse og startvinduet ved oppstart.
settings-desc-notifications = Alle meldinger som er vist i denne økten, og handlingene du kan utføre på nytt.
settings-desc-scene = Hvordan sceneteksten settes: skrift, størrelse, linjehøyde, innrykk og avstand.
settings-desc-synopsis = Hvordan synopsisruten settes, uavhengig av manuset.
settings-desc-notes = Hvordan notater settes, uavhengig av manuset.
settings-desc-corkboard = Kortstørrelse, hva et kort viser, og hvordan tavlen ordner dem.
settings-desc-distraction-free = Typografi, spaltebredde og kontrollstripe for skriving i fullskjerm.
settings-desc-distraction-free-themes = Temabiblioteket for skriving i fullskjerm: de medfølgende og dine egne.
settings-desc-editor-behavior = Tekstbredde, plassering av synopsis, skrivemaskinrulling og markeringen rundt markøren.
settings-desc-punctuation = Den typografiske husstilen nye prosjekter starter med. Alle prosjekter følger denne med mindre de har sin egen.
settings-desc-goals = Mål for ord og tegn, og hvordan ord telles.
settings-desc-games = Begrensninger du setter for deg selv mens du skriver utkast, som Alltid fremover.
settings-desc-spellcheck = Den ene bryteren som slår stavekontroll av og på overalt.
settings-desc-dictionaries = Installer, fjern og bla gjennom staveordbøkene på denne maskinen.
settings-desc-autosave = Om endringene dine skrives til disk av seg selv.
settings-desc-backup = Når kopier tas, hvor de lagres, og hvor mange. Alle prosjekter følger dette med mindre de har sine egne innstillinger.
settings-desc-export = Stilene hver eksport kompileres gjennom: de medfølgende og dine egne.
settings-desc-paratext = For- og ettertekstene et nytt prosjekt kan starte med.
settings-desc-keymap = Alle hurtigtaster, og hva de er knyttet til.
settings-page-user = Bruker
settings-desc-user = Hvem du er, for kommentarene du skriver.
settings-group-identity = Identitet
settings-field-user-name = Navnet ditt
settings-field-user-name-placeholder = Valgfritt
settings-field-user-name-hint = Signerer kommentarene og svarene du skriver. Ikke det samme som prosjektets forfatternavn, som står på boken og følger med i filen.
settings-field-user-initials = Initialene dine
settings-field-user-initials-placeholder = Fra navnet ditt
settings-field-user-initials-hint = Det en tekstbehandler viser ved siden av kommentaren din i margen. La feltet stå tomt for å bruke initialene som vises, hentet fra navnet ditt.
settings-field-user-hint = Begge er valgfrie og gjelder alle prosjekter på denne datamaskinen. Endrer du dem, signeres kommentarene du skriver heretter med de nye. Kommentarer som allerede er skrevet, beholder navnet de ble skrevet under.
settings-desc-author = Navnet som står på dette prosjektet.
settings-desc-structure = Om dette prosjektets kapitler er mapper eller enkeltelementer.
settings-desc-language = Språket dette prosjektets tekst kontrolleres mot.
settings-desc-work-backup = Dette prosjektets egne regler for sikkerhetskopiering, eller de generelle.
settings-desc-personal-dictionary = Ordene dette prosjektet regner som riktig stavet.
settings-desc-tags = De fargekodede stikkordene dette prosjektet merker elementene sine med.
settings-desc-templates = De notatmalene du kan sette inn mens du skriver i dette prosjektet.
settings-desc-text-replacements = Forkortelser som utvides mens du skriver i dette prosjektet.
settings-desc-work-punctuation = Anførselstegnene, tankestrekene og mellomrommene dette prosjektets tekst følger.

settings-group-typography = Typografi
settings-group-writing-column = Skrivespalte
settings-group-writing-view = Skrivevisning
settings-group-distraction-free-strip = Kontrollstripe
settings-group-theme = Tema
settings-group-language = Språk
settings-group-startup = Oppstart
settings-group-updates = Oppdateringer
settings-group-autosave = Autolagring
settings-field-typeface = Skrift
settings-field-size = Tekststørrelse

editor-size-changed-manuscript = Tekststørrelse i manus: { $percent }
editor-size-changed-synopsis = Tekststørrelse i synopsis: { $percent }
editor-size-changed-notes = Tekststørrelse i notater: { $percent }
editor-size-changed-corkboard = Tekststørrelse på oppslagstavlens kort: { $percent }
editor-size-changed-corkboard-expanded = Tekststørrelse i utvidet redigering: { $percent }
editor-size-changed-distraction-free = Tekststørrelse i distraksjonsfri modus: { $percent }

menu-text-size-increase = Stø&rre tekst
menu-text-size-decrease = M&indre tekst
menu-text-size-reset = &Tilbakestill tekststørrelse
settings-field-line-height = Linjehøyde
settings-field-first-line-indent = Innrykk i første linje
settings-field-paragraph-spacing-before = Avstand før avsnitt
settings-field-paragraph-spacing-after = Avstand etter avsnitt
settings-field-column-width = Spaltebredde
settings-distraction-free-width-hint = Gjelder bare mens distraksjonsfri modus er på. Spaltebreddene for scene, synopsis og notater endres ikke.
settings-distraction-free-title = Behold elementets navn
settings-distraction-free-word-count = Behold antall ord
settings-distraction-free-session = Behold skriveøkten
settings-distraction-free-go-to = Behold Gå til…-knappen
settings-distraction-free-go = Behold Forrige- og Neste-knappene
settings-distraction-free-chrome-hint = Avslutt-knappen blir alltid stående, uansett hva du velger her. Den er veien ut hvis Escape er opptatt.
settings-field-app-theme = Tema
settings-theme-light = Lys
settings-theme-dark = Mørk
settings-field-text-scale = Tekststørrelse i grensesnittet
settings-field-language = Grensesnittspråk
settings-synopsis-placement = Plassering av synopsis
settings-synopsis-placement-none = Ingen
settings-synopsis-placement-top = Over
settings-synopsis-placement-side = Ved siden av
settings-typewriter = Skrivemaskinrulling
settings-typewriter-tip = Holder linjen du skriver på i fast høyde mens manuset ruller under den. Klikker du, havner markøren fortsatt der du klikker.
settings-typewriter-position = Linjeposisjon
settings-typewriter-position-top-third = Øverste tredjedel
settings-typewriter-position-middle = Midten
settings-typewriter-position-bottom-quarter = Nederste fjerdedel
settings-highlight-scope = Marker rundt markøren
settings-highlight-scope-none = Ingen
settings-highlight-scope-sentence = Setning
settings-highlight-scope-paragraph = Avsnitt
settings-highlight-scope-tip-none = La siden være ren. Ingenting skyggelegges mens du skriver.
settings-highlight-scope-tip-sentence = Skyggelegg setningen du skriver, så den skiller seg ut fra dem rundt.
settings-highlight-scope-tip-paragraph = Skyggelegg hele avsnittet du skriver, så du har partiet du arbeider med, i fokus.
settings-group-container-views = Visninger for beholdere
settings-remember-view = Husk den siste visningen for hver elementtype
settings-remember-view-tip = Åpne en beholder i visningen du sist brukte for den typen
settings-remember-view-tip-more =
    En bok, en del og et kapittel har hver flere visninger (sin egen side, hele
    manuset, hele synopsisen). Slår du dette på, åpnes hver type igjen i
    visningen du sist valgte for den. Bytter du for eksempel ett kapittel til
    Hele kapittelet, viser også neste kapittel du åpner Hele kapittelet. Hver type husker sin egen visning.
settings-group-counting = Ordtelling
settings-counting-auto = Automatisk (etter språk)
settings-counting-whitespace = Del på mellomrom
settings-counting-unicode-words = Unicode-ord
settings-counting-cjk-hybrid = CJK-tilpasset (per tegn)
settings-counting-hint = Automatisk teller kinesisk og japansk per tegn og alle andre språk per ord. Endre dette bare hvis tellingen ser feil ut for språket ditt.
settings-group-goals-display = Visning
settings-show-characters = Vis antall tegn i statuslinjen
settings-autosave-hint = Endringene skrives automatisk til disken mens du skriver.

settings-sec-work = Prosjekt
settings-page-structure = Struktur
settings-page-author = Forfatter
settings-field-author-name = Forfatternavn
settings-field-author-placeholder = Valgfritt
settings-field-author-hint = Vises på den kompilerte tittelsiden og i metadataene til eksporterte filer. La feltet stå tomt for å utelate det. Dette er navnet som står på boken. Navnet kommentarene dine signeres med, finner du under Innstillinger ▸ Bruker.
settings-group-chapters = Kapitler
tidy-titles-title = Rydd i kapitteltitler
tidy-titles-none = Ingen kapittel- eller deltitler gjentar bare sitt eget nummer.
tidy-titles-lead = { $count ->
        [one] Ett kapittel eller én del har ikke annet enn sitt eget nummer som tittel.
       *[other] { $count } kapitler og deler har ikke annet enn sitt eget nummer som tittel.
    }
tidy-titles-explain = Når disse titlene tømmes, får hvert av dem navn etter nummeret boken allerede kjenner, det samme nummeret eksporten skriver ut. Ingenting annet endres, og én angring legger alt tilbake.
menu-document-tidy-titles = R&ydd i kapitteltitler…
settings-group-numbering = Nummerering
settings-number-chapters = Nummerer kapitler og deler
settings-number-chapters-tip = Kapitler og deler får et nummer ut fra plasseringen i boken. Det vises ved siden av tittelen her og skrives ut av eksporten.
settings-number-chapters-tip-more = Nummeret lagres aldri i tittelen, så det stemmer fortsatt når du endrer rekkefølgen, setter inn eller sletter. Slår du dette av, skriver eksporten bare ut titlene, uansett hva eksportstilen ber om. Vil du holde ett kapittel utenfor (en prolog, et mellomspill), bruker du heller Nummerering-bryteren i inspektøren: da blir kapittelet i boken, men får ikke noe nummer.
settings-part-resets-chapter = Start kapittelnummereringen på nytt i hver del
settings-part-resets-chapter-tip = Av som standard: kapitlene nummereres fortløpende gjennom bokens deler, så «Del to» begynner med kapittel elleve.
settings-part-resets-chapter-tip-more = Slik er vanlig praksis i bransjen, og det leseren venter seg. Slå det på for en bok der delene skal leses som separate bind, der hver del begynner med kapittel én.
settings-chapter-flat = Flate kapitler
settings-chapter-flat-hint = På: et kapittel er ett enkelt element. Du skriver i det, og det inneholder ingen scener. Av: et kapittel er en mappe. Du skriver fortsatt i det, men det kan også inneholde scener. Nye kapitler følger denne innstillingen; eksisterende konverteres med Konverter til.

settings-styles-builtin = Innebygde stiler
settings-styles-user = Mine stiler
settings-styles-builtin-badge = Innebygd
settings-styles-duplicate = Dupliser
settings-styles-edit = Rediger
settings-styles-delete = Slett
settings-styles-import = Importer…
settings-styles-export = Eksporter…
settings-styles-copy-suffix = (kopi)
settings-styles-json-filter = Eksportstil
settings-styles-editor-title = Rediger stil
settings-styles-editor-none = Velg en egen stil for å redigere den, eller dupliser en innebygd.
settings-styles-imported = Stil importert
settings-styles-import-failed = Kunne ikke importere stil
settings-styles-exported = Stil eksportert
settings-styles-export-failed = Kunne ikke eksportere stil
settings-styles-field-name = Navn
settings-styles-field-chapters = Kapitteloverskrifter
settings-styles-field-parts = Deloverskrifter
settings-styles-field-scene-break = Sceneskift
settings-styles-field-major-scene-break = Større sceneskift
settings-styles-field-spacing = Linjeavstand
settings-styles-field-justify = Blokkjuster tekst
settings-styles-field-notes = Ta med notater
settings-styles-field-synopses = Ta med synopser
settings-styles-field-scene-titles = Ta med scenetitler
settings-styles-field-epigraphs = Ta med epigrafer
settings-styles-field-paratexts = Ta med paratekster
settings-styles-field-epigraph-placement = Plassering av epigraf
settings-styles-epigraph-after = Etter tittelen
settings-styles-epigraph-before = Før tittelen
settings-styles-field-footnotes = Ta med fotnoter
settings-styles-field-footnote-numbering = Fotnotenummerering
settings-styles-footnote-numbering-continuous = Fortløpende
settings-styles-footnote-numbering-per-chapter = Start på nytt i hvert kapittel
settings-styles-footnote-numbering-per-book = Start på nytt i hver bok
settings-styles-field-images = Bilder
menu-image = &Bilde
settings-styles-images-beside = Ved siden av dokumentet
settings-styles-images-embed = Inne i dokumentet
settings-styles-images-omit = Utelat
settings-styles-group-round-trip = Sende til en redaktør
settings-styles-field-comments = Ta med kommentarer
settings-styles-field-round-trip-marks = Ta med returmarkører
settings-styles-round-trip-hint = Bare DOCX og ODT. Markørene er usynlige identifikatorer som lar en fil som kommer tilbake, oppdatere dette prosjektet i stedet for å havne ved siden av som en ekstra kopi.
settings-styles-group-pages = Sider
settings-styles-field-cover = Begynn med omslaget
settings-styles-field-word-count = Antall ord på tittelsiden
settings-styles-field-page-books = Ny side ved hver bok
settings-styles-field-page-parts = Ny side ved hver del
settings-styles-field-page-chapters = Ny side ved hvert kapittel
settings-styles-field-page-paratexts = Ny side ved hver paratekst
settings-styles-heading-none = Ingen overskrift
settings-styles-heading-numbered = Bare nummer
settings-styles-heading-title = Bare tittel
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
welcome-search = Søk i prosjekter
welcome-open = Åpne
welcome-new-work = Nytt prosjekt
welcome-create-from = Opprett fra…
welcome-recent-works = Nylige prosjekter
welcome-empty-recents = Ingen nylige prosjekter ennå.
welcome-no-matches = Ingen nylige prosjekter samsvarer med søket.
welcome-learn-soon = Guider og tips kommer snart.
welcome-about-blurb = Skribisto, skriveprogrammet skrevet på nytt i Rust + Teksilo.
welcome-tagline = *Et stille sted å skrive lange ting.*
welcome-github = GitHub
welcome-discord = Discord
nav-works = Prosjekter
nav-examples = Eksempler
nav-learn = Lær
nav-about = Om

synopsis = Synopsis
epigraph = Epigraf
pane-manuscript = Manus
corkboard = Oppslagstavle
overview = Oversikt

corkboard-card-count = { $count ->
    [one] { $count } kort
   *[other] { $count } kort
}
corkboard-child-count = { $count ->
    [one] { $count } element
   *[other] { $count } elementer
}
corkboard-view-nested = Nestet
corkboard-view-flat = Flat
corkboard-layout-hint = Nestet viser en beholders direkte underelementer. Åpne et mappekort for å gå inn i det. Flat viser alle scenene i hele beholderen samtidig.
corkboard-card-size = Kortstørrelse
corkboard-search-placeholder = Filtrer kort…
corkboard-empty-title = Ingenting her ennå
corkboard-empty-hint = Bruk «＋ Ny» ovenfor for å legge til det første.
corkboard-new = Ny
corkboard-show-card-numbers = Nummerer kortene
corkboard-modal-size = Størrelse på utvidet redigering
corkboard-scope-hint = Gjelder alle åpne oppslagstavler i alle prosjekter.
corkboard-sort-manuscript = Rekkefølge i manus
corkboard-sort-title-asc = Tittel A–Å
corkboard-sort-title-desc = Tittel Å–A
corkboard-card-number = Kort { $number }

duplicate = Dupliser
duplicate-n = { $count ->
    [one] Dupliser { $count } kort
   *[other] Dupliser { $count } kort
}
set-label-n = { $count ->
    [one] Sett etikett på { $count } kort
   *[other] Sett etikett på { $count } kort
}
move-to-trash-n = { $count ->
    [one] Flytt { $count } kort til papirkurven
   *[other] Flytt { $count } kort til papirkurven
}
reveal-in-outline = Vis i disposisjonen

corkboard-move-to = Flytt til…
corkboard-move-to-n = { $count ->
    [one] Flytt { $count } kort til…
   *[other] Flytt { $count } kort til…
}
corkboard-move-picker-title = Flytt til…
corkboard-move-picker-empty = Ingen permer ennå. Opprett en først.
corkboard-move-picker-cancel = Avbryt
corkboard-move-here = Flytt hit
corkboard-moved-ok = { $count ->
    [one] { $count } kort flyttet
   *[other] { $count } kort flyttet
}
corkboard-move-into-self = En beholder kan ikke flyttes inn i seg selv. Velg et sted utenfor den.
corkboard-move-failed = Disse kortene kunne ikke flyttes dit.

overview-col-title = Tittel
overview-col-type = Type
overview-col-label = Etikett
overview-col-tags = Stikkord
overview-col-books = Bøker
overview-col-own-words = Ord
overview-col-total-words = Totalt
overview-row-count = { $count ->
    [one] { $count } element
   *[other] { $count } elementer
}
overview-search-placeholder = Filtrer elementer…
overview-expand-all = Utvid alle
overview-collapse-all = Fold sammen alle
overview-table-label = Innhold
overview-empty-title = Ingenting her ennå
overview-gone-title = Denne beholderen er borte
overview-gone-hint = Den ble flyttet til papirkurven. Gjenopprett den, eller lukk denne fanen.
overview-empty-hint = Bruk «＋ Ny» ovenfor for å legge til det første.
corkboard-grid-label = Kort på oppslagstavlen
corkboard-rename-field = Gi elementet nytt navn
corkboard-expand-synopsis = Utvid synopsis
corkboard-synopsis-modal-title = Synopsis
corkboard-badge-scene = Scene
corkboard-badge-chapter = Kapittel
corkboard-badge-part = Del
corkboard-badge-book = Bok
corkboard-badge-note = Notat
corkboard-badge-folder = Mappe
corkboard-badge-paratext = Paratekst
corkboard-badge-text = Tekst
corkboard-badge-end = Slutt
settings-group-corkboard-layout = Oppsett
settings-group-corkboard-cards = Kort
corkboard-show-word-count = Vis antall ord på kortene
text-heading = Tekst
no-content = Dette elementet har ikke noe redigerbart innhold.
untitled = Uten tittel
placeholder-title = Tittel…
placeholder-subtitle = Undertittel…
placeholder-chapter-title = Kapitteltittel…
split-editor = Del redigeringsruten
close-split-view = Lukk delt visning
drop-open-here = Åpne her
drop-open-to-side = Åpne ved siden av

segment-chapter = Kapittel
segment-part = Del
segment-book = Bok
segment-pace = Tempo
segment-notes = Notater
segment-story-bible = Historiebibel
segment-note-own = Notat
segment-note-details = Detaljer
segment-note-in-prose = I teksten
pace-placeholder = Tempoplanleggeren vises her.
pace-empty-title = Planlegg denne bokens tempo
pace-empty-body = Sett et ordmål og en frist, så regner Skribisto ut hvor mye du må skrive per dag for å nå det.
pace-start-planning = Begynn å planlegge
pace-section-schedule = Plan
pace-section-progress = Fremdrift
pace-advancement = Framgang
pace-goal = Ordmål
pace-deadline = Frist
pace-active = Tempo aktivt
pace-day-mon = Man
pace-day-tue = Tir
pace-day-wed = Ons
pace-day-thu = Tor
pace-day-fri = Fre
pace-day-sat = Lør
pace-day-sun = Søn
pace-card-written = ord skrevet
pace-card-of-goal = av målet
pace-card-rate = ord / skrivedag
pace-card-days-left = skrivedager igjen
pace-card-streak = dager på rad
pace-card-ahead = ord foran tempoet
pace-card-behind = ord bak tempoet
pace-charts-empty = Fremdriftsdiagrammer vises her når antall ord er registrert ved en lagring.
pace-chart-progression = Ord skrevet mot mål
pace-chart-words-per-day = Ord per dag
pace-series-actual = Faktisk
pace-series-target = Mål
pace-series-words-per-day = Ord/dag
pace-daily-target-line = Jevnt tempo: { $count ->
    [one] { $count } ord/dag
   *[other] { $count } ord/dag
}
pace-section-holidays = Fridager
pace-section-milestones = Milepæler
pace-holidays-none = Ingen fridager. Alle planlagte dager teller.
pace-holiday-label = Navn på fridag
pace-add-holiday = Legg til
pace-remove = Fjern
pace-milestones-none = Ingen milepæler ennå. Sett en på en del eller et kapittel i inspektøren.
full-chapter = Hele kapittelet
full-part = Hele delen
full-book = Hele boken
full-synopsis = Hele synopsisen
rename = Gi nytt navn
set-label = Sett etikett
insert-scene = Sett inn scene
insert-chapter = Sett inn kapittel
split-scene = Del scene
move-up = Flytt opp
move-down = Flytt ned
merge-with-previous = Slå sammen med forrige
move-to-trash = Flytt til papirkurven
rename-chapter = Gi kapittelet nytt navn
add-scene = Legg til ny scene
add-chapter = Legg til nytt kapittel
new-scene-title = Ny scene
placeholder-scene-name = Scenenavn…
menu-cut = Klipp ut
menu-copy = Kopier
menu-paste = Lim inn
menu-paste-unformatted = Lim inn uten formatering
menu-select-all = Merk alt
format-bold = Fet
format-italic = Kursiv
format-underline = Understreket
format-strikethrough = Gjennomstreket

binder = Perm
no-work = Ingen prosjekter lastet inn
no-recent-works = Ingen nylige prosjekter
switcher-open-section = Åpent nå
switcher-recent-section = Nylig
switcher-this-window = dette vinduet
open-project-title = Åpne prosjekt
open-project-question = Hvordan vil du åpne «{ $title }»?
open-in-new-window = Åpne i nytt vindu
open-here = Åpne her

binder-all = Alle permer
binder-show-all = Vis alle permer
binder-new = Ny perm…
binder-default-name = Ny perm
binder-item-count = { $count ->
    [one] { $count } element
   *[other] { $count } elementer
}
binder-search-placeholder = Filtrer disposisjonen…
binder-search-scope = Søk i alle permer
binder-trash-confirm-title = Vil du flytte permen til papirkurven?
binder-trash-confirm-text = «{ $name }» og alle elementene i den flyttes til papirkurven.

dialog-rename = Gi nytt navn
dialog-new-binder = Ny perm
dialog-set-label = Sett etikett
dialog-new-scene = Ny scene
close-work-question = Vil du lagre endringene før prosjektet lukkes?
quit-question = Vil du lagre endringene før du avslutter?
quit-save-work-question = Vil du lagre endringene i { $title } før du avslutter?
unsaved-changes = Prosjektet har ulagrede endringer.
new-work-unsaved-question = Vil du lagre endringene før du oppretter et nytt prosjekt?
open-work-unsaved-question = Vil du lagre endringene før du åpner et annet prosjekt?
switch-backup-discard-title = Vil du forkaste endringene i denne sikkerhetskopien?
switch-backup-discard-text = Endringer i en sikkerhetskopi kan ikke lagres i den. Bruk Lagre som eller Gjenopprett for å beholde dem, eller forkast dem og åpne det andre prosjektet.
switch-save-failed = Prosjektet kunne ikke lagres, så det ble ikke erstattet: { $error }
switch-save-not-started = Prosjektet kunne ikke lagres, så det ble ikke erstattet.
close-save-failed = Prosjektet kunne ikke lagres, så det ble ikke lukket: { $error }
close-save-not-started = Prosjektet kunne ikke lagres, så det ble ikke lukket.
save-not-started = Prosjektet kunne ikke lagres.

could-not-open-work = Kunne ikke åpne «{ $file }»
could-not-open-work-too-new = «{ $file }» krever en nyere Skribisto
could-not-open-work-too-new-detail = Lagret med Skribisto-format { $written_by }. Det trengs format { $requires } eller nyere for å åpne det, og denne versjonen støtter opptil format { $supported }. Oppdater Skribisto for å åpne det.
could-not-open-work-importing = «{ $file }» skrives av en import
could-not-open-work-importing-detail = En import skriver over dette prosjektet og er ikke ferdig. Åpne det når importen er ferdig: åpner du det nå, viser det prosjektet importen erstatter, og neste lagring ville skrive det prosjektet tilbake over importen.
could-not-open-example = Kunne ikke åpne eksemplet: { $error }
could-not-open-link = Kunne ikke åpne { $url }: { $error }
could-not-create-work = Kunne ikke opprette prosjekt: { $error }
saving-as-file = Lagrer som { $target }…
saving-as-folder = Lagrer som { $target }/…
saved-as = Lagret til { $target }
save-error = Kunne ikke lagre: { $error }
save-flush-failed = Lagringen ble fullført, men disse dokumentene kunne ikke sendes med og er fortsatt ulagret: { $titles } ({ $error })
backup-error = Kunne ikke sikkerhetskopiere: { $error }
backing-up = Sikkerhetskopierer…
backup-nothing-open = Ingen prosjekter er åpne for sikkerhetskopiering.
backup-already-running = En sikkerhetskopi er allerede i gang.
backup-complete = Sikkerhetskopiering fullført ({ $ok } lagret, { $skipped } allerede oppdatert)
backup-partial = Sikkerhetskopiering ferdig: { $ok } lagret, { $failed ->
    [one] { $failed } lagringssted mislyktes
   *[other] { $failed } lagringssteder mislyktes
}
backup-no-destination-title = Ingen plassering for sikkerhetskopi er tilgjengelig
backup-no-destination-text = Ingen av de konfigurerte lagringsstedene for sikkerhetskopier kan nås (for eksempel kan en ekstern disk være koblet fra). Koble den til og prøv igjen, eller avslutt uten å sikkerhetskopiere.
backup-failed-close-title = Sikkerhetskopien kunne ikke lagres
backup-failed-close-text = Ingen sikkerhetskopi kunne skrives før lukking. Alle lagringssteder mislyktes (disken kan være fjernet, full eller skrivebeskyttet). Rett opp feilen og prøv igjen, eller avslutt uten å sikkerhetskopiere.
backup-failed-close-error-text = Ingen sikkerhetskopi kunne skrives før lukking: { $error }. Rett opp feilen og prøv igjen, eller avslutt uten å sikkerhetskopiere.
backup-no-result = Sikkerhetskopieringen ble avsluttet uten å si om den laget en kopi. Se i listen over sikkerhetskopier om det har kommet en ny.

backup-choice-title = Sikkerhetskopifil
backup-choice-heading = Du har åpnet en sikkerhetskopi
backup-choice-subtitle = Dette er en sikkerhetskopi av et prosjekt fra et bestemt tidspunkt.
backup-choice-subtitle-dated = Sikkerhetskopi tatt { $date }.
backup-choice-body = Du kan åpne den og redigere fritt, men endringer kan bare tas vare på med Lagre som. Den opprinnelige prosjektfilen røres ikke. Eller du kan gjenopprette dette prosjektet til nøyaktig denne sikkerhetskopien.
backup-choice-open = Åpne sikkerhetskopien
backup-choice-restore = Gjenopprett dette prosjektet til dette tidspunktet…
backup-choice-not-a-backup = Nei, åpne den på vanlig måte
backup-banner-title = Sikkerhetskopi: endringer kan ikke lagres her
backup-banner-description = Bruk Lagre som for å beholde endringene i en ny fil, eller Gjenopprett for å erstatte det opprinnelige prosjektet med denne sikkerhetskopien.
backup-banner-restore = Gjenopprett…
backup-banner-save-as = Lagre som…
backup-restore-original-missing = Finner ikke det opprinnelige prosjektet som skal gjenopprettes. Bruk Lagre som for å beholde denne sikkerhetskopien som et nytt prosjekt i stedet.
backup-restore-close-elsewhere-title = Prosjektet er åpent i et annet vindu
backup-restore-close-elsewhere-text = Prosjektet du gjenoppretter over, er åpent i et annet vindu. Lukk det der først, og prøv deretter igjen.
backup-restore-focus-window = Gå til det vinduet
backup-restore-confirm-title = Vil du gjenopprette denne sikkerhetskopien?
backup-restore-confirm-text = Den gjeldende versjonen av prosjektet kopieres til side som en sikkerhetskopi før den erstattes med denne.
backup-restore-confirm-ok = Gjenopprett
backup-restore-error = Kunne ikke gjenopprette: { $error }
backup-restored-ok = Prosjektet er gjenopprettet.
backup-restored-with-safety = Prosjektet er gjenopprettet. Den forrige versjonen ble lagret i { $path }.
close-backup-discard-title = Vil du forkaste endringene i denne sikkerhetskopien?
close-backup-discard-text = Endringer i en sikkerhetskopi kan ikke lagres i den. Bruk Lagre som for å beholde dem, eller forkast dem og lukk.
quit-backup-discard-title = Vil du forkaste endringene og avslutte?
quit-backup-discard-work-question = Vil du forkaste endringene i { $title } og avslutte?
quit-backup-discard-text = Endringer i en sikkerhetskopi kan ikke lagres i den. Bruk Lagre som for å beholde dem, eller forkast dem og avslutt.
backup-nudge-text = Ingen sikkerhetskopiering er satt opp for dette prosjektet.
backup-nudge-action = Sett opp sikkerhetskopiering…

menu-backups-list = Sikkerhets&kopier…
backups-title = Sikkerhetskopier
backups-loading = Leter etter sikkerhetskopier…
backups-empty = Fant ingen sikkerhetskopier for dette prosjektet ennå.
backups-open = Åpne
backups-reveal = Vis i mappe
backups-delete = Slett denne sikkerhetskopien
backups-delete-confirm-title = Vil du slette denne sikkerhetskopien?
backups-delete-confirm-text = «{ $name }» slettes permanent. Dette kan ikke angres.
backups-delete-error = Kunne ikke slette sikkerhetskopien: { $error }
backups-refresh = Oppdater
backups-close = Lukk

settings-page-backup = Standard for sikkerhetskopiering
settings-page-work-backup = Sikkerhetskopier
settings-backup-general-title = Standardinnstillinger for sikkerhetskopiering
settings-backup-work-title = Sikkerhetskopier for dette prosjektet
settings-backup-inherit = Bruk de generelle innstillingene for sikkerhetskopiering
settings-backup-inheriting = Dette prosjektet bruker de generelle innstillingene for sikkerhetskopiering.
settings-backup-none-hint = Ingen automatisk sikkerhetskopiering er satt opp (alle utløsere er av).
settings-backup-last = Siste sikkerhetskopi: { $date }
settings-backup-last-never = Ingen sikkerhetskopier ennå.
settings-backup-open-list = Åpne listen over sikkerhetskopier…
settings-backup-triggers = Når det skal sikkerhetskopieres
settings-backup-on-close = Sikkerhetskopier når prosjektet lukkes
settings-backup-on-open = Sikkerhetskopier når prosjektet åpnes
settings-backup-interval = Sikkerhetskopier jevnlig, hver
settings-backup-destinations = Lagringssteder for sikkerhetskopier
settings-backup-default-location = Standard plassering
settings-backup-usage-since = { $count ->
    [one] { $count } sikkerhetskopi · { $size } · eldste { $oldest }
   *[other] { $count } sikkerhetskopier · { $size } · eldste { $oldest }
}
settings-backup-usage = { $count ->
    [one] { $count } sikkerhetskopi · { $size }
   *[other] { $count } sikkerhetskopier · { $size }
}
settings-backup-usage-empty = Ingenting lagret her ennå
settings-backup-usage-measuring = Måler…
settings-backup-reveal-root = Vis mappe
settings-backup-dest-none = Ingen lagringssteder. Sikkerhetskopier lagres ved siden av prosjektet.
settings-backup-dest-remove = Fjern
settings-backup-dest-add = Legg til mappe…
settings-backup-dest-refresh = Oppdater
settings-backup-retention = Hvor mange som skal beholdes
settings-backup-retention-tiered = Trinnvis
settings-backup-retention-keep-n = Behold de siste N
settings-backup-retention-tip = Hvordan gamle sikkerhetskopier ryddes bort.
settings-backup-retention-tip-more = Trinnvis beholder én sikkerhetskopi per time det siste døgnet, per dag den siste uken, per uke den siste måneden og per måned utover det, så nylig historikk er tett og eldre historikk tynnes ut. «Behold de siste N» beholder ganske enkelt de N nyeste kopiene. I begge modusene beholdes alltid de nyeste kopiene (minimumet nedenfor).
settings-backup-gfs-hourly = Hver time (siste 24 t)
settings-backup-gfs-daily = Daglig (siste uke)
settings-backup-gfs-weekly = Ukentlig (siste måned)
settings-backup-gfs-monthly = Månedlig
settings-backup-keep-n = Antall som beholdes
settings-backup-min-keep = Behold alltid minst
settings-backup-dedup = Hopp over sikkerhetskopiering når ingenting er endret

new-work-title = Nytt prosjekt
new-work-close = Lukk
new-work-name = Prosjektnavn
new-work-name-placeholder = Uten tittel
new-work-author = Forfatter
new-work-author-placeholder = Valgfritt
new-work-format = Format
new-work-single-file = Enkeltfil
new-work-bundle = Mappe
new-work-convert-later = Du kan konvertere mellom formater senere.
new-work-location = Plassering
new-work-will-create = Oppretter
new-work-language = Standardspråk
new-work-language-hint = Brukes på nye tekster og i stavekontrollen. Hver tekst kan settes til et annet språk.
new-work-template = Mal
new-work-template-none = Ingen
new-work-template-empty-novel = Tom roman
new-work-template-light-novel = Light novel
new-work-template-novel = Roman
new-work-template-novel-in-parts = Roman i deler
new-work-template-notebook = Notatbok
new-work-cancel = Avbryt
new-work-create = Opprett prosjekt
new-work-step-details = Detaljer
new-work-step-language = Språk og struktur
new-work-step-template = Mal
new-work-back = Tilbake
new-work-next = Neste
new-work-documents-title = Nytt prosjekt fra dokumenter
new-work-step-import = Importer
new-work-create-and-import = Opprett og importer…
new-work-documents-next-title = Dokumentene dine kommer etterpå
new-work-documents-next-body = Når prosjektet opprettes, åpnes importveiviseren over det. Der velger du dokumentene, ser over strukturen Skribisto leser ut av dem, og velger hvor de skal havne. Ingenting skrives inn i prosjektet før du bekrefter det der.
new-work-documents-no-template = Dette prosjektet starter tomt med vilje: ingen mal, så de importerte dokumentene er det eneste som er i det.
new-work-documents-chapter-scene-hint = Gjelder kapitlene importen oppretter. Du kan endre dette senere i prosjektets innstillinger.
new-work-single-file-desc = Ett .skrib-arkiv (zip). Lett å flytte og lett å sikkerhetskopiere.
new-work-bundle-desc = En mappe med alle tekster og ressurser. Bedre egnet for versjonskontroll.
new-work-template-none-count = tom perm
new-work-template-empty-novel-count = 1 kapittel
new-work-template-light-novel-count = 15 kapitler
new-work-template-novel-count = 20 kapitler
new-work-template-novel-in-parts-count = 3 deler, 24 kapitler
new-work-template-notebook-count = frie notater
new-work-chapter-scene = Flate kapitler
new-work-chapter-scene-tip = Hvert kapittel er ett enkelt element du skriver rett inn i, uten scener under. La dette være av for det klassiske oppsettet, der et kapittel er en mappe: du skriver rett inn i den også, men den kan i tillegg inneholde scener.
new-work-chapter-scene-tip-more = Du skriver i et kapittel uansett. Den eneste forskjellen er om det kan *inneholde* scener. Skribistos permtre er bare til organisering, så begge oppsettene kompileres til den samme boken. Du kan blande dem fritt, og Konverter til gjør om et kapittel mellom de to uten å miste et eneste ord.
new-work-name-required = Skriv inn et navn på prosjektet
new-work-name-invalid = Dette navnet har ingen brukbare tegn
new-work-location-required = Velg en plassering
new-work-location-missing = Denne mappen finnes ikke
new-work-location-not-folder = Denne banen er ikke en mappe
new-work-location-readonly = Denne mappen er ikke skrivbar
new-work-target-exists = Det finnes allerede et prosjekt med dette navnet i denne mappen. Du blir spurt før det erstattes når prosjektet opprettes
new-work-target-is-folder = Det finnes allerede en mappe med dette navnet på denne plasseringen. Velg et annet navn
new-work-overwrite-title = Vil du erstatte det eksisterende prosjektet?
new-work-overwrite-text = «{ $name }» finnes allerede i denne mappen. Vil du erstatte det med det nye prosjektet?
new-work-target-open-title = Dette prosjektet er åpent
new-work-target-open-text = «{ $name }» er åpent i Skribisto. Et nytt prosjekt på samme sted ville erstatte prosjektet du arbeider med, og neste lagring ville skrive det prosjektet tilbake over det nye. Velg et annet navn eller en annen mappe, eller lukk prosjektet først.
target-importing-title = En import skriver dette prosjektet
target-importing-text = En import skriver fortsatt «{ $name }». Vent til den er ferdig, eller velg et annet navn.
backup-restore-importing-text = En import skriver fortsatt «{ $name }», prosjektet denne sikkerhetskopien ville bli gjenopprettet over. Gjenopprett sikkerhetskopien når importen er ferdig.

new-work-manuscript = Manus
new-work-notes = Notater
new-work-research = Research
new-work-notebook = Notatbok
new-work-scene = Scene
new-work-note = Notat
new-work-front-matter = Fortekst
new-work-back-matter = Ettertekst
new-work-paratext = Bokstruktur
new-work-paratext-none = Ingen struktur
new-work-paratext-hint = For- og etterteksten en tradisjon åpner og avslutter en bok med. Du kan flytte, gi nytt navn til eller slette hva som helst av det etterpå.
new-work-tags = Stikkord
new-work-tags-none = Ingen stikkord
new-work-tags-hint = Et utgangspunkt for å merke personer, steder og resten med stikkord. Valgfritt, og alle stikkord kan få nytt navn, ny farge eller slettes etterpå.
new-work-note-templates = Notatmaler
new-work-note-templates-none = Ingen maler
new-work-note-templates-hint = Formen et notat i historiebibelen starter med. Valgfritt, og alle maler kan redigeres eller slettes etterpå.
new-work-characters = Personer
new-work-places = Steder

import-plume-title = Importer Plume Creator-prosjekt
import-plume-source = Plume-prosjekt
import-plume-source-hint = Velg en .plume- eller .plume_backup-fil (alle versjoner av Plume Creator).
import-plume-trash-warning = ⚠ Elementer i papirkurven eller slettede elementer overføres ikke.
import-plume-source-required = Velg en Plume-prosjektfil
import-plume-source-missing = Denne filen finnes ikke
import-plume-source-not-file = Denne banen er ikke en fil
import-plume-manuscript-binder = Manus
import-plume-story-bible-binder = Historiebibel
import-plume-notes-of = { $name } (notater)
import-plume-content-of = { $name } (tekst)
import-plume-progress-title = Importerer Plume-prosjekt…
import-plume-done = Importerte { $imported ->
    [one] { $imported } element.
   *[other] { $imported } elementer.
} { $skipped ->
    [one] { $skipped } element fra papirkurven ble ikke overført.
   *[other] { $skipped } elementer fra papirkurven ble ikke overført.
}
import-plume-nested-too-deep = Prosjektet ble ikke importert: delen «{ $part }» er nestet mer enn { $limit } nivåer dypt. Ingen Plume Creator-prosjekter kommer i nærheten av det, og å lese det kunne få Skribisto til å krasje. Ingenting ble skrevet.
import-plume-declares-entities = Prosjektet ble ikke importert: delen «{ $part }» deklarerer XML-entiteter, noe ingen Plume Creator-prosjekter gjør. Å utvide dem kunne kreve mer minne enn datamaskinen har, og få Skribisto til å krasje. Ingenting ble skrevet.
import-plume-no-outline = Denne filen er et komprimert arkiv, men ikke et Plume Creator-prosjekt: den har ingen disposisjon. Ingenting ble skrevet.
import-plume-empty-outline = Disposisjonsfilen i dette Plume Creator-prosjektet er tom, så det er ingenting å importere. Ingenting ble skrevet.
import-plume-wrong-document = Dette er ikke et Plume Creator-prosjekt: delen «{ $part }» er et «{ $tag }»-dokument. Ingenting ble skrevet.
import-plume-too-new = Delen «{ $part }» i dette prosjektet ble skrevet av en nyere Plume Creator (versjon { $version }) enn Skribisto kan lese (opptil { $supported }). Ingenting ble skrevet.

import-manuskript-title = Importer Manuskript-prosjekt
import-manuskript-source = Manuskript-prosjekt
import-manuskript-source-hint = Velg .msk-filen eller prosjektmappen (alle versjoner av Manuskript).
import-manuskript-source-file = Velg fil…
import-manuskript-source-folder = Velg mappe…
import-manuskript-source-required = Velg et Manuskript-prosjekt
import-manuskript-source-missing = Denne filen eller mappen finnes ikke
import-manuskript-manuscript-binder = Manus
import-manuskript-story-bible-binder = Historiebibel
import-manuskript-characters-group = Personer
import-manuskript-world-group = Verden
import-manuskript-plots-group = Handlingslinjer
import-manuskript-project-info-note = Prosjektinformasjon
import-manuskript-summary-note = Sammendrag
import-manuskript-importance-minor = Mindre viktig
import-manuskript-importance-secondary = Sekundær
import-manuskript-importance-main = Hoved
import-manuskript-notes-of = { $title } (notater)
import-manuskript-progress-title = Importerer Manuskript-prosjekt…
import-manuskript-done = { $imported ->
    [one] { $imported } element importert.
   *[other] { $imported } elementer importert.
} { $revisions ->
    [0] { "" }
    [one] { $revisions } tidligere versjon fulgte med.
   *[other] { $revisions } tidligere versjoner fulgte med.
}
import-manuskript-nested-too-deep = Prosjektet ble ikke importert: filen «{ $part }» er nestet mer enn { $limit } nivåer dypt. Ingen Manuskript-prosjekter kommer i nærheten av det, og å lese den kunne få Skribisto til å krasje. Ingenting ble skrevet.
import-manuskript-folders-too-deep = Prosjektet ble ikke importert: en av filene ligger mer enn { $limit } mapper dypt. Ingen Manuskript-prosjekter kommer i nærheten av det, og å lese den kunne få Skribisto til å krasje. Ingenting ble skrevet.
import-manuskript-declares-entities = Prosjektet ble ikke importert: filen «{ $part }» deklarerer XML-entiteter, noe ingen Manuskript-prosjekter gjør. Å utvide dem kunne kreve mer minne enn datamaskinen har, og få Skribisto til å krasje. Ingenting ble skrevet.
import-manuskript-not-a-project-file = «{ $file }» er verken et Manuskript-prosjektarkiv eller den lille .msk-filen som peker på en prosjektmappe. Ingenting ble skrevet.
import-manuskript-folder-missing = «{ $stub }» peker på en prosjektmappe kalt «{ $folder }» ved siden av seg, og den mappen finnes ikke der. Et Manuskript-prosjekt i mappemodus består av begge, og de må flyttes sammen. Ingenting ble skrevet.
import-manuskript-not-a-project = Dette ser ikke ut som et Manuskript-prosjekt: det har verken Manuskript-markør eller disposisjon. Ingenting ble skrevet.
import-manuskript-format-too-new = Dette prosjektet er i Manuskript-format { $version }, nyere enn Skribisto kan lese (opptil { $supported }). Ingenting ble skrevet.
import-manuskript-too-many-files = Prosjektet ble ikke importert: det inneholder mer enn { $limit } filer, langt flere enn noe Manuskript-prosjekt. Ingenting ble skrevet.
import-manuskript-too-large = Prosjektet ble ikke importert: det er større enn de { $limit } MB Skribisto leser fra et Manuskript-prosjekt. Ingenting ble skrevet.
project-import-close = Lukk
project-import-location = Målmappe
project-import-location-title = Velg en målmappe
project-import-name = Filnavn
project-import-name-placeholder = Prosjektnavn
project-import-will-create = Vil opprette
project-import-cancel = Avbryt
project-import-import = Importer
project-import-location-required = Velg en målmappe
project-import-location-missing = Denne mappen finnes ikke
project-import-location-not-folder = Denne banen er ikke en mappe
project-import-location-readonly = Denne mappen er ikke skrivbar
project-import-name-required = Skriv inn et filnavn
project-import-name-exists = Det finnes allerede en fil med dette navnet her. Du blir bedt om å bekrefte overskriving ved import
project-import-name-is-folder = Det finnes allerede en mappe med dette navnet her, kanskje et prosjekt som er lagret som mappe. En import kan ikke erstatte den: velg et annet navn
project-import-overwrite-title = Vil du erstatte den eksisterende filen?
project-import-overwrite-text = «{ $name }» finnes allerede. Vil du erstatte den med det importerte prosjektet?
project-import-busy-title = Et annet prosjekt importeres
project-import-busy-text = Prosjekter importeres ett om gangen. Vent til importen som pågår, er ferdig, eller avbryt den, og start deretter denne på nytt.
project-import-progress-announcement = { $title } { $phase }
project-import-stopping = Stopper importen…
project-import-cancelled = Importen ble avbrutt
project-import-open-now = Åpne nå
project-import-details = Detaljer
project-import-report = { $count ->
    [one] 1 ting å vite om denne importen
   *[other] { $count } ting å vite om denne importen
}
project-import-report-title = Om denne importen
project-import-error-title = Kunne ikke importere prosjektet
project-import-failed-cancelled = Importen ble avbrutt. Ingenting ble skrevet.
project-import-failed-target-exists = «{ $file }» finnes allerede, og importen hadde ikke lov til å erstatte den. Ingenting ble skrevet.
project-import-failed-target-folder = «{ $file }» er en mappe, kanskje et prosjekt som er lagret som mappe, og en import kan ikke erstatte den. Velg et annet navn. Ingenting ble skrevet.
project-import-failed-not-written = «{ $file }» kunne ikke skrives i «{ $folder }» ({ $reason }). Ingenting ble skrevet.
project-import-failed-disk-full = Det er ikke nok ledig plass i «{ $folder }» til å skrive «{ $file }». Frigjør litt plass der, og importer deretter på nytt. Ingenting ble skrevet.
project-import-failed-source-missing = «{ $file }» finnes ikke lenger. Ingenting ble skrevet.
project-import-failed-unreadable = Skribisto kunne ikke lese «{ $file }» ({ $reason }). Ingenting ble skrevet.
project-import-failed-archive-damaged = «{ $file }» er en komprimert fil som ikke kunne leses, sannsynligvis fordi den er skadet ({ $reason }). Ingenting ble skrevet.
project-import-failed-archive-entries = Prosjektet ble ikke importert: arkivet oppgir { $entries ->
    [one] { $entries } fil
   *[other] { $entries } filer
}, flere enn de { $limit } et prosjekt kan inneholde, noe bare en fil laget for å bruke opp minnet gjør. Ingenting ble skrevet.
project-import-failed-archive-member = Prosjektet ble ikke importert: delen «{ $part }» pakkes ut til mer enn { $limit } MB, noe bare en fil laget for å bruke opp minnet gjør. Ingenting ble skrevet.
project-import-failed-archive-size = Prosjektet ble ikke importert: det pakkes ut til mer enn { $limit } MB, noe bare en fil laget for å bruke opp minnet gjør. Ingenting ble skrevet.
project-import-failed-nested = Prosjektet ble ikke importert: delen «{ $part }» er nestet mer enn { $limit } nivåer dypt, og å lese den kunne få Skribisto til å krasje. Ingenting ble skrevet.
project-import-failed-entities = Prosjektet ble ikke importert: delen «{ $part }» deklarerer XML-entiteter. Å utvide dem kunne kreve mer minne enn datamaskinen har, og få Skribisto til å krasje. Ingenting ble skrevet.
project-import-failed-folders = Prosjektet ble ikke importert: en av filene ligger mer enn { $limit } mapper dypt, og å lese den kunne få Skribisto til å krasje. Ingenting ble skrevet.
project-import-failed-malformed = Prosjektet ble ikke importert: delen «{ $part }» er skadet og kan ikke leses ({ $reason }). Ingenting ble skrevet.
project-import-failed-scrivener-refused = Skribisto kan ikke importere dette Scrivener-prosjektet. Ingenting ble skrevet.
project-import-failed-scrivener-question = Scrivener-prosjektet stiller et spørsmål som gjennomgangen ikke besvarte. Start importen av det på nytt, svar på spørsmålet, og importer deretter. Ingenting ble skrevet.
project-import-failed-scrivener-not-accepted = Før Scrivener-prosjektet kan importeres, må du krysse av i en boks på trinnet Prosjekt, og det ble ikke krysset av. Start importen av det på nytt, kryss av i boksen, og importer deretter. Ingenting ble skrevet.
project-import-failed-scrivener-changed = Scrivener-prosjektet ble endret etter at du gikk gjennom det. Start importen av det på nytt, gå gjennom det igjen, og importer deretter. Ingenting ble skrevet.
project-import-failed-internal = Importen stoppet på en feil Skribisto ikke ventet: { $detail }

import-target-open-title = Dette prosjektet er åpent
import-target-open-text = «{ $name }» er åpent i Skribisto. Å importere over det ville erstatte prosjektet du arbeider med, og neste lagring ville skrive det prosjektet tilbake over importen. Velg et annet filnavn, eller lukk prosjektet først.
import-warnings-announcement = { $result } { $notice }.
quit-import-running-title = En import pågår fortsatt
quit-import-running-text = Skribisto importerer fortsatt «{ $name }». Hvis du avslutter nå, stoppes importen: ingenting skrives, og prosjektet du importerer fra, blir værende som det var.
quit-import-stop = Stopp importen og avslutt

## Import a Scrivener project: the wizard
import-scrivener-title = Importer et Scrivener-prosjekt
import-scrivener-step-project = Prosjekt
import-scrivener-step-review = Gjennomgang
import-scrivener-step-save = Lagre
import-scrivener-back = Tilbake
import-scrivener-next = Neste
import-scrivener-import = Importer
import-scrivener-cancel = Avbryt
import-scrivener-help = Hjelp
import-scrivener-drop-title = Slipp et Scrivener-prosjekt her
import-scrivener-drop-hint = Et .scriv-prosjekt, .scrivx-filen inni det, en sikkerhetskopi i .zip eller en .scrivtemplate
import-scrivener-source = Scrivener-prosjekt
import-scrivener-source-placeholder = Bane til prosjektet
import-scrivener-choose-file = Velg en fil…
import-scrivener-choose-folder = Velg en mappe…
import-scrivener-source-hint = Velg prosjektets .scriv-mappe, eller åpne den og velg .scrivx-filen inni. En sikkerhetskopi i .zip fungerer også.
import-scrivener-source-hint-mac = Hvis prosjektet vises som én fil, bruker du «Velg en fil…». Hvis det vises som en mappe, bruker du «Velg en mappe…». En sikkerhetskopi i .zip fungerer også.
import-scrivener-dialog-file-title = Velg et Scrivener-prosjekt
import-scrivener-dialog-folder-title = Velg en Scrivener-prosjektmappe
import-scrivener-dialog-filter = Scrivener-prosjekt
import-scrivener-source-missing = Det finnes ingenting på denne banen
import-scrivener-source-unreadable-name = Dette navnet kan ikke leses. Gi prosjektet nytt navn i filbehandleren, og velg det deretter på nytt.
import-scrivener-empty-hint = Velg et prosjekt for å se hva Skribisto finner i det.
import-scrivener-reading = Leser prosjektet…
import-scrivener-stop-reading = Stopp lesingen
import-scrivener-read-failed = Prosjektet kunne ikke leses
import-scrivener-read-failed-body = Noe gikk galt under lesingen. Velg «Les prosjektet på nytt» for å prøve en gang til, eller «Detaljer» for å se hva som gikk galt.
import-scrivener-details = Detaljer
import-scrivener-check-again = Les prosjektet på nytt
import-scrivener-refusal-unknown-title = Dette prosjektet kan ikke importeres
import-scrivener-refusal-unknown-body = Skribisto vet ikke hvordan det skal overføres. Detaljene oppgir grunnen.
import-scrivener-made-with-mac = Sist lagret av Scrivener { $version } for macOS
import-scrivener-made-with-windows = Sist lagret av Scrivener { $version } for Windows
import-scrivener-made-with-linux = Sist lagret av Scrivener { $version } for Linux
import-scrivener-made-with-ios = Sist lagret av Scrivener { $version } for iPhone og iPad
import-scrivener-made-with-version = Sist lagret av Scrivener { $version }
import-scrivener-made-with-unknown = Scrivener-prosjekt, format { $format }
import-scrivener-saved-on = Lagret { $date } på «{ $device }»
import-scrivener-saved-on-no-device = Lagret { $date }
import-scrivener-card-counts = { $documents ->
    [one] 1 dokument
   *[other] { $documents } dokumenter
}, { $manuscript } av dem i manuset
import-scrivener-review-hint = Slik vil permen din bli overført. Ingenting skrives før du importerer, og mange elementer kan bytte type etterpå i permen, med Konverter til.
import-scrivener-choices-structure = Nivåer
import-scrivener-top-level = Mapper på øverste nivå
import-scrivener-choices-text = Tekst
import-scrivener-choice-containers = Mapper på nivå { $depth }
import-scrivener-choice-texts = Dokumenter på nivå { $depth }
import-scrivener-choice-count = { $count ->
    [one] 1 element
   *[other] { $count } elementer
}
import-scrivener-choice-count-example = { $count ->
    [1] 1 element: «{ $example }»
   *[other] { $count } elementer, for eksempel «{ $example }»
}
import-scrivener-rung-part = Deler
import-scrivener-rung-chapter = Kapitler
import-scrivener-rung-plain = Vanlige mapper
import-scrivener-texts-scenes = Scener
import-scrivener-texts-chapters = Kapitler
import-scrivener-choice-books = Bøker i manuset
import-scrivener-books-one = Én bok
import-scrivener-books-several = Én bok per mappe
import-scrivener-choice-split = { $count ->
    [1] Del opp «{ $title }» ved overskriften
   *[other] Del opp «{ $title }» ved de { $count } overskriftene
}
import-scrivener-root-items = { $count ->
    [one] 1 element
   *[other] { $count } elementer
}
import-scrivener-root-items-manuscript = { $count ->
    [one] 1 element. Denne mappen ser ut som manus.
   *[other] { $count } elementer. Denne mappen ser ut som manus.
}
import-scrivener-dest-notes = Notater
import-scrivener-dest-story-bible = Historiebibel
import-scrivener-dest-manuscript = En annen bok
import-scrivener-dest-leave-out = Utelat
import-scrivener-choice-trash = { $count ->
    [one] Ta med papirkurven (1 element)
   *[other] Ta med papirkurven ({ $count } elementer)
}
import-scrivener-trash-detail = { $count ->
    [0] Ingen av dem inneholder tekst.
    [one] 1 av dem inneholder tekst.
   *[other] { $count } av dem inneholder tekst.
}
import-scrivener-choice-blank-lines = Tomme linjer blir sceneskift
import-scrivener-choice-ornaments = Linjer med ornamenter blir sceneskift
import-scrivener-preview-label = Strukturen i det nye prosjektet
import-scrivener-col-title = Tittel
import-scrivener-col-becomes = Blir
import-scrivener-col-exports = I eksporter
import-scrivener-in-exports-yes = Ja
import-scrivener-in-exports-no = Nei
import-scrivener-becomes-item = Element
import-scrivener-becomes-trashed = I papirkurven
import-scrivener-before-title = Før du importerer
import-scrivener-review-error-unreadable = Gjennomgangen kunne ikke forberedes. Velg prosjektet på nytt.
import-scrivener-review-error-choice = Dette valget passer ikke til prosjektet. Velg prosjektet på nytt.
import-scrivener-review-error-unmappable = Med dette valget kunne ikke permen overføres uten at noe gikk tapt. Velg annerledes, eller rapporter problemet.
import-scrivener-save-hint = Importen lager et nytt Skribisto-prosjekt. Scrivener-prosjektet ditt blir bare lest, aldri endret.
import-scrivener-title-field = Tittel
import-scrivener-author = Forfatter
import-scrivener-language = Språk
import-scrivener-location-inside-project = Velg en mappe utenfor Scrivener-prosjektet. Scrivener holder bare sine egne filer der inne.
import-scrivener-copy-research-label = Kopier researchfiler ved siden av det nye prosjektet
import-scrivener-copy-research = { $count ->
    [one] 1 researchfil som Skribisto ikke kan åpne, for eksempel en PDF eller en lyd, legges i en mappe ved siden av prosjektet. Hvis boksen ikke er krysset av, sier notatet hvor den ligger i Scrivener-prosjektet ditt.
   *[other] { $count } researchfiler som Skribisto ikke kan åpne, for eksempel PDF-er eller lyder, legges i en mappe ved siden av prosjektet. Hvis boksen ikke er krysset av, sier notatene hvor de ligger i Scrivener-prosjektet ditt.
}
import-scrivener-default-name = Scrivener-prosjekt
import-scrivener-progress-title = Importerer Scrivener-prosjekt…
import-scrivener-done-title = Scrivener-prosjektet er importert
import-scrivener-done = { $items ->
    [one] Det nye prosjektet inneholder 1 element.
   *[other] Det nye prosjektet inneholder { $items } elementer.
}
import-scrivener-done-check = { $items ->
    [one] Det nye prosjektet inneholder 1 element.
   *[other] Det nye prosjektet inneholder { $items } elementer.
} { $findings ->
    [one] 1 ting bør du se på.
   *[other] { $findings } ting bør du se på.
}
import-scrivener-review-again = Se over importen igjen
import-scrivener-see-report = Se rapport
import-scrivener-report-from = Lest fra { $path }
import-scrivener-report-to = Skrevet til { $path }
import-scrivener-report-came-across = Dette ble overført
import-scrivener-report-nothing-counted = Ingenting ble telt.
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
    [one] notatene til 1 dokument
   *[other] notatene til { $count } dokumenter
}
import-scrivener-report-comments = { $count ->
    [one] 1 kommentar
   *[other] { $count } kommentarer
}
import-scrivener-report-footnotes = { $count ->
    [one] 1 fotnote
   *[other] { $count } fotnoter
}
import-scrivener-report-pictures = { $count ->
    [one] 1 bilde
   *[other] { $count } bilder
}
import-scrivener-report-templates = { $count ->
    [one] 1 notatmal
   *[other] { $count } notatmaler
}
import-scrivener-report-tags = { $count ->
    [one] 1 stikkord
   *[other] { $count } stikkord
}
import-scrivener-report-statuses = { $count ->
    [one] 1 status
   *[other] { $count } statuser
}
import-scrivener-report-trashed = { $count ->
    [one] 1 element i papirkurven
   *[other] { $count } elementer i papirkurven
}
import-scrivener-report-research-files = { $count ->
    [one] 1 researchfil
   *[other] { $count } researchfiler
}
import-scrivener-report-choices = Dette valgte du
import-scrivener-report-accepted-mobile = Du importerte prosjektet uten endringene som ble gjort i Scrivener for iPhone eller iPad, og som ennå ikke var kommet inn i det.
import-scrivener-report-accepted-no-text = Du importerte denne kopien med tomme dokumenter, fordi ingenting av teksten deres fantes i den.
import-scrivener-report-choice = { $subject }: { $answer }
import-scrivener-report-yes = ja
import-scrivener-report-no = nei
import-scrivener-report-research-copied = Researchfiler som Skribisto ikke kan åpne, ble kopiert til «{ $folder }», ved siden av det nye prosjektet.
import-scrivener-report-research-not-copied = Researchfiler som Skribisto ikke kan åpne, ble ikke kopiert. Notatene deres sier hvor de ligger i Scrivener-prosjektet ditt.
import-scrivener-report-research-copied-except = { $count ->
    [one] Researchfiler som Skribisto ikke kan åpne, ble kopiert til «{ $folder }», ved siden av det nye prosjektet, bortsett fra 1 som blir værende i Scrivener-prosjektet ditt. Notatet sier hvor den ligger.
   *[other] Researchfiler som Skribisto ikke kan åpne, ble kopiert til «{ $folder }», ved siden av det nye prosjektet, bortsett fra { $count } som blir værende i Scrivener-prosjektet ditt. Notatene deres sier hvor de ligger.
}
import-scrivener-report-research-none-copied = { $count ->
    [1] Du ba om at researchfiler skulle kopieres, men den eneste filen kunne ikke kopieres. Den blir værende i Scrivener-prosjektet ditt, og notatet sier hvor den ligger.
   *[other] Du ba om at researchfiler skulle kopieres, men ingen av de { $count } kunne kopieres. De blir værende i Scrivener-prosjektet ditt, og notatene deres sier hvor de ligger.
}
import-scrivener-report-chose-project = Du valgte å importere «{ $project }», ett av flere prosjekter i mappen eller arkivet du valgte ut.
import-scrivener-report-chose-manifest = Du valgte permfilen «{ $file }».
import-scrivener-report-chose-enclosing = Du valgte noe inne i «{ $project }», så hele prosjektet ble importert.
import-scrivener-report-chose-upgraded = Du importerte «{ $current }», prosjektet du fortsatte å skrive i, og ikke kopien Scrivener beholdt før en oppdatering.
import-scrivener-report-chose-backup = Du importerte «{ $backup }», den eldre kopien Scrivener beholdt før en oppdatering.
import-scrivener-report-title-placeholder-rejected = Tittelen i prosjektets innstillinger var en plassholder fra en mal, så Skribisto foreslo prosjektets navn i stedet.
import-scrivener-report-author-placeholder-rejected = Forfatternavnet i prosjektets innstillinger var en plassholder fra en mal, så Skribisto foreslo ikke noe.
import-scrivener-report-title-from-compile-settings = Skribisto foreslo tittelen som står i prosjektets kompileringsinnstillinger: navnet prosjektet hadde da det ble opprettet, med mindre en tittel ble satt der i Scrivener.
import-scrivener-report-title-from-template = Skribisto foreslo malens egen tittel.
import-scrivener-report-title-from-folder-name = Skribisto foreslo prosjektets navn som tittel.
import-scrivener-report-title-from-project-properties = Skribisto foreslo tittelen som står i prosjektets egenskaper.
import-scrivener-report-author-from-project = Skribisto foreslo forfatternavnet som står i prosjektets innstillinger. En mals forfatter kan bli liggende igjen der, og den ville bli skrevet ut på en tittelside.
import-scrivener-report-language-not-stated = Prosjektet oppga ikke språket sitt, så Skribisto foreslo språket programmet selv vises på. Det bestemmer hvordan ord telles og hvordan anførselstegn skrives.
import-scrivener-report-language-from-script = Skribisto foreslo språket ut fra skriftsystemet manuset er skrevet i ({ $tag }).
import-scrivener-report-you-changed-it = { $hint } Du erstattet dette forslaget.
import-scrivener-report-content-salvaged-text = Teksten i «{ $item }» manglet i prosjektet. Ordene kommer fra Scriveners søkeindeks, som ren tekst og kanskje fra en eldre versjon: sjekk dem.
import-scrivener-report-content-salvaged-notes = Notatene til «{ $item }» manglet i prosjektet. Ordene kommer fra Scriveners søkeindeks, som ren tekst og kanskje fra en eldre versjon: sjekk dem.
import-scrivener-report-content-salvaged-synopsis = Synopsisen til «{ $item }» manglet i prosjektet. Ordene kommer fra Scriveners søkeindeks, som ren tekst og kanskje fra en eldre versjon: sjekk dem.
import-scrivener-report-document-others = { $others ->
    [one] { $sentence } Det samme skjer på 1 sted til, oppført nedenfor.
   *[other] { $sentence } Det samme skjer på { $others } steder til, oppført nedenfor.
}
import-scrivener-report-item-notes = { $path } (dokumentnotater)
import-scrivener-report-item-synopsis = { $path } (synopsis)
import-scrivener-report-item-project-note = { $path } (prosjektnotat)
import-scrivener-report-item-recovered = { $path } (reddet tekst)
import-scrivener-report-untitled = Uten tittel
import-scrivener-report-no-findings = Importen har ikke noe mer å rapportere.
import-scrivener-report-group-unread = Kunne ikke leses
import-scrivener-report-group-check = Sjekk disse
import-scrivener-report-group-changed = Overført på en annen måte
import-scrivener-report-group-left = Ikke overført
import-scrivener-report-group-count = { $group } ({ $count })
import-scrivener-report-items-toggle = { $count ->
    [one] 1 element
   *[other] { $count } elementer
}
import-scrivener-report-items-toggle-named = { $count ->
    [one] 1 element: { $line }
   *[other] { $count } elementer: { $line }
}
import-scrivener-report-names-toggle = { $count ->
    [one] 1 navn
   *[other] { $count } navn
}
import-scrivener-report-names-toggle-named = { $count ->
    [one] 1 navn: { $line }
   *[other] { $count } navn: { $line }
}
import-scrivener-report-more = { $count ->
    [one] Og 1 til.
   *[other] Og { $count } til.
}
import-scrivener-report-open-item = Åpne i redigeringsruten
import-scrivener-report-links-hint = Åpne det nye prosjektet, og vis deretter denne rapporten igjen fra kommandopaletten: da åpnes hvert element som står her, med et klikk.
import-scrivener-copy-report = Kopier rapport
import-scrivener-report-copied = Rapporten er kopiert
import-scrivener-report-copy-failed = Rapporten kunne ikke kopieres.
import-scrivener-report-close = Lukk
import-scrivener-report-none-title = Ingen importrapport
import-scrivener-report-none = Ingen Scrivener-import er fullført siden Skribisto startet. Skribisto tar vare på rapportene fra de siste importene til programmet lukkes.
import-scrivener-report-gone = { $count ->
    [one] Rapporten fra denne importen er ikke lenger tatt vare på. Skribisto tar vare på rapporten fra den siste importen til programmet lukkes.
   *[other] Rapporten fra denne importen er ikke lenger tatt vare på. Skribisto tar vare på rapportene fra de { $count } siste importene til programmet lukkes.
}

## Words the import writes into the new project, in the language Skribisto shows itself in
import-scrivener-manuscript-binder = Manus
import-scrivener-notes-binder = Notater
import-scrivener-notes-of = { $title } (notater)
import-scrivener-project-notes = Prosjektnotater
import-scrivener-recovered = Reddede tekster
import-scrivener-untitled-template = Mal uten tittel
import-scrivener-research-folder = Researchfiler for { $name }
import-scrivener-list-separator = {", "}
import-scrivener-label-details = Fra feltet «{ $field }» i Scrivener
import-scrivener-status-details = Fra feltet «{ $field }» i Scrivener
import-scrivener-unnamed-label = Etikett { $number }
import-scrivener-unnamed-status = Status { $number }
import-scrivener-label-tag = { $name } (etikett)
import-scrivener-untitled-tag = Uten tittel
import-scrivener-keyword-details = Nøkkelordet «{ $path }» i Scrivener
import-scrivener-unnamed-keyword = Nøkkelord { $number }
import-scrivener-keyword-tag = { $name } (nøkkelord)
import-scrivener-collection-details = En samling i Scrivener
import-scrivener-collection-tag = { $name } (samling)
import-scrivener-field-details = Fra feltet «{ $field }» i Scrivener
import-scrivener-field-value = { $field }: { $option }
import-scrivener-field-tag = { $name } (felt)
import-scrivener-deadline-day = Frist
import-scrivener-from-scrivener = Fra Scrivener
import-scrivener-note-project = Prosjekt
import-scrivener-note-project-bookmarks = Prosjektbokmerker
import-scrivener-note-replacements = Erstatninger ved kompilering
import-scrivener-note-auto-complete = Autofullføringsoppføringer
import-scrivener-note-saved-searches = Lagrede søk
import-scrivener-note-custom-metadata = Egendefinerte metadata
import-scrivener-note-targets = Mål
import-scrivener-note-bookmarks = Bokmerker
import-scrivener-abbreviated-title = Forkortet tittel
import-scrivener-subjects = Emner
import-scrivener-company = Firma
import-scrivener-session-target = Øktmål
import-scrivener-session-target-calculated = Øktmål, beregnet av Scrivener ut fra fristen
import-scrivener-writing-days = Skrivedager
import-scrivener-weekday-monday = Mandag
import-scrivener-weekday-tuesday = Tirsdag
import-scrivener-weekday-wednesday = Onsdag
import-scrivener-weekday-thursday = Torsdag
import-scrivener-weekday-friday = Fredag
import-scrivener-weekday-saturday = Lørdag
import-scrivener-weekday-sunday = Søndag
import-scrivener-draft-target = Utkastmål
import-scrivener-deadline = Frist
import-scrivener-unit-words = ord
import-scrivener-unit-characters = tegn
import-scrivener-unit-pages = sider
import-scrivener-search-query = Søkte etter
import-scrivener-search-in = Søkte i
import-scrivener-search-operator = Samsvar
import-scrivener-search-in-all = alt
import-scrivener-search-in-title = titler
import-scrivener-search-in-text = tekst
import-scrivener-search-in-notes = notater
import-scrivener-search-in-synopsis = synopser
import-scrivener-search-in-keywords = nøkkelord
import-scrivener-search-in-label = etiketter
import-scrivener-search-in-status = statuser
import-scrivener-search-in-section-type = seksjonstyper
import-scrivener-search-in-custom-metadata = egendefinerte metadata
import-scrivener-search-any = hvilket som helst ord
import-scrivener-search-all = alle ordene
import-scrivener-search-exact = eksakt frase
import-scrivener-label-value = { $label }: { $value }
import-scrivener-replacement-regex = regulært uttrykk
import-scrivener-replacement-off = slått av
import-scrivener-in-own-note = Oppført i notatet dette dokumentet ble til.

## Import a Scrivener project: what the import says about the project, in the writer's words.
import-scrivener-paragraph = { $title }. { $body }
import-scrivener-refusal-missing-title = Det finnes ingenting på denne banen
import-scrivener-refusal-missing-body = «{ $file }» finnes ikke der lenger. Velg prosjektet på nytt.
import-scrivener-refusal-unreadable-title = Dette prosjektet kan ikke leses
import-scrivener-refusal-unreadable-body = Skribisto kunne ikke lese «{ $file }» ({ $reason }). Sjekk at du kan åpne det, og prøv deretter igjen.
import-scrivener-refusal-not-a-scrivener-project-title = Ikke noe Scrivener-prosjekt her
import-scrivener-refusal-not-a-scrivener-project-body = Velg prosjektets .scriv-mappe, .scrivx-filen inni den, en sikkerhetskopi i .zip eller en .scrivtemplate.
import-scrivener-refusal-manifest-without-files-title = Dokumentene finnes ikke i denne kopien
import-scrivener-refusal-manifest-without-files-body = «{ $manifest }» er her, men ikke Files-mappen som inneholder dokumentene den oppgir. En synkroniseringstjeneste er kanskje ikke ferdig med å kopiere prosjektet, eller bare .scrivx-filen ble kopiert. Hvis du valgte .scrivx-filen, velger du heller prosjektets mappe med «Velg en mappe…».
import-scrivener-refusal-search-too-wide-title = Denne mappen inneholder for mye til å søke gjennom
import-scrivener-refusal-search-too-wide-body = Den har mer enn { $limit } mapper å se gjennom. Velg selve Scrivener-prosjektet.
import-scrivener-refusal-scrivener-one-for-mac-title = Dette prosjektet ble laget med Scrivener 1 for Mac
import-scrivener-refusal-scrivener-one-for-mac-body = Skribisto leser prosjekter fra Scrivener 2 og nyere. Åpne det én gang i Scrivener 2 eller 3 for å oppdatere det, eller kompiler det til et Word-dokument og importer det.
import-scrivener-refusal-ios-stub-without-mobile-data-title = Dette er bare en plassholder fra Scrivener for iPhone eller iPad
import-scrivener-refusal-ios-stub-without-mobile-data-body = Selve prosjektet er ikke her. Synkroniser det fra iPhone eller iPad, eller del det derfra som en .zip, og importer den.
import-scrivener-refusal-newer-title = Dette prosjektet ble laget med en nyere Scrivener
import-scrivener-refusal-unknown-format-version-body = Formatet, { $version }, er ikke et format Skribisto kjenner. Se etter en oppdatering av Skribisto. Inntil da kan du kompilere det til et Word-dokument i Scrivener og importere det.
import-scrivener-refusal-newer-format-stamp-body = Filene er merket som versjon { $stamp }, og Skribisto leser opptil versjon { $newest }. Se etter en oppdatering av Skribisto. Inntil da kan du kompilere det til et Word-dokument i Scrivener og importere det.
import-scrivener-refusal-version-stamp-unreadable-title = Prosjektets versjon kan ikke leses
import-scrivener-refusal-version-stamp-unreadable-body = Filen Files/version.txt inneholder ikke noe versjonsnummer, så Skribisto kan ikke avgjøre om det kan lese dette prosjektet. Åpne prosjektet én gang i Scrivener, og importer det deretter på nytt.
import-scrivener-refusal-damaged-manifest-title = Permfilen kan ikke leses
import-scrivener-refusal-damaged-manifest-body = «{ $manifest }» er skadet. { $spare_copies ->
    [0] Åpne prosjektet i Scrivener, som kan reparere det, og importer det deretter på nytt.
    [1] Scrivener har en reservekopi av den inne i prosjektet, som Skribisto ikke leser. Åpne prosjektet i Scrivener, som kan reparere det, og importer det deretter på nytt.
   *[other] Scrivener har { $spare_copies } reservekopier av den inne i prosjektet, som Skribisto ikke leser. Åpne prosjektet i Scrivener, som kan reparere det, og importer det deretter på nytt.
}
import-scrivener-refusal-damaged-manifest-new-project-placeholder-body = «{ $manifest }» er den tomme filen Scrivener skriver for et prosjekt som ikke er lagret ennå. Åpne prosjektet i Scrivener og la det lagre, og importer det deretter på nytt.
import-scrivener-refusal-manifest-too-large-title = Denne permfilen er for stor
import-scrivener-refusal-manifest-too-large-body = «{ $manifest }» er større enn de { $limit } MB Skribisto leser. Ingen virkelige prosjekter kommer i nærheten av det, så filen kan være skadet.
import-scrivener-refusal-no-item-identifiers-title = Permen oppgir ingen dokumenter Skribisto kan finne
import-scrivener-refusal-no-item-identifiers-body = Ingen av elementene har en identifikator, så filene deres kan ikke finnes. Åpne prosjektet én gang i Scrivener, og importer det deretter på nytt.
import-scrivener-refusal-incomplete-copy-title = Dokumentene mangler i denne kopien
import-scrivener-refusal-incomplete-copy-body = Files-mappen er her, men dokumentene i den er ikke det. En synkroniseringstjeneste er kanskje ikke ferdig med å kopiere prosjektet. Vent til den er ferdig, eller kopier hele .scriv-mappen igjen, og les deretter prosjektet på nytt.
import-scrivener-refusal-archive-title = Dette arkivet kan ikke åpnes
import-scrivener-refusal-archive-unreadable-body = «{ $file }» kunne ikke leses ({ $reason }). Det kan være skadet. Pakk det ut selv, og velg deretter .scriv-mappen inni det.
import-scrivener-refusal-archive-unsafe-name-title = Dette arkivet kan ikke åpnes på en trygg måte
import-scrivener-refusal-archive-unsafe-name-body = «{ $file }» inneholder en fil kalt «{ $member }», som ville havnet utenfor arkivet, så Skribisto åpner det ikke.
import-scrivener-refusal-archive-unsafe-name-empty-body = «{ $file }» inneholder en fil uten navn, så Skribisto åpner det ikke.
import-scrivener-refusal-archive-unsafe-name-control-character-body = «{ $file }» inneholder en fil med et kontrolltegn i navnet, så Skribisto åpner det ikke.
import-scrivener-refusal-archive-too-large-title = Dette arkivet er for stort til å åpnes på en trygg måte
import-scrivener-refusal-archive-too-many-members-body = Det inneholder mer enn { $limit } filer. Pakk det ut selv, og velg deretter .scriv-mappen inni det.
import-scrivener-refusal-archive-member-too-large-body = «{ $member }» pakkes ut til mer enn { $limit } MB. Pakk ut arkivet selv, og velg deretter .scriv-mappen inni det.
import-scrivener-refusal-archive-too-large-body = Det pakkes ut til mer enn { $limit } MB. Pakk det ut selv, og velg deretter .scriv-mappen inni det.
import-scrivener-refusal-archive-encrypted-title = Dette arkivet er beskyttet med passord
import-scrivener-refusal-archive-encrypted-body = Pakk ut «{ $file }» selv, og velg deretter .scriv-mappen inni det.
import-scrivener-refusal-archive-unsupported-compression-title = Dette arkivet er pakket på en måte Skribisto ikke kan lese
import-scrivener-refusal-archive-unsupported-compression-body = «{ $member }» er komprimert med { $method }. Pakk ut arkivet selv, og velg deretter .scriv-mappen inni det.
import-scrivener-refusal-duplicate-archive-members-title = Dette arkivet oppgir noen filer to ganger
import-scrivener-refusal-duplicate-archive-members-body = «{ $file }» inneholder mer enn én fil med samme navn, så Skribisto kan ikke avgjøre hvilken som hører til prosjektet. Pakk det ut selv, og velg deretter .scriv-mappen inni det.
import-scrivener-refusal-template-undecodable-title = Denne malfilen er ufullstendig
import-scrivener-refusal-template-undecodable-too-large-title = Denne malfilen er for stor til å åpnes
import-scrivener-refusal-template-undecodable-not-a-template-title = Denne filen er ikke en Scrivener-mal
import-scrivener-refusal-template-undecodable-unknown-encoding-title = Denne malfilen er lagret i en form Skribisto ikke kjenner
import-scrivener-refusal-template-undecodable-body = «{ $file }» kan ikke dekodes: den slutter kanskje før prosjektet inni den gjør det. Last den ned på nytt, eller åpne den i Scrivener og lag et prosjekt fra den.
import-scrivener-refusal-template-undecodable-too-large-body = «{ $file }» er større enn { $limit } MB. Åpne den i Scrivener, lag et prosjekt fra den, og importer deretter det prosjektet.
import-scrivener-refusal-template-undecodable-not-a-template-body = «{ $file }» inneholder ikke noe Scrivener-prosjekt. Velg malfilen Scrivener lagret, eller et prosjekt laget fra den.
import-scrivener-refusal-template-undecodable-unknown-encoding-body = «{ $file }» lagrer prosjektet sitt på en måte Skribisto ikke kan dekode. Åpne den i Scrivener, lag et prosjekt fra den, og importer deretter det prosjektet.
import-scrivener-refusal-cloud-placeholders-title = { $count ->
    [one] 1 fil i dette prosjektet ligger fortsatt i skyen
   *[other] { $count } filer i dette prosjektet ligger fortsatt i skyen
}
import-scrivener-refusal-cloud-placeholders-body = { $count ->
    [1] Den er ikke lastet ned til denne datamaskinen: { $examples }. Gjør prosjektet tilgjengelig uten nett i synkroniseringsappen, og les deretter prosjektet på nytt.
   *[other] De er ikke lastet ned til denne datamaskinen, for eksempel { $examples }. Gjør prosjektet tilgjengelig uten nett i synkroniseringsappen, og les deretter prosjektet på nytt.
}
import-scrivener-refusal-unreadable-members-title = { $count ->
    [one] 1 fil i dette prosjektet kan ikke leses
   *[other] { $count } filer i dette prosjektet kan ikke leses
}
import-scrivener-refusal-unreadable-members-body = For eksempel { $examples }. Sjekk at du har lov til å lese hele prosjektet, og at disken er i orden, og les deretter prosjektet på nytt.
import-scrivener-refusal-needed-member-is-link-title = Dette prosjektet peker ut av sin egen mappe
import-scrivener-refusal-needed-member-is-link-body = «{ $member }» er en lenke til et annet sted på denne datamaskinen, og Skribisto følger ikke lenker. Kopier prosjektet slik at alle filene ligger inni det, og prøv deretter igjen.
import-scrivener-refusal-project-changed-during-read-title = Prosjektet ble endret mens det ble lest
import-scrivener-refusal-project-changed-during-read-body = Scrivener eller en synkroniseringstjeneste skriver fortsatt til det. Lukk det i Scrivener, vent til synkroniseringen er ferdig, og les deretter prosjektet på nytt.
import-scrivener-refusal-too-many-members-title = Dette prosjektet inneholder for mange filer
import-scrivener-refusal-too-many-members-body = Det har mer enn { $limit } filer, flere enn noe virkelig prosjekt. Det er kanskje ikke et Scrivener-prosjekt, eller det kan være skadet.
import-scrivener-refusal-too-large-title = Dette prosjektet er for stort
import-scrivener-refusal-too-large-body = Teksten i det utgjør mer enn { $limit } MB, mer enn Skribisto kan importere på en trygg måte.
import-scrivener-refusal-structure-unmappable-title = Skribisto kan ikke overføre denne permen
import-scrivener-refusal-structure-unmappable-body = Permen kunne ikke overføres uten at noe gikk tapt eller havnet på feil sted. Dette er en feil i Skribisto, ikke i prosjektet ditt. Rapporter den, sammen med det «Detaljer» viser.
import-scrivener-roots-title = { $count ->
    [one] { $count } prosjekt er her. Hvilket?
   *[other] { $count } prosjekter er her. Hvilket?
}
import-scrivener-roots-hint = Denne mappen eller dette arkivet inneholder mer enn ett Scrivener-prosjekt. Velg det du vil importere.
import-scrivener-manifests-title = { $count ->
    [one] Dette prosjektet har { $count } permfil
   *[other] Dette prosjektet har { $count } permfiler
}
import-scrivener-manifests-hint = Scrivener lagrer permen, listen over alle dokumenter og mapper, i en .scrivx-fil. De ekstra ble mest sannsynlig lagt igjen av en synkroniseringstjeneste. Velg den du sist arbeidet i.
import-scrivener-manifests-hint-preselected = Scrivener lagrer permen, listen over alle dokumenter og mapper, i en .scrivx-fil. De ekstra ble mest sannsynlig lagt igjen av en synkroniseringstjeneste. Den som er valgt, er den du mest sannsynlig arbeidet i sist: sjekk den før du går videre.
import-scrivener-manifest-row = { $file } · lagret { $date } · { $documents ->
    [one] 1 element
   *[other] { $documents } elementer
}
import-scrivener-manifest-row-undated = { $file } · { $documents ->
    [one] 1 element
   *[other] { $documents } elementer
}
import-scrivener-manifest-row-unread = { $file } · kan ikke leses
import-scrivener-enclosing-title = Dette er en del av Scrivener-prosjektet «{ $project }»
import-scrivener-enclosing-body = Du valgte en fil eller mappe inne i et prosjekt. Skribisto importerer hele prosjektet.
import-scrivener-enclosing-use = Importer «{ $project }»
import-scrivener-upgrade-title = Dette er kopien Scrivener beholdt før en oppdatering
import-scrivener-upgrade-body = Når Scrivener oppdaterer et prosjekt til en nyere versjon, tar det vare på det gamle som «{ $backup }». Prosjektet du fortsatte å skrive i, er «{ $current }», ved siden av det.
import-scrivener-upgrade-body-unclear = Når Scrivener oppdaterer et prosjekt til en nyere versjon, tar det vare på det gamle som «{ $backup }». Prosjektet ved siden av, «{ $current }», kunne ikke leses skikkelig, så velg hvilket du vil importere.
import-scrivener-upgrade-use-current = Importer «{ $current }», prosjektet du fortsatte å skrive i
import-scrivener-upgrade-keep-backup = Importer denne eldre kopien, «{ $backup }»
import-scrivener-ios-pending-title = Endringer gjort i Scrivener for iPhone eller iPad er ikke i denne kopien ennå
import-scrivener-ios-pending-body = Scrivener på en datamaskin henter inn disse endringene neste gang det åpner prosjektet. Åpne prosjektet én gang i Scrivener på datamaskinen, og importer det deretter på nytt.
import-scrivener-ios-pending-accept = Importer det uten disse endringene
import-scrivener-no-text-title = { $count ->
    [one] 1 dokument, og ikke et eneste ord tekst i denne kopien
   *[other] { $count } dokumenter, og ikke et eneste ord tekst i denne kopien
}
import-scrivener-no-text-body = En synkroniseringstjeneste kopierer kanskje fortsatt prosjektet, eller kopien har mistet dokumentene sine. Importerer du nå, kommer alle dokumentene inn tomme.
import-scrivener-no-text-accept = Importer dokumentene tomme
import-scrivener-notice-project-found-below-pick-title = Prosjektet ble funnet inne i mappen du valgte
import-scrivener-notice-project-found-below-pick-body = Det ligger i «{ $path }».
import-scrivener-notice-source-is-template-title = Dette er en Scrivener-mal
import-scrivener-notice-source-is-template-named = Dette er Scrivener-malen «{ $title }»
import-scrivener-notice-source-is-template-body = Når du importerer den, lages et nytt prosjekt av det malen inneholder, instruksjonene medregnet.
import-scrivener-notice-source-is-backup-title = Dette ser ut som en sikkerhetskopi Scrivener har laget
import-scrivener-notice-source-is-backup-body = Den inneholder prosjektet slik det var da sikkerhetskopien ble laget. Hvis du fortsatte å skrive etter det, velger du selve prosjektet.
import-scrivener-notice-importing-pre-upgrade-copy-title = Du importerer kopien Scrivener beholdt før en oppdatering
import-scrivener-notice-importing-pre-upgrade-copy-body = Prosjektet du fortsatte å skrive i, ligger ved siden av, og inneholder alt som er skrevet siden.
import-scrivener-notice-imported-from-ios-stub-title = Dette prosjektet ble laget i Scrivener for iPhone eller iPad
import-scrivener-notice-imported-from-ios-stub-body = Det har aldri vært åpnet på en datamaskin, så det leses fra kopien som iPhone eller iPad har.
import-scrivener-notice-stale-mobile-binder-ignored-title = En gammel perm fra en iPhone eller iPad legges til side
import-scrivener-notice-stale-mobile-binder-ignored-body = Scrivener på en datamaskin har allerede hentet inn endringene fra den.
import-scrivener-notice-identical-manifest-copies-title = { $count ->
    [one] Dette prosjektet har { $count } identisk permfil
   *[other] Dette prosjektet har { $count } identiske permfiler
}
import-scrivener-notice-identical-manifest-copies-body = De inneholder det samme, så hvilken som helst av dem gir det samme prosjektet.
import-scrivener-notice-manifest-copies-ignored-title = Ekstra kopier av permfilen legges til side
import-scrivener-notice-manifest-copies-ignored-body = «{ $chosen }» er den Scrivener sist arbeidet i. { $count ->
    [one] 1 kopi som en synkroniseringstjeneste la igjen, legges til side.
   *[other] { $count } kopier som en synkroniseringstjeneste la igjen, legges til side.
}
import-scrivener-notice-pinned-manifest-looks-like-a-copy-title = Permfilen du valgte, ser ut som en synkroniseringskopi
import-scrivener-notice-pinned-manifest-looks-like-a-copy-body = «{ $name }» er navngitt som en kopi av «{ $original }», som ligger ved siden av. Hvis importen ser gammel ut, velger du «{ $original }» i stedet.
import-scrivener-notice-pinned-manifest-has-siblings-title = { $count ->
    [one] 1 annen permfil ligger ved siden av den du valgte
   *[other] { $count } andre permfiler ligger ved siden av den du valgte
}
import-scrivener-notice-pinned-manifest-has-siblings-body = Bare den du valgte, blir lest.
import-scrivener-notice-legacy-v1-files-beside-title = Filer fra en eldre Scrivener legges til side
import-scrivener-notice-legacy-v1-files-beside-body = De ble liggende igjen da prosjektet ble oppdatert. Bare filene for gjeldende versjon blir lest.
import-scrivener-notice-mixed-payload-trees-title = Filer fra to Scrivener-versjoner finnes i dette prosjektet
import-scrivener-notice-mixed-payload-trees-body = Dokumentene leses der permfilen sier at de ligger.
import-scrivener-notice-version-disagreement-title = Filene i dette prosjektet er ikke enige om versjonen
import-scrivener-notice-id-scheme-mismatch-body = Permfilen sier versjon { $version }, men navngir dokumentene sine slik en annen versjon gjør. De leses der den peker. Hvis importen ser feil ut, åpner du prosjektet én gang i Scrivener og importerer det på nytt.
import-scrivener-notice-version-stamp-mismatch-body = Det leses slik permfilen beskriver det. Hvis importen ser feil ut, åpner du prosjektet én gang i Scrivener og importerer det på nytt.
import-scrivener-notice-unrecognised-writer-title = Dette prosjektet ble lagret av et program Skribisto ikke kjenner igjen
import-scrivener-notice-unrecognised-writer-named = Det kaller seg «{ $creator }». Det leses som et Scrivener-prosjekt.
import-scrivener-notice-unrecognised-writer-body = Det oppgir ikke noe navn. Det leses som et Scrivener-prosjekt.
import-scrivener-notice-invalid-item-identifier-title = { $count ->
    [one] 1 element i permen har en identifikator som ikke er gyldig
   *[other] { $count } elementer i permen har en identifikator som ikke er gyldig
}
import-scrivener-notice-invalid-item-identifier-body = En slik identifikator legges til side, og elementet som har den, beholdes.
import-scrivener-notice-items-without-identifier-title = { $count ->
    [one] 1 element i permen har ingen identifikator
   *[other] { $count } elementer i permen har ingen identifikator
}
import-scrivener-notice-items-without-identifier-body = Uten en identifikator kan ikke filene til et element finnes, så elementet kommer inn tomt.
import-scrivener-notice-duplicate-item-key-title = To elementer i permen deler én identifikator: «{ $item }»
import-scrivener-notice-duplicate-item-key-body = Det første beholder dokumentene. Det andre beholder plassen sin og kommer inn tomt.
import-scrivener-notice-lock-here-title = Scrivener har dette prosjektet åpent på denne datamaskinen
import-scrivener-notice-lock-here-body = Lukk det i Scrivener først, slik at alle endringer blir lagret.
import-scrivener-notice-lock-title = Scrivener hadde dette prosjektet åpent på «{ $host }»
import-scrivener-notice-lock-elsewhere-title = Scrivener hadde dette prosjektet åpent på en annen datamaskin
import-scrivener-notice-lock-body = Hvis det fortsatt er åpent der, lukker du det først: filene inneholder kanskje ikke de siste endringene ennå. Hvis Scrivener ble lukket for lenge siden, kan du importere det som det er.
import-scrivener-notice-archived-while-open-title = Dette arkivet ble laget mens Scrivener hadde prosjektet åpent
import-scrivener-notice-archived-while-open-body = Endringer gjort etter det tidspunktet kan mangle i det.
import-scrivener-notice-content-missing-notes = Notatene til «{ $item }» mangler
import-scrivener-notice-content-missing-synopsis = Synopsisen til «{ $item }» mangler
import-scrivener-notice-content-missing-text = Teksten i «{ $item }» mangler
import-scrivener-notice-content-missing-body = { $place ->
    [notes] Filen til notatene finnes ikke i denne kopien, så de kommer inn tomme. En synkroniseringstjeneste kopierer kanskje fortsatt prosjektet.
    [synopsis] Filen til synopsisen finnes ikke i denne kopien, så den kommer inn tom. En synkroniseringstjeneste kopierer kanskje fortsatt prosjektet.
   *[text] Filen til teksten finnes ikke i denne kopien, så den kommer inn tom. En synkroniseringstjeneste kopierer kanskje fortsatt prosjektet.
}
import-scrivener-notice-content-missing-salvaged = { $place ->
    [notes] Filen til notatene finnes ikke i denne kopien, men Scriveners søkeindeks hadde fortsatt ordene i dem. Ordene beholdes som ren tekst, kanskje fra en eldre versjon.
    [synopsis] Filen til synopsisen finnes ikke i denne kopien, men Scriveners søkeindeks hadde fortsatt ordene i den. Ordene beholdes som ren tekst, kanskje fra en eldre versjon.
   *[text] Filen til teksten finnes ikke i denne kopien, men Scriveners søkeindeks hadde fortsatt ordene i den. Ordene beholdes som ren tekst, kanskje fra en eldre versjon.
}
import-scrivener-notice-content-truncated-notes = Notatene til «{ $item }» er tomme på disken
import-scrivener-notice-content-truncated-synopsis = Synopsisen til «{ $item }» er tom på disken
import-scrivener-notice-content-truncated-text = Teksten i «{ $item }» er tom på disken
import-scrivener-notice-content-truncated-body = { $place ->
    [notes] Scrivener registrerte innhold i notatene, men filen til dem inneholder ingenting: en synkronisering kan ha stoppet halvveis. De kommer inn tomme.
    [synopsis] Scrivener registrerte innhold i synopsisen, men filen til den inneholder ingenting: en synkronisering kan ha stoppet halvveis. Den kommer inn tom.
   *[text] Scrivener registrerte innhold i teksten, men filen til den inneholder ingenting: en synkronisering kan ha stoppet halvveis. Den kommer inn tom.
}
import-scrivener-notice-missing-media-title = Filen til «{ $item }» mangler
import-scrivener-notice-missing-media-body = Bildet eller researchfilen som elementet inneholder, finnes ikke i denne kopien.
import-scrivener-notice-sync-conflict-copies-title = { $count ->
    [one] 1 synkroniseringskopi av et dokument ble funnet
   *[other] { $count } synkroniseringskopier av dokumenter ble funnet
}
import-scrivener-notice-sync-conflict-copies-body = { $missing ->
    [0] Synkroniseringskopier overføres som reddede tekster, i en egen mappe, så du kan sammenligne dem med dokumentene.
    [one] Synkroniseringskopier overføres som reddede tekster, i en egen mappe. { $missing } av dem er en kopi av et dokument som ikke lenger finnes i prosjektet.
   *[other] Synkroniseringskopier overføres som reddede tekster, i en egen mappe. { $missing } av dem er kopier av dokumenter som ikke lenger finnes i prosjektet.
}
import-scrivener-notice-recovered-unreferenced-text-title = { $count ->
    [one] 1 dokument som ikke hører til noe element i permen, ble funnet
   *[other] { $count } dokumenter som ikke hører til noe element i permen, ble funnet
}
import-scrivener-notice-recovered-unreferenced-text-body = Dokumenter som disse overføres som reddede tekster, i en egen mappe.
import-scrivener-notice-sync-debris-ignored-title = { $count ->
    [one] 1 ekstra kopi som en synkroniseringstjeneste la igjen, legges til side
   *[other] { $count } ekstra kopier som en synkroniseringstjeneste la igjen, legges til side
}
import-scrivener-notice-sync-debris-ignored-body = Scrivener leser heller ikke slike kopier. Bare prosjektets egne filer blir lest.
import-scrivener-notice-unrecognised-files-title = { $count ->
    [one] 1 fil som ikke hører til et Scrivener-prosjekt, legges til side
   *[other] { $count } filer som ikke hører til et Scrivener-prosjekt, legges til side
}
import-scrivener-notice-examples = For eksempel { $examples }.
import-scrivener-notice-symlink-not-followed-title = { $count ->
    [one] 1 lenke inne i prosjektet blir ikke fulgt
   *[other] { $count } lenker inne i prosjektet blir ikke fulgt
}
import-scrivener-notice-unreadable-files-skipped-title = { $count ->
    [one] 1 fil som importen ikke trenger, kunne ikke leses
   *[other] { $count } filer som importen ikke trenger, kunne ikke leses
}
import-scrivener-notice-cloud-placeholders-skipped-title = { $count ->
    [one] 1 fil som fortsatt ligger i skyen, hoppes over
   *[other] { $count } filer som fortsatt ligger i skyen, hoppes over
}
import-scrivener-notice-cloud-placeholders-skipped-body = Ingenting av det importen trenger, ligger fortsatt i skyen.
import-scrivener-notice-no-text-found-title = { $count ->
    [1] 1 dokument, og det inneholder ingen tekst
   *[other] { $count } dokumenter, og ingen av dem inneholder tekst
}
import-scrivener-notice-no-text-found-body = Dette prosjektet ser tomt ut. Det importeres som det er.
import-scrivener-notice-metadata-unreadable-title = «{ $member }» kunne ikke leses
import-scrivener-notice-metadata-unreadable-body = Det Scrivener lagret i den, blir ikke overført.
import-scrivener-notice-dangling-keyword-title = { $count ->
    [one] 1 henvisning peker på et nøkkelord prosjektet ikke definerer
   *[other] { $count } henvisninger peker på nøkkelord prosjektet ikke definerer
}
import-scrivener-notice-dangling-label-title = { $count ->
    [one] 1 henvisning peker på en etikett prosjektet ikke definerer
   *[other] { $count } henvisninger peker på etiketter prosjektet ikke definerer
}
import-scrivener-notice-dangling-status-title = { $count ->
    [one] 1 henvisning peker på en status prosjektet ikke definerer
   *[other] { $count } henvisninger peker på statuser prosjektet ikke definerer
}
import-scrivener-notice-dangling-section-type-title = { $count ->
    [one] 1 henvisning peker på en seksjonstype prosjektet ikke definerer
   *[other] { $count } henvisninger peker på seksjonstyper prosjektet ikke definerer
}
import-scrivener-notice-dangling-custom-field-title = { $count ->
    [one] 1 henvisning peker på et egendefinert metadatafelt prosjektet ikke definerer
   *[other] { $count } henvisninger peker på egendefinerte metadatafelt prosjektet ikke definerer
}
import-scrivener-notice-dangling-item-title = { $count ->
    [one] 1 henvisning peker på et dokument prosjektet ikke definerer
   *[other] { $count } henvisninger peker på dokumenter prosjektet ikke definerer
}
import-scrivener-notice-dangling-references-body = En henvisning til noe prosjektet ikke definerer, utelates.
import-scrivener-notice-snapshots-not-imported-line = { $count ->
    [one] 1 øyeblikksbilde blir værende i Scrivener-prosjektet ditt: Skribisto overfører ikke øyeblikksbilder.
   *[other] { $count } øyeblikksbilder blir værende i Scrivener-prosjektet ditt: Skribisto overfører ikke øyeblikksbilder.
}
import-scrivener-notice-writing-history-not-imported-line = { $days ->
    [one] Scriveners logg over ord skrevet per dag (1 dag) blir ikke overført. Skribisto starter sin egen telling fra første lagring.
   *[other] Scriveners logg over ord skrevet per dag ({ $days } dager) blir ikke overført. Skribisto starter sin egen telling fra første lagring.
}
import-scrivener-notice-title-placeholder-rejected-hint = Tittelen i prosjektets innstillinger er en plassholder fra en mal, så prosjektets navn foreslås i stedet.
import-scrivener-notice-author-placeholder-rejected-hint = Forfatternavnet i prosjektets innstillinger er en plassholder fra en mal, så ingen foreslås.
import-scrivener-notice-title-from-project-hint-compile-settings = Denne tittelen er den som står i prosjektets kompileringsinnstillinger. Inntil en tittel settes der, beholder Scrivener navnet prosjektet hadde da det ble opprettet.
import-scrivener-notice-title-from-project-hint-template = Denne tittelen er malens egen.
import-scrivener-notice-title-from-project-hint-folder-name = Denne tittelen er prosjektets navn.
import-scrivener-notice-title-from-project-hint-project-properties = Denne tittelen er den som står i prosjektets egenskaper.
import-scrivener-notice-author-from-project-hint = Dette navnet er det som står i prosjektets innstillinger. En mals forfatter kan bli liggende igjen der, og den ville bli skrevet ut på en tittelside.
import-scrivener-notice-language-not-stated-hint = Prosjektet oppgir ikke språket sitt, så språket Skribisto selv vises på, foreslås. Det bestemmer hvordan ord telles og hvordan anførselstegn skrives.
import-scrivener-notice-language-from-script-hint = Foreslått ut fra skriftsystemet manuset er skrevet i ({ $tag }).

## Before you import: what the import does with the binder, or leaves behind
import-scrivener-structure-script-mode-draft-line = { $script ->
    [1] { $documents ->
        [1] Manusets eneste dokument er skrevet i skriptmodus. Skribisto beholder ordene, ikke filmmanusoppsettet.
       *[other] 1 av de { $documents } dokumentene i manuset er skrevet i skriptmodus. Skribisto beholder ordene, ikke filmmanusoppsettet.
    }
   *[other] { $script } av de { $documents } dokumentene i manuset er skrevet i skriptmodus. Skribisto beholder ordene, ikke filmmanusoppsettet.
}
import-scrivener-structure-one-document-manuscript-line = { $count ->
    [1] Et dokument i manuset inneholder kapitlene sine som overskrifter.
   *[other] { $count } dokumenter i manuset inneholder kapitlene sine som overskrifter.
}
import-scrivener-structure-title-repeats-number-line = { $count ->
    [one] 1 kapittel- eller deltittel inneholder også nummeret sitt, for eksempel «Kapittel II» eller «Første del», så en eksport kan skrive ut nummeret to ganger.
   *[other] { $count } kapittel- eller deltitler inneholder også nummeret sitt, for eksempel «Kapittel II» eller «Første del», så en eksport kan skrive ut nummeret to ganger.
}
import-scrivener-structure-claim-overruled-line = { $count ->
    [one] 1 element ser ut som en bok, en del eller et kapittel, men ligger inni et element som allerede er på det nivået, så det overføres på et lavere nivå.
   *[other] { $count } elementer ser ut som bøker, deler eller kapitler, men hvert av dem ligger inni et element som allerede er på det nivået, så de overføres på et lavere nivå.
}
import-scrivener-structure-adopted-by-chapter-line = { $count ->
    [one] 1 dokument som følger etter et kapittel, blir slutten av det kapitlet.
   *[other] { $count } dokumenter som følger etter et kapittel, blir slutten av det kapitlet.
}
import-scrivener-structure-looks-like-manuscript-line = { $count ->
    [one] 1 mappe på øverste nivå som legges i notatene, ser ut som manus. Du kan sende den til manuset som en annen bok.
   *[other] { $count } mapper på øverste nivå som legges i notatene, ser ut som manus. Du kan sende dem til manuset som andre bøker.
}
import-scrivener-structure-generic-sheet-title-line = { $count ->
    [one] 1 oppføring i historiebibelen har fortsatt en tittel fra en mal, for eksempel «Helt». Skribisto leter etter den tittelen i teksten din og viser hvert sted den forekommer, under «I teksten»: gi oppføringen et nytt navn hvis ordet er vanlig i boken din.
   *[other] { $count } oppføringer i historiebibelen har fortsatt en tittel fra en mal, for eksempel «Helt». Skribisto leter etter de titlene i teksten din og viser hvert sted de forekommer, under «I teksten»: gi oppføringene nye navn hvis ordene er vanlige i boken din.
}
import-scrivener-structure-placeholder-title-line = { $count ->
    [one] 1 tittel navnga bare nivået sitt og blir stående tom: permen og eksportene nummererer den.
   *[other] { $count } titler navnga bare nivået sitt og blir stående tomme: permen og eksportene nummererer dem.
}
import-scrivener-structure-untitled-item-line = { $count ->
    [one] 1 element hadde ingen tittel i Scrivener og forblir uten tittel.
   *[other] { $count } elementer hadde ingen tittel i Scrivener og forblir uten tittel.
}
import-scrivener-structure-title-cleaned-line = { $count ->
    [one] 1 tittel inneholdt linjeskift eller tabulatorer, som fjernes.
   *[other] { $count } titler inneholdt linjeskift eller tabulatorer, som fjernes.
}
import-scrivener-structure-own-text-moved-line = { $count ->
    [one] 1 mappe har egen tekst, som flyttes til et første dokument inni den.
   *[other] { $count } mapper har egen tekst, som flyttes til et første dokument inni hver av dem.
}
import-scrivener-structure-text-with-children-line = { $count ->
    [one] 1 dokument med dokumenter inni seg blir en mappe.
   *[other] { $count } dokumenter med dokumenter inni seg blir mapper.
}
import-scrivener-structure-empty-folder-plain-line = { $count ->
    [one] 1 tom mappe forblir en vanlig mappe, så ingen eksport skriver ut en overskrift Scrivener aldri skrev ut.
   *[other] { $count } tomme mapper forblir vanlige mapper, så ingen eksport skriver ut en overskrift Scrivener aldri skrev ut.
}
import-scrivener-structure-book-inside-book-line = { $count ->
    [one] 1 bok inne i en bok blir en del.
   *[other] { $count } bøker inne i en bok blir deler.
}
import-scrivener-structure-prologue-not-numbered-line = { $count ->
    [one] 1 prolog eller epilog holdes utenfor kapittelnummereringen.
   *[other] { $count } prologer eller epiloger holdes utenfor kapittelnummereringen.
}
import-scrivener-structure-matter-inside-book-line = { $count ->
    [one] 1 element med fortekst eller ettertekst fra utenfor manuset legges inn i boken som paratekst, utelatt fra eksporter til du tar det med.
   *[other] { $count } elementer med fortekst eller ettertekst fra utenfor manuset legges inn i boken som paratekst, utelatt fra eksporter til du tar dem med.
}
import-scrivener-structure-paratext-not-counted-line = { $documents ->
    [one] 1 dokument i manuset blir paratekst (fortekst eller ettertekst), så { $words ->
        [one] { $words } ord teller
       *[other] { $words } ord teller
    } ikke lenger som manus.
   *[other] { $documents } dokumenter i manuset blir paratekst (fortekst eller ettertekst), så { $words ->
        [one] { $words } ord teller
       *[other] { $words } ord teller
    } ikke lenger som manus.
}
import-scrivener-structure-document-notes-line = { $count ->
    [one] Notatene til 1 dokument blir et notat ved siden av det.
   *[other] Notatene til { $count } dokumenter blir notater ved siden av dem.
}
import-scrivener-structure-draft-content-as-note-line = Manusmappens egen tekst blir et notat øverst i manuset.
import-scrivener-structure-story-bible-folder-line = { $count ->
    [one] 1 mappe med personer eller steder blir et stikkord i historiebibelen.
   *[other] { $count } mapper med personer eller steder blir stikkord i historiebibelen.
}
import-scrivener-structure-templates-created-line = { $count ->
    [one] 1 dokumentmal blir en notatmal.
   *[other] { $count } dokumentmaler blir notatmaler.
}
import-scrivener-structure-template-nesting-flattened-line = { $count ->
    [one] 1 malmappe inne i en annen flates ut: hver tekst i den blir en mal.
   *[other] { $count } malmapper inne i andre flates ut: hver tekst i dem blir en mal.
}
import-scrivener-structure-template-kept-as-note-line = { $count ->
    [one] 1 mal inneholder noe en notatmal ikke kan inneholde, og beholdes som et notat.
   *[other] { $count } maler inneholder noe en notatmal ikke kan inneholde, og beholdes som notater.
}
import-scrivener-structure-recovered-texts-line = { $count ->
    [one] 1 tekst som ikke hørte til noe element i permen, overføres i en egen mappe.
   *[other] { $count } tekster som ikke hørte til noe element i permen, overføres i en egen mappe.
}
import-scrivener-structure-project-notes-line = { $count ->
    [one] 1 prosjektnotat overføres i en egen mappe.
   *[other] { $count } prosjektnotater overføres i en egen mappe.
}
import-scrivener-structure-trash-brought-across-line = { $items ->
    [1] Papirkurven overføres: 1 element, som du kan gjenopprette fra papirkurven.
   *[other] Papirkurven overføres: { $items } elementer, { $writing ->
        [0] ingen av dem med tekst
       *[other] { $writing } av dem med tekst
    }, som du kan gjenopprette fra papirkurven.
}
import-scrivener-structure-not-compiled-line = { $count ->
    [one] 1 element som Scrivener holdt utenfor kompileringen, holdes også utenfor eksporter.
   *[other] { $count } elementer som Scrivener holdt utenfor kompileringen, holdes også utenfor eksporter.
}
import-scrivener-structure-media-in-draft-line = { $count ->
    [one] 1 bilde eller fil i manuset blir et notat, siden en bok ikke kan inneholde det.
   *[other] { $count } bilder eller filer i manuset blir notater, siden en bok ikke kan inneholde dem.
}
import-scrivener-structure-left-out-line = { $count ->
    [one] 1 mappe på øverste nivå utelates, med { $items ->
        [one] 1 element
       *[other] { $items } elementer
    }.
   *[other] { $count } mapper på øverste nivå utelates, med { $items ->
        [one] 1 element
       *[other] { $items } elementer
    }.
}
import-scrivener-structure-trash-left-out-line = { $items ->
    [1] Papirkurven blir ikke overført: 1 element.
   *[other] Papirkurven blir ikke overført: { $items } elementer, { $writing ->
        [0] ingen av dem med tekst
       *[other] { $writing } av dem med tekst
    }.
}
import-scrivener-structure-card-image-left-behind-line = { $count ->
    [one] 1 registerkortbilde på et element i manuset blir ikke overført: det ville blitt skrevet ut i boken.
   *[other] { $count } registerkortbilder på elementer i manuset blir ikke overført: de ville blitt skrevet ut i boken.
}
import-scrivener-structure-default-template-not-linked-line = { $count ->
    [one] 1 mappe hadde en standardmal for nye dokumenter i den. I Skribisto kan bare et stikkord i historiebibelen ha en mal, så den koblingen beholdes ikke.
   *[other] { $count } mapper hadde en standardmal for nye dokumenter i dem. I Skribisto kan bare et stikkord i historiebibelen ha en mal, så de koblingene beholdes ikke.
}
import-scrivener-structure-page-break-not-kept-line = { $count ->
    [one] 1 innstilling «Page Break Before» (sideskift før) har ingen plass i Skribisto og beholdes ikke.
   *[other] { $count } innstillinger «Page Break Before» (sideskift før) har ingen plass i Skribisto og beholdes ikke.
}
import-scrivener-structure-compile-as-is-not-kept-line = { $count ->
    [one] 1 innstilling «Compile As-Is» (kompiler uendret) har ingen motsvarighet i Skribisto og beholdes ikke.
   *[other] { $count } innstillinger «Compile As-Is» (kompiler uendret) har ingen motsvarighet i Skribisto og beholdes ikke.
}

## What the import did with the labels, the status list and the dates
import-scrivener-metadata-status-categories-guessed-line = { $count ->
    [one] 1 status som Skribisto ikke kjenner igjen, får en kategori ut fra plassen sin i listen. Sjekk den i Innstillinger ▸ Prosjekt ▸ Statuser.
   *[other] { $count } statuser som Skribisto ikke kjenner igjen, får en kategori ut fra plassen sin i listen. Sjekk dem i Innstillinger ▸ Prosjekt ▸ Statuser.
}
import-scrivener-metadata-dates-unreadable-line = { $count ->
    [one] Datoene til 1 element kunne ikke leses: det får datoen for importen.
   *[other] Datoene til { $count } elementer kunne ikke leses: de får datoen for importen.
}
import-scrivener-metadata-labels-became-tags-line = { $tags ->
    [one] Etikettene blir 1 stikkord, brukt på { $rows ->
        [one] { $rows } element
       *[other] { $rows } elementer
    }.
   *[other] Etikettene blir { $tags } stikkord, brukt på { $rows ->
        [one] { $rows } element
       *[other] { $rows } elementer
    }.
}
import-scrivener-metadata-statuses-became-ladder-line = { $rungs ->
    [one] Statuslisten overføres som prosjektets liste over statuser: { $rungs } status, brukt på { $rows ->
        [one] { $rows } element
       *[other] { $rows } elementer
    }.
   *[other] Statuslisten overføres som prosjektets liste over statuser: { $rungs } statuser, brukt på { $rows ->
        [one] { $rows } element
       *[other] { $rows } elementer
    }.
}
import-scrivener-metadata-ladder-seeded-line = { $count ->
    [1] Prosjektet hadde ingen statusliste, så det starter med Skribistos standardstatus.
   *[other] Prosjektet hadde ingen statusliste, så det starter med Skribistos { $count } standardstatuser.
}
import-scrivener-metadata-label-renamed-line = Etikettfeltet het «{ $title }» i Scrivener. Det navnet står i detaljene til hvert stikkord.
import-scrivener-metadata-status-renamed-line = Statusfeltet het «{ $title }» i Scrivener. Det navnet står i detaljene til hver status.
import-scrivener-metadata-none-label-collapsed-line = { $count ->
    [one] Etiketten «{ $name }», som ligger ved siden av Scriveners egen tomme etikett med samme navn, leses som ingen etikett på 1 element.
   *[other] Etiketten «{ $name }», som ligger ved siden av Scriveners egen tomme etikett med samme navn, leses som ingen etikett på { $count } elementer.
}
import-scrivener-metadata-none-status-collapsed-line = { $count ->
    [one] Statusen «{ $name }», som ligger ved siden av Scriveners egen tomme status med samme navn, leses som ingen status på 1 element.
   *[other] Statusen «{ $name }», som ligger ved siden av Scriveners egen tomme status med samme navn, leses som ingen status på { $count } elementer.
}
import-scrivener-metadata-unnamed-label-line = En etikett uten navn får navn etter nummeret sitt, { $id }.
import-scrivener-metadata-unnamed-status-line = En status uten navn får navn etter nummeret sitt, { $id }.
import-scrivener-metadata-tag-name-disambiguated-line = { $count ->
    [one] 1 stikkord får et nytt navn, fordi et annet stikkord allerede har navnet.
   *[other] { $count } stikkord får nye navn, fordi andre stikkord allerede har navnene.
}
import-scrivener-metadata-rung-names-disambiguated-line = { $count ->
    [one] 1 status har samme navn som en annen, så den får et nummer etter navnet. Sjekk den i Innstillinger ▸ Prosjekt ▸ Statuser.
   *[other] { $count } statuser har samme navn som en annen, så hver av dem får et nummer etter navnet. Sjekk dem i Innstillinger ▸ Prosjekt ▸ Statuser.
}
import-scrivener-metadata-unused-labels-skipped-line = { $count ->
    [one] 1 etikett som ingen dokumenter har, importeres ikke.
   *[other] { $count } etiketter som ingen dokumenter har, importeres ikke.
}
import-scrivener-metadata-labels-without-row-line = { $count ->
    [one] Etiketten til 1 element som ikke har noen plass i det nye prosjektet, blir ikke overført.
   *[other] Etikettene til { $count } elementer som ikke har noen plass i det nye prosjektet, blir ikke overført.
}
import-scrivener-metadata-statuses-without-row-line = { $count ->
    [one] Statusen til 1 element som ikke har noen plass i det nye prosjektet, blir ikke overført.
   *[other] Statusene til { $count } elementer som ikke har noen plass i det nye prosjektet, blir ikke overført.
}
import-scrivener-metadata-unnamed-keyword-line = Et nøkkelord uten navn får navn etter nummeret sitt, { $id }.
import-scrivener-metadata-keywords-without-row-line = { $count ->
    [one] Nøkkelordene til 1 element som ikke har noen plass i det nye prosjektet, blir ikke overført.
   *[other] Nøkkelordene til { $count } elementer som ikke har noen plass i det nye prosjektet, blir ikke overført.
}
import-scrivener-metadata-custom-values-without-row-line = { $count ->
    [one] De egendefinerte metadataene til 1 element som ikke har noen plass i det nye prosjektet, blir ikke overført.
   *[other] De egendefinerte metadataene til { $count } elementer som ikke har noen plass i det nye prosjektet, blir ikke overført.
}
import-scrivener-metadata-bookmarks-without-row-line = { $count ->
    [one] Bokmerkene til 1 element som ikke har noen plass i det nye prosjektet, blir ikke overført.
   *[other] Bokmerkene til { $count } elementer som ikke har noen plass i det nye prosjektet, blir ikke overført.
}
import-scrivener-metadata-targets-without-row-line = { $count ->
    [one] Målet til 1 element som ikke har noen plass i det nye prosjektet, blir ikke overført.
   *[other] Målene til { $count } elementer som ikke har noen plass i det nye prosjektet, blir ikke overført.
}

## What the import does with the keywords, custom metadata, collections, bookmarks, targets,
## the deadline, the auto-complete list and the icons.
import-scrivener-metadata-pov-hint-line = { $count ->
    [1] Etiketten eller feltverdien { $values } samsvarer med oppføringen { $matches } i historiebibelen. Hvis den angir synsvinkelen, setter du den i inspektøren: importen kobler ikke noe.
   *[other] Etikettene eller feltverdiene { $values } samsvarer med oppføringene { $matches } i historiebibelen. Hvis de angir synsvinkler, setter du dem i inspektøren: importen kobler ikke noe.
}
import-scrivener-metadata-checkbox-default-unknown-line = Avkrysningsfeltet «{ $field }» har en standardverdi Skribisto ikke kan lese, så det kan ikke avgjøre hvilke dokumenter som er krysset av som standard. Bare dokumentene som er krysset av ett for ett, får stikkordet.
import-scrivener-metadata-goals-in-other-unit-line = { $count ->
    [one] 1 mål er satt i den andre telleenheten. Det beholdes, og vises når du bytter telleenhet i Innstillinger ▸ Prosjekt ▸ Struktur.
   *[other] { $count } mål er satt i den andre telleenheten. De beholdes, og vises når du bytter telleenhet i Innstillinger ▸ Prosjekt ▸ Struktur.
}
import-scrivener-metadata-counting-differs-line = Scrivener regnet med andre dokumenter i utkastmålet enn Skribisto gjør. Bokens mål teller scenene som tas med i eksporter.
import-scrivener-metadata-writing-days-not-read-line = Skrivedagene «{ $raw }» kan ikke leses, så bokens Tempo regner hver dag som en skrivedag. Endre dem der.
import-scrivener-metadata-script-mode-rows-line = { $count ->
    [one] 1 element var i skriptmodus i Scrivener. Skribisto beholder ordene som prosa, uten filmmanusoppsettet.
   *[other] { $count } elementer var i skriptmodus i Scrivener. Skribisto beholder ordene som prosa, uten filmmanusoppsettet.
}
import-scrivener-metadata-keywords-became-tags-line = { $tags ->
    [one] 1 nøkkelord blir et stikkord, brukt på { $rows ->
        [one] { $rows } element
       *[other] { $rows } elementer
    }.
   *[other] { $tags } nøkkelord blir stikkord, brukt på { $rows ->
        [one] { $rows } element
       *[other] { $rows } elementer
    }.
}
import-scrivener-metadata-keyword-hierarchy-flattened-line = { $count ->
    [one] 1 nøkkelord lå inne i et annet i Scrivener. Stikkord ligger på ett nivå: detaljene til stikkordet sier hvor det lå.
   *[other] { $count } nøkkelord lå inne i andre i Scrivener. Stikkord ligger på ett nivå: detaljene til hvert stikkord sier hvor nøkkelordet lå.
}
import-scrivener-metadata-fields-became-tags-line = { $fields ->
    [one] 1 liste- eller avkrysningsfelt blir { $tags ->
        [one] { $tags } stikkord
       *[other] { $tags } stikkord
    }, brukt på { $rows ->
        [one] { $rows } element
       *[other] { $rows } elementer
    }.
   *[other] { $fields } liste- og avkrysningsfelt blir { $tags ->
        [one] { $tags } stikkord
       *[other] { $tags } stikkord
    }, brukt på { $rows ->
        [one] { $rows } element
       *[other] { $rows } elementer
    }.
}
import-scrivener-metadata-collections-became-tags-line = { $tags ->
    [one] 1 samling blir et stikkord, brukt på { $rows ->
        [one] { $rows } element
       *[other] { $rows } elementer
    }.
   *[other] { $tags } samlinger blir stikkord, brukt på { $rows ->
        [one] { $rows } element
       *[other] { $rows } elementer
    }.
}
import-scrivener-metadata-collection-order-lost-line = { $count ->
    [one] Et stikkord har ingen rekkefølge, så rekkefølgen på dokumentene i 1 samling beholdes ikke.
   *[other] Et stikkord har ingen rekkefølge, så rekkefølgen på dokumentene i { $count } samlinger beholdes ikke.
}
import-scrivener-metadata-bookmarks-became-links-line = { $count ->
    [one] 1 bokmerke blir en lenke mellom to notater, eller mellom en scene og en oppføring i historiebibelen: se Lenker og Persongalleri.
   *[other] { $count } bokmerker blir lenker mellom notater, eller mellom scener og oppføringer i historiebibelen: se Lenker og Persongalleri.
}
import-scrivener-metadata-goals-kept-line = { $count ->
    [one] Målet Scrivener satte for 1 dokument, blir dokumentets mål i Skribisto.
   *[other] Målene Scrivener satte for { $count } dokumenter, blir målene deres i Skribisto.
}
import-scrivener-metadata-draft-target-kept-words-line = { $value ->
    [one] Utkastmålet på 1 ord blir bokens mål.
   *[other] Utkastmålet på { $value } ord blir bokens mål.
}
import-scrivener-metadata-draft-target-kept-characters-line = { $value ->
    [one] Utkastmålet på 1 tegn blir bokens mål.
   *[other] Utkastmålet på { $value } tegn blir bokens mål.
}
import-scrivener-metadata-pace-planned-line = Fristen, { $day }, setter opp bokens Tempo, som fordeler ordene på hver skrivedag du valgte i Scrivener, fra og med importdagen.
import-scrivener-metadata-pace-planned-day-off-line = Fristen, { $day }, setter opp bokens Tempo, som fordeler ordene på hver skrivedag du valgte i Scrivener, fra og med importdagen. Som i Scrivener er selve fristdagen en fridag.
import-scrivener-metadata-dictionary-words-line = { $count ->
    [one] 1 autofullføringsord legges i prosjektets personlige ordbok, så det ikke markeres som feilstavet.
   *[other] { $count } autofullføringsord legges i prosjektets personlige ordbok, så de ikke markeres som feilstavet.
}
import-scrivener-metadata-listed-project-line = { $count ->
    [one] 1 opplysning om prosjektet står oppført i notatet «{ $note }» i mappen «{ $folder }».
   *[other] { $count } opplysninger om prosjektet står oppført i notatet «{ $note }» i mappen «{ $folder }».
}
import-scrivener-metadata-listed-project-bookmarks-line = { $count ->
    [one] 1 prosjektbokmerke står oppført i notatet «{ $note }» i mappen «{ $folder }».
   *[other] { $count } prosjektbokmerker står oppført i notatet «{ $note }» i mappen «{ $folder }».
}
import-scrivener-metadata-listed-replacements-line = { $count ->
    [one] 1 erstatning ved kompilering står oppført i notatet «{ $note }» i mappen «{ $folder }». Skribisto utfører den ikke.
   *[other] { $count } erstatninger ved kompilering står oppført i notatet «{ $note }» i mappen «{ $folder }». Skribisto utfører dem ikke.
}
import-scrivener-metadata-listed-auto-complete-line = { $count ->
    [one] 1 autofullføringsoppføring på flere ord står oppført i notatet «{ $note }» i mappen «{ $folder }».
   *[other] { $count } autofullføringsoppføringer på flere ord står oppført i notatet «{ $note }» i mappen «{ $folder }».
}
import-scrivener-metadata-listed-saved-searches-line = { $count ->
    [one] 1 lagret søk står oppført i notatet «{ $note }» i mappen «{ $folder }».
   *[other] { $count } lagrede søk står oppført i notatet «{ $note }» i mappen «{ $folder }».
}
import-scrivener-metadata-listed-custom-metadata-line = { $count ->
    [one] 1 egendefinert metadataverdi som et stikkord ikke kan inneholde, står oppført i notatet «{ $note }» i mappen «{ $folder }».
   *[other] { $count } egendefinerte metadataverdier som et stikkord ikke kan inneholde, står oppført i notatet «{ $note }» i mappen «{ $folder }».
}
import-scrivener-metadata-listed-targets-line = { $count ->
    [one] 1 dokumentmål står oppført i notatet «{ $note }» i mappen «{ $folder }».
   *[other] { $count } dokumentmål står oppført i notatet «{ $note }» i mappen «{ $folder }».
}
import-scrivener-metadata-listed-bookmarks-line = { $count ->
    [one] 1 bokmerke som ingen lenke i Skribisto viser, står oppført i notatet «{ $note }» i mappen «{ $folder }».
   *[other] { $count } bokmerker som ingen lenke i Skribisto viser, står oppført i notatet «{ $note }» i mappen «{ $folder }».
}
import-scrivener-metadata-targets-on-notes-line = { $count ->
    [one] 1 dokumentmål står på et notat, der teksten ikke telles av noe mål, så det blir ikke et mål.
   *[other] { $count } dokumentmål står på notater, der teksten ikke telles av noe mål, så de blir ikke mål.
}
import-scrivener-metadata-targets-on-split-documents-line = { $count ->
    [one] 1 dokumentmål står på et dokument som deles opp ved overskriftene, og ingen enkeltmål teller kapitlene i det sammen, så det blir ikke et mål.
   *[other] { $count } dokumentmål står på dokumenter som deles opp ved overskriftene, og ingen enkeltmål teller kapitlene i dem sammen, så de blir ikke mål.
}
import-scrivener-metadata-bookmark-ends-without-row-line = { $count ->
    [one] 1 bokmerke fører til et dokument som ikke har noen plass i det nye prosjektet, for eksempel en dokumentmal. Det står oppført etter hvor dokumentet lå i Scrivener.
   *[other] { $count } bokmerker fører til dokumenter som ikke har noen plass i det nye prosjektet, for eksempel dokumentmaler. De står oppført etter hvor dokumentene lå i Scrivener.
}
import-scrivener-metadata-unused-keywords-skipped-line = { $count ->
    [one] 1 nøkkelord som ingen dokumenter har, importeres ikke.
   *[other] { $count } nøkkelord som ingen dokumenter har, importeres ikke.
}
import-scrivener-metadata-list-value-invalid-line = { $count ->
    [one] 1 verdi i listefeltet «{ $field }» svarer ikke til noen av alternativene og importeres ikke.
   *[other] { $count } verdier i listefeltet «{ $field }» svarer ikke til noen av alternativene og importeres ikke.
}
import-scrivener-metadata-undefined-custom-field-line = { $count ->
    [one] 1 verdi hører til et egendefinert felt prosjektet ikke lenger definerer, og som Scrivener heller ikke viser. Den importeres ikke.
   *[other] { $count } verdier hører til egendefinerte felt prosjektet ikke lenger definerer, og som Scrivener heller ikke viser. De importeres ikke.
}
import-scrivener-metadata-collection-members-missing-line = { $count ->
    [one] 1 dokument i samlingen «{ $name }» har ingen plass i det nye prosjektet, eller er borte fra Scrivener-prosjektet.
   *[other] { $count } dokumenter i samlingen «{ $name }» har ingen plass i det nye prosjektet, eller er borte fra Scrivener-prosjektet.
}
import-scrivener-metadata-empty-collections-skipped-line = { $count ->
    [one] 1 samling uten dokumenter importeres ikke.
   *[other] { $count } samlinger uten dokumenter importeres ikke.
}
import-scrivener-metadata-bookmarks-unresolved-line = { $count ->
    [one] 1 bokmerke fører til et dokument Scrivener-prosjektet ikke lenger inneholder, og importeres ikke.
   *[other] { $count } bokmerker fører til dokumenter Scrivener-prosjektet ikke lenger inneholder, og importeres ikke.
}
import-scrivener-metadata-targets-in-pages-line = { $count ->
    [one] 1 dokumentmål teller sider, som Skribisto ikke teller, så det blir ikke et mål.
   *[other] { $count } dokumentmål teller sider, som Skribisto ikke teller, så de blir ikke mål.
}
import-scrivener-metadata-targets-unreadable-line = { $count ->
    [one] 1 mål importeres ikke: verdien er ikke et tall, er under null eller er større enn et mål kan romme.
   *[other] { $count } mål importeres ikke: verdiene er ikke tall, er under null eller er større enn et mål kan romme.
}
import-scrivener-metadata-draft-target-in-pages-line = { $value ->
    [one] Utkastmålet på 1 side blir ikke et mål: Skribisto teller ord eller tegn. Det står oppført i notatet «{ $note }» i mappen «{ $folder }».
   *[other] Utkastmålet på { $value } sider blir ikke et mål: Skribisto teller ord eller tegn. Det står oppført i notatet «{ $note }» i mappen «{ $folder }».
}
import-scrivener-metadata-draft-target-no-book-words-line = { $value ->
    [one] Utkastmålet på 1 ord blir ikke et mål, fordi manuset utgjør flere bøker, eller ingen. Det står oppført i notatet «{ $note }» i mappen «{ $folder }».
   *[other] Utkastmålet på { $value } ord blir ikke et mål, fordi manuset utgjør flere bøker, eller ingen. Det står oppført i notatet «{ $note }» i mappen «{ $folder }».
}
import-scrivener-metadata-draft-target-no-book-characters-line = { $value ->
    [one] Utkastmålet på 1 tegn blir ikke et mål, fordi manuset utgjør flere bøker, eller ingen. Det står oppført i notatet «{ $note }» i mappen «{ $folder }».
   *[other] Utkastmålet på { $value } tegn blir ikke et mål, fordi manuset utgjør flere bøker, eller ingen. Det står oppført i notatet «{ $note }» i mappen «{ $folder }».
}
import-scrivener-metadata-deadline-passed-line = Fristen, { $day }, er passert, så den setter ikke opp bokens Tempo. Den står oppført i notatet «{ $note }» i mappen «{ $folder }».
import-scrivener-metadata-deadline-no-writing-day-line = Det er ingen skrivedager igjen fra importdagen til fristen, { $day }, så den setter ikke opp bokens Tempo. Den står oppført i notatet «{ $note }» i mappen «{ $folder }».
import-scrivener-metadata-deadline-template-line = Fristen, { $day }, ble lagret i malen, så den setter ikke opp bokens Tempo. Den står oppført i notatet «{ $note }» i mappen «{ $folder }».
import-scrivener-metadata-deadline-no-word-goal-line = Fristen, { $day }, setter ikke opp bokens Tempo, fordi utkastet ikke har noe ordmål å planlegge mot. Den står oppført i notatet «{ $note }» i mappen «{ $folder }».
import-scrivener-metadata-deadline-character-goal-line = Fristen, { $day }, setter ikke opp bokens Tempo, fordi utkastmålet teller tegn og Tempo teller ord. Den står oppført i notatet «{ $note }» i mappen «{ $folder }».
import-scrivener-metadata-deadline-no-book-line = Fristen, { $day }, setter ikke opp Tempo for noen bok, fordi manuset utgjør flere bøker, eller ingen. Den står oppført i notatet «{ $note }» i mappen «{ $folder }».
import-scrivener-metadata-deadline-unreadable-line = Fristen kan ikke leses, så den setter ikke opp bokens Tempo. Den står oppført slik den var skrevet, i notatet «{ $note }» i mappen «{ $folder }».
import-scrivener-metadata-session-target-words-line = { $value ->
    [one] Øktmålet på 1 ord importeres ikke: i Skribisto er målet for skriveøkten én innstilling for alle prosjekter. Det står oppført i notatet «{ $note }» i mappen «{ $folder }».
   *[other] Øktmålet på { $value } ord importeres ikke: i Skribisto er målet for skriveøkten én innstilling for alle prosjekter. Det står oppført i notatet «{ $note }» i mappen «{ $folder }».
}
import-scrivener-metadata-session-target-characters-line = { $value ->
    [one] Øktmålet på 1 tegn importeres ikke: i Skribisto er målet for skriveøkten én innstilling for alle prosjekter, i ord. Det står oppført i notatet «{ $note }» i mappen «{ $folder }».
   *[other] Øktmålet på { $value } tegn importeres ikke: i Skribisto er målet for skriveøkten én innstilling for alle prosjekter, i ord. Det står oppført i notatet «{ $note }» i mappen «{ $folder }».
}
import-scrivener-metadata-session-target-pages-line = { $value ->
    [one] Øktmålet på 1 side importeres ikke: i Skribisto er målet for skriveøkten én innstilling for alle prosjekter, i ord. Det står oppført i notatet «{ $note }» i mappen «{ $folder }».
   *[other] Øktmålet på { $value } sider importeres ikke: i Skribisto er målet for skriveøkten én innstilling for alle prosjekter, i ord. Det står oppført i notatet «{ $note }» i mappen «{ $folder }».
}
import-scrivener-metadata-session-target-calculated-words-line = { $value ->
    [one] Øktmålet på 1 ord, som Scrivener beregnet ut fra fristen og skrivedagene, importeres ikke: i Skribisto er målet for skriveøkten én innstilling for alle prosjekter. Det står oppført i notatet «{ $note }» i mappen «{ $folder }».
   *[other] Øktmålet på { $value } ord, som Scrivener beregnet ut fra fristen og skrivedagene, importeres ikke: i Skribisto er målet for skriveøkten én innstilling for alle prosjekter. Det står oppført i notatet «{ $note }» i mappen «{ $folder }».
}
import-scrivener-metadata-session-target-calculated-characters-line = { $value ->
    [one] Øktmålet på 1 tegn, som Scrivener beregnet ut fra fristen og skrivedagene, importeres ikke: i Skribisto er målet for skriveøkten én innstilling for alle prosjekter, i ord. Det står oppført i notatet «{ $note }» i mappen «{ $folder }».
   *[other] Øktmålet på { $value } tegn, som Scrivener beregnet ut fra fristen og skrivedagene, importeres ikke: i Skribisto er målet for skriveøkten én innstilling for alle prosjekter, i ord. Det står oppført i notatet «{ $note }» i mappen «{ $folder }».
}
import-scrivener-metadata-session-target-calculated-pages-line = { $value ->
    [one] Øktmålet på 1 side, som Scrivener beregnet ut fra fristen og skrivedagene, importeres ikke: i Skribisto er målet for skriveøkten én innstilling for alle prosjekter, i ord. Det står oppført i notatet «{ $note }» i mappen «{ $folder }».
   *[other] Øktmålet på { $value } sider, som Scrivener beregnet ut fra fristen og skrivedagene, importeres ikke: i Skribisto er målet for skriveøkten én innstilling for alle prosjekter, i ord. Det står oppført i notatet «{ $note }» i mappen «{ $folder }».
}
import-scrivener-metadata-icons-dropped-line = { $count ->
    [one] 1 element hadde et ikon i permen, som ikke blir overført: Skribisto viser hvert elements ikon ut fra typen.
   *[other] { $count } elementer hadde ikoner i permen, som ikke blir overført: Skribisto viser hvert elements ikon ut fra typen.
}

## What converting the prose did, as the import's report says it.
import-scrivener-prose-unreadable-line = { $count ->
    [one] 1 tekst kunne ikke leses. Den kommer inn tom.
   *[other] { $count } tekster kunne ikke leses. De kommer inn tomme.
}
import-scrivener-prose-comments-unreadable-line = { $count ->
    [one] Kommentarene og de lenkede fotnotene i 1 tekst kunne ikke leses. Ordene de markerte, beholdes.
   *[other] Kommentarene og de lenkede fotnotene i { $count } tekster kunne ikke leses. Ordene de markerte, beholdes.
}
import-scrivener-prose-read-as-plain-text-line = { $count ->
    [one] 1 tekst var lagret som ren tekst i stedet for formatert tekst, og leses som det.
   *[other] { $count } tekster var lagret som ren tekst i stedet for formatert tekst, og leses som det.
}
import-scrivener-prose-inline-note-unmatched-line = { $count ->
    [one] 1 markør for et innebygd notat danner ikke par med en annen og beholdes som tekst.
   *[other] { $count } markører for innebygde notater danner ikke par med en annen og beholdes som tekst.
}
import-scrivener-prose-style-unresolved-line = { $count ->
    [one] 1 stilmarkør viser til en stil prosjektet ikke definerer. Den fjernes, og teksten beholder formateringen sin.
   *[other] { $count } stilmarkører viser til stiler prosjektet ikke definerer. De fjernes, og teksten beholder formateringen sin.
}
import-scrivener-prose-styles-lost-in-export-line = { $count ->
    [one] 1 avsnitt kan være en overskrift som mistet stilen sin da Scrivener 3 eksporterte prosjektet til det eldre formatet sitt, og overføres som et vanlig avsnitt. Sjekk det, eller importer prosjektet det ble eksportert fra.
   *[other] { $count } avsnitt kan være overskrifter som mistet stilen sin da Scrivener 3 eksporterte prosjektet til det eldre formatet sitt, og overføres som vanlige avsnitt. Sjekk dem, eller importer prosjektet de ble eksportert fra.
}
import-scrivener-prose-comment-anchor-without-entry-line = { $count ->
    [one] 1 kommentar eller fotnote hadde mistet innholdet sitt i Scrivener. Ordene den markerte, beholdes, uten den.
   *[other] { $count } kommentarer eller fotnoter hadde mistet innholdet sitt i Scrivener. Ordene de markerte, beholdes, uten dem.
}
import-scrivener-prose-comment-entry-without-anchor-line = { $count ->
    [one] 1 kommentar hadde ingen plass igjen i teksten sin og knyttes til det første avsnittet.
   *[other] { $count } kommentarer hadde ingen plass igjen i teksten sin og knyttes til det første avsnittet.
}
import-scrivener-prose-footnote-entry-without-anchor-line = { $count ->
    [one] 1 lenket fotnote hadde ingen plass igjen i teksten sin, så den settes inn på slutten av den teksten.
   *[other] { $count } lenkede fotnoter hadde ingen plass igjen i tekstene sine, så hver av dem settes inn på slutten av sin egen tekst.
}
import-scrivener-prose-range-comment-reading-line = { $count ->
    [one] 1 kommentar fra en eldre Scrivener ble plassert ut fra en annen tolkning av posisjonen. Sjekk at den står på de riktige ordene.
   *[other] { $count } kommentarer fra en eldre Scrivener ble plassert ut fra en annen tolkning av posisjonen. Sjekk at de står på de riktige ordene.
}
import-scrivener-prose-range-comment-reading-markers-counted-line = { $count ->
    [one] 1 kommentar fra en eldre Scrivener ble plassert ved å telle markørene for de innebygde notatene. Sjekk at den står på de riktige ordene.
   *[other] { $count } kommentarer fra en eldre Scrivener ble plassert ved å telle markørene for de innebygde notatene. Sjekk at de står på de riktige ordene.
}
import-scrivener-prose-range-comment-reading-pictures-not-counted-line = { $count ->
    [one] 1 kommentar fra en eldre Scrivener ble plassert ved å se bort fra bildene. Sjekk at den står på de riktige ordene.
   *[other] { $count } kommentarer fra en eldre Scrivener ble plassert ved å se bort fra bildene. Sjekk at de står på de riktige ordene.
}
import-scrivener-prose-range-comment-approximate-line = { $count ->
    [one] 1 kommentar fra en eldre Scrivener passer ikke nøyaktig med noen tolkning av posisjonen, og kan stå noen tegn feil.
   *[other] { $count } kommentarer fra en eldre Scrivener passer ikke nøyaktig med noen tolkning av posisjonen, og kan stå noen tegn feil.
}
import-scrivener-prose-document-link-unresolved-line = { $count ->
    [one] 1 lenke peker på et dokument importen ikke overførte. Ordene blir stående.
   *[other] { $count } lenker peker på dokumenter importen ikke overførte. Ordene blir stående.
}
import-scrivener-prose-linked-picture-not-read-line = { $count ->
    [1] 1 bilde som var lenket til en fil på disken din, «{ $example }», ble ikke overført.
   *[other] { $count } bilder som var lenket til filer på disken din, for eksempel «{ $example }», ble ikke overført.
}
import-scrivener-prose-heading-repeats-title-line = { $count ->
    [one] 1 tekst begynner med en overskrift som gjentar tittelen, så en eksport kan skrive ut tittelen to ganger.
   *[other] { $count } tekster begynner med en overskrift som gjentar tittelen, så en eksport kan skrive ut tittelen to ganger.
}
import-scrivener-prose-markdown-as-text-line = { $count ->
    [one] 1 avsnitt skrevet i Markdown beholdes som tekst, slik det står.
   *[other] { $count } avsnitt skrevet i Markdown beholdes som tekst, slik de står.
}
import-scrivener-prose-citation-keys-as-text-line = { $count ->
    [one] 1 avsnitt inneholder kildenøkler for et referanseprogram, som beholdes som tekst.
   *[other] { $count } avsnitt inneholder kildenøkler for et referanseprogram, som beholdes som tekst.
}
import-scrivener-prose-placeholders-in-exported-text-line = { $count ->
    [1] 1 Scrivener-plassholder, { $example }, beholdes som ren tekst i det du eksporterer. Erstatt den for hånd.
   *[other] { $count } Scrivener-plassholdere, for eksempel { $example }, beholdes som ren tekst i det du eksporterer. Erstatt dem for hånd.
}
import-scrivener-prose-placeholders-in-other-text-line = { $count ->
    [1] 1 Scrivener-plassholder, { $example }, beholdes som ren tekst i en tekst som ikke eksporteres.
   *[other] { $count } Scrivener-plassholdere, for eksempel { $example }, beholdes som ren tekst i tekster som ikke eksporteres.
}
import-scrivener-prose-ornament-lines-kept-line = { $count ->
    [one] 1 midtstilt linje med ornamenter kan markere et sceneskift og beholdes slik den er skrevet.
   *[other] { $count } midtstilte linjer med ornamenter kan markere sceneskift og beholdes slik de er skrevet.
}
import-scrivener-prose-long-document-line = { $count ->
    [one] 1 tekst er svært lang og kan være treg å åpne.
   *[other] { $count } tekster er svært lange og kan være trege å åpne.
}
import-scrivener-prose-pictures-without-description-line = { $count ->
    [one] 1 bilde har ingen beskrivelse for lesere som ikke kan se det. Du kan legge til en i redigeringsruten.
   *[other] { $count } bilder har ingen beskrivelse for lesere som ikke kan se dem. Du kan legge til en i redigeringsruten.
}
import-scrivener-prose-struck-text-compiled-away-line = { $count ->
    [one] Gjennomstreket tekst i 1 tekst beholdes. Scriveners kompilering utelot den fra boken, men en eksport fra Skribisto skriver den ut.
   *[other] Gjennomstreket tekst i { $count } tekster beholdes. Scriveners kompilering utelot den fra boken, men en eksport fra Skribisto skriver den ut.
}
import-scrivener-prose-compile-deleted-style-printed-line = { $count ->
    [1] Tekst i stilen «{ $example }», som Scriveners kompilering slettet, beholdes som prosa, og en eksport fra Skribisto skriver den ut.
   *[other] Tekst i { $count } stiler som Scriveners kompilering slettet, for eksempel «{ $example }», beholdes som prosa, og en eksport fra Skribisto skriver den ut.
}
import-scrivener-prose-compile-raw-style-printed-line = { $count ->
    [1] Tekst i stilen «{ $example }», som Scriveners kompilering sendte videre som kode for et annet program, beholdes som prosa, og en eksport fra Skribisto skriver den ut.
   *[other] Tekst i { $count } stiler som Scriveners kompilering sendte videre som kode for et annet program, for eksempel «{ $example }», beholdes som prosa, og en eksport fra Skribisto skriver den ut.
}
import-scrivener-prose-left-out-after-document-line = { $count ->
    [one] 1 bit av en skadet fil, etter slutten av den, utelates.
   *[other] { $count } biter av skadede filer, etter slutten av dem, utelates.
}
import-scrivener-prose-left-out-header-or-footer-line = { $count ->
    [one] 1 topptekst eller bunntekst utelates: den er ikke en del av teksten.
   *[other] { $count } topptekster og bunntekster utelates: de er ikke en del av teksten.
}
import-scrivener-prose-left-out-list-label-picture-line = { $count ->
    [one] 1 bilde brukt som punkttegn i en liste utelates.
   *[other] { $count } bilder brukt som punkttegn i lister utelates.
}
import-scrivener-prose-left-out-line = { $count ->
    [one] 1 bit av en fil som ikke hører til noen tekst, utelates.
   *[other] { $count } biter av filer som ikke hører til noen tekst, utelates.
}
import-scrivener-prose-undecodable-characters-line = { $count ->
    [one] 1 tegn kunne ikke dekodes og vises som et erstatningstegn (�) der det sto.
   *[other] { $count } tegn kunne ikke dekodes og vises som erstatningstegn (�) der de sto.
}
import-scrivener-prose-inline-notes-merged-line = { $count ->
    [one] 1 innebygd notat som Scrivener hadde delt der formateringen endret seg, settes sammen igjen.
   *[other] { $count } innebygde notater som Scrivener hadde delt der formateringen endret seg, settes sammen igjen.
}
import-scrivener-prose-inline-notes-joined-across-paragraphs-line = { $count ->
    [1] 1 innebygd notat skrevet ett avsnitt om gangen settes sammen til ett enkelt notat.
   *[other] { $count } innebygde notater skrevet ett avsnitt om gangen settes sammen til notater på flere avsnitt.
}
import-scrivener-prose-blank-notes-dropped-line = { $count ->
    [one] 1 tomt innebygd notat utelates.
   *[other] { $count } tomme innebygde notater utelates.
}
import-scrivener-prose-annotations-became-comments-line = { $count ->
    [one] 1 innebygd merknad ble en kommentar.
   *[other] { $count } innebygde merknader ble kommentarer.
}
import-scrivener-prose-annotation-only-document-kept-line = { $count ->
    [one] 1 dokument inneholdt bare en merknad. Ordene beholdes som prosa.
   *[other] { $count } dokumenter inneholdt bare en merknad. Ordene beholdes som prosa.
}
import-scrivener-prose-preserve-formatting-passages-line = { $count ->
    [one] 1 tekstparti merket «Preserve Formatting» (bevar formatering) kommer inn som vanlig tekst.
   *[other] { $count } tekstpartier merket «Preserve Formatting» (bevar formatering) kommer inn som vanlig tekst.
}
import-scrivener-prose-nested-note-unwrapped-line = { $count ->
    [one] 1 notat inne i en kommentar eller en fotnote beholdes som tekst der det sto.
   *[other] { $count } notater inne i kommentarer eller fotnoter beholdes som tekst der de sto.
}
import-scrivener-prose-style-not-carried-line = { $count ->
    [1] Stilen «{ $example }» har ingen motsvarighet i Skribisto. Teksten i den beholder ordene sine.
   *[other] { $count } stiler, for eksempel «{ $example }», har ingen motsvarighet i Skribisto. Teksten i dem beholder ordene sine.
}
import-scrivener-prose-document-links-dropped-line = { $count ->
    [one] 1 lenke til et annet dokument har ingen plass i Skribisto. Ordene blir stående.
   *[other] { $count } lenker til andre dokumenter har ingen plass i Skribisto. Ordene blir stående.
}
import-scrivener-prose-document-links-kept-line = { $count ->
    [one] 1 lenke til et annet dokument beholdes som en henvisning.
   *[other] { $count } lenker til andre dokumenter beholdes som henvisninger.
}
import-scrivener-prose-other-links-kept-as-text-line = { $count ->
    [one] 1 lenke til et annet prosjekt eller til en fil beholdes som tekst.
   *[other] { $count } lenker til andre prosjekter eller til filer beholdes som tekst.
}
import-scrivener-prose-soft-breaks-split-line = { $count ->
    [one] 1 linjeskift inne i et avsnitt starter nå et eget avsnitt.
   *[other] { $count } linjeskift inne i avsnitt starter nå egne avsnitt.
}
import-scrivener-prose-typed-indents-removed-line = { $count ->
    [one] 1 innrykk skrevet inn i begynnelsen av et avsnitt fjernes.
   *[other] { $count } innrykk skrevet inn i begynnelsen av avsnitt fjernes.
}
import-scrivener-prose-left-indents-not-kept-line = { $count ->
    [one] 1 avsnitt med innrykk fra margen mister innrykket.
   *[other] { $count } avsnitt med innrykk fra margen mister innrykket.
}
import-scrivener-prose-blank-lines-removed-line = { $count ->
    [one] 1 tomt avsnitt mellom to andre fjernes.
   *[other] { $count } tomme avsnitt mellom andre fjernes.
}
import-scrivener-prose-verse-stanza-gaps-lost-line = { $count ->
    [one] 1 tom linje mellom strofene i et dikt fjernes, så strofene går over i hverandre.
   *[other] { $count } tomme linjer mellom strofene i et dikt fjernes, så strofene går over i hverandre.
}
import-scrivener-prose-blank-lines-became-scene-breaks-line = { $count ->
    [one] 1 tom linje mellom avsnitt ble et sceneskift.
   *[other] { $count } tomme linjer mellom avsnitt ble sceneskift.
}
import-scrivener-prose-ornaments-became-scene-breaks-line = { $count ->
    [one] 1 linje med ornamenter ble et sceneskift.
   *[other] { $count } linjer med ornamenter ble sceneskift.
}
import-scrivener-prose-table-first-row-header-line = { $count ->
    [one] 1 tabell begynner nå med en overskriftsrad.
   *[other] { $count } tabeller begynner nå med en overskriftsrad.
}
import-scrivener-prose-table-cells-joined-line = { $count ->
    [one] 1 tabellcelle inneholdt flere avsnitt, som nå er slått sammen til ett.
   *[other] { $count } tabellceller inneholdt flere avsnitt, som nå er slått sammen til ett.
}
import-scrivener-prose-formatting-lost-colour-line = { $count ->
    [one] Tekstfarge fjernes i 1 tekst.
   *[other] Tekstfarge fjernes i { $count } tekster.
}
import-scrivener-prose-formatting-lost-highlight-line = { $count ->
    [one] Fargemarkering fjernes i 1 tekst.
   *[other] Fargemarkering fjernes i { $count } tekster.
}
import-scrivener-prose-formatting-lost-small-caps-line = { $count ->
    [one] Kapitéler vises som vanlige bokstaver i 1 tekst.
   *[other] Kapitéler vises som vanlige bokstaver i { $count } tekster.
}
import-scrivener-prose-formatting-lost-all-caps-line = { $count ->
    [one] Tekst som ble vist med store bokstaver, vises slik den ble skrevet, i 1 tekst.
   *[other] Tekst som ble vist med store bokstaver, vises slik den ble skrevet, i { $count } tekster.
}
import-scrivener-prose-formatting-lost-hidden-line = { $count ->
    [one] Skjult tekst utelates i 1 tekst.
   *[other] Skjult tekst utelates i { $count } tekster.
}
import-scrivener-prose-formatting-lost-line = { $count ->
    [one] Formatering som Skribisto ikke har plass til, fjernes i 1 tekst.
   *[other] Formatering som Skribisto ikke har plass til, fjernes i { $count } tekster.
}
import-scrivener-prose-justified-text-not-kept-line = { $count ->
    [one] 1 blokkjustert avsnitt følger eksportstilen i stedet.
   *[other] { $count } blokkjusterte avsnitt følger eksportstilen i stedet.
}
import-scrivener-prose-comment-colours-lost-line = { $count ->
    [one] 1 kommentar mister fargen sin.
   *[other] { $count } kommentarer mister fargen sin.
}
import-scrivener-prose-control-characters-removed-line = { $count ->
    [one] 1 usynlig kontrolltegn fjernes.
   *[other] { $count } usynlige kontrolltegn fjernes.
}
import-scrivener-prose-note-on-heading-lost-line = { $count ->
    [one] 1 fotnote eller kommentar på en kapitteloverskrift kunne ikke bli stående der: en kommentar flyttes til kapitlets første avsnitt, en fotnote utelates.
   *[other] { $count } fotnoter eller kommentarer på kapitteloverskrifter kunne ikke bli stående der: en kommentar flyttes til kapitlets første avsnitt, en fotnote utelates.
}
import-scrivener-prose-media-file-copied-line = { $count ->
    [one] 1 researchfil som Skribisto ikke kan åpne, ble kopiert ved siden av det nye prosjektet. Notatet sier hvor.
   *[other] { $count } researchfiler som Skribisto ikke kan åpne, ble kopiert ved siden av det nye prosjektet. Notatene deres sier hvor.
}
import-scrivener-prose-media-file-not-carried-line = { $count ->
    [one] 1 researchfil som Skribisto ikke kan åpne, blir værende i Scrivener-prosjektet ditt. Notatet oppgir den.
   *[other] { $count } researchfiler som Skribisto ikke kan åpne, blir værende i Scrivener-prosjektet ditt. Notatene deres oppgir dem.
}

## When the import fails
import-scrivener-error-refused = { $refusal } Ingenting ble skrevet.
import-scrivener-error-question = Prosjektet stiller nå et spørsmål som gjennomgangen ikke besvarte. Velg «Se over importen igjen», og importer deretter. Ingenting ble skrevet.
import-scrivener-error-not-accepted = Før prosjektet kan importeres, må du krysse av i en boks på trinnet Prosjekt, og det ble ikke krysset av. Velg «Se over importen igjen», kryss av i boksen, og importer deretter. Ingenting ble skrevet.
import-scrivener-error-changed = Prosjektet ble endret etter at du gikk gjennom det. Velg «Se over importen igjen», og importer deretter. Ingenting ble skrevet.
import-scrivener-error-choice = Strukturen som ble valgt i gjennomgangen, passer ikke lenger til dette prosjektet. Velg «Se over importen igjen», og importer deretter. Ingenting ble skrevet.

export-title = Eksporter
export-close = Lukk
export-scope-label = Hva
export-format-label = Format
export-style-label = Stil
export-path-label = Fil
export-browse = Bla gjennom…
export-show-non-exportable = Vis ikke-eksporterbare
export-choose-empty = Ingen prosjekter å velge fra.
export-cancel = Avbryt
export-export = Eksporter
export-save-dialog-title = Eksporter til fil
export-section-what = Hva som skal eksporteres
export-section-style = Stil
export-section-destination = Lagringssted
export-custom-selection = Eget utvalg
export-selected-count = { $count } valgt
export-chip-chapters-none = Kapitler: ingen
export-chip-chapters-numbered = Kapitler: nummerert
export-chip-chapters-title = Kapitler: bare tittel
export-chip-chapters-both = Kapitler: nummer + tittel
export-chip-scene-break-glyph = Sceneskift: { $glyph }
export-chip-scene-break-blank = Sceneskift: tom linje
export-chip-scene-break-none = Sceneskift: ingen
export-chip-major-break-glyph = Større skift: { $glyph }
export-chip-major-break-blank = Større skift: tom linje
export-chip-major-break-none = Større skift: ingen
export-chip-major-break-same = Begge skiftnivåene er like
export-chip-spacing-single = Linjeavstand: enkel
export-chip-spacing-onehalf = Linjeavstand: 1½
export-chip-spacing-double = Linjeavstand: dobbel
export-chip-notes-included = Notater tatt med
export-chip-notes-excluded = Notater utelatt
export-format-docx = Word
export-format-odt = LibreOffice
export-format-html = HTML
export-format-markdown = Markdown
export-format-djot = Djot
export-format-text = Tekst
export-format-latex = LaTeX
export-format-epub = EPUB
export-format-pdf = PDF
export-overwrite-title = Vil du erstatte den eksisterende filen?
export-overwrite-text = «{ $name }» finnes allerede. Vil du erstatte den?
export-progress-title = Eksporterer…
export-cancelled = Eksporten ble avbrutt
export-done = Eksporterte { $count ->
    [one] { $count } element
   *[other] { $count } elementer
}
export-open-file = Åpne
export-show-in-folder = Vis i mappe
export-error-title = Kunne ikke eksportere
export-error-details = Detaljer

backup-progress-start = Starter…
backup-progress-retention = Rydder bort gamle sikkerhetskopier…
backup-progress-done = Ferdig
backup-progress-destination = Lagringssted { $i } av { $n }
backup-details = Detaljer
backup-issues-title = Problemer med sikkerhetskopiering
backup-failed-title = Sikkerhetskopiering mislyktes
backup-complete-prune-warning = Sikkerhetskopiering fullført ({ $ok } lagret, { $skipped } allerede oppdatert). Noen gamle sikkerhetskopier kunne ikke fjernes

search = Søk
search-query-placeholder = Søk…
search-replace-placeholder = Erstatt med…
search-replace-toggle = Vis/skjul erstatning
search-replace-all = Erstatt alle
search-opt-case = Skill mellom store og små bokstaver
search-opt-whole-word = Hele ord
search-opt-diacritics = Skill mellom aksenter
search-scope-body = Brødtekst
search-scope-title = Tittel
search-scope-synopsis = Synopsis
search-scope-label = Etikett
search-facet-book = Bøker
search-facet-part = Deler
search-facet-chapter = Kapitler
search-facet-scene = Scener
search-facet-note = Notater
search-facet-folder = Mapper
search-tip-case = Skill mellom store og små bokstaver: store og små bokstaver regnes som forskjellige, så «Elena» og «elena» er separate treff.
search-tip-whole-word = Hele ord: finner bare hele ord, så «katt» blir ikke funnet inne i «kattunge».
search-tip-diacritics = Skill mellom aksenter: bokstaver med aksent regnes som egne bokstaver, så «kafe» gir ikke treff på «kafé».
search-tip-body = Brødtekst: søk i teksten til scener og notater.
search-tip-title = Tittel: søk i titlene til elementene i permen.
search-tip-synopsis = Synopsis: søk i sammendraget til hvert skriveelement.
search-tip-label = Etikett: søk i den korte merknaden som vises under tittelen til et element i permen.
search-tip-comment = Kommentarer: søk i teksten i kommentartråder og svarene på dem. En kommentar handler om manuset og er ikke en del av det, så den har sin egen bryter, og ved erstatning er kommentartreff ikke avkrysset før du krysser dem av.
search-tip-book = Bøker: bokbeholderen og markørene for start og slutt.
search-tip-part = Deler: skillene på delnivå.
search-tip-chapter = Kapitler: kapitler, uansett hvordan prosjektet lagrer dem.
search-tip-scene = Scener: elementene som inneholder teksten din.
search-tip-note = Notater: frie notater.
search-tip-folder = Mapper: vanlige mapper og skillelinjer for organisering.
search-tip-paratext = En tekst som hører til boken, men ikke til historien: et forord, en dedikasjon, et etterord. Telles aldri med i manuset.
search-tip-preserve-case = Bevar store/små bokstaver: erstatningen får samme store og små bokstaver som det som ble funnet, så «ELENA» blir «MARTA» og «Elena» blir «Marta».
search-tip-replace = Erstatt: vis erstatningsfeltet og Erstatt alle.
search-error = Søk mislyktes: { $message }
search-no-matches = Ingen treff
search-count =
    { $matches ->
        [one] { $matches } treff
       *[other] { $matches } treff
    } i { $items ->
        [one] { $items } dokument
       *[other] { $items } dokumenter
    }
search-count-truncated =
    { $matches ->
        [one] { $matches } treff
       *[other] { $matches } treff
    } i { $items ->
        [one] { $items } dokument
       *[other] { $items } dokumenter
    } (bare de første resultatene)
search-occurrences = { $count }
search-field-body = Brødtekst
search-field-title = Tittel
search-field-synopsis = Synopsis
search-field-label = Etikett
search-field-epigraph = Epigraf
search-field-comment = Kommentar
search-field-comment-reply = Svar
search-field-footnote = Fotnote
search-include-in-replace = Ta med i Erstatt alle
search-collapse-all = Fold sammen alle resultater
search-replace-here = Erstatt dette
search-dismiss = Fjern fra resultatene
search-undo-dismiss = Hent tilbake det sist fjernede resultatet
search-replace-nothing = (ingenting)
search-replace-confirm-title = Vil du erstatte alle treff?
search-replace-confirm-text =
    Vil du erstatte { $occurrences ->
        [one] { $occurrences } forekomst
       *[other] { $occurrences } forekomster
    } av «{ $query }» med «{ $replacement }» i { $items ->
        [one] { $items } dokument
       *[other] { $items } dokumenter
    }? Du kan angre dette fra varselet etterpå.
search-replace-done-title = Erstatning fullført
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
       *[other] { $skipped } felt
    } hoppet over (endret siden søket).
search-replace-undo = Angre
search-replace-failed-title = Erstatningen mislyktes
search-replace-skipped-title = Ingenting erstattet
search-replace-skipped-body =
    { $fields ->
        [one] Denne teksten
       *[other] { $fields } av disse tekstene
    } er endret siden søket, så { $fields ->
        [one] den ble
       *[other] de ble
    } ikke rørt. Søk på nytt for å se hvor ordene står nå.
search-replace-undo-failed-title = Angring mislyktes
search-preview = Forhåndsvisning
search-preview-empty = Velg et resultat for å forhåndsvise det her
search-preview-no-prose = Dette treffet har ingen redigerbar tekst
search-preview-prompt = Kjør et søk for å se en forhåndsvisning her.
search-preview-open-search = Søk i prosjektet
search-preview-footnote-prompt = Dette treffet er i teksten til en fotnote. Åpne den i fotnotepanelet for å se og redigere den.
search-preview-open-footnotes = Åpne fotnoter

find-placeholder = Finn i dette dokumentet…
find-count = { $current } av { $total }
find-no-results = Ingen resultater
find-close = Lukk søk
find-previous = Forrige treff (Shift+Enter)
find-next = Neste treff (Enter)
find-opt-case = Skill mellom store og små bokstaver
find-opt-whole-word = Hele ord
find-replace-toggle = Vis/skjul erstatning (Ctrl+R)
find-replace-placeholder = Erstatt med…
find-replace = Erstatt
find-replace-all = Erstatt alle
find-preserve-case = Bevar store/små bokstaver

dict-download-title = Laster ned { $name }…
dict-download-done = { $name } er installert
dict-download-failed = Kunne ikke laste ned { $name }: { $error }
dict-removed = { $name } er fjernet
dict-accept-first = Godta lisensen før du laster ned { $name }
settings-dict-tab-installed = Installert
settings-dict-tab-get-more = Hent flere
settings-dict-tab-personal = Personlige ord
dict-installed-empty = Fant ingen ordbøker på denne datamaskinen ennå.
dict-get-more-search = Søk i språk
dict-system-badge = på systemet ditt
dict-unusable-badge = ubrukelig
dict-download-button = Last ned
dict-downloading = Laster ned…
dict-installed-label = Installert
dict-remove = Fjern
dict-view-license = Vis lisens
dict-approx-size = ~{ $size }
dict-personal-empty = Ingen personlige ord i dette prosjektet ennå.
dict-personal-add = Legg til
dict-personal-placeholder = Legg til et ord…
dict-missing-toast = { $count ->
    [one] Dette prosjektet bruker { $count } ordbok du ikke har installert
   *[other] Dette prosjektet bruker { $count } ordbøker du ikke har installert
}
dict-missing-action = Hent ordbøker
dict-license-title = Lisens for { $name }
dict-license-accept = Godta og last ned
dict-license-cancel = Avbryt
dict-license-close = Lukk
dict-add-button = Legg til ordbok…
dict-add-title = Legg til en ordbok
dict-add-close = Lukk
dict-add-name = Navn
dict-add-name-placeholder = f.eks. Min latinske ordbok
dict-add-code = Språkkode
dict-add-code-placeholder = f.eks. la eller fr-FR-x-custom
dict-add-code-hint = En kort kode du velger selv. Det er denne du velger som språk for et dokument.
dict-add-aff = Affix-fil (.aff)
dict-add-dic = Ordliste (.dic)
dict-add-submit = Legg til
dict-add-cancel = Avbryt
dict-add-name-required = Gi ordboken et navn.
dict-add-code-required = Skriv inn en språkkode.
dict-add-code-invalid = Bruk kun bokstaver, siffer og - _ .
dict-add-code-reserved = Det er koden til en innebygd ordbok. Velg en annen.
dict-add-file-required = Velg en fil.
dict-add-file-missing = Den filen finnes ikke.
dict-add-code-taken = En ordbok for den koden er allerede installert.
dict-add-done = { $name } er lagt til
dict-add-unusable = Disse filene er ikke en brukbar ordbok: { $error }
dict-add-failed = Kunne ikke legge til ordboken: { $error }
inspector-export = Eksporter
inspector-exportable = Ta med i eksport
ctx-number = &Nummerer dette kapittelet
ctx-unnumber = Ikke &nummerer dette kapittelet
inspector-numbering = Nummerering
inspector-numbered = Nummerert
inspector-numbered-tip = Dette kapittelet får sin plass i bokens nummerering. Slå det av for en prolog, en epilog eller et mellomspill.
inspector-numbered-tip-more = Et unummerert kapittel blir stående i boken nøyaktig som før: overskriften, teksten og antall ord røres ikke. Det skrives bare ikke ut noe nummer, og det bruker heller ikke opp et: kapittelet etter en prolog er kapittel én, ikke kapittel to. Å holde det utenfor eksporten er en annen bryter, ovenfor, og den fjerner kapittelet helt fra boken.
inspector-apply-to-children = Bruk på underelementer
inspector-milestone = Dato for milepæl
inspector-milestone-none = Ingen dato
inspector-milestone-clear = Tøm
inspector-tags = Stikkord
inspector-aliases = Også kjent som
inspector-dict-language = Språk
inspector-apply-language-to-children = Bruk språket på underelementer
settings-page-language = Språk
settings-field-dict-language = Språk
dict-tradeoff-hint = Hvert ekstra språk godtar flere ord, så færre feil blir oppdaget.
lang-inherit-hint = Arvet: denne scenen bruker språkene til boken eller prosjektet.
lang-pill-list = Språk
lang-pill-add = Legg til et språk
lang-pill-remove = Fjern { $name }
lang-pill-mute = Slå av stavekontroll for { $name }
lang-pill-unmute = Slå på stavekontroll for { $name }

settings-page-personal-dictionary = Personlig ordbok
settings-user-dict-desc = Ord du legger til her, regnes som riktig stavet overalt i programmet og blir aldri markert igjen.
settings-user-dict-add-placeholder = Legg til et ord…
settings-user-dict-add = Legg til ord
settings-user-dict-search = Filtrer ord
settings-user-dict-count = { $count ->
    [one] { $count } ord
   *[other] { $count } ord
}
settings-user-dict-rename = Gi nytt navn
settings-user-dict-remove = Fjern
settings-user-dict-empty = Ingen ord ennå. Legg til ord stavekontrollen skal overse, eller importer en liste.
settings-user-dict-import = Importer…
settings-user-dict-export = Eksporter…
settings-user-dict-import-tip = Legg til ord fra en ren tekstfil, ett ord per linje. Eksisterende ord beholdes.
settings-user-dict-export-tip = Lagre hele listen i en ren tekstfil, ett ord per linje.
settings-user-dict-txt-filter = Tekstfiler
settings-user-dict-duplicate = Finnes allerede i ordboken
settings-user-dict-imported = Importerte { $count ->
    [one] { $count } ord
   *[other] { $count } ord
} ({ $duplicates } fantes fra før).
settings-user-dict-import-failed = Kunne ikke lese ordlisten: { $error }
settings-user-dict-exported = Lagret { $count ->
    [one] { $count } ord.
   *[other] { $count } ord.
}
settings-user-dict-export-failed = Kunne ikke lagre ordlisten: { $error }

settings-page-text-replacements = Teksterstatning
settings-text-repl-desc = Erstatt forkortelser med hele teksten mens du skriver: «f.eks» blir «for eksempel» i det øyeblikket du skriver et mellomrom eller et skilletegn.
settings-text-repl-enable = Bruk teksterstatninger i dette prosjektet
settings-text-repl-disabled-hint = Slå dette på for å definere forkortelser som utvides mens du skriver.
settings-text-repl-add = Legg til regel
settings-text-repl-trigger-placeholder = Forkortelse
settings-text-repl-replacement-placeholder = Hva det blir til
settings-text-repl-added = La til «{ $trigger }»
settings-text-repl-duplicate = «{ $trigger }» har allerede en regel
settings-text-repl-filter = Filtrer regler
settings-text-repl-count = { $n ->
    [one] 1 regel
   *[other] { $n } regler
}
settings-text-repl-row-enabled = Bruk denne regelen
settings-text-repl-delete = Slett regelen for { $trigger }
settings-text-repl-deleted = Slettet regelen for «{ $trigger }»
settings-text-repl-empty = Ingen regler ennå.
settings-text-repl-csv-filter = CSV-filer
settings-text-repl-import = Importer…
settings-text-repl-export = Eksporter…
settings-text-repl-imported = Importerte { $added }, hoppet over { $skipped }
settings-text-repl-exported = Eksporterte { $n ->
    [one] 1 regel
   *[other] { $n } regler
}

editor-menu-no-suggestions = Ingen forslag
editor-menu-add-to-dictionary = Legg til «{ $word }» i ordboken
editor-menu-add-words-to-dictionary = Legg til de merkede ordene i ordboken
editor-dict-added = La til «{ $word }» i ordboken din.
editor-dict-added-multi = { $count ->
    [one] La til { $count } ord i ordboken din.
   *[other] La til { $count } ord i ordboken din.
}
toast-undo = Angre

undo-superseded-title = Det trinnet er ikke lenger det siste
undo-superseded-body = Noe annet er endret i prosjektet siden. Angre ville tatt tilbake det i stedet, så ingenting ble angret. Ingenting av det dette trinnet gjorde, er reversert. Bruk Rediger ▸ Angre for å gå tilbake gjennom historikken selv.
undo-failed = Angre mislyktes: { $error }

menu-edit = &Rediger
menu-edit-undo = &Angre
menu-edit-redo = Gjør &om
menu-edit-undo-target = &Angre { $target }
menu-edit-redo-target = Gjør &om { $target }
shortcut-name-edit-undo = Angre
shortcut-name-edit-redo = Gjør om
undo-target-typing = skriving
undo-target-project = siste endring i dette prosjektet
prose-history-reset-title = Skrivehistorikken er nullstilt
prose-history-reset-body = { $count ->
    [one] En åpen scene ble gjenopprettet fra en tidligere tilstand, så skrivehistorikken gjelder ikke lenger.
   *[other] { $count } åpne scener ble gjenopprettet fra en tidligere tilstand, så skrivehistorikken deres gjelder ikke lenger.
}
undo-target-trash = flytting til papirkurven
undo-target-restore = gjenoppretting fra papirkurven
undo-target-delete-forever = permanent sletting
undo-target-replace-all = erstatning i hele prosjektet
undo-target-import = dokumentimporten
undo-target-duplicate = duplisering
undo-target-move = flytting
undo-target-merge = sammenslåing av to scener
undo-target-split = deling av en scene
undo-target-promote = endring av type
undo-target-tidy-titles = opprydding i kapitteltitlene
undo-target-import-tags = import av stikkord
undo-target-import-templates = import av notatmaler
undo-target-create = oppretting
undo-target-remove = sletting
undo-target-rename = navneendring
undo-target-edit = den endringen
menu-edit-find = &Finn…
menu-edit-find-next = Finn &neste
menu-edit-find-prev = Finn fo&rrige
menu-edit-replace = Finn og &erstatt…
menu-edit-find-in-project = Finn i &prosjektet…
menu-edit-replace-in-project = Erstatt &i prosjektet…
undo-frozen = Angre («Alltid fremover» er på)
redo-frozen = Gjør om («Alltid fremover» er på)
menu-edit-cut = Klipp u&t
menu-edit-copy = &Kopier
menu-edit-paste = &Lim inn
menu-edit-paste-plain = Lim inn &uten formatering
menu-edit-select-all = &Merk alt
shortcut-name-edit-cut = Klipp ut
shortcut-name-edit-copy = Kopier
shortcut-name-edit-paste = Lim inn
shortcut-name-edit-paste-plain = Lim inn uten formatering
shortcut-name-edit-select-all = Merk alt

titlebar-spellcheck-on = Stavekontroll er på. Klikk for å slå den av (F7)
titlebar-spellcheck-off = Stavekontroll er av. Klikk for å slå den på igjen (F7)
menu-spellcheck = &Kontroller stavemåte
menu-comments = Ko&mmentarer
settings-page-spellcheck = Stavekontroll
settings-group-spellcheck = Stavekontroll
settings-spellcheck-enabled = Kontroller stavemåten mens jeg skriver
settings-spellcheck-hint = Understreker ord som ingen installert ordbok kjenner. Slår du dette av, stanser all kontroll i alle prosjekter til du slår det på igjen. Vil du slutte å kontrollere bare ett språk, fjerner du haken for det i Språk-feltet for prosjektet eller et element.

menu-trash = &Papirkurv
trash-title = Papirkurv
trash-empty-state = Papirkurven er tom.
trash-empty-button = Tøm papirkurven…
trash-restore = &Gjenopprett
trash-restore-to = Gjenopprett &til…
trash-delete-forever = &Slett permanent
trash-restored-ok = { $count } gjenopprettet.
trash-restore-error = Kunne ikke gjenopprette: { $error }
trash-restore-orphaned = Den opprinnelige plassen til dette elementet finnes ikke lenger. Velg hvor det skal gjenopprettes.
trash-restore-no-project = Ingen prosjekter er åpne, så det er ingenting å gjenopprette.
trash-delete-no-project = Ingen prosjekter er åpne, så det er ingenting å slette.
trash-empty-confirm-title = Vil du tømme papirkurven?
trash-empty-confirm-text = { $count ->
    [one] { $count } oppføring i papirkurven blir slettet permanent.
   *[other] { $count } oppføringer i papirkurven blir slettet permanent.
} De forsvinner fra papirkurven for godt. Angre kan fortsatt hente dem tilbake helt til du gjør noe annet.
trash-emptied-title = Papirkurven er tømt
trash-emptied-body = Alt i papirkurven ble slettet permanent.
trash-empty-no-project = Ingen prosjekter er åpne, så det er ingen papirkurv å tømme.
trash-delete-forever-confirm-title = Vil du slette permanent?
trash-delete-forever-confirm-text = { $count ->
    [one] { $count } element blir slettet permanent.
   *[other] { $count } elementer blir slettet permanent.
} De forsvinner fra papirkurven for godt. Angre kan fortsatt hente dem tilbake helt til du gjør noe annet.
trash-deleted-title = Slettet permanent
trash-deleted-body = { $count } permanent slettet.
trash-undo = Angre
trash-restore-picker-title = Gjenopprett til…
trash-restore-to-confirm-title = Vil du gjenopprette her?
trash-restore-to-confirm-text = Vil du gjenopprette «{ $item }» i «{ $destination }»?
trash-restore-picker-restore-here = Gjenopprett her
trash-restore-picker-cancel = Avbryt
trash-restore-picker-empty = Ingen permer ennå. Opprett en først.
trash-banner-title = Dette elementet ligger i papirkurven
trash-banner-description = Det vises ikke i disposisjonen eller i eksport før du gjenoppretter det.
trash-banner-restore = Gjenopprett…
trash-tab-tooltip = I papirkurven

settings-styles-sheet-yes = Ja
settings-styles-sheet-no = Nei
settings-styles-sheet-font = Skrift
settings-styles-sheet-indent = Innrykk i første linje
settings-styles-sheet-para-spacing = Avsnittsavstand
settings-styles-sheet-page = Sidestørrelse
settings-styles-sheet-margins = Marger (T/H/B/V)
settings-styles-sheet-title-page = Bokens tittelside
settings-styles-sheet-heading-language = Språk i overskrifter
settings-styles-sheet-auto = Følger teksten
settings-styles-sheet-digits = Siffer
settings-styles-sheet-direction = Tekstretning
settings-styles-sheet-formats = Formater
settings-styles-sheet-all-formats = Alle

settings-styles-editor-group = Stilredigering
settings-styles-editor-missing = Denne stilen er ikke lenger tilgjengelig.

settings-styles-page-letter = Letter
settings-styles-digits-western = Vestlige (0–9)
settings-styles-digits-eastern-arabic = Østarabiske (٠–٩)
settings-styles-direction-ltr = Venstre til høyre
settings-styles-direction-rtl = Høyre til venstre

format-dock-title = Format
format-panel-empty = Plasser markøren i en scene, notat eller synopsis for å se formateringsalternativer.
format-group-marks = Tekst
format-group-block = Avsnitt
format-group-lists = Lister
format-group-tables = Tabell
format-group-breaks = Sceneskift
format-superscript = Hevet
format-subscript = Senket
format-link = Lenke…
link-dialog-insert-title = Sett inn lenke
link-dialog-edit-title = Rediger lenke
link-dialog-text-label = Tekst
link-dialog-text-placeholder = ordene leseren ser
link-dialog-url-label = Lenker til
link-dialog-url-placeholder = example.com
link-dialog-insert = Sett inn
link-dialog-apply = Bruk
link-dialog-cancel = Avbryt
link-dialog-remove = Fjern lenke
link-scheme-refused = Bare nett- og e-postlenker kan åpnes. { $url } ble ikke åpnet.
menu-format-link = Le&nke…
format-clear = Fjern formatering
format-heading = Overskriftsnivå
format-heading-normal = Normal tekst
format-heading-1 = Overskrift 1
format-heading-2 = Overskrift 2
format-heading-3 = Overskrift 3
format-heading-4 = Overskrift 4
format-heading-5 = Overskrift 5
format-heading-6 = Overskrift 6
format-align-left = Juster venstre
format-align-center = Midtstill
format-direction-rtl = Avsnitt fra høyre mot venstre
format-blockquote = Blokksitat
format-list-bullet = Punktliste
format-list-numbered = Nummerert liste
format-indent = Rykk inn
format-outdent = Rykk ut
format-table-insert = Sett inn tabell
format-table-row-above = Sett inn rad over
format-table-row-below = Sett inn rad under
format-table-col-before = Sett inn kolonne foran
format-table-col-after = Sett inn kolonne etter
format-table-row-delete = Slett rad
format-table-col-delete = Slett kolonne
format-table-remove = Fjern tabell

menu-format-marks-bold = &Fet
menu-format-marks-italic = &Kursiv
menu-format-marks-underline = &Understreket
menu-format-marks-strike = &Gjennomstreket
menu-format-marks-superscript = &Hevet
menu-format-marks-subscript = S&enket
menu-format-marks-clear = Fjern for&matering
menu-format-heading = &Overskrift
menu-format-heading-normal = &Normal tekst
menu-format-heading-1 = Overskrift &1
menu-format-heading-2 = Overskrift &2
menu-format-heading-3 = Overskrift &3
menu-format-heading-4 = Overskrift &4
menu-format-heading-5 = Overskrift &5
menu-format-heading-6 = Overskrift &6
menu-format-alignment = &Justering
menu-format-align-left = Juster &venstre
menu-format-align-center = &Midtstill
menu-format-direction = &Retning
menu-format-direction-auto = &Automatisk
menu-format-direction-ltr = &Venstre mot høyre
menu-format-direction-rtl = &Høyre mot venstre
menu-format-blockquote = &Blokksitat
menu-format-lists = &Lister
menu-format-list-bullet = &Punktliste
menu-format-list-numbered = &Nummerert liste
menu-format-indent = &Rykk inn
menu-format-outdent = Rykk &ut
menu-format-table = &Tabell
menu-format-table-insert = Sett &inn tabell
menu-format-table-2x2 = &2 x 2
menu-format-table-3x3 = &3 x 3
menu-format-table-4x4 = &4 x 4
menu-format-table-row-above = Sett inn rad &over
menu-format-table-row-below = Sett inn rad &under
menu-format-table-col-before = Sett inn kolonne &foran
menu-format-table-col-after = Sett inn kolonne &etter
menu-format-table-row-delete = &Slett rad
menu-format-table-col-delete = Slett &kolonne
menu-format-table-remove = Fje&rn tabell

menu-format-typo-marks = Formatering&smerke
menu-format-typo-nbsp = &Hardt mellomrom
menu-format-typo-nbhyphen = Hard &bindestrek
menu-format-typo-soft-hyphen = &Myk bindestrek
menu-format-typo-nnbsp = &Smalt hardt mellomrom
menu-format-typo-zwsp = Mellomrom med &null bredde
menu-format-typo-word-joiner = &Ordbinder


menu-about = &Om Skribisto…
about-title = Om Skribisto
about-version = Versjon { $version }
about-update-available = Versjon { $version } er tilgjengelig
about-update-see-notes = Se hva som har endret seg
about-update-flathub = Flathub holder denne kopien oppdatert.
about-update-distro = Distribusjonen din holder denne kopien oppdatert.
about-tagline = Et program for å skrive romaner og andre lange skjønnlitterære tekster, skrevet i Rust med verktøysettet Teksilo.
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
window-title-numbered = { $title } - { $app } (vindu { $n })
window-title-empty = { $app }

first-run-window-title = Velkommen
first-run-title = Sett opp { $app }
first-run-body = { $app } holder innstillingene sine atskilt fra Skribisto, så det starter tomt. Innstillingene dine, nylige prosjekter, ordbøker og vindusoppsett kan kopieres over nå.
first-run-from = Kopier fra
first-run-to = Kopier til
first-run-copy-note = Ingenting flyttes eller fjernes. Skribisto beholder alle sine egne innstillinger og fortsetter å virke nøyaktig som før.
first-run-import = Importer innstillinger
first-run-start-fresh = Start på nytt
first-run-import-failed = Noen innstillinger kunne ikke importeres: { $error }

settings-page-punctuation = Standard for tegnsetting
settings-group-punctuation = Smart tegnsetting
settings-page-work-punctuation = Tegnsetting
settings-punctuation-override = Gi dette prosjektet sine egne regler for tegnsetting
settings-punctuation-override-hint = Av: prosjektet følger programinnstillingen. Disse reglene følger med i .skrib-filen, så en medforfatter som åpner filen, skriver med samme typografi.
settings-punctuation-dashes = Gjør -- om til tankestrek og --- om til lang tankestrek
settings-punctuation-ellipsis = Gjør ... om til ellipse
settings-punctuation-quotes = Gjør anførselstegn og apostrofer typografiske
settings-quote-style = Anførselstegn
settings-quote-style-locale = Språkstandard
settings-quote-style-curly = “Doble”
settings-quote-style-curly-single = ‘Enkle’
settings-quote-style-guillemets = «Sjevroner»
settings-quote-style-low-high = „Lave og høye“
settings-punctuation-spacing = Mellomrom før ; : ! ?
settings-punctuation-spacing-hint = Fransk typografi setter et smalt hardt mellomrom før ; ! ? og et fullt foran :. Dette gjelder bare tekst skrevet på fransk. Mellomrommet innenfor sjevroner « » følger med selve sjevronene.
settings-punctuation-sample = Språket ditt gir
settings-punctuation-app-hint = Det alle prosjekter gjør, med mindre de har egne regler under Prosjekt ▸ Tegnsetting.
settings-punctuation-dialogue = Begynn et avsnitt som skrives som «- » med en replikkstrek
settings-punctuation-dialogue-hint = For språk som markerer replikker med strek i stedet for anførselstegn: fransk, spansk, russisk og andre. Den slår bare inn helt i begynnelsen av et avsnitt.

statusbar-go-to = Gå til…
go-to-placeholder = Søk i permen
go-to-no-matches = Ingen elementer samsvarer med søket.
menu-go-to = &Gå til…

settings-page-distraction-free-themes = Distraksjonsfrie temaer
settings-themes-builtin = Innebygde temaer
settings-themes-builtin-badge = Innebygd
settings-themes-user = Temaene mine
settings-themes-editor-group = Rediger tema
settings-themes-editor-empty = Velg et tema under «Temaene mine» for å redigere det.
settings-themes-use = Bruk
settings-themes-duplicate = Dupliser
settings-themes-edit = Rediger
settings-themes-delete = Slett
settings-themes-export = Eksporter…
settings-themes-import = Importer…
settings-themes-copy-suffix = kopi
settings-themes-json-filter = Tema (JSON)
settings-themes-imported = Tema importert
settings-themes-import-failed = Kunne ikke importere det temaet
settings-themes-exported = Tema eksportert
settings-themes-export-failed = Kunne ikke eksportere dette temaet
settings-themes-low-contrast = lav kontrast
settings-themes-low-contrast-band = markeringen skjuler teksten
settings-themes-field-name = Navn
settings-themes-field-paper = Side
settings-themes-field-ink = Tekst
settings-themes-field-general = Bakgrunn
settings-themes-field-widget-text = Tekst i kontrollstripen
settings-themes-field-caret-band = Markering rundt markøren

statusbar-focus-settings = Innstillinger for distraksjonsfri modus
statusbar-focus-manage-themes = Behandle temaer…

comments-title = Kommentarer
comments-document-title = Dette dokumentet
comments-empty-project = Ingen kommentarer i dette prosjektet ennå.
comments-empty-document = Ingen kommentarer til dette dokumentet.
comments-filter-all = Alle
comments-filter-open = Åpne
comments-filter-resolved = Løst
comments-filter-orphaned = Foreldreløse
comments-status-open = Åpen
comments-status-resolved = Løst
comments-status-orphaned = Mistet teksten
comments-orphan-snippet = (den kommenterte teksten er borte)
comments-status-unplaced = Ingen tekstposisjon
comments-unplaced-snippet = (ikke forankret i noen tekst)
comments-reply-count = { $count ->
    [one] { $count } svar
   *[other] { $count } svar
}
comments-menu-resolve = Merk som løst
comments-menu-reopen = Åpne igjen
comments-menu-delete = Slett
comments-sort-document = I dokumentrekkefølge
comments-sort-newest = Nyeste først
comments-sort-reader = Gruppert etter leser
comments-unsigned-author = Usignert
comments-agreement = { $count ->
    [one] { $count } leser
   *[other] { $count } lesere
}
comments-menu-add = Legg til kommentar
comments-menu-add-paragraph = Kommenter dette avsnittet
overview-col-comments = Kommentarer
overview-col-total-comments = Kommentarer totalt
comments-card-placeholder = Skriv en kommentar…
comments-card-unknown-author = Ukjent forfatter
comments-unsigned-toast = Kommentarene dine er usignerte: det er ikke angitt noe navn for deg på denne datamaskinen.
comments-unsigned-action = Angi navnet ditt
comments-card-reply = Svar
comments-card-reply-placeholder = Svar…
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
analysis-scope-book = Analyserer denne boken
analysis-run = Kjør analyse
analysis-stale = Endret siden analysen ble kjørt
analysis-not-run = Ikke analysert ennå.
analysis-running = Leser manuset…
analysis-failed = Analysen kunne ikke fullføres.
analysis-no-scenes = Ingen scener i denne boken ennå.

analysis-shape = Form

analysis-arrivals = Tilkomst
analysis-arrivals-explainer = Hvordan teksten kom inn i dette prosjektet mens det har vært åpent. Det viser hvilken vei tegnene kom, og ingenting om hvem som skrev dem: en forfatter som skriver utkast et annet sted og limer inn, har limt inn, og en som dikterer, har diktert. Her finnes det ikke noe tall å sikte mot.
analysis-arrivals-scope = Hele prosjektet, ikke bare denne boken.
analysis-arrivals-session = Siden du åpnet det. Når prosjektet lukkes, begynner tellingen på nytt.
analysis-arrivals-nothing = Ingen tekst er kommet inn ennå i denne økten.
analysis-arrivals-typed = Skrevet
analysis-arrivals-pasted = Limet inn
analysis-arrivals-dictated = Diktert
analysis-arrivals-imported = Importert
analysis-arrivals-programmatic = Satt inn for deg
analysis-arrivals-count = { $count ->
    [one] { $count } tegn
   *[other] { $count } tegn
}
analysis-arrivals-none = ingen

analysis-words-per-scene = Ord per scene
analysis-median-words = Medianscenen i denne boken er på { $count ->
    [one] { $count } ord.
   *[other] { $count } ord.
}
analysis-median-line = Median: { $count ->
    [one] { $count } ord
   *[other] { $count } ord
}
analysis-dialogue = Dialog
analysis-dialogue-unsupported = Dialog måles ikke for dette språket ennå.

analysis-footnote-words = Ord i fotnoter
analysis-footnote-words-count = { $count ->
    [one] { $count } ord står i denne bokens fotnoter, holdt utenfor totalen for manuset.
   *[other] { $count } ord står i denne bokens fotnoter, holdt utenfor totalen for manuset.
}
analysis-footnote-words-pending = Teller fotnotene…

analysis-ignore-empty = Se bort fra tekster der ingenting er skrevet ennå
analysis-empty-hidden = { $count } { $count ->
        [one] tom tekst
       *[other] tomme tekster
    } skjult.
analysis-all-texts-empty = Alle tekstene i denne boken er fortsatt tomme.

binder-filter-count = { $shown } av { $total } vist
binder-filter-clear = Tøm
binder-filter-none = Ingenting samsvarer med «{ $query }».

settings-paratext-intro = For- og etterteksten et nytt prosjekt kan starte med. Hver struktur hører til en utgivertradisjon, og sidetitlene er skrevet på den tradisjonens eget språk. Gi dem gjerne nye navn når prosjektet er opprettet.
settings-paratext-structures = Strukturer
settings-paratext-broken = Kunne ikke leses
settings-paratext-edit = Rediger
settings-paratext-duplicate = Dupliser
settings-paratext-delete = Slett
settings-paratext-new = Ny struktur
settings-paratext-save = Lagre
settings-paratext-editor-hint = Et navn, sidene som skal opprettes før manuset, og sidene som skal opprettes etter det. Hvor de havner, bestemmer du: de er vanlige elementer når prosjektet finnes.


image-insert = Sett inn &bilde…
image-no-project = Åpne et prosjekt før du setter inn et bilde.
image-choose-title = Velg et bilde
image-filter-label = Bilder
image-large-title = Dette er et stort bilde
image-large-text =
    { $name } er på { $megapixels } megapiksler ({ $width }×{ $height }).
    Beholder du det som det er, lagres originalfilen i prosjektet: den følger
    med i alle sikkerhetskopier og all eksport. Optimaliserer du, lagres i stedet
    en mindre kopi, med høyst 2560 piksler på den lengste siden.
image-large-keep = Behold originalen
image-large-downscale = Optimaliser
image-large-remember = Gjør dette fra nå av, ikke spør igjen
image-not-recorded = Bildet ble lagret, men kunne ikke registreres i prosjektet.
image-describe-title = Beskriv bildet
image-describe-explain = Hva bildet viser, for en leser som ikke kan se det. Det er ikke en del av manuset: det blir aldri telt, søkt i eller eksportert som tekst.
image-describe-placeholder = et fyr mot en grå himmel
image-resize-title = Endre størrelse på bildet
image-resize-explain = En prosentandel av størrelsen det vises i nå. 100 lar det være som det er.
image-resize-invalid = Skriv inn et tall mellom 1 og 1000.
image-menu-describe = &Beskriv bildet…
image-menu-resize = &Endre størrelse på bildet…
image-menu-reset-size = &Opprinnelig størrelse

cover-heading = Omslag
cover-pick = Velg…
cover-change = Bytt…
cover-remove = Fjern
cover-none = Ingen omslag ennå
cover-missing = Bildefilen mangler i prosjektet
cover-choose-title = Velg et omslag
cover-set = Omslaget er valgt.
cover-cleared = Omslaget er fjernet. Bildet ligger fortsatt i prosjektet.
card-label = Etikett
card-exportable = Eksportert
card-numbered = Nummerert
card-created = Opprettet
card-modified = Endret
card-yes = ja
card-no = nei
card-type = Type
card-position = Posisjon
card-children = Underelementer
card-goal = Mål
card-words = Ord
card-synopsis = Synopsis
card-point-of-view = Synsvinkel
card-aliases = Også kjent som

export-orphan-footnotes-title = Fotnoter uten henvisning
export-comments-dropped =
    { $count ->
        [one] Én kommentar kunne ikke plasseres i den eksporterte teksten og ble utelatt.
       *[other] { $count } kommentarer kunne ikke plasseres i den eksporterte teksten og ble utelatt.
    }
export-orphan-footnotes =
    { $count ->
        [one] Én fotnote har ikke lenger noen henvisning i manuset. Teksten i den kommer ikke med i den eksporterte boken.
       *[other] { $count } fotnoter har ikke lenger noen henvisning i manuset. Teksten i dem kommer ikke med i den eksporterte boken.
    }


menu-footnotes = Fot&noter
footnotes-title = Fotnoter
footnotes-insert = Sett inn &fotnote
footnotes-empty = Ingen fotnoter ennå. Plasser markøren i en scene, og bruk + ovenfor eller Ctrl+Alt+F.
footnotes-filter-all = Alle
footnotes-filter-document = Dette dokumentet
footnotes-filter-orphaned = Foreldreløse
footnotes-orphaned = Ingenting viser lenger til denne fotnoten
footnotes-untitled-home = Uten tittel
footnotes-body-placeholder = selve fotnoten
footnotes-insert-tooltip = Sett inn en fotnote ved markøren (Ctrl+Alt+F)
footnotes-actions = Handlinger for fotnote
footnotes-delete = &Slett fotnoten og henvisningen til den
footnotes-no-project = Åpne et prosjekt før du setter inn en fotnote.
footnotes-no-caret = Plasser markøren i teksten til en scene for å sette inn en fotnote der.
footnotes-not-created = Fotnoten kunne ikke legges til i prosjektet.
footnotes-deleted-toast = Fotnote slettet
footnotes-undo-delete = Angre

import-document-title = Importer dokumenter
import-document-close = Lukk
import-document-step-files = Filer
import-document-step-review = Gjennomgang
import-document-step-destination = Plassering
import-document-step-reconcile = Slå sammen
import-document-reconcile-hint = Noen av disse elementene har du allerede. Angi hva som skal skje med hvert av dem.
import-document-reconcile-all-new = Ingenting i denne filen samsvarer med prosjektet ditt: alle elementene legges til som nye.
import-document-duplicate-returns = Disse filene er flere kopier av det samme manuset som kommer tilbake. Importer dem én om gangen: samsvar kan bare finnes mellom én returnert kopi og prosjektet ditt, så flere importert sammen ville legge til en ekstra kopi av boken i stedet for å slå den sammen.
import-document-hunk-added = Ta dette nye avsnittet
import-document-hunk-removed = Forkast dette avsnittet
import-document-hunk-changed = Ta denne omskrivingen
import-document-col-stray-prose = Teksten
import-document-stray-as-paratext = Behold som paratekst
import-document-stray-discard = Forkast teksten
import-document-col-current = I prosjektet ditt
import-document-col-incoming = I denne filen
import-document-col-status = Status
import-document-col-action = Hva som skal gjøres
import-document-status-identical = Lik
import-document-status-editor-edited = De har redigert den
import-document-status-you-edited = Du har redigert den
import-document-status-conflict = Begge har redigert den
import-document-status-different = Forskjellig
import-document-status-new = Ny
import-document-status-missing = Ikke i denne filen
import-document-status-moved = Flyttet
import-document-action-comments-only = Bare kommentarer
import-document-action-take-import = Ta denne versjonen
import-document-action-keep-current = Behold min
import-document-action-create-new = Legg til som ny
import-document-action-ignore = Hopp over
import-document-compare = Sammenlign
import-document-compare-legend = Din versjon sammenlignet med versjonen i denne filen. Skrivebeskyttet.
import-document-compare-close = Lukk
import-document-drop-title = Slipp dokumenter her
import-document-drop-hint = Markdown (.md), Word (.docx), OpenDocument (.odt) og ren tekst (.txt)
import-document-browse = Bla gjennom…
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
import-document-col-breaks = Skift
import-document-col-comments = Kommentarer
import-document-col-footnotes = Fotnoter
import-document-col-epigraph = Epigraf
import-document-col-source = Kilde
import-document-level-rules = Overskriftsnivåer
import-document-level-n = Overskrift { $level }
import-document-add-top-level = Legg til toppnivå
import-document-add-top-level-tooltip = Sett inn en bok over alle analyserte elementer, for kapittelfiler uten bokoverskrift
import-document-destination = Plassering
import-document-destination-hint = Velg en perm eller et element: nye elementer havner inne i en mappe, eller etter en scene.
import-document-destination-empty = Ingen permer ennå. Opprett en først.
import-document-plan-empty = Ingenting å importere ennå.
import-document-pictures = { $count ->
    [1] Ett bilde følger med disse dokumentene.
   *[other] { $count } bilder følger med disse dokumentene.
}
import-document-summary = { $rows ->
    [one] 1 element
   *[other] { $rows } elementer
} · { $breaks ->
    [one] 1 sceneskift
   *[other] { $breaks } sceneskift
}
import-document-back = Tilbake
import-document-cancel = Avbryt
import-document-analyse = Neste
import-document-analysing = Leser dokumenter…
import-document-step-analysing = Leser
import-document-cancel-analysis = Stopp lesingen
import-document-analyse-failed = Dokumentene kunne ikke leses.
import-document-details = Detaljer
import-document-import = Importer
import-document-done = { $count ->
    [one] 1 element importert
   *[other] { $count } elementer importert
}
import-document-undo = Angre
import-diagnostic-file-unreadable = «{ $path }» kunne ikke leses: { $detail }. De andre filene importeres likevel.
import-diagnostic-nested-too-deep = «{ $path }» ble ikke lest: innholdet er nestet mer enn { $limit } nivåer dypt. Ingen virkelige dokumenter kommer i nærheten av det, og å lese det kunne få Skribisto til å krasje, så det er utelatt. De andre filene importeres likevel.
import-diagnostic-archive-too-large = «{ $path }» ble ikke lest: det er en komprimert fil laget for å pakkes ut til noe langt større enn den er, noe som kunne bruke opp minnet og få Skribisto til å krasje, så den er utelatt. De andre filene importeres likevel.
import-diagnostic-lossy-decode = { $count ->
    [one] Ett tegn i «{ $path }» kunne ikke dekodes. Lagre filen på nytt som UTF-8 for å beholde det.
   *[other] { $count } tegn i «{ $path }» kunne ikke dekodes. Lagre filen på nytt som UTF-8 for å beholde dem.
}
import-diagnostic-decoded-from-bom = «{ $path }» ble dekodet som { $detail }, ikke UTF-8.
import-diagnostic-empty-file = «{ $path }» er tom.
import-diagnostic-no-headings = «{ $path }» har ingen overskrifter, så den kommer inn som ett element.
import-diagnostic-unsupported-format = Ingenting kan lese «.{ $detail }»-filer, så «{ $path }» ble hoppet over.
import-diagnostic-front-matter-not-flat = Metadatablokken (front matter) i «{ $path }»: «{ $detail }» er ikke en enkel verdi og ble hoppet over.
import-diagnostic-footnotes-degraded = { $count ->
    [one] Én fotnote i «{ $path }» kommer inn som ren tekst: fotnoter leses ikke fra Markdown.
   *[other] { $count } fotnoter i «{ $path }» kommer inn som ren tekst: fotnoter leses ikke fra Markdown.
}
import-diagnostic-footnote-not-carried = { $count ->
    [one] Én fotnote i «{ $path }» kunne ikke tas med. De andre ble tatt med.
   *[other] { $count } fotnoter i «{ $path }» kunne ikke tas med. De andre ble tatt med.
}
import-diagnostic-raw-html-dropped = { $count ->
    [one] Én blokk med rå HTML i «{ $path }» ble forkastet.
   *[other] { $count } blokker med rå HTML i «{ $path }» ble forkastet.
}
import-diagnostic-nested-break-dropped = { $count ->
    [one] Ett sceneskift inne i et sitat eller en liste i «{ $path }» ble forkastet. Bare et skift på en egen linje beholdes.
   *[other] { $count } sceneskift inne i sitater eller lister i «{ $path }» ble forkastet. Bare et skift på en egen linje beholdes.
}
import-diagnostic-image-not-ingested-format = I «{ $path }» er bildet «{ $detail }» i et format Skribisto ikke kan vise, så det ble utelatt. Bilder i PNG, JPEG, WebP, GIF, TIFF eller BMP kommer inn; lagre det i et av disse formatene og sett det inn for hånd.
import-diagnostic-image-not-ingested-remote = I «{ $path }» ligger bildet «{ $detail }» på nettet. Bilder lastes aldri ned under en import, så det ble utelatt.
import-diagnostic-image-not-ingested-outside = I «{ $path }» ligger bildet «{ $detail }» utenfor dokumentet og mappen det ligger i, så det ble utelatt. Bare bilder som er lagret i dokumentet, eller i dokumentets egen mappe, kommer inn.
import-diagnostic-image-not-ingested-missing = I «{ $path }» ble bildet «{ $detail }» ikke funnet, så det ble utelatt.
import-diagnostic-image-not-ingested-too-large = I «{ $path }» er bildet «{ $detail }» større enn et prosjekt godtar, så det ble utelatt.
import-diagnostic-image-not-ingested-unreadable = I «{ $path }» kunne ikke bildet «{ $detail }» leses som et bilde, så det ble utelatt.
import-diagnostic-image-not-ingested-heading = I «{ $path }» står bildet «{ $detail }» i en overskrift. Hver overskrift blir tittelen på et element i permen, og en tittel kan ikke inneholde bilder, så bildet ble utelatt.
import-diagnostic-image-not-ingested-note = I «{ $path }» står bildet «{ $detail }» i en fotnote, en sluttnote eller en kommentar. Disse beholder ordene sine, men ingen bilder, så det ble utelatt.
import-diagnostic-image-not-ingested-html = I «{ $path }» er bildet «{ $detail }» skrevet som HTML, og en import utelater HTML, så det ble utelatt sammen med den. Skriv det som et Markdown-bilde for å få det med.
import-diagnostic-image-not-ingested-document-full = I «{ $path }» ble bildet «{ $detail }» utelatt: bildene i dette dokumentet er til sammen mer enn én import tar inn. Importer dokumentet i deler, eller sett inn resten for hånd.
import-diagnostic-picture-first-frame-only = I «{ $path }» kommer det animerte bildet «{ $detail }» inn som et stillbilde av den første rammen. Bilder i et prosjekt beveger seg ikke.
import-diagnostic-duplicate-title = «{ $title }» forekommer { $count } ganger. Hvis du har importert disse filene før, blir de duplisert.
import-diagnostic-heading-level-jump = «{ $title }» hopper fra overskriftsnivå { $from } til { $to }; elementet plasseres ett nivå under det overordnede.
import-diagnostic-illegal-combination = «{ $title }» inneholder tekst, men en { $kind } kan ikke inneholde tekst. Importen holdes tilbake til du endrer typen eller utelater elementet. Ellers importeres ingenting i det hele tatt.
import-diagnostic-tracked-changes-flattened = { $path } var midt i en revisjon: { $count ->
    [one] { $count } sporet endring ble godtatt
   *[other] { $count } sporede endringer ble godtatt
}, og slettinger ble utført. Det er den endelige teksten. Vil du heller beholde noe av din egen ordlyd, bruker du Sammenlign i det siste trinnet.
import-diagnostic-tracked-changes-flattened-by = { $path } var midt i en revisjon: { $count ->
    [one] { $count } sporet endring fra { $names } ble godtatt
   *[other] { $count } sporede endringer fra { $names } ble godtatt
}, og slettinger ble utført. Det er den endelige teksten. Vil du heller beholde noe av din egen ordlyd, bruker du Sammenlign i det siste trinnet.
import-diagnostic-text-box-dropped = { $count ->
    [one] { $path } inneholder { $count } tekstboks. Teksten i den står utenfor dokumentets flyt, så det er umulig å si hvor den hører hjemme i et manus. Den importeres ikke.
   *[other] { $path } inneholder { $count } tekstbokser. Teksten i dem står utenfor dokumentets flyt, så det er umulig å si hvor den hører hjemme i et manus. De importeres ikke.
}
import-diagnostic-embedded-object-dropped = { $count ->
    [one] { $path } inneholder { $count } innebygd objekt: et diagram, en formel eller lignende. Ingenting i et manus kan inneholde det.
   *[other] { $path } inneholder { $count } innebygde objekter: et diagram, en formel eller lignende. Ingenting i et manus kan inneholde dem.
}
import-diagnostic-field-flattened = { $count ->
    [one] { $path } inneholder { $count } felt: et sidetall, en kryssreferanse, en dato. Det beholder teksten det sist viste, og oppdateres ikke lenger.
   *[other] { $path } inneholder { $count } felt: et sidetall, en kryssreferanse, en dato. Hvert av dem beholder teksten det sist viste, og oppdateres ikke lenger.
}
import-diagnostic-unknown-style-level = { $path } bruker stilen «{ $detail }», som ser ut som en overskrift, men ikke angir noe nivå. Disse avsnittene importeres som brødtekst i stedet for å bli plassert på et gjettet nivå.
import-diagnostic-comment-unanchored = Kommentaren «{ $detail }» i { $path } kunne ikke knyttes til ordene den gjaldt. Den beholdes på elementet sitt, der du kan flytte den.
import-diagnostic-comment-not-carried = Kommentaren «{ $detail }» i { $path } importeres ikke, fordi ingenting i filen kommer inn som tekst den kan knyttes til. Kopier den fra filen hvis du trenger den.
import-diagnostic-comment-replies-flattened = { $count ->
    [one] { $count } svar i { $path } viste til en kommentar som ikke finnes i filen, så det kommer inn som en egen kommentar.
   *[other] { $count } svar i { $path } viste til en kommentar som ikke finnes i filen, så de kommer inn som egne kommentarer.
}
import-diagnostic-prose-not-verbatim = { $count ->
    [one] { $count } avsnitt i { $path } kunne ikke tas med med formateringen nøyaktig slik den var, så det kommer inn som ren tekst. Sammenlign det med originalen.
   *[other] { $count } avsnitt i { $path } kunne ikke tas med med formateringen nøyaktig slik den var, så de kommer inn som ren tekst. Sammenlign dem med originalen.
}
import-diagnostic-styled-spaces-not-carried = { $count ->
    [one] I { $path } var { $count } strekning med mellomrom understreket eller gjennomstreket, for eksempel en linje til utfylling. Skribisto beholder ingen av strekene på mellomrom alene, så den kommer inn som vanlige mellomrom, eller ikke i det hele tatt der den begynner eller avslutter et avsnitt.
   *[other] I { $path } var { $count } strekninger med mellomrom understreket eller gjennomstreket, for eksempel linjer til utfylling. Skribisto beholder ingen av strekene på mellomrom alene, så de kommer inn som vanlige mellomrom, eller ikke i det hele tatt der de begynner eller avslutter et avsnitt.
}
import-diagnostic-list-nesting-flattened = { $count ->
    [one] { $count } listepunkt i { $path } var nestet mer enn { $limit } nivåer dypt. Det kommer inn på nivå { $limit }, det dypeste Skribisto beholder, med ord og formatering.
   *[other] { $count } listepunkter i { $path } var nestet mer enn { $limit } nivåer dypt. De kommer inn på nivå { $limit }, det dypeste Skribisto beholder, med ord og formatering.
}
import-diagnostic-table-read-as-paragraphs = { $count ->
    [one] { $count } tabell i { $path } har en rad som er langt bredere enn resten, for skjev til å settes opp som et rutenett. Cellene kommer inn som avsnitt, etter hverandre, med ord og formatering.
   *[other] { $count } tabeller i { $path } har en rad som er langt bredere enn resten, for skjeve til å settes opp som et rutenett. Cellene kommer inn som avsnitt, etter hverandre, med ord og formatering.
}
import-diagnostic-spaces-shortened = { $count ->
    [one] { $count } rekke med mellomrom i { $path } var lengre enn { $limit } mellomrom, og kommer inn forkortet til { $limit }. Alle ordene rundt den er med.
   *[other] { $count } rekker med mellomrom i { $path } var lengre enn { $limit } mellomrom, og kommer inn forkortet til { $limit }. Alle ordene rundt dem er med.
}
import-diagnostic-epigraph-not-carried = «{ $title }» innledes med en epigraf, men en { $kind } kan ikke ha en. Sitatet beholdes i stedet øverst i teksten.
import-diagnostic-epigraph-placement-ambiguous = En epigraf står mellom «{ $title }» og «{ $below }» og kan høre til begge. Den ble gitt til «{ $title }», som er der en epigraf vanligvis står.
import-document-diagnostics = { $errors ->
    [0] { $warnings ->
            [one] 1 ting du bør vite
           *[other] { $warnings } ting du bør vite
        }
   *[other] { $errors ->
            [one] 1 fil kunne ikke leses
           *[other] { $errors } filer kunne ikke leses
        }
}
import-document-diagnostics-none = Ingenting å rapportere.

versions-title = Versjoner
versions-scope-synopsis = Synopsis
versions-scope-prose = Tekst
versions-loading = Ser gjennom sikkerhetskopiene dine…
versions-empty = Ingen tidligere versjon av dette ennå
versions-error = Kunne ikke lese sikkerhetskopiene dine. Ingenting er tapt, men listen kan være ufullstendig
versions-did-not-exist = Fantes ikke ennå { $date }
versions-deleted-after = Slettet en gang etter { $date }
versions-unreadable = { $count ->
    [one] { $count } sikkerhetskopi kunne ikke leses
   *[other] { $count } sikkerhetskopier kunne ikke leses
}
versions-thinned = { $count ->
    [one] Eldre versjoner tynnes ut etter hvert. Prosjektets egen historikk har allerede forkastet { $count } tidligere tilstand av denne teksten.
   *[other] Eldre versjoner tynnes ut etter hvert. Prosjektets egen historikk har allerede forkastet { $count } tidligere tilstander av denne teksten.
}
versions-source-backup = Fra en sikkerhetskopi
versions-source-project = Fra prosjektets egen historikk
versions-list-caption = En oppføring per endring, ikke per sikkerhetskopi
versions-pick-a-version = Velg en versjon for å se hva som endret seg
versions-earliest = Den tidligste registrerte versjonen. Det finnes ingenting eldre å sammenligne den med.
versions-no-change = Ingenting endret seg i denne delen av elementet
versions-formatting-only = Bare formateringen er endret her. Ordene er de samme
versions-show-unchanged = Vis uendret
versions-hide-unchanged = Skjul uendret
versions-next-change = Neste endring
versions-near = i nærheten av «{ $text }»
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
versions-pin = Fest sikkerhetskopien denne versjonen kom fra. Automatisk opprydding sletter den aldri
versions-unpin = Løsne sikkerhetskopien denne versjonen kom fra. Automatisk opprydding kan slette den igjen
versions-pinned-only = Vis bare festede versjoner
versions-pin-note = Bare versjoner fra en sikkerhetskopi kan festes. Å feste beholder en fil, og prosjektets egen historikk er ikke en fil.
versions-pinned-empty = Ingenting her er festet ennå
versions-pinned-empty-log = Ingenting her er festet ennå. Bare versjoner fra en sikkerhetskopi kan festes. Å feste beholder en fil, og prosjektets egen historikk er ikke en fil.
versions-range-filter = Vis bare versjoner mellom to datoer
versions-filtered-empty = Ingen versjoner samsvarer med filtrene du har satt
versions-clear-filters = Tøm filtrene
versions-last-30-days = Siste 30 dager
versions-restore-button = Gjenopprett denne versjonen
versions-restore-confirm-title = Vil du erstatte denne teksten med versjonen fra { $date }?
versions-restore-confirm-text = Det du har nå, erstattes med teksten dette elementet hadde { $date }.
versions-restore-confirm-with-comments = Det du har nå, erstattes med teksten dette elementet hadde { $date }. { $count ->
    [one] { $count } kommentar er forankret i den nåværende teksten og kan bli foreldreløs.
   *[other] { $count } kommentarer er forankret i den nåværende teksten og kan bli foreldreløse.
}
versions-restore-confirm-undo-note = Det tas en sikkerhetskopi først, og Ctrl+Z angrer dette i ett trinn.
versions-restored-toast = Gjenopprettet versjonen fra { $date }
versions-undo = Angre
versions-restore-row-gone = Det elementet finnes ikke lenger i dette prosjektet
versions-restore-no-home = Dette elementet har endret type siden da, og den gamle teksten har ingen plass i det
versions-restore-no-safety-copy = Sikkerhetskopien ble ikke tatt, så ingenting ble endret
versions-restore-failed = Gjenopprettingen mislyktes: { $error }
versions-restore-backup-busy = En sikkerhetskopi kjører allerede. Prøv igjen om et øyeblikk
versions-restore-in-backup-file = Du ser på en sikkerhetskopifil. Åpne selve prosjektet for å gjenopprette i det
versions-restore-no-project = Ingen prosjekter er åpne
versions-recreate-button = Hent tilbake…
versions-recreate-picker-title = Hvor skal det plasseres?
versions-recreate-picker-empty = Dette prosjektet har ingen perm å legge det i
versions-recreate-picker-confirm = Hent det tilbake hit
versions-recreate-picker-cancel = Avbryt
versions-recreate-untitled = dette elementet
versions-recreate-confirm-title = Vil du hente tilbake «{ $item }»?
versions-recreate-confirm-text = Det legges til i { $destination }, med teksten det hadde { $date }.
versions-recreate-confirm-undo-note = Angre i meldingen som følger, fjerner det straks igjen.
versions-recreated-toast = «{ $item }» er tilbake i prosjektet ditt
versions-recreated-partial-toast = { $count ->
    [one] «{ $item }» er tilbake, men én av tekstene kunne ikke leses
   *[other] «{ $item }» er tilbake, men { $count } av tekstene kunne ikke leses
}
versions-recreate-already-here = Det elementet finnes allerede i prosjektet ditt
versions-recreate-no-destination = Velg et sted i permen å legge det
versions-recreate-unreadable = Den sikkerhetskopien kunne ikke leses, så ingenting ble lagt til
versions-recreate-failed = Kunne ikke hente det tilbake: { $error }
versions-changed-percent = { $percent } % av dette er endret
versions-hidden-paragraphs = { $count ->
    [one] … { $count } uendret avsnitt …
   *[other] … { $count } uendrede avsnitt …
}

timeline-title = Gå tilbake i tid
timeline-coverage = { $count ->
    [one] { $count } versjon registrert, går tilbake til { $oldest }
   *[other] { $count } versjoner registrert, går tilbake til { $oldest }
}
timeline-loading = Ser gjennom prosjektets historikk…
timeline-empty = Ingen versjon av dette prosjektet er registrert ennå
timeline-no-changes = Ingenting har endret seg siden da
timeline-no-text-changes = Ingen tekst har endret seg siden da
timeline-slider-label = Registrert versjon
timeline-series-name = Prosjektstørrelse
timeline-bars-caption = Hver stolpe er en registrert versjon, like høy som prosjektet var da. Den uthevede er den du ser på.
timeline-bars-caption-periods = For mange versjoner til å vise én og én, så hver stolpe er én { $period }, like høy som prosjektet var ved slutten av den.
timeline-unit-hour = time
timeline-unit-day = dag
timeline-unit-week = uke
timeline-unit-month = måned
timeline-range-filter = Vis bare versjoner mellom to datoer
timeline-range-empty = Ingen versjoner ble registrert mellom de datoene
timeline-open-period = Åpne denne perioden
timeline-show-all = Vis hele historikken
timeline-changed-since = { $count ->
    [one] { $count } element er forskjellig mellom { $date } og prosjektet ditt nå
   *[other] { $count } elementer er forskjellige mellom { $date } og prosjektet ditt nå
}
timeline-kind-added = Skrevet siden
timeline-kind-removed = Ikke lenger i prosjektet
timeline-kind-changed = Redigert siden
timeline-kind-moved = Flyttet siden
timeline-not-yet-written = Dette fantes ikke ennå på det tidspunktet
timeline-no-text-of-its-own = Dette har ingen egen tekst. Det er en overskrift for det som ligger inni
timeline-reader-close = Lukk
timeline-reader-stamp = Slik det var { $date }
timeline-reader-loading = Åpner den registrerte versjonen…
timeline-reader-unreadable = Denne registreringen kunne ikke leses. Sikkerhetskopien kan være flyttet, slettet eller ligge på en disk som ikke er koblet til.
timeline-reader-compared = Slik det var { $date }, sammenlignet med hvordan det er nå
timeline-reader-diff-legend = Gjennomstreket er borte siden da; understreket er lagt til.
timeline-reader-view-label = Hva som skal vises
timeline-reader-view-diff = Endringer
timeline-reader-view-text = Tekst
timeline-reader-deleted = Dette finnes ikke lenger i prosjektet ditt. Du kan lese det og kopiere det ut herfra.
timeline-prose-only-record = Dette punktet kommer fra prosjektets egen historikk, som bare tar vare på tekst, så det som er slettet eller flyttet siden, kan ikke vises. Velg en sikkerhetskopi for det.

settings-sec-extensions = Utvidelser

settings-group-games-forward = Alltid fremover
settings-games-forward-toggle = Spill «Alltid fremover»
settings-games-forward-hint =
    Mens du spiller, kan ingenting du har skrevet tas tilbake: Backspace, Delete,
    Klipp ut, dra og slipp og Angre er slått av i flatene du velger nedenfor.
    Du kan fortsatt skrive, lime inn, formatere og bevege deg fritt. Utkastet bare vokser.
settings-games-session-warning =
    Dette valget gjelder bare økten: det lagres aldri. Når du lukker prosjektet eller
    avslutter Skribisto, slutter spillet alltid, og alt du skrev mens du spilte,
    kan angres igjen fra det øyeblikket.
settings-group-games-scope = Hvor det gjelder
settings-games-in-prose = Teksten i manuset
settings-games-in-synopsis = Synopser
settings-games-scope-hint =
    Kommentarer, fotnoter og titler fryses aldri: det er der du noterer rettelsen
    du nettopp har forbudt deg selv å gjøre.
settings-games-inert-warning =
    «Alltid fremover» er på, men gjelder ingenting. Kryss av for minst én flate ovenfor,
    ellers endrer det ingenting mens du skriver.

games-title = Skrivelek
games-forward-name = Alltid fremover
games-forward-blurb =
    Skriv utkast uten å ta noe tilbake. Sletting, utklipping og angring er slått av
    mens du spiller. Skriv neste setning i stedet for å rette den forrige.
games-forward-playing = Spiller: sletting er slått av
games-forward-idle = Spiller ikke
games-session-note = Slutter når du lukker prosjektet.
games-scope-prose-and-synopsis = Gjelder teksten og synopsene dine.
games-scope-prose = Gjelder teksten din.
games-scope-synopsis = Gjelder synopsene dine.
games-scope-nothing = Gjelder ingenting ennå. Velg en flate i Innstillinger.
games-settings-link = Innstillinger for skrivelek…
statusbar-games-forward = Alltid fremover
statusbar-games-forward-tooltip =
    «Alltid fremover» er på: sletting, utklipping og angring er slått av mens du
    skriver utkast. Klikk for å slutte å spille.
goal-progress-words = { $g ->
    [one] { $count } av { $goal } ord
   *[other] { $count } av { $goal } ord
}
goal-progress-characters = { $g ->
    [one] { $count } av { $goal } tegn
   *[other] { $count } av { $goal } tegn
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
overview-excluded-from-export = Utelatt fra eksporten, så det teller ikke med i noen total

goal-subtree-total-words = { $count ->
    [one] { $items } mål her inne utgjør til sammen { $words } ord
   *[other] { $items } mål her inne utgjør til sammen { $words } ord
}
goal-subtree-total-characters = { $count ->
    [one] { $items } mål her inne utgjør til sammen { $words } tegn
   *[other] { $items } mål her inne utgjør til sammen { $words } tegn
}

new-work-goal-unit = Tell i
new-work-goal-unit-hint = Brukes for alle mål for ord eller tegn i dette prosjektet. Kan endres senere.

settings-group-goal-unit = Mål
settings-goal-unit-switch-title = Bytt telleenhet
settings-goal-unit-switch-text = Målene og milepælene som allerede er satt for dette prosjektet, ble angitt i { $from }. Å bytte til { $to } regner dem ikke om, så alle eksisterende tall leses nå som { $to }.
settings-goal-unit-switch-informative = Ingenting går tapt. Bytt tilbake når som helst for å få den opprinnelige tolkningen igjen, og oppdater deretter målene du vil beholde.
settings-goal-unit-switch-confirm = Bytt

goal-distribute-action = Fordel…
distribute-title = Fordel målet
distribute-empty = Det er ingenting her inne å fordele målet på.
distribute-weight = Fordel
distribute-weight-length = Etter lengde
distribute-weight-rows = Etter antall deler
distribute-weight-even = Jevnt
distribute-overwrite = Erstatt mål som allerede er satt
distribute-col-item = Element
distribute-col-current = Nå
distribute-col-proposed = Etter
distribute-total = Til sammen blir dette { $total }, mot et mål på { $goal }.
distribute-over-budget = Målene som allerede er satt her inne, utgjør { $over } mer enn beholderens eget. Øk målet for beholderen, senk deres, eller erstatt dem.
distribute-apply = Fordel
distribute-cancel = Avbryt

milestone-target-gone = Mål slettet
milestone-no-target = Intet mål
milestone-add = Legg til
milestone-add-label = Hva som skal nås

pace-summary-title = Hvor boken står
pace-summary-remaining = { $words } igjen
pace-summary-open = Åpne planen
pace-summary-close = Lukk
pace-summary-dont-show = Ikke vis dette ved åpning
pace-summary-menu = Skriveplan…

menu-help-topics = &Hjelpeemner
menu-help-shortcuts = Hurtig&taster…
menu-help-website = Skribisto på &nettet
menu-help-report = &Rapporter et problem…
menu-help-check-updates = Se etter opp&dateringer
menu-command-palette = &Kommandopalett…

learn-help-topics = Hjelpeemner
learn-shortcuts = Hurtigtaster
learn-website = Skribisto på nettet

help-window-title = Hjelp
help-filter-topics = Filtrer emner
help-no-matching-topic = Ingen emner samsvarer.
help-topic-missing = Dette emnet er ikke lenger tilgjengelig.
help-back = Tilbake
help-not-translated = Denne siden er ikke oversatt ennå, så den vises på engelsk.

help-section-getting-started = Komme i gang
help-section-writing = Skriving
help-section-reviewing = Gjennomgang
help-section-exchanging = Få tekst inn og ut
help-section-keeping = Ta vare på arbeidet ditt
help-section-extensions = Utvidelser

help-topic-getting-started = Ditt første prosjekt
help-topic-writing-model = Hvordan en bok settes sammen
help-topic-drafts-and-old-versions = Ta vare på et eldre utkast
help-topic-goals-and-pace = Mål og tempo
help-topic-comments = Kommentarer
help-topic-round-trip = Sende boken til en leser
help-topic-export = Eksportere
help-topic-import-documents = Importere dokumenter
help-topic-import-projects = Hente inn et helt prosjekt
help-topic-import-scrivener = Fra Scrivener
help-topic-backups-and-versions = Sikkerhetskopier og versjoner

help-shortcuts-title = Hurtigtaster
help-shortcuts-filter = Filtrer hurtigtaster
help-shortcuts-no-matches = Ingen hurtigtaster samsvarer.
help-shortcuts-rebind = Endre hurtigtaster…
help-shortcuts-close = Lukk

command-palette-placeholder = Skriv inn en kommando

shortcut-name-binder-duplicate = Dupliser
shortcut-name-comments-add = Legg til kommentar
shortcut-name-comments-add-paragraph = Kommenter avsnitt
shortcut-name-spellcheck-toggle = Kontroller stavemåte
shortcut-name-editor-tab-close = Lukk fane
shortcut-name-editor-tab-pin = Fest eller løsne fane
shortcut-name-editor-save = Lagre
shortcut-name-work-export = Eksporter…
shortcut-name-work-new = Nytt prosjekt
shortcut-name-work-open = Åpne prosjekt
shortcut-name-window-new = Nytt vindu
shortcut-name-work-close = Lukk prosjekt
shortcut-name-work-import-scrivener = Importer Scrivener-prosjekt…
shortcut-name-work-import-scrivener-report = Vis rapporten fra Scrivener-importen
shortcut-name-app-settings = Innstillinger
shortcut-name-app-quit = Avslutt
shortcut-name-editor-insert-footnote = Sett inn fotnote
shortcut-name-format-scene-break = Sett inn sceneskift
shortcut-name-format-major-scene-break = Sett inn større sceneskift
shortcut-name-format-link = Sett inn lenke
shortcut-name-format-mark-nbsp = Sett inn hardt mellomrom
shortcut-name-format-mark-nbhyphen = Sett inn hard bindestrek
shortcut-name-format-mark-nnbsp = Sett inn smalt hardt mellomrom
shortcut-name-format-mark-zwsp = Sett inn mellomrom med null bredde
shortcut-name-go-next = Neste
shortcut-name-go-prev = Forrige
shortcut-name-go-to = Gå til
shortcut-name-outline-toggle = Vis/skjul disposisjonen
shortcut-name-preview-toggle = Vis/skjul forhåndsvisningsfeltet
shortcut-name-view-fullscreen = Slå fullskjerm av/på
shortcut-name-view-focus-mode = Slå distraksjonsfri modus av/på
shortcut-name-editor-size-increase = Større tekst
shortcut-name-editor-size-decrease = Mindre tekst
shortcut-name-editor-size-reset = Tilbakestill tekststørrelse
shortcut-name-editor-find = Finn
shortcut-name-editor-replace = Erstatt
shortcut-name-editor-find-next = Neste treff
shortcut-name-editor-find-prev = Forrige treff
shortcut-name-search-show = Søk i prosjektet
shortcut-name-search-replace = Erstatt i prosjektet
shortcut-name-outline-open-to-side = Åpne ved siden av
shortcut-name-help-topics = Hjelpeemner
shortcut-name-help-shortcuts = Hurtigtaster
shortcut-name-help-website = Skribisto på nettet
shortcut-name-help-report = Rapporter et problem
shortcut-name-help-check-updates = Se etter oppdateringer
update-check-behind = Versjon { $version } er tilgjengelig.
update-check-current = Du kjører den nyeste versjonen.
update-check-failed = Kunne ikke se etter en nyere versjon. { $error }
shortcut-name-command-palette = Kommandopalett
help-section-reference = Hva ting er

help-concept-scene-break = Sceneskift
help-concept-major-scene-break = Større sceneskift
help-concept-find-in-prose = Finn i teksten
help-concept-story-bible = Historiebibel
help-concept-goal-unit = Ord eller tegn
help-concept-goal-progress = Fremdrift
help-concept-manuscript-words = Hva som regnes som manus
help-concept-exportable = Utelatt fra eksporten
help-concept-distribute = Fordele et mål
help-concept-subtree-total = Hva målene her inne utgjør til sammen
help-concept-milestone = Milepæl
help-concept-pace-plan = Tempoplan

help-concept-tag = Stikkord
help-concept-label = Etikett
help-concept-point-of-view = Synsvinkel
help-concept-epigraph = Epigraf
help-concept-footnote = Fotnote
help-concept-chapter-mode = Kapittelform
help-concept-comment = Kommentar
help-concept-backup = Sikkerhetskopi
help-concept-version = Versjon
help-concept-trash = Papirkurv
help-concept-spellcheck = Stavekontroll
help-concept-search-replace = Søk og erstatt
help-concept-note-template = Notatmal
help-concept-story-bible-entry = Oppføring i historiebibelen
help-concept-text-replacement = Teksterstatning
help-concept-smart-punctuation = Smart tegnsetting
help-concept-export-style = Eksportstil
help-concept-round-trip-marks = Returmarkører

epigraph-mark-attribution = Kildelinje
epigraph-mark-attribution-tip = Merk linjen markøren står i som sitatets kilde, så den skrives ut som en kildeangivelse
margin-lane-name = Margmerker
margin-lane-provider-comments = Kommentarer
margin-lane-provider-story-bible = Oppføring i historiebibelen
margin-lane-provider-story-bible-hint = I visningen I teksten for et notat: hvor oppføringen er nevnt, og scenene som fortelles gjennom dens øyne. Markerer også navnene i selve teksten.
margin-lane-mark-point-of-view = Fortalt herfra
margin-lane-mark-named-here = Navngitt her
margin-lane-provider-comments-hint = Der en kommentar er knyttet til teksten
margin-lane-provider-search = Søketreff
margin-lane-provider-search-hint = Alle treff for det du sist søkte etter, uansett hvor i dokumentet de er
margin-lane-provider-boundaries = Hvor hvert dokument starter
margin-lane-provider-boundaries-hint = En strek øverst i hver scene i en strøm, så du ser hvor du er
margin-lane-provider-spelling = Stavemåte
margin-lane-provider-spelling-hint = Der stavekontrollen har markert et ord. Av med mindre du ber om det: det viser en maskins mening om teksten din
margin-lane-spelling = { $word }, mulig feilstaving
margin-lane-search-hit = { $text }, treff { $index } av { $total }
margin-lane-search-current = { $text }, treff { $index } av { $total }, det du står på
margin-lane-boundary = Starten på { $title }
margin-lane-boundary-untitled = et dokument uten tittel
settings-page-margin-lane = Margmerker
settings-desc-margin-lane = Stripen ved siden av rullefeltet, og hva den viser
settings-margin-lane-enabled = Vis margfeltet
settings-margin-lane-enabled-hint = Den samme bryteren som Vis ▸ Margmerker.
settings-margin-lane-enabled-more = Det sier aldri at noe er galt: det viser deg hvor ting er, og overlater til deg hva du gjør med det.
settings-group-margin-lane-marks = Hva det markerer
settings-margin-lane-no-providers = Ingenting markerer feltet ennå.
settings-group-margin-lane-texture = Dialogtekstur
settings-margin-lane-texture = Vis dialogteksturen
settings-margin-lane-texture-hint = Én stolpe per avsnitt: hvor langt det er, og hvor mye av det som er replikker.
settings-margin-lane-texture-more = Måles ikke for språk uten en kartlagt konvensjon, der stolpen bare viser lengden.
settings-group-margin-lane-surfaces = Hvor det vises
settings-margin-lane-surface-editor = Tekstredigering
settings-margin-lane-surface-stream = Strømmer
settings-margin-lane-surface-search-preview = Søkeforhåndsvisning
menu-margin-lane = &Margmerker

status-preset-drafting = Utkastskriving
status-preset-passes = Revisjonsrunder
status-preset-plume = Plume Creator
status-todo = Å gjøre
status-draft = Utkast
status-revised = Revidert
status-final = Endelig
status-outline = Disposisjon
status-first-edit = 1. revisjon
status-second-edit = 2. revisjon
status-done = Ferdig
status-plume-draft-1 = 1. utkast
status-plume-draft-2 = 2. utkast
status-plume-draft-3 = 3. utkast
status-plume-edit-1 = 1. revisjon
status-plume-edit-2 = 2. revisjon
status-plume-edit-3 = 3. revisjon
status-plume-proofread = Korrekturlest
status-plume-finished = Fullført
new-work-statuses = Arbeidsflyt
new-work-statuses-hint = Stadiene du fører en scene gjennom. Du kan gi dem nye navn, endre rekkefølgen eller legge til flere når som helst.
status-none = Ingen status
inspector-status = Status
overview-col-status = Status
overview-status-mixed = delene under er ikke like
help-concept-status = Status
status-completion-title = Hvor boken står
status-completion-headline = { $done } av { $total } scener er ferdige
status-completion-empty = Ingen scener ennå. Dette leser manuset, så det fylles ut mens du skriver.
status-completion-open = Hvor boken står…

settings-page-statuses = Statuser
settings-desc-statuses = Gi nytt navn til, endre rekkefølgen på, legg til og fjern stadiene du fører en scene gjennom
settings-statuses-add = Legg til status
settings-statuses-add-placeholder = Nytt statusnavn
settings-statuses-desc =
    Stadiene en scene går gjennom, i rekkefølge, fra minst ferdig øverst til mest
    ferdig nederst. Den rekkefølgen er det «mindre ferdig enn» betyr overalt ellers
    i programmet, så ordne dem slik arbeidsprosessen din faktisk går.
settings-statuses-apply-preset = Bruk en forhåndsinnstilling…
settings-statuses-preset-applied = { $added ->
    [one] 1 status lagt til
   *[other] { $added } statuser lagt til
}
settings-statuses-preset-refused = Dette prosjektet har allerede en statusstige. Forhåndsinnstillinger fyller bare en tom stige. Slett trinnene du ikke vil ha, først.
settings-statuses-duplicate = «{ $name }» står allerede på denne stigen
settings-statuses-added = La til «{ $name }»
settings-statuses-move-up = Flytt opp (mindre ferdig)
settings-statuses-move-down = Flytt ned (mer ferdig)
settings-statuses-details-placeholder = Hva dette stadiet betyr (valgfritt)
settings-statuses-delete = Slett «{ $name }»
settings-statuses-delete-in-use = { $count ->
    [one] Slett «{ $name }»: 1 element har denne statusen og mister den
   *[other] Slett «{ $name }»: { $count } elementer har denne statusen og mister den
}
settings-statuses-deleted = Slettet «{ $name }»
settings-statuses-deleted-in-use = { $count ->
    [one] Slettet «{ $name }». 1 element har ikke lenger noen status.
   *[other] Slettet «{ $name }». { $count } elementer har ikke lenger noen status.
}
settings-statuses-empty-title = Ingen arbeidsflyt ennå
settings-statuses-empty-body = En status forteller hvor langt en scene er kommet. Legg til en ovenfor, eller start fra en forhåndsinnstilling.
status-category-planned = Planlagt
status-category-drafting = Skrives
status-category-needs-work = Må bearbeides
status-category-revised = Revidert
status-category-final = Endelig

## Long-operation progress (shared::progress_text). The backend reports phase
## codes it cannot translate; these are their labels. `progress-percent` is the
## toast body on its own, `progress-percent-with-label` with the phase after it.
progress-percent = { $percent } %
progress-percent-with-label = { $percent } % · { $label }
progress-import-opening = Åpner prosjektet…
progress-import-outline = Leser disposisjonen…
progress-import-converting = Konverterer kapitler og scener…
progress-import-writing = Skriver .skrib-filen…
progress-import-done = Ferdig
progress-import-structure = Finner ut av strukturen…
progress-export-start = Starter…
progress-export-writing = Skriver…
progress-project-reading = Leser prosjektet…
# Settings: the open project's section title, "Work: <its title>".
settings-sec-work-titled = Prosjekt: { $title }

## Project-import notices (shared::import_warnings::notices): one line of the
## Plume / Manuskript warnings list each. $file, $folder, $title, $name, $field
## and $error are data (file names, titles, a parser's own message) and stay as
## they are. $container is "single-file" or "folder"; $kind is "scene", "note",
## "synopsis" or "title".
import-notice-flattened-rich-texts = { $count ->
    [one] Én tekst var nøstet dypere enn et Skribisto-prosjekt kan romme; ordene ble beholdt som ren tekst, uten formatering.
   *[other] { $count } tekster var nøstet dypere enn et Skribisto-prosjekt kan romme; ordene ble beholdt som ren tekst, uten formatering.
}
import-notice-free-text-badges = { $count ->
    [one] Ett sceneskilt ser ut som fritekst snarere enn en liste over statuser, så det ble beholdt som elementets etikett i stedet for å bli et stikkord.
   *[other] { $count } ulike sceneskilt ser ut som fritekst snarere enn en liste over statuser, så de ble beholdt som elementenes etiketter i stedet for å bli stikkord.
}
import-notice-separator-text-dropped = Skillelinjen «{ $name }» inneholdt tekst, som det ikke er plass til i et Skribisto-prosjekt; teksten ble ikke importert.
import-notice-separator-without-scene = Skillelinjen «{ $name }» hadde ingen scene foran seg i kapitlet sitt å knyttes til, så den ble ikke importert.
import-notice-chapter-without-scenes = Kapitlet «{ $name }» har ingen scener, så skillelinjene i det ble ikke importert.
import-notice-rich-text-unconverted = Noe formatert tekst kunne ikke konverteres ({ $error }); den ble ikke importert.
import-notice-unresolved-cross-link = En lenke til oppføring nr. { $number } i persongalleriet kunne ikke følges: oppføringen mangler eller lå i papirkurven.
import-notice-read-copy = Prosjektets kopi { $container ->
    [single-file] som én fil
   *[folder] som mappe
} ble lest.
import-notice-read-copy-dated = Prosjektets kopi { $container ->
    [single-file] som én fil
   *[folder] som mappe
} (sist endret { $day }) ble lest.
import-notice-original-format = Dette prosjektet er i Manuskripts opprinnelige format fra 2016. Det ble lest i sin helhet; ingenting i det ble endret.
import-notice-newer-single-file-beside = «{ $file }» er nyere enn denne mappen ({ $file_day } mot { $folder_day }). Manuskript lar den gamle mappen ligge når et prosjekt går over til enkeltfilmodus, så denne mappen kan være en forlatt kopi; importerer du .msk-filen i stedet, leses den nyere.
import-notice-pickle-skipped = «{ $file }» ble ikke lest. Det er en Python-pickle, og å laste inn en slik fra en prosjektfil var et sikkerhetshull i selve Manuskript (CVE-2021-35196); innstillingene den inneholder, er visningsvalg, ikke skriving.
import-notice-file-unreadable = «{ $file }» kunne ikke leses ({ $error }) og ble hoppet over; alt annet ble importert.
import-notice-not-utf8 = «{ $file }» er ikke gyldig UTF-8; tegnene som ikke kunne leses, ble erstattet i stedet for å stanse importen.
import-notice-table-unreadable = «{ $file }» kunne ikke leses ({ $error }); innholdet ble ikke importert.
import-notice-outline-missing = «{ $file }» mangler, så dette prosjektet har ikke noe manus å importere.
import-notice-outline-unreadable = «{ $file }» kunne ikke leses ({ $error }); ikke noe manus ble importert.
import-notice-named-none-row = «{ $file }» begynner med en navngitt rad («{ $name }») der Manuskript har sin tomme «none»-oppføring. Den ble utelatt, så radene etter den beholder numrene manuset bruker.
import-notice-unknown-field = «{ $file }» har et felt som denne importen ikke kjenner: «{ $field }». Det ble ikke importert.
import-notice-colorless-labels = { $count ->
    [one] «{ $file }» har en linje uten farge. Den ble importert uten; Manuskript slutter å laste inn prosjektet ved en slik linje.
   *[other] «{ $file }» har { $count } linjer uten farge. De ble importert uten; Manuskript slutter å laste inn prosjektet ved en slik linje.
}
import-notice-settings-unreadable = «{ $file }» kunne ikke leses ({ $error }); prosjektets språk ble ikke importert.
import-notice-revisions-unreadable = «{ $file }» kunne ikke leses ({ $error }); prosjektets historikk ble ikke importert, men ingenting annet ble berørt.
import-notice-foreign-files = { $count ->
    [one] Dette prosjektet inneholder en fil som ikke hører til Manuskript-formatet og ikke ble importert: { $files }.
   *[other] Dette prosjektet inneholder { $count } filer som ikke hører til Manuskript-formatet og ikke ble importert: { $files }.
}
import-notice-foreign-files-more = Dette prosjektet inneholder { $count } filer som ikke hører til Manuskript-formatet og ikke ble importert: { $files } og { $more ->
    [one] én til
   *[other] { $more } til
}.
import-notice-not-a-character-file = «{ $file }» er ikke en Manuskript-personfil og ble utelatt.
import-notice-not-a-text-item = «{ $file }» er ikke et Manuskript-tekstelement og ble utelatt fra disposisjonen.
import-notice-missing-blank-line = «{ $file }» har ingen tom linje etter toppteksten. Det siste feltet ble beholdt, selv om Manuskript mister det.
import-notice-folder-without-metadata = «{ $folder }» har ingen { $file }, så den ble importert som en vanlig mappe oppkalt etter katalogen. Manuskript hopper over en slik katalog og alt i den.
import-notice-rows-given-ids = { $count ->
    [one] Én rad i disposisjonen hadde ingen egen ID og fikk en. Manuskript dropper en slik rad, eller slutter å laste inn prosjektet når den er en mappe.
   *[other] { $count } rader i disposisjonen hadde ingen egen ID og fikk en. Manuskript dropper en slik rad, eller slutter å laste inn prosjektet når den er en mappe.
}
import-notice-plots-unreadable = «{ $file }» kunne ikke leses ({ $error }); handlingslinjene ble ikke importert.
import-notice-world-unreadable = «{ $file }» kunne ikke leses ({ $error }); verdensoppføringene ble ikke importert.
import-notice-world-not-opml = «{ $file }» er ikke et OPML-dokument; verdensoppføringene ble ikke importert.
import-notice-text-flattened = Teksten i «{ $title }» er nøstet dypere enn et Skribisto-prosjekt kan romme; ordene ble beholdt som ren tekst, uten formatering.
import-notice-text-unconverted = Teksten i «{ $title }» kunne ikke konverteres ({ $error }); den ble beholdt nøyaktig slik den var skrevet.
import-notice-text-without-file-name = { $kind ->
    [scene] Sceneteksten
    [note] Notatteksten
    [synopsis] Synopsisen
   *[title] Tittelen
} i «{ $title }» kunne ikke få et filnavn, så den ble ikke importert.
import-notice-unknown-label = «{ $title }» har etikett { $index }, som ikke finnes i prosjektets etikettliste. Elementet ble importert uten etikett.
import-notice-unknown-status = «{ $title }» har status { $index }, som ikke finnes i prosjektets statusliste. Elementet ble importert uten status.
import-notice-unknown-point-of-view = «{ $title }» fortelles gjennom en person som ikke er i prosjektets persongalleri. Synsvinkelen ble ikke satt.
import-notice-unresolved-references = Henvisningene i teksten til «{ $title }» kunne ikke løses opp ({ $error }); teksten ble beholdt slik den ble konvertert, henvisningsmarkører inkludert.
import-notice-custom-icons = { $count ->
    [one] Én rad hadde et egendefinert ikon. Skribisto gir hver rad ikonet for typen sin, så ikonet ble ikke importert; ingenting annet ved raden ble endret.
   *[other] { $count } rader hadde et egendefinert ikon. Skribisto gir hver rad ikonet for typen sin, så ikonene ble ikke importert; ingenting annet ved radene ble endret.
}
import-notice-character-colors = { $count ->
    [one] Én person hadde en farge ved navnet sitt. Skribisto fargelegger stikkord, ikke rader, så fargen ble ikke importert; selve personen ble det.
   *[other] { $count } personer hadde en farge ved navnet sitt. Skribisto fargelegger stikkord, ikke rader, så fargene ble ikke importert; selve personene ble alle importert.
}
import-notice-versions-kept = { $found ->
    [one] Dette prosjektet hadde registrert én tidligere versjon
   *[other] Dette prosjektet hadde registrert { $found } tidligere versjoner
}; { $kept ->
    [one] én ble beholdt
   *[other] { $kept } ble beholdt
}, tynnet ut slik Skribisto tynner ut sin egen historikk. De ligger i Versjoner-panelet.
import-notice-versions-unplaced = { $count ->
    [one] Én tidligere versjon hører til en rad som ikke lenger finnes i prosjektet, og kunne ikke plasseres.
   *[other] { $count } tidligere versjoner hører til en rad som ikke lenger finnes i prosjektet, og kunne ikke plasseres.
}

## Native file dialogs: the window title, and the label of the file-type filter.
dialog-open-work-title = Åpne Skribisto-prosjekt
dialog-save-as-file-title = Lagre som én .skrib-fil
dialog-save-as-folder-title = Velg en overordnet mappe for prosjektet
dialog-manuskript-file-title = Velg et Manuskript-prosjekt
dialog-manuskript-folder-title = Velg en Manuskript-prosjektmappe
dialog-plume-file-title = Velg et Plume Creator-prosjekt
dialog-filter-skrib = Skribisto-prosjekt
dialog-filter-manuskript = Manuskript-prosjekt
dialog-filter-plume = Plume Creator-prosjekt
dialog-filter-hunspell-aff = Hunspell-affiksfil (.aff)
dialog-filter-hunspell-dic = Hunspell-ordbok (.dic)
dialog-filter-export = Eksportert fil
# Help ▸ Keyboard shortcuts / Settings ▸ Keymap / command palette.
shortcut-name-view-margin-lane = Vis/skjul margmerker

## The built-in distraction-free themes' names (distraction_free::theme).
df-theme-paper = Papir
df-theme-sepia = Sepia
df-theme-night = Natt
df-theme-high-contrast = Høy kontrast
df-theme-midnight = Midnatt

## Welcome ▸ Examples: the line under a bundled example's title.
welcome-example-byline = av { $author }

## Spoken by a screen reader only.
# Welcome window: the name of the section tab bar.
welcome-sections-a11y = Velkomstseksjoner
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
margin-lane-a11y-mark = { $label }, { $percent } prosent ut i teksten
margin-lane-a11y-marks = { $label }, { $count } merker, { $percent } prosent ut i teksten
margin-lane-a11y-fallback = { $count ->
    [one] Margmerker, 1 merke
   *[other] Margmerker, { $count } merker
}
# Export ▸ Choose: the row standing for a binder that has no name.
export-choose-unnamed-binder = Perm uten navn
# The built-in export styles named in words (the national manuscript presets
# keep their own language's name).
export-style-neutral = Nøytral
export-style-manuscript-shunn = Standardmanus (Shunn)
export-style-ebook-clean = Enkel e-bok

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
settings-paratext-error-no-name = En struktur må ha en `name`-linje.
# Export: started with no project open (a menu raced a close).
export-error-no-project = Det er ikke noe åpent prosjekt å eksportere.

shortcut-group-work = Prosjekt
shortcut-group-edit = Rediger
shortcut-group-view = Vis
shortcut-group-document = Dokument
shortcut-group-format = Format
shortcut-group-go = Gå
shortcut-group-tools = Verktøy
shortcut-group-help = Hjelp

## Manuskript import: the headings of the notes it writes (a character's
## sheet, a world entry, a plot and its steps, the project details, the
## snowflake summary ladder) and the title of a project with no name. Written
## into the imported project once, in the interface language; the writer can
## rename them like any other text. Goal and Conflict reuse the note templates'
## own words.
import-manuskript-section-paragraph-summary = Sammendrag på ett avsnitt
import-manuskript-section-full-summary = Fullt sammendrag
import-manuskript-section-motivation = Motivasjon
import-manuskript-section-epiphany = Erkjennelse
import-manuskript-section-notes = Notater
import-manuskript-section-description = Beskrivelse
import-manuskript-section-passion = Lidenskap
import-manuskript-section-result = Resultat
import-manuskript-section-summary = Sammendrag
import-manuskript-section-meta = Metadata
import-manuskript-section-series = Serie
import-manuskript-section-volume = Bind
import-manuskript-section-genre = Sjanger
import-manuskript-section-license = Lisens
import-manuskript-section-email = E-post
# The snowflake ladder's five tellings, shortest first: one situation, one
# sentence, one paragraph, one page, the full summary.
import-manuskript-section-situation = Situasjon
import-manuskript-section-sentence = Én setning
import-manuskript-section-paragraph = Ett avsnitt
import-manuskript-section-page = Én side
import-manuskript-section-full = Fullt sammendrag
import-manuskript-untitled-project = Importert Manuskript-prosjekt

## Plume import: the story-bible groups an old (0.3 and earlier) project
## implies without naming them, and the title of a project with no name.
## Written into the imported project once, in the interface language.
import-plume-group-characters = Personer
import-plume-group-items = Gjenstander
import-plume-group-places = Steder
import-plume-untitled-project = Importert Plume-prosjekt
