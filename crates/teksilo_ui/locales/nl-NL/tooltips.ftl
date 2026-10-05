# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

wm-book = Een boek in uw project.
wm-book-more = Een boek is zo eenvoudig of zo gelaagd als u wilt. Een kort verhaal kan uit slechts een paar [scènes](:wm-scene) bestaan. Een langer werk kan [delen](:wm-part) en [hoofdstukken](:wm-chapter) zo diep nesten als u wilt. De vorm bepaalt u zelf.
wm-part = Een deel dat hoofdstukken groepeert.
wm-part-more = Een deel brengt [hoofdstukken](:wm-chapter) samen onder één kop. Net als een hoofdstuk loopt het door tot het volgende deel of het einde van het [boek](:wm-book).
wm-chapter = Een hoofdstuk van het boek.
wm-chapter-more = Een hoofdstuk loopt van hier tot het volgende hoofdstuk, het volgende [deel](:wm-part) of het [einde van het boek](:wm-end-of-book). Het bestaat in twee vormen: een hoofdstukmap die [scènes](:wm-scene) bevat, of een plat hoofdstuk met een eigen tekst. Kies welke vorm nieuwe hoofdstukken krijgen in Instellingen, onder Opbouw.
wm-scene = Een scène met proza.
wm-scene-more = Een scène bevat uw tekst. Elk schrijfitem heeft twee kanten: de hoofdtekst en een [synopsis](:wm-synopsis) om mee te plannen. Beide gaan samen open in de editor.
wm-note = Een notitie.
wm-note-more = Een notitie bevat tekst en een [synopsis](:wm-synopsis), net als een [scène](:wm-scene), voor research, zijsprongen of geheugensteuntjes. Elk item kan deel uitmaken van het samengestelde boek, ook een notitie in een [hoofdstuk](:wm-chapter).
wm-note-folder = Een notitie die andere items bevat.
wm-note-folder-more = Een [notitie](:wm-note) die ook items onder zich heeft, zodat u verwante notities kunt groeperen. Ze heeft een eigen [synopsis](:wm-synopsis), en de items erin hebben hun eigen tekst.
wm-folder = Een map om te ordenen.
wm-folder-more = Een container om items in de ordner te schikken zoals het u uitkomt. Hij heeft een [synopsis](:wm-synopsis) maar geen eigen proza. De items erin bevatten de tekst.
wm-end-of-book = Markeert waar een boek eindigt.
wm-end-of-book-more = Alle boeken delen één doorlopende lijst, dus het einde van een boek wordt niet door nesting bepaald. Deze markering zegt dat het [boek](:wm-book) hier stopt. Alles erna hoort bij het volgende boek.
wm-story-bible-entry = Een notitie over uw verhaal, met tags en alternatieve namen.
wm-story-bible-entry-more = Een [notitie](:wm-note) over een personage, een plaats of iets anders dat het bewaren waard is, in één stap aangemaakt met een naam, tags, alternatieve namen en een beginsjabloon. Daarna is er op de pagina niets meer dat haar onderscheidt: ze blijft een gewone notitie die u kunt hernoemen, opnieuw taggen of omzetten zoals elke andere.
wm-synopsis = Een samenvatting van een schrijfitem.
wm-synopsis-more = Een samenvatting bij een schrijfitem, meestal een korte alinea maar zo lang als u wilt. Gebruik ze om te plannen en te navigeren voordat het proza bestaat. Ze staat in de editor naast de hoofdtekst.

scene-break-minor = Een gewone scèneovergang: een sprong in tijd, plaats of perspectief binnen een hoofdstuk.
scene-break-minor-more =
    Markeert een overgang *waar u hem plaatst*, ook midden in een [scène](:wm-scene). Proza in twee items splitsen is een keuze voor de ordening en maakt dus nooit vanzelf een overgang.

    Wordt in het proza getypt als drie sterretjes met spaties ertussen. Hoe hij wordt afgedrukt, bepaalt de exportstijl, niet wat u typt: een Shunn-manuscript zet er één # neer, een paperback een dinkus, en de meeste Franse, Duitse, Spaanse, Russische en Italiaanse uitgevers een kale witregel zonder teken.
