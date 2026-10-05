# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

menu-work = &Project
menu-new-work = &Nieuw project
menu-open-work = Project &openen…
menu-new-window = Nieuw &venster
menu-import-from = &Importeren uit
menu-create-from = &Aanmaken uit
menu-import-plume = &Plume Creator (.plume)…
menu-import-document = &Documenten (Markdown, Word, ODT)…
menu-import-manuskript = &Manuskript (.msk)…
menu-export = E&xporteren
menu-export-book = Boek exporteren
menu-export-part = Deel exporteren
menu-export-chapter = Hoofdstuk exporteren
menu-export-scene = Scène exporteren
menu-export-note = Notitie exporteren
menu-export-paratext = Paratekst exporteren
menu-export-folder = Map exporteren
menu-export-choose = Kiezen…
menu-export-none = Open een document om te exporteren
menu-save = Op&slaan
menu-save-as-file = Opslaan als één &bestand…
menu-save-as-folder = Opslaan als &map…
menu-backup = Nu &reservekopie maken
menu-close-work = Project s&luiten
menu-welcome = &Welkom…
menu-settings = Ins&tellingen
menu-quit = A&fsluiten

menu-view = &Beeld
menu-outline = &Ordner
menu-search = &Zoeken in project
menu-timeline = &Terug in de tijd
menu-search-preview = &Voorbeeld zoekresultaat
menu-fullscreen = Volledig sc&herm
menu-focus-mode = &Afleidingsvrije modus
menu-format = &Opmaak
menu-scene-break = Scèneovergang i&nvoegen
menu-major-scene-break = &Grote scèneovergang invoegen

menu-go = &Ga
menu-go-next-scene = Volgende &scène
menu-go-prev-scene = &Vorige scène
menu-go-next-chapter = Volgend &hoofdstuk
menu-go-prev-chapter = V&orig hoofdstuk
menu-go-next-note = Volgende &notitie
menu-go-prev-note = Vorige no&titie

menu-tools = E&xtra
menu-help = &Help

ctx-add = &Toevoegen
ctx-new-item = &Nieuw item
ctx-new-folder = Nieuwe &map
ctx-rename = &Naam wijzigen
ctx-duplicate = &Dupliceren
ctx-import-here = Hier &importeren…
ctx-indent = &Inspringen
ctx-outdent = &Uitspringen
ctx-trash = Naar &prullenbak verplaatsen
ctx-open-to-side = &Openen aan de zijkant
ctx-open = &Openen
ctx-reveal-in-outline = Tonen in &ordner
ctx-move-up = Omhoog ver&plaatsen
ctx-move-down = Omlaag ver&plaatsen

ctx-tab-close = &Sluiten
ctx-tab-close-others = &Andere sluiten
ctx-tab-close-all = A&lle sluiten
ctx-tab-open-to-side = &Openen aan de zijkant
ctx-tab-open-in-main = &Openen in het hoofdpaneel
ctx-tab-move-to-side = &Verplaatsen naar de zijkant
ctx-tab-move-to-main = &Verplaatsen naar het hoofdpaneel
ctx-tab-move-to-new-window = Verplaatsen naar een &nieuw venster
ctx-tab-move-window-unsaved = Sla dit project eerst op: een tweede venster opent een bestand op schijf
ctx-tab-pin = Dit tabblad vast&zetten
ctx-tab-unpin = Dit tabblad los&maken
tab-pinned-tooltip = Vastgezet: “Andere sluiten” en “Alle sluiten” laten dit tabblad open

create-book = Boek
create-part = Deel
create-chapter = Hoofdstuk
create-scene = Scène
create-note = Notitie
create-note-folder = Notitiemap
create-folder = Map
create-paratext = Paratekst
create-paratext-folder = Paratekstmap
create-book-end = Einde van het boek
create-story-bible-entry = Item in verhaalbijbel…
type-book-start = Begin van het boek
type-text = Tekst

new-item-book = Nieuw boek
new-item-part = Nieuw deel
new-item-chapter = Nieuw hoofdstuk
new-item-note = Nieuwe notitie
new-item-note-folder = Nieuwe notitiemap
new-item-folder = Nieuwe map
new-item-paratext = Nieuwe paratekst
new-item-paratext-folder = Nieuwe paratekstmap
new-item-story-bible-entry = Nieuw item in verhaalbijbel

placement-inside = binnen
placement-after = na
placement-after-parent = na bovenliggend item
placement-top-level = topniveau


ctx-promote = O&mzetten naar
promote-chapter-folder = Hoofdstukmap
promote-flat-chapter = Plat hoofdstuk
promote-lossy-title = Geen plek voor de tekst
promote-lossy-text = Een { $target } heeft geen plek voor: { $kinds }. Verplaats of wis die tekst eerst en zet daarna om.
content-scene-text = Scènetekst
content-note-text = Notitietekst
content-book-title = Boektitel
content-book-subtitle = Ondertitel van het boek
content-part-title = Titel van het deel
content-chapter-title = Titel van het hoofdstuk
content-epigraph-text = Motto
content-paratext-text = Paratekst
promote-blocked-title = Hoofdstuk is niet leeg
promote-blocked-text = { $count ->
        [one] Dit hoofdstuk bevat nog { $count } item. Verplaats het of gooi het in de prullenbak voordat u het omzet naar een plat hoofdstuk.
       *[other] Dit hoofdstuk bevat nog { $count } items. Verplaats ze of gooi ze in de prullenbak voordat u het omzet naar een plat hoofdstuk.
}

inspector = Inspector
inspector-empty = Open een item om het te bekijken.
inspector-promote = Omzetten naar…
statusbar-toggle-outline = Ordner tonen/verbergen
statusbar-toggle-inspector = Inspector tonen/verbergen
statusbar-save-unsaved = Niet-opgeslagen wijzigingen. Klik om op te slaan
statusbar-save-saved = Alle wijzigingen opgeslagen
statusbar-save-autosave = Automatisch opslaan is ingeschakeld. Wijzigingen worden opgeslagen terwijl u schrijft
statusbar-saving = Bezig met opslaan…
statusbar-word-count = { $count ->
    [one] { $count } woord
   *[other] { $count } woorden
}
statusbar-word-count-tooltip = Woorden in de scène die u bewerkt
statusbar-word-char-count = { $words ->
    [one] { $words } woord
   *[other] { $words } woorden
} · { $chars ->
    [one] { $chars } teken
   *[other] { $chars } tekens
}
session-toggle = Schrijfsessie: een geconcentreerde sprint starten of pauzeren
session-configure = Het woordendoel en de tijdslimiet van de sessie instellen
session-configure-title = Schrijfsessie
session-word-goal = Woordendoel
session-time-limit = Tijdslimiet
session-time-limit-unit = min
session-no-goal = Geen doel
session-no-limit = Geen limiet
session-reset = Sessie opnieuw beginnen
session-readout = { $words ->
    [one] { $words } woord
   *[other] { $words } woorden
} · { $time }
session-readout-timed = { $words ->
    [one] { $words } woord
   *[other] { $words } woorden
} · nog { $time }
statusbar-focus-exit = Afleidingsvrije modus verlaten
statusbar-focus-synopsis = Synopsis
statusbar-focus-go-prev = Vorig item (Alt+Omhoog)
statusbar-focus-go-next = Volgend item (Alt+Omlaag)

rail-settings = Instellingen

language = Taal
theme = Thema
english = Engels
french = Frans
light = Licht
dark = Donker
settings-text-width = Tekstbreedte
settings-preview-width = Breedte van voorbeeld zoekresultaat
settings-autosave = Automatisch opslaan naar schijf
settings-show-welcome = Startscherm tonen bij het opstarten
settings-show-welcome-tip = Indien uitgeschakeld, wordt in plaats daarvan het laatste project heropend.
settings-check-for-updates = Controleren op nieuwe versies
settings-check-for-updates-tip = Eenmaal per dag vraagt Skribisto aan de website van het project welke versie actueel is. Het verzoek bevat geen identificatiecode en zelfs niet de versie die u uitvoert: de vergelijking gebeurt op deze computer.

settings-title = Instellingen
settings-close = Sluiten
settings-search = Instellingen doorzoeken
settings-reset = Standaardwaarden herstellen
settings-done = Gereed
settings-cancel = Annuleren
settings-apply = Toepassen
settings-ok = OK
settings-reset-confirm-title = Alle instellingen terugzetten naar de standaardwaarden?
settings-reset-confirm-body = Herstelt de instellingen voor uiterlijk, editor en schrijven van deze toepassing. De eigen instellingen van uw project, sneltoetsen en opgeslagen stijlen blijven ongewijzigd. Dit kan niet ongedaan worden gemaakt.
settings-empty-title = Hier zijn nog geen instellingen
settings-empty-hint = Deze sectie krijgt opties in een toekomstige versie.

settings-sec-appearance-behaviour = Uiterlijk en gedrag
settings-sec-editor = Editor
settings-sec-spelling = Spelling
settings-sec-backup = Reservekopie en synchronisatie
settings-sec-compile = Samenstellen en exporteren
settings-page-appearance = Uiterlijk
settings-field-image-size-policy = Grote afbeeldingen
settings-image-policy-ask = Elke keer vragen
settings-image-policy-keep = Origineel bewaren
settings-image-policy-downscale = Optimaliseren
settings-hint-image-size-policy = Wat er moet gebeuren als een ingevoegde afbeelding groter is dan het manuscript nodig heeft.
settings-page-notifications = Meldingen
settings-page-scene = Scène
settings-page-synopsis = Synopsis
settings-page-notes = Notities
settings-page-editor-behavior = Gedrag van de editor
settings-page-goals = Doelen en aantal woorden
settings-page-games = Schrijfspellen
settings-page-corkboard = Prikbord
settings-page-distraction-free = Afleidingsvrij
settings-page-dictionaries = Woordenboeken
settings-page-autosave = Automatisch opslaan
settings-page-export = Exportformaten
settings-page-paratext = Paratekststructuren
settings-page-keymap = Sneltoetsen
settings-keymap-filter = Sneltoetsen filteren

settings-desc-sec-appearance-behaviour = Hoe de toepassing er zelf uitziet en wat ze doet bij het opstarten.
settings-desc-sec-editor = Het schrijfvlak: hoe het eruitziet en wat het doet terwijl u typt.
settings-desc-sec-spelling = Spellingscontrole en de woordenboeken erachter.
settings-desc-sec-backup = Hoe uw werk op schijf terechtkomt en welke kopieën worden bewaard.
settings-desc-sec-compile = Wat Skribisto verlaat, en in welke vorm.
settings-desc-sec-work = Instellingen die bij dit project horen en in het projectbestand worden meegenomen.
settings-desc-sec-extensions = Pagina’s die worden aangeboden door de hier geïnstalleerde extensies.
settings-desc-group-typography = Eén pagina per soort tekst: lettertype, grootte, regelhoogte en witruimte.
settings-desc-appearance = Interfacetaal, thema, tekstgrootte en het startscherm bij het opstarten.
settings-desc-notifications = Alle meldingen die in deze sessie zijn getoond, en de acties die u opnieuw kunt uitvoeren.
settings-desc-scene = Hoe scèneproza wordt gezet: lettertype, grootte, regelhoogte, inspringingen en witruimte.
settings-desc-synopsis = Hoe het synopsispaneel wordt ingesteld, onafhankelijk van het manuscript.
settings-desc-notes = Hoe notities worden gezet, onafhankelijk van het manuscript.
settings-desc-corkboard = Kaartgrootte, wat een kaart toont en hoe het prikbord ze schikt.
settings-desc-distraction-free = Typografie, kolombreedte en bedieningsbalk voor schrijven op volledig scherm.
settings-desc-distraction-free-themes = De themabibliotheek voor schrijven op volledig scherm: de meegeleverde thema’s en uw eigen thema’s.
settings-desc-editor-behavior = Tekstbreedte, positie van de synopsis, typemachinescrollen en de cursormarkering.
settings-desc-punctuation = De typografische huisstijl waarmee nieuwe projecten beginnen. Elk project volgt deze, tenzij het een eigen stijl instelt.
settings-desc-goals = Doelen voor woorden en tekens, en hoe woorden worden geteld.
settings-desc-games = Beperkingen die u uzelf oplegt tijdens het schrijven van een eerste versie, zoals Altijd vooruit.
settings-desc-spellcheck = De ene schakelaar die de spellingscontrole overal in- en uitschakelt.
settings-desc-dictionaries = Spellingswoordenboeken op deze computer installeren, verwijderen en bekijken.
settings-desc-autosave = Of uw bewerkingen vanzelf naar schijf worden geschreven.
settings-desc-backup = Wanneer kopieën worden gemaakt, waar ze worden bewaard en hoeveel. Elk project volgt dit, tenzij het een eigen beleid instelt.
settings-desc-export = De stijlen waarmee elke export wordt samengesteld: de meegeleverde en uw eigen stijlen.
settings-desc-paratext = De voor- en nawerken waarmee een nieuw project kan beginnen.
settings-desc-keymap = Elke sneltoets en de opdracht waaraan die gekoppeld is.
settings-page-user = Gebruiker
settings-desc-user = Wie u bent, voor de opmerkingen die u schrijft.
settings-group-identity = Identiteit
settings-field-user-name = Uw naam
settings-field-user-name-placeholder = Optioneel
settings-field-user-name-hint = Ondertekent de opmerkingen en antwoorden die u schrijft. Staat los van de auteursnaam van een project: die is de naamsvermelding van het boek en wordt in het projectbestand meegenomen.
settings-field-user-initials = Uw initialen
settings-field-user-initials-placeholder = Afgeleid van uw naam
settings-field-user-initials-hint = Wat een tekstverwerker naast uw opmerking in de marge toont. Laat dit leeg om de getoonde initialen te gebruiken, afgeleid van uw naam.
settings-field-user-hint = Beide zijn optioneel en gelden voor elk project op deze computer. Een wijziging geldt voor de opmerkingen die u hierna schrijft. Opmerkingen die al geschreven zijn, behouden de naam waaronder ze geschreven werden.
settings-desc-author = De naam die op dit project staat.
settings-desc-structure = Of de hoofdstukken van dit project mappen of afzonderlijke items zijn.
settings-desc-language = De taal waartegen het proza van dit project wordt gecontroleerd.
settings-desc-work-backup = Het eigen reservekopiebeleid van dit project, of het algemene beleid.
settings-desc-personal-dictionary = De woorden die dit project als correct gespeld behandelt.
settings-desc-tags = De tags met kleurcode waarmee dit project zijn items markeert.
settings-desc-templates = De notitiesjablonen die u tijdens het schrijven in dit project kunt invoegen.
settings-desc-text-replacements = Afkortingen die worden uitgeschreven terwijl u in dit project typt.
settings-desc-work-punctuation = De aanhalingstekens, streepjes en spaties die het proza van dit project volgt.

settings-group-typography = Typografie
settings-group-writing-column = Schrijfkolom
settings-group-writing-view = Schrijfweergave
settings-group-distraction-free-strip = Bedieningsbalk
settings-group-theme = Thema
settings-group-language = Taal
settings-group-startup = Opstarten
settings-group-updates = Updates
settings-group-autosave = Automatisch opslaan
settings-field-typeface = Lettertype
settings-field-size = Tekstgrootte

editor-size-changed-manuscript = Tekstgrootte manuscript: { $percent }
editor-size-changed-synopsis = Tekstgrootte synopsis: { $percent }
editor-size-changed-notes = Tekstgrootte notities: { $percent }
editor-size-changed-corkboard = Tekstgrootte prikbordkaarten: { $percent }
editor-size-changed-corkboard-expanded = Tekstgrootte uitgevouwen editor: { $percent }
editor-size-changed-distraction-free = Tekstgrootte afleidingsvrije modus: { $percent }

menu-text-size-increase = Tekst &groter
menu-text-size-decrease = Tekst &kleiner
menu-text-size-reset = Tekstgrootte he&rstellen
settings-field-line-height = Regelhoogte
settings-field-first-line-indent = Inspringing eerste regel
settings-field-paragraph-spacing-before = Ruimte boven alinea
settings-field-paragraph-spacing-after = Ruimte onder alinea
settings-field-column-width = Kolombreedte
settings-distraction-free-width-hint = Geldt alleen zolang de afleidingsvrije modus aan staat. De kolombreedtes van Scène, Synopsis en Notities blijven ongewijzigd.
settings-distraction-free-title = De naam van het item behouden
settings-distraction-free-word-count = Het aantal woorden behouden
settings-distraction-free-session = De schrijfsessie behouden
settings-distraction-free-go-to = De knop Ga naar… behouden
settings-distraction-free-go = De knoppen Vorige en Volgende behouden
settings-distraction-free-chrome-hint = De knop Verlaten blijft altijd staan, wat u hier ook kiest. Daarmee komt u er altijd uit als Escape bezet is.
settings-field-app-theme = Thema
settings-theme-light = Licht
settings-theme-dark = Donker
settings-field-text-scale = Tekstgrootte van de interface
settings-field-language = Interfacetaal
settings-synopsis-placement = Synopsispositie
settings-synopsis-placement-none = Geen
settings-synopsis-placement-top = Boven
settings-synopsis-placement-side = Naast
settings-typewriter = Typemachinescrollen
settings-typewriter-tip = Houdt de regel waaraan u schrijft op een vaste hoogte terwijl het manuscript eronder verschuift. Klikken plaatst de cursor nog steeds waar u klikt.
settings-typewriter-position = Positie van de regel
settings-typewriter-position-top-third = Bovenste derde
settings-typewriter-position-middle = Halverwege
settings-typewriter-position-bottom-quarter = Onderste kwart
settings-highlight-scope = Markering rond de cursor
settings-highlight-scope-none = Geen
settings-highlight-scope-sentence = Zin
settings-highlight-scope-paragraph = Alinea
settings-highlight-scope-tip-none = Laat de pagina zoals ze is. Er wordt niets gearceerd terwijl u schrijft.
settings-highlight-scope-tip-sentence = Arceer de zin die u schrijft, zodat die zich onderscheidt van de zinnen eromheen.
settings-highlight-scope-tip-paragraph = Arceer de hele alinea die u schrijft, zodat de passage waaraan u werkt in beeld blijft.
settings-group-container-views = Weergaven van containers
settings-remember-view = De laatste weergave per itemtype onthouden
settings-remember-view-tip = Een container openen in de weergave die u het laatst voor dat type gebruikte
settings-remember-view-tip-more =
    Een boek, deel en hoofdstuk bieden elk meerdere weergaven (een eigen pagina, het volledige
    manuscript, de volledige synopsis). Schakel dit in en elk type opent opnieuw in de
    weergave die u er het laatst voor koos: zet u één hoofdstuk op Volledig hoofdstuk, dan
    toont het volgende hoofdstuk dat u opent ook Volledig hoofdstuk. Elk type onthoudt zijn eigen weergave.
settings-group-counting = Woorden tellen
settings-counting-auto = Automatisch (per taal)
settings-counting-whitespace = Splitsen op spaties
settings-counting-unicode-words = Unicode-woorden
settings-counting-cjk-hybrid = CJK-bewust (per teken)
settings-counting-hint = Automatisch telt Chinees en Japans per teken en alle andere talen per woord. Wijzig dit alleen als de telling voor uw taal niet klopt.
settings-group-goals-display = Weergave
settings-show-characters = Het aantal tekens in de statusbalk tonen
settings-autosave-hint = Wijzigingen worden automatisch naar schijf geschreven terwijl u schrijft.

