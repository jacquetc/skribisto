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
menu-import-scrivener = &Scrivener (.scriv)…
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
statusbar-focus-comments = Opmerkingen
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
backup-failed-close-error-text = Er kon vóór het sluiten geen reservekopie worden geschreven: { $error }. Los het probleem op en probeer het opnieuw, of sluit af zonder reservekopie.
backup-no-result = Het maken van de reservekopie is beëindigd zonder te melden of er een kopie is gemaakt. Kijk in de lijst met reservekopieën of er een nieuwe bij staat.

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
import-plume-source = Plume-project
import-plume-source-hint = Kies een .plume- of .plume_backup-bestand (elke versie van Plume Creator).
import-plume-trash-warning = ⚠ Items in de prullenbak of verwijderde items worden niet overgenomen.
import-plume-source-required = Kies een Plume-projectbestand
import-plume-source-missing = Dit bestand bestaat niet
import-plume-source-not-file = Dit pad is geen bestand
import-plume-manuscript-binder = Manuscript
import-plume-story-bible-binder = Verhaalbijbel
import-plume-notes-of = { $name } (notities)
import-plume-content-of = { $name } (tekst)
import-plume-progress-title = Plume-project wordt geïmporteerd…
import-plume-done = { $imported ->
        [one] { $imported } item geïmporteerd.
       *[other] { $imported } items geïmporteerd.
    } { $skipped ->
        [one] { $skipped } item uit de prullenbak is niet overgenomen.
       *[other] { $skipped } items uit de prullenbak zijn niet overgenomen.
    }
import-plume-nested-too-deep = Het project is niet geïmporteerd: het onderdeel “{ $part }” is meer dan { $limit } niveaus diep genest. Geen enkel Plume Creator-project komt daar in de buurt, en het inlezen zou Skribisto kunnen laten vastlopen. Er is niets geschreven.
import-plume-declares-entities = Het project is niet geïmporteerd: het onderdeel “{ $part }” declareert XML-entiteiten, wat geen enkel Plume Creator-project doet. Het uitvouwen ervan zou meer geheugen kunnen vragen dan de computer heeft en Skribisto laten vastlopen. Er is niets geschreven.
import-plume-no-outline = Dit bestand is een gecomprimeerd archief, maar geen Plume Creator-project: het heeft geen indeling. Er is niets geschreven.
import-plume-empty-outline = Het indelingsbestand van dit Plume Creator-project is leeg, dus er valt niets te importeren. Er is niets geschreven.
import-plume-wrong-document = Dit is geen Plume Creator-project: het onderdeel “{ $part }” is een “{ $tag }”-document. Er is niets geschreven.
import-plume-too-new = Het onderdeel “{ $part }” van dit project is geschreven door een nieuwere Plume Creator (versie { $version }) dan Skribisto kan lezen (tot en met { $supported }). Er is niets geschreven.

import-manuskript-title = Manuskript-project importeren
import-manuskript-source = Manuskript-project
import-manuskript-source-hint = Kies het .msk-bestand of de projectmap (elke versie van Manuskript).
import-manuskript-source-file = Bestand kiezen…
import-manuskript-source-folder = Map kiezen…
import-manuskript-source-required = Kies een Manuskript-project
import-manuskript-source-missing = Dit bestand of map bestaat niet
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
import-manuskript-notes-of = { $title } (notities)
import-manuskript-progress-title = Manuskript-project wordt geïmporteerd…
import-manuskript-done = { $imported ->
        [one] { $imported } item geïmporteerd.
       *[other] { $imported } items geïmporteerd.
    } { $revisions ->
        [0] { "" }
        [one] { $revisions } eerdere versie is meegekomen.
       *[other] { $revisions } eerdere versies zijn meegekomen.
    }
import-manuskript-nested-too-deep = Het project is niet geïmporteerd: het bestand “{ $part }” is meer dan { $limit } niveaus diep genest. Geen enkel Manuskript-project komt daar in de buurt, en het inlezen zou Skribisto kunnen laten vastlopen. Er is niets geschreven.
import-manuskript-folders-too-deep = Het project is niet geïmporteerd: een van de bestanden staat meer dan { $limit } mappen diep. Geen enkel Manuskript-project komt daar in de buurt, en het inlezen zou Skribisto kunnen laten vastlopen. Er is niets geschreven.
import-manuskript-declares-entities = Het project is niet geïmporteerd: het bestand “{ $part }” declareert XML-entiteiten, wat geen enkel Manuskript-project doet. Het uitvouwen ervan zou meer geheugen kunnen vragen dan de computer heeft en Skribisto laten vastlopen. Er is niets geschreven.
import-manuskript-not-a-project-file = “{ $file }” is geen archief van een Manuskript-project en ook niet het kleine .msk-bestand dat naar een projectmap verwijst. Er is niets geschreven.
import-manuskript-folder-missing = “{ $stub }” verwijst naar een projectmap met de naam “{ $folder }” ernaast, en die map is er niet. Een Manuskript-project in mapmodus bestaat uit allebei, en ze moeten samen worden meegenomen. Er is niets geschreven.
import-manuskript-not-a-project = Dit lijkt geen Manuskript-project: het heeft geen Manuskript-markering en geen indeling. Er is niets geschreven.
import-manuskript-format-too-new = Dit project heeft Manuskript-formaat { $version }, nieuwer dan Skribisto kan lezen (tot en met { $supported }). Er is niets geschreven.
import-manuskript-too-many-files = Het project is niet geïmporteerd: het bevat meer dan { $limit } bestanden, veel meer dan welk Manuskript-project ook. Er is niets geschreven.
import-manuskript-too-large = Het project is niet geïmporteerd: het is groter dan de { $limit } MB die Skribisto uit een Manuskript-project leest. Er is niets geschreven.
project-import-close = Sluiten
project-import-location = Doelmap
project-import-location-title = Kies een doelmap
project-import-name = Bestandsnaam
project-import-name-placeholder = Projectnaam
project-import-will-create = Wordt aangemaakt
project-import-cancel = Annuleren
project-import-import = Importeren
project-import-location-required = Kies een doelmap
project-import-location-missing = Deze map bestaat niet
project-import-location-not-folder = Dit pad is geen map
project-import-location-readonly = In deze map kan niet worden geschreven
project-import-name-required = Voer een bestandsnaam in
project-import-name-exists = Hier bestaat al een bestand met deze naam. Bij het importeren wordt gevraagd of het mag worden overschreven
project-import-name-is-folder = Hier staat al een map met deze naam, misschien een project dat als map is opgeslagen. Een import kan die niet vervangen: kies een andere naam
project-import-overwrite-title = Bestaand bestand vervangen?
project-import-overwrite-text = “{ $name }” bestaat al. Vervangen door het geïmporteerde project?
project-import-busy-title = Er wordt al een ander project geïmporteerd
project-import-busy-text = Projecten worden een voor een geïmporteerd. Wacht tot de lopende import klaar is, of annuleer die, en start deze daarna opnieuw.
project-import-progress-announcement = { $title } { $phase }
project-import-stopping = De import wordt gestopt…
project-import-cancelled = Importeren geannuleerd
project-import-open-now = Nu openen
project-import-details = Details
project-import-report = { $count ->
        [one] 1 punt om te weten over deze import
       *[other] { $count } punten om te weten over deze import
    }
project-import-report-title = Over deze import
project-import-error-title = Kan het project niet importeren
project-import-failed-cancelled = De import is geannuleerd. Er is niets geschreven.
project-import-failed-target-exists = “{ $file }” bestaat al, en de import mocht het niet vervangen. Er is niets geschreven.
project-import-failed-target-folder = “{ $file }” is een map, misschien een project dat als map is opgeslagen, en een import kan die niet vervangen. Kies een andere naam. Er is niets geschreven.
project-import-failed-not-written = “{ $file }” kon niet worden geschreven in “{ $folder }” ({ $reason }). Er is niets geschreven.
project-import-failed-disk-full = Er is niet genoeg ruimte meer in “{ $folder }” om “{ $file }” te schrijven. Maak daar ruimte vrij en importeer daarna opnieuw. Er is niets geschreven.
project-import-failed-source-missing = “{ $file }” is er niet meer. Er is niets geschreven.
project-import-failed-unreadable = Skribisto kon “{ $file }” niet lezen ({ $reason }). Er is niets geschreven.
project-import-failed-archive-damaged = “{ $file }” is een gecomprimeerd bestand dat niet kon worden gelezen, waarschijnlijk omdat het beschadigd is ({ $reason }). Er is niets geschreven.
project-import-failed-archive-entries = Het project is niet geïmporteerd: het archief vermeldt { $entries ->
        [one] { $entries } bestand
       *[other] { $entries } bestanden
    }, meer dan de { $limit } die een project mag bevatten, wat alleen voorkomt bij een bestand dat is gemaakt om het geheugen uit te putten. Er is niets geschreven.
project-import-failed-archive-member = Het project is niet geïmporteerd: het onderdeel “{ $part }” wordt uitgepakt groter dan { $limit } MB, wat alleen voorkomt bij een bestand dat is gemaakt om het geheugen uit te putten. Er is niets geschreven.
project-import-failed-archive-size = Het project is niet geïmporteerd: het wordt uitgepakt groter dan { $limit } MB, wat alleen voorkomt bij een bestand dat is gemaakt om het geheugen uit te putten. Er is niets geschreven.
project-import-failed-nested = Het project is niet geïmporteerd: het onderdeel “{ $part }” is meer dan { $limit } niveaus diep genest, en het inlezen zou Skribisto kunnen laten vastlopen. Er is niets geschreven.
project-import-failed-entities = Het project is niet geïmporteerd: het onderdeel “{ $part }” declareert XML-entiteiten. Het uitvouwen ervan zou meer geheugen kunnen vragen dan de computer heeft en Skribisto laten vastlopen. Er is niets geschreven.
project-import-failed-folders = Het project is niet geïmporteerd: een van de bestanden staat meer dan { $limit } mappen diep, en het inlezen zou Skribisto kunnen laten vastlopen. Er is niets geschreven.
project-import-failed-malformed = Het project is niet geïmporteerd: het onderdeel “{ $part }” is beschadigd en kan niet worden gelezen ({ $reason }). Er is niets geschreven.
project-import-failed-scrivener-refused = Skribisto kan dit Scrivener-project niet importeren. Er is niets geschreven.
project-import-failed-scrivener-question = Het Scrivener-project stelt een vraag die bij het controleren niet is beantwoord. Start de import opnieuw, beantwoord de vraag en importeer daarna. Er is niets geschreven.
project-import-failed-scrivener-not-accepted = Voordat het Scrivener-project kan worden geïmporteerd, moet u in de stap Project een vakje aanvinken, en dat is niet gebeurd. Start de import opnieuw, vink het vakje aan en importeer daarna. Er is niets geschreven.
project-import-failed-scrivener-changed = Het Scrivener-project is gewijzigd nadat u het hebt gecontroleerd. Start de import opnieuw, controleer het project nog eens en importeer daarna. Er is niets geschreven.
project-import-failed-internal = De import is gestopt door een fout die Skribisto niet verwachtte: { $detail }

import-target-open-title = Dit project is geopend
import-target-open-text = “{ $name }” is geopend in Skribisto. Eroverheen importeren zou het project waaraan u werkt vervangen, en bij de volgende opslag zou dat project weer over de import heen worden geschreven. Kies een andere bestandsnaam, of sluit eerst het project.
import-warnings-announcement = { $result } { $notice }.
quit-import-running-title = Er loopt nog een import
quit-import-running-text = Skribisto is nog bezig “{ $name }” te importeren. Nu afsluiten stopt de import: er wordt niets geschreven, en het project waaruit u importeert, blijft zoals het was.
quit-import-stop = De import stoppen en afsluiten

## Import a Scrivener project: the wizard
import-scrivener-title = Scrivener-project importeren
import-scrivener-step-project = Project
import-scrivener-step-review = Controleren
import-scrivener-step-save = Opslaan
import-scrivener-back = Terug
import-scrivener-next = Volgende
import-scrivener-import = Importeren
import-scrivener-cancel = Annuleren
import-scrivener-help = Help

## Project step: choosing the project
import-scrivener-drop-title = Scrivener-project hier neerzetten
import-scrivener-drop-hint = Een .scriv-project, het .scrivx-bestand erin, een .zip-reservekopie of een .scrivtemplate
import-scrivener-source = Scrivener-project
import-scrivener-source-placeholder = Pad naar het project
import-scrivener-choose-file = Bestand kiezen…
import-scrivener-choose-folder = Map kiezen…
import-scrivener-source-hint = Kies de .scriv-map van het project, of open die en kies het .scrivx-bestand erin. Een .zip-reservekopie werkt ook.
import-scrivener-source-hint-mac = Verschijnt het project als één bestand, gebruik dan “Bestand kiezen…”. Verschijnt het als map, gebruik dan “Map kiezen…”. Een .zip-reservekopie werkt ook.
import-scrivener-dialog-file-title = Kies een Scrivener-project
import-scrivener-dialog-folder-title = Kies een Scrivener-projectmap
import-scrivener-dialog-filter = Scrivener-project
import-scrivener-source-missing = Op dit pad staat niets
import-scrivener-source-unreadable-name = Deze naam kan niet worden gelezen. Wijzig de naam van het project in uw bestandsbeheerder en kies het daarna opnieuw.
import-scrivener-empty-hint = Kies een project om te zien wat Skribisto erin vindt.
import-scrivener-reading = Project wordt gelezen…
import-scrivener-stop-reading = Stop met lezen
import-scrivener-read-failed = Het project kon niet worden gelezen
import-scrivener-read-failed-body = Er ging iets mis bij het lezen. Kies “Project opnieuw inlezen” om het nog eens te proberen, of “Details” om te zien wat er misging.
import-scrivener-details = Details
import-scrivener-check-again = Project opnieuw inlezen
import-scrivener-refusal-unknown-title = Dit project kan niet worden geïmporteerd
import-scrivener-refusal-unknown-body = Skribisto weet niet hoe het dit project moet overnemen. De details noemen de reden.

## Project step: the project card
import-scrivener-made-with-mac = Laatst opgeslagen door Scrivener { $version } voor macOS
import-scrivener-made-with-windows = Laatst opgeslagen door Scrivener { $version } voor Windows
import-scrivener-made-with-linux = Laatst opgeslagen door Scrivener { $version } voor Linux
import-scrivener-made-with-ios = Laatst opgeslagen door Scrivener { $version } voor iPhone en iPad
import-scrivener-made-with-version = Laatst opgeslagen door Scrivener { $version }
import-scrivener-made-with-unknown = Scrivener-project, formaat { $format }
import-scrivener-saved-on = Opgeslagen op { $date } op het apparaat “{ $device }”
import-scrivener-saved-on-no-device = Opgeslagen op { $date }
import-scrivener-card-counts = { $documents ->
        [one] 1 document
       *[other] { $documents } documenten
    }, { $manuscript } in het manuscript

## Review step
import-scrivener-review-hint = Zo wordt uw ordner overgenomen. Er wordt niets geschreven totdat u importeert, en van veel items kunt u daarna in de ordner het type wijzigen met Omzetten naar.
import-scrivener-choices-structure = Niveaus
import-scrivener-top-level = Mappen op het hoogste niveau
import-scrivener-choices-text = Tekst
import-scrivener-choice-containers = Mappen op niveau { $depth }
import-scrivener-choice-texts = Documenten op niveau { $depth }
import-scrivener-choice-count = { $count ->
        [one] 1 item
       *[other] { $count } items
    }
import-scrivener-choice-count-example = { $count ->
        [1] 1 item: “{ $example }”
       *[other] { $count } items, zoals “{ $example }”
    }
import-scrivener-rung-part = Delen
import-scrivener-rung-chapter = Hoofdstukken
import-scrivener-rung-plain = Gewone mappen
import-scrivener-texts-scenes = Scènes
import-scrivener-texts-chapters = Hoofdstukken
import-scrivener-choice-books = Boeken in het manuscript
import-scrivener-books-one = Eén boek
import-scrivener-books-several = Eén boek per map
import-scrivener-choice-split = { $count ->
        [1] “{ $title }” splitsen bij de kop
       *[other] “{ $title }” splitsen bij de { $count } koppen
    }
