# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet
menu-work = &Projekt
menu-new-work = &Nytt projekt
menu-open-work = &Öppna projekt…
menu-new-window = Nytt fönste&r
menu-import-from = &Importera från
menu-create-from = &Skapa från
menu-import-plume = &Plume Creator (.plume)…
menu-import-document = &Dokument (Markdown, Word, ODT)…
menu-import-manuskript = &Manuskript (.msk)…
menu-export = &Exportera
menu-export-book = Exportera bok
menu-export-part = Exportera del
menu-export-chapter = Exportera kapitel
menu-export-scene = Exportera scen
menu-export-note = Exportera anteckning
menu-export-paratext = Exportera paratext
menu-export-folder = Exportera mapp
menu-export-choose = Välj…
menu-export-none = Öppna ett dokument för att exportera
menu-save = &Spara
menu-save-as-file = Spara som en enda &fil…
menu-save-as-folder = Spara som &mapp…
menu-backup = Säker&hetskopiera nu
menu-close-work = S&täng projekt
menu-welcome = &Välkommen…
menu-settings = Instä&llningar
menu-quit = &Avsluta
menu-view = &Visa
menu-outline = &Disposition
menu-search = &Sök i projektet
menu-timeline = &Gå tillbaka i tiden
menu-search-preview = Sök&förhandsvisning
menu-fullscreen = &Helskärm
menu-focus-mode = Dis&traktionsfritt läge
menu-format = F&ormat
menu-scene-break = Infoga &scenbrytning
menu-major-scene-break = Infoga st&örre scenbrytning
menu-go = &Gå
menu-go-next-scene = Nästa &scen
menu-go-prev-scene = Föregående s&cen
menu-go-next-chapter = Nästa &kapitel
menu-go-prev-chapter = Föregående ka&pitel
menu-go-next-note = Nästa &anteckning
menu-go-prev-note = Föregående a&nteckning
menu-tools = Ver&ktyg
menu-help = &Hjälp
ctx-add = &Lägg till
ctx-new-item = &Nytt objekt
ctx-new-folder = Ny &mapp
ctx-rename = Byt &namn
ctx-duplicate = &Duplicera
ctx-import-here = &Importera hit…
ctx-indent = Öka &indrag
ctx-outdent = &Minska indrag
ctx-trash = Flytta till &papperskorgen
ctx-open-to-side = Öppna &bredvid
ctx-open = &Öppna
ctx-reveal-in-outline = Visa i &dispositionen
ctx-move-up = Flytta &upp
ctx-move-down = Flytta &ned
ctx-tab-close = &Stäng
ctx-tab-close-others = Stäng &övriga
ctx-tab-close-all = Stäng &alla
ctx-tab-open-to-side = Öppna &bredvid
ctx-tab-open-in-main = Öppna i &huvudrutan
ctx-tab-move-to-side = &Flytta åt sidan
ctx-tab-move-to-main = &Flytta till huvudrutan
ctx-tab-move-to-new-window = Flytta till ett &nytt fönster
ctx-tab-move-window-unsaved = Spara projektet först: ett andra fönster öppnar en fil på disken
ctx-tab-pin = Fäs&t den här fliken
ctx-tab-unpin = &Lossa den här fliken
tab-pinned-tooltip = Fäst: ”Stäng övriga” och ”Stäng alla” låter den vara öppen
create-book = Bok
create-part = Del
create-chapter = Kapitel
create-scene = Scen
create-note = Anteckning
create-note-folder = Anteckningsmapp
create-folder = Mapp
create-paratext = Paratext
create-paratext-folder = Paratextmapp
create-book-end = Bokens slut
create-story-bible-entry = Post i berättelsebibeln…
type-book-start = Bokens början
type-text = Text
new-item-book = Ny bok
new-item-part = Ny del
new-item-chapter = Nytt kapitel
new-item-note = Ny anteckning
new-item-note-folder = Ny anteckningsmapp
new-item-folder = Ny mapp
new-item-paratext = Ny paratext
new-item-paratext-folder = Ny paratextmapp
new-item-story-bible-entry = Ny post i berättelsebibeln
placement-inside = inuti
placement-after = efter
placement-after-parent = efter överordnat objekt
placement-top-level = översta nivån
ctx-promote = &Konvertera till
promote-chapter-folder = Kapitelmapp
promote-flat-chapter = Platt kapitel
promote-lossy-title = Ingenstans att spara texten
promote-lossy-text = En { $target } har ingen plats för detta: { $kinds }. Flytta eller töm texten först och konvertera sedan.
content-scene-text = Scentext
content-note-text = Anteckningstext
content-book-title = Boktitel
content-book-subtitle = Bokens undertitel
content-part-title = Delens titel
content-chapter-title = Kapitelrubrik
content-epigraph-text = Motto
content-paratext-text = Paratext
promote-blocked-title = Kapitlet är inte tomt
promote-blocked-text = { $count ->
       [one] Kapitlet innehåller fortfarande { $count } objekt. Flytta det eller lägg det i papperskorgen innan du konverterar kapitlet till ett platt kapitel.
      *[other] Kapitlet innehåller fortfarande { $count } objekt. Flytta dem eller lägg dem i papperskorgen innan du konverterar kapitlet till ett platt kapitel.
   }
inspector = Inspektör
inspector-empty = Öppna ett objekt för att granska det.
inspector-promote = Konvertera till…
statusbar-toggle-outline = Visa eller dölj dispositionen
statusbar-toggle-inspector = Visa eller dölj inspektören
statusbar-save-unsaved = Osparade ändringar. Klicka för att spara
statusbar-save-saved = Alla ändringar är sparade
statusbar-save-autosave = Automatisk sparning är på. Ändringarna sparas medan du skriver
statusbar-saving = Sparar…
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
session-toggle = Skrivpass: starta eller pausa en fokuserad sprint
session-configure = Ställ in skrivpassets ordmål och tidsgräns
session-configure-title = Skrivpass
session-word-goal = Ordmål
session-time-limit = Tidsgräns
session-time-limit-unit = min
session-no-goal = Inget mål
session-no-limit = Ingen gräns
session-reset = Nollställ skrivpasset
session-readout = { $words ->
       [one] { $words } ord
      *[other] { $words } ord
   } · { $time }
session-readout-timed = { $words ->
       [one] { $words } ord
      *[other] { $words } ord
   } · { $time } kvar
statusbar-focus-exit = Lämna distraktionsfritt läge
statusbar-focus-synopsis = Synopsis
statusbar-focus-go-prev = Föregående objekt (Alt+Uppil)
statusbar-focus-go-next = Nästa objekt (Alt+Nedpil)
rail-settings = Inställningar
language = Språk
theme = Tema
english = Engelska
french = Franska
light = Ljust
dark = Mörkt
settings-text-width = Textbredd
settings-preview-width = Bredd på sökförhandsvisningen
settings-autosave = Spara automatiskt till disk
settings-show-welcome = Visa startfönstret vid start
settings-show-welcome-tip = När detta är avstängt öppnas det senaste projektet i stället.
settings-check-for-updates = Sök efter nya versioner
settings-check-for-updates-tip = Högst en gång om dagen frågar Skribisto projektets webbplats vilken version som är aktuell. Förfrågan innehåller ingen identifierare, inte ens vilken version du kör: jämförelsen görs på den här datorn.
settings-title = Inställningar
settings-close = Stäng
settings-search = Sök i inställningarna
settings-reset = Återställ standardvärden
settings-done = Klar
settings-cancel = Avbryt
settings-apply = Verkställ
settings-ok = OK
settings-reset-confirm-title = Återställa alla inställningar till standardvärden?
settings-reset-confirm-body = Återställer programmets inställningar för utseende, redigerare och skrivande. Projektets egna inställningar, kortkommandon och sparade stilar påverkas inte. Det går inte att ångra.
settings-empty-title = Inga inställningar här ännu
settings-empty-hint = Det här avsnittet får alternativ i en kommande uppdatering.
settings-sec-appearance-behaviour = Utseende och beteende
settings-sec-editor = Redigerare
settings-sec-spelling = Stavning
settings-sec-backup = Säkerhetskopiering och synkronisering
settings-sec-compile = Kompilering och export
settings-page-appearance = Utseende
settings-field-image-size-policy = Stora bilder
settings-image-policy-ask = Fråga varje gång
settings-image-policy-keep = Behåll originalet
settings-image-policy-downscale = Optimera
settings-hint-image-size-policy = Vad som ska hända när en infogad bild är större än manuset behöver.
settings-page-notifications = Aviseringar
settings-page-scene = Scen
settings-page-synopsis = Synopsis
settings-page-notes = Anteckningar
settings-page-editor-behavior = Redigerarens beteende
settings-page-goals = Mål och ordantal
settings-page-games = Skrivlekar
settings-page-corkboard = Anslagstavla
settings-page-distraction-free = Distraktionsfritt
settings-page-dictionaries = Ordlistor
settings-page-autosave = Automatisk sparning
settings-page-export = Exportformat
settings-page-paratext = Paratextstrukturer
settings-page-keymap = Kortkommandon
settings-keymap-filter = Filtrera kortkommandon
settings-desc-sec-appearance-behaviour = Hur själva programmet ser ut och vad det gör när det startar.
settings-desc-sec-editor = Skrivytan: hur den ser ut och vad den gör medan du skriver.
settings-desc-sec-spelling = Stavningskontroll och ordlistorna bakom den.
settings-desc-sec-backup = Hur ditt arbete hamnar på disken och vilka kopior som sparas.
settings-desc-sec-compile = Det som lämnar Skribisto, och i vilken form.
settings-desc-sec-work = Inställningar som hör till det här projektet och följer med i dess fil.
settings-desc-sec-extensions = Sidor som tilläggen installerade här bidrar med.
settings-desc-group-typography = En sida per typ av text: typsnitt, storlek, radhöjd och avstånd.
settings-desc-appearance = Gränssnittsspråk, tema, textstorlek och startfönstret vid start.
settings-desc-notifications = Alla meddelanden som visats under den här sessionen, och de åtgärder du kan köra igen.
settings-desc-scene = Hur scenernas prosa sätts: typsnitt, storlek, radhöjd, indrag och avstånd.
settings-desc-synopsis = Hur synopsisrutan sätts, oberoende av manuset.
settings-desc-notes = Hur anteckningar sätts, oberoende av manuset.
settings-desc-corkboard = Kortens storlek, vad ett kort visar och hur tavlan ordnar dem.
settings-desc-distraction-free = Typografi, spaltbredd och kontrollremsa för skrivande i helskärm.
settings-desc-distraction-free-themes = Temabiblioteket för skrivande i helskärm: de medföljande och dina egna.
settings-desc-editor-behavior = Textbredd, synopsisens placering, skrivmaskinsrullning och markering kring markören.
settings-desc-punctuation = Den typografiska husstil som nya projekt utgår från. Alla projekt följer den om de inte har en egen.
settings-desc-goals = Mål för ord och tecken, och hur ord räknas.
settings-desc-games = Begränsningar du själv sätter upp medan du skriver utkast, som Alltid framåt.
settings-desc-spellcheck = Den enda brytaren som slår på och av stavningskontrollen överallt.
settings-desc-dictionaries = Installera, ta bort och bläddra bland stavningsordlistorna på den här datorn.
settings-desc-autosave = Om dina ändringar skrivs till disk av sig själva.
settings-desc-backup = När kopior tas, var de sparas och hur många. Alla projekt följer detta om de inte har en egen inställning.
settings-desc-export = Stilarna som varje export kompileras genom: de medföljande och dina egna.
settings-desc-paratext = De för- och eftertexter ett nytt projekt kan börja med.
settings-desc-keymap = Alla kortkommandon och vad de är kopplade till.
settings-page-user = Användare
settings-desc-user = Vem du är, för kommentarerna du skriver.
settings-group-identity = Identitet
settings-field-user-name = Ditt namn
settings-field-user-name-placeholder = Valfritt
settings-field-user-name-hint = Signerar kommentarerna och svaren du skriver. Skiljer sig från ett projekts författarnamn, som är bokens författarrad och följer med i filen.
settings-field-user-initials = Dina initialer
settings-field-user-initials-placeholder = Från ditt namn
settings-field-user-initials-hint = Det en ordbehandlare visar bredvid din kommentar i marginalen. Lämna tomt för att använda de som visas, hämtade från ditt namn.
settings-field-user-hint = Båda är valfria och gäller alla projekt på den här datorn. Om du ändrar dem signeras de kommentarer du skriver härnäst. Kommentarer som redan är skrivna behåller namnet de skrevs under.
settings-desc-author = Namnet som står på det här projektet.
settings-desc-structure = Om projektets kapitel är mappar eller enskilda objekt.
settings-desc-language = Språket som projektets prosa kontrolleras mot.
settings-desc-work-backup = Projektets egen säkerhetskopieringspolicy, eller den allmänna.
settings-desc-personal-dictionary = Orden som det här projektet betraktar som rättstavade.
settings-desc-tags = De färgkodade taggar som projektet märker sina objekt med.
settings-desc-templates = Anteckningsmallarna du kan infoga när du skriver i det här projektet.
settings-desc-text-replacements = Förkortningar som expanderas medan du skriver i det här projektet.
settings-desc-work-punctuation = Citattecken, tankstreck och mellanrum som projektets prosa följer.
settings-group-typography = Typografi
settings-group-writing-column = Skrivspalt
settings-group-writing-view = Skrivvy
settings-group-distraction-free-strip = Kontrollremsa
settings-group-theme = Tema
settings-group-language = Språk
settings-group-startup = Start
settings-group-updates = Uppdateringar
settings-group-autosave = Automatisk sparning
settings-field-typeface = Typsnitt
settings-field-size = Textstorlek
editor-size-changed-manuscript = Manusets textstorlek: { $percent }
editor-size-changed-synopsis = Synopsisens textstorlek: { $percent }
editor-size-changed-notes = Anteckningarnas textstorlek: { $percent }
editor-size-changed-corkboard = Textstorlek på anslagstavlans kort: { $percent }
editor-size-changed-corkboard-expanded = Textstorlek i den expanderade redigeraren: { $percent }
editor-size-changed-distraction-free = Textstorlek i distraktionsfritt läge: { $percent }
menu-text-size-increase = &Öka textstorleken
menu-text-size-decrease = &Minska textstorleken
menu-text-size-reset = Åte&rställ textstorleken
settings-field-line-height = Radhöjd
settings-field-first-line-indent = Indrag på första raden
settings-field-paragraph-spacing-before = Avstånd före stycke
settings-field-paragraph-spacing-after = Avstånd efter stycke
settings-field-column-width = Spaltbredd
settings-distraction-free-width-hint = Gäller bara när distraktionsfritt läge är på. Spaltbredderna för Scen, Synopsis och Anteckningar påverkas inte.
settings-distraction-free-title = Behåll objektets namn
settings-distraction-free-word-count = Behåll ordantalet
settings-distraction-free-session = Behåll skrivpasset
settings-distraction-free-go-to = Behåll knappen Gå till…
settings-distraction-free-go = Behåll knapparna Föregående och Nästa
settings-distraction-free-chrome-hint = Knappen Avsluta finns alltid kvar, vad du än väljer här. Den är din väg ut om Esc är upptagen.
settings-field-app-theme = Tema
settings-theme-light = Ljust
settings-theme-dark = Mörkt
settings-field-text-scale = Gränssnittets textstorlek
settings-field-language = Gränssnittsspråk
settings-synopsis-placement = Synopsisens placering
settings-synopsis-placement-none = Ingen
settings-synopsis-placement-top = Ovanför
settings-synopsis-placement-side = Bredvid
settings-typewriter = Skrivmaskinsrullning
settings-typewriter-tip = Håller raden du skriver på en fast höjd medan manuset rullar under den. Ett klick placerar fortfarande markören där du klickar.
settings-typewriter-position = Radens position
settings-typewriter-position-top-third = Översta tredjedelen
settings-typewriter-position-middle = Mitten
settings-typewriter-position-bottom-quarter = Nedersta fjärdedelen
settings-highlight-scope = Markering kring markören
settings-highlight-scope-none = Ingen
settings-highlight-scope-sentence = Mening
settings-highlight-scope-paragraph = Stycke
settings-highlight-scope-tip-none = Låt sidan vara ren. Inget skuggas medan du skriver.
settings-highlight-scope-tip-sentence = Skugga meningen du skriver så att den sticker ut från meningarna runt omkring.
settings-highlight-scope-tip-paragraph = Skugga hela stycket du skriver, så att avsnittet du arbetar med hålls i blickfånget.
settings-group-container-views = Behållarvyer
settings-remember-view = Kom ihåg den senaste vyn för varje objekttyp
settings-remember-view-tip = Öppna en behållare i den vy du senast använde för den typen
settings-remember-view-tip-more =
       En bok, en del och ett kapitel har var och en flera vyer (den egna sidan, hela
       manuset, hela synopsisen). Slå på detta så öppnas varje typ i den vy du senast
       valde för den: växla ett kapitel till Hela kapitlet, så visar nästa kapitel du
       öppnar också Hela kapitlet. Varje typ minns sin egen vy.
