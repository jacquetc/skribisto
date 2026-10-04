# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

menu-work = Projekt (&W)
menu-new-work = Nytt projekt (&N)
menu-open-work = Öppna projekt… (&O)
menu-new-window = Nytt fönster (&W)
menu-import-from = Import från (&I)
menu-create-from = Skapa från (&C)
menu-import-plume = Plume Creator (.plume)… (&P)
menu-import-document = Dokument (Markdown, Word, ODT) (&D)
menu-import-manuskript = Manuskript (.msk)… (&M)
menu-export = Exportera (&X)
menu-export-book = Exportbok
menu-export-part = Exportdel
menu-export-chapter = Exportkapitel
menu-export-scene = Exportera scen
menu-export-note = Exportera anteckning
menu-export-paratext = Exportera paratext
menu-export-folder = Export mappar
menu-export-choose = Välj...
menu-export-none = Öppna ett dokument för export
menu-save = Spara (&S)
menu-save-as-file = Spara som en fil… (&F)
menu-save-as-folder = Spara som mapp… (&D)
menu-backup = Säkerhetskopiera nu (&B)
menu-close-work = Stäng projekt (&C)
menu-welcome = Välkommen... (&L)
menu-settings = Inställningar (&E)
menu-quit = Avsluta (&Q)

menu-view = Visa (&V)
menu-outline = Disposition (&O)
menu-search = Sök i Project (&S)
menu-timeline = Gå tillbaka i tiden (&G)
menu-search-preview = Sök förhandsgranska (&P)
menu-fullscreen = Fullscreen (&F)
menu-focus-mode = Distraktionsfritt läge (&D)
menu-format = Format (&O)
menu-scene-break = Infoga scen break (&K)
menu-major-scene-break = Infoga större scen break (&M)

menu-go = Gå till (&G)
menu-go-next-scene = Nästa scen (&S)
menu-go-prev-scene = Föregående scen (&V)
menu-go-next-chapter = Nästa kapitel (&C)
menu-go-prev-chapter = Föregående kapitel (&H)
menu-go-next-note = Nästa anteckning (&N)
menu-go-prev-note = Föregående anteckning (&T)

menu-tools = Verktyg (&T)
menu-help = Hjälp (&H)

ctx-add = Lägg till (&A)
ctx-new-item = Ny artikel (&N)
ctx-new-folder = Ny mapp (&F)
ctx-rename = Byt namn (&R)
ctx-duplicate = Duplicera (&D)
ctx-import-here = Importera här... (&I)
ctx-indent = Öka indrag (&I)
ctx-outdent = Minska indrag (&O)
ctx-trash = Flytta till papperskorgen (&T)
ctx-open-to-side = Öppen för Side (&S)
ctx-open = Öppna (&O)
ctx-reveal-in-outline = Reveal i Outline (&L)
ctx-move-up = Flytta upp (&U)
ctx-move-down = Flytta ned (&N)

ctx-tab-close = Stäng (&C)
ctx-tab-close-others = Stäng andra flikar (&O)
ctx-tab-close-all = Stäng alla flikar (&L)
ctx-tab-open-to-side = Öppen för Side (&S)
ctx-tab-open-in-main = Öppna i huvudpanelen (&I)
ctx-tab-move-to-side = Flytta till sidan (&M)
ctx-tab-move-to-main = Flytta till huvudrutan (&M)
ctx-tab-move-to-new-window = Flytta in i ett nytt fönster (&N)
ctx-tab-move-window-unsaved = Spara detta projekt först: ett andra fönster öppnas på en fil på disken
ctx-tab-pin = Pin denna flik (&P)
ctx-tab-unpin = Unpin denna flik (&U)
tab-pinned-tooltip = Pinned: "Stäng andra" och "Stäng allt" lämna den öppen

create-book = Bok
create-part = Del
create-chapter = Kapitel
create-scene = Scen
create-note = Anteckning
create-note-folder = Note mapp
create-folder = Mapp
create-paratext = Paratext
create-paratext-folder = Paratext mapp
create-book-end = Slutet av bok
create-story-bible-entry = Berätta bibelinträde...
type-book-start = Bokstart
type-text = Text

new-item-book = Ny bok
new-item-part = Ny del
new-item-chapter = Nytt kapitel
new-item-note = Ny anteckning
new-item-note-folder = Ny Note Folder
new-item-folder = Ny mapp
new-item-paratext = Ny paratext
new-item-paratext-folder = Ny Paratext Mappar
new-item-story-bible-entry = Ny post i berättelsebibeln

placement-inside = inuti inuti insidan
placement-after = efter efter
placement-after-parent = efter förälder
placement-top-level = Högsta nivå


ctx-promote = Konvertera till (&C)
promote-chapter-folder = Kapitelmapp
promote-flat-chapter = Fristående kapitel
promote-lossy-title = Ingenstans att hålla texten
promote-lossy-text = En { $target } har ingenstans att hålla dessa: { $kinds }. Flytta eller rensa texten först, sedan konvertera.
content-scene-text = Scentext
content-note-text = Notera text
content-book-title = Boktitel
content-book-subtitle = Boka undertitel
content-part-title = Deltitel
content-chapter-title = Kapitel titel
content-epigraph-text = Motto
content-paratext-text = Paratext
promote-blocked-title = Kapitel inte tom
promote-blocked-text = { $count ->
    [one] Detta kapitel innehåller fortfarande { $count } objekt. Flytta eller sopa den innan du konverterar den till ett platt kapitel.
   *[other] Det här kapitlet innehåller fortfarande { $count } objekt. Flytta eller sopa dem innan du konverterar det till ett platt kapitel.
}

inspector = Egenskaper
inspector-empty = Öppna ett objekt för att inspektera det.
inspector-promote = Konvertera till...
statusbar-toggle-outline = Växla bindemedel
statusbar-toggle-inspector = Växla inspektören
statusbar-save-unsaved = Osparade ändringar. Klicka för att spara
statusbar-save-saved = Alla ändringar som sparats
statusbar-save-autosave = Autosave är på. Förändringar sparas när du skriver
statusbar-saving = Spara...
statusbar-word-count = { $count ->
    [one] { $count } ord
   *[other] { $count } ord
}
statusbar-word-count-tooltip = Ord i scenen du redigerar
statusbar-word-char-count = { $words ->
    [one] { $words } ord
   *[other] { $words } ord
} · { $chars ->
    [one] { $chars } tecken
   *[other] { $chars } tecken
}
session-toggle = Skrivningssession: starta eller pausa en fokuserad sprint
session-configure = Ställ in sessionens ordmål och tidsgräns
session-configure-title = Skrivningssession
session-word-goal = Ordmål
session-time-limit = Tidsgräns
session-time-limit-unit = Min Min Min Min Min Min Min Min Min Min Min Min Min Min Min Min Min Min Min
session-no-goal = Inget mål
session-no-limit = Ingen gräns
session-reset = Återställ session
session-readout = { $words } ord · { $time }
session-readout-timed = { $words } ord · { $time } vänster
statusbar-focus-exit = Lämna distraktionsfritt läge
statusbar-focus-synopsis = Synopsis
statusbar-focus-go-prev = Föregående artikel (Alt+Up)
statusbar-focus-go-next = Nästa artikel (Alt+Down)

rail-settings = Inställningar

language = Språk
theme = Tema
english = Engelska engelska
french = Franska franska
light = Ljus ljus
dark = Mörk
settings-text-width = Textbredd
settings-preview-width = Sök förhandsgranskning bredd
settings-autosave = Autosave till disk
settings-show-welcome = Visa lanseringen vid start
settings-show-welcome-tip = När det är klart öppnar det sista projektet istället.
settings-check-for-updates = Kontrollera nya versioner
settings-check-for-updates-tip = När en dag högst, Skribisto frågar projektets webbplats vilken version är aktuell. Förfrågan bär ingen identifierare, och inte ens den version du kör: jämförelsen sker på denna dator.

settings-title = Inställningar
settings-close = Stäng
settings-search = Sökinställningar
settings-reset = Återställ till standarder
settings-done = Klar
settings-cancel = Avbryt
settings-apply = Verkställ
settings-ok = OK
settings-reset-confirm-title = Återställ alla inställningar till standarder?
settings-reset-confirm-body = Återställer denna applikations utseende, redaktör och skrivinställningar. Ditt projekts egna inställningar, tangentbordsgenvägar och sparade stilar påverkas inte. Det kan inte vara ogjort.
settings-empty-title = Inga inställningar här ännu
settings-empty-hint = Detta avsnitt kommer att få alternativ i en framtida uppdatering.

settings-sec-appearance-behaviour = Utseende beteende
settings-sec-editor = Redigerare
settings-sec-spelling = Stavning
settings-sec-backup = Backup Sync
settings-sec-compile = Kompilera export
settings-page-appearance = Utseende
settings-field-image-size-policy = Stora bilder
settings-image-policy-ask = Fråga varje gång
settings-image-policy-keep = Håll original
settings-image-policy-downscale = Optimera
settings-hint-image-size-policy = Vad man ska göra när en infogad bild är större än manuskriptet behöver.
settings-page-notifications = Meddelanden
settings-page-scene = Scen
settings-page-synopsis = Synopsis
settings-page-notes = Anteckningar
settings-page-editor-behavior = Redaktör Behavior
settings-page-goals = Mål Word räkna
settings-page-games = Skriva spel
settings-page-corkboard = Anslagstavla
settings-page-distraction-free = Distraktionsfri
settings-page-dictionaries = Ordböcker
settings-page-autosave = Automatisk sparning
settings-page-export = Exportformat
settings-page-paratext = Paratextstrukturer
settings-page-keymap = Tangentkopplingar
settings-keymap-filter = Filter genvägar

settings-desc-sec-appearance-behaviour = Hur själva applikationen ser ut och vad den gör när den börjar.
settings-desc-sec-editor = Skrivytan: hur det ser ut, och vad det gör som du skriver.
settings-desc-sec-spelling = Spell-checking och ordböckerna bakom det.
settings-desc-sec-backup = Hur ditt projekt når disken, och vilka kopior hålls.
settings-desc-sec-compile = Det som lämnar Skribisto och i vilken form.
settings-desc-sec-work = Inställningar som hör till detta projekt och reser inuti sin fil.
settings-desc-sec-extensions = Sidor som bidragits av tilläggen som installerats här.
settings-desc-group-typography = En sida per typ av text: typsnitt, storlek, linjehöjd och avstånd.
settings-desc-appearance = Interface språk, tema, textstorlek och lanseringen vid start.
settings-desc-notifications = Varje meddelande den här sessionen har visat, och de åtgärder du kan spela.
settings-desc-scene = Hur scen prosa är inställd: typsnitt, storlek, linje höjd, streck och avstånd.
settings-desc-synopsis = Hur synopsispanelen är inställd, oberoende av manuskriptet.
settings-desc-notes = Hur anteckningar är inställda, oberoende av manuskriptet.
settings-desc-corkboard = Kortstorlek, vad ett kort visar och hur styrelsen ordnar dem.
settings-desc-distraction-free = Typografi, kolumnbredd och kontrollremsa för fullskärmsskrivning.
settings-desc-distraction-free-themes = Temabiblioteket för fullskärmsskrivning: de skeppade och dina egna.
settings-desc-editor-behavior = Textbredd, synopsis position, skrivmaskin rullning och vårdhöjdpunkten.
settings-desc-punctuation = Den typografiska husstilen nya projekt börjar från. Varje projekt följer detta om det inte sätter sin egen.
settings-desc-goals = Ord och karaktärsmål och hur ord räknas.
settings-desc-games = Begränsningar du sätter dig själv medan du utarbetar, till exempel Alltid framåt.
settings-desc-spellcheck = Den enda strömbrytaren som slår på och av överallt.
settings-desc-dictionaries = Installera, ta bort och bläddra i stavningsordböckerna på den här maskinen.
settings-desc-autosave = Oavsett om dina redigeringar är skrivna till disk på egen hand.
settings-desc-backup = När kopior tas, där de hålls, och hur många. Varje projekt följer detta om det inte sätter sin egen.
settings-desc-export = De stilar varje export sammanställer genom: de skeppade och dina egna.
settings-desc-paratext = Den främre och bakre delen ett nytt projekt kan börja med.
settings-desc-keymap = Varje genväg och vad den är bunden till.
settings-page-user = Användare
settings-desc-user = Vem du är, för kommentarerna du skriver.
settings-group-identity = Identitet
settings-field-user-name = Ditt namn
settings-field-user-name-placeholder = Optionell
settings-field-user-name-hint = Tecken på kommentarerna och svaren du skriver. Distinkt från ett projekts författarenamn, som är bokens byline och reser in i filen.
settings-field-user-initials = Dina initialer
settings-field-user-initials-placeholder = Från ditt namn
settings-field-user-initials-hint = Vilken ordbehandlare visar bredvid din kommentar i marginalen. Lämna den tom för att använda de som visas, tas från ditt namn.
settings-field-user-hint = Båda är valfria och gäller för varje projekt på den här datorn. Ändra dem tecknar kommentarerna du skriver nästa. Kommentarer som redan skrivits håller det namn de skrevs under.
settings-desc-author = Namnet som går på detta projekt.
settings-desc-structure = Oavsett om projektets kapitel är mappar eller enskilda objekt.
settings-desc-language = Språket detta projekt prosa kontrolleras mot.
settings-desc-work-backup = Detta projekts egen säkerhetskopieringspolicy eller den allmänna.
settings-desc-personal-dictionary = Orden detta projekt behandlar lika korrekt stavat.
settings-desc-tags = De färgkodade etiketterna detta projekt markerar sina objekt med.
settings-desc-templates = Anteckningen mallar du kan infoga när du skriver i detta projekt.
settings-desc-text-replacements = Korsningar som expanderar när du skriver i det här projektet.
settings-desc-work-punctuation = Citat, streck och avstånd detta projekt prosa följer.

settings-group-typography = Typografi
settings-group-writing-column = Skriva kolumn
settings-group-writing-view = Skrivande vy
settings-group-distraction-free-strip = Kontrollband
settings-group-theme = Tema
settings-group-language = Språk
settings-group-startup = Start
settings-group-updates = Uppdateringar
settings-group-autosave = Automatisk sparning
settings-field-typeface = Typsnitt
settings-field-size = Textstorlek

editor-size-changed-manuscript = Manuskript textstorlek: { $percent }
editor-size-changed-synopsis = Synopsis textstorlek: { $percent }
editor-size-changed-notes = Anteckningar textstorlek: { $percent }
editor-size-changed-corkboard = Korkkortstextstorlek: { $percent }
editor-size-changed-corkboard-expanded = Expanderad redaktörstextstorlek: { $percent }
editor-size-changed-distraction-free = Distraktionsfri textstorlek: { $percent }

menu-text-size-increase = Öka textstorleken (&I)
menu-text-size-decrease = Minska textstorleken (&C)
menu-text-size-reset = Återställ textstorlek (&R)
settings-field-line-height = Line höjd
settings-field-first-line-indent = Första-line indent
settings-field-paragraph-spacing-before = Rymd före punkt
settings-field-paragraph-spacing-after = Rymd efter punkt
settings-field-column-width = Kolumnbredd
settings-distraction-free-width-hint = Gäller endast medan distraktionsfritt läge är på. Din Scen, Synopsis och Notes kolumnbredd är orörda.
settings-distraction-free-title = Håll objektets namn
settings-distraction-free-word-count = Håll ordet räknat
settings-distraction-free-session = Håll skrivsessionen
settings-distraction-free-go-to = Håll gå till... knapp
settings-distraction-free-go = Håll de tidigare och nästa knapparna
settings-distraction-free-chrome-hint = Utgångsknappen stannar alltid, oavsett vad du väljer här. Det är din väg ut om Escape är upptagen.
settings-field-app-theme = Tema
settings-theme-light = Ljus ljus
settings-theme-dark = Mörk
settings-field-text-scale = Interface textstorlek
settings-field-language = Interface språk
settings-synopsis-placement = Placering av synopsis
settings-synopsis-placement-none = Ingen
settings-synopsis-placement-top = Ovanför
settings-synopsis-placement-side = Förutom
settings-typewriter = Typewriter rullning
settings-typewriter-tip = Innehåller linjen du skriver på en fast höjd medan manuskriptet rullar under det. Clicking sätter fortfarande kundvagnen där du klickar.
settings-typewriter-position = Radens position
settings-typewriter-position-top-third = Topp tredje
settings-typewriter-position-middle = Mellan
settings-typewriter-position-bottom-quarter = Bottom kvartal
settings-highlight-scope = Höjdpunkt runt vården
settings-highlight-scope-none = Ingen
settings-highlight-scope-sentence = Mening
settings-highlight-scope-paragraph = Punkt
settings-highlight-scope-tip-none = Lämna sidan slätt. Ingenting skuggas när du skriver.
settings-highlight-scope-tip-sentence = Skugga den mening du skriver, så det sticker ut från de runt den.
settings-highlight-scope-tip-paragraph = Skugga hela stycket du skriver, för att hålla passagen du arbetar med i sikte.
settings-group-container-views = Behållarvyer
settings-remember-view = Kom ihåg den sista vyn för varje objekttyp
settings-remember-view-tip = Öppna en behållare på den vy du senast använde för den typen
settings-remember-view-tip-more =
    En bok, del och kapitel erbjuder varje flera vyer (den egna sidan, hela
    manuskript, fullständig synopsis). Vänd detta på och varje typ öppnas på
    Visa att du senast valde för det, t.ex. växla ett kapitel till Fullt kapitel och
    Nästa kapitel du öppnar visar Fullt kapitel också. Varje typ minns sin egen vy.
