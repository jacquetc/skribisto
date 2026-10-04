# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

menu-work = &Projekti
menu-new-work = Uu&si projekti
menu-open-work = Avaa p&rojekti…
menu-new-window = &Uusi ikkuna
menu-import-from = Tu&o kohteesta
menu-create-from = L&uo kohteesta
menu-import-plume = &Plume Creator (.plume)…
menu-import-document = &Asiakirjat (Markdown, Word, ODT)...
menu-import-manuskript = &Manuskript (.msk)…
menu-export = &Vie
menu-export-book = Vie kirja
menu-export-part = Vie osa
menu-export-chapter = Vie luku
menu-export-scene = Vie kohtaus
menu-export-note = Vie muistiinpano
menu-export-paratext = Vie parateksti
menu-export-folder = Vie kansio
menu-export-choose = Valitse…
menu-export-none = Avaa asiakirja vietäväksi
menu-save = &Tallenna
menu-save-as-file = Tallenna &yhtenä tiedostona…
menu-save-as-folder = Talle&nna kansiona…
menu-backup = Varmuuskop&ioi nyt
menu-close-work = Sul&je projekti
menu-welcome = T&ervetuloa…
menu-settings = &Asetukset
menu-quit = &Lopeta

menu-view = &Näytä
menu-outline = &Rakenne
menu-search = &Hae projektista
menu-timeline = Pa&laa ajassa taaksepäin
menu-search-preview = Ha&un esikatselu
menu-fullscreen = &Koko näyttö
menu-focus-mode = H&äiriötön tila
menu-format = M&uotoilu
menu-scene-break = Lis&ää kohtauskatkos
menu-major-scene-break = Lisää me&rkittävä kohtauskatkos

menu-go = &Siirry
menu-go-next-scene = Se&uraava kohtaus
menu-go-prev-scene = E&dellinen kohtaus
menu-go-next-chapter = &Seuraava luku
menu-go-prev-chapter = &Edellinen luku
menu-go-next-note = Seu&raava huomautus
menu-go-prev-note = Ede&llinen huomautus

menu-tools = &Työkalut
menu-help = &Ohje

ctx-add = &Lisää
ctx-new-item = &Uusi kohde
ctx-new-folder = &Uusi kansio
ctx-rename = &Nimeä uudelleen
ctx-duplicate = &Monista
ctx-import-here = &Tuo tähän…
ctx-indent = L&isää sisennystä
ctx-outdent = &Vähennä sisennystä
ctx-trash = Sii&rrä roskakoriin
ctx-open-to-side = &Avaa sivulle
ctx-open = &Avaa
ctx-reveal-in-outline = &Paljasta
ctx-move-up = &Siirrä ylös
ctx-move-down = &Siirrä alas

ctx-tab-close = &Sulje
ctx-tab-close-others = S&ulje muut
ctx-tab-close-all = Su&lje kaikki
ctx-tab-open-to-side = &Avaa sivulle
ctx-tab-open-in-main = &Avaa pääpaneelissa
ctx-tab-move-to-side = S&iirrä sivulle
ctx-tab-move-to-main = S&iirry pääpaneeliin
ctx-tab-move-to-new-window = Sii&rrä uuteen ikkunaan
ctx-tab-move-window-unsaved = Tallenna tämä projekti ensin: toinen ikkuna avautuu tiedostolle levyllä
ctx-tab-pin = &Pin tämän välilehden
ctx-tab-unpin = A&vaa tämä välilehti
tab-pinned-tooltip = Pinned: "Sulje muut" ja "Sulje kaikki" Jätä se auki

create-book = Kirja
create-part = Osa
create-chapter = Luku
create-scene = Kohtaus
create-note = Muistiinpano
create-note-folder = Muistiinpanokansio
create-folder = Kansio
create-paratext = Parateksti
create-paratext-folder = Paratekstikansio
create-book-end = Kirjan loppu
create-story-bible-entry = Tarinan tietopankin merkintä…
type-book-start = Kirjan alku
type-text = Teksti

new-item-book = Uusi kirja
new-item-part = Uusi osa
new-item-chapter = Uusi luku
new-item-note = Uusi muistiinpano
new-item-note-folder = Uusi muistilappukansio
new-item-folder = Uusi kansio
new-item-paratext = Uusi parateksti
new-item-paratext-folder = Uusi paratext-kansio
new-item-story-bible-entry = Uusi tarinan Raamattu

placement-inside = Sisään
placement-after = jälkeen
placement-after-parent = kun vanhempi
placement-top-level = ylin taso


ctx-promote = Muunna t&yypiksi
promote-chapter-folder = Lukukansio
promote-flat-chapter = Yksittäinen luku
promote-lossy-title = Tekstiä ei voi säilyttää missään
promote-lossy-text = A { $target } ei ole missään säilyttää näitä: { $kinds }. Siirrä tai tyhjennä, että teksti ensin, sitten muuntaa.
content-scene-text = Kohtausteksti
content-note-text = Huomautusteksti
content-book-title = Kirjan nimi
content-book-subtitle = Kirjan tekstitys
content-part-title = Osan otsikko
content-chapter-title = Luvun otsikko
content-epigraph-text = Motto
content-paratext-text = Parateksti
promote-blocked-title = Luku ei ole tyhjä
promote-blocked-text = { $count ->
    [one] Tässä luvussa on edelleen { $count } kohta. Siirrä tai roskata se ennen muuntaa sen litteä luku.
   *[other] Tässä luvussa on edelleen { $count } kappaletta. Siirrä tai roskata ne ennen muuntaa sen litteäksi luvuksi.
}

inspector = Ominaisuudet
inspector-empty = Avaa esine tarkastaaksesi sen.
inspector-promote = Muunna tyypiksi…
statusbar-toggle-outline = Näytä tai piilota kansiorakenne
statusbar-toggle-inspector = Näytä tai piilota ominaisuudet
statusbar-save-unsaved = Tallentamattomat muutokset. Klikkaa tallentaaksesi
statusbar-save-saved = Kaikki tallennetut muutokset
statusbar-save-autosave = Autotallennus on päällä. Muutokset tallentuvat kirjoittaessasi
statusbar-saving = Tallennetaan…
statusbar-word-count = { $count ->
    [one] { $count } sana
   *[other] { $count } words
}
statusbar-word-count-tooltip = Sanat kohtauksessa, jota editoit
statusbar-word-char-count = { $words ->
    [one] { $words } sana
   *[other] { $words } words
} · { $chars ->
    [one] { $chars } merkki
   *[other] { $chars } merkkiä
}
session-toggle = Kirjoitusistunto: Käynnistä tai keskeytä keskittynyt sprintti
session-configure = Aseta istunnon sanatavoite ja aikaraja
session-configure-title = Kirjoitetaan istuntoa
session-word-goal = Sanatavoite
session-time-limit = Määräaika
session-time-limit-unit = min
session-no-goal = Ei päämäärää
session-no-limit = Ei rajoituksia
session-reset = Nollaa istunto
session-readout = { $words } sanaa · { $time }
session-readout-timed = { $words } words · { $time } vasen
statusbar-focus-exit = Poistu häiriötilasta ilman häiriötekijöitä
statusbar-focus-synopsis = Tiivistelmä
statusbar-focus-go-prev = Edellinen tuote (Alt+Up)
statusbar-focus-go-next = Seuraava (Alt+ Alas)

rail-settings = Asetukset

language = Kieli
theme = Teema
english = englanti
french = ranska
light = Vaalea
dark = Tumma
settings-text-width = Tekstin leveys
settings-preview-width = Etsi esikatselun leveys
settings-autosave = Tallenna automaattisesti levylle
settings-show-welcome = Näytä kantoraketti käynnistettäessä
settings-show-welcome-tip = Kun se on ohi, viimeinen projekti avataan sen sijaan uudelleen.
settings-check-for-updates = Etsi uusia versioita
settings-check-for-updates-tip = Kerran päivässä enintään Skribisto kysyy projektin sivuilta, mikä versio on ajankohtainen. Pyynnössä ei ole tunnistetta eikä edes käynnissä olevaa versiota: vertailu tapahtuu tällä tietokoneella.

settings-title = Asetukset
settings-close = Sulje
settings-search = Hakuasetukset
settings-reset = Palauta oletukset
settings-done = Valmis
settings-cancel = Peruuta
settings-apply = Käytä
settings-ok = OK
settings-reset-confirm-title = Nollaa kaikki asetukset oletusarvoihin?
settings-reset-confirm-body = Palauttaa tämän sovelluksen ulkoasun, editorin ja kirjoitusasetukset. Projektin omat asetukset, näppäimistön pikanäppäimet ja tallennetut tyylit eivät muutu.
settings-empty-title = Täällä ei ole asetuksia vielä
settings-empty-hint = Tämä osio saa vaihtoehtoja tulevassa päivitys.

settings-sec-appearance-behaviour = Ulkoasu Käytös
settings-sec-editor = Muokkain
settings-sec-spelling = Oikeinkirjoitus
settings-sec-backup = Varmuuskopiointi
settings-sec-compile = Koostaminen ja vienti
settings-page-appearance = Ulkoasu
settings-field-image-size-policy = Suuret kuvat
settings-image-policy-ask = Kysy joka kerta
settings-image-policy-keep = Säilytä alkuperäinen
settings-image-policy-downscale = Optimoi
settings-hint-image-size-policy = Mitä tehdä, kun lisätty kuva on suurempi kuin käsikirjoitus tarvitsee.
settings-page-notifications = Ilmoitukset
settings-page-scene = Kohtaus
settings-page-synopsis = Tiivistelmä
settings-page-notes = Huomautuksia
settings-page-editor-behavior = Editorin toiminta
settings-page-goals = Tavoitteiden määrä
settings-page-games = Kirjoitetaan pelejä
settings-page-corkboard = Korttitaulu
settings-page-distraction-free = Häiriötön
settings-page-dictionaries = Sanakirjat
settings-page-autosave = Automaattinen tallennus
settings-page-export = Vientimuodot
settings-page-paratext = Paratekstirakenteet
settings-page-keymap = Pikanäppäimet
settings-keymap-filter = Suodata pikanäppäimet

settings-desc-sec-appearance-behaviour = Miltä sovellus itse näyttää ja mitä se tekee, kun se alkaa.
settings-desc-sec-editor = Kirjoituspinta: miltä se näyttää ja mitä se tekee kirjoitettaessa.
settings-desc-sec-spelling = Oikeinkirjoitus ja sanakirjat.
settings-desc-sec-backup = Miten työsi saavuttaa levyn ja mitä kopioita säilytetään.
settings-desc-sec-compile = Mikä jättää Skribisto, ja missä kunnossa.
settings-desc-sec-work = Tähän projektiin kuuluvat asetukset ja matkustavat sen tiedoston sisällä.
settings-desc-sec-extensions = Sivut mukana laajennukset asennettu tähän.
settings-desc-group-typography = Sivu per tekstilaji: tyyppi, koko, viivan korkeus ja väli.
settings-desc-appearance = Käyttöliittymän kieli, teema, tekstikoko ja aloitusikkuna.
settings-desc-notifications = Jokainen viesti tässä istunnossa on näyttänyt, ja toimia voit toistaa.
settings-desc-scene = Miten kohtaus proosa on asetettu: tyyppi kasvot, koko, linjan korkeus, luetelmakohdat ja väli.
settings-desc-synopsis = Miten synopsis-paneeli on asetettu, riippumatta käsikirjoituksesta.
settings-desc-notes = Miten muistiinpanot on asetettu, riippumatta käsikirjoituksesta.
settings-desc-corkboard = Kortin koko, mitä kortti näyttää, ja miten taulu järjestää ne.
settings-desc-distraction-free = Typografia, pylvään leveys ja ohjausnauha koko näytön kirjoittamiseen.
settings-desc-distraction-free-themes = Teemakirjasto koko näytön kirjoittamiseen: lähetetyt ja oma.
settings-desc-editor-behavior = Tekstin leveys, synopsis-asento, kirjoituskoneen selaaminen ja cart-korostus.
settings-desc-punctuation = Tyypillisen talon uudet projektit alkavat. Jokainen projekti seuraa tätä, ellei se aseta omaa projektiaan.
settings-desc-goals = Sana ja luonne kohteita, ja miten sanat lasketaan.
settings-desc-games = Rajoituksia asetat itsesi kirjoituksen aikana, kuten aina eteenpäin.
settings-desc-spellcheck = Yksi katkaisija, joka kytkee oikosulun päälle ja pois päältä kaikkialla.
settings-desc-dictionaries = Asenna, poista ja selaa tämän koneen oikeinkirjoitussanakirjoja.
settings-desc-autosave = Onko muokkaukset kirjoitetaan levylle yksin.
settings-desc-backup = Kun kopioita otetaan, missä niitä säilytetään ja kuinka monta. Jokainen projekti seuraa tätä, ellei se aseta omia.
settings-desc-export = Tyylit jokainen vienti kokoaa kautta: Lähetetyt ja oma.
settings-desc-paratext = Edessä ja takana materia uusi projekti voi alkaa.
settings-desc-keymap = Jokainen oikotie ja mihin se on sidottu.
settings-page-user = Käyttäjä
settings-desc-user = Kuka olet kirjoittamasi kommentit.
settings-group-identity = Henkilöllisyys
settings-field-user-name = Nimesi
settings-field-user-name-placeholder = Valinnainen
settings-field-user-name-hint = Merkitse kommentit ja vastaukset kirjoitat. Erona projektin tekijänimestä, joka on kirjan sivut ja matkustaa tiedoston sisällä.
settings-field-user-initials = Nimikirjaimesi
settings-field-user-initials-placeholder = Nimestäsi
settings-field-user-initials-hint = Mitä sana prosessori näyttää vieressä kommenttisi marginaali. Jätä se tyhjäksi käyttää niitä näytetään, otettu nimesi.
settings-field-user-hint = Molemmat ovat valinnaisia ja koskevat jokaista projektia tällä tietokoneella. Muuttaminen ne allekirjoittaa kommentteja kirjoitat seuraavaksi. Kommentit jo kirjoitettu pitää nimi ne on kirjoitettu alle.
settings-desc-author = Nimen, joka menee tähän projektiin.
settings-desc-structure = Olipa projektin luvut kansioita tai yksittäisiä kohteita.
settings-desc-language = Tämän projektin proosaa vastaan on tarkistettu kieli.
settings-desc-work-backup = Tämän projektin oma tai yleinen varasuunnitelma.
settings-desc-personal-dictionary = Tämän projektin sanat ovat oikein kirjoitettuja.
settings-desc-tags = Värikoodatut tarrat tässä projektissa ovat sen kohteita.
settings-desc-templates = Muistilappu malleja voit lisätä kirjoittaessaan tässä projektissa.
settings-desc-text-replacements = Pikanäppäimet, jotka laajenevat kirjoittaessasi projektiin.
settings-desc-work-punctuation = Lainaukset, viivat ja välitys tämän projektin proosa seuraa.

settings-group-typography = Typografi
settings-group-writing-column = Kirjoitetaan saraketta
settings-group-writing-view = Kirjoitetaan näkymää
settings-group-distraction-free-strip = Valvontanauha
settings-group-theme = Teema
settings-group-language = Kieli
settings-group-startup = Käynnistys
settings-group-updates = Päivitykset
settings-group-autosave = Automaattinen tallennus
settings-field-typeface = Kirjasinlaji
settings-field-size = Tekstikoko

editor-size-changed-manuscript = Käsikirjoituksen koko: { $percent }
editor-size-changed-synopsis = Synopsis tekstin koko: { $percent }
editor-size-changed-notes = Huomautusten tekstikoko: { $percent }
editor-size-changed-corkboard = Korkkikortin tekstin koko: { $percent }
editor-size-changed-corkboard-expanded = Laajennettu editorin tekstikoko: { $percent }
editor-size-changed-distraction-free = Häiriötön tekstikoko: { $percent }

menu-text-size-increase = &Suurenna tekstin kokoa
menu-text-size-decrease = &Pienennä tekstin kokoa
menu-text-size-reset = &Nollaa tekstin koko
settings-field-line-height = Viivan korkeus
settings-field-first-line-indent = Ensimmäinen luetelmakohta
settings-field-paragraph-spacing-before = Tila ennen kappaletta
settings-field-paragraph-spacing-after = Avaruus kappaleen jälkeen
settings-field-column-width = Sarakeen leveys
settings-distraction-free-width-hint = Koskee vain silloin, kun häiriötekijätön tila on päällä. Kohtauksesi, Synopsis- ja Notes-palstan leveys on koskematon.
settings-distraction-free-title = Säilytä kohteen nimi
settings-distraction-free-word-count = Pidä sanalasku
settings-distraction-free-session = Jatka kirjoitusta
settings-distraction-free-go-to = Pidä painike...
settings-distraction-free-go = Pidä edelliset ja seuraavat painikkeet
settings-distraction-free-chrome-hint = Poistumispainike pysyy aina, mitä tahansa valitset täällä. Se on tie ulos, jos Escape on varattu.
settings-field-app-theme = Teema
settings-theme-light = Vaalea
settings-theme-dark = Tumma
settings-field-text-scale = Liitäntätekstin koko
settings-field-language = Liitännän kieli
settings-synopsis-placement = Synopsis-asento
settings-synopsis-placement-none = Ei ole
settings-synopsis-placement-top = Yllä
settings-synopsis-placement-side = Vieressä
settings-typewriter = Kirjoituskoneen selaaminen
settings-typewriter-tip = Pitää rivin kirjoitat kiinteällä korkeudella, kun käsikirjoitus kääröjä sen alla. Klikkaaminen vielä laittaa cart jossa klikkaat.
settings-typewriter-position = Viivan sijainti
settings-typewriter-position-top-third = Kolmas
settings-typewriter-position-middle = Keski
settings-typewriter-position-bottom-quarter = Neljännesvuosi
settings-highlight-scope = Korosta ympärillesi
settings-highlight-scope-none = Ei ole
settings-highlight-scope-sentence = Tuomio
settings-highlight-scope-paragraph = Kohdat
settings-highlight-scope-tip-none = Mikään ei ole hämärtynyt kirjoittaessasi.
settings-highlight-scope-tip-sentence = Varjosta kirjoittamasi lause, niin se erottuu sen ympärillä olevista.
settings-highlight-scope-tip-paragraph = Varjosta koko kohta kirjoitat, pitääksesi kohta olet työstämässä silmällä.
settings-group-container-views = Konttinäkymät
settings-remember-view = Muista viimeinen näkymä kullekin kohdetyypille
settings-remember-view-tip = Avaa säiliö viimeksi käyttämällesi näkymälle.
settings-remember-view-tip-more =
    Kirja, osa ja luku jokainen tarjoaa useita näköaloja (sen oma sivu, koko
    käsikirjoitus, koko synopsis). Käynnistä tämä ja jokainen tyyppi avaa uudelleen
    Näkymä viimeksi valittu, esim. vaihtaa yksi Luku Full Chapter ja
    Seuraavassa luvussa avaat näyttää Full Chapter liian. Jokainen tyyppi muistaa oman näköalansa.
