# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

menu-work = Project (&W)
menu-new-work = Nieuw project (&N)
menu-open-work = Project openen… (&O)
menu-new-window = Nieuw venster (&W)
menu-import-from = Importeren uit (&I)
menu-create-from = Aanmaken van (&C)
menu-import-plume = Plume Creator (.plume)… (&P)
menu-import-document = Documenten (Markdown, Word, ODT)... (&D)
menu-import-manuskript = Manuskript (.msk)… (&M)
menu-export = Exporteren (&X)
menu-export-book = Boek exporteren
menu-export-part = Deel exporteren
menu-export-chapter = Hoofdstuk exporteren
menu-export-scene = Scène exporteren
menu-export-note = Notitie exporteren
menu-export-paratext = Paratext exporteren
menu-export-folder = Map exporteren
menu-export-choose = Kies...
menu-export-none = Een document openen om te exporteren
menu-save = Opslaan (&S)
menu-save-as-file = Opslaan als één bestand… (&F)
menu-save-as-folder = Opslaan als map… (&D)
menu-backup = Nu een reservekopie maken (&B)
menu-close-work = Project sluiten (&C)
menu-welcome = Welkom. (&L)
menu-settings = Instellingen (&E)
menu-quit = Afsluiten (&Q)

menu-view = Beeld (&V)
menu-outline = Overzicht (&O)
menu-search = Zoeken in project (&S)
menu-timeline = Ga terug in de tijd (&G)
menu-search-preview = Zoekvoorbeeld (&P)
menu-fullscreen = Volledig scherm (&F)
menu-focus-mode = Afleidingsvrije modus (&D)
menu-format = Opmaak (&O)
menu-scene-break = Scene break invoegen (&K)
menu-major-scene-break = Grote scèneafbreking invoegen (&M)

menu-go = Ga naar (&G)
menu-go-next-scene = Volgende scène (&S)
menu-go-prev-scene = Vorige scène (&V)
menu-go-next-chapter = Volgende hoofdstuk (&C)
menu-go-prev-chapter = Vorige hoofdstuk (&H)
menu-go-next-note = Volgende notitie (&N)
menu-go-prev-note = Vorige notitie (&T)

menu-tools = Extra (&T)
menu-help = Help (&H)

ctx-add = Toevoegen (&A)
ctx-new-item = Nieuw item (&N)
ctx-new-folder = Nieuwe map (&F)
ctx-rename = Naam wijzigen (&R)
ctx-duplicate = Dupliceren (&D)
ctx-import-here = Hier importeren... (&I)
ctx-indent = Inspringen (&I)
ctx-outdent = Uitspringen (&O)
ctx-trash = Naar prullenbak verplaatsen (&T)
ctx-open-to-side = Open naar de zijkant (&S)
ctx-open = Openen (&O)
ctx-reveal-in-outline = Onthullen in overzicht (&L)
ctx-move-up = Omhoog verplaatsen (&U)
ctx-move-down = Omlaag verplaatsen (&N)

ctx-tab-close = Sluiten (&C)
ctx-tab-close-others = Andere tabbladen sluiten (&O)
ctx-tab-close-all = Alle tabbladen sluiten (&L)
ctx-tab-open-to-side = Open naar de zijkant (&S)
ctx-tab-open-in-main = Openen in het hoofdpaneel (&I)
ctx-tab-move-to-side = Ga naar de zijkant (&M)
ctx-tab-move-to-main = Verplaats naar het hoofdpaneel (&M)
ctx-tab-move-to-new-window = Verplaatsen naar een nieuw venster (&N)
ctx-tab-move-window-unsaved = Dit project eerst opslaan: een tweede venster opent zich op een bestand op schijf
ctx-tab-pin = Pin dit tabblad (&P)
ctx-tab-unpin = Dit tabblad losmaken (&U)
tab-pinned-tooltip = Gepind: "Sluit anderen" en "Sluit alles" laat het open

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
create-story-bible-entry = Story Bijbel ingang...
type-book-start = Boekstart
type-text = Tekst

new-item-book = Nieuw boek
new-item-part = Nieuw deel
new-item-chapter = Nieuw hoofdstuk
new-item-note = Nieuwe notitie
new-item-note-folder = Nieuwe notitiemap
new-item-folder = Nieuwe map
new-item-paratext = Nieuwe paratekst
new-item-paratext-folder = Nieuwe paratekstmap
new-item-story-bible-entry = Nieuw verhaal Bijbelinvoer

placement-inside = binnen
placement-after = na
placement-after-parent = na ouder
placement-top-level = topniveau


ctx-promote = Omzetten naar (&C)
promote-chapter-folder = Hoofdstukmap
promote-flat-chapter = Zelfstandig hoofdstuk
promote-lossy-title = Nergens om de tekst te bewaren
promote-lossy-text = Een { $target } kan deze nergens bewaren: { $kinds }. Beweeg of wis die tekst eerst, dan bekeer je.
content-scene-text = Scènetekst
content-note-text = Notitietekst
content-book-title = Boektitel
content-book-subtitle = Ondertiteling boek
content-part-title = Titel deel
content-chapter-title = Titel hoofdstuk
content-epigraph-text = Motto
content-paratext-text = Paratekst
promote-blocked-title = Hoofdstuk niet leeg
promote-blocked-text = { $count ->
    [one] Dit hoofdstuk bevat nog steeds { $count } item. Verplaats of prullenbak voordat het wordt omgezet in een plat hoofdstuk.
   *[other] Dit hoofdstuk bevat nog steeds { $count } artikelen. Verplaats ze of gooi ze weg voordat ze worden omgezet in een plat hoofdstuk.
}

inspector = Eigenschappen
inspector-empty = Open een item om het te inspecteren.
inspector-promote = Omzetten naar...
statusbar-toggle-outline = Bindmiddel aan/uit
statusbar-toggle-inspector = Schakel de inspecteur in
statusbar-save-unsaved = Niet opgeslagen wijzigingen. Klik om op te slaan
statusbar-save-saved = Alle wijzigingen opgeslagen
statusbar-save-autosave = Automatisch opslaan is ingeschakeld. Wijzigingen worden opgeslagen als u schrijft
statusbar-saving = Opslaan...
statusbar-word-count = { $count ->
    [one] { $count } woord
   *[other] { $count } woorden
}
statusbar-word-count-tooltip = Woorden in de scène die je bewerkt
statusbar-word-char-count = { $words ->
    [one] { $words } woord
   *[other] { $words } woorden
} · { $chars ->
    [one] { $chars } teken
   *[other] { $chars } tekens
}
session-toggle = Schrijven sessie: start of pauzeer een gerichte sprint
session-configure = Het woord doel en de tijdslimiet van de sessie instellen
session-configure-title = sessie schrijven
session-word-goal = Woordendoel
session-time-limit = Termijn
session-time-limit-unit = min
session-no-goal = Geen doel
session-no-limit = Geen limiet
session-reset = Sessie resetten
session-readout = { $words } woorden · { $time }
session-readout-timed = { $words } woorden · { $time } links
statusbar-focus-exit = Afleidingsvrije modus verlaten
statusbar-focus-synopsis = Synopsis
statusbar-focus-go-prev = Vorige item (Alt+Up)
statusbar-focus-go-next = Volgende item (Alt+Down)

rail-settings = Instellingen

language = Taal
theme = Thema
english = Engels
french = Frans
light = Licht
dark = Donker
settings-text-width = Tekstbreedte
settings-preview-width = Zoekvoorbeeldbreedte
settings-autosave = Automatisch opslaan naar schijf
settings-show-welcome = Lancering tonen bij opstarten
settings-show-welcome-tip = Het laatste project wordt heropend.
settings-check-for-updates = Controleren op nieuwe versies
settings-check-for-updates-tip = Eenmaal per dag vraagt Skribisto aan de website van het project welke versie actueel is. Het verzoek bevat geen identificatiecode en zelfs niet de versie die u uitvoert: de vergelijking gebeurt op deze computer.

settings-title = Instellingen
settings-close = Sluiten
settings-search = Zoekinstellingen
settings-reset = Terugzetten naar standaardinstellingen
settings-done = Gereed
settings-cancel = Annuleren
settings-apply = Toepassen
settings-ok = OK
settings-reset-confirm-title = Alle instellingen terugzetten naar standaardinstellingen?
settings-reset-confirm-body = Herstelt de uiterlijke, editor- en schrijfinstellingen van deze toepassing. De eigen instellingen van uw project, sneltoetsen en opgeslagen stijlen worden niet beïnvloed. Het kan niet ongedaan worden gemaakt.
settings-empty-title = Nog geen instellingen hier
settings-empty-hint = Deze sectie krijgt opties in een toekomstige update.

settings-sec-appearance-behaviour = Uiterlijk gedrag
settings-sec-editor = Editor
settings-sec-spelling = Spelling
settings-sec-backup = Reservekopie synchroniseren
settings-sec-compile = Uitvoer compileren
settings-page-appearance = Uiterlijk
settings-field-image-size-policy = Grote afbeeldingen
settings-image-policy-ask = Elke keer vragen
settings-image-policy-keep = Origineel bewaren
settings-image-policy-downscale = Optimaliseren
settings-hint-image-size-policy = Wat te doen als een ingevoegd beeld groter is dan het manuscript nodig heeft.
settings-page-notifications = Kennisgevingen
settings-page-scene = Scène
settings-page-synopsis = Synopsis
settings-page-notes = Notities
settings-page-editor-behavior = Editorgedrag
settings-page-goals = Doelen Word Count
settings-page-games = Spelletjes schrijven
settings-page-corkboard = Prikbord
settings-page-distraction-free = Afleidingsvrij
settings-page-dictionaries = Woordenboeken
settings-page-autosave = Automatisch opslaan
settings-page-export = Formaten exporteren
settings-page-paratext = Paratekststructuren
settings-page-keymap = Sleutelkaart
settings-keymap-filter = Sneltoetsen filteren

settings-desc-sec-appearance-behaviour = Hoe de toepassing zelf eruit ziet, en wat het doet als het begint.
settings-desc-sec-editor = Het schrijfvlak: hoe het eruit ziet, en wat het doet als je typt.
settings-desc-sec-spelling = Spell-checking, en de woordenboeken erachter.
settings-desc-sec-backup = Hoe uw Project de schijf bereikt en welke kopieën bewaard worden.
settings-desc-sec-compile = Wat overblijft Skribisto, en in welke vorm.
settings-desc-sec-work = Instellingen die behoren tot dit project en reizen in zijn bestand.
settings-desc-sec-extensions = Pagina's bijgedragen door de extensies hier geïnstalleerd.
settings-desc-group-typography = Een pagina per soort tekst: lettertype, grootte, lijnhoogte en afstand.
settings-desc-appearance = Interface taal, thema, tekstgrootte, en de launcher bij het opstarten.
settings-desc-notifications = Elke boodschap die deze sessie heeft getoond, en de acties die je kunt herhalen.
settings-desc-scene = Hoe scène proza is ingesteld: lettertype, grootte, lijnhoogte, streepjes en afstand.
settings-desc-synopsis = Hoe het synopsispaneel wordt ingesteld, onafhankelijk van het manuscript.
settings-desc-notes = Hoe noten worden gezet, onafhankelijk van het manuscript.
settings-desc-corkboard = Kaartgrootte, wat een kaart laat zien, en hoe het bord ze regelt.
settings-desc-distraction-free = Typografie, kolombreedte en regelstrip voor volledig schermschrift.
settings-desc-distraction-free-themes = De themabibliotheek voor full-screen schrijven: de verzonden en je eigen.
settings-desc-editor-behavior = Tekstbreedte, synopsispositie, schrijfmachine scrollen en het caret highlight.
settings-desc-punctuation = De typografische huisstijl nieuwe projecten beginnen van. Elk project volgt dit tenzij het zijn eigen.
settings-desc-goals = Woord en karakter doelen, en hoe woorden worden geteld.
settings-desc-games = Restricties die je jezelf stelt tijdens het opstellen, zoals Altijd vooruit.
settings-desc-spellcheck = De enkele schakelaar die overal spellingscontrole aan en uit zet.
settings-desc-dictionaries = Installeer, verwijder en blader door de spelling woordenboeken op deze machine.
settings-desc-autosave = Of uw bewerkingen alleen naar de schijf zijn geschreven.
settings-desc-backup = Wanneer kopieën worden genomen, waar ze worden bewaard, en hoeveel. Elk project volgt dit tenzij het zijn eigen.
settings-desc-export = De stijlen die elke export compileert door: de verzonden en je eigen.
settings-desc-paratext = De voor- en achterkant van een nieuw project kan beginnen.
settings-desc-keymap = Elke kortere weg, en waaraan het gebonden is.
settings-page-user = Gebruiker
settings-desc-user = Wie je bent, voor de commentaren die je schrijft.
settings-group-identity = Identiteit
settings-field-user-name = Uw naam
settings-field-user-name-placeholder = Facultatief
settings-field-user-name-hint = Teken de commentaren en antwoorden die u schrijft. Onderscheiden van de auteur naam van een project, dat is de boek byline en reist in het bestand.
settings-field-user-initials = Uw initialen
settings-field-user-initials-placeholder = Uit uw naam
settings-field-user-initials-hint = Wat een tekstverwerker toont naast uw commentaar in de marge. Laat het leeg om de getoonde te gebruiken, genomen van uw naam.
settings-field-user-hint = Beide zijn optioneel en gelden voor elk project op deze computer. Veranderen van ze tekent de commentaren die u schrijft volgende. Reacties al geschreven houden de naam waarop ze werden geschreven onder.
settings-desc-author = De naam die op dit project gaat.
settings-desc-structure = Of de hoofdstukken van dit project mappen of afzonderlijke items zijn.
settings-desc-language = De taal waarmee dit project proza wordt gecheckt.
settings-desc-work-backup = Het eigen back-upbeleid van dit project, of het algemene beleid.
settings-desc-personal-dictionary = De woorden die dit project als correct gespeld behandelt.
settings-desc-tags = De kleurgecodeerde labels waarmee dit project zijn items markeert.
settings-desc-templates = De notitiesjablonen die je kunt invoegen tijdens het schrijven in dit project.
settings-desc-text-replacements = Sneltoetsen die uitbreiden als je in dit project typt.
settings-desc-work-punctuation = De citaten, streepjes en afstand van dit project proza volgt.

settings-group-typography = Typografie
settings-group-writing-column = Kolom schrijven
settings-group-writing-view = Weergave schrijven
settings-group-distraction-free-strip = Controlestrook
settings-group-theme = Thema
settings-group-language = Taal
settings-group-startup = Starten
settings-group-updates = Bijwerken
settings-group-autosave = Automatisch opslaan
settings-field-typeface = Typenaam
settings-field-size = Tekstgrootte

editor-size-changed-manuscript = Manuscriptgrootte: { $percent }
editor-size-changed-synopsis = Synopsis formaat tekst: { $percent }
editor-size-changed-notes = Opmerkingen grootte tekst: { $percent }
editor-size-changed-corkboard = Corkboard kaart tekstgrootte: { $percent }
editor-size-changed-corkboard-expanded = Uitgebreide editor tekstgrootte: { $percent }
editor-size-changed-distraction-free = Afleidingsvrije tekstgrootte: { $percent }

menu-text-size-increase = Tekstgrootte vergroten (&I)
menu-text-size-decrease = Tekst verkleinen (&C)
menu-text-size-reset = Tekstgrootte resetten (&R)
settings-field-line-height = Lijnhoogte
settings-field-first-line-indent = Eerste regel streepje
settings-field-paragraph-spacing-before = Spatie voor paragraaf
settings-field-paragraph-spacing-after = Spatie na paragraaf
settings-field-column-width = Kolombreedte
settings-distraction-free-width-hint = Geldt alleen terwijl afleidingsvrije modus is ingeschakeld. Uw Scene, Synopsis en Notes kolombreedtes zijn onaangetast.
settings-distraction-free-title = De naam van het item behouden
settings-distraction-free-word-count = Hou het woord tellen
settings-distraction-free-session = De schrijfsessie behouden
settings-distraction-free-go-to = Hou de Go naar... knop
settings-distraction-free-go = Houd de vorige en volgende knoppen
settings-distraction-free-chrome-hint = De Exit knop blijft altijd, wat u hier ook kiest. Het is uw uitweg als Escape is bezet.
settings-field-app-theme = Thema
settings-theme-light = Licht
settings-theme-dark = Donker
settings-field-text-scale = Interface tekstgrootte
settings-field-language = Interface-taal
settings-synopsis-placement = Synopsispositie
settings-synopsis-placement-none = Geen
settings-synopsis-placement-top = Boven
settings-synopsis-placement-side = Naast
settings-typewriter = Schrijfmachine scrollen
settings-typewriter-tip = Houdt de lijn die u schrijft op een vaste hoogte terwijl het manuscript eronder rolt. Klikken plaatst nog steeds de curret waar u klikt.
settings-typewriter-position = Lijnpositie
settings-typewriter-position-top-third = Bovenste derde
settings-typewriter-position-middle = Midden
settings-typewriter-position-bottom-quarter = Onderkwartier
settings-highlight-scope = Oplichten rond de caret
settings-highlight-scope-none = Geen
settings-highlight-scope-sentence = Veroordeeldheid
settings-highlight-scope-paragraph = Paragraaf
settings-highlight-scope-tip-none = Niets is in de schaduw als je schrijft.
settings-highlight-scope-tip-sentence = Schaduw de zin die je schrijft, dus het onderscheidt zich van die eromheen.
settings-highlight-scope-tip-paragraph = Schilder de hele paragraaf die je schrijft, om de passage waar je aan werkt in het zicht te houden.
settings-group-container-views = Containerweergaven
settings-remember-view = Onthoud de laatste weergave voor elk itemtype
settings-remember-view-tip = Open een container op het scherm dat u voor dat type gebruikt heeft
settings-remember-view-tip-more =
    Een boek, deel en hoofdstuk elk bieden verschillende views (zijn eigen pagina, de volledige
    manuscript, de volledige synopsis). Zet dit aan en elk type heropent de
    kijk je laatst gekozen voor het, bijvoorbeeld schakel een hoofdstuk naar volledig hoofdstuk en de
    volgende hoofdstuk u opent toont ook Full Chapter. Elk type herinnert zich zijn eigen uitzicht.