settings-group-counting = Ordräkning
settings-counting-auto = Automatisk (efter språk)
settings-counting-whitespace = Dela vid mellanslag
settings-counting-unicode-words = Unicode-ord
settings-counting-cjk-hybrid = CJK-anpassad (per tecken)
settings-counting-hint = Automatisk räknar kinesiska och japanska per tecken och alla andra språk per ord. Ändra bara om antalet ser fel ut för ditt språk.
settings-group-goals-display = Visning
settings-show-characters = Visa teckenantalet i statusfältet
settings-autosave-hint = Ändringar skrivs automatiskt till disk medan du skriver.
settings-sec-work = Projekt
settings-page-structure = Struktur
settings-page-author = Författare
settings-field-author-name = Författarnamn
settings-field-author-placeholder = Valfritt
settings-field-author-hint = Visas på den kompilerade titelsidan och i den exporterade filens metadata. Lämna tomt för att utelämna det. Detta är bokens författarrad. Namnet som dina kommentarer signeras med finns under Inställningar ▸ Användare.
settings-group-chapters = Kapitel
tidy-titles-title = Städa kapitelrubriker
tidy-titles-none = Inget kapitel och ingen del har en titel som bara upprepar sitt eget nummer.
tidy-titles-lead = { $count ->
           [one] Ett kapitel eller en del har inget annat än sitt eget nummer som titel.
          *[other] { $count } kapitel och delar har inget annat än sitt eget nummer som titel.
       }
tidy-titles-explain = När titlarna rensas benämns vart och ett av numret som boken redan känner till, samma nummer som exporten skriver ut. Inget annat ändras, och en enda ångring återställer alla.
menu-document-tidy-titles = Städa &kapitelrubriker…
settings-group-numbering = Numrering
settings-number-chapters = Numrera kapitel och delar
settings-number-chapters-tip = Kapitel och delar får ett nummer som räknas fram utifrån var de står i boken. Det visas bredvid titeln här och skrivs ut vid export.
settings-number-chapters-tip-more = Numret lagras aldrig i titeln, så det förblir rätt när du flyttar om, infogar eller tar bort. Stäng av detta så skriver exporten bara ut titlar, vad exportstilen än begär. Om ett enskilt kapitel ska lämnas utanför (en prolog, ett mellanspel) använder du i stället inspektörens brytare Numrering: kapitlet stannar i boken men får inget nummer.
settings-part-resets-chapter = Börja om kapitelnumreringen vid varje del
settings-part-resets-chapter-tip = Avstängt som standard: kapitlen numreras löpande genom bokens delar, så ”Del två” börjar med kapitel elva.
settings-part-resets-chapter-tip-more = Det är det vanliga i förlagsbranschen, och vad en läsare förväntar sig. Slå på det för en bok vars delar ska läsas som separata volymer, där var och en börjar med kapitel ett.
settings-chapter-flat = Platta kapitel
settings-chapter-flat-hint = På: ett kapitel är ett enda objekt. Du skriver i det, och det innehåller inga scener. Av: ett kapitel är en mapp. Du skriver fortfarande i det, men det kan också innehålla scener. Nya kapitel följer den här inställningen; befintliga konverteras med Konvertera till.
settings-styles-builtin = Inbyggda stilar
settings-styles-user = Mina stilar
settings-styles-builtin-badge = Inbyggd
settings-styles-duplicate = Duplicera
settings-styles-edit = Redigera
settings-styles-delete = Ta bort
settings-styles-import = Importera…
settings-styles-export = Exportera…
settings-styles-copy-suffix = (kopia)
settings-styles-json-filter = Exportstil
settings-styles-editor-title = Redigera stil
settings-styles-editor-none = Välj en egen stil för att redigera den, eller duplicera en inbyggd.
settings-styles-imported = Stilen har importerats
settings-styles-import-failed = Det gick inte att importera stilen
settings-styles-exported = Stilen har exporterats
settings-styles-export-failed = Det gick inte att exportera stilen
settings-styles-field-name = Namn
settings-styles-field-chapters = Kapitelrubriker
settings-styles-field-parts = Delrubriker
settings-styles-field-scene-break = Scenbrytning
settings-styles-field-major-scene-break = Större scenbrytning
settings-styles-field-spacing = Radavstånd
settings-styles-field-justify = Marginaljustera texten
settings-styles-field-notes = Ta med anteckningar
settings-styles-field-synopses = Ta med synopsisar
settings-styles-field-scene-titles = Ta med scenrubriker
settings-styles-field-epigraphs = Ta med motton
settings-styles-field-paratexts = Ta med paratexter
settings-styles-field-epigraph-placement = Mottots placering
settings-styles-epigraph-after = Efter titeln
settings-styles-epigraph-before = Före titeln
settings-styles-field-footnotes = Ta med fotnoter
settings-styles-field-footnote-numbering = Numrering av fotnoter
settings-styles-footnote-numbering-continuous = Löpande
settings-styles-footnote-numbering-per-chapter = Börja om i varje kapitel
settings-styles-footnote-numbering-per-book = Börja om i varje bok
settings-styles-field-images = Bilder
menu-image = &Bild
settings-styles-images-beside = Bredvid dokumentet
settings-styles-images-embed = Inuti dokumentet
settings-styles-images-omit = Utelämna
settings-styles-group-round-trip = Skicka till en redaktör
settings-styles-field-comments = Ta med kommentarer
settings-styles-field-round-trip-marks = Ta med returmarkörer
settings-styles-round-trip-hint = Endast DOCX och ODT. Markörerna är osynliga identifierare som gör att en returnerad fil uppdaterar det här projektet i stället för att hamna bredvid det som en andra kopia.
settings-styles-group-pages = Sidor
settings-styles-field-cover = Börja med omslaget
settings-styles-field-word-count = Ordantal på titelsidan
settings-styles-field-page-books = Ny sida vid varje bok
settings-styles-field-page-parts = Ny sida vid varje del
settings-styles-field-page-chapters = Ny sida vid varje kapitel
settings-styles-field-page-paratexts = Ny sida vid varje paratext
settings-styles-heading-none = Ingen rubrik
settings-styles-heading-numbered = Endast nummer
settings-styles-heading-title = Endast titel
settings-styles-heading-both = Nummer + titel
settings-styles-break-blank = Tom rad
settings-styles-break-none = Ingen
settings-styles-spacing-single = Enkelt
settings-styles-spacing-onehalf = 1½
settings-styles-spacing-double = Dubbelt
welcome-title = Välkommen till Skribisto
welcome-version = Version { $version }
welcome-update-available = Version { $version } finns tillgänglig
welcome-update-tip = Öppnar nedladdningssidan i webbläsaren.
welcome-search = Sök bland projekt
welcome-open = Öppna
welcome-new-work = Nytt projekt
welcome-create-from = Skapa från…
welcome-recent-works = Senaste projekt
welcome-empty-recents = Inga senaste projekt ännu.
welcome-no-matches = Inget av de senaste projekten matchar sökningen.
welcome-learn-soon = Guider och tips kommer snart.
welcome-about-blurb = Skribisto, skrivprogrammet omskrivet i Rust + Teksilo.
welcome-tagline = *En lugn plats att skriva långa texter.*
welcome-github = GitHub
welcome-discord = Discord
nav-works = Projekt
nav-examples = Exempel
nav-learn = Lär dig
nav-about = Om
synopsis = Synopsis
epigraph = Motto
pane-manuscript = Manus
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
corkboard-view-nested = Kapslad
corkboard-view-flat = Platt
corkboard-layout-hint = Kapslad visar en behållares direkta underobjekt. Öppna ett mappkort för att gå in i det. Platt visar alla scener i hela behållaren på en gång.
corkboard-card-size = Kortstorlek
corkboard-search-placeholder = Filtrera kort…
corkboard-empty-title = Inget här ännu
corkboard-empty-hint = Använd ”＋ Ny” ovan för att lägga till det första.
corkboard-new = Ny
corkboard-show-card-numbers = Numrera korten
corkboard-modal-size = Storlek på expanderad redigerare
corkboard-scope-hint = Gäller alla öppna anslagstavlor, i alla projekt.
corkboard-sort-manuscript = Manusordning
corkboard-sort-title-asc = Titel A–Ö
corkboard-sort-title-desc = Titel Ö–A
corkboard-card-number = Kort { $number }
duplicate = Duplicera
duplicate-n = { $count ->
       [one] Duplicera { $count } kort
      *[other] Duplicera { $count } kort
   }
set-label-n = { $count ->
       [one] Ange etikett för { $count } kort
      *[other] Ange etikett för { $count } kort
   }
move-to-trash-n = { $count ->
       [one] Flytta { $count } kort till papperskorgen
      *[other] Flytta { $count } kort till papperskorgen
   }
reveal-in-outline = Visa i dispositionen
corkboard-move-to = Flytta till…
corkboard-move-to-n = { $count ->
       [one] Flytta { $count } kort till…
      *[other] Flytta { $count } kort till…
   }
corkboard-move-picker-title = Flytta till…
corkboard-move-picker-empty = Inga pärmar ännu. Skapa en först.
corkboard-move-picker-cancel = Avbryt
corkboard-move-here = Flytta hit
corkboard-moved-ok = { $count ->
       [one] { $count } kort flyttat
      *[other] { $count } kort flyttade
   }
corkboard-move-into-self = En behållare kan inte flyttas in i sig själv. Välj en destination utanför den.
corkboard-move-failed = De korten kunde inte flyttas dit.
overview-col-title = Titel
overview-col-type = Typ
overview-col-label = Etikett
overview-col-tags = Taggar
overview-col-books = Böcker
overview-col-own-words = Ord
overview-col-total-words = Totalt
overview-row-count = { $count ->
       [one] { $count } objekt
      *[other] { $count } objekt
   }
overview-search-placeholder = Filtrera objekt…
overview-expand-all = Expandera alla
overview-collapse-all = Fäll ihop alla
overview-table-label = Innehåll
overview-empty-title = Inget här ännu
overview-gone-title = Den här behållaren finns inte längre
overview-gone-hint = Den har flyttats till papperskorgen. Återställ den eller stäng den här fliken.
overview-empty-hint = Använd ”＋ Ny” ovan för att lägga till det första.
corkboard-grid-label = Kort på anslagstavlan
corkboard-rename-field = Byt namn på objekt
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
corkboard-badge-end = Slut
settings-group-corkboard-layout = Layout
settings-group-corkboard-cards = Kort
corkboard-show-word-count = Visa ordantal på korten
text-heading = Text
no-content = Det här objektet har inget redigerbart innehåll.
untitled = Namnlös
placeholder-title = Titel…
placeholder-subtitle = Undertitel…
placeholder-chapter-title = Kapitelrubrik…
split-editor = Dela redigeraren
close-split-view = Stäng delad vy
drop-open-here = Öppna här
drop-open-to-side = Öppna bredvid
segment-chapter = Kapitel
segment-part = Del
segment-book = Bok
segment-pace = Tempo
segment-notes = Anteckningar
segment-story-bible = Berättelsebibel
segment-note-own = Anteckning
segment-note-details = Detaljer
segment-note-in-prose = I prosan
pace-placeholder = Tempoplaneraren visas här.
pace-empty-title = Planera bokens tempo
pace-empty-body = Ange ett ordmål och ett slutdatum, så räknar Skribisto ut det dagliga tempot som tar dig dit.
pace-start-planning = Börja planera
pace-section-schedule = Schema
pace-section-progress = Framsteg
pace-advancement = Avancemang
pace-goal = Ordmål
pace-deadline = Slutdatum
pace-active = Tempo aktivt
pace-day-mon = Mån
pace-day-tue = Tis
pace-day-wed = Ons
pace-day-thu = Tor
pace-day-fri = Fre
pace-day-sat = Lör
pace-day-sun = Sön
pace-card-written = ord skrivna
pace-card-of-goal = av målet
pace-card-rate = ord per skrivdag
pace-card-days-left = skrivdagar kvar
pace-card-streak = dagar i följd
pace-card-ahead = ord före tempot
pace-card-behind = ord efter tempot
pace-charts-empty = Framstegsdiagram visas här när ditt ordantal har registrerats vid en sparning.
pace-chart-progression = Skrivna ord jämfört med mål
pace-chart-words-per-day = Ord per dag
pace-series-actual = Faktiskt
pace-series-target = Mål
pace-series-words-per-day = Ord/dag
pace-daily-target-line = Jämnt tempo: { $count ->
       [one] { $count } ord/dag
      *[other] { $count } ord/dag
   }
pace-section-holidays = Lediga dagar
pace-section-milestones = Milstolpar
pace-holidays-none = Inga lediga dagar. Alla schemalagda dagar räknas.
pace-holiday-label = Namn på ledigheten
pace-add-holiday = Lägg till
pace-remove = Ta bort
pace-milestones-none = Inga milstolpar ännu. Ange en för en del eller ett kapitel i inspektören.
full-chapter = Hela kapitlet
full-part = Hela delen
full-book = Hela boken
full-synopsis = Hela synopsisen
rename = Byt namn
set-label = Ange etikett
insert-scene = Infoga scen
insert-chapter = Infoga kapitel
split-scene = Dela scenen
move-up = Flytta upp
move-down = Flytta ned
merge-with-previous = Slå ihop med föregående
move-to-trash = Flytta till papperskorgen
rename-chapter = Byt namn på kapitel
add-scene = Lägg till ny scen
add-chapter = Lägg till nytt kapitel
new-scene-title = Ny scen
placeholder-scene-name = Scenens namn…
menu-cut = Klipp ut
menu-copy = Kopiera
menu-paste = Klistra in
menu-paste-unformatted = Klistra in oformaterat
menu-select-all = Markera allt
format-bold = Fet
format-italic = Kursiv
format-underline = Understruken
format-strikethrough = Genomstruken
binder = Pärm
no-work = Inget projekt inläst
no-recent-works = Inga senaste projekt
switcher-open-section = Öppna just nu
switcher-recent-section = Senaste
switcher-this-window = det här fönstret
open-project-title = Öppna projekt
open-project-question = Hur vill du öppna ”{ $title }”?
open-in-new-window = Öppna i nytt fönster
open-here = Öppna här
binder-all = Alla pärmar
binder-show-all = Visa alla pärmar
binder-new = Ny pärm…
binder-default-name = Ny pärm
binder-item-count = { $count ->
       [one] { $count } objekt
      *[other] { $count } objekt
   }
binder-search-placeholder = Filtrera dispositionen…
binder-search-scope = Sök i alla pärmar
binder-trash-confirm-title = Flytta pärmen till papperskorgen?
binder-trash-confirm-text = ”{ $name }” och alla dess objekt flyttas till papperskorgen.
dialog-rename = Byt namn
dialog-new-binder = Ny pärm
dialog-set-label = Ange etikett
dialog-new-scene = Ny scen
close-work-question = Spara ändringarna innan projektet stängs?
quit-question = Spara ändringarna innan du avslutar?
quit-save-work-question = Spara ändringarna i { $title } innan du avslutar?
unsaved-changes = Projektet har osparade ändringar.
new-work-unsaved-question = Spara ändringarna innan du skapar ett nytt projekt?
open-work-unsaved-question = Spara ändringarna innan du öppnar ett annat projekt?
switch-backup-discard-title = Förkasta ändringarna i den här säkerhetskopian?
switch-backup-discard-text = Ändringar i en säkerhetskopia kan inte sparas i den. Använd Spara som eller Återställ för att behålla dem, eller förkasta dem och öppna det andra projektet.
switch-save-failed = Projektet kunde inte sparas och ersattes därför inte: { $error }
switch-save-not-started = Projektet kunde inte sparas och ersattes därför inte.
close-save-failed = Projektet kunde inte sparas och stängdes därför inte: { $error }
close-save-not-started = Projektet kunde inte sparas och stängdes därför inte.
save-not-started = Projektet kunde inte sparas.
could-not-open-work = Det gick inte att öppna ”{ $file }”
could-not-open-work-too-new = ”{ $file }” kräver en nyare Skribisto
could-not-open-work-too-new-detail = Sparat med Skribisto-format { $written_by }. Det krävs format { $requires } eller nyare för att öppna det, och den här versionen hanterar upp till format { $supported }. Uppdatera Skribisto för att öppna det.
could-not-open-work-importing = ”{ $file }” skrivs av en import
could-not-open-work-importing-detail = En import skriver över det här projektet och är inte klar. Öppna det när importen är färdig: öppnas det nu visar det projektet som importen ersätter, och nästa sparning skulle skriva tillbaka det projektet över importen.
could-not-open-example = Det gick inte att öppna exemplet: { $error }
could-not-open-link = Det gick inte att öppna { $url }: { $error }
could-not-create-work = Det gick inte att skapa projektet: { $error }
saving-as-file = Sparar som { $target }…
saving-as-folder = Sparar som { $target }/…
saved-as = Sparat i { $target }
save-error = Det gick inte att spara: { $error }
save-flush-failed = Sparningen genomfördes, men de här dokumenten kunde inte lämnas över till den och är fortfarande osparade: { $titles } ({ $error })
backup-error = Det gick inte att säkerhetskopiera: { $error }
backing-up = Säkerhetskopierar…
backup-nothing-open = Inget projekt är öppet att säkerhetskopiera.
backup-already-running = En säkerhetskopiering pågår redan.
backup-complete = Säkerhetskopieringen är klar ({ $ok } sparade, { $skipped } redan aktuella)
backup-partial = Säkerhetskopieringen är klar: { $ok } sparade, { $failed ->
       [one] { $failed } destination misslyckades
      *[other] { $failed } destinationer misslyckades
   }