settings-group-counting = Sananlasku
settings-counting-auto = Automaattinen (kielellä)
settings-counting-whitespace = Jaa välilyöntejä
settings-counting-unicode-words = Unicode- sanoja
settings-counting-cjk-hybrid = CJK-aware (merkkiä kohden)
settings-counting-hint = Automaattinen laskee kiinan ja japanin luonteen ja jokaisen muun kielen sanalta. Muuta se vain, jos kreivi näyttää väärältä kielesi.
settings-group-goals-display = Näyttö
settings-show-characters = Näytä merkkien määrä tilarivissä
settings-autosave-hint = Muutokset kirjoitetaan levylle automaattisesti kirjoittaessasi.

settings-sec-work = Projekti
settings-page-structure = Rakenne
settings-page-author = Tekijä
settings-field-author-name = Tekijän nimi
settings-field-author-placeholder = Valinnainen
settings-field-author-hint = Ilmestyy koottu otsikko sivu ja viety tiedosto metadata. Jätä se tyhjäksi jättää se. Tämä on kirjan byline. Nimi kommentit ovat allekirjoittaneet on Asetukset Käyttäjä.
settings-group-chapters = Luvut
tidy-titles-title = Siivoa luvun otsikot
tidy-titles-none = Mikään luku tai osa otsikosta ei vain toista omaa numeroaan.
tidy-titles-lead = { $count ->
        [one] Yksi luku tai osa on otsikoitu vain sen oma numero.
       *[other] { $count } lukuja ja osia otsikoidaan vain omalla lukumäärällään.
    }
tidy-titles-explain = Näiden nimien selvittäminen jättää jokaisen nimen kirjan jo tunteman numeron mukaan, saman numeron kuin vientijäljet. Mikään muu ei muutu ja yksi peruu ne kaikki.
menu-document-tidy-titles = &Siistit nimitykset...
settings-group-numbering = Numerointi
settings-number-chapters = Lukumäärä lukuja ja osia
settings-number-chapters-tip = Lukujen ja osien mukana on numero, joka on laadittu siitä, missä ne istuvat kirjassa, jotka on esitetty niiden otsikon vieressä tässä, ja jotka on painettu viennin.
settings-number-chapters-tip-more = Numeroa ei koskaan tallenneta otsikkoon, joten se pysyy oikeassa, kun tilaat, lisäät tai poistat. Sammuta tämä ja vienti tulostaa otsikot yksin, mitä vienti tyyli haluaa. Jätä yksi luku pois (prologi, interlude), käytä tarkastajan Numbering kytkin sen sijaan: joka pitää sen kirjassa, mutta estää sen ottamasta numeroa.
settings-part-resets-chapter = Aloita luvun numerot uudelleen jokaisesta osasta
settings-part-resets-chapter-tip = Poissa oletuksena: luvut kulkevat suoraan ympäri osia kirjan, joten "Osa kaksi" avautuu luvussa Yksitoista.
settings-part-resets-chapter-tip-more = Se on tavanomainen kaupan käytäntö, ja mitä lukija odottaa. Käynnistä se kirja, jonka osat on tarkoitus lukea erillisinä määrinä, jossa jokainen avaa luvun 1.
settings-chapter-flat = Yksittäiset luvut
settings-chapter-flat-hint = On: luku on yksi kohde. Kirjoitat siihen, ja se ei ole kohtauksia. Off: luku on kansio. Kirjoitat silti siihen, mutta se voi myös pitää kohtauksia. Uudet luvut seuraavat tätä asetusta; olemassa olevat muuntaa kautta Promote.

settings-styles-builtin = Sisäänrakennetut tyylit
settings-styles-user = Tyylini
settings-styles-builtin-badge = Sisäänrakennettu
settings-styles-duplicate = Monista
settings-styles-edit = Muokkaa
settings-styles-delete = Poista
settings-styles-import = Tuo...
settings-styles-export = Vie...
settings-styles-copy-suffix = (kopio)
settings-styles-json-filter = Vie
settings-styles-editor-title = Muokkaa tyyliä
settings-styles-editor-none = Valitse oma tyyli muokattavaksi tai monista valmis tyyli.
settings-styles-imported = Tyyli tuotu
settings-styles-import-failed = Tyyliä ei voitu tuoda
settings-styles-exported = Viety tyyli
settings-styles-export-failed = Tyyliä ei voitu viedä
settings-styles-field-name = Nimi
settings-styles-field-chapters = Lukujen otsikot
settings-styles-field-parts = Osien otsikot
settings-styles-field-scene-break = Kohtaustauko
settings-styles-field-major-scene-break = Suuri kohtauskatkos
settings-styles-field-spacing = Viivaväli
settings-styles-field-justify = Oikeuta teksti
settings-styles-field-notes = Sisältää muistiinpanoja
settings-styles-field-synopses = Sisällytä synopsit
settings-styles-field-scene-titles = Sisällytä kohtauksen otsikot
settings-styles-field-epigraphs = Sisällytä epigrafit
settings-styles-field-paratexts = Sisällytä paratekstit
settings-styles-field-epigraph-placement = Epigrafin sijainti
settings-styles-epigraph-after = Otsikon jälkeen
settings-styles-epigraph-before = Ennen otsikkoa
settings-styles-field-footnotes = Sisältää alaviitteet
settings-styles-field-footnote-numbering = Alaviitenumero
settings-styles-footnote-numbering-continuous = Jatkuva
settings-styles-footnote-numbering-per-chapter = Aloita jokainen luku uudelleen
settings-styles-footnote-numbering-per-book = Käynnistä jokainen kirja uudelleen
settings-styles-field-images = Kuvat
menu-image = &Kuva
settings-styles-images-beside = Asiakirjan vieressä
settings-styles-images-embed = Asiakirjan sisällä
settings-styles-images-omit = Jätä pois
settings-styles-group-round-trip = Lähettäminen toimittajalle
settings-styles-field-comments = Kommentteja
settings-styles-field-round-trip-marks = Sisällytä uudelleentuonnin tunnisteet
settings-styles-round-trip-hint = Vain DOCX ja ODT. Näkymättömien tunnisteiden avulla takaisin tuotu tiedosto päivittää tämän projektin sen sijaan, että siitä syntyisi toinen kopio.
settings-styles-group-pages = Sivu
settings-styles-field-cover = Avaa kannella
settings-styles-field-word-count = Sanamäärä otsikkosivulla
settings-styles-field-page-books = Uusi sivu jokaisessa kirjassa
settings-styles-field-page-parts = Uusi sivu jokaisessa osassa
settings-styles-field-page-chapters = Uusi sivu kussakin luvussa
settings-styles-field-page-paratexts = Uusi sivu jokaisessa paratekstissä
settings-styles-heading-none = Ei otsikkoa
settings-styles-heading-numbered = Ainoastaan numero
settings-styles-heading-title = Ainoastaan osasto
settings-styles-heading-both = Numero + nimi
settings-styles-break-blank = Tyhjä linja
settings-styles-break-none = Ei ole
settings-styles-spacing-single = Yksinkertainen
settings-styles-spacing-onehalf = 1½
settings-styles-spacing-double = Kaksinkertainen

welcome-title = Tervetuloa osoitteeseen Skribisto
welcome-version = Versio { $version }
welcome-update-available = Versio { $version } on saatavilla
welcome-update-tip = Avaa lataussivun selaimessasi.
welcome-search = Hakutyöt
welcome-open = Avaa
welcome-new-work = Uusi projekti
welcome-create-from = Luo kohteesta…
welcome-recent-works = Viimeisimmät projektit
welcome-empty-recents = Ei uusia teoksia vielä.
welcome-no-matches = Mikään uusi työ ei vastaa etsintääsi.
welcome-learn-soon = Oppaat ja vinkit tulevat pian.
welcome-about-blurb = Skribisto, Rust + Teksilo kirjoitussovelluksen uudelleenkirjoitus.
welcome-tagline = *Hiljainen paikka kirjoittaa pitkiä asioita. *
welcome-github = GitHub
welcome-discord = Erimielisyydet
nav-works = Hankkeet
nav-examples = Esimerkit
nav-learn = Opi
nav-about = Tietoja

synopsis = Tiivistelmä
epigraph = Motto
pane-manuscript = Käsikirjoitus
corkboard = Korttitaulu
overview = Yleiskatsaus

corkboard-card-count = { $count ->
    [one] { $count }-kortti
   *[other] { $count }-kortit
}
corkboard-child-count = { $count ->
    [one] { $count } kohta
   *[other] { $count } erää
}
corkboard-view-nested = Pesitty
corkboard-view-flat = Tasainen
corkboard-layout-hint = Avaa kansiokortti ja näytä kaikki kohdat kerralla.
corkboard-card-size = Kortin koko
corkboard-search-placeholder = Suodatuskortit...
corkboard-empty-title = Täällä ei ole vielä mitään.
corkboard-empty-hint = Käytä ...+...
corkboard-new = Uusi
corkboard-show-card-numbers = Numeroi kortit
corkboard-modal-size = Laajennetun muokkaimen koko
corkboard-scope-hint = Koskee jokaista avointa korkkitaulua joka projektissa.
corkboard-sort-manuscript = Käsikirjoitusjärjestys
corkboard-sort-title-asc = Osasto A.Z
corkboard-sort-title-desc = Osasto
corkboard-card-number = Kortti { $number }

duplicate = Monista
duplicate-n = { $count ->
    [one] Kaksinkertaista { $count } kortti
   *[other] Kaksinkertaista { $count } korttia
}
set-label-n = { $count ->
    [one] Aseta etiketti { $count }-korttiin
   *[other] Aseta etiketti { $count } korttiin
}
move-to-trash-n = { $count ->
    [one] Siirrä { $count } kortti roskakoriin
   *[other] Siirrä { $count } korttia roskakoriin
}
reveal-in-outline = Paljasta ääriviivat

corkboard-move-to = Siirry...
corkboard-move-to-n = { $count ->
    [one] Siirrä { $count } kortti...
   *[other] Siirrä { $count } korttia...
}
corkboard-move-picker-title = Siirry...
corkboard-move-picker-empty = Ei kansioita vielä.
corkboard-move-picker-cancel = Peruuta
corkboard-move-here = Siirrä tänne
corkboard-moved-ok = { $count ->
    [one] { $count } kortti siirretty
   *[other] { $count } korttia siirretty
}
corkboard-move-into-self = Säiliötä ei voi siirtää itseensä. Valitse määränpää sen ulkopuolelta.
corkboard-move-failed = Kortteja ei voitu siirtää sinne.

overview-col-title = Otsikko
overview-col-type = Tyyppi
overview-col-label = Lisäteksti
overview-col-tags = Tunnisteet
overview-col-books = Kirjat
overview-col-own-words = Sanat
overview-col-total-words = Yhteensä
overview-row-count = { $count ->
    [one] { $count } kohta
   *[other] { $count } erää
}
overview-search-placeholder = Suodata kohteita...
overview-expand-all = Laajenna kaikki
overview-collapse-all = Supista kaikki
overview-table-label = Sisällys
overview-empty-title = Täällä ei ole vielä mitään.
overview-gone-title = Tämä kontti on poissa.
overview-gone-hint = Se siirrettiin roskikseen.
overview-empty-hint = Käytä ...+...
corkboard-grid-label = Korkkikortit
corkboard-rename-field = Nimeä kohde uudelleen
corkboard-expand-synopsis = Laajenna synopsis
corkboard-synopsis-modal-title = Tiivistelmä
corkboard-badge-scene = Kohtaus
corkboard-badge-chapter = Luku
corkboard-badge-part = Osa
corkboard-badge-book = Kirja
corkboard-badge-note = Muistiinpano
corkboard-badge-folder = Kansio
corkboard-badge-paratext = Parateksti
corkboard-badge-text = Teksti
corkboard-badge-end = Loppu
settings-group-corkboard-layout = Asettelu
settings-group-corkboard-cards = Kortit
corkboard-show-word-count = Näytä sana lasketaan kortteja
text-heading = Teksti
no-content = Tämä kohta ei ole muokattavissa sisältöä.
untitled = Nimetön
placeholder-title = Otsikko…
placeholder-subtitle = Alaotsikko…
placeholder-chapter-title = Luvun nimi...
split-editor = Jaa muokkain
close-split-view = Sulje jakonäkymä
drop-open-here = Avaa tästä.
drop-open-to-side = Avaa sivulle

segment-chapter = Luku
segment-part = Osa
segment-book = Kirja
segment-pace = Tahti
segment-notes = Huomautuksia
segment-story-bible = Tarinan tietopankki
segment-note-own = Muistiinpano
segment-note-details = Yksityiskohdat
segment-note-in-prose = Proosaan
pace-placeholder = Pace Planner ilmestyy tänne.
pace-empty-title = Suunnittele tämän kirjan tahti
pace-empty-body = Aseta sana tavoite ja deadline, ja Skribisto toimii päivittäistä vauhtia saada sinut sinne.
pace-start-planning = Aloita suunnittelu
pace-section-schedule = Aikataulu
pace-section-progress = Edistyminen
pace-advancement = Eteneminen
pace-goal = Sanatavoite
pace-deadline = Määräaika
pace-active = Kirjoitussuunnitelma käytössä
pace-day-mon = ma
pace-day-tue = ti
pace-day-wed = ke
pace-day-thu = to
pace-day-fri = pe
pace-day-sat = la
pace-day-sun = su
pace-card-written = sanat kirjoitettu
pace-card-of-goal = tavoite
pace-card-rate = sanat / kirjoituspäivä
pace-card-days-left = kirjoituspäiviä jäljellä
pace-card-streak = päiväputki
pace-card-ahead = sanat edellä tahtia
pace-card-behind = sanat tahdin takana
pace-charts-empty = Edistyskaaviot näkyvät täällä, kun sanasi lasketaan tallennukseen.
pace-chart-progression = Sanat kirjoitettu vs. kohde
pace-chart-words-per-day = Sanat päivässä
pace-series-actual = Todellinen
pace-series-target = Tavoite
pace-series-words-per-day = Sanat/päivä
pace-daily-target-line = Tasainen tahti: { $count ->
    [one] { $count } sana/päivä
   *[other] { $count } words/day
}
pace-section-holidays = Loma
pace-section-milestones = Välitavoitteet
pace-holidays-none = Jokainen aikataulullinen päivä on tärkeä.
pace-holiday-label = Lomanimi
pace-add-holiday = Lisää
pace-remove = Poista
pace-milestones-none = Ei vielä virstanpylväitä.
full-chapter = Koko luku
full-part = Täysi osa
full-book = Koko kirja
full-synopsis = Synopsis
rename = Nimeä uudelleen
set-label = Aseta nimike
insert-scene = Lisää kohtaus
insert-chapter = Lisätään kappale.
split-scene = Jaettu kohtaus
move-up = Ylös.
move-down = Siirry alas
merge-with-previous = Yhdistä edelliseen
move-to-trash = Siirrä roskakoriin
rename-chapter = Nimeä luku uudelleen
add-scene = Lisää uusi kohtaus
add-chapter = Lisätään uusi luku
new-scene-title = Uusi kohtaus
placeholder-scene-name = Kohtauksen nimi...
menu-cut = Leikkaa
menu-copy = Kopioi
menu-paste = Liitä
menu-paste-unformatted = Liitä muotoilematon
menu-select-all = Valitse kaikki
format-bold = Lihavointi
format-italic = Kursivointi
format-underline = Alleviivaus
format-strikethrough = Yliviivaus

binder = Kansiorakenne
no-work = Työtä ei ladattu
no-recent-works = Ei tuoreita teoksia
switcher-open-section = Tällä hetkellä avoinna
switcher-recent-section = Äskettäin
switcher-this-window = tämä ikkuna
open-project-title = Avaa projekti
open-project-question = Miten haluat avata?{ $title }”?
open-in-new-window = Avaa uudessa ikkunassa
open-here = Avaa tästä.

binder-all = Kaikki kansiorakenteet
binder-show-all = Näytä kaikki kansiot
binder-new = Uusi kansio...
binder-default-name = Uusi kansio
binder-item-count = { $count ->
    [one] { $count } kohta
   *[other] { $count } erää
}
binder-search-placeholder = Suodata ääriviivat...
binder-search-scope = Etsi kaikki kansiot
binder-trash-confirm-title = Siirrä kansio roskikseen?
binder-trash-confirm-text = “{ $name }Ja kaikki sen kohteet siirretään roskakoriin.

dialog-rename = Nimeä uudelleen
dialog-new-binder = Uusi kansio
dialog-set-label = Aseta nimike
dialog-new-scene = Uusi kohtaus
close-work-question = Tallenna muutokset ennen työn päättämistä?
quit-question = Tallenna muutokset ennen lopettamista?
quit-save-work-question = Tallenna muutokset { $title } ennen lopettamista?
unsaved-changes = Tämä työ on pelastumaton muutoksia.
new-work-unsaved-question = Tallenna muutokset ennen uuden teoksen luomista?
open-work-unsaved-question = Tallenna muutokset ennen uuden teoksen avaamista?
switch-backup-discard-title = Hylkäätkö muutokset tähän varaukseen?
switch-backup-discard-text = Muutoksia varmuuskopioon ei voi tallentaa. Käytä Tallenna nimellä tai palauttaaksesi ne, tai hävitä ne ja avaa muut työt.
switch-save-failed = Työtä ei voitu tallentaa, joten sitä ei korvattu: { $error }
switch-save-not-started = Työtä ei voitu pelastaa, joten sitä ei vaihdettu.
close-save-failed = Työtä ei voitu pelastaa, joten sitä ei ollut suljettu: { $error }
close-save-not-started = Työtä ei voitu pelastaa, joten se ei ollut kiinni.
save-not-started = Työtä ei voitu pelastaa.