settings-group-counting = Aantal woorden
settings-counting-auto = Automatisch (per taal)
settings-counting-whitespace = Splitsen op spaties
settings-counting-unicode-words = Unicode woorden
settings-counting-cjk-hybrid = CJK-bewust (per teken)
settings-counting-hint = Automatisch telt Chinees en Japans op karakter en elke andere taal per woord. Verander het alleen als de telling ziet er verkeerd uit voor uw taal.
settings-group-goals-display = Scherm
settings-show-characters = Het aantal tekens in de statusbalk tonen
settings-autosave-hint = Wijzigingen worden automatisch naar schijf geschreven terwijl u schrijft.

settings-sec-work = Project
settings-page-structure = Structuur
settings-page-author = Auteur
settings-field-author-name = Auteursnaam
settings-field-author-placeholder = Facultatief
settings-field-author-hint = Verschijnt op de gecompileerde titelpagina en in geëxporteerde bestandsmetadata. Laat het leeg om het weg te laten. Dit is de byline van het boek. De naam waarmee uw opmerkingen zijn ondertekend is in Instellingen Gebruiker.
settings-group-chapters = Hoofdstukken
tidy-titles-title = Titels van net hoofdstuk
tidy-titles-none = Geen enkel hoofdstuk of deeltitel herhaalt slechts zijn eigen nummer.
tidy-titles-lead = { $count ->
        [one] Een hoofdstuk of deel is getiteld met niets anders dan zijn eigen nummer.
       *[other] { $count } hoofdstukken en delen hebben slechts een eigen nummer.
    }
tidy-titles-explain = Het wissen van die titels laat elk genoemd door het nummer dat het boek al weet, hetzelfde nummer de export afdrukken. Niets anders verandert, en een ongedaan zet ze allemaal terug.
menu-document-tidy-titles = Tidy hoofdstuk titels... (&Y)
settings-group-numbering = Nummering
settings-number-chapters = Aantal hoofdstukken en delen
settings-number-chapters-tip = Hoofdstukken en onderdelen dragen een aantal uitgewerkt uit waar ze zitten in het boek, naast hun titel hier, en gedrukt door de export.
settings-number-chapters-tip-more = Het nummer wordt nooit opgeslagen in de titel, dus het blijft juist wanneer u herorden, invoegen of verwijderen. Zet dit uit en de export afdrukken titels alleen, ongeacht de export stijl vraagt. Om een hoofdstuk uit (een proloog, een interlude), gebruik in plaats daarvan de Inspector's Numbering schakelaar: dat houdt het in het boek, maar stopt het nemen van een nummer.
settings-part-resets-chapter = Hoofdstuknummers voor elk deel herstarten
settings-part-resets-chapter-tip = Standaard: hoofdstukken lopen rechtdoor over de delen van een boek, dus "Part Two" opent op Hoofdstuk Elf.
settings-part-resets-chapter-tip-more = Dat is de gebruikelijke trade praktijk, en wat een lezer verwacht. Zet het aan voor een boek waarvan de onderdelen bedoeld zijn om te lezen als aparte volumes, waar elk opent op Hoofdstuk Een.
settings-chapter-flat = Platte hoofdstukken
settings-chapter-flat-hint = Op: een hoofdstuk is een enkel item. Je schrijft erin en het bevat geen scènes. Uit: een hoofdstuk is een map. Je schrijft er nog steeds in, maar het kan ook scènes bevatten. Nieuwe hoofdstukken volgen deze instelling; bestaande hoofdstukken converteren via Promote.

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
settings-styles-editor-none = Selecteer een aangepaste stijl om het te bewerken, of dupliceer een ingebouwde stijl.
settings-styles-imported = Stijl geïmporteerd
settings-styles-import-failed = Kon stijl niet importeren
settings-styles-exported = Stijl geëxporteerd
settings-styles-export-failed = Kon stijl niet exporteren
settings-styles-field-name = Naam
settings-styles-field-chapters = Hoofdstuktitels
settings-styles-field-parts = Rubrieken
settings-styles-field-scene-break = Scènebreuk
settings-styles-field-major-scene-break = Grote breuk in de scène
settings-styles-field-spacing = Lijnafstand
settings-styles-field-justify = Tekst rechtvaardigen
settings-styles-field-notes = Noten toevoegen
settings-styles-field-synopses = Synopsen toevoegen
settings-styles-field-scene-titles = Voeg scenetitels toe
settings-styles-field-epigraphs = Epigraphs opnemen
settings-styles-field-paratexts = Parateksten toevoegen
settings-styles-field-epigraph-placement = Epigraph positie
settings-styles-epigraph-after = Na de titel
settings-styles-epigraph-before = Vóór de titel
settings-styles-field-footnotes = voetnoten invoegen
settings-styles-field-footnote-numbering = Nummering van de voetnoot
settings-styles-footnote-numbering-continuous = Continu
settings-styles-footnote-numbering-per-chapter = Elk hoofdstuk herstarten
settings-styles-footnote-numbering-per-book = Herstart elk boek
settings-styles-field-images = Afbeeldingen
menu-image = Afbeelding (&I)
settings-styles-images-beside = Naast het document
settings-styles-images-embed = In het document
settings-styles-images-omit = Laat maar.
settings-styles-group-round-trip = Verzenden naar een editor
settings-styles-field-comments = Opmerkingen meenemen
settings-styles-field-round-trip-marks = Inclusief round-trip markers
settings-styles-round-trip-hint = Alleen DOCX en ODT. Markers zijn onzichtbare identificaties die een terugkerend bestand dit project laten bijwerken in plaats van ernaast te landen als een tweede kopie.
settings-styles-group-pages = Bladzijden
settings-styles-field-cover = Openen met de hoes
settings-styles-field-word-count = Word count op titelpagina
settings-styles-field-page-books = Nieuwe pagina bij elk boek
settings-styles-field-page-parts = Nieuwe pagina op elk deel
settings-styles-field-page-chapters = Nieuwe bladzijde bij elk hoofdstuk
settings-styles-field-page-paratexts = Nieuwe pagina bij elke paratekst
settings-styles-heading-none = Nr. van het
settings-styles-heading-numbered = Alleen nummer
settings-styles-heading-title = Uitsluitend titel
settings-styles-heading-both = Nummer + titel
settings-styles-break-blank = Lege lijn
settings-styles-break-none = Geen
settings-styles-spacing-single = Enkelvoudig
settings-styles-spacing-onehalf = 1½
settings-styles-spacing-double = Dubbel

welcome-title = Welkom bij Skribisto
welcome-version = Versie { $version }
welcome-update-available = Versie { $version } is beschikbaar
welcome-update-tip = Opent de download pagina in uw browser.
welcome-search = Zoeken projecten
welcome-open = Openen
welcome-new-work = Nieuw project
welcome-create-from = Aanmaken van...
welcome-recent-works = Recente projecten
welcome-empty-recents = Nog geen recente projecten.
welcome-no-matches = Geen recent project komt overeen met uw zoekopdracht.
welcome-learn-soon = Er komen binnenkort gidsen en tips.
welcome-about-blurb = Skribisto, een Rust + Teksilo herschrijven van de schrijfapp.
welcome-tagline = *Een rustige plek om lange dingen te schrijven. *
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
    [one] { $count } post
   *[other] { $count } posten
}
corkboard-view-nested = geneste
corkboard-view-flat = Plat
corkboard-layout-hint = Nested toont de directe kinderen van een container. Open een mapkaart om erin te gaan. Plat toont elke scène in de hele container tegelijk.
corkboard-card-size = Kaartgrootte
corkboard-search-placeholder = Filterkaarten...
corkboard-empty-title = Nog niets hier.
corkboard-empty-hint = Gebruik "+ New" hierboven om de eerste toe te voegen.
corkboard-new = Nieuw
corkboard-show-card-numbers = Nummer de kaarten
corkboard-modal-size = Uitgebreide editorgrootte
corkboard-scope-hint = Geldt voor elk open kurkenbord, in elk project.
corkboard-sort-manuscript = Manuscriptvolgorde
corkboard-sort-title-asc = Titel AZ
corkboard-sort-title-desc = Titel Z.A.
corkboard-card-number = Kaart { $number }

duplicate = Dupliceren
duplicate-n = { $count ->
    [one] Dubbele { $count } kaart
   *[other] Dubbele { $count } kaarten
}
set-label-n = { $count ->
    [one] Stel het etiket in op { $count } kaart
   *[other] Label instellen op { $count } kaarten
}
move-to-trash-n = { $count ->
    [one] { $count } kaart naar prullenbak verplaatsen
   *[other] { $count } kaarten naar prullenbak verplaatsen
}
reveal-in-outline = Onthullen in overzicht

corkboard-move-to = Ga naar...
corkboard-move-to-n = { $count ->
    [one] { $count } kaart verplaatsen naar...
   *[other] { $count } kaarten verplaatsen naar...
}
corkboard-move-picker-title = Ga naar...
corkboard-move-picker-empty = Maak er eerst een.
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
overview-col-label = Korte beschrijving
overview-col-tags = Tags
overview-col-books = Boeken
overview-col-own-words = Woorden
overview-col-total-words = Totaal
overview-row-count = { $count ->
    [one] { $count } post
   *[other] { $count } posten
}
overview-search-placeholder = Filter items...
overview-expand-all = Alles uitvouwen
overview-collapse-all = Alles invouwen
overview-table-label = Inhoud
overview-empty-title = Nog niets hier.
overview-gone-title = Deze container is weg.
overview-gone-hint = Het werd verplaatst naar de prullenbak. Herstel het, of sluit deze tab.
overview-empty-hint = Gebruik "+ New" hierboven om de eerste toe te voegen.
corkboard-grid-label = Kurkenbordkaarten
corkboard-rename-field = Item hernoemen
corkboard-expand-synopsis = Synopsis uitbreiden
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
settings-group-corkboard-layout = Opmaak
settings-group-corkboard-cards = Kaarten
corkboard-show-word-count = Het woord tellen op kaarten tonen
text-heading = Tekst
no-content = Dit item heeft geen bewerkbare inhoud.
untitled = Naamloos
placeholder-title = Titel…
placeholder-subtitle = Ondertiteling:
placeholder-chapter-title = Hoofdstuktitel...
split-editor = Editor splitsen
close-split-view = Gesplitste weergave sluiten
drop-open-here = Open hier
drop-open-to-side = Aan de zijkant openen

segment-chapter = Hoofdstuk
segment-part = Deel
segment-book = Boek
segment-pace = Schrijftempo
segment-notes = Notities
segment-story-bible = Verhaalbijbel
segment-note-own = Notitie
segment-note-details = Details
segment-note-in-prose = In proza
pace-placeholder = De Pace planner verschijnt hier.
pace-empty-title = Plan het tempo van dit boek
pace-empty-body = Stel een woord doel en een deadline, en Skribisto Projecten uit het dagelijkse tempo om je er te krijgen.
pace-start-planning = Beginnen met plannen
pace-section-schedule = Schema
pace-section-progress = Voortgang
pace-advancement = Voorschot
pace-goal = Woordendoel
pace-deadline = Deadline
pace-active = Pace actief
pace-day-mon = maabbreviated month name
pace-day-tue = Di
pace-day-wed = wo
pace-day-thu = do
pace-day-fri = Vr
pace-day-sat = Zat
pace-day-sun = Zon
pace-card-written = woorden geschreven
pace-card-of-goal = van het doel
pace-card-rate = woorden / schrijfdag
pace-card-days-left = nog schrijvende dagen over
pace-card-streak = dagstreak
pace-card-ahead = woorden voor tempo
pace-card-behind = woorden achter het tempo
pace-charts-empty = Voortgangsgrafieken verschijnen hier zodra uw woord tellen is opgenomen op een opslaan.
pace-chart-progression = Woorden geschreven vs. doel
pace-chart-words-per-day = Woorden per dag
pace-series-actual = Werkelijk
pace-series-target = Doel
pace-series-words-per-day = Woorden/dag
pace-daily-target-line = Even tempo: { $count ->
    [one] { $count } woord/dag
   *[other] { $count } woorden/dag
}
pace-section-holidays = Vakantie
pace-section-milestones = Mijlpalen
pace-holidays-none = Elke geplande dag telt.
pace-holiday-label = Vakantienaam
pace-add-holiday = Toevoegen
pace-remove = Verwijderen
pace-milestones-none = Nog geen mijlpalen. Zet er een op een deel of hoofdstuk in de inspecteur.
full-chapter = Volledig hoofdstuk
full-part = Volledig deel
full-book = Volledig boek
full-synopsis = Volledige samenvatting
rename = Naam wijzigen
set-label = Label instellen
insert-scene = Scene toevoegen
insert-chapter = Hoofdstuk invoegen
split-scene = Scene splitsen
move-up = Omhoog verplaatsen
move-down = Omlaag verplaatsen
merge-with-previous = Samenvoegen met vorige
move-to-trash = Naar prullenbak verplaatsen
rename-chapter = Hoofdstuk hernoemen
add-scene = Nieuwe scene toevoegen
add-chapter = Nieuw hoofdstuk toevoegen
new-scene-title = Nieuwe scène
placeholder-scene-name = Scènenaam...
menu-cut = Knippen
menu-copy = Kopiëren
menu-paste = Plakken
menu-paste-unformatted = Plakken zonder opmaak
menu-select-all = Alles selecteren
format-bold = Vet
format-italic = Cursief
format-underline = Onderstrepen
format-strikethrough = Doorgestreept

binder = Documentboom
no-work = Geen project geladen
no-recent-works = Geen recente projecten
switcher-open-section = Momenteel geopend
switcher-recent-section = Recent
switcher-this-window = dit venster
open-project-title = Project openen
open-project-question = Hoe wil je "{ $title }" openen?
open-in-new-window = In nieuw venster openen
open-here = Open hier

binder-all = Alle Bindmiddelen
binder-show-all = Alle bindmiddelen tonen
binder-new = Nieuw bindmiddel...
binder-default-name = Nieuw bindmiddel
binder-item-count = { $count ->
    [one] { $count } post
   *[other] { $count } posten
}
binder-search-placeholder = Filter de omtrek...
binder-search-scope = Alle bindmiddelen doorzoeken
binder-trash-confirm-title = Binder naar de prullenbak verplaatsen?
binder-trash-confirm-text = "{ $name }" en al zijn items zullen naar de prullenbak worden verplaatst.

dialog-rename = Naam wijzigen
dialog-new-binder = Nieuw bindmiddel
dialog-set-label = Label instellen
dialog-new-scene = Nieuwe scène
close-work-question = Wijzigingen opslaan vóór het sluiten van het project?
quit-question = Wijzigingen opslaan vóór het afsluiten?
quit-save-work-question = Wijzigingen opslaan in { $title } voordat u stopt?
unsaved-changes = Dit project bevat niet-opgeslagen wijzigingen.
new-work-unsaved-question = Wijzigingen opslaan voordat u een nieuw project aanmaakt?
open-work-unsaved-question = Wijzigingen opslaan voordat u een ander project opent?
switch-backup-discard-title = Wijzigingen in deze back-up verwerpen?
switch-backup-discard-text = Wijzigingen in een back-up kunnen niet worden opgeslagen. Gebruik Opslaan als of herstellen om ze te bewaren, of gooi ze weg en open het andere project.
switch-save-failed = Het project kon niet worden opgeslagen, dus het werd niet vervangen: { $error }
switch-save-not-started = Het project kon niet worden opgeslagen, dus het werd niet vervangen.
close-save-failed = Het project kon niet worden opgeslagen, dus het was niet gesloten: { $error }
close-save-not-started = Het project kon niet worden gered, dus het was niet gesloten.
save-not-started = Het project kon niet worden gered.