settings-group-counting = Ordräkning
settings-counting-auto = Automatisk (för språk)
settings-counting-whitespace = Split på utrymmen
settings-counting-unicode-words = Unicode ord
settings-counting-cjk-hybrid = CJK-medveten (per karaktär)
settings-counting-hint = Automatiskt räknar kinesiska och japanska med karaktär och alla andra språk med ord. Ändra det bara om räkningen ser fel för ditt språk.
settings-group-goals-display = Visning
settings-show-characters = Visa teckenräkningen i statusfältet
settings-autosave-hint = Ändringar skrivs till disk automatiskt när du skriver.

settings-sec-work = Projekt
settings-page-structure = Struktur
settings-page-author = Författare
settings-field-author-name = Författarens namn
settings-field-author-placeholder = Optionell
settings-field-author-hint = Utgår på den sammanställda titelsidan och i exporterade fil metadata. Lämna det tomt för att utelämna det. Detta är bokens byline. Namnet dina kommentarer är signerade med är i Inställningar ❤️ Användare.
settings-group-chapters = Kapitlen
tidy-titles-title = Tidy kapitel titlar
tidy-titles-none = Inget kapitel eller deltitel upprepar bara sitt eget nummer.
tidy-titles-lead = { $count ->
        [one] Ett kapitel eller en del heter ingenting annat än sitt eget nummer.
       *[other] { $count } kapitel och delar har inget annat än sitt eget nummer.
    }
tidy-titles-explain = Att rensa dessa titlar lämnar var och en namngiven av det nummer som boken redan vet, samma nummer som exportutskrifterna. Inget annat ändras, och en undo sätter dem alla tillbaka.
menu-document-tidy-titles = Tidy kapitel titlar... (&Y)
settings-group-numbering = Nummer
settings-number-chapters = Antal kapitel och delar
settings-number-chapters-tip = Kapitlen och delarna har ett nummer utarbetat där de sitter i boken, visas bredvid sin titel här och tryckt av exporten.
settings-number-chapters-tip-more = Numret lagras aldrig i titeln, så det stannar rätt när du ordnar, infogar eller tar bort. Stäng av detta och exporten trycker titlar ensam, oavsett exportstilen ber om. Att lämna ett kapitel (en prolog, en interlude), använd Inspektörens Nummerbrytare istället: som håller den i boken men stoppar det tar ett nummer.
settings-part-resets-chapter = Starta om kapitelnummer i varje del
settings-part-resets-chapter-tip = Av som standard: kapitlen går rakt på delar av en bok, så "Part Two" öppnar på kapitel Eleven.
settings-part-resets-chapter-tip-more = Det är den vanliga handelspraxis, och vad en läsare förväntar sig. Slå på den för en bok vars delar är avsedda att läsa som separata volymer, där var och en öppnar på kapitel ett.
settings-chapter-flat = Flat kapitel
settings-chapter-flat-hint = På: ett kapitel är ett enda objekt. Du skriver in det, och det håller inga scener. Av: ett kapitel är en mapp. Du skriver fortfarande i det, men det kan hålla scener också. Nya kapitel följer denna inställning; befintliga konverterar via Promote.

settings-styles-builtin = Inbyggd stil
settings-styles-user = Mina stilar
settings-styles-builtin-badge = Inbyggd
settings-styles-duplicate = Duplicera
settings-styles-edit = Redigera
settings-styles-delete = Ta bort
settings-styles-import = Importera…
settings-styles-export = Exportera…
settings-styles-copy-suffix = (kopia)
settings-styles-json-filter = Exportstil
settings-styles-editor-title = Edit stil
settings-styles-editor-none = Välj en anpassad stil för att redigera den eller duplicera en inbyggd en.
settings-styles-imported = Stil importerad
settings-styles-import-failed = Kunde inte importera stil
settings-styles-exported = Stil exporterad
settings-styles-export-failed = Kunde inte exportera stil
settings-styles-field-name = Namn
settings-styles-field-chapters = Kapitelrubriker
settings-styles-field-parts = Del rubriker
settings-styles-field-scene-break = Scenavskiljare
settings-styles-field-major-scene-break = Major scen break
settings-styles-field-spacing = Line avstånd
settings-styles-field-justify = Rättfärdiga text
settings-styles-field-notes = Inkludera anteckningar
settings-styles-field-synopses = Inkludera synops
settings-styles-field-scene-titles = Inkludera scentitlar
settings-styles-field-epigraphs = Inkludera epigrafer
settings-styles-field-paratexts = Inkludera paratexter
settings-styles-field-epigraph-placement = Placering av motto
settings-styles-epigraph-after = Efter titeln
settings-styles-epigraph-before = Före titeln
settings-styles-field-footnotes = Inkludera fotnoter
settings-styles-field-footnote-numbering = Footnote Nummer
settings-styles-footnote-numbering-continuous = Kontinuerlig
settings-styles-footnote-numbering-per-chapter = Starta om varje kapitel
settings-styles-footnote-numbering-per-book = Starta om varje bok
settings-styles-field-images = Bilder
menu-image = Bild (&I)
settings-styles-images-beside = Bredvid dokumentet
settings-styles-images-embed = Inne i dokumentet
settings-styles-images-omit = Lämna ut
settings-styles-group-round-trip = Skicka till en redaktör
settings-styles-field-comments = Inkludera kommentarer
settings-styles-field-round-trip-marks = Inkludera rundtursmarkörer
settings-styles-round-trip-hint = DOCX och ODT endast. Markörer är osynliga identifierare som låter en återkommande fil uppdatera detta projekt istället för att landa bredvid det som en andra kopia.
settings-styles-group-pages = Sidor
settings-styles-field-cover = Öppna med omslaget
settings-styles-field-word-count = Word count på titelsidan
settings-styles-field-page-books = Ny sida på varje bok
settings-styles-field-page-parts = Ny sida vid varje del
settings-styles-field-page-chapters = Ny sida vid varje kapitel
settings-styles-field-page-paratexts = Ny sida vid varje paratext
settings-styles-heading-none = Ingen rubrik
settings-styles-heading-numbered = nummer endast
settings-styles-heading-title = Titel endast
settings-styles-heading-both = Nummer + titel
settings-styles-break-blank = Blank linje
settings-styles-break-none = Ingen
settings-styles-spacing-single = Enkelt
settings-styles-spacing-onehalf = 1½
settings-styles-spacing-double = Dubbel dubbel

welcome-title = Välkommen till Skribisto
welcome-version = Version { $version }
welcome-update-available = Version { $version } är tillgänglig
welcome-update-tip = Öppnar nedladdningssidan i din webbläsare.
welcome-search = Sök projekt
welcome-open = Öppna
welcome-new-work = Nytt projekt
welcome-create-from = Skapa från...
welcome-recent-works = De senaste projekten
welcome-empty-recents = Inga nya projekt ännu.
welcome-no-matches = Inget nytt projekt matchar din sökning.
welcome-learn-soon = Guider och tips kommer snart.
welcome-about-blurb = Skribisto, en Rust + Teksilo skriva om appen.
welcome-tagline = Ett lugnt ställe att skriva långa saker.*
welcome-github = GitHub
welcome-discord = Discord
nav-works = Projekt
nav-examples = Exempel
nav-learn = Lär dig lära dig
nav-about = Om

synopsis = Synopsis
epigraph = Motto
pane-manuscript = Manuskript
corkboard = Anslagstavla
overview = Översikt

corkboard-card-count = { $count ->
    [one] { $count } kort
   *[other] { $count } kort
}
corkboard-child-count = { $count ->
    [one] { $count } objekt
   *[other] { $count } objekt
}
corkboard-view-nested = Hierarkisk
corkboard-view-flat = Platt
corkboard-layout-hint = Nested visar en behållares direkta barn. Öppna ett mappkort för att gå in i det. Flat visar varje scen i hela behållaren samtidigt.
corkboard-card-size = Kortstorlek
corkboard-search-placeholder = Filterkort...
corkboard-empty-title = Ingenting här än
corkboard-empty-hint = Använd "+ New" ovan för att lägga till den första.
corkboard-new = Ny
corkboard-show-card-numbers = Nummer korten
corkboard-modal-size = Expanderad redaktörsstorlek
corkboard-scope-hint = Gäller varje öppen korkbräda, i varje projekt.
corkboard-sort-manuscript = Manuskript order
corkboard-sort-title-asc = Titel A-Z
corkboard-sort-title-desc = Titel Z-A
corkboard-card-number = Kort { $number }

duplicate = Duplicera
duplicate-n = { $count ->
    [one] Duplicera { $count } kort
   *[other] Duplicera { $count } kort
}
set-label-n = { $count ->
    [one] Ange etikett på { $count } kort
   *[other] Ange etikett på { $count } kort
}
move-to-trash-n = { $count ->
    [one] Flytta { $count } kort till papperskorgen
   *[other] Flytta { $count } kort till papperskorgen
}
reveal-in-outline = Reveal i outline

corkboard-move-to = Flytta till...
corkboard-move-to-n = { $count ->
    [one] Flytta { $count } kort till...
   *[other] Flytta { $count } kort till...
}
corkboard-move-picker-title = Flytta till...
corkboard-move-picker-empty = Inga bindemedel ännu. Skapa en först.
corkboard-move-picker-cancel = Avbryt
corkboard-move-here = Flytta hit
corkboard-moved-ok = { $count ->
    [one] { $count } kort flyttat
   *[other] { $count } kort flyttade
}
corkboard-move-into-self = En behållare kan inte flyttas inuti sig själv. Välj en destination utanför den.
corkboard-move-failed = Dessa kort kunde inte flyttas dit.

overview-col-title = Titel
overview-col-type = Typ
overview-col-label = Kort beskrivning
overview-col-tags = Taggar
overview-col-books = Böcker
overview-col-own-words = Ord
overview-col-total-words = Totalt totalt
overview-row-count = { $count ->
    [one] { $count } objekt
   *[other] { $count } objekt
}
overview-search-placeholder = Filter objekt...
overview-expand-all = Expandera alla
overview-collapse-all = kollapsa alla
overview-table-label = Innehåll
overview-empty-title = Ingenting här än
overview-gone-title = Denna behållare är borta
overview-gone-hint = Den flyttades till papperskorgen. återställa den, eller stänga den här fliken.
overview-empty-hint = Använd "+ New" ovan för att lägga till den första.
corkboard-grid-label = Corkboard kort
corkboard-rename-field = Rename Item
corkboard-expand-synopsis = Expandera synopsis
corkboard-synopsis-modal-title = Synopsis
corkboard-badge-scene = Scen
corkboard-badge-chapter = Kapitel
corkboard-badge-part = Del
corkboard-badge-book = Bok
corkboard-badge-note = Anteckning
corkboard-badge-folder = Mapp
corkboard-badge-paratext = Paratext
corkboard-badge-text = Text
corkboard-badge-end = Slutet
settings-group-corkboard-layout = Layout
settings-group-corkboard-cards = Kort
corkboard-show-word-count = Visa ordräkning på kort
text-heading = Text
no-content = Det här objektet har inget redigerbart innehåll.
untitled = Namnlös
placeholder-title = Titel…
placeholder-subtitle = Undertitel...
placeholder-chapter-title = Kapitel titel...
split-editor = Dela redigeraren
close-split-view = Nära split view
drop-open-here = Öppna här
drop-open-to-side = Öppen till sidan

segment-chapter = Kapitel
segment-part = Del
segment-book = Bok
segment-pace = Skrivtempo
segment-notes = Anteckningar
segment-story-bible = Berättelsebibel
segment-note-own = Anteckning
segment-note-details = Detaljer
segment-note-in-prose = I prose
pace-placeholder = Pace Planner visas här.
pace-empty-title = Planera den här bokens takt
pace-empty-body = Ställ ett ord mål och en deadline, och Skribisto Projekt ut den dagliga takten för att få dig dit.
pace-start-planning = Börja planera
pace-section-schedule = Schema
pace-section-progress = Framsteg
pace-advancement = Avancemang
pace-goal = Ordmål
pace-deadline = Slutdatum
pace-active = Pace aktiv
pace-day-mon = Mon Mon Mon Mon Mon Mon Mon Mon
pace-day-tue = Tis
pace-day-wed = Ons
pace-day-thu = Tor
pace-day-fri = Fri Fri Fri Fri Fri
pace-day-sat = Sattar
pace-day-sun = Sol sol sol sol solen
pace-card-written = ord skrivna
pace-card-of-goal = av målet
pace-card-rate = Ord / skrivdag
pace-card-days-left = skrivande dagar kvar
pace-card-streak = Dag Streak
pace-card-ahead = Ord före takt
pace-card-behind = ord bakom tempo
pace-charts-empty = Framstegsdiagram visas här när ditt ord räknas in på en räddning.
pace-chart-progression = Ord skrivna vs. mål
pace-chart-words-per-day = Ord per dag
pace-series-actual = Faktiska
pace-series-target = Mål
pace-series-words-per-day = Ord/dag
pace-daily-target-line = Till och med takt: { $count ->
    [one] { $count } ord/dag
   *[other] { $count } ord/dag
}
pace-section-holidays = Semester
pace-section-milestones = Milstolpar
pace-holidays-none = Ingen semester. Varje schemalagd dag räknas.
pace-holiday-label = Semesternamn
pace-add-holiday = Lägg till
pace-remove = Ta bort
pace-milestones-none = Inga milstolpar ännu. Ställ in en på en del eller kapitel i inspektören.
full-chapter = Fullständigt kapitel
full-part = Full del
full-book = Full bok
full-synopsis = Full synopsis
rename = Byt namn
set-label = Ställ etikett
insert-scene = Infoga scenen
insert-chapter = Infoga kapitel
split-scene = Split Scen
move-up = Flytta upp
move-down = Flytta ned
merge-with-previous = Sammanslagning med tidigare
move-to-trash = Flytta till papperskorgen
rename-chapter = Rename kapitel
add-scene = Lägg till ny scen
add-chapter = Lägg till nytt kapitel
new-scene-title = Ny scen
placeholder-scene-name = Scennamn...
menu-cut = Klipp ut
menu-copy = Kopiera
menu-paste = Klistra in
menu-paste-unformatted = Klistra in utan formatering
menu-select-all = Markera allt
format-bold = Fet
format-italic = Kursiv
format-underline = Understruken
format-strikethrough = Genomstruken

binder = Dokumentträd
no-work = Inget projekt laddat
no-recent-works = Inga nya projekt
switcher-open-section = För närvarande öppen
switcher-recent-section = Nyligen
switcher-this-window = Detta fönster
open-project-title = Öppet projekt
open-project-question = Hur vill du öppna "{ $title }"?
open-in-new-window = Öppet i nytt fönster
open-here = Öppna här

binder-all = Alla Binders
binder-show-all = Visa alla bindemedel
binder-new = Ny bindemedel...
binder-default-name = Ny bindemedel
binder-item-count = { $count ->
    [one] { $count } objekt
   *[other] { $count } objekt
}
binder-search-placeholder = Filtrera skissen...
binder-search-scope = Sök alla bindemedel
binder-trash-confirm-title = Flytta bindemedel till skräp?
binder-trash-confirm-text = "{ $name }" och alla dess föremål kommer att flyttas till papperskorgen.

