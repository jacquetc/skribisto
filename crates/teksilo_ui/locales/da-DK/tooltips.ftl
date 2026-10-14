# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

# Skribisto: writing-model rich tooltips (source locale).
# Registered in tooltip_registry.rs and attached by key from the "＋ Create" /
# "Convert to" menus. The wm-*-more bodies cascade to one another via
# [label](:key) links, so this file doubles as an in-place Help doc.

wm-book = En bog i dit projekt.
wm-book-more = En bog er så enkel eller så lagdelt, som du vil. En novelle kan bestå af blot nogle få [scener](:wm-scene). Et længere værk kan indlejre [dele](:wm-part) og [kapitler](:wm-chapter) så dybt, du vil. Formen bestemmer du.
wm-part = En del, der samler kapitler.
wm-part-more = En del samler [kapitler](:wm-chapter) under én overskrift. Ligesom et kapitel varer den indtil næste del eller slutningen af [bogen](:wm-book).
wm-chapter = Et kapitel i bogen.
wm-chapter-more = Et kapitel varer herfra indtil næste kapitel, næste [del](:wm-part) eller [bogens slutning](:wm-end-of-book). Det findes i to former: en kapitelmappe, der indeholder [scener](:wm-scene), eller et fladt kapitel med sin egen tekst. Vælg under Indstillinger ▸ Struktur, hvilken form nye kapitler får.
wm-scene = En scene med tekst.
wm-scene-more = En scene rummer det, du skriver. Hvert skriveelement har to sider: hovedteksten og en [synopsis](:wm-synopsis) at planlægge med. Begge åbnes sammen i editoren.
wm-note = En note.
wm-note-more = En note har tekst og en [synopsis](:wm-synopsis) ligesom en [scene](:wm-scene), til research, sidebemærkninger eller huskesedler. Ethvert element kan indgå i den kompilerede bog, også en note, der ligger inde i et [kapitel](:wm-chapter).
wm-note-folder = En note, der indeholder andre elementer.
wm-note-folder-more = En [note](:wm-note), der også har elementer under sig, så du kan samle beslægtede noter. Den har sin egen [synopsis](:wm-synopsis), og elementerne inden i har hver deres tekst.
wm-folder = En mappe til organisering.
wm-folder-more = En beholder, så du kan arrangere elementerne i ringbindet, som det passer dig. Den har en [synopsis](:wm-synopsis), men ingen egen tekst. Det er elementerne inden i, der rummer teksten.
wm-end-of-book = Markerer, hvor en bog slutter.
wm-end-of-book-more = Alle bøger deler én sammenhængende liste, så en bogs slutning afgøres ikke af indlejring. Denne markør siger, at [bogen](:wm-book) stopper her. Alt efter den hører til den næste bog.
wm-story-bible-entry = En note om din historie, med mærker og alternative navne.
wm-story-bible-entry-more = En [note](:wm-note) om en person, et sted eller noget andet, der er værd at registrere, oprettet med navn, mærker, alternative navne og en startskabelon i ét trin. Bagefter er der intet på siden, der adskiller den: den forbliver en almindelig note, som du kan omdøbe, give nye mærker eller konvertere som alle andre.
wm-synopsis = Et resumé af et skriveelement.
wm-synopsis-more = Et resumé knyttet til et skriveelement, som regel et kort afsnit, men så langt du vil. Brug det til at planlægge og navigere, før teksten findes. Det står ved siden af hovedteksten i editoren.

scene-break-minor = Et almindeligt sceneskift: et skift i tid, sted eller synsvinkel inden for et kapitel.
scene-break-minor-more =
    Markerer et skift *dér, hvor du sætter det*, også midt i en [scene](:wm-scene). At dele tekst op i to elementer er et organisatorisk valg, så det skaber aldrig selv et skift.

    Skrives ind i teksten som tre stjerner adskilt af mellemrum. Hvordan det udskrives, afgøres af eksporttypografien, ikke af det, du skriver: et Shunn-manuskript sætter det som et enkelt #, en trade paperback som en dinkus, og de fleste franske, tyske, spanske, russiske og italienske forlag som et tomt mellemrum uden noget tegn.
