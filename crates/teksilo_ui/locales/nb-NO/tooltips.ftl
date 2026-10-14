# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

wm-book = En bok i prosjektet ditt.
wm-book-more = En bok kan være så enkel eller så lagdelt du vil. En novelle kan bestå av bare noen få [scener](:wm-scene). Et lengre verk kan neste [deler](:wm-part) og [kapitler](:wm-chapter) så dypt du vil. Formen bestemmer du.
wm-part = En del som samler kapitler.
wm-part-more = En del samler [kapitler](:wm-chapter) under én overskrift. Som et kapittel varer den til neste del eller til slutten av [boken](:wm-book).
wm-chapter = Et kapittel i boken.
wm-chapter-more = Et kapittel varer herfra til neste kapittel, neste [del](:wm-part) eller [slutten av boken](:wm-end-of-book). Det finnes i to former: en kapittelmappe som inneholder [scener](:wm-scene), eller et flatt kapittel med sin egen tekst. Velg hvilken form nye kapitler skal ha i Innstillinger, under Struktur.
wm-scene = En scene med tekst.
wm-scene-more = En scene inneholder skrivingen din. Hvert skriveelement har to sider: hovedteksten og en [synopsis](:wm-synopsis) til planlegging. Begge åpnes sammen i redigeringsvinduet.
wm-note = Et notat.
wm-note-more = Et notat har tekst og en [synopsis](:wm-synopsis), som en [scene](:wm-scene), til research, sidesprang eller påminnelser. Ethvert element kan være en del av den kompilerte boken, også et notat som ligger inne i et [kapittel](:wm-chapter).
wm-note-folder = Et notat som inneholder andre elementer.
wm-note-folder-more = Et [notat](:wm-note) som også har elementer under seg, så du kan samle notater som hører sammen. Det har sin egen [synopsis](:wm-synopsis), og elementene inni har sin egen tekst.
wm-folder = En mappe til organisering.
wm-folder-more = En beholder der du kan ordne elementene i permen slik det passer deg. Den har en [synopsis](:wm-synopsis), men ingen egen tekst. Det er elementene inni som inneholder skrivingen.
wm-end-of-book = Markerer hvor en bok slutter.
wm-end-of-book-more = Alle bøkene deler én sammenhengende liste, så slutten på en bok bestemmes ikke av nestingen. Denne markøren sier at [boken](:wm-book) slutter her. Alt etter den hører til neste bok.
wm-story-bible-entry = Et notat om historien din, med stikkord og alternative navn.
wm-story-bible-entry-more = Et [notat](:wm-note) om en person, et sted eller noe annet som fortjener en plass i arkivet, opprettet med navn, stikkord, alternative navn og en startmal i ett og samme trinn. Ingenting på siden skiller det ut etterpå: det forblir et vanlig notat som du kan gi nytt navn, nye stikkord eller konvertere som alle andre.
wm-synopsis = Et sammendrag av et skriveelement.
wm-synopsis-more = Et sammendrag knyttet til et skriveelement, vanligvis et kort avsnitt, men så langt du vil. Bruk det til å planlegge og finne frem før teksten finnes. Det står ved siden av hovedteksten i redigeringsvinduet.

scene-break-minor = Et vanlig sceneskift: et skifte i tid, sted eller synsvinkel innenfor et kapittel.
scene-break-minor-more =
    Markerer et skift *der du setter det*, også midt i en [scene](:wm-scene). Å dele tekst i to elementer er et organisatorisk valg, så det lager aldri et skift av seg selv.

    Skrives inn i teksten som tre stjerner med mellomrom mellom. Hvordan det skrives ut, bestemmes av eksportstilen, ikke av det du skriver: et Shunn-manus setter det som én #, en vanlig paperback som en dinkus, og det meste av fransk, tysk, spansk, russisk og italiensk forlagsvirksomhet som et rent mellomrom uten noe tegn i det hele tatt.
