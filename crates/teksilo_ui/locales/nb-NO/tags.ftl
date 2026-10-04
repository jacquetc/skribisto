# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

tags-preset-basic = Grunnleggende
tags-preset-scifi = Science fiction
tags-preset-fantasy = Fantasy
tags-preset-mystery = Mysterium
tags-preset-historical = Historisk


tags-preset-needs-research = trenger forskning
tags-preset-continuity-check = Kontinuitetskontroll
tags-preset-plot-point = plottepunkt

tags-preset-character = tegn
tags-preset-place = sted
tags-preset-item = varen

tags-preset-vessel = fartøy
tags-preset-planet = planet
tags-preset-organization = organisasjon
tags-preset-creature = vesen
tags-preset-faction = Fraksjon
tags-preset-artifact = Artefact
tags-preset-realm = riket
tags-preset-magic-system = Magisk system
tags-preset-suspect = mistenkelig
tags-preset-victim = offer
tags-preset-clue = ledetråd
tags-preset-red-herring = rød sild
tags-preset-historical-figure = historiske tall
tags-preset-source = kilde
tags-preset-period-detail = Periodedetaljer

tags-pill-list = Etiketter
tags-pill-add = Legg til et tag
tags-none-yet = Ingen etiketter
tags-pick-list = Tags å velge fra
tags-pill-remove = Fjerne { $name }
tags-pill-filter-placeholder = Filtrer eller navngi en ny tag
tags-pill-no-match = Ingen tag matches
tags-pill-create = Opprette -{ $name }"
tags-pill-create-failed = Kunne ikke opprette -{ $name }"
tags-pill-new-discoverable = Finn-in-prose tag
tags-pill-new-discoverable-hint = Elementer med denne etiketten er matchet mot prosaen din for å fylle rosteren.

tags-alias-list = Også kjent som
tags-alias-add = Legg til et annet navn
tags-alias-remove = Fjerne { $name }
tags-alias-placeholder = Et annet navn, deretter Skriv inn
tags-alias-hint = Navnene dette vises under i prosaen din, ved siden av sin tittel.
tags-alias-collision = { $name } svarer allerede på dette navnet.

settings-page-tags = Etiketter
settings-tags-desc = Tags etikett elementene i bindemiddel. En Find-in-prose tag forteller også Skribisto å se etter det elementets navn i prosaen.
settings-tags-add = Legg til tag
settings-tags-add-placeholder = Navn på en ny tag
settings-tags-added = Lagt til{ $name }"
settings-tags-duplicate = "{ $name }- allerede eksisterer
settings-tags-filter = Filtrer tagger
settings-tags-count = { $n ->
    [one] 1 tags
   *[other] { $n } tags
}
settings-tags-details-placeholder = Hva dette merket betyr
settings-tags-discoverable = Finn i prosa
settings-tags-creates-in = Nye notater går til
settings-tags-creates-in-unset = Spør meg første gang
settings-tags-creates-in-untitled = Mappe uten navn
settings-tags-creates-in-trashed = { $name } (i søppel)
settings-tags-template = Starter mal
settings-tags-template-unset = Tomt notat
settings-tags-delete = Slett { $name }
settings-tags-deleted = Slettet{ $name }- og fjernet det fra alle elementer
settings-tags-empty = Ingen etiketter ennå.
settings-tags-apply-preset = Bruk en forhåndsinnstilling...
settings-tags-preset-applied = Lagt { $added }, hoppet { $skipped } allerede tilstede
settings-tags-csv-filter = CSV filer
settings-tags-import = Importer…
settings-tags-export = Eksporter…
settings-tags-imported = Importert { $added }, hoppet { $skipped }
settings-tags-exported = Eksportert { $n ->
    [one] 1 tags
   *[other] { $n } tags
}

tags-chip-more = { $n ->
    [one] 1 tag til
   *[other] { $n } flere etiketter
}

cast-section = Persongalleri
cast-add = Legg til i...
cast-add-filter-placeholder = Filtrer historie bibel...
cast-add-empty = Ingen store bibeloppføringer å legge til
cast-pin = Legg { $name } til støpe
cast-unpin = Fjern { $name } fra støpe
cast-empty = Ingen fast ennå. Legg til eller hold et forslag
cast-unresolved = Ikke lenger i historien Bibelen

mentions-confirm = Legg til i støp av { $name }
mentions-hit-count = { $n ->
    [one] én gang
   *[other] { $n } ganger
}
mentions-point-of-view-badge = Synsvinkel
mentions-point-of-view-badge-tooltip = Deklarert som synspunktet her, satt for hånd, ikke detektert i prosen.

mentions-roster = Persongalleri
mentions-pin = Legg { $name } til støpe

pov-section = Synsvinkel
pov-empty = Ingen synspunkt sett
pov-add = Sette synspunkt...
pov-multiple = Denne scenen har to synspunkter.
pov-remove = Fjern { $name } som synspunkt
pov-unresolved = Et synspunkt ble festet her, men den oppføringen eksisterer ikke lenger eller mistet sin historie-bibel tag.

books-section = Arkivert under
books-empty = Ikke skrevet under en bok ennå
books-add = Fil under en bok...
books-remove = Fjern fra { $name }
books-apply-to-children = Søk om arkivering til barn

note-details-name-placeholder = Navn…
note-details-tags = Etiketter
note-details-aliases = Andre navn
note-details-books = Arkivert under
note-details-books-empty = Ikke skrevet under en bok ennå
note-details-links = Lenker
note-details-links-empty = Ingen koblinger ennå. Legg til en til punkt på en annen oppføring i historien Bibelen.
note-details-links-add = Lenke til et notat...
note-details-pov = Synsvinkel
note-details-pov-empty = Ingen synspunkt sett
note-details-pov-unresolved = Et synspunkt ble festet her, men den oppføringen eksisterer ikke lenger eller mistet sin historie-bibel tag.
note-details-pov-multiple = Denne noten har to synspunkter.
note-details-backlinks = Viser seg i manuskriptet
note-details-backlinks-empty = Når dette navnet vises i prosaen din, vises det her.
note-details-backlinks-outside = Utenfor bøkene
note-details-backlinks-confirm-all = Bekreft hvert utseende
note-details-backlinks-confirm-all-tooltip = Legg til denne oppføringen i støpingen av hvert dokument vist her som ikke har bekreftet det ennå
note-details-untitled-document = Uten navn

note-in-prose-mentions = { $n ->
    [one] 1 omtale
   *[other] { $n } nevner
}
note-in-prose-mention-at = { $current } av { $total }
note-in-prose-mention-previous = Forrige omtale
note-in-prose-mention-next = Neste omtale
