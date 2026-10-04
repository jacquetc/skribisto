# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

wm-book = En bok i ditt projekt.
wm-book-more = En bok är så enkel eller så skiktad som du vill. En kort historia kan vara bara några [scener](:wm-scene). Ett längre projekt kan bo [delar delar](:wm-part) och [Kapitel](:wm-chapter) så djupt som du vill. Formen är din.
wm-part = En del gruppering kapitel.
wm-part-more = En del samlar [Kapitel](:wm-chapter) under en rubrik. Som ett kapitel går den fram till nästa del eller slutet av [Bok](:wm-book).
wm-chapter = Ett kapitel i boken.
wm-chapter-more = Ett kapitel går härifrån fram till nästa kapitel, nästa [Del](:wm-part), eller [änden av boken](:wm-end-of-book). Det kommer i två former: en kapitelmapp som innehåller [scener](:wm-scene), eller ett platt kapitel som bär sin egen prosa. Välj vilken nya kapitel som används i Inställningar, under struktur.
wm-scene = En scen av prosa.
wm-scene-more = En scen håller ditt skrivande. Varje skrivobjekt har två sidor: huvudtexten och en [Synopsis](:wm-synopsis) att planera med. Båda öppnar tillsammans i redaktören.
wm-note = En anteckning.
wm-note-more = En anteckning håller text och en [Synopsis](:wm-synopsis), som en [Scen](:wm-scene), för forskning, åt sidan eller påminnelser. Alla objekt kan vara en del av den sammanställda boken, inklusive en anteckning som sitter i en [Kapitel](:wm-chapter).
wm-note-folder = En anteckning som innehåller andra objekt.
wm-note-folder-more = En [Anteckning](:wm-note) som också innehåller objekt under den, så att du kan gruppera relaterade anteckningar tillsammans. Det håller sin egen [Synopsis](:wm-synopsis), och objekten i hålla sin egen text.
wm-folder = En organiserande mapp.
wm-folder-more = En behållare för att ordna objekt i bindaren passar dig dock. Det bär en [Synopsis](:wm-synopsis) men ingen prosa av sig själv.
wm-end-of-book = Marks där en bok slutar.
wm-end-of-book-more = Varje bok delar en kontinuerlig lista, så en boks slut beslutas inte genom att häcka. Denna markör säger att [Bok](:wm-book) stannar här. Allt efter det hör till nästa bok.
wm-story-bible-entry = En taggad, aliased note om din historia.
wm-story-bible-entry-more = En [Anteckning](:wm-note) om en karaktär, en plats eller något annat värt arkivering, skapad med ett namn, taggar, alias och en startmall allt i ett steg. Ingenting på sidan skiljer det efteråt, eftersom det förblir en vanlig anteckning du kan byta namn, retag eller konvertera som någon annan.
wm-synopsis = En sammanfattning av ett skrivobjekt.
wm-synopsis-more = En sammanfattning bifogad till något skrivobjekt, vanligtvis en kort paragraf men så länge du vill. Använd den för att planera och navigera innan prosa finns. Det sitter bredvid huvudtexten i redaktören.

scene-break-minor = En vanlig scenbrytning: en tidsförskjutning, plats eller synpunkt i ett kapitel.
scene-break-minor-more =
    Markerar en paus *där du lägger den*, inklusive i mitten av en [Scen](:wm-scene). Splitting prosa i två objekt är ett organisatoriskt val, så det skapar aldrig en paus på egen hand.

    Skrivs in i prosa som tre asterisker separerade av utrymmen. Hur det utskrifter bestäms av exportstilen, inte av vad du skriver: en Shunn manuskript sätter det som en enda #, en handelspapper som en dinkus, och de flesta franska, tyska, spanska, ryska och italienska publicering som en nakna klyfta utan markering alls.
scene-break-major = En starkare uppdelning: en stor tid hoppa, eller en avgörande förändring av synpunkten.
scene-break-major-more =
    Samma idé som en vanlig scenbrytning, ett steg upp. Använd den när en vanlig paus skulle underskatta hoppet.

    Skrivs som # # #, och tryckt annorlunda från den vanliga nivån genom exportstilen. Shunn standard manuskriptformat sätter ett enda # mot # # # # för exakt denna distinktion. Där en tradition inte har något starkare märke, skriver båda nivåerna samma.