scene-break-major = En kraftigere opdeling: et stort tidsspring eller et afgørende skift i synsvinkel.
scene-break-major-more =
    Samme idé som et almindeligt sceneskift, blot et trin op. Brug det, når et almindeligt skift ville undervurdere springet.

    Skrives som # # # og udskrives anderledes end det almindelige niveau, afhængigt af eksporttypografien. Shunns standardformat for manuskripter sætter et enkelt # over for # # # netop for at vise denne forskel. Hvor en tradition ikke har et kraftigere tegn, udskrives begge niveauer ens.
wm-find-in-prose = Skribisto leder efter dette element i din tekst.
wm-find-in-prose-more = Slå dette til for mærker, der navngiver ting, du skriver om: personer, steder, genstande. Ethvert element med sådan et mærke matches mod din tekst ud fra titlen og de andre navne, du giver det, så hver [scene](:wm-scene) viser, hvem og hvad der optræder i den, uden at du skal sammenkæde noget manuelt. Lad det være slået fra for mærker, der beskriver et element i stedet for at navngive det, f.eks. en udkaststatus eller en påmindelse om at tjekke kontinuiteten. Hvert element, der matches, kommer også med i din [historiebibel](:wm-story-bible).
wm-story-bible = Alle personer, steder og andre mærkede opslag, samlet i ét gitter.
wm-story-bible-more = Åbnes fra en notemappes egen fane. Kortene grupperes efter mærke, og hvert kort viser, hvor mange alternative navne et opslag har, og i hvor mange scener det er dukket op, ud fra alle mærker med [søg i teksten](:wm-find-in-prose) i projektet. Filtrering efter bog afgrænser kun, hvilke kort der vises; den afgør aldrig, hvilke opslag der findes.

tooltip-go-to = Gå til et vilkårligt element i ringbindet (Ctrl+G)
synopsis-collapse-tooltip = Skjul synopsiskolonnen

wm-paratext = En tekst, der ikke er en del af historien.
wm-paratext-more = Et forord, en dedikation, et efterord, en kolofon: tekst, der hører til bogen, men ikke til selve fortællingen. Eksporteres, hvor du end placerer den, og tælles aldrig med i manuskriptets ordantal. Hvor den skal stå, bestemmer du: konventionerne varierer fra land til land og fra forlag til forlag.
wm-paratext-folder = En mappe til paratekster.
wm-paratext-folder-more = Et sted at opbevare forord og efterord, så de ikke roder i ringbindet. Kun til organisering: den har en [synopsis](:wm-synopsis), men tilføjer intet til den eksporterede bog, ikke engang sit eget navn.