scene-break-major = Een sterkere scheiding: een grote sprong in de tijd, of een ingrijpende wisseling van perspectief.
scene-break-major-more =
    Hetzelfde idee als een gewone scèneovergang, één stap sterker. Gebruik hem wanneer een gewone overgang de sprong te klein zou laten lijken.

    Wordt getypt als # # # en door de exportstijl anders afgedrukt dan de gewone overgang. Het standaardmanuscriptformaat van Shunn zet precies voor dit onderscheid één # tegenover # # #. Waar een traditie geen sterker teken kent, worden beide hetzelfde afgedrukt.
wm-find-in-prose = Skribisto zoekt dit item in uw proza.
wm-find-in-prose-more = Schakel dit in voor tags die dingen benoemen waarover u schrijft: personages, plaatsen, voorwerpen. Elk item met zo’n tag wordt op zijn titel en op de andere namen die u het geeft met uw proza vergeleken, zodat elke [scène](:wm-scene) laat zien wie en wat erin voorkomt zonder dat u iets met de hand koppelt. Laat het uit voor tags die een item beschrijven in plaats van benoemen, zoals een fase van het schrijven of een herinnering om de continuïteit te controleren. Elk item dat gevonden wordt, komt ook in uw [verhaalbijbel](:wm-story-bible).
wm-story-bible = Alle personages, plaatsen en andere getagde items, verzameld in één raster.
wm-story-bible-more = Te openen vanuit het eigen tabblad van een notitiemap. Kaarten worden per tag gegroepeerd, en elke kaart vermeldt naar hoeveel alternatieve namen een item luistert en in hoeveel scènes het is opgedoken, op basis van elke tag met [zoeken in proza](:wm-find-in-prose) in het project. Filteren per boek beperkt alleen welke kaarten worden getoond; het bepaalt nooit welke items bestaan.

tooltip-go-to = Naar een willekeurig item in de ordner springen (Ctrl+G)
synopsis-collapse-tooltip = De synopsiskolom verbergen

wm-paratext = Een tekst die geen deel uitmaakt van het verhaal.
wm-paratext-more = Een voorwoord, een opdracht, een nawoord, een colofon: tekst die bij het boek hoort maar niet bij de hoofdtekst. Wordt geëxporteerd waar u hem ook plaatst, en telt nooit mee in het aantal woorden van het manuscript. Waar hij komt, bepaalt u zelf: de gewoonten verschillen per land en per uitgever.
wm-paratext-folder = Een map voor parateksten.
wm-paratext-folder-more = Een plek voor voorwoorden en nawoorden, zodat ze de ordner niet onoverzichtelijk maken. Alleen om te ordenen: de map heeft een [synopsis](:wm-synopsis) maar voegt niets toe aan het geëxporteerde boek, zelfs haar eigen naam niet.

