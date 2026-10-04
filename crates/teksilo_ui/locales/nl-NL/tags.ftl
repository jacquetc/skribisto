# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

tags-preset-basic = Basis
tags-preset-scifi = Science fiction
tags-preset-fantasy = Fantasie
tags-preset-mystery = Mysterie
tags-preset-historical = Historisch


tags-preset-needs-research = heeft onderzoek nodig
tags-preset-continuity-check = continuïteitscontrole
tags-preset-plot-point = plotpunt

tags-preset-character = teken
tags-preset-place = Plaats
tags-preset-item = item

tags-preset-vessel = vaartuig
tags-preset-planet = planeet
tags-preset-organization = organisatie
tags-preset-creature = wezen
tags-preset-faction = factie
tags-preset-artifact = artefact
tags-preset-realm = rijk
tags-preset-magic-system = magisch systeem
tags-preset-suspect = verdachte
tags-preset-victim = slachtoffer
tags-preset-clue = hint
tags-preset-red-herring = rode haring
tags-preset-historical-figure = historische figuur
tags-preset-source = bron
tags-preset-period-detail = periode detail

tags-pill-list = Tags
tags-pill-add = Een tag toevoegen
tags-none-yet = Geen tags
tags-pick-list = Tags om uit te kiezen
tags-pill-remove = Verwijderen { $name }
tags-pill-filter-placeholder = Een nieuwe tag filteren of een naam geven
tags-pill-no-match = Geen tag overeenkomsten
tags-pill-create = Maak "{ $name }" aan
tags-pill-create-failed = Kon "{ $name }" niet aanmaken
tags-pill-new-discoverable = Zoek-in-prose-tag
tags-pill-new-discoverable-hint = Items met deze tag zijn afgestemd op uw proza om het rooster te vullen.

tags-alias-list = Ook bekend als
tags-alias-add = Een andere naam toevoegen
tags-alias-remove = Verwijderen { $name }
tags-alias-placeholder = Een andere naam, dan Enter
tags-alias-hint = Namen waar dit onder staat in je proza, naast de titel.
tags-alias-collision = { $name } antwoordt al op deze naam.

settings-page-tags = Tags
settings-tags-desc = Tags label de items in uw map. Een find-in-prose tag vertelt ook Skribisto om te zoeken naar de namen van dat item in uw proza.
settings-tags-add = Tag toevoegen
settings-tags-add-placeholder = Naam van een nieuwe tag
settings-tags-added = Toegevoegd "{ $name }"
settings-tags-duplicate = "{ $name }" bestaat al
settings-tags-filter = Filtertags
settings-tags-count = { $n ->
    [one] 1 label
   *[other] { $n } tags
}
settings-tags-details-placeholder = Wat deze tag betekent
settings-tags-discoverable = Zoek in proza
settings-tags-creates-in = Nieuwe notities gaan naar
settings-tags-creates-in-unset = Vraag het me de eerste keer
settings-tags-creates-in-untitled = Naamloze map
settings-tags-creates-in-trashed = { $name } (in de prullenbak)
settings-tags-template = Sjabloon starten
settings-tags-template-unset = Lege noot
settings-tags-delete = { $name } schrappen
settings-tags-deleted = Verwijderd "{ $name }" en verwijderd van elk item
settings-tags-empty = Nog geen labels.
settings-tags-apply-preset = Een voorinstelling toepassen...
settings-tags-preset-applied = Toegevoegd { $added }, overgeslagen { $skipped } reeds aanwezig
settings-tags-csv-filter = CSV bestanden
settings-tags-import = Importeren…
settings-tags-export = Exporteren…
settings-tags-imported = Geïmporteerd { $added }, overgeslagen { $skipped }
settings-tags-exported = Exporteren { $n ->
    [one] 1 label
   *[other] { $n } tags
}

tags-chip-more = { $n ->
    [one] 1 extra tag
   *[other] { $n } meer tags
}

cast-section = Personages
cast-add = Voeg toe aan cast...
cast-add-filter-placeholder = Filter verhaal bijbel...
cast-add-empty = Geen verhalen-bijbel items toe te voegen
cast-pin = Voeg { $name } toe aan cast
cast-unpin = Verwijderen { $name } uit gegoten
cast-empty = Nog niemand vastgebonden. Voeg een suggestie toe of houd deze bij
cast-unresolved = Niet meer in de verhaal bijbel

mentions-confirm = Voeg toe aan de cast van { $name }
mentions-hit-count = { $n ->
    [one] eenmaal
   *[other] { $n } keer
}
mentions-point-of-view-badge = Perspectief
mentions-point-of-view-badge-tooltip = Uitgesproken als het standpunt hier, ingesteld met de hand, niet gedetecteerd in de proza.

mentions-roster = Personages
mentions-pin = Voeg { $name } toe aan cast

pov-section = Perspectief
pov-empty = Geen standpunt vastgesteld
pov-add = Standpunt instellen...
pov-multiple = Deze scène heeft twee standpunten.
pov-remove = { $name } schrappen als standpunt
pov-unresolved = Een standpunt werd hier gepind, maar die ingang bestaat niet meer of verloren zijn verhaal-bijbel tag.

books-section = Geplaatst onder
books-empty = Nog niet ingediend onder een boek
books-add = Bestand onder een boek...
books-remove = Verwijderen uit { $name }
books-apply-to-children = Aanmelden voor kinderen

note-details-name-placeholder = Naam…
note-details-tags = Tags
note-details-aliases = Andere namen
note-details-books = Geplaatst onder
note-details-books-empty = Nog niet ingediend onder een boek
note-details-links = Links
note-details-links-empty = Nog geen links. Voeg er een toe aan een andere vermelding in je verhaal bijbel.
note-details-links-add = Link naar een briefje...
note-details-pov = Perspectief
note-details-pov-empty = Geen standpunt vastgesteld
note-details-pov-unresolved = Een standpunt werd hier gepind, maar die ingang bestaat niet meer of verloren zijn verhaal-bijbel tag.
note-details-pov-multiple = Deze nota heeft twee standpunten.
note-details-backlinks = Verschijnt in het manuscript
note-details-backlinks-empty = Zodra deze naam in je proza verschijnt, zal hij hier verschijnen.
note-details-backlinks-outside = Buiten de boeken
note-details-backlinks-confirm-all = Bevestig elke verschijning
note-details-backlinks-confirm-all-tooltip = Voeg dit item toe aan de cast van elk hier getoond document dat het nog niet heeft bevestigd
note-details-untitled-document = Naamloos

note-in-prose-mentions = { $n ->
    [one] 1 vermelding
   *[other] { $n } vermeldt
}
note-in-prose-mention-at = { $current } van { $total }
note-in-prose-mention-previous = Vorige vermelding
note-in-prose-mention-next = Volgende vermelding