settings-sec-work = Project
settings-page-structure = Opbouw
settings-page-author = Auteur
settings-field-author-name = Auteursnaam
settings-field-author-placeholder = Optioneel
settings-field-author-hint = Verschijnt op de samengestelde titelpagina en in de metagegevens van geëxporteerde bestanden. Laat dit leeg om het weg te laten. Dit is de naamsvermelding van het boek. De naam waarmee u uw opmerkingen ondertekent, staat in Instellingen ▸ Gebruiker.
settings-group-chapters = Hoofdstukken
tidy-titles-title = Hoofdstuktitels opschonen
tidy-titles-none = Geen enkele hoofdstuk- of deeltitel herhaalt alleen maar zijn eigen nummer.
tidy-titles-lead = { $count ->
        [one] Eén hoofdstuk of deel heeft als titel niets anders dan zijn eigen nummer.
       *[other] { $count } hoofdstukken en delen hebben als titel niets anders dan hun eigen nummer.
    }
tidy-titles-explain = Als u die titels wist, wordt elk ervan aangeduid met het nummer dat het boek al kent, hetzelfde nummer dat de export afdrukt. Verder verandert er niets, en met één keer ongedaan maken staan ze er allemaal weer.
menu-document-tidy-titles = Hoofdstuktitels &opschonen…
settings-group-numbering = Nummering
settings-number-chapters = Hoofdstukken en delen nummeren
settings-number-chapters-tip = Hoofdstukken en delen krijgen een nummer dat wordt afgeleid van hun plaats in het boek. Het staat hier naast hun titel en wordt door de export afgedrukt.
settings-number-chapters-tip-more = Het nummer wordt nooit in de titel opgeslagen, dus het blijft kloppen als u herschikt, invoegt of verwijdert. Schakel dit uit en de export drukt alleen titels af, wat de exportstijl ook vraagt. Wilt u één hoofdstuk overslaan (een proloog, een intermezzo), gebruik dan de schakelaar Nummering in de Inspector: het blijft dan in het boek, maar krijgt geen nummer.
settings-part-resets-chapter = Hoofdstuknummering bij elk deel opnieuw beginnen
settings-part-resets-chapter-tip = Standaard uit: hoofdstukken worden over de delen van een boek heen doorgenummerd, zodat “Deel twee” begint met hoofdstuk elf.
settings-part-resets-chapter-tip-more = Dat is gebruikelijk in de uitgeverswereld, en wat een lezer verwacht. Schakel dit in voor een boek waarvan de delen als afzonderlijke banden bedoeld zijn, die elk met hoofdstuk één beginnen.
settings-chapter-flat = Platte hoofdstukken
settings-chapter-flat-hint = Aan: een hoofdstuk is één enkel item. U schrijft erin en het bevat geen scènes. Uit: een hoofdstuk is een map. U schrijft er nog steeds in, maar het kan ook scènes bevatten. Nieuwe hoofdstukken volgen deze instelling; bestaande worden omgezet via Omzetten naar.

settings-styles-builtin = Ingebouwde stijlen
settings-styles-user = Mijn stijlen
settings-styles-builtin-badge = Ingebouwd
settings-styles-duplicate = Dupliceren
settings-styles-edit = Bewerken
settings-styles-delete = Verwijderen
settings-styles-import = Importeren…
settings-styles-export = Exporteren…
settings-styles-copy-suffix = (kopie)
settings-styles-json-filter = Exportstijl
settings-styles-editor-title = Stijl bewerken
settings-styles-editor-none = Selecteer een aangepaste stijl om die te bewerken, of dupliceer een ingebouwde stijl.
settings-styles-imported = Stijl geïmporteerd
settings-styles-import-failed = Stijl kan niet worden geïmporteerd
settings-styles-exported = Stijl geëxporteerd
settings-styles-export-failed = Stijl kan niet worden geëxporteerd
settings-styles-field-name = Naam
settings-styles-field-chapters = Koppen van hoofdstukken
settings-styles-field-parts = Koppen van delen
settings-styles-field-scene-break = Scèneovergang
settings-styles-field-major-scene-break = Grote scèneovergang
settings-styles-field-spacing = Regelafstand
settings-styles-field-justify = Tekst uitvullen
settings-styles-field-notes = Notities opnemen
settings-styles-field-synopses = Synopsissen opnemen
settings-styles-field-scene-titles = Scènetitels opnemen
settings-styles-field-epigraphs = Motto’s opnemen
settings-styles-field-paratexts = Parateksten opnemen
settings-styles-field-epigraph-placement = Positie van het motto
settings-styles-epigraph-after = Na de titel
settings-styles-epigraph-before = Vóór de titel
settings-styles-field-footnotes = Voetnoten opnemen
settings-styles-field-footnote-numbering = Nummering van voetnoten
settings-styles-footnote-numbering-continuous = Doorlopend
settings-styles-footnote-numbering-per-chapter = Opnieuw per hoofdstuk
settings-styles-footnote-numbering-per-book = Opnieuw per boek
settings-styles-field-images = Afbeeldingen
menu-image = A&fbeelding
settings-styles-images-beside = Naast het document
settings-styles-images-embed = In het document
settings-styles-images-omit = Weglaten
settings-styles-group-round-trip = Versturen naar een redacteur
settings-styles-field-comments = Opmerkingen meenemen
settings-styles-field-round-trip-marks = Retourmarkeringen opnemen
settings-styles-round-trip-hint = Alleen DOCX en ODT. Markeringen zijn onzichtbare kenmerken waarmee een teruggestuurd bestand dit project kan bijwerken, in plaats van ernaast te belanden als tweede kopie.
settings-styles-group-pages = Pagina’s
settings-styles-field-cover = Openen met de omslag
settings-styles-field-word-count = Aantal woorden op titelpagina
settings-styles-field-page-books = Nieuwe pagina bij elk boek
settings-styles-field-page-parts = Nieuwe pagina bij elk deel
settings-styles-field-page-chapters = Nieuwe pagina bij elk hoofdstuk
settings-styles-field-page-paratexts = Nieuwe pagina bij elke paratekst
settings-styles-heading-none = Geen kop
settings-styles-heading-numbered = Alleen nummer
settings-styles-heading-title = Alleen titel
settings-styles-heading-both = Nummer + titel
settings-styles-break-blank = Lege regel
settings-styles-break-none = Geen
settings-styles-spacing-single = Enkel
settings-styles-spacing-onehalf = 1½
settings-styles-spacing-double = Dubbel

welcome-title = Welkom bij Skribisto
welcome-version = Versie { $version }
welcome-update-available = Versie { $version } is beschikbaar
welcome-update-tip = Opent de downloadpagina in uw browser.
welcome-search = Projecten doorzoeken
welcome-open = Openen
welcome-new-work = Nieuw project
welcome-create-from = Aanmaken uit…
welcome-recent-works = Recente projecten
welcome-empty-recents = Nog geen recente projecten.
welcome-no-matches = Geen recent project komt overeen met uw zoekopdracht.
welcome-learn-soon = Er komen binnenkort gidsen en tips.
welcome-about-blurb = Skribisto, een herschreven versie van de schrijfapp in Rust + Teksilo.
welcome-tagline = *Een rustige plek om lange teksten te schrijven.*
welcome-github = GitHub
welcome-discord = Discord
nav-works = Projecten
nav-examples = Voorbeelden
nav-learn = Leren
nav-about = Info

synopsis = Synopsis
epigraph = Motto
pane-manuscript = Manuscript
corkboard = Prikbord
overview = Overzicht

corkboard-card-count = { $count ->
    [one] { $count } kaart
   *[other] { $count } kaarten
}
corkboard-child-count = { $count ->
        [one] { $count } item
       *[other] { $count } items
    }
corkboard-view-nested = Genest
corkboard-view-flat = Plat
corkboard-layout-hint = Genest toont de directe onderliggende items van een container. Open een mapkaart om erin te gaan. Plat toont alle scènes van de hele container tegelijk.
corkboard-card-size = Kaartgrootte
corkboard-search-placeholder = Kaarten filteren…
corkboard-empty-title = Hier is nog niets
corkboard-empty-hint = Gebruik “＋ Nieuw” hierboven om het eerste toe te voegen.
corkboard-new = Nieuw
corkboard-show-card-numbers = Kaarten nummeren
corkboard-modal-size = Grootte van de uitgevouwen editor
corkboard-scope-hint = Geldt voor elk geopend prikbord, in elk project.
corkboard-sort-manuscript = Manuscriptvolgorde
corkboard-sort-title-asc = Titel A–Z
corkboard-sort-title-desc = Titel Z–A
corkboard-card-number = Kaart { $number }

duplicate = Dupliceren
duplicate-n = { $count ->
        [one] { $count } kaart dupliceren
       *[other] { $count } kaarten dupliceren
    }
set-label-n = { $count ->
        [one] Label instellen voor { $count } kaart
       *[other] Label instellen voor { $count } kaarten
    }
move-to-trash-n = { $count ->
        [one] { $count } kaart naar de prullenbak verplaatsen
       *[other] { $count } kaarten naar de prullenbak verplaatsen
    }
reveal-in-outline = Tonen in ordner

corkboard-move-to = Verplaatsen naar…
corkboard-move-to-n = { $count ->
        [one] { $count } kaart verplaatsen naar…
       *[other] { $count } kaarten verplaatsen naar…
    }
corkboard-move-picker-title = Verplaatsen naar…
corkboard-move-picker-empty = Nog geen ordners. Maak er eerst een aan.
corkboard-move-picker-cancel = Annuleren
corkboard-move-here = Hierheen verplaatsen
corkboard-moved-ok = { $count ->
    [one] { $count } kaart verplaatst
   *[other] { $count } kaarten verplaatst
}
corkboard-move-into-self = Een container kan niet in zichzelf worden verplaatst. Kies een bestemming erbuiten.
corkboard-move-failed = Die kaarten konden daar niet worden verplaatst.

overview-col-title = Titel
overview-col-type = Type
overview-col-label = Label
overview-col-tags = Tags
overview-col-books = Boeken
overview-col-own-words = Woorden
overview-col-total-words = Totaal
overview-row-count = { $count ->
        [one] { $count } item
       *[other] { $count } items
    }
overview-search-placeholder = Items filteren…
overview-expand-all = Alles uitvouwen
overview-collapse-all = Alles invouwen
overview-table-label = Inhoud
overview-empty-title = Hier is nog niets
overview-gone-title = Deze container bestaat niet meer
overview-gone-hint = Hij is naar de prullenbak verplaatst. Zet hem terug, of sluit dit tabblad.
overview-empty-hint = Gebruik “＋ Nieuw” hierboven om het eerste toe te voegen.
corkboard-grid-label = Prikbordkaarten
corkboard-rename-field = Naam van item wijzigen
corkboard-expand-synopsis = Synopsis uitvouwen
corkboard-synopsis-modal-title = Synopsis
corkboard-badge-scene = Scène
corkboard-badge-chapter = Hoofdstuk
corkboard-badge-part = Deel
corkboard-badge-book = Boek
corkboard-badge-note = Notitie
corkboard-badge-folder = Map
corkboard-badge-paratext = Paratekst
corkboard-badge-text = Tekst
corkboard-badge-end = Einde
settings-group-corkboard-layout = Indeling
settings-group-corkboard-cards = Kaarten
corkboard-show-word-count = Aantal woorden op kaarten tonen
text-heading = Tekst
no-content = Dit item heeft geen bewerkbare inhoud.
untitled = Naamloos
placeholder-title = Titel…
placeholder-subtitle = Ondertitel…
placeholder-chapter-title = Titel van het hoofdstuk…
split-editor = Editor splitsen
close-split-view = Gesplitste weergave sluiten
drop-open-here = Hier openen
drop-open-to-side = Openen aan de zijkant

segment-chapter = Hoofdstuk
segment-part = Deel
segment-book = Boek
segment-pace = Tempo
segment-notes = Notities
segment-story-bible = Verhaalbijbel
segment-note-own = Notitie
segment-note-details = Details
segment-note-in-prose = In proza
pace-placeholder = De tempoplanner verschijnt hier.
pace-empty-title = Plan het tempo van dit boek
pace-empty-body = Stel een woordendoel en een deadline in, en Skribisto berekent het dagelijkse tempo dat u daar brengt.
pace-start-planning = Beginnen met plannen
pace-section-schedule = Schema
pace-section-progress = Voortgang
pace-advancement = Vordering
pace-goal = Woordendoel
pace-deadline = Deadline
pace-active = Tempo actief
pace-day-mon = ma
pace-day-tue = di
pace-day-wed = wo
pace-day-thu = do
pace-day-fri = vr
pace-day-sat = za
pace-day-sun = zo
pace-card-written = woorden geschreven
pace-card-of-goal = van het doel
pace-card-rate = woorden / schrijfdag
pace-card-days-left = schrijfdagen te gaan
pace-card-streak = dagen op rij
pace-card-ahead = woorden voor op schema
pace-card-behind = woorden achter op schema
pace-charts-empty = Voortgangsgrafieken verschijnen hier zodra uw aantal woorden bij het opslaan is vastgelegd.
pace-chart-progression = Geschreven woorden t.o.v. doel
pace-chart-words-per-day = Woorden per dag
pace-series-actual = Werkelijk
pace-series-target = Doel
pace-series-words-per-day = Woorden/dag
pace-daily-target-line = Gelijkmatig tempo: { $count ->
        [one] { $count } woord/dag
       *[other] { $count } woorden/dag
    }
pace-section-holidays = Vrije dagen
pace-section-milestones = Mijlpalen
pace-holidays-none = Geen vrije dagen. Elke geplande dag telt mee.
pace-holiday-label = Naam van de vrije dag
pace-add-holiday = Toevoegen
pace-remove = Verwijderen
pace-milestones-none = Nog geen mijlpalen. Stel er een in op een deel of hoofdstuk in de Inspector.
full-chapter = Volledig hoofdstuk
full-part = Volledig deel
full-book = Volledig boek
full-synopsis = Volledige synopsis
rename = Naam wijzigen
set-label = Label instellen
insert-scene = Scène invoegen
insert-chapter = Hoofdstuk invoegen
split-scene = Scène splitsen
move-up = Omhoog verplaatsen
move-down = Omlaag verplaatsen
merge-with-previous = Samenvoegen met vorige
move-to-trash = Naar de prullenbak verplaatsen
rename-chapter = Naam van hoofdstuk wijzigen
add-scene = Nieuwe scène toevoegen
add-chapter = Nieuw hoofdstuk toevoegen
new-scene-title = Nieuwe scène
placeholder-scene-name = Naam van de scène…
menu-cut = Knippen
menu-copy = Kopiëren
menu-paste = Plakken
menu-paste-unformatted = Plakken zonder opmaak
menu-select-all = Alles selecteren
format-bold = Vet
format-italic = Cursief
format-underline = Onderstreept
format-strikethrough = Doorgestreept

binder = Ordner
no-work = Geen project geladen
no-recent-works = Geen recente projecten
switcher-open-section = Momenteel geopend
switcher-recent-section = Recent
switcher-this-window = dit venster
open-project-title = Project openen
open-project-question = Hoe wilt u “{ $title }” openen?
open-in-new-window = In nieuw venster openen
open-here = Hier openen

binder-all = Alle ordners
binder-show-all = Alle ordners tonen
binder-new = Nieuwe ordner…
binder-default-name = Nieuwe ordner
binder-item-count = { $count ->
        [one] { $count } item
       *[other] { $count } items
    }
binder-search-placeholder = Ordner filteren…
binder-search-scope = Alle ordners doorzoeken
binder-trash-confirm-title = Ordner naar de prullenbak verplaatsen?
binder-trash-confirm-text = “{ $name }” en al zijn items worden naar de prullenbak verplaatst.

dialog-rename = Naam wijzigen
dialog-new-binder = Nieuwe ordner
dialog-set-label = Label instellen
dialog-new-scene = Nieuwe scène
close-work-question = Wijzigingen opslaan vóór het sluiten van het project?
quit-question = Wijzigingen opslaan vóór het afsluiten?
quit-save-work-question = Wijzigingen in { $title } opslaan voordat u afsluit?
unsaved-changes = Dit project bevat niet-opgeslagen wijzigingen.
new-work-unsaved-question = Wijzigingen opslaan voordat u een nieuw project aanmaakt?
open-work-unsaved-question = Wijzigingen opslaan voordat u een ander project opent?
switch-backup-discard-title = Wijzigingen in deze reservekopie verwerpen?
switch-backup-discard-text = Wijzigingen in een reservekopie kunnen daarin niet worden opgeslagen. Gebruik Opslaan als of Herstellen om ze te bewaren, of verwerp ze en open het andere project.
switch-save-failed = Het project kon niet worden opgeslagen en is dus niet vervangen: { $error }
switch-save-not-started = Het project kon niet worden opgeslagen en is dus niet vervangen.
close-save-failed = Het project kon niet worden opgeslagen en is dus niet gesloten: { $error }
close-save-not-started = Het project kon niet worden opgeslagen en is dus niet gesloten.
save-not-started = Het project kon niet worden opgeslagen.

could-not-open-work = Kan “{ $file }” niet openen
could-not-open-work-too-new = “{ $file }” vereist een nieuwere versie van Skribisto
could-not-open-work-too-new-detail = Opgeslagen in Skribisto-formaat { $written_by }; openen vereist formaat { $requires } of nieuwer, en deze versie ondersteunt tot en met formaat { $supported }. Werk Skribisto bij om het te openen.
could-not-open-work-importing = “{ $file }” wordt door een import geschreven
could-not-open-work-importing-detail = Een import overschrijft dit project en is nog niet klaar. Open het zodra de import voltooid is: als u het nu opent, toont het het project dat de import vervangt, en de volgende opslag zou dat project weer over de import heen schrijven.
could-not-open-example = Kan het voorbeeld niet openen: { $error }
could-not-open-link = Kan { $url } niet openen: { $error }
could-not-create-work = Kan het project niet aanmaken: { $error }
saving-as-file = Opslaan als { $target }…
saving-as-folder = Opslaan als { $target }/…
saved-as = Opgeslagen in { $target }
save-error = Kan niet opslaan: { $error }
save-flush-failed = Het opslaan is gelukt, maar deze documenten konden er niet aan worden doorgegeven en zijn nog niet opgeslagen: { $titles } ({ $error })
backup-error = Kan geen reservekopie maken: { $error }
backing-up = Reservekopie maken…
backup-nothing-open = Er is geen project geopend om een reservekopie van te maken.
backup-already-running = Er wordt al een reservekopie gemaakt.
backup-complete = Reservekopie voltooid ({ $ok } opgeslagen, { $skipped } al actueel)
backup-partial = Reservekopie klaar: { $ok } opgeslagen, { $failed ->
        [one] { $failed } bestemming mislukt
       *[other] { $failed } bestemmingen mislukt
    }