backup-no-destination-title = Ingen plats för säkerhetskopior är tillgänglig
backup-no-destination-text = Ingen av de konfigurerade destinationerna för säkerhetskopior kan nås (en extern enhet kan till exempel vara urkopplad). Anslut den och försök igen, eller avsluta utan att säkerhetskopiera.
backup-failed-close-title = Säkerhetskopian kunde inte sparas
backup-failed-close-text = Ingen säkerhetskopia kunde skrivas före stängningen. Alla destinationer misslyckades (enheten kan ha tagits bort, eller vara full eller skrivskyddad). Åtgärda felet och försök igen, eller avsluta utan att säkerhetskopiera.
backup-choice-title = Säkerhetskopia
backup-choice-heading = Du har öppnat en säkerhetskopia
backup-choice-subtitle = Det här är en säkerhetskopia av ett projekt vid en viss tidpunkt.
backup-choice-subtitle-dated = Säkerhetskopian togs { $date }.
backup-choice-body = Du kan öppna den och redigera fritt, men ändringar kan bara behållas med Spara som. Den ursprungliga projektfilen rörs inte. Du kan också återställa projektet till exakt den här säkerhetskopian.
backup-choice-open = Öppna säkerhetskopian
backup-choice-restore = Återställ projektet till den här tidpunkten…
backup-choice-not-a-backup = Nej, öppna den som vanligt
backup-banner-title = Säkerhetskopia: ändringar kan inte sparas här
backup-banner-description = Använd Spara som för att behålla dina ändringar i en ny fil, eller Återställ för att ersätta det ursprungliga projektet med den här säkerhetskopian.
backup-banner-restore = Återställ…
backup-banner-save-as = Spara som…
backup-restore-original-missing = Det ursprungliga projektet att återställa över hittas inte. Använd Spara som för att behålla säkerhetskopian som ett nytt projekt i stället.
backup-restore-close-elsewhere-title = Projektet är öppet i ett annat fönster
backup-restore-close-elsewhere-text = Projektet du återställer över är öppet i ett annat fönster. Stäng det där först och försök sedan igen.
backup-restore-focus-window = Gå till det fönstret
backup-restore-confirm-title = Återställa den här säkerhetskopian?
backup-restore-confirm-text = Den nuvarande versionen av projektet kopieras undan som en extra säkerhetskopia innan den ersätts med den här.
backup-restore-confirm-ok = Återställ
backup-restore-error = Det gick inte att återställa: { $error }
backup-restored-ok = Projektet har återställts.
backup-restored-with-safety = Projektet har återställts. Din tidigare version sparades i { $path }.
close-backup-discard-title = Förkasta ändringarna i den här säkerhetskopian?
close-backup-discard-text = Ändringar i en säkerhetskopia kan inte sparas i den. Använd Spara som för att behålla dem, eller förkasta dem och stäng.
quit-backup-discard-title = Förkasta ändringarna och avsluta?
quit-backup-discard-work-question = Förkasta ändringarna i { $title } och avsluta?
quit-backup-discard-text = Ändringar i en säkerhetskopia kan inte sparas i den. Använd Spara som för att behålla dem, eller förkasta dem och avsluta.
backup-nudge-text = Inga säkerhetskopior är inställda för det här projektet.
backup-nudge-action = Ställ in säkerhetskopiering…
menu-backups-list = Säkerhets&kopior…
backups-title = Säkerhetskopior
backups-loading = Söker efter säkerhetskopior…
backups-empty = Inga säkerhetskopior har hittats för det här projektet ännu.
backups-open = Öppna
backups-reveal = Visa i filhanteraren
backups-delete = Ta bort den här säkerhetskopian
backups-delete-confirm-title = Ta bort den här säkerhetskopian?
backups-delete-confirm-text = ”{ $name }” tas bort permanent. Det går inte att ångra.
backups-delete-error = Det gick inte att ta bort säkerhetskopian: { $error }
backups-refresh = Uppdatera
backups-close = Stäng
settings-page-backup = Standard för säkerhetskopiering
settings-page-work-backup = Säkerhetskopior
settings-backup-general-title = Standardinställningar för säkerhetskopiering
settings-backup-work-title = Säkerhetskopior för det här projektet
settings-backup-inherit = Använd de allmänna inställningarna för säkerhetskopiering
settings-backup-inheriting = Det här projektet använder de allmänna inställningarna för säkerhetskopiering.
settings-backup-none-hint = Inga automatiska säkerhetskopior är inställda (alla utlösare är avstängda).
settings-backup-last = Senaste säkerhetskopia: { $date }
settings-backup-last-never = Inga säkerhetskopior ännu.
settings-backup-open-list = Öppna listan över säkerhetskopior…
settings-backup-triggers = När säkerhetskopior ska tas
settings-backup-on-close = Säkerhetskopiera när projektet stängs
settings-backup-on-open = Säkerhetskopiera när projektet öppnas
settings-backup-interval = Säkerhetskopiera regelbundet, var
settings-backup-destinations = Destinationer för säkerhetskopior
settings-backup-default-location = Standardplats
settings-backup-usage-since = { $count ->
       [one] { $count } säkerhetskopia · { $size } · äldsta { $oldest }
      *[other] { $count } säkerhetskopior · { $size } · äldsta { $oldest }
   }
settings-backup-usage = { $count ->
       [one] { $count } säkerhetskopia · { $size }
      *[other] { $count } säkerhetskopior · { $size }
   }
settings-backup-usage-empty = Inget sparat här ännu
settings-backup-usage-measuring = Mäter…
settings-backup-reveal-root = Visa mappen
settings-backup-dest-none = Inga destinationer. Säkerhetskopior sparas bredvid projektet.
settings-backup-dest-remove = Ta bort
settings-backup-dest-add = Lägg till mapp…
settings-backup-dest-refresh = Uppdatera
settings-backup-retention = Hur många som ska behållas
settings-backup-retention-tiered = Stegvis
settings-backup-retention-keep-n = Behåll de senaste N
settings-backup-retention-tip = Hur gamla säkerhetskopior gallras.
settings-backup-retention-tip-more = Stegvis behåller en säkerhetskopia per timme under ett dygn, per dag under en vecka, per vecka under en månad och per månad därefter, så att den senaste historiken förblir tät medan den äldre glesas ut. ”Behåll de senaste N” behåller helt enkelt de N senaste kopiorna. I båda lägena behålls alltid de nyaste kopiorna (minimiantalet nedan).
settings-backup-gfs-hourly = Varje timme (senaste 24 h)
settings-backup-gfs-daily = Varje dag (senaste veckan)
settings-backup-gfs-weekly = Varje vecka (senaste månaden)
settings-backup-gfs-monthly = Varje månad
settings-backup-keep-n = Antal att behålla
settings-backup-min-keep = Behåll alltid minst
settings-backup-dedup = Hoppa över säkerhetskopian om inget har ändrats
new-work-title = Nytt projekt
new-work-close = Stäng
new-work-name = Projektnamn
new-work-name-placeholder = Namnlös
new-work-author = Författare
new-work-author-placeholder = Valfritt
new-work-format = Format
new-work-single-file = En enda fil
new-work-bundle = Mapp
new-work-convert-later = Du kan byta format senare.
new-work-location = Plats
new-work-will-create = Skapar
new-work-language = Standardspråk
new-work-language-hint = Används för nya texter och stavningskontroll. Varje text kan ställas om till ett annat språk.
new-work-template = Mall
new-work-template-none = Ingen
new-work-template-empty-novel = Tom roman
new-work-template-light-novel = Light novel
new-work-template-novel = Roman
new-work-template-novel-in-parts = Roman i delar
new-work-template-notebook = Anteckningsbok
new-work-cancel = Avbryt
new-work-create = Skapa projekt
new-work-step-details = Detaljer
new-work-step-language = Språk och struktur
new-work-step-template = Mall
new-work-back = Tillbaka
new-work-next = Nästa
new-work-documents-title = Nytt projekt från dokument
new-work-step-import = Import
new-work-create-and-import = Skapa och importera…
new-work-documents-next-title = Dina dokument kommer härnäst
new-work-documents-next-body = När projektet skapas öppnas importguiden över det. Där väljer du dokumenten, granskar strukturen som Skribisto läser ut ur dem och väljer var den ska hamna. Inget skrivs in i projektet förrän du bekräftar det där.
new-work-documents-no-template = Projektet börjar avsiktligt tomt: ingen mall, så att de importerade dokumenten är det enda som finns i det.
new-work-documents-chapter-scene-hint = Gäller kapitlen som importen skapar. Du kan ändra det senare i projektets inställningar.
new-work-single-file-desc = Ett .skrib-arkiv (zip). Lätt att flytta och att säkerhetskopiera.
new-work-bundle-desc = En mapp som innehåller alla texter och resurser. Fungerar bättre med versionshantering.
new-work-template-none-count = tom pärm
new-work-template-empty-novel-count = 1 kapitel
new-work-template-light-novel-count = 15 kapitel
new-work-template-novel-count = 20 kapitel
new-work-template-novel-in-parts-count = 3 delar, 24 kapitel
new-work-template-notebook-count = fria anteckningar
new-work-chapter-scene = Platta kapitel
new-work-chapter-scene-tip = Varje kapitel är ett enda objekt som du skriver direkt i, utan scener under. Lämna detta avstängt för den klassiska uppläggningen, där ett kapitel är en mapp: du skriver direkt i den också, men den kan även innehålla scener.
new-work-chapter-scene-tip-more = Du skriver i kapitlet i båda fallen. Den enda skillnaden är om det kan *innehålla* scener. Skribistos pärmträd är bara organisatoriskt, så båda uppläggen kompileras till samma bok, du kan blanda dem fritt, och Konvertera till flyttar ett kapitel mellan de två utan att ett ord går förlorat.
new-work-name-required = Ange ett namn på projektet
new-work-name-invalid = Namnet innehåller inga användbara tecken
new-work-location-required = Välj en plats
new-work-location-missing = Mappen finns inte
new-work-location-not-folder = Sökvägen är inte en mapp
new-work-location-readonly = Mappen är inte skrivbar
new-work-target-exists = Det finns redan ett projekt med det här namnet i mappen. Du får frågan innan det ersätts när projektet skapas
new-work-target-is-folder = Det finns redan en mapp med det här namnet på platsen. Välj ett annat namn
new-work-overwrite-title = Ersätta det befintliga projektet?
new-work-overwrite-text = ”{ $name }” finns redan i den här mappen. Vill du ersätta det med det nya projektet?
new-work-target-open-title = Projektet är öppet
new-work-target-open-text = ”{ $name }” är öppet i Skribisto. Om ett nytt projekt skapas i dess ställe ersätts projektet du arbetar med, och nästa sparning skulle skriva tillbaka det projektet över det nya. Välj ett annat namn eller en annan mapp, eller stäng projektet först.
target-importing-title = En import skriver det här projektet
target-importing-text = En import skriver fortfarande ”{ $name }”. Vänta tills den är klar, eller välj ett annat namn.
backup-restore-importing-text = En import skriver fortfarande ”{ $name }”, projektet som den här säkerhetskopian skulle återställas över. Återställ säkerhetskopian när importen är klar.
new-work-manuscript = Manus
new-work-notes = Anteckningar
new-work-research = Research
new-work-notebook = Anteckningsbok
new-work-scene = Scen
new-work-note = Anteckning
new-work-front-matter = Förtext
new-work-back-matter = Eftertext
new-work-paratext = Bokstruktur
new-work-paratext-none = Ingen struktur
new-work-paratext-hint = De förtexter och eftertexter som en tradition inleder och avslutar en bok med. Du kan flytta, byta namn på eller ta bort vad som helst av det efteråt.
new-work-tags = Taggar
new-work-tags-none = Inga taggar
new-work-tags-hint = En startpalett för att tagga personer, platser och allt annat. Valfritt, och varje tagg kan få nytt namn, ny färg eller tas bort efteråt.
new-work-note-templates = Anteckningsmallar
new-work-note-templates-none = Inga mallar
new-work-note-templates-hint = Formen som en anteckning i berättelsebibeln börjar med. Valfritt, och varje mall kan redigeras eller tas bort efteråt.
new-work-characters = Personer
new-work-places = Platser
import-plume-title = Importera Plume Creator-projekt
import-plume-source = Plume-projekt
import-plume-source-hint = Välj en .plume- eller .plume_backup-fil (valfri version av Plume Creator).
import-plume-trash-warning = ⚠ Objekt i papperskorgen och borttagna objekt överförs inte.
import-plume-source-required = Välj en Plume-projektfil
import-plume-source-missing = Filen finns inte
import-plume-source-not-file = Sökvägen är inte en fil
import-plume-manuscript-binder = Manus
import-plume-story-bible-binder = Berättelsebibel
import-plume-progress-title = Importerar Plume-projekt…
import-plume-done = { $imported ->
       [one] { $imported } objekt importerat.
      *[other] { $imported } objekt importerade.
   } { $skipped ->
       [one] { $skipped } objekt i papperskorgen överfördes inte.
      *[other] { $skipped } objekt i papperskorgen överfördes inte.
   }
import-plume-nested-too-deep = Projektet importerades inte: dess del ”{ $part }” är kapslad i mer än { $limit } nivåer. Inget Plume Creator-projekt kommer i närheten av det, och att läsa det kunde få Skribisto att krascha. Inget skrevs.
import-plume-declares-entities = Projektet importerades inte: dess del ”{ $part }” deklarerar XML-entiteter, vilket inget Plume Creator-projekt gör. Att expandera dem kunde kräva mer minne än datorn har och få Skribisto att krascha. Inget skrevs.
import-manuskript-title = Importera Manuskript-projekt
import-manuskript-source = Manuskript-projekt
import-manuskript-source-hint = Välj .msk-filen eller projektmappen (valfri version av Manuskript).
import-manuskript-source-file = Välj fil…
import-manuskript-source-folder = Välj mapp…
import-manuskript-source-required = Välj ett Manuskript-projekt
import-manuskript-source-missing = Filen eller mappen finns inte
import-manuskript-manuscript-binder = Manus
import-manuskript-story-bible-binder = Berättelsebibel
import-manuskript-characters-group = Personer
import-manuskript-world-group = Världen
import-manuskript-plots-group = Intriger
import-manuskript-project-info-note = Projektinformation
import-manuskript-summary-note = Sammanfattning
import-manuskript-importance-minor = Mindre viktig
import-manuskript-importance-secondary = Sekundär
import-manuskript-importance-main = Huvudperson
import-manuskript-progress-title = Importerar Manuskript-projekt…
import-manuskript-done = { $imported ->
       [one] { $imported } objekt importerat.
      *[other] { $imported } objekt importerade.
   } { $revisions ->
       [0] { "" }
       [one] { $revisions } tidigare version följde med.
      *[other] { $revisions } tidigare versioner följde med.
   }
import-manuskript-nested-too-deep = Projektet importerades inte: dess fil ”{ $part }” är kapslad i mer än { $limit } nivåer. Inget Manuskript-projekt kommer i närheten av det, och att läsa det kunde få Skribisto att krascha. Inget skrevs.
import-manuskript-folders-too-deep = Projektet importerades inte: en av dess filer ligger mer än { $limit } mappar ned. Inget Manuskript-projekt kommer i närheten av det, och att läsa det kunde få Skribisto att krascha. Inget skrevs.
import-manuskript-declares-entities = Projektet importerades inte: dess fil ”{ $part }” deklarerar XML-entiteter, vilket inget Manuskript-projekt gör. Att expandera dem kunde kräva mer minne än datorn har och få Skribisto att krascha. Inget skrevs.
import-target-open-title = Projektet är öppet
import-target-open-text = ”{ $name }” är öppet i Skribisto. En import över det skulle ersätta projektet du arbetar med, och nästa sparning skulle skriva tillbaka det projektet över importen. Välj ett annat filnamn, eller stäng projektet först.
import-warnings-announcement = { $result } { $notice }.
export-title = Exportera
export-close = Stäng
export-scope-label = Vad
export-format-label = Format
export-style-label = Stil
export-path-label = Fil
export-browse = Bläddra…
export-show-non-exportable = Visa även det som inte exporteras
export-choose-empty = Det finns inget projekt att välja från.
export-cancel = Avbryt
export-export = Exportera
export-save-dialog-title = Exportera till fil
export-section-what = Vad som ska exporteras
export-section-style = Stil
export-section-destination = Destination
export-custom-selection = Eget urval
export-selected-count = { $count ->
       [one] { $count } markerat
      *[other] { $count } markerade
   }
export-chip-chapters-none = Kapitel: inga rubriker
export-chip-chapters-numbered = Kapitel: numrerade
export-chip-chapters-title = Kapitel: endast titel
export-chip-chapters-both = Kapitel: nummer + titel
export-chip-scene-break-glyph = Scenbrytning: { $glyph }
export-chip-scene-break-blank = Scenbrytning: tom rad
export-chip-scene-break-none = Scenbrytning: ingen
export-chip-major-break-glyph = Större brytning: { $glyph }
export-chip-major-break-blank = Större brytning: tom rad
export-chip-major-break-none = Större brytning: ingen
export-chip-major-break-same = Båda brytningsnivåerna lika
export-chip-spacing-single = Radavstånd: enkelt
export-chip-spacing-onehalf = Radavstånd: 1½
export-chip-spacing-double = Radavstånd: dubbelt
export-chip-notes-included = Anteckningar medtagna
export-chip-notes-excluded = Anteckningar uteslutna
export-format-docx = Word
export-format-odt = LibreOffice
export-format-html = HTML
export-format-markdown = Markdown
export-format-djot = Djot
export-format-text = Text
export-format-latex = LaTeX
export-format-epub = EPUB
export-format-pdf = PDF
export-overwrite-title = Ersätta den befintliga filen?
export-overwrite-text = ”{ $name }” finns redan. Vill du ersätta den?
export-progress-title = Exporterar…
export-cancelled = Exporten avbröts
export-done = { $count ->
       [one] { $count } objekt exporterat
      *[other] { $count } objekt exporterade
   }
