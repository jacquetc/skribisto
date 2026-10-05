# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

tags-preset-basic = Grunnleggende
tags-preset-scifi = Science fiction
tags-preset-fantasy = Fantasy
tags-preset-mystery = Krim og mysterier
tags-preset-historical = Historisk


tags-preset-needs-research = må undersøkes
tags-preset-continuity-check = sjekk kontinuitet
tags-preset-plot-point = vendepunkt

tags-preset-character = person
tags-preset-place = sted
tags-preset-item = gjenstand

tags-preset-vessel = fartøy
tags-preset-planet = planet
tags-preset-organization = organisasjon
tags-preset-creature = vesen
tags-preset-faction = fraksjon
tags-preset-artifact = artefakt
tags-preset-realm = rike
tags-preset-magic-system = magisystem
tags-preset-suspect = mistenkt
tags-preset-victim = offer
tags-preset-clue = ledetråd
tags-preset-red-herring = villspor
tags-preset-historical-figure = historisk person
tags-preset-source = kilde
tags-preset-period-detail = tidsdetalj

tags-pill-list = Stikkord
tags-pill-add = Legg til et stikkord
tags-none-yet = Ingen stikkord
tags-pick-list = Stikkord å velge mellom
tags-pill-remove = Fjern { $name }
tags-pill-filter-placeholder = Filtrer, eller gi et nytt stikkord et navn
tags-pill-no-match = Ingen stikkord samsvarer
tags-pill-create = Opprett «{ $name }»
tags-pill-create-failed = Kunne ikke opprette «{ $name }»
tags-pill-new-discoverable = Finn i teksten-stikkord
tags-pill-new-discoverable-hint = Elementer med dette stikkordet sammenlignes med teksten din for å fylle persongalleriet.

tags-alias-list = Også kjent som
tags-alias-add = Legg til et annet navn
tags-alias-remove = Fjern { $name }
tags-alias-placeholder = Et annet navn, og trykk Enter
tags-alias-hint = Navn dette forekommer under i teksten din, i tillegg til tittelen.
tags-alias-collision = { $name } har allerede dette navnet.

settings-page-tags = Stikkord
settings-tags-desc = Stikkord merker elementene i permen din. Et Finn i teksten-stikkord ber i tillegg Skribisto lete etter navnene til elementet i teksten din.
settings-tags-add = Legg til stikkord
settings-tags-add-placeholder = Navn på et nytt stikkord
settings-tags-added = La til «{ $name }»
settings-tags-duplicate = «{ $name }» finnes allerede
settings-tags-filter = Filtrer stikkord
settings-tags-count = { $n ->
    [one] 1 stikkord
   *[other] { $n } stikkord
}
settings-tags-details-placeholder = Hva dette stikkordet betyr
settings-tags-discoverable = Finn i teksten
settings-tags-creates-in = Nye notater havner i
settings-tags-creates-in-unset = Spør meg første gang
settings-tags-creates-in-untitled = Mappe uten tittel
settings-tags-creates-in-trashed = { $name } (i papirkurven)
settings-tags-template = Startmal
settings-tags-template-unset = Tomt notat
settings-tags-delete = Slett { $name }
settings-tags-deleted = Slettet «{ $name }» og fjernet det fra alle elementer
settings-tags-empty = Ingen stikkord ennå.
settings-tags-apply-preset = Bruk en forhåndsinnstilling…
settings-tags-preset-applied = La til { $added }, hoppet over { $skipped } som fantes fra før
settings-tags-csv-filter = CSV-filer
settings-tags-import = Importer…
settings-tags-export = Eksporter…
settings-tags-imported = Importerte { $added }, hoppet over { $skipped }
settings-tags-exported = Eksporterte { $n ->
    [one] 1 stikkord
   *[other] { $n } stikkord
}

tags-chip-more = { $n ->
    [one] 1 stikkord til
   *[other] { $n } stikkord til
}

cast-section = Persongalleri
cast-add = Legg til i persongalleriet…
cast-add-filter-placeholder = Filtrer historiebibelen…
cast-add-empty = Ingen oppføringer i historiebibelen å legge til
cast-pin = Legg til { $name } i persongalleriet
cast-unpin = Fjern { $name } fra persongalleriet
cast-empty = Ingen er festet ennå. Legg til noen, eller behold et forslag
cast-unresolved = Finnes ikke lenger i historiebibelen

mentions-confirm = Legg til i persongalleriet for { $name }
mentions-hit-count = { $n ->
    [one] én gang
   *[other] { $n } ganger
}
mentions-point-of-view-badge = Synsvinkel
mentions-point-of-view-badge-tooltip = Angitt som synsvinkel her, satt for hånd, ikke funnet i teksten.

mentions-roster = Persongalleri
mentions-pin = Legg til { $name } i persongalleriet

pov-section = Synsvinkel
pov-empty = Ingen synsvinkel er satt
pov-add = Angi synsvinkel…
pov-multiple = Denne scenen har to synsvinkler.
pov-remove = Fjern { $name } som synsvinkel
pov-unresolved = En synsvinkel ble festet her, men den oppføringen finnes ikke lenger, eller har mistet historiebibel-stikkordet sitt.

books-section = Arkivert under
books-empty = Ikke arkivert under en bok ennå
books-add = Arkiver under en bok…
books-remove = Fjern fra { $name }
books-apply-to-children = Bruk arkiveringen på underelementer

note-details-name-placeholder = Navn…
note-details-tags = Stikkord
note-details-aliases = Andre navn
note-details-books = Arkivert under
note-details-books-empty = Ikke arkivert under en bok ennå
note-details-links = Lenker
note-details-links-empty = Ingen lenker ennå. Legg til en for å vise til en annen oppføring i historiebibelen din.
note-details-links-add = Lenk til et notat…
note-details-pov = Synsvinkel
note-details-pov-empty = Ingen synsvinkel er satt
note-details-pov-unresolved = En synsvinkel ble festet her, men den oppføringen finnes ikke lenger, eller har mistet historiebibel-stikkordet sitt.
note-details-pov-multiple = Dette notatet har to synsvinkler.
note-details-backlinks = Forekommer i manuset
note-details-backlinks-empty = Ingenting ennå. Når dette navnet forekommer i teksten din, vises det her.
note-details-backlinks-outside = Utenfor bøkene
note-details-backlinks-confirm-all = Bekreft alle forekomster
note-details-backlinks-confirm-all-tooltip = Legg til denne oppføringen i persongalleriet til alle dokumentene som vises her og ennå ikke har bekreftet den
note-details-untitled-document = Uten tittel

note-in-prose-mentions = { $n ->
    [one] 1 omtale
   *[other] { $n } omtaler
}
note-in-prose-mention-at = { $current } av { $total }
note-in-prose-mention-previous = Forrige omtale
note-in-prose-mention-next = Neste omtale