backup-no-destination-title = Geen reservekopielocatie beschikbaar
backup-no-destination-text = Geen van de ingestelde bestemmingen voor reservekopieën is bereikbaar (een externe schijf is bijvoorbeeld misschien niet aangesloten). Sluit die aan en probeer het opnieuw, of sluit af zonder reservekopie.
backup-failed-close-title = De reservekopie kon niet worden opgeslagen
backup-failed-close-text = Er kon vóór het sluiten geen reservekopie worden geschreven. Elke bestemming is mislukt (de schijf is misschien verwijderd, vol of tegen schrijven beveiligd). Los het probleem op en probeer het opnieuw, of sluit af zonder reservekopie.

backup-choice-title = Reservekopiebestand
backup-choice-heading = U hebt een reservekopie geopend
backup-choice-subtitle = Dit is een reservekopie van een project op een bepaald moment.
backup-choice-subtitle-dated = Reservekopie gemaakt op { $date }.
backup-choice-body = U kunt hem openen en vrij bewerken, maar wijzigingen kunnen alleen met Opslaan als worden bewaard. Het oorspronkelijke projectbestand blijft onaangeroerd. U kunt dit project ook precies naar deze reservekopie herstellen.
backup-choice-open = De reservekopie openen
backup-choice-restore = Dit project naar dit moment herstellen…
backup-choice-not-a-backup = Nee, gewoon openen
backup-banner-title = Reservekopie: wijzigingen kunnen hier niet worden opgeslagen
backup-banner-description = Gebruik Opslaan als om uw bewerkingen in een nieuw bestand te bewaren, of Herstellen om het oorspronkelijke project te vervangen door deze reservekopie.
backup-banner-restore = Herstellen…
backup-banner-save-as = Opslaan als…
backup-restore-original-missing = Het oorspronkelijke project om overheen te herstellen is niet gevonden. Gebruik Opslaan als om deze reservekopie in plaats daarvan als nieuw project te bewaren.
backup-restore-close-elsewhere-title = Project geopend in een ander venster
backup-restore-close-elsewhere-text = Het project dat u wilt herstellen, is geopend in een ander venster. Sluit het daar eerst en probeer het dan opnieuw.
backup-restore-focus-window = Naar dat venster gaan
backup-restore-confirm-title = Deze reservekopie herstellen?
backup-restore-confirm-text = De huidige versie van het project wordt als veiligheidskopie apart gezet voordat ze door deze wordt vervangen.
backup-restore-confirm-ok = Herstellen
backup-restore-error = Kan niet herstellen: { $error }
backup-restored-ok = Project hersteld.
backup-restored-with-safety = Project hersteld. Uw vorige versie is opgeslagen in { $path }.
close-backup-discard-title = Wijzigingen in deze reservekopie verwerpen?
close-backup-discard-text = Wijzigingen in een reservekopie kunnen daarin niet worden opgeslagen. Gebruik Opslaan als om ze te bewaren, of verwerp ze en sluit.
quit-backup-discard-title = Wijzigingen verwerpen en afsluiten?
quit-backup-discard-work-question = Wijzigingen in { $title } verwerpen en afsluiten?
quit-backup-discard-text = Wijzigingen in een reservekopie kunnen daarin niet worden opgeslagen. Gebruik Opslaan als om ze te bewaren, of verwerp ze en sluit af.
backup-nudge-text = Voor dit project zijn geen reservekopieën ingesteld.
backup-nudge-action = Reservekopieën instellen…

menu-backups-list = Reserve&kopieën…
backups-title = Reservekopieën
backups-loading = Zoeken naar reservekopieën…
backups-empty = Nog geen reservekopieën gevonden voor dit project.
backups-open = Openen
backups-reveal = Tonen in map
backups-delete = Deze reservekopie verwijderen
backups-delete-confirm-title = Deze reservekopie verwijderen?
backups-delete-confirm-text = “{ $name }” wordt definitief verwijderd. Dit kan niet ongedaan worden gemaakt.
backups-delete-error = Kan de reservekopie niet verwijderen: { $error }
backups-refresh = Verversen
backups-close = Sluiten

settings-page-backup = Standaardinstellingen voor reservekopieën
settings-page-work-backup = Reservekopieën
settings-backup-general-title = Standaardinstellingen voor reservekopieën
settings-backup-work-title = Reservekopieën voor dit project
settings-backup-inherit = Algemene instellingen voor reservekopieën gebruiken
settings-backup-inheriting = Dit project gebruikt de algemene instellingen voor reservekopieën.
settings-backup-none-hint = Er zijn geen automatische reservekopieën ingesteld (elke aanleiding staat uit).
settings-backup-last = Laatste reservekopie: { $date }
settings-backup-last-never = Nog geen reservekopieën.
settings-backup-open-list = Lijst met reservekopieën openen…
settings-backup-triggers = Wanneer een reservekopie maken
settings-backup-on-close = Reservekopie maken bij het sluiten van het project
settings-backup-on-open = Reservekopie maken bij het openen van het project
settings-backup-interval = Regelmatig een reservekopie maken, elke
settings-backup-destinations = Bestemmingen voor reservekopieën
settings-backup-default-location = Standaardlocatie
settings-backup-usage-since = { $count ->
        [one] { $count } reservekopie · { $size } · oudste { $oldest }
       *[other] { $count } reservekopieën · { $size } · oudste { $oldest }
    }
settings-backup-usage = { $count ->
        [one] { $count } reservekopie · { $size }
       *[other] { $count } reservekopieën · { $size }
    }
settings-backup-usage-empty = Hier is nog niets bewaard
settings-backup-usage-measuring = Meten…
settings-backup-reveal-root = Map tonen
settings-backup-dest-none = Geen bestemmingen. Reservekopieën worden naast het project opgeslagen.
settings-backup-dest-remove = Verwijderen
settings-backup-dest-add = Map toevoegen…
settings-backup-dest-refresh = Verversen
settings-backup-retention = Hoeveel bewaren
settings-backup-retention-tiered = Getrapt
settings-backup-retention-keep-n = Laatste N bewaren
settings-backup-retention-tip = Hoe oude reservekopieën worden opgeruimd.
settings-backup-retention-tip-more = Getrapt bewaart één reservekopie per uur gedurende een dag, per dag gedurende een week, per week gedurende een maand en daarna per maand, zodat de recente geschiedenis dicht blijft en de oude geschiedenis uitdunt. “Laatste N bewaren” bewaart gewoon de N meest recente kopieën. In beide modi worden de nieuwste kopieën (het minimum hieronder) altijd bewaard.
settings-backup-gfs-hourly = Per uur (laatste 24 u)
settings-backup-gfs-daily = Dagelijks (laatste week)
settings-backup-gfs-weekly = Wekelijks (laatste maand)
settings-backup-gfs-monthly = Maandelijks
settings-backup-keep-n = Aantal te bewaren
settings-backup-min-keep = Altijd minstens bewaren
settings-backup-dedup = Een reservekopie overslaan als er niets is veranderd

new-work-title = Nieuw project
new-work-close = Sluiten
new-work-name = Projectnaam
new-work-name-placeholder = Naamloos
new-work-author = Auteur
new-work-author-placeholder = Optioneel
new-work-format = Opslagvorm
new-work-single-file = Eén bestand
new-work-bundle = Bundel
new-work-convert-later = U kunt later naar een andere opslagvorm overschakelen.
new-work-location = Locatie
new-work-will-create = Wordt aangemaakt
new-work-language = Standaardtaal
new-work-language-hint = Gebruikt voor nieuwe teksten en de spellingscontrole. Elke tekst kan op een andere taal worden gezet.
new-work-template = Sjabloon
new-work-template-none = Geen
new-work-template-empty-novel = Lege roman
new-work-template-light-novel = Light novel
new-work-template-novel = Roman
new-work-template-novel-in-parts = Roman in delen
new-work-template-notebook = Notitieboek
new-work-cancel = Annuleren
new-work-create = Project aanmaken
new-work-step-details = Details
new-work-step-language = Taal en opbouw
new-work-step-template = Sjabloon
new-work-back = Terug
new-work-next = Volgende
new-work-documents-title = Nieuw project uit documenten
new-work-step-import = Importeren
new-work-create-and-import = Aanmaken en importeren…
new-work-documents-next-title = Daarna volgen uw documenten
new-work-documents-next-body = Bij het aanmaken van het project opent de importassistent erboven: daar kiest u de documenten, controleert u de structuur die Skribisto erin leest en bepaalt u waar alles terechtkomt. Er wordt niets in het project geschreven totdat u het daar bevestigt.
new-work-documents-no-template = Dit project begint met opzet leeg: geen sjabloon, zodat de geïmporteerde documenten het enige zijn wat erin staat.
new-work-documents-chapter-scene-hint = Geldt voor de hoofdstukken die de import aanmaakt. U kunt dit later wijzigen in de instellingen van het project.
new-work-single-file-desc = Eén .skrib-archief (zip). Draagbaar, eenvoudig om reservekopieën van te maken.
new-work-bundle-desc = Een map met elke tekst en elk bestand. Geschikter voor versiebeheer.
new-work-template-none-count = lege ordner
new-work-template-empty-novel-count = 1 hoofdstuk
new-work-template-light-novel-count = 15 hoofdstukken
new-work-template-novel-count = 20 hoofdstukken
new-work-template-novel-in-parts-count = 3 delen, 24 hoofdstukken
new-work-template-notebook-count = vrije notities
new-work-chapter-scene = Platte hoofdstukken
new-work-chapter-scene-tip = Elk hoofdstuk is één enkel item waarin u direct schrijft, zonder scènes eronder. Laat dit uit voor de klassieke indeling, waarin een hoofdstuk een map is: ook daarin schrijft u direct, maar het kan daarnaast scènes bevatten.
new-work-chapter-scene-tip-more = U schrijft hoe dan ook in een hoofdstuk. Het enige verschil is of het scènes kan *bevatten*. De ordnerboom van Skribisto dient alleen om te ordenen, dus beide indelingen leveren hetzelfde boek op, u kunt ze vrij combineren, en Omzetten naar zet een hoofdstuk van de ene naar de andere vorm zonder een woord te verliezen.
new-work-name-required = Voer een naam in voor het project
new-work-name-invalid = Deze naam heeft geen bruikbare tekens
new-work-location-required = Kies een locatie
new-work-location-missing = Deze map bestaat niet
new-work-location-not-folder = Dit pad is geen map
new-work-location-readonly = In deze map kan niet worden geschreven
new-work-target-exists = Er staat al een project met deze naam in deze map. Bij het aanmaken wordt gevraagd of het mag worden vervangen
new-work-target-is-folder = Op deze locatie staat al een map met deze naam. Kies een andere naam
new-work-overwrite-title = Het bestaande project vervangen?
new-work-overwrite-text = “{ $name }” bestaat al in deze map. Vervangen door het nieuwe project?
new-work-target-open-title = Dit project is geopend
new-work-target-open-text = “{ $name }” is geopend in Skribisto. Een nieuw project op die plaats aanmaken zou het project waaraan u werkt vervangen, en bij de volgende opslag zou dat project weer over het nieuwe heen worden geschreven. Kies een andere naam of map, of sluit eerst het project.
target-importing-title = Een import schrijft dit project
target-importing-text = Een import is nog bezig “{ $name }” te schrijven. Wacht tot die klaar is, of kies een andere naam.
import-plume-busy-title = Er wordt al een ander Plume Creator-project geïmporteerd
import-plume-busy-text = Plume Creator-projecten worden een voor een geïmporteerd. Wacht tot de lopende import klaar is, of annuleer die, en start deze daarna opnieuw.
import-manuskript-busy-title = Er wordt al een ander Manuskript-project geïmporteerd
import-manuskript-busy-text = Manuskript-projecten worden een voor een geïmporteerd. Wacht tot de lopende import klaar is, of annuleer die, en start deze daarna opnieuw.
backup-restore-importing-text = Een import is nog bezig “{ $name }” te schrijven, het project waarover deze reservekopie zou worden hersteld. Herstel de reservekopie zodra de import klaar is.

new-work-manuscript = Manuscript
new-work-notes = Notities
new-work-research = Onderzoek
new-work-notebook = Notitieboek
new-work-scene = Scène
new-work-note = Notitie
new-work-front-matter = Voorwerk
new-work-back-matter = Nawerk
new-work-paratext = Boekstructuur
new-work-paratext-none = Geen structuur
new-work-paratext-hint = Het voor- en nawerk waarmee een traditie een boek opent en afsluit. U kunt alles daarna verplaatsen, hernoemen of verwijderen.
new-work-tags = Tags
new-work-tags-none = Geen tags
new-work-tags-hint = Een startpalet om personages, plaatsen en de rest te taggen. Optioneel; elke tag kan daarna worden hernoemd, van kleur veranderd of verwijderd.
new-work-note-templates = Notitiesjablonen
new-work-note-templates-none = Geen sjablonen
new-work-note-templates-hint = De vorm waarin een notitie in de verhaalbijbel begint. Optioneel; elk sjabloon kan daarna worden bewerkt of verwijderd.
new-work-characters = Personages
new-work-places = Plaatsen

import-plume-title = Plume Creator-project importeren
import-plume-close = Sluiten
import-plume-source = Plume-project
import-plume-source-hint = Kies een .plume- of .plume_backup-bestand (elke versie van Plume Creator).
import-plume-location = Doelmap
import-plume-name = Bestandsnaam
import-plume-name-placeholder = Projectnaam
import-plume-will-create = Wordt aangemaakt
import-plume-trash-warning = ⚠ Items in de prullenbak of verwijderde items worden niet overgenomen.
import-plume-cancel = Annuleren
import-plume-import = Importeren
import-plume-source-required = Kies een Plume-projectbestand
import-plume-source-missing = Dit bestand bestaat niet
import-plume-source-not-file = Dit pad is geen bestand
import-plume-location-required = Kies een doelmap
import-plume-location-missing = Deze map bestaat niet
import-plume-location-not-folder = Dit pad is geen map
import-plume-location-readonly = In deze map kan niet worden geschreven
import-plume-name-required = Voer een bestandsnaam in
import-plume-name-exists = Hier bestaat al een bestand met deze naam. Bij het importeren wordt gevraagd of het mag worden overschreven
import-plume-overwrite-title = Bestaand bestand vervangen?
import-plume-overwrite-text = “{ $name }” bestaat al. Vervangen door het geïmporteerde project?
import-plume-manuscript-binder = Manuscript
import-plume-story-bible-binder = Verhaalbijbel
import-plume-progress-title = Plume-project wordt geïmporteerd…
import-plume-cancel-import = Annuleren
import-plume-cancelled = Importeren geannuleerd
import-plume-done = { $imported ->
        [one] { $imported } item geïmporteerd.
       *[other] { $imported } items geïmporteerd.
    } { $skipped ->
        [one] { $skipped } item uit de prullenbak is niet overgenomen.
       *[other] { $skipped } items uit de prullenbak zijn niet overgenomen.
    }
import-plume-open-now = Nu openen
import-plume-warnings = { $count ->
        [one] 1 onderdeel kon niet exact worden geïmporteerd
       *[other] { $count } onderdelen konden niet exact worden geïmporteerd
    }
import-plume-details = Details
import-plume-warnings-title = Importwaarschuwingen
import-plume-error-title = Kan het project niet importeren
import-plume-nested-too-deep = Het project is niet geïmporteerd: het onderdeel “{ $part }” is meer dan { $limit } niveaus diep genest. Geen enkel Plume Creator-project komt daar in de buurt, en het inlezen zou Skribisto kunnen laten vastlopen. Er is niets geschreven.
import-plume-declares-entities = Het project is niet geïmporteerd: het onderdeel “{ $part }” declareert XML-entiteiten, wat geen enkel Plume Creator-project doet. Het uitvouwen ervan zou meer geheugen kunnen vragen dan de computer heeft en Skribisto laten vastlopen. Er is niets geschreven.
import-plume-error-details = Details

import-manuskript-title = Manuskript-project importeren
import-manuskript-close = Sluiten
import-manuskript-source = Manuskript-project
import-manuskript-source-hint = Kies het .msk-bestand of de projectmap (elke versie van Manuskript).
import-manuskript-source-file = Bestand kiezen…
import-manuskript-source-folder = Map kiezen…
import-manuskript-location = Doelmap
import-manuskript-name = Bestandsnaam
import-manuskript-name-placeholder = Projectnaam
import-manuskript-will-create = Wordt aangemaakt
import-manuskript-cancel = Annuleren
import-manuskript-import = Importeren
import-manuskript-source-required = Kies een Manuskript-project
import-manuskript-source-missing = Dit bestand of map bestaat niet
import-manuskript-location-required = Kies een doelmap
import-manuskript-location-missing = Deze map bestaat niet
import-manuskript-location-not-folder = Dit pad is geen map
import-manuskript-location-readonly = In deze map kan niet worden geschreven
import-manuskript-name-required = Voer een bestandsnaam in
import-manuskript-name-exists = Hier bestaat al een bestand met deze naam. Bij het importeren wordt gevraagd of het mag worden overschreven
import-manuskript-overwrite-title = Bestaand bestand vervangen?
import-manuskript-overwrite-text = “{ $name }” bestaat al. Vervangen door het geïmporteerde project?
import-manuskript-manuscript-binder = Manuscript
import-manuskript-story-bible-binder = Verhaalbijbel
import-manuskript-characters-group = Personages
import-manuskript-world-group = Wereld
import-manuskript-plots-group = Verhaallijnen
import-manuskript-project-info-note = Projectinformatie
import-manuskript-summary-note = Samenvatting
import-manuskript-importance-minor = Bijfiguur
import-manuskript-importance-secondary = Secundair
import-manuskript-importance-main = Hoofdpersonage
import-manuskript-progress-title = Manuskript-project wordt geïmporteerd…
import-manuskript-cancel-import = Annuleren
import-manuskript-cancelled = Importeren geannuleerd
import-manuskript-done = { $imported ->
        [one] { $imported } item geïmporteerd.
       *[other] { $imported } items geïmporteerd.
    } { $revisions ->
        [0] { "" }
        [one] { $revisions } eerdere versie is meegekomen.
       *[other] { $revisions } eerdere versies zijn meegekomen.
    }
import-manuskript-open-now = Nu openen
import-manuskript-warnings = { $count ->
        [one] 1 punt om te weten over deze import
       *[other] { $count } punten om te weten over deze import
    }
import-manuskript-details = Details
import-manuskript-warnings-title = Over deze import
import-manuskript-error-title = Kan het project niet importeren
import-manuskript-nested-too-deep = Het project is niet geïmporteerd: het bestand “{ $part }” is meer dan { $limit } niveaus diep genest. Geen enkel Manuskript-project komt daar in de buurt, en het inlezen zou Skribisto kunnen laten vastlopen. Er is niets geschreven.
import-manuskript-folders-too-deep = Het project is niet geïmporteerd: een van de bestanden staat meer dan { $limit } mappen diep. Geen enkel Manuskript-project komt daar in de buurt, en het inlezen zou Skribisto kunnen laten vastlopen. Er is niets geschreven.
import-manuskript-declares-entities = Het project is niet geïmporteerd: het bestand “{ $part }” declareert XML-entiteiten, wat geen enkel Manuskript-project doet. Het uitvouwen ervan zou meer geheugen kunnen vragen dan de computer heeft en Skribisto laten vastlopen. Er is niets geschreven.
import-manuskript-error-details = Details

