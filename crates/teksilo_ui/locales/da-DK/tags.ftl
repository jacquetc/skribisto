# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

# Tags: the per-project palette, the inspector section, and the preset catalog.
# Preset tag NAMES are translated deliberately: presets are generated in code rather than
# shipped as data so that applying one lands in the language of the INTERFACE the writer is
# looking at, instead of the single language a data file would have had to pick at authoring
# time. The interface's locale and not the project's `dict_language`: a tag name is a filter
# chip and an inspector row, the compiler never reads one, and it is read only by the person
# at the keyboard. Applied rows are stored literally and are theirs to rename.

## Preset menu
tags-preset-basic = Grundlæggende
tags-preset-scifi = Science fiction
tags-preset-fantasy = Fantasy
tags-preset-mystery = Krimi
tags-preset-historical = Historisk

## Basic preset: the workflow ladder. The `status/` prefix is a naming convention:
## alphabetical sorting alone makes these cluster together in every list.

## Basic preset: flags. Deliberately unprefixed: a scene can be a draft AND need
## research at the same time, so these are not part of the ladder above.
tags-preset-needs-research = kræver research
tags-preset-continuity-check = tjek kontinuitet
tags-preset-plot-point = plotpunkt

## Basic preset: the taxonomy the mention index scans prose for.
tags-preset-character = person
tags-preset-place = sted
tags-preset-item = genstand

## Genre additions
tags-preset-vessel = fartøj
tags-preset-planet = planet
tags-preset-organization = organisation
tags-preset-creature = væsen
tags-preset-faction = fraktion
tags-preset-artifact = artefakt
tags-preset-realm = rige
tags-preset-magic-system = magisystem
tags-preset-suspect = mistænkt
tags-preset-victim = offer
tags-preset-clue = spor
tags-preset-red-herring = falsk spor
tags-preset-historical-figure = historisk person
tags-preset-source = kilde
tags-preset-period-detail = tidstypisk detalje

## The tag pill field (Inspector) and its "+" picker
tags-pill-list = Mærker
tags-pill-add = Tilføj et mærke
# The Overview's Tags cell on a row with no tags yet: the cell IS the picker, so
# it stays a target rather than rendering nothing. Names the muted "+" placeholder.
tags-none-yet = Ingen mærker
# The picker's own list of pickable tags, announced apart from the
# `tags-pill-list` row of tags already assigned.
tags-pick-list = Mærker at vælge imellem
tags-pill-remove = Fjern { $name }
tags-pill-filter-placeholder = Filtrér, eller navngiv et nyt mærke
tags-pill-no-match = Intet mærke matcher
tags-pill-create = Opret »{ $name }«
tags-pill-create-failed = »{ $name }« kunne ikke oprettes
tags-pill-new-discoverable = Mærke til søgning i teksten
tags-pill-new-discoverable-hint = Elementer med dette mærke matches mod din tekst for at udfylde persongalleriet.

## The alias pill field
tags-alias-list = Også kendt som
tags-alias-add = Tilføj et andet navn
tags-alias-remove = Fjern { $name }
tags-alias-placeholder = Et andet navn, og tryk på Enter
tags-alias-hint = Navne, elementet optræder under i din tekst, ud over titlen.
tags-alias-collision = { $name } kendes allerede under dette navn.

## Settings ▸ Work ▸ Tags
settings-page-tags = Mærker
settings-tags-desc = Mærker markerer elementerne i dit ringbind. Et mærke til søgning i teksten får desuden Skribisto til at lede efter elementets navne i din tekst.
settings-tags-add = Tilføj mærke
settings-tags-add-placeholder = Navngiv et nyt mærke
settings-tags-added = »{ $name }« er tilføjet
settings-tags-duplicate = »{ $name }« findes allerede
settings-tags-filter = Filtrér mærker
settings-tags-count = { $n ->
    [one] 1 mærke
   *[other] { $n } mærker
}
settings-tags-details-placeholder = Hvad dette mærke betyder
settings-tags-discoverable = Søg i teksten
settings-tags-creates-in = Nye noter placeres i
settings-tags-creates-in-unset = Spørg mig første gang
settings-tags-creates-in-untitled = Mappe uden titel
settings-tags-creates-in-trashed = { $name } (i papirkurven)
settings-tags-template = Startskabelon
settings-tags-template-unset = Tom note
settings-tags-delete = Slet { $name }
settings-tags-deleted = »{ $name }« er slettet og fjernet fra alle elementer
settings-tags-empty = Ingen mærker endnu.
settings-tags-apply-preset = Anvend en forudindstilling…
settings-tags-preset-applied = { $added } tilføjet, { $skipped } fandtes allerede og blev sprunget over
settings-tags-csv-filter = CSV-filer
settings-tags-import = Importér…
settings-tags-export = Eksportér…
settings-tags-imported = { $added } importeret, { $skipped } sprunget over
settings-tags-exported = { $n ->
    [one] 1 mærke er eksporteret
   *[other] { $n } mærker er eksporteret
}