goal-target = Hoe lang dit stuk moet worden.
goal-target-more = Stel een doel in voor wat u maar wilt: een [scène](:wm-scene), een [hoofdstuk](:wm-chapter), een [deel](:wm-part), het hele [boek](:wm-book). Elk doel is een eigen getal dat op zichzelf staat: een doel voor een hoofdstuk verandert nooit dat van het boek, en het doel van een boek is nooit de som van de hoofdstukken erin. Wat wel optelt, is het geschreven werk: de [voortgang](:goal-progress) van een container is alles wat eronder geschreven is. Een doel op nul is gewoon niet ingesteld. Het getal wordt geteld in [woorden of tekens](:goal-unit), afhankelijk van wat dit project gebruikt.
goal-unit = Of dit project in woorden of in tekens telt.
goal-unit-more = Een keuze voor het hele project, omdat een lengte-eenheid bij het manuscript en zijn markt hoort en niet bij één onderdeel: Duitse en Franse uitgevers meten in aanslagen, Japanse in vellen van 400 tekens, Chinese per duizend tekens, de meeste Engelstalige uitgevers in woorden. Beide getallen worden naast elkaar bijgehouden en nooit in elkaar omgerekend, dus als u van eenheid wisselt, wijst elk [doel](:goal-target) naar het andere getal, en terugschakelen herstelt het eerste. Er wordt niets omgerekend: doelen die al zijn ingevoerd, moet u met de hand bijwerken.
goal-progress = Hoeveel er geschreven is, afgezet tegen het doel.
goal-progress-explained = Een stuk met een eigen tekst wordt gemeten aan zijn eigen lengte; een container aan [alles wat eronder geschreven is](:goal-manuscript-words). De balk kleurt van rood via oranje naar groen naarmate de tekst binnenkomt, en verandert opnieuw zodra het stuk ruim voorbij de geplande lengte gaat, wat handig is om te zien wanneer een hoofdstuk een vaste lengte moet hebben.
goal-manuscript-words = Wat telt als het manuscript.
goal-manuscript-words-more = Alleen proza dat ook echt zou worden geëxporteerd: [scènes](:wm-scene) en [hoofdstukken](:wm-chapter) met een eigen tekst. Een [notitie](:wm-note) of een [paratekst](:wm-paratext) hoort niet bij het manuscript en telt nooit mee. Alles in de prullenbak valt weg, net als alles wat [uit de export is weggelaten](:goal-exportable).
goal-exportable = Dit item wordt uit de export weggelaten.
goal-exportable-more = De eigen lengte wordt nog getoond, omdat de tekst er nog is, maar telt voor geen enkel totaal mee, precies zoals het in geen enkel geëxporteerd boek zal verschijnen. Een container weglaten laat de stukken erin niet weg: elk item staat voor zichzelf, en daarvoor dient de knop “Toepassen op onderliggende items” naast de schakelaar.
goal-distribute = Dit doel verdelen over wat erin zit.
goal-distribute-more = Verdeelt het doel van een [boek](:wm-book) of [deel](:wm-part) over de items direct eronder, zodat de stukken er precies op uitkomen. Standaard vult het alleen de items zonder doel en laat het uw eigen getallen met rust. Wat het schrijft, zijn gewone [doelen](:goal-target), die u daarna vrij kunt bewerken en die vrij uit elkaar mogen lopen: het is één handeling, geen blijvende afspraak.
goal-subtree-total = Waarop de doelen hierin samen uitkomen.
goal-subtree-total-more = Een gegeven, geen doel. Dit project behandelt het doel van een container nooit als de som van de doelen erin, want zo gaat het getal van een map vanzelf verschuiven zodra een scène erin een getal krijgt. Deze regel vertelt u alleen waarop uw eigen getallen uitkomen, zodat u ze zelf kunt vergelijken met het [doel](:goal-target) van de container, of [er een kunt verdelen](:goal-distribute).
goal-milestone = Een datum waarop iets bereikt moet zijn.
goal-milestone-more = Er zijn twee soorten. De ene zegt dat het [boek](:wm-book) op een bepaalde datum een bepaalde lengte moet hebben, een tussenstation op zijn eigen curve. De andere zegt dat een bepaald [hoofdstuk](:wm-chapter) of [deel](:wm-part) dan [zijn eigen doel](:goal-target) moet hebben bereikt, en leest dat getal live in plaats van er een kopie van te bewaren, dus als u het doel bewerkt, wordt de mijlpaal ook bijgewerkt. Beide verschijnen op het [tempoplan](:pace-plan) van het boek.
pace-plan = Het schrijfschema van het boek.
pace-plan-more = Een [doel](:goal-target) en een deadline, samen met de dagen waarop u schrijft, omgerekend naar hoeveel er per dag nodig is. Het leest het eigen doel van het boek, dus het hier instellen of in de Inspector instellen is hetzelfde getal op twee plaatsen. Tussenstations onderweg zijn [mijlpalen](:goal-milestone).

