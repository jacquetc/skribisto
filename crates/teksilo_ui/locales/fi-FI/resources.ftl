# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

# Resurssit: mapin kohteisiin liitetyt tiedostot ja verkkolinkit.

## Paneeli

resources-title = Resurssit
shortcut-name-resources-show = Näytä resurssit
resources-add-files = Lisää tiedostoja
resources-add-files-title = Lisää tiedostoja tähän kohteeseen
resources-add-link = Lisää verkkolinkki
resources-drop-hint = Pudota tiedostoja tai verkko-osoitteita tähän liittääksesi ne
resources-no-item = Avaa kohde nähdäksesi sen resurssit.
resources-empty = Tähän kohteeseen ei ole vielä liitetty mitään. Lisää tiedostoja tai verkkolinkkejä yllä olevilla painikkeilla tai pudota ne tähän.
resources-actions = Toiminnot
resources-not-found = Ei löydy
resources-row-linked = Linkitetty tiedosto
resources-row-linked-size = Linkitetty tiedosto, { $size }
resources-row-name = { $title }, { $kind }, { $detail }
resources-row-name-missing = { $title }, { $kind }, { $detail }, ei löydy

## Tiedostotyypit niin kuin ruudunlukija ne lukee

resources-kind-picture = kuva
resources-kind-pdf = PDF-asiakirja
resources-kind-audio = äänitallenne
resources-kind-video = video
resources-kind-document = asiakirja
resources-kind-ebook = e-kirja
resources-kind-archive = arkisto
resources-kind-file = tiedosto
resources-kind-web-link = verkkolinkki

## Rivin valikko

resources-open = Avaa
resources-show-in-folder = Näytä kansiossa
resources-locate = Etsi uudelleen…
resources-insert-into-text = Lisää tekstiin
resources-insert-picture = Lisää kuva tekstiin
resources-insert-link = Lisää linkki tekstiin
resources-edit = Nimeä uudelleen ja kuvaile…
resources-detach = Irrota tästä kohteesta
resources-remove = Poista projektista…

## Kaksi valintaikkunaa

resources-link-dialog-title = Lisää verkkolinkki
resources-link-dialog-address = Osoite
resources-link-dialog-address-placeholder = https://
resources-link-dialog-name = Otsikko
resources-link-dialog-name-placeholder = Sivuston nimi, jos jätät tämän tyhjäksi
resources-link-dialog-add = Lisää
resources-edit-dialog-title = Nimeä uudelleen ja kuvaile
resources-edit-dialog-name = Otsikko
resources-edit-dialog-description = Kuvaus
resources-edit-dialog-description-placeholder = Mikä tämä on ja miksi säilytit sen
resources-edit-dialog-save = Tallenna
resources-dialog-cancel = Peruuta

## Lisääminen

resources-large-title = Suuri tiedosto
resources-large-text = ”{ $name }” on kooltaan { $size }. Projektiin tallennettuna se kulkee jokaisen projektin kopion ja varmuuskopion mukana. Linkitettynä se pysyy paikallaan, ja projekti muistaa vain, mistä sen löytää.
resources-large-text-many = ”{ $name }” on kooltaan { $size }, ja { $others ->
    [one] myös yksi muu tiedosto on suuri
   *[other] myös { $others } muuta tiedostoa on suuria
}. Projektiin tallennettuina ne kulkevat jokaisen projektin kopion ja varmuuskopion mukana. Linkitettyinä ne pysyvät paikallaan, ja projekti muistaa vain, mistä ne löytää.
resources-large-store = Tallenna se projektiin
resources-large-link = Linkitä se mieluummin
resources-large-remember = Älä kysy uudelleen
resources-storing-title = Tiedostoja lisätään projektiin
resources-store-cancel = Peruuta
resources-store-cancelled = Tiedostoja ei lisätty.
resources-store-busy = Tiedostoja lisätään vielä. Yritä uudelleen, kun ne ovat valmiina.
resources-store-failed = Tiedostoja ei voitu lisätä: { $error }
resources-no-project = Avaa projekti ennen tiedostojen liittämistä.
resources-moved-away = Projekti siirtyi, kun tiedostoa kopioitiin.
resources-some-failed = { $count ->
    [one] Yhtä tiedostoa ei voitu lisätä: { $names }
   *[other] { $count } tiedostoa ei voitu lisätä: { $names }
}
resources-add-failed = Resursseja ei voitu lisätä: { $error }
resources-added = { $count ->
    [one] Yksi resurssi lisätty
   *[other] { $count } resurssia lisätty
}
resources-dropped-added = { $count ->
    [one] Lisätty tämän kohteen resurssiksi ja linkitetty tekstiin
   *[other] { $count } tiedostoa lisätty tämän kohteen resursseiksi ja linkitetty tekstiin
}
resources-undo = Kumoa
resources-link-refused = ”{ $url }” ei ole verkko-osoite, jonka Skribisto voi säilyttää. Käytä osoitetta, joka alkaa http:// tai https://, ilman käyttäjänimeä tai salasanaa.