dialog-rename = Byt namn
dialog-new-binder = Ny bindemedel
dialog-set-label = Ställ etikett
dialog-new-scene = Ny scen
close-work-question = Spara ändringar innan projektet stängs?
quit-question = Spara ändringar innan du avslutar?
quit-save-work-question = Spara ändringar till { $title } innan du slutar?
unsaved-changes = Projektet har osparade ändringar.
new-work-unsaved-question = Spara ändringar innan du skapar ett nytt projekt?
open-work-unsaved-question = Spara ändringar innan du öppnar ett annat projekt?
switch-backup-discard-title = Kassera ändringar i denna backup?
switch-backup-discard-text = Ändringar i en säkerhetskopia kan inte sparas till den. Använd spara som eller återställa för att hålla dem, eller kassera dem och öppna det andra projektet.
switch-save-failed = Projektet kunde inte sparas, så det ersattes inte: { $error }
switch-save-not-started = Projektet kunde inte sparas, så det ersattes inte.
close-save-failed = Projektet kunde inte sparas, så det var inte stängt: { $error }
close-save-not-started = Projektet kunde inte sparas, så det var inte stängt.
save-not-started = Projektet kunde inte räddas.

could-not-open-work = Kunde inte öppna "{ $file }"
could-not-open-work-too-new = "{ $file }" behöver en nyare Skribisto
could-not-open-work-too-new-detail = Saved by Skribisto format { $written_by }; öppna det behöver format { $requires } eller nyare, och detta bygg stöder upp till format { $supported }. Update Skribisto för att öppna den.
could-not-open-work-importing = "{ $file }" skrivs av en import
could-not-open-work-importing-detail = En import skriver över detta projekt och har inte avslutats. Öppna det när importen är klar: öppnas nu, det skulle visa projektet importen ersätter, och nästa spara skulle skriva det projektet tillbaka över importen.
could-not-open-example = Kan inte öppna exempel: { $error }
could-not-open-link = Kunde inte öppna { $url }: { $error }
could-not-create-work = Kan inte skapa projekt: { $error }
saving-as-file = Spara som { $target }...
saving-as-folder = Spara som { $target }/...
saved-as = Säljs till { $target }
save-error = Kunde inte spara: { $error }
save-flush-failed = Besparingen gick igenom, men dessa dokument kunde inte överlämnas till den och är fortfarande osparade: { $titles } ({ $error })
backup-error = Kunde inte backa upp: { $error }
backing-up = Backing up...
backup-nothing-open = Inget projekt är öppet för att backa upp.
backup-already-running = En backup är redan på gång.
backup-complete = Backup komplett ({ $ok } sparad, { $skipped } redan ström)
backup-partial = Backup färdig: { $ok } sparad, { $failed ->
    [one] { $failed } destination misslyckades
   *[other] { $failed } destinationer misslyckades
}
backup-no-destination-title = Ingen backup plats tillgänglig
backup-no-destination-text = Ingen av de konfigurerade backup-destinationerna kan nås (till exempel kan en extern enhet anslutas). Anslut det i och retry, eller avsluta utan att säkerhetskopiera.
backup-failed-close-title = Säkerhetskopieringen kunde inte sparas
backup-failed-close-text = Ingen säkerhetskopia kan skrivas innan stängning. Varje destination misslyckades (enheten kan ha tagits bort, eller det kan vara full eller skrivskyddad).

backup-choice-title = Backup-fil
backup-choice-heading = Du har öppnat en backup kopia
backup-choice-subtitle = Detta är en point-in-time backup av ett projekt.
backup-choice-subtitle-dated = Backup tog { $date }.
backup-choice-body = Du kan öppna den och redigera fritt, men ändringar kan bara hållas med Save As. Den ursprungliga projektfilen är inte rörd. Eller återställa detta projekt till exakt denna säkerhetskopia.
backup-choice-open = Öppna backup
backup-choice-restore = Återställ detta projekt till denna punkt...
backup-choice-not-a-backup = Nej, öppna det normalt
backup-banner-title = Backup-kopia: ändringar kan inte sparas här
backup-banner-description = Använd Spara för att hålla dina redigeringar i en ny fil eller återställa för att ersätta det ursprungliga projektet med denna säkerhetskopia.
backup-banner-restore = Återställ…
backup-banner-save-as = Spara som...
backup-restore-original-missing = Kan inte hitta det ursprungliga projektet för att återställa över. Använd Spara för att hålla denna säkerhetskopia som ett nytt projekt istället.
backup-restore-close-elsewhere-title = Projekt öppna i ett annat fönster
backup-restore-close-elsewhere-text = Projektet du återställer är öppet i ett annat fönster. Stäng det där först, sedan retry.
backup-restore-focus-window = Fokusera detta fönster
backup-restore-confirm-title = Återställ denna backup?
backup-restore-confirm-text = Den aktuella versionen av projektet kommer att kopieras åt sidan som säkerhetskopiering innan den ersätts med den här.
backup-restore-confirm-ok = Återställ
backup-restore-error = Kunde inte återställa: { $error }
backup-restored-ok = Projektet återställs.
backup-restored-with-safety = Projektet återställdes. Din tidigare version sparades till { $path }.
close-backup-discard-title = Kassera ändringar i denna backup?
close-backup-discard-text = Ändringar i en säkerhetskopia kan inte sparas till den. Använd Spara för att hålla dem, eller kassera och stänga.
quit-backup-discard-title = Kassera förändringar och sluta?
quit-backup-discard-work-question = Kassera ändringar till { $title } och sluta?
quit-backup-discard-text = Ändringar i en säkerhetskopia kan inte sparas till den. Använd Spara för att hålla dem, eller kassera och sluta.
backup-nudge-text = Inga säkerhetskopior är inställda för detta projekt.
backup-nudge-action = Ställ in backups...

menu-backups-list = Säkerhetskopior… (&K)
backups-title = Säkerhetskopior
backups-loading = Scanning för backups...
backups-empty = Inga säkerhetskopior hittades för detta projekt ännu.
backups-open = Öppna
backups-reveal = Visa i filhanteraren
backups-delete = Ta bort denna backup
backups-delete-confirm-title = Ta bort denna backup?
backups-delete-confirm-text = ”{ $name }” tas bort permanent. Detta kan inte ångras.
backups-delete-error = Kan inte ta bort säkerhetskopian: { $error }
backups-refresh = Uppdatera
backups-close = Stäng

settings-page-backup = Backup standarder
settings-page-work-backup = Säkerhetskopior
settings-backup-general-title = Default backup inställningar
settings-backup-work-title = Säkerhetskopior för detta projekt
settings-backup-inherit = Använd de allmänna backup-inställningarna
settings-backup-inheriting = Detta projekt använder de allmänna säkerhetskopieringsinställningarna.
settings-backup-none-hint = Inga automatiska säkerhetskopior är inställda (varje utlösare är avstängd).
settings-backup-last = Sista backup: { $date }
settings-backup-last-never = Inga säkerhetskopior ännu.
settings-backup-open-list = Öppna säkerhetskopior lista...
settings-backup-triggers = När man backar upp
settings-backup-on-close = Tillbaka upp när du stänger projektet
settings-backup-on-open = Tillbaka upp när du öppnar projektet
settings-backup-interval = Back up periodiskt, varje
settings-backup-destinations = Backup destinationer
settings-backup-default-location = Standard plats
settings-backup-usage-since = { $count ->
    [one] { $count } backup · { $size } · äldsta { $oldest }
   *[other] { $count } säkerhetskopior · { $size } · äldsta { $oldest }
}
settings-backup-usage = { $count ->
    [one] { $count } backup · { $size }
   *[other] { $count } säkerhetskopior · { $size }
}
settings-backup-usage-empty = Ingenting hålls här ännu
settings-backup-usage-measuring = Mätning...
settings-backup-reveal-root = Show mapp
settings-backup-dest-none = Inga destinationer. Backups sparas bredvid projektet.
settings-backup-dest-remove = Ta bort
settings-backup-dest-add = Lägg till mapp...
settings-backup-dest-refresh = Uppdatera
settings-backup-retention = Hur många att hålla
settings-backup-retention-tiered = Stegvis
settings-backup-retention-keep-n = Håll sista N
settings-backup-retention-tip = Hur gamla säkerhetskopior beskärs.
settings-backup-retention-tip-more = Tiered håller en säkerhetskopia per timme för en dag, per dag i en vecka, per vecka i en månad, och per månad bortom, så den senaste historien förblir tät och gammal historia tunnar ut. "Keep förra N" håller helt enkelt de N senaste kopiorna. I båda lägena de senaste kopiorna (minst nedan) hålls alltid.
settings-backup-gfs-hourly = Timme (senast 24 h)
settings-backup-gfs-daily = Daglig (förra veckan)
settings-backup-gfs-weekly = Veckovis (förra månaden)
settings-backup-gfs-monthly = Månadsvis
settings-backup-keep-n = Nummer för att hålla
settings-backup-min-keep = Alltid hålla åtminstone
settings-backup-dedup = Hoppa över en backup när ingenting har förändrats

new-work-title = Nytt projekt
new-work-close = Stäng
new-work-name = Projektnamn
new-work-name-placeholder = Namnlös
new-work-author = Författare
new-work-author-placeholder = Optionell
new-work-format = Format
new-work-single-file = Enstaka fil
new-work-bundle = Samlad fil
new-work-convert-later = Du kan konvertera mellan format senare.
new-work-location = Plats
new-work-will-create = kommer att skapa
new-work-language = Standardspråk
new-work-language-hint = Tillämpad på nya texter spellcheck. Varje text kan bytas till ett annat språk.
new-work-template = Mall
new-work-template-none = Ingen
new-work-template-empty-novel = Tomma romaner
new-work-template-light-novel = Ljus Nyhet
new-work-template-novel = Roman
new-work-template-novel-in-parts = Nyheter i delar
new-work-template-notebook = Anteckningsbok
new-work-cancel = Avbryt
new-work-create = Skapa projekt
new-work-step-details = Detaljer
new-work-step-language = Språkstruktur
new-work-step-template = Mall
new-work-back = Tillbaka
new-work-next = Nästa
new-work-documents-title = Nytt projekt från dokument
new-work-step-import = Importera
new-work-create-and-import = Skapa import...
new-work-documents-next-title = Dina dokument kommer nästa
new-work-documents-next-body = Skapa projektet öppnar importguiden över den, där du väljer dokumenten, granskar strukturen Skribisto läser av dem och väljer var den landar. Ingenting är skrivet i projektet tills du bekräftar det där.
new-work-documents-no-template = Detta projekt börjar tomt med syfte: ingen mall, så de importerade dokumenten är det enda i det.
new-work-documents-chapter-scene-hint = Gäller för de kapitel som importen skapar. Du kan ändra den senare i projektets inställningar.
new-work-single-file-desc = En .skrib arkiv (zip). Portable, lätt att säkerhetskopiera.
new-work-bundle-desc = En mapp som håller varje texttillgång. Friendlier till versionskontroll.
new-work-template-none-count = tomma bindemedel
new-work-template-empty-novel-count = 1 kapitel
new-work-template-light-novel-count = 15 kapitel
new-work-template-novel-count = 20 kapitel
new-work-template-novel-in-parts-count = 3 delar, 24 kapitel
new-work-template-notebook-count = Free-form Notes
new-work-chapter-scene = Flat kapitel
new-work-chapter-scene-tip = Varje kapitel är ett enda objekt du skriver rakt in i, utan scener under det. Lämna detta för den klassiska layouten, där ett kapitel är en mapp: du skriver rakt in i det också, men det kan hålla scener också.
new-work-chapter-scene-tip-more = Du skriver i ett kapitel antingen sätt. Den enda skillnaden är om det kan *innehålla * scener. Skribisto bindeträd är organisatoriskt bara, så båda layouterna sammanställer samma bok, du kan blanda dem fritt, och Promote konverterar ett kapitel mellan de två utan att förlora ett ord.
new-work-name-required = Ange ett namn för projektet
new-work-name-invalid = Detta namn har inga användbara tecken
new-work-location-required = Välj en plats
new-work-location-missing = Denna mapp existerar inte
new-work-location-not-folder = Denna väg är inte en mapp
new-work-location-readonly = Denna mapp är inte skrivbar
new-work-target-exists = Ett projekt med detta namn är redan i den här mappen. Skapa projektet kommer att fråga innan du byter ut det.
new-work-target-is-folder = En mapp med detta namn är redan på denna plats. Välj ett annat namn.
new-work-overwrite-title = Ersätt det befintliga projektet?
new-work-overwrite-text = "{ $name }" finns redan i den här mappen. Byt ut den med det nya projektet?
new-work-target-open-title = Detta projekt är öppet
new-work-target-open-text = "{ $name }" är öppen i Skribisto. Skapa ett nytt projekt på sin plats skulle ersätta det projekt du arbetar med, och din nästa spara skulle skriva projektet tillbaka över det nya. Välj ett annat namn eller mapp, eller stäng projektet först.
target-importing-title = En import skriver detta projekt
target-importing-text = En import skriver fortfarande "{ $name }". Vänta tills den är klar eller välj ett annat namn.
import-plume-busy-title = Ytterligare Plume Creator-projekt importeras
import-plume-busy-text = Plume Creator projekt importeras en i taget. Vänta tills importen pågår har avslutats, eller avbryta den, sedan starta den här igen.
import-manuskript-busy-title = Ytterligare Manuskript-projekt importeras
import-manuskript-busy-text = Manuskript projekt importeras en i taget. Vänta tills importen pågår har avslutats, eller avbryta den, sedan starta den här igen.
backup-restore-importing-text = En import skriver fortfarande "{ $name }", projektet denna säkerhetskopia skulle återställas. Återställ säkerhetskopieringen när importen är klar.

new-work-manuscript = Manuskript
new-work-notes = Anteckningar
new-work-research = Forskning
new-work-notebook = Anteckningsbok
new-work-scene = Scen
new-work-note = Anteckning
new-work-front-matter = Inledande sidor
new-work-back-matter = Tillbaka materia
new-work-paratext = Bokstruktur
new-work-paratext-none = Ingen struktur
new-work-paratext-hint = Den främre och bakre delen en tradition öppnar och stänger en bok med. Du kan flytta, byta namn eller ta bort något av det efteråt.
new-work-tags = Taggar
new-work-tags-none = Inga taggar
new-work-tags-hint = En startpalett för att tagga tecken, platser och resten. Valfritt, och varje tag kan byta namn, luras eller tas bort efteråt.
new-work-note-templates = Notera mallar
new-work-note-templates-none = Inga mallar
new-work-note-templates-hint = Formen en berättelse-bibel anteckning börjar i. Valfritt, och varje mall kan redigeras eller tas bort efteråt.
new-work-characters = Personer
new-work-places = Platser

import-plume-title = Importera Plume Creator projekt
import-plume-close = Stäng
import-plume-source = Plume projekt
import-plume-source-hint = Välj en .plume eller .plume_backup fil (någon Plume Creator version).
import-plume-location = Destination mapp
import-plume-name = Filnamn
import-plume-name-placeholder = Projektnamn
import-plume-will-create = kommer att skapa
import-plume-trash-warning = • Spårade / borttagna objekt migreras inte.
import-plume-cancel = Avbryt
import-plume-import = Importera
import-plume-source-required = Välj en Plume projektfil
import-plume-source-missing = Denna fil existerar inte
import-plume-source-not-file = Denna väg är inte en fil
import-plume-location-required = Välj en destination mapp
import-plume-location-missing = Denna mapp existerar inte
import-plume-location-not-folder = Denna väg är inte en mapp
import-plume-location-readonly = Denna mapp är inte skrivbar
import-plume-name-required = Ange ett filnamn
import-plume-name-exists = En fil med detta namn finns redan här. Import kommer att bekräfta överskrivning
import-plume-overwrite-title = Ersätt befintlig fil?
import-plume-overwrite-text = "{ $name }" finns redan. Ersätt det med det importerade projektet?
import-plume-manuscript-binder = Manuskript
import-plume-story-bible-binder = Berättelsebibel
import-plume-progress-title = Importera Plume-projekt...
import-plume-cancel-import = Avbryt
import-plume-cancelled = Importera avbruten
import-plume-done = Importerad { $imported ->
    [one] { $imported } objekt.
   *[other] { $imported } objekt.
 } { $skipped ->
    [one] { $skipped } skräppost migrerades inte.
   *[other] { $skipped } skräpposter migrerades inte.
}
import-plume-open-now = Öppna nu
import-plume-warnings = { $count ->
    [one] 1 sak kan inte importeras exakt
   *[other] { $count } saker kan inte importeras exakt
}
import-plume-details = Detaljer
import-plume-warnings-title = Importera varningar
import-plume-error-title = Kan inte importera projektet
import-plume-nested-too-deep = Projektet var inte importerat: dess "{ $part }" del är kapslade mer än { $limit } nivåer djupt. No Plume Creator projektet kommer nära det, och läsa det kunde krascha Skribisto. Ingenting skrevs.
import-plume-declares-entities = Projektet importerades inte: dess "{ $part }" del förklarar XML-enheter, som ingen Plume Creator-projekt gör. Utöka dem kunde ta mer minne än datorn har och krascha Skribisto. Ingenting skrevs.
import-plume-error-details = Detaljer

