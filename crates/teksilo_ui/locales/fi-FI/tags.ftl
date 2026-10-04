# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

tags-preset-basic = Perus
tags-preset-scifi = Tiedefiktio
tags-preset-fantasy = Fantasia
tags-preset-mystery = Mysteeri
tags-preset-historical = Historia


tags-preset-needs-research = tarve tutkimukseen
tags-preset-continuity-check = Jatkuvuustarkastus
tags-preset-plot-point = tonttipiste

tags-preset-character = merkki
tags-preset-place = paikka
tags-preset-item = kohta

tags-preset-vessel = alus
tags-preset-planet = planeetta
tags-preset-organization = organisaatio
tags-preset-creature = olento
tags-preset-faction = Ryhmä
tags-preset-artifact = artefakti
tags-preset-realm = valtakunta
tags-preset-magic-system = taikajärjestelmä
tags-preset-suspect = epäilty
tags-preset-victim = uhri
tags-preset-clue = Johtolanka
tags-preset-red-herring = punasilakka
tags-preset-historical-figure = historiallinen luku
tags-preset-source = lähde
tags-preset-period-detail = jakso yksityiskohtaiset tiedot

tags-pill-list = Tunnisteet
tags-pill-add = Lisää tagi
tags-none-yet = Ei tunnisteita
tags-pick-list = Valittavat tunnisteet
tags-pill-remove = Poista { $name }
tags-pill-filter-placeholder = Suodata tai nimeä uusi tagi
tags-pill-no-match = Ei tunnisteiden osumia
tags-pill-create = Luo "{ $name }"
tags-pill-create-failed = Ei voitu luoda "{ $name }"
tags-pill-new-discoverable = Etsi-in-prose tagi
tags-pill-new-discoverable-hint = Tämän tunnisteen kohteet täsmäävät proosaasi vastaan täyttääkseen listan.

tags-alias-list = Muut nimet
tags-alias-add = Lisää toinen nimi
tags-alias-remove = Poista { $name }
tags-alias-placeholder = Toinen nimi, sitten kirjoita
tags-alias-hint = Nimet näkyvät proosassasi sen nimen vieressä.
tags-alias-collision = { $name } vastaa jo tähän nimeen.

settings-page-tags = Tunnisteet
settings-tags-desc = Tunnisteet etiketti kohteita kansiossa. Löytää-in-prose tag kertoo myös Skribisto etsiä kyseisen tuotteen nimet proosa.
settings-tags-add = Lisää tunniste
settings-tags-add-placeholder = Nimeä uusi tunniste
settings-tags-added = Lisätty "{ $name }"
settings-tags-duplicate = "{ $name }" on jo olemassa
settings-tags-filter = Suodatintunnisteet
settings-tags-count = { $n ->
    [one] 1 tag
   *[other] { $n } tagit
}
settings-tags-details-placeholder = Mitä tämä merkki tarkoittaa?
settings-tags-discoverable = Etsi tekstistä
settings-tags-creates-in = Uudet muistiinpanot
settings-tags-creates-in-unset = Kysy minulta ensimmäisellä kerralla.
settings-tags-creates-in-untitled = Nimeämätön kansio
settings-tags-creates-in-trashed = { $name } (roskassa)
settings-tags-template = Alkumalli
settings-tags-template-unset = Tyhjä viesti
settings-tags-delete = Poistetaan kohta { $name }.
settings-tags-deleted = Poistanut "{ $name }" ja poistanut sen jokaisesta kohdasta
settings-tags-empty = Ei vielä tunnisteita.
settings-tags-apply-preset = Käytä esiaseteltua...
settings-tags-preset-applied = Lisätty { $added }, ohitettu { $skipped } jo läsnä
settings-tags-csv-filter = CSV tiedostoa
settings-tags-import = Tuo...
settings-tags-export = Vie...
settings-tags-imported = Tuotu { $added }, ohitettu { $skipped }
settings-tags-exported = Viety { $n ->
    [one] 1 tag
   *[other] { $n } tagit
}

tags-chip-more = { $n ->
    [one] 1 tag lisää
   *[other] { $n } lisää tunnisteita
}

cast-section = Kohtauksen hahmot
cast-add = Lisää kipsiin...
cast-add-filter-placeholder = Suodata tarinan raamattu...
cast-add-empty = Ei lisättävää tarinan raamattua
cast-pin = Lisätään { $name } valukseen
cast-unpin = Poista { $name } valuksesta
cast-empty = Kukaan ei ole vielä jumissa.
cast-unresolved = Ei enää Raamatussa.

mentions-confirm = Lisää { $name } valukseen
mentions-hit-count = { $n ->
    [one] kerran
   *[other] { $n } kertaa
}
mentions-point-of-view-badge = Näkökulma
mentions-point-of-view-badge-tooltip = Näkemyksenä tässä, käsin asetettuna, ei havaittu proosa.

mentions-roster = Kohtauksen hahmot
mentions-pin = Lisätään { $name } valukseen

pov-section = Näkökulma
pov-empty = Näkökulmaa ei ole asetettu
pov-add = Aseta näkökulma...
pov-multiple = Tässä kohtauksessa on kaksi näkökulmaa.
pov-remove = Poista { $name } näkökulmasta
pov-unresolved = Näkymä jäi tähän, mutta sitä kohtaa ei ole enää olemassa tai se on menettänyt tarina-raamattunsa.

books-section = Liitetty kirjaan
books-empty = Ei ole vielä arkistoitu kirjan alle
books-add = Tiedosto kirjan alla...
books-remove = Poistetaan kohdasta { $name }
books-apply-to-children = Hae arkistointia lapsille

note-details-name-placeholder = Nimi…
note-details-tags = Tunnisteet
note-details-aliases = Muut nimet
note-details-books = Liitetty kirjaan
note-details-books-empty = Ei vielä arkistoitu kirjan alle
note-details-links = Linkit
note-details-links-empty = Ei linkkejä vielä. Lisää yksi piste toiseen kohtaan tarinasi Raamatussa.
note-details-links-add = Linkki viestiin...
note-details-pov = Näkökulma
note-details-pov-empty = Näkökulmaa ei ole asetettu
note-details-pov-unresolved = Näkymä jäi tähän, mutta sitä kohtaa ei ole enää olemassa tai se on menettänyt tarina-raamattunsa.
note-details-pov-multiple = Tässä huomautuksessa on kaksi näkökulmaa.
note-details-backlinks = Näyttää käsikirjoituksessa
note-details-backlinks-empty = Kun tämä nimi ilmestyy proosaasi, se ilmestyy tänne.
note-details-backlinks-outside = Kirjojen ulkopuolella.
note-details-backlinks-confirm-all = Vahvista jokainen ulkonäkö
note-details-backlinks-confirm-all-tooltip = Lisää tämä kohta jokaisen tässä esitetyn asiakirjan kipsiin, joka ei ole vielä vahvistanut sitä
note-details-untitled-document = Nimetön

note-in-prose-mentions = { $n ->
    [one] 1 maininta
   *[other] { $n }
}
note-in-prose-mention-at = { $current }, { $total }
note-in-prose-mention-previous = Edellinen maininta
note-in-prose-mention-next = Seuraava maininta