import-scrivener-root-items = { $count ->
        [one] 1 item
       *[other] { $count } items
    }
import-scrivener-root-items-manuscript = { $count ->
        [one] 1 item. Deze map lijkt op een manuscript.
       *[other] { $count } items. Deze map lijkt op een manuscript.
    }
import-scrivener-dest-notes = Notities
import-scrivener-dest-story-bible = Verhaalbijbel
import-scrivener-dest-manuscript = Een ander boek
import-scrivener-dest-leave-out = Weglaten
import-scrivener-choice-trash = { $count ->
        [one] De prullenbak overnemen (1 item)
       *[other] De prullenbak overnemen ({ $count } items)
    }
import-scrivener-trash-detail = { $count ->
        [0] Geen daarvan bevat tekst.
        [one] 1 daarvan bevat tekst.
       *[other] { $count } daarvan bevatten tekst.
    }
import-scrivener-choice-blank-lines = Lege regels worden scèneovergangen
import-scrivener-choice-ornaments = Regels met siertekens worden scèneovergangen
import-scrivener-preview-label = Structuur van het nieuwe project
import-scrivener-col-title = Titel
import-scrivener-col-becomes = Wordt
import-scrivener-col-exports = In exports
import-scrivener-in-exports-yes = Ja
import-scrivener-in-exports-no = Nee
import-scrivener-becomes-item = Item
import-scrivener-becomes-trashed = In de prullenbak
import-scrivener-before-title = Voordat u importeert
import-scrivener-review-error-unreadable = De controle kon niet worden voorbereid. Kies het project opnieuw.
import-scrivener-review-error-choice = Deze keuze past niet bij het project. Kies het project opnieuw.
import-scrivener-review-error-unmappable = Met deze keuze kon de ordner niet worden overgenomen zonder iets te verliezen. Maak een andere keuze, of meld het probleem.

## Save step
import-scrivener-save-hint = De import maakt een nieuw Skribisto-project. Uw Scrivener-project wordt alleen gelezen, nooit gewijzigd.
import-scrivener-title-field = Titel
import-scrivener-author = Auteur
import-scrivener-language = Taal
import-scrivener-location-inside-project = Kies een map buiten het Scrivener-project. Scrivener bewaart daarin alleen zijn eigen bestanden.
import-scrivener-copy-research-label = Onderzoeksbestanden naast het nieuwe project kopiëren
import-scrivener-copy-research = { $count ->
        [one] 1 onderzoeksbestand dat Skribisto niet kan openen, zoals een PDF of een geluidsbestand, gaat naar een map naast het project. Staat het vinkje uit, dan zegt de notitie ervan waar het in uw Scrivener-project staat.
       *[other] { $count } onderzoeksbestanden die Skribisto niet kan openen, zoals PDF’s of geluidsbestanden, gaan naar een map naast het project. Staat het vinkje uit, dan zeggen hun notities waar ze in uw Scrivener-project staan.
    }
import-scrivener-default-name = Scrivener-project

## Progress and result
import-scrivener-progress-title = Scrivener-project wordt geïmporteerd…
import-scrivener-done-title = Scrivener-project geïmporteerd
import-scrivener-done = { $items ->
        [one] Het nieuwe project bevat 1 item.
       *[other] Het nieuwe project bevat { $items } items.
    }
import-scrivener-done-check = { $items ->
        [one] Het nieuwe project bevat 1 item.
       *[other] Het nieuwe project bevat { $items } items.
    } { $findings ->
        [one] Er is 1 punt om na te kijken.
       *[other] Er zijn { $findings } punten om na te kijken.
    }
import-scrivener-review-again = Import hercontroleren

## The import's report (the dialog See report opens)
import-scrivener-see-report = Rapport bekijken
import-scrivener-report-from = Gelezen uit { $path }
import-scrivener-report-to = Geschreven naar { $path }
import-scrivener-report-came-across = Wat is overgenomen
import-scrivener-report-nothing-counted = Er is niets geteld.
import-scrivener-report-items = { $count ->
        [one] 1 item
       *[other] { $count } items
    }
import-scrivener-report-words = { $count ->
        [one] 1 woord
       *[other] { $count } woorden
    }
import-scrivener-report-synopses = { $count ->
        [one] 1 synopsis
       *[other] { $count } synopsissen
    }
import-scrivener-report-document-notes = { $count ->
        [one] notities van 1 document
       *[other] notities van { $count } documenten
    }
import-scrivener-report-comments = { $count ->
        [one] 1 opmerking
       *[other] { $count } opmerkingen
    }
import-scrivener-report-footnotes = { $count ->
        [one] 1 voetnoot
       *[other] { $count } voetnoten
    }
import-scrivener-report-pictures = { $count ->
        [one] 1 afbeelding
       *[other] { $count } afbeeldingen
    }
import-scrivener-report-templates = { $count ->
        [one] 1 notitiesjabloon
       *[other] { $count } notitiesjablonen
    }
import-scrivener-report-tags = { $count ->
        [one] 1 tag
       *[other] { $count } tags
    }
import-scrivener-report-statuses = { $count ->
        [one] 1 status
       *[other] { $count } statussen
    }
import-scrivener-report-trashed = { $count ->
        [one] 1 item in de prullenbak
       *[other] { $count } items in de prullenbak
    }
import-scrivener-report-research-files = { $count ->
        [one] 1 onderzoeksbestand
       *[other] { $count } onderzoeksbestanden
    }
import-scrivener-report-choices = Wat u hebt gekozen
import-scrivener-report-accepted-mobile = U hebt het project geïmporteerd zonder de wijzigingen uit Scrivener voor iPhone of iPad die er nog niet in stonden.
import-scrivener-report-accepted-no-text = U hebt deze kopie geïmporteerd met lege documenten, omdat er geen enkele tekst van in stond.
import-scrivener-report-choice = { $subject }: { $answer }
import-scrivener-report-yes = ja
import-scrivener-report-no = nee
import-scrivener-report-research-copied = Onderzoeksbestanden die Skribisto niet kan openen, zijn gekopieerd naar “{ $folder }”, naast het nieuwe project.
import-scrivener-report-research-not-copied = Onderzoeksbestanden die Skribisto niet kan openen, zijn niet gekopieerd. Hun notities zeggen waar ze in uw Scrivener-project staan.
import-scrivener-report-research-copied-except = { $count ->
        [one] Onderzoeksbestanden die Skribisto niet kan openen, zijn gekopieerd naar “{ $folder }”, naast het nieuwe project, behalve 1 dat in uw Scrivener-project blijft. De notitie ervan zegt waar het staat.
       *[other] Onderzoeksbestanden die Skribisto niet kan openen, zijn gekopieerd naar “{ $folder }”, naast het nieuwe project, behalve { $count } die in uw Scrivener-project blijven. Hun notities zeggen waar ze staan.
    }
import-scrivener-report-research-none-copied = { $count ->
        [1] U hebt gevraagd om onderzoeksbestanden te kopiëren, maar het enige bestand kon niet worden gekopieerd. Het blijft in uw Scrivener-project, en de notitie ervan zegt waar het staat.
       *[other] U hebt gevraagd om onderzoeksbestanden te kopiëren, maar geen van de { $count } kon worden gekopieerd. Ze blijven in uw Scrivener-project, en hun notities zeggen waar ze staan.
    }
import-scrivener-report-chose-project = U hebt ervoor gekozen “{ $project }” te importeren, een van meerdere projecten in de gekozen map of het gekozen archief.
import-scrivener-report-chose-manifest = U hebt het ordnerbestand “{ $file }” gekozen.
import-scrivener-report-chose-enclosing = U hebt iets binnen “{ $project }” gekozen, dus het hele project is geïmporteerd.
import-scrivener-report-chose-upgraded = U hebt “{ $current }” geïmporteerd, het project waarin u verder hebt geschreven, en niet de kopie die Scrivener vóór een update bewaarde.
import-scrivener-report-chose-backup = U hebt “{ $backup }” geïmporteerd, de oudere kopie die Scrivener vóór een update bewaarde.
import-scrivener-report-title-placeholder-rejected = De titel in de instellingen van het project was een plaatshouder uit een sjabloon, dus Skribisto heeft in plaats daarvan de naam van het project voorgesteld.
import-scrivener-report-author-placeholder-rejected = De auteursnaam in de instellingen van het project was een plaatshouder uit een sjabloon, dus Skribisto heeft er geen voorgesteld.
import-scrivener-report-title-from-compile-settings = Skribisto heeft de titel voorgesteld die in de compileerinstellingen van het project staat: de naam die het project bij het aanmaken kreeg, tenzij daar in Scrivener een titel is ingesteld.
import-scrivener-report-title-from-template = Skribisto heeft de eigen titel van het sjabloon voorgesteld.
import-scrivener-report-title-from-folder-name = Skribisto heeft de naam van het project als titel voorgesteld.
import-scrivener-report-title-from-project-properties = Skribisto heeft de titel voorgesteld die in de projecteigenschappen staat.
import-scrivener-report-author-from-project = Skribisto heeft de auteursnaam voorgesteld die in de instellingen van het project staat. Daar kan de naam van de auteur van een sjabloon zijn blijven staan, en die zou op een titelpagina worden afgedrukt.
import-scrivener-report-language-not-stated = Het project vermeldde zijn taal niet, dus Skribisto heeft de taal voorgesteld waarin het zelf wordt weergegeven. De taal bepaalt hoe woorden worden geteld en hoe aanhalingstekens worden geschreven.
import-scrivener-report-language-from-script = Skribisto heeft de taal voorgesteld op basis van het schrift waarin het manuscript is geschreven ({ $tag }).
import-scrivener-report-you-changed-it = { $hint } U hebt dit voorstel vervangen.
import-scrivener-report-content-salvaged-text = De tekst van “{ $item }” ontbrak in het project. De woorden komen uit de zoekindex van Scrivener, als platte tekst en misschien uit een oudere versie: controleer ze.
import-scrivener-report-content-salvaged-notes = De notities van “{ $item }” ontbraken in het project. Hun woorden komen uit de zoekindex van Scrivener, als platte tekst en misschien uit een oudere versie: controleer ze.
import-scrivener-report-content-salvaged-synopsis = De synopsis van “{ $item }” ontbrak in het project. De woorden ervan komen uit de zoekindex van Scrivener, als platte tekst en misschien uit een oudere versie: controleer ze.
import-scrivener-report-document-others = { $others ->
        [one] { $sentence } Hetzelfde gebeurt op nog 1 plek, hieronder vermeld.
       *[other] { $sentence } Hetzelfde gebeurt op nog { $others } plekken, hieronder vermeld.
    }
import-scrivener-report-item-notes = { $path } (documentnotities)
import-scrivener-report-item-synopsis = { $path } (synopsis)
import-scrivener-report-item-project-note = { $path } (projectnotitie)
import-scrivener-report-item-recovered = { $path } (teruggevonden tekst)
import-scrivener-report-untitled = Naamloos
import-scrivener-report-no-findings = De import heeft verder niets te melden.
import-scrivener-report-group-unread = Kon niet worden gelezen
import-scrivener-report-group-check = Na te kijken
import-scrivener-report-group-changed = Anders overgenomen
import-scrivener-report-group-left = Achtergebleven
import-scrivener-report-group-count = { $group } ({ $count })
import-scrivener-report-items-toggle = { $count ->
        [one] 1 item
       *[other] { $count } items
    }
import-scrivener-report-items-toggle-named = { $count ->
        [one] 1 item: { $line }
       *[other] { $count } items: { $line }
    }
import-scrivener-report-names-toggle = { $count ->
        [one] 1 naam
       *[other] { $count } namen
    }
import-scrivener-report-names-toggle-named = { $count ->
        [one] 1 naam: { $line }
       *[other] { $count } namen: { $line }
    }
import-scrivener-report-more = { $count ->
        [one] En nog 1.
       *[other] En nog { $count }.
    }
import-scrivener-report-open-item = Openen in de editor
import-scrivener-report-links-hint = Open het nieuwe project en toon dit rapport daarna opnieuw vanuit het opdrachtenpalet: elk item dat hier staat, opent dan met een klik.
import-scrivener-copy-report = Rapport kopiëren
import-scrivener-report-copied = Rapport gekopieerd
import-scrivener-report-copy-failed = Het rapport kon niet worden gekopieerd.
import-scrivener-report-close = Sluiten
import-scrivener-report-none-title = Geen importrapport
import-scrivener-report-none = Sinds Skribisto is gestart, is er geen Scrivener-import voltooid. Skribisto bewaart de rapporten van de laatste imports totdat het wordt afgesloten.
import-scrivener-report-gone = { $count ->
        [one] Het rapport van deze import wordt niet meer bewaard. Skribisto bewaart het rapport van de laatste import totdat het wordt afgesloten.
       *[other] Het rapport van deze import wordt niet meer bewaard. Skribisto bewaart de rapporten van de laatste { $count } imports totdat het wordt afgesloten.
    }

## Words the import writes into the new project, in the language Skribisto shows itself in
import-scrivener-manuscript-binder = Manuscript
import-scrivener-notes-binder = Notities
import-scrivener-notes-of = { $title } (notities)
import-scrivener-project-notes = Projectnotities
import-scrivener-recovered = Teruggevonden teksten
import-scrivener-untitled-template = Naamloos sjabloon
import-scrivener-research-folder = Onderzoeksbestanden van { $name }
import-scrivener-list-separator = {", "}
import-scrivener-label-details = Uit het veld “{ $field }” in Scrivener
import-scrivener-status-details = Uit het veld “{ $field }” in Scrivener
import-scrivener-unnamed-label = Label { $number }
import-scrivener-unnamed-status = Status { $number }
import-scrivener-label-tag = { $name } (label)
import-scrivener-untitled-tag = Naamloos
import-scrivener-keyword-details = Het trefwoord “{ $path }” in Scrivener
import-scrivener-unnamed-keyword = Trefwoord { $number }
import-scrivener-keyword-tag = { $name } (trefwoord)
import-scrivener-collection-details = Een verzameling in Scrivener
import-scrivener-collection-tag = { $name } (verzameling)
import-scrivener-field-details = Uit het veld “{ $field }” in Scrivener
import-scrivener-field-value = { $field }: { $option }
import-scrivener-field-tag = { $name } (veld)
import-scrivener-deadline-day = Deadline
import-scrivener-from-scrivener = Uit Scrivener
import-scrivener-note-project = Project
import-scrivener-note-project-bookmarks = Projectbladwijzers
import-scrivener-note-replacements = Vervangingen bij het compileren
import-scrivener-note-auto-complete = Termen voor automatisch aanvullen
import-scrivener-note-saved-searches = Opgeslagen zoekopdrachten
import-scrivener-note-custom-metadata = Aangepaste metagegevens
import-scrivener-note-targets = Doelen
import-scrivener-note-bookmarks = Bladwijzers
import-scrivener-abbreviated-title = Verkorte titel
import-scrivener-subjects = Onderwerpen
import-scrivener-company = Bedrijf
import-scrivener-session-target = Sessiedoel
import-scrivener-session-target-calculated = Sessiedoel, door Scrivener berekend op basis van de deadline
import-scrivener-writing-days = Schrijfdagen
import-scrivener-weekday-monday = maandag
import-scrivener-weekday-tuesday = dinsdag
import-scrivener-weekday-wednesday = woensdag
import-scrivener-weekday-thursday = donderdag
import-scrivener-weekday-friday = vrijdag
import-scrivener-weekday-saturday = zaterdag
import-scrivener-weekday-sunday = zondag
import-scrivener-draft-target = Manuscriptdoel
import-scrivener-deadline = Deadline
import-scrivener-unit-words = woorden
import-scrivener-unit-characters = tekens
import-scrivener-unit-pages = pagina’s
import-scrivener-search-query = Gezocht naar
import-scrivener-search-in = Gezocht in
import-scrivener-search-operator = Zoekwijze
import-scrivener-search-in-all = alles
import-scrivener-search-in-title = titels
import-scrivener-search-in-text = tekst
import-scrivener-search-in-notes = notities
import-scrivener-search-in-synopsis = synopsissen
import-scrivener-search-in-keywords = trefwoorden
import-scrivener-search-in-label = labels
import-scrivener-search-in-status = statussen
import-scrivener-search-in-section-type = sectietypen
import-scrivener-search-in-custom-metadata = aangepaste metagegevens
import-scrivener-search-any = een van de woorden
import-scrivener-search-all = alle woorden
import-scrivener-search-exact = exacte woordgroep
import-scrivener-label-value = { $label }: { $value }
import-scrivener-replacement-regex = reguliere expressie
import-scrivener-replacement-off = uitgeschakeld
import-scrivener-in-own-note = Vermeld in de notitie die van dit document is gemaakt.

