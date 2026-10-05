# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

tags-preset-basic = Basis
tags-preset-scifi = Sciencefiction
tags-preset-fantasy = Fantasy
tags-preset-mystery = Mysterie
tags-preset-historical = Historisch


tags-preset-needs-research = research nodig
tags-preset-continuity-check = continuïteitscontrole
tags-preset-plot-point = plotpunt

tags-preset-character = personage
tags-preset-place = plaats
tags-preset-item = voorwerp

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
tags-preset-clue = aanwijzing
tags-preset-red-herring = dwaalspoor
tags-preset-historical-figure = historische figuur
tags-preset-source = bron
tags-preset-period-detail = tijdsdetail

tags-pill-list = Tags
tags-pill-add = Een tag toevoegen
tags-none-yet = Geen tags
tags-pick-list = Tags om uit te kiezen
tags-pill-remove = { $name } verwijderen
tags-pill-filter-placeholder = Filteren of een nieuwe tag benoemen
tags-pill-no-match = Geen enkele tag komt overeen
tags-pill-create = “{ $name }” aanmaken
tags-pill-create-failed = Kan “{ $name }” niet aanmaken
tags-pill-new-discoverable = Tag met zoeken in proza
tags-pill-new-discoverable-hint = Items met deze tag worden met uw proza vergeleken om de rolbezetting te vullen.

tags-alias-list = Ook bekend als
tags-alias-add = Een andere naam toevoegen
tags-alias-remove = { $name } verwijderen
tags-alias-placeholder = Een andere naam, daarna Enter
tags-alias-hint = Namen waaronder dit in uw proza voorkomt, naast de titel.
tags-alias-collision = { $name } luistert al naar deze naam.

settings-page-tags = Tags
settings-tags-desc = Tags markeren de items in uw ordner. Een tag met zoeken in proza laat Skribisto bovendien naar de namen van dat item in uw proza zoeken.
settings-tags-add = Tag toevoegen
settings-tags-add-placeholder = Een nieuwe tag benoemen
settings-tags-added = “{ $name }” toegevoegd
settings-tags-duplicate = “{ $name }” bestaat al
settings-tags-filter = Tags filteren
settings-tags-count = { $n ->
        [one] 1 tag
       *[other] { $n } tags
    }
settings-tags-details-placeholder = Wat deze tag betekent
settings-tags-discoverable = Zoeken in proza
settings-tags-creates-in = Nieuwe notities gaan naar
settings-tags-creates-in-unset = De eerste keer vragen
settings-tags-creates-in-untitled = Naamloze map
settings-tags-creates-in-trashed = { $name } (in de prullenbak)
settings-tags-template = Beginsjabloon
settings-tags-template-unset = Lege notitie
settings-tags-delete = { $name } verwijderen
settings-tags-deleted = “{ $name }” verwijderd en van elk item gehaald
settings-tags-empty = Nog geen tags.
settings-tags-apply-preset = Een voorinstelling toepassen…
settings-tags-preset-applied = { $added } toegevoegd, { $skipped } overgeslagen die al aanwezig waren
settings-tags-csv-filter = CSV-bestanden
settings-tags-import = Importeren…
settings-tags-export = Exporteren…
settings-tags-imported = { $added } geïmporteerd, { $skipped } overgeslagen
settings-tags-exported = { $n ->
        [one] 1 tag geëxporteerd
       *[other] { $n } tags geëxporteerd
    }

tags-chip-more = { $n ->
        [one] nog 1 tag
       *[other] nog { $n } tags
    }

cast-section = Rolbezetting
cast-add = Toevoegen aan rolbezetting…
cast-add-filter-placeholder = Verhaalbijbel filteren…
cast-add-empty = Geen items uit de verhaalbijbel om toe te voegen
cast-pin = { $name } toevoegen aan rolbezetting
cast-unpin = { $name } uit rolbezetting verwijderen
cast-empty = Nog niemand vastgezet. Voeg iemand toe of houd een suggestie aan
cast-unresolved = Niet meer in de verhaalbijbel

mentions-confirm = Toevoegen aan de rolbezetting van { $name }
mentions-hit-count = { $n ->
    [one] eenmaal
   *[other] { $n } keer
}
mentions-point-of-view-badge = Vertelperspectief
mentions-point-of-view-badge-tooltip = Hier met de hand als vertelperspectief opgegeven, niet in het proza gevonden.

mentions-roster = Rolbezetting
mentions-pin = { $name } toevoegen aan rolbezetting

pov-section = Vertelperspectief
pov-empty = Geen vertelperspectief ingesteld
pov-add = Vertelperspectief instellen…
pov-multiple = Deze scène heeft twee vertelperspectieven.
pov-remove = { $name } als vertelperspectief verwijderen
pov-unresolved = Hier was een vertelperspectief vastgezet, maar dat item bestaat niet meer of heeft zijn tag voor de verhaalbijbel verloren.

books-section = Ingedeeld bij
books-empty = Nog niet bij een boek ingedeeld
books-add = Bij een boek indelen…
books-remove = Uit { $name } verwijderen
books-apply-to-children = Indeling toepassen op onderliggende items

note-details-name-placeholder = Naam…
note-details-tags = Tags
note-details-aliases = Andere namen
note-details-books = Ingedeeld bij
note-details-books-empty = Nog niet bij een boek ingedeeld
note-details-links = Koppelingen
note-details-links-empty = Nog geen koppelingen. Voeg er een toe om naar een ander item in uw verhaalbijbel te verwijzen.
note-details-links-add = Koppelen aan een notitie…
note-details-pov = Vertelperspectief
note-details-pov-empty = Geen vertelperspectief ingesteld
note-details-pov-unresolved = Hier was een vertelperspectief vastgezet, maar dat item bestaat niet meer of heeft zijn tag voor de verhaalbijbel verloren.
note-details-pov-multiple = Deze notitie heeft twee vertelperspectieven.
note-details-backlinks = Komt voor in het manuscript
note-details-backlinks-empty = Nog niets. Zodra deze naam in uw proza voorkomt, verschijnt die hier.
note-details-backlinks-outside = Buiten de boeken
note-details-backlinks-confirm-all = Elk voorkomen bevestigen
note-details-backlinks-confirm-all-tooltip = Dit item toevoegen aan de rolbezetting van elk hier getoond document dat het nog niet heeft bevestigd
note-details-untitled-document = Naamloos

note-in-prose-mentions = { $n ->
        [one] 1 vermelding
       *[other] { $n } vermeldingen
    }
note-in-prose-mention-at = { $current } van { $total }
note-in-prose-mention-previous = Vorige vermelding
note-in-prose-mention-next = Volgende vermelding
