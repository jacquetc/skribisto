# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

wm-book = En bok i prosjektet ditt.
wm-book-more = En bok er så enkel eller som lagdelt som du vil. En kort historie kan være bare noen få [scener](:wm-scene). Et lengre prosjekt kan reire [deler](:wm-part) og [Kapittler](:wm-chapter) så dypt som du vil. Formen er din.
wm-part = En del gruppering av kapitler.
wm-part-more = En del samles [Kapittler](:wm-chapter) under én overskrift. Som et kapittel går den til neste del eller slutten av [Bok](:wm-book).
wm-chapter = Et kapittel av boken.
wm-chapter-more = Et kapittel går fra her til neste kapittel, neste [Del](:wm-part), eller [Slutten på boken](:wm-end-of-book). Det kommer i to former: en kapittelmappe som har [scener](:wm-scene), eller et flatt kapittel som bærer sin egen prosa. Velg hvilke nye kapitler som brukes i Innstillinger, under struktur.
wm-scene = En scene av prosa.
wm-scene-more = En scene holder skrivingen din. Hvert skriveelement har to sider: hovedteksten og en [Sammendrag](:wm-synopsis) å planlegge med. Begge åpne sammen i redaktøren.
wm-note = Et notat.
wm-note-more = En note holder tekst og [Sammendrag](:wm-synopsis) som a [Scene](:wm-scene), for forskning, sidestykker eller påminnelser. Alle elementer kan være en del av den samlede boken, inkludert en notat som sitter inne i en [Kapittel](:wm-chapter).
wm-note-folder = Et notat som har andre elementer.
wm-note-folder-more = En [Notat](:wm-note) som også inneholder elementer under den, så du kan gruppere relaterte notater sammen. Den holder sin egen [Sammendrag](:wm-synopsis), og elementene inne holde sin egen tekst.
wm-folder = En organiseringsmappe.
wm-folder-more = En beholder for å arrangere elementer i bindemiddelet, men passer deg. Det bærer en [Sammendrag](:wm-synopsis), men ingen prosa av sin egen. Varene inne holder skrivingen.
wm-end-of-book = Merker hvor en bok slutter.
wm-end-of-book-more = Hver bok deler en kontinuerlig liste, så en boks slutt er ikke bestemt ved å reire. Denne markøren sier [Bok](:wm-book) stopper her. Alt etter at den tilhører neste bok.
wm-story-bible-entry = En tagget, aliased notat om din historie.
wm-story-bible-entry-more = En [Notat](:wm-note) om et tegn, en plassering eller noe annet som er verdt å arkivere, opprettet med et navn, tags, alias og en startmal alle i ett trinn. Ingenting på siden skiller det etterpå, siden det forblir en vanlig notat kan du omdøbe, ommerke eller konvertere som alle andre.
wm-synopsis = Oppsummering av et skriftlig element.
wm-synopsis-more = Et sammendrag knyttet til ethvert skriveelement, vanligvis et kort avsnitt, men så lenge du vil. Bruk det til å planlegge og navigere før prosaen eksisterer. Det sitter ved siden av hovedteksten i redaktøren.

scene-break-minor = En vanlig scenepaus: et skift av tid, sted eller synsvinkel i et kapittel.
scene-break-minor-more =
    Markerer en pause *hvor du setter det*, inkludert midt i en [Scene](:wm-scene). Splitting prosa i to elementer er et organisatorisk valg, så det aldri skaper en pause på egen hånd.

    Skrivet inn i prosen som tre asterisk separert av mellomrom. Hvordan det skriver ut bestemmes av eksportstilen, ikke av hva du skriver: et Shunn manuskript setter det som en enkelt #, en trade paperback som en dinkus, og de fleste franske, tyske, spanske, russiske og italienske publisering som et bare gap uten noe merke i det hele tatt.
