# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

menu-work = &Projekti
menu-new-work = &Uusi projekti
menu-open-work = &Avaa projekti…
menu-new-window = Uusi &ikkuna
menu-import-from = Tu&o lähteestä
menu-create-from = Lu&o lähteestä
menu-import-plume = &Plume Creator (.plume)…
menu-import-document = &Asiakirjat (Markdown, Word, ODT)…
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
menu-export-none = Avaa ensin vietävä asiakirja
menu-save = &Tallenna
menu-save-as-file = Tallenna &yhtenä tiedostona…
menu-save-as-folder = Tallenna &kansiona…
menu-backup = Varmuuskopioi &nyt
menu-close-work = &Sulje projekti
menu-welcome = Te&rvetuloa…
menu-settings = As&etukset
menu-quit = &Lopeta

menu-view = &Näytä
menu-outline = &Jäsennys
menu-search = &Hae projektista
menu-timeline = &Palaa ajassa taaksepäin
menu-search-preview = Haun &esikatselu
menu-fullscreen = &Koko näyttö
menu-focus-mode = Häiriötön &tila
menu-format = M&uotoilu
menu-scene-break = Lisää koh&tauskatko
menu-major-scene-break = Lisää suu&ri kohtauskatko

menu-go = &Siirry
menu-go-next-scene = Seuraava &kohtaus
menu-go-prev-scene = Edellinen k&ohtaus
menu-go-next-chapter = Seuraava &luku
menu-go-prev-chapter = Edellinen l&uku
menu-go-next-note = Seuraava &muistiinpano
menu-go-prev-note = Edellinen mu&istiinpano

menu-tools = &Työkalut
menu-help = &Ohje

ctx-add = &Lisää
ctx-new-item = Uusi &kohde
ctx-new-folder = Uusi k&ansio
ctx-rename = Nimeä &uudelleen
ctx-duplicate = &Monista
ctx-import-here = &Tuo tähän…
ctx-indent = &Sisennä
ctx-outdent = &Vähennä sisennystä
ctx-trash = Siirrä &roskakoriin
ctx-open-to-side = Avaa &sivulle
ctx-open = &Avaa
ctx-reveal-in-outline = Näytä &jäsennyksessä
ctx-move-up = Siirrä &ylös
ctx-move-down = Siirrä &alas

ctx-tab-close = &Sulje
ctx-tab-close-others = Sulje &muut
ctx-tab-close-all = Su&lje kaikki
ctx-tab-open-to-side = &Avaa sivulle
ctx-tab-open-in-main = &Avaa pääpaneelissa
ctx-tab-move-to-side = S&iirrä sivulle
ctx-tab-move-to-main = S&iirrä pääpaneeliin
ctx-tab-move-to-new-window = Siirrä &uuteen ikkunaan
ctx-tab-move-window-unsaved = Tallenna projekti ensin: toinen ikkuna avautuu levyllä olevaan tiedostoon
ctx-tab-pin = &Kiinnitä tämä välilehti
ctx-tab-unpin = I&rrota tämä välilehti
tab-pinned-tooltip = Kiinnitetty: ”Sulje muut” ja ”Sulje kaikki” jättävät sen auki

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
create-story-bible-entry = Tietopankin merkintä…
type-book-start = Kirjan alku
type-text = Teksti

new-item-book = Uusi kirja
new-item-part = Uusi osa
new-item-chapter = Uusi luku
new-item-note = Uusi muistiinpano
new-item-note-folder = Uusi muistiinpanokansio
new-item-folder = Uusi kansio
new-item-paratext = Uusi parateksti
new-item-paratext-folder = Uusi paratekstikansio
new-item-story-bible-entry = Uusi tietopankin merkintä

placement-inside = sisään
placement-after = jälkeen
placement-after-parent = ylätason jälkeen
placement-top-level = ylin taso


ctx-promote = Muunna t&yypiksi
promote-chapter-folder = Lukukansio
promote-flat-chapter = Kansioton luku
promote-lossy-title = Tekstiä ei voi säilyttää missään
promote-lossy-text = Tyypillä { $target } ei ole paikkaa näille: { $kinds }. Siirrä tai tyhjennä teksti ensin ja muunna sitten.
content-scene-text = Kohtausteksti
content-note-text = Muistiinpanon teksti
content-book-title = Kirjan nimi
content-book-subtitle = Kirjan alaotsikko
content-part-title = Osan otsikko
content-chapter-title = Luvun otsikko
content-epigraph-text = Motto
content-paratext-text = Parateksti
promote-blocked-title = Luku ei ole tyhjä
promote-blocked-text = { $count ->
       [one] Tässä luvussa on vielä { $count } kohde. Siirrä se muualle tai roskakoriin ennen kuin muunnat luvun kansiottomaksi.
      *[other] Tässä luvussa on vielä { $count } kohdetta. Siirrä ne muualle tai roskakoriin ennen kuin muunnat luvun kansiottomaksi.
   }

inspector = Ominaisuudet
inspector-empty = Avaa kohde tarkastellaksesi sitä.
inspector-promote = Muunna tyypiksi…
statusbar-toggle-outline = Näytä tai piilota mappi
statusbar-toggle-inspector = Näytä tai piilota ominaisuudet
statusbar-save-unsaved = Tallentamattomia muutoksia. Tallenna napsauttamalla
statusbar-save-saved = Kaikki muutokset on tallennettu
statusbar-save-autosave = Automaattinen tallennus on päällä. Muutokset tallentuvat kirjoittaessasi
statusbar-saving = Tallennetaan…
statusbar-word-count = { $count ->
       [one] { $count } sana
      *[other] { $count } sanaa
   }
statusbar-word-count-tooltip = Sanat muokattavana olevassa kohtauksessa
statusbar-word-char-count = { $words ->
       [one] { $words } sana
      *[other] { $words } sanaa
   } · { $chars ->
       [one] { $chars } merkki
      *[other] { $chars } merkkiä
   }
session-toggle = Kirjoitusistunto: aloita tai keskeytä keskittynyt kirjoitusrupeama
session-configure = Aseta istunnon sanatavoite ja aikaraja
session-configure-title = Kirjoitusistunto
session-word-goal = Sanatavoite
session-time-limit = Aikaraja
session-time-limit-unit = min
session-no-goal = Ei tavoitetta
session-no-limit = Ei aikarajaa
session-reset = Nollaa istunto
session-readout = { $words ->
    [one] { $words } sana
   *[other] { $words } sanaa
} · { $time }
session-readout-timed = { $words ->
    [one] { $words } sana
   *[other] { $words } sanaa
} · { $time } jäljellä
statusbar-focus-exit = Poistu häiriöttömästä tilasta
statusbar-focus-synopsis = Tiivistelmä
statusbar-focus-go-prev = Edellinen kohde (Alt+Ylänuoli)
statusbar-focus-go-next = Seuraava kohde (Alt+Alanuoli)

rail-settings = Asetukset

language = Kieli
theme = Teema
english = Englanti
french = Ranska
light = Vaalea
dark = Tumma
settings-text-width = Tekstin leveys
settings-preview-width = Haun esikatselun leveys
settings-autosave = Tallenna automaattisesti levylle
settings-show-welcome = Näytä aloitusikkuna käynnistettäessä
settings-show-welcome-tip = Kun asetus on pois päältä, viimeisin projekti avataan sen sijaan uudelleen.
settings-check-for-updates = Tarkista uudet versiot
settings-check-for-updates-tip = Skribisto kysyy enintään kerran päivässä projektin verkkosivustolta, mikä versio on uusin. Pyyntö ei sisällä mitään tunnistetta eikä edes käyttämäsi version numeroa: vertailu tehdään tällä tietokoneella.

settings-title = Asetukset
settings-close = Sulje
settings-search = Hae asetuksista
settings-reset = Palauta oletukset
settings-done = Valmis
settings-cancel = Peruuta
settings-apply = Käytä
settings-ok = OK
settings-reset-confirm-title = Palautetaanko kaikki asetukset oletusarvoihin?
settings-reset-confirm-body = Palauttaa tämän sovelluksen ulkoasun, muokkaimen ja kirjoittamisen asetukset. Projektin omat asetukset, pikanäppäimet ja tallennetut tyylit eivät muutu. Toimintoa ei voi kumota.
settings-empty-title = Täällä ei ole vielä asetuksia
settings-empty-hint = Tähän osioon tulee asetuksia tulevassa päivityksessä.

settings-sec-appearance-behaviour = Ulkoasu ja toiminta
settings-sec-editor = Muokkain
settings-sec-spelling = Oikeinkirjoitus
settings-sec-backup = Varmuuskopiointi ja synkronointi
settings-sec-compile = Koostaminen ja vienti
settings-page-appearance = Ulkoasu
settings-field-image-size-policy = Suuret kuvat
settings-image-policy-ask = Kysy joka kerta
settings-image-policy-keep = Säilytä alkuperäinen
settings-image-policy-downscale = Optimoi
settings-hint-image-size-policy = Mitä tehdään, kun lisätty kuva on suurempi kuin käsikirjoitus tarvitsee.
settings-page-notifications = Ilmoitukset
settings-page-scene = Kohtaus
settings-page-synopsis = Tiivistelmä
settings-page-notes = Muistiinpanot
settings-page-editor-behavior = Muokkaimen toiminta
settings-page-goals = Tavoitteet ja sanamäärä
settings-page-games = Kirjoituspelit
settings-page-corkboard = Korkkitaulu
settings-page-distraction-free = Häiriötön tila
settings-page-dictionaries = Sanakirjat
settings-page-autosave = Automaattinen tallennus
settings-page-export = Vientimuodot
settings-page-paratext = Paratekstirakenteet
settings-page-keymap = Näppäinkartta
settings-keymap-filter = Suodata pikanäppäimiä

settings-desc-sec-appearance-behaviour = Miltä sovellus näyttää ja mitä se tekee käynnistyessään.
settings-desc-sec-editor = Kirjoituspinta: miltä se näyttää ja mitä se tekee kirjoittaessasi.
settings-desc-sec-spelling = Oikoluku ja sen käyttämät sanakirjat.
settings-desc-sec-backup = Miten työsi tallentuu levylle ja mitä kopioita säilytetään.
settings-desc-sec-compile = Mitä Skribistosta viedään ja missä muodossa.
settings-desc-sec-work = Tämän projektin omat asetukset, jotka kulkevat sen tiedoston mukana.
settings-desc-sec-extensions = Tähän asennettujen laajennusten tuomat sivut.
settings-desc-group-typography = Oma sivu kullekin tekstilajille: kirjasin, koko, rivinkorkeus ja välit.
settings-desc-appearance = Käyttöliittymän kieli, teema, tekstikoko ja aloitusikkuna.
settings-desc-notifications = Kaikki tämän istunnon aikana näytetyt viestit ja toiminnot, jotka voit suorittaa uudelleen.
settings-desc-scene = Kohtausten tekstin asettelu: kirjasin, koko, rivinkorkeus, sisennykset ja välit.
settings-desc-synopsis = Tiivistelmäruudun asettelu käsikirjoituksesta riippumatta.
settings-desc-notes = Muistiinpanojen asettelu käsikirjoituksesta riippumatta.
settings-desc-corkboard = Korttien koko, mitä kortissa näkyy ja miten taulu järjestää kortit.
settings-desc-distraction-free = Typografia, palstan leveys ja ohjauspalkki koko näytön kirjoittamiseen.
settings-desc-distraction-free-themes = Koko näytön kirjoittamisen teemat: valmiit teemat ja omasi.
settings-desc-editor-behavior = Tekstin leveys, tiivistelmän sijainti, kirjoituskonevieritys ja kohdistimen korostus.
settings-desc-punctuation = Typografinen talotyyli, josta uudet projektit alkavat. Jokainen projekti noudattaa tätä, ellei se määritä omaansa.
settings-desc-goals = Sana- ja merkkitavoitteet sekä sanojen laskutapa.
settings-desc-games = Rajoitteet, jotka asetat itsellesi luonnostellessasi, kuten Aina eteenpäin.
settings-desc-spellcheck = Yksi kytkin, joka ottaa oikoluvun käyttöön tai pois käytöstä kaikkialla.
settings-desc-dictionaries = Asenna, poista ja selaa tämän tietokoneen oikolukusanakirjoja.
settings-desc-autosave = Kirjoitetaanko muokkauksesi levylle itsestään.
settings-desc-backup = Milloin kopiot otetaan, missä niitä säilytetään ja kuinka monta. Jokainen projekti noudattaa tätä, ellei se määritä omaansa.
settings-desc-export = Tyylit, joiden mukaan jokainen vienti kootaan: valmiit tyylit ja omasi.
settings-desc-paratext = Alku- ja loppusivut, joilla uusi projekti voi alkaa.
settings-desc-keymap = Kaikki pikanäppäimet ja mihin ne on sidottu.
settings-page-user = Käyttäjä
settings-desc-user = Kuka olet, kirjoittamiasi kommentteja varten.
settings-group-identity = Henkilöllisyys
settings-field-user-name = Nimesi
settings-field-user-name-placeholder = Valinnainen
settings-field-user-name-hint = Kirjoittamiesi kommenttien ja vastausten allekirjoitus. Eri asia kuin projektin tekijän nimi, joka on kirjan tekijätieto ja kulkee tiedoston mukana.
settings-field-user-initials = Nimikirjaimesi
settings-field-user-initials-placeholder = Nimestäsi
settings-field-user-initials-hint = Mitä tekstinkäsittelyohjelma näyttää kommenttisi vieressä marginaalissa. Jätä tyhjäksi, niin käytetään näytettyjä, nimestäsi muodostettuja nimikirjaimia.
settings-field-user-hint = Molemmat ovat valinnaisia ja koskevat kaikkia tämän tietokoneen projekteja. Muutos koskee seuraavaksi kirjoittamiasi kommentteja. Jo kirjoitetut kommentit säilyttävät nimen, jolla ne kirjoitettiin.
settings-desc-author = Tämän projektin tekijän nimi.
settings-desc-structure = Ovatko tämän projektin luvut kansioita vai yksittäisiä kohteita.
settings-desc-language = Kieli, jonka mukaan tämän projektin teksti oikoluetaan.
settings-desc-work-backup = Tämän projektin oma varmuuskopiointikäytäntö tai yleinen käytäntö.
settings-desc-personal-dictionary = Sanat, jotka tämä projekti hyväksyy oikein kirjoitetuiksi.
settings-desc-tags = Värikoodatut tunnisteet, joilla tämä projekti merkitsee kohteensa.
settings-desc-templates = Muistiinpanomallit, joita voit lisätä kirjoittaessasi tässä projektissa.
settings-desc-text-replacements = Lyhenteet, jotka laajenevat kirjoittaessasi tässä projektissa.
settings-desc-work-punctuation = Lainausmerkit, viivat ja välistys, joita tämän projektin teksti noudattaa.

settings-group-typography = Typografia
settings-group-writing-column = Kirjoituspalsta
settings-group-writing-view = Kirjoitusnäkymä
settings-group-distraction-free-strip = Ohjauspalkki
settings-group-theme = Teema
settings-group-language = Kieli
settings-group-startup = Käynnistys
settings-group-updates = Päivitykset
settings-group-autosave = Automaattinen tallennus
settings-field-typeface = Kirjasin
settings-field-size = Tekstikoko

editor-size-changed-manuscript = Käsikirjoituksen tekstikoko: { $percent }
editor-size-changed-synopsis = Tiivistelmän tekstikoko: { $percent }
editor-size-changed-notes = Muistiinpanojen tekstikoko: { $percent }
editor-size-changed-corkboard = Korkkitaulun korttien tekstikoko: { $percent }
editor-size-changed-corkboard-expanded = Laajennetun muokkaimen tekstikoko: { $percent }
editor-size-changed-distraction-free = Häiriöttömän tilan tekstikoko: { $percent }

menu-text-size-increase = &Suurenna tekstiä
menu-text-size-decrease = Pie&nennä tekstiä
menu-text-size-reset = Palauta tekstin k&oko
settings-field-line-height = Rivinkorkeus
settings-field-first-line-indent = Ensimmäisen rivin sisennys
settings-field-paragraph-spacing-before = Väli ennen kappaletta
settings-field-paragraph-spacing-after = Väli kappaleen jälkeen
settings-field-column-width = Palstan leveys
settings-distraction-free-width-hint = Koskee vain häiriötöntä tilaa. Kohtausten, tiivistelmien ja muistiinpanojen palstanleveydet eivät muutu.
settings-distraction-free-title = Säilytä kohteen nimi
settings-distraction-free-word-count = Säilytä sanamäärä
settings-distraction-free-session = Säilytä kirjoitusistunto
settings-distraction-free-go-to = Säilytä Siirry…-painike
settings-distraction-free-go = Säilytä Edellinen- ja Seuraava-painikkeet
settings-distraction-free-chrome-hint = Poistu-painike pysyy aina näkyvissä, valitsitpa tässä mitä tahansa. Sen kautta pääset ulos, jos Esc-näppäin on varattu.
settings-field-app-theme = Teema
settings-theme-light = Vaalea
settings-theme-dark = Tumma
settings-field-text-scale = Käyttöliittymän tekstikoko
settings-field-language = Käyttöliittymän kieli
settings-synopsis-placement = Tiivistelmän sijainti
settings-synopsis-placement-none = Ei näytetä
settings-synopsis-placement-top = Yläpuolella
settings-synopsis-placement-side = Vieressä
settings-typewriter = Kirjoituskonevieritys
settings-typewriter-tip = Pitää kirjoittamasi rivin kiinteällä korkeudella, ja käsikirjoitus vierii sen alla. Napsauttamalla kohdistin siirtyy edelleen napsautuskohtaan.
settings-typewriter-position = Rivin sijainti
settings-typewriter-position-top-third = Ylin kolmannes
settings-typewriter-position-middle = Keskellä
settings-typewriter-position-bottom-quarter = Alin neljännes
settings-highlight-scope = Korostus kohdistimen ympärillä
settings-highlight-scope-none = Ei mitään
settings-highlight-scope-sentence = Virke
settings-highlight-scope-paragraph = Kappale
settings-highlight-scope-tip-none = Sivu pysyy tavallisena. Mitään ei varjosteta kirjoittaessasi.
settings-highlight-scope-tip-sentence = Varjostaa kirjoittamasi virkkeen, jotta se erottuu ympäröivistä virkkeistä.
settings-highlight-scope-tip-paragraph = Varjostaa koko kappaleen, jota kirjoitat, jotta työn alla oleva kohta pysyy näkyvissä.
settings-group-container-views = Säiliöiden näkymät
settings-remember-view = Muista viimeinen näkymä kullekin kohdetyypille
settings-remember-view-tip = Avaa säiliö siinä näkymässä, jota käytit viimeksi samantyyppiselle kohteelle
settings-remember-view-tip-more =
       Kirjalla, osalla ja luvulla on kullakin useita näkymiä (oma sivu, koko
       käsikirjoitus, koko tiivistelmä). Kun asetus on päällä, kukin tyyppi avautuu
       viimeksi sille valitsemaasi näkymään: jos esimerkiksi vaihdat yhden luvun koko luvun
       näkymään, seuraavaksi avaamasi luku näyttää myös koko luvun. Kukin tyyppi muistaa oman näkymänsä.
settings-group-counting = Sanojen laskeminen
settings-counting-auto = Automaattinen (kielen mukaan)
settings-counting-whitespace = Jaa välilyöntien kohdalta
settings-counting-unicode-words = Unicode-sanat
settings-counting-cjk-hybrid = CJK-tietoinen (merkeittäin)
settings-counting-hint = Automaattinen laskee kiinan ja japanin merkeittäin ja kaikki muut kielet sanoittain. Muuta asetusta vain, jos määrä näyttää kielesi kohdalla väärältä.
settings-group-goals-display = Näyttö
settings-show-characters = Näytä merkkimäärä tilarivillä
settings-autosave-hint = Muutokset kirjoitetaan levylle automaattisesti kirjoittaessasi.