wm-find-in-prose = Skribisto letar efter detta objekt i din prosa.
wm-find-in-prose-more = Slå på detta för taggar som heter saker du skriver om: tecken, platser, objekt. Alla objekt som bär en sådan tagg matchas mot din prosa med sin titel och med de andra namnen du ger den, så varje [Scen](:wm-scene) listar vem och vad som visas i det utan att du länkar något för hand. Lämna det för taggar som beskriver ett objekt snarare än namn ett, till exempel ett utkast status eller en påminnelse om att kontrollera kontinuitet. Varje objekt som den matchar går också med din [Berättelsebibel](:wm-story-bible).
wm-story-bible = Varje karaktär, plats och annan taggad post, samlad i ett rutnät.
wm-story-bible-more = Öppna från alla anteckningar mappens egen flik. Kortgrupp med tag, och varje namn hur många aliases en post svarar på och hur många scener den har dykt upp, hämtad från varje [Hitta i prose](:wm-find-in-prose) tag i projektet. Filtrera med bok endast smala vilka kort visar; det bestämmer aldrig vilka poster som finns.

tooltip-go-to = Hoppa till något objekt i bindemedel (Ctrl+G)
synopsis-collapse-tooltip = Dölj synopsis-kolumnen

wm-paratext = En text som inte är en del av historien.
wm-paratext-more = Ett förord, ett engagemang, ett efterord, en colophon: skriva som hör till boken men inte till sin kropp. Exporteras var du än lägger den, och aldrig räknas i manuskriptets ordräkning. Där det går är upp till dig: konventioner skiljer sig från land och förläggare.
wm-paratext-folder = En mapp för paratexter.
wm-paratext-folder-more = Någonstans för att hålla förord och efterord så att de inte rör bindemedel. Organisera endast: det bär en [Synopsis](:wm-synopsis) men lägger ingenting till den exporterade boken, inte ens sitt eget namn.

goal-target = Hur länge detta stycke är tänkt att vara.
goal-target-more = Ställ in ett mål på allt: en [Scen](:wm-scene), en [Kapitel](:wm-chapter), en [Del](:wm-part)Hela [Bok](:wm-book)Var och en är sitt eget nummer och står på egen hand, så att sätta ett mål på ett kapitel ändrar aldrig sin bok, och en boks mål är aldrig en summa av kapitlen inuti det. [Framsteg](:goal-progress) är allt skrivet under det. Lämna ett mål på noll och det är helt enkelt inte inställt. Numret räknas in [Ord eller karaktärer](:goal-unit)Oavsett vilket projektet använder.
goal-unit = Oavsett om det här projektet räknas med ord eller karaktärer.
goal-unit-more = Ett projektomfattande val, eftersom en längd enhet tillhör manuskriptet och dess marknad snarare än till en sektion: tyska och franska publiceringsåtgärden i skyltar, japanska i 400-karaktärsblad, kinesiska av de tusen tecknen, de flesta engelskspråkiga publicering i ord. Båda siffrorna hålls sida vid sida och aldrig omvandlas till den andra, så byta enhetspunkter varje [Mål](:goal-target) på den andra siffran och växla tillbaka återställer den första. Vad det inte gör är att konvertera någonting, så mål som redan har angetts för hand.
goal-progress = Hur mycket är skrivet mot målet.
goal-progress-explained = En bit med prosa av sin egen mäts mot sin egen längd; en behållare mot [Allt skrivet under det](:goal-manuscript-words). Baren värmer från rött genom bärnsten till grönt när skrivandet kommer in, och ändrar igen när biten har gått bra förbi vad som planerades, vilket är värt att märka när ett kapitel har en längd att hålla sig till.
goal-manuscript-words = Vad räknas som manuskriptet.
goal-manuscript-words-more = Endast prosa som faktiskt skulle exporteras: [scener](:wm-scene) och [Kapitel](:wm-chapter) som bär sin egen text. En [Anteckning](:wm-note) eller en [Paratext](:wm-paratext) är inte en del av manuskriptet och räknas aldrig mot en. Allt i papperskorgen faller ut, och så gör allt [lämnas utanför exporten](:goal-exportable).
goal-exportable = Detta objekt lämnas utanför exporten.
goal-exportable-more = Dess egen längd visar fortfarande, eftersom skrivandet fortfarande finns där, men det räknas inte mot totalt, precis som det kommer att visas i ingen exporterad bok. Att lämna en behållare lämnar inte bitarna inuti det: varje objekt svarar för sig själv, vilket är vad "Apply to children" -knappen bredvid växeln är för.
goal-distribute = Dela detta mål ut över vad som finns inuti.
goal-distribute-more = Splits en [Bokens](:wm-book) eller [Delens](:wm-part) mål över sina omedelbara barn så bitarna lägga upp till det exakt. Som standard det fyller bara de utan mål ännu och lämnar dina egna siffror ensam. Vad det skriver är vanliga [mål](:goal-target), fri att redigera efteråt och fri att glida isär: detta är en åtgärd, inte en stående arrangemang.
goal-subtree-total = Vad målen inuti lägger till.
goal-subtree-total-more = Ett faktum, inte ett mål. Detta projekt behandlar aldrig en behållares mål som summan av de inuti det, eftersom det är hur en mapp siffra hamnar på egen hand när en scen inuti det ges ett nummer. Denna linje berättar helt enkelt vad dina egna nummer kommer till, så du kan jämföra det med behållarens [Mål](:goal-target) själv, eller [dela ut en](:goal-distribute).
goal-milestone = Ett datum för att nå något av.
goal-milestone-more = Två typer. En säger att [Bok](:wm-book) bör stå vid en viss längd efter ett datum, en waypoint på sin egen kurva. Den andra säger att en viss [Kapitel](:wm-chapter) eller [Del](:wm-part) borde ha nått [sitt eget mål](:goal-target) då, och läser det numret lever snarare än att hålla en kopia av det, så redigera målet uppdaterar milstolpen också. Båda visas på bokens [Pace Plan](:pace-plan).
pace-plan = Bokens skrivschema.
pace-plan-more = En [Mål](:goal-target) och en tidsfrist, med de dagar du skriver på, förvandlades till hur mycket en dag behöver. Det läser bokens eget mål, så att ställa in den här och ställa in den i inspektören är samma nummer på två ställen. Waypoints längs vägen är [Milstolpar](:goal-milestone).

