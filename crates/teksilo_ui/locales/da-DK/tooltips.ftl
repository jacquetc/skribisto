# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

wm-book = En bog i dit projekt.
wm-book-more = En bog er så enkel eller så lagdelt som du vil. En kort historie kan være blot et par [scener](:wm-scene). Et længere projekt kan rede [dele](:wm-part) og [kapitler](:wm-chapter) så dybt som du ønsker. Formen er din.
wm-part = En del, der grupperer kapitler.
wm-part-more = En del samler [kapitler](:wm-chapter) under en overskrift. Som et kapitel, det kører indtil næste del eller slutningen af [Bog](:wm-book).
wm-chapter = Et kapitel i bogen.
wm-chapter-more = Et kapitel løber herfra indtil næste kapitel, det næste [Del](:wm-part), eller [slutningen af bogen](:wm-end-of-book). Det kommer i to former: et kapitel mappe, der holder [scener](:wm-scene), eller et fladt kapitel, der bærer sin egen prosa. Vælg hvilke et nye kapitler, der bruges i Indstillinger, under Struktur.
wm-scene = En scene af prosa.
wm-scene-more = En scene holder din skrivning. Hvert skrivende element har to sider: den vigtigste tekst og en [Resumé](:wm-synopsis) til at planlægge med. Begge åbne sammen i editoren.
wm-note = En seddel.
wm-note-more = En note holder tekst og en [Resumé](:wm-synopsis), som en [Scene](:wm-scene), til forskning, sider, eller påmindelser. Enhver vare kan være en del af den udarbejdede bog, herunder en note, der sidder inde i en [Kapitel](:wm-chapter).
wm-note-folder = En note, der indeholder andre elementer.
wm-note-folder-more = En [Note](:wm-note), der også indeholder elementer under det, så du kan gruppere relaterede noter sammen. Det holder sin egen [Resumé](:wm-synopsis), og de elementer inde holde deres egen tekst.
wm-folder = En organiserende mappe.
wm-folder-more = En beholder til at arrangere elementer i bindemidlet dog passer dig. Det bærer en [Resumé](:wm-synopsis), men ingen prosa af sin egen. De elementer inde holde skriften.
wm-end-of-book = Marks hvor en bog ender.
wm-end-of-book-more = Hver bog deler en kontinuerlig liste, så en bog er slut ikke afgøres af reder. Denne markør siger, at [Bog](:wm-book) stopper her. Alt efter det tilhører den næste bog.
wm-story-bible-entry = En mærket, aliased note om din historie.
wm-story-bible-entry-more = En [Note](:wm-note) om en karakter, en placering, eller noget andet værd arkivering, skabt med et navn, tags, aliaser og en startskabelon alt i ét trin. Intet på siden adskiller det efterfølgende, da det forbliver en almindelig note, du kan omdøbe, ommærke eller konvertere som ethvert andet.
wm-synopsis = En oversigt over et skriftligt element.
wm-synopsis-more = Et resumé knyttet til et hvilket som helst skrivepunkt, normalt et kort afsnit, men så længe du vil. Brug det til at planlægge og navigere før prosen eksisterer. Det sidder ved siden af hovedteksten i editoren.

scene-break-minor = En almindelig scene break: et skift af tid, sted eller synspunkt inden for et kapitel.
scene-break-minor-more =
    Marker en pause *hvor du sætter det*, herunder i midten af en [Scene](:wm-scene). Opdeling prose i to punkter er et organisatorisk valg, så det aldrig skaber en pause på egen hånd.

    Skrivet ind i prosen som tre asterisker adskilt af mellemrum. Hvordan det udskriver afgøres af eksport stil, ikke af, hvad du skriver: et Shunn manuskript sætter det som en enkelt #, en trade paperback som en dinkus, og de fleste franske, tyske, spanske, russiske og italienske udgivelse som en bare hul uden mærke overhovedet.