settings-sec-work = Projekti
settings-page-structure = Rakenne
settings-page-author = Tekijä
settings-field-author-name = Tekijän nimi
settings-field-author-placeholder = Valinnainen
settings-field-author-hint = Näkyy kootulla nimiösivulla ja vietyjen tiedostojen metatiedoissa. Jätä tyhjäksi, jos et halua sitä mukaan. Tämä on kirjan tekijätieto. Nimi, jolla kommenttisi allekirjoitetaan, on kohdassa Asetukset ▸ Käyttäjä.
settings-group-chapters = Luvut
tidy-titles-title = Siivoa lukujen otsikot
tidy-titles-none = Yksikään luvun tai osan otsikko ei pelkästään toista omaa numeroaan.
tidy-titles-lead = { $count ->
           [one] Yhden luvun tai osan otsikkona on pelkkä sen oma numero.
          *[other] { $count } luvun ja osan otsikkona on pelkkä niiden oma numero.
       }
tidy-titles-explain = Kun nämä otsikot tyhjennetään, kutakin kutsutaan sillä numerolla, jonka kirja jo tietää, samalla numerolla, jonka vienti tulostaa. Mikään muu ei muutu, ja yksi kumoaminen palauttaa ne kaikki.
menu-document-tidy-titles = Siivoa lukujen &otsikot…
settings-group-numbering = Numerointi
settings-number-chapters = Numeroi luvut ja osat
settings-number-chapters-tip = Luvuilla ja osilla on numero, joka määräytyy niiden paikasta kirjassa. Se näytetään tässä otsikon vieressä ja tulostetaan viennissä.
settings-number-chapters-tip-more = Numeroa ei koskaan tallenneta otsikkoon, joten se pysyy oikeana, kun järjestät, lisäät tai poistat. Jos asetus on pois päältä, vienti tulostaa pelkät otsikot, pyysipä vientityyli mitä tahansa. Jos haluat jättää yhden luvun numeroinnin ulkopuolelle (prologin, välinäytöksen), käytä sen sijaan Ominaisuudet-paneelin Numerointi-kytkintä: luku pysyy kirjassa mutta ei saa numeroa.
settings-part-resets-chapter = Aloita lukujen numerointi alusta joka osassa
settings-part-resets-chapter-tip = Oletuksena pois päältä: luvut jatkuvat kirjan osien yli, joten ”Toinen osa” alkaa luvusta yksitoista.
settings-part-resets-chapter-tip-more = Tämä on tavallinen kustannuskäytäntö ja sitä lukija odottaa. Ota asetus käyttöön kirjassa, jonka osat on tarkoitettu luettaviksi erillisinä niteinä, joista kukin alkaa luvusta yksi.
settings-chapter-flat = Kansiottomat luvut
settings-chapter-flat-hint = Päällä: luku on yksittäinen kohde. Kirjoitat siihen, eikä se sisällä kohtauksia. Pois: luku on kansio. Voit edelleen kirjoittaa siihen, mutta se voi sisältää myös kohtauksia. Uudet luvut noudattavat tätä asetusta; olemassa olevat muunnetaan toiminnolla Muunna tyypiksi.

settings-styles-builtin = Sisäänrakennetut tyylit
settings-styles-user = Omat tyylit
settings-styles-builtin-badge = Sisäänrakennettu
settings-styles-duplicate = Monista
settings-styles-edit = Muokkaa
settings-styles-delete = Poista
settings-styles-import = Tuo…
settings-styles-export = Vie…
settings-styles-copy-suffix = (kopio)
settings-styles-json-filter = Vientityyli
settings-styles-editor-title = Muokkaa tyyliä
settings-styles-editor-none = Valitse oma tyyli muokattavaksi tai monista valmis tyyli.
settings-styles-imported = Tyyli tuotu
settings-styles-import-failed = Tyyliä ei voitu tuoda
settings-styles-exported = Tyyli viety
settings-styles-export-failed = Tyyliä ei voitu viedä
settings-styles-field-name = Nimi
settings-styles-field-chapters = Lukujen otsikot
settings-styles-field-parts = Osien otsikot
settings-styles-field-scene-break = Kohtauskatko
settings-styles-field-major-scene-break = Suuri kohtauskatko
settings-styles-field-spacing = Riviväli
settings-styles-field-justify = Tasaa molemmat reunat
settings-styles-field-notes = Sisällytä muistiinpanot
settings-styles-field-synopses = Sisällytä tiivistelmät
settings-styles-field-scene-titles = Sisällytä kohtauksen otsikot
settings-styles-field-epigraphs = Sisällytä motot
settings-styles-field-paratexts = Sisällytä paratekstit
settings-styles-field-epigraph-placement = Moton sijainti
settings-styles-epigraph-after = Otsikon jälkeen
settings-styles-epigraph-before = Ennen otsikkoa
settings-styles-field-footnotes = Sisällytä alaviitteet
settings-styles-field-footnote-numbering = Alaviitteiden numerointi
settings-styles-footnote-numbering-continuous = Jatkuva
settings-styles-footnote-numbering-per-chapter = Alusta jokaisessa luvussa
settings-styles-footnote-numbering-per-book = Alusta jokaisessa kirjassa
settings-styles-field-images = Kuvat
menu-image = &Kuva
settings-styles-images-beside = Asiakirjan vieressä
settings-styles-images-embed = Asiakirjan sisällä
settings-styles-images-omit = Jätä pois
settings-styles-group-round-trip = Lähetys kustannustoimittajalle
settings-styles-field-comments = Sisällytä kommentit
settings-styles-field-round-trip-marks = Sisällytä paluumerkit
settings-styles-round-trip-hint = Vain DOCX ja ODT. Paluumerkit ovat näkymättömiä tunnisteita, joiden ansiosta palaava tiedosto päivittää tämän projektin sen sijaan, että siitä syntyisi toinen kopio sen rinnalle.
settings-styles-group-pages = Sivut
settings-styles-field-cover = Aloita kansikuvalla
settings-styles-field-word-count = Sanamäärä otsikkosivulla
settings-styles-field-page-books = Uusi sivu jokaisessa kirjassa
settings-styles-field-page-parts = Uusi sivu jokaisessa osassa
settings-styles-field-page-chapters = Uusi sivu jokaisessa luvussa
settings-styles-field-page-paratexts = Uusi sivu jokaisessa paratekstissä
settings-styles-heading-none = Ei otsikkoa
settings-styles-heading-numbered = Vain numero
settings-styles-heading-title = Vain otsikko
settings-styles-heading-both = Numero + otsikko
settings-styles-break-blank = Tyhjä rivi
settings-styles-break-none = Ei mitään
settings-styles-spacing-single = Yksinkertainen
settings-styles-spacing-onehalf = 1½
settings-styles-spacing-double = Kaksinkertainen

welcome-title = Tervetuloa Skribistoon
welcome-version = Versio { $version }
welcome-update-available = Versio { $version } on saatavilla
welcome-update-tip = Avaa lataussivun selaimessasi.
welcome-search = Hae projekteista
welcome-open = Avaa
welcome-new-work = Uusi projekti
welcome-create-from = Luo lähteestä…
welcome-recent-works = Viimeisimmät projektit
welcome-empty-recents = Ei vielä viimeisimpiä projekteja.
welcome-no-matches = Mikään viimeisimmistä projekteista ei vastaa hakuasi.
welcome-learn-soon = Oppaat ja vinkit tulevat pian.
welcome-about-blurb = Skribisto, kirjoitussovellus uudelleen kirjoitettuna Rustilla ja Teksilolla.
welcome-tagline = *Rauhallinen paikka pitkien tekstien kirjoittamiseen.*
welcome-github = GitHub
welcome-discord = Discord
nav-works = Projektit
nav-examples = Esimerkit
nav-learn = Opi
nav-about = Tietoja

synopsis = Tiivistelmä
epigraph = Motto
pane-manuscript = Käsikirjoitus
corkboard = Korkkitaulu
overview = Yleiskatsaus

corkboard-card-count = { $count ->
       [one] { $count } kortti
      *[other] { $count } korttia
   }
corkboard-child-count = { $count ->
       [one] { $count } kohde
      *[other] { $count } kohdetta
   }
corkboard-view-nested = Sisäkkäinen
corkboard-view-flat = Litteä
corkboard-layout-hint = Sisäkkäinen näyttää säiliön välittömät alikohteet. Avaa kansiokortti siirtyäksesi sen sisään. Litteä näyttää kaikki koko säiliön kohtaukset kerralla.
corkboard-card-size = Kortin koko
corkboard-search-placeholder = Suodata kortteja…
corkboard-empty-title = Täällä ei ole vielä mitään
corkboard-empty-hint = Lisää ensimmäinen yllä olevalla ”＋ Uusi”-painikkeella.
corkboard-new = Uusi
corkboard-show-card-numbers = Numeroi kortit
corkboard-modal-size = Laajennetun muokkaimen koko
corkboard-scope-hint = Koskee jokaista avointa korkkitaulua joka projektissa.
corkboard-sort-manuscript = Käsikirjoituksen järjestys
corkboard-sort-title-asc = Otsikko A–Ö
corkboard-sort-title-desc = Otsikko Ö–A
corkboard-card-number = Kortti { $number }

duplicate = Monista
duplicate-n = { $count ->
       [one] Monista { $count } kortti
      *[other] Monista { $count } korttia
   }
set-label-n = { $count ->
       [one] Aseta nimiö { $count } kortille
      *[other] Aseta nimiö { $count } kortille
   }
move-to-trash-n = { $count ->
    [one] Siirrä { $count } kortti roskakoriin
   *[other] Siirrä { $count } korttia roskakoriin
}
reveal-in-outline = Näytä jäsennyksessä

corkboard-move-to = Siirrä kohteeseen…
corkboard-move-to-n = { $count ->
       [one] Siirrä { $count } kortti kohteeseen…
      *[other] Siirrä { $count } korttia kohteeseen…
   }
corkboard-move-picker-title = Siirrä kohteeseen…
corkboard-move-picker-empty = Mappeja ei ole vielä. Luo ensin mappi.
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
overview-col-label = Nimiö
overview-col-tags = Tunnisteet
overview-col-books = Kirjat
overview-col-own-words = Sanat
overview-col-total-words = Yhteensä
overview-row-count = { $count ->
       [one] { $count } kohde
      *[other] { $count } kohdetta
   }
overview-search-placeholder = Suodata kohteita…
overview-expand-all = Laajenna kaikki
overview-collapse-all = Supista kaikki
overview-table-label = Sisällys
overview-empty-title = Täällä ei ole vielä mitään
overview-gone-title = Tätä säiliötä ei enää ole
overview-gone-hint = Se siirrettiin roskakoriin. Palauta se tai sulje tämä välilehti.
overview-empty-hint = Lisää ensimmäinen yllä olevalla ”＋ Uusi”-painikkeella.
corkboard-grid-label = Korkkitaulun kortit
corkboard-rename-field = Nimeä kohde uudelleen
corkboard-expand-synopsis = Laajenna tiivistelmä
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
corkboard-show-word-count = Näytä sanamäärä korteissa
text-heading = Teksti
no-content = Tällä kohteella ei ole muokattavaa sisältöä.
untitled = Nimetön
placeholder-title = Otsikko…
placeholder-subtitle = Alaotsikko…
placeholder-chapter-title = Luvun otsikko…
split-editor = Jaa muokkain
close-split-view = Sulje jakonäkymä
drop-open-here = Avaa tähän
drop-open-to-side = Avaa sivulle

segment-chapter = Luku
segment-part = Osa
segment-book = Kirja
segment-pace = Tahti
segment-notes = Muistiinpanot
segment-story-bible = Tarinan tietopankki
segment-note-own = Muistiinpano
segment-note-details = Yksityiskohdat
segment-note-in-prose = Tekstissä
pace-placeholder = Tahtisuunnitelma näkyy tässä.
pace-empty-title = Suunnittele tämän kirjan tahti
pace-empty-body = Aseta sanatavoite ja määräaika, niin Skribisto laskee päivittäisen tahdin, jolla pääset perille.
pace-start-planning = Aloita suunnittelu
pace-section-schedule = Aikataulu
pace-section-progress = Edistyminen
pace-advancement = Eteneminen
pace-goal = Sanatavoite
pace-deadline = Määräaika
pace-active = Tahtisuunnitelma käytössä
pace-day-mon = ma
pace-day-tue = ti
pace-day-wed = ke
pace-day-thu = to
pace-day-fri = pe
pace-day-sat = la
pace-day-sun = su
pace-card-written = sanaa kirjoitettu
pace-card-of-goal = tavoitteesta
pace-card-rate = sanaa / kirjoituspäivä
pace-card-days-left = kirjoituspäiviä jäljellä
pace-card-streak = päivän putki
pace-card-ahead = sanaa tahtia edellä
pace-card-behind = sanaa tahdista jäljessä
pace-charts-empty = Edistymiskaaviot näkyvät tässä, kun sanamääräsi on tallennettu tallennuksen yhteydessä.
pace-chart-progression = Kirjoitetut sanat vs. tavoite
pace-chart-words-per-day = Sanaa päivässä
pace-series-actual = Todellinen
pace-series-target = Tavoite
pace-series-words-per-day = Sanaa/päivä
pace-daily-target-line = Tasainen tahti: { $count ->
       [one] { $count } sana/päivä
      *[other] { $count } sanaa/päivä
   }
pace-section-holidays = Vapaapäivät
pace-section-milestones = Virstanpylväät
pace-holidays-none = Ei vapaapäiviä. Jokainen aikataulun päivä lasketaan.
pace-holiday-label = Vapaapäivän nimi
pace-add-holiday = Lisää
pace-remove = Poista
pace-milestones-none = Ei vielä virstanpylväitä. Aseta sellainen osalle tai luvulle Ominaisuudet-paneelissa.
full-chapter = Koko luku
full-part = Koko osa
full-book = Koko kirja
full-synopsis = Koko tiivistelmä
rename = Nimeä uudelleen
set-label = Aseta nimiö
insert-scene = Lisää kohtaus
insert-chapter = Lisää luku
split-scene = Jaa kohtaus
move-up = Siirrä ylös
move-down = Siirrä alas
merge-with-previous = Yhdistä edelliseen
move-to-trash = Siirrä roskakoriin
rename-chapter = Nimeä luku uudelleen
add-scene = Lisää uusi kohtaus
add-chapter = Lisää uusi luku
new-scene-title = Uusi kohtaus
placeholder-scene-name = Kohtauksen nimi…
menu-cut = Leikkaa
menu-copy = Kopioi
menu-paste = Liitä
menu-paste-unformatted = Liitä muotoilematon
menu-select-all = Valitse kaikki
format-bold = Lihavointi
format-italic = Kursivointi
format-underline = Alleviivaus
format-strikethrough = Yliviivaus

binder = Mappi
no-work = Projektia ei ole ladattu
no-recent-works = Ei viimeisimpiä projekteja
switcher-open-section = Tällä hetkellä avoinna
switcher-recent-section = Viimeisimmät
switcher-this-window = tämä ikkuna
open-project-title = Avaa projekti
open-project-question = Miten haluat avata projektin ”{ $title }”?
open-in-new-window = Avaa uudessa ikkunassa
open-here = Avaa tähän

binder-all = Kaikki mapit
binder-show-all = Näytä kaikki mapit
binder-new = Uusi mappi…
binder-default-name = Uusi mappi
binder-item-count = { $count ->
       [one] { $count } kohde
      *[other] { $count } kohdetta
   }
binder-search-placeholder = Suodata jäsennystä…
binder-search-scope = Hae kaikista mapeista
binder-trash-confirm-title = Siirretäänkö mappi roskakoriin?
binder-trash-confirm-text = ”{ $name }” ja kaikki sen kohteet siirretään roskakoriin.

dialog-rename = Nimeä uudelleen
dialog-new-binder = Uusi mappi
dialog-set-label = Aseta nimiö
dialog-new-scene = Uusi kohtaus
close-work-question = Tallennetaanko muutokset ennen projektin sulkemista?
quit-question = Tallennetaanko muutokset ennen lopettamista?
quit-save-work-question = Tallennetaanko projektin { $title } muutokset ennen lopettamista?
unsaved-changes = Tässä projektissa on tallentamattomia muutoksia.
new-work-unsaved-question = Tallennetaanko muutokset ennen uuden projektin luomista?
open-work-unsaved-question = Tallennetaanko muutokset ennen toisen projektin avaamista?
switch-backup-discard-title = Hylätäänkö tähän varmuuskopioon tehdyt muutokset?
switch-backup-discard-text = Varmuuskopioon tehtyjä muutoksia ei voi tallentaa siihen. Säilytä ne toiminnolla Tallenna nimellä tai Palauta, tai hylkää ne ja avaa toinen projekti.
switch-save-failed = Projektia ei voitu tallentaa, joten sitä ei korvattu: { $error }
switch-save-not-started = Projektia ei voitu tallentaa, joten sitä ei korvattu.
close-save-failed = Projektia ei voitu tallentaa, joten sitä ei suljettu: { $error }
close-save-not-started = Projektia ei voitu tallentaa, joten sitä ei suljettu.
save-not-started = Projektia ei voitu tallentaa.

could-not-open-work = Tiedostoa ”{ $file }” ei voitu avata
could-not-open-work-too-new = ”{ $file }” vaatii uudemman Skribiston
could-not-open-work-too-new-detail = Tallennettu Skribisto-muodossa { $written_by }. Sen avaaminen vaatii muodon { $requires } tai uudemman, ja tämä versio tukee enintään muotoa { $supported }. Päivitä Skribisto avataksesi sen.
could-not-open-work-importing = Tuonti kirjoittaa parhaillaan tiedostoa ”{ $file }”
could-not-open-work-importing-detail = Tuonti kirjoittaa tämän projektin päälle eikä ole vielä valmis. Avaa projekti, kun tuonti on päättynyt: jos se avattaisiin nyt, näkyisi projekti, jonka tuonti on korvaamassa, ja seuraava tallennus kirjoittaisi sen takaisin tuonnin päälle.
could-not-open-example = Esimerkkiä ei voitu avata: { $error }
could-not-open-link = Osoitetta { $url } ei voitu avata: { $error }
could-not-create-work = Projektia ei voitu luoda: { $error }
saving-as-file = Tallennetaan nimellä { $target }…
saving-as-folder = Tallennetaan nimellä { $target }/…
saved-as = Tallennettu: { $target }
save-error = Tallennus epäonnistui: { $error }
save-flush-failed = Tallennus onnistui, mutta näitä asiakirjoja ei voitu välittää sille, joten ne ovat yhä tallentamatta: { $titles } ({ $error })
backup-error = Varmuuskopiointi epäonnistui: { $error }
backing-up = Varmuuskopioidaan…
backup-nothing-open = Varmuuskopioitavaa projektia ei ole auki.
backup-already-running = Varmuuskopiointi on jo käynnissä.
backup-complete = Varmuuskopiointi valmis ({ $ok } tallennettu, { $skipped } jo ajan tasalla)
backup-partial = Varmuuskopiointi päättyi: { $ok } tallennettu, { $failed ->
       [one] { $failed } sijainti epäonnistui
      *[other] { $failed } sijaintia epäonnistui
   }
backup-no-destination-title = Varmuuskopioiden sijaintia ei ole käytettävissä
backup-no-destination-text = Yhteenkään määritetyistä varmuuskopioiden sijainneista ei saada yhteyttä (esimerkiksi ulkoinen asema voi olla irrotettu). Kytke se ja yritä uudelleen, tai lopeta varmuuskopioimatta.
backup-failed-close-title = Varmuuskopiota ei voitu tallentaa
backup-failed-close-text = Varmuuskopiota ei voitu kirjoittaa ennen sulkemista. Kaikki sijainnit epäonnistuivat (asema on ehkä irrotettu, tai se voi olla täynnä tai kirjoitussuojattu). Korjaa tilanne ja yritä uudelleen, tai lopeta varmuuskopioimatta.