import-target-open-title = Dit project is geopend
import-target-open-text = “{ $name }” is geopend in Skribisto. Eroverheen importeren zou het project waaraan u werkt vervangen, en bij de volgende opslag zou dat project weer over de import heen worden geschreven. Kies een andere bestandsnaam, of sluit eerst het project.
import-warnings-announcement = { $result } { $notice }.

export-title = Exporteren
export-close = Sluiten
export-scope-label = Wat
export-format-label = Formaat
export-style-label = Stijl
export-path-label = Bestand
export-browse = Bladeren…
export-show-non-exportable = Niet-exporteerbare items tonen
export-choose-empty = Er is geen project om uit te kiezen.
export-cancel = Annuleren
export-export = Exporteren
export-save-dialog-title = Exporteren naar bestand
export-section-what = Wat te exporteren
export-section-style = Stijl
export-section-destination = Bestemming
export-custom-selection = Aangepaste selectie
export-selected-count = { $count } geselecteerd
export-chip-chapters-none = Hoofdstukken: geen
export-chip-chapters-numbered = Hoofdstukken: genummerd
export-chip-chapters-title = Hoofdstukken: alleen titel
export-chip-chapters-both = Hoofdstukken: nummer + titel
export-chip-scene-break-glyph = Scèneovergang: { $glyph }
export-chip-scene-break-blank = Scèneovergang: lege regel
export-chip-scene-break-none = Scèneovergang: geen
export-chip-major-break-glyph = Grote overgang: { $glyph }
export-chip-major-break-blank = Grote overgang: lege regel
export-chip-major-break-none = Grote overgang: geen
export-chip-major-break-same = Beide soorten overgangen gelijk
export-chip-spacing-single = Regelafstand: enkel
export-chip-spacing-onehalf = Regelafstand: 1½
export-chip-spacing-double = Regelafstand: dubbel
export-chip-notes-included = Met notities
export-chip-notes-excluded = Zonder notities
export-format-docx = Word
export-format-odt = LibreOffice
export-format-html = HTML
export-format-markdown = Markdown
export-format-djot = Djot
export-format-text = Tekst
export-format-latex = LaTeX
export-format-epub = EPUB
export-format-pdf = PDF
export-overwrite-title = Bestaand bestand vervangen?
export-overwrite-text = “{ $name }” bestaat al. Vervangen?
export-progress-title = Exporteren…
export-cancelled = Exporteren geannuleerd
export-done = { $count ->
        [one] { $count } item geëxporteerd
       *[other] { $count } items geëxporteerd
    }
export-open-file = Openen
export-show-in-folder = Tonen in map
export-error-title = Kan niet exporteren
export-error-details = Details

backup-progress-start = Starten…
backup-progress-retention = Oude reservekopieën opruimen…
backup-progress-done = Gereed
backup-progress-destination = Bestemming { $i } van { $n }
backup-details = Details
backup-issues-title = Problemen met reservekopie
backup-failed-title = Reservekopie is mislukt
backup-complete-prune-warning = Reservekopie voltooid ({ $ok } opgeslagen, { $skipped } al actueel). Sommige oude reservekopieën konden niet worden verwijderd

search = Zoeken
search-query-placeholder = Zoeken…
search-replace-placeholder = Vervangen door…
search-replace-toggle = Vervangen aan/uit
search-replace-all = Alles vervangen
search-opt-case = Hoofdlettergevoelig
search-opt-whole-word = Heel woord
search-opt-diacritics = Accentgevoelig
search-scope-body = Hoofdtekst
search-scope-title = Titel
search-scope-synopsis = Synopsis
search-scope-label = Label
search-facet-book = Boeken
search-facet-part = Delen
search-facet-chapter = Hoofdstukken
search-facet-scene = Scènes
search-facet-note = Notities
search-facet-folder = Mappen
search-tip-case = Hoofdlettergevoelig: hoofdletters en kleine letters worden als verschillend behandeld, dus “Elena” en “elena” zijn aparte treffers.
search-tip-whole-word = Heel woord: alleen volledige woorden zoeken, dus “kat” wordt niet gevonden in “kathedraal”.
search-tip-diacritics = Accentgevoelig: letters met accent worden als verschillend behandeld, dus “cafe” komt niet overeen met “café”.
search-tip-body = Hoofdtekst: het proza van scènes en notities doorzoeken.
search-tip-title = Titel: de titels van items in de ordner doorzoeken.
search-tip-synopsis = Synopsis: de samenvatting van elk schrijfitem doorzoeken.
search-tip-label = Label: de korte notitie onder de titel van een item in de ordner doorzoeken.
search-tip-comment = Opmerkingen: de tekst van opmerkingen en hun antwoorden doorzoeken. Een opmerking gaat over het manuscript en maakt er geen deel van uit; daarom heeft ze een eigen schakelaar, en bij vervangen blijven treffers in opmerkingen uitgevinkt totdat u ze aanvinkt.
search-tip-book = Boeken: de boekcontainer en de markeringen voor begin en einde.
search-tip-part = Delen: scheidingen op het niveau van delen.
search-tip-chapter = Hoofdstukken: hoofdstukken, op welke manier het project ze ook opslaat.
search-tip-scene = Scènes: de items die uw proza bevatten.
search-tip-note = Notities: vrije notities.
search-tip-folder = Mappen: gewone mappen om te ordenen, en scheidingslijnen.
search-tip-paratext = Een tekst die bij het boek hoort maar niet bij het verhaal: een voorwoord, een opdracht, een nawoord. Telt nooit mee in het manuscript.
search-tip-preserve-case = Hoofdletters behouden: een vervanging neemt de hoofdletters van de gevonden tekst over, dus “ELENA” wordt “MARTA” en “Elena” wordt “Marta”.
search-tip-replace = Vervangen: het vervangingsveld en Alles vervangen tonen.
search-error = Zoeken mislukt: { $message }
search-no-matches = Geen treffers
search-count =
    { $matches ->
        [one] { $matches } treffer
       *[other] { $matches } treffers
    } in { $items ->
        [one] { $items } document
       *[other] { $items } documenten
    }
search-count-truncated =
    { $matches ->
        [one] { $matches } treffer
       *[other] { $matches } treffers
    } in { $items ->
        [one] { $items } document
       *[other] { $items } documenten
    } (alleen de eerste resultaten)
search-occurrences = { $count }
search-field-body = Hoofdtekst
search-field-title = Titel
search-field-synopsis = Synopsis
search-field-label = Label
search-field-epigraph = Motto
search-field-comment = Opmerking
search-field-comment-reply = Antwoord
search-field-footnote = Voetnoot
search-include-in-replace = Opnemen in Alles vervangen
search-collapse-all = Alle resultaten invouwen
search-replace-here = Dit vervangen
search-dismiss = Uit de resultaten verwijderen
search-undo-dismiss = Het laatst verwijderde resultaat terughalen
search-replace-nothing = (niets)
search-replace-confirm-title = Alle treffers vervangen?
search-replace-confirm-text =
    { $occurrences ->
        [one] { $occurrences } keer
       *[other] { $occurrences } keer
    } “{ $query }” vervangen door “{ $replacement }” in { $items ->
        [one] { $items } document
       *[other] { $items } documenten
    }? U kunt dit achteraf ongedaan maken vanuit de melding.
search-replace-done-title = Vervangen voltooid
search-replace-done =
    { $occurrences ->
        [one] { $occurrences } keer
       *[other] { $occurrences } keer
    } vervangen in { $items ->
        [one] { $items } document
       *[other] { $items } documenten
    }.
search-replace-done-skipped =
    { $occurrences ->
        [one] { $occurrences } keer
       *[other] { $occurrences } keer
    } vervangen in { $items ->
        [one] { $items } document
       *[other] { $items } documenten
    }. { $skipped ->
        [one] { $skipped } veld
       *[other] { $skipped } velden
    } overgeslagen (gewijzigd sinds het zoeken).
search-replace-undo = Ongedaan maken
search-replace-failed-title = Vervangen mislukt
search-replace-skipped-title = Niets vervangen
search-replace-skipped-body =
    { $fields ->
        [one] Deze tekst is
       *[other] { $fields } van deze teksten zijn
    } sinds het zoeken gewijzigd en daarom ongemoeid gelaten. Zoek opnieuw om te zien waar de woorden nu staan.
search-replace-undo-failed-title = Ongedaan maken mislukt
search-preview = Voorbeeld
search-preview-empty = Selecteer een resultaat om het hier te bekijken
search-preview-no-prose = Deze treffer heeft geen bewerkbare tekst
search-preview-prompt = Zoek eerst om hier een voorbeeld te zien.
search-preview-open-search = Zoeken in project
search-preview-footnote-prompt = Deze treffer staat in de eigen tekst van een voetnoot. Open die in het paneel Voetnoten om hem te bekijken en te bewerken.
search-preview-open-footnotes = Voetnoten openen

find-placeholder = Zoeken in dit document…
find-count = { $current } van { $total }
find-no-results = Geen resultaten
find-close = Zoeken sluiten
find-previous = Vorige treffer (Shift+Enter)
find-next = Volgende treffer (Enter)
find-opt-case = Hoofdlettergevoelig
find-opt-whole-word = Heel woord
find-replace-toggle = Vervangen aan/uit (Ctrl+R)
find-replace-placeholder = Vervangen door…
find-replace = Vervangen
find-replace-all = Alles vervangen
find-preserve-case = Hoofdletters behouden

dict-download-title = { $name } downloaden…
dict-download-done = { $name } geïnstalleerd
dict-download-failed = Kan { $name } niet downloaden: { $error }
dict-removed = { $name } verwijderd
dict-accept-first = Accepteer de licentie voordat u { $name } downloadt
settings-dict-tab-installed = Geïnstalleerd
settings-dict-tab-get-more = Meer ophalen
settings-dict-tab-personal = Persoonlijke woorden
dict-installed-empty = Nog geen woordenboeken gevonden op deze computer.
dict-get-more-search = Talen zoeken
dict-system-badge = op uw systeem
dict-unusable-badge = onbruikbaar
dict-download-button = Downloaden
dict-downloading = Bezig met downloaden…
dict-installed-label = Geïnstalleerd
dict-remove = Verwijderen
dict-view-license = Licentie weergeven
dict-approx-size = ~{ $size }
dict-personal-empty = Nog geen persoonlijke woorden in dit project.
dict-personal-add = Toevoegen
dict-personal-placeholder = Een woord toevoegen…
dict-missing-toast = { $count ->
    [one] Dit project gebruikt { $count } woordenboek dat u niet hebt geïnstalleerd
   *[other] Dit project gebruikt { $count } woordenboeken die u niet hebt geïnstalleerd
}
dict-missing-action = Woordenboeken ophalen
dict-license-title = Licentie van { $name }
dict-license-accept = Accepteren en downloaden
dict-license-cancel = Annuleren
dict-license-close = Sluiten
dict-add-button = Woordenboek toevoegen…
dict-add-title = Een woordenboek toevoegen
dict-add-close = Sluiten
dict-add-name = Naam
dict-add-name-placeholder = bijv. Mijn Latijnse woordenboek
dict-add-code = Taalcode
dict-add-code-placeholder = bijv. la of fr-FR-x-custom
dict-add-code-hint = Een korte code naar keuze. Dit kiest u als taal van een document.
dict-add-aff = Affix-bestand (.aff)
dict-add-dic = Woordenlijst (.dic)
dict-add-submit = Toevoegen
dict-add-cancel = Annuleren
dict-add-name-required = Geef het woordenboek een naam.
dict-add-code-required = Vul een taalcode in.
dict-add-code-invalid = Gebruik alleen letters, cijfers en - _ .
dict-add-code-reserved = Dat is de code van een ingebouwd woordenboek. Kies een andere.
dict-add-file-required = Kies een bestand.
dict-add-file-missing = Dat bestand bestaat niet.
dict-add-code-taken = Een woordenboek voor die code is al geïnstalleerd.
dict-add-done = { $name } toegevoegd
dict-add-unusable = Deze bestanden zijn geen bruikbaar woordenboek: { $error }
dict-add-failed = Kan het woordenboek niet toevoegen: { $error }
inspector-export = Exporteren
inspector-exportable = Opnemen in exports
ctx-number = Dit hoofdstuk n&ummeren
ctx-unnumber = Dit hoofdstuk niet n&ummeren
inspector-numbering = Nummering
inspector-numbered = Genummerd
inspector-numbered-tip = Dit hoofdstuk krijgt een plaats in de nummering van het boek. Schakel dit uit voor een proloog, een epiloog of een intermezzo.
inspector-numbered-tip-more = Een ongenummerd hoofdstuk blijft precies zoals het was in het boek: de kop, het proza en het aantal woorden blijven ongewijzigd. Er wordt alleen geen nummer afgedrukt en er wordt er ook geen verbruikt: het hoofdstuk na een proloog is hoofdstuk één, niet hoofdstuk twee. Het uit de export weglaten is een andere schakelaar, hierboven, en die haalt het hoofdstuk helemaal uit het boek.
inspector-apply-to-children = Toepassen op onderliggende items
inspector-milestone = Mijlpaaldatum
inspector-milestone-none = Geen datum
inspector-milestone-clear = Wissen
inspector-tags = Tags
inspector-aliases = Ook bekend als
inspector-dict-language = Taal
inspector-apply-language-to-children = Taal toepassen op onderliggende items
settings-page-language = Taal
settings-field-dict-language = Talen
dict-tradeoff-hint = Elke extra taal accepteert meer woorden, waardoor er minder fouten worden opgemerkt.
lang-inherit-hint = Overgenomen: deze scène gebruikt de talen van het boek of het project.
lang-pill-list = Talen
lang-pill-add = Een taal toevoegen
lang-pill-remove = { $name } verwijderen
lang-pill-mute = Spellingscontrole uitschakelen voor { $name }
lang-pill-unmute = Spellingscontrole inschakelen voor { $name }

settings-page-personal-dictionary = Persoonlijk woordenboek
settings-user-dict-desc = Woorden die u hier toevoegt, gelden overal in de app als correct gespeld en worden nooit meer gemarkeerd.
settings-user-dict-add-placeholder = Een woord toevoegen…
settings-user-dict-add = Woord toevoegen
settings-user-dict-search = Woorden filteren
settings-user-dict-count = { $count ->
    [one] { $count } woord
   *[other] { $count } woorden
}
settings-user-dict-rename = Naam wijzigen
settings-user-dict-remove = Verwijderen
settings-user-dict-empty = Nog geen woorden. Voeg woorden toe die de spellingscontrole moet negeren, of importeer een lijst.
settings-user-dict-import = Importeren…
settings-user-dict-export = Exporteren…
settings-user-dict-import-tip = Woorden toevoegen uit een tekstbestand, één woord per regel. Bestaande woorden blijven behouden.
settings-user-dict-export-tip = De hele lijst opslaan in een tekstbestand, één woord per regel.
settings-user-dict-txt-filter = Tekstbestanden
settings-user-dict-duplicate = Staat al in het woordenboek
settings-user-dict-imported = { $count ->
        [one] { $count } woord
       *[other] { $count } woorden
    } geïmporteerd ({ $duplicates } al aanwezig).
settings-user-dict-import-failed = Kan de woordenlijst niet lezen: { $error }
settings-user-dict-exported = { $count ->
        [one] { $count } woord opgeslagen.
       *[other] { $count } woorden opgeslagen.
    }
settings-user-dict-export-failed = Kan de woordenlijst niet opslaan: { $error }

settings-page-text-replacements = Tekstvervanging
settings-text-repl-desc = Vervang afkortingen terwijl u typt door de volledige tekst: “bvd” wordt “bij voorbaat dank” zodra u een spatie of leesteken typt.
settings-text-repl-enable = Tekstvervangingen gebruiken in dit project
settings-text-repl-disabled-hint = Schakel dit in om afkortingen te definiëren die worden uitgeschreven terwijl u schrijft.
settings-text-repl-add = Regel toevoegen
settings-text-repl-trigger-placeholder = Afkorting
settings-text-repl-replacement-placeholder = Wat het wordt
settings-text-repl-added = “{ $trigger }” toegevoegd
settings-text-repl-duplicate = Er bestaat al een regel voor “{ $trigger }”
settings-text-repl-filter = Regels filteren
settings-text-repl-count = { $n ->
    [one] 1 regel
   *[other] { $n } regels
}
settings-text-repl-row-enabled = Deze regel gebruiken
settings-text-repl-delete = De regel voor { $trigger } verwijderen
settings-text-repl-deleted = De regel voor “{ $trigger }” is verwijderd
settings-text-repl-empty = Nog geen regels.
settings-text-repl-csv-filter = CSV-bestanden
settings-text-repl-import = Importeren…
settings-text-repl-export = Exporteren…
settings-text-repl-imported = { $added } geïmporteerd, { $skipped } overgeslagen
settings-text-repl-exported = { $n ->
        [one] 1 regel geëxporteerd
       *[other] { $n } regels geëxporteerd
    }

editor-menu-no-suggestions = Geen suggesties
editor-menu-add-to-dictionary = “{ $word }” toevoegen aan woordenboek
editor-menu-add-words-to-dictionary = Geselecteerde woorden toevoegen aan woordenboek
editor-dict-added = “{ $word }” toegevoegd aan uw woordenboek.
editor-dict-added-multi = { $count ->
        [one] { $count } woord toegevoegd aan uw woordenboek.
       *[other] { $count } woorden toegevoegd aan uw woordenboek.
    }
toast-undo = Ongedaan maken

undo-superseded-title = Die stap is niet meer de laatste
undo-superseded-body = Sindsdien is er iets anders in dit project veranderd. Ongedaan maken zou dat terugdraaien, dus dat is niet gebeurd. Niets van wat deze stap deed, is teruggedraaid. Gebruik Bewerken ▸ Ongedaan maken om zelf stap voor stap terug te gaan door de geschiedenis.
undo-failed = Ongedaan maken mislukt: { $error }

menu-edit = Be&werken
menu-edit-undo = &Ongedaan maken
menu-edit-redo = Opnie&uw
menu-edit-undo-target = &Ongedaan maken: { $target }
menu-edit-redo-target = Opnie&uw: { $target }
shortcut-name-edit-undo = Ongedaan maken
shortcut-name-edit-redo = Opnieuw
undo-target-typing = typen
undo-target-project = de laatste wijziging aan dit project
prose-history-reset-title = Typgeschiedenis gewist
prose-history-reset-body = { $count ->
        [one] Eén geopende scène is hersteld naar een eerdere toestand, dus de typgeschiedenis ervan geldt niet meer.
       *[other] { $count } geopende scènes zijn hersteld naar een eerdere toestand, dus hun typgeschiedenis geldt niet meer.
    }