import-manuskript-title = Importera Manuskript projekt
import-manuskript-close = Stäng
import-manuskript-source = Manuskript projekt
import-manuskript-source-hint = Välj .msk-filen eller projektmappen (någon Manuskript-version).
import-manuskript-source-file = Välj fil...
import-manuskript-source-folder = Välj mapp...
import-manuskript-location = Destination mapp
import-manuskript-name = Filnamn
import-manuskript-name-placeholder = Projektnamn
import-manuskript-will-create = kommer att skapa
import-manuskript-cancel = Avbryt
import-manuskript-import = Importera
import-manuskript-source-required = Välj ett Manuskript-projekt
import-manuskript-source-missing = Den här filen eller mappen finns inte
import-manuskript-location-required = Välj en destination mapp
import-manuskript-location-missing = Denna mapp existerar inte
import-manuskript-location-not-folder = Denna väg är inte en mapp
import-manuskript-location-readonly = Denna mapp är inte skrivbar
import-manuskript-name-required = Ange ett filnamn
import-manuskript-name-exists = En fil med detta namn finns redan här. Import kommer att bekräfta överskrivning
import-manuskript-overwrite-title = Ersätt befintlig fil?
import-manuskript-overwrite-text = "{ $name }" finns redan. Ersätt det med det importerade projektet?
import-manuskript-manuscript-binder = Manuskript
import-manuskript-story-bible-binder = Berättelsebibel
import-manuskript-characters-group = Personer
import-manuskript-world-group = Världen världen över
import-manuskript-plots-group = Handlingslinjer
import-manuskript-project-info-note = Projektinformation
import-manuskript-summary-note = Sammanfattning
import-manuskript-importance-minor = Mindre
import-manuskript-importance-secondary = Sekundär
import-manuskript-importance-main = Main Main
import-manuskript-progress-title = Importera Manuskript projekt...
import-manuskript-cancel-import = Avbryt
import-manuskript-cancelled = Importera avbruten
import-manuskript-done = { $imported ->
    [one] { $imported } varor importerade.
   *[other] { $imported } varor importerade.
 } { $revisions ->
    [0] { "" }
    [one] { $revisions } tidigare version kom med den.
   *[other] { $revisions } tidigare versioner kom med dem.
}
import-manuskript-open-now = Öppna nu
import-manuskript-warnings = { $count ->
    [one] 1 sak att veta om denna import
   *[other] { $count } saker att veta om denna import
}
import-manuskript-details = Detaljer
import-manuskript-warnings-title = Om denna import
import-manuskript-error-title = Kan inte importera projektet
import-manuskript-nested-too-deep = Projektet importerades inte: dess fil "{ $part }" är nedlagd mer än { $limit } nivåer djupt. No Manuskript projektet kommer nära det, och läsa det kunde krascha Skribisto. Ingenting skrevs.
import-manuskript-folders-too-deep = Projektet var inte importerat: en av dess filer sitter mer än { $limit } mappar djupt. No Manuskript projektet kommer nära det, och läsningen kunde krascha Skribisto. Ingenting skrevs.
import-manuskript-declares-entities = Projektet importerades inte: dess fil "{ $part }" förklarar XML-enheter, som ingen Manuskript-projekt gör. Utöka dem kunde ta mer minne än datorn har och krascha Skribisto. Ingenting skrevs.
import-manuskript-error-details = Detaljer

import-target-open-title = Detta projekt är öppet
import-target-open-text = "{ $name }" är öppet i Skribisto. Importera över det skulle ersätta det projekt du arbetar med, och din nästa spara skulle skriva det projektet tillbaka över importen. Välj ett annat filnamn eller stäng projektet först.
import-warnings-announcement = { $result } { $notice }.

export-title = Exportera
export-close = Stäng
export-scope-label = Vad är vad
export-format-label = Format
export-style-label = Stil
export-path-label = Fil
export-browse = Bläddra...
export-show-non-exportable = Visa icke-exportabel
export-choose-empty = Inget projekt att välja mellan.
export-cancel = Avbryt
export-export = Exportera
export-save-dialog-title = Exportera till fil
export-section-what = Vad att exportera
export-section-style = Stil
export-section-destination = Mål
export-custom-selection = Anpassat urval
export-selected-count = { $count } vald
export-chip-chapters-none = Kapitel: Ingen
export-chip-chapters-numbered = Kapitel: numrerad
export-chip-chapters-title = Kapitel: endast titel
export-chip-chapters-both = Kapitel: nummer + titel
export-chip-scene-break-glyph = Scenbrott: { $glyph }
export-chip-scene-break-blank = Scene break: blank linje
export-chip-scene-break-none = Scenbrott: ingen
export-chip-major-break-glyph = Stor paus { $glyph }
export-chip-major-break-blank = Major break: blank linje
export-chip-major-break-none = Större paus: ingen
export-chip-major-break-same = Båda bryta tiers lika
export-chip-spacing-single = Spacing: singel
export-chip-spacing-onehalf = Spacing: 11⁄2
export-chip-spacing-double = Spacing: Dubbel
export-chip-notes-included = Anteckningar som ingår
export-chip-notes-excluded = Anteckningar exkluderade
export-format-docx = Word Word Word Word Word
export-format-odt = LibreOffice
export-format-html = HTML
export-format-markdown = Markdown
export-format-djot = Djot
export-format-text = Text
export-format-latex = LaTeX
export-format-epub = EPUB
export-format-pdf = PDF
export-overwrite-title = Ersätt befintlig fil?
export-overwrite-text = "{ $name }" finns redan. Byt ut den?
export-progress-title = Exportera...
export-cancelled = Export inställd
export-done = Exporterad { $count ->
    [one] { $count } objekt
   *[other] { $count } objekt
}
export-open-file = Öppna
export-show-in-folder = Visa i mappen
export-error-title = Kunde inte exportera
export-error-details = Detaljer

backup-progress-start = Börjar...
backup-progress-retention = Rengöring av gamla säkerhetskopior...
backup-progress-done = Klar
backup-progress-destination = Destination { $i } av { $n }
backup-details = Detaljer
backup-issues-title = Backup-problem
backup-failed-title = Backup misslyckades
backup-complete-prune-warning = Säkerhetskopiering ({ $ok } sparad, { $skipped } redan strömmen). Vissa gamla säkerhetskopior kunde inte tas bort.

search = Sök
search-query-placeholder = Sök…
search-replace-placeholder = Ersätt med...
search-replace-toggle = Toggle ersätta
search-replace-all = Ersätter alla
search-opt-case = Skilj på stora och små bokstäver
search-opt-whole-word = Hela ordet
search-opt-diacritics = Match accenter
search-scope-body = Kropp
search-scope-title = Titel
search-scope-synopsis = Synopsis
search-scope-label = Kort beskrivning
search-facet-book = Böcker
search-facet-part = Delar
search-facet-chapter = Kapitlen
search-facet-scene = Scener
search-facet-note = Anteckningar
search-facet-folder = Mappar
search-tip-case = Matchfodral: behandla övre och nedre snitt som olika, så "Elena" och "elena" är separata matcher.
search-tip-whole-word = Hela ordet: matcha bara fullständiga ord, så "katt" finns inte inuti "kategori".
search-tip-diacritics = Match accenter: behandla accenterade bokstäver som distinkta, så "cafe" matchar inte "café".
search-tip-body = Kroppen: sök prosa av scener och anteckningar.
search-tip-title = Titel: Sök titlarna på bindeposter.
search-tip-synopsis = Synopsis: Sök varje skrivobjektets sammanfattning.
search-tip-label = Etikett: Sök den korta anteckningen som visas under en bindemedelspostens titel.
search-tip-comment = Kommentarer: Sök texten av kommentartrådar och deras svar. En kommentar handlar om manuskriptet snarare än en del av det, så det har en egen switch, och en ersättare lämnar kommentar matchar okontrollerade tills du kryssar dem.
search-tip-book = Böcker: Bokbehållaren och dess start / slutmarkörer.
search-tip-part = Delar: delnivå divider.
search-tip-chapter = Kapitel: Kapitel, oavsett hur projektet lagrar dem.
search-tip-scene = Scener: de objekt som håller din prosa.
search-tip-note = Anteckningar: friformsanteckningar.
search-tip-folder = Mappar: vanligt organisera mappar och separatorer.
search-tip-paratext = En text som hör till boken men inte till dess historia: ett förord, ett engagemang, ett efterord. räknas aldrig i manuskriptet.
search-tip-preserve-case = Bevara fallet: en ersättning tar fallet som den hittade, så "ELENA" blir "MARTA" och "Elena" blir "Marta".
search-tip-replace = Ersätt: visa ersättningsfältet och Byt ut allt.
search-error = Sök misslyckades: { $message }
search-no-matches = Inga matcher
search-count =
    { $matches ->
        [one] { $matches } match
       *[other] { $matches } matcher
    } in { $items ->
        [one] { $items } dokument
       *[other] { $items } dokument
    }
search-count-truncated =
    { $matches ->
        [one] { $matches } match
       *[other] { $matches } matcher
    } in { $items ->
        [one] { $items } dokument
       *[other] { $items } dokument
    } (endast första resultat)
search-occurrences = { $count }
search-field-body = Kropp
search-field-title = Titel
search-field-synopsis = Synopsis
search-field-label = Kort beskrivning
search-field-epigraph = Motto
search-field-comment = Kommentar
search-field-comment-reply = Svar
search-field-footnote = Fotnot
search-include-in-replace = Inkludera i ersätta alla
search-collapse-all = kollapsa alla resultat
search-replace-here = Ersätt detta
search-dismiss = Avfärda resultaten
search-undo-dismiss = Ta tillbaka det senaste avfärdade resultatet
search-replace-nothing = (ingenting)
search-replace-confirm-title = Ersätter alla matcher?
search-replace-confirm-text =
    Ersätt { $occurrences ->
        [one] { $occurrences } händelse
       *[other] { $occurrences } händelser
    } of "{ $query }" with "{ $replacement }" across { $items ->
        [one] { $items } dokument
       *[other] { $items } dokument
    } Du kan ångra detta från meddelandet efteråt.
search-replace-done-title = Ersätt komplett
search-replace-done =
    { $occurrences ->
        [one] { $occurrences } händelse
       *[other] { $occurrences } händelser
    } replaced in { $items ->
        [one] { $items } dokument
       *[other] { $items } dokument
    } .
search-replace-done-skipped =
    { $occurrences ->
        [one] { $occurrences } händelse
       *[other] { $occurrences } händelser
    } replaced in { $items ->
        [one] { $items } dokument
       *[other] { $items } dokument
    }. { $skipped ->
        [one] { $skipped } fält
       *[other] { $skipped } fält
    } hoppad (förändrad sedan sökningen).
search-replace-undo = Ångra
search-replace-failed-title = Ersätt misslyckad
search-replace-skipped-title = Ingenting ersatt
search-replace-skipped-body = Antal texter som har ändrats sedan sökningen: { $fields }. Dessa texter lämnades orörda. Sök igen för att se var orden finns nu.
search-replace-undo-failed-title = Undo misslyckades
search-preview = Förhandsvisning
search-preview-empty = Välj ett resultat för att förhandsgranska det här
search-preview-no-prose = Denna match har ingen redigerbar text
search-preview-prompt = För att se en förhandsvisning här, kör en sökning.
search-preview-open-search = Sök i Project
search-preview-footnote-prompt = Denna match är i en fotnot egen text. Öppna den i Footnotes dock för att se och redigera den.
search-preview-open-footnotes = Öppna fotnoter

find-placeholder = Hitta i detta dokument...
find-count = { $current } av { $total }
find-no-results = Inga resultat
find-close = Nära hitta
find-previous = Föregående match (Shift+Enter)
find-next = Nästa match (Enter)
find-opt-case = Skilj på stora och små bokstäver
find-opt-whole-word = Hela ordet
find-replace-toggle = Växelbyte (Ctrl+R)
find-replace-placeholder = Ersätt med...
find-replace = Ersätt
find-replace-all = Ersätt alla
find-preserve-case = Bevara fallet

dict-download-title = Ladda ner { $name }...
dict-download-done = Installerad { $name }
dict-download-failed = Kunde inte ladda ner { $name }: { $error }
dict-removed = Ta bort { $name }
dict-accept-first = Acceptera licensen innan du laddar ner { $name }
settings-dict-tab-installed = Installerad
settings-dict-tab-get-more = Få mer
settings-dict-tab-personal = Personliga ord
dict-installed-empty = Inga ordböcker finns på den här datorn ännu.
dict-get-more-search = Sök språk
dict-system-badge = på ditt system
dict-unusable-badge = Oanvändbar
dict-download-button = Ladda ner
dict-downloading = Ladda ner...
dict-installed-label = Installerad
dict-remove = Ta bort
dict-view-license = Visa licens
dict-approx-size = ~{ $size }
dict-personal-empty = Inga personliga ord i detta projekt ännu.
dict-personal-add = Lägg till
dict-personal-placeholder = Lägg till ett ord...
dict-missing-toast = { $count ->
    [one] Detta projekt använder { $count } ordbok du inte har installerat
   *[other] Detta projekt använder { $count } ordböcker som du inte har installerat.
}
dict-missing-action = Få ordböcker
dict-license-title = { $name } licens
dict-license-accept = Accept Download Ladda ner
dict-license-cancel = Avbryt
dict-license-close = Stäng
dict-add-button = Lägg till ordbok...
dict-add-title = Lägg till en ordbok
dict-add-close = Stäng
dict-add-name = Namn
dict-add-name-placeholder = t.ex. Min latinska ordbok
dict-add-code = Språkkod
dict-add-code-placeholder = t.ex. la eller fr-FR-x-custom
dict-add-code-hint = En kort tagg av ditt val. Detta är vad du väljer som ett dokuments språk.
dict-add-aff = Affix fil (.aff)
dict-add-dic = Ordlista (.d)
dict-add-submit = Lägg till
dict-add-cancel = Avbryt
dict-add-name-required = Ge ordboken ett namn.
dict-add-code-required = Ange en språkkod.
dict-add-code-invalid = Använd endast bokstäver, siffror och - _.
dict-add-code-reserved = Det är en inbyggd ordbokskod. Välj en annan.
dict-add-file-required = Välj en fil.
dict-add-file-missing = Den filen existerar inte.
dict-add-code-taken = En ordbok för den koden är redan installerad.
dict-add-done = Tillagd { $name }
dict-add-unusable = Dessa filer är inte en användbar ordbok: { $error }
dict-add-failed = Kunde inte lägga till ordboken: { $error }
inspector-export = Exportera
inspector-exportable = Inkludera export
ctx-number = Nummer detta kapitel (&N)
ctx-unnumber = Anmäl inte detta kapitel (&N)
inspector-numbering = Nummer
inspector-numbered = Numrerad
inspector-numbered-tip = Detta kapitel tar sin plats i bokens numrering. Växla bort det för en prolog, en epilog eller en interlude.
inspector-numbered-tip-more = Ett oräkneligt kapitel stannar i boken precis som det var: dess rubrik, dess prosa och dess ordräkning är orörd. Det trycker helt enkelt inget nummer, och använder inte en upp: kapitlet efter en prolog är kapitel ett, inte kapitel två. Att lämna ut den ur exporten istället är en annan växel, ovan, och att man tar bort kapitlet från boken helt och hållet.
inspector-apply-to-children = Ansök till barn
inspector-milestone = Milestone datum
inspector-milestone-none = Inget datum
inspector-milestone-clear = Rensa
inspector-tags = Taggar
inspector-aliases = Även känd som
inspector-dict-language = Språk
inspector-apply-language-to-children = Applicera språk till barn
settings-page-language = Språk
settings-field-dict-language = Språk
dict-tradeoff-hint = Varje extra språk accepterar fler ord, så färre misstag fångas.
lang-inherit-hint = Ärftlig: denna scen använder bokens eller projektets språk.
lang-pill-list = Språk
lang-pill-add = Lägg till ett språk
lang-pill-remove = Ta bort { $name }
lang-pill-mute = Slå av stavningskontroll för { $name }
lang-pill-unmute = Turn spell-checking på för { $name }