backup-choice-title = Varmuuskopiotiedosto
backup-choice-heading = Olet avannut varmuuskopion
backup-choice-subtitle = Tämä on projektin tietyn hetken varmuuskopio.
backup-choice-subtitle-dated = Varmuuskopio otettu { $date }.
backup-choice-body = Voit avata sen ja muokata sitä vapaasti, mutta muutokset voi säilyttää vain toiminnolla Tallenna nimellä. Alkuperäiseen projektitiedostoon ei kosketa. Voit myös palauttaa projektin täsmälleen tähän varmuuskopioon.
backup-choice-open = Avaa varmuuskopio
backup-choice-restore = Palauta projekti tähän ajankohtaan…
backup-choice-not-a-backup = Ei, avaa se tavallisesti
backup-banner-title = Varmuuskopio: muutoksia ei voi tallentaa tähän
backup-banner-description = Säilytä muokkauksesi uudessa tiedostossa toiminnolla Tallenna nimellä, tai korvaa alkuperäinen projekti tällä varmuuskopiolla toiminnolla Palauta.
backup-banner-restore = Palauta…
backup-banner-save-as = Tallenna nimellä…
backup-restore-original-missing = Alkuperäistä projektia, jonka päälle palautettaisiin, ei löydy. Säilytä tämä varmuuskopio uutena projektina toiminnolla Tallenna nimellä.
backup-restore-close-elsewhere-title = Projekti on auki toisessa ikkunassa
backup-restore-close-elsewhere-text = Projekti, jonka päälle palautat, on auki toisessa ikkunassa. Sulje se ensin siellä ja yritä sitten uudelleen.
backup-restore-focus-window = Siirry siihen ikkunaan
backup-restore-confirm-title = Palautetaanko tämä varmuuskopio?
backup-restore-confirm-text = Projektin nykyinen versio kopioidaan sivuun turvavarmuuskopioksi, ennen kuin se korvataan tällä.
backup-restore-confirm-ok = Palauta
backup-restore-error = Palautus epäonnistui: { $error }
backup-restored-ok = Projekti palautettu.
backup-restored-with-safety = Projekti palautettu. Edellinen versiosi tallennettiin kohteeseen { $path }.
close-backup-discard-title = Hylätäänkö tähän varmuuskopioon tehdyt muutokset?
close-backup-discard-text = Varmuuskopioon tehtyjä muutoksia ei voi tallentaa siihen. Säilytä ne toiminnolla Tallenna nimellä, tai hylkää ne ja sulje.
quit-backup-discard-title = Hylätäänkö muutokset ja lopetetaan?
quit-backup-discard-work-question = Hylätäänkö projektin { $title } muutokset ja lopetetaan?
quit-backup-discard-text = Varmuuskopioon tehtyjä muutoksia ei voi tallentaa siihen. Säilytä ne toiminnolla Tallenna nimellä, tai hylkää ne ja lopeta.
backup-nudge-text = Tälle projektille ei ole määritetty varmuuskopiointia.
backup-nudge-action = Määritä varmuuskopiointi…

menu-backups-list = Var&muuskopiot…
backups-title = Varmuuskopiot
backups-loading = Etsitään varmuuskopioita…
backups-empty = Tälle projektille ei vielä löytynyt varmuuskopioita.
backups-open = Avaa
backups-reveal = Näytä sijainti
backups-delete = Poista tämä varmuuskopio
backups-delete-confirm-title = Poistetaanko tämä varmuuskopio?
backups-delete-confirm-text = ”{ $name }” poistetaan pysyvästi. Toimintoa ei voi kumota.
backups-delete-error = Varmuuskopiota ei voitu poistaa: { $error }
backups-refresh = Päivitä
backups-close = Sulje

settings-page-backup = Varmuuskopioinnin oletukset
settings-page-work-backup = Varmuuskopiot
settings-backup-general-title = Varmuuskopioinnin oletusasetukset
settings-backup-work-title = Tämän projektin varmuuskopiot
settings-backup-inherit = Käytä yleisiä varmuuskopioasetuksia
settings-backup-inheriting = Tässä projektissa käytetään yleisiä varmuuskopioasetuksia.
settings-backup-none-hint = Automaattista varmuuskopiointia ei ole määritetty (kaikki ehdot ovat pois päältä).
settings-backup-last = Viimeisin varmuuskopio: { $date }
settings-backup-last-never = Ei vielä varmuuskopioita.
settings-backup-open-list = Avaa varmuuskopioluettelo…
settings-backup-triggers = Milloin varmuuskopioidaan
settings-backup-on-close = Varmuuskopioi projektia suljettaessa
settings-backup-on-open = Varmuuskopioi projektia avattaessa
settings-backup-interval = Varmuuskopioi säännöllisesti, välein
settings-backup-destinations = Varmuuskopioiden sijainnit
settings-backup-default-location = Oletussijainti
settings-backup-usage-since = { $count ->
       [one] { $count } varmuuskopio · { $size } · vanhin { $oldest }
      *[other] { $count } varmuuskopiota · { $size } · vanhin { $oldest }
   }
settings-backup-usage = { $count ->
       [one] { $count } varmuuskopio · { $size }
      *[other] { $count } varmuuskopiota · { $size }
   }
settings-backup-usage-empty = Täällä ei ole vielä säilytetty mitään
settings-backup-usage-measuring = Mitataan…
settings-backup-reveal-root = Näytä kansio
settings-backup-dest-none = Ei sijainteja. Varmuuskopiot tallennetaan projektin viereen.
settings-backup-dest-remove = Poista
settings-backup-dest-add = Lisää kansio…
settings-backup-dest-refresh = Päivitä
settings-backup-retention = Montako säilytetään
settings-backup-retention-tiered = Portaittain
settings-backup-retention-keep-n = Säilytä viimeiset N
settings-backup-retention-tip = Miten vanhoja varmuuskopioita karsitaan.
settings-backup-retention-tip-more = Portaittain säilyttää yhden varmuuskopion tuntia kohden vuorokauden ajan, päivää kohden viikon ajan, viikkoa kohden kuukauden ajan ja sen jälkeen kuukautta kohden, joten tuore historia pysyy tiheänä ja vanha harvenee. ”Säilytä viimeiset N” säilyttää yksinkertaisesti N uusinta kopiota. Kummassakin tilassa uusimmat kopiot (alla oleva vähimmäismäärä) säilytetään aina.
settings-backup-gfs-hourly = Tunneittain (viimeiset 24 h)
settings-backup-gfs-daily = Päivittäin (viimeinen viikko)
settings-backup-gfs-weekly = Viikoittain (viimeinen kuukausi)
settings-backup-gfs-monthly = Kuukausittain
settings-backup-keep-n = Säilytettävä määrä
settings-backup-min-keep = Säilytä aina vähintään
settings-backup-dedup = Ohita varmuuskopio, kun mikään ei ole muuttunut

new-work-title = Uusi projekti
new-work-close = Sulje
new-work-name = Projektin nimi
new-work-name-placeholder = Nimetön
new-work-author = Tekijä
new-work-author-placeholder = Valinnainen
new-work-format = Tallennusmuoto
new-work-single-file = Yksi tiedosto
new-work-bundle = Projektikansio
new-work-convert-later = Voit vaihtaa muotoa myöhemmin.
new-work-location = Sijainti
new-work-will-create = Luodaan
new-work-language = Oletuskieli
new-work-language-hint = Koskee uusia tekstejä ja oikolukua. Jokaisen tekstin voi vaihtaa toiselle kielelle.
new-work-template = Malli
new-work-template-none = Ei mitään
new-work-template-empty-novel = Tyhjä romaani
new-work-template-light-novel = Kevytromaani
new-work-template-novel = Romaani
new-work-template-novel-in-parts = Osiin jaettu romaani
new-work-template-notebook = Muistikirja
new-work-cancel = Peruuta
new-work-create = Luo projekti
new-work-step-details = Yksityiskohdat
new-work-step-language = Kieli ja rakenne
new-work-step-template = Malli
new-work-back = Takaisin
new-work-next = Seuraava
new-work-documents-title = Uusi projekti asiakirjoista
new-work-step-import = Tuo
new-work-create-and-import = Luo ja tuo…
new-work-documents-next-title = Asiakirjasi tulevat seuraavaksi
new-work-documents-next-body = Projektin luominen avaa sen päälle tuontiohjatun toiminnon, jossa valitset asiakirjat, tarkistat rakenteen, jonka Skribisto niistä lukee, ja valitset, mihin se sijoitetaan. Projektiin ei kirjoiteta mitään ennen kuin vahvistat sen siellä.
new-work-documents-no-template = Tämä projekti alkaa tarkoituksella tyhjänä: ei mallia, joten tuodut asiakirjat ovat sen ainoa sisältö.
new-work-documents-chapter-scene-hint = Koskee tuonnin luomia lukuja. Voit muuttaa tätä myöhemmin projektin asetuksissa.
new-work-single-file-desc = Yksi .skrib-arkisto (zip). Siirrettävä ja helppo varmuuskopioida.
new-work-bundle-desc = Kansio, joka sisältää kaikki tekstit ja liitetiedostot. Sopii paremmin versionhallintaan.
new-work-template-none-count = tyhjä mappi
new-work-template-empty-novel-count = 1 luku
new-work-template-light-novel-count = 15 lukua
new-work-template-novel-count = 20 lukua
new-work-template-novel-in-parts-count = 3 osaa, 24 lukua
new-work-template-notebook-count = vapaamuotoiset muistiinpanot
new-work-chapter-scene = Kansiottomat luvut
new-work-chapter-scene-tip = Jokainen luku on yksittäinen kohde, johon kirjoitat suoraan, eikä sen alla ole kohtauksia. Jätä tämä pois päältä perinteistä rakennetta varten, jossa luku on kansio: siihenkin kirjoitetaan suoraan, mutta se voi sisältää myös kohtauksia.
new-work-chapter-scene-tip-more = Lukuun kirjoitetaan kummassakin tapauksessa. Ainoa ero on, voiko se *sisältää* kohtauksia. Skribiston mappipuu on pelkästään järjestelyä varten, joten molemmat rakenteet kootaan samaksi kirjaksi, voit sekoittaa niitä vapaasti, ja Muunna tyypiksi vaihtaa luvun rakenteesta toiseen menettämättä sanaakaan.
new-work-name-required = Anna projektille nimi
new-work-name-invalid = Tässä nimessä ei ole käyttökelpoisia merkkejä
new-work-location-required = Valitse sijainti
new-work-location-missing = Tätä kansiota ei ole olemassa
new-work-location-not-folder = Tämä polku ei ole kansio
new-work-location-readonly = Tähän kansioon ei voi kirjoittaa
new-work-target-exists = Tässä kansiossa on jo tämän niminen projekti. Projektia luotaessa kysytään ennen sen korvaamista
new-work-target-is-folder = Tämän niminen kansio on jo tässä paikassa. Valitse toinen nimi
new-work-overwrite-title = Korvataanko olemassa oleva projekti?
new-work-overwrite-text = ”{ $name }” on jo tässä kansiossa. Korvataanko se uudella projektilla?
new-work-target-open-title = Projekti on auki
new-work-target-open-text = ”{ $name }” on auki Skribistossa. Uuden projektin luominen sen tilalle korvaisi projektin, jota työstät, ja seuraava tallennuksesi kirjoittaisi sen takaisin uuden päälle. Valitse toinen nimi tai kansio, tai sulje projekti ensin.
target-importing-title = Tuonti kirjoittaa tätä projektia
target-importing-text = Tuonti kirjoittaa vielä projektia ”{ $name }”. Odota, kunnes se on valmis, tai valitse toinen nimi.
backup-restore-importing-text = Tuonti kirjoittaa vielä projektia ”{ $name }”, jonka päälle tämä varmuuskopio palautettaisiin. Palauta varmuuskopio, kun tuonti on päättynyt.

new-work-manuscript = Käsikirjoitus
new-work-notes = Muistiinpanot
new-work-research = Tutkimus
new-work-notebook = Muistikirja
new-work-scene = Kohtaus
new-work-note = Muistiinpano
new-work-front-matter = Alkusivut
new-work-back-matter = Loppusivut
new-work-paratext = Kirjan rakenne
new-work-paratext-none = Ei rakennetta
new-work-paratext-hint = Alku- ja loppusivut, joilla perinne avaa ja päättää kirjan. Voit siirtää, nimetä uudelleen tai poistaa minkä tahansa niistä myöhemmin.
new-work-tags = Tunnisteet
new-work-tags-none = Ei tunnisteita
new-work-tags-hint = Valmis valikoima henkilöiden, paikkojen ja muun merkitsemiseen. Valinnainen, ja jokaisen tunnisteen voi myöhemmin nimetä uudelleen, värittää toisin tai poistaa.
new-work-note-templates = Muistiinpanomallit
new-work-note-templates-none = Ei malleja
new-work-note-templates-hint = Muoto, josta tietopankin muistiinpano alkaa. Valinnainen, ja jokaista mallia voi myöhemmin muokata tai sen voi poistaa.
new-work-characters = Henkilöt
new-work-places = Paikat

import-plume-title = Tuo Plume Creator -projekti
import-plume-source = Plume-projekti
import-plume-source-hint = Valitse .plume- tai .plume_backup-tiedosto (mikä tahansa Plume Creatorin versio).
import-plume-trash-warning = ⚠ Roskakoriin siirrettyjä tai poistettuja kohteita ei siirretä.
import-plume-source-required = Valitse Plume-projektitiedosto
import-plume-source-missing = Tätä tiedostoa ei ole olemassa
import-plume-source-not-file = Tämä polku ei ole tiedosto
import-plume-manuscript-binder = Käsikirjoitus
import-plume-story-bible-binder = Tarinan tietopankki
import-plume-progress-title = Tuodaan Plume-projektia…
import-plume-done = Tuotiin { $imported ->
       [one] { $imported } kohde.
      *[other] { $imported } kohdetta.
   } { $skipped ->
       [one] { $skipped } roskakorissa ollut kohde jätettiin siirtämättä.
      *[other] { $skipped } roskakorissa ollutta kohdetta jätettiin siirtämättä.
   }
import-plume-nested-too-deep = Projektia ei tuotu: sen osa ”{ $part }” on sisäkkäistetty yli { $limit } tason syvyyteen. Mikään Plume Creator -projekti ei ole lähelläkään sitä, ja sen lukeminen voisi kaataa Skribiston. Mitään ei kirjoitettu.
import-plume-declares-entities = Projektia ei tuotu: sen osa ”{ $part }” määrittelee XML-entiteettejä, mitä yksikään Plume Creator -projekti ei tee. Niiden laajentaminen voisi viedä enemmän muistia kuin tietokoneessa on ja kaataa Skribiston. Mitään ei kirjoitettu.

import-manuskript-title = Tuo Manuskript-projekti
import-manuskript-source = Manuskript-projekti
import-manuskript-source-hint = Valitse .msk-tiedosto tai projektikansio (mikä tahansa Manuskriptin versio).
import-manuskript-source-file = Valitse tiedosto…
import-manuskript-source-folder = Valitse kansio…
import-manuskript-source-required = Valitse Manuskript-projekti
import-manuskript-source-missing = Tätä tiedostoa tai kansiota ei ole olemassa
import-manuskript-manuscript-binder = Käsikirjoitus
import-manuskript-story-bible-binder = Tarinan tietopankki
import-manuskript-characters-group = Henkilöt
import-manuskript-world-group = Maailma
import-manuskript-plots-group = Juonet
import-manuskript-project-info-note = Projektin tiedot
import-manuskript-summary-note = Yhteenveto
import-manuskript-importance-minor = Vähäinen
import-manuskript-importance-secondary = Sivuhenkilö
import-manuskript-importance-main = Päähenkilö
import-manuskript-progress-title = Tuodaan Manuskript-projektia…
import-manuskript-done = { $imported ->
       [one] { $imported } kohde tuotiin.
      *[other] { $imported } kohdetta tuotiin.
   } { $revisions ->
       [0] { "" }
       [one] Sen mukana tuli { $revisions } aiempi versio.
      *[other] Niiden mukana tuli { $revisions } aiempaa versiota.
   }
import-manuskript-nested-too-deep = Projektia ei tuotu: sen tiedosto ”{ $part }” on sisäkkäistetty yli { $limit } tason syvyyteen. Mikään Manuskript-projekti ei ole lähelläkään sitä, ja sen lukeminen voisi kaataa Skribiston. Mitään ei kirjoitettu.
import-manuskript-folders-too-deep = Projektia ei tuotu: yksi sen tiedostoista on yli { $limit } kansion syvyydessä. Mikään Manuskript-projekti ei ole lähelläkään sitä, ja sen lukeminen voisi kaataa Skribiston. Mitään ei kirjoitettu.
import-manuskript-declares-entities = Projektia ei tuotu: sen tiedosto ”{ $part }” määrittelee XML-entiteettejä, mitä yksikään Manuskript-projekti ei tee. Niiden laajentaminen voisi viedä enemmän muistia kuin tietokoneessa on ja kaataa Skribiston. Mitään ei kirjoitettu.

import-target-open-title = Projekti on auki
import-target-open-text = ”{ $name }” on auki Skribistossa. Tuonti sen päälle korvaisi projektin, jota työstät, ja seuraava tallennuksesi kirjoittaisi sen takaisin tuonnin päälle. Valitse toinen tiedostonimi, tai sulje projekti ensin.
import-warnings-announcement = { $result } { $notice }.

export-title = Vie
export-close = Sulje
export-scope-label = Mitä
export-format-label = Tiedostomuoto
export-style-label = Tyyli
export-path-label = Tiedosto
export-browse = Selaa…
export-show-non-exportable = Näytä myös vientiin kuulumattomat
export-choose-empty = Ei valittavaa projektia.
export-cancel = Peruuta
export-export = Vie
export-save-dialog-title = Vie tiedostoon
export-section-what = Mitä viedään
export-section-style = Tyyli
export-section-destination = Määränpää
export-custom-selection = Oma valinta
export-selected-count = { $count } valittu
export-chip-chapters-none = Luvut: ei otsikkoa
export-chip-chapters-numbered = Luvut: numeroitu
export-chip-chapters-title = Luvut: vain otsikko
export-chip-chapters-both = Luvut: numero + otsikko
export-chip-scene-break-glyph = Kohtauskatko: { $glyph }
export-chip-scene-break-blank = Kohtauskatko: tyhjä rivi
export-chip-scene-break-none = Kohtauskatko: ei mitään
export-chip-major-break-glyph = Suuri katko: { $glyph }
export-chip-major-break-blank = Suuri katko: tyhjä rivi
export-chip-major-break-none = Suuri katko: ei mitään
export-chip-major-break-same = Molemmat katkotasot samanlaiset
export-chip-spacing-single = Riviväli: yksinkertainen
export-chip-spacing-onehalf = Riviväli: 1½
export-chip-spacing-double = Riviväli: kaksinkertainen
export-chip-notes-included = Muistiinpanot mukana
export-chip-notes-excluded = Muistiinpanot pois
export-format-docx = Word
export-format-odt = LibreOffice
export-format-html = HTML
export-format-markdown = Markdown
export-format-djot = Djot
export-format-text = Teksti
export-format-latex = LaTeX
export-format-epub = EPUB
export-format-pdf = PDF
export-overwrite-title = Korvataanko olemassa oleva tiedosto?
export-overwrite-text = ”{ $name }” on jo olemassa. Korvataanko se?
export-progress-title = Viedään…
export-cancelled = Vienti peruttu
export-done = Vietiin { $count ->
       [one] { $count } kohde
      *[other] { $count } kohdetta
   }
export-open-file = Avaa
export-show-in-folder = Näytä kansiossa
export-error-title = Vienti epäonnistui
export-error-details = Yksityiskohdat

backup-progress-start = Käynnistetään…
backup-progress-retention = Siivotaan vanhoja varmuuskopioita…
backup-progress-done = Valmis
backup-progress-destination = Sijainti { $i }/{ $n }
backup-details = Yksityiskohdat
backup-issues-title = Varmuuskopioinnin ongelmat
backup-failed-title = Varmuuskopiointi epäonnistui
backup-complete-prune-warning = Varmuuskopiointi valmis ({ $ok } tallennettu, { $skipped } jo ajan tasalla). Joitakin vanhoja varmuuskopioita ei voitu poistaa