could-not-open-work = Kon "{ $file }" niet openen
could-not-open-work-too-new = "{ $file }" heeft een nieuwer Skribisto nodig
could-not-open-work-too-new-detail = Opgeslagen door Skribisto formaat { $written_by }; het openen van het moet formaat { $requires } of nieuwer, en deze bouw ondersteunt tot formaat { $supported }. Update Skribisto om het te openen.
could-not-open-work-importing = "{ $file }" wordt geschreven door een import
could-not-open-work-importing-detail = Een import is aan het schrijven over dit project en is nog niet klaar. Open het zodra de import is gedaan: geopend nu, het zou tonen het project de import wordt vervangen, en de volgende opslag zou dat project terug te schrijven over de import.
could-not-open-example = Kon het voorbeeld niet openen: { $error }
could-not-open-link = Kon { $url } niet openen: { $error }
could-not-create-work = Kon project niet aanmaken: { $error }
saving-as-file = Opslaan als { $target }...
saving-as-folder = Opslaan als { $target }/...
saved-as = Opgeslagen naar { $target }
save-error = Kon niet opslaan: { $error }
save-flush-failed = De redding ging door, maar deze documenten konden niet worden overhandigd en zijn nog steeds niet gered: { $titles } ({ $error })
backup-error = Kon geen back-up maken: { $error }
backing-up = Terugtrekken...
backup-nothing-open = Geen enkel project is open voor back-up.
backup-already-running = Er is al een backup aan de gang.
backup-complete = Backup voltooid ({ $ok } opgeslagen, { $skipped } reeds actueel)
backup-partial = Back-up voltooid: { $ok } opgeslagen, { $failed ->
    [one] { $failed }-doel is mislukt
   *[other] { $failed } bestemmingen mislukt
}
backup-no-destination-title = Geen reservekopielocatie beschikbaar
backup-no-destination-text = Geen van de geconfigureerde back-upbestemmingen kan worden bereikt (bijvoorbeeld, een externe schijf kan worden uitgeschakeld). Plug het in en probeer het opnieuw, of sluit af zonder een back-up te maken.
backup-failed-close-title = De reservekopie kon niet worden opgeslagen
backup-failed-close-text = Er kon geen back-upkopie worden geschreven voor het sluiten. Elke bestemming is mislukt (de schijf kan zijn verwijderd, of het kan volledig of schrijf-beschermd zijn). Fix it and retry, or exit without back-up.

backup-choice-title = Reservekopiebestand
backup-choice-heading = Je hebt een reservekopie geopend.
backup-choice-subtitle = Dit is een point-in-time back-up van een project.
backup-choice-subtitle-dated = Back-up genomen { $date }.
backup-choice-body = U kunt het openen en vrij bewerken, maar wijzigingen kunnen alleen bewaard worden met Opslaan als. Het oorspronkelijke projectbestand wordt niet aangeraakt. Of dit project herstellen naar precies deze back-up.
backup-choice-open = Open de back-up
backup-choice-restore = Herstel dit project tot op dit punt...
backup-choice-not-a-backup = Nee, doe het normaal open.
backup-banner-title = Reservekopie: wijzigingen kunnen hier niet worden opgeslagen
backup-banner-description = Gebruik Opslaan als om uw bewerkingen in een nieuw bestand te houden, of Herstel om het oorspronkelijke project te vervangen door deze back-up.
backup-banner-restore = Herstellen…
backup-banner-save-as = Opslaan als...
backup-restore-original-missing = Kan het oorspronkelijke project niet vinden om overheen te herstellen. Gebruik Opslaan als om deze back-up als nieuw project te bewaren.
backup-restore-close-elsewhere-title = Project geopend in een ander venster
backup-restore-close-elsewhere-text = Het project dat u aan het herstellen bent, is geopend in een ander venster. Sluit het daar eerst af en probeer het dan opnieuw.
backup-restore-focus-window = Concentreer dat venster
backup-restore-confirm-title = Deze back-up herstellen?
backup-restore-confirm-text = De huidige versie van het project zal worden gekopieerd als veiligheidsback-up voordat deze wordt vervangen door deze.
backup-restore-confirm-ok = Herstellen
backup-restore-error = Kon niet herstellen: { $error }
backup-restored-ok = Project hersteld.
backup-restored-with-safety = Project hersteld. Uw vorige versie is opgeslagen op { $path }.
close-backup-discard-title = Wijzigingen in deze back-up verwerpen?
close-backup-discard-text = Wijzigingen in een back-up kunnen niet worden opgeslagen. Gebruik Opslaan als om ze te bewaren, of weggooien en sluiten.
quit-backup-discard-title = Veranderingen weggooien en afsluiten?
quit-backup-discard-work-question = Wijzigingen verwerpen naar { $title } en stoppen?
quit-backup-discard-text = Wijzigingen in een reservekopie kunnen er niet in opgeslagen worden. Gebruik Opslaan om ze te bewaren, of gooi ze weg en stop ermee.
backup-nudge-text = Er zijn geen back-ups voor dit project ingesteld.
backup-nudge-action = Reservekopieën instellen...

menu-backups-list = Reservekopieën… (&K)
backups-title = Reservekopieën
backups-loading = Scannen naar back-ups...
backups-empty = Nog geen back-ups gevonden voor dit project.
backups-open = Openen
backups-reveal = Openen
backups-delete = Deze reservekopie verwijderen
backups-delete-confirm-title = Deze back-up verwijderen?
backups-delete-confirm-text = ‘{ $name }’ wordt definitief verwijderd. Dit kan niet ongedaan worden gemaakt.
backups-delete-error = Kon de reservekopie niet verwijderen: { $error }
backups-refresh = Verversen
backups-close = Sluiten

settings-page-backup = Standaardinstellingen voor backups
settings-page-work-backup = Reservekopieën
settings-backup-general-title = Standaard reservekopieinstellingen
settings-backup-work-title = Reservekopieën voor dit project
settings-backup-inherit = Algemene reservekopie-instellingen gebruiken
settings-backup-inheriting = Dit project gebruikt de algemene back-upinstellingen.
settings-backup-none-hint = Er worden geen automatische back-ups ingesteld (elke trigger is uitgeschakeld).
settings-backup-last = Laatste back-up: { $date }
settings-backup-last-never = Nog geen back-ups.
settings-backup-open-list = Reservekopielijst openen...
settings-backup-triggers = Wanneer moet ik me terugtrekken?
settings-backup-on-close = Een back-up maken bij het afsluiten van het project
settings-backup-on-open = Back-up maken bij het openen van het project
settings-backup-interval = Af en toe een back-up, elke
settings-backup-destinations = Reservekopie van bestemmingen
settings-backup-default-location = Standaardlocatie
settings-backup-usage-since = { $count ->
    [one] { $count } back-up · { $size } · oudste { $oldest }
   *[other] { $count } back-ups · { $size } · oudste { $oldest }
}
settings-backup-usage = { $count ->
    [one] { $count } back-up · { $size }
   *[other] { $count } back-ups · { $size }
}
settings-backup-usage-empty = Er is hier nog niets bewaard.
settings-backup-usage-measuring = Meten...
settings-backup-reveal-root = Map tonen
settings-backup-dest-none = Geen bestemmingen. Back-ups worden opgeslagen naast het project.
settings-backup-dest-remove = Verwijderen
settings-backup-dest-add = Map toevoegen...
settings-backup-dest-refresh = Verversen
settings-backup-retention = Hoeveel te houden.
settings-backup-retention-tiered = Geplaatst
settings-backup-retention-keep-n = Laatste N houden
settings-backup-retention-tip = Hoe oude back-ups zijn gesnoeid.
settings-backup-retention-tip-more = Tiered houdt een back-up per uur voor een dag, per dag voor een week, per week voor een maand, en per maand daarna, zodat de recente geschiedenis blijft dicht en oude geschiedenis uitdunt. "Keep last N" gewoon houdt de N meest recente kopieën. In beide modi de nieuwste kopieën (het minimum hieronder) worden altijd gehouden.
settings-backup-gfs-hourly = Uur (laatste 24 uur)
settings-backup-gfs-daily = Dagelijks (laatste week)
settings-backup-gfs-weekly = Wekelijks (laatste maand)
settings-backup-gfs-monthly = Maandelijks
settings-backup-keep-n = Aantal te bewaren
settings-backup-min-keep = Altijd op zijn minst houden
settings-backup-dedup = Een reservekopie overslaan als er niets is veranderd

new-work-title = Nieuw project
new-work-close = Sluiten
new-work-name = Projectnaam
new-work-name-placeholder = Naamloos
new-work-author = Auteur
new-work-author-placeholder = Facultatief
new-work-format = Opmaak
new-work-single-file = Eén bestand
new-work-bundle = Bundel
new-work-convert-later = U kunt converteren tussen formaten later.
new-work-location = Locatie
new-work-will-create = Zal aanmaken
new-work-language = Standaardtaal
new-work-language-hint = Toegepast op nieuwe teksten spellingscontrole. Elke tekst kan worden omgezet naar een andere taal.
new-work-template = Sjabloon
new-work-template-none = Geen
new-work-template-empty-novel = Lege novel
new-work-template-light-novel = Lichtnevel
new-work-template-novel = Roman
new-work-template-novel-in-parts = Nieuwe delen
new-work-template-notebook = Notitieboek
new-work-cancel = Annuleren
new-work-create = Project aanmaken
new-work-step-details = Details
new-work-step-language = Taalstructuur
new-work-step-template = Sjabloon
new-work-back = Terug
new-work-next = Volgende
new-work-documents-title = Nieuw project uit documenten
new-work-step-import = Importeren
new-work-create-and-import = Importeren...
new-work-documents-next-title = Uw documenten komen nu.
new-work-documents-next-body = Het maken van het project opent de import wizard over het, waar u de documenten kiest, de structuur bekijken Skribisto leest van hen, en kies waar het landt. Niets is geschreven in het project totdat u het daar bevestigt.
new-work-documents-no-template = Dit project start met opzet leeg: geen sjabloon, dus de geïmporteerde documenten zijn het enige wat erin staat.
new-work-documents-chapter-scene-hint = Geldt voor de hoofdstukken die de import aanmaakt. U kunt het later wijzigen in de instellingen van het project.
new-work-single-file-desc = Een .skrib archief (zip). Draagbaar, makkelijk te back-uppen.
new-work-bundle-desc = Een map met elke tekst.
new-work-template-none-count = leeg bindmiddel
new-work-template-empty-novel-count = 1 hoofdstuk
new-work-template-light-novel-count = 15 hoofdstukken
new-work-template-novel-count = 20 hoofdstukken
new-work-template-novel-in-parts-count = 3 delen, 24 hoofdstukken
new-work-template-notebook-count = Noten in vrije vorm
new-work-chapter-scene = Platte hoofdstukken
new-work-chapter-scene-tip = Elk hoofdstuk is een enkel item waar je direct in schrijft, zonder scènes eronder. Laat dit achterwege voor de klassieke lay-out, waar een hoofdstuk een map is: je schrijft daar ook direct in, maar het kan ook scènes bevatten.
new-work-chapter-scene-tip-more = Je schrijft hoe dan ook in een hoofdstuk. Het enige verschil is of het *contain* scènes. Skribisto's mapboom is alleen organisatorisch, dus beide lay-outs compileren naar hetzelfde boek, je kunt ze vrij mengen, en Promoot converteert een hoofdstuk tussen de twee zonder een woord te verliezen.
new-work-name-required = Geef een naam voor het project
new-work-name-invalid = Deze naam heeft geen bruikbare tekens
new-work-location-required = Kies een locatie
new-work-location-missing = Deze map bestaat niet
new-work-location-not-folder = Dit pad is geen map
new-work-location-readonly = Deze map is niet beschrijfbaar
new-work-target-exists = Een project met deze naam staat al in deze map. Het project aanmaken zal vragen voordat het wordt vervangen
new-work-target-is-folder = Een map met deze naam bevindt zich al in deze locatie. Kies een andere naam
new-work-overwrite-title = Het bestaande project vervangen?
new-work-overwrite-text = "{ $name }" bestaat al in deze map. Vervangen door het nieuwe project?
new-work-target-open-title = Dit project is geopend
new-work-target-open-text = "{ $name }" is geopend in Skribisto. Het creëren van een nieuw project op zijn plaats zou het project waar u aan werkt vervangen, en uw volgende opslaan zou dat project terug te schrijven over de nieuwe. Kies een andere naam of map, of sluit het project eerst.
target-importing-title = Een import is dit project aan het schrijven
target-importing-text = Een import is nog steeds bezig met het schrijven van "{ $name }." Wacht tot het klaar is, of kies een andere naam.
import-plume-busy-title = Een ander Plume Creator project wordt geïmporteerd
import-plume-busy-text = Plume Creator projecten worden een voor een geïmporteerd. Wacht tot de huidige import is voltooid, of annuleer het, dan start deze opnieuw.
import-manuskript-busy-title = Een ander Manuskript project wordt geïmporteerd
import-manuskript-busy-text = Manuskript projecten worden een voor een geïmporteerd. Wacht tot de huidige import is voltooid, of annuleer het, dan start deze opnieuw.
backup-restore-importing-text = Een import is nog bezig met het schrijven van "{ $name }," het project dat deze back-up zou worden hersteld. Herstel de back-up zodra de import is voltooid.

new-work-manuscript = Manuscript
new-work-notes = Notities
new-work-research = Onderzoek
new-work-notebook = Notitieboek
new-work-scene = Scène
new-work-note = Notitie
new-work-front-matter = Voorgrond
new-work-back-matter = Achterzijde
new-work-paratext = Boekstructuur
new-work-paratext-none = Geen structuur
new-work-paratext-hint = De voor- en achterkant een traditie opent en sluit een boek met. U kunt bewegen, hernoemen of verwijderen van een van de achteraf.
new-work-tags = Tags
new-work-tags-none = Geen tags
new-work-tags-hint = Een startpalet voor het taggen van tekens, plaatsen en de rest. Optioneel, en elke tag kan worden hernoemd, herkleurd of verwijderd achteraf.
new-work-note-templates = Nootsjablonen
new-work-note-templates-none = Geen sjablonen
new-work-note-templates-hint = De vorm van een verhaal-bijbel noot begint in. Optioneel, en elke sjabloon kan achteraf worden bewerkt of verwijderd.
new-work-characters = Personages
new-work-places = Plaatsen

import-plume-title = Project importeren Plume Creator
import-plume-close = Sluiten
import-plume-source = Plume-project
import-plume-source-hint = Kies een .plume of .plume_backup bestand (elk Plume Creator versie).
import-plume-location = Doelmap
import-plume-name = Bestandnaam
import-plume-name-placeholder = Projectnaam
import-plume-will-create = Zal aanmaken
import-plume-trash-warning = Getrapte / verwijderde items worden niet gemigreerd.
import-plume-cancel = Annuleren
import-plume-import = Importeren
import-plume-source-required = Een Plume-projectbestand kiezen
import-plume-source-missing = Dit bestand bestaat niet
import-plume-source-not-file = Dit pad is geen bestand
import-plume-location-required = Kies een doelmap
import-plume-location-missing = Deze map bestaat niet
import-plume-location-not-folder = Dit pad is geen map
import-plume-location-readonly = Deze map is niet beschrijfbaar
import-plume-name-required = Een bestandsnaam invoeren
import-plume-name-exists = Een bestand met deze naam bestaat hier al. Importeren zal overschrijven bevestigen
import-plume-overwrite-title = Bestaande bestand vervangen?
import-plume-overwrite-text = "{ $name }" bestaat al. Vervangen door het geïmporteerde project?
import-plume-manuscript-binder = Manuscript
import-plume-story-bible-binder = Verhaalbijbel
import-plume-progress-title = Plume-project wordt geïmporteerd...
import-plume-cancel-import = Annuleren
import-plume-cancelled = Importeren geannuleerd
import-plume-done = Geïmporteerd { $imported ->
    [one] { $imported } punt.
   *[other] { $imported } items.
 } { $skipped ->
    [one] { $skipped } vernielde item werd niet gemigreerd.
   *[other] { $skipped } voorwerpen werden niet gemigreerd.
}
import-plume-open-now = Nu openen
import-plume-warnings = { $count ->
    [one] 1 ding kon niet precies geïmporteerd worden
   *[other] { $count } dingen konden niet precies worden geïmporteerd
}
import-plume-details = Details
import-plume-warnings-title = Waarschuwingen importeren
import-plume-error-title = Kon het project niet importeren
import-plume-nested-too-deep = Het project werd niet geïmporteerd: het "{ $part }" deel is meer dan { $limit } niveaus diep genesteld. No Plume Creator project komt in de buurt van dat, en het lezen kon crashen Skribisto. Niets werd geschreven.
import-plume-declares-entities = Het project werd niet geïmporteerd: het "{ $part }" deel verklaart XML entiteiten, wat geen Plume Creator project doet. Uitbreiden kan meer geheugen dan de computer heeft nodig en crash Skribisto. Er is niets geschreven.
import-plume-error-details = Details