undo-target-trash = verplaatsen naar de prullenbak
undo-target-restore = terugzetten uit de prullenbak
undo-target-delete-forever = definitief verwijderen
undo-target-replace-all = vervangen in het hele project
undo-target-import = de documentimport
undo-target-duplicate = dupliceren
undo-target-move = verplaatsen
undo-target-merge = twee scènes samenvoegen
undo-target-split = een scène splitsen
undo-target-promote = het type wijzigen
undo-target-tidy-titles = hoofdstuktitels opschonen
undo-target-import-tags = tags importeren
undo-target-import-templates = notitiesjablonen importeren
undo-target-create = aanmaken
undo-target-remove = verwijderen
undo-target-rename = naam wijzigen
undo-target-edit = die bewerking
menu-edit-find = &Zoeken…
menu-edit-find-next = &Volgende zoeken
menu-edit-find-prev = Vorig&e zoeken
menu-edit-replace = Zoeken en ve&rvangen…
menu-edit-find-in-project = Zoeken in pro&ject…
menu-edit-replace-in-project = Vervangen &in project…
undo-frozen = Ongedaan maken (“Altijd vooruit” is aan)
redo-frozen = Opnieuw (“Altijd vooruit” is aan)
menu-edit-cut = K&nippen
menu-edit-copy = &Kopiëren
menu-edit-paste = &Plakken
menu-edit-paste-plain = Plakken zonder op&maak
menu-edit-select-all = &Alles selecteren
shortcut-name-edit-cut = Knippen
shortcut-name-edit-copy = Kopiëren
shortcut-name-edit-paste = Plakken
shortcut-name-edit-paste-plain = Plakken zonder opmaak
shortcut-name-edit-select-all = Alles selecteren

titlebar-spellcheck-on = Spellingscontrole is ingeschakeld. Klik om te stoppen met controleren (F7)
titlebar-spellcheck-off = Spellingscontrole is uit. Klik om opnieuw te controleren (F7)
menu-spellcheck = &Spelling controleren
menu-comments = Op&merkingen
settings-page-spellcheck = Spellingscontrole
settings-group-spellcheck = Spellingscontrole
settings-spellcheck-enabled = Spelling controleren terwijl ik schrijf
settings-spellcheck-hint = Onderstreept woorden die geen enkel geïnstalleerd woordenboek kent. Als u dit uitschakelt, stopt alle controle, in elk project, totdat u het weer inschakelt. Wilt u slechts één taal niet meer laten controleren, haal dan het vinkje bij die taal weg in het veld Taal van het project of van een item.

menu-trash = &Prullenbak
trash-title = Prullenbak
trash-empty-state = De prullenbak is leeg.
trash-empty-button = Prullenbak legen…
trash-restore = &Terugzetten
trash-restore-to = Terugzetten &naar…
trash-delete-forever = &Definitief verwijderen
trash-restored-ok = { $count } teruggezet.
trash-restore-error = Kan niet terugzetten: { $error }
trash-restore-orphaned = De oorspronkelijke plek van dit item bestaat niet meer. Kies waar het moet worden teruggezet.
trash-restore-no-project = Er is geen project geopend, dus er valt niets terug te zetten.
trash-delete-no-project = Er is geen project geopend, dus er valt niets te verwijderen.
trash-empty-confirm-title = De prullenbak legen?
trash-empty-confirm-text = { $count ->
        [one] { $count } item in de prullenbak wordt definitief verwijderd.
       *[other] { $count } items in de prullenbak worden definitief verwijderd.
    } Ze verdwijnen voorgoed uit de prullenbak. Met Ongedaan maken kunt u ze nog terughalen, zolang u niets anders doet.
trash-emptied-title = Prullenbak geleegd
trash-emptied-body = Alles in de prullenbak is definitief verwijderd.
trash-empty-no-project = Er is geen project geopend, dus er is geen prullenbak om te legen.
trash-delete-forever-confirm-title = Definitief verwijderen?
trash-delete-forever-confirm-text = { $count ->
        [one] { $count } item wordt definitief verwijderd.
       *[other] { $count } items worden definitief verwijderd.
    } Ze verdwijnen voorgoed uit de prullenbak. Met Ongedaan maken kunt u ze nog terughalen, zolang u niets anders doet.
trash-deleted-title = Definitief verwijderd
trash-deleted-body = { $count } definitief verwijderd.
trash-undo = Ongedaan maken
trash-restore-picker-title = Terugzetten naar…
trash-restore-to-confirm-title = Hier terugzetten?
trash-restore-to-confirm-text = “{ $item }” terugzetten in “{ $destination }”?
trash-restore-picker-restore-here = Hier terugzetten
trash-restore-picker-cancel = Annuleren
trash-restore-picker-empty = Nog geen ordners. Maak er eerst een aan.
trash-banner-title = Dit item staat in de prullenbak
trash-banner-description = Het verschijnt niet in de ordner of in exports totdat u het terugzet.
trash-banner-restore = Terugzetten…
trash-tab-tooltip = In de prullenbak

settings-styles-sheet-yes = Ja
settings-styles-sheet-no = Nee
settings-styles-sheet-font = Lettertype
settings-styles-sheet-indent = Inspringing eerste regel
settings-styles-sheet-para-spacing = Alinea-afstand
settings-styles-sheet-page = Paginagrootte
settings-styles-sheet-margins = Marges (B/R/O/L)
settings-styles-sheet-title-page = Titelpagina van het boek
settings-styles-sheet-heading-language = Taal van de koppen
settings-styles-sheet-auto = Volgt de tekst
settings-styles-sheet-digits = Cijfers
settings-styles-sheet-direction = Tekstrichting
settings-styles-sheet-formats = Formaten
settings-styles-sheet-all-formats = Alles

settings-styles-editor-group = Stijlbewerker
settings-styles-editor-missing = Die stijl is niet meer beschikbaar.

settings-styles-page-letter = Letter (VS)
settings-styles-digits-western = Westers (0–9)
settings-styles-digits-eastern-arabic = Oost-Arabisch (٠–٩)
settings-styles-direction-ltr = Links naar rechts
settings-styles-direction-rtl = Rechts naar links

format-dock-title = Opmaak
format-panel-empty = Plaats uw cursor in een scène, notitie of synopsis om opmaakopties te zien.
format-group-marks = Tekst
format-group-block = Alinea
format-group-lists = Lijsten
format-group-tables = Tabel
format-group-breaks = Scèneovergangen
format-superscript = Superscript
format-subscript = Subscript
format-link = Koppeling…
link-dialog-insert-title = Koppeling invoegen
link-dialog-edit-title = Koppeling bewerken
link-dialog-text-label = Tekst
link-dialog-text-placeholder = de woorden die de lezer ziet
link-dialog-url-label = Verwijst naar
link-dialog-url-placeholder = example.com
link-dialog-insert = Invoegen
link-dialog-apply = Toepassen
link-dialog-cancel = Annuleren
link-dialog-remove = Koppeling verwijderen
link-scheme-refused = Alleen web- en e-mailkoppelingen kunnen worden geopend. { $url } is niet geopend.
menu-format-link = Ko&ppeling…
format-clear = Opmaak wissen
format-heading = Kopniveau
format-heading-normal = Normale tekst
format-heading-1 = Kop 1
format-heading-2 = Kop 2
format-heading-3 = Kop 3
format-heading-4 = Kop 4
format-heading-5 = Kop 5
format-heading-6 = Kop 6
format-align-left = Links uitlijnen
format-align-center = Centreren
format-direction-rtl = Alinea van rechts naar links
format-blockquote = Blokcitaat
format-list-bullet = Lijst met opsommingstekens
format-list-numbered = Genummerde lijst
format-indent = Inspringen
format-outdent = Uitspringen
format-table-insert = Tabel invoegen
format-table-row-above = Rij boven invoegen
format-table-row-below = Rij onder invoegen
format-table-col-before = Kolom ervoor invoegen
format-table-col-after = Kolom erna invoegen
format-table-row-delete = Rij verwijderen
format-table-col-delete = Kolom verwijderen
format-table-remove = Tabel verwijderen

menu-format-marks-bold = &Vet
menu-format-marks-italic = &Cursief
menu-format-marks-underline = &Onderstreept
menu-format-marks-strike = &Doorgehaald
menu-format-marks-superscript = &Superscript
menu-format-marks-subscript = Su&bscript
menu-format-marks-clear = Opmaak &wissen
menu-format-heading = &Kop
menu-format-heading-normal = &Normale tekst
menu-format-heading-1 = Kop &1
menu-format-heading-2 = Kop &2
menu-format-heading-3 = Kop &3
menu-format-heading-4 = Kop &4
menu-format-heading-5 = Kop &5
menu-format-heading-6 = Kop &6
menu-format-alignment = &Uitlijning
menu-format-align-left = &Links uitlijnen
menu-format-align-center = &Centreren
menu-format-direction = &Richting
menu-format-direction-auto = &Automatisch
menu-format-direction-ltr = &Links naar rechts
menu-format-direction-rtl = &Rechts naar links
menu-format-blockquote = B&lokcitaat
menu-format-lists = L&ijsten
menu-format-list-bullet = Lijst met &opsommingstekens
menu-format-list-numbered = &Genummerde lijst
menu-format-indent = &Inspringen
menu-format-outdent = &Uitspringen
menu-format-table = &Tabel
menu-format-table-insert = Tabel &invoegen
menu-format-table-2x2 = &2 x 2
menu-format-table-3x3 = &3 x 3
menu-format-table-4x4 = &4 x 4
menu-format-table-row-above = Rij &boven invoegen
menu-format-table-row-below = Rij &onder invoegen
menu-format-table-col-before = Kolom &ervoor invoegen
menu-format-table-col-after = Kolom ern&a invoegen
menu-format-table-row-delete = Rij &verwijderen
menu-format-table-col-delete = &Kolom verwijderen
menu-format-table-remove = Tabel ve&rwijderen

menu-format-typo-marks = Op&maakteken
menu-format-typo-nbsp = &Vaste spatie
menu-format-typo-nbhyphen = Vast &koppelteken
menu-format-typo-soft-hyphen = &Zacht afbreekstreepje
menu-format-typo-nnbsp = Smalle vaste &spatie
menu-format-typo-zwsp = Spatie zonder &breedte
menu-format-typo-word-joiner = &Woordverbinder


menu-about = &Over Skribisto…
about-title = Over Skribisto
about-version = Versie { $version }
about-update-available = Versie { $version } is beschikbaar
about-update-see-notes = Bekijken wat er veranderd is
about-update-flathub = Flathub houdt deze installatie up-to-date.
about-update-distro = Uw distributie houdt deze installatie up-to-date.
about-tagline = Een schrijfprogramma voor romans en ander lang fictiewerk, geschreven in Rust met de Teksilo-toolkit.
about-license = Vrijgegeven onder de GNU General Public License, versie 3.
about-copyright = © 2026 Cyril Jacquet
about-close = Sluiten

native-menu-about = Over { $app }
native-menu-hide = { $app } verbergen
native-menu-quit = { $app } afsluiten
native-menu-settings = Instellingen…
native-menu-window = Venster
native-menu-minimize = Minimaliseren
native-menu-zoom = Zoomen

window-title = { $title } - { $app }
window-title-numbered = { $title } - { $app } (Venster { $n })
window-title-empty = { $app }

first-run-window-title = Welkom
first-run-title = { $app } instellen
first-run-body = { $app } bewaart zijn instellingen los van Skribisto en begint dus leeg. Uw voorkeuren, recente projecten, woordenboeken en vensterindeling kunnen nu worden overgenomen.
first-run-from = Kopiëren van
first-run-to = Kopiëren naar
first-run-copy-note = Er wordt niets verplaatst of verwijderd. Skribisto behoudt al zijn eigen instellingen en werkt precies zoals voorheen.
first-run-import = Instellingen importeren
first-run-start-fresh = Opnieuw beginnen
first-run-import-failed = Sommige instellingen konden niet geïmporteerd worden: { $error }

settings-page-punctuation = Standaardinterpunctie
settings-group-punctuation = Slimme interpunctie
settings-page-work-punctuation = Interpunctie
settings-punctuation-override = Dit project eigen interpunctieregels geven
settings-punctuation-override-hint = Uit: het project volgt de voorkeur van de toepassing. Deze regels worden in het .skrib-bestand meegenomen, zodat een medeauteur die het bestand opent met dezelfde typografie schrijft.
settings-punctuation-dashes = -- omzetten in een en-dash, --- in een em-dash
settings-punctuation-ellipsis = ... omzetten in een beletselteken
settings-punctuation-quotes = Gekrulde aanhalingstekens en apostroffen
settings-quote-style = Aanhalingstekens
settings-quote-style-locale = Taalstandaard
settings-quote-style-curly = “Dubbel”
settings-quote-style-curly-single = ‘Enkel’
settings-quote-style-guillemets = «Guillemets»
settings-quote-style-low-high = „Laag-hoog“
settings-punctuation-spacing = Spatie vóór ; : ! ?
settings-punctuation-spacing-hint = De Franse typografie plaatst een smalle vaste spatie vóór ; ! ? en een volle vóór :. Dit geldt alleen voor tekst in het Frans. De spatie binnen guillemets « » hoort bij de guillemets zelf.
settings-punctuation-sample = Uw taal geeft
settings-punctuation-app-hint = Wat elk project doet, tenzij het eigen regels krijgt in Project ▸ Interpunctie.
settings-punctuation-dialogue = Een alinea die begint met "- " openen met een dialoogstreepje
settings-punctuation-dialogue-hint = Voor talen die gesproken tekst met een streepje aangeven in plaats van met aanhalingstekens: Frans, Spaans, Russisch en andere. Werkt alleen helemaal aan het begin van een alinea.

statusbar-go-to = Ga naar…
go-to-placeholder = De ordner doorzoeken
go-to-no-matches = Geen enkel item komt overeen met die zoekopdracht.
menu-go-to = &Ga naar…

settings-page-distraction-free-themes = Afleidingsvrije thema’s
settings-themes-builtin = Ingebouwde thema’s
settings-themes-builtin-badge = Ingebouwd
settings-themes-user = Mijn thema’s
settings-themes-editor-group = Thema bewerken
settings-themes-editor-empty = Kies een thema onder “Mijn thema’s” om het te bewerken.
settings-themes-use = Gebruiken
settings-themes-duplicate = Dupliceren
settings-themes-edit = Bewerken
settings-themes-delete = Verwijderen
settings-themes-export = Exporteren…
settings-themes-import = Importeren…
settings-themes-copy-suffix = kopie
settings-themes-json-filter = Thema (JSON)
settings-themes-imported = Thema geïmporteerd
settings-themes-import-failed = Kan dat thema niet importeren
settings-themes-exported = Thema geëxporteerd
settings-themes-export-failed = Kan dat thema niet exporteren
settings-themes-low-contrast = laag contrast
settings-themes-low-contrast-band = markering verbergt de tekst
settings-themes-field-name = Naam
settings-themes-field-paper = Pagina
settings-themes-field-ink = Tekst
settings-themes-field-general = Achtergrond
settings-themes-field-widget-text = Tekst van bedieningsbalk
settings-themes-field-caret-band = Markering rond de cursor

statusbar-focus-settings = Instellingen voor afleidingsvrij schrijven
statusbar-focus-manage-themes = Thema’s beheren…

comments-title = Opmerkingen
comments-document-title = Dit document
comments-empty-project = Nog geen opmerkingen in dit project.
comments-empty-document = Geen opmerkingen bij dit document.
comments-filter-all = Alles
comments-filter-open = Open
comments-filter-resolved = Opgelost
comments-filter-orphaned = Verweesd
comments-status-open = Open
comments-status-resolved = Opgelost
comments-status-orphaned = Tekst kwijt
comments-orphan-snippet = (de becommentarieerde tekst bestaat niet meer)
comments-status-unplaced = Geen tekstpositie
comments-unplaced-snippet = (niet verankerd aan tekst)
comments-reply-count = { $count ->
    [one] { $count } antwoord
   *[other] { $count } antwoorden
}
comments-menu-resolve = Oplossen
comments-menu-reopen = Heropenen
comments-menu-delete = Verwijderen
comments-sort-document = In documentvolgorde
comments-sort-newest = Nieuwste eerst
comments-sort-reader = Gegroepeerd per lezer
comments-unsigned-author = Niet ondertekend
comments-agreement = { $count ->
    [one] { $count } lezer
   *[other] { $count } lezers
}
comments-menu-add = Opmerking toevoegen
comments-menu-add-paragraph = Opmerking bij deze alinea
overview-col-comments = Opmerkingen
overview-col-total-comments = Opmerkingen totaal
comments-card-placeholder = Schrijf een opmerking…
comments-card-unknown-author = Onbekende auteur
comments-unsigned-toast = Uw opmerkingen zijn niet ondertekend: er is geen naam voor u ingesteld op deze computer.
comments-unsigned-action = Uw naam instellen
comments-card-reply = Antwoord
comments-card-reply-placeholder = Antwoorden…
comments-card-actions = Acties voor opmerking
comments-reply-actions = Acties voor antwoord
comments-menu-delete-reply = Antwoord verwijderen
comments-menu-delete-all = Alle opmerkingen hier verwijderen
comments-deleted-toast = Opmerking verwijderd
comments-reply-deleted-toast = Antwoord verwijderd
comments-deleted-all-toast = { $count ->
        [one] { $count } opmerking verwijderd
       *[other] { $count } opmerkingen verwijderd
    }
comments-undo = Ongedaan maken

analysis-segment = Analyse
analysis-scope-book = Dit boek wordt geanalyseerd
analysis-run = Analyse uitvoeren
analysis-stale = Gewijzigd sinds deze analyse
analysis-not-run = Nog niet geanalyseerd.
analysis-running = Het manuscript wordt gelezen…
analysis-failed = De analyse kon niet worden afgerond.
analysis-no-scenes = Nog geen scènes in dit boek.

analysis-shape = Vorm

analysis-arrivals = Herkomst
analysis-arrivals-explainer = Hoe tekst in dit project terechtkwam terwijl het geopend was. Het zegt langs welke weg de tekens binnenkwamen, en helemaal niets over wie ze schreef: een schrijver die elders een opzet maakt en die plakt, heeft geplakt, en wie dicteert, heeft gedicteerd. Er is hier geen getal om naar te streven.
analysis-arrivals-scope = Dit hele project, niet alleen dit boek.
analysis-arrivals-session = Sinds u het opende. Als u het project sluit, begint de telling opnieuw.
analysis-arrivals-nothing = In deze sessie is nog geen tekst binnengekomen.
analysis-arrivals-typed = Getypt
analysis-arrivals-pasted = Geplakt
analysis-arrivals-dictated = Gedicteerd
analysis-arrivals-imported = Geïmporteerd
analysis-arrivals-programmatic = Voor u ingevoegd
analysis-arrivals-count = { $count ->
    [one] { $count } teken
   *[other] { $count } tekens
}
analysis-arrivals-none = geen

analysis-words-per-scene = Woorden per scène
analysis-median-words = De mediane scène van dit boek telt { $count ->
        [one] { $count } woord.
       *[other] { $count } woorden.
    }
analysis-median-line = Mediaan: { $count ->
    [one] { $count } woord
   *[other] { $count } woorden
}
analysis-dialogue = Dialoog
analysis-dialogue-unsupported = Dialoog wordt voor deze taal nog niet gemeten.

analysis-footnote-words = Woorden in voetnoten
analysis-footnote-words-count = { $count ->
        [one] { $count } woord staat in de voetnoten van dit boek, apart gehouden van het totaal van het manuscript.
       *[other] { $count } woorden staan in de voetnoten van dit boek, apart gehouden van het totaal van het manuscript.
    }
analysis-footnote-words-pending = Voetnoten tellen…

analysis-ignore-empty = Teksten waarin nog niets geschreven is negeren
analysis-empty-hidden = { $count } lege { $count ->
        [one] tekst
       *[other] teksten
    } verborgen.
analysis-all-texts-empty = Alle teksten in dit boek zijn nog leeg.