search = Hae
search-query-placeholder = Hae…
search-replace-placeholder = Korvaa tekstillä…
search-replace-toggle = Näytä tai piilota korvaus
search-replace-all = Korvaa kaikki
search-opt-case = Sama kirjainkoko
search-opt-whole-word = Koko sana
search-opt-diacritics = Huomioi tarkkeet
search-scope-body = Leipäteksti
search-scope-title = Otsikko
search-scope-synopsis = Tiivistelmä
search-scope-label = Nimiö
search-facet-book = Kirjat
search-facet-part = Osat
search-facet-chapter = Luvut
search-facet-scene = Kohtaukset
search-facet-note = Muistiinpanot
search-facet-folder = Kansiot
search-tip-case = Sama kirjainkoko: isot ja pienet kirjaimet katsotaan eri merkeiksi, joten ”Elena” ja ”elena” ovat eri osumia.
search-tip-whole-word = Koko sana: vain kokonaiset sanat täsmäävät, joten ”kissa” ei löydy sanan ”kissanpentu” sisältä.
search-tip-diacritics = Huomioi tarkkeet: tarkkeelliset kirjaimet katsotaan eri merkeiksi, joten ”cafe” ei täsmää sanaan ”café”.
search-tip-body = Leipäteksti: hae kohtausten ja muistiinpanojen tekstistä.
search-tip-title = Otsikko: hae mapin kohteiden otsikoista.
search-tip-synopsis = Tiivistelmä: hae kunkin kirjoituskohteen tiivistelmästä.
search-tip-label = Nimiö: hae lyhyestä merkinnästä, joka näkyy mapin kohteen otsikon alla.
search-tip-comment = Kommentit: hae kommenttiketjujen ja niiden vastausten tekstistä. Kommentti koskee käsikirjoitusta mutta ei ole osa sitä, joten sillä on oma kytkimensä, ja korvattaessa kommenttien osumat jäävät valitsematta, kunnes valitset ne.
search-tip-book = Kirjat: kirjasäiliö ja sen alku- ja loppumerkit.
search-tip-part = Osat: osatason jakajat.
search-tip-chapter = Luvut: luvut, tallentaa projekti ne kummalla tavalla tahansa.
search-tip-scene = Kohtaukset: kohteet, jotka sisältävät tekstisi.
search-tip-note = Muistiinpanot: vapaamuotoiset muistiinpanot.
search-tip-folder = Kansiot: tavalliset järjestelykansiot ja erottimet.
search-tip-paratext = Teksti, joka kuuluu kirjaan mutta ei sen tarinaan: esipuhe, omistuskirjoitus, jälkisanat. Ei koskaan lasketa käsikirjoitukseen.
search-tip-preserve-case = Säilytä kirjainkoko: korvaava teksti saa löydetyn tekstin kirjainkoon, joten ”ELENA” muuttuu muotoon ”MARTA” ja ”Elena” muotoon ”Marta”.
search-tip-replace = Korvaa: näytä korvauskenttä ja Korvaa kaikki.
search-error = Haku epäonnistui: { $message }
search-no-matches = Ei osumia
search-count =
       { $matches ->
           [one] { $matches } osuma
          *[other] { $matches } osumaa
       } { $items ->
           [one] { $items } asiakirjassa
          *[other] { $items } asiakirjassa
       }
search-count-truncated =
       { $matches ->
           [one] { $matches } osuma
          *[other] { $matches } osumaa
       } { $items ->
           [one] { $items } asiakirjassa
          *[other] { $items } asiakirjassa
       } (vain ensimmäiset tulokset)
search-occurrences = { $count }
search-field-body = Leipäteksti
search-field-title = Otsikko
search-field-synopsis = Tiivistelmä
search-field-label = Nimiö
search-field-epigraph = Motto
search-field-comment = Kommentti
search-field-comment-reply = Vastaus
search-field-footnote = Alaviite
search-include-in-replace = Sisällytä toimintoon Korvaa kaikki
search-collapse-all = Supista kaikki tulokset
search-replace-here = Korvaa tämä
search-dismiss = Poista tuloksista
search-undo-dismiss = Palauta viimeinen hylätty tulos
search-replace-nothing = (ei mitään)
search-replace-confirm-title = Korvataanko kaikki osumat?
search-replace-confirm-text =
       Korvataanko ”{ $query }” tekstillä ”{ $replacement }” ({ $occurrences ->
           [one] { $occurrences } esiintymä
          *[other] { $occurrences } esiintymää
       }, { $items ->
           [one] { $items } asiakirja
          *[other] { $items } asiakirjaa
       })? Voit kumota tämän jälkeenpäin ilmoituksesta.
search-replace-done-title = Korvaus valmis
search-replace-done =
       { $occurrences ->
           [one] { $occurrences } esiintymä
          *[other] { $occurrences } esiintymää
       } korvattu { $items ->
           [one] { $items } asiakirjassa
          *[other] { $items } asiakirjassa
       }.
search-replace-done-skipped =
       { $occurrences ->
           [one] { $occurrences } esiintymä
          *[other] { $occurrences } esiintymää
       } korvattu { $items ->
           [one] { $items } asiakirjassa
          *[other] { $items } asiakirjassa
       }. { $skipped ->
           [one] { $skipped } kenttä
          *[other] { $skipped } kenttää
       } ohitettiin (muuttunut haun jälkeen).
search-replace-undo = Kumoa
search-replace-failed-title = Korvaus epäonnistui
search-replace-skipped-title = Mitään ei korvattu
search-replace-skipped-body =
       { $fields ->
           [one] Tämä teksti
          *[other] { $fields } näistä teksteistä
       } muuttui haun jälkeen, joten { $fields ->
           [one] siihen
          *[other] niihin
       } ei koskettu. Hae uudelleen nähdäksesi, missä sanat nyt ovat.
search-replace-undo-failed-title = Kumoaminen epäonnistui
search-preview = Esikatselu
search-preview-empty = Valitse tulos esikatsellaksesi sitä tässä
search-preview-no-prose = Tässä osumassa ei ole muokattavaa tekstiä
search-preview-prompt = Tee haku nähdäksesi esikatselun tässä.
search-preview-open-search = Hae projektista
search-preview-footnote-prompt = Tämä osuma on alaviitteen omassa tekstissä. Avaa se Alaviitteet-paneelissa nähdäksesi ja muokataksesi sitä.
search-preview-open-footnotes = Avaa alaviitteet

find-placeholder = Etsi tästä asiakirjasta…
find-count = { $current }/{ $total }
find-no-results = Ei tuloksia
find-close = Sulje haku
find-previous = Edellinen osuma (Vaihto+Enter)
find-next = Seuraava osuma (Enter)
find-opt-case = Sama kirjainkoko
find-opt-whole-word = Koko sana
find-replace-toggle = Näytä tai piilota korvaus (Ctrl+R)
find-replace-placeholder = Korvaa tekstillä…
find-replace = Korvaa
find-replace-all = Korvaa kaikki
find-preserve-case = Säilytä kirjainkoko

dict-download-title = Ladataan { $name }…
dict-download-done = { $name } asennettu
dict-download-failed = Sanakirjaa { $name } ei voitu ladata: { $error }
dict-removed = { $name } poistettu
dict-accept-first = Hyväksy lisenssi ennen kuin lataat sanakirjan { $name }
settings-dict-tab-installed = Asennettu
settings-dict-tab-get-more = Hanki lisää
settings-dict-tab-personal = Omat sanat
dict-installed-empty = Tästä tietokoneesta ei löytynyt vielä sanakirjoja.
dict-get-more-search = Etsi kieliä
dict-system-badge = järjestelmässäsi
dict-unusable-badge = käyttökelvoton
dict-download-button = Lataa
dict-downloading = Ladataan…
dict-installed-label = Asennettu
dict-remove = Poista
dict-view-license = Näytä lisenssi
dict-approx-size = ~{ $size }
dict-personal-empty = Tässä projektissa ei ole vielä omia sanoja.
dict-personal-add = Lisää
dict-personal-placeholder = Lisää sana…
dict-missing-toast = { $count ->
       [one] { $count } tämän projektin käyttämä sanakirja ei ole asennettuna
      *[other] { $count } tämän projektin käyttämää sanakirjaa ei ole asennettuna
   }
dict-missing-action = Hae sanakirjoja
dict-license-title = Sanakirjan { $name } lisenssi
dict-license-accept = Hyväksy ja lataa
dict-license-cancel = Peruuta
dict-license-close = Sulje
dict-add-button = Lisää sanakirja…
dict-add-title = Lisää sanakirja
dict-add-close = Sulje
dict-add-name = Nimi
dict-add-name-placeholder = esim. Oma latinan sanakirja
dict-add-code = Kielikoodi
dict-add-code-placeholder = esim. la tai fr-FR-x-custom
dict-add-code-hint = Lyhyt, itse valitsemasi tunnus. Tämän valitset asiakirjan kieleksi.
dict-add-aff = Affiksitiedosto (.aff)
dict-add-dic = Sanaluettelo (.dic)
dict-add-submit = Lisää
dict-add-cancel = Peruuta
dict-add-name-required = Anna sanakirjalle nimi.
dict-add-code-required = Syötä kielikoodi.
dict-add-code-invalid = Käytä vain kirjaimia, numeroita ja - _ .
dict-add-code-reserved = Tämä on sisäänrakennetun sanakirjan koodi. Valitse toinen.
dict-add-file-required = Valitse tiedosto.
dict-add-file-missing = Tätä tiedostoa ei ole olemassa.
dict-add-code-taken = Tämän koodin sanakirja on jo asennettu.
dict-add-done = { $name } lisätty
dict-add-unusable = Nämä tiedostot eivät ole käyttökelpoinen sanakirja: { $error }
dict-add-failed = Sanakirjaa ei voitu lisätä: { $error }
inspector-export = Vie
inspector-exportable = Sisällytä vientiin
ctx-number = &Numeroi tämä luku
ctx-unnumber = Älä &numeroi tätä lukua
inspector-numbering = Numerointi
inspector-numbered = Numeroitu
inspector-numbered-tip = Tämä luku saa paikkansa kirjan numeroinnissa. Kytke pois päältä prologia, epilogia tai välinäytöstä varten.
inspector-numbered-tip-more = Numeroimaton luku pysyy kirjassa täsmälleen ennallaan: sen otsikko, teksti ja sanamäärä säilyvät. Sille ei vain tulosteta numeroa, eikä se kuluta numeroa: prologin jälkeinen luku on luku yksi, ei luku kaksi. Luvun jättäminen pois viennistä on eri kytkin, yllä, ja se poistaa luvun kirjasta kokonaan.
inspector-apply-to-children = Käytä alakohteisiin
inspector-milestone = Virstanpylvään päivämäärä
inspector-milestone-none = Ei päivämäärää
inspector-milestone-clear = Tyhjennä
inspector-tags = Tunnisteet
inspector-aliases = Muut nimet
inspector-dict-language = Kieli
inspector-apply-language-to-children = Käytä kieltä alakohteisiin
settings-page-language = Kieli
settings-field-dict-language = Kielet
dict-tradeoff-hint = Jokainen ylimääräinen kieli hyväksyy enemmän sanoja, joten vähemmän virheitä jää kiinni.
lang-inherit-hint = Peritty: tämä kohtaus käyttää kirjan tai projektin kieliä.
lang-pill-list = Kielet
lang-pill-add = Lisää kieli
lang-pill-remove = Poista { $name }
lang-pill-mute = Poista oikoluku käytöstä kielelle { $name }
lang-pill-unmute = Ota oikoluku käyttöön kielelle { $name }

settings-page-personal-dictionary = Oma sanakirja
settings-user-dict-desc = Tähän lisäämäsi sanat katsotaan oikein kirjoitetuiksi kaikkialla sovelluksessa, eikä niitä enää merkitä virheiksi.
settings-user-dict-add-placeholder = Lisää sana…
settings-user-dict-add = Lisää sana
settings-user-dict-search = Suodata sanoja
settings-user-dict-count = { $count ->
       [one] { $count } sana
      *[other] { $count } sanaa
   }
settings-user-dict-rename = Nimeä uudelleen
settings-user-dict-remove = Poista
settings-user-dict-empty = Ei vielä sanoja. Lisää sanoja, jotka oikoluvun pitää ohittaa, tai tuo luettelo.
settings-user-dict-import = Tuo…
settings-user-dict-export = Vie…
settings-user-dict-import-tip = Lisää sanoja tavallisesta tekstitiedostosta, yksi sana riviä kohti. Olemassa olevat sanat säilytetään.
settings-user-dict-export-tip = Tallenna koko luettelo tavalliseen tekstitiedostoon, yksi sana riviä kohti.
settings-user-dict-txt-filter = Tekstitiedostot
settings-user-dict-duplicate = Jo sanakirjassa
settings-user-dict-imported = Tuotiin { $count ->
       [one] { $count } sana
      *[other] { $count } sanaa
   } ({ $duplicates } oli jo mukana).
settings-user-dict-import-failed = Sanaluetteloa ei voitu lukea: { $error }
settings-user-dict-exported = Tallennettiin { $count ->
       [one] { $count } sana.
      *[other] { $count } sanaa.
   }
settings-user-dict-export-failed = Sanaluetteloa ei voitu tallentaa: { $error }

settings-page-text-replacements = Tekstinkorvaukset
settings-text-repl-desc = Korvaa lyhenteet täydellä tekstillä kirjoittaessasi: ”mm.” muuttuu muotoon ”muun muassa” heti, kun kirjoitat välilyönnin tai välimerkin.
settings-text-repl-enable = Käytä tekstinkorvauksia tässä projektissa
settings-text-repl-disabled-hint = Ota tämä käyttöön määrittääksesi lyhenteitä, jotka laajenevat kirjoittaessasi.
settings-text-repl-add = Lisää sääntö
settings-text-repl-trigger-placeholder = Lyhenne
settings-text-repl-replacement-placeholder = Miksi se muuttuu
settings-text-repl-added = Lisätty ”{ $trigger }”
settings-text-repl-duplicate = Lyhenteellä ”{ $trigger }” on jo sääntö
settings-text-repl-filter = Suodata sääntöjä
settings-text-repl-count = { $n ->
       [one] 1 sääntö
      *[other] { $n } sääntöä
   }
settings-text-repl-row-enabled = Käytä tätä sääntöä
settings-text-repl-delete = Poista lyhenteen { $trigger } sääntö
settings-text-repl-deleted = Lyhenteen ”{ $trigger }” sääntö poistettu
settings-text-repl-empty = Ei vielä sääntöjä.
settings-text-repl-csv-filter = CSV-tiedostot
settings-text-repl-import = Tuo…
settings-text-repl-export = Vie…
settings-text-repl-imported = Tuotiin { $added }, ohitettiin { $skipped }
settings-text-repl-exported = Vietiin { $n ->
       [one] 1 sääntö
      *[other] { $n } sääntöä
   }

editor-menu-no-suggestions = Ei ehdotuksia
editor-menu-add-to-dictionary = Lisää ”{ $word }” sanakirjaan
editor-menu-add-words-to-dictionary = Lisää valitut sanat sanakirjaan
editor-dict-added = ”{ $word }” lisätty sanakirjaasi.
editor-dict-added-multi = { $count ->
       [one] { $count } sana lisätty sanakirjaasi.
      *[other] { $count } sanaa lisätty sanakirjaasi.
   }
toast-undo = Kumoa

undo-superseded-title = Tämä vaihe ei ole enää viimeisin
undo-superseded-body = Projektissa on sen jälkeen muuttunut jotain muuta. Kumoaminen peruisi sen sijaan sen, joten sitä ei tehty. Mitään tämän vaiheen tekemää ei ole peruttu. Voit kulkea historiassa taaksepäin itse valikosta Muokkaa ▸ Kumoa.
undo-failed = Kumoaminen epäonnistui: { $error }

menu-edit = &Muokkaa
menu-edit-undo = Ku&moa
menu-edit-redo = Tee &uudelleen
menu-edit-undo-target = Ku&moa: { $target }
menu-edit-redo-target = Tee &uudelleen: { $target }
shortcut-name-edit-undo = Kumoa
shortcut-name-edit-redo = Tee uudelleen
undo-target-typing = kirjoitus
undo-target-project = viimeisin muutos tähän projektiin
prose-history-reset-title = Kirjoitushistoria nollattu
prose-history-reset-body = { $count ->
       [one] Yksi avoin kohtaus palautettiin aiempaan tilaan, joten sen kirjoitushistoria ei enää päde.
      *[other] { $count } avointa kohtausta palautettiin aiempaan tilaan, joten niiden kirjoitushistoria ei enää päde.
   }
undo-target-trash = roskakoriin siirto
undo-target-restore = palautus roskakorista
undo-target-delete-forever = lopullinen poisto
undo-target-replace-all = korvaus koko projektissa
undo-target-import = asiakirjojen tuonti
undo-target-duplicate = monistus
undo-target-move = siirto
undo-target-merge = kahden kohtauksen yhdistäminen
undo-target-split = kohtauksen jakaminen
undo-target-promote = tyypin muutos
undo-target-tidy-titles = lukujen otsikoiden siivous
undo-target-import-tags = tunnisteiden tuonti
undo-target-import-templates = muistiinpanomallien tuonti
undo-target-create = luonti
undo-target-remove = poisto
undo-target-rename = nimen muutos
undo-target-edit = kyseinen muokkaus
menu-edit-find = &Etsi…
menu-edit-find-next = Etsi &seuraava
menu-edit-find-prev = Etsi e&dellinen
menu-edit-replace = Etsi ja k&orvaa…
menu-edit-find-in-project = Etsi &projektista…
menu-edit-replace-in-project = Korvaa p&rojektissa…
undo-frozen = Kumoa (”Aina eteenpäin” on päällä)
redo-frozen = Tee uudelleen (”Aina eteenpäin” on päällä)
menu-edit-cut = &Leikkaa
menu-edit-copy = &Kopioi
menu-edit-paste = L&iitä
menu-edit-paste-plain = Liitä ilm&an muotoilua
menu-edit-select-all = &Valitse kaikki
shortcut-name-edit-cut = Leikkaa
shortcut-name-edit-copy = Kopioi
shortcut-name-edit-paste = Liitä
shortcut-name-edit-paste-plain = Liitä ilman muotoilua
shortcut-name-edit-select-all = Valitse kaikki

titlebar-spellcheck-on = Oikoluku on päällä. Lopeta tarkistus napsauttamalla (F7)
titlebar-spellcheck-off = Oikoluku on pois päältä. Ota tarkistus käyttöön napsauttamalla (F7)
menu-spellcheck = &Oikolue
menu-comments = &Kommentit
settings-page-spellcheck = Oikoluku
settings-group-spellcheck = Oikoluku
settings-spellcheck-enabled = Oikolue kirjoittaessani
settings-spellcheck-hint = Alleviivaa sanat, joita mikään asennettu sanakirja ei tunne. Kun tämä on pois päältä, kaikki tarkistus loppuu kaikissa projekteissa, kunnes otat sen taas käyttöön. Jos haluat lopettaa vain yhden kielen tarkistuksen, poista sen valintamerkki projektin tai kohteen Kieli-kentästä.

menu-trash = &Roskakori
trash-title = Roskakori
trash-empty-state = Roskakori on tyhjä.
trash-empty-button = Tyhjennä roskakori…
trash-restore = &Palauta
trash-restore-to = Palauta &kohteeseen…
trash-delete-forever = P&oista lopullisesti
trash-restored-ok = { $count } palautettu.
trash-restore-error = Palautus epäonnistui: { $error }
trash-restore-orphaned = Tämän kohteen alkuperäistä paikkaa ei enää ole. Valitse, mihin se palautetaan.
trash-restore-no-project = Projektia ei ole auki, joten palautettavaa ei ole.
trash-delete-no-project = Projektia ei ole auki, joten poistettavaa ei ole.
trash-empty-confirm-title = Tyhjennetäänkö roskakori?
trash-empty-confirm-text = { $count ->
       [one] { $count } roskakorissa oleva kohde poistetaan pysyvästi.
      *[other] { $count } roskakorissa olevaa kohdetta poistetaan pysyvästi.
   } Ne poistuvat roskakorista lopullisesti. Kumoa voi vielä palauttaa ne, kunnes teet jotain muuta.
trash-emptied-title = Roskakori tyhjennetty
trash-emptied-body = Kaikki roskakorissa ollut poistettiin pysyvästi.
trash-empty-no-project = Projektia ei ole auki, joten tyhjennettävää roskakoria ei ole.
trash-delete-forever-confirm-title = Poistetaanko lopullisesti?
trash-delete-forever-confirm-text = { $count ->
       [one] { $count } kohde poistetaan pysyvästi.
      *[other] { $count } kohdetta poistetaan pysyvästi.
   } Ne poistuvat roskakorista lopullisesti. Kumoa voi vielä palauttaa ne, kunnes teet jotain muuta.