scene-break-major = En sterkere divisjon: en stor tid hoppe over, eller en avgjørende endring av synspunkt.
scene-break-major-more =
    Den samme ideen som en vanlig scenepause, et steg opp. Bruk den når en vanlig pause ville undervurdere hoppet.

    Skrivet som # # #, og trykt forskjellig fra det vanlige nivået av eksportstilen. Shunns standard manuskriptformat setter en enkelt # mot # # # for nøyaktig denne forskjellen. Der en tradisjon ikke har noe sterkere merke, skriver begge nivåer ut det samme.
wm-find-in-prose = Skribisto ser etter dette produktet i prosaen.
wm-find-in-prose-more = Slå på dette for tagger som navn ting du skriver om: tegn, steder, objekter. Alle elementer som bærer en slik tag er matchet mot prosaen din med tittelen og med de andre navnene du gir den, så hver [Scene](:wm-scene) lister hvem og hva som vises i den uten at du kobler noe for hånd. La det være av for tagger som beskriver et element i stedet for navn ett, som et utkast status eller en påminnelse for å sjekke kontinuitet. Hvert element det passer sammen med din [Historiebibel](:wm-story-bible).
wm-story-bible = Alle tegn, sted og annen merket oppføring, samlet i ett rutenett.
wm-story-bible-more = Åpne fra enhver notatmappes egen fane. Kortgruppe etter tag, og hvert navn hvor mange aliaser en oppføringssvar til og hvor mange scener den har dukket opp i, tegnet fra hver [Finn i prosa](:wm-find-in-prose) tag i prosjektet. Filtrer etter bok bare begrenser hvilke kort som viser; det avgjør aldri hvilke oppføringer som eksisterer.

tooltip-go-to = Hopp til ethvert element i bindemidlet (Ctrl+G)
synopsis-collapse-tooltip = Skjul Synopsis-kolonnen

wm-paratext = En tekst som ikke er en del av historien.
wm-paratext-more = Et forord, en dedikasjon, et etterord, en kolofon: å skrive som tilhører boken, men ikke kroppen. Eksportert hvor du legger den, og aldri talt i manuskriptets ordtall. Der det går opp til deg: konvensjoner varierer fra land og forlag.
wm-paratext-folder = En mappe for paratekster.
wm-paratext-folder-more = Et sted å holde forord og etterord slik at de ikke klør bindemiddel. Organisering bare: det bærer en [Sammendrag](:wm-synopsis), men legger ingenting til den eksporterte boken, ikke engang sitt eget navn.