concept-tag = En färgkodad etikett som du kan sätta på något objekt, återanvändbart över hela projektet.
concept-tag-more = Tag ett objekt för att markera det: en status, en plats, en tråd du spårar. Samma tagg kan sitta på ett antal objekt, och ett objekt kan bära flera. Slå på [Hitta i prose](:wm-find-in-prose) för en tagg som namnger något i din bok, en karaktär eller en plats, och Skribisto börjar leta efter det i din prosa.
concept-label = En kort anteckning du skriver under en artikel titel, för dina egna ögon.
concept-label-more = Inte en [Tagg](:concept-tag): en etikett tillhör ett objekt ensam, vanlig text utan färg eller katalog bakom det, något som "1st plot point" eller "behöver ett namn". Ställ in det från objektets kontextmeny, eller redigera det inline i översiktens etikkolumn; det visar som en liten undertext under titeln, i konturen och i strömmen lika.
concept-point-of-view = Vars ögon en scen berättas genom: en eller flera av din berättelses gjutning.
concept-point-of-view-more = Beläget i inspektören, bredvid gjutningen. Välja någon ännu inte i gjutningen lägger dem där också. Mer än en synpunkt är ett juridiskt val, användbart för en delad scen eller en förändring av ögonen i mitten av boken, inte något Skribisto block. Kandidater kommer från objekten en [Hitta i prose](:wm-find-in-prose) tag sätter i din historia bibel.
concept-epigraph = Ett citat som anges i huvudet på en del eller ett kapitel.
concept-epigraph-more = Dess eget fält, separat från [Kapitel](:wm-chapter) eller [Del](:wm-part) egna prosa; en [Scen](:wm-scene) eller en [Anteckning](:wm-note) bär ingen. Skriv en som en vanlig citat; skriv flera och varje blir sin egen blockquote, tryckt som sin egen epigraf. Dess ord räknas aldrig som manuskriptord.
concept-footnote = En citat markerad i prosa, tryckning som en numrerad anteckning.
concept-footnote-more = Referensen är ett enda märke inuti dina ord, så det rör sig och raderar exakt som orden runt det gör. Dess nummer lagras aldrig: Skribisto Projekt det ut fräscht från var varje referens sitter i hela manuskriptet, så införa en ryggrad varje anteckning efter det. En fotnot egna ord räknas bort från manuskriptet, aldrig vikas in i det.
concept-chapter-mode = Oavsett om ett nytt kapitel är en mapp av scener eller ett enda platt objekt.
concept-chapter-mode-more = Ställ in Inställningar, under struktur. Du skriver direkt till en [Kapitel](:wm-chapter) antingen sätt; den enda skillnaden är om det också kan innehålla scener under det. Båda kodningarna sammanställer samma bok, och du kan blanda dem fritt i ett projekt. Främja konverterar ett befintligt kapitel mellan de två utan att förlora ett ord.
concept-comment = En anteckning förankrad till en exakt sträcka av prosa, som visas i marginalen.
concept-comment-more = Det kommer ihåg orden det pekar på och vad omger dem, inte en fast position, så det överlever vanlig redigering och en full reload. Svara under för att hålla en tråd går. Om det exakta citatet inte längre kan hittas, säger kommentaren så snarare än tyst glider till fel mening.
concept-backup = En punkt-i-tid kopia av hela projektet, hålls bredvid det.
concept-backup-more = Tas på ett schema du väljer: på öppet, på nära håll, på ett intervall eller någon blandning, med gamla kopior beskärda efter ålder eller genom räkning. Öppna en laddar den i sitt eget fönster: du kan se och till och med redigera, men bara Spara som eller återställa håller något. Olika från en [version version version version](:concept-version), som spårar ett objekts eget förflutna.
concept-version = Ett objekts eget förflutna: varje förändring av texten eller synopsisen.
concept-version-more = Byggd från projektets egen spara historia och dess säkerhetskopior tillsammans, en post per förändring snarare än en per fil. Välj en post för att se vad som ändrats och återställa endast objektets text till det; resten av projektet är orörd, och en säkerhetskopia tas först. Olika från en [Säkerhetskopia](:concept-backup), som är en kopia av allt.
concept-trash = Ett papperskorsat objekt är dolt, inte flyttat: det stannar precis där det var.
concept-trash-more = Trashing bara vänder en flagga: objektet håller sin plats i bindaren, markerad inaktiv snarare än bort. Återställ den och det återkommer på den exakta platsen, utan återfilering att göra. Empty Trash tar bort skräpposter för gott, och var och en av dessa är ett ogiltigt steg, som alla andra projektredigering.
concept-spellcheck = Understryker ord som ingen installerad ordbok erkänner, per projektspråk.
concept-spellcheck-more = Varje projekt har sitt eget arbetsspråk eller språk; ett objekt kan åsidosätta dem för en scen skriven på en annan tunga. Lägg till ett ord Skribisto vet inte från redaktörens kontextmeny, till en personlig ordbok som bär över till varje projekt. Turning stavkontroll av stoppar den överallt, tills du slår tillbaka den.
concept-search-replace = Hittar ett ord eller fras överallt i projektet, inte bara det öppna dokumentet.
concept-search-replace-more = Tittar igenom varje scen, notera, titel, synopsis och etikett, och genom kommentarer och fotnoter också. Fall, helordsmatchning och accentmatchning är tre separata switchar. Byt alla omskriver varje granskad match i ett pass och ett undo steg, så Ctrl + Z tar tillbaka hela satsen, aldrig bara den sista förändringen.
concept-note-template = En återanvändbar bit av att skriva in dig i allt du arbetar på.
concept-note-template-more = Ett tomt teckenblad, en platsprofil, ett slagplåt: skriv det en gång och släpp det där du behöver det, i något objekt, inte bara en anteckning. Börja från en inbyggd förinställd, importera en .md eller .djot-fil eller skriv något och välj Dokument - Spara som mall. Förvarad i projektet, så en medförfattare som öppnar den får samma uppsättning.
concept-text-replacement = Din egen shorthand, expanderade automatiskt när du skriver.
concept-text-replacement-more = Ställ en trigger och dess ersättare, "btw" för "förresten", och det expanderar det ögonblick du skriver ett utrymme eller skiljetecken efter det. Case följer vad du skrev: kapitalisera triggern och expansionen kapitaliserar också. En Backspace strax efter en expansion ångrar bara den ena substitutionen. Switched på per projekt, distinkt från [Smart Punctuation](:concept-smart-punctuation).
concept-smart-punctuation = Locale-aware typografi tillämpas automatiskt när du skriver: citat, dashes, ellipser.
concept-smart-punctuation-more = Curls raka citat till typografiska, förvandlas - till en en dash och - till en em dash, förvandlas ... till en ellipsis, och, där du slår på, öppnar en paragraf som är skriven som "-" med en dialog streck. Ställ in per projekt, och reser in i .skrib så en medförfattare skriver med samma regler. Distinct från [Dina egna regler](:concept-text-replacement), som du skriver själv.
concept-export-style = Ett namngivet paket av exportval: rubriker, avstånd, vad som ingår.
concept-export-style-more = En stil, återanvänds över varje export oavsett format du väljer. De inbyggda stilarna är lättlästa; duplicera en för att få en utgångspunkt du faktiskt kan ändra. Bland dess val är att inkludera [rundtur markörer](:concept-round-trip-marks), för en fil som är bunden för en redaktör och rygg.
concept-round-trip-marks = Osynliga bokmärken som låter en redigerad DOCX eller ODT komma hem igen.
concept-round-trip-marks-more = Skriven endast i de två formaten som en redaktör kan skicka tillbaka, DOCX och ODT; varje annan export bär ingen. Ett bokmärke, inte ett anpassat attribut: LibreOffice remsor anpassade attribut på spara men lämnar ett bokmärke intakt. Turned per [exportstil](:concept-export-style), under "Sändning till en redaktör".
concept-status = Där ett objekt står i ditt eget arbetsflöde: ett steg i taget, framåt.
concept-status-more = Inte en [Tagg](:concept-tag): ett objekt har exakt en status, stadierna beställs, och projektet äger listan, så att du kan byta namn, ordna eller lägga till det när du vill. En asterisk bredvid ett kapitel eller bok betyder delarna nedan inte alla håller med det. Ställ in den från inspektören, ett kort, en ström objekt, eller här.