settings-page-personal-dictionary = Personlig ordbok
settings-user-dict-desc = Ord som du lägger till här behandlas som korrekt stavade överallt i appen och flaggas aldrig igen.
settings-user-dict-add-placeholder = Lägg till ett ord...
settings-user-dict-add = Lägg till ord
settings-user-dict-search = Filtrera ord
settings-user-dict-count = { $count ->
    [one] { $count } ord
   *[other] { $count } ord
}
settings-user-dict-rename = Byt namn
settings-user-dict-remove = Ta bort
settings-user-dict-empty = Inga ord ännu. Lägg till ord stavningschecker bör ignorera, eller importera en lista.
settings-user-dict-import = Importera…
settings-user-dict-export = Exportera…
settings-user-dict-import-tip = Lägg till ord från en vanlig textfil, ett ord per rad. Befintliga ord hålls.
settings-user-dict-export-tip = Spara hela listan till en vanlig textfil, ett ord per rad.
settings-user-dict-txt-filter = Textfiler
settings-user-dict-duplicate = Redan i ordboken
settings-user-dict-imported = Importerad { $count ->
    [one] { $count } ord
   *[other] { $count } ord
} ({ $duplicates } finns redan nu).
settings-user-dict-import-failed = Kan inte läsa ordlistan: { $error }
settings-user-dict-exported = Sparad { $count ->
    [one] { $count } ord.
   *[other] { $count } ord.
}
settings-user-dict-export-failed = Kunde inte spara ordlistan: { $error }

settings-page-text-replacements = Textbyten
settings-text-repl-desc = Ersätt korthand med den fullständiga texten när du skriver: "btw" blir "förresten" det ögonblick du skriver ett utrymme eller skiljetecken.
settings-text-repl-enable = Använd textbyten i detta projekt
settings-text-repl-disabled-hint = Slå på detta för att definiera shorthand som expanderar när du skriver.
settings-text-repl-add = Lägga till regel
settings-text-repl-trigger-placeholder = Förkortning
settings-text-repl-replacement-placeholder = Vad det blir
settings-text-repl-added = Tillagd "{ $trigger }"
settings-text-repl-duplicate = "{ $trigger }" har redan en regel
settings-text-repl-filter = Filterregler
settings-text-repl-count = { $n ->
    [one] 1 regel
   *[other] { $n } regler
}
settings-text-repl-row-enabled = Använd denna regel
settings-text-repl-delete = Ta bort regeln för { $trigger }
settings-text-repl-deleted = Radera regeln för "{ $trigger }"
settings-text-repl-empty = Inga regler ännu.
settings-text-repl-csv-filter = CSV filer
settings-text-repl-import = Importera…
settings-text-repl-export = Exportera…
settings-text-repl-imported = Importerad { $added }, hoppad { $skipped }
settings-text-repl-exported = Exporterad { $n ->
    [one] 1 regel
   *[other] { $n } regler
}

editor-menu-no-suggestions = Inga förslag
editor-menu-add-to-dictionary = Lägg till "{ $word }" till ordbok
editor-menu-add-words-to-dictionary = Lägg till utvalda ord till ordbok
editor-dict-added = Tillagd "{ $word }" till din ordbok.
editor-dict-added-multi = { $count ->
    [one] Tillagt { $count } ord till din ordbok.
   *[other] Tillagd { $count } ord till din ordbok.
}
toast-undo = Ångra

undo-superseded-title = Det steget är inte längre den sista
undo-superseded-body = Något annat har förändrats i det här projektet sedan. Undo skulle ta det tillbaka istället, så det har inte. Inget det här steget har vänts. Använd redigera - Undo för att gå tillbaka genom historien själv.
undo-failed = Undo misslyckades: { $error }

menu-edit = Redigera (&E)
menu-edit-undo = Ångra (&U)
menu-edit-redo = Gör om (&R)
menu-edit-undo-target = Undo { $target } (&U)
menu-edit-redo-target = Redo { $target } (&R)
shortcut-name-edit-undo = Ångra
shortcut-name-edit-redo = Gör om
undo-target-typing = Typing
undo-target-project = Den sista förändringen i detta projekt
prose-history-reset-title = Typer historia återställning
prose-history-reset-body = { $count ->
    [one] En öppen scen restaurerades från ett tidigare tillstånd, så dess skrivarhistoria gäller inte längre.
   *[other] { $count } öppna scener restaurerades från ett tidigare tillstånd, så deras skrivhistorik gäller inte längre.
}
undo-target-trash = flytta till papperskorgen
undo-target-restore = Återställande från papperskorgen
undo-target-delete-forever = Radera för gott
undo-target-replace-all = Ersätter över hela projektet
undo-target-import = Dokumentimport
undo-target-duplicate = Duplicera
undo-target-move = Att flytta
undo-target-merge = sammanslagning av två scener
undo-target-split = Att dela en scen
undo-target-promote = ändra typ
undo-target-tidy-titles = Titta på kapitlet titlar
undo-target-import-tags = importera etiketter
undo-target-import-templates = importera anteckningsmallar
undo-target-create = Att skapa
undo-target-remove = Att ta bort
undo-target-rename = Reaming
undo-target-edit = Den edit
menu-edit-find = Sök… (&F)
menu-edit-find-next = Hitta nästa (&N)
menu-edit-find-prev = Hitta tidigare (&V)
menu-edit-replace = Hitta och ersätta... (&L)
menu-edit-find-in-project = Hitta i projekt... (&J)
menu-edit-replace-in-project = Ersätt i projekt... (&I)
undo-frozen = Undo ("Alltid framåt" är på)
redo-frozen = Redo ("Alltid framåt" är på)
menu-edit-cut = Klipp ut (&T)
menu-edit-copy = Kopiera (&C)
menu-edit-paste = Klistra in (&P)
menu-edit-paste-plain = Klistra in utan formatering (&W)
menu-edit-select-all = Markera allt (&A)
shortcut-name-edit-cut = Klipp ut
shortcut-name-edit-copy = Kopiera
shortcut-name-edit-paste = Klistra in
shortcut-name-edit-paste-plain = Klistra in utan formatering
shortcut-name-edit-select-all = Markera allt

titlebar-spellcheck-on = Spell-checking är på. Klicka för att sluta kontrollera (F7)
titlebar-spellcheck-off = Spell-checking är avstängd. Klicka för att kontrollera igen (F7)
menu-spellcheck = Kontroll stavning (&C)
menu-comments = Kommentarer (&M)
settings-page-spellcheck = Stavningskontroll
settings-group-spellcheck = Stavningskontroll
settings-spellcheck-enabled = Kontrollera stavning när jag skriver
settings-spellcheck-hint = Understryker ord som ingen installerad ordbok vet. Att stänga av detta stoppar all kontroll, i varje projekt, tills du slår på det igen. För att sluta kontrollera ett språk bara, rensa kontrollen på projektets språkfält eller ett objekt.

menu-trash = Papperskorg (&T)
trash-title = Papperskorg
trash-empty-state = Trash är tom.
trash-empty-button = Tom Trash...
trash-restore = Återställ (&R)
trash-restore-to = Återställ till... (&T)
trash-delete-forever = Radera för alltid (&D)
trash-restored-ok = { $count } återställdes.
trash-restore-error = Kunde inte återställa: { $error }
trash-restore-orphaned = Det här objektets ursprungliga plats är borta. Välj var du ska återställa den.
trash-restore-no-project = Inget projekt är öppet, så det finns inget att återställa.
trash-delete-no-project = Inget projekt är öppet, så det finns inget att ta bort.
trash-empty-confirm-title = Tomma papperskorgen?
trash-empty-confirm-text = { $count ->
    [one] { $count } papperskorgen kommer att raderas permanent.
   *[other] { $count } skräpposter raderas permanent.
} De lämnar papperskorgen för gott. Undo kan fortfarande ta dem tillbaka tills du gör något annat.
trash-emptied-title = Trash tömd
trash-emptied-body = Allt i papperskorgen raderades permanent.
trash-empty-no-project = Inget projekt är öppet, så det finns ingen papperskorg till tomt.
trash-delete-forever-confirm-title = Ta bort för alltid?
trash-delete-forever-confirm-text = { $count ->
    [one] { $count }-objektet raderas permanent.
   *[other] { $count } objekt raderas permanent.
} De lämnar papperskorgen för gott. Undo kan fortfarande ta dem tillbaka tills du gör något annat.
trash-deleted-title = Tas bort för alltid
trash-deleted-body = { $count } raderades permanent.
trash-undo = Ångra
trash-restore-picker-title = Återställ till...
trash-restore-to-confirm-title = Återställ här?
trash-restore-to-confirm-text = Återställ "{ $item }" till "{ $destination }"?
trash-restore-picker-restore-here = Återställ här
trash-restore-picker-cancel = Avbryt
trash-restore-picker-empty = Inga bindemedel ännu. Skapa en först.
trash-banner-title = Detta objekt är i papperskorgen
trash-banner-description = Det kommer inte att visas i konturen eller exporten tills du återställer den.
trash-banner-restore = Återställ…
trash-tab-tooltip = I Trash

settings-styles-sheet-yes = Ja
settings-styles-sheet-no = Nej
settings-styles-sheet-font = Teckensnitt
settings-styles-sheet-indent = Första-line indent
settings-styles-sheet-para-spacing = Punktavstånd
settings-styles-sheet-page = Sidstorlek
settings-styles-sheet-margins = Marginaler (T/R/B/L)
settings-styles-sheet-title-page = Bok titel sida
settings-styles-sheet-heading-language = Rubrik språk
settings-styles-sheet-auto = Följer texten
settings-styles-sheet-digits = Siffror
settings-styles-sheet-direction = Text riktning
settings-styles-sheet-formats = Format
settings-styles-sheet-all-formats = Allt allt allt

settings-styles-editor-group = Stil redaktör
settings-styles-editor-missing = Den stilen är inte längre tillgänglig.

settings-styles-page-letter = Brev
settings-styles-digits-western = Västerlandet (0–9)
settings-styles-digits-eastern-arabic = Östra arabiskan (?-)
settings-styles-direction-ltr = Vänster till höger
settings-styles-direction-rtl = Höger till vänster

format-dock-title = Format
format-panel-empty = Placera din markör i en scen, notera eller synopsis för att se formateringsalternativ.
format-group-marks = Text
format-group-block = Punkt
format-group-lists = Listor
format-group-tables = Bord
format-group-breaks = Scenbrott
format-superscript = Upphöjd
format-subscript = Prenumerera
format-link = Länk...
link-dialog-insert-title = Infoga länk
link-dialog-edit-title = Redigera länk
link-dialog-text-label = Text
link-dialog-text-placeholder = Orden läsaren ser
link-dialog-url-label = Länkar till
link-dialog-url-placeholder = Exempel.com
link-dialog-insert = Infoga
link-dialog-apply = Verkställ
link-dialog-cancel = Avbryt
link-dialog-remove = Ta bort länk
link-scheme-refused = Endast webb- och e-postlänkar kan öppnas. { $url } lämnades ensam.
menu-format-link = Länk... (&N)
format-clear = Tydlig formatering
format-heading = Rubriknivå
format-heading-normal = Vanlig text
format-heading-1 = Rubrik 1
format-heading-2 = Rubrik 2
format-heading-3 = Rubrik 3
format-heading-4 = Rubrik 4
format-heading-5 = Rubrik 5
format-heading-6 = Rubrik 6
format-align-left = Align vänster
format-align-center = Center Center Center
format-direction-rtl = Höger-till-vänster punkt
format-blockquote = Blockcitat
format-list-bullet = Bulleted List
format-list-numbered = Numrerad lista
format-indent = Öka indrag
format-outdent = Minska indrag
format-table-insert = Infoga tabell
format-table-row-above = Infoga rad ovan
format-table-row-below = Infoga rad nedan
format-table-col-before = Infoga kolumn innan
format-table-col-after = Infoga kolumn efter
format-table-row-delete = Ta bort rad
format-table-col-delete = Radera kolumn
format-table-remove = Ta bort tabellen

menu-format-marks-bold = Fet (&D)
menu-format-marks-italic = Kursiv (&I)
menu-format-marks-underline = Understruken (&U)
menu-format-marks-strike = Genomstruken (&S)
menu-format-marks-superscript = Upphöjd (&P)
menu-format-marks-subscript = Prenumerera (&B)
menu-format-marks-clear = Tydlig formatering (&C)
menu-format-heading = Heading (&H)
menu-format-heading-normal = Vanlig text (&N)
menu-format-heading-1 = Rubrik 1 (&1)
menu-format-heading-2 = Rubrik 2 (&2)
menu-format-heading-3 = Rubrik 3 (&3)
menu-format-heading-4 = Rubrik 4 (&4)
menu-format-heading-5 = Rubrik 5 (&5)
menu-format-heading-6 = Rubrik 6 (&6)
menu-format-alignment = Inriktning (&A)
menu-format-align-left = Align vänster (&L)
menu-format-align-center = Center Center Center (&C)
menu-format-direction = Riktning (&E)
menu-format-direction-auto = Automatisk (&A)
menu-format-direction-ltr = Vänster till höger (&L)
menu-format-direction-rtl = Rätt till vänster (&R)
menu-format-blockquote = Blockcitat (&Q)
menu-format-lists = Listor (&L)
menu-format-list-bullet = Bulleted List (&B)
menu-format-list-numbered = Numrerad lista (&N)
menu-format-indent = Öka indrag (&I)
menu-format-outdent = Minska indrag (&O)
menu-format-table = Bord (&T)
menu-format-table-insert = Infoga tabell (&I)
menu-format-table-2x2 = 2 x 2 (&2)
menu-format-table-3x3 = 3 x 3 x 3 (&3)
menu-format-table-4x4 = 4 x 4 (&4)
menu-format-table-row-above = Infoga Row ovanför (&V)
menu-format-table-row-below = Infoga Row nedan (&W)
menu-format-table-col-before = Infoga kolumn innan (&L)
menu-format-table-col-after = Infoga kolumn efter (&T)
menu-format-table-row-delete = Ta bort rad (&D)
menu-format-table-col-delete = Radera kolumn (&C)
menu-format-table-remove = Ta bort tabellen (&R)

menu-format-typo-marks = Formatera Mark (&F)
menu-format-typo-nbsp = No-break Space (&N)
menu-format-typo-nbhyphen = No-break Hyphen (&H)
menu-format-typo-soft-hyphen = Soft Hyphen (&S)
menu-format-typo-nnbsp = Narrow No-break Space (&R)
menu-format-typo-zwsp = Zero-width Space (&Z)
menu-format-typo-word-joiner = Word Joiner (&W)


menu-about = Omkring Skribisto... (&A)
about-title = Omkring Skribisto
about-version = Version { $version }
about-update-available = Version { $version } är tillgänglig
about-update-see-notes = Se vad som förändrats
about-update-flathub = Flathub håller denna kopia uppdaterad.
about-update-distro = Din distribution håller denna kopia uppdaterad.
about-tagline = En nyskrivande app för långformad fiktion, skriven i Rust med Teksilo-verktyget.
about-license = Släppt under GNU General Public License, version 3.
about-copyright = © 2026 Cyril Jacquet
about-close = Stäng

native-menu-about = Omkring { $app }
native-menu-hide = Dölj { $app }
native-menu-quit = Avsluta { $app }
native-menu-settings = Inställningar…
native-menu-window = fönster
native-menu-minimize = Minimera
native-menu-zoom = Zooma

window-title = { $title } - { $app }
window-title-numbered = { $title } - { $app } (Fönster { $n })
window-title-empty = { $app }

first-run-window-title = Välkommen Välkommen
first-run-title = Ställ in { $app }
first-run-body = { $app } håller sina inställningar separat från Skribisto, så det börjar tomma. Dina preferenser, nya projekt, ordböcker och fönsterlayout kan kopieras över nu.
first-run-from = Kopiera från
first-run-to = Kopiera till
first-run-copy-note = Ingenting flyttas eller tas bort. Skribisto håller var och en av sina egna inställningar och fortsätter att fungera precis som tidigare.
first-run-import = Importera inställningar
first-run-start-fresh = Börja färskt
first-run-import-failed = Vissa inställningar kan inte importeras: { $error }