scene-break-major = Et sterkere skille: et stort tidshopp eller et avgjørende skifte i synsvinkel.
scene-break-major-more =
    Samme idé som et vanlig sceneskift, ett trinn opp. Bruk det når et vanlig skift ville undervurdere spranget.

    Skrives som # # #, og skrives ut annerledes enn det vanlige nivået av eksportstilen. Shunns standard manusformat setter én # mot # # # nettopp for dette skillet. Der en tradisjon ikke har noe sterkere tegn, skrives begge nivåene ut likt.
wm-find-in-prose = Skribisto leter etter dette elementet i teksten din.
wm-find-in-prose-more = Slå dette på for stikkord som navngir ting du skriver om: personer, steder, gjenstander. Ethvert element med et slikt stikkord blir sammenlignet med teksten din etter tittel og de andre navnene du gir det, så hver [scene](:wm-scene) viser hvem og hva som forekommer i den, uten at du trenger å koble noe for hånd. La det være av for stikkord som beskriver et element i stedet for å navngi noe, som en utkaststatus eller en påminnelse om å sjekke kontinuiteten. Hvert element det finner, kommer også med i [historiebibelen](:wm-story-bible).
wm-story-bible = Alle personer, steder og andre oppføringer med stikkord, samlet i ett rutenett.
wm-story-bible-more = Åpnes fra fanen til en hvilken som helst notatmappe. Kortene grupperes etter stikkord, og hvert av dem viser hvor mange alternative navn en oppføring har, og i hvor mange scener den har dukket opp, hentet fra alle [finn i teksten](:wm-find-in-prose)-stikkord i prosjektet. Filtrering etter bok begrenser bare hvilke kort som vises; den avgjør aldri hvilke oppføringer som finnes.

tooltip-go-to = Gå til et hvilket som helst element i permen (Ctrl+G)
synopsis-collapse-tooltip = Skjul synopsisspalten

wm-paratext = En tekst som ikke er en del av historien.
wm-paratext-more = Et forord, en dedikasjon, et etterord, en kolofon: tekst som hører til boken, men ikke til selve fortellingen. Eksporteres der du plasserer den, og telles aldri med i manusets antall ord. Hvor den skal stå, bestemmer du: konvensjonene varierer fra land til land og fra forlag til forlag.
wm-paratext-folder = En mappe til paratekster.
wm-paratext-folder-more = Et sted å samle forord og etterord så de ikke roter til permen. Bare til organisering: den har en [synopsis](:wm-synopsis), men legger ingenting til den eksporterte boken, ikke engang sitt eget navn.