# ── Word / character targets ──────────────────────────────────────────────────
# A second cascade web beside the writing-model one. It doubles as the app's only
# explanation of how counting works, since there is no Help surface anywhere else.
goal-target = Hvor langt dette stykke skal være.
goal-target-more = Sæt et mål på hvad som helst: en [scene](:wm-scene), et [kapitel](:wm-chapter), en [del](:wm-part), hele [bogen](:wm-book). Hvert mål er sit eget tal og står for sig selv, så et mål på et kapitel ændrer aldrig bogens, og en bogs mål er aldrig summen af kapitlerne i den. Det, der lægges sammen, er det skrevne: en beholders [fremskridt](:goal-progress) er alt, hvad der er skrevet under den. Står et mål på nul, er det simpelthen ikke sat. Tallet tælles i [ord eller tegn](:goal-unit), alt efter hvad projektet bruger.
goal-unit = Om projektet tæller i ord eller i tegn.
goal-unit-more = Et valg for hele projektet, fordi en længdeenhed hører til manuskriptet og dets marked og ikke til ét afsnit: tyske og franske forlag måler i anslag, japanske i ark på 400 tegn, kinesiske i tusinder af tegn, de fleste engelsksprogede forlag i ord. Begge tal gemmes side om side, og det ene omregnes aldrig til det andet, så når du skifter enhed, peger hvert [mål](:goal-target) på det andet tal, og skifter du tilbage, får du det første igen. Der omregnes intet, så mål, der allerede er angivet, skal opdateres manuelt.
goal-progress = Hvor meget der er skrevet i forhold til målet.
goal-progress-explained = Et stykke med egen tekst måles mod sin egen længde; en beholder mod [alt, hvad der er skrevet under den](:goal-manuscript-words). Bjælken går fra rød over rav til grøn, efterhånden som teksten kommer, og skifter igen, når stykket er nået et godt stykke forbi det planlagte, hvilket er værd at lægge mærke til, når et kapitel har en længde at overholde.
goal-manuscript-words = Hvad der tæller som manuskriptet.
goal-manuscript-words-more = Kun tekst, der faktisk ville blive eksporteret: [scener](:wm-scene) og [kapitler](:wm-chapter) med egen tekst. En [note](:wm-note) eller en [paratekst](:wm-paratext) er ikke en del af manuskriptet og tælles aldrig med. Alt i papirkurven falder fra, og det samme gør alt, der er [udeladt af eksporten](:goal-exportable).
goal-exportable = Dette element er udeladt af eksporten.
goal-exportable-more = Dets egen længde vises stadig, fordi teksten stadig findes, men den tæller ikke med i nogen sum, præcis som den heller ikke kommer med i nogen eksporteret bog. Udelades en beholder, udelades elementerne inden i den ikke: hvert element står for sig selv, og det er dét, knappen »Anvend på underelementer« ved siden af kontakten er til.
goal-distribute = Fordel dette mål på det, der ligger inden i.
goal-distribute-more = Deler en [bogs](:wm-book) eller [dels](:wm-part) mål ud på dens direkte underelementer, så delene tilsammen giver præcis målet. Som standard udfylder den kun dem, der endnu ikke har et mål, og lader dine egne tal være. Det, den skriver, er almindelige [mål](:goal-target), som du frit kan redigere bagefter, og som frit kan glide fra hinanden: det er én handling, ikke en fast ordning.
goal-subtree-total = Hvad målene herinde giver tilsammen.
goal-subtree-total-more = En oplysning, ikke et mål. Projektet behandler aldrig en beholders mål som summen af målene inden i den, for det er sådan, en mappes tal ender med at flytte sig af sig selv, når en scene i den får et tal. Linjen fortæller blot, hvad dine egne tal giver, så du selv kan sammenligne med beholderens [mål](:goal-target) eller [fordele et](:goal-distribute).
goal-milestone = En dato, hvor noget skal være nået.
goal-milestone-more = Der er to slags. Den ene siger, at [bogen](:wm-book) skal have en bestemt længde på en bestemt dato, et punkt på dens egen kurve. Den anden siger, at et bestemt [kapitel](:wm-chapter) eller en bestemt [del](:wm-part) skal have nået [sit eget mål](:goal-target) til den tid, og læser tallet direkte i stedet for at gemme en kopi, så redigerer du målet, opdateres milepælen også. Begge vises på bogens [tempoplan](:pace-plan).
pace-plan = Bogens skriveplan.
pace-plan-more = Et [mål](:goal-target) og en deadline, sammen med de dage, du skriver, omregnet til hvor meget der skal skrives om dagen. Planen læser bogens eget mål, så det er det samme tal, uanset om du sætter det her eller i inspektøren. Punkter undervejs er [milepæle](:goal-milestone).