## The dot row shown on the stream, corkboard and editor subtitle
tags-chip-more = { $n ->
    [one] 1 mærke mere
   *[other] { $n } mærker mere
}

## Cast of the scene (references-first story-bible pins + suggestions)
cast-section = Persongalleri
cast-add = Føj til persongalleriet…
cast-add-filter-placeholder = Filtrér historiebiblen…
cast-add-empty = Ingen opslag i historiebiblen at tilføje
cast-pin = Føj { $name } til persongalleriet
cast-unpin = Fjern { $name } fra persongalleriet
cast-empty = Ingen er fastgjort endnu. Tilføj, eller behold et forslag
cast-unresolved = Findes ikke længere i historiebiblen

## Backlinks on a story-bible item
# There is no `mentions-backlinks` heading any more: the Inspector's "Appears in"
# list was removed, and the entry's own Details page names its list
# `note-details-backlinks`. The keys below are the row controls, which that page
# still uses.
# The control on a suggested backlink row: the writer says yes, this really is her,
# and the entry is written into that document's cast. Named after what it does rather
# than after the agreement, and worded to mirror `cast-pin`, which is the same write
# made from the other end. Confirm only: there is no "not her" to record; see
# `teksilo_ui::mentions::presence`.
mentions-confirm = Føj til persongalleriet i { $name }
mentions-hit-count = { $n ->
    [one] én gang
   *[other] { $n } gange
}
# The badge on a row whose target is the owner's declared point of view (see
# `point_of_view`, distinct from `references`). Shown on both the Cast list and the
# backlinks list, since a point of view can appear in either direction.
mentions-point-of-view-badge = Synsvinkel
mentions-point-of-view-badge-tooltip = Angivet som synsvinkel her, sat manuelt, ikke fundet i teksten.

## Legacy keys kept so older scripts/tests that still reference them compile
mentions-roster = Persongalleri
mentions-pin = Føj { $name } til persongalleriet

# ── Point of view ────────────────────────────────────────────────────────────
# Whose eyes a scene is told through. Distinct from the cast above: the cast is
# who appears, the point of view is who holds the camera. A scene may have none
# (unassigned) or, deliberately, more than one, which is what head-hopping is.
pov-section = Synsvinkel
pov-empty = Ingen synsvinkel angivet
pov-add = Angiv synsvinkel…
pov-multiple = Denne scene har to synsvinkler.
pov-remove = Fjern { $name } som synsvinkel
pov-unresolved = En synsvinkel blev fastgjort her, men opslaget findes ikke længere eller har mistet sit historiebibel-mærke.

# ── Book filing ──────────────────────────────────────────────────────────────
# Which Book or Books a note or note folder is declared to belong to. Shown
# only on story-bible material outside the manuscript flow, since a scene's
# Book is already given by where it sits in the binder, and only once the
# project holds two or more Books; a one-Book project has nothing to file
# against.
books-section = Hører til
books-empty = Hører endnu ikke til en bog
books-add = Knyt til en bog…
books-remove = Fjern fra { $name }
books-apply-to-children = Anvend tilknytningen på underelementer

# ── The Details segment on an `Item/Note` tab ────────────────────────────────
# Almost the same field set as the Inspector's own story-bible section (tags,
# aliases, cast, point of view, books), reused deliberately, but under its own
# `note-details-` keys: this segment is a full-width page, not a dock, so its
# copy is free to say a little more than the dock's narrower strings do. See
# `teksilo_ui::tabs::note_details`.
note-details-name-placeholder = Navn…
note-details-tags = Mærker
note-details-aliases = Andre navne
note-details-books = Hører til
note-details-books-empty = Hører endnu ikke til en bog
note-details-links = Links
note-details-links-empty = Ingen links endnu. Tilføj et for at pege på et andet opslag i din historiebibel.
note-details-links-add = Link til en note…
note-details-pov = Synsvinkel
note-details-pov-empty = Ingen synsvinkel angivet
note-details-pov-unresolved = En synsvinkel blev fastgjort her, men opslaget findes ikke længere eller har mistet sit historiebibel-mærke.
note-details-pov-multiple = Denne note har to synsvinkler.
note-details-backlinks = Optræder i manuskriptet
note-details-backlinks-empty = Intet endnu. Når navnet optræder i din tekst, vises det her.
note-details-backlinks-outside = Uden for bøgerne
note-details-backlinks-confirm-all = Bekræft alle forekomster
note-details-backlinks-confirm-all-tooltip = Føj dette opslag til persongalleriet i alle de viste dokumenter, der endnu ikke har bekræftet det
note-details-untitled-document = Uden titel

## The In prose reading's own counter: how many times this entry is named across
## the rows on screen, and which of them the reader has stepped to. See
## `teksilo_ui::story_bible::highlight::SubjectWalk`.
note-in-prose-mentions = { $n ->
    [one] 1 omtale
   *[other] { $n } omtaler
}
note-in-prose-mention-at = { $current } af { $total }
note-in-prose-mention-previous = Forrige omtale
note-in-prose-mention-next = Næste omtale
