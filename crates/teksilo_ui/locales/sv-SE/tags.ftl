# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet
tags-preset-basic = Grundläggande
tags-preset-scifi = Science fiction
tags-preset-fantasy = Fantasy
tags-preset-mystery = Deckare
tags-preset-historical = Historisk
tags-preset-needs-research = behöver research
tags-preset-continuity-check = kontrollera kontinuitet
tags-preset-plot-point = vändpunkt
tags-preset-character = person
tags-preset-place = plats
tags-preset-item = föremål
tags-preset-vessel = farkost
tags-preset-planet = planet
tags-preset-organization = organisation
tags-preset-creature = varelse
tags-preset-faction = fraktion
tags-preset-artifact = artefakt
tags-preset-realm = rike
tags-preset-magic-system = magisystem
tags-preset-suspect = misstänkt
tags-preset-victim = offer
tags-preset-clue = ledtråd
tags-preset-red-herring = villospår
tags-preset-historical-figure = historisk person
tags-preset-source = källa
tags-preset-period-detail = tidsdetalj
tags-pill-list = Taggar
tags-pill-add = Lägg till en tagg
tags-none-yet = Inga taggar
tags-pick-list = Taggar att välja bland
tags-pill-remove = Ta bort { $name }
tags-pill-filter-placeholder = Filtrera eller namnge en ny tagg
tags-pill-no-match = Ingen tagg matchar
tags-pill-create = Skapa ”{ $name }”
tags-pill-create-failed = Det gick inte att skapa ”{ $name }”
tags-pill-new-discoverable = Tagg som söks i prosan
tags-pill-new-discoverable-hint = Objekt med den här taggen matchas mot din prosa för att fylla i persongalleriet.
tags-alias-list = Även känd som
tags-alias-add = Lägg till ett annat namn
tags-alias-remove = Ta bort { $name }
tags-alias-placeholder = Ett annat namn, tryck sedan Retur
tags-alias-hint = Namn som detta förekommer under i din prosa, utöver titeln.
tags-alias-collision = { $name } svarar redan på det namnet.
settings-page-tags = Taggar
settings-tags-desc = Taggar märker objekten i din pärm. En tagg som söks i prosan får också Skribisto att leta efter objektets namn i din prosa.
settings-tags-add = Lägg till tagg
settings-tags-add-placeholder = Namnge en ny tagg
settings-tags-added = ”{ $name }” har lagts till
settings-tags-duplicate = ”{ $name }” finns redan
settings-tags-filter = Filtrera taggar
settings-tags-count = { $n ->
       [one] 1 tagg
      *[other] { $n } taggar
   }
settings-tags-details-placeholder = Vad den här taggen betyder
settings-tags-discoverable = Sök i prosan
settings-tags-creates-in = Nya anteckningar hamnar i
settings-tags-creates-in-unset = Fråga mig första gången
settings-tags-creates-in-untitled = Namnlös mapp
settings-tags-creates-in-trashed = { $name } (i papperskorgen)
settings-tags-template = Startmall
settings-tags-template-unset = Tom anteckning
settings-tags-delete = Ta bort { $name }
settings-tags-deleted = ”{ $name }” har tagits bort, även från alla objekt
settings-tags-empty = Inga taggar ännu.
settings-tags-apply-preset = Använd en förinställning…
settings-tags-preset-applied = Tillagda: { $added }, överhoppade eftersom de redan fanns: { $skipped }
settings-tags-csv-filter = CSV-filer
settings-tags-import = Importera…
settings-tags-export = Exportera…
settings-tags-imported = Importerade: { $added }, överhoppade: { $skipped }
settings-tags-exported = { $n ->
       [one] 1 tagg exporterad
      *[other] { $n } taggar exporterade
   }
tags-chip-more = { $n ->
       [one] 1 tagg till
      *[other] { $n } taggar till
   }
cast-section = Persongalleri
cast-add = Lägg till i persongalleriet…
cast-add-filter-placeholder = Filtrera berättelsebibeln…
cast-add-empty = Inga poster i berättelsebibeln att lägga till
cast-pin = Lägg till { $name } i persongalleriet
cast-unpin = Ta bort { $name } från persongalleriet
cast-empty = Ingen fäst ännu. Lägg till eller behåll ett förslag
cast-unresolved = Finns inte längre i berättelsebibeln
mentions-confirm = Lägg till i persongalleriet för { $name }
mentions-hit-count = { $n ->
       [one] en gång
      *[other] { $n } gånger
   }
mentions-point-of-view-badge = Berättarperspektiv
mentions-point-of-view-badge-tooltip = Angivet som berättarperspektiv här, för hand, inte upptäckt i prosan.
mentions-roster = Persongalleri
mentions-pin = Lägg till { $name } i persongalleriet
pov-section = Berättarperspektiv
pov-empty = Inget berättarperspektiv angivet
pov-add = Ange berättarperspektiv…
pov-multiple = Den här scenen har två berättarperspektiv.
pov-remove = Ta bort { $name } som berättarperspektiv
pov-unresolved = Ett berättarperspektiv fästes här, men posten finns inte längre eller har förlorat sin tagg för berättelsebibeln.
books-section = Arkiverad under
books-empty = Ännu inte arkiverad under någon bok
books-add = Arkivera under en bok…
books-remove = Ta bort från { $name }
books-apply-to-children = Tillämpa arkiveringen på underobjekt
note-details-name-placeholder = Namn…
note-details-tags = Taggar
note-details-aliases = Andra namn
note-details-books = Arkiverad under
note-details-books-empty = Ännu inte arkiverad under någon bok
note-details-links = Länkar
note-details-links-empty = Inga länkar ännu. Lägg till en för att peka på en annan post i din berättelsebibel.
note-details-links-add = Länka till en anteckning…
note-details-pov = Berättarperspektiv
note-details-pov-empty = Inget berättarperspektiv angivet
note-details-pov-unresolved = Ett berättarperspektiv fästes här, men posten finns inte längre eller har förlorat sin tagg för berättelsebibeln.
note-details-pov-multiple = Den här anteckningen har två berättarperspektiv.
note-details-backlinks = Förekommer i manuset
note-details-backlinks-empty = Inget ännu. När namnet förekommer i din prosa visas det här.
note-details-backlinks-outside = Utanför böckerna
note-details-backlinks-confirm-all = Bekräfta alla förekomster
note-details-backlinks-confirm-all-tooltip = Lägg till posten i persongalleriet för alla dokument som visas här och som inte har bekräftat den ännu
note-details-untitled-document = Namnlöst
note-in-prose-mentions = { $n ->
       [one] 1 omnämnande
      *[other] { $n } omnämnanden
   }
note-in-prose-mention-at = { $current } av { $total }
note-in-prose-mention-previous = Föregående omnämnande
note-in-prose-mention-next = Nästa omnämnande