settings-page-punctuation = Punctuation Standards
settings-group-punctuation = Smart Punctuation
settings-page-work-punctuation = Skiljetecken
settings-punctuation-override = Ge detta projekt sina egna skiljetecken
settings-punctuation-override-hint = Av: projektet följer ansökningspreferensen. Dessa regler reser inuti .skrib, så en medförfattare som öppnar filen skriver med samma typografi.
settings-punctuation-dashes = Förvandla - till en en dash - till en em dash
settings-punctuation-ellipsis = Förvandla ... till en ellipsis
settings-punctuation-quotes = Curl citatmärken och apostroper
settings-quote-style = citatmärken
settings-quote-style-locale = Språkstandard
settings-quote-style-curly = "Dubbel"
settings-quote-style-curly-single = "Single"
settings-quote-style-guillemets = "Guillemets"
settings-quote-style-low-high = "Low-High"
settings-punctuation-spacing = Rymd före ; : ! ?
settings-punctuation-spacing-hint = Fransk typografi sätter en smal no-break utrymme innan ; ! ? och en full före: Det gäller endast text skriven på franska. Utrymmet inuti guillemets « kommer med guillemet citat själva.
settings-punctuation-sample = Ditt språk ger
settings-punctuation-app-hint = Vad varje projekt gör såvida det inte tar egna regler i projektet.
settings-punctuation-dialogue = Öppna ett stycke som är skrivet som "-" med en dialogdash
settings-punctuation-dialogue-hint = För språk som markerar tal med en streck snarare än citatmärken: franska, spanska, ryska och andra. Det brinner bara i början av en paragraf.

statusbar-go-to = Gå till...
go-to-placeholder = Sök bindemedel
go-to-no-matches = Inget objekt matchar den sökningen.
menu-go-to = Gå till... (&G)

settings-page-distraction-free-themes = Distraktionsfria teman
settings-themes-builtin = Inbyggda teman
settings-themes-builtin-badge = Inbyggd
settings-themes-user = mina teman
settings-themes-editor-group = Redigera tema
settings-themes-editor-empty = Välj ett tema under "Mina teman" för att redigera det.
settings-themes-use = Användning
settings-themes-duplicate = Duplicera
settings-themes-edit = Redigera
settings-themes-delete = Ta bort
settings-themes-export = Exportera…
settings-themes-import = Importera…
settings-themes-copy-suffix = Kopiera
settings-themes-json-filter = Tema (JSON)
settings-themes-imported = Tema importerat
settings-themes-import-failed = Kunde inte importera detta tema
settings-themes-exported = Tema exporterat
settings-themes-export-failed = Kunde inte exportera detta tema
settings-themes-low-contrast = låg kontrast
settings-themes-low-contrast-band = Höjdpunkt döljer texten
settings-themes-field-name = Namn
settings-themes-field-paper = Sidan
settings-themes-field-ink = Text
settings-themes-field-general = Bakgrund
settings-themes-field-widget-text = Kontrollband text
settings-themes-field-caret-band = Höjdpunkt runt vården

statusbar-focus-settings = Distraktionsfria inställningar
statusbar-focus-manage-themes = Hantera teman...

comments-title = Kommentarer
comments-document-title = Detta dokument
comments-empty-project = Inga kommentarer i detta projekt ännu.
comments-empty-document = Inga kommentarer till detta dokument.
comments-filter-all = Allt allt allt
comments-filter-open = Öppna
comments-filter-resolved = Lösning
comments-filter-orphaned = Föräldralös
comments-status-open = Öppna
comments-status-resolved = Lösning
comments-status-orphaned = Förlorat sin text
comments-orphan-snippet = (Den kommenterade texten är borta)
comments-status-unplaced = Ingen textposition
comments-unplaced-snippet = (inte förankrad till någon text)
comments-reply-count = { $count ->
    [one] { $count } svar
   *[other] { $count } svar
}
comments-menu-resolve = Lösning
comments-menu-reopen = Öppna igen
comments-menu-delete = Ta bort
comments-sort-document = I Dokumentorder
comments-sort-newest = Nyaste första
comments-sort-reader = Grupperad av läsare
comments-unsigned-author = Utan författarnamn
comments-agreement = { $count ->
    [one] { $count } läsare
   *[other] { $count } läsare
}
comments-menu-add = Lägg till kommentar
comments-menu-add-paragraph = Kommentar om denna punkt
overview-col-comments = Kommentarer
overview-col-total-comments = Totala kommentarer
comments-card-placeholder = Skriv en kommentar...
comments-card-unknown-author = Okänd författare
comments-unsigned-toast = Dina kommentarer är osignerade: inget namn är inställt på den här datorn.
comments-unsigned-action = Ställ ditt namn
comments-card-reply = Svar
comments-card-reply-placeholder = Svara...
comments-card-actions = Kommentera åtgärder
comments-reply-actions = Svara på åtgärder
comments-menu-delete-reply = Radera svar
comments-menu-delete-all = Ta bort alla kommentarer här
comments-deleted-toast = Kommentar borttagen
comments-reply-deleted-toast = Reply raderad
comments-deleted-all-toast = { $count ->
    [one] { $count } kommentar raderad
   *[other] { $count } kommentarer raderade
}
comments-undo = Ångra

analysis-segment = Analys
analysis-scope-book = Analysera denna bok
analysis-run = Kör analys
analysis-stale = Ändras sedan detta sprang
analysis-not-run = Inte analyserat ännu.
analysis-running = Läsa manuskriptet...
analysis-failed = Analysen kunde inte avslutas.
analysis-no-scenes = Inga scener i den här boken ännu.

analysis-shape = Form

analysis-arrivals = Ankomster
analysis-arrivals-explainer = Hur texten nådde detta projekt medan det har varit öppet. Det står vilken väg karaktärerna kom ner, och ingenting alls om vem som skrev dem: en författare som utarbetar någon annanstans och pastor har klistrat, och en som dikterar har dikterat. Det finns inget nummer att sträva efter här.
analysis-arrivals-scope = Hela projektet, inte bara den här boken.
analysis-arrivals-session = Sedan du öppnade det. Stängning av projektet startar räkningen igen.
analysis-arrivals-nothing = Ingen text har kommit ännu i denna session.
analysis-arrivals-typed = Typerad
analysis-arrivals-pasted = Inklistrat
analysis-arrivals-dictated = Dikterad
analysis-arrivals-imported = Importerad
analysis-arrivals-programmatic = Infogad för dig
analysis-arrivals-count = { $count ->
    [one] { $count } tecken
   *[other] { $count } tecken
}
analysis-arrivals-none = Ingen

analysis-words-per-scene = Ord per scen
analysis-median-words = Den här bokens median scen löper { $count ->
    [one] { $count } ord.
   *[other] { $count } ord.
}
analysis-median-line = Median: { $count ->
    [one] { $count } ord
   *[other] { $count } ord
}
analysis-dialogue = Dialog
analysis-dialogue-unsupported = Dialogen mäts inte för detta språk än.

analysis-footnote-words = Fotnot ord
analysis-footnote-words-count = { $count ->
    [one] { $count } ordet är i denna boks fotnoter, hålls borta från manuskriptet totalt.
   *[other] { $count } ord finns i denna boks fotnoter, hålls borta från manuskriptet totalt.
}
analysis-footnote-words-pending = Räkna fotnoterna...

analysis-ignore-empty = Ignorera texter med inget skrivet än
analysis-empty-hidden = { $count } tom { $count ->
        [one] Text
       *[other] Texter
    } gömd.
analysis-all-texts-empty = Varje text i den här boken är fortfarande tom.

binder-filter-count = { $shown } av { $total } visas
binder-filter-clear = Rensa
binder-filter-none = Ingenting matchar "{ $query }".

settings-paratext-intro = Den främre och bakre delen ett nytt projekt kan börja med. Varje struktur tillhör en förlagstradition, och dess sidtitlar är skrivna i den traditionens eget språk. Namnge dem fritt när ett projekt skapas.
settings-paratext-structures = Strukturer
settings-paratext-broken = Kunde inte läsas
settings-paratext-edit = Redigera
settings-paratext-duplicate = Duplicera
settings-paratext-delete = Ta bort
settings-paratext-new = Ny struktur
settings-paratext-save = Spara
settings-paratext-editor-hint = Ett namn, sidorna för att skapa före manuskriptet och sidorna för att skapa efter det. Där de hamnar är dina: de är vanliga objekt när projektet finns.


image-insert = Infoga bild... (&M)
image-no-project = Öppna ett projekt innan du sätter in en bild.
image-choose-title = Välj en bild
image-filter-label = Bilder
image-large-title = Detta är en stor bild
image-large-text =
    { $name } är { $megapixels } megapixlar ({ $width }×{ $height }).
    Hålla det som det lagrar din ursprungliga fil i projektet: det reser
    med varje backup och varje export. Optimering lagrar en mindre kopia
    upp till 2560 pixlar på sin längsta sida.
image-large-keep = Håll original
image-large-downscale = Optimera
image-large-remember = Gör detta från och med nu, fråga inte igen
image-not-recorded = Bilden sparades men kunde inte registreras i projektet.
image-describe-title = Beskriv bilden
image-describe-explain = Vad bilden visar, för en läsare som inte kan se det. Det är inte en del av manuskriptet: det räknas aldrig, söks eller exporteras som prosa.
image-describe-placeholder = En fyr mot en grå himmel
image-resize-title = Storlek på bilden
image-resize-explain = En procentandel av storleken visas nu. 100 lämnar den som den är.
image-resize-invalid = Ange ett nummer mellan 1 och 1000.
image-menu-describe = Beskriv bilden... (&D)
image-menu-resize = Storlek på bilden... (&R)
image-menu-reset-size = Originalstorlek (&Z)

cover-choose = Bokomslag... (&V)
cover-clear = Ta bort omslaget (&R)
cover-choose-title = Välj ett cover
cover-set = Omslaget är inställt.
cover-cleared = Omslaget har tagits bort. Bilden är fortfarande i projektet.
card-label = Kort beskrivning
card-exportable = Exporterad
card-numbered = Numrerad
card-created = Skapad
card-modified = Modifierad
card-yes = Ja
card-no = Nej
card-type = Typ
card-position = Position
card-children = Barnbarn
card-goal = Mål
card-words = Ord
card-synopsis = Synopsis
card-point-of-view = Perspektiv
card-aliases = Även känd som

export-orphan-footnotes-title = Fotnoter utan referens
export-comments-dropped =
    { $count ->
        [one] En kommentar kunde inte placeras i den exporterade texten och lämnades ut.
       *[other] { $count } kommentarer kunde inte placeras i den exporterade texten och lämnades ut.
    }
export-orphan-footnotes =
    { $count ->
        [one] En fotnot refereras inte längre någonstans i manuskriptet. Texten visas inte i den exporterade boken.
       *[other] { $count } fotnoter refereras inte längre någonstans i manuskriptet. Deras text visas inte i den exporterade boken.
    }


menu-footnotes = Fotnoter (&N)
footnotes-title = Fotnoter
footnotes-insert = Infoga fotnot (&F)
footnotes-empty = Inga fotnoter ännu. Sätt markören i en scen, använd sedan + ovan, eller Ctrl+Alt+F.
footnotes-filter-all = Allt allt allt
footnotes-filter-document = Detta dokument
footnotes-filter-orphaned = Föräldralös
footnotes-orphaned = Ingenting pekar på denna anteckning längre
footnotes-untitled-home = Namnlös
footnotes-body-placeholder = Notera sig själv
footnotes-insert-tooltip = Sätt in en fotnot på markören (Ctrl+Alt+F)
footnotes-actions = Footnote åtgärder
footnotes-delete = Delete note och dess referens (&D)
footnotes-no-project = Öppna ett projekt innan du sätter in en fotnot.
footnotes-no-caret = Sätt markören i en scen text för att infoga en fotnot där.
footnotes-not-created = Fotnoten kunde inte läggas till i projektet.
footnotes-deleted-toast = Observera raderad
footnotes-undo-delete = Ångra