## Import a Scrivener project: what the import says about the project, in the writer's words.
## Every key the converter can give has a line here (see `import_scrivener::messages`).
import-scrivener-paragraph = { $title }. { $body }

## When the project cannot be imported
## A body that ends “then read the project again” is shown in the wizard beside the button
## import-scrivener-check-again, so it uses the same words. A failed import's notice shows
## the same body beside import-scrivener-review-again, which reads the project again too:
## the body quotes no button.
import-scrivener-refusal-missing-title = Op dit pad staat niets
import-scrivener-refusal-missing-body = “{ $file }” is er niet meer. Kies het project opnieuw.
import-scrivener-refusal-unreadable-title = Dit project kan niet worden gelezen
import-scrivener-refusal-unreadable-body = Skribisto kon “{ $file }” niet lezen ({ $reason }). Controleer of u het kunt openen en probeer het daarna opnieuw.
import-scrivener-refusal-not-a-scrivener-project-title = Hier staat geen Scrivener-project
import-scrivener-refusal-not-a-scrivener-project-body = Kies de .scriv-map van het project, het .scrivx-bestand erin, een .zip-reservekopie of een .scrivtemplate.
import-scrivener-refusal-manifest-without-files-title = De documenten staan niet in deze kopie
import-scrivener-refusal-manifest-without-files-body = “{ $manifest }” staat hier, maar niet de map Files met de documenten die het vermeldt. Een synchronisatiedienst is misschien nog niet klaar met het kopiëren van het project, of alleen het .scrivx-bestand is gekopieerd. Als u het .scrivx-bestand hebt gekozen, kies dan in plaats daarvan de map van het project met “Map kiezen…”.
import-scrivener-refusal-search-too-wide-title = Deze map bevat te veel om te doorzoeken
import-scrivener-refusal-search-too-wide-body = Er zijn meer dan { $limit } mappen te doorzoeken. Kies het Scrivener-project zelf.
import-scrivener-refusal-scrivener-one-for-mac-title = Dit project is gemaakt met Scrivener 1 voor Mac
import-scrivener-refusal-scrivener-one-for-mac-body = Skribisto leest projecten uit Scrivener 2 en later. Open het één keer in Scrivener 2 of 3 om het bij te werken, of compileer het naar een Word-document en importeer dat.
import-scrivener-refusal-ios-stub-without-mobile-data-title = Dit is alleen een verwijsbestand van Scrivener voor iPhone of iPad
import-scrivener-refusal-ios-stub-without-mobile-data-body = Het project zelf staat hier niet. Synchroniseer het vanaf uw iPhone of iPad, of deel het van daaruit als .zip, en importeer dat.
import-scrivener-refusal-newer-title = Dit project is gemaakt met een nieuwere versie van Scrivener
import-scrivener-refusal-unknown-format-version-body = Het formaat ervan, { $version }, kent Skribisto niet. Kijk of er een update van Skribisto is. Compileer het tot die tijd in Scrivener naar een Word-document en importeer dat.
import-scrivener-refusal-newer-format-stamp-body = De bestanden ervan zijn gemarkeerd als versie { $stamp }, en Skribisto leest tot en met versie { $newest }. Kijk of er een update van Skribisto is. Compileer het tot die tijd in Scrivener naar een Word-document en importeer dat.
import-scrivener-refusal-version-stamp-unreadable-title = De versie van het project kan niet worden gelezen
import-scrivener-refusal-version-stamp-unreadable-body = Het bestand Files/version.txt bevat geen versienummer, dus Skribisto kan niet nagaan of het dit project kan lezen. Open het project één keer in Scrivener en importeer het daarna opnieuw.
import-scrivener-refusal-damaged-manifest-title = Het ordnerbestand kan niet worden gelezen
import-scrivener-refusal-damaged-manifest-body = “{ $manifest }” is beschadigd. { $spare_copies ->
        [0] Open het project in Scrivener, dat het kan herstellen, en importeer het daarna opnieuw.
        [1] Scrivener bewaart in het project een extra kopie ervan, die Skribisto niet leest. Open het project in Scrivener, dat het kan herstellen, en importeer het daarna opnieuw.
       *[other] Scrivener bewaart in het project { $spare_copies } extra kopieën ervan, die Skribisto niet leest. Open het project in Scrivener, dat het kan herstellen, en importeer het daarna opnieuw.
    }
import-scrivener-refusal-damaged-manifest-new-project-placeholder-body = “{ $manifest }” is het lege bestand dat Scrivener schrijft voor een project dat het nog niet heeft opgeslagen. Open het project in Scrivener, laat het opslaan en importeer het daarna opnieuw.
import-scrivener-refusal-manifest-too-large-title = Dit ordnerbestand is te groot
import-scrivener-refusal-manifest-too-large-body = “{ $manifest }” is groter dan de { $limit } MB die Skribisto leest. Geen echt project komt daar in de buurt, dus het bestand is misschien beschadigd.
import-scrivener-refusal-no-item-identifiers-title = De ordner vermeldt geen document dat Skribisto kan vinden
import-scrivener-refusal-no-item-identifiers-body = Geen van de items heeft een ID, dus hun bestanden kunnen niet worden gevonden. Open het project één keer in Scrivener en importeer het daarna opnieuw.
import-scrivener-refusal-incomplete-copy-title = De documenten ontbreken in deze kopie
import-scrivener-refusal-incomplete-copy-body = De map Files staat hier, maar de documenten erin niet. Een synchronisatiedienst is misschien nog niet klaar met het kopiëren van het project. Wacht tot die klaar is, of kopieer de hele .scriv-map opnieuw, en lees het project daarna opnieuw in.
import-scrivener-refusal-archive-title = Dit archief kan niet worden geopend
import-scrivener-refusal-archive-unreadable-body = “{ $file }” kon niet worden gelezen ({ $reason }). Het is misschien beschadigd. Pak het zelf uit en kies daarna de .scriv-map erin.
import-scrivener-refusal-archive-unsafe-name-title = Dit archief kan niet veilig worden geopend
import-scrivener-refusal-archive-unsafe-name-body = “{ $file }” bevat een bestand met de naam “{ $member }”, dat buiten het archief terecht zou komen, dus Skribisto opent het niet.
import-scrivener-refusal-archive-unsafe-name-empty-body = “{ $file }” bevat een bestand zonder naam, dus Skribisto opent het niet.
import-scrivener-refusal-archive-unsafe-name-control-character-body = “{ $file }” bevat een bestand met een stuurteken in de naam, dus Skribisto opent het niet.
import-scrivener-refusal-archive-too-large-title = Dit archief is te groot om veilig te openen
import-scrivener-refusal-archive-too-many-members-body = Het bevat meer dan { $limit } bestanden. Pak het zelf uit en kies daarna de .scriv-map erin.
import-scrivener-refusal-archive-member-too-large-body = “{ $member }” wordt uitgepakt groter dan { $limit } MB. Pak het archief zelf uit en kies daarna de .scriv-map erin.
import-scrivener-refusal-archive-too-large-body = Het wordt uitgepakt groter dan { $limit } MB. Pak het zelf uit en kies daarna de .scriv-map erin.
import-scrivener-refusal-archive-encrypted-title = Dit archief is met een wachtwoord beveiligd
import-scrivener-refusal-archive-encrypted-body = Pak “{ $file }” zelf uit en kies daarna de .scriv-map erin.
import-scrivener-refusal-archive-unsupported-compression-title = Dit archief is ingepakt op een manier die Skribisto niet kan lezen
import-scrivener-refusal-archive-unsupported-compression-body = “{ $member }” is gecomprimeerd met { $method }. Pak het archief zelf uit en kies daarna de .scriv-map erin.
import-scrivener-refusal-duplicate-archive-members-title = Dit archief vermeldt sommige bestanden twee keer
import-scrivener-refusal-duplicate-archive-members-body = “{ $file }” bevat meer dan één bestand met dezelfde naam, dus Skribisto kan niet nagaan welk bestand bij het project hoort. Pak het zelf uit en kies daarna de .scriv-map erin.
import-scrivener-refusal-template-undecodable-title = Dit sjabloonbestand is onvolledig
import-scrivener-refusal-template-undecodable-too-large-title = Dit sjabloonbestand is te groot om te openen
import-scrivener-refusal-template-undecodable-not-a-template-title = Dit bestand is geen Scrivener-sjabloon
import-scrivener-refusal-template-undecodable-unknown-encoding-title = Dit sjabloonbestand is opgeslagen in een vorm die Skribisto niet kent
import-scrivener-refusal-template-undecodable-body = “{ $file }” kan niet worden gedecodeerd: het bestand houdt misschien eerder op dan het project erin. Download het opnieuw, of open het in Scrivener en maak er een project van.
import-scrivener-refusal-template-undecodable-too-large-body = “{ $file }” is groter dan { $limit } MB. Open het in Scrivener, maak er een project van en importeer dat project.
import-scrivener-refusal-template-undecodable-not-a-template-body = “{ $file }” bevat geen Scrivener-project. Kies het sjabloonbestand dat Scrivener heeft opgeslagen, of een project dat ervan is gemaakt.
import-scrivener-refusal-template-undecodable-unknown-encoding-body = “{ $file }” slaat zijn project op een manier op die Skribisto niet kan decoderen. Open het in Scrivener, maak er een project van en importeer dat project.
import-scrivener-refusal-cloud-placeholders-title = { $count ->
        [one] 1 bestand van dit project staat nog in de cloud
       *[other] { $count } bestanden van dit project staan nog in de cloud
    }
import-scrivener-refusal-cloud-placeholders-body = { $count ->
        [1] Dit bestand is niet naar deze computer gedownload: { $examples }. Maak het project offline beschikbaar in uw synchronisatie-app en lees het project daarna opnieuw in.
       *[other] Ze zijn niet naar deze computer gedownload, bijvoorbeeld { $examples }. Maak het project offline beschikbaar in uw synchronisatie-app en lees het project daarna opnieuw in.
    }
import-scrivener-refusal-unreadable-members-title = { $count ->
        [one] 1 bestand van dit project kan niet worden gelezen
       *[other] { $count } bestanden van dit project kunnen niet worden gelezen
    }
import-scrivener-refusal-unreadable-members-body = Bijvoorbeeld { $examples }. Controleer of u het hele project mag lezen en of de schijf in orde is, en lees het project daarna opnieuw in.
import-scrivener-refusal-needed-member-is-link-title = Dit project verwijst naar buiten zijn eigen map
import-scrivener-refusal-needed-member-is-link-body = “{ $member }” is een koppeling naar een plek elders op deze computer, en Skribisto volgt geen koppelingen. Kopieer het project zo dat elk bestand erin staat, en probeer het daarna opnieuw.
import-scrivener-refusal-project-changed-during-read-title = Het project is gewijzigd terwijl het werd gelezen
import-scrivener-refusal-project-changed-during-read-body = Scrivener of een synchronisatiedienst schrijft er nog naar. Sluit het in Scrivener, wacht tot de synchronisatie klaar is en lees het project daarna opnieuw in.
import-scrivener-refusal-too-many-members-title = Dit project bevat te veel bestanden
import-scrivener-refusal-too-many-members-body = Het heeft meer dan { $limit } bestanden, meer dan welk echt project ook. Het is misschien geen Scrivener-project, of het is beschadigd.
import-scrivener-refusal-too-large-title = Dit project is te groot
import-scrivener-refusal-too-large-body = De tekst ervan beslaat meer dan { $limit } MB, meer dan Skribisto veilig kan importeren.
import-scrivener-refusal-structure-unmappable-title = Skribisto kan deze ordner niet overnemen
import-scrivener-refusal-structure-unmappable-body = De ordner kon niet worden overgenomen zonder iets te verliezen of op de verkeerde plek te zetten. Dit is een fout in Skribisto, niet in uw project. Meld het alstublieft, met wat “Details” toont.

## When there is a choice to make
import-scrivener-roots-title = { $count ->
        [one] Hier staat { $count } project. Welk project?
       *[other] Hier staan { $count } projecten. Welk project?
    }
import-scrivener-roots-hint = Deze map of dit archief bevat meer dan één Scrivener-project. Kies het project dat u wilt importeren.
import-scrivener-manifests-title = { $count ->
        [one] Dit project heeft { $count } ordnerbestand
       *[other] Dit project heeft { $count } ordnerbestanden
    }
import-scrivener-manifests-hint = Scrivener bewaart de ordner, de lijst van elk document en elke map, in een .scrivx-bestand. De extra bestanden zijn hoogstwaarschijnlijk achtergelaten door een synchronisatiedienst. Kies het bestand waarin u het laatst hebt gewerkt.
import-scrivener-manifests-hint-preselected = Scrivener bewaart de ordner, de lijst van elk document en elke map, in een .scrivx-bestand. De extra bestanden zijn hoogstwaarschijnlijk achtergelaten door een synchronisatiedienst. Het geselecteerde bestand is waarschijnlijk het bestand waarin u het laatst hebt gewerkt: controleer dat voordat u verdergaat.
import-scrivener-manifest-row = { $file } · opgeslagen op { $date } · { $documents ->
        [one] 1 item
       *[other] { $documents } items
    }
import-scrivener-manifest-row-undated = { $file } · { $documents ->
        [one] 1 item
       *[other] { $documents } items
    }
import-scrivener-manifest-row-unread = { $file } · kan niet worden gelezen
import-scrivener-enclosing-title = Dit maakt deel uit van het Scrivener-project “{ $project }”
import-scrivener-enclosing-body = U hebt een bestand of map binnen een project gekozen. Skribisto importeert het hele project.
import-scrivener-enclosing-use = “{ $project }” importeren
import-scrivener-upgrade-title = Dit is de kopie die Scrivener vóór een update bewaarde
import-scrivener-upgrade-body = Wanneer Scrivener een project bijwerkt naar een nieuwere versie, bewaart het het oude als “{ $backup }”. Het project waarin u verder hebt geschreven, is “{ $current }”, ernaast.
import-scrivener-upgrade-body-unclear = Wanneer Scrivener een project bijwerkt naar een nieuwere versie, bewaart het het oude als “{ $backup }”. Het project ernaast, “{ $current }”, kon niet goed worden gelezen, dus kies welk van de twee u wilt importeren.
import-scrivener-upgrade-use-current = “{ $current }” importeren, het project waarin u verder hebt geschreven
import-scrivener-upgrade-keep-backup = Deze oudere kopie importeren, “{ $backup }”

## What must be accepted before going on
import-scrivener-ios-pending-title = Wijzigingen uit Scrivener voor iPhone of iPad staan nog niet in deze kopie
import-scrivener-ios-pending-body = Scrivener op een computer haalt die wijzigingen binnen wanneer het het project de volgende keer opent. Open het project één keer in Scrivener op uw computer en importeer het daarna opnieuw.
import-scrivener-ios-pending-accept = Importeren zonder die wijzigingen
import-scrivener-no-text-title = { $count ->
        [one] 1 document, en geen enkel woord tekst in deze kopie
       *[other] { $count } documenten, en geen enkel woord tekst in deze kopie
    }
import-scrivener-no-text-body = Een synchronisatiedienst is misschien nog bezig het project te kopiëren, of de kopie is haar documenten kwijtgeraakt. Als u nu importeert, komt elk document leeg binnen.
import-scrivener-no-text-accept = De documenten leeg importeren