goal-target = Hvor lenge dette stykket er ment å være.
goal-target-more = Sett et mål på noe: a [Scene](:wm-scene), a [Kapittel](:wm-chapter), a [Del](:wm-part), hele [Bok](:wm-book). Hver er sitt eget nummer og står på egen hånd, så å sette et mål på et kapittel endrer aldri bokens, og en boks mål er aldri en sum av kapitlene inne i den. Det som legger til er skriften: en beholders [Fremdrift](:goal-progress) er alt skrevet under det. Legg igjen et mål på null, og det er rett og slett ikke satt. Tallet telles i [ord eller tegn](:goal-unit) Uansett hva dette prosjektet bruker.
goal-unit = Om dette prosjektet teller i ord eller i tegn.
goal-unit-more = Et prosjektomfattende valg, fordi en lengdeenhet tilhører manuskriptet og markedet i stedet for den ene delen: Tysk og fransk publiseringsmål i tegn, japansk i 400-tegnsark, kinesisk med tusen tegn, de fleste engelskspråklige publisering med ord. Begge tallene holdes side om side og heller ikke konverteres til den andre, så bytte enhetspunktene hver [Mål](:goal-target) på den andre figuren og bytte tilbake gjenoppretter det første. Hva det ikke gjør er å konvertere noe, så mål som allerede er angitt vil trenge oppdatering for hånd.
goal-progress = Hvor mye er skrevet mot målet.
goal-progress-explained = Et stykke med egen prosa måles mot sin egen lengde; en beholder mot [Alt skrevet under det](:goal-manuscript-words). Baren varmes fra rød til grønn som skriveren kommer inn, og endres igjen når stykket har gått godt forbi det som var planlagt, som er verdt å merke når et kapittel har en lengde å holde.
goal-manuscript-words = Det som teller som manus.
goal-manuscript-words-more = Bare prosa som faktisk ville bli eksportert: [scener](:wm-scene) og [Kapittler](:wm-chapter) bærer sin egen tekst. En [Notat](:wm-note) eller en [Paratekst](:wm-paratext) er ikke en del av manuskriptet og blir aldri regnet mot ett. Alt i søppelet faller ut, og det gjør alt [ut av eksporten](:goal-exportable).
goal-exportable = Dette produktet er utelatt fra eksporten.
goal-exportable-more = Dens egen lengde viser fortsatt, fordi skrivingen fortsatt er der, men det teller mot ingen total, nøyaktig som den vil vises i ingen eksportert bok. La en beholder ute ikke la brikkene inne i den ut: hvert element svar for seg selv, som er hva - Anvend til barn - knappen ved siden av bryteren er for.
goal-distribute = Del dette målet ut over det som er inne.
goal-distribute-more = Splitter et [Bokens](:wm-book) eller [Dels](:wm-part) mål på tvers av sine nærmeste barn, slik at brikkene legger til det nøyaktig. Som standard fyller det bare de som ikke har noe mål ennå og etterlater dine egne tall alene. Hva det skriver er vanlig [mål](:goal-target), gratis å redigere etterpå og fritt å drive fra hverandre: dette er en handling, ikke en stående ordning.
goal-subtree-total = Hva målene i innsiden legger til.
goal-subtree-total-more = Et faktum, ikke et mål. Dette prosjektet behandler aldri en beholders mål som summen av dem inne i det, fordi det er slik en mappes figur ender opp med å bevege seg på egen hånd når en scene inne i det gis et tall. Denne linjen forteller deg bare hva dine egne tall kommer til, så du kan sammenligne det med beholderens [Mål](:goal-target) deg selv, eller [Del en ut](:goal-distribute).
goal-milestone = En date å nå noe innen.
goal-milestone-more = To typer. Den ene sier at [Bok](:wm-book) skulle stå i en gitt lengde innen en dato, et veipunkt på sin egen kurve. Den andre sier en bestemt [Kapittel](:wm-chapter) eller [Del](:wm-part) skulle ha nådd [sitt eget mål](:goal-target) ved da, og leser det nummeret live i stedet for å holde en kopi av den, så redigering av måloppdateringene også. Begge vises på bokens [tempoplan](:pace-plan).
pace-plan = Bokens skriveplan.
pace-plan-more = En [Mål](:goal-target) og en deadline, med dagene du skriver på, ble til hvor mye en dag behov. Den leser bokens eget mål, så å sette den her og sette den i inspektøren er det samme nummer på to steder. Veipunkter langs veien er [milepæler](:goal-milestone).