binder-filter-count = { $shown } van { $total } getoond
binder-filter-clear = Wissen
binder-filter-none = Niets komt overeen met “{ $query }”.

settings-paratext-intro = Het voor- en nawerk waarmee een nieuw project kan beginnen. Elke structuur hoort bij een uitgeefstraditie, en de paginatitels zijn in de taal van die traditie geschreven. Hernoem ze gerust zodra een project is aangemaakt.
settings-paratext-structures = Structuren
settings-paratext-broken = Kan niet worden gelezen
settings-paratext-edit = Bewerken
settings-paratext-duplicate = Dupliceren
settings-paratext-delete = Verwijderen
settings-paratext-new = Nieuwe structuur
settings-paratext-save = Opslaan
settings-paratext-editor-hint = Een naam, de pagina’s die vóór het manuscript worden aangemaakt en de pagina’s die erna worden aangemaakt. Waar ze uiteindelijk staan, bepaalt u zelf: zodra het project bestaat, zijn het gewone items.


image-insert = Af&beelding invoegen…
image-no-project = Open een project voordat u een afbeelding invoegt.
image-choose-title = Kies een afbeelding
image-filter-label = Afbeeldingen
image-large-title = Dit is een grote afbeelding
image-large-text =
    { $name } is { $megapixels } megapixel ({ $width }×{ $height }).
    Als u de afbeelding houdt zoals ze is, wordt uw oorspronkelijke bestand in het
    project opgeslagen: het gaat mee in elke reservekopie en elke export. Optimaliseren
    slaat in plaats daarvan een kleinere kopie op, met hoogstens 2560 pixels aan de langste zijde.
image-large-keep = Origineel bewaren
image-large-downscale = Optimaliseren
image-large-remember = Dit voortaan doen, niet meer vragen
image-not-recorded = De afbeelding is opgeslagen, maar kon niet in het project worden vastgelegd.
image-describe-title = Afbeelding beschrijven
image-describe-explain = Wat de afbeelding toont, voor een lezer die haar niet kan zien. Dit maakt geen deel uit van het manuscript: het wordt nooit als proza geteld, doorzocht of geëxporteerd.
image-describe-placeholder = een vuurtoren tegen een grijze lucht
image-resize-title = Afbeeldingsgrootte wijzigen
image-resize-explain = Een percentage van de grootte waarop ze nu wordt weergegeven. Bij 100 blijft ze zoals ze is.
image-resize-invalid = Voer een getal in tussen 1 en 1000.
image-menu-describe = Afbeelding &beschrijven…
image-menu-resize = &Grootte van afbeelding wijzigen…
image-menu-reset-size = &Oorspronkelijke grootte

cover-choose = Omsla&g van het boek…
cover-clear = Omslag verwij&deren
cover-choose-title = Kies een omslag
cover-set = De omslag is ingesteld.
cover-cleared = De omslag is verwijderd. De afbeelding staat nog in het project.
card-label = Label
card-exportable = Geëxporteerd
card-numbered = Genummerd
card-created = Aangemaakt
card-modified = Gewijzigd
card-yes = ja
card-no = nee
card-type = Type
card-position = Positie
card-children = Onderliggende items
card-goal = Doel
card-words = Woorden
card-synopsis = Synopsis
card-point-of-view = Vertelperspectief
card-aliases = Ook bekend als

export-orphan-footnotes-title = Voetnoten zonder referentie
export-comments-dropped =
    { $count ->
        [one] Eén opmerking kon niet in de geëxporteerde tekst worden geplaatst en werd weggelaten.
       *[other] { $count } opmerkingen konden niet in de geëxporteerde tekst worden geplaatst en werden weggelaten.
    }
export-orphan-footnotes =
    { $count ->
        [one] Eén voetnoot wordt nergens meer in het manuscript aangehaald. De tekst ervan verschijnt niet in het geëxporteerde boek.
       *[other] { $count } voetnoten worden nergens meer in het manuscript aangehaald. Hun tekst verschijnt niet in het geëxporteerde boek.
    }


menu-footnotes = Voet&noten
footnotes-title = Voetnoten
footnotes-insert = &Voetnoot invoegen
footnotes-empty = Nog geen voetnoten. Plaats de cursor in een scène en gebruik dan + hierboven, of Ctrl+Alt+F.
footnotes-filter-all = Alles
footnotes-filter-document = Dit document
footnotes-filter-orphaned = Verweesd
footnotes-orphaned = Niets verwijst nog naar deze voetnoot
footnotes-untitled-home = Naamloos
footnotes-body-placeholder = de voetnoot zelf
footnotes-insert-tooltip = Een voetnoot invoegen bij de cursor (Ctrl+Alt+F)
footnotes-actions = Voetnootacties
footnotes-delete = Voetnoot en verwijzing &verwijderen
footnotes-no-project = Open een project voordat u een voetnoot invoegt.
footnotes-no-caret = Plaats de cursor in de tekst van een scène om daar een voetnoot in te voegen.
footnotes-not-created = De voetnoot kon niet aan het project worden toegevoegd.
footnotes-deleted-toast = Voetnoot verwijderd
footnotes-undo-delete = Ongedaan maken

import-document-title = Documenten importeren
import-document-close = Sluiten
import-document-step-files = Bestanden
import-document-step-review = Controleren
import-document-step-destination = Bestemming
import-document-step-reconcile = Samenvoegen
import-document-reconcile-hint = Sommige van deze items hebt u al. Geef aan wat er met elk moet gebeuren.
import-document-reconcile-all-new = Niets in dit bestand komt overeen met uw project: elk item wordt als nieuw toegevoegd.
import-document-duplicate-returns = Deze bestanden zijn meerdere teruggekomen kopieën van hetzelfde manuscript. Importeer ze een voor een: het vergelijken legt telkens maar één teruggekomen kopie naast uw project, dus als u er meerdere tegelijk importeert, wordt een tweede kopie van het boek toegevoegd in plaats van samengevoegd.
import-document-hunk-added = Deze nieuwe alinea overnemen
import-document-hunk-removed = Deze alinea laten vallen
import-document-hunk-changed = Deze herschreven versie overnemen
import-document-col-stray-prose = De tekst ervan
import-document-stray-as-paratext = Als paratekst bewaren
import-document-stray-discard = De tekst laten vallen
import-document-col-current = In uw project
import-document-col-incoming = In dit bestand
import-document-col-status = Status
import-document-col-action = Wat te doen
import-document-status-identical = Hetzelfde
import-document-status-editor-edited = Zij hebben het bewerkt
import-document-status-you-edited = U hebt het bewerkt
import-document-status-conflict = Beiden hebben het bewerkt
import-document-status-different = Verschilt
import-document-status-new = Nieuw
import-document-status-missing = Niet in dit bestand
import-document-status-moved = Verplaatst
import-document-action-comments-only = Alleen opmerkingen
import-document-action-take-import = Deze versie overnemen
import-document-action-keep-current = De mijne behouden
import-document-action-create-new = Als nieuw toevoegen
import-document-action-ignore = Overslaan
import-document-compare = Vergelijken
import-document-compare-legend = Uw versie vergeleken met die van dit bestand. Alleen-lezen.
import-document-compare-close = Sluiten
import-document-drop-title = Documenten hier neerzetten
import-document-drop-hint = Markdown (.md), Word (.docx), OpenDocument (.odt) en platte tekst (.txt)
import-document-browse = Bladeren…
import-document-move-up = Omhoog verplaatsen
import-document-move-down = Omlaag verplaatsen
import-document-remove-file = Verwijderen
import-document-file-count = { $count ->
    [one] 1 bestand
   *[other] { $count } bestanden
}
import-document-no-files = Nog geen bestanden gekozen.
import-document-col-included = Importeren
import-document-col-title = Titel
import-document-col-type = Type
import-document-col-words = Woorden
import-document-col-breaks = Overgangen
import-document-col-comments = Opmerkingen
import-document-col-footnotes = Voetnoten
import-document-col-epigraph = Motto
import-document-col-source = Bron
import-document-level-rules = Kopniveaus
import-document-level-n = Kop { $level }
import-document-add-top-level = Hoogste niveau toevoegen
import-document-add-top-level-tooltip = Een boek invoegen boven elk geanalyseerd item, voor hoofdstukbestanden zonder boekkop
import-document-destination = Bestemming
import-document-destination-hint = Kies een ordner of item: nieuwe items komen in een map terecht, of na een scène.
import-document-destination-empty = Nog geen ordners. Maak er eerst een aan.
import-document-plan-empty = Nog niets te importeren.
import-document-summary = { $rows ->
        [one] 1 item
       *[other] { $rows } items
    } · { $breaks ->
        [one] 1 scèneovergang
       *[other] { $breaks } scèneovergangen
    }
import-document-back = Terug
import-document-cancel = Annuleren
import-document-analyse = Volgende
import-document-analysing = Documenten lezen…
import-document-step-analysing = Lezen
import-document-cancel-analysis = Stop met lezen
import-document-analyse-failed = De documenten konden niet worden gelezen.
import-document-details = Details
import-document-import = Importeren
import-document-done = { $count ->
        [one] 1 item geïmporteerd
       *[other] { $count } items geïmporteerd
    }
import-document-undo = Ongedaan maken
import-diagnostic-file-unreadable = “{ $path }” kon niet worden gelezen: { $detail }. De andere bestanden worden wel geïmporteerd.
import-diagnostic-nested-too-deep = “{ $path }” is niet gelezen: de inhoud is meer dan { $limit } niveaus diep genest. Geen echt document komt daar in de buurt, en het inlezen zou Skribisto kunnen laten vastlopen, dus het wordt weggelaten. De andere bestanden worden wel geïmporteerd.
import-diagnostic-archive-too-large = “{ $path }” is niet gelezen: het is een gecomprimeerd bestand dat zo is gemaakt dat het uitgepakt veel groter wordt, wat het geheugen kan uitputten en Skribisto laten vastlopen, dus het wordt weggelaten. De andere bestanden worden wel geïmporteerd.
import-diagnostic-lossy-decode = { $count ->
        [one] Eén teken in “{ $path }” kon niet worden gedecodeerd. Sla het bestand opnieuw op als UTF-8 om het te behouden.
       *[other] { $count } tekens in “{ $path }” konden niet worden gedecodeerd. Sla het bestand opnieuw op als UTF-8 om ze te behouden.
    }
import-diagnostic-decoded-from-bom = “{ $path }” is gedecodeerd als { $detail }, niet als UTF-8.
import-diagnostic-empty-file = “{ $path }” is leeg.
import-diagnostic-no-headings = “{ $path }” heeft geen koppen en komt dus binnen als één item.
import-diagnostic-unsupported-format = “.{ $detail }”-bestanden kunnen niet worden gelezen, dus “{ $path }” is overgeslagen.
import-diagnostic-front-matter-not-flat = Metagegevenskop (front matter) in “{ $path }”: “{ $detail }” is geen eenvoudige waarde en is overgeslagen.
import-diagnostic-footnotes-degraded = { $count ->
        [one] Eén voetnoot in “{ $path }” komt binnen als platte tekst: voetnoten worden niet uit Markdown gelezen.
       *[other] { $count } voetnoten in “{ $path }” komen binnen als platte tekst: voetnoten worden niet uit Markdown gelezen.
    }
import-diagnostic-footnote-not-carried = { $count ->
        [one] Eén voetnoot in “{ $path }” kon niet worden overgenomen. De andere wel.
       *[other] { $count } voetnoten in “{ $path }” konden niet worden overgenomen. De andere wel.
    }
import-diagnostic-raw-html-dropped = { $count ->
        [one] Eén blok onbewerkte HTML in “{ $path }” is weggelaten.
       *[other] { $count } blokken onbewerkte HTML in “{ $path }” zijn weggelaten.
    }
import-diagnostic-nested-break-dropped = { $count ->
        [one] Eén scèneovergang binnen een citaat of lijst in “{ $path }” is weggelaten. Alleen een overgang op een eigen regel blijft behouden.
       *[other] { $count } scèneovergangen binnen citaten of lijsten in “{ $path }” zijn weggelaten. Alleen een overgang op een eigen regel blijft behouden.
    }
import-diagnostic-image-not-ingested = “{ $path }” verwijst naar de afbeelding “{ $detail }”. De verwijzing komt binnen als tekst; de afbeelding zelf wordt niet gekopieerd.
import-diagnostic-duplicate-title = “{ $title }” komt { $count } keer voor. Als u deze bestanden al eerder hebt geïmporteerd, worden ze nu gedupliceerd.
import-diagnostic-heading-level-jump = “{ $title }” springt van kopniveau { $from } naar { $to }; het wordt één niveau onder het bovenliggende item geplaatst.
import-diagnostic-illegal-combination = “{ $title }” bevat proza, maar een { $kind } kan geen proza bevatten. De import wordt aangehouden totdat u het type wijzigt of het item weglaat. Anders wordt er helemaal niets geïmporteerd.
import-diagnostic-tracked-changes-flattened = { $path } zat midden in een revisie: { $count ->
        [one] { $count } bijgehouden wijziging is geaccepteerd
       *[other] { $count } bijgehouden wijzigingen zijn geaccepteerd
    } en verwijderingen zijn doorgevoerd. Dat is de definitieve tekst. Wilt u in plaats daarvan iets van uw eigen formulering behouden, gebruik dan Vergelijken in de laatste stap.
import-diagnostic-tracked-changes-flattened-by = { $path } zat midden in een revisie: { $count ->
        [one] { $count } bijgehouden wijziging van { $names } is geaccepteerd
       *[other] { $count } bijgehouden wijzigingen van { $names } zijn geaccepteerd
    } en verwijderingen zijn doorgevoerd. Dat is de definitieve tekst. Wilt u in plaats daarvan iets van uw eigen formulering behouden, gebruik dan Vergelijken in de laatste stap.
import-diagnostic-text-box-dropped = { $count ->
        [one] { $path } bevat { $count } tekstvak. De tekst ervan staat buiten de doorlopende tekst van het document, dus waar die in een manuscript thuishoort, valt niet te zeggen. Het wordt niet geïmporteerd.
       *[other] { $path } bevat { $count } tekstvakken. Hun tekst staat buiten de doorlopende tekst van het document, dus waar die in een manuscript thuishoort, valt niet te zeggen. Ze worden niet geïmporteerd.
    }
import-diagnostic-embedded-object-dropped = { $count ->
        [one] { $path } bevat { $count } ingesloten object: een grafiek, een formule of iets dergelijks. Een manuscript heeft niets waarin het past.
       *[other] { $path } bevat { $count } ingesloten objecten: een grafiek, een formule of iets dergelijks. Een manuscript heeft niets waarin ze passen.
    }
import-diagnostic-field-flattened = { $count ->
        [one] { $path } bevat { $count } veld: een paginanummer, een kruisverwijzing, een datum. Het behoudt de tekst die het het laatst toonde en wordt niet meer bijgewerkt.
       *[other] { $path } bevat { $count } velden: een paginanummer, een kruisverwijzing, een datum. Elk veld behoudt de tekst die het het laatst toonde en wordt niet meer bijgewerkt.
    }
import-diagnostic-unknown-style-level = { $path } gebruikt de stijl “{ $detail }”, die op een kop lijkt maar geen niveau noemt. Die alinea’s worden als proza geïmporteerd in plaats van op een geraden niveau.
import-diagnostic-comment-unanchored = De opmerking “{ $detail }” in { $path } kon niet worden gekoppeld aan de woorden waarover ze ging. Ze blijft bij haar item, waar u haar kunt verplaatsen.
import-diagnostic-comment-not-carried = De opmerking “{ $detail }” in { $path } wordt niet geïmporteerd, omdat niets in het bestand binnenkomt als tekst waaraan ze kan worden gekoppeld. Kopieer haar uit het bestand als u haar nodig hebt.
import-diagnostic-comment-replies-flattened = { $count ->
        [one] { $count } antwoord in { $path } verwees naar een opmerking die niet in het bestand staat, dus het komt binnen als zelfstandige opmerking.
       *[other] { $count } antwoorden in { $path } verwezen naar een opmerking die niet in het bestand staat, dus ze komen binnen als zelfstandige opmerkingen.
    }
import-diagnostic-prose-not-verbatim = { $count ->
        [one] { $count } alinea in { $path } kon niet met de opmaak precies zoals geschreven worden overgenomen, dus ze komt binnen als platte tekst. Vergelijk ze met het origineel.
       *[other] { $count } alinea’s in { $path } konden niet met de opmaak precies zoals geschreven worden overgenomen, dus ze komen binnen als platte tekst. Vergelijk ze met het origineel.
    }
import-diagnostic-styled-spaces-not-carried = { $count ->
        [one] In { $path } was { $count } reeks lege ruimte onderstreept of doorgehaald, bijvoorbeeld een lijn om in te vullen. Skribisto bewaart zulke lijnen niet op alleen spaties, dus ze komt binnen als gewone spaties, of helemaal niet waar ze een alinea begint of eindigt.
       *[other] In { $path } waren { $count } reeksen lege ruimte onderstreept of doorgehaald, bijvoorbeeld lijnen om in te vullen. Skribisto bewaart zulke lijnen niet op alleen spaties, dus ze komen binnen als gewone spaties, of helemaal niet waar ze een alinea beginnen of eindigen.
    }
import-diagnostic-list-nesting-flattened = { $count ->
        [one] { $count } lijstitem in { $path } was meer dan { $limit } niveaus diep genest. Het komt binnen op niveau { $limit }, het diepste dat Skribisto bewaart, met zijn woorden en opmaak.
       *[other] { $count } lijstitems in { $path } waren meer dan { $limit } niveaus diep genest. Ze komen binnen op niveau { $limit }, het diepste dat Skribisto bewaart, met hun woorden en opmaak.
    }
import-diagnostic-table-read-as-paragraphs = { $count ->
        [one] { $count } tabel in { $path } heeft een rij die veel breder is dan de rest, te scheef om als raster te worden opgemaakt. De cellen komen binnen als alinea’s, de een na de ander, met hun woorden en opmaak.
       *[other] { $count } tabellen in { $path } hebben een rij die veel breder is dan de rest, te scheef om als raster te worden opgemaakt. Hun cellen komen binnen als alinea’s, de een na de ander, met hun woorden en opmaak.
    }
import-diagnostic-spaces-shortened = { $count ->
        [one] { $count } reeks spaties in { $path } was langer dan { $limit } spaties en komt ingekort tot { $limit } binnen. De woorden eromheen zijn er allemaal.
       *[other] { $count } reeksen spaties in { $path } waren langer dan { $limit } spaties en komen ingekort tot { $limit } binnen. De woorden eromheen zijn er allemaal.
    }
import-diagnostic-epigraph-not-carried = “{ $title }” begint met een motto, maar een { $kind } kan er geen bevatten. Het citaat blijft in plaats daarvan bovenaan de tekst staan.
import-diagnostic-epigraph-placement-ambiguous = Een motto staat tussen “{ $title }” en “{ $below }” en kan bij beide horen. Het is aan “{ $title }” toegekend, waar een motto meestal staat.
import-document-diagnostics = { $errors ->
        [0] { $warnings ->
                [one] 1 punt om te weten
               *[other] { $warnings } punten om te weten
            }
       *[other] { $errors ->
                [one] 1 bestand kon niet worden gelezen
               *[other] { $errors } bestanden konden niet worden gelezen
            }
    }