could-not-open-work = Ei voitu avata "{ $file }"
could-not-open-work-too-new = '{ $file }' tarvitsee uudemman Skribisto
could-not-open-work-too-new-detail = Tallennettu Skribisto muodossa { $written_by }; avaaminen se tarvitsee muodossa { $requires } tai uudempi, ja tämä rakentaa tukee jopa muodossa { $supported }. Päivitys Skribisto avata sen.
could-not-open-work-importing = "{ $file }" on kirjoitettu tuonti
could-not-open-work-importing-detail = Tuonti on kirjoittaa tämän projektin eikä ole valmis. Avaa se tuonnin jälkeen: avataan nyt, se näyttää projektin tuonti on korvaamassa, ja sen seuraava tallennus kirjoittaa projektin takaisin tuonnin.
could-not-open-example = Esimerkkiä ei voitu avata: { $error }
could-not-open-link = Ei voitu avata { $url }: { $error }
could-not-create-work = Työtä ei voitu luoda: { $error }
saving-as-file = Säästän { $target }...
saving-as-folder = Säästetään { $target }/...
saved-as = Säästetty { $target }
save-error = Ei voitu tallentaa: { $error }
save-flush-failed = Pelastaminen meni läpi, mutta näitä asiakirjoja ei voitu luovuttaa sille ja ne ovat edelleen tallentamattomia: { $titles } ({ $error })
backup-error = Ei voitu varmuuskopioida: { $error }
backing-up = Peruuta...
backup-nothing-open = Mikään projekti ei ole auki.
backup-already-running = Apujoukkoja on jo käynnissä.
backup-complete = Varmuuskopiointi valmis ({ $ok } tallennettu, { $skipped } jo nykyinen)
backup-partial = Varmuuskopiointi valmis: { $ok } tallennettu, { $failed ->
    [one] { $failed } kohde epäonnistui
   *[other] { $failed } kohdetta ei löytynyt
}
backup-no-destination-title = Varmuuskopiointia ei ole saatavilla
backup-no-destination-text = Asennetuista varmuuskopiokohteista ei ole mahdollista saavuttaa mitään (esimerkiksi ulkoinen asema voidaan irrottaa). Kytke se sisään ja uudelleen tai poistu ilman varmuuskopiota.
backup-failed-close-title = Varmuuskopiointia ei voitu tallentaa
backup-failed-close-text = Varakopiota ei voitu kirjoittaa ennen sulkemista. Jokainen kohde epäonnistui (asema on voitu poistaa, tai se voi olla täynnä tai kirjoitussuojattu). Korjaa se ja yritä uudelleen tai poistu ilman varmuuskopiota.

backup-choice-title = Varmuustiedosto
backup-choice-heading = Olet avannut varmuuskopion
backup-choice-subtitle = Tämä on projektin varasuunnitelma.
backup-choice-subtitle-dated = Varmistettu { $date }.
backup-choice-body = Voit avata sen ja muokata sitä vapaasti, mutta muutokset voidaan säilyttää vain Save Asilla. Alkuperäiseen projektitiedostoon ei kosketa. Tai palauttaa tämä projekti juuri tähän varmuuskopioon.
backup-choice-open = Avaa varmuuskopio
backup-choice-restore = Palauta tämä projekti tähän pisteeseen...
backup-choice-not-a-backup = Ei, avaa se normaalisti.
backup-banner-title = Varmuuskopiointi: muutoksia ei voi tallentaa tähän
backup-banner-description = Käytä Tallenna Pidä muokkaukset uudessa tiedostossa tai Palauta korvata alkuperäisen projektin tällä varmuuskopiolla.
backup-banner-restore = Palauta…
backup-banner-save-as = Tallenna...
backup-restore-original-missing = Alkuperäinen projekti ennallistaa yli. Käytä Tallenna Pidä tämä varmuuskopio uutena projektina sen sijaan.
backup-restore-close-elsewhere-title = Projekti avautuu toisessa ikkunassa
backup-restore-close-elsewhere-text = Projekti on auki toisessa ikkunassa. Sulje se ensin ja yritä sitten uudelleen.
backup-restore-focus-window = Keskitä tuo ikkuna
backup-restore-confirm-title = Palautatko nämä?
backup-restore-confirm-text = Projektin nykyinen versio kopioidaan turvavarmistukseksi, ennen kuin se korvataan tällä versiolla.
backup-restore-confirm-ok = Palauta
backup-restore-error = Ei voitu palauttaa: { $error }
backup-restored-ok = Projekti palautettu.
backup-restored-with-safety = Projekti palautettu. Edellinen versiosi tallennettiin numeroon { $path }.
close-backup-discard-title = Hylkäätkö muutokset tähän varaukseen?
close-backup-discard-text = Varmuuskopioihin tehtäviä muutoksia ei voi tallentaa. Käytä Tallenna Kuten pitääksesi ne, tai hävitä ja sulje.
quit-backup-discard-title = Hylkäätkö muutokset ja lopetat?
quit-backup-discard-work-question = Hylkää muutokset { $title } ja lopettaa?
quit-backup-discard-text = Varmuuskopioihin tehtäviä muutoksia ei voi tallentaa. Käytä Tallenna Kuten pitääksesi ne, tai hävitä ja lopeta.
backup-nudge-text = Tätä projektia varten ei ole perustettu varmuuskopioita.
backup-nudge-action = Järjestä varmuuskopiot...

menu-backups-list = Var&muuskopioita...
backups-title = Varmuuskopioita
backups-loading = Etsitään varmuuskopioita…
backups-empty = Tälle projektille ei vielä löytynyt varmuuskopioita.
backups-open = Avaa
backups-reveal = Näytä sijainti
backups-delete = Poista tämä varmuuskopio
backups-delete-confirm-title = Poistatko tämän varajärjestelmän?
backups-delete-confirm-text = “{ $name } Tätä ei voi perua.
backups-delete-error = Varmuuskopiointia ei voitu poistaa: { $error }
backups-refresh = Päivitä
backups-close = Sulje

settings-page-backup = Varmuuskopioiden oletusarvot
settings-page-work-backup = Varmuuskopioita
settings-backup-general-title = Oletus varmuuskopioasetukset
settings-backup-work-title = Tämän projektin varmuuskopiot
settings-backup-inherit = Käytä yleisiä varmuuskopioasetuksia
settings-backup-inheriting = Tässä projektissa käytetään yleisiä varmuuskopioasetuksia.
settings-backup-none-hint = Automaattiset varmuuskopiot eivät ole käytössä (jokainen laukaisin on pois päältä).
settings-backup-last = Viimeinen varmuuskopio: { $date }
settings-backup-last-never = Ei varmuuskopioita vielä.
settings-backup-open-list = Avaa varmuuskopioluettelo...
settings-backup-triggers = Milloin peruutat
settings-backup-on-close = Peruuta projektin sulkemisen yhteydessä
settings-backup-on-open = Peruuta kun avaat projektin
settings-backup-interval = Peruuta säännöllisesti, joka
settings-backup-destinations = Varmuuskohteet
settings-backup-default-location = Oletuspaikka
settings-backup-usage-since = { $count ->
    [one] { $count } varmuuskopio · { $size } · vanhin { $oldest }
   *[other] { $count } varmuuskopiot · { $size } · vanhin { $oldest }
}
settings-backup-usage = { $count ->
    [one] { $count } varmuuskopiot · { $size }
   *[other] { $count } varmuuskopiot · { $size }
}
settings-backup-usage-empty = Täällä ei ole mitään.
settings-backup-usage-measuring = Mittaa...
settings-backup-reveal-root = Näytä kansio
settings-backup-dest-none = Ei kohteita. Varaosat tallennetaan projektin viereen.
settings-backup-dest-remove = Poista
settings-backup-dest-add = Lisää kansio
settings-backup-dest-refresh = Päivitä
settings-backup-retention = Kuinka monta pitää
settings-backup-retention-tiered = Portaittain
settings-backup-retention-keep-n = Säilytä viimeinen N
settings-backup-retention-tip = Miten vanhoja varmuuskopioita leikataan.
settings-backup-retention-tip-more = Tiered pitää yhden varmuuskopion tunnissa yhden päivän, per päivä viikon, viikossa kuukauden, ja kuukaudessa yli, niin viime historia pysyy tiheä ja vanha historia ohut. . Säilytä viimeinen N. yksinkertaisesti pitää N uusimmat kopiot. Molemmissa moodissa uusimmat kopiot (minimi alla) pidetään aina.
settings-backup-gfs-hourly = Tunti (viimeinen 24 h)
settings-backup-gfs-daily = Päivittäin (viime viikolla)
settings-backup-gfs-weekly = Viikoittain (viimeinen kuukausi)
settings-backup-gfs-monthly = Kuukausittain
settings-backup-keep-n = Säilytettävä numero
settings-backup-min-keep = Pidä aina ainakin
settings-backup-dedup = Ohita varmuuskopio, kun mikään ei ole muuttunut

new-work-title = Uusi projekti
new-work-close = Sulje
new-work-name = Projektin nimi
new-work-name-placeholder = Nimetön
new-work-author = Tekijä
new-work-author-placeholder = Valinnainen
new-work-format = Muotoilu
new-work-single-file = Yksi tiedosto
new-work-bundle = Projektikansio
new-work-convert-later = Voit muuntaa formaatteja myöhemmin.
new-work-location = Paikka
new-work-will-create = Luo
new-work-language = Oletuskieli
new-work-language-hint = Sovelletaan uusia tekstejä oikoluku. Jokainen teksti voidaan vaihtaa toiseen kieleen.
new-work-template = Malli
new-work-template-none = Ei ole
new-work-template-empty-novel = Tyhjä romaani
new-work-template-light-novel = Kevyt uutuus
new-work-template-novel = Novelli
new-work-template-novel-in-parts = Uusia osia
new-work-template-notebook = Muistikirja
new-work-cancel = Peruuta
new-work-create = Luo projekti
new-work-step-details = Yksityiskohdat
new-work-step-language = Kielirakenne
new-work-step-template = Malli
new-work-back = Takaisin
new-work-next = Seuraava
new-work-documents-title = Uusi projekti asiakirjoista
new-work-step-import = Tuo
new-work-create-and-import = Luo tuonti...
new-work-documents-next-title = Sinun paperisi tulevat seuraavaksi.
new-work-documents-next-body = Projektin luominen avaa sille tuontivelhon, jossa valitset asiakirjat, tarkista rakenne Skribisto lukee niistä ja valitse minne se laskeutuu. Projektiin ei ole kirjoitettu mitään ennen kuin olet vahvistanut sen siellä.
new-work-documents-no-template = Tämä projekti alkaa tyhjänä tarkoituksella: ei mallia, joten maahan tuodut asiakirjat ovat ainoa asia siinä.
new-work-documents-chapter-scene-hint = Koskee tuonnin luomia lukuja. Voit muuttaa niitä myöhemmin projektin asetuksissa.
new-work-single-file-desc = Yksi .skrib arkisto (zip). Kannettava, helppo varmuuskopioida.
new-work-bundle-desc = Kansio, jossa on kaikki tekstivaltit.
new-work-template-none-count = tyhjä kansio
new-work-template-empty-novel-count = 1 luku
new-work-template-light-novel-count = 15 lukua
new-work-template-novel-count = 20 lukua
new-work-template-novel-in-parts-count = 3 osaa, 24 lukua
new-work-template-notebook-count = vapaamuotoiset muistiinpanot
new-work-chapter-scene = Yksittäiset luvut
new-work-chapter-scene-tip = Jokainen luku on yksi kohde kirjoitat suoraan, ilman kohtauksia sen alla. Jätä tämä pois klassinen ulkoasu, jossa luku on kansio: kirjoitat suoraan siihen myös, mutta se voi pitää kohtauksia samoin.
new-work-chapter-scene-tip-more = Kirjoitat lukuun joka tapauksessa. Ainoa ero on se, voiko se *sisältää * kohtauksia. Skribisto:n kansiopuu on vain organisatorinen, joten molemmat asettelut koota samaan kirjaan, voit sekoittaa ne vapaasti, ja Edistää muuntaa luvun kahden menettämättä sanaakaan.
new-work-name-required = Anna työn nimi
new-work-name-invalid = Tämä nimi ei ole käytettävissä olevia merkkejä
new-work-location-required = Valitse sijainti
new-work-location-missing = Kansiota ei ole olemassa
new-work-location-not-folder = Tämä polku ei ole kansio
new-work-location-readonly = Tämä kansio ei ole kirjoitettava
new-work-target-exists = Projekti tällä nimellä on jo tässä kansiossa. Työn luominen kysyy ennen sen korvaamista
new-work-target-is-folder = Tämän niminen kansio on jo tässä paikassa. Valitse toinen nimi
new-work-overwrite-title = Korvaako olemassa oleva projekti?
new-work-overwrite-text = “{ $name } Vaihdako se uuteen projektiin?
new-work-target-open-title = Projekti on auki
new-work-target-open-text = “{ $name }... Skribisto. Luominen uuden projektin tilalle korvaisi projektin, jota työstät, ja seuraava tallennuksesi kirjoittaisi projektin takaisin uuden päälle. Valitse toinen nimi tai kansio tai sulje projekti ensin.
target-importing-title = Tämän projektin kirjoittaminen on tuonti
target-importing-text = Tuonti on vielä kirjoittaa ...{ $name } Odota, kunnes se on valmis, tai valitse toinen nimi.
import-plume-busy-title = Toinen Plume Creator-hanke tuodaan
import-plume-busy-text = Plume Creator projektit tuodaan yksi kerrallaan. Odota, kunnes meneillään oleva tuonti on päättynyt tai peruuta se, aloita tämä uudelleen.
import-manuskript-busy-title = Toinen Manuskript-hanke tuodaan
import-manuskript-busy-text = Manuskript projektit tuodaan yksi kerrallaan. Odota, kunnes meneillään oleva tuonti on päättynyt tai peruuta se, aloita tämä uudelleen.
backup-restore-importing-text = Tuonti on vielä kirjoittaa ...{ $name }Projekti tämä varmuuskopio olisi palautettu yli. Palauta varmuuskopio, kun tuonti on päättynyt.

new-work-manuscript = Käsikirjoitus
new-work-notes = Huomautuksia
new-work-research = Tutkimus
new-work-notebook = Muistikirja
new-work-scene = Kohtaus
new-work-note = Muistiinpano
new-work-front-matter = Etusivu
new-work-back-matter = Taka-aines
new-work-paratext = Kirjan rakenne
new-work-paratext-none = Ei rakennetta
new-work-paratext-hint = Etu- ja taka-asia perinnettä avaa ja sulkee kirjan. Voit siirtää, nimetä tai poistaa sen jälkeenpäin.
new-work-tags = Tunnisteet
new-work-tags-none = Ei tunnisteita
new-work-tags-hint = Alkava paletti merkkien, paikkojen ja muiden merkitsemiseen. Valinnainen, ja jokainen tagi voidaan nimetä uudelleen, värittää tai poistaa jälkeenpäin.
new-work-note-templates = Huomautusmallit
new-work-note-templates-none = Ei malleja
new-work-note-templates-hint = Muoto tarinan raamattu note alkaa. Valinnainen, ja jokainen malli voidaan muokata tai poistaa jälkeenpäin.
new-work-characters = Merkit
new-work-places = Paikat

import-plume-title = Tuo Plume Creator projekti
import-plume-close = Sulje
import-plume-source = Plume-projekti
import-plume-source-hint = Valitse .plume tai .plume_backup tiedosto (mikä tahansa Plume Creator versio).
import-plume-location = Kohdekansio
import-plume-name = Tiedostonimi
import-plume-name-placeholder = Projektin nimi
import-plume-will-create = Luo
import-plume-trash-warning = Romutetut / poistetut kohdat eivät muutu.
import-plume-cancel = Peruuta
import-plume-import = Tuo
import-plume-source-required = Valitse Plume-projektitiedosto
import-plume-source-missing = Tiedostoa ei ole olemassa
import-plume-source-not-file = Tämä polku ei ole tiedosto
import-plume-location-required = Valitse kohdekansio
import-plume-location-missing = Kansiota ei ole olemassa
import-plume-location-not-folder = Tämä polku ei ole kansio
import-plume-location-readonly = Tämä kansio ei ole kirjoitettava
import-plume-name-required = Anna tiedoston nimi
import-plume-name-exists = Tiedosto tällä nimellä on jo olemassa tässä. Tuo vahvistaa korvata
import-plume-overwrite-title = Korvaako nykyinen tiedosto?
import-plume-overwrite-text = “{ $name } Korvaa se tuodun projektin kanssa?
import-plume-manuscript-binder = Käsikirjoitus
import-plume-story-bible-binder = Tarinan raamattu
import-plume-progress-title = Tuodaan Plume-projektia...
import-plume-cancel-import = Peruuta
import-plume-cancelled = Tuo peruttu
import-plume-done = Tuotu { $imported ->
    [one] { $imported } kohta.
   *[other] { $imported } kohteita.
} { $skipped ->
    [one] { $skipped } roskattua tavaraa ei siirretty.
   *[other] { $skipped } roskia ei siirretty.
}
import-plume-open-now = Avaa nyt
import-plume-warnings = { $count ->
    [one] 1 asia ei voitu tuoda täsmälleen
   *[other] { $count } asioita ei voitu tuoda täsmälleen
}
import-plume-details = Yksityiskohdat
import-plume-warnings-title = Tuontivaroitukset
import-plume-error-title = Projektia ei voitu tuoda
import-plume-nested-too-deep = Hanketta ei tuotu: sen{ $part } Osa on pesitty enemmän kuin { $limit }  tasot syvä. Ei Plume Creator Projekti lähestyy sitä, ja lukeminen se voi kaatua Skribisto Mitään ei kirjoitettu.
import-plume-declares-entities = Hanketta ei tuotu: sen{ $part } Osa ilmoittaa XML-yksiköt, jotka eivät Plume Creator project does. Laajentaminen ne voivat ottaa enemmän muistia kuin tietokone on ja kaatuu Skribisto Mitään ei kirjoitettu.
import-plume-error-details = Yksityiskohdat