import-document-title = Importera dokument
import-document-close = Stäng
import-document-step-files = Filer
import-document-step-review = Granska
import-document-step-destination = Mål
import-document-step-reconcile = Sammanfoga
import-document-reconcile-hint = Några av dessa objekt är de du redan har. Säg vad som ska hända med var och en.
import-document-reconcile-all-new = Ingenting i den här filen matchar ditt projekt: varje objekt kommer att läggas till som nytt.
import-document-duplicate-returns = Dessa filer är mer än en kopia av samma manuskript som kommer tillbaka. Importera dem en i taget: matchning endast rader upp en återvändande kopia mot ditt projekt, så importera flera tillsammans skulle lägga till en andra kopia av boken istället för att slå samman den.
import-document-hunk-added = Ta detta nya stycke
import-document-hunk-removed = Släpp denna punkt
import-document-hunk-changed = Ta denna omskrivning
import-document-col-stray-prose = Dess text
import-document-stray-as-paratext = Håll som paratext
import-document-stray-discard = Släpp texten
import-document-col-current = I ditt projekt
import-document-col-incoming = I denna fil
import-document-col-status = Status
import-document-col-action = Vad man ska göra
import-document-status-identical = Samma
import-document-status-editor-edited = De redigerade det
import-document-status-you-edited = Du redigerade det
import-document-status-conflict = Båda redigerade den
import-document-status-different = Skillnader
import-document-status-new = Ny
import-document-status-missing = Inte i denna fil
import-document-status-moved = Flyttas
import-document-action-comments-only = Kommentarer endast
import-document-action-take-import = Ta denna version
import-document-action-keep-current = Håll min
import-document-action-create-new = Lägg till som ny
import-document-action-ignore = Hoppa över
import-document-compare = Jämför
import-document-compare-legend = Din version jämfört med den här filen. Read-only.
import-document-compare-close = Stäng
import-document-drop-title = Släpp dokument här
import-document-drop-hint = Markdown (.md), Word (.docx), OpenDocument (.odt) och vanlig text (.txt)
import-document-browse = Bläddra...
import-document-move-up = Flytta upp
import-document-move-down = Flytta ned
import-document-remove-file = Ta bort
import-document-file-count = { $count ->
    [one] 1 fil
   *[other] { $count } filer
}
import-document-no-files = Inga filer som valts ännu.
import-document-col-included = Importera
import-document-col-title = Titel
import-document-col-type = Typ
import-document-col-words = Ord
import-document-col-breaks = Avskiljare
import-document-col-comments = Kommentarer
import-document-col-footnotes = Fotnoter
import-document-col-epigraph = Motto
import-document-col-source = Källa
import-document-level-rules = Rubriknivåer
import-document-level-n = Rubrik { $level }
import-document-add-top-level = Lägg till toppnivå
import-document-add-top-level-tooltip = Sätt in en bok ovanför varje analyserat objekt, för kapitelfiler som inte har någon bokrubrik
import-document-destination = Mål
import-document-destination-hint = Välj ett bindemedel eller objekt: nya objekt landar inuti en mapp eller efter en scen.
import-document-destination-empty = Inga bindemedel ännu. Skapa en först.
import-document-plan-empty = Inget att importera ännu.
import-document-summary = { $rows ->
    [one] 1 objekt
   *[other] { $rows } objekt
} · { $breaks ->
    [one] 1 scen break
   *[other] { $breaks } scenbrott
}
import-document-back = Tillbaka
import-document-cancel = Avbryt
import-document-analyse = Nästa
import-document-analysing = Läsa dokument...
import-document-step-analysing = Läsa
import-document-cancel-analysis = Sluta läsa
import-document-analyse-failed = Dokumenten kunde inte läsas.
import-document-details = Detaljer
import-document-import = Importera
import-document-done = { $count ->
    [one] 1 Objekt importerat
   *[other] { $count } varor importerade
}
import-document-undo = Ångra
import-diagnostic-file-unreadable = "{ $path }" kunde inte läsas: { $detail }. De andra filerna importerar fortfarande.
import-diagnostic-nested-too-deep = "{ $path }" lästes inte: dess innehåll är nedlagt mer än { $limit } nivåer djupt. Inget riktigt dokument kommer nära det, och läsa det kan krascha Skribisto, så det lämnas ut. De andra filerna importerar fortfarande.
import-diagnostic-archive-too-large = "{ $path }" lästes inte: det är en komprimerad fil byggd för att packa upp mycket större än det är, vilket kan uttömma minnet och kraschen Skribisto, så det lämnas ut. De andra filerna importerar fortfarande.
import-diagnostic-lossy-decode = { $count ->
    [one] En karaktär i "{ $path }" avkodade inte. Re-save filen som UTF-8 för att hålla den.
   *[other] { $count } tecken i "{ $path }" avkodade inte. Rädda filen som UTF-8 för att hålla dem.
}
import-diagnostic-decoded-from-bom = "{ $path }" avkodades som { $detail }, inte UTF-8.
import-diagnostic-empty-file = "{ $path }" är tom.
import-diagnostic-no-headings = "{ $path }" har inga rubriker, så det kommer som ett objekt.
import-diagnostic-unsupported-format = Ingenting läser ".{ $detail }" filer, så "{ $path }" hoppades.
import-diagnostic-front-matter-not-flat = Front materia i "{ $path }": "{ $detail }" är inte ett enkelt värde och hoppades.
import-diagnostic-footnotes-degraded = { $count ->
    [one] En fotnot i "{ $path }" kommer som vanlig text: fotnoter läses inte från Markdown.
   *[other] { $count } fotnoter i "{ $path }" anländer som vanlig text: fotnoter läses inte från Markdown.
}
import-diagnostic-footnote-not-carried = { $count ->
    [one] En fotnot i "{ $path }" kunde inte överföras.
   *[other] { $count } fotnoter i "{ $path }" kunde inte överföras.
}
import-diagnostic-raw-html-dropped = { $count ->
    [one] Ett kvarter av rå HTML i "{ $path }" släpptes.
   *[other] { $count } block av rå HTML i "{ $path }" släpptes.
}
import-diagnostic-nested-break-dropped = { $count ->
    [one] En scenbrytning inuti ett citat eller en lista i "{ $path }" släpptes. Endast en paus på sin egen linje hålls.
   *[other] { $count } scenbrott inuti citat eller listor i "{ $path }" släpptes. Endast en paus på sin egen linje hålls.
}
import-diagnostic-image-not-ingested = "{ $path }" avser bilden "{ $detail }". Referensen anländer som text; själva bilden kopieras inte.
import-diagnostic-duplicate-title = "{ $title }" visas { $count } gånger.Om du har importerat dessa filer innan, kommer detta att duplicera dem.
import-diagnostic-heading-level-jump = "{ $title }" hoppar från rubriknivå { $from } till { $to }; den placeras en nivå under sin förälder.
import-diagnostic-illegal-combination = "{ $title }" bär prosa, men en { $kind } kan inte hålla någon. Import hålls tills du ändrar sin typ eller lämnar den. Ingenting importeras alls.
import-diagnostic-tracked-changes-flattened = { $path } var mitten av revideringen: { $count ->
    [one] { $count } spårade förändringar accepterades
   *[other] { $count } spårade förändringar accepterades
} , och raderingar tappade. Det är den slutliga texten. För att hålla någon av dina egna ordalydelser i stället, använd Jämför i det sista steget.
import-diagnostic-tracked-changes-flattened-by = { $path } var mitten av revideringen: { $count ->
    [one] { $count } spårade förändringar med { $names } accepterades
   *[other] { $count } spårade förändringar med { $names } godtogs
} , och raderingar tappade. Det är den slutliga texten. För att hålla någon av dina egna ordalydelser i stället, använd Jämför i det sista steget.
import-diagnostic-text-box-dropped = { $count ->
    [one] { $path } innehåller { $count } textruta. Texten sitter utanför dokumentets flöde, så där den hör hemma i ett manuskript kan inte besvaras. Den importeras inte.
   *[other] { $path } har { $count } textrutor. Deras text sitter utanför dokumentets flöde, så där den hör hemma i ett manuskript kan inte besvaras. De importeras inte.
}
import-diagnostic-embedded-object-dropped = { $count ->
    [one] { $path } har { $count } inbäddat objekt: ett diagram, en ekvation eller liknande. Det finns inget i ett manuskript som kan hålla det.
   *[other] { $path } har { $count } inbäddade objekt: ett diagram, en ekvation eller liknande. Det finns inget i ett manuskript som kan hålla dem.
}
import-diagnostic-field-flattened = { $count ->
    [one] { $path } har { $count } fält: ett sidnummer, en korsreferens, ett datum. Det håller texten som den senast visade och kommer inte att uppdatera igen.
   *[other] { $path } har { $count } fält: ett sidnummer, en korsreferens, ett datum. Varje håller texten som den senast visade och kommer inte att uppdatera igen.
}
import-diagnostic-unknown-style-level = { $path } använder stilen "{ $detail }", som ser ut som en rubrik men inte namn på någon nivå. Dessa punkter importeras som prosa snarare än gissade på ett djup.
import-diagnostic-comment-unanchored = Kommentaren "{ $detail }" i { $path } kunde inte fästas på de ord det handlade om. Det hålls på sitt objekt, där du kan flytta det.
import-diagnostic-comment-not-carried = Kommentaren "{ $detail }" i { $path } är inte importerad, eftersom ingenting i filen anländer som text som den kan hållas på. Kopiera den från filen om du behöver den.
import-diagnostic-comment-replies-flattened = { $count ->
    [one] { $count } svarar i { $path } heter en kommentar som inte finns i filen, så den kommer som en kommentar av sin egen.
   *[other] { $count } svarar i { $path } som heter en kommentar som inte finns i filen, så de anländer som kommentarer av sig själv.
}
import-diagnostic-prose-not-verbatim = { $count ->
    [one] { $count } stycket i { $path } kunde inte överföras med dess formatering precis som skrivet, så det kommer som vanligt text. Jämför det med originalet.
   *[other] { $count } stycken i { $path } kunde inte överföras med sin formatering precis som skrivet, så de kommer som vanligt text. Jämför dem med originalet.
}
import-diagnostic-styled-spaces-not-carried = { $count ->
    [one] I { $path }, { $count } sträcka av tomt utrymme underströks eller slog igenom, en linje kvar att fylla i till exempel. Skribisto håller varken linje på utrymmen ensam, så det kommer som vanliga utrymmen, eller inte alls där det startar eller slutar en punkt.
   *[other] { $path }, { $count } sträckor av tomt utrymme underströks eller slog igenom, linjer kvar för att fylla i till exempel. Skribisto håller varken linje på utrymmen ensam, så de anländer som vanliga utrymmen, eller inte alls där de startar eller avslutar ett stycke.
}
import-diagnostic-list-nesting-flattened = { $count ->
    [one] { $count } listobjektet i { $path } var nested mer än { $limit } nivåer djupt. Det anländer på nivå { $limit }, den djupaste Skribisto håller, med sina ord och formatering.
   *[other] { $count } listobjekt i { $path } var kapslade mer än { $limit } nivåer djupt. De anländer till nivå { $limit }, den djupaste Skribisto håller, med sina ord och formatering.
}
import-diagnostic-table-read-as-paragraphs = { $count ->
    [one] { $count } tabellen i { $path } har en rad mycket bredare än resten, alltför lopsided att lägga ut som ett rutnät. Dess celler anländer som punkter, en efter en, med sina ord och formatering.
   *[other] { $count } tabeller i { $path } har en rad mycket bredare än resten, alltför lopsided att lägga ut som ett rutnät. Deras celler anländer som punkter, en efter en, med sina ord och formatering.
}
import-diagnostic-spaces-shortened = { $count ->
    [one] { $count } körning av utrymmen i { $path } var längre än { $limit } utrymmen, och anländer förkortas till { $limit }. Orden runt det är alla där.
   *[other] { $count } runs av utrymmen i { $path } var längre än { $limit } utrymmen, och anländer förkortas till { $limit }. Orden runt dem är alla där.
}
import-diagnostic-epigraph-not-carried = "{ $title }" leds av en epigraf, men en { $kind } kan inte hålla en. Citatet hålls högst upp i texten istället.
import-diagnostic-epigraph-placement-ambiguous = En epigraf sitter mellan "{ $title }" och "{ $below }" och kunde leda antingen. Det gavs till "{ $title }", vilket är där en epigraf vanligtvis går.
import-document-diagnostics = { $errors ->
    [0] { $warnings ->
            [one] 1 sak att veta
           *[other] { $warnings } saker att veta
        }
   *[other] { $errors ->
            [one] 1 fil kan inte läsas
           *[other] { $errors } filer kan inte läsas
        }
}
import-document-diagnostics-none = Inget att rapportera.

versions-title = Versioner
versions-scope-synopsis = Synopsis
versions-scope-prose = Text
versions-loading = Tittar igenom dina säkerhetskopior...
versions-empty = Ingen tidigare version av detta ännu
versions-error = Kunde inte läsa dina säkerhetskopior. Ingenting har gått förlorat, men den här listan kan vara ofullständig.
versions-did-not-exist = Har inte funnits på { $date }
versions-deleted-after = Tas bort någon gång efter { $date }
versions-unreadable = { $count ->
    [one] { $count } backup kunde inte läsas
   *[other] { $count } säkerhetskopior kunde inte läsas
}
versions-thinned = { $count ->
    [one] Äldre versioner tunna ut när de åldras. Projektets egen historia har redan släppt { $count } tidigare tillstånd av denna text.
   *[other] Äldre versioner tunna ut när de åldras. Projektets egen historia har redan släppt { $count } tidigare tillstånd av denna text.
}
versions-source-backup = Från en backup
versions-source-project = Från projektets egen historia
versions-list-caption = En post per förändring, inte per backup
versions-pick-a-version = Välj en version för att se vad som ändrats
versions-earliest = Den tidigaste versionen på rekord. Det finns inget äldre att jämföra den med.
versions-no-change = Ingenting förändrades i denna del av objektet
versions-formatting-only = Endast formateringen ändras här. Orden är desamma
versions-show-unchanged = Visa oförändrad
versions-hide-unchanged = Dölja oförändrad
versions-next-change = Nästa förändring
versions-near = nära "{ $text }"
versions-words-added = { $count ->
    [one] { $count } ord tillagt
   *[other] { $count } ord tillagda
}
versions-words-removed = { $count ->
    [one] { $count } ord bort
   *[other] { $count } ord borttagna
}
versions-blocks-moved = { $count ->
    [one] { $count } stycket flyttade
   *[other] { $count } stycken flyttade
}
versions-pin = Pin backup denna version kom från. Automatisk rengöring kommer aldrig att radera den
versions-unpin = Ställ in säkerhetskopian den här versionen kom från. Automatisk rensning kan ta bort den igen
versions-pinned-only = Visa endast stiftade versioner
versions-pin-note = Endast versioner från en säkerhetskopia kan fästas. En pin håller en fil, och projektets egen historia är inte en.
versions-pinned-empty = Inget här är pinned ännu
versions-pinned-empty-log = Ingenting här är fäst ännu. Endast versioner från en säkerhetskopia kan fästas. En stift håller en fil, och projektets egen historia är inte en.
versions-range-filter = Visa endast versioner mellan två datum
versions-filtered-empty = Ingen version matchar de filter du har ställt
versions-clear-filters = Rensa filter
versions-last-30-days = Sista 30 dagarna
versions-restore-button = Återställ denna version
versions-restore-confirm-title = Ersätt denna text med versionen från { $date }?
versions-restore-confirm-text = Det du har nu kommer att ersättas med den text som denna artikel hade på { $date }.
versions-restore-confirm-with-comments = Det du har nu kommer att ersättas med den text som denna artikel hade på { $date }. { $count ->
    [one] { $count } kommentar är förankrad i den aktuella texten och kan lämnas föräldralösa.
   *[other] { $count } kommentarer förankras i den aktuella texten och kan lämnas föräldralösa.
}
versions-restore-confirm-undo-note = En säkerhetskopia görs först, och Ctrl+Z gör det i ett steg.
versions-restored-toast = Återställt versionen från { $date }
versions-undo = Ångra
versions-restore-row-gone = Objektet är inte längre i detta projekt
versions-restore-no-home = Det här objektet har ändrat typ sedan dess, och den gamla texten har ingenstans att gå in i den.
versions-restore-no-safety-copy = Säkerhetskopian kördes inte, så ingenting ändrades
versions-restore-failed = Återställningen misslyckades: { $error }
versions-restore-backup-busy = En backup är redan igång. Försök igen i ett ögonblick
versions-restore-in-backup-file = Du tittar på en säkerhetskopia; öppna projektet själv för att återställa det
versions-restore-no-project = Inget projekt är öppet
versions-recreate-button = Ta tillbaka detta...
versions-recreate-picker-title = Vart ska det gå?
versions-recreate-picker-empty = Detta projekt har ingen bindemedel för att sätta in det
versions-recreate-picker-confirm = Ta tillbaka den här
versions-recreate-picker-cancel = Avbryt
versions-recreate-untitled = Detta objekt
versions-recreate-confirm-title = Ta tillbaka "{ $item }"?
versions-recreate-confirm-text = Den kommer att läggas till { $destination }, med texten den hade på { $date }.
versions-recreate-confirm-undo-note = Undo, på meddelandet som följer, tar det rakt ut.
versions-recreated-toast = "{ $item }" är tillbaka i ditt projekt
versions-recreated-partial-toast = { $count ->
    [one] "{ $item }" är tillbaka, men en av dess texter kunde inte läsas.
   *[other] "{ $item }" är tillbaka, men { $count } av dess texter kunde inte läsas.
}
versions-recreate-already-here = Objektet finns i ditt projekt redan
versions-recreate-no-destination = Välj någonstans i bindemedel för att sätta den
versions-recreate-unreadable = Den säkerhetskopian kunde inte läsas, så ingenting lades till
versions-recreate-failed = Kunde inte ta tillbaka den: { $error }
versions-changed-percent = { $percent }% av detta förändrade
versions-hidden-paragraphs = { $count ->
    [one] { $count } oförändrad paragraf ...
   *[other] { $count } oförändrade punkter ...
}

timeline-title = Gå tillbaka i tiden
timeline-coverage = { $count ->
    [one] { $count } version inspelad, går tillbaka till { $oldest }
   *[other] { $count } versioner inspelade, går tillbaka till { $oldest }
}
timeline-loading = Titta igenom ditt projekts förflutna...
timeline-empty = Ingen version av detta projekt har spelats in ännu
timeline-no-changes = Inget har förändrats sedan dess
timeline-no-text-changes = Ingen text har ändrats sedan dess
timeline-slider-label = Inspelad version
timeline-series-name = Projektets storlek
timeline-bars-caption = Varje bar är en inspelad version, så lång som projektet var då. Den markerade är vad du tittar på.
timeline-bars-caption-periods = För många versioner att visa en efter en, så varje bar är en { $period }, så lång som projektet var i slutet av det.
timeline-unit-hour = timme
timeline-unit-day = Dag dag dag dag dag
timeline-unit-week = Vecka vecka
timeline-unit-month = Månad månad
timeline-range-filter = Visa endast versioner mellan två datum
timeline-range-empty = Ingen version spelades in i dessa datum
timeline-open-period = Öppna denna period
timeline-show-all = Visa hela historien
timeline-changed-since = { $count ->
    [one] { $count } objekt skiljer sig mellan { $date } och ditt projekt nu
   *[other] { $count } objekt skiljer sig mellan { $date } och ditt projekt nu
}
timeline-kind-added = Skriftligt sedan
timeline-kind-removed = Inte längre i projektet
timeline-kind-changed = Redigerad sedan
timeline-kind-moved = Flyttas sedan
timeline-not-yet-written = Detta existerade inte ännu vid den tidpunkten
timeline-no-text-of-its-own = Detta har ingen egen text. Det är en rubrik för vad som finns inuti det.
timeline-reader-close = Stäng
timeline-reader-stamp = Som det var på { $date }
timeline-reader-loading = Öppna den inspelade versionen...
timeline-reader-unreadable = Denna inspelning kunde inte läsas. Säkerhetskopian kan ha flyttats, raderats eller vara på en enhet som inte är ansluten.
timeline-reader-compared = Som det var på { $date }, sätt mot vad det säger nu.
timeline-reader-diff-legend = Struck genom har gått sedan; understruken har lagts till.
timeline-reader-view-label = Vad ska man visa
timeline-reader-view-diff = Förändringar
timeline-reader-view-text = Text
timeline-reader-deleted = Det här är inte längre i ditt projekt. Du kan läsa det och kopiera det här.
timeline-prose-only-record = Denna punkt kommer från projektets egen historia, som håller text och inget annat, så vad som raderades eller flyttades eftersom det inte kan visas. Välj en säkerhetskopia för det.

settings-sec-extensions = Förlängningar