trash-deleted-title = Poistettu lopullisesti
trash-deleted-body = { $count } poistettu pysyvästi.
trash-undo = Kumoa
trash-restore-picker-title = Palauta kohteeseen…
trash-restore-to-confirm-title = Palautetaanko tähän?
trash-restore-to-confirm-text = Palautetaanko ”{ $item }” kohteeseen ”{ $destination }”?
trash-restore-picker-restore-here = Palauta tähän
trash-restore-picker-cancel = Peruuta
trash-restore-picker-empty = Mappeja ei ole vielä. Luo ensin mappi.
trash-banner-title = Tämä kohde on roskakorissa
trash-banner-description = Se ei näy jäsennyksessä eikä vienneissä, ennen kuin palautat sen.
trash-banner-restore = Palauta…
trash-tab-tooltip = Roskakorissa

settings-styles-sheet-yes = Kyllä
settings-styles-sheet-no = Ei
settings-styles-sheet-font = Kirjasin
settings-styles-sheet-indent = Ensimmäisen rivin sisennys
settings-styles-sheet-para-spacing = Kappaleväli
settings-styles-sheet-page = Sivun koko
settings-styles-sheet-margins = Marginaalit (ylä/oikea/ala/vasen)
settings-styles-sheet-title-page = Kirjan otsikkosivu
settings-styles-sheet-heading-language = Otsikoiden kieli
settings-styles-sheet-auto = Tekstin mukaan
settings-styles-sheet-digits = Numerot
settings-styles-sheet-direction = Tekstin suunta
settings-styles-sheet-formats = Muodot
settings-styles-sheet-all-formats = Kaikki

settings-styles-editor-group = Tyylimuokkain
settings-styles-editor-missing = Tämä tyyli ei ole enää käytettävissä.

settings-styles-page-letter = Letter
settings-styles-digits-western = Länsimaiset (0–9)
settings-styles-digits-eastern-arabic = Itäarabialaiset (٠–٩)
settings-styles-direction-ltr = Vasemmalta oikealle
settings-styles-direction-rtl = Oikealta vasemmalle

format-dock-title = Muotoilu
format-panel-empty = Vie kohdistin kohtaukseen, muistiinpanoon tai tiivistelmään nähdäksesi muotoiluvaihtoehdot.
format-group-marks = Teksti
format-group-block = Kappale
format-group-lists = Luettelot
format-group-tables = Taulukko
format-group-breaks = Kohtauskatkot
format-superscript = Yläindeksi
format-subscript = Alaindeksi
format-link = Linkki…
link-dialog-insert-title = Lisää linkki
link-dialog-edit-title = Muokkaa linkkiä
link-dialog-text-label = Teksti
link-dialog-text-placeholder = sanat, jotka lukija näkee
link-dialog-url-label = Linkin kohde
link-dialog-url-placeholder = example.com
link-dialog-insert = Lisää
link-dialog-apply = Käytä
link-dialog-cancel = Peruuta
link-dialog-remove = Poista linkki
link-scheme-refused = Vain verkko- ja sähköpostilinkkejä voi avata. Linkkiin { $url } ei koskettu.
menu-format-link = &Linkki…
format-clear = Tyhjennä muotoilu
format-heading = Otsikkotaso
format-heading-normal = Tavallinen teksti
format-heading-1 = Otsikko 1
format-heading-2 = Otsikko 2
format-heading-3 = Otsikko 3
format-heading-4 = Otsikko 4
format-heading-5 = Otsikko 5
format-heading-6 = Otsikko 6
format-align-left = Tasaa vasemmalle
format-align-center = Keskitä
format-direction-rtl = Oikealta vasemmalle -kappale
format-blockquote = Lainaus
format-list-bullet = Luettelomerkkiluettelo
format-list-numbered = Numeroitu luettelo
format-indent = Sisennä
format-outdent = Vähennä sisennystä
format-table-insert = Lisää taulukko
format-table-row-above = Lisää rivi yläpuolelle
format-table-row-below = Lisää rivi alapuolelle
format-table-col-before = Lisää sarake eteen
format-table-col-after = Lisää sarake perään
format-table-row-delete = Poista rivi
format-table-col-delete = Poista sarake
format-table-remove = Poista taulukko

menu-format-marks-bold = Li&havointi
menu-format-marks-italic = &Kursivointi
menu-format-marks-underline = &Alleviivaus
menu-format-marks-strike = &Yliviivaus
menu-format-marks-superscript = Yl&äindeksi
menu-format-marks-subscript = Ala&indeksi
menu-format-marks-clear = Tyh&jennä muotoilu
menu-format-heading = &Otsikkotaso
menu-format-heading-normal = &Tavallinen teksti
menu-format-heading-1 = Otsikko &1
menu-format-heading-2 = Otsikko &2
menu-format-heading-3 = Otsikko &3
menu-format-heading-4 = Otsikko &4
menu-format-heading-5 = Otsikko &5
menu-format-heading-6 = Otsikko &6
menu-format-alignment = Ta&saus
menu-format-align-left = Tasaa &vasemmalle
menu-format-align-center = &Keskitä
menu-format-direction = Ka&ppaleen suunta
menu-format-direction-auto = &Automaattinen
menu-format-direction-ltr = &Vasemmalta oikealle
menu-format-direction-rtl = &Oikealta vasemmalle
menu-format-blockquote = Lai&naus
menu-format-lists = Lu&ettelot
menu-format-list-bullet = &Luettelomerkit
menu-format-list-numbered = &Numeroitu luettelo
menu-format-indent = &Sisennä
menu-format-outdent = &Vähennä sisennystä
menu-format-table = Ta&ulukko
menu-format-table-insert = &Lisää taulukko
menu-format-table-2x2 = &2 x 2
menu-format-table-3x3 = &3 x 3
menu-format-table-4x4 = &4 x 4
menu-format-table-row-above = Lisää rivi &yläpuolelle
menu-format-table-row-below = Lisää rivi &alapuolelle
menu-format-table-col-before = Lisää sarake &eteen
menu-format-table-col-after = Lisää sarake &perään
menu-format-table-row-delete = Poista &rivi
menu-format-table-col-delete = Poista &sarake
menu-format-table-remove = Poista &taulukko

menu-format-typo-marks = &Muotoilumerkki
menu-format-typo-nbsp = &Sitova välilyönti
menu-format-typo-nbhyphen = Sitova &tavuviiva
menu-format-typo-soft-hyphen = Tavutus&vihje
menu-format-typo-nnbsp = &Kapea sitova välilyönti
menu-format-typo-zwsp = &Leveydetön välilyönti
menu-format-typo-word-joiner = Sanali&itin


menu-about = &Tietoja Skribistosta…
about-title = Tietoja Skribistosta
about-version = Versio { $version }
about-update-available = Versio { $version } on saatavilla
about-update-see-notes = Katso muutokset
about-update-flathub = Flathub pitää tämän kopion ajan tasalla.
about-update-distro = Jakelusi pitää tämän kopion ajan tasalla.
about-tagline = Romaanien kirjoitussovellus pitkälle fiktiolle, kirjoitettu Rustilla Teksilo-työkalupakin avulla.
about-license = Julkaistu GNU General Public License -lisenssin version 3 ehdoin.
about-copyright = © 2026 Cyril Jacquet
about-close = Sulje

native-menu-about = Tietoja: { $app }
native-menu-hide = Kätke { $app }
native-menu-quit = Lopeta { $app }
native-menu-settings = Asetukset…
native-menu-window = Ikkuna
native-menu-minimize = Pienennä
native-menu-zoom = Zoomaa

window-title = { $title } - { $app }
window-title-numbered = { $title } - { $app } (Ikkuna { $n })
window-title-empty = { $app }

first-run-window-title = Tervetuloa
first-run-title = Ota { $app } käyttöön
first-run-body = { $app } säilyttää asetuksensa erillään Skribistosta, joten se aloittaa tyhjänä. Asetuksesi, viimeisimmät projektit, sanakirjat ja ikkunoiden asettelu voidaan kopioida nyt.
first-run-from = Kopioi lähteestä
first-run-to = Kopioi kohteeseen
first-run-copy-note = Mitään ei siirretä eikä poisteta. Skribisto säilyttää kaikki omat asetuksensa ja toimii täsmälleen kuten ennenkin.
first-run-import = Tuo asetukset
first-run-start-fresh = Aloita alusta
first-run-import-failed = Joitakin asetuksia ei voitu tuoda: { $error }

settings-page-punctuation = Välimerkkien oletukset
settings-group-punctuation = Älykäs välimerkitys
settings-page-work-punctuation = Välimerkit
settings-punctuation-override = Anna tälle projektille omat välimerkkisäännöt
settings-punctuation-override-hint = Pois: projekti noudattaa sovelluksen asetusta. Nämä säännöt kulkevat .skrib-tiedoston mukana, joten tiedoston avaava kanssakirjoittaja kirjoittaa samalla typografialla.
settings-punctuation-dashes = Muuta -- ajatusviivaksi (–) ja --- pitkäksi ajatusviivaksi (—)
settings-punctuation-ellipsis = Muuta ... ellipsimerkiksi (…)
settings-punctuation-quotes = Käytä kaarevia lainausmerkkejä ja heittomerkkejä
settings-quote-style = Lainausmerkit
settings-quote-style-locale = Kielen oletus
settings-quote-style-curly = “Kaksinkertaiset”
settings-quote-style-curly-single = ‘Yksinkertaiset’
settings-quote-style-guillemets = «Kulmalainausmerkit»
settings-quote-style-low-high = „Ala-ylä“
settings-punctuation-spacing = Väli ennen merkkejä ; : ! ?
settings-punctuation-spacing-hint = Ranskalaisessa typografiassa merkkien ; ! ? eteen tulee kapea sitova välilyönti ja kaksoispisteen eteen tavallinen. Asetus koskee vain ranskaksi kirjoitettua tekstiä. Kulmalainausmerkkien « » sisäpuolinen väli tulee itse lainausmerkkityylin mukana.
settings-punctuation-sample = Kielesi tuottaa
settings-punctuation-app-hint = Mitä jokainen projekti tekee, ellei sillä ole omia sääntöjä kohdassa Projekti ▸ Välimerkit.
settings-punctuation-dialogue = Aloita kappale, joka kirjoitetaan muodossa ”- ”, repliikkiviivalla
settings-punctuation-dialogue-hint = Kielille, jotka merkitsevät puheen viivalla eivätkä lainausmerkeillä: ranska, espanja, venäjä ja muut. Toimii vain aivan kappaleen alussa.

statusbar-go-to = Siirry kohteeseen…
go-to-placeholder = Hae mapista
go-to-no-matches = Mikään kohde ei vastaa hakua.
menu-go-to = &Siirry kohteeseen…

settings-page-distraction-free-themes = Häiriöttömän tilan teemat
settings-themes-builtin = Sisäänrakennetut teemat
settings-themes-builtin-badge = Sisäänrakennettu
settings-themes-user = Omat teemat
settings-themes-editor-group = Muokkaa teemaa
settings-themes-editor-empty = Valitse teema kohdasta ”Omat teemat” muokataksesi sitä.
settings-themes-use = Ota käyttöön
settings-themes-duplicate = Monista
settings-themes-edit = Muokkaa
settings-themes-delete = Poista
settings-themes-export = Vie…
settings-themes-import = Tuo…
settings-themes-copy-suffix = kopio
settings-themes-json-filter = Teema (JSON)
settings-themes-imported = Teema tuotu
settings-themes-import-failed = Tätä teemaa ei voitu tuoda
settings-themes-exported = Teema viety
settings-themes-export-failed = Tätä teemaa ei voitu viedä
settings-themes-low-contrast = heikko kontrasti
settings-themes-low-contrast-band = korostus peittää tekstin
settings-themes-field-name = Nimi
settings-themes-field-paper = Paperi
settings-themes-field-ink = Teksti
settings-themes-field-general = Tausta
settings-themes-field-widget-text = Ohjauspalkin teksti
settings-themes-field-caret-band = Korostus kohdistimen ympärillä

statusbar-focus-settings = Häiriöttömän tilan asetukset
statusbar-focus-manage-themes = Hallitse teemoja…

comments-title = Kommentit
comments-document-title = Tämä asiakirja
comments-empty-project = Tässä projektissa ei ole vielä kommentteja.
comments-empty-document = Tässä asiakirjassa ei ole kommentteja.
comments-filter-all = Kaikki
comments-filter-open = Avoimet
comments-filter-resolved = Ratkaistut
comments-filter-orphaned = Orvot
comments-status-open = Avoin
comments-status-resolved = Ratkaistu
comments-status-orphaned = Teksti kadonnut
comments-orphan-snippet = (kommentoitu teksti on poissa)
comments-status-unplaced = Ei sijaintia tekstissä
comments-unplaced-snippet = (ei ankkuroitu mihinkään tekstiin)
comments-reply-count = { $count ->
       [one] { $count } vastaus
      *[other] { $count } vastausta
   }
comments-menu-resolve = Ratkaise
comments-menu-reopen = Avaa uudelleen
comments-menu-delete = Poista
comments-sort-document = Asiakirjan järjestyksessä
comments-sort-newest = Uusimmat ensin
comments-sort-reader = Lukijoittain ryhmiteltynä
comments-unsigned-author = Allekirjoittamaton
comments-agreement = { $count ->
    [one] { $count } lukija
   *[other] { $count } lukijaa
}
comments-menu-add = Lisää kommentti
comments-menu-add-paragraph = Kommentoi tätä kappaletta
overview-col-comments = Kommentit
overview-col-total-comments = Kommentteja yhteensä
comments-card-placeholder = Kirjoita kommentti…
comments-card-unknown-author = Tuntematon tekijä
comments-unsigned-toast = Kommenttisi ovat allekirjoittamattomia: tällä tietokoneella ei ole asetettu nimeäsi.
comments-unsigned-action = Aseta nimesi
comments-card-reply = Vastaa
comments-card-reply-placeholder = Vastaa…
comments-card-actions = Kommentin toiminnot
comments-reply-actions = Vastauksen toiminnot
comments-menu-delete-reply = Poista vastaus
comments-menu-delete-all = Poista kaikki kommentit tästä
comments-deleted-toast = Kommentti poistettu
comments-reply-deleted-toast = Vastaus poistettu
comments-deleted-all-toast = { $count ->
       [one] { $count } kommentti poistettu
      *[other] { $count } kommenttia poistettu
   }
comments-undo = Kumoa

analysis-segment = Analyysi
analysis-scope-book = Analysoidaan tätä kirjaa
analysis-run = Suorita analyysi
analysis-stale = Muuttunut analyysin jälkeen
analysis-not-run = Ei vielä analysoitu.
analysis-running = Luetaan käsikirjoitusta…
analysis-failed = Analyysia ei voitu viedä loppuun.
analysis-no-scenes = Tässä kirjassa ei ole vielä kohtauksia.

analysis-shape = Muoto

analysis-arrivals = Tekstin alkuperä
analysis-arrivals-explainer = Miten teksti on tullut tähän projektiin sen ollessa auki. Se kertoo, mitä reittiä merkit tulivat, eikä mitään siitä, kuka ne kirjoitti: muualla luonnosteleva ja liittävä kirjoittaja on liittänyt, ja saneleva on sanellut. Tässä ei ole mitään lukua tavoiteltavaksi.
analysis-arrivals-scope = Koko projekti, ei vain tämä kirja.
analysis-arrivals-session = Avaamisesta lähtien. Projektin sulkeminen aloittaa laskennan alusta.
analysis-arrivals-nothing = Tässä istunnossa ei ole vielä tullut tekstiä.
analysis-arrivals-typed = Kirjoitettu
analysis-arrivals-pasted = Liitetty
analysis-arrivals-dictated = Saneltu
analysis-arrivals-imported = Tuotu
analysis-arrivals-programmatic = Lisätty puolestasi
analysis-arrivals-count = { $count ->
    [one] { $count } merkki
   *[other] { $count } merkkiä
}
analysis-arrivals-none = ei yhtään

analysis-words-per-scene = Sanaa kohtausta kohden
analysis-median-words = Tämän kirjan mediaanikohtauksen pituus on { $count ->
       [one] { $count } sana.
      *[other] { $count } sanaa.
   }
analysis-median-line = Mediaani: { $count ->
       [one] { $count } sana
      *[other] { $count } sanaa
   }
analysis-dialogue = Vuoropuhelu
analysis-dialogue-unsupported = Vuoropuhelua ei vielä mitata tällä kielellä.

analysis-footnote-words = Sanat alaviitteissä
analysis-footnote-words-count = { $count ->
       [one] Tämän kirjan alaviitteissä on { $count } sana. Se pidetään erillään käsikirjoituksen kokonaismäärästä.
      *[other] Tämän kirjan alaviitteissä on { $count } sanaa. Ne pidetään erillään käsikirjoituksen kokonaismäärästä.
   }
analysis-footnote-words-pending = Lasketaan alaviitteitä…

analysis-ignore-empty = Ohita tekstit, joihin ei ole vielä kirjoitettu mitään
analysis-empty-hidden = { $count ->
       [one] { $count } tyhjä teksti piilotettu.
      *[other] { $count } tyhjää tekstiä piilotettu.
   }
analysis-all-texts-empty = Kaikki tämän kirjan tekstit ovat yhä tyhjiä.

binder-filter-count = { $shown }/{ $total } näytetään
binder-filter-clear = Tyhjennä
binder-filter-none = Mikään ei vastaa hakua ”{ $query }”.

settings-paratext-intro = Alku- ja loppusivut, joilla uusi projekti voi alkaa. Kukin rakenne kuuluu johonkin julkaisuperinteeseen, ja sen sivujen otsikot on kirjoitettu kyseisen perinteen omalla kielellä. Voit nimetä ne vapaasti uudelleen, kun projekti on luotu.
settings-paratext-structures = Rakenteet
settings-paratext-broken = Ei voitu lukea
settings-paratext-edit = Muokkaa
settings-paratext-duplicate = Monista
settings-paratext-delete = Poista
settings-paratext-new = Uusi rakenne
settings-paratext-save = Tallenna
settings-paratext-editor-hint = Nimi, ennen käsikirjoitusta luotavat sivut ja sen jälkeen luotavat sivut. Niiden lopullinen paikka on sinun päätettävissäsi: ne ovat tavallisia kohteita, kun projekti on olemassa.


image-insert = Lisää &kuva…
image-no-project = Avaa projekti ennen kuvan lisäämistä.
image-choose-title = Valitse kuva
image-filter-label = Kuvat
image-large-title = Tämä on suuri kuva
image-large-text =
       { $name } on { $megapixels } megapikseliä ({ $width }×{ $height }).
       Jos säilytät sen sellaisenaan, alkuperäinen tiedosto tallennetaan projektiin: se kulkee
       jokaisen varmuuskopion ja jokaisen viennin mukana. Optimointi tallentaa sen sijaan
       pienemmän kopion, jonka pisin sivu on enintään 2560 pikseliä.
image-large-keep = Säilytä alkuperäinen
image-large-downscale = Optimoi
image-large-remember = Tee näin jatkossa, älä kysy uudelleen
image-not-recorded = Kuva tallennettiin, mutta sitä ei voitu kirjata projektiin.
image-describe-title = Kuvaile kuvaa
image-describe-explain = Mitä kuvassa on, lukijalle, joka ei näe sitä. Se ei ole osa käsikirjoitusta: sitä ei koskaan lasketa, haeta eikä viedä tekstinä.
image-describe-placeholder = majakka harmaata taivasta vasten
image-resize-title = Muuta kuvan kokoa
image-resize-explain = Prosenttiosuus nykyisestä näyttökoosta. 100 jättää sen ennalleen.
image-resize-invalid = Anna luku väliltä 1–1000.
image-menu-describe = &Kuvaile kuvaa…
image-menu-resize = &Muuta kuvan kokoa…
image-menu-reset-size = &Alkuperäinen koko