scene-break-major = En stærkere opdeling: en stor tidsforskydning eller en afgørende holdningsændring.
scene-break-major-more =
    Den samme idé som en almindelig scene break, et skridt op. Brug det, når en almindelig pause ville undervurdere springet.

    Udskrives som # # #, og udskrives anderledes end det almindelige niveau ved eksport stil. Shunn Means standard manuskript format sætter en enkelt # mod # # # for præcis denne sondring. Hvor en tradition ikke har nogen stærkere mærke, begge lag udskrive det samme.
wm-find-in-prose = Skribisto leder efter dette punkt i din prosa.
wm-find-in-prose-more = Slå dette til for tags, der navngiver ting, du skriver om: tegn, steder, objekter. Enhver vare, der bærer et sådant mærke, matches mod din prosa ved sin titel og ved de andre navne, du giver det, så hver [Scene](:wm-scene) lister hvem og hvad der vises i det uden du forbinder noget i hånden. Lad det være ud for tags, der beskriver et element snarere end navn én, såsom et udkast til status eller en påmindelse om at kontrollere kontinuitet. Hvert element, det matcher, slutter også din [Historiebibel](:wm-story-bible).
wm-story-bible = Hver karakter, sted og andre mærkede indgang, samlet i et gitter.
wm-story-bible-more = Åbn fra alle noter mappe Buds egen fane. Kort gruppe ved tag, og hver navne, hvor mange aliaser en indgang svar på, og hvor mange scener det har dukket op i, trukket fra hver [finde i prose](:wm-find-in-prose) tag i projektet. Filtrering ved bog kun indsnævrer hvilke kort viser; det aldrig afgøre, hvilke indgange findes.

tooltip-go-to = Hop til alle emner i bindemidlet (Ctrl + G)
synopsis-collapse-tooltip = Skjul synopsis kolonnen

wm-paratext = En tekst, der ikke er en del af historien.
wm-paratext-more = Et forord, en dedikation, et efterord, en kolofon: skriftligt, der tilhører bogen, men ikke dens krop. Eksporteret uanset hvor du lægger det, og aldrig talt i manuskriptet 's ordoptælling. Hvor det går er op til dig: konventioner forskellige land og udgiver.
wm-paratext-folder = En mappe til paratekter.
wm-paratext-folder-more = Et eller andet sted at holde forord og efterord, så de ikke blande bindemidlet. Organiserer kun: det bærer en [Resumé](:wm-synopsis), men tilføjer intet til den eksporterede bog, ikke engang sit eget navn.