import-manuskript-title = Tuo Manuskript projekti
import-manuskript-close = Sulje
import-manuskript-source = Manuskript hanke
import-manuskript-source-hint = Valitse .msk tiedosto tai projektikansion (mikä tahansa Manuskript versio).
import-manuskript-source-file = Valitse...
import-manuskript-source-folder = Valitse tiedosto...
import-manuskript-location = Kohdekansio
import-manuskript-name = Tiedostonimi
import-manuskript-name-placeholder = Projektin nimi
import-manuskript-will-create = Luo
import-manuskript-cancel = Peruuta
import-manuskript-import = Tuo
import-manuskript-source-required = Valitse Manuskript projekti
import-manuskript-source-missing = Tätä tiedostoa tai kansiota ei ole olemassa
import-manuskript-location-required = Valitse kohdekansio
import-manuskript-location-missing = Kansiota ei ole olemassa
import-manuskript-location-not-folder = Tämä polku ei ole kansio
import-manuskript-location-readonly = Tämä kansio ei ole kirjoitettava
import-manuskript-name-required = Anna tiedoston nimi
import-manuskript-name-exists = Tiedosto tällä nimellä on jo olemassa tässä. Tuo vahvistaa korvata
import-manuskript-overwrite-title = Korvaako nykyinen tiedosto?
import-manuskript-overwrite-text = “{ $name } Korvaa se tuodun projektin kanssa?
import-manuskript-manuscript-binder = Käsikirjoitus
import-manuskript-story-bible-binder = Tarinan tietopankki
import-manuskript-characters-group = Merkit
import-manuskript-world-group = Maailma
import-manuskript-plots-group = Tontit
import-manuskript-project-info-note = Hankkeen tiedot
import-manuskript-summary-note = Yhteenveto
import-manuskript-importance-minor = Vähäinen
import-manuskript-importance-secondary = Toissijainen
import-manuskript-importance-main = Pääasiallinen
import-manuskript-progress-title = Tuodaan Manuskript projektia...
import-manuskript-cancel-import = Peruuta
import-manuskript-cancelled = Tuo peruttu
import-manuskript-done = { $imported ->
    [one] { $imported } tuote tuotu.
   *[other] { $imported } tuotuja tuotteita.
} { $revisions ->
    [0] { "" }
    [one] { $revisions } aiempi versio tuli sen mukana.
   *[other] Niiden mukana tuli { $revisions } aiempaa versiota.
}
import-manuskript-open-now = Avaa nyt
import-manuskript-warnings = { $count ->
    [one] Yksi asia tietää tästä tuonti
   *[other] { $count } asioita tietää tästä tuonti
}
import-manuskript-details = Yksityiskohdat
import-manuskript-warnings-title = Tietoja tästä tuonnista
import-manuskript-error-title = Projektia ei voitu tuoda
import-manuskript-nested-too-deep = Projektia ei tuotu: sen tiedostoa ...87{ $part }... on pesitty yli { $limit } syvemmälle. No Manuskript projekti tulee lähelle sitä, ja se voi kaatua Skribisto. Mitään ei kirjoitettu.
import-manuskript-folders-too-deep = Projektia ei tuotu: yksi sen tiedostoista istuu yli { $limit } kansiot syvä. No Manuskript projekti tulee lähelle sitä, ja lukeminen voi kaatua Skribisto. Mitään ei kirjoitettu.
import-manuskript-declares-entities = Projektia ei tuotu: sen tiedosto ...{ $part }... julistaa XML-yksiköt, joita ei ole Manuskript projektia. Niiden laajentaminen voisi ottaa enemmän muistia kuin tietokoneella on ja kaatuu Skribisto. Mitään ei kirjoitettu.
import-manuskript-error-details = Yksityiskohdat

import-target-open-title = Projekti on auki
import-target-open-text = “{ $name }... Skribisto. Tuominen sen tilalle korvaisi projektin, jota työstät, ja seuraava tallennuksesi kirjoittaisi projektin takaisin tuonnin päälle. Valitse toinen tiedostonimi tai sulje projekti ensin.
import-warnings-announcement = { $result } { $notice }.

export-title = Vie
export-close = Sulje
export-scope-label = Mitä?
export-format-label = Muotoilu
export-style-label = Tyyli
export-path-label = Tiedosto
export-browse = Selaa…
export-show-non-exportable = Näytä ei-viedä
export-choose-empty = Ei projektia, josta valita.
export-cancel = Peruuta
export-export = Vie
export-save-dialog-title = Vie tiedostoon
export-section-what = Mitä viedä?
export-section-style = Tyyli
export-section-destination = Määräpaikka
export-custom-selection = Oma valinta
export-selected-count = { $count } valittu
export-chip-chapters-none = Lukuja ei ole
export-chip-chapters-numbered = Luvut: numeroitu
export-chip-chapters-title = Luvut: ainoastaan otsikko
export-chip-chapters-both = Luvut: numero + otsikko
export-chip-scene-break-glyph = Kohtaus: { $glyph }
export-chip-scene-break-blank = Kohtauskatkos: tyhjä viiva
export-chip-scene-break-none = Kohtaustauko: ei mitään
export-chip-major-break-glyph = Suuri tauko { $glyph }
export-chip-major-break-blank = Suuri tauko: tyhjä rivi
export-chip-major-break-none = Suuri tauko: ei mitään
export-chip-major-break-same = Molemmat murtotasot ovat samanlaisia
export-chip-spacing-single = Riviväli: yksi
export-chip-spacing-onehalf = Välilyönti: 11⁄2
export-chip-spacing-double = Välilyönti: tupla
export-chip-notes-included = Mukaan lukien
export-chip-notes-excluded = Huomautuksia, joita ei oteta huomioon
export-format-docx = Sana
export-format-odt = LibreOffice
export-format-html = HTML
export-format-markdown = Markdown
export-format-djot = Djot
export-format-text = Teksti
export-format-latex = LaTeX
export-format-epub = EPUB
export-format-pdf = PDF
export-overwrite-title = Korvaako nykyinen tiedosto?
export-overwrite-text = “{ $name } Korvaatko sen?
export-progress-title = Viedä...
export-cancelled = Vienti peruttu
export-done = Viety { $count ->
    [one] { $count } kohta
   *[other] { $count } erää
}
export-open-file = Avaa
export-show-in-folder = Näytä kansiossa
export-error-title = Ei voitu viedä
export-error-details = Yksityiskohdat

backup-progress-start = Käynnistetään…
backup-progress-retention = Siivoan vanhoja varmuuskopioita.
backup-progress-done = Valmis
backup-progress-destination = Määräpaikka { $i }, { $n }
backup-details = Yksityiskohdat
backup-issues-title = Tukikysymykset
backup-failed-title = Varmuuskopiointi epäonnistui
backup-complete-prune-warning = Varmuuskopiointi valmis ({ $ok } tallennettu, { $skipped } jo nykyinen). Joitakin vanhoja varmuuskopioita ei voitu poistaa

search = Hae
search-query-placeholder = Hae…
search-replace-placeholder = Korvaa...
search-replace-toggle = Vaihda korvaa
search-replace-all = Korvaa kaikki
search-opt-case = Tulitikkukotelo
search-opt-whole-word = Koko sana
search-opt-diacritics = Täsmää aksentti
search-scope-body = Teksti
search-scope-title = Otsikko
search-scope-synopsis = Tiivistelmä
search-scope-label = Lisäteksti
search-facet-book = Kirjat
search-facet-part = Osat
search-facet-chapter = Luvut
search-facet-scene = Esiintymiset
search-facet-note = Huomautuksia
search-facet-folder = Kansiot
search-tip-case = Match case: kohtele isoja ja pieniä kuin eri, joten ...Elena... ja ...Elena.
search-tip-whole-word = Koko sana: ottelu vain kokonaisia sanoja, joten ... ei löydy sisällä Luokka.
search-tip-diacritics = Ottelu aksentit: kohtele korostuneet kirjaimet erillisenä, joten cafe... ei vastaa kahvila.
search-tip-body = Body: etsi proosa kohtauksia ja muistiinpanoja.
search-tip-title = Otsikko: etsi kansiokappaleiden otsikot.
search-tip-synopsis = Synopsis: etsi jokainen kirjoitus kohta.
search-tip-label = Etiketti: hae hakemiston otsikon alla olevaa lyhyttä muistiota.
search-tip-comment = Kommentit: etsi teksti kommentit ja niiden vastaukset. Kommentti on noin käsikirjoitus eikä osa sitä, joten se on oma kytkin, ja korvaavat lehdet kommentoida vastaa näppäimittäin kunnes rastitat niitä.
search-tip-book = Kirjat: kirjan astia ja sen alku / päättyy markkereita.
search-tip-part = Osat: osatason jakajat.
search-tip-chapter = Luvut: luvut, riippumatta siitä, miten hanke varastoi ne.
search-tip-scene = Kohtaukset: kohteita, jotka pitävät proosasi.
search-tip-note = Huomautuksia: vapaamuotoiset huomautukset.
search-tip-folder = Kansiot: tavallinen järjestää kansiot ja erottimet.
search-tip-paratext = Teksti, joka kuuluu kirjaan, mutta ei sen tarinaan: esipuhe, omistautuminen, jälkisana.
search-tip-preserve-case = Säilöntätapaus: korvaaja ottaa tapauksen se löysi, joten ...ELENA... tulee ...MARTA
search-tip-replace = Korvaa: näytä korvaava kenttä ja Korvaa kaikki.
search-error = Haku epäonnistui: { $message }
search-no-matches = Ei tulitikkuja
search-count =
    { $matches ->
        [one] { $matches } ottelu
       *[other] { $matches } matsia
    } { $items ->
        [one] { $items } asiakirja
       *[other] { $items } asiakirjaa
    }
search-count-truncated =
    { $matches ->
        [one] { $matches } ottelu
       *[other] { $matches } matsia
    } { $items ->
        [one] { $items } asiakirja
       *[other] { $items } asiakirjaa
    } (ainoastaan ensimmäiset tulokset)
search-occurrences = { $count }
search-field-body = Teksti
search-field-title = Otsikko
search-field-synopsis = Tiivistelmä
search-field-label = Lisäteksti
search-field-epigraph = Motto
search-field-comment = Kommentti
search-field-comment-reply = Vastaa
search-field-footnote = Alaviite
search-include-in-replace = Sisällytä Korvaa kaikki
search-collapse-all = Tuhoa kaikki tulokset
search-replace-here = Korvaa tämä
search-dismiss = Irtisanoutuminen tuloksista
search-undo-dismiss = Palauta viimeinen hylätty tulos
search-replace-nothing = (Ei mitään)
search-replace-confirm-title = Korvataanko kaikki osumat?
search-replace-confirm-text =
    Korvaa { $occurrences ->
        [one] Esiintyminen { $occurrences }
       *[other] { $occurrences } esiintymät
    } "{ $query }" ja "{ $replacement }" { $items ->
        [one] { $items } asiakirja
       *[other] { $items } asiakirjaa
    }? Voit peruuttaa tämän ilmoituksen jälkeen.
search-replace-done-title = Korvaa valmis
search-replace-done =
    { $occurrences ->
        [one] Esiintyminen { $occurrences }
       *[other] { $occurrences } esiintymät
    } korvattu { $items ->
        [one] { $items } asiakirja
       *[other] { $items } asiakirjaa
    }.
search-replace-done-skipped =
    { $occurrences ->
        [one] Esiintyminen { $occurrences }
       *[other] { $occurrences } esiintymät
    } korvattu { $items ->
        [one] { $items } asiakirja
       *[other] { $items } asiakirjaa
    }. { $skipped ->
        [one] { $skipped } kenttä
       *[other] { $skipped } kentät
    } hyppäsi ohi (vaihdettu haun jälkeen).
search-replace-undo = Kumoa
search-replace-failed-title = Korvaaminen epäonnistui
search-replace-skipped-title = Mikään ei ole korvattu.
search-replace-skipped-body =
    { $fields ->
        [one] Tämä teksti
       *[other] { $fields }
    } muuttunut haun jälkeen, joten { $fields ->
        [one] Se oli.
       *[other] Olivat.
    } jäi yksin. Etsi uudelleen nähdä, missä sanat ovat nyt.
search-replace-undo-failed-title = Peru
search-preview = Esikatselu
search-preview-empty = Valitse tulos esikatsellaksesi sitä tästä
search-preview-no-prose = Tässä ottelussa ei ole muokattavaa tekstiä
search-preview-prompt = Jos haluat nähdä esikatselun, tee haku.
search-preview-open-search = Hae projektista
search-preview-footnote-prompt = Tämä ottelu on alaviitteen omassa tekstissä. Avaa se Footnotes laiturilla nähdä ja muokata sitä.
search-preview-open-footnotes = Avaa alaviitteet

find-placeholder = Etsi tästä asiakirjasta...
find-count = { $current }, { $total }
find-no-results = Ei tuloksia
find-close = Sulje
find-previous = Edellinen ottelu (Shift+Enter)
find-next = Seuraava ottelu (Enter)
find-opt-case = Tulitikkukotelo
find-opt-whole-word = Koko sana
find-replace-toggle = Näytä tai piilota korvaaminen (Ctrl+R)
find-replace-placeholder = Korvaa...
find-replace = Korvaa
find-replace-all = Korvaa kaikki
find-preserve-case = Säilytä

dict-download-title = Ladataan { $name }...
dict-download-done = Asennettu { $name }
dict-download-failed = Ei voitu ladata { $name }: { $error }
dict-removed = Poistettu { $name }
dict-accept-first = Hyväksy lisenssi ennen lataamista { $name }
settings-dict-tab-installed = Asennettu
settings-dict-tab-get-more = Hanki lisää
settings-dict-tab-personal = Henkilökohtaiset sanat
dict-installed-empty = Tietokoneella ei ole vielä sanakirjoja.
dict-get-more-search = Etsi kieliä
dict-system-badge = järjestelmässäsi
dict-unusable-badge = Käyttämätön
dict-download-button = Lataa
dict-downloading = Ladataan...
dict-installed-label = Asennettu
dict-remove = Poista
dict-view-license = Näytä lisenssi
dict-approx-size = ~{ $size }
dict-personal-empty = Ei henkilökohtaisia sanoja tässä projektissa vielä.
dict-personal-add = Lisää
dict-personal-placeholder = Lisää sana.
dict-missing-toast = { $count ->
    [one] Tässä projektissa käytetään { $count } sanakirjaa, jota et ole asentanut
   *[other] Tässä projektissa käytetään { $count } sanakirjoja, joita et ole asentanut
}
dict-missing-action = Hae sanakirjoja
dict-license-title = { $name } lisenssi
dict-license-accept = Hyväksy lataus
dict-license-cancel = Peruuta
dict-license-close = Sulje
dict-add-button = Lisää sanakirja...
dict-add-title = Lisää sanakirja
dict-add-close = Sulje
dict-add-name = Nimi
dict-add-name-placeholder = esim. latinankielinen sanakirja
dict-add-code = Kielikoodi
dict-add-code-placeholder = esim. la tai fr-FR-x-custom
dict-add-code-hint = Valitset tämän asiakirjan kieleksi.
dict-add-aff = Liitetiedosto (.aff)
dict-add-dic = Sanaluettelo (.dic)
dict-add-submit = Lisää
dict-add-cancel = Peruuta
dict-add-name-required = Anna sanakirjalle nimi.
dict-add-code-required = Syötä kielikoodi.
dict-add-code-invalid = Käytä vain kirjaimia, numeroita ja - _ .
dict-add-code-reserved = Se on sisäänrakennettu sanakirjakoodi. Valitse toinen.
dict-add-file-required = Valitse tiedosto.
dict-add-file-missing = Tiedostoa ei ole olemassa.
dict-add-code-taken = Sen koodin sanakirja on jo asennettu.
dict-add-done = Lisätty { $name }
dict-add-unusable = Nämä tiedostot eivät ole käyttökelpoinen sanakirja: { $error }
dict-add-failed = Sanakirjaa ei voitu lisätä: { $error }
inspector-export = Vie
inspector-exportable = Sisällytä vientiin
ctx-number = N&umero tämän luvun
ctx-unnumber = &Älä numeroi tätä lukua
inspector-numbering = Numerointi
inspector-numbered = Numeroitu
inspector-numbered-tip = Tämä luku ottaa paikkansa kirjan numeroinnissa ja sammuttaa sen esipuhetta, epilogia tai interludeja varten.
inspector-numbered-tip-more = Numeroimaton luku pysyy kirjassa juuri sellaisena kuin se oli: sen otsikko, sen proosa ja sen sanamäärä ovat koskemattomia. Se yksinkertaisesti tulostaa mitään numeroa, ja ei käytä yksi: luku prelogin jälkeen on luku yksi, ei luku kaksi. Sen jättäminen pois vienti sen sijaan on eri kytkin, edellä, ja että yksi poistaa luvun kirjan kokonaan.
inspector-apply-to-children = Levitä lapsille
inspector-milestone = Välitavoitepäivämäärä
inspector-milestone-none = Ei päivämäärää
inspector-milestone-clear = Tyhjennä
inspector-tags = Tunnisteet
inspector-aliases = Muut nimet
inspector-dict-language = Kieli
inspector-apply-language-to-children = Levitä kieltä lapsille
settings-page-language = Kieli
settings-field-dict-language = Kielet
dict-tradeoff-hint = Jokainen ylimääräinen kieli hyväksyy enemmän sanoja, joten vähemmän virheitä jää kiinni.
lang-inherit-hint = Peritty: tämä kohtaus käyttää kirjan tai projektin kieliä.
lang-pill-list = Kielet
lang-pill-add = Lisää kieli
lang-pill-remove = Poista { $name }
lang-pill-mute = Sammuta loitsuntarkistus { $name }:lle
lang-pill-unmute = Käännä loitsutarkistus päälle { $name }