## Things to know, nothing to decide
import-scrivener-notice-project-found-below-pick-title = Het project is gevonden in de map die u koos
import-scrivener-notice-project-found-below-pick-body = Het staat op “{ $path }”.
import-scrivener-notice-source-is-template-title = Dit is een Scrivener-sjabloon
import-scrivener-notice-source-is-template-named = Dit is het Scrivener-sjabloon “{ $title }”
import-scrivener-notice-source-is-template-body = Als u het importeert, wordt er een nieuw project gemaakt van wat het sjabloon bevat, de instructies inbegrepen.
import-scrivener-notice-source-is-backup-title = Dit lijkt een reservekopie die Scrivener heeft gemaakt
import-scrivener-notice-source-is-backup-body = Het bevat het project zoals het was toen de reservekopie werd gemaakt. Als u daarna verder hebt geschreven, kies dan het project zelf.
import-scrivener-notice-importing-pre-upgrade-copy-title = U importeert de kopie die Scrivener vóór een update bewaarde
import-scrivener-notice-importing-pre-upgrade-copy-body = Het project waarin u verder hebt geschreven, staat ernaast en bevat alles wat sindsdien is geschreven.
import-scrivener-notice-imported-from-ios-stub-title = Dit project is gemaakt in Scrivener voor iPhone of iPad
import-scrivener-notice-imported-from-ios-stub-body = Het is nooit op een computer geopend, dus het wordt gelezen uit de kopie die de iPhone of iPad bewaart.
import-scrivener-notice-stale-mobile-binder-ignored-title = Een oude ordner van een iPhone of iPad wordt opzijgelegd
import-scrivener-notice-stale-mobile-binder-ignored-body = Scrivener op een computer heeft de wijzigingen ervan al binnengehaald.
import-scrivener-notice-identical-manifest-copies-title = { $count ->
        [one] Dit project heeft { $count } identiek ordnerbestand
       *[other] Dit project heeft { $count } identieke ordnerbestanden
    }
import-scrivener-notice-identical-manifest-copies-body = Ze bevatten hetzelfde, dus elk ervan levert hetzelfde project op.
import-scrivener-notice-manifest-copies-ignored-title = Extra kopieën van het ordnerbestand worden opzijgelegd
import-scrivener-notice-manifest-copies-ignored-body = “{ $chosen }” is het bestand waarin Scrivener het laatst heeft gewerkt. { $count ->
        [one] 1 kopie die een synchronisatiedienst heeft achtergelaten, wordt opzijgelegd.
       *[other] { $count } kopieën die een synchronisatiedienst heeft achtergelaten, worden opzijgelegd.
    }
import-scrivener-notice-pinned-manifest-looks-like-a-copy-title = Het ordnerbestand dat u koos, lijkt een synchronisatiekopie
import-scrivener-notice-pinned-manifest-looks-like-a-copy-body = “{ $name }” heeft de naam van een kopie van “{ $original }”, dat ernaast staat. Als de import er oud uitziet, kies dan in plaats daarvan “{ $original }”.
import-scrivener-notice-pinned-manifest-has-siblings-title = { $count ->
        [one] Naast het bestand dat u koos, staat nog 1 ordnerbestand
       *[other] Naast het bestand dat u koos, staan nog { $count } ordnerbestanden
    }
import-scrivener-notice-pinned-manifest-has-siblings-body = Alleen het bestand dat u koos, wordt gelezen.
import-scrivener-notice-legacy-v1-files-beside-title = Bestanden van een oudere versie van Scrivener worden opzijgelegd
import-scrivener-notice-legacy-v1-files-beside-body = Ze zijn achtergebleven toen het project werd bijgewerkt. Alleen de bestanden van de huidige versie worden gelezen.
import-scrivener-notice-mixed-payload-trees-title = Dit project bevat bestanden van twee versies van Scrivener
import-scrivener-notice-mixed-payload-trees-body = De documenten worden gelezen op de plek waar het ordnerbestand zegt dat ze staan.
import-scrivener-notice-version-disagreement-title = De bestanden van dit project zijn het niet eens over de versie ervan
import-scrivener-notice-id-scheme-mismatch-body = Het ordnerbestand zegt versie { $version }, maar benoemt de documenten zoals een andere versie dat doet. Ze worden gelezen op de plek waarnaar het verwijst. Als de import er verkeerd uitziet, open het project dan één keer in Scrivener en importeer het opnieuw.
import-scrivener-notice-version-stamp-mismatch-body = Het wordt gelezen zoals het ordnerbestand het beschrijft. Als de import er verkeerd uitziet, open het project dan één keer in Scrivener en importeer het opnieuw.
import-scrivener-notice-unrecognised-writer-title = Dit project is opgeslagen door een programma dat Skribisto niet herkent
import-scrivener-notice-unrecognised-writer-named = Het noemt zichzelf “{ $creator }”. Het wordt gelezen als Scrivener-project.
import-scrivener-notice-unrecognised-writer-body = Het noemt zijn eigen naam niet. Het wordt gelezen als Scrivener-project.
import-scrivener-notice-invalid-item-identifier-title = { $count ->
        [one] 1 item in de ordner heeft een ID dat geen geldig ID is
       *[other] { $count } items in de ordner hebben een ID dat geen geldig ID is
    }
import-scrivener-notice-invalid-item-identifier-body = Zo’n ID wordt opzijgelegd, en het item dat die draagt, blijft behouden.
import-scrivener-notice-items-without-identifier-title = { $count ->
        [one] 1 item in de ordner heeft geen ID
       *[other] { $count } items in de ordner hebben geen ID
    }
import-scrivener-notice-items-without-identifier-body = Zonder ID kunnen de bestanden van een item niet worden gevonden, dus het komt leeg binnen.
import-scrivener-notice-duplicate-item-key-title = Twee items in de ordner delen één ID: “{ $item }”
import-scrivener-notice-duplicate-item-key-body = Het eerste houdt de documenten. Het tweede houdt zijn plek en komt leeg binnen.
import-scrivener-notice-lock-here-title = Scrivener heeft dit project geopend op deze computer
import-scrivener-notice-lock-here-body = Sluit het eerst in Scrivener, zodat elke wijziging is opgeslagen.
import-scrivener-notice-lock-title = Scrivener had dit project geopend op “{ $host }”
import-scrivener-notice-lock-elsewhere-title = Scrivener had dit project geopend op een andere computer
import-scrivener-notice-lock-body = Als het daar nog open is, sluit het dan eerst: de bestanden bevatten misschien nog niet de laatste wijzigingen. Als Scrivener al lang geleden is afgesloten, kunt u het importeren zoals het is.
import-scrivener-notice-archived-while-open-title = Dit archief is gemaakt terwijl Scrivener het project geopend had
import-scrivener-notice-archived-while-open-body = Wijzigingen van na dat moment ontbreken er misschien in.
import-scrivener-notice-content-missing-notes = De notities van “{ $item }” ontbreken
import-scrivener-notice-content-missing-synopsis = De synopsis van “{ $item }” ontbreekt
import-scrivener-notice-content-missing-text = De tekst van “{ $item }” ontbreekt
import-scrivener-notice-content-missing-body = { $place ->
        [notes] Het bestand met de notities staat niet in deze kopie, dus ze komen leeg binnen. Een synchronisatiedienst is misschien nog bezig het project te kopiëren.
        [synopsis] Het bestand met de synopsis staat niet in deze kopie, dus die komt leeg binnen. Een synchronisatiedienst is misschien nog bezig het project te kopiëren.
       *[text] Het bestand met de tekst staat niet in deze kopie, dus die komt leeg binnen. Een synchronisatiedienst is misschien nog bezig het project te kopiëren.
    }
import-scrivener-notice-content-missing-salvaged = { $place ->
        [notes] Het bestand met de notities staat niet in deze kopie, maar de zoekindex van Scrivener bevatte de woorden ervan nog. Ze worden bewaard als platte tekst, misschien uit een oudere versie.
        [synopsis] Het bestand met de synopsis staat niet in deze kopie, maar de zoekindex van Scrivener bevatte de woorden ervan nog. Ze worden bewaard als platte tekst, misschien uit een oudere versie.
       *[text] Het bestand met de tekst staat niet in deze kopie, maar de zoekindex van Scrivener bevatte de woorden ervan nog. Ze worden bewaard als platte tekst, misschien uit een oudere versie.
    }
import-scrivener-notice-content-truncated-notes = De notities van “{ $item }” zijn leeg op schijf
import-scrivener-notice-content-truncated-synopsis = De synopsis van “{ $item }” is leeg op schijf
import-scrivener-notice-content-truncated-text = De tekst van “{ $item }” is leeg op schijf
import-scrivener-notice-content-truncated-body = { $place ->
        [notes] Volgens Scrivener bevatten de notities tekst, maar hun bestand is leeg: een synchronisatie is misschien halverwege gestopt. Ze komen leeg binnen.
        [synopsis] Volgens Scrivener bevat de synopsis tekst, maar het bestand ervan is leeg: een synchronisatie is misschien halverwege gestopt. De synopsis komt leeg binnen.
       *[text] Volgens Scrivener is de tekst niet leeg, maar het bestand ervan bevat niets: een synchronisatie is misschien halverwege gestopt. De tekst komt leeg binnen.
    }
import-scrivener-notice-missing-media-title = Het bestand van “{ $item }” ontbreekt
import-scrivener-notice-missing-media-body = De afbeelding of het onderzoeksbestand dat het bevat, staat niet in deze kopie.
import-scrivener-notice-sync-conflict-copies-title = { $count ->
        [one] Er is 1 synchronisatiekopie van een document gevonden
       *[other] Er zijn { $count } synchronisatiekopieën van documenten gevonden
    }
import-scrivener-notice-sync-conflict-copies-body = { $missing ->
        [0] Synchronisatiekopieën worden overgenomen als teruggevonden teksten, in een eigen map, zodat u ze met de documenten kunt vergelijken.
        [one] Synchronisatiekopieën worden overgenomen als teruggevonden teksten, in een eigen map. { $missing } ervan is een kopie van een document dat niet meer in het project staat.
       *[other] Synchronisatiekopieën worden overgenomen als teruggevonden teksten, in een eigen map. { $missing } ervan zijn kopieën van documenten die niet meer in het project staan.
    }
import-scrivener-notice-recovered-unreferenced-text-title = { $count ->
        [one] Er is 1 document gevonden dat door geen enkel item in de ordner wordt vermeld
       *[other] Er zijn { $count } documenten gevonden die door geen enkel item in de ordner worden vermeld
    }
import-scrivener-notice-recovered-unreferenced-text-body = Zulke documenten worden overgenomen als teruggevonden teksten, in een eigen map.
import-scrivener-notice-sync-debris-ignored-title = { $count ->
        [one] 1 extra kopie die een synchronisatiedienst heeft achtergelaten, wordt opzijgelegd
       *[other] { $count } extra kopieën die een synchronisatiedienst heeft achtergelaten, worden opzijgelegd
    }
import-scrivener-notice-sync-debris-ignored-body = Scrivener leest zulke kopieën ook niet. Alleen de eigen bestanden van het project worden gelezen.
import-scrivener-notice-unrecognised-files-title = { $count ->
        [one] 1 bestand dat geen deel uitmaakt van een Scrivener-project, wordt opzijgelegd
       *[other] { $count } bestanden die geen deel uitmaken van een Scrivener-project, worden opzijgelegd
    }
import-scrivener-notice-examples = Bijvoorbeeld { $examples }.
import-scrivener-notice-symlink-not-followed-title = { $count ->
        [one] 1 koppeling in het project wordt niet gevolgd
       *[other] { $count } koppelingen in het project worden niet gevolgd
    }
import-scrivener-notice-unreadable-files-skipped-title = { $count ->
        [one] 1 bestand dat de import niet nodig heeft, kon niet worden gelezen
       *[other] { $count } bestanden die de import niet nodig heeft, konden niet worden gelezen
    }
import-scrivener-notice-cloud-placeholders-skipped-title = { $count ->
        [one] 1 bestand dat nog in de cloud staat, wordt overgeslagen
       *[other] { $count } bestanden die nog in de cloud staan, worden overgeslagen
    }
import-scrivener-notice-cloud-placeholders-skipped-body = Niets wat de import nodig heeft, staat nog in de cloud.
import-scrivener-notice-no-text-found-title = { $count ->
        [1] 1 document, en het bevat geen tekst
       *[other] { $count } documenten, en geen ervan bevat tekst
    }
import-scrivener-notice-no-text-found-body = Dit project lijkt leeg. Het wordt geïmporteerd zoals het is.
import-scrivener-notice-metadata-unreadable-title = “{ $member }” kon niet worden gelezen
import-scrivener-notice-metadata-unreadable-body = Wat Scrivener erin bewaarde, wordt niet overgenomen.
import-scrivener-notice-dangling-keyword-title = { $count ->
        [one] 1 verwijzing wijst naar een trefwoord dat het project niet definieert
       *[other] { $count } verwijzingen wijzen naar trefwoorden die het project niet definieert
    }
import-scrivener-notice-dangling-label-title = { $count ->
        [one] 1 verwijzing wijst naar een label dat het project niet definieert
       *[other] { $count } verwijzingen wijzen naar labels die het project niet definieert
    }
import-scrivener-notice-dangling-status-title = { $count ->
        [one] 1 verwijzing wijst naar een status die het project niet definieert
       *[other] { $count } verwijzingen wijzen naar statussen die het project niet definieert
    }
import-scrivener-notice-dangling-section-type-title = { $count ->
        [one] 1 verwijzing wijst naar een sectietype dat het project niet definieert
       *[other] { $count } verwijzingen wijzen naar sectietypen die het project niet definieert
    }
import-scrivener-notice-dangling-custom-field-title = { $count ->
        [one] 1 verwijzing wijst naar een veld voor aangepaste metagegevens dat het project niet definieert
       *[other] { $count } verwijzingen wijzen naar velden voor aangepaste metagegevens die het project niet definieert
    }
import-scrivener-notice-dangling-item-title = { $count ->
        [one] 1 verwijzing wijst naar een document dat het project niet definieert
       *[other] { $count } verwijzingen wijzen naar documenten die het project niet definieert
    }
import-scrivener-notice-dangling-references-body = Een verwijzing naar iets wat het project niet definieert, wordt weggelaten.
import-scrivener-notice-snapshots-not-imported-line = { $count ->
        [one] 1 momentopname blijft in uw Scrivener-project: Skribisto neemt geen momentopnamen over.
       *[other] { $count } momentopnamen blijven in uw Scrivener-project: Skribisto neemt geen momentopnamen over.
    }
import-scrivener-notice-writing-history-not-imported-line = { $days ->
        [one] Wat Scrivener per dag bijhield over het aantal geschreven woorden (1 dag), wordt niet overgenomen. Skribisto begint zijn eigen telling bij de eerste keer opslaan.
       *[other] Wat Scrivener per dag bijhield over het aantal geschreven woorden ({ $days } dagen), wordt niet overgenomen. Skribisto begint zijn eigen telling bij de eerste keer opslaan.
    }
import-scrivener-notice-title-placeholder-rejected-hint = De titel in de instellingen van het project is een plaatshouder uit een sjabloon, dus in plaats daarvan wordt de naam van het project voorgesteld.
import-scrivener-notice-author-placeholder-rejected-hint = De auteursnaam in de instellingen van het project is een plaatshouder uit een sjabloon, dus er wordt geen naam voorgesteld.
import-scrivener-notice-title-from-project-hint-compile-settings = Deze titel is de titel die in de compileerinstellingen van het project staat. Zolang daar geen titel is ingesteld, houdt Scrivener de naam aan die het project bij het aanmaken kreeg.
import-scrivener-notice-title-from-project-hint-template = Deze titel is de eigen titel van het sjabloon.
import-scrivener-notice-title-from-project-hint-folder-name = Deze titel is de naam van het project.
import-scrivener-notice-title-from-project-hint-project-properties = Deze titel is de titel die in de projecteigenschappen staat.
import-scrivener-notice-author-from-project-hint = Deze naam is de naam die in de instellingen van het project staat. Daar kan de naam van de auteur van een sjabloon zijn blijven staan, en die zou op een titelpagina worden afgedrukt.
import-scrivener-notice-language-not-stated-hint = Het project vermeldt zijn taal niet, dus wordt de taal voorgesteld waarin Skribisto zelf wordt weergegeven. De taal bepaalt hoe woorden worden geteld en hoe aanhalingstekens worden geschreven.
import-scrivener-notice-language-from-script-hint = Voorgesteld op basis van het schrift waarin het manuscript is geschreven ({ $tag }).