export-open-file = Öppna
export-show-in-folder = Visa i mappen
export-error-title = Det gick inte att exportera
export-error-details = Detaljer
backup-progress-start = Startar…
backup-progress-retention = Rensar gamla säkerhetskopior…
backup-progress-done = Klar
backup-progress-destination = Destination { $i } av { $n }
backup-details = Detaljer
backup-issues-title = Problem med säkerhetskopieringen
backup-failed-title = Säkerhetskopieringen misslyckades
backup-complete-prune-warning = Säkerhetskopieringen är klar ({ $ok } sparade, { $skipped } redan aktuella). Vissa gamla säkerhetskopior kunde inte tas bort
search = Sök
search-query-placeholder = Sök…
search-replace-placeholder = Ersätt med…
search-replace-toggle = Visa eller dölj ersättning
search-replace-all = Ersätt alla
search-opt-case = Matcha skiftläge
search-opt-whole-word = Hela ord
search-opt-diacritics = Matcha accenter
search-scope-body = Brödtext
search-scope-title = Titel
search-scope-synopsis = Synopsis
search-scope-label = Etikett
search-facet-book = Böcker
search-facet-part = Delar
search-facet-chapter = Kapitel
search-facet-scene = Scener
search-facet-note = Anteckningar
search-facet-folder = Mappar
search-tip-case = Matcha skiftläge: behandla versaler och gemener som olika, så att ”Elena” och ”elena” blir separata träffar.
search-tip-whole-word = Hela ord: matcha bara fullständiga ord, så att ”bil” inte hittas inuti ”bild”.
search-tip-diacritics = Matcha accenter: behandla bokstäver med accent som olika, så att ”cafe” inte matchar ”café”.
search-tip-body = Brödtext: sök i prosan i scener och anteckningar.
search-tip-title = Titel: sök i titlarna på pärmens objekt.
search-tip-synopsis = Synopsis: sök i varje skrivobjekts sammanfattning.
search-tip-label = Etikett: sök i den korta anteckning som visas under ett objekts titel i pärmen.
search-tip-comment = Kommentarer: sök i texten i kommentarstrådar och deras svar. En kommentar handlar om manuset snarare än är en del av det, så den har en egen brytare, och vid ersättning lämnas kommentarsträffar omarkerade tills du markerar dem.
search-tip-book = Böcker: bokbehållaren och dess start- och slutmarkörer.
search-tip-part = Delar: avdelare på delnivå.
search-tip-chapter = Kapitel: kapitel, hur projektet än lagrar dem.
search-tip-scene = Scener: objekten som innehåller din prosa.
search-tip-note = Anteckningar: fria anteckningar.
search-tip-folder = Mappar: vanliga organiseringsmappar och avdelare.
search-tip-paratext = En text som hör till boken men inte till berättelsen: ett förord, en dedikation, ett efterord. Räknas aldrig in i manuset.
search-tip-preserve-case = Bevara skiftläge: ersättningen tar samma skiftläge som det som hittades, så att ”ELENA” blir ”MARTA” och ”Elena” blir ”Marta”.
search-tip-replace = Ersätt: visa ersättningsfältet och Ersätt alla.
search-error = Sökningen misslyckades: { $message }
search-no-matches = Inga träffar
search-count =
       { $matches ->
           [one] { $matches } träff
          *[other] { $matches } träffar
       } i { $items ->
           [one] { $items } dokument
          *[other] { $items } dokument
       }
search-count-truncated =
       { $matches ->
           [one] { $matches } träff
          *[other] { $matches } träffar
       } i { $items ->
           [one] { $items } dokument
          *[other] { $items } dokument
       } (endast de första resultaten)
search-occurrences = { $count }
search-field-body = Brödtext
search-field-title = Titel
search-field-synopsis = Synopsis
search-field-label = Etikett
search-field-epigraph = Motto
search-field-comment = Kommentar
search-field-comment-reply = Svar
search-field-footnote = Fotnot
search-include-in-replace = Ta med i Ersätt alla
search-collapse-all = Fäll ihop alla resultat
search-replace-here = Ersätt här
search-dismiss = Ta bort från resultaten
search-undo-dismiss = Hämta tillbaka det senast borttagna resultatet
search-replace-nothing = (ingenting)
search-replace-confirm-title = Ersätta alla träffar?
search-replace-confirm-text =
       Ersätta { $occurrences ->
           [one] { $occurrences } förekomst
          *[other] { $occurrences } förekomster
       } av ”{ $query }” med ”{ $replacement }” i { $items ->
           [one] { $items } dokument
          *[other] { $items } dokument
       }? Du kan ångra detta från aviseringen efteråt.
search-replace-done-title = Ersättningen är klar
search-replace-done =
       { $occurrences ->
           [one] { $occurrences } förekomst ersatt
          *[other] { $occurrences } förekomster ersatta
       } i { $items ->
           [one] { $items } dokument
          *[other] { $items } dokument
       }.
search-replace-done-skipped =
       { $occurrences ->
           [one] { $occurrences } förekomst ersatt
          *[other] { $occurrences } förekomster ersatta
       } i { $items ->
           [one] { $items } dokument
          *[other] { $items } dokument
       }. { $skipped ->
           [one] { $skipped } fält hoppades över
          *[other] { $skipped } fält hoppades över
       } (ändrade sedan sökningen).
search-replace-undo = Ångra
search-replace-failed-title = Ersättningen misslyckades
search-replace-skipped-title = Inget ersattes
search-replace-skipped-body =
       { $fields ->
           [one] Den här texten
          *[other] { $fields } av de här texterna
       } har ändrats sedan sökningen och { $fields ->
           [one] lämnades därför orörd
          *[other] lämnades därför orörda
       }. Sök igen för att se var orden finns nu.
search-replace-undo-failed-title = Ångringen misslyckades
search-preview = Förhandsvisning
search-preview-empty = Välj ett resultat för att förhandsvisa det här
search-preview-no-prose = Den här träffen har ingen redigerbar text
search-preview-prompt = Kör en sökning för att se en förhandsvisning här.
search-preview-open-search = Sök i projektet
search-preview-footnote-prompt = Den här träffen finns i en fotnots egen text. Öppna den i panelen Fotnoter för att se och redigera den.
search-preview-open-footnotes = Öppna Fotnoter
find-placeholder = Sök i det här dokumentet…
find-count = { $current } av { $total }
find-no-results = Inga resultat
find-close = Stäng sökningen
find-previous = Föregående träff (Skift+Retur)
find-next = Nästa träff (Retur)
find-opt-case = Matcha skiftläge
find-opt-whole-word = Hela ord
find-replace-toggle = Visa eller dölj ersättning (Ctrl+R)
find-replace-placeholder = Ersätt med…
find-replace = Ersätt
find-replace-all = Ersätt alla
find-preserve-case = Bevara skiftläge
dict-download-title = Hämtar { $name }…
dict-download-done = { $name } har installerats
dict-download-failed = Det gick inte att hämta { $name }: { $error }
dict-removed = { $name } har tagits bort
dict-accept-first = Godkänn licensen innan du hämtar { $name }
settings-dict-tab-installed = Installerade
settings-dict-tab-get-more = Hämta fler
settings-dict-tab-personal = Egna ord
dict-installed-empty = Inga ordlistor har hittats på den här datorn ännu.
dict-get-more-search = Sök språk
dict-system-badge = i ditt system
dict-unusable-badge = oanvändbar
dict-download-button = Hämta
dict-downloading = Hämtar…
dict-installed-label = Installerad
dict-remove = Ta bort
dict-view-license = Visa licens
dict-approx-size = ~{ $size }
dict-personal-empty = Inga egna ord i det här projektet ännu.
dict-personal-add = Lägg till
dict-personal-placeholder = Lägg till ett ord…
dict-missing-toast = { $count ->
       [one] Projektet använder { $count } ordlista som du inte har installerad
      *[other] Projektet använder { $count } ordlistor som du inte har installerade
   }
dict-missing-action = Hämta ordlistor
dict-license-title = Licens för { $name }
dict-license-accept = Godkänn och hämta
dict-license-cancel = Avbryt
dict-license-close = Stäng
dict-add-button = Lägg till ordlista…
dict-add-title = Lägg till en ordlista
dict-add-close = Stäng
dict-add-name = Namn
dict-add-name-placeholder = t.ex. Min latinska ordlista
dict-add-code = Språkkod
dict-add-code-placeholder = t.ex. la eller fr-FR-x-custom
dict-add-code-hint = En kort beteckning som du väljer själv. Det är den du väljer som ett dokuments språk.
dict-add-aff = Affixfil (.aff)
dict-add-dic = Ordlistefil (.dic)
dict-add-submit = Lägg till
dict-add-cancel = Avbryt
dict-add-name-required = Ge ordlistan ett namn.
dict-add-code-required = Ange en språkkod.
dict-add-code-invalid = Använd bara bokstäver, siffror och - _ .
dict-add-code-reserved = Det är koden för en inbyggd ordlista. Välj en annan.
dict-add-file-required = Välj en fil.
dict-add-file-missing = Filen finns inte.
dict-add-code-taken = En ordlista med den koden är redan installerad.
dict-add-done = { $name } har lagts till
dict-add-unusable = Filerna är inte en användbar ordlista: { $error }
dict-add-failed = Det gick inte att lägga till ordlistan: { $error }
inspector-export = Export
inspector-exportable = Ta med i exporter
ctx-number = N&umrera det här kapitlet
ctx-unnumber = N&umrera inte det här kapitlet
inspector-numbering = Numrering
inspector-numbered = Numrerat
inspector-numbered-tip = Kapitlet får sin plats i bokens numrering. Stäng av det för en prolog, en epilog eller ett mellanspel.
inspector-numbered-tip-more = Ett onumrerat kapitel finns kvar i boken precis som det var: rubriken, prosan och ordantalet påverkas inte. Det skrivs bara inte ut något nummer, och inget nummer förbrukas: kapitlet efter en prolog är kapitel ett, inte kapitel två. Att i stället utesluta det från exporten är en annan brytare, ovanför, och den tar bort kapitlet från boken helt och hållet.
inspector-apply-to-children = Tillämpa på underobjekt
inspector-milestone = Datum för milstolpe
inspector-milestone-none = Inget datum
inspector-milestone-clear = Rensa
inspector-tags = Taggar
inspector-aliases = Även känd som
inspector-dict-language = Språk
inspector-apply-language-to-children = Tillämpa språket på underobjekt
settings-page-language = Språk
settings-field-dict-language = Språk
dict-tradeoff-hint = Varje extra språk godtar fler ord, så färre fel upptäcks.
lang-inherit-hint = Ärvt: den här scenen använder bokens eller projektets språk.
lang-pill-list = Språk
lang-pill-add = Lägg till ett språk
lang-pill-remove = Ta bort { $name }
lang-pill-mute = Stäng av stavningskontrollen för { $name }
lang-pill-unmute = Slå på stavningskontrollen för { $name }
settings-page-personal-dictionary = Egen ordlista
settings-user-dict-desc = Ord som du lägger till här betraktas som rättstavade överallt i programmet och markeras aldrig igen.
settings-user-dict-add-placeholder = Lägg till ett ord…
settings-user-dict-add = Lägg till ord
settings-user-dict-search = Filtrera ord
settings-user-dict-count = { $count ->
       [one] { $count } ord
      *[other] { $count } ord
   }
settings-user-dict-rename = Byt namn
settings-user-dict-remove = Ta bort
settings-user-dict-empty = Inga ord ännu. Lägg till ord som stavningskontrollen ska strunta i, eller importera en lista.
settings-user-dict-import = Importera…
settings-user-dict-export = Exportera…
settings-user-dict-import-tip = Lägg till ord från en vanlig textfil, ett ord per rad. Befintliga ord behålls.
settings-user-dict-export-tip = Spara hela listan i en vanlig textfil, ett ord per rad.
settings-user-dict-txt-filter = Textfiler
settings-user-dict-duplicate = Finns redan i ordlistan
settings-user-dict-imported = { $count ->
       [one] { $count } ord importerat
      *[other] { $count } ord importerade
   } ({ $duplicates } fanns redan).
settings-user-dict-import-failed = Det gick inte att läsa ordlistan: { $error }
settings-user-dict-exported = { $count ->
       [one] { $count } ord sparat.
      *[other] { $count } ord sparade.
   }
settings-user-dict-export-failed = Det gick inte att spara ordlistan: { $error }
settings-page-text-replacements = Textersättningar
settings-text-repl-desc = Ersätt förkortningar med hela texten medan du skriver: ”mvh” blir ”med vänliga hälsningar” i samma stund som du skriver ett mellanslag eller ett skiljetecken.
settings-text-repl-enable = Använd textersättningar i det här projektet
settings-text-repl-disabled-hint = Slå på detta för att definiera förkortningar som expanderas medan du skriver.
settings-text-repl-add = Lägg till regel
settings-text-repl-trigger-placeholder = Förkortning
settings-text-repl-replacement-placeholder = Vad den blir
settings-text-repl-added = ”{ $trigger }” har lagts till
settings-text-repl-duplicate = ”{ $trigger }” har redan en regel
settings-text-repl-filter = Filtrera regler
settings-text-repl-count = { $n ->
       [one] 1 regel
      *[other] { $n } regler
   }
settings-text-repl-row-enabled = Använd den här regeln
settings-text-repl-delete = Ta bort regeln för { $trigger }
settings-text-repl-deleted = Regeln för ”{ $trigger }” har tagits bort
settings-text-repl-empty = Inga regler ännu.
settings-text-repl-csv-filter = CSV-filer
settings-text-repl-import = Importera…
settings-text-repl-export = Exportera…
settings-text-repl-imported = Importerade: { $added }, överhoppade: { $skipped }
settings-text-repl-exported = { $n ->
       [one] 1 regel exporterad
      *[other] { $n } regler exporterade
   }
editor-menu-no-suggestions = Inga förslag
editor-menu-add-to-dictionary = Lägg till ”{ $word }” i ordlistan
editor-menu-add-words-to-dictionary = Lägg till de markerade orden i ordlistan
editor-dict-added = ”{ $word }” har lagts till i din ordlista.
editor-dict-added-multi = { $count ->
       [one] { $count } ord har lagts till i din ordlista.
      *[other] { $count } ord har lagts till i din ordlista.
   }
toast-undo = Ångra
undo-superseded-title = Det steget är inte längre det senaste
undo-superseded-body = Något annat har ändrats i projektet sedan dess. Ångra skulle ta tillbaka det i stället, så inget har ångrats. Inget som det här steget gjorde har återställts. Använd Redigera ▸ Ångra för att själv gå bakåt genom historiken.
undo-failed = Ångringen misslyckades: { $error }
menu-edit = &Redigera
menu-edit-undo = &Ångra
menu-edit-redo = &Gör om
menu-edit-undo-target = &Ångra { $target }
menu-edit-redo-target = &Gör om { $target }
shortcut-name-edit-undo = Ångra
shortcut-name-edit-redo = Gör om
undo-target-typing = skrivning
undo-target-project = den senaste ändringen i projektet
prose-history-reset-title = Skrivhistoriken har nollställts
prose-history-reset-body = { $count ->
       [one] En öppen scen återställdes från ett tidigare tillstånd, så dess skrivhistorik gäller inte längre.
      *[other] { $count } öppna scener återställdes från ett tidigare tillstånd, så deras skrivhistorik gäller inte längre.
   }
undo-target-trash = flytt till papperskorgen
undo-target-restore = återställning från papperskorgen
undo-target-delete-forever = permanent borttagning
undo-target-replace-all = ersättning i hela projektet
undo-target-import = dokumentimporten
undo-target-duplicate = duplicering
undo-target-move = flytt
undo-target-merge = sammanslagning av två scener
undo-target-split = delning av en scen
undo-target-promote = typbyte
undo-target-tidy-titles = städning av kapitelrubriker
undo-target-import-tags = import av taggar
undo-target-import-templates = import av anteckningsmallar
undo-target-create = skapande
undo-target-remove = borttagning
undo-target-rename = namnbyte
undo-target-edit = den ändringen
menu-edit-find = &Sök…
menu-edit-find-next = Sök &nästa
menu-edit-find-prev = Sök &föregående
menu-edit-replace = Sök och &ersätt…
menu-edit-find-in-project = Sök i &projektet…
menu-edit-replace-in-project = Ersätt i p&rojektet…
undo-frozen = Ångra (”Alltid framåt” är på)
redo-frozen = Gör om (”Alltid framåt” är på)
menu-edit-cut = Klipp &ut
menu-edit-copy = &Kopiera
menu-edit-paste = Klistra &in
menu-edit-paste-plain = Klistra in u&tan formatering
menu-edit-select-all = &Markera allt
shortcut-name-edit-cut = Klipp ut
shortcut-name-edit-copy = Kopiera
shortcut-name-edit-paste = Klistra in
shortcut-name-edit-paste-plain = Klistra in utan formatering
shortcut-name-edit-select-all = Markera allt
titlebar-spellcheck-on = Stavningskontrollen är på. Klicka för att sluta kontrollera (F7)
titlebar-spellcheck-off = Stavningskontrollen är av. Klicka för att kontrollera igen (F7)
menu-spellcheck = &Kontrollera stavning
menu-comments = Ko&mmentarer
settings-page-spellcheck = Stavningskontroll
settings-group-spellcheck = Stavningskontroll
settings-spellcheck-enabled = Kontrollera stavningen medan jag skriver
settings-spellcheck-hint = Stryker under ord som ingen installerad ordlista känner till. Om du stänger av detta upphör all kontroll, i alla projekt, tills du slår på den igen. Om bara ett språk ska sluta kontrolleras avmarkerar du det i fältet Språk för projektet eller för ett objekt.
menu-trash = &Papperskorg
trash-title = Papperskorg
trash-empty-state = Papperskorgen är tom.
trash-empty-button = Töm papperskorgen…
trash-restore = Å&terställ
trash-restore-to = Återställ &till…
trash-delete-forever = Ta &bort permanent
trash-restored-ok = Återställda: { $count }.
trash-restore-error = Det gick inte att återställa: { $error }
trash-restore-orphaned = Objektets ursprungliga plats finns inte längre. Välj var det ska återställas.
trash-restore-no-project = Inget projekt är öppet, så det finns inget att återställa.
trash-delete-no-project = Inget projekt är öppet, så det finns inget att ta bort.
trash-empty-confirm-title = Tömma papperskorgen?
trash-empty-confirm-text = { $count ->
       [one] { $count } post i papperskorgen tas bort permanent.
      *[other] { $count } poster i papperskorgen tas bort permanent.
   } De lämnar papperskorgen för gott. Ångra kan fortfarande hämta tillbaka dem tills du gör något annat.