# ── Feature concepts ───────────────────────────────────────────────────────────
# A third cascade web, beside the writing-model one and the word/character-target
# one above it. Each entry here teaches one project feature that otherwise has no
# page of its own: tags, the per-row label, point of view, epigraphs, footnotes,
# chapter mode, comments, backups, versions, trash, spell-check, search and
# replace, note templates, text replacement, smart punctuation, export styles and
# the round-trip markers an export can carry. Registered the same way as the two
# webs above, and cross-linking into them wherever a concept genuinely cites one.
concept-tag = En farvekodet markering, du kan sætte på ethvert element og genbruge i hele projektet.
concept-tag-more = Giv et element et mærke for at markere det: en status, et sted, en tråd, du følger. Det samme mærke kan sidde på et vilkårligt antal elementer, og ét element kan have flere. Slå [søg i teksten](:wm-find-in-prose) til for et mærke, der navngiver noget i din bog, en person eller et sted, så begynder Skribisto at lede efter det i din tekst.
concept-label = En kort note, du skriver under et elements titel, til dig selv.
concept-label-more = Ikke et [mærke](:concept-tag): en etiket hører til ét enkelt element, ren tekst uden farve eller katalog bag sig, f.eks. »1. plotpunkt« eller »mangler et navn«. Angiv den fra elementets genvejsmenu, eller redigér den direkte i oversigtens kolonne Etiket; den vises som en lille undertitel under titlen, både i dispositionen og i forløbet.
concept-point-of-view = Gennem hvis øjne en scene fortælles: en eller flere af historiens personer.
concept-point-of-view-more = Angives i inspektøren ved siden af persongalleriet. Vælger du en, der endnu ikke er med i persongalleriet, tilføjes vedkommende også dér. Mere end én synsvinkel er et lovligt valg, nyttigt i en fælles scene eller ved et skift af øjne midt i bogen, og ikke noget, Skribisto forhindrer. Kandidaterne kommer fra de elementer, som et mærke med [søg i teksten](:wm-find-in-prose) placerer i din historiebibel.
concept-epigraph = Et citat, der står i spidsen for en del eller et kapitel.
concept-epigraph-more = Et felt for sig, adskilt fra [kapitlets](:wm-chapter) eller [delens](:wm-part) egen tekst; en [scene](:wm-scene) eller en [note](:wm-note) har intet. Skriv det som et almindeligt citat; skriver du flere, bliver hvert sit eget blokcitat og udskrives som sit eget motto. Ordene tælles aldrig med som manuskriptord.
concept-footnote = En henvisning markeret i teksten, der udskrives som en nummereret note.
concept-footnote-more = Henvisningen er et enkelt mærke inde i dine ord, så den flyttes og slettes præcis som ordene omkring den. Nummeret gemmes aldrig: Skribisto beregner det på ny ud fra, hvor hver henvisning står i hele manuskriptet, så indsætter du én, omnummereres alle noter efter den. En fodnotes egne ord tælles adskilt fra manuskriptet og lægges aldrig sammen med det.
concept-chapter-mode = Om et nyt kapitel er en mappe med scener eller et enkelt fladt element.
concept-chapter-mode-more = Angives under Indstillinger ▸ Struktur. Du skriver direkte i et [kapitel](:wm-chapter) uanset hvad; den eneste forskel er, om det også kan indeholde scener under sig. Begge former kompileres til den samme bog, og du kan frit blande dem i ét projekt. Konvertér til skifter et eksisterende kapitel mellem de to uden at miste et eneste ord.
concept-comment = En note forankret til et bestemt tekststykke, vist i margenen.
concept-comment-more = Den husker de ord, den peger på, og det, der omgiver dem, ikke en fast position, så den overlever almindelig redigering og en fuld genindlæsning. Svar nedenunder for at holde en tråd i gang. Kan det nøjagtige citat ikke længere findes, siger kommentaren det i stedet for i stilhed at glide over til den forkerte sætning.
concept-backup = En kopi af hele projektet fra et bestemt tidspunkt, gemt ved siden af det.
concept-backup-more = Tages efter en plan, du selv vælger: ved åbning, ved lukning, med et interval eller en blanding, og gamle kopier tyndes ud efter alder eller antal. Åbner du en, indlæses den i sit eget vindue: du kan se og endda redigere, men kun Gem som eller Gendan beholder noget. Adskiller sig fra en [version](:concept-version), som følger ét elements egen fortid.
concept-version = Ét elements egen fortid: alle ændringer i dets tekst eller synopsis.
concept-version-more = Bygges ud fra projektets egen gemmehistorik og dets sikkerhedskopier tilsammen, én post pr. ændring i stedet for én pr. fil. Vælg en post for at se, hvad der er ændret, og gendan kun det elements tekst til den; resten af projektet røres ikke, og der tages først en sikkerhedskopi. Adskiller sig fra en [sikkerhedskopi](:concept-backup), som er en kopi af det hele.
concept-trash = Et element i papirkurven skjules, det flyttes ikke: det bliver præcis, hvor det var.
concept-trash-more = At lægge noget i papirkurven ændrer kun ét flag: elementet beholder sin plads i ringbindet, markeret som inaktivt i stedet for at blive fjernet. Gendanner du det, dukker det op igen på præcis samme sted, uden at du skal sortere det på plads. Tøm papirkurven fjerner elementerne i papirkurven for altid, og hver af disse handlinger er ét trin, der kan fortrydes, som enhver anden ændring i projektet.
concept-spellcheck = Understreger ord, som ingen installeret ordbog kender, pr. projektsprog.
concept-spellcheck-more = Hvert projekt har sit eget arbejdssprog eller sine egne sprog; et element kan tilsidesætte dem for en scene, der er skrevet på et andet sprog. Tilføj et ord, Skribisto ikke kender, fra editorens genvejsmenu til en personlig ordbog, som gælder i alle projekter. Slår du stavekontrollen fra, stopper den overalt, indtil du slår den til igen.
concept-search-replace = Finder et ord eller en vending overalt i projektet, ikke kun i det åbne dokument.
concept-search-replace-more = Gennemsøger alle scener, noter, titler, synopser og etiketter, og også kommentarer og fodnoter. Store/små bogstaver, hele ord og accenter er tre separate kontakter. Erstat alle omskriver alle gennemsete forekomster i ét gennemløb og ét fortrydelsestrin, så Ctrl+Z tager hele omgangen tilbage, aldrig kun den sidste ændring.
concept-note-template = Et genbrugeligt tekststykke, du kan indsætte i alt, hvad du arbejder på.
concept-note-template-more = Et tomt personark, en stedbeskrivelse, en beat sheet: skriv det én gang, og indsæt det, hvor du har brug for det, i ethvert element, ikke kun en note. Start fra en indbygget forudindstilling, importér en .md- eller .djot-fil, eller skriv noget og vælg Dokument ▸ Gem som skabelon. Gemmes i projektet, så en medforfatter, der åbner det, får de samme.
concept-text-replacement = Dine egne forkortelser, der automatisk udfoldes, mens du skriver.
concept-text-replacement-more = Angiv en forkortelse og dens erstatning, f.eks. »bl.a.« for »blandt andet«, så udfoldes den, så snart du skriver et mellemrum eller et tegn efter den. Store og små bogstaver følger det, du skrev: begynder forkortelsen med stort, gør udfoldningen det også. Et tryk på Backspace lige efter en udfoldning fortryder kun den ene erstatning. Slås til pr. projekt og adskiller sig fra [smart tegnsætning](:concept-smart-punctuation).
concept-smart-punctuation = Sprogtilpasset typografi, der anvendes automatisk, mens du skriver: anførselstegn, tankestreger, ellipser.
concept-smart-punctuation-more = Laver lige anførselstegn om til typografiske, -- om til en kort tankestreg og --- om til en lang tankestreg, ... om til en ellipse, og, hvor du slår det til, begynder et afsnit, der indledes med »- «, med en replikstreg. Et afsnit, der skrives som "- ", "* " eller "1. ", bliver til en liste. Angives pr. projekt og følger med i .skrib-filen, så en medforfatter skriver efter de samme regler. Adskiller sig fra [dine egne regler](:concept-text-replacement), som du selv skriver.
concept-export-style = Et navngivet sæt eksportvalg: overskrifter, afstand, hvad der medtages.
concept-export-style-more = Én typografi, der genbruges i alle eksporter, uanset hvilket format du vælger. De indbyggede typografier er skrivebeskyttede; duplikér en for at få et udgangspunkt, du faktisk kan ændre. Blandt valgene er, om der skal medtages [returmarkører](:concept-round-trip-marks), til en fil, der skal til en redaktør og tilbage igen.
concept-round-trip-marks = Usynlige bogmærker, der gør, at en redigeret DOCX- eller ODT-fil genkendes, når den kommer tilbage.
concept-round-trip-marks-more = Skrives kun i de to formater, en redaktør kan sende tilbage, DOCX og ODT; alle andre eksporter har ingen. Et bogmærke, ikke en egen attribut: LibreOffice fjerner egne attributter ved gemning, men lader et bogmærke være. Slås til pr. [eksporttypografi](:concept-export-style) under »Sende til en redaktør«.
concept-status = Hvor et element står i din egen arbejdsgang: én fase ad gangen, fremad.
concept-status-more = Ikke et [mærke](:concept-tag): et element har præcis én status, faserne har en rækkefølge, og projektet ejer listen, så du kan omdøbe, omordne eller tilføje, når du vil. En stjerne ved siden af et kapitel eller en bog betyder, at delene under det ikke alle er enige med det. Angives fra inspektøren, et kort, et element i forløbet eller her.