settings-page-personal-dictionary = Henkilökohtainen sanakirja
settings-user-dict-desc = Tähän lisäämiäsi sanoja käsitellään oikein kirjoitettuna kaikkialla sovelluksessa, eikä niitä ole merkitty enää koskaan.
settings-user-dict-add-placeholder = Lisää sana.
settings-user-dict-add = Lisää sana
settings-user-dict-search = Suodatinsanat
settings-user-dict-count = { $count ->
    [one] { $count } sana
   *[other] { $count } words
}
settings-user-dict-rename = Nimeä uudelleen
settings-user-dict-remove = Poista
settings-user-dict-empty = Ei sanoja vielä. Lisää sanoja oikoluku pitäisi sivuuttaa, tai tuoda lista.
settings-user-dict-import = Tuo...
settings-user-dict-export = Vie...
settings-user-dict-import-tip = Lisää sanoja tavallisesta tekstitiedostosta, yksi sana riviä kohti. Olemassa olevat sanat säilytetään.
settings-user-dict-export-tip = Tallenna koko lista tavalliseen tekstitiedostoon, yksi sana per rivi.
settings-user-dict-txt-filter = Tekstitiedostot
settings-user-dict-duplicate = Jo sanakirjassa
settings-user-dict-imported = Tuotu { $count ->
    [one] { $count } sana
   *[other] { $count } words
} ({ $duplicates }).
settings-user-dict-import-failed = Sanaluetteloa ei voitu lukea: { $error }
settings-user-dict-exported = Tallennettu { $count ->
    [one] { $count } sana.
   *[other] { $count } sanoja.
}
settings-user-dict-export-failed = Sanaluetteloa ei voitu tallentaa: { $error }

settings-page-text-replacements = Tekstinkorjaukset
settings-text-repl-desc = Korvaa pikakirjoitus koko tekstin kanssa kirjoittaessasi: "btw" muuttuu " muuten" sillä hetkellä, kun kirjoitat tilan tai välimerkit.
settings-text-repl-enable = Käytä tekstikorjauksia tässä projektissa
settings-text-repl-disabled-hint = Laita tämä päälle määritellä pikakirjoitus, joka laajenee kuin kirjoitat.
settings-text-repl-add = Lisää sääntö
settings-text-repl-trigger-placeholder = Lyhenne
settings-text-repl-replacement-placeholder = Mitä siitä tulee
settings-text-repl-added = Lisätty "{ $trigger }"
settings-text-repl-duplicate = "{ $trigger }" on jo sääntö
settings-text-repl-filter = Suodatussäännöt
settings-text-repl-count = { $n ->
    [one] 1 sääntö
   *[other] { $n } säännöt
}
settings-text-repl-row-enabled = Käytä tätä sääntöä
settings-text-repl-delete = Poistetaan kohdan { $trigger } sääntö.
settings-text-repl-deleted = Poistetaan otsikkoa "{ $trigger }" koskeva sääntö.
settings-text-repl-empty = Ei vielä sääntöjä.
settings-text-repl-csv-filter = CSV tiedostoa
settings-text-repl-import = Tuo...
settings-text-repl-export = Vie...
settings-text-repl-imported = Tuotu { $added }, ohitettu { $skipped }
settings-text-repl-exported = Viety { $n ->
    [one] 1 sääntö
   *[other] { $n } säännöt
}

editor-menu-no-suggestions = Ei ehdotuksia
editor-menu-add-to-dictionary = Lisätään{ $word }Sanakirjaan
editor-menu-add-words-to-dictionary = Lisää valitut sanat sanakirjaan
editor-dict-added = Lisätty{ $word }Sanakirjaasi.
editor-dict-added-multi = { $count ->
    [one] Lisätty { $count } sanakirjaasi.
   *[other] Lisätty { $count } sanoja sanakirjaasi.
}
toast-undo = Kumoa

undo-superseded-title = Se askel ei ole enää viimeinen.
undo-superseded-body = Jotain muuta on muuttunut tässä projektissa vuodesta. Undo ottaisi, että takaisin sen sijaan, joten se ei ole. Mikään tämä vaihe ei ole käännetty. Käytä Muokkaa Peru astua takaisin historian itse.
undo-failed = Peru epäonnistui: { $error }

menu-edit = &Muokkaa
menu-edit-undo = Ku&moa
menu-edit-redo = Tee &uudelleen
menu-edit-undo-target = &Kumoa: { $target }
menu-edit-redo-target = &Tee uudelleen: { $target }
shortcut-name-edit-undo = Kumoa
shortcut-name-edit-redo = Tee uudelleen
undo-target-typing = kirjoittaminen
undo-target-project = viimeinen muutos tähän projektiin
prose-history-reset-title = Kirjoitushistorian nollaus
prose-history-reset-body = { $count ->
    [one] Yksi avoin kohtaus palautettiin aikaisemmasta tilasta, joten sen kirjoitushistoria ei enää päde.
   *[other] { $count } avoimia kohtauksia palautettiin aikaisemmasta tilasta, joten niiden kirjoitushistoria ei enää päde.
}
undo-target-trash = roskakoriin siirtyminen
undo-target-restore = Palautus roskiksesta
undo-target-delete-forever = poistetaan lopullisesti
undo-target-replace-all = korvataan koko hanke
undo-target-import = Asiakirjan tuonti
undo-target-duplicate = päällekkäisyys
undo-target-move = muutto
undo-target-merge = yhdistää kaksi kohtausta
undo-target-split = kohtauksen jakaminen
undo-target-promote = tyypin muuttaminen
undo-target-tidy-titles = Siivotaan luvun otsikot
undo-target-import-tags = tuontietiketit
undo-target-import-templates = Tuo muistiinpanomallit
undo-target-create = luo
undo-target-remove = poistetaan
undo-target-rename = nimeäminen
undo-target-edit = muokkaus
menu-edit-find = &Etsi…
menu-edit-find-next = Et&si seuraava
menu-edit-find-prev = E&tsi edellinen
menu-edit-replace = Etsi &ja korvaa...
menu-edit-find-in-project = Etsi &projekti...
menu-edit-replace-in-project = K&orvaa projektissa...
undo-frozen = Peru (Aina eteenpäin on päällä)
redo-frozen = Redo (Aina eteenpäin on käynnissä)
menu-edit-cut = &Leikkaa
menu-edit-copy = &Kopioi
menu-edit-paste = L&iitä
menu-edit-paste-plain = Liit&ä ilman muotoilua
menu-edit-select-all = &Valitse kaikki
shortcut-name-edit-cut = Leikkaa
shortcut-name-edit-copy = Kopioi
shortcut-name-edit-paste = Liitä
shortcut-name-edit-paste-plain = Liitä ilman muotoilua
shortcut-name-edit-select-all = Valitse kaikki

titlebar-spellcheck-on = Oikoluku on päällä. Klikkaa lopettaaksesi tarkistuksen (F7)
titlebar-spellcheck-off = Oikoluku on pois päältä. Klikkaa tarkistaaksesi uudelleen (F7)
menu-spellcheck = &Tarkista oikeinkirjoitus
menu-comments = &Kommentit
settings-page-spellcheck = Oikolukutarkistus
settings-group-spellcheck = Oikolukutarkistus
settings-spellcheck-enabled = Tarkista kirjoitustapani
settings-spellcheck-hint = Alleviivaa sanoja, joita ei ole asennettu sanakirja tietää. Tämän kytkeminen pois päältä lopettaa kaikki tarkastukset, jokaisessa projektissa, kunnes käynnistät sen. Lopettaa yhden kielen tarkistaminen vain, tyhjentää sen tarkistusmerkki Kieli alalla teoksen tai kohteen.

menu-trash = R&oskakori
trash-title = Roskakori
trash-empty-state = Roskat ovat tyhjiä.
trash-empty-button = Tyhjennä roskakori…
trash-restore = &Palauta
trash-restore-to = &Palauta...
trash-delete-forever = &Poista ikuisesti
trash-restored-ok = { $count } palautettu.
trash-restore-error = Ei voitu palauttaa: { $error }
trash-restore-orphaned = Tämän esineen alkuperäinen paikka on poissa. Valitse missä palauttaa se.
trash-restore-no-project = Mikään projekti ei ole auki, joten mitään ei ole palautettavaa.
trash-delete-no-project = Mikään projekti ei ole auki, joten mitään poistettavaa ei ole.
trash-empty-confirm-title = Tyhjentää roskat?
trash-empty-confirm-text = { $count ->
    [one] { $count } romutettu merkintä poistetaan pysyvästi.
   *[other] { $count } Tuhotut merkinnät poistetaan pysyvästi.
} Jättävät roskat lopullisesti.
trash-emptied-title = Tyhjennetty jäte
trash-emptied-body = Kaikki roskiksesta poistettiin pysyvästi.
trash-empty-no-project = Mikään projekti ei ole auki, joten ei ole roskaa tyhjennettäväksi.
trash-delete-forever-confirm-title = Poistaa ikuisesti?
trash-delete-forever-confirm-text = { $count ->
    [one] { $count } kohta poistetaan pysyvästi.
   *[other] { $count } kohdat poistetaan pysyvästi.
} Jättävät roskat lopullisesti.
trash-deleted-title = Poistettu lopullisesti
trash-deleted-body = { $count } pysyvästi poistettu.
trash-undo = Kumoa
trash-restore-picker-title = Palauta...
trash-restore-to-confirm-title = - Palauttaa tänne?
trash-restore-to-confirm-text = Palauttaa "{ $item }" muotoon "{ $destination }"?
trash-restore-picker-restore-here = Palauta tähän
trash-restore-picker-cancel = Peruuta
trash-restore-picker-empty = Ei kansioita vielä.
trash-banner-title = Tämä kohta on roskakorissa.
trash-banner-description = Se ei näy ääriviivoissa tai viennissä, ennen kuin palautat sen.
trash-banner-restore = Palauta…
trash-tab-tooltip = Roskakorissa

settings-styles-sheet-yes = Kyllä
settings-styles-sheet-no = Ei.
settings-styles-sheet-font = Kirjasin
settings-styles-sheet-indent = Ensimmäinen luetelmakohta
settings-styles-sheet-para-spacing = Kohtaväli
settings-styles-sheet-page = Sivun koko
settings-styles-sheet-margins = Marginaalit (ylä/oikea/ala/vasen)
settings-styles-sheet-title-page = Kirjan otsikkosivu
settings-styles-sheet-heading-language = Suuntakieli
settings-styles-sheet-auto = Tekstin mukaan
settings-styles-sheet-digits = Numerot
settings-styles-sheet-direction = Tekstin suunta
settings-styles-sheet-formats = Muotoilu
settings-styles-sheet-all-formats = Kaikki

settings-styles-editor-group = Tyylieditori
settings-styles-editor-missing = Se tyyli ei ole enää saatavilla.

settings-styles-page-letter = Kirje
settings-styles-digits-western = Läntinen (0...)
settings-styles-digits-eastern-arabic = Itä-Arabia (...)
settings-styles-direction-ltr = Vasemmalta oikealle
settings-styles-direction-rtl = Oikealta vasemmalle

format-dock-title = Muotoilu
format-panel-empty = Aseta kohdistin kohtaukseen, muistiinpano, tai synopsis nähdä muotoilu vaihtoehtoja.
format-group-marks = Teksti
format-group-block = Kohdat
format-group-lists = Luettelot
format-group-tables = Taulukko
format-group-breaks = Kohtaus taukoja
format-superscript = Yläindeksi
format-subscript = Alaindeksi
format-link = Linkki…
link-dialog-insert-title = Lisää linkki
link-dialog-edit-title = Muokkaa linkkiä
link-dialog-text-label = Teksti
link-dialog-text-placeholder = sanat, jotka lukija näkee
link-dialog-url-label = Linkit
link-dialog-url-placeholder = esimerkiksi.com
link-dialog-insert = Lisää
link-dialog-apply = Käytä
link-dialog-cancel = Peruuta
link-dialog-remove = Poista linkki
link-scheme-refused = Vain web- ja sähköpostilinkkejä voi avata. { $url } jäi yksin.
menu-format-link = L&inkki…
format-clear = Tyhjennä muotoilu
format-heading = Otsakkeen taso
format-heading-normal = Tavallinen teksti
format-heading-1 = Otsikko 1
format-heading-2 = Otsikko 2
format-heading-3 = Otsikko 3
format-heading-4 = Otsikko 4
format-heading-5 = Otsikko 5
format-heading-6 = Otsikko 6
format-align-left = Aseta vasemmalle
format-align-center = Keskitä
format-direction-rtl = Oikealta vasemmalle -kappale
format-blockquote = Lainauslohko
format-list-bullet = Luettelomerkkiluettelo
format-list-numbered = Numeroitu luettelo
format-indent = Lisää sisennystä
format-outdent = Vähennä sisennystä
format-table-insert = Lisää taulukko
format-table-row-above = Lisää rivi edellä
format-table-row-below = Lisää alla oleva rivi
format-table-col-before = Lisää sarake ennen
format-table-col-after = Lisätään sarakkeen jälkeen
format-table-row-delete = Poista rivi
format-table-col-delete = Poista sarake
format-table-remove = Poista taulukko

menu-format-marks-bold = Li&havointi
menu-format-marks-italic = &Kursivointi
menu-format-marks-underline = &Alleviivaus
menu-format-marks-strike = &Yliviivaus
menu-format-marks-superscript = Yläin&deksi
menu-format-marks-subscript = Alaind&eksi
menu-format-marks-clear = Tyh&jennä muotoilu
menu-format-heading = &Otsikko
menu-format-heading-normal = Ta&vallinen teksti
menu-format-heading-1 = &Otsikko 1
menu-format-heading-2 = O&tsikko 2
menu-format-heading-3 = Ot&sikko 3
menu-format-heading-4 = Ots&ikko 4
menu-format-heading-5 = Otsi&kko 5
menu-format-heading-6 = Otsikko &6
menu-format-alignment = &Tasaus
menu-format-align-left = &Aseta vasen
menu-format-align-center = &Keskitä
menu-format-direction = &Suunta
menu-format-direction-auto = &Automaattinen
menu-format-direction-ltr = &Vasemmalle oikealle
menu-format-direction-rtl = &Oikealta vasemmalle
menu-format-blockquote = Lai&nauslohko
menu-format-lists = &Luettelot
menu-format-list-bullet = L&uettelomerkkiluettelo
menu-format-list-numbered = &Numeroitu luettelo
menu-format-indent = &Lisää sisennystä
menu-format-outdent = &Vähennä sisennystä
menu-format-table = Ta&ulukko
menu-format-table-insert = &Lisää taulukko
menu-format-table-2x2 = &2 x 2
menu-format-table-3x3 = &3 x 3
menu-format-table-4x4 = &4 x 4
menu-format-table-row-above = Lisää &rivin yläpuolelle
menu-format-table-row-below = Li&sää rivin alle
menu-format-table-col-before = Lis&ää sarake ennen
menu-format-table-col-after = Lisää s&arake jälkeen
menu-format-table-row-delete = &Poista rivi
menu-format-table-col-delete = P&oista sarake
menu-format-table-remove = Po&ista taulukko

menu-format-typo-marks = &Muotoillaan merkkiä
menu-format-typo-nbsp = &Ei taukoa
menu-format-typo-nbhyphen = &Ei-break- väliviiva
menu-format-typo-soft-hyphen = &Pehmeä yhdysmerkki
menu-format-typo-nnbsp = &Raja-avaruus
menu-format-typo-zwsp = &Nollaleveysalue
menu-format-typo-word-joiner = &Sanojen yhdistäjä


menu-about = &Noin Skribisto...
about-title = Noin Skribisto
about-version = Versio { $version }
about-update-available = Versio { $version } on saatavilla
about-update-see-notes = Katso, mikä muuttui.
about-update-flathub = Flathub pitää tämän kopion ajan tasalla.
about-update-distro = Jakelusi pitää tämän kopion ajan tasalla.
about-tagline = Teksilo-työkalupatilla kirjoitettu romaanikirjallisuussovellus pitkälle fiktiolle.
about-license = Julkaistu GNU General Public License, versio 3.
about-copyright = © 2026 Cyril Jacquet
about-close = Sulje

native-menu-about = Noin { $app }
native-menu-hide = Piilota { $app }
native-menu-quit = Lopeta { $app }
native-menu-settings = Asetukset
native-menu-window = Ikkuna
native-menu-minimize = Pienennä
native-menu-zoom = Suurenna

window-title = { $title } - { $app }
window-title-numbered = { $title } - { $app } (Ikkuna { $n })
window-title-empty = { $app }

first-run-window-title = Tervetulotoivotukset
first-run-title = Perustettu { $app }
first-run-body = { $app } pitää asetukset erillään Skribisto:sta, joten se alkaa tyhjänä. Mielipiteesi, viimeaikaiset projektit, sanakirjat ja ikkunan ulkoasu voidaan kopioida nyt.
first-run-from = Kopioi
first-run-to = Kopioi
first-run-copy-note = Mitään ei siirretä tai poisteta. Skribisto pitää kaikki omat asetukset ja jatkaa työtään juuri niin kuin ennenkin.
first-run-import = Tuo asetukset
first-run-start-fresh = Aloita alusta
first-run-import-failed = Joitakin asetuksia ei voitu tuoda: { $error }

