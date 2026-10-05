# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

tags-preset-basic = Perus
tags-preset-scifi = Tiedefiktio
tags-preset-fantasy = Fantasia
tags-preset-mystery = Mysteeri
tags-preset-historical = Historiallinen


tags-preset-needs-research = vaatii taustatyötä
tags-preset-continuity-check = jatkuvuuden tarkistus
tags-preset-plot-point = käännekohta

tags-preset-character = henkilö
tags-preset-place = paikka
tags-preset-item = esine

tags-preset-vessel = alus
tags-preset-planet = planeetta
tags-preset-organization = organisaatio
tags-preset-creature = olento
tags-preset-faction = ryhmittymä
tags-preset-artifact = artefakti
tags-preset-realm = valtakunta
tags-preset-magic-system = taikajärjestelmä
tags-preset-suspect = epäilty
tags-preset-victim = uhri
tags-preset-clue = johtolanka
tags-preset-red-herring = harhautus
tags-preset-historical-figure = historiallinen henkilö
tags-preset-source = lähde
tags-preset-period-detail = ajankuvan yksityiskohta

tags-pill-list = Tunnisteet
tags-pill-add = Lisää tunniste
tags-none-yet = Ei tunnisteita
tags-pick-list = Valittavat tunnisteet
tags-pill-remove = Poista { $name }
tags-pill-filter-placeholder = Suodata tai nimeä uusi tunniste
tags-pill-no-match = Mikään tunniste ei vastaa hakua
tags-pill-create = Luo ”{ $name }”
tags-pill-create-failed = Tunnistetta ”{ $name }” ei voitu luoda
tags-pill-new-discoverable = Etsi tekstistä -tunniste
tags-pill-new-discoverable-hint = Tämän tunnisteen kohteita haetaan tekstistäsi esiintyjäluettelon täyttämiseksi.

tags-alias-list = Muut nimet
tags-alias-add = Lisää toinen nimi
tags-alias-remove = Poista { $name }
tags-alias-placeholder = Toinen nimi ja sitten Enter
tags-alias-hint = Nimet, joilla tämä esiintyy tekstissäsi otsikkonsa lisäksi.
tags-alias-collision = { $name } tunnetaan jo tällä nimellä.

settings-page-tags = Tunnisteet
settings-tags-desc = Tunnisteet merkitsevät mappisi kohteita. Etsi tekstistä -tunniste kertoo lisäksi Skribistolle, että kohteen nimiä pitää etsiä tekstistäsi.
settings-tags-add = Lisää tunniste
settings-tags-add-placeholder = Nimeä uusi tunniste
settings-tags-added = Lisätty ”{ $name }”
settings-tags-duplicate = ”{ $name }” on jo olemassa
settings-tags-filter = Suodata tunnisteita
settings-tags-count = { $n ->
       [one] 1 tunniste
      *[other] { $n } tunnistetta
   }
settings-tags-details-placeholder = Mitä tämä tunniste tarkoittaa
settings-tags-discoverable = Etsi tekstistä
settings-tags-creates-in = Uudet muistiinpanot sijoitetaan
settings-tags-creates-in-unset = Kysy ensimmäisellä kerralla
settings-tags-creates-in-untitled = Nimeämätön kansio
settings-tags-creates-in-trashed = { $name } (roskakorissa)
settings-tags-template = Aloitusmalli
settings-tags-template-unset = Tyhjä muistiinpano
settings-tags-delete = Poista { $name }
settings-tags-deleted = ”{ $name }” poistettu ja irrotettu kaikista kohteista
settings-tags-empty = Ei vielä tunnisteita.
settings-tags-apply-preset = Käytä valmista valikoimaa…
settings-tags-preset-applied = Lisätty { $added }, ohitettu { $skipped } jo olemassa olevaa
settings-tags-csv-filter = CSV-tiedostot
settings-tags-import = Tuo…
settings-tags-export = Vie…
settings-tags-imported = Tuotiin { $added }, ohitettiin { $skipped }
settings-tags-exported = Vietiin { $n ->
       [one] 1 tunniste
      *[other] { $n } tunnistetta
   }

tags-chip-more = { $n ->
       [one] 1 tunniste lisää
      *[other] { $n } tunnistetta lisää
   }

cast-section = Esiintyjät
cast-add = Lisää esiintyjiin…
cast-add-filter-placeholder = Suodata tietopankkia…
cast-add-empty = Ei lisättäviä tietopankin merkintöjä
cast-pin = Lisää { $name } esiintyjiin
cast-unpin = Poista { $name } esiintyjistä
cast-empty = Ketään ei ole vielä lisätty. Lisää esiintyjä tai hyväksy ehdotus
cast-unresolved = Ei enää tarinan tietopankissa

mentions-confirm = Lisää kohteen { $name } esiintyjiin
mentions-hit-count = { $n ->
    [one] kerran
   *[other] { $n } kertaa
}
mentions-point-of-view-badge = Näkökulma
mentions-point-of-view-badge-tooltip = Merkitty tässä näkökulmaksi käsin, ei tunnistettu tekstistä.

mentions-roster = Esiintyjät
mentions-pin = Lisää { $name } esiintyjiin

pov-section = Näkökulma
pov-empty = Näkökulmaa ei ole asetettu
pov-add = Aseta näkökulma…
pov-multiple = Tässä kohtauksessa on kaksi näkökulmaa.
pov-remove = Poista { $name } näkökulmista
pov-unresolved = Tähän oli kiinnitetty näkökulma, mutta kyseistä merkintää ei enää ole tai se on menettänyt tietopankin tunnisteensa.

books-section = Liitetty kirjaan
books-empty = Ei vielä liitetty mihinkään kirjaan
books-add = Liitä kirjaan…
books-remove = Poista kirjasta { $name }
books-apply-to-children = Käytä liitoksia alakohteisiin

note-details-name-placeholder = Nimi…
note-details-tags = Tunnisteet
note-details-aliases = Muut nimet
note-details-books = Liitetty kirjaan
note-details-books-empty = Ei vielä liitetty mihinkään kirjaan
note-details-links = Linkit
note-details-links-empty = Ei vielä linkkejä. Lisää linkki toiseen tarinan tietopankin merkintään.
note-details-links-add = Linkitä muistiinpanoon…
note-details-pov = Näkökulma
note-details-pov-empty = Näkökulmaa ei ole asetettu
note-details-pov-unresolved = Tähän oli kiinnitetty näkökulma, mutta kyseistä merkintää ei enää ole tai se on menettänyt tietopankin tunnisteensa.
note-details-pov-multiple = Tällä muistiinpanolla on kaksi näkökulmaa.
note-details-backlinks = Esiintyy käsikirjoituksessa
note-details-backlinks-empty = Ei vielä mitään. Kun tämä nimi esiintyy tekstissäsi, se näkyy tässä.
note-details-backlinks-outside = Kirjojen ulkopuolella
note-details-backlinks-confirm-all = Vahvista kaikki esiintymät
note-details-backlinks-confirm-all-tooltip = Lisää tämä merkintä kaikkien tässä näkyvien asiakirjojen esiintyjiin, joissa sitä ei ole vielä vahvistettu
note-details-untitled-document = Nimetön

note-in-prose-mentions = { $n ->
       [one] 1 maininta
      *[other] { $n } mainintaa
   }
note-in-prose-mention-at = { $current }/{ $total }
note-in-prose-mention-previous = Edellinen maininta
note-in-prose-mention-next = Seuraava maininta