## Before you import: what the import does with the binder, or leaves behind
import-scrivener-structure-script-mode-draft-line = { $script ->
        [1] { $documents ->
            [1] Het enige manuscriptdocument is geschreven in scriptmodus. Skribisto behoudt de woorden ervan, niet de opmaak als scenario.
           *[other] { $script } van de { $documents } manuscriptdocumenten is geschreven in scriptmodus. Skribisto behoudt de woorden ervan, niet de opmaak als scenario.
        }
       *[other] { $script } van de { $documents } manuscriptdocumenten zijn geschreven in scriptmodus. Skribisto behoudt hun woorden, niet hun opmaak als scenario.
    }
import-scrivener-structure-one-document-manuscript-line = { $count ->
        [1] Een manuscriptdocument bevat zijn hoofdstukken als koppen.
       *[other] { $count } manuscriptdocumenten bevatten hun hoofdstukken als koppen.
    }
import-scrivener-structure-title-repeats-number-line = { $count ->
        [one] 1 titel van een hoofdstuk of deel bevat ook het eigen nummer, zoals “Hoofdstuk II” of “Eerste deel”, dus een export kan het nummer twee keer afdrukken.
       *[other] { $count } titels van hoofdstukken of delen bevatten ook hun eigen nummer, zoals “Hoofdstuk II” of “Eerste deel”, dus een export kan het nummer twee keer afdrukken.
    }
import-scrivener-structure-claim-overruled-line = { $count ->
        [one] 1 item lijkt een boek, een deel of een hoofdstuk, maar staat in een item dat al dat niveau heeft, dus het komt een niveau lager binnen.
       *[other] { $count } items lijken boeken, delen of hoofdstukken, maar elk staat in een item dat al dat niveau heeft, dus ze komen een niveau lager binnen.
    }
import-scrivener-structure-adopted-by-chapter-line = { $count ->
        [one] 1 document dat op een hoofdstuk volgt, wordt het einde van dat hoofdstuk.
       *[other] { $count } documenten die op een hoofdstuk volgen, worden het einde van dat hoofdstuk.
    }
import-scrivener-structure-looks-like-manuscript-line = { $count ->
        [one] 1 map op het hoogste niveau die bij de notities blijft, lijkt op manuscript. U kunt die als ander boek naar het manuscript sturen.
       *[other] { $count } mappen op het hoogste niveau die bij de notities blijven, lijken op manuscript. U kunt ze als andere boeken naar het manuscript sturen.
    }
import-scrivener-structure-generic-sheet-title-line = { $count ->
        [one] 1 item in de verhaalbijbel heeft nog de titel van een sjabloon, zoals “Held”. Skribisto zoekt die titel in uw tekst en toont elke plek waar die voorkomt in de weergave In proza: geef het item een andere naam als dat woord vaak voorkomt in uw boek.
       *[other] { $count } items in de verhaalbijbel hebben nog de titel van een sjabloon, zoals “Held”. Skribisto zoekt die titels in uw tekst en toont elke plek waar ze voorkomen in de weergave In proza: geef de items een andere naam als die woorden vaak voorkomen in uw boek.
    }
import-scrivener-structure-placeholder-title-line = { $count ->
        [one] 1 titel noemde alleen het niveau en wordt leeg gelaten: de ordner en de exports nummeren het item.
       *[other] { $count } titels noemden alleen hun niveau en worden leeg gelaten: de ordner en de exports nummeren de items.
    }
import-scrivener-structure-untitled-item-line = { $count ->
        [one] 1 item had in Scrivener geen titel en blijft naamloos.
       *[other] { $count } items hadden in Scrivener geen titel en blijven naamloos.
    }
import-scrivener-structure-title-cleaned-line = { $count ->
        [one] 1 titel bevatte regeleinden of tabs, die worden verwijderd.
       *[other] { $count } titels bevatten regeleinden of tabs, die worden verwijderd.
    }
import-scrivener-structure-own-text-moved-line = { $count ->
        [one] 1 map heeft een eigen tekst, die naar een eerste document erin wordt verplaatst.
       *[other] { $count } mappen hebben een eigen tekst, die naar een eerste document erin wordt verplaatst.
    }
import-scrivener-structure-text-with-children-line = { $count ->
        [one] 1 document met documenten erin wordt een map.
       *[other] { $count } documenten met documenten erin worden mappen.
    }
import-scrivener-structure-empty-folder-plain-line = { $count ->
        [one] 1 lege map blijft een gewone map, zodat geen enkele export een kop afdrukt die Scrivener nooit afdrukte.
       *[other] { $count } lege mappen blijven gewone mappen, zodat geen enkele export een kop afdrukt die Scrivener nooit afdrukte.
    }
import-scrivener-structure-book-inside-book-line = { $count ->
        [one] 1 boek binnen een boek wordt een deel.
       *[other] { $count } boeken binnen een boek worden delen.
    }
import-scrivener-structure-prologue-not-numbered-line = { $count ->
        [one] 1 proloog of epiloog blijft buiten de hoofdstuknummering.
       *[other] { $count } prologen of epilogen blijven buiten de hoofdstuknummering.
    }
import-scrivener-structure-matter-inside-book-line = { $count ->
        [one] 1 item voor- of nawerk van buiten het manuscript gaat als paratekst het boek in en blijft uit de exports weggelaten totdat u het opneemt.
       *[other] { $count } items voor- of nawerk van buiten het manuscript gaan als paratekst het boek in en blijven uit de exports weggelaten totdat u ze opneemt.
    }
import-scrivener-structure-paratext-not-counted-line = { $documents ->
        [one] 1 manuscriptdocument wordt paratekst (voor- of nawerk), dus { $words ->
            [one] { $words } woord telt
           *[other] { $words } woorden tellen
        } niet meer mee als manuscript.
       *[other] { $documents } manuscriptdocumenten worden paratekst (voor- of nawerk), dus { $words ->
            [one] { $words } woord telt
           *[other] { $words } woorden tellen
        } niet meer mee als manuscript.
    }
import-scrivener-structure-document-notes-line = { $count ->
        [one] De notities van 1 document worden een notitie ernaast.
       *[other] De notities van { $count } documenten worden notities ernaast.
    }
import-scrivener-structure-draft-content-as-note-line = De eigen tekst van de manuscriptmap wordt een notitie bovenaan het manuscript.
import-scrivener-structure-story-bible-folder-line = { $count ->
        [one] 1 map met personages of plaatsen wordt een tag in de verhaalbijbel.
       *[other] { $count } mappen met personages of plaatsen worden tags in de verhaalbijbel.
    }
import-scrivener-structure-templates-created-line = { $count ->
        [one] 1 documentsjabloon wordt een notitiesjabloon.
       *[other] { $count } documentsjablonen worden notitiesjablonen.
    }
import-scrivener-structure-template-nesting-flattened-line = { $count ->
        [one] 1 sjabloonmap binnen een andere wordt platgeslagen: elke tekst erin wordt een sjabloon.
       *[other] { $count } sjabloonmappen binnen andere worden platgeslagen: elke tekst erin wordt een sjabloon.
    }
import-scrivener-structure-template-kept-as-note-line = { $count ->
        [one] 1 sjabloon bevat iets wat een notitiesjabloon niet kan bevatten, en wordt als notitie bewaard.
       *[other] { $count } sjablonen bevatten iets wat een notitiesjabloon niet kan bevatten, en worden als notities bewaard.
    }
import-scrivener-structure-recovered-texts-line = { $count ->
        [one] 1 tekst die door geen enkel item in de ordner werd vermeld, wordt overgenomen in een eigen map.
       *[other] { $count } teksten die door geen enkel item in de ordner werden vermeld, worden overgenomen in een eigen map.
    }
import-scrivener-structure-project-notes-line = { $count ->
        [one] 1 projectnotitie wordt overgenomen in een eigen map.
       *[other] { $count } projectnotities worden overgenomen in een eigen map.
    }
import-scrivener-structure-trash-brought-across-line = { $items ->
        [1] De prullenbak wordt overgenomen: 1 item, dat u uit de prullenbak kunt terugzetten.
       *[other] De prullenbak wordt overgenomen: { $items } items, { $writing ->
            [0] waarvan geen met tekst
           *[other] waarvan { $writing } met tekst
        }, die u uit de prullenbak kunt terugzetten.
    }
import-scrivener-structure-not-compiled-line = { $count ->
        [one] 1 item dat Scrivener bij het compileren wegliet, wordt ook uit de exports weggelaten.
       *[other] { $count } items die Scrivener bij het compileren wegliet, worden ook uit de exports weggelaten.
    }
import-scrivener-structure-media-in-draft-line = { $count ->
        [one] 1 afbeelding of bestand in het manuscript wordt een notitie, omdat een boek het niet kan bevatten.
       *[other] { $count } afbeeldingen of bestanden in het manuscript worden notities, omdat een boek ze niet kan bevatten.
    }
import-scrivener-structure-left-out-line = { $count ->
        [one] 1 map op het hoogste niveau wordt weggelaten, met { $items ->
            [one] 1 item
           *[other] { $items } items
        }.
       *[other] { $count } mappen op het hoogste niveau worden weggelaten, met { $items ->
            [one] 1 item
           *[other] { $items } items
        }.
    }
import-scrivener-structure-trash-left-out-line = { $items ->
        [1] De prullenbak blijft achter: 1 item.
       *[other] De prullenbak blijft achter: { $items } items, { $writing ->
            [0] waarvan geen met tekst
           *[other] waarvan { $writing } met tekst
        }.
    }
import-scrivener-structure-card-image-left-behind-line = { $count ->
        [one] 1 afbeelding op de indexkaart van een manuscriptitem blijft achter: die zou in het boek worden afgedrukt.
       *[other] { $count } afbeeldingen op indexkaarten van manuscriptitems blijven achter: ze zouden in het boek worden afgedrukt.
    }
import-scrivener-structure-default-template-not-linked-line = { $count ->
        [one] 1 map had een standaardsjabloon voor de nieuwe documenten erin. In Skribisto kan alleen een tag voor de verhaalbijbel een sjabloon dragen, dus die koppeling wordt niet bewaard.
       *[other] { $count } mappen hadden een standaardsjabloon voor de nieuwe documenten erin. In Skribisto kan alleen een tag voor de verhaalbijbel een sjabloon dragen, dus die koppelingen worden niet bewaard.
    }
import-scrivener-structure-page-break-not-kept-line = { $count ->
        [one] 1 instelling “Page Break Before” (pagina-einde ervoor) heeft geen plek in Skribisto en wordt niet bewaard.
       *[other] { $count } instellingen “Page Break Before” (pagina-einde ervoor) hebben geen plek in Skribisto en worden niet bewaard.
    }
import-scrivener-structure-compile-as-is-not-kept-line = { $count ->
        [one] 1 instelling “Compile As-Is” (ongewijzigd compileren) heeft geen tegenhanger in Skribisto en wordt niet bewaard.
       *[other] { $count } instellingen “Compile As-Is” (ongewijzigd compileren) hebben geen tegenhanger in Skribisto en worden niet bewaard.
    }

## What the import did with the labels, the status list and the dates
import-scrivener-metadata-status-categories-guessed-line = { $count ->
        [one] 1 status die Skribisto niet herkent, krijgt een soort op basis van de plaats ervan in de lijst. Controleer die in Instellingen ▸ Project ▸ Statussen.
       *[other] { $count } statussen die Skribisto niet herkent, krijgen een soort op basis van hun plaats in de lijst. Controleer ze in Instellingen ▸ Project ▸ Statussen.
    }
import-scrivener-metadata-dates-unreadable-line = { $count ->
        [one] De datums van 1 item konden niet worden gelezen: het krijgt de datum van de import.
       *[other] De datums van { $count } items konden niet worden gelezen: ze krijgen de datum van de import.
    }
import-scrivener-metadata-labels-became-tags-line = { $tags ->
        [one] De labels worden 1 tag, toegekend aan { $rows ->
            [one] { $rows } item
           *[other] { $rows } items
        }.
       *[other] De labels worden { $tags } tags, toegekend aan { $rows ->
            [one] { $rows } item
           *[other] { $rows } items
        }.
    }
import-scrivener-metadata-statuses-became-ladder-line = { $rungs ->
        [one] De statuslijst wordt de lijst met statussen van het project: { $rungs } status, toegekend aan { $rows ->
            [one] { $rows } item
           *[other] { $rows } items
        }.
       *[other] De statuslijst wordt de lijst met statussen van het project: { $rungs } statussen, toegekend aan { $rows ->
            [one] { $rows } item
           *[other] { $rows } items
        }.
    }
import-scrivener-metadata-ladder-seeded-line = { $count ->
        [1] Het project had geen statuslijst, dus het begint met de standaardstatus van Skribisto.
       *[other] Het project had geen statuslijst, dus het begint met de { $count } standaardstatussen van Skribisto.
    }
import-scrivener-metadata-label-renamed-line = Het veld Label heette in Scrivener “{ $title }”. Die naam staat in de details van elke tag.
import-scrivener-metadata-status-renamed-line = Het veld Status heette in Scrivener “{ $title }”. Die naam staat in de details van elke status.
import-scrivener-metadata-none-label-collapsed-line = { $count ->
        [one] Het label “{ $name }”, dat onder dezelfde naam naast het eigen lege label van Scrivener stond, wordt bij 1 item gelezen als geen label.
       *[other] Het label “{ $name }”, dat onder dezelfde naam naast het eigen lege label van Scrivener stond, wordt bij { $count } items gelezen als geen label.
    }
import-scrivener-metadata-none-status-collapsed-line = { $count ->
        [one] De status “{ $name }”, die onder dezelfde naam naast de eigen lege status van Scrivener stond, wordt bij 1 item gelezen als geen status.
       *[other] De status “{ $name }”, die onder dezelfde naam naast de eigen lege status van Scrivener stond, wordt bij { $count } items gelezen als geen status.
    }
import-scrivener-metadata-unnamed-label-line = Een label zonder naam wordt genoemd naar zijn nummer, { $id }.
import-scrivener-metadata-unnamed-status-line = Een status zonder naam wordt genoemd naar zijn nummer, { $id }.
import-scrivener-metadata-tag-name-disambiguated-line = { $count ->
        [one] 1 tag krijgt een nieuwe naam, omdat een andere tag die naam al heeft.
       *[other] { $count } tags krijgen een nieuwe naam, omdat andere tags hun naam al hebben.
    }
import-scrivener-metadata-rung-names-disambiguated-line = { $count ->
        [one] 1 status heeft dezelfde naam als een andere, dus er komt een nummer achter de naam. Controleer die in Instellingen ▸ Project ▸ Statussen.
       *[other] { $count } statussen hebben dezelfde naam als een andere, dus elk krijgt een nummer achter de naam. Controleer ze in Instellingen ▸ Project ▸ Statussen.
    }
import-scrivener-metadata-unused-labels-skipped-line = { $count ->
        [one] 1 label dat geen enkel document draagt, wordt niet geïmporteerd.
       *[other] { $count } labels die geen enkel document draagt, worden niet geïmporteerd.
    }
import-scrivener-metadata-labels-without-row-line = { $count ->
        [one] Het label van 1 item zonder plek in het nieuwe project wordt niet overgenomen.
       *[other] De labels van { $count } items zonder plek in het nieuwe project worden niet overgenomen.
    }