settings-page-punctuation = Välimerkkien oletusarvot
settings-group-punctuation = Älykäs välimerkki
settings-page-work-punctuation = Välimerkit
settings-punctuation-override = Anna tälle projektille omat välimerkit
settings-punctuation-override-hint = Off: Projekti noudattaa sovelluksen mieltymystä. Nämä säännöt kulkevat sisällä .skrib, joten toinen tekijä avaa tiedoston kirjoittaa samalla typografialla.
settings-punctuation-dashes = Muuta -- en dash, -- osaksi em dash
settings-punctuation-ellipsis = Muuta ... osaksi ellipsis
settings-punctuation-quotes = Curlin lainausmerkit ja heittomerkit
settings-quote-style = Noteerausmerkit
settings-quote-style-locale = Kielivirhe
settings-quote-style-curly = - Kaksinkertainen.
settings-quote-style-curly-single = ...
settings-quote-style-guillemets = Guillemets...
settings-quote-style-low-high = Hitaasti.
settings-punctuation-spacing = Tilaa ennen ;: !?
settings-punctuation-spacing-hint = Ranskalainen typografia asettaa kapea ei-break tilaa ennen; ! ? ja täysi ennen:. Se koskee vain tekstiä kirjoitettu ranskaksi. Tila sisällä guillemets .
settings-punctuation-sample = Sinun kielesi antaa
settings-punctuation-app-hint = Mitä jokainen projekti tekee, ellei se ota omia sääntöjä Projekti ...
settings-punctuation-dialogue = Avaa kappale, jonka nimi on "-" dialogiviivalla
settings-punctuation-dialogue-hint = Kielet, jotka leimaavat puheen viivalla eikä lainausmerkeillä: ranska, espanja, venäjä ja muut. Se tulee vain kappaleen alussa.

statusbar-go-to = Mene...
go-to-placeholder = Etsi kansiosta
go-to-no-matches = Mikään esine ei täsmää etsintöihin.
menu-go-to = &Mene...

settings-page-distraction-free-themes = Häiriöttömiä teemoja
settings-themes-builtin = Sisäänrakennetut teemat
settings-themes-builtin-badge = Sisäänrakennettu
settings-themes-user = Aihealueeni
settings-themes-editor-group = Muokkaa teemaa
settings-themes-editor-empty = Valitse teema otsikon "Teemani" alla muokataksesi sitä.
settings-themes-use = Käyttö
settings-themes-duplicate = Monista
settings-themes-edit = Muokkaa
settings-themes-delete = Poista
settings-themes-export = Vie...
settings-themes-import = Tuo...
settings-themes-copy-suffix = kappale
settings-themes-json-filter = Teema (JSON)
settings-themes-imported = Teema tuotu
settings-themes-import-failed = Tätä teemaa ei voitu tuoda
settings-themes-exported = Teema viety
settings-themes-export-failed = Tätä teemaa ei voitu viedä
settings-themes-low-contrast = alhainen kontrasti
settings-themes-low-contrast-band = Korosta piilottaa tekstin
settings-themes-field-name = Nimi
settings-themes-field-paper = Sivu
settings-themes-field-ink = Teksti
settings-themes-field-general = Tausta
settings-themes-field-widget-text = Ohjausnauhan teksti
settings-themes-field-caret-band = Korosta ympärillesi

statusbar-focus-settings = Häiriöttömät asetukset
statusbar-focus-manage-themes = Hallitse teemoja...

comments-title = Kommentit
comments-document-title = Tämä asiakirja
comments-empty-project = Ei vielä kommentteja tässä hankkeessa.
comments-empty-document = Ei kommentteja tästä asiakirjasta.
comments-filter-all = Kaikki
comments-filter-open = Avaa
comments-filter-resolved = Ratkaistu
comments-filter-orphaned = Ilman kohdetekstiä
comments-status-open = Avaa
comments-status-resolved = Ratkaistu
comments-status-orphaned = Kohdeteksti puuttuu
comments-orphan-snippet = (kommentoitu teksti on poissa)
comments-status-unplaced = Ei tekstiä
comments-unplaced-snippet = (ei ankkuroitu mihinkään tekstiin)
comments-reply-count = { $count ->
    [one] { $count } Vastaus
   *[other] { $count } vastausta
}
comments-menu-resolve = Ratkaise
comments-menu-reopen = Avaa uudelleen
comments-menu-delete = Poista
comments-sort-document = Asiakirjajärjestyksessä
comments-sort-newest = Uusimmat ensin
comments-sort-reader = Lukija ryhmitteli
comments-unsigned-author = Allekirjoittamaton
comments-agreement = { $count ->
    [one] { $count } lukija
   *[other] { $count } lukijaa
}
comments-menu-add = Lisää kommentti
comments-menu-add-paragraph = Kommentoi tätä kohtaa
overview-col-comments = Kommentit
overview-col-total-comments = Huomioita yhteensä
comments-card-placeholder = Kirjoita kommentti...
comments-card-unknown-author = Tuntematon tekijä
comments-unsigned-toast = Kommenttisi ovat signeeraamattomia: sinulle ei ole asetettu nimeä tällä tietokoneella.
comments-unsigned-action = Asettakaa nimenne
comments-card-reply = Vastaa
comments-card-reply-placeholder = Vastaa.
comments-card-actions = Kommenttitoimet
comments-reply-actions = Vastaa
comments-menu-delete-reply = Poista vastaus
comments-menu-delete-all = Poista kaikki kommentit tästä
comments-deleted-toast = Kommentti poistettu
comments-reply-deleted-toast = Vastaa poistettu
comments-deleted-all-toast = { $count ->
    [one] { $count } kommentti poistettu
   *[other] { $count } huomautukset poistettu
}
comments-undo = Kumoa

analysis-segment = Analyysi
analysis-scope-book = Analysoin tätä kirjaa
analysis-run = Suorita analyysi
analysis-stale = Muuttunut tämän jälkeen
analysis-not-run = Ei vielä analysoitu.
analysis-running = Luen käsikirjoitusta...
analysis-failed = Analyysi ei voinut päättyä.
analysis-no-scenes = Ei kohtauksia tässä kirjassa vielä.

analysis-shape = Muoto

analysis-arrivals = Saapuminen
analysis-arrivals-explainer = Miten teksti on saavuttanut tämän projektin, kun se on ollut auki. Se kertoo, mikä reitti merkkiä tuli alas, eikä mitään siitä, kuka kirjoitti ne: kirjailija, joka luonnostelee muualla ja pastat on mested, ja joka sanelee on sanellut. Ei ole mitään määrää tavoitella täällä.
analysis-arrivals-scope = Tämä koko projekti, ei vain tämä kirja.
analysis-arrivals-session = Kun avasit sen, projektin sulkeminen aloittaa laskun uudelleen.
analysis-arrivals-nothing = Tekstiä ei ole vielä saapunut tähän istuntoon.
analysis-arrivals-typed = Tyyppi
analysis-arrivals-pasted = Liitetty
analysis-arrivals-dictated = Diktoitu
analysis-arrivals-imported = Tuotu
analysis-arrivals-programmatic = Lisätty sinulle
analysis-arrivals-count = { $count ->
    [one] { $count } merkki
   *[other] { $count } merkkiä
}
analysis-arrivals-none = ei yhtään

analysis-words-per-scene = Sanat per kohtaus
analysis-median-words = Tämän kirjan keskikohtaus kulkee. { $count ->
    [one] { $count } sana.
   *[other] { $count } sanoja.
}
analysis-median-line = Mediaani: { $count ->
    [one] { $count } sana
   *[other] { $count } words
}
analysis-dialogue = Vuoropuhelu
analysis-dialogue-unsupported = Vuoropuhelua ei vielä mitata tämän kielen osalta.

analysis-footnote-words = Alaviitesanat
analysis-footnote-words-count = { $count ->
    [one] { $count } sana on tämän kirjan alaviitteitä, pidetään erillään käsikirjoituksen yhteensä.
   *[other] { $count } sanat ovat tämän kirjan alaviitteitä, pidetään erillään käsikirjoituksen yhteensä.
}
analysis-footnote-words-pending = Lasken alaviitteet...

analysis-ignore-empty = Älä välitä teksteistä, joita ei ole vielä kirjoitettu
analysis-empty-hidden = { $count } tyhjä { $count ->
        [one] teksti
       *[other] tekstit
    } piilotettu.
analysis-all-texts-empty = Kaikki tämän kirjan tekstit ovat yhä tyhjiä.

binder-filter-count = { $shown }, { $total }
binder-filter-clear = Tyhjennä
binder-filter-none = Mikään ei täsmää.{ $query }”.

settings-paratext-intro = Etu- ja taka-asiasta voi aloittaa uudella projektilla. Jokainen rakenne kuuluu julkaisuperinteeseen, ja sen sivun otsikot on kirjoitettu perinteen omalla kielellä. Nimeä ne vapaasti, kun projekti on luotu.
settings-paratext-structures = Rakenteet
settings-paratext-broken = Ei voitu lukea
settings-paratext-edit = Muokkaa
settings-paratext-duplicate = Monista
settings-paratext-delete = Poista
settings-paratext-new = Uusi rakenne
settings-paratext-save = Tallenna
settings-paratext-editor-hint = Nimi, sivut, jotka luodaan ennen käsikirjoitusta, ja sivut, jotka luodaan sen jälkeen. Jos ne päätyvät, on sinun: ne ovat tavallisia kohteita, kun projekti on olemassa.


image-insert = Lisää &kuva...
image-no-project = Avaa projekti ennen kuvan lisäämistä.
image-choose-title = Valitse kuva
image-filter-label = Kuvat
image-large-title = Tämä on suuri kuva
image-large-text =
    { $name } ilu { $megapixels } megapikselit ({ $width }×{ $height }).
    Pidä se sellaisena kuin se on tallentaa alkuperäisen tiedoston projektissa: se matkustaa
    jokaisen varmuuskopioita ja jokaisen viennin. Optimoimalla tallentaa pienemmän kopion
    Sen sijaan jopa 2560 pikseliä sen pisimmällä puolella.
image-large-keep = Säilytä alkuperäinen
image-large-downscale = Optimoi
image-large-remember = Tee tämä tästä lähtien, älä kysy uudestaan.
image-not-recorded = Kuva tallennettiin, mutta sitä ei voitu tallentaa projektiin.
image-describe-title = Kuvaile kuvaa
image-describe-explain = Mitä kuva näyttää, lukijalle, joka ei näe sitä. Se ei ole osa käsikirjoitusta: sitä ei koskaan lasketa, tutkitaan tai viedään proosana.
image-describe-placeholder = majakka harmaata taivasta vasten
image-resize-title = Muuta kuvan kokoa
image-resize-explain = Prosentuaalinen osuus sen koosta on nyt näkyvissä. 100 jättää sen ennalleen.
image-resize-invalid = Anna numero välillä 1 ja 1000.
image-menu-describe = &Kuvaile kuvaa...
image-menu-resize = &Muuta kuvan kokoa...
image-menu-reset-size = &Alkuperäinen koko

cover-choose = &Kirjankannet...
cover-clear = &Poista kansi
cover-choose-title = Valitse kansi
cover-set = Kansi on asetettu.
cover-cleared = Kansi on poistettu. Kuva on yhä projektissa.
card-label = Lisäteksti
card-exportable = Viety
card-numbered = Numeroitu
card-created = Luotu
card-modified = Muokattu
card-yes = kyllä
card-no = ei
card-type = Tyyppi
card-position = Sijainti
card-children = Lapset
card-goal = Tavoite
card-words = Sanat
card-synopsis = Tiivistelmä
card-point-of-view = Näkökulma
card-aliases = Muut nimet

export-orphan-footnotes-title = Alaviitteet ilman viitettä
export-comments-dropped =
    { $count ->
        [one] Yhtä huomautusta ei voitu sisällyttää vietyyn tekstiin, ja se jätettiin pois.
       *[other] { $count } huomautuksia ei voitu sisällyttää vietyyn tekstiin, ja ne jätettiin pois.
    }
export-orphan-footnotes =
    { $count ->
        [one] Käsikirjoituksessa ei enää viitata yhteen alaviitteeseen. Sen tekstiä ei näy vientikirjassa.
       *[other] { $count } alaviitteitä ei enää mainita missään käsikirjoituksessa.
    }


menu-footnotes = &Alaviitteet
footnotes-title = Alaviitteet
footnotes-insert = Lis&ätään alaviite
footnotes-empty = Ei vielä alaviitteitä. Laita kohdistin kohtaukseen, sitten käytä + yläpuolella, tai Ctrl+Alt+F.
footnotes-filter-all = Kaikki
footnotes-filter-document = Tämä asiakirja
footnotes-filter-orphaned = Ilman kohdetekstiä
footnotes-orphaned = Mikään ei enää viittaa tähän viestiin.
footnotes-untitled-home = Nimetön
footnotes-body-placeholder = itse viesti
footnotes-insert-tooltip = Lisätään kursoriin alaviite (Ctrl+Alt+F)
footnotes-actions = Alaviitteet
footnotes-delete = &Poista huomautus ja sen viite
footnotes-no-project = Avaa projekti ennen alaviitteen lisäämistä.
footnotes-no-caret = Laita kohdistin kohtauksen teksti lisätä alaviitteen.
footnotes-not-created = Tätä alaviitettä ei voitu lisätä hankkeeseen.
footnotes-deleted-toast = Huomautus poistettu
footnotes-undo-delete = Kumoa