trash-emptied-title = Papperskorgen har tömts
trash-emptied-body = Allt i papperskorgen har tagits bort permanent.
trash-empty-no-project = Inget projekt är öppet, så det finns ingen papperskorg att tömma.
trash-delete-forever-confirm-title = Ta bort permanent?
trash-delete-forever-confirm-text = { $count ->
       [one] { $count } objekt tas bort permanent.
      *[other] { $count } objekt tas bort permanent.
   } De lämnar papperskorgen för gott. Ångra kan fortfarande hämta tillbaka dem tills du gör något annat.
trash-deleted-title = Permanent borttaget
trash-deleted-body = Permanent borttagna: { $count }.
trash-undo = Ångra
trash-restore-picker-title = Återställ till…
trash-restore-to-confirm-title = Återställa här?
trash-restore-to-confirm-text = Återställa ”{ $item }” till ”{ $destination }”?
trash-restore-picker-restore-here = Återställ här
trash-restore-picker-cancel = Avbryt
trash-restore-picker-empty = Inga pärmar ännu. Skapa en först.
trash-banner-title = Det här objektet ligger i papperskorgen
trash-banner-description = Det visas inte i dispositionen eller i exporter förrän du återställer det.
trash-banner-restore = Återställ…
trash-tab-tooltip = I papperskorgen
settings-styles-sheet-yes = Ja
settings-styles-sheet-no = Nej
settings-styles-sheet-font = Teckensnitt
settings-styles-sheet-indent = Indrag på första raden
settings-styles-sheet-para-spacing = Styckeavstånd
settings-styles-sheet-page = Sidstorlek
settings-styles-sheet-margins = Marginaler (Ö/H/N/V)
settings-styles-sheet-title-page = Bokens titelsida
settings-styles-sheet-heading-language = Rubrikernas språk
settings-styles-sheet-auto = Följer texten
settings-styles-sheet-digits = Siffror
settings-styles-sheet-direction = Textriktning
settings-styles-sheet-formats = Format
settings-styles-sheet-all-formats = Alla
settings-styles-editor-group = Stilredigerare
settings-styles-editor-missing = Den stilen finns inte längre.
settings-styles-page-letter = Letter
settings-styles-digits-western = Västerländska (0–9)
settings-styles-digits-eastern-arabic = Östarabiska (٠–٩)
settings-styles-direction-ltr = Vänster till höger
settings-styles-direction-rtl = Höger till vänster
format-dock-title = Format
format-panel-empty = Placera markören i en scen, en anteckning eller en synopsis för att se formateringsalternativen.
format-group-marks = Text
format-group-block = Stycke
format-group-lists = Listor
format-group-tables = Tabell
format-group-breaks = Scenbrytningar
format-superscript = Upphöjd
format-subscript = Nedsänkt
format-link = Länk…
link-dialog-insert-title = Infoga länk
link-dialog-edit-title = Redigera länk
link-dialog-text-label = Text
link-dialog-text-placeholder = orden som läsaren ser
link-dialog-url-label = Länkar till
link-dialog-url-placeholder = example.com
link-dialog-insert = Infoga
link-dialog-apply = Verkställ
link-dialog-cancel = Avbryt
link-dialog-remove = Ta bort länk
link-scheme-refused = Bara webb- och e-postlänkar kan öppnas. { $url } lämnades orörd.
menu-format-link = L&änk…
format-clear = Rensa formatering
format-heading = Rubriknivå
format-heading-normal = Normal text
format-heading-1 = Rubrik 1
format-heading-2 = Rubrik 2
format-heading-3 = Rubrik 3
format-heading-4 = Rubrik 4
format-heading-5 = Rubrik 5
format-heading-6 = Rubrik 6
format-align-left = Vänsterjustera
format-align-center = Centrera
format-direction-rtl = Stycke från höger till vänster
format-blockquote = Blockcitat
format-list-bullet = Punktlista
format-list-numbered = Numrerad lista
format-indent = Öka indrag
format-outdent = Minska indrag
format-table-insert = Infoga tabell
format-table-row-above = Infoga rad ovanför
format-table-row-below = Infoga rad nedanför
format-table-col-before = Infoga kolumn före
format-table-col-after = Infoga kolumn efter
format-table-row-delete = Ta bort rad
format-table-col-delete = Ta bort kolumn
format-table-remove = Ta bort tabellen
menu-format-marks-bold = &Fet
menu-format-marks-italic = &Kursiv
menu-format-marks-underline = &Understruken
menu-format-marks-strike = &Genomstruken
menu-format-marks-superscript = Upp&höjd
menu-format-marks-subscript = &Nedsänkt
menu-format-marks-clear = Rensa f&ormatering
menu-format-heading = Rubr&ik
menu-format-heading-normal = &Normal text
menu-format-heading-1 = Rubrik &1
menu-format-heading-2 = Rubrik &2
menu-format-heading-3 = Rubrik &3
menu-format-heading-4 = Rubrik &4
menu-format-heading-5 = Rubrik &5
menu-format-heading-6 = Rubrik &6
menu-format-alignment = &Justering
menu-format-align-left = &Vänsterjustera
menu-format-align-center = &Centrera
menu-format-direction = &Riktning
menu-format-direction-auto = &Automatisk
menu-format-direction-ltr = &Vänster till höger
menu-format-direction-rtl = &Höger till vänster
menu-format-blockquote = &Blockcitat
menu-format-lists = &Listor
menu-format-list-bullet = &Punktlista
menu-format-list-numbered = &Numrerad lista
menu-format-indent = &Öka indrag
menu-format-outdent = &Minska indrag
menu-format-table = &Tabell
menu-format-table-insert = &Infoga tabell
menu-format-table-2x2 = &2 x 2
menu-format-table-3x3 = &3 x 3
menu-format-table-4x4 = &4 x 4
menu-format-table-row-above = Infoga rad &ovanför
menu-format-table-row-below = Infoga rad &nedanför
menu-format-table-col-before = Infoga kolumn &före
menu-format-table-col-after = Infoga kolumn &efter
menu-format-table-row-delete = &Ta bort rad
menu-format-table-col-delete = Ta bort &kolumn
menu-format-table-remove = Ta bort ta&bellen
menu-format-typo-marks = Formateringst&ecken
menu-format-typo-nbsp = &Hårt mellanslag
menu-format-typo-nbhyphen = Hårt &bindestreck
menu-format-typo-soft-hyphen = &Mjukt bindestreck
menu-format-typo-nnbsp = &Smalt hårt mellanslag
menu-format-typo-zwsp = &Nollbreddsmellanslag
menu-format-typo-word-joiner = &Ordsammanfogare
menu-about = &Om Skribisto…
about-title = Om Skribisto
about-version = Version { $version }
about-update-available = Version { $version } finns tillgänglig
about-update-see-notes = Se vad som har ändrats
about-update-flathub = Flathub håller den här kopian uppdaterad.
about-update-distro = Din distribution håller den här kopian uppdaterad.
about-tagline = Ett skrivprogram för romaner och längre fiktion, skrivet i Rust med verktygslådan Teksilo.
about-license = Utgiven under GNU General Public License, version 3.
about-copyright = © 2026 Cyril Jacquet
about-close = Stäng
native-menu-about = Om { $app }
native-menu-hide = Göm { $app }
native-menu-quit = Avsluta { $app }
native-menu-settings = Inställningar…
native-menu-window = Fönster
native-menu-minimize = Minimera
native-menu-zoom = Zooma
window-title = { $title } - { $app }
window-title-numbered = { $title } - { $app } (fönster { $n })
window-title-empty = { $app }
first-run-window-title = Välkommen
first-run-title = Konfigurera { $app }
first-run-body = { $app } sparar sina inställningar separat från Skribisto och börjar därför tomt. Dina inställningar, senaste projekt, ordlistor och fönsterlayout kan kopieras över nu.
first-run-from = Kopiera från
first-run-to = Kopiera till
first-run-copy-note = Inget flyttas eller tas bort. Skribisto behåller alla sina egna inställningar och fortsätter att fungera precis som tidigare.
first-run-import = Importera inställningar
first-run-start-fresh = Börja från början
first-run-import-failed = Vissa inställningar kunde inte importeras: { $error }
settings-page-punctuation = Standard för interpunktion
settings-group-punctuation = Smart interpunktion
settings-page-work-punctuation = Interpunktion
settings-punctuation-override = Ge projektet egna regler för interpunktion
settings-punctuation-override-hint = Av: projektet följer programmets inställning. Reglerna följer med i .skrib-filen, så en medförfattare som öppnar filen skriver med samma typografi.
settings-punctuation-dashes = Gör -- till ett kort tankstreck och --- till ett långt tankstreck
settings-punctuation-ellipsis = Gör ... till en ellips
settings-punctuation-quotes = Typografiska citattecken och apostrofer
settings-quote-style = Citattecken
settings-quote-style-locale = Språkets standard
settings-quote-style-curly = “Dubbla”
settings-quote-style-curly-single = ‘Enkla’
settings-quote-style-guillemets = «Gåsögon»
settings-quote-style-low-high = „Låga-höga“
settings-punctuation-spacing = Mellanslag före ; : ! ?
settings-punctuation-spacing-hint = Fransk typografi sätter ett smalt hårt mellanslag före ; ! ? och ett vanligt hårt mellanslag före :. Det gäller bara text skriven på franska. Mellanrummet innanför gåsögon « » följer med själva gåsögonen.
settings-punctuation-sample = Ditt språk ger
settings-punctuation-app-hint = Det som alla projekt gör om de inte har egna regler under Projekt ▸ Interpunktion.
settings-punctuation-dialogue = Inled ett stycke som skrivs som ”- ” med ett repliksstreck
settings-punctuation-dialogue-hint = För språk som markerar repliker med ett streck i stället för citattecken: franska, spanska, ryska med flera. Det utlöses bara allra först i ett stycke.
statusbar-go-to = Gå till…
go-to-placeholder = Sök i pärmen
go-to-no-matches = Inget objekt matchar sökningen.
menu-go-to = &Gå till…
settings-page-distraction-free-themes = Teman för distraktionsfritt läge
settings-themes-builtin = Inbyggda teman
settings-themes-builtin-badge = Inbyggt
settings-themes-user = Mina teman
settings-themes-editor-group = Redigera tema
settings-themes-editor-empty = Välj ett tema under ”Mina teman” för att redigera det.
settings-themes-use = Använd
settings-themes-duplicate = Duplicera
settings-themes-edit = Redigera
settings-themes-delete = Ta bort
settings-themes-export = Exportera…
settings-themes-import = Importera…
settings-themes-copy-suffix = kopia
settings-themes-json-filter = Tema (JSON)
settings-themes-imported = Temat har importerats
settings-themes-import-failed = Det gick inte att importera temat
settings-themes-exported = Temat har exporterats
settings-themes-export-failed = Det gick inte att exportera temat
settings-themes-low-contrast = låg kontrast
settings-themes-low-contrast-band = markeringen döljer texten
settings-themes-field-name = Namn
settings-themes-field-paper = Sida
settings-themes-field-ink = Text
settings-themes-field-general = Bakgrund
settings-themes-field-widget-text = Kontrollremsans text
settings-themes-field-caret-band = Markering kring markören
statusbar-focus-settings = Inställningar för distraktionsfritt läge
statusbar-focus-manage-themes = Hantera teman…
comments-title = Kommentarer
comments-document-title = Det här dokumentet
comments-empty-project = Inga kommentarer i det här projektet ännu.
comments-empty-document = Inga kommentarer i det här dokumentet.
comments-filter-all = Alla
comments-filter-open = Öppna
comments-filter-resolved = Lösta
comments-filter-orphaned = Övergivna
comments-status-open = Öppen
comments-status-resolved = Löst
comments-status-orphaned = Har förlorat sin text
comments-orphan-snippet = (den kommenterade texten finns inte längre)
comments-status-unplaced = Ingen textposition
comments-unplaced-snippet = (inte förankrad i någon text)
comments-reply-count = { $count ->
       [one] { $count } svar
      *[other] { $count } svar
   }
comments-menu-resolve = Markera som löst
comments-menu-reopen = Öppna igen
comments-menu-delete = Ta bort
comments-sort-document = I dokumentordning
comments-sort-newest = Nyaste först
comments-sort-reader = Grupperade efter läsare
comments-unsigned-author = Osignerad
comments-agreement = { $count ->
       [one] { $count } läsare
      *[other] { $count } läsare
   }
comments-menu-add = Lägg till kommentar
comments-menu-add-paragraph = Kommentera det här stycket
overview-col-comments = Kommentarer
overview-col-total-comments = Kommentarer totalt
comments-card-placeholder = Skriv en kommentar…
comments-card-unknown-author = Okänd författare
comments-unsigned-toast = Dina kommentarer är osignerade: inget namn är angivet för dig på den här datorn.
comments-unsigned-action = Ange ditt namn
comments-card-reply = Svara
comments-card-reply-placeholder = Svara…
comments-card-actions = Åtgärder för kommentaren
comments-reply-actions = Åtgärder för svaret
comments-menu-delete-reply = Ta bort svaret
comments-menu-delete-all = Ta bort alla kommentarer här
comments-deleted-toast = Kommentaren har tagits bort
comments-reply-deleted-toast = Svaret har tagits bort
comments-deleted-all-toast = { $count ->
       [one] { $count } kommentar borttagen
      *[other] { $count } kommentarer borttagna
   }
comments-undo = Ångra
analysis-segment = Analys
analysis-scope-book = Analyserar den här boken
analysis-run = Kör analys
analysis-stale = Ändrat sedan analysen kördes
analysis-not-run = Inte analyserat ännu.
analysis-running = Läser manuset…
analysis-failed = Analysen kunde inte slutföras.
analysis-no-scenes = Inga scener i den här boken ännu.
analysis-shape = Form
analysis-arrivals = Tillkomst
analysis-arrivals-explainer = Hur text har kommit in i projektet medan det har varit öppet. Det visar vilken väg tecknen kom, och ingenting alls om vem som skrev dem: en författare som skriver utkast någon annanstans och klistrar in har klistrat in, och en som dikterar har dikterat. Det finns inget mål att sträva mot här.
analysis-arrivals-scope = Hela projektet, inte bara den här boken.
analysis-arrivals-session = Sedan du öppnade det. När projektet stängs börjar räkningen om.
analysis-arrivals-nothing = Ingen text har kommit in ännu under den här sessionen.
analysis-arrivals-typed = Inskrivet
analysis-arrivals-pasted = Inklistrat
analysis-arrivals-dictated = Dikterat
analysis-arrivals-imported = Importerat
analysis-arrivals-programmatic = Infogat åt dig
analysis-arrivals-count = { $count ->
       [one] { $count } tecken
      *[other] { $count } tecken
   }
analysis-arrivals-none = inget
analysis-words-per-scene = Ord per scen
analysis-median-words = Bokens medianscen är { $count ->
       [one] { $count } ord lång.
      *[other] { $count } ord lång.
   }
analysis-median-line = Median: { $count ->
       [one] { $count } ord
      *[other] { $count } ord
   }
analysis-dialogue = Dialog
analysis-dialogue-unsupported = Dialog mäts inte för det här språket ännu.
analysis-footnote-words = Ord i fotnoter
analysis-footnote-words-count = { $count ->
       [one] { $count } ord finns i bokens fotnoter och hålls utanför manusets totalsumma.
      *[other] { $count } ord finns i bokens fotnoter och hålls utanför manusets totalsumma.
   }
analysis-footnote-words-pending = Räknar fotnoterna…
analysis-ignore-empty = Hoppa över texter där inget har skrivits ännu
analysis-empty-hidden = { $count } { $count ->
           [one] tom text dold
          *[other] tomma texter dolda
       }.
analysis-all-texts-empty = Alla texter i den här boken är fortfarande tomma.
binder-filter-count = { $shown } av { $total } visas
binder-filter-clear = Rensa
binder-filter-none = Inget matchar ”{ $query }”.
settings-paratext-intro = De förtexter och eftertexter som ett nytt projekt kan börja med. Varje struktur hör till en förlagstradition, och dess sidtitlar är skrivna på den traditionens eget språk. Byt gärna namn på dem när projektet har skapats.
settings-paratext-structures = Strukturer
settings-paratext-broken = Kunde inte läsas
settings-paratext-edit = Redigera
settings-paratext-duplicate = Duplicera
settings-paratext-delete = Ta bort
settings-paratext-new = Ny struktur
settings-paratext-save = Spara
settings-paratext-editor-hint = Ett namn, sidorna som skapas före manuset och sidorna som skapas efter det. Var de hamnar bestämmer du: de är vanliga objekt när projektet väl finns.
image-insert = Infoga &bild…
image-no-project = Öppna ett projekt innan du infogar en bild.
image-choose-title = Välj en bild
image-filter-label = Bilder
image-large-title = Det här är en stor bild
image-large-text =
       { $name } är på { $megapixels } megapixlar ({ $width }×{ $height }).
       Om du behåller den som den är sparas originalfilen i projektet: den följer
       med i varje säkerhetskopia och varje export. Optimera sparar i stället en
       mindre kopia, högst 2560 pixlar på den längsta sidan.