goal-target = Hvor længe det her stykke skal være.
goal-target-more = Sæt et mål på noget som helst: en [Scene](:wm-scene), en [Kapitel](:wm-chapter), en [Del](:wm-part), hele [Bog](:wm-book). Hver er sit eget antal og står på sin egen, så indstilling af et mål på et kapitel aldrig ændrer sin bog Buds, og en bog Buds mål er aldrig en sum af kapitlerne inde i det. Hvad der tilføje op er skriften: en container Buds [Fremskridt](:goal-progress) er alt skrevet under det. Efterlad et mål på nul, og det er simpelthen ikke sat. Tallet i [ord eller tegn](:goal-unit), uanset hvad dette projekt bruger.
goal-unit = Om dette projekt tæller med ord eller med bogstaver.
goal-unit-more = Et projekt-bredt valg, fordi en længde enhed tilhører manuskriptet og dens marked snarere end den ene sektion: tysk og fransk udgivelse foranstaltning i tegn, japansk i 400- karakter ark, kinesisk af de tusind tegn, de fleste engelsk- sprog udgivelse i ord. Begge numre er holdt side om side, og ingen af dem er nogensinde konverteret til den anden, så skifte enhed peger hver [Mål](:goal-target) på den anden figur og skifte tilbage genopretter den første. Hvad det ikke gør, er at konvertere noget, så mål, der allerede er indtastet skal opdateres ved hånden.
goal-progress = Hvor meget er der skrevet, imod målet.
goal-progress-explained = Et stykke med prosa af sin egen måles i forhold til sin egen længde; en beholder mod [Alt skrevet under det](:goal-manuscript-words). Baren varmer fra rød gennem rav til grøn som skriften kommer ind, og ændrer sig igen, når stykket er løbet langt forbi, hvad der var planlagt, hvilket er værd at bemærke, når et kapitel har en længde at holde sig til.
goal-manuscript-words = Hvad der tæller som manuskriptet.
goal-manuscript-words-more = Kun prose, der rent faktisk ville blive eksporteret: [scener](:wm-scene) og [kapitler](:wm-chapter) bærer deres egen tekst. A [Note](:wm-note) eller en [Paratekst](:wm-paratext) er ikke en del af manuskriptet og er aldrig tælles mod én. Alt i papirkurven falder ud, og så gør noget [udeladt fra eksport](:goal-exportable).
goal-exportable = Denne post er udeladt fra eksporten.
goal-exportable-more = Dens egen længde stadig viser, fordi skriften er der stadig, men det tæller mod ingen total, præcis som det vil vises i ingen eksporteret bog. Efterlade en beholder ud ikke efterlader brikkerne inde i det ud: hvert element svar for sig selv, som er, hvad den gælder for børn knap ved siden af kontakten er for.
goal-distribute = Del dette mål ud på tværs af, hvad der er indeni.
goal-distribute-more = Splitter en [boghandler](:wm-book) eller [dele af dyr](:wm-part) mål på tværs af sine umiddelbare børn, så brikkerne tilføje op til det præcist. Som standard det fylder kun dem, der ikke har noget mål endnu og efterlader dine egne numre alene. Hvad det skriver er almindelige [mål](:goal-target), gratis at redigere bagefter og frit at drive fra hinanden: dette er en handling, ikke en stående ordning.
goal-subtree-total = Hvad målene inde i det giver.
goal-subtree-total-more = Et faktum, ikke et mål. Dette projekt aldrig behandler en container Buds mål som summen af dem inde i det, fordi det er, hvordan en mappe Buds figur ender med at bevæge sig på egen hånd, når en scene inde i det er givet et nummer. Denne linje blot fortæller dig, hvad dine egne numre kommer til, så du kan sammenligne det med container Buds [Mål](:goal-target) selv, eller [Del en ud](:goal-distribute).
goal-milestone = En date at nå frem til.
goal-milestone-more = To slags. Den ene siger, at [Bog](:wm-book) bør stå på en given længde ved en dato, et waypoint på sin egen kurve. Den anden siger en bestemt [Kapitel](:wm-chapter) eller [Del](:wm-part) skulle have nået [sit eget mål](:goal-target) på det tidspunkt, og læser, at tallet lever snarere end at holde en kopi af det, så redigering af målet opdaterer milepæl også. Begge vises på bogen bl. [temaplan](:pace-plan).
pace-plan = Bogen skriver tidsplan.
pace-plan-more = En [Mål](:goal-target) og en deadline, med de dage, du skriver på, forvandlet til, hvor meget en dag behov. Det læser bogen 's eget mål, så sætte det her og indstille det i Inspektør er det samme antal i to steder. Waypoints undervejs er [mileston](:goal-milestone).