concept-tag = En fargekodet etikett du kan sette på et element, gjenbrukbar over hele prosjektet.
concept-tag-more = Tagge et element for å markere det: en status, en plassering, en tråd du sporer. Samme tag kan sitte på alle antall elementer, og ett element kan bære flere. Slå på [Finn i prosa](:wm-find-in-prose) for en tag som navngir noe i boken din, et tegn eller et sted, og Skribisto begynner å lete etter det i prosen din.
concept-label = Et kort notat du skriver under et elements tittel, for dine egne øyne.
concept-label-more = Ikke en [Etikett](:concept-tag): en etikett tilhører et element alene, vanlig tekst uten farge eller katalog bak det, noe som -st plotpunkt - eller - trenger et navn - Velg det fra elementets kontekstmeny, eller rediger den på linje i Oversiktens etikettkolonne; den viser som en liten undertittel under tittelen, i konturen og i streamen.
concept-point-of-view = Hvems øyne en scene er fortalt gjennom: en eller flere av historiens kaste.
concept-point-of-view-more = Sett i Inspector, ved siden av støpe. Velge noen ennå ikke i kastet legger dem også der. Mer enn ett synspunkt er et lovlig valg, nyttig for en delt scene eller en endring av øyne midt-bok, ikke noe Skribisto blokker. Kandidater kommer fra elementene en [Finn i prosa](:wm-find-in-prose) tag setter i historien Bibelen.
concept-epigraph = Et sitat satt i hodet på en del eller et kapittel.
concept-epigraph-more = Dets eget felt, som skiller seg fra [Kapittel](:wm-chapter)s eller [Del](:wm-part)s egen prosa; en [Scene](:wm-scene) eller en [Notat](:wm-note) bærer ingen. Skriv et som et vanlig sitat; skriv flere og hver blir sin egen blockquote, trykt som sin egen epigraf. Ordene blir aldri talt som manuskriptord.
concept-footnote = En sitering merket i prosa, skriver ut som et nummerert notat.
concept-footnote-more = Referansen er et enkelt merke inne i ordene dine, så det beveger seg og sletter nøyaktig som ordene rundt det gjør. Dens nummer er aldri lagret: Skribisto Prosjekter det ut friskt fra hvor hver referanse sitter i hele manuskriptet, så sett inn en omnummerer hver notat etter det. En fotnote egne ord telles bortsett fra manuskriptet, aldri foldet inn i det.
concept-chapter-mode = Enten et nytt kapittel er en mappe med scener eller et enkelt flatt element.
concept-chapter-mode-more = Sett i Innstillinger, under struktur. Du skriver direkte i en [Kapittel](:wm-chapter) uansett, den eneste forskjellen er om det også kan inneholde scener under den. Begge teiknkodingene kompilerer til samme bok, og du kan blande dem fritt i ett prosjekt. Fremmer et eksisterende kapittel mellom de to uten å miste et ord.
concept-comment = En note forankret til en nøyaktig strekning av prosa, vist i margen.
concept-comment-more = Det husker ordene det peker på og hva som omgir dem, ikke en fast posisjon, så det overlever vanlig redigering og en full reload. Svar under for å holde en tråd i gang. Hvis det eksakte sitatet ikke lenger kan bli funnet, kommentaren sier så i stedet for å stille drive til feil setning.
concept-backup = En punkt-i-tid kopi av hele prosjektet, holdt ved siden av det.
concept-backup-more = Tatt på en tidsplan du velger: på åpen, på nært hold, på et intervall, eller enhver blanding, med gamle kopier prunket etter alder eller ved telling. Åpning en laster det i sitt eget vindu: du kan se og til og med redigere, men bare Lagre som eller gjenopprette holder noe. Forskjellig fra en [Utgave](:concept-version), som sporer ett elements egen fortid.
concept-version = Ett elements egen fortid: hver endring i teksten eller synopsis.
concept-version-more = Bygget fra prosjektets egen lagre historie og sikkerhetskopier sammen, en oppføring per endring i stedet for en per fil. Velg en oppføring for å se hva som er endret og gjenopprette bare det elementets tekst til det; resten av prosjektet er urørt, og en sikkerhetskopi tas først. Forskjellig fra en [Sikkerhetskopi](:concept-backup), som er en kopi av alt.
concept-trash = En søppelt gjenstand er skjult, ikke flyttet: det forblir nøyaktig der det var.
concept-trash-more = Kasting bare vipper ett flagg: elementet holder sin plass i bindemiddelet, merket inaktiv i stedet for fjernet. Gjenopprett det og det høster pears på det nøyaktige punktet, uten om-filering å gjøre. Tomt papirkurv fjerner søppel gjenstander for godt, og hver av disse er et angrelig steg, som alle andre prosjektredigering.
concept-spellcheck = Understreker ord ingen installert ordbok gjenkjenner, per prosjektspråk.
concept-spellcheck-more = Hvert prosjekt har sitt eget arbeidsspråk eller språk; et element kan overstyre dem for en scene skrevet på et annet språk. Legg til et ord Skribisto vet ikke fra redigeringsens kontekstmeny, i en personlig ordbok som fører over til hvert prosjekt. Slår stavekontroll av stopper det overalt, til du slår det på igjen.
concept-search-replace = Finner et ord eller frase overalt i prosjektet, ikke bare det åpne dokumentet.
concept-search-replace-more = Ser gjennom hver scene, notat, tittel, synopsis og etikett, og gjennom kommentarer og fotnoter også. Case, helords matching og aksent matching er tre separate brytere. Erstatt alle omskriver hver revurdert kamp i ett pass og ett angretrinn, så Ctrl+Z tar tilbake hele partiet, aldri bare den siste endringen.
concept-note-template = Et gjenbrukbart stykke å skrive du setter inn i alt du jobber på.
concept-note-template-more = Et blankt tegnark, en plasseringsprofil, et slagark: skriv det én gang og slepp det inn der du trenger det, i et element, ikke bare et notat. Start fra en innebygd forhåndsinnstilling, importer en .md  eller .djot  fil, eller skriv noe og velg Dokument  - Lagre som mal. Lagret i prosjektet, så en medforfatter som åpner det får det samme settet.
concept-text-replacement = Din egen shorthand, utvides automatisk mens du skriver.
concept-text-replacement-more = Sette en utløser og dens utskiftning, -btw - for forresten - og den utvider det øyeblikket du skriver inn et rom eller tegnsetting etter det. Case følger det du skrev: kapitalisere utløseren og utvidelsen kapitaliseres også. En Backspace rett etter en utvidelse angrer bare den ene substitusjonen. Bytt på per prosjekt, forskjellig fra [smart tegnsetting](:concept-smart-punctuation).
concept-smart-punctuation = Locale-aware typografi brukes automatisk som du skriver: sitater, streker, ellipser.
concept-smart-punctuation-more = Curls rett sitater til typografiske, forvandler -- til en bindestrek og -- til en em dash, forvandler ... til en ellipse, og, hvor du slår på, åpner et avsnitt skrevet som -- med en dialog dash. Sett per prosjekt, og reiser inne i det .skrib  så en medforfatter skriver med samme regler. Skilt fra [dine egne regler](:concept-text-replacement) som du skriver deg selv.
concept-export-style = En navngitt pakke med eksportvalg: overskrifter, avstand, det som blir inkludert.
concept-export-style-more = En stil, gjenbrukt på tvers av hver eksport uansett hvilket format du velger. De innebygde stilene er skrivebare; duplisert en for å få et utgangspunkt kan du faktisk endre. Blant sine valg er om å inkludere [runde markører](:concept-round-trip-marks), for en fil bundet for en redaktør og tilbake.
concept-round-trip-marks = Usynlige bokmerker som lar en redigert DOCX eller ODT komme hjem anerkjent.
concept-round-trip-marks-more = Skrevet bare i de to formatene en redaktør kan sende tilbake, DOCX og ODT; hver annen eksport bærer ingen. Et bokmerke, ikke en egendefinert attributt: LibreOffice strimler tilpassede attributter på lagret, men etterlater et bokmerke intakt. Slås på per [eksporter stil](:concept-export-style), under -Sending til en redaktør -
concept-status = Når et element står i din egen arbeidsflyt: ett trinn om gangen, beveger seg fremover.
concept-status-more = Ikke en [Etikett](:concept-tag): et element har nøyaktig én status, trinnene er bestilt, og prosjektet eier listen, slik at du kan omdøbe, ombestille eller legge til det når du vil. En asterisk ved siden av et kapittel eller bok betyr delene under det ikke alle er enige med det. Sett det fra inspektøren, et kort, et strømelement eller her.