image-large-keep = Behåll originalet
image-large-downscale = Optimera
image-large-remember = Gör så här framöver och fråga inte igen
image-not-recorded = Bilden sparades men kunde inte registreras i projektet.
image-describe-title = Beskriv bilden
image-describe-explain = Vad bilden visar, för en läsare som inte kan se den. Beskrivningen är inte en del av manuset: den räknas, söks eller exporteras aldrig som prosa.
image-describe-placeholder = en fyr mot en grå himmel
image-resize-title = Ändra storlek på bilden
image-resize-explain = En procentsats av storleken den visas i nu. 100 lämnar den som den är.
image-resize-invalid = Ange ett tal mellan 1 och 1000.
image-menu-describe = &Beskriv bilden…
image-menu-resize = Än&dra storlek på bilden…
image-menu-reset-size = &Originalstorlek
cover-choose = &Bokomslag…
cover-clear = Ta bort omsla&get
cover-choose-title = Välj ett omslag
cover-set = Omslaget är valt.
cover-cleared = Omslaget har tagits bort. Bilden finns kvar i projektet.
card-label = Etikett
card-exportable = Exporteras
card-numbered = Numrerat
card-created = Skapat
card-modified = Ändrat
card-yes = ja
card-no = nej
card-type = Typ
card-position = Position
card-children = Underobjekt
card-goal = Mål
card-words = Ord
card-synopsis = Synopsis
card-point-of-view = Berättarperspektiv
card-aliases = Även känd som
export-orphan-footnotes-title = Fotnoter utan referens
export-comments-dropped =
       { $count ->
           [one] En kommentar kunde inte placeras i den exporterade texten och utelämnades.
          *[other] { $count } kommentarer kunde inte placeras i den exporterade texten och utelämnades.
       }
export-orphan-footnotes =
       { $count ->
           [one] En fotnot refereras inte längre någonstans i manuset. Dess text kommer inte med i den exporterade boken.
          *[other] { $count } fotnoter refereras inte längre någonstans i manuset. Deras text kommer inte med i den exporterade boken.
       }
menu-footnotes = Fot&noter
footnotes-title = Fotnoter
footnotes-insert = Infoga &fotnot
footnotes-empty = Inga fotnoter ännu. Placera markören i en scen och använd sedan + ovan, eller Ctrl+Alt+F.
footnotes-filter-all = Alla
footnotes-filter-document = Det här dokumentet
footnotes-filter-orphaned = Övergivna
footnotes-orphaned = Inget pekar längre på den här fotnoten
footnotes-untitled-home = Namnlös
footnotes-body-placeholder = själva fotnoten
footnotes-insert-tooltip = Infoga en fotnot vid markören (Ctrl+Alt+F)
footnotes-actions = Åtgärder för fotnoten
footnotes-delete = &Ta bort fotnoten och dess referens
footnotes-no-project = Öppna ett projekt innan du infogar en fotnot.
footnotes-no-caret = Placera markören i en scens text för att infoga en fotnot där.
footnotes-not-created = Fotnoten kunde inte läggas till i projektet.
footnotes-deleted-toast = Fotnoten har tagits bort
footnotes-undo-delete = Ångra
import-document-title = Importera dokument
import-document-close = Stäng
import-document-step-files = Filer
import-document-step-review = Granska
import-document-step-destination = Destination
import-document-step-reconcile = Sammanfoga
import-document-reconcile-hint = Några av objekten finns redan hos dig. Ange vad som ska hända med vart och ett.
import-document-reconcile-all-new = Inget i filen matchar ditt projekt: alla objekt läggs till som nya.
import-document-duplicate-returns = Filerna är flera kopior av samma manus som kommer tillbaka. Importera dem en i taget: matchningen ställer bara en returnerad kopia mot ditt projekt, så om flera importeras tillsammans läggs en andra kopia av boken till i stället för att sammanfogas.
import-document-hunk-added = Ta med det här nya stycket
import-document-hunk-removed = Ta bort det här stycket
import-document-hunk-changed = Ta med den här omskrivningen
import-document-col-stray-prose = Dess text
import-document-stray-as-paratext = Behåll som paratext
import-document-stray-discard = Släng texten
import-document-col-current = I ditt projekt
import-document-col-incoming = I den här filen
import-document-col-status = Status
import-document-col-action = Åtgärd
import-document-status-identical = Samma
import-document-status-editor-edited = De har redigerat den
import-document-status-you-edited = Du har redigerat den
import-document-status-conflict = Båda har redigerat den
import-document-status-different = Skiljer sig
import-document-status-new = Ny
import-document-status-missing = Inte i den här filen
import-document-status-moved = Flyttad
import-document-action-comments-only = Bara kommentarer
import-document-action-take-import = Ta den här versionen
import-document-action-keep-current = Behåll min
import-document-action-create-new = Lägg till som ny
import-document-action-ignore = Hoppa över
import-document-compare = Jämför
import-document-compare-legend = Din version jämförd med filens. Skrivskyddad.
import-document-compare-close = Stäng
import-document-drop-title = Släpp dokument här
import-document-drop-hint = Markdown (.md), Word (.docx), OpenDocument (.odt) och oformaterad text (.txt)
import-document-browse = Bläddra…
import-document-move-up = Flytta upp
import-document-move-down = Flytta ned
import-document-remove-file = Ta bort
import-document-file-count = { $count ->
       [one] 1 fil
      *[other] { $count } filer
   }
import-document-no-files = Inga filer har valts ännu.
import-document-col-included = Importera
import-document-col-title = Titel
import-document-col-type = Typ
import-document-col-words = Ord
import-document-col-breaks = Brytningar
import-document-col-comments = Kommentarer
import-document-col-footnotes = Fotnoter
import-document-col-epigraph = Motto
import-document-col-source = Källa
import-document-level-rules = Rubriknivåer
import-document-level-n = Rubrik { $level }
import-document-add-top-level = Lägg till översta nivå
import-document-add-top-level-tooltip = Infoga en bok ovanför alla analyserade objekt, för kapitelfiler som saknar bokrubrik
import-document-destination = Destination
import-document-destination-hint = Välj en pärm eller ett objekt: nya objekt hamnar inuti en mapp eller efter en scen.
import-document-destination-empty = Inga pärmar ännu. Skapa en först.
import-document-plan-empty = Inget att importera ännu.
import-document-summary = { $rows ->
       [one] 1 objekt
      *[other] { $rows } objekt
   } · { $breaks ->
       [one] 1 scenbrytning
      *[other] { $breaks } scenbrytningar
   }
import-document-back = Tillbaka
import-document-cancel = Avbryt
import-document-analyse = Nästa
import-document-analysing = Läser dokument…
import-document-step-analysing = Läser
import-document-cancel-analysis = Sluta läsa
import-document-analyse-failed = Dokumenten kunde inte läsas.
import-document-details = Detaljer
import-document-import = Importera
import-document-done = { $count ->
       [one] 1 objekt importerat
      *[other] { $count } objekt importerade
   }
import-document-undo = Ångra
import-diagnostic-file-unreadable = ”{ $path }” kunde inte läsas: { $detail }. De andra filerna importeras ändå.
import-diagnostic-nested-too-deep = ”{ $path }” lästes inte: innehållet är kapslat i mer än { $limit } nivåer. Inget verkligt dokument kommer i närheten av det, och att läsa det kunde få Skribisto att krascha, så det utelämnas. De andra filerna importeras ändå.
import-diagnostic-archive-too-large = ”{ $path }” lästes inte: det är en komprimerad fil som är byggd för att packas upp till något mycket större än den själv, vilket kunde ta slut på minnet och få Skribisto att krascha, så den utelämnas. De andra filerna importeras ändå.
import-diagnostic-lossy-decode = { $count ->
       [one] Ett tecken i ”{ $path }” kunde inte avkodas. Spara om filen som UTF-8 för att behålla det.
      *[other] { $count } tecken i ”{ $path }” kunde inte avkodas. Spara om filen som UTF-8 för att behålla dem.
   }
import-diagnostic-decoded-from-bom = ”{ $path }” avkodades som { $detail }, inte UTF-8.
import-diagnostic-empty-file = ”{ $path }” är tom.
import-diagnostic-no-headings = ”{ $path }” har inga rubriker och kommer därför in som ett enda objekt.
import-diagnostic-unsupported-format = Inget kan läsa ”.{ $detail }”-filer, så ”{ $path }” hoppades över.
import-diagnostic-front-matter-not-flat = Front matter i ”{ $path }”: ”{ $detail }” är inte ett enkelt värde och hoppades över.
import-diagnostic-footnotes-degraded = { $count ->
       [one] En fotnot i ”{ $path }” kommer in som vanlig text: fotnoter läses inte från Markdown.
      *[other] { $count } fotnoter i ”{ $path }” kommer in som vanlig text: fotnoter läses inte från Markdown.
   }
import-diagnostic-footnote-not-carried = { $count ->
       [one] En fotnot i ”{ $path }” kunde inte föras över. De andra fördes över.
      *[other] { $count } fotnoter i ”{ $path }” kunde inte föras över. De andra fördes över.
   }
import-diagnostic-raw-html-dropped = { $count ->
       [one] Ett block med rå HTML i ”{ $path }” utelämnades.
      *[other] { $count } block med rå HTML i ”{ $path }” utelämnades.
   }
import-diagnostic-nested-break-dropped = { $count ->
       [one] En scenbrytning inuti ett citat eller en lista i ”{ $path }” utelämnades. Bara en brytning på en egen rad behålls.
      *[other] { $count } scenbrytningar inuti citat eller listor i ”{ $path }” utelämnades. Bara en brytning på en egen rad behålls.
   }
import-diagnostic-duplicate-title = ”{ $title }” förekommer { $count } gånger. Om du har importerat de här filerna tidigare kommer de att dupliceras.
import-diagnostic-heading-level-jump = ”{ $title }” hoppar från rubriknivå { $from } till { $to }; den placeras en nivå under sitt överordnade objekt.
import-diagnostic-illegal-combination = ”{ $title }” innehåller prosa, men typen { $kind } kan inte innehålla någon. Importen väntar tills du ändrar dess typ eller utesluter den. Annars importeras ingenting alls.
import-diagnostic-tracked-changes-flattened = { $path } var mitt i en granskning: { $count ->
       [one] { $count } spårad ändring godtogs
      *[other] { $count } spårade ändringar godtogs
   } och borttagningar genomfördes. Det är den slutliga texten. Om du vill behålla något av dina egna formuleringar i stället använder du Jämför i det sista steget.
import-diagnostic-tracked-changes-flattened-by = { $path } var mitt i en granskning: { $count ->
       [one] { $count } spårad ändring av { $names } godtogs
      *[other] { $count } spårade ändringar av { $names } godtogs
   } och borttagningar genomfördes. Det är den slutliga texten. Om du vill behålla något av dina egna formuleringar i stället använder du Jämför i det sista steget.
import-diagnostic-text-box-dropped = { $count ->
       [one] { $path } innehåller { $count } textruta. Dess text står utanför dokumentets flöde, så det går inte att avgöra var den hör hemma i ett manus. Den importeras inte.
      *[other] { $path } innehåller { $count } textrutor. Deras text står utanför dokumentets flöde, så det går inte att avgöra var den hör hemma i ett manus. De importeras inte.
   }
import-diagnostic-embedded-object-dropped = { $count ->
       [one] { $path } innehåller { $count } inbäddat objekt: ett diagram, en formel eller liknande. Inget i ett manus kan rymma det.
      *[other] { $path } innehåller { $count } inbäddade objekt: diagram, formler eller liknande. Inget i ett manus kan rymma dem.
   }
import-diagnostic-field-flattened = { $count ->
       [one] { $path } innehåller { $count } fält: ett sidnummer, en korsreferens, ett datum. Det behåller texten det senast visade och uppdateras inte längre.
      *[other] { $path } innehåller { $count } fält: sidnummer, korsreferenser, datum. Vart och ett behåller texten det senast visade och uppdateras inte längre.
   }
import-diagnostic-unknown-style-level = { $path } använder formatmallen ”{ $detail }”, som ser ut som en rubrik men inte anger någon nivå. De styckena importeras som prosa i stället för att tilldelas en gissad nivå.
import-diagnostic-comment-unanchored = Kommentaren ”{ $detail }” i { $path } kunde inte fästas vid orden den gällde. Den behålls på sitt objekt, där du kan flytta den.
import-diagnostic-comment-not-carried = Kommentaren ”{ $detail }” i { $path } importeras inte, eftersom inget i filen kommer in som text som den kunde fästas vid. Kopiera den från filen om du behöver den.
import-diagnostic-comment-replies-flattened = { $count ->
       [one] { $count } svar i { $path } hänvisade till en kommentar som inte finns i filen och kommer därför in som en egen kommentar.
      *[other] { $count } svar i { $path } hänvisade till en kommentar som inte finns i filen och kommer därför in som egna kommentarer.
   }
import-diagnostic-prose-not-verbatim = { $count ->
       [one] { $count } stycke i { $path } kunde inte föras över med formateringen exakt som den var och kommer därför in som vanlig text. Jämför det med originalet.
      *[other] { $count } stycken i { $path } kunde inte föras över med formateringen exakt som den var och kommer därför in som vanlig text. Jämför dem med originalet.
   }
import-diagnostic-styled-spaces-not-carried = { $count ->
       [one] I { $path } var { $count } tomt utrymme understruket eller genomstruket, till exempel en rad att fylla i. Skribisto behåller ingen av linjerna på enbart mellanslag, så det kommer in som vanliga mellanslag, eller inte alls där det inleder eller avslutar ett stycke.
      *[other] I { $path } var { $count } tomma utrymmen understrukna eller genomstrukna, till exempel rader att fylla i. Skribisto behåller ingen av linjerna på enbart mellanslag, så de kommer in som vanliga mellanslag, eller inte alls där de inleder eller avslutar ett stycke.
   }
import-diagnostic-list-nesting-flattened = { $count ->
       [one] { $count } listpunkt i { $path } var kapslad i mer än { $limit } nivåer. Den kommer in på nivå { $limit }, den djupaste som Skribisto behåller, med sina ord och sin formatering.
      *[other] { $count } listpunkter i { $path } var kapslade i mer än { $limit } nivåer. De kommer in på nivå { $limit }, den djupaste som Skribisto behåller, med sina ord och sin formatering.
   }
import-diagnostic-table-read-as-paragraphs = { $count ->
       [one] { $count } tabell i { $path } har en rad som är mycket bredare än de andra, för ojämn för att ställas upp som ett rutnät. Dess celler kommer in som stycken, ett efter ett, med sina ord och sin formatering.
      *[other] { $count } tabeller i { $path } har en rad som är mycket bredare än de andra, för ojämna för att ställas upp som ett rutnät. Deras celler kommer in som stycken, ett efter ett, med sina ord och sin formatering.
   }
import-diagnostic-spaces-shortened = { $count ->
       [one] { $count } följd av mellanslag i { $path } var längre än { $limit } mellanslag och kommer in förkortad till { $limit }. Orden runt omkring finns kvar.
      *[other] { $count } följder av mellanslag i { $path } var längre än { $limit } mellanslag och kommer in förkortade till { $limit }. Orden runt omkring finns kvar.
   }
import-diagnostic-epigraph-not-carried = ”{ $title }” inleds med ett motto, men typen { $kind } kan inte ha något. Citatet behålls i stället överst i texten.
import-diagnostic-epigraph-placement-ambiguous = Ett motto står mellan ”{ $title }” och ”{ $below }” och kunde inleda vilket som helst av dem. Det gavs till ”{ $title }”, där ett motto brukar stå.
import-document-diagnostics = { $errors ->
       [0] { $warnings ->
               [one] 1 sak att känna till
              *[other] { $warnings } saker att känna till
           }
      *[other] { $errors ->
               [one] 1 fil kunde inte läsas
              *[other] { $errors } filer kunde inte läsas
           }
   }
import-document-diagnostics-none = Inget att rapportera.
versions-title = Versioner
versions-scope-synopsis = Synopsis
versions-scope-prose = Text
versions-loading = Går igenom dina säkerhetskopior…
versions-empty = Ingen tidigare version av detta ännu
versions-error = Det gick inte att läsa dina säkerhetskopior. Inget har gått förlorat, men listan kan vara ofullständig
versions-did-not-exist = Fanns ännu inte den { $date }
versions-deleted-after = Borttaget någon gång efter { $date }
versions-unreadable = { $count ->
       [one] { $count } säkerhetskopia kunde inte läsas
      *[other] { $count } säkerhetskopior kunde inte läsas
   }
versions-thinned = { $count ->
       [one] Äldre versioner glesas ut med tiden. Projektets egen historik har redan tagit bort { $count } tidigare tillstånd av den här texten.
      *[other] Äldre versioner glesas ut med tiden. Projektets egen historik har redan tagit bort { $count } tidigare tillstånd av den här texten.
   }
versions-source-backup = Från en säkerhetskopia
versions-source-project = Från projektets egen historik
versions-list-caption = En post per ändring, inte per säkerhetskopia
versions-pick-a-version = Välj en version för att se vad som ändrades
versions-earliest = Den tidigaste registrerade versionen. Det finns inget äldre att jämföra med.
versions-no-change = Inget ändrades i den här delen av objektet
versions-formatting-only = Bara formateringen ändrades här. Orden är desamma
versions-show-unchanged = Visa oförändrat
versions-hide-unchanged = Dölj oförändrat
versions-next-change = Nästa ändring
versions-near = nära ”{ $text }”
versions-words-added = { $count ->
       [one] { $count } ord tillagt
      *[other] { $count } ord tillagda
   }
versions-words-removed = { $count ->
       [one] { $count } ord borttaget
      *[other] { $count } ord borttagna
   }
versions-blocks-moved = { $count ->
       [one] { $count } stycke flyttat
      *[other] { $count } stycken flyttade
   }