settings-group-games-forward = Alltid framåt
settings-games-forward-toggle = Spela "Alltid framåt"
settings-games-forward-hint =
    Medan du spelar kan ingenting du har skrivit tas tillbaka: Backspace, Delete
    Skär, drag-and-drop och Undo är alla funktionshindrade i de ytor du väljer nedan.
    Du kan fortfarande skriva, klistra in, formatera och flytta runt fritt. Utkastet växer bara.
settings-games-session-warning =
    Detta är en per-session val: det sparas aldrig. Stänga projektet, eller sluta.
    Skribisto, slutar alltid spelet, och allt du skrev när du spelar blir
    Oförmögen igen det ögonblick det gör.
settings-group-games-scope = Om det gäller
settings-games-in-prose = Manuskript prosa
settings-games-in-synopsis = Synopsis
settings-games-scope-hint =
    Kommentarer, fotnoter och titlar är aldrig frusna: de är där du noterar fixen
    Du har bara förbjudit dig själv från att göra.
settings-games-inert-warning =
    "Alltid framåt" är på men gäller ingenting. Tick minst en yta ovan,
    Det förändrar ingenting som du skriver.

games-title = Skriva spel
games-forward-name = Alltid framåt
games-forward-blurb =
    Utkast utan att ta något tillbaka. Ta bort, skära och ångra är funktionshindrade
    När du spelar. Skriv nästa mening istället för att fixa den sista.
games-forward-playing = Spela: Radering är inaktiverad
games-forward-idle = Att inte spela
games-session-note = Avslutar när du stänger projektet.
games-scope-prose-and-synopsis = Gäller för dina prosa och synopsar.
games-scope-prose = Gäller för din prosa.
games-scope-synopsis = Gäller dina synopser.
games-scope-nothing = Gäller ingenting ännu. Välj en yta i Inställningar.
games-settings-link = Skriva spelinställningar...
statusbar-games-forward = Alltid framåt
statusbar-games-forward-tooltip =
    "Alltid framåt" är på: ta bort, skära och ångra är funktionshindrade medan du
    Utkast. Klicka för att sluta spela.
goal-progress-words = { $g ->
   *[other] Ord: { $count } / { $goal }
}
goal-progress-characters = { $g ->
   *[other] Tecken: { $count } / { $goal }
}
goal-count-words = { $n ->
    [one] { $count } ord
   *[other] { $count } ord
}
goal-count-characters = { $n ->
    [one] { $count } tecken
   *[other] { $count } tecken
}
goal-unit-words = Ord
goal-unit-characters = Tecken

inspector-goal = Mål
inspector-goal-none = Inget mål

overview-col-goal = Mål
overview-excluded-from-export = Vänster ut ur exporten, så den räknas inte mot totalt

goal-subtree-total-words = { $count ->
    [one] { $items } mål inuti lägger till upp till { $words } ord
   *[other] { $items } mål inuti lägga upp till { $words } ord
}
goal-subtree-total-characters = { $count ->
    [one] { $items } mål inuti lägger till upp till { $words } tecken
   *[other] { $items } mål inuti lägga upp till { $words } tecken
}

new-work-goal-unit = Räkna i
new-work-goal-unit-hint = Används för varje ord eller teckenmål i detta projekt. Ändras senare.

settings-group-goal-unit = Mål
settings-goal-unit-switch-title = Byt räkneenhet
settings-goal-unit-switch-text = De mål och milstolpar som redan sattes för detta projekt ingicks i { $from }. Växla till { $to } konverterar inte dem, så varje befintligt nummer kommer nu att läsas som { $to }.
settings-goal-unit-switch-informative = Ingenting går förlorat. Växla tillbaka när som helst för att återställa den ursprungliga läsning, sedan uppdatera de mål du vill behålla.
settings-goal-unit-switch-confirm = Byt

goal-distribute-action = Distribuera...
distribute-title = Distribuera målet
distribute-empty = Det finns inget i detta för att dela målet ut till.
distribute-weight = Dela ut
distribute-weight-length = Genom längd
distribute-weight-rows = av bitar
distribute-weight-even = Till och med
distribute-overwrite = Ersätt mål som redan har ställts
distribute-col-item = Föremål
distribute-col-current = Numera nu
distribute-col-proposed = Efter efter
distribute-total = Dessa lägger till upp till { $total }, mot ett mål på { $goal }.
distribute-over-budget = De mål som redan satts in kommer till { $over } mer än den här behållarens eget. Höj målet, sänka deras eller ersätta dem.
distribute-apply = Distribuera
distribute-cancel = Avbryt

milestone-target-gone = Target tas bort
milestone-no-target = Inget mål
milestone-add = Lägg till
milestone-add-label = Vad bör nås

pace-summary-title = Där boken står
pace-summary-remaining = { $words } för att gå
pace-summary-open = Öppna planen
pace-summary-close = Stäng
pace-summary-dont-show = Visa inte detta när du öppnar
pace-summary-menu = Skriva plan...

menu-help-topics = Hjälp Ämnen (&T)
menu-help-shortcuts = Tangentbordsgenvägar... (&K)
menu-help-website = Skribisto på webben (&W)
menu-help-report = Rapportera ett problem... (&R)
menu-help-check-updates = Kolla för uppdateringar (&U)
menu-command-palette = Kommandopalett... (&C)

learn-help-topics = Hjälp ämnen
learn-shortcuts = Tangentbordsgenvägar
learn-website = Skribisto på webben

help-window-title = Hjälp
help-filter-topics = Filtrera ämnen
help-no-matching-topic = Inget ämne matchar.
help-topic-missing = Detta ämne är inte längre tillgängligt.
help-back = Tillbaka
help-not-translated = Den här artikeln finns ännu inte på svenska. Den visas på engelska.

help-section-getting-started = Komma igång
help-section-writing = Skrivande
help-section-reviewing = Granskning
help-section-exchanging = Få ord in och ut
help-section-keeping = Hålla ditt projekt säkert
help-section-extensions = Förlängningar

help-topic-getting-started = Ditt första projekt
help-topic-writing-model = Hur en bok är sammansatt
help-topic-drafts-and-old-versions = Att hålla ett äldre utkast
help-topic-goals-and-pace = Mål och takt
help-topic-comments = Kommentarer
help-topic-round-trip = Skicka din bok till en läsare
help-topic-export = Exportera
help-topic-import-documents = Importera dokument
help-topic-import-projects = Att få ett helt projekt över
help-topic-backups-and-versions = Backups och versioner

help-shortcuts-title = Tangentbordsgenvägar
help-shortcuts-filter = Filter genvägar
help-shortcuts-no-matches = Inga genvägsmatcher.
help-shortcuts-rebind = Ändra genvägar...
help-shortcuts-close = Stäng

command-palette-placeholder = Skriv ett kommando

shortcut-name-binder-duplicate = Duplicera
shortcut-name-comments-add = Lägg till kommentar
shortcut-name-comments-add-paragraph = Kommentar om stycket
shortcut-name-spellcheck-toggle = Kontrollera Spelling
shortcut-name-editor-tab-close = Nära Tab
shortcut-name-editor-tab-pin = Pin eller Unpin Tab
shortcut-name-editor-save = Spara
shortcut-name-work-export = Exportera…
shortcut-name-work-new = Nytt projekt
shortcut-name-work-open = Öppet projekt
shortcut-name-window-new = Nytt fönster
shortcut-name-work-close = Stäng projekt
shortcut-name-app-settings = Inställningar
shortcut-name-app-quit = Avsluta
shortcut-name-editor-insert-footnote = Infoga Footnote
shortcut-name-format-scene-break = Infoga Scene Break
shortcut-name-format-major-scene-break = Infoga större Scene Break
shortcut-name-format-link = Infoga Link
shortcut-name-format-mark-nbsp = Infoga No-break Space
shortcut-name-format-mark-nbhyphen = Infoga No-break Hyphen
shortcut-name-format-mark-nnbsp = Infoga Narrow No-break Space
shortcut-name-format-mark-zwsp = Infoga Zero-width Space
shortcut-name-go-next = Nästa
shortcut-name-go-prev = Föregående
shortcut-name-go-to = Gå till
shortcut-name-outline-toggle = Visa eller dölj disposition
shortcut-name-preview-toggle = Visa eller dölj förhandsgranskningsfält
shortcut-name-view-fullscreen = Växla helskärmsläge
shortcut-name-view-focus-mode = Toggle Distraction-fria läget
shortcut-name-editor-size-increase = Öka textstorleken
shortcut-name-editor-size-decrease = Minska textstorleken
shortcut-name-editor-size-reset = Återställ textstorlek
shortcut-name-editor-find = Sök
shortcut-name-editor-replace = Ersätt
shortcut-name-editor-find-next = Nästa match
shortcut-name-editor-find-prev = Föregående Match
shortcut-name-search-show = Sök i Project
shortcut-name-search-replace = Ersätts i projekt
shortcut-name-outline-open-to-side = Öppen för Side
shortcut-name-help-topics = Hjälp Ämnen
shortcut-name-help-shortcuts = Tangentbordsgenvägar
shortcut-name-help-website = Skribisto på webben
shortcut-name-help-report = Rapportera ett problem
shortcut-name-help-check-updates = Kolla för uppdateringar
update-check-behind = Version { $version } är tillgänglig.
update-check-current = Du kör den senaste versionen.
update-check-failed = Kunde inte kontrollera en nyare version. { $error }
shortcut-name-command-palette = Kommando Palette
help-section-reference = Vilka saker är

help-concept-scene-break = Scenavskiljare
help-concept-major-scene-break = Major scen break
help-concept-find-in-prose = Hitta i prose
help-concept-story-bible = Berättelsebibel
help-concept-goal-unit = Ord eller tecken
help-concept-goal-progress = Framsteg
help-concept-manuscript-words = Vad räknas som manuskriptet
help-concept-exportable = Vänster ut ur exporten
help-concept-distribute = Dela ett mål ut
help-concept-subtree-total = Vad målen inuti lägger upp till
help-concept-milestone = Milstolpe
help-concept-pace-plan = Pace Plan

help-concept-tag = Tagg
help-concept-label = Kort beskrivning
help-concept-point-of-view = Perspektiv
help-concept-epigraph = Motto
help-concept-footnote = Fotnot
help-concept-chapter-mode = Kapitel form
help-concept-comment = Kommentar
help-concept-backup = Säkerhetskopia
help-concept-version = Version
help-concept-trash = Papperskorg
help-concept-spellcheck = Stavningskontroll
help-concept-search-replace = Sök och ersätt
help-concept-note-template = Anteckningsmall
help-concept-story-bible-entry = Story Bible Entry
help-concept-text-replacement = Textbyte
help-concept-smart-punctuation = Smart Punctuation
help-concept-export-style = Exportstil
help-concept-round-trip-marks = Round-trip markörer

epigraph-mark-attribution = Källa linje
epigraph-mark-attribution-tip = Markera linjen varukorgen är i som citatets källa, så det skriver ut som en attribution
margin-lane-name = Marginalmärken
margin-lane-provider-comments = Kommentarer
margin-lane-provider-story-bible = Story Bible Entry
margin-lane-provider-story-bible-hint = I prosa läsning, där den posten heter, och scenerna berättade genom sina ögon. markerar också dessa namn i prosa själv.
margin-lane-mark-point-of-view = Berätta härifrån
margin-lane-mark-named-here = Uppkallad här
margin-lane-provider-comments-hint = Där en anteckning är knuten till texten
margin-lane-provider-search = Sök hits
margin-lane-provider-search-hint = Varje träff för vad du senast sökte efter, oavsett var det är i dokumentet
margin-lane-provider-boundaries = Där varje dokument startar
margin-lane-provider-boundaries-hint = En regel på toppen av varje scen i en ström, så du kan se var du är
margin-lane-provider-spelling = Stavning
margin-lane-provider-spelling-hint = Där stavningskontrollen har flaggat ett ord. Av om du inte ber om det: det markerar en maskins åsikt om din prosa.
margin-lane-spelling = { $word }, möjlig felstavning
margin-lane-search-hit = { $text }, match { $index } av { $total }
margin-lane-search-current = { $text }, match { $index } av { $total }, den du är på
margin-lane-boundary = Början på { $title }
margin-lane-boundary-untitled = Ett oberättigat dokument
settings-page-margin-lane = Marginalmärken
settings-desc-margin-lane = Strippa bredvid rullningsfältet, och vad det visar
settings-margin-lane-enabled = Visa marginalen lane
settings-margin-lane-enabled-hint = Samma växel som View - Marginalmärken.
settings-margin-lane-enabled-more = Det säger aldrig att något är fel: det visar dig var saker och ting är, och lämnar vad du ska göra av det till dig.
settings-group-margin-lane-marks = Vad det markerar
settings-margin-lane-no-providers = Inget markerar körfältet ännu.
settings-group-margin-lane-texture = Dialogtextur
settings-margin-lane-texture = Visa dialogtexturen
settings-margin-lane-texture-hint = En bar per stycke: hur länge det är, och hur mycket av det talas.
settings-margin-lane-texture-more = Inte mätt för språk utan kuraterad konvention, där baren visar längd ensam.
settings-group-margin-lane-surfaces = Där det verkar
settings-margin-lane-surface-editor = Text redaktör
settings-margin-lane-surface-stream = Textflöden
settings-margin-lane-surface-search-preview = Sök förhandsgranska
menu-margin-lane = Marginalmärken (&M)

status-preset-drafting = Utkast
status-preset-passes = Revision passerar
status-preset-plume = Plume Creator
status-todo = Att göra
status-draft = Utkast
status-revised = Reviderat
status-final = Slutlig
status-outline = Disposition
status-first-edit = Första bearbetningen
status-second-edit = Andra bearbetningen
status-done = Klar
status-plume-draft-1 = 1:a utkastet
status-plume-draft-2 = 2nd utkast
status-plume-draft-3 = 3:e utkastet
status-plume-edit-1 = Första bearbetningen
status-plume-edit-2 = Andra bearbetningen
status-plume-edit-3 = Tredje bearbetningen
status-plume-proofread = Korrekturläst
status-plume-finished = Färdigt
new-work-statuses = Arbetsflöde
new-work-statuses-hint = Stegen du flyttar en scen genom. Du kan byta namn, ordna eller lägga till dessa när som helst.
status-none = Ingen status
inspector-status = Status
overview-col-status = Status
overview-status-mixed = delarna nedan är oeniga
help-concept-status = Status
status-completion-title = Där boken står
status-completion-headline = { $done } av { $total } scener färdiga
status-completion-empty = Inga scener ännu. Detta läser manuskriptet, så det fyller i när du skriver.
status-completion-open = Där boken står...

settings-page-statuses = Status
settings-desc-statuses = Byt namn, ordna, lägg till och ta bort de steg du flyttar en scen genom
settings-statuses-add = Lägg till status
settings-statuses-add-placeholder = Nytt statusnamn
settings-statuses-desc =
    Scenerna en scen rör sig genom, för att, från minst färdiga på toppen till de flesta
    Den ordningen är det som "mindre än" betyder överallt annat.
    i appen, så sätt dem hur din process faktiskt körs.
settings-statuses-apply-preset = Applicera ett förinställt...
settings-statuses-preset-applied = { $added ->
    [one] 1 status läggs till
   *[other] { $added } status läggs till
}
settings-statuses-preset-refused = Detta projekt har redan en stege. Förinställningar fyller bara en tom. Ta bort de trasor du inte vill först.
settings-statuses-duplicate = "{ $name }" är redan på denna stege
settings-statuses-added = Tillagd "{ $name }"
settings-statuses-move-up = Flytta upp (mindre än färdigt)
settings-statuses-move-down = Flytta ner (mer färdig)
settings-statuses-details-placeholder = Vad detta stadium betyder (valfritt)
settings-statuses-delete = Radera "{ $name }"
settings-statuses-delete-in-use = { $count ->
    [one] Ta bort "{ $name }": 1 objekt är på det och kommer att förlora sin status
   *[other] Radera "{ $name }": { $count } objekt är på den och kommer att förlora sin status
}
settings-statuses-deleted = Ta bort "{ $name }"
settings-statuses-deleted-in-use = { $count ->
    [one] Ta bort "{ $name }".1 objekt har inte längre en status.
   *[other] Ta bort "{ $name }" { $count } objekt har inte längre en status.
}
settings-statuses-empty-title = Inget arbetsflöde ännu
settings-statuses-empty-body = En status säger hur långt längs en scen är. Lägg till en ovan, eller börja från en förinställning.
status-category-planned = Planerade
status-category-drafting = Utkast
status-category-needs-work = Behöver arbete
status-category-revised = Reviderat
status-category-final = Slutlig