cover-choose = Kir&jan kansi…
cover-clear = &Poista kansi
cover-choose-title = Valitse kansikuva
cover-set = Kansi on asetettu.
cover-cleared = Kansi on poistettu. Kuva on yhä projektissa.
card-label = Nimiö
card-exportable = Mukana viennissä
card-numbered = Numeroitu
card-created = Luotu
card-modified = Muokattu
card-yes = kyllä
card-no = ei
card-type = Tyyppi
card-position = Järjestys
card-children = Alakohteet
card-goal = Tavoite
card-words = Sanat
card-synopsis = Tiivistelmä
card-point-of-view = Näkökulma
card-aliases = Muut nimet

export-orphan-footnotes-title = Alaviitteet ilman viitettä
export-comments-dropped =
       { $count ->
           [one] Yhtä kommenttia ei voitu sijoittaa vietyyn tekstiin, joten se jätettiin pois.
          *[other] { $count } kommenttia ei voitu sijoittaa vietyyn tekstiin, joten ne jätettiin pois.
       }
export-orphan-footnotes =
       { $count ->
           [one] Yhteen alaviitteeseen ei enää viitata missään käsikirjoituksessa. Sen teksti ei näy viedyssä kirjassa.
          *[other] { $count } alaviitteeseen ei enää viitata missään käsikirjoituksessa. Niiden teksti ei näy viedyssä kirjassa.
       }


menu-footnotes = &Alaviitteet
footnotes-title = Alaviitteet
footnotes-insert = Lisää &alaviite
footnotes-empty = Ei vielä alaviitteitä. Vie kohdistin kohtaukseen ja käytä sitten yllä olevaa +-painiketta tai pikanäppäintä Ctrl+Alt+F.
footnotes-filter-all = Kaikki
footnotes-filter-document = Tämä asiakirja
footnotes-filter-orphaned = Orvot
footnotes-orphaned = Mikään ei enää viittaa tähän alaviitteeseen
footnotes-untitled-home = Nimetön
footnotes-body-placeholder = itse alaviite
footnotes-insert-tooltip = Lisää alaviite kohdistimen kohtaan (Ctrl+Alt+F)
footnotes-actions = Alaviitteen toiminnot
footnotes-delete = &Poista alaviite ja sen viittaus
footnotes-no-project = Avaa projekti ennen alaviitteen lisäämistä.
footnotes-no-caret = Vie kohdistin kohtauksen tekstiin lisätäksesi alaviitteen siihen.
footnotes-not-created = Alaviitettä ei voitu lisätä projektiin.
footnotes-deleted-toast = Alaviite poistettu
footnotes-undo-delete = Kumoa

import-document-title = Tuo asiakirjoja
import-document-close = Sulje
import-document-step-files = Tiedostot
import-document-step-review = Tarkistus
import-document-step-destination = Määränpää
import-document-step-reconcile = Yhdistä
import-document-reconcile-hint = Osa näistä kohteista on sinulla jo. Kerro, mitä kullekin tehdään.
import-document-reconcile-all-new = Mikään tässä tiedostossa ei vastaa projektiasi: jokainen kohde lisätään uudeksi.
import-document-duplicate-returns = Nämä tiedostot ovat useampi kopio samasta takaisin palaavasta käsikirjoituksesta. Tuo ne yksi kerrallaan: täsmäytys kohdistaa projektiisi vain yhden palaavan kopion, joten usean tuominen kerralla lisäisi kirjasta toisen kopion yhdistämisen sijaan.
import-document-hunk-added = Ota tämä uusi kappale
import-document-hunk-removed = Poista tämä kappale
import-document-hunk-changed = Ota tämä muutettu versio
import-document-col-stray-prose = Sen teksti
import-document-stray-as-paratext = Säilytä paratekstina
import-document-stray-discard = Hylkää teksti
import-document-col-current = Projektissasi
import-document-col-incoming = Tässä tiedostossa
import-document-col-status = Tila
import-document-col-action = Toimenpide
import-document-status-identical = Sama
import-document-status-editor-edited = Toinen muokkasi
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
import-document-compare-legend = Oma versiosi verrattuna tämän tiedoston versioon. Vain luku.
import-document-compare-close = Sulje
import-document-drop-title = Pudota asiakirjat tähän
import-document-drop-hint = Markdown (.md), Word (.docx), OpenDocument (.odt) ja pelkkä teksti (.txt)
import-document-browse = Selaa…
import-document-move-up = Siirrä ylös
import-document-move-down = Siirrä alas
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
import-document-level-rules = Otsikkotasot
import-document-level-n = Otsikko { $level }
import-document-add-top-level = Lisää ylin taso
import-document-add-top-level-tooltip = Lisää kirja kaikkien analysoitujen kohteiden yläpuolelle lukutiedostoja varten, joissa ei ole kirjan otsikkoa
import-document-destination = Määränpää
import-document-destination-hint = Valitse mappi tai kohde: uudet kohteet sijoitetaan kansion sisään tai kohtauksen perään.
import-document-destination-empty = Mappeja ei ole vielä. Luo ensin mappi.
import-document-plan-empty = Ei ole vielä mitään tuotavaa.
import-document-summary = { $rows ->
       [one] 1 kohde
      *[other] { $rows } kohdetta
   } · { $breaks ->
       [one] 1 kohtauskatko
      *[other] { $breaks } kohtauskatkoa
   }
import-document-back = Takaisin
import-document-cancel = Peruuta
import-document-analyse = Seuraava
import-document-analysing = Luetaan asiakirjoja…
import-document-step-analysing = Luetaan
import-document-cancel-analysis = Lopeta lukeminen
import-document-analyse-failed = Asiakirjoja ei voitu lukea.
import-document-details = Yksityiskohdat
import-document-import = Tuo
import-document-done = { $count ->
       [one] 1 kohde tuotiin
      *[other] { $count } kohdetta tuotiin
   }
import-document-undo = Kumoa
import-diagnostic-file-unreadable = Tiedostoa ”{ $path }” ei voitu lukea: { $detail }. Muut tiedostot tuodaan silti.
import-diagnostic-nested-too-deep = Tiedostoa ”{ $path }” ei luettu: sen sisältö on sisäkkäistetty yli { $limit } tason syvyyteen. Mikään todellinen asiakirja ei ole lähelläkään sitä, ja sen lukeminen voisi kaataa Skribiston, joten se jätetään pois. Muut tiedostot tuodaan silti.
import-diagnostic-archive-too-large = Tiedostoa ”{ $path }” ei luettu: se on pakattu tiedosto, joka on tehty purkautumaan moninkertaisesti omaa kokoaan suuremmaksi, mikä voisi kuluttaa muistin loppuun ja kaataa Skribiston, joten se jätetään pois. Muut tiedostot tuodaan silti.
import-diagnostic-lossy-decode = { $count ->
       [one] Yhtä merkkiä tiedostossa ”{ $path }” ei voitu tulkita. Tallenna tiedosto uudelleen UTF-8-muodossa säilyttääksesi sen.
      *[other] { $count } merkkiä tiedostossa ”{ $path }” ei voitu tulkita. Tallenna tiedosto uudelleen UTF-8-muodossa säilyttääksesi ne.
   }
import-diagnostic-decoded-from-bom = Tiedosto ”{ $path }” tulkittiin merkistönä { $detail }, ei UTF-8:na.
import-diagnostic-empty-file = Tiedosto ”{ $path }” on tyhjä.
import-diagnostic-no-headings = Tiedostossa ”{ $path }” ei ole otsikoita, joten se tuodaan yhtenä kohteena.
import-diagnostic-unsupported-format = Mikään ei lue ”.{ $detail }”-tiedostoja, joten ”{ $path }” ohitettiin.
import-diagnostic-front-matter-not-flat = Tiedoston ”{ $path }” alkutiedot: ”{ $detail }” ei ole yksinkertainen arvo, ja se ohitettiin.
import-diagnostic-footnotes-degraded = { $count ->
       [one] Yksi alaviite tiedostossa ”{ $path }” tuodaan tavallisena tekstinä: Markdownista ei lueta alaviitteitä.
      *[other] { $count } alaviitettä tiedostossa ”{ $path }” tuodaan tavallisena tekstinä: Markdownista ei lueta alaviitteitä.
   }
import-diagnostic-footnote-not-carried = { $count ->
       [one] Yhtä alaviitettä tiedostossa ”{ $path }” ei voitu tuoda. Muut tuotiin.
      *[other] { $count } alaviitettä tiedostossa ”{ $path }” ei voitu tuoda. Muut tuotiin.
   }
import-diagnostic-raw-html-dropped = { $count ->
       [one] Yksi raaka HTML-lohko tiedostossa ”{ $path }” jätettiin pois.
      *[other] { $count } raakaa HTML-lohkoa tiedostossa ”{ $path }” jätettiin pois.
   }
import-diagnostic-nested-break-dropped = { $count ->
       [one] Yksi lainauksen tai luettelon sisällä ollut kohtauskatko tiedostossa ”{ $path }” jätettiin pois. Vain omalla rivillään oleva katko säilytetään.
      *[other] { $count } lainausten tai luetteloiden sisällä ollutta kohtauskatkoa tiedostossa ”{ $path }” jätettiin pois. Vain omalla rivillään oleva katko säilytetään.
   }
import-diagnostic-duplicate-title = ”{ $title }” esiintyy { $count } kertaa. Jos olet tuonut nämä tiedostot aiemmin, ne monistuvat.
import-diagnostic-heading-level-jump = ”{ $title }” hyppää otsikkotasolta { $from } tasolle { $to }; se sijoitetaan yhden tason ylätasonsa alapuolelle.
import-diagnostic-illegal-combination = ”{ $title }” sisältää tekstiä, mutta tyyppi { $kind } ei voi sisältää tekstiä. Tuonti odottaa, kunnes muutat sen tyyppiä tai jätät sen pois. Muuten mitään ei tuoda.
import-diagnostic-tracked-changes-flattened = { $path } oli kesken muokkauskierroksen: { $count ->
       [one] { $count } jäljitetty muutos hyväksyttiin
      *[other] { $count } jäljitettyä muutosta hyväksyttiin
   } ja poistot toteutettiin. Tämä on lopullinen teksti. Jos haluat säilyttää omia sanamuotojasi, käytä viimeisen vaiheen Vertaa-toimintoa.
import-diagnostic-tracked-changes-flattened-by = { $path } oli kesken muokkauskierroksen: { $count ->
       [one] { $count } jäljitetty muutos (tekijä: { $names }) hyväksyttiin
      *[other] { $count } jäljitettyä muutosta (tekijä: { $names }) hyväksyttiin
   } ja poistot toteutettiin. Tämä on lopullinen teksti. Jos haluat säilyttää omia sanamuotojasi, käytä viimeisen vaiheen Vertaa-toimintoa.
import-diagnostic-text-box-dropped = { $count ->
       [one] Tiedostossa { $path } on { $count } tekstiruutu. Sen teksti on asiakirjan tekstivirran ulkopuolella, joten sen paikkaa käsikirjoituksessa ei voi määrittää. Sitä ei tuoda.
      *[other] Tiedostossa { $path } on { $count } tekstiruutua. Niiden teksti on asiakirjan tekstivirran ulkopuolella, joten niiden paikkaa käsikirjoituksessa ei voi määrittää. Niitä ei tuoda.
   }
import-diagnostic-embedded-object-dropped = { $count ->
       [one] Tiedostossa { $path } on { $count } upotettu objekti: kaavio, yhtälö tai vastaava. Käsikirjoituksessa ei ole mitään, mihin sen voisi sijoittaa.
      *[other] Tiedostossa { $path } on { $count } upotettua objektia: kaavioita, yhtälöitä tai vastaavia. Käsikirjoituksessa ei ole mitään, mihin ne voisi sijoittaa.
   }
import-diagnostic-field-flattened = { $count ->
       [one] Tiedostossa { $path } on { $count } kenttä: sivunumero, ristiviittaus, päivämäärä. Se säilyttää viimeksi näyttämänsä tekstin eikä enää päivity.
      *[other] Tiedostossa { $path } on { $count } kenttää: sivunumeroita, ristiviittauksia, päivämääriä. Kukin säilyttää viimeksi näyttämänsä tekstin eikä enää päivity.
   }
import-diagnostic-unknown-style-level = { $path } käyttää tyyliä ”{ $detail }”, joka näyttää otsikolta mutta ei kerro tasoa. Nämä kappaleet tuodaan tekstinä sen sijaan, että niiden tasoa arvattaisiin.
import-diagnostic-comment-unanchored = Kommenttia ”{ $detail }” tiedostossa { $path } ei voitu liittää sanoihin, joita se koski. Se säilytetään kohteessaan, josta voit siirtää sen.
import-diagnostic-comment-not-carried = Kommenttia ”{ $detail }” tiedostossa { $path } ei tuoda, koska tiedostosta ei tule tekstiä, johon sen voisi liittää. Kopioi se tiedostosta, jos tarvitset sitä.
import-diagnostic-comment-replies-flattened = { $count ->
       [one] { $count } vastaus tiedostossa { $path } viittasi kommenttiin, jota tiedostossa ei ole, joten se tuodaan omana kommenttinaan.
      *[other] { $count } vastausta tiedostossa { $path } viittasi kommenttiin, jota tiedostossa ei ole, joten ne tuodaan omina kommentteinaan.
   }
import-diagnostic-prose-not-verbatim = { $count ->
       [one] { $count } kappaletta tiedostossa { $path } ei voitu tuoda täsmälleen alkuperäisellä muotoilullaan, joten se tuodaan tavallisena tekstinä. Vertaa sitä alkuperäiseen.
      *[other] { $count } kappaletta tiedostossa { $path } ei voitu tuoda täsmälleen alkuperäisellä muotoilullaan, joten ne tuodaan tavallisena tekstinä. Vertaa niitä alkuperäiseen.
   }
import-diagnostic-styled-spaces-not-carried = { $count ->
       [one] Tiedostossa { $path } { $count } tyhjä välijakso oli alleviivattu tai yliviivattu, esimerkiksi täytettäväksi jätetty viiva. Skribisto ei säilytä kumpaakaan viivaa pelkissä välilyönneissä, joten se tuodaan tavallisina välilyönteinä, tai ei lainkaan kappaleen alussa tai lopussa.
      *[other] Tiedostossa { $path } { $count } tyhjää välijaksoa oli alleviivattu tai yliviivattu, esimerkiksi täytettäviksi jätettyjä viivoja. Skribisto ei säilytä kumpaakaan viivaa pelkissä välilyönneissä, joten ne tuodaan tavallisina välilyönteinä, tai ei lainkaan kappaleen alussa tai lopussa.
   }
import-diagnostic-list-nesting-flattened = { $count ->
       [one] { $count } luettelokohta tiedostossa { $path } oli sisäkkäistetty yli { $limit } tason syvyyteen. Se tuodaan tasolle { $limit }, syvimmälle, jonka Skribisto säilyttää, sanoineen ja muotoiluineen.
      *[other] { $count } luettelokohtaa tiedostossa { $path } oli sisäkkäistetty yli { $limit } tason syvyyteen. Ne tuodaan tasolle { $limit }, syvimmälle, jonka Skribisto säilyttää, sanoineen ja muotoiluineen.
   }
import-diagnostic-table-read-as-paragraphs = { $count ->
       [one] { $count } taulukossa tiedostossa { $path } on rivi, joka on paljon muita leveämpi, liian epäsymmetrinen ruudukoksi aseteltavaksi. Sen solut tuodaan kappaleina peräkkäin, sanoineen ja muotoiluineen.
      *[other] { $count } taulukossa tiedostossa { $path } on rivi, joka on paljon muita leveämpi, liian epäsymmetrinen ruudukoksi aseteltavaksi. Niiden solut tuodaan kappaleina peräkkäin, sanoineen ja muotoiluineen.
   }
import-diagnostic-spaces-shortened = { $count ->
       [one] { $count } välilyöntijakso tiedostossa { $path } oli pidempi kuin { $limit } välilyöntiä, ja se lyhennetään { $limit } välilyöntiin. Sen ympärillä olevat sanat ovat kaikki tallella.
      *[other] { $count } välilyöntijaksoa tiedostossa { $path } oli pidempiä kuin { $limit } välilyöntiä, ja ne lyhennetään { $limit } välilyöntiin. Niiden ympärillä olevat sanat ovat kaikki tallella.
   }