goal-target = Hvor lang denne delen skal være.
goal-target-more = Sett et mål på hva som helst: en [scene](:wm-scene), et [kapittel](:wm-chapter), en [del](:wm-part), hele [boken](:wm-book). Hvert mål er sitt eget tall og står for seg selv, så et mål på et kapittel endrer aldri bokens mål, og en boks mål er aldri summen av kapitlene i den. Det som summeres, er skrivingen: [fremdriften](:goal-progress) til en beholder er alt som er skrevet under den. Lar du et mål stå på null, er det rett og slett ikke satt. Tallet telles i [ord eller tegn](:goal-unit), alt etter hva prosjektet bruker.
goal-unit = Om dette prosjektet teller i ord eller i tegn.
goal-unit-more = Et valg for hele prosjektet, fordi en lengdeenhet hører til manuset og markedet det er skrevet for, ikke til én del av det: tyske og franske forlag måler i anslag, japanske i ark på 400 tegn, kinesiske i tusen tegn, og de fleste engelskspråklige forlag i ord. Begge tallene holdes side om side, og det ene regnes aldri om til det andre, så når du bytter enhet, peker hvert [mål](:goal-target) på det andre tallet, og bytter du tilbake, får du det første igjen. Det som ikke skjer, er noen omregning, så mål som allerede er lagt inn, må oppdateres for hånd.
goal-progress = Hvor mye som er skrevet, sammenlignet med målet.
goal-progress-explained = En del med egen tekst måles mot sin egen lengde; en beholder mot [alt som er skrevet under den](:goal-manuscript-words). Stolpen går fra rødt via gult til grønt etter hvert som teksten kommer, og endres igjen når delen har gått godt forbi det som var planlagt, noe som er greit å legge merke til når et kapittel har en lengde å holde seg innenfor.
goal-manuscript-words = Hva som regnes som manus.
goal-manuscript-words-more = Bare tekst som faktisk ville blitt eksportert: [scener](:wm-scene) og [kapitler](:wm-chapter) med egen tekst. Et [notat](:wm-note) eller en [paratekst](:wm-paratext) er ikke en del av manuset og telles aldri med. Alt i papirkurven faller bort, og det samme gjør alt som er [utelatt fra eksporten](:goal-exportable).
goal-exportable = Dette elementet er utelatt fra eksporten.
goal-exportable-more = Lengden vises fortsatt, fordi teksten fortsatt er der, men den teller ikke med i noen total, akkurat som den ikke kommer med i noen eksportert bok. Å utelate en beholder utelater ikke delene inni den: hvert element står for seg selv, og det er det knappen «Bruk på underelementer» ved siden av bryteren er til for.
goal-distribute = Fordel dette målet på det som ligger inni.
goal-distribute-more = Deler målet til en [bok](:wm-book) eller en [del](:wm-part) mellom de nærmeste underelementene, så delene til sammen blir nøyaktig målet. Som standard fylles bare de som ikke har noe mål ennå, og dine egne tall blir stående. Det som skrives, er vanlige [mål](:goal-target), som du fritt kan redigere etterpå og som fritt kan skille lag: dette er én handling, ikke en fast ordning.
goal-subtree-total = Hva målene her inne utgjør til sammen.
goal-subtree-total-more = Et faktum, ikke et mål. Dette prosjektet regner aldri målet til en beholder som summen av målene inni den, for det er slik tallet til en mappe ender med å endre seg av seg selv når en scene inni den får et tall. Denne linjen forteller deg bare hva dine egne tall blir til sammen, så du selv kan sammenligne med beholderens [mål](:goal-target), eller [fordele et](:goal-distribute).
goal-milestone = En dato noe skal være nådd innen.
goal-milestone-more = To typer. Den ene sier at [boken](:wm-book) skal ha nådd en viss lengde innen en dato, et veipunkt på sin egen kurve. Den andre sier at et bestemt [kapittel](:wm-chapter) eller en bestemt [del](:wm-part) skal ha nådd [sitt eget mål](:goal-target) innen da, og leser det tallet direkte i stedet for å ta en kopi, så når du endrer målet, oppdateres milepælen også. Begge vises i bokens [tempoplan](:pace-plan).
pace-plan = Bokens skriveplan.
pace-plan-more = Et [mål](:goal-target) og en frist, sammen med dagene du skriver, omgjort til hvor mye som trengs per dag. Den leser bokens eget mål, så å sette det her og å sette det i inspektøren er det samme tallet to steder. Veipunkter underveis er [milepæler](:goal-milestone).

