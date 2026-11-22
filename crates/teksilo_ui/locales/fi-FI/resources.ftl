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

## Kansion Resurssit-sivu

segment-folder = Kansio
resources-folder-count = { $count ->
    [one] Yksi resurssi
   *[other] { $count } resurssia
}
resources-view-label = Näytä resurssit
resources-view-list = Luettelona
resources-view-grid = Ruudukkona
resources-grid-label = Resurssit
resources-folder-drop-hint = Pudota tiedostoja tai verkko-osoitteita tähän liittääksesi ne tähän kansioon
resources-folder-empty = Tähän kansioon tai sen sisältöön ei ole vielä liitetty mitään. Lisää tiedostoja tai verkkolinkkejä yllä olevilla painikkeilla tai pudota ne tähän: ne liitetään kansioon.
resources-attached-to = Kohteissa
resources-attached-separator = ,
resources-attached-more = { $count ->
    [one] ja yksi muu
   *[other] ja { $count } muuta
}
resources-row-name-attached = { $name }, kohteissa { $items }
resources-open-item = Avaa ”{ $title }”
resources-detach-from = Irrota kohteesta ”{ $title }”

## Kuvaresurssit Galleriassa

gallery-resource-badge = Resurssi
gallery-tile-resource = Kohteen { $title } kuvaresurssi: { $description }

## Projektin asetusten Resurssit-sivu

settings-page-resources = Resurssit
settings-desc-resources = Tiedostot ja verkkolinkit, joita mikään kohde ei enää kanna: liitä ne uudelleen, poista ne tai vie niistä kopioita
settings-resources-description = Resurssi ei ole liitettynä mihinkään, kun se on irrotettu viimeisestä kohteestaan tai kun kohde, jossa se oli, on poistettu lopullisesti. Se pysyy projektissa, ja tallennettu tiedosto kulkee projektin jokaisen kopion ja jokaisen varmuuskopion mukana, kunnes liität sen uudelleen tai poistat sen.
settings-resources-group-storage = Tallennustila
settings-resources-group-unattached = Ei liitetty mihinkään
settings-resources-stored = Tähän projektiin tallennetut tiedostot: { $size }
settings-resources-folder-advice = Yhtenä tiedostona pidetty projekti kirjoitetaan kokonaan uudelleen joka tallennuksella, myös sen tallennetut tiedostot. Kansiona pidetty tallentaa vain sen, mikä on muuttunut, ja se on nopeampaa.
settings-resources-count = { $count ->
    [one] Yksi resurssi
   *[other] { $count } resurssia
}
settings-resources-empty = Jokainen tämän projektin resurssi on liitetty johonkin kohteeseen.
resources-attach-to-items = Liitä kohteisiin…
resources-export-copies = Vie kopiot…
resources-export-copy = Vie kopio…
resources-remove-many-title = Poistetaanko { $count } resurssia projektista?
resources-attach-title = Liitä kohteisiin
resources-attach-hint = Valitse kohteet, joihin liitetään. Ctrl+napsautus tai Vaihto+napsautus valitsee useita.
resources-attach-empty = Tässä projektissa ei ole vielä kohteita.
resources-attach-confirm = Liitä
resources-attached = { $count ->
    [one] Yksi resurssi liitetty
   *[other] { $count } resurssia liitetty
} { $items ->
    [one] yhteen kohteeseen
   *[other] { $items } kohteeseen
}
resources-export-title = Vie kopiot kansioon
resources-exported = { $count ->
    [one] Yksi resurssi viety
   *[other] { $count } resurssia viety
}
resources-export-missing = { $count ->
    [one] Yksi tiedosto ei ole siellä, missä projekti sen odottaa olevan, ja se jätettiin pois: { $names }
   *[other] { $count } tiedostoa ei ole siellä, missä projekti niiden odottaa olevan, ja ne jätettiin pois: { $names }
}
resources-export-failed = { $count ->
    [one] Yhtä tiedostoa ei voitu kopioida: { $names }
   *[other] { $count } tiedostoa ei voitu kopioida: { $names }
}

## Suurten tiedostojen asetus (Editorin toiminta)

settings-resources-large-file = Yli 10 Mt:n tiedostot
settings-resources-large-ask = Kysy joka kerta
settings-resources-large-store = Tallenna aina projektiin
settings-resources-large-link = Linkitä aina tiedostoon
settings-hint-resources-large-file = Mitä tapahtuu yli 10 Mt:n tiedostolle, jonka liität resurssiksi. Projektiin tallennettuna se kulkee projektin jokaisen kopion ja jokaisen varmuuskopion mukana. Linkitettynä se pysyy paikallaan, ja projekti muistaa vain, mistä sen löytää.