import-document-title = Tuontiasiakirjat
import-document-close = Sulje
import-document-step-files = Tiedostot
import-document-step-review = Uudelleentarkastelu
import-document-step-destination = Määräpaikka
import-document-step-reconcile = Yhdistä
import-document-reconcile-hint = Jotkut näistä ovat niitä, joita sinulla on jo. Kerro, mitä pitäisi tapahtua jokaiselle.
import-document-reconcile-all-new = Mikään tässä tiedostossa ei vastaa projektiasi: jokainen kohde lisätään uudeksi.
import-document-duplicate-returns = Nämä tiedostot ovat useampi kuin yksi kopio samasta käsikirjoituksesta, joka tulee takaisin. Tuo ne yksi kerrallaan: täsmäävät vain yhden palanneen kopion kanssa projektiasi vastaan, joten useiden tuominen yhteen lisäisi kirjan toisen kopion sen sijaan, että yhdistäisi sitä.
import-document-hunk-added = Ota tämä uusi kappale
import-document-hunk-removed = Poista tämä kappale
import-document-hunk-changed = Ota tämä uusiksi.
import-document-col-stray-prose = Teksti
import-document-stray-as-paratext = Säilytä paratekstina
import-document-stray-discard = Pudota teksti
import-document-col-current = Projekti
import-document-col-incoming = Tässä tiedostossa
import-document-col-status = Tila
import-document-col-action = Mitä tehdä?
import-document-status-identical = Sama
import-document-status-editor-edited = Lukija muokkasi
import-document-status-you-edited = Sinä muokkasit
import-document-status-conflict = Molemmat muokkasivat
import-document-status-different = Eroaa
import-document-status-new = Uusi
import-document-status-missing = Ei tässä tiedostossa
import-document-status-moved = Siirretty
import-document-action-comments-only = Vain kommentit
import-document-action-take-import = Käytä tätä versiota
import-document-action-keep-current = Säilytä omani
import-document-action-create-new = Lisää uutena
import-document-action-ignore = Ohita
import-document-compare = Vertaa
import-document-compare-legend = Sinun versiosi verrattuna tähän tiedostoon.
import-document-compare-close = Sulje
import-document-drop-title = Pudota dokumentit tähän
import-document-drop-hint = Markdown (.md), Word (.docx), OpenDocument (.odt) ja selkeä teksti (.txt)
import-document-browse = Selaa…
import-document-move-up = Ylös.
import-document-move-down = Siirry alas
import-document-remove-file = Poista
import-document-file-count = { $count ->
    [one] 1 tiedosto
   *[other] { $count } tiedostoa
}
import-document-no-files = Tiedostoja ei ole vielä valittu.
import-document-col-included = Tuo
import-document-col-title = Otsikko
import-document-col-type = Tyyppi
import-document-col-words = Sanat
import-document-col-breaks = Katkot
import-document-col-comments = Kommentit
import-document-col-footnotes = Alaviitteet
import-document-col-epigraph = Motto
import-document-col-source = Lähde
import-document-level-rules = Otsakkeet
import-document-level-n = Otsake { $level }
import-document-add-top-level = Lisää huipputaso
import-document-add-top-level-tooltip = Lisää kirjan jokaisen analysoidun kohteen yläpuolelle lukutiedostoille, joilla ei ole kirjan otsikkoa
import-document-destination = Määräpaikka
import-document-destination-hint = Valitse kansio tai kohde: uudet kohteet laskeutuvat kansioon tai kohtauksen jälkeen.
import-document-destination-empty = Ei kansioita vielä.
import-document-plan-empty = Ei ole vielä mitään tuotavaa.
import-document-summary = { $rows ->
    [one] 1 kohta
   *[other] { $rows } erää
} · { $breaks ->
    [one] 1 kohtauskatkos
   *[other] { $breaks } kohtauskatkoksia
}
import-document-back = Takaisin
import-document-cancel = Peruuta
import-document-analyse = Seuraava
import-document-analysing = Lukee asiakirjoja...
import-document-step-analysing = Luen
import-document-cancel-analysis = Lopeta lukeminen
import-document-analyse-failed = Asiakirjoja ei voitu lukea.
import-document-details = Yksityiskohdat
import-document-import = Tuo
import-document-done = { $count ->
    [one] 1 erä tuotu
   *[other] { $count } tuotuja tuotteita
}
import-document-undo = Kumoa
import-diagnostic-file-unreadable = “{ $path } Tekstiä ei voitu lukea: { $detail } Muut tiedostot tuodaan yhä.
import-diagnostic-nested-too-deep = “{ $path } Ei luettu: sen sisältö on pesiytynyt enemmän kuin { $limit }  tasot syvä. Ei todellista asiakirjaa lähellä, ja lukeminen se voi kaatua Skribisto Toiset tiedostot tuodaan yhä.
import-diagnostic-archive-too-large = “{ $path } Ei luettu: se on pakattu tiedosto, joka on rakennettu purkamaan paljon suurempi kuin se on, joka voisi tyhjentää muistia ja kaatua Skribisto Toiset tiedostot tuodaan yhä.
import-diagnostic-lossy-decode = { $count ->
    [one] Yksi merkki ...{ $path }... ei dekoodaa. Tallenna tiedosto uudelleen UTF-8 pitää sen.
   *[other] { $count }  kirjaimet ...{ $path }Tallenna tiedosto uudelleen kuin UTF-8 pitää ne.
}
import-diagnostic-decoded-from-bom = “{ $path }... { $detail } Ei UTF-8:aa.
import-diagnostic-empty-file = “{ $path }- Se on tyhjä.
import-diagnostic-no-headings = “{ $path } Siinä ei ole otsakkeita, joten se saapuu yhtenä kohtana.
import-diagnostic-unsupported-format = Mikään ei lue sitä.{ $detail } Tiedostot, joten...{ $path }- Se jäi väliin.
import-diagnostic-front-matter-not-flat = Etukäteen{ $path }”: “{ $detail }... ei ole yksinkertainen arvo, ja se jätettiin väliin.
import-diagnostic-footnotes-degraded = { $count ->
    [one] Asetuksen (EY) N:o { $path }.n alaviitteitä ei lueta kohdasta Markdown.
   *[other] { $count } alaviitteitä ...87{ $path }... tulee selkokielellä: alaviitteitä ei lueta kohdasta Markdown.
}
import-diagnostic-footnote-not-carried = { $count ->
    [one] Yksi alaviite{ $path } Sitä ei voitu tuoda tänne.
   *[other] { $count } alaviitettä 87{ $path }.
}
import-diagnostic-raw-html-dropped = { $count ->
    [one] Yksi raa'an HTML:n lohko putosi 87{ $path }.
   *[other] { $count }  raaka-aineet HTML  vuonna{ $path }-Kyllä.
}
import-diagnostic-nested-break-dropped = { $count ->
    [one] Yksi kohtaus break sisällä lainaus tai listan{ $path } Vain tauko omalla linjallaan pidetään.
   *[other] { $count }  kohtaus taukoja sisällä lainausmerkkejä tai luettelot ...{ $path } Vain tauko omalla linjallaan pidetään.
}
import-diagnostic-image-not-ingested = “{ $path }... tarkoittaa kuvaa{ $detail }Viitaus saapuu tekstinä; itse kuvaa ei kopioida.
import-diagnostic-duplicate-title = “{ $title } Ilmeisesti { $count }  kertaa. Jos olet tuonut nämä tiedostot aiemmin, tämä kopioi ne.
import-diagnostic-heading-level-jump = “{ $title }Hyppy otsakkeen tasolta { $from } - { $to }; se on sijoitettu yksi taso sen emon alle.
import-diagnostic-illegal-combination = “{ $title }Proosa, mutta... { $kind }  ei voi pitää mitään. Tuontia pidetään, kunnes muutat sen tyyppiä tai jätät sen pois. Mitään muuta ei tuoda.
import-diagnostic-tracked-changes-flattened = { $path } oli puolivälin revisio: { $count ->
    [one] { $count } Jäljitetty muutos hyväksyttiin
   *[other] { $count } tehdyt muutokset hyväksyttiin
}, ja poistot putosivat. Se on lopullinen teksti. Säilyttää oman sanamuodon sijaan, käytä Vertaa viimeisessä vaiheessa.
import-diagnostic-tracked-changes-flattened-by = { $path } oli puolivälin revisio: { $count ->
    [one] { $count } Jäljitetty muutos { $names } hyväksyttiin
   *[other] { $count } seuraavat muutokset hyväksyttiin { $names }
}, ja poistot putosivat. Se on lopullinen teksti. Säilyttää oman sanamuodon sijaan, käytä Vertaa viimeisessä vaiheessa.
import-diagnostic-text-box-dropped = { $count ->
    [one] { $path }:ssä on { $count } tekstikenttä. Teksti ei ole asiakirjassa, joten siihen ei voida vastata, jos se kuuluu käsikirjoitukseen.
   *[other] { $path }:ssä on { $count } tekstiruutua, joiden teksti on asiakirjan julkaisun ulkopuolella, joten siihen ei voida vastata, jos se kuuluu käsikirjoitukseen.
}
import-diagnostic-embedded-object-dropped = { $count ->
    [one] { $path } omistaa { $count } sulautettu objekti: kaavio, yhtälö tai vastaava. Ei ole mitään käsikirjoituksessa, joka voisi pitää sitä.
   *[other] { $path } omistaa { $count } sulautettuja esineitä: kaavio, yhtälö tai vastaava. Ei ole mitään käsikirjoituksessa, joka voisi pitää niitä.
}
import-diagnostic-field-flattened = { $count ->
    [one] { $path } on { $count } kenttä: sivunumero, ristiviittaus, päivämäärä. Se pitää tekstin se oli viimeksi esillä ja ei päivitä uudelleen.
   *[other] { $path } omistaa { $count } kentät: sivunumeron, ristiviittauksen, päivämäärän. Jokainen pitää tekstin, jonka se viimeksi näytti ja ei päivitä uudelleen.
}
import-diagnostic-unknown-style-level = { $path } käyttää tyyliä ...87{ $detail }..., joka näyttää otsikolta, mutta ei nimeä mitään tasoa. Nämä kohdat tuodaan proosana sen sijaan, että arvattaisiin syvyydessä.
import-diagnostic-comment-unanchored = Kommenttia { $detail }... vuonna { $path } ei voitu liittää sanoihin, joista siinä oli kyse. Se pidetään sen kohteen, jossa voit siirtää sitä.
import-diagnostic-comment-not-carried = Kommenttia .87{ $detail }... vuonna { $path } ei tuoda, koska mitään tiedostossa ei saapua tekstinä se voitaisiin säilyttää. Kopioi se tiedostosta, jos tarvitset sitä.
import-diagnostic-comment-replies-flattened = { $count ->
    [one] { $count } vastaus vuonna { $path } nimeltään kommentti, joka ei ole asiakirja, joten se saapuu oman kommenttinsa.
   *[other] { $count } vastauksia vuonna { $path } nimeltään kommentti, joka ei ole tiedostossa, joten ne saapuvat kommentit omia.
}
import-diagnostic-prose-not-verbatim = { $count ->
    [one] { $count } kappale { $path } ei voitu tuoda sen muotoilu täsmälleen niin kuin kirjoitettu, joten se saapuu kuin tavallinen teksti. Vertaa sitä alkuperäiseen.
   *[other] { $count } kappaleita { $path } ei voitu tuoda niiden muotoilu juuri niin kuin kirjoitettu, joten ne saapuvat tavallisena tekstinä. Vertaa niitä alkuperäiseen.
}
import-diagnostic-styled-spaces-not-carried = { $count ->
    [one] Vuonna { $path }, { $count } venytys tyhjä tila oli korostettu tai iski läpi, rivi vasemmalle täyttää esimerkiksi. Skribisto ei pidä kumpikaan riviä välilyöntejä yksin, joten se saapuu kuin tavallinen välilyöntejä, tai ei ollenkaan, jos se alkaa tai päättyy kappale.
   *[other] Vuonna { $path }, { $count } venytyksiä tyhjä tila on korostettu tai iski läpi, linjat vasemmalle täytettäväksi esimerkiksi. Skribisto ei pidä kumpikaan riviä välilyöntejä yksin, joten ne saapuvat kuin tavallinen tilat, tai ei ollenkaan, jos ne alkavat tai päättyy kappale.
}
import-diagnostic-list-nesting-flattened = { $count ->
    [one] { $count } lista kohta { $path } oli pesinyt yli { $limit } tasoa syvä. Se saapuu tasolle { $limit }, syvin Skribisto pitää, sen sanoja ja muotoilu.
   *[other] { $count } listan kohteita { $path } pesi yli { $limit } tasoa syvä. Ne saapuvat tasolle { $limit }, syvin Skribisto pitää, sanoineen ja muotoiluineen.
}
import-diagnostic-table-read-as-paragraphs = { $count ->
    [one] { $count } taulukko { $path } on rivi paljon leveämpi kuin loput, liian vinosti luoda ruudukko. Sen solut saapuvat kappaleina, yksi toisensa jälkeen, niiden sanoja ja muotoilu.
   *[other] { $count } pöydät { $path } on rivi paljon leveämpi kuin muut, liian vinosti luoda ruudukko. Heidän solunsa saapuvat kappaleina, yksi toisensa jälkeen, niiden sanoja ja muotoilu.
}
import-diagnostic-spaces-shortened = { $count ->
    [one] { $count } välilyönnissä { $path } oli pidempi kuin { $limit } tilaa, ja saapuu lyhennetty { $limit }. Sanat sen ympärillä ovat kaikki siellä.
   *[other] { $count } juoksut vuonna { $path } olivat pidempiä kuin { $limit } tilaa, ja saapuvat lyhennetty { $limit }. Sanat niiden ympärillä ovat kaikki siellä.
}
import-diagnostic-epigraph-not-carried = “{ $title }Epigraph, mutta... { $kind }  ei voi pitää yhtä. Lainaus säilytetään sen sijaan tekstin yläreunassa.
import-diagnostic-epigraph-placement-ambiguous = Epigraph istuu välillä{ $title }.. ja...{ $below }Se annettiin ...{ $title } Siinä kohtaa epigraph yleensä menee.
import-document-diagnostics = { $errors ->
    [0] { $warnings ->
            [one] 1 asia, jonka tietää
           *[other] { $warnings } asioita, jotka on syytä tietää
        }
   *[other] { $errors ->
            [one] 1 tiedostoa ei voitu lukea
           *[other] { $errors } tiedostoa ei voitu lukea
        }
}
import-document-diagnostics-none = Ei mitään raportoitavaa.

versions-title = Versiot
versions-scope-synopsis = Tiivistelmä
versions-scope-prose = Teksti
versions-loading = Tutkin varmuuskopioitasi.
versions-empty = Ei aikaisempaa versiota tästä vielä
versions-error = Mitään ei ole kadonnut, mutta tämä lista voi olla puutteellinen.
versions-did-not-exist = Ei ollut vielä olemassa { $date }
versions-deleted-after = Poistettu jonkin aikaa { $date } jälkeen
versions-unreadable = { $count ->
    [one] { $count } varmuuskopiota ei voitu lukea
   *[other] { $count } varmuuskopioita ei voitu lukea
}
versions-thinned = { $count ->
    [one] Vanhemmat versiot ohenevat iän myötä. Projektin oma historia on jo laskenut { $count } aiemmin.
   *[other] Vanhemmat versiot ohenevat iän myötä. Projektin oma historia on jo pudonnut { $count } aiemmin tämän tekstin tilasta.
}
versions-source-backup = Varmuuskopioinnista
versions-source-project = Hankkeen omasta historiasta
versions-list-caption = Yksi merkintä muutosta kohti, ei varmuuskopiota kohti
versions-pick-a-version = Valitse versio nähdäksesi, mikä on muuttunut
versions-earliest = Ei ole mitään vanhempaa, johon verrata sitä.
versions-no-change = Kohteen tässä osassa mikään ei muuttunut
versions-formatting-only = Vain muotoilu muuttui täällä. Sanat ovat samat
versions-show-unchanged = Näytä muuttumaton
versions-hide-unchanged = Piilota muuttumaton
versions-next-change = Seuraava muutos
versions-near = lähellä "{ $text }"
versions-words-added = { $count ->
    [one] { $count } lisätty sana
   *[other] { $count } lisätty ilmaisu
}
versions-words-removed = { $count ->
    [one] { $count } Sana poistettu
   *[other] { $count } sanat poistettu
}
versions-blocks-moved = { $count ->
    [one] { $count } kappale siirretty
   *[other] { $count } kappaleet siirretty
}
versions-pin = Pin varmuuskopio tämä versio tuli. Automaattinen puhdistus ei koskaan poista sitä
versions-unpin = Avaa varmuuskopio tästä versiosta. Automaattinen puhdistus voi poistaa sen uudelleen
versions-pinned-only = Näytä vain pinnoitettuja versioita
versions-pin-note = Vain varmuuskopion versiot voidaan kiinnittää. Nuppi pitää tiedostoa, eikä projektin oma historia ole sellainen.
versions-pinned-empty = Mikään täällä ei ole vielä jumissa.
versions-pinned-empty-log = Vain varmuuskopion versiot voidaan ladata, - pinillä on tiedosto, eikä projektin oma historia ole sellainen.
versions-range-filter = Näytä vain versioita kahden päivän välillä
versions-filtered-empty = Asettamiasi suodattimia ei vastaa mikään versio
versions-clear-filters = Tyhjennä suodattimet
versions-last-30-days = Viimeiset 30 päivää
versions-restore-button = Palauta tämä versio
versions-restore-confirm-title = Korvaa tämä teksti versiolla { $date }?
versions-restore-confirm-text = Se, mitä teillä nyt on, korvataan tämän kohdan tekstillä { $date }.
versions-restore-confirm-with-comments = Se, mitä teillä nyt on, korvataan tämän kohdan tekstillä { $date }. { $count ->
    [one] { $count } kommentti on ankkuroitu nykyiseen tekstiin, ja se voidaan jättää orvoksi.
   *[other] { $count } kommentit on ankkuroitu nykyiseen tekstiin, ja ne voidaan jättää orvoksi.
}
versions-restore-confirm-undo-note = Varmuuskopiointi tehdään ensin, ja Ctrl+Z purkaa tämän yhdessä vaiheessa.
versions-restored-toast = Palautti version { $date }
versions-undo = Kumoa
versions-restore-row-gone = Tämä kohta ei ole enää mukana tässä projektissa
versions-restore-no-home = Tämä kohta on muuttunut sen jälkeen, ja vanha teksti ei ole mennä siihen
versions-restore-no-safety-copy = Turvavarmistuksesi ei toiminut, joten mikään ei muuttunut.
versions-restore-failed = Palautus epäonnistui: { $error }
versions-restore-backup-busy = Apujoukko on jo käynnissä. Yritä uudelleen hetken kuluttua.
versions-restore-in-backup-file = Katselet varmuuskopiotiedostoa, avaa projekti itse palauttaaksesi sen
versions-restore-no-project = Projektia ei ole avattu
versions-recreate-button = Tuo tämä takaisin...
versions-recreate-picker-title = Minne sen pitäisi mennä?
versions-recreate-picker-empty = Projektissa ei ole kansiota.
versions-recreate-picker-confirm = Tuo se tänne.
versions-recreate-picker-cancel = Peruuta
versions-recreate-untitled = kohta
versions-recreate-confirm-title = Tuo{ $item }- Takaisin?
versions-recreate-confirm-text = Se lisätään kohtaan { $destination }, ja teksti on numerossa { $date }.
versions-recreate-confirm-undo-note = Perukaa seuraava viesti ja ottakaa se takaisin.
versions-recreated-toast = “{ $item }.. on palannut projektiisi
versions-recreated-partial-toast = { $count ->
    [one] “{ $item }On palannut, mutta yhtä sen tekstejä ei voitu lukea
   *[other] “{ $item }Se on palannut, mutta... { $count }  Tekstiä ei voitu lukea
}
versions-recreate-already-here = Tämä kohde on jo projektissasi
versions-recreate-no-destination = Valitse jostain kansiosta.
versions-recreate-unreadable = Että varmuuskopioita ei voitu lukea, joten mitään ei lisätty
versions-recreate-failed = En saanut sitä takaisin: { $error }
versions-changed-percent = { $percent }% tästä muuttui
versions-hidden-paragraphs = { $count ->
    [one] ... { $count } kappale, jota ei muuteta...
   *[other] ... { $count } kappaleita, jotka eivät muutu...
}

timeline-title = Palaa ajassa taaksepäin
timeline-coverage = { $count ->
    [one] { $count } versio tallennettu, jatkuu { $oldest }
   *[other] { $count } versiot tallennettu, jatkuu { $oldest }
}
timeline-loading = Projekti on mennyttä.
timeline-empty = Tästä projektista ei ole vielä tallennettu versiota
timeline-no-changes = Mikään ei ole muuttunut sen jälkeen.
timeline-no-text-changes = Tekstiä ei ole muutettu sen jälkeen
timeline-slider-label = Tallennettu versio
timeline-series-name = Hankkeen koko
timeline-bars-caption = Jokainen palkki on nauhoitettu versio, yhtä pitkä kuin projekti oli silloin. Korostettu versio on se, mitä katsot.
timeline-bars-caption-periods = Liian monta versiota näyttää yksitellen, joten jokainen palkki on { $period }, niin pitkä kuin projekti oli lopussa.
timeline-unit-hour = tunti
timeline-unit-day = päivä
timeline-unit-week = viikko
timeline-unit-month = kuukausi
timeline-range-filter = Näytä vain versioita kahden päivän välillä
timeline-range-empty = Kyseisiin päivämääriin ei kirjattu versiota
timeline-open-period = Avaa tämä jakso
timeline-show-all = Näytä koko historia
timeline-changed-since = { $count ->
    [one] { $count } kohta eroaa välillä { $date } ja projektisi nyt
   *[other] { $count } kohteita eroaa välillä { $date } ja projektisi nyt
}
timeline-kind-added = Kirjoitettu vuodesta
timeline-kind-removed = Ei enää projektissa
timeline-kind-changed = Muokkattu sen jälkeen
timeline-kind-moved = Muutettu
timeline-not-yet-written = Tätä ei ollut vielä olemassa.
timeline-no-text-of-its-own = Tässä ei ole omaa tekstiä.
timeline-reader-close = Sulje
timeline-reader-stamp = Kuten se oli { $date }
timeline-reader-loading = Tallennetun version avaaminen...
timeline-reader-unreadable = Tätä nauhoitusta ei voitu lukea.
timeline-reader-compared = Kuten se oli { $date }, asettaa vastaan mitä se sanoo nyt
timeline-reader-diff-legend = Sen jälkeen on käyty läpi, ja siihen on lisätty alleviivattu.
timeline-reader-view-label = Mitä näytetään
timeline-reader-view-diff = Muutokset
timeline-reader-view-text = Teksti
timeline-reader-deleted = Voit lukea sen ja kopioida sen täällä.
timeline-prose-only-record = Tämä piste tulee projektin omasta historiasta, joka pitää tekstiä eikä mitään muuta, joten mitä poistettiin tai siirrettiin, koska sitä ei voida näyttää. Valitse varmuuskopio siitä.

settings-sec-extensions = Laajennukset

settings-group-games-forward = Aina eteenpäin
settings-games-forward-toggle = Soita "Aina eteenpäin"
settings-games-forward-hint =
    Kun pelaat, mitään mitä olet kirjoittanut ei voi ottaa takaisin: Backspace, Poista,
    Leikkaa, vedä ja pudota ja Peru ovat kaikki pois käytöstä pinnoilla valitset alla.
    Voit silti kirjoittaa, liittää, muotoilla ja liikkua vapaasti. Luonnos vain kasvaa.