## Ilmoitus yhtenä tiedostona säilytetystä projektista

resources-folder-title = Tämä projekti kasvaa suureksi
resources-folder-text = Sen tallennetut tiedostot vievät nyt { $size }. Yhtenä tiedostona säilytetty projekti kirjoitetaan kokonaan uudelleen jokaisella tallennuksella, myös sen tiedostot. Kansiona säilytetty projekti tallentaa vain sen, mikä on muuttunut. Voit tallentaa sen nyt kansiona ja jatkaa siitä lähtien työtä kansiossa.
resources-folder-switch = Tallenna kansiona…
resources-folder-not-now = Ei nyt
resources-folder-never = Älä kysy uudelleen tästä projektista

## Avaaminen

resources-not-found-toast = ”{ $name }” ei ole siellä, missä projekti odottaa sen olevan.
resources-unsafe-title = Avataanko ”{ $name }”?
resources-unsafe-text = Skribisto ei tiedä, onko tämäntyyppinen tiedosto turvallinen. Sen avaaminen voisi käynnistää ohjelman tietokoneellasi, ja projekti voi olla peräisin joltakulta muulta. Näytä se ensin kansiossaan nähdäksesi, mikä se on.
resources-open-anyway = Avaa silti
resources-open-failed = Tiedostoa ei voitu avata: { $error }
resources-no-data-folder = tämä järjestelmä ei anna Skribistolle datakansiota, johon kopion voisi avata
resources-link-gone = Tämä linkki osoittaa resurssiin, jota projektissa ei enää ole.
resources-locate-title = Missä ”{ $name }” on nyt?
resources-relinked-more = { $count ->
    [one] Samasta kansiosta löytyi vielä yksi puuttuva tiedosto, ja se linkitettiin uudelleen.
   *[other] Samasta kansiosta löytyi vielä { $count } puuttuvaa tiedostoa, ja ne linkitettiin uudelleen.
}
resources-changed-elsewhere = ”{ $name }” muuttui toisessa sovelluksessa. Projekti säilyttää uuden version.

## Muokkaaminen

resources-edit-failed = Resurssia ei voitu muuttaa: { $error }
resources-remove-title = Poistetaanko ”{ $title }” projektista?
resources-remove-text = Se irrotetaan jokaisesta kohteesta, johon se on liitetty. Projektiin tallennettu tiedosto lähtee sen mukana; linkitetty tiedosto ja verkkosivu pysyvät paikallaan. Kumoa tuo sen takaisin.
resources-remove-confirm = Poista
resources-remove-failed = Resurssia ei voitu poistaa: { $error }

## Tekstiin

resources-no-text = Vie ensin kohdistin tekstiin ja lisää resurssi sitten siihen.
resources-drop-insert-picture = Lisää kuva
resources-drop-insert-link = Lisää linkki resurssiin

## Edistyminen

progress-resources-storing = Tiedostoja kopioidaan
progress-resources-stored = Tiedostot kopioitu