import-manuskript-title = Project importeren Manuskript
import-manuskript-close = Sluiten
import-manuskript-source = Manuskript-project
import-manuskript-source-hint = Kies het .msk bestand of de projectmap (een Manuskript versie).
import-manuskript-source-file = Bestand kiezen...
import-manuskript-source-folder = Map kiezen...
import-manuskript-location = Doelmap
import-manuskript-name = Bestandnaam
import-manuskript-name-placeholder = Projectnaam
import-manuskript-will-create = Zal aanmaken
import-manuskript-cancel = Annuleren
import-manuskript-import = Importeren
import-manuskript-source-required = Kies een Manuskript project
import-manuskript-source-missing = Dit bestand of map bestaat niet
import-manuskript-location-required = Kies een doelmap
import-manuskript-location-missing = Deze map bestaat niet
import-manuskript-location-not-folder = Dit pad is geen map
import-manuskript-location-readonly = Deze map is niet beschrijfbaar
import-manuskript-name-required = Een bestandsnaam invoeren
import-manuskript-name-exists = Een bestand met deze naam bestaat hier al. Importeren zal overschrijven bevestigen
import-manuskript-overwrite-title = Bestaande bestand vervangen?
import-manuskript-overwrite-text = "{ $name }" bestaat al. Vervangen door het geïmporteerde project?
import-manuskript-manuscript-binder = Manuscript
import-manuskript-story-bible-binder = Verhaalbijbel
import-manuskript-characters-group = Personages
import-manuskript-world-group = Wereld
import-manuskript-plots-group = Verhaallijnen
import-manuskript-project-info-note = Projectinformatie
import-manuskript-summary-note = Samenvatting
import-manuskript-importance-minor = Klein
import-manuskript-importance-secondary = Secundair
import-manuskript-importance-main = Hoofd
import-manuskript-progress-title = Het importeren van Manuskript project...
import-manuskript-cancel-import = Annuleren
import-manuskript-cancelled = Importeren geannuleerd
import-manuskript-done = { $imported ->
    [one] { $imported } invoer.
   *[other] { $imported } ingevoerde producten.
 } { $revisions ->
    [0] { "" }
    [one] { $revisions } eerdere versie kwam met het.
   *[other] { $revisions } eerdere versies kwam met hen.
}
import-manuskript-open-now = Nu openen
import-manuskript-warnings = { $count ->
    [one] 1 ding te weten over deze import
   *[other] { $count } dingen te weten over deze import
}
import-manuskript-details = Details
import-manuskript-warnings-title = Info over deze import
import-manuskript-error-title = Kon het project niet importeren
import-manuskript-nested-too-deep = Het project werd niet geïmporteerd: het bestand "{ $part }" is meer dan { $limit } niveaus diep genesteld. No Manuskript project komt in de buurt van dat, en het lezen van het kon crash Skribisto. Niets werd geschreven.
import-manuskript-folders-too-deep = Het project werd niet geïmporteerd: een van de bestanden zit meer dan { $limit } mappen diep. No Manuskript project komt in de buurt van dat, en het lezen kan crashen Skribisto. Niets werd geschreven.
import-manuskript-declares-entities = Het project werd niet geïmporteerd: het bestand "{ $part }" verklaart XML entiteiten, wat geen Manuskript project doet. Uitbreiden kan meer geheugen dan de computer heeft nodig en crash Skribisto. Er werd niets geschreven.
import-manuskript-error-details = Details

import-target-open-title = Dit project is geopend
import-target-open-text = "{ $name }" is geopend in Skribisto. Importeren over het zou het project waar u aan werkt vervangen, en uw volgende opslaan zou dat project terug te schrijven over de import. Kies een andere bestandsnaam, of sluit het project eerst.
import-warnings-announcement = { $result } { $notice }.

export-title = Exporteren
export-close = Sluiten
export-scope-label = Wat?
export-format-label = Opmaak
export-style-label = Stijl
export-path-label = Bestand
export-browse = Bladeren...
export-show-non-exportable = Niet-exporteerbaar tonen
export-choose-empty = Geen project om uit te kiezen.
export-cancel = Annuleren
export-export = Exporteren
export-save-dialog-title = Exporteren naar bestand
export-section-what = Wat te exporteren
export-section-style = Stijl
export-section-destination = Bestemming
export-custom-selection = Zelfgemaakte selectie
export-selected-count = { $count } geselecteerd
export-chip-chapters-none = Hoofdstukken: geen
export-chip-chapters-numbered = Hoofdstukken: genummerd
export-chip-chapters-title = Hoofdstukken: uitsluitend titel
export-chip-chapters-both = Hoofdstukken: nummer + titel
export-chip-scene-break-glyph = Scènebreuk: { $glyph }
export-chip-scene-break-blank = Scènebreuk: lege regel
export-chip-scene-break-none = Scènebreuk: geen
export-chip-major-break-glyph = Grote breuk { $glyph }
export-chip-major-break-blank = Grote breuk: lege regel
export-chip-major-break-none = Grote breuk: geen
export-chip-major-break-same = Beide breukniveaus gelijk
export-chip-spacing-single = Spatiëring: enkele
export-chip-spacing-onehalf = Ruimte: 11⁄2
export-chip-spacing-double = Spatiëring: dubbel
export-chip-notes-included = Inclusief aantekeningen
export-chip-notes-excluded = Uitgesloten opmerkingen
export-format-docx = Woord
export-format-odt = LibreOffice
export-format-html = HTML
export-format-markdown = Markdown
export-format-djot = Djot
export-format-text = Tekst
export-format-latex = LaTeX
export-format-epub = EPUB
export-format-pdf = PDF
export-overwrite-title = Bestaande bestand vervangen?
export-overwrite-text = '{ $name }' bestaat al.
export-progress-title = Exporteren...
export-cancelled = Exporteren geannuleerd
export-done = Exporteren { $count ->
    [one] { $count } post
   *[other] { $count } posten
}
export-open-file = Openen
export-show-in-folder = Toon in map
export-error-title = Kon niet exporteren
export-error-details = Details

backup-progress-start = Beginnen...
backup-progress-retention = Oude backups opruimen...
backup-progress-done = Gereed
backup-progress-destination = Bestemming { $i } van { $n }
backup-details = Details
backup-issues-title = Back-upproblemen
backup-failed-title = Reservekopie is mislukt
backup-complete-prune-warning = Back-up voltooid ({ $ok } opgeslagen, { $skipped } reeds huidige). Sommige oude back-ups konden niet verwijderd worden

search = Zoeken
search-query-placeholder = Zoeken…
search-replace-placeholder = Vervangen door ...
search-replace-toggle = Vervangen aan/uit
search-replace-all = Alles vervangen
search-opt-case = Hoofdlettergevoelig
search-opt-whole-word = Het hele woord
search-opt-diacritics = Bijpassende accenten
search-scope-body = Lichaam
search-scope-title = Titel
search-scope-synopsis = Synopsis
search-scope-label = Korte beschrijving
search-facet-book = Boeken
search-facet-part = Delen
search-facet-chapter = Hoofdstukken
search-facet-scene = Scènes
search-facet-note = Notities
search-facet-folder = Mappen
search-tip-case = Match case: behandel hoofdletter en kleine letters als verschillend, dus "Elena" en "elena" zijn aparte overeenkomsten.
search-tip-whole-word = Het hele woord: match alleen volledige woorden, dus "kat" is niet gevonden in "categorie."
search-tip-diacritics = Match accenten: behandel geaccentueerde letters als onderscheidend, dus "cafe" komt niet overeen met "café."
search-tip-body = Lichaam: doorzoek de proza van scènes en notities.
search-tip-title = Titel: doorzoek de titels van de map items.
search-tip-synopsis = Synopsis: zoek de samenvatting van elk schrijvend item.
search-tip-label = Label: doorzoek de korte noot die getoond wordt onder de titel van een bindmiddel.
search-tip-comment = Commentaar: doorzoek de tekst van commentaar threads en hun antwoorden. Een commentaar is over het manuscript in plaats van een deel ervan, dus het heeft zijn eigen switch, en een vervanger laat commentaar komt overeen met unticked totdat u ze aanvinkt.
search-tip-book = Boeken: de boekencontainer en de begin-/eindmarkeringen.
search-tip-part = Delen: delen van verdelers op partlevel.
search-tip-chapter = Hoofdstukken: hoofdstukken, op welke manier het project ze ook opslaat.
search-tip-scene = Scènes: de items die je proza vasthouden.
search-tip-note = Opmerkingen: nota's in vrije vorm.
search-tip-folder = Mappen: eenvoudig mappen en scheidingstekens organiseren.
search-tip-paratext = Een tekst die tot het boek behoort maar niet tot het verhaal: een voorwoord, een toewijding, een nawoord. Nooit in het manuscript geteld.
search-tip-preserve-case = Bewaar geval: een vervanger neemt de gevonden zaak, dus "LENA" wordt "MARTA" en "Elena" wordt "Marta."
search-tip-replace = Vervangen: toon het vervangende veld en Vervang alles.
search-error = Zoeken mislukt: { $message }
search-no-matches = Geen overeenkomsten
search-count =
    { $matches ->
        [one] { $matches } overeenkomst
       *[other] { $matches } wedstrijden
    } in { $items ->
        [one] { $items } document
       *[other] { $items } documenten
    }
search-count-truncated =
    { $matches ->
        [one] { $matches } overeenkomst
       *[other] { $matches } wedstrijden
    } in { $items ->
        [one] { $items } document
       *[other] { $items } documenten
    } (alleen eerste resultaten)
search-occurrences = { $count }
search-field-body = Lichaam
search-field-title = Titel
search-field-synopsis = Synopsis
search-field-label = Korte beschrijving
search-field-epigraph = Motto
search-field-comment = Opmerking
search-field-comment-reply = Antwoord
search-field-footnote = Voetnoot
search-include-in-replace = Alles vervangen invoegen
search-collapse-all = Alle resultaten invouwen
search-replace-here = Vervang dit
search-dismiss = Ontslag uit de resultaten
search-undo-dismiss = Breng het laatste afgewezen resultaat terug
search-replace-nothing = (niets)
search-replace-confirm-title = Alle lucifers vervangen?
search-replace-confirm-text =
    Vervangen { $occurrences ->
        [one] { $occurrences } voorval
       *[other] { $occurrences } voorvallen
    } of "{ $query }" with "{ $replacement }" across { $items ->
        [one] { $items } document
       *[other] { $items } documenten
    } ? U kunt dit ongedaan maken uit de kennisgeving achteraf.
search-replace-done-title = Volledig vervangen
search-replace-done =
    { $occurrences ->
        [one] { $occurrences } voorval
       *[other] { $occurrences } voorvallen
    } replaced in { $items ->
        [one] { $items } document
       *[other] { $items } documenten
    } .
search-replace-done-skipped =
    { $occurrences ->
        [one] { $occurrences } voorval
       *[other] { $occurrences } voorvallen
    } replaced in { $items ->
        [one] { $items } document
       *[other] { $items } documenten
    }. { $skipped ->
        [one] { $skipped } veld
       *[other] { $skipped } velden
    } overgeslagen (veranderd sinds de zoektocht).
search-replace-undo = Ongedaan maken
search-replace-failed-title = Vervangen mislukt
search-replace-skipped-title = Niets vervangen
search-replace-skipped-body = Aantal teksten dat sinds de zoekopdracht is gewijzigd: { $fields }. Deze teksten zijn ongemoeid gelaten. Zoek opnieuw om te zien waar de woorden nu staan.
search-replace-undo-failed-title = Ongedaan maken mislukt
search-preview = Voorbeeld
search-preview-empty = Selecteer een resultaat om het hier te bekijken
search-preview-no-prose = Deze overeenkomst heeft geen bewerkbare tekst
search-preview-prompt = Om hier een voorbeeld te zien, zoek dan.
search-preview-open-search = Zoeken in project
search-preview-footnote-prompt = Deze match staat in de eigen tekst van een voetnoot. Open het in het dok Voetnoten om het te zien en te bewerken.
search-preview-open-footnotes = Voetnoten openen

find-placeholder = Zoek in dit document...
find-count = { $current } van { $total }
find-no-results = Geen resultaten
find-close = Dichtbij zoeken
find-previous = Vorige wedstrijd (Shift+Enter)
find-next = Volgende wedstrijd (Enter)
find-opt-case = Hoofdlettergevoelig
find-opt-whole-word = Het hele woord
find-replace-toggle = Vervangen aan/uit (Ctrl+R)
find-replace-placeholder = Vervangen door ...
find-replace = Vervangen
find-replace-all = Alles vervangen
find-preserve-case = Geval behouden

dict-download-title = Downloaden { $name }...
dict-download-done = In totaal { $name }
dict-download-failed = Kon niet downloaden { $name }: { $error }
dict-removed = Verwijderd { $name }
dict-accept-first = Accepteer de licentie voordat u downloadt { $name }
settings-dict-tab-installed = Geïnstalleerd
settings-dict-tab-get-more = Haal meer
settings-dict-tab-personal = Persoonlijke woorden
dict-installed-empty = Nog geen woordenboeken gevonden op deze computer.
dict-get-more-search = Talen zoeken
dict-system-badge = op uw systeem
dict-unusable-badge = onbruikbaar
dict-download-button = Downloaden
dict-downloading = Downloaden
dict-installed-label = Geïnstalleerd
dict-remove = Verwijderen
dict-view-license = Licentie weergeven
dict-approx-size = ~{ $size }
dict-personal-empty = Nog geen persoonlijke woorden in dit project.
dict-personal-add = Toevoegen
dict-personal-placeholder = Een woord toevoegen...
dict-missing-toast = { $count ->
    [one] Dit project gebruikt { $count } woordenboek dat u niet hebt geïnstalleerd
   *[other] Dit project gebruikt { $count } woordenboeken die u niet hebt geïnstalleerd
}
dict-missing-action = Woordenboeken ophalen
dict-license-title = { $name } licentie
dict-license-accept = Download accepteren
dict-license-cancel = Annuleren
dict-license-close = Sluiten
dict-add-button = Woordenboek toevoegen...
dict-add-title = Een woordenboek toevoegen
dict-add-close = Sluiten
dict-add-name = Naam
dict-add-name-placeholder = Bijvoorbeeld mijn Latijns woordenboek
dict-add-code = Taalcode
dict-add-code-placeholder = b.v. la of fr-FR-x-custom
dict-add-code-hint = Een korte tag van uw keuze. Dit is wat je kiest als de taal van een document.
dict-add-aff = Affix-bestand (.aff)
dict-add-dic = Woordenlijst (.dic)
dict-add-submit = Toevoegen
dict-add-cancel = Annuleren
dict-add-name-required = Geef het woordenboek een naam.
dict-add-code-required = Vul een taalcode in.
dict-add-code-invalid = Gebruik alleen letters, cijfers en - _ .
dict-add-code-reserved = Dat is een ingebouwde woordenboek code. Kies een andere.
dict-add-file-required = Kies een bestand.
dict-add-file-missing = Dat bestand bestaat niet.
dict-add-code-taken = Een woordenboek voor die code is al geïnstalleerd.
dict-add-done = Toegevoegd { $name }
dict-add-unusable = Deze bestanden zijn geen bruikbaar woordenboek: { $error }
dict-add-failed = Kon het woordenboek niet toevoegen: { $error }
inspector-export = Exporteren
inspector-exportable = Include in export
ctx-number = Dit hoofdstuk nummer (&N)
ctx-unnumber = Dit hoofdstuk nietnummeren (&N)
inspector-numbering = Nummering
inspector-numbered = Genummerd
inspector-numbered-tip = Dit hoofdstuk neemt zijn plaats in in de nummering van het boek. Schakel het uit voor een proloog, een epiloog of een interlude.
inspector-numbered-tip-more = Een niet-genummerd hoofdstuk blijft in het boek zoals het was: de titel, het proza en het woord tellen zijn onaangetast. Het drukt gewoon geen nummer af, en gebruikt er geen één op: het hoofdstuk na een proloog is hoofdstuk één, niet hoofdstuk twee. Het verlaten van het uit de export in plaats daarvan is een andere schakelaar, boven, en dat men verwijdert het hoofdstuk uit het boek helemaal.
inspector-apply-to-children = Toepassen op kinderen
inspector-milestone = Mijlpaaldatum
inspector-milestone-none = Geen datum
inspector-milestone-clear = Wissen
inspector-tags = Tags
inspector-aliases = Ook bekend als
inspector-dict-language = Taal
inspector-apply-language-to-children = Taal toepassen op kinderen
settings-page-language = Taal
settings-field-dict-language = Talen
dict-tradeoff-hint = Elke extra taal accepteert meer woorden, zodat er minder fouten worden gemaakt.
lang-inherit-hint = Erfelijk: deze scène gebruikt de talen van het boek of project.
lang-pill-list = Talen
lang-pill-add = Voeg een taal toe
lang-pill-remove = Verwijderen { $name }
lang-pill-mute = Zet spellingscontrole uit voor { $name }
lang-pill-unmute = Zet spellingscontrole aan voor { $name }