concept-tag = Een gekleurde markering die u op elk item kunt zetten en in het hele project kunt hergebruiken.
concept-tag-more = Tag een item om het te markeren: een fase, een plaats, een verhaallijn die u volgt. Dezelfde tag kan op een willekeurig aantal items staan, en één item kan er meerdere hebben. Schakel [zoeken in proza](:wm-find-in-prose) in voor een tag die iets in uw boek benoemt, een personage of een plaats, en Skribisto gaat ernaar zoeken in uw proza.
concept-label = Een korte notitie die u onder de titel van een item schrijft, voor uzelf.
concept-label-more = Geen [tag](:concept-tag): een label hoort bij één item alleen, platte tekst zonder kleur of lijst erachter, zoiets als “1e plotpunt” of “heeft nog een naam nodig”. Stel het in via het contextmenu van het item, of bewerk het direct in de kolom Label van het Overzicht; het verschijnt als kleine ondertitel onder de titel, zowel in de ordner als in de doorlopende weergave.
concept-point-of-view = Door wiens ogen een scène verteld wordt: een of meer personages uit uw verhaal.
concept-point-of-view-more = In te stellen in de Inspector, naast de rolbezetting. Kiest u iemand die nog niet in de rolbezetting staat, dan wordt die daar ook toegevoegd. Meer dan één perspectief is een geldige keuze, handig voor een gedeelde scène of een wisseling halverwege het boek, en Skribisto houdt dat niet tegen. De kandidaten komen uit de items die een tag met [zoeken in proza](:wm-find-in-prose) in uw verhaalbijbel zet.
concept-epigraph = Een citaat aan het begin van een deel of hoofdstuk.
concept-epigraph-more = Een eigen veld, los van het proza van het [hoofdstuk](:wm-chapter) of [deel](:wm-part) zelf; een [scène](:wm-scene) of een [notitie](:wm-note) heeft er geen. Schrijf er een als gewoon citaat; schrijf er meerdere en elk wordt een eigen blokcitaat, afgedrukt als een eigen motto. De woorden ervan tellen nooit mee als woorden van het manuscript.
concept-footnote = Een verwijzing in het proza, afgedrukt als genummerde voetnoot.
concept-footnote-more = De verwijzing is één enkel teken tussen uw woorden, dus ze verschuift en verdwijnt precies zoals de woorden eromheen. Het nummer wordt nooit opgeslagen: Skribisto berekent het telkens opnieuw op basis van de plaats van elke verwijzing in het hele manuscript, dus als u er een invoegt, worden alle voetnoten erna opnieuw genummerd. De eigen woorden van een voetnoot worden apart van het manuscript geteld, nooit erbij opgeteld.
concept-chapter-mode = Of een nieuw hoofdstuk een map met scènes is of één enkel plat item.
concept-chapter-mode-more = In te stellen in Instellingen, onder Opbouw. U schrijft hoe dan ook direct in een [hoofdstuk](:wm-chapter); het enige verschil is of het daarnaast scènes kan bevatten. Beide vormen leveren hetzelfde boek op, en u kunt ze in één project vrij combineren. Omzetten naar zet een bestaand hoofdstuk van de ene naar de andere vorm zonder een woord te verliezen.
concept-comment = Een notitie die aan een precies stuk proza vastzit en in de marge wordt getoond.
concept-comment-more = Ze onthoudt de woorden waarnaar ze verwijst en wat eromheen staat, geen vaste positie, dus ze overleeft gewoon bewerken en opnieuw laden. Antwoord eronder om een gesprek gaande te houden. Als het precieze citaat niet meer te vinden is, zegt de opmerking dat, in plaats van ongemerkt naar de verkeerde zin te verschuiven.
concept-backup = Een kopie van het hele project op een bepaald moment, ernaast bewaard.
concept-backup-more = Gemaakt volgens een schema dat u kiest: bij het openen, bij het sluiten, met een vaste tussenpoos, of een combinatie daarvan, waarbij oude kopieën worden opgeruimd op leeftijd of op aantal. Een reservekopie openen laadt haar in een eigen venster: u kunt kijken en zelfs bewerken, maar alleen Opslaan als of Herstellen bewaart iets. Anders dan een [versie](:concept-version), die het eigen verleden van één item bijhoudt.
concept-version = Het eigen verleden van één item: elke wijziging in de tekst of de synopsis ervan.
concept-version-more = Samengesteld uit de eigen opslaggeschiedenis van het project en de reservekopieën samen, één regel per wijziging in plaats van één per bestand. Kies een regel om te zien wat er veranderd is en zet alleen de tekst van dat item terug naar die toestand; de rest van het project blijft onaangeroerd, en er wordt eerst een veiligheidskopie gemaakt. Anders dan een [reservekopie](:concept-backup), die een kopie van alles is.
concept-trash = Een item in de prullenbak is verborgen, niet verplaatst: het blijft precies waar het was.
concept-trash-more = In de prullenbak gooien zet alleen een vlag om: het item houdt zijn plaats in de ordner en wordt als inactief gemarkeerd in plaats van verwijderd. Zet het terug en het verschijnt weer op precies die plek, zonder dat u iets opnieuw hoeft in te delen. Prullenbak legen verwijdert de items voorgoed, en elk van deze handelingen is één stap die ongedaan kan worden gemaakt, zoals elke andere bewerking van het project.
concept-spellcheck = Onderstreept woorden die geen enkel geïnstalleerd woordenboek kent, per projecttaal.
concept-spellcheck-more = Elk project heeft een of meer eigen werktalen; een item kan die overschrijven voor een scène in een andere taal. Voeg een woord dat Skribisto niet kent toe via het contextmenu van de editor, aan een persoonlijk woordenboek dat voor elk project geldt. Als u de spellingscontrole uitschakelt, stopt die overal, totdat u haar weer inschakelt.
concept-search-replace = Vindt een woord of zinsdeel overal in het project, niet alleen in het geopende document.
concept-search-replace-more = Doorzoekt elke scène, notitie, titel, synopsis en elk label, en ook opmerkingen en voetnoten. Hoofdlettergevoelig, heel woord en accentgevoelig zijn drie aparte schakelaars. Alles vervangen herschrijft alle gecontroleerde treffers in één keer en in één stap ongedaan maken, dus Ctrl+Z draait de hele reeks terug, nooit alleen de laatste wijziging.
concept-note-template = Een herbruikbaar stuk tekst dat u invoegt in alles waaraan u werkt.
concept-note-template-more = Een leeg personageblad, een profiel van een locatie, een beat sheet: schrijf het één keer en zet het neer waar u het nodig hebt, in elk item, niet alleen in een notitie. Begin met een ingebouwde voorinstelling, importeer een .md- of .djot-bestand, of schrijf iets en kies Document ▸ Opslaan als sjabloon. Wordt in het project opgeslagen, zodat een medeauteur die het opent dezelfde set krijgt.
concept-text-replacement = Uw eigen afkortingen, automatisch uitgeschreven terwijl u typt.
concept-text-replacement-more = Stel een afkorting en de vervanging ervan in, “bvd” voor “bij voorbaat dank”, en ze wordt uitgeschreven zodra u erna een spatie of leesteken typt. Hoofdletters volgen wat u typte: begint de afkorting met een hoofdletter, dan de uitgeschreven tekst ook. Een Backspace direct na het uitschrijven draait alleen die ene vervanging terug. Per project in te schakelen, los van [slimme interpunctie](:concept-smart-punctuation).
concept-smart-punctuation = Typografie die zich aan de taal aanpast en automatisch wordt toegepast terwijl u typt: aanhalingstekens, streepjes, beletseltekens.
concept-smart-punctuation-more = Maakt van rechte aanhalingstekens typografische, zet -- om in een en-dash en --- in een em-dash, zet ... om in een beletselteken en opent, als u dat inschakelt, een alinea die begint met "- " met een dialoogstreepje. Per project in te stellen en meegenomen in het .skrib-bestand, zodat een medeauteur met dezelfde regels schrijft. Los van [uw eigen regels](:concept-text-replacement), die u zelf schrijft.
concept-export-style = Een benoemde verzameling exportkeuzes: koppen, witruimte, wat er wordt opgenomen.
concept-export-style-more = Eén stijl, hergebruikt bij elke export, in welk formaat ook. De ingebouwde stijlen zijn alleen-lezen; dupliceer er een om een vertrekpunt te krijgen dat u wel kunt wijzigen. Een van de keuzes is of er [retourmarkeringen](:concept-round-trip-marks) worden opgenomen, voor een bestand dat naar een redacteur gaat en terugkomt.
concept-round-trip-marks = Onzichtbare bladwijzers waardoor een bewerkt DOCX- of ODT-bestand bij terugkomst wordt herkend.
concept-round-trip-marks-more = Worden alleen geschreven in de twee formaten die een redacteur kan terugsturen, DOCX en ODT; elke andere export heeft ze niet. Een bladwijzer en geen aangepast attribuut: LibreOffice verwijdert aangepaste attributen bij het opslaan, maar laat een bladwijzer intact. In te schakelen per [exportstijl](:concept-export-style), onder “Versturen naar een redacteur”.
concept-status = Waar een item staat in uw eigen werkwijze: één fase tegelijk, steeds vooruit.
concept-status-more = Geen [tag](:concept-tag): een item heeft precies één status, de fasen staan op volgorde, en de lijst hoort bij het project, zodat u die kunt hernoemen, herschikken of aanvullen wanneer u maar wilt. Een sterretje naast een hoofdstuk of boek betekent dat de delen eronder niet allemaal dezelfde status hebben. In te stellen in de Inspector, op een kaart, bij een item in een doorlopende weergave, of hier.