versions-pin = Fäst säkerhetskopian som den här versionen kommer från. Den automatiska rensningen tar aldrig bort den
versions-unpin = Lossa säkerhetskopian som den här versionen kommer från. Den automatiska rensningen kan ta bort den igen
versions-pinned-only = Visa bara fästa versioner
versions-pin-note = Bara versioner från en säkerhetskopia kan fästas. Att fästa behåller en fil, och projektets egen historik är ingen fil.
versions-pinned-empty = Inget här är fäst ännu
versions-pinned-empty-log = Inget här är fäst ännu. Bara versioner från en säkerhetskopia kan fästas. Att fästa behåller en fil, och projektets egen historik är ingen fil.
versions-range-filter = Visa bara versioner mellan två datum
versions-filtered-empty = Ingen version matchar filtren du har angett
versions-clear-filters = Rensa filtren
versions-last-30-days = Senaste 30 dagarna
versions-restore-button = Återställ den här versionen
versions-restore-confirm-title = Ersätta den här texten med versionen från { $date }?
versions-restore-confirm-text = Det du har nu ersätts med texten som objektet hade den { $date }.
versions-restore-confirm-with-comments = Det du har nu ersätts med texten som objektet hade den { $date }. { $count ->
       [one] { $count } kommentar är förankrad i den nuvarande texten och kan bli övergiven.
      *[other] { $count } kommentarer är förankrade i den nuvarande texten och kan bli övergivna.
   }
versions-restore-confirm-undo-note = En säkerhetskopia tas först, och Ctrl+Z ångrar detta i ett steg.
versions-restored-toast = Versionen från { $date } har återställts
versions-undo = Ångra
versions-restore-row-gone = Objektet finns inte längre i projektet
versions-restore-no-home = Objektet har bytt typ sedan dess, och den gamla texten har ingen plats i det
versions-restore-no-safety-copy = Den extra säkerhetskopian kördes inte, så inget ändrades
versions-restore-failed = Återställningen misslyckades: { $error }
versions-restore-backup-busy = En säkerhetskopiering pågår redan. Försök igen om en stund
versions-restore-in-backup-file = Du tittar på en säkerhetskopia; öppna själva projektet för att återställa till det
versions-restore-no-project = Inget projekt är öppet
versions-recreate-button = Hämta tillbaka…
versions-recreate-picker-title = Var ska det placeras?
versions-recreate-picker-empty = Projektet har ingen pärm att placera det i
versions-recreate-picker-confirm = Hämta tillbaka hit
versions-recreate-picker-cancel = Avbryt
versions-recreate-untitled = det här objektet
versions-recreate-confirm-title = Hämta tillbaka ”{ $item }”?
versions-recreate-confirm-text = Det läggs till i { $destination }, med texten det hade den { $date }.
versions-recreate-confirm-undo-note = Ångra i meddelandet som följer tar bort det direkt igen.
versions-recreated-toast = ”{ $item }” finns i ditt projekt igen
versions-recreated-partial-toast = { $count ->
       [one] ”{ $item }” är tillbaka, men en av dess texter kunde inte läsas
      *[other] ”{ $item }” är tillbaka, men { $count } av dess texter kunde inte läsas
   }
versions-recreate-already-here = Objektet finns redan i ditt projekt
versions-recreate-no-destination = Välj en plats i pärmen att placera det på
versions-recreate-unreadable = Säkerhetskopian kunde inte läsas, så inget lades till
versions-recreate-failed = Det gick inte att hämta tillbaka det: { $error }
versions-changed-percent = { $percent } % av detta har ändrats
versions-hidden-paragraphs = { $count ->
       [one] … { $count } oförändrat stycke …
      *[other] … { $count } oförändrade stycken …
   }
timeline-title = Gå tillbaka i tiden
timeline-coverage = { $count ->
       [one] { $count } version registrerad, tillbaka till { $oldest }
      *[other] { $count } versioner registrerade, tillbaka till { $oldest }
   }
timeline-loading = Går igenom projektets historik…
timeline-empty = Ingen version av projektet har registrerats ännu
timeline-no-changes = Inget har ändrats sedan dess
timeline-no-text-changes = Ingen text har ändrats sedan dess
timeline-slider-label = Registrerad version
timeline-series-name = Projektets storlek
timeline-bars-caption = Varje stapel är en registrerad version, lika hög som projektet var stort då. Den markerade är den du tittar på.
timeline-bars-caption-periods = För många versioner för att visas en och en, så varje stapel är en { $period }, lika hög som projektet var stort i slutet av den.
timeline-unit-hour = timme
timeline-unit-day = dag
timeline-unit-week = vecka
timeline-unit-month = månad
timeline-range-filter = Visa bara versioner mellan två datum
timeline-range-empty = Ingen version registrerades mellan de datumen
timeline-open-period = Öppna den här perioden
timeline-show-all = Visa hela historiken
timeline-changed-since = { $count ->
       [one] { $count } objekt skiljer sig mellan { $date } och ditt projekt nu
      *[other] { $count } objekt skiljer sig mellan { $date } och ditt projekt nu
   }
timeline-kind-added = Skrivet sedan dess
timeline-kind-removed = Finns inte längre i projektet
timeline-kind-changed = Redigerat sedan dess
timeline-kind-moved = Flyttat sedan dess
timeline-not-yet-written = Det här fanns ännu inte vid den tidpunkten
timeline-no-text-of-its-own = Det här har ingen egen text. Det är en rubrik för det som ligger inuti
timeline-reader-close = Stäng
timeline-reader-stamp = Som det var den { $date }
timeline-reader-loading = Öppnar den registrerade versionen…
timeline-reader-unreadable = Den här registreringen kunde inte läsas. Säkerhetskopian kan ha flyttats eller tagits bort, eller ligga på en enhet som inte är ansluten.
timeline-reader-compared = Som det var den { $date }, jämfört med hur det lyder nu
timeline-reader-diff-legend = Genomstruket har försvunnit sedan dess; understruket har lagts till.
timeline-reader-view-label = Vad som ska visas
timeline-reader-view-diff = Ändringar
timeline-reader-view-text = Text
timeline-reader-deleted = Det här finns inte längre i ditt projekt. Du kan läsa det och kopiera ut det här.
timeline-prose-only-record = Den här punkten kommer från projektets egen historik, som bara sparar text, så det som har tagits bort eller flyttats sedan dess kan inte visas. Välj en säkerhetskopia för det.
settings-sec-extensions = Tillägg
settings-group-games-forward = Alltid framåt
settings-games-forward-toggle = Spela ”Alltid framåt”
settings-games-forward-hint =
       Medan du spelar kan inget du har skrivit tas tillbaka: Backsteg, Delete,
       Klipp ut, dra och släpp samt Ångra är avstängda i de ytor du väljer nedan.
       Du kan fortfarande skriva, klistra in, formatera och röra dig fritt. Utkastet bara växer.
settings-games-session-warning =
       Det här valet gäller bara den här sessionen: det sparas aldrig. När du stänger projektet
       eller avslutar Skribisto tar leken alltid slut, och allt du skrev medan du spelade
       går att ångra igen i samma stund.
settings-group-games-scope = Var den gäller
settings-games-in-prose = Manusets prosa
settings-games-in-synopsis = Synopsisar
settings-games-scope-hint =
       Kommentarer, fotnoter och titlar fryses aldrig: där antecknar du den rättelse
       du just har förbjudit dig själv att göra.
settings-games-inert-warning =
       ”Alltid framåt” är på men gäller ingenting. Markera minst en yta ovan,
       annars ändrar den ingenting medan du skriver.
games-title = Skrivlekar
games-forward-name = Alltid framåt
games-forward-blurb =
       Skriv utkast utan att ta tillbaka något. Att ta bort, klippa ut och ångra är avstängt
       medan du spelar. Skriv nästa mening i stället för att rätta den förra.
games-forward-playing = Spelar: borttagning är avstängd
games-forward-idle = Spelar inte
games-session-note = Avslutas när du stänger projektet.
games-scope-prose-and-synopsis = Gäller din prosa och dina synopsisar.
games-scope-prose = Gäller din prosa.
games-scope-synopsis = Gäller dina synopsisar.
games-scope-nothing = Gäller ingenting ännu. Välj en yta i Inställningar.
games-settings-link = Inställningar för skrivlekar…
statusbar-games-forward = Alltid framåt
statusbar-games-forward-tooltip =
       ”Alltid framåt” är på: att ta bort, klippa ut och ångra är avstängt medan du
       skriver utkast. Klicka för att sluta spela.
goal-progress-words = { $g ->
       [one] { $count } av { $goal } ord
      *[other] { $count } av { $goal } ord
   }