concept-tag = En farvekodet etiket, du kan sætte på ethvert element, genbruges på tværs af hele projektet.
concept-tag-more = Tag et element for at markere det: en status, en placering, en tråd du sporer. Det samme mærke kan sidde på et vilkårligt antal elementer, og et element kan bære flere. Tænd for [finde i prose](:wm-find-in-prose) for et mærke, der navngiver noget i din bog, en karakter eller et sted, og Skribisto begynder at lede efter det i din prose.
concept-label = En kort bemærkning du skriver under en genstand titel, for dine egne øjne.
concept-label-more = Ikke en [Mærke](:concept-tag): en etiket tilhører et element alene, almindelig tekst uden farve eller katalog bag det, noget i retning af 1. plot punkt skal have et navn. Sæt det fra elementet Buds kontekst menu, eller redigere det inline i Oversigt Label kolonne; det viser som en lille undertitel under titlen, i omridset og i strømmen.
concept-point-of-view = Hvis øjne en scene er fortalt gennem: en eller flere af dine historier cast.
concept-point-of-view-more = At vælge nogen endnu ikke i gipsen tilføjer dem der også. Mere end et synspunkt er et juridisk valg, nyttigt for en delt scene eller en ændring af øjne midbog, ikke noget Skribisto blokke. Kandidater kommer fra de elementer en [finde i prose](:wm-find-in-prose) tag sætter i din historie bibel.
concept-epigraph = Et citat sat i spidsen for en del eller et kapitel.
concept-epigraph-more = Dens eget felt, adskilt fra [Kapitel](:wm-chapter) eller [Del](:wm-part) eller egen prosa; en [Scene](:wm-scene) eller en [Note](:wm-note) bærer ingen. Skriv en som en almindelig citat; skrive flere og hver bliver sin egen blokkort, trykt som sin egen epigraph. Dens ord er aldrig tælles som manuskript ord.
concept-footnote = En henvisning markeret i prosen, udskrives som en nummereret note.
concept-footnote-more = Referencen er et enkelt mærke inde i dine ord, så det bevæger sig og sletter præcis som ordene omkring det gør. Dens antal er aldrig gemt: Skribisto Projekter det ud frisk fra hvor hver reference sidder i hele manuskriptet, så indsætte en renumbers hver note efter det. En fodnote egne ord tælles ud fra manuskriptet, aldrig foldet ind i det.
concept-chapter-mode = Hvorvidt et nyt kapitel er en mappe med scener eller en enkelt flad genstand.
concept-chapter-mode-more = Sæt i Indstillinger, under Struktur. Du skriver direkte ind i en [Kapitel](:wm-chapter) begge veje; den eneste forskel er, om det også kan indeholde scener under det. Begge indkodninger kompilerer til den samme bog, og du kan blande dem frit i et projekt. Promote konverterer et eksisterende kapitel mellem de to uden at miste et ord.
concept-comment = En note forankret til en nøjagtig strækning af prosa, vist i margenen.
concept-comment-more = Det husker de ord, det peger på, og hvad der omgiver dem, ikke en fast position, så det overlever almindelig redigering og en fuld genindlæsning. Svar nedenunder for at holde en tråd i gang. Hvis det nøjagtige citat ikke længere kan findes, kommentaren siger det snarere end lydløst drivende til den forkerte sætning.
concept-backup = En point- in- time kopi af hele projektet, holdt ved siden af det.
concept-backup-more = Taget på en tidsplan, du vælger: på åben, på tæt, på et interval, eller en blanding, med gamle kopier beskæres efter alder eller ved optælling. Åbning en indlæser det i sit eget vindue: du kan se og endda redigere, men kun Gem som eller Gendanne holder noget. Forskellig fra en [version](:concept-version), som sporer en vare, der er egen fortid.
concept-version = Et element skal have sin egen fortid: hver ændring i sin tekst eller synopsis.
concept-version-more = Bygget fra projektets egen gemme historie og dens sikkerhedskopier sammen, en post per ændring i stedet for en per fil. Vælg en post for at se, hvad der ændret og gendanne kun dette element Buds tekst til det; resten af projektet er uberørt, og en sikkerhedskopi er taget først. Forskellige fra en [Sikkerhedskopi](:concept-backup), som er en kopi af alt.
concept-trash = En skrottet genstand er skjult, ikke flyttet: det forbliver præcis, hvor det var.
concept-trash-more = Trashing kun flipper et flag: elementet holder sin plads i bindemidlet, markeret inaktiv i stedet for fjernet. Gendanne det, og det dukker op på det nøjagtige sted, uden at gen- arkivering til at gøre. Tomme papirkurven fjerner trashede elementer for god, og hver af disse er et undoable skridt, som ethvert andet projekt redigere.
concept-spellcheck = Understøtter ord ingen installeret ordbog anerkender, per projektsprog.
concept-spellcheck-more = Hvert projekt har sit eget arbejdssprog eller sprog; et punkt kan tilsidesætte dem til en scene skrevet på en anden tunge. Tilføj et ord Skribisto ikke kender fra editor-kontekstmenuen, i en personlig ordbog, der bærer over til hvert projekt. Tænder stavekontrol stopper det overalt, indtil du tænder det igen.
concept-search-replace = Finder et ord eller en sætning overalt i projektet, ikke kun det åbne dokument.
concept-search-replace-more = Ser gennem hver scene, note, titel, synopsis og etiket, og gennem kommentarer og fodnoter også. Case, hel- ord matching og accent matching er tre separate kontakter. Erstat alle omskriver hver revideret match i et pass og et fortryde trin, så Ctrl + Z tager tilbage hele partiet, aldrig kun den sidste ændring.
concept-note-template = Et genanvendeligt stykke skrift du indsætter i noget du arbejder på.
concept-note-template-more = Et tomt tegnark, en lokaliseringsprofil, et beat ark: skrive det en gang og slippe det i hvor som helst du har brug for det, i ethvert element, ikke kun en note. Start fra en bygning-in forudindstillet, importere en .md eller .djot fil, eller skrive noget og vælg Document Save som skabelon. Gemt i projektet, så en co-forfatter, der åbner det får det samme sæt.
concept-text-replacement = Din egen kortslutning, udvidet automatisk som du skriver.
concept-text-replacement-more = Sæt en udløser og dens erstatning, og det udvider det øjeblik, du skriver en plads eller punktuering efter det. Case følger, hvad du skrev: kapitalisere udløseren og udvidelsen kapitaliserer også. En Backspace lige efter en udvidelse ødelægger kun, at en substitution. Tændt på per projekt, adskilt fra [Smart punktuering](:concept-smart-punctuation).
concept-smart-punctuation = Lokal-opmærksomme typografi anvendes automatisk som du skriver: citater, dash, ellipser.
concept-smart-punctuation-more = Krøller lige citater til typografiske dem, forvandler -- til en en dash og --- til en em dash, forvandler... til en ellipsis, og hvor du tænder det, åbner et afsnit skrevet som - med en dialog dash. Set per projekt, og rejser inden for .skrib, så en co-forfatter skriver med de samme regler. Distinkt fra [dine egne regler](:concept-text-replacement), som du skriver dig selv.
concept-export-style = Et navngivet bundt af eksportvalg: overskrifter, afstand, hvad der bliver inkluderet.
concept-export-style-more = En stil, genbruges på tværs af hver eksport uanset format, du vælger. Bygge- in stilarter er read- only; duplikat en for at få et udgangspunkt, du kan faktisk ændre. Blandt sine valg er, om at inkludere [rundkøremarkeringer](:concept-round-trip-marks), for en fil, der er bundet til en editor og tilbage.
concept-round-trip-marks = Usynlige bogmærker, der lader en redigeret DOCX eller ODT komme hjem anerkendt.
concept-round-trip-marks-more = Skrevet kun i de to formater en editor kan sende tilbage, DOCX og ODT; hver anden eksport bærer ingen. Et bogmærke, ikke en brugerdefineret attribut: LibreOffice strimler brugerdefinerede attributter på gemme, men efterlader et bogmærke intakt. Drejes på pr [eksport- stil](:concept-export-style), under Budding til en editor.
concept-status = Hvor en genstand står i din egen arbejdsgang: et trin ad gangen, bevæger sig fremad.
concept-status-more = Ikke en [Mærke](:concept-tag): et element har præcis én status, faserne er bestilt, og projektet ejer listen, så du kan omdøbe, ombestille eller tilføje til det, når du har lyst. En stjerne ved siden af et kapitel eller en bog betyder, at delene nedenfor det ikke alle er enige med det. Sæt det fra Inspektør, et kort, en stream element, eller her.