concept-tag = Et fargekodet stikkord du kan sette på et hvilket som helst element, og bruke om igjen i hele prosjektet.
concept-tag-more = Gi et element et stikkord for å markere det: en status, et sted, en tråd du følger. Det samme stikkordet kan stå på så mange elementer du vil, og ett element kan ha flere. Slå på [finn i teksten](:wm-find-in-prose) for et stikkord som navngir noe i boken din, en person eller et sted, så begynner Skribisto å lete etter det i teksten din.
concept-label = En kort merknad du skriver under tittelen til et element, bare for deg selv.
concept-label-more = Ikke et [stikkord](:concept-tag): en etikett hører til ett enkelt element, ren tekst uten farge eller katalog bak, for eksempel «1. vendepunkt» eller «trenger et navn». Sett den fra elementets hurtigmeny, eller rediger den direkte i Etikett-kolonnen i oversikten; den vises som en liten undertittel under tittelen, både i disposisjonen og i strømmen.
concept-point-of-view = Hvem sine øyne en scene fortelles gjennom: én eller flere av personene i historien.
concept-point-of-view-more = Settes i inspektøren, ved siden av persongalleriet. Velger du noen som ikke er i persongalleriet ennå, blir de lagt til der også. Mer enn én synsvinkel er et gyldig valg, nyttig for en delt scene eller et skifte av øyne midt i boken, ikke noe Skribisto hindrer. Kandidatene kommer fra elementene et [finn i teksten](:wm-find-in-prose)-stikkord legger i historiebibelen din.
concept-epigraph = Et sitat som står øverst i en del eller et kapittel.
concept-epigraph-more = Et eget felt, atskilt fra teksten til [kapittelet](:wm-chapter) eller [delen](:wm-part); en [scene](:wm-scene) eller et [notat](:wm-note) har ingen. Skriv ett som et vanlig sitat; skriver du flere, blir hvert av dem et eget blokksitat, skrevet ut som en egen epigraf. Ordene telles aldri som ord i manuset.
concept-footnote = En henvisning markert i teksten, skrevet ut som en nummerert note.
concept-footnote-more = Henvisningen er ett enkelt merke inne i ordene dine, så den flyttes og slettes nøyaktig som ordene rundt den. Nummeret lagres aldri: Skribisto regner det ut på nytt fra hvor hver henvisning står i hele manuset, så setter du inn én, nummereres alle notene etter den på nytt. En fotnotes egne ord telles atskilt fra manuset og regnes aldri inn i det.
concept-chapter-mode = Om et nytt kapittel er en mappe med scener eller ett enkelt, flatt element.
concept-chapter-mode-more = Settes i Innstillinger, under Struktur. Du skriver direkte i et [kapittel](:wm-chapter) uansett; den eneste forskjellen er om det i tillegg kan inneholde scener under seg. Begge formene kompileres til den samme boken, og du kan blande dem fritt i ett prosjekt. Konverter til gjør om et eksisterende kapittel mellom de to uten å miste et eneste ord.
concept-comment = En merknad forankret til et bestemt stykke tekst, vist i margen.
concept-comment-more = Den husker ordene den viser til og det som står rundt dem, ikke en fast posisjon, så den overlever vanlig redigering og en fullstendig ny innlasting. Svar under for å holde en tråd i gang. Hvis det nøyaktige sitatet ikke lenger finnes, sier kommentaren fra om det i stedet for å gli stille over til feil setning.
concept-backup = En kopi av hele prosjektet fra et bestemt tidspunkt, lagret ved siden av det.
concept-backup-more = Tas etter en plan du velger: ved åpning, ved lukking, med jevne mellomrom, eller en kombinasjon, og gamle kopier ryddes bort etter alder eller antall. Åpner du en, lastes den inn i sitt eget vindu: du kan se og til og med redigere, men bare Lagre som eller Gjenopprett tar vare på noe. Ikke det samme som en [versjon](:concept-version), som følger ett elements egen historikk.
concept-version = Ett elements egen historikk: alle endringer i teksten eller synopsisen.
concept-version-more = Bygget fra prosjektets egen lagringshistorikk og sikkerhetskopiene til sammen, med én oppføring per endring i stedet for én per fil. Velg en oppføring for å se hva som ble endret, og gjenopprett bare det elementets tekst til den; resten av prosjektet røres ikke, og det tas en sikkerhetskopi først. Ikke det samme som en [sikkerhetskopi](:concept-backup), som er en kopi av alt.
concept-trash = Et element i papirkurven er skjult, ikke flyttet: det blir stående nøyaktig der det var.
concept-trash-more = Å legge noe i papirkurven endrer bare ett flagg: elementet beholder plassen sin i permen, merket som inaktivt i stedet for fjernet. Gjenoppretter du det, dukker det opp igjen på nøyaktig samme sted, uten at noe må sorteres på nytt. Tøm papirkurven fjerner elementene i papirkurven for godt, og hver av disse handlingene er ett trinn som kan angres, som enhver annen endring i prosjektet.
concept-spellcheck = Understreker ord som ingen installert ordbok kjenner igjen, for hvert av prosjektets språk.
concept-spellcheck-more = Hvert prosjekt har ett eller flere arbeidsspråk; et element kan overstyre dem for en scene skrevet på et annet språk. Legg til et ord Skribisto ikke kjenner, fra hurtigmenyen i redigeringsvinduet, i en personlig ordbok som følger med til alle prosjekter. Slår du av stavekontrollen, stanser den overalt til du slår den på igjen.
concept-search-replace = Finner et ord eller en frase overalt i prosjektet, ikke bare i det åpne dokumentet.
concept-search-replace-more = Søker gjennom alle scener, notater, titler, synopser og etiketter, og i kommentarer og fotnoter også. Store/små bokstaver, hele ord og aksenter er tre separate brytere. Erstatt alle skriver om alle gjennomgåtte treff i én omgang og ett angretrinn, så Ctrl+Z tar tilbake hele omgangen, aldri bare den siste endringen.
concept-note-template = En tekst du kan bruke om igjen og sette inn i hva som helst du arbeider med.
concept-note-template-more = Et tomt personskjema, en stedsprofil, en scenedisposisjon: skriv den én gang og sett den inn der du trenger den, i et hvilket som helst element, ikke bare et notat. Start fra en innebygd forhåndsinnstilling, importer en .md- eller .djot-fil, eller skriv noe og velg Dokument ▸ Lagre som mal. Lagres i prosjektet, så en medforfatter som åpner det, får de samme malene.
concept-text-replacement = Dine egne forkortelser, utvidet automatisk mens du skriver.
concept-text-replacement-more = Angi en forkortelse og hva den skal erstattes med, «f.eks» for «for eksempel», og den utvides i det øyeblikket du skriver et mellomrom eller et skilletegn etter den. Store og små bokstaver følger det du skrev: skriver du forkortelsen med stor forbokstav, får også utvidelsen stor forbokstav. Et trykk på Backspace rett etter en utvidelse angrer bare den ene erstatningen. Slås på for hvert prosjekt, atskilt fra [smart tegnsetting](:concept-smart-punctuation).
concept-smart-punctuation = Språktilpasset typografi som brukes automatisk mens du skriver: anførselstegn, tankestreker, ellipser.
concept-smart-punctuation-more = Gjør rette anførselstegn om til typografiske, -- om til tankestrek og --- om til lang tankestrek, ... om til ellipse, og, der du slår det på, begynner et avsnitt skrevet som «- » med en replikkstrek. Et avsnitt som skrives som "- ", "* " eller "1. ", blir en liste. Settes for hvert prosjekt og følger med i .skrib-filen, så en medforfatter skriver med de samme reglene. Atskilt fra [dine egne regler](:concept-text-replacement), som du skriver selv.
concept-export-style = En navngitt samling eksportvalg: overskrifter, avstand, hva som tas med.
concept-export-style-more = Én stil, brukt om igjen ved all eksport uansett hvilket format du velger. De innebygde stilene er skrivebeskyttede; dupliser en for å få et utgangspunkt du faktisk kan endre. Blant valgene er om [returmarkører](:concept-round-trip-marks) skal tas med, for en fil som skal til en redaktør og tilbake.
concept-round-trip-marks = Usynlige bokmerker som gjør at en redigert DOCX- eller ODT-fil blir gjenkjent når den kommer tilbake.
concept-round-trip-marks-more = Skrives bare inn i de to formatene en redaktør kan sende tilbake, DOCX og ODT; all annen eksport er uten. Et bokmerke, ikke et egendefinert attributt: LibreOffice fjerner egendefinerte attributter ved lagring, men lar bokmerker være i fred. Slås på for hver [eksportstil](:concept-export-style), under «Sende til en redaktør».
concept-status = Hvor et element står i din egen arbeidsflyt: ett stadium om gangen, fremover.
concept-status-more = Ikke et [stikkord](:concept-tag): et element har nøyaktig én status, stadiene står i en bestemt rekkefølge, og listen hører til prosjektet, så du kan gi stadiene nye navn, endre rekkefølgen eller legge til flere når du vil. En stjerne ved siden av et kapittel eller en bok betyr at delene under ikke alle stemmer overens med det. Sett statusen fra inspektøren, et kort, et element i strømmen, eller her.