import-document-diagnostics-none = Niets te melden.

versions-title = Versies
versions-scope-synopsis = Synopsis
versions-scope-prose = Tekst
versions-loading = Uw reservekopieën worden doorzocht…
versions-empty = Nog geen eerdere versie hiervan
versions-error = Kan uw reservekopieën niet lezen. Er is niets verloren gegaan, maar deze lijst is mogelijk onvolledig
versions-did-not-exist = Bestond nog niet op { $date }
versions-deleted-after = Verwijderd enige tijd na { $date }
versions-unreadable = { $count ->
        [one] { $count } reservekopie kon niet worden gelezen
       *[other] { $count } reservekopieën konden niet worden gelezen
    }
versions-thinned = { $count ->
        [one] Oudere versies worden met de tijd uitgedund. De eigen geschiedenis van het project heeft al { $count } eerdere toestand van deze tekst laten vallen.
       *[other] Oudere versies worden met de tijd uitgedund. De eigen geschiedenis van het project heeft al { $count } eerdere toestanden van deze tekst laten vallen.
    }
versions-source-backup = Uit een reservekopie
versions-source-project = Uit de eigen geschiedenis van het project
versions-list-caption = Eén regel per wijziging, niet per reservekopie
versions-pick-a-version = Kies een versie om te zien wat er veranderd is
versions-earliest = De vroegste vastgelegde versie. Er is niets ouders om ze mee te vergelijken.
versions-no-change = Er is niets veranderd in dit deel van het item
versions-formatting-only = Alleen de opmaak is hier veranderd. De woorden zijn hetzelfde
versions-show-unchanged = Ongewijzigd tonen
versions-hide-unchanged = Ongewijzigd verbergen
versions-next-change = Volgende wijziging
versions-near = bij “{ $text }”
versions-words-added = { $count ->
    [one] { $count } woord toegevoegd
   *[other] { $count } woorden toegevoegd
}
versions-words-removed = { $count ->
    [one] { $count } woord verwijderd
   *[other] { $count } woorden verwijderd
}
versions-blocks-moved = { $count ->
        [one] { $count } alinea verplaatst
       *[other] { $count } alinea’s verplaatst
    }
versions-pin = De reservekopie waaruit deze versie komt vastzetten. Automatisch opruimen verwijdert haar dan nooit
versions-unpin = De reservekopie waaruit deze versie komt losmaken. Automatisch opruimen kan haar dan weer verwijderen
versions-pinned-only = Alleen vastgezette versies tonen
versions-pin-note = Alleen versies uit een reservekopie kunnen worden vastgezet. Vastzetten bewaart een bestand, en de eigen geschiedenis van het project is geen bestand.
versions-pinned-empty = Hier is nog niets vastgezet
versions-pinned-empty-log = Hier is nog niets vastgezet. Alleen versies uit een reservekopie kunnen worden vastgezet. Vastzetten bewaart een bestand, en de eigen geschiedenis van het project is geen bestand.
versions-range-filter = Alleen versies tussen twee datums tonen
versions-filtered-empty = Geen enkele versie komt overeen met de filters die u hebt ingesteld
versions-clear-filters = De filters wissen
versions-last-30-days = Afgelopen 30 dagen
versions-restore-button = Deze versie herstellen
versions-restore-confirm-title = Deze tekst vervangen door de versie van { $date }?
versions-restore-confirm-text = Wat u nu hebt, wordt vervangen door de tekst die dit item op { $date } had.
versions-restore-confirm-with-comments = Wat u nu hebt, wordt vervangen door de tekst die dit item op { $date } had. { $count ->
        [one] { $count } opmerking is in de huidige tekst verankerd en kan verweesd raken.
       *[other] { $count } opmerkingen zijn in de huidige tekst verankerd en kunnen verweesd raken.
    }
versions-restore-confirm-undo-note = Eerst wordt een reservekopie gemaakt, en Ctrl+Z maakt dit in één stap ongedaan.
versions-restored-toast = De versie van { $date } is hersteld
versions-undo = Ongedaan maken
versions-restore-row-gone = Dat item staat niet meer in dit project
versions-restore-no-home = Het type van dit item is sindsdien veranderd, en de oude tekst kan er nergens in terecht
versions-restore-no-safety-copy = Uw veiligheidskopie is niet gemaakt, dus er is niets gewijzigd
versions-restore-failed = Het herstel is mislukt: { $error }
versions-restore-backup-busy = Er wordt al een reservekopie gemaakt. Probeer het zo meteen opnieuw
versions-restore-in-backup-file = U bekijkt een reservekopiebestand; open het project zelf om erin te herstellen
versions-restore-no-project = Er is geen project geopend
versions-recreate-button = Dit terughalen…
versions-recreate-picker-title = Waar moet het heen?
versions-recreate-picker-empty = Dit project heeft geen ordner om het in te zetten
versions-recreate-picker-confirm = Hier terughalen
versions-recreate-picker-cancel = Annuleren
versions-recreate-untitled = dit item
versions-recreate-confirm-title = “{ $item }” terughalen?
versions-recreate-confirm-text = Het wordt toegevoegd aan { $destination }, met de tekst die het op { $date } had.
versions-recreate-confirm-undo-note = Ongedaan maken, in de melding die volgt, haalt het meteen weer weg.
versions-recreated-toast = “{ $item }” staat weer in uw project
versions-recreated-partial-toast = { $count ->
        [one] “{ $item }” is terug, maar een van de teksten kon niet worden gelezen
       *[other] “{ $item }” is terug, maar { $count } van de teksten konden niet worden gelezen
    }
versions-recreate-already-here = Dat item staat al in uw project
versions-recreate-no-destination = Kies een plek in de ordner om het neer te zetten
versions-recreate-unreadable = Die reservekopie kon niet worden gelezen, dus er is niets toegevoegd
versions-recreate-failed = Kan het niet terughalen: { $error }
versions-changed-percent = { $percent }% hiervan is veranderd
versions-hidden-paragraphs = { $count ->
        [one] … { $count } ongewijzigde alinea …
       *[other] … { $count } ongewijzigde alinea’s …
    }

timeline-title = Terug in de tijd
timeline-coverage = { $count ->
        [one] { $count } versie vastgelegd, teruggaand tot { $oldest }
       *[other] { $count } versies vastgelegd, teruggaand tot { $oldest }
    }
timeline-loading = Het verleden van uw project wordt doorzocht…
timeline-empty = Er is nog geen versie van dit project vastgelegd
timeline-no-changes = Sindsdien is er niets veranderd
timeline-no-text-changes = Sindsdien is er geen tekst veranderd
timeline-slider-label = Vastgelegde versie
timeline-series-name = Projectgrootte
timeline-bars-caption = Elke staaf is een vastgelegde versie, zo hoog als het project toen groot was. De gemarkeerde is de versie die u bekijkt.
timeline-bars-caption-periods = Te veel versies om ze een voor een te tonen, dus elke staaf is een { $period }, zo hoog als het project aan het eind ervan groot was.
timeline-unit-hour = uur
timeline-unit-day = dag
timeline-unit-week = week
timeline-unit-month = maand
timeline-range-filter = Alleen versies tussen twee datums tonen
timeline-range-empty = In die periode is geen versie vastgelegd
timeline-open-period = Deze periode openen
timeline-show-all = De hele geschiedenis tonen
timeline-changed-since = { $count ->
    [one] { $count } item verschilt tussen { $date } en uw project nu
   *[other] { $count } items verschillen tussen { $date } en uw project nu
}
timeline-kind-added = Geschreven sinds
timeline-kind-removed = Niet meer in het project
timeline-kind-changed = Bewerkt sinds
timeline-kind-moved = Verplaatst sinds
timeline-not-yet-written = Dit bestond op dat moment nog niet
timeline-no-text-of-its-own = Dit heeft geen eigen tekst. Het is een kop voor wat erin zit
timeline-reader-close = Sluiten
timeline-reader-stamp = Zoals op { $date }
timeline-reader-loading = De vastgelegde versie wordt geopend…
timeline-reader-unreadable = Deze vastlegging kon niet worden gelezen. De reservekopie is misschien verplaatst of verwijderd, of staat op een schijf die niet is aangesloten.
timeline-reader-compared = Zoals het was op { $date }, naast wat er nu staat
timeline-reader-diff-legend = Doorgehaald is sindsdien verdwenen; onderstreept is toegevoegd.
timeline-reader-view-label = Wat tonen
timeline-reader-view-diff = Wijzigingen
timeline-reader-view-text = Tekst
timeline-reader-deleted = Dit staat niet meer in uw project. U kunt het hier lezen en eruit kopiëren.
timeline-prose-only-record = Dit punt komt uit de eigen geschiedenis van het project, die alleen tekst bewaart, dus wat er sindsdien is verwijderd of verplaatst, kan niet worden getoond. Kies daarvoor een reservekopie.

settings-sec-extensions = Extensies

settings-group-games-forward = Altijd vooruit
settings-games-forward-toggle = “Altijd vooruit” spelen
settings-games-forward-hint =
    Zolang u speelt, kan niets van wat u hebt geschreven worden teruggenomen: Backspace, Delete,
    Knippen, slepen en neerzetten en Ongedaan maken zijn allemaal uitgeschakeld in de onderdelen die u hieronder kiest.
    U kunt nog steeds vrij typen, plakken, opmaken en navigeren. Het concept groeit alleen maar.
settings-games-session-warning =
    Dit is een keuze per sessie: ze wordt nooit opgeslagen. Het project sluiten of Skribisto
    afsluiten beëindigt het spel altijd, en alles wat u tijdens het spel schreef, kan op dat
    moment weer ongedaan worden gemaakt.
settings-group-games-scope = Waar het geldt
settings-games-in-prose = Proza van het manuscript
settings-games-in-synopsis = Synopsissen
settings-games-scope-hint =
    Opmerkingen, voetnoten en titels worden nooit bevroren: daar noteert u de verbetering
    die u zichzelf net hebt verboden.
settings-games-inert-warning =
    “Altijd vooruit” staat aan, maar geldt nergens voor. Vink hierboven minstens één onderdeel aan,
    anders verandert het niets aan uw schrijven.

games-title = Schrijfspellen
games-forward-name = Altijd vooruit
games-forward-blurb =
    Schrijf zonder iets terug te nemen. Verwijderen, knippen en ongedaan maken zijn
    uitgeschakeld zolang u speelt. Schrijf de volgende zin in plaats van de vorige te verbeteren.
games-forward-playing = Bezig: verwijderen is uitgeschakeld
games-forward-idle = Niet bezig
games-session-note = Eindigt wanneer u het project sluit.
games-scope-prose-and-synopsis = Geldt voor uw proza en synopsissen.
games-scope-prose = Geldt voor uw proza.
games-scope-synopsis = Geldt voor uw synopsissen.
games-scope-nothing = Geldt nog nergens voor. Kies een onderdeel in Instellingen.
games-settings-link = Instellingen voor schrijfspellen…
statusbar-games-forward = Altijd vooruit
statusbar-games-forward-tooltip =
    “Altijd vooruit” staat aan: verwijderen, knippen en ongedaan maken zijn uitgeschakeld
    zolang u schrijft. Klik om te stoppen met spelen.
goal-progress-words = { $g ->
        [one] { $count } van { $goal } woord
       *[other] { $count } van { $goal } woorden
    }
goal-progress-characters = { $g ->
        [one] { $count } van { $goal } teken
       *[other] { $count } van { $goal } tekens
    }
goal-count-words = { $n ->
    [one] { $count } woord
   *[other] { $count } woorden
}
goal-count-characters = { $n ->
    [one] { $count } teken
   *[other] { $count } tekens
}
goal-unit-words = Woorden
goal-unit-characters = Tekens

inspector-goal = Doel
inspector-goal-none = Geen doel

overview-col-goal = Doel
overview-excluded-from-export = Weggelaten uit de export, dus telt het voor geen enkel totaal mee

goal-subtree-total-words = { $count ->
        [one] { $items } doel hierin komt samen op { $words } woorden
       *[other] { $items } doelen hierin komen samen op { $words } woorden
    }
goal-subtree-total-characters = { $count ->
        [one] { $items } doel hierin komt samen op { $words } tekens
       *[other] { $items } doelen hierin komen samen op { $words } tekens
    }

new-work-goal-unit = Tellen in
new-work-goal-unit-hint = Gebruikt voor elk doel in woorden of tekens in dit project. Kan later worden gewijzigd.

settings-group-goal-unit = Doelen
settings-goal-unit-switch-title = Telleenheid wijzigen
settings-goal-unit-switch-text = De doelen en mijlpalen die al voor dit project zijn ingesteld, zijn ingevoerd in { $from }. Overschakelen naar { $to } zet ze niet om, dus elk bestaand getal wordt voortaan gelezen als { $to }.
settings-goal-unit-switch-informative = Er gaat niets verloren. Schakel op elk moment terug om de oorspronkelijke lezing te herstellen, en werk daarna de doelen bij die u wilt behouden.
settings-goal-unit-switch-confirm = Wijzigen

goal-distribute-action = Verdelen…
distribute-title = Het doel verdelen
distribute-empty = Er zit niets in om het doel over te verdelen.
distribute-weight = Verdeling
distribute-weight-length = Op lengte
distribute-weight-rows = Per onderdeel
distribute-weight-even = Gelijk
distribute-overwrite = Doelen vervangen die al zijn ingesteld
distribute-col-item = Item
distribute-col-current = Nu
distribute-col-proposed = Na
distribute-total = Samen { $total }, tegenover een doel van { $goal }.
distribute-over-budget = De doelen die hierin al zijn ingesteld, komen { $over } hoger uit dan het doel van deze container zelf. Verhoog het doel ervan, verlaag de hunne of vervang ze.
distribute-apply = Verdelen
distribute-cancel = Annuleren

milestone-target-gone = Doel verwijderd
milestone-no-target = Geen doel
milestone-add = Toevoegen
milestone-add-label = Wat moet er worden bereikt

pace-summary-title = Hoe het boek ervoor staat
pace-summary-remaining = { $words } te gaan
pace-summary-open = Het plan openen
pace-summary-close = Sluiten
pace-summary-dont-show = Dit niet tonen bij openen
pace-summary-menu = Schrijfplan…

menu-help-topics = &Helponderwerpen
menu-help-shortcuts = &Sneltoetsen…
menu-help-website = Skribisto op het &web
menu-help-report = Een probleem &melden…
menu-help-check-updates = Controleren op &updates
menu-command-palette = Opdrachten&palet…

learn-help-topics = Helponderwerpen
learn-shortcuts = Sneltoetsen
learn-website = Skribisto op het web

help-window-title = Help
help-filter-topics = Onderwerpen filteren
help-no-matching-topic = Geen onderwerp komt overeen.
help-topic-missing = Dit onderwerp is niet meer beschikbaar.
help-back = Terug
help-not-translated = Deze pagina is nog niet vertaald en wordt daarom in het Engels getoond.

help-section-getting-started = Aan de slag
help-section-writing = Schrijven
help-section-reviewing = Revisie
help-section-exchanging = Tekst importeren en exporteren
help-section-keeping = Uw werk veilig bewaren
help-section-extensions = Extensies

help-topic-getting-started = Uw eerste project
help-topic-writing-model = Hoe een boek wordt samengesteld
help-topic-drafts-and-old-versions = Een oudere versie bewaren
help-topic-goals-and-pace = Doelen en tempo
help-topic-comments = Opmerkingen
help-topic-round-trip = Uw boek naar een lezer sturen
help-topic-export = Exporteren
help-topic-import-documents = Documenten importeren
help-topic-import-projects = Een volledig project overzetten
help-topic-backups-and-versions = Reservekopieën en versies

help-shortcuts-title = Sneltoetsen
help-shortcuts-filter = Sneltoetsen filteren
help-shortcuts-no-matches = Geen enkele sneltoets komt overeen.
help-shortcuts-rebind = Sneltoetsen wijzigen…
help-shortcuts-close = Sluiten

command-palette-placeholder = Typ een opdracht

shortcut-name-binder-duplicate = Dupliceren
shortcut-name-comments-add = Opmerking toevoegen
shortcut-name-comments-add-paragraph = Opmerking bij alinea
shortcut-name-spellcheck-toggle = Spelling controleren
shortcut-name-editor-tab-close = Tabblad sluiten
shortcut-name-editor-tab-pin = Tabblad vastzetten of losmaken
shortcut-name-editor-save = Opslaan
shortcut-name-work-export = Exporteren…
shortcut-name-work-new = Nieuw project
shortcut-name-work-open = Project openen
shortcut-name-window-new = Nieuw venster
shortcut-name-work-close = Project sluiten
shortcut-name-app-settings = Instellingen
shortcut-name-app-quit = Afsluiten
shortcut-name-editor-insert-footnote = Voetnoot invoegen
shortcut-name-format-scene-break = Scèneovergang invoegen
shortcut-name-format-major-scene-break = Grote scèneovergang invoegen
shortcut-name-format-link = Koppeling invoegen
shortcut-name-format-mark-nbsp = Vaste spatie invoegen
shortcut-name-format-mark-nbhyphen = Vast koppelteken invoegen
shortcut-name-format-mark-nnbsp = Smalle vaste spatie invoegen
shortcut-name-format-mark-zwsp = Spatie zonder breedte invoegen
shortcut-name-go-next = Volgende
shortcut-name-go-prev = Vorige
shortcut-name-go-to = Ga naar
shortcut-name-outline-toggle = Ordner tonen/verbergen
shortcut-name-preview-toggle = Voorbeeldstrook tonen/verbergen
shortcut-name-view-fullscreen = Volledig scherm aan/uit
shortcut-name-view-focus-mode = Afleidingsvrije modus aan/uit
shortcut-name-editor-size-increase = Tekst groter
shortcut-name-editor-size-decrease = Tekst kleiner
shortcut-name-editor-size-reset = Tekstgrootte herstellen
shortcut-name-editor-find = Zoeken
shortcut-name-editor-replace = Vervangen
shortcut-name-editor-find-next = Volgende treffer
shortcut-name-editor-find-prev = Vorige treffer
shortcut-name-search-show = Zoeken in project
shortcut-name-search-replace = Vervangen in project
shortcut-name-outline-open-to-side = Openen aan de zijkant
shortcut-name-help-topics = Helponderwerpen
shortcut-name-help-shortcuts = Sneltoetsen
shortcut-name-help-website = Skribisto op het web
shortcut-name-help-report = Een probleem melden
shortcut-name-help-check-updates = Controleren op updates
update-check-behind = Versie { $version } is beschikbaar.
update-check-current = U gebruikt de nieuwste versie.
update-check-failed = Kan niet controleren op een nieuwere versie. { $error }
shortcut-name-command-palette = Opdrachtenpalet
help-section-reference = Wat is wat

help-concept-scene-break = Scèneovergang
help-concept-major-scene-break = Grote scèneovergang
help-concept-find-in-prose = Zoeken in proza
help-concept-story-bible = Verhaalbijbel
help-concept-goal-unit = Woorden of tekens
help-concept-goal-progress = Voortgang
help-concept-manuscript-words = Wat telt als het manuscript
help-concept-exportable = Weggelaten uit de export
help-concept-distribute = Een doel verdelen
help-concept-subtree-total = Waarop de doelen hierin samen uitkomen
help-concept-milestone = Mijlpaal
help-concept-pace-plan = Tempoplan