settings-page-personal-dictionary = Persoonlijk woordenboek
settings-user-dict-desc = Woorden die u hier toevoegt worden behandeld als correct gespeld overal in de app, en nooit meer gemarkeerd.
settings-user-dict-add-placeholder = Een woord toevoegen...
settings-user-dict-add = Woord toevoegen
settings-user-dict-search = Filter woorden
settings-user-dict-count = { $count ->
    [one] { $count } woord
   *[other] { $count } woorden
}
settings-user-dict-rename = Naam wijzigen
settings-user-dict-remove = Verwijderen
settings-user-dict-empty = Nog geen woorden. Voeg woorden toe die de spellingcontrole moet negeren, of importeer een lijst.
settings-user-dict-import = Importeren…
settings-user-dict-export = Exporteren…
settings-user-dict-import-tip = Voeg woorden toe uit een platte tekstbestand, één woord per regel. Bestaande woorden worden bewaard.
settings-user-dict-export-tip = Sla de hele lijst op in een platte tekstbestand, één woord per regel.
settings-user-dict-txt-filter = Tekstbestanden
settings-user-dict-duplicate = Al in het woordenboek
settings-user-dict-imported = Geïmporteerd { $count ->
    [one] { $count } woord
   *[other] { $count } woorden
} ({ $duplicates } reeds aanwezig).
settings-user-dict-import-failed = Kon de woordenlijst niet lezen: { $error }
settings-user-dict-exported = Opgeslagen { $count ->
    [one] { $count } woord.
   *[other] { $count } woorden.
}
settings-user-dict-export-failed = Kon de woordenlijst niet opslaan: { $error }

settings-page-text-replacements = Tekstvervangers
settings-text-repl-desc = Vervang steno door de volledige tekst als je typt: "btw" wordt "door de manier" op het moment dat je een spatie of interpunctie typt.
settings-text-repl-enable = Tekstvervangingen gebruiken in dit project
settings-text-repl-disabled-hint = Zet dit aan om steno te definiëren die uitbreidt terwijl je schrijft.
settings-text-repl-add = Regel toevoegen
settings-text-repl-trigger-placeholder = Afkorting
settings-text-repl-replacement-placeholder = Wat het wordt
settings-text-repl-added = Toegevoegd "{ $trigger }"
settings-text-repl-duplicate = "{ $trigger }" heeft al een regel
settings-text-repl-filter = Filterregels
settings-text-repl-count = { $n ->
    [one] 1 regel
   *[other] { $n } regels
}
settings-text-repl-row-enabled = Deze regel gebruiken
settings-text-repl-delete = De regel voor { $trigger } schrappen
settings-text-repl-deleted = De regel voor "{ $trigger }" is geschrapt
settings-text-repl-empty = Nog geen regels.
settings-text-repl-csv-filter = CSV bestanden
settings-text-repl-import = Importeren…
settings-text-repl-export = Exporteren…
settings-text-repl-imported = Geïmporteerd { $added }, overgeslagen { $skipped }
settings-text-repl-exported = Exporteren { $n ->
    [one] 1 regel
   *[other] { $n } regels
}

editor-menu-no-suggestions = Geen suggesties
editor-menu-add-to-dictionary = "{ $word }" toevoegen aan woordenboek
editor-menu-add-words-to-dictionary = Geselecteerde woorden toevoegen aan woordenboek
editor-dict-added = Toegevoegd "{ $word }" aan uw woordenboek.
editor-dict-added-multi = { $count ->
    [one] Toegevoegd { $count } woord aan uw woordenboek.
   *[other] Toegevoegd { $count } woorden aan uw woordenboek.
}
toast-undo = Ongedaan maken

undo-superseded-title = Die stap is niet langer de laatste.
undo-superseded-body = Iets anders is veranderd in dit project sinds. Ongedaan maken zou in plaats daarvan terug te nemen, dus het is niet. Niets van deze stap is teruggedraaid. Gebruik Bewerken Ongedaan maken om terug te stappen door de geschiedenis zelf.
undo-failed = Ongedaan maken mislukt: { $error }

menu-edit = Bewerken (&E)
menu-edit-undo = Ongedaan maken (&U)
menu-edit-redo = Opnieuw uitvoeren (&R)
menu-edit-undo-target = Ongedaan maken { $target } (&U)
menu-edit-redo-target = Opnieuw { $target } (&R)
shortcut-name-edit-undo = Ongedaan maken
shortcut-name-edit-redo = Opnieuw uitvoeren
undo-target-typing = typen
undo-target-project = de laatste wijziging van dit project
prose-history-reset-title = Geschiedenis opnieuw instellen
prose-history-reset-body = { $count ->
    [one] Een open scène werd hersteld uit een eerdere staat, dus de typegeschiedenis is niet langer van toepassing.
   *[other] { $count } open scènes werden hersteld uit een eerdere staat, zodat hun typegeschiedenis niet langer van toepassing is.
}
undo-target-trash = verplaatsen naar de prullenbak
undo-target-restore = herstellen uit de prullenbak
undo-target-delete-forever = verwijderen voorgoed
undo-target-replace-all = Vervangen door het hele project
undo-target-import = het document importeren
undo-target-duplicate = dupliceren
undo-target-move = bewegend
undo-target-merge = twee scènes samenvoegen
undo-target-split = een scène splitsen
undo-target-promote = het type wijzigen
undo-target-tidy-titles = het opruimen van de hoofdstuktitels
undo-target-import-tags = labels importeren
undo-target-import-templates = note templates importeren
undo-target-create = aanmaken
undo-target-remove = verwijderen
undo-target-rename = hernoemen
undo-target-edit = die bewerken
menu-edit-find = Zoeken… (&F)
menu-edit-find-next = Volgende zoeken (&N)
menu-edit-find-prev = Vorige zoeken (&V)
menu-edit-replace = Zoeken en vervangen... (&L)
menu-edit-find-in-project = Zoeken in project... (&J)
menu-edit-replace-in-project = In project vervangen... (&I)
undo-frozen = Ongedaan maken ("Always forward" is on)
redo-frozen = Opnieuw ("Always forward" is aan)
menu-edit-cut = Knippen (&T)
menu-edit-copy = Kopiëren (&C)
menu-edit-paste = Plakken (&P)
menu-edit-paste-plain = Plakken zonder opmaak (&W)
menu-edit-select-all = Alles selecteren (&A)
shortcut-name-edit-cut = Knippen
shortcut-name-edit-copy = Kopiëren
shortcut-name-edit-paste = Plakken
shortcut-name-edit-paste-plain = Plakken zonder opmaak
shortcut-name-edit-select-all = Alles selecteren

titlebar-spellcheck-on = Spellingscontrole is ingeschakeld. Klik om te stoppen met controleren (F7)
titlebar-spellcheck-off = Spellingscontrole is uit. Klik om opnieuw te controleren (F7)
menu-spellcheck = Spelling controleren (&C)
menu-comments = Opmerkingen (&M)
settings-page-spellcheck = Spellingscontrole
settings-group-spellcheck = Spellingscontrole
settings-spellcheck-enabled = Spelling controleren terwijl ik schrijf
settings-spellcheck-hint = Onderlijnen woorden geen geïnstalleerd woordenboek weet. Het uitschakelen van deze uit stopt alle controle, in elk project, totdat u het weer aan. Om te stoppen met het controleren van een taal, wissen van de check-mark op het taalveld van het project of van een item.

menu-trash = Prullenbak (&T)
trash-title = Prullenbak
trash-empty-state = De vuilnisbak is leeg.
trash-empty-button = Lege prullenbak...
trash-restore = Herstellen (&R)
trash-restore-to = Terugzetten naar... (&T)
trash-delete-forever = Voor altijd verwijderen (&D)
trash-restored-ok = { $count } gerestaureerd.
trash-restore-error = Kon niet herstellen: { $error }
trash-restore-orphaned = De originele plek van dit item is weg. Kies waar het moet worden hersteld.
trash-restore-no-project = Er is geen project geopend, dus er valt niets te herstellen.
trash-delete-no-project = Er is geen project geopend, dus er valt niets te verwijderen.
trash-empty-confirm-title = Het vuilnis legen?
trash-empty-confirm-text = { $count ->
    [one] { $count } gesloopte ingang wordt definitief verwijderd.
   *[other] { $count } puinhopen worden definitief verwijderd.
} Ze laten het afval voorgoed achter. Ongedaan maken kan ze nog terugbrengen tot je iets anders doet.
trash-emptied-title = Prullenbak geleegd
trash-emptied-body = Alles in de prullenbak is definitief verwijderd.
trash-empty-no-project = Er is geen project geopend, dus er is geen prullenbak te leeg.
trash-delete-forever-confirm-title = Voor altijd verwijderen?
trash-delete-forever-confirm-text = { $count ->
    [one] { $count } wordt definitief verwijderd.
   *[other] { $count } items worden definitief verwijderd.
} Ze laten het afval voorgoed achter. Ongedaan maken kan ze nog terugbrengen tot je iets anders doet.
trash-deleted-title = Voor altijd verwijderd
trash-deleted-body = { $count } definitief geschrapt.
trash-undo = Ongedaan maken
trash-restore-picker-title = Terugzetten naar...
trash-restore-to-confirm-title = Hier herstellen?
trash-restore-to-confirm-text = "{ $item }" terugzetten in "{ $destination }"?
trash-restore-picker-restore-here = Hier herstellen
trash-restore-picker-cancel = Annuleren
trash-restore-picker-empty = Maak er eerst een.
trash-banner-title = Dit item staat in de prullenbak
trash-banner-description = Het zal niet verschijnen in de omtrek of export totdat u het herstelt.
trash-banner-restore = Herstellen…
trash-tab-tooltip = In prullenbak

settings-styles-sheet-yes = Ja
settings-styles-sheet-no = Nee
settings-styles-sheet-font = Lettertype
settings-styles-sheet-indent = Eerste regel streepje
settings-styles-sheet-para-spacing = Paragraafafstand
settings-styles-sheet-page = Paginagrootte
settings-styles-sheet-margins = Marges (T/R/B/L)
settings-styles-sheet-title-page = Titelpagina van het boek
settings-styles-sheet-heading-language = Rubriektaal
settings-styles-sheet-auto = Volgt de tekst
settings-styles-sheet-digits = Cijfers
settings-styles-sheet-direction = Tekstrichting
settings-styles-sheet-formats = Formaten
settings-styles-sheet-all-formats = Alles

settings-styles-editor-group = Stijlbewerker
settings-styles-editor-missing = Die stijl is niet meer beschikbaar.