import-scrivener-metadata-statuses-without-row-line = { $count ->
        [one] De status van 1 item zonder plek in het nieuwe project wordt niet overgenomen.
       *[other] De statussen van { $count } items zonder plek in het nieuwe project worden niet overgenomen.
    }
import-scrivener-metadata-unnamed-keyword-line = Een trefwoord zonder naam wordt genoemd naar zijn nummer, { $id }.
import-scrivener-metadata-keywords-without-row-line = { $count ->
        [one] De trefwoorden van 1 item zonder plek in het nieuwe project worden niet overgenomen.
       *[other] De trefwoorden van { $count } items zonder plek in het nieuwe project worden niet overgenomen.
    }
import-scrivener-metadata-custom-values-without-row-line = { $count ->
        [one] De aangepaste metagegevens van 1 item zonder plek in het nieuwe project worden niet overgenomen.
       *[other] De aangepaste metagegevens van { $count } items zonder plek in het nieuwe project worden niet overgenomen.
    }
import-scrivener-metadata-bookmarks-without-row-line = { $count ->
        [one] De bladwijzers van 1 item zonder plek in het nieuwe project worden niet overgenomen.
       *[other] De bladwijzers van { $count } items zonder plek in het nieuwe project worden niet overgenomen.
    }
import-scrivener-metadata-targets-without-row-line = { $count ->
        [one] Het doel van 1 item zonder plek in het nieuwe project wordt niet overgenomen.
       *[other] De doelen van { $count } items zonder plek in het nieuwe project worden niet overgenomen.
    }

## What the import does with the keywords, custom metadata, collections, bookmarks, targets,
## the deadline, the auto-complete list and the icons. Each line is said twice: on the Review
## page before the import, then in its report, so it reads the same at both times. `$note`
## and `$folder` are the titles the import gives a note and its "From Scrivener" folder;
## `$day` is a day as YYYY-MM-DD.
import-scrivener-metadata-pov-hint-line = { $count ->
        [1] Het label of de veldwaarde { $values } komt overeen met het item { $matches } in de verhaalbijbel. Als dit het vertelperspectief noemt, stel dat dan in de Inspector in: de import koppelt niets.
       *[other] De labels of veldwaarden { $values } komen overeen met de items { $matches } in de verhaalbijbel. Als ze vertelperspectieven noemen, stel die dan in de Inspector in: de import koppelt niets.
    }
import-scrivener-metadata-checkbox-default-unknown-line = Het selectievakjesveld “{ $field }” heeft een standaardwaarde die Skribisto niet kan lezen, dus het kan niet nagaan welke documenten standaard zijn aangevinkt. Alleen de documenten die afzonderlijk zijn aangevinkt, krijgen de tag ervan.
import-scrivener-metadata-goals-in-other-unit-line = { $count ->
        [one] 1 doel is ingesteld in de andere telleenheid. Het blijft bewaard en verschijnt zodra u de telleenheid wijzigt in Instellingen ▸ Project ▸ Opbouw.
       *[other] { $count } doelen zijn ingesteld in de andere telleenheid. Ze blijven bewaard en verschijnen zodra u de telleenheid wijzigt in Instellingen ▸ Project ▸ Opbouw.
    }
import-scrivener-metadata-counting-differs-line = Scrivener telde voor het manuscriptdoel andere documenten mee dan Skribisto doet. Het doel van het boek telt de scènes die in exports worden opgenomen.
import-scrivener-metadata-writing-days-not-read-line = De schrijfdagen “{ $raw }” kunnen niet worden gelezen, dus het Tempo van het boek telt elke dag als schrijfdag. Wijzig ze daar.
import-scrivener-metadata-script-mode-rows-line = { $count ->
        [one] 1 item stond in Scrivener in scriptmodus. Skribisto behoudt de woorden ervan als proza, zonder de opmaak als scenario.
       *[other] { $count } items stonden in Scrivener in scriptmodus. Skribisto behoudt hun woorden als proza, zonder de opmaak als scenario.
    }
import-scrivener-metadata-keywords-became-tags-line = { $tags ->
        [one] 1 trefwoord wordt een tag, toegekend aan { $rows ->
            [one] { $rows } item
           *[other] { $rows } items
        }.
       *[other] { $tags } trefwoorden worden tags, toegekend aan { $rows ->
            [one] { $rows } item
           *[other] { $rows } items
        }.
    }
import-scrivener-metadata-keyword-hierarchy-flattened-line = { $count ->
        [one] 1 trefwoord stond in Scrivener binnen een ander. Tags hebben geen hiërarchie: de details van de tag zeggen waar het trefwoord stond.
       *[other] { $count } trefwoorden stonden in Scrivener binnen andere. Tags hebben geen hiërarchie: de details van elke tag zeggen waar het trefwoord ervan stond.
    }
import-scrivener-metadata-fields-became-tags-line = { $fields ->
        [one] 1 lijst- of selectievakjesveld wordt { $tags ->
            [one] { $tags } tag
           *[other] { $tags } tags
        }, toegekend aan { $rows ->
            [one] { $rows } item
           *[other] { $rows } items
        }.
       *[other] { $fields } lijst- en selectievakjesvelden worden { $tags ->
            [one] { $tags } tag
           *[other] { $tags } tags
        }, toegekend aan { $rows ->
            [one] { $rows } item
           *[other] { $rows } items
        }.
    }
import-scrivener-metadata-collections-became-tags-line = { $tags ->
        [one] 1 verzameling wordt een tag, toegekend aan { $rows ->
            [one] { $rows } item
           *[other] { $rows } items
        }.
       *[other] { $tags } verzamelingen worden tags, toegekend aan { $rows ->
            [one] { $rows } item
           *[other] { $rows } items
        }.
    }
import-scrivener-metadata-collection-order-lost-line = { $count ->
        [one] Een tag heeft geen volgorde, dus de volgorde van de documenten in 1 verzameling blijft niet behouden.
       *[other] Een tag heeft geen volgorde, dus de volgorde van de documenten in { $count } verzamelingen blijft niet behouden.
    }
import-scrivener-metadata-bookmarks-became-links-line = { $count ->
        [one] 1 bladwijzer wordt een koppeling tussen twee notities, of tussen een scène en een item in de verhaalbijbel: zie Koppelingen en Rolbezetting.
       *[other] { $count } bladwijzers worden koppelingen tussen notities, of tussen scènes en items in de verhaalbijbel: zie Koppelingen en Rolbezetting.
    }
import-scrivener-metadata-goals-kept-line = { $count ->
        [one] Het doel dat Scrivener voor 1 document had ingesteld, wordt in Skribisto het doel ervan.
       *[other] De doelen die Scrivener voor { $count } documenten had ingesteld, worden in Skribisto hun doelen.
    }
import-scrivener-metadata-draft-target-kept-words-line = { $value ->
        [one] Het manuscriptdoel van 1 woord wordt het doel van het boek.
       *[other] Het manuscriptdoel van { $value } woorden wordt het doel van het boek.
    }
import-scrivener-metadata-draft-target-kept-characters-line = { $value ->
        [one] Het manuscriptdoel van 1 teken wordt het doel van het boek.
       *[other] Het manuscriptdoel van { $value } tekens wordt het doel van het boek.
    }
import-scrivener-metadata-pace-planned-line = De deadline, { $day }, stelt het Tempo van het boek in. Dat plant vanaf de dag van de import de woorden voor elke schrijfdag die u in Scrivener hebt gekozen.
import-scrivener-metadata-pace-planned-day-off-line = De deadline, { $day }, stelt het Tempo van het boek in. Dat plant vanaf de dag van de import de woorden voor elke schrijfdag die u in Scrivener hebt gekozen. Net als in Scrivener is de deadline zelf een vrije dag.
import-scrivener-metadata-dictionary-words-line = { $count ->
        [one] 1 woord voor automatisch aanvullen gaat naar het persoonlijke woordenboek van het project, zodat het niet als spelfout wordt gemarkeerd.
       *[other] { $count } woorden voor automatisch aanvullen gaan naar het persoonlijke woordenboek van het project, zodat ze niet als spelfout worden gemarkeerd.
    }
import-scrivener-metadata-listed-project-line = { $count ->
        [one] 1 gegeven over het project staat in de notitie “{ $note }” van de map “{ $folder }”.
       *[other] { $count } gegevens over het project staan in de notitie “{ $note }” van de map “{ $folder }”.
    }
import-scrivener-metadata-listed-project-bookmarks-line = { $count ->
        [one] 1 projectbladwijzer staat in de notitie “{ $note }” van de map “{ $folder }”.
       *[other] { $count } projectbladwijzers staan in de notitie “{ $note }” van de map “{ $folder }”.
    }
import-scrivener-metadata-listed-replacements-line = { $count ->
        [one] 1 vervanging bij het compileren staat in de notitie “{ $note }” van de map “{ $folder }”. Skribisto voert die niet uit.
       *[other] { $count } vervangingen bij het compileren staan in de notitie “{ $note }” van de map “{ $folder }”. Skribisto voert die niet uit.
    }
import-scrivener-metadata-listed-auto-complete-line = { $count ->
        [one] 1 term voor automatisch aanvullen die uit meerdere woorden bestaat, staat in de notitie “{ $note }” van de map “{ $folder }”.
       *[other] { $count } termen voor automatisch aanvullen die uit meerdere woorden bestaan, staan in de notitie “{ $note }” van de map “{ $folder }”.
    }
import-scrivener-metadata-listed-saved-searches-line = { $count ->
        [one] 1 opgeslagen zoekopdracht staat in de notitie “{ $note }” van de map “{ $folder }”.
       *[other] { $count } opgeslagen zoekopdrachten staan in de notitie “{ $note }” van de map “{ $folder }”.
    }
import-scrivener-metadata-listed-custom-metadata-line = { $count ->
        [one] 1 waarde van aangepaste metagegevens die een tag niet kan bevatten, staat in de notitie “{ $note }” van de map “{ $folder }”.
       *[other] { $count } waarden van aangepaste metagegevens die een tag niet kan bevatten, staan in de notitie “{ $note }” van de map “{ $folder }”.
    }
import-scrivener-metadata-listed-targets-line = { $count ->
        [one] 1 documentdoel staat in de notitie “{ $note }” van de map “{ $folder }”.
       *[other] { $count } documentdoelen staan in de notitie “{ $note }” van de map “{ $folder }”.
    }
import-scrivener-metadata-listed-bookmarks-line = { $count ->
        [one] 1 bladwijzer die geen enkele koppeling in Skribisto toont, staat in de notitie “{ $note }” van de map “{ $folder }”.
       *[other] { $count } bladwijzers die geen enkele koppeling in Skribisto toont, staan in de notitie “{ $note }” van de map “{ $folder }”.
    }
import-scrivener-metadata-targets-on-notes-line = { $count ->
        [one] 1 documentdoel staat op een notitie, waarvan de tekst door geen enkel doel wordt geteld, dus het wordt geen doel.
       *[other] { $count } documentdoelen staan op notities, waarvan de tekst door geen enkel doel wordt geteld, dus ze worden geen doelen.
    }
import-scrivener-metadata-targets-on-split-documents-line = { $count ->
        [one] 1 documentdoel staat op een document dat bij de koppen wordt gesplitst, en geen enkel doel telt die hoofdstukken samen, dus het wordt geen doel.
       *[other] { $count } documentdoelen staan op documenten die bij de koppen worden gesplitst, en geen enkel doel telt die hoofdstukken samen, dus ze worden geen doelen.
    }
import-scrivener-metadata-bookmark-ends-without-row-line = { $count ->
        [one] 1 bladwijzer leidt naar een document zonder plek in het nieuwe project, zoals een documentsjabloon. De bladwijzer wordt vermeld met de plek waar het document in Scrivener stond.
       *[other] { $count } bladwijzers leiden naar documenten zonder plek in het nieuwe project, zoals documentsjablonen. Ze worden vermeld met de plek waar de documenten in Scrivener stonden.
    }
import-scrivener-metadata-unused-keywords-skipped-line = { $count ->
        [one] 1 trefwoord dat geen enkel document draagt, wordt niet geïmporteerd.
       *[other] { $count } trefwoorden die geen enkel document draagt, worden niet geïmporteerd.
    }
import-scrivener-metadata-list-value-invalid-line = { $count ->
        [one] 1 waarde van het lijstveld “{ $field }” noemt geen van de opties ervan, en wordt niet geïmporteerd.
       *[other] { $count } waarden van het lijstveld “{ $field }” noemen geen van de opties ervan, en worden niet geïmporteerd.
    }
import-scrivener-metadata-undefined-custom-field-line = { $count ->
        [one] 1 waarde hoort bij een aangepast veld dat het project niet meer definieert en dat Scrivener ook niet toont. Ze wordt niet geïmporteerd.
       *[other] { $count } waarden horen bij aangepaste velden die het project niet meer definieert en die Scrivener ook niet toont. Ze worden niet geïmporteerd.
    }
import-scrivener-metadata-collection-members-missing-line = { $count ->
        [one] 1 document van de verzameling “{ $name }” heeft geen plek in het nieuwe project, of is uit het Scrivener-project verdwenen.
       *[other] { $count } documenten van de verzameling “{ $name }” hebben geen plek in het nieuwe project, of zijn uit het Scrivener-project verdwenen.
    }
import-scrivener-metadata-empty-collections-skipped-line = { $count ->
        [one] 1 verzameling zonder documenten wordt niet geïmporteerd.
       *[other] { $count } verzamelingen zonder documenten worden niet geïmporteerd.
    }
import-scrivener-metadata-bookmarks-unresolved-line = { $count ->
        [one] 1 bladwijzer leidt naar een document dat niet meer in het Scrivener-project staat, en wordt niet geïmporteerd.
       *[other] { $count } bladwijzers leiden naar documenten die niet meer in het Scrivener-project staan, en worden niet geïmporteerd.
    }
import-scrivener-metadata-targets-in-pages-line = { $count ->
        [one] 1 documentdoel telt pagina’s, die Skribisto niet telt, dus het wordt geen doel.
       *[other] { $count } documentdoelen tellen pagina’s, die Skribisto niet telt, dus ze worden geen doelen.
    }
import-scrivener-metadata-targets-unreadable-line = { $count ->
        [one] 1 doel wordt niet geïmporteerd: het aantal ervan is geen getal, is kleiner dan nul of is groter dan een doel kan bevatten.
       *[other] { $count } doelen worden niet geïmporteerd: hun aantallen zijn geen getallen, zijn kleiner dan nul of zijn groter dan een doel kan bevatten.
    }
import-scrivener-metadata-draft-target-in-pages-line = { $value ->
        [one] Het manuscriptdoel van 1 pagina wordt geen doel: Skribisto telt woorden of tekens. Het staat in de notitie “{ $note }” van de map “{ $folder }”.
       *[other] Het manuscriptdoel van { $value } pagina’s wordt geen doel: Skribisto telt woorden of tekens. Het staat in de notitie “{ $note }” van de map “{ $folder }”.
    }
import-scrivener-metadata-draft-target-no-book-words-line = { $value ->
        [one] Het manuscriptdoel van 1 woord wordt geen doel, omdat het manuscript meerdere boeken vormt, of geen enkel. Het staat in de notitie “{ $note }” van de map “{ $folder }”.
       *[other] Het manuscriptdoel van { $value } woorden wordt geen doel, omdat het manuscript meerdere boeken vormt, of geen enkel. Het staat in de notitie “{ $note }” van de map “{ $folder }”.
    }
import-scrivener-metadata-draft-target-no-book-characters-line = { $value ->
        [one] Het manuscriptdoel van 1 teken wordt geen doel, omdat het manuscript meerdere boeken vormt, of geen enkel. Het staat in de notitie “{ $note }” van de map “{ $folder }”.
       *[other] Het manuscriptdoel van { $value } tekens wordt geen doel, omdat het manuscript meerdere boeken vormt, of geen enkel. Het staat in de notitie “{ $note }” van de map “{ $folder }”.
    }
