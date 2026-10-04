# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

tags-preset-basic = Grundlæggende
tags-preset-scifi = Videnskabelig fiktion
tags-preset-fantasy = Fantasy
tags-preset-mystery = Krimi
tags-preset-historical = Historisk


tags-preset-needs-research = behov for forskning
tags-preset-continuity-check = Kontinuitetskontrol
tags-preset-plot-point = plottet punkt

tags-preset-character = tegn
tags-preset-place = sted
tags-preset-item = post

tags-preset-vessel = fartøj
tags-preset-planet = planet
tags-preset-organization = organisation
tags-preset-creature = væsen
tags-preset-faction = fraktion
tags-preset-artifact = artefakt
tags-preset-realm = rige
tags-preset-magic-system = Magisk system
tags-preset-suspect = mistænkt
tags-preset-victim = offer
tags-preset-clue = ledetråd
tags-preset-red-herring = rødsild
tags-preset-historical-figure = historisk tal
tags-preset-source = kilde
tags-preset-period-detail = periodedetaljer

tags-pill-list = Mærker
tags-pill-add = Tilføj et mærke
tags-none-yet = Ingen mærker
tags-pick-list = Mærker at vælge fra
tags-pill-remove = Fjern { $name }
tags-pill-filter-placeholder = Filter eller navngiv et nyt mærke
tags-pill-no-match = Ingen mærkekampe
tags-pill-create = Opret { $name }
tags-pill-create-failed = Kunne ikke oprette { $name }
tags-pill-new-discoverable = Finde-in-prose tag
tags-pill-new-discoverable-hint = Elementer med dette mærke er matchet med din prosa til at udfylde listen.

tags-alias-list = Også kendt som
tags-alias-add = Tilføj et andet navn
tags-alias-remove = Fjern { $name }
tags-alias-placeholder = Et andet navn, derefter Enter
tags-alias-hint = Navne, der står i din prosa, ved siden af titlen.
tags-alias-collision = { $name } svarer allerede på dette navn.

settings-page-tags = Mærker
settings-tags-desc = Mærker mærke de elementer i dit bindemiddel. En find- in- prose tag også fortæller Skribisto til at kigge efter dette element bøjer navne i din prose.
settings-tags-add = Tilføj mærke
settings-tags-add-placeholder = Nævn et nyt mærke
settings-tags-added = Tilføjet: { $name }
settings-tags-duplicate = Der findes allerede { $name }
settings-tags-filter = Filtermærker
settings-tags-count = { $n ->
    [one] 1 mærke
   *[other] { $n } tags
}
settings-tags-details-placeholder = Hvad betyder dette mærke?
settings-tags-discoverable = Find i prose
settings-tags-creates-in = Nye noter går til
settings-tags-creates-in-unset = Spørg mig første gang
settings-tags-creates-in-untitled = Unavngivet mappe
settings-tags-creates-in-trashed = { $name } (affald)
settings-tags-template = Startskabelon
settings-tags-template-unset = Tom note
settings-tags-delete = Slet { $name }
settings-tags-deleted = Slettet { $name } og fjernet det fra hver post
settings-tags-empty = Ingen mærker endnu.
settings-tags-apply-preset = Påfør et forudindstillet
settings-tags-preset-applied = Tilføjet { $added }, sprunget { $skipped } allerede til stede
settings-tags-csv-filter = CSV filer
settings-tags-import = Importér…
settings-tags-export = Eksportér…
settings-tags-imported = Importeret { $added }, sprunget { $skipped }
settings-tags-exported = Eksporteret { $n ->
    [one] 1 mærke
   *[other] { $n } tags
}

tags-chip-more = { $n ->
    [one] 1 mere tag
   *[other] { $n } flere tags
}

cast-section = Persongalleri
cast-add = Tilføj til cast bench
cast-add-filter-placeholder = Filtrér historiebibel
cast-add-empty = Ingen historie- bible indgange at tilføje
cast-pin = Tilføj { $name } til cast
cast-unpin = Fjern { $name } fra støbt
cast-empty = Ingen sidder fast endnu. Tilføj eller behold et forslag
cast-unresolved = Ikke længere i historien bible

mentions-confirm = Tilføj til cast af { $name }
mentions-hit-count = { $n ->
    [one] én gang
   *[other] { $n } gange
}
mentions-point-of-view-badge = Synsvinkel
mentions-point-of-view-badge-tooltip = Kendt som synspunkt her, sat i hånden, ikke opdaget i prosen.

mentions-roster = Persongalleri
mentions-pin = Tilføj { $name } til cast

pov-section = Synsvinkel
pov-empty = Intet synspunkt sæt
pov-add = Sæt synsvinklen
pov-multiple = Denne scene har to synspunkter.
pov-remove = Fjern { $name } som synspunkt
pov-unresolved = Et synspunkt blev fastlåst her, men denne indgang ikke længere eksisterer eller mistet sin story- bible tag.

books-section = Gemt under
books-empty = Ikke indgivet under en bog endnu
books-add = Fil under en bog
books-remove = Fjern fra { $name }
books-apply-to-children = Anvende ansøgning til børn

note-details-name-placeholder = Navn…
note-details-tags = Mærker
note-details-aliases = Andre navne
note-details-books = Gemt under
note-details-books-empty = Ikke gemt under en bog endnu
note-details-links = Links
note-details-links-empty = Ingen links endnu. Tilføj en til at pege på en anden post i din historie bibel.
note-details-links-add = Link til en note
note-details-pov = Synsvinkel
note-details-pov-empty = Intet synspunkt sæt
note-details-pov-unresolved = Et synspunkt blev fastlåst her, men denne indgang ikke længere eksisterer eller mistet sin story- bible tag.
note-details-pov-multiple = Dette notat har to synspunkter.
note-details-backlinks = Udseende i manuskriptet
note-details-backlinks-empty = Når dette navn kommer frem i din prosa, dukker det op her.
note-details-backlinks-outside = Uden for bøgerne
note-details-backlinks-confirm-all = Bekræft alle fremtoninger
note-details-backlinks-confirm-all-tooltip = Tilføj denne indgang til cast af hvert dokument vist her, der ikke har bekræftet det endnu
note-details-untitled-document = Uden titel

note-in-prose-mentions = { $n ->
    [one] 1 ciffer
   *[other] { $n } nævner
}
note-in-prose-mention-at = { $current } af { $total }
note-in-prose-mention-previous = Tidligere omtale
note-in-prose-mention-next = Næste ciffer
