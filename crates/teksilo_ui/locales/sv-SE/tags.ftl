# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

tags-preset-basic = Grundläggande
tags-preset-scifi = Vetenskap Fiktion
tags-preset-fantasy = Fantasi
tags-preset-mystery = Mysteriet
tags-preset-historical = Historiska


tags-preset-needs-research = Behov forskning
tags-preset-continuity-check = Kontinuitetskontroll
tags-preset-plot-point = Plot Point

tags-preset-character = karaktär karaktär karaktär
tags-preset-place = Plats
tags-preset-item = Föremål

tags-preset-vessel = fartyg
tags-preset-planet = planeten planet planet planet
tags-preset-organization = Organisation
tags-preset-creature = varelse
tags-preset-faction = Fraktion
tags-preset-artifact = artefakt
tags-preset-realm = Verkligheten
tags-preset-magic-system = Magiskt system
tags-preset-suspect = misstänkt misstänkt
tags-preset-victim = Offer
tags-preset-clue = Ledtråd
tags-preset-red-herring = Röd sill
tags-preset-historical-figure = Historisk figur
tags-preset-source = Källan
tags-preset-period-detail = perioddetalj

tags-pill-list = Taggar
tags-pill-add = Lägg till en tagga
tags-none-yet = Inga taggar
tags-pick-list = Taggar att välja från
tags-pill-remove = Ta bort { $name }
tags-pill-filter-placeholder = Filtrera eller namnge en ny tagga
tags-pill-no-match = Inga tag-matcher
tags-pill-create = Skapa "{ $name }"
tags-pill-create-failed = Kunde inte skapa "{ $name }"
tags-pill-new-discoverable = Tagg som hittas i texten
tags-pill-new-discoverable-hint = Objekt med denna tagg matchas mot din prosa för att fylla roster.

tags-alias-list = Även känd som
tags-alias-add = Lägg till ett annat namn
tags-alias-remove = Ta bort { $name }
tags-alias-placeholder = Ett annat namn, sen Enter
tags-alias-hint = Namn detta visas under i din prosa, bredvid dess titel.
tags-alias-collision = { $name } svarar redan på detta namn.

settings-page-tags = Taggar
settings-tags-desc = Taggar märker objekten i din bindemedel. En hitta-in-prose-taggen berättar också Skribisto för att leta efter objektets namn i din prosa.
settings-tags-add = Lägg till tagga
settings-tags-add-placeholder = Nämn en ny tag
settings-tags-added = Tillagd "{ $name }"
settings-tags-duplicate = "{ $name }" finns redan
settings-tags-filter = Filter taggar
settings-tags-count = { $n ->
    [one] 1 tag
   *[other] { $n } taggar
}
settings-tags-details-placeholder = Vad denna tag betyder
settings-tags-discoverable = Hitta i prose
settings-tags-creates-in = Nya anteckningar går till
settings-tags-creates-in-unset = Fråga mig första gången
settings-tags-creates-in-untitled = Untitled mapp
settings-tags-creates-in-trashed = { $name } (i papperskorgen)
settings-tags-template = Starta mall
settings-tags-template-unset = Blank Note
settings-tags-delete = Radera { $name }
settings-tags-deleted = Ta bort "{ $name }" och ta bort den från varje objekt
settings-tags-empty = Inga taggar ännu.
settings-tags-apply-preset = Applicera ett förinställt...
settings-tags-preset-applied = Tillagd { $added }, hoppade { $skipped } redan närvarande
settings-tags-csv-filter = CSV filer
settings-tags-import = Importera…
settings-tags-export = Exportera…
settings-tags-imported = Importerad { $added }, hoppad { $skipped }
settings-tags-exported = Exporterad { $n ->
    [one] 1 tag
   *[other] { $n } taggar
}

tags-chip-more = { $n ->
    [one] 1 mer tagga
   *[other] { $n } fler taggar
}

cast-section = Persongalleri
cast-add = Lägg till gjutna...
cast-add-filter-placeholder = Filter story bibel...
cast-add-empty = Inga story-bibelposter att lägga till
cast-pin = Lägg till { $name } för att kasta
cast-unpin = Ta bort { $name } från cast
cast-empty = Ingen stiftade ännu. Lägg till eller håll ett förslag
cast-unresolved = Inte längre i berättelsen bibel

mentions-confirm = Lägg till { $name }
mentions-hit-count = { $n ->
    [one] en gång
   *[other] { $n } gånger
}
mentions-point-of-view-badge = Perspektiv
mentions-point-of-view-badge-tooltip = Deklarerad som synpunkt här, ställd för hand, inte upptäckt i prosa.

mentions-roster = Persongalleri
mentions-pin = Lägg till { $name } för att kasta

pov-section = Perspektiv
pov-empty = Ingen synpunkt
pov-add = Ställ in synpunkt...
pov-multiple = Denna scen har två synpunkter.
pov-remove = Ta bort { $name } som synvinkel
pov-unresolved = En synvinkel pinnades här, men det inträdet inte längre existerar eller förlorade sin historia-bibel tag.

books-section = Arkiverad under
books-empty = Inte lämnat in under en bok ännu
books-add = Fil under en bok...
books-remove = Ta bort från { $name }
books-apply-to-children = Applicera arkivering till barn

note-details-name-placeholder = Namn…
note-details-tags = Taggar
note-details-aliases = Andra namn
note-details-books = Arkiverad under
note-details-books-empty = Inte lämnat in under en bok ännu
note-details-links = Länkar
note-details-links-empty = Inga länkar ännu. Lägg till en för att peka på en annan post i din historia bibel.
note-details-links-add = Länk till en anteckning...
note-details-pov = Perspektiv
note-details-pov-empty = Ingen synpunkt
note-details-pov-unresolved = En synvinkel pinnades här, men det inträdet inte längre existerar eller förlorade sin historia-bibel tag.
note-details-pov-multiple = Denna anteckning har två synpunkter.
note-details-backlinks = framträder i manuskriptet
note-details-backlinks-empty = Ingenting ännu. När detta namn visas i din prosa, kommer det att dyka upp här.
note-details-backlinks-outside = utanför böckerna
note-details-backlinks-confirm-all = Bekräfta varje utseende
note-details-backlinks-confirm-all-tooltip = Lägg till detta bidrag till gjutningen av varje dokument som visas här som inte har bekräftat det ännu
note-details-untitled-document = Namnlös

note-in-prose-mentions = { $n ->
    [one] 1 omnämnande
   *[other] { $n } nämner
}
note-in-prose-mention-at = { $current } av { $total }
note-in-prose-mention-previous = Föregående omnämnande
note-in-prose-mention-next = Nästa omnämnande