import-scrivener-metadata-deadline-passed-line = De deadline, { $day }, is verstreken, dus die stelt het Tempo van het boek niet in. Ze staat in de notitie “{ $note }” van de map “{ $folder }”.
import-scrivener-metadata-deadline-no-writing-day-line = Tussen de dag van de import en de deadline, { $day }, blijft geen enkele schrijfdag over, dus de deadline stelt het Tempo van het boek niet in. Ze staat in de notitie “{ $note }” van de map “{ $folder }”.
import-scrivener-metadata-deadline-template-line = De deadline, { $day }, is in het sjabloon opgeslagen, dus die stelt het Tempo van het boek niet in. Ze staat in de notitie “{ $note }” van de map “{ $folder }”.
import-scrivener-metadata-deadline-no-word-goal-line = De deadline, { $day }, stelt het Tempo van het boek niet in, omdat het manuscript geen woordendoel heeft om naartoe te plannen. Ze staat in de notitie “{ $note }” van de map “{ $folder }”.
import-scrivener-metadata-deadline-character-goal-line = De deadline, { $day }, stelt het Tempo van het boek niet in, omdat het manuscriptdoel tekens telt en het Tempo woorden telt. Ze staat in de notitie “{ $note }” van de map “{ $folder }”.
import-scrivener-metadata-deadline-no-book-line = De deadline, { $day }, stelt voor geen enkel boek het Tempo in, omdat het manuscript meerdere boeken vormt, of geen enkel. Ze staat in de notitie “{ $note }” van de map “{ $folder }”.
import-scrivener-metadata-deadline-unreadable-line = De deadline kan niet worden gelezen, dus die stelt het Tempo van het boek niet in. Ze staat zoals ze geschreven is in de notitie “{ $note }” van de map “{ $folder }”.
import-scrivener-metadata-session-target-words-line = { $value ->
        [one] Het sessiedoel van 1 woord wordt niet geïmporteerd: in Skribisto is het doel van de schrijfsessie één instelling voor alle projecten. Het staat in de notitie “{ $note }” van de map “{ $folder }”.
       *[other] Het sessiedoel van { $value } woorden wordt niet geïmporteerd: in Skribisto is het doel van de schrijfsessie één instelling voor alle projecten. Het staat in de notitie “{ $note }” van de map “{ $folder }”.
    }
import-scrivener-metadata-session-target-characters-line = { $value ->
        [one] Het sessiedoel van 1 teken wordt niet geïmporteerd: in Skribisto is het doel van de schrijfsessie één instelling voor alle projecten, in woorden. Het staat in de notitie “{ $note }” van de map “{ $folder }”.
       *[other] Het sessiedoel van { $value } tekens wordt niet geïmporteerd: in Skribisto is het doel van de schrijfsessie één instelling voor alle projecten, in woorden. Het staat in de notitie “{ $note }” van de map “{ $folder }”.
    }
import-scrivener-metadata-session-target-pages-line = { $value ->
        [one] Het sessiedoel van 1 pagina wordt niet geïmporteerd: in Skribisto is het doel van de schrijfsessie één instelling voor alle projecten, in woorden. Het staat in de notitie “{ $note }” van de map “{ $folder }”.
       *[other] Het sessiedoel van { $value } pagina’s wordt niet geïmporteerd: in Skribisto is het doel van de schrijfsessie één instelling voor alle projecten, in woorden. Het staat in de notitie “{ $note }” van de map “{ $folder }”.
    }
import-scrivener-metadata-session-target-calculated-words-line = { $value ->
        [one] Het sessiedoel van 1 woord, dat Scrivener berekende op basis van de deadline en de schrijfdagen, wordt niet geïmporteerd: in Skribisto is het doel van de schrijfsessie één instelling voor alle projecten. Het staat in de notitie “{ $note }” van de map “{ $folder }”.
       *[other] Het sessiedoel van { $value } woorden, dat Scrivener berekende op basis van de deadline en de schrijfdagen, wordt niet geïmporteerd: in Skribisto is het doel van de schrijfsessie één instelling voor alle projecten. Het staat in de notitie “{ $note }” van de map “{ $folder }”.
    }
import-scrivener-metadata-session-target-calculated-characters-line = { $value ->
        [one] Het sessiedoel van 1 teken, dat Scrivener berekende op basis van de deadline en de schrijfdagen, wordt niet geïmporteerd: in Skribisto is het doel van de schrijfsessie één instelling voor alle projecten, in woorden. Het staat in de notitie “{ $note }” van de map “{ $folder }”.
       *[other] Het sessiedoel van { $value } tekens, dat Scrivener berekende op basis van de deadline en de schrijfdagen, wordt niet geïmporteerd: in Skribisto is het doel van de schrijfsessie één instelling voor alle projecten, in woorden. Het staat in de notitie “{ $note }” van de map “{ $folder }”.
    }
import-scrivener-metadata-session-target-calculated-pages-line = { $value ->
        [one] Het sessiedoel van 1 pagina, dat Scrivener berekende op basis van de deadline en de schrijfdagen, wordt niet geïmporteerd: in Skribisto is het doel van de schrijfsessie één instelling voor alle projecten, in woorden. Het staat in de notitie “{ $note }” van de map “{ $folder }”.
       *[other] Het sessiedoel van { $value } pagina’s, dat Scrivener berekende op basis van de deadline en de schrijfdagen, wordt niet geïmporteerd: in Skribisto is het doel van de schrijfsessie één instelling voor alle projecten, in woorden. Het staat in de notitie “{ $note }” van de map “{ $folder }”.
    }
import-scrivener-metadata-icons-dropped-line = { $count ->
        [one] 1 item had een pictogram in de ordner, en dat wordt niet overgenomen: Skribisto toont bij elk item het pictogram van zijn type.
       *[other] { $count } items hadden een pictogram in de ordner, en die worden niet overgenomen: Skribisto toont bij elk item het pictogram van zijn type.
    }

## What converting the prose did, as the import's report says it. `$count` is how many texts,
## or how many of a thing, as each line says; `$example` quotes the project.
import-scrivener-prose-unreadable-line = { $count ->
        [one] 1 tekst kon niet worden gelezen. Die komt leeg binnen.
       *[other] { $count } teksten konden niet worden gelezen. Ze komen leeg binnen.
    }
import-scrivener-prose-comments-unreadable-line = { $count ->
        [one] De opmerkingen en gekoppelde voetnoten van 1 tekst konden niet worden gelezen. De woorden die ze markeerden, blijven behouden.
       *[other] De opmerkingen en gekoppelde voetnoten van { $count } teksten konden niet worden gelezen. De woorden die ze markeerden, blijven behouden.
    }
import-scrivener-prose-read-as-plain-text-line = { $count ->
        [one] 1 tekst was opgeslagen als platte tekst in plaats van als opgemaakte tekst, en wordt ook zo gelezen.
       *[other] { $count } teksten waren opgeslagen als platte tekst in plaats van als opgemaakte tekst, en worden ook zo gelezen.
    }
import-scrivener-prose-inline-note-unmatched-line = { $count ->
        [one] 1 markering van een inline notitie vormt geen paar met een andere en blijft als tekst staan.
       *[other] { $count } markeringen van inline notities vormen geen paar met een andere en blijven als tekst staan.
    }
import-scrivener-prose-style-unresolved-line = { $count ->
        [one] 1 stijlmarkering noemt een stijl die het project niet definieert. Ze wordt verwijderd, en de tekst behoudt zijn opmaak.
       *[other] { $count } stijlmarkeringen noemen een stijl die het project niet definieert. Ze worden verwijderd, en de tekst behoudt zijn opmaak.
    }
import-scrivener-prose-styles-lost-in-export-line = { $count ->
        [one] 1 alinea is misschien een kop die haar stijl verloor toen Scrivener 3 het project naar het oudere formaat exporteerde, en wordt overgenomen als gewone alinea. Controleer die, of importeer het project waaruit ze is geëxporteerd.
       *[other] { $count } alinea’s zijn misschien koppen die hun stijl verloren toen Scrivener 3 het project naar het oudere formaat exporteerde, en worden overgenomen als gewone alinea’s. Controleer ze, of importeer het project waaruit ze zijn geëxporteerd.
    }
import-scrivener-prose-comment-anchor-without-entry-line = { $count ->
        [one] 1 opmerking of voetnoot was in Scrivener haar inhoud kwijtgeraakt. De woorden die ze markeerde, blijven behouden, zonder de opmerking of voetnoot.
       *[other] { $count } opmerkingen of voetnoten waren in Scrivener hun inhoud kwijtgeraakt. De woorden die ze markeerden, blijven behouden, zonder de opmerkingen of voetnoten.
    }
import-scrivener-prose-comment-entry-without-anchor-line = { $count ->
        [one] 1 opmerking had geen plek meer in haar tekst en wordt aan de eerste alinea gekoppeld.
       *[other] { $count } opmerkingen hadden geen plek meer in hun tekst en worden aan de eerste alinea gekoppeld.
    }
import-scrivener-prose-footnote-entry-without-anchor-line = { $count ->
        [one] 1 gekoppelde voetnoot had geen plek meer in haar tekst, dus die wordt aan het einde van die tekst aangehaald.
       *[other] { $count } gekoppelde voetnoten hadden geen plek meer in hun teksten, dus elke voetnoot wordt aan het einde van haar eigen tekst aangehaald.
    }
import-scrivener-prose-range-comment-reading-line = { $count ->
        [one] 1 opmerking uit een oudere versie van Scrivener is geplaatst volgens een andere lezing van haar positie. Controleer of ze op de juiste woorden staat.
       *[other] { $count } opmerkingen uit een oudere versie van Scrivener zijn geplaatst volgens een andere lezing van hun positie. Controleer of ze op de juiste woorden staan.
    }
import-scrivener-prose-range-comment-reading-markers-counted-line = { $count ->
        [one] 1 opmerking uit een oudere versie van Scrivener is geplaatst door de markeringen van haar inline notities mee te tellen. Controleer of ze op de juiste woorden staat.
       *[other] { $count } opmerkingen uit een oudere versie van Scrivener zijn geplaatst door de markeringen van hun inline notities mee te tellen. Controleer of ze op de juiste woorden staan.
    }
import-scrivener-prose-range-comment-reading-pictures-not-counted-line = { $count ->
        [one] 1 opmerking uit een oudere versie van Scrivener is geplaatst door haar afbeeldingen niet mee te tellen. Controleer of ze op de juiste woorden staat.
       *[other] { $count } opmerkingen uit een oudere versie van Scrivener zijn geplaatst door hun afbeeldingen niet mee te tellen. Controleer of ze op de juiste woorden staan.
    }
import-scrivener-prose-range-comment-approximate-line = { $count ->
        [one] 1 opmerking uit een oudere versie van Scrivener past bij geen enkele lezing van haar positie precies, en staat misschien een paar tekens verschoven.
       *[other] { $count } opmerkingen uit een oudere versie van Scrivener passen bij geen enkele lezing van hun positie precies, en staan misschien een paar tekens verschoven.
    }
import-scrivener-prose-document-link-unresolved-line = { $count ->
        [one] 1 koppeling verwijst naar een document dat de import niet heeft overgenomen. De woorden ervan blijven staan.
       *[other] { $count } koppelingen verwijzen naar documenten die de import niet heeft overgenomen. Hun woorden blijven staan.
    }
import-scrivener-prose-linked-picture-not-read-line = { $count ->
        [1] 1 afbeelding die gekoppeld was aan een bestand op uw schijf, “{ $example }”, is niet overgenomen.
       *[other] { $count } afbeeldingen die gekoppeld waren aan bestanden op uw schijf, zoals “{ $example }”, zijn niet overgenomen.
    }
import-scrivener-prose-heading-repeats-title-line = { $count ->
        [one] 1 tekst begint met een kop die de titel ervan herhaalt, dus een export kan de titel twee keer afdrukken.
       *[other] { $count } teksten beginnen met een kop die hun titel herhaalt, dus een export kan de titel twee keer afdrukken.
    }
import-scrivener-prose-markdown-as-text-line = { $count ->
        [one] 1 alinea die in Markdown is geschreven, blijft staan zoals ze geschreven is.
       *[other] { $count } alinea’s die in Markdown zijn geschreven, blijven staan zoals ze geschreven zijn.
    }
import-scrivener-prose-citation-keys-as-text-line = { $count ->
        [one] 1 alinea bevat citeersleutels voor een bibliografieprogramma, die als tekst blijven staan.
       *[other] { $count } alinea’s bevatten citeersleutels voor een bibliografieprogramma, die als tekst blijven staan.
    }
import-scrivener-prose-placeholders-in-exported-text-line = { $count ->
        [1] 1 plaatshouder van Scrivener, { $example }, blijft als platte tekst staan in wat u exporteert. Vervang die met de hand.
       *[other] { $count } plaatshouders van Scrivener, zoals { $example }, blijven als platte tekst staan in wat u exporteert. Vervang ze met de hand.
    }
import-scrivener-prose-placeholders-in-other-text-line = { $count ->
        [1] 1 plaatshouder van Scrivener, { $example }, blijft als platte tekst staan in een tekst die niet wordt geëxporteerd.
       *[other] { $count } plaatshouders van Scrivener, zoals { $example }, blijven als platte tekst staan in teksten die niet worden geëxporteerd.
    }
import-scrivener-prose-ornament-lines-kept-line = { $count ->
        [one] 1 gecentreerde regel met siertekens markeert misschien een scèneovergang en blijft staan zoals hij geschreven is.
       *[other] { $count } gecentreerde regels met siertekens markeren misschien scèneovergangen en blijven staan zoals ze geschreven zijn.
    }
import-scrivener-prose-long-document-line = { $count ->
        [one] 1 tekst is erg lang en opent misschien traag.
       *[other] { $count } teksten zijn erg lang en openen misschien traag.
    }
import-scrivener-prose-pictures-without-description-line = { $count ->
        [one] 1 afbeelding heeft geen beschrijving voor lezers die haar niet kunnen zien. U kunt er een toevoegen in de editor.
       *[other] { $count } afbeeldingen hebben geen beschrijving voor lezers die ze niet kunnen zien. U kunt er een toevoegen in de editor.
    }
import-scrivener-prose-struck-text-compiled-away-line = { $count ->
        [one] Doorgehaalde tekst in 1 tekst blijft behouden. Scrivener liet die bij het compileren uit het boek weg, maar een export uit Skribisto drukt die af.
       *[other] Doorgehaalde tekst in { $count } teksten blijft behouden. Scrivener liet die bij het compileren uit het boek weg, maar een export uit Skribisto drukt die af.
    }
import-scrivener-prose-compile-deleted-style-printed-line = { $count ->
        [1] Tekst in de stijl “{ $example }”, die Scrivener bij het compileren verwijderde, blijft behouden als proza, en een export uit Skribisto drukt die af.
       *[other] Tekst in { $count } stijlen die Scrivener bij het compileren verwijderde, zoals “{ $example }”, blijft behouden als proza, en een export uit Skribisto drukt die af.
    }
import-scrivener-prose-compile-raw-style-printed-line = { $count ->
        [1] Tekst in de stijl “{ $example }”, die Scrivener bij het compileren doorgaf als code voor een ander programma, blijft behouden als proza, en een export uit Skribisto drukt die af.
       *[other] Tekst in { $count } stijlen die Scrivener bij het compileren doorgaf als code voor een ander programma, zoals “{ $example }”, blijft behouden als proza, en een export uit Skribisto drukt die af.
    }
import-scrivener-prose-left-out-after-document-line = { $count ->
        [one] 1 stuk van een beschadigd bestand, na het einde ervan, wordt weggelaten.
       *[other] { $count } stukken van beschadigde bestanden, na het einde ervan, worden weggelaten.
    }
import-scrivener-prose-left-out-header-or-footer-line = { $count ->
        [one] 1 koptekst of voettekst wordt weggelaten: die hoort niet bij de tekst.
       *[other] { $count } kop- en voetteksten worden weggelaten: ze horen niet bij de tekst.
    }
import-scrivener-prose-left-out-list-label-picture-line = { $count ->
        [one] 1 afbeelding die als opsommingsteken werd gebruikt, wordt weggelaten.
       *[other] { $count } afbeeldingen die als opsommingstekens werden gebruikt, worden weggelaten.
    }