settings-games-session-warning =
    Tämä on per-session valinta: sitä ei koskaan tallenneta. Lopettaa projektin, tai lopettaa
    Skribisto, aina päättyy pelin, ja kaikki mitä kirjoitit pelatessasi tulee
    Se ei ole enää mahdollista.
settings-group-games-scope = Mikäli sovelletaan
settings-games-in-prose = Käsikirjoitusproosa
settings-games-in-synopsis = Synopsit
settings-games-scope-hint =
    Kommentit, alaviitteet ja otsikot eivät koskaan jäädy: ne ovat missä huomaat korjata
    Olet juuri kieltänyt itseäsi tekemästä sitä.
settings-games-inert-warning =
    "Aina eteenpäin" on päällä, mutta se ei koske mitään.
    Tai se ei muuta mitään kirjoittaessasi.

games-title = Kirjoitetaan pelejä
games-forward-name = Aina eteenpäin
games-forward-blurb =
    Vedä ottamatta mitään takaisin. Poistaminen, leikkaaminen ja poistaminen ovat pois käytöstä
    Kirjoita seuraava lause sen sijaan, että korjaisit viimeisen.
games-forward-playing = Soitto: poistaminen ei ole käytössä
games-forward-idle = Ei soita.
games-session-note = Se loppuu, kun suljet projektin.
games-scope-prose-and-synopsis = Koskee proosaasi ja synopsiasi.
games-scope-prose = Se vastaa proosaasi.
games-scope-synopsis = Se vastaa synopsejasi.
games-scope-nothing = Ei vielä mitään. Valitse pinta Asetuksista.
games-settings-link = Kirjoitetaan peliasetuksia...
statusbar-games-forward = Aina eteenpäin
statusbar-games-forward-tooltip =
    "Aina eteenpäin" on päällä: poistetaan, leikataan ja perutaan käytöstä, kun taas sinä
    Vedä. Klikkaa lopettaaksesi pelaamisen.
goal-progress-words = { $g ->
    [one] { $count }, { $goal }, sana
   *[other] { $count } of { $goal } words
}
goal-progress-characters = { $g ->
    [one] { $count }, luonteeltaan { $goal }
   *[other] { $count }, { $goal } merkkiä
}
goal-count-words = { $n ->
    [one] { $count } sana
   *[other] { $count } words
}
goal-count-characters = { $n ->
    [one] { $count } merkki
   *[other] { $count } merkkiä
}
goal-unit-words = Sanat
goal-unit-characters = Merkit

inspector-goal = Tavoite
inspector-goal-none = Ei tavoitetta

overview-col-goal = Tavoite
overview-excluded-from-export = Jätetty pois viennistä, joten se ei lasketa yhteensä

goal-subtree-total-words = { $count ->
    [one] { $items } kohde sisällä lisää jopa { $words } sanat
   *[other] { $items } kohteet sisällä lisää jopa { $words } words
}
goal-subtree-total-characters = { $count ->
    [one] { $items } kohde sisällä on jopa { $words } merkkiä
   *[other] { $items } kohteita sisällä lisää jopa { $words } merkkiä
}

new-work-goal-unit = Laske
new-work-goal-unit-hint = Käytetään jokaiseen sanaan tai merkkikohteeseen tässä projektissa. Muutettavissa myöhemmin.

settings-group-goal-unit = Tavoitteet
settings-goal-unit-switch-title = Kytkinten laskentayksikkö
settings-goal-unit-switch-text = Projektille jo asetetut tavoitteet ja virstanpylvät kirjattiin vuonna { $from }. Siirtyminen { $to }:een ei muunna niitä, joten jokaista nykyistä numeroa pidetään nyt { $to }:na.
settings-goal-unit-switch-informative = Mitään ei ole menetetty. Vaihda takaisin milloin tahansa palauttaa alkuperäisen lukeman, sitten päivittää kohteet haluat säilyttää.
settings-goal-unit-switch-confirm = Vaihda

goal-distribute-action = Jaa...
distribute-title = Jaa kohde
distribute-empty = Tämän sisällä ei ole mitään jaettavaa.
distribute-weight = Jaa
distribute-weight-length = Pituussuunnassa
distribute-weight-rows = paloina
distribute-weight-even = Tasaisesti
distribute-overwrite = Korvaa jo asetetut tavoitteet
distribute-col-item = Kohta
distribute-col-current = Nyt.
distribute-col-proposed = Sen jälkeen
distribute-total = Näitä on jopa { $total }, kun tavoitteena on { $goal }.
distribute-over-budget = Sisällä olevat kohteet ovat { $over } enemmän kuin tämän säiliön omat. Kohota sen kohde, laskea niiden, tai korvata ne.
distribute-apply = Jakaudu
distribute-cancel = Peruuta

milestone-target-gone = Kohde poistettu
milestone-no-target = Ei tavoitetta
milestone-add = Lisää
milestone-add-label = Mitä pitäisi saavuttaa?

pace-summary-title = Missä kirja seisoo
pace-summary-remaining = { $words } jäljellä
pace-summary-open = Avaa suunnitelma.
pace-summary-close = Sulje
pace-summary-dont-show = Älä näytä tätä avattaessa
pace-summary-menu = Kirjoitan suunnitelmaa...

menu-help-topics = &Ohjeet
menu-help-shortcuts = N&äppäimistön pikanäppäimet...
menu-help-website = &Skribisto verkossa
menu-help-report = &Raportoi ongelmasta...
menu-help-check-updates = &Tarkista päivitykset
menu-command-palette = &Komentopaletti...

learn-help-topics = Ohjeaiheet
learn-shortcuts = Pikanäppäimet
learn-website = Skribisto verkossa

help-window-title = Ohje
help-filter-topics = Suodata aiheita
help-no-matching-topic = Ei aihe täsmää.
help-topic-missing = Tämä aihe ei ole enää käytettävissä.
help-back = Takaisin
help-not-translated = Tätä sivua ei ole vielä käännetty, joten se näytetään englanniksi.

help-section-getting-started = Aloittaminen
help-section-writing = Kirjoitetaan
help-section-reviewing = Uudelleentarkastelu
help-section-exchanging = Sanat sisään ja ulos
help-section-keeping = Pidä työsi turvassa
help-section-extensions = Laajennukset

help-topic-getting-started = Ensimmäinen projekti
help-topic-writing-model = Miten kirja kootaan
help-topic-drafts-and-old-versions = Vanhemman luonnoksen säilyttäminen
help-topic-goals-and-pace = Tavoitteet ja tahti
help-topic-comments = Kommentit
help-topic-round-trip = Kirjan lähettäminen lukijalle
help-topic-export = Vienti
help-topic-import-documents = Tuodaan asiakirjoja
help-topic-import-projects = Koko projektin tuominen
help-topic-backups-and-versions = Varmuuskopioita ja versioita

help-shortcuts-title = Pikanäppäimet
help-shortcuts-filter = Suodata pikanäppäimet
help-shortcuts-no-matches = Ei oikotietä.
help-shortcuts-rebind = Vaihda oikoteitä...
help-shortcuts-close = Sulje

command-palette-placeholder = Kirjoita komento

shortcut-name-binder-duplicate = Monista
shortcut-name-comments-add = Lisää kommentti
shortcut-name-comments-add-paragraph = Kohtaa koskeva huomautus
shortcut-name-spellcheck-toggle = Tarkista oikeinkirjoitus
shortcut-name-editor-tab-close = Sulje välilehti
shortcut-name-editor-tab-pin = Kiinnitä tai irrota välilehti
shortcut-name-editor-save = Tallenna
shortcut-name-work-export = Vie...
shortcut-name-work-new = Uusi projekti
shortcut-name-work-open = Avaa projekti
shortcut-name-window-new = Uusi ikkuna
shortcut-name-work-close = Sulje projekti
shortcut-name-app-settings = Asetukset
shortcut-name-app-quit = Lopeta
shortcut-name-editor-insert-footnote = Lisää alaviite
shortcut-name-format-scene-break = Lisää kohtauskatkos
shortcut-name-format-major-scene-break = Lisää suuri kohtauskatkos
shortcut-name-format-link = Lisää linkki
shortcut-name-format-mark-nbsp = Lisää taukoamaton tila
shortcut-name-format-mark-nbhyphen = Lisää taukoamaton väliviiva
shortcut-name-format-mark-nnbsp = Lisää kapea ei-break- tila
shortcut-name-format-mark-zwsp = Lisää nollaleveysalue
shortcut-name-go-next = Seuraava
shortcut-name-go-prev = Edellinen
shortcut-name-go-to = Siirry
shortcut-name-outline-toggle = Näytä tai piilota rakenne
shortcut-name-preview-toggle = Näytä tai piilota esikatselu
shortcut-name-view-fullscreen = Vaihda koko näytön tilaa
shortcut-name-view-focus-mode = Valitse häiriöttömyystila
shortcut-name-editor-size-increase = Suurenna tekstin kokoa
shortcut-name-editor-size-decrease = Pienennä tekstin kokoa
shortcut-name-editor-size-reset = Nollaa tekstin koko
shortcut-name-editor-find = Etsi
shortcut-name-editor-replace = Korvaa
shortcut-name-editor-find-next = Seuraava ottelu
shortcut-name-editor-find-prev = Edellinen ottelu
shortcut-name-search-show = Hae projektista
shortcut-name-search-replace = Korvaa projektissa
shortcut-name-outline-open-to-side = Avaa sivulle
shortcut-name-help-topics = Ohjeet
shortcut-name-help-shortcuts = Näppäimistön pikanäppäimet
shortcut-name-help-website = Skribisto verkossa
shortcut-name-help-report = Ilmoita ongelmasta
shortcut-name-help-check-updates = Tarkista päivitykset
update-check-behind = Versio { $version } on saatavilla.
update-check-current = Ajat uusinta versiota.
update-check-failed = Uudempaa versiota ei voitu tarkistaa. { $error }
shortcut-name-command-palette = Komentopaletti
help-section-reference = Mitä asiat ovat?

help-concept-scene-break = Kohtaustauko
help-concept-major-scene-break = Suuri kohtauskatkos
help-concept-find-in-prose = Etsi tekstistä
help-concept-story-bible = Tarinan tietopankki
help-concept-goal-unit = Sanat tai merkit
help-concept-goal-progress = Edistyminen
help-concept-manuscript-words = Mikä lasketaan käsikirjoitukseksi
help-concept-exportable = Jätetään pois viennistä
help-concept-distribute = Kohteen jakaminen
help-concept-subtree-total = Mitä sisällä olevat kohteet merkitsevät?
help-concept-milestone = Välitavoite
help-concept-pace-plan = Kirjoitussuunnitelma

help-concept-tag = Tunniste
help-concept-label = Lisäteksti
help-concept-point-of-view = Näkökulma
help-concept-epigraph = Motto
help-concept-footnote = Alaviite
help-concept-chapter-mode = Kappaleen muoto
help-concept-comment = Kommentti
help-concept-backup = Varmistus
help-concept-version = Versio
help-concept-trash = Roskakori
help-concept-spellcheck = Oikolukutarkistus
help-concept-search-replace = Etsi ja korvaa
help-concept-note-template = Huomautusmalli
help-concept-story-bible-entry = Tarinan raamattumerkintä
help-concept-text-replacement = Tekstin korvaaminen
help-concept-smart-punctuation = Älykäs välimerkki
help-concept-export-style = Vie
help-concept-round-trip-marks = Pyöreän matkan merkkiaineet

epigraph-mark-attribution = Lähdelinja
epigraph-mark-attribution-tip = Merkitse rivi, johon kaappi on lainauksen lähteenä, joten se tulostaa jakona
margin-lane-name = Marginaalit
margin-lane-provider-comments = Kommentit
margin-lane-provider-story-bible = Tarinan raamattumerkintä
margin-lane-provider-story-bible-hint = On note on Prose käsittelyssä, jossa se merkintä on nimetty, ja kohtaukset, jotka kerrotaan sen silmin. Myös kirjoitetaan ne nimet proosa itse.
margin-lane-mark-point-of-view = Kerroin täältä.
margin-lane-mark-named-here = Nimi on tässä.
margin-lane-provider-comments-hint = Jos tekstiin on liitetty huomautus
margin-lane-provider-search = Etsi osumia
margin-lane-provider-search-hint = Jokainen osuma siihen, mitä viimeksi etsit, missä se on asiakirjassa
margin-lane-provider-boundaries = Mistä jokainen asiakirja alkaa?
margin-lane-provider-boundaries-hint = Säännön jokaisen kohtauksen päällä purossa, jotta voit nähdä missä olet
margin-lane-provider-spelling = Oikeinkirjoitus
margin-lane-provider-spelling-hint = Jossa loitsun tarkistaja on merkitty sana. Pois ellet pyydä sitä: se merkitsee koneen mielipide proosa
margin-lane-spelling = { $word }, mahdollinen erehtyminen
margin-lane-search-hit = { $text }, match { $index } / { $total }
margin-lane-search-current = { $text }, ottelu { $index } of { $total }, jonka päällä olet
margin-lane-boundary = Aloitetaan { $title }
margin-lane-boundary-untitled = nimittämätön asiakirja
settings-page-margin-lane = Marginaalit
settings-desc-margin-lane = Strippi kääröpalkin vieressä, ja mitä se näyttää
settings-margin-lane-enabled = Näytä marginaalikaista
settings-margin-lane-enabled-hint = Sama kytkin kuin View ... Marginaalimerkit.
settings-margin-lane-enabled-more = Se ei koskaan sano mitään väärää: se näyttää, missä asiat ovat, ja jättää mitä tehdä siitä sinulle.
settings-group-margin-lane-marks = Mitä se merkitsee?
settings-margin-lane-no-providers = Mikään ei vielä merkitse kaistaa.
settings-group-margin-lane-texture = Vuoropuhelun rakenne
settings-margin-lane-texture = Näytä vuoropuhelun rakenne
settings-margin-lane-texture-hint = Yksi palkki kappaleessa: kuinka kauan se on, ja kuinka paljon siitä puhutaan.
settings-margin-lane-texture-more = Ei mitattu kielille, joilla ei ole kuratoituja käytäntöjä, joissa palkki näyttää pituuden yksin.
settings-group-margin-lane-surfaces = Mistä se ilmestyy
settings-margin-lane-surface-editor = Tekstieditori
settings-margin-lane-surface-stream = Virrat
settings-margin-lane-surface-search-preview = Etsi esikatselu
menu-margin-lane = &Marginaalit

status-preset-drafting = Laadinta
status-preset-passes = Revision läpäisy
status-preset-plume = Plume Creator
status-todo = Tehdä
status-draft = Luonnos
status-revised = Tarkistettu
status-final = Lopullinen
status-outline = Rakenne
status-first-edit = 1. muokkaus
status-second-edit = 2. muokkaus
status-done = Valmis
status-plume-draft-1 = Ensimmäinen luonnos
status-plume-draft-2 = 2. luonnos
status-plume-draft-3 = Kolmas luonnos
status-plume-edit-1 = 1. muokkaus
status-plume-edit-2 = 2. muokkaus
status-plume-edit-3 = Kolmas muokkaus
status-plume-proofread = Oikoluku
status-plume-finished = Valmis
new-work-statuses = Työnkulku
new-work-statuses-hint = Vaiheet voit siirtää kohtauksen läpi. Voit nimetä uudelleen tai lisätä nämä milloin tahansa.
status-none = Ei asemaa
inspector-status = Tila
overview-col-status = Tila
overview-status-mixed = jäljempänä olevat kohdat ovat eri mieltä
help-concept-status = Tila
status-completion-title = Missä kirja seisoo
status-completion-headline = { $done } of { $total } kohtaus valmistui
status-completion-empty = Tämä lukee käsikirjoituksen.
status-completion-open = Missä kirja seisoo...

settings-page-statuses = Tilat
settings-desc-statuses = Nimeä, tilaa uudelleen, lisää ja poista vaiheet liikut kohtaus läpi
settings-statuses-add = Lisää tila
settings-statuses-add-placeholder = Uusi tilanimi
settings-statuses-desc =
    Vaiheet kohtaus liikkuu läpi, jotta, ainakin valmis alkuun useimmat
    "Vähemmän valmis kuin" tarkoittaa kaikkialla muualla.
    sovelluksessa, joten laita ne miten prosessisi todella toimii.
settings-statuses-apply-preset = Käytä esiaseteltua...
settings-statuses-preset-applied = { $added ->
    [one] 1 tila lisätty
   *[other] { $added } lisätty asemat
}
settings-statuses-preset-refused = Tässä projektissa on jo tikkaat. Esiasetukset täyttävät vain tyhjän. Poista puolat, joita et halua ensin.
settings-statuses-duplicate = "{ $name }" on jo tikkailla
settings-statuses-added = Lisätty "{ $name }"
settings-statuses-move-up = Siirrä ylös (vähemmän valmis)
settings-statuses-move-down = Siirry alas (enemmän valmis)
settings-statuses-details-placeholder = Mitä tämä vaihe tarkoittaa (valinnainen)
settings-statuses-delete = Poistetaan ilmaisu "{ $name }."
settings-statuses-delete-in-use = { $count ->
    [one] Poistetaan ilmaisu "{ $name }": 1 kohta on siinä ja menettää asemansa.
   *[other] Poistetaan ilmaus "{ $name }": siinä on { $count } kohtaa, jotka menettävät asemansa.
}
settings-statuses-deleted = Poistettu "{ $name }"
settings-statuses-deleted-in-use = { $count ->
    [one] Poistettu "{ $name }." 1 kohta ei ole enää asemassa.
   *[other] Poistettu "{ $name }." { $count } kohteesta ei ole enää asemaa.
}
settings-statuses-empty-title = Ei vielä työnkulkua
settings-statuses-empty-body = Tila kertoo, kuinka pitkälle kohtaus on. Lisää yksi ylle, tai aloita esiasetuksesta.
status-category-planned = Suunniteltu
status-category-drafting = Laadinta
status-category-needs-work = Tarvitsee työtä
status-category-revised = Tarkistettu
status-category-final = Lopullinen