settings-styles-page-letter = Brief
settings-styles-digits-western = Westers (0
settings-styles-digits-eastern-arabic = Oost-Arabisch
settings-styles-direction-ltr = Links naar rechts
settings-styles-direction-rtl = Rechts naar links

format-dock-title = Opmaak
format-panel-empty = Plaats uw cursor in een scène, notitie of synopsis om opmaakopties te zien.
format-group-marks = Tekst
format-group-block = Paragraaf
format-group-lists = Lijsten
format-group-tables = Tabel
format-group-breaks = Scènebreuken
format-superscript = Superscript
format-subscript = Subscript
format-link = Link...
link-dialog-insert-title = Verwijzing invoegen
link-dialog-edit-title = Verwijzing bewerken
link-dialog-text-label = Tekst
link-dialog-text-placeholder = de woorden die de lezer ziet
link-dialog-url-label = Links naar
link-dialog-url-placeholder = example.com
link-dialog-insert = Invoegen
link-dialog-apply = Toepassen
link-dialog-cancel = Annuleren
link-dialog-remove = Verwijzing verwijderen
link-scheme-refused = Alleen web- en e-maillinks kunnen worden geopend. { $url } werd met rust gelaten.
menu-format-link = Link... (&N)
format-clear = Opmaak wissen
format-heading = Omschrijving
format-heading-normal = Normale tekst
format-heading-1 = Rubriek 1
format-heading-2 = Kop 2
format-heading-3 = Rubriek 3
format-heading-4 = Rubriek 4
format-heading-5 = Rubriek 5
format-heading-6 = Rubriek 6
format-align-left = Links uitlijnen
format-align-center = Midden
format-direction-rtl = Paragraaf rechts naar links
format-blockquote = Blokcitaat
format-list-bullet = Gestichte lijst
format-list-numbered = Genummerde lijst
format-indent = Inspringen
format-outdent = Uitspringen
format-table-insert = Tabel invoegen
format-table-row-above = Rij invoegen boven
format-table-row-below = rij hieronder invoegen
format-table-col-before = Kolom invoegen voor
format-table-col-after = Kolom invoegen na
format-table-row-delete = Rij verwijderen
format-table-col-delete = Kolom verwijderen
format-table-remove = Tabel verwijderen

menu-format-marks-bold = Vet (&D)
menu-format-marks-italic = Cursief (&I)
menu-format-marks-underline = Onderstrepen (&U)
menu-format-marks-strike = Doorgestreept (&S)
menu-format-marks-superscript = Superscript (&P)
menu-format-marks-subscript = Subscript (&B)
menu-format-marks-clear = Opmaak wissen (&C)
menu-format-heading = Omschrijving (&H)
menu-format-heading-normal = Normale tekst (&N)
menu-format-heading-1 = Rubriek 1 (&1)
menu-format-heading-2 = Kop 2 (&2)
menu-format-heading-3 = Rubriek 3 (&3)
menu-format-heading-4 = Rubriek 4 (&4)
menu-format-heading-5 = Rubriek 5 (&5)
menu-format-heading-6 = Rubriek 6 (&6)
menu-format-alignment = Uitlijning (&A)
menu-format-align-left = Links uitlijnen (&L)
menu-format-align-center = Midden (&C)
menu-format-direction = Richting (&E)
menu-format-direction-auto = Automatisch (&A)
menu-format-direction-ltr = Links naar rechts (&L)
menu-format-direction-rtl = Rechts naar links (&R)
menu-format-blockquote = Blokcitaat (&Q)
menu-format-lists = Lijsten (&L)
menu-format-list-bullet = Gebullette lijst (&B)
menu-format-list-numbered = Lijst met nummers (&N)
menu-format-indent = Inspringen (&I)
menu-format-outdent = Uitspringen (&O)
menu-format-table = Tabel (&T)
menu-format-table-insert = Tabel invoegen (&I)
menu-format-table-2x2 = 2 x 2 (&2)
menu-format-table-3x3 = 3 x 3 (&3)
menu-format-table-4x4 = 4 x 4 (&4)
menu-format-table-row-above = Rij hierboven invoegen (&V)
menu-format-table-row-below = Rij hieronder invoegen (&W)
menu-format-table-col-before = Kolom invoegen voor (&L)
menu-format-table-col-after = Kolom invoegen na (&T)
menu-format-table-row-delete = Rij verwijderen (&D)
menu-format-table-col-delete = Kolom verwijderen (&C)
menu-format-table-remove = Tabel verwijderen (&R)

menu-format-typo-marks = Opmaakmarkering (&F)
menu-format-typo-nbsp = No-break space (&N)
menu-format-typo-nbhyphen = Geen-breuk Hyphen (&H)
menu-format-typo-soft-hyphen = Soft Hyphen (&S)
menu-format-typo-nnbsp = Narrow no-break space (&R)
menu-format-typo-zwsp = Nulbreedteruimte (&Z)
menu-format-typo-word-joiner = Woordaanhanger (&W)


menu-about = Ongeveer Skribisto... (&A)
about-title = Ongeveer Skribisto
about-version = Versie { $version }
about-update-available = Versie { $version } is beschikbaar
about-update-see-notes = Zien wat er veranderd is
about-update-flathub = Flathub houdt dit exemplaar up-to-date.
about-update-distro = Uw distributie houdt dit exemplaar up-to-date.
about-tagline = Een roman-writing app voor lange-vorm fictie, geschreven in Rust met de Teksilo toolkit.
about-license = Vrijgegeven onder de GNU General Public License, versie 3.
about-copyright = © 2026 Cyril Jacquet
about-close = Sluiten

native-menu-about = Ongeveer { $app }
native-menu-hide = Verbergen { $app }
native-menu-quit = Afsluiten { $app }
native-menu-settings = Instellingen…
native-menu-window = Venster
native-menu-minimize = Minimaliseren
native-menu-zoom = Zoomen

window-title = { $title } - { $app }
window-title-numbered = { $title } - { $app } (Venster { $n })
window-title-empty = { $app }

first-run-window-title = Welkomstwoord
first-run-title = Instellen { $app }
first-run-body = { $app } houdt de instellingen apart van Skribisto, dus het begint leeg. Uw voorkeuren, recente projecten, woordenboeken en vensterindeling kunnen nu worden gekopieerd.
first-run-from = Kopiëren van
first-run-to = Kopiëren naar
first-run-copy-note = Niets wordt verplaatst of verwijderd. Skribisto behoudt elk van zijn eigen instellingen, en gaat verder werken precies zoals voorheen.
first-run-import = Instellingen importeren
first-run-start-fresh = Hernieuwd starten
first-run-import-failed = Sommige instellingen konden niet geïmporteerd worden: { $error }

settings-page-punctuation = Punctuatie-standaarden
settings-group-punctuation = Slimme interpunctie
settings-page-work-punctuation = Punctuatie
settings-punctuation-override = Geef dit project zijn eigen leesregels
settings-punctuation-override-hint = Uit: het project volgt de voorkeur voor de toepassing. Deze regels gaan binnen de .skrib, dus een co-auteur die het bestand opent schrijft met dezelfde typografie.
settings-punctuation-dashes = Draai -- in een en dashboard, --- in een em dash
settings-punctuation-ellipsis = Verander... in een ellipse.
settings-punctuation-quotes = Krulaanhalingstekens en apostrofen
settings-quote-style = Noterings
settings-quote-style-locale = Taalstandaard
settings-quote-style-curly = "Dubbel"
settings-quote-style-curly-single = Single"
settings-quote-style-guillemets = Guillemets.
settings-quote-style-low-high = Laag-hoog"
settings-punctuation-spacing = Ruimte voor;: ! ?
settings-punctuation-spacing-hint = Franse typografie stelt een smalle no-break ruimte voor; ! ? en een volledige voor:. Het is alleen van toepassing op tekst geschreven in het Frans. De ruimte binnen guillemets wordt geleverd met de guillemet citaten zelf.
settings-punctuation-sample = Uw taal geeft
settings-punctuation-app-hint = Wat elk project doet tenzij het neemt regels van zijn eigen in Project Punctuation.
settings-punctuation-dialogue = Open een alinea getypt als "- " met een dialoogdash
settings-punctuation-dialogue-hint = Voor talen die spreken met een streepje in plaats van aanhalingstekens: Frans, Spaans, Russisch en anderen. Het vuurt alleen bij het begin van een paragraaf.

statusbar-go-to = Ga...
go-to-placeholder = Doorzoek de map
go-to-no-matches = Geen item komt overeen met die zoekopdracht.
menu-go-to = Ga... (&G)

settings-page-distraction-free-themes = Afleidingsvrije thema's
settings-themes-builtin = Ingebouwde thema's
settings-themes-builtin-badge = Ingebouwd
settings-themes-user = Mijn thema's
settings-themes-editor-group = Thema bewerken
settings-themes-editor-empty = Kies een thema onder "Mijn thema's" om het te bewerken.
settings-themes-use = Gebruik
settings-themes-duplicate = Dupliceren
settings-themes-edit = Bewerken
settings-themes-delete = Verwijderen
settings-themes-export = Exporteren…
settings-themes-import = Importeren…
settings-themes-copy-suffix = Kopiëren
settings-themes-json-filter = Thema (JSON)
settings-themes-imported = Thema geïmporteerd
settings-themes-import-failed = Kon dat thema niet importeren
settings-themes-exported = Thema geëxporteerd
settings-themes-export-failed = Kon dat thema niet exporteren
settings-themes-low-contrast = laag contrast
settings-themes-low-contrast-band = verbergt de tekst
settings-themes-field-name = Naam
settings-themes-field-paper = Pagina
settings-themes-field-ink = Tekst
settings-themes-field-general = Achtergrond
settings-themes-field-widget-text = Tekst regelstrook
settings-themes-field-caret-band = Oplichten rond de caret

statusbar-focus-settings = Afleidingsvrije instellingen
statusbar-focus-manage-themes = Themes beheren...

comments-title = Opmerkingen
comments-document-title = Dit document
comments-empty-project = Nog geen commentaar in dit project.
comments-empty-document = Geen opmerkingen over dit document.
comments-filter-all = Alles
comments-filter-open = Openen
comments-filter-resolved = Opgelost
comments-filter-orphaned = Wees
comments-status-open = Openen
comments-status-resolved = Opgelost
comments-status-orphaned = Verloren tekst
comments-orphan-snippet = (de commentaartekst is weg)
comments-status-unplaced = Geen tekstpositie
comments-unplaced-snippet = (niet verankerd aan een tekst)
comments-reply-count = { $count ->
    [one] { $count } antwoord
   *[other] { $count } antwoorden
}
comments-menu-resolve = Oplossen
comments-menu-reopen = Heropenen
comments-menu-delete = Verwijderen
comments-sort-document = In volgorde van documenten
comments-sort-newest = Eerste nieuwste
comments-sort-reader = Groeperen per lezer
comments-unsigned-author = Niet ondertekend
comments-agreement = { $count ->
    [one] { $count } lezer
   *[other] { $count } lezers
}
comments-menu-add = Commentaar toevoegen
comments-menu-add-paragraph = Toelichting bij deze paragraaf
overview-col-comments = Opmerkingen
overview-col-total-comments = Totaal commentaar
comments-card-placeholder = Schrijf een commentaar...
comments-card-unknown-author = Onbekende auteur
comments-unsigned-toast = Uw opmerkingen zijn niet ondertekend: er is geen naam voor u ingesteld op deze computer.
comments-unsigned-action = Uw naam instellen
comments-card-reply = Antwoord
comments-card-reply-placeholder = Antwoord...
comments-card-actions = Opmerkingen
comments-reply-actions = Maatregelen beantwoorden
comments-menu-delete-reply = Antwoord schrappen
comments-menu-delete-all = Verwijder alle opmerkingen hier
comments-deleted-toast = Commentaar verwijderd
comments-reply-deleted-toast = Verwijderd beantwoorden
comments-deleted-all-toast = { $count ->
    [one] { $count } opmerking geschrapt
   *[other] { $count } opmerkingen geschrapt
}
comments-undo = Ongedaan maken

analysis-segment = Analyse
analysis-scope-book = Analyseren van dit boek
analysis-run = Analyse uitvoeren
analysis-stale = Veranderd sinds dit liep
analysis-not-run = Nog niet geanalyseerd.
analysis-running = Het manuscript lezen...
analysis-failed = De analyse kon niet worden afgerond.
analysis-no-scenes = Nog geen scènes in dit boek.

analysis-shape = Vorm

analysis-arrivals = Aankomsten
analysis-arrivals-explainer = Hoe tekst bereikte dit project terwijl het open was. Er staat welke route de karakters naar beneden kwamen, en helemaal niets over wie ze schreef: een schrijver die elders ontwerpt en pasta's plakt, en iemand die dicteert heeft gedicteerd. Er is geen aantal om naar te streven hier.
analysis-arrivals-scope = Dit hele project, niet alleen dit boek.
analysis-arrivals-session = Het project sluiten begint weer met tellen.
analysis-arrivals-nothing = Er is nog geen tekst in deze sessie aangekomen.
analysis-arrivals-typed = Getypt
analysis-arrivals-pasted = geplakt
analysis-arrivals-dictated = Gedifferentieerd
analysis-arrivals-imported = Geïmporteerd
analysis-arrivals-programmatic = Voor u invoegen
analysis-arrivals-count = { $count ->
    [one] { $count } teken
   *[other] { $count } tekens
}
analysis-arrivals-none = Geen

analysis-words-per-scene = Woorden per scène
analysis-median-words = De mediane scene van dit boek loopt { $count ->
    [one] { $count } woord.
   *[other] { $count } woorden.
}
analysis-median-line = Mediaan: { $count ->
    [one] { $count } woord
   *[other] { $count } woorden
}
analysis-dialogue = Dialoog
analysis-dialogue-unsupported = De dialoog wordt nog niet gemeten voor deze taal.

analysis-footnote-words = Voetnootwoorden
analysis-footnote-words-count = { $count ->
    [one] { $count } woord staat in de voetnoten van dit boek, gescheiden gehouden van het manuscript totaal.
   *[other] { $count } woorden staan in de voetnoten van dit boek, gescheiden gehouden van het manuscript totaal.
}
analysis-footnote-words-pending = De voetnoten tellen...

analysis-ignore-empty = Teksten met nog niets geschreven negeren
analysis-empty-hidden = { $count } leeg { $count ->
        [one] Tekst
       *[other] teksten
    } Verborgen.
analysis-all-texts-empty = Elke tekst in dit boek is nog steeds leeg.

binder-filter-count = { $shown } van { $total } getoond
binder-filter-clear = Wissen
binder-filter-none = Niets komt overeen met "{ $query }."

settings-paratext-intro = De voor- en achterkant van een nieuw project kan beginnen met. Elke structuur behoort tot een uitgeverstraditie, en de paginatitels zijn geschreven in de eigen taal van die traditie. Hernoem ze vrij zodra een project is gemaakt.
settings-paratext-structures = Structuur
settings-paratext-broken = Kan niet gelezen worden
settings-paratext-edit = Bewerken
settings-paratext-duplicate = Dupliceren
settings-paratext-delete = Verwijderen
settings-paratext-new = Nieuwe structuur
settings-paratext-save = Opslaan
settings-paratext-editor-hint = Een naam, de pagina's die gemaakt moeten worden voor het manuscript, en de pagina's die erna gemaakt moeten worden. Waar ze eindigen is de jouwe: ze zijn gewone items zodra het project bestaat.


image-insert = Afbeelding toevoegen... (&M)
image-no-project = Een project openen voordat u een afbeelding invoegt.
image-choose-title = Een afbeelding kiezen
image-filter-label = Afbeeldingen
image-large-title = Dit is een grote afbeelding
image-large-text =
    { $name } is { $megapixels } megapixels ({ $width }×{ $height }).
    Houden het zoals het is slaat uw oorspronkelijke bestand in het project: het reist
    bij elke back-up en elke export. Optimising slaat een kleinere kopie op
    In plaats daarvan, tot 2560 pixels op zijn langste kant.
image-large-keep = Origineel bewaren
image-large-downscale = Optimaliseren
image-large-remember = Doe dit vanaf nu, vraag het niet meer.
image-not-recorded = De afbeelding is opgeslagen maar kon niet worden opgenomen in het project.
image-describe-title = Beschrijf de afbeelding
image-describe-explain = Wat de afbeelding laat zien, voor een lezer die het niet kan zien. Het maakt geen deel uit van het manuscript: het wordt nooit geteld, doorzocht of geëxporteerd als proza.
image-describe-placeholder = een vuurtoren tegen een grijze lucht
image-resize-title = De afbeelding wijzigen
image-resize-explain = Een percentage van de grootte wordt nu weergegeven. 100 laat het zoals het is.
image-resize-invalid = Voer een getal in tussen 1 en 1000.
image-menu-describe = Beschrijf het beeld... (&D)
image-menu-resize = Grootte van afbeelding wijzigen... (&R)
image-menu-reset-size = Oorspronkelijke grootte (&Z)

cover-choose = Boekenhoes... (&V)
cover-clear = Verwijder de hoes (&R)
cover-choose-title = Kies een cover
cover-set = De dekking is ingesteld.
cover-cleared = De hoes is verwijderd, de foto is nog in het project.
card-label = Korte beschrijving
card-exportable = Exporteren
card-numbered = Genummerd
card-created = Aangemaakt
card-modified = Gewijzigd
card-yes = Ja
card-no = Nee
card-type = Type
card-position = Positie
card-children = Kinderen
card-goal = Doel
card-words = Woorden
card-synopsis = Synopsis
card-point-of-view = Perspectief
card-aliases = Ook bekend als

export-orphan-footnotes-title = Voetnoten zonder referentie
export-comments-dropped =
    { $count ->
        [one] Eén opmerking kon niet in de geëxporteerde tekst worden geplaatst en werd weggelaten.
       *[other] { $count } opmerkingen konden niet in de geëxporteerde tekst worden geplaatst en werden weggelaten.
    }
export-orphan-footnotes =
    { $count ->
        [one] Een voetnoot wordt nergens meer in het manuscript genoemd. De tekst ervan zal niet in het geëxporteerde boek verschijnen.
       *[other] { $count } voetnoten worden nergens meer in het manuscript vermeld. Hun tekst zal niet in het geëxporteerde boek verschijnen.
    }


menu-footnotes = Voetnoten (&N)
footnotes-title = Voetnoten
footnotes-insert = voetnoot invoegen (&F)
footnotes-empty = Nog geen voetnoten. Zet de cursor in een scène, gebruik dan + hierboven, of Ctrl+Alt+F.
footnotes-filter-all = Alles
footnotes-filter-document = Dit document
footnotes-filter-orphaned = Wees
footnotes-orphaned = Niets wijst meer op deze noot.
footnotes-untitled-home = Naamloos
footnotes-body-placeholder = de noot zelf
footnotes-insert-tooltip = Voeg een voetnoot toe aan de cursor (Ctrl+Alt+F)
footnotes-actions = Voetnootacties
footnotes-delete = Notitie en verwijzing schrappen (&D)
footnotes-no-project = Een project openen alvorens een voetnoot in te voegen.
footnotes-no-caret = Zet de cursor in de tekst van een scène om daar een voetnoot in te voegen.
footnotes-not-created = De voetnoot kon niet aan het project worden toegevoegd.
footnotes-deleted-toast = Notities verwijderd
footnotes-undo-delete = Ongedaan maken

import-document-title = Invoerdocumenten
import-document-close = Sluiten
import-document-step-files = Bestanden
import-document-step-review = Evaluatie
import-document-step-destination = Bestemming
import-document-step-reconcile = Samenvoegen
import-document-reconcile-hint = Sommige van deze items zijn degenen die je al hebt. Zeg wat er met elk moet gebeuren.
import-document-reconcile-all-new = Niets in dit bestand komt overeen met uw project: elk item wordt als nieuw toegevoegd.
import-document-duplicate-returns = Deze bestanden zijn meer dan één kopie van hetzelfde manuscript dat terugkomt. Importeer ze één voor één: het vergelijken van één kopie met het project, zodat het importeren van meerdere exemplaren samen een tweede kopie van het boek zou toevoegen in plaats van het samen te voegen.
import-document-hunk-added = Neem deze nieuwe paragraaf
import-document-hunk-removed = Deze paragraaf laten vallen
import-document-hunk-changed = Neem deze herschrijven
import-document-col-stray-prose = De tekst
import-document-stray-as-paratext = Als paratekst bewaren
import-document-stray-discard = Tekst laten vallen
import-document-col-current = In uw project
import-document-col-incoming = In dit bestand
import-document-col-status = Status
import-document-col-action = Wat te doen?
import-document-status-identical = Hetzelfde
import-document-status-editor-edited = Ze hebben het bewerkt.
import-document-status-you-edited = Je hebt het bewerkt.
import-document-status-conflict = Beiden bewerkt het
import-document-status-different = Verschillen
import-document-status-new = Nieuw
import-document-status-missing = Niet in dit bestand
import-document-status-moved = Verplaatst
import-document-action-comments-only = Uitsluitend opmerkingen
import-document-action-take-import = Neem deze versie
import-document-action-keep-current = Hou de mijne maar.
import-document-action-create-new = Als nieuw toevoegen
import-document-action-ignore = Overslaan
import-document-compare = Vergelijken
import-document-compare-legend = Uw versie vergeleken met dit bestand. Alleen-lezen.
import-document-compare-close = Sluiten
import-document-drop-title = Documenten hier neerzetten
import-document-drop-hint = Markdown (.md), Woord (.docx), OpenDocument (.odt) en platte tekst (.txt)
import-document-browse = Bladeren...
import-document-move-up = Omhoog verplaatsen
import-document-move-down = Omlaag verplaatsen
import-document-remove-file = Verwijderen
import-document-file-count = { $count ->
    [one] 1 bestand
   *[other] { $count } bestanden
}
import-document-no-files = Nog geen bestanden geselecteerd.
import-document-col-included = Importeren
import-document-col-title = Titel
import-document-col-type = Type
import-document-col-words = Woorden
import-document-col-breaks = Pauzes
import-document-col-comments = Opmerkingen
import-document-col-footnotes = Voetnoten
import-document-col-epigraph = Motto
import-document-col-source = Bron
import-document-level-rules = Rubrieken
import-document-level-n = Rubriek { $level }
import-document-add-top-level = Bovenste niveau toevoegen
import-document-add-top-level-tooltip = Een boek boven elk geanalyseerd item invoegen, voor hoofdstukbestanden zonder boektitel
import-document-destination = Bestemming
import-document-destination-hint = Kies een map of item: nieuwe items landen in een map, of na een scène.
import-document-destination-empty = Maak er eerst een.
import-document-plan-empty = Nog niets te importeren.
import-document-summary = { $rows ->
    [one] 1 item
   *[other] { $rows } posten
} · { $breaks ->
    [one] 1 scène pauze
   *[other] { $breaks } scène pauzes
}
import-document-back = Terug
import-document-cancel = Annuleren
import-document-analyse = Volgende
import-document-analysing = Documenten lezen...
import-document-step-analysing = Lezen
import-document-cancel-analysis = Stop met lezen
import-document-analyse-failed = De documenten konden niet worden gelezen.
import-document-details = Details
import-document-import = Importeren
import-document-done = { $count ->
    [one] 1 ingevoerd product
   *[other] { $count } ingevoerde producten
}
import-document-undo = Ongedaan maken
import-diagnostic-file-unreadable = "{ $path }" kon niet gelezen worden: { $detail }. De andere bestanden importeren nog steeds.
import-diagnostic-nested-too-deep = "{ $path }" werd niet gelezen: de inhoud ervan is meer dan { $limit } niveaus diep genesteld. Geen echt document komt in de buurt dat, en het lezen kan crashen Skribisto, dus het wordt weggelaten. De andere bestanden nog steeds importeren.
import-diagnostic-archive-too-large = "{ $path }" werd niet gelezen: het is een gecomprimeerd bestand gebouwd om veel groter uit te pakken dan het is, die zou kunnen uitputten geheugen en crash Skribisto, dus het wordt weggelaten. De andere bestanden nog steeds importeren.
import-diagnostic-lossy-decode = { $count ->
    [one] Eén teken in "{ $path }" decodeerde niet. Het bestand opnieuw opslaan als UTF-8 om het te bewaren.
   *[other] { $count } tekens in "{ $path }" niet gedecodeerd. Het bestand opnieuw opslaan als UTF-8 om ze te behouden.
}
import-diagnostic-decoded-from-bom = "{ $path }" werd gedecodeerd als { $detail }, niet UTF-8.
import-diagnostic-empty-file = "{ $path }" is leeg.
import-diagnostic-no-headings = "{ $path }" heeft geen rubrieken, dus het komt als één item.
import-diagnostic-unsupported-format = Niets leest ".{ $detail }" bestanden, dus "{ $path }" werd overgeslagen.
import-diagnostic-front-matter-not-flat = Voorgrond in "{ $path }": "{ $detail }" is geen eenvoudige waarde en werd overgeslagen.
import-diagnostic-footnotes-degraded = { $count ->
    [one] Een voetnoot in "{ $path }" komt als gewone tekst: voetnoten worden niet gelezen uit Markdown.
   *[other] { $count } voetnoten in "{ $path }" komen als gewone tekst: voetnoten worden niet gelezen uit Markdown.
}
import-diagnostic-footnote-not-carried = { $count ->
    [one] Een voetnoot in "{ $path }" kon niet worden overgenomen, de andere wel.
   *[other] { $count } voetnoten in "{ $path }" konden niet worden aangebracht.
}
import-diagnostic-raw-html-dropped = { $count ->
    [one] Een blok ruwe HTML in "{ $path }" werd gedropt.
   *[other] { $count } blokken ruwe HTML in "{ $path }" werden gedropt.
}
import-diagnostic-nested-break-dropped = { $count ->
    [one] Een scènebreuk in een citaat of lijst in "{ $path }" werd gedropt. Alleen een breuk op zijn eigen lijn wordt gehouden.
   *[other] { $count } scène breekt binnen citaten of lijsten in "{ $path }" werden geschrapt. Alleen een breuk op zijn eigen lijn wordt gehouden.
}
import-diagnostic-image-not-ingested = "{ $path }" verwijst naar de afbeelding "{ $detail }." De referentie komt als tekst; de afbeelding zelf wordt niet gekopieerd.
import-diagnostic-duplicate-title = "{ $title }" verschijnt { $count } keer. Als u deze bestanden eerder hebt geïmporteerd, zal dit ze dupliceren.
import-diagnostic-heading-level-jump = "{ $title }" springt van koersniveau { $from } naar { $to }; het wordt één niveau onder de ouder geplaatst.
import-diagnostic-illegal-combination = "{ $title }" draagt proza, maar een { $kind } kan er geen houden. Importeren wordt gehouden totdat u van type verandert of het buiten laat. Niets wordt anders geïmporteerd.
import-diagnostic-tracked-changes-flattened = { $path } was halverwege de herziening: { $count ->
    [one] { $count } traced change is geaccepteerd
   *[other] { $count } traced changes werden geaccepteerd
} , en verwijderingen vallen. Dat is de laatste tekst. Om een van uw eigen formulering in plaats daarvan te houden, gebruik Vergelijk in de laatste stap.
import-diagnostic-tracked-changes-flattened-by = { $path } was halverwege de herziening: { $count ->
    [one] { $count } traced change met { $names } werd geaccepteerd
   *[other] { $count } gevolgde wijzigingen door { $names } werden geaccepteerd
} , en verwijderingen vallen. Dat is de laatste tekst. Om een van uw eigen formulering in plaats daarvan te houden, gebruik Vergelijk in de laatste stap.
import-diagnostic-text-box-dropped = { $count ->
    [one] { $path } bevat { $count } tekstvak. De tekst ervan zit buiten de flow van het document, zodat waar het behoort in een manuscript niet kan worden beantwoord. Het wordt niet geïmporteerd.
   *[other] { $path } bevat { $count } tekstvakken. Hun tekst bevindt zich buiten de stroming van het document, zodat waar het in een manuscript thuishoort, niet beantwoord kan worden. Ze worden niet geïmporteerd.
}
import-diagnostic-embedded-object-dropped = { $count ->
    [one] { $path } bevat { $count } ingebed object: een grafiek, een vergelijking of soortgelijke. Er is niets in een manuscript dat het kon houden.
   *[other] { $path } bevat { $count } embedded objecten: een grafiek, een vergelijking of soortgelijke. Er is niets in een manuscript dat ze zou kunnen houden.
}
import-diagnostic-field-flattened = { $count ->
    [one] { $path } bevat { $count } veld: een paginanummer, een kruisverwijzing, een datum. Het houdt de tekst die het laatst werd weergegeven en zal niet opnieuw worden bijgewerkt.
   *[other] { $path } bevat { $count } velden: een paginanummer, een kruisverwijzing, een datum. Elk houdt de tekst die het laatst werd weergegeven en zal niet opnieuw worden bijgewerkt.
}
import-diagnostic-unknown-style-level = { $path } gebruikt de stijl "{ $detail }," die eruit ziet als een kop maar geen niveau noemt. Deze paragrafen worden geïmporteerd als proza in plaats van geraden op een diepte.
import-diagnostic-comment-unanchored = Het commentaar "{ $detail }" in { $path } kon niet worden gekoppeld aan de woorden waar het over ging. Het wordt gehouden op zijn item, waar u het kunt verplaatsen.
import-diagnostic-comment-not-carried = Het commentaar "{ $detail }" in { $path } wordt niet geïmporteerd, omdat niets in het bestand komt als tekst die het kan worden bewaard. Kopieer het uit het bestand als je het nodig hebt.
import-diagnostic-comment-replies-flattened = { $count ->
    [one] { $count } antwoord in { $path } noemde een opmerking die niet in het dossier, dus het komt als een commentaar van zijn eigen.
   *[other] { $count } antwoorden in { $path } noemde een opmerking die niet in het dossier, dus ze komen als opmerkingen van hun eigen.
}
import-diagnostic-prose-not-verbatim = { $count ->
    [one] { $count } paragraaf in { $path } kon niet worden gebracht met de opmaak precies zoals geschreven, dus het komt als platte tekst. Vergelijk het met het origineel.
   *[other] { $count } paragrafen in { $path } konden niet worden gebracht met hun opmaak precies zoals geschreven, dus ze komen als platte tekst. Vergelijk ze met het origineel.
}
import-diagnostic-styled-spaces-not-carried = { $count ->
    [one] In { $path }, { $count } stretch of blank space werd onderstreept of doorgeslagen, een lijn links om in te vullen bijvoorbeeld. Skribisto houdt geen lijn op ruimten alleen, dus het komt als gewone ruimtes, of helemaal niet waar het begint of eindigt een paragraaf.
   *[other] In { $path }, { $count } stretchs van lege ruimte werden onderstreept of doorgeslagen, lijnen links om in te vullen bijvoorbeeld. Skribisto houdt geen lijn op ruimten alleen, zodat ze komen als gewone ruimtes, of helemaal niet waar ze beginnen of eindigen een paragraaf.
}
import-diagnostic-list-nesting-flattened = { $count ->
    [one] { $count } lijst item in { $path } werd genest meer dan { $limit } niveaus diep. Het komt op niveau { $limit }, de diepste Skribisto houdt, met zijn woorden en opmaak.
   *[other] { $count } lijst items in { $path } werden genest meer dan { $limit } niveaus diep. Ze komen op niveau { $limit }, de diepste Skribisto houdt, met hun woorden en opmaak.
}
import-diagnostic-table-read-as-paragraphs = { $count ->
    [one] { $count } tabel in { $path } heeft een rij veel breder dan de rest, te scheef om uit te leggen als een raster. Zijn cellen komen als paragrafen, de een na de ander, met hun woorden en opmaak.
   *[other] { $count } tafels in { $path } hebben een rij veel breder dan de rest, te scheef om als raster uit te leggen. Hun cellen komen als paragrafen, de een na de ander, met hun woorden en opmaak.
}
import-diagnostic-spaces-shortened = { $count ->
    [one] { $count } run van ruimtes in { $path } was langer dan { $limit } ruimtes, en komt verkort tot { $limit }. De woorden eromheen zijn er allemaal.
   *[other] { $count } runs van ruimtes in { $path } waren langer dan { $limit } ruimtes, en komen verkort tot { $limit }. De woorden om hen heen zijn er allemaal.
}
import-diagnostic-epigraph-not-carried = "{ $title }" wordt geleid door een epigraaf, maar een { $kind } kan er geen houden. Het citaat wordt bovenaan de tekst gehouden.
import-diagnostic-epigraph-placement-ambiguous = Een epigraph zit tussen "{ $title }" en "{ $below }" en kan ook hoofd zijn. Het werd gegeven aan "{ $title }," waar een epigraph meestal gaat.
import-document-diagnostics = { $errors ->
    [0] { $warnings ->
            [one] 1 ding om te weten
           *[other] { $warnings } dingen te weten
        }
   *[other] { $errors ->
            [one] 1 bestand kon niet gelezen worden
           *[other] { $errors } bestanden konden niet worden gelezen
        }
}
import-document-diagnostics-none = Niets te melden.

versions-title = Versies
versions-scope-synopsis = Synopsis
versions-scope-prose = Tekst
versions-loading = Kijken door je back-ups...
versions-empty = Nog geen eerdere versie hiervan
versions-error = Kon uw back-ups niet lezen. Er is niets verloren gegaan, maar deze lijst kan onvolledig zijn
versions-did-not-exist = bestond nog niet op { $date }
versions-deleted-after = Verwijderd enige tijd na { $date }
versions-unreadable = { $count ->
    [one] { $count } back-up kon niet gelezen worden
   *[other] { $count } back-ups konden niet gelezen worden
}
versions-thinned = { $count ->
    [one] Oudere versies dunner naarmate ze ouder worden. De eigen geschiedenis van het project is al gedaald { $count } eerdere staat van deze tekst.
   *[other] Oudere versies dun uit naarmate ze ouder worden. De eigen geschiedenis van het project is al gedaald { $count } eerdere staten van deze tekst.
}
versions-source-backup = Van een back-up
versions-source-project = Uit de eigen geschiedenis van het project
versions-list-caption = Eén ingang per verandering, niet per backup
versions-pick-a-version = Kies een versie om te zien wat er veranderd is
versions-earliest = Er is niets ouders om mee te vergelijken.
versions-no-change = Er is niets veranderd in dit deel van het item
versions-formatting-only = Alleen de opmaak is hier veranderd. De woorden zijn hetzelfde
versions-show-unchanged = Ongewijzigd tonen
versions-hide-unchanged = Niet gewijzigd verbergen
versions-next-change = Volgende wijziging
versions-near = in de buurt van "{ $text }"
versions-words-added = { $count ->
    [one] { $count } woord toegevoegd
   *[other] { $count } woorden toegevoegd
}
versions-words-removed = { $count ->
    [one] { $count } woord verwijderd
   *[other] { $count } woorden verwijderd
}
versions-blocks-moved = { $count ->
    [one] { $count } paragraaf verplaatst
   *[other] { $count } alinea's verplaatst
}
versions-pin = Pin de back-up van deze versie kwam van. Automatische opruiming zal nooit verwijderen
versions-unpin = Verwijder de back-up van deze versie kwam van. Automatische opruiming kan het opnieuw verwijderen
versions-pinned-only = Alleen gepinde versies tonen
versions-pin-note = Alleen versies van een back-up kunnen worden gepind. Een pin houdt een bestand bij, en de eigen geschiedenis van het project is er geen.
versions-pinned-empty = Er zit hier nog niets vast.
versions-pinned-empty-log = Alleen versies van een back-up kunnen worden gepind. Een pin houdt een bestand bij, en de eigen geschiedenis van het project is er niet één.
versions-range-filter = Alleen versies tussen twee datums tonen
versions-filtered-empty = Geen versie die overeenkomt met de filters die u hebt ingesteld
versions-clear-filters = De filters wissen
versions-last-30-days = Afgelopen 30 dagen
versions-restore-button = Deze versie herstellen
versions-restore-confirm-title = Deze tekst vervangen door de versie van { $date }?
versions-restore-confirm-text = Wat u nu hebt zal worden vervangen door de tekst van dit artikel op { $date }.
versions-restore-confirm-with-comments = Wat u nu hebt zal worden vervangen door de tekst van dit artikel op { $date }. { $count ->
    [one] { $count } commentaar is verankerd in de huidige tekst en mag worden achtergelaten wees.
   *[other] { $count } opmerkingen zijn verankerd in de huidige tekst en mogen worden achtergelaten wees.
}
versions-restore-confirm-undo-note = Een back-up wordt eerst gemaakt, en Ctrl+Z maakt dit in één stap ongedaan.
versions-restored-toast = Herstelde de versie van { $date }
versions-undo = Ongedaan maken
versions-restore-row-gone = Dat punt zit niet meer in dit project
versions-restore-no-home = Dit item is veranderd type sindsdien, en de oude tekst kan nergens meer in
versions-restore-no-safety-copy = Je veiligheids back-up liep niet, dus er was niets veranderd.
versions-restore-failed = Het herstel is mislukt: { $error }
versions-restore-backup-busy = Er is al een backup aan de gang. Probeer het zo opnieuw
versions-restore-in-backup-file = U kijkt naar een back-upbestand; open het project zelf om het te herstellen
versions-restore-no-project = Geen project is geopend
versions-recreate-button = Breng dit terug...
versions-recreate-picker-title = Waar moet het heen?
versions-recreate-picker-empty = Dit project heeft geen map om het in te zetten
versions-recreate-picker-confirm = Breng het terug.
versions-recreate-picker-cancel = Annuleren
versions-recreate-untitled = Dit item
versions-recreate-confirm-title = Breng je '{ $item }' terug?
versions-recreate-confirm-text = Het zal worden toegevoegd aan { $destination }, met de tekst die het had op { $date }.
versions-recreate-confirm-undo-note = Ongedaan maken, over de boodschap die volgt, neemt het direct terug.
versions-recreated-toast = "{ $item }" is terug in uw project
versions-recreated-partial-toast = { $count ->
    [one] "{ $item }" is terug, maar een van zijn teksten kon niet gelezen worden.
   *[other] "{ $item }" is terug, maar { $count } van zijn teksten kon niet worden gelezen
}
versions-recreate-already-here = Dat item zit al in uw project
versions-recreate-no-destination = Kies ergens in het bindmiddel om het te plaatsen
versions-recreate-unreadable = Die back-up kon niet worden gelezen, dus er werd niets toegevoegd
versions-recreate-failed = Kan het niet terugbrengen: { $error }
versions-changed-percent = { $percent }% hiervan is veranderd
versions-hidden-paragraphs = { $count ->
    [one] ... { $count } ongewijzigd punt ...
   *[other] ... { $count } ongewijzigde punten ...
}

timeline-title = Ga terug in de tijd
timeline-coverage = { $count ->
    [one] { $count } versie opgenomen, terug te gaan naar { $oldest }
   *[other] { $count } versies opgenomen, terug te gaan naar { $oldest }
}
timeline-loading = Kijken door het verleden van je project...
timeline-empty = Er is nog geen versie van dit project opgenomen
timeline-no-changes = Sindsdien is er niets veranderd.
timeline-no-text-changes = Sindsdien is er geen tekst veranderd
timeline-slider-label = Opgenomen versie
timeline-series-name = Projectgrootte
timeline-bars-caption = Elke bar is een opgenomen versie, zo groot als het project toen was. De gemarkeerde is wat je kijkt.
timeline-bars-caption-periods = Te veel versies om een voor een te laten zien, dus elke staaf is een { $period }, zo groot als het project was aan het einde van het.
timeline-unit-hour = uur
timeline-unit-day = dag
timeline-unit-week = week
timeline-unit-month = maand
timeline-range-filter = Alleen versies tussen twee datums tonen
timeline-range-empty = Er is geen versie opgenomen in die data
timeline-open-period = Deze periode openen
timeline-show-all = De hele geschiedenis tonen
timeline-changed-since = { $count ->
    [one] { $count } item verschilt tussen { $date } en uw project nu
   *[other] { $count } items verschillen tussen { $date } en uw project nu
}
timeline-kind-added = Geschreven sinds
timeline-kind-removed = Niet meer in het project
timeline-kind-changed = Uitgegeven sinds
timeline-kind-moved = Verplaatst sinds
timeline-not-yet-written = Dit bestond toen nog niet.
timeline-no-text-of-its-own = Dit heeft geen eigen tekst, het is een koers naar wat erin zit.
timeline-reader-close = Sluiten
timeline-reader-stamp = Zoals op { $date }
timeline-reader-loading = Het openen van de opgenomen versie...
timeline-reader-unreadable = Deze opname kon niet gelezen worden. De back-up kan verplaatst, verwijderd of op een schijf gezet zijn die niet verbonden is.
timeline-reader-compared = Zoals het was op { $date }, ingesteld tegen wat het nu zegt
timeline-reader-diff-legend = Sindsdien is het doorgebroken; onderstreept is toegevoegd.
timeline-reader-view-label = Wat te laten zien
timeline-reader-view-diff = Wijzigingen
timeline-reader-view-text = Tekst
timeline-reader-deleted = Dit zit niet meer in je project. Je kunt het hier lezen en kopiëren.
timeline-prose-only-record = Dit punt komt uit de eigen geschiedenis van het project, die tekst en niets anders houdt, dus wat werd verwijderd of verplaatst omdat het niet kan worden getoond. Kies daarvoor een back-up.

settings-sec-extensions = Uitbreidingen

settings-group-games-forward = Altijd vooruit
settings-games-forward-toggle = Speel "Altijd vooruit"
settings-games-forward-hint =
    Terwijl je speelt, kan niets wat je hebt geschreven worden teruggenomen: Backspace, Delete,
    Cut, drag-and-drop and Undo zijn allemaal uitgeschakeld in de oppervlakken die u hieronder kiest.
    U kunt nog steeds typen, plakken, formatteren en vrij bewegen. Het concept groeit alleen maar.
settings-games-session-warning =
    Dit is een keuze per sessie: het wordt nooit opgeslagen. Het project sluiten of stoppen
    Skribisto, eindigt altijd het spel, en alles wat je schreef tijdens het spelen wordt
    Weer ondoenbaar op het moment dat het gebeurt.
settings-group-games-scope = Indien van toepassing
settings-games-in-prose = Manuscript proza
settings-games-in-synopsis = Synopsen
settings-games-scope-hint =
    Opmerkingen, voetnoten en titels worden nooit bevroren: ze zijn waar je de fix noteert
    Je hebt jezelf net verboden het te maken.
settings-games-inert-warning =
    "Altijd vooruit" staat aan, maar is van toepassing op niets. Tik minstens één oppervlak boven,
    of het verandert niets als je schrijft.

games-title = Spelletjes schrijven
games-forward-name = Altijd vooruit
games-forward-blurb =
    Draft zonder iets terug te nemen. Verwijderen, snijden en ongedaan maken zijn uitgeschakeld
    Schrijf de volgende zin in plaats van de laatste.
games-forward-playing = Afspelen: verwijderen is uitgeschakeld
games-forward-idle = Niet spelen
games-session-note = Beëindigt wanneer je het project sluit.
games-scope-prose-and-synopsis = Geldt voor je proza en synopsen.
games-scope-prose = Geldt voor je proza.
games-scope-synopsis = Geldt voor je synopsen.
games-scope-nothing = Is nog niet van toepassing. Kies een oppervlakte in Instellingen.
games-settings-link = Spelinstellingen worden geschreven...
statusbar-games-forward = Altijd vooruit
statusbar-games-forward-tooltip =
    "Altijd vooruit" is op: verwijderen, snijden en ongedaan maken zijn uitgeschakeld terwijl u
    Klik om te stoppen met spelen.
goal-progress-words = { $g ->
   *[other] Woorden: { $count } / { $goal }
}
goal-progress-characters = { $g ->
   *[other] Tekens: { $count } / { $goal }
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
overview-excluded-from-export = Buiten de export gelaten, dus telt het voor geen totaal

goal-subtree-total-words = { $count ->
    [one] { $items } doel binnen telt tot { $words } woorden
   *[other] { $items } doelen binnen toe te voegen tot { $words } woorden
}
goal-subtree-total-characters = { $count ->
    [one] { $items } doel binnen telt tot { $words } tekens
   *[other] { $items } doelen binnen tellen tot { $words } tekens
}

new-work-goal-unit = Tel in
new-work-goal-unit-hint = Gebruikt voor elk woord of teken doel in dit project. Veranderbaar later.

settings-group-goal-unit = Doelstellingen
settings-goal-unit-switch-title = Teleenheid wisselen
settings-goal-unit-switch-text = De reeds voor dit project vastgestelde doelstellingen en mijlpalen werden ingevoerd in { $from }. Overschakelen naar { $to } zet ze niet om, dus elk bestaand getal wordt nu gelezen als { $to }.
settings-goal-unit-switch-informative = Niets is verloren. Schakel terug op elk moment om de oorspronkelijke lezing te herstellen, dan bijwerken van de doelen die u wilt houden.
settings-goal-unit-switch-confirm = Omschakelen

goal-distribute-action = Verdeel...
distribute-title = Het doel verdelen
distribute-empty = Er zit niets in om het doel te delen.
distribute-weight = Uitdelen
distribute-weight-length = Op lengte
distribute-weight-rows = Per stuk
distribute-weight-even = Even
distribute-overwrite = Reeds vastgestelde doelstellingen vervangen
distribute-col-item = Post
distribute-col-current = Nu
distribute-col-proposed = Na
distribute-total = Deze bedragen { $total }, tegen een streefcijfer van { $goal }.
distribute-over-budget = De doelen die al zijn vastgesteld binnen komen tot { $over } meer dan die van deze container. Verhoog het doel, lager hun, of vervang ze.
distribute-apply = Verdelen
distribute-cancel = Annuleren

milestone-target-gone = Doel verwijderd
milestone-no-target = Geen doel
milestone-add = Toevoegen
milestone-add-label = Wat moet worden bereikt?

pace-summary-title = Waar het boek staat
pace-summary-remaining = { $words } te gaan
pace-summary-open = Open het plan
pace-summary-close = Sluiten
pace-summary-dont-show = Dit niet tonen bij openen
pace-summary-menu = Plan schrijven...

menu-help-topics = Hulpthema's (&T)
menu-help-shortcuts = Sneltoetsen... (&K)
menu-help-website = Skribisto op het web (&W)
menu-help-report = Een probleem melden... (&R)
menu-help-check-updates = Controleren op updates (&U)
menu-command-palette = Commandopalet... (&C)

learn-help-topics = Onderwerpen helpen
learn-shortcuts = Sneltoetsen
learn-website = Skribisto op het web

help-window-title = Help
help-filter-topics = Onderwerpen filteren
help-no-matching-topic = Geen onderwerp komt overeen.
help-topic-missing = Dit onderwerp is niet meer beschikbaar.
help-back = Terug
help-not-translated = Dit artikel is nog niet beschikbaar in het Nederlands. Het wordt in het Engels weergegeven.

help-section-getting-started = Aan de slag
help-section-writing = Schrijven
help-section-reviewing = Beoordelen
help-section-exchanging = Woorden in en uit krijgen
help-section-keeping = Uw project veilig houden
help-section-extensions = Uitbreidingen

help-topic-getting-started = Uw eerste project
help-topic-writing-model = Hoe een boek wordt samengesteld
help-topic-drafts-and-old-versions = Een oudere versie behouden
help-topic-goals-and-pace = Doelstellingen en tempo
help-topic-comments = Opmerkingen
help-topic-round-trip = Uw boek naar een lezer sturen
help-topic-export = Exporteren
help-topic-import-documents = Documenten importeren
help-topic-import-projects = Een heel project oversteken
help-topic-backups-and-versions = Backups en versies

help-shortcuts-title = Sneltoetsen
help-shortcuts-filter = Sneltoetsen filteren
help-shortcuts-no-matches = Geen kortere weg overeenkomsten.
help-shortcuts-rebind = Sneltoetsen wijzigen...
help-shortcuts-close = Sluiten

command-palette-placeholder = Typ een opdracht

shortcut-name-binder-duplicate = Dupliceren
shortcut-name-comments-add = Opmerking toevoegen
shortcut-name-comments-add-paragraph = Commentaar op paragraaf
shortcut-name-spellcheck-toggle = Spelling controleren
shortcut-name-editor-tab-close = Tabblad sluiten
shortcut-name-editor-tab-pin = Tabblad pinnen of losmaken
shortcut-name-editor-save = Opslaan
shortcut-name-work-export = Exporteren…
shortcut-name-work-new = Nieuw project
shortcut-name-work-open = Project openen
shortcut-name-window-new = Nieuw venster
shortcut-name-work-close = Project sluiten
shortcut-name-app-settings = Instellingen
shortcut-name-app-quit = Afsluiten
shortcut-name-editor-insert-footnote = Voetnoot invoegen
shortcut-name-format-scene-break = Scènepauze invoegen
shortcut-name-format-major-scene-break = Grote scènebreuk invoegen
shortcut-name-format-link = Verwijzing invoegen
shortcut-name-format-mark-nbsp = No-break space invoegen
shortcut-name-format-mark-nbhyphen = No-break Hyphen invoegen
shortcut-name-format-mark-nnbsp = Narrow no-break space invoegen
shortcut-name-format-mark-zwsp = Nulbreedteruimte invoegen
shortcut-name-go-next = Volgende
shortcut-name-go-prev = Vorige
shortcut-name-go-to = Ga naar
shortcut-name-outline-toggle = Omtrek aan/uit
shortcut-name-preview-toggle = Schakel voorbeeldband aan/uit
shortcut-name-view-fullscreen = Volledig scherm aan/uit
shortcut-name-view-focus-mode = Afleidingsvrije modus aan/uit
shortcut-name-editor-size-increase = Tekstgrootte vergroten
shortcut-name-editor-size-decrease = Tekstgrootte verkleinen
shortcut-name-editor-size-reset = Tekstgrootte resetten
shortcut-name-editor-find = Zoeken
shortcut-name-editor-replace = Vervangen
shortcut-name-editor-find-next = Volgende overeenkomst
shortcut-name-editor-find-prev = Vorige overeenkomst
shortcut-name-search-show = Zoeken in project
shortcut-name-search-replace = Vervangen in project
shortcut-name-outline-open-to-side = Open naar de zijkant
shortcut-name-help-topics = Hulpthema's
shortcut-name-help-shortcuts = Sneltoetsen
shortcut-name-help-website = Skribisto op het web
shortcut-name-help-report = Een probleem melden
shortcut-name-help-check-updates = Controleren op updates
update-check-behind = Versie { $version } is beschikbaar.
update-check-current = Je draait de nieuwste versie.
update-check-failed = Kon niet controleren op een nieuwere versie. { $error }
shortcut-name-command-palette = Commandopalet
help-section-reference = Wat zijn dingen?

help-concept-scene-break = Scènebreuk
help-concept-major-scene-break = Grote breuk in de scène
help-concept-find-in-prose = Zoek in proza
help-concept-story-bible = Verhaalbijbel
help-concept-goal-unit = Woorden of tekens
help-concept-goal-progress = Voortgang
help-concept-manuscript-words = Wat telt als het manuscript
help-concept-exportable = Buiten de export gelaten
help-concept-distribute = Een doel delen
help-concept-subtree-total = Wat de doelen binnenin optellen tot
help-concept-milestone = Mijlpaal
help-concept-pace-plan = Schrijftempoplan

help-concept-tag = Tag
help-concept-label = Korte beschrijving
help-concept-point-of-view = Perspectief
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
help-concept-story-bible-entry = Verhaal bijbeltekst
help-concept-text-replacement = Tekstvervanging
help-concept-smart-punctuation = Slimme interpunctie
help-concept-export-style = Exportstijl
help-concept-round-trip-marks = Markers voor de round-trip

epigraph-mark-attribution = Bronregel
epigraph-mark-attribution-tip = Markeer de lijn waarin de curret zich bevindt als bron van de citaat, zodat het afdrukt als een attributie
margin-lane-name = Margemarkeringen
margin-lane-provider-comments = Opmerkingen
margin-lane-provider-story-bible = Verhaal bijbeltekst
margin-lane-provider-story-bible-hint = Op een noot In proza lezing, waar die ingang wordt genoemd, en de scènes verteld door zijn ogen. Ook markeert die namen in de proza zelf.
margin-lane-mark-point-of-view = Vertelde vanaf hier
margin-lane-mark-named-here = Hier genoemd
margin-lane-provider-comments-hint = Wanneer een noot aan de tekst is gehecht
margin-lane-provider-search = Zoekresultaten
margin-lane-provider-search-hint = Elke hit naar wat je voor het laatst zocht, waar het ook in het document staat
margin-lane-provider-boundaries = Wanneer elk document begint
margin-lane-provider-boundaries-hint = Een regel aan de top van elke scène in een stroom, zodat je kunt zien waar je bent
margin-lane-provider-spelling = Spelling
margin-lane-provider-spelling-hint = Waar de spellingcontrole een woord heeft gemarkeerd. Uit tenzij u erom vraagt: het markeert de mening van een machine over uw proza
margin-lane-spelling = { $word }, mogelijk spelfouten
margin-lane-search-hit = { $text }, match { $index } van { $total }
margin-lane-search-current = { $text }, match { $index } van { $total }, degene op wie u bent
margin-lane-boundary = Begin van { $title }
margin-lane-boundary-untitled = een titelloos document
settings-page-margin-lane = Margemarkeringen
settings-desc-margin-lane = De strip naast de scroll bar, en wat het laat zien
settings-margin-lane-enabled = De margestrook tonen
settings-margin-lane-enabled-hint = Dezelfde schakelaar als Weergave Marge markeringen.
settings-margin-lane-enabled-more = Het zegt nooit dat er iets mis is: het laat je zien waar de dingen zijn, en laat wat je daarvan moet maken aan je over.
settings-group-margin-lane-marks = Wat het markeert.
settings-margin-lane-no-providers = Nog niets markeert de baan.
settings-group-margin-lane-texture = Structuur van de dialoog
settings-margin-lane-texture = De dialoogtextuur tonen
settings-margin-lane-texture-hint = Een streepje per paragraaf: hoe lang is het en hoeveel ervan wordt er gesproken.
settings-margin-lane-texture-more = Niet gemeten voor talen zonder curated conventie, waar de balk alleen de lengte toont.
settings-group-margin-lane-surfaces = Waar het lijkt
settings-margin-lane-surface-editor = Tekstbewerker
settings-margin-lane-surface-stream = Stroomt
settings-margin-lane-surface-search-preview = Zoekvoorbeeld
menu-margin-lane = Margemarkeringen (&M)

status-preset-drafting = Concept schrijven
status-preset-passes = Herzieningspassen
status-preset-plume = Plume Creator
status-todo = Te doen
status-draft = Concept
status-revised = Herzien
status-final = Definitief
status-outline = Overzicht
status-first-edit = 1e bewerking
status-second-edit = 2e bewerking
status-done = Gereed
status-plume-draft-1 = 1e ontwerp
status-plume-draft-2 = 2e ontwerp
status-plume-draft-3 = 3e ontwerp
status-plume-edit-1 = 1e bewerking
status-plume-edit-2 = 2e bewerking
status-plume-edit-3 = 3e bewerking
status-plume-proofread = Proefgelezen
status-plume-finished = Voltooid
new-work-statuses = Werkstroom
new-work-statuses-hint = De stadia waar je een scène doorheen verplaatst. Je kunt deze op elk moment hernoemen, herschikken of toevoegen.
status-none = Geen status
inspector-status = Status
overview-col-status = Status
overview-status-mixed = de onderstaande delen zijn het niet eens
help-concept-status = Status
status-completion-title = Waar het boek staat
status-completion-headline = { $done } van { $total } scènes afgewerkt
status-completion-empty = Dit leest het manuscript, dus het vult in terwijl je schrijft.
status-completion-open = Waar het boek staat...

settings-page-statuses = Statussen
settings-desc-statuses = Hernoemen, herschikken, toevoegen en verwijderen van de stadia waar je een scène doorheen verplaatst
settings-statuses-add = Status toevoegen
settings-statuses-add-placeholder = Nieuwe statusnaam
settings-statuses-desc =
    De etappes een scène beweegt door, in volgorde, van het minst afgewerkt aan de top naar de meeste
    die volgorde is wat "minder afgewerkt dan" betekent overal anders
    in de app, dus zet ze op de manier waarop uw proces daadwerkelijk draait.
settings-statuses-apply-preset = Een voorinstelling toepassen...
settings-statuses-preset-applied = { $added ->
    [one] 1 status toegevoegd
   *[other] { $added } statussen toegevoegd
}
settings-statuses-preset-refused = Dit project heeft al een ladder. Voorinstellingen vullen alleen een lege. Verwijder de sporten die je niet eerst wilt.
settings-statuses-duplicate = "{ $name }" staat al op deze ladder
settings-statuses-added = Toegevoegd "{ $name }"
settings-statuses-move-up = Naar boven (minder afgewerkt)
settings-statuses-move-down = Naar beneden (afgewerkt)
settings-statuses-details-placeholder = Wat betekent deze fase (facultatief)
settings-statuses-delete = "{ $name }" schrappen
settings-statuses-delete-in-use = { $count ->
    [one] Verwijderen "{ $name }": 1 item staat erop en verliest zijn status
   *[other] Verwijderen "{ $name }": { $count } items staan erop en zullen hun status verliezen
}
settings-statuses-deleted = geschrapt "{ $name }"
settings-statuses-deleted-in-use = { $count ->
    [one] Deleted "{ $name }." 1 item heeft geen status meer.
   *[other] Geschrapt "{ $name }." { $count } items hebben geen status meer.
}
settings-statuses-empty-title = Nog geen workflow
settings-statuses-empty-body = Een status zegt hoe ver een scène is. Voeg er een toe of begin met een preset.
status-category-planned = Gepland
status-category-drafting = Concept schrijven
status-category-needs-work = Werk nodig
status-category-revised = Herzien
status-category-final = Definitief