import-scrivener-prose-left-out-line = { $count ->
        [one] 1 stuk bestandsinhoud dat bij geen enkele tekst hoort, wordt weggelaten.
       *[other] { $count } stukken bestandsinhoud die bij geen enkele tekst horen, worden weggelaten.
    }
import-scrivener-prose-undecodable-characters-line = { $count ->
        [one] 1 teken kon niet worden gedecodeerd en wordt getoond als vervangingsteken (�) op de plek waar het stond.
       *[other] { $count } tekens konden niet worden gedecodeerd en worden getoond als vervangingstekens (�) op de plek waar ze stonden.
    }
import-scrivener-prose-inline-notes-merged-line = { $count ->
        [one] 1 inline notitie die Scrivener had gesplitst waar de opmaak veranderde, wordt weer samengevoegd.
       *[other] { $count } inline notities die Scrivener had gesplitst waar hun opmaak veranderde, worden weer samengevoegd.
    }
import-scrivener-prose-inline-notes-joined-across-paragraphs-line = { $count ->
        [1] 1 inline notitie die alinea voor alinea was geschreven, wordt samengevoegd tot één notitie.
       *[other] { $count } inline notities die alinea voor alinea waren geschreven, worden samengevoegd tot notities van meerdere alinea’s.
    }
import-scrivener-prose-blank-notes-dropped-line = { $count ->
        [one] 1 lege inline notitie wordt weggelaten.
       *[other] { $count } lege inline notities worden weggelaten.
    }
import-scrivener-prose-annotations-became-comments-line = { $count ->
        [one] 1 inline annotatie is een opmerking geworden.
       *[other] { $count } inline annotaties zijn opmerkingen geworden.
    }
import-scrivener-prose-annotation-only-document-kept-line = { $count ->
        [one] 1 document bevatte alleen een annotatie. De woorden ervan blijven behouden als proza.
       *[other] { $count } documenten bevatten alleen een annotatie. Hun woorden blijven behouden als proza.
    }
import-scrivener-prose-preserve-formatting-passages-line = { $count ->
        [one] 1 passage gemarkeerd met “Preserve Formatting” (opmaak behouden) komt binnen als gewone tekst.
       *[other] { $count } passages gemarkeerd met “Preserve Formatting” (opmaak behouden) komen binnen als gewone tekst.
    }
import-scrivener-prose-nested-note-unwrapped-line = { $count ->
        [one] 1 notitie binnen een opmerking of voetnoot blijft als tekst staan, op de plek waar ze stond.
       *[other] { $count } notities binnen opmerkingen of voetnoten blijven als tekst staan, op de plek waar ze stonden.
    }
import-scrivener-prose-style-not-carried-line = { $count ->
        [1] De stijl “{ $example }” heeft geen tegenhanger in Skribisto. De tekst ervan behoudt zijn woorden.
       *[other] { $count } stijlen, zoals “{ $example }”, hebben geen tegenhanger in Skribisto. Hun tekst behoudt zijn woorden.
    }
import-scrivener-prose-document-links-dropped-line = { $count ->
        [one] 1 koppeling naar een ander document heeft geen plek in Skribisto. De woorden ervan blijven staan.
       *[other] { $count } koppelingen naar andere documenten hebben geen plek in Skribisto. Hun woorden blijven staan.
    }
import-scrivener-prose-document-links-kept-line = { $count ->
        [one] 1 koppeling naar een ander document blijft bewaard als verwijzing.
       *[other] { $count } koppelingen naar andere documenten blijven bewaard als verwijzingen.
    }
import-scrivener-prose-other-links-kept-as-text-line = { $count ->
        [one] 1 koppeling naar een ander project of naar een bestand blijft als tekst staan.
       *[other] { $count } koppelingen naar andere projecten of naar bestanden blijven als tekst staan.
    }
import-scrivener-prose-soft-breaks-split-line = { $count ->
        [one] 1 regeleinde binnen een alinea begint nu een eigen alinea.
       *[other] { $count } regeleinden binnen alinea’s beginnen nu elk een eigen alinea.
    }
import-scrivener-prose-typed-indents-removed-line = { $count ->
        [one] 1 inspringing die aan het begin van een alinea was getypt, wordt verwijderd.
       *[other] { $count } inspringingen die aan het begin van alinea’s waren getypt, worden verwijderd.
    }
import-scrivener-prose-left-indents-not-kept-line = { $count ->
        [one] 1 alinea die vanaf de marge inspringt, verliest haar inspringing.
       *[other] { $count } alinea’s die vanaf de marge inspringen, verliezen hun inspringing.
    }
import-scrivener-prose-blank-lines-removed-line = { $count ->
        [one] 1 lege alinea tussen twee andere wordt verwijderd.
       *[other] { $count } lege alinea’s tussen andere worden verwijderd.
    }
import-scrivener-prose-verse-stanza-gaps-lost-line = { $count ->
        [one] 1 lege regel tussen de strofen van een gedicht wordt verwijderd, zodat de strofen doorlopen.
       *[other] { $count } lege regels tussen de strofen van een gedicht worden verwijderd, zodat de strofen doorlopen.
    }
import-scrivener-prose-blank-lines-became-scene-breaks-line = { $count ->
        [one] 1 lege regel tussen alinea’s is een scèneovergang geworden.
       *[other] { $count } lege regels tussen alinea’s zijn scèneovergangen geworden.
    }
import-scrivener-prose-ornaments-became-scene-breaks-line = { $count ->
        [one] 1 regel met siertekens is een scèneovergang geworden.
       *[other] { $count } regels met siertekens zijn scèneovergangen geworden.
    }
import-scrivener-prose-table-first-row-header-line = { $count ->
        [one] 1 tabel begint nu met een kopregel.
       *[other] { $count } tabellen beginnen nu met een kopregel.
    }
import-scrivener-prose-table-cells-joined-line = { $count ->
        [one] 1 tabelcel bevatte meerdere alinea’s, die nu tot één zijn samengevoegd.
       *[other] { $count } tabelcellen bevatten meerdere alinea’s, die nu tot één zijn samengevoegd.
    }
import-scrivener-prose-formatting-lost-colour-line = { $count ->
        [one] Tekstkleur wordt verwijderd in 1 tekst.
       *[other] Tekstkleur wordt verwijderd in { $count } teksten.
    }
import-scrivener-prose-formatting-lost-highlight-line = { $count ->
        [one] Markeringen met de markeerstift worden verwijderd in 1 tekst.
       *[other] Markeringen met de markeerstift worden verwijderd in { $count } teksten.
    }
import-scrivener-prose-formatting-lost-small-caps-line = { $count ->
        [one] Kleinkapitalen worden in 1 tekst als gewone letters weergegeven.
       *[other] Kleinkapitalen worden in { $count } teksten als gewone letters weergegeven.
    }
import-scrivener-prose-formatting-lost-all-caps-line = { $count ->
        [one] Tekst die in hoofdletters werd weergegeven, verschijnt in 1 tekst zoals hij werd getypt.
       *[other] Tekst die in hoofdletters werd weergegeven, verschijnt in { $count } teksten zoals hij werd getypt.
    }
import-scrivener-prose-formatting-lost-hidden-line = { $count ->
        [one] Verborgen tekst wordt in 1 tekst weggelaten.
       *[other] Verborgen tekst wordt in { $count } teksten weggelaten.
    }
import-scrivener-prose-formatting-lost-line = { $count ->
        [one] Opmaak waarvoor Skribisto geen plek heeft, wordt in 1 tekst verwijderd.
       *[other] Opmaak waarvoor Skribisto geen plek heeft, wordt in { $count } teksten verwijderd.
    }
import-scrivener-prose-justified-text-not-kept-line = { $count ->
        [one] 1 uitgevulde alinea volgt in plaats daarvan de exportstijl.
       *[other] { $count } uitgevulde alinea’s volgen in plaats daarvan de exportstijl.
    }
import-scrivener-prose-comment-colours-lost-line = { $count ->
        [one] 1 opmerking verliest haar kleur.
       *[other] { $count } opmerkingen verliezen hun kleur.
    }
import-scrivener-prose-control-characters-removed-line = { $count ->
        [one] 1 onzichtbaar stuurteken wordt verwijderd.
       *[other] { $count } onzichtbare stuurtekens worden verwijderd.
    }
import-scrivener-prose-note-on-heading-lost-line = { $count ->
        [one] 1 voetnoot of opmerking bij een hoofdstukkop kon daar niet blijven: een opmerking gaat naar de eerste alinea van het hoofdstuk, een voetnoot wordt weggelaten.
       *[other] { $count } voetnoten of opmerkingen bij hoofdstukkoppen konden daar niet blijven: een opmerking gaat naar de eerste alinea van het hoofdstuk, een voetnoot wordt weggelaten.
    }
import-scrivener-prose-media-file-copied-line = { $count ->
        [one] 1 onderzoeksbestand dat Skribisto niet kan openen, is naast het nieuwe project gekopieerd. De notitie ervan zegt waar.
       *[other] { $count } onderzoeksbestanden die Skribisto niet kan openen, zijn naast het nieuwe project gekopieerd. Hun notities zeggen waar.
    }
import-scrivener-prose-media-file-not-carried-line = { $count ->
        [one] 1 onderzoeksbestand dat Skribisto niet kan openen, blijft in uw Scrivener-project. De notitie ervan noemt het.
       *[other] { $count } onderzoeksbestanden die Skribisto niet kan openen, blijven in uw Scrivener-project. Hun notities noemen ze.
    }

## When the import fails
## Shown in the notice of a failed import, above the button import-scrivener-review-again,
## which these sentences quote.
import-scrivener-error-refused = { $refusal } Er is niets geschreven.
import-scrivener-error-question = Het project stelt nu een vraag die bij het controleren niet is beantwoord. Kies “Import hercontroleren” en importeer daarna. Er is niets geschreven.
import-scrivener-error-not-accepted = Voordat het project kan worden geïmporteerd, moet u in de stap Project een vakje aanvinken, en dat is niet gebeurd. Kies “Import hercontroleren”, vink het vakje aan en importeer daarna. Er is niets geschreven.
import-scrivener-error-changed = Het project is gewijzigd nadat u het hebt gecontroleerd. Kies “Import hercontroleren” en importeer daarna. Er is niets geschreven.
import-scrivener-error-choice = De structuur die bij het controleren is gekozen, past niet meer bij dit project. Kies “Import hercontroleren” en importeer daarna. Er is niets geschreven.

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
settings-punctuation-auto-lists = Een lijst beginnen met een alinea die begint met "- ", "* " of "1. "
settings-punctuation-auto-lists-hint = De alinea wordt een opsomming of een genummerde lijst. Waar het dialoogstreepje aan staat en de taal er een heeft, opent "- " in plaats daarvan een dialoogregel. Druk direct daarna op Backspace om terug te krijgen wat u typte.

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

cover-heading = Omslag
cover-pick = Kiezen…
cover-change = Wijzigen…
cover-remove = Verwijderen
cover-none = Nog geen omslag
cover-missing = Het afbeeldingsbestand ontbreekt in het project
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
import-document-pictures = { $count ->
        [1] Er hoort één afbeelding bij deze documenten.
       *[other] Er horen { $count } afbeeldingen bij deze documenten.
    }
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
import-diagnostic-image-not-ingested-format = In “{ $path }” staat de afbeelding “{ $detail }” in een formaat dat Skribisto niet kan tonen, dus die is weggelaten. Afbeeldingen in PNG, JPEG, WebP, GIF, TIFF of BMP komen wel binnen: sla de afbeelding in een van die formaten op en voeg haar met de hand in.
import-diagnostic-image-not-ingested-remote = In “{ $path }” staat de afbeelding “{ $detail }” op het web. Tijdens een import worden nooit afbeeldingen gedownload, dus die is weggelaten.
import-diagnostic-image-not-ingested-outside = In “{ $path }” staat de afbeelding “{ $detail }” buiten het document en de map ervan, dus die is weggelaten. Alleen afbeeldingen die in het document zelf, of in de eigen map van het document, zijn opgeslagen, komen binnen.
import-diagnostic-image-not-ingested-missing = In “{ $path }” kon de afbeelding “{ $detail }” niet worden gevonden, dus die is weggelaten.
import-diagnostic-image-not-ingested-too-large = In “{ $path }” is de afbeelding “{ $detail }” groter dan een project accepteert, dus die is weggelaten.
import-diagnostic-image-not-ingested-unreadable = In “{ $path }” kon de afbeelding “{ $detail }” niet als afbeelding worden gelezen, dus die is weggelaten.
import-diagnostic-image-not-ingested-heading = In “{ $path }” staat de afbeelding “{ $detail }” in een kop. Elke kop wordt de titel van een item in de ordner, en een titel bevat geen afbeelding, dus de afbeelding is weggelaten.
import-diagnostic-image-not-ingested-note = In “{ $path }” staat de afbeelding “{ $detail }” in een voetnoot, een eindnoot of een opmerking. Daarin blijven alleen de woorden bewaard, geen afbeeldingen, dus de afbeelding is weggelaten.
import-diagnostic-image-not-ingested-html = In “{ $path }” is de afbeelding “{ $detail }” als HTML geschreven, en een import laat HTML weg, dus de afbeelding is mee weggelaten. Schrijf haar als Markdown-afbeelding om haar binnen te halen.
import-diagnostic-image-not-ingested-document-full = In “{ $path }” is de afbeelding “{ $detail }” weggelaten: de afbeeldingen van dit document gaan samen verder dan wat één import binnenhaalt. Importeer het document in delen, of voeg de rest met de hand in.
import-diagnostic-picture-first-frame-only = In “{ $path }” komt de bewegende afbeelding “{ $detail }” binnen als haar eerste beeld. Afbeeldingen in een project bewegen niet.
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
help-topic-import-scrivener = Overstappen van Scrivener
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
shortcut-name-work-import-scrivener = Scrivener-project importeren…
shortcut-name-work-import-scrivener-report = Scrivener-importrapport tonen
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
dialog-plume-file-title = Kies een Plume Creator-project
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

shortcut-group-work = Project
shortcut-group-edit = Bewerken
shortcut-group-view = Beeld
shortcut-group-document = Document
shortcut-group-format = Opmaak
shortcut-group-go = Ga
shortcut-group-tools = Extra
shortcut-group-help = Help

## Manuskript import: the headings of the notes it writes (a character's
## sheet, a world entry, a plot and its steps, the project details, the
## snowflake summary ladder) and the title of a project with no name. Written
## into the imported project once, in the interface language; the writer can
## rename them like any other text. Goal and Conflict reuse the note templates'
## own words.
import-manuskript-section-paragraph-summary = Samenvatting in één alinea
import-manuskript-section-full-summary = Volledige samenvatting
import-manuskript-section-motivation = Motivatie
import-manuskript-section-epiphany = Inzicht
import-manuskript-section-notes = Notities
import-manuskript-section-description = Beschrijving
import-manuskript-section-passion = Passie
import-manuskript-section-result = Resultaat
import-manuskript-section-summary = Samenvatting
import-manuskript-section-meta = Metadata
import-manuskript-section-series = Reeks
import-manuskript-section-volume = Deel
import-manuskript-section-genre = Genre
import-manuskript-section-license = Licentie
import-manuskript-section-email = E-mail
# The snowflake ladder's five tellings, shortest first: one situation, one
# sentence, one paragraph, one page, the full summary.
import-manuskript-section-situation = Situatie
import-manuskript-section-sentence = Eén zin
import-manuskript-section-paragraph = Eén alinea
import-manuskript-section-page = Eén pagina
import-manuskript-section-full = Volledige samenvatting
import-manuskript-untitled-project = Geïmporteerd Manuskript-project

## Plume import: the story-bible groups an old (0.3 and earlier) project
## implies without naming them, and the title of a project with no name.
## Written into the imported project once, in the interface language.
import-plume-group-characters = Personages
import-plume-group-items = Voorwerpen
import-plume-group-places = Plaatsen
import-plume-untitled-project = Geïmporteerd Plume-project