goal-progress-characters = { $g ->
       [one] { $count } av { $goal } tecken
      *[other] { $count } av { $goal } tecken
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
overview-excluded-from-export = Utesluts från exporten och räknas därför inte in i någon summa
goal-subtree-total-words = { $count ->
       [one] { $items } mål här inuti blir sammanlagt { $words } ord
      *[other] { $items } mål här inuti blir sammanlagt { $words } ord
   }
goal-subtree-total-characters = { $count ->
       [one] { $items } mål här inuti blir sammanlagt { $words } tecken
      *[other] { $items } mål här inuti blir sammanlagt { $words } tecken
   }
new-work-goal-unit = Räkna i
new-work-goal-unit-hint = Används för alla mål för ord eller tecken i det här projektet. Kan ändras senare.
settings-group-goal-unit = Mål
settings-goal-unit-switch-title = Byt räkneenhet
settings-goal-unit-switch-text = Målen och milstolparna som redan har angetts för projektet angavs i enheten ”{ $from }”. Att byta till ”{ $to }” räknar inte om dem, så varje befintligt tal kommer nu att läsas i enheten ”{ $to }”.
settings-goal-unit-switch-informative = Inget går förlorat. Byt tillbaka när som helst för att återfå den ursprungliga tolkningen, och uppdatera sedan de mål du vill behålla.
settings-goal-unit-switch-confirm = Byt
goal-distribute-action = Fördela…
distribute-title = Fördela målet
distribute-empty = Det finns inget här inuti att fördela målet på.
distribute-weight = Fördela
distribute-weight-length = Efter längd
distribute-weight-rows = Efter antal delar
distribute-weight-even = Jämnt
distribute-overwrite = Ersätt mål som redan har angetts
distribute-col-item = Objekt
distribute-col-current = Nu
distribute-col-proposed = Efter
distribute-total = De blir sammanlagt { $total }, mot ett mål på { $goal }.
distribute-over-budget = Målen som redan har angetts här inuti är { $over } mer än behållarens eget mål. Höj dess mål, sänk deras eller ersätt dem.
distribute-apply = Fördela
distribute-cancel = Avbryt
milestone-target-gone = Målet har tagits bort
milestone-no-target = Inget mål
milestone-add = Lägg till
milestone-add-label = Vad ska vara uppnått
pace-summary-title = Så ligger boken till
pace-summary-remaining = { $words } kvar
pace-summary-open = Öppna planen
pace-summary-close = Stäng
pace-summary-dont-show = Visa inte detta när projektet öppnas
pace-summary-menu = Skrivplan…
menu-help-topics = &Hjälpavsnitt
menu-help-shortcuts = Ko&rtkommandon…
menu-help-website = Skribisto på &webben
menu-help-report = Rapportera ett &problem…
menu-help-check-updates = Sök efter &uppdateringar
menu-command-palette = &Kommandopalett…
learn-help-topics = Hjälpavsnitt
learn-shortcuts = Kortkommandon
learn-website = Skribisto på webben
help-window-title = Hjälp
help-filter-topics = Filtrera avsnitt
help-no-matching-topic = Inget avsnitt matchar.
help-topic-missing = Det här avsnittet finns inte längre.
help-back = Tillbaka
help-not-translated = Den här sidan har inte översatts ännu, så den visas på engelska.
help-section-getting-started = Komma igång
help-section-writing = Skriva
help-section-reviewing = Granska
help-section-exchanging = Få in och ut text
help-section-keeping = Skydda ditt arbete
help-section-extensions = Tillägg
help-topic-getting-started = Ditt första projekt
help-topic-writing-model = Hur en bok är uppbyggd
help-topic-drafts-and-old-versions = Behålla ett äldre utkast
help-topic-goals-and-pace = Mål och tempo
help-topic-comments = Kommentarer
help-topic-round-trip = Skicka boken till en läsare
help-topic-export = Exportera
help-topic-import-documents = Importera dokument
help-topic-import-projects = Flytta över ett helt projekt
help-topic-backups-and-versions = Säkerhetskopior och versioner
help-shortcuts-title = Kortkommandon
help-shortcuts-filter = Filtrera kortkommandon
help-shortcuts-no-matches = Inget kortkommando matchar.
help-shortcuts-rebind = Ändra kortkommandon…
help-shortcuts-close = Stäng
command-palette-placeholder = Skriv ett kommando
shortcut-name-binder-duplicate = Duplicera
shortcut-name-comments-add = Lägg till kommentar
shortcut-name-comments-add-paragraph = Kommentera stycket
shortcut-name-spellcheck-toggle = Kontrollera stavning
shortcut-name-editor-tab-close = Stäng flik
shortcut-name-editor-tab-pin = Fäst eller lossa flik
shortcut-name-editor-save = Spara
shortcut-name-work-export = Exportera…
shortcut-name-work-new = Nytt projekt
shortcut-name-work-open = Öppna projekt
shortcut-name-window-new = Nytt fönster
shortcut-name-work-close = Stäng projekt
shortcut-name-app-settings = Inställningar
shortcut-name-app-quit = Avsluta
shortcut-name-editor-insert-footnote = Infoga fotnot
shortcut-name-format-scene-break = Infoga scenbrytning
shortcut-name-format-major-scene-break = Infoga större scenbrytning
shortcut-name-format-link = Infoga länk
shortcut-name-format-mark-nbsp = Infoga hårt mellanslag
shortcut-name-format-mark-nbhyphen = Infoga hårt bindestreck
shortcut-name-format-mark-nnbsp = Infoga smalt hårt mellanslag
shortcut-name-format-mark-zwsp = Infoga nollbreddsmellanslag
shortcut-name-go-next = Nästa
shortcut-name-go-prev = Föregående
shortcut-name-go-to = Gå till
shortcut-name-outline-toggle = Visa eller dölj dispositionen
shortcut-name-preview-toggle = Visa eller dölj förhandsvisningsfältet
shortcut-name-view-fullscreen = Växla helskärm
shortcut-name-view-focus-mode = Växla distraktionsfritt läge
shortcut-name-editor-size-increase = Öka textstorleken
shortcut-name-editor-size-decrease = Minska textstorleken
shortcut-name-editor-size-reset = Återställ textstorleken
shortcut-name-editor-find = Sök
shortcut-name-editor-replace = Ersätt
shortcut-name-editor-find-next = Nästa träff
shortcut-name-editor-find-prev = Föregående träff
shortcut-name-search-show = Sök i projektet
shortcut-name-search-replace = Ersätt i projektet
shortcut-name-outline-open-to-side = Öppna bredvid
shortcut-name-help-topics = Hjälpavsnitt
shortcut-name-help-shortcuts = Kortkommandon
shortcut-name-help-website = Skribisto på webben
shortcut-name-help-report = Rapportera ett problem
shortcut-name-help-check-updates = Sök efter uppdateringar
update-check-behind = Version { $version } finns tillgänglig.
update-check-current = Du kör den senaste versionen.
update-check-failed = Det gick inte att söka efter en nyare version. { $error }
shortcut-name-command-palette = Kommandopalett
help-section-reference = Vad saker är
help-concept-scene-break = Scenbrytning
help-concept-major-scene-break = Större scenbrytning
help-concept-find-in-prose = Sök i prosan
help-concept-story-bible = Berättelsebibel
help-concept-goal-unit = Ord eller tecken
help-concept-goal-progress = Framsteg
help-concept-manuscript-words = Vad som räknas som manus
help-concept-exportable = Utesluten från exporten
help-concept-distribute = Fördela ett mål
help-concept-subtree-total = Vad målen här inuti blir sammanlagt
help-concept-milestone = Milstolpe
help-concept-pace-plan = Tempoplan
help-concept-tag = Tagg
help-concept-label = Etikett
help-concept-point-of-view = Berättarperspektiv
help-concept-epigraph = Motto
help-concept-footnote = Fotnot
help-concept-chapter-mode = Kapitlens form
help-concept-comment = Kommentar
help-concept-backup = Säkerhetskopia
help-concept-version = Version
help-concept-trash = Papperskorg
help-concept-spellcheck = Stavningskontroll
help-concept-search-replace = Sök och ersätt
help-concept-note-template = Anteckningsmall
help-concept-story-bible-entry = Post i berättelsebibeln
help-concept-text-replacement = Textersättning
help-concept-smart-punctuation = Smart interpunktion
help-concept-export-style = Exportstil
help-concept-round-trip-marks = Returmarkörer
epigraph-mark-attribution = Källrad
epigraph-mark-attribution-tip = Markera raden där markören står som citatets källa, så att den skrivs ut som en källhänvisning
margin-lane-name = Marginalmarkeringar
margin-lane-provider-comments = Kommentarer
margin-lane-provider-story-bible = Post i berättelsebibeln
margin-lane-provider-story-bible-hint = I en antecknings vy I prosan: var posten nämns, och scenerna som berättas genom dess ögon. Markerar också namnen i själva prosan.
margin-lane-mark-point-of-view = Berättas härifrån
margin-lane-mark-named-here = Nämns här
margin-lane-provider-comments-hint = Där en kommentar är fäst vid texten
margin-lane-provider-search = Sökträffar
margin-lane-provider-search-hint = Varje träff för det du senast sökte efter, var den än finns i dokumentet
margin-lane-provider-boundaries = Var varje dokument börjar
margin-lane-provider-boundaries-hint = En linje överst i varje scen i ett flöde, så att du ser var du är
margin-lane-provider-spelling = Stavning
margin-lane-provider-spelling-hint = Där stavningskontrollen har markerat ett ord. Av om du inte ber om det: det markerar en maskins åsikt om din prosa
margin-lane-spelling = { $word }, möjligt stavfel
margin-lane-search-hit = { $text }, träff { $index } av { $total }
margin-lane-search-current = { $text }, träff { $index } av { $total }, den du står på
margin-lane-boundary = Början av { $title }
margin-lane-boundary-untitled = ett namnlöst dokument
settings-page-margin-lane = Marginalmarkeringar
settings-desc-margin-lane = Remsan bredvid rullningslisten, och vad den visar
settings-margin-lane-enabled = Visa marginalremsan
settings-margin-lane-enabled-hint = Samma brytare som Visa ▸ Marginalmarkeringar.
settings-margin-lane-enabled-more = Den säger aldrig att något är fel: den visar var saker finns och överlåter åt dig att dra slutsatser.
settings-group-margin-lane-marks = Vad den markerar
settings-margin-lane-no-providers = Inget markerar remsan ännu.
settings-group-margin-lane-texture = Dialogtextur
settings-margin-lane-texture = Visa dialogtexturen
settings-margin-lane-texture-hint = En stapel per stycke: hur långt det är, och hur mycket av det som är repliker.
settings-margin-lane-texture-more = Mäts inte för språk utan en sammanställd konvention; där visar stapeln bara längden.
settings-group-margin-lane-surfaces = Var den visas
settings-margin-lane-surface-editor = Textredigerare
settings-margin-lane-surface-stream = Flöden
settings-margin-lane-surface-search-preview = Sökförhandsvisning
menu-margin-lane = Margin&almarkeringar
status-preset-drafting = Skrivande
status-preset-passes = Bearbetningsomgångar
status-preset-plume = Plume Creator
status-todo = Att göra
status-draft = Utkast
status-revised = Reviderad
status-final = Slutgiltig
status-outline = Skiss
status-first-edit = Första bearbetningen
status-second-edit = Andra bearbetningen
status-done = Klar
status-plume-draft-1 = Första utkastet
status-plume-draft-2 = Andra utkastet
status-plume-draft-3 = Tredje utkastet
status-plume-edit-1 = Första bearbetningen
status-plume-edit-2 = Andra bearbetningen
status-plume-edit-3 = Tredje bearbetningen
status-plume-proofread = Korrekturläst
status-plume-finished = Färdig
new-work-statuses = Arbetsflöde
new-work-statuses-hint = Stegen som en scen går igenom. Du kan när som helst byta namn på dem, ändra ordningen eller lägga till fler.
status-none = Ingen status
inspector-status = Status
overview-col-status = Status
overview-status-mixed = delarna nedanför skiljer sig åt
help-concept-status = Status
status-completion-title = Så ligger boken till
status-completion-headline = { $done } av { $total } scener färdiga
status-completion-empty = Inga scener ännu. Det här läser manuset, så det fylls i medan du skriver.
status-completion-open = Så ligger boken till…
settings-page-statuses = Statusar
settings-desc-statuses = Byt namn på, ordna om, lägg till och ta bort stegen som en scen går igenom
settings-statuses-add = Lägg till status
settings-statuses-add-placeholder = Namn på ny status
settings-statuses-desc =
       Stegen som en scen går igenom, i ordning, från minst färdig överst till mest
       färdig nederst. Den ordningen är vad ”mindre färdig än” betyder överallt annars
       i programmet, så ordna dem så som din arbetsprocess faktiskt ser ut.
settings-statuses-apply-preset = Använd en förinställning…
settings-statuses-preset-applied = { $added ->
       [one] 1 status har lagts till
      *[other] { $added } statusar har lagts till
   }
settings-statuses-preset-refused = Projektet har redan en statusstege. Förinställningar fyller bara en tom stege. Ta först bort de steg du inte vill ha.
settings-statuses-duplicate = ”{ $name }” finns redan på stegen
settings-statuses-added = ”{ $name }” har lagts till
settings-statuses-move-up = Flytta upp (mindre färdig)
settings-statuses-move-down = Flytta ned (mer färdig)
settings-statuses-details-placeholder = Vad det här steget betyder (valfritt)
settings-statuses-delete = Ta bort ”{ $name }”
settings-statuses-delete-in-use = { $count ->
       [one] Ta bort ”{ $name }”: 1 objekt har den och förlorar sin status
      *[other] Ta bort ”{ $name }”: { $count } objekt har den och förlorar sin status
   }
settings-statuses-deleted = ”{ $name }” har tagits bort
settings-statuses-deleted-in-use = { $count ->
       [one] ”{ $name }” har tagits bort. 1 objekt har inte längre någon status.
      *[other] ”{ $name }” har tagits bort. { $count } objekt har inte längre någon status.
   }
settings-statuses-empty-title = Inget arbetsflöde ännu
settings-statuses-empty-body = En status anger hur långt en scen har kommit. Lägg till en ovan, eller utgå från en förinställning.
status-category-planned = Planerad
status-category-drafting = Under skrivning
status-category-needs-work = Behöver bearbetas
status-category-revised = Reviderad
status-category-final = Slutgiltig
progress-percent = { $percent } %
progress-percent-with-label = { $percent } % · { $label }
progress-import-opening = Öppnar projektet…
progress-import-outline = Läser strukturen…
progress-import-converting = Konverterar kapitel och scener…
progress-import-writing = Skriver .skrib-filen…
progress-import-done = Klar
progress-import-structure = Räknar ut strukturen…
progress-export-start = Startar…
progress-export-writing = Skriver…
progress-project-reading = Läser projektet…
settings-sec-work-titled = Projekt: { $title }
import-notice-flattened-rich-texts = { $count ->
       [one] En text var kapslad djupare än ett Skribisto-projekt kan rymma; orden behölls som vanlig text, utan formatering.
      *[other] { $count } texter var kapslade djupare än ett Skribisto-projekt kan rymma; orden behölls som vanlig text, utan formatering.
   }
import-notice-free-text-badges = { $count ->
       [one] En scenmärkning ser ut som fritext snarare än en lista över statusar, så den behölls som objektets etikett i stället för att bli en tagg.
      *[other] { $count } olika scenmärkningar ser ut som fritext snarare än en lista över statusar, så de behölls som objektens etiketter i stället för att bli taggar.
   }
import-notice-separator-text-dropped = Avdelaren ”{ $name }” innehöll text, vilket inte har någon plats i ett Skribisto-projekt; texten importerades inte.
import-notice-separator-without-scene = Avdelaren ”{ $name }” hade ingen scen före sig i sitt kapitel att fästas vid och importerades därför inte.
import-notice-chapter-without-scenes = Kapitlet ”{ $name }” har inga scener, så dess avdelare importerades inte.
import-notice-rich-text-unconverted = En del formaterad text kunde inte konverteras ({ $error }); den importerades inte.
import-notice-unresolved-cross-link = En länk till post nr { $number } i persongalleriet kunde inte följas: posten saknas eller låg i papperskorgen.
import-notice-read-copy = Läste projektets { $container ->
       [single-file] enfilskopia
      *[folder] mappkopia
   }.
import-notice-read-copy-dated = Läste projektets { $container ->
       [single-file] enfilskopia
      *[folder] mappkopia
   }, senast ändrad { $day }.
import-notice-original-format = Projektet är i Manuskripts ursprungliga format från 2016. Det lästes i sin helhet; inget i det ändrades.
import-notice-newer-single-file-beside = ”{ $file }” är nyare än den här mappen ({ $file_day } jämfört med { $folder_day }). Manuskript lämnar den gamla mappen kvar när ett projekt går över till enfilsläge, så mappen kan vara en övergiven kopia; om du importerar .msk-filen i stället läses den nyare.
import-notice-pickle-skipped = ”{ $file }” lästes inte. Det är en Python-pickle, och att läsa in en sådan från en projektfil var ett säkerhetshål i själva Manuskript (CVE-2021-35196); inställningarna i den gäller visning, inte skrivande.
import-notice-file-unreadable = ”{ $file }” kunde inte läsas ({ $error }) och hoppades över; allt annat importerades.
import-notice-not-utf8 = ”{ $file }” är inte giltig UTF-8; tecken som inte kunde läsas ersattes i stället för att importen avbröts.
import-notice-table-unreadable = ”{ $file }” kunde inte läsas ({ $error }); innehållet importerades inte.
import-notice-outline-missing = ”{ $file }” saknas, så projektet har inget manus att importera.
import-notice-outline-unreadable = ”{ $file }” kunde inte läsas ({ $error }); inget manus importerades.
import-notice-named-none-row = ”{ $file }” börjar med en namngiven rad (”{ $name }”) där Manuskript har sin tomma post ”none”. Den utelämnades, så att raderna efter den behåller de nummer som manuset använder.
import-notice-unknown-field = ”{ $file }” har ett fält som den här importen inte känner till, ”{ $field }”. Det importerades inte.
import-notice-colorless-labels = { $count ->
       [one] ”{ $file }” har en rad utan färg. Den importerades utan färg; Manuskript slutar läsa in projektet vid en sådan rad.
      *[other] ”{ $file }” har { $count } rader utan färg. De importerades utan färg; Manuskript slutar läsa in projektet vid en sådan rad.
   }
import-notice-settings-unreadable = ”{ $file }” kunde inte läsas ({ $error }); projektets språk importerades inte.
import-notice-revisions-unreadable = ”{ $file }” kunde inte läsas ({ $error }); projektets historik importerades inte, men inget annat påverkades.
import-notice-foreign-files = { $count ->
       [one] Projektet innehåller en fil som inte hör till Manuskript-formatet och som inte importerades: { $files }.
      *[other] Projektet innehåller { $count } filer som inte hör till Manuskript-formatet och som inte importerades: { $files }.
   }
import-notice-foreign-files-more = Projektet innehåller { $count } filer som inte hör till Manuskript-formatet och som inte importerades: { $files } och { $more ->
       [one] en till
      *[other] { $more } till
   }.
import-notice-not-a-character-file = ”{ $file }” är ingen personfil från Manuskript och utelämnades.
import-notice-not-a-text-item = ”{ $file }” är inget textobjekt från Manuskript och utelämnades ur strukturen.
import-notice-missing-blank-line = ”{ $file }” saknar en tom rad efter sitt huvud. Det sista fältet behölls, trots att Manuskript tappar det.
import-notice-folder-without-metadata = ”{ $folder }” saknar { $file }, så den importerades som en vanlig mapp med katalogens namn. Manuskript hoppar över en sådan katalog och allt i den.
import-notice-rows-given-ids = { $count ->
       [one] En rad i strukturen saknade eget id och fick ett. Manuskript tappar en sådan rad, eller slutar läsa in projektet om den är en mapp.
      *[other] { $count } rader i strukturen saknade eget id och fick ett. Manuskript tappar en sådan rad, eller slutar läsa in projektet om den är en mapp.
   }
import-notice-plots-unreadable = ”{ $file }” kunde inte läsas ({ $error }); intrigerna importerades inte.
import-notice-world-unreadable = ”{ $file }” kunde inte läsas ({ $error }); världsposterna importerades inte.
import-notice-world-not-opml = ”{ $file }” är inget OPML-dokument; världsposterna importerades inte.
import-notice-text-flattened = Texten i ”{ $title }” är kapslad djupare än ett Skribisto-projekt kan rymma; orden behölls som vanlig text, utan formatering.
import-notice-text-unconverted = Texten i ”{ $title }” kunde inte konverteras ({ $error }); den behölls exakt som den var skriven.
import-notice-text-without-file-name = { $kind ->
       [scene] Scentexten
       [note] Anteckningstexten
       [synopsis] Synopsisen
      *[title] Titeln
   } för ”{ $title }” kunde inte få något filnamn och importerades därför inte.
import-notice-unknown-label = ”{ $title }” har etikett { $index }, som inte finns i projektets etikettlista. Objektet importerades utan etikett.
import-notice-unknown-status = ”{ $title }” har status { $index }, som inte finns i projektets statuslista. Objektet importerades utan status.
import-notice-unknown-point-of-view = ”{ $title }” berättas genom en person som inte finns i projektets persongalleri. Berättarperspektivet angavs inte.
import-notice-unresolved-references = Hänvisningarna i texten i ”{ $title }” kunde inte lösas upp ({ $error }); texten behölls som den konverterades, inklusive hänvisningsmarkörer.
import-notice-custom-icons = { $count ->
       [one] En rad hade en egen ikon. Skribisto ger varje rad ikonen för dess typ, så ikonen importerades inte; inget annat med raden ändrades.
      *[other] { $count } rader hade en egen ikon. Skribisto ger varje rad ikonen för dess typ, så ikonerna importerades inte; inget annat med raderna ändrades.
   }
import-notice-character-colors = { $count ->
       [one] En person hade en färg bredvid sitt namn. Skribisto färgar taggar snarare än rader, så färgen importerades inte; personen själv gjorde det.
      *[other] { $count } personer hade en färg bredvid sitt namn. Skribisto färgar taggar snarare än rader, så färgerna importerades inte; personerna själva importerades alla.
   }
import-notice-versions-kept = { $found ->
       [one] Projektet hade registrerat en tidigare version
      *[other] Projektet hade registrerat { $found } tidigare versioner
   }; { $kept ->
       [one] en behölls
      *[other] { $kept } behölls
   }, utglesade på samma sätt som Skribisto glesar ut sin egen historik. De finns i panelen Versioner.
import-notice-versions-unplaced = { $count ->
       [one] En tidigare version hör till en rad som inte längre finns i projektet och kunde inte placeras.
      *[other] { $count } tidigare versioner hör till en rad som inte längre finns i projektet och kunde inte placeras.
   }
dialog-open-work-title = Öppna Skribisto-projekt
dialog-save-as-file-title = Spara som en enda .skrib-fil
dialog-save-as-folder-title = Välj en överordnad mapp för projektet
dialog-manuskript-file-title = Välj ett Manuskript-projekt
dialog-manuskript-folder-title = Välj en projektmapp från Manuskript
dialog-filter-skrib = Skribisto-projekt
dialog-filter-manuskript = Manuskript-projekt
dialog-filter-plume = Plume Creator-projekt
dialog-filter-hunspell-aff = Hunspell-affixfil (.aff)
dialog-filter-hunspell-dic = Hunspell-ordlista (.dic)
dialog-filter-export = Exporterad fil
shortcut-name-view-margin-lane = Växla marginalmarkeringar
df-theme-paper = Papper
df-theme-sepia = Sepia
df-theme-night = Natt
df-theme-high-contrast = Hög kontrast
df-theme-midnight = Midnatt
welcome-example-byline = av { $author }
welcome-sections-a11y = Välkomstavsnitt
comments-a11y-summary-author = { $author }: { $text }
comments-a11y-summary-replies = { $text } ({ $count ->
       [one] 1 svar
      *[other] { $count } svar
   })
comments-a11y-summary-author-replies = { $author }: { $text } ({ $count ->
       [one] 1 svar
      *[other] { $count } svar
   })
margin-lane-a11y-mark = { $label }, { $percent } procent in
margin-lane-a11y-marks = { $label }, { $count } markeringar, { $percent } procent in
margin-lane-a11y-fallback = { $count ->
       [one] Marginalmarkeringar, 1 markering
      *[other] Marginalmarkeringar, { $count } markeringar
   }
export-choose-unnamed-binder = Namnlös pärm
export-style-neutral = Neutral
export-style-manuscript-shunn = Standardmanus (Shunn)
export-style-ebook-clean = Ren e-bok
date-time-short = { $year }-{ $month }-{ $day } { $hour }:{ $minute }
pace-day-label = { $day }/{ $month }
byte-size-bytes = { $value } B
byte-size-kb = { $value } kB
byte-size-mb = { $whole },{ $tenth } MB
byte-size-gb = { $whole },{ $tenth } GB
unit-percent = { $value } %
unit-pixels = { $value } px
settings-paratext-error-no-name = En förinställning behöver en `name`-rad.
export-error-no-project = Inget projekt är öppet att exportera.

shortcut-group-work = Projekt
shortcut-group-edit = Redigera
shortcut-group-view = Visa
shortcut-group-document = Dokument
shortcut-group-format = Format
shortcut-group-go = Gå
shortcut-group-tools = Verktyg
shortcut-group-help = Hjälp

## Manuskript import: the headings of the notes it writes (a character's
## sheet, a world entry, a plot and its steps, the project details, the
## snowflake summary ladder) and the title of a project with no name. Written
## into the imported project once, in the interface language; the writer can
## rename them like any other text. Goal and Conflict reuse the note templates'
## own words.
import-manuskript-section-paragraph-summary = Sammanfattning i ett stycke
import-manuskript-section-full-summary = Fullständig sammanfattning
import-manuskript-section-motivation = Motivation
import-manuskript-section-epiphany = Insikt
import-manuskript-section-notes = Anteckningar
import-manuskript-section-description = Beskrivning
import-manuskript-section-passion = Passion
import-manuskript-section-result = Resultat
import-manuskript-section-summary = Sammanfattning
import-manuskript-section-meta = Metadata
import-manuskript-section-series = Serie
import-manuskript-section-volume = Volym
import-manuskript-section-genre = Genre
import-manuskript-section-license = Licens
import-manuskript-section-email = E-post
# The snowflake ladder's five tellings, shortest first: one situation, one
# sentence, one paragraph, one page, the full summary.
import-manuskript-section-situation = Situation
import-manuskript-section-sentence = En mening
import-manuskript-section-paragraph = Ett stycke
import-manuskript-section-page = En sida
import-manuskript-section-full = Fullständig sammanfattning
import-manuskript-untitled-project = Importerat Manuskript-projekt

## Plume import: the story-bible groups an old (0.3 and earlier) project
## implies without naming them, and the title of a project with no name.
## Written into the imported project once, in the interface language.
import-plume-group-characters = Personer
import-plume-group-items = Föremål
import-plume-group-places = Platser
import-plume-untitled-project = Importerat Plume-projekt