help-concept-tag = Tag
help-concept-label = Label
help-concept-point-of-view = Vertelperspectief
help-concept-epigraph = Motto
help-concept-footnote = Voetnoot
help-concept-chapter-mode = Hoofdstukvorm
help-concept-comment = Opmerking
help-concept-backup = Reservekopie
help-concept-version = Versie
help-concept-trash = Prullenbak
help-concept-spellcheck = Spellingscontrole
help-concept-search-replace = Zoeken en vervangen
help-concept-note-template = Notitiesjabloon
help-concept-story-bible-entry = Item in verhaalbijbel
help-concept-text-replacement = Tekstvervanging
help-concept-smart-punctuation = Slimme interpunctie
help-concept-export-style = Exportstijl
help-concept-round-trip-marks = Retourmarkeringen

epigraph-mark-attribution = Bronregel
epigraph-mark-attribution-tip = De regel waarin de cursor staat markeren als bron van het citaat, zodat die als bronvermelding wordt afgedrukt
margin-lane-name = Margemarkeringen
margin-lane-provider-comments = Opmerkingen
margin-lane-provider-story-bible = Item in verhaalbijbel
margin-lane-provider-story-bible-hint = In de weergave In proza van een notitie: waar dat item wordt genoemd, en de scènes die door zijn ogen worden verteld. Markeert die namen ook in het proza zelf.
margin-lane-mark-point-of-view = Vanaf hier verteld
margin-lane-mark-named-here = Hier genoemd
margin-lane-provider-comments-hint = Waar een opmerking aan de tekst hangt
margin-lane-provider-search = Zoekresultaten
margin-lane-provider-search-hint = Elke treffer voor wat u het laatst zocht, waar die ook in het document staat
margin-lane-provider-boundaries = Waar elk document begint
margin-lane-provider-boundaries-hint = Een lijn bovenaan elke scène in een doorlopende weergave, zodat u ziet waar u bent
margin-lane-provider-spelling = Spelling
margin-lane-provider-spelling-hint = Waar de spellingscontrole een woord heeft gemarkeerd. Uit, tenzij u erom vraagt: het markeert de mening van een machine over uw proza
margin-lane-spelling = { $word }, mogelijke spelfout
margin-lane-search-hit = { $text }, treffer { $index } van { $total }
margin-lane-search-current = { $text }, treffer { $index } van { $total }, de huidige
margin-lane-boundary = Begin van { $title }
margin-lane-boundary-untitled = een naamloos document
settings-page-margin-lane = Margemarkeringen
settings-desc-margin-lane = De balk naast de schuifbalk, en wat die toont
settings-margin-lane-enabled = De margebalk tonen
settings-margin-lane-enabled-hint = Dezelfde schakelaar als Beeld ▸ Margemarkeringen.
settings-margin-lane-enabled-more = Hij zegt nooit dat er iets fout is: hij laat zien waar dingen staan, en wat u daarvan vindt, laat hij aan u over.
settings-group-margin-lane-marks = Wat hij markeert
settings-margin-lane-no-providers = Er markeert nog niets de margebalk.
settings-group-margin-lane-texture = Dialoogtextuur
settings-margin-lane-texture = De dialoogtextuur tonen
settings-margin-lane-texture-hint = Eén streepje per alinea: hoe lang ze is, en hoeveel ervan gesproken wordt.
settings-margin-lane-texture-more = Wordt niet gemeten voor talen zonder vastgelegde conventie; daar toont het streepje alleen de lengte.
settings-group-margin-lane-surfaces = Waar hij verschijnt
settings-margin-lane-surface-editor = Teksteditor
settings-margin-lane-surface-stream = Doorlopende weergaven
settings-margin-lane-surface-search-preview = Voorbeeld zoekresultaat
menu-margin-lane = &Margemarkeringen

status-preset-drafting = Eerste versie
status-preset-passes = Revisierondes
status-preset-plume = Plume Creator
status-todo = Te doen
status-draft = Concept
status-revised = Herzien
status-final = Definitief
status-outline = Opzet
status-first-edit = 1e bewerking
status-second-edit = 2e bewerking
status-done = Gereed
status-plume-draft-1 = 1e versie
status-plume-draft-2 = 2e versie
status-plume-draft-3 = 3e versie
status-plume-edit-1 = 1e bewerking
status-plume-edit-2 = 2e bewerking
status-plume-edit-3 = 3e bewerking
status-plume-proofread = Proefgelezen
status-plume-finished = Voltooid
new-work-statuses = Werkstroom
new-work-statuses-hint = De fasen die een scène doorloopt. U kunt ze op elk moment hernoemen, herschikken of aanvullen.
status-none = Geen status
inspector-status = Status
overview-col-status = Status
overview-status-mixed = de onderliggende delen verschillen
help-concept-status = Status
status-completion-title = Hoe het boek ervoor staat
status-completion-headline = { $done } van { $total } scènes klaar
status-completion-empty = Nog geen scènes. Dit leest het manuscript en vult zich dus terwijl u schrijft.
status-completion-open = Hoe het boek ervoor staat…

settings-page-statuses = Statussen
settings-desc-statuses = De fasen die een scène doorloopt hernoemen, herschikken, toevoegen en verwijderen
settings-statuses-add = Status toevoegen
settings-statuses-add-placeholder = Nieuwe statusnaam
settings-statuses-desc =
    De fasen die een scène doorloopt, op volgorde, van minst af bovenaan tot meest af
    onderaan. Die volgorde bepaalt overal in de app wat “minder af dan” betekent, dus zet ze
    zoals uw werkwijze werkelijk verloopt.
settings-statuses-apply-preset = Een voorinstelling toepassen…
settings-statuses-preset-applied = { $added ->
    [one] 1 status toegevoegd
   *[other] { $added } statussen toegevoegd
}
settings-statuses-preset-refused = Dit project heeft al een statusladder. Voorinstellingen vullen alleen een lege ladder. Verwijder eerst de treden die u niet wilt.
settings-statuses-duplicate = “{ $name }” staat al op deze ladder
settings-statuses-added = “{ $name }” toegevoegd
settings-statuses-move-up = Omhoog (minder af)
settings-statuses-move-down = Omlaag (verder af)
settings-statuses-details-placeholder = Wat deze fase betekent (optioneel)
settings-statuses-delete = “{ $name }” verwijderen
settings-statuses-delete-in-use = { $count ->
        [one] “{ $name }” verwijderen: 1 item heeft deze status en verliest hem
       *[other] “{ $name }” verwijderen: { $count } items hebben deze status en verliezen hem
    }
settings-statuses-deleted = “{ $name }” verwijderd
settings-statuses-deleted-in-use = { $count ->
        [one] “{ $name }” verwijderd. 1 item heeft geen status meer.
       *[other] “{ $name }” verwijderd. { $count } items hebben geen status meer.
    }
settings-statuses-empty-title = Nog geen werkstroom
settings-statuses-empty-body = Een status geeft aan hoe ver een scène is. Voeg er hierboven een toe, of begin met een voorinstelling.
status-category-planned = Gepland
status-category-drafting = Eerste versie
status-category-needs-work = Te bewerken
status-category-revised = Herzien
status-category-final = Definitief

progress-percent = { $percent }%
progress-percent-with-label = { $percent }% · { $label }
progress-import-opening = Project wordt geopend…
progress-import-outline = Indeling wordt gelezen…
progress-import-converting = Hoofdstukken en scènes worden omgezet…
progress-import-writing = Het .skrib-bestand wordt geschreven…
progress-import-done = Klaar
progress-import-structure = Structuur wordt bepaald…
progress-export-start = Bezig met starten…
progress-export-writing = Bezig met schrijven…
progress-project-reading = Project wordt gelezen…
settings-sec-work-titled = Project: { $title }
import-notice-flattened-rich-texts = { $count ->
    [one] Eén tekst was dieper genest dan een Skribisto-project aankan; de woorden zijn als platte tekst bewaard, zonder opmaak.
   *[other] { $count } teksten waren dieper genest dan een Skribisto-project aankan; de woorden zijn als platte tekst bewaard, zonder opmaak.
}
import-notice-free-text-badges = { $count ->
    [one] Eén scènebadge lijkt op vrije tekst in plaats van op een lijst met statussen, dus is die als label van het item bewaard in plaats van een tag te worden.
   *[other] { $count } verschillende scènebadges lijken op vrije tekst in plaats van op een lijst met statussen, dus zijn ze als labels van de items bewaard in plaats van tags te worden.
}
import-notice-separator-text-dropped = De scheidingslijn “{ $name }” bevatte tekst, waarvoor in een Skribisto-project geen plaats is; de tekst is niet geïmporteerd.
import-notice-separator-without-scene = De scheidingslijn “{ $name }” had in zijn hoofdstuk geen voorafgaande scène om aan te koppelen, dus is die niet geïmporteerd.
import-notice-chapter-without-scenes = Het hoofdstuk “{ $name }” heeft geen scènes, dus de scheidingslijnen ervan zijn niet geïmporteerd.
import-notice-rich-text-unconverted = Een deel van de opgemaakte tekst kon niet worden omgezet ({ $error }); het is niet geïmporteerd.
import-notice-unresolved-cross-link = Een koppeling naar item #{ $number } van de rolbezetting kon niet worden gevolgd: het item ontbreekt of lag in de prullenbak.
import-notice-read-copy = De { $container ->
    [single-file] kopie in één bestand
   *[folder] mapkopie
} van dit project is gelezen.
import-notice-read-copy-dated = De { $container ->
    [single-file] kopie in één bestand
   *[folder] mapkopie
} van dit project is gelezen, laatst gewijzigd op { $day }.
import-notice-original-format = Dit project heeft de oorspronkelijke indeling van Manuskript uit 2016. Het is volledig gelezen; er is niets in gewijzigd.
import-notice-newer-single-file-beside = “{ $file }” is nieuwer dan deze map ({ $file_day } tegenover { $folder_day }). Manuskript laat de oude map staan wanneer een project overgaat naar de modus met één bestand, dus deze map kan een achtergelaten kopie zijn; als u in plaats daarvan het .msk-bestand importeert, wordt de nieuwere versie gelezen.
import-notice-pickle-skipped = “{ $file }” is niet gelezen. Het is een Python-pickle, en het laden daarvan uit een projectbestand was een beveiligingslek in Manuskript zelf (CVE-2021-35196); de instellingen erin zijn weergavevoorkeuren, geen geschreven tekst.
import-notice-file-unreadable = “{ $file }” kon niet worden gelezen ({ $error }) en is overgeslagen; al het andere is geïmporteerd.
import-notice-not-utf8 = “{ $file }” is geen geldige UTF-8; de tekens die niet konden worden gelezen, zijn vervangen in plaats van de import af te breken.
import-notice-table-unreadable = “{ $file }” kon niet worden gelezen ({ $error }); wat erin stond, is niet geïmporteerd.
import-notice-outline-missing = “{ $file }” ontbreekt, dus dit project heeft geen manuscript om te importeren.
import-notice-outline-unreadable = “{ $file }” kon niet worden gelezen ({ $error }); er is geen manuscript geïmporteerd.
import-notice-named-none-row = “{ $file }” begint met een rij met een naam (“{ $name }”) op de plek waar Manuskript zijn lege item “none” bewaart. Die rij is weggelaten, zodat de rijen erna de nummers houden die het manuscript gebruikt.
import-notice-unknown-field = “{ $file }” heeft een veld dat deze importfunctie niet kent: “{ $field }”. Het is niet geïmporteerd.
import-notice-colorless-labels = { $count ->
    [one] “{ $file }” heeft een regel zonder kleur. Die is zonder kleur geïmporteerd; Manuskript stopt bij zo’n regel met het laden van het project.
   *[other] “{ $file }” heeft { $count } regels zonder kleur. Ze zijn zonder kleur geïmporteerd; Manuskript stopt bij zo’n regel met het laden van het project.
}
import-notice-settings-unreadable = “{ $file }” kon niet worden gelezen ({ $error }); de taal van het project is niet geïmporteerd.
import-notice-revisions-unreadable = “{ $file }” kon niet worden gelezen ({ $error }); de geschiedenis van het project is niet geïmporteerd, maar verder is er niets aangetast.
import-notice-foreign-files = { $count ->
    [one] Dit project bevat een bestand dat geen deel uitmaakt van de Manuskript-indeling en niet is geïmporteerd: { $files }.
   *[other] Dit project bevat { $count } bestanden die geen deel uitmaken van de Manuskript-indeling en niet zijn geïmporteerd: { $files }.
}
import-notice-foreign-files-more = Dit project bevat { $count } bestanden die geen deel uitmaken van de Manuskript-indeling en niet zijn geïmporteerd: { $files } en { $more ->
    [one] nog één
   *[other] nog { $more }
}.
import-notice-not-a-character-file = “{ $file }” is geen personagebestand van Manuskript en is weggelaten.
import-notice-not-a-text-item = “{ $file }” is geen tekstitem van Manuskript en is uit de structuur weggelaten.
import-notice-missing-blank-line = “{ $file }” heeft geen lege regel na de kop. Het laatste veld is bewaard, hoewel Manuskript het kwijtraakt.
import-notice-folder-without-metadata = “{ $folder }” heeft geen { $file }, dus die is geïmporteerd als gewone map met de naam van de directory. Manuskript slaat zo’n directory met alles erin over.
import-notice-rows-given-ids = { $count ->
    [one] Eén rij in de structuur had geen eigen ID en heeft er een gekregen. Manuskript laat zo’n rij weg, of stopt met het laden van het project als het een map is.
   *[other] { $count } rijen in de structuur hadden geen eigen ID en hebben er een gekregen. Manuskript laat zo’n rij weg, of stopt met het laden van het project als het een map is.
}
import-notice-plots-unreadable = “{ $file }” kon niet worden gelezen ({ $error }); de verhaallijnen zijn niet geïmporteerd.
import-notice-world-unreadable = “{ $file }” kon niet worden gelezen ({ $error }); de wereldelementen zijn niet geïmporteerd.
import-notice-world-not-opml = “{ $file }” is geen OPML-document; de wereldelementen zijn niet geïmporteerd.
import-notice-text-flattened = De tekst van “{ $title }” is dieper genest dan een Skribisto-project aankan; de woorden zijn als platte tekst bewaard, zonder opmaak.
import-notice-text-unconverted = De tekst van “{ $title }” kon niet worden omgezet ({ $error }); die is precies zo bewaard als hij geschreven was.
import-notice-text-without-file-name = { $kind ->
    [scene] De scènetekst
    [note] De notitietekst
    [synopsis] De synopsis
   *[title] De titel
} van “{ $title }” kon geen bestandsnaam krijgen en is daarom niet geïmporteerd.
import-notice-unknown-label = “{ $title }” heeft label { $index }, dat niet in de labellijst van dit project staat. Het is zonder label geïmporteerd.
import-notice-unknown-status = “{ $title }” heeft status { $index }, die niet in de statuslijst van dit project staat. Het is zonder status geïmporteerd.
import-notice-unknown-point-of-view = “{ $title }” wordt verteld vanuit een personage dat niet in de rolbezetting van dit project staat. Het vertelperspectief is niet ingesteld.
import-notice-unresolved-references = De verwijzingen in de tekst van “{ $title }” konden niet worden opgelost ({ $error }); de tekst is bewaard zoals hij is omgezet, verwijzingsmarkeringen inbegrepen.
import-notice-custom-icons = { $count ->
    [one] Eén item had een eigen pictogram. Skribisto geeft elk item het pictogram van zijn type, dus het pictogram is niet geïmporteerd; verder is er niets aan het item veranderd.
   *[other] { $count } items hadden een eigen pictogram. Skribisto geeft elk item het pictogram van zijn type, dus de pictogrammen zijn niet geïmporteerd; verder is er niets aan die items veranderd.
}
import-notice-character-colors = { $count ->
    [one] Eén personage had een kleur naast de naam. Skribisto kleurt tags, geen items, dus de kleur is niet geïmporteerd; het personage zelf wel.
   *[other] { $count } personages hadden een kleur naast hun naam. Skribisto kleurt tags, geen items, dus de kleuren zijn niet geïmporteerd; de personages zelf allemaal wel.
}
import-notice-versions-kept = { $found ->
    [one] Dit project had één eerdere versie vastgelegd
   *[other] Dit project had { $found } eerdere versies vastgelegd
}; { $kept ->
    [one] daarvan is er één bewaard
   *[other] daarvan zijn er { $kept } bewaard
}, uitgedund zoals Skribisto zijn eigen geschiedenis uitdunt. Ze staan in het paneel Versies.
import-notice-versions-unplaced = { $count ->
    [one] Eén eerdere versie hoort bij een item dat niet meer in het project staat en kon niet worden geplaatst.
   *[other] { $count } eerdere versies horen bij een item dat niet meer in het project staat en konden niet worden geplaatst.
}
dialog-open-work-title = Skribisto-project openen
dialog-save-as-file-title = Opslaan als één .skrib-bestand
dialog-save-as-folder-title = Kies een bovenliggende map voor het project
dialog-manuskript-file-title = Kies een Manuskript-project
dialog-manuskript-folder-title = Kies een Manuskript-projectmap
dialog-filter-skrib = Skribisto-project
dialog-filter-manuskript = Manuskript-project
dialog-filter-plume = Plume Creator-project
dialog-filter-hunspell-aff = Hunspell-affixbestand (.aff)
dialog-filter-hunspell-dic = Hunspell-woordenboek (.dic)
dialog-filter-export = Geëxporteerd bestand
shortcut-name-view-margin-lane = Margemarkeringen aan/uit
df-theme-paper = Papier
df-theme-sepia = Sepia
df-theme-night = Nacht
df-theme-high-contrast = Hoog contrast
df-theme-midnight = Middernacht
welcome-example-byline = door { $author }
welcome-sections-a11y = Onderdelen van het welkomstvenster
comments-a11y-summary-author = { $author }: { $text }
comments-a11y-summary-replies = { $text } ({ $count ->
    [one] 1 antwoord
   *[other] { $count } antwoorden
})
comments-a11y-summary-author-replies = { $author }: { $text } ({ $count ->
    [one] 1 antwoord
   *[other] { $count } antwoorden
})
margin-lane-a11y-mark = { $label }, op { $percent } procent van de tekst
margin-lane-a11y-marks = { $label }, { $count } markeringen, op { $percent } procent van de tekst
margin-lane-a11y-fallback = { $count ->
    [one] Margemarkeringen, 1 markering
   *[other] Margemarkeringen, { $count } markeringen
}
export-choose-unnamed-binder = Naamloze ordner
export-style-neutral = Neutraal
export-style-manuscript-shunn = Standaardmanuscript (Shunn)
export-style-ebook-clean = Strak e-book
date-time-short = { $day }-{ $month }-{ $year } { $hour }:{ $minute }
pace-day-label = { $day }-{ $month }
byte-size-bytes = { $value } B
byte-size-kb = { $value } KB
byte-size-mb = { $whole },{ $tenth } MB
byte-size-gb = { $whole },{ $tenth } GB
unit-percent = { $value }%
unit-pixels = { $value } px
settings-paratext-error-no-name = Een structuur heeft een `name` nodig.
export-error-no-project = Er is geen project geopend om te exporteren.