import-diagnostic-epigraph-not-carried = ”{ $title }” alkaa motolla, mutta tyyppi { $kind } ei voi sisältää mottoa. Lainaus säilytetään sen sijaan tekstin alussa.
import-diagnostic-epigraph-placement-ambiguous = Motto on kohteiden ”{ $title }” ja ”{ $below }” välissä, ja se voisi kuulua kummalle tahansa. Se annettiin kohteelle ”{ $title }”, jonne motto tavallisesti kuuluu.
import-document-diagnostics = { $errors ->
       [0] { $warnings ->
               [one] 1 huomio
              *[other] { $warnings } huomiota
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
versions-loading = Käydään läpi varmuuskopioitasi…
versions-empty = Tästä ei ole vielä aiempaa versiota
versions-error = Varmuuskopioitasi ei voitu lukea. Mitään ei ole kadonnut, mutta tämä luettelo voi olla puutteellinen
versions-did-not-exist = Ei ollut vielä olemassa { $date }
versions-deleted-after = Poistettu joskus { $date } jälkeen
versions-unreadable = { $count ->
       [one] { $count } varmuuskopiota ei voitu lukea
      *[other] { $count } varmuuskopiota ei voitu lukea
   }
versions-thinned = { $count ->
       [one] Vanhemmat versiot harvenevat ajan myötä. Projektin oma historia on jo pudottanut { $count } tämän tekstin aiemman tilan.
      *[other] Vanhemmat versiot harvenevat ajan myötä. Projektin oma historia on jo pudottanut { $count } tämän tekstin aiempaa tilaa.
   }
versions-source-backup = Varmuuskopiosta
versions-source-project = Projektin omasta historiasta
versions-list-caption = Yksi merkintä muutosta kohti, ei varmuuskopiota kohti
versions-pick-a-version = Valitse versio nähdäksesi, mikä on muuttunut
versions-earliest = Varhaisin tallessa oleva versio. Sitä vanhempaa ei ole, johon verrata.
versions-no-change = Kohteen tässä osassa mikään ei muuttunut
versions-formatting-only = Vain muotoilu muuttui tässä. Sanat ovat samat
versions-show-unchanged = Näytä muuttumattomat
versions-hide-unchanged = Piilota muuttumattomat
versions-next-change = Seuraava muutos
versions-near = lähellä kohtaa ”{ $text }”
versions-words-added = { $count ->
       [one] { $count } sana lisätty
      *[other] { $count } sanaa lisätty
   }
versions-words-removed = { $count ->
       [one] { $count } sana poistettu
      *[other] { $count } sanaa poistettu
   }
versions-blocks-moved = { $count ->
       [one] { $count } kappale siirretty
      *[other] { $count } kappaletta siirretty
   }
versions-pin = Kiinnitä varmuuskopio, josta tämä versio on peräisin. Automaattinen siivous ei koskaan poista sitä
versions-unpin = Irrota varmuuskopio, josta tämä versio on peräisin. Automaattinen siivous voi taas poistaa sen
versions-pinned-only = Näytä vain kiinnitetyt versiot
versions-pin-note = Vain varmuuskopioista peräisin olevat versiot voi kiinnittää. Kiinnitys säilyttää tiedoston, eikä projektin oma historia ole tiedosto.
versions-pinned-empty = Mitään ei ole vielä kiinnitetty
versions-pinned-empty-log = Mitään ei ole vielä kiinnitetty. Vain varmuuskopioista peräisin olevat versiot voi kiinnittää. Kiinnitys säilyttää tiedoston, eikä projektin oma historia ole tiedosto.
versions-range-filter = Näytä vain kahden päivämäärän väliset versiot
versions-filtered-empty = Mikään versio ei vastaa asettamiasi suodattimia
versions-clear-filters = Tyhjennä suodattimet
versions-last-30-days = Viimeiset 30 päivää
versions-restore-button = Palauta tämä versio
versions-restore-confirm-title = Korvataanko tämä teksti versiolla ajankohdalta { $date }?
versions-restore-confirm-text = Nykyinen teksti korvataan tekstillä, joka tällä kohteella oli ajankohtana { $date }.
versions-restore-confirm-with-comments = Nykyinen teksti korvataan tekstillä, joka tällä kohteella oli ajankohtana { $date }. { $count ->
       [one] { $count } kommentti on ankkuroitu nykyiseen tekstiin ja voi jäädä orvoksi.
      *[other] { $count } kommenttia on ankkuroitu nykyiseen tekstiin ja voi jäädä orvoiksi.
   }
versions-restore-confirm-undo-note = Ensin otetaan varmuuskopio, ja Ctrl+Z kumoaa tämän yhdellä kertaa.
versions-restored-toast = Versio ajankohdalta { $date } palautettu
versions-undo = Kumoa
versions-restore-row-gone = Tätä kohdetta ei enää ole tässä projektissa
versions-restore-no-home = Tämän kohteen tyyppi on muuttunut sen jälkeen, eikä vanhalle tekstille ole siinä paikkaa
versions-restore-no-safety-copy = Turvavarmuuskopiota ei otettu, joten mitään ei muutettu
versions-restore-failed = Palautus epäonnistui: { $error }
versions-restore-backup-busy = Varmuuskopiointi on jo käynnissä. Yritä hetken kuluttua uudelleen
versions-restore-in-backup-file = Katselet varmuuskopiotiedostoa. Avaa itse projekti palauttaaksesi siihen
versions-restore-no-project = Projektia ei ole auki
versions-recreate-button = Tuo tämä takaisin…
versions-recreate-picker-title = Minne sen pitäisi mennä?
versions-recreate-picker-empty = Tässä projektissa ei ole mappia, johon sen voisi sijoittaa
versions-recreate-picker-confirm = Tuo se tähän
versions-recreate-picker-cancel = Peruuta
versions-recreate-untitled = tämä kohde
versions-recreate-confirm-title = Tuodaanko ”{ $item }” takaisin?
versions-recreate-confirm-text = Se lisätään kohteeseen { $destination } tekstillä, joka sillä oli ajankohtana { $date }.
versions-recreate-confirm-undo-note = Seuraavan ilmoituksen Kumoa poistaa sen heti uudelleen.
versions-recreated-toast = ”{ $item }” on taas projektissasi
versions-recreated-partial-toast = { $count ->
       [one] ”{ $item }” on palautettu, mutta yhtä sen teksteistä ei voitu lukea
      *[other] ”{ $item }” on palautettu, mutta { $count } sen teksteistä ei voitu lukea
   }
versions-recreate-already-here = Tämä kohde on jo projektissasi
versions-recreate-no-destination = Valitse mapista paikka, johon se sijoitetaan
versions-recreate-unreadable = Varmuuskopiota ei voitu lukea, joten mitään ei lisätty
versions-recreate-failed = Sitä ei voitu tuoda takaisin: { $error }
versions-changed-percent = { $percent } % tästä muuttui
versions-hidden-paragraphs = { $count ->
       [one] … { $count } muuttumaton kappale …
      *[other] … { $count } muuttumatonta kappaletta …
   }

timeline-title = Palaa ajassa taaksepäin
timeline-coverage = { $count ->
       [one] { $count } tallennettu versio, ulottuu ajankohtaan { $oldest }
      *[other] { $count } tallennettua versiota, ulottuu ajankohtaan { $oldest }
   }
timeline-loading = Käydään läpi projektisi historiaa…
timeline-empty = Tästä projektista ei ole vielä tallennettu versiota
timeline-no-changes = Mikään ei ole muuttunut sen jälkeen
timeline-no-text-changes = Tekstiä ei ole muutettu sen jälkeen
timeline-slider-label = Tallennettu versio
timeline-series-name = Projektin koko
timeline-bars-caption = Jokainen palkki on tallennettu versio, yhtä korkea kuin projekti oli silloin. Korostettu on se, jota katsot.
timeline-bars-caption-periods = Versioita on liian monta näytettäväksi yksitellen, joten jokainen palkki on yksi { $period }, yhtä korkea kuin projekti oli sen lopussa.
timeline-unit-hour = tunti
timeline-unit-day = päivä
timeline-unit-week = viikko
timeline-unit-month = kuukausi
timeline-range-filter = Näytä vain kahden päivämäärän väliset versiot
timeline-range-empty = Näiltä päiviltä ei ole tallennettu versiota
timeline-open-period = Avaa tämä jakso
timeline-show-all = Näytä koko historia
timeline-changed-since = { $count ->
       [one] { $count } kohde eroaa ajankohdan { $date } ja nykyisen projektisi välillä
      *[other] { $count } kohdetta eroaa ajankohdan { $date } ja nykyisen projektisi välillä
   }
timeline-kind-added = Kirjoitettu sen jälkeen
timeline-kind-removed = Ei enää projektissa
timeline-kind-changed = Muokattu sen jälkeen
timeline-kind-moved = Siirretty sen jälkeen
timeline-not-yet-written = Tätä ei ollut vielä olemassa silloin
timeline-no-text-of-its-own = Tällä ei ole omaa tekstiä. Se on otsikko sisältämilleen kohteille
timeline-reader-close = Sulje
timeline-reader-stamp = Sellaisena kuin se oli { $date }
timeline-reader-loading = Avataan tallennettua versiota…
timeline-reader-unreadable = Tätä tallennetta ei voitu lukea. Varmuuskopio on ehkä siirretty tai poistettu, tai se on asemalla, joka ei ole kytkettynä.
timeline-reader-compared = Sellaisena kuin se oli { $date }, verrattuna nykyiseen
timeline-reader-diff-legend = Yliviivattu on poistunut sen jälkeen; alleviivattu on lisätty.
timeline-reader-view-label = Mitä näytetään
timeline-reader-view-diff = Muutokset
timeline-reader-view-text = Teksti
timeline-reader-deleted = Tämä ei ole enää projektissasi. Voit lukea sen ja kopioida sen täältä.
timeline-prose-only-record = Tämä kohta on peräisin projektin omasta historiasta, joka säilyttää vain tekstin, joten sen jälkeen poistettua tai siirrettyä ei voi näyttää. Valitse sitä varten varmuuskopio.

settings-sec-extensions = Laajennukset

settings-group-games-forward = Aina eteenpäin
settings-games-forward-toggle = Pelaa ”Aina eteenpäin”
settings-games-forward-hint =
       Pelatessasi mitään kirjoittamaasi ei voi ottaa takaisin: askelpalautin, Delete,
       leikkaaminen, vetäminen ja pudottaminen sekä kumoaminen ovat pois käytöstä alla valitsemillasi pinnoilla.
       Voit silti kirjoittaa, liittää, muotoilla ja liikkua vapaasti. Luonnos vain kasvaa.
settings-games-session-warning =
       Tämä on istuntokohtainen valinta: sitä ei koskaan tallenneta. Projektin sulkeminen tai
       Skribiston lopettaminen päättää pelin aina, ja kaikki pelin aikana kirjoittamasi
       tulee samalla hetkellä taas kumottavaksi.
settings-group-games-scope = Missä se on voimassa
settings-games-in-prose = Käsikirjoituksen teksti
settings-games-in-synopsis = Tiivistelmät
settings-games-scope-hint =
       Kommentit, alaviitteet ja otsikot eivät koskaan jäädy: niihin kirjaat korjauksen,
       jonka juuri kielsit itseltäsi.
settings-games-inert-warning =
       ”Aina eteenpäin” on päällä, mutta se ei koske mitään. Valitse yllä vähintään yksi pinta,
       muuten se ei muuta kirjoittamisessasi mitään.

games-title = Kirjoituspelit
games-forward-name = Aina eteenpäin
games-forward-blurb =
       Luonnostele ottamatta mitään takaisin. Poistaminen, leikkaaminen ja kumoaminen ovat pois käytöstä
       pelin ajan. Kirjoita seuraava virke sen sijaan, että korjaisit edellistä.
games-forward-playing = Peli käynnissä: poistaminen on pois käytöstä
games-forward-idle = Ei pelissä
games-session-note = Päättyy, kun suljet projektin.
games-scope-prose-and-synopsis = Koskee tekstiäsi ja tiivistelmiäsi.
games-scope-prose = Koskee tekstiäsi.
games-scope-synopsis = Koskee tiivistelmiäsi.
games-scope-nothing = Ei vielä koske mitään. Valitse pinta asetuksista.
games-settings-link = Kirjoituspelien asetukset…
statusbar-games-forward = Aina eteenpäin
statusbar-games-forward-tooltip =
       ”Aina eteenpäin” on päällä: poistaminen, leikkaaminen ja kumoaminen ovat pois käytöstä
       luonnostellessasi. Lopeta peli napsauttamalla.
goal-progress-words = { $g ->
       [one] { $count }/{ $goal } sana
      *[other] { $count }/{ $goal } sanaa
   }
goal-progress-characters = { $g ->
       [one] { $count }/{ $goal } merkki
      *[other] { $count }/{ $goal } merkkiä
   }
goal-count-words = { $n ->
       [one] { $count } sana
      *[other] { $count } sanaa
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
overview-excluded-from-export = Jätetty pois viennistä, joten sitä ei lasketa mihinkään kokonaismäärään

goal-subtree-total-words = { $count ->
       [one] { $items } sisällä oleva tavoite on yhteensä { $words } sanaa
      *[other] { $items } sisällä olevaa tavoitetta on yhteensä { $words } sanaa
   }
goal-subtree-total-characters = { $count ->
       [one] { $items } sisällä oleva tavoite on yhteensä { $words } merkkiä
      *[other] { $items } sisällä olevaa tavoitetta on yhteensä { $words } merkkiä
   }

new-work-goal-unit = Laskentayksikkö
new-work-goal-unit-hint = Käytetään kaikissa tämän projektin sana- ja merkkitavoitteissa. Voi muuttaa myöhemmin.

settings-group-goal-unit = Tavoitteet
settings-goal-unit-switch-title = Vaihda laskentayksikkö
settings-goal-unit-switch-text = Tälle projektille jo asetetut tavoitteet ja virstanpylväät on syötetty yksikössä ”{ $from }”. Vaihto yksikköön ”{ $to }” ei muunna niitä, joten jokainen olemassa oleva luku tulkitaan nyt yksikössä ”{ $to }”.
settings-goal-unit-switch-informative = Mitään ei menetetä. Voit vaihtaa takaisin milloin tahansa palauttaaksesi alkuperäisen tulkinnan ja päivittää sitten tavoitteet, jotka haluat säilyttää.
settings-goal-unit-switch-confirm = Vaihda

goal-distribute-action = Jaa…
distribute-title = Jaa tavoite
distribute-empty = Tämän sisällä ei ole mitään, jolle tavoitteen voisi jakaa.
distribute-weight = Jakoperuste
distribute-weight-length = Pituuden mukaan
distribute-weight-rows = Kappalemäärän mukaan
distribute-weight-even = Tasaisesti
distribute-overwrite = Korvaa jo asetetut tavoitteet
distribute-col-item = Kohde
distribute-col-current = Nyt
distribute-col-proposed = Jälkeen
distribute-total = Nämä ovat yhteensä { $total }, tavoitteena { $goal }.
distribute-over-budget = Sisällä jo asetetut tavoitteet ovat { $over } suuremmat kuin tämän säiliön oma tavoite. Nosta sen tavoitetta, laske niiden tavoitteita tai korvaa ne.
distribute-apply = Jaa
distribute-cancel = Peruuta

milestone-target-gone = Tavoitekohde poistettu
milestone-no-target = Ei tavoitetta
milestone-add = Lisää
milestone-add-label = Mitä pitäisi saavuttaa

pace-summary-title = Kirjan tilanne
pace-summary-remaining = { $words } jäljellä
pace-summary-open = Avaa suunnitelma
pace-summary-close = Sulje
pace-summary-dont-show = Älä näytä tätä avattaessa
pace-summary-menu = Kirjoitussuunnitelma…

menu-help-topics = Ohje&aiheet
menu-help-shortcuts = &Pikanäppäimet…
menu-help-website = &Skribisto verkossa
menu-help-report = &Ilmoita ongelmasta…
menu-help-check-updates = Tarkista päivit&ykset
menu-command-palette = &Komentopaletti…

learn-help-topics = Ohjeaiheet
learn-shortcuts = Pikanäppäimet
learn-website = Skribisto verkossa

help-window-title = Ohje
help-filter-topics = Suodata aiheita
help-no-matching-topic = Mikään aihe ei vastaa hakua.
help-topic-missing = Tämä aihe ei ole enää käytettävissä.
help-back = Takaisin
help-not-translated = Tätä sivua ei ole vielä käännetty, joten se näytetään englanniksi.

help-section-getting-started = Aloittaminen
help-section-writing = Kirjoittaminen
help-section-reviewing = Tarkistaminen
help-section-exchanging = Tekstin tuonti ja vienti
help-section-keeping = Pidä työsi turvassa
help-section-extensions = Laajennukset

help-topic-getting-started = Ensimmäinen projektisi
help-topic-writing-model = Miten kirja kootaan
help-topic-drafts-and-old-versions = Vanhemman luonnoksen säilyttäminen
help-topic-goals-and-pace = Tavoitteet ja tahti
help-topic-comments = Kommentit
help-topic-round-trip = Kirjan lähettäminen lukijalle
help-topic-export = Vienti
help-topic-import-documents = Asiakirjojen tuonti
help-topic-import-projects = Koko projektin tuominen
help-topic-backups-and-versions = Varmuuskopiot ja versiot

help-shortcuts-title = Pikanäppäimet
help-shortcuts-filter = Suodata pikanäppäimiä
help-shortcuts-no-matches = Mikään pikanäppäin ei vastaa hakua.
help-shortcuts-rebind = Muuta pikanäppäimiä…
help-shortcuts-close = Sulje

command-palette-placeholder = Kirjoita komento

shortcut-name-binder-duplicate = Monista
shortcut-name-comments-add = Lisää kommentti
shortcut-name-comments-add-paragraph = Kommentoi kappaletta
shortcut-name-spellcheck-toggle = Oikolue
shortcut-name-editor-tab-close = Sulje välilehti
shortcut-name-editor-tab-pin = Kiinnitä tai irrota välilehti
shortcut-name-editor-save = Tallenna
shortcut-name-work-export = Vie…
shortcut-name-work-new = Uusi projekti
shortcut-name-work-open = Avaa projekti
shortcut-name-window-new = Uusi ikkuna
shortcut-name-work-close = Sulje projekti
shortcut-name-app-settings = Asetukset
shortcut-name-app-quit = Lopeta
shortcut-name-editor-insert-footnote = Lisää alaviite
shortcut-name-format-scene-break = Lisää kohtauskatko
shortcut-name-format-major-scene-break = Lisää suuri kohtauskatko
shortcut-name-format-link = Lisää linkki
shortcut-name-format-mark-nbsp = Lisää sitova välilyönti
shortcut-name-format-mark-nbhyphen = Lisää sitova tavuviiva
shortcut-name-format-mark-nnbsp = Lisää kapea sitova välilyönti
shortcut-name-format-mark-zwsp = Lisää leveydetön välilyönti
shortcut-name-go-next = Seuraava
shortcut-name-go-prev = Edellinen
shortcut-name-go-to = Siirry kohteeseen
shortcut-name-outline-toggle = Näytä tai piilota jäsennys
shortcut-name-preview-toggle = Näytä tai piilota esikatselukaista
shortcut-name-view-fullscreen = Koko näyttö päälle/pois
shortcut-name-view-focus-mode = Häiriötön tila päälle/pois
shortcut-name-editor-size-increase = Suurenna tekstiä
shortcut-name-editor-size-decrease = Pienennä tekstiä
shortcut-name-editor-size-reset = Palauta tekstin koko
shortcut-name-editor-find = Etsi
shortcut-name-editor-replace = Korvaa
shortcut-name-editor-find-next = Seuraava osuma
shortcut-name-editor-find-prev = Edellinen osuma
shortcut-name-search-show = Hae projektista
shortcut-name-search-replace = Korvaa projektissa
shortcut-name-outline-open-to-side = Avaa sivulle
shortcut-name-help-topics = Ohjeaiheet
shortcut-name-help-shortcuts = Pikanäppäimet
shortcut-name-help-website = Skribisto verkossa
shortcut-name-help-report = Ilmoita ongelmasta
shortcut-name-help-check-updates = Tarkista päivitykset
update-check-behind = Versio { $version } on saatavilla.
update-check-current = Käytössäsi on uusin versio.
update-check-failed = Uudempaa versiota ei voitu tarkistaa. { $error }
shortcut-name-command-palette = Komentopaletti
help-section-reference = Käsitteet

help-concept-scene-break = Kohtauskatko
help-concept-major-scene-break = Suuri kohtauskatko
help-concept-find-in-prose = Etsi tekstistä
help-concept-story-bible = Tarinan tietopankki
help-concept-goal-unit = Sanat tai merkit
help-concept-goal-progress = Edistyminen
help-concept-manuscript-words = Mikä lasketaan käsikirjoitukseksi
help-concept-exportable = Jätetään pois viennistä
help-concept-distribute = Tavoitteen jakaminen
help-concept-subtree-total = Sisällä olevien tavoitteiden summa
help-concept-milestone = Virstanpylväs
help-concept-pace-plan = Tahtisuunnitelma

help-concept-tag = Tunniste
help-concept-label = Nimiö
help-concept-point-of-view = Näkökulma
help-concept-epigraph = Motto
help-concept-footnote = Alaviite
help-concept-chapter-mode = Luvun rakenne
help-concept-comment = Kommentti
help-concept-backup = Varmuuskopio
help-concept-version = Versio
help-concept-trash = Roskakori
help-concept-spellcheck = Oikoluku
help-concept-search-replace = Etsi ja korvaa
help-concept-note-template = Muistiinpanomalli
help-concept-story-bible-entry = Tietopankin merkintä
help-concept-text-replacement = Tekstinkorvaus
help-concept-smart-punctuation = Älykäs välimerkitys
help-concept-export-style = Vientityyli
help-concept-round-trip-marks = Paluumerkit

epigraph-mark-attribution = Lähderivi
epigraph-mark-attribution-tip = Merkitse kohdistimen rivi lainauksen lähteeksi, jotta se tulostetaan lähdeviitteenä
margin-lane-name = Marginaalimerkinnät
margin-lane-provider-comments = Kommentit
margin-lane-provider-story-bible = Tietopankin merkintä
margin-lane-provider-story-bible-hint = Muistiinpanon Tekstissä-näkymässä: missä merkintä mainitaan nimeltä ja mitkä kohtaukset kerrotaan sen silmin. Merkitsee nimet myös itse tekstiin.
margin-lane-mark-point-of-view = Kerrottu tästä näkökulmasta
margin-lane-mark-named-here = Mainittu tässä
margin-lane-provider-comments-hint = Kohdat, joihin on liitetty kommentti
margin-lane-provider-search = Hakuosumat
margin-lane-provider-search-hint = Jokainen viimeisimmän hakusi osuma, missä kohtaa asiakirjaa tahansa
margin-lane-provider-boundaries = Kunkin asiakirjan alku
margin-lane-provider-boundaries-hint = Viiva jokaisen kohtauksen yläpuolella virrassa, jotta näet, missä olet
margin-lane-provider-spelling = Oikeinkirjoitus
margin-lane-provider-spelling-hint = Kohdat, joissa oikoluku on merkinnyt sanan. Pois päältä, ellet pyydä: se merkitsee koneen mielipiteen tekstistäsi
margin-lane-spelling = { $word }, mahdollinen kirjoitusvirhe
margin-lane-search-hit = { $text }, osuma { $index }/{ $total }
margin-lane-search-current = { $text }, osuma { $index }/{ $total }, jonka kohdalla olet
margin-lane-boundary = Alku: { $title }
margin-lane-boundary-untitled = nimetön asiakirja
settings-page-margin-lane = Marginaalimerkinnät
settings-desc-margin-lane = Vierityspalkin vieressä oleva kaista ja mitä se näyttää
settings-margin-lane-enabled = Näytä marginaalikaista
settings-margin-lane-enabled-hint = Sama kytkin kuin Näytä ▸ Marginaalimerkinnät.
settings-margin-lane-enabled-more = Se ei koskaan väitä mitään vääräksi: se näyttää, missä asiat ovat, ja jättää tulkinnan sinulle.
settings-group-margin-lane-marks = Mitä se merkitsee
settings-margin-lane-no-providers = Mikään ei vielä merkitse kaistaa.
settings-group-margin-lane-texture = Vuoropuhelun tekstuuri
settings-margin-lane-texture = Näytä vuoropuhelun tekstuuri
settings-margin-lane-texture-hint = Yksi palkki kappaletta kohden: kuinka pitkä se on ja kuinka suuri osa siitä on repliikkiä.
settings-margin-lane-texture-more = Ei mitata kielillä, joille ei ole koottu käytäntöä; silloin palkki näyttää vain pituuden.
settings-group-margin-lane-surfaces = Missä se näkyy
settings-margin-lane-surface-editor = Tekstimuokkain
settings-margin-lane-surface-stream = Virrat
settings-margin-lane-surface-search-preview = Haun esikatselu
menu-margin-lane = &Marginaalimerkinnät

status-preset-drafting = Luonnostelu
status-preset-passes = Muokkauskierrokset
status-preset-plume = Plume Creator
status-todo = Tekemättä
status-draft = Luonnos
status-revised = Tarkistettu
status-final = Lopullinen
status-outline = Hahmotelma
status-first-edit = 1. muokkaus
status-second-edit = 2. muokkaus
status-done = Valmis
status-plume-draft-1 = 1. luonnos
status-plume-draft-2 = 2. luonnos
status-plume-draft-3 = 3. luonnos
status-plume-edit-1 = 1. muokkaus
status-plume-edit-2 = 2. muokkaus
status-plume-edit-3 = 3. muokkaus
status-plume-proofread = Oikoluettu
status-plume-finished = Valmistunut
new-work-statuses = Työnkulku
new-work-statuses-hint = Vaiheet, joiden läpi kohtaus kulkee. Voit nimetä niitä uudelleen, järjestää niitä tai lisätä niitä milloin tahansa.
status-none = Ei tilaa
inspector-status = Tila
overview-col-status = Tila
overview-status-mixed = alla olevien tilat vaihtelevat
help-concept-status = Tila
status-completion-title = Kirjan tilanne
status-completion-headline = { $done }/{ $total } kohtausta valmiina
status-completion-empty = Ei vielä kohtauksia. Tämä lukee käsikirjoitusta, joten se täyttyy kirjoittaessasi.
status-completion-open = Kirjan tilanne…

settings-page-statuses = Tilat
settings-desc-statuses = Nimeä uudelleen, järjestä, lisää ja poista vaiheet, joiden läpi kohtaus kulkee
settings-statuses-add = Lisää tila
settings-statuses-add-placeholder = Uuden tilan nimi
settings-statuses-desc =
       Vaiheet, joiden läpi kohtaus kulkee, järjestyksessä keskeneräisimmästä ylhäällä
       valmiimpaan alhaalla. Tämä järjestys määrää, mitä ”vähemmän valmis kuin” tarkoittaa
       kaikkialla muualla sovelluksessa, joten järjestä ne työtapasi mukaan.
settings-statuses-apply-preset = Käytä valmista sarjaa…
settings-statuses-preset-applied = { $added ->
       [one] 1 tila lisätty
      *[other] { $added } tilaa lisätty
   }
settings-statuses-preset-refused = Tällä projektilla on jo tilasarja. Valmiit sarjat täyttävät vain tyhjän. Poista ensin tilat, joita et halua.
settings-statuses-duplicate = ”{ $name }” on jo tilasarjassa
settings-statuses-added = Lisätty ”{ $name }”
settings-statuses-move-up = Siirrä ylös (vähemmän valmis)
settings-statuses-move-down = Siirrä alas (valmiimpi)
settings-statuses-details-placeholder = Mitä tämä vaihe tarkoittaa (valinnainen)
settings-statuses-delete = Poista ”{ $name }”
settings-statuses-delete-in-use = { $count ->
       [one] Poista ”{ $name }”: 1 kohteella on tämä tila, ja se menettää tilansa
      *[other] Poista ”{ $name }”: { $count } kohteella on tämä tila, ja ne menettävät tilansa
   }
settings-statuses-deleted = Poistettu ”{ $name }”
settings-statuses-deleted-in-use = { $count ->
       [one] Poistettu ”{ $name }”. 1 kohteella ei ole enää tilaa.
      *[other] Poistettu ”{ $name }”. { $count } kohteella ei ole enää tilaa.
   }
settings-statuses-empty-title = Ei vielä työnkulkua
settings-statuses-empty-body = Tila kertoo, kuinka pitkällä kohtaus on. Lisää tila yllä tai aloita valmiista sarjasta.
status-category-planned = Suunniteltu
status-category-drafting = Luonnostelu
status-category-needs-work = Vaatii työtä
status-category-revised = Tarkistettu
status-category-final = Lopullinen

progress-percent = { $percent } %
progress-percent-with-label = { $percent } % · { $label }
progress-import-opening = Avataan projektia…
progress-import-outline = Luetaan jäsennystä…
progress-import-converting = Muunnetaan lukuja ja kohtauksia…
progress-import-writing = Kirjoitetaan .skrib-tiedostoa…
progress-import-done = Valmis
progress-import-structure = Selvitetään rakennetta…
progress-export-start = Aloitetaan…
progress-export-writing = Kirjoitetaan…
progress-project-reading = Luetaan projektia…
settings-sec-work-titled = Projekti: { $title }
import-notice-flattened-rich-texts = { $count ->
    [one] Yksi teksti oli sisäkkäin syvemmällä kuin Skribisto-projekti sallii; sen sanat säilytettiin pelkkänä tekstinä ilman muotoilua.
   *[other] { $count } tekstiä oli sisäkkäin syvemmällä kuin Skribisto-projekti sallii; niiden sanat säilytettiin pelkkänä tekstinä ilman muotoilua.
}
import-notice-free-text-badges = { $count ->
    [one] Yksi kohtausmerkki näyttää vapaalta tekstiltä eikä tilaluettelolta, joten se säilytettiin kohteen nimiönä eikä siitä tehty tunnistetta.
   *[other] { $count } eri kohtausmerkkiä näyttää vapaalta tekstiltä eikä tilaluettelolta, joten ne säilytettiin kohteiden nimiöinä eikä niistä tehty tunnisteita.
}
import-notice-separator-text-dropped = Erotin ”{ $name }” sisälsi tekstiä, jolle Skribisto-projektissa ei ole paikkaa; tekstiä ei tuotu.
import-notice-separator-without-scene = Erottimella ”{ $name }” ei ollut luvussaan edeltävää kohtausta, johon se olisi voitu liittää, joten sitä ei tuotu.
import-notice-chapter-without-scenes = Luvussa ”{ $name }” ei ole kohtauksia, joten sen erottimia ei tuotu.
import-notice-rich-text-unconverted = Osaa muotoillusta tekstistä ei voitu muuntaa ({ $error }); sitä ei tuotu.
import-notice-unresolved-cross-link = Linkkiä esiintyjämerkintään #{ $number } ei voitu seurata: merkintä puuttuu tai oli roskakorissa.
import-notice-read-copy = Tästä projektista luettiin { $container ->
    [single-file] yhden tiedoston kopio
   *[folder] kansiokopio
}.
import-notice-read-copy-dated = Tästä projektista luettiin { $container ->
    [single-file] yhden tiedoston kopio
   *[folder] kansiokopio
}, jota muutettiin viimeksi { $day }.
import-notice-original-format = Tämä projekti on Manuskriptin alkuperäisessä vuoden 2016 muodossa. Se luettiin kokonaan; mitään siinä ei muutettu.
import-notice-newer-single-file-beside = ”{ $file }” on uudempi kuin tämä kansio ({ $file_day } vs. { $folder_day }). Manuskript jättää vanhan kansion paikalleen, kun projekti siirtyy yhden tiedoston tilaan, joten tämä kansio voi olla hylätty kopio; .msk-tiedoston tuominen sen sijaan lukisi uudemman version.
import-notice-pickle-skipped = Tiedosto ”{ $file }” jätettiin lukematta. Se on Python-pickle, ja sellaisen lataaminen projektitiedostosta oli tietoturvaongelma itse Manuskriptissa (CVE-2021-35196); sen sisältämät asetukset ovat näyttöasetuksia, eivät kirjoitusta.
import-notice-file-unreadable = Tiedostoa ”{ $file }” ei voitu lukea ({ $error }), joten se ohitettiin; kaikki muu tuotiin.
import-notice-not-utf8 = Tiedosto ”{ $file }” ei ole kelvollista UTF-8:aa; merkit, joita ei voitu lukea, korvattiin sen sijaan, että tuonti olisi keskeytetty.
import-notice-table-unreadable = Tiedostoa ”{ $file }” ei voitu lukea ({ $error }); sen sisältöä ei tuotu.
import-notice-outline-missing = Tiedosto ”{ $file }” puuttuu, joten tässä projektissa ei ole tuotavaa käsikirjoitusta.
import-notice-outline-unreadable = Tiedostoa ”{ $file }” ei voitu lukea ({ $error }); käsikirjoitusta ei tuotu.
import-notice-named-none-row = Tiedosto ”{ $file }” alkaa nimetyllä rivillä (”{ $name }”) kohdassa, jossa Manuskript pitää tyhjää ”none”-merkintäänsä. Rivi jätettiin pois, jotta sen jälkeiset rivit säilyttävät käsikirjoituksen käyttämät numerot.
import-notice-unknown-field = Tiedostossa ”{ $file }” on kenttä, jota tämä tuoja ei tunne: ”{ $field }”. Sitä ei tuotu.
import-notice-colorless-labels = { $count ->
    [one] Tiedostossa ”{ $file }” on rivi ilman väriä. Se tuotiin ilman väriä; Manuskript lopettaa projektin lataamisen tällaiseen riviin.
   *[other] Tiedostossa ”{ $file }” on { $count } riviä ilman väriä. Ne tuotiin ilman väriä; Manuskript lopettaa projektin lataamisen tällaiseen riviin.
}
import-notice-settings-unreadable = Tiedostoa ”{ $file }” ei voitu lukea ({ $error }); projektin kieltä ei tuotu.
import-notice-revisions-unreadable = Tiedostoa ”{ $file }” ei voitu lukea ({ $error }); projektin historiaa ei tuotu, mutta mihinkään muuhun tämä ei vaikuttanut.
import-notice-foreign-files = { $count ->
    [one] Projektissa on tiedosto, joka ei kuulu Manuskript-muotoon, eikä sitä tuotu: { $files }.
   *[other] Projektissa on { $count } tiedostoa, jotka eivät kuulu Manuskript-muotoon, eikä niitä tuotu: { $files }.
}
import-notice-foreign-files-more = Projektissa on { $count } tiedostoa, jotka eivät kuulu Manuskript-muotoon, eikä niitä tuotu: { $files } ja { $more ->
    [one] yksi muu
   *[other] { $more } muuta
}.
import-notice-not-a-character-file = ”{ $file }” ei ole Manuskriptin henkilötiedosto, ja se jätettiin pois.
import-notice-not-a-text-item = ”{ $file }” ei ole Manuskriptin tekstikohde, ja se jätettiin pois jäsennyksestä.
import-notice-missing-blank-line = Tiedostossa ”{ $file }” ei ole tyhjää riviä otsakkeen jälkeen. Sen viimeinen kenttä säilytettiin, vaikka Manuskript menettää sen.
import-notice-folder-without-metadata = Kansiossa ”{ $folder }” ei ole tiedostoa { $file }, joten se tuotiin tavallisena kansiona, joka on nimetty hakemiston mukaan. Manuskript ohittaa tällaisen hakemiston ja kaiken sen sisällön.
import-notice-rows-given-ids = { $count ->
    [one] Yhdellä jäsennyksen rivillä ei ollut omaa tunnusta, joten sille annettiin sellainen. Manuskript pudottaa tällaisen rivin tai lopettaa projektin lataamisen, jos rivi on kansio.
   *[other] { $count } jäsennyksen rivillä ei ollut omaa tunnusta, joten niille annettiin sellainen. Manuskript pudottaa tällaisen rivin tai lopettaa projektin lataamisen, jos rivi on kansio.
}
import-notice-plots-unreadable = Tiedostoa ”{ $file }” ei voitu lukea ({ $error }); juonia ei tuotu.
import-notice-world-unreadable = Tiedostoa ”{ $file }” ei voitu lukea ({ $error }); maailman merkintöjä ei tuotu.
import-notice-world-not-opml = ”{ $file }” ei ole OPML-asiakirja; maailman merkintöjä ei tuotu.
import-notice-text-flattened = Kohteen ”{ $title }” teksti on sisäkkäin syvemmällä kuin Skribisto-projekti sallii; sen sanat säilytettiin pelkkänä tekstinä ilman muotoilua.
import-notice-text-unconverted = Kohteen ”{ $title }” tekstiä ei voitu muuntaa ({ $error }); se säilytettiin täsmälleen sellaisena kuin se oli kirjoitettu.
import-notice-text-without-file-name = Kohteen ”{ $title }” { $kind ->
    [scene] kohtaustekstille
    [note] muistiinpanotekstille
    [synopsis] tiivistelmälle
   *[title] otsikolle
} ei voitu antaa tiedostonimeä, joten sitä ei tuotu.
import-notice-unknown-label = Kohteella ”{ $title }” on nimiö { $index }, jota ei ole tämän projektin nimiöluettelossa. Se tuotiin ilman nimiötä.
import-notice-unknown-status = Kohteella ”{ $title }” on tila { $index }, jota ei ole tämän projektin tilaluettelossa. Se tuotiin ilman tilaa.
import-notice-unknown-point-of-view = Kohde ”{ $title }” kerrotaan henkilön kautta, joka ei kuulu tämän projektin esiintyjiin. Sen näkökulmaa ei asetettu.
import-notice-unresolved-references = Kohteen ”{ $title }” tekstin viittauksia ei voitu ratkaista ({ $error }); teksti säilytettiin sellaisena kuin se muunnettiin, viittausmerkit mukaan lukien.
import-notice-custom-icons = { $count ->
    [one] Yhdellä kohteella oli oma kuvake. Skribisto antaa jokaiselle kohteelle sen tyypin kuvakkeen, joten kuvaketta ei tuotu; muuten kohde ei muuttunut.
   *[other] { $count } kohteella oli oma kuvake. Skribisto antaa jokaiselle kohteelle sen tyypin kuvakkeen, joten kuvakkeita ei tuotu; muuten kohteet eivät muuttuneet.
}
import-notice-character-colors = { $count ->
    [one] Yhdellä henkilöllä oli väri nimensä vieressä. Skribisto värittää tunnisteita eikä kohteita, joten väriä ei tuotu; itse henkilö tuotiin.
   *[other] { $count } henkilöllä oli väri nimensä vieressä. Skribisto värittää tunnisteita eikä kohteita, joten värejä ei tuotu; itse henkilöt tuotiin kaikki.
}
import-notice-versions-kept = { $found ->
    [one] Tähän projektiin oli tallennettu yksi aiempi versio
   *[other] Tähän projektiin oli tallennettu { $found } aiempaa versiota
}; { $kept ->
    [one] yksi säilytettiin
   *[other] { $kept } säilytettiin
} harvennettuna samalla tavalla kuin Skribisto harventaa omaa historiaansa. Ne ovat Versiot-paneelissa.
import-notice-versions-unplaced = { $count ->
    [one] Yksi aiempi versio kuuluu kohteeseen, jota ei enää ole projektissa, eikä sitä voitu sijoittaa.
   *[other] { $count } aiempaa versiota kuuluu kohteeseen, jota ei enää ole projektissa, eikä niitä voitu sijoittaa.
}
dialog-open-work-title = Avaa Skribisto-projekti
dialog-save-as-file-title = Tallenna yhtenä .skrib-tiedostona
dialog-save-as-folder-title = Valitse projektille yläkansio
dialog-manuskript-file-title = Valitse Manuskript-projekti
dialog-manuskript-folder-title = Valitse Manuskript-projektin kansio
dialog-filter-skrib = Skribisto-projekti
dialog-filter-manuskript = Manuskript-projekti
dialog-filter-plume = Plume Creator -projekti
dialog-filter-hunspell-aff = Hunspell-affiksitiedosto (.aff)
dialog-filter-hunspell-dic = Hunspell-sanakirja (.dic)
dialog-filter-export = Viety tiedosto
shortcut-name-view-margin-lane = Marginaalimerkinnät päälle/pois
df-theme-paper = Paperi
df-theme-sepia = Seepia
df-theme-night = Yö
df-theme-high-contrast = Suuri kontrasti
df-theme-midnight = Keskiyö
welcome-example-byline = Kirjoittanut { $author }
welcome-sections-a11y = Tervetulo-ikkunan osiot
comments-a11y-summary-author = { $author }: { $text }
comments-a11y-summary-replies = { $text } ({ $count ->
    [one] 1 vastaus
   *[other] { $count } vastausta
})
comments-a11y-summary-author-replies = { $author }: { $text } ({ $count ->
    [one] 1 vastaus
   *[other] { $count } vastausta
})
margin-lane-a11y-mark = { $label }, { $percent } prosentin kohdalla
margin-lane-a11y-marks = { $label }, { $count } merkintää, { $percent } prosentin kohdalla
margin-lane-a11y-fallback = { $count ->
    [one] Marginaalimerkinnät, 1 merkintä
   *[other] Marginaalimerkinnät, { $count } merkintää
}
export-choose-unnamed-binder = Nimetön mappi
export-style-neutral = Neutraali
export-style-manuscript-shunn = Vakiokäsikirjoitus (Shunn)
export-style-ebook-clean = Pelkistetty e-kirja
date-time-short = { $day }.{ $month }.{ $year } klo { $hour }.{ $minute }
pace-day-label = { $day }.{ $month }.
byte-size-bytes = { $value } t
byte-size-kb = { $value } kt
byte-size-mb = { $whole },{ $tenth } Mt
byte-size-gb = { $whole },{ $tenth } Gt
unit-percent = { $value } %
unit-pixels = { $value } px
settings-paratext-error-no-name = Rakenteelle on annettava `name`.
export-error-no-project = Vietävää projektia ei ole auki.

shortcut-group-work = Projekti
shortcut-group-edit = Muokkaa
shortcut-group-view = Näytä
shortcut-group-document = Asiakirja
shortcut-group-format = Muotoilu
shortcut-group-go = Siirry
shortcut-group-tools = Työkalut
shortcut-group-help = Ohje

## Manuskript import: the headings of the notes it writes (a character's
## sheet, a world entry, a plot and its steps, the project details, the
## snowflake summary ladder) and the title of a project with no name. Written
## into the imported project once, in the interface language; the writer can
## rename them like any other text. Goal and Conflict reuse the note templates'
## own words.
import-manuskript-section-paragraph-summary = Yhteenveto yhtenä kappaleena
import-manuskript-section-full-summary = Koko yhteenveto
import-manuskript-section-motivation = Motivaatio
import-manuskript-section-epiphany = Oivallus
import-manuskript-section-notes = Muistiinpanot
import-manuskript-section-description = Kuvaus
import-manuskript-section-passion = Intohimo
import-manuskript-section-result = Tulos
import-manuskript-section-summary = Yhteenveto
import-manuskript-section-meta = Metatiedot
import-manuskript-section-series = Sarja
import-manuskript-section-volume = Osa
import-manuskript-section-genre = Lajityyppi
import-manuskript-section-license = Lisenssi
import-manuskript-section-email = Sähköposti
# The snowflake ladder's five tellings, shortest first: one situation, one
# sentence, one paragraph, one page, the full summary.
import-manuskript-section-situation = Tilanne
import-manuskript-section-sentence = Yksi virke
import-manuskript-section-paragraph = Yksi kappale
import-manuskript-section-page = Yksi sivu
import-manuskript-section-full = Koko yhteenveto
import-manuskript-untitled-project = Tuotu Manuskript-projekti

## Plume import: the story-bible groups an old (0.3 and earlier) project
## implies without naming them, and the title of a project with no name.
## Written into the imported project once, in the interface language.
import-plume-group-characters = Henkilöt
import-plume-group-items = Esineet
import-plume-group-places = Paikat
import-plume-untitled-project = Tuotu Plume-projekti
